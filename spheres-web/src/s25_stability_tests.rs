//! Test-only ordinary campaign stability cell. The external matrix runner owns
//! cryptographic provenance and coverage; this test never awards S25 or CP1.
use super::*;
use serde::Deserialize;
use serde_json::{json, Value};
use spheres_sim::clock;
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

type Check<T = ()> = Result<T, String>;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    format: String,
    id: String,
    country: String,
    seed: u64,
    through: String,
    revision: String,
}

fn require(ok: bool, message: impl Into<String>) -> Check {
    if ok {
        Ok(())
    } else {
        Err(message.into())
    }
}

fn day_label(day: i32) -> String {
    let (year, month, date) = clock::date_from_day(day);
    format!("{year:04}-{month:02}-{date:02}")
}

fn through_day(value: &str) -> Check<i32> {
    let pieces: Vec<_> = value.split('-').collect();
    require(pieces.len() == 3, "through must be YYYY-MM-DD")?;
    let year: i32 = pieces[0].parse().map_err(|_| "Invalid through year")?;
    let month: u32 = pieces[1].parse().map_err(|_| "Invalid through month")?;
    let day: u32 = pieces[2].parse().map_err(|_| "Invalid through day")?;
    require(
        (1990..=2035).contains(&year) && (1..=12).contains(&month),
        "through outside the 1990-2035 campaign",
    )?;
    require(
        (1..=days_in_month(year, month)).contains(&day),
        "Invalid Gregorian through date",
    )?;
    let absolute = clock::date_day(year, month, day);
    require(
        day_label(absolute) == value,
        "through must use exact YYYY-MM-DD notation",
    )?;
    Ok(absolute)
}

fn validate_request(r: &Request) -> Check<(NationId, i32)> {
    require(
        r.format == "spheres-stability-cell/v1",
        "Unsupported stability request",
    )?;
    require(
        !r.id.is_empty()
            && r.id.len() <= 96
            && r.id
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_'),
        "Invalid cell ID",
    )?;
    let nation: NationId = serde_json::from_value(json!(r.country))
        .map_err(|_| "country must be a native NationId")?;
    require(
        [
            NationId::France,
            NationId::Japan,
            NationId::India,
            NationId::Brazil,
            NationId::SouthAfrica,
            NationId::Tonga,
            NationId::SaudiArabia,
            NationId::USSR,
        ]
        .contains(&nation),
        "Country outside the eight declared cases",
    )?;
    require(
        [1990, 7, 42].contains(&r.seed),
        "Seed outside the declared three-seed panel",
    )?;
    require(
        r.revision.len() == 40
            && r.revision
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)),
        "revision must be a full lowercase Git identity",
    )?;
    Ok((nation, through_day(&r.through)?))
}

fn fingerprint(bytes: &[u8]) -> String {
    let value = bytes.iter().fold(0xcbf29ce484222325u64, |v, b| {
        (v ^ u64::from(*b)).wrapping_mul(0x100000001b3)
    });
    format!("{value:016x}")
}

// Called only for storage::encode output/native written files. Keep every byte,
// including key order and number spelling, except the final wall-clock member.
// No parse/reserialize round trip is allowed to normalize a divergent archive.
fn canonical_archive(text: &str) -> Check<Vec<u8>> {
    require(
        text.starts_with("{\"format\":\"spheres-campaign\",\"version\":1,"),
        "Expected current compact native campaign envelope",
    )?;
    let (prefix, tail) = text
        .rsplit_once(",\"saved_unix\":")
        .ok_or("Native archive lacks terminal saved_unix")?;
    let stamp = tail
        .strip_suffix('}')
        .ok_or("Malformed terminal saved_unix")?;
    require(
        !stamp.is_empty()
            && stamp.bytes().all(|b| b.is_ascii_digit())
            && stamp.parse::<u64>().is_ok(),
        "Invalid terminal saved_unix",
    )?;
    let mut bytes = Vec::with_capacity(prefix.len() + 1);
    bytes.extend_from_slice(prefix.as_bytes());
    bytes.push(b'}');
    Ok(bytes)
}

fn cheap_invariants(w: &WorldState) -> Check {
    require(clock::is_daily(w), "Daily rule disappeared")?;
    require(
        w.oil_price.is_finite() && w.oil_price > 0.0,
        "Invalid oil price",
    )?;
    for n in &w.nations {
        require(
            n.population.is_finite() && n.population >= 0.0 && (!n.alive || n.population > 0.0),
            format!("{:?}: invalid population", n.id),
        )?;
        require(
            n.gdp.is_finite() && (!n.alive || n.gdp > 0.0),
            format!("{:?}: invalid GDP", n.id),
        )?;
        require(
            n.inflation.is_finite() && n.debt_gdp.is_finite(),
            format!("{:?}: nonfinite fiscal ratio", n.id),
        )?;
        for (name, money) in [("treasury", n.treasury_bn), ("debt", n.debt_bn)] {
            require(
                money.is_none_or(|x| x.is_finite() && x >= 0.0),
                format!("{:?}: invalid {name}", n.id),
            )?;
        }
    }
    require(
        w.districts
            .values()
            .all(|id| w.nation_opt(*id).is_some_and(|n| n.alive)),
        "A missing or ceased government owns territory",
    )
}

fn native_invariants(w: &WorldState) -> Check {
    cheap_invariants(w)?;
    for n in &w.nations {
        spheres_sim::equipment::validate_state(n)?;
    }
    spheres_sim::manufacturing::validate_state(w)?;
    spheres_sim::companies::validate_state(w)?;
    spheres_sim::connected_economy::validate(w)?;
    spheres_sim::company_network::validate(w)?;
    spheres_sim::operational_warfare::validate(w)?;
    spheres_sim::aviation::validate(w)?;
    spheres_sim::airbases::validate(w)?;
    spheres_sim::airmissions::validate(w)?;
    spheres_sim::supplier_catalogue::validate(w)?;
    spheres_sim::military_ai::validate(w)?;
    spheres_sim::party_leadership::validate_state(w)?;
    Ok(())
}

fn equivalent(left: &Game, right: &Game) -> Check<Vec<u8>> {
    equivalent_with_diagnostic_digest(left, right, fingerprint)
}

fn equivalent_with_diagnostic_digest(
    left: &Game,
    right: &Game,
    mut diagnostic_digest: impl FnMut(&[u8]) -> String,
) -> Check<Vec<u8>> {
    let a = canonical_archive(&storage::encode(left)?)?;
    let b = canonical_archive(&storage::encode(right)?)?;
    // These full-buffer digests explain a mismatch; they do not establish
    // equality. Keep them off the success path, whose report digest is still
    // computed separately by compare().
    if a != b {
        return Err(format!("Complete archive mismatch at {}: bytes {}/{}, FNV {}/{}; no finance/history fields excluded",
            day_label(clock::absolute_day(&left.world)), a.len(), b.len(), diagnostic_digest(&a), diagnostic_digest(&b)));
    }
    Ok(a)
}

