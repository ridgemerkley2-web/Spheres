//! Capacity invariants, not a statistical growth calibration. Physical assets
//! below are explicit test fixtures; no historical factory stock is granted.
use spheres_sim::{
    apply_command, economic_ai,
    init::world_1990,
    production::{self, ProjectKind as K},
    programs, save,
    world::{GameRules, NationId, WorldState},
    Command,
};
use spheres_sim::{
    clock,
    commerce::{self, Good},
    industry_planning as planning, load,
};

fn world() -> WorldState {
    let mut w = world_1990(GameRules {
        daily_simulation: true,
        economic_competition: true,
        production_system: true,
        resource_market: true,
        physical_logistics: true,
        logistics_routes: true,
        ..GameRules::default()
    });
    w.conflicts.clear();
    let id = NationId::USA;
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
    w
}
fn districts(w: &WorldState) -> Vec<String> {
    w.districts
        .iter()
        .filter(|(_, n)| **n == NationId::USA)
        .map(|(d, _)| d.clone())
        .collect()
}
fn estate(w: &mut WorldState, d: &str, grid: u8) {
    w.production
        .provinces
        .push(production::ProvinceCapabilities {
            district: d.into(),
            civilian_industry: 1,
            power_grid: grid,
            infrastructure: 0,
            research_centers: 0,
            arms_plants: 0,
        });
}

#[test]
fn existing_fractional_grid_and_processor_are_not_bought_again() {
    let mut w = world();
    let d = districts(&w)[0].clone();
    estate(&mut w, &d, 0);
    w.production.industry.modules.insert(d.clone(), 500_000);
    let before = save(&w);
    assert_eq!(
        economic_ai::candidate(&w, NationId::USA).unwrap().1,
        K::MachineryWorks
    );
    assert_eq!(
        save(&w),
        before,
        "planning must not create factories or GDP"
    );
}

#[test]
fn a_factory_on_another_owned_province_is_not_invisible() {
    let mut w = world();
    let ds = districts(&w);
    estate(&mut w, &ds[0], 1);
    w.production
        .industry
        .sites
        .insert(ds[0].clone(), [0, 1, 0, 0, 0, 0, 0]);
    // The acquired producer lives on a paid module estate, not the legacy
    // integer-estate list used by the old planner.
    w.production
        .industry
        .modules
        .insert(ds[1].clone(), 1_000_000);
    w.production
        .industry
        .sites
        .insert(ds[1].clone(), [1, 0, 0, 0, 0, 0, 0]);
    if let Ok((_, kind, _)) = economic_ai::candidate(&w, NationId::USA) {
        assert!(
            !matches!(kind, K::ProcessingPlant | K::MachineryWorks),
            "duplicate {kind:?}"
        );
    }
}

#[test]
fn a_full_unsold_stockpile_does_not_commission_a_warehouse() {
    let mut w = world();
    let d = districts(&w)[0].clone();
    estate(&mut w, &d, 1);
    w.production.industry.sites.insert(d, [1, 1, 1, 0, 0, 0, 0]);
    w.production.industry.goods.insert(
        NationId::USA,
        spheres_sim::industry::Goods {
            intermediates: 250.0,
            capital_goods: 250.0,
        },
    );
    if let Ok((_, kind, _)) = economic_ai::candidate(&w, NationId::USA) {
        assert!(
            !matches!(
                kind,
                K::Warehouse | K::ProcessingPlant | K::MachineryWorks | K::Automation
            ),
            "unsold stock caused {kind:?}"
        );
    }
}

fn receipt(w: &mut WorldState, good: Good, quantity: f64, day: i32) {
    w.commerce
        .get_or_insert_with(Default::default)
        .goods_deliveries
        .push(commerce::GoodsDelivery {
            contract: 77,
            day,
            buyer: NationId::Canada,
            seller: NationId::USA,
            good,
            quantity,
        });
}

