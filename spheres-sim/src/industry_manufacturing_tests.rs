//! Invariants for the rebuilt industry's links to the existing procurement
//! ledger. Fixtures commission MODEL sites; they assert no historical counts.
use crate::{
    arsenal, clock, industry_operations, manufacturing, production,
    production::ProjectKind as K,
    programs, resources,
    world::{GameRules, NationId, WorldState},
};

const USA: NationId = NationId::USA;

fn fixture() -> (WorldState, String) {
    let mut w = crate::init::world_1990(GameRules {
        daily_simulation: true,
        production_system: true,
        manufacturing_system: true,
        resource_market: true,
        resource_gates: true,
        industry_rebuild: true,
        ..Default::default()
    });
    w.player = Some(USA);
    programs::set_construction_budget(&mut w, USA, 0.0).unwrap();
    let district = w
        .districts
        .iter()
        .find(|(d, n)| **n == USA && crate::logistics::has_terminal(d))
        .unwrap()
        .0
        .clone();
    for c in resources::ALL {
        if c != resources::Commodity::Oil {
            resources::set_stockpile_for_test(&mut w, USA, c, 1_000_000.0);
        }
    }
    (w, district)
}

fn next_day(w: &mut WorldState) {
    programs::stage_fiscal(w.nation_mut(USA), 0.0, 0.0);
    programs::finish_day(w);
    clock::advance_date(w);
    programs::begin_day(w);
    industry_operations::begin_day(w);
}

fn near(a: f64, b: f64) {
    assert!((a - b).abs() < 1e-8, "{a} != {b}");
}

#[test]
fn a_shipyard_and_a_military_factory_supply_separate_production_slots() {
    let (mut w, d) = fixture();
    production::complete_capability(&mut w, &d, K::Shipyard);
    assert!(manufacturing::start_line_error(&w, USA, &d, "arm_gen3").is_some());
    manufacturing::start_line(&mut w, USA, &d, "nav_escort").unwrap();
    assert!(manufacturing::start_line_error(&w, USA, &d, "nav_patrol").is_some());
    production::complete_capability(&mut w, &d, K::ArmsPlant);
    manufacturing::start_line(&mut w, USA, &d, "arm_gen3").unwrap();
    assert_eq!(
        manufacturing::used_slots_for_kit(&w, USA, &d, "arm_gen3"),
        1
    );
    assert_eq!(
        manufacturing::used_slots_for_kit(&w, USA, &d, "nav_escort"),
        1
    );
    assert_eq!(w.manufacturing.lines.len(), 2);
    assert_eq!(
        manufacturing::used_slots(&w, USA, &d),
        1,
        "naval berths cannot occupy military-factory slots used by custom equipment or ammunition"
    );
    let inland = w
        .districts
        .iter()
        .find(|(d, n)| **n == USA && !crate::logistics::has_terminal(d))
        .unwrap()
        .0
        .clone();
    let before = crate::save(&w);
    assert!(production::start_project(&mut w, USA, &inland, K::Shipyard)
        .unwrap_err()
        .contains("coastal"));
    assert_eq!(crate::save(&w), before);
}

#[test]
fn advanced_naval_orders_consume_components_and_power_outages_stop_real_output() {
    let (mut w, d) = fixture();
    for kind in [K::Shipyard, K::Generation, K::PowerGrid] {
        production::complete_capability(&mut w, &d, kind);
    }
    let kit = "la_ssn";
    let technology = crate::tech::index_of("aero_quiet_submarine").unwrap();
    if !w.nation(USA).tech.knows_index(technology) {
        // Explicit synthetic knowledge isolates component supply from the
        // separate discovery system; it does not alter starting data.
        w.nation_mut(USA).tech.grant_1990(&[technology]);
    }
    let id = manufacturing::start_line(&mut w, USA, &d, kit).unwrap();
    programs::begin_day(&mut w);
    industry_operations::begin_day(&mut w);
    assert!(industry_operations::naval_capacity_bonus(&w, USA) > 0.0);
    assert!(manufacturing::tick_allocations(&w, USA)[0].budget_bn > 0.0);
    let held = serde_json::to_string(&w.nation(USA).arsenal.held).unwrap();
    let raw = w.resources.clone();
    manufacturing::settle_nation(&mut w, USA);
    near(w.manufacturing.lines[0].ordered_bn, 0.0);
    assert!(w.nation(USA).arsenal.orders.is_empty());
    assert_eq!(
        w.resources, raw,
        "an incomplete component bundle consumes no raw inputs"
    );

    next_day(&mut w);
    let plan = manufacturing::tick_allocations(&w, USA)
        .into_iter()
        .find(|p| p.line == id)
        .unwrap();
    let required = manufacturing::advanced_components_for(&w, plan.kit, plan.budget_bn);
    assert!(required > 0.0);
    w.production
        .operations
        .advanced_components
        .insert(USA, required * 0.5);
    let iron_before = resources::stockpile(&w, USA, resources::Commodity::Iron);
    manufacturing::settle_nation(&mut w, USA);
    near(w.manufacturing.lines[0].ordered_bn, plan.budget_bn * 0.5);
    near(industry_operations::advanced_component_stock(&w, USA), 0.0);
    near(
        iron_before - resources::stockpile(&w, USA, resources::Commodity::Iron),
        plan.required[resources::Commodity::Iron.idx()] * 0.5,
    );
    assert!(w
        .nation(USA)
        .arsenal
        .orders
        .iter()
        .any(|p| p.kit == arsenal::index_of(kit).unwrap()
            && p.units > 0.0
            && p.due_days.unwrap() > 365));
    assert_eq!(
        serde_json::to_string(&w.nation(USA).arsenal.held).unwrap(),
        held,
        "ordering cannot deliver the vessel immediately"
    );

    // The next date has every input but no local electricity. Existing orders
    // remain on the ledger and no additional work or raw draw can appear.
    programs::stage_fiscal(w.nation_mut(USA), 0.0, 0.0);
    programs::finish_day(&mut w);
    clock::advance_date(&mut w);
    w.production
        .provinces
        .iter_mut()
        .find(|p| p.district == d)
        .unwrap()
        .power_grid = 0;
    programs::begin_day(&mut w);
    w.production
        .operations
        .advanced_components
        .insert(USA, 100.0);
    industry_operations::begin_day(&mut w);
    let ordered = w.manufacturing.lines[0].ordered_bn;
    let resources = w.resources.clone();
    manufacturing::settle_nation(&mut w, USA);
    near(w.manufacturing.lines[0].ordered_bn, ordered);
    near(
        industry_operations::advanced_component_stock(&w, USA),
        100.0,
    );
    assert_eq!(w.resources, resources);
}

