//! EF05: pre-adoption property shares the same existing factory across the
//! connected-economy boundary. Opening plants, stock, cash and a certified
//! design are synthetic endowments; every order below is created by paid work.
use spheres_sim::{
    apply_command, arsenal, clock, companies, connected_economy, equipment,
    init::world_1990, load, manufacturing, production::ProvinceCapabilities,
    programs, resources, save, Command,
};
use spheres_sim::world::{GameRules, NationId as N, WorldState, BUDGET_DEFENSE as D};

const HOME: N = N::France;
const OPENING_CASH: f64 = 1000.0;
const REVISION: &str = "ef05-existing-public-tank";

fn near(actual: f64, expected: f64) {
    assert!((actual - expected).abs() < 1e-9, "{actual:.12} != {expected:.12}");
}

fn procurement_authority(w: &mut WorldState, amount: f64) {
    let p = w.nation_mut(HOME).program_budget.as_mut().unwrap();
    // Controlled daily appropriation, never a cash receipt or prepaid grant.
    p.available_bn[D][3] = amount;
    p.prepaid_bn[D][3] = 0.0;
}

fn close_fiscal_day(w: &mut WorldState) -> f64 {
    let before = w.nation(HOME).treasury_bn.unwrap();
    let p = w.nation_mut(HOME).program_budget.as_mut().unwrap();
    // Isolate actual purchases and standing services from tax/interest flows.
    // The public settlement function still owns the only cash posting.
    p.revenue_today_bn = 0.0;
    p.interest_today_bn = 0.0;
    p.fiscal_staged = true;
    let invoice = p.spent_today_bn.iter().flatten().sum::<f64>();
    programs::finish_day(w);
    companies::settle_receivables(w);
    near(before - w.nation(HOME).treasury_bn.unwrap(), invoice);
    assert_eq!(w.nation(HOME).debt_bn, Some(0.0));
    invoice
}

fn next_work_day(w: &mut WorldState, budget: f64) -> f64 {
    clock::advance_date(w);
    programs::begin_day(w);
    // The game driver rebuilds the derived resource ledger before Arsenal.
    // A reloaded world intentionally starts with a cold, unserialized cache.
    resources::tick(w);
    procurement_authority(w, budget);
    equipment::tick_day(w);
    // This fixture funds only the frozen custom batch after the first naval
    // purchase. Unused authorization is withdrawn, not spent or refunded;
    // the unfunded old naval line must keep its physical reservation.
    procurement_authority(w, 0.0);
    arsenal::tick(w);
    close_fiscal_day(w)
}

fn custom_job(w: &WorldState, id: u32) -> &equipment::EquipmentProject {
    w.nation(HOME).equipment.as_ref().unwrap().projects.iter()
        .find(|p| p.id == id).unwrap()
}

fn custom_units(w: &WorldState) -> u32 {
    w.nation(HOME).arsenal.held.iter()
        .filter(|h| h.design_id.as_deref() == Some(REVISION))
        .map(arsenal::available_design_units).sum()
}

fn first_difference(a: &serde_json::Value, b: &serde_json::Value, path: &str) -> Option<String> {
    if a == b { return None; }
    match (a, b) {
        (serde_json::Value::Object(left), serde_json::Value::Object(right)) => {
            let keys: std::collections::BTreeSet<_> = left.keys().chain(right.keys()).collect();
            for key in keys {
                let next = format!("{path}.{key}");
                match (left.get(key), right.get(key)) {
                    (Some(x), Some(y)) => {
                        if let Some(difference) = first_difference(x, y, &next) { return Some(difference); }
                    }
                    _ => return Some(format!("{next}: key presence differs")),
                }
            }
        }
        (serde_json::Value::Array(left), serde_json::Value::Array(right)) => {
            if left.len() != right.len() {
                return Some(format!("{path}: array lengths {} / {}", left.len(), right.len()));
            }
            for (index, (x, y)) in left.iter().zip(right).enumerate() {
                if let Some(difference) = first_difference(x, y, &format!("{path}[{index}]")) {
                    return Some(difference);
                }
            }
        }
        _ => return Some(format!("{path}: {a} / {b}")),
    }
    Some(format!("{path}: unequal structure"))
}