#[test]
fn pending_lines_and_automation_count_once_not_as_missing_capacity() {
    let mut w = world();
    let d = districts(&w)[0].clone();
    estate(&mut w, &d, 1);
    w.production
        .industry
        .sites
        .insert(d.clone(), [1, 1, 1, 0, 0, 1, 0]);
    apply_command(
        &mut w,
        &Command::StartProject {
            nation: NationId::USA,
            district: d.clone(),
            kind: K::ProcessingPlant,
        },
    )
    .unwrap();
    let today = clock::absolute_day(&w);
    receipt(&mut w, Good::Intermediates, 90.0, today);
    let p = planning::plan(&w, NationId::USA);
    assert_eq!(p.goods[0].installed_daily, 1.2);
    assert_eq!(p.goods[0].committed_daily, 1.2);
    assert_eq!(p.goods[0].expansion_daily, 0.0);
    assert_eq!(p.goods[0].status, "already_committed");
    // A queued factory is not yet in the real operating ledger.
    assert_eq!(production::level(&w, &d, K::ProcessingPlant), 1);
    assert_eq!(w.production.projects.len(), 1);
}

#[test]
fn queued_fractional_workshop_reserves_every_component_without_free_output() {
    let mut w = world();
    let d = districts(&w)[0].clone();
    apply_command(
        &mut w,
        &Command::StartIndustryModule {
            nation: NationId::USA,
            district: d.clone(),
            capacity_micros: 250_000,
        },
    )
    .unwrap();
    let p = planning::plan(&w, NationId::USA);
    let province = p.provinces.iter().find(|p| p.district == d).unwrap();
    assert_eq!(province.estate, 0.0);
    assert_eq!(province.estate_committed, 0.25);
    assert_eq!(province.grid_committed_daily, 1.25);
    assert_eq!(p.goods[0].installed_daily, 0.0);
    assert_eq!(p.goods[0].committed_daily, 0.25);
    assert_eq!(p.generation_committed_daily, 2.5);
    assert_eq!(p.goods[0].stock, 0.0);
    assert!(
        economic_ai::candidate(&w, NationId::USA).is_err(),
        "do not order another bootstrap over paid work"
    );
}

#[test]
fn real_demand_can_expand_but_stock_or_inbound_supply_defers_it() {
    let mut w = world();
    let d = districts(&w)[0].clone();
    estate(&mut w, &d, 1);
    w.production.industry.sites.insert(d, [1, 1, 1, 0, 0, 0, 0]);
    let today = clock::absolute_day(&w);
    receipt(&mut w, Good::CapitalGoods, 180.0, today);
    let p = planning::plan(&w, NationId::USA);
    assert_eq!(p.goods[1].demand_daily, 2.0);
    assert_eq!(p.goods[1].expansion_daily, 2.0);
    w.production
        .industry
        .goods
        .entry(NationId::USA)
        .or_default()
        .capital_goods = 181.0;
    assert_eq!(
        planning::plan(&w, NationId::USA).goods[1].expansion_daily,
        0.0
    );
    w.production
        .industry
        .goods
        .get_mut(&NationId::USA)
        .unwrap()
        .capital_goods = 0.0;
    let c = commerce::Contract {
        id: 78,
        buyer: NationId::USA,
        seller: NationId::Canada,
        good: Good::CapitalGoods,
        quantity: 181.0,
        unit_price_bn: 0.00025,
        remaining_quantity: 181.0,
        escrow_bn: 181.0 * 0.00025,
        delivered_quantity: 0.0,
        cancelled_quantity: 0.0,
        paid_bn: 0.0,
        accepted_day: today,
        expires_day: today + 30,
        status: "active".into(),
        reason: None,
    };
    w.commerce.as_mut().unwrap().contracts.push(c);
    let p = planning::plan(&w, NationId::USA);
    assert_eq!(p.goods[1].incoming, 181.0);
    assert_eq!(p.goods[1].expansion_daily, 0.0);
    assert_eq!(
        p.goods[1].stock, 0.0,
        "inbound supply is not usable inventory"
    );
}

