//! Opt-in, uncontended measurements of the actual browser game configuration.
//! Run with --ignored --exact performance::campaign_lifetime_profile --nocapture
//! and SPHERES_PROFILE_OUT set to a disposable output file. No live server used.
//! PREPARE_ONLY=1 (with the SPHERES_PROFILE_ prefix) saves actual checkpoint
//! campaigns without timing. INPUT names an absolute profile-campaigns directory
//! to measure its saved checkpoints, without warming between years or overwriting
//! inputs. PREPARE_RESUME_SLOT can resume preparation from an INPUT slot.
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
            if let Some(root) = input.as_ref().filter(|_| !prepare_only) {
                let loaded = storage::read(root, &slot, false)
                    .unwrap_or_else(|e| panic!("Cannot measure {slot}: {e}"));
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
            while g.world.year < 1990 + age {
                g.advance_days(1, vec![]);
                if g.world.month == 1 && g.world.day == 1 {
                    eprintln!("PROFILE {name}: {}", g.world.year);
                }
            }
            let date = g.world.date_str();
            if prepare_only {
                std::fs::create_dir_all(&campaign_root).unwrap();
                let saved = storage::write(&campaign_root, &slot, g).unwrap();
                results.push(serde_json::json!({"scenario":name,"checkpoint_years":age,
                    "actual_date":date,"prepared":true,"campaign":saved,
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
            let mut simulation = Vec::new();
            let mut read_model = Vec::new();
            let mut serialization = Vec::new();
            let mut history_time = Vec::new();
            let mut whole = Vec::new();
            let mut state_bytes = Vec::new();
            let mut delta_bytes = Vec::new();
            let mut max_wars = 0usize;
            let mut max_projects = 0usize;
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
                max_wars = max_wars.max(state["wars"].as_array().map_or(0, Vec::len));
                max_projects = max_projects.max(
                    production_json(&g.world, NationId::USA)["queue"]
                        .as_array()
                        .map_or(0, Vec::len),
                );
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
                "checkpoint_source":if input.is_some(){"saved_campaign"}else{"continuous_fixture"},
                "simulation_and_history_recording":summary(&simulation),"state_read_model":summary(&read_model),"state_serialization":summary(&serialization),"selected_history_delta_and_serialization":summary(&history_time),"whole_server_turn":summary(&whole),
                "max_state_bytes":state_bytes.iter().max(),"max_delta_bytes":delta_bytes.iter().max(),"selected_history_bytes":selected_bytes,"selected_history_ms":selected_ms,"all_history_bytes":all_bytes,"all_history_ms":all_ms,
                "production_room_bytes":production_bytes,"production_room_ms":production_ms,"max_active_wars":max_wars,"max_player_projects":max_projects}));
            let report = serde_json::json!({"version":env!("CARGO_PKG_VERSION"),"revision":env!("SPHERES_REVISION"),"seed":1990,"results":results,
                "mode":if input.is_some(){"measure_input"}else{"continuous"},"input_directory":input.as_ref().map(|p|p.to_string_lossy()),
                "method":"31 consecutive one-day requests from each reported actual starting date. INPUT loads independent saved checkpoints with complete archives and no interyear warming; default mode advances uninterrupted browser-rule worlds. Timings separate simulation/history recording, state read model, JSON serialization and selected-country delta history. No concurrent workloads should run. No network, browser rendering or disk autosave time included.",
                "busy_fixture":"Opt-in economic competition for every eligible AI with one priced Iraqi declaration against Kuwait; USA remains human. Observe actual active wars/projects rather than asserting they remain busy decades later.",
                "limits":"Exploratory machine-specific sample, not a statistical performance guarantee. Browser input/render observations are separate. New budgets or human projects are not silently automated."});
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
