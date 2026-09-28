//! Opt-in test instrumentation for the already-used A1 development seed 0.
//! Every hook is cfg(test); release code/state and the acceptance test are unchanged.
//! Stage labels name the branch actually reached, not a reconstructed prediction.
use super::*;
use crate::world::GameRules;
use serde_json::{json, Value};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

thread_local! {
    static ROWS: RefCell<Option<Vec<Value>>> = const { RefCell::new(None) };
}

struct Session;
impl Session {
    fn start() -> Self {
        ROWS.with(|rows| {
            let mut rows = rows.borrow_mut();
            assert!(rows.is_none(), "nested A1 observer session");
            *rows = Some(Vec::new());
        });
        Self
    }
    fn take(&self) -> Vec<Value> {
        ROWS.with(|rows| std::mem::take(rows.borrow_mut().as_mut().expect("active observer")))
    }
}
impl Drop for Session {
    fn drop(&mut self) {
        ROWS.with(|rows| *rows.borrow_mut() = None);
    }
}

pub(super) fn record(w: &WorldState, id: NationId, stage: &'static str) {
    // Return before any world read or allocation when the explicit test session is off.
    if !ROWS.with(|rows| rows.borrow().is_some()) {
        return;
    }
    let row = snapshot(w, id, stage);
    ROWS.with(|rows| {
        rows.borrow_mut()
            .as_mut()
            .expect("active observer")
            .push(row)
    });
}

fn snapshot(w: &WorldState, id: NationId, stage: &str) -> Value {
    let Some(n) = w.nation_opt(id) else {
        return json!({"stage":stage,"country":id.code(),"date":[w.year,w.month,w.day],"missing_nation":true});
    };
    let g = state(w, id);
    let has_army = g.is_some_and(|g| g.pillars.iter().any(|(p, _)| *p == Pillar::Army));
    let pn = pains(w, id);
    let conditions = crate::economy::Conditions::of(w, id);
    let terms = crate::economy::growth_terms(n, n.state_invest_gdp, n.interest_rate, &conditions);
    let fiscal = crate::economy::Fiscal::of(n, &terms);
    let floor = ai_army_funding_floor(w, id);
    let funding_quote = floor.map(|share| {
        let command = crate::Command::SetMilSpend { nation: id, share };
        json!({"share":share,"price_pc":crate::price_of(w,&command),
               "standing_affordable":crate::affordable(w,&command),
               "clears_existing_hysteresis":share >= n.mil_spend_gdp + 0.001})
    });
    let army_target = has_army.then(|| pillar_targets(w, id, &[Pillar::Army])[0].1);
    json!({
        "stage":stage,"country":id.code(),"date":[w.year,w.month,w.day],
        "rng_state":w.rng.state,
        "rules":{"ideology_blocs":w.rules.ideology_blocs,"ideology_takeover":w.rules.ideology_takeover,
                 "daily_simulation":w.rules.daily_simulation,"crisis_intensity":w.rules.crisis_intensity},
        "government":{"alive":n.alive,"electoral":is_electoral(w,id),"has_army":has_army,
            "leader":g.and_then(|g|g.leader()),"regime_bloc":g.and_then(|g|g.regime_bloc),
            "elected":g.map(|g|g.elected),"unrestricted_mandate":g.map(|g|g.unrestricted_mandate),
            "awaiting_first_election":g.map(|g|g.awaiting_first_election),
            "months_in_office":g.map(|g|g.months_in_office),"settled_months":electoral_coup_settled_months(w,id),
            "next_election":g.map(|g|g.next_election),"record":g.and_then(|g|g.political_record.as_ref()),
            "authoritarianism":n.authoritarianism},
        "army":{"raw_loyalty":g.map(|g|g.loyalty(Pillar::Army)),
            "effective_loyalty":crate::blocs::effective_army_loyalty(w,id),"target":army_target,
            "resources_per_member":army_resources_per_member(w,id),
            "civilian_confidence_penalty":army_civilian_confidence_penalty(w,id),
            "executive_leverage":civilian_army_executive_leverage(w,id),
            "public_mandate":civilian_public_mandate(w,id),"war_exhaustion":n.war_exhaustion,
            "pressure":g.map(|g|g.coup_pressure)},
        "discontent":{"total":crate::blocs::discontent(w,id),"prices":pn.prices,
            "growth":pn.growth,"war":pn.war,"order_uncapped":pn.order},
        "fiscal":{"gdp_bn":n.gdp,"population_m":n.population,"mil_spend_gdp":n.mil_spend_gdp,
            "tax_rate":n.tax_rate,"state_invest_gdp":n.state_invest_gdp,"debt_gdp":n.debt_gdp,
            "revenue_gdp":fiscal.revenue_gdp,"spend_gdp":fiscal.spend_gdp,
            "interest_gdp":fiscal.interest_gdp,"balance_gdp":fiscal.balance_gdp,
            "affordable_military_share":(fiscal.balance_gdp+n.mil_spend_gdp).max(0.0),
            "political_capital":n.political_capital,"funding_quote":funding_quote},
        "budget_ownership":{"player":w.player==Some(id),"on_the_books":n.on_the_books(),
            "annual_plan":n.annual_budget.is_some(),"programs_enrolled":crate::programs::enrolled(w,id),
            "fiscal_recovery":crate::fiscal_recovery::enabled(w)},
        "thresholds":{"army":ELECTORAL_COUP_ARMY,"discontent":ELECTORAL_COUP_DISCONTENT,
            "settled_months":ELECTORAL_COUP_SETTLED,"pressure":1.0/w.rules.crisis_intensity.max(0.1)}
    })
}

