//! Planner ownership and opt-in boundaries. The fresh-world operating journey
//! is separate; these tests do not grant money, plant levels or vehicle stock.
use spheres_sim::{clock, companies, load, save, supplier_catalogue as catalogue,
    world::{GameRules, NationId as N, WorldState}};

fn world()->WorldState {
    spheres_sim::init::world_1990(GameRules {daily_simulation:true,
        military_operations:true, production_system:true, manufacturing_system:true,
        resource_market:true, economic_competition:true,
        ai_aggression:0.0,crisis_intensity:0.0,..Default::default()})
}
#[test]
fn catalogue_adoption_is_metadata_only_and_passive_views_are_pure() {
    let mut w=world();
    let before=save(&w);
    let view=catalogue::view(&w);
    assert_eq!(view["programs"].as_array().unwrap().len(),7);
    assert!(view["programs"].as_array().unwrap().iter().all(|p|p["status"]=="not_enabled"));
    assert_eq!(save(&w),before);
    catalogue::enable(&mut w).unwrap();
    assert!(w.companies.is_empty());
    assert!(w.production.projects.is_empty());
    let mut reverted=w.clone(); reverted.supplier_catalogue=Default::default();
    assert_eq!(save(&reverted),before);
    let enabled=save(&w);
    catalogue::enable(&mut w).unwrap(); catalogue::view(&w);
    assert_eq!(save(&w),enabled);
    assert_eq!(save(&load(&enabled).unwrap()),enabled);
}
#[test]
fn old_worlds_and_current_player_are_never_directed() {
    let mut w=world();
    let before=save(&w);
    catalogue::tick_day(&mut w);
    assert_eq!(save(&w),before);
    catalogue::enable(&mut w).unwrap();
    w.player=Some(N::France);
    while clock::absolute_day(&w).rem_euclid(30)!=0 {clock::advance_date(&mut w);}
    let before=save(&w);
    catalogue::tick_day(&mut w);
    assert_eq!(save(&w),before);
    w.player=Some(N::Tonga);
    w.rules.economic_competition=false;
    let before=save(&w); catalogue::tick_day(&mut w); assert_eq!(save(&w),before);
}
#[test]
fn scheduled_review_orders_paid_work_once_without_completed_property() {
    let mut w=world(); w.player=Some(N::Tonga);
    catalogue::enable(&mut w).unwrap();
    while clock::absolute_day(&w).rem_euclid(30)!=0 {clock::advance_date(&mut w);}
    let original=save(&w);
    let buyer=w.nation(N::Tonga).clone();
    catalogue::tick_day(&mut w);
    let plan=w.supplier_catalogue.plans.get(&N::France).unwrap();
    assert_eq!(plan.last_review_day,Some(clock::absolute_day(&w)));
    assert_eq!(plan.status,"construction","{}",plan.reason);
    assert_eq!(w.production.projects.len(),1);
    assert_eq!(w.production.projects[0].progress_days,0.0);
    assert_eq!(w.production.projects[0].kind,spheres_sim::production::ProjectKind::ArmsPlant);
    assert_eq!(serde_json::to_value(w.nation(N::Tonga)).unwrap(),serde_json::to_value(&buyer).unwrap());
    assert!(w.companies.firms.is_empty());
    assert!(companies::imports_enabled(&w,N::France));
    let once=save(&w); assert_ne!(once,original);
    catalogue::tick_day(&mut w); assert_eq!(save(&w),once);
    catalogue::validate(&w).unwrap();
    assert_eq!(save(&load(&once).unwrap()),once);
}
#[test]
fn malformed_planner_metadata_is_rejected_on_load() {
    let mut w=world();
    catalogue::enable(&mut w).unwrap();
    w.rules.manufacturing_system=false;
    assert!(load(&save(&w)).is_err());
    w=world();
    w.supplier_catalogue.plans.insert(N::France,Default::default());
    assert!(load(&save(&w)).is_err());
    w.supplier_catalogue.enabled=true;
    w.supplier_catalogue.plans.get_mut(&N::France).unwrap().company=Some(999);
    assert!(load(&save(&w)).is_err());
    w.supplier_catalogue.plans.clear();
    w.supplier_catalogue.plans.insert(N::Tonga,Default::default());
    assert!(load(&save(&w)).is_err());
}