fn bump(report: &mut Value, name: &str, count: u64) {
    let old = report["checks"][name].as_u64().unwrap_or(0);
    report["checks"][name] = json!(old + count);
}

fn compare(left: &Game, right: &Game, kind: &str, report: &mut Value) -> Check {
    require(
        report["initial_rules"].is_object()
            && serde_json::to_value(&left.world.rules).map_err(|e| e.to_string())?
                == report["initial_rules"]
            && serde_json::to_value(&right.world.rules).map_err(|e| e.to_string())?
                == report["initial_rules"],
        "Campaign rules changed from the ordinary adopted opening",
    )?;
    native_invariants(&left.world)?;
    native_invariants(&right.world)?;
    let bytes = equivalent(left, right)?;
    bump(report, "native_validation_checks", 2);
    report["comparisons"].as_array_mut().unwrap().push(json!({"kind":kind,
        "date":day_label(clock::absolute_day(&left.world)), "absolute_day":clock::absolute_day(&left.world),
        "matched":true,"canonical_bytes":bytes.len(),"canonical_fnv64":fingerprint(&bytes),
        "history_rows":left.history.len(),"log_rows":left.log.len(),"player":left.world.player}));
    Ok(())
}

fn write_report(out: &Path, report: &Value) -> Check {
    fs::write(
        out.join("result.json"),
        serde_json::to_vec_pretty(report).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())
}

fn progress(out: &Path, report: &Value, label: &str) -> Check {
    let mut f = fs::OpenOptions::new()
        .append(true)
        .open(out.join("progress.jsonl"))
        .map_err(|e| e.to_string())?;
    writeln!(f, "{}", json!({"kind":label,"days_each_leg":report["days_each_leg"],
        "end_native_date":report["end_native_date"],"comparisons":report["comparisons"].as_array().unwrap().len(),
        "actions":report["actions"].as_array().unwrap().len()})).map_err(|e| e.to_string())?;
    f.flush().map_err(|e| e.to_string())?;
    write_report(out, report)
}

fn fresh(seed: u64, nation: NationId) -> Check<Game> {
    let mut g = Game::new_fresh(seed, None);
    let (response, ok) = new_game(&mut g, seed, Some(nation));
    require(
        ok,
        format!(
            "Ordinary new_game refused: {}",
            response.get("error").unwrap_or(&Value::Null)
        ),
    )?;
    require(
        clock::absolute_day(&g.world) == 0 && g.world.player == Some(nation),
        "Fresh campaign identity/date mismatch",
    )?;
    Ok(g)
}

fn adopt(g: &mut Game) -> Check<Value> {
    let nation = g.world.player.ok_or("No player for adoption")?;
    require(
        !g.world.rules.economic_competition,
        "Fresh startup unexpectedly already adopted competition; review workload",
    )?;
    let pc = g.world.nation(nation).political_capital;
    let price = spheres_sim::price_of(&g.world, &Command::EnableEconomicCompetition { nation });
    let payload = json!({"session_id":g.session_id,"player_context":nation,"commands":[{"kind":"enable_economic_competition"}]});
    let response = transport::immediate_request(g, &payload).map_err(|e| e.message)?;
    require(
        response["errors"].as_array().is_none_or(|e| e.is_empty()),
        "Ordinary competition adoption refused",
    )?;
    require(
        spheres_sim::economic_ai::enabled(&g.world)
            && g.world.supplier_catalogue.enabled
            && g.world.military_ai.enabled,
        "Autonomous policies must be enabled through the ordinary adoption",
    )?;
    require(
        clock::absolute_day(&g.world) == 0,
        "Adoption advanced calendar",
    )?;
    Ok(
        json!({"command":{"kind":"enable_economic_competition"},"price_pc":price,
        "pc_before":pc,"pc_after":g.world.nation(nation).political_capital}),
    )
}

fn renewal(g: &Game) -> Check<Vec<Command>> {
    if g.journey.observing {
        return Ok(vec![]);
    }
    let me = g.world.player.ok_or("No government")?;
    let n = g
        .world
        .nation_opt(me)
        .filter(|n| n.alive)
        .ok_or("Cessation requires an ordinary continuation")?;
    let due = n.program_budget.as_ref().map_or_else(
        || {
            n.annual_budget
                .as_ref()
                .is_none_or(|p| p.fiscal_year != g.world.year)
        },
        |p| p.fiscal_year != g.world.year,
    );
    if !due {
        return Ok(vec![]);
    }
    let allocations = n.budget_for(g.world.year).allocations;
    Ok(vec![if let Some(plan) = &n.program_budget {
        Command::SetProgramBudget {
            nation: me,
            fiscal_year: g.world.year,
            allocations,
            departments: plan.departments,
        }
    } else {
        Command::SetAnnualBudget {
            nation: me,
            fiscal_year: g.world.year,
            allocations,
        }
    }])
}

fn continuation(g: &mut Game, sandbox: bool) -> Check<Option<Value>> {
    if campaign_journey::pause_reason(g).is_none() {
        return Ok(None);
    }
    let action = if sandbox {
        require(
            g.world.year == 2036
                && g.world.month == 1
                && g.world.day == 1
                && !g.journey.beyond_2035,
            "Sandbox continuation must follow the exact horizon",
        )?;
        json!({"kind":"continue_campaign","action":"beyond_2035","date":g.world.date_str(),"player":g.world.player})
    } else {
        require(
            g.world.year <= 2035 || g.journey.beyond_2035,
            "Unexpected horizon before requested target",
        )?;
        let can_russia = g.world.player == Some(NationId::USSR)
            && g.world.has_flag("ussr_dissolved")
            && g.world
                .nation_opt(NationId::Russia)
                .is_some_and(|n| n.alive);
        json!({"kind":"continue_campaign","action":if can_russia {"successor"} else {"observe"},
            "target":if can_russia {Some(NationId::Russia)} else {None},"date":g.world.date_str(),"player":g.world.player})
    };
    let payload = json!({"session_id":g.session_id,"player_context":g.world.player,"commands":[action.clone()]});
    let response = transport::immediate_request(g, &payload).map_err(|e| e.message)?;
    require(
        response["errors"].as_array().is_none_or(|a| a.is_empty()),
        "Continuation refused",
    )?;
    // If the government ceased on the last campaign day, acknowledging the
    // horizon legitimately exposes a second, separate government review.
    require(
        campaign_journey::pause_reason(g).is_none()
            || (sandbox
                && g.journey.beyond_2035
                && g.world
                    .player
                    .is_some_and(|id| !g.world.nation_opt(id).is_some_and(|n| n.alive))
                && !g.journey.observing),
        "Continuation left an unexpected terminal pause",
    )?;
    Ok(Some(action))
}

