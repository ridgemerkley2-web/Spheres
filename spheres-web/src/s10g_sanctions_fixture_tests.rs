//! Opt-in, explicitly authored sanctions fixture. All output goes to a new
//! disposable directory; its diplomatic setup is not historical evidence.
use super::*;
use serde_json::{json, Value};
use std::{fs, io::Write, path::Path};

fn exclusive(path: &Path, bytes: &[u8]) {
    let mut file = fs::OpenOptions::new().write(true).create_new(true).open(path).unwrap();
    file.write_all(bytes).unwrap();
    file.sync_all().unwrap();
}

fn facts(g: &Game) -> Value {
    let state = state_json(g, None);
    json!({"date":state["date"],"agency":state["agency"],"log":state["log"],
        "dispatch_count":state["dispatch_count"],"sanctions":g.world.sanctions,
        "relation":g.world.relation(NationId::France,NationId::Japan),
        "political_capital":g.world.nation(NationId::France).political_capital,
        "trade_depth":g.world.trade_depth(NationId::France,NationId::Japan)})
}

#[test]
#[ignore = "Exports only to a NEW SPHERES_S10G_SANCTIONS_FIXTURE_DIR"]
fn s10g_export_disposable_sanctions_fixture() {
    let destination = std::path::PathBuf::from(std::env::var_os("SPHERES_S10G_SANCTIONS_FIXTURE_DIR")
        .expect("Set SPHERES_S10G_SANCTIONS_FIXTURE_DIR to a new disposable directory"));
    assert!(destination.is_absolute());
    assert!(!destination.exists(), "Never overwrite an existing fixture or campaign directory");
    let mut g = Game::new(13, Some(NationId::France));
    fresh_play_rules(&mut g).unwrap();
    g.world.statecraft.pacts.clear();
    g.world.statecraft.trade.clear();
    g.world.sanctions.clear();
    g.world.conflicts.clear();
    g.world.nation_mut(NationId::France).political_capital = 100.0;
    g.world.set_relation(NationId::France,NationId::Japan,20.0);
    g.world.sanctions.extend([
        (NationId::France,NationId::Japan), (NationId::Japan,NationId::France),
        (NationId::India,NationId::Japan), (NationId::Brazil,NationId::France),
    ]);
    let (a,b) = if NationId::France < NationId::Japan {
        (NationId::France,NationId::Japan)
    } else { (NationId::Japan,NationId::France) };
    g.world.statecraft.trade.push(spheres_sim::world::TradePact {a,b,depth:0.425});
    resources::warm(&mut g.world);
    g.history.clear();
    g.snapshot();
    // Canonicalize exactly as an ordinary campaign import before the oracles.
    let mut g = storage::decode(&storage::encode(&g).unwrap()).unwrap();
    let initial_history = g.history.clone();
    let before = storage::encode(&g).unwrap();
    let before_facts = facts(&g);
    let mut steps = Vec::<Value>::new();
    let mut archives = vec![("before.json".to_owned(), before)];
    for (index,kind) in ["lift","sanction","improve"].into_iter().enumerate() {
        let command = json!({"kind":kind,"target":"Japan"});
        let original = save(&g.world);
        let old_log = g.log.clone();
        let before_step = facts(&g);
        let quote = decision_review::preview(&g,NationId::France,"decisions",&command);
        assert_eq!(quote["valid"],true,"{quote}");
        assert_eq!(save(&g.world),original,"Preview must preserve every world field");
        assert_eq!(g.log,old_log);
        assert_eq!(g.history,initial_history);
        let context = &quote["bilateral_context"];
        assert_eq!(context["actor"],"France");
        assert_eq!(context["target"],"Japan");
        assert_eq!(context["before"]["their_sanctions"],true);
        assert_eq!(context["after"]["their_sanctions"],true);
        assert_eq!(context["after"]["trade_treaty"]["saved"],true);
        assert!(context["after"]["other_sanctioners"].as_array().unwrap().iter().any(|r|r["id"]=="India"));
        let mut expected_world = g.world.clone();
        let native = parse_command(&g.world,&command,NationId::France).unwrap();
        apply_command(&mut expected_world,&native).unwrap();
        let payload = json!({"session_id":g.session_id,"client_id":"s10g-native-fixture",
            "request_seq":index+1,"commands":[command.clone()],"review_kind":"decisions","review_token":quote["review_token"]});
        let response = transport::immediate_request(&mut g,&payload).unwrap();
        assert_eq!(response["errors"],json!([]));
        assert_eq!(save(&g.world),save(&expected_world),"Protected transport must equal native command execution");
        assert!(g.world.is_sanctioning(NationId::Japan,NationId::France));
        assert!(g.world.is_sanctioning(NationId::India,NationId::Japan));
        assert!(g.world.is_sanctioning(NationId::Brazil,NationId::France));
        assert_eq!(g.world.is_sanctioning(NationId::France,NationId::Japan),kind!="lift");
        assert_eq!(g.world.trade_depth(NationId::France,NationId::Japan),0.425);
        let expected_relation = [20.0,5.0,11.0][index];
        let expected_pc = [97.0,91.0,89.0][index];
        assert_eq!(g.world.relation(NationId::France,NationId::Japan),expected_relation);
        assert_eq!(g.world.nation(NationId::France).political_capital,expected_pc);
        assert_eq!(g.history,initial_history,"Immediate decisions must not advance campaign history");
        let archive = storage::encode(&g).unwrap();
        let resumed = storage::decode(&archive).unwrap();
        assert_eq!(save(&resumed.world),save(&g.world));
        assert_eq!(resumed.log,g.log);
        assert_eq!(resumed.history,g.history);
        let filename = format!("expected-after-{kind}.json");
        steps.push(json!({"kind":kind,"command":command,"quote":quote,"before":before_step,
            "after":facts(&g),"expected_file":filename,"days_advanced":0}));
        archives.push((filename,archive));
    }
    let manifest = json!({"version":1,"fixture":"s10g-authored-native-sanctions-desk",
        "compiled_revision":env!("SPHERES_REVISION"),"player":"France","date":before_facts["date"],"days_advanced":0,
        "scope":"Disposable authored France sanctions fixture, reviewed diplomatic commands and save continuity. Not historical sanctions, organic diplomacy or campaign-duration evidence.",
        "authored_preconditions":["Integrated fresh France, seed 13; zero days advanced",
            "Starting defense pacts, trade agreements, sanctions and conflicts cleared",
            "France political capital set to 100 and France-Japan relations set to 20",
            "France sanctions Japan; Japan sanctions France; India sanctions Japan; Brazil sanctions France",
            "A France-Japan trade agreement with saved depth 0.425 is authored before any reviewed command"],
        "native_actions":["Preview then independently execute lift, sanction and improve for France toward Japan",
            "Apply each once through protected immediate native transport and compare every world field with native execution",
            "Save/decode every resulting state, preserving complete world, dated log and campaign history"],
        "before_file":"before.json","before":before_facts,"steps":steps});
    fs::create_dir(&destination).unwrap();
    for (name,archive) in archives { exclusive(&destination.join(name),archive.as_bytes()); }
    exclusive(&destination.join("manifest.json"),serde_json::to_string_pretty(&manifest).unwrap().as_bytes());
    println!("S10G_FIXTURE_MANIFEST={}",destination.join("manifest.json").display());
}
