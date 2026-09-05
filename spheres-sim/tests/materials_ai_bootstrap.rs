//! Exact AI bootstrap invariants, not a historical factory calibration.
//! Every installed asset, raw stock and political reserve below is an explicit
//! paid-capacity test fixture. Initialization grants none of them in the game.
use spheres_sim::{
    apply_command, clock,
    commerce::{self, Good},
    economic_ai, industry,
    init::world_1990,
    load, materials,
    production::{self, ProjectKind as K},
    programs, province_economy,
    resources::{self, Commodity},
    save, starting_industry,
    world::{GameRules, NationId, WorldState, BUDGET_INDUSTRY},
    Command,
};

const ME: NationId = NationId::USA;
const STARTER_PACKS: f64 = 15.0;
const WINDOW: u32 = 30;

fn set_raw(w: &mut WorldState, commodity: Commodity, quantity: f64) {
    let stocks = &mut w.resources.market.as_mut().unwrap().stocks;
    match stocks.binary_search_by_key(&(ME, commodity), |s| (s.nation, s.commodity)) {
        Ok(i) => stocks[i].quantity = quantity,
        Err(i) => stocks.insert(
            i,
            resources::Stock {
                nation: ME,
                commodity,
                quantity,
                reserve_target: 0.0,
            },
        ),
    }
}

fn prepared() -> (WorldState, String) {
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
    starting_industry::enable_new_world(&mut w).unwrap();
    province_economy::enable(&mut w);
    w.conflicts.clear();
    w.nation_mut(ME).political_capital = 1000.0;
    w.nation_mut(ME).debt_gdp = 0.0;
    let allocations = w.nation(ME).budget_for(w.year).allocations;
    let mut departments = programs::default_departments();
    departments[BUDGET_INDUSTRY] = [6000, 1000, 1000, 1000, 1000];
    apply_command(
        &mut w,
        &Command::SetProgramBudget {
            nation: ME,
            fiscal_year: 1990,
            allocations,
            departments,
        },
    )
    .unwrap();
    let district = w
        .districts
        .iter()
        .find(|(_, n)| **n == ME)
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
    // Generation only. Neither a physical processor nor a machine shop exists.
    w.production
        .industry
        .sites
        .insert(district.clone(), [0, 1, 0, 0, 0, 0, 0]);
    resources::tick(&mut w);
    for commodity in resources::ALL {
        set_raw(&mut w, commodity, 1000.0);
    }
    programs::begin_day(&mut w);
    assert_eq!(
        commerce::demand(&w, ME, Good::Intermediates),
        0.0,
        "The first machine is an AI intention, not invented public consumption"
    );
    assert_eq!(commerce::stock(&w, ME, Good::Intermediates), 0.0);
    assert_eq!(commerce::stock(&w, ME, Good::CapitalGoods), 0.0);
    (w, district)
}

fn freeze_other_reviews(w: &mut WorldState) {
    let today = clock::absolute_day(w);
    let year = w.year;
    for id in w
        .nations
        .iter()
        .filter(|n| n.alive && n.id != ME)
        .map(|n| n.id)
    {
        w.economic_ai.nations.insert(
            id,
            economic_ai::NationPlan {
                last_review_day: today,
                fiscal_year: year,
                ..Default::default()
            },
        );
    }
}

fn consenting_import_alternative(w: &mut WorldState) {
    let seller = NationId::Canada;
    w.nation_mut(seller).political_capital = 1000.0;
    let allocations = w.nation(seller).budget_for(w.year).allocations;
    apply_command(
        w,
        &Command::SetProgramBudget {
            nation: seller,
            fiscal_year: 1990,
            allocations,
            departments: programs::default_departments(),
        },
    )
    .unwrap();
    // Already-produced seller stock is a fixture, not an inherited-capacity grant.
    w.production
        .industry
        .goods
        .entry(seller)
        .or_default()
        .intermediates = 100.0;
    apply_command(
        w,
        &Command::SetGoodsSale {
            nation: seller,
            good: Good::Intermediates,
            reserve: 0.0,
            ask_multiplier: 1.0,
            enabled: true,
        },
    )
    .unwrap();
    assert!(
        commerce::market_quotes(w, ME, Good::Intermediates, STARTER_PACKS, 365)
            .iter()
            .any(|q| q.quantity >= STARTER_PACKS),
        "A real consenting import alternative must exist"
    );
}

