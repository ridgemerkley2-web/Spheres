//! Finite supplier ammunition uses paid company work, a reviewed public sale,
//! and one dated arrival. Initial money, raw stocks, compatible certifications
//! and the test factory are explicit synthetic endowments, never game grants.
use spheres_sim::world::{GameRules, NationId as N, WorldState, BUDGET_DEFENSE as D};
use spheres_sim::{
    arsenal, clock, companies, equipment, manufacturing, production, programs, resources, Command,
    EquipmentOrder,
};

const HOME: N = N::France;
const GROUND: &str = "mg_127";
const AIR: &str = "air_bomb_unguided";

fn near(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < 1e-12 + 1e-12 * actual.abs().max(expected.abs()),
        "{actual:.15} != {expected:.15}"
    );
}

fn company_order(w: &mut WorldState, order: companies::CompanyOrder) {
    spheres_sim::apply_command(
        w,
        &Command::Company {
            nation: HOME,
            order,
        },
    )
    .unwrap();
}

fn equipment_order(w: &mut WorldState, order: EquipmentOrder) {
    spheres_sim::apply_command(
        w,
        &Command::Equipment {
            nation: HOME,
            order,
        },
    )
    .unwrap();
}

fn compatible_spec(family: &str) -> equipment::DesignSpec {
    let mut spec;
    if family.starts_with("tank_") {
        let parts: Vec<_> = family.split('_').collect();
        spec = equipment::default_spec("tank_heavy");
        spec.components
            .insert("armament".into(), format!("gun_{}", parts[1]));
        spec.components
            .insert("ammunition".into(), format!("ammo_{}", parts[2]));
        if parts[1] == "125" {
            spec.components
                .insert("turret".into(), "turret_autoload".into());
        }
    } else if family.starts_with("autocannon_") {
        spec = equipment::default_spec("ground_ifv");
        spec.components.insert(
            "armament".into(),
            format!("ground_gun_{}", family.split('_').nth(1).unwrap()),
        );
    } else if family == GROUND {
        spec = equipment::default_spec("ground_apc");
    } else if family.starts_with("howitzer_") {
        let parts: Vec<_> = family.split('_').collect();
        spec = equipment::default_spec("ground_artillery");
        spec.components
            .insert("armament".into(), format!("ground_howitzer_{}", parts[1]));
        spec.components
            .insert("ammunition".into(), format!("ground_ammo_{}", parts[2]));
        if parts[2] == "guided" {
            spec.components
                .insert("fire_control".into(), "fcs_digital".into());
        }
    } else if family.starts_with("aa_") {
        spec = equipment::default_spec("ground_air_defense");
        if family == "aa_missile" {
            for (slot, component) in [
                ("armament", "ground_aa_missiles"),
                ("ammunition", "ground_ammo_missiles"),
                ("radar", "ground_radar_tracking"),
                ("fire_control", "fcs_digital"),
            ] {
                spec.components.insert(slot.into(), component.into());
            }
        }
    } else {
        spec = equipment::default_spec("air_light_attack");
        if family == "air_bomb_guided" {
            spec.components
                .insert("air_payload".into(), "air_payload_guided".into());
            spec.components
                .insert("air_avionics".into(), "air_avionics_digital".into());
        }
    }
    assert_eq!(equipment::ammunition_family(&spec), Some(family));
    spec
}

fn certify(w: &mut WorldState, family: &str) -> String {
    let spec = compatible_spec(family);
    let preview = equipment::design_preview(w, HOME, &spec);
    assert!(preview.valid, "{family}: {:?}", preview.blockers);
    let id = format!("synthetic-{family}");
    let day = clock::absolute_day(w);
    w.nation_mut(HOME)
        .equipment
        .as_mut()
        .unwrap()
        .revisions
        .insert(
            id.clone(),
            equipment::DesignRevision {
                id: id.clone(),
                name: format!("Synthetic {family} carrier"),
                spec,
                specification_key: preview.specification_key,
                profile: preview.profile.unwrap(),
                created_day: day,
                certified_day: Some(day),
            },
        );
    id
}

fn fixture(families: &[&str]) -> (WorldState, String) {
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
            .find(|s| s.nation == HOME && s.commodity == commodity)
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
    market.stocks.sort_by_key(|s| (s.nation, s.commodity));
    let n = w.nation_mut(HOME);
    n.treasury_bn = Some(100.0);
    n.debt_bn = Some(0.0);
    n.debt_gdp = 0.0;
    n.arsenal.held.clear();
    n.arsenal.orders.clear();
    n.arsenal.banked = 0.0;
    let plan = n.program_budget.as_mut().unwrap();
    for department in [2, 3, 4] {
        plan.available_bn[D][department] = 10.0;
        plan.prepaid_bn[D][department] = 0.0;
    }
    n.equipment = Some(equipment::EquipmentState::default());
    n.equipment.as_mut().unwrap().learned =
        equipment::RESEARCH.iter().map(|r| r.id.into()).collect();
    for family in families {
        certify(&mut w, family);
    }
    equipment_order(
        &mut w,
        EquipmentOrder::Maintenance {
            daily_budget_bn: 0.001,
        },
    );
    (w, district)
}

