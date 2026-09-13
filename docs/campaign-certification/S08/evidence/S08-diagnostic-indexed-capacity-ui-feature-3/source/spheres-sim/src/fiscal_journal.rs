//! Dated explanations of existing government money movements. This book never
//! pays, borrows, reprices policy, or supplies an input to fiscal AI. Recording
//! starts prospectively at a real payment or successful policy decision.
use crate::{clock, fiscal_recovery, programs, world::*, Command};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const RETAINED_DAYS: usize = 45;
pub const RETAINED_DECISIONS: usize = 12;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum CashCause {
    BudgetSettlement,
    ResourceMarket,
    ResourceTransfer,
    GoodsImportEscrow,
    GoodsExportReceipt,
    GoodsRefund,
    FreightService,
    PactUpkeep,
    ForeignAid,
    CovertOperation,
    Patronage,
    SupplierInputs,
    RefitRefund,
    EquipmentImportRefund,
    DebtRestructuring,
    AssetSale,
    LegacyMineConstruction,
    Other,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct MoneyFlow {
    pub postings: u64,
    /// Actual net payment legs, separated by sign. A net budget posting's gross
    /// tax, service and interest components are in MoneyDay.fiscal instead.
    pub inflow_bn: f64,
    pub outflow_bn: f64,
    pub treasury_delta_bn: f64,
    pub debt_delta_bn: f64,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct FiscalDetail {
    pub revenue_bn: f64,
    pub spending_bn: f64,
    /// The existing real-interest model can post a negative interest charge.
    pub interest_bn: f64,
    pub departments_bn: Option<programs::Amounts>,
    /// Old, already-paid authority consumed today; excluded from spending_bn.
    pub prepaid_used_bn: f64,
    pub construction_bn: f64,
    /// Actual supplier receipts settled under this fiscal day's appropriation.
    /// These are inclusions, NOT new government cash legs. A receipt may use
    /// prepaid authority; the day's separate prepaid total makes that explicit.
    pub supplier_inclusions_bn: BTreeMap<SupplierInclusion, f64>,
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum SupplierInclusion {
    Capitalization,
    Development,
    Equipment,
    Ammunition,
    RefitEscrow,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct MoneyDay {
    pub day: i32,
    pub opening_treasury_bn: f64,
    pub opening_debt_bn: f64,
    pub closing_treasury_bn: f64,
    pub closing_debt_bn: f64,
    /// Observed balance changes outside the tagged payment owners. Never
    /// relabel these as revenue or silently force the reconciliation to zero.
    pub unrecorded_cash_delta_bn: f64,
    pub unrecorded_debt_delta_bn: f64,
    pub flows: BTreeMap<CashCause, MoneyFlow>,
    pub fiscal: Option<FiscalDetail>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PolicySnapshot {
    pub tax_rate: f64,
    pub interest_rate: f64,
    pub allocations: [f64; BUDGET_MINISTRIES],
    pub departments: Option<programs::Shares>,
    pub construction_daily_budget_bn: Option<f64>,
    pub treasury_bn: f64,
    pub debt_bn: f64,
    pub gdp: f64,
    pub political_capital: f64,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PolicyOutcome {
    pub day: i32,
    pub treasury_bn: f64,
    pub debt_bn: f64,
    pub gdp: f64,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PolicyDecision {
    pub id: u64,
    pub day: i32,
    pub kind: String,
    pub before: PolicySnapshot,
    pub after: PolicySnapshot,
    pub baseline_observation: Option<fiscal_recovery::Observation>,
    /// Last later closed month, with its actual exposure. This is an observed
    /// outcome after a decision, not a causal estimate of that decision alone.
    pub latest_observation: Option<fiscal_recovery::Observation>,
    pub latest_outcome: Option<PolicyOutcome>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct MoneyJournal {
    pub started_day: i32,
    pub next_decision_id: u64,
    pub days: Vec<MoneyDay>,
    pub decisions: Vec<PolicyDecision>,
}

#[derive(Clone, Debug, Serialize)]
pub struct JournalView {
    pub started_day: i32,
    pub current_day: i32,
    pub current_treasury_bn: f64,
    pub current_debt_bn: f64,
    pub unrecorded_cash_delta_bn: f64,
    pub unrecorded_debt_delta_bn: f64,
    pub days: Vec<MoneyDay>,
    pub decisions: Vec<PolicyDecision>,
}

pub fn view(w: &WorldState, id: NationId) -> Option<JournalView> {
    let journal = w.fiscal_recovery.nations.get(&id)?.money_journal.as_ref()?;
    let (cash, debt) = balances(w, id)?;
    let last = journal.days.last()?;
    Some(JournalView {
        started_day: journal.started_day,
        current_day: clock::absolute_day(w),
        current_treasury_bn: cash,
        current_debt_bn: debt,
        unrecorded_cash_delta_bn: cash - last.closing_treasury_bn,
        unrecorded_debt_delta_bn: debt - last.closing_debt_bn,
        days: journal.days.clone(),
        decisions: journal.decisions.clone(),
    })
}

fn balances(w: &WorldState, id: NationId) -> Option<(f64, f64)> {
    let n = w.nation_opt(id)?;
    Some((n.treasury_bn?, n.debt_bn?))
}
fn today(journal: &mut MoneyJournal, day: i32, cash: f64, debt: f64) -> Option<&mut MoneyDay> {
    if journal.days.last().is_some_and(|last| day < last.day) {
        return None;
    }
    if journal.days.last().is_none_or(|last| last.day != day) {
        let (opening_cash, opening_debt) = journal.days.last().map_or((cash, debt), |last| {
            (last.closing_treasury_bn, last.closing_debt_bn)
        });
        journal.days.push(MoneyDay {
            day,
            opening_treasury_bn: opening_cash,
            opening_debt_bn: opening_debt,
            closing_treasury_bn: cash,
            closing_debt_bn: debt,
            unrecorded_cash_delta_bn: cash - opening_cash,
            unrecorded_debt_delta_bn: debt - opening_debt,
            flows: BTreeMap::new(),
            fiscal: None,
        });
        journal
            .days
            .retain(|row| row.day >= day.saturating_sub(RETAINED_DAYS as i32 - 1));
    }
    journal.days.last_mut()
}
fn start(w: &mut WorldState, id: NationId, before: (f64, f64)) -> Option<&mut MoneyJournal> {
    if !fiscal_recovery::enabled(w) {
        return None;
    }
    let day = clock::absolute_day(w);
    let state = w.fiscal_recovery.nations.get_mut(&id)?;
    let journal = state.money_journal.get_or_insert_with(|| MoneyJournal {
        started_day: day,
        next_decision_id: 1,
        days: vec![],
        decisions: vec![],
    });
    today(journal, day, before.0, before.1)?;
    Some(journal)
}

/// Observe exactly one existing money operation after it has paid. The caller
/// supplies actual before/after stocks, preserving its original float order.
pub(crate) fn record_payment(
    w: &mut WorldState,
    id: NationId,
    cause: CashCause,
    amount_bn: f64,
    before: (f64, f64),
    after: (f64, f64),
) {
    if amount_bn == 0.0 && before == after && cause != CashCause::BudgetSettlement {
        return;
    }
    let Some(journal) = start(w, id, before) else {
        return;
    };
    let row = journal.days.last_mut().unwrap();
    row.unrecorded_cash_delta_bn += before.0 - row.closing_treasury_bn;
    row.unrecorded_debt_delta_bn += before.1 - row.closing_debt_bn;
    let flow = row.flows.entry(cause).or_default();
    flow.postings = flow.postings.saturating_add(1);
    flow.inflow_bn += (-amount_bn).max(0.0);
    flow.outflow_bn += amount_bn.max(0.0);
    flow.treasury_delta_bn += after.0 - before.0;
    flow.debt_delta_bn += after.1 - before.1;
    row.closing_treasury_bn = after.0;
    row.closing_debt_bn = after.1;
}

pub(crate) fn record_fiscal_detail(w: &mut WorldState, id: NationId, detail: FiscalDetail) {
    let day = clock::absolute_day(w);
    if let Some(row) = w
        .fiscal_recovery
        .nations
        .get_mut(&id)
        .and_then(|n| n.money_journal.as_mut())
        .and_then(|j| j.days.last_mut())
        .filter(|row| row.day == day)
    {
        row.fiscal = Some(detail);
    }
}
pub(crate) fn supplier_settled(
    w: &mut WorldState,
    id: NationId,
    day: i32,
    kind: SupplierInclusion,
    amount: f64,
) {
    // No new public charge or backfilled journal: only append an inclusion to
    // the already recorded fiscal owner once its real receivable is removed.
    if let Some(fiscal) = w
        .fiscal_recovery
        .nations
        .get_mut(&id)
        .and_then(|n| n.money_journal.as_mut())
        .and_then(|j| j.days.iter_mut().find(|row| row.day == day))
        .and_then(|row| row.fiscal.as_mut())
    {
        *fiscal.supplier_inclusions_bn.entry(kind).or_default() += amount;
    }
}

fn snapshot(w: &WorldState, id: NationId) -> Option<PolicySnapshot> {
    let n = w.nation_opt(id)?;
    Some(PolicySnapshot {
        tax_rate: n.tax_rate,
        interest_rate: n.interest_rate,
        allocations: n.budget_for(w.year).allocations,
        departments: n.program_budget.as_ref().map(|p| p.departments),
        construction_daily_budget_bn: n
            .program_budget
            .as_ref()
            .and_then(|p| p.construction_daily_budget_bn),
        treasury_bn: n.treasury_bn?,
        debt_bn: n.debt_bn?,
        gdp: n.gdp,
        political_capital: n.political_capital,
    })
}
pub(crate) struct PendingPolicy {
    id: NationId,
    kind: String,
    before: PolicySnapshot,
}
pub(crate) fn before_policy(w: &WorldState, command: &Command) -> Option<PendingPolicy> {
    if !fiscal_recovery::enabled(w) {
        return None;
    }
    let (id, kind) = match command {
        Command::SetTaxRate { nation, .. } => (*nation, "tax_rate"),
        Command::SetInterestRate { nation, .. } => (*nation, "interest_rate"),
        Command::SetAnnualBudget { nation, .. } => (*nation, "annual_budget"),
        Command::SetProgramBudget { nation, .. } => (*nation, "program_budget"),
        Command::SetConstructionBudget { nation, .. } => (*nation, "construction_budget"),
        Command::SetBudget { nation, .. } => (*nation, "aggregate_budget"),
        Command::SetMilSpend { nation, .. } => (*nation, "military_budget"),
        Command::SetStateInvest { nation, .. } => (*nation, "investment_budget"),
        // The package dispatches its child settings internally. Observe its
        // single approved outer command, with its single political price.
        Command::EnactStratagem { nation, id } if id == "austerity" => {
            (*nation, "fiscal_consolidation")
        }
        Command::EnactStratagem { nation, id } if id == "debt_restructuring" => {
            (*nation, "debt_restructuring")
        }
        Command::EnactStratagem { nation, id } if id == "mass_privatisation" => {
            (*nation, "asset_sale")
        }
        _ => return None,
    };
    Some(PendingPolicy {
        id,
        kind: kind.into(),
        before: snapshot(w, id)?,
    })
}
pub(crate) fn after_policy(w: &mut WorldState, pending: PendingPolicy) {
    let Some(after) = snapshot(w, pending.id) else {
        return;
    };
    let baseline = w
        .fiscal_recovery
        .nations
        .get(&pending.id)
        .and_then(|f| f.observations.last())
        .cloned();
    let day = clock::absolute_day(w);
    let Some(journal) = start(
        w,
        pending.id,
        (pending.before.treasury_bn, pending.before.debt_bn),
    ) else {
        return;
    };
    // Tagged stratagem payments have already updated this day. Other policies
    // authorize future spending and do not invent an immediate saving.
    let id = journal.next_decision_id;
    let Some(next) = id.checked_add(1) else {
        return;
    };
    journal.next_decision_id = next;
    journal.decisions.push(PolicyDecision {
        id,
        day,
        kind: pending.kind,
        before: pending.before,
        after,
        baseline_observation: baseline,
        latest_observation: None,
        latest_outcome: None,
    });
    if journal.decisions.len() > RETAINED_DECISIONS {
        journal.decisions.remove(0);
    }
}

/// Called after the existing fiscal observer has closed the actual day. This
/// updates existing decision evidence only; it cannot start a missing journal.
#[doc(hidden)]
pub fn finish_day(w: &mut WorldState) {
    if !fiscal_recovery::enabled(w) {
        return;
    }
    let day = clock::absolute_day(w);
    for n in w.nations.iter().filter(|n| n.alive) {
        let (Some(cash), Some(debt)) = (n.treasury_bn, n.debt_bn) else {
            continue;
        };
        let Some(fiscal) = w.fiscal_recovery.nations.get_mut(&n.id) else {
            continue;
        };
        let last = fiscal.observations.last().cloned();
        let Some(journal) = fiscal.money_journal.as_mut() else {
            continue;
        };
        let Some(row) = today(journal, day, cash, debt) else {
            continue;
        };
        row.unrecorded_cash_delta_bn += cash - row.closing_treasury_bn;
        row.unrecorded_debt_delta_bn += debt - row.closing_debt_bn;
        row.closing_treasury_bn = cash;
        row.closing_debt_bn = debt;
        for decision in &mut journal.decisions {
            if decision.day > day {
                continue;
            }
            decision.latest_outcome = Some(PolicyOutcome {
                day,
                treasury_bn: cash,
                debt_bn: debt,
                gdp: n.gdp,
            });
            if let Some(observation) = last.as_ref().filter(|o| {
                decision
                    .baseline_observation
                    .as_ref()
                    .is_none_or(|b| o.month > b.month)
            }) {
                decision.latest_observation = Some(observation.clone());
            }
        }
    }
}

fn finite(values: &[f64]) -> bool {
    values.iter().all(|v| v.is_finite())
}
fn nonnegative(values: &[f64]) -> bool {
    values.iter().all(|v| v.is_finite() && *v >= 0.0)
}
// Grouping by cause changes addition order. This forward-error bound permits
// floating addition dust, not a fixed monetary discrepancy or repaired balance.
fn close(a: f64, b: f64, postings: u64, operands: f64) -> bool {
    // Stock subtraction can lose more precision than the resulting tiny fee:
    // e.g. 10_000 - 1e-8 differs from 1e-8 by ~8e-13. Gross fiscal operands
    // likewise matter when revenue and spending nearly cancel. Scale by the
    // actual operands as well as grouped sums, never by an arbitrary cash floor.
    let scale = a.abs().max(b.abs()).max(operands.abs()).max(1.0);
    a.is_finite()
        && b.is_finite()
        && operands.is_finite()
        && (a - b).abs() <= 128.0 * f64::EPSILON * scale * (postings.min(1_000_000) as f64 + 1.0)
}
fn valid_snapshot(s: &PolicySnapshot) -> bool {
    // Descriptive snapshots do not impose a new sign restriction on an older
    // accepted policy. Current command limits remain owned by their commands.
    finite(&[s.tax_rate, s.interest_rate, s.political_capital])
        && nonnegative(&[s.treasury_bn, s.debt_bn, s.gdp])
        && s.gdp > 0.0
        && finite(&s.allocations)
        && s.construction_daily_budget_bn
            .is_none_or(|v| v.is_finite() && v >= 0.0)
        && s.departments.as_ref().is_none_or(|rows| {
            rows.iter()
                .all(|row| row.iter().map(|v| *v as u32).sum::<u32>() == 10_000)
        })
}
fn valid_observation(o: &fiscal_recovery::Observation, month: i32) -> bool {
    o.month <= month
        && nonnegative(&[
            o.opening_debt_bn,
            o.closing_debt_bn,
            o.opening_treasury_bn,
            o.closing_treasury_bn,
            o.opening_gdp,
            o.closing_gdp,
            o.spending_bn,
            o.gdp_years,
            o.year_fraction,
            o.tax_gdp_years,
        ])
        && finite(&[o.revenue_bn, o.interest_bn])
        && o.opening_gdp > 0.0
        && o.closing_gdp > 0.0
        && o.gdp_years > 0.0
        && o.year_fraction > 0.0
        && o.year_fraction <= 1.0 / 12.0 + 1e-10
}
pub(crate) fn validate(
    j: &MoneyJournal,
    started: i32,
    today: i32,
    month: i32,
) -> Result<(), String> {
    let refuse = || "Invalid dated money journal or policy outcome.".to_string();
    if j.started_day < started
        || j.started_day > today
        || j.days.is_empty()
        || j.days.len() > RETAINED_DAYS
        || j.decisions.len() > RETAINED_DECISIONS
        || j.next_decision_id == 0
        || j.days.windows(2).any(|p| p[0].day >= p[1].day)
        || j.decisions
            .windows(2)
            .any(|p| p[0].id >= p[1].id || p[0].day > p[1].day)
    {
        return Err(refuse());
    }
    for row in &j.days {
        if row.day < j.started_day
            || row.day > today
            || !nonnegative(&[
                row.opening_treasury_bn,
                row.opening_debt_bn,
                row.closing_treasury_bn,
                row.closing_debt_bn,
            ])
            || !finite(&[row.unrecorded_cash_delta_bn, row.unrecorded_debt_delta_bn])
        {
            return Err(refuse());
        }
        let mut cash = row.unrecorded_cash_delta_bn;
        let mut debt = row.unrecorded_debt_delta_bn;
        let mut count = 0_u64;
        let stock_scale = [
            row.opening_treasury_bn,
            row.opening_debt_bn,
            row.closing_treasury_bn,
            row.closing_debt_bn,
            row.unrecorded_cash_delta_bn.abs(),
            row.unrecorded_debt_delta_bn.abs(),
        ]
        .into_iter()
        .fold(0.0_f64, f64::max)
            + row
                .flows
                .values()
                .map(|f| f.inflow_bn + f.outflow_bn)
                .sum::<f64>();
        for (cause, flow) in &row.flows {
            if flow.postings == 0
                || !nonnegative(&[flow.inflow_bn, flow.outflow_bn])
                || !finite(&[flow.treasury_delta_bn, flow.debt_delta_bn])
                || !close(
                    flow.treasury_delta_bn - flow.debt_delta_bn,
                    flow.inflow_bn - flow.outflow_bn,
                    flow.postings,
                    stock_scale,
                )
            {
                return Err("Money payment does not reconcile to its cash and debt stocks.".into());
            }
            if *cause == CashCause::DebtRestructuring
                && (flow.outflow_bn != 0.0
                    || flow.treasury_delta_bn != 0.0
                    || flow.debt_delta_bn > 0.0)
            {
                return Err(refuse());
            }
            cash += flow.treasury_delta_bn;
            debt += flow.debt_delta_bn;
            count = count.saturating_add(flow.postings);
        }
        if !close(
            row.closing_treasury_bn - row.opening_treasury_bn,
            cash,
            count,
            stock_scale,
        ) || !close(
            row.closing_debt_bn - row.opening_debt_bn,
            debt,
            count,
            stock_scale,
        ) {
            return Err("Dated money stocks do not reconcile to their recorded movements.".into());
        }
        if let Some(f) = &row.fiscal {
            if !finite(&[f.revenue_bn, f.interest_bn])
                || !nonnegative(&[f.spending_bn, f.prepaid_used_bn, f.construction_bn])
                || f.departments_bn
                    .as_ref()
                    .is_some_and(|a| a.iter().any(|r| !nonnegative(r)))
                || f.supplier_inclusions_bn
                    .values()
                    .any(|v| !v.is_finite() || *v < 0.0)
                || !row.flows.contains_key(&CashCause::BudgetSettlement)
            {
                return Err(refuse());
            }
            let flow = &row.flows[&CashCause::BudgetSettlement];
            if !close(
                flow.inflow_bn - flow.outflow_bn,
                f.revenue_bn - f.spending_bn - f.interest_bn,
                flow.postings,
                f.revenue_bn.abs() + f.spending_bn + f.interest_bn.abs(),
            ) || f.construction_bn > f.spending_bn + 1e-10
                || f.supplier_inclusions_bn.values().sum::<f64>()
                    > f.spending_bn + f.prepaid_used_bn + 1e-10
            {
                return Err(refuse());
            }
            if let Some(rows) = &f.departments_bn {
                if !close(
                    rows.iter().flatten().sum(),
                    f.spending_bn,
                    50,
                    f.spending_bn,
                ) {
                    return Err(refuse());
                }
            }
        }
    }
    for pair in j.days.windows(2) {
        if pair[1].opening_treasury_bn != pair[0].closing_treasury_bn
            || pair[1].opening_debt_bn != pair[0].closing_debt_bn
        {
            return Err(refuse());
        }
    }
    for decision in &j.decisions {
        let (decision_year, decision_month, _) = clock::date_from_day(decision.day);
        let decision_month = (decision_year - 1990) * 12 + decision_month as i32 - 1;
        if decision.id == 0
            || decision.id >= j.next_decision_id
            || decision.day < j.started_day
            || decision.day > today
            || !matches!(
                decision.kind.as_str(),
                "tax_rate"
                    | "interest_rate"
                    | "annual_budget"
                    | "program_budget"
                    | "construction_budget"
                    | "aggregate_budget"
                    | "military_budget"
                    | "investment_budget"
                    | "fiscal_consolidation"
                    | "debt_restructuring"
                    | "asset_sale"
            )
            || !valid_snapshot(&decision.before)
            || !valid_snapshot(&decision.after)
            || decision
                .baseline_observation
                .as_ref()
                .is_some_and(|o| !valid_observation(o, decision_month))
            || decision.latest_observation.as_ref().is_some_and(|o| {
                !valid_observation(o, month)
                    || decision
                        .baseline_observation
                        .as_ref()
                        .is_some_and(|b| o.month <= b.month)
            })
            || decision.latest_outcome.as_ref().is_some_and(|o| {
                o.day < decision.day
                    || o.day > today
                    || !nonnegative(&[o.treasury_bn, o.debt_bn, o.gdp])
                    || o.gdp <= 0.0
            })
        {
            return Err(refuse());
        }
    }
    Ok(())
}
