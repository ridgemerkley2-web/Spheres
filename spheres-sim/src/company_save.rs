//! Classify incompatible company dialects before serde can discard ownership.
use crate::{companies, company_network, connected_economy, equipment_save_version,
    world::WorldState};
use serde_json::Value;

pub(crate) fn decode(s: &str) -> Result<WorldState, String> {
    let shape: Value = serde_json::from_str(s).map_err(|e| e.to_string())?;
    let format = shape.get("format").and_then(Value::as_str);
    let combined = format == Some("spheres-companies-save");
    let economy = format == Some("spheres-economy-save")
        || (combined && shape["economy_version"].as_u64() == Some(1));
    let party = format == Some("spheres-party-leadership-save")
        || ((combined || format == Some("spheres-economy-save"))
            && shape["party_leadership_version"].as_u64() == Some(1));
    let equipment = match format {
        Some("spheres-companies-save" | "spheres-economy-save" | "spheres-party-leadership-save") => shape["equipment_version"].as_u64(),
        Some("spheres-equipment-save") => shape["version"].as_u64(),
        None => Some(0),
        _ => None,
    };
    let valid = match format {
        Some("spheres-companies-save") => shape["version"] == 1
            && matches!(shape["economy_version"].as_u64(), Some(0..=1))
            && matches!(shape["supplier_operations_version"].as_u64(), Some(0..=1))
            && matches!(shape["party_leadership_version"].as_u64(), Some(0..=1))
            && matches!(equipment, Some(0..=5)),
        Some("spheres-economy-save") => shape["version"] == 1
            && matches!(shape["party_leadership_version"].as_u64(), Some(0..=1))
            && matches!(equipment, Some(0..=5)),
        Some("spheres-party-leadership-save") => shape["version"] == 1 && matches!(equipment, Some(0..=5)),
        Some("spheres-equipment-save") => matches!(equipment, Some(1..=5)),
        None => shape.get("format").is_none(),
        _ => false,
    };
    if !valid { return Err("This save format or version is not supported by this build.".into()); }
    let mut payload = if format.is_some() { shape["world"].clone() } else { shape.clone() };
    if !payload.is_object() { return Err("The saved campaign must be an object.".into()); }
    if payload["rules"].get("operational_warfare").is_some_and(|v| !v.is_null() && *v != false && v.as_u64() != Some(0))
        || ["campaign", "campaign_supply", "campaign_peace"].iter().any(|k|
            payload.get(*k).is_some_and(|v| !v.is_null() && !v.as_object().is_some_and(|o| o.is_empty()))) {
        return Err("This campaign contains operational-war property. Integrate that save dialect in S04/S05 before adopting it; no military ownership was discarded.".into());
    }
    let contractor_keys = ["enabled", "roster", "assignments", "growth", "news", "next_id", "last_day", "last_month"];
    let supplier_keys = ["version", "next_id", "firms", "deliveries", "ammunition_deliveries", "last_tick_day"];
    let old = payload.get("companies");
    let master = old.is_some_and(|b| ["enabled", "roster", "assignments", "growth", "news", "last_day", "last_month"].iter().any(|k| b.get(*k).is_some()));
    if let Some(book) = old {
        let keys = book.as_object().ok_or("The company book has an unrecognized ownership shape.")?;
        let allowed = if master { &contractor_keys[..] } else { &supplier_keys[..] };
        if keys.keys().any(|k| !allowed.contains(&k.as_str())) {
            return Err("Mixed or unknown company property cannot be migrated. Keep supplier assets and service contractor identities in separate books.".into());
        }
    }
    if master {
        // Only the pinned master's complete, original dialect is recognized.
        // A newer wrapper cannot use an old field name to bypass its capability.
        if !matches!(format, None | Some("spheres-equipment-save"))
            || equipment.is_some_and(|v| v > 1)
            || contractor_keys.iter().any(|k| payload["companies"].get(*k).is_none())
            || payload.get("sector_contractors").is_some()
            || payload.get("supplier_operations").is_some() {
            return Err("Ambiguous contractor migration: use the complete original roster save or the versioned combined company format.".into());
        }
        let roster = payload.as_object_mut().unwrap().remove("companies").unwrap();
        // Validate its exact shape, including every nested identity and receipt.
        let _: crate::sector_contractors::Companies = serde_json::from_value(roster.clone()).map_err(|e| format!("Contractor migration: {e}"))?;
        payload["sector_contractors"] = roster;
    }
    let w: WorldState = serde_json::from_value(payload).map_err(|e| e.to_string())?;
    if party != (w.rules.historical_party_leadership && w.party_leadership.is_some())
        || (!party && w.party_leadership.is_some()) {
        return Err("Campaign party identities require their enabled rule, saved book and matching save envelope.".into());
    }
    let envelope = equipment.unwrap();
    if matches!(format, Some("spheres-companies-save" | "spheres-economy-save" | "spheres-party-leadership-save"))
        && envelope != equipment_save_version(&w) as u64 {
        return Err("The campaign has an incorrect equipment format version.".into());
    }
    let expected = match w.companies.version {
        companies::TANK_VERSION => 2, companies::EQUIPMENT_VERSION => 3,
        companies::AMMUNITION_VERSION => 4, companies::VERSION => 5, _ => 0,
    };
    if (!w.companies.is_empty() && (expected == 0 || envelope != expected))
        || (w.companies.is_empty() && envelope >= 2) {
        return Err("Company property requires its matching tank, equipment, ammunition or refit-service save envelope; refusing to discard or silently downgrade corporate assets.".into());
    }
    if economy != connected_economy::has_state(&w) && !master {
        return Err("Connected economy state requires its versioned economy save envelope. Refusing to discard or silently enable economic ownership.".into());
    }
    if combined != company_network::has_state(&w) && !master {
        return Err("Company identities, operating contracts and service receipts require their versioned combined company save envelope.".into());
    }
    if combined && shape["supplier_operations_version"].as_u64() != Some(w.supplier_operations.version as u64) {
        return Err("Supplier operating property requires its declared capability version.".into());
    }
    Ok(w)
}
