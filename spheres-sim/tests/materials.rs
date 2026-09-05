//! Physical invariants for the Materials pilot, not statistical calibration.
//! Test-only stocks and generators below are explicit paid-asset fixtures;
//! fresh campaigns receive neither from the historical estimate.
use spheres_sim::{
    apply_command, clock,
    commerce::{self, Good},
    industry,
    init::world_1990,
    load, materials, production, programs, province_economy,
    resources::{self, Commodity as C, ALL},
    save, starting_industry,
    world::{start_nations, GameRules, NationId, WorldState, BUDGET_INDUSTRY},
    Command,
};

fn world() -> WorldState {
    let mut w = world_1990(GameRules {
        daily_simulation: true,
        economic_competition: true,
        production_system: true,
        resource_market: true,
        resource_gates: true,
        physical_logistics: true,
        logistics_routes: true,
        ..Default::default()
    });
    starting_industry::enable_new_world(&mut w).unwrap();
    province_economy::enable(&mut w);
    w.conflicts.clear();
    w
}
fn enroll(w: &mut WorldState, nation: NationId) {
    w.nation_mut(nation).political_capital = 1000.0;
    let allocations = w.nation(nation).budget_for(w.year).allocations;
    apply_command(
        w,
        &Command::SetProgramBudget {
            nation,
            fiscal_year: w.year,
            allocations,
            departments: programs::default_departments(),
        },
    )
    .unwrap();
    programs::begin_day(w);
}
fn districts(w: &WorldState, nation: NationId) -> Vec<String> {
    w.districts
        .iter()
        .filter(|(_, n)| **n == nation)
        .map(|(d, _)| d.clone())
        .collect()
}
fn stock(w: &mut WorldState, nation: NationId, c: C, amount: f64) {
    if w.resources.market.is_none() {
        resources::tick(w);
    }
    let rows = &mut w.resources.market.as_mut().unwrap().stocks;
    if let Some(r) = rows
        .iter_mut()
        .find(|r| r.nation == nation && r.commodity == c)
    {
        r.quantity = amount;
    } else {
        rows.push(resources::Stock {
            nation,
            commodity: c,
            quantity: amount,
            reserve_target: 0.0,
        });
    }
    rows.sort_by_key(|r| (r.nation, r.commodity));
}
fn power(w: &mut WorldState, d: &str, generators: u8) {
    w.production
        .provinces
        .push(production::ProvinceCapabilities {
            district: d.into(),
            civilian_industry: 1,
            power_grid: 1,
            infrastructure: 0,
            research_centers: 0,
            arms_plants: 0,
        });
    if generators > 0 {
        w.production
            .industry
            .sites
            .insert(d.into(), [0, generators, 0, 0, 0, 0, 0]);
    }
}
fn prepared() -> (WorldState, String) {
    let mut w = world();
    enroll(&mut w, NationId::USA);
    let d = districts(&w, NationId::USA)[0].clone();
    power(&mut w, &d, 1);
    for c in ALL {
        stock(&mut w, NationId::USA, c, 10000.0);
    }
    (w, d)
}
fn next(w: &mut WorldState) {
    programs::finish_day(w);
    clock::advance_date(w);
    programs::begin_day(w);
}
fn close(a: f64, b: f64) {
    assert!(
        (a - b).abs() <= 1e-9 * a.abs().max(b.abs()).max(1.0),
        "{a} != {b}"
    );
}

#[test]
fn unactivated_default_and_new_campaign_are_inert_and_quotes_are_pure() {
    for daily in [false, true] {
        let mut w = world_1990(GameRules {
            daily_simulation: daily,
            ..Default::default()
        });
        let before = save(&w);
        industry::tick_day(&mut w);
        assert_eq!(save(&w), before);
        assert!(w.materials.is_none());
    }
    let mut w = world();
    let d = districts(&w, NationId::USA)[0].clone();
    let before = save(&w);
    let q = materials::quote(&w, NationId::USA, &d, 30.0, 30);
    assert!(!q.eligible);
    assert!(q.refusal.unwrap().contains("budget"));
    let s = materials::snapshot(&w, NationId::USA).unwrap();
    assert_eq!(s.stock, 0.0);
    assert!(s.capacity_daily > 0.0);
    industry::tick_day(&mut w);
    assert_eq!(save(&w), before);
}

