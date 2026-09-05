//! Structural coverage, not a statistical calibration claim: every 1990
//! country is examined, including the smallest and unmapped governments.
use spheres_sim::world::{GameRules, NationId, WorldState, BUDGET_INDUSTRY};
use spheres_sim::{
    apply_command, clock, economic_ai,
    init::world_1990,
    load, politics,
    production::{self, ProjectKind},
    programs, resources, save, tick_day, Command,
};

fn world() -> WorldState {
    world_1990(GameRules {
        daily_simulation: true,
        economic_competition: true,
        production_system: true,
        resource_market: true,
        manufacturing_system: true,
        logistics_routes: true,
        physical_logistics: true,
        ai_aggression: 0.0,
        ..GameRules::default()
    })
}

fn tier(gdp: f64) -> usize {
    if gdp < 1.0 {
        0
    } else if gdp < 10.0 {
        1
    } else if gdp < 100.0 {
        2
    } else if gdp < 1000.0 {
        3
    } else {
        4
    }
}

#[test]
fn all_137_starting_countries_are_independently_eligible_not_a_rich_whitelist() {
    let base = world();
    let roster: Vec<_> = base
        .nations
        .iter()
        .filter(|n| n.alive)
        .map(|n| n.id)
        .collect();
    assert_eq!(roster.len(), 137);
    let mut tiers = [0; 5];
    for id in roster {
        let mut w = base.clone();
        // Viable political capacity is a fixture, not a granted game stock.
        w.nation_mut(id).political_capital = 1000.0;
        let before = w.nation(id).political_capital;
        let gdp = w.nation(id).gdp;
        economic_ai::evaluate(&mut w, id);
        let p = w
            .economic_ai
            .nations
            .get(&id)
            .expect("every nation evaluated");
        assert_eq!(p.evaluations, 1, "{}", id.name());
        assert!(!p.reason.is_empty(), "{}", id.name());
        assert!(programs::enrolled(&w, id), "{}: {}", id.name(), p.reason);
        assert_eq!(
            w.nation(id).political_capital,
            before - 6.0,
            "same enrollment price for {}",
            id.name()
        );
        assert_eq!(w.nation(id).gdp, gdp, "enrollment is not a GDP gift");
        assert!(w.production.projects.is_empty());
        assert!(w.production.industry.goods.is_empty());
        tiers[tier(gdp)] += 1;
        let unchanged = save(&w);
        economic_ai::evaluate(&mut w, id);
        assert_eq!(save(&w), unchanged, "same-day review is inert");
    }
    assert!(tiers.into_iter().all(|n| n > 0));
}

#[test]
fn viable_paid_work_is_nonzero_in_every_size_tier_and_every_mapped_country() {
    let mut base = world();
    resources::tick(&mut base);
    let roster: Vec<_> = base
        .nations
        .iter()
        .filter(|n| n.alive)
        .map(|n| n.id)
        .collect();
    let mut tier_paid = [0; 5];
    let mut unmapped = 0;
    for id in roster {
        let mut w = base.clone();
        w.nation_mut(id).political_capital = 1000.0;
        // Isolate feasibility from macro debt/sanctions. Cash and materials
        // remain normal finite ledger claims; game initialization is untouched.
        w.nation_mut(id).debt_gdp = 0.0;
        economic_ai::evaluate(&mut w, id);
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
        for _ in 0..economic_ai::REVIEW_DAYS {
            clock::advance_date(&mut w);
        }
        economic_ai::evaluate(&mut w, id);
        if !w.districts.values().any(|owner| *owner == id) {
            unmapped += 1;
            assert!(w.economic_ai.nations[&id]
                .reason
                .contains("mapped province"));
            continue;
        }
        assert_eq!(
            production::projects_for(&w, id).count(),
            1,
            "{}: {}",
            id.name(),
            w.economic_ai.nations[&id].reason
        );
        let project = production::projects_for(&w, id).next().unwrap().id;
        let before = resources::stockpile(&w, id, resources::Commodity::Iron);
        programs::begin_day(&mut w);
        production::tick_day(&mut w);
        let p = production::projects_for(&w, id).next().unwrap();
        assert!(p.progress_days > 0.0, "{}: {:?}", id.name(), p.reason);
        assert!(w.production.industry.projects[&project].spent_bn > 0.0);
        assert!(resources::stockpile(&w, id, resources::Commodity::Iron) < before);
        assert!(
            w.nation(id).program_budget.as_ref().unwrap().spent_today_bn[BUDGET_INDUSTRY][0] > 0.0
        );
        tier_paid[tier(w.nation(id).gdp)] += 1;
    }
    assert!(tier_paid.into_iter().all(|n| n > 0));
    assert!(unmapped < 137, "the progress assertion must not be vacuous");
}

