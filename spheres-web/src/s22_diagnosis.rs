// Diagnostic only: named costs on an immutable actual S22 checkpoint. This
// mirrors today's native daily schedule, including commands and airbases, and
// checks full serialized world and returned headlines against Game every day.
// Each driver owns an independent persistent route pool. Instrumentation runs
// first, so neither its times nor the following native times qualify S22.

fn s22_observed_day(
    w: &mut WorldState,
    commands: &[Command],
    routes: &mut logistics::NominalRoutePool,
) -> (Vec<String>, Vec<(String, f64)>) {
    assert!(spheres_sim::clock::is_daily(w));
    let mut stages = Vec::new();
    macro_rules! timed {
        ($name:expr, $operation:expr) => {{
            let start = Instant::now();
            $operation;
            stages.push(($name.to_string(), ms(start)));
        }};
    }
    // The offset must be captured after the native month-start clear.
    timed!("prelude.headlines_and_reindex", {
        if w.day <= 1 { w.headlines.clear(); }
        w.reindex();
    });
    let headline_start = w.headlines.len();
    timed!("prelude.commands", {
        for command in commands {
            if let Err(error) = apply_command(w, command) {
                w.headline(format!("[rejected] {:?}: {}", command, error));
            }
        }
    });
    timed!("prelude.company_receivables", spheres_sim::companies::settle_receivables(w));
    timed!("prelude.fiscal_prepare", spheres_sim::fiscal_recovery::prepare(w));
    timed!("prelude.province_begin", spheres_sim::province_economy::begin_day(w));
    timed!("prelude.programs_begin", spheres_sim::programs::begin_day(w));
    timed!("prelude.production", spheres_sim::production::tick_day(w));
    timed!("prelude.airbases", {
        let events = spheres_sim::airbases::tick_day(w);
        w.headlines.extend(events);
    });
    for (name, system) in spheres_sim::SYSTEMS {
        if *name == "arsenal" {
            // Same persistent-pool clearing position as tick_day_with_routes.
            // Arsenal's subsequent clearing sees the settled marker.
            let start = Instant::now();
            spheres_sim::resources::clear_spot_market_observed_with_pool(w, routes,
                &mut |stage, commodity, elapsed| {
                    stages.push((format!("detail.market.{stage}.{}", commodity.map_or("all", |c| c.key())),
                        elapsed.as_secs_f64() * 1000.0));
                });
            stages.push(("prelude.arsenal_spot_market".into(), ms(start)));
        }
        if *name == "resources" || *name == "war" {
            let start = Instant::now();
            let mut observe = |stage, elapsed: std::time::Duration| {
                stages.push((format!("detail.{name}.{stage}"), elapsed.as_secs_f64() * 1000.0));
            };
            if *name == "resources" { spheres_sim::resources::tick_observed(w, &mut observe); }
            else { spheres_sim::war::tick_observed(w, &mut observe); }
            stages.push((format!("system.{name}"), ms(start)));
        } else if *name == "economic_ai" || *name == "military_ai" {
            let start = Instant::now();
            let mut observe = |stage: &str, nation: Option<NationId>, elapsed: std::time::Duration| {
                stages.push((format!("detail.{name}.{stage}.{}", nation.map_or("all", |id| id.name())),
                    elapsed.as_secs_f64() * 1000.0));
            };
            if *name == "economic_ai" { spheres_sim::economic_ai::tick_observed_detailed(w, &mut observe); }
            else { spheres_sim::military_ai::tick_observed_detailed(w, &mut observe); }
            stages.push((format!("system.{name}"), ms(start)));
        } else {
            timed!(format!("system.{name}"), system(w));
        }
    }
    timed!("postlude.programs_finish", spheres_sim::programs::finish_day(w));
    timed!("postlude.company_receivables", spheres_sim::companies::settle_receivables(w));
    timed!("postlude.province_finish", spheres_sim::province_economy::finish_day(w));
    timed!("postlude.sector_contractors", spheres_sim::sector_contractors::tick_day(w));
    timed!("postlude.supply_automation", spheres_sim::equipment::tick_supply_automation(w));
    timed!("postlude.campaign_aims", spheres_sim::campaign_aims::tick(w));
    timed!("postlude.fiscal_observation", spheres_sim::fiscal_recovery::tick(w));
    timed!("postlude.fiscal_journal", spheres_sim::fiscal_journal::finish_day(w));
    timed!("postlude.advance_date", spheres_sim::clock::advance_date(w));
    (w.headlines[headline_start..].to_vec(), stages)
}

fn s22_diagnostic_summaries(rows: &[Value]) -> Value {
    let mut samples = std::collections::BTreeMap::<String, Vec<f64>>::new();
    for row in rows {
        for stage in row["stages"].as_array().unwrap() {
            samples.entry(stage["name"].as_str().unwrap().to_owned()).or_default()
                .push(stage["elapsed_ms"].as_f64().unwrap());
        }
    }
    json!(samples.into_iter().map(|(name, values)| {
        let mut stats = summary(&values);
        let total = values.iter().sum::<f64>();
        stats["total_ms"] = json!(total);
        stats["mean_per_observation_ms"] = json!(total / values.len() as f64);
        stats["mean_per_completed_day_ms"] = json!(total / rows.len() as f64);
        (name, stats)
    }).collect::<std::collections::BTreeMap<_, _>>())
}

