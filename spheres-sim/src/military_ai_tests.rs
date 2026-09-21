use super::*;
use crate::{init, production, resources, world::GameRules};
const HOME: NationId = NationId::France;
const SMALL: NationId = NationId::Malta;

// Explicit test endowments: one completed plant, cash, research-independent raw
// inputs and opening appropriation. Models, company cash settlement, stock,
// procurement and delivery below are earned through their ordinary owners.
fn fixture() -> (WorldState, String, u32) {
    let mut w = init::world_1990(GameRules {
        daily_simulation: true,
        military_operations: true,
        production_system: true,
        manufacturing_system: true,
        resource_market: true,
        resource_gates: true,
        economic_competition: true,
        ai_aggression: 0.0,
        crisis_intensity: 0.0,
        ..Default::default()
    });
    w.player = Some(NationId::USA);
    w.conflicts.clear();
    w.sanctions.clear();
    crate::operational_warfare::enable(&mut w).unwrap();
    for n in [HOME, SMALL] {
        w.nation_mut(n).political_capital = 1000.0;
        programs::set_construction_budget(&mut w, n, 0.002).unwrap();
        let nat = w.nation_mut(n);
        nat.treasury_bn = Some(100.0);
        nat.debt_bn = Some(0.0);
        nat.debt_gdp = 0.0;
        nat.arsenal.held.clear();
        nat.arsenal.orders.clear();
        nat.arsenal.banked = 0.0;
        let p = nat.program_budget.as_mut().unwrap();
        for d in [2, 3, 4] {
            p.available_bn[DEF][d] = 10.0;
            p.prepaid_bn[DEF][d] = 0.0;
        }
        let q = co::import_enrollment_quote(&w, n);
        assert!(q.valid, "{:?}", q.reason);
        act(
            &mut w,
            n,
            co::CompanyOrder::EnableImports { quote: q.token },
        )
        .unwrap();
    }
    let d = w
        .districts
        .iter()
        .find(|(_, owner)| **owner == HOME)
        .unwrap()
        .0
        .clone();
    production::complete_capability(&mut w, &d, production::ProjectKind::ArmsPlant);
    for raw in resources::ALL {
        resources::set_stockpile_for_test(&mut w, HOME, raw, 1_000_000.0);
    }
    let q = co::establishment_quote(&w, HOME, "Authored staff works", &d, 1.0);
    assert!(q.valid, "{:?}", q.reason);
    act(
        &mut w,
        HOME,
        co::CompanyOrder::Establish {
            name: "Authored staff works".into(),
            district: d.clone(),
            capitalization_bn: 1.0,
            quote: q.token,
        },
    )
    .unwrap();
    let cid = w.companies.firms.last().unwrap().id;
    w.supplier_catalogue.plans.insert(
        HOME,
        crate::supplier_catalogue::Plan {
            district: Some(d.clone()),
            company: Some(cid),
            ..Default::default()
        },
    );
    enable(&mut w).unwrap();
    work_day(&mut w);
    (w, d, cid)
}
fn work_day(w: &mut WorldState) {
    for n in [HOME, SMALL] {
        if w.nation(n).program_budget.as_ref().unwrap().fiscal_year != w.year {
            programs::set_construction_budget(w, n, 0.002).unwrap();
        }
    }
    programs::begin_day(w);
    eq::tick_day(w);
    co::tick_day(w);
    eq::settle_support(w);
    eq::tick_air_support(w);
    for n in [HOME, SMALL] {
        programs::stage_fiscal(w.nation_mut(n), 0.0, 0.0);
    }
    programs::finish_day(w);
    co::settle_receivables(w);
    clock::advance_date(w);
}
fn stocked() -> (WorldState, u32) {
    let (mut w, _, cid) = fixture();
    programs::begin_day(&mut w);
    let mut limit = 1.0;
    let result = develop(&mut w, HOME, &mut limit).unwrap();
    assert!(result.contains("Ordered one"), "{result}");
    assert_eq!(co::company(&w, HOME, cid).unwrap().products[0].stock, 0);
    for _ in 0..1000 {
        if co::company(&w, HOME, cid).unwrap().products[0].stock == 4 {
            break;
        }
        work_day(&mut w);
    }
    let c = co::company(&w, HOME, cid).unwrap();
    let p = &c.products[0];
    assert_eq!(p.stock, 4, "{} {}", p.status, p.reason);
    assert!(p.certified_day.is_some() && p.development_spent_bn > 0.0 && p.tooling_spent_bn > 0.0);
    assert!(c.fabrication_expense_bn > 0.0 && c.materials_expense_bn > 0.0);
    assert!(w.nation(HOME).arsenal.held.is_empty());
    programs::begin_day(&mut w);
    (w, cid)
}