fn assert_same_save(a: &WorldState, b: &WorldState, elapsed: u32) {
    let left = save(a);
    let right = save(b);
    if left != right {
        let left: serde_json::Value = serde_json::from_str(&left).unwrap();
        let right: serde_json::Value = serde_json::from_str(&right).unwrap();
        panic!("save/resume diverged on delivery day {elapsed}: {}",
            first_difference(&left, &right, "$").unwrap_or_else(|| "serialization only".into()));
    }
}

fn paid_property(w: &WorldState) -> serde_json::Value {
    let n = w.nation(HOME);
    serde_json::json!({
        "factory": w.production.provinces,
        "lines": w.manufacturing,
        "companies": w.companies,
        "equipment": n.equipment,
        "arsenal": n.arsenal,
        "program_budget": n.program_budget,
        "resources": w.resources,
        "cash": n.treasury_bn,
        "debt": n.debt_bn,
        "debt_gdp": n.debt_gdp,
        "political_capital": n.political_capital,
    })
}

fn assert_slots(w: &WorldState, district: &str, naval: u32, company: u32) {
    assert_eq!(manufacturing::plant_slots(w, district), 3);
    assert_eq!(manufacturing::used_slots(w, HOME, district), 3);
    assert_eq!(equipment::reserved_site_slots(w.nation(HOME), district), 1);
    assert_eq!(companies::reserved_slots(w, HOME, district), 1);
    assert_eq!(manufacturing::naval_slots(w, district), 0);
    assert_eq!(manufacturing::used_naval_slots(w, HOME, district), 0);
    assert!(!w.manufacturing.shipyard_lines.contains(&naval),
        "adoption cannot rewrite an old Arms Plant entitlement as a dock lease");
    let line = manufacturing::lines_for(w, HOME).find(|l| l.id == naval).unwrap();
    assert_eq!(line.kit, "nav_patrol");
    assert!(manufacturing::line_blocker(w, line).is_none());
    let firm = companies::company(w, HOME, company).unwrap();
    assert_eq!(firm.district, district);
    assert!(companies::facility_blocker(w, firm).is_none());
    assert!(manufacturing::start_line_error(w, HOME, district, "nav_patrol").is_some(),
        "a fourth claimant cannot mint an additional factory slot");
}

