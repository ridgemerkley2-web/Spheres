use spheres_sim::{
    apply_command, arsenal, clock, economic_ai,
    industry::{self, MineFunding, ProjectFunding},
    init::world_1990,
    load, logistics,
    production::{self, Priority, Project, ProjectKind as K, ProjectStatus},
    programs,
    resources::{self, Commodity, Contract, Leg, MineProject, ShipmentSource},
    save,
    world::{GameRules, NationId, WorldState},
    Command,
};

fn raw_world(physical: bool) -> WorldState {
    let mut w = world_1990(GameRules {
        daily_simulation: true,
        economic_competition: true,
        production_system: true,
        resource_gates: true,
        resource_market: true,
        manufacturing_system: true,
        physical_logistics: physical,
        logistics_routes: physical,
        ..GameRules::default()
    });
    resources::tick(&mut w);
    w
}

fn set_raw(w: &mut WorldState, nation: NationId, commodity: Commodity, quantity: f64) {
    let stocks = &mut w.resources.market.as_mut().unwrap().stocks;
    match stocks.binary_search_by_key(&(nation, commodity), |row| {
        (row.nation, row.commodity)
    }) {
        Ok(index) => stocks[index].quantity = quantity,
        Err(index) => stocks.insert(
            index,
            resources::Stock {
                nation,
                commodity,
                quantity,
                reserve_target: 0.0,
            },
        ),
    }
}

fn fund_programs(w: &mut WorldState, nation: NationId) {
    w.player = Some(nation);
    w.nation_mut(nation).political_capital = 1_000.0;
    let fiscal_year = w.year;
    let allocations = w.nation(nation).budget_for(fiscal_year).allocations;
    apply_command(
        w,
        &Command::SetProgramBudget {
            nation,
            fiscal_year,
            allocations,
            departments: programs::default_departments(),
        },
    )
    .expect("fund test programmes");
}

fn isolated_raw_project(
    id: u32,
    nation: NationId,
    district: String,
    kind: K,
    priority: Priority,
    progress_days: f64,
    remaining_iron: f64,
) -> Project {
    let spec = production::catalog(kind);
    let mut resources_used = spec.recipe;
    resources_used[Commodity::Iron.idx()] =
        (spec.recipe[Commodity::Iron.idx()] - remaining_iron).max(0.0);
    Project {
        id,
        nation,
        district,
        kind,
        priority,
        status: ProjectStatus::Building,
        reason: None,
        progress_days,
        total_days: spec.total_days,
        resources_used,
        capacity_micros: None,
        started_day: None,
    }
}

#[test]
fn cold_raw_forecast_matches_warmed_have_and_is_byte_pure() {
    let cold = world_1990(GameRules {
        daily_simulation: false,
        economic_competition: false,
        production_system: true,
        resource_gates: false,
        resource_market: false,
        ..GameRules::default()
    });
    let before = save(&cold);
    let cold_view = economic_ai::raw_supply_forecast(&cold, NationId::USA);
    assert_eq!(save(&cold), before, "a cold report may not warm persisted state");

    let mut warm = cold.clone();
    resources::tick(&mut warm);
    let warm_view = economic_ai::raw_supply_forecast(&warm, NationId::USA);
    assert_eq!(cold_view, warm_view);
    assert_eq!(cold_view.lines.len(), 12);
    for (index, row) in cold_view.lines.iter().enumerate() {
        assert_eq!(row.commodity, resources::ALL[index]);
    }
    let oil = &cold_view.lines[Commodity::Oil.idx()];
    assert_eq!(oil.status, "informational");
    assert_eq!(oil.stock, 0.0);
    assert_eq!(oil.demand, [0.0; 3]);
    assert_eq!(oil.coverage, [0.0; 3]);
}