#[test]
fn disabled_monthly_player_and_dead_paths_remain_byte_inert() {
    for mode in 0..4 {
        let mut w = world();
        match mode {
            0 => w.rules.economic_competition = false,
            1 => w.rules.daily_simulation = false,
            2 => w.player = Some(NationId::USA),
            _ => w.nation_mut(NationId::USA).alive = false,
        }
        let before = save(&w);
        economic_ai::evaluate(&mut w, NationId::USA);
        assert_eq!(save(&w), before);
    }
}

#[test]
fn political_cost_ownership_material_and_queue_rules_are_not_bypassed() {
    let mut w = world();
    let id = NationId::USA;
    w.nation_mut(id).political_capital = 0.0;
    economic_ai::evaluate(&mut w, id);
    assert!(!programs::enrolled(&w, id));
    assert_eq!(w.economic_ai.nations[&id].action, "blocked");
    assert!(w.economic_ai.nations[&id]
        .reason
        .contains("political capital"));
    let own = w
        .districts
        .iter()
        .find(|(_, n)| **n == id)
        .unwrap()
        .0
        .clone();
    let foreign = w
        .districts
        .iter()
        .find(|(_, n)| **n != id)
        .unwrap()
        .0
        .clone();
    w.nation_mut(id).political_capital = 1000.0;
    let allocations = w.nation(id).budget_for(w.year).allocations;
    apply_command(
        &mut w,
        &Command::SetProgramBudget {
            nation: id,
            fiscal_year: 1990,
            allocations,
            departments: programs::default_departments(),
        },
    )
    .unwrap();
    assert!(apply_command(
        &mut w,
        &Command::StartProject {
            nation: id,
            district: foreign,
            kind: ProjectKind::CivilianIndustry
        }
    )
    .is_err());
    let pc = w.nation(id).political_capital;
    apply_command(
        &mut w,
        &Command::StartProject {
            nation: id,
            district: own,
            kind: ProjectKind::CivilianIndustry,
        },
    )
    .unwrap();
    assert_eq!(
        w.nation(id).political_capital,
        pc - production::catalog(ProjectKind::CivilianIndustry).political_cost
    );
    programs::begin_day(&mut w);
    production::tick_day(&mut w);
    assert_eq!(
        production::projects_for(&w, id)
            .next()
            .unwrap()
            .progress_days,
        0.0
    );
    assert_eq!(
        w.production
            .industry
            .projects
            .values()
            .next()
            .unwrap()
            .spent_bn,
        0.0
    );
    let before = save(&w);
    economic_ai::evaluate(&mut w, id);
    assert_eq!(save(&w), before, "same-day retries cannot spam the queue");
}

#[test]
fn enrolled_fiscal_plans_are_not_silently_overwritten_by_legacy_ai() {
    let mut w = world();
    let id = NationId::USA;
    w.nation_mut(id).political_capital = 1000.0;
    economic_ai::evaluate(&mut w, id);
    w.nation_mut(id).debt_gdp = 1.2;
    let gdp = w.nation(id).gdp;
    w.nation_mut(id).debt_bn = Some(gdp * 1.2);
    w.nation_mut(id)
        .program_budget
        .as_mut()
        .unwrap()
        .settled_spending_annual_bn = gdp * 0.5;
    let before = (
        w.nation(id).tax_rate,
        w.nation(id).mil_spend_gdp,
        w.nation(id).state_invest_gdp,
    );
    politics::tick(&mut w);
    assert_eq!(
        before,
        (
            w.nation(id).tax_rate,
            w.nation(id).mil_spend_gdp,
            w.nation(id).state_invest_gdp
        )
    );
    for _ in 0..30 {
        clock::advance_date(&mut w);
    }
    economic_ai::evaluate(&mut w, id);
    assert_eq!(w.nation(id).tax_rate, (before.0 + 0.01).min(0.55));
    assert_eq!(w.economic_ai.nations[&id].action, "fiscal_consolidation");
}

