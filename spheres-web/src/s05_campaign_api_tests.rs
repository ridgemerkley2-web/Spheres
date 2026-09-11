//! Integrated browser receipts and campaign archives, without an HTTP server.
//! Supplier WIP, deliveries, refits and ammunition have separate lifecycle cases.
use crate::*;
use serde_json::{json, Value};

pub(crate) struct PaidWorkFixture {
    pub game: Game,
    pub formation: Value,
    pub assignment: Value,
    pub project: u32,
    pub supplier: u32,
}

fn request(g: &Game, client: &str, command: Value) -> Value {
    json!({"session_id":g.session_id,"client_id":client,"request_seq":1,"commands":[command]})
}
fn submit(g: &mut Game, payload: &Value) {
    let response=immediate_request(g,payload).unwrap();
    assert_eq!(response["errors"],json!([]),"{response}");
    assert_eq!(response["command_replayed"],false);
}

pub(crate) fn paid_work_fixture() -> PaidWorkFixture {
    let me=NationId::France;
    let mut game=Game::new(1990,Some(me));play_rules(&mut game);
    let district=game.world.districts.iter().find(|(_,owner)|**owner==me).unwrap().0.clone();
    // Disclosed starting fixture authority and one plant, not historical data.
    // Actual commands below create every firm, invoice, assignment and payment.
    if let Some(p)=game.world.production.provinces.iter_mut().find(|p|p.district==district) {p.arms_plants=1;}
    else {game.world.production.provinces.push(spheres_sim::production::ProvinceCapabilities{
        district:district.clone(),arms_plants:1,infrastructure:0,civilian_industry:0,power_grid:0,research_centers:0});}
    spheres_sim::programs::set_construction_budget(&mut game.world,me,0.001).unwrap();
    game.world.nation_mut(me).program_budget.as_mut().unwrap().available_bn[spheres_sim::world::BUDGET_DEFENSE][3]=2.0;
    let adopt=request(&game,"s05-economy",json!({"kind":"enable_connected_economy"}));
    submit(&mut game,&adopt);
    spheres_sim::resources::tick(&mut game.world);
    for (client,kind) in [("s05-companies","enable_companies"),("s05-warfare","enable_operational_warfare")] {
        let payload=request(&game,client,json!({"kind":kind}));submit(&mut game,&payload);
    }
    let saved=save(&game.world);
    let review=equipment_view::preview(&game.world,me,&game.session_id,&json!({"command":{
        "kind":"company_establish","name":"Receipt Test Arsenal","district":district,"capital_mn":25.0}})).unwrap();
    assert_eq!(review["valid"],true,"{review}");assert_eq!(save(&game.world),saved);
    let formation=request(&game,"s05-formation",review["actions"][0]["command"].clone());
    submit(&mut game,&formation);
    let firm=game.world.companies.firms.iter().find(|c|c.nation==me).unwrap();
    let supplier=firm.id;
    assert_eq!(firm.cash_bn,0.0,"Authority is a receivable until the paid day closes");
    assert_eq!(firm.capital_received_bn,0.0);
    assert_eq!(firm.receivables.len(),1);
    assert!((firm.receivables[0].amount_bn-0.025).abs()<1e-12);
    let project=spheres_sim::production::start_project(&mut game.world,me,&district,spheres_sim::production::ProjectKind::Infrastructure).unwrap();
    let company=game.world.sector_contractors.roster.iter().find(|c|c.nation==me&&c.sector==spheres_sim::sector_contractors::CompanySector::Construction).unwrap().id;
    let before=save(&game.world);
    let review=companies_view::preview(&game.world,me,&game.session_id,&json!({"command":{
        "kind":"assign_sector_contractor","company":company,"target":{"kind":"construction","project":project}}})).unwrap();
    assert_eq!(review["valid"],true,"{review}");assert_eq!(save(&game.world),before);
    let assignment=request(&game,"s05-assignment",review["command"].clone());submit(&mut game,&assignment);
    game.record("Integrated paid formation and construction service accepted.".into());
    PaidWorkFixture{game,formation,assignment,project,supplier}
}