#[test]
fn custom_equipment_supply_preview_matches_joint_component_and_raw_shortages() {
    let (mut base, d) = fixture();
    base.rules.military_operations = true;
    for kind in [K::ArmsPlant, K::PowerGrid, K::Generation] {
        production::complete_capability(&mut base, &d, kind);
    }
    let spec = crate::equipment::default_spec("ground_apc");
    let profile = crate::equipment::design_preview(&base, USA, &spec)
        .profile
        .unwrap();
    let today = clock::absolute_day(&base);
    let state = base
        .nation_mut(USA)
        .equipment
        .get_or_insert_with(Default::default);
    state.finance_from_day = today;
    state.revisions.insert(
        "joint-supply".into(),
        crate::equipment::DesignRevision {
            id: "joint-supply".into(),
            name: "Joint supply fixture".into(),
            specification_key: crate::equipment::specification_key(&spec),
            spec,
            profile,
            created_day: today,
            certified_day: Some(today),
        },
    );
    let id =
        crate::equipment::start_production(&mut base, USA, "joint-supply", &d, 2, 0.1).unwrap();
    // Synthetic already-paid tooling isolates fabrication's atomic preflight.
    let p = base
        .nation_mut(USA)
        .equipment
        .as_mut()
        .unwrap()
        .projects
        .iter_mut()
        .find(|p| p.id == id)
        .unwrap();
    p.work_days = p.tooling_days as f64;
    p.spent_bn = p.tooling_cost_bn;
    clock::advance_date(&mut base);
    programs::begin_day(&mut base);
    industry_operations::begin_day(&mut base);
    let nominal = crate::equipment::supply_plan(&base, USA)
        .into_iter()
        .find(|p| p.project_id == id)
        .unwrap();
    assert!(nominal.planned_work_days > 0.0);
    assert!(nominal.advanced_components_required > 0.0);
    for (component_fraction, iron_fraction) in
        [(0.5, 0.75), (0.75, 0.5), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0)]
    {
        let mut w = base.clone();
        w.production.operations.advanced_components.insert(
            USA,
            nominal.advanced_components_required * component_fraction,
        );
        resources::set_stockpile_for_test(
            &mut w,
            USA,
            resources::Commodity::Iron,
            nominal.planned_day[resources::Commodity::Iron.idx()] * iron_fraction,
        );
        let before = crate::save(&w);
        let planned = crate::equipment::supply_plan(&w, USA)
            .into_iter()
            .find(|p| p.project_id == id)
            .unwrap();
        assert_eq!(
            crate::save(&w),
            before,
            "supply preview must not reserve or consume inventory"
        );
        let old = w
            .nation(USA)
            .equipment
            .as_ref()
            .unwrap()
            .projects
            .iter()
            .find(|p| p.id == id)
            .unwrap()
            .clone();
        let component_before = industry_operations::advanced_component_stock(&w, USA);
        crate::equipment::tick_day(&mut w);
        let p = w
            .nation(USA)
            .equipment
            .as_ref()
            .unwrap()
            .projects
            .iter()
            .find(|p| p.id == id)
            .unwrap();
        near(p.work_days - old.work_days, planned.executable_work_days);
        near(p.spent_bn - old.spent_bn, planned.executable_payment_bn);
        near(
            component_before - industry_operations::advanced_component_stock(&w, USA),
            planned.advanced_components_used,
        );
        for c in resources::ALL {
            near(
                p.resources_used[c.idx()] - old.resources_used[c.idx()],
                planned.executable_raw[c.idx()],
            );
        }
    }
}