#[test]
fn year_renewal_precedes_fiscal_adjustment_and_preserves_work() {
    let mut w = world();
    let id = NationId::USA;
    w.nation_mut(id).political_capital = 1000.0;
    economic_ai::evaluate(&mut w, id);
    let district = w
        .districts
        .iter()
        .find(|(_, owner)| **owner == id)
        .unwrap()
        .0
        .clone();
    apply_command(
        &mut w,
        &Command::StartProject {
            nation: id,
            district,
            kind: ProjectKind::CivilianIndustry,
        },
    )
    .unwrap();
    // A partially installed fixture must survive the change of authorization
    // year; neither work nor its already-spent money is reset or repaid.
    w.production.projects[0].progress_days = 17.0;
    let project = w.production.projects[0].id;
    w.production
        .industry
        .projects
        .get_mut(&project)
        .unwrap()
        .spent_bn = 0.18 * 17.0 / 540.0;
    let spent = w.production.industry.projects[&project].spent_bn;
    w.year = 1991;
    w.month = 1;
    w.day = 1;
    w.nation_mut(id).debt_gdp = 2.0;
    w.nation_mut(id).political_capital = 0.0;
    economic_ai::evaluate(&mut w, id);
    assert_eq!(
        w.nation(id).program_budget.as_ref().unwrap().fiscal_year,
        1991
    );
    assert_eq!(w.economic_ai.nations[&id].action, "budget");
    assert_eq!(
        w.nation(id).political_capital,
        0.0,
        "unchanged annual renewal is genuinely zero-cost"
    );
    assert_eq!(w.production.projects[0].progress_days, 17.0);
    assert_eq!(w.production.industry.projects[&project].spent_bn, spent);
}

#[test]
fn daily_competition_replays_across_save_resume_and_review_boundaries() {
    let mut a = world();
    let mut b = a.clone();
    for day in 0..65 {
        tick_day(&mut a, &[]);
        tick_day(&mut b, &[]);
        if day == 29 || day == 31 {
            b = load(&save(&b)).unwrap();
        }
    }
    assert_eq!(save(&a), save(&b));
    assert_eq!(a.economic_ai.nations.len(), 137);
    assert!(a.economic_ai.nations.values().all(|p| p.evaluations == 3));
}

#[test]
fn ai_sells_only_surplus_and_buys_real_goods_with_cash_not_free_inputs() {
    use spheres_sim::commerce::{self, Good};
    let mut w = world();
    resources::tick(&mut w);
    let buyer = NationId::USA;
    let seller = NationId::Germany;
    for id in [buyer, seller] {
        w.nation_mut(id).political_capital = 1000.0;
        w.nation_mut(id).debt_gdp = 0.0;
        economic_ai::evaluate(&mut w, id);
        w.nation_mut(id).treasury_bn = Some(1.0);
    }
    // Finite production inventory is a fixture; it is not seeded in gameplay.
    w.production.industry.goods.insert(
        seller,
        spheres_sim::industry::Goods {
            intermediates: 50.0,
            capital_goods: 50.0,
        },
    );
    let district = w
        .districts
        .iter()
        .find(|(_, owner)| **owner == buyer)
        .unwrap()
        .0
        .clone();
    apply_command(
        &mut w,
        &Command::StartProject {
            nation: buyer,
            district,
            kind: ProjectKind::Warehouse,
        },
    )
    .unwrap();
    for _ in 0..30 {
        clock::advance_date(&mut w);
    }
    economic_ai::evaluate(&mut w, seller);
    assert!(
        commerce::sale(&w, seller, Good::Intermediates)
            .unwrap()
            .enabled
    );
    assert!(commerce::available_to_sell(&w, seller, Good::Intermediates) <= 50.0);
    economic_ai::evaluate(&mut w, buyer); // same-priced budget reallocation
    for _ in 0..30 {
        clock::advance_date(&mut w);
    }
    // Put this transaction in the micro-economy band so the GDP-scaled lot
    // ceiling, not just the goods recipe, is the binding constraint.
    w.nation_mut(buyer).gdp = 0.1;
    let cash = w.nation(buyer).treasury_bn.unwrap();
    economic_ai::evaluate(&mut w, buyer);
    assert_eq!(
        w.economic_ai.nations[&buyer].action, "goods_trade",
        "{}",
        w.economic_ai.nations[&buyer].reason
    );
    assert!(w.nation(buyer).treasury_bn.unwrap() < cash);
    assert_eq!(
        commerce::stock(&w, buyer, Good::Intermediates),
        0.0,
        "orders are not delivered goods"
    );
    assert!(commerce::pending(&w, buyer, Good::Intermediates) > 0.0);
    let paid = cash - w.nation(buyer).treasury_bn.unwrap();
    assert!(paid <= w.nation(buyer).gdp * 0.001);
    assert!(paid > w.nation(buyer).gdp * 0.00099);
    assert_eq!(production::projects_for(&w, buyer).count(), 1);
    let copy = save(&w);
    economic_ai::evaluate(&mut w, buyer);
    assert_eq!(save(&w), copy, "no duplicate same-day escrow");
}