#[test]
fn recurring_contracts_share_one_opening_stockpile_once() {
    let mut w = raw_world(false);
    let seller = NationId::USA;
    let buyers = [NationId::Canada, NationId::Mexico];
    let commodity = Commodity::Bauxite;
    assert_eq!(resources::flow(&w, seller, commodity), 0.0);
    set_raw(&mut w, seller, commodity, 10.0);
    for (offset, buyer) in buyers.into_iter().enumerate() {
        w.resources.contracts.push(Contract {
            id: 900 + offset as u32,
            from: seller,
            to: buyer,
            give: vec![Leg::Commodity {
                c: commodity,
                per_month: 1_000.0,
            }],
            take: vec![],
            months_left: 1,
            months_total: 1,
            days_left: Some(1),
            since: resources::month_abs(&w),
            depth: 0.0,
        });
    }
    let before = save(&w);
    let projection = resources::contract_supply_forecast(&w, [30, 90, 365]);
    assert_eq!(save(&w), before, "contract projection is a pure read");
    let out = projection.outbound[seller.index()][commodity.idx()][0];
    let a = projection.inbound[NationId::Canada.index()][commodity.idx()][0];
    let b = projection.inbound[NationId::Mexico.index()][commodity.idx()][0];
    assert!((out - 10.0).abs() < 1e-8, "out={out}");
    assert!((a - 5.0).abs() < 1e-8, "a={a}");
    assert!((b - 5.0).abs() < 1e-8, "b={b}");
    assert!((a + b - out).abs() < 1e-8);
}

#[test]
fn intermediary_exports_are_claimed_once_from_the_combined_inbound_pool() {
    let mut w = raw_world(false);
    let origin = NationId::Canada;
    let intermediary = NationId::USA;
    let destination = NationId::Mexico;
    let commodity = Commodity::Bauxite;
    assert_eq!(resources::flow(&w, intermediary, commodity), 0.0);
    set_raw(&mut w, origin, commodity, 100.0);
    set_raw(&mut w, intermediary, commodity, 0.0);
    for (id, from, to) in [
        (920, origin, intermediary),
        (921, intermediary, destination),
    ] {
        w.resources.contracts.push(Contract {
            id,
            from,
            to,
            give: vec![Leg::Commodity {
                c: commodity,
                per_month: 30.0,
            }],
            take: vec![],
            months_left: 1,
            months_total: 1,
            days_left: Some(30),
            since: resources::month_abs(&w),
            depth: 0.0,
        });
    }
    let projection = resources::contract_supply_forecast(&w, [30, 90, 365]);
    let gross_in = projection.inbound[intermediary.index()][commodity.idx()][0];
    let gross_out = projection.outbound[intermediary.index()][commodity.idx()][0];
    assert!(gross_in > 0.0 && gross_out > 0.0, "in={gross_in} out={gross_out}");
    assert!(gross_out < gross_in, "one-day atomic lag leaves one retained slice");

    let row = &economic_ai::raw_supply_forecast(&w, intermediary).lines[commodity.idx()];
    let displayed = row.allocable_stock[0]
        + row.domestic_coverage[0]
        + row.pending[0]
        + row.contract_coverage[0];
    assert!((displayed - row.coverage[0]).abs() < 1e-8);
    assert!((row.prior_claims[0] - gross_out).abs() < 1e-8);
    assert!((row.contract_coverage[0] - (gross_in - gross_out)).abs() < 1e-8);
    assert!((row.coverage[0] - (gross_in - gross_out)).abs() < 1e-8);
}

