//! An explicitly authored S16 save supplies starting forces; S17 staff choose
//! all new opposing missions. This is a bounded regression, not a long campaign.
use super::*;
use serde_json::json;
use std::{fs, io::Write, path::Path};
fn put(path: &Path, value: &str) {
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .unwrap();
    f.write_all(value.as_bytes()).unwrap();
    f.sync_all().unwrap();
}
#[test]
#[ignore = "Requires existing SPHERES_S17_INPUT and new absolute SPHERES_S17_FIXTURE_DIR"]
fn s17_export_disposable_staff_fixture() {
    let input = std::path::PathBuf::from(std::env::var_os("SPHERES_S17_INPUT").unwrap());
    let out = std::path::PathBuf::from(std::env::var_os("SPHERES_S17_FIXTURE_DIR").unwrap());
    assert!(input.is_absolute() && input.is_file() && out.is_absolute() && !out.exists());
    let mut g = storage::decode(&fs::read_to_string(&input).unwrap()).unwrap();
    assert_eq!(g.world.player, Some(NationId::France));
    assert!(g.world.military_ai.is_empty());
    // The original S16 fixture deliberately had no autonomous opposition.
    // Enabling its existing economic rule is the sole new authored precondition.
    g.world.rules.economic_competition = true;
    resources::warm(&mut g.world);
    g = storage::decode(&storage::encode(&g).unwrap()).unwrap();
    fs::create_dir(&out).unwrap();
    put(&out.join("before.json"), &storage::encode(&g).unwrap());
    let initial_orders = g.world.air_missions.as_ref().map_or(0, |s| s.orders.len());
    let mut seq = 0;
    for (enabled, name, days) in [(true, "active", 4), (false, "paused", 2)] {
        let command = json!({"kind":"military_ai","enabled":enabled});
        let before = save(&g.world);
        let q = equipment_view::preview(
            &g.world,
            NationId::France,
            &g.session_id,
            &json!({"command":command}),
        )
        .unwrap();
        assert_eq!(q["valid"], true, "{q}");
        assert_eq!(save(&g.world), before);
        seq += 1;
        let payload = json!({"session_id":g.session_id,"client_id":"s17-native","request_seq":seq,"commands":[command]});
        let r = transport::immediate_request(&mut g, &payload).unwrap();
        assert_eq!(r["errors"], json!([]));
        let once = save(&g.world);
        assert_eq!(
            transport::immediate_request(&mut g, &payload).unwrap()["command_replayed"],
            true
        );
        assert_eq!(save(&g.world), once);
        put(
            &out.join(format!("{name}-confirmed.json")),
            &storage::encode(&g).unwrap(),
        );
        let plans = g.world.military_ai.plans.clone();
        let mut resumed = storage::decode(&storage::encode(&g).unwrap()).unwrap();
        for _ in 0..days {
            g.advance_days(1, vec![]);
            resumed.advance_days(1, vec![]);
        }
        assert_eq!(save(&g.world), save(&resumed.world));
        assert_eq!(g.history, resumed.history);
        assert_eq!(g.log, resumed.log);
        assert!(!g.world.military_ai.plans.contains_key(&NationId::France));
        put(
            &out.join(format!("{name}.json")),
            &storage::encode(&g).unwrap(),
        );
        if enabled {
            assert!(
                g.world
                    .air_missions
                    .as_ref()
                    .unwrap()
                    .orders
                    .iter()
                    .skip(initial_orders)
                    .any(|o| o.nation == NationId::Italy
                        && o.report.as_ref().is_some_and(|r| r.stores_used > 0.0)),
                "Actual AI-launched Italian mission must consume paid stores: {:?}",
                g.world.military_ai.plans.get(&NationId::Italy)
            );
        } else {
            assert_eq!(
                g.world.military_ai.plans, plans,
                "Pause stops new staff reviews without cancelling owned obligations"
            );
        }
    }
    let manifest = json!({"revision":env!("SPHERES_REVISION"),"input":input,"player":"France","initial_orders":initial_orders,"days":6,
        "scope":"Authored S16 forces and supplies retained; economic competition explicitly enabled in setup. Only reviewed military staff controls and six ordinary daily advances follow. No new aircraft, cash, research, ammunition or base grants. Includes exact save/resume comparisons and actual autonomous Italian combat with finite stores.",
        "staff":g.world.military_ai,"missions":g.world.air_missions});
    put(
        &out.join("manifest.json"),
        &serde_json::to_string_pretty(&manifest).unwrap(),
    );
    println!("S17_FIXTURE={}", out.display());
}