#[test]
fn actual_export_offer_allows_specialization_without_inventing_a_processor() {
    use spheres_sim::commerce::{self, Good};
    let mut w = world();
    for id in [NationId::USA, NationId::Germany] {
        w.nation_mut(id).political_capital = 1000.0;
        economic_ai::evaluate(&mut w, id);
        w.nation_mut(id).treasury_bn = Some(1.0);
    }
    let d = w
        .districts
        .iter()
        .find(|(_, owner)| **owner == NationId::USA)
        .unwrap()
        .0
        .clone();
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
        .insert(d.clone(), [0, 1, 0, 0, 0, 0, 0]);
    assert_eq!(
        economic_ai::candidate(&w, NationId::USA).unwrap().1,
        ProjectKind::ProcessingPlant
    );
    w.production.industry.goods.insert(
        NationId::Germany,
        spheres_sim::industry::Goods {
            intermediates: 50.0,
            capital_goods: 0.0,
        },
    );
    apply_command(
        &mut w,
        &Command::SetGoodsSale {
            nation: NationId::Germany,
            good: Good::Intermediates,
            reserve: 15.0,
            ask_multiplier: 1.0,
            enabled: true,
        },
    )
    .unwrap();
    assert!(economic_ai::candidate(&w, NationId::USA)
        .unwrap_err()
        .contains("Accumulate"));
    w.economic_ai
        .nations
        .get_mut(&NationId::USA)
        .unwrap()
        .last_review_day -= economic_ai::REVIEW_DAYS;
    economic_ai::evaluate(&mut w, NationId::USA);
    assert!(commerce::pending(&w, NationId::USA, Good::Intermediates) >= 15.0);
    assert!(
        production::projects_for(&w, NationId::USA).all(|p| !matches!(
            p.kind,
            ProjectKind::ProcessingPlant | ProjectKind::MachineryWorks
        ))
    );
    assert_eq!(
        economic_ai::candidate(&w, NationId::USA).unwrap().1,
        ProjectKind::MachineryWorks,
        "the real paid inbound lot, not a visible offer, enables specialization"
    );
    assert_eq!(production::level(&w, &d, ProjectKind::ProcessingPlant), 0);
}

#[test]
fn export_policy_protects_commissioned_consumers_before_their_completion() {
    use spheres_sim::commerce::Good;
    let mut w = world();
    let id = NationId::USA;
    w.nation_mut(id).political_capital = 1000.0;
    economic_ai::evaluate(&mut w, id);
    let district = w
        .districts
        .iter()
        .find(|(_, owner)| **owner == id)
        .unwrap()
        .0
        .clone();
    w.production
        .provinces
        .push(production::ProvinceCapabilities {
            district: district.clone(),
            civilian_industry: 1,
            power_grid: 0,
            infrastructure: 0,
            research_centers: 0,
            arms_plants: 0,
        });
    apply_command(
        &mut w,
        &Command::StartProject {
            nation: id,
            district: district.clone(),
            kind: ProjectKind::MachineryWorks,
        },
    )
    .unwrap();
    apply_command(
        &mut w,
        &Command::StartProject {
            nation: id,
            district,
            kind: ProjectKind::ResearchCenter,
        },
    )
    .unwrap();
    assert_eq!(
        economic_ai::export_reserve(&w, id, Good::Intermediates),
        18.0
    );
    assert_eq!(economic_ai::export_reserve(&w, id, Good::CapitalGoods), 3.0);
    assert!(
        w.production.industry.goods.is_empty(),
        "reserves are policies, not granted inventory"
    );
}

#[test]
fn funding_horizon_exposes_micro_economy_delays_without_discounting_the_site() {
    let mut w = world();
    let id = NationId::USA;
    w.nation_mut(id).political_capital = 1000.0;
    economic_ai::evaluate(&mut w, id);
    let d = w
        .districts
        .iter()
        .find(|(_, owner)| **owner == id)
        .unwrap()
        .0
        .clone();
    w.nation_mut(id).gdp = 0.1;
    let small = economic_ai::funding_horizon(&w, id, &d, ProjectKind::CivilianIndustry);
    w.nation_mut(id).gdp = 1.0;
    let large = economic_ai::funding_horizon(&w, id, &d, ProjectKind::CivilianIndustry);
    assert_eq!(small.remaining_work_cost_bn, 0.18);
    assert_eq!(small.remaining_work_cost_bn, large.remaining_work_cost_bn);
    assert!((small.annual_authority_bn * 10.0 - large.annual_authority_bn).abs() < 1e-12);
    assert!(
        small.earliest_years.unwrap() > 5.0,
        "a paid slice is not an arcade-speed completion"
    );
    assert!(small.funding_years.unwrap() > large.funding_years.unwrap());
    assert_eq!(small.unshared_work_years, large.unshared_work_years);
    assert_eq!(production::level(&w, &d, ProjectKind::CivilianIndustry), 0);
}

