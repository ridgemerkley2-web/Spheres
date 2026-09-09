//! Read-only audit of an actual web campaign save and the narrow slot upgrade.
//! The input file is never rewritten and no simulation tick is executed.
use serde_json::{json, Value};
use spheres_sim::{party_leadership, world::WorldState};

fn strip_reviewed_component_slots(w: &WorldState) -> Value {
    let mut value = serde_json::to_value(w).unwrap();
    value["party_leadership"]["assignments"].as_array_mut().unwrap().retain(|a| {
        !((a["nation"] == "Japan" && matches!(a["party"].as_str(), Some("jp_jsp" | "jp_komeito")))
            || (a["nation"] == "France" && a["party"] == "fr_udf")
            || (a["nation"] == "Germany" && a["party"] == "de_union"))
    });
    value
}
fn main() {
    let path = std::env::args().nth(1).expect("provide the saved campaign path");
    let original_bytes = std::fs::read(&path).expect("read saved campaign");
    let root: Value = serde_json::from_slice(&original_bytes).expect("parse saved campaign");
    let mut simulation = if root["format"] == "spheres-campaign" { root["world"].clone() } else { root };
    if let Some(text) = simulation.as_str() { simulation = serde_json::from_str(text).unwrap(); }
    let world = if simulation.get("format").is_some() { simulation["world"].clone() } else { simulation.clone() };
    let before: WorldState = serde_json::from_value(world).expect("decode original world without upgrading it");
    let loaded = spheres_sim::load(&simulation.to_string()).expect("load saved campaign with structural upgrade");
    party_leadership::validate_state(&loaded).expect("strict party identity validation");
    assert_eq!(strip_reviewed_component_slots(&before), strip_reviewed_component_slots(&loaded),
        "the upgrade may not alter any other campaign state");
    let old_book = before.party_leadership.as_ref().unwrap();
    let new_book = loaded.party_leadership.as_ref().unwrap();
    let mut expanded = 0;
    let mut new_slots = 0;
    for (nation,party,count) in [("Japan","jp_jsp",2),("Japan","jp_komeito",3),
        ("France","fr_udf",11),("Germany","de_union",2)] {
        let old = old_book.assignments.iter().filter(|a| a.nation.code() == nation && a.party == party).collect::<Vec<_>>();
        let new = new_book.assignments.iter().filter(|a| a.nation.code() == nation && a.party == party).collect::<Vec<_>>();
        let schema = party_leadership::roster().unwrap().parties.iter()
            .find(|p|p.nation.code()==nation&&p.party==party).unwrap();
        if schema.components.is_empty() {
            assert!(matches!(nation,"France"|"Germany"),"Japanese reviewed schema must be present");
            assert_eq!(json!(old),json!(new));
            continue;
        }
        assert_eq!(schema.components.len(),count);
        assert_eq!(new.len(), count);
        if old.len() == 1 && old[0].component.is_none() {
            assert!(old[0].holders.is_empty());
            assert!(new.iter().all(|a| a.holders.is_empty() && a.since_day == old[0].since_day
                && a.reason == old[0].reason && a.component.is_some()));
            expanded += 1;
            new_slots += count;
        } else { assert_eq!(json!(old), json!(new)); }
    }
    let saved = spheres_sim::save(&loaded);
    assert_eq!(spheres_sim::save(&spheres_sim::load(&saved).unwrap()), saved, "second load is unchanged");
    assert_eq!(std::fs::read(&path).unwrap(), original_bytes, "input file must remain byte-for-byte unchanged");
    println!("{}", json!({"valid":true,"expanded_legacy_party_slots":expanded,
        "new_component_slots":new_slots,"other_campaign_state_unchanged":true,"holders_assigned":0,
        "save_roundtrip_stable":true,"input_file_unchanged":true}));
}
