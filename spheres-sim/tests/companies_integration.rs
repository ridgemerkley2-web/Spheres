//! Company ownership and procurement boundaries. Opening funds, raw stocks and
//! facilities are synthetic test endowments; development and equipment thereafter
//! must be paid for and completed by the same dated commands used by the game.
use spheres_sim::world::{
    Belligerent, Conflict, GameRules, NationId as N, Objective, WorldState, BUDGET_DEFENSE as D,
};
use spheres_sim::{
    arsenal, clock, companies, equipment, manufacturing, operations, production, programs,
    resources, Command,
};

const HOME: N = N::France;

fn near(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < 1e-9,
        "{actual:.15} != {expected:.15}"
    );
}

fn fixture() -> (WorldState, String) {
    let mut w = spheres_sim::init::world_1990(GameRules {
        daily_simulation: true,
        military_operations: true,
        production_system: true,
        manufacturing_system: true,
        resource_market: true,
        resource_gates: true,
        ai_aggression: 0.0,
        crisis_intensity: 0.0,
        ..Default::default()
    });
    w.player = Some(HOME);
    w.conflicts.clear();
    w.nation_mut(HOME).political_capital = 1000.0;
    programs::set_construction_budget(&mut w, HOME, 0.0).unwrap();
    resources::tick(&mut w);
    let district = w
        .districts
        .iter()
        .find(|(_, owner)| **owner == HOME)
        .unwrap()
        .0
        .clone();
    if let Some(site) = w
        .production
        .provinces
        .iter_mut()
        .find(|site| site.district == district)
    {
        site.arms_plants = 1;
    } else {
        w.production
            .provinces
            .push(production::ProvinceCapabilities {
                district: district.clone(),
                arms_plants: 1,
                infrastructure: 0,
                civilian_industry: 0,
                power_grid: 0,
                research_centers: 0,
            });
        w.production
            .provinces
            .sort_by(|a, b| a.district.cmp(&b.district));
    }
    let market = w.resources.market.as_mut().unwrap();
    for commodity in resources::ALL {
        if let Some(stock) = market
            .stocks
            .iter_mut()
            .find(|row| row.nation == HOME && row.commodity == commodity)
        {
            stock.quantity = 1_000_000.0;
        } else {
            market.stocks.push(resources::Stock {
                nation: HOME,
                commodity,
                quantity: 1_000_000.0,
                reserve_target: 0.0,
            });
        }
    }
    market
        .stocks
        .sort_by_key(|stock| (stock.nation, stock.commodity));
    let n = w.nation_mut(HOME);
    n.treasury_bn = Some(100.0);
    n.debt_bn = Some(0.0);
    n.debt_gdp = 0.0;
    n.arsenal.held.clear();
    n.arsenal.orders.clear();
    n.arsenal.banked = 0.0;
    let plan = n.program_budget.as_mut().unwrap();
    plan.available_bn[D][3] = 10.0;
    plan.available_bn[D][4] = 10.0;
    plan.prepaid_bn[D][3] = 0.0;
    (w, district)
}

fn apply(w: &mut WorldState, order: companies::CompanyOrder) {
    spheres_sim::apply_command(
        w,
        &Command::Company {
            nation: HOME,
            order,
        },
    )
    .unwrap();
}

fn net_public(w: &WorldState) -> f64 {
    let n = w.nation(HOME);
    n.treasury_bn.unwrap() - n.debt_bn.unwrap()
}

fn held(w: &WorldState, revision: &str) -> u32 {
    w.nation(HOME)
        .arsenal
        .held
        .iter()
        .filter(|row| row.design_id.as_deref() == Some(revision))
        .map(arsenal::available_design_units)
        .sum()
}

fn occupation(w: &mut WorldState, district: &str) {
    let mut conflict = Conflict {
        id: 999,
        theatre: spheres_sim::war::theatre_between(w, HOME, N::USSR),
        side_a: vec![HOME],
        side_b: vec![N::USSR],
        posture: vec![
            Belligerent::new(HOME, 8, Objective::Hold),
            Belligerent::new(N::USSR, 8, Objective::Seize),
        ],
        control: 0.0,
        months: 0,
        quiet_months: 0,
        frozen_since: None,
        start_year: w.year,
        start_month: w.month,
        origin_attacker: N::USSR,
        invasion_declared: true,
        front: Default::default(),
        pockets: vec![],
        aim: None,
    };
    conflict.front.insert(district.into(), -1.0);
    w.conflicts = vec![conflict];
    assert!(!spheres_sim::control::can_operate(w, HOME, district));
}

fn firm(w: &WorldState, company: u32) -> &companies::Company {
    companies::company(w, HOME, company).unwrap()
}

fn product(w: &WorldState, company: u32, product: u32) -> &companies::Product {
    firm(w, company)
        .products
        .iter()
        .find(|row| row.id == product)
        .unwrap()
}

fn establish(w: &mut WorldState, district: &str, capital: f64) -> u32 {
    let quote =
        companies::establishment_quote(w, HOME, "Synthetic Defense Works", district, capital);
    assert!(quote.valid, "{:?}", quote.reason);
    apply(
        w,
        companies::CompanyOrder::Establish {
            name: "Synthetic Defense Works".into(),
            district: district.into(),
            capitalization_bn: capital,
            quote: quote.token,
        },
    );
    w.companies.firms.last().unwrap().id
}

