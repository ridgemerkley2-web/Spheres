// Included in performance.rs. This deliberately mirrors the daily driver only
// for diagnosis: every observed day must exactly match a normal Game advance.
// The observed WorldState has no persistent route pool; the normal Game owns
// its own pool, starting empty after decode. No retained searches are shared
// between them. Timings remain exploratory (the normal tick follows the
// observed tick), not the predefined campaign performance acceptance run.

fn s08_diagnostic_input_fingerprint(path: &std::path::Path) -> (u64, u64) {
    use std::io::Read;
    let mut file = std::fs::File::open(path).unwrap();
    let mut buffer = [0_u8; 65536];
    let mut bytes = 0_u64;
    let mut fnv = 0xcbf29ce484222325_u64;
    loop {
        let count = file.read(&mut buffer).unwrap();
        if count == 0 { break; }
        bytes += count as u64;
        for byte in &buffer[..count] {
            fnv = (fnv ^ u64::from(*byte)).wrapping_mul(0x100000001b3);
        }
    }
    (bytes, fnv)
}

fn s08_diagnostic_write(file: &mut std::fs::File, report: &Value) {
    use std::io::{Seek, Write};
    file.rewind().unwrap();
    file.set_len(0).unwrap();
    serde_json::to_writer_pretty(&mut *file, report).unwrap();
    file.write_all(b"\n").unwrap();
    file.flush().unwrap();
}

