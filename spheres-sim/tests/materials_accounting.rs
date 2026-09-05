//! Universal accounting identities, not a long-run calibration statistic.
//! Paid inherited output must replace its represented share, never add it twice.
use spheres_sim::{
    clock, gdp_projects as gdp, industry, init::world_1990, load, production,
    province_economy as economy, resources, save, starting_industry,
    world::{GameRules, NationId, WorldState},
};

const USA: NationId = NationId::USA;
fn near(a: f64, b: f64) {
    assert!((a-b).abs() < 1e-10 * a.abs().max(b.abs()).max(1.0), "{a} != {b}");
}
fn prepared() -> (WorldState, String) {
    let mut w = world_1990(GameRules { daily_simulation: true, ..GameRules::default() });
    starting_industry::enable_new_world(&mut w).unwrap();
    economy::enable(&mut w);
    economy::begin_day(&mut w);
    let district = w.districts.iter().find(|(_, n)| **n == USA).unwrap().0.clone();
    (w, district)
}
fn next(w: &mut WorldState) {
    clock::advance_date(w);
    economy::begin_day(w);
}
fn produce(w: &mut WorldState, district: &str, id: u32, output: f64, reserved: f64) {
    let raw = recipe(output, 0.0);
    gdp::record_materials_operation(w, USA, district, id, output, reserved, 0.0, raw, 0.00003 * output, 0.0);
}
fn recipe(output: f64, power: f64) -> [f64; 12] {
    let mut raw = [0.0; 12];
    raw[resources::Commodity::Iron.idx()] = output;
    raw[resources::Commodity::Bauxite.idx()] = output * 0.2;
    raw[resources::Commodity::Coal.idx()] = output * 0.25 + power * 0.02;
    raw.map(|v| (v * 1e9).round() / 1e9)
}
fn stock(w: &mut WorldState, commodity: resources::Commodity, quantity: f64) {
    if w.resources.market.is_none() { resources::tick(w); }
    let stocks = &mut w.resources.market.as_mut().unwrap().stocks;
    match stocks.binary_search_by_key(&(USA, commodity), |s| (s.nation, s.commodity)) {
        Ok(i) => stocks[i].quantity = quantity,
        Err(i) => stocks.insert(i, resources::Stock { nation: USA, commodity, quantity, reserve_target: 0.0 }),
    }
}
fn reconcile(w: &WorldState, nation: NationId) {
    let s = economy::snapshot(w, nation).unwrap();
    near(s.inherited_gdp_bn + s.project_gdp_bn, s.total_gdp_bn);
    near(s.provinces.iter().map(|p| p.total_gdp_bn).sum::<f64>() + s.unallocated_gdp_bn, s.total_gdp_bn);
    near(s.sectors.iter().map(|p| p.gdp_bn).sum(), s.total_gdp_bn);
    for p in &s.provinces {
        near(p.inherited_gdp_bn + p.project_gdp_bn, p.total_gdp_bn);
        near(p.sectors.iter().map(|s| s.gdp_bn).sum(), p.total_gdp_bn);
        if let Some(m) = &p.materials_accounting {
            near(m.background_annual_bn + m.additional_annual_bn, m.total_annual_bn);
            near(m.unobserved_annual_bn + m.observed_annual_bn, m.total_annual_bn);
            near(m.already_included_annual_bn + m.additional_annual_bn, m.observed_annual_bn);
            assert!(m.unobserved_annual_bn >= 0.0);
            assert!(m.already_included_annual_bn <= m.background_annual_bn);
        }
    }
}

#[test]
fn unused_or_underutilized_reservation_does_not_mint_gdp_or_erase_background() {
    let (mut w, d) = prepared();
    let initial = w.nation(USA).gdp;
    let cash = (w.nation(USA).treasury_bn, w.nation(USA).debt_bn);
    let background = economy::province(&w, &d).unwrap().materials_accounting.unwrap().background_annual_bn;
    // Five packs against ten reserved is below the inherited 80% utilization.
    produce(&mut w, &d, 1, 5.0, 10.0);
    let row = &gdp::contributions(&w)[0];
    assert!(row.annual_gdp_bn > 0.0);
    near(row.inherited_annual_gdp_bn, row.annual_gdp_bn);
    near(gdp::incremental_gdp_bn(row), 0.0);
    assert_eq!(economy::materials_summary(&w, USA), (0.0, 0.0), "Unsettled flows are not achieved GDP");
    economy::finish_day(&mut w);
    assert_eq!(w.nation(USA).gdp, initial);
    let m = economy::province(&w, &d).unwrap().materials_accounting.unwrap();
    assert!(m.observed_annual_bn > 0.0);
    near(m.background_annual_bn, background);
    near(m.additional_annual_bn, 0.0);
    reconcile(&w, USA);
    next(&mut w);
    economy::finish_day(&mut w); // blocked, paused, cancelled: no production receipt
    assert_eq!(w.nation(USA).gdp, initial);
    let m = economy::province(&w, &d).unwrap().materials_accounting.unwrap();
    near(m.observed_annual_bn, 0.0);
    near(m.unobserved_annual_bn, background);
    assert_eq!((w.nation(USA).treasury_bn, w.nation(USA).debt_bn), cash, "Accounting never charges or refunds cash");
}

