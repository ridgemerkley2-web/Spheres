//! Materials delivery windows count service dates, not an already-settled
//! signing date. All operational assets and stocks are explicit test fixtures.
use spheres_sim::{
    apply_command, clock, commerce::{self, Good}, economic_ai, industry,
    init::world_1990, load, materials, production::{self, ProjectKind as K},
    programs, province_economy, resources::{self, Commodity}, save, starting_industry,
    world::{GameRules, NationId, WorldState, BUDGET_INDUSTRY}, Command,
};

const ME: NationId = NationId::USA;

fn prepared() -> (WorldState, String) {
    let mut w = world_1990(GameRules {
        daily_simulation: true, economic_competition: true, production_system: true,
        resource_market: true, manufacturing_system: true, ai_aggression: 0.0,
        ..GameRules::default()
    });
    starting_industry::enable_new_world(&mut w).unwrap();
    province_economy::enable(&mut w);
    w.conflicts.clear();
    w.nation_mut(ME).political_capital = 1000.0;
    w.nation_mut(ME).debt_gdp = 0.0;
    let allocations = w.nation(ME).budget_for(w.year).allocations;
    let mut departments = programs::default_departments();
    departments[BUDGET_INDUSTRY] = [6000, 1000, 1000, 1000, 1000];
    apply_command(&mut w, &Command::SetProgramBudget {
        nation: ME, fiscal_year: 1990, allocations, departments,
    }).unwrap();
    let district = w.districts.iter().find(|(_, owner)| **owner == ME).unwrap().0.clone();
    w.production.provinces.push(production::ProvinceCapabilities {
        district: district.clone(), civilian_industry: 1, power_grid: 1,
        infrastructure: 0, research_centers: 0, arms_plants: 0,
    });
    w.production.industry.sites.insert(district.clone(), [0, 1, 0, 0, 0, 0, 0]);
    resources::tick(&mut w);
    for c in resources::ALL {
        let stocks = &mut w.resources.market.as_mut().unwrap().stocks;
        match stocks.binary_search_by_key(&(ME, c), |s| (s.nation, s.commodity)) {
            Ok(i) => stocks[i].quantity = 1000.0,
            Err(i) => stocks.insert(i, resources::Stock {
                nation: ME, commodity: c, quantity: 1000.0, reserve_target: 0.0,
            }),
        }
    }
    province_economy::begin_day(&mut w);
    programs::begin_day(&mut w);
    (w, district)
}

fn finish_day(w: &mut WorldState) {
    programs::finish_day(w);
    province_economy::finish_day(w);
    clock::advance_date(w);
}

fn service_day(w: &mut WorldState) {
    province_economy::begin_day(w);
    programs::begin_day(w);
    production::tick_day(w);
    industry::tick_day(w);
    let once = save(w);
    industry::tick_day(w);
    assert_eq!(save(w), once, "A second industry call cannot duplicate today's delivery");
    finish_day(w);
}

#[test]
fn ai_signing_after_industry_gets_thirty_future_paid_service_dates() {
    let (mut w, _) = prepared();
    let signing_day = clock::absolute_day(&w);
    // This is the real SYSTEMS order: industry settles before economic AI.
    industry::tick_day(&mut w);
    assert_eq!(w.production.industry.last_day, Some(signing_day));
    economic_ai::evaluate(&mut w, ME);
    assert_eq!(w.materials.as_ref().unwrap().orders[0].quantity, 15.0);
    assert!(production::projects_for(&w, ME).any(|p| p.kind == K::MachineryWorks));
    let signed = save(&w);
    industry::tick_day(&mut w);
    assert_eq!(save(&w), signed, "Signing cannot reopen an already-settled production date");
    assert_eq!(commerce::stock(&w, ME, Good::Intermediates), 0.0);
    finish_day(&mut w);
    for _ in 0..13 { service_day(&mut w); }
    let mut resumed = load(&save(&w)).unwrap();
    for _ in 13..30 { service_day(&mut w); service_day(&mut resumed); }
    assert_eq!(save(&w), save(&resumed), "A mid-contract save preserves the service window");
    let order = &w.materials.as_ref().unwrap().orders[0];
    assert_eq!(order.delivered, 15.0,
        "Thirty supplied future service dates must deliver 15 packs, not expire after 29 dates at 14.5");
    assert_eq!(order.status, "completed");
    assert_eq!(order.remaining, 0.0);
    assert_eq!(order.start_day, signing_day + 1);
    assert_eq!(order.deadline_day, signing_day + 31);
    assert_eq!(commerce::stock(&w, ME, Good::Intermediates), 15.0);
    assert!((order.spent_conversion_bn - 15.0 * materials::CONVERSION_CASH_PER_PACK_BN).abs() < 1e-12);
    assert!((order.spent_energy_bn - 15.0 * materials::ENERGY_CASH_PER_POWER_BN).abs() < 1e-12);
}