#[test]
#[ignore = "31-day actual S22 checkpoint subsystem diagnosis; not qualification or a memory benchmark"]
fn s22_daily_subsystem_diagnosis() {
    let (input, output) = s22_paths();
    let mut report_file = std::fs::OpenOptions::new().write(true).create_new(true).open(&output).unwrap();
    let input_fingerprint = s08_diagnostic_input_fingerprint(&input);
    assert_ne!(std::env::var("SPHERES_S22_ADOPT_COMPETITION").as_deref(), Ok("1"),
        "This diagnostic never adopts rules or authors workload");
    let mut g = storage::decode(&std::fs::read_to_string(&input).unwrap()).unwrap();
    assert_eq!(g.world.player, Some(NationId::France), "Retain the actual France lineage");
    assert!(spheres_sim::clock::is_daily(&g.world));
    assert!(g.freight_routes.is_empty(), "Decoded route pool starts cold");
    let certified = std::env::var("SPHERES_S22_REQUIRE_CERTIFIED").as_deref() == Ok("1");
    if certified { s22_certified_capabilities(&g.world, false).expect("Already adopted profile"); }
    if let Ok(expected) = std::env::var("SPHERES_S22_DIAGNOSTIC_EXPECT_DATE") {
        assert_eq!(s22_date(&g), s22_target(&expected), "Unexpected actual input date");
    }
    let renew = std::env::var("SPHERES_S22_RENEW_BUDGET").as_deref() == Ok("1");
    let mut observed = g.world.clone();
    let mut observed_routes = logistics::NominalRoutePool::default();
    let mut rows = Vec::new();
    let mut report = json!({"format":"spheres-s22-subsystem-diagnosis/v1", "revision":env!("SPHERES_REVISION"),
        "input":input, "input_fingerprint":{"bytes":input_fingerprint.0,"fnv1a64":format!("{:016x}",input_fingerprint.1)},
        "starting":s22_facts(&g), "requested_days":31, "renew_existing_budget":renew,
        "certified_profile_required":certified, "status":"running", "passed":false, "rows":[],
        "system_order":spheres_sim::SYSTEMS.iter().map(|(name,_)|*name).collect::<Vec<_>>(),
        "timing_hierarchy":{"detail.market.*":"Nested inside prelude.arsenal_spot_market; never add to its enclosing time.",
            "detail.market.dispatch_*":"Nested inside detail.market.orders_and_dispatch for the same commodity; sub-timers also overlap (source search/assembly within dispatch_plan). Zero-duration count/buffer labels are observations, not measured work.",
            "detail.economic_ai.*":"Nested inside system.economic_ai; never add to its enclosing time.",
            "detail.military_ai.*":"Nested inside system.military_ai; review stages are also nested inside review.total for the same nation. Never add nested measurements to their enclosing time."},
        "scope":"Diagnostic only. Instrumented daily schedule followed by ordinary Game advancement on independent worlds and independent persistent route pools. No authored state, pending-import assertion, seed/date rewrite or grants. Existing budgets may renew through the shared S22 normal command helper. Clone, command preflight, exact world serialization, facts and report I/O are outside stage clocks. Native Game timing includes its actual tick, event logging and history; those components are not inferred by subtracting independent timings. Detail timers are nested inside system timers and must not be added to them. This is neither S22 latency acceptance nor a memory measurement; the outer runner records cryptographic provenance."});
    s08_diagnostic_write(&mut report_file, &report);
    for index in 0..31 {
        let preflight_start = Instant::now();
        let commands = match s22_renewal(&g, renew) {
            Ok(commands) => commands,
            Err(error) => {
                report["status"] = json!("command_preflight_failed");
                report["failure"] = json!(error);
                s08_diagnostic_write(&mut report_file, &report);
                panic!("S22 diagnosis cannot continue; see {}", output.display());
            }
        };
        let preflight_ms = ms(preflight_start);
        let ordered = !commands.is_empty();
        let command_row = json!({"commands":commands,
            "prices_pc":commands.iter().map(|command|spheres_sim::price_of(&g.world,command)).collect::<Vec<_>>(),
            "payer_pc_before":g.world.nation(NationId::France).political_capital});
        let before = spheres_sim::clock::absolute_day(&g.world);
        let date_before = g.world.date_str();
        let log_start = g.log.len();
        let history_before = g.history.len();
        let epoch_before = g.history_epoch;
        let instrumented_start = Instant::now();
        let (headlines, stages) = s22_observed_day(&mut observed, &commands, &mut observed_routes);
        let instrumented_ms = ms(instrumented_start);
        let normal_start = Instant::now();
        let outcome = g.advance_days(1, commands);
        let native_game_ms = ms(normal_start);
        let validation = s22_validate_day(&g, before, log_start, ordered)
            .and_then(|()| if certified {s22_certified_capabilities(&g.world,index>=29)} else {Ok(())});
        let compare_start = Instant::now();
        let expected = spheres_sim::save(&observed);
        let actual = spheres_sim::save(&g.world);
        let exact = expected == actual;
        let first_difference = if exact {None} else {
            Some(expected.as_bytes().iter().zip(actual.as_bytes()).position(|(a,b)|a!=b)
                .unwrap_or(expected.len().min(actual.len())))
        };
        let exact_headlines = headlines.iter().map(String::as_str)
            .eq(g.log[log_start..].iter().map(|event|event.text.as_str()));
        let row = json!({"index":index,"date_before":date_before,"date_after":g.world.date_str(),
            "orders":command_row,"payer_pc_after":g.world.nation(NationId::France).political_capital,
            "command_preflight_ms":preflight_ms,"instrumented_tick_ms":instrumented_ms,"native_game_tick_log_history_ms":native_game_ms,
            "stages":stages.into_iter().map(|(name,elapsed_ms)|json!({"name":name,"elapsed_ms":elapsed_ms})).collect::<Vec<_>>(),
            "exact_native_world":exact,"expected_bytes":expected.len(),"actual_bytes":actual.len(),
            "first_difference_byte":first_difference,"exact_returned_headlines":exact_headlines,
            "comparison_ms":ms(compare_start),"headlines":headlines.len(),"event_pause":outcome.1,
            "history_before":history_before,"history_after":g.history.len(),
            "history_epoch_before":epoch_before,"history_epoch_after":g.history_epoch,
            "log_before":log_start,"log_after":g.log.len(),"validation_error":validation.as_ref().err()});
        // Release large comparison buffers before facts/report serialization.
        drop(expected); drop(actual);
        rows.push(row);
        report["rows"] = json!(rows);
        report["subsystem_summaries"] = s22_diagnostic_summaries(&rows);
        let passed = exact && exact_headlines && validation.is_ok();
        report["status"] = json!(if passed {"running"} else {"daily_equivalence_failed"});
        s08_diagnostic_write(&mut report_file, &report);
        assert!(passed,"Diagnostic daily equivalence failed; bounded details saved in {}",output.display());
        eprintln!("S22 diagnosis day {}/31: {} -> {}, observed {:.3} ms, native {:.3} ms; exact world/headlines",
            index+1,date_before,g.world.date_str(),instrumented_ms,native_game_ms);
    }
    let input_unchanged = input_fingerprint == s08_diagnostic_input_fingerprint(&input);
    report["input_unchanged"] = json!(input_unchanged);
    report["passed"] = json!(input_unchanged && rows.len() == 31);
    report["status"] = json!(if input_unchanged {"complete"} else {"input_changed"});
    report["ending"] = s22_facts(&g);
    for name in ["instrumented_tick_ms","native_game_tick_log_history_ms","command_preflight_ms","comparison_ms"] {
        report["timing_summaries"][name] = summary(&rows.iter().map(|row|row[name].as_f64().unwrap()).collect::<Vec<_>>());
    }
    s08_diagnostic_write(&mut report_file,&report);
    assert!(input_unchanged,"Immutable diagnostic input changed");
}

