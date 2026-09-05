//! Exact planning identities with explicit paid-asset fixtures; no calibration.
use spheres_sim::{
    apply_command, clock,
    commerce::{self, Good},
    economic_ai, industry_planning,
    init::world_1990,
    materials, production, programs, province_economy, resources, save, starting_industry,
    world::{GameRules, NationId, WorldState},
    Command,
};
const ME: NationId = NationId::USA;

fn prepared() -> (WorldState, String) {
    let mut w = world_1990(GameRules {
        daily_simulation: true,
        economic_competition: true,
        production_system: true,
        resource_market: true,
        manufacturing_system: true,
        ..GameRules::default()
    });
    starting_industry::enable_new_world(&mut w).unwrap();
    province_economy::enable(&mut w);
    w.conflicts.clear();
    w.nation_mut(ME).political_capital = 100.0;
    let allocations = w.nation(ME).budget_for(w.year).allocations;
    apply_command(
        &mut w,
        &Command::SetProgramBudget {
            nation: ME,
            fiscal_year: 1990,
            allocations,
            departments: programs::default_departments(),
        },
    )
    .unwrap();
    let d = w
        .districts
        .iter()
        .find(|(_, n)| **n == ME)
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
    // Generation + machinery, deliberately no government processor.
    w.production
        .industry
        .sites
        .insert(d.clone(), [1, 1, 0, 0, 0, 0, 0]);
    resources::tick(&mut w);
    for c in [
        resources::Commodity::Iron,
        resources::Commodity::Bauxite,
        resources::Commodity::Coal,
    ] {
        let stocks = &mut w.resources.market.as_mut().unwrap().stocks;
        match stocks.binary_search_by_key(&(ME, c), |s| (s.nation, s.commodity)) {
            Ok(i) => stocks[i].quantity = 100.0,
            Err(i) => stocks.insert(
                i,
                resources::Stock {
                    nation: ME,
                    commodity: c,
                    quantity: 100.0,
                    reserve_target: 0.0,
                },
            ),
        }
    }
    (w, d)
}
fn balance(w: &WorldState) -> industry_planning::GoodsBalance {
    industry_planning::plan(w, ME)
        .goods
        .into_iter()
        .find(|g| g.good == Good::Intermediates)
        .unwrap()
}

#[test]
fn finite_domestic_orders_cover_demand_without_becoming_factories_stock_or_imports() {
    let (mut w, d) = prepared();
    let before = balance(&w);
    assert!(before.demand_daily > 0.0);
    let opening = save(&w);
    assert_eq!(balance(&w).contracted_daily, 0.0);
    assert_eq!(save(&w), opening);
    materials::start_order(&mut w, ME, &d, 30.0, 30).unwrap();
    let after = balance(&w);
    assert_eq!(after.contracted_daily, 1.0);
    assert_eq!(after.contracted_remaining, 30.0);
    assert_eq!(after.installed_daily, before.installed_daily);
    assert_eq!(after.committed_daily, before.committed_daily);
    assert_eq!(after.stock, before.stock);
    assert_eq!(after.incoming, before.incoming);
    assert!(after.expansion_daily < before.expansion_daily);
    assert_eq!(
        industry_planning::plan(&w, ME).goods[1].contracted_remaining,
        0.0
    );
    for _ in 0..30 {
        clock::advance_date(&mut w);
    }
    let expired = balance(&w);
    assert_eq!(expired.contracted_daily, 0.0);
    assert_eq!(expired.contracted_remaining, 0.0);
    assert_eq!(expired.expansion_daily, before.expansion_daily);
}

#[test]
fn cancelled_and_lost_province_contracts_do_not_mask_future_shortages() {
    let (mut w, d) = prepared();
    let id = materials::start_order(&mut w, ME, &d, 30.0, 30).unwrap();
    materials::cancel_order(&mut w, ME, id).unwrap();
    assert_eq!(balance(&w).contracted_remaining, 0.0);
    materials::start_order(&mut w, ME, &d, 30.0, 30).unwrap();
    w.districts.insert(d, NationId::Canada);
    assert_eq!(balance(&w).contracted_remaining, 0.0);
    assert_eq!(
        materials::pending(&w, NationId::Canada),
        0.0,
        "No government contract transfers as a free asset"
    );
}

#[test]
fn ai_orders_a_feasible_finite_quantity_then_accounts_for_its_existing_order() {
    let (mut w, d) = prepared();
    let before = save(&w);
    let cmd = economic_ai::materials_order_candidate(&w, ME)
        .expect("A powered, supplied inherited province can meet real machinery demand");
    assert_eq!(before, save(&w), "AI proposal is a pure calculation");
    let quantity = match &cmd {
        Command::OrderMaterials {
            district,
            quantity,
            delivery_days,
            ..
        } => {
            assert_eq!(district, &d);
            assert_eq!(*delivery_days, 30);
            assert!(*quantity > 0.0);
            *quantity
        }
        _ => panic!("wrong proposal"),
    };
    let pc = w.nation(ME).political_capital;
    apply_command(&mut w, &cmd).unwrap();
    assert_eq!(w.nation(ME).political_capital, pc - materials::ORDER_PC);
    assert_eq!(materials::pending(&w, ME), quantity);
    assert_eq!(commerce::stock(&w, ME, Good::Intermediates), 0.0);
    assert!(
        economic_ai::materials_order_candidate(&w, ME).is_none(),
        "One contract is enough; no duplicate reservation"
    );
}

#[test]
fn ai_does_not_order_unsupplied_or_unpowered_inherited_capacity() {
    let (mut w, d) = prepared();
    let stock = w
        .resources
        .market
        .as_mut()
        .unwrap()
        .stocks
        .iter_mut()
        .find(|s| s.nation == ME && s.commodity == resources::Commodity::Iron)
        .unwrap();
    stock.quantity = 0.0;
    assert!(economic_ai::materials_order_candidate(&w, ME).is_none());
    w.production.industry.sites.get_mut(&d).unwrap()[0] = 0;
    assert!(economic_ai::materials_order_candidate(&w, ME).is_none());
}
