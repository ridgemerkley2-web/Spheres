//! Opt-in, uncontended measurements of the actual browser game configuration.
//! Run with --ignored --exact performance::campaign_lifetime_profile --nocapture
//! and SPHERES_PROFILE_OUT set to a disposable output file. No live server used.
//! PREPARE_ONLY=1 (with the SPHERES_PROFILE_ prefix) saves actual checkpoint
//! campaigns without timing. INPUT names an absolute profile-campaigns directory
//! to measure its saved checkpoints, without warming between years or overwriting
//! inputs. PREPARE_RESUME_SLOT can resume preparation from an INPUT slot.
//! INPUT uses idle_human checkpoints for both cases: the busy case establishes
//! priced war and construction work on a copy at the reported starting date.
//! This measures an aged archive under new stress, not decades of economic AI.
use super::*;
use std::time::Instant;

fn ms(start: Instant) -> f64 {
    start.elapsed().as_secs_f64() * 1000.0
}
fn summary(values: &[f64]) -> serde_json::Value {
    let mut sorted = values.to_vec();
    sorted.sort_by(f64::total_cmp);
    let n = sorted.len();
    serde_json::json!({"samples":n,"median_ms":sorted[n/2],"p95_ms":sorted[((n as f64*0.95).ceil() as usize).saturating_sub(1)],"max_ms":sorted[n-1]})
}

fn profile_counts(w: &WorldState) -> (usize, usize) {
    (
        w.conflicts.iter().filter(|c| c.shooting()).count(),
        spheres_sim::production::projects_for(w, NationId::USA).count(),
    )
}

fn profile_command(
    w: &mut WorldState,
    command: Command,
    commands: &mut Vec<serde_json::Value>,
) -> Result<(), String> {
    let payer = match &command {
        Command::DeclareWar { attacker, .. } => *attacker,
        Command::EnableEconomicCompetition { nation }
        | Command::SetAnnualBudget { nation, .. }
        | Command::SetProgramBudget { nation, .. }
        | Command::StartIndustryModule { nation, .. }
        | Command::StartProject { nation, .. } => *nation,
        _ => panic!("Unexpected performance fixture command"),
    };
    let price = spheres_sim::price_of(w, &command);
    let before = w.nation(payer).political_capital;
    apply_command(w, &command)?;
    commands.push(
        serde_json::json!({"command":command,"price_pc":price,"payer":payer,
        "payer_pc_before":before,"payer_pc_after":w.nation(payer).political_capital}),
    );
    Ok(())
}