#[test]
fn physical_intermediary_waits_for_arrival_then_reexports_once() {
    let mut w = raw_world(true);
    let origin = NationId::Canada;
    let intermediary = NationId::USA;
    let destination = NationId::Mexico;
    let commodity = Commodity::Bauxite;
    assert_eq!(resources::flow(&w, intermediary, commodity), 0.0);
    set_raw(&mut w, origin, commodity, 100.0);
    set_raw(&mut w, intermediary, commodity, 0.0);
    let upstream_travel = logistics::plan(&w, origin, intermediary)
        .unwrap()
        .estimated_days
        .max(1) as i32;
    let downstream_travel = logistics::plan(&w, intermediary, destination)
        .unwrap()
        .estimated_days
        .max(1) as i32;
    for (id, from, to) in [
        (925, origin, intermediary),
        (926, intermediary, destination),
    ] {
        w.resources.contracts.push(Contract {
            id,
            from,
            to,
            give: vec![Leg::Commodity {
                c: commodity,
                per_month: 1.0,
            }],
            take: vec![],
            months_left: 1,
            months_total: 1,
            days_left: Some(30),
            since: resources::month_abs(&w),
            depth: 0.0,
        });
    }

    let dispatch_boundary = resources::contract_supply_forecast(
        &w,
        [upstream_travel, upstream_travel + 1, 365],
    );
    assert_eq!(
        dispatch_boundary.outbound[intermediary.index()][commodity.idx()][0],
        0.0,
        "the intermediary cannot export before upstream freight arrives"
    );
    assert!(
        dispatch_boundary.outbound[intermediary.index()][commodity.idx()][1] > 0.0,
        "a due arrival is credited before that date's contract dispatch"
    );
    let receipt_boundary = resources::contract_supply_forecast(
        &w,
        [
            upstream_travel + downstream_travel,
            upstream_travel + downstream_travel + 1,
            365,
        ],
    );
    assert_eq!(
        receipt_boundary.inbound[destination.index()][commodity.idx()][0],
        0.0
    );
    assert!(receipt_boundary.inbound[destination.index()][commodity.idx()][1] > 0.0);

    let gross_in = receipt_boundary.inbound[intermediary.index()][commodity.idx()][2];
    let gross_out = receipt_boundary.outbound[intermediary.index()][commodity.idx()][2];
    let destination_in = receipt_boundary.inbound[destination.index()][commodity.idx()][2];
    assert!((gross_out - destination_in).abs() < 1e-8);
    let row = &economic_ai::raw_supply_forecast(&w, intermediary).lines[commodity.idx()];
    assert!((gross_in - gross_out - row.coverage[2]).abs() < 1e-8);
}

#[test]
fn reciprocal_physical_terms_settle_as_one_atomic_bundle() {
    let mut w = raw_world(true);
    let a = NationId::USA;
    let b = NationId::Canada;
    let give = Commodity::Bauxite;
    let take = Commodity::RareEarths;
    set_raw(&mut w, a, give, 10.0);
    set_raw(&mut w, b, take, 0.0);
    w.resources.contracts.push(Contract {
        id: 930,
        from: a,
        to: b,
        give: vec![Leg::Commodity {
            c: give,
            per_month: 1.0,
        }],
        take: vec![Leg::Commodity {
            c: take,
            per_month: 1.0,
        }],
        months_left: 1,
        months_total: 1,
        days_left: Some(1),
        since: resources::month_abs(&w),
        depth: 0.0,
    });
    let empty_counterleg = resources::contract_supply_forecast(&w, [30, 90, 365]);
    assert_eq!(empty_counterleg.outbound[a.index()][give.idx()][0], 0.0);
    assert_eq!(empty_counterleg.outbound[b.index()][take.idx()][0], 0.0);

    set_raw(&mut w, b, take, 5.0);
    let partial = resources::contract_supply_forecast(&w, [30, 90, 365]);
    let a_out = partial.outbound[a.index()][give.idx()][0];
    let b_out = partial.outbound[b.index()][take.idx()][0];
    assert!(a_out > 0.0, "a_out={a_out}");
    assert!((a_out - b_out).abs() < 1e-8, "a_out={a_out} b_out={b_out}");
}

#[test]
fn one_day_contract_uses_one_day_of_flow_not_a_full_horizon() {
    let mut w = raw_world(false);
    let seller = NationId::USA;
    let buyer = NationId::Canada;
    let commodity = Commodity::Iron;
    set_raw(&mut w, seller, commodity, 0.0);
    let annual = resources::flow(&w, seller, commodity);
    assert!(annual > 0.0);
    w.resources.contracts.push(Contract {
        id: 940,
        from: seller,
        to: buyer,
        give: vec![Leg::Commodity {
            c: commodity,
            per_month: 1.0e12,
        }],
        take: vec![],
        months_left: 1,
        months_total: 1,
        days_left: Some(1),
        since: resources::month_abs(&w),
        depth: 0.0,
    });
    let projection = resources::contract_supply_forecast(&w, [30, 90, 365]);
    let expected = annual
        / 12.0
        / spheres_sim::world::days_in_month(w.year, w.month) as f64;
    let out = projection.outbound[seller.index()][commodity.idx()][2];
    assert!((out - expected).abs() < 1e-8, "out={out} expected={expected}");
    assert_eq!(
        projection.outbound[seller.index()][commodity.idx()][0],
        out
    );
    assert_eq!(
        projection.outbound[seller.index()][commodity.idx()][1],
        out
    );
}

