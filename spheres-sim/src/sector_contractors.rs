//! Fictional, opt-in specialist contractors. All names, logos and coefficients
//! are game content, not claims about historical businesses. Companies improve
//! work that another ledger actually funds and delivers; they never award GDP.
use crate::{
    clock, districts, industry,
    production::{self, ProjectKind as K},
    tech,
    world::{NationId, WorldState},
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum CompanySector {
    Construction,
    Mining,
    Manufacturing,
    Energy,
    Logistics,
    Research,
    Defense,
}
pub const SECTORS: [CompanySector; 7] = [
    CompanySector::Construction,
    CompanySector::Mining,
    CompanySector::Manufacturing,
    CompanySector::Energy,
    CompanySector::Logistics,
    CompanySector::Research,
    CompanySector::Defense,
];
impl CompanySector {
    pub fn index(self) -> usize {
        self as usize
    }
    pub fn key(self) -> &'static str {
        match self {
            Self::Construction => "construction",
            Self::Mining => "mining",
            Self::Manufacturing => "manufacturing",
            Self::Energy => "energy",
            Self::Logistics => "logistics",
            Self::Research => "research",
            Self::Defense => "defense",
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Self::Construction => "Construction",
            Self::Mining => "Mining",
            Self::Manufacturing => "Manufacturing",
            Self::Energy => "Energy",
            Self::Logistics => "Logistics",
            Self::Research => "Research",
            Self::Defense => "Defense",
        }
    }
    pub fn parse(key: &str) -> Option<Self> {
        SECTORS.into_iter().find(|s| s.key() == key)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum CompanyTarget {
    Construction {
        project: u32,
    },
    Facility {
        district: String,
        sector: CompanySector,
    },
    Research {
        domain: String,
    },
    /// The legacy procurement line namespace.
    Equipment {
        project: u32,
    },
    /// Versioned equipment development/production uses a separate namespace.
    CustomEquipment {
        project: u32,
    },
}
impl CompanyTarget {
    pub fn sector(&self) -> CompanySector {
        match self {
            Self::Construction { .. } => CompanySector::Construction,
            Self::Facility { sector, .. } => *sector,
            Self::Research { .. } => CompanySector::Research,
            Self::Equipment { .. } | Self::CustomEquipment { .. } => CompanySector::Defense,
        }
    }
    fn canonical(self) -> Self {
        match self {
            Self::Research { domain } => Self::Research {
                domain: tech::Domain::parse(&domain)
                    .map(|d| format!("{d:?}"))
                    .unwrap_or(domain),
            },
            other => other,
        }
    }
}

/// The UI draws a small custom SVG from this immutable identity. Seed is kept
/// within JavaScript's exact integer range, and distinct for every company.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CompanyLogo {
    pub seed: u64,
    pub mark: u8,
    pub palette: u8,
    pub initials: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Company {
    pub id: u32,
    pub nation: NationId,
    pub name: String,
    pub sector: CompanySector,
    pub specialty: String,
    pub description: String,
    pub strength: String,
    pub weakness: String,
    pub logo: CompanyLogo,
    pub founded_day: i32,
    pub home_district: String,
    pub experience: f64,
    pub capacity: u8,
    /// Positive speed gains improve with experience; slow specialist penalties do not.
    pub work_bonus: f64,
    /// Negative savings mean a real material-intensity tradeoff.
    pub input_saving: f64,
    /// Fee is a share of the real service's base cash, never a flat daily tax.
    pub fee_rate: f64,
    pub total_work: f64,
    pub total_bonus: f64,
    pub total_fees_bn: f64,
    #[serde(default)]
    pub experience_day: Option<i32>,
    #[serde(default)]
    pub experience_today: f64,
}
impl Company {
    pub fn level(&self) -> u8 {
        if self.experience >= 540.0 {
            2
        } else if self.experience >= 180.0 {
            1
        } else {
            0
        }
    }
    pub fn level_name(&self) -> &'static str {
        ["Established", "Experienced", "Leading"][self.level() as usize]
    }
    pub fn effective_capacity(&self) -> u8 {
        self.capacity.saturating_add(self.level())
    }
    pub fn modifiers(&self) -> CompanyModifiers {
        let experience = 1.0 + self.level() as f64 * 0.25;
        // Save files are editable. Invalid contractor coefficients must not
        // create negative recipes or divide the physical ledgers by zero.
        let bounded = |value: f64, neutral: f64, min: f64, max: f64| {
            if value.is_finite() {
                value.clamp(min, max)
            } else {
                neutral
            }
        };
        CompanyModifiers {
            company_id: Some(self.id),
            work_rate: bounded(
                1.0 + self.work_bonus
                    * if self.work_bonus > 0.0 {
                        experience
                    } else {
                        1.0
                    },
                1.0,
                0.25,
                2.0,
            ),
            input_rate: bounded(
                1.0 - self.input_saving
                    * if self.input_saving > 0.0 {
                        experience
                    } else {
                        1.0
                    },
                1.0,
                0.5,
                1.5,
            ),
            fee_rate: bounded(self.fee_rate, 0.0, 0.0, 1.0),
        }
    }
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CompanyModifiers {
    pub company_id: Option<u32>,
    pub work_rate: f64,
    pub input_rate: f64,
    pub fee_rate: f64,
}
impl Default for CompanyModifiers {
    fn default() -> Self {
        Self {
            company_id: None,
            work_rate: 1.0,
            input_rate: 1.0,
            fee_rate: 0.0,
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CompanyAssignment {
    pub company_id: u32,
    pub nation: NationId,
    pub target: CompanyTarget,
    pub assigned_day: i32,
    pub last_day: Option<i32>,
    pub work_today: f64,
    pub bonus_today: f64,
    pub fees_today_bn: f64,
    pub total_work: f64,
    pub total_bonus: f64,
    pub total_fees_bn: f64,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct GrowthSample {
    pub month: i32,
    pub gdp: f64,
    pub activity: [f64; 7],
}
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CompanyGrowth {
    pub nation: Option<NationId>,
    pub samples: Vec<GrowthSample>,
    pub month_activity: [f64; 7],
    pub last_entry_month: Option<i32>,
    pub entrants: u32,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CompanyNews {
    pub nation: NationId,
    pub company_id: u32,
    pub day: i32,
    pub message: String,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, deny_unknown_fields)]
pub struct Companies {
    pub enabled: bool,
    pub roster: Vec<Company>,
    pub assignments: Vec<CompanyAssignment>,
    pub growth: BTreeMap<NationId, CompanyGrowth>,
    pub news: Vec<CompanyNews>,
    pub next_id: u32,
    pub last_day: Option<i32>,
    pub last_month: Option<i32>,
}
impl Companies {
    pub fn is_empty(&self) -> bool {
        !self.enabled
            && self.roster.is_empty()
            && self.assignments.is_empty()
            && self.growth.is_empty()
            && self.news.is_empty()
            && self.next_id == 0
            && self.last_day.is_none()
            && self.last_month.is_none()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CompanyTargetOption {
    pub target: CompanyTarget,
    pub label: String,
    pub sector: CompanySector,
    pub current_company_id: Option<u32>,
}
pub fn used_capacity(w: &WorldState, company_id: u32) -> usize {
    w.sector_contractors
        .assignments
        .iter()
        .filter(|a| a.company_id == company_id && target_valid(w, a.nation, &a.target))
        .count()
}
pub fn companies_for(w: &WorldState, nation: NationId) -> impl Iterator<Item = &Company> {
    w.sector_contractors
        .roster
        .iter()
        .filter(move |c| c.nation == nation)
}

fn installed(w: &WorldState, district: &str, sector: CompanySector) -> bool {
    match sector {
        CompanySector::Manufacturing => {
            [K::ProcessingPlant, K::MachineryWorks, K::StarterIndustry]
                .into_iter()
                .any(|k| production::level(w, district, k) > 0)
                || (w.rules.industry_rebuild && production::level(w, district, K::AdvancedIndustry) > 0)
                || w.production
                    .industry
                    .modules
                    .get(district)
                    .copied()
                    .unwrap_or(0)
                    > 0
                || crate::materials::capacity_daily(w, district) > 0.0
        }
        CompanySector::Energy => {
            industry::site_level(w, district, K::Generation) > 0
                || w.production
                    .industry
                    .modules
                    .get(district)
                    .copied()
                    .unwrap_or(0)
                    > 0
        }
        CompanySector::Logistics => {
            industry::site_level(w, district, K::FreightTerminal) > 0
                && crate::logistics::has_terminal(district)
        }
        CompanySector::Mining => w
            .resources
            .mines
            .iter()
            .any(|m| m.district == district && m.commodity != crate::resources::Commodity::Oil),
        _ => false,
    }
}
fn target_valid(w: &WorldState, nation: NationId, target: &CompanyTarget) -> bool {
    if !w.nation_opt(nation).is_some_and(|n| n.alive) {
        return false;
    }
    match target {
        CompanyTarget::Construction { project } => w.production.projects.iter().any(|p| {
            p.id == *project && p.nation == nation && w.districts.get(&p.district) == Some(&nation)
                && p.progress_days + 1e-9 < p.total_days as f64
        }),
        CompanyTarget::Facility { district, sector } => {
            w.districts.get(district) == Some(&nation) && installed(w, district, *sector)
        }
        CompanyTarget::Research { domain } => {
            tech::Domain::parse(domain).is_some()
                && w.production.provinces.iter().any(|p| {
                    p.research_centers > 0 && w.districts.get(&p.district) == Some(&nation)
                })
        }
        CompanyTarget::Equipment { project } => w.manufacturing.lines.iter().any(|l| {
            l.id == *project && l.nation == nation && w.districts.get(&l.district) == Some(&nation)
        }),
        CompanyTarget::CustomEquipment { project } => {
            w.nation(nation).equipment.as_ref().is_some_and(|e| {
                e.projects.iter().any(|p| {
                    p.id == *project
                        && !matches!(
                            p.status,
                            crate::equipment::ProjectStatus::Complete
                                | crate::equipment::ProjectStatus::Cancelled
                        )
                        && p.district
                            .as_ref()
                            .is_none_or(|d| w.districts.get(d) == Some(&nation))
                })
            })
        }
    }
}
pub fn targets(w: &WorldState, nation: NationId) -> Vec<CompanyTargetOption> {
    if !w.nation_opt(nation).is_some_and(|n| n.alive) {
        return vec![];
    }
    let mut options = vec![];
    let mut push = |target: CompanyTarget, label: String| {
        let current_company_id = w
            .sector_contractors
            .assignments
            .iter()
            .find(|a| a.nation == nation && a.target == target)
            .map(|a| a.company_id);
        options.push(CompanyTargetOption {
            sector: target.sector(),
            target,
            label,
            current_company_id,
        });
    };
    for p in w
        .production
        .projects
        .iter()
        .filter(|p| p.nation == nation && w.districts.get(&p.district) == Some(&nation)
            && p.progress_days + 1e-9 < p.total_days as f64)
    {
        push(
            CompanyTarget::Construction { project: p.id },
            format!(
                "{} — {} #{}",
                districts::name_of(&p.district).unwrap_or(&p.district),
                p.kind.key().replace('_', " "),
                p.id
            ),
        );
    }
    for (d, owner) in &w.districts {
        if *owner != nation {
            continue;
        }
        for sector in [
            CompanySector::Mining,
            CompanySector::Manufacturing,
            CompanySector::Energy,
            CompanySector::Logistics,
        ] {
            if installed(w, d, sector) {
                push(
                    CompanyTarget::Facility {
                        district: d.clone(),
                        sector,
                    },
                    format!("{} — {}", districts::name_of(d).unwrap_or(d), sector.name()),
                );
            }
        }
    }
    for domain in tech::DOMAINS {
        let target = CompanyTarget::Research {
            domain: format!("{domain:?}"),
        };
        if target_valid(w, nation, &target) {
            push(target, format!("{} research programme", domain.name()));
        }
    }
    for l in w
        .manufacturing
        .lines
        .iter()
        .filter(|l| l.nation == nation && w.districts.get(&l.district) == Some(&nation))
    {
        push(
            CompanyTarget::Equipment { project: l.id },
            format!(
                "{} — {} line #{}",
                districts::name_of(&l.district).unwrap_or(&l.district),
                l.kit,
                l.id
            ),
        );
    }
    if let Some(e) = w.nation(nation).equipment.as_ref() {
        for p in &e.projects {
            let target = CompanyTarget::CustomEquipment { project: p.id };
            if target_valid(w, nation, &target) {
                push(
                    target,
                    format!("{} — equipment project #{}", p.revision_id, p.id),
                );
            }
        }
    }
    options
}

/// Read-only projection shared by the directory and command target selector.
/// It cannot enroll companies, spend cash, advance time, or grant experience.
pub fn snapshot(w: &WorldState, nation: NationId) -> serde_json::Value {
    use serde_json::json;
    let options = targets(w, nation);
    let assignments: Vec<_> = w
        .sector_contractors
        .assignments
        .iter()
        .filter(|a| a.nation == nation && target_valid(w, nation, &a.target))
        .map(|a| {
            let mut value = serde_json::to_value(a).expect("serializable assignment");
            value["label"] = json!(options
                .iter()
                .find(|o| o.target == a.target)
                .map(|o| o.label.clone())
                .unwrap_or_else(|| a.target.sector().name().into()));
            value
        })
        .collect();
    let rows: Vec<_> = companies_for(w, nation)
        .map(|c| {
            let mut row = serde_json::to_value(c).expect("serializable company");
            let m = c.modifiers();
            let (year, month, day) = clock::date_from_day(c.founded_day);
            row["level"] = json!(c.level_name());
            row["level_rank"] = json!(c.level());
            row["effective_capacity"] = json!(c.effective_capacity());
            row["used_capacity"] = json!(used_capacity(w, c.id));
            row["effective_work_bonus"] = json!(m.work_rate - 1.0);
            row["effective_input_saving"] = json!(1.0 - m.input_rate);
            row["work_rate"] = json!(m.work_rate);
            row["input_rate"] = json!(m.input_rate);
            row["sector_label"] = json!(c.sector.name());
            row["logo_seed"] = json!(c.logo.seed);
            row["home_name"] = json!(districts::name_of(&c.home_district).unwrap_or(nation.name()));
            row["founded_label"] = json!(format!("{year:04}-{month:02}-{day:02}"));
            row["origin"] = json!(&c.description);
            row["strength"] = json!(if m.input_rate < 1.0 && m.work_rate < 1.0 {
                format!("{:.1}% fewer material inputs", (1.0 - m.input_rate) * 100.0)
            } else if m.input_rate < 1.0 {
                format!(
                    "+{:.1}% work rate; {:.1}% fewer material inputs",
                    (m.work_rate - 1.0) * 100.0,
                    (1.0 - m.input_rate) * 100.0
                )
            } else {
                format!("+{:.1}% work rate", (m.work_rate - 1.0) * 100.0)
            });
            row["weakness"] = json!(if m.work_rate < 1.0 {
                format!(
                    "{:.1}% slower work; {:.1}% service fee",
                    (1.0 - m.work_rate) * 100.0,
                    m.fee_rate * 100.0
                )
            } else if m.input_rate > 1.0 {
                format!(
                    "{:.1}% more material inputs; {:.1}% service fee",
                    (m.input_rate - 1.0) * 100.0,
                    m.fee_rate * 100.0
                )
            } else {
                format!(
                    "{:.1}% service fee; {} simultaneous contract{}",
                    m.fee_rate * 100.0,
                    c.effective_capacity(),
                    if c.effective_capacity() == 1 { "" } else { "s" }
                )
            });
            row["assignments"] = json!(assignments
                .iter()
                .filter(|a| a["company_id"].as_u64() == Some(c.id as u64))
                .collect::<Vec<_>>());
            row["targets"] = json!(options
                .iter()
                .filter(|o| o.sector == c.sector)
                .collect::<Vec<_>>());
            row
        })
        .collect();
    let growth=w.sector_contractors.growth.get(&nation).map(|g| {
        let growth=g.samples.first().zip(g.samples.last()).map(|(first,last)|last.gdp/first.gdp.max(0.01)-1.0).unwrap_or(0.0);
        let positive=g.samples.windows(2).filter(|s|s[1].gdp>s[0].gdp*1.001).count();
        json!({"months_observed":g.samples.len().saturating_sub(1),"required_months":18,"growing_months":positive,
            "cumulative_growth":growth,"required_growth":0.12,"entrants":g.entrants,
            "cooldown_months_remaining":g.last_entry_month.map_or(0,|m|(18-(clock::month_index(w)-m)).max(0)),
            "eligible_sector":growing_sector(g),"samples":g.samples})
    });
    json!({"enabled":w.sector_contractors.enabled,"nation":nation,"name":nation.name(),"day":clock::absolute_day(w),
        "companies":rows,"targets":options,"growth":growth,
        "assignments":assignments,
        "news":w.sector_contractors.news.iter().filter(|n|n.nation==nation).collect::<Vec<_>>()})
}

pub fn assign(
    w: &mut WorldState,
    nation: NationId,
    company_id: u32,
    target: CompanyTarget,
) -> Result<(), String> {
    let target = target.canonical();
    assignment_refusal(w, nation, company_id, &target)?;
    if w.sector_contractors.assignments.iter().any(|a|
        a.nation == nation && a.target == target && a.company_id == company_id) { return Ok(()); }
    // Replacing the lead contractor is atomic. No fees or XP are granted by signing.
    w.sector_contractors.assignments.retain(|a| !(a.nation == nation && a.target == target));
    w.sector_contractors.assignments.push(CompanyAssignment {
        company_id, nation, target, assigned_day: clock::absolute_day(w), last_day: None,
        work_today: 0.0, bonus_today: 0.0, fees_today_bn: 0.0,
        total_work: 0.0, total_bonus: 0.0, total_fees_bn: 0.0,
    });
    Ok(())
}

fn assignment_refusal(w: &WorldState, nation: NationId, company_id: u32, target: &CompanyTarget) -> Result<(), String> {
    if !w.sector_contractors.enabled {
        return Err("Enable companies before assigning a contractor.".into());
    }
    if !target_valid(w, nation, target) {
        return Err("That assignment is unavailable or is not owned by this country.".into());
    }
    let company = w
        .sector_contractors
        .roster
        .iter()
        .find(|c| c.id == company_id && c.nation == nation)
        .ok_or("Choose a domestic company from this country's directory.")?;
    if company.sector != target.sector() {
        return Err("This company does not work in that sector.".into());
    }
    if w.sector_contractors
        .assignments
        .iter()
        .any(|a| a.nation == nation && &a.target == target && a.company_id == company_id)
    {
        return Ok(());
    }
    if used_capacity(w, company_id) >= company.effective_capacity() as usize {
        return Err("This company has no free contract capacity.".into());
    }
    Ok(())
}
pub fn unassign(
    w: &mut WorldState,
    nation: NationId,
    target: &CompanyTarget,
) -> Result<(), String> {
    let target = target.clone().canonical();
    let before = w.sector_contractors.assignments.len();
    w.sector_contractors
        .assignments
        .retain(|a| !(a.nation == nation && a.target == target));
    if before == w.sector_contractors.assignments.len() {
        return Err("No company is assigned to that work.".into());
    }
    Ok(())
}
pub fn modifiers(w: &WorldState, nation: NationId, target: &CompanyTarget) -> CompanyModifiers {
    if !w.sector_contractors.enabled {
        return CompanyModifiers::default();
    }
    // Most production reads have no contractor. Find the sparse assignment
    // before checking inherited capacity, site ownership or research facilities.
    let Some(assignment) = w
        .sector_contractors
        .assignments
        .iter()
        .find(|a| a.nation == nation && &a.target == target)
    else {
        return CompanyModifiers::default();
    };
    if !target_valid(w, nation, target) {
        return CompanyModifiers::default();
    }
    w.sector_contractors
        .roster
        .iter()
        .find(|c| {
            c.id == assignment.company_id && c.nation == nation && c.sector == target.sector()
        })
        .map(Company::modifiers)
        .unwrap_or_default()
}

fn finite_nonnegative(v: f64) -> f64 {
    if v.is_finite() {
        v.max(0.0)
    } else {
        0.0
    }
}
fn normalized_work(sector: CompanySector, work: f64) -> f64 {
    let unit = match sector {
        CompanySector::Research => 0.001,
        CompanySector::Defense => 0.0001,
        CompanySector::Mining => 100.0,
        _ => 1.0,
    };
    finite_nonnegative(work) / unit
}
/// Called once for each real operating receipt, including activity without a
/// contractor. Normalized values only compare a sector with its own history.
pub fn record_sector_activity(
    w: &mut WorldState,
    nation: NationId,
    sector: CompanySector,
    work: f64,
) {
    if !w.sector_contractors.enabled || !work.is_finite() || work <= 0.0 {
        return;
    }
    if let Some(g) = w.sector_contractors.growth.get_mut(&nation) {
        g.month_activity[sector.index()] += normalized_work(sector, work).min(1e12);
    }
}
/// Record delivered work after the existing owner settles inputs and cash.
/// Does not itself spend, mint production, or record sector activity a second
/// time. At most one productive-day XP unit per company prevents order farming.
pub fn record_work(
    w: &mut WorldState,
    nation: NationId,
    target: &CompanyTarget,
    base_work: f64,
    bonus_work: f64,
    fee_bn: f64,
) {
    if !w.sector_contractors.enabled
        || !base_work.is_finite()
        || base_work <= 0.0
        || !bonus_work.is_finite()
        || !fee_bn.is_finite()
    {
        return;
    }
    let today = clock::absolute_day(w);
    let Some(a) = w
        .sector_contractors
        .assignments
        .iter_mut()
        .find(|a| a.nation == nation && &a.target == target)
    else {
        return;
    };
    let Some(c) = w
        .sector_contractors
        .roster
        .iter_mut()
        .find(|c| c.id == a.company_id && c.nation == nation)
    else {
        return;
    };
    if a.last_day != Some(today) {
        a.last_day = Some(today);
        a.work_today = 0.0;
        a.bonus_today = 0.0;
        a.fees_today_bn = 0.0;
    }
    let fee = fee_bn.max(0.0);
    a.work_today += base_work;
    a.bonus_today += bonus_work;
    a.fees_today_bn += fee;
    a.total_work += base_work;
    a.total_bonus += bonus_work;
    a.total_fees_bn += fee;
    c.total_work += base_work;
    c.total_bonus += bonus_work;
    c.total_fees_bn += fee;
    if c.experience_day != Some(today) {
        c.experience_day = Some(today);
        c.experience_today = 0.0;
    }
    let gain = normalized_work(c.sector, base_work).min((1.0 - c.experience_today).max(0.0));
    c.experience_today += gain;
    c.experience += gain;
}

fn sector_weights(w: &WorldState, nation: NationId) -> [f64; 7] {
    let shares = crate::sector_profiles::data()
        .get(&nation)
        .map(|p| p.shares)
        .unwrap_or(crate::province_economy::MODEL_SECTOR_SHARES);
    let n = w.nation(nation);
    [
        shares[4],
        shares[1],
        shares[2],
        shares[3],
        shares[5],
        shares[6] * 0.12,
        (shares[2] * 0.15 + n.mil_spend_gdp * 0.4).max(0.004),
    ]
}
/// Land area is deliberately absent. Broad GDP and the existing sector profile
/// set starting breadth; specialization can earn additional firms later.
fn starting_sectors(w: &WorldState, nation: NationId) -> Vec<CompanySector> {
    let gdp = finite_nonnegative(w.nation(nation).gdp);
    let weights = sector_weights(w, nation);
    if gdp >= 2000.0 {
        return SECTORS
            .into_iter()
            .flat_map(|s| {
                let sector_gdp = gdp * weights[s.index()];
                let count = if sector_gdp >= 600.0 {
                    5
                } else if sector_gdp >= 200.0 {
                    4
                } else {
                    3
                };
                std::iter::repeat_n(s, count)
            })
            .collect();
    }
    let count = if gdp >= 500.0 {
        16
    } else if gdp >= 100.0 {
        10
    } else if gdp >= 25.0 {
        6
    } else if gdp >= 2.0 {
        3
    } else {
        2
    };
    let mut ranked = SECTORS.to_vec();
    ranked.sort_by(|a, b| {
        weights[b.index()]
            .total_cmp(&weights[a.index()])
            .then(a.cmp(b))
    });
    let mut out = vec![CompanySector::Construction];
    // Botswana's compact starting ecosystem is an authored game preset, not a
    // historical company count. Names/strengths below are explicitly fictional.
    if nation.code() == "Botswana" {
        return vec![
            CompanySector::Construction,
            CompanySector::Mining,
            CompanySector::Logistics,
        ];
    }
    for sector in ranked.iter().copied().cycle() {
        if out.len() >= count {
            break;
        }
        if out.len() < ranked.len() && out.contains(&sector) {
            continue;
        }
        out.push(sector);
    }
    out
}
fn mix(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9e3779b97f4a7c15);
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
    x ^ (x >> 31)
}
const US_NAMES: [[&str; 5]; 7] = [
    [
        "Continental Works",
        "Prairie Construction",
        "Summit Engineering",
        "Ironspan Builders",
        "Redwood Civic",
    ],
    [
        "Copperhead Resources",
        "Blue Mesa Minerals",
        "Deepwell Extraction",
        "Granite Star Mining",
        "Cinder Peak Metals",
    ],
    [
        "Forgepoint Industries",
        "Lakefront Precision",
        "Unioncrest Machinery",
        "Mercury Toolworks",
        "Foundry Nine",
    ],
    [
        "Northstar Grid",
        "Sunward Generation",
        "Cobalt Current",
        "Sequoia Power",
        "Clearwater Turbines",
    ],
    [
        "Meridian Freight",
        "Crossrail Logistics",
        "Atlas Gate Cargo",
        "Harborline Transport",
        "Peregrine Dispatch",
    ],
    [
        "Aster Research",
        "Lumen Laboratories",
        "Cedarbridge Science",
        "Helix Harbor",
        "Brightwell Systems",
    ],
    [
        "Sentinel Dynamics",
        "Falcon Ridge Systems",
        "Ironclad Ordnance",
        "Bastion Aeroworks",
        "Argent Defense",
    ],
];
const ROOTS: [&str; 24] = [
    "Aster",
    "Copperleaf",
    "Northlight",
    "Stonewing",
    "Silverwell",
    "Suncrest",
    "Meridian",
    "Clearforge",
    "Ironwood",
    "Amberline",
    "Bluehaven",
    "Cedarpoint",
    "Lumen",
    "Goldfinch",
    "Riverstone",
    "Swiftpeak",
    "Alder",
    "Brightwater",
    "Copperwind",
    "Harborcrest",
    "Peregrine",
    "Redstone",
    "Willowspan",
    "Starling",
];
const SUFFIXES: [[&str; 5]; 7] = [
    ["Works", "Construction", "Engineering", "Builders", "Civic"],
    ["Resources", "Minerals", "Extraction", "Mining", "Metals"],
    [
        "Industries",
        "Precision",
        "Machinery",
        "Toolworks",
        "Manufacturing",
    ],
    ["Grid", "Generation", "Current", "Power", "Turbines"],
    ["Freight", "Logistics", "Cargo", "Transport", "Dispatch"],
    [
        "Research",
        "Laboratories",
        "Science",
        "Innovations",
        "Systems",
    ],
    [
        "Dynamics",
        "Defense Systems",
        "Ordnance",
        "Aeroworks",
        "Defense",
    ],
];
fn make_company(
    w: &WorldState,
    nation: NationId,
    sector: CompanySector,
    ordinal: usize,
    id: u32,
    entrant: bool,
) -> Company {
    let seed = mix(id as u64 ^ ((nation.index() as u64) << 32));
    let districts: Vec<_> = w
        .districts
        .iter()
        .filter(|(_, n)| **n == nation)
        .map(|(d, _)| d.as_str())
        .collect();
    let home = districts
        .get(seed as usize % districts.len().max(1))
        .copied()
        .unwrap_or("");
    let home_name = districts::name_of(home).unwrap_or(nation.name());
    let profile = ordinal % 5;
    let mut name = if nation.code() == "USA" && ordinal < 5 {
        US_NAMES[sector.index()][ordinal].to_owned()
    } else if nation.code() == "Botswana" && ordinal == 0 {
        match sector {
            CompanySector::Construction => "Kgale Stoneworks",
            CompanySector::Mining => "Moruwa Minerals",
            CompanySector::Logistics => "Kalahari Freight",
            _ => "",
        }
        .to_owned()
    } else {
        format!(
            "{} {} {}",
            home_name,
            ROOTS[(seed as usize) % ROOTS.len()],
            SUFFIXES[sector.index()][profile]
        )
    };
    if name.is_empty() {
        name = format!(
            "{} {} {}",
            home_name,
            ROOTS[(seed as usize) % ROOTS.len()],
            SUFFIXES[sector.index()][profile]
        );
    }
    if w.sector_contractors.roster.iter().any(|c| c.name == name) {
        name = format!(
            "{} {} {}",
            name,
            ROOTS[((seed >> 16) as usize) % ROOTS.len()],
            id
        );
    }
    // Numeric variation supplies different bids; the explicit collision check
    // below guarantees a new gameplay profile as the sector keeps growing.
    let variation = (seed % 17) as f64 * 0.001;
    let efficient = matches!(
        sector,
        CompanySector::Manufacturing | CompanySector::Energy | CompanySector::Defense
    );
    let (mut work_bonus, input_saving, fee_rate, capacity, trait_name) = match profile {
        0 => (
            0.14 + variation,
            if efficient { -0.03 } else { 0.0 },
            0.065 + (seed % 7) as f64 * 0.001,
            2,
            "Rapid delivery",
        ),
        1 if efficient => (
            -0.03,
            0.07 + variation,
            0.018 + (seed % 7) as f64 * 0.001,
            1,
            "Resource discipline",
        ),
        1 => (
            0.07 + variation,
            0.0,
            0.015 + (seed % 7) as f64 * 0.001,
            1,
            "Focused service",
        ),
        2 => (
            0.075 + variation,
            0.0,
            0.032 + (seed % 7) as f64 * 0.001,
            4,
            "Large contracts",
        ),
        3 => (
            0.105 + variation,
            if efficient { 0.025 } else { 0.0 },
            0.044 + (seed % 7) as f64 * 0.001,
            1,
            "Precision teams",
        ),
        _ => (
            0.05 + variation,
            if efficient { 0.025 } else { 0.0 },
            0.014 + (seed % 7) as f64 * 0.001,
            2,
            "Steady operations",
        ),
    };
    let capacity = if entrant { capacity.min(2) } else { capacity };
    while w.sector_contractors.roster.iter().any(|c| {
        c.nation == nation
            && c.sector == sector
            && c.work_bonus == work_bonus
            && c.input_saving == input_saving
            && c.fee_rate == fee_rate
            && c.capacity == capacity
    }) {
        work_bonus += if work_bonus < 0.0 { -0.001 } else { 0.001 };
    }
    let strength = if input_saving > 0.0 && work_bonus < 0.0 {
        format!("{:.1}% fewer material inputs", input_saving * 100.0)
    } else if input_saving > 0.0 {
        format!(
            "+{:.1}% work rate; {:.1}% fewer material inputs",
            work_bonus * 100.0,
            input_saving * 100.0
        )
    } else {
        format!("+{:.1}% work rate", work_bonus * 100.0)
    };
    let weakness = if work_bonus < 0.0 {
        format!(
            "{:.1}% slower work; {:.1}% service fee",
            -work_bonus * 100.0,
            fee_rate * 100.0
        )
    } else if input_saving < 0.0 {
        format!(
            "{:.1}% more material inputs; {:.1}% service fee",
            -input_saving * 100.0,
            fee_rate * 100.0
        )
    } else {
        format!(
            "{:.1}% service fee; {} simultaneous contract{}",
            fee_rate * 100.0,
            capacity,
            if capacity == 1 { "" } else { "s" }
        )
    };
    let initials = name
        .split_whitespace()
        .take(2)
        .filter_map(|s| s.chars().next())
        .collect();
    let founded_day = clock::absolute_day(w);
    Company {
        id,
        nation,
        name,
        sector,
        specialty: format!("{} · {}", sector.name(), trait_name),
        description: if entrant {
            format!("Founded in {home_name} after sustained economic expansion supported new {} work. {} is its signature specialty.",sector.key(),trait_name.to_lowercase())
        } else {
            format!(
                "An established fictional {} specialist based in {home_name}, known for {}.",
                sector.key(),
                trait_name.to_lowercase()
            )
        },
        strength,
        weakness,
        logo: CompanyLogo {
            seed: ((id as u64) << 20) | (seed & 0xfffff),
            mark: (seed % 12) as u8,
            palette: ((seed >> 8) % 12) as u8,
            initials,
        },
        founded_day,
        home_district: home.into(),
        experience: 0.0,
        capacity,
        work_bonus,
        input_saving,
        fee_rate,
        total_work: 0.0,
        total_bonus: 0.0,
        total_fees_bn: 0.0,
        experience_day: None,
        experience_today: 0.0,
    }
}
fn add_initial_nation(w: &mut WorldState, nation: NationId) {
    let sectors = starting_sectors(w, nation);
    let mut counts = [0usize; 7];
    for sector in sectors {
        w.sector_contractors.next_id = w.sector_contractors.next_id.saturating_add(1);
        let company = make_company(
            w,
            nation,
            sector,
            counts[sector.index()],
            w.sector_contractors.next_id,
            false,
        );
        counts[sector.index()] += 1;
        w.sector_contractors.roster.push(company);
    }
    w.sector_contractors.growth.insert(
        nation,
        CompanyGrowth {
            nation: Some(nation),
            samples: vec![GrowthSample {
                month: clock::month_index(w) - 1,
                gdp: finite_nonnegative(w.nation(nation).gdp),
                activity: [0.0; 7],
            }],
            ..Default::default()
        },
    );
}
/// Explicit, idempotent opt-in; never called by the ordinary legacy simulator.
pub fn enable(w: &mut WorldState) {
    if w.sector_contractors.enabled {
        return;
    }
    w.sector_contractors.enabled = true;
    let alive: Vec<_> = w.nations.iter().filter(|n| n.alive).map(|n| n.id).collect();
    for nation in alive {
        add_initial_nation(w, nation);
    }
}

fn growing_sector(g: &CompanyGrowth) -> Option<CompanySector> {
    if g.samples.len() < 19 {
        return None;
    }
    let samples = &g.samples[g.samples.len() - 19..];
    if samples.windows(2).any(|s| s[1].month != s[0].month + 1) {
        return None;
    }
    let first = samples.first()?.gdp.max(0.01);
    let last = samples.last()?.gdp;
    let increases = samples
        .windows(2)
        .filter(|s| s[1].gdp > s[0].gdp * 1.001)
        .count();
    // At least 12 growing months and 12% cumulative GDP expansion. A land
    // transfer or a one-month GDP jump cannot pass the sustained-growth gate.
    if last < first * 1.12 || increases < 12 {
        return None;
    }
    let recent = &samples[1..];
    SECTORS
        .into_iter()
        .filter_map(|sector| {
            let i = sector.index();
            if recent.iter().filter(|s| s.activity[i] >= 1.0).count() < 12 {
                return None;
            }
            let early = recent[..6].iter().map(|s| s.activity[i]).sum::<f64>() / 6.0;
            let late = recent[12..].iter().map(|s| s.activity[i]).sum::<f64>() / 6.0;
            // Actual sector work must itself expand; signing idle contracts or
            // merely receiving a national GDP change does not start businesses.
            if late < early.max(1.0) * 1.10 {
                return None;
            }
            Some((sector, late / early.max(1.0)))
        })
        .max_by(|a, b| a.1.total_cmp(&b.1).then_with(|| b.0.cmp(&a.0)))
        .map(|p| p.0)
}

/// Economic Competition opponents review at most two unfilled contracts each
/// month. They use ordinary command validation, retain experienced partners,
/// and never change the player's choices or opt anyone into competition.
fn choose_ai_contractors(w: &mut WorldState) {
    if !w.sector_contractors.enabled || !crate::economic_ai::enabled(w) {
        return;
    }
    let opponents: Vec<_> = w
        .nations
        .iter()
        .filter(|n| n.alive && w.player != Some(n.id))
        .map(|n| n.id)
        .collect();
    for nation in opponents {
        let vacant: Vec<_> = targets(w, nation)
            .into_iter()
            .filter(|o| o.current_company_id.is_none())
            .collect();
        let mut hired = 0;
        for option in vacant {
            if hired >= 2 {
                break;
            }
            let choice = companies_for(w, nation)
                .filter(|c| {
                    c.sector == option.sector
                        && used_capacity(w, c.id) < c.effective_capacity() as usize
                })
                .map(|c| {
                    let m = c.modifiers();
                    (c.id, m.work_rate / ((1.0 + m.fee_rate) * m.input_rate))
                })
                .filter(|(_, score)| *score > 1.0)
                .max_by(|a, b| a.1.total_cmp(&b.1).then_with(|| b.0.cmp(&a.0)))
                .map(|p| p.0);
            if let Some(company) = choice {
                let quote = assignment_quote(w,nation,company,&option.target);
                if !quote.valid { continue; }
                if crate::apply_command(
                    w,
                    &crate::Command::AssignSectorContractor {
                        nation,
                        company,
                        target: option.target,
                        quote: quote.token,
                    },
                )
                .is_ok()
                {
                    hired += 1;
                }
            }
        }
    }
}
pub fn tick_day(w: &mut WorldState) {
    if !w.sector_contractors.enabled {
        return;
    }
    let today = clock::absolute_day(w);
    if w.sector_contractors.last_day == Some(today) {
        return;
    }
    w.sector_contractors.last_day = Some(today);
    // End contracts after completion, cancellation, annexation or dissolution.
    let valid: BTreeSet<_> = w
        .sector_contractors
        .assignments
        .iter()
        .filter(|a| target_valid(w, a.nation, &a.target))
        .map(|a| (a.nation, a.target.clone()))
        .collect();
    w.sector_contractors
        .assignments
        .retain(|a| valid.contains(&(a.nation, a.target.clone())));
    for a in &mut w.sector_contractors.assignments {
        if a.last_day != Some(today) {
            a.last_day = Some(today);
            a.work_today = 0.0;
            a.bonus_today = 0.0;
            a.fees_today_bn = 0.0;
        }
    }
    let month = clock::month_index(w);
    if !clock::month_end(w) || w.sector_contractors.last_month == Some(month) {
        return;
    }
    w.sector_contractors.last_month = Some(month);
    let alive: Vec<_> = w.nations.iter().filter(|n| n.alive).map(|n| n.id).collect();
    for nation in alive {
        if !w.sector_contractors.growth.contains_key(&nation) {
            add_initial_nation(w, nation);
        }
        let gdp = finite_nonnegative(w.nation(nation).gdp);
        let g = w.sector_contractors.growth.get_mut(&nation).unwrap();
        if g.samples.last().is_some_and(|s| s.month == month) {
            continue;
        }
        g.samples.push(GrowthSample {
            month,
            gdp,
            activity: std::mem::take(&mut g.month_activity),
        });
        if g.samples.len() > 19 {
            g.samples.remove(0);
        }
        if g.last_entry_month.is_some_and(|m| month - m < 18) {
            continue;
        }
        let Some(sector) = growing_sector(g) else {
            continue;
        };
        // Reproducible opportunity, isolated from the world's simulation RNG.
        if mix(nation.index() as u64 ^ ((month as u64) << 16) ^ g.entrants as u64) % 100 >= 45 {
            continue;
        }
        g.last_entry_month = Some(month);
        g.entrants += 1;
        let ordinal = w
            .sector_contractors
            .roster
            .iter()
            .filter(|c| c.nation == nation && c.sector == sector)
            .count();
        w.sector_contractors.next_id = w.sector_contractors.next_id.saturating_add(1);
        let company = make_company(w, nation, sector, ordinal, w.sector_contractors.next_id, true);
        let message=format!("{} has opened in {} after 18 months of sustained expansion. A new {} specialist is available.",company.name,districts::name_of(&company.home_district).unwrap_or(nation.name()),sector.key());
        w.headline(format!("COMPANIES: {message}"));
        w.sector_contractors.news.push(CompanyNews {
            nation,
            company_id: company.id,
            day: today,
            message,
        });
        w.sector_contractors.roster.push(company);
    }
    // The directory and full founding records persist; only notification history is bounded.
    if w.sector_contractors.news.len() > 120 {
        w.sector_contractors.news.drain(..w.sector_contractors.news.len() - 120);
    }
    choose_ai_contractors(w);
}


/// A dated review of a service agreement. Tokens cover only the participating
/// identity, its live assignments, the target and relevant rule switches.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ContractorQuote {
    pub valid: bool,
    pub reason: Option<String>,
    pub token: String,
    pub work_rate: f64,
    pub input_rate: f64,
    pub fee_rate: f64,
    pub previous_company_id: Option<u32>,
    pub fee_basis: String,
}

pub fn fee_basis(target: &CompanyTarget) -> &'static str {
    match target {
        CompanyTarget::Construction { .. } => "Fee on new funded construction work, included within the construction budget; no materials, crew or capacity requirement. Previously paid work keeps its price.",
        CompanyTarget::Facility { sector: CompanySector::Mining, .. } => "Fee on the modeled extraction service cost (30% of reference output value), paid from Minerals & processing only when additional ore is delivered. Oil is excluded.",
        CompanyTarget::Facility { sector: CompanySector::Energy, .. } => "Fee on actual dispatched electricity service cost, paid from Energy systems; idle generators charge nothing.",
        CompanyTarget::Facility { sector: CompanySector::Logistics, .. } => "Fee on a modeled $10 per tonne handling basis at each contracted terminal, charged through the government's fiscal account only when cargo moves.",
        CompanyTarget::Facility { .. } => "Fee on actual funded processing or advanced-component operating work; ingredient savings and generator fuel savings have separate owners.",
        CompanyTarget::Research { .. } => "Fee on actual funded prototype work from Basic research; no fee for idle research or merely signing an agreement.",
        CompanyTarget::Equipment { .. } => "Fee is included in the public procurement line's existing budget; the funded output uses the quoted work and input rates. Supplier stock and paid purchases are separate.",
        CompanyTarget::CustomEquipment { .. } => "Fee on newly funded public equipment work only. Existing paid inputs, company-certified products and manufacturer refit agreements keep their terms.",
    }
}

fn reviewed_quote(w: &WorldState, nation: NationId, company_id: Option<u32>, target: &CompanyTarget, release: bool) -> ContractorQuote {
    use serde_json::json;
    let target = target.clone().canonical();
    let previous = w.sector_contractors.assignments.iter().find(|a| a.nation == nation && a.target == target);
    let company = company_id.and_then(|id| w.sector_contractors.roster.iter().find(|c| c.id == id));
    let reason = if release {
        if !w.nation_opt(nation).is_some_and(|n| n.alive) { Some("An active government must release this agreement.".into()) }
        else if previous.is_none() { Some("No company is assigned to that work.".into()) } else { None }
    } else { assignment_refusal(w,nation,company_id.unwrap_or(0),&target).err() };
    let terms = if release { CompanyModifiers::default() } else { company.map(Company::modifiers).unwrap_or_default() };
    let company_assignments: Vec<_> = w.sector_contractors.assignments.iter()
        .filter(|a| Some(a.company_id) == company_id)
        .map(|a| json!({"nation":a.nation,"target":a.target,"assigned_day":a.assigned_day,"active":target_valid(w,a.nation,&a.target)})).collect();
    let target_state = match &target {
        CompanyTarget::Construction{project} => json!(w.production.projects.iter().find(|p|p.id==*project)
            .map(|p|(&p.district,p.nation,p.progress_days,p.total_days,w.districts.get(&p.district)))),
        CompanyTarget::Facility{district,sector} => json!({"owner":w.districts.get(district),"installed":installed(w,district,*sector),"contested":resources_contested(w,district)}),
        CompanyTarget::Research{domain} => json!({"domain":domain,"eligible":target_valid(w,nation,&target)}),
        CompanyTarget::Equipment{project} => json!(w.manufacturing.lines.iter().find(|p|p.id==*project)
            .map(|p|(p.nation,&p.district,&p.kit,w.districts.get(&p.district)))),
        CompanyTarget::CustomEquipment{project} => json!(w.nation_opt(nation).and_then(|n|n.equipment.as_ref())
            .and_then(|e|e.projects.iter().find(|p|p.id==*project)).map(|p|(p.id,&p.revision_id,&p.district,p.status))),
    };
    let payload = json!({"version":1,"release":release,"day":clock::absolute_day(w),"nation":nation,
        "enabled":w.sector_contractors.enabled,"alive":w.nation_opt(nation).is_some_and(|n|n.alive),
        "rules":[clock::is_daily(w),w.rules.industry_rebuild,w.rules.production_system,w.rules.resource_market,w.rules.military_operations],
        "company":company,"company_assignments":company_assignments,"target":target,"target_state":target_state,
        "previous":previous,"eligible":target_valid(w,nation,&target),"reason":reason});
    let bytes = serde_json::to_vec(&payload).expect("finite contractor quote");
    let hash = bytes.into_iter().fold(0xcbf29ce484222325u64,|hash,b|(hash ^ b as u64).wrapping_mul(0x100000001b3));
    ContractorQuote {valid:reason.is_none(),reason,token:format!("contractor-v1-{hash:016x}"),
        work_rate:terms.work_rate,input_rate:terms.input_rate,fee_rate:terms.fee_rate,
        previous_company_id:previous.map(|a|a.company_id),fee_basis:fee_basis(&target).into()}
}
fn resources_contested(w:&WorldState,district:&str)->bool { crate::resources::district_contested(w,district) }
pub fn assignment_quote(w:&WorldState,nation:NationId,company:u32,target:&CompanyTarget)->ContractorQuote {
    reviewed_quote(w,nation,Some(company),target,false)
}
pub fn release_quote(w:&WorldState,nation:NationId,target:&CompanyTarget)->ContractorQuote {
    let canonical=target.clone().canonical();
    let company=w.sector_contractors.assignments.iter().find(|a|a.nation==nation && a.target==canonical).map(|a|a.company_id);
    reviewed_quote(w,nation,company,&canonical,true)
}
pub fn assignment_review_error(w:&WorldState,nation:NationId,company:u32,target:&CompanyTarget,token:&str)->Option<String> {
    let quote=assignment_quote(w,nation,company,target);
    if !quote.valid {return Some(quote.reason.unwrap_or_else(||"This agreement is unavailable.".into()));}
    (token!=quote.token).then(||"The contractor offer changed. Review its current terms before assigning.".into())
}
pub fn release_review_error(w:&WorldState,nation:NationId,target:&CompanyTarget,token:&str)->Option<String> {
    let quote=release_quote(w,nation,target);
    if !quote.valid {return Some(quote.reason.unwrap_or_else(||"This agreement is unavailable.".into()));}
    (token!=quote.token).then(||"The contractor agreement changed. Review it again before releasing.".into())
}
pub fn assign_reviewed(w:&mut WorldState,nation:NationId,company:u32,target:CompanyTarget,token:&str)->Result<(),String> {
    if let Some(reason)=assignment_review_error(w,nation,company,&target,token){return Err(reason);}
    assign(w,nation,company,target)
}
pub fn unassign_reviewed(w:&mut WorldState,nation:NationId,target:&CompanyTarget,token:&str)->Result<(),String> {
    if let Some(reason)=release_review_error(w,nation,target,token){return Err(reason);}
    unassign(w,nation,target)
}

/// One read-only directory with qualified references. Corporate property stays
/// in the supplier book; a service identity owns no implied factory or stock.
pub fn directory(w:&WorldState,nation:NationId)->serde_json::Value {
    use serde_json::json;
    let services=snapshot(w,nation);
    let mut rows:Vec<serde_json::Value>=w.companies.firms.iter().filter(|c|c.nation==nation).map(|c|json!({
        "reference":format!("supplier:{}",c.id),"role":"supplier","id":c.id,"nation":c.nation,
        "name":c.name,"ownership":c.ownership,"district":c.district,"founded_day":c.established_day,
        "cash_bn":c.cash_bn,"capital_received_bn":c.capital_received_bn,
        "earned_revenue_bn":c.development_revenue_bn+c.sales_revenue_bn+c.refit_revenue_bn,
        "stock_units":c.products.iter().map(|p|p.stock as u64).sum::<u64>(),
        "identity_basis":"Player-established equipment supplier; owns its recorded cash, work and stock.",
        "physical_factory_slots":1,"supplier_company_id":c.id
    })).collect();
    if let Some(contractors)=services["companies"].as_array() {
        rows.extend(contractors.iter().map(|c|{let mut row=c.clone();
            row["reference"]=json!(format!("contractor:{}",c["id"]));row["role"]=json!("sector_contractor");
            row["identity_basis"]=json!("Fictional specialist service company; modeled identity, not a historical business claim.");
            row["capacity_label"]=json!("Simultaneous service agreements");row["physical_factory_slots"]=json!(0);
            row["fee_receipts_note"]=json!("Historical service fees paid by the work owner; separate from supplier cash and government holdings.");row}));
    }
    json!({"nation":nation,"enabled":w.sector_contractors.enabled,"rows":rows,"contractors":services,
        "suppliers":crate::companies::view(w,nation),"note":"Service agreements and equipment suppliers share this directory, with separate identities and property accounts."})
}

/// Strict, read-only validation. Expired targets may remain until the next
/// ordinary contractor tick; loading never deletes their receipt history.
pub fn validate(w:&WorldState)->Result<(),String> {
    let book=&w.sector_contractors;
    if book.is_empty(){return Ok(());}
    let today=clock::absolute_day(w);let month=clock::month_index(w);
    let finite=|v:f64|v.is_finite()&&v>=0.0;
    let date=|v:i32|v>=0&&v<=today;
    let fail=|message:&str|Err(format!("Invalid sector contractor state: {message}"));
    if book.roster.len()>100_000||book.assignments.len()>1_000_000||book.news.len()>120
        ||book.last_day.is_some_and(|d|!date(d))||book.last_month.is_some_and(|m|m<0||m>month){return fail("invalid ledger bounds or future settlement date");}
    let mut ids=BTreeSet::new();let mut names=BTreeSet::new();let mut logos=BTreeSet::new();
    for c in &book.roster {
        if c.id==0||c.id>book.next_id||!ids.insert(c.id)||!names.insert(&c.name)||!logos.insert(c.logo.seed)
            ||w.nation_opt(c.nation).is_none()||c.name.trim().is_empty()||c.name.len()>512
            ||c.specialty.len()>2048||c.description.len()>8192||c.strength.len()>2048||c.weakness.len()>2048
            ||(!c.home_district.is_empty()&&!w.districts.contains_key(&c.home_district))||!date(c.founded_day)||c.capacity==0
            ||c.logo.seed>9_007_199_254_740_991||c.logo.mark>=12||c.logo.palette>=12||c.logo.initials.is_empty()||c.logo.initials.len()>32
            ||![c.experience,c.total_work,c.total_fees_bn,c.experience_today].into_iter().all(finite)
            ||!c.total_bonus.is_finite()||c.total_work+c.total_bonus < -1e-9
            ||!c.work_bonus.is_finite()||!(-0.75..=1.0).contains(&c.work_bonus)
            ||!c.input_saving.is_finite()||!(-0.5..=0.5).contains(&c.input_saving)
            ||!c.fee_rate.is_finite()||!(0.0..=1.0).contains(&c.fee_rate)
            ||c.experience_today>1.0+1e-9||c.experience_today>c.experience+1e-9
            ||c.experience_day.is_some_and(|d|!date(d)||d<c.founded_day)
            ||(c.experience_day.is_none()&&c.experience_today!=0.0) {return fail("invalid or duplicated company identity, coefficients or receipts");}
    }
    let mut targets_seen=BTreeSet::new();
    for a in &book.assignments {
        let Some(c)=book.roster.iter().find(|c|c.id==a.company_id)else{return fail("orphan service assignment");};
        if c.nation!=a.nation||c.sector!=a.target.sector()||!targets_seen.insert((a.nation,&a.target))
            ||!date(a.assigned_day)||a.assigned_day<c.founded_day
            ||a.last_day.is_some_and(|d|!date(d)||d<a.assigned_day)
            ||![a.work_today,a.fees_today_bn,a.total_work,a.total_fees_bn].into_iter().all(finite)
            ||!a.bonus_today.is_finite()||!a.total_bonus.is_finite()
            ||a.work_today+a.bonus_today < -1e-9||a.total_work+a.total_bonus < -1e-9
            ||a.work_today>a.total_work+1e-9||a.fees_today_bn>a.total_fees_bn+1e-9
            ||a.total_work>c.total_work+1e-9||a.total_fees_bn>c.total_fees_bn+1e-9
            ||(a.last_day.is_none()&&(a.work_today!=0.0||a.bonus_today!=0.0||a.fees_today_bn!=0.0)) {return fail("invalid assignment ownership, dates or receipts");}
        match &a.target {
            CompanyTarget::Facility{district,sector} if !w.districts.contains_key(district)
                ||!matches!(sector,CompanySector::Mining|CompanySector::Manufacturing|CompanySector::Energy|CompanySector::Logistics)=>return fail("invalid facility target"),
            CompanyTarget::Research{domain} if tech::Domain::parse(domain).is_none()=>return fail("invalid research target"),
            _=>{}
        }
    }
    for c in &book.roster {if used_capacity(w,c.id)>c.effective_capacity() as usize{return fail("service assignments exceed contract capacity");}}
    for (nation,g) in &book.growth {
        if w.nation_opt(*nation).is_none()||g.nation.is_some_and(|n|n!=*nation)||g.samples.len()>19
            ||!g.month_activity.into_iter().all(finite)||g.last_entry_month.is_some_and(|m|m<0||m>month)
            ||g.samples.windows(2).any(|a|a[0].month>=a[1].month)
            ||g.samples.iter().any(|s|s.month < -1||s.month>month||!finite(s.gdp)||!s.activity.into_iter().all(finite)){return fail("invalid growth history");}
    }
    for n in &book.news {if !date(n.day)||n.message.len()>8192
        ||!book.roster.iter().any(|c|c.id==n.company_id&&c.nation==n.nation&&c.founded_day<=n.day){return fail("orphan or invalid founding notice");}}
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::GameRules;
    fn world() -> WorldState {
        let mut w = crate::init::world_1990(GameRules::default());
        clock::enable_daily_play(&mut w);
        enable(&mut w);
        w
    }
    fn nation(code: &str) -> NationId {
        NationId::from_code(code).unwrap()
    }
    fn project(w: &mut WorldState, nation: NationId, id: u32) -> CompanyTarget {
        let district = w
            .districts
            .iter()
            .find(|(_, n)| **n == nation)
            .unwrap()
            .0
            .clone();
        w.production.projects.push(production::Project {
            id,
            nation,
            district,
            kind: K::Infrastructure,
            priority: production::Priority::Normal,
            status: production::ProjectStatus::Building,
            reason: None,
            progress_days: 0.0,
            total_days: 100,
            resources_used: [0.0; 12],
            capacity_micros: None,
            started_day: None,
        });
        CompanyTarget::Construction { project: id }
    }
    #[test]
    fn country_size_unique_identities_and_real_tradeoffs() {
        let w = world();
        let us = nation("USA");
        let bw = nation("Botswana");
        for sector in SECTORS {
            assert!((3..=5).contains(
                &w.sector_contractors
                    .roster
                    .iter()
                    .filter(|c| c.nation == us && c.sector == sector)
                    .count()
            ));
        }
        assert_eq!(
            w.sector_contractors.roster.iter().filter(|c| c.nation == bw).count(),
            3
        );
        let names: BTreeSet<_> = w.sector_contractors.roster.iter().map(|c| &c.name).collect();
        let logos: BTreeSet<_> = w.sector_contractors.roster.iter().map(|c| c.logo.seed).collect();
        assert_eq!(names.len(), w.sector_contractors.roster.len());
        assert_eq!(logos.len(), names.len());
        assert!(w.sector_contractors.roster.iter().all(|c| c.fee_rate > 0.0
            && c.capacity > 0
            && !c.weakness.is_empty()
            && !c.logo.initials.is_empty()));
        for sector in SECTORS {
            let profiles: BTreeSet<_> = w
                .sector_contractors
                .roster
                .iter()
                .filter(|c| c.nation == us && c.sector == sector)
                .map(|c| {
                    (
                        c.work_bonus.to_bits(),
                        c.input_saving.to_bits(),
                        c.fee_rate.to_bits(),
                        c.capacity,
                    )
                })
                .collect();
            assert!(profiles.len() >= 3);
        }
    }
    #[test]
    fn default_is_inert_and_enable_is_idempotent_and_deterministic() {
        let mut w = crate::init::world_1990(GameRules::default());
        let before = serde_json::to_string(&w).unwrap();
        tick_day(&mut w);
        assert_eq!(before, serde_json::to_string(&w).unwrap());
        assert!(w.sector_contractors.is_empty());
        enable(&mut w);
        let first = w.sector_contractors.clone();
        enable(&mut w);
        assert_eq!(first, w.sector_contractors);
        let mut other = crate::init::world_1990(GameRules::default());
        enable(&mut other);
        assert_eq!(first, other.sector_contractors);
        let serialized = serde_json::to_string(&w.sector_contractors).unwrap();
        let restored: Companies = serde_json::from_str(&serialized).unwrap();
        assert_eq!(restored, w.sector_contractors);
    }
    #[test]
    fn assignments_validate_ownership_sector_capacity_and_replacement() {
        let mut w = world();
        let us = nation("USA");
        let bw = nation("Botswana");
        let t = project(&mut w, us, 1);
        let id = w
            .sector_contractors
            .roster
            .iter()
            .find(|c| c.nation == us && c.sector == CompanySector::Construction)
            .unwrap()
            .id;
        let wrong = w
            .sector_contractors
            .roster
            .iter()
            .find(|c| c.nation == us && c.sector == CompanySector::Mining)
            .unwrap()
            .id;
        assert!(assign(&mut w, bw, id, t.clone()).is_err());
        assert!(assign(&mut w, us, wrong, t.clone()).is_err());
        assign(&mut w, us, id, t.clone()).unwrap();
        assign(&mut w, us, id, t.clone()).unwrap();
        assert_eq!(used_capacity(&w, id), 1);
        assert!(modifiers(&w, us, &t).work_rate > 1.0);
        assert_eq!(modifiers(&w, bw, &t), CompanyModifiers::default());
        let capacity = w
            .sector_contractors
            .roster
            .iter()
            .find(|c| c.id == id)
            .unwrap()
            .effective_capacity();
        for i in 2..=capacity as u32 {
            let p = project(&mut w, us, i);
            assign(&mut w, us, id, p).unwrap();
        }
        let extra = project(&mut w, us, 100);
        assert!(assign(&mut w, us, id, extra).is_err());
        let replacement = w
            .sector_contractors
            .roster
            .iter()
            .find(|c| c.nation == us && c.sector == CompanySector::Construction && c.id != id)
            .unwrap()
            .id;
        assign(&mut w, us, replacement, t.clone()).unwrap();
        assert_eq!(modifiers(&w, us, &t).company_id, Some(replacement));
        assert_eq!(
            w.sector_contractors
                .assignments
                .iter()
                .filter(|a| a.target == t)
                .count(),
            1
        );
        unassign(&mut w, us, &t).unwrap();
        assert_eq!(modifiers(&w, us, &t), CompanyModifiers::default());
    }
    #[test]
    fn work_receipts_idle_days_capacity_progression_and_capture_cleanup() {
        let mut w = world();
        let us = nation("USA");
        let t = project(&mut w, us, 1);
        let id = w
            .sector_contractors
            .roster
            .iter()
            .find(|c| c.nation == us && c.sector == CompanySector::Construction)
            .unwrap()
            .id;
        assign(&mut w, us, id, t.clone()).unwrap();
        record_work(&mut w, us, &t, 0.0, 0.0, 0.0);
        assert_eq!(
            w.sector_contractors
                .roster
                .iter()
                .find(|c| c.id == id)
                .unwrap()
                .experience,
            0.0
        );
        for _ in 0..100 {
            record_work(&mut w, us, &t, 1.0, 0.1, 0.001);
        }
        let c = w.sector_contractors.roster.iter().find(|c| c.id == id).unwrap();
        assert_eq!(c.experience, 1.0);
        assert_eq!(c.total_work, 100.0);
        let mut experienced = c.clone();
        experienced.experience = 180.0;
        assert_eq!(experienced.effective_capacity(), c.capacity + 1);
        assert!(experienced.modifiers().work_rate > c.modifiers().work_rate);
        w.day = 2;
        tick_day(&mut w);
        assert_eq!(w.sector_contractors.assignments[0].work_today, 0.0);
        let district = w.production.projects[0].district.clone();
        w.districts.insert(district, nation("Botswana"));
        assert_eq!(modifiers(&w, us, &t), CompanyModifiers::default());
        w.day = 3;
        tick_day(&mut w);
        assert!(w.sector_contractors.assignments.is_empty());
    }
    fn growth_history(spike: bool, activity: bool) -> CompanyGrowth {
        CompanyGrowth {
            samples: (0..19)
                .map(|month| GrowthSample {
                    month,
                    gdp: if spike {
                        if month >= 10 {
                            150.0
                        } else {
                            100.0
                        }
                    } else {
                        100.0 + month as f64 * 1.0
                    },
                    activity: if activity {
                        [10.0 + month as f64; 7]
                    } else {
                        [0.0; 7]
                    },
                })
                .collect(),
            ..Default::default()
        }
    }
    #[test]
    fn growth_requires_sustained_real_activity_not_spikes_or_idle_gdp() {
        assert!(growing_sector(&growth_history(false, true)).is_some());
        assert!(growing_sector(&growth_history(true, true)).is_none());
        assert!(growing_sector(&growth_history(false, false)).is_none());
        let mut short = growth_history(false, true);
        short.samples.remove(0);
        assert!(growing_sector(&short).is_none());
        let mut flat = growth_history(false, true);
        for s in &mut flat.samples {
            s.activity = [10.0; 7];
        }
        assert!(growing_sector(&flat).is_none());
    }
    #[test]
    fn sustained_growth_spawns_unique_company_once_with_cooldown_and_save_continuity() {
        let mut w = world();
        let bw = nation("Botswana");
        let count = w.sector_contractors.roster.len();
        for month in 0..36 {
            w.year = 1990 + month / 12;
            w.month = (month % 12 + 1) as u32;
            w.day = crate::world::days_in_month(w.year, w.month);
            w.nation_mut(bw).gdp = 10.0 + month as f64 * 0.2;
            record_sector_activity(&mut w, bw, CompanySector::Construction, 10.0 + month as f64);
            tick_day(&mut w);
            if w.sector_contractors.roster.len() > count {
                break;
            }
        }
        assert_eq!(w.sector_contractors.roster.len(), count + 1);
        assert!(w.headlines.iter().any(|h| h.starts_with("COMPANIES:")));
        let entrant = w.sector_contractors.roster.last().unwrap();
        assert_eq!(entrant.nation, bw);
        assert!(entrant.description.contains("sustained"));
        assert!(entrant.capacity <= 2);
        let saved = serde_json::to_string(&w).unwrap();
        let mut restored: WorldState = serde_json::from_str(&saved).unwrap();
        let current = clock::month_index(&w);
        for offset in 1..12 {
            for state in [&mut w, &mut restored] {
                let month = current + offset;
                state.year = 1990 + month / 12;
                state.month = (month % 12 + 1) as u32;
                state.day = crate::world::days_in_month(state.year, state.month);
                state.nation_mut(bw).gdp += 0.2;
                record_sector_activity(
                    state,
                    bw,
                    CompanySector::Construction,
                    100.0 + offset as f64,
                );
                tick_day(state);
            }
        }
        assert_eq!(w.sector_contractors.roster.len(), count + 1);
        assert_eq!(w.sector_contractors, restored.sector_contractors);
    }

    #[test]
    fn ai_uses_ordinary_contracts_only_when_competition_is_enabled_and_never_for_player() {
        let mut w = world();
        let us = nation("USA");
        let bw = nation("Botswana");
        w.player = Some(us);
        let player = project(&mut w, us, 1);
        let opponent = project(&mut w, bw, 2);
        choose_ai_contractors(&mut w);
        assert!(w.sector_contractors.assignments.is_empty());
        w.rules.economic_competition = true;
        choose_ai_contractors(&mut w);
        assert!(w
            .sector_contractors
            .assignments
            .iter()
            .any(|a| a.nation == bw && a.target == opponent));
        assert!(!w.sector_contractors.assignments.iter().any(|a| a.target == player));
        let before = w.sector_contractors.assignments.clone();
        choose_ai_contractors(&mut w);
        assert_eq!(before, w.sector_contractors.assignments);
    }
    #[test]
    fn unlimited_roster_growth_cannot_clone_a_domestic_sector_profile_or_logo() {
        let mut w = world();
        let us = nation("USA");
        for ordinal in 5..140 {
            w.sector_contractors.next_id += 1;
            let c = make_company(
                &w,
                us,
                CompanySector::Manufacturing,
                ordinal,
                w.sector_contractors.next_id,
                true,
            );
            w.sector_contractors.roster.push(c);
        }
        let firms: Vec<_> = companies_for(&w, us)
            .filter(|c| c.sector == CompanySector::Manufacturing)
            .collect();
        let profiles: BTreeSet<_> = firms
            .iter()
            .map(|c| {
                (
                    c.work_bonus.to_bits(),
                    c.input_saving.to_bits(),
                    c.fee_rate.to_bits(),
                    c.capacity,
                )
            })
            .collect();
        assert_eq!(profiles.len(), firms.len());
        assert_eq!(
            firms.iter().map(|c| &c.name).collect::<BTreeSet<_>>().len(),
            firms.len()
        );
        assert_eq!(
            firms
                .iter()
                .map(|c| c.logo.seed)
                .collect::<BTreeSet<_>>()
                .len(),
            firms.len()
        );
    }
    #[test]
    fn reviewed_terms_are_pure_and_reject_date_identity_capacity_and_target_changes() {
        let mut w=world();let us=nation("USA");let t=project(&mut w,us,901);
        let id=companies_for(&w,us).find(|c|c.sector==CompanySector::Construction).unwrap().id;
        let before=serde_json::to_value(&w).unwrap();
        let q=assignment_quote(&w,us,id,&t);assert!(q.valid);
        let _=directory(&w,us);assert_eq!(before,serde_json::to_value(&w).unwrap());
        for mutation in 0..4 {
            let mut changed=w.clone();
            match mutation {
                0=>changed.day+=1,
                1=>changed.sector_contractors.roster.iter_mut().find(|c|c.id==id).unwrap().fee_rate+=0.001,
                2=>changed.sector_contractors.roster.iter_mut().find(|c|c.id==id).unwrap().capacity+=1,
                _=>changed.production.projects.iter_mut().find(|p|p.id==901).unwrap().progress_days=1.0,
            }
            let before=serde_json::to_value(&changed).unwrap();
            assert!(assign_reviewed(&mut changed,us,id,t.clone(),&q.token).is_err());
            assert_eq!(before,serde_json::to_value(&changed).unwrap());
        }
        assign_reviewed(&mut w,us,id,t.clone(),&q.token).unwrap();
        let release=release_quote(&w,us,&t);assert!(release.valid);assert_eq!(release.previous_company_id,Some(id));
        unassign_reviewed(&mut w,us,&t,&release.token).unwrap();
        assert_eq!(modifiers(&w,us,&t),CompanyModifiers::default());
    }
    #[test]
    fn validation_checks_inert_history_and_preserves_expired_target_receipts() {
        let mut w=world();validate(&w).unwrap();let us=nation("USA");let t=project(&mut w,us,902);
        let id=companies_for(&w,us).find(|c|c.sector==CompanySector::Construction).unwrap().id;
        assign(&mut w,us,id,t.clone()).unwrap();record_work(&mut w,us,&t,1.0,0.1,0.001);
        validate(&w).unwrap();
        let c=w.sector_contractors.roster.iter().find(|c|c.id==id).unwrap().clone();
        w.production.projects.retain(|p|p.id!=902);
        let before=serde_json::to_value(&w).unwrap();validate(&w).unwrap();
        assert_eq!(before,serde_json::to_value(&w).unwrap());
        w.day+=1;tick_day(&mut w);assert!(w.sector_contractors.assignments.is_empty());
        assert_eq!(w.sector_contractors.roster.iter().find(|c|c.id==id).unwrap(),&c);
        let mut malformed=w.clone();malformed.sector_contractors.enabled=false;
        malformed.sector_contractors.roster[0].fee_rate=f64::NAN;
        assert!(validate(&malformed).is_err(),"disabled books cannot hide malformed receipts");
        let mut malformed=w.clone();malformed.sector_contractors.roster[0].id=malformed.sector_contractors.next_id+1;
        assert!(validate(&malformed).is_err());
        let mut malformed=w.clone();malformed.sector_contractors.last_day=Some(clock::absolute_day(&w)+1);
        assert!(validate(&malformed).is_err());
    }
    #[test]
    fn completed_construction_is_not_an_assignable_service_or_capacity_claim() {
        let mut w=world();let us=nation("USA");let t=project(&mut w,us,903);
        let id=companies_for(&w,us).find(|c|c.sector==CompanySector::Construction).unwrap().id;
        w.production.projects.iter_mut().find(|p|p.id==903).unwrap().progress_days=100.0;
        assert!(!assignment_quote(&w,us,id,&t).valid);
        assert!(!targets(&w,us).iter().any(|o|o.target==t));
        assert_eq!(used_capacity(&w,id),0);
        assert!(!crate::companies::procurement_active(&w,us),"service directory does not enroll supplier procurement");
        assert!(w.companies.firms.is_empty());
    }

}
