//! Explicit compatibility evidence from the original active compiled exporters.
//! Not part of the normal test run: requires all 28 recorded, unchanged saves.
use crate::*;
use serde_json::{json,Value};
use std::{fs,path::Path};

fn property(w:&WorldState)->Value {
    json!({"companies":w.companies,"production":w.production,"resources":w.resources,
        "date":[w.year,w.month,w.day],"nations":w.nations.iter().map(|n|json!({
            "id":n.id,"alive":n.alive,"gdp":n.gdp,"population":n.population,
            "treasury":n.treasury_bn.unwrap_or(0.0),"debt":n.debt_bn.unwrap_or(n.debt_gdp*n.gdp),"debt_ratio":n.debt_gdp,
            "programs":n.program_budget,"arsenal":n.arsenal,"equipment":n.equipment})).collect::<Vec<_>>()})
}
fn unadopted(w:&WorldState,file:&str){
    assert!(!spheres_sim::connected_economy::has_state(w),"{file}: loading cannot adopt S02");
    assert!(!spheres_sim::company_network::has_state(w),"{file}: loading cannot adopt S03");
    assert!(!spheres_sim::campaign::enabled(w),"{file}: loading cannot adopt S04");
    assert!(!w.rules.historical_party_leadership&&w.party_leadership.is_none(),"{file}: loading cannot adopt historical succession");
}
fn assert_phase(w:&WorldState,row:&Value){
    let family=row["family"].as_str().unwrap();let phase=row["phase"].as_str().unwrap();let file=row["file"].as_str().unwrap();
    let products:Vec<_>=w.companies.firms.iter().flat_map(|c|&c.products).collect();
    let refits:Vec<_>=w.companies.firms.iter().flat_map(|c|&c.refits).collect();
    let ammo:Vec<_>=w.companies.firms.iter().flat_map(|c|&c.ammunition_products).collect();
    let holds=|air:bool|w.nation(NationId::France).arsenal.held.iter().filter(|h|h.design_id.as_ref().and_then(|id|w.nation(NationId::France).equipment.as_ref()?.revisions.get(id)).is_some_and(|r|r.spec.platform.starts_with("air_")==air)).map(|h|h.units).sum::<f64>();
    let valid=match (family,phase){
        ("tank","eligible")=>w.companies.is_empty(),
        ("tank"|"mixed","development")=>products.iter().any(|p|p.certified_day.is_none()&&p.development_work_days>0.0),
        ("tank","stock")=>products.iter().map(|p|p.stock).sum::<u32>()==2&&holds(false)==0.0,
        ("mixed","ground-stock")=>products.iter().map(|p|p.stock).sum::<u32>()==1,
        ("mixed","stock")=>products.iter().filter(|p|p.stock==1).count()==2,
        ("tank"|"mixed","transit")=>w.companies.deliveries.iter().all(|d|d.settled_day.is_some()&&d.delivered_day.is_none())&&w.companies.deliveries.len()==if family=="tank"{1}else{2},
        ("tank","arrived")=>holds(false)==1.0&&w.companies.deliveries.iter().any(|d|d.delivered_day.is_some()),
        ("mixed","arrived")=>holds(false)==1.0&&holds(true)==1.0,
        ("ammunition","stock")=>ammo.iter().map(|p|p.stock).sum::<u32>()==30_120&&w.companies.ammunition_deliveries.is_empty(),
        ("ammunition","awaiting-settlement")=>w.companies.ammunition_deliveries.len()==2&&w.companies.ammunition_deliveries.iter().all(|d|d.settled_day.is_none()),
        ("ammunition","transit")=>w.companies.ammunition_deliveries.len()==2&&w.companies.ammunition_deliveries.iter().all(|d|d.settled_day.is_some()&&d.delivered_day.is_none()),
        ("ammunition","arrived")=>w.companies.ammunition_deliveries.len()==2&&w.companies.ammunition_deliveries.iter().all(|d|d.delivered_day.is_some()),
        ("ground-refit"|"aircraft-refit","ready")=>refits.is_empty()&&holds(family=="aircraft-refit")==4.0,
        ("ground-refit"|"aircraft-refit",phase)=>refits.len()==1&&match phase{
            "awaiting-settlement"=>refits[0].settled_day.is_none(),
            "escrow-settled"=>refits[0].settled_day.is_some()&&refits[0].escrow_bn>0.0&&refits[0].unit_work_days==0.0,
            "active-work"=>refits[0].unit_work_days>0.0&&refits[0].cancelled_units==0,
            "partial-cancel"=>refits[0].unit_work_days>0.0&&refits[0].cancelled_units==2,
            "cancelled-remainder-returned"=>refits[0].completed_units==1&&refits[0].cancelled_units==2,
            "complete"=>refits[0].completed_units==3&&refits[0].cancelled_units==0,
            _=>false},
        _=>false};
    assert!(valid,"{file}: input does not contain the promised original {family}/{phase} lifecycle phase");
}
fn replay_one_day(mut game:Game,file:&str){
    let mut resumed=storage::decode(&storage::encode(&game).unwrap()).unwrap();
    assert!(save(&resumed.world)==save(&game.world),"{file}: campaign archive changed the loaded state");
    assert!(resumed.history==game.history&&resumed.log==game.log,"{file}: archive changed presentation history");
    game.advance_days(1,vec![]);resumed.advance_days(1,vec![]);
    assert!(save(&resumed.world)==save(&game.world),"{file}: next-day resumed simulation diverged");
    assert!(resumed.history==game.history&&resumed.log==game.log,"{file}: next-day archive history diverged");
    let once=save(&game.world);let twice=storage::decode(&once).unwrap();
    assert!(save(&twice.world)==once,"{file}: next-day saved state is not canonical");
}
fn qualify(root:&Path,row:&Value){
    let file=row["file"].as_str().unwrap();let path=root.join(file);let bytes=fs::read(&path).unwrap();
    assert_eq!(bytes.len() as u64,row["bytes"].as_u64().unwrap(),"{file}: fixture changed since manifest");
    let text=std::str::from_utf8(&bytes).unwrap();let native=load(text).unwrap();
    assert_phase(&native,row);unadopted(&native,file);let original=property(&native);
    let game=storage::decode(text).unwrap();unadopted(&game.world,file);
    assert!(property(&game.world)==original,"{file}: browser import changed owned funds, reservations, specifications or paid work");
    assert!(native.nations.iter().map(|n|(n.id,n.treasury_bn,n.debt_bn)).eq(game.world.nations.iter().map(|n|(n.id,n.treasury_bn,n.debt_bn))),"{file}: loading opened a previously unopened fiscal book");
    let canonical=save(&game.world);let twice=storage::decode(&canonical).unwrap();
    assert!(save(&twice.world)==canonical,"{file}: second browser load changed canonical state");
    replay_one_day(twice,file);
    // Start the adoption branch from the original loaded day, not its later copy.
    // The S02 explicit observer seats unopened books at zero cash and existing
    // debt_ratio*GDP. Compare those effective balances; never require a grant.
    let mut adopted=game;let before=property(&adopted.world);let me=adopted.world.player.unwrap();
    for command in [Command::EnableConnectedEconomy{nation:me},Command::EnableCompanies{nation:me},Command::EnableOperationalWarfare{nation:me}] {
        apply_command(&mut adopted.world,&command).unwrap();
        assert!(property(&adopted.world)==before,"{file}: explicit adoption changed existing property");
    }
    assert!(spheres_sim::connected_economy::has_state(&adopted.world)&&spheres_sim::company_network::has_state(&adopted.world)&&spheres_sim::campaign::enabled(&adopted.world));
    let party=if adopted.world.rules.ideology_blocs {
        spheres_sim::party_leadership::enable_campaign(&mut adopted.world).unwrap();
        assert!(adopted.world.party_leadership.is_some());
        assert!(property(&adopted.world)==before,"{file}: historical leadership enrollment changed equipment or financial ownership");
        true
    }else{false};
    replay_one_day(adopted,file);
    assert!(fs::read(path).unwrap()==bytes,"{file}: input bytes were modified during qualification");
    println!("S05_ACTIVE_FIXTURE {}",json!({"file":file,"sha256":row["sha256"],"family":row["family"],"phase":row["phase"],"import":"native through browser storage","canonical_loads":2,"archive_next_day":"exact","explicit_adoption_next_day":"exact","party_adopted":party}));
}