#[test]
fn projected_contract_arrival_respects_transit_boundary_and_capacity() {
    let mut w = raw_world(true);
    let seller = NationId::Canada;
    let buyer = NationId::USA;
    let commodity = Commodity::Iron;
    set_raw(&mut w, seller, commodity, 1.0e12);
    let route = logistics::plan(&w, seller, buyer).expect("Canada-USA freight route");
    let travel = route.estimated_days.max(1) as i32;
    w.resources.contracts.push(Contract {
        id: 950,
        from: seller,
        to: buyer,
        give: vec![Leg::Commodity {
            c: commodity,
            per_month: 1.0e12,
        }],
        take: vec![],
        months_left: 1,
        months_total: 1,
        days_left: Some(1),
        since: resources::month_abs(&w),
        depth: 0.0,
    });
    let projection = resources::contract_supply_forecast(
        &w,
        [travel, travel.saturating_add(1), 365],
    );
    assert_eq!(
        projection.inbound[buyer.index()][commodity.idx()][0],
        0.0,
        "dispatch at offset zero arrives just outside a travel-day horizon"
    );
    let arrived = projection.inbound[buyer.index()][commodity.idx()][1];
    let dispatched = projection.outbound[seller.index()][commodity.idx()][1];
    assert!(arrived > 0.0 && (arrived - dispatched).abs() < 1e-8);
    let uncapped_daily = 1.0e12
        / spheres_sim::world::days_in_month(w.year, w.month) as f64;
    assert!(
        dispatched < uncapped_daily,
        "the physical route bottleneck must cap an oversized leg"
    );
}

#[test]
fn capacity_limited_contract_is_phase_stable_across_month_boundary() {
    let mut w = raw_world(true);
    w.year = 1990;
    w.month = 1;
    w.day = 31;
    resources::tick(&mut w);
    let seller = NationId::Canada;
    let buyer = NationId::USA;
    let commodity = Commodity::Iron;
    set_raw(&mut w, seller, commodity, 1.0e12);
    w.resources.contracts.push(Contract {
        id: 960,
        from: seller,
        to: buyer,
        give: vec![Leg::Commodity {
            c: commodity,
            per_month: 1.0e12,
        }],
        take: vec![],
        months_left: 1,
        months_total: 1,
        days_left: Some(1),
        since: resources::month_abs(&w),
        depth: 0.0,
    });
    let before = resources::contract_supply_forecast(&w, [30, 90, 365]);
    let before_out = before.outbound[seller.index()][commodity.idx()][0];
    assert!(before_out > 0.0);
    clock::advance_date(&mut w);
    let after = resources::contract_supply_forecast(&w, [30, 90, 365]);
    assert_eq!(
        before,
        after,
        "Jan-31 post-settlement and Feb-1 pre-settlement describe the same next dates"
    );
}