#[test]
fn adoption_and_reload_keep_paid_deliveries_and_three_existing_factory_entitlements() {
    let mut w = world_1990(GameRules {
        daily_simulation: true, production_system: true, manufacturing_system: true,
        military_operations: true, resource_market: true, resource_gates: true,
        ai_aggression: 0.0, crisis_intensity: 0.0, ..Default::default()
    });
    w.player = Some(HOME);
    w.conflicts.clear();
    {
        let n = w.nation_mut(HOME);
        n.political_capital = 1000.0;
        n.treasury_bn = Some(OPENING_CASH);
        n.debt_bn = Some(0.0);
        n.debt_gdp = 0.0;
        n.arsenal.held.clear();
        n.arsenal.orders.clear();
        n.arsenal.banked = 0.0;
    }
    programs::set_construction_budget(&mut w, HOME, 0.0).unwrap();
    resources::tick(&mut w);
    programs::begin_day(&mut w);
    procurement_authority(&mut w, 2.0);
    let district = w.districts.iter().find(|(_, n)| **n == HOME).unwrap().0.clone();
    w.production.provinces.retain(|p| p.district != district);
    w.production.provinces.push(ProvinceCapabilities {
        district: district.clone(), infrastructure: 0, civilian_industry: 0,
        power_grid: 0, research_centers: 0, arms_plants: 3,
    });
    w.production.provinces.sort_by(|a, b| a.district.cmp(&b.district));
    let market = w.resources.market.as_mut().unwrap();
    market.stocks.retain(|s| s.nation != HOME);
    for commodity in resources::ALL {
        market.stocks.push(resources::Stock {
            nation: HOME, commodity, quantity: 1_000_000.0, reserve_target: 0.0,
        });
    }
    market.stocks.sort_by_key(|s| (s.nation, s.commodity));

    let naval = manufacturing::start_line(&mut w, HOME, &district, "nav_patrol").unwrap();
    let spec = equipment::baseline_spec();
    let preview = equipment::design_preview(&w, HOME, &spec);
    assert!(preview.valid, "{:?}", preview.blockers);
    let profile = preview.profile.unwrap();
    let days_to_first_unit = profile.tooling_days + profile.production_days;
    let first_unit_cost = profile.tooling_cost_bn + profile.fabrication_cost_bn;
    let opening_day = clock::absolute_day(&w);
    w.nation_mut(HOME).equipment.get_or_insert_with(Default::default).revisions.insert(
        REVISION.into(), equipment::DesignRevision {
            id: REVISION.into(), name: "Existing public tank".into(),
            specification_key: equipment::specification_key(&spec), spec, profile,
            created_day: opening_day, certified_day: Some(opening_day),
        },
    );
    let public = equipment::start_production(&mut w, HOME, REVISION, &district, 2, 1.0).unwrap();
    let quote = companies::establishment_quote(&w, HOME, "EF05 Supplier", &district, 1.0);
    assert!(quote.valid, "{:?}", quote.reason);
    apply_command(&mut w, &Command::Company {
        nation: HOME,
        order: companies::CompanyOrder::Establish {
            name: "EF05 Supplier".into(), district: district.clone(),
            capitalization_bn: 1.0, quote: quote.token,
        },
    }).unwrap();
    let company = w.companies.firms.last().unwrap().id;
    near(companies::company(&w, HOME, company).unwrap().cash_bn, 0.0);
    let kit = arsenal::index_of("nav_patrol").unwrap();
    let naval_price = arsenal::DECK[kit as usize].unit_cost;
    procurement_authority(&mut w, naval_price);
    arsenal::tick(&mut w);
    let line = manufacturing::lines_for(&w, HOME).find(|l| l.id == naval).unwrap();
    near(line.ordered_bn, naval_price);
    assert!(line.resources_used.iter().any(|x| *x > 0.0));
    let naval_resources = line.resources_used;
    near(w.nation(HOME).arsenal.orders[0].units, 1.0);
    let mut total_paid = close_fiscal_day(&mut w);
    near(companies::company(&w, HOME, company).unwrap().cash_bn, 1.0);
    assert!(companies::company(&w, HOME, company).unwrap().receivables.is_empty());

    // Run normal per-day work and payment for the first actual unit. No order,
    // spent amount, work progress, input receipt or lead time is fabricated.
    for _ in 0..days_to_first_unit {
        total_paid += next_work_day(&mut w, 1.0);
    }
    assert_eq!(custom_job(&w, public).completed_units, 1);
    assert_eq!(custom_job(&w, public).quantity, 2);
    near(custom_job(&w, public).spent_bn, first_unit_cost);
    assert!(custom_job(&w, public).resources_used.iter().any(|x| *x > 0.0));
    assert_eq!(custom_units(&w), 0);
    let incoming = w.nation(HOME).arsenal.orders.iter()
        .find(|o| o.design_id.as_deref() == Some(REVISION)).unwrap();
    assert_eq!(incoming.due_days, Some(equipment::DELIVERY_DAYS));
    near(incoming.units, 1.0);
    equipment::set_project_paused(&mut w, HOME, public, true).unwrap();
    let frozen_job = custom_job(&w, public).clone();
    let frozen_company = serde_json::to_value(&w.companies).unwrap();
    let naval_remaining = w.nation(HOME).arsenal.orders.iter()
        .find(|o| o.design_id.is_none() && o.kit == kit).unwrap().due_days.unwrap();
    assert!(naval_remaining > equipment::DELIVERY_DAYS,
        "the older paid naval order keeps its real catalogue lead time");
    assert_slots(&w, &district, naval, company);
    near(OPENING_CASH - w.nation(HOME).treasury_bn.unwrap(), total_paid);
    assert!(!connected_economy::has_state(&w));
    let legacy_save = save(&w);
    assert!(save(&load(&legacy_save).unwrap()) == legacy_save,
        "the populated legacy equipment save must be a valid adoption input");

    let property_before = paid_property(&w);
    let adopt = Command::EnableConnectedEconomy { nation: HOME };
    apply_command(&mut w, &adopt).unwrap();
    assert_eq!(paid_property(&w), property_before,
        "adoption preserves payment, ownership, frozen work and input receipts");
    assert_slots(&w, &district, naval, company);
    let adopted = save(&w);
    assert_eq!(serde_json::from_str::<serde_json::Value>(&adopted).unwrap()["format"],
        "spheres-economy-save");
    apply_command(&mut w, &adopt).unwrap();
    assert!(save(&w) == adopted, "repeated adoption cannot grant cash or reset deliveries");
    let mut resumed = load(&adopted).unwrap();
    assert!(save(&resumed) == adopted, "the versioned save retains all paid property");

    for elapsed in 1..=equipment::DELIVERY_DAYS {
        total_paid += next_work_day(&mut w, 0.0);
        next_work_day(&mut resumed, 0.0);
        let expected = u32::from(elapsed == equipment::DELIVERY_DAYS);
        assert_eq!(custom_units(&w), expected);
        assert_eq!(custom_units(&resumed), expected);
        assert_same_save(&w, &resumed, elapsed);
        assert_slots(&w, &district, naval, company);
        let once = save(&resumed);
        equipment::tick_day(&mut resumed);
        arsenal::tick(&mut resumed);
        programs::finish_day(&mut resumed);
        companies::settle_receivables(&mut resumed);
        assert!(save(&resumed) == once, "re-entering settlement cannot double-post day {elapsed}");
    }

    assert_eq!(serde_json::to_value(&w.companies).unwrap(), frozen_company);
    let after_job = custom_job(&w, public);
    assert_eq!(after_job.work_days, frozen_job.work_days);
    assert_eq!(after_job.spent_bn, frozen_job.spent_bn);
    assert_eq!(after_job.cost_bn, frozen_job.cost_bn);
    assert_eq!(after_job.resources_used, frozen_job.resources_used);
    assert_eq!(after_job.completed_units, frozen_job.completed_units);
    assert!(after_job.paused);
    let after_line = manufacturing::lines_for(&w, HOME).find(|l| l.id == naval).unwrap();
    near(after_line.ordered_bn, naval_price);
    assert_eq!(after_line.resources_used, naval_resources);
    let remaining_orders = &w.nation(HOME).arsenal.orders;
    assert_eq!(remaining_orders.len(), 1, "only the still-due paid naval order remains");
    assert_eq!(remaining_orders[0].kit, kit);
    assert!(remaining_orders[0].design_id.is_none());
    assert_eq!(remaining_orders[0].due_days, Some(naval_remaining - equipment::DELIVERY_DAYS));
    near(remaining_orders[0].units, 1.0);
    near(OPENING_CASH - w.nation(HOME).treasury_bn.unwrap(), total_paid);
    near(companies::company(&w, HOME, company).unwrap().cash_bn, 1.0);
    assert_eq!(custom_units(&w), 1, "the paid vehicle is delivered exactly once");
    connected_economy::validate(&w).unwrap();
    let delivered_save = save(&w);
    assert!(save(&load(&delivered_save).unwrap()) == delivered_save,
        "paid delivery, unfinished work and factory reservations remain reloadable");
}