#[test]
fn a_player_order_before_settlement_keeps_today_as_its_first_service_date() {
    let (mut w, district) = prepared();
    w.player = Some(ME);
    let today = clock::absolute_day(&w);
    apply_command(&mut w, &Command::OrderMaterials {
        nation: ME, district, quantity: 15.0, delivery_days: 30,
    }).unwrap();
    assert_eq!(w.materials.as_ref().unwrap().orders[0].start_day, today);
    assert_eq!(w.materials.as_ref().unwrap().orders[0].deadline_day, today + 30);
    industry::tick_day(&mut w);
    assert_eq!(commerce::stock(&w, ME, Good::Intermediates), 0.5);
    let once = save(&w);
    industry::tick_day(&mut w);
    assert_eq!(save(&w), once);
    finish_day(&mut w);
    for _ in 1..30 { service_day(&mut w); }
    let order = &w.materials.as_ref().unwrap().orders[0];
    assert_eq!(order.status, "completed");
    assert_eq!(order.delivered, 15.0);
    assert_eq!(order.last_day, Some(today + 29));
}

#[test]
fn a_future_order_cannot_run_when_a_caller_reenters_industry_on_the_signing_date() {
    let (mut w, district) = prepared();
    let today = clock::absolute_day(&w);
    industry::tick_day(&mut w);
    apply_command(&mut w, &Command::OrderMaterials {
        nation: ME, district, quantity: 15.0, delivery_days: 30,
    }).unwrap();
    assert_eq!(w.materials.as_ref().unwrap().orders[0].start_day, today + 1);
    let raw = resources::stockpile(&w, ME, Commodity::Iron);
    let conversion = programs::available_bn(&w, ME, BUDGET_INDUSTRY, 2);
    // Deliberately bypass only the caller's idempotence marker to exercise
    // Materials' own future-date guard, independently of industry::tick_day.
    w.production.industry.last_day = None;
    industry::tick_day(&mut w);
    assert_eq!(commerce::stock(&w, ME, Good::Intermediates), 0.0);
    assert_eq!(resources::stockpile(&w, ME, Commodity::Iron), raw);
    assert_eq!(programs::available_bn(&w, ME, BUDGET_INDUSTRY, 2), conversion);
    let order = &w.materials.as_ref().unwrap().orders[0];
    assert_eq!(order.status, "pending");
    assert_eq!(order.delivered, 0.0);
    assert_eq!(order.last_day, None);
}

#[test]
fn the_materials_settlement_marker_also_defers_a_later_order_and_old_saves_keep_dates() {
    let (mut w, district) = prepared();
    let today = clock::absolute_day(&w);
    let first = materials::start_order(&mut w, ME, &district, 15.0, 30).unwrap();
    industry::tick_day(&mut w);
    assert_eq!(w.materials.as_ref().unwrap().last_day, Some(today));
    materials::cancel_order(&mut w, ME, first).unwrap();
    w.production.industry.last_day = None;
    let second = materials::start_order(&mut w, ME, &district, 7.0, 7).unwrap();
    let orders = &w.materials.as_ref().unwrap().orders;
    assert_eq!(orders.iter().find(|o| o.id == second).unwrap().start_day, today + 1);
    assert_eq!(orders.iter().find(|o| o.id == second).unwrap().deadline_day, today + 8);
    assert_eq!(orders.iter().find(|o| o.id == first).unwrap().start_day, today);
    assert_eq!(orders.iter().find(|o| o.id == first).unwrap().deadline_day, today + 30);
    let saved = save(&w);
    assert_eq!(save(&load(&saved).unwrap()), saved, "Loading must not shift old contracts' saved dates");
    assert_eq!(materials::pending(&w, ME), 7.0);
}

#[test]
fn default_world_and_unactivated_inherited_estimates_remain_exactly_inert() {
    for daily in [false, true] {
        let mut w = world_1990(GameRules {
            daily_simulation: daily, economic_competition: daily,
            production_system: daily, resource_market: daily,
            manufacturing_system: daily, ..GameRules::default()
        });
        if daily { starting_industry::enable_new_world(&mut w).unwrap(); }
        let before = save(&w);
        industry::tick_day(&mut w);
        assert!(w.materials.is_none());
        assert_eq!(save(&w), before);
    }
}