fn paired_continuation(a: &mut Game, b: &mut Game, sandbox: bool, report: &mut Value) -> Check {
    let day = day_label(clock::absolute_day(&a.world));
    let first = continuation(a, sandbox)?;
    let second = continuation(b, sandbox)?;
    require(first == second, "Continuation choices diverged")?;
    if let Some(action) = first {
        report["actions"].as_array_mut().unwrap().push(
            json!({"date":day,"kind":"continuation","action":action,"legs":2,"matched":true}),
        );
    }
    Ok(())
}

fn one_day(g: &mut Game, commands: Vec<Command>) -> Check<Option<String>> {
    let before = clock::absolute_day(&g.world);
    require(
        campaign_journey::pause_reason(g).is_none(),
        "Refuse to step through terminal pause",
    )?;
    let log_start = g.log.len();
    let ordered = !commands.is_empty();
    let player = g.world.player;
    let (_, interruption) = g.advance_days(1, commands);
    require(
        clock::absolute_day(&g.world) == before + 1,
        "Ordinary advance did not settle exactly one day",
    )?;
    if ordered {
        require(
            !g.log[log_start..]
                .iter()
                .any(|e| e.text.starts_with("[rejected]")),
            "Budget renewal was rejected during ordinary settlement",
        )?;
        let n = g
            .world
            .nation_opt(player.ok_or("No budget payer")?)
            .ok_or("Budget payer disappeared")?;
        let enacted = n.program_budget.as_ref().map_or_else(
            || n.annual_budget.as_ref().map(|p| p.fiscal_year),
            |p| Some(p.fiscal_year),
        );
        require(
            enacted == Some(clock::date_from_day(before).0),
            "Budget renewal did not enact the settled year",
        )?;
    }
    cheap_invariants(&g.world)?;
    Ok(interruption)
}

fn paired_day(a: &mut Game, b: &mut Game, report: &mut Value, sandbox: bool) -> Check {
    let first = renewal(a)?;
    let second = renewal(b)?;
    require(
        serde_json::to_vec(&first).unwrap() == serde_json::to_vec(&second).unwrap(),
        "Ordinary renewal decisions diverged",
    )?;
    let day = day_label(clock::absolute_day(&a.world));
    if !first.is_empty() {
        let prices_a: Vec<_> = first
            .iter()
            .map(|c| spheres_sim::price_of(&a.world, c))
            .collect();
        let prices_b: Vec<_> = second
            .iter()
            .map(|c| spheres_sim::price_of(&b.world, c))
            .collect();
        require(
            serde_json::to_vec(&prices_a).unwrap() == serde_json::to_vec(&prices_b).unwrap(),
            "Renewal prices diverged",
        )?;
        report["actions"]
            .as_array_mut()
            .unwrap()
            .push(json!({"kind":"budget_renewal","date":day,
            "commands":first,"prices_pc":prices_a,"legs":2,"matched":true}));
    }
    let ia = one_day(a, first)?;
    let ib = one_day(b, second)?;
    require(ia == ib, "Native event interruptions diverged")?;
    if let Some(reason) = ia {
        report["actions"]
            .as_array_mut()
            .unwrap()
            .push(json!({"kind":"event_acknowledgment","date":day,
            "reason":reason,"legs":2,"matched":true}));
    }
    bump(
        report,
        if sandbox {
            "sandbox_daily_invariant_checks"
        } else {
            "daily_invariant_checks"
        },
        2,
    );
    Ok(())
}

fn mandatory(day: i32) -> bool {
    let (year, month, date) = clock::date_from_day(day);
    (month == 1 && date == 1)
        || matches!(
            (year, month, date),
            (2026, 9, 7) | (2026, 9, 8) | (2035, 12, 31)
        )
}

fn endpoint(left: i32, right: i32, through: i32) -> Check {
    require(
        left == through + 1 && right == through + 1,
        "Short or overshot target",
    )
}

fn retain(root: &Path, leg: &str, slot: &str, g: &Game, report: &mut Value) -> Check {
    let dir = root.join(leg);
    storage::write(&dir, slot, g)?;
    let relative = format!("{leg}/saves/{slot}.json");
    let bytes = fs::read(root.join(&relative)).map_err(|e| e.to_string())?;
    let canon = canonical_archive(std::str::from_utf8(&bytes).map_err(|e| e.to_string())?)?;
    report["artifacts"]
        .as_array_mut()
        .unwrap()
        .push(json!({"leg":leg,"kind":slot,"path":relative,
        "bytes":bytes.len(),"file_fnv64":fingerprint(&bytes),"canonical_fnv64":fingerprint(&canon),
        "date":day_label(clock::absolute_day(&g.world))}));
    Ok(())
}

fn run_pair(r: &Request, out: &Path, report: &mut Value) -> Check {
    let (nation, through) = validate_request(r)?;
    require(
        env!("SPHERES_REVISION") == &r.revision[..12],
        "Compiled revision does not match frozen request (modified builds are refused)",
    )?;
    let mut a = fresh(r.seed, nation)?;
    let mut b = fresh(r.seed, nation)?;
    let outcome = (|| -> Check {
        let aa = adopt(&mut a)?;
        let ab = adopt(&mut b)?;
        require(aa == ab, "Ordinary adoption outcomes diverged")?;
        report["initial_rules"] =
            serde_json::to_value(&a.world.rules).map_err(|e| e.to_string())?;
        report["actions"].as_array_mut().unwrap().push(json!({"kind":"competition_adoption","date":"1990-01-01","observation":aa,"legs":2,"matched":true}));
        report["checks"]["competition_adoptions"] = json!(2);
        compare(&a, &b, "opening", report)?;
        progress(out, report, "opening")?;
        let mut after_reload = None;
        while clock::absolute_day(&a.world) <= through {
            paired_continuation(&mut a, &mut b, false, report)?;
            paired_day(&mut a, &mut b, report, false)?;
            let now = clock::absolute_day(&a.world);
            report["days_each_leg"] = json!(now);
            report["end_native_date"] = json!(day_label(now));
            if after_reload == Some(now) {
                compare(&a, &b, "day_after_reload", report)?;
                after_reload = None;
            }
            if b.world.day == 1 {
                storage::write(&out.join("resumed"), "monthly", &b)?;
                b = storage::read(&out.join("resumed"), "monthly", false)?;
                bump(report, "monthly_reloads", 1);
                compare(&a, &b, "monthly_after_reload", report)?;
                after_reload = Some(now + 1);
            }
            if mandatory(now) {
                compare(&a, &b, "mandatory", report)?;
            }
            if b.world.day == 1 || mandatory(now) {
                progress(out, report, "checkpoint")?;
            }
        }
        endpoint(
            clock::absolute_day(&a.world),
            clock::absolute_day(&b.world),
            through,
        )?;
        compare(&a, &b, "terminal", report)?;
        retain(out, "uninterrupted", "final", &a, report)?;
        retain(out, "resumed", "final", &b, report)?;
        let terminal_bytes = equivalent(&a, &b)?;
        a = storage::read(&out.join("uninterrupted"), "final", false)?;
        b = storage::read(&out.join("resumed"), "final", false)?;
        require(
            equivalent(&a, &b)? == terminal_bytes,
            "Terminal load changed the complete saved archive",
        )?;
        report["checks"]["terminal_reloads"] = json!(2);
        compare(&a, &b, "terminal_reload", report)?;
        if r.through == "2035-12-31" {
            require(
                day_label(clock::absolute_day(&a.world)) == "2036-01-01"
                    && campaign_journey::pause_reason(&a).is_some()
                    && campaign_journey::pause_reason(&b).is_some()
                    && !a.journey.beyond_2035
                    && !b.journey.beyond_2035,
                "Complete horizon must pause before sandbox",
            )?;
            report["checks"]["full_horizon_pause"] = json!(true);
            let previous = equivalent(&a, &b)?;
            let ia = a.advance_days(1, vec![]);
            let ib = b.advance_days(1, vec![]);
            require(
                ia.1.is_some() && ib.1.is_some() && equivalent(&a, &b)? == previous,
                "Unconfirmed horizon advanced or changed archive",
            )?;
            paired_continuation(&mut a, &mut b, true, report)?;
            // A ceased observer remains a legitimate observer after the horizon.
            paired_continuation(&mut a, &mut b, false, report)?;
            paired_day(&mut a, &mut b, report, true)?;
            compare(&a, &b, "sandbox_continuation", report)?;
            retain(out, "uninterrupted", "sandbox", &a, report)?;
            retain(out, "resumed", "sandbox", &b, report)?;
            report["checks"]["sandbox_continued"] = json!(true);
            report["sandbox_end_native_date"] = json!(day_label(clock::absolute_day(&a.world)));
        }
        Ok(())
    })();
    if let Err(reason) = &outcome {
        report["failure"] = json!(reason);
        report["failure_native_dates"] = json!([
            day_label(clock::absolute_day(&a.world)),
            day_label(clock::absolute_day(&b.world))
        ]);
        // Preserve the actual failing worlds when they still serialize; retain
        // the original diagnostic even if the failing state cannot be saved.
        for (name, g) in [("uninterrupted", &a), ("resumed", &b)] {
            if let Err(e) = retain(out, name, "failure", g, report) {
                report["artifact_errors"]
                    .as_array_mut()
                    .unwrap()
                    .push(json!({"leg":name,"error":e}));
            }
        }
    }
    outcome
}