#[test]
fn exports_use_delivery_dates_and_goods_not_promises_or_high_prices() {
    let mut w = world();
    let today = clock::absolute_day(&w);
    receipt(&mut w, Good::Intermediates, 90.0, today);
    receipt(&mut w, Good::CapitalGoods, 900.0, today - 90); // expired evidence
    receipt(&mut w, Good::CapitalGoods, 900.0, today + 1); // not yet delivered
    assert_eq!(
        planning::delivered_daily(&w, NationId::USA, Good::Intermediates),
        1.0
    );
    assert_eq!(
        planning::delivered_daily(&w, NationId::USA, Good::CapitalGoods),
        0.0
    );
    // Legacy recent-contract evidence remains readable and is not counted
    // twice when its actual physical delivery also has a dated receipt.
    w.commerce
        .as_mut()
        .unwrap()
        .contracts
        .push(commerce::Contract {
            id: 77,
            buyer: NationId::Canada,
            seller: NationId::USA,
            good: Good::Intermediates,
            quantity: 90.0,
            unit_price_bn: 100.0,
            remaining_quantity: 0.0,
            escrow_bn: 0.0,
            delivered_quantity: 90.0,
            cancelled_quantity: 0.0,
            paid_bn: 9000.0,
            accepted_day: today - 5,
            expires_day: today + 5,
            status: "delivered".into(),
            reason: None,
        });
    assert_eq!(
        planning::delivered_daily(&w, NationId::USA, Good::Intermediates),
        1.0
    );
    let resumed = load(&save(&w)).unwrap();
    assert_eq!(
        planning::plan(&w, NationId::USA),
        planning::plan(&resumed, NationId::USA)
    );
    w.commerce.as_mut().unwrap().goods_deliveries.clear();
    assert_eq!(
        planning::delivered_daily(&w, NationId::USA, Good::Intermediates),
        1.0
    );
    for _ in 0..90 {
        clock::advance_date(&mut w);
    }
    assert_eq!(
        planning::delivered_daily(&w, NationId::USA, Good::Intermediates),
        0.0
    );
}

#[test]
fn ownership_changes_reassociate_assets_but_not_foreign_sponsored_work() {
    let mut w = world();
    let d = districts(&w)[0].clone();
    estate(&mut w, &d, 1);
    w.production
        .industry
        .sites
        .insert(d.clone(), [2, 1, 3, 0, 0, 0, 0]);
    apply_command(
        &mut w,
        &Command::StartProject {
            nation: NationId::USA,
            district: d.clone(),
            kind: K::ProcessingPlant,
        },
    )
    .unwrap();
    assert_eq!(
        planning::plan(&w, NationId::USA).goods[0].committed_daily,
        1.0
    );
    w.districts.insert(d.clone(), NationId::Canada);
    let previous = planning::plan(&w, NationId::USA);
    let current = planning::plan(&w, NationId::Canada);
    assert_eq!(previous.goods[0].installed_daily, 0.0);
    assert_eq!(previous.goods[0].committed_daily, 0.0);
    assert_eq!(current.goods[0].installed_daily, 3.0);
    assert_eq!(current.goods[1].installed_daily, 1.0);
    assert_eq!(current.goods[0].committed_daily, 0.0);
}

#[test]
fn all_starting_country_gdp_estimates_remain_uninvented_factory_assets() {
    let w = world();
    let before = save(&w);
    let mut count = 0;
    for n in w.nations.iter().filter(|n| n.alive) {
        let p = planning::plan(&w, n.id);
        assert!(p
            .goods
            .iter()
            .all(|g| g.installed_daily == 0.0 && g.committed_daily == 0.0));
        assert_eq!(p.generation_daily, 0.0);
        assert!(p.goods.iter().all(|g| g.expansion_daily.is_finite()));
        count += 1;
    }
    assert_eq!(count, 137);
    assert_eq!(before, save(&w));
}

