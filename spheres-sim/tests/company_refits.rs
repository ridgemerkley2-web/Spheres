//! Reviewed supplier refits transform government-owned vehicles. Initial cash,
//! materials, source certifications and source vehicles are explicit synthetic
//! fixture inputs; supplier target development and all service work use commands.
use spheres_sim::world::{GameRules, NationId as N, WorldState, BUDGET_DEFENSE as D};
use spheres_sim::{
    arsenal, clock, companies, equipment, manufacturing, production, programs, resources, Command,
    EquipmentOrder,
};

const HOME: N = N::France;
const SOURCE: &str = "synthetic-source";

fn near(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= 1e-11 + 1e-11 * actual.abs().max(expected.abs()),
        "{actual:.15} != {expected:.15}"
    );
}
fn command(w: &mut WorldState, order: companies::CompanyOrder) {
    spheres_sim::apply_command(
        w,
        &Command::Company {
            nation: HOME,
            order,
        },
    )
    .unwrap();
}
fn equipment_command(w: &mut WorldState, order: EquipmentOrder) {
    spheres_sim::apply_command(
        w,
        &Command::Equipment {
            nation: HOME,
            order,
        },
    )
    .unwrap();
}
fn firm(w: &WorldState, id: u32) -> &companies::Company {
    companies::company(w, HOME, id).unwrap()
}
fn net_public(w: &WorldState) -> f64 {
    w.nation(HOME).treasury_bn.unwrap() - w.nation(HOME).debt_bn.unwrap()
}
fn holding<'a>(w: &'a WorldState, id: &str) -> Option<&'a arsenal::Holding> {
    w.nation(HOME)
        .arsenal
        .held
        .iter()
        .find(|h| h.design_id.as_deref() == Some(id))
}
fn units(w: &WorldState, id: &str) -> u32 {
    holding(w, id).map_or(0, |h| h.units as u32)
}
fn available(w: &WorldState, id: &str) -> u32 {
    holding(w, id).map_or(0, arsenal::available_design_units)
}
fn age_mass(w: &WorldState) -> f64 {
    w.nation(HOME)
        .arsenal
        .held
        .iter()
        .filter(|h| h.design_id.is_some())
        .map(|h| h.units * h.age)
        .sum()
}
fn total_units(w: &WorldState) -> u32 {
    w.nation(HOME)
        .arsenal
        .held
        .iter()
        .filter(|h| h.design_id.is_some())
        .map(|h| h.units as u32)
        .sum()
}

