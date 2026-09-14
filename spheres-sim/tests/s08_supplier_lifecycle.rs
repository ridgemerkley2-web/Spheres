//! S08 integration: synthetic opening company capital, Arms Plant, sovereign raw
//! inputs and buyer appropriation are disclosed fixture endowments. Every model
//! thereafter is actually developed, tooled, fabricated, sold and delivered by
//! the ordinary paid owners. Fresh-world supply qualification is separate.
use spheres_sim::world::{GameRules, NationId as N, WorldState, BUDGET_DEFENSE as D};
use spheres_sim::{arsenal, clock, companies, equipment, production, programs, resources, Command};
const HOME: N = N::France;
const BUYER: N = N::Tonga;
fn near(a: f64, b: f64) {
    assert!(
        (a - b).abs() <= 1e-12 + 1e-10 * a.abs().max(b.abs()),
        "{a} != {b}"
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

fn develop_model(
    w: &mut WorldState,
    company: u32,
    platform: &str,
    name: &str,
    budget: f64,
    target: u32,
) -> u32 {
    // Authored seller-only research for the added S16 platform. The buying
    // country still receives only the paid finished model, with no research grant.
    if platform == "air_fighter" {
        w.nation_mut(HOME).equipment.get_or_insert_with(Default::default).learned
            .extend(["air_propulsion_integration", "air_mission_systems", "air_fighter_integration"].map(str::to_string));
    }
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
fn command_as(w: &mut WorldState, n: N, order: companies::CompanyOrder) {
    spheres_sim::apply_command(w, &Command::Company { nation: n, order }).unwrap();
}
fn buyer(w: &mut WorldState) {
    w.player = Some(BUYER);
    w.sanctions.clear();
    w.nation_mut(BUYER).political_capital = 1000.0;
    programs::set_construction_budget(w, BUYER, 0.0).unwrap();
    let n = w.nation_mut(BUYER);
    n.treasury_bn = Some(100.0);
    n.debt_bn = Some(0.0);
    n.debt_gdp = 0.0;
    n.arsenal.held.clear();
    n.arsenal.orders.clear();
    n.arsenal.banked = 0.0;
    let p = n.program_budget.as_mut().unwrap();
    p.available_bn[D][3] = 10.0;
    p.available_bn[D][2] = 10.0;
    p.prepaid_bn[D][3] = 0.0;
    let q = companies::import_enrollment_quote(w, BUYER);
    assert!(q.valid, "{:?}", q.reason);
    command_as(
        w,
        BUYER,
        companies::CompanyOrder::EnableImports { quote: q.token },
    );
    assert!(!w.companies.firms.iter().any(|c| c.nation == BUYER));
    assert!(w
        .production
        .provinces
        .iter()
        .filter(|p| w.districts.get(&p.district) == Some(&BUYER))
        .all(|p| p.arms_plants == 0));
}
fn stocked(platform: &str) -> (WorldState, u32, u32) {
    let (mut w, d) = fixture();
    let c = establish(&mut w, &d, 1.0);
    isolated_day(&mut w);
    let p = develop_model(&mut w, c, platform, "S08 modeled export", 1.0, 2);
    until_stock(&mut w, c, p, 2);
    apply(
        &mut w,
        companies::CompanyOrder::Inventory {
            company: c,
            product: p,
            stock_target: 0,
        },
    );
    buyer(&mut w);
    (w, c, p)
}
fn end_day(w: &mut WorldState) {
    for n in [HOME, BUYER] {
        if let Some(p) = w.nation_mut(n).program_budget.as_mut() {
            p.revenue_today_bn = 0.0;
            p.interest_today_bn = 0.0;
            p.fiscal_staged = true;
        }
    }
    programs::finish_day(w);
    companies::settle_receivables(w);
    clock::advance_date(w);
}
fn advance(w: &mut WorldState) {
    programs::begin_day(w);
    equipment::tick_day(w);
    companies::tick_day(w);
    end_day(w);
}
fn imported_units(w: &WorldState, id: &str) -> u32 {
    w.nation(BUYER)
        .arsenal
        .held
        .iter()
        .filter(|h| h.design_id.as_deref() == Some(id))
        .map(arsenal::available_design_units)
        .sum()
}
fn book(w: &mut WorldState, c: u32, p: u32, ammo: bool, quantity: u32) -> u32 {
    let q = companies::import_purchase_quote(w, BUYER, HOME, c, p, ammo, quantity);
    assert!(q.valid, "{:?}", q.reason);
    command_as(
        w,
        BUYER,
        companies::CompanyOrder::ImportPurchase {
            seller: HOME,
            company: c,
            product: p,
            ammunition: ammo,
            quantity,
            quote: q.token,
        },
    );
    w.companies.imports.contracts.last().unwrap().id
}
fn restored(w: &WorldState) -> WorldState {
    let text = spheres_sim::save(w);
    let loaded = spheres_sim::load(&text).unwrap();
    assert_eq!(spheres_sim::save(&loaded), text);
    loaded
}
#[test]
fn s08_all_paid_supplier_platforms_import_exact_models_without_factory_research_or_duplicate_payment(
) {
    assert_eq!(equipment::PLATFORMS.len(), 12);
    for platform in equipment::PLATFORMS {
        let (mut w, c, p) = stocked(platform.id);
        let source = product(&w, c, p).revision_id.clone();
        let frozen = w.nation(HOME).equipment.as_ref().unwrap().revisions[&source].clone();
        let research = w.nation(BUYER).equipment.as_ref().unwrap().learned.clone();
        let company_cash = firm(&w, c).cash_bn;
        let before = spheres_sim::save(&w);
        let offers = companies::import_offers(&w, BUYER);
        assert!(offers
            .iter()
            .any(|o| o.company == c && o.product == p && o.allowed && o.affordable_quantity == 2));
        assert_eq!(spheres_sim::save(&w), before);
        let q = companies::import_purchase_quote(&w, BUYER, HOME, c, p, false, 1);
        let authority = programs::available_bn(&w, BUYER, D, 3);
        let buyer_cash = w.nation(BUYER).treasury_bn;
        let order = companies::CompanyOrder::ImportPurchase {
            seller: HOME,
            company: c,
            product: p,
            ammunition: false,
            quantity: 1,
            quote: q.token,
        };
        command_as(&mut w, BUYER, order.clone());
        assert_eq!(product(&w, c, p).stock, 1);
        near(
            programs::available_bn(&w, BUYER, D, 3),
            authority - q.cost_bn,
        );
        assert_eq!(w.nation(BUYER).treasury_bn, buyer_cash);
        near(firm(&w, c).cash_bn, company_cash);
        let after = spheres_sim::save(&w);
        assert!(spheres_sim::apply_command(
            &mut w,
            &Command::Company {
                nation: BUYER,
                order
            }
        )
        .is_err());
        assert_eq!(spheres_sim::save(&w), after);
        let id = w.companies.imports.contracts[0]
            .buyer_revision
            .clone()
            .unwrap();
        assert_eq!(imported_units(&w, &id), 0);
        assert!(!w
            .nation(BUYER)
            .equipment
            .as_ref()
            .unwrap()
            .revisions
            .contains_key(&id));
        let mut loaded = restored(&w);
        end_day(&mut w);
        end_day(&mut loaded);
        near(w.companies.imports.contracts[0].escrow_bn, q.cost_bn);
        near(firm(&w, c).cash_bn, company_cash);
        let due = w.companies.imports.contracts[0].due_day.unwrap();
        while clock::absolute_day(&w) <= due {
            advance(&mut w);
            advance(&mut loaded);
        }
        assert_eq!(spheres_sim::save(&w), spheres_sim::save(&loaded));
        assert_eq!(imported_units(&w, &id), 1);
        assert_eq!(product(&w, c, p).stock, 1);
        near(firm(&w, c).cash_bn, company_cash + q.cost_bn);
        near(w.companies.imports.contracts[0].escrow_bn, 0.0);
        let received = &w.nation(BUYER).equipment.as_ref().unwrap().revisions[&id];
        assert_eq!(received.profile, frozen.profile);
        assert_eq!(received.spec, frozen.spec);
        assert_eq!(
            w.nation(BUYER).equipment.as_ref().unwrap().learned,
            research
        );
        assert!(w
            .nation(BUYER)
            .equipment
            .as_ref()
            .unwrap()
            .ammunition
            .as_ref()
            .is_none_or(|a| a.stocks.is_empty()));
        let no_license = equipment::production_quote(&w, BUYER, &id, "TO-01", 1, 1.0);
        assert!(no_license
            .reason
            .as_deref()
            .unwrap()
            .contains("manufacturing license"));
        spheres_sim::apply_command(
            &mut w,
            &Command::Equipment {
                nation: BUYER,
                order: spheres_sim::EquipmentOrder::Maintenance {
                    daily_budget_bn: 0.01,
                },
            },
        )
        .unwrap();
        advance(&mut w);
        advance(&mut w);
        assert!(equipment::fleet_maintenance_requirement(w.nation(BUYER)) > 0.0);
        restored(&w);
    }
}
#[test]
fn s08_import_sanctions_access_loss_and_stale_reviews_preserve_owned_stock_and_resume_once() {
    let (mut w, c, p) = stocked("ground_apc");
    let q = companies::import_purchase_quote(&w, BUYER, HOME, c, p, false, 1);
    assert!(q.valid, "{:?}", q.reason);
    w.sanctions.push((HOME, BUYER));
    let saved = spheres_sim::save(&w);
    assert!(!companies::import_purchase_quote(&w, BUYER, HOME, c, p, false, 1).valid);
    assert!(spheres_sim::apply_command(
        &mut w,
        &Command::Company {
            nation: BUYER,
            order: companies::CompanyOrder::ImportPurchase {
                seller: HOME,
                company: c,
                product: p,
                ammunition: false,
                quantity: 1,
                quote: q.token
            }
        }
    )
    .is_err());
    assert_eq!(spheres_sim::save(&w), saved);
    w.sanctions.clear();
    assert!(!companies::import_purchase_quote(&w, BUYER, HOME, c, p, false, 3).valid);
    book(&mut w, c, p, false, 1);
    end_day(&mut w);
    let old_due = w.companies.imports.contracts[0].due_day.unwrap();
    let public_before = w.nation(BUYER).treasury_bn;
    w.sanctions.push((BUYER, HOME));
    for _ in 0..3 {
        advance(&mut w);
    }
    assert_eq!(w.companies.imports.contracts[0].due_day, Some(old_due + 3));
    assert!(w.companies.imports.contracts[0]
        .reason
        .contains("sanctions"));
    assert_eq!(product(&w, c, p).stock, 1);
    assert_eq!(w.companies.imports.contracts[0].delivered_day, None);
    let mut loaded = restored(&w);
    w.sanctions.clear();
    loaded.sanctions.clear();
    let district = firm(&w, c).district.clone();
    w.districts.insert(district.clone(), N::Belgium);
    loaded.districts.insert(district.clone(), N::Belgium);
    advance(&mut w);
    advance(&mut loaded);
    assert!(w.companies.imports.contracts[0]
        .reason
        .contains("shipping province"));
    assert_eq!(w.companies.imports.contracts[0].due_day, Some(old_due + 4));
    w.districts.insert(district.clone(), HOME);
    loaded.districts.insert(district, HOME);
    let due = w.companies.imports.contracts[0].due_day.unwrap();
    while clock::absolute_day(&w) <= due {
        advance(&mut w);
        advance(&mut loaded);
    }
    assert_eq!(spheres_sim::save(&w), spheres_sim::save(&loaded));
    assert_eq!(imported_units(&w, &format!("import-{c}-{p}")), 1);
    assert!(w.nation(BUYER).treasury_bn <= public_before);
    let mut corrupt = serde_json::to_value(&w).unwrap();
    corrupt["companies"]["imports"]["contracts"][0]["escrow_bn"] = serde_json::json!(0.1);
    let mut envelope: serde_json::Value = serde_json::from_str(&spheres_sim::save(&w)).unwrap();
    envelope["world"] = corrupt;
    assert!(spheres_sim::load(&envelope.to_string()).is_err());
}
#[test]
fn s08_import_cancellation_before_and_after_settlement_returns_exact_property_and_refund_once() {
    for (settled, borrowing) in [(false, false), (true, false), (false, true), (true, true)] {
        let (mut w, c, p) = stocked("tank_light");
        if borrowing {
            let n = w.nation_mut(BUYER);
            n.treasury_bn = Some(0.0);
            n.debt_bn = Some(0.05);
            n.debt_gdp = 0.05 / n.gdp;
        }
        let stock = product(&w, c, p).stock;
        let basis = product(&w, c, p).stock_cost_bn;
        let company_cash = firm(&w, c).cash_bn;
        let id = book(&mut w, c, p, false, 1);
        let cost = w.companies.imports.contracts[0].total_price_bn;
        if settled {
            end_day(&mut w);
        }
        let q = companies::import_cancel_quote(&w, BUYER, id);
        assert!(q.valid, "{:?}", q.reason);
        assert_eq!(q.refund_after_settlement, !settled);
        let cash = w.nation(BUYER).treasury_bn.unwrap();
        let debt = w.nation(BUYER).debt_bn.unwrap();
        let authority = programs::available_bn(&w, BUYER, D, 3);
        let order = companies::CompanyOrder::CancelImport {
            contract: id,
            quote: q.token,
        };
        command_as(&mut w, BUYER, order.clone());
        assert_eq!(product(&w, c, p).stock, stock);
        near(product(&w, c, p).stock_cost_bn, basis);
        near(firm(&w, c).cash_bn, company_cash);
        near(programs::available_bn(&w, BUYER, D, 3), authority);
        if settled {
            if borrowing {
                near(w.nation(BUYER).debt_bn.unwrap(), debt - cost);
                near(w.nation(BUYER).treasury_bn.unwrap(), 0.0);
            } else {
                near(w.nation(BUYER).treasury_bn.unwrap(), cash + cost);
            }
        } else {
            assert_eq!(w.nation(BUYER).treasury_bn, Some(cash));
            assert_eq!(w.companies.imports.contracts[0].refunded_day, None);
        }
        let saved = spheres_sim::save(&w);
        assert!(spheres_sim::apply_command(
            &mut w,
            &Command::Company {
                nation: BUYER,
                order
            }
        )
        .is_err());
        assert_eq!(spheres_sim::save(&w), saved);
        let mut loaded = restored(&w);
        if !settled {
            end_day(&mut w);
            end_day(&mut loaded);
        }
        near(w.companies.imports.contracts[0].refunded_bn, cost);
        assert_eq!(w.companies.imports.contracts[0].escrow_bn, 0.0);
        for _ in 0..25 {
            advance(&mut w);
            advance(&mut loaded);
        }
        assert_eq!(spheres_sim::save(&w), spheres_sim::save(&loaded));
        assert_eq!(product(&w, c, p).stock, stock);
        assert!(w.companies.imports.contracts[0].delivered_day.is_none());
        assert_eq!(imported_units(&w, &format!("import-{c}-{p}")), 0);
        restored(&w);
    }
}

// Representative foreign ammunition branches: basic ground rounds and guided
// aircraft stores. The existing domestic suite independently covers all23 recipes.
#[test]
fn s08_imported_rounds_and_guided_air_stores_keep_one_paid_receipt_without_research_grants() {
    for guided in [false, true] {
        let (mut w, d) = fixture();
        let c = establish(&mut w, &d, 1.0);
        isolated_day(&mut w);
        let mut spec = equipment::default_spec(if guided {
            "air_light_attack"
        } else {
            "ground_apc"
        });
        if guided {
            // Explicit seller-only research endowment to exercise a locked foreign component.
            w.nation_mut(HOME).equipment.as_mut().unwrap().learned =
                equipment::RESEARCH.iter().map(|r| r.id.into()).collect();
            spec.components
                .insert("air_payload".into(), "air_payload_guided".into());
            spec.components
                .insert("air_avionics".into(), "air_avionics_digital".into());
        }
        let family = equipment::ammunition_family(&spec).unwrap().to_string();
        assert_eq!(family, if guided { "air_bomb_guided" } else { "mg_127" });
        let q =
            companies::development_quote(&w, HOME, c, "S08 ammunition interface", &spec, 1.0, 1);
        assert!(q.valid, "{:?}", q.reason);
        apply(
            &mut w,
            companies::CompanyOrder::Develop {
                company: c,
                name: "S08 ammunition interface".into(),
                spec,
                daily_budget_bn: 1.0,
                stock_target: 1,
                quote: q.token,
            },
        );
        let p = firm(&w, c).products.last().unwrap().id;
        until_stock(&mut w, c, p, 1);
        apply(
            &mut w,
            companies::CompanyOrder::Inventory {
                company: c,
                product: p,
                stock_target: 0,
            },
        );
        spheres_sim::apply_command(
            &mut w,
            &Command::Equipment {
                nation: HOME,
                order: spheres_sim::EquipmentOrder::Maintenance {
                    daily_budget_bn: 0.01,
                },
            },
        )
        .unwrap();
        let q = companies::ammo_supply_quote(&w, HOME, c, &family, 12);
        assert!(q.valid, "{:?}", q.reason);
        apply(
            &mut w,
            companies::CompanyOrder::AmmoSupply {
                company: c,
                family: family.clone(),
                stock_target: 12,
                quote: q.token,
            },
        );
        let ammo = firm(&w, c).ammunition_products.last().unwrap().id;
        for _ in 0..10 {
            if firm(&w, c).ammunition_products[0].stock == 12 {
                break;
            }
            isolated_day(&mut w);
        }
        assert_eq!(firm(&w, c).ammunition_products[0].stock, 12);
        apply(
            &mut w,
            companies::CompanyOrder::AmmoInventory {
                company: c,
                product: ammo,
                stock_target: 0,
            },
        );
        buyer(&mut w);
        let learned = w.nation(BUYER).equipment.as_ref().unwrap().learned.clone();
        assert!(
            !companies::import_purchase_quote(&w, BUYER, HOME, c, ammo, true, 3).valid,
            "A compatible owned model and maintenance plan are prerequisites"
        );
        book(&mut w, c, p, false, 1);
        end_day(&mut w);
        let due = w.companies.imports.contracts[0].due_day.unwrap();
        while clock::absolute_day(&w) <= due {
            advance(&mut w);
        }
        spheres_sim::apply_command(
            &mut w,
            &Command::Equipment {
                nation: BUYER,
                order: spheres_sim::EquipmentOrder::Maintenance {
                    daily_budget_bn: 0.001,
                },
            },
        )
        .unwrap();
        advance(&mut w);
        advance(&mut w);
        let q = companies::import_purchase_quote(&w, BUYER, HOME, c, ammo, true, 3);
        assert!(q.valid, "{:?}", q.reason);
        assert!(q.protected_maintenance_bn >= 0.0);
        let cash = firm(&w, c).cash_bn;
        book(&mut w, c, ammo, true, 3);
        assert_eq!(firm(&w, c).ammunition_products[0].stock, 9);
        assert_eq!(companies::ammo_inbound_units(&w, BUYER, &family), 3);
        let mut loaded = restored(&w);
        end_day(&mut w);
        end_day(&mut loaded);
        let due = w
            .companies
            .imports
            .contracts
            .last()
            .unwrap()
            .due_day
            .unwrap();
        while clock::absolute_day(&w) <= due {
            advance(&mut w);
            advance(&mut loaded);
        }
        assert_eq!(spheres_sim::save(&w), spheres_sim::save(&loaded));
        let stores = w
            .nation(BUYER)
            .equipment
            .as_ref()
            .unwrap()
            .ammunition
            .as_ref()
            .unwrap();
        assert_eq!(stores.stocks[&family], 3.0);
        assert_eq!(stores.supplier_receipts.len(), 1);
        assert!(stores.active_from_day.is_none());
        assert!(stores.orders.is_empty());
        assert_eq!(companies::ammo_inbound_units(&w, BUYER, &family), 0);
        near(firm(&w, c).cash_bn, cash + q.cost_bn);
        assert_eq!(w.nation(BUYER).equipment.as_ref().unwrap().learned, learned);
        restored(&w);
    }
}
#[test]
fn s08_catalogue_only_adoption_is_versioned_without_assets_and_downgrades_are_refused() {
    let (mut w, _) = fixture();
    let before = spheres_sim::save(&w);
    assert!(!before.contains("supplier_catalogue"));
    assert!(w.companies.is_empty());
    let cash = w
        .nations
        .iter()
        .map(|n| {
            (
                n.id,
                n.treasury_bn,
                n.debt_bn,
                serde_json::to_value(&n.arsenal).unwrap(),
                n.equipment.clone(),
            )
        })
        .collect::<Vec<_>>();
    let production = w.production.clone();
    spheres_sim::supplier_catalogue::enable(&mut w).unwrap();
    assert!(w.companies.is_empty());
    assert_eq!(w.production, production);
    assert_eq!(
        w.nations
            .iter()
            .map(|n| (
                n.id,
                n.treasury_bn,
                n.debt_bn,
                serde_json::to_value(&n.arsenal).unwrap(),
                n.equipment.clone()
            ))
            .collect::<Vec<_>>(),
        cash
    );
    let text = spheres_sim::save(&w);
    let envelope: serde_json::Value = serde_json::from_str(&text).unwrap();
    assert_eq!(envelope["format"], "spheres-equipment-save");
    assert_eq!(envelope["version"], 6);
    restored(&w);
    for old in [0, 1, 2, 3, 4, 5] {
        let mut invalid = envelope.clone();
        invalid["version"] = serde_json::json!(old);
        assert!(
            spheres_sim::load(&invalid.to_string()).is_err(),
            "downgrade{old}"
        );
    }
    assert!(spheres_sim::load(&envelope["world"].to_string()).is_err());
}
#[test]
fn s08_serialized_import_origin_route_and_escrow_corruption_is_rejected() {
    let (mut w, c, p) = stocked("ground_recon");
    book(&mut w, c, p, false, 1);
    end_day(&mut w);
    let good: serde_json::Value = serde_json::from_str(&spheres_sim::save(&w)).unwrap();
    for case in 0..10 {
        let mut invalid = good.clone();
        let d = &mut invalid["world"]["companies"]["imports"]["contracts"][0];
        match case {
            0 => d["source_revision"]["profile"]["unit_cost_bn"] = serde_json::json!(77.0),
            1 => d["buyer_revision"] = serde_json::json!("stolen-local-model"),
            2 => d["route"]["estimated_days"] = serde_json::json!(1),
            3 => d["route"]["segments"][0] = serde_json::json!("invented-route"),
            4 => d["route"]["nodes"][0]["id"] = serde_json::json!("TO-01"),
            5 => d["route"]["nodes"][0]["lon"] = serde_json::json!(720.0),
            6 => d["escrow_bn"] = serde_json::json!(0.0),
            7 => d["refund_grant"] = serde_json::json!(123),
            8 => d["settled_day"] = serde_json::json!(d["purchased_day"].as_i64().unwrap() + 1),
            _ => d["quantity"] = serde_json::json!(0),
        }
        assert!(
            spheres_sim::load(&invalid.to_string()).is_err(),
            "invalid import case{case}"
        );
    }
    restored(&w);
}
#[test]
fn s08_missing_or_reidentified_supplier_retains_paid_import_before_any_buyer_mutation() {
    for reidentified in [false, true] {
        let (mut w, c, p) = stocked("ground_apc");
        book(&mut w, c, p, false, 1);
        end_day(&mut w);
        let due = w.companies.imports.contracts[0].due_day.unwrap();
        while clock::absolute_day(&w) < due {
            advance(&mut w);
        }
        let revision = format!("import-{c}-{p}");
        assert_eq!(imported_units(&w, &revision), 0);
        let contract = w.companies.imports.contracts[0].clone();
        assert!(contract.escrow_bn > 0.0);
        let buyer_before = serde_json::to_value(w.nation(BUYER)).unwrap();
        // Deliberately break the live source identity after real payment. Such
        // a save is still invalid; the dated owner must independently fail
        // closed before touching the buyer, whether the firm vanished or its
        // ID now belongs to another government's company.
        if reidentified {
            w.companies
                .firms
                .iter_mut()
                .find(|f| f.id == c)
                .unwrap()
                .nation = N::Germany;
        } else {
            w.companies.firms.retain(|f| f.id != c);
        }
        companies::tick_day(&mut w);
        assert_eq!(serde_json::to_value(w.nation(BUYER)).unwrap(), buyer_before);
        let retained = &w.companies.imports.contracts[0];
        assert_eq!(retained.status, "blocked");
        assert!(retained.reason.contains("original supplier identity"));
        assert_eq!(retained.escrow_bn, contract.escrow_bn);
        assert_eq!(retained.refunded_bn, 0.0);
        assert_eq!(retained.refunded_day, None);
        assert_eq!(retained.delivered_day, None);
        assert_eq!(retained.cancelled_day, None);
        assert_eq!(retained.due_day, Some(due + 1));
        assert_eq!(companies::inbound_units(&w, BUYER, &revision), 1);
        let after = spheres_sim::save(&w);
        companies::tick_day(&mut w);
        assert_eq!(spheres_sim::save(&w), after, "one delay per dated tick");
        assert!(
            spheres_sim::load(&after).is_err(),
            "the save validator must still reject a missing source"
        );
    }
}

#[test]
fn s08_market_affordability_matches_review_at_sub_ulp_funding_boundaries() {
    let mut witnessed_division_rounding = false;
    for platform in equipment::PLATFORMS {
        let (mut w, c, p) = stocked(platform.id);
        w.player = Some(HOME);
        apply(
            &mut w,
            companies::CompanyOrder::Inventory {
                company: c,
                product: p,
                stock_target: 12,
            },
        );
        until_stock(&mut w, c, p, 12);
        w.player = Some(BUYER);
        programs::set_construction_budget(&mut w, BUYER, 0.0).unwrap();
        programs::begin_day(&mut w);
        let quote = companies::import_purchase_quote(&w, BUYER, HOME, c, p, false, 1);
        let price = quote.unit_price_bn;
        assert!(price > 0.0);
        for quantity in 1..=12 {
            let cost = price * quantity as f64;
            let boundary = f64::from_bits(cost.to_bits() - 1);
            let plan = w.nation_mut(BUYER).program_budget.as_mut().unwrap();
            plan.available_bn[D][3] = boundary;
            plan.prepaid_bn[D][3] = 0.0;
            let before = spheres_sim::save(&w);
            let offer = companies::import_offers(&w, BUYER)
                .into_iter()
                .find(|o| o.company == c && o.product == p)
                .unwrap();
            assert_eq!(offer.purchase_available_bn, boundary);
            assert!(price * offer.affordable_quantity as f64 <= boundary);
            assert!(
                !companies::import_purchase_quote(&w, BUYER, HOME, c, p, false, quantity).valid
            );
            if offer.affordable_quantity > 0 {
                let review = companies::import_purchase_quote(
                    &w,
                    BUYER,
                    HOME,
                    c,
                    p,
                    false,
                    offer.affordable_quantity,
                );
                assert!(review.valid, "{}: {:?}", quantity, review.reason);
            }
            witnessed_division_rounding |= (boundary / price).floor() >= quantity as f64;
            assert_eq!(spheres_sim::save(&w), before, "offers and reviews are pure");
        }
    }
    assert!(
        witnessed_division_rounding,
        "fixture must exercise division rounding upward past a payable whole lot"
    );
}

#[test]
fn s08_new_physical_supplier_preserves_preproduction_expenses_through_each_saved_phase() {
    let (mut w, district) = fixture();
    // Explicit utility/component endowments for this bounded accounting fixture.
    // Enrollment is before company formation and development, so no paid
    // programme or work packet can take the grandfathered path.
    w.production
        .provinces
        .iter_mut()
        .find(|p| p.district == district)
        .unwrap()
        .power_grid = 10;
    w.production
        .industry
        .sites
        .insert(district.clone(), [0, 1, 0, 0, 0, 0, 0]);
    spheres_sim::connected_economy::enable(&mut w).unwrap();
    spheres_sim::company_network::enable(&mut w).unwrap();
    w.production
        .operations
        .advanced_components
        .insert(HOME, 100.0);
    let c = establish(&mut w, &district, 1.0);
    isolated_day(&mut w);
    let p = develop_model(
        &mut w,
        c,
        "ground_apc",
        "S08 actual phased operating costs",
        1.0,
        1,
    );
    assert!(!w.supplier_operations.grandfathered_programs.contains(&p));
    let mut seen = std::collections::BTreeSet::new();
    let mut running_preproduction = 0.0;
    for _ in 0..1000 {
        let before_materials = firm(&w, c).materials_expense_bn;
        let before_product = product(&w, c, p).clone();
        let profile = equipment::profile(w.nation(HOME), &before_product.revision_id).unwrap();
        let before_phase = if before_product.certified_day.is_none() {
            "development"
        } else if before_product.tooling_work_days + 1e-9 < profile.tooling_days as f64 {
            "tooling"
        } else {
            "manufacturing"
        };
        isolated_day(&mut w);
        let input_paid = firm(&w, c).materials_expense_bn - before_materials;
        if before_phase != "manufacturing" {
            running_preproduction += input_paid;
        }
        let Some(term) = w.supplier_operations.contracts.get(&p) else {
            continue;
        };
        near(term.preproduction_inputs_bn, running_preproduction);
        near(
            firm(&w, c).materials_expense_bn + firm(&w, c).fabrication_expense_bn,
            term.preproduction_inputs_bn
                + product(&w, c, p).stock_cost_bn
                + product(&w, c, p).unit_spent_bn,
        );
        if input_paid > 0.0 && seen.insert(before_phase) {
            assert!(term.preproduction_inputs_bn > 0.0);
            let saved = spheres_sim::save(&w);
            let mut resumed = spheres_sim::load(&saved).expect("actual supplier phase must reload");
            assert_eq!(spheres_sim::save(&resumed), saved);
            let mut continued = w.clone();
            isolated_day(&mut continued);
            isolated_day(&mut resumed);
            assert_eq!(spheres_sim::save(&resumed), spheres_sim::save(&continued));
            let good: serde_json::Value = serde_json::from_str(&saved).unwrap();
            let mut absent = good.clone();
            absent["world"]["supplier_operations"]["contracts"][p.to_string()]
                .as_object_mut()
                .unwrap()
                .remove("preproduction_inputs_bn");
            if before_phase == "manufacturing" {
                let error = spheres_sim::load(&absent.to_string()).unwrap_err();
                assert!(error.contains("ownership ambiguous"), "{error}");
            } else {
                let recovered = spheres_sim::load(&absent.to_string()).unwrap();
                assert_eq!(
                    spheres_sim::save(&recovered),
                    saved,
                    "only the uniquely proven missing expense classification may be added"
                );
            }
            let mut downgraded = good.clone();
            downgraded["supplier_operations_version"] = serde_json::json!(0);
            assert!(spheres_sim::load(&downgraded.to_string()).is_err());
            let mut downgraded = good.clone();
            downgraded["world"]["supplier_operations"]["version"] = serde_json::json!(0);
            assert!(spheres_sim::load(&downgraded.to_string()).is_err());
            for bad in [
                serde_json::json!(-1.0),
                serde_json::json!(0.0),
                serde_json::json!(term.input_cost_bn + 0.01),
            ] {
                let mut corrupt = good.clone();
                corrupt["world"]["supplier_operations"]["contracts"][p.to_string()]
                    ["preproduction_inputs_bn"] = bad;
                assert!(
                    spheres_sim::load(&corrupt.to_string()).is_err(),
                    "changed expense ownership must be rejected"
                );
            }
        }
        if product(&w, c, p).stock == 1 {
            break;
        }
    }
    assert_eq!(
        seen,
        std::collections::BTreeSet::from(["development", "tooling", "manufacturing"])
    );
    assert_eq!(
        product(&w, c, p).stock,
        1,
        "one genuinely paid unit must finish"
    );
    assert!(running_preproduction > 0.0);
    assert!(w.supplier_operations.contracts[&p].input_cost_bn > running_preproduction);
    restored(&w);
}

#[test]
fn s08_import_enrollment_reviews_all_supplier_rules_and_refuses_atomically() {
    let (original, _) = fixture();
    assert!(companies::import_enrollment_quote(&original, HOME).valid);
    for missing in [
        "daily",
        "operations",
        "resources",
        "manufacturing",
        "construction",
    ] {
        let mut w = original.clone();
        match missing {
            "daily" => w.rules.daily_simulation = false,
            "operations" => w.rules.military_operations = false,
            "resources" => w.rules.resource_market = false,
            "manufacturing" => w.rules.manufacturing_system = false,
            _ => w.rules.production_system = false,
        }
        let before = spheres_sim::save(&w);
        let q = companies::import_enrollment_quote(&w, HOME);
        assert!(!q.valid, "missing {missing}");
        if missing == "construction" {
            assert!(q
                .reason
                .as_deref()
                .unwrap()
                .contains("financial construction"));
        }
        assert_eq!(spheres_sim::save(&w), before, "preview must be pure");
        let result = spheres_sim::apply_command(
            &mut w,
            &Command::Company {
                nation: HOME,
                order: companies::CompanyOrder::EnableImports { quote: q.token },
            },
        );
        assert!(result.is_err());
        assert_eq!(
            spheres_sim::save(&w),
            before,
            "refusal cannot activate equipment, imports or programmes"
        );
    }
}