fn develop(w: &mut WorldState, company: u32, budget: f64, target: u32) -> u32 {
    let spec = equipment::default_spec("tank_standard");
    let quote = companies::development_quote(
        w,
        HOME,
        company,
        "Synthetic Atlas MBT",
        &spec,
        budget,
        target,
    );
    assert!(quote.valid, "{:?}", quote.reason);
    apply(
        w,
        companies::CompanyOrder::Develop {
            company,
            name: "Synthetic Atlas MBT".into(),
            spec,
            daily_budget_bn: budget,
            stock_target: target,
            quote: quote.token,
        },
    );
    firm(w, company).products.last().unwrap().id
}

fn purchase(w: &mut WorldState, company: u32, product: u32, quantity: u32) -> companies::Quote {
    let quote = companies::purchase_quote(w, HOME, company, product, quantity);
    assert!(quote.valid, "{:?}", quote.reason);
    apply(
        w,
        companies::CompanyOrder::Purchase {
            company,
            product,
            quantity,
            quote: quote.token.clone(),
        },
    );
    quote
}

fn close_fiscal_day(w: &mut WorldState) -> f64 {
    let plan = w.nation_mut(HOME).program_budget.as_mut().unwrap();
    // Isolate actual company flows from taxes, interest and the rest of the
    // economy. Standing service costs still post and are measured explicitly.
    plan.revenue_today_bn = 0.0;
    plan.interest_today_bn = 0.0;
    plan.fiscal_staged = true;
    let fresh = plan.spent_today_bn.iter().flatten().sum::<f64>();
    programs::finish_day(w);
    companies::settle_receivables(w);
    fresh
}

fn renew_budget(w: &mut WorldState) {
    if w.nation(HOME).program_budget.as_ref().unwrap().fiscal_year != w.year {
        let departments = w.nation(HOME).program_budget.as_ref().unwrap().departments;
        let allocations = w.nation(HOME).budget_for(w.year).allocations;
        spheres_sim::apply_command(
            w,
            &Command::SetProgramBudget {
                nation: HOME,
                fiscal_year: w.year,
                allocations,
                departments,
            },
        )
        .unwrap();
    }
}

fn isolated_day(w: &mut WorldState) {
    renew_budget(w);
    programs::begin_day(w);
    equipment::tick_day(w);
    companies::tick_day(w);
    close_fiscal_day(w);
    clock::advance_date(w);
}

fn real_day(w: &mut WorldState) {
    renew_budget(w);
    let news = spheres_sim::tick_day(w, &[]);
    assert!(
        !news.iter().any(|line| line.starts_with("[rejected]")),
        "{news:?}"
    );
}

fn until_stock(w: &mut WorldState, company: u32, id: u32, wanted: u32) {
    for _ in 0..1000 {
        if product(w, company, id).stock >= wanted {
            return;
        }
        isolated_day(w);
    }
    panic!(
        "No finished stock after 1,000 funded days: {:?}",
        product(w, company, id)
    );
}

fn ready(target: u32) -> (WorldState, String, u32, u32) {
    let (mut w, district) = fixture();
    let company = establish(&mut w, &district, 1.0);
    isolated_day(&mut w);
    let id = develop(&mut w, company, 1.0, target);
    until_stock(&mut w, company, id, target);
    (w, district, company, id)
}

fn reconcile_company(w: &WorldState, id: u32) {
    let c = firm(w, id);
    near(
        c.cash_bn,
        c.capital_received_bn + c.development_revenue_bn + c.sales_revenue_bn
            - c.development_expense_bn
            - c.tooling_expense_bn
            - c.materials_expense_bn
            - c.fabrication_expense_bn,
    );
    for p in &c.products {
        assert_eq!(
            p.produced_units,
            p.stock + p.sold_units,
            "every completed unit has one owner"
        );
    }
}

#[test]
fn inactive_company_readers_and_ticks_preserve_legacy_world_bytes() {
    for daily in [false, true] {
        let mut w = spheres_sim::init::world_1990(GameRules {
            daily_simulation: daily,
            ..Default::default()
        });
        let before = spheres_sim::save(&w);
        assert!(!serde_json::to_value(&w)
            .unwrap()
            .as_object()
            .unwrap()
            .contains_key("companies"));
        companies::tick_day(&mut w);
        companies::settle_receivables(&mut w);
        let quote = companies::establishment_quote(&w, HOME, "No supplier", "missing", 1.0);
        assert!(!quote.valid);
        assert_eq!(
            spheres_sim::save(&w),
            before,
            "an unused company layer cannot change the old path"
        );
        let loaded = spheres_sim::load(&before).unwrap();
        assert_eq!(spheres_sim::save(&loaded), before);
    }
}

#[test]
fn capitalization_waits_for_one_fiscal_posting_and_prepaid_money_is_not_charged_twice() {
    for prepaid in [0.0, 0.4] {
        let (mut w, district) = fixture();
        w.nation_mut(HOME)
            .program_budget
            .as_mut()
            .unwrap()
            .prepaid_bn[D][3] = prepaid;
        let before = net_public(&w);
        let company = establish(&mut w, &district, 0.3);
        near(net_public(&w), before);
        near(firm(&w, company).cash_bn, 0.0);
        near(
            firm(&w, company)
                .receivables
                .iter()
                .map(|r| r.amount_bn)
                .sum(),
            0.3,
        );
        companies::settle_receivables(&mut w);
        near(firm(&w, company).cash_bn, 0.0);
        let p = w.nation(HOME).program_budget.as_ref().unwrap();
        near(p.prepaid_used_today_bn[D][3], prepaid.min(0.3));
        near(p.spent_today_bn[D][3], (0.3 - prepaid).max(0.0));
        let fresh = close_fiscal_day(&mut w);
        near(net_public(&w), before - fresh);
        near(firm(&w, company).cash_bn, 0.3);
        assert!(firm(&w, company).receivables.is_empty());
        reconcile_company(&w, company);
        let settled = spheres_sim::save(&w);
        programs::finish_day(&mut w);
        companies::settle_receivables(&mut w);
        assert_eq!(
            spheres_sim::save(&w),
            settled,
            "retrying settlement cannot credit capital twice"
        );
    }
}