fn firm(w: &WorldState, company: u32) -> &companies::Company {
    companies::company(w, HOME, company).unwrap()
}
fn ammo(w: &WorldState, company: u32, product: u32) -> &companies::AmmoProduct {
    firm(w, company)
        .ammunition_products
        .iter()
        .find(|p| p.id == product)
        .unwrap()
}
fn national_stock(w: &WorldState, family: &str) -> f64 {
    w.nation(HOME)
        .equipment
        .as_ref()
        .and_then(|s| s.ammunition.as_ref())
        .and_then(|a| a.stocks.get(family))
        .copied()
        .unwrap_or(0.0)
}
fn activation(w: &WorldState) -> Option<i32> {
    w.nation(HOME)
        .equipment
        .as_ref()
        .and_then(|s| s.ammunition.as_ref())
        .and_then(|a| a.active_from_day)
}
fn establish(w: &mut WorldState, district: &str, capital: f64) -> u32 {
    let q = companies::establishment_quote(w, HOME, "Synthetic Ordnance Works", district, capital);
    assert!(q.valid, "{:?}", q.reason);
    company_order(
        w,
        companies::CompanyOrder::Establish {
            name: "Synthetic Ordnance Works".into(),
            district: district.into(),
            capitalization_bn: capital,
            quote: q.token,
        },
    );
    w.companies.firms.last().unwrap().id
}
fn supply(w: &mut WorldState, company: u32, family: &str, target: u32) -> u32 {
    let q = companies::ammo_supply_quote(w, HOME, company, family, target);
    assert!(q.valid, "{:?}", q.reason);
    company_order(
        w,
        companies::CompanyOrder::AmmoSupply {
            company,
            family: family.into(),
            stock_target: target,
            quote: q.token,
        },
    );
    firm(w, company).ammunition_products.last().unwrap().id
}
fn purchase(w: &mut WorldState, company: u32, product: u32, quantity: u32) -> companies::Quote {
    let q = companies::ammo_purchase_quote(w, HOME, company, product, quantity);
    assert!(q.valid, "{:?}", q.reason);
    company_order(
        w,
        companies::CompanyOrder::AmmoPurchase {
            company,
            product,
            quantity,
            quote: q.token.clone(),
        },
    );
    q
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
fn close_day(w: &mut WorldState) -> f64 {
    let plan = w.nation_mut(HOME).program_budget.as_mut().unwrap();
    plan.revenue_today_bn = 0.0;
    plan.interest_today_bn = 0.0;
    plan.fiscal_staged = true;
    let fresh = plan.spent_today_bn.iter().flatten().sum();
    programs::finish_day(w);
    companies::settle_receivables(w);
    fresh
}
fn isolated_day(w: &mut WorldState) {
    renew_budget(w);
    programs::begin_day(w);
    equipment::tick_day(w);
    companies::tick_day(w);
    equipment::settle_support(w);
    close_day(w);
    clock::advance_date(w);
}
fn real_day(w: &mut WorldState) {
    renew_budget(w);
    let events = spheres_sim::tick_day(w, &[]);
    assert!(
        !events.iter().any(|e| e.starts_with("[rejected]")),
        "{events:?}"
    );
}
fn until_stock(w: &mut WorldState, company: u32, product: u32, wanted: u32) {
    for _ in 0..1000 {
        if ammo(w, company, product).stock >= wanted {
            return;
        }
        isolated_day(w);
    }
    panic!(
        "Ammunition did not reach shelf: {:?}",
        ammo(w, company, product)
    );
}
fn ready(family: &str, target: u32) -> (WorldState, String, u32, u32) {
    let (mut w, district) = fixture(&[family]);
    let company = establish(&mut w, &district, 1.0);
    isolated_day(&mut w);
    let product = supply(&mut w, company, family, target);
    until_stock(&mut w, company, product, target);
    (w, district, company, product)
}
fn reconcile(w: &WorldState, company: u32) {
    let c = firm(w, company);
    near(
        c.cash_bn,
        c.capital_received_bn + c.development_revenue_bn + c.sales_revenue_bn
            - c.development_expense_bn
            - c.tooling_expense_bn
            - c.materials_expense_bn
            - c.fabrication_expense_bn,
    );
    for p in &c.ammunition_products {
        assert_eq!(p.produced_units, p.stock + p.sold_units);
        for (used, per_round) in p
            .resources_used
            .iter()
            .zip(equipment::ammo_def(&p.family).unwrap().recipe)
        {
            near(*used, per_round * p.produced_units as f64);
        }
        near(
            p.fabrication_expense_bn,
            equipment::ammo_def(&p.family).unwrap().fabrication_bn * p.produced_units as f64,
        );
    }
}

#[test]
fn all_twenty_three_families_require_company_inputs_and_cash_then_one_paid_delivery() {
    assert_eq!(equipment::ammo_catalog().len(), 23);
    for def in equipment::ammo_catalog() {
        let (mut w, district) = fixture(&[def.id]);
        let company = establish(&mut w, &district, 1.0);
        isolated_day(&mut w);
        let target = def.rounds_per_day as u32 + 3;
        let opening_raw = resources::ALL.map(|c| resources::stockpile(&w, HOME, c));
        let opening_cash = firm(&w, company).cash_bn;
        let national_before = w
            .nation(HOME)
            .equipment
            .as_ref()
            .unwrap()
            .ammunition
            .clone();
        let product = supply(&mut w, company, def.id, target);
        assert_eq!(ammo(&w, company, product).stock, 0);
        assert_eq!(
            w.nation(HOME).equipment.as_ref().unwrap().ammunition,
            national_before
        );
        assert_eq!(manufacturing::used_slots(&w, HOME, &district), 1);
        until_stock(&mut w, company, product, target);
        let p = ammo(&w, company, product);
        assert_eq!(p.produced_units, target);
        assert_eq!(national_stock(&w, def.id), 0.0);
        assert_eq!(activation(&w), None);
        near(
            opening_cash - firm(&w, company).cash_bn,
            p.materials_expense_bn + p.fabrication_expense_bn,
        );
        for (idx, commodity) in resources::ALL.into_iter().enumerate() {
            let consumed = opening_raw[idx] - resources::stockpile(&w, HOME, commodity);
            let expected = target as f64 * def.recipe[idx];
            assert!(
                (consumed - expected).abs() < 1e-8,
                "{} raw conservation: {consumed} != {expected}",
                def.id
            );
        }
        reconcile(&w, company);
        let quantity = target - 1;
        let q = purchase(&mut w, company, product, quantity);
        assert_eq!(ammo(&w, company, product).stock, 1);
        assert_eq!(
            companies::ammo_inbound_units(&w, HOME, def.id),
            quantity as u64
        );
        assert_eq!(national_stock(&w, def.id), 0.0);
        assert_eq!(w.companies.ammunition_deliveries[0].settled_day, None);
        assert_eq!(w.companies.ammunition_deliveries[0].due_day, None);
        let pending_cash = firm(&w, company).cash_bn;
        companies::settle_receivables(&mut w);
        near(firm(&w, company).cash_bn, pending_cash);
        close_day(&mut w);
        near(firm(&w, company).cash_bn, pending_cash + q.cost_bn);
        company_order(
            &mut w,
            companies::CompanyOrder::AmmoInventory {
                company,
                product,
                stock_target: 0,
            },
        );
        let saved = spheres_sim::save(&w);
        let mut resumed = spheres_sim::load(&saved).unwrap();
        assert_eq!(spheres_sim::save(&resumed), saved);
        let due = w.companies.ammunition_deliveries[0].due_day.unwrap();
        while clock::absolute_day(&w) <= due {
            if clock::absolute_day(&w) < due {
                assert_eq!(national_stock(&w, def.id), 0.0);
            }
            isolated_day(&mut w);
            isolated_day(&mut resumed);
        }
        assert_eq!(spheres_sim::save(&resumed), spheres_sim::save(&w));
        near(national_stock(&w, def.id), quantity as f64);
        assert_eq!(companies::ammo_inbound_units(&w, HOME, def.id), 0);
        assert_eq!(
            activation(&w),
            None,
            "arriving {} cannot activate ground ammunition",
            def.id
        );
        let national = w
            .nation(HOME)
            .equipment
            .as_ref()
            .unwrap()
            .ammunition
            .as_ref()
            .unwrap();
        assert!(
            national.orders.is_empty(),
            "company purchases must not forge completed public manufacturing orders"
        );
        assert!(national.consumed.values().all(|v| *v == 0.0));
        assert!(w.nation(HOME).arsenal.held.is_empty());
        let before = spheres_sim::save(&w);
        companies::settle_receivables(&mut w);
        assert_eq!(spheres_sim::save(&w), before);
        reconcile(&w, company);
    }
}

#[test]
fn purchase_quantities_and_replayed_reviews_cannot_overdraw_stock_or_charge_twice() {
    let (mut w, _, company, product) = ready(GROUND, 9);
    for quantity in [0, 10, u32::MAX] {
        let before = spheres_sim::save(&w);
        let q = companies::ammo_purchase_quote(&w, HOME, company, product, quantity);
        assert!(!q.valid);
        assert!(spheres_sim::apply_command(
            &mut w,
            &Command::Company {
                nation: HOME,
                order: companies::CompanyOrder::AmmoPurchase {
                    company,
                    product,
                    quantity,
                    quote: q.token
                }
            }
        )
        .is_err());
        assert_eq!(spheres_sim::save(&w), before);
    }
    let old = companies::ammo_purchase_quote(&w, HOME, company, product, 3);
    purchase(&mut w, company, product, 2);
    let before = spheres_sim::save(&w);
    assert!(spheres_sim::apply_command(
        &mut w,
        &Command::Company {
            nation: HOME,
            order: companies::CompanyOrder::AmmoPurchase {
                company,
                product,
                quantity: 3,
                quote: old.token
            }
        }
    )
    .is_err());
    assert_eq!(spheres_sim::save(&w), before);
    assert_eq!(ammo(&w, company, product).stock, 7);
    assert_eq!(w.companies.ammunition_deliveries.len(), 1);
    let foreign = companies::ammo_purchase_quote(&w, N::USSR, company, product, 1);
    assert!(!foreign.valid);
    assert_eq!(spheres_sim::save(&w), before);
}

#[test]
fn ammunition_purchases_protect_fleet_upkeep_and_settle_fresh_or_prepaid_authority_once() {
    let (base, _, company, product) = ready(GROUND, 100);
    for prepaid in [false, true] {
        let mut w = base.clone();
        arsenal::deliver_design(w.nation_mut(HOME), "synthetic-mg_127", 10, 0.0).unwrap();
        programs::begin_day(&mut w);
        let upkeep = equipment::fleet_maintenance_requirement(w.nation(HOME));
        assert!(upkeep > 0.0);
        let cost = companies::ammo_purchase_quote(&w, HOME, company, product, 20).cost_bn;
        assert!(cost > 0.0);
        {
            let plan = w.nation_mut(HOME).program_budget.as_mut().unwrap();
            plan.available_bn[D][2] = upkeep + cost * 0.5;
            plan.prepaid_bn[D][2] = 0.0;
        }
        let before = spheres_sim::save(&w);
        let insufficient = companies::ammo_purchase_quote(&w, HOME, company, product, 20);
        assert!(
            !insufficient.valid,
            "ammunition cannot consume the unpaid fleet's maintenance reservation"
        );
        assert!(spheres_sim::apply_command(
            &mut w,
            &Command::Company {
                nation: HOME,
                order: companies::CompanyOrder::AmmoPurchase {
                    company,
                    product,
                    quantity: 20,
                    quote: insufficient.token
                }
            }
        )
        .is_err());
        assert_eq!(spheres_sim::save(&w), before);
        {
            let plan = w.nation_mut(HOME).program_budget.as_mut().unwrap();
            plan.available_bn[D][2] = upkeep + if prepaid { 0.0 } else { cost };
            plan.prepaid_bn[D][2] = if prepaid { cost } else { 0.0 };
        }
        let cash = firm(&w, company).cash_bn;
        let public = w.nation(HOME).treasury_bn.unwrap() - w.nation(HOME).debt_bn.unwrap();
        purchase(&mut w, company, product, 20);
        near(firm(&w, company).cash_bn, cash);
        equipment::settle_support(&mut w);
        let receipt = w
            .nation(HOME)
            .equipment
            .as_ref()
            .unwrap()
            .maintenance_plan
            .as_ref()
            .unwrap()
            .receipt
            .as_ref()
            .unwrap();
        near(receipt.custom_paid_bn, upkeep);
        let fresh = close_day(&mut w);
        near(
            public - (w.nation(HOME).treasury_bn.unwrap() - w.nation(HOME).debt_bn.unwrap()),
            fresh,
        );
        near(firm(&w, company).cash_bn, cash + cost);
        let plan = w.nation(HOME).program_budget.as_ref().unwrap();
        near(
            plan.prepaid_used_today_bn[D][2],
            if prepaid { cost } else { 0.0 },
        );
        let saved = spheres_sim::save(&w);
        companies::settle_receivables(&mut w);
        programs::finish_day(&mut w);
        companies::settle_receivables(&mut w);
        assert_eq!(spheres_sim::save(&w), saved);
        spheres_sim::load(&saved).unwrap();
        reconcile(&w, company);
    }
}

#[test]
fn company_liquidity_and_complete_inputs_bound_whole_rounds_without_public_fabrication_bills() {
    let (mut w, district) = fixture(&[AIR]);
    let def = equipment::ammo_def(AIR).unwrap();
    let raw_cost: f64 = resources::ALL
        .into_iter()
        .map(|c| def.recipe[c.idx()] * resources::market_current_price(&w, c) / 1e9)
        .sum();
    let complete_cost = raw_cost + def.fabrication_bn;
    let company = establish(&mut w, &district, complete_cost * 0.4);
    isolated_day(&mut w);
    let product = supply(&mut w, company, AIR, 1);
    let opening_raw = w.resources.market.as_ref().unwrap().stocks.clone();
    for _ in 0..4 {
        isolated_day(&mut w);
    }
    assert_eq!(ammo(&w, company, product).stock, 0);
    assert_eq!(ammo(&w, company, product).produced_units, 0);
    assert_eq!(w.resources.market.as_ref().unwrap().stocks, opening_raw);
    near(firm(&w, company).fabrication_expense_bn, 0.0);
    near(firm(&w, company).materials_expense_bn, 0.0);
    let q = companies::capitalization_quote(&w, HOME, company, complete_cost);
    company_order(
        &mut w,
        companies::CompanyOrder::Capitalize {
            company,
            amount_bn: complete_cost,
            quote: q.token,
        },
    );
    companies::tick_day(&mut w);
    assert_eq!(
        ammo(&w, company, product).stock,
        0,
        "unsettled capital cannot manufacture a store"
    );
    close_day(&mut w);
    clock::advance_date(&mut w);
    let copper = w
        .resources
        .market
        .as_mut()
        .unwrap()
        .stocks
        .iter_mut()
        .find(|s| s.nation == HOME && s.commodity == resources::Commodity::Copper)
        .unwrap();
    let previous_copper = copper.quantity;
    copper.quantity = 0.0;
    let cash = firm(&w, company).cash_bn;
    isolated_day(&mut w);
    assert_eq!(ammo(&w, company, product).stock, 0);
    near(firm(&w, company).cash_bn, cash);
    w.resources
        .market
        .as_mut()
        .unwrap()
        .stocks
        .iter_mut()
        .find(|s| s.nation == HOME && s.commodity == resources::Commodity::Copper)
        .unwrap()
        .quantity = previous_copper;
    let saved = spheres_sim::save(&w);
    let mut resumed = spheres_sim::load(&saved).unwrap();
    isolated_day(&mut w);
    isolated_day(&mut resumed);
    assert_eq!(spheres_sim::save(&resumed), spheres_sim::save(&w));
    assert_eq!(ammo(&w, company, product).stock, 1);
    assert_eq!(national_stock(&w, AIR), 0.0);
    near(ammo(&w, company, product).stock_cost_bn, complete_cost);
    near(
        w.nation(HOME)
            .program_budget
            .as_ref()
            .unwrap()
            .spent_today_bn[D][2],
        0.0,
    );
    reconcile(&w, company);
}

#[test]
fn market_changes_affect_only_later_purchased_inputs_and_the_weighted_available_shelf_price() {
    let (mut w, district) = fixture(&[AIR]);
    let company = establish(&mut w, &district, 1.0);
    isolated_day(&mut w);
    let def = equipment::ammo_def(AIR).unwrap();
    let rate = def.rounds_per_day as u32;
    let product = supply(&mut w, company, AIR, rate * 2);
    until_stock(&mut w, company, product, rate);
    assert_eq!(ammo(&w, company, product).stock, rate);
    let first_basis = ammo(&w, company, product).stock_cost_bn;
    let first_price = companies::ammo_purchase_quote(&w, HOME, company, product, 1).unit_price_bn;
    let revision =
        w.nation(HOME).equipment.as_ref().unwrap().revisions["synthetic-air_bomb_unguided"].clone();
    for price in &mut w.resources.market.as_mut().unwrap().prices {
        *price *= 5.0;
    }
    near(
        companies::ammo_purchase_quote(&w, HOME, company, product, 1).unit_price_bn,
        first_price,
    );
    let second_raw: f64 = resources::ALL
        .into_iter()
        .map(|c| def.recipe[c.idx()] * resources::market_current_price(&w, c) / 1e9)
        .sum();
    isolated_day(&mut w);
    assert_eq!(ammo(&w, company, product).stock, rate * 2);
    let expected = first_basis + (second_raw + def.fabrication_bn) * rate as f64;
    near(ammo(&w, company, product).stock_cost_bn, expected);
    let quoted = companies::ammo_purchase_quote(&w, HOME, company, product, rate);
    near(quoted.unit_price_bn, expected / (rate * 2) as f64 * 1.15);
    purchase(&mut w, company, product, rate);
    near(ammo(&w, company, product).stock_cost_bn, expected * 0.5);
    assert_eq!(
        w.nation(HOME).equipment.as_ref().unwrap().revisions[&revision.id],
        revision
    );
    reconcile(&w, company);
    spheres_sim::load(&spheres_sim::save(&w)).unwrap();
}

#[test]
fn one_contractor_shares_one_daily_work_packet_across_vehicle_development_and_ammunition_families()
{
    let (mut w, district) = fixture(&[GROUND, AIR]);
    let company = establish(&mut w, &district, 1.0);
    isolated_day(&mut w);
    let first = supply(&mut w, company, GROUND, 25_003);
    let second = supply(&mut w, company, AIR, 63);
    let spec = equipment::default_spec("ground_ifv");
    let q =
        companies::development_quote(&w, HOME, company, "Synthetic IFV contract", &spec, 1.0, 1);
    assert!(q.valid, "{:?}", q.reason);
    company_order(
        &mut w,
        companies::CompanyOrder::Develop {
            company,
            name: "Synthetic IFV contract".into(),
            spec,
            daily_budget_bn: 1.0,
            stock_target: 1,
            quote: q.token,
        },
    );
    let vehicle = firm(&w, company).products[0].id;
    for _ in 0..4 {
        real_day(&mut w);
    }
    assert!(firm(&w, company).products[0].development_work_days > 0.0);
    assert_eq!(ammo(&w, company, first).stock, 0);
    assert_eq!(ammo(&w, company, second).stock, 0);
    assert_eq!(manufacturing::used_slots(&w, HOME, &district), 1);
    company_order(
        &mut w,
        companies::CompanyOrder::CancelDevelopment {
            company,
            product: vehicle,
        },
    );
    for _ in 0..5 {
        let old = [
            ammo(&w, company, first).produced_units,
            ammo(&w, company, second).produced_units,
        ];
        real_day(&mut w);
        let increments = [
            ammo(&w, company, first).produced_units - old[0],
            ammo(&w, company, second).produced_units - old[1],
        ];
        assert!(
            increments.iter().filter(|v| **v > 0).count() <= 1,
            "the single leased plant cannot produce two ammunition families in one day"
        );
    }
    assert_eq!(ammo(&w, company, first).stock, 25_003);
    assert_eq!(ammo(&w, company, second).stock, 63);
    assert!(w.nation(HOME).arsenal.held.is_empty());
    assert_eq!(national_stock(&w, GROUND), 0.0);
    assert_eq!(national_stock(&w, AIR), 0.0);
    reconcile(&w, company);
    spheres_sim::load(&spheres_sim::save(&w)).unwrap();
}

#[test]
fn converted_reserves_keep_targets_and_prior_paid_batches_but_net_company_transit_and_stop_duplicate_public_orders(
) {
    let (mut w, district) = fixture(&[GROUND]);
    w.production
        .provinces
        .iter_mut()
        .find(|p| p.district == district)
        .unwrap()
        .arms_plants = 2;
    equipment_order(
        &mut w,
        EquipmentOrder::AmmoOrder {
            family: GROUND.into(),
            district: district.clone(),
            quantity: 50_000,
            daily_budget_bn: 1.0,
        },
    );
    equipment_order(
        &mut w,
        EquipmentOrder::AmmoReserve {
            family: GROUND.into(),
            target_rounds: 60_000,
            district: district.clone(),
            daily_budget_bn: 1.0,
            automatic: true,
        },
    );
    let old = w
        .nation(HOME)
        .equipment
        .as_ref()
        .unwrap()
        .ammunition
        .as_ref()
        .unwrap()
        .orders[0]
        .id;
    // Exercise the already-funded public batch before enrolling a contractor;
    // isolate unrelated legacy catalogue purchasing from this ownership test.
    isolated_day(&mut w);
    isolated_day(&mut w);
    equipment_order(
        &mut w,
        EquipmentOrder::AmmoPause {
            project: old,
            paused: true,
        },
    );
    let paused_work = w
        .nation(HOME)
        .equipment
        .as_ref()
        .unwrap()
        .ammunition
        .as_ref()
        .unwrap()
        .orders[0]
        .clone();
    assert!(paused_work.completed_rounds > 0 && paused_work.spent_bn > 0.0);
    let company = establish(&mut w, &district, 1.0);
    let product = supply(&mut w, company, GROUND, 10_000);
    assert!(companies::ammo_supplier_active(&w, HOME, GROUND));
    assert_eq!(
        equipment::ammo_reserve_status(&w, HOME, GROUND).target_rounds,
        Some(60_000)
    );
    assert!(!equipment::ammo_reserve_status(&w, HOME, GROUND).can_schedule);
    assert_eq!(manufacturing::used_slots(&w, HOME, &district), 2);
    real_day(&mut w);
    real_day(&mut w);
    let paused = &w
        .nation(HOME)
        .equipment
        .as_ref()
        .unwrap()
        .ammunition
        .as_ref()
        .unwrap()
        .orders[0];
    assert_eq!(paused.completed_rounds, paused_work.completed_rounds);
    near(paused.spent_bn, paused_work.spent_bn);
    assert!(
        paused.paused,
        "conversion preserves the previous public batch's paid progress and pause"
    );
    equipment_order(
        &mut w,
        EquipmentOrder::AmmoPause {
            project: old,
            paused: false,
        },
    );
    for _ in 0..4 {
        real_day(&mut w);
    }
    let national = w
        .nation(HOME)
        .equipment
        .as_ref()
        .unwrap()
        .ammunition
        .as_ref()
        .unwrap();
    assert_eq!(national.orders.len(), 1);
    assert_eq!(national.orders[0].id, old);
    assert_eq!(
        national.orders[0].completed_rounds, 50_000,
        "conversion must complete existing paid public work"
    );
    assert!(national.orders[0].spent_bn > 0.0);
    near(national_stock(&w, GROUND), 50_000.0);
    assert_eq!(ammo(&w, company, product).stock, 10_000);
    let status = equipment::ammo_reserve_status(&w, HOME, GROUND);
    assert_eq!(
        status.gap, 10_000,
        "unsold supplier stock is no national reserve commitment"
    );
    assert_eq!(
        equipment::ammunition_supply_demand(&w, HOME).remaining,
        [0.0; 12],
        "converted reserve plans cannot forecast automatic public material draw"
    );
    purchase(&mut w, company, product, 7_000);
    let status = equipment::ammo_reserve_status(&w, HOME, GROUND);
    assert_eq!(status.committed, 7_000);
    near(status.projected, 57_000.0);
    assert_eq!(status.gap, 3_000);
    assert!(!status.can_schedule);
    company_order(
        &mut w,
        companies::CompanyOrder::AmmoInventory {
            company,
            product,
            stock_target: 0,
        },
    );
    for _ in 0..10 {
        real_day(&mut w);
    }
    near(national_stock(&w, GROUND), 57_000.0);
    assert_eq!(equipment::ammo_reserve_status(&w, HOME, GROUND).gap, 3_000);
    assert_eq!(
        w.nation(HOME)
            .equipment
            .as_ref()
            .unwrap()
            .ammunition
            .as_ref()
            .unwrap()
            .orders
            .len(),
        1
    );
    let new_public = equipment::ammo_order_quote(&w, HOME, GROUND, &district, 1, 1.0);
    assert!(
        new_public.valid,
        "explicit public commands remain compatible on a separate free slot; conversion suppresses automatic reserve batches"
    );
    reconcile(&w, company);
    spheres_sim::load(&spheres_sim::save(&w)).unwrap();
}

#[test]
fn in_transit_ammunition_pauses_outside_government_control_and_arrives_once_without_changing_activation(
) {
    let (mut w, district, company, product) = ready(GROUND, 12);
    equipment_order(&mut w, EquipmentOrder::AmmoActivate);
    let from = activation(&w);
    purchase(&mut w, company, product, 5);
    close_day(&mut w);
    company_order(
        &mut w,
        companies::CompanyOrder::AmmoInventory {
            company,
            product,
            stock_target: 0,
        },
    );
    let initial_due = w.companies.ammunition_deliveries[0].due_day.unwrap();
    let cash = firm(&w, company).cash_bn;
    w.districts.insert(district.clone(), N::USSR);
    for _ in 0..10 {
        isolated_day(&mut w);
    }
    assert_eq!(national_stock(&w, GROUND), 0.0);
    assert_eq!(activation(&w), from);
    near(firm(&w, company).cash_bn, cash);
    assert!(w.companies.ammunition_deliveries[0].due_day.unwrap() > initial_due);
    assert_eq!(companies::ammo_inbound_units(&w, HOME, GROUND), 5);
    assert_eq!(companies::ammo_inbound_units(&w, N::USSR, GROUND), 0);
    let serialized = spheres_sim::save(&w);
    let mut resumed = spheres_sim::load(&serialized).unwrap();
    assert_eq!(spheres_sim::save(&resumed), serialized);
    w.districts.insert(district.clone(), HOME);
    resumed.districts.insert(district, HOME);
    for _ in 0..10 {
        isolated_day(&mut w);
        isolated_day(&mut resumed);
    }
    assert_eq!(spheres_sim::save(&w), spheres_sim::save(&resumed));
    near(national_stock(&w, GROUND), 5.0);
    assert_eq!(activation(&w), from);
    assert_eq!(
        w.nation(HOME)
            .equipment
            .as_ref()
            .unwrap()
            .ammunition
            .as_ref()
            .unwrap()
            .supplier_receipts
            .len(),
        1
    );
    companies::tick_day(&mut w);
    let once = spheres_sim::save(&w);
    companies::tick_day(&mut w);
    companies::settle_receivables(&mut w);
    assert_eq!(spheres_sim::save(&w), once);
    reconcile(&w, company);
}

#[test]
fn old_company_save_versions_remain_exact_until_explicit_ammunition_supply() {
    for (platform, expected) in [("tank_standard", 2), ("ground_ifv", 3)] {
        let (mut w, district) = fixture(&[GROUND]);
        let company = establish(&mut w, &district, 1.0);
        let spec = equipment::default_spec(platform);
        let quote = companies::development_quote(
            &w,
            HOME,
            company,
            "Earlier equipment contract",
            &spec,
            1.0,
            1,
        );
        company_order(
            &mut w,
            companies::CompanyOrder::Develop {
                company,
                name: "Earlier equipment contract".into(),
                spec,
                daily_budget_bn: 1.0,
                stock_target: 1,
                quote: quote.token,
            },
        );
        let old = spheres_sim::save(&w);
        let envelope: serde_json::Value = serde_json::from_str(&old).unwrap();
        assert_eq!(envelope["version"], expected);
        assert!(
            !old.contains("ammunition_products")
                && !old.contains("ammunition_deliveries")
                && !old.contains("supplier_receipts")
        );
        let mut resumed = spheres_sim::load(&old).unwrap();
        assert_eq!(spheres_sim::save(&resumed), old);
        let product_before = firm(&w, company).products.clone();
        let profile_before = w.nation(HOME).equipment.as_ref().unwrap().revisions.clone();
        companies::ammo_supply_quote(&w, HOME, company, GROUND, 10);
        companies::view(&w, HOME);
        assert_eq!(
            spheres_sim::save(&w),
            old,
            "opening ammunition readers cannot migrate an old company save"
        );
        supply(&mut w, company, GROUND, 10);
        supply(&mut resumed, company, GROUND, 10);
        assert_eq!(spheres_sim::save(&w), spheres_sim::save(&resumed));
        assert_eq!(firm(&w, company).products, product_before);
        assert_eq!(
            w.nation(HOME).equipment.as_ref().unwrap().revisions,
            profile_before
        );
        let upgraded = spheres_sim::save(&w);
        let envelope: serde_json::Value = serde_json::from_str(&upgraded).unwrap();
        assert_eq!(envelope["version"], 4);
        assert_eq!(envelope["world"]["companies"]["version"], 3);
        assert_eq!(
            spheres_sim::save(&spheres_sim::load(&upgraded).unwrap()),
            upgraded
        );
        for old_envelope in [1, 2, 3] {
            let mut forged = envelope.clone();
            forged["version"] = old_envelope.into();
            assert!(spheres_sim::load(&forged.to_string()).is_err());
        }
        let mut forged = envelope.clone();
        forged["version"] = 3.into();
        forged["world"]["companies"]["version"] = 2.into();
        assert!(
            spheres_sim::load(&forged.to_string()).is_err(),
            "an ammunition book cannot hide inside an older equipment-only envelope"
        );
    }
}

#[test]
fn saves_reject_fabricated_rounds_unmatched_supplier_receipts_and_duplicate_purchase_invoices() {
    let (mut w, _, company, product) = ready(AIR, 8);
    purchase(&mut w, company, product, 3);
    let pending = spheres_sim::save(&w);
    spheres_sim::load(&pending).unwrap();
    let mut duplicate = w.clone();
    let mut receipt = duplicate.companies.firms[0]
        .receivables
        .iter()
        .find(|r| r.kind == "ammo_sale")
        .unwrap()
        .clone();
    receipt.id = duplicate.companies.next_id;
    duplicate.companies.next_id += 1;
    duplicate.companies.firms[0].receivables.push(receipt);
    assert!(spheres_sim::load(&spheres_sim::save(&duplicate)).is_err());
    let mut missing = w.clone();
    missing.companies.firms[0]
        .receivables
        .retain(|r| r.kind != "ammo_sale");
    assert!(spheres_sim::load(&spheres_sim::save(&missing)).is_err());
    let mut unfunded = w.clone();
    unfunded
        .nation_mut(HOME)
        .program_budget
        .as_mut()
        .unwrap()
        .spent_today_bn[D][2] = 0.0;
    unfunded
        .nation_mut(HOME)
        .program_budget
        .as_mut()
        .unwrap()
        .prepaid_used_today_bn[D][2] = 0.0;
    assert!(spheres_sim::load(&spheres_sim::save(&unfunded)).is_err());
    close_day(&mut w);
    company_order(
        &mut w,
        companies::CompanyOrder::AmmoInventory {
            company,
            product,
            stock_target: 0,
        },
    );
    for _ in 0..9 {
        isolated_day(&mut w);
    }
    assert_eq!(national_stock(&w, AIR), 3.0);
    spheres_sim::load(&spheres_sim::save(&w)).unwrap();
    let mut created = w.clone();
    created
        .nation_mut(HOME)
        .equipment
        .as_mut()
        .unwrap()
        .ammunition
        .as_mut()
        .unwrap()
        .stocks
        .insert(AIR.into(), 4.0);
    assert!(spheres_sim::load(&spheres_sim::save(&created)).is_err());
    let mut unmatched = w.clone();
    let mut fake = unmatched
        .nation(HOME)
        .equipment
        .as_ref()
        .unwrap()
        .ammunition
        .as_ref()
        .unwrap()
        .supplier_receipts[0]
        .clone();
    fake.delivery = unmatched.companies.next_id;
    fake.quantity = 1;
    let national = unmatched
        .nation_mut(HOME)
        .equipment
        .as_mut()
        .unwrap()
        .ammunition
        .as_mut()
        .unwrap();
    national.supplier_receipts.push(fake);
    *national.stocks.get_mut(AIR).unwrap() += 1.0;
    assert!(equipment::validate_ammunition(unmatched.nation(HOME)).is_ok(), "the world validator must enforce actual company provenance beyond native inventory arithmetic");
    assert!(spheres_sim::load(&spheres_sim::save(&unmatched)).is_err());
    let mut disappeared = w.clone();
    let national = disappeared
        .nation_mut(HOME)
        .equipment
        .as_mut()
        .unwrap()
        .ammunition
        .as_mut()
        .unwrap();
    national.supplier_receipts.clear();
    national.stocks.clear();
    assert!(spheres_sim::load(&spheres_sim::save(&disappeared)).is_err(), "an arrived shipment requires its native receipt even if both receipt and public stock are erased");
    let mut dates = w.clone();
    dates.companies.ammunition_deliveries[0].due_day =
        dates.companies.ammunition_deliveries[0].settled_day;
    assert!(spheres_sim::load(&spheres_sim::save(&dates)).is_err());
    let mut erased_company: serde_json::Value =
        serde_json::from_str(&spheres_sim::save(&w)).unwrap();
    erased_company["version"] = 1.into();
    erased_company["world"]
        .as_object_mut()
        .unwrap()
        .remove("companies");
    assert!(spheres_sim::load(&erased_company.to_string()).is_err(), "native supplier receipts cannot survive the erasure of their corporate payment and delivery provenance");
}

#[test]
fn delivered_company_air_stores_feed_existing_sorties_and_remain_conserved_after_consumption_and_reload(
) {
    use spheres_sim::world::{Belligerent, Conflict, Objective};
    let (mut w, district) = fixture(&[AIR]);
    arsenal::deliver_design(w.nation_mut(HOME), "synthetic-air_bomb_unguided", 5, 0.0).unwrap();
    let company = establish(&mut w, &district, 1.0);
    real_day(&mut w);
    let product = supply(&mut w, company, AIR, 120);
    for _ in 0..10 {
        if ammo(&w, company, product).stock == 120 {
            break;
        }
        real_day(&mut w);
    }
    purchase(&mut w, company, product, 60);
    company_order(
        &mut w,
        companies::CompanyOrder::AmmoInventory {
            company,
            product,
            stock_target: 0,
        },
    );
    for _ in 0..12 {
        if national_stock(&w, AIR) == 60.0 {
            break;
        }
        real_day(&mut w);
    }
    assert_eq!(national_stock(&w, AIR), 60.0);
    let theatre = spheres_sim::war::theatre_between(&w, HOME, N::Germany);
    assert!(spheres_sim::theatre::has_access(&w, HOME, theatre));
    w.conflicts.push(Conflict {
        id: 999,
        theatre,
        side_a: vec![HOME],
        side_b: vec![N::Germany],
        posture: vec![
            Belligerent::new(HOME, 6, Objective::Seize),
            Belligerent::new(N::Germany, 8, Objective::Hold),
        ],
        control: 0.0,
        months: 0,
        quiet_months: 0,
        frozen_since: None,
        start_year: w.year,
        start_month: w.month,
        origin_attacker: HOME,
        invasion_declared: true,
        front: Default::default(),
        pockets: vec![],
        aim: None,
    });
    assert!(equipment::ammunition_overview(&w, HOME)
        .families
        .iter()
        .any(|f| f.family == AIR && f.required > 0.0));
    let saved = spheres_sim::save(&w);
    let mut resumed = spheres_sim::load(&saved).unwrap();
    real_day(&mut w);
    real_day(&mut resumed);
    assert_eq!(spheres_sim::save(&w), spheres_sim::save(&resumed));
    let national = w
        .nation(HOME)
        .equipment
        .as_ref()
        .unwrap()
        .ammunition
        .as_ref()
        .unwrap();
    assert!(national.consumed[AIR] > 0.0);
    near(national.stocks[AIR] + national.consumed[AIR], 60.0);
    assert_eq!(national.supplier_receipts.len(), 1);
    assert!(national.orders.is_empty());
    assert_eq!(activation(&w), None);
    spheres_sim::load(&spheres_sim::save(&w)).unwrap();
    reconcile(&w, company);
}

#[test]
#[ignore = "Writes isolated synthetic browser stages only when SPHERES_COMPANY_AMMO_QA_DIR is explicitly set"]
fn export_company_ammunition_qa_stages() {
    let Some(output) = std::env::var_os("SPHERES_COMPANY_AMMO_QA_DIR") else {
        return;
    };
    let directory = std::path::PathBuf::from(output);
    assert!(directory.is_absolute());
    std::fs::create_dir_all(&directory).unwrap();
    let write = |name: &str, w: &WorldState| {
        let saved = spheres_sim::save(w);
        assert_eq!(
            spheres_sim::save(&spheres_sim::load(&saved).unwrap()),
            saved
        );
        std::fs::write(directory.join(name), saved).unwrap();
    };
    let (mut w, district) = fixture(&[GROUND, AIR]);
    let company = establish(&mut w, &district, 1.0);
    real_day(&mut w);
    let ground = supply(&mut w, company, GROUND, 30_000);
    let air = supply(&mut w, company, AIR, 120);
    for _ in 0..20 {
        if ammo(&w, company, ground).stock == 30_000 && ammo(&w, company, air).stock == 120 {
            break;
        }
        real_day(&mut w);
    }
    assert_eq!(ammo(&w, company, ground).stock, 30_000);
    assert_eq!(ammo(&w, company, air).stock, 120);
    assert_eq!(national_stock(&w, GROUND), 0.0);
    assert_eq!(national_stock(&w, AIR), 0.0);
    equipment_order(
        &mut w,
        EquipmentOrder::AmmoReserve {
            family: GROUND.into(),
            target_rounds: 50_000,
            district: district.clone(),
            daily_budget_bn: 0.001,
            automatic: false,
        },
    );
    equipment_order(
        &mut w,
        EquipmentOrder::AmmoReserve {
            family: AIR.into(),
            target_rounds: 180,
            district: district.clone(),
            daily_budget_bn: 0.001,
            automatic: false,
        },
    );
    write("01-ammunition-stock.json", &w);
    purchase(&mut w, company, ground, 20_000);
    purchase(&mut w, company, air, 60);
    for product in [ground, air] {
        company_order(
            &mut w,
            companies::CompanyOrder::AmmoInventory {
                company,
                product,
                stock_target: 0,
            },
        );
    }
    write("02-ammunition-awaiting-settlement.json", &w);
    real_day(&mut w);
    assert_eq!(national_stock(&w, GROUND), 0.0);
    assert_eq!(national_stock(&w, AIR), 0.0);
    write("03-ammunition-transit.json", &w);
    for _ in 0..12 {
        if national_stock(&w, GROUND) == 20_000.0 && national_stock(&w, AIR) == 60.0 {
            break;
        }
        real_day(&mut w);
    }
    assert_eq!(national_stock(&w, GROUND), 20_000.0);
    assert_eq!(national_stock(&w, AIR), 60.0);
    assert_eq!(activation(&w), None);
    write("04-ammunition-arrived.json", &w);
    std::fs::write(directory.join("README.txt"), format!("Synthetic company ammunition browser QA. France is endowed with explicit starting cash, authority, raw stocks, one Arms Plant in {district}, and certified APC/light-attack specifications. The compatible certifications are test prerequisites, not historical claims. One company ({company}) buys real warehouse inputs and fabricates 30,000 machine-gun rounds and 120 unguided aircraft stores through actual commands and complete daily ticks. Reviewed purchases buy 20,000 rounds and 60 stores, settle once, and arrive through the ordinary seven-day shipment path. No progress, completed production, receipt, or ammunition stock is fabricated. Physical ground ammunition remains unactivated. No live save is touched.\n")).unwrap();
    println!("Synthetic ammunition QA stages: {}", directory.display());
}
