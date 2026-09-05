//! Physical control is distinct from sovereignty. Fronts do not transfer GDP,
//! population, warehouses or industrial ownership; settlements still do that.
use crate::{front::HELD_BAND, world::{NationId, WorldState}};

/// None means contested or contradictory military control. Friendly occupation
/// preserves the legal owner's operating authority. Conflicting hostile claims
/// are contested, independent of conflict-vector order.
pub fn controller(w: &WorldState, district: &str) -> Option<NationId> {
    let owner = *w.districts.get(district)?;
    if !w.rules.military_operations { return Some(owner); }
    let mut hostile = None;
    for c in &w.conflicts {
        let Some(owner_side) = c.side_of(owner) else { continue };
        let Some(&hold) = c.front.get(district) else { continue };
        if !hold.is_finite() || (hold as f64).abs() <= HELD_BAND { return None; }
        let held_side = hold > 0.0;
        if held_side == owner_side { continue; }
        let side = if held_side { &c.side_a } else { &c.side_b };
        let Some(occupier) = side.iter().copied().find(|id| w.nation_opt(*id).is_some_and(|n| n.alive)) else { continue };
        if hostile.is_some_and(|id| id != occupier) { return None; }
        hostile = Some(occupier);
    }
    Some(hostile.unwrap_or(owner))
}

pub fn can_operate(w: &WorldState, nation: NationId, district: &str) -> bool {
    w.districts.get(district) == Some(&nation) && controller(w, district) == Some(nation)
}

pub fn blocker(w: &WorldState, nation: NationId, district: &str) -> Option<String> {
    if w.districts.get(district) != Some(&nation) {
        return Some("The sponsoring government no longer owns this province.".into());
    }
    match controller(w, district) {
        Some(id) if id == nation => None,
        Some(id) => Some(format!("{} holds this province militarily; operations resume after recapture.", id.name())),
        None => Some("The province is contested; operations resume after control is restored.".into()),
    }
}