#[test]
fn full_throughput_replaces_only_spare_capacity_rate_and_never_accumulates() {
    let (mut w, d) = prepared();
    let initial = w.nation(USA).gdp;
    produce(&mut w, &d, 2, 10.0, 10.0);
    let row = gdp::contributions(&w).remove(0);
    near(row.inherited_annual_gdp_bn, row.annual_gdp_bn * 0.8);
    let extra = gdp::incremental_gdp_bn(&row);
    assert!(extra > 0.0);
    let before = save(&w);
    produce(&mut w, &d, 2, 10.0, 10.0);
    assert_eq!(save(&w), before, "Same order/day cannot repost observed GDP");
    economy::finish_day(&mut w);
    near(w.nation(USA).gdp, initial + extra);
    near(economy::project_level(&w, USA).unwrap(), extra);
    let settled = save(&w);
    economy::finish_day(&mut w);
    assert_eq!(settled, save(&w));
    let mut restored = load(&settled).unwrap();
    for _ in 0..8 {
        for game in [&mut w, &mut restored] {
            next(game);
            produce(game, &d, 2, 10.0, 10.0);
            economy::finish_day(game);
            near(game.nation(USA).gdp, initial + extra);
            reconcile(game, USA);
        }
        assert_eq!(save(&w), save(&restored));
    }
    next(&mut w);
    economy::finish_day(&mut w);
    near(w.nation(USA).gdp, initial);
    next(&mut w);
    produce(&mut w, &d, 3, 10.0, 10.0);
    economy::finish_day(&mut w);
    near(w.nation(USA).gdp, initial + extra);
}

#[test]
fn explicit_materials_do_not_remove_background_from_macro_growth() {
    let (mut w, d) = prepared();
    let initial = w.nation(USA).gdp;
    produce(&mut w, &d, 4, 10.0, 10.0);
    economy::finish_day(&mut w);
    let added = economy::project_level(&w, USA);
    let mut gdp = w.nation(USA).gdp;
    economy::apply_macro_factor(&mut gdp, added, 1.01);
    near(gdp, initial * 1.01 + added.unwrap());
}

#[test]
fn paid_conversion_and_generation_conserve_intermediate_value_once() {
    let (mut w, d) = prepared();
    w.production.industry.sites.insert(d.clone(), [0, 1, 0, 0, 0, 0, 0]);
    let raw = recipe(10.0, 10.0);
    gdp::record_materials_operation(&mut w, USA, &d, 5, 10.0, 10.0, 10.0, raw, 0.01, 0.02);
    let rows = gdp::contributions(&w);
    let raw_cost: f64 = resources::ALL.iter().map(|c| raw[c.idx()] * resources::unit_price_bn(*c).unwrap_or(0.0)).sum();
    near(rows.iter().map(|r| r.daily_value_added_bn).sum(), 10.0 * gdp::INTERMEDIATE_PACK_BN - raw_cost);
    near(rows.iter().map(|r| r.payments_daily_bn).sum(), 0.03);
    let energy = rows.iter().find(|r| r.kind == "generation").unwrap();
    near(energy.output_quantity_daily, 10.0);
    assert_eq!(energy.inherited_annual_gdp_bn, 0.0);
    let material = rows.iter().find(|r| r.kind == "inherited_materials").unwrap();
    near(material.inherited_annual_gdp_bn, material.annual_gdp_bn * 0.8);
    economy::finish_day(&mut w);
    reconcile(&w, USA);
}

#[test]
fn territorial_transfer_moves_observation_without_a_new_award_or_stale_national_claim() {
    let (mut w, d) = prepared();
    produce(&mut w, &d, 6, 5.0, 10.0);
    economy::finish_day(&mut w);
    let observer = save(&w);
    let moved_gdp = economy::province(&w, &d).unwrap().total_gdp_bn;
    w.nation_mut(USA).gdp -= moved_gdp;
    w.nation_mut(NationId::Tonga).gdp += moved_gdp;
    w.districts.insert(d.clone(), NationId::Tonga);
    let after_transfer = (w.nation(USA).gdp, w.nation(NationId::Tonga).gdp);
    assert_eq!(economy::materials_summary(&w, USA), (0.0, 0.0));
    assert!(economy::materials_summary(&w, NationId::Tonga).0 > 0.0);
    reconcile(&w, USA);
    reconcile(&w, NationId::Tonga);
    let before = save(&w);
    let _ = economy::snapshot(&w, USA);
    let _ = economy::snapshot(&w, NationId::Tonga);
    assert_eq!(before, save(&w), "Ownership-aware account views are pure");
    next(&mut w);
    economy::finish_day(&mut w);
    assert_eq!(after_transfer, (w.nation(USA).gdp, w.nation(NationId::Tonga).gdp));
    assert_ne!(observer, save(&w));
}