#[test]
fn a_first_machine_can_source_a_bounded_lot_without_inventing_commerce_demand() {
    let (w, district) = prepared();
    let before = save(&w);
    let quote = materials::quote(&w, ME, &district, STARTER_PACKS, WINDOW);
    assert!(quote.can_start && quote.feasible_today >= STARTER_PACKS / WINDOW as f64);
    let command = economic_ai::materials_order_candidate(&w, ME)
        .expect("A fully supplied first-machinery intention should use inherited Materials before another processor");
    match command {
        Command::OrderMaterials {
            nation,
            district: chosen,
            quantity,
            delivery_days,
        } => {
            assert_eq!(nation, ME);
            assert_eq!(chosen, district);
            assert_eq!(quantity, STARTER_PACKS);
            assert_eq!(delivery_days, WINDOW);
        }
        _ => panic!("Expected the finite paid Materials command"),
    }
    assert_eq!(
        save(&w),
        before,
        "A proposal may not grant stock, GDP, cash or consume RNG"
    );
    assert_eq!(commerce::demand(&w, ME, Good::Intermediates), 0.0);
}

#[test]
fn the_review_pairs_the_order_with_one_machine_not_a_duplicate_processor_or_import() {
    let (mut w, district) = prepared();
    consenting_import_alternative(&mut w);
    freeze_other_reviews(&mut w);
    let pc = w.nation(ME).political_capital;
    let gdp = w.nation(ME).gdp;
    let stock = commerce::stock(&w, ME, Good::Intermediates);
    let rng = w.rng.clone();
    let machine_price = production::catalog(K::MachineryWorks).political_cost;
    economic_ai::tick(&mut w);
    let orders = &w
        .materials
        .as_ref()
        .expect("AI must commission the viable inherited supply")
        .orders;
    assert_eq!(orders.len(), 1);
    assert_eq!(orders[0].quantity, STARTER_PACKS);
    assert_eq!(orders[0].delivered, 0.0);
    let projects: Vec<_> = production::projects_for(&w, ME).collect();
    assert_eq!(
        projects.len(),
        1,
        "One strategic project, not a processor plus a machine"
    );
    assert_eq!(projects[0].kind, K::MachineryWorks);
    assert_eq!(projects[0].district, district);
    assert_eq!(
        w.nation(ME).political_capital,
        pc - materials::ORDER_PC - machine_price
    );
    assert_eq!(w.nation(ME).gdp, gdp);
    assert_eq!(w.rng, rng);
    assert_eq!(commerce::stock(&w, ME, Good::Intermediates), stock);
    assert!(
        w.commerce.as_ref().is_none_or(|c| c.contracts.is_empty()),
        "Do not import a duplicate startup lot"
    );
    assert!(
        economic_ai::materials_order_candidate(&w, ME).is_none(),
        "The queued machine's finite lot is already committed"
    );
    assert_eq!(
        economic_ai::export_reserve(&w, ME, Good::Intermediates),
        STARTER_PACKS,
        "Preserve the startup lot for its real commissioned consumer"
    );
    let settled = save(&w);
    economic_ai::tick(&mut w);
    assert_eq!(
        save(&w),
        settled,
        "Same-date review cannot buy another contract or factory"
    );
}