#[test]
fn an_unaffordable_new_priority_cannot_prevent_free_annual_renewal() {
    let mut w = world();
    let id = NationId::USA;
    w.nation_mut(id).political_capital = 1000.0;
    economic_ai::evaluate(&mut w, id);
    let shares = w.nation(id).program_budget.as_ref().unwrap().departments;
    let district = w
        .districts
        .iter()
        .find(|(_, owner)| **owner == id)
        .unwrap()
        .0
        .clone();
    w.production
        .provinces
        .push(production::ProvinceCapabilities {
            district,
            civilian_industry: 1,
            infrastructure: 0,
            power_grid: 0,
            research_centers: 0,
            arms_plants: 0,
        });
    assert_eq!(
        economic_ai::candidate(&w, id).unwrap().1,
        ProjectKind::Generation
    );
    w.year = 1991;
    w.month = 1;
    w.day = 1;
    w.nation_mut(id).political_capital = 0.0;
    economic_ai::evaluate(&mut w, id);
    let plan = w.nation(id).program_budget.as_ref().unwrap();
    assert_eq!(plan.fiscal_year, 1991);
    assert_eq!(
        plan.departments, shares,
        "future reprioritization is a separate priced choice"
    );
    assert_eq!(w.nation(id).political_capital, 0.0);
}

#[test]
fn lost_province_project_is_cancelled_without_refund_then_replanned() {
    let mut w = world();
    resources::tick(&mut w);
    let id = NationId::USA;
    w.nation_mut(id).political_capital = 1000.0;
    w.nation_mut(id).debt_gdp = 0.0;
    economic_ai::evaluate(&mut w, id);
    for _ in 0..30 {
        clock::advance_date(&mut w);
    }
    economic_ai::evaluate(&mut w, id);
    let p = production::projects_for(&w, id).next().unwrap().clone();
    w.production.projects[0].progress_days = 10.0;
    w.production
        .industry
        .projects
        .get_mut(&p.id)
        .unwrap()
        .spent_bn = 0.01;
    spheres_sim::districts::transfer_district(&mut w, id, NationId::Canada, &p.district).unwrap();
    for _ in 0..30 {
        clock::advance_date(&mut w);
    }
    let cash = w.nation(id).treasury_bn;
    let pc = w.nation(id).political_capital;
    let stocks = w.resources.clone();
    economic_ai::evaluate(&mut w, id);
    assert!(
        !w.production.projects.iter().any(|q| q.id == p.id),
        "a permanently foreign site cannot pin the planner forever"
    );
    assert_eq!(
        w.nation(id).treasury_bn,
        cash,
        "ordinary cancellation does not refund spent capital"
    );
    assert_eq!(
        w.nation(id).political_capital,
        pc,
        "ordinary cancellation is zero PC"
    );
    assert_eq!(
        serde_json::to_string(&w.resources).unwrap(),
        serde_json::to_string(&stocks).unwrap()
    );
    assert_eq!(w.economic_ai.nations[&id].action, "cancel_project");
    for _ in 0..30 {
        clock::advance_date(&mut w);
    }
    economic_ai::evaluate(&mut w, id);
    let replacement = production::projects_for(&w, id).next().unwrap();
    assert_ne!(replacement.district, p.district);
    assert_eq!(w.districts.get(&replacement.district), Some(&id));
    assert_eq!(
        w.nation(id).political_capital,
        pc - production::catalog(replacement.kind).political_cost
    );
}

