//! Opt-in succession presentation fixtures. Calendar positions and succession
//! events are explicitly authored; neither archive proves a campaign duration.
use super::*;
use serde_json::{json, Value};
use spheres_sim::{government as gov, party_leadership};
use std::{fs, io::Write, path::Path};

fn exclusive(path: &Path, bytes: &[u8]) {
    let mut file = fs::OpenOptions::new().write(true).create_new(true).open(path).unwrap();
    file.write_all(bytes).unwrap();
    file.sync_all().unwrap();
}

fn party<'a>(board: &'a Value) -> &'a Value {
    board["parties"].as_array().unwrap().iter().find(|p| p["party_id"] == "uk_con").unwrap()
}

fn finish(mut g: Game, name: &str, label: &str, reference_date: &str, reference: Value,
          person: &str, scope: &str, actions: Value, destination: &Path) -> Value {
    party_leadership::validate_state(&g.world).unwrap();
    resources::warm(&mut g.world);
    g.history.clear();
    g.snapshot();
    // Import canonicalization precedes every expected reading and world oracle.
    let g = storage::decode(&storage::encode(&g).unwrap()).unwrap();
    let before = save(&g.world);
    let state = state_json(&g, None);
    let government = government_json(&g.world, NationId::UK);
    let campaign = person_portraits::campaign_view(&g.world, NationId::UK);
    let after_reference = person_portraits::reference_view(&g.world, NationId::UK, reference_date).unwrap();
    assert_eq!(government["party_leadership"], campaign);
    assert_eq!(after_reference["parties"], reference["parties"], "Campaign succession must not rewrite historical source rows");
    assert_eq!(save(&g.world), before, "All native readings are pure");
    let archive = storage::encode(&g).unwrap();
    let loaded = storage::decode(&archive).unwrap();
    assert_eq!(save(&loaded.world), save(&g.world));
    assert_eq!(loaded.log, g.log);
    assert_eq!(loaded.history, g.history);
    let file = format!("{name}.json");
    exclusive(&destination.join(&file), archive.as_bytes());
    json!({"name":name,"label":label,"file":file,"scope":scope,"native_actions":actions,
        "player":"UK","date":state["date"],"campaign_date":campaign["date"],"reference_date":reference_date,
        "party_id":"uk_con","focus_person":person,"government":government,"campaign":campaign,
        "reference":after_reference,"log":state["log"],"dispatch_count":state["dispatch_count"],"days_advanced":0})
}

#[test]
#[ignore = "Exports authored succession archives only to a NEW SPHERES_S10F_SUCCESSION_FIXTURE_DIR"]
fn s10f_export_disposable_succession_fixtures() {
    let destination = std::path::PathBuf::from(std::env::var_os("SPHERES_S10F_SUCCESSION_FIXTURE_DIR")
        .expect("Set SPHERES_S10F_SUCCESSION_FIXTURE_DIR to a new disposable directory"));
    assert!(destination.is_absolute());
    assert!(!destination.exists(), "Never overwrite a fixture or save directory");
    let mut initial = Game::new_fresh(13, Some(NationId::UK));
    fresh_play_rules(&mut initial).unwrap();
    let historical = person_portraits::reference_view(&initial.world, NationId::UK, "1990-01-01").unwrap();
    let thatcher = party(&historical)["historical"].as_array().unwrap().iter()
        .find(|e| e["person"]["name"] == "Margaret Thatcher").unwrap()["person"]["id"].as_str().unwrap().to_string();
    assert_eq!(party_leadership::executive_person(&initial.world, NationId::UK).unwrap().id, thatcher);
    let mut same_day = storage::decode(&storage::encode(&initial).unwrap()).unwrap();
    gov::seat_office(&mut same_day.world, NationId::UK, &gov::Succession::Death);
    assert!(same_day.world.party_leadership.as_ref().unwrap().deaths.iter().any(|d| d.person == thatcher));
    assert!(party_leadership::executive_person(&same_day.world, NationId::UK).is_none_or(|p| p.id != thatcher));

    // This is a dated laboratory fixture, not a 37-year run. Calendar movement
    // below resolves no day, election, budget, market or military simulation.
    let mut future = initial;
    future.world.year = 2027;
    future.world.month = 1;
    future.world.day = 1;
    let future_reference = person_portraits::reference_view(&future.world, NationId::UK, "2026-09-07").unwrap();
    gov::seat_office(&mut future.world, NationId::UK, &gov::Succession::Death);
    let first = party_leadership::executive_person(&future.world, NationId::UK).unwrap().id.clone();
    assert_eq!(person_portraits::campaign_view(&future.world, NationId::UK)["executive_person"]["fiction"]["origin"], "fictional_successor");
    future.world.day = 2;
    gov::seat_office(&mut future.world, NationId::UK, &gov::Succession::TermLimit);
    assert_ne!(party_leadership::executive_person(&future.world, NationId::UK).unwrap().id, first);
    let model = person_portraits::campaign_view(&future.world, NationId::UK);
    let holder = party(&model)["campaign"].as_array().unwrap().iter().find(|e| e["person"]["id"] == first).unwrap();
    assert_eq!(holder["campaign_succession"]["party"]["status"], "recorded_holder");
    assert_eq!(holder["campaign_succession"]["national_office"]["status"], "excluded");
    assert_eq!(holder["campaign_succession"]["office_excluded_date"], "2027-01-02");
    assert_eq!(holder["executive_eligibility"]["authorized"], true, "Static role permission remains a separate fact");
    fs::create_dir(&destination).unwrap();
    let cases = vec![
        finish(same_day, "uk-1990-same-day-death", "UK 1990: historical reference survives campaign death", "1990-01-01", historical, &thatcher,
            "Integrated fresh UK seed 13 with one deliberately authored native officeholder Death on 1990-01-01. This is a counterfactual test event, not Thatcher's historical death.",
            json!(["fresh_play_rules on an ordinary UK 1990 seed-13 game", "gov::seat_office Death on 1990-01-01"]), &destination),
        finish(future, "uk-2027-term-limit", "UK 2027: party holder retained after national term limit", "2026-09-07", future_reference, &first,
            "Integrated UK seed 13 calendar authored at 2027-01-01; native Death selects a fictional successor. Calendar authored at 2027-01-02; native TermLimit excludes that person from national office while retaining their party post. No simulated years or days elapsed.",
            json!(["fresh_play_rules on an ordinary UK 1990 seed-13 game", "Set calendar to 2027-01-01 without simulation", "gov::seat_office Death", "Set calendar to 2027-01-02 without simulation", "gov::seat_office TermLimit"]), &destination),
    ];
    let manifest = json!({"version":1,"fixture":"s10f-authored-native-succession-context", "compiled_revision":env!("SPHERES_REVISION"),
        "scope":"Two explicitly authored UK succession presentation fixtures. Ordinary Load, read-only leadership browsing, cancelled Load, Save/Load and Continue must preserve the complete native world. No UI appointments, no browser clock advance, no historical event claim and no campaign-duration certification.",
        "days_advanced":0,"cases":cases});
    exclusive(&destination.join("manifest.json"), serde_json::to_string_pretty(&manifest).unwrap().as_bytes());
    println!("S10F_FIXTURE_MANIFEST={}", destination.join("manifest.json").display());
}
