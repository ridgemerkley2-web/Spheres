//! Unmodified 137-country start, daily Materials operating pilot census.
//! This is a universal invariant harness and descriptive census, not a chosen
//! adoption-rate target. No gifts, forced orders, softened wars or seeded funds.
//! Usage: materials_census DAYS SEED [REPORT.json] [FINAL_SAVE.json]
use spheres_sim::{
    clock, commerce::Good, init::world_1990, load, materials, production::ProjectKind as K,
    province_economy, save, starting_industry, tick_day, world::{GameRules, NationId, WorldState},
};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Default, serde::Serialize)]
struct Country {
    name: String,
    size: String,
    starting_gdp_bn: f64,
    end_gdp_bn: f64,
    end_alive: bool,
    mapped_provinces: usize,
    material_provinces: usize,
    inherited_material_equivalents: f64,
    unallocated_material_equivalents: f64,
    orders_started: u64,
    orders_completed: u64,
    orders_cancelled: u64,
    orders_expired: u64,
    blocked_order_days: u64,
    limited_order_days: u64,
    running_order_days: u64,
    delivered_packs: f64,
    conversion_paid_bn: f64,
    energy_paid_bn: f64,
    first_order_day: Option<i32>,
    first_output_day: Option<i32>,
    raw_used: [f64;12],
    final_retained_orders: usize,
    final_active_orders: usize,
    final_closed_orders: usize,
    lifetime: materials::Account,
    final_materials_covered_annual_bn: f64,
    final_materials_additional_annual_bn: f64,
    final_debt_gdp: f64,
    final_treasury_bn: Option<f64>,
    final_debt_bn: Option<f64>,
    final_last_ai_action: String,
    final_last_ai_reason: String,
    bootstrap: Bootstrap,
}
/// Dates are absolute simulation days, not inferred ETAs or queued capacity.
#[derive(Default, serde::Serialize)]
struct Milestones {
    enrollment: Option<i32>,
    paid_processing_output: Option<i32>,
    machinery_project_started: Option<i32>,
    machinery_installed_observed: Option<i32>,
    machinery_output: Option<i32>,
    intermediate_import_delivered: Option<i32>,
    capital_import_delivered: Option<i32>,
    end_of_day_intermediate_stock: Option<i32>,
    end_of_day_capital_stock: Option<i32>,
    research_goods_consumed: Option<i32>,
}
#[derive(Default, serde::Serialize)]
struct SupplyUse {
    paid_processing_intermediates: f64,
    inherited_materials_intermediates: f64,
    domestic_capital_goods: f64,
    imported_intermediates_delivered: f64,
    imported_capital_goods_delivered: f64,
    intermediates_used_by_machinery: f64,
    intermediates_used_by_research: f64,
    capital_goods_used_by_research: f64,
    prototype_credit_created: f64,
}
impl SupplyUse {
    fn add(&mut self, other: &Self) {
        self.paid_processing_intermediates += other.paid_processing_intermediates;
        self.inherited_materials_intermediates += other.inherited_materials_intermediates;
        self.domestic_capital_goods += other.domestic_capital_goods;
        self.imported_intermediates_delivered += other.imported_intermediates_delivered;
        self.imported_capital_goods_delivered += other.imported_capital_goods_delivered;
        self.intermediates_used_by_machinery += other.intermediates_used_by_machinery;
        self.intermediates_used_by_research += other.intermediates_used_by_research;
        self.capital_goods_used_by_research += other.capital_goods_used_by_research;
        self.prototype_credit_created += other.prototype_credit_created;
    }
}
#[derive(Default, serde::Serialize)]
struct ReasonCount {
    observations: u64,
    first_exact_example: String,
}
type Reasons = BTreeMap<String, ReasonCount>;
#[derive(Default, serde::Serialize)]
struct Bootstrap {
    first_day: Milestones,
    supply_and_use: SupplyUse,
    /// Source-prefixed templates; numeric runs are replaced by #. Bounded at
    /// 64 templates per source plus an overflow bucket, not an event journal.
    blocked_operation_reasons: Reasons,
    ai_review_actions: BTreeMap<String, u64>,
    ai_review_reasons: Reasons,
    #[serde(skip)]
    last_observed_evaluations: u32,
}
#[derive(Default, serde::Serialize)]
struct SizeSummary {
    countries: usize,
    countries_ordering: usize,
    countries_producing: usize,
    unmapped_countries: usize,
    orders_started: u64,
    orders_completed: u64,
    orders_expired: u64,
    delivered_packs: f64,
    service_paid_bn: f64,
    blocked_order_days: u64,
    countries_enrolled: usize,
    countries_with_paid_processing_output: usize,
    countries_starting_machinery: usize,
    countries_with_installed_machinery_observed: usize,
    countries_producing_capital_goods: usize,
    countries_receiving_intermediate_imports: usize,
    countries_receiving_capital_imports: usize,
    countries_using_goods_in_research: usize,
    supply_and_use: SupplyUse,
    blocked_operation_reasons: Reasons,
    ai_review_actions: BTreeMap<String, u64>,
    ai_review_reasons: Reasons,
}