#[test]
fn sparse_legacy_setting_and_read_model_are_pure_and_versioned() {
    let mut w = init::world_1990(GameRules::default());
    w.player = Some(HOME);
    let original = crate::save(&w);
    assert!(!original.contains("military_ai"));
    let old = crate::load(&original).unwrap();
    assert!(old.military_ai.is_empty());
    tick(&mut w);
    let _ = view(&w);
    assert_eq!(crate::save(&w), original);
    assert!(configure(&mut w, HOME, true).is_err());
    assert_eq!(crate::save(&w), original);
    let (mut w, _, _) = fixture();
    let before = crate::save(&w);
    let _ = view(&w);
    assert_eq!(crate::save(&w), before);
    assert!(configure(&mut w, HOME, false).is_err());
    let player = w.player.unwrap();
    configure(&mut w, player, false).unwrap();
    let paused = crate::save(&w);
    tick(&mut w);
    assert_eq!(crate::save(&w), paused);
    let loaded = crate::load(&paused).unwrap();
    assert_eq!(crate::save(&loaded), paused);
    enable(&mut w).unwrap();
    let mut corrupt: serde_json::Value = serde_json::from_str(&crate::save(&w)).unwrap();
    corrupt["equipment_version"] = serde_json::json!(6);
    assert!(crate::load(&corrupt.to_string()).is_err());
    w.military_ai.plans.entry(HOME).or_default().last_review_day =
        Some(clock::absolute_day(&w) + 1);
    assert!(crate::load(&crate::save(&w)).is_err());
}

#[test]
fn domestic_staff_develops_pays_buys_delivers_and_counts_pending_aircraft() {
    let (mut w, cid) = stocked();
    support(&mut w, HOME).unwrap();
    let cash = w.nation(HOME).treasury_bn.unwrap() - w.nation(HOME).debt_bn.unwrap();
    let mut allowance = 1.0;
    let decision = procure(&mut w, HOME, &mut allowance).unwrap();
    assert!(decision.starts_with("Bought 4"), "{decision}");
    assert_eq!(co::company(&w, HOME, cid).unwrap().products[0].stock, 0);
    assert_eq!(committed_units(&w, HOME, "air_light_attack"), 4);
    assert!(w.nation(HOME).arsenal.held.is_empty());
    let before = w.companies.deliveries.len();
    procure(&mut w, HOME, &mut allowance).unwrap();
    assert_eq!(w.companies.deliveries.len(), before);
    for _ in 0..10 {
        work_day(&mut w);
    }
    assert_eq!(committed_units(&w, HOME, "air_light_attack"), 4);
    assert!(w.nation(HOME).treasury_bn.unwrap() - w.nation(HOME).debt_bn.unwrap() < cash);
    assert!(w.companies.deliveries[0].delivered_day.is_some());
    let decision = basing(&mut w, HOME).unwrap();
    assert!(decision.contains("Formed"), "{decision}");
    assert_eq!(
        w.nation(HOME).aviation.as_ref().unwrap().squadrons[0].assigned,
        4
    );
    assert!(w.nation(HOME).aviation.as_ref().unwrap().squadrons[0]
        .base
        .is_none());
    // Airfield construction has its own appropriation. Exhausting equipment
    // procurement must not veto an affordable, slowly funded foundation.
    programs::begin_day(&mut w);
    w.nation_mut(HOME).treasury_bn = Some(1.0);
    w.nation_mut(HOME)
        .program_budget
        .as_mut()
        .unwrap()
        .available_bn[DEF][3] = 0.0;
    w.nation_mut(HOME)
        .program_budget
        .as_mut()
        .unwrap()
        .prepaid_bn[DEF][3] = 0.0;
    assert_eq!(review_limit(&w, HOME), 0.0);
    let result = basing(&mut w, HOME).unwrap();
    assert!(result.contains("Ordered one funded airfield"), "{result}");
    let base = &w.airbases.as_ref().unwrap().bases[0];
    assert_eq!(base.capacity_level, 0);
    assert_eq!(base.project.as_ref().unwrap().paid_bn, 0.0);
    crate::load(&crate::save(&w)).unwrap();
}