#[test]
fn development_tooling_and_finite_stock_are_distinct_paid_stages_without_public_equipment() {
    let (mut w, district) = fixture();
    let company = establish(&mut w, &district, 1.0);
    isolated_day(&mut w);
    let id = develop(&mut w, company, 1.0, 3);
    let revision = product(&w, company, id).revision_id.clone();
    let profile = equipment::profile(w.nation(HOME), &revision)
        .unwrap()
        .clone();
    let national_capabilities =
        serde_json::to_value(operations::capabilities(w.nation(HOME))).unwrap();
    let opening_gdp = w.nation(HOME).gdp;
    assert_eq!(product(&w, company, id).stock, 0);
    assert!(!companies::purchase_quote(&w, HOME, company, id, 1).valid);
    for _ in 0..profile.development_days {
        assert_eq!(held(&w, &revision), 0);
        assert_eq!(product(&w, company, id).stock, 0);
        isolated_day(&mut w);
    }
    // Issuing a contract never performs that date's engineering retroactively.
    while product(&w, company, id).certified_day.is_none() {
        isolated_day(&mut w);
    }
    let certified = product(&w, company, id);
    near(certified.development_spent_bn, profile.development_cost_bn);
    assert_eq!(certified.stock, 0, "certification is not a saleable tank");
    assert_eq!(certified.tooling_work_days, 0.0);
    let resources_before: Vec<_> = resources::ALL
        .iter()
        .map(|&c| resources::stockpile(&w, HOME, c))
        .collect();
    let net_before = net_public(&w);
    let material_before = firm(&w, company).materials_expense_bn;
    let mut fresh_services = 0.0;
    for _ in 0..1000 {
        if product(&w, company, id).stock == 3 {
            break;
        }
        renew_budget(&mut w);
        programs::begin_day(&mut w);
        equipment::tick_day(&mut w);
        companies::tick_day(&mut w);
        near(
            w.nation(HOME)
                .program_budget
                .as_ref()
                .unwrap()
                .spent_today_bn[D][3],
            0.0,
        );
        fresh_services += close_fiscal_day(&mut w);
        clock::advance_date(&mut w);
    }
    let p = product(&w, company, id);
    assert_eq!((p.produced_units, p.stock, p.sold_units), (3, 3, 0));
    near(p.tooling_spent_bn, profile.tooling_cost_bn);
    let raw_spend = firm(&w, company).materials_expense_bn - material_before;
    near(net_public(&w), net_before - fresh_services + raw_spend);
    for (index, &commodity) in resources::ALL.iter().enumerate() {
        near(
            resources_before[index] - resources::stockpile(&w, HOME, commodity),
            profile.recipe[index] * 3.0,
        );
    }
    assert_eq!(held(&w, &revision), 0);
    assert!(w.nation(HOME).arsenal.orders.is_empty());
    assert_eq!(
        serde_json::to_value(operations::capabilities(w.nation(HOME))).unwrap(),
        national_capabilities
    );
    assert_eq!(
        w.nation(HOME).gdp,
        opening_gdp,
        "company sales and production add no duplicate macro bonus"
    );
    let cash = firm(&w, company).cash_bn;
    let inputs = w.resources.market.as_ref().unwrap().stocks.clone();
    for _ in 0..4 {
        isolated_day(&mut w);
    }
    assert_eq!(product(&w, company, id).stock, 3);
    near(firm(&w, company).cash_bn, cash);
    assert_eq!(
        w.resources.market.as_ref().unwrap().stocks,
        inputs,
        "a full shelf stops further input purchases"
    );
    reconcile_company(&w, company);
}

#[test]
fn finite_stock_sale_is_atomic_stale_safe_and_fields_only_after_settled_delivery() {
    let (mut w, _district, company, id) = ready(3);
    let revision = product(&w, company, id).revision_id.clone();
    let frozen = equipment::profile(w.nation(HOME), &revision)
        .unwrap()
        .clone();
    let old = companies::purchase_quote(&w, HOME, company, id, 2);
    let before = spheres_sim::save(&w);
    assert!(!companies::purchase_quote(&w, HOME, company, id, 4).valid);
    assert!(companies::apply(
        &mut w,
        HOME,
        &companies::CompanyOrder::Purchase {
            company,
            product: id,
            quantity: 4,
            quote: old.token.clone(),
        }
    )
    .is_err());
    assert_eq!(spheres_sim::save(&w), before, "overselling is write-free");
    let cash_before = firm(&w, company).cash_bn;
    let stock_before = product(&w, company, id).stock_cost_bn;
    let quote = purchase(&mut w, company, id, 2);
    assert_eq!(product(&w, company, id).stock, 1);
    assert_eq!(held(&w, &revision), 0);
    assert!(
        w.nation(HOME).arsenal.orders.is_empty(),
        "the company delivery is the sole inbound inventory owner"
    );
    assert_eq!(w.companies.deliveries.len(), 1);
    let delivery = &w.companies.deliveries[0];
    assert_eq!(
        (delivery.quantity, delivery.revision_id.as_str()),
        (2, revision.as_str())
    );
    near(delivery.total_price_bn, quote.cost_bn);
    near(delivery.cost_basis_bn, stock_before * 2.0 / 3.0);
    near(delivery.unit_price_bn, quote.unit_price_bn);
    near(firm(&w, company).cash_bn, cash_before);
    let sold = spheres_sim::save(&w);
    assert!(companies::apply(
        &mut w,
        HOME,
        &companies::CompanyOrder::Purchase {
            company,
            product: id,
            quantity: 2,
            quote: old.token,
        }
    )
    .is_err());
    assert_eq!(
        spheres_sim::save(&w),
        sold,
        "retrying an old order spends nothing"
    );
    close_fiscal_day(&mut w);
    near(firm(&w, company).cash_bn, cash_before + quote.cost_bn);
    let due = w.companies.deliveries[0].due_day.unwrap();
    assert_eq!(
        due - w.companies.deliveries[0].settled_day.unwrap(),
        companies::DELIVERY_DAYS as i32
    );
    apply(
        &mut w,
        companies::CompanyOrder::Inventory {
            company,
            product: id,
            stock_target: 0,
        },
    );
    while clock::absolute_day(&w) <= due {
        let date = clock::absolute_day(&w);
        if date < due {
            assert_eq!(held(&w, &revision), 0);
        }
        isolated_day(&mut w);
    }
    assert_eq!(held(&w, &revision), 2);
    assert_eq!(w.companies.deliveries[0].delivered_day, Some(due));
    assert_eq!(
        equipment::profile(w.nation(HOME), &revision).unwrap(),
        &frozen,
        "seller margin never changes combat specifications"
    );
    for _ in 0..3 {
        isolated_day(&mut w);
    }
    assert_eq!(
        held(&w, &revision),
        2,
        "completed deliveries cannot field a second copy"
    );
    reconcile_company(&w, company);
    spheres_sim::load(&spheres_sim::save(&w)).unwrap();
}

