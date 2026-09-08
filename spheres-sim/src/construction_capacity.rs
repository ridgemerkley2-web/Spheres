//! National construction allocation for the opted-in daily industry rebuild.
//!
//! MODEL units: ten capacity perform one physical work-day, each site accepts
//! twenty capacity, and a ten-capacity starter floor lets small economies build
//! before operating a complete domestic industrial chain. These are gameplay
//! conversions, not a census of historical construction firms. Actual funding
//! and material draws remain in the single industry WorkPlan/settlement path.
use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::{
    clock, industrial_modules, industry_operations, production,
    production::{Project, ProjectKind},
    resources,
    world::{NationId, WorldState},
};

pub const CAPACITY_PER_WORK_DAY: f64 = 10.0;
pub const MAX_PER_PROJECT: f64 = 20.0;
pub const STARTER_CAPACITY: f64 = 10.0;
pub const CIVILIAN_CAPACITY_PER_LEVEL: f64 = 10.0;
pub const INFRASTRUCTURE_WORK_BONUS: f64 = 0.10;
pub const CREW_SKILL_RECIPE: [f64; 3] = [0.72, 0.24, 0.04];
const EPS: f64 = 1e-9;

/// Local crews can use at most a full site's twenty assigned capacity. Their
/// qualification limit is a separate ceiling on work, not another multiplier
/// on the already staffed civilian factory service in the national pool.
pub fn crew_work_limit(w: &WorldState, district: &str) -> f64 {
    if !crate::population::active(w) { return f64::INFINITY; }
    let maximum = if w.rules.industry_rebuild { MAX_PER_PROJECT / CAPACITY_PER_WORK_DAY } else { 1.0 };
    let infrastructure = production::level(w, district, ProjectKind::Infrastructure) as f64;
    maximum * (1.0 + infrastructure * INFRASTRUCTURE_WORK_BONUS)
        * crate::population::district_skill_staffing(w, district, 4, CREW_SKILL_RECIPE)
}

