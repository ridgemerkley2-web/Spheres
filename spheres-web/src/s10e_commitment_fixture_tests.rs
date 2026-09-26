//! Opt-in authored commitment fixture, exported into an exclusive disposable
//! directory. Its treaties and conflict are test setup, not historical evidence.
use super::*;
use serde_json::{json, Value};
use spheres_sim::{agency, commitment, statecraft, war};
use std::{fs, io::Write, path::Path};

fn exclusive(path: &Path, bytes: &[u8]) {
    let mut file = fs::OpenOptions::new().write(true).create_new(true).open(path).unwrap();
    file.write_all(bytes).unwrap();
    file.sync_all().unwrap();
}

fn pair(a: NationId, b: NationId) -> (NationId, NationId) {
    if a < b { (a,b) } else { (b,a) }
}

#[test]
#[ignore = "Exports authored native commitment archives only to a NEW SPHERES_S10E_COMMITMENT_FIXTURE_DIR"]
fn s10e_export_disposable_diplomatic_commitment_fixture() {
    let destination = std::path::PathBuf::from(std::env::var_os("SPHERES_S10E_COMMITMENT_FIXTURE_DIR")
        .expect("Set SPHERES_S10E_COMMITMENT_FIXTURE_DIR to a new disposable directory"));
    assert!(destination.is_absolute(), "The fixture destination must be absolute");
    assert!(!destination.exists(), "Never overwrite an existing fixture or save directory");
    let mut g = Game::new_fresh(13, Some(NationId::France));
    fresh_play_rules(&mut g).unwrap();
    // All of these preconditions are disclosed, including coalition membership.
    g.world.statecraft.pacts.clear();
    g.world.statecraft.trade.clear();
    g.world.sanctions.clear();
    g.world.conflicts.clear();
    g.world.nation_mut(NationId::France).political_capital = 100.0;
    for other in [NationId::Japan, NationId::Brazil, NationId::Kuwait, NationId::India] {
        g.world.set_relation(NationId::France, other, 98.0);
    }
    for (partner, year, month) in [(NationId::Kuwait,1990,1),(NationId::Brazil,1985,6)] {
        let (a,b) = pair(NationId::France,partner);
        g.world.statecraft.pacts.push(spheres_sim::world::Pact { a,b,since_year:year,since_month:month });
    }
    let (a,b) = pair(NationId::France,NationId::Japan);
    g.world.statecraft.trade.push(spheres_sim::world::TradePact {a,b,depth:0.425});
    statecraft::propose_trade(&mut g.world, NationId::India, NationId::France).unwrap();
    let theatre = war::theatre_between(&g.world, NationId::Iraq, NationId::Kuwait);
    let cid = commitment::open_conflict(&mut g.world, NationId::Iraq, NationId::Kuwait, theatre).unwrap();
    let index = g.world.conflicts.iter().position(|c| c.id == cid).unwrap();
    let mut conflict = g.world.conflicts.remove(index);
    war::join_side(&mut conflict,NationId::Jordan,true,2,spheres_sim::world::Objective::Deny);
    war::join_side(&mut conflict,NationId::SaudiArabia,false,2,spheres_sim::world::Objective::Deny);
    agency::offer_call(&mut g.world,&mut conflict,NationId::France,2,true);
    g.world.conflicts.insert(index,conflict);
    resources::warm(&mut g.world);
    g.history.clear();
    g.snapshot();
    // Match ordinary campaign import canonicalization before deriving oracles.
    let mut g = storage::decode(&storage::encode(&g).unwrap()).unwrap();
    let before = storage::encode(&g).unwrap();
    let initial_world = save(&g.world);
    let before_state = state_json(&g,None);
    let before_conflict = serde_json::to_value(g.world.conflict(cid).unwrap()).unwrap();
    let offers = agency::view(&g.world,NationId::France).offers;
    assert_eq!(offers.len(),2);
    let call = offers.iter().find(|o| o.from == NationId::Kuwait).unwrap();
    assert!(call.accept_blocked.is_none(), "Guaranteed call must be answerable: {:?}",call.accept_blocked);
    let command = json!({"kind":"respond_diplomacy","offer":call.id,"accept":true});
    let mut quotes = Vec::<Value>::new();
    for accept in [true,false] {
        let answer = json!({"kind":"respond_diplomacy","offer":call.id,"accept":accept});
        let quote = decision_review::preview(&g,NationId::France,"decisions",&answer);
        assert_eq!(quote["valid"],true,"{quote}");
        quotes.push(quote);
    }
    assert_eq!(save(&g.world),initial_world,"Every native preview must preserve the complete world");
    let quote = decision_review::preview(&g,NationId::France,"decisions",&command);
    let payload = json!({"session_id":g.session_id,"client_id":"s10e-native-fixture","request_seq":1,
        "commands":[command],"review_kind":"decisions","review_token":quote["review_token"]});
    let result = transport::immediate_request(&mut g,&payload).unwrap();
    assert_eq!(result["errors"],json!([]));
    let joined = g.world.conflict(cid).unwrap();
    assert_eq!(joined.side_of(NationId::France),Some(false));
    assert_eq!(joined.posture_of(NationId::France).unwrap().rung,2);
    assert_eq!(joined.posture_of(NationId::France).unwrap().objective,spheres_sim::world::Objective::Deny);
    assert_eq!(g.world.agency.history.iter().filter(|h|h.offer.id==call.id&&h.outcome=="accepted").count(),1);
    assert_eq!(g.world.statecraft.pacts.len(),2);
    assert_eq!(g.world.trade_depth(NationId::France,NationId::Japan),0.425);
    let after = storage::encode(&g).unwrap();
    let after_state = state_json(&g,None);
    let after_conflict = serde_json::to_value(joined).unwrap();
    let mut resumed = storage::decode(&after).unwrap();
    assert_eq!(save(&resumed.world),save(&g.world));
    assert_eq!(resumed.log,g.log);
    assert_eq!(resumed.history,g.history);
    // A single ordinary day is a continuation oracle, not campaign duration.
    g.advance_days(1,vec![]);
    resumed.advance_days(1,vec![]);
    assert_eq!(save(&resumed.world),save(&g.world));
    assert_eq!(resumed.log,g.log);
    assert_eq!(resumed.history,g.history);
    let after_day = storage::encode(&g).unwrap();
    let day_state = state_json(&g,None);
    let manifest = json!({
        "version":1,"fixture":"s10e-authored-native-diplomatic-commitments",
        "compiled_revision":env!("SPHERES_REVISION"),
        "scope":"Disposable authored commitments, reply and one-day continuation fixture. Not historical treaties, organically generated offers, a full war or campaign-certification duration evidence.",
        "authored_preconditions":["Integrated fresh France, seed 13", "All starting defense pacts, trade agreements, sanctions and conflicts cleared", "France political capital set to 100; relations with Japan, Brazil, Kuwait and India set to 98", "France-Kuwait pact dated January 1990 and France-Brazil pact dated June 1985 authored", "France-Japan trade integration authored at 0.425", "Jordan authored on Iraq's side and Saudi Arabia on Kuwait's side, both at rung 2"],
        "native_actions":["India proposes trade to France through statecraft::propose_trade", "Native Iraq-Kuwait conflict opened and guaranteed rung-2 France call queued through agency::offer_call", "Preview both call replies without mutation", "Accept the call once through protected native review transport", "Save/load complete joined state and independently compare one ordinary subsequent day"],
        "player":"France","date":before_state["date"],"after_date":after_state["date"],"day_date":day_state["date"],"days_advanced":1,"reply_days_advanced":0,
        "before_file":"before.json","expected_after_file":"expected-after.json","expected_day_file":"expected-after-day.json",
        "conflict_id":cid,"command":command,"quotes":quotes,
        "before_conflict":before_conflict,"after_conflict":after_conflict,"day_conflict":g.world.conflict(cid),
        "before_agency":before_state["agency"],"after_agency":after_state["agency"],"day_agency":day_state["agency"],
        "before_log":before_state["log"],"after_log":after_state["log"],"day_log":day_state["log"],
        "before_dispatch_count":before_state["dispatch_count"],"after_dispatch_count":after_state["dispatch_count"],"day_dispatch_count":day_state["dispatch_count"]
    });
    fs::create_dir(&destination).unwrap();
    exclusive(&destination.join("before.json"),before.as_bytes());
    exclusive(&destination.join("expected-after.json"),after.as_bytes());
    exclusive(&destination.join("expected-after-day.json"),after_day.as_bytes());
    exclusive(&destination.join("manifest.json"),serde_json::to_string_pretty(&manifest).unwrap().as_bytes());
    println!("S10E_FIXTURE_MANIFEST={}",destination.join("manifest.json").display());
}
