//! Opt-in, incremental civilian industry. Coefficients below are GAME recipes,
//! not claims about historical factories, generating stations or district GDP.
//! Industrial packs are manufactured goods, not additional mapped minerals.
//! There is no sales/profit cash or flat GDP multiplier: goods must be used by
//! operating, research or commercial activity. When province GDP is enabled, actual production sends
//! value-added receipts to that ledger. Only new activity can lack power.
use crate::{
    clock,
    companies::{self, CompanySector, CompanyTarget},
    production::{self, Project, ProjectKind as K, ProjectSpec, ProjectStatus},
    programs,
    resources::{self, Commodity as C, ALL},
    world::{NationId, WorldState, BUDGET_INDUSTRY, BUDGET_SCIENCE},
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

const EPS: f64 = 1e-9;
pub const EXTENDED: [K; 7] = [
    K::MachineryWorks,
    K::Generation,
    K::ProcessingPlant,
    K::FreightTerminal,
    K::Warehouse,
    K::Automation,
    K::Efficiency,
];
pub fn extended(kind: K) -> bool {
    EXTENDED.contains(&kind)
}
fn site_index(kind: K) -> usize {
    EXTENDED
        .iter()
        .position(|k| *k == kind)
        .expect("extended industry kind")
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Goods {
    /// Modeled processing packs used by machinery, research and operating lines.
    pub intermediates: f64,
    /// Modeled machine/tool packs for research, operations and commerce.
    pub capital_goods: f64,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct ProjectFunding {
    pub spent_bn: f64,
    /// Frozen total cash contract, including previously paid work.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_cost_bn: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_spent_bn: Option<f64>,
    pub goods_used: Goods,
    pub last_day: Option<i32>,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct MineFunding {
    pub progress_days: f64,
    pub total_days: u32,
    pub spent_bn: f64,
    pub resources_used: [f64; 12],
    pub last_day: Option<i32>,
    pub reason: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SiteStatus {
    pub district: String,
    pub kind: K,
    pub level: u8,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity_micros: Option<u32>,
    pub status: String,
    pub reason: Option<String>,
    pub output_daily: f64,
    pub power_used_daily: f64,
    pub cash_spent_daily_bn: f64,
}

/// A settled prototype-service receipt, not a second research-money balance.
/// Credits belong to the sponsoring nation and named technology; capturing a
/// building never transfers the former owner's research or prepaid work.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ResearchOperation {
    pub district: String,
    pub nation: NationId,
    pub level: u8,
    pub day: i32,
    pub technology: Option<u16>,
    pub technology_name: Option<String>,
    pub status: String,
    pub reason: String,
    pub prototype_credit: f64,
    pub cash_spent_daily_bn: f64,
    pub goods_used: Goods,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct ResearchProgram {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty", with = "research_credit_serde")]
    pub credits: BTreeMap<u16, f64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operations: Vec<ResearchOperation>,
}

// Technology indices are runtime details: adding an entry to an earlier
// domain changes later indices. Persist prototype ownership by the same stable
// technology ids as TechState. A retired id loses its unusable credit rather
// than accidentally subsidizing whichever technology inherited its index.
mod research_credit_serde {
    use super::*;
    use serde::{Deserializer, Serializer};

    pub fn serialize<S: Serializer>(credits: &BTreeMap<u16, f64>, s: S) -> Result<S::Ok, S::Error> {
        let ids: BTreeMap<_, _> = credits.iter().filter_map(|(t, value)|
            crate::tech::registry().get(*t as usize).map(|def| (def.id, *value))).collect();
        ids.serialize(s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<BTreeMap<u16, f64>, D::Error> {
        let ids = BTreeMap::<String, f64>::deserialize(d)?;
        Ok(ids.into_iter().filter_map(|(id, value)| crate::tech::index_of(&id).map(|t| (t, value))).collect())
    }
}

/// MODEL service recipe, not historical R&D prices. One completed level can
/// supply at most 0.01 acquisition-cost units of prototype/testing work/day.
/// The other bound is 25% of the target domain's actual daily research effort,
/// shared by ALL laboratories, so a microstate is never sold a rich-country
/// work package. Lifetime credit can cover at most 20% of today's total bill.
pub const PROTOTYPE_WORK_PER_LEVEL_DAY: f64 = 0.01;
pub const PROTOTYPE_EFFORT_SHARE: f64 = 0.25;
pub const PROTOTYPE_COST_SHARE: f64 = 0.20;
pub const PROTOTYPE_CASH_PER_LEVEL_DAY_BN: f64 = 0.0001;
pub const PROTOTYPE_INTERMEDIATES_PER_LEVEL_DAY: f64 = 0.1;
pub const PROTOTYPE_CAPITAL_PER_LEVEL_DAY: f64 = 0.1;
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Industry {
    /// Completed fractional starter packages, separate from integer sites.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub modules: BTreeMap<String, u32>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub sites: BTreeMap<String, [u8; 7]>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub goods: BTreeMap<NationId, Goods>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub projects: BTreeMap<u32, ProjectFunding>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub mines: BTreeMap<String, MineFunding>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operations: Vec<SiteStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_day: Option<i32>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    /// Legacy saved workforce receipts; daily construction no longer reads these.
    pub work: BTreeMap<NationId, (i32, f64, f64)>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub research: BTreeMap<NationId, ResearchProgram>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub research_day: Option<i32>,
}

/// Pure raw-material components for the strategic forecast. `operating_daily`
/// is the installed civilian plan the resource market already attempts each
/// day. Construction rows remain zero for compatibility: turnkey projects and
/// mines no longer place separate physical material claims.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RawDemandComponents {
    pub operating_daily: [f64; 12],
    pub projects_remaining: [f64; 12],
    pub projects_daily: [f64; 12],
    /// Compatibility rows for the canonical 30/90/365-day horizons, all zero.
    pub projects_horizon: [[f64; 3]; 12],
    pub mines_remaining: [f64; 12],
    pub mines_daily: [f64; 12],
    /// Per-mine equivalent of `projects_horizon`, all zero.
    pub mines_horizon: [[f64; 3]; 12],
}
impl Industry {
    pub fn is_empty(&self) -> bool {
        self.modules.is_empty()
            && self.sites.is_empty()
            && self.goods.is_empty()
            && self.projects.is_empty()
            && self.mines.is_empty()
            && self.operations.is_empty()
            && self.last_day.is_none()
            && self.work.is_empty()
            && self.research.is_empty()
            && self.research_day.is_none()
    }
}

pub fn catalog(kind: K) -> ProjectSpec {
    if kind == K::StarterIndustry {
        let mut recipe = [0.0; 12];
        for component in crate::industrial_modules::COMPONENTS {
            let part = production::catalog(component);
            for i in 0..12 { recipe[i] += part.recipe[i]; }
        }
        return ProjectSpec {kind, name:"Starter Industry", description:"A proportional industrial estate, generator, local grid and materials-processing line.",
            effect:"Paid fractional processing capacity, with matching power and operating inputs. No stock is granted.",
            total_days:1800,political_cost:production::catalog(K::CivilianIndustry).political_cost,
            funding_ministry:BUDGET_INDUSTRY,funding_label:"Industry & Energy",funding_required:0.020,recipe};
    }
    let (name, description, effect, days, recipe) = match kind {
        K::MachineryWorks => ("Machinery Works", "Tool and machine production for civilian capital projects.", "Uses intermediate packs, copper and power to produce 0.5 capital-goods packs/day per level.", 420, [(C::Iron,40.0),(C::Copper,12.0),(C::Coal,15.0)]),
        K::Generation => ("Power Plant", "Dispatchable electricity for industrial operations.", "Supplies up to 10 modeled power units/day per level; consumes generating fuel and funds when electricity is used.", 480, [(C::Iron,50.0),(C::Copper,18.0),(C::Coal,20.0)]),
        K::ProcessingPlant => ("Materials Plant", "Converts ore and fuel into prepared industrial materials.", "Uses iron, bauxite, coal and power to produce 1 intermediate pack/day per level for installation, machinery, offices and trade.", 360, [(C::Iron,35.0),(C::Copper,8.0),(C::Coal,20.0)]),
        K::FreightTerminal => ("Freight Terminal", "An upgrade to a mapped coastal freight gateway.", "+25% modeled terminal throughput per level, shared by every shipment through this gateway.", 360, [(C::Iron,35.0),(C::Copper,6.0),(C::Coal,12.0)]),
        K::Warehouse => ("Industrial Warehouses", "Storage for manufactured intermediate and capital-goods packs.", "+250 storage capacity for each industrial good per level. Full storage pauses its producer; no stock is destroyed.", 240, [(C::Iron,20.0),(C::Copper,4.0),(C::Coal,8.0)]),
        K::Automation => ("Factory Automation", "Robot cells and controls fitted to a real civilian plant.", "+20% local civilian line throughput per level, with proportionate inputs, power and cash. Requires Industrial Robot Cells.", 300, [(C::Iron,12.0),(C::Copper,10.0),(C::RareEarths,2.0)]),
        K::Efficiency => ("Energy Efficiency", "Process improvements fitted to a real civilian plant.", "Cuts new local plant power and generating-fuel use by 10% per level, capped at 50%. Requires Lean Production.", 240, [(C::Iron,8.0),(C::Copper,8.0),(C::RareEarths,1.0)]),
        _ => panic!("extended catalog only"),
    };
    let mut raw = [0.0; 12];
    for (c, amount) in recipe {
        raw[c.idx()] = amount;
    }
    ProjectSpec {
        kind,
        name,
        description,
        effect,
        total_days: days,
        political_cost: 10.0,
        funding_ministry: BUDGET_INDUSTRY,
        funding_label: "Industry & Energy",
        funding_required: 0.02,
        recipe: raw,
    }
}

/// Modeled turnkey construction contract, $bn. Construction buys no separate
/// raw or manufactured inventory; operating facilities still consume inputs.
pub fn work_cost_bn(kind: K) -> f64 {
    match kind {
        K::Infrastructure => 0.20,
        K::CivilianIndustry => 0.18,
        K::PowerGrid => 0.12,
        K::ResearchCenter => 0.20,
        K::ArmsPlant => 0.25,
        K::MachineryWorks => 0.10,
        K::Generation => 0.16,
        K::ProcessingPlant => 0.12,
        K::FreightTerminal => 0.12,
        K::Warehouse => 0.06,
        K::Automation => 0.08,
        K::Efficiency => 0.06,
        K::StarterIndustry => 0.58,
        K::OfficeDistrict => 0.16,
        K::Shipyard => 0.32,
        K::AdvancedIndustry => 0.24,
    }
}
pub fn project_cost_bn(p: &Project) -> f64 {
    work_cost_bn(p.kind) * crate::industrial_modules::scale(p)
}
/// New construction contracts include all installation inputs in their cash price.
/// Historical material receipts remain saved, but create no future demand.
pub fn project_recipe(_p: &Project) -> [f64; 12] { [0.0; 12] }
pub fn goods_recipe(_kind: K) -> Goods { Goods::default() }
/// Preview the same legacy migration used at the next construction tick. Paid
/// work is preserved, and only the unfinished fraction retains a future bill.
fn funding_for(w: &WorldState, p: &Project) -> ProjectFunding {
    let nominal = project_cost_bn(p);
    let mut f = w.production.industry.projects.get(&p.id).cloned().unwrap_or_else(|| ProjectFunding {
        spent_bn: nominal * p.progress_fraction(),
        ..Default::default()
    });
    if f.contract_cost_bn.is_none() {
        f.contract_cost_bn = Some(f.spent_bn + nominal * (1.0 - p.progress_fraction()));
    }
    f
}
pub fn contract_cost_bn(w: &WorldState, p: &Project) -> f64 {
    funding_for(w, p).contract_cost_bn.expect("project contract")
}
pub fn site_level(w: &WorldState, district: &str, kind: K) -> u8 {
    w.production
        .industry
        .sites
        .get(district)
        .map_or(0, |row| row[site_index(kind)])
}
pub(crate) fn complete_site(w: &mut WorldState, district: &str, kind: K) {
    let row = w
        .production
        .industry
        .sites
        .entry(district.into())
        .or_default();
    row[site_index(kind)] = row[site_index(kind)]
        .saturating_add(1)
        .min(production::MAX_PROVINCE_LEVEL);
}
pub fn project_refusal(
    w: &WorldState,
    nation: NationId,
    district: &str,
    kind: K,
) -> Option<String> {
    if w.rules.military_operations {
        if let Some(reason) = crate::control::blocker(w, nation, district) { return Some(reason); }
    }
    if clock::is_daily(w) && resources::district_contested(w, district) {
        return Some("Construction cannot start in a contested province.".into());
    }
    if !extended(kind) {
        return None;
    }
    if resources::district_contested(w, district) {
        return Some("Construction cannot start in a contested province.".into());
    }
    match kind {
        K::FreightTerminal if !w.rules.physical_logistics || !w.rules.logistics_routes => {
            return Some("Enable physical freight routes before upgrading a terminal.".into())
        }
        K::MachineryWorks | K::ProcessingPlant
            if !w.rules.industry_rebuild &&
            crate::industrial_modules::effective_capacity(w, district, K::CivilianIndustry) < 1.0 =>
        {
            return Some("Build an Industrial Estate in this province first.".into())
        }
        K::FreightTerminal if !crate::logistics::has_terminal(district) => {
            return Some("This province has no mapped coastal freight gateway to upgrade.".into())
        }
        K::Automation | K::Efficiency => {
            if site_level(w, district, K::MachineryWorks) == 0
                && crate::industrial_modules::effective_capacity(w, district, K::ProcessingPlant) <= 0.0
            {
                return Some(
                    "Build a Machinery Works or Materials Processing plant here first.".into(),
                );
            }
            let (tech, label) = if kind == K::Automation {
                ("matl_industrial_robotics", "Industrial Robot Cells")
            } else {
                ("matl_lean_production", "Lean Production")
            };
            if !w.nation(nation).tech.knows(tech) {
                return Some(format!("Research {label} before installing this upgrade."));
            }
        }
        _ => {}
    }
    None
}

pub fn enroll_projects(w: &mut WorldState, nation: NationId) {
    let rows: Vec<_> = production::projects_for(w, nation)
        .map(|p| (p.id, funding_for(w, p))).collect();
    for (id, funding) in rows { w.production.industry.projects.insert(id, funding); }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct WorkPlan {
    /// Site lead-time ceiling before the shared construction budget is applied.
    pub target_advance_days: f64,
    pub advance_days: f64,
    pub cash_bn: f64,
    /// Included in cash_bn, but not in the frozen underlying building price.
    pub company_fee_bn: f64,
    /// Quote-time base work; another contract can level up the same firm before
    /// this reserved plan settles, without changing its recipe or work receipt.
    pub company_base_work: f64,
    pub required: [f64; 12],
    pub goods: Goods,
    pub reason: Option<String>,
    /// A temporary funding constraint; positive affordable work can settle.
    pub slow_reason: Option<String>,
    pub department_draws_bn: [f64; 5],
}
fn q(v: f64) -> f64 {
    (v.max(0.0) * 1e9).round() / 1e9
}
/// Migrate daily projects before settlement, preserving IDs, progress, historical
/// input receipts and every previously paid dollar. Monthly replay is unchanged.
pub fn begin_work_day(w: &mut WorldState) {
    if !clock::is_daily(w) { return; }
    let nations: std::collections::BTreeSet<_> = w.production.projects.iter().map(|p| p.nation).collect();
    for nation in nations { enroll_projects(w, nation); }
    crate::construction_capacity::begin_day(w);
}
pub fn project_authority(w: &WorldState, nation: NationId, _kind: K) -> f64 {
    programs::construction_available_bn(w, nation)
}
/// Pure allocator: project priority then stable ID reserves the shared daily
/// construction budget once. Parallel sites have independent lead times.
pub fn project_plans(w: &WorldState) -> BTreeMap<u32, WorkPlan> {
    let mut out = BTreeMap::new();
    if !clock::is_daily(w) { return out; }
    let mut cash = BTreeMap::new();
    let allocations = if w.rules.industry_rebuild {
        crate::construction_capacity::allocations(w)
    } else { BTreeMap::new() };
    let mut raw_stock = BTreeMap::<NationId, [f64; 12]>::new();
    let mut goods_stock = w.production.industry.goods.clone();
    let mut queue: Vec<_> = w.production.projects.iter().collect();
    queue.sort_by_key(|p| (p.priority.dispatch_rank(), p.id));
    for p in queue {
        let f = funding_for(w, p);
        let mut plan = WorkPlan::default();
        if !programs::enrolled(w, p.nation) {
            plan.reason = Some("PAUSED: set a construction budget to fund this project.".into());
        } else if w.districts.get(&p.district) != Some(&p.nation) || !w.nation(p.nation).alive {
            plan.reason = Some("BLOCKED: sponsoring government no longer controls this province.".into());
        } else if resources::district_contested(w, &p.district) {
            plan.reason = Some("BLOCKED: this province is contested.".into());
        } else if w.rules.military_operations {
            plan.reason = crate::control::blocker(w, p.nation, &p.district).map(|r| format!("BLOCKED: {r}"));
        }
        if f.last_day == Some(clock::absolute_day(w)) {
            plan.reason = Some("Today's construction has already settled.".into());
        }
        if plan.reason.is_some() { out.insert(p.id, plan); continue; }
        let balance = cash.entry(p.nation).or_insert_with(|| programs::construction_available_bn(w, p.nation));
        let remaining = (p.total_days as f64 - p.progress_days).max(0.0);
        let due = (f.contract_cost_bn.expect("frozen contract") - f.spent_bn).max(0.0);
        let company = companies::modifiers(w, p.nation, &CompanyTarget::Construction { project: p.id });
        let physical_work = if w.rules.industry_rebuild {
            let assigned = allocations.get(&p.id).copied().unwrap_or(0.0)
                .min(crate::construction_capacity::MAX_PER_PROJECT);
            assigned / crate::construction_capacity::CAPACITY_PER_WORK_DAY
                * (1.0 + production::level(w, &p.district, K::Infrastructure) as f64
                    * crate::construction_capacity::INFRASTRUCTURE_WORK_BONUS)
        } else { 1.0 };
        let ordinary_site_work = crate::industrial_modules::normalized_advance(w, p, physical_work).min(remaining);
        let site_work = crate::industrial_modules::normalized_advance(w, p, physical_work * company.work_rate).min(remaining);
        // A commissioning ceiling or the final fraction of a project can
        // leave no work for a faster contractor to add. Do not charge for an
        // unavailable service or report normalized days as invented bonuses.
        let fee_rate = if (site_work - ordinary_site_work).abs() <= EPS { 0.0 } else { company.fee_rate };
        plan.target_advance_days = site_work;
        if site_work <= EPS && remaining > EPS {
            plan.slow_reason = Some(if w.rules.industry_rebuild && allocations.get(&p.id).copied().unwrap_or(0.0) <= EPS {
                "PAUSED: no construction capacity assigned. Use Auto or assign available capacity."
            } else { "PAUSED: this site's commissioning lead time preserves completed work until the next work date." }.into());
        }
        let daily_price = if remaining > EPS { due / remaining } else { 0.0 };
        let ordinary_advance = if daily_price > 0.0 { ordinary_site_work.min(*balance / daily_price) } else { ordinary_site_work };
        let billed_price = daily_price * (1.0 + fee_rate);
        plan.advance_days = if billed_price > 0.0 { site_work.min(*balance / billed_price) } else { site_work };
        let base_cash = if plan.advance_days + EPS >= remaining { due } else { daily_price * plan.advance_days };
        plan.cash_bn = (base_cash * (1.0 + fee_rate)).min(*balance);
        plan.company_fee_bn = plan.cash_bn * fee_rate / (1.0 + fee_rate);
        // Recompute progress from the exact final cash amount; no underpaid
        // completion and no second bill after a save or a partial budget day.
        if billed_price > 0.0 { plan.advance_days = (plan.cash_bn / billed_price).min(site_work); }
        plan.company_base_work = ordinary_advance;
        if w.rules.industry_rebuild && plan.advance_days > EPS {
            // MODEL installation inputs. Prepared materials can replace iron;
            // a machine/tool pack replaces its 0.1 copper input. Both routes
            // reserve the same physical bundle once, in project priority order.
            let stock = raw_stock.entry(p.nation).or_insert_with(||
                std::array::from_fn(|i| resources::stockpile(w, p.nation, ALL[i])));
            let goods = goods_stock.entry(p.nation).or_default();
            let recipe = production::catalog(p.kind).recipe;
            let fraction = plan.advance_days / p.total_days.max(1) as f64
                * crate::industrial_modules::scale(p);
            let mut required = recipe.map(|amount| amount * fraction);
            let material = required[C::Iron.idx()].min(goods.intermediates);
            let machinery = (required[C::Copper.idx()] / 0.1).min(goods.capital_goods);
            required[C::Iron.idx()] -= material;
            required[C::Copper.idx()] -= machinery * 0.1;
            let mut ratio: f64 = 1.0;
            let mut limiting = None;
            for c in ALL {
                if required[c.idx()] > EPS {
                    let feasible = (stock[c.idx()] / required[c.idx()]).clamp(0.0, 1.0);
                    if feasible < ratio { ratio = feasible; limiting = Some(c); }
                }
            }
            plan.advance_days *= ratio;
            plan.cash_bn *= ratio;
            plan.company_fee_bn *= ratio;
            plan.company_base_work *= ratio;
            plan.required = required.map(|amount| (amount * ratio * 1e9).floor() / 1e9);
            plan.goods = Goods { intermediates: (material * ratio * 1e9).floor()/1e9,
                capital_goods: (machinery * ratio * 1e9).floor()/1e9 };
            for c in ALL { stock[c.idx()] = (stock[c.idx()] - plan.required[c.idx()]).max(0.0); }
            goods.intermediates = (goods.intermediates - plan.goods.intermediates).max(0.0);
            goods.capital_goods = (goods.capital_goods - plan.goods.capital_goods).max(0.0);
            if let Some(c) = limiting {
                plan.slow_reason = Some(format!("{}: missing {} limits construction to {:.0}% of allocated work. Import inputs or produce prepared materials.",
                    if ratio <= EPS { "PAUSED" } else { "SLOWED" }, c.name(), ratio * 100.0));
            }
        }
        if plan.advance_days + EPS < site_work {
            if plan.slow_reason.is_none() { plan.slow_reason = Some(if plan.advance_days <= EPS {
                "PAUSED: today's construction budget is exhausted; completed work is preserved.".into()
            } else { "SLOWED: today's construction budget funds part of this site's work.".into() }); }
        }
        *balance = (*balance - plan.cash_bn).max(0.0);
        out.insert(p.id, plan);
    }
    out
}
pub(crate) fn settle_project(
    w: &mut WorldState,
    p: &Project,
    plan: &WorkPlan,
) -> Result<(), String> {
    if let Some(reason) = &plan.reason {
        return Err(reason.clone());
    }
    if !funding_day_open(w, p.nation) {
        return Err("BLOCKED: the daily construction ledger is not open.".into());
    }
    if w.rules.industry_rebuild {
        // Preflight all accounts before charging any of them.
        let goods = w.production.industry.goods.get(&p.nation).cloned().unwrap_or_default();
        if goods.intermediates + EPS < plan.goods.intermediates || goods.capital_goods + EPS < plan.goods.capital_goods {
            return Err("PAUSED: prepared construction inputs are no longer available.".into());
        }
        if programs::construction_available_bn(w,p.nation) + EPS < plan.cash_bn {
            return Err("PAUSED: construction funding is no longer available.".into());
        }
        resources::consume_stockpile_atomic(w,p.nation,&plan.required)
            .map_err(|(c,_,_)|format!("PAUSED: {} is no longer available for construction.",c.name()))?;
        if plan.goods.intermediates > 0.0 || plan.goods.capital_goods > 0.0 {
            let goods = w.production.industry.goods.entry(p.nation).or_default();
            goods.intermediates = q(goods.intermediates - plan.goods.intermediates);
            goods.capital_goods = q(goods.capital_goods - plan.goods.capital_goods);
        }
    }
    programs::spend_construction(w, p.nation, plan.cash_bn)?;
    let today = clock::absolute_day(w);
    let f = w.production.industry.projects.get_mut(&p.id).unwrap();
    f.spent_bn += plan.cash_bn - plan.company_fee_bn;
    f.last_spent_bn = Some(plan.cash_bn);
    f.last_day = Some(today);
    f.goods_used.intermediates += plan.goods.intermediates;
    f.goods_used.capital_goods += plan.goods.capital_goods;
    let row = w
        .production
        .projects
        .iter_mut()
        .find(|row| row.id == p.id)
        .unwrap();
    row.progress_days = (row.progress_days + plan.advance_days).min(row.total_days as f64);
    for c in ALL { row.resources_used[c.idx()] += plan.required[c.idx()]; }
    row.status = if plan.advance_days <= EPS {
        ProjectStatus::Paused
    } else if plan.slow_reason.is_some() {
        ProjectStatus::Slowed
    } else {
        ProjectStatus::Building
    };
    row.reason = if let Some(reason) = &plan.slow_reason {
        Some(reason.clone())
    } else {
        None
    };
    crate::gdp_projects::record_construction(w, p, plan.advance_days * crate::industrial_modules::scale(p), plan.cash_bn, true);
    let target = CompanyTarget::Construction { project: p.id };
    let scale = crate::industrial_modules::scale(p);
    let base_work = plan.company_base_work * scale;
    let actual_work = plan.advance_days * scale;
    companies::record_work(w, p.nation, &target, base_work, actual_work - base_work, plan.company_fee_bn);
    companies::record_sector_activity(w, p.nation, CompanySector::Construction, actual_work);
    Ok(())
}
#[derive(Clone, Debug, Serialize)]
pub struct ProjectFinanceView {
    pub department: usize,
    pub department_name: &'static str,
    pub department_draws_bn: [f64; 5],
    pub cost_bn: f64,
    pub spent_bn: f64,
    pub remaining_bn: f64,
    pub daily_request_bn: f64,
    pub daily_budget_bn: f64,
    pub spent_today_bn: f64,
    pub next_work_days: f64,
    pub goods_recipe: Goods,
    pub goods_used: Goods,
    pub next_goods: Goods,
    pub reason: Option<String>,
}
pub fn project_finance(w: &WorldState, p: &Project) -> Option<ProjectFinanceView> {
    if !clock::is_daily(w) { return None; }
    let f = funding_for(w, p);
    let plan = project_plans(w).remove(&p.id).unwrap_or_default();
    let display_reason = plan.reason.clone().or_else(|| plan.slow_reason.clone());
    let d = production::funding_department(p.kind);
    Some(ProjectFinanceView {
        department: d,
        department_name: "Construction budget",
        department_draws_bn: plan.department_draws_bn,
        cost_bn: f.contract_cost_bn.expect("frozen contract"),
        spent_bn: f.spent_bn,
        remaining_bn: (f.contract_cost_bn.expect("frozen contract") - f.spent_bn).max(0.0),
        daily_request_bn: plan.cash_bn,
        daily_budget_bn: programs::construction_daily_budget_bn(w, p.nation),
        spent_today_bn: if f.last_day == Some(clock::absolute_day(w)) { f.last_spent_bn.unwrap_or(0.0) } else { 0.0 },
        next_work_days: plan.advance_days,
        goods_recipe: goods_recipe(p.kind),
        goods_used: f.goods_used.clone(),
        next_goods: plan.goods,
        reason: display_reason,
    })
}

pub fn goods_capacity(w: &WorldState, nation: NationId) -> f64 {
    250.0
        + w.production
            .industry
            .sites
            .iter()
            .filter(|(d, _)| w.districts.get(*d) == Some(&nation))
            .map(|(_, v)| v[site_index(K::Warehouse)] as f64 * 250.0)
            .sum::<f64>()
}
pub fn power_capacity(w: &WorldState, nation: NationId) -> f64 {
    let legacy: f64 = w.production
        .industry
        .sites
        .iter()
        .filter(|(d, _)| {
            w.districts.get(*d) == Some(&nation) && !resources::district_contested(w, d)
        })
        .map(|(d, v)| v[site_index(K::Generation)] as f64 * 10.0
            * companies::modifiers(w, nation, &CompanyTarget::Facility { district: d.clone(), sector: CompanySector::Energy }).work_rate)
        .sum();
    legacy + crate::industry_operations::inherited_power_headroom(w, nation) + w.production.industry.modules.iter()
        .filter(|(d,_)| w.districts.get(*d)==Some(&nation) && !resources::district_contested(w,d))
        .map(|(d,micros)| *micros as f64 / 1_000_000.0 * 10.0
            * companies::modifiers(w, nation, &CompanyTarget::Facility { district: d.clone(), sector: CompanySector::Energy }).work_rate).sum::<f64>()
}
/// Generation serves the shared national grid. Dispatch and its operator fees
/// are apportioned over installed, uncontested generation by available capacity.
fn energy_dispatch(w: &WorldState, nation: NationId) -> Vec<(CompanyTarget, f64, companies::CompanyModifiers)> {
    operating_districts(w).into_iter().filter_map(|district| {
        if w.districts.get(&district) != Some(&nation) || resources::district_contested(w, &district) { return None; }
        let capacity = site_level(w, &district, K::Generation) as f64 * 10.0
            + w.production.industry.modules.get(&district).copied().unwrap_or(0) as f64 / 1_000_000.0 * 10.0;
        if capacity <= 0.0 { return None; }
        let target = CompanyTarget::Facility { district, sector: CompanySector::Energy };
        let modifier = companies::modifiers(w, nation, &target);
        Some((target, capacity * modifier.work_rate, modifier))
    }).collect()
}
pub(crate) fn energy_company_rates(w: &WorldState, nation: NationId) -> (f64, f64) {
    if !w.companies.enabled || !w.companies.assignments.iter().any(|a|
        a.nation == nation && a.target.sector() == CompanySector::Energy) { return (1.0, 0.0); }
    let dispatch = energy_dispatch(w, nation);
    let inherited = crate::industry_operations::inherited_power_headroom(w, nation);
    let capacity: f64 = dispatch.iter().map(|(_, cap, _)| cap).sum::<f64>() + inherited;
    if capacity <= 0.0 { return (1.0, 0.0); }
    let input = (dispatch.iter().map(|(_, cap, m)| cap * m.input_rate).sum::<f64>() + inherited) / capacity;
    let fee = dispatch.iter().map(|(_, cap, m)| cap * m.fee_rate).sum::<f64>() / capacity;
    (input, fee)
}
pub(crate) fn record_energy_work(w: &mut WorldState, nation: NationId, power: f64, base_cash_bn: f64) {
    if power <= 0.0 || !w.companies.enabled { return; }
    companies::record_sector_activity(w, nation, CompanySector::Energy, power);
    if !w.companies.assignments.iter().any(|a| a.nation == nation && a.target.sector() == CompanySector::Energy) { return; }
    let dispatch = energy_dispatch(w, nation);
    let capacity: f64 = dispatch.iter().map(|(_, cap, _)| cap).sum::<f64>()
        + crate::industry_operations::inherited_power_headroom(w, nation);
    if capacity <= 0.0 { return; }
    for (target, cap, modifier) in dispatch {
        let actual = power * cap / capacity;
        let base = actual / modifier.work_rate;
        companies::record_work(w, nation, &target, base, actual - base, base_cash_bn * cap / capacity * modifier.fee_rate);
    }
}
pub(crate) fn manufacturing_company(w: &WorldState, nation: NationId, district: &str) -> companies::CompanyModifiers {
    companies::modifiers(w, nation, &CompanyTarget::Facility { district: district.into(), sector: CompanySector::Manufacturing })
}
pub(crate) fn record_manufacturing_work(w: &mut WorldState, nation: NationId, district: &str, output: f64, base_cash_bn: f64) {
    let target = CompanyTarget::Facility { district: district.into(), sector: CompanySector::Manufacturing };
    let modifier = companies::modifiers(w, nation, &target);
    let base = output / modifier.work_rate;
    companies::record_work(w, nation, &target, base, output - base, base_cash_bn * modifier.fee_rate);
    companies::record_sector_activity(w, nation, CompanySector::Manufacturing, output);
}
pub(crate) fn plant_rate(w: &WorldState, district: &str, kind: K) -> f64 {
    let base = if is_processing(kind) { 1.0 } else { 0.5 };
    let capacity = if kind==K::StarterIndustry {crate::industrial_modules::capacity(w,district)} else {site_level(w,district,kind) as f64};
    base * capacity
        * (1.0 + site_level(w, district, K::Automation) as f64 * 0.2)
        * w.districts.get(district).map_or(1.0, |nation| manufacturing_company(w, *nation, district).work_rate)
}
fn is_processing(kind: K)->bool {matches!(kind,K::ProcessingPlant|K::StarterIndustry)}
fn operating_districts(w:&WorldState)->Vec<String>{
    if w.production.industry.modules.is_empty() {return w.production.industry.sites.keys().cloned().collect();}
    w.production.industry.sites.keys().chain(w.production.industry.modules.keys()).cloned()
        .collect::<std::collections::BTreeSet<_>>().into_iter().collect()
}
pub(crate) fn power_per_pack(w: &WorldState, district: &str, kind: K) -> f64 {
    (if is_processing(kind) { 1.0 } else { 2.0 })
        * (1.0 - site_level(w, district, K::Efficiency) as f64 * 0.1).max(0.5)
}
pub(crate) fn operating_recipe(kind: K, output: f64, power: f64) -> [f64; 12] {
    let mut raw = [0.0; 12];
    raw[C::Coal.idx()] = power * 0.02;
    if is_processing(kind) {
        raw[C::Iron.idx()] = output;
        raw[C::Bauxite.idx()] = output * 0.2;
        raw[C::Coal.idx()] += output * 0.25;
    } else {
        raw[C::Copper.idx()] = output * 0.1;
    }
    raw.map(q)
}
/// Keep industrial ingredients and generating fuel separate: a materials
/// specialist saves ingredients, while the dispatched generators save fuel.
pub(crate) fn company_operating_recipe(w: &WorldState, nation: NationId, district: &str, kind: K, output: f64, power: f64) -> [f64; 12] {
    let company = manufacturing_company(w, nation, district);
    let (fuel_rate, _) = energy_company_rates(w, nation);
    if company.input_rate == 1.0 && fuel_rate == 1.0 { return operating_recipe(kind, output, power); }
    let mut raw = operating_recipe(kind, output, 0.0).map(|v| q(v * company.input_rate));
    raw[C::Coal.idx()] = q(raw[C::Coal.idx()] + power * 0.02 * fuel_rate);
    raw
}
fn funding_day_open(w: &WorldState, nation: NationId) -> bool {
    let today = clock::absolute_day(w);
    w.nation_opt(nation)
        .and_then(|n| n.program_budget.as_ref())
        .is_some_and(|p| p.day == Some(today) && p.settled_day != Some(today))
}
/// Potential daily raw demand of commissioned civilian lines. No stockpile or
/// market access here, so the initial reserve-policy read cannot recurse.
pub fn resource_demand_daily(w: &WorldState, nation: NationId) -> [f64; 12] {
    resource_demand_daily_inner(w, nation, true)
}
/// Existing funded activities retain their automatic raw-market policy.
/// Inherited Materials orders are manually supplied and cannot authorize a
/// foreign purchase merely by appearing in the public requirements forecast.
pub(crate) fn automatic_resource_demand_daily(w: &WorldState, nation: NationId) -> [f64; 12] {
    resource_demand_daily_inner(w, nation, false)
}

/// Dates inside the next-unsettled horizon for which this government's current
/// enacted department programme still has fiscal authority.
pub fn funded_days_in_horizon(
    w: &WorldState,
    nation: NationId,
    horizon_days: i32,
) -> f64 {
    let Some(plan) = w.nation_opt(nation).and_then(|nation| nation.program_budget.as_ref()) else {
        return 0.0;
    };
    let start = resources::forecast_start_day(w);
    let end = start.saturating_add(horizon_days.max(0));
    let fiscal_start = crate::clock::date_day(plan.fiscal_year, 1, 1);
    let fiscal_end = crate::clock::date_day(plan.fiscal_year.saturating_add(1), 1, 1);
    end.min(fiscal_end)
        .saturating_sub(start.max(fiscal_start))
        .max(0) as f64
}

/// Decompose the raw demand already owned by civilian industry. This reads
/// only committed facilities/projects and cumulative input receipts; it does
/// not inspect a forecast, market option, or prospective AI project.
pub fn raw_demand_components(w: &WorldState, nation: NationId) -> RawDemandComponents {
    let mut out = RawDemandComponents::default();
    if programs::enrolled(w, nation) && w.nation_opt(nation).is_some_and(|n| n.alive) {
        for district in operating_districts(w) {
            if w.districts.get(&district) != Some(&nation)
                || resources::district_contested(w, &district)
            {
                continue;
            }
            for kind in [K::ProcessingPlant, K::StarterIndustry, K::MachineryWorks] {
                let rate = plant_rate(w, &district, kind);
                let raw = company_operating_recipe(w, nation, &district,
                    kind,
                    rate,
                    rate * power_per_pack(w, &district, kind),
                );
                for i in 0..12 {
                    out.operating_daily[i] += raw[i];
                }
            }
        }
    }

    if w.rules.industry_rebuild {
        let operating = crate::industry_operations::demand_daily(w, nation);
        for i in 0..12 { out.operating_daily[i] += operating[i]; }
        for p in production::projects_for(w,nation) {
            let recipe = production::catalog(p.kind).recipe;
            let left = (1.0 - p.progress_fraction()) * crate::industrial_modules::scale(p);
            // Rated demand excludes stockpile constraints so shortages remain
            // visible instead of disappearing when a project is paused.
            let daily = crate::industrial_modules::scale(p) / p.total_days.max(1) as f64;
            for i in 0..12 {
                out.projects_remaining[i] += recipe[i] * left;
                out.projects_daily[i] += recipe[i] * daily;
                for (h,days) in [30.0,90.0,365.0].into_iter().enumerate() {
                    out.projects_horizon[i][h] += (recipe[i] * daily * days).min(recipe[i] * left);
                }
            }
        }
    }
    out.operating_daily = out.operating_daily.map(q);
    out.projects_remaining = out.projects_remaining.map(q);
    out.projects_daily = out.projects_daily.map(q);
    out.projects_horizon = out.projects_horizon.map(|row| row.map(q));
    out.mines_remaining = out.mines_remaining.map(q);
    out.mines_daily = out.mines_daily.map(q);
    out.mines_horizon = out.mines_horizon.map(|row| row.map(q));
    out
}

fn resource_demand_daily_inner(w: &WorldState, nation: NationId, include_materials: bool) -> [f64; 12] {
    let mut out = [0.0; 12];
    if !programs::enrolled(w, nation) {
        return out;
    }
    for d in operating_districts(w) {
        if w.districts.get(&d) != Some(&nation) {
            continue;
        }
        for k in [K::ProcessingPlant, K::StarterIndustry, K::MachineryWorks] {
            let rate = plant_rate(w, &d, k);
            let raw = company_operating_recipe(w, nation, &d, k, rate, rate * power_per_pack(w, &d, k));
            for i in 0..12 {
                out[i] += raw[i];
            }
        }
    }
    if include_materials {
        let materials = crate::materials::resource_demand_daily(w, nation);
        for i in 0..12 { out[i] += materials[i]; }
    }
    if w.rules.industry_rebuild {
        let additional = crate::industry_operations::demand_daily(w, nation);
        for i in 0..12 { out[i] += additional[i]; }
        for p in production::projects_for(w,nation) {
            let recipe = production::catalog(p.kind).recipe;
            let daily = crate::industrial_modules::scale(p) / p.total_days.max(1) as f64;
            for i in 0..12 { out[i] += recipe[i] * daily; }
        }
    }
    out
}
/// Runs after raw resource settlement. Processing precedes machinery, in
/// stable district order. Shared grid/generation capacity and authority are
/// consumed only by a complete, feasible operating bundle.
pub fn tick_day(w: &mut WorldState) {
    if !clock::is_daily(w)
        || !w.rules.production_system
        || !w.rules.resource_market
        || (w.production.industry.sites.is_empty() && w.production.industry.modules.is_empty()
            && !crate::materials::has_work(w))
    {
        return;
    }
    let today = clock::absolute_day(w);
    if w.production.industry.last_day == Some(today) {
        return;
    }
    w.production.industry.last_day = Some(today);
    w.production.industry.operations.clear();
    let sites = operating_districts(w);
    let mut power: BTreeMap<NationId, f64> = BTreeMap::new();
    let mut grids: BTreeMap<String, f64> = BTreeMap::new();
    for kind in [K::ProcessingPlant, K::StarterIndustry, K::MachineryWorks] {
        for d in &sites {
            let level = if kind==K::StarterIndustry {0} else {site_level(w, d, kind)};
            if plant_rate(w,d,kind) <= 0.0 {
                continue;
            }
            let Some(&nation) = w.districts.get(d) else {
                continue;
            };
            let mut status = SiteStatus {
                district: d.clone(),
                kind,
                level,
                capacity_micros: if kind==K::StarterIndustry {w.production.industry.modules.get(d).copied()} else {None},
                status: "blocked".into(),
                reason: None,
                output_daily: 0.0,
                power_used_daily: 0.0,
                cash_spent_daily_bn: 0.0,
            };
            if !funding_day_open(w, nation) || !w.nation(nation).alive {
                status.reason =
                    Some("The controlling government needs an active department budget.".into());
                w.production.industry.operations.push(status);
                continue;
            }
            if resources::district_contested(w, d) {
                status.reason = Some("This province is contested.".into());
                w.production.industry.operations.push(status);
                continue;
            }
            let available_power = power
                .entry(nation)
                .or_insert_with(|| (power_capacity(w, nation) - crate::industry_operations::support_power_used(w, nation)).max(0.0));
            let grid = grids
                .entry(d.clone())
                .or_insert_with(|| (crate::industry_operations::grid_capacity(w, d) - crate::industry_operations::support_grid_used(w, d)).max(0.0));
            let per_power = power_per_pack(w, d, kind);
            let target = plant_rate(w, d, kind) * crate::industry_operations::worker_fraction(w, nation, kind);
            let pile = w
                .production
                .industry
                .goods
                .get(&nation)
                .cloned()
                .unwrap_or_default();
            let stored = if is_processing(kind) {
                pile.intermediates
            } else {
                pile.capital_goods
            };
            let room = (goods_capacity(w, nation) - stored).max(0.0);
            let dept = if is_processing(kind) { 2 } else { 0 };
            let company = manufacturing_company(w, nation, d);
            let (_, energy_fee) = energy_company_rates(w, nation);
            let cash_per_pack = 0.00001 * (1.0 + company.fee_rate);
            let generating_cost_per_power = 0.000002 * (1.0 + energy_fee);
            let mut output = target
                .min(room)
                .min(*available_power / per_power)
                .min(*grid / per_power)
                .min(programs::available_bn(w, nation, BUDGET_INDUSTRY, dept) / cash_per_pack)
                .min(
                    programs::available_bn(w, nation, BUDGET_INDUSTRY, 1)
                        / (generating_cost_per_power * per_power),
                );
            if kind == K::MachineryWorks {
                output = output.min(pile.intermediates / company.input_rate);
            }
            if output <= EPS {
                status.status = "paused".into();
                status.reason = Some(
                    if room <= EPS {
                        "Storage is full; use these packs or build a Warehouse."
                    } else if *available_power <= EPS {
                        "No spare modeled generation; build Power Generation."
                    } else if *grid <= EPS {
                        "No spare local grid capacity; build a Power Grid."
                    } else if kind == K::MachineryWorks && pile.intermediates <= EPS {
                        "No intermediate packs; run Materials Processing first."
                    } else {
                        "No department operating authority is available."
                    }
                    .into(),
                );
                w.production.industry.operations.push(status);
                continue;
            }
            // Proportional feasible output, followed by one atomic raw draw. A
            // missing raw component does not consume the others, cash or power.
            let unit = company_operating_recipe(w, nation, d, kind, 1.0, per_power);
            let mut raw_limiter: Option<(C, f64)> = None;
            for c in ALL {
                if unit[c.idx()] > 0.0 {
                    let limit = resources::stockpile(w, nation, c) / unit[c.idx()];
                    if limit < output {
                        raw_limiter = Some((c, limit.max(0.0)));
                        output = limit.max(0.0);
                    }
                }
            }
            if output <= EPS {
                status.status = "paused".into();
                let missing = ALL.into_iter().find(|commodity| {
                    unit[commodity.idx()] > 0.0
                        && resources::stockpile(w, nation, *commodity)
                            / unit[commodity.idx()]
                            <= EPS
                });
                status.reason = Some(missing.map_or_else(
                    || "Missing raw inputs or generating fuel; this plant is paused, not the national economy.".into(),
                    |commodity| {
                        format!(
                            "Missing {} for the complete operating bundle.",
                            commodity.name()
                        )
                    },
                ));
                w.production.industry.operations.push(status);
                continue;
            }
            // Manufactured stocks use the same nanounit lattice as raw stocks.
            // Round throughput DOWN so capacity/authority never creates a pack.
            output = (output * 1e9).floor() / 1e9;
            if output <= EPS {
                status.status = "paused".into();
                status.reason =
                    Some("Available inputs cannot cover one industrial inventory quantum.".into());
                w.production.industry.operations.push(status);
                continue;
            }
            let draw = company_operating_recipe(w, nation, d, kind, output, output * per_power);
            if let Err((c, _, _)) = resources::consume_stockpile_atomic(w, nation, &draw) {
                status.status = "paused".into();
                status.reason = Some(format!(
                    "Missing {} for the complete operating bundle.",
                    c.name()
                ));
                w.production.industry.operations.push(status);
                continue;
            }
            let cash = (output * cash_per_pack).min(programs::available_bn(
                w,
                nation,
                BUDGET_INDUSTRY,
                dept,
            ));
            let energy_cash = (output * per_power * generating_cost_per_power)
                .min(programs::available_bn(w, nation, BUDGET_INDUSTRY, 1));
            programs::spend_operating(w, nation, BUDGET_INDUSTRY, dept, cash)
                .expect("preflighted operating authority");
            programs::spend_operating(w, nation, BUDGET_INDUSTRY, 1, energy_cash)
                .expect("preflighted generation authority");
            let g = w.production.industry.goods.entry(nation).or_default();
            if is_processing(kind) {
                g.intermediates = q(g.intermediates + output);
            } else {
                g.intermediates = q(g.intermediates - output * company.input_rate);
                g.capital_goods = q(g.capital_goods + output);
            }
            *available_power -= output * per_power;
            *grid -= output * per_power;
            status.status = if output + EPS < target {
                "limited"
            } else {
                "running"
            }
            .into();
            status.output_daily = output;
            status.power_used_daily = output * per_power;
            status.cash_spent_daily_bn = cash + energy_cash;
            if output + EPS < target {
                status.reason = Some(raw_limiter.map_or_else(
                    || "Output is limited by shared power, local grid, storage or department authority.".into(),
                    |(commodity, limit)| format!(
                        "{} supply limits output to {:.0}% of the planned line rate.",
                        commodity.name(),
                        (limit / target.max(EPS) * 100.0).clamp(0.0, 100.0)
                    ),
                ));
            }
            w.production.industry.operations.push(status);
            crate::gdp_projects::record_factory(
                w,
                nation,
                d,
                kind,
                output,
                output * per_power,
                draw,
                cash,
                energy_cash,
            );
            record_manufacturing_work(w, nation, d, output, output * 0.00001);
            record_energy_work(w, nation, output * per_power, output * per_power * 0.000002);
        }
    }
    crate::materials::operate(w, &mut power, &mut grids);
}

pub(crate) fn research_enabled(w: &WorldState) -> bool {
    clock::is_daily(w) && w.rules.economic_competition && w.rules.production_system
        && w.rules.resource_market
}

/// Pure and shared by the technology quote and acquisition charge. Work is
/// never refundable or transferable, and a cheaper world price cannot turn
/// previously purchased prototype work into cash or a negative research bill.
pub fn prototype_credit(w: &WorldState, nation: NationId, technology: u16, base_cost: f64) -> f64 {
    if !research_enabled(w) || !base_cost.is_finite() || base_cost <= 0.0 {
        return 0.0;
    }
    let credit = w.production.industry.research.get(&nation)
        .and_then(|p| p.credits.get(&technology)).copied().unwrap_or(0.0);
    if credit.is_finite() { credit.clamp(0.0, base_cost * PROTOTYPE_COST_SHARE) } else { 0.0 }
}

pub fn research_cost(w: &WorldState, nation: NationId, technology: u16, base_cost: f64) -> f64 {
    credit_adjusted_cost(base_cost, prototype_credit(w, nation, technology, base_cost))
}

pub(crate) fn credit_adjusted_cost(base_cost: f64, credit: f64) -> f64 {
    // Keep the untouched path's exact arithmetic (including signed zero).
    if credit.is_finite() && credit > 0.0 && base_cost.is_finite() && base_cost > 0.0 {
        base_cost - credit.min(base_cost * PROTOTYPE_COST_SHARE)
    } else { base_cost }
}

/// Last settled operations. The receipt carries its day and owner so readers
/// cannot relabel yesterday's work as a promise about the next simulation day.
pub fn research_status(w: &WorldState, nation: NationId) -> Vec<ResearchOperation> {
    if !research_enabled(w) { return vec![]; }
    w.production.industry.research.get(&nation)
        .map(|p| p.operations.clone()).unwrap_or_default()
}

/// Useful prototype work on today's eligible focus, before physical capacity
/// and authority. Also lets the economic planner distinguish a useful lab
/// investment from an empty construction target. It never creates a focus or
/// counts banked effort that can already finish without purchased prototypes.
fn research_targets(w: &WorldState, nation: NationId, work_today: &BTreeMap<(NationId, u16), f64>) -> Vec<(u16, usize, f64, f64)> {
    if !research_enabled(w) || !w.nation_opt(nation).is_some_and(|n| n.alive) { return vec![]; }
    let n = w.nation(nation);
    let dev = crate::tech::dev_of(n);
    let output = crate::tech::research_output(w, n, dev) * clock::month_fraction(w);
    if !output.is_finite() || output <= 0.0 { return vec![]; }
    let weights = crate::tech::domain_weights_of(w, n, dev);
    crate::tech::DOMAINS.iter().filter_map(|domain| {
        let di = domain.index();
        let t = n.tech.focus.get(di).copied().flatten()?;
        let def = crate::tech::registry().get(t as usize)?;
        if def.domain != *domain || n.tech.knows_index(t) || def.earliest_year > w.year
            || !crate::tech::prereqs_of(t).iter().all(|p| n.tech.knows_index(*p))
            || !weights[di].is_finite() || weights[di] <= 0.0 { return None; }
        let base = crate::tech::undiscounted_cost_of(w, nation, t);
        let bank = n.tech.progress.get(di).copied().unwrap_or(0.0);
        if !base.is_finite() || base <= 0.0 || !bank.is_finite() || bank < 0.0 { return None; }
        let credited = prototype_credit(w, nation, t, base);
        let effort = output * weights[di];
        let useful = (base * PROTOTYPE_COST_SHARE - credited).max(0.0)
            .min((base - credited - bank - effort).max(0.0))
            .min((effort * PROTOTYPE_EFFORT_SHARE
                - work_today.get(&(nation, t)).copied().unwrap_or(0.0)).max(0.0));
        (useful > 1e-12).then_some((t, di, weights[di], useful))
    }).collect()
}

pub fn research_work_demand(w: &WorldState, nation: NationId) -> f64 {
    research_targets(w, nation, &BTreeMap::new()).iter().map(|target| target.3).sum()
}

/// Pure DAILY next-operation demand; callers may turn it into a reserve window.
/// Bounded by owned uncontested completed capacity, useful active research and
/// department authority, NOT by goods already on hand (which would hide the
/// very shortage a trade planner needs to fill). Multiple active domains make
/// this an upper bound: each individual center selects one target per day.
pub fn research_goods_demand(w: &WorldState, nation: NationId) -> Goods {
    if !research_enabled(w) || !programs::enrolled(w, nation) { return Goods::default(); }
    let levels: f64 = w.districts.iter().filter(|(d, n)| **n == nation
        && !resources::district_contested(w, d))
        .map(|(d, _)| production::level(w, d, K::ResearchCenter) as f64).sum();
    if levels <= 0.0 { return Goods::default(); }
    let useful = research_work_demand(w, nation);
    if useful <= 0.0 { return Goods::default(); }
    let authority = programs::available_bn(w, nation, BUDGET_SCIENCE, 0);
    if !authority.is_finite() || authority <= 0.0 { return Goods::default(); }
    let work = levels.min(useful / PROTOTYPE_WORK_PER_LEVEL_DAY)
        .min(authority / PROTOTYPE_CASH_PER_LEVEL_DAY_BN);
    Goods {
        intermediates: work * PROTOTYPE_INTERMEDIATES_PER_LEVEL_DAY,
        capital_goods: work * PROTOTYPE_CAPITAL_PER_LEVEL_DAY,
    }
}

/// Called from technology settlement, after factories and before the research
/// charge. Science/Basic research pays once via the existing fiscal ledger;
/// materials are consumed once from manufactured inventory. Education alone
/// still determines research output. No GDP, research bank or generic bonus is
/// granted here: this buys bounded work on one existing project.
pub fn research_day(w: &mut WorldState) {
    if !research_enabled(w) { return; }
    let today = clock::absolute_day(w);
    if w.production.industry.research_day == Some(today) { return; }
    let centers: Vec<_> = w.districts.iter().filter_map(|(district, nation)| {
        let level = production::level(w, district, K::ResearchCenter);
        (level > 0).then(|| (district.clone(), *nation, level))
    }).collect();
    if centers.is_empty() && w.production.industry.research.is_empty() { return; }
    w.production.industry.research_day = Some(today);
    let alive_known: BTreeMap<_, _> = w.nations.iter()
        .map(|n| (n.id, (n.alive, n.tech.known.clone()))).collect();
    for (nation, program) in &mut w.production.industry.research {
        program.operations.clear();
        program.credits.retain(|t, credit| alive_known.get(nation)
            .is_some_and(|(alive, known)| *alive && !known.contains(t))
            && credit.is_finite() && *credit > 0.0);
    }
    let mut work_today: BTreeMap<(NationId, u16), f64> = BTreeMap::new();
    for (district, nation, level) in centers {
        let mut operation = ResearchOperation {
            district: district.clone(), nation, level, day: today,
            technology: None, technology_name: None, status: "idle".into(),
            reason: "No active, eligible research project needs prototype work today.".into(),
            prototype_credit: 0.0, cash_spent_daily_bn: 0.0, goods_used: Goods::default(),
        };
        if !w.nation(nation).alive || resources::district_contested(w, &district) {
            operation.status = "blocked".into();
            operation.reason = "BLOCKED: research center needs a living owner and an uncontested province.".into();
        } else if !funding_day_open(w, nation) {
            operation.status = "blocked".into();
            operation.reason = "BLOCKED: enact an active daily Science department budget.".into();
        } else if crate::industry_operations::operating_fraction(w,&district,K::ResearchCenter) <= EPS {
            operation.status = "paused".into();
            operation.reason = "PAUSED: scientists, electricity or facility operating funds are unavailable; restore the research center's operating service.".into();
        } else {
            let mut candidates = research_targets(w, nation, &work_today);
            // Follow the nation's existing research allocation. Stable domain
            // ordering resolves equal priorities without any additional RNG.
            candidates.sort_by(|a, b| b.2.total_cmp(&a.2).then(a.1.cmp(&b.1)));
            if let Some((t, _, _, useful)) = candidates.first().copied() {
                operation.technology = Some(t);
                operation.technology_name = Some(crate::tech::registry()[t as usize].name.into());
                let target = CompanyTarget::Research { domain: format!("{:?}", crate::tech::registry()[t as usize].domain) };
                let company = companies::modifiers(w, nation, &target);
                let cash_per_work = PROTOTYPE_CASH_PER_LEVEL_DAY_BN * (1.0 + company.fee_rate);
                let authority = programs::available_bn(w, nation, BUDGET_SCIENCE, 0);
                let goods = w.production.industry.goods.get(&nation).cloned().unwrap_or_default();
                if !authority.is_finite() || authority <= 0.0 {
                    operation.status = "paused".into();
                    operation.reason = "PAUSED: Science / Basic research has no available operating authority; prototype work will resume automatically.".into();
                } else if !goods.intermediates.is_finite() || !goods.capital_goods.is_finite()
                    || goods.intermediates <= 0.0 || goods.capital_goods <= 0.0 {
                    operation.status = "paused".into();
                    operation.reason = "PAUSED: prototype testing is waiting for both intermediate and capital-goods packs.".into();
                } else {
                    let desired = (level as f64 * crate::industry_operations::operating_fraction(w,&district,K::ResearchCenter))
                        .min(useful / (PROTOTYPE_WORK_PER_LEVEL_DAY * company.work_rate));
                    let work = desired
                        .min(authority / cash_per_work)
                        .min(goods.intermediates / PROTOTYPE_INTERMEDIATES_PER_LEVEL_DAY)
                        .min(goods.capital_goods / PROTOTYPE_CAPITAL_PER_LEVEL_DAY);
                    if work.is_finite() && work > 1e-12 {
                        let cash = work * cash_per_work;
                        let draw = Goods {
                            intermediates: work * PROTOTYPE_INTERMEDIATES_PER_LEVEL_DAY,
                            capital_goods: work * PROTOTYPE_CAPITAL_PER_LEVEL_DAY,
                        };
                        // All bounds have been checked before either ledger is
                        // written; failed fiscal settlement consumes no goods.
                        if programs::spend_operating(w, nation, BUDGET_SCIENCE, 0, cash).is_ok() {
                            let inventory = w.production.industry.goods.get_mut(&nation).unwrap();
                            inventory.intermediates = (inventory.intermediates - draw.intermediates).max(0.0);
                            inventory.capital_goods = (inventory.capital_goods - draw.capital_goods).max(0.0);
                            let base_credit = work * PROTOTYPE_WORK_PER_LEVEL_DAY;
                            let credit = base_credit * company.work_rate;
                            *w.production.industry.research.entry(nation).or_default().credits.entry(t).or_default() += credit;
                            *work_today.entry((nation, t)).or_default() += credit;
                            operation.prototype_credit = credit;
                            operation.cash_spent_daily_bn = cash;
                            operation.goods_used = draw;
                            operation.status = if work + 1e-12 < desired { "limited" } else { "running" }.into();
                            operation.reason = format!("Prototype/testing work on {}. Specific acquisition credit; no extra research money or direct GDP.{}",
                                operation.technology_name.as_deref().unwrap_or("the active project"),
                                if operation.status == "limited" { " Limited by Science authority or manufactured supplies." } else { "" });
                            companies::record_work(w, nation, &target, base_credit, credit - base_credit,
                                work * PROTOTYPE_CASH_PER_LEVEL_DAY_BN * company.fee_rate);
                            companies::record_sector_activity(w, nation, CompanySector::Research, credit);
                        } else {
                            operation.status = "blocked".into();
                            operation.reason = "BLOCKED: Science operating authority could not settle; no supplies consumed.".into();
                        }
                    }
                }
            }
        }
        crate::gdp_projects::record_research_service(w, &operation);
        w.production.industry.research.entry(nation).or_default().operations.push(operation);
    }
}

#[derive(Serialize)]
pub struct Snapshot {
    pub goods: Goods,
    pub capacity_each: f64,
    pub power_capacity_daily: f64,
    pub power_used_daily: f64,
    pub settled_day: Option<i32>,
    pub sites: Vec<SiteStatus>,
    pub research_operations: Vec<ResearchOperation>,
    pub goods_unit: &'static str,
    pub note: &'static str,
}
pub fn snapshot(w: &WorldState, nation: NationId) -> Snapshot {
    let mut sites = vec![];
    let research_operations = research_status(w, nation);
    for (d, _) in w.districts.iter().filter(|(_, n)| **n == nation) {
        for k in production::PROJECT_KINDS {
            let level = production::level(w, d, k);
            let capacity_micros = if k == K::StarterIndustry {w.production.industry.modules.get(d).copied()} else {None};
            if level == 0 && capacity_micros.unwrap_or(0) == 0 {
                continue;
            }
            sites.push(
                w.production
                    .industry
                    .operations
                    .iter()
                    .find(|s| s.district == *d && s.kind == k)
                    .cloned()
                    .or_else(|| research_operations.iter().find(|s| k == K::ResearchCenter && s.district == *d)
                        .map(|s| SiteStatus {
                            district: d.clone(), kind: k, level, capacity_micros: None, status: s.status.clone(),
                            reason: Some(s.reason.clone()), output_daily: 0.0,
                            power_used_daily: 0.0, cash_spent_daily_bn: s.cash_spent_daily_bn,
                        }))
                    .unwrap_or(SiteStatus {
                        district: d.clone(),
                        kind: k,
                        level,
                        capacity_micros,
                        status: "ready".into(),
                        reason: Some(production::catalog(k).effect.into()),
                        output_daily: 0.0,
                        power_used_daily: 0.0,
                        cash_spent_daily_bn: 0.0,
                    }),
            );
        }
    }
    Snapshot {
        goods: w
            .production
            .industry
            .goods
            .get(&nation)
            .cloned()
            .unwrap_or_default(),
        capacity_each: goods_capacity(w, nation),
        power_capacity_daily: power_capacity(w, nation),
        power_used_daily: sites.iter().map(|s| s.power_used_daily).sum::<f64>()
            + crate::materials::power_used_daily(w, nation),
        settled_day: w.production.industry.last_day,
        sites,
        research_operations,
        goods_unit: "modeled industrial packs",
        note: if crate::province_economy::active(w) {
            "Goods are inventory, not cash. Actual production less intermediate inputs appears as modeled value added in province GDP. No inferred 1990 factories or power stations."
        } else {
            "Incremental civilian facilities only; no inferred 1990 factories or power stations. Packs supply operating and research activity, not construction payments or raw deposits."
        },
    }
}

pub fn mine_key(district: &str, c: C) -> String {
    format!("{district}:{}", c.key())
}
pub fn enroll_mine(w: &mut WorldState, district: &str, c: C, total_days: u32) {
    w.production.industry.mines.insert(
        mine_key(district, c),
        MineFunding {
            total_days,
            ..Default::default()
        },
    );
}
/// Pure mine quote against the remaining shared budget after earlier queue
/// entries. Legacy prepaid rows have no funding entry and return None.
pub fn mine_work_plan(w: &WorldState, p: &resources::MineProject, available_bn: f64) -> Option<WorkPlan> {
    let f = w.production.industry.mines.get(&mine_key(&p.district, p.commodity))?;
    let mut plan = WorkPlan::default();
    if !clock::is_daily(w) || !programs::enrolled(w, p.started_by)
        || w.districts.get(&p.district) != Some(&p.started_by)
        || !w.nation(p.started_by).alive || resources::district_contested(w, &p.district) {
        plan.reason = Some("BLOCKED: mine requires its sponsor's controlled, uncontested province and an active construction budget.".into());
    } else if f.last_day == Some(clock::absolute_day(w)) {
        plan.reason = Some("Today's mine construction has already settled.".into());
    }
    if plan.reason.is_some() { return Some(plan); }
    let remaining = (f.total_days as f64 - f.progress_days).max(0.0);
    let due = (p.investment_bn - f.spent_bn).max(0.0);
    let price = if remaining > EPS { due / remaining } else { 0.0 };
    let available = available_bn.max(0.0).min(programs::construction_available_bn(w, p.started_by));
    plan.target_advance_days = if w.rules.industry_rebuild { crate::construction_capacity::mine_nominal_work(w,p) }
        else { remaining.min(1.0) };
    if plan.target_advance_days <= EPS && w.rules.industry_rebuild {
        plan.slow_reason=Some("PAUSED: no construction capacity assigned to this mine. Use Auto or assign capacity.".into());
    }
    plan.advance_days = if price > 0.0 { plan.target_advance_days.min(available / price) } else { plan.target_advance_days };
    plan.cash_bn = (if plan.advance_days + EPS >= remaining { due } else { price * plan.advance_days }).min(available).min(due);
    if price > 0.0 { plan.advance_days = (plan.cash_bn / price).min(plan.target_advance_days); }
    if plan.advance_days + EPS < plan.target_advance_days {
        plan.slow_reason = Some(if plan.advance_days <= EPS {
            "PAUSED: today's construction budget is exhausted; completed mine work is preserved.".into()
        } else { "SLOWED: today's construction budget funds part of this mine's work.".into() });
    }
    Some(plan)
}
/// One daily-funded mapped mine. Legacy prepaid rows have no funding entry.
pub fn advance_mine(w: &mut WorldState, p: &resources::MineProject) -> Option<f64> {
    let key = mine_key(&p.district, p.commodity);
    let f = w.production.industry.mines.get(&key)?.clone();
    let today = clock::absolute_day(w);
    if f.last_day == Some(today) { return Some(f.progress_days); }
    let mut next = f.clone();
    next.last_day = Some(today);
    let plan = mine_work_plan(w, p, programs::construction_available_bn(w, p.started_by))?;
    next.reason = plan.reason.clone().or_else(|| plan.slow_reason.clone());
    if !funding_day_open(w, p.started_by) {
        next.reason = Some("BLOCKED: the daily construction ledger is not open.".into());
    } else if plan.reason.is_none() && plan.advance_days > EPS {
        if let Err(reason) = programs::spend_construction(w, p.started_by, plan.cash_bn) {
            next.reason = Some(reason);
        } else {
            next.progress_days = (f.progress_days + plan.advance_days).min(f.total_days as f64);
            next.spent_bn += plan.cash_bn;
        }
    }
    let progress = next.progress_days;
    let advance = (next.progress_days - f.progress_days).max(0.0);
    let paid = (next.spent_bn - f.spent_bn).max(0.0);
    w.production.industry.mines.insert(key, next);
    if advance > 0.0 { crate::gdp_projects::record_mine_construction(w, p, advance, paid, true); }
    Some(progress)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{init::world_1990, load, save, world::GameRules};
    const USA: NationId = NationId::USA;
    fn prepared() -> WorldState {
        let mut w = world_1990(GameRules {
            daily_simulation: true,
            resource_market: true,
            production_system: true,
            ..GameRules::default()
        });
        w.player = Some(USA);
        let year = w.year;
        programs::install(&mut w, USA, year, programs::default_departments());
        programs::begin_day(&mut w);
        for c in ALL {
            if c != C::Oil {
                resources::set_stockpile_for_test(&mut w, USA, c, 1_000_000.0);
            }
        }
        w
    }
    fn districts(w: &WorldState) -> Vec<String> {
        w.districts
            .iter()
            .filter(|(_, n)| **n == USA)
            .map(|(d, _)| d.clone())
            .collect()
    }
    fn next_day(w: &mut WorldState) {
        programs::stage_fiscal(w.nation_mut(USA), 0.0, 0.0);
        programs::finish_day(w);
        clock::advance_date(w);
        programs::begin_day(w);
    }
    fn near(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-8, "{a} != {b}");
    }
    fn chain(w: &mut WorldState, d: &str) {
        production::complete_capability(w, d, K::CivilianIndustry);
        production::complete_capability(w, d, K::PowerGrid);
        for k in [K::Generation, K::ProcessingPlant, K::MachineryWorks] {
            complete_site(w, d, k);
        }
    }
    fn company_for(w: &WorldState, sector: CompanySector, saving: bool) -> u32 {
        w.companies.roster.iter().filter(|c| c.nation == USA && c.sector == sector)
            .max_by(|a, b| if saving { a.input_saving.total_cmp(&b.input_saving) } else { a.work_bonus.total_cmp(&b.work_bonus) })
            .unwrap().id
    }
    #[test]
    fn rebuilt_construction_scales_company_fees_with_reserved_physical_inputs() {
        let mut w=prepared();
        w.rules.industry_rebuild=true;
        let district=districts(&w)[0].clone();
        programs::set_construction_budget(&mut w,USA,1.0).unwrap();
        let id=production::start_project(&mut w,USA,&district,K::Warehouse).unwrap();
        companies::enable(&mut w);
        let target=CompanyTarget::Construction{project:id};
        let company=company_for(&w,CompanySector::Construction,false);
        companies::assign(&mut w,USA,company,target).unwrap();
        let full=project_plans(&w)[&id].clone();
        assert!(full.required[C::Iron.idx()]>0.0&&full.company_fee_bn>0.0);
        resources::set_stockpile_for_test(&mut w,USA,C::Iron,full.required[C::Iron.idx()]*0.5);
        let limited=project_plans(&w)[&id].clone();
        near(limited.advance_days,full.advance_days*0.5);
        near(limited.cash_bn,full.cash_bn*0.5);
        near(limited.company_fee_bn,full.company_fee_bn*0.5);
        let before=programs::construction_available_bn(&w,USA);
        production::tick_day(&mut w);
        near(before-programs::construction_available_bn(&w,USA),limited.cash_bn);
        near(w.production.industry.projects[&id].spent_bn,limited.cash_bn-limited.company_fee_bn);
        near(w.companies.assignments[0].fees_today_bn,limited.company_fee_bn);
        near(resources::stockpile(&w,USA,C::Iron),0.0);
        let once=save(&w);production::tick_day(&mut w);assert_eq!(save(&w),once);
    }
    #[test]
    fn company_construction_is_funded_performed_work_and_replays_once() {
        let mut w = prepared();
        let district = districts(&w)[0].clone();
        programs::set_construction_budget(&mut w, USA, 1.0).unwrap();
        let project = production::start_project(&mut w, USA, &district, K::Warehouse).unwrap();
        companies::enable(&mut w);
        let mut plain = w.clone();
        let company = company_for(&w, CompanySector::Construction, false);
        let target = CompanyTarget::Construction { project };
        companies::assign(&mut w, USA, company, target.clone()).unwrap();
        let modifier = companies::modifiers(&w, USA, &target);
        let plan = project_plans(&w)[&project].clone();
        assert!(plan.advance_days > project_plans(&plain)[&project].advance_days);
        assert!(plan.company_fee_bn > 0.0);
        production::tick_day(&mut w);
        production::tick_day(&mut plain);
        assert!(w.production.projects[0].progress_days > plain.production.projects[0].progress_days);
        near(w.production.industry.projects[&project].spent_bn, plan.cash_bn - plan.company_fee_bn);
        let assignment = &w.companies.assignments[0];
        near(assignment.fees_today_bn, plan.company_fee_bn);
        assert!(assignment.total_work > 0.0 && assignment.total_bonus > 0.0);
        near(plan.advance_days, modifier.work_rate);
        let saved = save(&w);
        let mut restored = load(&saved).unwrap();
        production::tick_day(&mut restored);
        assert_eq!(save(&restored), saved, "same day cannot charge or train twice");
        next_day(&mut restored);
        programs::set_construction_budget(&mut restored, USA, 0.0).unwrap();
        let experience = restored.companies.roster.iter().find(|c| c.id == company).unwrap().experience;
        let progress = restored.production.projects[0].progress_days;
        production::tick_day(&mut restored);
        near(restored.production.projects[0].progress_days, progress);
        near(restored.companies.roster.iter().find(|c| c.id == company).unwrap().experience, experience);
    }
    #[test]
    fn company_factories_save_real_inputs_and_missing_inputs_charge_nothing() {
        let mut w = prepared();
        let d = districts(&w)[0].clone();
        chain(&mut w, &d);
        companies::enable(&mut w);
        let company = company_for(&w, CompanySector::Manufacturing, true);
        let target = CompanyTarget::Facility { district: d.clone(), sector: CompanySector::Manufacturing };
        companies::assign(&mut w, USA, company, target.clone()).unwrap();
        let modifier = companies::modifiers(&w, USA, &target);
        assert!(modifier.input_rate < 1.0);
        let opening_iron = resources::stockpile(&w, USA, C::Iron);
        tick_day(&mut w);
        let output = w.production.industry.operations.iter().find(|o| o.kind == K::ProcessingPlant).unwrap().output_daily;
        assert!(output > 0.0);
        near(opening_iron - resources::stockpile(&w, USA, C::Iron), output * modifier.input_rate);
        assert!(w.companies.assignments[0].fees_today_bn > 0.0);
        let saved = save(&w);
        tick_day(&mut w);
        assert_eq!(save(&w), saved);
        next_day(&mut w);
        resources::set_stockpile_for_test(&mut w, USA, C::Coal, 0.0);
        let company_before = w.companies.roster.clone();
        let goods_before = w.production.industry.goods.clone();
        tick_day(&mut w);
        assert!(w.production.industry.operations.iter().all(|o| o.output_daily == 0.0 && o.cash_spent_daily_bn == 0.0));
        assert_eq!(w.companies.roster, company_before, "stalled firms earn neither fees nor experience");
        assert_eq!(w.production.industry.goods, goods_before);
    }
    #[test]
    fn company_generators_charge_for_dispatch_and_save_only_generating_fuel() {
        let mut w = prepared();
        let d = districts(&w)[0].clone();
        chain(&mut w, &d);
        companies::enable(&mut w);
        let mut plain = w.clone();
        let company = company_for(&w, CompanySector::Energy, true);
        let target = CompanyTarget::Facility { district: d.clone(), sector: CompanySector::Energy };
        companies::assign(&mut w, USA, company, target.clone()).unwrap();
        let modifier = companies::modifiers(&w, USA, &target);
        let coal_before = resources::stockpile(&w, USA, C::Coal);
        tick_day(&mut w);
        tick_day(&mut plain);
        assert_eq!(w.production.industry.goods, plain.production.industry.goods);
        let actual_power: f64 = w.production.industry.operations.iter().map(|o| o.power_used_daily).sum();
        let saving = resources::stockpile(&w, USA, C::Coal) - resources::stockpile(&plain, USA, C::Coal);
        near(saving, actual_power * 0.02 * (1.0 - modifier.input_rate));
        assert!(coal_before > resources::stockpile(&w, USA, C::Coal));
        near(w.companies.assignments[0].fees_today_bn, actual_power * 0.000002 * modifier.fee_rate);
    }
    #[test]
    fn company_cannot_charge_or_claim_bonus_through_module_commissioning_ceiling() {
        let mut w = prepared();
        let district = districts(&w)[0].clone();
        programs::set_construction_budget(&mut w, USA, 1.0).unwrap();
        crate::industrial_modules::start(&mut w, USA, &district, 1).unwrap();
        let project = w.production.projects[0].id;
        companies::enable(&mut w);
        let company = company_for(&w, CompanySector::Construction, false);
        let target = CompanyTarget::Construction { project };
        let mut plain = w.clone();
        companies::assign(&mut w, USA, company, target).unwrap();
        let plan = project_plans(&w)[&project].clone();
        let baseline = project_plans(&plain)[&project].clone();
        near(plan.advance_days, baseline.advance_days);
        near(plan.cash_bn, baseline.cash_bn);
        near(plan.company_fee_bn, 0.0);
        production::tick_day(&mut w);
        production::tick_day(&mut plain);
        assert_eq!(w.production.projects, plain.production.projects);
        near(w.companies.assignments[0].bonus_today, 0.0);
        near(w.companies.assignments[0].fees_today_bn, 0.0);
        assert!(w.companies.roster.iter().find(|c|c.id == company).unwrap().experience < 0.001,
            "tiny normalized modules cannot farm a full-size work day's experience");
    }
    #[test]
    fn untouched_industry_is_byte_inert_and_absent_from_saves() {
        for daily in [false, true] {
            let mut w = world_1990(GameRules {
                daily_simulation: daily,
                ..GameRules::default()
            });
            let before = save(&w);
            tick_day(&mut w);
            begin_work_day(&mut w);
            assert_eq!(save(&w), before);
            assert!(!before.contains("\"industry\""));
        }
    }
    #[test]
    fn new_industrial_demand_does_not_mint_opening_resources() {
        let mut w = prepared();
        w.resources.market = None;
        let opening = ALL.map(|c| resources::stockpile(&w, USA, c));
        let demand = resources::draw(&w, USA);
        let d = districts(&w)[0].clone();
        production::start_project(&mut w, USA, &d, K::Warehouse).unwrap();
        assert_eq!(resources::draw(&w, USA), demand, "turnkey construction creates no raw demand");
        let components = raw_demand_components(&w, USA);
        assert_eq!(components.projects_remaining, [0.0; 12]);
        assert_eq!(components.projects_horizon, [[0.0; 3]; 12]);
        assert_eq!(ALL.map(|c| resources::stockpile(&w, USA, c)), opening);
        assert!(
            w.resources.market.is_none(),
            "a forecast must not materialize stock"
        );
    }
    #[test]
    fn whole_queue_reserves_one_budget_without_material_or_site_capacity_gates() {
        let mut w = prepared();
        let ds = districts(&w);
        programs::set_construction_budget(&mut w, USA, 0.0001).unwrap();
        for d in ds.iter().take(6) {
            production::start_project(&mut w, USA, d, K::CivilianIndustry).unwrap();
        }
        for c in ALL { resources::set_stockpile_for_test(&mut w, USA, c, 0.0); }
        let raw = w.resources.clone();
        let first = w.production.projects[0].id;
        let last = w.production.projects[5].id;
        production::set_priority(&mut w, USA, last, production::Priority::High).unwrap();
        let before = save(&w);
        let plans = project_plans(&w);
        assert_eq!(save(&w), before, "preview is pure");
        near(plans.values().map(|p| p.cash_bn).sum(), 0.0001);
        assert_eq!(plans[&first].advance_days, 0.0);
        near(plans[&last].advance_days, 0.3);
        production::tick_day(&mut w);
        assert_eq!(w.resources, raw);
        near(w.nation(USA).program_budget.as_ref().unwrap().construction_spent_today_bn, 0.0001);
        let once = save(&w);
        production::tick_day(&mut w);
        assert_eq!(save(&w), once, "same-day replay cannot double-build");
    }

    #[test]
    fn partial_budget_scales_cash_and_work_without_consuming_inputs() {
        let mut w = prepared();
        let d = districts(&w)[0].clone();
        let id = production::start_project(&mut w, USA, &d, K::Warehouse).unwrap();
        let full = project_plans(&w)[&id].clone();
        near(full.advance_days, 1.0);
        assert_eq!(full.required, [0.0; 12]);
        assert_eq!(full.goods, Goods::default());
        programs::set_construction_budget(&mut w, USA, full.cash_bn * 0.5).unwrap();
        for c in ALL { resources::set_stockpile_for_test(&mut w, USA, c, 0.0); }
        let raw = w.resources.clone();
        let slowed = project_plans(&w)[&id].clone();
        near(slowed.advance_days, 0.5);
        near(slowed.cash_bn, full.cash_bn * 0.5);
        assert!(slowed.slow_reason.as_deref().unwrap().contains("budget"));
        production::tick_day(&mut w);
        let p = &w.production.projects[0];
        assert_eq!(p.status, ProjectStatus::Slowed);
        near(p.progress_days, 0.5);
        assert_eq!(p.resources_used, [0.0; 12]);
        assert_eq!(w.resources, raw);
        near(w.production.industry.projects[&id].spent_bn, slowed.cash_bn);
        near(project_finance(&w,p).unwrap().spent_today_bn, slowed.cash_bn);
    }

    #[test]
    fn construction_uses_pooled_civilian_capital_not_maintenance_or_procurement() {
        let mut w = prepared();
        let d = districts(&w)[0].clone();
        production::start_project(&mut w, USA, &d, K::ArmsPlant).unwrap();
        let before = w.nation(USA).program_budget.as_ref().unwrap().clone();
        let plan = project_plans(&w).into_values().next().unwrap();
        assert!(plan.cash_bn > 0.0);
        production::tick_day(&mut w);
        let after = w.nation(USA).program_budget.as_ref().unwrap();
        near(after.construction_spent_today_bn - before.construction_spent_today_bn, plan.cash_bn);
        near(after.spent_today_bn[crate::world::BUDGET_INFRASTRUCTURE][4], before.spent_today_bn[crate::world::BUDGET_INFRASTRUCTURE][4]);
        assert_eq!(after.available_bn[crate::world::BUDGET_DEFENSE], before.available_bn[crate::world::BUDGET_DEFENSE]);
        assert_eq!(after.prepaid_bn, before.prepaid_bn);
        assert!(after.spent_today_bn[BUDGET_INDUSTRY].iter().sum::<f64>() > before.spent_today_bn[BUDGET_INDUSTRY].iter().sum::<f64>());
    }

    #[test]
    fn civilian_chain_conserves_inputs_power_cash_and_produces_real_goods() {
        let mut w = prepared();
        let d = districts(&w)[0].clone();
        chain(&mut w, &d);
        let iron = resources::stockpile(&w, USA, C::Iron);
        let coal = resources::stockpile(&w, USA, C::Coal);
        let copper = resources::stockpile(&w, USA, C::Copper);
        let gdp = w.nation(USA).gdp;
        let debt = w.nation(USA).debt_gdp;
        tick_day(&mut w);
        let goods = &w.production.industry.goods[&USA];
        near(goods.intermediates, 0.5);
        near(goods.capital_goods, 0.5);
        near(iron - resources::stockpile(&w, USA, C::Iron), 1.0);
        near(coal - resources::stockpile(&w, USA, C::Coal), 0.29);
        near(copper - resources::stockpile(&w, USA, C::Copper), 0.05);
        near(snapshot(&w, USA).power_used_daily, 2.0);
        assert_eq!(w.nation(USA).gdp, gdp);
        assert_eq!(
            w.nation(USA).debt_gdp,
            debt,
            "cash is charged once by the closing fiscal ledger"
        );
        let plan = w.nation(USA).program_budget.as_ref().unwrap();
        near(plan.noncapital_spent_today_bn[BUDGET_INDUSTRY][0], 0.000005);
        near(plan.noncapital_spent_today_bn[BUDGET_INDUSTRY][2], 0.00001);
        near(plan.noncapital_spent_today_bn[BUDGET_INDUSTRY][1], 0.000004);
        let once = save(&w);
        tick_day(&mut w);
        assert_eq!(save(&w), once);
        let mut restored = load(&once).unwrap();
        next_day(&mut w);
        next_day(&mut restored);
        tick_day(&mut w);
        tick_day(&mut restored);
        assert_eq!(save(&w), save(&restored));
    }
    #[test]
    fn no_power_or_fuel_pauses_only_new_activity_and_does_not_partially_draw() {
        let mut w = prepared();
        let d = districts(&w)[0].clone();
        chain(&mut w, &d);
        resources::set_stockpile_for_test(&mut w, USA, C::Coal, 0.0);
        let raw = w.resources.clone();
        let spending = w
            .nation(USA)
            .program_budget
            .as_ref()
            .unwrap()
            .spent_today_bn;
        let gdp = w.nation(USA).gdp;
        tick_day(&mut w);
        assert_eq!(w.resources, raw);
        assert_eq!(
            w.nation(USA)
                .program_budget
                .as_ref()
                .unwrap()
                .spent_today_bn,
            spending
        );
        assert_eq!(w.nation(USA).gdp, gdp);
        assert!(w.production.industry.goods.is_empty());
        resources::set_stockpile_for_test(&mut w, USA, C::Coal, 100.0);
        next_day(&mut w);
        w.production.industry.sites.get_mut(&d).unwrap()[site_index(K::Generation)] = 0;
        tick_day(&mut w);
        assert!(w.production.industry.goods.is_empty());
        assert!(snapshot(&w, USA).sites.iter().any(|s| s
            .reason
            .as_deref()
            .is_some_and(|r| r.contains("generation"))));
    }
    #[test]
    fn goods_storage_upgrade_and_retrofits_have_distinct_operating_consumers() {
        let mut w = prepared();
        let d = districts(&w)[0].clone();
        chain(&mut w, &d);
        w.production.industry.goods.insert(
            USA,
            Goods {
                intermediates: 250.0,
                capital_goods: 250.0,
            },
        );
        tick_day(&mut w);
        assert!(w
            .production
            .industry
            .operations
            .iter()
            .all(|s| s.output_daily == 0.0));
        complete_site(&mut w, &d, K::Warehouse);
        near(goods_capacity(&w, USA), 500.0);
        next_day(&mut w);
        tick_day(&mut w);
        assert!(w
            .production
            .industry
            .operations
            .iter()
            .any(|s| s.output_daily > 0.0));
        let mut plain = prepared();
        chain(&mut plain, &d);
        let mut efficient = plain.clone();
        complete_site(&mut efficient, &d, K::Efficiency);
        tick_day(&mut plain);
        tick_day(&mut efficient);
        near(
            plain.production.industry.goods[&USA].capital_goods,
            efficient.production.industry.goods[&USA].capital_goods,
        );
        assert!(
            snapshot(&efficient, USA).power_used_daily < snapshot(&plain, USA).power_used_daily
        );
        let mut automated = prepared();
        chain(&mut automated, &d);
        complete_site(&mut automated, &d, K::Automation);
        tick_day(&mut automated);
        assert!(
            automated.production.industry.goods[&USA].capital_goods
                > plain.production.industry.goods[&USA].capital_goods
        );
    }
    #[test]
    fn manufactured_goods_remain_available_for_operations_during_construction() {
        let mut w = prepared();
        let d = districts(&w)[0].clone();
        production::start_project(&mut w, USA, &d, K::Warehouse).unwrap();
        assert!(project_finance(&w, &w.production.projects[0]).unwrap().reason.is_none());
        let goods = Goods { intermediates: 12.0, capital_goods: 5.0 };
        w.production.industry.goods.insert(USA, goods.clone());
        let gdp = w.nation(USA).gdp;
        production::tick_day(&mut w);
        assert_eq!(w.production.industry.goods[&USA], goods);
        assert_eq!(w.nation(USA).gdp, gdp);
        assert_eq!(w.production.projects[0].progress_days, 1.0);
    }

    #[test]
    fn completed_project_closes_exact_cash_and_preserves_inventory() {
        let mut w = prepared();
        let d = districts(&w)[0].clone();
        let kind = K::Warehouse;
        w.production.industry.goods.insert(USA, Goods { intermediates: 12.0, capital_goods: 5.0 });
        let raw = w.resources.clone();
        let goods = w.production.industry.goods.clone();
        production::start_project(&mut w, USA, &d, kind).unwrap();
        let mut paid = 0.0;
        for day in 0..240 {
            production::tick_day(&mut w);
            paid += w.nation(USA).program_budget.as_ref().unwrap().construction_spent_today_bn;
            if day < 239 { assert_eq!(production::level(&w,&d,kind), 0); next_day(&mut w); }
        }
        assert!(w.production.projects.is_empty());
        assert_eq!(production::level(&w, &d, kind), 1);
        assert!(w.production.industry.projects.is_empty());
        assert_eq!(w.production.industry.goods, goods);
        assert_eq!(w.resources, raw);
        near(paid, work_cost_bn(kind));
        let once = save(&w);
        production::tick_day(&mut w);
        assert_eq!(save(&w), once);
    }

    #[test]
    fn full_civilian_chain_bootstraps_from_raw_inputs_without_seeded_goods() {
        let mut w = prepared();
        let d = districts(&w)[0].clone();
        assert!(w.production.industry.goods.is_empty());
        for kind in [
            K::CivilianIndustry,
            K::PowerGrid,
            K::Generation,
            K::ProcessingPlant,
            K::MachineryWorks,
            K::Warehouse,
        ] {
            production::start_project(&mut w, USA, &d, kind).unwrap();
            for _ in 0..1000 {
                if w.nation(USA).program_budget.as_ref().unwrap().fiscal_year != w.year {
                    let year = w.year;
                    programs::install(&mut w, USA, year, programs::default_departments());
                }
                production::tick_day(&mut w);
                tick_day(&mut w);
                if w.production.projects.is_empty() {
                    break;
                }
                next_day(&mut w);
            }
            assert_eq!(
                production::level(&w, &d, kind),
                1,
                "{:?}: {:?}",
                kind,
                w.production.projects
            );
            next_day(&mut w);
        }
        let g = &w.production.industry.goods[&USA];
        assert!(g.intermediates > 0.0 && g.capital_goods > 0.0);
        near(goods_capacity(&w, USA), 500.0);
        assert!(w
            .production
            .industry
            .operations
            .iter()
            .any(|s| s.kind == K::MachineryWorks && s.output_daily > 0.0));
    }
    #[test]
    fn new_mines_share_funding_but_have_independent_site_lead_times() {
        let mut w = prepared();
        let (d, c) = w
            .districts
            .iter()
            .filter(|(_, n)| **n == USA)
            .flat_map(|(d, _)| ALL.map(|c| (d.clone(), c)))
            .find(|(d, c)| resources::mine_refusal(&w, USA, d, *c).is_none())
            .unwrap();
        let treasury = w.nation(USA).treasury_bn;
        let debt = w.nation(USA).debt_gdp;
        resources::start_mine(&mut w, USA, &d, c).unwrap();
        assert_eq!(resources::mine_investment_bn(&w, USA), 0.0);
        assert_eq!(w.nation(USA).treasury_bn, treasury);
        assert_eq!(w.nation(USA).debt_gdp, debt);
        let other = districts(&w).into_iter().find(|x| *x != d).unwrap();
        production::start_project(&mut w, USA, &other, K::CivilianIndustry).unwrap();
        begin_work_day(&mut w);
        let project_plan = project_plans(&w).into_values().next().unwrap();
        production::tick_day(&mut w);
        let p = w.resources.mine_projects[0].clone();
        let progress = advance_mine(&mut w, &p).unwrap();
        assert!(progress > 0.0);
        near(progress, 1.0);
        near(project_plan.advance_days, 1.0);
        near(w.nation(USA).program_budget.as_ref().unwrap().construction_spent_today_bn,
            project_plan.cash_bn + w.production.industry.mines[&mine_key(&d,c)].spent_bn);
        assert!(w.production.industry.mines[&mine_key(&d, c)].spent_bn > 0.0);
        near(
            resources::mine_investment_bn(&w, USA),
            w.production.industry.mines[&mine_key(&d, c)].spent_bn,
        );
        let once = save(&w);
        advance_mine(&mut w, &p);
        assert_eq!(save(&w), once);
        next_day(&mut w);
        w.districts.insert(d.clone(), NationId::Canada);
        advance_mine(&mut w, &p);
        near(
            w.production.industry.mines[&mine_key(&d, c)].progress_days,
            progress,
        );
    }

    #[test]
    fn mine_construction_uses_partial_budget_and_no_materials() {
        let mut w = prepared();
        let (d,c) = w.districts.iter().filter(|(_, n)| **n == USA)
            .flat_map(|(d,_)| ALL.map(|c| (d.clone(),c)))
            .find(|(d,c)| resources::mine_refusal(&w,USA,d,*c).is_none()).unwrap();
        resources::start_mine(&mut w,USA,&d,c).unwrap();
        let project = w.resources.mine_projects[0].clone();
        let funding = w.production.industry.mines[&mine_key(&d,c)].clone();
        let daily_cost = project.investment_bn / funding.total_days as f64;
        programs::set_construction_budget(&mut w,USA,daily_cost * 0.5).unwrap();
        for c in ALL { resources::set_stockpile_for_test(&mut w,USA,c,0.0); }
        let raw = w.resources.clone();
        let before = save(&w);
        let preview = mine_work_plan(&w,&project,programs::construction_available_bn(&w,USA)).unwrap();
        assert_eq!(save(&w),before);
        near(preview.advance_days,0.5);
        let progress = advance_mine(&mut w,&project).unwrap();
        near(progress,preview.advance_days);
        let settled = &w.production.industry.mines[&mine_key(&d,c)];
        near(settled.spent_bn,daily_cost*0.5);
        assert!(settled.reason.as_deref().unwrap().contains("budget"));
        assert_eq!(settled.resources_used,[0.0;12]);
        assert_eq!(w.resources,raw);
        let forecast = raw_demand_components(&w,USA);
        assert_eq!(forecast.mines_remaining,[0.0;12]);
        assert_eq!(forecast.mines_horizon,[[0.0;3];12]);
    }

    #[test]
    fn migrated_contract_preserves_paid_history_and_only_prices_unfinished_work() {
        let mut w = prepared();
        let d = districts(&w)[0].clone();
        let id = production::start_project(&mut w,USA,&d,K::Warehouse).unwrap();
        w.production.projects[0].progress_days = 120.0;
        w.production.projects[0].resources_used[C::Iron.idx()] = 9.0;
        let f = w.production.industry.projects.get_mut(&id).unwrap();
        f.contract_cost_bn = None; // Save predating frozen contracts.
        f.spent_bn = 0.012;
        f.goods_used.intermediates = 3.0;
        let before = save(&w);
        let quote = project_finance(&w,&w.production.projects[0]).unwrap();
        assert_eq!(save(&w),before);
        near(quote.cost_bn,0.042);
        near(quote.remaining_bn,0.03);
        begin_work_day(&mut w);
        let f = &w.production.industry.projects[&id];
        near(f.spent_bn,0.012);
        near(f.goods_used.intermediates,3.0);
        assert_eq!(w.production.projects[0].progress_days,120.0);
        assert_eq!(w.production.projects[0].resources_used[C::Iron.idx()],9.0);
        let mut resumed = load(&save(&w)).unwrap();
        let mut paid = 0.0;
        for day in 0..120 {
            production::tick_day(&mut w);
            production::tick_day(&mut resumed);
            paid += w.nation(USA).program_budget.as_ref().unwrap().construction_spent_today_bn;
            assert_eq!(save(&w),save(&resumed));
            if day < 119 { next_day(&mut w); next_day(&mut resumed); }
        }
        assert!(w.production.projects.is_empty());
        near(paid,0.03);
    }

    #[test]
    fn legacy_daily_rows_gain_financing_and_budget_activation_preserves_sunk_work() {
        let mut w = prepared();
        let d = districts(&w)[0].clone();
        let id = production::start_project(&mut w,USA,&d,K::Warehouse).unwrap();
        w.production.projects[0].progress_days = 120.0;
        w.production.projects[0].resources_used[C::Iron.idx()] = 9.0;
        w.production.industry.projects.remove(&id);
        w.nation_mut(USA).program_budget = None;
        let raw = w.resources.clone();
        production::tick_day(&mut w);
        near(w.production.projects[0].progress_days,120.0);
        assert_eq!(w.production.projects[0].status,ProjectStatus::Paused);
        near(w.production.industry.projects[&id].spent_bn,0.03);
        near(w.production.industry.projects[&id].contract_cost_bn.unwrap(),0.06);
        programs::set_construction_budget(&mut w,USA,0.00025).unwrap();
        programs::begin_day(&mut w);
        production::tick_day(&mut w);
        near(w.production.projects[0].progress_days,121.0);
        near(w.nation(USA).program_budget.as_ref().unwrap().construction_spent_today_bn,0.00025);
        near(w.production.industry.projects[&id].spent_bn,0.03025);
        assert_eq!(w.resources,raw);
        assert_eq!(w.production.projects[0].resources_used[C::Iron.idx()],9.0);
    }

    #[test]
    fn mine_preview_reserves_only_budget_left_after_the_building_queue() {
        let mut w = prepared();
        let (d,c) = w.districts.iter().filter(|(_, n)| **n == USA)
            .flat_map(|(d,_)| ALL.map(|c| (d.clone(),c)))
            .find(|(d,c)| resources::mine_refusal(&w,USA,d,*c).is_none()).unwrap();
        resources::start_mine(&mut w,USA,&d,c).unwrap();
        let project = w.resources.mine_projects[0].clone();
        let other = districts(&w).into_iter().find(|x| *x != d).unwrap();
        production::start_project(&mut w,USA,&other,K::Warehouse).unwrap();
        let daily_mine = project.investment_bn / w.production.industry.mines[&mine_key(&d,c)].total_days as f64;
        let cap = 0.00025 + daily_mine * 0.5;
        programs::set_construction_budget(&mut w,USA,cap).unwrap();
        let building = project_plans(&w).into_values().next().unwrap();
        let preview = mine_work_plan(&w,&project,programs::construction_available_bn(&w,USA)-building.cash_bn).unwrap();
        near(preview.advance_days,0.5);
        production::tick_day(&mut w);
        near(advance_mine(&mut w,&project).unwrap(),preview.advance_days);
        near(w.nation(USA).program_budget.as_ref().unwrap().construction_spent_today_bn,cap);
        assert_eq!(programs::construction_available_bn(&w,USA),0.0);
    }

}