#[test]
#[ignore = "Explicit native stability request and new output directory; run only through the frozen matrix orchestrator"]
fn s25_stability_cell() {
    let request =
        PathBuf::from(std::env::var_os("SPHERES_S25_REQUEST").expect("SPHERES_S25_REQUEST"));
    let out = PathBuf::from(std::env::var_os("SPHERES_S25_OUT").expect("SPHERES_S25_OUT"));
    assert!(
        request.is_absolute() && request.is_file() && out.is_absolute() && !out.exists(),
        "Absolute immutable request and NEW output directory required"
    );
    let raw = fs::read(&request).expect("Read request");
    let r: Request = serde_json::from_slice(&raw).expect("Strict stability request JSON");
    validate_request(&r).expect("Valid stability request");
    fs::create_dir(&out).expect("Create only the new cell output directory");
    fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(out.join("progress.jsonl"))
        .unwrap();
    let mut report = json!({"format":"spheres-stability-result/v1","id":r.id,"country":r.country,"seed":r.seed,
        "through":r.through,"revision":r.revision,"compiled_revision":env!("SPHERES_REVISION"),"passed":false,
        "scope":"Native paired stability cell only; no S25 or CP1 certification",
        "start_native_date":"1990-01-01","end_native_date":"1990-01-01","days_each_leg":0,"legs":2,
        "comparisons":[],"actions":[],"artifacts":[],"artifact_errors":[],"failure":null,
        "checks":{"daily_invariant_checks":0,"sandbox_daily_invariant_checks":0,"native_validation_checks":0,
            "monthly_reloads":0,"terminal_reloads":0,"competition_adoptions":0,"full_horizon_pause":null,"sandbox_continued":false},
        "provenance":{"request_path":request,"request_bytes":raw.len(),"request_fnv64":fingerprint(&raw),
            "request_unchanged":false,"fingerprint_scope":"FNV64 is diagnostic only; the matrix wrapper supplies SHA256 provenance",
            "initialization":"Real new_game; ordinary competition adoption; existing budget allocations/departments renewed through dated native commands",
            "archive_exception":"Only terminal top-level saved_unix removed; remaining native encode bytes, key order and numeric spelling unchanged",
            "cessation_policy":"Ordinary Russia continuation only for a dissolved USSR with an eligible living Russia; otherwise ordinary observer continuation. No grants or forced dissolution.",
            "retention":"Two final native archives plus two sandbox archives for full-horizon cells; resumed/saves/monthly.json plus its native backup are bounded scratch; failing worlds retained when serializable"}});
    write_report(&out, &report).unwrap();
    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        run_pair(&r, &out, &mut report)
    }));
    let failure = match outcome {
        Ok(Ok(())) => None,
        Ok(Err(e)) => Some(e),
        Err(payload) => Some(format!(
            "Native panic: {}",
            payload
                .downcast_ref::<String>()
                .map(String::as_str)
                .or_else(|| payload.downcast_ref::<&str>().copied())
                .unwrap_or("non-string panic")
        )),
    };
    let unchanged = fs::read(&request).ok().as_deref() == Some(raw.as_slice());
    report["provenance"]["request_unchanged"] = json!(unchanged);
    report["failure"] =
        json!(failure.or_else(|| (!unchanged).then(|| "Immutable request changed".to_owned())));
    report["passed"] = json!(report["failure"].is_null());
    progress(&out, &report, "finished").expect("Retain final result");
    assert_eq!(
        report["passed"],
        true,
        "Stability cell failed; inspect {}: {}",
        out.join("result.json").display(),
        report["failure"]
    );
}

#[test]
fn stability_request_rejects_missing_identity_bad_calendar_and_unapproved_cells() {
    let good = json!({"format":"spheres-stability-cell/v1","id":"pilot-fr-1990","country":"France","seed":1990,"through":"1991-02-02","revision":"a".repeat(40)});
    let parse = |value: Value| -> Check {
        let r: Request = serde_json::from_value(value).map_err(|e| e.to_string())?;
        validate_request(&r).map(|_| ())
    };
    assert!(parse(good.clone()).is_ok());
    for field in ["id", "country", "seed", "through", "revision"] {
        let mut v = good.clone();
        v.as_object_mut().unwrap().remove(field);
        assert!(parse(v).is_err(), "{field}");
    }
    for (key, value) in [
        ("through", json!("2036-01-01")),
        ("through", json!("1991-02-29")),
        ("through", json!("1990-1-1")),
        ("country", json!("Russia")),
        ("seed", json!(17)),
        ("revision", json!("a".repeat(12))),
        ("id", json!("../escape")),
    ] {
        let mut v = good.clone();
        v[key] = value;
        assert!(parse(v).is_err(), "{key}");
    }
    assert_eq!(
        through_day("2035-12-31").unwrap() + 1,
        clock::date_day(2036, 1, 1)
    );
}

