//! Opt-in, uncontended measurements of the actual browser game configuration.
//! Run with --ignored --exact performance::campaign_lifetime_profile --nocapture
//! and SPHERES_PROFILE_OUT set to a disposable output file. No live server used.
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
        for age in [0, 10, 30].into_iter().filter(|n| *n <= years) {
            while g.world.year < 1990 + age {
                g.advance_days(1, vec![]);
                if g.world.month == 1 && g.world.day == 1 {
                    eprintln!("PROFILE {name}: {}", g.world.year);
                }
            }
            let date = g.world.date_str();
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
                "simulation_and_history_recording":summary(&simulation),"state_read_model":summary(&read_model),"state_serialization":summary(&serialization),"selected_history_delta_and_serialization":summary(&history_time),"whole_server_turn":summary(&whole),
                "max_state_bytes":state_bytes.iter().max(),"max_delta_bytes":delta_bytes.iter().max(),"selected_history_bytes":selected_bytes,"selected_history_ms":selected_ms,"all_history_bytes":all_bytes,"all_history_ms":all_ms,
                "production_room_bytes":production_bytes,"production_room_ms":production_ms,"max_active_wars":max_wars,"max_player_projects":max_projects}));
            let report = serde_json::json!({"version":env!("CARGO_PKG_VERSION"),"revision":env!("SPHERES_REVISION"),"seed":1990,"results":results,
                "method":"31 consecutive one-day requests at each checkpoint in each uninterrupted browser-rule world. Timings separate simulation/history recording, state read model, JSON serialization and selected-country delta history. No concurrent workloads should run. No network, browser rendering or disk autosave time included.",
                "busy_fixture":"Opt-in economic competition for every eligible AI with one priced Iraqi declaration against Kuwait; USA remains human. Observe actual active wars/projects rather than asserting they remain busy decades later.",
                "limits":"Exploratory machine-specific sample, not a statistical performance guarantee. Browser input/render observations are separate. New budgets or human projects are not silently automated."});
            std::fs::write(&path, serde_json::to_string_pretty(&report).unwrap()).unwrap();
            let campaign_root = std::path::Path::new(&path)
                .parent()
                .unwrap_or(std::path::Path::new("."))
                .join("profile-campaigns");
            std::fs::create_dir_all(&campaign_root).unwrap();
            storage::write(&campaign_root, &format!("profile-{name}-{age}"), &g).unwrap();
            eprintln!("PROFILE WROTE {name} year {age} -> {path}");
        }
    }
}