#[test]
fn s22_observed_schedule_matches_native_commands_airbases_and_headlines() {
    let mut g = Game::new_fresh(1990,Some(NationId::France));
    fresh_play_rules(&mut g).unwrap();
    // Ordinary enrollment settles before the explicit competition adoption,
    // just as in the existing S22 adoption regression and real source lineage.
    g.advance_days(1,vec![]);
    s22_adopt_competition(&mut g).unwrap();
    let mut observed = g.world.clone();
    let mut routes = logistics::NominalRoutePool::default();
    for _ in 0..3 {
        let commands = s22_renewal(&g,true).unwrap();
        let log_start = g.log.len();
        let (headlines, stages) = s22_observed_day(&mut observed,&commands,&mut routes);
        g.advance_days(1,commands);
        assert!(spheres_sim::save(&observed)==spheres_sim::save(&g.world),"Observed schedule changed native world");
        assert!(headlines.iter().map(String::as_str).eq(g.log[log_start..].iter().map(|event|event.text.as_str())));
        assert!(stages.iter().any(|(name,_)|name=="prelude.airbases"));
        assert_eq!(stages.iter().filter(|(name,_)|name=="prelude.arsenal_spot_market").count(),1);
        assert!(stages.iter().any(|(name,_)|name=="detail.market.opening_and_enrolled_draws.all"));
        assert!(stages.iter().any(|(name,_)|name=="detail.market.prices_and_finance.all"));
        assert!(stages.iter().any(|(name,_)|name=="detail.resources.post_market_flows"));
        assert!(stages.iter().any(|(name,_)|name=="detail.war.resolve_conflicts"));
        assert!(stages.iter().any(|(name,_)|name=="detail.military_ai.review.selection.all"));
        for (name,_) in spheres_sim::SYSTEMS {
            assert_eq!(stages.iter().filter(|(stage,_)|stage==&format!("system.{name}")).count(),1);
        }
    }
}