#[test]
fn stability_invariants_reject_nonfinite_negative_money_and_invalid_population() {
    let g = fresh(1990, NationId::France).unwrap();
    assert!(cheap_invariants(&g.world).is_ok());
    for value in [f64::NAN, f64::INFINITY, -1.0] {
        let mut w = g.world.clone();
        w.nation_mut(NationId::France).treasury_bn = Some(value);
        assert!(cheap_invariants(&w).is_err());
        let mut w = g.world.clone();
        w.nation_mut(NationId::France).debt_bn = Some(value);
        assert!(cheap_invariants(&w).is_err());
        let mut w = g.world.clone();
        w.nation_mut(NationId::France).population = value;
        assert!(cheap_invariants(&w).is_err());
    }
}

#[test]
fn stability_archive_comparison_excludes_only_wall_clock_and_detects_finance_history_journey() {
    let g = fresh(7, NationId::Tonga).unwrap();
    let original = storage::encode(&g).unwrap();
    let (prefix, _) = original.rsplit_once(",\"saved_unix\":").unwrap();
    let canon = canonical_archive(&original).unwrap();
    assert_eq!(
        canon,
        canonical_archive(&format!("{prefix},\"saved_unix\":1}}")).unwrap()
    );
    for key in [
        "world",
        "history",
        "log",
        "journey",
        "history_epoch",
        "saved_date",
        "player",
    ] {
        let changed = original.replacen(&format!("\"{key}\":"), &format!("\"{key}_changed\":"), 1);
        assert!(canonical_archive(&changed).unwrap() != canon, "{key}");
    }
    for extra in [
        ",\"future_field\":0.0",
        ",\"future_field\":-0.0",
        ",\"future_field\":0e0",
    ] {
        assert!(
            canonical_archive(&format!("{prefix}{extra},\"saved_unix\":1}}")).unwrap() != canon
        );
    }
    let positive =
        canonical_archive(&format!("{prefix},\"future_field\":0.0,\"saved_unix\":1}}")).unwrap();
    let negative = canonical_archive(&format!(
        "{prefix},\"future_field\":-0.0,\"saved_unix\":2}}"
    ))
    .unwrap();
    assert_ne!(positive, negative, "Signed zero must not be normalized");
    for tail in [
        "",
        ",\"saved_unix\":-1}",
        ",\"saved_unix\":null}",
        ",\"saved_unix\":1,\"later\":1}",
    ] {
        assert!(canonical_archive(&format!("{prefix}{tail}")).is_err());
    }
}

#[test]
fn stability_archive_mismatch_digests_are_lazy_and_match_the_legacy_oracle() {
    // Literal previous comparison path: preserve its exact returned bytes and
    // mismatch text while avoiding its eager diagnostic work on equal states.
    fn legacy_equivalent(left: &Game, right: &Game) -> Check<Vec<u8>> {
        let a = canonical_archive(&storage::encode(left)?)?;
        let b = canonical_archive(&storage::encode(right)?)?;
        require(a == b, format!("Complete archive mismatch at {}: bytes {}/{}, FNV {}/{}; no finance/history fields excluded",
            day_label(clock::absolute_day(&left.world)), a.len(), b.len(), fingerprint(&a), fingerprint(&b)))?;
        Ok(a)
    }

    let mut left = fresh(7, NationId::Tonga).unwrap();
    let mut right = fresh(7, NationId::Tonga).unwrap();
    left.record("A complete native archive comparison.".into());
    right.record("A complete native archive comparison.".into());
    native_invariants(&left.world).unwrap();
    native_invariants(&right.world).unwrap();

    let expected = legacy_equivalent(&left, &right).unwrap();
    let mut calls = 0;
    let actual = equivalent_with_diagnostic_digest(&left, &right, |bytes| {
        calls += 1;
        fingerprint(bytes)
    }).unwrap();
    assert_eq!(calls, 0, "Matching archives must not compute mismatch diagnostics");
    assert!(actual == expected, "Matching return bytes changed from the legacy path");
    assert!(equivalent(&left, &right).unwrap() == expected,
        "The ordinary wrapper must preserve the complete native archive");

    // A same-length log change remains valid native campaign data. Length or
    // world-only equality cannot detect it; the whole archive must differ.
    right.log.last_mut().unwrap().text = "B complete native archive comparison.".into();
    let changed = storage::encode(&right).unwrap();
    assert!(storage::decode(&changed).is_ok(), "Mismatch fixture must be a valid native archive");
    assert_eq!(canonical_archive(&changed).unwrap().len(), expected.len());
    let expected_error = legacy_equivalent(&left, &right).unwrap_err();
    calls = 0;
    let actual_error = equivalent_with_diagnostic_digest(&left, &right, |bytes| {
        calls += 1;
        fingerprint(bytes)
    }).unwrap_err();
    assert_eq!(calls, 2, "A mismatch must retain both diagnostic fingerprints");
    assert_eq!(actual_error, expected_error, "Legacy mismatch details changed");
    assert_eq!(equivalent(&left, &right).unwrap_err(), expected_error);

    // Even identical diagnostic digests cannot turn unequal bytes into a pass.
    assert!(equivalent_with_diagnostic_digest(&left, &right, |_| "same".into()).is_err());
}

#[test]
fn stability_endpoint_and_checkpoint_schedule_reject_short_or_overshot_coverage() {
    let through = through_day("1991-02-02").unwrap();
    assert!(endpoint(through + 1, through + 1, through).is_ok());
    for (a, b) in [
        (through, through + 1),
        (through + 1, through),
        (through + 2, through + 1),
        (through + 1, through + 2),
    ] {
        assert!(endpoint(a, b, through).is_err());
    }
    for (year, month, day) in [
        (1991, 1, 1),
        (2000, 1, 1),
        (2026, 9, 7),
        (2026, 9, 8),
        (2035, 12, 31),
    ] {
        assert!(mandatory(clock::date_day(year, month, day)));
    }
    assert!(!mandatory(clock::date_day(2026, 9, 9)));
}

#[test]
fn stability_real_opening_and_one_day_preserve_complete_archive_across_native_reload() {
    let mut a = fresh(7, NationId::Tonga).unwrap();
    let mut b = fresh(7, NationId::Tonga).unwrap();
    assert_eq!(adopt(&mut a).unwrap(), adopt(&mut b).unwrap());
    b = storage::decode(&storage::encode(&b).unwrap()).unwrap();
    assert!(equivalent(&a, &b).is_ok());
    let ca = renewal(&a).unwrap();
    let cb = renewal(&b).unwrap();
    assert_eq!(one_day(&mut a, ca).unwrap(), one_day(&mut b, cb).unwrap());
    native_invariants(&a.world).unwrap();
    native_invariants(&b.world).unwrap();
    assert!(equivalent(&a, &b).is_ok());
    b.world.nation_mut(NationId::Tonga).political_capital += 1.0;
    assert!(equivalent(&a, &b).is_err());
}