#[test]
fn finite_projects_and_mines_cap_each_item_before_summing_horizons() {
    let nation = NationId::USA;
    let iron = Commodity::Iron.idx();

    let mut projects = raw_world(false);
    fund_programs(&mut projects, nation);
    let districts: Vec<_> = projects
        .districts
        .iter()
        .filter(|(_, owner)| **owner == nation)
        .map(|(district, _)| district.clone())
        .take(2)
        .collect();
    assert_eq!(districts.len(), 2);
    let warehouse_days = production::catalog(K::Warehouse).total_days as f64;
    projects.production.projects.push(isolated_raw_project(
        70_001,
        nation,
        districts[0].clone(),
        K::Warehouse,
        Priority::High,
        warehouse_days - 0.1,
        1.0,
    ));
    projects.production.projects.push(isolated_raw_project(
        70_002,
        nation,
        districts[1].clone(),
        K::Generation,
        Priority::Low,
        0.0,
        50.0,
    ));
    projects
        .production
        .industry
        .projects
        .insert(70_001, ProjectFunding::default());
    projects
        .production
        .industry
        .projects
        .insert(70_002, ProjectFunding::default());
    // A deliberately stale execution receipt must not affect tomorrow's share.
    projects.production.industry.work.insert(
        nation,
        (clock::absolute_day(&projects), 999.0, 0.001),
    );
    let project_components = industry::raw_demand_components(&projects, nation);
    let project_daily = project_components.projects_daily[iron];
    assert!(project_daily > 1.0 && project_daily < 51.0, "daily={project_daily}");
    let slow_daily = project_daily - 1.0;
    let expected_30 = 1.0 + (slow_daily * 30.0).min(50.0);
    assert!(
        (project_components.projects_horizon[iron][0] - expected_30).abs() < 1e-8,
        "h30={} expected={expected_30}",
        project_components.projects_horizon[iron][0]
    );
    let pooled_30 = (project_daily * 30.0).min(51.0);
    assert!(project_components.projects_horizon[iron][0] + 1e-8 < pooled_30);
    assert!(project_components.projects_horizon[iron][0]
        <= project_components.projects_horizon[iron][1]);
    assert!(project_components.projects_horizon[iron][1]
        <= project_components.projects_horizon[iron][2]);
    assert!(project_components.projects_horizon[iron][2] <= 51.0 + 1e-8);
    let mut advanced = projects.clone();
    clock::advance_date(&mut advanced);
    assert_eq!(
        project_components.projects_horizon,
        industry::raw_demand_components(&advanced, nation).projects_horizon,
        "the same next unsettled dates are phase-stable"
    );

    let mut mines = raw_world(false);
    fund_programs(&mut mines, nation);
    let districts: Vec<_> = mines
        .districts
        .iter()
        .filter(|(_, owner)| **owner == nation)
        .map(|(district, _)| district.clone())
        .take(2)
        .collect();
    for (id, district, progress, iron_used) in [
        (71_001, districts[0].clone(), 364.9, 19.0),
        (71_002, districts[1].clone(), 0.0, 0.0),
    ] {
        mines.resources.mine_projects.push(MineProject {
            district: district.clone(),
            commodity: Commodity::Iron,
            started_by: nation,
            months_left: 12,
            months_total: 12,
            days_left: Some(365),
            investment_bn: 1.0,
            output: id as f64,
        });
        let mut used = [0.0; 12];
        used[Commodity::Iron.idx()] = iron_used;
        used[Commodity::Copper.idx()] = 4.0;
        used[Commodity::Coal.idx()] = 8.0;
        mines.production.industry.mines.insert(
            industry::mine_key(&district, Commodity::Iron),
            MineFunding {
                progress_days: progress,
                total_days: 365,
                spent_bn: 0.0,
                resources_used: used,
                last_day: Some(clock::absolute_day(&mines)),
                reason: None,
            },
        );
    }
    let mine_components = industry::raw_demand_components(&mines, nation);
    let mine_daily = mine_components.mines_daily[iron];
    assert!(mine_daily > 1.0 && mine_daily < 21.0, "daily={mine_daily}");
    let expected_30 = 1.0 + ((mine_daily - 1.0) * 30.0).min(20.0);
    assert!(
        (mine_components.mines_horizon[iron][0] - expected_30).abs() < 1e-8,
        "h30={} expected={expected_30}",
        mine_components.mines_horizon[iron][0]
    );
    assert!(mine_components.mines_horizon[iron][0]
        <= mine_components.mines_horizon[iron][1]);
    assert!(mine_components.mines_horizon[iron][1]
        <= mine_components.mines_horizon[iron][2]);
    assert!(mine_components.mines_horizon[iron][2] <= 21.0 + 1e-8);
}