#[test]
fn player_can_override_capacity_advice_without_free_or_instant_factories() {
    let mut w = world();
    let d = districts(&w)[0].clone();
    estate(&mut w, &d, 1);
    w.production
        .industry
        .sites
        .insert(d.clone(), [1, 1, 1, 0, 0, 0, 0]);
    w.production.industry.goods.insert(
        NationId::USA,
        spheres_sim::industry::Goods {
            intermediates: 250.0,
            capital_goods: 250.0,
        },
    );
    assert_eq!(
        planning::plan(&w, NationId::USA).goods[0].expansion_daily,
        0.0
    );
    let pc = w.nation(NationId::USA).political_capital;
    apply_command(
        &mut w,
        &Command::StartProject {
            nation: NationId::USA,
            district: d.clone(),
            kind: K::ProcessingPlant,
        },
    )
    .unwrap();
    assert!(w.nation(NationId::USA).political_capital < pc);
    assert_eq!(production::level(&w, &d, K::ProcessingPlant), 1);
    assert_eq!(w.production.projects[0].progress_days, 0.0);
    assert_eq!(
        planning::plan(&w, NationId::USA).goods[0].committed_daily,
        1.0
    );
}

#[test]
fn repeated_reviews_do_not_stack_producers_behind_unsold_goods() {
    let mut w = world();
    let d = districts(&w)[0].clone();
    estate(&mut w, &d, 1);
    w.production.provinces[0].research_centers = 1;
    w.production.industry.sites.insert(d, [3, 1, 5, 0, 0, 0, 0]);
    w.production.industry.goods.insert(
        NationId::USA,
        spheres_sim::industry::Goods {
            intermediates: 250.0,
            capital_goods: 250.0,
        },
    );
    let installed = w.production.industry.sites.clone();
    // This isolates repeated planning decisions; it is not a physical-world
    // growth simulation. Stock remains unsold throughout all twelve reviews.
    for _ in 0..12 {
        economic_ai::evaluate(&mut w, NationId::USA);
        assert!(w.production.projects.is_empty());
        assert!(w.production.industry.modules.is_empty());
        assert_eq!(w.production.industry.sites, installed);
        for _ in 0..30 {
            clock::advance_date(&mut w);
        }
    }
}

#[test]
fn fresh_input_blockers_call_for_supply_not_duplicate_factories_and_expire() {
    let mut w = world();
    let d = districts(&w)[0].clone();
    estate(&mut w, &d, 1);
    w.production.provinces[0].research_centers = 1;
    w.production
        .industry
        .sites
        .insert(d.clone(), [3, 1, 1, 0, 0, 0, 0]);
    let today = clock::absolute_day(&w);
    for reason in [
        "Missing raw inputs or generating fuel; this plant is paused, not the national economy.",
        "Missing Copper for the complete operating bundle.",
        "No department operating authority is available.",
    ] {
        w.production.industry.last_day = Some(today);
        w.production.industry.operations = vec![spheres_sim::industry::SiteStatus {
            district: d.clone(),
            kind: K::ProcessingPlant,
            level: 1,
            capacity_micros: None,
            status: "blocked".into(),
            reason: Some(reason.into()),
            output_daily: 0.0,
            power_used_daily: 0.0,
            cash_spent_daily_bn: 0.0,
        }];
        assert!(planning::plan(&w, NationId::USA).goods[0].expansion_daily > 0.0);
        let why = economic_ai::candidate(&w, NationId::USA).unwrap_err();
        assert!(why.contains("Restore the existing"), "{why}");
    }
    w.production.industry.last_day = Some(today - 2);
    assert_eq!(
        economic_ai::candidate(&w, NationId::USA).unwrap().1,
        K::StarterIndustry,
        "stale receipt must not permanently freeze useful demand-sized expansion"
    );
}

#[test]
fn existing_empty_estates_do_not_all_receive_grids_before_a_useful_factory() {
    let mut w = world();
    let ds = districts(&w);
    estate(&mut w, &ds[0], 0); // intentionally empty, alphabetical first
    estate(&mut w, &ds[1], 1);
    w.production
        .industry
        .sites
        .insert(ds[1].clone(), [0, 1, 1, 0, 0, 0, 0]);
    let (d, k, _) = economic_ai::candidate(&w, NationId::USA).unwrap();
    assert_eq!(k, K::MachineryWorks);
    assert_eq!(d, ds[1], "prefer an already powered eligible site");
}
