//! Controlled physical/fiscal feasibility, not a live-world success-rate bar.
//! GDP and ministry allocations are the actual 1990 values. Political capital
//! and finite raw stocks are explicit test fixtures so conflict, imports and
//! approval scarcity cannot masquerade as a construction-size test.
use spheres_sim::{
    apply_command, clock, economic_ai, industrial_modules as modules, industry,
    init::world_1990,
    load,
    production::{self, ProjectKind as K},
    programs, resources, save,
    world::{GameRules, NationId, WorldState, BUDGET_INDUSTRY},
    Command,
};

fn world() -> WorldState {
    let mut w = world_1990(GameRules {
        daily_simulation: true,
        economic_competition: true,
        production_system: true,
        resource_market: true,
        manufacturing_system: true,
        physical_logistics: true,
        logistics_routes: true,
        ai_aggression: 0.0,
        ..GameRules::default()
    });
    resources::tick(&mut w);
    w
}

fn prepare(base: &WorldState, id: NationId) -> WorldState {
    let mut w = base.clone();
    w.conflicts.clear();
    w.nation_mut(id).political_capital = 1000.0;
    let allocations = w.nation(id).budget_for(w.year).allocations;
    let mut departments = programs::default_departments();
    departments[BUDGET_INDUSTRY] = [6000, 1000, 1000, 1000, 1000];
    apply_command(
        &mut w,
        &Command::SetProgramBudget {
            nation: id,
            fiscal_year: 1990,
            allocations,
            departments,
        },
    )
    .unwrap();
    for c in resources::ALL {
        let stocks = &mut w.resources.market.as_mut().unwrap().stocks;
        if let Some(s) = stocks
            .iter_mut()
            .find(|s| s.nation == id && s.commodity == c)
        {
            s.quantity = 1000.0;
        } else {
            stocks.push(resources::Stock {
                nation: id,
                commodity: c,
                quantity: 1000.0,
                reserve_target: 0.0,
            });
        }
    }
    w.resources
        .market
        .as_mut()
        .unwrap()
        .stocks
        .sort_by_key(|s| (s.nation, s.commodity));
    w
}

fn district(w: &WorldState, id: NationId) -> String {
    w.districts
        .iter()
        .find(|(_, n)| **n == id)
        .unwrap()
        .0
        .clone()
}

/// The real funding, work and operating systems, with GDP held at its sourced
/// starting value. No taxes/interest are fabricated: this fixture charges all
/// actual spending through finish_day, and may finance that spending as debt.
fn physical_day(w: &mut WorldState, id: NationId) -> f64 {
    if w.nation(id).program_budget.as_ref().unwrap().fiscal_year != w.year {
        let allocations = w.nation(id).budget_for(w.year).allocations;
        let departments = w.nation(id).program_budget.as_ref().unwrap().departments;
        let fiscal_year = w.year;
        apply_command(
            w,
            &Command::SetProgramBudget {
                nation: id,
                fiscal_year,
                allocations,
                departments,
            },
        )
        .unwrap();
    }
    programs::begin_day(w);
    production::tick_day(w);
    let work = w.nation(id).program_budget.as_ref().unwrap().spent_today_bn[BUDGET_INDUSTRY][0];
    industry::tick_day(w);
    let p = w.nation_mut(id).program_budget.as_mut().unwrap();
    p.fiscal_staged = true;
    p.revenue_today_bn = 0.0;
    p.interest_today_bn = 0.0;
    programs::finish_day(w);
    clock::advance_date(w);
    work
}

fn start_recommended(w: &mut WorldState, id: NationId, d: &str) -> u32 {
    let micros = modules::recommended_capacity_micros(w, id);
    let pc = w.nation(id).political_capital;
    apply_command(
        w,
        &Command::StartIndustryModule {
            nation: id,
            district: d.into(),
            capacity_micros: micros,
        },
    )
    .unwrap();
    assert_eq!(w.nation(id).political_capital, pc - 12.0);
    assert_eq!(
        modules::capacity(w, d),
        0.0,
        "an order is not installed capacity"
    );
    micros
}