#[test]
fn development_pause_partial_funding_cancellation_and_reload_retain_paid_work_exactly() {
    let (mut w, district) = fixture();
    let company = establish(&mut w, &district, 1.0);
    isolated_day(&mut w);
    let id = develop(&mut w, company, 0.0, 2);
    let revision = product(&w, company, id).revision_id.clone();
    for _ in 0..3 {
        isolated_day(&mut w);
    }
    near(product(&w, company, id).development_work_days, 0.0);
    near(product(&w, company, id).development_spent_bn, 0.0);
    let profile = equipment::profile(w.nation(HOME), &revision).unwrap();
    let half_day = profile.development_cost_bn / profile.development_days as f64 / 2.0;
    apply(
        &mut w,
        companies::CompanyOrder::Funding {
            company,
            product: id,
            daily_budget_bn: half_day,
        },
    );
    for _ in 0..5 {
        isolated_day(&mut w);
    }
    near(product(&w, company, id).development_work_days, 2.5);
    near(
        product(&w, company, id).development_spent_bn,
        half_day * 5.0,
    );
    let raw = spheres_sim::save(&w);
    let mut loaded = spheres_sim::load(&raw).unwrap();
    assert_eq!(spheres_sim::save(&loaded), raw);
    for _ in 0..5 {
        isolated_day(&mut w);
        isolated_day(&mut loaded);
    }
    assert_eq!(
        spheres_sim::save(&w),
        spheres_sim::save(&loaded),
        "partial funded work resumes identically"
    );
    let paid = product(&w, company, id).development_spent_bn;
    apply(
        &mut w,
        companies::CompanyOrder::CancelDevelopment {
            company,
            product: id,
        },
    );
    for _ in 0..5 {
        isolated_day(&mut w);
    }
    near(product(&w, company, id).development_spent_bn, paid);
    assert_eq!(product(&w, company, id).stock, 0);
    assert!(product(&w, company, id).certified_day.is_none());
    assert_eq!(held(&w, &revision), 0);
    near(firm(&w, company).development_revenue_bn, paid);
    near(firm(&w, company).development_expense_bn, paid);
    reconcile_company(&w, company);
    spheres_sim::load(&spheres_sim::save(&w)).unwrap();
}

#[test]
fn company_rights_share_the_existing_factory_and_preserve_paid_public_work() {
    let (mut w, district) = fixture();
    let legacy = manufacturing::start_line(&mut w, HOME, &district, "arm_gen3").unwrap();
    let before = spheres_sim::save(&w);
    let refused = companies::establishment_quote(&w, HOME, "No second plant", &district, 1.0);
    assert!(!refused.valid);
    assert!(companies::apply(
        &mut w,
        HOME,
        &companies::CompanyOrder::Establish {
            name: "No second plant".into(),
            district: district.clone(),
            capitalization_bn: 1.0,
            quote: refused.token,
        }
    )
    .is_err());
    assert_eq!(spheres_sim::save(&w), before);
    manufacturing::stop_line(&mut w, HOME, legacy).unwrap();
    let company = establish(&mut w, &district, 1.0);
    assert_eq!(
        manufacturing::plant_slots(&w, &district),
        1,
        "establishment grants use, not a new factory"
    );
    assert_eq!(manufacturing::used_slots(&w, HOME, &district), 1);
    let claimed = spheres_sim::save(&w);
    assert!(manufacturing::start_line(&mut w, HOME, &district, "arm_gen3").is_err());
    assert_eq!(spheres_sim::save(&w), claimed);
    // Capacity loss respects a pre-existing public reservation. Introduce a
    // second physical plant, start its public line, then lose that extra plant.
    w.production
        .provinces
        .iter_mut()
        .find(|site| site.district == district)
        .unwrap()
        .arms_plants = 2;
    let old = manufacturing::start_line(&mut w, HOME, &district, "arm_gen3").unwrap();
    let record = manufacturing::lines_for(&w, HOME)
        .find(|line| line.id == old)
        .unwrap()
        .clone();
    w.production
        .provinces
        .iter_mut()
        .find(|site| site.district == district)
        .unwrap()
        .arms_plants = 1;
    assert!(companies::facility_blocker(&w, firm(&w, company)).is_some());
    assert!(
        manufacturing::line_blocker(&w, &record).is_none(),
        "the old public reservation retains its sole physical slot"
    );
    assert_eq!(
        manufacturing::lines_for(&w, HOME)
            .find(|line| line.id == old)
            .unwrap(),
        &record
    );
    spheres_sim::load(&spheres_sim::save(&w)).unwrap();
}

