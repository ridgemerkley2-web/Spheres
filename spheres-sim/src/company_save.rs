//! Classify incompatible company dialects before serde can discard ownership.
use crate::{companies, company_network, connected_economy, equipment_save_version,
    world::WorldState};
use serde_json::Value;

pub(crate) fn decode(s: &str) -> Result<WorldState, String> {
    decode_value(serde_json::from_str(s).map_err(|e| e.to_string())?)
}

pub(crate) fn decode_value(mut shape: Value) -> Result<WorldState, String> {
    // Retain only the small envelope metadata. Moving the payload into serde
    // avoids keeping a second complete JSON world alive during typed decoding.
    let format_name = shape.get("format").and_then(Value::as_str).map(str::to_owned);
    let format = format_name.as_deref();
    let integrated = format == Some("spheres-integrated-save");
    let combined = format == Some("spheres-companies-save")
        || (integrated && shape["company_network_version"].as_u64() == Some(1));
    let economy = format == Some("spheres-economy-save")
        || ((combined || integrated) && shape["economy_version"].as_u64() == Some(1));
    let party = format == Some("spheres-party-leadership-save")
        || ((combined || integrated || format == Some("spheres-economy-save"))
            && shape["party_leadership_version"].as_u64() == Some(1));
    let equipment = match format {
        Some("spheres-integrated-save" | "spheres-companies-save" | "spheres-economy-save" | "spheres-party-leadership-save") => shape["equipment_version"].as_u64(),
        Some("spheres-equipment-save") => shape["version"].as_u64(),
        None => Some(0),
        _ => None,
    };
    let valid = match format {
        Some("spheres-integrated-save") => shape["version"] == 1
            && shape["warfare_version"] == 1
            && matches!(shape["company_network_version"].as_u64(), Some(0..=1))
            && matches!(shape["economy_version"].as_u64(), Some(0..=1))
            && matches!(shape["supplier_operations_version"].as_u64(), Some(0..=1))
            && matches!(shape["party_leadership_version"].as_u64(), Some(0..=1))
            && matches!(equipment, Some(0..=7)),
        Some("spheres-companies-save") => shape["version"] == 1
            && matches!(shape["economy_version"].as_u64(), Some(0..=1))
            && matches!(shape["supplier_operations_version"].as_u64(), Some(0..=1))
            && matches!(shape["party_leadership_version"].as_u64(), Some(0..=1))
            && matches!(equipment, Some(0..=7)),
        Some("spheres-economy-save") => shape["version"] == 1
            && matches!(shape["party_leadership_version"].as_u64(), Some(0..=1))
            && matches!(equipment, Some(0..=7)),
        Some("spheres-party-leadership-save") => shape["version"] == 1 && matches!(equipment, Some(0..=7)),
        Some("spheres-equipment-save") => matches!(equipment, Some(1..=7)),
        None => shape.get("format").is_none(),
        _ => false,
    };
    if !valid { return Err("This save format or version is not supported by this build.".into()); }
    let supplier_operations_version = shape["supplier_operations_version"].as_u64();
    let mut payload = if format.is_some() {
        let payload = shape.get_mut("world").map(Value::take).unwrap_or(Value::Null);
        drop(shape);
        payload
    } else {
        shape
    };
    if !payload.is_object() { return Err("The saved campaign must be an object.".into()); }
    // Serde also accepts sequences for defaulted structs. Military books have
    // object identities, so classify their outer shape before typed decoding.
    // The older empty object/null representation contains no ownership to migrate.
    for key in ["campaign", "campaign_supply", "campaign_peace"] {
        if let Some(book) = payload.get(key) {
            if book.is_null() || book.as_object().is_some_and(|fields|fields.is_empty()) {
                payload.as_object_mut().unwrap().remove(key);
            }
            else if !book.is_object() {
                return Err(format!("The saved {key} book must be an object or empty null."));
            }
        }
    }
    if payload.get("military_ai").is_some_and(|v| !v.is_object()) {
        return Err("The saved military staff book must be an object.".into());
    }
    let contractor_keys = ["enabled", "roster", "assignments", "growth", "news", "next_id", "last_day", "last_month"];
    let supplier_keys = ["version", "next_id", "firms", "deliveries", "ammunition_deliveries", "last_tick_day", "imports"];
    let old = payload.get("companies");
    let master = old.is_some_and(|b| ["enabled", "roster", "assignments", "growth", "news", "last_day", "last_month"].iter().any(|k| b.get(*k).is_some()));
    // The pinned master wrote operational books in raw worlds or equipment v1.
    // It never owned the active supplier book or the later company namespaces.
    let master_warfare = payload["rules"]["operational_warfare"].as_u64() == Some(1)
        && matches!(format, None | Some("spheres-equipment-save"))
        && equipment.is_some_and(|v| v <= 1)
        && payload.get("sector_contractors").is_none()
        && payload.get("supplier_operations").is_none()
        && (old.is_none() || master);
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
    // Presence belongs to the wire contract: an explicit zero is an assertion,
    // never permission to repair a corrupt expense classification.
    let unclassified_inputs: std::collections::BTreeSet<u32> = payload
        .get("supplier_operations").and_then(|s|s.get("contracts"))
        .and_then(Value::as_object).into_iter().flatten()
        .filter(|(_,t)|t.get("preproduction_inputs_bn").is_none())
        .filter_map(|(id,_)|id.parse().ok()).collect();
    let mut w: WorldState = serde_json::from_value(payload).map_err(|e| e.to_string())?;
    crate::supplier_operations::retain_unclassified_preproduction_inputs(&mut w, &unclassified_inputs)?;
    if master || master_warfare {
        crate::fiscal_recovery::retain_original_master_receipts(&mut w)?;
    }
    if party != (w.rules.historical_party_leadership && w.party_leadership.is_some())
        || (!party && w.party_leadership.is_some()) {
        return Err("Campaign party identities require their enabled rule, saved book and matching save envelope.".into());
    }
    let envelope = equipment.unwrap();
    if matches!(format, Some("spheres-integrated-save" | "spheres-companies-save" | "spheres-economy-save" | "spheres-party-leadership-save"))
        && envelope != equipment_save_version(&w) as u64 {
        return Err("The campaign has an incorrect equipment format version.".into());
    }
    let expected = if !w.military_ai.is_empty() { 7 } else if !w.supplier_catalogue.is_empty() { 6 } else { match w.companies.version {
        companies::TANK_VERSION => 2, companies::EQUIPMENT_VERSION => 3,
        companies::AMMUNITION_VERSION => 4, companies::VERSION => 5, companies::IMPORT_VERSION => 6, _ => 0,
    }};
    let supplier_capability = !w.companies.is_empty() || !w.supplier_catalogue.is_empty() || !w.military_ai.is_empty();
    if (supplier_capability && (expected == 0 || envelope != expected))
        || (!supplier_capability && envelope >= 2) {
        return Err("Company property requires its matching tank, equipment, ammunition, refit-service or import save envelope; refusing to discard or silently downgrade corporate assets.".into());
    }
    if economy != connected_economy::has_state(&w) && !master && !master_warfare {
        return Err("Connected economy state requires its versioned economy save envelope. Refusing to discard or silently enable economic ownership.".into());
    }
    if combined != company_network::has_state(&w) && !master {
        return Err("Company identities, operating contracts and service receipts require their versioned combined company save envelope.".into());
    }
    if (combined || integrated) && supplier_operations_version != Some(w.supplier_operations.version as u64) {
        return Err("Supplier operating property requires its declared capability version.".into());
    }
    if integrated != crate::operational_warfare::has_state(&w) && !master_warfare {
        return Err("Operational orders, forces, supplies and peace require their integrated save capability or the recognized original master dialect.".into());
    }
    Ok(w)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::{GameRules, NationId};
    use serde_json::json;

    #[test]
    fn consumed_save_tree_preserves_each_capability_envelope_and_refusal() {
        let mut w = crate::init::world_1990(GameRules {
            daily_simulation: true, military_operations: true, ideology_blocs: true,
            production_system: true, manufacturing_system: true, resource_market: true,
            ..Default::default()
        });
        w.player = Some(NationId::France);
        for expected in [None, Some("spheres-equipment-save"), Some("spheres-party-leadership-save"),
            Some("spheres-economy-save"), Some("spheres-companies-save"), Some("spheres-integrated-save")] {
            match expected {
                Some("spheres-equipment-save") => w.nation_mut(NationId::France).equipment = Some(Default::default()),
                Some("spheres-party-leadership-save") => crate::party_leadership::enable_campaign(&mut w).unwrap(),
                Some("spheres-economy-save") => crate::connected_economy::enable(&mut w).unwrap(),
                Some("spheres-companies-save") => {
                    crate::resources::tick(&mut w);
                    crate::company_network::enable(&mut w).unwrap();
                }
                Some("spheres-integrated-save") => crate::operational_warfare::enable(&mut w).unwrap(),
                _ => {}
            }
            let saved = crate::save(&w);
            let tree: Value = serde_json::from_str(&saved).unwrap();
            assert_eq!(tree.get("format").and_then(Value::as_str), expected);
            assert_eq!(crate::save(&crate::load_value(tree.clone()).unwrap()), saved);
            assert_eq!(crate::save(&crate::load(&saved).unwrap()), saved);
            if expected.is_some() {
                for key in ["version", "equipment_version", "party_leadership_version", "economy_version",
                    "company_network_version", "supplier_operations_version", "warfare_version"] {
                    if tree.get(key).is_none() { continue; }
                    let mut bad = tree.clone();
                    bad[key] = json!(999);
                    let original_error = crate::load(&bad.to_string()).unwrap_err();
                    assert_eq!(crate::load_value(bad).unwrap_err(), original_error, "{expected:?}: {key}");
                }
            }
        }
    }

    #[test]
    fn consumed_save_tree_keeps_legacy_migrations_and_shape_errors() {
        let mut legacy = serde_json::to_value(crate::init::world_1990(GameRules::default())).unwrap();
        for field in ["districts", "district_population", "district_population_scale", "theatres"] {
            legacy.as_object_mut().unwrap().remove(field);
        }
        let expected = crate::load(&legacy.to_string()).unwrap();
        let migrated = crate::load_value(legacy.clone()).unwrap();
        assert!(!migrated.districts.is_empty());
        assert!(!migrated.theatres.is_empty());
        assert_eq!(crate::save(&migrated), crate::save(&expected));
        for (key, value, error) in [
            ("format", Value::Null, "This save format or version is not supported by this build."),
            ("campaign", json!([]), "The saved campaign book must be an object or empty null."),
            ("military_ai", json!([]), "The saved military staff book must be an object."),
            ("companies", json!({"unknown": 0}), "Mixed or unknown company property cannot be migrated. Keep supplier assets and service contractor identities in separate books."),
        ] {
            let mut bad = legacy.clone();
            bad[key] = value;
            assert_eq!(crate::load(&bad.to_string()).unwrap_err(), error);
            assert_eq!(crate::load_value(bad).unwrap_err(), error);
        }
        assert_eq!(crate::load_value(json!({"format":"spheres-equipment-save", "version":1})).unwrap_err(),
            "The saved campaign must be an object.");
    }
}