#[test]
fn bad_quantities_durations_identity_and_capacity_refuse_without_mutation() {
    let (mut w, d) = prepared();
    for quantity in [
        0.0,
        -1.0,
        f64::NAN,
        f64::INFINITY,
        f64::MAX,
        0.0000000001,
        materials::MAX_ORDER_QUANTITY + 1.0,
    ] {
        let before = save(&w);
        assert!(materials::start_order(&mut w, NationId::USA, &d, quantity, 30).is_err());
        assert_eq!(save(&w), before);
        let q = materials::quote(&w, NationId::USA, &d, quantity, 30);
        assert!(!q.eligible);
        assert!(q.reserved_daily.is_finite());
    }
    for days in [0, 6, 366, u32::MAX] {
        assert!(materials::order_refusal(&w, NationId::USA, &d, 1.0, days).is_some());
    }
    assert!(materials::order_refusal(&w, NationId::Canada, &d, 1.0, 30).is_some());
    let quantity = materials::capacity_daily(&w, &d) * 31.0;
    assert!(materials::order_refusal(&w, NationId::USA, &d, quantity, 30).is_some());
    materials::start_order(&mut w, NationId::USA, &d, 1.0, 30).unwrap();
    assert!(materials::order_refusal(&w, NationId::USA, &d, 1.0, 30)
        .unwrap()
        .contains("already"));
}

#[test]
fn missing_component_spends_nothing_and_does_not_grant_free_goods() {
    let (mut w, d) = prepared();
    stock(&mut w, NationId::USA, C::Bauxite, 0.0);
    let id = materials::start_order(&mut w, NationId::USA, &d, 7.0, 7).unwrap();
    let resources = w.resources.clone();
    let budget = w.nation(NationId::USA).program_budget.clone();
    let gdp = w.nation(NationId::USA).gdp;
    industry::tick_day(&mut w);
    assert_eq!(w.resources, resources);
    assert_eq!(w.nation(NationId::USA).program_budget, budget);
    assert_eq!(commerce::stock(&w, NationId::USA, Good::Intermediates), 0.0);
    assert_eq!(w.nation(NationId::USA).gdp, gdp);
    let o = &w.materials.as_ref().unwrap().orders[0];
    assert_eq!(o.id, id);
    assert_eq!(o.status, "paused");
    assert!(o.reason.as_ref().unwrap().contains(C::Bauxite.name()));
    assert_eq!(materials::pending(&w, NationId::USA), 7.0);
}

#[test]
fn paid_delivery_conserves_inputs_operating_authority_and_exact_finite_quantity() {
    let (mut w, d) = prepared();
    let total = 1.000000001;
    materials::start_order(&mut w, NationId::USA, &d, total, 7).unwrap();
    let before = ALL.map(|c| resources::stockpile(&w, NationId::USA, c));
    let treasury = w.nation(NationId::USA).treasury_bn;
    industry::tick_day(&mut w);
    let o = &w.materials.as_ref().unwrap().orders[0];
    assert!(o.delivered > 0.0);
    close(o.spent_conversion_bn, o.delivered * 0.00001);
    close(o.spent_energy_bn, o.delivered * 0.000002);
    for c in ALL {
        close(
            before[c.idx()] - resources::stockpile(&w, NationId::USA, c),
            o.raw_used[c.idx()],
        );
    }
    let p = w.nation(NationId::USA).program_budget.as_ref().unwrap();
    close(
        p.noncapital_spent_today_bn[BUDGET_INDUSTRY][2],
        o.spent_conversion_bn,
    );
    close(
        p.noncapital_spent_today_bn[BUDGET_INDUSTRY][1],
        o.spent_energy_bn,
    );
    assert_eq!(
        w.nation(NationId::USA).treasury_bn,
        treasury,
        "fiscal settlement owns treasury, never producer"
    );
    let once = save(&w);
    industry::tick_day(&mut w);
    assert_eq!(save(&w), once);
    for _ in 1..7 {
        next(&mut w);
        industry::tick_day(&mut w);
    }
    let o = &w.materials.as_ref().unwrap().orders[0];
    assert_eq!(o.status, "completed");
    assert_eq!(o.remaining, 0.0);
    assert_eq!(o.delivered, total);
    close(
        commerce::stock(&w, NationId::USA, Good::Intermediates),
        total,
    );
    close(o.spent_conversion_bn, total * 0.00001);
    close(
        w.materials.as_ref().unwrap().accounts[&NationId::USA].delivered,
        total,
    );
    assert_eq!(materials::pending(&w, NationId::USA), 0.0);
}

