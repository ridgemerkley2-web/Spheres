//! Explicitly enrolled ministry programmes: authorization is not money.
//! One opening-GDP daily appropriation, shared departmental pools, and one
//! end-of-day fiscal disbursement. No field is present on legacy nations.
use crate::{clock, world::*};
use serde::{Deserialize, Serialize};

pub const DEPARTMENTS: usize = 5;
pub type Shares = [[u16; DEPARTMENTS]; BUDGET_MINISTRIES];
pub type Amounts = [[f64; DEPARTMENTS]; BUDGET_MINISTRIES];
pub const ZERO: Amounts = [[0.0; DEPARTMENTS]; BUDGET_MINISTRIES];
pub const NAMES: [[&str; DEPARTMENTS]; BUDGET_MINISTRIES] = [
    [
        "Primary care",
        "Hospitals",
        "Medicines & supplies",
        "Prevention",
        "Emergency medicine",
    ],
    [
        "Primary schools",
        "Secondary schools",
        "Vocational training",
        "Universities",
        "Teachers & facilities",
    ],
    [
        "Public housing",
        "Home renovation",
        "Housing assistance",
        "Water & sanitation",
        "Urban development",
    ],
    [
        "Retirement benefits",
        "Disability benefits",
        "Survivor benefits",
        "Minimum-income supplements",
        "Benefits administration",
    ],
    [
        "Roads & bridges",
        "Railways",
        "Ports",
        "Airports",
        "Network maintenance",
    ],
    [
        "Factories & construction",
        "Energy supply",
        "Minerals & processing",
        "Industrial supply chains",
        "Industrial modernization",
    ],
    [
        "Basic research",
        "Computing & communications",
        "Materials & energy research",
        "Life sciences",
        "Aerospace research",
    ],
    [
        "Personnel & training",
        "Operations",
        "Maintenance & supply",
        "Equipment procurement",
        "Military research",
    ],
    [
        "Policing",
        "Courts & corrections",
        "Border security",
        "Civil protection",
        "Domestic intelligence",
    ],
    [
        "Embassies",
        "Trade diplomacy",
        "Foreign aid",
        "International institutions",
        "Negotiations & mediation",
    ],
];

pub fn default_departments() -> Shares {
    [[2000; DEPARTMENTS]; BUDGET_MINISTRIES]
}

