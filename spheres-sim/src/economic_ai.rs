//! Opt-in civilian economic competition. These are MODEL decision policies,
//! never historical starting endowments or additional production multipliers.
//! Every enactment, project and purchase uses the same priced commands as play.
use crate::{
    clock, economy, industrial_modules, industry,
    industry_planning::{self, CapacityPlan, GoodsBalance, ProvinceCapacity},
    production::{self, ProjectKind as K},
    programs,
    resources::{self, Commodity},
    world::*,
    Command,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A strategic review is not a daily instruction flood. Physical work, freight
/// and fiscal settlement continue on every intervening day.
pub const REVIEW_DAYS: i32 = 30;
/// Supply is planned farther ahead than the strategic review cadence. One-off
/// construction recipes still enter once; only operating use spans this window.
pub const SUPPLY_HORIZON_DAYS: i32 = 90;
/// Fixed strategic raw-material windows. Monthly rates use 12/365 below, so
/// the WATCH window is exactly twelve policy months.
pub const RAW_HORIZON_DAYS: [i32; 3] = [30, 90, 365];
pub const CIVILIAN_QUEUE_LIMIT: usize = 2;
const PC_RESERVE: f64 = 8.0;
/// MODEL startup inventory for the first machine shop: one basic planning
/// month. This is an AI reservation target, never public recurring demand.
const MACHINERY_STARTER_PACKS: f64 = 15.0;

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct NationPlan {
    pub last_review_day: i32,
    pub fiscal_year: i32,
    pub evaluations: u32,
    pub action: String,
    pub reason: String,
    pub project: Option<u32>,
    pub district: Option<String>,
    pub project_kind: Option<K>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity_micros: Option<u32>,
    #[serde(default)]
    pub offered_reserves: [Option<f64>; 2],
    #[serde(default)]
    pub funding: Option<FundingHorizon>,
    /// Snapshot taken at this government's latest strategic review. Keeping it
    /// avoids recomputing 137 country plans in a browser read and records the
    /// supply position after the AI's latest action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supply_review: Option<SupplyForecast>,
    /// Raw-material snapshot captured by the latest strategic review. Old
    /// saves default to no snapshot; a passive/default world writes nothing.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw_supply_review: Option<RawSupplyForecast>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SupplyForecast {
    pub as_of_day: i32,
    pub horizon_days: i32,
    pub lines: Vec<SupplyLine>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SupplyLine {
    pub good: crate::commerce::Good,
    pub operating_daily: f64,
    pub project_remaining: f64,
    pub startup_reserve: f64,
    pub target: f64,
    pub stock: f64,
    pub imports: f64,
    pub domestic_contracts: f64,
    /// Positive output on the most recently settled industry date. Capacity
    /// that is blocked, unfinished or merely estimated is not called supply.
    pub recent_domestic_daily: f64,
    pub projected_domestic: f64,
    pub coverage: f64,
    pub shortage: f64,
    pub storage_capacity: f64,
    pub storage_headroom: f64,
    pub status: String,
    pub reason: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RawSupplyForecast {
    pub as_of_day: i32,
    pub horizons_days: [i32; 3],
    pub lines: Vec<RawSupplyLine>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RawSupplyLine {
    pub commodity: Commodity,
    pub civilian_operating_daily: f64,
    pub military_recurring_monthly: f64,
    pub project_remaining: f64,
    pub mine_remaining: f64,
    pub materials_remaining: f64,
    pub finite_remaining: f64,
    pub demand: [f64; 3],
    pub stock: f64,
    /// Opening warehouse stock retained for domestic use after executable
    /// export dispatches claim the combined supply pool in source order.
    pub allocable_stock: [f64; 3],
    /// Total executable outbound contract dispatch across opening stock,
    /// domestic production, pending arrivals, and new contract arrivals.
    /// This is a conserved prior claim, not merely a stock reservation.
    pub prior_claims: [f64; 3],
    /// Paid freight arrivals retained after the same outbound claim waterfall.
    pub pending: [f64; 3],
    pub net_domestic_monthly: f64,
    pub contracted_in_monthly: f64,
    /// Domestic production retained after the same outbound claim waterfall.
    pub domestic_coverage: [f64; 3],
    /// Newly dispatched contract arrivals retained after the same outbound
    /// claim waterfall. Already-dispatched cargo remains solely in `pending`.
    pub contract_coverage: [f64; 3],
    pub coverage: [f64; 3],
    pub shortage: [f64; 3],
    pub immediate_draw: f64,
    pub immediate_shortage: f64,
    pub blocked_now: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocker_reason: Option<String>,
    pub status: String,
    pub reason: String,
}

/// Immutable, pure opening-of-review inputs shared by every nation in one AI
/// decision wave. Live project and Materials demand is still read per nation;
/// only derived resource production, calendar fractions, and the expensive
/// all-contract physical projection are snapshotted here.
pub struct RawSupplyContext {
    resource_have: resources::Have,
    contract_supply: resources::ContractSupplyForecast,
    flow_fractions: [f64; 3],
}

impl RawSupplyContext {
    pub fn new(w: &WorldState) -> Self {
        let resource_have = resources::have(w).into_owned();
        let contract_supply = resources::contract_supply_forecast_with(
            w,
            &resource_have,
            RAW_HORIZON_DAYS,
        );
        let flow_fractions = std::array::from_fn(|h| {
            resources::annual_flow_fraction_for_days(w, RAW_HORIZON_DAYS[h])
        });
        Self {
            resource_have,
            contract_supply,
            flow_fractions,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FundingHorizon {
    pub as_of_day: i32,
    pub annual_authority_bn: f64,
    pub available_authority_bn: f64,
    pub remaining_work_cost_bn: f64,
    pub funding_years: Option<f64>,
    pub unshared_work_years: f64,
    pub earliest_years: Option<f64>,
    pub basis: String,
}

/// Optimistic lower bounds, not a promised delivery date. Available authority
/// is shared; today's GDP/appropriation is held constant and renewed each year.
/// Missing imports, priority competition and war can only extend this horizon.
pub fn funding_horizon(
    w: &WorldState,
    nation: NationId,
    district: &str,
    kind: K,
) -> FundingHorizon {
    let n = w.nation(nation);
    let spec = production::catalog(kind);
    let project =
        production::projects_for(w, nation).find(|p| p.district == district && p.kind == kind);
    let spent = project
        .and_then(|p| w.production.industry.projects.get(&p.id))
        .map_or(0.0, |f| f.spent_bn);
    let module = (kind == K::StarterIndustry).then(|| {
        industrial_modules::quote(
            w,
            nation,
            district,
            project
                .and_then(|p| p.capacity_micros)
                .unwrap_or_else(|| module_order_capacity(w, nation, district)),
        )
    });
    let cost = module
        .as_ref()
        .map_or(industry::work_cost_bn(kind), |q| q.cost_bn);
    let remaining = (cost - spent).max(0.0);
    let remaining_days = project.map_or(spec.total_days as f64, |p| {
        (p.total_days as f64 - p.progress_days).max(0.0)
    });
    let shares = n
        .program_budget
        .as_ref()
        .map_or_else(programs::default_departments, |p| p.departments);
    let part = if kind == K::Infrastructure {
        shares[spec.funding_ministry][..4]
            .iter()
            .map(|v| *v as f64 / 10_000.0)
            .sum()
    } else {
        shares[spec.funding_ministry][production::funding_department(kind)] as f64 / 10_000.0
    };
    let annual = n.budget_for(w.year).allocations[spec.funding_ministry] * n.gdp * part;
    let available = industry::project_authority(w, nation, kind);
    let funding = if annual > 0.0 {
        Some((remaining - available).max(0.0) / annual)
    } else if remaining <= available {
        Some(0.0)
    } else {
        None
    };
    let rate = (production::construction_capacity(w, nation)
        * (1.0 + production::level(w, district, K::Infrastructure) as f64 * 0.1))
        .min(1.5);
    let work_years = if let Some(q) = &module {
        let progress = project.map_or(0.0, |p| p.progress_fraction());
        // The 90-day commissioning limit also caps normalized work per day;
        // pausing does not bank additional construction throughput.
        (remaining_days * q.scale / rate.max(1e-12))
            .max(q.minimum_calendar_days as f64 * (1.0 - progress))
            / 365.0
    } else {
        remaining_days / rate.max(1e-12) / 365.0
    };
    FundingHorizon {
        as_of_day: clock::absolute_day(w),
        annual_authority_bn: annual,
        available_authority_bn: available,
        remaining_work_cost_bn: remaining,
        funding_years: funding,
        unshared_work_years: work_years,
        earliest_years: funding.map(|v| v.max(work_years)),
        basis: if module.is_some() {
            "Optimistic lower bound for the frozen module size, at current GDP and renewed allocation. Shared work, raw inputs, freight and conflict can extend it; commissioning takes at least 90 funded work dates.".into()
        } else {
            "Optimistic lower bound at current GDP and renewed annual allocation. Authority and workforce are shared; raw inputs, goods, freight and conflict may extend it. A positive paid slice is not a completed industrial site.".into()
        },
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct EconomicAi {
    pub nations: BTreeMap<NationId, NationPlan>,
}
impl EconomicAi {
    pub fn is_empty(&self) -> bool {
        self.nations.is_empty()
    }
}

pub fn enabled(w: &WorldState) -> bool {
    w.rules.economic_competition && clock::is_daily(w)
}

/// Simulation authorization only. The browser independently requires its
/// authenticated player actor; enabling opponents never gives it their seat.
pub fn may_direct(w: &WorldState, nation: NationId) -> bool {
    w.player == Some(nation) || (enabled(w) && w.nation_opt(nation).is_some_and(|n| n.alive))
}

fn record(
    w: &mut WorldState,
    nation: NationId,
    action: &str,
    mut reason: String,
    candidate: Option<(String, K)>,
    raw_context: &RawSupplyContext,
) {
    let day = clock::absolute_day(w);
    let year = w.year;
    let project = match candidate.as_ref() {
        Some((d, k)) => production::projects_for(w, nation)
            .find(|p| p.district == *d && p.kind == *k)
            .map(|p| p.id),
        None => production::projects_for(w, nation).next().map(|p| p.id),
    };
    let funding = candidate
        .as_ref()
        .map(|(district, kind)| funding_horizon(w, nation, district, *kind));
    if let Some(f) = &funding {
        if let Some(years) = f.earliest_years {
            if years > 5.0 {
                reason.push_str(&format!(" Even with inputs available, current funding/capacity needs at least {:.1} years; {}.",years,
                    if candidate.as_ref().is_some_and(|(_,k)| *k == K::StarterIndustry) { "the ordered capacity and cost remain fixed" }
                    else { "this is a full-size site, not a subsidized micro-factory" }));
            }
        } else {
            reason.push_str(" No annual funding is assigned to finish this work.");
        }
    }
    let capacity_micros = candidate
        .as_ref()
        .filter(|(_, k)| *k == K::StarterIndustry)
        .map(|(d, _)| {
            production::projects_for(w, nation)
                .find(|p| p.district == *d && p.kind == K::StarterIndustry)
                .and_then(|p| p.capacity_micros)
                .unwrap_or_else(|| module_order_capacity(w, nation, d))
        });
    let supply_review = supply_forecast(w, nation);
    let raw_supply_review = raw_supply_forecast_with_context(w, nation, raw_context);
    let p = w.economic_ai.nations.entry(nation).or_default();
    p.last_review_day = day;
    p.fiscal_year = year;
    p.evaluations = p.evaluations.saturating_add(1);
    p.action = action.into();
    p.reason = reason;
    p.project = project;
    p.district = candidate.as_ref().map(|(d, _)| d.clone());
    p.project_kind = candidate.map(|(_, k)| k);
    p.capacity_micros = capacity_micros;
    p.funding = funding;
    p.supply_review = Some(supply_review);
    p.raw_supply_review = Some(raw_supply_review);
}

fn execute(w: &mut WorldState, command: &Command) -> Result<(), String> {
    if let Some((nation, price, _)) = crate::command_price(w, command) {
        let held = w.nation(nation).political_capital;
        if price > 0.0 && held < price + PC_RESERVE {
            return Err(format!(
                "Saving political capital: {:.1} held; {:.1} command cost plus {:.0} reserve.",
                held, price, PC_RESERVE
            ));
        }
    }
    crate::apply_command(w, command)
}

/// Same preset for every country, proportional to its actual ministry budget.
/// One current bottleneck gets most authority; other departments retain a
/// working allowance, including power and materials used by operating plants.
fn departments(w: &WorldState, nation: NationId, target: Option<K>) -> programs::Shares {
    let mut shares = w
        .nation(nation)
        .program_budget
        .as_ref()
        .map_or_else(programs::default_departments, |p| p.departments);
    if let Some(kind) = target {
        if production::catalog(kind).funding_ministry == BUDGET_INDUSTRY {
            shares[BUDGET_INDUSTRY] = [1000; 5];
            shares[BUDGET_INDUSTRY][production::funding_department(kind)] = 6000;
        }
    }
    shares
}

/// Conservative modeled budget response: never increase the aggregate envelope
/// automatically. Hot debt trims at most 5% of each existing ministry per
/// review toward revenue less interest; peaceful debt permits a 2%-GDP deficit.
/// Taxes use the ordinary priced command, not a hidden treasury side door.
fn fiscal_command(
    w: &WorldState,
    nation: NationId,
    renewal: bool,
    target: Option<K>,
) -> Option<Command> {
    let n = w.nation(nation);
    let current = n.budget_for(w.year);
    let mut allocations = current.allocations;
    if n.program_budget.is_some() && !renewal {
        let terms = economy::growth_terms(
            n,
            n.state_invest_gdp,
            n.interest_rate,
            &economy::Conditions::of(w, nation),
        );
        let fiscal = economy::Fiscal::of(n, &terms);
        let revenue = fiscal.revenue_gdp;
        let limit = (revenue - economy::interest_gdp(n)
            + if n.debt_gdp > 0.85 { 0.0 } else { 0.02 })
        .max(0.0);
        let total = current.total();
        if n.debt_gdp > 0.85 && fiscal.balance_gdp < -0.01 && total > limit + 0.01 {
            // The existing fiscal AI's tax response survives, but is now a
            // command with a political price and strategic review cadence.
            if n.tax_rate < 0.55 && n.tax_rate + 0.005 < total + economy::interest_gdp(n) {
                return Some(Command::SetTaxRate {
                    nation,
                    rate: (n.tax_rate + 0.01).min(0.55),
                });
            }
            let factor = (limit / total.max(1e-12)).clamp(0.95, 1.0);
            for allocation in &mut allocations {
                *allocation *= factor;
            }
        }
    }
    // Renew the standing plan before proposing a different project mix. An
    // unaffordable new priority must not block a zero-cost annual renewal.
    let desired = if renewal {
        n.program_budget
            .as_ref()
            .map_or_else(|| departments(w, nation, target), |p| p.departments)
    } else {
        departments(w, nation, target)
    };
    let changed = allocations != current.allocations
        || n.program_budget
            .as_ref()
            .is_some_and(|p| p.departments != desired);
    if renewal || n.program_budget.is_none() || changed {
        Some(Command::SetProgramBudget {
            nation,
            fiscal_year: w.year,
            allocations,
            departments: desired,
        })
    } else {
        None
    }
}

fn queued(w: &WorldState, nation: NationId, district: &str, kind: K) -> bool {
    production::projects_for(w, nation).any(|p| p.district == district && p.kind == kind)
}

/// Standard-package affordability chooses the planning path, not a discount.
/// Before enrollment only, use the proposed 60% factory focus. Every actual
/// order is sized from the enacted budget by the core quote helper.
fn standard_module_affordable(w: &WorldState, nation: NationId) -> bool {
    let n = w.nation(nation);
    let part = n
        .program_budget
        .as_ref()
        .map_or(0.6, |p| p.departments[BUDGET_INDUSTRY][0] as f64 / 10_000.0);
    n.gdp * n.budget_for(w.year).allocations[BUDGET_INDUSTRY] * part
        >= industry::work_cost_bn(K::StarterIndustry)
}

fn goods_balance(plan: &CapacityPlan, good: crate::commerce::Good) -> &GoodsBalance {
    plan.goods
        .iter()
        .find(|row| row.good == good)
        .expect("capacity plan contains both manufactured goods")
}

fn committed_supply_reserve(
    w: &WorldState,
    nation: NationId,
    good: crate::commerce::Good,
) -> f64 {
    use crate::commerce::Good;
    production::projects_for(w, nation)
        .filter(|p| w.districts.get(&p.district) == Some(&nation))
        .map(|p| match (p.kind, good) {
            // A first machine needs one operating month waiting when it opens.
            (K::MachineryWorks, Good::Intermediates) => MACHINERY_STARTER_PACKS,
            (K::ResearchCenter, Good::Intermediates) =>
                industry::PROTOTYPE_INTERMEDIATES_PER_LEVEL_DAY * REVIEW_DAYS as f64,
            (K::ResearchCenter, Good::CapitalGoods) =>
                industry::PROTOTYPE_CAPITAL_PER_LEVEL_DAY * REVIEW_DAYS as f64,
            _ => 0.0,
        })
        .sum()
}

/// A bounded, read-only supply forecast shared by AI decisions and the Exchange.
/// Remaining project goods are counted once; only installed operating demand is
/// extended from the existing 30-day ledger to ninety days. Paid imports,
/// domestic contracts and recent actual output count as coverage, never as stock.
pub fn supply_forecast(w: &WorldState, nation: NationId) -> SupplyForecast {
    use crate::commerce::{self, Good};
    let today = clock::absolute_day(w);
    let prospective_machine_reserve = prospective_first_machine_reserve(w, nation);
    let recent = w.production.industry.last_day
        .filter(|day| *day == today || *day == today - 1);
    let line = |good: Good| {
        let operating = commerce::recurring_demand_daily(w, nation, good).max(0.0);
        let current = commerce::demand(w, nation, good).max(0.0);
        let project = (current - operating * REVIEW_DAYS as f64).max(0.0);
        let startup = committed_supply_reserve(w, nation, good)
            + if good == Good::Intermediates {
                prospective_machine_reserve
            } else {
                0.0
            };
        let target = project + operating * SUPPLY_HORIZON_DAYS as f64 + startup;
        let stock = commerce::stock(w, nation, good);
        let imports = commerce::pending(w, nation, good);
        let contracts = if good == Good::Intermediates {
            crate::materials::pending(w, nation)
        } else {
            0.0
        };
        let actual_daily: f64 = if recent.is_some() {
            w.production.industry.operations.iter().filter(|op| {
                w.districts.get(&op.district) == Some(&nation)
                    && op.output_daily > 0.0
                    && match good {
                        Good::Intermediates => matches!(op.kind, K::ProcessingPlant | K::StarterIndustry),
                        Good::CapitalGoods => op.kind == K::MachineryWorks,
                    }
            }).map(|op| op.output_daily).sum()
        } else { 0.0 };
        let projected = (actual_daily * SUPPLY_HORIZON_DAYS as f64).min(target);
        let coverage = stock + imports + contracts + projected;
        let shortage = (target - coverage).max(0.0);
        let storage = industry::goods_capacity(w, nation);
        // Paid inbound lots already claim warehouse room even though they are
        // not called stock. Report the same usable headroom the order policy
        // applies, so the Exchange cannot imply that room is still free.
        let headroom = (storage - stock - imports - contracts).max(0.0);
        let (status, reason) = if target <= 1e-9 {
            ("idle", "No installed operating use or unfinished project currently needs this good.".into())
        } else if shortage <= 1e-9 {
            ("covered", format!("Stock, paid incoming supply and recent actual domestic output cover the {}-day plan.", SUPPLY_HORIZON_DAYS))
        } else {
            ("replenish", format!("The {}-day plan is short {:.4} packs after stock, paid incoming supply, finite domestic contracts and recent actual output.", SUPPLY_HORIZON_DAYS, shortage))
        };
        SupplyLine { good, operating_daily: operating, project_remaining: project,
            startup_reserve: startup, target, stock, imports, domestic_contracts: contracts,
            recent_domestic_daily: actual_daily, projected_domestic: projected,
            coverage, shortage, storage_capacity: storage, storage_headroom: headroom,
            status: status.into(), reason }
    };
    SupplyForecast { as_of_day: today, horizon_days: SUPPLY_HORIZON_DAYS,
        lines: vec![line(Good::Intermediates), line(Good::CapitalGoods)] }
}

fn actual_raw_blocker(
    w: &WorldState,
    nation: NationId,
    commodity: Commodity,
) -> Option<String> {
    let key = commodity.name().to_ascii_lowercase();
    let names_resource = |reason: &str| {
        let reason = reason.to_ascii_lowercase();
        reason.contains(&key)
            && (reason.contains("need")
                || reason.contains("missing")
                || reason.contains("limits")
                || reason.contains("waiting")
                || reason.contains("no longer available"))
    };

    if let Some(reason) = w
        .manufacturing
        .lines
        .iter()
        .filter(|line| line.nation == nation)
        .filter(|line| {
            matches!(
                line.status,
                crate::manufacturing::LineStatus::Slowed
                    | crate::manufacturing::LineStatus::Paused
                    | crate::manufacturing::LineStatus::Blocked
            )
        })
        .filter_map(|line| line.reason.as_deref())
        .find(|reason| names_resource(reason))
    {
        return Some(reason.into());
    }
    if let Some(reason) = production::projects_for(w, nation)
        .filter(|project| {
            matches!(
                project.status,
                production::ProjectStatus::Slowed
                    | production::ProjectStatus::Paused
                    | production::ProjectStatus::Blocked
            )
        })
        .filter_map(|project| project.reason.as_deref())
        .find(|reason| names_resource(reason))
    {
        return Some(reason.into());
    }
    if let Some(reason) = w
        .resources
        .mine_projects
        .iter()
        .filter(|project| project.started_by == nation)
        .filter_map(|project| {
            w.production
                .industry
                .mines
                .get(&industry::mine_key(&project.district, project.commodity))
        })
        .filter(|funding| funding.last_day.is_some())
        .filter_map(|funding| funding.reason.as_deref())
        .find(|reason| names_resource(reason))
    {
        return Some(reason.into());
    }
    if let Some(reason) = w.materials.as_ref().and_then(|materials| {
        materials
            .orders
            .iter()
            .filter(|order| {
                order.nation == nation && order.last_day == materials.last_day
            })
            .filter(|order| matches!(order.status.as_str(), "limited" | "paused" | "blocked"))
            .filter_map(|order| order.reason.as_deref())
            .find(|reason| names_resource(reason))
    }) {
        return Some(reason.into());
    }
    if w.production.industry.last_day.is_some() {
        if let Some(reason) = w
            .production
            .industry
            .operations
            .iter()
            .filter(|operation| {
                w.districts.get(&operation.district) == Some(&nation)
                    && matches!(operation.status.as_str(), "limited" | "paused" | "blocked")
            })
            .filter_map(|operation| operation.reason.as_deref())
            .find(|reason| names_resource(reason))
        {
            return Some(reason.into());
        }
    }
    if let Some(stall) = w
        .nation_opt(nation)
        .and_then(|nation| nation.arsenal.last_resource_stall.as_ref())
        .filter(|stall| stall.commodity == commodity)
    {
        return Some(stall.reason.clone());
    }
    None
}

/// Pure strategic view over the existing twelve-line physical economy. It
/// grants and reserves nothing: every supply component remains in its owner
/// ledger until the ordinary settlement moves or consumes it.
pub fn raw_supply_forecast(w: &WorldState, nation: NationId) -> RawSupplyForecast {
    let context = RawSupplyContext::new(w);
    raw_supply_forecast_with_context(w, nation, &context)
}

/// Context-sharing variant for a deterministic multi-nation AI review wave.
/// The context is a read-only opening snapshot; this function still reads each
/// nation's current committed consumers and actual blocker receipts.
pub fn raw_supply_forecast_with_context(
    w: &WorldState,
    nation: NationId,
    context: &RawSupplyContext,
) -> RawSupplyForecast {
    let today = clock::absolute_day(w);
    let resource_have = &context.resource_have;
    let contract_supply = &context.contract_supply;
    let flow_fractions = context.flow_fractions;
    let civilian = industry::raw_demand_components(w, nation);
    let military = resources::recurring_procurement_draw(w, nation);
    let materials_remaining = crate::materials::resource_reserve(w, nation);
    let immediate = resources::tick_draw(w, nation);
    let round = |value: f64| (value.max(0.0) * 1e9).round() / 1e9;

    let mut lines = Vec::with_capacity(resources::ALL.len());
    for commodity in resources::ALL {
        let i = commodity.idx();
        let project_remaining = civilian.projects_remaining[i];
        let mine_remaining = civilian.mines_remaining[i];
        let materials_remaining = materials_remaining[i];
        let finite_remaining =
            round(project_remaining + mine_remaining + materials_remaining);

        if commodity == Commodity::Oil {
            lines.push(RawSupplyLine {
                commodity,
                civilian_operating_daily: 0.0,
                military_recurring_monthly: 0.0,
                project_remaining: 0.0,
                mine_remaining: 0.0,
                materials_remaining: 0.0,
                finite_remaining: 0.0,
                demand: [0.0; 3],
                stock: 0.0,
                allocable_stock: [0.0; 3],
                prior_claims: [0.0; 3],
                pending: [0.0; 3],
                net_domestic_monthly: round(
                    resources::flow_from(resource_have, nation, commodity) / 12.0,
                ),
                contracted_in_monthly: 0.0,
                domestic_coverage: [0.0; 3],
                contract_coverage: [0.0; 3],
                coverage: [0.0; 3],
                shortage: [0.0; 3],
                immediate_draw: 0.0,
                immediate_shortage: 0.0,
                blocked_now: false,
                blocker_reason: None,
                status: "informational".into(),
                reason: "Oil remains a priced national flow. It is shown for context but is never stored, spot-cleared, or consumed through the physical raw warehouse.".into(),
            });
            continue;
        }

        let stock = round(resources::stockpile(w, nation, commodity));
        let gross_pending: [f64; 3] = std::array::from_fn(|h| {
            round(crate::logistics::pending_within_days(
                w,
                nation,
                commodity,
                RAW_HORIZON_DAYS[h],
            ))
        });
        let gross_domestic: [f64; 3] = std::array::from_fn(|h| {
            round(
                resources::flow_from(resource_have, nation, commodity).max(0.0)
                    * flow_fractions[h],
            )
        });
        let outgoing: [f64; 3] = std::array::from_fn(|h| {
            contract_supply.outbound[nation.index()][commodity.idx()][h]
        });
        let gross_contract_coverage: [f64; 3] = std::array::from_fn(|h| {
            contract_supply.inbound[nation.index()][commodity.idx()][h]
        });
        let prior_claims: [f64; 3] = std::array::from_fn(|h| {
            round(
                outgoing[h].min(
                    stock
                        + gross_domestic[h]
                        + gross_pending[h]
                        + gross_contract_coverage[h],
                ),
            )
        });
        let allocable_stock: [f64; 3] =
            std::array::from_fn(|h| round((stock - outgoing[h].min(stock)).max(0.0)));
        let domestic_coverage: [f64; 3] = std::array::from_fn(|h| {
            let after_stock = (outgoing[h] - stock).max(0.0);
            round((gross_domestic[h] - after_stock).max(0.0))
        });
        let pending: [f64; 3] = std::array::from_fn(|h| {
            let after_stock_and_domestic =
                (outgoing[h] - stock - gross_domestic[h]).max(0.0);
            round((gross_pending[h] - after_stock_and_domestic).max(0.0))
        });
        let contract_coverage: [f64; 3] = std::array::from_fn(|h| {
            let after_earlier_sources =
                (outgoing[h] - stock - gross_domestic[h] - gross_pending[h]).max(0.0);
            round((gross_contract_coverage[h] - after_earlier_sources).max(0.0))
        });
        let demand: [f64; 3] = std::array::from_fn(|h| {
            let horizon_days = RAW_HORIZON_DAYS[h] as f64;
            let funded_days = if programs::enrolled(w, nation) {
                industry::funded_days_in_horizon(w, nation, RAW_HORIZON_DAYS[h])
            } else {
                // Legacy, non-enrolled procurement retains its existing
                // standing monthly policy; only department-funded consumers
                // stop at an enacted fiscal programme's expiry.
                horizon_days
            };
            // Civilian operating recipes are daily. Military appropriations
            // are policy-monthly: funded_days*12/365 makes a fully funded
            // WATCH exactly 12 months without inventing next-year authority.
            let recurring = civilian.operating_daily[i] * funded_days
                + military[i] * funded_days * 12.0 / 365.0;
            // Remaining bills are disclosed above, while only the currently
            // executable daily slice is paced into each independent horizon.
            // This is a committed bill, not a promised completion ETA.
            let projects = civilian.projects_horizon[i][h];
            let mines = civilian.mines_horizon[i][h];
            let materials = materials_remaining.min(
                crate::materials::resource_demand_for_days(
                    w,
                    nation,
                    RAW_HORIZON_DAYS[h],
                )[i],
            );
            round(recurring + projects + mines + materials)
        });
        let coverage: [f64; 3] = std::array::from_fn(|h| {
            round(
                allocable_stock[h]
                    + domestic_coverage[h]
                    + pending[h]
                    + contract_coverage[h],
            )
        });
        let shortage: [f64; 3] =
            std::array::from_fn(|h| round((demand[h] - coverage[h]).max(0.0)));
        let immediate_draw = round(immediate[i]);
        let blocker_reason = actual_raw_blocker(w, nation, commodity);
        let blocked_now = blocker_reason.is_some();
        // A post-settlement empty pile is not evidence of failure: successful
        // last-unit consumption also leaves zero. Quantity becomes a current
        // shortage only when an authoritative consumer recorded a block.
        let immediate_shortage = if blocked_now {
            round((immediate_draw - stock).max(0.0))
        } else {
            0.0
        };
        let (status, lead) = if demand.iter().all(|quantity| *quantity <= 1e-9)
            && finite_remaining <= 1e-9
        {
            ("idle", "No committed consumer currently needs this material.".into())
        } else if blocked_now {
            (
                "short",
                blocker_reason
                    .clone()
                    .unwrap_or_else(|| "A named consumer is blocked now.".into()),
            )
        } else if shortage[0] > 1e-9 {
            ("short", format!(
                "RUN is short {:.3} {} over the next 30 days.",
                shortage[0],
                commodity.unit()
            ))
        } else if shortage[1] > 1e-9 || shortage[2] > 1e-9 {
            let (days, amount) = if shortage[1] > 1e-9 {
                (RAW_HORIZON_DAYS[1], shortage[1])
            } else {
                (RAW_HORIZON_DAYS[2], shortage[2])
            };
            ("watch", format!(
                "The {}-day plan is short {:.3} {}.",
                days,
                amount,
                commodity.unit()
            ))
        } else {
            ("ready", "RUN, PLAN, and WATCH are covered by secured supply.".into())
        };
        let reason = format!(
            "{} Remaining project, mine, and Materials quantities are disclosed as committed bills; each horizon paces only today's executable draw, capped by that remaining bill, once. They are not delivery ETAs. Enrolled recurring and finite work stops at the end of its enacted fiscal authority. Executable export dispatches are netted once across stock, domestic production, paid freight, then new contract arrivals; the retained source rows therefore sum to coverage.",
            lead
        );
        let policy_months_30 = RAW_HORIZON_DAYS[0] as f64 * 12.0 / 365.0;
        lines.push(RawSupplyLine {
            commodity,
            civilian_operating_daily: round(civilian.operating_daily[i]),
            military_recurring_monthly: round(military[i]),
            project_remaining: round(project_remaining),
            mine_remaining: round(mine_remaining),
            materials_remaining: round(materials_remaining),
            finite_remaining,
            demand,
            stock,
            allocable_stock,
            prior_claims,
            pending,
            net_domestic_monthly: round(domestic_coverage[0] / policy_months_30),
            contracted_in_monthly: round(contract_coverage[0] / policy_months_30),
            domestic_coverage,
            contract_coverage,
            coverage,
            shortage,
            immediate_draw,
            immediate_shortage,
            blocked_now,
            blocker_reason,
            status: status.into(),
            reason,
        });
    }
    RawSupplyForecast {
        as_of_day: today,
        horizons_days: RAW_HORIZON_DAYS,
        lines,
    }
}

/// Buy at most the next review's work, even though the warning horizon is
/// ninety days. This gives the AI time to react without filling a warehouse
/// with a blind three-month purchase. Paid inbound lots share the same room.
fn replenishment_quantity(line: &SupplyLine) -> f64 {
    replenishment_quantity_excluding_startup(line, 0.0)
}

fn replenishment_quantity_excluding_startup(line: &SupplyLine, excluded: f64) -> f64 {
    let excluded = excluded.clamp(0.0, line.startup_reserve);
    let next_review = line.project_remaining + (line.startup_reserve - excluded)
        + line.operating_daily * REVIEW_DAYS as f64;
    let order_room = (line.storage_capacity - line.stock - line.imports
        - line.domestic_contracts).max(0.0);
    (line.shortage - excluded)
        .max(0.0)
        .min(next_review)
        .min(order_room)
        .max(0.0)
}

/// The one import lot this review can actually execute under both the market
/// quote and the AI's GDP risk cap. Planning and enactment share this helper so
/// a tiny economy cannot wait forever on a quote its own policy will not buy.
fn goods_import_candidate(
    w: &WorldState,
    nation: NationId,
    good: crate::commerce::Good,
    missing: f64,
) -> Option<(NationId, f64, f64)> {
    use crate::commerce;
    if !commerce::enabled(w) || missing <= 1e-9 {
        return None;
    }
    let budget = w.nation(nation).gdp.max(0.0) * 0.001;
    let desired = missing.min(budget / commerce::reference_price_bn(good));
    let quote = commerce::market_quotes(w, nation, good, desired, REVIEW_DAYS as u32)
        .into_iter()
        .next()?;
    let mut quantity = quote.quantity.min(budget / quote.unit_price_bn);
    if quantity * quote.unit_price_bn > budget {
        quantity *= 1.0 - f64::EPSILON;
    }
    (quantity > 1e-9).then_some((quote.seller, quantity, quote.unit_price_bn))
}

fn expansion_blocker(w: &WorldState, nation: NationId) -> Option<String> {
    let today = clock::absolute_day(w);
    if !w
        .production
        .industry
        .last_day
        .is_some_and(|day| day == today || day == today - 1)
    {
        return None;
    }
    w.production.industry.operations.iter().find_map(|op| {
        if w.districts.get(&op.district)!=Some(&nation)
            || !matches!(op.status.as_str(), "limited" | "paused" | "blocked")
        {
            return None;
        }
        let reason=op.reason.as_deref()?;
        let lower=reason.to_ascii_lowercase();
        (lower.contains("raw input") || lower.contains("generating fuel")
            || lower.contains("complete operating bundle")
            || lower.contains("operating authority") || lower.contains("active department budget"))
            .then(||format!("Restore the existing {} in {} before expanding production: {} No duplicate factory is commissioned to solve an input or operating-budget shortage.",
                production::catalog(op.kind).name,op.district,reason))
    })
}

/// Frozen size for a NEW order. Completed modules retain their original size
/// when GDP, appropriations or ownership subsequently change.
pub fn module_order_capacity(w: &WorldState, nation: NationId, district: &str) -> u32 {
    module_capacity_from_plan(w, nation, district, &industry_planning::plan(w, nation))
}

fn module_capacity_from_plan(
    w: &WorldState,
    nation: NationId,
    district: &str,
    plan: &CapacityPlan,
) -> u32 {
    let recommended = industrial_modules::recommended_capacity_micros(w, nation);
    let n = w.nation(nation);
    let share = n
        .program_budget
        .as_ref()
        .map_or(2000, |p| p.departments[BUDGET_INDUSTRY][0]) as f64
        / 10_000.0;
    let annual = n.gdp * n.budget_for(w.year).allocations[BUDGET_INDUSTRY] * share;
    if !annual.is_finite()
        || annual
            < industry::work_cost_bn(K::StarterIndustry) * recommended as f64
                / industrial_modules::STANDARD_MICROS as f64
    {
        return 0;
    }
    let room = industrial_modules::COMPONENTS
        .iter()
        .map(|k| {
            industrial_modules::MAX_MICROS
                .saturating_sub(industrial_modules::reserved_capacity(w, district, *k))
        })
        .min()
        .unwrap_or(0);
    let has_base = plan.provinces.iter().any(|p| {
        p.estate + p.estate_committed > 0.0
            || p.processing_daily + p.processing_committed_daily > 0.0
            || p.machinery_daily + p.machinery_committed_daily > 0.0
    });
    let output_per_standard = 1.0 + production::level(w, district, K::Automation) as f64 * 0.2;
    let demand_size = if has_base {
        (goods_balance(plan, crate::commerce::Good::Intermediates).expansion_daily
            / output_per_standard
            * industrial_modules::STANDARD_MICROS as f64)
            .floor()
            .min(u32::MAX as f64) as u32
    } else {
        industrial_modules::STANDARD_MICROS
    };
    recommended.min(room).min(demand_size)
}

fn module_candidate(
    w: &WorldState,
    nation: NationId,
    districts: &[String],
    plan: &CapacityPlan,
) -> Result<(String, K, String), String> {
    let balance = goods_balance(plan, crate::commerce::Good::Intermediates);
    if balance.expansion_daily <= 1e-9 {
        return Err("The productive module is an intermediate-goods specialist. Waiting for actual domestic use or delivered export demand before adding affordable capacity; no full-size machinery project is forced.".into());
    }
    if let Some(reason) = expansion_blocker(w, nation) {
        return Err(reason);
    }
    for district in districts {
        if !queued(w, nation, district, K::StarterIndustry)
            && module_capacity_from_plan(w, nation, district, plan) > 0
        {
            return Ok((district.clone(),K::StarterIndustry,
                format!("Add only the remaining {:.4} intermediate packs/day of useful capacity after existing and queued plants, inventory and imports. The new order pays its full scaled cost; prior capacity is unchanged.",balance.expansion_daily)));
        }
    }
    Err("The module's demonstrated demand cannot fit another affordable capacity order in a controlled province. Existing capacity and paid work remain unchanged.".into())
}

/// A proposed factory needs actual local delivery and national generation.
/// Pending utility capacity prevents duplicate orders, but is not live power.
fn factory_support(
    w: &WorldState,
    nation: NationId,
    plan: &CapacityPlan,
    site: &ProvinceCapacity,
    kind: K,
) -> Result<Option<(String, K, String)>, String> {
    let output = (if kind == K::ProcessingPlant { 1.0 } else { 0.5 })
        * (1.0 + production::level(w, &site.district, K::Automation) as f64 * 0.2);
    let added_power = output * industry::power_per_pack(w, &site.district, kind);
    if plan.generation_daily + plan.generation_committed_daily + 1e-9
        < plan.power_required_daily + added_power
    {
        if let Some(d) = plan.provinces.iter().filter(|p| !p.contested).find(|p| {
            !queued(w, nation, &p.district, K::Generation)
                && production::start_project_error(w, nation, &p.district, K::Generation).is_none()
        }) {
            return Ok(Some((d.district.clone(),K::Generation,
                "Generation is short of the existing, queued and proposed industrial load. Add paid supply once; queued generators already count toward the requirement.".into())));
        }
        return Err("Existing and queued generation cannot support another factory, and no eligible generating project can be started.".into());
    }
    if site.grid_daily + site.grid_committed_daily + 1e-9 < site.power_required_daily + added_power
    {
        if queued(w, nation, &site.district, K::PowerGrid) {
            return Err("The selected site's grid expansion is already committed; finish it before commissioning another dependent factory.".into());
        }
        if let Some(reason) =
            production::start_project_error(w, nation, &site.district, K::PowerGrid)
        {
            return Err(format!(
                "The proposed factory needs a grid expansion that cannot start: {reason}"
            ));
        }
        return Ok(Some((site.district.clone(),K::PowerGrid,
            "This selected factory's load exceeds its existing and queued local grid capacity. Empty estates do not receive automatic grid projects.".into())));
    }
    Ok(None)
}

fn factory_candidate(
    w: &WorldState,
    nation: NationId,
    plan: &CapacityPlan,
    kind: K,
    bootstrap: bool,
    gap: f64,
) -> Option<(String, K, String)> {
    let mut sites: Vec<_> = plan
        .provinces
        .iter()
        .filter(|p| !p.contested && p.estate >= 1.0)
        .collect();
    sites.sort_by(|a, b| {
        production::level(w, &b.district, kind)
            .cmp(&production::level(w, &a.district, kind))
            .then(a.district.cmp(&b.district))
    });
    let mut support_fallback = None;
    for site in sites {
        let output = (if kind == K::ProcessingPlant { 1.0 } else { 0.5 })
            * (1.0 + production::level(w, &site.district, K::Automation) as f64 * 0.2);
        if (!bootstrap && output > gap + 1e-9)
            || queued(w, nation, &site.district, kind)
            || production::start_project_error(w, nation, &site.district, kind).is_some()
        {
            continue;
        }
        match factory_support(w, nation, plan, site, kind) {
            Ok(Some(support)) => {
                // Prefer using any already-powered eligible site before
                // buying utility capacity for the first alphabetical estate.
                support_fallback.get_or_insert(support);
                continue;
            }
            Err(_) => continue,
            Ok(None) => {}
        }
        return Some((
            site.district.clone(),
            kind,
            if bootstrap {
                if kind == K::ProcessingPlant {
                "Bootstrap one real raw-input processor for the first machinery consumer; no inherited, fractional or queued processor was overlooked."
            } else {
                "Bootstrap the first raw-input machinery producer. Existing national capital inventory does not already cover useful consumption."
            }.into()
            } else {
                format!("Add {:.3} packs/day within the {:.3} remaining demand gap; installed and queued capacity, stock and imports are already counted.",output,gap)
            },
        ));
    }
    support_fallback
}

fn first_machine_needed(plan: &CapacityPlan) -> bool {
    let capital = goods_balance(plan, crate::commerce::Good::CapitalGoods);
    let cover = capital.stock + capital.incoming;
    capital.installed_daily + capital.committed_daily <= 1e-9
        && !(cover > 1e-9
            && (capital.demand_daily <= 1e-9 || cover >= capital.demand_daily * 90.0))
}

/// A non-player government with a real full-size estate may prepare one paid
/// startup lot before it queues its first machine shop. This intention remains
/// outside public commerce demand: it cannot pull raw goods automatically,
/// grant stock, or impose an AI plan on the player's country. Once machinery
/// is queued, `committed_supply_reserve` owns the same reservation instead.
fn prospective_first_machine_reserve(w: &WorldState, nation: NationId) -> f64 {
    if !enabled(w)
        || w.player == Some(nation)
        || !programs::enrolled(w, nation)
        || production::projects_for(w, nation).any(|p| p.kind == K::MachineryWorks)
        || !w.districts.iter().any(|(district, owner)| {
            *owner == nation
                && !resources::district_contested(w, district)
                && production::level(w, district, K::CivilianIndustry) > 0
        })
    {
        return 0.0;
    }
    first_machine_needed(&industry_planning::plan(w, nation))
        .then_some(MACHINERY_STARTER_PACKS)
        .unwrap_or(0.0)
}

/// A physical plan and its current readiness are separate: changing the budget
/// toward machinery must not make the target oscillate back to a processor.
/// This transient intention is not saved, purchased, or counted as consumption.
struct MaterialsBootstrap {
    command: Command,
    machinery_district: String,
    starts_machine: bool,
    waiting: Option<String>,
}

fn materials_bootstrap(
    w: &WorldState,
    nation: NationId,
    plan: &CapacityPlan,
) -> Option<MaterialsBootstrap> {
    use crate::commerce::{self, Good};
    if !crate::materials::enabled(w) || w.player == Some(nation)
        || !programs::enrolled(w, nation)
    {
        return None;
    }
    let goods = goods_balance(plan, Good::Intermediates);
    let capital = goods_balance(plan, Good::CapitalGoods);
    // Existing/queued processors already supply this bootstrap. Fractional
    // specialists must not be pushed into an unaffordable full machine shop.
    if goods.installed_daily + goods.committed_daily > 1e-9
        || capital.installed_daily > 1e-9
        || !plan.provinces.iter().any(|p| !p.contested && p.estate >= 1.0)
    {
        return None;
    }
    let jobs: Vec<_> = production::projects_for(w, nation).collect();
    if jobs.iter().any(|p| {
        w.districts.get(&p.district) != Some(&nation)
            || resources::district_contested(w, &p.district)
            || !matches!(p.kind, K::Warehouse | K::MachineryWorks)
    }) {
        return None;
    }
    let machine = jobs.iter().find(|p| p.kind == K::MachineryWorks);
    let starts_machine = machine.is_none();
    if starts_machine {
        let occupied = jobs.len() + w.resources.mine_projects.iter()
            .filter(|p| p.started_by == nation).count();
        if !first_machine_needed(plan) || occupied >= CIVILIAN_QUEUE_LIMIT {
            return None;
        }
        if !jobs.is_empty() {
            // Match the warehouse rescue's decision: available capital goods
            // finish that paid job without any first machine or startup lot.
            let missing_capital = commerce::shortage(w, nation, Good::CapitalGoods);
            if missing_capital <= 1e-9 || (commerce::enabled(w)
                && !commerce::market_quotes(w, nation, Good::CapitalGoods, missing_capital, 365).is_empty())
            {
                return None;
            }
        }
    }
    // Existing warehouse work consumes its own packs before commissioning.
    // Net the startup reserve and that real demand against coverage ONCE.
    let target = MACHINERY_STARTER_PACKS + commerce::demand(w, nation, Good::Intermediates);
    let quantity = ((target - goods.stock - goods.incoming - goods.contracted_remaining)
        .max(0.0) * 1e9).ceil() / 1e9;
    if quantity <= 1e-9 || quantity > 1_000_000.0 {
        return None;
    }
    let reserved_raw = crate::materials::resource_reserve(w, nation);
    for source in plan.provinces.iter().filter(|p| !p.contested && p.grid_daily > 0.0) {
        let quote = crate::materials::quote(w, nation, &source.district, quantity, REVIEW_DAYS as u32);
        if !quote.eligible {
            continue;
        }
        let power_per_pack = industry::power_per_pack(w, &source.district, K::ProcessingPlant);
        let raw = industry::operating_recipe(K::ProcessingPlant, quantity, quantity * power_per_pack);
        if resources::ALL.iter().any(|c| raw[c.idx()] >
            (resources::stockpile(w, nation, *c) - reserved_raw[c.idx()]).max(0.0))
        {
            continue;
        }
        // Reuse exactly the ordinary factory support decision, with the NEW
        // Materials load included. Queued utilities/efficiency are not live.
        let mut powered = plan.clone();
        powered.generation_committed_daily = 0.0;
        powered.power_required_daily = 0.0;
        for p in &mut powered.provinces {
            p.grid_committed_daily = 0.0;
            p.power_required_daily = (p.processing_daily + p.processing_committed_daily)
                * industry::power_per_pack(w, &p.district, K::ProcessingPlant)
                + (p.machinery_daily + p.machinery_committed_daily)
                * industry::power_per_pack(w, &p.district, K::MachineryWorks)
                + p.materials_power_required_daily;
            if p.district == source.district {
                let added = quote.reserved_daily * power_per_pack;
                p.materials_power_required_daily += added;
                p.power_required_daily += added;
            }
            powered.power_required_daily += p.power_required_daily;
        }
        if powered.generation_daily + 1e-9 < powered.power_required_daily
            || powered.provinces.iter().any(|p| p.grid_daily + 1e-9 < p.power_required_daily)
        {
            continue;
        }
        let machinery_district = if let Some(p) = machine {
            p.district.clone()
        } else {
            match factory_candidate(w, nation, &powered, K::MachineryWorks, true, 0.0) {
                Some((district, K::MachineryWorks, _)) => district,
                _ => continue, // A promised grid/generator cannot power a lot today.
            }
        };
        if starts_machine && raw_access_reason(w, nation, K::MachineryWorks).is_some() {
            continue;
        }
        let command = Command::OrderMaterials { nation, district: source.district.clone(),
            quantity, delivery_days: REVIEW_DAYS as u32 };
        let project_price = if starts_machine {
            crate::command_price(w, &Command::StartProject { nation,
                district: machinery_district.clone(), kind: K::MachineryWorks })?.1
        } else { 0.0 };
        let needed_pc = quote.political_cost + project_price + PC_RESERVE;
        let waiting = if w.nation(nation).political_capital < needed_pc {
            Some(format!("Saving political capital for the backed Materials startup lot{}: {:.1} held; {:.1} needed including the {:.0} reserve. No order or new machine shop has been signed.",
                if starts_machine { " and first machine shop together" } else { "" },
                w.nation(nation).political_capital, needed_pc, PC_RESERVE))
        } else if quote.feasible_today + 1e-9 < quote.reserved_daily {
            Some(format!("The inherited Materials startup plan is physically backed, but its current daily bundle is not ready: {} No unfunded order or new machine shop has been signed.", quote.blockers.join(" ")))
        } else { None };
        // Stable district order keeps the physical target unchanged through a
        // temporary shared-budget wait; readiness does not invent another site.
        return Some(MaterialsBootstrap { command, machinery_district, starts_machine, waiting });
    }
    None
}

/// Pure next investment, selected from real capacity and input bottlenecks.
/// Alphabetical district ordering is a reproducible tie-break, never a nation
/// whitelist. Unmapped nations remain eligible for budgets and goods trade.
pub fn candidate(w: &WorldState, nation: NationId) -> Result<(String, K, String), String> {
    let districts: Vec<_> = w
        .districts
        .iter()
        .filter(|(d, owner)| **owner == nation && !resources::district_contested(w, d))
        .map(|(d, _)| d.clone())
        .collect();
    if districts.is_empty() {
        return Err(
            "No controlled, uncontested mapped province is available for physical construction."
                .into(),
        );
    }
    let plan = industry_planning::plan(w, nation);
    let estates: Vec<_> = districts
        .iter()
        .filter(|d| production::level(w, d, K::CivilianIndustry) > 0)
        .collect();
    // Acquiring a small module must not replace an established full-size
    // chain's next investment. Only actual integer estates select that path;
    // fractional starter capacity still remains an affordable specialist.
    if estates.is_empty()
        && w.districts
            .iter()
            .any(|(d, n)| *n == nation && industrial_modules::capacity(w, d) > 0.0)
    {
        return module_candidate(w, nation, &districts, &plan);
    }
    if estates.is_empty() {
        if plan
            .provinces
            .iter()
            .any(|p| p.estate + p.estate_committed > 0.0)
        {
            return Err("Existing or queued industrial estates are unavailable for new work. Keep the paid capacity and clear its ownership, conflict or construction blocker rather than buying a duplicate.".into());
        }
        if !standard_module_affordable(w, nation) {
            if module_order_capacity(w, nation, &districts[0]) == 0 {
                return Err("The factory department cannot fund even the minimum module at its current allocation. Existing capacity is unchanged; enact affordable authority before commissioning work.".into());
            }
            return Ok((districts[0].clone(),K::StarterIndustry,
                "Commission affordable, proportional estate, generation, grid and processing together. The module produces intermediates only after paid work and real raw inputs complete; it is not a free full-size factory.".into()));
        }
        return Ok((districts[0].clone(), K::CivilianIndustry,
            "Build the first civilian industrial site and paid construction capacity; no starting factory is invented.".into()));
    }
    use crate::commerce::Good;
    let intermediate = goods_balance(&plan, Good::Intermediates);
    let capital = goods_balance(&plan, Good::CapitalGoods);
    let processing = intermediate.installed_daily + intermediate.committed_daily;
    let first_machine = first_machine_needed(&plan);
    if first_machine {
        if let Some(bootstrap) = materials_bootstrap(w, nation, &plan) {
            return Ok((bootstrap.machinery_district, K::MachineryWorks,
                "Use a backed finite Materials startup lot from existing domestic industry for the first machine shop. Raw inputs, both political prices, operating funds and shared power must be ready before signing; no duplicate processor is required.".into()));
        }
    }
    // Countries can specialize when a consenting, reachable and cash-affordable
    // producer already exists. No synthetic global stock or historical plant
    // is created to make that specialization possible.
    let starter_target = MACHINERY_STARTER_PACKS + if crate::materials::enabled(w) {
        crate::commerce::demand(w, nation, Good::Intermediates)
    } else { 0.0 };
    let paid_coverage =
        intermediate.stock + intermediate.incoming + intermediate.contracted_remaining;
    let import_source = paid_coverage + 1e-9 >= starter_target;
    if first_machine && processing <= 1e-9 && !import_source {
        let missing = (starter_target - paid_coverage).max(0.0);
        if goods_import_candidate(w, nation, Good::Intermediates, missing).is_some() {
            return Err(format!(
                "Accumulate the remaining {:.4} paid intermediate startup packs through the reachable market before commissioning a processor or first machine shop. Partial quotes count; signing and freight remain required.",
                missing
            ));
        }
        if let Some(project) = factory_candidate(w, nation, &plan, K::ProcessingPlant, true, 0.0) {
            return Ok(project);
        }
        return Err("The first machinery consumer needs a real processor or reachable intermediate supplies, but no eligible site can commission the prerequisite. Existing and queued capacity remains counted.".into());
    }
    if first_machine {
        if let Some(project) = factory_candidate(w, nation, &plan, K::MachineryWorks, true, 0.0) {
            return Ok(project);
        }
    }
    // Expansion is not another automatic bootstrap. Count every province,
    // including acquired/module sites and pending output, before buying more.
    let expansion_blocked = expansion_blocker(w, nation);
    if expansion_blocked.is_none() {
        // Historical industrial structure breaks ties between evidenced pack
        // needs. It is not free physical supply or a reason to build without use.
        for kind in industry_planning::expansion_order(&plan) {
            let gap = if kind == K::ProcessingPlant {
                intermediate.expansion_daily
            } else {
                capital.expansion_daily
            };
            if gap <= 1e-9 {
                continue;
            }
            if let Some(project) = factory_candidate(w, nation, &plan, kind, false, gap) {
                return Ok(project);
            }
            if kind == K::ProcessingPlant {
                if let Ok(project) = module_candidate(w, nation, &districts, &plan) {
                    return Ok(project);
                }
            }
        }
    }
    // More space is useful only for a real turnover requirement. Unsold stock
    // alone is a reason to stop producing, not to finance another warehouse.
    let storage = plan.storage + plan.storage_committed;
    if plan.goods.iter().any(|g| {
        g.demand_daily > 1e-9
            && g.stock + g.incoming + g.contracted_remaining >= storage * 0.9
            && g.demand_daily * 90.0 > storage
    }) {
        if let Some(d) = districts.iter().find(|d| {
            !queued(w, nation, d, K::Warehouse)
                && production::start_project_error(w, nation, d, K::Warehouse).is_none()
        }) {
            return Ok((d.clone(),K::Warehouse,
                "Existing and queued storage cannot hold the evidenced turnover buffer. Add storage for actual use, not merely because unsold goods have filled a pile.".into()));
        }
    }
    if industry::research_work_demand(w, nation) > 0.0
        && w.districts
            .iter()
            .filter(|(_, owner)| **owner == nation)
            .all(|(d, _)| production::level(w, d, K::ResearchCenter) == 0)
        && !production::projects_for(w, nation)
            .any(|p| p.kind == K::ResearchCenter && w.districts.get(&p.district) == Some(&nation))
    {
        return Ok((estates[0].to_string(),K::ResearchCenter,
            "Active technology work can use a funded prototype laboratory; completed space alone grants no research or GDP.".into()));
    }
    for (kind, why) in [
        (
            K::Efficiency,
            "Reduce the operating plant's power and fuel inputs.",
        ),
        (
            K::Automation,
            "Increase realized plant throughput where storage and inputs permit it.",
        ),
    ] {
        for site in plan.provinces.iter().filter(|p| !p.contested) {
            let district = &site.district;
            let productive = w.production.industry.operations.iter().any(|op| {
                op.district == *district
                    && op.output_daily > 1e-9
                    && match op.kind {
                        K::ProcessingPlant | K::StarterIndustry => intermediate.demand_daily > 1e-9,
                        K::MachineryWorks => capital.demand_daily > 1e-9,
                        _ => false,
                    }
            });
            if !productive
                || expansion_blocked.is_some()
                || queued(w, nation, district, kind)
                || production::level(w, district, kind) > 0
                || production::start_project_error(w, nation, district, kind).is_some()
            {
                continue;
            }
            if kind == K::Automation {
                let factor = 1.0 + production::level(w, district, K::Automation) as f64 * 0.2;
                let added_intermediates = site.processing_daily / factor * 0.2;
                let added_capital = site.machinery_daily / factor * 0.2;
                if added_intermediates > intermediate.expansion_daily + 1e-9
                    || added_capital > capital.expansion_daily + 1e-9
                {
                    continue;
                }
            }
            return Ok((district.clone(), kind, why.into()));
        }
    }
    Err(expansion_blocked.unwrap_or_else(||format!("Use existing or queued industrial capacity before adding more. Intermediates: {} Capital goods: {} Idle capacity and unsold inventory do not justify duplicate factories.",intermediate.reason,capital.reason)))
}

fn raw_access_reason(w: &WorldState, nation: NationId, kind: K) -> Option<String> {
    for c in resources::ALL {
        if production::catalog(kind).recipe[c.idx()] <= 0.0
            || resources::stockpile(w, nation, c) > 0.0
            || resources::flow(w, nation, c) > 0.0
        {
            continue;
        }
        let reachable = w
            .nations
            .iter()
            .filter(|n| n.alive && n.id != nation)
            .any(|seller| {
                resources::flow(w, seller.id, c) > 0.0
                    && !crate::statecraft::belligerents(w, seller.id, nation)
                    && !w.is_sanctioning(seller.id, nation)
                    && !w.is_sanctioning(nation, seller.id)
                    && w.relation(nation, seller.id) >= resources::relation_floor()
                    && (!w.rules.physical_logistics
                        || crate::logistics::plan(w, seller.id, nation).is_ok())
            });
        if !reachable {
            return Some(format!("No reachable source of {} for {}. A functioning raw-input route is required; no materials are granted.",c.name(),production::catalog(kind).name));
        }
    }
    None
}

pub fn tick(w: &mut WorldState) {
    if !enabled(w) {
        return;
    }
    let mut ids: Vec<_> = w
        .nations
        .iter()
        .filter(|n| n.alive && Some(n.id) != w.player)
        .map(|n| n.id)
        .collect();
    ids.sort();
    if !ids.is_empty() {
        // Rotate who sees scarce export offers first. Nation-id order remains
        // the tie-break, but it is not a permanent rich-first allocation rule.
        let offset = (clock::absolute_day(w).div_euclid(REVIEW_DAYS) as usize) % ids.len();
        ids.rotate_left(offset);
    }
    ids.retain(|nation| review_is_due(w, *nation));
    if ids.is_empty() {
        return;
    }
    let raw_context = RawSupplyContext::new(w);
    for nation in ids {
        evaluate_with_context(w, nation, &raw_context);
    }
}

/// One independently testable country's scheduled review. Does nothing for a
/// player, dead government, disabled rule, or already reviewed date.
pub fn evaluate(w: &mut WorldState, nation: NationId) {
    if !review_is_due(w, nation) {
        return;
    }
    let raw_context = RawSupplyContext::new(w);
    evaluate_with_context(w, nation, &raw_context);
}

fn review_is_due(w: &WorldState, nation: NationId) -> bool {
    if !enabled(w) || w.player == Some(nation) || !w.nation_opt(nation).is_some_and(|n| n.alive) {
        return false;
    }
    let today = clock::absolute_day(w);
    !w.economic_ai.nations.get(&nation).is_some_and(|p| {
        p.last_review_day == today
            || (today - p.last_review_day < REVIEW_DAYS && p.fiscal_year == w.year)
    })
}

fn evaluate_with_context(
    w: &mut WorldState,
    nation: NationId,
    raw_context: &RawSupplyContext,
) {
    if !review_is_due(w, nation) {
        return;
    }
    review(w, nation, raw_context);
}

fn mine_for_shortage(
    w: &WorldState,
    nation: NationId,
    raw_context: &RawSupplyContext,
) -> Option<(String, Commodity)> {
    if w.resources
        .mine_projects
        .iter()
        .any(|p| p.started_by == nation)
    {
        return None;
    }
    // Resources clear before economic AI in the daily system order. Do not call
    // a mine the trade fallback until today's ordinary raw order really had its
    // chance; manual Materials reserves are protected but are not spot demand.
    let today = clock::absolute_day(w);
    if w.resources.market.as_ref()
        .is_none_or(|market| market.last_cleared_day != Some(today))
    {
        return None;
    }
    let demand = resources::automatic_tick_draw(w, nation);
    let forecast = raw_supply_forecast_with_context(w, nation, raw_context);
    for c in resources::ALL {
        let stock = resources::stockpile(w, nation, c);
        let run_gap = forecast.lines[c.idx()].shortage[0];
        if demand[c.idx()] <= stock || run_gap <= 1e-9 {
            continue;
        }
        // The forecast already nets domestic output, executable freight and
        // contracts for RUN. A pending player offer is a live remedy rather
        // than secured supply, so it remains an explicit wait condition.
        if resources::has_new_inbound_contract(w, nation, c)
            || w.resources.offers.iter().any(|offer| {
                offer.from == nation
                    && Some(offer.to) == w.player
                    && resources::offer_refusal(w, offer.to, offer.id).is_none()
                    && offer.take.iter().any(|leg| {
                        matches!(leg, resources::Leg::Commodity { c: asked, .. } if *asked == c)
                    })
            })
        {
            continue;
        }
        let foreign_producers: Vec<_> = resources::producers(w, c)
            .into_iter()
            .filter(|producer| *producer != nation)
            .collect();
        let peaceful_option = foreign_producers
            .iter()
            .any(|producer| {
                resources::peaceful_supplier_available(w, nation, *producer, c)
            });
        // A real foreign producer keeps trade live until every producer has
        // supplied the existing twice-asked hard-refusal evidence. With no
        // foreign producer there is honestly no market to ask. Forecasts,
        // spot misses, routes, priced-out clocks, and buyer-created embargoes
        // never synthesize universal refusal evidence.
        let trade_closed = !peaceful_option
            || resources::refused_all(w, nation, c).is_some();
        if !trade_closed {
            continue;
        }
        for (district, owner) in &w.districts {
            if *owner == nation && resources::mine_refusal(w, nation, district, c).is_none() {
                return Some((district.clone(), c));
            }
        }
    }
    None
}

/// Explicit AI safety-stock policy, not additional mechanical consumption.
/// Protect one bounded starter lot once a government has begun accumulating
/// paid or actually produced inputs for its first machine shop, plus claims for
/// already commissioned consumers. Ordinary player policies remain untouched.
pub fn export_reserve(w: &WorldState, nation: NationId, good: crate::commerce::Good) -> f64 {
    let line = supply_forecast(w, nation).lines.into_iter()
        .find(|line| line.good == good);
    line.map_or(0.0, |line| {
        let committed = committed_supply_reserve(w, nation, good);
        // Without this latch, a standing reserve-zero offer can resell each
        // partial import between reviews, so an otherwise willing buyer trades
        // forever without reaching the 15-pack machine threshold. Evidence of
        // accumulation is required and the claim is capped at one starter lot;
        // the rest of the stock and the rest of the 90-day plan remain tradable.
        let prospective = if good == crate::commerce::Good::Intermediates
            && line.stock + line.imports + line.domestic_contracts
                + line.recent_domestic_daily > 1e-9
        {
            (line.startup_reserve - committed)
                .max(0.0)
                .min(MACHINERY_STARTER_PACKS)
        } else {
            0.0
        };
        line.project_remaining
            + committed
            + line.operating_daily * REVIEW_DAYS as f64
            + prospective
    })
}

fn offer_surplus(w: &mut WorldState, nation: NationId) {
    use crate::commerce::{self, Good};
    if !commerce::enabled(w) {
        return;
    }
    // Explicit standing consent. These are two bounded, zero-PC policy
    // commands, not two fabricated buyers. Protect real domestic claims first.
    for (index, good) in [Good::Intermediates, Good::CapitalGoods]
        .into_iter()
        .enumerate()
    {
        let reserve = export_reserve(w, nation, good);
        let previous = w
            .economic_ai
            .nations
            .get(&nation)
            .and_then(|p| p.offered_reserves[index]);
        if reserve <= 1e-9
            && commerce::stock(w, nation, good) <= reserve
            && previous.is_none()
            && commerce::sale(w, nation, good).is_none()
        {
            continue;
        }
        let current = commerce::sale(w, nation, good);
        if previous == Some(reserve) && current.is_some_and(|policy| {
            policy.enabled && policy.reserve == reserve && policy.ask_multiplier == 1.05
        }) {
            continue;
        }
        if crate::apply_command(
            w,
            &Command::SetGoodsSale {
                nation,
                good,
                reserve,
                ask_multiplier: 1.05,
                enabled: true,
            },
        )
        .is_ok()
        {
            w.economic_ai
                .nations
                .entry(nation)
                .or_default()
                .offered_reserves[index] = Some(reserve);
        }
    }
}

/// Use existing domestic industry for evidenced unmet Materials use or the
/// separately bounded first-machine startup lot. A quote
/// must be feasible with real inputs and shared power; the AI never signs an
/// empty promise just to avoid building a needed producer. Finite contracts
/// are subtracted once, and never establish their own demand.
pub fn materials_order_candidate(w: &WorldState, nation: NationId) -> Option<Command> {
    use crate::commerce::Good;
    if !enabled(w) || w.player == Some(nation) || w.starting_industry.is_none() || !programs::enrolled(w,nation)
        || industry::power_capacity(w,nation) <= 1e-9 {return None;}
    let plan=industry_planning::plan(w,nation);
    if let Some(bootstrap) = materials_bootstrap(w, nation, &plan) {
        return bootstrap.waiting.is_none().then_some(bootstrap.command);
    }
    let prospective = prospective_first_machine_reserve(w, nation);
    let need=supply_forecast(w,nation).lines.into_iter()
        .find(|line|line.good==Good::Intermediates)
        .map(|line|replenishment_quantity_excluding_startup(&line, prospective))?;
    if need <= 1e-9 {return None;}
    // A one-day feasibility quote is a flow check. It must not be multiplied
    // into a thirty-day promise when the warehouse only owns one day's ore.
    // Other active Materials contracts already claim their full remaining raw
    // bundles, so subtract them before sizing this finite order.
    let reserved_raw = crate::materials::resource_reserve(w, nation);
    let mut best:Option<(String,f64)>=None;
    for (district,owner) in &w.districts {
        if *owner!=nation || resources::district_contested(w,district)
            || industrial_modules::effective_capacity(w,district,K::PowerGrid)<=0.0 {continue;}
        let probe=crate::materials::quote(w,nation,district,1e-9,REVIEW_DAYS as u32);
        if !probe.eligible {continue;}
        let power_per_pack = industry::power_per_pack(w, district, K::ProcessingPlant);
        let unit = industry::operating_recipe(K::ProcessingPlant, 1.0, power_per_pack);
        let raw_capacity = resources::ALL.iter().filter_map(|commodity| {
            let required = unit[commodity.idx()];
            (required > 0.0).then(|| {
                (resources::stockpile(w, nation, *commodity)
                    - reserved_raw[commodity.idx()])
                    .max(0.0) / required
            })
        }).fold(f64::INFINITY, f64::min);
        let quantity=(need.min(probe.capacity_daily*REVIEW_DAYS as f64)
            .min(raw_capacity).min(1_000_000.0)*1e9).floor()/1e9;
        let quote=crate::materials::quote(w,nation,district,quantity,REVIEW_DAYS as u32);
        if !quote.eligible {continue;}
        let quantity=(quantity.min(quote.feasible_today*REVIEW_DAYS as f64)*1e9).floor()/1e9;
        if quantity <= 1e-9 {continue;}
        // A rounded finite total can change the ceil-to-nanopack daily rate.
        // Validate the exact command, not only the earlier, larger quote.
        if !crate::materials::quote(w,nation,district,quantity,REVIEW_DAYS as u32).eligible {continue;}
        if best.as_ref().is_none_or(|(_,previous)|quantity>*previous) {best=Some((district.clone(),quantity));}
    }
    best.map(|(district,quantity)|Command::OrderMaterials{nation,district,quantity,delivery_days:REVIEW_DAYS as u32})
}

fn commission_missing_materials(w:&mut WorldState,nation:NationId)->Option<(bool,String)> {
    let command=materials_order_candidate(w,nation)?;
    Some(match execute(w,&command) {
        Ok(())=>(true,"Commissioned a finite Materials order from inherited domestic industry. It uses real inputs, power and department funds; no packs arrive on signing.".into()),
        Err(why)=>(false,why),
    })
}

fn buy_missing_goods(w: &mut WorldState, nation: NationId) -> Option<(bool, String)> {
    use crate::commerce::Good;
    if !crate::commerce::enabled(w) {
        return None;
    }
    let forecast = supply_forecast(w, nation);
    for good in [Good::Intermediates, Good::CapitalGoods] {
        let missing = forecast.lines.iter().find(|line|line.good==good)
            .map_or(0.0,replenishment_quantity);
        if missing <= 1e-9 {
            continue;
        }
        // Each review may risk at most 0.1% of actual GDP in one lot. The
        // shared quote helper also caps it to treasury cash; there is no loan,
        // extra department debit, or invented stock behind an order.
        if let Some((seller, quantity, unit_price_bn)) =
            goods_import_candidate(w, nation, good, missing)
        {
            let command = Command::ProposeGoodsTrade {
                buyer: nation,
                seller,
                good,
                quantity,
                unit_price_bn,
                delivery_days: REVIEW_DAYS as u32,
            };
            return Some(match execute(w,&command) {
                Ok(())=>(true,format!("Purchased {:.4} {} from {} for the {}-day supply plan; paid goods are usable only after freight arrival.",quantity,good.name(),seller.name(),SUPPLY_HORIZON_DAYS)),
                Err(why)=>(false,why),
            });
        }
    }
    None
}

/// Rescue the specific old-queue bootstrap deadlock without erasing paid work.
/// Imports are preferred when executable. Otherwise only a viable, raw-only
/// first machine shop is commissioned; absent power/grid/processing or a full
/// queue is an explicit wait, not an invented prerequisite or extra slot.
fn warehouse_prerequisite(
    w: &WorldState,
    nation: NationId,
) -> Option<Result<(String, K, String), String>> {
    use crate::commerce::{self, Good};
    let warehouse = production::projects_for(w, nation).find(|p| {
        p.kind == K::Warehouse
            && w.districts.get(&p.district) == Some(&nation)
            && !resources::district_contested(w, &p.district)
    })?;
    if w.districts
        .iter()
        .any(|(d, owner)| *owner == nation && production::level(w, d, K::MachineryWorks) > 0)
    {
        return None;
    }
    let used = w
        .production
        .industry
        .projects
        .get(&warehouse.id)
        .map_or(0.0, |f| f.goods_used.capital_goods);
    let missing = (industry::goods_recipe(K::Warehouse).capital_goods
        - used
        - commerce::stock(w, nation, Good::CapitalGoods)
        - commerce::pending(w, nation, Good::CapitalGoods))
    .max(0.0);
    if missing <= 1e-9 {
        return None;
    }
    // Keep funding an already queued prerequisite, rather than oscillating
    // between it and the older warehouse that still lacks its output.
    if let Some(p) = production::projects_for(w, nation).find(|p| p.kind == K::MachineryWorks) {
        return Some(
            if w.districts.get(&p.district) == Some(&nation)
                && !resources::district_contested(w, &p.district)
                && industry::power_capacity(w, nation) >= 2.0
                && production::level(w, &p.district, K::PowerGrid) > 0
            {
                let mut reason="Finish the paid machinery prerequisite while preserving the capital-starved warehouse's prior work.".to_string();
                if let Some(blocker) = &p.reason {
                    reason.push(' ');
                    reason.push_str(blocker);
                }
                Ok((p.district.clone(), p.kind, reason))
            } else {
                Err("The warehouse's queued machinery prerequisite lacks a controlled powered site; existing work is retained, and no additional project is granted.".into())
            },
        );
    }
    if commerce::enabled(w)
        && !commerce::market_quotes(w, nation, Good::CapitalGoods, missing, 365).is_empty()
    {
        return None;
    }
    let committed = production::projects_for(w, nation).count()
        + w.resources
            .mine_projects
            .iter()
            .filter(|p| p.started_by == nation)
            .count();
    if committed >= CIVILIAN_QUEUE_LIMIT {
        return Some(Err("The warehouse needs a first machinery producer, but both civilian construction slots are committed. Paid projects are retained; no extra slot or free capital is granted.".into()));
    }
    // This bounded rescue retains its stronger prerequisite: a temporary
    // inventory pile alone must not initiate another dependent paid project.
    // Count processors on every owned site, including fractional modules and
    // pending additions; a real imported supply route can also satisfy it.
    let plan = industry_planning::plan(w, nation);
    let intermediate = goods_balance(&plan, Good::Intermediates);
    // In the inherited pilot, owned packs and independently finite supply lots
    // can jointly back both the old warehouse and the first machine's reserve.
    // The legacy rescue retains its stronger processor/import prerequisite.
    let supplied = if crate::materials::enabled(w) {
        intermediate.stock + intermediate.incoming + intermediate.contracted_remaining
            >= commerce::demand(w, nation, Good::Intermediates) + MACHINERY_STARTER_PACKS
    } else {
        intermediate.incoming >= 15.0 || intermediate.contracted_remaining >= 15.0
        || (commerce::enabled(w)
            && commerce::market_quotes(w, nation, Good::Intermediates, 15.0, 365)
                .iter()
                .any(|q| q.quantity >= 15.0))
    };
    if intermediate.installed_daily + intermediate.committed_daily <= 1e-9 && !supplied {
        return Some(Err(if crate::materials::enabled(w) {
            "The warehouse and first machine need their combined Materials lot covered by stock and finite incoming supply, or an installed/queued processor. Short or unsigned lots do not cover both consumers; paid work is retained.".into()
        } else {
            "The warehouse's first machine shop still needs an installed or queued processor, or a real imported intermediate supply route. Existing paid work is retained; temporary stock alone does not supply this rescue.".into()
        }));
    }
    match candidate(w,nation) {
        Ok((d,K::MachineryWorks,_)) => Some(Ok((d,K::MachineryWorks,
            "Build the raw-input machinery prerequisite for the capital-starved warehouse. Its paid work and materials remain installed; no refund or free goods are granted.".into()))),
        Ok((_,kind,_)) => Some(Err(format!("The warehouse needs capital goods, but a viable first machine shop requires {} first. Existing paid work is retained; the rescue does not invent power, grid or processing.",production::catalog(kind).name))),
        Err(why) => Some(Err(format!("The warehouse needs a first machinery producer. {}",why))),
    }
}

fn review(w: &mut WorldState, nation: NationId, raw_context: &RawSupplyContext) {
    if !(w.rules.production_system && w.rules.resource_market) {
        record(
            w,
            nation,
            "blocked",
            "Civilian investment requires production and the resource market to be enabled.".into(),
            None,
            raw_context,
        );
        return;
    }
    // Legal ownership loss is permanent for this sponsor's construction.
    // Occupation alone does not change this map and remains a temporary work
    // blocker. Cancel one stranded job per review through the ordinary command;
    // sunk money/materials are not refunded or transferred into a replacement.
    let stranded = production::projects_for(w, nation)
        .find(|p| w.districts.get(&p.district) != Some(&nation))
        .map(|p| (p.id, p.district.clone()));
    if let Some((project, district)) = stranded {
        let renewal = w
            .nation(nation)
            .program_budget
            .as_ref()
            .is_some_and(|p| p.fiscal_year != w.year);
        if renewal {
            // Cleanup must not stamp the review year while leaving actual
            // capital authority expired for the next 30 days. Renewal of the
            // standing plan retains the ordinary zero-PC command semantics.
            if let Some(command) = fiscal_command(w, nation, true, None) {
                if let Err(why) = execute(w, &command) {
                    record(w, nation, "blocked", why, None, raw_context);
                    return;
                }
            }
        }
        let (action, reason) = match execute(w, &Command::CancelProject { nation, project }) {
            Ok(()) => ("cancel_project", format!("{}Cancelled the stranded project in {} after ownership changed. Prior spending and materials remain sunk; the next review can choose an owned province.", if renewal { "Renewed the standing annual budget. " } else { "" }, district)),
            Err(why) => ("blocked", why),
        };
        record(w, nation, action, reason, None, raw_context);
        return;
    }
    offer_surplus(w, nation);
    let active = production::projects_for(w, nation).count();
    let mut prerequisite = warehouse_prerequisite(w, nation);
    let mut next = if let Some(Ok(target)) = &prerequisite {
        Ok(target.clone())
    } else if active > 0 {
        let p = production::projects_for(w, nation).next().unwrap();
        Ok((
            p.district.clone(),
            p.kind,
            p.reason
                .clone()
                .unwrap_or_else(|| "Existing paid work is progressing.".into()),
        ))
    } else {
        candidate(w, nation)
    };
    let target = next.as_ref().ok().map(|(_, k, _)| *k);
    let renewal = w
        .nation(nation)
        .program_budget
        .as_ref()
        .is_none_or(|p| p.fiscal_year != w.year);
    if let Some(command) = fiscal_command(w, nation, renewal, target) {
        let action = if matches!(command, Command::SetTaxRate { .. }) {
            "fiscal_consolidation"
        } else {
            "budget"
        };
        let (action,reason) = match execute(w,&command) { Ok(()) =>
            (action,"Enacted the priced fiscal decision. Capital authority funds actual work only; construction starts on a later review.".into()), Err(why)=>("blocked",why) };
        record(
            w,
            nation,
            action,
            reason,
            next.ok().map(|(d, k, _)| (d, k)),
            raw_context,
        );
        return;
    }
    // Replenishment is considered before executing the strategic candidate.
    // A foreign purchase gets this review to itself; the finite domestic
    // first-machine bundle may still pair with its deliberately backed project.
    let bootstrap = materials_bootstrap(w, nation, &industry_planning::plan(w, nation));
    if let Some(bootstrap) = &bootstrap {
        if let Some(reason) = &bootstrap.waiting {
            record(
                w,
                nation,
                "waiting",
                reason.clone(),
                Some((bootstrap.machinery_district.clone(), K::MachineryWorks)),
                raw_context,
            );
            return;
        }
    }
    let pair_domestic_with_first_machine = bootstrap
        .as_ref()
        .is_some_and(|bootstrap| bootstrap.starts_machine);
    let domestic_order = commission_missing_materials(w, nation);
    if active == 0 && domestic_order.as_ref().is_some_and(|(ok,_)|*ok) {
        // The new order changed capacity coverage after the initial budget
        // target was selected. Re-read it before placing a duplicate factory.
        next = candidate(w,nation);
    }
    if active > 0 && domestic_order.as_ref().is_some_and(|(ok,_)|*ok) {
        prerequisite = warehouse_prerequisite(w, nation);
        if let Some(Ok(target)) = &prerequisite { next = Ok(target.clone()); }
    }
    let domestic_attempt = domestic_order.is_some();
    let goods_trade = domestic_order.or_else(||buy_missing_goods(w, nation));
    let with_trade = |mut why: String| {
        if let Some((_, trade_reason)) = &goods_trade {
            why.push(' ');
            why.push_str(trade_reason);
        }
        why
    };
    // A paid foreign supply decision gets a clean review of its own. The old
    // path could sign an import and then build the processor selected from the
    // stale pre-import plan in the same review. Physical work already queued
    // continues daily; the next review sees the accepted inbound lot.
    if !domestic_attempt && goods_trade.as_ref().is_some_and(|(ok,_)|*ok) {
        // Do not publish the pre-purchase candidate as the next target. The
        // accepted inbound lot can change that choice, so the next review must
        // recompute it from the delivered/pending ledger.
        // Refresh a pre-existing sale order as soon as the paid starter lot is
        // inbound. Otherwise a reserve-zero policy can resell partial arrivals
        // before the next strategic review and the buyer never accumulates 15.
        offer_surplus(w, nation);
        record(w,nation,"goods_trade",goods_trade.unwrap().1,None,raw_context);
        return;
    }
    // A routine finite domestic order is also a complete supply decision. Its
    // real pending quantity must be observed at the next review before imports
    // or a processor are chosen. The sole paired exception is the preflighted
    // first-machine bootstrap, which proves the whole startup lot and both
    // command prices together.
    if domestic_attempt
        && goods_trade.as_ref().is_some_and(|(ok, _)| *ok)
        && !pair_domestic_with_first_machine
    {
        record(
            w,
            nation,
            "materials_order",
            goods_trade.unwrap().1,
            None,
            raw_context,
        );
        return;
    }
    if active > 0 {
        let (district, kind, why) = next.unwrap();
        if let Some(Err(reason)) = prerequisite {
            record(
                w,
                nation,
                "waiting",
                with_trade(reason),
                Some((district, kind)),
                raw_context,
            );
            return;
        }
        if prerequisite.is_some() && !queued(w, nation, &district, kind) {
            let result = if let Some(reason) = raw_access_reason(w, nation, kind) {
                Err(reason)
            } else {
                execute(
                    w,
                    &Command::StartProject {
                        nation,
                        district: district.clone(),
                        kind,
                    },
                )
            };
            let (action, reason) = match result {
                Ok(()) => {
                    if kind == K::MachineryWorks && crate::materials::enabled(w) {
                        offer_surplus(w, nation);
                    }
                    ("build_prerequisite", why)
                },
                Err(reason) => ("blocked", reason),
            };
            record(
                w,
                nation,
                action,
                with_trade(reason),
                Some((district, kind)),
                raw_context,
            );
            return;
        }
        // A proportional starter must not quietly commission a much larger
        // mine behind it. Its raw deficits use the existing market and remain
        // explicit blockers when no route can supply them.
        if active < CIVILIAN_QUEUE_LIMIT
            && kind != K::StarterIndustry
            && !goods_trade.as_ref().is_some_and(|(success, _)| *success)
        {
            if let Some((mine_district, commodity)) =
                mine_for_shortage(w, nation, raw_context)
            {
                let command = Command::DevelopResource {
                    nation,
                    district: mine_district.clone(),
                    commodity,
                };
                match execute(w, &command) {
                    Ok(()) => {
                        record(w,nation,"mine",with_trade(format!("Develop mapped {} in {} to address a real input shortage. Work, money and materials are still required.",commodity.name(),mine_district)),Some((district,kind)),raw_context);
                        return;
                    }
                    Err(_) => {} // Keep the primary project's actual blocker visible.
                }
            }
        }
        record(
            w,
            nation,
            if goods_trade.as_ref().is_some_and(|(success, _)| *success) {
                "goods_trade"
            } else {
                "work_in_progress"
            },
            with_trade(format!(
                "{} active project(s), limit {}. {}",
                active, CIVILIAN_QUEUE_LIMIT, why
            )),
            Some((district, kind)),
            raw_context,
        );
        return;
    }
    let (district, kind, why) = match next {
        Ok(v) => v,
        Err(why) => {
            if !goods_trade.as_ref().is_some_and(|(success, _)| *success) {
                if let Some((mine_district, commodity)) =
                    mine_for_shortage(w, nation, raw_context)
                {
                    let command = Command::DevelopResource { nation,
                        district: mine_district.clone(), commodity };
                    if execute(w,&command).is_ok() {
                        record(w,nation,"mine",with_trade(format!("The ordinary raw market cleared without covering today's {} bundle. Develop the mapped deposit in {}; work, money and materials are still required.",commodity.name(),mine_district)),None,raw_context);
                        return;
                    }
                }
            }
            record(
                w,
                nation,
                if goods_trade.as_ref().is_some_and(|(success, _)| *success) {
                    "goods_trade"
                } else {
                    "waiting"
                },
                with_trade(why),
                None,
                raw_context,
            );
            return;
        }
    };
    if queued(w, nation, &district, kind) || active >= CIVILIAN_QUEUE_LIMIT {
        record(
            w,
            nation,
            "waiting",
            with_trade("The civilian construction queue is already committed.".into()),
            Some((district, kind)),
            raw_context,
        );
        return;
    }
    if let Some(why) = raw_access_reason(w, nation, kind) {
        record(
            w,
            nation,
            "blocked",
            with_trade(why),
            Some((district, kind)),
            raw_context,
        );
        return;
    }
    let command = if kind == K::StarterIndustry {
        Command::StartIndustryModule {
            nation,
            district: district.clone(),
            capacity_micros: module_order_capacity(w, nation, &district),
        }
    } else {
        Command::StartProject {
            nation,
            district: district.clone(),
            kind,
        }
    };
    let reason = match execute(w, &command) {
        Ok(()) => {
            if kind == K::MachineryWorks && crate::materials::enabled(w) {
                // Protect incoming startup production immediately, not only
                // when the next thirty-day strategic review comes around.
                offer_surplus(w, nation);
            }
            why
        },
        Err(why) => why,
    };
    let action = if queued(w, nation, &district, kind) {
        "build"
    } else {
        "blocked"
    };
    record(
        w,
        nation,
        action,
        with_trade(reason),
        Some((district, kind)),
        raw_context,
    );
}
