//! Player-directed military manufacturing.
//!
//! This is a routing layer over procurement the simulation already pays for.
//! It does not create a second defence budget or a second equipment stockpile:
//! directed lines divide [`crate::arsenal::line_of`], consume the resource
//! market's physical stock, and place the ordinary long-lead
//! [`crate::arsenal::Order`] rows.  Delivery, ageing, book value, and the war
//! model remain owned by `arsenal.rs`.

use serde::{Deserialize, Serialize};

use crate::arsenal::{self, DECK};
use crate::production::{self, Priority};
use crate::resources::{self, Commodity, ALL};
use crate::world::{NationId, WorldState};

pub const START_LINE_PC_COST: f64 = 8.0;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LineStatus {
    Producing,
    Slowed,
    Paused,
    Blocked,
}

impl LineStatus {
    pub fn key(self) -> &'static str {
        match self {
            Self::Producing => "producing",
            Self::Slowed => "slowed",
            Self::Paused => "paused",
            Self::Blocked => "blocked",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ManufacturingLine {
    pub id: u32,
    pub nation: NationId,
    pub district: String,
    /// Stable `arsenal::DECK` id, never a table index on disk.
    pub kit: String,
    pub priority: Priority,
    pub status: LineStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Cumulative value successfully put on the existing order book, $bn.
    pub ordered_bn: f64,
    /// Cumulative physical inputs consumed, in the resource table's units.
    pub resources_used: [f64; 12],
    /// Actual procurement value ordered on the latest settlement date. This is
    /// transient presentation state: persisting it would make otherwise
    /// identical daily and monthly simulations serialize differently.
    #[serde(default, skip)]
    pub ordered_today_bn: f64,
    /// Fraction of the scheduled line that physical inputs supplied in its
    /// latest settlement. Presentation state only; after loading a save the UI
    /// explicitly waits for the next settlement instead of inventing a rate.
    #[serde(default, skip)]
    pub throughput_today: f64,
    #[serde(default, skip)]
    pub settled_day: Option<i32>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Manufacturing {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub lines: Vec<ManufacturingLine>,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub next_id: u32,
}

fn is_zero(value: &u32) -> bool {
    *value == 0
}

impl Manufacturing {
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty() && self.next_id == 0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LinePlan {
    pub line: u32,
    pub kit: u16,
    pub budget_bn: f64,
    pub required: [f64; 12],
}

pub fn lines_for(w: &WorldState, nation: NationId) -> impl Iterator<Item = &ManufacturingLine> {
    w.manufacturing
        .lines
        .iter()
        .filter(move |line| line.nation == nation)
}

/// The recurring monthly procurement appropriation, excluding money banked
/// from an earlier under-supplied month. This is the server-authoritative budget
/// number a surface should quote.
pub fn budget_bn(w: &WorldState, nation: NationId) -> f64 {
    arsenal::budget_of(w.nation(nation))
}

/// What can be routed this month, including the existing capped procurement
/// bank. Kept separate from [`budget_bn`] so a surface cannot mistake a one-off
/// backlog for the standing appropriation.
pub fn available_bn(w: &WorldState, nation: NationId) -> f64 {
    arsenal::line_of(w.nation(nation))
}

pub fn plant_slots(w: &WorldState, district: &str) -> u8 {
    production::province_capabilities(w, district).arms_plants
}

pub fn used_slots(w: &WorldState, nation: NationId, district: &str) -> usize {
    lines_for(w, nation)
        .filter(|line| line.district == district)
        .count()
}

fn priority_weight(priority: Priority) -> f64 {
    match priority {
        Priority::High => 3.0,
        Priority::Normal => 2.0,
        Priority::Low => 1.0,
    }
}

fn dispatch_rank(priority: Priority) -> u8 {
    match priority {
        Priority::High => 0,
        Priority::Normal => 1,
        Priority::Low => 2,
    }
}

fn line_error(w: &WorldState, line: &ManufacturingLine) -> Option<String> {
    if !w.nation_opt(line.nation).is_some_and(|n| n.alive) {
        return Some("BLOCKED: the sponsoring government no longer exists.".into());
    }
    if w.districts.get(&line.district) != Some(&line.nation) {
        return Some(format!(
            "BLOCKED: {} is no longer controlled.",
            line.district
        ));
    }
    let slots = plant_slots(w, &line.district) as usize;
    if slots == 0 {
        return Some(format!("BLOCKED: {} has no arms plant.", line.district));
    }

    // Saves are allowed to outlive the code that created them. If a malformed
    // or future save carries too many lines, the same priority order used for
    // scarce material decides which physical slots still run.
    let mut at_site: Vec<&ManufacturingLine> = lines_for(w, line.nation)
        .filter(|candidate| candidate.district == line.district)
        .collect();
    at_site.sort_by_key(|candidate| (dispatch_rank(candidate.priority), candidate.id));
    if at_site
        .iter()
        .position(|candidate| candidate.id == line.id)
        .is_none_or(|i| i >= slots)
    {
        return Some(format!(
            "BLOCKED: all {} arms-plant slots in {} are assigned.",
            slots, line.district
        ));
    }

    let Some(kit) = arsenal::index_of(&line.kit) else {
        return Some(format!(
            "BLOCKED: equipment programme {} is unknown.",
            line.kit
        ));
    };
    if !arsenal::available(w.nation(line.nation)).contains(&kit) {
        return Some(format!(
            "BLOCKED: {} is not technologically available.",
            line.kit
        ));
    }
    None
}

/// A live structural or technology blocker for a standing line. Settlement
/// persists material slowdowns separately, while ownership and plant capacity
/// can change between month-ends; UI readers use this preview so a captured
/// factory never remains labelled as producing until the next tick.
pub fn line_blocker(w: &WorldState, line: &ManufacturingLine) -> Option<String> {
    line_error(w, line)
}

pub fn start_line_error(
    w: &WorldState,
    nation: NationId,
    district: &str,
    kit: &str,
) -> Option<String> {
    if !w.rules.manufacturing_system {
        return Some("Military manufacturing is not enabled in this game.".into());
    }
    if !w.rules.resource_market {
        return Some("Military manufacturing requires the resource market to be enabled.".into());
    }
    if !crate::economic_ai::may_direct(w, nation) {
        return Some("Only the player can direct military manufacturing.".into());
    }
    if !w.nation_opt(nation).is_some_and(|n| n.alive) {
        return Some(format!("{} is not an active government.", nation.name()));
    }
    match w.districts.get(district) {
        None => return Some(format!("No province called {}.", district)),
        Some(owner) if *owner != nation => {
            return Some(format!("{} does not control {}.", nation.name(), district));
        }
        _ => {}
    }
    let slots = plant_slots(w, district) as usize;
    if slots == 0 {
        return Some(format!("{} has no completed arms plant.", district));
    }
    if used_slots(w, nation, district) >= slots {
        return Some(format!(
            "All {} arms-plant slots in {} are assigned.",
            slots, district
        ));
    }
    let Some(index) = arsenal::index_of(kit) else {
        return Some(format!("No equipment programme called {}.", kit));
    };
    if !arsenal::available(w.nation(nation)).contains(&index) {
        let def = &DECK[index as usize];
        return Some(match def.tech {
            Some(tech) => format!("{} requires technology {}.", def.name, tech),
            None => format!("{} is not available to {}.", def.name, nation.name()),
        });
    }
    None
}

pub fn start_line(
    w: &mut WorldState,
    nation: NationId,
    district: &str,
    kit: &str,
) -> Result<u32, String> {
    if let Some(reason) = start_line_error(w, nation, district, kit) {
        return Err(reason);
    }
    let id = w.manufacturing.next_id.max(1);
    w.manufacturing.next_id = id.saturating_add(1);
    w.manufacturing.lines.push(ManufacturingLine {
        id,
        nation,
        district: district.to_string(),
        kit: kit.to_string(),
        priority: Priority::Normal,
        status: LineStatus::Producing,
        reason: None,
        ordered_bn: 0.0,
        resources_used: [0.0; 12],
        ordered_today_bn: 0.0,
        throughput_today: 0.0,
        settled_day: None,
    });
    let name = arsenal::index_of(kit)
        .and_then(|i| DECK.get(i as usize))
        .map_or(kit, |def| def.name);
    w.headline(format!(
        "{} opens a {} manufacturing line in {}.",
        nation.name(),
        name,
        district
    ));
    Ok(id)
}

pub fn set_priority(
    w: &mut WorldState,
    nation: NationId,
    line: u32,
    priority: Priority,
) -> Result<(), String> {
    if !w.rules.manufacturing_system {
        return Err("Military manufacturing is not enabled in this game.".into());
    }
    if !crate::economic_ai::may_direct(w, nation) {
        return Err("Only the player can direct military manufacturing.".into());
    }
    let row = w
        .manufacturing
        .lines
        .iter_mut()
        .find(|row| row.id == line)
        .ok_or_else(|| format!("No manufacturing line {}.", line))?;
    if row.nation != nation {
        return Err(format!(
            "{} does not control manufacturing line {}.",
            nation.name(),
            line
        ));
    }
    row.priority = priority;
    Ok(())
}

pub fn stop_line(w: &mut WorldState, nation: NationId, line: u32) -> Result<(), String> {
    if !w.rules.manufacturing_system {
        return Err("Military manufacturing is not enabled in this game.".into());
    }
    if !crate::economic_ai::may_direct(w, nation) {
        return Err("Only the player can direct military manufacturing.".into());
    }
    let Some(index) = w.manufacturing.lines.iter().position(|row| row.id == line) else {
        return Err(format!("No manufacturing line {}.", line));
    };
    if w.manufacturing.lines[index].nation != nation {
        return Err(format!(
            "{} does not control manufacturing line {}.",
            nation.name(),
            line
        ));
    }
    let removed = w.manufacturing.lines.remove(index);
    let name = arsenal::index_of(&removed.kit)
        .and_then(|i| DECK.get(i as usize))
        .map_or(removed.kit.as_str(), |def| def.name);
    w.headline(format!(
        "{} closes the {} manufacturing line in {}; existing orders remain.",
        nation.name(),
        name,
        removed.district
    ));
    Ok(())
}

/// This month's directed split, in material-dispatch order. Every line receives
/// a share of the one opening procurement envelope; an inoperable line has a
/// zero recipe but keeps its money slice so settlement can bank rather than
/// silently redirect it.
pub fn planned_allocations(w: &WorldState, nation: NationId) -> Vec<LinePlan> {
    allocations_with_envelope(w, nation, available_bn(w, nation))
}

/// Standing one-month material recipe for the directed lines, excluding the
/// one-off bank accumulated while earlier settlements were under-supplied. Strategic
/// forecasts use this instead of [`resource_draw`] so saved money is never
/// mistaken for recurring demand.
pub fn recurring_resource_draw(w: &WorldState, nation: NationId) -> [f64; 12] {
    let mut total = [0.0; 12];
    for plan in allocations_with_envelope(w, nation, budget_bn(w, nation)) {
        for commodity in ALL {
            total[commodity.idx()] += plan.required[commodity.idx()];
        }
    }
    total.map(|quantity| (quantity * 1e9).round() / 1e9)
}

/// Actual slices for the current daily or legacy monthly settlement. Monthly
/// forecasts remain monthly so reserve policies do not shrink with the clock.
pub fn tick_allocations(w: &WorldState, nation: NationId) -> Vec<LinePlan> {
    allocations_with_envelope(w, nation, arsenal::tick_line(w, nation))
}

fn allocations_with_envelope(w: &WorldState, nation: NationId, envelope: f64) -> Vec<LinePlan> {
    let mut lines: Vec<&ManufacturingLine> = lines_for(w, nation).collect();
    lines.sort_by_key(|line| (dispatch_rank(line.priority), line.id));
    let total_weight: f64 = lines
        .iter()
        .map(|line| priority_weight(line.priority))
        .sum();
    if lines.is_empty() || total_weight <= 0.0 {
        return vec![];
    }
    let program_funded = crate::programs::enrolled(w, nation);
    let mut remaining = envelope;
    let mut assigned = 0.0;
    let last = lines.len() - 1;
    lines
        .into_iter()
        .enumerate()
        .map(|(position, line)| {
            // `start_line` only writes known ids. The sentinel lets a save
            // whose deck entry was removed keep its budget slice and reach
            // `line_error`, where it is blocked and banked rather than silently
            // disappearing from the envelope.
            let kit = arsenal::index_of(&line.kit).unwrap_or(u16::MAX);
            let budget = if position == last {
                // Strict shared-pool debits must use their own sequential
                // residual. Preserve inherited arithmetic for unenrolled saves.
                if program_funded { remaining.max(0.0) } else { (envelope - assigned).max(0.0) }
            } else {
                let slice = envelope * priority_weight(line.priority) / total_weight;
                remaining -= slice;
                assigned += slice;
                slice
            };
            let required = if line_error(w, line).is_none() {
                resources::manufacturing_need(kit, budget)
            } else {
                [0.0; 12]
            };
            LinePlan {
                line: line.id,
                kit,
                budget_bn: budget,
                required,
            }
        })
        .collect()
}

pub fn line_resource_draw(w: &WorldState, line_id: u32) -> [f64; 12] {
    let Some(line) = w.manufacturing.lines.iter().find(|line| line.id == line_id) else {
        return [0.0; 12];
    };
    planned_allocations(w, line.nation)
        .into_iter()
        .find(|plan| plan.line == line_id)
        .map_or([0.0; 12], |plan| plan.required)
}

pub fn line_shortfalls(w: &WorldState, line_id: u32) -> [f64; 12] {
    let Some(line) = w.manufacturing.lines.iter().find(|line| line.id == line_id) else {
        return [0.0; 12];
    };
    shortfalls_for_plans(w, line.nation, line_id, planned_allocations(w, line.nation))
}

/// Present-tense shortfall for the exact settlement recipe. Earlier successful
/// lines reserve their daily bundles; banked funding is included once, not
/// divided by the current month length as though it were recurring income.
pub fn tick_line_shortfalls(w: &WorldState, line_id: u32) -> [f64; 12] {
    let Some(line) = w.manufacturing.lines.iter().find(|line| line.id == line_id) else {
        return [0.0; 12];
    };
    shortfalls_for_plans(w, line.nation, line_id, tick_allocations(w, line.nation))
}

fn shortfalls_for_plans(w: &WorldState, nation: NationId, line_id: u32, plans: Vec<LinePlan>) -> [f64; 12] {
    if !w.rules.resource_gates {
        return [0.0; 12];
    }

    // Preview the same proportional, priority-ordered settlement as
    // `settle_nation`. Each earlier plan reserves the fraction its tightest
    // input can support, leaving any physical remainder for lower priorities.
    let mut remaining: [f64; 12] = std::array::from_fn(|i| {
        let commodity = Commodity::from_idx(i).expect("twelve resource rows");
        resources::stockpile(w, nation, commodity)
    });
    for plan in plans {
        let shortfalls: [f64; 12] =
            std::array::from_fn(|i| (plan.required[i] - remaining[i]).max(0.0));
        if plan.line == line_id {
            return shortfalls;
        }
        let mut throughput: f64 = 1.0;
        for commodity in ALL {
            let want = plan.required[commodity.idx()];
            if want > 1e-12 {
                throughput = throughput
                    .min((remaining[commodity.idx()] / want).clamp(0.0, 1.0));
            }
        }
        let used = resources::scale_bundle(&plan.required, throughput);
        for commodity in ALL {
            remaining[commodity.idx()] =
                (remaining[commodity.idx()] - used[commodity.idx()]).max(0.0);
        }
    }
    [0.0; 12]
}

/// Aggregate demand submitted to the resource market before it clears.
pub fn resource_draw(w: &WorldState, nation: NationId) -> [f64; 12] {
    let mut total = [0.0; 12];
    for plan in planned_allocations(w, nation) {
        for commodity in ALL {
            total[commodity.idx()] += plan.required[commodity.idx()];
        }
    }
    total.map(|quantity| (quantity * 1e9).round() / 1e9)
}

fn set_blocked(w: &mut WorldState, id: u32, reason: String) {
    if let Some(line) = w.manufacturing.lines.iter_mut().find(|line| line.id == id) {
        line.status = LineStatus::Blocked;
        line.reason = Some(reason);
    }
}

/// Place one nation's directed monthly order slices. Called from `arsenal::tick`
/// after the spot market has cleared and after old orders have delivered.
pub(crate) fn settle_nation(w: &mut WorldState, nation: NationId) {
    let program_funded = crate::programs::enrolled(w, nation);
    let plans = tick_allocations(w, nation);
    if plans.is_empty() {
        return;
    }
    let recurring = budget_bn(w, nation);
    let mut banked = 0.0;
    w.nation_mut(nation).arsenal.banked = 0.0;
    let today = crate::clock::absolute_day(w);
    let settlement_period = if crate::clock::is_daily(w) { "day" } else { "month" };

    for plan in plans {
        if let Some(line) = w.manufacturing.lines.iter_mut().find(|line| line.id == plan.line) {
            line.ordered_today_bn = 0.0;
            line.throughput_today = 0.0;
            line.settled_day = Some(today);
        }
        let opening_line = w
            .manufacturing
            .lines
            .iter()
            .find(|line| line.id == plan.line)
            .cloned()
            .expect("planned line still exists");
        if let Some(reason) = line_error(w, &opening_line) {
            banked += plan.budget_bn;
            set_blocked(w, plan.line, reason);
            continue;
        }
        if plan.budget_bn <= 1e-12 {
            banked += plan.budget_bn;
            let line = w.manufacturing.lines.iter_mut().find(|line| line.id == plan.line).unwrap();
            line.status = LineStatus::Paused;
            line.reason = Some("PAUSED: the defense budget has no procurement funding; the line retains its place and will resume automatically.".into());
            continue;
        }

        let mut throughput = 1.0;
        let mut required = plan.required;
        let mut limiting_input = None;
        if w.rules.resource_gates {
            throughput = resources::bundle_throughput(w, nation, &plan.required);
            limiting_input = resources::limiting_bundle_input(w, nation, &plan.required);
            if throughput <= 1e-12 {
                banked += plan.budget_bn;
                let line = w.manufacturing.lines.iter_mut().find(|line| line.id == plan.line).unwrap();
                line.status = LineStatus::Paused;
                line.reason = Some(limiting_input.map_or_else(
                    || "PAUSED: the equipment line is waiting for physical inputs.".into(),
                    |(commodity, want, have)| format!(
                        "PAUSED: waiting for {}; full-speed production needs {:.2} this {}, with {:.2} available.",
                        commodity.name(), want, settlement_period, have
                    ),
                ));
                continue;
            }
            required = resources::scale_bundle(&plan.required, throughput);
            if let Err((commodity, want, have)) =
                resources::consume_stockpile_atomic(w, nation, &required)
            {
                banked += plan.budget_bn;
                let line = w.manufacturing.lines.iter_mut().find(|line| line.id == plan.line).unwrap();
                line.status = LineStatus::Paused;
                line.reason = Some(format!(
                    "PAUSED: {} supply changed before settlement; needs {:.2}, with {:.2} available.",
                    commodity.name(), want, have
                ));
                continue;
            }
        }

        let ordered_today = plan.budget_bn * throughput;
        banked += (plan.budget_bn - ordered_today).max(0.0);
        if program_funded {
            crate::programs::spend(w, nation, crate::world::BUDGET_DEFENSE, 3, ordered_today)
                .expect("the priority allocation reserves this proportional department slice before its atomic recipe");
        }
        let def = &DECK[plan.kit as usize];
        let units = ordered_today / def.unit_cost.max(1e-9);
        if units > 0.0 {
            let due_days = crate::clock::is_daily(w)
                .then(|| crate::clock::days_for_months(w, def.lead_months));
            // One row per kit per delivery month (arsenal::book_order).
            crate::arsenal::book_order(
                &mut w.nation_mut(nation).arsenal.orders,
                today,
                plan.kit,
                units,
                def.lead_months,
                due_days,
            );
        }
        let line = w
            .manufacturing
            .lines
            .iter_mut()
            .find(|line| line.id == plan.line)
            .expect("planned line still exists");
        line.status = if throughput + 1e-12 < 1.0 {
            LineStatus::Slowed
        } else {
            LineStatus::Producing
        };
        line.reason = if throughput + 1e-12 < 1.0 {
            limiting_input.map_or_else(
                || Some(format!("SLOWED: physical inputs limit this line to {:.0}% throughput today.", throughput * 100.0)),
                |(commodity, want, have)| Some(format!(
                    "SLOWED: {} limits this line to {:.0}% throughput; full speed needs {:.2}, with {:.2} available before production.",
                    commodity.name(), throughput * 100.0, want, have
                )),
            )
        } else {
            None
        };
        line.ordered_bn += ordered_today;
        line.ordered_today_bn = ordered_today;
        line.throughput_today = throughput;
        if w.rules.resource_gates {
            for commodity in ALL {
                line.resources_used[commodity.idx()] += required[commodity.idx()];
            }
        }
        crate::gdp_projects::record_manufacturing_commitment(w,&opening_line,ordered_today,units);
    }

    w.nation_mut(nation).arsenal.banked = if program_funded { 0.0 } else { banked.min(recurring * 24.0) };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::init::world_1990;
    use crate::production::{ProjectKind, ProvinceCapabilities};
    use crate::world::GameRules;
    use crate::{apply_command, load, save, tick_day, tick_month, Command};

    #[test]
    fn daily_manufacturing_places_a_daily_slice_without_shrinking_forecasts() {
        let (mut w, nation, district) = enabled();
        w.rules.daily_simulation = true;
        let line = start_line(&mut w, nation, &district, "arm_gen3").unwrap();
        let monthly = line_resource_draw(&w, line);
        fill_draw(&mut w, nation, monthly, 12.0);
        let day = tick_allocations(&w, nation)[0].clone();
        assert!((day.budget_bn * 31.0 - budget_bn(&w, nation)).abs() < 1e-12);
        settle_nation(&mut w, nation);
        assert!((w.manufacturing.lines[0].ordered_bn - day.budget_bn).abs() < 1e-12);
        for c in ALL {
            assert!((w.manufacturing.lines[0].resources_used[c.idx()] - day.required[c.idx()]).abs() < 1e-9);
            assert!((line_resource_draw(&w, line)[c.idx()] - monthly[c.idx()]).abs() < 1e-9);
        }
        assert!(w.nation(nation).arsenal.orders[0].due_days.unwrap() > 365);
    }

    #[test]
    fn daily_manufacturing_two_days_stock_is_not_a_monthly_shortage() {
        let (mut w, nation, district) = enabled();
        w.rules.daily_simulation = true;
        let line = start_line(&mut w, nation, &district, "arm_gen3").unwrap();
        let day = tick_allocations(&w, nation)[0].clone();
        fill_draw(&mut w, nation, day.required, 2.0);
        assert!(line_shortfalls(&w, line).iter().any(|gap| *gap > 1e-12),
            "the fixture must be insufficient for the unchanged monthly forecast");
        assert_eq!(tick_line_shortfalls(&w, line), [0.0; 12]);
        settle_nation(&mut w, nation);
        assert_eq!(w.manufacturing.lines[0].status, LineStatus::Producing);
        crate::clock::advance_date(&mut w);
        assert_eq!(tick_line_shortfalls(&w, line), [0.0; 12]);
        settle_nation(&mut w, nation);
        assert_eq!(w.manufacturing.lines[0].status, LineStatus::Producing);
        assert!(tick_line_shortfalls(&w, line).iter().any(|gap| *gap > 1e-12));
    }

    #[test]
    fn daily_shortfalls_include_banked_cash_and_prior_successful_bundles() {
        let (mut w, nation, district) = enabled();
        w.rules.daily_simulation = true;
        let low = start_line(&mut w, nation, &district, "arm_gen2").unwrap();
        let high = start_line(&mut w, nation, &district, "arm_gen3").unwrap();
        set_priority(&mut w, nation, low, Priority::Low).unwrap();
        set_priority(&mut w, nation, high, Priority::High).unwrap();
        w.nation_mut(nation).arsenal.banked = 2.0;
        let plans = tick_allocations(&w, nation);
        let high_plan = plans.iter().find(|p| p.line == high).unwrap();
        let low_plan = plans.iter().find(|p| p.line == low).unwrap();
        assert!((plans.iter().map(|p| p.budget_bn).sum::<f64>()
            - (budget_bn(&w, nation) / 31.0 + 2.0)).abs() < 1e-12);
        fill_draw(&mut w, nation, high_plan.required, 1.0);
        assert_eq!(tick_line_shortfalls(&w, high), [0.0; 12]);
        assert_eq!(tick_line_shortfalls(&w, low), low_plan.required,
            "the earlier successful bundle must reserve its complete stock");
        let high_budget = high_plan.budget_bn;
        let low_budget = low_plan.budget_bn;
        settle_nation(&mut w, nation);
        assert_eq!(w.manufacturing.lines.iter().find(|l| l.id == high).unwrap().status,
            LineStatus::Producing);
        assert_eq!(w.manufacturing.lines.iter().find(|l| l.id == low).unwrap().status,
            LineStatus::Paused);
        assert!((w.nation(nation).arsenal.banked - low_budget).abs() < 1e-12);
        assert!((w.manufacturing.lines.iter().find(|l| l.id == high).unwrap().ordered_bn
            - high_budget).abs() < 1e-12);
    }

    #[test]
    fn tiny_daily_manufacturing_funding_accumulates_instead_of_disappearing() {
        let (mut w, nation, district) = enabled();
        w.rules.daily_simulation = true;
        start_line(&mut w, nation, &district, "inf_light").unwrap();
        let gdp = w.nation(nation).gdp;
        w.nation_mut(nation).mil_spend_gdp = 1.55e-11 * 12.0 / (gdp * arsenal::PROCUREMENT_SHARE);
        let recurring = budget_bn(&w, nation);
        settle_nation(&mut w, nation);
        assert!(w.nation(nation).arsenal.banked > 0.0);
        assert_eq!(w.manufacturing.lines[0].ordered_bn, 0.0);
        for _ in 1..31 {
            crate::clock::advance_date(&mut w);
            settle_nation(&mut w, nation);
        }
        let conserved = w.nation(nation).arsenal.banked + w.manufacturing.lines[0].ordered_bn;
        assert!((conserved - recurring).abs() < 1e-22);
        assert!(w.manufacturing.lines[0].ordered_bn > 0.0);
    }

    fn enabled() -> (WorldState, NationId, String) {
        let mut w = world_1990(GameRules {
            resource_market: true,
            production_system: true,
            manufacturing_system: true,
            ..GameRules::default()
        });
        let nation = NationId::USA;
        w.player = Some(nation);
        w.nation_mut(nation).political_capital = 100.0;
        let district = w
            .districts
            .iter()
            .find_map(|(district, owner)| (*owner == nation).then(|| district.clone()))
            .expect("USA owns a district");
        w.production.provinces.push(ProvinceCapabilities {
            district: district.clone(),
            infrastructure: 0,
            civilian_industry: 0,
            power_grid: 0,
            research_centers: 0,
            arms_plants: 2,
        });
        w.production
            .provinces
            .sort_by(|a, b| a.district.cmp(&b.district));
        resources::warm(&mut w);
        (w, nation, district)
    }

    fn fill_draw(w: &mut WorldState, nation: NationId, draw: [f64; 12], multiplier: f64) {
        for commodity in ALL {
            if draw[commodity.idx()] > 0.0 {
                resources::set_stockpile_for_test(
                    w,
                    nation,
                    commodity,
                    draw[commodity.idx()] * multiplier,
                );
            }
        }
    }

    #[test]
    fn default_world_is_inert_and_serializes_no_manufacturing() {
        let w = world_1990(GameRules::default());
        assert!(!w.rules.manufacturing_system);
        assert!(w.manufacturing.is_empty());
        let body = save(&w);
        assert!(!body.contains("manufacturing_system"));
        assert!(!body.contains("\"manufacturing\""));
    }

    #[test]
    fn enabling_the_system_with_no_lines_preserves_legacy_procurement_exactly() {
        let mut legacy = world_1990(GameRules {
            resource_market: true,
            ..GameRules::default()
        });
        let mut enabled = legacy.clone();
        enabled.rules.manufacturing_system = true;
        tick_month(&mut legacy, &[]);
        tick_month(&mut enabled, &[]);
        // The switch itself is the only serialized difference permitted.
        enabled.rules.manufacturing_system = false;
        assert_eq!(save(&enabled), save(&legacy));
    }

    #[test]
    fn a_line_requires_the_player_owned_plant_and_charges_eight_pc_atomically() {
        let (mut w, nation, district) = enabled();
        let before = w.nation(nation).political_capital;
        let line = apply_command(
            &mut w,
            &Command::StartManufacturingLine {
                nation,
                district: district.clone(),
                kit: "arm_gen3".into(),
            },
        );
        assert!(line.is_ok());
        assert_eq!(w.manufacturing.lines.len(), 1);
        assert_eq!(
            w.nation(nation).political_capital,
            before - START_LINE_PC_COST
        );

        let before_state = w.manufacturing.clone();
        let before_pc = w.nation(nation).political_capital;
        let foreign = w
            .districts
            .iter()
            .find_map(|(d, owner)| (*owner != nation).then(|| d.clone()))
            .unwrap();
        assert!(apply_command(
            &mut w,
            &Command::StartManufacturingLine {
                nation,
                district: foreign,
                kit: "arm_gen3".into(),
            },
        )
        .is_err());
        assert_eq!(w.manufacturing, before_state);
        assert_eq!(w.nation(nation).political_capital, before_pc);
    }

    #[test]
    fn priorities_split_one_budget_and_high_priority_consumes_scarcity_first() {
        let (mut w, nation, district) = enabled();
        let low = start_line(&mut w, nation, &district, "arm_gen2").unwrap();
        let high = start_line(&mut w, nation, &district, "arm_gen3").unwrap();
        set_priority(&mut w, nation, low, Priority::Low).unwrap();
        set_priority(&mut w, nation, high, Priority::High).unwrap();
        let plans = planned_allocations(&w, nation);
        assert_eq!(
            plans.iter().map(|p| p.budget_bn).sum::<f64>(),
            available_bn(&w, nation)
        );
        let high_plan = plans.iter().find(|p| p.line == high).unwrap().clone();
        let low_plan = plans.iter().find(|p| p.line == low).unwrap().clone();
        assert!((high_plan.budget_bn / low_plan.budget_bn - 3.0).abs() < 1e-9);

        // Give the high-priority line its full bundle and leave roughly half a
        // bundle for the low-priority line. Stable priority dispatch gives the
        // first line full speed; the second must continue at partial speed.
        for commodity in ALL {
            let available = high_plan.required[commodity.idx()]
                + low_plan.required[commodity.idx()] * 0.5;
            if available > 0.0 {
                resources::set_stockpile_for_test(&mut w, nation, commodity, available);
            }
        }
        assert!(line_shortfalls(&w, high)
            .iter()
            .all(|shortfall| *shortfall <= 1e-12));
        assert!(line_shortfalls(&w, low)
            .iter()
            .any(|shortfall| *shortfall > 1e-12));
        let stock_before: [f64; 12] = std::array::from_fn(|i| {
            resources::stockpile(&w, nation, Commodity::from_idx(i).unwrap())
        });
        settle_nation(&mut w, nation);
        assert_eq!(
            w.manufacturing
                .lines
                .iter()
                .find(|l| l.id == high)
                .unwrap()
                .status,
            LineStatus::Producing
        );
        assert_eq!(
            w.manufacturing
                .lines
                .iter()
                .find(|l| l.id == low)
                .unwrap()
                .status,
            LineStatus::Slowed
        );
        assert!(w
            .nation(nation)
            .arsenal
            .orders
            .iter()
            .any(|o| o.kit == high_plan.kit));
        assert!(w
            .nation(nation)
            .arsenal
            .orders
            .iter()
            .any(|o| o.kit == low_plan.kit));
        let low_line = w
            .manufacturing
            .lines
            .iter()
            .find(|line| line.id == low)
            .unwrap();
        assert!((low_line.throughput_today - 0.5).abs() < 2e-8);
        assert!((low_line.ordered_today_bn - low_plan.budget_bn * 0.5).abs() < 2e-8);
        for commodity in ALL {
            let closing = resources::stockpile(&w, nation, commodity);
            assert!(
                (stock_before[commodity.idx()]
                    - high_plan.required[commodity.idx()]
                    - low_plan.required[commodity.idx()] * low_line.throughput_today
                    - closing)
                    .abs()
                    < 2e-9
            );
        }
    }

    #[test]
    fn directed_settlement_buys_once_and_writes_no_macro_or_force_quantity() {
        let (mut w, nation, district) = enabled();
        let line = start_line(&mut w, nation, &district, "air_gen4").unwrap();
        let draw = line_resource_draw(&w, line);
        fill_draw(&mut w, nation, draw, 2.0);
        let before = {
            let n = w.nation(nation);
            (
                n.gdp,
                n.debt_gdp,
                n.mil_strength,
                n.munitions,
                n.arsenal.orders.len(),
            )
        };
        let envelope = available_bn(&w, nation);
        settle_nation(&mut w, nation);
        let n = w.nation(nation);
        assert_eq!(
            (n.gdp, n.debt_gdp, n.mil_strength, n.munitions),
            (before.0, before.1, before.2, before.3)
        );
        let new_value: f64 = n.arsenal.orders[before.4..]
            .iter()
            .map(|order| order.units * DECK[order.kit as usize].unit_cost)
            .sum();
        assert!(
            (new_value - envelope).abs() < 1e-9,
            "directed and automatic procurement both fired"
        );
    }

    #[test]
    fn enrolled_lines_spend_only_the_exact_shared_department_residual_once() {
        let (mut w, nation, district) = enabled();
        w.rules.daily_simulation = true;
        w.rules.resource_gates = false;
        w.production.provinces.iter_mut().find(|p|p.district==district).unwrap().arms_plants=3;
        let allocations=w.nation(nation).budget_for(w.year).allocations;
        crate::apply_command(&mut w,&crate::Command::SetProgramBudget {
            nation,fiscal_year:1990,allocations,departments:crate::programs::default_departments(),
        }).unwrap();
        for (kit, priority) in [("arm_gen2",Priority::Low),("arm_gen3",Priority::Normal),("air_gen4",Priority::High)] {
            let line=start_line(&mut w,nation,&district,kit).unwrap();
            set_priority(&mut w,nation,line,priority).unwrap();
        }
        crate::programs::begin_day(&mut w);
        let pool=crate::programs::available_bn(&w,nation,crate::world::BUDGET_DEFENSE,3);
        // A same-day plant project already reserved part of the same pool.
        crate::programs::spend(&mut w,nation,crate::world::BUDGET_DEFENSE,3,pool/3.0).unwrap();
        let residual=crate::programs::available_bn(&w,nation,crate::world::BUDGET_DEFENSE,3);
        let before=w.nation(nation).arsenal.orders.len();
        settle_nation(&mut w,nation);
        assert_eq!(crate::programs::available_bn(&w,nation,crate::world::BUDGET_DEFENSE,3),0.0);
        assert_eq!(w.nation(nation).arsenal.banked,0.0,"unused authority is never duplicated into the old prepaid bank");
        let orders=&w.nation(nation).arsenal.orders[before..];
        assert_eq!(orders.len(),3);
        let bought: f64=orders.iter().map(|o|o.units*DECK[o.kit as usize].unit_cost).sum();
        assert!((bought-residual).abs()<1e-12);
        let order_count=w.nation(nation).arsenal.orders.len();
        settle_nation(&mut w,nation);
        assert_eq!(w.nation(nation).arsenal.orders.len(),order_count,"spent authority cannot place another batch");
    }

    #[test]
    fn disabling_resource_gates_makes_directed_lines_materially_ungated() {
        let (mut w, nation, district) = enabled();
        w.rules.resource_gates = false;
        let line = start_line(&mut w, nation, &district, "air_gen4").unwrap();
        assert!(line_resource_draw(&w, line).iter().any(|need| *need > 0.0));
        assert_eq!(line_shortfalls(&w, line), [0.0; 12]);

        settle_nation(&mut w, nation);

        assert_eq!(w.manufacturing.lines[0].status, LineStatus::Producing);
        assert_eq!(w.manufacturing.lines[0].resources_used, [0.0; 12]);
        assert!(!w.nation(nation).arsenal.orders.is_empty());
    }

    #[test]
    fn save_load_keeps_lines_and_existing_orders_outlive_a_stopped_line() {
        let (mut w, nation, district) = enabled();
        let line = start_line(&mut w, nation, &district, "air_gen2").unwrap();
        let draw = line_resource_draw(&w, line);
        fill_draw(&mut w, nation, draw, 2.0);
        settle_nation(&mut w, nation);
        assert!(!w.nation(nation).arsenal.orders.is_empty());
        let restored = load(&save(&w)).unwrap();
        let mut durable = w.manufacturing.clone();
        for line in &mut durable.lines {
            line.ordered_today_bn = 0.0;
            line.throughput_today = 0.0;
            line.settled_day = None;
        }
        assert_eq!(restored.manufacturing, durable);

        stop_line(&mut w, nation, line).unwrap();
        let orders = w.nation(nation).arsenal.orders.clone();
        assert!(w.manufacturing.lines.is_empty());
        assert_eq!(w.nation(nation).arsenal.orders.len(), orders.len());
        // Exercise the normal monthly owner of delivery semantics; stopping a
        // standing direction does not recall anything already on order.
        tick_month(&mut w, &[]);
        let kept = &orders[0];
        let after = w
            .nation(nation)
            .arsenal
            .orders
            .iter()
            .find(|order| order.kit == kept.kit && order.units >= kept.units)
            .expect("the directed order survived while automatic procurement resumed");
        assert_eq!(after.due, kept.due - 1);
    }

    #[test]
    fn province_loss_blocks_without_consuming_or_ordering() {
        let (mut w, nation, district) = enabled();
        let line = start_line(&mut w, nation, &district, "arm_gen3").unwrap();
        let draw = line_resource_draw(&w, line);
        fill_draw(&mut w, nation, draw, 2.0);
        let stock: [f64; 12] = std::array::from_fn(|i| {
            resources::stockpile(&w, nation, Commodity::from_idx(i).unwrap())
        });
        let foreign = NationId::Canada;
        *w.districts.get_mut(&district).unwrap() = foreign;
        let orders = w.nation(nation).arsenal.orders.len();
        settle_nation(&mut w, nation);
        assert_eq!(w.manufacturing.lines[0].status, LineStatus::Blocked);
        assert_eq!(w.nation(nation).arsenal.orders.len(), orders);
        for commodity in ALL {
            assert_eq!(
                resources::stockpile(&w, nation, commodity),
                stock[commodity.idx()]
            );
        }
    }

    #[test]
    fn two_levels_are_two_slots_not_three() {
        let (mut w, nation, district) = enabled();
        start_line(&mut w, nation, &district, "inf_light").unwrap();
        start_line(&mut w, nation, &district, "arm_gen2").unwrap();
        let before = w.manufacturing.clone();
        assert!(start_line(&mut w, nation, &district, "air_gen2").is_err());
        assert_eq!(w.manufacturing, before);
        assert_eq!(plant_slots(&w, &district), 2);
        assert_eq!(
            production::province_capabilities(&w, &district).level(ProjectKind::ArmsPlant),
            2
        );
    }

    #[test]
    fn a_directed_line_is_identical_under_daily_and_monthly_calendars() {
        let (mut monthly, nation, district) = enabled();
        let line = start_line(&mut monthly, nation, &district, "arm_gen3").unwrap();
        let draw = line_resource_draw(&monthly, line);
        fill_draw(&mut monthly, nation, draw, 24.0);
        let mut daily = monthly.clone();

        tick_month(&mut monthly, &[]);
        let days = crate::world::days_in_month(daily.year, daily.month);
        for _ in 0..days {
            tick_day(&mut daily, &[]);
        }
        assert_eq!(save(&daily), save(&monthly));
    }
}
