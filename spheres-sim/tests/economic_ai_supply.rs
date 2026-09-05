//! Exact industrial-supply policy regressions. All factories, inventories,
//! cash and inputs below are explicit fixtures; inherited estimates never
//! become free goods. These are invariants, not historical calibration bars.

use spheres_sim::{
    apply_command, clock,
    commerce::{self, Good},
    economic_ai,
    init::world_1990,
    load, logistics, materials,
    production::{self, ProjectKind as K},
    programs, province_economy,
    resources::{self, Commodity},
    save, starting_industry, tick_day,
    world::{GameRules, NationId, WorldState, BUDGET_INDUSTRY},
    Command,
};

const ME: NationId = NationId::USA;
const SELLER: NationId = NationId::Canada;

fn near(actual: f64, expected: f64) {
    assert!((actual - expected).abs() < 1e-9, "{actual} != {expected}");
}

fn set_raw(w: &mut WorldState, nation: NationId, commodity: Commodity, quantity: f64) {
    let stocks = &mut w.resources.market.as_mut().unwrap().stocks;
    match stocks.binary_search_by_key(&(nation, commodity), |s| (s.nation, s.commodity)) {
        Ok(i) => stocks[i].quantity = quantity,
        Err(i) => stocks.insert(
            i,
            resources::Stock {
                nation,
                commodity,
                quantity,
                reserve_target: 0.0,
            },
        ),
    }
}