#[test]
fn routine_goods_replenishment_defers_a_processor_until_the_trade_route_closes() {
    use spheres_sim::{
        commerce::{self, Good},
        industry,
    };
    let mut w = world();
    resources::tick(&mut w);
    let id = NationId::USA;
    let seller = NationId::Germany;
    for nation in [id, seller] {
        w.nation_mut(nation).political_capital = 1000.0;
        w.nation_mut(nation).debt_gdp = 0.0;
        economic_ai::evaluate(&mut w, nation);
        w.nation_mut(nation).treasury_bn = Some(1.0);
    }
    let d = w
        .districts
        .iter()
        .find(|(_, owner)| **owner == id)
        .unwrap()
        .0
        .clone();
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
        .insert(d.clone(), [1, 1, 0, 0, 0, 0, 0]);
    w.production.industry.goods.insert(
        id,
        industry::Goods {
            intermediates: 1.0,
            capital_goods: 5.0,
        },
    );
    w.production.industry.goods.insert(
        seller,
        industry::Goods {
            intermediates: 50.0,
            capital_goods: 0.0,
        },
    );
    apply_command(
        &mut w,
        &Command::SetGoodsSale {
            nation: seller,
            good: Good::Intermediates,
            reserve: 0.0,
            ask_multiplier: 1.0,
            enabled: true,
        },
    )
    .unwrap();
    let (_, kind, _) = economic_ai::candidate(&w, id).unwrap();
    let allocations = w.nation(id).budget_for(w.year).allocations;
    let mut departments = w.nation(id).program_budget.as_ref().unwrap().departments;
    if production::catalog(kind).funding_ministry == BUDGET_INDUSTRY {
        departments[BUDGET_INDUSTRY] = [1000; 5];
        departments[BUDGET_INDUSTRY][production::funding_department(kind)] = 6000;
    }
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
    programs::begin_day(&mut w);
    industry::tick_day(&mut w);
    assert!(
        industry::snapshot(&w, id)
            .sites
            .iter()
            .any(|s| s.output_daily > 0.0),
        "the fixture actually operates its existing machinery"
    );
    assert!(commerce::shortage(&w, id, Good::Intermediates) > 0.0);
    for _ in 0..30 {
        clock::advance_date(&mut w);
    }
    let pc = w.nation(id).political_capital;
    let cash = w.nation(id).treasury_bn.unwrap();
    economic_ai::evaluate(&mut w, id);
    assert!(
        commerce::pending(&w, id, Good::Intermediates) > 0.0,
        "routine replenishment still executes"
    );
    assert!(w.nation(id).treasury_bn.unwrap() < cash);
    assert_eq!(
        production::projects_for(&w, id).count(),
        0,
        "an accepted import must preempt the stale processor candidate"
    );
    assert_eq!(
        w.nation(id).political_capital,
        pc - 2.0,
        "this review pays only the normal goods-trade command cost"
    );
    assert_eq!(w.economic_ai.nations[&id].action, "goods_trade");
    let copy = save(&w);
    economic_ai::evaluate(&mut w, id);
    assert_eq!(
        save(&w),
        copy,
        "no second import or project on the same day"
    );

    apply_command(
        &mut w,
        &Command::SetGoodsSale {
            nation: seller,
            good: Good::Intermediates,
            reserve: 0.0,
            ask_multiplier: 1.0,
            enabled: false,
        },
    )
    .unwrap();
    let market = w.commerce.as_mut().unwrap();
    for contract in market.contracts.iter_mut().filter(|c| c.buyer == id) {
        contract.remaining_quantity = 0.0;
        contract.status = "expired".into();
    }
    market.cargo.retain(|cargo| cargo.buyer != id);
    w.economic_ai.nations.get_mut(&id).unwrap().last_review_day -= economic_ai::REVIEW_DAYS;
    economic_ai::evaluate(&mut w, id);
    assert_eq!(
        production::projects_for(&w, id).count(),
        1,
        "the paid physical fallback is allowed after no executable route remains"
    );
    assert_eq!(production::projects_for(&w, id).next().unwrap().kind, kind);
    assert_eq!(
        w.nation(id).political_capital,
        pc - 2.0 - production::catalog(kind).political_cost,
        "trade and later construction each pay their ordinary command price"
    );
}

#[test]
fn january_stranded_cleanup_preserves_zero_cost_annual_renewal() {
    let mut w = world();
    let id = NationId::USA;
    w.nation_mut(id).political_capital = 1000.0;
    economic_ai::evaluate(&mut w, id);
    let district = w
        .districts
        .iter()
        .find(|(_, owner)| **owner == id)
        .unwrap()
        .0
        .clone();
    apply_command(
        &mut w,
        &Command::StartProject {
            nation: id,
            district: district.clone(),
            kind: ProjectKind::CivilianIndustry,
        },
    )
    .unwrap();
    spheres_sim::districts::transfer_district(&mut w, id, NationId::Canada, &district).unwrap();
    w.year = 1991;
    w.month = 1;
    w.day = 1;
    w.nation_mut(id).political_capital = 0.0;
    let cash = w.nation(id).treasury_bn;
    let departments = w.nation(id).program_budget.as_ref().unwrap().departments;
    economic_ai::evaluate(&mut w, id);
    let budget = w.nation(id).program_budget.as_ref().unwrap();
    assert_eq!(
        budget.fiscal_year, 1991,
        "cleanup must not freeze capital authority until the next 30-day review"
    );
    assert_eq!(budget.departments, departments);
    assert_eq!(
        production::projects_for(&w, id).count(),
        0,
        "the lost province still receives ordinary cancellation"
    );
    assert_eq!(w.nation(id).political_capital, 0.0);
    assert_eq!(w.nation(id).treasury_bn, cash);
}

