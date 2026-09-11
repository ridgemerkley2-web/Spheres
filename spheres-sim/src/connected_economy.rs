//! Explicit adoption boundary for the integrated daily economy.
//! Saved balances and paid construction/supplier property remain authoritative.
use crate::{clock, fiscal_recovery, population, world::{NationId, WorldState}};

pub fn enrollment_refusal(w: &WorldState, nation: NationId) -> Option<String> {
    if let Some(reason) = daily_enrollment_refusal(w, nation) { return Some(reason); }
    if !w.rules.production_system {
        return Some("Enable production and construction before adopting the connected economy.".into());
    }
    None
}

pub(crate) fn daily_enrollment_refusal(w: &WorldState, nation: NationId) -> Option<String> {
    if w.player != Some(nation) || !w.nation_opt(nation).is_some_and(|n| n.alive) {
        return Some("Only the living player government can upgrade this campaign's economy.".into());
    }
    if !clock::is_daily(w) {
        return Some("The connected economy requires daily simulation.".into());
    }
    None
}

/// Also used explicitly during fresh campaign creation, before selecting a player.
/// Stage the whole adoption: a malformed population book cannot half-upgrade cash.
pub fn enable(w: &mut WorldState) -> Result<(), String> {
    if !clock::is_daily(w) { return Err("The connected economy requires daily simulation.".into()); }
    if !w.rules.production_system {
        return Err("Enable production and construction before adopting the connected economy.".into());
    }
    let mut candidate = w.clone();
    candidate.rules.industry_rebuild = true;
    population::enable(&mut candidate)?;
    fiscal_recovery::enable(&mut candidate);
    validate(&candidate)?;
    *w = candidate;
    Ok(())
}

pub fn has_state(w: &WorldState) -> bool {
    w.rules.industry_rebuild || w.rules.fiscal_recovery
        || !w.population_system.is_empty() || !w.fiscal_recovery.is_empty()
        || crate::industry_operations::has_state(w)
        || w.nations.iter().any(|n| n.population_outcomes.is_some())
}

pub fn validate(w: &WorldState) -> Result<(), String> {
    population::validate(w)?;
    fiscal_recovery::validate(w)?;
    crate::industry_operations::validate(w)?;
    Ok(())
}
