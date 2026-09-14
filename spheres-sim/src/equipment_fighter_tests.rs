//! Fighter lifecycle checks. Money, factory capacity and raw stocks are
//! disclosed fixture endowments; no fighter, company product or missile is
//! granted in the paid lifecycle test. Research points enter the same existing
//! component-research boundary used by Aerospace grants.
use super::*;
use crate::{aviation, companies as firms, programs, resources, Command, EquipmentOrder};
use crate::world::{NationId as N, BUDGET_DEFENSE as D};

const HOME: N = N::USA;
const FAMILY: &str = "air_missile_short_range";
const RESEARCH_PATH: [&str; 3] = ["air_propulsion_integration", "air_mission_systems", "air_fighter_integration"];

fn fixture() -> (WorldState, String) {
    let (mut w, district) = super::tests::fixture();
    w.conflicts.clear();
    let n = w.nation_mut(HOME);
    n.arsenal.held.clear();
    n.arsenal.orders.clear();
    n.arsenal.banked = 0.0;
    n.treasury_bn = Some(100.0);
    n.debt_bn = Some(0.0);
    n.debt_gdp = 0.0;
    for department in [2, 3, 4] {
        n.program_budget.as_mut().unwrap().available_bn[D][department] = 10.0;
    }
    (w, district)
}

fn component_research(w: &mut WorldState) {
    for key in RESEARCH_PATH {
        crate::apply_command(w, &Command::Equipment { nation: HOME,
            order: EquipmentOrder::Research { component: key.into() } }).unwrap();
        let day = clock::absolute_day(w);
        let year = w.year;
        let points = research(key).unwrap().points;
        assert!(research_step(w.nation_mut(HOME), points / 2.0, year, day));
        assert!(!w.nation(HOME).equipment.as_ref().unwrap().learned.contains(key));
        // Repeating the Aerospace boundary on the same date cannot award a
        // second grant. A second dated allocation is required to finish.
        assert!(research_step(w.nation_mut(HOME), points, year, day));
        assert!(!w.nation(HOME).equipment.as_ref().unwrap().learned.contains(key));
        clock::advance_date(w);
        let day = clock::absolute_day(w);
        let year = w.year;
        assert!(research_step(w.nation_mut(HOME), points / 2.0, year, day));
        assert!(w.nation(HOME).equipment.as_ref().unwrap().learned.contains(key));
        clock::advance_date(w);
    }
}

fn order(w: &mut WorldState, order: firms::CompanyOrder) {
    crate::apply_command(w, &Command::Company { nation: HOME, order }).unwrap();
}

fn work_day(w: &mut WorldState) {
    if w.nation(HOME).program_budget.as_ref().unwrap().fiscal_year != w.year {
        let departments = w.nation(HOME).program_budget.as_ref().unwrap().departments;
        let allocations = w.nation(HOME).budget_for(w.year).allocations;
        crate::apply_command(w, &Command::SetProgramBudget {
            nation: HOME, fiscal_year: w.year, allocations, departments,
        }).unwrap();
    }
    programs::begin_day(w);
    tick_day(w);
    firms::tick_day(w);
    settle_support(w);
    tick_air_support(w);
    let plan = w.nation_mut(HOME).program_budget.as_mut().unwrap();
    plan.revenue_today_bn = 0.0;
    plan.interest_today_bn = 0.0;
    plan.fiscal_staged = true;
    programs::finish_day(w);
    firms::settle_receivables(w);
    clock::advance_date(w);
}

fn until(w: &mut WorldState, limit: usize, ready: impl Fn(&WorldState) -> bool) {
    for _ in 0..limit {
        if ready(w) { return; }
        work_day(w);
    }
    assert!(ready(w), "Dated fighter work failed to finish: {:?}", w.companies);
}

fn national_missiles(w: &WorldState) -> f64 {
    w.nation(HOME).equipment.as_ref().and_then(|s| s.ammunition.as_ref())
        .and_then(|a| a.stocks.get(FAMILY)).copied().unwrap_or(0.0)
}

fn near(a: f64, b: f64) { assert!((a-b).abs() < 1e-10 * a.abs().max(b.abs()).max(1.0), "{a} != {b}"); }

