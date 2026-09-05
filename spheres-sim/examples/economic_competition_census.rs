//! A real-world daily competition census, not a calibrated success-rate bar.
//! Run `cargo run -p spheres-sim --release --example economic_competition_census -- 3650 42`.
//! No starting assets, cash, inputs or politics are overridden. JSON to stdout;
//! periodic progress to stderr. An optional argument after the report path
//! writes an explicitly named final-world replay artifact. No live campaign or
//! server is read or written.
use spheres_sim::{
    clock, commerce, economic_ai, gdp_projects,
    init::world_1990,
    production, programs, province_economy, tick_day,
    world::{GameRules, NationId},
};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Default, serde::Serialize)]
struct Country {
    name: String,
    starting_gdp_bn: f64,
    tier: String,
    mapped_provinces: usize,
    evaluated: u32,
    enrolled: bool,
    construction_days_with_paid_work: u32,
    construction_value_added_bn: f64,
    completed_sites: u32,
    /// Fractional modules are not rounded into old integer facility counts.
    module_provinces: usize,
    module_capacity_standards: f64,
    active_modules: usize,
    module_construction_value_added_bn: f64,
    first_productive_day: Option<i32>,
    active_projects: usize,
    completed_mines: usize,
    active_mines: usize,
    produced_packs: f64,
    purchased_goods_bn: f64,
    exported_goods_bn: f64,
    delivered_import_reference_bn: f64,
    delivered_export_reference_bn: f64,
    days_with_delivered_imports: u32,
    end_import_escrow_bn: f64,
    end_in_flight_intermediate_packs: f64,
    end_in_flight_capital_packs: f64,
    end_in_flight_reference_bn: f64,
    end_held_in_flight_reference_bn: f64,
    #[serde(skip)]
    last_delivery_day: Option<i32>,
    end_gdp_bn: f64,
    gdp_change_percent: f64,
    project_gdp_bn: f64,
    end_debt_gdp: f64,
    end_treasury_bn: Option<f64>,
    end_debt_bn: Option<f64>,
    alive: bool,
    last_action: String,
    last_reason: String,
    funding: Option<economic_ai::FundingHorizon>,
}

/// Observe only the day that just settled, before the world clock advanced.
/// These receipt values are fixed reference prices, not the negotiated cash
/// price. Read daily so the sim's 365-day dependency window cannot erase earlier
/// deliveries from this cumulative census. The observer never mutates the sim.
fn record_deliveries(
    countries: &mut BTreeMap<NationId, Country>,
    receipts: &[commerce::DeliveredSource],
    settled_day: i32,
) {
    let mut imports = BTreeMap::<NationId, f64>::new();
    let mut exports = BTreeMap::<NationId, f64>::new();
    for r in receipts.iter().filter(|r| r.day == settled_day) {
        assert!(r.reference_value_bn.is_finite() && r.reference_value_bn >= 0.0);
        *imports.entry(r.buyer).or_default() += r.reference_value_bn;
        *exports.entry(r.seller).or_default() += r.reference_value_bn;
    }
    for (id, row) in countries {
        if row.last_delivery_day.is_some_and(|day| day >= settled_day) {
            continue;
        }
        row.last_delivery_day = Some(settled_day);
        let received = imports.get(id).copied().unwrap_or(0.0);
        row.delivered_import_reference_bn += received;
        row.delivered_export_reference_bn += exports.get(id).copied().unwrap_or(0.0);
        if received > 0.0 {
            row.days_with_delivered_imports += 1;
        }
    }
}