#[test]
fn an_existing_valid_startup_order_covers_one_first_machine_without_becoming_stock() {
    let (mut w, district) = prepared();
    apply_command(
        &mut w,
        &Command::OrderMaterials {
            nation: ME,
            district,
            quantity: STARTER_PACKS,
            delivery_days: WINDOW,
        },
    )
    .unwrap();
    assert!(economic_ai::materials_order_candidate(&w, ME).is_none());
    assert_eq!(economic_ai::candidate(&w, ME).unwrap().1, K::MachineryWorks);
    assert_eq!(commerce::stock(&w, ME, Good::Intermediates), 0.0);
    assert_eq!(
        production::projects_for(&w, ME).count(),
        0,
        "A read-only recommendation is not a grant"
    );
}

#[test]
fn bootstrap_requires_the_whole_raw_lot_not_just_one_feasible_day() {
    let (mut w, district) = prepared();
    let quote = materials::quote(&w, ME, &district, STARTER_PACKS, WINDOW);
    let one_day_iron = quote.inputs_daily[Commodity::Iron.idx()];
    assert!(one_day_iron > 0.0);
    set_raw(&mut w, Commodity::Iron, one_day_iron * 2.0);
    assert!(
        materials::quote(&w, ME, &district, STARTER_PACKS, WINDOW).feasible_today > 0.0,
        "This fixture distinguishes a feasible day from a feasible startup lot"
    );
    assert!(
        economic_ai::materials_order_candidate(&w, ME).is_none(),
        "Do not pair an unbacked 30-day promise with irreversible first-machine construction"
    );
    assert_eq!(
        economic_ai::candidate(&w, ME).unwrap().1,
        K::ProcessingPlant,
        "Without a backed domestic lot the ordinary producer fallback remains available"
    );
}

#[test]
fn bootstrap_keeps_enough_political_capital_for_both_commands_and_the_ai_reserve() {
    let (mut w, district) = prepared();
    let machine_price = production::catalog(K::MachineryWorks).political_cost;
    w.nation_mut(ME).political_capital = materials::ORDER_PC + machine_price + 8.0 - 0.25;
    assert!(
        materials::quote(&w, ME, &district, STARTER_PACKS, WINDOW).can_start,
        "The order alone is affordable; the paired strategic move is not"
    );
    assert!(economic_ai::materials_order_candidate(&w, ME).is_none());
    assert_eq!(
        economic_ai::candidate(&w, ME).unwrap().1,
        K::MachineryWorks,
        "Keep the physical first-machine target without committing unaffordable work"
    );
    economic_ai::evaluate(&mut w, ME);
    assert!(w.materials.as_ref().is_none_or(|m| m.orders.is_empty()));
    assert_eq!(production::projects_for(&w, ME).count(), 0);
}

#[test]
fn empty_daily_authority_keeps_the_machine_target_but_places_neither_half_of_the_pair() {
    let (mut w, _) = prepared();
    let plan = w.nation_mut(ME).program_budget.as_mut().unwrap();
    for department in [1, 2] {
        plan.available_bn[BUDGET_INDUSTRY][department] = 0.0;
        plan.prepaid_bn[BUDGET_INDUSTRY][department] = 0.0;
    }
    assert!(economic_ai::materials_order_candidate(&w, ME).is_none());
    assert_eq!(
        economic_ai::candidate(&w, ME).unwrap().1,
        K::MachineryWorks,
        "A transient funding wait must not redirect the budget into an unnecessary processor"
    );
    economic_ai::evaluate(&mut w, ME);
    assert!(w.materials.as_ref().is_none_or(|m| m.orders.is_empty()));
    assert_eq!(production::projects_for(&w, ME).count(), 0);
}

#[test]
fn absent_power_grid_or_inputs_never_turns_inherited_estimates_into_a_supply_promise() {
    for mode in 0..3 {
        let (mut w, district) = prepared();
        match mode {
            0 => w.production.industry.sites.get_mut(&district).unwrap()[1] = 0,
            1 => w.production.provinces[0].power_grid = 0,
            _ => set_raw(&mut w, Commodity::Iron, 0.0),
        }
        assert!(
            economic_ai::materials_order_candidate(&w, ME).is_none(),
            "mode {mode}"
        );
        assert!(
            matches!(
                economic_ai::candidate(&w, ME).unwrap().1,
                K::Generation | K::PowerGrid | K::ProcessingPlant
            ),
            "Keep the ordinary capacity/input fallback"
        );
    }
}

