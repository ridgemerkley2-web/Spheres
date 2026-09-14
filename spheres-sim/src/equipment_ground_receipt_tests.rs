use super::*;
use crate::{arsenal, operations::Snapshot};

const USA: NationId = NationId::USA;

fn fixture(platform: &str) -> WorldState {
    let mut w = ammunition_operation_tests::fixture(platform);
    w.conflicts.clear();
    w.conflicts.push(ammunition_operation_tests::conflict(&w, 401, NationId::Canada));
    w.conflicts.push(ammunition_operation_tests::conflict(&w, 402, NationId::Mexico));
    ammunition_operation_tests::stock(&mut w, 100_000);
    w
}

fn settle(w: &mut WorldState, fraction: f64) {
    let mut snapshot = Snapshot::new(w);
    for cid in [402, 401] { snapshot.record_loss(cid, USA, fraction); }
    snapshot.settle(w);
}

fn receipt(w: &WorldState) -> &GroundOperationsReceipt {
    w.nation(USA).equipment.as_ref().unwrap().ground_operations_receipt.as_ref().unwrap()
}

#[test]
fn ground_receipt_all_nine_platforms_report_actual_whole_losses_across_fronts() {
    for platform in PLATFORMS.iter().filter(|p| p.id.starts_with("tank_") || p.id.starts_with("ground_")) {
        let mut w = fixture(platform.id);
        let opening = w.nation(USA).arsenal.held[0].clone();
        let mut reversed = w.clone();
        reversed.conflicts.reverse();
        settle(&mut w, 0.25);
        settle(&mut reversed, 0.25);
        let r = receipt(&w);
        assert_eq!(r.day, clock::absolute_day(&w));
        assert_eq!(r.conflicts, vec![401, 402]);
        assert_eq!(r, receipt(&reversed));
        assert_eq!(r.revisions.len(), 1);
        let row = &r.revisions[0];
        let remaining = &w.nation(USA).arsenal.held[0];
        assert_eq!(row.opening_delivered, opening.units as u64);
        assert_eq!(row.opening_available, arsenal::available_design_units(&opening) as u64);
        assert_eq!(row.lost, (opening.units - remaining.units) as u64);
        assert!(row.lost > 0, "{}", platform.id);
        assert_eq!(row.remaining_delivered, remaining.units as u64);
        assert_eq!(row.remaining_loss_remainder, remaining.loss_remainder);
        validate_ground_operations_receipts(&w).unwrap();
        validate_state(w.nation(USA)).unwrap();
    }
}

#[test]
fn ground_receipt_fractional_loss_and_refit_reserve_continue_exactly_after_reload() {
    let mut w = fixture("tank_standard");
    let district = w.districts.iter().find(|(_, id)| **id == USA).unwrap().0.clone();
    crate::production::complete_capability(&mut w, &district, crate::production::ProjectKind::ArmsPlant);
    let mut spec = default_spec("tank_standard");
    spec.components.insert("mobility".into(), "engine_diesel_1200".into());
    let profile = design_preview(&w, USA, &spec).profile.unwrap();
    let day = clock::absolute_day(&w);
    state_mut(w.nation_mut(USA)).revisions.insert("refit-target".into(), DesignRevision {
        id: "refit-target".into(), name: "Explicit test upgrade".into(),
        specification_key: specification_key(&spec), spec, profile,
        created_day: day, certified_day: Some(day),
    });
    start_refit(&mut w, USA, "ammo-model", "refit-target", &district, 250, 0.0).unwrap();
    // Start from the canonical complete save, then take a fractional loss too
    // small to destroy a vehicle. The reserved 250 remain in the same holding.
    w = crate::load(&crate::save(&w)).unwrap();
    settle(&mut w, 0.0002);
    let row = &receipt(&w).revisions[0];
    assert_eq!(row.opening_reserved, 250);
    assert_eq!(row.remaining_reserved, 250);
    assert_eq!(row.lost, 0);
    assert!(row.remaining_loss_remainder > row.opening_loss_remainder);
    let raw = crate::save(&w);
    let mut loaded = crate::load(&raw).unwrap();
    assert_eq!(crate::save(&loaded), raw);
    for _ in 0..12 {
        clock::advance_date(&mut w);
        clock::advance_date(&mut loaded);
        // Consumption is dated against the ordinary funding day. Advancing
        // only the calendar would leave the authored ammunition ledger stale.
        crate::programs::begin_day(&mut w);
        crate::programs::begin_day(&mut loaded);
        let opening = w.nation(USA).arsenal.held[0].clone();
        let snap = Snapshot::new(&w);
        let fraction: f64 = [401, 402].iter().map(|id| snap.deployed(*id, USA)
            / snap.strength(USA) * 0.001 * 0.5).sum();
        let carried = (arsenal::available_design_units(&opening) as f64 * fraction + opening.loss_remainder)
            .min(arsenal::available_design_units(&opening) as f64);
        settle(&mut w, 0.001);
        settle(&mut loaded, 0.001);
        let row = &receipt(&w).revisions[0];
        assert_eq!(row.lost, carried.floor() as u64);
        assert!((row.remaining_loss_remainder - (carried - carried.floor())).abs() < 1e-12);
        assert_eq!(row.remaining_reserved, 250);
        assert_eq!(crate::save(&w), crate::save(&loaded));
        validate_state(w.nation(USA)).unwrap();
    }
    assert!(w.nation(USA).arsenal.held[0].units < 1000.0);
}