/// Controlled benchmark setup only. No date advance, free output, inventory,
/// infrastructure or paid work is granted. Ordinary command validation remains
/// authoritative; political standing overrides are explicit fixture metadata.
fn establish_profile_stress(g: &mut Game) -> serde_json::Value {
    use spheres_sim::{industrial_modules, production, programs, sovereignty};
    let player = NationId::USA;
    assert!(
        g.world.nation(player).alive,
        "The source campaign's USA player must still be alive"
    );
    let mut commands = Vec::new();
    let mut overrides = vec![serde_json::json!({"nation":player,
        "field":"political_capital","before":g.world.nation(player).political_capital,"after":100.0})];
    g.world.nation_mut(player).political_capital = 100.0;
    profile_command(
        &mut g.world,
        Command::EnableEconomicCompetition { nation: player },
        &mut commands,
    )
    .unwrap();

    let mut allocations = g.world.nation(player).budget_for(g.world.year).allocations;
    // Preserve the inherited plan except a missing industry allocation. Move
    // authority within the total cap; this is a recorded policy, never cash.
    allocations[BUDGET_INDUSTRY] = allocations[BUDGET_INDUSTRY].max(0.02);
    let excess = (allocations.iter().sum::<f64>() - 0.70).max(0.0);
    if excess > 0.0 {
        let donor = (0..BUDGET_MINISTRIES)
            .filter(|i| *i != BUDGET_INDUSTRY)
            .max_by(|a, b| allocations[*a].total_cmp(&allocations[*b]))
            .unwrap();
        allocations[donor] -= excess;
    }
    let fiscal_year = g.world.year;
    if !programs::enrolled(&g.world, player) {
        profile_command(
            &mut g.world,
            Command::SetAnnualBudget {
                nation: player,
                fiscal_year,
                allocations,
            },
            &mut commands,
        )
        .unwrap();
    }
    profile_command(
        &mut g.world,
        Command::SetProgramBudget {
            nation: player,
            fiscal_year,
            allocations,
            departments: programs::default_departments(),
        },
        &mut commands,
    )
    .unwrap();
    let districts: Vec<_> = g
        .world
        .districts
        .iter()
        .filter(|(_, owner)| **owner == player)
        .map(|(district, _)| district.clone())
        .collect();
    let capacity_micros = industrial_modules::recommended_capacity_micros(&g.world, player);
    let project = districts
        .iter()
        .find(|district| {
            industrial_modules::start_error(&g.world, player, district, capacity_micros).is_none()
        })
        .map(|district| Command::StartIndustryModule {
            nation: player,
            district: district.clone(),
            capacity_micros,
        })
        .or_else(|| {
            [
                production::ProjectKind::Infrastructure,
                production::ProjectKind::CivilianIndustry,
                production::ProjectKind::PowerGrid,
                production::ProjectKind::Warehouse,
            ]
            .into_iter()
            .find_map(|kind| {
                districts
                    .iter()
                    .find(|district| {
                        production::start_project_error(&g.world, player, district, kind).is_none()
                    })
                    .map(|district| Command::StartProject {
                        nation: player,
                        district: district.clone(),
                        kind,
                    })
            })
        });
    if let Some(command) = project {
        profile_command(&mut g.world, command, &mut commands).unwrap();
    }
    assert!(
        profile_counts(&g.world).1 > 0,
        "No legal player construction project can establish the stress fixture"
    );

    // Prefer the original Gulf pair; otherwise adjacent living AI governments
    // in stable nation order. Preflight on a clone so refused candidates never
    // change the measured world or leave their PC override behind.
    let living: Vec<_> = g
        .world
        .nations
        .iter()
        .filter(|n| n.alive && n.id != player && n.mil_strength > 0.0)
        .map(|n| n.id)
        .collect();
    let mut pairs = Vec::new();
    for &attacker in &living {
        for &defender in &living {
            if attacker != defender
                && !(g.world.nation(attacker).nuclear && g.world.nation(defender).nuclear)
                && sovereignty::hostility_reason(&g.world, attacker, defender).is_none()
                && !g
                    .world
                    .conflicts
                    .iter()
                    .any(|c| c.involves(attacker) && c.involves(defender))
            {
                pairs.push((attacker, defender));
            }
        }
    }
    pairs.sort_by_key(|(a, b)| {
        (
            (*a, *b) != (NationId::Iraq, NationId::Kuwait),
            !spheres_sim::nations::adjacent(*a, *b),
            *a,
            *b,
        )
    });
    let mut opened = None;
    for (attacker, defender) in pairs {
        let mut trial = g.world.clone();
        let before = trial.nation(attacker).political_capital;
        trial.nation_mut(attacker).political_capital = 100.0;
        let mut trial_commands = Vec::new();
        if profile_command(
            &mut trial,
            Command::DeclareWar { attacker, defender },
            &mut trial_commands,
        )
        .is_ok()
        {
            g.world = trial;
            commands.extend(trial_commands);
            overrides.push(serde_json::json!({"nation":attacker,"field":"political_capital","before":before,"after":100.0}));
            opened = Some((attacker, defender));
            break;
        }
    }
    assert!(
        opened.is_some(),
        "No legal living AI pair can establish the war stress fixture"
    );
    let (wars, projects) = profile_counts(&g.world);
    assert!(wars > 0 && projects > 0);
    serde_json::json!({"kind":"priced_stress_on_aged_idle_checkpoint","setup_commands":commands,
        "political_capital_overrides":overrides,"new_war":opened,"starting_active_wars":wars,
        "starting_player_projects":projects,"project_details":production::projects_for(&g.world, player).collect::<Vec<_>>(),
        "limits":"Controlled political capital only; all budget, construction and war commands pay normal prices. No output, cash, materials or completed capacity granted. Projects can encounter real supply/funding blockers; activity during the sample is reported. This is not an uninterrupted economic-AI campaign."})
}