fn world() -> WorldState {
    crate::init::world_1990(GameRules {
        seed: 0,
        ideology_blocs: true,
        ideology_takeover: true,
        ..GameRules::default()
    })
}

#[test]
fn observer_is_off_by_default_and_snapshot_leaves_complete_world_unchanged() {
    let w = world();
    assert!(ROWS.with(|rows| rows.borrow().is_none()));
    let before = crate::save(&w);
    record(&w, NationId::Pakistan, "disabled_probe");
    assert!(ROWS.with(|rows| rows.borrow().is_none()));
    {
        let session = Session::start();
        record(&w, NationId::Pakistan, "pure_probe");
        let rows = session.take();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0]["country"], "Pakistan");
        assert_eq!(rows[0]["stage"], "pure_probe");
        assert!(
            crate::save(&w) == before,
            "observer changed complete serialized world"
        );
    }
    assert!(ROWS.with(|rows| rows.borrow().is_none()));
}

#[test]
fn observer_captures_actual_guard_and_firing_branches_without_driving_them() {
    let id = NationId::Pakistan;
    for reason in [
        "trigger_takeover_disabled",
        "trigger_no_government",
        "trigger_unsettled_interim_or_no_army",
        "trigger_live_conditions_inactive",
        "trigger_pressure_not_ready",
        "trigger_firing",
    ] {
        let mut base = world();
        base.nation_mut(id).stability = 10.0;
        let g = state_mut(&mut base, id).unwrap();
        g.awaiting_first_election = false;
        g.months_in_office = 24;
        g.political_record = None;
        g.coup_pressure = 1.5;
        for (p, v) in &mut g.pillars {
            if *p == Pillar::Army {
                *v = 0.1;
            }
        }
        match reason {
            "trigger_takeover_disabled" => base.rules.ideology_takeover = false,
            "trigger_no_government" => base.governments.states.retain(|g| g.nation != id),
            "trigger_unsettled_interim_or_no_army" => {
                state_mut(&mut base, id).unwrap().awaiting_first_election = true
            }
            "trigger_live_conditions_inactive" => {
                for (p, v) in &mut state_mut(&mut base, id).unwrap().pillars {
                    if *p == Pillar::Army {
                        *v = 0.9;
                    }
                }
            }
            "trigger_pressure_not_ready" => state_mut(&mut base, id).unwrap().coup_pressure = 0.0,
            "trigger_firing" => {}
            _ => unreachable!(),
        }
        let mut observed = base.clone();
        let ordinary = maybe_electoral_coup(&mut base, id);
        let session = Session::start();
        let watched = maybe_electoral_coup(&mut observed, id);
        let rows = session.take();
        assert_eq!(ordinary, watched);
        assert_eq!(rows.len(), 1, "one actual trigger branch");
        assert_eq!(rows[0]["stage"], reason);
        assert!(
            crate::save(&base) == crate::save(&observed),
            "guard/firing observer changed world: {reason}"
        );
        if reason == "trigger_firing" {
            assert!(watched);
            assert!(rows[0]["army"]["effective_loyalty"].as_f64().unwrap() < ELECTORAL_COUP_ARMY);
            assert!(rows[0]["discontent"]["total"].as_f64().unwrap() >= ELECTORAL_COUP_DISCONTENT);
            assert!(observed
                .headlines
                .iter()
                .any(|h| h.contains("removes the elected government")));
        }
    }
}

#[test]
fn observed_army_walk_and_funding_keep_exact_control_state() {
    let id = NationId::Pakistan;
    let base = world();
    let mut plain = base.clone();
    let mut observed = base;
    electoral_army_tick(&mut plain, id);
    ai_government(&mut plain);
    let session = Session::start();
    electoral_army_tick(&mut observed, id);
    ai_government(&mut observed);
    let rows = session.take();
    assert!(
        crate::save(&plain) == crate::save(&observed),
        "walk/funding observer changed complete world"
    );
    assert!(rows.iter().any(|r| r["stage"] == "army_before_walk"));
    assert!(rows.iter().any(|r| r["stage"] == "army_after_walk"));
    assert!(rows.iter().any(|r| r["stage"] == "ai_before_funding"));
    assert!(rows.iter().any(|r| r["stage"] == "ai_after_funding"));
}

