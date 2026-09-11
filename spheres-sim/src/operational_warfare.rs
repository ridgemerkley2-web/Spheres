//! Explicit adoption and save capability for the integrated operational path.
use crate::{campaign,campaign_supply,campaign_peace,clock,world::{NationId,WorldState}};

pub fn has_state(w:&WorldState)->bool {
    w.rules.operational_warfare!=0 || !w.campaign.is_empty()
        || !w.campaign_supply.is_empty() || !w.campaign_peace.is_empty()
}
pub fn enrollment_refusal(w:&WorldState,nation:NationId)->Option<String> {
    if w.player!=Some(nation) || !w.nation_opt(nation).is_some_and(|n|n.alive) {
        return Some("Only the living player government can adopt operational warfare.".into());
    }
    if !clock::is_daily(w) || !w.rules.military_operations {
        return Some("Operational warfare requires daily play and the military operations system.".into());
    }
    if w.rules.operational_warfare>1 {
        return Some("This campaign has an unsupported operational warfare version.".into());
    }
    None
}
/// Fresh-world setup may call this before a player is selected. Loading never
/// calls it: existing fronts and one-time enrollment IDs are saved ownership.
pub fn enable(w:&mut WorldState)->Result<(),String> {
    if !clock::is_daily(w)||!w.rules.military_operations||w.rules.operational_warfare>1 {
        return Err("Operational warfare requires supported daily military operations.".into());
    }
    if w.rules.operational_warfare==1 {return validate(w);}
    if !w.campaign.is_empty()||!w.campaign_supply.is_empty()||!w.campaign_peace.is_empty() {
        return Err("Undeclared operational property must be migrated before enabling this campaign.".into());
    }
    let mut staged=w.clone();staged.rules.operational_warfare=1;
    campaign::enroll(&mut staged);validate(&staged)?;*w=staged;Ok(())
}
pub fn validate(w:&WorldState)->Result<(),String> {
    if has_state(w)&&!campaign::enabled(w) {
        return Err("Saved operational orders, forces, supplies and peace require their enabled daily warfare capability.".into());
    }
    campaign::validate(w)?;campaign_supply::validate(w)?;campaign_peace::validate(w)?;
    Ok(())
}