pub fn enabled(w: &WorldState) -> bool {
    w.rules.industry_rebuild && w.rules.production_system && clock::is_daily(w)
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct CapacitySnapshot {
    pub inherited_capacity: f64,
    pub civilian_capacity: f64,
    pub starter_capacity: f64,
    pub total_capacity: f64,
    pub assigned_capacity: f64,
    pub mine_assigned_capacity: f64,
    pub idle_capacity: f64,
    pub max_per_project: f64,
    pub capacity_per_work_day: f64,
    pub projects: Vec<AllocationView>,
    pub mines: Vec<MineAllocationView>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
struct NationAllocation {
    projects: BTreeMap<u32, f64>,
    mines: BTreeMap<String, f64>,
}

/// Opening-day grants persist until the next date. A completed building cannot
/// donate the capacity it already used to a later mine in the same tick.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DayAllocation {
    pub day: i32,
    nations: BTreeMap<NationId, NationAllocation>,
    /// Inherited, commissioned civilian, bootstrap, total capacity.
    pools: BTreeMap<NationId, [f64; 4]>,
}

#[derive(Clone, Debug, Serialize)]
pub struct MineAllocationView {
    pub district: String,
    pub commodity: resources::Commodity,
    pub requested: Option<f64>,
    pub assigned: f64,
    pub max: f64,
    pub mode: &'static str,
    pub nominal_work_days: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct AllocationView {
    pub project: u32,
    pub requested: Option<f64>,
    pub assigned: f64,
    pub max: f64,
    pub mode: &'static str,
    pub nominal_work_days: f64,
}

fn finite_nonnegative(value: f64) -> f64 {
    if value.is_finite() {
        value.max(0.0)
    } else {
        0.0
    }
}

/// Installed, usable national capacity before any project assignments. A lost
/// province immediately leaves its former owner's pool; frozen inherited data
/// and completed civilian factories use the same current operating constraints.
pub fn pool(w: &WorldState, nation: NationId) -> CapacitySnapshot {
    let mut out = CapacitySnapshot {
        max_per_project: MAX_PER_PROJECT,
        capacity_per_work_day: CAPACITY_PER_WORK_DAY,
        ..Default::default()
    };
    if !enabled(w) || !w.nation_opt(nation).is_some_and(|n| n.alive) {
        return out;
    }
    if let Some([inherited, civilian, starter, total]) =
        today(w).and_then(|d| d.pools.get(&nation)).copied()
    {
        out.inherited_capacity = inherited;
        out.civilian_capacity = civilian;
        out.starter_capacity = starter;
        out.total_capacity = total;
        out.idle_capacity = total;
        return out;
    }
    out.inherited_capacity = finite_nonnegative(
        industry_operations::inherited_construction_capacity(w, nation),
    );
    out.civilian_capacity = w
        .districts
        .iter()
        .filter(|(_, owner)| **owner == nation)
        .map(|(district, _)| {
            let levels =
                industrial_modules::effective_capacity(w, district, ProjectKind::CivilianIndustry);
            if levels <= 0.0 {
                return 0.0;
            }
            levels
                * CIVILIAN_CAPACITY_PER_LEVEL
                * industry_operations::civilian_operating_fraction(w, district).clamp(0.0, 1.0)
        })
        .map(finite_nonnegative)
        .sum();
    // The bootstrap service supplements the inherited base only. A small
    // country's first factory must add work instead of merely replacing its
    // starting ability to build that factory.
    out.starter_capacity = (STARTER_CAPACITY - out.inherited_capacity).max(0.0);
    out.total_capacity = out.inherited_capacity + out.starter_capacity + out.civilian_capacity;
    out.idle_capacity = out.total_capacity;
    out
}

fn eligible(w: &WorldState, p: &Project) -> bool {
    w.nation_opt(p.nation).is_some_and(|n| n.alive)
        && w.districts.get(&p.district) == Some(&p.nation)
        && !resources::district_contested(w, &p.district)
        && (!w.rules.military_operations
            || crate::control::blocker(w, p.nation, &p.district).is_none())
        && p.progress_days + EPS < p.total_days as f64
}

/// Deterministic capped water filling. Manual requests reserve the pool first;
/// if an existing allocation becomes unaffordable after lost operating capacity,
/// its grant shrinks proportionally without erasing the saved request. Remaining
/// capacity follows priority weights among automatic projects, with each capped
/// at twenty. Zero manual requests never re-enter automatic allocation.
fn distribute(total: f64, rows: &[(u32, f64, Option<f64>)]) -> BTreeMap<u32, f64> {
    let mut out: BTreeMap<_, _> = rows.iter().map(|(id, _, _)| (*id, 0.0)).collect();
    let manual_total: f64 = rows
        .iter()
        .filter_map(|(_, _, requested)| *requested)
        .map(|v| finite_nonnegative(v).min(MAX_PER_PROJECT))
        .sum();
    let manual_scale = if manual_total > EPS {
        (total / manual_total).min(1.0)
    } else {
        0.0
    };
    let mut remaining = total;
    for (id, _, requested) in rows {
        if let Some(requested) = requested {
            let assigned = (finite_nonnegative(*requested).min(MAX_PER_PROJECT) * manual_scale)
                .min(remaining)
                .max(0.0);
            out.insert(*id, assigned);
            remaining = (remaining - assigned).max(0.0);
        }
    }
    let mut automatic: Vec<_> = rows
        .iter()
        .filter(|(_, _, requested)| requested.is_none())
        .collect();
    while remaining > EPS && !automatic.is_empty() {
        let weight: f64 = automatic.iter().map(|(_, weight, _)| *weight).sum();
        if weight <= EPS {
            break;
        }
        let share = remaining / weight;
        let capped: Vec<u32> = automatic
            .iter()
            .filter(|(_, weight, _)| share * *weight >= MAX_PER_PROJECT)
            .map(|(id, _, _)| *id)
            .collect();
        if capped.is_empty() {
            for (id, weight, _) in automatic {
                let assigned = (share * *weight).min(remaining).max(0.0);
                out.insert(*id, assigned);
                remaining = (remaining - assigned).max(0.0);
            }
            break;
        }
        for id in &capped {
            let assigned = MAX_PER_PROJECT.min(remaining);
            out.insert(*id, assigned);
            remaining = (remaining - assigned).max(0.0);
        }
        automatic.retain(|(id, _, _)| !capped.contains(id));
    }
    out
}

fn mine_eligible(w: &WorldState, p: &resources::MineProject) -> bool {
    let funding = w
        .production
        .industry
        .mines
        .get(&crate::industry::mine_key(&p.district, p.commodity));
    funding.is_some_and(|f| f.progress_days + EPS < f.total_days as f64)
        && w.nation_opt(p.started_by).is_some_and(|n| n.alive)
        && w.districts.get(&p.district) == Some(&p.started_by)
        && !resources::district_contested(w, &p.district)
        && (!w.rules.military_operations
            || crate::control::blocker(w, p.started_by, &p.district).is_none())
}

fn nation_allocations(w: &WorldState, nation: NationId, total: f64) -> NationAllocation {
    if let Some(allocation) = today(w).and_then(|d| d.nations.get(&nation)) {
        return allocation.clone();
    }
    let mut projects: Vec<_> = production::projects_for(w, nation)
        .filter(|p| eligible(w, p))
        .map(|p| {
            (
                p.id,
                p.priority.weight(),
                w.production.construction_assignments.get(&p.id).copied(),
            )
        })
        .collect();
    projects.sort_by_key(|(id, _, _)| *id);
    let mut mines: Vec<_> = w
        .resources
        .mine_projects
        .iter()
        .filter(|p| p.started_by == nation && mine_eligible(w, p))
        .map(|p| {
            let key = crate::industry::mine_key(&p.district, p.commodity);
            let requested = w
                .production
                .mine_construction_assignments
                .get(&key)
                .copied();
            (key, production::Priority::Normal.weight(), requested)
        })
        .collect();
    mines.sort_by(|a, b| a.0.cmp(&b.0));
    let rows: Vec<_> = projects
        .iter()
        .map(|(_, weight, requested)| (*weight, *requested))
        .chain(
            mines
                .iter()
                .map(|(_, weight, requested)| (*weight, *requested)),
        )
        .enumerate()
        .map(|(index, (weight, requested))| (index as u32, weight, requested))
        .collect();
    let assigned = distribute(total, &rows);
    let mut out = NationAllocation::default();
    for (index, (id, _, _)) in projects.iter().enumerate() {
        out.projects.insert(*id, assigned[&(index as u32)]);
    }
    for (index, (key, _, _)) in mines.iter().enumerate() {
        out.mines
            .insert(key.clone(), assigned[&((index + projects.len()) as u32)]);
    }
    out
}

fn today(w: &WorldState) -> Option<&DayAllocation> {
    w.production
        .construction_day
        .as_ref()
        .filter(|d| d.day == clock::absolute_day(w))
}

/// Tick hook, called once before either buildings or mines perform work.
/// Planning/inspection never calls this mutation. Midday orders and assignment
/// changes take effect on the next construction date after grants are frozen.
pub fn begin_day(w: &mut WorldState) {
    if !enabled(w) || today(w).is_some() {
        return;
    }
    let nations: BTreeSet<_> = w
        .production
        .projects
        .iter()
        .map(|p| p.nation)
        .chain(w.resources.mine_projects.iter().map(|p| p.started_by))
        .collect();
    if nations.is_empty() {
        return;
    }
    let mut day = DayAllocation {
        day: clock::absolute_day(w),
        nations: BTreeMap::new(),
        pools: BTreeMap::new(),
    };
    for nation in nations {
        let pool = pool(w, nation);
        day.pools.insert(
            nation,
            [
                pool.inherited_capacity,
                pool.civilian_capacity,
                pool.starter_capacity,
                pool.total_capacity,
            ],
        );
        day.nations
            .insert(nation, nation_allocations(w, nation, pool.total_capacity));
    }
    w.production.construction_day = Some(day);
}

/// One allocation pass for the entire day's WorkPlan builder. This is read-only
/// so inspection and ETA previews cannot consume or reassign capacity.
pub fn allocations(w: &WorldState) -> BTreeMap<u32, f64> {
    if !enabled(w) {
        return BTreeMap::new();
    }
    let nations: BTreeSet<_> = w.production.projects.iter().map(|p| p.nation).collect();
    let mut out = BTreeMap::new();
    for nation in nations {
        out.extend(nation_allocations(w, nation, pool(w, nation).total_capacity).projects);
    }
    out
}

/// Physical work before the shared finance/input preflight, including the same
/// fractional-module commissioning clock used for settlement and preview.
pub fn nominal_work_with_assignment(w: &WorldState, p: &Project, assigned: f64) -> f64 {
    if !enabled(w) || !eligible(w, p) {
        return 0.0;
    }
    let infrastructure = production::level(w, &p.district, ProjectKind::Infrastructure) as f64;
    let physical = finite_nonnegative(assigned).min(MAX_PER_PROJECT) / CAPACITY_PER_WORK_DAY
        * (1.0 + infrastructure * INFRASTRUCTURE_WORK_BONUS);
    industrial_modules::normalized_advance(w, p, physical.min(crew_work_limit(w, &p.district)))
        .min((p.total_days as f64 - p.progress_days).max(0.0))
}

pub fn nominal_work(w: &WorldState, p: &Project) -> f64 {
    let assigned = nation_allocations(w, p.nation, pool(w, p.nation).total_capacity)
        .projects
        .get(&p.id)
        .copied()
        .unwrap_or(0.0);
    nominal_work_with_assignment(w, p, assigned)
}

pub fn snapshot(w: &WorldState, nation: NationId) -> CapacitySnapshot {
    let mut out = pool(w, nation);
    if !enabled(w) {
        return out;
    }
    let assigned = nation_allocations(w, nation, out.total_capacity);
    out.projects = production::projects_for(w, nation)
        .map(|p| {
            let requested = w.production.construction_assignments.get(&p.id).copied();
            let assigned = assigned.projects.get(&p.id).copied().unwrap_or(0.0);
            AllocationView {
                project: p.id,
                requested,
                assigned,
                max: MAX_PER_PROJECT,
                mode: if requested.is_some() {
                    "manual"
                } else {
                    "auto"
                },
                nominal_work_days: nominal_work_with_assignment(w, p, assigned),
            }
        })
        .collect();
    out.projects.sort_by_key(|p| p.project);
    out.mines = w
        .resources
        .mine_projects
        .iter()
        .filter(|p| p.started_by == nation)
        .filter(|p| {
            w.production
                .industry
                .mines
                .contains_key(&crate::industry::mine_key(&p.district, p.commodity))
        })
        .map(|p| {
            let key = crate::industry::mine_key(&p.district, p.commodity);
            let requested = w
                .production
                .mine_construction_assignments
                .get(&key)
                .copied();
            let amount = assigned.mines.get(&key).copied().unwrap_or(0.0);
            MineAllocationView {
                district: p.district.clone(),
                commodity: p.commodity,
                requested,
                assigned: amount,
                max: MAX_PER_PROJECT,
                mode: if requested.is_some() {
                    "manual"
                } else {
                    "auto"
                },
                nominal_work_days: mine_work_with_assignment(w, p, amount),
            }
        })
        .collect();
    out.mines.sort_by(|a, b| {
        a.district
            .cmp(&b.district)
            .then(a.commodity.key().cmp(b.commodity.key()))
    });
    out.mine_assigned_capacity = assigned.mines.values().sum();
    out.assigned_capacity = assigned.projects.values().sum::<f64>() + out.mine_assigned_capacity;
    out.idle_capacity = (out.total_capacity - out.assigned_capacity).max(0.0);
    out
}

pub fn allocation_for(w: &WorldState, project: u32) -> Option<AllocationView> {
    let p = w.production.projects.iter().find(|p| p.id == project)?;
    snapshot(w, p.nation)
        .projects
        .into_iter()
        .find(|p| p.project == project)
}

pub fn mine_capacity(
    w: &WorldState,
    nation: NationId,
    district: &str,
    commodity: resources::Commodity,
) -> f64 {
    if !enabled(w) {
        return 0.0;
    }
    nation_allocations(w, nation, pool(w, nation).total_capacity)
        .mines
        .get(&crate::industry::mine_key(district, commodity))
        .copied()
        .unwrap_or(0.0)
}

pub fn mine_work_with_assignment(w: &WorldState, p: &resources::MineProject, assigned: f64) -> f64 {
    if !enabled(w) || !mine_eligible(w, p) {
        return 0.0;
    }
    let funding =
        &w.production.industry.mines[&crate::industry::mine_key(&p.district, p.commodity)];
    let infrastructure = production::level(w, &p.district, ProjectKind::Infrastructure) as f64;
    (finite_nonnegative(assigned).min(MAX_PER_PROJECT) / CAPACITY_PER_WORK_DAY
        * (1.0 + infrastructure * INFRASTRUCTURE_WORK_BONUS))
        .min(crew_work_limit(w, &p.district))
        .min((funding.total_days as f64 - funding.progress_days).max(0.0))
}

pub fn mine_nominal_work(w: &WorldState, p: &resources::MineProject) -> f64 {
    mine_work_with_assignment(
        w,
        p,
        mine_capacity(w, p.started_by, &p.district, p.commodity),
    )
}

fn manual_total(
    w: &WorldState,
    nation: NationId,
    except_project: Option<u32>,
    except_mine: Option<&str>,
) -> f64 {
    let building: f64 = production::projects_for(w, nation)
        .filter(|p| Some(p.id) != except_project && eligible(w, p))
        .filter_map(|p| w.production.construction_assignments.get(&p.id))
        .map(|v| finite_nonnegative(*v).min(MAX_PER_PROJECT))
        .sum();
    let mines: f64 = w
        .resources
        .mine_projects
        .iter()
        .filter(|p| p.started_by == nation && mine_eligible(w, p))
        .map(|p| crate::industry::mine_key(&p.district, p.commodity))
        .filter(|key| Some(key.as_str()) != except_mine)
        .filter_map(|key| w.production.mine_construction_assignments.get(&key))
        .map(|v| finite_nonnegative(*v).min(MAX_PER_PROJECT))
        .sum();
    building + mines
}

pub fn set_mine_assignment(
    w: &mut WorldState,
    nation: NationId,
    district: &str,
    commodity: resources::Commodity,
    capacity: Option<f64>,
) -> Result<(), String> {
    if !enabled(w) {
        return Err("Construction allocation requires the rebuilt daily industry system.".into());
    }
    if w.player != Some(nation) {
        return Err("Only the player can assign national construction capacity.".into());
    }
    if !w.nation_opt(nation).is_some_and(|n| n.alive) {
        return Err("This government is no longer active.".into());
    }
    if capacity.is_some_and(|v| !v.is_finite() || !(0.0..=MAX_PER_PROJECT).contains(&v)) {
        return Err(format!(
            "Assign a finite capacity between 0 and {MAX_PER_PROJECT:.0}, or choose Auto."
        ));
    }
    let key = crate::industry::mine_key(district, commodity);
    if w.districts.get(district) != Some(&nation)
        || !w
            .resources
            .mine_projects
            .iter()
            .any(|p| p.started_by == nation && p.district == district && p.commodity == commodity)
        || !w.production.industry.mines.contains_key(&key)
    {
        return Err("No controlled active construction project exists for this mine.".into());
    }
    if let Some(requested) = capacity.filter(|v| *v > 0.0) {
        let available =
            (pool(w, nation).total_capacity - manual_total(w, nation, None, Some(&key))).max(0.0);
        if requested > available + EPS {
            return Err(format!("Only {available:.2} construction capacity remains after other manual assignments. Reduce one or choose Auto."));
        }
    }
    match capacity {
        Some(value) => {
            w.production
                .mine_construction_assignments
                .insert(key, value);
        }
        None => {
            w.production.mine_construction_assignments.remove(&key);
        }
    }
    Ok(())
}

/// Command endpoint for explicit assignments. It never buys capacity or work:
/// it only changes a saved request against capacity the nation already has.
pub fn set_assignment(
    w: &mut WorldState,
    nation: NationId,
    project: u32,
    capacity: Option<f64>,
) -> Result<(), String> {
    if !enabled(w) {
        return Err("Construction allocation requires the rebuilt daily industry system.".into());
    }
    if w.player != Some(nation) {
        return Err("Only the player can assign national construction capacity.".into());
    }
    if !w.nation_opt(nation).is_some_and(|n| n.alive) {
        return Err("This government is no longer active.".into());
    }
    if capacity.is_some_and(|value| !value.is_finite() || !(0.0..=MAX_PER_PROJECT).contains(&value))
    {
        return Err(format!(
            "Assign a finite capacity between 0 and {MAX_PER_PROJECT:.0}, or choose Auto."
        ));
    }
    let p = w
        .production
        .projects
        .iter()
        .find(|p| p.id == project)
        .ok_or_else(|| format!("No active project {project}."))?;
    if p.nation != nation || w.districts.get(&p.district) != Some(&nation) {
        return Err("This government does not control the project and its province.".into());
    }
    if let Some(requested) = capacity.filter(|v| *v > 0.0) {
        let other = manual_total(w, nation, Some(project), None);
        let available = (pool(w, nation).total_capacity - other).max(0.0);
        if requested > available + EPS {
            return Err(format!("Only {available:.2} construction capacity remains after other manual assignments. Reduce one or choose Auto."));
        }
    }
    match capacity {
        Some(value) => {
            w.production.construction_assignments.insert(project, value);
        }
        None => {
            w.production.construction_assignments.remove(&project);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{init::world_1990, load, save, world::GameRules};

    fn prepared() -> (WorldState, String) {
        let mut w = world_1990(GameRules {
            daily_simulation: true,
            production_system: true,
            industry_rebuild: true,
            ..Default::default()
        });
        w.player = Some(NationId::USA);
        let district = w
            .districts
            .iter()
            .find(|(_, n)| **n == NationId::USA)
            .unwrap()
            .0
            .clone();
        (w, district)
    }

    fn near(actual: f64, expected: f64) {
        assert!((actual - expected).abs() < EPS, "{actual} != {expected}");
    }

    #[test]
    fn priority_weights_and_site_caps_conserve_the_national_pool() {
        let rows = [(1, 3.0, None), (2, 2.0, None), (3, 1.0, None)];
        let allocation = distribute(12.0, &rows);
        near(allocation[&1], 6.0);
        near(allocation[&2], 4.0);
        near(allocation[&3], 2.0);
        let allocation = distribute(50.0, &rows);
        near(allocation[&1], 20.0);
        near(allocation[&2], 20.0);
        near(allocation[&3], 10.0);
        // Universal invariant over capacities, including less than one unit,
        // oversubscribed explicit requests and pools larger than all site caps.
        for tenths in 0..1000 {
            let total = tenths as f64 / 10.0;
            for requests in [
                rows,
                [(1, 3.0, Some(0.0)), (2, 2.0, Some(20.0)), (3, 1.0, None)],
            ] {
                let allocation = distribute(total, &requests);
                assert!(allocation
                    .values()
                    .all(|a| a.is_finite() && (0.0..=MAX_PER_PROJECT).contains(a)));
                assert!(allocation.values().sum::<f64>() <= total + EPS);
            }
        }
    }

    #[test]
    fn capacity_losses_reduce_grants_without_forgetting_requested_allocations() {
        let rows = [(1, 1.0, Some(20.0)), (2, 1.0, Some(10.0)), (3, 3.0, None)];
        let low = distribute(15.0, &rows);
        near(low[&1], 10.0);
        near(low[&2], 5.0);
        near(low[&3], 0.0);
        let restored = distribute(40.0, &rows);
        near(restored[&1], 20.0);
        near(restored[&2], 10.0);
        near(restored[&3], 10.0);
    }

    #[test]
    fn assignment_validation_pause_resume_and_save_are_exact() {
        let (mut w, district) = prepared();
        let first = production::start_project(
            &mut w,
            NationId::USA,
            &district,
            ProjectKind::Infrastructure,
        )
        .unwrap();
        let second = production::start_project(
            &mut w,
            NationId::USA,
            &district,
            ProjectKind::OfficeDistrict,
        )
        .unwrap();
        near(pool(&w, NationId::USA).total_capacity, STARTER_CAPACITY);
        for invalid in [f64::NAN, f64::INFINITY, -1.0, 20.01] {
            let before = save(&w);
            assert!(set_assignment(&mut w, NationId::USA, first, Some(invalid)).is_err());
            assert_eq!(save(&w), before);
        }
        assert!(set_assignment(&mut w, NationId::UK, first, Some(1.0)).is_err());
        assert!(set_assignment(&mut w, NationId::USA, u32::MAX, Some(1.0)).is_err());
        set_assignment(&mut w, NationId::USA, first, Some(10.0)).unwrap();
        assert!(set_assignment(&mut w, NationId::USA, second, Some(1.0)).is_err());
        set_assignment(&mut w, NationId::USA, first, Some(0.0)).unwrap();
        let before = save(&w);
        let paused = snapshot(&w, NationId::USA);
        near(paused.projects[0].assigned, 0.0);
        near(paused.projects[1].assigned, 10.0);
        assert_eq!(save(&w), before, "preview must remain read-only");
        let restored = load(&before).unwrap();
        assert_eq!(restored.production.construction_assignments[&first], 0.0);
        assert_eq!(
            serde_json::to_string(&snapshot(&restored, NationId::USA)).unwrap(),
            serde_json::to_string(&paused).unwrap()
        );
        set_assignment(&mut w, NationId::USA, first, None).unwrap();
        near(allocation_for(&w, first).unwrap().assigned, 5.0);
        near(allocation_for(&w, second).unwrap().assigned, 5.0);
        w.districts.insert(district, NationId::UK);
        assert!(set_assignment(&mut w, NationId::USA, first, None).is_err());
        near(snapshot(&w, NationId::USA).assigned_capacity, 0.0);
    }

    #[test]
    fn infrastructure_and_explicit_capacity_use_one_work_rate() {
        let (mut w, district) = prepared();
        let id = production::start_project(
            &mut w,
            NationId::USA,
            &district,
            ProjectKind::OfficeDistrict,
        )
        .unwrap();
        set_assignment(&mut w, NationId::USA, id, Some(10.0)).unwrap();
        near(nominal_work(&w, &w.production.projects[0]), 1.0);
        production::complete_capability(&mut w, &district, ProjectKind::Infrastructure);
        near(nominal_work(&w, &w.production.projects[0]), 1.1);
        set_assignment(&mut w, NationId::USA, id, Some(5.0)).unwrap();
        near(nominal_work(&w, &w.production.projects[0]), 0.55);
        set_assignment(&mut w, NationId::USA, id, Some(0.0)).unwrap();
        near(nominal_work(&w, &w.production.projects[0]), 0.0);
    }

    #[test]
    fn population_crews_cap_work_without_squaring_a_factory_capacity_shortage() {
        let (mut w, district) = prepared();
        crate::population::enable(&mut w);
        production::start_project(&mut w, NationId::USA, &district, ProjectKind::OfficeDistrict).unwrap();
        crate::population::tick(&mut w);
        let p = w.population_system.provinces.get_mut(&district).unwrap();
        for grade in 0..3 { p.filled[4][grade] = (p.jobs[4][grade] + p.project_jobs[4][grade]) * 0.5; }
        near(crew_work_limit(&w, &district), 1.0);
        // Half the site's crew can perform one work day. A smaller physical
        // factory assignment remains the limiting input, without another x0.5.
        near(nominal_work_with_assignment(&w, &w.production.projects[0], 20.0), 1.0);
        near(nominal_work_with_assignment(&w, &w.production.projects[0], 5.0), 0.5);
        w.population_system.provinces.get_mut(&district).unwrap().filled[4][2] = 0.0;
        near(nominal_work_with_assignment(&w, &w.production.projects[0], 20.0), 0.0);
    }

    #[test]
    fn new_capabilities_preserve_the_original_seven_slot_save_schema() {
        let (mut w, district) = prepared();
        for kind in [
            ProjectKind::OfficeDistrict,
            ProjectKind::Shipyard,
            ProjectKind::AdvancedIndustry,
        ] {
            production::complete_capability(&mut w, &district, kind);
            assert_eq!(production::level(&w, &district, kind), 1);
        }
        assert!(w.production.industry.sites.is_empty());
        assert_eq!(w.production.rebuild_sites[&district], [1, 1, 1]);
        assert_eq!(load(&save(&w)).unwrap().production, w.production);
        let legacy: production::Production = serde_json::from_str("{}").unwrap();
        assert!(legacy.is_empty());
        assert_eq!(serde_json::to_string(&legacy).unwrap(), "{}");
    }

    fn add_test_mine(w: &mut WorldState, district: &str) {
        // A synthetic active queue record tests allocation, not deposit data.
        let commodity = resources::Commodity::Iron;
        w.resources.mine_projects.push(resources::MineProject {
            district: district.to_string(),
            commodity,
            started_by: NationId::USA,
            months_left: 3,
            months_total: 3,
            days_left: Some(90),
            investment_bn: 0.1,
            output: 1.0,
        });
        crate::industry::enroll_mine(w, district, commodity, 90);
    }

    #[test]
    fn buildings_and_mines_share_one_pool_even_when_a_building_completes_first() {
        let (mut w, district) = prepared();
        let id = production::start_project(
            &mut w,
            NationId::USA,
            &district,
            ProjectKind::Infrastructure,
        )
        .unwrap();
        add_test_mine(&mut w, &district);
        let commodity = resources::Commodity::Iron;
        let before = save(&w);
        let quote = snapshot(&w, NationId::USA);
        near(quote.projects[0].assigned, 5.0);
        near(quote.mine_assigned_capacity, 5.0);
        near(quote.assigned_capacity, 10.0);
        assert_eq!(save(&w), before, "mine and building preview is pure");
        begin_day(&mut w);
        // Completion removes the queue record before the resource tick. The
        // already-reserved five units cannot become another five units of work.
        w.production.projects.retain(|p| p.id != id);
        near(mine_capacity(&w, NationId::USA, &district, commodity), 5.0);
        near(snapshot(&w, NationId::USA).assigned_capacity, 10.0);
        let restored = load(&save(&w)).unwrap();
        near(
            mine_capacity(&restored, NationId::USA, &district, commodity),
            5.0,
        );
        clock::advance_date(&mut w);
        near(mine_capacity(&w, NationId::USA, &district, commodity), 10.0);
        set_mine_assignment(&mut w, NationId::USA, &district, commodity, Some(0.0)).unwrap();
        near(mine_capacity(&w, NationId::USA, &district, commodity), 0.0);
        assert_eq!(
            load(&save(&w))
                .unwrap()
                .production
                .mine_construction_assignments[&crate::industry::mine_key(&district, commodity)],
            0.0
        );
    }

    #[test]
    fn mine_manual_requests_and_building_manual_requests_cannot_overbook() {
        let (mut w, district) = prepared();
        let id = production::start_project(
            &mut w,
            NationId::USA,
            &district,
            ProjectKind::Infrastructure,
        )
        .unwrap();
        add_test_mine(&mut w, &district);
        let commodity = resources::Commodity::Iron;
        set_mine_assignment(&mut w, NationId::USA, &district, commodity, Some(8.0)).unwrap();
        assert!(set_assignment(&mut w, NationId::USA, id, Some(3.0)).is_err());
        set_assignment(&mut w, NationId::USA, id, Some(2.0)).unwrap();
        assert!(
            set_mine_assignment(&mut w, NationId::USA, &district, commodity, Some(9.0)).is_err()
        );
        near(snapshot(&w, NationId::USA).assigned_capacity, 10.0);
        for invalid in [f64::NAN, f64::INFINITY, -1.0, 20.01] {
            assert!(set_mine_assignment(
                &mut w,
                NationId::USA,
                &district,
                commodity,
                Some(invalid)
            )
            .is_err());
        }
        assert!(set_mine_assignment(&mut w, NationId::UK, &district, commodity, None).is_err());
    }

    #[test]
    fn dated_allocation_commands_and_construction_settlement_replay_after_save() {
        let (mut base, district) = prepared();
        base.rules.resource_market = true;
        crate::programs::set_construction_budget(&mut base, NationId::USA, 0.01).unwrap();
        let id = production::start_project(
            &mut base,
            NationId::USA,
            &district,
            ProjectKind::Infrastructure,
        )
        .unwrap();
        for commodity in resources::ALL {
            if commodity != resources::Commodity::Oil {
                resources::set_stockpile_for_test(&mut base, NationId::USA, commodity, 1_000_000.0);
            }
        }
        let mut uninterrupted = base.clone();
        let mut resumed = base;
        fn run_date(w: &mut WorldState, id: u32, day: usize) {
            if day == 2 || day == 4 {
                crate::apply_command(
                    w,
                    &crate::Command::SetConstructionAllocation {
                        nation: NationId::USA,
                        project: id,
                        capacity: if day == 2 { Some(0.0) } else { None },
                    },
                )
                .unwrap();
            }
            crate::programs::begin_day(w);
            let before = w.production.projects[0].progress_days;
            let planned = production::work_rate_today(w, &w.production.projects[0]);
            let eta = production::estimated_days_left(w, &w.production.projects[0]);
            if day == 2 || day == 3 {
                near(planned, 0.0);
                assert_eq!(eta, None);
            } else {
                assert!(planned > 0.0, "a supplied, funded assignment must build");
                assert_eq!(
                    eta,
                    Some(
                        ((w.production.projects[0].total_days as f64 - before) / planned).ceil()
                            as u32
                    )
                );
            }
            production::tick_day(w);
            near(w.production.projects[0].progress_days - before, planned);
            let once = save(w);
            production::tick_day(w);
            assert_eq!(
                save(w),
                once,
                "a second call cannot spend the same date's capacity"
            );
            crate::programs::stage_fiscal(w.nation_mut(NationId::USA), 0.0, 0.0);
            crate::programs::finish_day(w);
            clock::advance_date(w);
        }
        for day in 0..8 {
            run_date(&mut uninterrupted, id, day);
            run_date(&mut resumed, id, day);
            if day == 3 {
                resumed = load(&save(&resumed)).unwrap();
            }
        }
        assert_eq!(save(&resumed), save(&uninterrupted));
        assert!(uninterrupted.production.projects[0].progress_days > 0.0);
        assert!(uninterrupted.production.industry.projects[&id].spent_bn > 0.0);
    }
}
