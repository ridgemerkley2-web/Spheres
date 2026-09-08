//! Shared, priced fiscal recovery policy for enabled daily campaigns.
//!
//! These are MODEL policy choices, not historical tax or spending targets.
//! Governments review every 90 days. The same cash accounts and commands are
//! available to the player, whose policies this system never changes.
use crate::{clock, fiscal_recovery, programs, world::*, Command};

pub const REVIEW_DAYS: i32 = 90;
pub const RESTRUCTURING_COOLDOWN_DAYS: i32 = 5 * 365;
const PC_RESERVE: f64 = 8.0;

/// A prospective reduction to automatically expensed services. Project-funded
/// departments are excluded: reducing unused authority is not a cash saving.
pub struct BudgetAdjustment {
    pub command: Command,
    pub expected_operating_savings_gdp: f64,
}

/// The explicit, limited set of operating economies used by a recovery vote.
/// Essential services, teaching, pensions, security, productive maintenance,
/// industry operations and science are preserved. Enrolled defense staffing/
/// operations may be trimmed only in peace; procurement and maintenance never
/// are. Without departments defense stays intact because its support bill
/// cannot be separated from its aggregate envelope.
/// Floors refer to the inherited department envelope, not last quarter's cut,
/// so repeating reviews cannot erase a department by compounding forever.
fn floor_fraction(
    w: &WorldState,
    nation: NationId,
    ministry: usize,
    department: usize,
) -> Option<f64> {
    match (ministry, department) {
        (BUDGET_DIPLOMACY, 2) => Some(0.50),   // Foreign aid.
        (BUDGET_HOUSING, 1 | 4) => Some(0.80), // Renovation / urban development.
        (BUDGET_DEFENSE, 0 | 1)
            if !w.at_war(nation) && w.nation(nation).program_budget.is_some() =>
        {
            Some(0.85)
        }
        _ => None,
    }
}

/// Pure quote shared by quarterly AI and the existing Fiscal Consolidation
/// Cabinet action. It changes future spending only; receipts, prepaid funds,
/// available authority and the current day's obligations retain their owners.
pub fn budget_adjustment(
    w: &WorldState,
    nation: NationId,
    cut_fraction: f64,
    maximum_savings_gdp: f64,
) -> Option<BudgetAdjustment> {
    let n = w.nation(nation);
    let current = n.budget_for(w.year);
    let mut allocations = current.allocations;
    let mut departments = n
        .program_budget
        .as_ref()
        .map_or_else(programs::default_departments, |p| p.departments);
    let reference_departments = n
        .program_budget
        .as_ref()
        .map_or_else(programs::default_departments, |p| p.reference_departments);
    let mut remaining = maximum_savings_gdp.max(0.0);
    let mut savings = 0.0;

    // Fixed priority: diplomatic aid, optional housing services, then peacetime
    // defense overhead. Never multiply every ministry by one austerity factor.
    for ministry in [BUDGET_DIPLOMACY, BUDGET_HOUSING, BUDGET_DEFENSE] {
        let old = std::array::from_fn::<_, { programs::DEPARTMENTS }, _>(|d| {
            current.allocations[ministry] * departments[ministry][d] as f64 / 10_000.0
        });
        let mut desired = old;
        for d in 0..programs::DEPARTMENTS {
            let Some(floor) = floor_fraction(w, nation, ministry, d) else {
                continue;
            };
            if n.program_budget.is_some() && programs::is_project_funded(n, ministry, d) {
                continue;
            }
            let minimum = current.reference[ministry] * reference_departments[ministry][d] as f64
                / 10_000.0
                * floor;
            // Four other integer shares can each round upward by less than
            // one basis point. Leave five basis points of the old ministry
            // envelope above the true floor so the residual can absorb their
            // rounding while still delivering a feasible cut near that floor.
            let rounding_margin = if n.program_budget.is_some() {
                current.allocations[ministry] * programs::DEPARTMENTS as f64 / 10_000.0
            } else {
                0.0
            };
            let reduction = (old[d] - minimum - rounding_margin)
                .max(0.0)
                .min(old[d] * cut_fraction.clamp(0.0, 1.0))
                .min(remaining);
            desired[d] -= reduction;
            remaining -= reduction;
        }
        if desired == old {
            continue;
        }
        let new_total = desired.iter().sum::<f64>();
        if new_total >= current.allocations[ministry] || new_total <= 0.0 {
            continue;
        }
        if n.program_budget.is_none() {
            // Legacy aggregate services are all expensed. Seating a regular
            // annual budget keeps their original reference and opens no new
            // departmental or physical-production ledger.
            allocations[ministry] = new_total;
            savings += current.allocations[ministry] - new_total;
            continue;
        }

        // Department votes use integer basis points. Round preserved envelopes
        // upward; the department actually being cut absorbs the residual, so
        // rounding cannot quietly cut a protected service or maintenance row.
        let residual = (0..programs::DEPARTMENTS)
            .filter(|&d| desired[d] < old[d])
            .max_by(|&a, &b| (old[a] - desired[a]).total_cmp(&(old[b] - desired[b])))?;
        let mut shares = [0_u16; programs::DEPARTMENTS];
        let mut used = 0_u32;
        for d in 0..programs::DEPARTMENTS {
            if d == residual {
                continue;
            }
            shares[d] = ((desired[d] / new_total * 10_000.0).ceil() as u32).min(10_000) as u16;
            used += shares[d] as u32;
        }
        if used > 10_000 {
            continue;
        }
        shares[residual] = (10_000 - used) as u16;
        let residual_floor = current.reference[ministry]
            * reference_departments[ministry][residual] as f64
            / 10_000.0
            * floor_fraction(w, nation, ministry, residual).unwrap_or(1.0);
        if new_total * shares[residual] as f64 / 10_000.0 < residual_floor {
            continue;
        }
        allocations[ministry] = new_total;
        departments[ministry] = shares;
        savings += current.allocations[ministry] - new_total;
    }
    if savings < 0.000_01
        || allocations
            .iter()
            .enumerate()
            .any(|(m, v)| !v.is_finite() || *v < 0.0 || *v > BUDGET_CAPS[m])
        || allocations.iter().sum::<f64>() > 0.70
    {
        return None;
    }
    let command = if n.program_budget.is_some() {
        Command::SetProgramBudget {
            nation,
            fiscal_year: w.year,
            allocations,
            departments,
        }
    } else {
        Command::SetAnnualBudget {
            nation,
            fiscal_year: w.year,
            allocations,
        }
    };
    Some(BudgetAdjustment {
        command,
        expected_operating_savings_gdp: savings,
    })
}

