//! Native supported-save boundaries for the unified operational path.
use spheres_sim::{apply_command, campaign, init::world_1990, load, operational_warfare,
    save, tick_day, Command};
use spheres_sim::world::{GameRules, NationId as N, WorldState};

fn fixture() -> WorldState {
    let mut w = world_1990(GameRules { daily_simulation:true, military_operations:true,
        physical_logistics:true, resource_market:true, production_system:true,
        ai_aggression:0.0, crisis_intensity:0.0, ..Default::default() });
    w.player=Some(N::France); w.nation_mut(N::France).political_capital=1000.0;
    w
}
fn twice(w:&WorldState)->WorldState {
    let bytes=save(w); let a=load(&bytes).unwrap();
    assert!(save(&a)==bytes,"first load changed campaign property");
    let b=load(&save(&a)).unwrap(); assert!(save(&b)==bytes,"second load changed campaign property"); b
}
#[test]
fn operational_adoption_is_explicit_atomic_and_preserves_existing_fronts() {
    let mut old=world_1990(GameRules::default());let bytes=save(&old);
    assert!(operational_warfare::enable(&mut old).is_err()); assert_eq!(save(&old),bytes);
    let mut w=fixture();
    apply_command(&mut w,&Command::DeclareWar{attacker:N::Iraq,defender:N::Kuwait}).unwrap();
    let fronts=serde_json::to_value(&w.conflicts).unwrap();
    let nations=serde_json::to_value(&w.nations).unwrap();
    assert!(!operational_warfare::has_state(&w));
    apply_command(&mut w,&Command::EnableOperationalWarfare{nation:N::France}).unwrap();
    assert!(campaign::enabled(&w));
    assert_eq!(serde_json::to_value(&w.conflicts).unwrap(),fronts);
    assert_eq!(serde_json::to_value(&w.nations).unwrap(),nations);
    assert!(w.campaign.migration_conflicts.as_ref().unwrap().contains(&w.conflicts[0].id));
    let bytes=save(&w); apply_command(&mut w,&Command::EnableOperationalWarfare{nation:N::France}).unwrap();
    assert_eq!(save(&w),bytes);
    assert!(apply_command(&mut w,&Command::EnableOperationalWarfare{nation:N::Japan}).is_err());
    assert_eq!(save(&w),bytes); twice(&w);
}
#[test]
fn integrated_capabilities_refuse_downgrades_and_nested_unknown_property() {
    let mut w=fixture();operational_warfare::enable(&mut w).unwrap();
    w.campaign.conflict_id_high_water=41;
    let saved=serde_json::from_str::<serde_json::Value>(&save(&w)).unwrap();
    assert_eq!(saved["format"],"spheres-integrated-save");
    for (field,bad) in [("version",2),("warfare_version",0),("company_network_version",1),
        ("supplier_operations_version",1),("economy_version",1),("party_leadership_version",1),("equipment_version",5)] {
        let mut value=saved.clone();value[field]=bad.into();
        assert!(load(&value.to_string()).is_err(),"accepted mismatched {field}");
    }
    let mut value=saved.clone();value["world"]["campaign"]["future_owned_force"]=7.into();
    assert!(load(&value.to_string()).is_err());
    let mut value=saved.clone();value["world"]["campaign_supply"]=serde_json::json!({"future_owned_stock":7});
    assert!(load(&value.to_string()).is_err());
    let mut value=saved.clone();value["world"]["campaign_peace"]=serde_json::json!({"future_treaty":7});
    assert!(load(&value.to_string()).is_err());
    let mut value=saved.clone();value["world"]["rules"]["operational_warfare"]=0.into();
    assert!(load(&value.to_string()).is_err());
    assert_eq!(twice(&w).campaign.conflict_id_high_water,41);
}
#[test]
fn original_master_warfare_dialect_preserves_books_without_loading_events() {
    let mut w=fixture();operational_warfare::enable(&mut w).unwrap();
    apply_command(&mut w,&Command::DeclareWar{attacker:N::Iraq,defender:N::Kuwait}).unwrap();
    tick_day(&mut w,&[]);
    // Source-derived original master dialect: raw worlds and equipment-v1
    // envelopes existed before the integrated wrapper. This is not an archive.
    let raw=serde_json::to_value(&w).unwrap();
    let mut restored=load(&raw.to_string()).unwrap();
    assert_eq!(serde_json::to_value(&restored).unwrap(),raw);
    let envelope=serde_json::json!({"format":"spheres-equipment-save","version":1,"world":raw});
    let wrapped=load(&envelope.to_string()).unwrap();
    assert!(save(&wrapped)==save(&w));twice(&wrapped);
    tick_day(&mut w,&[]);tick_day(&mut restored,&[]);
    assert!(save(&w)==save(&restored),"migrated next day diverged");
}
