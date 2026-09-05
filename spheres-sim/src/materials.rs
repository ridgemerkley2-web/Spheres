//! Finite government toll-manufacturing orders against inherited Materials.
//!
//! These are MODEL pack conversions, not transcribed historical production.
//! Government supplies already-owned raw inputs and pays conversion and power
//! service fees from the shared daily ministry ledger. Opening capacity grants
//! no inventory. Only a completely funded and supplied bundle creates packs.
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{
    clock,
    commerce::{self, Good},
    industry,
    production::ProjectKind as K,
    programs,
    resources::{self, Commodity, ALL},
    world::{NationId, WorldState, BUDGET_INDUSTRY},
};

pub const ORDER_PC: f64 = 2.0;
pub const MIN_DELIVERY_DAYS: u32 = 7;
pub const MAX_DELIVERY_DAYS: u32 = 365;
pub const CONVERSION_CASH_PER_PACK_BN: f64 = 0.00001;
pub const ENERGY_CASH_PER_POWER_BN: f64 = 0.000002;
pub const MAX_ACTIVE_ORDERS: usize = 32;
pub const MAX_ORDER_QUANTITY: f64 = 1_000_000.0;
const QUANTUM: f64 = 1e-9;
const NOTE: &str = "A government toll-manufacturing contract, not free national inventory. Supply your own raw inputs; pay conversion and generating services only as packs are delivered. Existing funded plants run first and share power, grids, storage and ministry authority. Historical capacity and its GDP are estimates. No automatic imports, private cash ledger or ambient shortage penalty.";

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Materials {
    pub next_id: u32,
    pub orders: Vec<Order>,
    pub last_day: Option<i32>,
    pub accounts: BTreeMap<NationId, Account>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Account {
    pub delivered: f64,
    pub conversion_paid_bn: f64,
    pub energy_paid_bn: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Order {
    pub id: u32,
    pub nation: NationId,
    pub district: String,
    pub quantity: f64,
    pub delivered: f64,
    pub remaining: f64,
    pub delivery_days: u32,
    /// First eligible service date, which can follow a post-settlement signing.
    pub start_day: i32,
    /// Exclusive: work may occur on start_day through deadline_day - 1.
    pub deadline_day: i32,
    pub reserved_daily: f64,
    pub status: String,
    pub reason: Option<String>,
    pub last_day: Option<i32>,
    pub closed_day: Option<i32>,
    pub spent_conversion_bn: f64,
    pub spent_energy_bn: f64,
    pub output_today: f64,
    pub power_today: f64,
    pub raw_used: [f64; 12],
}
impl Order {
    pub fn active(&self) -> bool {
        matches!(
            self.status.as_str(),
            "pending" | "running" | "limited" | "paused" | "blocked"
        )
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct InputRequirement {
    pub commodity: Commodity,
    pub name: String,
    pub unit: String,
    pub required: f64,
    pub stock_available: f64,
}
#[derive(Clone, Debug, Serialize)]
pub struct Quote {
    pub nation: NationId,
    pub district: String,
    pub eligible: bool,
    pub can_start: bool,
    pub refusal: Option<String>,
    pub quantity: f64,
    pub delivery_days: u32,
    pub reserved_daily: f64,
    pub capacity_daily: f64,
    pub inputs_daily: [f64; 12],
    pub requirements: Vec<InputRequirement>,
    pub conversion_daily_bn: f64,
    pub energy_daily_bn: f64,
    pub conversion_total_bn: f64,
    pub energy_total_bn: f64,
    pub available_conversion_bn: f64,
    pub available_energy_bn: f64,
    pub feasible_today: f64,
    pub blockers: Vec<String>,
    pub political_cost: f64,
    pub note: String,
}
#[derive(Clone, Debug, Serialize)]
pub struct ProvinceView {
    pub district: String,
    pub name: String,
    pub capacity_daily: f64,
    pub reserved_daily: f64,
    pub available_daily: f64,
    pub output_daily: f64,
    pub utilization: f64,
    pub order: Option<u32>,
    pub recommended_quantity: f64,
    pub recommended_days: u32,
    pub quote: Quote,
}
#[derive(Clone, Debug, Serialize)]
pub struct Snapshot {
    pub nation: NationId,
    pub capacity_daily: f64,
    pub output_daily: f64,
    pub demand_daily: f64,
    pub reserved_daily: f64,
    pub stock: f64,
    pub storage_capacity: f64,
    pub imports_daily: f64,
    pub exports_daily: f64,
    pub inherited_gdp_annual_bn: f64,
    pub new_gdp_annual_bn: f64,
    pub status: String,
    pub reason: String,
    pub provinces: Vec<ProvinceView>,
    pub orders: Vec<Order>,
    pub min_delivery_days: u32,
    pub max_delivery_days: u32,
    pub as_of_day: Option<i32>,
    pub account: Account,
    pub note: String,
}

fn floor(v: f64) -> f64 {
    (v.max(0.0) * 1e9).floor() / 1e9
}
fn round(v: f64) -> f64 {
    (v.max(0.0) * 1e9).round() / 1e9
}
fn sane(v: f64) -> f64 {
    if v.is_finite() {
        v.max(0.0)
    } else {
        0.0
    }
}
pub fn enabled(w: &WorldState) -> bool {
    clock::is_daily(w)
        && w.rules.economic_competition
        && w.rules.production_system
        && w.rules.resource_market
        && w.starting_industry.is_some()
}
pub fn has_work(w: &WorldState) -> bool {
    w.materials.as_ref().is_some_and(|m| !m.orders.is_empty())
}
/// The accounting definition used to convert annual inherited value-added
/// capacity into daily pack capacity. Generating coal belongs to power, once.
pub fn processing_value_added_per_pack(w: &WorldState, district: &str) -> f64 {
    let power = industry::power_per_pack(w, district, K::ProcessingPlant);
    let mut raw = industry::operating_recipe(K::ProcessingPlant, 1.0, power);
    raw[Commodity::Coal.idx()] = (raw[Commodity::Coal.idx()] - round(power * 0.02)).max(0.0);
    let mut inputs = power * crate::gdp_projects::POWER_UNIT_BN;
    for c in ALL {
        if raw[c.idx()] > 0.0 {
            let Some(price) = resources::unit_price_bn(c) else {
                return 0.0;
            };
            inputs += raw[c.idx()] * price;
        }
    }
    sane(crate::gdp_projects::INTERMEDIATE_PACK_BN - inputs)
}
pub fn capacity_daily(w: &WorldState, district: &str) -> f64 {
    let Some(s) = w.starting_industry.as_ref() else {
        return 0.0;
    };
    let Some(a) = s.provinces.get(district) else {
        return 0.0;
    };
    let va = processing_value_added_per_pack(w, district);
    if va <= 0.0 {
        return 0.0;
    }
    floor(sane(
        a.factory_equivalents[1] * s.annual_capacity_per_equivalent_bn / (va * 365.0),
    ))
}
fn live_order<'a>(w: &'a WorldState, district: &str) -> Option<&'a Order> {
    w.materials.as_ref()?.orders.iter().find(|o| {
        o.district == district
            && o.active()
            && o.deadline_day > clock::absolute_day(w)
            && w.districts.get(district) == Some(&o.nation)
    })
}
fn eligibility(w: &WorldState, nation: NationId, district: &str) -> Option<String> {
    if !enabled(w) {
        return Some("Materials contracts require daily Economic Competition and a new-campaign industry estimate.".into());
    }
    if !w.nation_opt(nation).is_some_and(|n| n.alive) {
        return Some("This government is not active.".into());
    }
    if w.districts.get(district) != Some(&nation) {
        return Some("Choose a province controlled by your government.".into());
    }
    if resources::district_contested(w, district) {
        return Some("The selected province is contested.".into());
    }
    if capacity_daily(w, district) < QUANTUM {
        return Some("This province has no located inherited Materials capacity.".into());
    }
    if !programs::enrolled(w, nation) {
        return Some("Enact the five-department ministry budget first.".into());
    }
    None
}
fn rate(quantity: f64, days: u32) -> f64 {
    if !quantity.is_finite() || quantity <= 0.0 || days == 0 {
        0.0
    } else {
        sane((quantity / days as f64 * 1e9).ceil() / 1e9)
    }
}
/// AI reviews run after industry in SYSTEMS. A contract signed then cannot
/// reuse the settled date: its whole finite service window starts tomorrow.
/// Between normal API ticks the calendar has already advanced, so a player's
/// pre-settlement order still starts today. Existing saved dates are untouched.
fn first_work_day(w: &WorldState) -> Option<i32> {
    let today = clock::absolute_day(w);
    if w.production.industry.last_day == Some(today)
        || w.materials.as_ref().is_some_and(|m| m.last_day == Some(today))
    {
        today.checked_add(1)
    } else {
        Some(today)
    }
}
pub fn order_refusal(
    w: &WorldState,
    nation: NationId,
    district: &str,
    quantity: f64,
    delivery_days: u32,
) -> Option<String> {
    if let Some(r) = eligibility(w, nation, district) {
        return Some(r);
    }
    if !quantity.is_finite()
        || !(QUANTUM..=MAX_ORDER_QUANTITY).contains(&quantity)
        || (round(quantity) - quantity).abs() > 1e-12
    {
        return Some("Quantity must be positive, at most one million packs, finite and use at most nine decimal places.".into());
    }
    if !(MIN_DELIVERY_DAYS..=MAX_DELIVERY_DAYS).contains(&delivery_days) {
        return Some(format!(
            "Choose a delivery window of {MIN_DELIVERY_DAYS} to {MAX_DELIVERY_DAYS} days."
        ));
    }
    if first_work_day(w)
        .and_then(|start| start.checked_add(delivery_days as i32))
        .is_none()
    {
        return Some("The delivery deadline exceeds the game calendar.".into());
    }
    if quantity > capacity_daily(w, district) * delivery_days as f64
        || rate(quantity, delivery_days) > capacity_daily(w, district)
    {
        return Some(
            "The requested daily delivery exceeds this province's inherited Materials capacity."
                .into(),
        );
    }
    if live_order(w, district).is_some() {
        return Some(
            "This province already has an active Materials contract; finish or cancel it first."
                .into(),
        );
    }
    if w.materials.as_ref().is_some_and(|m| {
        m.orders
            .iter()
            .filter(|o| {
                o.nation == nation
                    && o.active()
                    && o.deadline_day > clock::absolute_day(w)
                    && w.districts.get(&o.district) == Some(&nation)
            })
            .count()
            >= MAX_ACTIVE_ORDERS
    }) {
        return Some(format!(
            "At most {MAX_ACTIVE_ORDERS} active Materials contracts can be administered at once."
        ));
    }
    if w.materials
        .as_ref()
        .is_some_and(|m| m.next_id.checked_add(1).is_none())
    {
        return Some("Materials order identifiers are exhausted.".into());
    }
    None
}
pub fn start_order(
    w: &mut WorldState,
    nation: NationId,
    district: &str,
    quantity: f64,
    delivery_days: u32,
) -> Result<u32, String> {
    if let Some(r) = order_refusal(w, nation, district, quantity, delivery_days) {
        return Err(r);
    }
    let day = first_work_day(w).ok_or("The delivery deadline exceeds the game calendar.")?;
    let deadline = day.checked_add(delivery_days as i32)
        .ok_or("The delivery deadline exceeds the game calendar.")?;
    let m = w.materials.get_or_insert_with(Materials::default);
    let id = m.next_id;
    m.next_id = id
        .checked_add(1)
        .ok_or("Materials order identifiers are exhausted.")?;
    m.orders.push(Order {
        id,
        nation,
        district: district.into(),
        quantity: round(quantity),
        delivered: 0.0,
        remaining: round(quantity),
        delivery_days,
        start_day: day,
        deadline_day: deadline,
        reserved_daily: rate(quantity, delivery_days),
        status: "pending".into(),
        reason: None,
        last_day: None,
        closed_day: None,
        spent_conversion_bn: 0.0,
        spent_energy_bn: 0.0,
        output_today: 0.0,
        power_today: 0.0,
        raw_used: [0.0; 12],
    });
    Ok(id)
}
pub fn cancel_refusal(w: &WorldState, nation: NationId, order: u32) -> Option<String> {
    let o = w
        .materials
        .as_ref()
        .and_then(|m| m.orders.iter().find(|o| o.id == order));
    match o {
        None => Some("This Materials order does not exist.".into()),
        Some(o) if o.nation != nation => {
            Some("Only the sponsoring government can cancel this order.".into())
        }
        Some(o) if !o.active() => Some("This Materials order is already closed.".into()),
        _ => None,
    }
}
pub fn cancel_order(w: &mut WorldState, nation: NationId, order: u32) -> Result<(), String> {
    if let Some(r) = cancel_refusal(w, nation, order) {
        return Err(r);
    }
    let today = clock::absolute_day(w);
    let o = w
        .materials
        .as_mut()
        .unwrap()
        .orders
        .iter_mut()
        .find(|o| o.id == order)
        .unwrap();
    o.status = "cancelled".into();
    o.closed_day = Some(today);
    o.reason = Some("Cancelled. Delivered packs and paid service work are retained; no unfinished work is charged or refunded.".into());
    Ok(())
}

fn current_output(w: &WorldState, order: &Order) -> f64 {
    // Between daily ticks the calendar points at tomorrow; show the most recent
    // settled production date, not a misleading zero throughout browser play.
    if w.materials
        .as_ref()
        .is_some_and(|m| m.last_day == order.last_day)
    {
        order.output_today
    } else {
        0.0
    }
}
fn remaining_power(w: &WorldState, nation: NationId, district: &str) -> (f64, f64) {
    let day = clock::absolute_day(w);
    let mut national = industry::power_capacity(w, nation);
    let mut local = crate::industrial_modules::effective_capacity(w, district, K::PowerGrid) * 5.0;
    if w.production.industry.last_day == Some(day) {
        for op in &w.production.industry.operations {
            if w.districts.get(&op.district) == Some(&nation) {
                national -= op.power_used_daily;
            }
            if op.district == district {
                local -= op.power_used_daily;
            }
        }
    }
    if let Some(m) = &w.materials {
        for o in m.orders.iter().filter(|o| o.last_day == Some(day)) {
            if o.nation == nation {
                national -= o.power_today;
            }
            if o.district == district {
                local -= o.power_today;
            }
        }
    }
    (national.max(0.0), local.max(0.0))
}
fn feasible(
    w: &WorldState,
    nation: NationId,
    district: &str,
    target: f64,
    power: f64,
    grid: f64,
) -> (f64, Vec<String>) {
    let per_power = industry::power_per_pack(w, district, K::ProcessingPlant);
    let room = (industry::goods_capacity(w, nation)
        - commerce::stock(w, nation, Good::Intermediates))
    .max(0.0);
    let conversion = programs::available_bn(w, nation, BUDGET_INDUSTRY, 2);
    let energy = programs::available_bn(w, nation, BUDGET_INDUSTRY, 1);
    let limits = [
        (
            room,
            "Storage is full; use or sell packs, or build a Warehouse.",
        ),
        (
            power / per_power,
            "Needs spare Power Generation after existing plants.",
        ),
        (
            grid / per_power,
            "Needs spare local Power Grid capacity after existing plants.",
        ),
        (
            conversion / CONVERSION_CASH_PER_PACK_BN,
            "Minerals & processing has insufficient operating authority.",
        ),
        (
            energy / (ENERGY_CASH_PER_POWER_BN * per_power),
            "Energy systems has insufficient operating authority.",
        ),
    ];
    let mut output = target;
    let mut blockers = Vec::new();
    for (limit, message) in limits {
        output = output.min(limit);
        if limit + 1e-12 < target {
            blockers.push(message.into());
        }
    }
    let unit = industry::operating_recipe(K::ProcessingPlant, 1.0, per_power);
    for c in ALL {
        if unit[c.idx()] > 0.0 {
            let have = resources::stockpile(w, nation, c);
            output = output.min(have / unit[c.idx()]);
            if have + 1e-12 < unit[c.idx()] * target {
                blockers.push(format!(
                    "Needs {} from your stockpile; no automatic purchase.",
                    c.name()
                ));
            }
        }
    }
    (floor(output), blockers)
}
pub fn quote(
    w: &WorldState,
    nation: NationId,
    district: &str,
    quantity: f64,
    delivery_days: u32,
) -> Quote {
    let refusal = order_refusal(w, nation, district, quantity, delivery_days);
    let capacity = capacity_daily(w, district);
    let target = rate(quantity, delivery_days).min(capacity);
    let power_per = industry::power_per_pack(w, district, K::ProcessingPlant);
    let inputs = industry::operating_recipe(K::ProcessingPlant, target, target * power_per);
    let (power, grid) = remaining_power(w, nation, district);
    let (feasible_today, mut blockers) = if w.nation_opt(nation).is_some() {
        feasible(w, nation, district, target, power, grid)
    } else {
        (0.0, vec!["This government is not active.".into()])
    };
    let pc = w.nation_opt(nation).map_or(0.0, |n| n.political_capital);
    if pc < ORDER_PC {
        blockers.push(format!(
            "Needs {ORDER_PC} political capital to place the contract."
        ));
    }
    let requirements = ALL
        .into_iter()
        .filter(|c| inputs[c.idx()] > 0.0)
        .map(|c| InputRequirement {
            commodity: c,
            name: c.name().into(),
            unit: c.unit().into(),
            required: inputs[c.idx()],
            stock_available: resources::stockpile(w, nation, c),
        })
        .collect();
    Quote {
        nation,
        district: district.into(),
        eligible: refusal.is_none(),
        can_start: refusal.is_none() && pc >= ORDER_PC,
        refusal,
        quantity: sane(quantity),
        delivery_days,
        reserved_daily: rate(quantity, delivery_days),
        capacity_daily: capacity,
        inputs_daily: inputs,
        requirements,
        conversion_daily_bn: target * CONVERSION_CASH_PER_PACK_BN,
        energy_daily_bn: target * power_per * ENERGY_CASH_PER_POWER_BN,
        conversion_total_bn: sane(quantity) * CONVERSION_CASH_PER_PACK_BN,
        energy_total_bn: sane(quantity) * power_per * ENERGY_CASH_PER_POWER_BN,
        available_conversion_bn: programs::available_bn(w, nation, BUDGET_INDUSTRY, 2),
        available_energy_bn: programs::available_bn(w, nation, BUDGET_INDUSTRY, 1),
        feasible_today,
        blockers,
        political_cost: ORDER_PC,
        note: NOTE.into(),
    }
}

/// Nonrecursive raw forecast. It never consults stockpiles, market prices or
/// resource draw, so merely ordering cannot increase the legacy opening stock.
pub fn resource_demand_daily(w: &WorldState, nation: NationId) -> [f64; 12] {
    let mut out = [0.0; 12];
    if !enabled(w) {
        return out;
    }
    if let Some(m) = &w.materials {
        for o in m.orders.iter().filter(|o| {
            o.nation == nation
                && o.active()
                && o.deadline_day > clock::absolute_day(w)
                && w.districts.get(&o.district) == Some(&nation)
                && !resources::district_contested(w, &o.district)
        }) {
            let target = o
                .remaining
                .min(o.reserved_daily)
                .min(capacity_daily(w, &o.district));
            let p = target * industry::power_per_pack(w, &o.district, K::ProcessingPlant);
            let raw = industry::operating_recipe(K::ProcessingPlant, target, p);
            for i in 0..12 {
                out[i] += raw[i];
            }
        }
    }
    out
}
/// Ingredients already owned for still-deliverable manual orders are protected
/// from automatic surplus sales. This is not a purchase order or stock grant.
pub fn resource_reserve(w: &WorldState, nation: NationId) -> [f64; 12] {
    let mut out = [0.0; 12];
    if !enabled(w) || !w.nation_opt(nation).is_some_and(|n| n.alive) {
        return out;
    }
    let today = clock::absolute_day(w);
    if let Some(m) = &w.materials {
        for o in m.orders.iter().filter(|o| {
            o.nation == nation
                && o.active()
                && o.deadline_day > today
                && w.districts.get(&o.district) == Some(&nation)
        }) {
            let target = o.remaining.min(
                o.reserved_daily.min(capacity_daily(w, &o.district))
                    * o.deadline_day.saturating_sub(today.max(o.start_day)).max(0) as f64,
            );
            let power = target * industry::power_per_pack(w, &o.district, K::ProcessingPlant);
            let raw = industry::operating_recipe(K::ProcessingPlant, target, power);
            for i in 0..12 {
                out[i] += raw[i];
            }
        }
    }
    out
}

/// Raw inputs scheduled by active finite Materials orders inside the next
/// `days`. Each order is capped by its remaining quantity, reserved daily
/// throughput, site capacity, and delivery window; its recipe is applied once
/// to that bounded output rather than repeating the whole order every day.
pub fn resource_demand_for_days(
    w: &WorldState,
    nation: NationId,
    days: i32,
) -> [f64; 12] {
    let mut out = [0.0; 12];
    if days <= 0 || !enabled(w) || !w.nation_opt(nation).is_some_and(|n| n.alive) {
        return out;
    }
    let today = clock::absolute_day(w);
    let forecast_start = resources::forecast_start_day(w);
    if let Some(materials) = &w.materials {
        for order in materials.orders.iter().filter(|order| {
            order.nation == nation
                && order.active()
                && order.deadline_day > today
                && w.districts.get(&order.district) == Some(&nation)
                && !resources::district_contested(w, &order.district)
        }) {
            // Forecast the next `days` unsettled execution dates. During the
            // in-tick AI review today's Materials slice has already run, while
            // after the clock advances that same next date is simply today.
            // Intersect the two half-open intervals so future starts and the
            // exclusive deadline cannot add a phantom day.
            let starts = forecast_start.max(order.start_day);
            let ends = forecast_start
                .saturating_add(days)
                .min(order.deadline_day);
            let usable_days = ends.saturating_sub(starts).max(0);
            let output = order
                .remaining
                .min(order.reserved_daily * usable_days as f64)
                .min(capacity_daily(w, &order.district) * usable_days as f64);
            let power = output
                * industry::power_per_pack(w, &order.district, K::ProcessingPlant);
            let raw = industry::operating_recipe(K::ProcessingPlant, output, power);
            for i in 0..12 {
                out[i] += raw[i];
            }
        }
    }
    out.map(round)
}

pub fn reserved_daily(w: &WorldState, nation: NationId) -> f64 {
    if !enabled(w) || !w.nation_opt(nation).is_some_and(|n| n.alive) {
        return 0.0;
    }
    w.materials.as_ref().map_or(0.0, |m| {
        m.orders
            .iter()
            .filter(|o| {
                o.nation == nation
                    && o.active()
                    && o.deadline_day > clock::absolute_day(w)
                    && w.districts.get(&o.district) == Some(&nation)
            })
            .map(|o| {
                o.remaining
                    .min(o.reserved_daily)
                    .min(capacity_daily(w, &o.district))
            })
            .sum()
    })
}
/// Planned finite delivery, including a temporarily paused contract. As with
/// an installed or queued plant, restoring inputs/power is preferable to buying
/// duplicate capacity. Expiry, cancellation or ownership loss releases it.
pub fn province_reserved_daily(w: &WorldState, nation: NationId, district: &str) -> f64 {
    if !enabled(w) || !w.nation_opt(nation).is_some_and(|n| n.alive) {
        return 0.0;
    }
    live_order(w, district)
        .filter(|o| o.nation == nation)
        .map_or(0.0, |o| {
            o.remaining
                .min(o.reserved_daily)
                .min(capacity_daily(w, district))
        })
}
pub fn power_used_daily(w: &WorldState, nation: NationId) -> f64 {
    w.materials.as_ref().map_or(0.0, |m| {
        m.orders
            .iter()
            .filter(|o| o.nation == nation && o.last_day == m.last_day)
            .map(|o| o.power_today)
            .sum()
    })
}
pub fn pending(w: &WorldState, nation: NationId) -> f64 {
    if !enabled(w) || !w.nation_opt(nation).is_some_and(|n| n.alive) {
        return 0.0;
    }
    w.materials.as_ref().map_or(0.0, |m| {
        m.orders
            .iter()
            .filter(|o| {
                o.nation == nation
                    && o.active()
                    && o.deadline_day > clock::absolute_day(w)
                    && w.districts.get(&o.district) == Some(&nation)
            })
            .map(|o| {
                o.remaining.min(
                    o.reserved_daily.min(capacity_daily(w, &o.district))
                        * o.deadline_day.saturating_sub(clock::absolute_day(w).max(o.start_day)).max(0) as f64,
                )
            })
            .sum()
    })
}
pub fn recommended_quantity(w: &WorldState, _nation: NationId, district: &str) -> f64 {
    floor((capacity_daily(w, district) * 0.1).min(1.0) * 30.0)
}

/// Called once after funded processing and machinery, using their exact
/// remaining shared dispatch counters. No separate generation allowance exists.
pub(crate) fn operate(
    w: &mut WorldState,
    power: &mut BTreeMap<NationId, f64>,
    grids: &mut BTreeMap<String, f64>,
) {
    if !enabled(w) || !has_work(w) {
        return;
    }
    let day = clock::absolute_day(w);
    if w.materials
        .as_ref()
        .is_some_and(|m| m.last_day == Some(day))
    {
        return;
    }
    w.materials.as_mut().unwrap().last_day = Some(day);
    let orders = w.materials.as_ref().unwrap().orders.clone();
    for (index, mut o) in orders.into_iter().enumerate() {
        if !o.active() || day < o.start_day {
            continue;
        }
        o.last_day = Some(day);
        o.output_today = 0.0;
        o.power_today = 0.0;
        if o.deadline_day <= day {
            o.status = "expired".into();
            o.reason = Some("Delivery window ended; unfinished quantity was not charged. Delivered goods remain yours.".into());
        } else if w.districts.get(&o.district) != Some(&o.nation)
            || !w.nation_opt(o.nation).is_some_and(|n| n.alive)
        {
            o.status = "cancelled".into();
            o.reason = Some("The sponsoring government lost the province; the order and its inputs do not transfer to the new owner.".into());
        } else if let Some(reason) = eligibility(w, o.nation, &o.district) {
            o.status = "blocked".into();
            o.reason = Some(reason);
        } else if !w
            .nation(o.nation)
            .program_budget
            .as_ref()
            .is_some_and(|p| p.day == Some(day) && p.settled_day != Some(day))
        {
            o.status = "blocked".into();
            o.reason = Some("The daily department funding ledger is not open.".into());
        } else {
            let available_power = power
                .entry(o.nation)
                .or_insert_with(|| industry::power_capacity(w, o.nation));
            let grid = grids.entry(o.district.clone()).or_insert_with(|| {
                crate::industrial_modules::effective_capacity(w, &o.district, K::PowerGrid) * 5.0
            });
            let target = o
                .reserved_daily
                .min(o.remaining)
                .min(capacity_daily(w, &o.district));
            let (output, blockers) =
                feasible(w, o.nation, &o.district, target, *available_power, *grid);
            if output < QUANTUM {
                o.status = "paused".into();
                o.reason = Some(if blockers.is_empty() {
                    "Waiting for usable power, inputs, storage or operating authority; the order will resume automatically.".into()
                } else {
                    format!("Waiting for supply. {}", blockers.join(" "))
                });
            } else {
                let used_power =
                    output * industry::power_per_pack(w, &o.district, K::ProcessingPlant);
                let draw = industry::operating_recipe(K::ProcessingPlant, output, used_power);
                let conversion = (output * CONVERSION_CASH_PER_PACK_BN)
                    .min(programs::available_bn(w, o.nation, BUDGET_INDUSTRY, 2));
                let energy = (used_power * ENERGY_CASH_PER_POWER_BN).min(programs::available_bn(
                    w,
                    o.nation,
                    BUDGET_INDUSTRY,
                    1,
                ));
                if let Err((c, _, _)) = resources::consume_stockpile_atomic(w, o.nation, &draw) {
                    o.status = "paused".into();
                    o.reason = Some(format!(
                        "Waiting for {} after available supply changed; the order will resume automatically.",
                        c.name()
                    ));
                } else {
                    programs::spend_operating(w, o.nation, BUDGET_INDUSTRY, 2, conversion)
                        .expect("preflighted conversion authority");
                    programs::spend_operating(w, o.nation, BUDGET_INDUSTRY, 1, energy)
                        .expect("preflighted generation authority");
                    let goods = w.production.industry.goods.entry(o.nation).or_default();
                    goods.intermediates = round(goods.intermediates + output);
                    *available_power = (*available_power - used_power).max(0.0);
                    *grid = (*grid - used_power).max(0.0);
                    o.delivered = round(o.delivered + output).min(o.quantity);
                    o.remaining = round(o.quantity - o.delivered);
                    o.output_today = output;
                    o.power_today = used_power;
                    o.spent_conversion_bn += conversion;
                    o.spent_energy_bn += energy;
                    let account = w
                        .materials
                        .as_mut()
                        .unwrap()
                        .accounts
                        .entry(o.nation)
                        .or_default();
                    account.delivered = round(account.delivered + output);
                    account.conversion_paid_bn += conversion;
                    account.energy_paid_bn += energy;
                    for i in 0..12 {
                        o.raw_used[i] = round(o.raw_used[i] + draw[i]);
                    }
                    o.status = if o.remaining < QUANTUM {
                        "completed"
                    } else if output + 1e-12 < target {
                        "limited"
                    } else {
                        "running"
                    }
                    .into();
                    o.reason = if blockers.is_empty() {
                        None
                    } else {
                        Some(blockers.join(" "))
                    };
                    crate::gdp_projects::record_materials_operation(
                        w,
                        o.nation,
                        &o.district,
                        o.id,
                        output,
                        o.reserved_daily,
                        used_power,
                        draw,
                        conversion,
                        energy,
                    );
                }
            }
        }
        if !o.active() {
            o.closed_day = Some(day);
        }
        w.materials.as_mut().unwrap().orders[index] = o;
    }
    // Receipts remain auditable for a year, bounded further to the latest 256
    // closed orders per nation. Lifetime payments/output survive in accounts.
    let mut counts = BTreeMap::<NationId, usize>::new();
    let m = w.materials.as_mut().unwrap();
    let mut keep = vec![true; m.orders.len()];
    for (i, o) in m.orders.iter().enumerate().rev() {
        if !o.active() {
            let count = counts.entry(o.nation).or_default();
            *count += 1;
            keep[i] = *count <= 256 && o.closed_day.is_some_and(|d| day.saturating_sub(d) <= 365);
        }
    }
    let mut i = 0;
    m.orders.retain(|_| {
        let retain = keep[i];
        i += 1;
        retain
    });
}

pub fn snapshot(w: &WorldState, nation: NationId) -> Option<Snapshot> {
    let inherited = w.starting_industry.as_ref()?;
    w.nation_opt(nation)?;
    let orders: Vec<_> = w.materials.as_ref().map_or_else(Vec::new, |m| {
        m.orders
            .iter()
            .filter(|o| o.nation == nation)
            .cloned()
            .collect()
    });
    let mut provinces = Vec::new();
    for d in inherited
        .provinces
        .keys()
        .filter(|d| w.districts.get(*d) == Some(&nation))
    {
        let capacity = capacity_daily(w, d);
        let order = live_order(w, d);
        let reserved = order.map_or(0.0, |o| o.reserved_daily.min(o.remaining));
        let output = orders
            .iter()
            .filter(|o| o.district == *d)
            .map(|o| current_output(w, o))
            .sum::<f64>();
        let days = 30;
        let quantity = recommended_quantity(w, nation, d);
        provinces.push(ProvinceView {
            district: d.clone(),
            name: crate::districts::name_of(d).unwrap_or(d).into(),
            capacity_daily: capacity,
            reserved_daily: reserved,
            available_daily: if order.is_some() { 0.0 } else { capacity },
            output_daily: output,
            utilization: if capacity > 0.0 {
                output / capacity
            } else {
                0.0
            },
            order: order.map(|o| o.id),
            recommended_quantity: quantity,
            recommended_days: days,
            quote: quote(w, nation, d, quantity, days),
        });
    }
    let day = w
        .commerce
        .as_ref()
        .and_then(|c| c.last_day)
        .unwrap_or(clock::absolute_day(w));
    let (mut imports, mut exports) = (0.0, 0.0);
    if let Some(c) = &w.commerce {
        for d in c
            .goods_deliveries
            .iter()
            .filter(|d| d.day == day && d.good == Good::Intermediates)
        {
            if d.buyer == nation {
                imports += d.quantity;
            }
            if d.seller == nation {
                exports += d.quantity;
            }
        }
    }
    let capacity = provinces.iter().map(|p| p.capacity_daily).sum();
    let output = orders.iter().map(|o| current_output(w, o)).sum();
    let reserved = reserved_daily(w, nation);
    let (inherited_gdp, new_gdp) = crate::province_economy::materials_summary(w, nation);
    let (status, reason) = if provinces.is_empty() {
        (
            "unmapped",
            "Inherited industry remains in the national account; no province location is invented.",
        )
    } else if orders
        .iter()
        .any(|o| o.active() && matches!(o.status.as_str(), "paused" | "blocked"))
    {
        (
            "needs_inputs",
            "A contract is waiting. Inspect its power, input, storage and budget requirements.",
        )
    } else if reserved > 0.0 {
        ("producing", "Finite government orders reserve existing capacity; fulfilled packs can feed projects, machinery or trade.")
    } else {
        ("available", "Choose a province and commission a finite delivery. Uncontracted industry stays in the background economy.")
    };
    Some(Snapshot {
        nation,
        capacity_daily: capacity,
        output_daily: output,
        demand_daily: commerce::demand(w, nation, Good::Intermediates) / 30.0,
        reserved_daily: reserved,
        stock: commerce::stock(w, nation, Good::Intermediates),
        storage_capacity: industry::goods_capacity(w, nation),
        imports_daily: imports,
        exports_daily: exports,
        inherited_gdp_annual_bn: inherited_gdp,
        new_gdp_annual_bn: new_gdp,
        status: status.into(),
        reason: reason.into(),
        provinces,
        orders,
        min_delivery_days: MIN_DELIVERY_DAYS,
        max_delivery_days: MAX_DELIVERY_DAYS,
        as_of_day: w.materials.as_ref().and_then(|m| m.last_day),
        account: w
            .materials
            .as_ref()
            .and_then(|m| m.accounts.get(&nation))
            .cloned()
            .unwrap_or_default(),
        note: NOTE.into(),
    })
}