#[test]
fn full_intermediate_storage_cannot_preempt_the_first_capital_goods_producer() {
    use spheres_sim::industry;
    let mut w = world();
    resources::tick(&mut w);
    let id = NationId::USA;
    w.nation_mut(id).political_capital = 1000.0;
    w.nation_mut(id).debt_gdp = 0.0;
    economic_ai::evaluate(&mut w, id);
    let district = w
        .districts
        .iter()
        .find(|(_, owner)| **owner == id)
        .unwrap()
        .0
        .clone();
    w.production
        .provinces
        .push(production::ProvinceCapabilities {
            district: district.clone(),
            civilian_industry: 1,
            power_grid: 1,
            infrastructure: 0,
            research_centers: 0,
            arms_plants: 0,
        });
    w.production
        .industry
        .sites
        .insert(district.clone(), [0, 1, 1, 0, 0, 0, 0]);
    // A finite, legitimately producible inventory fixture. There is no machine
    // shop or capital pack anywhere, so a warehouse has no capital supplier.
    w.production.industry.goods.insert(
        id,
        industry::Goods {
            intermediates: 250.0,
            capital_goods: 0.0,
        },
    );
    assert_eq!(
        industry::goods_recipe(ProjectKind::MachineryWorks).capital_goods,
        0.0
    );
    assert_eq!(
        industry::goods_recipe(ProjectKind::Warehouse).capital_goods,
        5.0
    );
    let (_, kind, _) = economic_ai::candidate(&w, id).unwrap();
    assert_eq!(kind, ProjectKind::MachineryWorks, "a full processor must build its capital-goods consumer before a warehouse requiring those unavailable goods");
    for _ in 0..30 {
        clock::advance_date(&mut w);
    }
    let pc = w.nation(id).political_capital;
    economic_ai::evaluate(&mut w, id);
    let project = production::projects_for(&w, id).next().unwrap();
    assert_eq!(project.kind, ProjectKind::MachineryWorks);
    assert_eq!(
        w.nation(id).political_capital,
        pc - production::catalog(ProjectKind::MachineryWorks).political_cost
    );
    assert_eq!(
        production::level(&w, &district, ProjectKind::MachineryWorks),
        0,
        "the queued fix does not grant completed output"
    );
}

#[test]
fn a_loaded_capital_starved_warehouse_keeps_paid_work_and_builds_its_missing_machine() {
    use spheres_sim::industry;
    let mut w = world();
    resources::tick(&mut w);
    let id = NationId::USA;
    w.nation_mut(id).political_capital = 1000.0;
    w.nation_mut(id).debt_gdp = 0.0;
    economic_ai::evaluate(&mut w, id);
    let district = w
        .districts
        .iter()
        .find(|(_, owner)| **owner == id)
        .unwrap()
        .0
        .clone();
    w.production
        .provinces
        .push(production::ProvinceCapabilities {
            district: district.clone(),
            civilian_industry: 1,
            power_grid: 1,
            infrastructure: 0,
            research_centers: 0,
            arms_plants: 0,
        });
    w.production
        .industry
        .sites
        .insert(district.clone(), [0, 1, 1, 0, 0, 0, 0]);
    // A saved warehouse may already have made genuine paid progress using a
    // finite capital lot, then exhausted it without a domestic machine shop.
    w.production.industry.goods.insert(
        id,
        industry::Goods {
            intermediates: 250.0,
            capital_goods: 0.25,
        },
    );
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
    let allocations = w.nation(id).budget_for(w.year).allocations;
    let mut departments = w.nation(id).program_budget.as_ref().unwrap().departments;
    departments[BUDGET_INDUSTRY] = [1000, 1000, 1000, 6000, 1000];
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
    apply_command(
        &mut w,
        &Command::StartProject {
            nation: id,
            district: district.clone(),
            kind: ProjectKind::Warehouse,
        },
    )
    .unwrap();
    for _ in 0..20 {
        programs::begin_day(&mut w);
        production::tick_day(&mut w);
        programs::finish_day(&mut w);
        clock::advance_date(&mut w);
    }
    let warehouse = production::projects_for(&w, id).next().unwrap().clone();
    let spent = w.production.industry.projects[&warehouse.id].spent_bn;
    assert!(spent > 0.0 && warehouse.progress_days > 0.0);
    assert!(warehouse
        .reason
        .as_ref()
        .is_some_and(|r| r.contains("capital-goods")));
    w = load(&save(&w)).unwrap();
    for _ in 0..10 {
        clock::advance_date(&mut w);
    }
    economic_ai::evaluate(&mut w, id); // normal priced funding reprioritization
    assert_eq!(
        w.nation(id).program_budget.as_ref().unwrap().departments[BUDGET_INDUSTRY][0],
        6000,
        "fund the missing raw-only prerequisite rather than repeating the blocked warehouse target"
    );
    assert_eq!(
        w.economic_ai.nations[&id].project, None,
        "an unqueued prerequisite must not display the older warehouse's id"
    );
    for _ in 0..30 {
        clock::advance_date(&mut w);
    }
    let pc = w.nation(id).political_capital;
    let cash = w.nation(id).treasury_bn;
    economic_ai::evaluate(&mut w, id);
    assert_eq!(production::projects_for(&w, id).count(), 2);
    assert!(production::projects_for(&w, id).any(|p| p.kind == ProjectKind::MachineryWorks));
    assert_eq!(
        w.economic_ai.nations[&id].project,
        production::projects_for(&w, id)
            .find(|p| p.kind == ProjectKind::MachineryWorks)
            .map(|p| p.id)
    );
    let retained = w
        .production
        .projects
        .iter()
        .find(|p| p.id == warehouse.id)
        .unwrap();
    assert_eq!(retained.progress_days, warehouse.progress_days);
    assert_eq!(
        w.production.industry.projects[&warehouse.id].spent_bn,
        spent
    );
    assert_eq!(
        w.nation(id).treasury_bn,
        cash,
        "neither refunds nor free project funding"
    );
    assert_eq!(
        w.nation(id).political_capital,
        pc - production::catalog(ProjectKind::MachineryWorks).political_cost
    );
    for _ in 0..30 {
        clock::advance_date(&mut w);
    }
    economic_ai::evaluate(&mut w, id);
    assert_eq!(w.nation(id).program_budget.as_ref().unwrap().departments[BUDGET_INDUSTRY][0],6000,
        "do not oscillate the funding target back to the blocked parent while its prerequisite is queued");
    assert_eq!(
        production::projects_for(&w, id).count(),
        2,
        "the two-project bound is retained"
    );
    programs::begin_day(&mut w);
    production::tick_day(&mut w);
    assert!(
        production::projects_for(&w, id)
            .any(|p| p.kind == ProjectKind::MachineryWorks && p.progress_days > 0.0),
        "the prerequisite must perform real paid work without manufactured capital inputs"
    );
}

