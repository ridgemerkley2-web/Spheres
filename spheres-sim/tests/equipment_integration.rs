//! Accounting and physical-inventory boundaries of the opt-in tank designer.
use spheres_sim::{arsenal, clock, equipment, manufacturing, operations, production, programs, tech, war};
use spheres_sim::world::{GameRules, NationId as N, WorldState, BUDGET_DEFENSE as D};

fn world() -> WorldState {
    let mut w = spheres_sim::init::world_1990(GameRules {
        daily_simulation: true, military_operations: true, manufacturing_system: true,
        resource_market: true, ..GameRules::default()
    });
    w.player = Some(N::USA);
    w.nation_mut(N::USA).political_capital = 1000.0;
    w
}
fn revision(w: &mut WorldState, id: &str) {
    let spec = equipment::baseline_spec();
    let quote = equipment::design_preview(w, N::USA, &spec);
    assert!(quote.valid, "{:?}", quote.blockers);
    let day = clock::absolute_day(w);
    let n = w.nation_mut(N::USA);
    let state = n.equipment.get_or_insert_with(Default::default);
    state.revisions.insert(id.into(), equipment::DesignRevision {
        id:id.into(), name:id.into(), specification_key:equipment::specification_key(&spec),
        spec, profile:quote.profile.unwrap(), created_day:day, certified_day:Some(day),
    });
}
fn enroll(w: &mut WorldState) {
    programs::set_construction_budget(w, N::USA, 0.0).unwrap();
    programs::begin_day(w);
}
fn activate(w: &mut WorldState, from: i32) {
    let n = w.nation_mut(N::USA);
    let p = n.program_budget.as_ref().unwrap();
    let base = p.departments[D][0] as f64 + p.departments[D][1] as f64;
    let scale = (base + p.departments[D][4] as f64) / base;
    let s = n.equipment.get_or_insert_with(Default::default);
    s.finance_from_day = from;
    s.force_support_scale = scale;
}
fn near(a: f64, b: f64) { assert!((a-b).abs() < 1e-10, "{a} != {b}"); }
fn plant(w: &mut WorldState) -> String {
    let district = w.districts.iter().find(|(_, owner)| **owner == N::USA).unwrap().0.clone();
    if let Some(p) = w.production.provinces.iter_mut().find(|p|p.district == district) {
        p.arms_plants = 1;
    } else {
        w.production.provinces.push(production::ProvinceCapabilities {
            district:district.clone(), infrastructure:0, civilian_industry:0,
            power_grid:0, research_centers:0, arms_plants:1,
        });
        w.production.provinces.sort_by(|a,b|a.district.cmp(&b.district));
    }
    district
}