#[test]
fn ground_receipt_stays_sparse_for_aircraft_orders_only_peace_and_zero_allocation() {
    let mut air = fixture("air_light_attack");
    settle(&mut air, 0.5);
    assert!(air.nation(USA).equipment.as_ref().unwrap().ground_operations_receipt.is_none());
    let mut orders = fixture("ground_ifv");
    orders.nation_mut(USA).arsenal.held.clear();
    arsenal::queue_design_order(orders.nation_mut(USA), "ammo-model", 10, 7, 0.0).unwrap();
    settle(&mut orders, 0.5);
    assert!(orders.nation(USA).equipment.as_ref().unwrap().ground_operations_receipt.is_none());
    let mut w = fixture("ground_ifv");
    settle(&mut w, 0.2);
    let previous = receipt(&w).clone();
    clock::advance_date(&mut w);
    for c in &mut w.conflicts { c.posture_mut(USA).unwrap().force_share_bp = Some(0); }
    settle(&mut w, 1.0);
    assert_eq!(receipt(&w), &previous);
    w.conflicts.clear();
    clock::advance_date(&mut w);
    settle(&mut w, 1.0);
    assert_eq!(receipt(&w), &previous);
}

#[test]
fn ground_receipt_old_saves_remain_absent_and_ended_wars_retired_stock_remain_valid() {
    let mut w = fixture("ground_apc");
    let raw = crate::save(&w);
    assert!(!raw.contains("ground_operations_receipt"));
    let loaded = crate::load(&raw).unwrap();
    assert!(!crate::save(&loaded).contains("ground_operations_receipt"));
    assert_eq!(serde_json::to_value(&loaded.nation(USA).equipment).unwrap(),
        serde_json::to_value(&w.nation(USA).equipment).unwrap());
    w.rules.operational_warfare = 1;
    crate::campaign::enroll(&mut w);
    settle(&mut w, 0.1);
    let previous = receipt(&w).clone();
    w.conflicts.clear();
    let remaining = w.nation(USA).arsenal.held[0].units as u32;
    retire(&mut w, USA, "ammo-model", remaining).unwrap();
    assert!(w.nation(USA).arsenal.held.is_empty());
    clock::advance_date(&mut w);
    validate_ground_operations_receipts(&w).unwrap();
    let loaded = crate::load(&crate::save(&w)).unwrap();
    assert_eq!(receipt(&loaded), &previous);
}

