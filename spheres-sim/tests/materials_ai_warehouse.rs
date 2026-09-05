//! Bounded warehouse-rescue regressions. Installed assets, money and stocks
//! here are explicit test fixtures, never grants from the inherited estimates.
use spheres_sim::{
    apply_command, commerce::{self, Good}, economic_ai, init::world_1990,
    materials, production::{self, ProjectKind as K}, programs, province_economy,
    resources::{self, Commodity}, starting_industry,
    world::{GameRules, NationId, WorldState, BUDGET_INDUSTRY}, Command,
};

const ME: NationId = NationId::USA;

fn set_raw(w: &mut WorldState, commodity: Commodity, quantity: f64) {
    let stocks = &mut w.resources.market.as_mut().unwrap().stocks;
    match stocks.binary_search_by_key(&(ME, commodity), |s| (s.nation, s.commodity)) {
        Ok(i) => stocks[i].quantity = quantity,
        Err(i) => stocks.insert(i, resources::Stock {
            nation: ME, commodity, quantity, reserve_target: 0.0,
        }),
    }
}

fn warehouse_fixture(target_department: usize) -> (WorldState, String) {
    let mut w = world_1990(GameRules {
        daily_simulation: true, economic_competition: true, production_system: true,
        resource_market: true, manufacturing_system: true, physical_logistics: true,
        logistics_routes: true, ai_aggression: 0.0, ..GameRules::default()
    });
    starting_industry::enable_new_world(&mut w).unwrap();
    province_economy::enable(&mut w);
    w.conflicts.clear();
    w.sanctions.clear();
    w.nation_mut(ME).political_capital = 1000.0;
    w.nation_mut(ME).debt_gdp = 0.0;
    let allocations = w.nation(ME).budget_for(w.year).allocations;
    let mut departments = programs::default_departments();
    departments[BUDGET_INDUSTRY] = [1000; 5];
    departments[BUDGET_INDUSTRY][target_department] = 6000;
    apply_command(&mut w, &Command::SetProgramBudget {
        nation: ME, fiscal_year: 1990, allocations, departments,
    }).unwrap();
    let district = w.districts.iter().find(|(_, n)| **n == ME).unwrap().0.clone();
    w.production.provinces.push(production::ProvinceCapabilities {
        district: district.clone(), civilian_industry: 1, power_grid: 1,
        infrastructure: 0, research_centers: 0, arms_plants: 0,
    });
    w.production.industry.sites.insert(district.clone(), [0, 1, 0, 0, 0, 0, 0]);
    resources::tick(&mut w);
    for c in resources::ALL { set_raw(&mut w, c, 1000.0); }
    programs::begin_day(&mut w);
    apply_command(&mut w, &Command::StartProject {
        nation: ME, district: district.clone(), kind: K::Warehouse,
    }).unwrap();
    assert_eq!(commerce::demand(&w, ME, Good::Intermediates), 12.0);
    assert_eq!(commerce::demand(&w, ME, Good::CapitalGoods), 5.0);
    assert_eq!(commerce::stock(&w, ME, Good::Intermediates), 0.0);
    (w, district)
}

#[test]
fn an_inherited_warehouse_cannot_bypass_full_cover_with_a_large_but_short_contract() {
    let (mut w, district) = warehouse_fixture(0);
    apply_command(&mut w, &Command::OrderMaterials {
        nation: ME, district, quantity: 26.999, delivery_days: 30,
    }).unwrap();
    // Prevent the new AI helper legitimately buying the missing 0.001 pack.
    // This existing contract remains a real finite order, not usable stock.
    set_raw(&mut w, Commodity::Iron, 0.0);
    assert_eq!(materials::pending(&w, ME), 26.999);
    assert_eq!(commerce::stock(&w, ME, Good::Intermediates), 0.0);
    assert!(economic_ai::materials_order_candidate(&w, ME).is_none());
    economic_ai::evaluate(&mut w, ME);
    assert!(production::projects_for(&w, ME).all(|p| p.kind != K::MachineryWorks),
        "26.999 contracted packs cannot cover the warehouse's 12 plus the machine's 15; the old >=15 contract shortcut must not bypass the new 27-pack gate");
    assert_eq!(w.materials.as_ref().unwrap().orders.len(), 1,
        "A missing ingredient must not create an unbacked top-up contract");
}

#[test]
fn a_reachable_capital_import_does_not_create_an_orphan_materials_startup_order() {
    let (mut w, _) = warehouse_fixture(3);
    // The warehouse already owns all twelve of its construction packs. Only
    // capital goods are missing; no ordinary Materials replenishment is due.
    w.production.industry.goods.entry(ME).or_default().intermediates = 12.0;
    assert_eq!(commerce::shortage(&w, ME, Good::Intermediates), 0.0);
    let seller = NationId::Canada;
    w.nation_mut(seller).political_capital = 1000.0;
    let allocations = w.nation(seller).budget_for(w.year).allocations;
    apply_command(&mut w, &Command::SetProgramBudget {
        nation: seller, fiscal_year: 1990, allocations,
        departments: programs::default_departments(),
    }).unwrap();
    // Explicit already-earned treasury and produced seller inventory.
    w.nation_mut(ME).treasury_bn = Some(10.0);
    w.production.industry.goods.entry(seller).or_default().capital_goods = 100.0;
    apply_command(&mut w, &Command::SetGoodsSale {
        nation: seller, good: Good::CapitalGoods, reserve: 0.0,
        ask_multiplier: 1.0, enabled: true,
    }).unwrap();
    assert!(commerce::market_quotes(&w, ME, Good::CapitalGoods, 5.0, 365)
        .iter().any(|q| q.seller == seller && q.quantity >= 5.0 && q.accepted),
        "The fixture must have an actual consenting, funded, reachable capital-goods import");
    economic_ai::evaluate(&mut w, ME);
    assert!(w.materials.as_ref().is_none_or(|m| m.orders.is_empty()),
        "The warehouse can import its missing capital goods, so no machinery startup is selected and its 15-pack Materials lot must not be ordered");
    assert!(production::projects_for(&w, ME).all(|p| p.kind != K::MachineryWorks));
    assert!(w.commerce.as_ref().is_some_and(|c| c.contracts.iter().any(|k|
        k.buyer == ME && k.seller == seller && k.good == Good::CapitalGoods)),
        "The existing priced capital-import path should remain available");
}