#[test]
fn funded_factories_and_contracts_share_exact_same_generation_grid_and_cash() {
    let (mut w, d) = prepared();
    let second = districts(&w, NationId::USA)[1].clone();
    let third = districts(&w, NationId::USA)[2].clone();
    power(&mut w, &second, 0);
    power(&mut w, &third, 0);
    // Industry::EXTENDED order: machinery, generation, processing, freight,
    // warehouse, automation, efficiency. Existing processing+machinery use2.
    w.production
        .industry
        .sites
        .insert(d.clone(), [1, 1, 1, 0, 0, 0, 0]);
    materials::start_order(&mut w, NationId::USA, &d, 56.0, 7).unwrap();
    materials::start_order(&mut w, NationId::USA, &second, 56.0, 7).unwrap();
    materials::start_order(&mut w, NationId::USA, &third, 14.0, 7).unwrap();
    industry::tick_day(&mut w);
    let m = w.materials.as_ref().unwrap();
    close(m.orders[0].output_today, 3.0);
    close(m.orders[1].output_today, 5.0);
    assert_eq!(m.orders[2].output_today, 0.0);
    assert_eq!(m.orders[2].status, "paused");
    close(industry::snapshot(&w, NationId::USA).power_used_daily, 10.0);
    close(commerce::stock(&w, NationId::USA, Good::Intermediates), 8.5);
    close(commerce::stock(&w, NationId::USA, Good::CapitalGoods), 0.5);
    let p = w.nation(NationId::USA).program_budget.as_ref().unwrap();
    close(
        p.noncapital_spent_today_bn[BUDGET_INDUSTRY][2],
        9.0 * 0.00001,
    );
    close(
        p.noncapital_spent_today_bn[BUDGET_INDUSTRY][1],
        10.0 * 0.000002,
    );
    assert!(m.orders[..2].iter().all(|o| o.status == "limited"));
}

#[test]
fn expired_cancelled_lost_provinces_and_full_storage_never_spend_or_transfer_orders() {
    let (mut w, d) = prepared();
    let id = materials::start_order(&mut w, NationId::USA, &d, 7.0, 7).unwrap();
    w.production
        .industry
        .goods
        .entry(NationId::USA)
        .or_default()
        .intermediates = industry::goods_capacity(&w, NationId::USA);
    industry::tick_day(&mut w);
    assert_eq!(w.materials.as_ref().unwrap().orders[0].status, "paused");
    for _ in 0..7 {
        next(&mut w);
        industry::tick_day(&mut w);
    }
    assert_eq!(w.materials.as_ref().unwrap().orders[0].status, "expired");
    assert_eq!(w.materials.as_ref().unwrap().orders[0].delivered, 0.0);
    assert!(materials::cancel_order(&mut w, NationId::USA, id).is_err());
    let id = materials::start_order(&mut w, NationId::USA, &d, 7.0, 7).unwrap();
    assert!(materials::cancel_order(&mut w, NationId::Canada, id).is_err());
    materials::cancel_order(&mut w, NationId::USA, id).unwrap();
    let third = materials::start_order(&mut w, NationId::USA, &d, 7.0, 7).unwrap();
    w.districts.insert(d.clone(), NationId::Canada);
    assert_eq!(materials::pending(&w, NationId::USA), 0.0);
    next(&mut w);
    industry::tick_day(&mut w);
    let o = w
        .materials
        .as_ref()
        .unwrap()
        .orders
        .iter()
        .find(|o| o.id == third)
        .unwrap();
    assert_eq!(o.status, "cancelled");
    assert_eq!(o.nation, NationId::USA);
    assert_eq!(o.delivered, 0.0);
}

#[test]
fn save_resume_and_readonly_snapshots_preserve_daily_deliveries_and_reservations() {
    let (mut w, d) = prepared();
    materials::start_order(&mut w, NationId::USA, &d, 7.0, 7).unwrap();
    industry::tick_day(&mut w);
    next(&mut w);
    let serial = save(&w);
    let mut restored = load(&serial).unwrap();
    let view = materials::snapshot(&w, NationId::USA).unwrap();
    assert_eq!(view.output_daily, 1.0);
    assert_eq!(view.as_of_day, Some(clock::absolute_day(&w) - 1));
    materials::quote(&w, NationId::USA, &d, 7.0, 7);
    assert_eq!(save(&w), serial);
    for _ in 0..10 {
        industry::tick_day(&mut w);
        industry::tick_day(&mut restored);
        assert_eq!(save(&w), save(&restored));
        next(&mut w);
        next(&mut restored);
    }
}