#[test]
#[ignore="requires original pinned active exporter files and provenance in SPHERES_S05_ACTIVE_FIXTURES"]
fn original_active_company_stages_preserve_property_across_load_and_explicit_adoption(){
    let root=std::path::PathBuf::from(std::env::var_os("SPHERES_S05_ACTIVE_FIXTURES").expect("Set SPHERES_S05_ACTIVE_FIXTURES to the recorded original-exporter directory"));
    assert!(root.is_absolute());
    let layout:Value=serde_json::from_str(include_str!("../../tools/campaign/active-fixture-layout.json")).unwrap();
    let manifest:Value=serde_json::from_slice(&fs::read(root.join("active-fixture-manifest.json")).unwrap()).unwrap();
    assert_eq!(manifest["format"],"spheres-active-fixture-matrix");assert_eq!(manifest["version"],1);
    assert_eq!(manifest["source_revision"],layout["source_revision"]);
    assert_eq!(manifest["source_binaries"].as_array().unwrap().len(),3);
    let rows=manifest["fixtures"].as_array().unwrap();let expected=layout["fixtures"].as_array().unwrap();
    assert_eq!(rows.len(),28);assert_eq!(rows.len(),expected.len());
    let mut failures=Vec::new();
    for (row,wanted) in rows.iter().zip(expected){
        for field in ["file","family","phase"]{assert_eq!(row[field],wanted[field]);}
        let digest=row["sha256"].as_str().unwrap();assert!(digest.len()==64&&digest.bytes().all(|b|b.is_ascii_hexdigit()));
        if std::panic::catch_unwind(std::panic::AssertUnwindSafe(||qualify(&root,row))).is_err(){failures.push(row["file"].as_str().unwrap().to_string());}
    }
    assert!(failures.is_empty(),"Original active fixture failures: {}",failures.join(", "));
}