#[test]
fn ground_receipt_closed_legacy_wars_reserve_identity_once_on_operational_adoption() {
    let mut w = fixture("tank_standard");
    settle(&mut w, 0.2);
    let previous = receipt(&w).clone();
    w.conflicts.clear();
    assert!(!crate::campaign::enabled(&w));
    assert_eq!(crate::campaign::conflict_id_high_water(&w), 0);
    w = crate::load(&crate::save(&w)).unwrap();
    let command = crate::Command::EnableOperationalWarfare { nation: USA };
    let mut future = w.clone();
    state_mut(future.nation_mut(USA)).ground_operations_receipt.as_mut().unwrap().day += 1;
    let before = crate::save(&future);
    assert!(crate::apply_command(&mut future, &command).is_err());
    assert_eq!(crate::save(&future), before, "invalid receipt cannot partially enable warfare");
    crate::apply_command(&mut w, &command).unwrap();
    assert!(crate::campaign::enabled(&w));
    assert_eq!(w.campaign.conflict_id_high_water, 402);
    assert!(w.campaign.migration_conflicts.as_ref().unwrap().is_empty(),
        "ended wars cannot migrate as active fronts");
    assert_eq!(receipt(&w), &previous);
    let raw = crate::save(&w);
    let mut loaded = crate::load(&raw).unwrap();
    assert_eq!(crate::save(&loaded), raw);
    assert_eq!(loaded.next_conflict_id(), 403);
    crate::apply_command(&mut loaded, &command).unwrap();
    assert_eq!(crate::save(&loaded), raw, "adoption is idempotent");
    // Existing operational archives remain strictly bounded; a fabricated new
    // receipt identity must not raise high-water during loading or readoption.
    state_mut(loaded.nation_mut(USA)).ground_operations_receipt.as_mut().unwrap().conflicts = vec![999];
    let corrupt = crate::save(&loaded);
    assert!(crate::load(&corrupt).is_err());
    assert!(crate::apply_command(&mut loaded, &command).is_err());
    assert_eq!(crate::save(&loaded), corrupt);
    assert_eq!(loaded.campaign.conflict_id_high_water, 402);
}

#[test]
fn ground_receipt_rejects_corrupt_dates_ids_arithmetic_and_nonfinite_carry_without_mutation() {
    let mut w = fixture("tank_standard");
    settle(&mut w, 0.2);
    for change in 0..8 {
        let mut bad = w.clone();
        let r = state_mut(bad.nation_mut(USA)).ground_operations_receipt.as_mut().unwrap();
        match change {
            0 => r.day += 1,
            1 => r.revisions[0].revision_id = "missing-revision".into(),
            2 => r.revisions[0].lost += 1,
            3 => r.revisions[0].remaining_reserved += 1,
            4 => r.revisions[0].opening_loss_remainder = f64::NAN,
            5 => r.conflicts.push(401),
            6 => r.conflicts = vec![0],
            _ => r.day = -1,
        }
        let before = crate::save(&bad);
        assert!(set_maintenance_plan(&mut bad, USA, 0.01).unwrap_err().contains("ground operations receipt"));
        assert_eq!(crate::save(&bad), before);
        assert!(crate::load(&before).is_err());
    }
    w.rules.operational_warfare = 1;
    crate::campaign::enroll(&mut w);
    state_mut(w.nation_mut(USA)).ground_operations_receipt.as_mut().unwrap().conflicts = vec![u32::MAX];
    assert!(validate_ground_operations_receipts(&w).is_err());
}

#[test]
fn ground_receipt_aggregates_physical_rows_without_merging_or_mutating_them() {
    let mut w = fixture("tank_standard");
    let mut second = w.nation(USA).arsenal.held[0].clone();
    second.units = 100.0;
    second.loss_remainder = 0.75;
    w.nation_mut(USA).arsenal.held.push(second);
    w.nation_mut(USA).arsenal.held[0].loss_remainder = 0.75;
    settle(&mut w, 0.1);
    let row = &receipt(&w).revisions[0];
    assert_eq!(row.holding_rows, 2);
    assert_eq!(row.opening_delivered, 1100);
    assert_eq!(row.opening_loss_remainder, 1.5);
    assert_eq!(row.remaining_delivered, w.nation(USA).arsenal.held.iter().map(|h| h.units as u64).sum::<u64>());
    assert_eq!(w.nation(USA).arsenal.held.len(), 2);
    validate_ground_operations_receipts(&w).unwrap();
}