#[test]
fn all_137_countries_have_honest_capacity_quotes_without_reseeding_or_free_inventory() {
    let mut w = world();
    for &nation in start_nations() {
        enroll(&mut w, nation);
    }
    let before = save(&w);
    for &nation in start_nations() {
        let s = materials::snapshot(&w, nation).unwrap();
        assert!(
            s.capacity_daily.is_finite() && s.capacity_daily >= 0.0,
            "{}",
            nation.name()
        );
        assert_eq!(s.stock, 0.0);
        assert!(s.orders.is_empty());
        for p in &s.provinces {
            assert!(p.capacity_daily.is_finite());
            assert!(p.quote.conversion_total_bn.is_finite());
            assert_eq!(p.quote.quantity, p.recommended_quantity);
        }
        if s.provinces.is_empty() {
            assert_eq!(s.status, "unmapped");
            assert_eq!(s.capacity_daily, 0.0);
        }
    }
    assert_eq!(save(&w), before);
    assert!(w.materials.is_none());
}

#[test]
fn tiny_country_fractional_delivery_is_finite_and_below_its_provincial_capacity() {
    let mut w = world();
    let nation = NationId::Tonga;
    enroll(&mut w, nation);
    let d = districts(&w, nation)[0].clone();
    power(&mut w, &d, 1);
    for c in ALL {
        stock(&mut w, nation, c, 100.0);
    }
    let quantity = materials::recommended_quantity(&w, nation, &d);
    assert!(quantity > 0.0);
    materials::start_order(&mut w, nation, &d, quantity, 30).unwrap();
    industry::tick_day(&mut w);
    let o = &w.materials.as_ref().unwrap().orders[0];
    assert!(o.delivered > 0.0);
    assert!(o.delivered <= materials::capacity_daily(&w, &d));
    assert!(o.spent_conversion_bn.is_finite());
}

#[test]
fn resource_forecast_does_not_increase_initial_stockpile_cover() {
    let mut w = world();
    enroll(&mut w, NationId::USA);
    let d = districts(&w, NationId::USA)[0].clone();
    let before = ALL.map(|c| resources::stockpile(&w, NationId::USA, c));
    let demand = industry::resource_demand_daily(&w, NationId::USA);
    materials::start_order(&mut w, NationId::USA, &d, 30.0, 30).unwrap();
    assert!(
        industry::resource_demand_daily(&w, NationId::USA)[C::Iron.idx()] > demand[C::Iron.idx()]
    );
    assert_eq!(
        ALL.map(|c| resources::stockpile(&w, NationId::USA, c)),
        before
    );
    assert!(w.resources.market.is_none());
}

#[test]
fn orders_without_any_power_assets_still_expire_and_history_pruning_keeps_receipts() {
    let mut w = world();
    enroll(&mut w, NationId::USA);
    let d = districts(&w, NationId::USA)[0].clone();
    materials::start_order(&mut w, NationId::USA, &d, 7.0, 7).unwrap();
    assert!(w.production.industry.sites.is_empty());
    for _ in 0..8 {
        industry::tick_day(&mut w);
        next(&mut w);
    }
    assert_eq!(w.materials.as_ref().unwrap().orders[0].status, "expired");
    let mut paid = prepared();
    materials::start_order(&mut paid.0, NationId::USA, &paid.1, 7.0, 7).unwrap();
    industry::tick_day(&mut paid.0);
    materials::cancel_order(&mut paid.0, NationId::USA, 0).unwrap();
    let total = paid.0.materials.as_ref().unwrap().accounts.clone();
    for _ in 0..367 {
        next(&mut paid.0);
        industry::tick_day(&mut paid.0);
    }
    assert!(paid.0.materials.as_ref().unwrap().orders.is_empty());
    assert_eq!(paid.0.materials.as_ref().unwrap().accounts, total);
}