#[test]
fn aged_checkpoint_stress_uses_priced_commands_and_preserves_the_archive() {
    for missing_gulf_pair in [false, true] {
        let mut g = Game::new(1990, Some(NationId::USA));
        fresh_play_rules(&mut g).unwrap();
        g.history.clear();
        g.snapshot();
        // Fixture only: ensure fallback is exercised without warming decades.
        if missing_gulf_pair {
            g.world.nation_mut(NationId::Iraq).alive = false;
            g.world.nation_mut(NationId::Kuwait).alive = false;
        }
        let source = save(&g.world);
        let history_before = serde_json::to_string(&g.history).unwrap();
        let date = g.world.date_str();
        let gdp = g.world.nation(NationId::USA).gdp;
        let stock = g.world.production.industry.goods.clone();
        let mut replay = g.world.clone();
        let details = establish_profile_stress(&mut g);
        assert_eq!(
            g.world.date_str(),
            date,
            "fixture preparation is outside the daily samples"
        );
        assert_eq!(serde_json::to_string(&g.history).unwrap(), history_before);
        assert_eq!(g.world.nation(NationId::USA).gdp, gdp, "no output grant");
        assert_eq!(g.world.production.industry.goods, stock, "no input grant");
        assert!(profile_counts(&g.world).0 > 0 && profile_counts(&g.world).1 > 0);
        assert_ne!(save(&g.world), source);
        let commands = details["setup_commands"].as_array().unwrap();
        assert!(commands
            .iter()
            .any(|v| v["command"].get("StartIndustryModule").is_some()
                || v["command"].get("StartProject").is_some()));
        let declaration = commands
            .iter()
            .find(|v| v["command"].get("DeclareWar").is_some())
            .unwrap();
        assert_eq!(declaration["payer_pc_before"], 100.0);
        assert_eq!(declaration["price_pc"], 30.0);
        // Declaration can also trigger ordinary priced access requests. Replay
        // every recorded command to verify all costs/consequences, not just the
        // declaration's quoted direct bill.
        let mut overridden = std::collections::BTreeSet::new();
        for record in commands {
            let payer: NationId = serde_json::from_value(record["payer"].clone()).unwrap();
            if overridden.insert(payer) {
                replay.nation_mut(payer).political_capital = 100.0;
            }
            let command: Command = serde_json::from_value(record["command"].clone()).unwrap();
            assert_eq!(
                serde_json::json!(spheres_sim::price_of(&replay, &command)),
                record["price_pc"]
            );
            apply_command(&mut replay, &command).unwrap();
            assert_eq!(
                serde_json::json!(replay.nation(payer).political_capital),
                record["payer_pc_after"]
            );
        }
        assert_eq!(
            save(&replay),
            save(&g.world),
            "normal commands reproduce the entire measured-start world"
        );
        if missing_gulf_pair {
            assert_ne!(
                declaration["command"]["DeclareWar"]["attacker"],
                serde_json::json!(NationId::Iraq)
            );
            assert_ne!(
                declaration["command"]["DeclareWar"]["defender"],
                serde_json::json!(NationId::Kuwait)
            );
        }
    }
    // Optional release preflight against the actual prepared archives. This
    // checks all aged setups without simulating any of the 31 measured days.
    if let Some(root) = std::env::var_os("SPHERES_PROFILE_INPUT") {
        for age in [0, 10, 30] {
            let mut g = storage::read(
                std::path::Path::new(&root),
                &format!("profile-idle_human-{age}"),
                false,
            )
            .unwrap();
            assert_eq!(g.world.year, 1990 + age);
            assert!(!g.world.rules.economic_competition);
            let source_hash = format!("{:016x}", spheres_sim::state_hash(&g.world));
            let details = establish_profile_stress(&mut g);
            eprintln!(
                "PROFILE_SETUP {}",
                serde_json::json!({"age":age,"date":g.world.date_str(),
                "source_world_hash":source_hash,"measured_start_world_hash":format!("{:016x}",spheres_sim::state_hash(&g.world)),
                "stress_fixture":details})
            );
        }
    }
}

