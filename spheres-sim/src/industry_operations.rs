//! One operating vocabulary for the inherited economy and commissioned sites.
//! Workforce, power and conversion coefficients are explicit GAME estimates.
//! Opening inherited output already has workers, utility service and suppliers;
//! it is never debited again or paid a second GDP award. Only spare reconciled
//! utility capacity can serve new activity. New receipts spend actual inputs.
use crate::{clock, economy, gdp_projects, industrial_modules as modules, industry,
    production::{self, ProjectKind as K}, programs, resources::{self, Commodity as C},
    starting_industry, world::{NationId, WorldState, BUDGET_INDUSTRY}};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

const EPS: f64 = 1e-9;
pub const INHERITED_CONSTRUCTION_PER_EQUIVALENT: f64 = 0.02;
pub const INHERITED_POWER_PER_EQUIVALENT: f64 = 0.02;
pub const INHERITED_SPARE_POWER_SHARE: f64 = 0.10;
pub const OFFICE_GROSS_ANNUAL_BN: f64 = 0.10;
pub const OFFICE_INTERMEDIATES_DAY: f64 = 0.01;
pub const ADVANCED_OUTPUT_DAY: f64 = 0.20;
pub const ADVANCED_INTERMEDIATES_DAY: f64 = 0.40;
pub const OPERATING_CASH_LEVEL_DAY_BN: f64 = 0.00001;
pub const ENERGY_CASH_POWER_DAY_BN: f64 = 0.000002;

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct State {
    pub support_day: Option<i32>,
    pub last_day: Option<i32>,
    pub receipts: Vec<FacilityOperation>,
    pub advanced_components: BTreeMap<NationId, f64>,
}
impl State {
    pub fn is_empty(&self) -> bool {
        self.support_day.is_none() && self.last_day.is_none() && self.receipts.is_empty() && self.advanced_components.is_empty()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FacilityOperation {
    #[serde(default)]
    pub nation: Option<NationId>,
    #[serde(default)]
    pub recorded_day: Option<i32>,
    pub district: String,
    pub kind: String,
    pub name: String,
    pub inherited: bool,
    pub installed_capacity: f64,
    /// Today's feasible staffed operating levels; not a production receipt.
    pub operating_capacity: f64,
    pub utilization: f64,
    #[serde(default)]
    pub actual_operating_capacity: Option<f64>,
    #[serde(default)]
    pub actual_utilization: Option<f64>,
    /// Actual settled output only. Before settlement this is zero.
    pub output_daily: f64,
    pub output_unit: String,
    pub jobs_required: f64,
    pub jobs_filled: f64,
    pub power_required_daily: f64,
    pub power_used_daily: f64,
    pub worker_fraction: f64,
    pub power_fraction: f64,
    pub input_fraction: f64,
    pub funding_fraction: f64,
    pub status: String,
    pub reason: String,
    pub annual_gdp_bn: f64,
    pub annual_tax_bn: f64,
    pub cash_spent_daily_bn: f64,
}

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct OperationsSnapshot {
    pub nation: NationId,
    pub as_of_day: Option<i32>,
    pub inherited_factory_equivalents: f64,
    pub inherited_output_annual_bn: f64,
    pub inherited_jobs_required: f64,
    pub inherited_jobs_filled: f64,
    pub inherited_power_required_daily: f64,
    pub inherited_power_used_daily: f64,
    pub power_capacity_daily: f64,
    pub power_required_daily: f64,
    pub workers_available: f64,
    pub jobs_required: f64,
    pub jobs_filled: f64,
    pub facilities: Vec<FacilityOperation>,
    pub advanced_components_stock: f64,
    pub advanced_components_capacity: f64,
    pub advanced_components_required_daily: f64,
    pub note: String,
}

pub fn enabled(w: &WorldState) -> bool {
    w.rules.industry_rebuild && clock::is_daily(w) && w.rules.production_system
}
pub fn advanced_component_stock(w: &WorldState, nation: NationId) -> f64 {
    w.production.operations.advanced_components.get(&nation).copied().unwrap_or(0.0)
}
pub fn advanced_component_capacity(w: &WorldState, nation: NationId) -> f64 {
    industry::goods_capacity(w,nation)
}

fn clean(v: f64) -> f64 { if v.is_finite() { v.max(0.0) } else { 0.0 } }
fn ratio(have: f64, need: f64) -> f64 {
    if need <= EPS { 1.0 } else { (clean(have) / need).clamp(0.0, 1.0) }
}
fn quantize(v: f64) -> f64 { (clean(v) * 1e9).floor() / 1e9 }
fn site_blocked(w:&WorldState,district:&str)->bool {
    resources::district_contested(w,district) || (w.rules.military_operations
        && w.districts.get(district).is_some_and(|n|crate::control::blocker(w,*n,district).is_some()))
}
fn levels(w: &WorldState, d: &str, k: K) -> f64 {
    if matches!(k, K::CivilianIndustry | K::StarterIndustry | K::Generation | K::PowerGrid) {
        modules::effective_capacity(w, d, k)
    } else { production::level(w, d, k) as f64 }
}

/// Converts the existing sourced/proxied manufacturing account, not GDP from
/// new buildings. The coefficient is a construction game scale, not a count
/// of historically measured civilian factories.
pub fn inherited_construction_capacity(w: &WorldState, nation: NationId) -> f64 {
    if !enabled(w) { return 0.0; }
    starting_industry::snapshot(w, nation).map_or(0.0, |s| {
        let available=w.starting_industry.as_ref().map_or(0.0,|state|
            state.provinces.iter().filter(|(d,_)|w.districts.get(*d)==Some(&nation)&&!site_blocked(w,d))
                .map(|(_,a)|a.factory_equivalents.iter().sum::<f64>()).sum::<f64>()
                +state.unallocated.get(&nation).map_or(0.0,|a|a.factory_equivalents.iter().sum()));
        available * s.utilization.clamp(0.0, 1.0)
            * INHERITED_CONSTRUCTION_PER_EQUIVALENT
    })
}

fn opening_active_equivalents(w: &WorldState, district: &str) -> f64 {
    w.starting_industry.as_ref().map_or(0.0, |s| {
        s.provinces.get(district).map_or(0.0, |a|
            a.factory_equivalents.iter().sum::<f64>() * s.starting_utilization)
    })
}

pub fn inherited_grid_headroom(w: &WorldState, district: &str) -> f64 {
    if !enabled(w) || site_blocked(w, district) { return 0.0; }
    opening_active_equivalents(w, district) * INHERITED_POWER_PER_EQUIVALENT
        * INHERITED_SPARE_POWER_SHARE
}
pub fn inherited_power_headroom(w: &WorldState, nation: NationId) -> f64 {
    if !enabled(w) { return 0.0; }
    w.districts.iter().filter(|(_, n)| **n == nation)
        .map(|(d, _)| inherited_grid_headroom(w, d)).sum()
}
pub fn grid_capacity(w: &WorldState, district: &str) -> f64 {
    if site_blocked(w, district) { return 0.0; }
    modules::effective_capacity(w, district, K::PowerGrid) * 5.0
        + inherited_grid_headroom(w, district)
}

pub fn jobs_per_level(kind: K) -> f64 {
    match kind {
        K::CivilianIndustry => 5_000.0, K::OfficeDistrict => 10_000.0,
        K::ArmsPlant => 6_000.0, K::Shipyard => 8_000.0,
        K::AdvancedIndustry => 5_000.0, K::ProcessingPlant => 4_000.0,
        K::StarterIndustry => 4_000.0, K::MachineryWorks => 4_000.0,
        K::ResearchCenter => 2_000.0, _ => 0.0,
    }
}
fn skilled(kind: K) -> bool { matches!(kind, K::OfficeDistrict | K::AdvancedIndustry | K::ResearchCenter) }

/// The macro unemployment result supplies available new hires. Existing
/// employment remains reserved for inherited output; jobs are not a second
/// growth multiplier and no extra GDP/unemployment write is made here.
pub fn available_workers(w: &WorldState, nation: NationId) -> f64 {
    w.nation_opt(nation).map_or(0.0, |n|
        clean(n.population) * 1_000_000.0 * 0.50
            * economy::unemployment_rate(n, w.at_war(nation)))
}
fn workforce_demand(w: &WorldState, nation: NationId, skilled_only: bool) -> f64 {
    w.districts.iter().filter(|(d, n)| **n == nation && !resources::district_contested(w, d))
        .map(|(d, _)| production::PROJECT_KINDS.iter()
            .filter(|k| !skilled_only || skilled(**k))
            .map(|k| levels(w, d, *k) * jobs_per_level(*k)).sum::<f64>()).sum()
}
pub fn worker_fraction(w: &WorldState, nation: NationId, kind: K) -> f64 {
    if !enabled(w) || jobs_per_level(kind) == 0.0 { return 1.0; }
    let available = available_workers(w, nation);
    let general = ratio(available, workforce_demand(w, nation, false));
    if !skilled(kind) { return general; }
    // Explicit broad skills proxy, bounded rather than asserting a census.
    let qualified_share = w.nation_opt(nation).map_or(0.25, |n|
        (0.25 + 0.75 * (n.gdp * 1_000.0 / n.population.max(0.1) / 24_000.0).clamp(0.0, 1.0)).clamp(0.25, 1.0));
    general.min(ratio(available * qualified_share, workforce_demand(w, nation, true)))
}

pub fn power_per_level(w: &WorldState, d: &str, kind: K) -> f64 {
    let efficient = (1.0 - production::level(w, d, K::Efficiency) as f64 * 0.1).max(0.5);
    match kind {
        K::CivilianIndustry => 0.20 * efficient,
        K::OfficeDistrict => 0.25 * efficient,
        K::Shipyard => 0.50 * efficient,
        K::AdvancedIndustry => 0.80 * efficient * w.districts.get(d)
            .map_or(1.0, |n| industry::manufacturing_company(w, *n, d).work_rate),
        K::ArmsPlant => 0.50 * efficient,
        K::ResearchCenter => 0.10 * efficient,
        K::ProcessingPlant | K::StarterIndustry | K::MachineryWorks => {
            let count = levels(w, d, kind);
            if count > 0.0 { industry::plant_rate(w, d, kind) * industry::power_per_pack(w, d, kind) / count }
            else if kind == K::MachineryWorks { 1.0 * efficient } else { efficient }
        },
        _ => 0.0,
    }
}
pub fn local_power_required(w: &WorldState, d: &str) -> f64 {
    if resources::district_contested(w, d) { return 0.0; }
    production::PROJECT_KINDS.iter().map(|k| levels(w, d, *k) * power_per_level(w, d, *k)).sum()
}
pub fn power_required(w: &WorldState, nation: NationId) -> f64 {
    w.districts.iter().filter(|(_, n)| **n == nation).map(|(d, _)| local_power_required(w, d)).sum()
}
pub fn power_fraction(w: &WorldState, district: &str) -> f64 {
    if !enabled(w) { return 1.0; }
    let Some(&nation) = w.districts.get(district) else { return 0.0; };
    if resources::district_contested(w, district) { return 0.0; }
    ratio(industry::power_capacity(w, nation), power_required(w, nation))
        .min(ratio(grid_capacity(w, district), local_power_required(w, district)))
}
/// Physical staffing/grid bound. Existing producers retain their own actual
/// inventory, budget and storage settlement; callers apply this bound once.
pub fn operating_fraction(w: &WorldState, district: &str, kind: K) -> f64 {
    if !enabled(w) { return 1.0; }
    if support(kind) && w.production.operations.support_day == Some(clock::absolute_day(w)) {
        return w.production.operations.receipts.iter().find(|s| s.district == district && s.kind == kind.key())
            .map_or(0.0, |s| s.utilization);
    }
    let Some(&nation) = w.districts.get(district) else { return 0.0; };
    if resources::district_contested(w, district) || !w.nation(nation).alive { return 0.0; }
    worker_fraction(w, nation, kind).min(if power_per_level(w, district, kind) > 0.0 {
        power_fraction(w, district)
    } else { 1.0 })
}

pub fn raw_recipe(kind: K, level_days: f64, power: f64) -> [f64; 12] {
    let mut raw = [0.0; 12];
    raw[C::Coal.idx()] = power * 0.02;
    if kind == K::AdvancedIndustry {
        raw[C::Copper.idx()] = level_days * 0.02;
        raw[C::RareEarths.idx()] = level_days * 0.002;
    }
    raw.map(quantize)
}
pub fn intermediates_per_level(kind: K) -> f64 {
    match kind { K::OfficeDistrict => OFFICE_INTERMEDIATES_DAY,
        K::AdvancedIndustry => ADVANCED_INTERMEDIATES_DAY, _ => 0.0 }
}
fn production_company(w: &WorldState, nation: NationId, district: &str, kind: K) -> crate::companies::CompanyModifiers {
    if kind == K::AdvancedIndustry { industry::manufacturing_company(w, nation, district) }
    else { crate::companies::CompanyModifiers::default() }
}
pub(crate) fn company_raw_recipe(w: &WorldState, nation: NationId, district: &str, kind: K, work: f64, power: f64) -> [f64; 12] {
    let company = production_company(w, nation, district, kind);
    let (fuel_rate, _) = industry::energy_company_rates(w, nation);
    raw_recipe(kind, work * company.work_rate * company.input_rate, power * fuel_rate)
}
pub(crate) fn intermediate_requirement(w: &WorldState, nation: NationId, district: &str, kind: K, work: f64) -> f64 {
    let company = production_company(w, nation, district, kind);
    work * intermediates_per_level(kind) * company.work_rate * company.input_rate
}
/// Rated output at current contractor terms. Deliberately independent of
/// available inputs: a shortage must remain visible in forward demand.
pub(crate) fn advanced_output_daily(w: &WorldState, district: &str, work: f64) -> f64 {
    let rate=w.districts.get(district).map_or(1.0,|n|
        production_company(w,*n,district,K::AdvancedIndustry).work_rate);
    work * rate * ADVANCED_OUTPUT_DAY
}
pub(crate) fn operating_cash_required(w: &WorldState, nation: NationId, district: &str, kind: K, work: f64) -> f64 {
    let company=production_company(w,nation,district,kind);
    work * company.work_rate * OPERATING_CASH_LEVEL_DAY_BN * (1.0 + company.fee_rate)
}
fn support(kind: K) -> bool { matches!(kind, K::CivilianIndustry | K::Shipyard | K::ArmsPlant | K::ResearchCenter) }
fn own_producer(kind: K) -> bool { matches!(kind, K::OfficeDistrict | K::AdvancedIndustry) || support(kind) }
fn demand_fraction(w: &WorldState, nation: NationId, kind: K) -> f64 {
    if kind != K::OfficeDistrict { return 1.0; }
    let office: f64 = w.districts.iter().filter(|(_, n)| **n == nation)
        .map(|(d, _)| levels(w, d, kind)).sum();
    let underlying = w.nation(nation).gdp - crate::province_economy::project_level(w, nation).unwrap_or(0.0);
    // New service absorption is capped at 1% of the existing economy. The
    // office's own GDP never manufactures demand for the next office.
    ratio(clean(underlying) * 0.01, office * OFFICE_GROSS_ANNUAL_BN)
}
pub fn annual_tax_estimate(w: &WorldState, nation: NationId, annual_gdp_bn: f64) -> f64 {
    let Some(n) = w.nation_opt(nation) else { return 0.0; };
    let mut conditions = economy::Conditions::of(w, nation);
    // Exclude oil rents: these projects add a non-oil tax base, not barrels.
    conditions.export_share = 0.0;
    let mut terms = economy::growth_terms(n, n.state_invest_gdp, n.interest_rate, &conditions);
    terms.budget_oil_revenue = 0.0;
    economy::Fiscal::of(n, &terms).in_billions(clean(annual_gdp_bn)).0
}
pub fn office_annual_value_added(w: &WorldState, district: &str, operating_levels: f64) -> f64 {
    let input = OFFICE_INTERMEDIATES_DAY * gdp_projects::INTERMEDIATE_PACK_BN
        + power_per_level(w, district, K::OfficeDistrict) * gdp_projects::POWER_UNIT_BN;
    clean(operating_levels * (OFFICE_GROSS_ANNUAL_BN - input * gdp_projects::DAYS_PER_ACCOUNTING_YEAR))
}

pub fn site(w: &WorldState, district: &str, kind: K) -> FacilityOperation {
    let count = levels(w, district, kind);
    let owner = w.districts.get(district).copied();
    let workers = owner.map_or(0.0, |n| worker_fraction(w, n, kind));
    let power = count * power_per_level(w, district, kind);
    let power_fraction = if power > 0.0 { power_fraction(w, district) } else { 1.0 };
    let mut funding_fraction: f64 = 1.0;
    let mut input_fraction: f64 = 1.0;
    let mut constraint = "Ready for assigned work; operating inputs and funding are checked when used.".to_string();
    if own_producer(kind) && count > 0.0 {
        if let Some(n) = owner {
            let (_, energy_fee) = industry::energy_company_rates(w,n);
            funding_fraction = ratio(programs::available_bn(w, n, BUDGET_INDUSTRY, 0), operating_cash_required(w,n,district,kind,count))
                .min(ratio(programs::available_bn(w, n, BUDGET_INDUSTRY, 1), power * ENERGY_CASH_POWER_DAY_BN * (1.0 + energy_fee)));
            let raw = company_raw_recipe(w,n,district,kind,count,power);
            for c in resources::ALL {
                input_fraction = input_fraction.min(ratio(resources::stockpile(w, n, c), raw[c.idx()]));
            }
            let goods = w.production.industry.goods.get(&n).cloned().unwrap_or_default();
            input_fraction = input_fraction.min(ratio(goods.intermediates, intermediate_requirement(w,n,district,kind,count)));
            if kind == K::AdvancedIndustry {
                input_fraction = input_fraction.min(ratio((advanced_component_capacity(w,n) - advanced_component_stock(w,n)).max(0.0), advanced_output_daily(w,district,count)));
            }
        } else { input_fraction = 0.0; funding_fraction = 0.0; }
    } else if matches!(kind, K::ProcessingPlant | K::StarterIndustry | K::MachineryWorks) && count > 0.0 {
        if let Some(n) = owner {
            let rate = industry::plant_rate(w,district,kind);
            let company = industry::manufacturing_company(w,n,district);
            let (_,energy_fee) = industry::energy_company_rates(w,n);
            let raw = industry::company_operating_recipe(w,n,district,kind,rate,power);
            for c in resources::ALL { input_fraction=input_fraction.min(ratio(resources::stockpile(w,n,c),raw[c.idx()])); }
            let goods = w.production.industry.goods.get(&n).cloned().unwrap_or_default();
            if kind==K::MachineryWorks { input_fraction=input_fraction.min(ratio(goods.intermediates,rate*company.input_rate)); }
            let stored=if kind==K::MachineryWorks {goods.capital_goods}else{goods.intermediates};
            input_fraction=input_fraction.min(ratio((industry::goods_capacity(w,n)-stored).max(0.0),rate));
            funding_fraction=ratio(programs::available_bn(w,n,BUDGET_INDUSTRY,if kind==K::MachineryWorks {0}else{2}),rate*0.00001*(1.0+company.fee_rate))
                .min(ratio(programs::available_bn(w,n,BUDGET_INDUSTRY,1),power*ENERGY_CASH_POWER_DAY_BN*(1.0+energy_fee)));
        } else { input_fraction=0.0; funding_fraction=0.0; }
    }
    let demand = owner.map_or(0.0, |n| demand_fraction(w, n, kind));
    let active = owner.is_some_and(|n| w.nation(n).alive) && !site_blocked(w, district);
    let fraction = if active { workers.min(power_fraction).min(input_fraction).min(funding_fraction).min(demand) } else { 0.0 };
    if !active { constraint = "Province control or an active government is required.".into(); }
    else if fraction + EPS < 1.0 {
        constraint = if workers <= fraction + EPS { "Not enough available workers or qualified workers; expand at a pace the labor market can staff." }
            else if power_fraction <= fraction + EPS { "Electricity limits this site; add national generation or local grid capacity." }
            else if input_fraction <= fraction + EPS { "Operating inputs or storage limit output; replenish raw stocks and intermediate packs, or use finished inventory." }
            else if funding_fraction <= fraction + EPS { "Industry or electricity operating funds limit this site; fund its department." }
            else { "Service demand limits this office; the existing economy cannot absorb its full output yet." }.into();
    }
    FacilityOperation { nation:owner, recorded_day:None, district: district.into(), kind: kind.key().into(), name: production::catalog(kind).name.into(), inherited: false,
        installed_capacity: count, operating_capacity: count * fraction, utilization: fraction,
        actual_operating_capacity:None, actual_utilization:None,
        output_daily: 0.0, output_unit: match kind { K::OfficeDistrict => "service value added ($bn/day)", K::AdvancedIndustry => "advanced components/day", K::MachineryWorks => "capital-goods packs/day", K::ProcessingPlant | K::StarterIndustry => "intermediate packs/day", K::CivilianIndustry => "staffed civilian factory levels", K::Shipyard => "staffed shipyard levels", _ => "operating levels" }.into(),
        jobs_required: count * jobs_per_level(kind), jobs_filled: count * jobs_per_level(kind) * fraction,
        power_required_daily: power, power_used_daily: 0.0, worker_fraction: workers, power_fraction,
        input_fraction, funding_fraction, status: if fraction <= EPS { "blocked" } else if fraction + EPS < 1.0 { "limited" } else { "awaiting_operation" }.into(),
        reason: constraint, annual_gdp_bn: 0.0, annual_tax_bn: 0.0, cash_spent_daily_bn: 0.0 }
}

pub fn civilian_operating_fraction(w: &WorldState, district: &str) -> f64 {
    settled_fraction(w, district, K::CivilianIndustry)
}
pub fn shipyard_operating_fraction(w: &WorldState, district: &str) -> f64 {
    settled_fraction(w, district, K::Shipyard)
}
fn settled_fraction(w: &WorldState, district: &str, kind: K) -> f64 {
    if !enabled(w) { return 1.0; }
    if w.production.operations.support_day == Some(clock::absolute_day(w)) {
        return w.production.operations.receipts.iter().find(|s| s.district == district && s.kind == kind.key())
            .map_or(0.0, |s| s.utilization);
    }
    site(w, district, kind).utilization
}
pub fn naval_capacity_bonus(w: &WorldState, nation: NationId) -> f64 {
    if !enabled(w) { return 0.0; }
    w.districts.iter().filter(|(_, n)| **n == nation)
        .map(|(d, _)| levels(w, d, K::Shipyard) * shipyard_operating_fraction(w, d)).sum()
}
pub fn support_power_used(w: &WorldState, nation: NationId) -> f64 {
    if !enabled(w) || w.production.operations.support_day != Some(clock::absolute_day(w)) { return 0.0; }
    w.production.operations.receipts.iter().filter(|s| w.districts.get(&s.district) == Some(&nation))
        .map(|s| s.power_used_daily).sum()
}
pub fn support_grid_used(w: &WorldState, district: &str) -> f64 {
    if !enabled(w) || w.production.operations.support_day != Some(clock::absolute_day(w)) { return 0.0; }
    w.production.operations.receipts.iter().filter(|s| s.district == district).map(|s| s.power_used_daily).sum()
}

pub fn demand_daily(w: &WorldState, nation: NationId) -> [f64; 12] {
    let mut total = [0.0; 12];
    if !enabled(w) || !programs::enrolled(w, nation) { return total; }
    for (d, owner) in &w.districts {
        if *owner != nation || resources::district_contested(w, d) { continue; }
        for k in [K::CivilianIndustry, K::Shipyard, K::ArmsPlant, K::ResearchCenter, K::OfficeDistrict, K::AdvancedIndustry] {
            let count = levels(w, d, k);
            let raw = company_raw_recipe(w,nation,d,k,count,count * power_per_level(w,d,k));
            for i in 0..12 { total[i] += raw[i]; }
        }
    }
    total.map(quantize)
}
pub fn intermediate_demand_daily(w: &WorldState, nation: NationId) -> f64 {
    if !enabled(w) { return 0.0; }
    w.districts.iter().filter(|(_, n)| **n == nation).map(|(d, _)|
        intermediate_requirement(w,nation,d,K::OfficeDistrict,levels(w,d,K::OfficeDistrict))
            + intermediate_requirement(w,nation,d,K::AdvancedIndustry,levels(w,d,K::AdvancedIndustry))).sum()
}

fn remaining_power(w: &WorldState, nation: NationId, d: &str) -> (f64, f64) {
    let day = clock::absolute_day(w);
    let mut national = industry::power_capacity(w, nation) - support_power_used(w, nation);
    let mut local = grid_capacity(w, d) - support_grid_used(w, d);
    if w.production.industry.last_day == Some(day) {
        for s in &w.production.industry.operations {
            if w.districts.get(&s.district) == Some(&nation) { national -= s.power_used_daily; }
            if s.district == d { local -= s.power_used_daily; }
        }
    }
    if let Some(m) = &w.materials {
        for s in m.orders.iter().filter(|s| s.last_day == Some(day)) {
            if s.nation == nation { national -= s.power_today; }
            if s.district == d { local -= s.power_today; }
        }
    }
    (national.max(0.0), local.max(0.0))
}

fn operate(w: &mut WorldState, d: &str, k: K) {
    let Some(&nation) = w.districts.get(d) else { return; };
    let mut receipt = site(w, d, k);
    if receipt.installed_capacity <= 0.0 { return; }
    let power_per = power_per_level(w, d, k);
    let (national, local) = remaining_power(w, nation, d);
    let mut work = receipt.operating_capacity.min(if power_per > 0.0 { national.min(local) / power_per } else { f64::MAX });
    work = quantize(work);
    let today = clock::absolute_day(w);
    receipt.recorded_day=Some(today);
    let open = w.nation(nation).program_budget.as_ref().is_some_and(|b| b.day == Some(today) && b.settled_day != Some(today));
    if !open || !w.rules.resource_market { work = 0.0; receipt.reason = "An open operating budget and physical resource market are required.".into(); }
    let power = work * power_per;
    let company = production_company(w,nation,d,k);
    let (_,energy_fee) = industry::energy_company_rates(w,nation);
    let raw = company_raw_recipe(w,nation,d,k,work,power);
    let intermediate_draw = intermediate_requirement(w,nation,d,k,work);
    if work > EPS && resources::consume_stockpile_atomic(w, nation, &raw).is_ok() {
        let base_cash = work * company.work_rate * OPERATING_CASH_LEVEL_DAY_BN;
        let cash = operating_cash_required(w,nation,d,k,work);
        let energy_cash = power * ENERGY_CASH_POWER_DAY_BN * (1.0 + energy_fee);
        programs::spend_operating(w, nation, BUDGET_INDUSTRY, 0, cash).expect("preflighted factory service cash");
        programs::spend_operating(w, nation, BUDGET_INDUSTRY, 1, energy_cash).expect("preflighted generating service cash");
        let goods = w.production.industry.goods.entry(nation).or_default();
        goods.intermediates = ((goods.intermediates - intermediate_draw).max(0.0) * 1e9).round() / 1e9;
        if k == K::AdvancedIndustry {
            let output = quantize(advanced_output_daily(w,d,work));
            let components=w.production.operations.advanced_components.entry(nation).or_default();
            *components=((*components+output)*1e9).round()/1e9;
            receipt.output_daily = output;
            gdp_projects::record_advanced(w, nation, d, work * company.work_rate * company.input_rate, output, power, raw, cash, energy_cash);
            industry::record_manufacturing_work(w,nation,d,output,base_cash);
        } else if k == K::OfficeDistrict {
            receipt.output_daily = office_annual_value_added(w, d, work) / gdp_projects::DAYS_PER_ACCOUNTING_YEAR;
            receipt.annual_gdp_bn = receipt.output_daily * gdp_projects::DAYS_PER_ACCOUNTING_YEAR;
            receipt.annual_tax_bn = annual_tax_estimate(w, nation, receipt.annual_gdp_bn);
            gdp_projects::record_office(w, nation, d, work, power, raw, cash, energy_cash);
        } else {
            receipt.output_daily = work;
            gdp_projects::record_power_dispatch(w,nation,power,raw[C::Coal.idx()],energy_cash);
        }
        industry::record_energy_work(w,nation,power,power * ENERGY_CASH_POWER_DAY_BN);
        receipt.operating_capacity = work;
        receipt.utilization = ratio(work, receipt.installed_capacity);
        receipt.jobs_filled = work * jobs_per_level(k);
        receipt.power_used_daily = power;
        receipt.cash_spent_daily_bn = cash + energy_cash;
        receipt.status = if work + EPS < receipt.installed_capacity { "limited" } else { "running" }.into();
        if receipt.status == "running" { receipt.reason = "Workers, electricity, operating inputs and funding supplied the full operating bundle.".into(); }
    } else {
        receipt.operating_capacity = 0.0; receipt.utilization = 0.0; receipt.jobs_filled = 0.0;
        receipt.status = "blocked".into();
        if receipt.reason.starts_with("Ready") { receipt.reason = "Earlier activity used today's shared power or inputs; no incomplete bundle was consumed.".into(); }
    }
    receipt.actual_operating_capacity=Some(receipt.operating_capacity);
    receipt.actual_utilization=Some(receipt.utilization);
    w.production.operations.receipts.push(receipt);
}

/// Settle the support services before the construction allocation reads them.
pub fn begin_day(w: &mut WorldState) {
    if !enabled(w) { return; }
    let day = clock::absolute_day(w);
    if w.production.operations.support_day == Some(day) { return; }
    w.production.operations.support_day = Some(day);
    w.production.operations.receipts.clear();
    let districts: Vec<_> = w.districts.keys().cloned().collect();
    for k in [K::CivilianIndustry, K::Shipyard, K::ArmsPlant, K::ResearchCenter] {
        for d in &districts { if levels(w, d, k) > 0.0 { operate(w, d, k); } }
    }
}
/// New industry runs after existing processors and inherited Materials orders;
/// those systems keep ownership of their receipts and are never run twice.
pub fn tick_day(w: &mut WorldState) {
    if !enabled(w) { return; }
    begin_day(w);
    let day = clock::absolute_day(w);
    if w.production.operations.last_day == Some(day) { return; }
    w.production.operations.last_day = Some(day);
    let districts: Vec<_> = w.production.rebuild_sites.keys().cloned().collect();
    for k in [K::AdvancedIndustry, K::OfficeDistrict] {
        for d in &districts { if levels(w, d, k) > 0.0 { operate(w, d, k); } }
    }
}

pub fn snapshot(w: &WorldState, nation: NationId) -> OperationsSnapshot {
    let inherited = starting_industry::snapshot(w, nation);
    let mut facilities = vec![];
    if let Some(state) = &w.starting_industry {
        for (d, asset) in &state.provinces {
            if w.districts.get(d) != Some(&nation) { continue; }
            let Some(p) = starting_industry::province(w, d) else { continue; };
            for g in p.groups {
                if g.factory_equivalents <= 0.0 { continue; }
                let utilization = g.utilization.clamp(0.0, 1.0);
                let jobs = g.factory_equivalents * 1_000.0;
                let power = g.factory_equivalents * INHERITED_POWER_PER_EQUIVALENT;
                facilities.push(FacilityOperation { nation:Some(nation), recorded_day:Some(clock::absolute_day(w)), district: d.clone(), kind: format!("inherited_{}", g.key), name: g.name.into(), inherited: true,
                    installed_capacity: g.factory_equivalents, operating_capacity: g.factory_equivalents * utilization,
                    utilization, actual_operating_capacity:Some(g.factory_equivalents*utilization), actual_utilization:Some(utilization), output_daily: g.current_output_annual_bn / 365.0, output_unit: "inherited value added ($bn/day)".into(),
                    jobs_required: jobs, jobs_filled: jobs * utilization, power_required_daily: power,
                    power_used_daily: power * utilization, worker_fraction: utilization, power_fraction: 1.0,
                    input_fraction: 1.0, funding_fraction: 1.0, status: "inherited_operation".into(),
                    reason: format!("Estimated {} capacity from the inherited manufacturing account. Its existing workers, utility service and supply are reconciled to opening output; {:.4} $bn/year is already in GDP. Unused capacity is not free government inventory.", g.name, g.current_output_annual_bn),
                    annual_gdp_bn: g.current_output_annual_bn, annual_tax_bn: annual_tax_estimate(w, nation, g.current_output_annual_bn), cash_spent_daily_bn: 0.0 });
            }
            let _ = asset;
        }
    }
    for (d, owner) in &w.districts {
        if *owner != nation { continue; }
        for k in production::PROJECT_KINDS {
            if levels(w, d, k) <= 0.0 { continue; }
            let mut row = site(w, d, k);
            let today=clock::absolute_day(w);
            let controlled=!site_blocked(w,d);
            row.jobs_filled=0.0;
            if let Some(last) = w.production.operations.receipts.iter().find(|r| r.district == *d && r.kind == k.key()
                && r.nation==Some(nation) && controlled && r.recorded_day.is_some_and(|day|day>=today-1&&day<=today)) {
                row.output_daily = last.output_daily; row.power_used_daily = last.power_used_daily;
                row.jobs_filled = last.jobs_filled; row.cash_spent_daily_bn = last.cash_spent_daily_bn;
                row.annual_gdp_bn = last.annual_gdp_bn; row.annual_tax_bn = last.annual_tax_bn;
                row.status = last.status.clone(); row.reason = last.reason.clone();
                row.recorded_day=last.recorded_day; row.actual_utilization=last.actual_utilization;
                row.actual_operating_capacity=last.actual_operating_capacity;
            } else if let Some(last) = w.production.industry.operations.iter().find(|r| r.district == *d && r.kind == k && controlled
                && w.production.industry.last_day.is_some_and(|day|day>=today-1&&day<=today)) {
                row.output_daily = last.output_daily; row.power_used_daily = last.power_used_daily;
                row.cash_spent_daily_bn = last.cash_spent_daily_bn;
                row.status = last.status.clone(); row.reason = last.reason.clone().unwrap_or_else(|| "Full operating bundle supplied.".into());
                let rated = industry::plant_rate(w, d, k);
                row.jobs_filled = row.jobs_required * ratio(last.output_daily, rated);
                row.recorded_day=w.production.industry.last_day;
                row.actual_utilization=Some(ratio(last.output_daily,rated));
                row.actual_operating_capacity=Some(row.installed_capacity*ratio(last.output_daily,rated));
            }
            if row.recorded_day.is_some() {
                if let Some(gdp)=w.province_economy.as_ref().and_then(|p|p.flows.receipts.get(&format!("site:{d}:{}",k.key()))) {
                    row.annual_gdp_bn=crate::gdp_projects::incremental_gdp_bn(gdp);
                    row.annual_tax_bn=annual_tax_estimate(w,nation,row.annual_gdp_bn);
                }
            }
            facilities.push(row);
        }
    }
    let jobs_required = facilities.iter().filter(|r| !r.inherited).map(|r| r.jobs_required).sum();
    let jobs_filled = facilities.iter().filter(|r| !r.inherited).map(|r| r.jobs_filled).sum();
    let inherited_jobs_required=facilities.iter().filter(|r|r.inherited).map(|r|r.jobs_required).sum();
    let inherited_jobs_filled=facilities.iter().filter(|r|r.inherited).map(|r|r.jobs_filled).sum();
    let inherited_power_required_daily=facilities.iter().filter(|r|r.inherited).map(|r|r.power_required_daily).sum();
    let inherited_power_used_daily=facilities.iter().filter(|r|r.inherited).map(|r|r.power_used_daily).sum();
    OperationsSnapshot { nation, as_of_day: w.production.operations.last_day.or(w.production.industry.last_day),
        inherited_factory_equivalents: inherited.as_ref().map_or(0.0, |s| s.factory_equivalents),
        inherited_output_annual_bn: inherited.as_ref().map_or(0.0, |s| s.current_output_annual_bn),
        inherited_jobs_required,inherited_jobs_filled,inherited_power_required_daily,inherited_power_used_daily,
        power_capacity_daily: industry::power_capacity(w, nation), power_required_daily: power_required(w, nation),
        workers_available: available_workers(w, nation), jobs_required, jobs_filled, facilities,
        advanced_components_stock:advanced_component_stock(w,nation), advanced_components_capacity:advanced_component_capacity(w,nation),
        advanced_components_required_daily:crate::manufacturing::advanced_components_demand_daily(w,nation)+crate::equipment::advanced_components_demand_daily(w,nation),
        note: "Factory equivalents, staffing, qualification and spare power are explicit model estimates. Inherited value added is already inside GDP; only actual new production reaches the existing province ledger, then normal fiscal taxes. New jobs use the available hiring pool without adding a second growth multiplier. Power totals show supply available to additional activity after inherited demand; dated output is a receipt, operating capacity is today's conditional forecast.".into() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{init::world_1990, load, save, world::GameRules};
    const USA:NationId=NationId::USA;
    fn ready()->(WorldState,String) {
        ready_with_inherited(false)
    }
    fn ready_with_inherited(inherited:bool)->(WorldState,String) {
        let mut w=world_1990(GameRules{daily_simulation:true,production_system:true,resource_market:true,industry_rebuild:true,..Default::default()});
        if inherited { starting_industry::enable_new_world(&mut w).unwrap(); }
        w.player=Some(USA);
        crate::province_economy::enable(&mut w);
        let year=w.year;
        programs::install(&mut w,USA,year,programs::default_departments());
        programs::begin_day(&mut w);
        crate::province_economy::begin_day(&mut w);
        for c in resources::ALL {if c!=C::Oil {resources::set_stockpile_for_test(&mut w,USA,c,1_000.0);}}
        w.production.industry.goods.insert(USA,industry::Goods{intermediates:10.0,capital_goods:4.0});
        let d=w.districts.iter().find(|(_,n)|**n==USA).unwrap().0.clone();
        for k in [K::Generation,K::PowerGrid] {production::complete_capability(&mut w,&d,k);}
        (w,d)
    }
    fn near(a:f64,b:f64){assert!((a-b).abs()<1e-8,"{a} != {b}");}
    #[test]
    fn company_components_and_inherited_power_share_one_paid_physical_bundle() {
        use crate::companies::{self,CompanySector,CompanyTarget};
        let (mut w,d)=ready_with_inherited(true);
        production::complete_capability(&mut w,&d,K::AdvancedIndustry);
        companies::enable(&mut w);
        let mut targets=Vec::new();
        for sector in [CompanySector::Manufacturing,CompanySector::Energy] {
            let id=w.companies.roster.iter().filter(|c|c.nation==USA&&c.sector==sector)
                .max_by(|a,b|a.input_saving.total_cmp(&b.input_saving)).unwrap().id;
            let target=CompanyTarget::Facility{district:d.clone(),sector};
            companies::assign(&mut w,USA,id,target.clone()).unwrap();
            targets.push(target);
        }
        let factory=companies::modifiers(&w,USA,&targets[0]);
        let generator=companies::modifiers(&w,USA,&targets[1]);
        let inherited=inherited_power_headroom(&w,USA);
        let paid_capacity=10.0*generator.work_rate;
        let paid_share=paid_capacity/(paid_capacity+inherited);
        let (fuel_rate,energy_fee)=industry::energy_company_rates(&w,USA);
        near(fuel_rate,1.0-paid_share+paid_share*generator.input_rate);
        near(energy_fee,paid_share*generator.fee_rate);
        let power=0.8*factory.work_rate;
        let coal=resources::stockpile(&w,USA,C::Coal);
        let copper=resources::stockpile(&w,USA,C::Copper);
        let mut starved=w.clone();
        tick_day(&mut w);
        near(advanced_component_stock(&w,USA),ADVANCED_OUTPUT_DAY*factory.work_rate);
        near(coal-resources::stockpile(&w,USA,C::Coal),power*0.02*fuel_rate);
        near(copper-resources::stockpile(&w,USA,C::Copper),0.02*factory.work_rate*factory.input_rate);
        near(w.production.industry.goods[&USA].intermediates,10.0-ADVANCED_INTERMEDIATES_DAY*factory.work_rate*factory.input_rate);
        let operation=&w.production.operations.receipts[0];
        near(operation.jobs_filled,5_000.0);
        near(operation.cash_spent_daily_bn, factory.work_rate*OPERATING_CASH_LEVEL_DAY_BN*(1.0+factory.fee_rate)
            +power*ENERGY_CASH_POWER_DAY_BN*(1.0+energy_fee));
        let manufacturing=w.companies.assignments.iter().find(|a|a.target==targets[0]).unwrap();
        let energy=w.companies.assignments.iter().find(|a|a.target==targets[1]).unwrap();
        near(manufacturing.fees_today_bn,factory.work_rate*OPERATING_CASH_LEVEL_DAY_BN*factory.fee_rate);
        near(energy.fees_today_bn,power*ENERGY_CASH_POWER_DAY_BN*energy_fee);
        let rows=gdp_projects::contributions(&w);
        let generated=rows.iter().find(|r|r.kind==K::Generation.key()).unwrap();
        near(generated.output_quantity_daily,power*paid_share);
        near(generated.intermediate_inputs_daily_bn,power*paid_share*0.02*generator.input_rate*resources::unit_price_bn(C::Coal).unwrap());
        let once=save(&w);tick_day(&mut w);assert_eq!(save(&w),once);
        resources::set_stockpile_for_test(&mut starved,USA,C::Copper,0.0);
        let companies_before=starved.companies.clone();
        tick_day(&mut starved);
        assert_eq!(advanced_component_stock(&starved,USA),0.0);
        assert_eq!(starved.companies,companies_before,"a blocked bundle earns no company fees or experience");
        near(resources::stockpile(&starved,USA,C::Coal),coal);
    }
    #[test]
    fn legacy_world_does_not_gain_a_second_operating_system() {
        let mut w=world_1990(GameRules::default());
        let before=save(&w);
        begin_day(&mut w);tick_day(&mut w);
        assert_eq!(save(&w),before);
        assert!(w.production.operations.is_empty());
    }
    #[test]
    fn inherited_account_is_reconciled_without_stock_cash_or_gdp_awards() {
        let mut w=world_1990(GameRules{daily_simulation:true,production_system:true,industry_rebuild:true,..Default::default()});
        starting_industry::enable_new_world(&mut w).unwrap();
        crate::province_economy::enable(&mut w);
        let before=save(&w);
        let a=snapshot(&w,USA);
        assert!(a.inherited_factory_equivalents>0.0);
        assert!(inherited_construction_capacity(&w,USA)>0.0);
        assert!(a.facilities.iter().all(|s|s.inherited&&s.power_fraction==1.0&&s.input_fraction==1.0));
        near(a.facilities.iter().map(|s|s.annual_gdp_bn).sum(),a.inherited_output_annual_bn);
        assert_eq!(save(&w),before);
        assert_eq!(a,snapshot(&w,USA));
    }
    #[test]
    fn civilian_construction_capacity_requires_a_paid_complete_service_bundle() {
        let (mut w,d)=ready();
        production::complete_capability(&mut w,&d,K::CivilianIndustry);
        let coal=resources::stockpile(&w,USA,C::Coal);
        begin_day(&mut w);
        near(civilian_operating_fraction(&w,&d),1.0);
        near(coal-resources::stockpile(&w,USA,C::Coal),0.004);
        near(support_power_used(&w,USA),0.2);
        assert!(w.production.operations.receipts[0].cash_spent_daily_bn>0.0);
        let once=save(&w);begin_day(&mut w);assert_eq!(save(&w),once);
        let (mut starved,d)=ready();
        production::complete_capability(&mut starved,&d,K::CivilianIndustry);
        resources::set_stockpile_for_test(&mut starved,USA,C::Coal,0.0);
        begin_day(&mut starved);
        assert_eq!(civilian_operating_fraction(&starved,&d),0.0);
        assert_eq!(support_power_used(&starved,USA),0.0);
        assert_eq!(w.production.operations.receipts[0].jobs_filled,5_000.0);
    }
    #[test]
    fn office_and_components_consume_inputs_and_post_value_added_once() {
        let (mut w,d)=ready();
        for k in [K::OfficeDistrict,K::AdvancedIndustry] {production::complete_capability(&mut w,&d,k);}
        let gdp=w.nation(USA).gdp;
        let treasury=w.nation(USA).treasury_bn;
        tick_day(&mut w);
        near(w.production.industry.goods[&USA].intermediates,10.0-OFFICE_INTERMEDIATES_DAY-ADVANCED_INTERMEDIATES_DAY);
        near(w.production.industry.goods[&USA].capital_goods,4.0);
        near(advanced_component_stock(&w,USA),ADVANCED_OUTPUT_DAY);
        near(w.nation(USA).gdp,gdp);
        assert_eq!(w.nation(USA).treasury_bn,treasury,"the displayed tax is not deposited directly");
        let rows=crate::gdp_projects::contributions(&w);
        let office=rows.iter().find(|r|r.kind==K::OfficeDistrict.key()).unwrap();
        assert_eq!(office.sector,"services");
        near(office.annual_gdp_bn,office_annual_value_added(&w,&d,1.0));
        let components=rows.iter().find(|r|r.kind==K::AdvancedIndustry.key()).unwrap();
        assert_eq!(components.output_unit,"advanced components");
        assert!(components.intermediate_inputs_daily_bn>=ADVANCED_INTERMEDIATES_DAY*gdp_projects::INTERMEDIATE_PACK_BN);
        assert!(office.annual_gdp_bn>annual_tax_estimate(&w,USA,office.annual_gdp_bn));
        let once=save(&w);tick_day(&mut w);assert_eq!(save(&w),once);
        let mut resumed=load(&once).unwrap();tick_day(&mut resumed);assert_eq!(save(&resumed),once);
        crate::province_economy::finish_day(&mut w);
        assert!(w.nation(USA).gdp>gdp);
        let first=w.nation(USA).gdp;
        crate::province_economy::finish_day(&mut w);near(w.nation(USA).gdp,first);
    }
    #[test]
    fn missing_component_raw_input_never_partially_consumes_a_bundle() {
        let (mut w,d)=ready();production::complete_capability(&mut w,&d,K::AdvancedIndustry);
        resources::set_stockpile_for_test(&mut w,USA,C::Copper,0.0);
        let coal=resources::stockpile(&w,USA,C::Coal);
        let goods=w.production.industry.goods[&USA].clone();
        tick_day(&mut w);
        assert_eq!(advanced_component_stock(&w,USA),0.0);
        assert_eq!(w.production.industry.goods[&USA],goods);
        near(resources::stockpile(&w,USA,C::Coal),coal);
        assert_eq!(w.production.operations.receipts[0].cash_spent_daily_bn,0.0);
    }
    #[test]
    fn all_receipts_share_power_and_workers_and_stale_output_is_not_current() {
        let (mut w,d)=ready();
        for k in [K::CivilianIndustry,K::ProcessingPlant,K::ArmsPlant,K::AdvancedIndustry,K::OfficeDistrict] {
            for _ in 0..5 {production::complete_capability(&mut w,&d,k);}
        }
        begin_day(&mut w);industry::tick_day(&mut w);tick_day(&mut w);
        let total=support_power_used(&w,USA)+w.production.industry.operations.iter().map(|s|s.power_used_daily).sum::<f64>();
        assert!(total<=industry::power_capacity(&w,USA)+EPS);
        assert!(total<=grid_capacity(&w,&d)+EPS);
        let a=snapshot(&w,USA);
        assert!(a.jobs_filled<=a.workers_available+EPS);
        assert!(a.facilities.iter().any(|r|r.output_daily>0.0));
        clock::advance_date(&mut w);clock::advance_date(&mut w);
        assert!(snapshot(&w,USA).facilities.iter().all(|r|r.output_daily==0.0));
    }
}