#[test]
fn small_country_imports_one_aircraft_without_research_or_factory_grants() {
    let (mut w, cid) = stocked();
    support(&mut w, SMALL).unwrap();
    // Current-year buyer appropriation is a disclosed isolated purchase fixture.
    w.nation_mut(SMALL)
        .program_budget
        .as_mut()
        .unwrap()
        .available_bn[DEF][3] = 1.0;
    let old_research = w.nation(SMALL).equipment.as_ref().unwrap().learned.clone();
    let old_firms = w.companies.firms.len();
    let mut allowance = 1.0;
    assert_eq!(target(&w, SMALL), 1);
    let decision = procure(&mut w, SMALL, &mut allowance).unwrap();
    assert!(
        decision.starts_with("Bought 1"),
        "{decision}; support={}",
        support_authority(&w, SMALL)
    );
    assert_eq!(co::company(&w, HOME, cid).unwrap().products[0].stock, 3);
    assert!(w.nation(SMALL).arsenal.held.is_empty());
    let before = w.companies.imports.contracts.len();
    procure(&mut w, SMALL, &mut allowance).unwrap();
    assert_eq!(w.companies.imports.contracts.len(), before);
    for _ in 0..50 {
        work_day(&mut w);
    }
    assert!(
        w.companies.imports.contracts[0].delivered_day.is_some(),
        "{:?}",
        w.companies.imports.contracts[0]
    );
    assert_eq!(committed_units(&w, SMALL, "air_light_attack"), 1);
    assert_eq!(
        w.nation(SMALL).equipment.as_ref().unwrap().learned,
        old_research
    );
    assert_eq!(w.companies.firms.len(), old_firms);
    crate::load(&crate::save(&w)).unwrap();
}

#[test]
fn cash_upkeep_access_and_player_control_block_staff_without_inventory_grants() {
    let (mut w, _) = stocked();
    w.nation_mut(HOME).treasury_bn = Some(0.0);
    assert_eq!(review_limit(&w, HOME), 0.0);
    let mut allowance = 0.0;
    let before = w.companies.clone();
    assert!(procure(&mut w, HOME, &mut allowance)
        .unwrap()
        .contains("cap"));
    assert_eq!(w.companies, before);
    w.nation_mut(HOME)
        .program_budget
        .as_mut()
        .unwrap()
        .departments[DEF][2] = 0;
    allowance = 1.0;
    assert!(procure(&mut w, HOME, &mut allowance)
        .unwrap()
        .contains("Maintenance"));
    assert_eq!(w.companies, before);
    let player = w.player.unwrap();
    let old = serde_json::to_string(w.nation(player)).unwrap();
    tick(&mut w);
    assert_eq!(serde_json::to_string(w.nation(player)).unwrap(), old);
    assert!(!w.military_ai.plans.contains_key(&player));
}

#[test]
fn review_cadence_and_save_resume_are_deterministic() {
    let (mut a, _, _) = fixture();
    let mut b = crate::load(&crate::save(&a)).unwrap();
    for _ in 0..62 {
        programs::begin_day(&mut a);
        programs::begin_day(&mut b);
        tick(&mut a);
        tick(&mut b);
        let before = crate::save(&a);
        tick(&mut a);
        assert_eq!(crate::save(&a), before, "same day repeated");
        assert_eq!(crate::save(&a), crate::save(&b));
        clock::advance_date(&mut a);
        clock::advance_date(&mut b);
    }
    assert!(a.military_ai.plans.len() > 50);
    assert!(a.military_ai.plans.values().all(|p| p.reviews <= 3));
    validate(&a).unwrap();
}

#[test]
fn managed_supplier_identity_is_required() {
    let (mut w, _, cid) = fixture();
    w.supplier_catalogue.plans.remove(&HOME);
    let before = w.companies.clone();
    let mut allowance = 1.0;
    assert!(develop(&mut w, HOME, &mut allowance)
        .unwrap()
        .contains("no staff-managed"));
    assert_eq!(w.companies, before);
    assert_eq!(co::company(&w, HOME, cid).unwrap().products.len(), 0);
}

#[test]
fn supplier_ammunition_and_fighter_research_are_paid_and_not_switched_daily() {
    let (mut w, cid) = stocked();
    support(&mut w, HOME).unwrap();
    work_day(&mut w);
    work_day(&mut w);
    programs::begin_day(&mut w);
    let mut allowance = 1.0;
    let first = develop(&mut w, HOME, &mut allowance).unwrap();
    assert!(first.contains("finite compatible"), "{first}");
    for _ in 0..60 {
        let c = co::company(&w, HOME, cid).unwrap();
        if c.ammunition_products[0].stock >= c.ammunition_products[0].stock_target {
            break;
        }
        work_day(&mut w);
    }
    programs::begin_day(&mut w);
    let c = co::company(&w, HOME, cid).unwrap();
    assert!(
        c.ammunition_products[0].stock > 0 && c.ammunition_products[0].fabrication_expense_bn > 0.0
    );
    let stock = c.products[0].stock;
    let result = develop(&mut w, HOME, &mut allowance).unwrap();
    assert!(result.starts_with("Selected"), "{result}");
    let eq = w.nation(HOME).equipment.as_ref().unwrap();
    assert_eq!(
        eq.active_research.as_deref(),
        Some("air_propulsion_integration")
    );
    assert!(!eq.learned.contains("air_propulsion_integration"));
    let before = crate::save(&w);
    let result = develop(&mut w, HOME, &mut allowance).unwrap();
    assert!(result.contains("Waiting for the selected"), "{result}");
    assert_eq!(crate::save(&w), before);
    assert_eq!(co::company(&w, HOME, cid).unwrap().products[0].stock, stock);
}