// Separate from the ordinary long-campaign seed matrix. This is the existing
// S21/S24 N1 authored collapse, now used to test interruption boundaries. It
// cannot establish organic historical timing or qualify either S24 or S25.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SuccessionRequest {
    format: String,
    id: String,
    seed: u64,
    revision: String,
}

fn validate_succession_request(r: &SuccessionRequest) -> Check {
    require(r.format == "spheres-controlled-succession-request/v1", "Unsupported controlled succession request")?;
    require(r.id == "controlled-ussr-russia-1990" && r.seed == 1990,
        "This controlled case is exactly the existing USSR/Russia N1 seed-1990 fixture")?;
    require(r.revision.len() == 40 && r.revision.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)),
        "revision must be a full lowercase Git identity")
}

fn succession_fixture() -> Check<Game> {
    // Copy the setup of campaign_journey::tests::dissolved, stopping BEFORE
    // its ordinary day. Those are the only two directly authored world fields.
    let mut g = Game::new_fresh(1990, Some(NationId::USSR));
    fresh_play_rules(&mut g)?;
    spheres_sim::campaign_aims::choose(&mut g.world, NationId::USSR,
        spheres_sim::campaign_aims::Aim::Prosperity)?;
    g.world.nation_mut(NationId::USSR).stability = 0.0;
    g.world.nation_mut(NationId::USSR).separatism = 1.0;
    require(clock::absolute_day(&g.world) == 0 && g.world.player == Some(NationId::USSR)
        && g.world.nation(NationId::USSR).alive && !g.world.has_flag("ussr_dissolved")
        && campaign_journey::pause_reason(&g).is_none(), "Authored fixture did not start before dissolution")?;
    Ok(g)
}

fn selected_russia(g: &Game) -> Check {
    require(g.world.player == Some(NationId::Russia) && !g.journey.observing
        && !g.journey.beyond_2035 && g.world.has_flag("ussr_dissolved")
        && !g.world.nation(NationId::USSR).alive
        && g.world.nation_opt(NationId::Russia).is_some_and(|n| n.alive)
        && campaign_journey::pause_reason(g).is_none(),
        "A real live Russia succession is required; observer or unchanged USSR cannot pass")?;
    require(g.journey.transitions.len() == 1
        && g.journey.transitions[0].from == NationId::USSR
        && g.journey.transitions[0].to == NationId::Russia
        && g.journey.transitions[0].date == "2 Jan 1990",
        "Exactly the actual USSR-to-Russia journey transition must survive")?;
    require(g.world.districts.values().any(|id| *id == NationId::Russia)
        && !g.world.districts.values().any(|id| *id == NationId::USSR)
        && spheres_sim::government::state(&g.world, NationId::Russia).is_some(),
        "Successor ownership/government missing or territory remains assigned to ceased USSR")?;
    require(g.world.campaign_aims.active.is_none() && g.world.campaign_aims.history.len() == 1
        && g.world.campaign_aims.history[0].goal.nation == NationId::USSR
        && g.world.campaign_aims.history[0].goal.aim == spheres_sim::campaign_aims::Aim::Prosperity
        && g.world.campaign_aims.history[0].goal.completed_day.is_none()
        && g.world.campaign_aims.history[0].ended_day == 1
        && g.world.campaign_aims.history[0].outcome == "government ended",
        "The USSR aim must remain archived as government-ended, never transferred to or awarded to Russia")
}

fn succession_observation(g: &Game, kind: &str) -> Check<Value> {
    let pin = |v: Value| -> Check<String> {
        Ok(fingerprint(&serde_json::to_vec(&v).map_err(|e| e.to_string())?))
    };
    Ok(json!({"kind":kind,"date":day_label(clock::absolute_day(&g.world)),
        "player":g.world.player,"ussr_alive":g.world.nation(NationId::USSR).alive,
        "russia_alive":g.world.nation_opt(NationId::Russia).is_some_and(|n| n.alive),
        "dissolved":g.world.has_flag("ussr_dissolved"),"paused":campaign_journey::pause_reason(g).is_some(),
        "observing":g.journey.observing,"journey":g.journey,
        "ussr_districts":g.world.districts.values().filter(|id| **id == NationId::USSR).count(),
        "russia_districts":g.world.districts.values().filter(|id| **id == NationId::Russia).count(),
        "russia_nation_fnv64":pin(json!(g.world.nation_opt(NationId::Russia)))?,
        "russia_government_fnv64":pin(json!(spheres_sim::government::state(&g.world,NationId::Russia)))?,
        "ownership_fnv64":pin(json!(g.world.districts))?,
        "history_rows":g.history.len(),"log_rows":g.log.len(),
        "reading_scope":"Diagnostic fingerprints only; comparison covers every full archive byte including all nations, government, property, finances, forces, ammunition, histories and journey."}))
}

fn succession_checkpoint(a: &Game, b: &mut Game, out: &Path, slot: &str, report: &mut Value) -> Check {
    compare(a, b, &format!("{slot}_before_reload"), report)?;
    let before = equivalent(a, b)?;
    retain(out, "uninterrupted", slot, a, report)?;
    retain(out, "resumed", slot, b, report)?;
    *b = storage::read(&out.join("resumed"), slot, false)?;
    require(equivalent(a, b)? == before, "Resume normalized or changed complete checkpoint archive")?;
    bump(report, "scheduled_reloads", 1);
    compare(a, b, &format!("{slot}_after_reload"), report)?;
    report["observations"].as_array_mut().unwrap().push(succession_observation(a, slot)?);
    progress(out, report, slot)
}

fn succession_day(g: &mut Game) -> Check<Value> {
    let before = clock::absolute_day(&g.world);
    let payload = json!({"session_id":g.session_id,"player_context":g.world.player,"days":1,"commands":[]});
    let result = transport::advance_request(g, &payload).map_err(|e| e.message)?;
    require(clock::absolute_day(&g.world) == before + 1, "Released daily advance did not settle exactly one day")?;
    require(result.get("interrupt").is_some(), "Released daily response omitted interruption status")?;
    cheap_invariants(&g.world)?;
    // The complete archives are checked separately. Transport sessions are not
    // campaign data and deliberately differ after a genuine load/restart.
    Ok(json!({"from":day_label(before),"to":day_label(before+1),
        "interruption":result["interrupt"],"requested_days":1,"commands":[]}))
}