#[test]
fn legacy_inventory_serialization_and_fractional_loss_remain_exact() {
    let mut h: arsenal::Holding = serde_json::from_str(r#"{"kit":"arm_gen3","units":25.0,"age":12.0}"#).unwrap();
    assert!(h.design_id.is_none());
    assert_eq!(serde_json::to_value(&h).unwrap(), serde_json::json!({"kit":"arm_gen3","units":25.0,"age":12.0}));
    let o: arsenal::Order = serde_json::from_str(r#"{"kit":"arm_gen3","units":25.0,"due":3}"#).unwrap();
    assert_eq!(serde_json::to_value(o).unwrap(), serde_json::json!({"kit":"arm_gen3","units":25.0,"due":3}));
    let mut expected = h.units;
    for _ in 0..4 { expected *= 1.0 - 0.01; arsenal::apply_holding_loss(&mut h, 0.01); }
    assert_eq!(h.units.to_bits(), expected.to_bits());
}

#[test]
fn custom_delivery_waits_and_never_merges_into_the_legacy_base_kit() {
    let mut w = world();
    revision(&mut w, "delivery");
    for n in &mut w.nations { if n.id != N::USA { n.alive = false; } }
    let n = w.nation_mut(N::USA);
    n.mil_spend_gdp = 0.0; n.arsenal.banked = 0.0;
    n.arsenal.held = vec![serde_json::from_value(serde_json::json!({"kit":"arm_gen3","units":3.0,"age":100.0})).unwrap()];
    n.arsenal.orders.clear();
    arsenal::queue_design_order(n, "delivery", 5, 2, 12.0).unwrap();
    arsenal::tick(&mut w);
    assert_eq!(w.nation(N::USA).arsenal.held.len(), 1);
    assert_eq!(w.nation(N::USA).arsenal.orders[0].due_days, Some(1));
    clock::advance_date(&mut w); arsenal::tick(&mut w);
    let n = w.nation(N::USA);
    assert_eq!(n.arsenal.held.len(), 2);
    assert_eq!(n.arsenal.held.iter().find(|h|h.design_id.is_none()).unwrap().units, 3.0);
    let h = n.arsenal.held.iter().find(|h|h.design_id.is_some()).unwrap();
    assert_eq!((h.units, h.age), (5.0, 12.0));
    assert!(n.arsenal.orders.is_empty());
}

#[test]
fn refits_reserve_once_preserve_current_age_and_exclude_battlefield_losses() {
    let mut w = world(); revision(&mut w,"old"); revision(&mut w,"new");
    let n = w.nation_mut(N::USA); n.arsenal.held.clear();
    arsenal::deliver_design(n,"old",10,25.0).unwrap();
    assert_eq!(arsenal::reserve_refit(n,"old",6).unwrap(),25.0);
    let before = serde_json::to_string(&n.arsenal).unwrap();
    assert!(arsenal::reserve_refit(n,"old",5).is_err());
    assert_eq!(before,serde_json::to_string(&n.arsenal).unwrap());
    n.arsenal.held[0].age = 27.0;
    arsenal::apply_holding_loss(&mut n.arsenal.held[0],0.5);
    assert_eq!(n.arsenal.held[0].units,8.0);
    assert_eq!(n.arsenal.held[0].refit_reserved,6);
    arsenal::complete_refit(n,"old","new",4).unwrap();
    let target=n.arsenal.held.iter().find(|h|h.design_id.as_deref()==Some("new")).unwrap();
    assert_eq!((target.units,target.age),(4.0,27.0));
    arsenal::release_refit(n,"old",2).unwrap();
    assert_eq!(n.arsenal.held.iter().map(|h|h.units).sum::<f64>(),8.0);
    assert_eq!(n.arsenal.held.iter().map(arsenal::available_design_units).sum::<u32>(),8);
}

#[test]
fn fractional_custom_attrition_carries_until_one_whole_vehicle_is_lost() {
    let mut w=world();revision(&mut w,"tank");let n=w.nation_mut(N::USA);n.arsenal.held.clear();
    arsenal::deliver_design(n,"tank",25,0.0).unwrap();
    for _ in 0..3 {arsenal::apply_holding_loss(&mut n.arsenal.held[0],0.01);assert_eq!(n.arsenal.held[0].units,25.0);}
    arsenal::apply_holding_loss(&mut n.arsenal.held[0],0.01);
    assert_eq!(n.arsenal.held[0].units,24.0);assert_eq!(n.arsenal.held[0].loss_remainder,0.0);
}

#[test]
fn custom_price_changes_book_value_but_not_combat_and_components_only_change_land() {
    let mut w=world();revision(&mut w,"tank");let n=w.nation_mut(N::USA);n.arsenal.held.clear();
    arsenal::deliver_design(n,"tank",100_000,0.0).unwrap();
    let before=(arsenal::book_value(n),arsenal::adequacy(n),operations::capabilities(n),war::sustained_force(n,n.mil_spend_gdp));
    n.equipment.as_mut().unwrap().revisions.get_mut("tank").unwrap().profile.unit_cost_bn *= 10.0;
    near(arsenal::book_value(n),before.0*10.0);assert_eq!(arsenal::adequacy(n),before.1);
    assert_eq!(war::sustained_force(n,n.mil_spend_gdp),before.3);
    n.equipment.as_mut().unwrap().revisions.get_mut("tank").unwrap().profile.land_factor=1.25;
    let after=operations::capabilities(n);
    near(after.land,before.2.land*1.25);assert_eq!(after.strike,before.2.strike);assert_eq!(after.lift,before.2.lift);
    n.equipment.as_mut().unwrap().maintenance_fraction=0.0;
    assert!(arsenal::adequacy(n)<before.1);
}

#[test]
fn mapped_hardware_discovery_does_not_upgrade_unmodified_custom_stock() {
    let mut w=world();revision(&mut w,"tank");let n=w.nation_mut(N::USA);n.arsenal.held.clear();
    arsenal::deliver_design(n,"tank",1000,0.0).unwrap();
    n.tech.grant_1990(&[]);let before=arsenal::combat_technology(n);
    let hardware=tech::index_of("aero_active_protection_system").unwrap();
    n.tech.grant_1990(&[hardware]);
    assert!(tech::military_multiplier(n)>before.0);
    near(arsenal::combat_technology(n).0,before.0);near(arsenal::combat_technology(n).1,before.1);
    let organizational=tech::registry().iter().position(|t|t.id.starts_with("core_")&&t.effects.iter().any(|e|matches!(e,tech::Effect::MilitaryEfficiency(x) if *x>0.0))).unwrap() as u16;
    n.tech.grant_1990(&[hardware,organizational]);
    assert!(arsenal::combat_technology(n).0>before.0,"organizational capability is retained");
    n.equipment=None;n.arsenal.held.clear();
    assert_eq!(arsenal::combat_technology(n),(tech::military_multiplier(n),tech::military_floor(n)));
}

#[test]
fn development_switches_only_on_next_funding_day_and_pays_only_delivered_work() {
    let mut w=world();enroll(&mut w);let today=clock::absolute_day(&w);
    let old=w.nation(N::USA).program_budget.clone();let support=programs::force_support_share(w.nation(N::USA),w.nation(N::USA).mil_spend_gdp);
    activate(&mut w,today+1);
    assert!(!programs::equipment_finance_active(w.nation(N::USA)));
    assert_eq!(programs::available_bn(&w,N::USA,D,4),0.0);
    programs::begin_day(&mut w);assert_eq!(w.nation(N::USA).program_budget,old);
    clock::advance_date(&mut w);programs::begin_day(&mut w);
    assert!(programs::equipment_finance_active(w.nation(N::USA)));
    assert!(!programs::is_project_funded_on(w.nation(N::USA),D,4,today));
    assert!(programs::is_project_funded_on(w.nation(N::USA),D,4,today+1));
    let available=programs::available_bn(&w,N::USA,D,4);assert!(available>0.0);
    assert_eq!(w.nation(N::USA).program_budget.as_ref().unwrap().spent_today_bn[D][4],0.0);
    programs::spend_operating(&mut w,N::USA,D,4,available/4.0).unwrap();
    let p=w.nation(N::USA).program_budget.as_ref().unwrap();
    near(p.spent_today_bn[D][4],available/4.0);near(p.noncapital_spent_today_bn[D][4],available/4.0);near(p.available_bn[D][4],available*0.75);
    near(programs::force_support_share(w.nation(N::USA),w.nation(N::USA).mil_spend_gdp),support);
    let n=w.nation_mut(N::USA);n.program_budget.as_mut().unwrap().departments[D][4]=0;n.program_budget.as_mut().unwrap().departments[D][3]+=2000;
    near(programs::force_support_share(n,n.mil_spend_gdp),support);
}

#[test]
fn upkeep_allocates_existing_service_spending_and_reduces_legacy_refill() {
    let mut w=world();revision(&mut w,"tank");enroll(&mut w);let today=clock::absolute_day(&w);activate(&mut w,today);
    let n=w.nation_mut(N::USA);n.arsenal.held.clear();arsenal::deliver_design(n,"tank",1_000_000,0.0).unwrap();
    let spending=n.program_budget.as_ref().unwrap().spent_today_bn;let refill=programs::refill_multiplier(n,None);
    equipment::tick_day(&mut w);
    let n=w.nation(N::USA);assert_eq!(n.program_budget.as_ref().unwrap().spent_today_bn,spending);
    let s=n.equipment.as_ref().unwrap();assert!(s.maintenance_allocated_today_bn>0.0);assert!(s.maintenance_fraction<1.0);
    assert!(programs::refill_multiplier(n,None)<refill);
    let once=serde_json::to_string(&n.equipment).unwrap();equipment::tick_day(&mut w);
    assert_eq!(once,serde_json::to_string(&w.nation(N::USA).equipment).unwrap());
}

#[test]
fn custom_production_and_legacy_manufacturing_share_one_site_capacity() {
    let mut w=world();revision(&mut w,"tank");enroll(&mut w);let d=plant(&mut w);
    equipment::start_production(&mut w,N::USA,"tank",&d,1,0.001).unwrap();
    assert_eq!(manufacturing::used_slots(&w,N::USA,&d),1);
    assert!(manufacturing::start_line(&mut w,N::USA,&d,"arm_gen3").is_err());
    assert!(equipment::start_production(&mut w,N::USA,"tank",&d,1,0.001).is_err());
    let id=w.nation(N::USA).equipment.as_ref().unwrap().projects[0].id;
    equipment::cancel_project(&mut w,N::USA,id).unwrap();
    assert_eq!(manufacturing::used_slots(&w,N::USA,&d),0);
}


#[test]
fn extreme_activation_split_cannot_multiply_force_beyond_the_legacy_maximum() {
    let mut w=world();enroll(&mut w);
    w.nation_mut(N::USA).program_budget.as_mut().unwrap().departments[D]=[1,0,0,0,9999];
    let today=clock::absolute_day(&w);let share=w.nation(N::USA).mil_spend_gdp;
    let original=programs::force_support_share(w.nation(N::USA),share);
    activate(&mut w,today);
    near(programs::force_support_share(w.nation(N::USA),share),original);
    w.nation_mut(N::USA).program_budget.as_mut().unwrap().departments[D]=[10000,0,0,0,0];
    let bounded=programs::force_support_share(w.nation(N::USA),share);
    assert!(bounded<=share*10_000.0/6000.0);
    assert!(bounded.is_finite());
}

#[test]
fn zero_maintenance_funding_removes_custom_readiness_without_an_extra_charge() {
    let mut w=world();revision(&mut w,"tank");enroll(&mut w);
    let n=w.nation_mut(N::USA);arsenal::deliver_design(n,"tank",100,0.0).unwrap();
    n.program_budget.as_mut().unwrap().departments[D][2]=0;
    n.program_budget.as_mut().unwrap().departments[D][0]+=2000;
    let today=clock::absolute_day(&w);activate(&mut w,today+1);
    clock::advance_date(&mut w);programs::begin_day(&mut w);
    let ledger=w.nation(N::USA).program_budget.clone();
    equipment::tick_day(&mut w);
    let n=w.nation(N::USA);let state=n.equipment.as_ref().unwrap();
    assert_eq!(state.maintenance_allocated_today_bn,0.0);
    assert_eq!(state.maintenance_fraction,0.0);
    assert_eq!(n.program_budget,ledger);
    for h in &n.arsenal.held {
        let v=arsenal::combat_value(n,h);assert!(v.is_finite());
        if h.design_id.is_some(){assert_eq!(v,0.0);}else{assert!(v>0.0);}
    }
}

#[test]
fn custom_and_legacy_procurement_share_residual_prepaid_authority_once() {
    let mut w=world();w.rules.resource_gates=false;
    w.nation_mut(N::USA).arsenal.banked=3.0;enroll(&mut w);
    let available=programs::available_bn(&w,N::USA,D,3);
    let fresh=w.nation(N::USA).program_budget.as_ref().unwrap().accrued_today_bn[D][3];
    // The designer uses this same helper before legacy procurement runs.
    programs::spend(&mut w,N::USA,D,3,2.0).unwrap();
    w.nation_mut(N::USA).arsenal.orders.clear();
    for n in &mut w.nations {if n.id!=N::USA{n.alive=false;}}
    arsenal::tick(&mut w);
    let n=w.nation(N::USA);
    let ordered=n.arsenal.orders.iter().map(|o|o.units*arsenal::DECK[o.kit as usize].unit_cost).sum::<f64>();
    near(ordered,available-2.0);
    let p=n.program_budget.as_ref().unwrap();
    near(p.prepaid_used_today_bn[D][3],3.0);
    near(p.spent_today_bn[D][3],fresh);
    near(programs::available_bn(&w,N::USA,D,3),0.0);
}