#[test]
fn player_and_fractional_specialist_do_not_get_an_ai_first_machine_imposed() {
    let (mut w, district) = prepared();
    freeze_other_reviews(&mut w);
    w.player = Some(ME);
    let before = save(&w);
    economic_ai::tick(&mut w);
    economic_ai::evaluate(&mut w, ME);
    assert_eq!(save(&w), before);
    assert!(
        economic_ai::materials_order_candidate(&w, ME).is_none(),
        "No unsolicited player proposal"
    );
    w.player = None;
    w.production.provinces.clear();
    w.production.industry.sites.clear();
    w.production.industry.modules.insert(district, 5000);
    assert!(
        economic_ai::materials_order_candidate(&w, ME).is_none(),
        "A fractional specialist is not obliged to build a full first machine shop"
    );
}

#[test]
fn a_preexisting_export_offer_protects_the_new_machine_startup_lot_immediately() {
    let (mut w, _) = prepared();
    apply_command(
        &mut w,
        &Command::SetGoodsSale {
            nation: ME,
            good: Good::Intermediates,
            reserve: 0.0,
            ask_multiplier: 1.05,
            enabled: true,
        },
    )
    .unwrap();
    // The existing AI-owned standing policy was last reviewed a month ago.
    w.economic_ai.nations.insert(
        ME,
        economic_ai::NationPlan {
            last_review_day: clock::absolute_day(&w) - economic_ai::REVIEW_DAYS,
            fiscal_year: w.year,
            offered_reserves: [Some(0.0), None],
            ..Default::default()
        },
    );
    economic_ai::evaluate(&mut w, ME);
    assert_eq!(
        production::projects_for(&w, ME).next().unwrap().kind,
        K::MachineryWorks
    );
    assert_eq!(
        commerce::sale(&w, ME, Good::Intermediates).unwrap().reserve,
        STARTER_PACKS,
        "Protect arriving packs now, not thirty days after commissioning their consumer"
    );
}

// Exercise real construction, operation, money and GDP settlement without
// unrelated war/diplomacy/AI reviews. This is component continuity, not a
// substitute for the full-world multi-year balance harness.
fn industrial_day(w: &mut WorldState) {
    province_economy::begin_day(w);
    programs::begin_day(w);
    production::tick_day(w);
    industry::tick_day(w);
    programs::finish_day(w);
    province_economy::finish_day(w);
    clock::advance_date(w);
}

#[test]
fn ai_order_delivers_exactly_fifteen_paid_packs_with_save_continuity() {
    let (mut w, _) = prepared();
    economic_ai::evaluate(&mut w, ME);
    assert_eq!(w.materials.as_ref().unwrap().orders.len(), 1);
    let before_iron = resources::stockpile(&w, ME, Commodity::Iron);
    for _ in 0..13 {
        industrial_day(&mut w);
    }
    assert!(commerce::stock(&w, ME, Good::Intermediates) > 0.0);
    let mut resumed = load(&save(&w)).unwrap();
    for _ in 13..WINDOW {
        industrial_day(&mut w);
        industrial_day(&mut resumed);
    }
    assert_eq!(save(&w), save(&resumed));
    let order = &w.materials.as_ref().unwrap().orders[0];
    assert_eq!(order.status, "completed");
    assert_eq!(order.remaining, 0.0);
    assert_eq!(order.delivered, STARTER_PACKS);
    assert_eq!(commerce::stock(&w, ME, Good::Intermediates), STARTER_PACKS);
    assert!(resources::stockpile(&w, ME, Commodity::Iron) < before_iron);
    assert!(
        (order.spent_conversion_bn - STARTER_PACKS * materials::CONVERSION_CASH_PER_PACK_BN).abs()
            < 1e-12
    );
    assert!(order.spent_energy_bn > 0.0);
    assert!(
        economic_ai::materials_order_candidate(&w, ME).is_none(),
        "Do not replace a fully delivered startup lot"
    );
    assert!(production::projects_for(&w, ME).all(|p| p.kind != K::ProcessingPlant));
}

