//! S03 physical supplier operating receipts. Opening facilities/resources/cash
//! are synthetic fixture endowments. Development, paid inventory, settlement
//! and ownership thereafter use the same reviewed company commands as gameplay.
use spheres_sim::world::{GameRules, NationId as N, WorldState, BUDGET_DEFENSE as D};
use spheres_sim::{
    clock, companies, company_network, connected_economy, equipment, industry_operations,
    manufacturing, production, programs, resources, supplier_operations as ops, Command,
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
    develop_model(
        w,
        company,
        "tank_standard",
        "Synthetic Atlas MBT",
        budget,
        target,
    )
}

fn develop_model(
    w: &mut WorldState,
    company: u32,
    platform: &str,
    name: &str,
    budget: f64,
    target: u32,
) -> u32 {
    let spec = equipment::default_spec(platform);
    let quote = companies::development_quote(w, HOME, company, name, &spec, budget, target);
    assert!(quote.valid, "{:?}", quote.reason);
    apply(
        w,
        companies::CompanyOrder::Develop {
            company,
            name: name.into(),
            spec,
            daily_budget_bn: budget,
            stock_target: target,
            quote: quote.token,
        },
    );
    firm(w, company).products.last().unwrap().id
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

fn adopt(w: &mut WorldState, district: &str) {
    // Explicit synthetic utility endowment, present before workforce enrollment.
    w.production
        .provinces
        .iter_mut()
        .find(|s| s.district == district)
        .unwrap()
        .power_grid = 10;
    w.production
        .industry
        .sites
        .insert(district.into(), [0, 1, 0, 0, 0, 0, 0]);
    connected_economy::enable(w).unwrap();
    company_network::enable(w).unwrap();
    assert!(ops::enabled(w));
}
fn raise_target(w: &mut WorldState, c: u32, p: u32, target: u32) {
    apply(
        w,
        companies::CompanyOrder::Inventory {
            company: c,
            product: p,
            stock_target: target,
        },
    );
}
fn raw(w: &WorldState) -> [f64; 12] {
    resources::ALL.map(|r| resources::stockpile(w, HOME, r))
}
fn set_raw(w: &mut WorldState, r: resources::Commodity, amount: f64) {
    w.resources
        .market
        .as_mut()
        .unwrap()
        .stocks
        .iter_mut()
        .find(|s| s.nation == HOME && s.commodity == r)
        .unwrap()
        .quantity = amount;
}
fn endowment_components(w: &mut WorldState, amount: f64) {
    w.production
        .operations
        .advanced_components
        .insert(HOME, amount);
}
fn physical(w: &WorldState, c: u32, p: u32) -> serde_json::Value {
    let company = firm(w, c);
    let product = product(w, c, p);
    serde_json::json!({"raw":raw(w),"components":industry_operations::advanced_component_stock(w,HOME),
        "cash":company.cash_bn,"materials":company.materials_expense_bn,"fabrication":company.fabrication_expense_bn,
        "public_cash":w.nation(HOME).treasury_bn,"public_debt":w.nation(HOME).debt_bn,
        "work":product.unit_work_days,"inputs":product.unit_inputs,"stock":product.stock,
        "basis":product.unit_spent_bn,"ledger":w.supplier_operations})
}
fn staffed(w: &mut WorldState, d: &str, fraction: f64) {
    // This isolated fixture changes only derived job assignments. It deliberately
    // does not pretend this is a complete population save; separate continuation
    // tests retain the real coherent enrollment and workforce accounts.
    let p = w.population_system.provinces.get_mut(d).unwrap();
    for grade in 0..3 {
        p.filled[2][grade] = (p.jobs[2][grade] + p.project_jobs[2][grade]) * fraction;
    }
}

#[test]
fn absent_operations_are_byte_inert_and_adoption_preserves_existing_property() {
    let (mut w, d, c, p) = ready(1);
    let before = spheres_sim::save(&w);
    let _ = ops::snapshot(&w, HOME);
    ops::validate(&w).unwrap();
    assert_eq!(before, spheres_sim::save(&w));
    assert!(!before.contains("supplier_operations"));
    let company = serde_json::to_value(&w.companies).unwrap();
    let nation = w.nation(HOME);
    let property = (
        nation.treasury_bn,
        nation.debt_bn,
        nation.gdp,
        product(&w, c, p).stock,
    );
    adopt(&mut w, &d);
    assert_eq!(company, serde_json::to_value(&w.companies).unwrap());
    assert_eq!(
        property,
        (
            w.nation(HOME).treasury_bn,
            w.nation(HOME).debt_bn,
            w.nation(HOME).gdp,
            product(&w, c, p).stock
        )
    );
    assert!(w.supplier_operations.contracts.is_empty());
    let before = spheres_sim::save(&w);
    company_network::enable(&mut w).unwrap();
    assert_eq!(before, spheres_sim::save(&w));
    let loaded = spheres_sim::load(&before).unwrap();
    assert_eq!(before, spheres_sim::save(&loaded));
}

#[test]
fn component_coal_staff_and_company_cash_shortages_cannot_partially_buy_a_unit() {
    let (mut original, d, c, p) = ready(1);
    adopt(&mut original, &d);
    raise_target(&mut original, c, p, 2);
    for shortage in ["components", "coal", "staff", "cash"] {
        let mut w = original.clone();
        endowment_components(&mut w, 100.0);
        match shortage {
            "components" => endowment_components(&mut w, 0.0),
            "coal" => set_raw(&mut w, resources::Commodity::Coal, 0.0),
            "staff" => staffed(&mut w, &d, 0.0),
            "cash" => {
                w.companies
                    .firms
                    .iter_mut()
                    .find(|x| x.id == c)
                    .unwrap()
                    .cash_bn = 0.0
            }
            _ => unreachable!(),
        }
        let before = physical(&w, c, p);
        let people = w.population_system.clone();
        companies::tick_day(&mut w);
        assert_eq!(before, physical(&w, c, p), "partial posting for {shortage}");
        assert_eq!(
            people, w.population_system,
            "supplier work cannot hire a second workforce"
        );
        assert_eq!(product(&w, c, p).status, "blocked");
        assert!(!product(&w, c, p).reason.is_empty());
        assert!(w.supplier_operations.receipts.is_empty());
    }
}

#[test]
fn new_unit_purchases_components_once_and_shared_inputs_have_one_cash_owner() {
    let (mut w, d, c, p) = ready(1);
    adopt(&mut w, &d);
    raise_target(&mut w, c, p, 2);
    endowment_components(&mut w, 100.0);
    let profile = equipment::profile(w.nation(HOME), &product(&w, c, p).revision_id)
        .unwrap()
        .clone();
    let fraction = ops::work_fraction(&w, firm(&w, c));
    assert!(fraction > 0.0);
    let before_raw = raw(&w);
    let before_cash = firm(&w, c).cash_bn;
    let treasury = w.nation(HOME).treasury_bn.unwrap();
    let debt = w.nation(HOME).debt_bn.unwrap();
    let gdp = w.nation(HOME).gdp;
    let spending = w
        .nation(HOME)
        .program_budget
        .as_ref()
        .unwrap()
        .spent_today_bn;
    let people = w.population_system.clone();
    companies::tick_day(&mut w);
    let receipt = w.supplier_operations.receipts[0].clone();
    near(receipt.work_days, fraction);
    near(receipt.power_used, fraction * ops::POWER_PER_WORK_DAY);
    near(
        industry_operations::advanced_component_stock(&w, HOME),
        100.0 - ops::components_for(profile.fabrication_cost_bn),
    );
    for r in resources::ALL {
        near(
            before_raw[r.idx()] - raw(&w)[r.idx()],
            receipt.raw_used[r.idx()],
        );
    }
    near(
        receipt.raw_used[resources::Commodity::Coal.idx()],
        profile.recipe[resources::Commodity::Coal.idx()] + receipt.power_used * 0.02,
    );
    near(before_cash - firm(&w, c).cash_bn, receipt.cash_paid_bn);
    let inputs = w.supplier_operations.contracts[&p].input_cost_bn;
    near(
        w.nation(HOME).treasury_bn.unwrap() - w.nation(HOME).debt_bn.unwrap() - (treasury - debt),
        inputs,
    );
    assert_eq!(
        w.nation(HOME).gdp,
        gdp,
        "supplier purchases do not pay GDP again"
    );
    assert_eq!(
        w.nation(HOME)
            .program_budget
            .as_ref()
            .unwrap()
            .spent_today_bn,
        spending,
        "no extra public appropriation"
    );
    assert_eq!(w.population_system, people);
    companies::validate_state(&w).unwrap();
    ops::validate(&w).unwrap();
    let mut corrupt = w.clone();
    corrupt
        .supplier_operations
        .contracts
        .get_mut(&p)
        .unwrap()
        .raw_used[0] += 1.0;
    assert!(ops::validate(&corrupt).is_err());
    let mut corrupt = w.clone();
    let t = corrupt.supplier_operations.contracts.get_mut(&p).unwrap();
    t.work_days += 0.25;
    t.power_used += 0.25 * ops::POWER_PER_WORK_DAY;
    assert!(
        ops::validate(&corrupt).is_err(),
        "raw and power receipts cannot manufacture extra work"
    );
    let saved = spheres_sim::save(&w);
    companies::tick_day(&mut w);
    assert_eq!(saved, spheres_sim::save(&w));
    let mut resumed = spheres_sim::load(&saved).unwrap();
    let components = industry_operations::advanced_component_stock(&w, HOME);
    clock::advance_date(&mut w);
    clock::advance_date(&mut resumed);
    companies::tick_day(&mut w);
    companies::tick_day(&mut resumed);
    near(
        industry_operations::advanced_component_stock(&w, HOME),
        components,
    );
    assert_eq!(spheres_sim::save(&w), spheres_sim::save(&resumed));
    companies::validate_state(&w).unwrap();
    ops::validate(&w).unwrap();
}

#[test]
fn quarter_staffing_changes_real_work_and_cash_without_adding_workers() {
    let (mut w, d, c, p) = ready(1);
    adopt(&mut w, &d);
    raise_target(&mut w, c, p, 2);
    endowment_components(&mut w, 100.0);
    staffed(&mut w, &d, 0.25);
    let people = w.population_system.clone();
    let profile = equipment::profile(w.nation(HOME), &product(&w, c, p).revision_id)
        .unwrap()
        .clone();
    let fab = firm(&w, c).fabrication_expense_bn;
    companies::tick_day(&mut w);
    near(product(&w, c, p).unit_work_days, 0.25);
    near(
        firm(&w, c).fabrication_expense_bn - fab,
        profile.fabrication_cost_bn / profile.production_days as f64 * 0.25,
    );
    assert_eq!(people, w.population_system);
    near(
        ops::power_used(&w, HOME, Some(&d)),
        0.25 * ops::POWER_PER_WORK_DAY,
    );
    companies::validate_state(&w).unwrap();
    ops::validate(&w).unwrap();
}

#[test]
fn existing_wip_finishes_under_original_terms_then_next_unit_requires_components() {
    let (mut w, d, c, p) = ready(1);
    raise_target(&mut w, c, p, 2);
    isolated_day(&mut w);
    assert!(product(&w, c, p).unit_work_days > 0.0);
    let before_inputs = product(&w, c, p).unit_inputs;
    let before_basis = product(&w, c, p).unit_material_cost_bn;
    adopt(&mut w, &d);
    endowment_components(&mut w, 0.0);
    assert!(w.supplier_operations.grandfathered_units.contains(&p));
    assert_eq!(product(&w, c, p).unit_inputs, before_inputs);
    near(product(&w, c, p).unit_material_cost_bn, before_basis);
    until_stock(&mut w, c, p, 2);
    assert!(!w.supplier_operations.grandfathered_units.contains(&p));
    assert!(!w.supplier_operations.contracts.contains_key(&p));
    raise_target(&mut w, c, p, 3);
    let before = physical(&w, c, p);
    companies::tick_day(&mut w);
    assert_eq!(before, physical(&w, c, p));
    assert!(product(&w, c, p).reason.contains("components"));
    companies::validate_state(&w).unwrap();
    ops::validate(&w).unwrap();
}

#[test]
fn same_day_new_development_is_not_mistaken_for_a_grandfathered_program() {
    let (mut w, d, c, _) = ready(1);
    adopt(&mut w, &d);
    let p = develop_model(&mut w, c, "tank_light", "S03 new light tank", 1.0, 1);
    assert!(!w.supplier_operations.grandfathered_programs.contains(&p));
    clock::advance_date(&mut w);
    programs::begin_day(&mut w);
    w.nation_mut(HOME)
        .program_budget
        .as_mut()
        .unwrap()
        .available_bn[D][4] = 10.0;
    set_raw(&mut w, resources::Commodity::Coal, 0.0);
    let public = w
        .nation(HOME)
        .program_budget
        .as_ref()
        .unwrap()
        .spent_today_bn;
    companies::tick_day(&mut w);
    assert_eq!(product(&w, c, p).development_work_days, 0.0);
    assert_eq!(
        w.nation(HOME)
            .program_budget
            .as_ref()
            .unwrap()
            .spent_today_bn,
        public
    );
    assert!(w.supplier_operations.receipts.is_empty());
}

#[test]
fn separate_shipyard_lease_cannot_steal_an_existing_arms_plant_slot() {
    let (mut w, d) = fixture();
    w.production
        .provinces
        .iter_mut()
        .find(|s| s.district == d)
        .unwrap()
        .arms_plants = 2;
    let legacy = manufacturing::start_line(&mut w, HOME, &d, "nav_patrol").unwrap();
    let c = establish(&mut w, &d, 1.0);
    adopt(&mut w, &d);
    w.production.rebuild_sites.insert(d.clone(), [0, 1, 0]);
    let dock = manufacturing::start_line(&mut w, HOME, &d, "nav_escort").unwrap();
    assert!(w.manufacturing.shipyard_lines.contains(&dock));
    assert!(!w.manufacturing.shipyard_lines.contains(&legacy));
    assert_eq!(manufacturing::used_slots(&w, HOME, &d), 2);
    assert_eq!(manufacturing::used_naval_slots(&w, HOME, &d), 1);
    assert!(companies::facility_blocker(&w, firm(&w, c)).is_none());
}

#[test]
fn supplier_and_prior_industry_consumer_share_remaining_power_once() {
    let (mut w, d, c, p) = ready(1);
    adopt(&mut w, &d);
    raise_target(&mut w, c, p, 2);
    endowment_components(&mut w, 100.0);
    staffed(&mut w, &d, 1.0);
    let available = spheres_sim::industry::power_capacity(&w, HOME)
        .min(industry_operations::grid_capacity(&w, &d));
    assert!(available > ops::POWER_PER_WORK_DAY);
    // A prior operating consumer owns this dated power receipt. The supplier
    // cannot use a new copy of that same national and local allocation.
    let mut prior = industry_operations::site(&w, &d, production::ProjectKind::ArmsPlant);
    prior.recorded_day = Some(clock::absolute_day(&w));
    prior.power_used_daily = available - ops::POWER_PER_WORK_DAY * 0.75;
    w.production.operations.support_day = Some(clock::absolute_day(&w));
    w.production.operations.receipts.push(prior);
    let people = w.population_system.clone();
    companies::tick_day(&mut w);
    near(product(&w, c, p).unit_work_days, 0.75);
    assert_eq!(w.supplier_operations.receipts.len(), 1);
    near(
        ops::power_used(&w, HOME, None),
        ops::POWER_PER_WORK_DAY * 0.75,
    );
    near(ops::work_fraction(&w, firm(&w, c)), 0.0);
    let saved = spheres_sim::save(&w);
    companies::tick_day(&mut w);
    assert_eq!(saved, spheres_sim::save(&w));
    assert_eq!(people, w.population_system);
    assert_eq!(companies::reserved_slots(&w, HOME, &d), 1);
    ops::validate(&w).unwrap();
    companies::validate_state(&w).unwrap();
}

#[test]
fn basic_ammunition_uses_only_whole_staffed_paid_packets_and_no_component_recipe() {
    let (mut w, d, c, p) = ready(1);
    adopt(&mut w, &d);
    endowment_components(&mut w, 0.0);
    let revision = product(&w, c, p).revision_id.clone();
    let family = equipment::ammunition_family(
        &w.nation(HOME).equipment.as_ref().unwrap().revisions[&revision].spec,
    )
    .unwrap()
    .to_string();
    spheres_sim::apply_command(
        &mut w,
        &Command::Equipment {
            nation: HOME,
            order: spheres_sim::EquipmentOrder::Maintenance {
                daily_budget_bn: 0.001,
            },
        },
    )
    .unwrap();
    let q = companies::ammo_supply_quote(&w, HOME, c, &family, 1000);
    assert!(q.valid, "{:?}", q.reason);
    apply(
        &mut w,
        companies::CompanyOrder::AmmoSupply {
            company: c,
            family: family.clone(),
            stock_target: 1000,
            quote: q.token,
        },
    );
    clock::advance_date(&mut w);
    staffed(&mut w, &d, 0.25);
    let before = raw(&w);
    companies::tick_day(&mut w);
    let a = &firm(&w, c).ammunition_products[0];
    let def = equipment::ammo_def(&family).unwrap();
    let expected = (def.rounds_per_day * 0.25).floor() as u32;
    assert_eq!(a.stock, expected);
    assert_eq!(a.produced_units, expected);
    assert!(w
        .nation(HOME)
        .equipment
        .as_ref()
        .unwrap()
        .ammunition
        .as_ref()
        .is_none_or(|a| a.supplier_receipts.is_empty()));
    let r = &w.supplier_operations.receipts[0];
    assert_eq!(r.units, expected);
    near(r.components_used, 0.0);
    near(r.work_days, expected as f64 / def.rounds_per_day);
    for k in 0..12 {
        near(a.resources_used[k], def.recipe[k] * expected as f64);
        near(before[k] - raw(&w)[k], r.raw_used[k]);
    }
    ops::validate(&w).unwrap();
    companies::validate_state(&w).unwrap();
}

#[test]
fn new_refit_freezes_fee_and_component_recipe_preserving_escrow_and_source_ownership() {
    let (mut w, d, c, source_product) = ready(1);
    let source = product(&w, c, source_product).revision_id.clone();
    let q = companies::purchase_quote(&w, HOME, c, source_product, 1);
    assert!(q.valid, "{:?}", q.reason);
    apply(
        &mut w,
        companies::CompanyOrder::Purchase {
            company: c,
            product: source_product,
            quantity: 1,
            quote: q.token,
        },
    );
    for _ in 0..20 {
        isolated_day(&mut w);
    }
    assert_eq!(
        w.nation(HOME)
            .arsenal
            .held
            .iter()
            .find(|h| h.design_id.as_deref() == Some(&source))
            .unwrap()
            .units,
        1.0
    );
    // Explicit research endowment allows an authored compatible upgrade; the
    // company still pays all actual development/tooling/stock work below.
    w.nation_mut(HOME).equipment.as_mut().unwrap().learned =
        equipment::RESEARCH.iter().map(|r| r.id.into()).collect();
    let mut spec = w.nation(HOME).equipment.as_ref().unwrap().revisions[&source]
        .spec
        .clone();
    spec.components
        .insert("sensors".into(), "optics_night".into());
    let q = companies::development_quote(&w, HOME, c, "S03 optics upgrade", &spec, 1.0, 1);
    assert!(q.valid, "{:?}", q.reason);
    apply(
        &mut w,
        companies::CompanyOrder::Develop {
            company: c,
            name: "S03 optics upgrade".into(),
            spec,
            daily_budget_bn: 1.0,
            stock_target: 1,
            quote: q.token,
        },
    );
    let target_product = firm(&w, c).products.last().unwrap().id;
    until_stock(&mut w, c, target_product, 1);
    adopt(&mut w, &d);
    endowment_components(&mut w, 100.0);
    let q = companies::refit_quote(&w, HOME, c, &source, target_product, 1);
    assert!(q.valid, "{:?}", q.reason);
    let fixed = q.cost_bn;
    apply(
        &mut w,
        companies::CompanyOrder::Refit {
            company: c,
            source: source.clone(),
            product: target_product,
            quantity: 1,
            quote: q.token,
        },
    );
    let id = firm(&w, c).refits.last().unwrap().id;
    assert!(w.supplier_operations.contracts.contains_key(&id));
    isolated_day(&mut w); // Deposit settles; no same-day work.
    let escrow = firm(&w, c).refits.last().unwrap().escrow_bn;
    near(escrow, fixed);
    // An exactly sufficient corporate cash balance must start atomically,
    // even when subtraction of the input bill has a sub-ulp rounding residue.
    let mut exact = w.clone();
    let required = ops::snapshot(&exact, HOME)["companies"][0]["next_packet"]
        ["company_cash_required_bn"]
        .as_f64()
        .unwrap();
    let f = exact
        .companies
        .firms
        .iter_mut()
        .find(|f| f.id == c)
        .unwrap();
    f.capital_received_bn -= f.cash_bn - required;
    f.cash_bn = required;
    companies::tick_day(&mut exact);
    assert!(firm(&exact, c)
        .refits
        .last()
        .unwrap()
        .unit_started_day
        .is_some());
    near(firm(&exact, c).cash_bn, 0.0);
    companies::validate_state(&exact).unwrap();
    ops::validate(&exact).unwrap();
    let people = w.population_system.clone();
    staffed(&mut w, &d, 0.25);
    companies::tick_day(&mut w);
    w.population_system = people;
    let r = firm(&w, c).refits.last().unwrap();
    near(r.unit_work_days, 0.25);
    near(r.escrow_bn, fixed);
    near(r.earned_revenue_bn, 0.0);
    assert_eq!(
        w.nation(HOME)
            .arsenal
            .held
            .iter()
            .find(|h| h.design_id.as_deref() == Some(&source))
            .unwrap()
            .refit_reserved,
        1
    );
    near(
        w.supplier_operations.contracts[&id].components_used,
        ops::components_for(r.unit_fabrication_cost_bn),
    );
    let components = industry_operations::advanced_component_stock(&w, HOME);
    ops::validate(&w).unwrap();
    companies::validate_state(&w).unwrap();
    let saved = spheres_sim::save(&w);
    let mut replay = spheres_sim::load(&saved).unwrap();
    clock::advance_date(&mut w);
    clock::advance_date(&mut replay);
    companies::tick_day(&mut w);
    companies::tick_day(&mut replay);
    near(
        industry_operations::advanced_component_stock(&w, HOME),
        components,
    );
    assert_eq!(spheres_sim::save(&w), spheres_sim::save(&replay));
    for _ in 0..100 {
        if firm(&w, c).refits.last().unwrap().completed_units == 1 {
            break;
        }
        isolated_day(&mut w);
    }
    let r = firm(&w, c).refits.last().unwrap();
    assert_eq!(r.completed_units, 1);
    near(r.total_price_bn, fixed);
    near(r.earned_revenue_bn, fixed);
    near(r.escrow_bn, 0.0);
    assert!(!w
        .nation(HOME)
        .arsenal
        .held
        .iter()
        .any(|h| h.design_id.as_deref() == Some(&source) && h.units > 0.0));
    ops::validate(&w).unwrap();
    companies::validate_state(&w).unwrap();
}