#[test]
fn s16_fighter_research_is_prerequisite_gated_dated_and_grants_no_property() {
    let (mut w, _) = fixture();
    let spec = default_spec("air_fighter");
    let original = crate::save(&w);
    let old_technology = crate::arsenal::combat_technology(w.nation(HOME));
    assert!(research_refusal(&w, HOME, "air_fighter_integration").unwrap().contains("Complete"));
    assert!(!design_preview(&w, HOME, &spec).valid);
    assert!(start_development(&mut w, HOME, "Locked fighter", spec.clone(), 0.01).is_err());
    assert_eq!(crate::save(&w), original);
    component_research(&mut w);
    assert_eq!(crate::arsenal::combat_technology(w.nation(HOME)), old_technology);
    assert!(FIGHTER_COMPONENTS.iter().all(|c| c.technology.is_none()));
    assert!(design_preview(&w, HOME, &spec).valid);
    assert_eq!(research_branch("air_fighter_integration"), "weapons");
    assert!(w.nation(HOME).arsenal.held.is_empty());
    assert!(w.nation(HOME).equipment.as_ref().unwrap().revisions.is_empty());
    assert!(w.nation(HOME).equipment.as_ref().unwrap().ammunition.is_none());
    let bytes = crate::save(&w);
    assert_eq!(crate::save(&crate::load(&bytes).unwrap()), bytes);
}

#[test]
fn s16_fighter_profile_has_only_interception_and_rejects_bomber_hardware() {
    let (mut w, _) = fixture();
    component_research(&mut w);
    let spec = default_spec("air_fighter");
    let p = design_preview(&w, HOME, &spec).profile.unwrap();
    assert_eq!(p.rules_version, 5);
    assert_eq!(p.land_factor, 1.0);
    assert!(p.ground_roles.is_none());
    let air = p.aviation.as_ref().unwrap();
    assert!(air.valid());
    assert_eq!(air.strike_factor, 0.0);
    assert_eq!(air.intercept_factor, 1.0);
    assert_eq!(air.store_family, FAMILY);
    assert_eq!(ammunition_family(&spec), Some(FAMILY));
    assert_eq!(crate::airbases::range_km(&spec), 900.0);
    for (slot, id) in [("air_engine", "air_engine_interceptor_efficient"),
        ("air_radar", "air_radar_interceptor_tracking"), ("air_avionics", "air_avionics_interceptor_digital"),
        ("air_countermeasures", "air_countermeasures_ecm"), ("air_fuel", "air_fuel_extended")] {
        let mut improved = spec.clone();
        improved.components.insert(slot.into(), id.into());
        let q = design_preview(&w, HOME, &improved);
        assert!(q.valid, "{slot}: {:?}", q.blockers);
        let improved_profile = q.profile.unwrap();
        assert!(improved_profile.aviation.unwrap().intercept_factor > air.intercept_factor);
        assert!(improved_profile.maintenance_bn_day > p.maintenance_bn_day);
        if slot == "air_fuel" { assert_eq!(crate::airbases::range_km(&improved), 1215.0); }
    }
    for slot in AVIATION_SLOTS {
        let mut bad = spec.clone();
        bad.components.remove(slot);
        assert!(!design_preview(&w, HOME, &bad).valid, "{slot}");
    }
    for (slot, id) in [("air_payload", "air_payload_unguided"), ("air_payload", "air_payload_guided"),
        ("air_radar", "air_radar_mapping"), ("air_engine", "air_engine_economical")] {
        let mut bad = spec.clone();
        bad.components.insert(slot.into(), id.into());
        assert!(!design_preview(&w, HOME, &bad).valid, "{id}");
    }
    for platform in ["air_light_attack", "air_tactical_strike"] {
        for c in FIGHTER_COMPONENTS {
            let mut bad = default_spec(platform);
            bad.components.insert(c.slot.into(), c.id.into());
            assert!(!design_preview(&w, HOME, &bad).valid, "{platform}/{}", c.id);
        }
    }
}

#[test]
fn s16_fighter_v8_air_profiles_remain_sparse_and_frozen_claims_are_checked() {
    let (mut w, _) = fixture();
    let id = start_development(&mut w, HOME, "Prior strike", default_spec("air_light_attack"), 0.01).unwrap();
    w.nation_mut(HOME).equipment.as_mut().unwrap().version = 8;
    let bytes = crate::save(&w);
    assert!(!bytes.contains("intercept_factor"));
    assert_eq!(crate::save(&crate::load(&bytes).unwrap()), bytes);
    w.nation_mut(HOME).equipment.as_mut().unwrap().revisions.get_mut(&id).unwrap()
        .profile.aviation.as_mut().unwrap().intercept_factor = 1.0;
    assert!(crate::load(&crate::save(&w)).is_err());

    let (mut w, _) = fixture();
    component_research(&mut w);
    let id = start_development(&mut w, HOME, "Frozen fighter", default_spec("air_fighter"), 0.01).unwrap();
    let baseline = w.clone();
    for flaw in 0..3 {
        let mut bad = baseline.clone();
        let s = bad.nation_mut(HOME).equipment.as_mut().unwrap();
        match flaw {
            0 => s.version = 8,
            1 => s.revisions.get_mut(&id).unwrap().profile.aviation.as_mut().unwrap().intercept_factor += 0.01,
            _ => s.revisions.get_mut(&id).unwrap().profile.aviation.as_mut().unwrap().store_family = "air_bomb_unguided".into(),
        }
        assert!(crate::load(&crate::save(&bad)).is_err(), "{flaw}");
    }
    let bytes = crate::save(&baseline);
    assert_eq!(crate::save(&crate::load(&bytes).unwrap()), bytes);
}

