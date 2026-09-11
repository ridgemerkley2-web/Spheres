//! Shared fiscal surveillance for explicitly upgraded daily campaigns.
//!
//! This is a reading of the existing cash ledger, not a second bill. Policy
//! commands and the ordinary fiscal settlement still own every dollar. The only
//! new economic consequence is a bounded, slowly accumulated confidence arm in
//! stability. No inflation, credit limit, arrears or civil war is invented here.
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{clock, economy, world::{NationId, WorldState}};

/// MODEL tuning, not historical thresholds: twelve months to adjust inherited
/// policy, then up to two years of continuing deterioration to reach the full
/// confidence arm. At its cap this is -0.05 stability/month, not -0.20/day.
pub const GRACE_MONTHS: u32 = 12;
pub const MAX_CONFIDENCE_PRESSURE: f64 = 0.20;
pub const AFFORDABLE_INTEREST_REVENUE: f64 = 0.30;
const HISTORY_MONTHS: usize = 12;

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct State {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub nations: BTreeMap<NationId, NationFiscal>,
}

impl State {
    pub fn is_empty(&self) -> bool { self.nations.is_empty() }
}

/// Save adoption validates observations without enrolling, settling or filling
/// absent nations. A newly born successor is legitimately absent until prepare.
pub fn validate(w: &WorldState) -> Result<(), String> {
    if w.rules.fiscal_recovery && !clock::is_daily(w) {
        return Err("Fiscal observations require daily campaign accounting.".into());
    }
    if !w.rules.fiscal_recovery && !w.fiscal_recovery.is_empty() {
        return Err("Fiscal observations require their enabled campaign rule.".into());
    }
    let today=clock::absolute_day(w);
    let month=clock::month_index(w);
    let nonnegative=|values:&[f64]| values.iter().all(|v|v.is_finite()&&*v>=0.0);
    if w.rules.fiscal_recovery {
        for n in w.nations.iter().filter(|n|n.alive) {
            if (n.treasury_bn.is_some()||n.debt_bn.is_some())&&!w.fiscal_recovery.nations.contains_key(&n.id) {
                return Err(format!("Open government accounts lack fiscal observations for {}.",n.id.name()));
            }
        }
    }
    for (id,f) in &w.fiscal_recovery.nations {
        let refuse=|reason:&str|format!("Invalid saved fiscal observations for {}: {reason}.",id.name());
        let Some(n)=w.nation_opt(*id) else{return Err(refuse("unknown nation"));};
        let Some(started)=f.started_day.filter(|day|*day<=today) else{return Err(refuse("missing or future enrollment date"));};
        if !nonnegative(&[f.opening_debt_bn,f.opening_treasury_bn,f.confidence_pressure,f.observed_month_fraction])
            || !f.opening_gdp.is_finite() || f.opening_gdp<=0.0
            || f.confidence_pressure>MAX_CONFIDENCE_PRESSURE
            || f.months_observed!=(f.observed_month_fraction+1e-8).floor() as u32
            || f.observations.len()>HISTORY_MONTHS
            || [f.last_day,f.last_ai_review_day,f.last_restructuring_day].into_iter().flatten().any(|day|day<started||day>today)
            || [f.previous_adjustment,f.previous_trend,f.previous_primary].into_iter().flatten().any(|v|!v.is_finite()) {
            return Err(refuse("invalid opening balances, counters or dated review"));
        }
        // A dissolved government retains its dated observations; its current
        // nation row is no longer an open public account. Stored cash/debt in
        // the historical observations still receives the full validation below.
        if n.alive && (n.treasury_bn.is_none_or(|cash|!cash.is_finite()||cash<0.0)
            || n.debt_bn.is_none_or(|debt|!debt.is_finite()||debt<0.0)) {
            return Err(refuse("living government has invalid current cash or debt"));
        }
        let valid_receipt=|r:&Receipt| r.day>=started && r.day<=today
            && r.revenue_bn.is_finite() && r.interest_bn.is_finite() && nonnegative(&[r.spending_bn]);
        if f.legacy_ordinary_receipt.as_ref().is_some_and(|legacy|
            !valid_receipt(&legacy.receipt) || legacy.receipt.day>legacy.observed_through_day
                || f.last_day.is_none_or(|last|legacy.observed_through_day>last)) {
            return Err(refuse("invalid retained original-master receipt"));
        }
        if let Some(receipt)=&f.pending {
            let retained=f.legacy_ordinary_receipt.as_ref().is_some_and(|legacy|legacy.receipt==*receipt);
            if receipt.day<started || receipt.day>today || (f.last_day.is_some_and(|last|receipt.day<=last)&&!retained)
                || !receipt.revenue_bn.is_finite() || !receipt.interest_bn.is_finite()
                || !nonnegative(&[receipt.spending_bn]) {
                return Err(refuse("invalid or already observed pending receipt"));
            }
        }
        let valid_observation=|o:&Observation| {
            o.month<=month && o.opening_gdp.is_finite() && o.opening_gdp>0.0
                && o.closing_gdp.is_finite() && o.closing_gdp>0.0
                && o.revenue_bn.is_finite() && o.interest_bn.is_finite()
                && nonnegative(&[o.opening_debt_bn,o.closing_debt_bn,o.opening_treasury_bn,o.closing_treasury_bn,
                    o.spending_bn,o.gdp_years,o.tax_gdp_years,o.year_fraction])
                && o.gdp_years>0.0 && o.year_fraction>0.0 && o.year_fraction<=1.0/12.0+1e-10
        };
        if f.observations.iter().any(|o|!valid_observation(o)) {
            return Err(refuse("invalid closed observation values or exposure"));
        }
        if f.accumulating.as_ref().is_some_and(|o|!valid_observation(o)) {
            return Err(refuse("invalid partial observation values or exposure"));
        }
        if f.observations.windows(2).any(|rows|rows[0].month>=rows[1].month)
            || f.observations.last().is_some_and(|o|o.month==month&&!clock::month_end(w))
            || f.accumulating.as_ref().is_some_and(|o|!valid_observation(o)
                || f.observations.last().is_some_and(|last|o.month<=last.month))
            || ((!f.observations.is_empty()||f.accumulating.is_some())&&f.last_day.is_none())
            || f.observations.iter().map(|o|o.year_fraction*12.0).sum::<f64>()>f.observed_month_fraction+1e-8 {
            return Err(refuse("inconsistent closed and partial observation dates or totals"));
        }
    }
    Ok(())
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct NationFiscal {
    pub started_day: Option<i32>,
    pub last_day: Option<i32>,
    pub last_ai_review_day: Option<i32>,
    pub last_restructuring_day: Option<i32>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub last_ai_action: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub last_ai_reason: String,
    pub months_observed: u32,
    pub observed_month_fraction: f64,
    pub stress_months: u32,
    pub improving_months: u32,
    pub confidence_pressure: f64,
    pub observations: Vec<Observation>,
    pub accumulating: Option<Observation>,
    /// An ordinary non-program fiscal settlement posts exactly one receipt.
    pub pending: Option<Receipt>,
    /// Exact historical artifact from the recognized original-master decoder.
    /// Its observer kept an old ordinary receipt when a program budget took
    /// over. Retain the evidence, without claiming it was observed or paying it
    /// again. `tick` may drain `pending`; this copy remains after continuation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_ordinary_receipt: Option<LegacyOrdinaryReceipt>,
    pub previous_adjustment: Option<f64>,
    pub previous_trend: Option<f64>,
    pub previous_primary: Option<f64>,
    /// Opening values are recorded before the first daily economic settlement.
    pub opening_gdp: f64,
    pub opening_debt_bn: f64,
    pub opening_treasury_bn: f64,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Receipt {
    pub day: i32,
    pub revenue_bn: f64,
    pub spending_bn: f64,
    /// The existing real-rate fiscal model can post negative net interest.
    /// Observation must retain that exact signed amount, never clamp or reprice.
    pub interest_bn: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct LegacyOrdinaryReceipt {
    pub receipt: Receipt,
    /// Original last observed day, not a new observation or import timestamp.
    pub observed_through_day: i32,
}

/// Called only after the save classifier recognizes the original master
/// dialect. Master's program branch never drained `pending`, leaving ordinary
/// receipts behind after budget enrollment. Add provenance without changing
/// any original receipt, date, observation, quantity, balance or RNG state.
/// The normal validator still rejects malformed values and all other stale
/// pending receipts. No simulation system, payment or observation runs here.
pub(crate) fn retain_original_master_receipts(w: &mut WorldState) -> Result<(), String> {
    if !enabled(w) { return Ok(()); }
    for (id, f) in &mut w.fiscal_recovery.nations {
        if f.legacy_ordinary_receipt.is_some() {
            return Err("The original master fiscal dialect cannot contain later receipt provenance.".into());
        }
        let Some(receipt) = f.pending.as_ref() else { continue; };
        let Some(last) = f.last_day.filter(|last|receipt.day<=*last) else { continue; };
        // The source program observer, rather than the ordinary branch, must
        // own the most recently observed receipt. This is the old code's exact
        // stale-pending pattern; a free-standing corrupt pending row is refused.
        if w.nations.iter().find(|n|n.id==*id).and_then(|n|n.program_budget.as_ref())
            .is_some_and(|p|p.settled_day==Some(last)) {
            f.legacy_ordinary_receipt=Some(LegacyOrdinaryReceipt {
                receipt:receipt.clone(), observed_through_day:last,
            });
        }
    }
    Ok(())
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Observation {
    pub month: i32,
    pub opening_gdp: f64,
    pub opening_debt_bn: f64,
    #[serde(default)]
    pub opening_treasury_bn: f64,
    pub closing_gdp: f64,
    pub closing_debt_bn: f64,
    #[serde(default)]
    pub closing_treasury_bn: f64,
    pub revenue_bn: f64,
    pub spending_bn: f64,
    pub interest_bn: f64,
    /// GDP times the portion of a year actually observed. Dividing cash by
    /// this exposure annualizes partial months without assuming 30-day months.
    pub gdp_years: f64,
    pub year_fraction: f64,
    /// Reporting basis used to distinguish a tax change already enacted from
    /// the trailing cash window; never another source of tax receipts.
    #[serde(default)]
    pub tax_gdp_years: f64,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Status { Stable, Watch, Adjustment, Recovering, Crisis }

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct Assessment {
    pub debt_gdp: f64,
    /// Annualized observed ratio change; not a nominal cash-debt change.
    pub annual_debt_change_gdp: f64,
    pub primary_balance_gdp: f64,
    pub required_primary_balance_gdp: f64,
    pub adjustment_needed_gdp: f64,
    /// Latest posted month's cash primary balance, with an already-enacted tax
    /// change applied once. AI uses this to avoid repeating a stale adjustment.
    pub current_policy_primary_balance_gdp: f64,
    pub current_policy_adjustment_gdp: f64,
    /// Conservative recurring cash requirement outside the ministry receipt:
    /// three consecutive positive residuals, or at least three positive months
    /// over nine observed months. The smallest payment rate caps larger spikes.
    pub other_obligations_gdp: f64,
    pub interest_gdp: f64,
    /// None means no positive observed revenue, not zero debt service.
    pub interest_revenue: Option<f64>,
    pub revenue_gdp: f64,
    pub spending_gdp: f64,
    pub projected_debt_gdp_5y: f64,
    pub annual_real_growth: f64,
    pub months_observed: u32,
    pub grace_months_remaining: u32,
    pub stress_months: u32,
    pub improving_months: u32,
    pub recovery_required: bool,
    pub status: Status,
    pub status_label: String,
    pub reason: String,
    pub next_action: String,
    pub confidence_pressure: f64,
    pub stability_change_per_month: f64,
    pub forecast_assumptions: String,
}

pub fn enabled(w: &WorldState) -> bool {
    w.rules.fiscal_recovery && clock::is_daily(w)
}

pub fn enable(w: &mut WorldState) {
    w.rules.fiscal_recovery = true;
    prepare(w);
}

/// All countries use the same existing stock accounting once enabled. A save
/// upgrade creates no cash and waives no debt. An already open treasury keeps
/// its actual cash. This also seats newly born successors before their first
/// ordinary economic tick; it neither changes their authored debt ratio nor
/// carries over the predecessor government's adjustment clock.
pub fn prepare(w: &mut WorldState) {
    if !enabled(w) { return; }
    let today = clock::absolute_day(w);
    for n in w.nations.iter_mut().filter(|n| n.alive) {
        n.treasury_bn.get_or_insert(0.0);
        n.debt_bn.get_or_insert_with(|| n.debt_gdp * n.gdp);
        let f = w.fiscal_recovery.nations.entry(n.id).or_default();
        if f.started_day.is_none() {
            f.started_day = Some(today);
            f.opening_gdp = n.gdp;
            f.opening_debt_bn = n.debt_bn.unwrap_or(0.0);
            f.opening_treasury_bn = n.treasury_bn.unwrap_or(0.0);
        }
    }
}

/// Called with the exact amounts the ordinary non-program fiscal branch paid.
/// Repeated calls for the same date replace the receipt; they cannot accrue a
/// second bill or a second day of surveillance. Program nations read their own
/// posted receipt in `tick`, after `programs::finish_day`.
pub fn record_fiscal(state: &mut State, nation: NationId, day: i32,
    revenue_bn: f64, spending_bn: f64, interest_bn: f64)
{
    let Some(f) = state.nations.get_mut(&nation) else { return; };
    if f.last_day.is_some_and(|last| day <= last) { return; }
    f.pending = Some(Receipt { day, revenue_bn, spending_bn, interest_bn });
}

/// Consume posted cash receipts once, and change confidence only on a closed
/// calendar month. Call after programs::finish_day, company receivable settlement
/// and the remaining paid daily tail, before advancing the date. Company sales,
/// capitalization and refit invoices already belong to the programme receipt;
/// they are not additional spending here. Closing cash includes actual tail
/// payments/refunds. Nothing reads unused authorization as an expense.
pub fn tick(w: &mut WorldState) {
    if !enabled(w) { return; }
    let today = clock::absolute_day(w);
    let month = clock::month_index(w);
    let year_fraction = clock::year_fraction(w);
    let close_month = clock::month_end(w);
    let mut closed = Vec::new();
    for n in w.nations.iter_mut().filter(|n| n.alive) {
        // Coups, peace and other later systems may change GDP after economy
        // settled the bill. The stock remains owed; publish its ratio against
        // the final denominator before surveillance or the browser reads it.
        economy::refresh_debt_ratio(n);
        let Some(f) = w.fiscal_recovery.nations.get_mut(&n.id) else { continue; };
        // Remove obsolete ordinary receipts even after budget enrollment or an
        // already observed date. Keep future-dated corruption for validation to
        // reject; neither a skipped date nor a duplicate becomes today's bill.
        let pending = if f.pending.as_ref().is_some_and(|r|r.day<=today) {
            f.pending.take()
        } else { None };
        if f.last_day.is_some_and(|last| today <= last) { continue; }
        let receipt = if let Some(p) = n.program_budget.as_ref().filter(|p|p.settled_day==Some(today)) {
            Receipt { day: today, revenue_bn: p.revenue_today_bn,
                spending_bn: p.spent_today_bn.iter().flatten().sum(),
                interest_bn: p.interest_today_bn }
        } else {
            // An ordinary bill may have been paid before a late daily system
            // first installed a program budget. That new plan has no settled
            // day yet; its mere presence cannot hide the actual paid receipt.
            let Some(receipt) = pending.filter(|r| r.day == today) else { continue; };
            receipt
        };
        if f.accumulating.as_ref().is_some_and(|o| o.month != month) {
            // A direct date jump is not a month of paid services. Keep its
            // partial receipt, but never fabricate observations for skipped time.
            finish_observation(f);
        }
        let (opening_gdp, opening_debt_bn, opening_treasury_bn) = f.observations.last()
            .map_or((f.opening_gdp, f.opening_debt_bn, f.opening_treasury_bn),
                |o| (o.closing_gdp, o.closing_debt_bn, o.closing_treasury_bn));
        let o = f.accumulating.get_or_insert_with(|| Observation {
            month, opening_gdp, opening_debt_bn, opening_treasury_bn, ..Observation::default()
        });
        o.closing_gdp = n.gdp;
        o.closing_debt_bn = n.debt_bn.unwrap_or(n.debt_gdp * n.gdp);
        o.closing_treasury_bn = n.treasury_bn.unwrap_or(0.0);
        o.revenue_bn += receipt.revenue_bn;
        o.spending_bn += receipt.spending_bn;
        o.interest_bn += receipt.interest_bn;
        // The cash receipt already carries its actual opening GDP basis; GDP
        // exposure uses the current close, a disclosed reporting annualization.
        o.gdp_years += n.gdp * year_fraction;
        o.tax_gdp_years += n.tax_rate * n.gdp * year_fraction;
        o.year_fraction += year_fraction;
        f.last_day = Some(today);
        if close_month {
            finish_observation(f);
            closed.push(n.id);
        }
    }
    for id in closed {
        let Some(a) = assessment(w, id) else { continue; };
        let f = w.fiscal_recovery.nations.get_mut(&id).unwrap();
        let actual_improvement = f.previous_primary.is_some_and(|primary|
            a.primary_balance_gdp >= primary - 0.001)
            && (f.previous_adjustment.is_some_and(|gap| a.adjustment_needed_gdp < gap - 0.001)
                || f.previous_trend.is_some_and(|trend| a.annual_debt_change_gdp < trend - 0.002));
        if !a.recovery_required {
            f.improving_months = f.improving_months.saturating_add(1);
            if f.improving_months >= 3 {
                f.confidence_pressure = (f.confidence_pressure - 0.01).max(0.0);
                f.stress_months = f.stress_months.saturating_sub(2);
            }
        } else if actual_improvement {
            f.improving_months = f.improving_months.saturating_add(1);
            // Posted improvement earns breathing room. A declaration or a cut
            // in unused authority does not enter any of these comparisons.
            if f.improving_months >= 3 {
                f.confidence_pressure = (f.confidence_pressure - 0.002).max(0.0);
            }
        } else {
            f.improving_months = 0;
            f.stress_months = f.stress_months.saturating_add(1);
            if f.months_observed > GRACE_MONTHS {
                let interest_stress = a.interest_revenue.unwrap_or(if a.interest_gdp > 0.0 { 1.0 } else { 0.0 });
                let severity = (a.adjustment_needed_gdp / 0.05)
                    .max((interest_stress - AFFORDABLE_INTEREST_REVENUE) / 0.30)
                    .max(0.10).clamp(0.0, 1.0);
                let pace = severity * MAX_CONFIDENCE_PRESSURE / 24.0;
                f.confidence_pressure = (f.confidence_pressure + pace).min(MAX_CONFIDENCE_PRESSURE);
            }
        }
        f.previous_adjustment = Some(a.adjustment_needed_gdp);
        f.previous_trend = Some(a.annual_debt_change_gdp);
        f.previous_primary = Some(a.primary_balance_gdp);
    }
}

fn finish_observation(f: &mut NationFiscal) {
    let Some(o) = f.accumulating.take() else { return; };
    if o.year_fraction <= 0.0 { return; }
    f.observed_month_fraction += o.year_fraction * 12.0;
    f.observations.push(o);
    if f.observations.len() > HISTORY_MONTHS { f.observations.remove(0); }
    f.months_observed = (f.observed_month_fraction + 1e-8).floor() as u32;
}

/// Negative raw pressure, applied once through economy's existing stability
/// flow. Service cuts, unemployment and inflation retain their existing owners.
pub fn stability_pressure(w: &WorldState, nation: NationId) -> f64 {
    if !enabled(w) { return 0.0; }
    -w.fiscal_recovery.nations.get(&nation).map_or(0.0, |f| f.confidence_pressure)
        .clamp(0.0, MAX_CONFIDENCE_PRESSURE)
}

/// Pure common reading for player, AI, Treasury and testing. The trailing cash
/// window is deliberately separate from the projected constant-policy path.
pub fn assessment(w: &WorldState, nation: NationId) -> Option<Assessment> {
    if !enabled(w) { return None; }
    let n = w.nation_opt(nation).filter(|n| n.alive && n.gdp > 0.0)?;
    let state = w.fiscal_recovery.nations.get(&nation);
    let rows: Vec<&Observation> = state.into_iter().flat_map(|f| f.observations.iter()).collect();
    let exposure: f64 = rows.iter().map(|o| o.gdp_years).sum();
    let years: f64 = rows.iter().map(|o| o.year_fraction).sum();
    let terms = economy::growth_terms(n, n.state_invest_gdp, n.interest_rate, &economy::Conditions::of(w, nation));
    let fiscal = economy::Fiscal::of(n, &terms);
    let (revenue, spending, interest, trend, growth) = if exposure > 0.0 && years > 0.0 {
        let first = rows.first().unwrap();
        let last = rows.last().unwrap();
        let first_ratio = first.opening_debt_bn / first.opening_gdp;
        let last_ratio = last.closing_debt_bn / last.closing_gdp;
        let observed_growth = crate::exact::powf(last.closing_gdp / first.opening_gdp, 1.0 / years) - 1.0;
        (rows.iter().map(|o| o.revenue_bn).sum::<f64>() / exposure,
         rows.iter().map(|o| o.spending_bn).sum::<f64>() / exposure,
         rows.iter().map(|o| o.interest_bn).sum::<f64>() / exposure,
         (last_ratio - first_ratio) / years, observed_growth)
    } else {
        // Before the first closed month this is a provisional run-rate, not a
        // claim that the annual authorization has already been spent.
        (fiscal.revenue_gdp, fiscal.spend_gdp, fiscal.interest_gdp,
         -fiscal.balance_gdp - n.growth_last * n.debt_gdp, n.growth_last)
    };
    // A one-month rebound should not promise exponential catch-up for five
    // years. These are disclosed scenario bounds, not writes to actual GDP.
    let growth = growth.clamp(-0.10, 0.08);
    let primary = revenue - spending;
    let recent: Vec<_> = rows.iter().rev().take(3).collect();
    let consecutive_obligations = if recent.len() == 3
        && recent.windows(2).all(|pair| pair[0].month == pair[1].month + 1)
    {
        recent.iter().map(|o| {
            let net_borrowing = o.closing_debt_bn - o.closing_treasury_bn
                - o.opening_debt_bn + o.opening_treasury_bn;
            let ordinary_deficit = o.spending_bn + o.interest_bn - o.revenue_bn;
            if o.gdp_years > 0.0 { ((net_borrowing - ordinary_deficit) / o.gdp_years).max(0.0) } else { 0.0 }
        }).fold(f64::INFINITY, f64::min)
    } else { 0.0 };
    // Quarterly procurement/imports are ordinary cash obligations even when
    // the intervening months run a surplus. After nine paid months, recognize
    // at least three positive residual months. Cap every payment at the
    // smallest observed positive rate so one exceptional purchase cannot set
    // the standing bill. A lone purchase never meets this recurrence test;
    // negative one-offs (write-downs/asset proceeds) never subsidize it.
    // This is a disclosed recurrence heuristic, not knowledge of future orders.
    let positive: Vec<_> = rows.iter().filter_map(|o| {
        let net_borrowing = o.closing_debt_bn - o.closing_treasury_bn
            - o.opening_debt_bn + o.opening_treasury_bn;
        let ordinary_deficit = o.spending_bn + o.interest_bn - o.revenue_bn;
        let rate = if o.gdp_years > 0.0 { (net_borrowing - ordinary_deficit) / o.gdp_years } else { 0.0 };
        // Ignore rounding dust: one part per million of annual GDP is not a
        // separate funded obligation and must not erase the minimum real bill.
        (rate > 1e-6).then_some((rate, o.gdp_years))
    }).collect();
    let periodic_obligations = if years + 1e-8 >= 0.75 && positive.len() >= 3 {
        positive.iter().map(|(rate, _)| *rate).fold(f64::INFINITY, f64::min)
            * positive.iter().map(|(_, paid_exposure)| *paid_exposure).sum::<f64>() / exposure
    } else { 0.0 };
    let other_obligations = consecutive_obligations.max(periodic_obligations);
    let interest_revenue = (revenue > 0.0).then_some(interest.max(0.0) / revenue);
    let burden = interest_revenue.unwrap_or(if interest > 0.0 { 1.0 } else { 0.0 });
    let forecast = project(n.debt_gdp, n.treasury_bn.unwrap_or(0.0) / n.gdp,
        primary - other_obligations, growth, n.interest_rate, n.inflation);
    let projected_rising = forecast > n.debt_gdp + 0.10 && (n.debt_gdp > 0.60 || forecast > 0.90);
    let months = state.map_or(0, |f| f.months_observed);
    let sustained_ratio_rise = recent.len() == 3 && recent.iter().all(|o|
        o.opening_gdp > 0.0 && o.closing_gdp > 0.0
            && o.closing_debt_bn / o.closing_gdp - o.opening_debt_bn / o.opening_gdp
                > 0.03 * o.year_fraction);
    let observed_rising = months >= 3 && trend > 0.03 && sustained_ratio_rise
        && (n.debt_gdp > 0.60 || n.debt_gdp + trend * 5.0 > 0.90);
    let rising = projected_rising || observed_rising;
    let expensive = burden > AFFORDABLE_INTEREST_REVENUE && n.debt_gdp > 0.0;
    let recovery_required = rising || expensive;
    let debt_paydown = if expensive { ((burden - AFFORDABLE_INTEREST_REVENUE) * 0.10).clamp(0.0, 0.05) } else { 0.0 };
    let required_primary = fiscal.interest_gdp - growth * n.debt_gdp + debt_paydown + other_obligations;
    let adjustment = (required_primary - primary).max(0.0);
    let current_primary = rows.last().filter(|o| o.gdp_years > 0.0).map_or(
        fiscal.revenue_gdp - fiscal.spend_gdp,
        |o| (o.revenue_bn - o.spending_bn) / o.gdp_years
            + n.tax_rate - o.tax_gdp_years / o.gdp_years);
    let current_adjustment = (required_primary - current_primary).max(0.0);
    let grace = GRACE_MONTHS.saturating_sub(months);
    let pressure = -stability_pressure(w, nation);
    let improving = state.map_or(0, |f| f.improving_months);
    // Improving cash results do not make a still-severe burden affordable.
    // Keep that condition visible; these presentation fields do not drive
    // confidence settlement or the AI's priced policy proposals.
    let severe_crisis = recovery_required && (burden > 0.50 || adjustment > 0.05);
    let status = if months == 0 || (recovery_required && grace > 0) { Status::Watch }
        else if severe_crisis { Status::Crisis }
        else if improving >= 3 && (recovery_required || pressure > 0.0) { Status::Recovering }
        else if recovery_required { Status::Adjustment }
        else if pressure > 0.0 { Status::Recovering }
        else { Status::Stable };
    let label = match status { Status::Stable => "Debt under control", Status::Watch => "Fiscal watch",
        Status::Adjustment => "Recovery required", Status::Recovering => "Recovery in progress",
        Status::Crisis => "Fiscal confidence crisis" };
    let reason = if months == 0 {
        "Provisional outlook: a month of actual cash receipts is still being collected.".to_string()
    } else if expensive && rising {
        "Interest consumes an unaffordable share of revenue and the unchanged cash budget keeps the debt ratio rising.".to_string()
    } else if expensive {
        "Interest consumes more than 30% of revenue; falling debt still needs time to restore room for public services.".to_string()
    } else if projected_rising {
        "The unchanged cash budget projects a materially rising debt ratio over five years.".to_string()
    } else if observed_rising {
        "The observed debt ratio keeps rising despite the operating budget outlook. Review other cash obligations, transfers and recent financing changes.".to_string()
    } else {
        "The debt path is broadly stable or falling and interest remains affordable. A high debt ratio alone adds no confidence penalty.".to_string()
    };
    let next_action = if months == 0 {
        "Review tax and actual spending, then let a monthly cash report close. Unused construction authority is not a paid expense."
    } else if !recovery_required {
        "Maintain the sustainable cash balance and protect essential services and productive maintenance."
    } else if status == Status::Crisis && improving >= 3 {
        "Recent cash results have improved, but severe fiscal stress remains. Keep effective measures in place and review whether further adjustment is needed; available Cabinet recovery actions, including debt restructuring, still carry their political and diplomatic costs."
    } else if improving >= 3 {
        "Continue the measures that improved the posted cash balance; confidence recovers as results persist."
    } else if n.debt_gdp > 1.10 && burden > 0.50 {
        "Phase in revenue or actual-spending changes; review the existing debt restructuring Cabinet action if its political and diplomatic costs are acceptable."
    } else {
        "Phase in tax or actual-spending changes. Protect essential services and maintenance; reducing unused authorization creates no cash savings."
    };
    Some(Assessment {
        debt_gdp: n.debt_gdp, annual_debt_change_gdp: trend, primary_balance_gdp: primary,
        required_primary_balance_gdp: required_primary, adjustment_needed_gdp: adjustment,
        current_policy_primary_balance_gdp: current_primary,
        current_policy_adjustment_gdp: current_adjustment,
        other_obligations_gdp: other_obligations,
        interest_gdp: interest, interest_revenue, revenue_gdp: revenue, spending_gdp: spending,
        projected_debt_gdp_5y: forecast, annual_real_growth: growth, months_observed: months,
        grace_months_remaining: grace, stress_months: state.map_or(0, |f| f.stress_months),
        improving_months: improving, recovery_required, status, status_label: label.into(),
        reason, next_action: next_action.into(), confidence_pressure: pressure,
        stability_change_per_month: -pressure * 0.25,
        forecast_assumptions: "Illustrative five-year path: hold the observed primary cash balance and policy rate; real GDP growth is bounded to -10%..8%/year and the existing sovereign premium changes with debt. Other cash obligations use a recurrence heuristic: three consecutive positive monthly financing residuals, or at least three positive months across nine observed months. The smallest positive payment rate caps larger spikes; periodic bills retain their observed frequency. Cash and debt are billions of 1990 dollars, so inflation is not subtracted again. One-off transfers, asset sales and write-downs are not recurring budget revenue. Borrowing has no credit limit; this gauge does not declare arrears or default.".into(),
    })
}

/// Sixty monthly steps through the existing real-rate and cash-first financing
/// rules. No universal debt ceiling; a growing denominator can stabilize a
/// modest primary deficit, and a cash reserve can finance a temporary deficit.
pub fn project(mut debt: f64, mut cash: f64, primary: f64, growth: f64,
    policy_rate: f64, inflation: f64) -> f64
{
    let growth_factor = crate::exact::powf(1.0 + growth, 1.0 / 12.0);
    for _ in 0..60 {
        let interest = debt * economy::effective_interest_rate(policy_rate, inflation, debt);
        let bill = (interest - primary) / 12.0;
        if bill >= 0.0 {
            let paid_cash = cash.min(bill);
            cash -= paid_cash;
            debt += bill - paid_cash;
        } else {
            let retired = debt.min(-bill);
            debt -= retired;
            cash += -bill - retired;
        }
        debt /= growth_factor;
        cash /= growth_factor;
    }
    debt
}