fn fnv(bytes: &[u8]) -> String {
    format!(
        "{:016x}",
        bytes
            .iter()
            .fold(0xcbf29ce484222325_u64, |h, b| (h ^ u64::from(*b))
                .wrapping_mul(0x100000001b3))
    )
}
fn write_json(path: &std::path::Path, value: &Value) {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .unwrap();
    serde_json::to_writer_pretty(&mut file, value).unwrap();
    file.write_all(b"\n").unwrap();
    file.sync_all().unwrap();
}

/// Diagnostic only: no seed/horizon knobs, no new cohort, no A1 acceptance claim.
#[test]
#[ignore]
fn a1_seed0_exact_firing_diagnosis() {
    let out = PathBuf::from(
        std::env::var_os("SPHERES_A1_OBSERVER_OUT").expect("SPHERES_A1_OBSERVER_OUT is required"),
    );
    assert!(out.is_absolute(), "output must be absolute");
    std::fs::create_dir(&out).expect("output must be a NEW directory with an existing parent");
    let file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(out.join("observations.jsonl"))
        .unwrap();
    let mut writer = BufWriter::new(file);
    write_json(
        &out.join("plan.json"),
        &json!({"format":"spheres-a1-firing-observer/v1","seed":0,"months":252,
        "qualification":false,"acceptance_test_changed":false,"rules":"GameRules default with ideology_blocs and ideology_takeover true",
        "comparison":"Complete native save bytes, RNG and headlines after each ordinary month, unobserved control versus observed leg",
        "source_provenance":"Build/library/executable/source hashes must be supplied by the invoking coordinator; this test does not infer Git identity."}),
    );
    let mut control = world();
    let mut observed = control.clone();
    let mut totals = BTreeMap::<String, u64>::new();
    let mut firing = Vec::<Value>::new();
    let mut comparisons = Vec::<Value>::new();
    let mut failure = None;
    let mut elected_coup_headlines = Vec::<Value>::new();
    for month in 0..252 {
        let plain_news = crate::tick_month(&mut control, &[]);
        let session = Session::start();
        let watched_news = crate::tick_month(&mut observed, &[]);
        let rows = session.take();
        drop(session);
        for row in rows {
            let stage = row["stage"].as_str().unwrap().to_owned();
            *totals.entry(stage.clone()).or_default() += 1;
            if stage == "trigger_firing" {
                firing.push(row.clone());
            }
            serde_json::to_writer(&mut writer, &row).unwrap();
            writer.write_all(b"\n").unwrap();
        }
        writer.flush().unwrap();
        for headline in &watched_news {
            if headline.starts_with("COUP IN ")
                && headline.contains("removes the elected government")
            {
                elected_coup_headlines.push(json!({"month":month+1,"headline":headline}));
            }
        }
        let plain = crate::save(&control);
        let watched = crate::save(&observed);
        let same = plain == watched
            && control.rng == observed.rng
            && control.headlines == observed.headlines
            && plain_news == watched_news;
        comparisons.push(json!({"month":month+1,"date":[control.year,control.month,control.day],"equal":same,
            "control_bytes":plain.len(),"observed_bytes":watched.len(),"control_fnv64":fnv(plain.as_bytes()),"observed_fnv64":fnv(watched.as_bytes())}));
        eprintln!(
            "A1_OBSERVER month={} exact={} rows={} coups={}",
            month + 1,
            same,
            totals.values().sum::<u64>(),
            firing.len()
        );
        if !same {
            failure = Some(format!(
                "Complete-world/RNG/headline mismatch after ordinary month {}",
                month + 1
            ));
            break;
        }
    }
    writer.flush().unwrap();
    writer.get_ref().sync_all().unwrap();
    for (name, w) in [
        ("control-final.json", &control),
        ("observed-final.json", &observed),
    ] {
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(out.join(name))
            .unwrap();
        f.write_all(crate::save(w).as_bytes()).unwrap();
        f.sync_all().unwrap();
    }
    let passed = failure.is_none()
        && comparisons.len() == 252
        && !firing.is_empty()
        && firing.len() == elected_coup_headlines.len();
    write_json(
        &out.join("result.json"),
        &json!({"format":"spheres-a1-firing-result/v1","seed":0,"months_requested":252,
        "months_compared":comparisons.len(),"passed":passed,"qualification":false,"a1_pass_claimed":false,
        "failure":failure,"stage_counts":totals,"firing_cases":firing,"elected_coup_headlines":elected_coup_headlines,
        "observer_firings_match_actual_headline_count":firing.len()==elected_coup_headlines.len(),"comparisons":comparisons,
        "final_date":[observed.year,observed.month,observed.day],"final_rng":observed.rng.state,
        "scope":"Pure observer equivalence and fixed development diagnostic only. No threshold/model change or reserved cohort."}),
    );
    assert!(
        passed,
        "A1 observer diagnostic failed; inspect retained compact result, raw rows and final worlds"
    );
}