#[test]
fn low_book_value_transfer_reconciles_individual_observation_and_background_view() {
    let (mut w, d) = prepared();
    produce(&mut w, &d, 60, 5.0, 10.0);
    economy::finish_day(&mut w);
    w.districts.insert(d.clone(), NationId::Tonga);
    w.nation_mut(NationId::Tonga).gdp = 0.000001;
    let before = save(&w);
    let p = economy::province(&w, &d).unwrap();
    let m = p.materials_accounting.as_ref().unwrap();
    let r = p.projects.iter().find(|r| r.kind == "inherited_materials").unwrap();
    near(r.inherited_annual_gdp_bn, m.already_included_annual_bn);
    near(r.annual_gdp_bn, m.observed_annual_bn);
    near(r.output_quantity_daily, 5.0);
    assert!(r.valuation_basis.contains("reconciliation"));
    near(m.unobserved_annual_bn, 0.0);
    reconcile(&w, NationId::Tonga);
    assert_eq!(before, save(&w));
}

#[test]
fn legacy_absence_stays_byte_inert_and_new_coverage_validates() {
    let mut legacy = world_1990(GameRules::default());
    let before = save(&legacy);
    gdp::record_materials_operation(&mut legacy, USA, "missing", 7, 1.0, 1.0, 0.0, [0.0;12], 0.0, 0.0);
    assert_eq!(before, save(&legacy));
    assert!(!before.contains("inherited_annual_gdp_bn"));
    let (mut w, d) = prepared();
    produce(&mut w, &d, 8, 1.0, 1.0);
    w.province_economy.as_mut().unwrap().flows.receipts.get_mut("inherited_materials:8").unwrap().inherited_annual_gdp_bn = f64::NAN;
    let gdp = w.nation(USA).gdp;
    economy::finish_day(&mut w);
    assert_eq!(w.nation(USA).gdp, gdp);
    assert!(w.province_economy.as_ref().unwrap().posted_contributions.iter().all(|r| !r.counted));
}

#[test]
fn real_order_paused_resumed_and_cancelled_uses_same_accounting_without_legacy_drag() {
    let (mut w, d) = prepared();
    w.rules.economic_competition = true;
    w.rules.production_system = true;
    w.rules.resource_market = true;
    w.player = Some(USA);
    let allocations = w.nation(USA).budget_for(w.year).allocations;
    spheres_sim::apply_command(&mut w, &spheres_sim::Command::SetProgramBudget {
        nation: USA, fiscal_year: 1990, allocations,
        departments: spheres_sim::programs::default_departments(),
    }).unwrap();
    spheres_sim::programs::begin_day(&mut w);
    for c in resources::ALL {
        if c != resources::Commodity::Oil {
            stock(&mut w, c, 1e6);
        }
    }
    w.production.industry.sites.insert(d.clone(), [0, 1, 0, 0, 0, 0, 0]);
    w.production.provinces.push(production::ProvinceCapabilities {
        district: d.clone(), infrastructure: 0, civilian_industry: 0, power_grid: 1,
        arms_plants: 0, research_centers: 0,
    });
    let baseline = w.nation(USA).gdp;
    let id = spheres_sim::materials::start_order(&mut w, USA, &d, 70.0, 7).unwrap();
    stock(&mut w, resources::Commodity::Iron, 0.0);
    industry::tick_day(&mut w);
    economy::finish_day(&mut w);
    assert_eq!(w.nation(USA).gdp, baseline);
    assert_eq!(w.materials.as_ref().unwrap().orders[0].status, "paused");
    next(&mut w);
    spheres_sim::programs::begin_day(&mut w);
    stock(&mut w, resources::Commodity::Iron, 1e6);
    industry::tick_day(&mut w);
    economy::finish_day(&mut w);
    let order = &w.materials.as_ref().unwrap().orders[0];
    near(order.output_today, 5.0); // one local grid serves 5 of 10 reserved packs
    assert_eq!(order.status, "limited");
    let (represented, extra) = economy::materials_summary(&w, USA);
    assert!(represented > 0.0);
    near(extra, 0.0);
    assert!(w.nation(USA).gdp >= baseline, "Only the separately produced power is additional");
    let delivered = order.delivered;
    spheres_sim::materials::cancel_order(&mut w, USA, id).unwrap();
    let saved = save(&w);
    let mut resumed = load(&saved).unwrap();
    for game in [&mut w, &mut resumed] {
        next(game);
        spheres_sim::programs::begin_day(game);
        industry::tick_day(game);
        economy::finish_day(game);
        near(game.nation(USA).gdp, baseline);
        near(game.materials.as_ref().unwrap().orders[0].delivered, delivered);
        near(game.production.industry.goods[&USA].intermediates, delivered);
        assert_eq!(economy::materials_summary(game, USA), (0.0, 0.0));
        reconcile(game, USA);
    }
    assert_eq!(save(&w), save(&resumed));
}