fn reason_template(reason: &str) -> String {
    let mut out = String::new();
    let mut number = false;
    for ch in reason.chars() {
        if ch.is_ascii_digit() {
            if !number { out.push('#'); }
            number = true;
        } else {
            number = false;
            out.push(ch);
        }
    }
    out
}
fn record_reason(reasons: &mut Reasons, source: &str, reason: &str, count: u64) {
    let prefix = format!("{source}: ");
    record_template(reasons, source, format!("{prefix}{}", reason_template(reason)), reason, count);
}
fn record_template(reasons: &mut Reasons, source: &str, mut key: String, example: &str, count: u64) {
    let prefix = format!("{source}: ");
    if !reasons.contains_key(&key) && reasons.keys().filter(|k| k.starts_with(&prefix)).count() >= 64 {
        key = format!("{prefix}[other reason templates]");
    }
    let row = reasons.entry(key).or_default();
    row.observations += count;
    if row.first_exact_example.is_empty() { row.first_exact_example = example.into(); }
}
fn merge_reasons(out: &mut Reasons, source: &Reasons) {
    for (key, row) in source {
        let (kind, _) = key.split_once(": ").expect("observer reason source");
        record_template(out, kind, key.clone(), &row.first_exact_example, row.observations);
    }
}

/// Read only existing daily receipts. The ownership map is captured before the
/// tick for active/project sites: later warfare cannot reattribute production
/// that already occurred before the war phase. No province snapshots/quotes.
fn operating_owners(w: &WorldState) -> BTreeMap<String, NationId> {
    w.production.industry.sites.keys().chain(w.production.industry.modules.keys())
        .chain(w.production.projects.iter().map(|p| &p.district))
        .filter_map(|d| w.districts.get(d).map(|&n| (d.clone(), n))).collect()
}
fn observe_bootstrap(w: &WorldState, settled_day: i32, owners: &BTreeMap<String, NationId>,
    countries: &mut BTreeMap<NationId, Country>) {
    for n in &w.nations {
        let Some(c) = countries.get_mut(&n.id) else { continue; };
        if n.program_budget.is_some() { c.bootstrap.first_day.enrollment.get_or_insert(settled_day); }
    }
    for p in &w.production.projects {
        if p.kind != K::MachineryWorks { continue; }
        let Some(c) = countries.get_mut(&p.nation) else { continue; };
        c.bootstrap.first_day.machinery_project_started.get_or_insert(p.started_day.unwrap_or(settled_day));
    }
    if w.production.industry.last_day == Some(settled_day) {
        for operation in &w.production.industry.operations {
            let Some(c) = owners.get(&operation.district).and_then(|n| countries.get_mut(n)) else { continue; };
            let b = &mut c.bootstrap;
            if operation.kind == K::MachineryWorks {
                b.first_day.machinery_installed_observed.get_or_insert(settled_day);
                if operation.output_daily > 0.0 {
                    b.first_day.machinery_output.get_or_insert(settled_day);
                    b.supply_and_use.domestic_capital_goods += operation.output_daily;
                    // The settled machinery recipe consumes exactly one
                    // intermediate per actual capital pack (industry::tick_day).
                    b.supply_and_use.intermediates_used_by_machinery += operation.output_daily;
                }
            } else if matches!(operation.kind, K::ProcessingPlant | K::StarterIndustry) {
                if operation.output_daily > 0.0 {
                    b.first_day.paid_processing_output.get_or_insert(settled_day);
                    b.supply_and_use.paid_processing_intermediates += operation.output_daily;
                }
            }
            if operation.status == "blocked" {
                record_reason(&mut b.blocked_operation_reasons, operation.kind.key(),
                    operation.reason.as_deref().unwrap_or("No reason recorded"), 1);
            }
        }
    }
    for (nation, program) in &w.production.industry.research {
        let Some(c) = countries.get_mut(nation) else { continue; };
        for operation in program.operations.iter().filter(|o| o.day == settled_day) {
            let b = &mut c.bootstrap;
            if operation.goods_used.intermediates > 0.0 || operation.goods_used.capital_goods > 0.0 {
                b.first_day.research_goods_consumed.get_or_insert(settled_day);
                b.supply_and_use.intermediates_used_by_research += operation.goods_used.intermediates;
                b.supply_and_use.capital_goods_used_by_research += operation.goods_used.capital_goods;
                b.supply_and_use.prototype_credit_created += operation.prototype_credit;
            }
            if operation.status == "blocked" {
                record_reason(&mut b.blocked_operation_reasons, "research", &operation.reason, 1);
            }
        }
    }
    if let Some(commerce) = &w.commerce {
        // Deliveries are appended in date order and old history is pruned.
        // Only today's tail is visited, never a whole retained trading year.
        for delivery in commerce.goods_deliveries.iter().rev().take_while(|d| d.day >= settled_day)
            .filter(|d| d.day == settled_day) {
            let Some(c) = countries.get_mut(&delivery.buyer) else { continue; };
            if delivery.quantity <= 0.0 { continue; }
            match delivery.good {
                Good::Intermediates => {
                    c.bootstrap.first_day.intermediate_import_delivered.get_or_insert(settled_day);
                    c.bootstrap.supply_and_use.imported_intermediates_delivered += delivery.quantity;
                }
                Good::CapitalGoods => {
                    c.bootstrap.first_day.capital_import_delivered.get_or_insert(settled_day);
                    c.bootstrap.supply_and_use.imported_capital_goods_delivered += delivery.quantity;
                }
            }
        }
    }
    for (nation, stock) in &w.production.industry.goods {
        let Some(c) = countries.get_mut(nation) else { continue; };
        if stock.intermediates > 0.0 { c.bootstrap.first_day.end_of_day_intermediate_stock.get_or_insert(settled_day); }
        if stock.capital_goods > 0.0 { c.bootstrap.first_day.end_of_day_capital_stock.get_or_insert(settled_day); }
    }
    for (nation, plan) in &w.economic_ai.nations {
        let Some(c) = countries.get_mut(nation) else { continue; };
        let b = &mut c.bootstrap;
        if plan.evaluations > b.last_observed_evaluations {
            *b.ai_review_actions.entry(plan.action.clone()).or_default() += 1;
            record_reason(&mut b.ai_review_reasons, "decision", &plan.reason, 1);
        }
        b.last_observed_evaluations = plan.evaluations;
    }
}
fn size(gdp: f64) -> &'static str {
    if gdp < 1.0 { "micro_under_1bn" }
    else if gdp < 10.0 { "small_1_to_10bn" }
    else if gdp < 100.0 { "medium_10_to_100bn" }
    else if gdp < 1000.0 { "large_100_to_1000bn" }
    else { "major_over_1000bn" }
}
fn close(a: f64, b: f64, message: &str) {
    assert!(a.is_finite() && b.is_finite() && (a-b).abs() <= 1e-9 * a.abs().max(b.abs()).max(1.0),
        "{message}: {a} versus {b}");
}
fn nonnegative(value: f64, name: &str) {
    assert!(value.is_finite() && value >= 0.0, "Invalid {name}: {value}");
}
fn daily_invariants(w: &WorldState) {
    for n in w.nations.iter().filter(|n| n.alive) {
        assert!(n.gdp.is_finite() && n.gdp > 0.0, "{} GDP on {}", n.id.name(), w.date_str());
        nonnegative(n.debt_gdp, "national debt ratio");
        if let Some(v) = n.treasury_bn { nonnegative(v, "treasury"); }
        if let Some(v) = n.debt_bn { nonnegative(v, "debt principal"); }
    }
    if let Some(m) = &w.resources.market {
        for s in &m.stocks { nonnegative(s.quantity, "raw stock"); }
        for c in &m.cash { nonnegative(c.balance_bn, "resource cash"); }
    }
    for g in w.production.industry.goods.values() {
        nonnegative(g.intermediates, "intermediate packs");
        nonnegative(g.capital_goods, "capital-goods packs");
    }
    if let Some(c) = &w.commerce {
        for cargo in &c.cargo { nonnegative(cargo.quantity, "industrial cargo"); }
        for contract in &c.contracts { nonnegative(contract.escrow_bn, "goods escrow"); }
    }
    if let Some(m) = &w.materials {
        for a in m.accounts.values() {
            nonnegative(a.delivered, "lifetime Materials delivery");
            nonnegative(a.conversion_paid_bn, "lifetime conversion payment");
            nonnegative(a.energy_paid_bn, "lifetime energy payment");
        }
        for o in &m.orders {
            nonnegative(o.quantity, "order quantity"); nonnegative(o.delivered, "order delivery");
            nonnegative(o.remaining, "order remainder"); nonnegative(o.reserved_daily, "reserved capacity");
            close(o.delivered + o.remaining, o.quantity, "finite order conservation");
            assert!(o.delivered <= o.quantity + 1e-9);
        }
    }
}
fn accounting_invariants(w: &WorldState, opening: &starting_industry::StartingIndustry) {
    assert_eq!(w.starting_industry.as_ref().unwrap(), opening, "Inherited capacity/provenance must stay frozen, not grow with GDP");
    for n in w.nations.iter().filter(|n| n.alive) {
        let Some(s) = province_economy::snapshot(w, n.id) else { continue; };
        close(s.total_gdp_bn, n.gdp, "national accounting anchor");
        close(s.provinces.iter().map(|p| p.total_gdp_bn).sum::<f64>() + s.unallocated_gdp_bn,
            n.gdp, "province plus unmapped accounts");
        close(s.inherited_gdp_bn + s.project_gdp_bn, n.gdp, "inherited plus additional output");
        close(s.sectors.iter().map(|p| p.gdp_bn).sum(), n.gdp, "national sectors");
        for m in s.materials_accounting.iter().chain(s.provinces.iter().filter_map(|p| p.materials_accounting.as_ref())) {
            nonnegative(m.background_annual_bn, "background Materials");
            nonnegative(m.unobserved_annual_bn, "unobserved Materials");
            close(m.observed_annual_bn + m.unobserved_annual_bn, m.total_annual_bn, "observed plus unobserved Materials");
            close(m.background_annual_bn + m.additional_annual_bn, m.total_annual_bn, "background plus additional Materials");
            close(m.already_included_annual_bn + m.additional_annual_bn, m.observed_annual_bn, "overlap plus new Materials");
            assert!(m.already_included_annual_bn <= m.background_annual_bn + 1e-9);
        }
    }
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let days = args.get(1).map(|s| s.parse::<u32>().expect("DAYS must be an integer")).unwrap_or(1096);
    let seed = args.get(2).map(|s| s.parse::<u64>().expect("SEED must be an integer")).unwrap_or(42);
    assert!(days > 0, "DAYS must be positive");
    let started = std::time::Instant::now();
    let mut w = world_1990(GameRules { seed, daily_simulation: true, economic_competition: true,
        production_system: true, resource_market: true, manufacturing_system: true,
        logistics_routes: true, physical_logistics: true, ..GameRules::default() });
    assert!(w.player.is_none());
    assert_eq!(w.nations.iter().filter(|n| n.alive).count(), 137);
    assert!(w.production.is_empty() && w.resources.is_empty() && w.materials.is_none());
    starting_industry::enable_new_world(&mut w).unwrap();
    assert!(w.production.is_empty() && w.resources.is_empty() && w.materials.is_none(), "Estimates cannot grant usable assets/inputs");
    province_economy::enable(&mut w);
    let opening = w.starting_industry.as_ref().unwrap().clone();
    let mut countries: BTreeMap<_, _> = w.nations.iter().filter(|n| n.alive).map(|n| {
        let mapped = w.districts.iter().filter(|(_, id)| **id == n.id).map(|(d, _)| d).collect::<Vec<_>>();
        let materials = mapped.iter().filter_map(|d| opening.provinces.get(*d)).collect::<Vec<_>>();
        (n.id, Country { name: n.id.name().into(), size: size(n.gdp).into(), starting_gdp_bn: n.gdp,
            mapped_provinces: mapped.len(), material_provinces: materials.iter().filter(|a| a.factory_equivalents[1] > 0.0).count(),
            inherited_material_equivalents: materials.iter().map(|a| a.factory_equivalents[1]).sum(),
            unallocated_material_equivalents: opening.unallocated.get(&n.id).map_or(0.0, |a| a.factory_equivalents[1]),
            ..Country::default() })
    }).collect();
    let mut seen_orders = BTreeSet::new();
    let mut seen_closed = BTreeSet::new();
    let mut raw_totals = BTreeMap::<u32,[f64;12]>::new();
    for step in 0..days {
        let settled_day = clock::absolute_day(&w);
        let owners = operating_owners(&w);
        tick_day(&mut w, &[]);
        daily_invariants(&w);
        observe_bootstrap(&w, settled_day, &owners, &mut countries);
        if let Some(m) = &w.materials {
            for o in &m.orders {
                let Some(c) = countries.get_mut(&o.nation) else { continue; };
                if seen_orders.insert(o.id) {
                    c.orders_started += 1;
                    // Observe the signing date. A post-production AI command
                    // may schedule its first service date for tomorrow.
                    c.first_order_day.get_or_insert(settled_day);
                }
                if !o.active() && seen_closed.insert(o.id) {
                    match o.status.as_str() { "completed" => c.orders_completed += 1,
                        "cancelled" => c.orders_cancelled += 1, "expired" => c.orders_expired += 1, _ => {} }
                }
                if o.last_day == Some(settled_day) {
                    match o.status.as_str() { "blocked" => {
                            c.blocked_order_days += 1;
                            record_reason(&mut c.bootstrap.blocked_operation_reasons, "materials",
                                o.reason.as_deref().unwrap_or("No reason recorded"), 1);
                        },
                        "limited" => c.limited_order_days += 1, "running" => c.running_order_days += 1, _ => {} }
                    if o.output_today > 0.0 { c.first_output_day.get_or_insert(settled_day); }
                    let old_raw = raw_totals.entry(o.id).or_insert([0.0;12]);
                    for i in 0..12 { c.raw_used[i] += (o.raw_used[i] - old_raw[i]).max(0.0); }
                    *old_raw = o.raw_used;
                }
            }
            for (id, account) in &m.accounts {
                if let Some(c) = countries.get_mut(id) {
                    c.delivered_packs = account.delivered;
                    c.conversion_paid_bn = account.conversion_paid_bn;
                    c.energy_paid_bn = account.energy_paid_bn;
                    c.bootstrap.supply_and_use.inherited_materials_intermediates = account.delivered;
                }
            }
        }
        if (step + 1) % 90 == 0 || step + 1 == days {
            accounting_invariants(&w, &opening);
            eprintln!("seed={seed} day={}/{} date={} orders={} delivered={:.6} machinery_started={} machinery_producing={} importers={} elapsed_s={:.1}", step + 1, days,
                w.date_str(), seen_orders.len(), countries.values().map(|c| c.delivered_packs).sum::<f64>(),
                countries.values().filter(|c| c.bootstrap.first_day.machinery_project_started.is_some()).count(),
                countries.values().filter(|c| c.bootstrap.first_day.machinery_output.is_some()).count(),
                countries.values().filter(|c| c.bootstrap.first_day.intermediate_import_delivered.is_some()
                    || c.bootstrap.first_day.capital_import_delivered.is_some()).count(), started.elapsed().as_secs_f64());
        }
    }
    let census_save = save(&w);
    let hash = spheres_sim::state_hash(&w);
    for (id, c) in &mut countries {
        if let Some(n) = w.nation_opt(*id) {
            c.end_gdp_bn = n.gdp; c.end_alive = n.alive;
            c.final_debt_gdp = n.debt_gdp; c.final_treasury_bn = n.treasury_bn; c.final_debt_bn = n.debt_bn;
        }
        let (covered, added) = province_economy::materials_summary(&w, *id);
        c.final_materials_covered_annual_bn = covered; c.final_materials_additional_annual_bn = added;
        if let Some(m) = &w.materials {
            for o in m.orders.iter().filter(|o| o.nation == *id) {
                c.final_retained_orders += 1;
                if o.active() { c.final_active_orders += 1; } else { c.final_closed_orders += 1; }
            }
            c.lifetime = m.accounts.get(id).cloned().unwrap_or_default();
            close(c.lifetime.delivered, c.delivered_packs, "observer and lifetime delivered packs");
        }
        if let Some(p) = w.economic_ai.nations.get(id) {
            c.final_last_ai_action = p.action.clone(); c.final_last_ai_reason = p.reason.clone();
        }
    }
    let mut sizes = BTreeMap::<String, SizeSummary>::new();
    for c in countries.values() {
        let s = sizes.entry(c.size.clone()).or_default();
        s.countries += 1; s.countries_ordering += usize::from(c.orders_started > 0);
        s.countries_producing += usize::from(c.delivered_packs > 0.0);
        s.unmapped_countries += usize::from(c.mapped_provinces == 0);
        s.orders_started += c.orders_started; s.orders_completed += c.orders_completed; s.orders_expired += c.orders_expired;
        s.delivered_packs += c.delivered_packs; s.service_paid_bn += c.conversion_paid_bn + c.energy_paid_bn;
        s.blocked_order_days += c.blocked_order_days;
        let m = &c.bootstrap.first_day;
        s.countries_enrolled += usize::from(m.enrollment.is_some());
        s.countries_with_paid_processing_output += usize::from(m.paid_processing_output.is_some());
        s.countries_starting_machinery += usize::from(m.machinery_project_started.is_some());
        s.countries_with_installed_machinery_observed += usize::from(m.machinery_installed_observed.is_some());
        s.countries_producing_capital_goods += usize::from(m.machinery_output.is_some());
        s.countries_receiving_intermediate_imports += usize::from(m.intermediate_import_delivered.is_some());
        s.countries_receiving_capital_imports += usize::from(m.capital_import_delivered.is_some());
        s.countries_using_goods_in_research += usize::from(m.research_goods_consumed.is_some());
        s.supply_and_use.add(&c.bootstrap.supply_and_use);
        merge_reasons(&mut s.blocked_operation_reasons, &c.bootstrap.blocked_operation_reasons);
        merge_reasons(&mut s.ai_review_reasons, &c.bootstrap.ai_review_reasons);
        for (action, count) in &c.bootstrap.ai_review_actions {
            *s.ai_review_actions.entry(action.clone()).or_default() += count;
        }
    }
    // The final branch is an extra 30-day replay check, NOT part of census totals.
    let mut resumed = load(&census_save).expect("final census save must load");
    assert_eq!(census_save, save(&resumed), "exact save roundtrip");
    for day in 1..=30 {
        tick_day(&mut w, &[]); tick_day(&mut resumed, &[]);
        daily_invariants(&w); daily_invariants(&resumed);
        assert_eq!(save(&w), save(&resumed), "save/resume branch diverged on continuation day {day}");
        assert_eq!(w.rng.state, resumed.rng.state, "RNG replay");
    }
    accounting_invariants(&w, &opening);
    let report = serde_json::json!({
        "days": days, "seed": seed, "start_countries": 137, "census_final_hash": format!("{hash:016x}"),
        "census_basis": "Descriptive daily-world census; no imposed adoption target, gifts, forced orders, or macro overrides.",
        "capacity_basis": "Frozen, historically grounded game equivalents; missing geography stays unallocated. No literal establishment counts claimed.",
        "bootstrap_basis": {
            "dates": "First observed absolute simulation day, null if never observed. A machinery project start is not installation; installation requires a real operating entry, even if blocked; output requires positive settled packs.",
            "supply": "Actual domestic paid processing, inherited Materials production and delivered manufactured imports are separate. Fungible inventory has no lot provenance: no claim that a particular consumed pack came from a domestic plant or importer. Orders, offers and cargo in transit are not delivered supply.",
            "use": "Machinery consumes one intermediate per actual capital pack in the current recipe. Research consumption and prototype credits come from dated settled receipts; this is not extra GDP or a claim of completed technology.",
            "ownership": "Plant output belongs to the owner before the production/war tick; Materials orders and research retain their recorded sponsoring nation. Only the original 137-country cohort enters size summaries; invariants still inspect every current living nation.",
            "reasons": "Counts are blocked operation-days (not distinct country-days) and fresh AI review decisions, never repeated resting text. Numeric runs are replaced by #; at most 64 templates plus an overflow bucket per source, with one exact example. No adoption-rate bar is imposed."
        },
        "invariants": { "daily_finite_balances": true, "quarterly_gdp_and_materials_reconciliation": true,
            "frozen_inherited_capacity": true, "initial_no_gifts": true, "save_roundtrip": true,
            "save_resume_continuation_days": 30, "continuation_exact_bytes_and_rng": true },
        "countries_ordering": countries.values().filter(|c| c.orders_started > 0).count(),
        "countries_producing": countries.values().filter(|c| c.delivered_packs > 0.0).count(),
        "unmapped_countries": countries.values().filter(|c| c.mapped_provinces == 0).map(|c| &c.name).collect::<Vec<_>>(),
        "mapped_without_materials": countries.values().filter(|c| c.mapped_provinces > 0 && c.material_provinces == 0).map(|c| &c.name).collect::<Vec<_>>(),
        "sizes": sizes, "countries": countries, "elapsed_seconds": started.elapsed().as_secs_f64()
    });
    let json = serde_json::to_string_pretty(&report).unwrap();
    if let Some(path) = args.get(3) { std::fs::write(path, &json).expect("write report artifact"); }
    else { println!("{json}"); }
    if let Some(path) = args.get(4) { std::fs::write(path, census_save).expect("write final census save"); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reason_templates_bound_cardinality_without_losing_observations() {
        let mut reasons = Reasons::new();
        record_reason(&mut reasons, "materials", "Needs 10.250 packs", 1);
        record_reason(&mut reasons, "materials", "Needs 900.500 packs", 2);
        assert_eq!(reasons.len(), 1);
        assert_eq!(reasons.values().next().unwrap().observations, 3);
        assert_eq!(reasons.values().next().unwrap().first_exact_example, "Needs 10.250 packs");
        for length in 1..=100 { record_reason(&mut reasons, "materials", &"X".repeat(length), 1); }
        assert_eq!(reasons.len(), 65);
        assert_eq!(reasons.values().map(|r| r.observations).sum::<u64>(), 103);
        let mut aggregate = Reasons::new();
        merge_reasons(&mut aggregate, &reasons);
        assert_eq!(aggregate.values().map(|r| r.observations).sum::<u64>(), 103);
        assert!(aggregate.contains_key("materials: [other reason templates]"));
    }

    #[test]
    fn observer_counts_only_actual_dated_receipts_and_never_changes_the_world() {
        use spheres_sim::{commerce, industry};
        // Synthetic observer receipts test accounting, NOT natural adoption.
        // The executable census always starts without these fixtures.
        let mut w = world_1990(GameRules { daily_simulation: true, ..GameRules::default() });
        let today = clock::absolute_day(&w);
        let district = w.districts.iter().find(|(_, n)| **n == NationId::USA).unwrap().0.clone();
        let mut countries = BTreeMap::from([(NationId::USA, Country::default()), (NationId::France, Country::default())]);
        // Model a later territorial transfer: the observed daily production
        // remains with its pre-tick owner, not the new end-of-day owner.
        let owners = BTreeMap::from([(district.clone(), NationId::France)]);
        w.production.industry.last_day = Some(today);
        w.production.industry.operations = vec![
            industry::SiteStatus { district: district.clone(), kind: K::StarterIndustry, level: 0,
                capacity_micros: Some(100_000), status: "running".into(), reason: None,
                output_daily: 3.0, power_used_daily: 3.0, cash_spent_daily_bn: 0.01 },
            industry::SiteStatus { district: district.clone(), kind: K::MachineryWorks, level: 1,
                capacity_micros: None, status: "running".into(), reason: None,
                output_daily: 2.0, power_used_daily: 4.0, cash_spent_daily_bn: 0.01 },
        ];
        w.commerce = Some(commerce::Commerce { goods_deliveries: vec![
            commerce::GoodsDelivery { contract: 1, day: today - 1, buyer: NationId::USA,
                seller: NationId::France, good: Good::Intermediates, quantity: 99.0 },
            commerce::GoodsDelivery { contract: 2, day: today, buyer: NationId::USA,
                seller: NationId::France, good: Good::Intermediates, quantity: 4.0 },
        ], ..commerce::Commerce::default() });
        w.production.industry.research.insert(NationId::USA, industry::ResearchProgram {
            operations: vec![industry::ResearchOperation { district, nation: NationId::USA,
                level: 1, day: today, technology: None, technology_name: None, status: "running".into(),
                reason: "Settled testing work".into(), prototype_credit: 0.001,
                cash_spent_daily_bn: 0.0001, goods_used: industry::Goods { intermediates: 0.1, capital_goods: 0.1 } }],
            ..industry::ResearchProgram::default()
        });
        w.economic_ai.nations.insert(NationId::USA, spheres_sim::economic_ai::NationPlan {
            evaluations: 1, action: "wait".into(), reason: "Needs 5 packs".into(),
            ..spheres_sim::economic_ai::NationPlan::default()
        });
        let before = save(&w);
        observe_bootstrap(&w, today, &owners, &mut countries);
        assert_eq!(save(&w), before);
        let france = &countries[&NationId::France].bootstrap;
        assert_eq!(france.supply_and_use.paid_processing_intermediates, 3.0);
        assert_eq!(france.supply_and_use.domestic_capital_goods, 2.0);
        assert_eq!(france.supply_and_use.intermediates_used_by_machinery, 2.0);
        assert_eq!(france.first_day.machinery_installed_observed, Some(today));
        let usa = &countries[&NationId::USA].bootstrap;
        assert_eq!(usa.supply_and_use.imported_intermediates_delivered, 4.0);
        assert_eq!(usa.supply_and_use.intermediates_used_by_research, 0.1);
        assert_eq!(usa.first_day.paid_processing_output, None);
        assert_eq!(usa.first_day.end_of_day_intermediate_stock, None, "receipts cannot invent remaining stock");
        assert_eq!(usa.ai_review_actions["wait"], 1);
        // No new dated receipts or review: yesterday's retained rows do not
        // count again merely because another day is being observed.
        observe_bootstrap(&w, today + 1, &owners, &mut countries);
        assert_eq!(countries[&NationId::France].bootstrap.supply_and_use.domestic_capital_goods, 2.0);
        assert_eq!(countries[&NationId::USA].bootstrap.supply_and_use.imported_intermediates_delivered, 4.0);
        assert_eq!(countries[&NationId::USA].bootstrap.ai_review_actions["wait"], 1);
        assert_eq!(save(&w), before);
    }
}
