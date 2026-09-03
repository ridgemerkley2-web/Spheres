//! Player-directed production and construction.
//!
//! This is deliberately a capability ledger, not a second economy. Projects
//! read the annual budget already enacted, draw real inputs from the resource
//! market's physical stockpile, and leave a level on a province. They never
//! write GDP, debt, growth, stability, technology, or military strength. That
//! boundary lets the arcade board become useful now while later systems decide
//! what a power grid, laboratory, or arms plant actually produces.

use serde::{Deserialize, Serialize};

use crate::resources::{self, Commodity, ALL};
use crate::world::{
    NationId, WorldState, BUDGET_DEFENSE, BUDGET_INDUSTRY, BUDGET_INFRASTRUCTURE, BUDGET_SCIENCE,
};

pub const MAX_ACTIVE_PROJECTS: usize = 4;
pub const MAX_PROVINCE_LEVEL: u8 = 5;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ProjectKind {
    Infrastructure,
    CivilianIndustry,
    PowerGrid,
    ResearchCenter,
    ArmsPlant,
}

pub const PROJECT_KINDS: [ProjectKind; 5] = [
    ProjectKind::Infrastructure,
    ProjectKind::CivilianIndustry,
    ProjectKind::PowerGrid,
    ProjectKind::ResearchCenter,
    ProjectKind::ArmsPlant,
];