fn controlled_refusals(g: &mut Game) -> Check<Value> {
    let before = canonical_archive(&storage::encode(g)?)?;
    let p = json!({"session_id":g.session_id,"player_context":g.world.player,"days":1,"commands":[]});
    let turn = transport::advance_request(g, &p).err().ok_or("Paused successor choice allowed a turn")?;
    require(turn.not_applied, "Refused paused turn has ambiguous application status")?;
    let p = json!({"session_id":g.session_id,"player_context":g.world.player,"commands":[
        {"kind":"continue_campaign","action":"successor","target":"France","player":g.world.player,"date":g.world.date_str()}]});
    let choice = transport::immediate_request(g, &p).err().ok_or("Unrelated France accepted as USSR successor")?;
    require(choice.not_applied && canonical_archive(&storage::encode(g)?)? == before,
        "Refused turn or illegal successor changed the complete archive")?;
    Ok(json!({"paused_turn":turn.message,"invalid_successor":choice.message,"not_applied":true,"archive_unchanged":true}))
}

fn served_russia_choice(g: &mut Game) -> Check<Value> {
    let summary = campaign_journey::summary(g);
    let offers = summary["actions"].as_array().ok_or("Missing served continuation choices")?;
    let choices: Vec<_> = offers.iter().filter(|o| o["command"]["action"] == "successor"
        && o["command"]["target"] == "Russia").collect();
    require(choices.len() == 1, "Exactly one served Russia continuation is required")?;
    let command = choices[0]["command"].clone();
    let nation_before = serde_json::to_vec(g.world.nation(NationId::Russia)).map_err(|e|e.to_string())?;
    let government_before = serde_json::to_vec(&spheres_sim::government::state(&g.world,NationId::Russia)).map_err(|e|e.to_string())?;
    let ownership_before = g.world.districts.clone();
    let history_before = serde_json::to_vec(&g.history).map_err(|e|e.to_string())?;
    let aim_before = g.world.campaign_aims.active.clone().ok_or("Authored USSR aim disappeared before successor choice")?;
    let date = clock::absolute_day(&g.world);
    let payload = json!({"session_id":g.session_id,"player_context":g.world.player,"client_id":"controlled-succession",
        "request_seq":1,"commands":[command.clone()]});
    let response = transport::immediate_request(g,&payload).map_err(|e|e.message)?;
    require(response["errors"] == json!([]) && response["command_replayed"] == false, "Served Russia command was refused or already replayed")?;
    selected_russia(g)?;
    require(g.world.campaign_aims.history[0].goal == aim_before, "Continuation rewrote the original USSR aim record")?;
    require(clock::absolute_day(&g.world) == date
        && serde_json::to_vec(g.world.nation(NationId::Russia)).map_err(|e|e.to_string())? == nation_before
        && serde_json::to_vec(&spheres_sim::government::state(&g.world,NationId::Russia)).map_err(|e|e.to_string())? == government_before
        && g.world.districts == ownership_before && serde_json::to_vec(&g.history).map_err(|e|e.to_string())? == history_before,
        "Continuation granted or changed successor finance, property, forces, ammunition, government, history or calendar")?;
    let before_replay = canonical_archive(&storage::encode(g)?)?;
    let replay = transport::immediate_request(g,&payload).map_err(|e|e.message)?;
    require(replay["command_replayed"] == true && canonical_archive(&storage::encode(g)?)? == before_replay,
        "Lost-response replay duplicated the succession or changed campaign state")?;
    Ok(json!({"command":command,"served_label":choices[0]["label"],"replay_unchanged":true,"successor_state_unchanged":true,
        "legacy_aim_archived":true,"legacy_aim":g.world.campaign_aims.history[0]}))
}

fn run_controlled_succession(r: &SuccessionRequest, out: &Path, report: &mut Value) -> Check {
    validate_succession_request(r)?;
    require(env!("SPHERES_REVISION") == &r.revision[..12], "Compiled revision differs from controlled request; modified builds refused")?;
    let mut a = succession_fixture()?;
    let mut b = succession_fixture()?;
    report["initial_rules"] = json!(a.world.rules);
    let outcome = (|| -> Check {
        succession_checkpoint(&a,&mut b,out,"before_collapse",report)?;
        let aa = succession_day(&mut a)?;
        let ab = succession_day(&mut b)?;
        require(aa == ab, "Dissolution turn interruptions differ")?;
        report["actions"].as_array_mut().unwrap().push(json!({"kind":"collapse_day","observation":aa,"legs":2,"matched":true}));
        bump(report,"daily_invariant_checks",2);
        report["days_each_leg"] = json!(1);
        report["end_native_date"] = json!("1990-01-02");
        for g in [&a,&b] {
            require(g.world.player == Some(NationId::USSR) && !g.world.nation(NationId::USSR).alive
                && g.world.has_flag("ussr_dissolved") && campaign_journey::pause_reason(g).is_some()
                && !g.journey.observing && g.journey.transitions.is_empty(),
                "Actual dissolution must pause the ceased USSR before any successor choice")?;
        }
        succession_checkpoint(&a,&mut b,out,"paused_succession",report)?;
        let ra = controlled_refusals(&mut a)?;
        let rb = controlled_refusals(&mut b)?;
        require(ra == rb, "Ordinary refusal outcomes differ")?;
        report["actions"].as_array_mut().unwrap().push(json!({"kind":"legal_refusals","observation":ra,"legs":2,"matched":true}));
        let ca = served_russia_choice(&mut a)?;
        let cb = served_russia_choice(&mut b)?;
        require(ca == cb, "Served Russia continuation outcomes differ")?;
        report["actions"].as_array_mut().unwrap().push(json!({"kind":"served_russia_choice","observation":ca,"legs":2,"matched":true}));
        succession_checkpoint(&a,&mut b,out,"after_russia_choice",report)?;
        for day in 1..=7 {
            selected_russia(&a)?; selected_russia(&b)?;
            let aa = succession_day(&mut a)?;
            let ab = succession_day(&mut b)?;
            require(aa == ab, "Successor day interruptions differ")?;
            report["actions"].as_array_mut().unwrap().push(json!({"kind":"successor_day","index":day,"observation":aa,"legs":2,"matched":true}));
            bump(report,"daily_invariant_checks",2);
            report["days_each_leg"] = json!(day + 1);
            report["successor_days_each_leg"] = json!(day);
            report["end_native_date"] = json!(day_label(clock::absolute_day(&a.world)));
            selected_russia(&a)?; selected_russia(&b)?;
            succession_checkpoint(&a,&mut b,out,&format!("successor_day_{day}"),report)?;
        }
        require(clock::absolute_day(&a.world)==8 && clock::absolute_day(&b.world)==8,
            "Controlled case must end on 9 January after seven actual Russia days")?;
        require(report["checks"]["scheduled_reloads"]==10 && report["artifacts"].as_array().unwrap().len()==20,
            "Every required boundary must retain both archives and resume the scheduled leg")?;
        Ok(())
    })();
    if let Err(reason) = &outcome {
        report["failure"] = json!(reason);
        for (leg,g) in [("uninterrupted",&a),("resumed",&b)] {
            if let Err(e) = retain(out,leg,"failure",g,report) {
                report["artifact_errors"].as_array_mut().unwrap().push(json!({"leg":leg,"error":e}));
            }
        }
    }
    outcome
}