#[test]
fn all_131_mapped_countries_can_complete_a_paid_productive_module_at_normal_gdp_and_budget() {
    let base = world();
    let mut completed = 0;
    let mut unmapped = vec![];
    let mut tiers = [0; 5];
    for id in base.nations.iter().filter(|n| n.alive).map(|n| n.id) {
        let mut w = prepare(&base, id);
        let gdp = w.nation(id).gdp;
        let original_allocations = w.nation(id).budget_for(w.year).allocations;
        if !w.districts.values().any(|n| *n == id) {
            let before = save(&w);
            assert!(apply_command(
                &mut w,
                &Command::StartIndustryModule {
                    nation: id,
                    district: "UNMAPPED".into(),
                    capacity_micros: 1
                }
            )
            .is_err());
            assert_eq!(save(&w), before, "unmapped refusal must be atomic");
            unmapped.push(id);
            continue;
        }
        let d = district(&w, id);
        let micros = start_recommended(&mut w, id, &d);
        let q = modules::quote(&w, id, &d, micros);
        let annual = gdp * original_allocations[BUDGET_INDUSTRY] * 0.6;
        assert!(
            q.cost_bn <= annual,
            "{}: module cost {} must fit actual annual authority {annual}, size {micros}",
            id.name(),
            q.cost_bn
        );
        assert!(q.nominal_work_days <= 365.0 * production::construction_capacity(&w, id));
        // The recommendation sizes against the slowest daily calendar release
        // as well as workforce; equal monthly appropriations are not 365 equal
        // daily payments. Both resulting constraints must fit one year.
        assert!(q.cost_bn <= annual * (365.0 / (12.0 * 31.0)));
        let mut cost = 0.0;
        let mut days = 0;
        while modules::capacity(&w, &d) == 0.0 && days < 365 {
            cost += physical_day(&mut w, id);
            days += 1;
        }
        assert!(
            modules::capacity(&w, &d) > 0.0,
            "{} remains incomplete: {:?}",
            id.name(),
            w.production.projects
        );
        assert!(
            (90..=365).contains(&days),
            "{}: {days} days exceeds the one-year recommendation",
            id.name()
        );
        assert!(
            (cost - q.cost_bn).abs() < 1e-9,
            "{}: paid {cost}, quoted {}",
            id.name(),
            q.cost_bn
        );
        assert_eq!(w.production.industry.modules[&d], micros);
        assert_eq!(
            production::level(&w, &d, K::CivilianIndustry),
            0,
            "fractional capacity is not a free integer level"
        );
        assert_eq!(production::level(&w, &d, K::ProcessingPlant), 0);
        assert!(
            w.production.industry.goods[&id].intermediates > 0.0,
            "{} completed but produced nothing",
            id.name()
        );
        assert_eq!(
            w.nation(id).gdp,
            gdp,
            "this fixture isolates physical work, not a new GDP bonus"
        );
        assert_eq!(
            w.nation(id).budget_for(w.year).allocations,
            original_allocations
        );
        assert!(
            resources::stockpile(&w, id, resources::Commodity::Iron)
                < 1000.0 - q.recipe[resources::Commodity::Iron.idx()]
        );
        tiers[if gdp < 1.0 {
            0
        } else if gdp < 10.0 {
            1
        } else if gdp < 100.0 {
            2
        } else if gdp < 1000.0 {
            3
        } else {
            4
        }] += 1;
        completed += 1;
    }
    unmapped.sort();
    let mut expected = vec![
        NationId::Bahrain,
        NationId::Mauritius,
        NationId::Seychelles,
        NationId::Comoros,
        NationId::CapeVerde,
        NationId::Maldives,
    ];
    expected.sort();
    assert_eq!(
        unmapped, expected,
        "six explicit map-data gaps are not coverage successes"
    );
    assert_eq!(completed, 131);
    assert!(tiers.iter().all(|count| *count > 0));
}

#[test]
fn small_planner_orders_actual_enacted_size_while_rich_full_site_path_remains() {
    let base = world();
    for id in [
        NationId::Tonga,
        NationId::EquatorialGuinea,
        NationId::Bhutan,
        NationId::Malta,
    ] {
        let mut w = prepare(&base, id);
        // This planner fixture isolates the order from the separate, correctly
        // priced fiscal-consolidation decision. It does not raise GDP/budgets.
        w.nation_mut(id).debt_gdp = 0.0;
        w.nation_mut(id).debt_bn = Some(0.0);
        let d = district(&w, id);
        assert_eq!(
            economic_ai::candidate(&w, id).unwrap().1,
            K::StarterIndustry
        );
        let size = modules::recommended_capacity_micros(&w, id);
        economic_ai::evaluate(&mut w, id);
        let p = production::projects_for(&w, id).next().unwrap_or_else(|| {
            panic!(
                "{}: normal command starts affordable module: {}",
                id.name(),
                w.economic_ai.nations[&id].reason
            )
        });
        assert_eq!(p.kind, K::StarterIndustry);
        assert_eq!(p.capacity_micros, Some(size));
        assert_eq!(w.economic_ai.nations[&id].capacity_micros, Some(size));
        assert_eq!(w.economic_ai.nations[&id].project, Some(p.id));
        assert_eq!(modules::capacity(&w, &d), 0.0);
        let before = save(&w);
        economic_ai::evaluate(&mut w, id);
        assert_eq!(save(&w), before);
    }
    let w = prepare(&base, NationId::USA);
    assert_eq!(
        economic_ai::candidate(&w, NationId::USA).unwrap().1,
        K::CivilianIndustry
    );
}