#[test]
fn occupation_stops_work_and_holds_paid_transit_without_transferring_cash_or_designs() {
    let (mut w, district, company, id) = ready(2);
    let revision = product(&w, company, id).revision_id.clone();
    purchase(&mut w, company, id, 1);
    close_fiscal_day(&mut w);
    let original_due = w.companies.deliveries[0].due_day.unwrap();
    let cash = firm(&w, company).cash_bn;
    let intellectual_property =
        serde_json::to_value(&w.nation(HOME).equipment.as_ref().unwrap().revisions).unwrap();
    occupation(&mut w, &district);
    for _ in 0..10 {
        isolated_day(&mut w);
    }
    assert_eq!(held(&w, &revision), 0);
    assert_eq!(w.companies.deliveries[0].quantity, 1);
    assert!(w.companies.deliveries[0].delivered_day.is_none());
    assert!(w.companies.deliveries[0].due_day.unwrap() > original_due);
    assert_eq!(w.companies.deliveries[0].status, "blocked");
    assert_eq!(product(&w, company, id).stock, 1);
    near(firm(&w, company).cash_bn, cash);
    assert_eq!(
        serde_json::to_value(&w.nation(HOME).equipment.as_ref().unwrap().revisions).unwrap(),
        intellectual_property
    );
    let save = spheres_sim::save(&w);
    let mut loaded = spheres_sim::load(&save).unwrap();
    assert_eq!(spheres_sim::save(&loaded), save);
    w.conflicts.clear();
    loaded.conflicts.clear();
    apply(
        &mut w,
        companies::CompanyOrder::Inventory {
            company,
            product: id,
            stock_target: 0,
        },
    );
    apply(
        &mut loaded,
        companies::CompanyOrder::Inventory {
            company,
            product: id,
            stock_target: 0,
        },
    );
    for _ in 0..10 {
        isolated_day(&mut w);
        isolated_day(&mut loaded);
    }
    assert_eq!(held(&w, &revision), 1);
    assert_eq!(spheres_sim::save(&w), spheres_sim::save(&loaded));
    reconcile_company(&w, company);
}

#[test]
fn company_work_cannot_create_missing_inputs_or_use_unsettled_sales_cash() {
    let (mut w, district) = fixture();
    let company = establish(&mut w, &district, 1.0);
    isolated_day(&mut w);
    let id = develop(&mut w, company, 1.0, 1);
    let revision = product(&w, company, id).revision_id.clone();
    let profile = equipment::profile(w.nation(HOME), &revision)
        .unwrap()
        .clone();
    let required = resources::ALL
        .into_iter()
        .find(|c| profile.recipe[c.idx()] > 0.0)
        .unwrap();
    w.resources
        .market
        .as_mut()
        .unwrap()
        .stocks
        .iter_mut()
        .find(|s| s.nation == HOME && s.commodity == required)
        .unwrap()
        .quantity = 0.0;
    for _ in 0..profile.development_days + profile.tooling_days + 5 {
        isolated_day(&mut w);
    }
    assert_eq!(product(&w, company, id).stock, 0);
    near(firm(&w, company).materials_expense_bn, 0.0);
    assert!(product(&w, company, id).reason.contains("warehouse"));
    let waiting = firm(&w, company).cash_bn;
    for _ in 0..3 {
        isolated_day(&mut w);
    }
    near(firm(&w, company).cash_bn, waiting);
    w.resources
        .market
        .as_mut()
        .unwrap()
        .stocks
        .iter_mut()
        .find(|s| s.nation == HOME && s.commodity == required)
        .unwrap()
        .quantity = profile.recipe[required.idx()];
    until_stock(&mut w, company, id, 1);
    assert_eq!(resources::stockpile(&w, HOME, required), 0.0);
    let cash = firm(&w, company).cash_bn;
    purchase(&mut w, company, id, 1);
    companies::settle_receivables(&mut w);
    near(firm(&w, company).cash_bn, cash);
    assert_eq!(product(&w, company, id).stock, 0);
    assert_eq!(held(&w, &revision), 0);
}

#[test]
fn cash_limited_company_waits_then_real_capitalization_funds_resumable_work_in_progress() {
    let (mut w, district) = fixture();
    let spec = equipment::default_spec("tank_standard");
    let profile = equipment::design_preview(&w, HOME, &spec).profile.unwrap();
    let company = establish(&mut w, &district, profile.tooling_cost_bn + 0.000001);
    isolated_day(&mut w);
    let id = develop(&mut w, company, 1.0, 1);
    for _ in 0..profile.development_days + profile.tooling_days + 5 {
        isolated_day(&mut w);
    }
    assert!(product(&w, company, id).certified_day.is_some());
    assert_eq!(product(&w, company, id).stock, 0);
    assert!(product(&w, company, id).reason.contains("cash"));
    assert!(product(&w, company, id)
        .unit_inputs
        .iter()
        .all(|amount| *amount == 0.0));
    let input_stock = w.resources.market.as_ref().unwrap().stocks.clone();
    let quote = companies::capitalization_quote(&w, HOME, company, 0.1);
    assert!(quote.valid, "{:?}", quote.reason);
    let cash = firm(&w, company).cash_bn;
    apply(
        &mut w,
        companies::CompanyOrder::Capitalize {
            company,
            amount_bn: 0.1,
            quote: quote.token,
        },
    );
    companies::tick_day(&mut w);
    near(firm(&w, company).cash_bn, cash);
    assert_eq!(
        w.resources.market.as_ref().unwrap().stocks,
        input_stock,
        "unsettled capital cannot buy inputs"
    );
    close_fiscal_day(&mut w);
    clock::advance_date(&mut w);
    isolated_day(&mut w);
    let p = product(&w, company, id);
    assert!(p.unit_work_days > 0.0 && p.unit_work_days < profile.production_days as f64);
    assert_eq!(p.unit_inputs, profile.recipe);
    let raw = spheres_sim::save(&w);
    let mut loaded = spheres_sim::load(&raw).unwrap();
    assert_eq!(spheres_sim::save(&loaded), raw);
    until_stock(&mut w, company, id, 1);
    until_stock(&mut loaded, company, id, 1);
    assert_eq!(spheres_sim::save(&loaded), spheres_sim::save(&w));
    reconcile_company(&w, company);
}