#[test]
#[ignore = "30-year actual daily worlds; run alone for meaningful timing"]
fn campaign_lifetime_profile() {
    let path = std::env::var("SPHERES_PROFILE_OUT")
        .expect("Set SPHERES_PROFILE_OUT to a disposable JSON output path");
    let years = std::env::var("SPHERES_PROFILE_YEARS")
        .ok()
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(30);
    assert!((0..=30).contains(&years));
    let only = std::env::var("SPHERES_PROFILE_SCENARIO").ok();
    let prepare_only = std::env::var("SPHERES_PROFILE_PREPARE_ONLY").as_deref() == Ok("1");
    let input = std::env::var_os("SPHERES_PROFILE_INPUT").map(std::path::PathBuf::from);
    if let Some(root) = &input {
        assert!(
            root.is_absolute(),
            "SPHERES_PROFILE_INPUT must be an absolute profile-campaigns directory"
        );
    }
    let resume = std::env::var("SPHERES_PROFILE_PREPARE_RESUME_SLOT").ok();
    assert!(
        resume.is_none() || prepare_only,
        "PREPARE_RESUME_SLOT requires PREPARE_ONLY=1"
    );
    let campaign_root = std::path::Path::new(&path)
        .parent()
        .unwrap_or(std::path::Path::new("."))
        .join("profile-campaigns");
    let resume_scenario = resume.as_ref().map(|slot| {
        ["idle_human", "industry_and_war"]
            .into_iter()
            .find(|name| {
                slot.strip_prefix(&format!("profile-{name}-"))
                    .is_some_and(|age| matches!(age, "0" | "10" | "30"))
            })
            .expect("Resume slot must be profile-{idle_human|industry_and_war}-{0|10|30}")
    });
    assert!(
        only.as_ref()
            .is_none_or(|s| matches!(s.as_str(), "idle_human" | "industry_and_war")),
        "Unknown profile scenario"
    );
    assert!(
        resume_scenario.is_none_or(|name| only.as_ref().is_none_or(|s| s == name)),
        "Resume slot and scenario disagree"
    );
    let mut results = Vec::new();
    for busy in [false, true] {
        let name = if busy {
            "industry_and_war"
        } else {
            "idle_human"
        };
        if only.as_ref().is_some_and(|s| s != name) {
            continue;
        }
        if resume_scenario.is_some_and(|scenario| scenario != name) {
            continue;
        }
        // Ordinary preparation needs only one aged idle archive per checkpoint.
        // Explicit legacy busy preparation/resume remains available for parity.
        if prepare_only && busy && only.as_deref() != Some(name) && resume_scenario != Some(name) {
            continue;
        }
        // Measurement from inputs never constructs/reseeds a new fixture. The
        // storage reader restores the full history and log through browser load.
        let mut campaign = if input.is_some() && !prepare_only {
            None
        } else if let Some(slot) = &resume {
            Some(
                storage::read(input.as_deref().unwrap_or(&campaign_root), slot, false)
                    .unwrap_or_else(|e| panic!("Cannot resume {slot}: {e}")),
            )
        } else {
            let mut g = Game::new(1990, Some(NationId::USA));
            fresh_play_rules(&mut g).unwrap();
            g.history.clear();
            g.snapshot();
            if busy {
                apply_command(
                    &mut g.world,
                    &Command::EnableEconomicCompetition {
                        nation: NationId::USA,
                    },
                )
                .unwrap();
                apply_command(
                    &mut g.world,
                    &Command::DeclareWar {
                        attacker: NationId::Iraq,
                        defender: NationId::Kuwait,
                    },
                )
                .unwrap();
            }
            Some(g)
        };
        let resumed_year = if resume.is_some() {
            campaign.as_ref().map(|g| g.world.year)
        } else {
            None
        };
        if let Some(g) = &campaign {
            assert_eq!(
                g.world.player,
                Some(NationId::USA),
                "Profile checkpoint must retain the USA human fixture"
            );
            assert_eq!(g.world.rules.seed, 1990, "Profile checkpoint seed changed");
        }
        for age in [0, 10, 30].into_iter().filter(|n| *n <= years) {
            if resumed_year.is_some_and(|year| 1990 + age < year) {
                continue;
            }
            let slot = format!("profile-{name}-{age}");
            let source_slot = format!("profile-idle_human-{age}");
            if let Some(root) = input.as_ref().filter(|_| !prepare_only) {
                let loaded = storage::read(root, &source_slot, false)
                    .unwrap_or_else(|e| panic!("Cannot measure {source_slot}: {e}"));
                assert_eq!(
                    loaded.world.year,
                    1990 + age,
                    "Checkpoint slot and actual saved year disagree"
                );
                assert_eq!(
                    loaded.world.player,
                    Some(NationId::USA),
                    "Checkpoint player changed"
                );
                assert_eq!(loaded.world.rules.seed, 1990, "Checkpoint seed changed");
                campaign = Some(loaded);
            }
            let g = campaign
                .as_mut()
                .expect("Profile campaign must be prepared or loaded");
            assert_eq!(
                g.world.rules.economic_competition,
                busy && !(input.is_some() && !prepare_only),
                "Checkpoint economic-AI flag must match its scenario"
            );
            assert!(
                g.world.rules.daily_simulation
                    && g.world.rules.physical_logistics
                    && g.world.rules.military_operations,
                "Performance checkpoints must retain the actual daily browser rules"
            );
            while g.world.year < 1990 + age {
                g.advance_days(1, vec![]);
                if g.world.month == 1 && g.world.day == 1 {
                    eprintln!("PROFILE {name}: {}", g.world.year);
                }
            }
            let date = g.world.date_str();
            let checkpoint_hash = format!("{:016x}", spheres_sim::state_hash(&g.world));
            let checkpoint_rules = serde_json::to_value(&g.world.rules).unwrap();
            if prepare_only {
                std::fs::create_dir_all(&campaign_root).unwrap();
                let saved = storage::write(&campaign_root, &slot, g).unwrap();
                results.push(serde_json::json!({"scenario":name,"checkpoint_years":age,
                    "actual_date":date,"prepared":true,"campaign":saved,
                    "checkpoint_world_hash":checkpoint_hash,"actual_rules":checkpoint_rules,
                    "resume_slot":resume,"history_points":g.history.len(),"dispatches_retained":g.log.len()}));
                let report = serde_json::json!({"version":env!("CARGO_PKG_VERSION"),"revision":env!("SPHERES_REVISION"),
                    "seed":1990,"mode":"prepare_only","results":results,
                    "method":"Actual uninterrupted browser-rule worlds advanced to calendar checkpoints and saved with complete archives. No performance samples taken. Resume preserves its actual saved date and skips earlier checkpoint years."});
                std::fs::write(&path, serde_json::to_string_pretty(&report).unwrap()).unwrap();
                eprintln!(
                    "PROFILE PREPARED {name} year {age} at {date} -> {}",
                    campaign_root.display()
                );
                continue;
            }
            let starting_history_points = g.history.len();
            let starting_dispatches = g.log.len();
            let fixture = if input.is_some() && busy {
                establish_profile_stress(g)
            } else {
                serde_json::Value::Null
            };
            let measured_hash = format!("{:016x}", spheres_sim::state_hash(&g.world));
            let measured_rules = serde_json::to_value(&g.world.rules).unwrap();
            let starting_counts = profile_counts(&g.world);
            let stress_export = if input.is_some()
                && busy
                && std::env::var("SPHERES_PROFILE_EXPORT_STRESS").as_deref() == Ok("1")
            {
                let export_root = std::path::Path::new(&path)
                    .parent()
                    .unwrap_or(std::path::Path::new("."))
                    .join("profile-stress-campaigns");
                Some(storage::write(&export_root, &slot, g).unwrap())
            } else {
                None
            };
            let mut simulation = Vec::new();
            let mut read_model = Vec::new();
            let mut serialization = Vec::new();
            let mut history_time = Vec::new();
            let mut whole = Vec::new();
            let mut state_bytes = Vec::new();
            let mut delta_bytes = Vec::new();
            let mut max_wars = starting_counts.0;
            let mut max_projects = starting_counts.1;
            let mut min_wars = starting_counts.0;
            let mut min_projects = starting_counts.1;
            let mut sample_activity = Vec::new();
            for _ in 0..31 {
                let epoch = g.history_epoch;
                let after = g.history.last().unwrap().t;
                let whole_start = Instant::now();
                let start = Instant::now();
                g.advance_days(1, vec![]);
                simulation.push(ms(start));
                let start = Instant::now();
                let state = state_json(&g, None);
                read_model.push(ms(start));
                let start = Instant::now();
                state_bytes.push(serde_json::to_vec(&state).unwrap().len());
                serialization.push(ms(start));
                let start = Instant::now();
                let history = history::request(
                    &g,
                    &format!("/api/history?nations=USA&epoch={epoch}&after={after}"),
                );
                delta_bytes.push(serde_json::to_vec(&history).unwrap().len());
                history_time.push(ms(start));
                whole.push(ms(whole_start));
                let (wars, projects) = profile_counts(&g.world);
                max_wars = max_wars.max(wars);
                max_projects = max_projects.max(projects);
                min_wars = min_wars.min(wars);
                min_projects = min_projects.min(projects);
                sample_activity.push(serde_json::json!({"date":g.world.date_str(),"active_wars":wars,"player_projects":projects}));
            }
            let start = Instant::now();
            let selected = history::request(&g, "/api/history?nations=USA");
            let selected_bytes = serde_json::to_vec(&selected).unwrap().len();
            let selected_ms = ms(start);
            let start = Instant::now();
            let all = history::request(&g, "/api/history");
            let all_bytes = serde_json::to_vec(&all).unwrap().len();
            let all_ms = ms(start);
            let start = Instant::now();
            let production = production_json(&g.world, NationId::USA);
            let production_bytes = serde_json::to_vec(&production).unwrap().len();
            let production_ms = ms(start);
            results.push(serde_json::json!({"scenario":name,"checkpoint_years":age,"starting_date":date,"history_points":g.history.len(),"dispatches_retained":g.log.len(),
                "checkpoint_world_hash":checkpoint_hash,"source_world_hash":checkpoint_hash,"measured_start_world_hash":measured_hash,
                "source_rules":checkpoint_rules,"actual_rules":measured_rules,"stress_fixture":fixture,
                "stress_export":stress_export,
                "source_slot":if input.is_some(){source_slot}else{slot.clone()},
                "starting_history_points":starting_history_points,"starting_dispatches":starting_dispatches,
                "starting_active_wars":starting_counts.0,"starting_player_projects":starting_counts.1,
                "minimum_active_wars":min_wars,"minimum_player_projects":min_projects,"sample_activity":sample_activity,
                "checkpoint_source":if input.is_some(){"aged_idle_campaign"}else{"continuous_fixture"},
                "simulation_and_history_recording":summary(&simulation),"state_read_model":summary(&read_model),"state_serialization":summary(&serialization),"selected_history_delta_and_serialization":summary(&history_time),"whole_server_turn":summary(&whole),
                "max_state_bytes":state_bytes.iter().max(),"max_delta_bytes":delta_bytes.iter().max(),"selected_history_bytes":selected_bytes,"selected_history_ms":selected_ms,"all_history_bytes":all_bytes,"all_history_ms":all_ms,
                "production_room_bytes":production_bytes,"production_room_ms":production_ms,"max_active_wars":max_wars,"max_player_projects":max_projects}));
            let report = serde_json::json!({"version":env!("CARGO_PKG_VERSION"),"revision":env!("SPHERES_REVISION"),"seed":1990,"results":results,
                "mode":if input.is_some(){"measure_input"}else{"continuous"},"input_directory":input.as_ref().map(|p|p.to_string_lossy()),
                "method":"31 consecutive one-day requests from each reported actual starting date. INPUT loads the same aged idle checkpoint independently for both scenarios, retaining its complete archive with no interyear warming. Busy setup is outside timing; its exact priced commands, PC overrides, source/measured hashes and activity are reported. Default mode retains uninterrupted original browser-rule worlds. Timings separate simulation/history recording, state read model, JSON serialization and selected-country delta history. No concurrent workloads should run. No network, browser rendering or disk autosave time included.",
                "busy_fixture":if input.is_some(){"Controlled new stress on an aged idle world, not an uninterrupted economic-AI campaign: USA enacts its budget and queues real construction, eligible economic AI is enabled, and a legal living AI pair declares war through normal priced commands. Require active war and player work at measurement start; report subsequent activity."}else{"Legacy continuous fixture: economic competition and Iraqi declaration against Kuwait enabled in 1990; USA remains human. No claim that projects or wars remain active decades later."},
                "hash_format":"FNV-1a 64-bit over the simulation's canonical serialized world, excluding presentation history; starting history/archive lengths are separately reported.",
                "limits":"Exploratory machine-specific sample, not a statistical performance guarantee. Browser input/render observations are separate. Controlled stress is not daily long-run balance evidence; the daily calibration panel retains actual uninterrupted AI-on trajectories."});
            std::fs::write(&path, serde_json::to_string_pretty(&report).unwrap()).unwrap();
            if input.is_none() {
                // Keep the original default behavior: its archival checkpoint
                // is AFTER the 31 samples. INPUT reports this actual later date
                // honestly, and never rewrites the source checkpoint.
                std::fs::create_dir_all(&campaign_root).unwrap();
                storage::write(&campaign_root, &slot, g).unwrap();
            }
            eprintln!("PROFILE WROTE {name} year {age} -> {path}");
        }
    }
}