#[test]
fn module_planning_does_not_erase_paid_full_size_work() {
    let id = NationId::Tonga;
    let mut w = prepare(&world(), id);
    let d = district(&w, id);
    apply_command(
        &mut w,
        &Command::StartProject {
            nation: id,
            district: d,
            kind: K::CivilianIndustry,
        },
    )
    .unwrap();
    for _ in 0..30 {
        physical_day(&mut w, id);
    }
    let p = production::projects_for(&w, id).next().unwrap().clone();
    let f = w.production.industry.projects[&p.id].clone();
    assert!(f.spent_bn > 0.0 && p.progress_days > 0.0);
    economic_ai::evaluate(&mut w, id);
    assert_eq!(production::projects_for(&w, id).next().unwrap(), &p);
    assert_eq!(w.production.industry.projects[&p.id], f);
    assert!(w.production.industry.modules.is_empty());
    assert_eq!(production::projects_for(&w, id).count(), 1);
}

#[test]
fn an_acquired_tiny_module_does_not_replace_an_established_full_size_chain() {
    let id = NationId::USA;
    let mut w = prepare(&world(), id);
    let d = district(&w, id);
    // Completed-capacity fixture: this represents an existing paid full-size
    // estate, power and processor, not a free asset granted by the planner.
    w.production
        .provinces
        .push(production::ProvinceCapabilities {
            district: d.clone(),
            civilian_industry: 1,
            power_grid: 1,
            infrastructure: 0,
            research_centers: 0,
            arms_plants: 0,
        });
    w.production
        .industry
        .sites
        .insert(d.clone(), [0, 1, 1, 0, 0, 0, 0]);
    assert_eq!(economic_ai::candidate(&w, id).unwrap().1, K::MachineryWorks);

    // A small module acquired alongside that chain must not permanently
    // redirect an established industrial nation into module-only waiting.
    w.production.industry.modules.insert(d.clone(), 10_000);
    let before = save(&w);
    let (target, kind, _) = economic_ai::candidate(&w, id).unwrap();
    assert_eq!(target, d);
    assert_eq!(kind, K::MachineryWorks);
    assert_eq!(save(&w), before, "planning does not grant capacity or GDP");
    assert_eq!(production::level(&w, &d, K::CivilianIndustry), 1);
    assert_eq!(modules::capacity(&w, &d), 0.01);
}

#[test]
fn a_productive_small_module_does_not_force_full_size_machinery_or_idle_expansion() {
    let id = NationId::Tonga;
    let mut w = prepare(&world(), id);
    let d = district(&w, id);
    start_recommended(&mut w, id, &d);
    for _ in 0..365 {
        physical_day(&mut w, id);
    }
    assert!(w.production.industry.goods[&id].intermediates > 0.0);
    let installed = w.production.industry.modules.clone();
    let candidate = economic_ai::candidate(&w, id).unwrap_err();
    assert!(candidate.contains("specialist") && candidate.contains("actual"));
    // Changing GDP or budget does not resize a completed physical object.
    w.nation_mut(id).gdp *= 10.0;
    assert_eq!(w.production.industry.modules, installed);
    assert!(economic_ai::candidate(&w, id).is_err());
    assert_eq!(production::level(&w, &d, K::MachineryWorks), 0);
}

#[test]
fn frozen_size_and_daily_accounts_replay_across_save_and_batch_boundaries() {
    let id = NationId::Bhutan;
    let mut a = prepare(&world(), id);
    let d = district(&a, id);
    start_recommended(&mut a, id, &d);
    let size = a.production.projects[0].capacity_micros;
    let mut b = load(&save(&a)).unwrap();
    for _ in 0..365 {
        physical_day(&mut a, id);
    }
    for count in [31, 59, 91, 184] {
        for _ in 0..count {
            physical_day(&mut b, id);
        }
        b = load(&save(&b)).unwrap();
    }
    assert_eq!(save(&a), save(&b));
    assert_eq!(a.production.industry.modules[&d], size.unwrap());
    assert!(a.production.industry.goods[&id].intermediates > 0.0);
}

