/// Last national ground-combat inventory settlement, aggregated across every
/// participating front. These are delivered vehicles, not orders or a claim
/// that a particular vehicle was assigned to a particular theatre.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct GroundOperationsReceipt {
    pub day: i32,
    pub conflicts: Vec<u32>,
    pub revisions: Vec<GroundOperationsRevisionReceipt>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct GroundOperationsRevisionReceipt {
    pub revision_id: String,
    pub holding_rows: u32,
    pub opening_delivered: u64,
    pub opening_available: u64,
    pub opening_reserved: u64,
    pub lost: u64,
    pub remaining_delivered: u64,
    pub remaining_available: u64,
    pub remaining_reserved: u64,
    /// Aggregated fractional attrition carried by the physical holding rows.
    /// It is not rounded into the whole-vehicle loss reported above.
    pub opening_loss_remainder: f64,
    pub remaining_loss_remainder: f64,
}

pub(crate) fn ground_operations_opening(n: &Nation) -> Vec<GroundOperationsRevisionReceipt> {
    let mut rows = BTreeMap::<String, GroundOperationsRevisionReceipt>::new();
    for h in &n.arsenal.held {
        let Some(revision_id) = h.design_id.as_deref() else { continue; };
        let Some(revision) = n.equipment.as_ref().and_then(|s| s.revisions.get(revision_id)) else { continue; };
        if revision.profile.aviation.is_some()
            || !(revision.spec.platform.starts_with("tank_") || revision.spec.platform.starts_with("ground_"))
            || h.units <= 0.0 { continue; }
        let row = rows.entry(revision_id.to_owned()).or_insert_with(|| GroundOperationsRevisionReceipt {
            revision_id: revision_id.to_owned(), holding_rows: 0,
            opening_delivered: 0, opening_available: 0, opening_reserved: 0,
            lost: 0, remaining_delivered: 0, remaining_available: 0, remaining_reserved: 0,
            opening_loss_remainder: 0.0, remaining_loss_remainder: 0.0,
        });
        row.holding_rows += 1;
        row.opening_delivered += h.units as u64;
        row.opening_available += crate::arsenal::available_design_units(h) as u64;
        row.opening_reserved += h.refit_reserved as u64;
        row.opening_loss_remainder += h.loss_remainder;
    }
    rows.into_values().collect()
}

pub(crate) fn settle_ground_operations_receipt(n: &mut Nation, day: i32, conflicts: Vec<u32>,
    mut revisions: Vec<GroundOperationsRevisionReceipt>) {
    if conflicts.is_empty() || revisions.is_empty() { return; }
    for row in &mut revisions {
        for h in n.arsenal.held.iter().filter(|h| h.design_id.as_deref() == Some(row.revision_id.as_str())) {
            row.remaining_delivered += h.units as u64;
            row.remaining_available += crate::arsenal::available_design_units(h) as u64;
            row.remaining_reserved += h.refit_reserved as u64;
            row.remaining_loss_remainder += h.loss_remainder;
        }
        // The holding debit is authoritative. Never estimate a killed vehicle
        // from a rounded national casualty percentage.
        row.lost = row.opening_delivered - row.remaining_delivered;
    }
    if let Some(s) = &mut n.equipment {
        s.ground_operations_receipt = Some(GroundOperationsReceipt { day, conflicts, revisions });
    }
}

fn validate_ground_operations_receipt(n: &Nation) -> Result<(), String> {
    let Some(s) = &n.equipment else { return Ok(()); };
    let Some(receipt) = &s.ground_operations_receipt else { return Ok(()); };
    let fail = || format!("Invalid national ground operations receipt for {}.", n.id.name());
    if receipt.day < 0 || receipt.conflicts.is_empty() || receipt.conflicts.contains(&0)
        || receipt.conflicts.windows(2).any(|pair| pair[0] >= pair[1])
        || receipt.revisions.is_empty() || receipt.revisions.len() > MAX_REVISIONS
        || receipt.revisions.windows(2).any(|pair| pair[0].revision_id >= pair[1].revision_id) {
        return Err(fail());
    }
    for row in &receipt.revisions {
        let Some(revision) = s.revisions.get(&row.revision_id) else { return Err(fail()); };
        if revision.profile.aviation.is_some()
            || !(revision.spec.platform.starts_with("tank_") || revision.spec.platform.starts_with("ground_"))
            || revision.certified_day.is_none_or(|day| day > receipt.day)
            || revision.created_day > receipt.day || row.holding_rows == 0
            || row.opening_delivered == 0
            || row.opening_delivered > row.holding_rows as u64 * u32::MAX as u64
            || row.opening_available.checked_add(row.opening_reserved) != Some(row.opening_delivered)
            || row.remaining_available.checked_add(row.remaining_reserved) != Some(row.remaining_delivered)
            || row.remaining_delivered.checked_add(row.lost) != Some(row.opening_delivered)
            || row.remaining_available.checked_add(row.lost) != Some(row.opening_available)
            || row.opening_reserved != row.remaining_reserved
            || [row.opening_loss_remainder, row.remaining_loss_remainder].iter()
                .any(|r| !r.is_finite() || *r < 0.0 || *r >= row.holding_rows as f64) {
            return Err(fail());
        }
    }
    Ok(())
}

/// Load-time context checks. A receipt outlives its war and its retired stock;
/// current holdings are deliberately not substituted for the dated inventory.
pub fn validate_ground_operations_receipts(w: &WorldState) -> Result<(), String> {
    for n in &w.nations {
        validate_ground_operations_receipt_on(w, n)?;
    }
    Ok(())
}

fn validate_ground_operations_receipt_on(w: &WorldState, n: &Nation) -> Result<(), String> {
    validate_ground_operations_receipt(n)?;
    if let Some(r) = n.equipment.as_ref().and_then(|s| s.ground_operations_receipt.as_ref()) {
        if r.day > clock::absolute_day(w) || (crate::campaign::enabled(w)
            && r.conflicts.iter().any(|id| *id > crate::campaign::conflict_id_high_water(w))) {
            return Err(format!("Invalid dated ground operations receipt for {}.", n.id.name()));
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "equipment_ground_receipt_tests.rs"]
mod ground_receipt_tests;