fn fixture(platform: &str) -> (WorldState, String, equipment::DesignSpec) {
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
    if let Some(p) = w
        .production
        .provinces
        .iter_mut()
        .find(|p| p.district == district)
    {
        p.arms_plants = 1;
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
        if let Some(s) = market
            .stocks
            .iter_mut()
            .find(|s| s.nation == HOME && s.commodity == commodity)
        {
            s.quantity = 1_000_000.0;
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
    for d in [2, 3, 4] {
        plan.available_bn[D][d] = 10.0;
        plan.prepaid_bn[D][d] = 0.0;
    }
    n.equipment = Some(equipment::EquipmentState::default());
    n.equipment.as_mut().unwrap().learned =
        equipment::RESEARCH.iter().map(|r| r.id.into()).collect();
    let source = equipment::default_spec(platform);
    let p = equipment::design_preview(&w, HOME, &source);
    assert!(p.valid, "{:?}", p.blockers);
    let day = clock::absolute_day(&w);
    w.nation_mut(HOME)
        .equipment
        .as_mut()
        .unwrap()
        .revisions
        .insert(
            SOURCE.into(),
            equipment::DesignRevision {
                id: SOURCE.into(),
                name: format!("Synthetic {} source", platform),
                spec: source.clone(),
                specification_key: p.specification_key,
                profile: p.profile.unwrap(),
                created_day: day,
                certified_day: Some(day),
            },
        );
    arsenal::deliver_design(w.nation_mut(HOME), SOURCE, 4, 120.0).unwrap();
    equipment_command(
        &mut w,
        EquipmentOrder::Maintenance {
            daily_budget_bn: 0.001,
        },
    );
    let mut target = source;
    if equipment::is_aviation_platform(platform) {
        target
            .components
            .insert("air_fuel".into(), "air_fuel_extended".into());
    } else {
        target
            .components
            .insert("sensors".into(), "optics_night".into());
    }
    let preview = equipment::design_preview(&w, HOME, &target);
    assert!(preview.valid, "{platform}: {:?}", preview.blockers);
    (w, district, target)
}
fn renew(w: &mut WorldState) {
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
fn close(w: &mut WorldState) -> f64 {
    let p = w.nation_mut(HOME).program_budget.as_mut().unwrap();
    p.revenue_today_bn = 0.0;
    p.interest_today_bn = 0.0;
    p.fiscal_staged = true;
    let fresh = p.spent_today_bn.iter().flatten().sum();
    programs::finish_day(w);
    companies::settle_receivables(w);
    fresh
}
fn day(w: &mut WorldState) {
    renew(w);
    programs::begin_day(w);
    equipment::tick_day(w);
    companies::tick_day(w);
    equipment::settle_support(w);
    close(w);
    clock::advance_date(w);
}
fn real_day(w: &mut WorldState) {
    renew(w);
    let events = spheres_sim::tick_day(w, &[]);
    assert!(
        !events.iter().any(|e| e.starts_with("[rejected]")),
        "{events:?}"
    );
}
fn establish(w: &mut WorldState, district: &str, capital: f64) -> u32 {
    let q = companies::establishment_quote(w, HOME, "Synthetic Refit Works", district, capital);
    assert!(q.valid, "{:?}", q.reason);
    command(
        w,
        companies::CompanyOrder::Establish {
            name: "Synthetic Refit Works".into(),
            district: district.into(),
            capitalization_bn: capital,
            quote: q.token,
        },
    );
    w.companies.firms.last().unwrap().id
}
fn ready(platform: &str) -> (WorldState, String, u32, u32, String) {
    ready_with_capital(platform, 1.0)
}
fn ready_with_capital(platform: &str, capital: f64) -> (WorldState, String, u32, u32, String) {
    ready_using(platform, capital, day)
}
fn ready_using(
    platform: &str,
    capital: f64,
    step: fn(&mut WorldState),
) -> (WorldState, String, u32, u32, String) {
    let (mut w, district, target) = fixture(platform);
    let company = establish(&mut w, &district, capital);
    step(&mut w);
    let q = companies::development_quote(
        &w,
        HOME,
        company,
        "Synthetic certified upgrade",
        &target,
        1.0,
        1,
    );
    assert!(q.valid, "{:?}", q.reason);
    command(
        &mut w,
        companies::CompanyOrder::Develop {
            company,
            name: "Synthetic certified upgrade".into(),
            spec: target,
            daily_budget_bn: 1.0,
            stock_target: 1,
            quote: q.token,
        },
    );
    let product = firm(&w, company).products.last().unwrap().id;
    let revision = firm(&w, company)
        .products
        .last()
        .unwrap()
        .revision_id
        .clone();
    for _ in 0..1000 {
        if firm(&w, company)
            .products
            .last()
            .unwrap()
            .certified_day
            .is_some()
        {
            break;
        }
        step(&mut w);
    }
    assert!(firm(&w, company)
        .products
        .last()
        .unwrap()
        .certified_day
        .is_some());
    command(
        &mut w,
        companies::CompanyOrder::Inventory {
            company,
            product,
            stock_target: 0,
        },
    );
    assert_eq!(units(&w, SOURCE), 4);
    assert_eq!(units(&w, &revision), 0);
    spheres_sim::load(&spheres_sim::save(&w)).unwrap();
    (w, district, company, product, revision)
}

fn book(w: &mut WorldState, company: u32, product: u32, quantity: u32) -> u32 {
    let q = companies::refit_quote(w, HOME, company, SOURCE, product, quantity);
    assert!(q.valid, "{:?}", q.reason);
    command(
        w,
        companies::CompanyOrder::Refit {
            company,
            source: SOURCE.into(),
            product,
            quantity,
            quote: q.token,
        },
    );
    firm(w, company).refits.last().unwrap().id
}
fn cancel(w: &mut WorldState, company: u32, refit: u32) -> f64 {
    let q = companies::refit_cancel_quote(w, HOME, company, refit);
    assert!(q.valid, "{:?}", q.reason);
    command(
        w,
        companies::CompanyOrder::CancelRefit {
            company,
            refit,
            quote: q.token,
        },
    );
    q.refund_bn
}
fn service(w: &WorldState, company: u32, id: u32) -> &companies::RefitContract {
    firm(w, company).refits.iter().find(|p| p.id == id).unwrap()
}
fn reconcile(w: &WorldState, company: u32) {
    let c = firm(w, company);
    let locked: f64 = c.refits.iter().map(|r| r.working_capital_locked_bn).sum();
    near(
        c.cash_bn + locked,
        c.capital_received_bn + c.development_revenue_bn + c.sales_revenue_bn + c.refit_revenue_bn
            - c.development_expense_bn
            - c.tooling_expense_bn
            - c.materials_expense_bn
            - c.fabrication_expense_bn,
    );
    near(
        c.refit_advances_received_bn,
        c.refits.iter().map(|r| r.advance_received_bn).sum(),
    );
    near(
        c.refit_revenue_bn,
        c.refits.iter().map(|r| r.earned_revenue_bn).sum(),
    );
    near(
        c.refit_refunds_bn,
        c.refits.iter().map(|r| r.refunded_bn).sum(),
    );
    for r in &c.refits {
        near(
            r.advance_received_bn,
            r.escrow_bn + r.earned_revenue_bn + r.refunded_bn,
        );
        near(
            r.earned_revenue_bn,
            r.completed_units as f64 * r.unit_price_bn,
        );
        near(
            r.refunded_bn,
            if r.settled_day.is_some() {
                r.cancelled_units as f64 * r.unit_price_bn
            } else {
                0.0
            },
        );
        near(
            r.fabrication_expense_bn + r.working_capital_locked_bn,
            (r.completed_units + u32::from(r.unit_started_day.is_some())) as f64
                * r.unit_fabrication_cost_bn,
        );
    }
    load_exact(w);
}
fn load_exact(w: &WorldState) -> WorldState {
    let saved = spheres_sim::save(w);
    let loaded = spheres_sim::load(&saved).unwrap();
    assert_same(w, &loaded);
    loaded
}
fn first_difference(a: &serde_json::Value, b: &serde_json::Value, path: &str) -> Option<String> {
    if a == b {
        return None;
    }
    match (a, b) {
        (serde_json::Value::Object(a), serde_json::Value::Object(b)) => {
            for (k, v) in a {
                if let Some(result) = first_difference(
                    v,
                    b.get(k).unwrap_or(&serde_json::Value::Null),
                    &format!("{path}/{k}"),
                ) {
                    return Some(result);
                }
            }
            Some(format!("{path}: object keys differ"))
        }
        (serde_json::Value::Array(a), serde_json::Value::Array(b)) => {
            for (i, (a, b)) in a.iter().zip(b).enumerate() {
                if let Some(result) = first_difference(a, b, &format!("{path}/{i}")) {
                    return Some(result);
                }
            }
            Some(format!("{path}: lengths {} != {}", a.len(), b.len()))
        }
        _ => Some(format!("{path}: {a} != {b}")),
    }
}
fn assert_same(a: &WorldState, b: &WorldState) {
    let a = spheres_sim::save(a);
    let b = spheres_sim::save(b);
    if a != b {
        panic!(
            "save divergence {}",
            first_difference(
                &serde_json::from_str(&a).unwrap(),
                &serde_json::from_str(&b).unwrap(),
                ""
            )
            .unwrap_or_else(|| "ordering".into())
        );
    }
}

#[test]
fn reviewed_booking_reserves_real_units_without_new_stock_or_immediate_public_payment() {
    let (mut w, district, company, product, revision) = ready("tank_standard");
    let q = companies::refit_quote(&w, HOME, company, SOURCE, product, 3);
    assert!(q.valid, "{:?}", q.reason);
    assert!(q.cost_bn > 0.0);
    assert!(q.minimum_days > 0);
    let company_cash = firm(&w, company).cash_bn;
    let public_cash = net_public(&w);
    let strength = arsenal::combat_value(w.nation(HOME), holding(&w, SOURCE).unwrap());
    let upkeep = equipment::fleet_maintenance_requirement(w.nation(HOME));
    let masses = age_mass(&w);
    let raw = w.resources.market.as_ref().unwrap().stocks.clone();
    let projects = w.nation(HOME).equipment.as_ref().unwrap().projects.clone();
    let mut expected = w.clone();
    programs::begin_day(&mut expected);
    let opening = programs::available_bn(&expected, HOME, D, 3);
    book(&mut w, company, product, 3);
    assert_eq!(units(&w, SOURCE), 4);
    assert_eq!(available(&w, SOURCE), 1);
    assert_eq!(units(&w, &revision), 0);
    assert_eq!(total_units(&w), 4);
    near(age_mass(&w), masses);
    near(firm(&w, company).cash_bn, company_cash);
    near(net_public(&w), public_cash);
    near(
        equipment::fleet_maintenance_requirement(w.nation(HOME)),
        upkeep,
    );
    assert!(arsenal::combat_value(w.nation(HOME), holding(&w, SOURCE).unwrap()) < strength);
    near(programs::available_bn(&w, HOME, D, 3), opening - q.cost_bn);
    assert_eq!(w.resources.market.as_ref().unwrap().stocks, raw);
    assert_eq!(
        w.nation(HOME).equipment.as_ref().unwrap().projects,
        projects
    );
    assert!(w.nation(HOME).arsenal.orders.is_empty());
    assert_eq!(firm(&w, company).products[0].stock, 0);
    assert_eq!(manufacturing::plant_slots(&w, &district), 1);
    load_exact(&w);
}

#[test]
fn invalid_and_replayed_bookings_cannot_overreserve_or_charge_a_second_time() {
    let (base, _, company, product, _) = ready("ground_ifv");
    for quantity in [0, 5, u32::MAX] {
        let mut w = base.clone();
        let saved = spheres_sim::save(&w);
        let q = companies::refit_quote(&w, HOME, company, SOURCE, product, quantity);
        assert!(!q.valid);
        assert!(spheres_sim::apply_command(
            &mut w,
            &Command::Company {
                nation: HOME,
                order: companies::CompanyOrder::Refit {
                    company,
                    source: SOURCE.into(),
                    product,
                    quantity,
                    quote: q.token,
                }
            }
        )
        .is_err());
        assert_eq!(spheres_sim::save(&w), saved);
    }
    let mut w = base;
    let q = companies::refit_quote(&w, HOME, company, SOURCE, product, 2);
    let order = companies::CompanyOrder::Refit {
        company,
        source: SOURCE.into(),
        product,
        quantity: 2,
        quote: q.token,
    };
    command(&mut w, order.clone());
    let saved = spheres_sim::save(&w);
    assert!(spheres_sim::apply_command(
        &mut w,
        &Command::Company {
            nation: HOME,
            order
        }
    )
    .is_err());
    assert_eq!(spheres_sim::save(&w), saved);
    assert!(!companies::refit_quote(&w, N::USSR, company, SOURCE, product, 1).valid);
    assert!(!companies::refit_quote(&w, HOME, company, SOURCE, u32::MAX, 1).valid);
    assert!(spheres_sim::apply_command(
        &mut w,
        &Command::Equipment {
            nation: HOME,
            order: EquipmentOrder::Retire {
                revision: SOURCE.into(),
                quantity: 3
            }
        }
    )
    .is_err());
    assert_eq!(available(&w, SOURCE), 2);
    load_exact(&w);
}

#[test]
fn service_advance_is_locked_until_return_and_fresh_or_prepaid_refund_posts_once() {
    let (base, _, company, product, _) = ready("tank_standard");
    for prepaid in [false, true] {
        for before_settlement in [false, true] {
            let mut w = base.clone();
            programs::begin_day(&mut w);
            let cost = companies::refit_quote(&w, HOME, company, SOURCE, product, 3).cost_bn;
            {
                let p = w.nation_mut(HOME).program_budget.as_mut().unwrap();
                p.available_bn[D][3] = if prepaid { 0.0 } else { cost };
                p.prepaid_bn[D][3] = if prepaid { cost } else { 0.0 };
            }
            let company_cash = firm(&w, company).cash_bn;
            let public_cash = net_public(&w);
            let refit = book(&mut w, company, product, 3);
            near(service(&w, company, refit).escrow_bn, 0.0);
            companies::settle_receivables(&mut w);
            near(service(&w, company, refit).escrow_bn, 0.0);
            near(firm(&w, company).cash_bn, company_cash);
            let plan = w.nation(HOME).program_budget.as_ref().unwrap();
            near(
                plan.prepaid_used_today_bn[D][3],
                if prepaid { cost } else { 0.0 },
            );
            near(plan.spent_today_bn[D][3], if prepaid { 0.0 } else { cost });
            near(programs::available_bn(&w, HOME, D, 3), 0.0);
            if before_settlement {
                near(cancel(&mut w, company, refit), cost);
                assert_eq!(available(&w, SOURCE), 4);
                near(service(&w, company, refit).refunded_bn, 0.0);
                near(net_public(&w), public_cash);
                load_exact(&w);
            }
            let fresh = close(&mut w);
            if !before_settlement {
                near(net_public(&w), public_cash - fresh);
                near(service(&w, company, refit).escrow_bn, cost);
                near(firm(&w, company).cash_bn, company_cash);
                near(firm(&w, company).refit_revenue_bn, 0.0);
                near(cancel(&mut w, company, refit), cost);
            }
            near(net_public(&w), public_cash - fresh + cost);
            near(service(&w, company, refit).escrow_bn, 0.0);
            near(service(&w, company, refit).refunded_bn, cost);
            near(firm(&w, company).cash_bn, company_cash);
            near(programs::available_bn(&w, HOME, D, 3), 0.0);
            assert_eq!(available(&w, SOURCE), 4);
            let saved = spheres_sim::save(&w);
            let q = companies::refit_cancel_quote(&w, HOME, company, refit);
            assert!(!q.valid);
            assert!(spheres_sim::apply_command(
                &mut w,
                &Command::Company {
                    nation: HOME,
                    order: companies::CompanyOrder::CancelRefit {
                        company,
                        refit,
                        quote: q.token
                    }
                }
            )
            .is_err());
            companies::settle_receivables(&mut w);
            assert_eq!(spheres_sim::save(&w), saved);
            reconcile(&w, company);
        }
    }
}

#[test]
fn every_supported_platform_returns_the_exact_revision_preserving_current_age_and_no_ammunition() {
    for platform in equipment::PLATFORMS {
        let (mut w, district, company, product, target) = ready(platform.id);
        let source_revision = w.nation(HOME).equipment.as_ref().unwrap().revisions[SOURCE].clone();
        let target_revision = w.nation(HOME).equipment.as_ref().unwrap().revisions[&target].clone();
        // An existing differently aged destination cohort must merge by mass.
        arsenal::deliver_design(w.nation_mut(HOME), &target, 2, 12.0).unwrap();
        let mut expected_mass = age_mass(&w);
        let original_ammo = w
            .nation(HOME)
            .equipment
            .as_ref()
            .unwrap()
            .ammunition
            .clone();
        let cost = companies::refit_quote(&w, HOME, company, SOURCE, product, 2).cost_bn;
        let refit = book(&mut w, company, product, 2);
        assert_eq!(available(&w, SOURCE), 2);
        let mut resumed = load_exact(&w);
        for _ in 0..200 {
            expected_mass += 6.0 * clock::month_fraction(&w);
            real_day(&mut w);
            real_day(&mut resumed);
            assert_eq!(total_units(&w), 6, "{}", platform.id);
            near(age_mass(&w), expected_mass);
            assert_eq!(units(&w, SOURCE) + units(&w, &target), 6);
            assert_eq!(
                service(&w, company, refit).completed_units,
                units(&w, &target) - 2
            );
            assert_eq!(firm(&w, company).products[0].stock, 0);
            assert_eq!(firm(&w, company).products[0].sold_units, 0);
            assert_eq!(manufacturing::plant_slots(&w, &district), 1);
            if service(&w, company, refit).completed_units == 2 {
                break;
            }
        }
        assert_eq!(
            service(&w, company, refit).completed_units,
            2,
            "{}",
            platform.id
        );
        assert_same(&w, &resumed);
        assert_eq!(units(&w, SOURCE), 2);
        assert_eq!(available(&w, SOURCE), 2);
        assert_eq!(units(&w, &target), 4);
        assert_eq!(
            w.nation(HOME).equipment.as_ref().unwrap().revisions[SOURCE],
            source_revision
        );
        assert_eq!(
            w.nation(HOME).equipment.as_ref().unwrap().revisions[&target],
            target_revision
        );
        assert_eq!(
            w.nation(HOME).equipment.as_ref().unwrap().ammunition,
            original_ammo
        );
        assert!(w.nation(HOME).arsenal.orders.is_empty());
        assert!(w.companies.deliveries.is_empty());
        assert!(w
            .nation(HOME)
            .equipment
            .as_ref()
            .unwrap()
            .company_refits
            .is_empty());
        near(service(&w, company, refit).earned_revenue_bn, cost);
        near(service(&w, company, refit).escrow_bn, 0.0);
        reconcile(&w, company);
    }
}

#[test]
fn public_escrow_cannot_start_work_until_private_capital_exists_and_partial_cancel_keeps_labor_reserved(
) {
    let (mut w, _, company, product, target) = ready_with_capital("tank_standard", 0.000001);
    let q = companies::refit_quote(&w, HOME, company, SOURCE, product, 3);
    assert!(q.valid, "{:?}", q.reason);
    assert_eq!(q.first_return_days, None);
    assert!(q.company_cash_needed_bn > firm(&w, company).cash_bn);
    let refit = book(&mut w, company, product, 3);
    close(&mut w);
    let raw = w.resources.market.as_ref().unwrap().stocks.clone();
    let escrow = service(&w, company, refit).escrow_bn;
    let cash = firm(&w, company).cash_bn;
    for _ in 0..5 {
        day(&mut w);
    }
    assert_eq!(service(&w, company, refit).unit_started_day, None);
    near(service(&w, company, refit).unit_work_days, 0.0);
    near(service(&w, company, refit).escrow_bn, escrow);
    near(firm(&w, company).cash_bn, cash);
    assert_eq!(w.resources.market.as_ref().unwrap().stocks, raw);
    let contribution = q.company_cash_needed_bn - cash + 0.000001;
    let capital = companies::capitalization_quote(&w, HOME, company, contribution);
    assert!(capital.valid);
    command(
        &mut w,
        companies::CompanyOrder::Capitalize {
            company,
            amount_bn: contribution,
            quote: capital.token,
        },
    );
    close(&mut w);
    for _ in 0..3 {
        day(&mut w);
        if service(&w, company, refit).unit_started_day.is_some() {
            break;
        }
    }
    let active = service(&w, company, refit).clone();
    assert!(active.unit_started_day.is_some());
    assert!(active.working_capital_locked_bn > 0.0);
    near(firm(&w, company).cash_bn, 0.000001);
    assert!(active.working_capital_locked_bn > firm(&w, company).cash_bn);
    let net = net_public(&w);
    let authority = programs::available_bn(&w, HOME, D, 3);
    near(cancel(&mut w, company, refit), active.unit_price_bn * 2.0);
    assert_eq!(available(&w, SOURCE), 3);
    assert_eq!(service(&w, company, refit).cancelled_units, 2);
    near(net_public(&w), net + active.unit_price_bn * 2.0);
    near(programs::available_bn(&w, HOME, D, 3), authority);
    near(
        service(&w, company, refit).working_capital_locked_bn,
        active.working_capital_locked_bn,
    );
    let mut resumed = load_exact(&w);
    for _ in 0..100 {
        day(&mut w);
        day(&mut resumed);
        if service(&w, company, refit).completed_units == 1 {
            break;
        }
    }
    assert_same(&w, &resumed);
    assert_eq!(service(&w, company, refit).completed_units, 1);
    assert_eq!(units(&w, SOURCE), 3);
    assert_eq!(units(&w, &target), 1);
    assert_eq!(available(&w, SOURCE), 3);
    near(service(&w, company, refit).working_capital_locked_bn, 0.0);
    near(service(&w, company, refit).escrow_bn, 0.0);
    near(firm(&w, company).cash_bn, 0.000001 + active.unit_price_bn);
    reconcile(&w, company);
}

#[test]
fn company_refit_reservations_net_source_and_target_goals_exactly_once() {
    let (mut w, _, company, product, target) = ready("air_light_attack");
    equipment_command(
        &mut w,
        EquipmentOrder::Target {
            revision: SOURCE.into(),
            quantity: Some(1),
        },
    );
    equipment_command(
        &mut w,
        EquipmentOrder::Target {
            revision: target.clone(),
            quantity: Some(3),
        },
    );
    let suggestions = equipment::fleet_target_refits_world(&w, HOME, &target);
    assert!(suggestions
        .iter()
        .any(|p| p.source_revision == SOURCE && p.quantity == 3));
    let refit = book(&mut w, company, product, 3);
    for _ in 0..160 {
        let plans = equipment::fleet_target_plans_world(&w, HOME);
        let source = plans.iter().find(|p| p.revision == SOURCE).unwrap();
        let dest = plans.iter().find(|p| p.revision == target).unwrap();
        let remaining = companies::refit_remaining(service(&w, company, refit)) as u64;
        assert_eq!(source.projected, 1);
        assert_eq!(source.refit_outgoing, remaining);
        assert_eq!(dest.projected, 3);
        assert_eq!(dest.refit_incoming, remaining);
        assert_eq!(dest.conditional_incoming, remaining);
        assert_eq!(dest.shortfall, 0);
        assert!(equipment::fleet_target_refits_world(&w, HOME, &target).is_empty());
        if remaining == 0 {
            break;
        }
        day(&mut w);
    }
    assert_eq!(service(&w, company, refit).completed_units, 3);
    reconcile(&w, company);
}

#[test]
fn paid_public_refits_and_supplier_claims_share_source_units_without_releasing_each_others_reservations(
) {
    let (mut w, district, company, product, target) = ready("ground_ifv");
    w.production
        .provinces
        .iter_mut()
        .find(|p| p.district == district)
        .unwrap()
        .arms_plants = 2;
    equipment_command(
        &mut w,
        EquipmentOrder::Refit {
            source: SOURCE.into(),
            target: target.clone(),
            district: district.clone(),
            quantity: 2,
            daily_budget_bn: 0.001,
        },
    );
    let public = w
        .nation(HOME)
        .equipment
        .as_ref()
        .unwrap()
        .projects
        .last()
        .unwrap()
        .id;
    day(&mut w);
    let refit = book(&mut w, company, product, 2);
    assert_eq!(available(&w, SOURCE), 0);
    day(&mut w);
    let project = w
        .nation(HOME)
        .equipment
        .as_ref()
        .unwrap()
        .projects
        .iter()
        .find(|p| p.id == public)
        .unwrap();
    assert!(project.spent_bn > 0.0);
    assert!(project.work_days > 0.0);
    assert_eq!(holding(&w, SOURCE).unwrap().refit_reserved, 4);
    load_exact(&w);
    // Capacity loss leaves the already paid public job with priority. Company
    // escrow and source reservations survive without creating a second slot.
    w.production
        .provinces
        .iter_mut()
        .find(|p| p.district == district)
        .unwrap()
        .arms_plants = 1;
    day(&mut w);
    assert_eq!(service(&w, company, refit).status, "blocked");
    near(service(&w, company, refit).unit_work_days, 0.0);
    assert_eq!(holding(&w, SOURCE).unwrap().refit_reserved, 4);
    near(
        cancel(&mut w, company, refit),
        service(&w, company, refit).total_price_bn,
    );
    assert_eq!(holding(&w, SOURCE).unwrap().refit_reserved, 2);
    assert_eq!(available(&w, SOURCE), 2);
    let before = w
        .nation(HOME)
        .equipment
        .as_ref()
        .unwrap()
        .projects
        .iter()
        .find(|p| p.id == public)
        .unwrap()
        .spent_bn;
    day(&mut w);
    assert!(
        w.nation(HOME)
            .equipment
            .as_ref()
            .unwrap()
            .projects
            .iter()
            .find(|p| p.id == public)
            .unwrap()
            .spent_bn
            > before
    );
    equipment_command(&mut w, EquipmentOrder::Cancel { project: public });
    assert_eq!(available(&w, SOURCE), 4);
    reconcile(&w, company);
}

#[test]
fn occupation_pauses_real_work_and_cancellation_releases_only_untouched_government_property() {
    let (mut w, district, company, product, target) = ready("air_tactical_strike");
    let refit = book(&mut w, company, product, 3);
    for _ in 0..3 {
        day(&mut w);
    }
    let progress = service(&w, company, refit).clone();
    assert!(progress.unit_work_days > 0.0);
    let cash = firm(&w, company).cash_bn;
    w.districts.insert(district.clone(), N::USSR);
    let mut resumed = load_exact(&w);
    for _ in 0..5 {
        day(&mut w);
        day(&mut resumed);
    }
    assert_same(&w, &resumed);
    assert_eq!(service(&w, company, refit).status, "blocked");
    near(
        service(&w, company, refit).unit_work_days,
        progress.unit_work_days,
    );
    near(service(&w, company, refit).escrow_bn, progress.escrow_bn);
    near(
        service(&w, company, refit).working_capital_locked_bn,
        progress.working_capital_locked_bn,
    );
    near(firm(&w, company).cash_bn, cash);
    assert_eq!(units(&w, SOURCE), 4);
    assert_eq!(units(&w, &target), 0);
    assert_eq!(
        w.nation(N::USSR)
            .arsenal
            .held
            .iter()
            .filter(|h| h.design_id.as_deref() == Some(SOURCE))
            .count(),
        0
    );
    assert!(!companies::refit_quote(&w, HOME, company, SOURCE, product, 1).valid);
    cancel(&mut w, company, refit);
    assert_eq!(available(&w, SOURCE), 3);
    load_exact(&w);
    w.districts.insert(district, HOME);
    for _ in 0..100 {
        day(&mut w);
        if service(&w, company, refit).completed_units == 1 {
            break;
        }
    }
    assert_eq!(service(&w, company, refit).completed_units, 1);
    assert_eq!(service(&w, company, refit).cancelled_units, 2);
    assert_eq!(units(&w, SOURCE), 3);
    assert_eq!(units(&w, &target), 1);
    reconcile(&w, company);
}

#[test]
fn the_same_factory_cannot_refit_make_ammunition_and_restock_vehicles_in_one_work_packet() {
    let (mut w, district, company, product, target) = ready("ground_apc");
    let refit = book(&mut w, company, product, 1);
    command(
        &mut w,
        companies::CompanyOrder::Inventory {
            company,
            product,
            stock_target: 1,
        },
    );
    let q = companies::ammo_supply_quote(&w, HOME, company, "mg_127", 10);
    assert!(q.valid, "{:?}", q.reason);
    command(
        &mut w,
        companies::CompanyOrder::AmmoSupply {
            company,
            family: "mg_127".into(),
            stock_target: 10,
            quote: q.token,
        },
    );
    for _ in 0..100 {
        day(&mut w);
        let c = firm(&w, company);
        assert_eq!(c.products[0].produced_units, 0);
        near(c.products[0].tooling_work_days, 0.0);
        assert_eq!(c.ammunition_products[0].produced_units, 0);
        assert_eq!(manufacturing::used_slots(&w, HOME, &district), 1);
        if service(&w, company, refit).completed_units == 1 {
            break;
        }
    }
    assert_eq!(units(&w, &target), 1);
    assert_eq!(firm(&w, company).products[0].sold_units, 0);
    day(&mut w);
    assert!(firm(&w, company).products[0].tooling_work_days > 0.0);
    assert_eq!(firm(&w, company).ammunition_products[0].produced_units, 0);
    reconcile(&w, company);
}

#[test]
fn old_supplier_books_remain_byte_exact_until_service_booking_and_native_claims_cannot_be_forged() {
    for (platform, version) in [("tank_standard", 1), ("ground_ifv", 2), ("ground_apc", 3)] {
        let (mut w, _, company, product, _) = ready(platform);
        if version == 3 {
            let q = companies::ammo_supply_quote(&w, HOME, company, "mg_127", 1);
            assert!(q.valid);
            command(
                &mut w,
                companies::CompanyOrder::AmmoSupply {
                    company,
                    family: "mg_127".into(),
                    stock_target: 1,
                    quote: q.token,
                },
            );
            let ammo_product = firm(&w, company).ammunition_products[0].id;
            command(
                &mut w,
                companies::CompanyOrder::AmmoInventory {
                    company,
                    product: ammo_product,
                    stock_target: 0,
                },
            );
        }
        let old = spheres_sim::save(&w);
        assert_eq!(w.companies.version, version);
        assert!(
            !old.contains("\"refits\"")
                && !old.contains("company_refits")
                && !old.contains("refit_revenue_bn")
        );
        let _ = companies::refit_quote(&w, HOME, company, SOURCE, product, 1);
        let _ = companies::view(&w, HOME);
        assert_eq!(spheres_sim::save(&w), old);
        let mut resumed = load_exact(&w);
        let refit = book(&mut w, company, product, 2);
        book(&mut resumed, company, product, 2);
        assert_same(&w, &resumed);
        let new = spheres_sim::save(&w);
        let json: serde_json::Value = serde_json::from_str(&new).unwrap();
        assert_eq!(json["version"], 5);
        assert_eq!(w.companies.version, 4);
        load_exact(&w);
        for downgraded in [1, 2, 3, 4] {
            let mut forged = json.clone();
            forged["version"] = downgraded.into();
            assert!(spheres_sim::load(&forged.to_string()).is_err());
        }
        let mut erased = w.clone();
        erased.companies = Default::default();
        assert!(spheres_sim::load(&spheres_sim::save(&erased)).is_err());
        let mut missing = w.clone();
        missing
            .nation_mut(HOME)
            .equipment
            .as_mut()
            .unwrap()
            .company_refits
            .clear();
        assert!(spheres_sim::load(&spheres_sim::save(&missing)).is_err());
        let mut wrong = w.clone();
        wrong
            .nation_mut(HOME)
            .equipment
            .as_mut()
            .unwrap()
            .company_refits
            .get_mut(&refit)
            .unwrap()
            .company += 1;
        assert!(spheres_sim::load(&spheres_sim::save(&wrong)).is_err());
        let mut funds = w.clone();
        funds
            .nation_mut(HOME)
            .program_budget
            .as_mut()
            .unwrap()
            .spent_today_bn[D][3] = 0.0;
        assert!(spheres_sim::load(&spheres_sim::save(&funds)).is_err());
        let mut duplicate = w.clone();
        let mut receipt = duplicate.companies.firms[0]
            .receivables
            .iter()
            .find(|r| r.kind == "refit_advance")
            .unwrap()
            .clone();
        receipt.id = duplicate.companies.next_id;
        duplicate.companies.next_id += 1;
        duplicate.companies.firms[0].receivables.push(receipt);
        assert!(spheres_sim::load(&spheres_sim::save(&duplicate)).is_err());
        close(&mut w);
        let mut cash = w.clone();
        let fee = service(&w, company, refit).total_price_bn;
        cash.companies.firms[0].cash_bn += fee;
        assert!(spheres_sim::load(&spheres_sim::save(&cash)).is_err());
        day(&mut w);
        day(&mut w);
        let mut free = w.clone();
        free.companies.firms[0].refits[0].working_capital_locked_bn = 0.0;
        assert!(spheres_sim::load(&spheres_sim::save(&free)).is_err());
        reconcile(&w, company);
    }
}

#[test]
fn scarce_inputs_pause_atomically_and_market_changes_cannot_reprice_the_active_unit_or_fixed_fee() {
    let (mut w, _, company, product, target) = ready("ground_ifv");
    let refit = book(&mut w, company, product, 2);
    close(&mut w);
    let recipe = service(&w, company, refit).recipe_per_unit;
    let scarce = resources::ALL
        .into_iter()
        .find(|c| recipe[c.idx()] > 0.0)
        .unwrap();
    let stock = w
        .resources
        .market
        .as_mut()
        .unwrap()
        .stocks
        .iter_mut()
        .find(|s| s.nation == HOME && s.commodity == scarce)
        .unwrap();
    let before_quantity = stock.quantity;
    stock.quantity = recipe[scarce.idx()] * 0.5;
    let before_raw = w.resources.market.as_ref().unwrap().stocks.clone();
    let cash = firm(&w, company).cash_bn;
    for _ in 0..3 {
        day(&mut w);
    }
    assert_eq!(service(&w, company, refit).unit_started_day, None);
    near(firm(&w, company).cash_bn, cash);
    assert_eq!(w.resources.market.as_ref().unwrap().stocks, before_raw);
    w.resources
        .market
        .as_mut()
        .unwrap()
        .stocks
        .iter_mut()
        .find(|s| s.nation == HOME && s.commodity == scarce)
        .unwrap()
        .quantity = before_quantity;
    day(&mut w);
    let first = service(&w, company, refit).clone();
    assert!(first.unit_started_day.is_some());
    assert_eq!(first.unit_inputs, recipe);
    assert!(first.unit_material_cost_bn > 0.0);
    let public = net_public(&w);
    let raw_cost = first.unit_material_cost_bn;
    for price in &mut w.resources.market.as_mut().unwrap().prices {
        *price *= 5.0;
    }
    let new_raw: f64 = resources::ALL
        .into_iter()
        .map(|c| recipe[c.idx()] * resources::market_current_price(&w, c) / 1e9)
        .sum();
    assert!(new_raw > raw_cost);
    let raw_owned = w.resources.market.as_ref().unwrap().stocks.clone();
    let mut resumed = load_exact(&w);
    while service(&w, company, refit).completed_units == 0 {
        day(&mut w);
        day(&mut resumed);
        assert_eq!(w.resources.market.as_ref().unwrap().stocks, raw_owned);
        near(service(&w, company, refit).materials_expense_bn, raw_cost);
        near(
            service(&w, company, refit).unit_price_bn,
            first.unit_price_bn,
        );
    }
    assert_same(&w, &resumed);
    near(
        service(&w, company, refit).fabrication_expense_bn,
        first.unit_fabrication_cost_bn,
    );
    assert_eq!(units(&w, &target), 1);
    assert!(
        net_public(&w) <= public + 1e-10,
        "there is no second warehouse payment for active inputs"
    );
    day(&mut w);
    near(service(&w, company, refit).unit_material_cost_bn, new_raw);
    near(
        service(&w, company, refit).materials_expense_bn,
        raw_cost + new_raw,
    );
    while service(&w, company, refit).completed_units < 2 {
        day(&mut w);
    }
    near(
        service(&w, company, refit).fabrication_expense_bn,
        first.unit_fabrication_cost_bn * 2.0,
    );
    near(
        service(&w, company, refit).earned_revenue_bn,
        first.total_price_bn,
    );
    reconcile(&w, company);
}

#[test]
fn prior_year_service_refund_returns_cash_without_recreating_expired_or_current_authority() {
    let (mut w, _, company, product, _) = ready("tank_standard");
    w.year += 1;
    w.month = 12;
    w.day = 31;
    renew(&mut w);
    programs::begin_day(&mut w);
    let refit = book(&mut w, company, product, 2);
    let fee = service(&w, company, refit).total_price_bn;
    close(&mut w);
    let paid_year = w.year;
    clock::advance_date(&mut w);
    renew(&mut w);
    programs::begin_day(&mut w);
    assert_eq!(w.year, paid_year + 1);
    let before = w.nation(HOME).program_budget.as_ref().unwrap().clone();
    let public = net_public(&w);
    near(cancel(&mut w, company, refit), fee);
    near(net_public(&w), public + fee);
    assert_eq!(w.nation(HOME).program_budget.as_ref().unwrap(), &before);
    assert_eq!(available(&w, SOURCE), 4);
    let saved = spheres_sim::save(&w);
    companies::settle_receivables(&mut w);
    assert_eq!(spheres_sim::save(&w), saved);
    reconcile(&w, company);
}

#[test]
#[ignore = "exports explicitly synthetic browser fixtures only when SPHERES_COMPANY_REFIT_QA_DIR is set"]
fn export_company_refit_qa_stages() {
    let Some(dir) = std::env::var_os("SPHERES_COMPANY_REFIT_QA_DIR") else {
        return;
    };
    let dir = std::path::PathBuf::from(dir);
    std::fs::create_dir_all(&dir).unwrap();
    for (prefix, platform) in [("ground", "ground_ifv"), ("aircraft", "air_light_attack")] {
        let (mut w, _, company, product, target) = ready_using(platform, 1.0, real_day);
        equipment_command(
            &mut w,
            EquipmentOrder::Target {
                revision: SOURCE.into(),
                quantity: Some(1),
            },
        );
        equipment_command(
            &mut w,
            EquipmentOrder::Target {
                revision: target.clone(),
                quantity: Some(3),
            },
        );
        let write = |stage: &str, w: &WorldState| {
            load_exact(w);
            std::fs::write(
                dir.join(format!("{prefix}-{stage}.json")),
                spheres_sim::save(w),
            )
            .unwrap();
        };
        write("01-ready", &w);
        let refit = book(&mut w, company, product, 3);
        write("02-awaiting-settlement", &w);
        real_day(&mut w);
        assert!(service(&w, company, refit).settled_day.is_some());
        assert_eq!(service(&w, company, refit).unit_work_days, 0.0);
        write("03-escrow-settled", &w);
        for _ in 0..6 {
            real_day(&mut w);
        }
        assert!(service(&w, company, refit).unit_work_days > 0.0);
        write("04-active-work", &w);
        let mut cancelled = w.clone();
        cancel(&mut cancelled, company, refit);
        assert_eq!(service(&cancelled, company, refit).cancelled_units, 2);
        write("05-partial-cancel", &cancelled);
        for _ in 0..100 {
            real_day(&mut cancelled);
            if service(&cancelled, company, refit).completed_units == 1 {
                break;
            }
        }
        assert_eq!(service(&cancelled, company, refit).completed_units, 1);
        assert_eq!(units(&cancelled, SOURCE), 3);
        assert_eq!(units(&cancelled, &target), 1);
        write("06-cancelled-remainder-returned", &cancelled);
        for _ in 0..180 {
            real_day(&mut w);
            if service(&w, company, refit).completed_units == 3 {
                break;
            }
        }
        assert_eq!(service(&w, company, refit).completed_units, 3);
        assert_eq!(units(&w, SOURCE), 1);
        assert_eq!(units(&w, &target), 3);
        write("07-complete", &w);
        reconcile(&w, company);
    }
    std::fs::write(dir.join("README.txt"),"SYNTHETIC QA ONLY. Each fixture starts with explicitly endowed French opening cash, raw materials, one existing test Arms Plant, learned components and four certified source vehicles aged 120 months. Company formation, target development/certification, service reservation, escrow settlement, all physical conversion work and cancellation use real commands and the full daily tick. No completed work, target stock or returned vehicles were fabricated. Company 1 owns target product 3; source revision is synthetic-source. Ready: source4, target0, goals source1/target3. Contract: quantity3. Partial cancellation refunds/releases2 untouched units and finishes the one started unit. Complete: source1,target3. No company vehicle stock, new ammunition, public fake orders or shipping. Use copies; these are not live player saves.\n").unwrap();
}