#[test]
#[ignore = "explicit saved-campaign subsystem diagnosis; not a performance acceptance run"]
fn s08_daily_subsystem_diagnosis() {
    let input = std::path::PathBuf::from(
        std::env::var("SPHERES_S08_DIAGNOSTIC_INPUT").expect("Set the actual paid-import archive"),
    );
    let output = std::path::PathBuf::from(
        std::env::var("SPHERES_S08_DIAGNOSTIC_OUT").expect("Set a new diagnostic report file"),
    );
    assert!(input.is_absolute() && output.is_absolute() && input != output);
    // Exclusive creation prevents this opt-in diagnostic overwriting evidence.
    let mut report_file = std::fs::OpenOptions::new()
        .write(true).create_new(true).open(&output).unwrap();
    let fingerprint = s08_diagnostic_input_fingerprint(&input);
    let mut g = storage::decode(&std::fs::read_to_string(&input).unwrap()).unwrap();
    assert!(g.freight_routes.is_empty(), "Decoded campaigns must start with an empty derived route pool");
    let buyer = g.world.player.expect("The actual archive must have a player");
    assert!(spheres_sim::clock::is_daily(&g.world));
    assert!(g.world.supplier_catalogue.enabled && g.world.rules.economic_competition);
    assert!(g.world.companies.imports.contracts.iter().any(|contract|
        contract.buyer == buyer && contract.delivered_day.is_none()
            && contract.cancelled_day.is_none()));
    let starting_date = g.world.date_str();
    let starting_owned = profile_owned_work(&g.world);
    // Clone and full-world serialization are outside every timed subsystem.
    let mut observed = g.world.clone();
    let mut rows = Vec::new();
    let mut samples = std::collections::BTreeMap::<String, Vec<f64>>::new();
    let mut report = json!({
        "revision": env!("SPHERES_REVISION"), "input": input,
        "input_fingerprint": {"bytes":fingerprint.0,"fnv1a64":format!("{:016x}",fingerprint.1)},
        "starting_date": starting_date, "starting_owned_work": starting_owned,
        "commands": [], "requested_days":31, "status":"running", "rows":[],
        "normal_route_pool_initially_empty": true,
        "scope":"Exploratory named daily-system timings on a disposable decoded campaign. The observed driver uses cold per-clearing nominal searches; normal Game advance owns a separate persistent pool, empty after decode. No route pool is shared between drivers. The observed driver still runs first, so these are not independently ordered performance measurements. Clone, exact native serialization comparison and report writes are outside subsystem timings. The outer runner supplies cryptographic input/source provenance. No performance acceptance claim."
    });
    s08_diagnostic_write(&mut report_file, &report);
    for offset in 1..=31 {
        assert!(spheres_sim::clock::is_daily(&observed));
        let date_before = observed.date_str();
        let mut stages = Vec::new();
        let manual_start = Instant::now();
        macro_rules! timed {
            ($name:expr, $operation:expr) => {{
                let started = Instant::now();
                $operation;
                let elapsed = ms(started);
                let name = $name.to_string();
                samples.entry(name.clone()).or_default().push(elapsed);
                stages.push(json!({"name":name,"elapsed_ms":elapsed}));
            }};
        }
        // Exact prelude/order from spheres_sim::tick_day, with commands [].
        timed!("prelude.headlines_and_reindex", {
            if observed.day <= 1 { observed.headlines.clear(); }
            observed.reindex();
        });
        timed!("prelude.company_receivables", spheres_sim::companies::settle_receivables(&mut observed));
        timed!("prelude.fiscal_prepare", spheres_sim::fiscal_recovery::prepare(&mut observed));
        timed!("prelude.province_begin", spheres_sim::province_economy::begin_day(&mut observed));
        timed!("prelude.programs_begin", spheres_sim::programs::begin_day(&mut observed));
        timed!("prelude.production", spheres_sim::production::tick_day(&mut observed));
        for (name, system) in spheres_sim::SYSTEMS {
            if *name == "arsenal" {
                // Clearing is idempotent within a day. Time it explicitly;
                // the subsequent native arsenal call sees the same settled
                // world. Full equality below checks that split on every day.
                let started = Instant::now();
                spheres_sim::resources::clear_spot_market_observed(&mut observed, &mut |stage, commodity, elapsed| {
                    let key = format!("detail.market.{stage}.{}", commodity.map_or("all", |c| c.key()));
                    let elapsed_ms = elapsed.as_secs_f64() * 1000.0;
                    samples.entry(key.clone()).or_default().push(elapsed_ms);
                    stages.push(json!({"name":key,"elapsed_ms":elapsed_ms}));
                });
                let elapsed_ms = ms(started);
                samples.entry("detail.spot_market".into()).or_default().push(elapsed_ms);
                stages.push(json!({"name":"detail.spot_market","elapsed_ms":elapsed_ms}));
            }
            if *name == "economic_ai" {
                let started = Instant::now();
                spheres_sim::economic_ai::tick_observed_detailed(&mut observed, &mut |stage, nation, elapsed| {
                    let key = format!("detail.economic_ai.{stage}.{}", nation.map_or("all", |id| id.name()));
                    let elapsed_ms = elapsed.as_secs_f64() * 1000.0;
                    samples.entry(key.clone()).or_default().push(elapsed_ms);
                    stages.push(json!({"name":key,"elapsed_ms":elapsed_ms}));
                });
                let elapsed_ms = ms(started);
                samples.entry(format!("system.{name}")).or_default().push(elapsed_ms);
                stages.push(json!({"name":format!("system.{name}"),"elapsed_ms":elapsed_ms}));
            } else {
                timed!(format!("system.{name}"), system(&mut observed));
            }
        }
        timed!("postlude.programs_finish", spheres_sim::programs::finish_day(&mut observed));
        timed!("postlude.company_receivables", spheres_sim::companies::settle_receivables(&mut observed));
        timed!("postlude.province_finish", spheres_sim::province_economy::finish_day(&mut observed));
        timed!("postlude.sector_contractors", spheres_sim::sector_contractors::tick_day(&mut observed));
        timed!("postlude.supply_automation", spheres_sim::equipment::tick_supply_automation(&mut observed));
        timed!("postlude.campaign_aims", spheres_sim::campaign_aims::tick(&mut observed));
        timed!("postlude.fiscal_observation", spheres_sim::fiscal_recovery::tick(&mut observed));
        timed!("postlude.fiscal_journal", spheres_sim::fiscal_journal::finish_day(&mut observed));
        timed!("postlude.advance_date", spheres_sim::clock::advance_date(&mut observed));
        let manual_elapsed = ms(manual_start);
        let normal_start = Instant::now();
        g.advance_days(1, vec![]);
        let normal_elapsed = ms(normal_start);
        let expected = spheres_sim::save(&observed);
        let actual = spheres_sim::save(&g.world);
        let exact = expected == actual;
        let first_difference = if exact { None } else {
            Some(expected.as_bytes().iter().zip(actual.as_bytes())
                .position(|(left, right)| left != right)
                .unwrap_or(expected.len().min(actual.len())))
        };
        rows.push(json!({"day":offset,"date_before":date_before,
            "date_after":g.world.date_str(),"stages":stages,
            "manual_driver_ms":manual_elapsed,"normal_game_ms":normal_elapsed,
            "exact_native_world":exact,"expected_bytes":expected.len(),
            "actual_bytes":actual.len(),"first_difference_byte":first_difference}));
        report["rows"] = json!(rows);
        report["status"] = json!(if exact { "running" } else { "world_mismatch" });
        s08_diagnostic_write(&mut report_file, &report);
        // Never ask assert_eq! to format two enormous worlds on a failure.
        assert!(exact, "Daily diagnostic differs from normal Game advance on day {offset}; bounded details saved in {}", output.display());
        eprintln!("S08 diagnostic day {offset}/31: {} -> {}, observed {:.3} ms, normal {:.3} ms, exact world",
            date_before, g.world.date_str(), manual_elapsed, normal_elapsed);
    }
    let input_unchanged = fingerprint == s08_diagnostic_input_fingerprint(&input);
    report["status"] = json!(if input_unchanged { "complete" } else { "input_changed" });
    report["input_unchanged"] = json!(input_unchanged);
    report["ending_date"] = json!(g.world.date_str());
    report["ending_owned_work"] = profile_owned_work(&g.world);
    report["normal_route_pool_empty_at_end"] = json!(g.freight_routes.is_empty());
    report["subsystem_summaries"] = json!(samples.iter().map(|(name, values)|
        (name.clone(), summary(values))).collect::<std::collections::BTreeMap<_,_>>());
    s08_diagnostic_write(&mut report_file, &report);
    assert!(input_unchanged, "Original diagnostic input changed");
}
