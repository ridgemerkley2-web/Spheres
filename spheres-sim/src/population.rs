//! Arcade population economy. People are conserved, qualifications take time,
//! and a worker can fill only one job. Historical observations seed national
//! age/education totals where available; provincial distributions, job recipes,
//! class ownership and missing values are explicitly modeled game estimates.
//! This opt-in module owns demography. It never writes GDP, treasury or RNG.
use crate::world::{
    NationId, WorldState, BUDGET_EDUCATION, BUDGET_HEALTH, BUDGET_HOUSING, BUDGET_PENSIONS,
};
use crate::{clock, districts, exact, province_economy, starting_industry};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

const SKILLS: usize = 5;
const GRADES: usize = 3;
const EDUCATION_KEYS: [&str; 5] = [
    "below_basic",
    "basic",
    "secondary",
    "vocational",
    "tertiary",
];
const EDUCATION_NAMES: [&str; 5] = [
    "Learning foundations",
    "Basic education",
    "Secondary education",
    "Trades & technical",
    "University graduates",
];
const CLASS_KEYS: [&str; 6] = [
    "agricultural",
    "routine_workers",
    "skilled_workers",
    "professionals",
    "proprietors",
    "capital_owners",
];
const CLASS_NAMES: [&str; 6] = [
    "Agricultural households",
    "Wage workers",
    "Skilled workers",
    "Professionals & managers",
    "Small business owners",
    "Capital owners",
];
const CLASS_INCOME: [f64; 6] = [0.70, 0.85, 1.15, 1.55, 1.25, 2.80];
const JOB_RECIPE: [[f64; 3]; 8] = [
    [0.93, 0.05, 0.02],
    [0.65, 0.26, 0.09],
    [0.58, 0.34, 0.08],
    [0.42, 0.42, 0.16],
    [0.72, 0.24, 0.04],
    [0.70, 0.22, 0.08],
    [0.70, 0.14, 0.16],
    [0.52, 0.12, 0.36],
];
const LABOR_PRODUCTIVITY: [f64; 8] = [0.45, 2.2, 1.05, 1.8, 0.9, 1.1, 1.0, 1.0];
const WAGES: [f64; 8] = [0.78, 1.25, 1.05, 1.30, 1.0, 1.10, 1.0, 1.15];

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Policy {
    #[default]
    Balanced,
    TradeSchools,
    Universities,
    BackToWork,
}
impl Policy {
    pub const ALL: [Self; 4] = [
        Self::Balanced,
        Self::TradeSchools,
        Self::Universities,
        Self::BackToWork,
    ];
    pub fn key(self) -> &'static str {
        match self {
            Self::Balanced => "balanced",
            Self::TradeSchools => "trade_schools",
            Self::Universities => "universities",
            Self::BackToWork => "back_to_work",
        }
    }
    pub fn parse(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|p| p.key() == s)
    }
    pub fn name(self) -> &'static str {
        match self {
            Self::Balanced => "Balanced opportunity",
            Self::TradeSchools => "Trade schools",
            Self::Universities => "University drive",
            Self::BackToWork => "Back to work",
        }
    }
    pub fn description(self) -> &'static str {
        match self {
            Self::Balanced => "Keep schools, technical training and universities moving together.",
            Self::TradeSchools => {
                "Prioritize 18-month technical courses to fill skilled vacancies."
            }
            Self::Universities => "Prioritize four-year degrees for professionals and researchers.",
            Self::BackToWork => {
                "Help adults seek jobs with six-month foundations and two-year secondary catch-up courses."
            }
        }
    }
    fn course_weights(self) -> [f64; 4] {
        match self {
            Self::Balanced => [0.15, 0.20, 0.40, 0.25],
            Self::TradeSchools => [0.10, 0.15, 0.65, 0.10],
            Self::Universities => [0.10, 0.15, 0.15, 0.60],
            Self::BackToWork => [0.35, 0.45, 0.15, 0.05],
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PopulationOutcomes {
    pub labor_growth: f64,
    pub unemployment: f64,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct PopulationState {
    pub enabled: bool,
    pub provinces: BTreeMap<String, ProvincePopulation>,
    pub unallocated: BTreeMap<NationId, ProvincePopulation>,
    pub nations: BTreeMap<NationId, NationPopulation>,
    pub last_day: Option<i32>,
}
impl PopulationState {
    pub fn is_empty(&self) -> bool {
        !self.enabled
            && self.provinces.is_empty()
            && self.unallocated.is_empty()
            && self.nations.is_empty()
            && self.last_day.is_none()
    }
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Course {
    pub from: usize,
    pub to: usize,
    pub people_m: f64,
    pub started_day: i32,
    pub required_days: f64,
    pub funded_days: f64,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProvincePopulation {
    /// Birth-year cohorts: newborns cannot appear in the working-age pool.
    pub children: BTreeMap<i32, f64>,
    /// Mutually exclusive qualifications; vocational is a secondary route.
    pub working: [f64; SKILLS],
    pub retirees_m: f64,
    pub birth_rate_per_adult: f64,
    pub reference_population_m: f64,
    pub participation_reference: f64,
    pub participation: f64,
    pub school_coverage: f64,
    pub school_reference: f64,
    pub jobs_reference: [[f64; GRADES]; 8],
    pub jobs: [[f64; GRADES]; 8],
    pub project_jobs: [[f64; GRADES]; 8],
    pub filled: [[f64; GRADES]; 8],
    pub inherited_filled_m: f64,
    pub employed_by_skill: [f64; SKILLS],
    pub unemployment_by_skill: [f64; SKILLS],
    pub underemployed_m: f64,
    pub labor_force_m: f64,
    pub military_m: f64,
    pub courses: Vec<Course>,
    pub intake_month: i32,
    pub training_capacity_m: f64,
    pub annual_graduates_m: f64,
    pub class_shares: [f64; 6],
    pub wealth_indices: [f64; 6],
    pub class_living: [f64; 6],
    pub class_risk_reference: [f64; 6],
    pub living_standards: f64,
    pub social_mobility: f64,
    pub staffing_reference: [f64; 8],
    pub wage_indices: [f64; 8],
    pub baseline_education_share: f64,
    pub last_owner: NationId,
    pub opening_unemployment: f64,
}
impl ProvincePopulation {
    pub fn children_m(&self) -> f64 {
        self.children.values().sum()
    }
    pub fn working_age_m(&self) -> f64 {
        self.working.iter().sum()
    }
    pub fn population_m(&self) -> f64 {
        self.children_m() + self.working_age_m() + self.retirees_m
    }
    pub fn students_m(&self) -> f64 {
        self.courses.iter().map(|c| c.people_m).sum()
    }
    pub fn employed_m(&self) -> f64 {
        self.filled.iter().flatten().sum()
    }
    pub fn unemployed_m(&self) -> f64 {
        (self.labor_force_m - self.employed_m()).max(0.0)
    }
    pub fn vacancies_m(&self) -> f64 {
        self.jobs.iter().flatten().sum::<f64>() + self.project_jobs.iter().flatten().sum::<f64>()
            - self.employed_m()
    }
    fn raw_staffing(&self, s: usize) -> f64 {
        let total = self.jobs[s].iter().sum::<f64>() + self.project_jobs[s].iter().sum::<f64>();
        if total > 1e-12 {
            self.filled[s].iter().sum::<f64>() / total
        } else {
            1.0
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct NationPopulation {
    pub policy: Policy,
    pub last_policy_day: Option<i32>,
    pub opening_gdp_bn: f64,
    pub opening_population_m: f64,
    pub opening_education_share: f64,
    pub opening_military_strength: f64,
    pub research_reference: f64,
    pub labor_growth: f64,
    pub population_growth: f64,
    pub last_inherited_employed_m: f64,
    pub hardship_reference: f64,
    pub source_notes: Vec<String>,
    pub province_ids: Vec<String>,
}
#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct PolicyQuote {
    pub policy: Policy,
    pub name: &'static str,
    pub description: &'static str,
    pub political_cost: f64,
    pub cooldown_days: u32,
    pub available: bool,
    pub reason: Option<String>,
    pub selected: bool,
}
#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct EducationSnapshot {
    pub key: &'static str,
    pub name: &'static str,
    pub people_m: f64,
    pub share: f64,
}
#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct ClassSnapshot {
    pub key: &'static str,
    pub name: &'static str,
    pub people_m: f64,
    pub share: f64,
    pub income_index: f64,
    pub living_standards: f64,
    pub wealth_index: f64,
    pub discontent: f64,
}
#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct SectorSnapshot {
    pub key: &'static str,
    pub name: &'static str,
    pub jobs_m: f64,
    pub employed_m: f64,
    pub vacancies_m: f64,
    pub staffing: f64,
    pub wage_index: f64,
}
#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct ProvinceSnapshot {
    pub district: String,
    pub name: String,
    pub population_m: f64,
    pub employed_m: f64,
    pub unemployed_m: f64,
    pub vacancies_m: f64,
    pub unemployment_rate: f64,
    pub living_standards: f64,
    pub students_m: f64,
}
#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct CourseSnapshot {
    pub name: &'static str,
    pub people_m: f64,
    pub progress: f64,
    pub days_remaining: u32,
}
#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct NationSnapshot {
    pub population_m: f64,
    pub children_m: f64,
    pub working_age_m: f64,
    pub retirees_m: f64,
    pub labor_force_m: f64,
    pub employed_m: f64,
    pub unemployed_m: f64,
    pub students_m: f64,
    pub outside_workforce_m: f64,
    pub military_m: f64,
    pub unemployment_rate: f64,
    pub participation_rate: f64,
    pub employment_rate: f64,
    pub underemployment_rate: f64,
    pub vacancies_m: f64,
    pub skilled_vacancies_m: f64,
    pub living_standards: f64,
    pub social_mobility: f64,
    pub research_multiplier: f64,
    pub education_budget_bn: f64,
    pub school_coverage: f64,
    pub training_capacity_m: f64,
    pub annual_graduates_m: f64,
    pub policy: Policy,
    pub policies: Vec<PolicyQuote>,
    pub education: Vec<EducationSnapshot>,
    pub classes: Vec<ClassSnapshot>,
    pub sectors: Vec<SectorSnapshot>,
    pub provinces: Vec<ProvinceSnapshot>,
    pub notes: Vec<String>,
    pub courses: Vec<CourseSnapshot>,
}

fn ratio(a: f64, b: f64) -> f64 {
    if b > 1e-12 {
        (a / b).max(0.0)
    } else {
        0.0
    }
}
pub fn active(w: &WorldState) -> bool {
    w.population_system.enabled && clock::is_daily(w)
}
fn source_data() -> &'static serde_json::Value {
    static DATA: OnceLock<serde_json::Value> = OnceLock::new();
    DATA.get_or_init(|| {
        serde_json::from_str(include_str!("../data/population_1990.json"))
            .expect("checked population source data")
    })
}
fn observation(row: Option<&serde_json::Value>, key: &str) -> Option<f64> {
    row?.get(key)?
        .get("value")?
        .as_f64()
        .filter(|v| v.is_finite() && *v >= 0.0)
}
fn sum_rows<F: Fn(&ProvincePopulation) -> f64>(w: &WorldState, id: NationId, f: F) -> f64 {
    let mut total = 0.0;
    if let Some(n) = w.population_system.nations.get(&id) {
        for d in &n.province_ids {
            if w.districts.get(d) == Some(&id) {
                if let Some(p) = w.population_system.provinces.get(d) {
                    total += f(p);
                }
            }
        }
    }
    if let Some(p) = w.population_system.unallocated.get(&id) {
        total += f(p);
    }
    total
}
fn seed_province(w: &WorldState, id: NationId, d: Option<&str>, pop: f64) -> ProvincePopulation {
    let n = w.nation(id);
    let source = source_data()
        .get("countries")
        .and_then(|v| v.get(format!("{:?}", id)));
    let mut ages = [
        observation(source, "population_age_0_14").unwrap_or(0.30),
        observation(source, "population_age_15_64").unwrap_or(0.63),
        observation(source, "population_age_65_plus").unwrap_or(0.07),
    ];
    let sum = ages.iter().sum::<f64>();
    for a in &mut ages {
        *a /= sum.max(1e-12);
    }
    let tertiary = observation(source, "education_attainment_tertiary_25_plus")
        .unwrap_or_else(|| observation(source, "tertiary_enrollment").unwrap_or(0.12) * 0.45)
        .clamp(0.0, 1.0);
    let secondary = observation(source, "education_attainment_secondary_25_plus")
        .unwrap_or_else(|| observation(source, "secondary_enrollment").unwrap_or(0.45) * 0.55)
        .clamp(tertiary, 1.0);
    let literacy = observation(source, "literacy")
        .unwrap_or(0.80)
        .clamp(secondary, 1.0);
    let technical = (secondary - tertiary) * 0.28;
    let shares = [
        1.0 - literacy,
        literacy - secondary,
        secondary - tertiary - technical,
        technical,
        tertiary,
    ];
    let working = shares.map(|s| s * ages[1] * pop);
    let participation = observation(source, "labor_force_participation_15_64")
        .unwrap_or(0.72)
        .clamp(0.3, 0.95);
    let unemployment = observation(source, "unemployment")
        .unwrap_or(0.07)
        .clamp(0.0, 0.6);
    let school = observation(source, "secondary_enrollment")
        .unwrap_or(0.65)
        .clamp(0.20, 1.0);
    let budget = n.budget_for(w.year).allocations[BUDGET_EDUCATION];
    let military = ages[1] * pop * 0.012; // Fraction of working age, explicitly a game manpower estimate.
    let workforce = (ages[1] * pop - military) * participation;
    let jobs_total = workforce * (1.0 - unemployment);
    let sectors = starting_industry::sector_shares(w, id, d);
    let mut sector_jobs = [0.0; 8];
    let mass = (0..8)
        .map(|s| sectors[s] / LABOR_PRODUCTIVITY[s])
        .sum::<f64>();
    for s in 0..8 {
        sector_jobs[s] = jobs_total * sectors[s] / LABOR_PRODUCTIVITY[s] / mass.max(1e-12);
    }
    let tech_demand = (0..8)
        .map(|s| sector_jobs[s] * JOB_RECIPE[s][1])
        .sum::<f64>();
    let prof_demand = (0..8)
        .map(|s| sector_jobs[s] * JOB_RECIPE[s][2])
        .sum::<f64>();
    let tech_scale = ratio(working[3] * participation * 0.92, tech_demand).min(1.0);
    let prof_scale = ratio(working[4] * participation * 0.92, prof_demand).min(1.0);
    let mut jobs = [[0.0; 3]; 8];
    for s in 0..8 {
        jobs[s][1] = sector_jobs[s] * JOB_RECIPE[s][1] * tech_scale;
        jobs[s][2] = sector_jobs[s] * JOB_RECIPE[s][2] * prof_scale;
        jobs[s][0] = sector_jobs[s] - jobs[s][1] - jobs[s][2];
    }
    let children = (1..=15)
        .map(|age| (w.year - age, pop * ages[0] / 15.0))
        .collect();
    let deaths = pop * (ages[0] * 0.002 + ages[1] * 0.002 + ages[2] * 0.055);
    let births = (crate::economy::population_growth(n) * pop + deaths).max(0.0);
    ProvincePopulation {
        children,
        working,
        retirees_m: pop * ages[2],
        birth_rate_per_adult: ratio(births, pop * ages[1]),
        reference_population_m: pop,
        participation_reference: participation,
        participation,
        school_coverage: school,
        school_reference: school,
        jobs_reference: jobs,
        jobs,
        project_jobs: [[0.0; 3]; 8],
        filled: [[0.0; 3]; 8],
        inherited_filled_m: 0.0,
        employed_by_skill: [0.0; 5],
        unemployment_by_skill: [0.0; 5],
        underemployed_m: 0.0,
        labor_force_m: workforce,
        military_m: military,
        courses: vec![],
        intake_month: i32::MIN,
        training_capacity_m: 0.0,
        annual_graduates_m: 0.0,
        class_shares: [0.1, 0.45, 0.15, 0.15, 0.12, 0.03],
        wealth_indices: [35.0, 45.0, 70.0, 100.0, 135.0, 320.0],
        class_living: [100.0; 6],
        class_risk_reference: [0.0; 6],
        living_standards: 100.0,
        social_mobility: 0.0,
        staffing_reference: [1.0; 8],
        wage_indices: WAGES,
        baseline_education_share: budget.max(0.005),
        last_owner: id,
        opening_unemployment: unemployment,
    }
}
/// Explicit opt-in and idempotent migration; no legacy world is automatically seeded.
pub fn enable(w: &mut WorldState) {
    if w.population_system.enabled
        || (!clock::is_daily(w) && w.daily.activate_after_month.is_none())
    {
        return;
    }
    w.population_system.enabled = true;
    ensure(w);
    // Resolve opening matching, then freeze normalization. Enabling is not a tick.
    let mut state = std::mem::take(&mut w.population_system);
    let demand = project_demands(w);
    for (d, p) in &mut state.provinces {
        let id = *w.districts.get(d).unwrap();
        p.project_jobs = demand.get(d).copied().unwrap_or([[0.0; 3]; 8]);
        match_jobs(p);
        classes(p, 0.0, true);
        for s in 0..8 {
            p.staffing_reference[s] = p.raw_staffing(s).max(0.10);
        }
        p.last_owner = id;
    }
    for p in state.unallocated.values_mut() {
        match_jobs(p);
        classes(p, 0.0, true);
        for s in 0..8 {
            p.staffing_reference[s] = p.raw_staffing(s).max(0.10);
        }
    }
    w.population_system = state;
    let ids: Vec<_> = w.population_system.nations.keys().copied().collect();
    for id in ids {
        let employed = sum_rows(w, id, |p| p.inherited_filled_m);
        let research = researchers(w, id);
        let hard = hardship_level(w, id);
        let row = w.population_system.nations.get_mut(&id).unwrap();
        row.last_inherited_employed_m = employed;
        row.research_reference = research.max(1e-9);
        row.hardship_reference = hard;
    }
    publish_outcomes(w);
}
fn ensure(w: &mut WorldState) {
    let initial_enrollment = w.population_system.nations.is_empty();
    let alive: Vec<_> = w.nations.iter().filter(|n| n.alive).map(|n| n.id).collect();
    let mut owned_by: BTreeMap<NationId, Vec<String>> = BTreeMap::new();
    for (d, &owner) in &w.districts {
        owned_by.entry(owner).or_default().push(d.clone());
    }
    for id in alive {
        if !w.population_system.nations.contains_key(&id) {
            let n = w.nation(id);
            let source = source_data()
                .get("countries")
                .and_then(|v| v.get(format!("{:?}", id)));
            let sourced = [
                "population_age_0_14",
                "population_age_15_64",
                "population_age_65_plus",
                "labor_force_participation_15_64",
                "unemployment",
                "literacy",
                "education_attainment_secondary_25_plus",
                "education_attainment_tertiary_25_plus",
            ];
            let missing: Vec<_> = sourced
                .iter()
                .filter(|k| observation(source, k).is_none())
                .copied()
                .collect();
            let mut notes=vec!["Province age and education shares use national observations. Jobs, ownership, wages and class shares are game estimates, not a historical census.".into()];
            if !missing.is_empty() {
                notes.push(format!(
                    "Modeled starting values where observations are unavailable: {}.",
                    missing.join(", ")
                ));
            }
            notes.push("Courses have calendar delays; enrollment and qualification counts are separate. Gross enrollment is never treated as an observed attainment rate.".into());
            w.population_system.nations.insert(
                id,
                NationPopulation {
                    policy: Policy::Balanced,
                    last_policy_day: None,
                    opening_gdp_bn: n.gdp,
                    opening_population_m: n.population,
                    opening_education_share: n.budget_for(w.year).allocations[BUDGET_EDUCATION]
                        .max(0.005),
                    opening_military_strength: n.mil_strength.max(1e-6),
                    research_reference: 0.0,
                    labor_growth: 0.0,
                    population_growth: crate::economy::population_growth(n),
                    last_inherited_employed_m: 0.0,
                    hardship_reference: 0.0,
                    source_notes: notes,
                    province_ids: vec![],
                },
            );
        }
        let owned = owned_by.get(&id).cloned().unwrap_or_default();
        w.population_system
            .nations
            .get_mut(&id)
            .unwrap()
            .province_ids = owned.clone();
        let mut mapped = 0.0;
        for d in owned {
            let pop = districts::population_of(w, &d).unwrap_or(0.0).max(0.0);
            mapped += pop;
            if !w.population_system.provinces.contains_key(&d) {
                let p = seed_province(w, id, Some(&d), pop);
                w.population_system.provinces.insert(d, p);
            }
        }
        let missing = (w.nation(id).population - mapped).max(0.0);
        if initial_enrollment
            && missing > 1e-8
            && !w.population_system.unallocated.contains_key(&id)
        {
            let p = seed_province(w, id, None, missing);
            w.population_system.unallocated.insert(id, p);
        }
    }
}
/// The same affordable-place multiplier used by the Education ministry card.
/// Money is already spent by the fiscal ledger; this does not charge it again.
pub fn education_capacity_multiplier(w: &WorldState, id: NationId, gap: f64) -> f64 {
    let base = w
        .population_system
        .nations
        .get(&id)
        .map_or(0.035, |p| p.opening_education_share);
    let reference = w
        .nation_opt(id)
        .map_or(base, |n| n.budget_for(w.year).reference[BUDGET_EDUCATION]);
    ((reference + gap).max(0.0) / base).clamp(0.0, 4.0)
}
fn project_demands(w: &WorldState) -> BTreeMap<String, [[f64; 3]; 8]> {
    let mut out: BTreeMap<String, [[f64; 3]; 8]> = BTreeMap::new();
    let mut add = |d: &str, s: usize, people: f64, recipe: [f64; 3]| {
        let row = out.entry(d.into()).or_insert([[0.0; 3]; 8]);
        for g in 0..3 {
            row[s][g] += people * recipe[g];
        }
    };
    if crate::industry_operations::enabled(w) {
        use crate::production::ProjectKind as K;
        // Raw installed capacity only: never call an operating/staffing view
        // here, because those views consume the assignments we are building.
        let sites: BTreeSet<_> = w
            .production
            .provinces
            .iter()
            .map(|p| p.district.clone())
            .chain(w.production.industry.sites.keys().cloned())
            .chain(w.production.industry.modules.keys().cloned())
            .chain(w.production.rebuild_sites.keys().cloned())
            .collect();
        for d in sites {
            for kind in crate::production::PROJECT_KINDS {
                let jobs = crate::industry_operations::jobs_per_level(kind);
                if jobs <= 0.0 {
                    continue;
                }
                let levels = if matches!(
                    kind,
                    K::CivilianIndustry | K::StarterIndustry | K::Generation | K::PowerGrid
                ) {
                    crate::industrial_modules::effective_capacity(w, &d, kind)
                } else {
                    crate::production::level(w, &d, kind) as f64
                };
                if levels > 0.0 {
                    add(
                        &d,
                        crate::industry_operations::workforce_sector(kind),
                        levels * jobs / 1_000_000.0,
                        crate::industry_operations::skill_recipe(kind),
                    );
                }
            }
        }
    } else {
        for p in &w.production.provinces {
            add(
                &p.district,
                2,
                p.civilian_industry as f64 * 0.004,
                JOB_RECIPE[2],
            );
            add(
                &p.district,
                2,
                p.arms_plants as f64 * 0.004,
                [0.45, 0.42, 0.13],
            );
            add(&p.district, 3, p.power_grid as f64 * 0.001, JOB_RECIPE[3]);
            add(
                &p.district,
                7,
                p.research_centers as f64 * 0.0015,
                [0.10, 0.15, 0.75],
            );
            add(
                &p.district,
                5,
                p.infrastructure as f64 * 0.0007,
                JOB_RECIPE[5],
            );
        }
        for (d, levels) in &w.production.industry.sites {
            // Site order: machinery, generation, processing, freight, warehouse,
            // automation, efficiency. The last two are improvements, not employers.
            for (i, &level) in levels.iter().enumerate().take(5) {
                let s = match i {
                    0 | 2 => 2,
                    1 => 3,
                    _ => 5,
                };
                add(d, s, level as f64 * 0.002, JOB_RECIPE[s]);
            }
        }
        for (d, micros) in &w.production.industry.modules {
            add(d, 2, *micros as f64 / 1_000_000.0 * 0.004, JOB_RECIPE[2]);
            add(d, 3, *micros as f64 / 1_000_000.0 * 0.001, JOB_RECIPE[3]);
        }
    }
    for p in &w.production.projects {
        if matches!(
            p.status,
            crate::production::ProjectStatus::Building | crate::production::ProjectStatus::Slowed
        ) {
            add(
                &p.district,
                4,
                0.0015 * p.capacity_micros.map_or(1.0, |m| m as f64 / 1_000_000.0),
                crate::construction_capacity::CREW_SKILL_RECIPE,
            );
        }
    }
    for p in &w.resources.mine_projects {
        if p.days_left.map_or(p.months_left > 0, |days| days > 0) {
            // A mine's civil works compete for the same construction crew.
            // Extraction after completion remains in the inherited sector
            // until its resource-output accounting is explicitly replaced.
            add(
                &p.district,
                4,
                0.0015,
                crate::construction_capacity::CREW_SKILL_RECIPE,
            );
        }
    }
    out
}
#[derive(Clone)]
struct Context {
    policy: Policy,
    education_ratio: f64,
    welfare_gap: f64,
    fertility_support: f64,
    mortality_support: f64,
    demand_cycle: f64,
    real_income: f64,
    military_ratio: f64,
    year: i32,
    day: i32,
    year_fraction: f64,
    days: f64,
    month_blend: f64,
    year_days_left: f64,
}
fn context(w: &WorldState, id: NationId, row: &NationPopulation) -> Context {
    let n = w.nation(id);
    let education_ratio = education_capacity_multiplier(w, id, n.budget_gap(BUDGET_EDUCATION));
    // Employment demand responds to the non-labor business cycle, with a bounded
    // level and slow adjustment. Labor's own GDP contribution is subtracted;
    // cumulative GDP is never converted back into unbounded extra job creation.
    let non_labor_cycle = n.growth_last - row.labor_growth;
    let demand_cycle = (1.0 + (non_labor_cycle - 0.02) * 1.25).clamp(0.75, 1.12);
    let real_income = ratio(
        ratio(n.gdp, n.population),
        ratio(row.opening_gdp_bn, row.opening_population_m),
    )
    .clamp(0.15, 8.0);
    Context {
        policy: row.policy,
        education_ratio,
        welfare_gap: n.budget_gap(BUDGET_PENSIONS),
        fertility_support: crate::ministries::housing_population(n.budget_gap(BUDGET_HOUSING)),
        mortality_support: crate::ministries::health_population(n.budget_gap(BUDGET_HEALTH))
            + crate::tech::demographic_growth(n),
        demand_cycle,
        real_income,
        military_ratio: (n.mil_strength / row.opening_military_strength).clamp(0.0, 20.0),
        year: w.year,
        day: clock::absolute_day(w),
        year_fraction: clock::year_fraction(w),
        days: if clock::is_daily(w) {
            1.0
        } else {
            crate::world::days_in_month(w.year, w.month) as f64
        },
        month_blend: clock::blend(w, 0.05),
        year_days_left: (clock::date_day(w.year + 1, 1, 1) - clock::absolute_day(w)).max(1) as f64,
    }
}
fn demography(p: &mut ProvincePopulation, c: &Context) {
    let before = p.population_m();
    let adult_before = p.working_age_m();
    // Positive health/technology support reduces observed mortality first; it
    // cannot create negative deaths. Housing enters births, the named arm.
    let raw_deaths = p.children_m() * 0.002 + adult_before * 0.002 + p.retirees_m * 0.055;
    let mortality = ratio(
        (raw_deaths - c.mortality_support * before).max(0.0),
        raw_deaths,
    )
    .clamp(0.0, 3.0);
    let child_survival = (1.0 - 0.002 * mortality * c.year_fraction).clamp(0.0, 1.0);
    for child in p.children.values_mut() {
        *child *= child_survival;
    }
    let mature_keys: Vec<_> = p
        .children
        .keys()
        .filter(|&&year| year <= c.year - 15)
        .copied()
        .collect();
    let mut entrants = 0.0;
    for year in mature_keys {
        let amount = p.children.get_mut(&year).unwrap();
        let graduated = (*amount * c.days / c.year_days_left).min(*amount);
        *amount -= graduated;
        entrants += graduated;
    }
    p.children.retain(|_, p| *p > 1e-14);
    let retirement_fraction = (c.year_fraction / 50.0).min(1.0);
    let survival = (1.0 - 0.002 * mortality * c.year_fraction).clamp(0.0, 1.0);
    let mut retired = 0.0;
    for adult in &mut p.working {
        *adult *= survival;
        let exits = *adult * retirement_fraction;
        *adult -= exits;
        retired += exits;
    }
    // Courses track subsets of these adults, so the same exits shrink both.
    for course in &mut p.courses {
        course.people_m *= survival * (1.0 - retirement_fraction);
    }
    p.retirees_m =
        p.retirees_m * (1.0 - 0.055 * mortality * c.year_fraction).clamp(0.0, 1.0) + retired;
    // Schooling produces basic/secondary entrants; degrees and trade tickets
    // require the separate, funded adult pipeline.
    let secondary = entrants * p.school_coverage * 0.55;
    let basic = entrants * p.school_coverage * 0.45;
    p.working[0] += entrants - secondary - basic;
    p.working[1] += basic;
    p.working[2] += secondary;
    let births = (adult_before * p.birth_rate_per_adult + c.fertility_support * before).max(0.0)
        * c.year_fraction;
    *p.children.entry(c.year).or_default() += births;
}
fn schooling(p: &mut ProvincePopulation, c: &Context) {
    let adults = p.working_age_m();
    let teacher_staff = ratio(p.filled[7][2], p.jobs[7][2] + p.project_jobs[7][2]).min(1.0);
    let funded = c.education_ratio.min(teacher_staff);
    let school_target = (p.school_reference * c.education_ratio).clamp(0.0, 1.0) * teacher_staff;
    // A school budget changes the pipeline, not next day's qualifications.
    p.school_coverage += (school_target - p.school_coverage) * (c.month_blend * 0.30);
    p.training_capacity_m = adults * 0.04 * c.education_ratio * teacher_staff;
    let mut completed = 0.0;
    for course in &mut p.courses {
        course.funded_days += c.days * funded.min(1.0);
        if course.funded_days + 1e-9 >= course.required_days {
            let amount = course.people_m.min(p.working[course.from]);
            p.working[course.from] -= amount;
            p.working[course.to] += amount;
            completed += amount;
            course.people_m = 0.0;
        }
    }
    p.courses.retain(|course| course.people_m > 1e-14);
    // Exponentially smoothed actual annual completion pace, with a one-year
    // window. At startup no fictional completed graduates are awarded.
    p.annual_graduates_m = p.annual_graduates_m * (1.0 - c.year_fraction).max(0.0) + completed;
    let mut free = (p.training_capacity_m - p.students_m()).max(0.0);
    let weights = c.policy.course_weights();
    let date = clock::date_from_day(c.day);
    let intake_month = date.0 * 12 + date.1 as i32;
    if p.intake_month == intake_month {
        return;
    }
    p.intake_month = intake_month;
    for (route, (from, to, duration)) in
        [(0, 1, 183.0), (1, 2, 730.0), (2, 3, 548.0), (2, 4, 1461.0)]
            .into_iter()
            .enumerate()
    {
        let unemployed = p.unemployment_by_skill[from];
        let already = p
            .courses
            .iter()
            .filter(|x| x.from == from)
            .map(|x| x.people_m)
            .sum::<f64>();
        let available = (p.working[from] - already).max(0.0);
        // Priority goes to jobseekers. Full-time workers are never yanked out
        // of factories by a one-click change of education focus.
        let intake = (unemployed * 0.25 * weights[route])
            .min(available)
            .min(free * weights[route]);
        if intake > 1e-12 {
            p.courses.push(Course {
                from,
                to,
                people_m: intake,
                started_day: c.day,
                required_days: duration,
                funded_days: 0.0,
            });
            free -= intake;
        }
    }
}
fn match_jobs(p: &mut ProvincePopulation) {
    let adults = p.working_age_m();
    let mut enrolled = [0.0; SKILLS];
    for c in &p.courses {
        enrolled[c.from] += c.people_m;
    }
    let nonmilitary = (1.0 - ratio(p.military_m, adults - p.students_m())).clamp(0.0, 1.0);
    let mut pool = [0.0; SKILLS];
    for k in 0..SKILLS {
        pool[k] = (p.working[k] - enrolled[k]).max(0.0) * nonmilitary * p.participation;
    }
    p.labor_force_m = pool.iter().sum();
    p.filled = [[0.0; GRADES]; 8];
    p.employed_by_skill = [0.0; SKILLS];
    p.underemployed_m = 0.0;
    p.inherited_filled_m = 0.0;
    for grade in (0..GRADES).rev() {
        let eligible: &[usize] = match grade {
            2 => &[4],
            1 => &[3, 4],
            _ => &[0, 1, 2, 3, 4],
        };
        let demand = (0..8)
            .map(|s| p.jobs[s][grade] + p.project_jobs[s][grade])
            .sum::<f64>();
        let available = eligible.iter().map(|&k| pool[k]).sum::<f64>();
        let filled = demand.min(available);
        if filled <= 0.0 {
            continue;
        }
        let mut remaining = filled;
        // Exact-qualified people are considered before overqualified workers.
        for &k in eligible {
            let assigned = pool[k].min(remaining);
            pool[k] -= assigned;
            remaining -= assigned;
            p.employed_by_skill[k] += assigned;
            if (grade == 0 && k >= 3) || (grade == 1 && k == 4) {
                p.underemployed_m += assigned;
            }
        }
        let mut left = filled;
        let last = (0..8)
            .rfind(|&s| p.jobs[s][grade] + p.project_jobs[s][grade] > 0.0)
            .unwrap_or(7);
        for s in 0..8 {
            let total = p.jobs[s][grade] + p.project_jobs[s][grade];
            let assigned = if s == last {
                left
            } else {
                (filled * ratio(total, demand)).min(left)
            };
            if total > 0.0 {
                p.filled[s][grade] = assigned;
                p.inherited_filled_m += assigned * ratio(p.jobs[s][grade], total);
                left -= assigned;
            }
        }
    }
    p.unemployment_by_skill = pool;
    for s in 0..8 {
        p.wage_indices[s] = WAGES[s] * (1.0 + (1.0 - p.raw_staffing(s)).max(0.0) * 0.40);
    }
}
fn classes(p: &mut ProvincePopulation, blend: f64, opening: bool) {
    let employed = p.employed_m();
    let jobless = p.unemployed_m();
    let routine = p.filled.iter().map(|s| s[0]).sum::<f64>();
    let technical = p.filled.iter().map(|s| s[1]).sum::<f64>();
    let professional = p.filled.iter().map(|s| s[2]).sum::<f64>();
    let agriculture = p.filled[0].iter().sum::<f64>();
    let mut target = [
        agriculture * 0.94,
        (routine - agriculture).max(0.0) * 0.87,
        technical * 0.94,
        professional * 0.94,
        employed * 0.10,
        employed * 0.018,
    ];
    // Job loss initially stays in the person's household class. Permanent
    // mobility follows sustained occupational changes, not one bad day.
    for i in 0..6 {
        target[i] += jobless * p.class_shares[i];
    }
    let total = target.iter().sum::<f64>();
    if total > 1e-12 {
        for v in &mut target {
            *v /= total;
        }
    } else {
        target = p.class_shares;
    }
    let risk = class_risks(p);
    let mut moved = 0.0;
    for i in 0..6 {
        let delta = if opening {
            target[i] - p.class_shares[i]
        } else {
            (target[i] - p.class_shares[i]) * blend
        };
        p.class_shares[i] += delta;
        moved += delta.abs();
        if opening {
            p.class_risk_reference[i] = risk[i];
            p.class_living[i] = 100.0;
        } else {
            let grade = match i {
                2 => 1,
                3 => 2,
                _ => 0,
            };
            let grade_jobs = p.jobs.iter().map(|s| s[grade]).sum::<f64>()
                + p.project_jobs.iter().map(|s| s[grade]).sum::<f64>();
            let grade_filled = p.filled.iter().map(|s| s[grade]).sum::<f64>();
            // Scarce qualifications bid up their wage; small proprietors and
            // capital owners depend more on customer demand and job security.
            let scarcity = if (i == 2 || i == 3) && grade_jobs > 1e-12 {
                (1.0 - ratio(grade_filled, grade_jobs)).max(0.0) * 0.30
            } else {
                0.0
            };
            let target_living = p.living_standards * (1.0 - risk[i] * 0.55)
                / (1.0 - p.class_risk_reference[i] * 0.55)
                * (1.0 + scarcity);
            p.class_living[i] += (target_living - p.class_living[i]) * blend;
            let income = CLASS_INCOME[i] * p.class_living[i] / 100.0;
            p.wealth_indices[i] =
                (p.wealth_indices[i] + (income - 0.90) * blend * 4.0).clamp(0.0, 2000.0);
        }
    }
    let total = p.class_shares.iter().sum::<f64>();
    if total > 0.0 {
        for v in &mut p.class_shares {
            *v /= total;
        }
    }
    p.social_mobility = if opening { 0.0 } else { moved * 0.5 };
}
fn class_risks(p: &ProvincePopulation) -> [f64; 6] {
    let general = ratio(p.unemployed_m(), p.labor_force_m);
    let routine = ratio(
        p.unemployment_by_skill[..3].iter().sum(),
        p.working[..3].iter().sum::<f64>() * p.participation,
    )
    .min(1.0);
    let skilled = ratio(p.unemployment_by_skill[3], p.working[3] * p.participation).min(1.0);
    let professional = ratio(p.unemployment_by_skill[4], p.working[4] * p.participation).min(1.0);
    [
        general * 0.6 + (1.0 - p.raw_staffing(0)) * 0.4,
        routine,
        skilled,
        professional,
        general * 0.65,
        general * 0.30,
    ]
}
fn write_populations(w: &mut WorldState) {
    let mut totals: BTreeMap<NationId, f64> = BTreeMap::new();
    for (d, p) in &w.population_system.provinces {
        if let Some(&id) = w.districts.get(d) {
            let amount = p.population_m();
            *totals.entry(id).or_default() += amount;
            let scale = w
                .district_population_scale
                .get(id.index())
                .copied()
                .unwrap_or(1.0);
            w.district_population
                .insert(d.clone(), amount / scale.max(1e-30));
        }
    }
    for (&id, p) in &w.population_system.unallocated {
        if w.nation_opt(id).is_some_and(|n| n.alive) {
            *totals.entry(id).or_default() += p.population_m();
        }
    }
    for (id, total) in totals {
        if w.nation_opt(id).is_some_and(|n| n.alive) {
            w.nation_mut(id).population = total;
        }
    }
}
/// One observation per actual date; all recurring quantities use model time.
pub fn tick(w: &mut WorldState) {
    if !active(w) {
        return;
    }
    let day = clock::absolute_day(w);
    if w.population_system.last_day == Some(day) {
        return;
    }
    ensure(w);
    let contexts: BTreeMap<_, _> = w
        .population_system
        .nations
        .iter()
        .filter(|(id, _)| w.nation_opt(**id).is_some_and(|n| n.alive))
        .map(|(&id, row)| (id, context(w, id, row)))
        .collect();
    let before_population: BTreeMap<_, _> = contexts
        .keys()
        .map(|&id| (id, sum_rows(w, id, |p| p.population_m())))
        .collect();
    let demands = project_demands(w);
    let mut state = std::mem::take(&mut w.population_system);
    let mut owner_changed = BTreeMap::new();
    for (d, p) in &mut state.provinces {
        let Some(&id) = w.districts.get(d) else {
            continue;
        };
        let Some(c) = contexts.get(&id) else { continue };
        if p.last_owner != id {
            owner_changed.insert(p.last_owner, true);
            owner_changed.insert(id, true);
            p.last_owner = id;
        }
        advance_province(p, c, demands.get(d).copied().unwrap_or([[0.0; 3]; 8]));
    }
    for (&id, p) in &mut state.unallocated {
        if let Some(c) = contexts.get(&id) {
            advance_province(p, c, [[0.0; 3]; 8]);
        }
    }
    state.last_day = Some(day);
    w.population_system = state;
    migrate(w);
    write_populations(w);
    for (&id, c) in &contexts {
        let employed = sum_rows(w, id, |p| p.inherited_filled_m);
        let research = researchers(w, id);
        let hard = hardship_level(w, id);
        let population = sum_rows(w, id, |p| p.population_m());
        let row = w.population_system.nations.get_mut(&id).unwrap();
        if row.last_inherited_employed_m > 1e-12 && !owner_changed.contains_key(&id) {
            row.labor_growth = (exact::ln((employed / row.last_inherited_employed_m).max(0.001))
                * 0.60
                / c.year_fraction)
                .clamp(-0.15, 0.15);
        } else {
            row.labor_growth = 0.0;
        }
        row.last_inherited_employed_m = employed;
        row.population_growth = if before_population[&id] > 1e-12 {
            (population - before_population[&id]) / before_population[&id] / c.year_fraction
        } else {
            0.0
        };
        if row.research_reference <= 1e-12 {
            row.research_reference = research.max(1e-9);
            row.hardship_reference = hard;
        }
    }
    publish_outcomes(w);
    ai_tick(w);
}
fn advance_province(p: &mut ProvincePopulation, c: &Context, project: [[f64; 3]; 8]) {
    demography(p, c);
    let adults = p.working_age_m();
    let target_participation = (p.participation_reference
        + if c.policy == Policy::BackToWork {
            0.035
        } else {
            0.0
        }
        - crate::ministries::pensions_jobs(c.welfare_gap))
    .clamp(0.25, 0.95);
    p.participation += (target_participation - p.participation) * c.month_blend;
    let target_military = adults * 0.012 * c.military_ratio;
    p.military_m += (target_military - p.military_m) * c.month_blend;
    p.military_m = p.military_m.clamp(0.0, adults * 0.10);
    schooling(p, c);
    // Natural population growth increases civilian demand, while the cyclic
    // correction remains a level bounded around inherited capacity.
    let size = ratio(p.population_m(), p.reference_population_m);
    for s in 0..8 {
        for g in 0..3 {
            let target = p.jobs_reference[s][g] * size * c.demand_cycle;
            p.jobs[s][g] += (target - p.jobs[s][g]) * c.month_blend;
        }
    }
    p.project_jobs = project;
    match_jobs(p);
    let security = (1.0 - ratio(p.unemployed_m(), p.labor_force_m) * 0.40).max(0.5);
    let target_living = 100.0 * c.real_income * security / (1.0 - p.opening_unemployment * 0.40);
    p.living_standards += (target_living - p.living_standards) * c.month_blend;
    classes(p, c.month_blend, false);
    p.social_mobility = ratio(p.social_mobility, c.year_fraction);
}
/// Households relocate inside their country toward job openings. Moves are
/// capped at 0.5% of source population/year and 10% of destination vacancies.
/// Skills and schooling progress travel with people. No international teleport.
fn migrate(w: &mut WorldState) {
    let dt = clock::year_fraction(w);
    let mut changed = BTreeSet::new();
    let mut by_owner: BTreeMap<NationId, Vec<String>> = BTreeMap::new();
    for d in w.population_system.provinces.keys() {
        if let Some(&id) = w.districts.get(d) {
            by_owner.entry(id).or_default().push(d.clone());
        }
    }
    for (_, ids) in by_owner {
        if ids.len() < 2 {
            continue;
        }
        let dest = ids
            .iter()
            .filter_map(|d| {
                w.population_system
                    .provinces
                    .get(d)
                    .map(|p| (d, p.vacancies_m().max(0.0)))
            })
            .max_by(|a, b| a.1.total_cmp(&b.1).then_with(|| b.0.cmp(a.0)));
        let Some((dest, mut room)) = dest.map(|(d, p)| (d.clone(), p * 0.10 * dt)) else {
            continue;
        };
        if room < 1e-10 {
            continue;
        }
        let target_rate = {
            let p = &w.population_system.provinces[&dest];
            ratio(p.unemployed_m(), p.labor_force_m)
        };
        for source in &ids {
            if *source == dest || room <= 1e-12 {
                continue;
            }
            let p = &w.population_system.provinces[source];
            let gap = ratio(p.unemployed_m(), p.labor_force_m) - target_rate;
            if gap <= 0.015 {
                continue;
            }
            let amount = (p.population_m() * 0.005 * dt)
                .min(room)
                .min(p.unemployed_m() * 0.05 * dt);
            if amount <= 1e-12 {
                continue;
            }
            let fraction = ratio(amount, p.population_m());
            let mut moved = p.clone();
            scale_people(&mut moved, fraction);
            let original = w.population_system.provinces.get_mut(source).unwrap();
            scale_people(original, 1.0 - fraction);
            // Capacity stays at the site; only residents and their history move.
            add_people(
                w.population_system.provinces.get_mut(&dest).unwrap(),
                &moved,
            );
            changed.insert(source.clone());
            changed.insert(dest.clone());
            room -= amount;
        }
    }
    // Matching follows relocation so no departed worker remains on a payroll.
    for d in changed {
        match_jobs(w.population_system.provinces.get_mut(&d).unwrap());
    }
}
fn scale_people(p: &mut ProvincePopulation, f: f64) {
    for v in p.children.values_mut() {
        *v *= f;
    }
    for v in &mut p.working {
        *v *= f;
    }
    p.retirees_m *= f;
    p.military_m *= f;
    for course in &mut p.courses {
        course.people_m *= f;
    }
}
fn add_people(to: &mut ProvincePopulation, from: &ProvincePopulation) {
    let old = to.population_m();
    let incoming = from.population_m();
    let weight = ratio(incoming, old + incoming);
    for (&year, &pop) in &from.children {
        *to.children.entry(year).or_default() += pop;
    }
    for i in 0..SKILLS {
        to.working[i] += from.working[i];
    }
    to.retirees_m += from.retirees_m;
    to.military_m += from.military_m;
    for course in &from.courses {
        if let Some(same) = to.courses.iter_mut().find(|x| {
            x.from == course.from && x.to == course.to && x.started_day == course.started_day
        }) {
            same.funded_days = (same.funded_days * same.people_m
                + course.funded_days * course.people_m)
                / (same.people_m + course.people_m).max(1e-30);
            same.people_m += course.people_m;
        } else {
            to.courses.push(course.clone());
        }
    }
    for i in 0..6 {
        let class_weight = ratio(
            incoming * from.class_shares[i],
            old * to.class_shares[i] + incoming * from.class_shares[i],
        );
        to.class_shares[i] += (from.class_shares[i] - to.class_shares[i]) * weight;
        to.wealth_indices[i] += (from.wealth_indices[i] - to.wealth_indices[i]) * class_weight;
        to.class_living[i] += (from.class_living[i] - to.class_living[i]) * class_weight;
        to.class_risk_reference[i] +=
            (from.class_risk_reference[i] - to.class_risk_reference[i]) * class_weight;
    }
}
fn researchers(w: &WorldState, id: NationId) -> f64 {
    // The inherited research workforce supplies ordinary research. Explicit
    // centers use their assigned professionals for paid prototype services in
    // industry::research_day; counting them here would reward the same center
    // again through both ordinary effort and a cheaper acquisition bill.
    sum_rows(w, id, |p| {
        p.filled[7][2] * ratio(p.jobs[7][2], p.jobs[7][2] + p.project_jobs[7][2]) * 0.20
    })
}
fn hardship_level(w: &WorldState, id: NationId) -> f64 {
    let pop = sum_rows(w, id, |p| p.population_m());
    ratio(
        sum_rows(w, id, |p| {
            let risks = class_risks(p);
            p.population_m()
                * (0..6)
                    .map(|i| {
                        p.class_shares[i]
                            * (risks[i] * 0.35
                                + ((100.0 - p.class_living[i]) / 100.0).max(0.0) * 0.65)
                    })
                    .sum::<f64>()
        }),
        pop,
    )
}
pub fn hardship(w: &WorldState, id: NationId) -> f64 {
    if !active(w) {
        return 0.0;
    }
    w.population_system.nations.get(&id).map_or(0.0, |r| {
        (hardship_level(w, id) - r.hardship_reference).clamp(-0.25, 0.50)
    })
}
pub fn unemployment(w: &WorldState, id: NationId) -> Option<f64> {
    if !active(w) || !w.population_system.nations.contains_key(&id) {
        return None;
    }
    Some(
        ratio(
            sum_rows(w, id, |p| p.unemployed_m()),
            sum_rows(w, id, |p| p.labor_force_m),
        )
        .min(1.0),
    )
}
pub fn labor_growth(w: &WorldState, id: NationId) -> Option<f64> {
    if !active(w) {
        return None;
    }
    w.population_system.nations.get(&id).map(|n| n.labor_growth)
}
pub fn effective_demographic_growth(w: &WorldState, id: NationId) -> Option<f64> {
    if !active(w) {
        return None;
    }
    w.population_system
        .nations
        .get(&id)
        .map(|n| n.population_growth)
}
pub fn research_multiplier(w: &WorldState, id: NationId) -> Option<f64> {
    if !active(w) {
        return None;
    }
    w.population_system
        .nations
        .get(&id)
        .map(|n| ratio(researchers(w, id), n.research_reference).clamp(0.0, 2.25))
}
pub fn district_staffing(w: &WorldState, d: &str, sector: usize) -> f64 {
    if !active(w) || sector >= 8 {
        return 1.0;
    }
    w.population_system.provinces.get(d).map_or(1.0, |p| {
        (p.raw_staffing(sector) / p.staffing_reference[sector].max(0.10)).clamp(0.0, 1.0)
    })
}
pub fn staffing(w: &WorldState, id: NationId, sector: usize) -> f64 {
    if !active(w) || sector >= 8 {
        return 1.0;
    }
    let demand = sum_rows(w, id, |p| {
        p.jobs[sector].iter().sum::<f64>() + p.project_jobs[sector].iter().sum::<f64>()
    });
    if demand <= 1e-12 {
        return 1.0;
    }
    ratio(
        sum_rows(w, id, |p| {
            (p.jobs[sector].iter().sum::<f64>() + p.project_jobs[sector].iter().sum::<f64>())
                * (p.raw_staffing(sector) / p.staffing_reference[sector].max(0.10)).clamp(0.0, 1.0)
        }),
        demand,
    )
    .min(1.0)
}
pub fn military_staffing(w: &WorldState, id: NationId) -> f64 {
    if !active(w) {
        return 1.0;
    }
    let Some(r) = w.population_system.nations.get(&id) else {
        return 1.0;
    };
    let relative = ratio(w.nation(id).mil_strength, r.opening_military_strength);
    let desired = sum_rows(w, id, |p| p.working_age_m() * 0.012 * relative);
    if desired <= 1e-12 {
        1.0
    } else {
        ratio(sum_rows(w, id, |p| p.military_m), desired).min(1.0)
    }
}
/// Called only for explicit combat losses, not force decay or demobilization.
/// The model treats one fifth of lost exposed personnel as fatalities; force
/// points remain an estimate rather than a claim to sourced military headcounts.
pub fn record_casualties(w: &mut WorldState, id: NationId, force_loss_fraction: f64) {
    if !active(w) || force_loss_fraction <= 0.0 {
        return;
    }
    let fraction = force_loss_fraction.clamp(0.0, 1.0) * 0.20;
    let keys: Vec<_> = w
        .population_system
        .provinces
        .keys()
        .filter(|d| w.districts.get(*d) == Some(&id))
        .cloned()
        .collect();
    for d in keys {
        casualties(w.population_system.provinces.get_mut(&d).unwrap(), fraction);
    }
    if let Some(p) = w.population_system.unallocated.get_mut(&id) {
        casualties(p, fraction);
    }
    write_populations(w);
    publish_outcomes(w);
}
fn casualties(p: &mut ProvincePopulation, f: f64) {
    let deaths = (p.military_m * f).min(p.working_age_m() - p.students_m());
    let mut students = [0.0; SKILLS];
    for c in &p.courses {
        students[c.from] += c.people_m;
    }
    let available = p.working_age_m() - p.students_m();
    for (k, adult) in p.working.iter_mut().enumerate() {
        *adult -= deaths * ratio((*adult - students[k]).max(0.0), available);
    }
    p.military_m = (p.military_m - deaths).max(0.0);
    match_jobs(p);
}
fn publish_outcomes(w: &mut WorldState) {
    if !active(w) {
        return;
    }
    let ids: Vec<_> = w.nations.iter().filter(|n| n.alive).map(|n| n.id).collect();
    for id in ids {
        if let (Some(labor_growth), Some(unemployment)) = (labor_growth(w, id), unemployment(w, id))
        {
            w.nation_mut(id).population_outcomes = Some(PopulationOutcomes {
                labor_growth,
                unemployment,
            });
        }
    }
}
pub fn policy_quote(w: &WorldState, id: NationId, policy: Policy) -> PolicyQuote {
    let row = w.population_system.nations.get(&id);
    let selected = row.is_some_and(|n| n.policy == policy);
    let political_cost = if selected { 0.0 } else { 8.0 };
    let cooldown_days = row.and_then(|n| n.last_policy_day).map_or(0, |last| {
        (180 - (clock::absolute_day(w) - last)).max(0) as u32
    });
    let reason = if !active(w) {
        Some("Start a daily population campaign to set a focus.".into())
    } else if !w.nation_opt(id).is_some_and(|n| n.alive) || row.is_none() {
        Some("This country has no active population accounts.".into())
    } else if selected {
        Some("This focus is already active.".into())
    } else if cooldown_days > 0 {
        Some(format!(
            "The current focus settles in {cooldown_days} days."
        ))
    } else if w.nation(id).political_capital < political_cost {
        Some(format!("Needs {political_cost:.0} political capital."))
    } else {
        None
    };
    PolicyQuote {
        policy,
        name: policy.name(),
        description: policy.description(),
        political_cost,
        cooldown_days,
        available: reason.is_none(),
        reason,
        selected,
    }
}
pub fn apply_policy(w: &mut WorldState, id: NationId, policy: Policy) -> Result<(), String> {
    let quote = policy_quote(w, id, policy);
    if let Some(reason) = quote.reason {
        return Err(reason);
    }
    let day = clock::absolute_day(w);
    let n = w
        .population_system
        .nations
        .get_mut(&id)
        .expect("quote checked population account");
    n.policy = policy;
    n.last_policy_day = Some(day);
    w.headline(format!(
        "{} chooses {}: {}",
        id.name(),
        policy.name(),
        policy.description()
    ));
    Ok(())
}
fn province_view(d: &str, p: &ProvincePopulation) -> ProvinceSnapshot {
    ProvinceSnapshot {
        district: d.into(),
        name: if d.is_empty() {
            "Unallocated national population".into()
        } else {
            districts::name_of(d).unwrap_or(d).into()
        },
        population_m: p.population_m(),
        employed_m: p.employed_m(),
        unemployed_m: p.unemployed_m(),
        vacancies_m: p.vacancies_m().max(0.0),
        unemployment_rate: ratio(p.unemployed_m(), p.labor_force_m),
        living_standards: p.living_standards,
        students_m: p.children_m() * p.school_coverage + p.students_m(),
    }
}
pub fn province(w: &WorldState, d: &str) -> Option<ProvinceSnapshot> {
    if !active(w) || !w.districts.contains_key(d) {
        return None;
    }
    w.population_system
        .provinces
        .get(d)
        .map(|p| province_view(d, p))
}
pub fn snapshot(w: &WorldState, id: NationId) -> Option<NationSnapshot> {
    if !active(w) || !w.nation_opt(id).is_some_and(|n| n.alive) {
        return None;
    }
    let n = w.population_system.nations.get(&id)?;
    let mut rows: Vec<(&str, &ProvincePopulation)> = w
        .population_system
        .provinces
        .iter()
        .filter(|(d, _)| w.districts.get(*d) == Some(&id))
        .map(|(d, p)| (d.as_str(), p))
        .collect();
    if let Some(p) = w.population_system.unallocated.get(&id) {
        rows.push(("", p));
    }
    let mut population_m = 0.0;
    let mut children_m = 0.0;
    let mut working_age_m = 0.0;
    let mut retirees_m = 0.0;
    let mut labor_force_m = 0.0;
    let mut employed_m = 0.0;
    let mut adult_students = 0.0;
    let mut school_students = 0.0;
    let mut military_m = 0.0;
    let mut vacancies_m = 0.0;
    let mut skilled_vacancies_m = 0.0;
    let mut living = 0.0;
    let mut mobility = 0.0;
    let mut training_capacity_m = 0.0;
    let mut annual_graduates_m = 0.0;
    let mut underemployed = 0.0;
    let mut education = [0.0; 5];
    let mut class_people = [0.0; 6];
    let mut wealth = [0.0; 6];
    let mut class_living = [0.0; 6];
    let mut class_risk = [0.0; 6];
    let mut jobs = [0.0; 8];
    let mut employed = [0.0; 8];
    let mut wages = [0.0; 8];
    let mut course_people = [0.0; 4];
    let mut course_progress = [0.0; 4];
    let mut course_days = [u32::MAX; 4];
    for (_, p) in &rows {
        let pop = p.population_m();
        population_m += pop;
        children_m += p.children_m();
        working_age_m += p.working_age_m();
        retirees_m += p.retirees_m;
        labor_force_m += p.labor_force_m;
        employed_m += p.employed_m();
        adult_students += p.students_m();
        school_students += p.children_m() * p.school_coverage;
        military_m += p.military_m;
        vacancies_m += p.vacancies_m().max(0.0);
        living += p.living_standards * pop;
        mobility += p.social_mobility * pop;
        training_capacity_m += p.training_capacity_m;
        annual_graduates_m += p.annual_graduates_m;
        underemployed += p.underemployed_m;
        for k in 0..5 {
            education[k] += p.working[k];
        }
        let risks = class_risks(p);
        for k in 0..6 {
            let weight = p.class_shares[k] * pop;
            class_people[k] += weight;
            wealth[k] += p.wealth_indices[k] * weight;
            class_living[k] += p.class_living[k] * weight;
            class_risk[k] += risks[k] * weight;
        }
        for s in 0..8 {
            let j = p.jobs[s].iter().sum::<f64>() + p.project_jobs[s].iter().sum::<f64>();
            jobs[s] += j;
            employed[s] += p.filled[s].iter().sum::<f64>();
            wages[s] += p.wage_indices[s] * j;
            for g in 1..3 {
                skilled_vacancies_m +=
                    (p.jobs[s][g] + p.project_jobs[s][g] - p.filled[s][g]).max(0.0);
            }
        }
        for course in &p.courses {
            let r = match course.to {
                1 => 0,
                2 => 1,
                3 => 2,
                _ => 3,
            };
            course_people[r] += course.people_m;
            course_progress[r] += ratio(course.funded_days, course.required_days) * course.people_m;
            course_days[r] = course_days[r]
                .min((course.required_days - course.funded_days).max(0.0).ceil() as u32);
        }
    }
    let unemployment_rate = ratio((labor_force_m - employed_m).max(0.0), labor_force_m);
    let living_standards = ratio(living, population_m);
    let classes = (0..6)
        .map(|i| {
            let living_standards = ratio(class_living[i], class_people[i]);
            ClassSnapshot {
                key: CLASS_KEYS[i],
                name: CLASS_NAMES[i],
                people_m: class_people[i],
                share: ratio(class_people[i], population_m),
                income_index: CLASS_INCOME[i] * living_standards,
                living_standards,
                wealth_index: ratio(wealth[i], class_people[i]),
                discontent: (ratio(class_risk[i], class_people[i]) * 0.70
                    + ((100.0 - living_standards) / 100.0).max(0.0) * 0.60)
                    .clamp(0.0, 1.0),
            }
        })
        .collect();
    let sectors = (0..8)
        .map(|s| SectorSnapshot {
            key: province_economy::SECTORS[s],
            name: province_economy::SECTOR_NAMES[s],
            jobs_m: jobs[s],
            employed_m: employed[s],
            vacancies_m: (jobs[s] - employed[s]).max(0.0),
            staffing: if jobs[s] > 1e-12 {
                ratio(employed[s], jobs[s])
            } else {
                1.0
            },
            wage_index: ratio(wages[s], jobs[s]) * 100.0,
        })
        .collect();
    let courses = (0..4)
        .filter(|&i| course_people[i] > 1e-12)
        .map(|i| CourseSnapshot {
            name: [
                "Foundation courses",
                "Secondary catch-up",
                "Trade qualifications",
                "University degrees",
            ][i],
            people_m: course_people[i],
            progress: ratio(course_progress[i], course_people[i]),
            days_remaining: course_days[i],
        })
        .collect();
    Some(NationSnapshot {
        population_m,
        children_m,
        working_age_m,
        retirees_m,
        labor_force_m,
        employed_m,
        unemployed_m: (labor_force_m - employed_m).max(0.0),
        students_m: adult_students + school_students,
        outside_workforce_m: (working_age_m - labor_force_m - adult_students - military_m).max(0.0),
        military_m,
        unemployment_rate,
        participation_rate: ratio(labor_force_m, working_age_m),
        employment_rate: ratio(employed_m, working_age_m),
        underemployment_rate: ratio(underemployed, employed_m),
        vacancies_m,
        skilled_vacancies_m,
        living_standards,
        social_mobility: ratio(mobility, population_m),
        research_multiplier: research_multiplier(w, id).unwrap_or(1.0),
        education_budget_bn: w.nation(id).gdp
            * w.nation(id).budget_for(w.year).allocations[BUDGET_EDUCATION],
        school_coverage: ratio(school_students, children_m),
        training_capacity_m,
        annual_graduates_m,
        policy: n.policy,
        policies: Policy::ALL
            .into_iter()
            .map(|p| policy_quote(w, id, p))
            .collect(),
        education: (0..5)
            .map(|i| EducationSnapshot {
                key: EDUCATION_KEYS[i],
                name: EDUCATION_NAMES[i],
                people_m: education[i],
                share: ratio(education[i], working_age_m),
            })
            .collect(),
        classes,
        sectors,
        provinces: rows.into_iter().map(|(d, p)| province_view(d, p)).collect(),
        notes: n.source_notes.clone(),
        courses,
    })
}
/// Quote the realized civilian participation target change, including its caps.
pub fn welfare_participation_change(w: &WorldState, id: NationId, gap: f64) -> f64 {
    if !active(w) {
        return -crate::ministries::pensions_jobs(gap);
    }
    let focus = if w
        .population_system
        .nations
        .get(&id)
        .is_some_and(|p| p.policy == Policy::BackToWork)
    {
        0.035
    } else {
        0.0
    };
    let total = sum_rows(w, id, |p| p.working_age_m());
    if total <= 1e-12 {
        return 0.0;
    }
    sum_rows(w, id, |p| {
        let base = p.participation_reference + focus;
        ((base - crate::ministries::pensions_jobs(gap)).clamp(0.25, 0.95) - base.clamp(0.25, 0.95))
            * p.working_age_m()
    }) / total
}
/// Settlement hook for residents with no mapped province. The established
/// annexation/succession caller chooses the recipient, never a guessed neighbor.
pub fn transfer_unallocated(w: &mut WorldState, from: NationId, to: NationId) {
    if !w.population_system.enabled || from == to {
        return;
    }
    if let Some(mut moved) = w.population_system.unallocated.remove(&from) {
        moved.last_owner = to;
        if let Some(existing) = w.population_system.unallocated.get_mut(&to) {
            add_people(existing, &moved);
            for s in 0..8 {
                for g in 0..3 {
                    existing.jobs_reference[s][g] += moved.jobs_reference[s][g];
                    existing.jobs[s][g] += moved.jobs[s][g];
                }
            }
            existing.reference_population_m += moved.reference_population_m;
        } else {
            w.population_system.unallocated.insert(to, moved);
        }
    }
}
/// Autonomous countries use the same priced command and cooldown as the player.
/// Annual review gives each focus time to work; no RNG or free policy changes.
pub fn ai_tick(w: &mut WorldState) {
    if !active(w) || w.month != 1 || w.day != 1 {
        return;
    }
    let ids: Vec<_> = w
        .nations
        .iter()
        .filter(|n| n.alive && Some(n.id) != w.player)
        .map(|n| n.id)
        .collect();
    for id in ids {
        let unemployment = unemployment(w, id).unwrap_or(0.0);
        let labor = sum_rows(w, id, |p| p.labor_force_m);
        let technical = sum_rows(w, id, |p| {
            (0..8)
                .map(|s| (p.jobs[s][1] + p.project_jobs[s][1] - p.filled[s][1]).max(0.0))
                .sum()
        });
        let professional = sum_rows(w, id, |p| {
            (0..8)
                .map(|s| (p.jobs[s][2] + p.project_jobs[s][2] - p.filled[s][2]).max(0.0))
                .sum()
        });
        let policy = if ratio(technical, labor) > 0.015 {
            Policy::TradeSchools
        } else if ratio(professional, labor) > 0.015 {
            Policy::Universities
        } else if unemployment > 0.10 {
            Policy::BackToWork
        } else {
            Policy::Balanced
        };
        if policy_quote(w, id, policy).available {
            let _ = crate::apply_command(
                w,
                &crate::Command::SetPopulationPolicy { nation: id, policy },
            );
        }
    }
}
/// Territorial settlement refreshes membership and totals immediately. It never
/// ages residents, enrolls students, pays GDP or interprets transfer as growth.
pub fn reconcile_ownership(w: &mut WorldState) {
    if !w.population_system.enabled {
        return;
    }
    ensure(w);
    for (d, p) in &mut w.population_system.provinces {
        if let Some(&owner) = w.districts.get(d) {
            p.last_owner = owner;
            match_jobs(p);
        }
    }
    write_populations(w);
    let ids: Vec<_> = w.nations.iter().filter(|n| n.alive).map(|n| n.id).collect();
    for id in ids {
        let employed = sum_rows(w, id, |p| p.inherited_filled_m);
        if let Some(row) = w.population_system.nations.get_mut(&id) {
            row.last_inherited_employed_m = employed;
            row.labor_growth = 0.0;
        }
    }
    publish_outcomes(w);
}
/// New facilities need every required qualification. A sector average cannot
/// let abundant general workers stand in for absent engineers or researchers.
/// Inherited output uses its separate frozen baseline; this gate is physical.
pub fn district_skill_staffing(w: &WorldState, d: &str, sector: usize, recipe: [f64; 3]) -> f64 {
    if !active(w) || sector >= 8 {
        return 1.0;
    }
    let Some(p) = w.population_system.provinces.get(d) else {
        return 1.0;
    };
    let mut result: f64 = 1.0;
    for (g, &required) in recipe.iter().enumerate() {
        if required > 0.0 {
            result = result.min(
                ratio(
                    p.filled[sector][g],
                    p.jobs[sector][g] + p.project_jobs[sector][g],
                )
                .min(1.0),
            );
        }
    }
    result
}