/// `focus` is an already-enacted Industry department priority. It prevents a
/// test about supply from spending its first review merely changing the budget.
fn prepared(machine: u8, processor: u8, focus: Option<usize>) -> (WorldState, String) {
    let mut w = world_1990(GameRules {
        daily_simulation: true,
        economic_competition: true,
        production_system: true,
        resource_gates: true,
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
    w.sanctions.clear();
    w.nation_mut(ME).political_capital = 1000.0;
    w.nation_mut(ME).debt_gdp = 0.0;
    let allocations = w.nation(ME).budget_for(w.year).allocations;
    let mut departments = programs::default_departments();
    if let Some(department) = focus {
        departments[BUDGET_INDUSTRY] = [1000; 5];
        departments[BUDGET_INDUSTRY][department] = 6000;
    }
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
    w.nation_mut(ME).treasury_bn = Some(100.0);
    w.nation_mut(ME).debt_bn = Some(0.0);

    let district = w
        .districts
        .iter()
        .filter(|(_, owner)| **owner == ME)
        .map(|(district, _)| district.clone())
        .find(|district| materials::capacity_daily(&w, district) >= 0.5)
        .expect("USA fixture needs one located inherited Materials province");
    let mut found = false;
    for province in w
        .production
        .provinces
        .iter_mut()
        .filter(|province| province.district == district)
    {
        found = true;
        province.civilian_industry = 1;
        province.power_grid = 3;
    }
    if !found {
        w.production
            .provinces
            .push(production::ProvinceCapabilities {
                district: district.clone(),
                civilian_industry: 1,
                power_grid: 3,
                infrastructure: 0,
                research_centers: 0,
                arms_plants: 0,
            });
    }
    w.production
        .provinces
        .sort_by(|a, b| a.district.cmp(&b.district));
    assert_eq!(
        production::level(&w, &district, K::CivilianIndustry),
        1,
        "explicit supply fixture needs an installed estate; matching rows={:?}",
        w.production
            .provinces
            .iter()
            .filter(|province| province.district == district)
            .collect::<Vec<_>>()
    );
    // Build today's raw market before adding the explicit industrial sites.
    // That prevents an unrelated automatic shipment from contaminating a
    // fixture that is meant to choose its own raw stock and access state.
    resources::tick(&mut w);
    // Site order: machinery, generation, processing, freight, warehouse,
    // automation, efficiency.
    w.production
        .industry
        .sites
        .insert(district.clone(), [machine, 2, processor, 0, 0, 0, 0]);
    for commodity in resources::ALL {
        set_raw(&mut w, ME, commodity, 1000.0);
    }
    programs::begin_day(&mut w);
    (w, district)
}

fn line(w: &WorldState, good: Good) -> economic_ai::SupplyLine {
    let before = save(w);
    let forecast = economic_ai::supply_forecast(w, ME);
    assert_eq!(forecast.as_of_day, clock::absolute_day(w));
    assert_eq!(forecast.horizon_days, economic_ai::SUPPLY_HORIZON_DAYS);
    assert_eq!(
        save(w),
        before,
        "forecasting cannot mutate the world or RNG"
    );
    forecast
        .lines
        .into_iter()
        .find(|row| row.good == good)
        .expect("forecast contains both manufactured goods")
}

fn consenting_seller(w: &mut WorldState, good: Good, quantity: f64) {
    w.nation_mut(SELLER).political_capital = 1000.0;
    let allocations = w.nation(SELLER).budget_for(w.year).allocations;
    apply_command(
        w,
        &Command::SetProgramBudget {
            nation: SELLER,
            fiscal_year: 1990,
            allocations,
            departments: programs::default_departments(),
        },
    )
    .unwrap();
    w.nation_mut(SELLER).treasury_bn = Some(100.0);
    w.nation_mut(SELLER).debt_bn = Some(0.0);
    let goods = w.production.industry.goods.entry(SELLER).or_default();
    match good {
        Good::Intermediates => goods.intermediates = quantity,
        Good::CapitalGoods => goods.capital_goods = quantity,
    }
    apply_command(
        w,
        &Command::SetGoodsSale {
            nation: SELLER,
            good,
            reserve: 0.0,
            ask_multiplier: 1.0,
            enabled: true,
        },
    )
    .unwrap();
}

#[test]
fn ninety_day_forecast_counts_recurring_use_three_times_but_project_goods_once() {
    let (mut w, district) = prepared(1, 0, Some(3));
    apply_command(
        &mut w,
        &Command::StartProject {
            nation: ME,
            district,
            kind: K::Warehouse,
        },
    )
    .unwrap();

    let intermediate = line(&w, Good::Intermediates);
    near(intermediate.operating_daily, 0.5);
    near(intermediate.project_remaining, 12.0);
    near(intermediate.startup_reserve, 0.0);
    near(intermediate.target, 57.0); // 90 * 0.5 + the one-off twelve.
    near(intermediate.coverage, 0.0);
    near(intermediate.shortage, 57.0);
    assert!(!intermediate.status.is_empty() && !intermediate.reason.is_empty());

    let capital = line(&w, Good::CapitalGoods);
    near(capital.operating_daily, 0.0);
    near(capital.project_remaining, 5.0);
    near(capital.target, 5.0);
    near(capital.shortage, 5.0);
}

#[test]
fn forecast_nets_stock_imports_and_domestic_contracts_exactly_once() {
    let (mut w, district) = prepared(1, 0, Some(0));
    w.production
        .industry
        .goods
        .entry(ME)
        .or_default()
        .intermediates = 5.0;
    consenting_seller(&mut w, Good::Intermediates, 100.0);
    apply_command(
        &mut w,
        &Command::ProposeGoodsTrade {
            buyer: ME,
            seller: SELLER,
            good: Good::Intermediates,
            quantity: 10.0,
            unit_price_bn: commerce::reference_price_bn(Good::Intermediates),
            delivery_days: 90,
        },
    )
    .unwrap();
    apply_command(
        &mut w,
        &Command::OrderMaterials {
            nation: ME,
            district,
            quantity: 15.0,
            delivery_days: 30,
        },
    )
    .unwrap();

    let row = line(&w, Good::Intermediates);
    near(row.target, 45.0);
    near(row.stock, 5.0);
    near(row.imports, 10.0);
    near(row.domestic_contracts, 15.0);
    near(row.projected_domestic, 0.0);
    near(row.coverage, 30.0);
    near(row.shortage, 15.0);
    near(row.storage_headroom, 220.0);
}

#[test]
fn domestic_stock_cover_prevents_an_order_import_or_processor() {
    let (mut w, _) = prepared(1, 0, Some(0));
    w.production
        .industry
        .goods
        .entry(ME)
        .or_default()
        .intermediates = 45.0;
    consenting_seller(&mut w, Good::Intermediates, 100.0);
    let row = line(&w, Good::Intermediates);
    near(row.target, 45.0);
    near(row.shortage, 0.0);

    economic_ai::evaluate(&mut w, ME);
    assert!(w.materials.as_ref().is_none_or(|m| m.orders.is_empty()));
    assert!(w
        .commerce
        .as_ref()
        .is_none_or(|c| { c.contracts.iter().all(|contract| contract.buyer != ME) }));
    assert!(production::projects_for(&w, ME)
        .all(|p| !matches!(p.kind, K::ProcessingPlant | K::StarterIndustry)));
}

#[test]
fn import_wait_uses_the_same_gdp_cap_as_the_actual_purchase() {
    let (mut w, _) = prepared(0, 0, Some(2));
    set_raw(&mut w, ME, Commodity::Iron, 0.0);
    consenting_seller(&mut w, Good::Intermediates, 100.0);
    assert!(
        !commerce::market_quotes(&w, ME, Good::Intermediates, 15.0, 30).is_empty(),
        "the seller and treasury can quote before the AI's separate GDP risk cap"
    );
    w.nation_mut(ME).gdp = 0.0;

    let (_, kind, reason) = economic_ai::candidate(&w, ME).unwrap();
    assert_eq!(
        kind,
        K::ProcessingPlant,
        "a zero executable import tranche cannot suppress the physical fallback"
    );
    assert!(!reason.contains("Accumulate"));
}

#[test]
fn inherited_materials_places_one_finite_tranche_before_buying_abroad() {
    let (mut w, district) = prepared(1, 0, Some(0));
    consenting_seller(&mut w, Good::Intermediates, 100.0);
    assert!(
        commerce::market_quotes(&w, ME, Good::Intermediates, 45.0, 90)
            .iter()
            .any(|quote| quote.seller == SELLER && quote.accepted)
    );

    economic_ai::evaluate(&mut w, ME);
    let orders = &w
        .materials
        .as_ref()
        .unwrap_or_else(|| {
            panic!(
                "domestic order ledger; candidate={:?}; review={:?}",
                economic_ai::materials_order_candidate(&w, ME),
                w.economic_ai.nations.get(&ME)
            )
        })
        .orders;
    assert_eq!(orders.len(), 1);
    assert_eq!(orders[0].district, district);
    assert_eq!(orders[0].delivery_days, economic_ai::REVIEW_DAYS as u32);
    assert!(orders[0].quantity > 0.0);
    assert!(
        orders[0].quantity
            <= materials::capacity_daily(&w, &orders[0].district) * economic_ai::REVIEW_DAYS as f64
                + 1e-9
    );
    assert!(w
        .commerce
        .as_ref()
        .is_none_or(|c| { c.contracts.iter().all(|contract| contract.buyer != ME) }));
    assert!(production::projects_for(&w, ME).all(|p| p.kind != K::ProcessingPlant));

    let once = save(&w);
    economic_ai::evaluate(&mut w, ME);
    assert_eq!(save(&w), once, "one review cannot duplicate its supply lot");
}

#[test]
fn domestic_order_never_multiplies_one_day_of_raw_stock_into_a_monthly_promise() {
    let (mut w, expected_district) = prepared(1, 0, Some(0));
    // A Processing Plant needs 0.2 bauxite per pack. This is enough for two
    // whole packs, not two packs every day for the thirty-day contract.
    set_raw(&mut w, ME, Commodity::Bauxite, 0.4);

    let command = economic_ai::materials_order_candidate(&w, ME)
        .expect("owned raw stock can back a small, finite domestic lot");
    let (district, quantity, delivery_days) = match command {
        Command::OrderMaterials {
            nation,
            district,
            quantity,
            delivery_days,
        } => {
            assert_eq!(nation, ME);
            (district, quantity, delivery_days)
        }
        other => panic!("expected a Materials order, got {other:?}"),
    };
    assert_eq!(district, expected_district);
    assert_eq!(delivery_days, economic_ai::REVIEW_DAYS as u32);
    assert!(quantity > 0.0 && quantity <= 2.0 + 1e-9, "quantity={quantity}");
    let quote = materials::quote(&w, ME, &district, quantity, delivery_days);
    let full_lot_bauxite = quote.inputs_daily[Commodity::Bauxite.idx()]
        * delivery_days as f64;
    assert!(
        full_lot_bauxite <= resources::stockpile(&w, ME, Commodity::Bauxite) + 1e-9,
        "a finite AI contract must own its whole raw bundle: {full_lot_bauxite}"
    );
}

#[test]
fn accepted_import_is_replanned_before_a_stale_processor_can_start() {
    let (mut w, _) = prepared(1, 0, Some(0));
    // Keep inherited conversion from competing with the import in this case.
    set_raw(&mut w, ME, Commodity::Iron, 0.0);
    w.production
        .industry
        .goods
        .entry(ME)
        .or_default()
        .intermediates = 16.0;
    consenting_seller(&mut w, Good::Intermediates, 100.0);
    near(line(&w, Good::Intermediates).shortage, 29.0);

    economic_ai::evaluate(&mut w, ME);
    let contracts: Vec<_> = w
        .commerce
        .as_ref()
        .unwrap()
        .contracts
        .iter()
        .filter(|contract| contract.buyer == ME && contract.good == Good::Intermediates)
        .collect();
    assert_eq!(
        contracts.len(),
        1,
        "one funded 90-day supply purchase; review={:?}",
        w.economic_ai.nations.get(&ME)
    );
    assert_eq!(contracts[0].seller, SELLER);
    assert!(contracts[0].quantity > 0.0);
    assert!(
        production::projects_for(&w, ME).all(|p| p.kind != K::ProcessingPlant),
        "a successful import must be reflected before the old processor candidate is executed"
    );
    assert!(w.resources.mine_projects.iter().all(|p| p.started_by != ME));
}

#[test]
fn a_partial_quote_accumulates_paid_startup_stock_before_machinery_is_eligible() {
    let (mut w, _) = prepared(0, 0, Some(2));
    // Make inherited Materials infeasible without fabricating a processor.
    set_raw(&mut w, ME, Commodity::Iron, 0.0);
    consenting_seller(&mut w, Good::Intermediates, 7.0);
    // Reproduce a government that was already exporting surplus before it
    // began the first-machine plan. The paid partial lot must replace this
    // reserve-zero policy rather than being sold back out between reviews.
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

    let before = line(&w, Good::Intermediates);
    near(before.operating_daily, 0.0);
    near(before.startup_reserve, 15.0);
    near(before.target, 15.0);
    near(before.shortage, 15.0);
    near(
        economic_ai::export_reserve(&w, ME, Good::Intermediates),
        0.0,
    );

    economic_ai::evaluate(&mut w, ME);
    let contracts = &w.commerce.as_ref().unwrap().contracts;
    assert_eq!(contracts.len(), 1);
    near(contracts[0].quantity, 7.0);
    assert!(production::projects_for(&w, ME)
        .all(|p| { !matches!(p.kind, K::MachineryWorks | K::ProcessingPlant) }));
    assert!(w.resources.mine_projects.iter().all(|p| p.started_by != ME));
    let review = w
        .economic_ai
        .nations
        .get(&ME)
        .and_then(|plan| plan.supply_review.as_ref())
        .expect("the accepted partial lot is visible in the saved review");
    near(
        review
            .lines
            .iter()
            .find(|row| row.good == Good::Intermediates)
            .unwrap()
            .shortage,
        8.0,
    );
    near(
        economic_ai::export_reserve(&w, ME, Good::Intermediates),
        15.0,
    );
    near(
        commerce::sale(&w, ME, Good::Intermediates)
            .expect("the inbound starter lot refreshes the live export policy")
            .reserve,
        15.0,
    );

    // Isolate the next policy review from unrelated world ticks. A second
    // executable lot may finish the paid reserve; it still cannot queue the
    // dependent machine in the same review as that trade.
    w.production
        .industry
        .goods
        .entry(SELLER)
        .or_default()
        .intermediates = 8.0;
    w.economic_ai.nations.get_mut(&ME).unwrap().last_review_day =
        clock::absolute_day(&w) - economic_ai::REVIEW_DAYS;
    economic_ai::evaluate(&mut w, ME);
    let contracts = &w.commerce.as_ref().unwrap().contracts;
    assert_eq!(contracts.len(), 2);
    near(
        contracts.iter().map(|contract| contract.quantity).sum(),
        15.0,
    );
    assert!(production::projects_for(&w, ME)
        .all(|p| { !matches!(p.kind, K::MachineryWorks | K::ProcessingPlant) }));
    near(line(&w, Good::Intermediates).shortage, 0.0);
    assert_eq!(
        economic_ai::candidate(&w, ME).unwrap().1,
        K::MachineryWorks,
        "machinery becomes eligible only after the full paid starter lot exists"
    );
}

#[test]
fn a_successful_domestic_materials_order_cannot_also_start_a_mine() {
    let (mut w, district) = prepared(1, 0, Some(0));
    apply_command(
        &mut w,
        &Command::StartProject {
            nation: ME,
            district,
            kind: K::CivilianIndustry,
        },
    )
    .unwrap();
    set_raw(&mut w, ME, Commodity::RareEarths, 0.0);
    for producer in resources::producers(&w, Commodity::RareEarths) {
        if producer != ME {
            w.sanctions.push((producer, ME));
        }
    }
    assert!(w
        .districts
        .iter()
        .filter(|(_, owner)| **owner == ME)
        .any(|(district, _)| {
            resources::mine_refusal(&w, ME, district, Commodity::RareEarths).is_none()
        }));
    assert!(economic_ai::materials_order_candidate(&w, ME).is_some());
    spheres_sim::arsenal::tick(&mut w);
    assert!(resources::tick_draw(&w, ME)[Commodity::RareEarths.idx()] > 0.0);

    economic_ai::evaluate(&mut w, ME);
    assert_eq!(w.materials.as_ref().unwrap().orders.len(), 1);
    assert!(
        w.resources.mine_projects.iter().all(|p| p.started_by != ME),
        "one review may place its finite domestic order, but must not also start irreversible mine work"
    );
}

#[test]
fn reachable_raw_trade_prevents_a_mine_even_while_paid_work_is_active() {
    let (mut w, district) = prepared(0, 0, Some(0));
    apply_command(
        &mut w,
        &Command::StartProject {
            nation: ME,
            district,
            kind: K::CivilianIndustry,
        },
    )
    .unwrap();
    for commodity in resources::ALL {
        set_raw(&mut w, ME, commodity, 0.0);
        for producer in resources::producers(&w, commodity) {
            if producer != ME {
                w.shift_relation(ME, producer, 200.0);
            }
        }
    }
    let draw = resources::tick_draw(&w, ME);
    let needed: Vec<_> = resources::ALL
        .into_iter()
        .filter(|commodity| draw[commodity.idx()] > 0.0)
        .collect();
    assert!(!needed.is_empty());
    assert!(needed
        .iter()
        .all(|commodity| resources::open_holder(&w, ME, *commodity).is_some()));
    spheres_sim::arsenal::tick(&mut w);
    assert_eq!(
        w.resources.market.as_ref().unwrap().last_cleared_day,
        Some(clock::absolute_day(&w)),
        "the ordinary raw market must actually get its chance before AI review"
    );

    economic_ai::evaluate(&mut w, ME);
    assert!(
        w.resources.mine_projects.iter().all(|p| p.started_by != ME),
        "a reachable raw market must be tried before irreversible mine development"
    );
}

#[test]
fn closed_raw_market_can_start_one_mapped_mine_without_an_active_project() {
    let (mut w, _) = prepared(1, 1, None);
    w.production
        .industry
        .goods
        .entry(ME)
        .or_default()
        .intermediates = 45.0;
    for commodity in resources::ALL {
        set_raw(&mut w, ME, commodity, 1000.0);
    }
    // USA has large inherited iron production, so an empty iron warehouse is
    // correctly replenished by domestic flow. Bauxite is a real installed
    // processor input with no USA baseline flow and a mapped owned deposit.
    set_raw(&mut w, ME, Commodity::Bauxite, 0.0);
    for producer in resources::producers(&w, Commodity::Bauxite) {
        if producer != ME {
            w.sanctions.push((producer, ME));
        }
    }
    assert!(resources::open_holder(&w, ME, Commodity::Bauxite).is_none());
    spheres_sim::arsenal::tick(&mut w);
    assert_eq!(
        w.resources.market.as_ref().unwrap().last_cleared_day,
        Some(clock::absolute_day(&w))
    );
    assert!(
        resources::tick_draw(&w, ME)[Commodity::Bauxite.idx()] > 0.0,
        "the installed processor must expose a real bauxite operating draw"
    );
    near(resources::stockpile(&w, ME, Commodity::Bauxite), 0.0);
    let mine_district = w
        .districts
        .iter()
        .filter(|(_, owner)| **owner == ME)
        .map(|(district, _)| district.clone())
        .find(|district| resources::mine_refusal(&w, ME, district, Commodity::Bauxite).is_none())
        .expect("USA has a mapped bauxite deposit eligible for development");
    let mut executable = w.clone();
    apply_command(
        &mut executable,
        &Command::DevelopResource {
            nation: ME,
            district: mine_district.clone(),
            commodity: Commodity::Bauxite,
        },
    )
    .expect("the exact fallback mine is executable");
    assert_eq!(production::projects_for(&w, ME).count(), 0);

    economic_ai::evaluate(&mut w, ME);
    let mines: Vec<_> = w
        .resources
        .mine_projects
        .iter()
        .filter(|p| p.started_by == ME)
        .collect();
    assert_eq!(
        mines.len(),
        1,
        "closed raw market should choose the eligible deposit; review={:?}",
        w.economic_ai.nations.get(&ME)
    );
    assert_eq!(mines[0].district, mine_district);
    assert_eq!(mines[0].commodity, Commodity::Bauxite);
    assert!(production::projects_for(&w, ME).all(|p| p.kind != K::ProcessingPlant));
}

#[test]
fn route_less_foreign_supply_creates_no_dead_contract_or_war_evidence_and_allows_a_mine() {
    let (mut w, _) = prepared(1, 1, None);
    w.production
        .industry
        .goods
        .entry(ME)
        .or_default()
        .intermediates = 45.0;
    for commodity in resources::ALL {
        set_raw(&mut w, ME, commodity, 1000.0);
    }
    set_raw(&mut w, ME, Commodity::Bauxite, 0.0);
    logistics::set_policy(&mut w, ME, logistics::RoutePolicy::LandOnly).unwrap();
    let producers = resources::producers(&w, Commodity::Bauxite);
    let unreachable = producers
        .iter()
        .copied()
        .filter(|producer| *producer != ME)
        .find(|producer| logistics::plan(&w, *producer, ME).is_err())
        .expect("the land-only USA fixture needs an overseas bauxite producer");
    for producer in producers {
        if producer != unreachable && producer != ME {
            w.nation_mut(producer).alive = false;
        }
    }
    spheres_sim::arsenal::tick(&mut w);
    assert_eq!(
        w.resources.market.as_ref().unwrap().last_cleared_day,
        Some(clock::absolute_day(&w))
    );
    resources::ai_purchases(&mut w);
    assert!(w.resources.contracts.iter().all(|contract| {
        !((contract.from == ME || contract.to == ME)
            && contract.give.iter().chain(&contract.take).any(|leg| {
                matches!(leg, resources::Leg::Commodity { c, .. } if *c == Commodity::Bauxite)
            }))
    }));
    assert!(w.resources.refusals.iter().all(|row| {
        row.buyer != ME || row.c != Commodity::Bauxite
    }), "route failure is not a diplomatic refusal");

    w.player = Some(unreachable);
    w.resources.next_id += 1;
    let offer_id = w.resources.next_id;
    w.resources.offers.push(resources::Offer {
        id: offer_id,
        from: ME,
        to: unreachable,
        give: vec![resources::Leg::Money { bn_per_year: 1.0 }],
        take: vec![resources::Leg::Commodity {
            c: Commodity::Bauxite,
            per_month: 1.0,
        }],
        months: 36,
        expires: resources::month_abs(&w) + 6,
    });
    w.sanctions.push((unreachable, ME));
    assert!(resources::offer_refusal(&w, unreachable, offer_id).is_some());
    let refusal_rows = w.resources.refusals.clone();

    economic_ai::evaluate(&mut w, ME);
    assert!(w.resources.mine_projects.iter().any(|mine| {
        mine.started_by == ME && mine.commodity == Commodity::Bauxite
    }));
    assert!(w.resources.refusals.iter().all(|row| {
        row.buyer != ME || row.c != Commodity::Bauxite
    }));
    assert_eq!(w.resources.refusals, refusal_rows);
}

#[test]
fn a_contract_signed_after_clearing_blocks_a_same_review_mine() {
    let (mut w, _) = prepared(1, 1, None);
    w.production
        .industry
        .goods
        .entry(ME)
        .or_default()
        .intermediates = 45.0;
    for commodity in resources::ALL {
        set_raw(&mut w, ME, commodity, 1000.0);
    }
    set_raw(&mut w, ME, Commodity::Bauxite, 0.0);
    let producers: Vec<_> = resources::producers(&w, Commodity::Bauxite)
        .into_iter()
        .filter(|producer| *producer != ME)
        .collect();
    // Close the spot market for this already-settled date, then reopen
    // diplomacy before the strategic recurring-contract pass.
    for producer in &producers {
        w.sanctions.push((*producer, ME));
    }
    spheres_sim::arsenal::tick(&mut w);
    w.sanctions
        .retain(|(from, to)| !(*to == ME && producers.contains(from)));
    for producer in producers {
        w.shift_relation(ME, producer, 200.0);
    }
    resources::ai_purchases(&mut w);
    assert!(
        resources::has_new_inbound_contract(&w, ME, Commodity::Bauxite),
        "the ordinary recurring buy pass should sign a current-month remedy; contracts={:?} offers={:?} refusals={:?} forecast={:?}",
        w.resources.contracts,
        w.resources.offers,
        w.resources.refusals,
        economic_ai::raw_supply_forecast(&w, ME).lines[Commodity::Bauxite.idx()]
    );
    let mut invalidated = w.clone();
    for producer in resources::producers(&invalidated, Commodity::Bauxite) {
        if producer != ME {
            invalidated.sanctions.push((producer, ME));
        }
    }
    assert!(!resources::has_new_inbound_contract(
        &invalidated,
        ME,
        Commodity::Bauxite
    ));
    let refusals = invalidated.resources.refusals.clone();
    economic_ai::evaluate(&mut invalidated, ME);
    assert!(invalidated.resources.mine_projects.iter().any(|mine| {
        mine.started_by == ME && mine.commodity == Commodity::Bauxite
    }));
    assert_eq!(invalidated.resources.refusals, refusals);

    economic_ai::evaluate(&mut w, ME);
    assert!(
        w.resources.mine_projects.iter().all(|mine| mine.started_by != ME),
        "one strategic review may not sign a raw contract and start a mine"
    );
}

#[test]
fn warehouse_requires_real_turnover_and_unsold_output_does_not_justify_one() {
    let (mut useful, useful_district) = prepared(10, 10, Some(3));
    useful
        .production
        .industry
        .goods
        .entry(ME)
        .or_default()
        .intermediates = 210.0;
    apply_command(
        &mut useful,
        &Command::OrderMaterials {
            nation: ME,
            district: useful_district,
            quantity: 15.0,
            delivery_days: 30,
        },
    )
    .unwrap();
    set_raw(&mut useful, ME, Commodity::Iron, 0.0);
    let row = line(&useful, Good::Intermediates);
    near(row.target, 450.0);
    near(row.storage_capacity, 250.0);
    near(row.storage_headroom, 25.0);
    economic_ai::evaluate(&mut useful, ME);
    assert_eq!(
        production::projects_for(&useful, ME)
            .filter(|p| p.kind == K::Warehouse)
            .count(),
        1,
        "a used 90-day buffer that cannot fit has a real storage case"
    );

    let (mut unsold, _) = prepared(0, 1, None);
    unsold
        .production
        .industry
        .goods
        .entry(ME)
        .or_default()
        .intermediates = 250.0;
    let row = line(&unsold, Good::Intermediates);
    near(row.operating_daily, 0.0);
    near(row.startup_reserve, 15.0);
    near(row.target, 15.0);
    near(row.shortage, 0.0);
    economic_ai::evaluate(&mut unsold, ME);
    assert!(
        production::projects_for(&unsold, ME).all(|p| p.kind != K::Warehouse),
        "a full pile without a consumer should be sold or idled, not enlarged"
    );
}

#[test]
fn export_policy_protects_one_review_tranche_not_the_entire_forecast() {
    let (mut w, district) = prepared(1, 0, Some(2));
    let forecast = line(&w, Good::Intermediates);
    near(forecast.target, 45.0);
    near(
        economic_ai::export_reserve(&w, ME, Good::Intermediates),
        15.0,
    );

    apply_command(
        &mut w,
        &Command::StartProject {
            nation: ME,
            district: district.clone(),
            kind: K::Warehouse,
        },
    )
    .unwrap();
    near(
        economic_ai::export_reserve(&w, ME, Good::Intermediates),
        27.0,
    );
    w.production.projects.clear();
    w.production.industry.projects.clear();
    w.production.industry.sites.get_mut(&district).unwrap()[0] = 0;
    near(
        economic_ai::export_reserve(&w, ME, Good::Intermediates),
        0.0,
    );
}

#[test]
fn cached_export_reserve_cannot_hide_a_stale_or_disabled_sale_policy() {
    let (mut w, _) = prepared(1, 0, Some(2));
    w.production
        .industry
        .goods
        .entry(ME)
        .or_default()
        .intermediates = 100.0;
    apply_command(
        &mut w,
        &Command::SetGoodsSale {
            nation: ME,
            good: Good::Intermediates,
            reserve: 0.0,
            ask_multiplier: 2.0,
            enabled: false,
        },
    )
    .unwrap();
    let mut plan = economic_ai::NationPlan {
        last_review_day: -economic_ai::REVIEW_DAYS,
        fiscal_year: w.year,
        ..Default::default()
    };
    plan.offered_reserves[0] = Some(15.0);
    w.economic_ai.nations.insert(ME, plan);

    economic_ai::evaluate(&mut w, ME);
    let sale = commerce::sale(&w, ME, Good::Intermediates)
        .expect("the AI repairs the actual standing order, not only its cache");
    assert!(sale.enabled);
    near(sale.reserve, 15.0);
    near(sale.ask_multiplier, 1.05);
}

#[test]
fn supply_actions_replay_through_freight_and_review_boundaries() {
    let (mut a, _) = prepared(1, 0, Some(0));
    set_raw(&mut a, ME, Commodity::Iron, 0.0);
    a.production
        .industry
        .goods
        .entry(ME)
        .or_default()
        .intermediates = 16.0;
    consenting_seller(&mut a, Good::Intermediates, 100.0);
    economic_ai::evaluate(&mut a, ME);
    assert!(
        a.commerce
            .as_ref()
            .is_some_and(|c| c.contracts.iter().any(|contract| contract.buyer == ME)),
        "review={:?}",
        a.economic_ai.nations.get(&ME)
    );

    // Keep this focused on the chosen country's supply loop. The normal AI
    // review gate remains serialized and deterministic for every other state.
    let today = clock::absolute_day(&a);
    for id in a
        .nations
        .iter()
        .filter(|n| n.alive && n.id != ME)
        .map(|n| n.id)
        .collect::<Vec<_>>()
    {
        a.economic_ai.nations.insert(
            id,
            economic_ai::NationPlan {
                last_review_day: today + 365,
                fiscal_year: a.year,
                ..Default::default()
            },
        );
    }
    let mut b = load(&save(&a)).unwrap();
    for day in 0..45 {
        tick_day(&mut a, &[]);
        tick_day(&mut b, &[]);
        if day == 17 || day == 31 {
            b = load(&save(&b)).unwrap();
        }
    }
    assert_eq!(save(&a), save(&b));
    assert_eq!(a.rng, b.rng);
    assert!(a.commerce.as_ref().is_some_and(|commerce| commerce
        .contracts
        .iter()
        .any(|contract| contract.buyer == ME && contract.delivered_quantity > 0.0)),
        "the exact replay crosses actual freight arrival even when the live machine consumes delivered packs");
}

#[test]
fn player_and_default_paths_remain_byte_inert() {
    let (mut player, _) = prepared(1, 0, Some(2));
    consenting_seller(&mut player, Good::Intermediates, 100.0);
    player.player = Some(ME);
    let before = save(&player);
    let _ = economic_ai::supply_forecast(&player, ME);
    economic_ai::evaluate(&mut player, ME);
    assert_eq!(save(&player), before);

    let mut legacy = world_1990(GameRules::default());
    let before = save(&legacy);
    let _ = economic_ai::supply_forecast(&legacy, ME);
    economic_ai::evaluate(&mut legacy, ME);
    assert_eq!(save(&legacy), before);
    assert!(!before.contains("\"supply_review\""));
}