#[test]
fn malformed_save_cannot_mint_uncertified_stock_or_duplicate_a_paid_delivery() {
    let (mut w, district) = fixture();
    let company = establish(&mut w, &district, 1.0);
    isolated_day(&mut w);
    let id = develop(&mut w, company, 1.0, 2);
    let mut forged = w.clone();
    let p = &mut forged.companies.firms[0].products[0];
    p.stock = 1;
    p.produced_units = 1;
    p.stock_cost_bn = 0.01;
    assert!(
        spheres_sim::load(&spheres_sim::save(&forged)).is_err(),
        "an undeveloped prototype cannot acquire saleable stock through loading"
    );
    until_stock(&mut w, company, id, 2);
    purchase(&mut w, company, id, 1);
    close_fiscal_day(&mut w);
    spheres_sim::load(&spheres_sim::save(&w)).unwrap();
    let mut duplicate = w.companies.deliveries[0].clone();
    duplicate.id = w.companies.next_id;
    w.companies.next_id += 1;
    w.companies.deliveries.push(duplicate);
    assert!(
        spheres_sim::load(&spheres_sim::save(&w)).is_err(),
        "a second shipment needs a second real stock sale and payment"
    );
}

#[test]
fn company_save_envelope_cannot_be_downgraded_or_silently_drop_corporate_property() {
    let (mut w, district) = fixture();
    let legacy = spheres_sim::save(&w);
    assert_eq!(
        spheres_sim::save(&spheres_sim::load(&legacy).unwrap()),
        legacy
    );
    establish(&mut w, &district, 1.0);
    let valid = spheres_sim::save(&w);
    let mut envelope: serde_json::Value = serde_json::from_str(&valid).unwrap();
    assert_eq!(envelope["version"], 2);
    assert_eq!(
        spheres_sim::save(&spheres_sim::load(&valid).unwrap()),
        valid
    );
    envelope["version"] = 1.into();
    assert!(spheres_sim::load(&envelope.to_string()).is_err());
    assert!(spheres_sim::load(&serde_json::to_string(&w).unwrap()).is_err());
    envelope["version"] = 2.into();
    envelope["world"]
        .as_object_mut()
        .unwrap()
        .remove("companies");
    assert!(
        spheres_sim::load(&envelope.to_string()).is_err(),
        "version two cannot silently erase the supplier book"
    );
}

#[test]
fn pending_invoices_require_one_real_fiscal_payment_and_one_unsettled_purchase() {
    let (mut w, _, company, id) = ready(1);
    purchase(&mut w, company, id, 1);
    let sale = firm(&w, company)
        .receivables
        .iter()
        .find(|r| r.kind == "sale")
        .unwrap()
        .clone();
    spheres_sim::load(&spheres_sim::save(&w)).unwrap();
    let mut duplicate = w.clone();
    let mut forged = sale.clone();
    forged.id = duplicate.companies.next_id;
    duplicate.companies.next_id += 1;
    duplicate.companies.firms[0].receivables.push(forged);
    assert!(
        spheres_sim::load(&spheres_sim::save(&duplicate)).is_err(),
        "duplicated sale invoices must not credit one stock sale twice"
    );

    let mut unpaid_capital = w.clone();
    let mut forged = sale.clone();
    forged.id = unpaid_capital.companies.next_id;
    unpaid_capital.companies.next_id += 1;
    forged.kind = "capitalization".into();
    forged.product = None;
    forged.delivery = None;
    forged.amount_bn = 1000.0;
    unpaid_capital.companies.firms[0].receivables.push(forged);
    assert!(
        spheres_sim::load(&spheres_sim::save(&unpaid_capital)).is_err(),
        "a receivable needs matching posted department funding"
    );

    close_fiscal_day(&mut w);
    let mut already_paid = sale;
    already_paid.id = w.companies.next_id;
    w.companies.next_id += 1;
    w.companies.firms[0].receivables.push(already_paid);
    assert!(
        spheres_sim::load(&spheres_sim::save(&w)).is_err(),
        "settled purchases cannot acquire a fresh payable invoice"
    );
}