impl ProjectKind {
    pub fn key(self) -> &'static str {
        match self {
            Self::Infrastructure => "infrastructure",
            Self::CivilianIndustry => "civilian_industry",
            Self::PowerGrid => "power_grid",
            Self::ResearchCenter => "research_center",
            Self::ArmsPlant => "arms_plant",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        PROJECT_KINDS.into_iter().find(|kind| kind.key() == value)
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Priority {
    High,
    Normal,
    Low,
}

impl Priority {
    pub fn key(self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Normal => "normal",
            Self::Low => "low",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "high" => Some(Self::High),
            "normal" => Some(Self::Normal),
            "low" => Some(Self::Low),
            _ => None,
        }
    }

    fn weight(self) -> f64 {
        match self {
            Self::High => 3.0,
            Self::Normal => 2.0,
            Self::Low => 1.0,
        }
    }

    fn dispatch_rank(self) -> u8 {
        match self {
            Self::High => 0,
            Self::Normal => 1,
            Self::Low => 2,
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProjectStatus {
    Building,
    Slowed,
    Blocked,
}

impl ProjectStatus {
    pub fn key(self) -> &'static str {
        match self {
            Self::Building => "building",
            Self::Slowed => "slowed",
            Self::Blocked => "blocked",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Project {
    pub id: u32,
    pub nation: NationId,
    pub district: String,
    pub kind: ProjectKind,
    pub priority: Priority,
    pub status: ProjectStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub progress_days: f64,
    pub total_days: u32,
    /// Cumulative input in the resource table's raw units, indexed by
    /// `Commodity::idx()`. It closes exactly to `ProjectSpec::recipe`.
    pub resources_used: [f64; 12],
}

impl Project {
    pub fn progress_fraction(&self) -> f64 {
        (self.progress_days / self.total_days.max(1) as f64).clamp(0.0, 1.0)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProvinceCapabilities {
    pub district: String,
    pub infrastructure: u8,
    pub civilian_industry: u8,
    pub power_grid: u8,
    pub research_centers: u8,
    pub arms_plants: u8,
}

impl ProvinceCapabilities {
    fn empty(district: &str) -> Self {
        Self {
            district: district.to_string(),
            infrastructure: 0,
            civilian_industry: 0,
            power_grid: 0,
            research_centers: 0,
            arms_plants: 0,
        }
    }

    pub fn level(&self, kind: ProjectKind) -> u8 {
        match kind {
            ProjectKind::Infrastructure => self.infrastructure,
            ProjectKind::CivilianIndustry => self.civilian_industry,
            ProjectKind::PowerGrid => self.power_grid,
            ProjectKind::ResearchCenter => self.research_centers,
            ProjectKind::ArmsPlant => self.arms_plants,
        }
    }

    fn complete(&mut self, kind: ProjectKind) {
        let slot = match kind {
            ProjectKind::Infrastructure => &mut self.infrastructure,
            ProjectKind::CivilianIndustry => &mut self.civilian_industry,
            ProjectKind::PowerGrid => &mut self.power_grid,
            ProjectKind::ResearchCenter => &mut self.research_centers,
            ProjectKind::ArmsPlant => &mut self.arms_plants,
        };
        *slot = slot.saturating_add(1).min(MAX_PROVINCE_LEVEL);
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Production {
    /// Active work only, in stable project-id order.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub projects: Vec<Project>,
    /// Completed capabilities, sorted by stable district id.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub provinces: Vec<ProvinceCapabilities>,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub next_id: u32,
}

fn is_zero(value: &u32) -> bool {
    *value == 0
}

impl Production {
    pub fn is_empty(&self) -> bool {
        self.projects.is_empty() && self.provinces.is_empty() && self.next_id == 0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProjectSpec {
    pub kind: ProjectKind,
    pub name: &'static str,
    pub description: &'static str,
    pub effect: &'static str,
    pub total_days: u32,
    pub political_cost: f64,
    /// One of the stable `BUDGET_*` indexes in `world.rs`.
    pub funding_ministry: usize,
    pub funding_label: &'static str,
    /// The ministry share of GDP that funds one project at full throughput.
    pub funding_required: f64,
    /// Total project inputs, in each commodity's table unit.
    pub recipe: [f64; 12],
}

pub fn catalog(kind: ProjectKind) -> ProjectSpec {
    match kind {
        ProjectKind::Infrastructure => ProjectSpec {
            kind,
            name: "Infrastructure",
            description: "Roads, rail, bridges, and freight terminals.",
            effect: "+10% construction speed in this province per level.",
            total_days: 360,
            political_cost: 8.0,
            funding_ministry: BUDGET_INFRASTRUCTURE,
            funding_label: "Infrastructure",
            funding_required: 0.025,
            recipe: [0.0, 40.0, 0.0, 4.0, 0.0, 0.0, 60.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        },
        ProjectKind::CivilianIndustry => ProjectSpec {
            kind,
            name: "Civilian Industry",
            description: "Factories, machine shops, and construction suppliers.",
            effect: "+0.15 national construction capacity per level.",
            total_days: 540,
            political_cost: 12.0,
            funding_ministry: BUDGET_INDUSTRY,
            funding_label: "Industry & Energy",
            funding_required: 0.020,
            recipe: [
                12.0, 30.0, 0.0, 8.0, 0.0, 0.0, 45.0, 0.0, 0.0, 0.0, 2.0, 0.0,
            ],
        },
        ProjectKind::PowerGrid => ProjectSpec {
            kind,
            name: "Power Grid",
            description: "Generation, substations, and resilient transmission.",
            effect: "Adds one province power-grid capability level.",
            total_days: 420,
            political_cost: 10.0,
            funding_ministry: BUDGET_INDUSTRY,
            funding_label: "Industry & Energy",
            funding_required: 0.018,
            recipe: [
                6.0, 25.0, 0.0, 18.0, 12.0, 0.0, 35.0, 0.0, 0.0, 0.0, 3.0, 0.0,
            ],
        },
        ProjectKind::ResearchCenter => ProjectSpec {
            kind,
            name: "Research Center",
            description: "Laboratories, computing rooms, and test facilities.",
            effect: "Adds one province research-center capability level.",
            total_days: 600,
            political_cost: 14.0,
            funding_ministry: BUDGET_SCIENCE,
            funding_label: "Science",
            funding_required: 0.012,
            recipe: [0.0, 0.0, 0.0, 10.0, 0.0, 2.0, 8.0, 0.0, 0.0, 1.0, 8.0, 0.0],
        },
        ProjectKind::ArmsPlant => ProjectSpec {
            kind,
            name: "Arms Plant",
            description: "Hardened tooling for future equipment production lines.",
            effect: "Adds one province arms-plant capability level.",
            total_days: 720,
            political_cost: 16.0,
            funding_ministry: BUDGET_DEFENSE,
            funding_label: "Defense",
            funding_required: 0.025,
            recipe: [
                12.0, 15.0, 4.0, 10.0, 0.0, 0.0, 55.0, 0.0, 0.0, 2.0, 3.0, 0.0,
            ],
        },
    }
}

pub fn catalog_all() -> [ProjectSpec; 5] {
    PROJECT_KINDS.map(catalog)
}

pub fn province_capabilities(w: &WorldState, district: &str) -> ProvinceCapabilities {
    w.production
        .provinces
        .binary_search_by(|row| row.district.as_str().cmp(district))
        .ok()
        .map(|i| w.production.provinces[i].clone())
        .unwrap_or_else(|| ProvinceCapabilities::empty(district))
}

pub fn projects_for(w: &WorldState, nation: NationId) -> impl Iterator<Item = &Project> {
    w.production
        .projects
        .iter()
        .filter(move |p| p.nation == nation)
}

pub fn funding_ratio(w: &WorldState, nation: NationId, kind: ProjectKind) -> f64 {
    let spec = catalog(kind);
    let allocation = w.nation(nation).budget_for(w.year).allocations[spec.funding_ministry];
    (allocation / spec.funding_required.max(1e-9)).clamp(0.0, 1.25)
}

/// Shared national work capacity in project-days per calendar day.
pub fn construction_capacity(w: &WorldState, nation: NationId) -> f64 {
    let industry: u32 = w
        .production
        .provinces
        .iter()
        .filter(|row| w.districts.get(&row.district) == Some(&nation))
        .map(|row| row.civilian_industry as u32)
        .sum();
    (1.25 + industry as f64 * 0.15).min(4.0)
}

fn rate_for(w: &WorldState, project: &Project) -> f64 {
    let total_weight: f64 = projects_for(w, project.nation)
        .map(|p| p.priority.weight())
        .sum();
    if total_weight <= 0.0 {
        return 0.0;
    }
    let site = province_capabilities(w, &project.district);
    let site_speed = 1.0 + site.infrastructure as f64 * 0.10;
    (construction_capacity(w, project.nation) * project.priority.weight() / total_weight
        * funding_ratio(w, project.nation, project.kind)
        * site_speed)
        .min(1.5)
}

/// ETA at today's funding, queue priorities, and completed capacity. A blocked
/// project has no honest ETA until its stated blocker is cleared.
pub fn estimated_days_left(w: &WorldState, project: &Project) -> Option<u32> {
    if project.status == ProjectStatus::Blocked
        || input_shortfalls(w, project)
            .into_iter()
            .any(|amount| amount > 1e-9)
    {
        return None;
    }
    let rate = rate_for(w, project);
    (rate > 1e-9).then(|| {
        ((project.total_days as f64 - project.progress_days).max(0.0) / rate).ceil() as u32
    })
}

/// The exact bundle the next day of work will attempt to draw. This is the
/// server-authoritative present-tense requirement; the catalog recipe is the
/// whole-project bill and must not be mistaken for today's shortage.
pub fn next_resource_draw(w: &WorldState, project: &Project) -> [f64; 12] {
    if w.districts.get(&project.district) != Some(&project.nation)
        || !w.nation_opt(project.nation).is_some_and(|n| n.alive)
        || funding_ratio(w, project.nation, project.kind) < 0.05
    {
        return [0.0; 12];
    }
    let rate = rate_for(w, project);
    let advance = rate.min((project.total_days as f64 - project.progress_days).max(0.0));
    if advance <= 1e-9 {
        return [0.0; 12];
    }
    let spec = catalog(project.kind);
    let completes = project.progress_days + advance + 1e-9 >= project.total_days as f64;
    std::array::from_fn(|i| {
        let remaining = (spec.recipe[i] - project.resources_used[i]).max(0.0);
        let raw = if completes {
            remaining
        } else {
            (spec.recipe[i] * advance / project.total_days as f64).min(remaining)
        };
        // The physical market ledger is nanounit-quantized. Quoting and
        // recording the identical quantum prevents a tolerance from creating
        // material or an unrounded daily draw from falsely missing a rounded
        // stock row by a fraction of a nanounit.
        (raw * 1e9).round() / 1e9
    })
}

/// Missing quantities for the next atomic draw, in commodity index order.
/// Fully supplied projects and non-resource blockers return all zeroes.
pub fn input_shortfalls(w: &WorldState, project: &Project) -> [f64; 12] {
    let next = next_resource_draw(w, project);
    std::array::from_fn(|i| {
        let c = Commodity::from_idx(i).expect("twelve resource rows");
        (next[i] - resources::stockpile(w, project.nation, c)).max(0.0)
    })
}

pub fn start_project_error(
    w: &WorldState,
    nation: NationId,
    district: &str,
    kind: ProjectKind,
) -> Option<String> {
    if !w.rules.production_system {
        return Some("Production and construction are not enabled in this game.".into());
    }
    if !w.rules.resource_market {
        return Some("Production requires the resource market to be enabled.".into());
    }
    if w.player != Some(nation) {
        return Some("Only the player can direct construction.".into());
    }
    if !w.nation_opt(nation).is_some_and(|n| n.alive) {
        return Some(format!("{} is not an active government.", nation.name()));
    }
    match w.districts.get(district) {
        None => return Some(format!("No province called {}.", district)),
        Some(owner) if *owner != nation => {
            return Some(format!("{} does not control {}.", nation.name(), district));
        }
        _ => {}
    }
    if projects_for(w, nation).count() >= MAX_ACTIVE_PROJECTS {
        return Some(format!(
            "All {} construction slots are already active.",
            MAX_ACTIVE_PROJECTS
        ));
    }
    if projects_for(w, nation).any(|p| p.district == district && p.kind == kind) {
        return Some(format!(
            "{} is already being built in {}.",
            catalog(kind).name,
            district
        ));
    }
    if province_capabilities(w, district).level(kind) >= MAX_PROVINCE_LEVEL {
        return Some(format!(
            "{} is already at level {} in {}.",
            catalog(kind).name,
            MAX_PROVINCE_LEVEL,
            district
        ));
    }
    None
}

pub fn start_project(
    w: &mut WorldState,
    nation: NationId,
    district: &str,
    kind: ProjectKind,
) -> Result<u32, String> {
    if let Some(reason) = start_project_error(w, nation, district, kind) {
        return Err(reason);
    }
    let id = w.production.next_id.max(1);
    w.production.next_id = id.saturating_add(1);
    let spec = catalog(kind);
    w.production.projects.push(Project {
        id,
        nation,
        district: district.to_string(),
        kind,
        priority: Priority::Normal,
        status: ProjectStatus::Building,
        reason: None,
        progress_days: 0.0,
        total_days: spec.total_days,
        resources_used: [0.0; 12],
    });
    w.headline(format!(
        "{} starts {} in {}.",
        nation.name(),
        spec.name,
        district
    ));
    Ok(id)
}

pub fn set_priority(
    w: &mut WorldState,
    nation: NationId,
    project: u32,
    priority: Priority,
) -> Result<(), String> {
    if !w.rules.production_system {
        return Err("Production and construction are not enabled in this game.".into());
    }
    if w.player != Some(nation) {
        return Err("Only the player can direct construction.".into());
    }
    let row = w
        .production
        .projects
        .iter_mut()
        .find(|p| p.id == project)
        .ok_or_else(|| format!("No active project {}.", project))?;
    if row.nation != nation {
        return Err(format!(
            "{} does not control project {}.",
            nation.name(),
            project
        ));
    }
    row.priority = priority;
    Ok(())
}

pub fn cancel_project(w: &mut WorldState, nation: NationId, project: u32) -> Result<(), String> {
    if !w.rules.production_system {
        return Err("Production and construction are not enabled in this game.".into());
    }
    if w.player != Some(nation) {
        return Err("Only the player can direct construction.".into());
    }
    let Some(index) = w.production.projects.iter().position(|p| p.id == project) else {
        return Err(format!("No active project {}.", project));
    };
    if w.production.projects[index].nation != nation {
        return Err(format!(
            "{} does not control project {}.",
            nation.name(),
            project
        ));
    }
    let removed = w.production.projects.remove(index);
    w.headline(format!(
        "{} cancels {} in {}; committed materials are sunk.",
        nation.name(),
        catalog(removed.kind).name,
        removed.district
    ));
    Ok(())
}

fn set_blocked(w: &mut WorldState, id: u32, reason: String) {
    if let Some(project) = w.production.projects.iter_mut().find(|p| p.id == id) {
        project.status = ProjectStatus::Blocked;
        project.reason = Some(reason);
    }
}

fn complete_capability(w: &mut WorldState, district: &str, kind: ProjectKind) {
    match w
        .production
        .provinces
        .binary_search_by(|row| row.district.as_str().cmp(district))
    {
        Ok(i) => w.production.provinces[i].complete(kind),
        Err(i) => {
            let mut row = ProvinceCapabilities::empty(district);
            row.complete(kind);
            w.production.provinces.insert(i, row);
        }
    }
}

/// Advance construction by one playable calendar day.
pub fn tick_day(w: &mut WorldState) {
    if !w.rules.production_system || w.production.projects.is_empty() {
        return;
    }

    // Snapshot the opening queue. A blocked project does not partly consume,
    // and a completion cannot change another project's allocation until the
    // next day.
    let mut opening = w.production.projects.clone();
    // Priority governs scarce inputs as well as capacity. Stable id breaks a
    // tie, so replay order never depends on a map or on a UI's card order.
    opening.sort_by_key(|project| (project.priority.dispatch_rank(), project.id));
    let mut completed: Vec<(u32, NationId, String, ProjectKind)> = vec![];
    for project in opening {
        if w.districts.get(&project.district) != Some(&project.nation) {
            set_blocked(
                w,
                project.id,
                format!("BLOCKED: {} is no longer controlled.", project.district),
            );
            continue;
        }
        if !w.nation_opt(project.nation).is_some_and(|n| n.alive) {
            set_blocked(
                w,
                project.id,
                "BLOCKED: the sponsoring government no longer exists.".into(),
            );
            continue;
        }

        let spec = catalog(project.kind);
        let funding = funding_ratio(w, project.nation, project.kind);
        if funding < 0.05 {
            set_blocked(
                w,
                project.id,
                format!(
                    "BLOCKED: the {} budget has no construction funding.",
                    spec.funding_label
                ),
            );
            continue;
        }
        let rate = rate_for(w, &project);
        if rate <= 1e-9 {
            set_blocked(
                w,
                project.id,
                "BLOCKED: no construction capacity is assigned.".into(),
            );
            continue;
        }
        let advance = rate.min((project.total_days as f64 - project.progress_days).max(0.0));
        if advance <= 1e-9 {
            completed.push((
                project.id,
                project.nation,
                project.district.clone(),
                project.kind,
            ));
            continue;
        }

        let required = next_resource_draw(w, &project);
        if let Err((commodity, want, have)) =
            resources::consume_stockpile_atomic(w, project.nation, &required)
        {
            set_blocked(
                w,
                project.id,
                format!(
                    "BLOCKED: needs {:.2} {} today, have {:.2}.",
                    want,
                    commodity.name(),
                    have
                ),
            );
            continue;
        }

        let row = w
            .production
            .projects
            .iter_mut()
            .find(|p| p.id == project.id)
            .expect("opening project still active");
        row.progress_days = (row.progress_days + advance).min(row.total_days as f64);
        for c in ALL {
            row.resources_used[c.idx()] =
                (row.resources_used[c.idx()] + required[c.idx()]).min(spec.recipe[c.idx()]);
        }
        if funding + 1e-9 < 1.0 {
            row.status = ProjectStatus::Slowed;
            row.reason = Some(format!(
                "SLOWED: the {} budget funds {:.0}% throughput.",
                spec.funding_label,
                funding * 100.0
            ));
        } else if rate + 1e-9 < 0.80 {
            row.status = ProjectStatus::Slowed;
            row.reason =
                Some("SLOWED: national construction capacity is shared across active work.".into());
        } else {
            row.status = ProjectStatus::Building;
            row.reason = None;
        }
        if row.progress_days + 1e-9 >= row.total_days as f64 {
            completed.push((row.id, row.nation, row.district.clone(), row.kind));
        }
    }

    for (id, nation, district, kind) in completed {
        let Some(index) = w.production.projects.iter().position(|p| p.id == id) else {
            continue;
        };
        w.production.projects.remove(index);
        complete_capability(w, &district, kind);
        let level = province_capabilities(w, &district).level(kind);
        w.headline(format!(
            "{} completes {} level {} in {}.",
            nation.name(),
            catalog(kind).name,
            level,
            district
        ));
    }
}

pub fn tick_days(w: &mut WorldState, days: u32) {
    for _ in 0..days {
        tick_day(w);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::init::world_1990;
    use crate::world::{AnnualBudget, GameRules};
    use crate::{apply_command, load, save, tick_day as world_tick_day, tick_month, Command};

    fn enabled() -> WorldState {
        let mut w = world_1990(GameRules {
            resource_market: true,
            production_system: true,
            ..GameRules::default()
        });
        let nation = NationId::USA;
        w.player = Some(nation);
        w.nation_mut(nation).political_capital = 100.0;
        let mut plan = w.nation(nation).budget_for(w.year);
        for spec in catalog_all() {
            plan.allocations[spec.funding_ministry] =
                plan.allocations[spec.funding_ministry].max(spec.funding_required);
        }
        w.nation_mut(nation).annual_budget = Some(AnnualBudget {
            fiscal_year: w.year,
            allocations: plan.allocations,
            reference: plan.reference,
        });
        w
    }

    fn owned(w: &WorldState, nation: NationId) -> Vec<String> {
        w.districts
            .iter()
            .filter(|(_, owner)| **owner == nation)
            .map(|(district, _)| district.clone())
            .collect()
    }

    fn fill_recipe(w: &mut WorldState, nation: NationId, kind: ProjectKind, multiple: f64) {
        for c in ALL {
            let amount = catalog(kind).recipe[c.idx()] * multiple;
            if amount > 0.0 {
                resources::set_stockpile_for_test(w, nation, c, amount);
            }
        }
    }

    #[test]
    fn production_off_is_byte_inert_and_absent_from_default_saves() {
        let mut a = world_1990(GameRules::default());
        let mut b = a.clone();
        assert!(!save(&a).contains("production_system"));
        assert!(!save(&a).contains("\"production\""));
        tick_days(&mut a, 31);
        tick_month(&mut a, &[]);
        tick_month(&mut b, &[]);
        assert_eq!(save(&a), save(&b));
    }

    #[test]
    fn ownership_slots_and_foreign_project_clicks_are_refused_atomically() {
        let mut w = enabled();
        let nation = NationId::USA;
        let foreign = w
            .districts
            .iter()
            .find(|(_, owner)| **owner != nation)
            .map(|(district, _)| district.clone())
            .unwrap();
        let pc = w.nation(nation).political_capital;
        let before = w.production.clone();
        assert!(apply_command(
            &mut w,
            &Command::StartProject {
                nation,
                district: foreign,
                kind: ProjectKind::Infrastructure,
            },
        )
        .is_err());
        assert_eq!(w.production, before);
        assert_eq!(w.nation(nation).political_capital, pc);

        let districts = owned(&w, nation);
        for district in districts.iter().take(MAX_ACTIVE_PROJECTS) {
            apply_command(
                &mut w,
                &Command::StartProject {
                    nation,
                    district: district.clone(),
                    kind: ProjectKind::Infrastructure,
                },
            )
            .unwrap();
        }
        let before = w.production.clone();
        let pc = w.nation(nation).political_capital;
        assert!(apply_command(
            &mut w,
            &Command::StartProject {
                nation,
                district: districts[MAX_ACTIVE_PROJECTS].clone(),
                kind: ProjectKind::Infrastructure,
            },
        )
        .is_err());
        assert_eq!(w.production, before);
        assert_eq!(w.nation(nation).political_capital, pc);

        let id = w.production.projects[0].id;
        assert!(set_priority(&mut w, NationId::Japan, id, Priority::High).is_err());
        assert!(cancel_project(&mut w, NationId::Japan, id).is_err());
    }

    #[test]
    fn a_shortage_blocks_without_consuming_then_resumes() {
        let mut w = enabled();
        let nation = NationId::USA;
        let district = owned(&w, nation)[0].clone();
        let id = start_project(&mut w, nation, &district, ProjectKind::Infrastructure).unwrap();
        let missing = ALL
            .into_iter()
            .find(|c| catalog(ProjectKind::Infrastructure).recipe[c.idx()] > 0.0)
            .unwrap();
        resources::set_stockpile_for_test(&mut w, nation, missing, 0.0);
        let before = w.resources.clone();
        tick_day(&mut w);
        let project = w.production.projects.iter().find(|p| p.id == id).unwrap();
        assert_eq!(project.status, ProjectStatus::Blocked);
        assert_eq!(project.progress_days, 0.0);
        assert_eq!(w.resources, before);

        fill_recipe(&mut w, nation, ProjectKind::Infrastructure, 1.0);
        tick_day(&mut w);
        let project = w.production.projects.iter().find(|p| p.id == id).unwrap();
        assert!(project.progress_days > 0.0);
        assert_ne!(project.status, ProjectStatus::Blocked);
    }

    #[test]
    fn completion_leaves_a_province_level_that_speeds_future_work() {
        let mut w = enabled();
        let nation = NationId::USA;
        let district = owned(&w, nation)[0].clone();
        fill_recipe(&mut w, nation, ProjectKind::Infrastructure, 2.0);
        let id = start_project(&mut w, nation, &district, ProjectKind::Infrastructure).unwrap();
        let before_rate = rate_for(&w, &w.production.projects[0]);
        {
            let project = w
                .production
                .projects
                .iter_mut()
                .find(|p| p.id == id)
                .unwrap();
            project.progress_days = project.total_days as f64 - 0.5;
            let spec = catalog(project.kind);
            for c in ALL {
                project.resources_used[c.idx()] =
                    spec.recipe[c.idx()] * project.progress_days / project.total_days as f64;
            }
        }
        tick_day(&mut w);
        assert!(w.production.projects.is_empty());
        assert_eq!(province_capabilities(&w, &district).infrastructure, 1);

        start_project(&mut w, nation, &district, ProjectKind::Infrastructure).unwrap();
        let after_rate = rate_for(&w, &w.production.projects[0]);
        assert!(after_rate > before_rate);
    }

    #[test]
    fn projects_and_capabilities_survive_save_and_load() {
        let mut w = enabled();
        let nation = NationId::USA;
        let district = owned(&w, nation)[0].clone();
        fill_recipe(&mut w, nation, ProjectKind::CivilianIndustry, 1.0);
        start_project(&mut w, nation, &district, ProjectKind::CivilianIndustry).unwrap();
        tick_day(&mut w);
        complete_capability(&mut w, &district, ProjectKind::PowerGrid);
        let restored = load(&save(&w)).unwrap();
        assert_eq!(restored.production, w.production);
    }

    #[test]
    fn daily_and_monthly_clocks_leave_the_same_production_world() {
        let mut monthly = enabled();
        let nation = NationId::USA;
        let district = owned(&monthly, nation)[0].clone();
        fill_recipe(&mut monthly, nation, ProjectKind::Infrastructure, 10.0);
        start_project(&mut monthly, nation, &district, ProjectKind::Infrastructure).unwrap();
        let mut daily = monthly.clone();
        let days = crate::world::days_in_month(monthly.year, monthly.month);
        tick_month(&mut monthly, &[]);
        for _ in 0..days {
            world_tick_day(&mut daily, &[]);
        }
        assert_eq!(save(&daily), save(&monthly));
    }

    #[test]
    fn every_unit_recorded_as_used_leaves_the_same_physical_stockpile() {
        let mut w = enabled();
        let nation = NationId::USA;
        let district = owned(&w, nation)[0].clone();
        let kind = ProjectKind::ArmsPlant;
        fill_recipe(&mut w, nation, kind, 10.0);
        let opening: [f64; 12] = std::array::from_fn(|i| {
            resources::stockpile(&w, nation, Commodity::from_idx(i).unwrap())
        });
        start_project(&mut w, nation, &district, kind).unwrap();
        for _ in 0..17 {
            tick_day(&mut w);
        }
        let project = &w.production.projects[0];
        for c in ALL {
            let closing = resources::stockpile(&w, nation, c);
            assert!(
                ((opening[c.idx()] - closing) - project.resources_used[c.idx()]).abs() < 1e-6,
                "{} was not conserved",
                c.name()
            );
        }
    }

    #[test]
    fn high_priority_work_gets_a_scarce_daily_bundle_first() {
        let mut w = enabled();
        let nation = NationId::USA;
        let districts = owned(&w, nation);
        let low =
            start_project(&mut w, nation, &districts[0], ProjectKind::Infrastructure).unwrap();
        let high =
            start_project(&mut w, nation, &districts[1], ProjectKind::Infrastructure).unwrap();
        set_priority(&mut w, nation, low, Priority::Low).unwrap();
        set_priority(&mut w, nation, high, Priority::High).unwrap();
        let high_project = w
            .production
            .projects
            .iter()
            .find(|project| project.id == high)
            .unwrap()
            .clone();
        let draw = next_resource_draw(&w, &high_project);
        for c in ALL {
            if draw[c.idx()] > 0.0 {
                resources::set_stockpile_for_test(&mut w, nation, c, draw[c.idx()]);
            }
        }

        tick_day(&mut w);
        let high_project = w.production.projects.iter().find(|p| p.id == high).unwrap();
        let low_project = w.production.projects.iter().find(|p| p.id == low).unwrap();
        assert!(high_project.progress_days > 0.0);
        assert_eq!(low_project.progress_days, 0.0);
        assert_eq!(low_project.status, ProjectStatus::Blocked);
    }

    #[test]
    fn fresh_market_consumes_the_same_opening_cover_that_it_reports() {
        let mut w = enabled();
        let nation = NationId::USA;
        resources::warm(&mut w);
        assert!(
            w.resources.market.is_none(),
            "the opening ledger was already materialised"
        );
        let (commodity, opening) = ALL
            .into_iter()
            .filter(|c| *c != Commodity::Oil)
            .map(|c| (c, resources::stock_quantity(&w, nation, c)))
            .find(|(_, quantity)| *quantity > 1e-6)
            .expect("USA's opening procurement cover reports a physical input");
        assert_eq!(resources::stockpile(&w, nation, commodity), opening);
        let used = (opening.min(1.0) * 0.25 * 1e9).floor() / 1e9;
        let mut bundle = [0.0; 12];
        bundle[commodity.idx()] = used;

        resources::consume_stockpile_atomic(&mut w, nation, &bundle).unwrap();

        let market = w
            .resources
            .market
            .as_ref()
            .expect("the opening cover became a warehouse");
        assert_eq!(
            market.last_produced,
            i32::MIN,
            "no monthly production was posted"
        );
        assert!(market.fills.is_empty() && market.contract_fills.is_empty());
        assert!((resources::stock_quantity(&w, nation, commodity) - (opening - used)).abs() < 2e-9);
    }

    #[test]
    fn production_is_not_advertised_without_a_physical_resource_market() {
        let mut w = enabled();
        let nation = NationId::USA;
        let district = owned(&w, nation)[0].clone();
        w.rules.resource_market = false;
        let reason = start_project_error(&w, nation, &district, ProjectKind::Infrastructure)
            .expect("the coupled switch is required");
        assert!(reason.contains("resource market"));
        assert_eq!(resources::stockpile(&w, nation, Commodity::Iron), 0.0);
    }

    #[test]
    fn equal_priority_projects_report_shared_capacity_neutrally() {
        let mut w = enabled();
        let nation = NationId::USA;
        let districts = owned(&w, nation);
        fill_recipe(&mut w, nation, ProjectKind::Infrastructure, 2.0);
        start_project(&mut w, nation, &districts[0], ProjectKind::Infrastructure).unwrap();
        start_project(&mut w, nation, &districts[1], ProjectKind::Infrastructure).unwrap();

        tick_day(&mut w);

        for project in &w.production.projects {
            assert_eq!(project.status, ProjectStatus::Slowed);
            assert_eq!(
                project.reason.as_deref(),
                Some("SLOWED: national construction capacity is shared across active work.")
            );
        }
    }
}