pub fn restructuring_ready(w: &WorldState, nation: NationId) -> bool {
    w.fiscal_recovery
        .nations
        .get(&nation)
        .and_then(|n| n.last_restructuring_day)
        .is_none_or(|day| clock::absolute_day(w) - day >= RESTRUCTURING_COOLDOWN_DAYS)
}

fn policy(w: &WorldState, nation: NationId, view: &fiscal_recovery::Assessment) -> Option<Command> {
    if !view.recovery_required || view.months_observed == 0 {
        return None;
    }
    let n = w.nation(nation);
    // Surveillance deliberately uses paid trailing receipts. A new tax policy
    // therefore enters that average over twelve months. Do not buy the same
    // revenue improvement at each intervening quarterly review while waiting
    // for receipts to catch up. This changes advice only, never actual pressure.
    let needed = view.current_policy_adjustment_gdp.max(0.0);
    // A write-down remains the existing costly Cabinet decision. It is reserved
    // for a sustained service burden that ordinary small adjustments cannot
    // promptly close, and remains unavailable during its shared cooldown.
    if view.months_observed >= 12
        && needed > 0.04
        && view.interest_revenue.is_none_or(|v| v > 0.60)
        && (n.tax_rate >= 0.45 || view.stress_months >= 12)
        && crate::stratagems::closed_reason(w, nation, "debt_restructuring").is_none()
    {
        let cmd = Command::EnactStratagem {
            nation,
            id: "debt_restructuring".into(),
        };
        if crate::price_of(w, &cmd).is_some_and(|p| p + PC_RESERVE <= n.political_capital) {
            return Some(cmd);
        }
    }
    // One quarter buys at most one point of tax. Stop at the assessed gap;
    // already stable high-debt countries take no action at all.
    let tax = || Command::SetTaxRate {
        nation,
        rate: (n.tax_rate + needed.min(0.01)).min(0.55),
    };
    if n.tax_rate < 0.40 && needed >= 0.000_5 {
        return Some(tax());
    }
    if let Some(cut) = budget_adjustment(w, nation, 0.05, needed.min(0.01)) {
        if cut.expected_operating_savings_gdp >= 0.000_25
            && crate::price_of(w, &cut.command)
                .is_some_and(|p| p + PC_RESERVE <= n.political_capital)
        {
            return Some(cut.command);
        }
    }
    if n.tax_rate < 0.55 && needed >= 0.000_5 {
        return Some(tax());
    }
    None
}