#[test]
fn ground_staff_uses_finite_ammunition_and_counts_inbound_rounds() {
    let (mut w, _, cid) = fixture();
    programs::begin_day(&mut w);
    let spec = eq::default_spec("ground_apc");
    let family = eq::ammunition_family(&spec).unwrap();
    let q = co::development_quote(&w, HOME, cid, "Paid ground fixture", &spec, 0.01, 12);
    assert!(q.valid, "{:?}", q.reason);
    act(
        &mut w,
        HOME,
        co::CompanyOrder::Develop {
            company: cid,
            name: "Paid ground fixture".into(),
            spec,
            daily_budget_bn: 0.01,
            stock_target: 12,
            quote: q.token,
        },
    )
    .unwrap();
    for _ in 0..1000 {
        if co::company(&w, HOME, cid).unwrap().products[0].stock == 12 {
            break;
        }
        work_day(&mut w);
    }
    assert_eq!(co::company(&w, HOME, cid).unwrap().products[0].stock, 12);
    programs::begin_day(&mut w);
    support(&mut w, HOME).unwrap();
    let mut allowance = 1.0;
    let bought = procure(&mut w, HOME, &mut allowance).unwrap();
    assert!(bought.starts_with("Bought 12"), "{bought}");
    let pid = co::company(&w, HOME, cid).unwrap().products[0].id;
    act(
        &mut w,
        HOME,
        co::CompanyOrder::Inventory {
            company: cid,
            product: pid,
            stock_target: 0,
        },
    )
    .unwrap();
    for _ in 0..10 {
        work_day(&mut w);
    }
    programs::begin_day(&mut w);
    support(&mut w, HOME).unwrap();
    assert!(w
        .nation(HOME)
        .equipment
        .as_ref()
        .unwrap()
        .ammunition
        .as_ref()
        .unwrap()
        .active_from_day
        .is_some());
    let count = (eq::ammo_def(family).unwrap().rounds_per_vehicle_month * 12.0).ceil() as u32;
    let q = co::ammo_supply_quote(&w, HOME, cid, family, count);
    assert!(q.valid, "{:?}", q.reason);
    act(
        &mut w,
        HOME,
        co::CompanyOrder::AmmoSupply {
            company: cid,
            family: family.into(),
            stock_target: count,
            quote: q.token,
        },
    )
    .unwrap();
    let mut scheduled = std::collections::BTreeSet::new();
    for offset in [0, REVIEW_DAYS] {
        let mut branch = w.clone();
        let (year, month, day) = clock::date_from_day(clock::absolute_day(&branch) + offset);
        branch.year = year;
        branch.month = month;
        branch.day = day;
        let cash = co::company(&branch, HOME, cid).unwrap().cash_bn;
        let resources = serde_json::to_string(&branch.resources).unwrap();
        rotate_supplier_work(&mut branch, HOME).unwrap();
        let c = co::company(&branch, HOME, cid).unwrap();
        scheduled.insert(co::scheduled_product(c).unwrap());
        assert_eq!(c.cash_bn, cash);
        assert_eq!(serde_json::to_string(&branch.resources).unwrap(), resources);
        assert_eq!(c.products[0].stock, 0);
        assert_eq!(c.ammunition_products[0].stock, 0);
    }
    assert_eq!(
        scheduled.len(),
        2,
        "Both equipment and ammunition must receive actual shared-slot opportunities"
    );
    for _ in 0..1000 {
        if co::company(&w, HOME, cid).unwrap().ammunition_products[0].stock == count {
            break;
        }
        work_day(&mut w);
    }
    assert_eq!(
        co::company(&w, HOME, cid).unwrap().ammunition_products[0].stock,
        count,
        "{:?}",
        co::company(&w, HOME, cid).unwrap().ammunition_products[0]
    );
    programs::begin_day(&mut w);
    let result = ground_stores(&mut w, HOME, &mut allowance).unwrap();
    assert!(result.starts_with("Purchased"), "{result}");
    let ledger = w.companies.ammunition_deliveries.len();
    let again = ground_stores(&mut w, HOME, &mut allowance).unwrap();
    assert_eq!(w.companies.ammunition_deliveries.len(), ledger, "{again}");
    assert_eq!(eq::ammo_reserve_status(&w, HOME, family).stock, 0.0);
    for _ in 0..10 {
        work_day(&mut w);
    }
    assert_eq!(
        eq::ammo_reserve_status(&w, HOME, family).stock,
        count as f64
    );
    crate::load(&crate::save(&w)).unwrap();
}
