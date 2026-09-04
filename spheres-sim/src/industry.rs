//! Opt-in, incremental civilian industry. Coefficients below are GAME recipes,
//! not claims about historical factories, generating stations or district GDP.
//! Industrial packs are manufactured goods, not additional mapped minerals.
//! There is no sales/profit cash or flat GDP multiplier: goods must be used by
//! real construction. When province GDP is enabled, actual production sends
//! value-added receipts to that ledger. Only new activity can lack power.
use crate::{
    clock,
    production::{self, Project, ProjectKind as K, ProjectSpec, ProjectStatus},
    programs,
    resources::{self, Commodity as C, ALL},
    world::{NationId, WorldState, BUDGET_INDUSTRY},
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
    /// Modeled processing packs; one pack feeds one machinery pack, or upgrades.
    pub intermediates: f64,
    /// Modeled machine/tool packs, consumed by new capital projects.
    pub capital_goods: f64,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct ProjectFunding {
    pub spent_bn: f64,
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
    pub status: String,
    pub reason: Option<String>,
    pub output_daily: f64,
    pub power_used_daily: f64,
    pub cash_spent_daily_bn: f64,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Industry {
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
    pub work: BTreeMap<NationId, (i32, f64, f64)>,
}
impl Industry {
    pub fn is_empty(&self) -> bool {
        self.sites.is_empty()
            && self.goods.is_empty()
            && self.projects.is_empty()
            && self.mines.is_empty()
            && self.operations.is_empty()
            && self.last_day.is_none()
            && self.work.is_empty()
    }
}

pub fn catalog(kind: K) -> ProjectSpec {
    let (name, description, effect, days, recipe) = match kind {
        K::MachineryWorks => ("Machinery Works", "Tool and machine production for civilian capital projects.", "Uses intermediate packs, copper and power to produce 0.5 capital-goods packs/day per level.", 420, [(C::Iron,40.0),(C::Copper,12.0),(C::Coal,15.0)]),
        K::Generation => ("Power Generation", "New dispatchable generation for modeled industrial activity only.", "Supplies up to 10 modeled power units/day per level; burns coal only when a factory runs.", 480, [(C::Iron,50.0),(C::Copper,18.0),(C::Coal,20.0)]),
        K::ProcessingPlant => ("Materials Processing", "Converts ore and fuel into usable industrial intermediates.", "Uses iron, bauxite, coal and power to produce 1 intermediate pack/day per level.", 360, [(C::Iron,35.0),(C::Copper,8.0),(C::Coal,20.0)]),
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

/// Modeled installation/labor bill, $bn, separate from already-bought inputs.
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
    }
}
pub fn goods_recipe(kind: K) -> Goods {
    match kind {
        K::FreightTerminal => Goods {
            intermediates: 20.0,
            capital_goods: 10.0,
        },
        K::Warehouse => Goods {
            intermediates: 12.0,
            capital_goods: 5.0,
        },
        K::Automation => Goods {
            intermediates: 5.0,
            capital_goods: 15.0,
        },
        K::Efficiency => Goods {
            intermediates: 5.0,
            capital_goods: 8.0,
        },
        _ => Goods::default(), // Basic chain bootstraps from the twelve raw lines.
    }
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
    if programs::enrolled(w, nation) && resources::district_contested(w, district) {
        return Some("Construction cannot start in a contested province.".into());
    }
    if !extended(kind) {
        return None;
    }
    if !programs::enrolled(w, nation) {
        return Some(
            "Enact the five-department budget before commissioning this industrial investment."
                .into(),
        );
    }
    if resources::district_contested(w, district) {
        return Some("Construction cannot start in a contested province.".into());
    }
    match kind {
        K::FreightTerminal if !w.rules.physical_logistics || !w.rules.logistics_routes => {
            return Some("Enable physical freight routes before upgrading a terminal.".into())
        }
        K::MachineryWorks | K::ProcessingPlant
            if production::level(w, district, K::CivilianIndustry) == 0 =>
        {
            return Some("Build an Industrial Estate in this province first.".into())
        }
        K::FreightTerminal if !crate::logistics::has_terminal(district) => {
            return Some("This province has no mapped coastal freight gateway to upgrade.".into())
        }
        K::Automation | K::Efficiency => {
            if site_level(w, district, K::MachineryWorks) == 0
                && site_level(w, district, K::ProcessingPlant) == 0
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
    for p in w.production.projects.iter().filter(|p| p.nation == nation) {
        let f = p.progress_fraction();
        let g = goods_recipe(p.kind);
        w.production
            .industry
            .projects
            .entry(p.id)
            .or_insert(ProjectFunding {
                spent_bn: work_cost_bn(p.kind) * f,
                goods_used: Goods {
                    intermediates: g.intermediates * f,
                    capital_goods: g.capital_goods * f,
                },
                last_day: None,
            });
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct WorkPlan {
    pub advance_days: f64,
    pub cash_bn: f64,
    pub required: [f64; 12],
    pub goods: Goods,
    pub reason: Option<String>,
    pub department_draws_bn: [f64; 5],
}
fn q(v: f64) -> f64 {
    (v.max(0.0) * 1e9).round() / 1e9
}
fn total_work_weight(w: &WorldState, nation: NationId) -> f64 {
    production::projects_for(w, nation)
        .map(|p| p.priority.weight())
        .sum::<f64>()
        + w.resources
            .mine_projects
            .iter()
            .filter(|p| {
                p.started_by == nation
                    && w.production
                        .industry
                        .mines
                        .contains_key(&mine_key(&p.district, p.commodity))
            })
            .count() as f64
}
fn allocated_work(w: &WorldState, nation: NationId, weight: f64) -> f64 {
    let (total, capacity) = w
        .production
        .industry
        .work
        .get(&nation)
        .filter(|(day, _, _)| *day == clock::absolute_day(w))
        .map_or_else(
            || {
                (
                    total_work_weight(w, nation),
                    production::construction_capacity(w, nation),
                )
            },
            |(_, total, capacity)| (*total, *capacity),
        );
    capacity * weight / total.max(1.0)
}
/// Freeze queue weights before any completion removes a project. Mines and
/// projects consequently cannot each claim the same national workforce.
pub fn begin_work_day(w: &mut WorldState) {
    let today = clock::absolute_day(w);
    let nations: Vec<_> = w
        .nations
        .iter()
        .filter(|n| n.alive && programs::enrolled(w, n.id))
        .map(|n| n.id)
        .collect();
    for nation in nations {
        if w.production
            .industry
            .work
            .get(&nation)
            .is_some_and(|(d, _, _)| *d == today)
        {
            continue;
        }
        let weight = total_work_weight(w, nation);
        if weight > 0.0 {
            let capacity = production::construction_capacity(w, nation);
            w.production
                .industry
                .work
                .insert(nation, (today, weight, capacity));
        }
    }
}
pub fn project_authority(w: &WorldState, nation: NationId, kind: K) -> f64 {
    let m = production::catalog(kind).funding_ministry;
    if kind == K::Infrastructure {
        (0..4)
            .map(|d| programs::available_bn(w, nation, m, d))
            .sum()
    } else {
        programs::available_bn(w, nation, m, production::funding_department(kind))
    }
}
/// Pure opening-state allocator. Priority orders scarce authority AND inputs;
/// each department, resource and good is reserved once across the whole queue.
pub fn project_plans(w: &WorldState) -> BTreeMap<u32, WorkPlan> {
    let mut out = BTreeMap::new();
    let mut cash = BTreeMap::new();
    let mut stocks: BTreeMap<NationId, [f64; 12]> = BTreeMap::new();
    let mut goods = w.production.industry.goods.clone();
    let mut queue: Vec<_> = w
        .production
        .projects
        .iter()
        .filter(|p| w.production.industry.projects.contains_key(&p.id))
        .collect();
    queue.sort_by_key(|p| (p.priority.dispatch_rank(), p.id));
    for p in queue {
        let f = &w.production.industry.projects[&p.id];
        let mut plan = WorkPlan::default();
        if !clock::is_daily(w) || !programs::enrolled(w, p.nation) {
            plan.reason = Some("BLOCKED: daily department funding is not enabled.".into());
        } else if w.districts.get(&p.district) != Some(&p.nation) || !w.nation(p.nation).alive {
            plan.reason =
                Some("BLOCKED: sponsoring government no longer controls this province.".into());
        } else if resources::district_contested(w, &p.district) {
            plan.reason = Some("BLOCKED: this province is contested.".into());
        } else if f.last_day == Some(clock::absolute_day(w)) {
            plan.reason = Some("Today's construction has already settled.".into());
        }
        if plan.reason.is_some() {
            out.insert(p.id, plan);
            continue;
        }
        let spec = production::catalog(p.kind);
        let dept = production::funding_department(p.kind);
        let departments: Vec<usize> = if p.kind == K::Infrastructure {
            (0..4).collect()
        } else {
            vec![dept]
        };
        let balances: Vec<f64> = departments
            .iter()
            .map(|d| {
                *cash
                    .entry((p.nation, spec.funding_ministry, *d))
                    .or_insert_with(|| {
                        programs::available_bn(w, p.nation, spec.funding_ministry, *d)
                    })
            })
            .collect();
        let balance = balances.iter().sum::<f64>();
        let cost = work_cost_bn(p.kind);
        let remaining = (p.total_days as f64 - p.progress_days).max(0.0);
        let speed = 1.0 + production::level(w, &p.district, K::Infrastructure) as f64 * 0.10;
        let capacity = (allocated_work(w, p.nation, p.priority.weight()) * speed).min(1.5);
        plan.advance_days = capacity
            .min(remaining)
            .min(balance / (cost / p.total_days as f64));
        if plan.advance_days <= EPS {
            plan.reason = Some(format!(
                "BLOCKED: {} has no available project authority.",
                programs::NAMES[spec.funding_ministry][dept]
            ));
            out.insert(p.id, plan);
            continue;
        }
        let completes = plan.advance_days + EPS >= remaining;
        let fraction = plan.advance_days / p.total_days as f64;
        plan.cash_bn = (if completes {
            (cost - f.spent_bn).max(0.0)
        } else {
            (cost * fraction).min((cost - f.spent_bn).max(0.0))
        })
        .min(balance);
        let mut unassigned = plan.cash_bn;
        for (index, d) in departments.iter().enumerate() {
            let part = (if index + 1 == departments.len() {
                unassigned.max(0.0)
            } else {
                (plan.cash_bn * balances[index] / balance).min(unassigned)
            })
            .min(balances[index]);
            plan.department_draws_bn[*d] = part;
            unassigned -= part;
        }
        plan.cash_bn = plan.department_draws_bn.iter().sum();
        plan.required = std::array::from_fn(|i| {
            q(if completes {
                spec.recipe[i] - p.resources_used[i]
            } else {
                (spec.recipe[i] * fraction).min((spec.recipe[i] - p.resources_used[i]).max(0.0))
            })
        });
        let recipe = goods_recipe(p.kind);
        plan.goods = Goods {
            intermediates: q(if completes {
                (recipe.intermediates - f.goods_used.intermediates).max(0.0)
            } else {
                recipe.intermediates * fraction
            }),
            capital_goods: q(if completes {
                (recipe.capital_goods - f.goods_used.capital_goods).max(0.0)
            } else {
                recipe.capital_goods * fraction
            }),
        };
        let stock = stocks
            .entry(p.nation)
            .or_insert_with(|| std::array::from_fn(|i| resources::stockpile(w, p.nation, ALL[i])));
        let pile = goods.entry(p.nation).or_default();
        if let Some(c) = ALL
            .into_iter()
            .find(|c| stock[c.idx()] < plan.required[c.idx()])
        {
            plan.reason = Some(format!(
                "BLOCKED: needs {:.3} {} today, have {:.3}.",
                plan.required[c.idx()],
                c.name(),
                stock[c.idx()]
            ));
        } else if pile.intermediates < plan.goods.intermediates {
            plan.reason = Some(format!(
                "BLOCKED: needs {:.3} intermediate packs, have {:.3}.",
                plan.goods.intermediates, pile.intermediates
            ));
        } else if pile.capital_goods < plan.goods.capital_goods {
            plan.reason = Some(format!(
                "BLOCKED: needs {:.3} capital-goods packs, have {:.3}.",
                plan.goods.capital_goods, pile.capital_goods
            ));
        }
        if plan.reason.is_none() {
            for d in &departments {
                *cash
                    .get_mut(&(p.nation, spec.funding_ministry, *d))
                    .unwrap() -= plan.department_draws_bn[*d];
            }
            for i in 0..12 {
                stock[i] -= plan.required[i];
            }
            pile.intermediates = q(pile.intermediates - plan.goods.intermediates);
            pile.capital_goods = q(pile.capital_goods - plan.goods.capital_goods);
        }
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
        return Err("BLOCKED: the daily department ledger is not open.".into());
    }
    let ministry = production::catalog(p.kind).funding_ministry;
    for d in 0..5 {
        if programs::available_bn(w, p.nation, ministry, d) < plan.department_draws_bn[d] {
            return Err("BLOCKED: department authority was exhausted.".into());
        }
    }
    let pile = w
        .production
        .industry
        .goods
        .get(&p.nation)
        .cloned()
        .unwrap_or_default();
    if pile.intermediates < plan.goods.intermediates
        || pile.capital_goods < plan.goods.capital_goods
    {
        return Err("BLOCKED: industrial goods are no longer available.".into());
    }
    resources::consume_stockpile_atomic(w, p.nation, &plan.required)
        .map_err(|(c, _, _)| format!("BLOCKED: {} is no longer available.", c.name()))?;
    for d in 0..5 {
        if plan.department_draws_bn[d] > 0.0 {
            programs::spend(w, p.nation, ministry, d, plan.department_draws_bn[d])
                .expect("preflighted daily department authority");
        }
    }
    if plan.goods.intermediates > 0.0 || plan.goods.capital_goods > 0.0 {
        let g = w.production.industry.goods.entry(p.nation).or_default();
        g.intermediates = q(g.intermediates - plan.goods.intermediates);
        g.capital_goods = q(g.capital_goods - plan.goods.capital_goods);
    }
    let today = clock::absolute_day(w);
    let f = w.production.industry.projects.get_mut(&p.id).unwrap();
    f.spent_bn += plan.cash_bn;
    f.goods_used.intermediates = q(f.goods_used.intermediates + plan.goods.intermediates);
    f.goods_used.capital_goods = q(f.goods_used.capital_goods + plan.goods.capital_goods);
    f.last_day = Some(today);
    let row = w
        .production
        .projects
        .iter_mut()
        .find(|row| row.id == p.id)
        .unwrap();
    row.progress_days = (row.progress_days + plan.advance_days).min(row.total_days as f64);
    for i in 0..12 {
        row.resources_used[i] += plan.required[i];
    }
    row.status = if plan.advance_days < 0.8 {
        ProjectStatus::Slowed
    } else {
        ProjectStatus::Building
    };
    row.reason = if plan.advance_days < 0.8 {
        Some("SLOWED: available department authority and shared construction capacity limit today's work.".into())
    } else {
        None
    };
    crate::gdp_projects::record_construction(w, p, plan.advance_days, plan.cash_bn, true);
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
    pub next_work_days: f64,
    pub goods_recipe: Goods,
    pub goods_used: Goods,
    pub next_goods: Goods,
    pub reason: Option<String>,
}
pub fn project_finance(w: &WorldState, p: &Project) -> Option<ProjectFinanceView> {
    let f = w.production.industry.projects.get(&p.id)?;
    let plan = project_plans(w).remove(&p.id).unwrap_or_default();
    let d = production::funding_department(p.kind);
    Some(ProjectFinanceView {
        department: d,
        department_name: if p.kind == K::Infrastructure {
            "Network works (roads, rail, ports and airports)"
        } else {
            programs::NAMES[production::catalog(p.kind).funding_ministry][d]
        },
        department_draws_bn: plan.department_draws_bn,
        cost_bn: work_cost_bn(p.kind),
        spent_bn: f.spent_bn,
        remaining_bn: (work_cost_bn(p.kind) - f.spent_bn).max(0.0),
        daily_request_bn: plan.cash_bn,
        next_work_days: if plan.reason.is_some() {
            0.0
        } else {
            plan.advance_days
        },
        goods_recipe: goods_recipe(p.kind),
        goods_used: f.goods_used.clone(),
        next_goods: plan.goods,
        reason: plan.reason,
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
    w.production
        .industry
        .sites
        .iter()
        .filter(|(d, _)| {
            w.districts.get(*d) == Some(&nation) && !resources::district_contested(w, d)
        })
        .map(|(_, v)| v[site_index(K::Generation)] as f64 * 10.0)
        .sum()
}
pub(crate) fn plant_rate(w: &WorldState, district: &str, kind: K) -> f64 {
    let base = if kind == K::ProcessingPlant { 1.0 } else { 0.5 };
    base * site_level(w, district, kind) as f64
        * (1.0 + site_level(w, district, K::Automation) as f64 * 0.2)
}
pub(crate) fn power_per_pack(w: &WorldState, district: &str, kind: K) -> f64 {
    (if kind == K::ProcessingPlant { 1.0 } else { 2.0 })
        * (1.0 - site_level(w, district, K::Efficiency) as f64 * 0.1).max(0.5)
}
pub(crate) fn operating_recipe(kind: K, output: f64, power: f64) -> [f64; 12] {
    let mut raw = [0.0; 12];
    raw[C::Coal.idx()] = power * 0.02;
    if kind == K::ProcessingPlant {
        raw[C::Iron.idx()] = output;
        raw[C::Bauxite.idx()] = output * 0.2;
        raw[C::Coal.idx()] += output * 0.25;
    } else {
        raw[C::Copper.idx()] = output * 0.1;
    }
    raw.map(q)
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
    let mut out = [0.0; 12];
    if !programs::enrolled(w, nation) {
        return out;
    }
    for (d, _) in &w.production.industry.sites {
        if w.districts.get(d) != Some(&nation) {
            continue;
        }
        for k in [K::ProcessingPlant, K::MachineryWorks] {
            let rate = plant_rate(w, d, k);
            let raw = operating_recipe(k, rate, rate * power_per_pack(w, d, k));
            for i in 0..12 {
                out[i] += raw[i];
            }
        }
    }
    // Construction demand is a forecast of attempted work, not a free stock
    // grant. The resource market explicitly excludes this from legacy cover.
    for p in production::projects_for(w, nation)
        .filter(|p| w.production.industry.projects.contains_key(&p.id))
    {
        if w.districts.get(&p.district) != Some(&nation)
            || resources::district_contested(w, &p.district)
        {
            continue;
        }
        let spec = production::catalog(p.kind);
        let speed = 1.0 + production::level(w, &p.district, K::Infrastructure) as f64 * 0.10;
        let advance = (allocated_work(w, nation, p.priority.weight()) * speed)
            .min(1.5)
            .min((p.total_days as f64 - p.progress_days).max(0.0));
        for i in 0..12 {
            out[i] += q((spec.recipe[i] * advance / p.total_days as f64)
                .min((spec.recipe[i] - p.resources_used[i]).max(0.0)));
        }
    }
    for p in w
        .resources
        .mine_projects
        .iter()
        .filter(|p| p.started_by == nation)
    {
        let Some(f) = w
            .production
            .industry
            .mines
            .get(&mine_key(&p.district, p.commodity))
        else {
            continue;
        };
        if w.districts.get(&p.district) != Some(&nation)
            || resources::district_contested(w, &p.district)
        {
            continue;
        }
        let advance = allocated_work(w, nation, 1.0)
            .min(1.5)
            .min((f.total_days as f64 - f.progress_days).max(0.0));
        for (c, amount) in [(C::Iron, 20.0), (C::Copper, 4.0), (C::Coal, 8.0)] {
            out[c.idx()] += q(amount * advance / f.total_days.max(1) as f64);
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
        || w.production.industry.sites.is_empty()
    {
        return;
    }
    let today = clock::absolute_day(w);
    if w.production.industry.last_day == Some(today) {
        return;
    }
    w.production.industry.last_day = Some(today);
    w.production.industry.operations.clear();
    let sites: Vec<String> = w.production.industry.sites.keys().cloned().collect();
    let mut power: BTreeMap<NationId, f64> = BTreeMap::new();
    let mut grids: BTreeMap<String, f64> = BTreeMap::new();
    for kind in [K::ProcessingPlant, K::MachineryWorks] {
        for d in &sites {
            let level = site_level(w, d, kind);
            if level == 0 {
                continue;
            }
            let Some(&nation) = w.districts.get(d) else {
                continue;
            };
            let mut status = SiteStatus {
                district: d.clone(),
                kind,
                level,
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
                .or_insert_with(|| power_capacity(w, nation));
            let grid = grids
                .entry(d.clone())
                .or_insert_with(|| production::level(w, d, K::PowerGrid) as f64 * 5.0);
            let per_power = power_per_pack(w, d, kind);
            let target = plant_rate(w, d, kind);
            let pile = w
                .production
                .industry
                .goods
                .get(&nation)
                .cloned()
                .unwrap_or_default();
            let stored = if kind == K::ProcessingPlant {
                pile.intermediates
            } else {
                pile.capital_goods
            };
            let room = (goods_capacity(w, nation) - stored).max(0.0);
            let dept = if kind == K::ProcessingPlant { 2 } else { 0 };
            let cash_per_pack = 0.00001;
            let generating_cost_per_power = 0.000002;
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
                output = output.min(pile.intermediates);
            }
            if output <= EPS {
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
            let unit = operating_recipe(kind, 1.0, per_power);
            for c in ALL {
                if unit[c.idx()] > 0.0 {
                    output = output.min(resources::stockpile(w, nation, c) / unit[c.idx()]);
                }
            }
            if output <= EPS {
                status.reason=Some("Missing raw inputs or generating fuel; this plant is paused, not the national economy.".into());
                w.production.industry.operations.push(status);
                continue;
            }
            // Manufactured stocks use the same nanounit lattice as raw stocks.
            // Round throughput DOWN so capacity/authority never creates a pack.
            output = (output * 1e9).floor() / 1e9;
            if output <= EPS {
                status.reason =
                    Some("Available inputs cannot cover one industrial inventory quantum.".into());
                w.production.industry.operations.push(status);
                continue;
            }
            let draw = operating_recipe(kind, output, output * per_power);
            if let Err((c, _, _)) = resources::consume_stockpile_atomic(w, nation, &draw) {
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
            if kind == K::ProcessingPlant {
                g.intermediates = q(g.intermediates + output);
            } else {
                g.intermediates = q(g.intermediates - output);
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
                status.reason=Some("Output is limited by shared power, local grid, inputs, storage or department authority.".into());
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
        }
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
    pub goods_unit: &'static str,
    pub note: &'static str,
}
pub fn snapshot(w: &WorldState, nation: NationId) -> Snapshot {
    let mut sites = vec![];
    for (d, _) in w.districts.iter().filter(|(_, n)| **n == nation) {
        for k in production::PROJECT_KINDS {
            let level = production::level(w, d, k);
            if level == 0 {
                continue;
            }
            sites.push(
                w.production
                    .industry
                    .operations
                    .iter()
                    .find(|s| s.district == *d && s.kind == k)
                    .cloned()
                    .unwrap_or(SiteStatus {
                        district: d.clone(),
                        kind: k,
                        level,
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
        power_used_daily: sites.iter().map(|s| s.power_used_daily).sum(),
        settled_day: w.production.industry.last_day,
        sites,
        goods_unit: "modeled industrial packs",
        note: if crate::province_economy::active(w) {
            "Goods are inventory, not cash. Actual production less intermediate inputs appears as modeled value added in province GDP. No inferred 1990 factories or power stations."
        } else {
            "Incremental civilian facilities only; no inferred 1990 factories or power stations. Packs are usable construction inventory, not revenue or raw deposits."
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
/// Finance and materials for one already-validated mapped mine. Legacy prepaid
/// projects have no entry and never call this branch.
pub fn advance_mine(w: &mut WorldState, p: &resources::MineProject) -> Option<f64> {
    let key = mine_key(&p.district, p.commodity);
    let f = w.production.industry.mines.get(&key)?.clone();
    let today = clock::absolute_day(w);
    if f.last_day == Some(today) {
        return Some(f.progress_days);
    }
    let mut next = f.clone();
    next.last_day = Some(today);
    next.reason = None;
    if !clock::is_daily(w)
        || !funding_day_open(w, p.started_by)
        || w.districts.get(&p.district) != Some(&p.started_by)
        || !w.nation(p.started_by).alive
        || resources::district_contested(w, &p.district)
    {
        next.reason=Some("BLOCKED: mine requires its sponsor's controlled, uncontested province and daily department budget.".into());
    }
    if next.reason.is_none() {
        let daily_cost = p.investment_bn / f.total_days.max(1) as f64;
        let advance = allocated_work(w, p.started_by, 1.0)
            .min(1.5)
            .min(f.total_days as f64 - f.progress_days)
            .min(programs::available_bn(w, p.started_by, BUDGET_INDUSTRY, 2) / daily_cost.max(EPS));
        if advance <= EPS {
            next.reason = Some("BLOCKED: Minerals & processing has no project authority.".into());
        } else {
            // Modeled mine-installation materials; the work-price excludes raw
            // input purchases. No commodity output is granted before completion.
            let mut recipe = [0.0; 12];
            recipe[C::Iron.idx()] = 20.0;
            recipe[C::Copper.idx()] = 4.0;
            recipe[C::Coal.idx()] = 8.0;
            let completes = f.progress_days + advance + EPS >= f.total_days as f64;
            let draw = std::array::from_fn(|i| {
                q(if completes {
                    recipe[i] - f.resources_used[i]
                } else {
                    recipe[i] * advance / f.total_days as f64
                })
            });
            let cash = (if completes {
                (p.investment_bn - f.spent_bn).max(0.0)
            } else {
                daily_cost * advance
            })
            .min(programs::available_bn(w, p.started_by, BUDGET_INDUSTRY, 2));
            if let Err((c, _, _)) = resources::consume_stockpile_atomic(w, p.started_by, &draw) {
                next.reason = Some(format!("BLOCKED: mine construction needs {}.", c.name()));
            } else {
                programs::spend(w, p.started_by, BUDGET_INDUSTRY, 2, cash)
                    .expect("preflighted mine authority");
                next.progress_days = (f.progress_days + advance).min(f.total_days as f64);
                next.spent_bn += cash;
                for i in 0..12 {
                    next.resources_used[i] += draw[i];
                }
            }
        }
    }
    let progress = next.progress_days;
    let advance = (next.progress_days - f.progress_days).max(0.0);
    let paid = (next.spent_bn - f.spent_bn).max(0.0);
    w.production.industry.mines.insert(key, next);
    if advance > 0.0 {
        crate::gdp_projects::record_mine_construction(w, p, advance, paid, true);
    }
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
        assert!(resources::draw(&w, USA)[C::Iron.idx()] > demand[C::Iron.idx()]);
        assert_eq!(ALL.map(|c| resources::stockpile(&w, USA, c)), opening);
        assert!(
            w.resources.market.is_none(),
            "a forecast must not materialize stock"
        );
    }
    #[test]
    fn whole_queue_reserves_one_department_and_atomic_material_bundles() {
        let mut w = prepared();
        let ds = districts(&w);
        for d in ds.iter().take(2) {
            production::start_project(&mut w, USA, d, K::CivilianIndustry).unwrap();
        }
        w.nation_mut(USA)
            .program_budget
            .as_mut()
            .unwrap()
            .available_bn[BUDGET_INDUSTRY][0] = 0.0001;
        let before = save(&w);
        let plans = project_plans(&w);
        assert_eq!(save(&w), before, "preview is pure");
        assert!(
            plans
                .values()
                .filter(|p| p.reason.is_none())
                .map(|p| p.cash_bn)
                .sum::<f64>()
                <= 0.0001
        );
        let first = w.production.projects[0].id;
        let missing = C::Copper;
        resources::set_stockpile_for_test(&mut w, USA, missing, 0.0);
        let raw = w.resources.clone();
        let funds = programs::available_bn(&w, USA, BUDGET_INDUSTRY, 0);
        production::tick_day(&mut w);
        assert_eq!(w.resources, raw);
        near(programs::available_bn(&w, USA, BUDGET_INDUSTRY, 0), funds);
        assert_eq!(
            w.production
                .projects
                .iter()
                .find(|p| p.id == first)
                .unwrap()
                .progress_days,
            0.0
        );
        resources::set_stockpile_for_test(&mut w, USA, missing, 100.0);
        production::tick_day(&mut w);
        assert!(w.production.projects[0].progress_days > 0.0);
        assert!(
            w.nation(USA)
                .program_budget
                .as_ref()
                .unwrap()
                .spent_today_bn[BUDGET_INDUSTRY][0]
                <= 0.0001
        );
        let once = save(&w);
        production::tick_day(&mut w);
        assert_eq!(
            save(&w),
            once,
            "same day cannot double-build or change a successful project's status"
        );
    }
    #[test]
    fn infrastructure_uses_all_four_capital_departments_not_maintenance() {
        let mut w = prepared();
        let d = districts(&w)[0].clone();
        production::start_project(&mut w, USA, &d, K::Infrastructure).unwrap();
        let before = w
            .nation(USA)
            .program_budget
            .as_ref()
            .unwrap()
            .spent_today_bn;
        let plan = project_plans(&w).into_values().next().unwrap();
        assert!(plan.department_draws_bn[..4].iter().all(|v| *v > 0.0));
        assert_eq!(plan.department_draws_bn[4], 0.0);
        production::tick_day(&mut w);
        let after = w
            .nation(USA)
            .program_budget
            .as_ref()
            .unwrap()
            .spent_today_bn;
        for d in 0..4 {
            near(
                after[crate::world::BUDGET_INFRASTRUCTURE][d]
                    - before[crate::world::BUDGET_INFRASTRUCTURE][d],
                plan.department_draws_bn[d],
            );
        }
        near(
            after[crate::world::BUDGET_INFRASTRUCTURE][4],
            before[crate::world::BUDGET_INFRASTRUCTURE][4],
        );
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
    fn manufactured_goods_are_consumed_by_construction_not_sold_for_fake_profit() {
        let mut w = prepared();
        let d = districts(&w)[0].clone();
        let id = production::start_project(&mut w, USA, &d, K::Warehouse).unwrap();
        assert!(project_finance(&w, &w.production.projects[0])
            .unwrap()
            .reason
            .unwrap()
            .contains("intermediate"));
        w.production.industry.goods.insert(
            USA,
            Goods {
                intermediates: 12.0,
                capital_goods: 5.0,
            },
        );
        let plan = project_plans(&w)[&id].clone();
        let gdp = w.nation(USA).gdp;
        production::tick_day(&mut w);
        near(
            w.production.industry.goods[&USA].intermediates,
            12.0 - plan.goods.intermediates,
        );
        near(
            w.production.industry.goods[&USA].capital_goods,
            5.0 - plan.goods.capital_goods,
        );
        assert_eq!(w.nation(USA).gdp, gdp);
    }
    #[test]
    fn completed_project_closes_exact_cash_goods_and_raw_recipe() {
        let mut w = prepared();
        let d = districts(&w)[0].clone();
        let kind = K::Warehouse;
        let recipe = production::catalog(kind).recipe;
        w.production.industry.goods.insert(USA, goods_recipe(kind));
        for c in ALL {
            if recipe[c.idx()] > 0.0 {
                resources::set_stockpile_for_test(&mut w, USA, c, recipe[c.idx()]);
            }
        }
        production::start_project(&mut w, USA, &d, kind).unwrap();
        for _ in 0..300 {
            production::tick_day(&mut w);
            if w.production.projects.is_empty() {
                break;
            }
            next_day(&mut w);
        }
        assert!(
            w.production.projects.is_empty(),
            "{:?}",
            w.production.projects
        );
        assert_eq!(production::level(&w, &d, kind), 1);
        assert!(w.production.industry.projects.is_empty());
        assert_eq!(w.production.industry.goods[&USA], Goods::default());
        for c in ALL {
            if recipe[c.idx()] > 0.0 {
                near(resources::stockpile(&w, USA, c), 0.0);
            }
        }
        near(
            w.nation(USA).program_budget.as_ref().unwrap().spent_ytd_bn[BUDGET_INDUSTRY][3],
            work_cost_bn(kind),
        );
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
    fn new_mines_replace_upfront_debit_and_share_work_with_projects() {
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
        near(
            progress + project_plan.advance_days,
            production::construction_capacity(&w, USA),
        );
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
}