#[test]
fn department_cap_and_id_exhaustion_are_finite_and_zero_authority_is_atomic() {
    let (mut w, d) = prepared();
    materials::start_order(&mut w, NationId::USA, &d, 7.0, 7).unwrap();
    w.nation_mut(NationId::USA)
        .program_budget
        .as_mut()
        .unwrap()
        .available_bn[BUDGET_INDUSTRY][2] = 0.0;
    let before = w.resources.clone();
    industry::tick_day(&mut w);
    assert_eq!(w.resources, before);
    assert_eq!(w.materials.as_ref().unwrap().orders[0].delivered, 0.0);
    assert_eq!(
        w.materials.as_ref().unwrap().orders[0].spent_conversion_bn,
        0.0
    );
    let ds = districts(&w, NationId::USA);
    for d in ds.iter().skip(1).take(materials::MAX_ACTIVE_ORDERS - 1) {
        materials::start_order(&mut w, NationId::USA, d, 7.0, 7).unwrap();
    }
    assert!(
        materials::order_refusal(&w, NationId::USA, &ds[materials::MAX_ACTIVE_ORDERS], 7.0, 7)
            .unwrap()
            .contains("32")
    );
    materials::cancel_order(&mut w, NationId::USA, 0).unwrap();
    w.materials.as_mut().unwrap().next_id = u32::MAX;
    let before = save(&w);
    assert!(materials::start_order(&mut w, NationId::USA, &d, 7.0, 7)
        .unwrap_err()
        .contains("identifiers"));
    assert_eq!(save(&w), before);
}

#[test]
fn contracts_request_real_grid_and_generation_in_the_shared_investment_plan() {
    let (mut w, d) = prepared();
    let before = spheres_sim::industry_planning::plan(&w, NationId::USA);
    materials::start_order(&mut w, NationId::USA, &d, 140.0, 7).unwrap();
    let serial = save(&w);
    let plan = spheres_sim::industry_planning::plan(&w, NationId::USA);
    let province = plan.provinces.iter().find(|p| p.district == d).unwrap();
    close(province.materials_power_required_daily, 20.0);
    close(province.power_required_daily, 20.0);
    close(
        plan.power_required_daily - before.power_required_daily,
        20.0,
    );
    assert_eq!(plan.generation_daily, before.generation_daily);
    assert_eq!(
        plan.generation_committed_daily,
        before.generation_committed_daily
    );
    assert!(province.power_required_daily > province.grid_daily);
    assert!(plan.power_required_daily > plan.generation_daily);
    assert_eq!(save(&w), serial);
    materials::cancel_order(&mut w, NationId::USA, 0).unwrap();
    let released = spheres_sim::industry_planning::plan(&w, NationId::USA);
    close(released.power_required_daily, before.power_required_daily);
}

#[test]
fn materials_forecast_does_not_issue_automatic_raw_purchases_in_actual_spot_clearing() {
    let mut baseline = world();
    baseline.rules.physical_logistics = false;
    baseline.rules.logistics_routes = false;
    enroll(&mut baseline, NationId::USA);
    let d = districts(&baseline, NationId::USA)[0].clone();
    for c in ALL {
        stock(&mut baseline, NationId::USA, c, 10000.0);
    }
    stock(&mut baseline, NationId::USA, C::Bauxite, 0.0);
    stock(&mut baseline, NationId::Canada, C::Bauxite, 1_000_000.0);
    let mut manual = baseline.clone();
    materials::start_order(&mut manual, NationId::USA, &d, 30.0, 30).unwrap();
    assert!(
        resources::draw(&manual, NationId::USA)[C::Bauxite.idx()]
            > resources::draw(&baseline, NationId::USA)[C::Bauxite.idx()]
    );
    // This invokes the REAL automatic spot clearing, not just the isolated
    // industry dispatcher. No power exists, so the Materials order cannot run.
    spheres_sim::arsenal::tick(&mut baseline);
    spheres_sim::arsenal::tick(&mut manual);
    let bought = |w: &WorldState| {
        w.resources
            .market
            .as_ref()
            .unwrap()
            .fills
            .iter()
            .filter(|f| f.buyer == NationId::USA && f.commodity == C::Bauxite)
            .map(|f| f.quantity)
            .sum::<f64>()
    };
    close(bought(&manual), bought(&baseline));
    close(
        resources::spot_imports_bn(&manual, NationId::USA),
        resources::spot_imports_bn(&baseline, NationId::USA),
    );
    assert_eq!(
        manual.nation(NationId::USA).treasury_bn,
        baseline.nation(NationId::USA).treasury_bn
    );

    // Existing funded industrial plants retain their established automatic
    // replenishment behavior; excluding all civilian demand would be wrong.
    let mut funded = world();
    funded.rules.physical_logistics = false;
    funded.rules.logistics_routes = false;
    enroll(&mut funded, NationId::USA);
    for c in ALL {
        stock(&mut funded, NationId::USA, c, 10000.0);
    }
    stock(&mut funded, NationId::USA, C::Bauxite, 0.0);
    stock(&mut funded, NationId::Canada, C::Bauxite, 1_000_000.0);
    funded
        .production
        .industry
        .sites
        .insert(d, [0, 0, 1, 0, 0, 0, 0]);
    spheres_sim::arsenal::tick(&mut funded);
    assert!(bought(&funded) > bought(&baseline));
}