#[test]
fn warehouse_rescue_obeys_power_processing_ownership_pc_and_queue_constraints() {
    use spheres_sim::industry;
    let id = NationId::USA;
    let mut base = world();
    resources::tick(&mut base);
    base.nation_mut(id).political_capital = 1000.0;
    base.nation_mut(id).debt_gdp = 0.0;
    economic_ai::evaluate(&mut base, id);
    let d = base
        .districts
        .iter()
        .find(|(_, owner)| **owner == id)
        .unwrap()
        .0
        .clone();
    base.production
        .provinces
        .push(production::ProvinceCapabilities {
            district: d.clone(),
            civilian_industry: 1,
            power_grid: 1,
            infrastructure: 0,
            research_centers: 0,
            arms_plants: 0,
        });
    base.production
        .industry
        .sites
        .insert(d.clone(), [0, 1, 1, 0, 0, 0, 0]);
    base.production.industry.goods.insert(
        id,
        industry::Goods {
            intermediates: 250.0,
            capital_goods: 0.0,
        },
    );
    apply_command(
        &mut base,
        &Command::StartProject {
            nation: id,
            district: d.clone(),
            kind: ProjectKind::Warehouse,
        },
    )
    .unwrap();
    for mode in [
        "power",
        "grid",
        "processing",
        "pc",
        "queue",
        "foreign",
        "contested",
    ] {
        let mut w = base.clone();
        match mode {
            "power" => w.production.industry.sites.get_mut(&d).unwrap()[1] = 0,
            "grid" => {
                w.production
                    .provinces
                    .iter_mut()
                    .find(|p| p.district == d)
                    .unwrap()
                    .power_grid = 0
            }
            "processing" => w.production.industry.sites.get_mut(&d).unwrap()[2] = 0,
            "pc" => w.nation_mut(id).political_capital = 0.0,
            "queue" => {
                apply_command(
                    &mut w,
                    &Command::StartProject {
                        nation: id,
                        district: d.clone(),
                        kind: ProjectKind::ResearchCenter,
                    },
                )
                .unwrap();
            }
            "foreign" => {
                w.districts.insert(d.clone(), NationId::Canada);
            }
            "contested" => {
                spheres_sim::war::declare_war(&mut w, NationId::Canada, id).unwrap();
                assert!(resources::district_contested(&w, &d));
            }
            _ => unreachable!(),
        }
        for _ in 0..30 {
            clock::advance_date(&mut w);
        }
        economic_ai::evaluate(&mut w, id);
        assert!(
            !production::projects_for(&w, id).any(|p| p.kind == ProjectKind::MachineryWorks),
            "rescue bypassed {mode}"
        );
        assert!(
            production::projects_for(&w, id).count() <= 2,
            "rescue exceeded queue in {mode}"
        );
        assert_eq!(industry::site_level(&w, &d, ProjectKind::MachineryWorks), 0);
        assert_eq!(industry::snapshot(&w, id).goods.capital_goods, 0.0);
        if mode == "pc" {
            assert_eq!(w.nation(id).political_capital, 0.0);
        }
    }
}
