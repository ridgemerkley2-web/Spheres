//! Saved establishments refer to aircraft in the national Arsenal. Assigning,
//! resizing or changing an establishment never produces, converts or pays for
//! an aircraft. Legacy formation equivalents retain their existing accounting.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::{
    arsenal, clock, equipment,
    world::{Nation, NationId, WorldState},
};

pub const MAX_SQUADRONS: usize = 1024;
pub const LEGACY_AIRCRAFT_NOTE: &str = "Inherited air holdings are formation equivalents, including exact fractional property. They remain in the Arsenal under their existing rules and cannot be assigned as individual aircraft. No rounding or conversion grants aircraft, research or additional capability.";

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AirTransit {
    pub from: Option<String>,
    pub to: String,
    pub departed_day: i32,
    pub arrival_day: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Squadron {
    pub id: u32,
    pub name: String,
    /// An immutable, certified aircraft revision already owned by the nation.
    pub revision: String,
    /// A claim on whole aircraft in Arsenal, never a second physical stock.
    /// Losses or an explicit resize can leave an empty establishment.
    pub assigned: u32,
    pub base: Option<String>,
    pub transit: Option<AirTransit>,
    pub service_days_left: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AviationState {
    pub next_id: u32,
    pub squadrons: Vec<Squadron>,
    pub last_service_day: Option<i32>,
}

impl Default for AviationState {
    fn default() -> Self {
        Self {
            next_id: 1,
            squadrons: Vec::new(),
            last_service_day: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub enum SquadronCommand {
    Create {
        name: String,
        revision: String,
        quantity: u32,
    },
    Resize {
        squadron: u32,
        quantity: u32,
    },
    ChangeRevision {
        squadron: u32,
        revision: String,
    },
    Disband {
        squadron: u32,
    },
}

/// Each row keeps the original fractional quantity, without aggregating or
/// rounding it into a misleading whole-aircraft figure.
#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct LegacyAircraftHolding {
    pub kit: String,
    pub name: String,
    pub formation_equivalents: f64,
    pub age_months: f64,
}

pub fn legacy_summary(n: &Nation) -> Vec<LegacyAircraftHolding> {
    n.arsenal
        .held
        .iter()
        .filter(|h| h.design_id.is_none())
        .filter_map(|h| {
            let def = arsenal::DECK.get(h.kit as usize)?;
            (def.class == arsenal::Class::Air).then(|| LegacyAircraftHolding {
                kit: def.id.into(),
                name: def.name.into(),
                formation_equivalents: h.units,
                age_months: h.age,
            })
        })
        .collect()
}

fn aircraft_revision(n: &Nation, revision: &str) -> bool {
    n.equipment
        .as_ref()
        .and_then(|s| s.revisions.get(revision))
        .is_some_and(|r| {
            r.certified_day.is_some()
                && r.profile.aviation.is_some()
                && equipment::is_aviation_platform(&r.spec.platform)
        })
}

/// Delivered whole aircraft after existing refit reservations. Pending
/// purchases, legacy formations and uncertified models never enter this pool.
fn physical_units(n: &Nation, revision: &str) -> u32 {
    if !aircraft_revision(n, revision) {
        return 0;
    }
    n.arsenal
        .held
        .iter()
        .filter(|h| h.design_id.as_deref() == Some(revision))
        .fold(0u32, |sum, h| {
            sum.saturating_add(arsenal::available_design_units(h))
        })
}

pub fn assigned_units(n: &Nation, revision: &str) -> u32 {
    n.aviation.as_ref().map_or(0, |s| {
        s.squadrons
            .iter()
            .filter(|q| q.revision == revision)
            .fold(0u32, |sum, q| sum.saturating_add(q.assigned))
    })
}

/// The only aircraft that a new establishment, refit or retirement can claim.
/// Ground-equipment callers should continue using their existing availability.
pub fn unassigned_units(n: &Nation, revision: &str) -> u32 {
    physical_units(n, revision).saturating_sub(assigned_units(n, revision))
}

/// Physical eligibility before geographic access, capacity, range, maintenance
/// coverage and compatible stores. Even an overdue transit remains unavailable
/// until the basing scheduler records arrival; this read never finishes work.
pub fn ready_units(n: &Nation, revision: &str, _day: i32) -> u32 {
    let assigned = n.aviation.as_ref().map_or(0, |s| {
        s.squadrons
            .iter()
            .filter(|q| {
                q.revision == revision
                    && q.base.is_some()
                    && q.transit.is_none()
                    && q.service_days_left == 0
            })
            .fold(0u32, |sum, q| sum.saturating_add(q.assigned))
    });
    assigned.min(physical_units(n, revision))
}

fn valid_name(name: &str) -> bool {
    !name.trim().is_empty() && name.chars().count() <= 80 && !name.chars().any(char::is_control)
}

fn validate_nation(w: &WorldState, n: &Nation) -> Result<(), String> {
    let Some(state) = &n.aviation else {
        return Ok(());
    };
    let fail = |detail: &str| format!("Invalid aviation state for {}: {detail}", n.id.name());
    if state.next_id == 0 || state.squadrons.len() > MAX_SQUADRONS {
        return Err(fail(
            "invalid squadron identity counter or establishment limit",
        ));
    }
    let today = clock::absolute_day(w);
    if state.last_service_day.is_some_and(|day| day > today) {
        return Err(fail(
            "routine support is dated after the current campaign day",
        ));
    }
    equipment::validate_state(n)?;
    let mut ids = BTreeSet::new();
    let mut assigned = BTreeMap::<&str, u32>::new();
    for q in &state.squadrons {
        if q.id == 0 || q.id >= state.next_id || !ids.insert(q.id) || !valid_name(&q.name) {
            return Err(fail("invalid or duplicate squadron identity"));
        }
        if !aircraft_revision(n, &q.revision) {
            return Err(fail(
                "a squadron does not reference a certified aircraft revision",
            ));
        }
        if q.base
            .as_ref()
            .is_some_and(|base| !w.districts.contains_key(base))
        {
            return Err(fail("a squadron base is not a known province"));
        }
        if let Some(t) = &q.transit {
            if !w.districts.contains_key(&t.to)
                || t.from
                    .as_ref()
                    .is_some_and(|base| !w.districts.contains_key(base))
                || t.from.as_ref() == Some(&t.to)
                || q.base
                    .as_ref()
                    .is_some_and(|base| t.from.as_ref() != Some(base))
                || t.departed_day > today
                || t.arrival_day <= t.departed_day
            {
                return Err(fail("invalid aircraft transit route or dates"));
            }
        }
        let total = assigned.entry(&q.revision).or_default();
        *total = total
            .checked_add(q.assigned)
            .ok_or_else(|| fail("aircraft assignments overflow their physical inventory"))?;
    }
    for (revision, total) in assigned {
        if total > physical_units(n, revision) {
            return Err(fail(
                "squadrons claim more aircraft than delivered, unreserved holdings",
            ));
        }
    }
    Ok(())
}

/// Loading validates ownership rather than silently shrinking malformed saves.
/// `reconcile` is called explicitly after a real physical loss settlement.
pub fn validate(w: &WorldState) -> Result<(), String> {
    for n in &w.nations {
        validate_nation(w, n)?;
    }
    Ok(())
}

fn editable_squadron<'a>(w: &WorldState, n: &'a Nation, id: u32) -> Result<&'a Squadron, String> {
    let q = n
        .aviation
        .as_ref()
        .and_then(|s| s.squadrons.iter().find(|q| q.id == id))
        .ok_or("This squadron no longer exists. Refresh the aircraft roster.")?;
    if crate::airmissions::busy(w, n.id, id) {
        return Err("This squadron has a queued mission. Cancel that order or wait for its dated result before changing or disbanding the squadron.".into());
    }
    if q.transit.is_some() {
        return Err("This squadron is rebasing. Wait for its recorded arrival before changing its aircraft or disbanding it.".into());
    }
    if q.service_days_left > 0 {
        return Err("This squadron is undergoing routine service. Fund support and wait for service to finish before changing its aircraft or disbanding it.".into());
    }
    Ok(q)
}

fn command_refusal(w: &WorldState, id: NationId, command: &SquadronCommand) -> Result<(), String> {
    if let Some(reason) = equipment::actor_refusal(w, id) {
        return Err(reason);
    }
    let n = w.nation(id);
    equipment::validate_state(n)?;
    validate_nation(w, n)?;
    match command {
        SquadronCommand::Create {
            name,
            revision,
            quantity,
        } => {
            if !valid_name(name) {
                return Err(
                    "Use a squadron name of 1–80 characters without control characters.".into(),
                );
            }
            if n.aviation
                .as_ref()
                .is_some_and(|s| s.squadrons.len() >= MAX_SQUADRONS || s.next_id == u32::MAX)
            {
                return Err("This campaign's squadron establishment roster is full.".into());
            }
            if !aircraft_revision(n, revision) {
                return Err("Choose a certified physical aircraft revision. Inherited fractional air formations cannot be assigned as individual aircraft.".into());
            }
            if *quantity == 0 || *quantity > unassigned_units(n, revision) {
                return Err("Choose at least one delivered, unassigned aircraft. Pending deliveries, other squadrons and refit reservations are unavailable.".into());
            }
        }
        SquadronCommand::Resize { squadron, quantity } => {
            let q = editable_squadron(w, n, *squadron)?;
            let available = unassigned_units(n, &q.revision).saturating_add(q.assigned);
            if *quantity > available {
                return Err("There are not enough delivered, unassigned aircraft of this exact revision. Reduce the establishment or wait for delivery or refit completion.".into());
            }
            if let Some(base) = q.base.as_deref() {
                if let Some(reason) =
                    crate::airbases::assignment_refusal(w, id, Some(q.id), base, *quantity)
                {
                    return Err(reason);
                }
            }
        }
        SquadronCommand::ChangeRevision { squadron, revision } => {
            let q = editable_squadron(w, n, *squadron)?;
            if !aircraft_revision(n, revision) {
                return Err("Choose another certified physical aircraft revision already in the national design library.".into());
            }
            if revision != &q.revision && q.assigned > unassigned_units(n, revision) {
                return Err("The replacement revision has too few delivered, unassigned aircraft. Buy and receive them, or reduce this squadron first. Changing revision does not refit or create aircraft.".into());
            }
            if let Some(base) = q.base.as_deref() {
                if let Some(reason) =
                    crate::airbases::assignment_refusal(w, id, Some(q.id), base, q.assigned)
                {
                    return Err(reason);
                }
            }
        }
        SquadronCommand::Disband { squadron } => {
            editable_squadron(w, n, *squadron)?;
        }
    }
    Ok(())
}

pub fn refusal(w: &WorldState, id: NationId, command: &SquadronCommand) -> Option<String> {
    command_refusal(w, id, command).err()
}

/// The ordinary command layer owns review tokens and response replay. This
/// commit validates all conditions before changing the establishment ledger.
pub fn apply(w: &mut WorldState, id: NationId, command: &SquadronCommand) -> Result<(), String> {
    command_refusal(w, id, command)?;
    let state = w
        .nation_mut(id)
        .aviation
        .get_or_insert_with(AviationState::default);
    match command {
        SquadronCommand::Create {
            name,
            revision,
            quantity,
        } => {
            state.squadrons.push(Squadron {
                id: state.next_id,
                name: name.trim().into(),
                revision: revision.clone(),
                assigned: *quantity,
                base: None,
                transit: None,
                service_days_left: 0,
            });
            state.next_id += 1;
        }
        SquadronCommand::Resize { squadron, quantity } => {
            state
                .squadrons
                .iter_mut()
                .find(|q| q.id == *squadron)
                .unwrap()
                .assigned = *quantity;
        }
        SquadronCommand::ChangeRevision { squadron, revision } => {
            state
                .squadrons
                .iter_mut()
                .find(|q| q.id == *squadron)
                .unwrap()
                .revision = revision.clone();
        }
        SquadronCommand::Disband { squadron } => {
            state.squadrons.retain(|q| q.id != *squadron);
        }
    }
    Ok(())
}

/// Reconcile claims after one real physical loss settlement. Existing refit
/// reservations are deducted first. Highest squadron IDs lose claims first;
/// moving or servicing units are preserved before idle units. Callers must
/// already have excluded protected aircraft from the physical casualty pool.
/// Repeating this function is inert, and new deliveries never refill squads.
pub fn reconcile(n: &mut Nation) {
    let Some(state) = n.aviation.as_ref() else {
        return;
    };
    let mut totals = BTreeMap::<String, u32>::new();
    for q in &state.squadrons {
        let total = totals.entry(q.revision.clone()).or_default();
        *total = total.saturating_add(q.assigned);
    }
    let excess: Vec<_> = totals
        .into_iter()
        .map(|(revision, assigned)| {
            let amount = assigned.saturating_sub(physical_units(n, &revision));
            (revision, amount)
        })
        .filter(|(_, amount)| *amount > 0)
        .collect();
    let state = n.aviation.as_mut().unwrap();
    for (revision, mut remaining) in excess {
        let mut candidates: Vec<_> = state
            .squadrons
            .iter()
            .enumerate()
            .filter(|(_, q)| q.revision == revision)
            .map(|(index, q)| {
                (
                    q.transit.is_some() || q.service_days_left > 0,
                    std::cmp::Reverse(q.id),
                    index,
                )
            })
            .collect();
        candidates.sort_unstable();
        for (_, _, index) in candidates {
            let q = &mut state.squadrons[index];
            let removed = remaining.min(q.assigned);
            q.assigned -= removed;
            remaining -= removed;
            if remaining == 0 {
                break;
            }
        }
    }
}

#[cfg(test)]
#[path = "aviation_tests.rs"]
mod tests;