#[test]
fn integrated_paid_orders_reconcile_exactly_once_and_changed_receipts_refuse() {
    let PaidWorkFixture{mut game,formation,assignment,..}=paid_work_fixture();
    let before=save(&game.world);let log=game.log.clone();
    for payload in [&formation,&assignment] {
        assert_eq!(immediate_request(&mut game,payload).unwrap()["command_replayed"],true);
        assert_eq!(save(&game.world),before,"A lost response cannot create another firm, invoice, assignment or fee");
        assert_eq!(game.log,log);
        let mut changed=payload.clone();changed["commands"]=json!([]);
        assert!(immediate_request(&mut game,&changed).is_err());
        assert_eq!(save(&game.world),before);
    }
    let mut newer=assignment.clone();newer["request_seq"]=json!(2);newer["commands"]=json!([]);
    assert_eq!(immediate_request(&mut game,&newer).unwrap()["errors"],json!([]));
    assert!(immediate_request(&mut game,&assignment).unwrap_err().requires_review);
    assert_eq!(save(&game.world),before);
}

#[test]
fn integrated_paid_day_retry_archive_reload_and_next_day_keep_one_timeline() {
    let PaidWorkFixture{mut game,formation,assignment,project,supplier}=paid_work_fixture();
    let unsettled=save(&game.world);
    let mut mid_day=storage::decode(&storage::encode(&game).unwrap()).unwrap();
    assert_eq!(save(&mid_day.world),unsettled);
    assert_ne!(mid_day.session_id,game.session_id);
    for old in [&formation,&assignment] {
        assert!(immediate_request(&mut mid_day,old).unwrap_err().requires_review);
        assert_eq!(save(&mid_day.world),unsettled);
    }
    let turn=json!({"session_id":game.session_id,"client_id":"s05-turn","request_seq":1,"days":1,"commands":[]});
    let first=advance_request(&mut game,&turn).unwrap();
    let saved=save(&game.world);let history=game.history.clone();let log=game.log.clone();
    assert_eq!(serde_json::from_str::<Value>(&saved).unwrap()["format"],"spheres-integrated-save");
    let firm=game.world.companies.firms.iter().find(|c|c.id==supplier).unwrap();
    assert_eq!(firm.capital_received_bn,0.025);assert!(firm.receivables.is_empty());
    assert!(game.world.production.industry.projects[&project].company_fees_bn>0.0);
    assert_eq!(advance_request(&mut game,&turn).unwrap(),first);
    assert_eq!(save(&game.world),saved);assert_eq!(game.history,history);assert_eq!(game.log,log);
    let mut changed=turn.clone();changed["days"]=json!(2);
    assert!(advance_request(&mut game,&changed).is_err());assert_eq!(save(&game.world),saved);
    mid_day.advance_days(1,vec![]);
    assert_eq!(save(&mid_day.world),saved,"Loading between invoice and settlement cannot pay twice");
    assert_eq!(mid_day.history,game.history);assert_eq!(mid_day.log,game.log);
    let mut archive=storage::decode(&storage::encode(&game).unwrap()).unwrap();
    let mut standalone=storage::decode(&saved).unwrap();
    assert_eq!(save(&archive.world),saved);assert_eq!(save(&standalone.world),saved);
    assert_eq!(archive.history,game.history);assert_eq!(archive.log,game.log);
    for loaded in [&mut archive,&mut standalone] {
        assert_ne!(loaded.session_id,game.session_id);
        assert!(advance_request(loaded,&turn).unwrap_err().requires_review);
        assert!(immediate_request(loaded,&formation).unwrap_err().requires_review);
        assert_eq!(save(&loaded.world),saved);
    }
    game.advance_days(1,vec![]);archive.advance_days(1,vec![]);standalone.advance_days(1,vec![]);
    assert_eq!(save(&archive.world),save(&game.world));assert_eq!(save(&standalone.world),save(&game.world));
    assert_eq!(archive.history,game.history);assert_eq!(archive.log,game.log);
}