pub fn is_capital(ministry: usize, department: usize) -> bool {
    department < DEPARTMENTS
        && match ministry {
            BUDGET_INDUSTRY => true,
            BUDGET_INFRASTRUCTURE => department < 4,
            BUDGET_SCIENCE => department == 0,
            BUDGET_DEFENSE => department == 3,
            _ => false,
        }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProgramBudget {
    pub fiscal_year: i32,
    pub departments: Shares,
    pub reference_departments: Shares,
    pub authority_year: i32,
    pub available_bn: Amounts,
    /// Legacy equipment money already expensed before enrollment. Never charged
    /// a second time and not expired as though it were unspent authorization.
    pub prepaid_bn: Amounts,
    pub spent_ytd_bn: Amounts,
    pub spent_today_bn: Amounts,
    /// Running costs of new civilian lines share a capital-department pool but
    /// are not another unit of national capital formation.
    pub noncapital_spent_today_bn: Amounts,
    pub prepaid_used_today_bn: Amounts,
    pub accrued_today_bn: Amounts,
    pub expired_authority_bn: f64,
    pub day: Option<i32>,
    pub settled_day: Option<i32>,
    pub basis_gdp: f64,
    pub fraction: f64,
    pub revenue_today_bn: f64,
    pub interest_today_bn: f64,
    pub fiscal_staged: bool,
    /// Settled public investment, annualized at that day's GDP/calendar basis.
    /// The next day's economy reads this: no partial mid-tick investment bill.
    pub realized_investment_share: f64,
    /// Latest posted ministry expense annualized, excluding already-paid
    /// procurement. Kept separately so an open day cannot leak partial bills
    /// into the Treasury run-rate card.
    #[serde(default)]
    pub settled_spending_annual_bn: f64,
}

pub fn enrolled(w: &WorldState, nation: NationId) -> bool {
    w.nation_opt(nation)
        .is_some_and(|n| n.program_budget.is_some())
}
pub fn enabled(w: &WorldState, nation: NationId) -> bool {
    enrolled(w, nation)
}

pub fn validation(
    w: &WorldState,
    nation: NationId,
    year: i32,
    allocations: &[f64; BUDGET_MINISTRIES],
    departments: &Shares,
) -> Option<String> {
    if !clock::is_daily(w) {
        return Some("Department programmes require daily simulation.".into());
    }
    if w.player != Some(nation) {
        return Some("Only the player may enroll a department budget.".into());
    }
    if !w.nation_opt(nation).is_some_and(|n| n.alive) {
        return Some("The sponsoring government no longer exists.".into());
    }
    if year != w.year {
        return Some(format!(
            "The {year} budget cannot be enacted in {}.",
            w.year
        ));
    }
    if allocations
        .iter()
        .enumerate()
        .any(|(i, v)| !v.is_finite() || *v < 0.0 || *v > BUDGET_CAPS[i])
    {
        return Some("A ministry is outside its budget range.".into());
    }
    if allocations.iter().sum::<f64>() > 0.70 {
        return Some("The state cannot budget more than 70% of GDP.".into());
    }
    if departments
        .iter()
        .any(|row| row.iter().map(|v| *v as u32).sum::<u32>() != 10_000)
    {
        return Some(
            "Each ministry's five departments must total exactly 10,000 basis points (100%)."
                .into(),
        );
    }
    None
}

pub fn department_price(
    w: &WorldState,
    nation: NationId,
    year: i32,
    allocations: &[f64; BUDGET_MINISTRIES],
    departments: &Shares,
) -> f64 {
    let n = w.nation(nation);
    let Some(plan) = &n.program_budget else {
        return 6.0;
    };
    let movement: f64 = (0..BUDGET_MINISTRIES)
        .map(|m| {
            (0..DEPARTMENTS)
                .map(|d| {
                    (departments[m][d] as f64 - plan.departments[m][d] as f64).abs() / 10_000.0
                })
                .sum::<f64>()
                * allocations[m].max(0.0)
                * 100.0
        })
        .sum();
    let current = n.budget_for(w.year);
    let top_unchanged = allocations
        .iter()
        .zip(current.allocations)
        .all(|(a, b)| a.to_bits() == b.to_bits());
    // The enclosing annual vote already charges reopening when its top-level
    // shares move. A combined ministry+department vote pays that toll once.
    movement
        + if movement > 1e-9 && plan.fiscal_year == year && top_unchanged {
            4.0
        } else {
            0.0
        }
}

/// Called only after the enclosing annual command has validated and seated its
/// top-level plan. Re-enactment changes future accrual, never today's balance.
pub(crate) fn install(w: &mut WorldState, nation: NationId, year: i32, departments: Shares) {
    let n = w.nation_mut(nation);
    if let Some(plan) = &mut n.program_budget {
        plan.fiscal_year = year;
        plan.departments = departments;
        return;
    }
    let mut prepaid = ZERO;
    prepaid[BUDGET_DEFENSE][3] = n.arsenal.banked.max(0.0);
    n.arsenal.banked = 0.0;
    let allocation = n.budget_for(year).allocations;
    let operating_investment = [BUDGET_INFRASTRUCTURE, BUDGET_INDUSTRY, BUDGET_SCIENCE]
        .into_iter()
        .map(|m| {
            (0..DEPARTMENTS)
                .filter(|d| !is_capital(m, *d))
                .map(|d| allocation[m] * departments[m][d] as f64 / 10_000.0)
                .sum::<f64>()
        })
        .sum();
    n.program_budget = Some(ProgramBudget {
        fiscal_year: year,
        departments,
        reference_departments: default_departments(),
        authority_year: year,
        available_bn: ZERO,
        prepaid_bn: prepaid,
        spent_ytd_bn: ZERO,
        spent_today_bn: ZERO,
        noncapital_spent_today_bn: ZERO,
        prepaid_used_today_bn: ZERO,
        accrued_today_bn: ZERO,
        expired_authority_bn: 0.0,
        day: None,
        settled_day: None,
        basis_gdp: 0.0,
        fraction: 0.0,
        revenue_today_bn: 0.0,
        interest_today_bn: 0.0,
        fiscal_staged: false,
        realized_investment_share: operating_investment,
        settled_spending_annual_bn: 0.0,
    });
}

fn open(
    plan: &mut ProgramBudget,
    year: i32,
    day: i32,
    gdp: f64,
    fraction: f64,
    allocation: &[f64; BUDGET_MINISTRIES],
) {
    if plan.day == Some(day) {
        return;
    }
    if plan.authority_year != year {
        plan.expired_authority_bn += plan.available_bn.iter().flatten().sum::<f64>();
        plan.available_bn = ZERO;
        plan.spent_ytd_bn = ZERO;
        plan.authority_year = year;
    }
    plan.day = Some(day);
    plan.basis_gdp = gdp.max(0.0);
    plan.fraction = fraction;
    plan.spent_today_bn = ZERO;
    plan.noncapital_spent_today_bn = ZERO;
    plan.prepaid_used_today_bn = ZERO;
    plan.accrued_today_bn = ZERO;
    plan.revenue_today_bn = 0.0;
    plan.interest_today_bn = 0.0;
    plan.fiscal_staged = false;
    for m in 0..BUDGET_MINISTRIES {
        // Last child gets the exact floating remainder, not five separately
        // rounded pots whose sum can exceed the parent.
        let total = allocation[m] * plan.basis_gdp * fraction;
        let mut assigned = 0.0;
        for d in 0..DEPARTMENTS {
            let amount = if d == DEPARTMENTS - 1 {
                (total - assigned).max(0.0)
            } else {
                total * plan.departments[m][d] as f64 / 10_000.0
            };
            assigned += amount;
            if is_capital(m, d) {
                if plan.fiscal_year == year {
                    plan.accrued_today_bn[m][d] = amount;
                    plan.available_bn[m][d] += amount;
                }
            } else {
                // Existing services continue at their standing appropriation;
                // capital work needs a renewed fiscal-year authorization.
                plan.accrued_today_bn[m][d] = amount;
                plan.spent_today_bn[m][d] = amount;
                plan.spent_ytd_bn[m][d] += amount;
            }
        }
    }
}

pub fn begin_day(w: &mut WorldState) {
    if !clock::is_daily(w) {
        return;
    }
    let year = w.year;
    let day = clock::absolute_day(w);
    let fraction = clock::year_fraction(w);
    for n in w
        .nations
        .iter_mut()
        .filter(|n| n.alive && n.program_budget.is_some())
    {
        let allocation = n.budget_for(year).allocations;
        let gdp = n.gdp;
        open(
            n.program_budget.as_mut().unwrap(),
            year,
            day,
            gdp,
            fraction,
            &allocation,
        );
    }
}

fn projected(w: &WorldState, nation: NationId) -> Option<ProgramBudget> {
    let n = w.nation_opt(nation)?;
    let mut plan = n.program_budget.clone()?;
    if clock::is_daily(w) && n.alive {
        open(
            &mut plan,
            w.year,
            clock::absolute_day(w),
            n.gdp,
            clock::year_fraction(w),
            &n.budget_for(w.year).allocations,
        );
    }
    Some(plan)
}

pub fn available_bn(w: &WorldState, nation: NationId, ministry: usize, department: usize) -> f64 {
    if !is_capital(ministry, department) {
        return 0.0;
    }
    projected(w, nation).map_or(0.0, |p| {
        p.available_bn[ministry][department] + p.prepaid_bn[ministry][department]
    })
}

/// Commit only after material preflight succeeds. Callers use the same exact
/// amount they preflighted; no asynchronous mutation occurs between the two.
pub fn spend(
    w: &mut WorldState,
    nation: NationId,
    ministry: usize,
    department: usize,
    amount_bn: f64,
) -> Result<(), String> {
    if !is_capital(ministry, department) || !amount_bn.is_finite() || amount_bn < 0.0 {
        return Err("Invalid department expenditure.".into());
    }
    let today = clock::absolute_day(w);
    let plan = w
        .nation_mut(nation)
        .program_budget
        .as_mut()
        .ok_or("No department budget is enrolled.")?;
    if plan.day != Some(today) || plan.settled_day == Some(today) {
        return Err("The department funding day is not open.".into());
    }
    let available = plan.available_bn[ministry][department] + plan.prepaid_bn[ministry][department];
    if amount_bn > available {
        return Err("Insufficient shared department funding.".into());
    }
    let prepaid = plan.prepaid_bn[ministry][department].min(amount_bn);
    let fresh = amount_bn - prepaid;
    plan.prepaid_bn[ministry][department] -= prepaid;
    plan.available_bn[ministry][department] -= fresh;
    plan.prepaid_used_today_bn[ministry][department] += prepaid;
    plan.spent_today_bn[ministry][department] += fresh;
    plan.spent_ytd_bn[ministry][department] += fresh;
    Ok(())
}

/// Industrial wages/operating costs use the same finite authority and the same
/// one fiscal posting, but do not masquerade as newly built capital.
pub fn spend_operating(
    w: &mut WorldState,
    nation: NationId,
    ministry: usize,
    department: usize,
    amount_bn: f64,
) -> Result<(), String> {
    let before = w.nation(nation).program_budget.as_ref().map_or(0.0, |p| {
        if ministry < BUDGET_MINISTRIES && department < DEPARTMENTS {
            p.spent_today_bn[ministry][department]
        } else {
            0.0
        }
    });
    spend(w, nation, ministry, department, amount_bn)?;
    let p = w.nation_mut(nation).program_budget.as_mut().unwrap();
    p.noncapital_spent_today_bn[ministry][department] +=
        p.spent_today_bn[ministry][department] - before;
    Ok(())
}

/// Stage tax/interest from the same opening GDP as authorization. The fiscal
/// engine remains the sole source of rates; production owns neither rates nor cash.
pub(crate) fn stage_fiscal(n: &mut Nation, revenue_share: f64, annual_interest_bn: f64) {
    if let Some(p) = &mut n.program_budget {
        p.revenue_today_bn = revenue_share * p.basis_gdp * p.fraction;
        p.interest_today_bn = annual_interest_bn * p.fraction;
        p.fiscal_staged = true;
    }
}

pub fn finish_day(w: &mut WorldState) {
    if !clock::is_daily(w) {
        return;
    }
    let today = clock::absolute_day(w);
    let mut bills = vec![];
    for n in w.nations.iter_mut().filter(|n| n.program_budget.is_some()) {
        let p = n.program_budget.as_mut().unwrap();
        if p.day != Some(today) || p.settled_day == Some(today) || !p.fiscal_staged {
            continue;
        }
        let spent = p.spent_today_bn.iter().flatten().sum::<f64>();
        let net = spent + p.interest_today_bn - p.revenue_today_bn;
        let investment: f64 = [BUDGET_INFRASTRUCTURE, BUDGET_INDUSTRY, BUDGET_SCIENCE]
            .into_iter()
            .map(|m| {
                (0..DEPARTMENTS)
                    .map(|d| p.spent_today_bn[m][d] - p.noncapital_spent_today_bn[m][d])
                    .sum::<f64>()
            })
            .sum();
        p.realized_investment_share = if p.basis_gdp > 0.0 && p.fraction > 0.0 {
            investment / (p.basis_gdp * p.fraction)
        } else {
            0.0
        };
        p.settled_spending_annual_bn = if p.fraction > 0.0 {
            spent / p.fraction
        } else {
            0.0
        };
        p.settled_day = Some(today);
        bills.push((n.id, net, net / p.basis_gdp.max(0.1)));
    }
    for (id, net, share) in bills {
        crate::economy::charge(w, id, net, share);
    }
}

pub fn effective_investment(n: &Nation, legacy_share: f64) -> f64 {
    if n.program_budget.is_some() {
        if let Some(inherited) = n.province_investment_reference {
            // Province accounting prices actual explicit work/output itself.
            // Its inherited public capital remains in the macro baseline, but
            // newly funded projects cannot enter that same channel again.
            return inherited;
        }
    }
    n.program_budget
        .as_ref()
        .map_or(legacy_share, |p| p.realized_investment_share)
}

/// A reporting run-rate, not an appropriation or a second fiscal charge. Before
/// the first closed day an enrolled government has no posted programme expense.
pub fn fiscal_spending_share(n: &Nation, legacy_share: f64) -> f64 {
    n.program_budget.as_ref().map_or(legacy_share, |p| {
        if n.gdp > 0.0 {
            p.settled_spending_annual_bn / n.gdp
        } else {
            0.0
        }
    })
}

/// Existing operating arms follow their protected service departments. Capital
/// authorization is not simultaneously a fully delivered maintenance service.
/// Default preset fractions preserve the old reference-anchored arm's scale.
pub fn service_gap(n: &Nation, ministry: usize, allocation: f64) -> f64 {
    let reference = n
        .annual_budget
        .as_ref()
        .map_or(allocation, |b| b.reference[ministry]);
    let Some(p) = &n.program_budget else {
        return allocation - reference;
    };
    match ministry {
        BUDGET_INDUSTRY => 0.0,
        BUDGET_INFRASTRUCTURE => {
            allocation * p.departments[ministry][4] as f64 / 2000.0 - reference
        }
        BUDGET_SCIENCE => {
            allocation
                * p.departments[ministry][1..]
                    .iter()
                    .map(|v| *v as f64)
                    .sum::<f64>()
                / 8000.0
                - reference
        }
        _ => allocation - reference,
    }
}

pub fn procurement_share(n: &Nation) -> f64 {
    n.program_budget
        .as_ref()
        .map_or(crate::arsenal::PROCUREMENT_SHARE, |p| {
            p.departments[BUDGET_DEFENSE][3] as f64 / 10_000.0
        })
}

/// Personnel, operations and military research sustain force support. The game
/// preset assigns these 60% together, preserving the inherited force curve at
/// that split while making transfers into equipment/supply real tradeoffs.
pub fn force_support_share(n: &Nation, defense_allocation: f64) -> f64 {
    n.program_budget.as_ref().map_or(defense_allocation, |p| {
        defense_allocation
            * [0, 1, 4]
                .into_iter()
                .map(|d| p.departments[BUDGET_DEFENSE][d] as f64)
                .sum::<f64>()
            / 6000.0
    })
}

/// Replace Industry's magazine arm on enrolled plans with Defense maintenance.
/// Reference is the default department fraction of the inherited Defense plan.
pub fn refill_multiplier(n: &Nation, defense_allocation: Option<f64>) -> f64 {
    match &n.program_budget {
        None => crate::ministries::industry_refill(n.budget_gap(BUDGET_INDUSTRY)),
        Some(p) => {
            let b = n
                .annual_budget
                .as_ref()
                .expect("program budget owns an annual plan");
            let now = defense_allocation.unwrap_or(b.allocations[BUDGET_DEFENSE])
                * p.departments[BUDGET_DEFENSE][2] as f64
                / 10_000.0;
            let baseline = b.reference[BUDGET_DEFENSE]
                * p.reference_departments[BUDGET_DEFENSE][2] as f64
                / 10_000.0;
            crate::ministries::industry_refill((now - baseline) / 0.20)
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct DepartmentPreview {
    pub ministry: usize,
    pub department: usize,
    pub name: &'static str,
    pub capital: bool,
    pub mode: &'static str,
    pub share_bp: u16,
    pub annual_bn: f64,
    pub daily_bn: f64,
    pub available_bn: f64,
    pub spent_today_bn: f64,
    pub spent_ytd_bn: f64,
    pub carry_bn: f64,
    pub prepaid_bn: f64,
}
#[derive(Clone, Debug, Serialize)]
pub struct ProgramPreview {
    pub enabled: bool,
    pub fiscal_year: i32,
    pub renewed: bool,
    pub basis_gdp: f64,
    pub departments: Shares,
    pub defaults: Shares,
    pub rows: Vec<DepartmentPreview>,
    pub annual_authorized_bn: f64,
    pub daily_authorized_bn: f64,
    pub actual_spent_today_bn: f64,
    pub capital_available_bn: f64,
    pub expired_authority_bn: f64,
    pub realized_investment_share: f64,
    pub political_cost: f64,
    pub last_spending_day: Option<i32>,
    pub spending_fiscal_year: Option<i32>,
    pub defense_force: f64,
    pub magazine_refill_mult: f64,
    pub note: &'static str,
}
pub fn preview(w: &WorldState, nation: NationId) -> ProgramPreview {
    let n = w.nation(nation);
    let budget = n.budget_for(w.year);
    let p = projected(w, nation);
    let actual = n.program_budget.as_ref();
    let departments = p
        .as_ref()
        .map_or_else(default_departments, |p| p.departments);
    let gdp = p.as_ref().map_or(n.gdp, |p| p.basis_gdp);
    let fraction = clock::year_fraction(w);
    let mut rows = vec![];
    for m in 0..BUDGET_MINISTRIES {
        for d in 0..DEPARTMENTS {
            let annual = budget.allocations[m] * gdp * departments[m][d] as f64 / 10_000.0;
            let capital = is_capital(m, d);
            let (available, prepaid, accrual) = p.as_ref().map_or((0.0, 0.0, 0.0), |p| {
                (
                    p.available_bn[m][d],
                    p.prepaid_bn[m][d],
                    p.accrued_today_bn[m][d],
                )
            });
            // Forecast availability may include the next day. Historical spending
            // MUST come from the live ledger, never from projected service delivery.
            let today = actual.map_or(0.0, |p| p.spent_today_bn[m][d]);
            let ytd = actual
                .filter(|p| p.authority_year == w.year)
                .map_or(0.0, |p| p.spent_ytd_bn[m][d]);
            rows.push(DepartmentPreview {
                ministry: m,
                department: d,
                name: NAMES[m][d],
                capital,
                mode: if capital {
                    "shared_project_pool"
                } else {
                    "managed_service"
                },
                share_bp: departments[m][d],
                annual_bn: annual,
                daily_bn: annual * fraction,
                available_bn: available + prepaid,
                spent_today_bn: today,
                spent_ytd_bn: ytd,
                carry_bn: (available - accrual).max(0.0),
                prepaid_bn: prepaid,
            });
        }
    }
    ProgramPreview { enabled:p.is_some(), fiscal_year:p.as_ref().map_or(w.year, |p|p.fiscal_year),
        renewed:p.as_ref().is_some_and(|p|p.fiscal_year == w.year), basis_gdp:gdp, departments, defaults:default_departments(),
        annual_authorized_bn:budget.total() * gdp, daily_authorized_bn:budget.total() * gdp * fraction,
        actual_spent_today_bn:rows.iter().map(|r|r.spent_today_bn).sum(), capital_available_bn:rows.iter().map(|r|r.available_bn).sum(),
        expired_authority_bn:actual.map_or(0.0, |p|p.expired_authority_bn),
        realized_investment_share:p.as_ref().map_or(n.state_invest_gdp, |p|p.realized_investment_share), political_cost:0.0, rows,
        last_spending_day:actual.and_then(|p|p.day), spending_fiscal_year:actual.map(|p|p.authority_year),
        defense_force:crate::war::sustained_force(n,n.mil_spend_gdp), magazine_refill_mult:refill_multiplier(n,None),
        note:"Game-preset departments. Services are managed together; capital pools fund real work. Annual dollars are a GDP-based run-rate, not a fixed cash cap. Imported materials are paid separately. Realized investment reaches the economy on the following day.",
    }
}

pub fn preview_with_plan(
    w: &WorldState,
    nation: NationId,
    allocations: [f64; BUDGET_MINISTRIES],
    departments: Shares,
) -> Result<ProgramPreview, String> {
    if let Some(reason) = validation(w, nation, w.year, &allocations, &departments) {
        return Err(reason);
    }
    let command = crate::Command::SetProgramBudget {
        nation,
        fiscal_year: w.year,
        allocations,
        departments,
    };
    let price = crate::command_price(w, &command).map_or(0.0, |(_, price, _)| price);
    let mut copy = w.clone();
    crate::dispatch(&mut copy, &command)?;
    let mut result = preview(&copy, nation);
    result.political_cost = price;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{apply_command, init::world_1990, load, save, Command};

    fn prepared() -> WorldState {
        let mut w = world_1990(GameRules {
            daily_simulation: true,
            ..GameRules::default()
        });
        w.player = Some(NationId::USA);
        w.nation_mut(NationId::USA).political_capital = 1000.0;
        w
    }
    fn enroll(w: &mut WorldState) {
        let allocations = w.nation(NationId::USA).budget_for(w.year).allocations;
        apply_command(
            w,
            &Command::SetProgramBudget {
                nation: NationId::USA,
                fiscal_year: w.year,
                allocations,
                departments: default_departments(),
            },
        )
        .unwrap();
    }
    fn near(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-9, "{a} != {b}");
    }
    fn total(a: &Amounts) -> f64 {
        a.iter().flatten().sum()
    }
    fn settle(w: &mut WorldState) {
        stage_fiscal(w.nation_mut(NationId::USA), 0.0, 0.0);
        finish_day(w);
    }

    #[test]
    fn unenrolled_default_and_daily_worlds_remain_byte_identical() {
        for daily in [false, true] {
            let mut w = world_1990(GameRules {
                daily_simulation: daily,
                ..GameRules::default()
            });
            let before = save(&w);
            begin_day(&mut w);
            finish_day(&mut w);
            assert_eq!(save(&w), before);
            assert!(w.nations.iter().all(|n| n.program_budget.is_none()));
            assert!(!before.contains("program_budget"));
        }
    }

    #[test]
    fn child_totals_equal_parent_and_preview_never_accrues_real_authority() {
        let mut w = prepared();
        enroll(&mut w);
        let before = save(&w);
        let p = preview(&w, NationId::USA);
        for m in 0..BUDGET_MINISTRIES {
            assert_eq!(
                p.departments[m].iter().map(|x| *x as u32).sum::<u32>(),
                10_000
            );
            near(
                p.rows
                    .iter()
                    .filter(|r| r.ministry == m)
                    .map(|r| r.annual_bn)
                    .sum(),
                w.nation(NationId::USA).budget_for(w.year).allocations[m] * p.basis_gdp,
            );
        }
        for _ in 0..5 {
            preview(&w, NationId::USA);
        }
        assert_eq!(save(&w), before);
        let quote = available_bn(&w, NationId::USA, BUDGET_INDUSTRY, 0);
        begin_day(&mut w);
        near(quote, available_bn(&w, NationId::USA, BUDGET_INDUSTRY, 0));
        let once = save(&w);
        begin_day(&mut w);
        assert_eq!(once, save(&w));
    }

    #[test]
    fn shared_pool_and_fiscal_settlement_spend_each_dollar_once() {
        let mut w = prepared();
        enroll(&mut w);
        let n = w.nation_mut(NationId::USA);
        n.treasury_bn = Some(1000.0);
        n.debt_bn = Some(0.0);
        n.debt_gdp = 0.0;
        begin_day(&mut w);
        let pool = available_bn(&w, NationId::USA, BUDGET_INDUSTRY, 0);
        spend(&mut w, NationId::USA, BUDGET_INDUSTRY, 0, pool * 0.60).unwrap();
        assert_eq!(
            w.nation(NationId::USA).treasury_bn,
            Some(1000.0),
            "authority consumption is not a second cash debit"
        );
        let before = save(&w);
        assert!(spend(&mut w, NationId::USA, BUDGET_INDUSTRY, 0, pool * 0.60).is_err());
        assert_eq!(save(&w), before, "rejected overspend is atomic");
        let left = available_bn(&w, NationId::USA, BUDGET_INDUSTRY, 0);
        spend(&mut w, NationId::USA, BUDGET_INDUSTRY, 0, left).unwrap();
        near(available_bn(&w, NationId::USA, BUDGET_INDUSTRY, 0), 0.0);
        let charged = total(
            &w.nation(NationId::USA)
                .program_budget
                .as_ref()
                .unwrap()
                .spent_today_bn,
        );
        settle(&mut w);
        near(
            w.nation(NationId::USA).treasury_bn.unwrap(),
            1000.0 - charged,
        );
        let once = save(&w);
        finish_day(&mut w);
        assert_eq!(save(&w), once);
        assert!(
            spend(&mut w, NationId::USA, BUDGET_INDUSTRY, 1, 0.0).is_err(),
            "a closed day cannot receive late expenses"
        );
    }

    #[test]
    fn unused_capital_is_not_spent_or_given_a_growth_reward() {
        let mut w = prepared();
        enroll(&mut w);
        begin_day(&mut w);
        let appropriation = preview(&w, NationId::USA).daily_authorized_bn;
        let plan = w.nation(NationId::USA).program_budget.as_ref().unwrap();
        let operating = total(&plan.spent_today_bn);
        assert!(operating < appropriation);
        near(operating + total(&plan.available_bn), appropriation);
        settle(&mut w);
        let n = w.nation(NationId::USA);
        assert!(effective_investment(n, n.state_invest_gdp) < n.state_invest_gdp);
        let prior = effective_investment(n, n.state_invest_gdp);
        clock::advance_date(&mut w);
        begin_day(&mut w);
        let pool = available_bn(&w, NationId::USA, BUDGET_INDUSTRY, 0);
        spend(&mut w, NationId::USA, BUDGET_INDUSTRY, 0, pool).unwrap();
        assert_eq!(
            effective_investment(w.nation(NationId::USA), 0.0),
            prior,
            "mid-tick partial spending is not the macro input"
        );
        settle(&mut w);
        assert!(effective_investment(w.nation(NationId::USA), 0.0) > prior);
    }

    #[test]
    fn re_enactment_and_invalid_commands_cannot_mint_or_lose_balances() {
        let mut w = prepared();
        enroll(&mut w);
        begin_day(&mut w);
        let original = w.nation(NationId::USA).program_budget.clone();
        enroll(&mut w);
        assert_eq!(w.nation(NationId::USA).program_budget, original);
        let allocations = w.nation(NationId::USA).budget_for(w.year).allocations;
        let before = save(&w);
        let mut bad = default_departments();
        bad[5][0] = 2001;
        assert!(apply_command(
            &mut w,
            &Command::SetProgramBudget {
                nation: NationId::USA,
                fiscal_year: 1990,
                allocations,
                departments: bad
            }
        )
        .is_err());
        assert_eq!(save(&w), before);
        assert!(apply_command(
            &mut w,
            &Command::SetProgramBudget {
                nation: NationId::USA,
                fiscal_year: 1991,
                allocations,
                departments: default_departments()
            }
        )
        .is_err());
        assert_eq!(save(&w), before);
        let mut nan = allocations;
        nan[0] = f64::NAN;
        assert!(apply_command(
            &mut w,
            &Command::SetProgramBudget {
                nation: NationId::USA,
                fiscal_year: 1990,
                allocations: nan,
                departments: default_departments()
            }
        )
        .is_err());
        assert_eq!(save(&w), before);
        assert!(apply_command(
            &mut w,
            &Command::SetStateInvest {
                nation: NationId::USA,
                share: 0.2
            }
        )
        .is_err());
        assert_eq!(save(&w), before);
    }

    #[test]
    fn all_calendar_months_accrue_one_twelfth_with_constant_gdp() {
        for (year, month, days) in [(1991, 2, 28), (1992, 2, 29), (1990, 4, 30), (1990, 1, 31)] {
            let mut w = prepared();
            w.year = year;
            w.month = month;
            w.day = 1;
            enroll(&mut w);
            let gdp = w.nation(NationId::USA).gdp;
            let annual =
                w.nation(NationId::USA).budget_for(year).allocations[BUDGET_INDUSTRY] * gdp / 5.0;
            for _ in 0..days {
                begin_day(&mut w);
                settle(&mut w);
                clock::advance_date(&mut w);
            }
            let p = w.nation(NationId::USA).program_budget.as_ref().unwrap();
            near(p.available_bn[BUDGET_INDUSTRY][0], annual / 12.0);
            assert_eq!(w.day, 1);
        }
    }

    #[test]
    fn rollover_expires_authority_but_never_refunds_cash_or_prepaid_orders() {
        let mut w = prepared();
        w.month = 12;
        w.day = 31;
        w.nation_mut(NationId::USA).arsenal.banked = 2.0;
        enroll(&mut w);
        begin_day(&mut w);
        let old = total(
            &w.nation(NationId::USA)
                .program_budget
                .as_ref()
                .unwrap()
                .available_bn,
        );
        settle(&mut w);
        let treasury = w.nation(NationId::USA).treasury_bn;
        clock::advance_date(&mut w);
        begin_day(&mut w);
        let p = w.nation(NationId::USA).program_budget.as_ref().unwrap();
        near(total(&p.available_bn), 0.0);
        near(p.expired_authority_bn, old);
        assert_eq!(p.prepaid_bn[BUDGET_DEFENSE][3], 2.0);
        assert_eq!(w.nation(NationId::USA).treasury_bn, treasury);
        assert!(!preview(&w, NationId::USA).renewed);
        enroll(&mut w); // Renewal after accrual never opens the same day twice.
        near(
            total(
                &w.nation(NationId::USA)
                    .program_budget
                    .as_ref()
                    .unwrap()
                    .available_bn,
            ),
            0.0,
        );
        clock::advance_date(&mut w);
        begin_day(&mut w);
        assert!(available_bn(&w, NationId::USA, BUDGET_INDUSTRY, 0) > 0.0);
    }

    #[test]
    fn inherited_procurement_bank_is_spent_without_being_charged_again() {
        let mut w = prepared();
        w.nation_mut(NationId::USA).arsenal.banked = 3.0;
        enroll(&mut w);
        begin_day(&mut w);
        assert_eq!(w.nation(NationId::USA).arsenal.banked, 0.0);
        let before = total(
            &w.nation(NationId::USA)
                .program_budget
                .as_ref()
                .unwrap()
                .spent_today_bn,
        );
        spend(&mut w, NationId::USA, BUDGET_DEFENSE, 3, 2.0).unwrap();
        let p = w.nation(NationId::USA).program_budget.as_ref().unwrap();
        assert_eq!(p.prepaid_bn[BUDGET_DEFENSE][3], 1.0);
        near(total(&p.spent_today_bn), before);
    }

    #[test]
    fn save_load_preserves_open_day_authority_spend_and_settlement_guards() {
        let mut a = prepared();
        enroll(&mut a);
        begin_day(&mut a);
        let share = available_bn(&a, NationId::USA, BUDGET_INDUSTRY, 0) / 2.0;
        spend(&mut a, NationId::USA, BUDGET_INDUSTRY, 0, share).unwrap();
        let mut b = load(&save(&a)).unwrap();
        for w in [&mut a, &mut b] {
            begin_day(w);
            spend(w, NationId::USA, BUDGET_INDUSTRY, 0, share).unwrap();
            settle(w);
            clock::advance_date(w);
            begin_day(w);
            settle(w);
        }
        assert_eq!(save(&a), save(&b));
    }

    #[test]
    fn draft_preview_is_unpriced_statewise_and_refill_moves_to_defense_only() {
        let mut w = prepared();
        enroll(&mut w);
        let mut allocation = w.nation(NationId::USA).budget_for(w.year).allocations;
        let baseline = refill_multiplier(w.nation(NationId::USA), None);
        allocation[BUDGET_INDUSTRY] += 0.01;
        let before = save(&w);
        let p = preview_with_plan(&w, NationId::USA, allocation, default_departments()).unwrap();
        assert!(p.political_cost > 0.0);
        assert_eq!(before, save(&w));
        crate::dispatch(
            &mut w,
            &Command::SetProgramBudget {
                nation: NationId::USA,
                fiscal_year: 1990,
                allocations: allocation,
                departments: default_departments(),
            },
        )
        .unwrap();
        assert_eq!(refill_multiplier(w.nation(NationId::USA), None), baseline);
        assert!(
            refill_multiplier(
                w.nation(NationId::USA),
                Some(allocation[BUDGET_DEFENSE] + 0.02)
            ) > baseline
        );
    }

    #[test]
    fn civilian_operating_spend_is_cash_cost_but_not_new_macro_capital() {
        let mut a = prepared();
        enroll(&mut a);
        begin_day(&mut a);
        let mut b = a.clone();
        let amount = available_bn(&a, NationId::USA, BUDGET_INDUSTRY, 0);
        spend_operating(&mut b, NationId::USA, BUDGET_INDUSTRY, 0, amount).unwrap();
        settle(&mut a);
        settle(&mut b);
        assert_eq!(
            a.nation(NationId::USA)
                .program_budget
                .as_ref()
                .unwrap()
                .realized_investment_share,
            b.nation(NationId::USA)
                .program_budget
                .as_ref()
                .unwrap()
                .realized_investment_share
        );
        let apos =
            a.nation(NationId::USA).treasury_bn.unwrap() - a.nation(NationId::USA).debt_bn.unwrap();
        let bpos =
            b.nation(NationId::USA).treasury_bn.unwrap() - b.nation(NationId::USA).debt_bn.unwrap();
        near(apos - bpos, amount);
    }

    #[test]
    fn full_daily_world_batches_and_saved_continuation_are_identical() {
        let mut a = prepared();
        enroll(&mut a);
        let mut b = a.clone();
        for day in 0..31 {
            crate::tick_day(&mut a, &[]);
            if day == 13 {
                a = load(&save(&a)).unwrap();
            }
        }
        crate::tick_month(&mut b, &[]);
        assert_eq!((a.year, a.month, a.day), (1990, 2, 1));
        assert_eq!(save(&a), save(&b));
        let p = a.nation(NationId::USA).program_budget.as_ref().unwrap();
        assert_eq!(p.settled_day, Some(clock::absolute_day(&a) - 1));
        assert!(total(&p.spent_ytd_bn) > 0.0);
        assert!(
            a.nation(NationId::USA)
                .arsenal
                .orders
                .iter()
                .any(|o| o.units > 0.0),
            "equipment procurement consumes its funded pool through the real arsenal"
        );
    }

    #[test]
    fn dated_budget_reallocation_replays_without_duplicate_accrual() {
        let mut a = prepared();
        enroll(&mut a);
        let mut b = a.clone();
        let allocation = a.nation(NationId::USA).budget_for(1990).allocations;
        let mut departments = default_departments();
        departments[BUDGET_INDUSTRY] = [6000, 1000, 1000, 1000, 1000];
        let change = Command::SetProgramBudget {
            nation: NationId::USA,
            fiscal_year: 1990,
            allocations: allocation,
            departments,
        };
        for day in 0..40 {
            let commands = if day == 15 {
                std::slice::from_ref(&change)
            } else {
                &[]
            };
            crate::tick_day(&mut a, commands);
            crate::tick_day(&mut b, commands);
            if day == 15 {
                b = load(&save(&b)).unwrap();
            }
            assert_eq!(save(&a), save(&b));
        }
    }

    #[test]
    fn next_day_and_draft_previews_never_report_unposted_spending() {
        let mut w = prepared();
        enroll(&mut w);
        assert_eq!(preview(&w, NationId::USA).actual_spent_today_bn, 0.0);
        begin_day(&mut w);
        let amount = available_bn(&w, NationId::USA, BUDGET_INDUSTRY, 0) * 0.4;
        spend(&mut w, NationId::USA, BUDGET_INDUSTRY, 0, amount).unwrap();
        settle(&mut w);
        let posted = preview(&w, NationId::USA);
        let day = clock::absolute_day(&w);
        clock::advance_date(&mut w);
        let before = save(&w);
        let next = preview(&w, NationId::USA);
        assert_eq!(next.actual_spent_today_bn, posted.actual_spent_today_bn);
        assert_eq!(next.last_spending_day, Some(day));
        assert!(next.capital_available_bn > posted.capital_available_bn);
        let mut departments = default_departments();
        departments[BUDGET_INDUSTRY] = [6000, 1000, 1000, 1000, 1000];
        let draft = preview_with_plan(
            &w,
            NationId::USA,
            w.nation(NationId::USA).budget_for(w.year).allocations,
            departments,
        )
        .unwrap();
        for view in [&next, &draft] {
            for (actual, row) in posted.rows.iter().zip(&view.rows) {
                assert_eq!(row.spent_today_bn, actual.spent_today_bn);
                assert_eq!(row.spent_ytd_bn, actual.spent_ytd_bn);
            }
        }
        assert_eq!(save(&w), before);
        // At a year boundary old spending is still dated correctly, but it
        // must never be relabeled as current-year spending by a forecast.
        w.year = 1991;
        w.month = 1;
        w.day = 1;
        let january = preview(&w, NationId::USA);
        assert!(january.rows.iter().all(|r| r.spent_ytd_bn == 0.0));
        assert_eq!(january.spending_fiscal_year, Some(1990));
        assert_eq!(january.actual_spent_today_bn, posted.actual_spent_today_bn);
    }

    #[test]
    fn defense_department_transfers_change_real_force_and_preview_together() {
        let mut w = prepared();
        let n = w.nation(NationId::USA);
        assert_eq!(
            force_support_share(n, n.mil_spend_gdp).to_bits(),
            n.mil_spend_gdp.to_bits()
        );
        let inherited = crate::war::sustained_force(n, n.mil_spend_gdp);
        enroll(&mut w);
        let baseline = preview(&w, NationId::USA);
        near(baseline.defense_force, inherited);
        let allocations = w.nation(NationId::USA).budget_for(w.year).allocations;
        let mut departments = default_departments();
        departments[BUDGET_DEFENSE] = [0, 0, 0, 10000, 0];
        let draft = preview_with_plan(&w, NationId::USA, allocations, departments).unwrap();
        assert!(
            draft.defense_force < baseline.defense_force,
            "equipment does not also buy force operating support"
        );
        assert!(draft.magazine_refill_mult < baseline.magazine_refill_mult);
        crate::dispatch(
            &mut w,
            &Command::SetProgramBudget {
                nation: NationId::USA,
                fiscal_year: 1990,
                allocations,
                departments,
            },
        )
        .unwrap();
        let n = w.nation(NationId::USA);
        assert_eq!(
            draft.defense_force,
            crate::war::sustained_force(n, n.mil_spend_gdp)
        );
        assert_eq!(draft.magazine_refill_mult, refill_multiplier(n, None));
        assert_eq!(procurement_share(n), 1.0);
        assert_eq!(force_support_share(n, n.mil_spend_gdp), 0.0);
    }

    #[test]
    fn fiscal_card_annualizes_only_posted_actual_expenses_not_authority() {
        let mut w = prepared();
        assert_eq!(fiscal_spending_share(w.nation(NationId::USA), 0.35), 0.35);
        enroll(&mut w);
        begin_day(&mut w);
        assert_eq!(fiscal_spending_share(w.nation(NationId::USA), 0.35), 0.0);
        let amount = available_bn(&w, NationId::USA, BUDGET_INDUSTRY, 0) * 0.5;
        spend(&mut w, NationId::USA, BUDGET_INDUSTRY, 0, amount).unwrap();
        let p = w.nation(NationId::USA).program_budget.as_ref().unwrap();
        let annual = total(&p.spent_today_bn) / p.fraction;
        settle(&mut w);
        near(
            fiscal_spending_share(w.nation(NationId::USA), 0.35) * w.nation(NationId::USA).gdp,
            annual,
        );
        assert!(annual < preview(&w, NationId::USA).annual_authorized_bn);
        clock::advance_date(&mut w);
        begin_day(&mut w);
        let available = available_bn(&w, NationId::USA, BUDGET_INDUSTRY, 0);
        spend(&mut w, NationId::USA, BUDGET_INDUSTRY, 0, available).unwrap();
        near(
            fiscal_spending_share(w.nation(NationId::USA), 0.35) * w.nation(NationId::USA).gdp,
            annual,
        );
    }
}