/// A pure next-step proposal for either a human government or the AI. Reading
/// this never changes policy; the player may review and submit its ordinary
/// command, with the same political price and affordability checks.
pub fn proposal(w: &WorldState, nation: NationId) -> Option<Command> {
    let view = fiscal_recovery::assessment(w, nation)?;
    policy(w, nation, &view)
}

fn record(w: &mut WorldState, nation: NationId, action: &str, reason: String) {
    let row = w.fiscal_recovery.nations.entry(nation).or_default();
    row.last_ai_action = action.into();
    row.last_ai_reason = reason;
}

pub fn tick(w: &mut WorldState) {
    if !fiscal_recovery::enabled(w) {
        return;
    }
    fiscal_recovery::prepare(w);
    let today = clock::absolute_day(w);
    let mut actors: Vec<_> = w
        .nations
        .iter()
        .filter(|n| n.alive && Some(n.id) != w.player)
        .map(|n| n.id)
        .collect();
    actors.sort();
    for nation in actors {
        let standing = w.nation(nation);
        if let Some(plan) = standing
            .program_budget
            .as_ref()
            .filter(|p| p.fiscal_year < w.year)
        {
            // Annual authority has its own calendar, even when industrial
            // competition is disabled. Renew exactly the standing settlement;
            // this grants no cash, changes no policy and is not a recovery act.
            let command = Command::SetProgramBudget {
                nation,
                fiscal_year: w.year,
                allocations: standing.budget_for(w.year).allocations,
                departments: plan.departments,
            };
            if let Err(why) = crate::apply_command(w, &command) {
                record(w, nation, "Budget renewal refused", why);
            }
        }
        let due = w
            .fiscal_recovery
            .nations
            .get(&nation)
            .and_then(|n| n.last_ai_review_day)
            .is_none_or(|day| today - day >= REVIEW_DAYS);
        if !due {
            continue;
        }
        let Some(view) = fiscal_recovery::assessment(w, nation) else {
            continue;
        };
        if view.months_observed == 0 {
            continue;
        }
        w.fiscal_recovery
            .nations
            .entry(nation)
            .or_default()
            .last_ai_review_day = Some(today);
        let Some(cmd) = proposal(w, nation) else {
            let reason = if !view.recovery_required {
                "Debt service and the current trajectory need no additional adjustment.".into()
            } else if view.current_policy_adjustment_gdp < 0.000_5 {
                "The enacted policy covers the estimated adjustment; waiting for actual receipts to confirm recovery.".into()
            } else {
                "Protected service floors and the tax ceiling leave no feasible small adjustment; the fiscal warning remains active.".into()
            };
            record(w, nation, "Hold policy", reason);
            continue;
        };
        let price = crate::price_of(w, &cmd).unwrap_or(f64::INFINITY);
        if price + PC_RESERVE > w.nation(nation).political_capital {
            record(w, nation, "Waiting for political capital", format!(
                "The next ordinary command costs {price:.1} PC plus an {PC_RESERVE:.0} PC governing reserve."));
            continue;
        }
        // No synthetic income, PC or change to the confidence ledger. The
        // assessment must observe actual results before pressure eases.
        let action = match &cmd {
            Command::SetTaxRate { .. } => "Raised taxes",
            Command::SetAnnualBudget { .. } | Command::SetProgramBudget { .. } => {
                "Trimmed discretionary services"
            }
            Command::EnactStratagem { .. } => "Restructured debt",
            _ => "Reviewed fiscal policy",
        };
        match crate::apply_command(w, &cmd) {
            Ok(()) => record(w, nation, action, format!(
                "Quarterly response to a {:.2}% GDP current-policy adjustment gap; paid {price:.1} PC. Confidence awaits observed results.",
                view.current_policy_adjustment_gdp * 100.0)),
            Err(why) => record(w, nation, "Recovery command refused", why),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn world() -> WorldState {
        let mut w = crate::init::world_1990(GameRules {
            daily_simulation: true,
            fiscal_recovery: true,
            ..GameRules::default()
        });
        w.player = Some(NationId::USA);
        w.nation_mut(NationId::USA).political_capital = 500.0;
        w
    }

    fn enroll(w: &mut WorldState) {
        let allocations = w.nation(NationId::USA).budget_for(w.year).allocations;
        crate::apply_command(
            w,
            &Command::SetProgramBudget {
                nation: NationId::USA,
                fiscal_year: w.year,
                allocations,
                departments: programs::default_departments(),
            },
        )
        .unwrap();
    }

    fn department_envelopes(w: &WorldState) -> programs::Amounts {
        let n = w.nation(NationId::USA);
        let allocations = n.budget_for(w.year).allocations;
        let departments = n.program_budget.as_ref().unwrap().departments;
        std::array::from_fn(|m| {
            std::array::from_fn(|d| allocations[m] * departments[m][d] as f64 / 10_000.0)
        })
    }

    fn observed_deficit(w: &mut WorldState) {
        fiscal_recovery::prepare(w);
        let n = w.nation_mut(NationId::USA);
        n.tax_rate = 0.20;
        n.debt_bn = Some(n.gdp * 3.0);
        n.debt_gdp = 3.0;
        for _ in 0..31 {
            let gdp = w.nation(NationId::USA).gdp;
            let fraction = clock::year_fraction(w);
            let day = clock::absolute_day(w);
            fiscal_recovery::record_fiscal(
                &mut w.fiscal_recovery,
                NationId::USA,
                day,
                gdp * fraction * 0.20,
                gdp * fraction * 0.30,
                gdp * fraction * 0.10,
            );
            fiscal_recovery::tick(w);
            clock::advance_date(w);
        }
        assert!(
            fiscal_recovery::assessment(w, NationId::USA)
                .unwrap()
                .recovery_required
        );
    }

    #[test]
    fn quarterly_ai_pays_ordinary_tax_price_and_never_changes_player_policy() {
        let mut w = world();
        observed_deficit(&mut w);
        let before = w.nation(NationId::USA).tax_rate;
        let pc = w.nation(NationId::USA).political_capital;
        tick(&mut w);
        assert_eq!(w.nation(NationId::USA).tax_rate, before);
        assert_eq!(w.nation(NationId::USA).political_capital, pc);
        assert!(w.fiscal_recovery.nations[&NationId::USA]
            .last_ai_review_day
            .is_none());
        w.player = None;
        let proposed = proposal(&w, NationId::USA).unwrap();
        let price = crate::price_of(&w, &proposed).unwrap();
        let pressure = w.fiscal_recovery.nations[&NationId::USA].confidence_pressure;
        tick(&mut w);
        assert_eq!(w.nation(NationId::USA).tax_rate, before + 0.01);
        assert_eq!(w.nation(NationId::USA).political_capital, pc - price);
        assert_eq!(
            w.fiscal_recovery.nations[&NationId::USA].confidence_pressure,
            pressure
        );
        let after = crate::state_hash(&w);
        tick(&mut w);
        assert_eq!(crate::state_hash(&w), after, "same-day duplicate review");
        for _ in 0..89 {
            clock::advance_date(&mut w);
        }
        tick(&mut w);
        assert_eq!(w.nation(NationId::USA).tax_rate, before + 0.01);
        clock::advance_date(&mut w);
        tick(&mut w);
        assert_eq!(w.nation(NationId::USA).tax_rate, (before + 0.01) + 0.01);
        assert_eq!(
            w.fiscal_recovery.nations[&NationId::USA].last_ai_action,
            "Raised taxes"
        );
    }

    #[test]
    fn consolidation_updates_live_departments_for_the_single_cabinet_price() {
        let mut w = world();
        observed_deficit(&mut w);
        enroll(&mut w);
        let before = department_envelopes(&w);
        let tax = w.nation(NationId::USA).tax_rate;
        let stability = w.nation(NationId::USA).stability;
        let pc = w.nation(NationId::USA).political_capital;
        crate::apply_command(
            &mut w,
            &Command::EnactStratagem {
                nation: NationId::USA,
                id: "austerity".into(),
            },
        )
        .unwrap();
        let after = department_envelopes(&w);
        assert!(after[BUDGET_DIPLOMACY][2] < before[BUDGET_DIPLOMACY][2]);
        for (ministry, department, floor) in [
            (BUDGET_HOUSING, 1, 0.80),
            (BUDGET_HOUSING, 4, 0.80),
            (BUDGET_DEFENSE, 0, 0.85),
            (BUDGET_DEFENSE, 1, 0.85),
        ] {
            assert!(
                after[ministry][department] < before[ministry][department],
                "rounding discarded the feasible cut to {ministry}/{department}"
            );
            assert!(
                after[ministry][department] >= before[ministry][department] * floor,
                "the Cabinet cut crossed the protected floor for {ministry}/{department}"
            );
        }
        assert!(after[BUDGET_DEFENSE][2] >= before[BUDGET_DEFENSE][2]);
        assert_eq!(after[BUDGET_EDUCATION], before[BUDGET_EDUCATION]);
        assert_eq!(w.nation(NationId::USA).tax_rate, tax + 0.05);
        assert_eq!(w.nation(NationId::USA).stability, stability - 9.0);
        assert_eq!(w.nation(NationId::USA).political_capital, pc - 34.0);
    }

    #[test]
    fn consolidation_does_not_try_to_seat_an_illegal_inherited_envelope() {
        let mut w = world();
        observed_deficit(&mut w);
        // The older coarse command permits aggregate totals above the newer
        // annual envelope's cap. A small recovery vote cannot legalize that
        // entire inherited policy by pretending the annual bounds disappeared.
        crate::apply_command(
            &mut w,
            &Command::SetBudget {
                nation: NationId::USA,
                social: 0.40,
                investment: 0.40,
                military: 0.35,
            },
        )
        .unwrap();
        assert!(budget_adjustment(&w, NationId::USA, 0.20, 0.02).is_none());
        crate::apply_command(
            &mut w,
            &Command::EnactStratagem {
                nation: NationId::USA,
                id: "austerity".into(),
            },
        )
        .unwrap();
        assert!(w.nation(NationId::USA).annual_budget.is_none());
        assert_eq!(w.nation(NationId::USA).tax_rate, 0.25);
    }

    #[test]
    fn exhausted_consolidation_is_refused_without_charging_or_changing_state() {
        let mut w = world();
        observed_deficit(&mut w);
        enroll(&mut w);
        for _ in 0..100 {
            let Some(quote) = budget_adjustment(&w, NationId::USA, 0.20, 0.02) else {
                break;
            };
            w.nation_mut(NationId::USA).political_capital = 500.0;
            crate::apply_command(&mut w, &quote.command).unwrap();
        }
        assert!(budget_adjustment(&w, NationId::USA, 0.20, 0.02).is_none());
        crate::apply_command(
            &mut w,
            &Command::SetTaxRate {
                nation: NationId::USA,
                rate: 0.60,
            },
        )
        .unwrap();
        // The trailing fiscal alarm still requires recovery, but this
        // particular Cabinet package has no remaining policy change to buy.
        assert!(
            fiscal_recovery::assessment(&w, NationId::USA)
                .unwrap()
                .recovery_required
        );
        let before = crate::state_hash(&w);
        assert!(crate::apply_command(
            &mut w,
            &Command::EnactStratagem {
                nation: NationId::USA,
                id: "austerity".into(),
            }
        )
        .is_err());
        assert_eq!(crate::state_hash(&w), before);
    }

    #[test]
    fn recovery_budget_cuts_cash_services_and_preserves_protected_departments() {
        let mut w = world();
        enroll(&mut w);
        programs::begin_day(&mut w);
        let before = department_envelopes(&w);
        let receipts = w.nation(NationId::USA).program_budget.clone().unwrap();
        let quote = budget_adjustment(&w, NationId::USA, 0.20, 0.02).unwrap();
        let quoted_saving = quote.expected_operating_savings_gdp;
        assert!(quoted_saving > 0.0);
        crate::apply_command(&mut w, &quote.command).unwrap();
        let after = department_envelopes(&w);
        let mut actual_operating_change = 0.0;
        for m in 0..BUDGET_MINISTRIES {
            for d in 0..programs::DEPARTMENTS {
                if floor_fraction(&w, NationId::USA, m, d).is_none() {
                    assert!(
                        after[m][d] >= before[m][d],
                        "protected department {m}/{d} was cut"
                    );
                }
                if !programs::is_project_funded(w.nation(NationId::USA), m, d) {
                    actual_operating_change += before[m][d] - after[m][d];
                }
            }
        }
        // Some rounding can enlarge protected project authority (procurement),
        // so the quoted automatic operating saving must never exceed delivery.
        assert!(quoted_saving <= actual_operating_change + 1e-12);
        let plan = w.nation(NationId::USA).program_budget.as_ref().unwrap();
        assert_eq!(plan.spent_today_bn, receipts.spent_today_bn);
        assert_eq!(plan.available_bn, receipts.available_bn);
        assert_eq!(plan.prepaid_bn, receipts.prepaid_bn);
        assert_ne!(plan.departments, receipts.departments);
    }

    #[test]
    fn unused_capital_authority_is_never_quoted_as_a_recovery_saving() {
        let mut w = world();
        enroll(&mut w);
        let baseline = budget_adjustment(&w, NationId::USA, 0.05, 0.01)
            .unwrap()
            .expected_operating_savings_gdp;
        let p = w.nation_mut(NationId::USA).program_budget.as_mut().unwrap();
        p.available_bn[BUDGET_INDUSTRY] = [1_000_000.0; programs::DEPARTMENTS];
        p.available_bn[BUDGET_DEFENSE][3] = 1_000_000.0;
        assert_eq!(
            budget_adjustment(&w, NationId::USA, 0.05, 0.01)
                .unwrap()
                .expected_operating_savings_gdp,
            baseline
        );
    }

    #[test]
    fn existing_ai_department_budgets_renew_without_industrial_competition() {
        let mut w = world();
        enroll(&mut w);
        let allocations = w.nation(NationId::USA).budget_for(w.year).allocations;
        let departments = w
            .nation(NationId::USA)
            .program_budget
            .as_ref()
            .unwrap()
            .departments;
        let pc = w.nation(NationId::USA).political_capital;
        w.player = None;
        w.year = 1991;
        assert!(!w.rules.economic_competition);
        fiscal_recovery::prepare(&mut w);
        let cash = w.nation(NationId::USA).treasury_bn;
        tick(&mut w);
        let n = w.nation(NationId::USA);
        assert_eq!(n.budget_for(w.year).allocations, allocations);
        assert_eq!(n.program_budget.as_ref().unwrap().departments, departments);
        assert_eq!(n.program_budget.as_ref().unwrap().fiscal_year, 1991);
        assert_eq!(n.political_capital, pc);
        assert_eq!(n.treasury_bn, cash);
        assert!(w.fiscal_recovery.nations[&NationId::USA]
            .last_ai_review_day
            .is_none());
    }

    #[test]
    fn repeated_recovery_votes_keep_inherited_operating_floors() {
        let mut w = world();
        enroll(&mut w);
        let initial = department_envelopes(&w);
        for _ in 0..100 {
            let Some(quote) = budget_adjustment(&w, NationId::USA, 0.20, 0.02) else {
                break;
            };
            w.nation_mut(NationId::USA).political_capital = 500.0;
            crate::apply_command(&mut w, &quote.command).unwrap();
        }
        let final_envelopes = department_envelopes(&w);
        for m in 0..BUDGET_MINISTRIES {
            for d in 0..programs::DEPARTMENTS {
                let floor = floor_fraction(&w, NationId::USA, m, d).unwrap_or(1.0);
                assert!(
                    final_envelopes[m][d] >= initial[m][d] * floor,
                    "department {m}/{d} fell below its inherited floor"
                );
            }
        }
    }

    #[test]
    fn restructuring_pays_once_retires_stock_and_has_a_shared_cooldown() {
        let mut w = world();
        fiscal_recovery::prepare(&mut w);
        let debt = w.nation(NationId::USA).gdp * 3.0;
        let n = w.nation_mut(NationId::USA);
        n.debt_bn = Some(debt);
        n.debt_gdp = 3.0;
        n.treasury_bn = Some(7.0);
        let cmd = Command::EnactStratagem {
            nation: NationId::USA,
            id: "debt_restructuring".into(),
        };
        let pc = n.political_capital;
        crate::apply_command(&mut w, &cmd).unwrap();
        let n = w.nation(NationId::USA);
        assert!((n.debt_bn.unwrap() - debt * 0.55).abs() < 1e-9);
        assert_eq!(n.treasury_bn, Some(7.0));
        assert_eq!(n.political_capital, pc - 30.0);
        assert!(n.debt_gdp > 1.10);
        let before_retry = crate::state_hash(&w);
        assert!(crate::apply_command(&mut w, &cmd)
            .unwrap_err()
            .contains("Creditors"));
        assert_eq!(crate::state_hash(&w), before_retry);
        let (year, month, day) =
            clock::date_from_day(clock::absolute_day(&w) + RESTRUCTURING_COOLDOWN_DAYS);
        w.year = year;
        w.month = month;
        w.day = day;
        assert!(
            crate::stratagems::closed_reason(&w, NationId::USA, "debt_restructuring").is_none()
        );
    }
}