#[test]
fn no_political_capital_or_raw_inputs_still_means_no_free_progress() {
    let id = NationId::Tonga;
    let mut w = prepare(&world(), id);
    let d = district(&w, id);
    let size = modules::recommended_capacity_micros(&w, id);
    w.nation_mut(id).political_capital = 0.0;
    let before = save(&w);
    assert!(apply_command(
        &mut w,
        &Command::StartIndustryModule {
            nation: id,
            district: d.clone(),
            capacity_micros: size
        }
    )
    .is_err());
    assert_eq!(save(&w), before);
    w.nation_mut(id).political_capital = 1000.0;
    start_recommended(&mut w, id, &d);
    w.resources
        .market
        .as_mut()
        .unwrap()
        .stocks
        .iter_mut()
        .find(|s| s.nation == id && s.commodity == resources::Commodity::Copper)
        .unwrap()
        .quantity = 0.0;
    programs::begin_day(&mut w);
    let stocks = w.resources.market.as_ref().unwrap().stocks.clone();
    production::tick_day(&mut w);
    assert_eq!(w.production.projects[0].progress_days, 0.0);
    assert_eq!(
        w.production.industry.projects[&w.production.projects[0].id].spent_bn,
        0.0
    );
    assert_eq!(w.resources.market.as_ref().unwrap().stocks, stocks);
}

#[test]
fn module_expansion_requires_delivered_demand_not_just_an_offer_or_idle_inventory() {
    use spheres_sim::commerce::{self, Good};
    let id = NationId::Tonga;
    let mut w = prepare(&world(), id);
    let d = district(&w, id);
    let micros = start_recommended(&mut w, id, &d);
    for _ in 0..365 {
        physical_day(&mut w, id);
    }
    let capacity = modules::capacity(&w, &d);
    assert!(capacity > 0.0);
    assert!(economic_ai::candidate(&w, id).is_err());
    // A settled export record is a read-model fixture here, not a sale or
    // inventory grant performed by AI. Unfulfilled escrow alone proves no use.
    w.production
        .industry
        .goods
        .get_mut(&id)
        .unwrap()
        .intermediates = 0.0;
    let today = clock::absolute_day(&w);
    let quantity = capacity * 100.0;
    let price = commerce::reference_price_bn(Good::Intermediates);
    w.commerce
        .get_or_insert_with(Default::default)
        .contracts
        .push(commerce::Contract {
            id: 1,
            buyer: NationId::USA,
            seller: id,
            good: Good::Intermediates,
            quantity,
            unit_price_bn: price,
            remaining_quantity: quantity,
            escrow_bn: quantity * price,
            delivered_quantity: 0.0,
            cancelled_quantity: 0.0,
            paid_bn: 0.0,
            accepted_day: today - 10,
            expires_day: today + 20,
            status: "active".into(),
            reason: None,
        });
    assert!(economic_ai::candidate(&w, id).is_err());
    let c = &mut w.commerce.as_mut().unwrap().contracts[0];
    c.remaining_quantity = 0.0;
    c.escrow_bn = 0.0;
    c.delivered_quantity = quantity;
    c.paid_bn = quantity * price;
    c.status = "completed".into();
    let (target, kind, _) = economic_ai::candidate(&w, id).unwrap();
    assert_eq!(kind, K::StarterIndustry);
    let next = economic_ai::module_order_capacity(&w, id, &target);
    assert!(next > 0 && next <= micros);
    let old = w.production.industry.modules.clone();
    // A fiscal-stability fixture isolates the priced project decision, not
    // an implicit debt waiver in gameplay.
    w.nation_mut(id).debt_bn = Some(0.0);
    w.nation_mut(id).debt_gdp = 0.0;
    economic_ai::evaluate(&mut w, id); // unchanged annual renewal
    for _ in 0..30 {
        clock::advance_date(&mut w);
    }
    let pc = w.nation(id).political_capital;
    economic_ai::evaluate(&mut w, id);
    let p = production::projects_for(&w, id).next().unwrap();
    assert_eq!(p.kind, K::StarterIndustry);
    assert_eq!(p.capacity_micros, Some(next));
    assert_eq!(w.nation(id).political_capital, pc - 12.0);
    assert_eq!(
        w.production.industry.modules, old,
        "expansion remains unbuilt until funded"
    );
}

#[test]
fn no_factory_authority_does_not_queue_an_unfinishable_minimum_module() {
    let id = NationId::Tonga;
    let mut w = prepare(&world(), id);
    let d = district(&w, id);
    w.nation_mut(id)
        .program_budget
        .as_mut()
        .unwrap()
        .departments[BUDGET_INDUSTRY] = [0, 2500, 2500, 2500, 2500];
    assert_eq!(economic_ai::module_order_capacity(&w, id, &d), 0);
    assert!(economic_ai::candidate(&w, id)
        .unwrap_err()
        .contains("cannot fund"));
    assert!(w.production.projects.is_empty());
}