#[test]
fn expiring_fiscal_authority_stops_at_the_year_boundary() {
    let nation = NationId::USA;
    let iron = Commodity::Iron.idx();
    let bauxite = Commodity::Bauxite.idx();
    let mut w = raw_world(false);
    // Keep the legacy one-line arsenal path so its standing raw recipe is
    // visible beside the enrolled civilian operating recipe in this fixture.
    w.rules.manufacturing_system = false;
    w.year = 1990;
    w.month = 12;
    w.day = 31;
    fund_programs(&mut w, nation);
    let district = w
        .districts
        .iter()
        .find_map(|(district, owner)| (*owner == nation).then(|| district.clone()))
        .unwrap();
    w.production
        .industry
        .sites
        .insert(district.clone(), [0, 2, 1, 0, 0, 0, 0]);
    let kit = arsenal::index_of("f15e").expect("F-15E programme exists");
    let tech = spheres_sim::tech::index_of(
        arsenal::DECK[kit as usize]
            .tech
            .expect("F-15E has a technology"),
    )
    .expect("F-15E technology exists");
    if let Err(index) = w.nation(nation).tech.known.binary_search(&tech) {
        w.nation_mut(nation).tech.known.insert(index, tech);
    }
    w.nation_mut(nation).arsenal.preference = Some("f15e".into());
    w.nation_mut(nation).mil_spend_gdp = 0.05;
    w.production.projects.push(isolated_raw_project(
        72_001,
        nation,
        district,
        K::Warehouse,
        Priority::Normal,
        0.0,
        20.0,
    ));
    w.production
        .industry
        .projects
        .insert(72_001, ProjectFunding::default());
    w.resources.mine_projects.push(MineProject {
        district: w.production.projects[0].district.clone(),
        commodity: Commodity::Iron,
        started_by: nation,
        months_left: 12,
        months_total: 12,
        days_left: Some(365),
        investment_bn: 1.0,
        output: 1.0,
    });
    w.production.industry.mines.insert(
        industry::mine_key(&w.production.projects[0].district, Commodity::Iron),
        MineFunding {
            progress_days: 0.0,
            total_days: 365,
            spent_bn: 0.0,
            resources_used: [0.0; 12],
            last_day: None,
            reason: None,
        },
    );
    assert_eq!(resources::forecast_start_day(&w), clock::absolute_day(&w));
    let before = industry::raw_demand_components(&w, nation);
    assert!(before.projects_daily[iron] > 0.0);
    assert!(
        before.projects_horizon[iron]
            .iter()
            .all(|amount| (*amount - before.projects_daily[iron]).abs() < 1e-8),
        "only the one remaining 1990 date is funded: {:?}",
        before.projects_horizon[iron]
    );
    assert!(before.mines_daily[iron] > 0.0);
    assert!(before.mines_horizon[iron]
        .iter()
        .all(|amount| (*amount - before.mines_daily[iron]).abs() < 1e-8));
    let forecast_before = economic_ai::raw_supply_forecast(&w, nation);
    let recurring_before = &forecast_before.lines[bauxite];
    assert!(recurring_before.civilian_operating_daily > 0.0);
    assert!(recurring_before.military_recurring_monthly > 0.0);
    let one_funded_day = recurring_before.civilian_operating_daily
        + recurring_before.military_recurring_monthly * 12.0 / 365.0;
    assert!(recurring_before
        .demand
        .iter()
        .all(|amount| (*amount - one_funded_day).abs() < 3e-8),
        "civilian and military recurring draws stop with 1990 authority: {:?} expected={one_funded_day}",
        recurring_before.demand);
    clock::advance_date(&mut w);
    let after = industry::raw_demand_components(&w, nation);
    assert_eq!(after.projects_horizon[iron], [0.0; 3]);
    assert_eq!(after.mines_horizon[iron], [0.0; 3]);
    let forecast_after = economic_ai::raw_supply_forecast(&w, nation);
    assert_eq!(forecast_after.lines[bauxite].demand, [0.0; 3]);

    let mut legacy = w.clone();
    legacy.nation_mut(nation).program_budget = None;
    let legacy_forecast = economic_ai::raw_supply_forecast(&legacy, nation);
    let legacy_line = &legacy_forecast.lines[bauxite];
    assert!(legacy_line.military_recurring_monthly > 0.0);
    assert!(legacy_line.demand.iter().zip([30.0, 90.0, 365.0]).all(
        |(amount, days)| {
            (*amount - legacy_line.military_recurring_monthly * days * 12.0 / 365.0)
                .abs()
                < 3e-8
        }
    ));
}

