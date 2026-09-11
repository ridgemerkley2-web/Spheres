//! Combined company capability without combining the property of unrelated firms.
use crate::{clock, connected_economy, population, sector_contractors, supplier_operations,
    world::{NationId, WorldState}};

pub fn enrollment_refusal(w: &WorldState, nation: NationId) -> Option<String> {
    connected_economy::daily_enrollment_refusal(w, nation).or_else(|| {
        if !w.rules.industry_rebuild || !population::active(w) || !w.rules.production_system {
            Some("Adopt the connected economy before connecting company operations.".into())
        } else if !w.rules.resource_market || w.resources.market.is_none() {
            Some("Company operations require an existing physical resource market.".into())
        } else { None }
    })
}

/// Called explicitly for a fresh campaign or a reviewed player upgrade. Loading
/// does not call this function, seed identities, settle invoices or advance work.
pub fn enable(w: &mut WorldState) -> Result<(), String> {
    if !clock::is_daily(w) || !w.rules.industry_rebuild || !population::active(w) || !w.rules.production_system {
        return Err("The company network requires the connected daily economy.".into());
    }
    connected_economy::validate(w)?;
    crate::companies::validate_state(w)?;
    let mut staged = w.clone();
    sector_contractors::enable(&mut staged);
    supplier_operations::enable(&mut staged)?;
    validate(&staged)?;
    *w = staged;
    Ok(())
}

pub fn has_state(w: &WorldState) -> bool {
    !w.sector_contractors.is_empty() || !w.supplier_operations.is_empty()
        || w.production.industry.projects.values().any(|p| p.company_fees_bn != 0.0)
        || w.nations.iter().filter_map(|n| n.equipment.as_ref()).any(|e|
            e.projects.iter().any(|p| p.company_fees_bn != 0.0 || p.company_inputs_saved.iter().any(|v| *v != 0.0)))
}

pub fn validate(w: &WorldState) -> Result<(), String> {
    if w.production.industry.projects.values().any(|f| !f.company_fees_bn.is_finite()
        || f.company_fees_bn < 0.0 || f.company_fees_bn > f.spent_bn + 1e-9) {
        return Err("Construction service fees must be finite, paid receipts within the earned base contract.".into());
    }
    sector_contractors::validate(w)?;
    supplier_operations::validate(w)?;
    Ok(())
}

/// The web adapter supplies the existing detailed, reviewed supplier actions.
/// Directory references distinguish supplier and contractor numeric namespaces.
pub fn view(w: &WorldState, nation: NationId) -> serde_json::Value {
    let directory=sector_contractors::directory(w,nation);
    serde_json::json!({
        "enabled":w.sector_contractors.enabled && supplier_operations::enabled(w),
        "enable_reason":enrollment_refusal(w,nation),
        "directory":directory["rows"],
        "contractors":directory["contractors"],
        "operations":supplier_operations::snapshot(w,nation),
    })
}