#[test]
fn saved_purchase_dates_cannot_skip_transit_or_claim_future_payment() {
    let (mut w, _, company, id) = ready(1);
    purchase(&mut w, company, id, 1);
    close_fiscal_day(&mut w);
    let today = clock::absolute_day(&w);
    spheres_sim::load(&spheres_sim::save(&w)).unwrap();
    for corruption in ["early_due", "future_settlement", "early_arrival"] {
        let mut invalid = w.clone();
        let delivery = &mut invalid.companies.deliveries[0];
        match corruption {
            "early_due" => delivery.due_day = Some(delivery.purchased_day),
            "future_settlement" => {
                delivery.settled_day = Some(today + 1);
                delivery.due_day = Some(today + 1 + companies::DELIVERY_DAYS as i32);
            }
            "early_arrival" => delivery.delivered_day = Some(today),
            _ => unreachable!(),
        }
        assert!(
            spheres_sim::load(&spheres_sim::save(&invalid)).is_err(),
            "{corruption} must not bypass dated payment and transport"
        );
    }
}

#[test]
fn changing_market_prices_after_input_purchase_cannot_reprice_work_or_vehicle_performance() {
    let (mut w, district) = fixture();
    let company = establish(&mut w, &district, 1.0);
    isolated_day(&mut w);
    let id = develop(&mut w, company, 1.0, 1);
    let revision = product(&w, company, id).revision_id.clone();
    let profile = equipment::profile(w.nation(HOME), &revision)
        .unwrap()
        .clone();
    for _ in 0..1000 {
        if product(&w, company, id).unit_work_days > 0.0 {
            break;
        }
        isolated_day(&mut w);
    }
    assert!(product(&w, company, id).unit_work_days > 0.0);
    let paid_materials = product(&w, company, id).unit_material_cost_bn;
    assert!(paid_materials > 0.0);
    let stocks = w.resources.market.as_ref().unwrap().stocks.clone();
    for price in &mut w.resources.market.as_mut().unwrap().prices {
        *price *= 100.0;
    }
    w.oil_price *= 100.0;
    let raw = spheres_sim::save(&w);
    let mut restored = spheres_sim::load(&raw).unwrap();
    until_stock(&mut w, company, id, 1);
    until_stock(&mut restored, company, id, 1);
    assert_eq!(spheres_sim::save(&w), spheres_sim::save(&restored));
    near(firm(&w, company).materials_expense_bn, paid_materials);
    near(
        firm(&w, company).fabrication_expense_bn,
        profile.fabrication_cost_bn,
    );
    near(
        product(&w, company, id).stock_cost_bn,
        paid_materials + profile.fabrication_cost_bn,
    );
    let quote = companies::purchase_quote(&w, HOME, company, id, 1);
    assert!(quote.valid, "{:?}", quote.reason);
    near(
        quote.unit_price_bn,
        (paid_materials + profile.fabrication_cost_bn) * (1.0 + companies::MARGIN),
    );
    assert_eq!(
        w.resources.market.as_ref().unwrap().stocks,
        stocks,
        "already owned inputs are consumed only once"
    );
    assert_eq!(
        equipment::profile(w.nation(HOME), &revision).unwrap(),
        &profile
    );
    reconcile_company(&w, company);
}

#[test]
fn insufficient_purchase_authority_refuses_without_mutating_stock_receipts_or_the_funding_day() {
    let (mut w, _district, company, id) = ready(1);
    programs::begin_day(&mut w);
    let plan = w.nation_mut(HOME).program_budget.as_mut().unwrap();
    plan.available_bn[D][3] = 0.0;
    plan.prepaid_bn[D][3] = 0.0;
    let quote = companies::purchase_quote(&w, HOME, company, id, 1);
    assert!(!quote.valid);
    let before = spheres_sim::save(&w);
    assert!(companies::apply(
        &mut w,
        HOME,
        &companies::CompanyOrder::Purchase {
            company,
            product: id,
            quantity: 1,
            quote: quote.token,
        }
    )
    .is_err());
    assert_eq!(spheres_sim::save(&w), before);
}

#[test]
fn company_enrollment_preserves_opening_raw_stock_and_accrues_procurement_for_reviewed_purchases() {
    let (mut w, district) = fixture();
    // Exercise the actual old-save boundary: the physical warehouse has not
    // been materialized yet and still exists as nation-owned opening cover.
    w.resources.market = None;
    let before: Vec<_> = w
        .nations
        .iter()
        .filter(|n| n.alive)
        .map(|n| {
            (
                n.id,
                resources::ALL.map(|c| resources::stockpile(&w, n.id, c)),
            )
        })
        .collect();
    establish(&mut w, &district, 1.0);
    for (nation, amounts) in before {
        for c in resources::ALL {
            near(resources::stockpile(&w, nation, c), amounts[c.idx()]);
        }
    }
    assert!(companies::procurement_active(&w, HOME));
    real_day(&mut w);
    assert!(w.nation(HOME).arsenal.orders.is_empty());
    let mut available = programs::available_bn(&w, HOME, D, 3);
    for _ in 0..5 {
        real_day(&mut w);
        near(
            w.nation(HOME)
                .program_budget
                .as_ref()
                .unwrap()
                .spent_today_bn[D][3],
            0.0,
        );
        let after = programs::available_bn(&w, HOME, D, 3);
        assert!(
            after > available,
            "unassigned procurement funding must accrue for reviewed purchases"
        );
        available = after;
        assert!(
            w.nation(HOME).arsenal.orders.is_empty(),
            "the company path cannot silently issue old catalogue orders"
        );
    }
}