#[test]
fn cancelled_or_expired_partial_orders_are_topped_up_only_for_the_missing_startup_packs() {
    for cancel in [true, false] {
        let (mut w, _) = prepared();
        economic_ai::evaluate(&mut w, ME);
        for _ in 0..5 {
            industrial_day(&mut w);
        }
        let delivered = commerce::stock(&w, ME, Good::Intermediates);
        assert_eq!(delivered, 2.5);
        if cancel {
            let id = w.materials.as_ref().unwrap().orders[0].id;
            apply_command(
                &mut w,
                &Command::CancelMaterialsOrder {
                    nation: ME,
                    order: id,
                },
            )
            .unwrap();
        } else {
            // A subsequent input loss is explicit fixture state, not a grant.
            set_raw(&mut w, Commodity::Iron, 0.0);
            for _ in 5..=WINDOW {
                industrial_day(&mut w);
            }
            set_raw(&mut w, Commodity::Iron, 1000.0);
            assert_eq!(w.materials.as_ref().unwrap().orders[0].status, "expired");
        }
        assert_eq!(materials::pending(&w, ME), 0.0);
        let command = economic_ai::materials_order_candidate(&w, ME)
            .expect("The queued machine still needs its undelivered startup remainder");
        if let Command::OrderMaterials { quantity, .. } = &command {
            assert_eq!(*quantity, STARTER_PACKS - delivered);
        } else {
            panic!("Expected a Materials top-up");
        }
        apply_command(&mut w, &command).unwrap();
        assert!(economic_ai::materials_order_candidate(&w, ME).is_none());
        assert_eq!(commerce::stock(&w, ME, Good::Intermediates), delivered);
        assert_eq!(production::projects_for(&w, ME).count(), 1);
    }
}

#[test]
fn a_stocked_warehouse_needs_only_the_net_seven_pack_machine_startup_topup() {
    let (mut w, district) = prepared();
    apply_command(
        &mut w,
        &Command::StartProject {
            nation: ME,
            district: district.clone(),
            kind: K::Warehouse,
        },
    )
    .unwrap();
    w.production
        .industry
        .goods
        .entry(ME)
        .or_default()
        .intermediates = 20.0;
    let command = economic_ai::materials_order_candidate(&w, ME)
        .expect("20 stocked packs cover 12 warehouse inputs plus 8 of the 15 startup lot");
    assert!(matches!(
        command,
        Command::OrderMaterials { quantity: 7.0, .. }
    ));
    economic_ai::evaluate(&mut w, ME);
    if production::projects_for(&w, ME).count() == 1 {
        // A fiscal retarget may use the first review. No physical work occurs
        // while this fixture moves to the next review; finite deadlines still apply.
        for _ in 0..economic_ai::REVIEW_DAYS {
            clock::advance_date(&mut w);
        }
        economic_ai::evaluate(&mut w, ME);
    }
    let kinds: Vec<_> = production::projects_for(&w, ME).map(|p| p.kind).collect();
    assert_eq!(
        kinds.len(),
        2,
        "Keep the warehouse and add exactly its machinery prerequisite"
    );
    assert!(kinds.contains(&K::Warehouse) && kinds.contains(&K::MachineryWorks));
    let orders = &w.materials.as_ref().unwrap().orders;
    assert_eq!(orders.len(), 1);
    assert_eq!(orders[0].quantity, 7.0);
    assert_eq!(commerce::stock(&w, ME, Good::Intermediates), 20.0);
    assert_eq!(
        commerce::sale(&w, ME, Good::Intermediates).unwrap().reserve,
        27.0,
        "The standing export policy protects warehouse inputs plus the machine startup lot"
    );
}