#[test]
fn component_blocked_priority_line_cannot_reserve_raw_inputs_from_a_working_line() {
    let (mut w, d) = fixture();
    for kind in [K::Shipyard, K::ArmsPlant, K::PowerGrid, K::Generation] {
        production::complete_capability(&mut w, &d, kind);
    }
    let technology = crate::tech::index_of("aero_quiet_submarine").unwrap();
    if !w.nation(USA).tech.knows_index(technology) {
        w.nation_mut(USA).tech.grant_1990(&[technology]);
    }
    let high = manufacturing::start_line(&mut w, USA, &d, "la_ssn").unwrap();
    let low = manufacturing::start_line(&mut w, USA, &d, "arm_gen3").unwrap();
    manufacturing::set_priority(&mut w, USA, high, production::Priority::High).unwrap();
    manufacturing::set_priority(&mut w, USA, low, production::Priority::Low).unwrap();
    programs::begin_day(&mut w);
    industry_operations::begin_day(&mut w);
    let plans = manufacturing::tick_allocations(&w, USA);
    let high_plan = plans.iter().find(|p| p.line == high).unwrap();
    let low_plan = plans.iter().find(|p| p.line == low).unwrap().clone();
    assert!(manufacturing::advanced_components_for(&w, high_plan.kit, high_plan.budget_bn) > 0.0);
    assert_eq!(
        manufacturing::advanced_components_for(&w, low_plan.kit, low_plan.budget_bn),
        0.0
    );
    assert!(low_plan.required.iter().any(|v| *v > 0.0));
    w.production.operations.advanced_components.insert(USA, 0.0);
    for c in resources::ALL {
        if c != resources::Commodity::Oil {
            resources::set_stockpile_for_test(&mut w, USA, c, low_plan.required[c.idx()]);
        }
    }
    let saved = crate::save(&w);
    assert_eq!(
        manufacturing::tick_line_shortfalls(&w, low),
        [0.0; 12],
        "the earlier component-blocked line cannot reserve an operating bundle it cannot consume"
    );
    assert_eq!(crate::save(&w), saved);
    manufacturing::settle_nation(&mut w, USA);
    let upper = w.manufacturing.lines.iter().find(|p| p.id == high).unwrap();
    let lower = w.manufacturing.lines.iter().find(|p| p.id == low).unwrap();
    near(upper.ordered_bn, 0.0);
    assert_eq!(upper.resources_used, [0.0; 12]);
    near(lower.ordered_bn, low_plan.budget_bn);
    for c in resources::ALL {
        near(lower.resources_used[c.idx()], low_plan.required[c.idx()]);
    }
}

#[test]
fn company_naval_output_uses_its_frozen_component_recipe_and_actual_unit_receipt() {
    use crate::companies::{self, CompanySector, CompanyTarget};
    let (mut w, d) = fixture();
    for kind in [K::Shipyard, K::PowerGrid, K::Generation] {
        production::complete_capability(&mut w, &d, kind);
    }
    let technology = crate::tech::index_of("aero_quiet_submarine").unwrap();
    w.nation_mut(USA).tech.grant_1990(&[technology]);
    let line = manufacturing::start_line(&mut w, USA, &d, "la_ssn").unwrap();
    companies::enable(&mut w);
    let contractor = w.companies.roster.iter()
        .filter(|c|c.nation == USA && c.sector == CompanySector::Defense)
        .max_by(|a,b|a.work_bonus.total_cmp(&b.work_bonus)).unwrap().id;
    let target = CompanyTarget::Equipment { project: line };
    companies::assign(&mut w, USA, contractor, target.clone()).unwrap();
    programs::begin_day(&mut w);
    industry_operations::begin_day(&mut w);
    let plan = manufacturing::tick_allocations(&w, USA)[0].clone();
    let full_components = manufacturing::plan_advanced_components(&w, &plan);
    let physical_value = plan.budget_bn * plan.company.work_rate / (1.0 + plan.company.fee_rate);
    near(full_components, physical_value * 100.0 * plan.company.input_rate);
    assert!(full_components > 0.0);
    w.production.operations.advanced_components.insert(USA, full_components * 0.5);
    manufacturing::settle_nation(&mut w, USA);
    let receipt = &w.manufacturing.lines[0];
    near(receipt.ordered_today_bn, plan.budget_bn * 0.5);
    let expected_units = physical_value * 0.5 / arsenal::registry()[plan.kit as usize].unit_cost;
    near(receipt.ordered_today_units, expected_units);
    near(w.nation(USA).arsenal.orders.iter().map(|o|o.units).sum(), expected_units);
    near(industry_operations::advanced_component_stock(&w, USA), 0.0);
    for c in resources::ALL {
        near(receipt.resources_used[c.idx()], plan.required[c.idx()] * 0.5);
    }
    let paid_fee = w.companies.assignments[0].fees_today_bn;
    near(paid_fee, plan.budget_bn * 0.5 * plan.company.fee_rate / (1.0 + plan.company.fee_rate));
    companies::unassign(&mut w, USA, &target).unwrap();
    near(w.manufacturing.lines[0].ordered_today_units, expected_units);
}