#[test]
fn a_saved_fiscal_close_settles_its_company_receipt_before_new_year_authority_rolls_over() {
    let (mut w, district) = fixture();
    w.month = 12;
    w.day = 31;
    let company = establish(&mut w, &district, 1.0);
    let plan = w.nation_mut(HOME).program_budget.as_mut().unwrap();
    plan.revenue_today_bn = 0.0;
    plan.interest_today_bn = 0.0;
    plan.fiscal_staged = true;
    programs::finish_day(&mut w);
    assert_eq!(firm(&w, company).cash_bn, 0.0);
    assert_eq!(firm(&w, company).receivables.len(), 1);
    clock::advance_date(&mut w);
    assert_eq!((w.year, w.month, w.day), (1991, 1, 1));
    let mut restored = spheres_sim::load(&spheres_sim::save(&w)).unwrap();
    real_day(&mut w);
    real_day(&mut restored);
    near(firm(&w, company).cash_bn, 1.0);
    near(firm(&w, company).capital_received_bn, 1.0);
    assert!(firm(&w, company).receivables.is_empty());
    assert_eq!(
        w.nation(HOME)
            .program_budget
            .as_ref()
            .unwrap()
            .authority_year,
        1991
    );
    assert_eq!(spheres_sim::save(&w), spheres_sim::save(&restored));
    let once = firm(&w, company).clone();
    companies::settle_receivables(&mut w);
    assert_eq!(firm(&w, company), &once);
}

#[test]
fn establishment_preserves_existing_owned_vehicles_paid_deliveries_and_public_contracts() {
    let (mut w, district) = fixture();
    let spec = equipment::default_spec("tank_light");
    let profile = equipment::design_preview(&w, HOME, &spec).profile.unwrap();
    let day = clock::absolute_day(&w);
    w.nation_mut(HOME)
        .equipment
        .get_or_insert_with(Default::default)
        .revisions
        .insert(
            "paid-legacy".into(),
            equipment::DesignRevision {
                id: "paid-legacy".into(),
                name: "Earlier government model".into(),
                specification_key: equipment::specification_key(&spec),
                spec,
                profile,
                created_day: day,
                certified_day: Some(day),
            },
        );
    arsenal::deliver_design(w.nation_mut(HOME), "paid-legacy", 2, 18.0).unwrap();
    arsenal::queue_design_order(w.nation_mut(HOME), "paid-legacy", 3, 5, 0.0).unwrap();
    w.production
        .provinces
        .iter_mut()
        .find(|p| p.district == district)
        .unwrap()
        .arms_plants = 2;
    equipment::start_production(&mut w, HOME, "paid-legacy", &district, 1, 0.01).unwrap();
    let arsenal = serde_json::to_value(&w.nation(HOME).arsenal).unwrap();
    let contracts = w.nation(HOME).equipment.as_ref().unwrap().projects.clone();
    let company = establish(&mut w, &district, 1.0);
    assert_eq!(
        serde_json::to_value(&w.nation(HOME).arsenal).unwrap(),
        arsenal
    );
    assert_eq!(
        w.nation(HOME).equipment.as_ref().unwrap().projects,
        contracts
    );
    assert!(firm(&w, company).products.is_empty());
    for _ in 0..6 {
        real_day(&mut w);
    }
    assert!(
        held(&w, "paid-legacy") >= 5,
        "earlier paid cargo still arrives without being bought back from the company"
    );
    assert!(firm(&w, company).products.is_empty());
    assert!(
        w.nation(HOME).equipment.as_ref().unwrap().projects[0].work_days > 0.0,
        "existing public manufacture continues its paid contract"
    );
    spheres_sim::load(&spheres_sim::save(&w)).unwrap();
}

#[test]
#[ignore = "Writes synthetic browser QA stage saves only when SPHERES_COMPANY_QA_DIR is explicitly set"]
fn export_company_qa_stages() {
    let Some(folder) = std::env::var_os("SPHERES_COMPANY_QA_DIR") else {
        return;
    };
    let path = std::path::PathBuf::from(folder);
    assert!(
        path.is_absolute(),
        "Use an explicit absolute directory for synthetic QA saves"
    );
    std::fs::create_dir_all(&path).unwrap();
    let write = |name: &str, w: &WorldState| {
        let save = spheres_sim::save(w);
        let restored = spheres_sim::load(&save).unwrap();
        assert_eq!(spheres_sim::save(&restored), save);
        std::fs::write(path.join(name), save).unwrap();
    };
    let (mut w, district) = fixture();
    write("01-eligible.json", &w);
    let company = establish(&mut w, &district, 1.0);
    real_day(&mut w);
    let id = develop(&mut w, company, 1.0, 2);
    let revision = product(&w, company, id).revision_id.clone();
    for _ in 0..10 {
        real_day(&mut w);
    }
    assert!(product(&w, company, id).development_work_days > 0.0);
    assert!(product(&w, company, id).certified_day.is_none());
    write("02-development.json", &w);
    for _ in 0..700 {
        if product(&w, company, id).stock >= 2 {
            break;
        }
        real_day(&mut w);
    }
    assert_eq!(
        product(&w, company, id).stock,
        2,
        "{:?}",
        product(&w, company, id)
    );
    assert_eq!(held(&w, &revision), 0);
    write("03-stock.json", &w);
    purchase(&mut w, company, id, 1);
    real_day(&mut w);
    assert_eq!(held(&w, &revision), 0);
    assert!(w.companies.deliveries[0].settled_day.is_some());
    write("04-transit.json", &w);
    for _ in 0..20 {
        if held(&w, &revision) > 0 {
            break;
        }
        real_day(&mut w);
    }
    assert_eq!(held(&w, &revision), 1);
    write("05-arrived.json", &w);
    std::fs::write(path.join("README.txt"), format!(
        "Synthetic company lifecycle browser QA. France starts with $100bn treasury, explicit departmental authority, test raw-material stocks and one test Arms Plant in {district}. No historical company balance or inventory is asserted. After those opening endowments all development, tooling, manufacturing, purchases and arrivals use real commands and full daily system ticks. Company {company}; product {id}; revision {revision}.\n"
    )).unwrap();
    println!("Synthetic company QA saves: {}", path.display());
}
