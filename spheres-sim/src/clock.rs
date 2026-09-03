//! Deterministic model time. Daily play accrues one calendar month's quantities
//! over its actual days; annual rates still sum to twelve equal monthly shares.
//! Legacy saves keep their old monthly model until daily play is enabled.
use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use crate::world::{days_in_month, NationId, WorldState};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct DailyState {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub economic_shocks: BTreeMap<NationId, f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oil_shock: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shock_month: Option<i32>,
    /// Fractional model-month ages for old integer reporting counters.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub counters: BTreeMap<String, f64>,
    /// Old mid-month saves have not posted any of that month's flows yet.
    /// Finish their one pending monthly settlement before changing integrator.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activate_after_month: Option<i32>,
}
impl DailyState {
    pub fn is_empty(&self) -> bool {
        self.economic_shocks.is_empty() && self.oil_shock.is_none()
            && self.shock_month.is_none() && self.counters.is_empty()
            && self.activate_after_month.is_none()
    }
}
pub fn is_daily(w: &WorldState) -> bool { w.rules.daily_simulation }
pub fn month_fraction(w: &WorldState) -> f64 {
    if is_daily(w) { 1.0 / days_in_month(w.year, w.month) as f64 } else { 1.0 }
}
pub fn year_fraction(w: &WorldState) -> f64 { month_fraction(w) / 12.0 }
pub fn blend(w: &WorldState, rate: f64) -> f64 {
    if !is_daily(w) { rate } else { 1.0 - crate::exact::powf(1.0 - rate.clamp(0.0, 1.0), month_fraction(w)) }
}
pub fn decay(w: &WorldState, factor: f64) -> f64 {
    if !is_daily(w) { factor } else { crate::exact::powf(factor.max(0.0), month_fraction(w)) }
}
pub fn chance(w: &WorldState, probability: f64) -> f64 { blend(w, probability) }
pub fn month_end(w: &WorldState) -> bool { !is_daily(w) || w.day >= days_in_month(w.year, w.month) }
pub fn month_index(w: &WorldState) -> i32 { (w.year - 1990) * 12 + w.month as i32 - 1 }

pub fn enable_daily_play(w: &mut WorldState) {
    if is_daily(w) { return; }
    if w.day > 1 { w.daily.activate_after_month = Some(month_index(w)); }
    else { w.rules.daily_simulation = true; w.daily.activate_after_month = None; }
}

pub fn finish_pending_transition(w: &mut WorldState) {
    if w.daily.activate_after_month.is_some_and(|month| month_index(w) > month) {
        w.rules.daily_simulation = true;
        w.daily.activate_after_month = None;
    }
}

/// Gregorian day number relative to January 1, 1990 (no wall clock).
pub fn date_day(year: i32, month: u32, day: u32) -> i32 {
    let y = year - i32::from(month <= 2);
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let m = month as i32 + if month > 2 { -3 } else { 9 };
    let doy = (153 * m + 2) / 5 + day as i32 - 1;
    era * 146_097 + yoe * 365 + yoe / 4 - yoe / 100 + doy - 726_773
}
pub fn absolute_day(w: &WorldState) -> i32 { date_day(w.year, w.month, w.day.max(1)) }
pub fn days_for_months(w: &WorldState, months: u32) -> u32 {
    let index = w.year as i64 * 12 + w.month as i64 - 1 + months as i64;
    let year = index.div_euclid(12) as i32;
    let month = index.rem_euclid(12) as u32 + 1;
    let day = w.day.max(1).min(days_in_month(year, month));
    (date_day(year, month, day) - absolute_day(w)).max(0) as u32
}
pub fn advance_date(w: &mut WorldState) {
    if w.day >= days_in_month(w.year, w.month) {
        w.day = 1;
        w.month += 1;
        if w.month > 12 { w.month = 1; w.year += 1; }
    } else { w.day += 1; }
}
/// Carry sub-month age without changing the legacy schema of public counters.
pub fn advance_counter(w: &mut WorldState, key: String, current: u32) -> u32 {
    if !is_daily(w) { return current.saturating_add(1); }
    let dt = month_fraction(w);
    let age = w.daily.counters.entry(key).or_insert(current as f64);
    if (*age + 1e-9).floor() as u32 != current { *age = current as f64; }
    *age += dt;
    (*age + 1e-9).floor() as u32
}

/// A paced action, not a probability: withdrawals still take one model month
/// per rung while their schedule is advanced every day.
pub fn interval_due(w: &mut WorldState, key: String, months: f64) -> bool {
    if !is_daily(w) { return true; }
    let dt = month_fraction(w);
    let age = w.daily.counters.entry(key).or_default();
    *age += dt;
    if *age + 1e-9 >= months { *age = (*age - months).max(0.0); true } else { false }
}

pub fn date_from_day(day: i32) -> (i32, u32, u32) {
    let z = day + 726_773;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe/1460 + doe/36524 - doe/146096) / 365;
    let mut y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe/4 - yoe/100);
    let mp = (5*doy+2)/153;
    let d = doy - (153*mp+2)/5 + 1;
    let m = mp + if mp < 10 { 3 } else { -9 };
    y += i32::from(m <= 2);
    (y, m as u32, d as u32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::init::world_1990;
    use crate::world::GameRules;
    #[test]
    fn calendar_and_leap_days_are_exact() {
        assert_eq!(date_day(1990,1,1), 0);
        assert_eq!(date_day(1990,2,1),31);
        assert_eq!(date_day(2000,3,1)-date_day(2000,2,1),29);
        assert_eq!(date_day(2100,3,1)-date_day(2100,2,1),28);
    }
    #[test]
    fn daily_rates_sum_to_a_year_without_multiplying_hazards() {
        let mut w = world_1990(GameRules { daily_simulation:true, ..GameRules::default() });
        w.year=2000;
        let mut years=0.0;
        for _ in 0..366 { years+=year_fraction(&w); advance_date(&mut w); }
        assert!((years-1.0).abs()<1e-12);
        let retention=1.0-chance(&w,0.15);
        assert!((crate::exact::powf(retention,31.0)-0.85).abs()<1e-12);
    }
}