fn record_open_commerce(row: &mut Country, ledger: &commerce::Commerce, id: NationId) {
    row.end_import_escrow_bn = ledger
        .contracts
        .iter()
        .filter(|c| c.buyer == id)
        .map(|c| c.escrow_bn)
        .sum();
    row.end_in_flight_intermediate_packs = 0.0;
    row.end_in_flight_capital_packs = 0.0;
    row.end_in_flight_reference_bn = 0.0;
    row.end_held_in_flight_reference_bn = 0.0;
    for cargo in ledger.cargo.iter().filter(|c| c.buyer == id) {
        match cargo.good {
            commerce::Good::Intermediates => row.end_in_flight_intermediate_packs += cargo.quantity,
            commerce::Good::CapitalGoods => row.end_in_flight_capital_packs += cargo.quantity,
        }
        let value = cargo.quantity * commerce::reference_price_bn(cargo.good);
        row.end_in_flight_reference_bn += value;
        if cargo.hold_reason.is_some() {
            row.end_held_in_flight_reference_bn += value;
        }
    }
}
fn tier(gdp: f64) -> &'static str {
    if gdp < 1.0 {
        "micro_under_1bn"
    } else if gdp < 10.0 {
        "small_1_to_10bn"
    } else if gdp < 100.0 {
        "medium_10_to_100bn"
    } else if gdp < 1000.0 {
        "large_100_to_1000bn"
    } else {
        "major_over_1000bn"
    }
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let days = args
        .get(1)
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(3650);
    let seed = args
        .get(2)
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(42);
    let started = std::time::Instant::now();
    let mut w = world_1990(GameRules {
        seed,
        daily_simulation: true,
        economic_competition: true,
        production_system: true,
        resource_market: true,
        manufacturing_system: true,
        logistics_routes: true,
        physical_logistics: true,
        ..GameRules::default()
    });
    province_economy::enable(&mut w);
    let mut countries: BTreeMap<NationId, Country> = w
        .nations
        .iter()
        .filter(|n| n.alive)
        .map(|n| {
            (
                n.id,
                Country {
                    name: n.id.name().into(),
                    starting_gdp_bn: n.gdp,
                    tier: tier(n.gdp).into(),
                    mapped_provinces: w.districts.values().filter(|id| **id == n.id).count(),
                    ..Country::default()
                },
            )
        })
        .collect();
    for day in 0..days {
        let settled_day = clock::absolute_day(&w);
        tick_day(&mut w, &[]);
        if let Some(ledger) = &w.commerce {
            record_deliveries(&mut countries, &ledger.sourcing, settled_day);
        }
        for n in w.nations.iter().filter(|n| n.alive) {
            assert!(
                n.gdp.is_finite() && n.gdp > 0.0,
                "{} invalid GDP on {}",
                n.id.name(),
                w.date_str()
            );
            assert!(
                n.debt_gdp.is_finite() && n.debt_gdp >= 0.0,
                "{} invalid debt on {}",
                n.id.name(),
                w.date_str()
            );
            assert!(n.treasury_bn.is_none_or(|v| v.is_finite() && v >= 0.0));
        }
        let mut paid_today = BTreeSet::new();
        for row in gdp_projects::contributions(&w) {
            if row.sector == "construction" && row.counted && row.daily_value_added_bn > 0.0 {
                if let Some(owner) = w.districts.get(&row.district) {
                    if let Some(record) = countries.get_mut(owner) {
                        record.construction_value_added_bn += row.daily_value_added_bn;
                        if row.kind == "starter_industry" {
                            record.module_construction_value_added_bn += row.daily_value_added_bn;
                        }
                        paid_today.insert(*owner);
                    }
                }
            }
        }
        for owner in paid_today {
            countries
                .get_mut(&owner)
                .unwrap()
                .construction_days_with_paid_work += 1;
        }
        for site in &w.production.industry.operations {
            if let Some(owner) = w.districts.get(&site.district) {
                if let Some(row) = countries.get_mut(owner) {
                    row.produced_packs += site.output_daily;
                    if site.output_daily > 0.0 {
                        row.first_productive_day.get_or_insert(settled_day);
                    }
                }
            }
        }
        if day % 90 == 89 {
            eprintln!(
                "seed {seed}, day {}, {}, {} evaluated, {} active projects, {:.1}s elapsed",
                day + 1,
                w.date_str(),
                w.economic_ai.nations.len(),
                w.production.projects.len(),
                started.elapsed().as_secs_f64()
            );
        }
    }
    for (id, row) in &mut countries {
        let n = w.nation(*id);
        row.end_gdp_bn = n.gdp;
        row.end_debt_gdp = n.debt_gdp;
        row.alive = n.alive;
        row.gdp_change_percent = (n.gdp / row.starting_gdp_bn - 1.0) * 100.0;
        row.project_gdp_bn = province_economy::project_level(&w, *id).unwrap_or(0.0);
        row.end_treasury_bn = n.treasury_bn;
        row.end_debt_bn = n.debt_bn;
        row.enrolled = programs::enrolled(&w, *id);
        row.active_projects = production::projects_for(&w, *id).count();
        row.active_modules = production::projects_for(&w, *id)
            .filter(|p| p.kind == production::ProjectKind::StarterIndustry)
            .count();
        row.module_provinces = w
            .production
            .industry
            .modules
            .iter()
            .filter(|(d, size)| **size > 0 && w.districts.get(*d) == Some(id))
            .count();
        row.module_capacity_standards = w
            .production
            .industry
            .modules
            .iter()
            .filter(|(d, _)| w.districts.get(*d) == Some(id))
            .map(|(_, size)| *size as f64 / 1_000_000.0)
            .sum();
        row.active_mines = w
            .resources
            .mine_projects
            .iter()
            .filter(|m| m.started_by == *id)
            .count();
        row.completed_mines = w
            .resources
            .mines
            .iter()
            .filter(|m| w.districts.get(&m.district) == Some(id))
            .count();
        row.completed_sites = w
            .districts
            .iter()
            .filter(|(_, owner)| **owner == *id)
            .map(|(district, _)| {
                production::PROJECT_KINDS
                    .iter()
                    .map(|kind| production::level(&w, district, *kind) as u32)
                    .sum::<u32>()
            })
            .sum();
        if let Some(p) = w.economic_ai.nations.get(id) {
            row.evaluated = p.evaluations;
            row.last_action = p.action.clone();
            row.last_reason = p.reason.clone();
            row.funding = p
                .district
                .as_ref()
                .zip(p.project_kind)
                .map(|(d, k)| economic_ai::funding_horizon(&w, *id, d, k));
        }
        if let Some(a) = w.commerce.as_ref().and_then(|c| c.accounts.get(id)) {
            row.purchased_goods_bn = a.imports_reserved_bn - a.imports_refunded_bn;
            row.exported_goods_bn = a.exports_received_bn;
        }
        if let Some(ledger) = &w.commerce {
            record_open_commerce(row, ledger, *id);
        }
    }
    let mut tiers = BTreeMap::new();
    for key in [
        "micro_under_1bn",
        "small_1_to_10bn",
        "medium_10_to_100bn",
        "large_100_to_1000bn",
        "major_over_1000bn",
    ] {
        let rows: Vec<_> = countries.values().filter(|r| r.tier == key).collect();
        tiers.insert(key,serde_json::json!({"countries":rows.len(),"evaluated":rows.iter().filter(|r|r.evaluated>0).count(),
            "enrolled":rows.iter().filter(|r|r.enrolled).count(),"paid_progress":rows.iter().filter(|r|r.construction_days_with_paid_work>0).count(),
            "completed_sites":rows.iter().filter(|r|r.completed_sites>0).count(),"produced_packs":rows.iter().filter(|r|r.produced_packs>0.0).count(),
            "countries_with_modules":rows.iter().filter(|r|r.module_provinces>0).count(),
            "module_capacity_standards":rows.iter().map(|r|r.module_capacity_standards).sum::<f64>(),
            "module_construction_value_added_bn":rows.iter().map(|r|r.module_construction_value_added_bn).sum::<f64>(),
            "bought_goods":rows.iter().filter(|r|r.purchased_goods_bn>0.0).count(),"sold_goods":rows.iter().filter(|r|r.exported_goods_bn>0.0).count(),
            "received_deliveries":rows.iter().filter(|r|r.delivered_import_reference_bn>0.0).count(),
            "supplied_deliveries":rows.iter().filter(|r|r.delivered_export_reference_bn>0.0).count(),
            "delivered_import_reference_bn":rows.iter().map(|r|r.delivered_import_reference_bn).sum::<f64>(),
            "delivered_export_reference_bn":rows.iter().map(|r|r.delivered_export_reference_bn).sum::<f64>(),
            "net_imports_reserved_bn":rows.iter().map(|r|r.purchased_goods_bn).sum::<f64>(),
            "export_dispatch_cash_bn":rows.iter().map(|r|r.exported_goods_bn).sum::<f64>(),
            "end_import_escrow_bn":rows.iter().map(|r|r.end_import_escrow_bn).sum::<f64>(),
            "end_in_flight_reference_bn":rows.iter().map(|r|r.end_in_flight_reference_bn).sum::<f64>(),
            "end_held_in_flight_reference_bn":rows.iter().map(|r|r.end_held_in_flight_reference_bn).sum::<f64>()}));
    }
    let output=serde_json::to_string_pretty(&serde_json::json!({"seed":seed,"days":days,"end_day":clock::absolute_day(&w),
        "elapsed_seconds":started.elapsed().as_secs_f64(),
        "report_version":6,"review_days":economic_ai::REVIEW_DAYS,"note":"Measured run, not a historical calibration bar. Paid work and blockers are reported, not replaced by enrollment counts. Fractional starter-module capacity and first actual productive day are separate from integer legacy site counts. purchased_goods_bn is net escrowed import cash, exported_goods_bn is seller dispatch cash; neither proves arrival. Delivered reference value records only actual usable arrivals at fixed modeled prices, accumulated daily across the full run; in-flight cargo and undispatched escrow remain separate.",
        "tiers":tiers,"countries":countries.values().collect::<Vec<_>>()})).unwrap();
    if let Some(path) = args.get(3) {
        std::fs::write(path, output).expect("write census report");
        eprintln!("Census report: {path}");
    } else {
        println!("{output}");
    }
    if let Some(path) = args.get(4) {
        std::fs::write(path, spheres_sim::save(&w))
            .expect("write explicitly requested final-world artifact");
        eprintln!("Final world: {path}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deliveries_count_settled_day_once_and_survive_receipt_retention() {
        let mut countries = BTreeMap::from([
            (NationId::USA, Country::default()),
            (NationId::Germany, Country::default()),
            (NationId::France, Country::default()),
        ]);
        let receipts = vec![
            commerce::DeliveredSource {
                day: 9,
                buyer: NationId::USA,
                seller: NationId::Germany,
                reference_value_bn: 8.0,
            },
            commerce::DeliveredSource {
                day: 10,
                buyer: NationId::USA,
                seller: NationId::Germany,
                reference_value_bn: 0.01,
            },
            commerce::DeliveredSource {
                day: 10,
                buyer: NationId::USA,
                seller: NationId::France,
                reference_value_bn: 0.02,
            },
        ];
        record_deliveries(&mut countries, &receipts, 10);
        record_deliveries(&mut countries, &receipts, 10);
        record_deliveries(&mut countries, &receipts, 9);
        assert_eq!(
            countries[&NationId::USA].delivered_import_reference_bn,
            0.03
        );
        assert_eq!(countries[&NationId::USA].days_with_delivered_imports, 1);
        assert_eq!(
            countries[&NationId::Germany].delivered_export_reference_bn,
            0.01
        );
        assert_eq!(
            countries[&NationId::France].delivered_export_reference_bn,
            0.02
        );
        record_deliveries(&mut countries, &[], 400);
        assert_eq!(
            countries[&NationId::USA].delivered_import_reference_bn,
            0.03
        );
        assert_eq!(
            countries[&NationId::USA].purchased_goods_bn,
            0.0,
            "delivery value is not guessed negotiated cash"
        );
    }

    #[test]
    fn unfulfilled_escrow_is_not_a_delivery_receipt() {
        let mut countries = BTreeMap::from([(NationId::USA, Country::default())]);
        let ledger = commerce::Commerce {
            contracts: vec![commerce::Contract {
                id: 1,
                buyer: NationId::USA,
                seller: NationId::Germany,
                good: commerce::Good::Intermediates,
                quantity: 10.0,
                unit_price_bn: 0.001,
                remaining_quantity: 10.0,
                escrow_bn: 0.01,
                delivered_quantity: 0.0,
                cancelled_quantity: 0.0,
                paid_bn: 0.0,
                accepted_day: 0,
                expires_day: 30,
                status: "active".into(),
                reason: None,
            }],
            ..commerce::Commerce::default()
        };
        record_deliveries(&mut countries, &ledger.sourcing, 0);
        record_open_commerce(
            countries.get_mut(&NationId::USA).unwrap(),
            &ledger,
            NationId::USA,
        );
        let row = &countries[&NationId::USA];
        assert_eq!(row.end_import_escrow_bn, 0.01);
        assert_eq!(row.delivered_import_reference_bn, 0.0);
        assert_eq!(row.end_in_flight_reference_bn, 0.0);
    }

    #[test]
    fn held_and_moving_cargo_remain_separate_from_delivered_value() {
        let route = spheres_sim::logistics::RoutePlan {
            mode: "observer fixture".into(),
            nodes: vec![],
            distance_km: 10,
            estimated_days: 2,
            months: 1,
            capacity_tonnes: 20.0,
            bottleneck: "fixture".into(),
            chokepoints: vec![],
            segments: vec![],
        };
        let ledger = commerce::Commerce {
            cargo: vec![
                commerce::Cargo {
                    id: 1,
                    contract: 1,
                    buyer: NationId::USA,
                    seller: NationId::Germany,
                    good: commerce::Good::Intermediates,
                    quantity: 2.0,
                    route: route.clone(),
                    dispatched_day: 0,
                    due_day: 2,
                    hold_reason: None,
                },
                commerce::Cargo {
                    id: 2,
                    contract: 2,
                    buyer: NationId::USA,
                    seller: NationId::Germany,
                    good: commerce::Good::CapitalGoods,
                    quantity: 3.0,
                    route,
                    dispatched_day: 0,
                    due_day: 2,
                    hold_reason: Some("Warehouse full".into()),
                },
            ],
            ..commerce::Commerce::default()
        };
        let mut row = Country::default();
        record_open_commerce(&mut row, &ledger, NationId::USA);
        let moving = 2.0 * commerce::reference_price_bn(commerce::Good::Intermediates);
        let held = 3.0 * commerce::reference_price_bn(commerce::Good::CapitalGoods);
        assert_eq!(row.end_in_flight_intermediate_packs, 2.0);
        assert_eq!(row.end_in_flight_capital_packs, 3.0);
        assert_eq!(row.end_in_flight_reference_bn, moving + held);
        assert_eq!(row.end_held_in_flight_reference_bn, held);
        assert_eq!(row.delivered_import_reference_bn, 0.0);
        assert_eq!(
            row.end_import_escrow_bn, 0.0,
            "dispatched cargo is paid, not undispatched escrow"
        );
    }
}