#[test]
fn pending_quantity_tracks_current_physical_capacity_after_efficiency_changes() {
    let (mut w, d) = prepared();
    let before = materials::capacity_daily(&w, &d);
    let quantity = (before * 7.0 * 0.999 * 1e9).floor() / 1e9;
    materials::start_order(&mut w, NationId::USA, &d, quantity, 7).unwrap();
    close(materials::pending(&w, NationId::USA), quantity);
    w.production.industry.sites.get_mut(&d).unwrap()[6] = 1;
    let after = materials::capacity_daily(&w, &d);
    assert!(after < before * 0.999);
    close(materials::pending(&w, NationId::USA), after * 7.0);
    close(
        materials::province_reserved_daily(&w, NationId::USA, &d),
        after,
    );
    let reserve = materials::resource_reserve(&w, NationId::USA);
    close(reserve[C::Iron.idx()], after * 7.0);
}

#[test]
fn full_daily_player_materials_order_never_signs_or_funds_automatic_raw_imports() {
    let mut baseline = world();
    baseline.player = Some(NationId::USA);
    baseline.rules.physical_logistics = false;
    baseline.rules.logistics_routes = false;
    baseline.rules.ai_aggression = 0.0;
    enroll(&mut baseline, NationId::USA);
    let d = districts(&baseline, NationId::USA)[0].clone();
    for c in ALL {
        stock(&mut baseline, NationId::USA, c, 10000.0);
    }
    stock(&mut baseline, NationId::USA, C::Bauxite, 0.0);
    stock(&mut baseline, NationId::Canada, C::Bauxite, 1_000_000.0);
    let mut manual = baseline.clone();
    materials::start_order(&mut manual, NationId::USA, &d, 30.0, 30).unwrap();
    for _ in 0..2 {
        spheres_sim::tick_day(&mut baseline, &[]);
        spheres_sim::tick_day(&mut manual, &[]);
        close(
            resources::spot_imports_bn(&manual, NationId::USA),
            resources::spot_imports_bn(&baseline, NationId::USA),
        );
        assert_eq!(
            manual.nation(NationId::USA).treasury_bn,
            baseline.nation(NationId::USA).treasury_bn
        );
        assert_eq!(
            manual.nation(NationId::USA).debt_bn,
            baseline.nation(NationId::USA).debt_bn
        );
        let imports = |w: &WorldState| {
            w.resources
                .contracts
                .iter()
                .filter(|c| c.from == NationId::USA || c.to == NationId::USA)
                .count()
        };
        assert_eq!(imports(&manual), imports(&baseline));
        assert_eq!(
            manual
                .materials
                .as_ref()
                .unwrap()
                .accounts
                .get(&NationId::USA),
            None
        );
    }
}

#[test]
fn manual_ingredients_are_reserved_against_surplus_sales_not_bought() {
    let mut w = world();
    w.rules.physical_logistics = false;
    w.rules.logistics_routes = false;
    enroll(&mut w, NationId::USA);
    let d = districts(&w, NationId::USA)[0].clone();
    for c in ALL {
        stock(&mut w, NationId::USA, c, 0.0);
    }
    materials::start_order(&mut w, NationId::USA, &d, 30.0, 30).unwrap();
    let reserve = materials::resource_reserve(&w, NationId::USA);
    stock(&mut w, NationId::USA, C::Bauxite, reserve[C::Bauxite.idx()]);
    spheres_sim::arsenal::tick(&mut w);
    let m = w.resources.market.as_ref().unwrap();
    assert!(!m
        .fills
        .iter()
        .any(|f| f.seller == NationId::USA && f.commodity == C::Bauxite));
    let row = m
        .stocks
        .iter()
        .find(|s| s.nation == NationId::USA && s.commodity == C::Bauxite)
        .unwrap();
    assert!(row.reserve_target >= reserve[C::Bauxite.idx()]);
    assert_eq!(row.quantity, reserve[C::Bauxite.idx()]);
}