#[test]
fn pending_window_is_phase_stable_and_has_an_exclusive_end() {
    let mut w = raw_world(true);
    let seller = NationId::Canada;
    let buyer = NationId::USA;
    let commodity = Commodity::Iron;
    let route = logistics::plan(&w, seller, buyer).expect("Canada-USA freight route");
    let settled = clock::absolute_day(&w);
    for (id, due) in [(1, settled + 29), (2, settled + 30), (3, settled + 31)] {
        w.logistics.cargo.push(logistics::Cargo {
            id,
            seller,
            buyer,
            commodity,
            quantity: 1.0,
            source: ShipmentSource::Spot,
            contract: None,
            route: route.clone(),
            dispatched_month: resources::month_abs(&w),
            due_month: resources::month_abs(&w) + 1,
            dispatched_day: Some(settled),
            due_day: Some(due),
            hold_reason: None,
        });
    }
    assert_eq!(resources::forecast_start_day(&w), settled + 1);
    assert_eq!(logistics::pending_within_days(&w, buyer, commodity, 30), 2.0);
    clock::advance_date(&mut w);
    assert_eq!(resources::forecast_start_day(&w), settled + 1);
    assert_eq!(logistics::pending_within_days(&w, buyer, commodity, 30), 2.0);
    w.sanctions.push((seller, buyer));
    assert_eq!(logistics::pending_within_days(&w, buyer, commodity, 30), 0.0);
    w.sanctions.clear();
    w.logistics.cargo[0].hold_reason = Some("stale closure receipt".into());
    assert_eq!(
        logistics::pending_within_days(&w, buyer, commodity, 30),
        2.0,
        "a reopened current route supersedes a stale held flag"
    );
}

#[test]
fn exact_success_after_a_real_arsenal_block_clears_red_even_at_zero_stock() {
    let mut w = raw_world(false);
    w.rules.manufacturing_system = false;
    let nation = NationId::USA;
    let commodity = Commodity::Bauxite;
    let kit = arsenal::index_of("f15e").expect("F-15E programme exists");
    let tech = spheres_sim::tech::index_of(
        arsenal::DECK[kit as usize]
            .tech
            .expect("F-15E has a technology"),
    )
    .expect("F-15E technology exists");
    if let Err(index) = w.nation(nation).tech.known.binary_search(&tech) {
        w.nation_mut(nation).tech.known.insert(index, tech);
    }
    w.nation_mut(nation).arsenal.preference = Some("f15e".into());
    w.nation_mut(nation).mil_spend_gdp = 0.05;
    w.nation_mut(nation).program_budget = None;
    w.nation_mut(nation).arsenal.banked = 1.0;
    assert_eq!(resources::flow(&w, nation, commodity), 0.0);
    for producer in resources::producers(&w, commodity) {
        if producer != nation {
            w.sanctions.push((producer, nation));
        }
    }
    set_raw(&mut w, nation, commodity, 0.0);
    let initial_draw = resources::tick_draw(&w, nation)[commodity.idx()];
    assert!(
        initial_draw > 0.0,
        "fixture needs a bauxite line: gdp={} mil={} bank={} program={:?}",
        w.nation(nation).gdp,
        w.nation(nation).mil_spend_gdp,
        w.nation(nation).arsenal.banked,
        w.nation(nation).program_budget
    );
    arsenal::tick(&mut w);
    let blocked = economic_ai::raw_supply_forecast(&w, nation);
    let row = &blocked.lines[commodity.idx()];
    assert!(
        row.blocked_now,
        "the actual failed gate must be red; receipt={:?}; draw={:?}; stock={}; headlines={:?}",
        w.nation(nation).arsenal.last_resource_stall,
        resources::tick_draw(&w, nation),
        resources::stockpile(&w, nation, commodity),
        w.headlines
    );
    assert!(row.blocker_reason.as_deref().is_some_and(|reason| {
        reason.to_ascii_lowercase().contains("bauxite")
    }));

    clock::advance_date(&mut w);
    resources::tick(&mut w);
    w.nation_mut(nation).arsenal.banked = 1.0;
    let exact = resources::tick_draw(&w, nation)[commodity.idx()];
    assert!(exact > 0.0);
    set_raw(&mut w, nation, commodity, exact);
    arsenal::tick(&mut w);
    assert!(resources::stockpile(&w, nation, commodity).abs() < 1e-8);
    let success = economic_ai::raw_supply_forecast(&w, nation);
    let row = &success.lines[commodity.idx()];
    assert!(!row.blocked_now, "successful last-unit use is not a block");
    assert_eq!(row.immediate_shortage, 0.0);
    assert!(row.blocker_reason.is_none());

    let loaded = load(&save(&w)).expect("structured outcome survives save/load");
    let row = &economic_ai::raw_supply_forecast(&loaded, nation).lines[commodity.idx()];
    assert!(!row.blocked_now);
    assert!(row.blocker_reason.is_none());
}