#[test]
fn s16_company_fighter_and_missiles_complete_paid_delivery_upkeep_and_assignment() {
    let (mut w, district) = fixture();
    component_research(&mut w);
    set_maintenance_plan(&mut w, HOME, 0.001).unwrap();
    let q = firms::establishment_quote(&w, HOME, "Authored fighter works", &district, 1.0);
    assert!(q.valid, "{:?}", q.reason);
    order(&mut w, firms::CompanyOrder::Establish { name: "Authored fighter works".into(),
        district, capitalization_bn: 1.0, quote: q.token });
    let company = w.companies.firms[0].id;
    work_day(&mut w);
    assert!(!firms::ammo_supply_quote(&w, HOME, company, FAMILY, 12).valid,
        "Research alone cannot license compatible finished stores");
    let spec = default_spec("air_fighter");
    let q = firms::development_quote(&w, HOME, company, "Paid defensive fighter", &spec, 0.01, 2);
    assert!(q.valid, "{:?}", q.reason);
    order(&mut w, firms::CompanyOrder::Develop { company, name: "Paid defensive fighter".into(),
        spec, daily_budget_bn: 0.01, stock_target: 2, quote: q.token });
    let revision = w.companies.firms[0].products[0].revision_id.clone();
    let product = w.companies.firms[0].products[0].id;
    assert_eq!(aviation::unassigned_units(w.nation(HOME), &revision), 0);
    assert!(!firms::purchase_quote(&w, HOME, company, product, 2).valid);
    assert_eq!(national_missiles(&w), 0.0);
    until(&mut w, 1200, |w| w.companies.firms[0].products[0].stock == 2);
    let p = &w.companies.firms[0].products[0];
    let frozen = profile(w.nation(HOME), &revision).unwrap().clone();
    assert!(p.certified_day.is_some());
    near(p.development_spent_bn, frozen.development_cost_bn);
    near(p.tooling_spent_bn, frozen.tooling_cost_bn);
    assert_eq!(p.produced_units, 2);
    assert_eq!(aviation::unassigned_units(w.nation(HOME), &revision), 0);
    let inputs: [f64; 12] = std::array::from_fn(|i| resources::stockpile(&w, HOME, resources::ALL[i]));
    let q = firms::purchase_quote(&w, HOME, company, product, 2);
    assert!(q.valid && q.cost_bn > 0.0, "{:?}", q.reason);
    order(&mut w, firms::CompanyOrder::Purchase { company, product, quantity: 2, quote: q.token });
    order(&mut w, firms::CompanyOrder::Inventory { company, product, stock_target: 0 });
    assert_eq!(aviation::unassigned_units(w.nation(HOME), &revision), 0);
    let save = crate::save(&w);
    w = crate::load(&save).unwrap();
    until(&mut w, 12, |w| aviation::unassigned_units(w.nation(HOME), &revision) == 2);
    let delivered = &w.companies.deliveries[0];
    assert!(delivered.total_price_bn > 0.0 && delivered.settled_day.is_some());
    assert!(delivered.delivered_day.unwrap() >= delivered.settled_day.unwrap() + firms::DELIVERY_DAYS as i32);
    assert_eq!(national_missiles(&w), 0.0, "The fighter includes no ammunition");
    near(fleet_maintenance_requirement(w.nation(HOME)), 2.0 * frozen.maintenance_bn_day);

    let q = firms::ammo_supply_quote(&w, HOME, company, FAMILY, 12);
    assert!(q.valid, "{:?}", q.reason);
    order(&mut w, firms::CompanyOrder::AmmoSupply { company, family: FAMILY.into(), stock_target: 12, quote: q.token });
    until(&mut w, 12, |w| w.companies.firms[0].ammunition_products[0].stock == 12);
    let ammo_product = w.companies.firms[0].ammunition_products[0].id;
    let ammo_def = ammunition_def(FAMILY).unwrap();
    for i in 0..12 { near(inputs[i] - resources::stockpile(&w, HOME, resources::ALL[i]), 12.0 * ammo_def.recipe[i]); }
    assert_eq!(national_missiles(&w), 0.0, "Company shelf is not national property");
    // The same paid shelf also supports the ordinary automatic support policy.
    // Target six days is exactly twelve missiles after the planning ceil.
    let mut automatic = w.clone();
    order(&mut automatic, firms::CompanyOrder::AmmoInventory { company, product: ammo_product, stock_target: 0 });
    set_air_support(&mut automatic, HOME, 10.0, 6, true).unwrap();
    until(&mut automatic, 4, |w| w.companies.ammunition_deliveries.len() == 1);
    let support = automatic.nation(HOME).equipment.as_ref().unwrap().air_support.as_ref().unwrap().receipt.as_ref().unwrap();
    assert_eq!(support.purchases.len(), 1);
    assert_eq!(support.purchases[0].family, FAMILY);
    assert_eq!(support.purchases[0].quantity, 12);
    assert!(support.stores_paid_bn > 0.0);
    let bytes = crate::save(&automatic);
    automatic = crate::load(&bytes).unwrap();
    until(&mut automatic, 12, |w| national_missiles(w) == 12.0);
    assert_eq!(automatic.companies.ammunition_deliveries.len(), 1,
        "Pending missile arrivals count toward support's target");
    let bytes = crate::save(&automatic);
    assert_eq!(crate::save(&crate::load(&bytes).unwrap()), bytes);

    let q = firms::ammo_purchase_quote(&w, HOME, company, ammo_product, 12);
    assert!(q.valid && q.cost_bn > 0.0, "{:?}", q.reason);
    order(&mut w, firms::CompanyOrder::AmmoPurchase { company, product: ammo_product, quantity: 12, quote: q.token });
    order(&mut w, firms::CompanyOrder::AmmoInventory { company, product: ammo_product, stock_target: 0 });
    assert_eq!(national_missiles(&w), 0.0);
    until(&mut w, 12, |w| national_missiles(w) == 12.0);
    assert!(w.companies.ammunition_deliveries[0].total_price_bn > 0.0);
    assert!(w.companies.ammunition_deliveries[0].delivered_day.is_some());
    work_day(&mut w);
    let receipt = w.nation(HOME).equipment.as_ref().unwrap().maintenance_plan.as_ref().unwrap().receipt.as_ref().unwrap();
    near(receipt.custom_required_bn, 2.0 * frozen.maintenance_bn_day);
    near(receipt.custom_paid_bn, receipt.custom_required_bn);
    let expected = crate::save(&w);
    firms::settle_receivables(&mut w);
    assert_eq!(crate::save(&w), expected, "Repeated settlement must not pay twice");

    // Even with delivered compatible missiles, an unassigned fighter cannot
    // consume them in the legacy implicit bombing path.
    assert!(w.nation(HOME).aviation.is_none());
    let deployment = AmmoDeployment { ground: false, deployed_share: 1.0,
        aircraft_share: 1.0, intensity: 1.0, air_exposure: 0.0 };
    let planned = plan_ammunition(&w, HOME, &[deployment]);
    assert_eq!(planned.families.iter().find(|f| f.family == FAMILY).unwrap().required, 0.0);
    let effect = aviation_ammunition_effects(&w, HOME, &deployment,
        crate::operations::capabilities(w.nation(HOME)), &planned);
    assert_eq!(effect.attack_coefficient, 0.0);
    assert_eq!(effect.fire_fraction, 0.0);
    let day = clock::absolute_day(&w);
    aviation::apply(&mut w, HOME, &aviation::SquadronCommand::Create {
        name: "Delivered fighter squadron".into(), revision: revision.clone(), quantity: 2,
    }).unwrap();
    assert_eq!(aviation::assigned_units(w.nation(HOME), &revision), 2);
    assert_eq!(aviation::unassigned_units(w.nation(HOME), &revision), 0);
    assert_eq!(aviation::ready_units(w.nation(HOME), &revision, day), 0,
        "Assigned fighters still require a completed accessible base");
    assert_eq!(national_missiles(&w), 12.0);
    firms::validate_state(&w).unwrap();
    validate_state(w.nation(HOME)).unwrap();
    let save = crate::save(&w);
    assert_eq!(crate::save(&crate::load(&save).unwrap()), save);
}