#[test]
#[ignore = "Explicit authored paired succession preflight; requires immutable request and NEW output, never organic history or S25 qualification"]
fn s25_controlled_ussr_russia_continuity() {
    let request = PathBuf::from(std::env::var_os("SPHERES_S25_SUCCESSION_REQUEST").expect("SPHERES_S25_SUCCESSION_REQUEST"));
    let out = PathBuf::from(std::env::var_os("SPHERES_S25_SUCCESSION_OUT").expect("SPHERES_S25_SUCCESSION_OUT"));
    assert!(request.is_absolute() && request.is_file() && out.is_absolute() && !out.exists(), "Absolute request and NEW output required");
    let raw = fs::read(&request).unwrap();
    let r: SuccessionRequest = serde_json::from_slice(&raw).expect("Strict controlled request");
    validate_succession_request(&r).unwrap();
    fs::create_dir(&out).unwrap();
    fs::OpenOptions::new().create_new(true).write(true).open(out.join("progress.jsonl")).unwrap();
    let mut report = json!({"format":"spheres-controlled-succession-result/v1","id":r.id,"seed":r.seed,
        "revision":r.revision,"compiled_revision":env!("SPHERES_REVISION"),"passed":false,"failure":null,
        "qualification":false,"s25_complete":false,"organic_history":false,"fixture":true,
        "scope":"Controlled paired authored USSR-to-Russia continuity, separate from the ordinary 24-cell seed matrix",
        "setup":{"recipe":"S24 N1 / campaign_journey::tests::dissolved","constructor":"Game::new_fresh(1990, Some(USSR)); fresh_play_rules",
            "native_aim":"Prosperity","authored_fields":[{"nation":"USSR","field":"stability","value":0.0},{"nation":"USSR","field":"separatism","value":1.0}],
            "other_direct_world_edits":0,"no_competition_adoption":true},
        "start_native_date":"1990-01-01","end_native_date":"1990-01-01","days_each_leg":0,"successor_days_each_leg":0,"legs":2,
        "comparisons":[],"observations":[],"actions":[],"artifacts":[],"artifact_errors":[],
        "checks":{"native_validation_checks":0,"daily_invariant_checks":0,"scheduled_reloads":0},
        "provenance":{"request_path":request,"request_bytes":raw.len(),"request_fnv64":fingerprint(&raw),"request_unchanged":false,
            "archive_exception":"Only terminal top-level saved_unix; every other encoded byte retained",
            "fingerprint_scope":"FNV64 diagnostic; external runner supplies cryptographic pins",
            "no_observer_fallback":true,"uninterrupted_leg_never_loaded":true}});
    write_report(&out,&report).unwrap();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(||run_controlled_succession(&r,&out,&mut report)));
    let failure = match result { Ok(Ok(()))=>None, Ok(Err(e))=>Some(e), Err(p)=>Some(format!("Native panic: {}",
        p.downcast_ref::<String>().map(String::as_str).or_else(||p.downcast_ref::<&str>().copied()).unwrap_or("non-string panic"))) };
    let unchanged = fs::read(&request).ok().as_deref()==Some(raw.as_slice());
    report["provenance"]["request_unchanged"] = json!(unchanged);
    report["failure"] = json!(failure.or_else(||(!unchanged).then(||"Immutable request changed".into())));
    report["passed"] = json!(report["failure"].is_null());
    progress(&out,&report,"finished").unwrap();
    assert!(report["passed"]==true,"Controlled succession failed: {} (inspect {})",report["failure"],out.display());
}

#[test]
fn controlled_succession_request_rejects_other_fixture_identity_and_unknown_fields() {
    let good=json!({"format":"spheres-controlled-succession-request/v1","id":"controlled-ussr-russia-1990","seed":1990,"revision":"a".repeat(40)});
    let valid=|v:Value|serde_json::from_value::<SuccessionRequest>(v).map_err(|e|e.to_string()).and_then(|r|validate_succession_request(&r));
    assert!(valid(good.clone()).is_ok());
    for (key,value) in [("format",json!("spheres-stability-cell/v1")),("id",json!("organic-ussr")),("seed",json!(7)),("revision",json!("a".repeat(12))),("through",json!("2035-12-31"))] {
        let mut changed=good.clone(); changed[key]=value; assert!(valid(changed).is_err(),"{key}");
    }
}

#[test]
fn controlled_succession_comparison_detects_each_required_state_family() {
    let a=fresh(1990,NationId::USSR).unwrap();
    for family in ["finance","forces","ammunition","ownership","government","history","log","journey"] {
        let mut b=storage::decode(&storage::encode(&a).unwrap()).unwrap();
        assert!(equivalent(&a,&b).is_ok(),"Unmodified fixture must roundtrip before {family}");
        match family {
            "finance"=>{let n=b.world.nation_mut(NationId::USSR); n.treasury_bn=Some(n.treasury_bn.unwrap_or(0.0)+1.0);},
            "forces"=>b.world.nation_mut(NationId::USSR).mil_strength+=1.0,
            "ammunition"=>b.world.nation_mut(NationId::USSR).munitions=0.123,
            "ownership"=>{let id=b.world.districts.values_mut().next().unwrap(); *id=if *id==NationId::Tonga {NationId::France}else{NationId::Tonga};},
            "government"=>b.world.governments.states[0].coup_pressure+=0.123,
            "history"=>b.history.clear(),
            "log"=>b.record("Synthetic comparison mutation".into()),
            "journey"=>b.journey.observing=true,
            _=>unreachable!(),
        }
        assert!(equivalent(&a,&b).is_err(),"Full archive must detect {family}");
    }
    assert!(selected_russia(&a).is_err(),"An unchanged USSR cannot pass successor selection");
}

#[test]
fn controlled_succession_guards_reject_observer_missing_journey_and_revival() {
    let mut g=succession_fixture().unwrap();
    succession_day(&mut g).unwrap();
    controlled_refusals(&mut g).unwrap();
    served_russia_choice(&mut g).unwrap();
    selected_russia(&g).unwrap();
    for field in ["observer","missing_journey","wrong_player","parent_revived","missing_ownership"] {
        let mut changed=storage::decode(&storage::encode(&g).unwrap()).unwrap();
        selected_russia(&changed).unwrap();
        match field {
            "observer"=>changed.journey.observing=true,
            "missing_journey"=>changed.journey.transitions.clear(),
            "wrong_player"=>changed.world.player=Some(NationId::USSR),
            "parent_revived"=>changed.world.nation_mut(NationId::USSR).alive=true,
            "missing_ownership"=>{for id in changed.world.districts.values_mut(){if *id==NationId::Russia{*id=NationId::Ukraine;}}},
            _=>unreachable!(),
        }
        assert!(selected_russia(&changed).is_err(),"Must reject {field}");
    }
}
