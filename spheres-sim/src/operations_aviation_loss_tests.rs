use super::*;
use crate::equipment::{self, ammunition_operation_tests};

const USA: NationId = NationId::USA;

fn fixture() -> WorldState {
    // Explicit certified fleet fixture; ammunition comes from a retained paid
    // fabrication ledger. No campaign aircraft or stores are granted by code.
    let mut w = ammunition_operation_tests::fixture("air_light_attack");
    w.nation_mut(USA)
        .equipment
        .as_mut()
        .unwrap()
        .ammunition
        .as_mut()
        .unwrap()
        .active_from_day = None;
    w.conflicts.clear();
    let mut c = ammunition_operation_tests::conflict(&w, 400, NationId::Mexico);
    c.theatre = crate::war::theatre_between(&w, USA, NationId::Canada);
    c.side_a.push(NationId::Canada);
    c.posture_of(USA).unwrap();
    c.posture_mut(USA).unwrap().rung = 6;
    c.posture
        .push(Belligerent::new(NationId::Canada, 8, Objective::Seize));
    w.conflicts.push(c);
    w
}

fn near(a: f64, b: f64) {
    assert!(
        (a - b).abs() < 1e-10 * a.abs().max(b.abs()).max(1.0),
        "{a} != {b}"
    );
}

#[test]
fn grounded_aircraft_in_a_fighting_coalition_take_no_force_or_inventory_loss() {
    for (denied, rung) in [(false, 6), (true, 6), (false, 8)] {
        let mut w = fixture();
        w.conflicts[0].posture_mut(USA).unwrap().rung = rung;
        if denied {
            ammunition_operation_tests::stock(&mut w, 100_000);
            w.access.clear();
            w.conflicts[0].theatre = crate::war::theatre_between(&w, USA, NationId::Iraq);
            assert!(!theatre::has_access(&w, USA, w.conflicts[0].theatre));
        }
        let before = w.nation(USA).mil_strength;
        let inventory = serde_json::to_value(&w.nation(USA).arsenal.held).unwrap();
        let ally_before = w.nation(NationId::Canada).mil_strength;
        let mut snapshot = Snapshot::new(&w);
        assert_eq!(
            snapshot.rows[&(400, USA)]
                .ammunition
                .unwrap()
                .maneuver_fraction,
            0.0
        );
        assert!(snapshot.rows[&(400, NationId::Canada)].effective_force > 0.0);
        snapshot.record_loss(400, USA, 0.5);
        snapshot.record_loss(400, NationId::Canada, 0.5);
        snapshot.settle(&mut w);
        assert_eq!(w.nation(USA).mil_strength, before);
        assert_eq!(
            serde_json::to_value(&w.nation(USA).arsenal.held).unwrap(),
            inventory
        );
        assert!(w.nation(NationId::Canada).mil_strength < ally_before);
    }
}

#[test]
fn mixed_inventory_losses_follow_each_aircrafts_launch_coverage_once() {
    let mut w = fixture();
    let mut spec = equipment::default_spec("air_light_attack");
    spec.components
        .insert("air_payload".into(), "air_payload_guided".into());
    spec.components
        .insert("air_avionics".into(), "air_avionics_digital".into());
    let profile = equipment::design_preview(&w, USA, &spec).profile.unwrap();
    let day = crate::clock::absolute_day(&w);
    w.nation_mut(USA)
        .equipment
        .as_mut()
        .unwrap()
        .revisions
        .insert(
            "guided".into(),
            equipment::DesignRevision {
                id: "guided".into(),
                name: "Synthetic guided aircraft".into(),
                specification_key: equipment::specification_key(&spec),
                spec,
                profile,
                created_day: day,
                certified_day: Some(day),
            },
        );
    arsenal::deliver_design(w.nation_mut(USA), "guided", 1000, 0.0).unwrap();
    let kit = arsenal::DECK
        .iter()
        .position(|d| d.id == "air_gen3")
        .unwrap() as u16;
    w.nation_mut(USA).arsenal.held.push(arsenal::Holding {
        kit,
        design_id: None,
        units: 1000.0,
        age: 0.0,
        refit_reserved: 0,
        loss_remainder: 0.0,
    });
    // Supported and unsupplied planes coexist; two independent constraints
    // must each apply once to the physical aircraft that actually launch.
    w.nation_mut(USA)
        .equipment
        .as_mut()
        .unwrap()
        .maintenance_fraction = 0.5;
    ammunition_operation_tests::stock(&mut w, 100_000);
    let required = equipment::ammunition_overview(&w, USA)
        .families
        .iter()
        .find(|r| r.family == "air_bomb_unguided")
        .unwrap()
        .required;
    let stores = w
        .nation_mut(USA)
        .equipment
        .as_mut()
        .unwrap()
        .ammunition
        .as_mut()
        .unwrap();
    stores
        .stocks
        .insert("air_bomb_unguided".into(), required / 2.0);
    stores
        .consumed
        .insert("air_bomb_unguided".into(), 100_000.0 - required / 2.0);
    let opening = w.nation(USA).mil_strength;
    let mut snapshot = Snapshot::new(&w);
    let row = &snapshot.rows[&(400, USA)];
    let material_fraction = row.deployed / opening * 0.4 * 0.5;
    let expected_force = opening - row.deployed * 0.4 * row.ammunition.unwrap().maneuver_fraction;
    let guided_before = w
        .nation(USA)
        .arsenal
        .held
        .iter()
        .find(|h| h.design_id.as_deref() == Some("guided"))
        .unwrap()
        .clone();
    snapshot.record_loss(400, USA, 0.4);
    snapshot.settle(&mut w);
    near(w.nation(USA).mil_strength, expected_force);
    let holdings = &w.nation(USA).arsenal.held;
    let launched = holdings
        .iter()
        .find(|h| h.design_id.as_deref() == Some("ammo-model"))
        .unwrap();
    near(
        1000.0 - launched.units + launched.loss_remainder,
        1000.0 * material_fraction * 0.5 * 0.5,
    );
    let grounded = holdings
        .iter()
        .find(|h| h.design_id.as_deref() == Some("guided"))
        .unwrap();
    assert_eq!(
        serde_json::to_value(grounded).unwrap(),
        serde_json::to_value(guided_before).unwrap()
    );
    let legacy = holdings.iter().find(|h| h.design_id.is_none()).unwrap();
    near(legacy.units, 1000.0 * (1.0 - material_fraction));
}

#[test]
fn land_battle_preserves_parked_custom_aircraft_and_legacy_writeoff_arithmetic() {
    let mut w = fixture();
    w.conflicts[0].posture_mut(USA).unwrap().rung = 8;
    let kit = arsenal::DECK
        .iter()
        .position(|d| d.id == "air_gen3")
        .unwrap() as u16;
    w.nation_mut(USA).arsenal.held.push(arsenal::Holding {
        kit,
        design_id: None,
        units: 1000.0,
        age: 0.0,
        refit_reserved: 0,
        loss_remainder: 0.0,
    });
    let opening = w.nation(USA).mil_strength;
    let mut snapshot = Snapshot::new(&w);
    let deployed = snapshot.rows[&(400, USA)].deployed;
    let maneuver = snapshot.rows[&(400, USA)]
        .ammunition
        .unwrap()
        .maneuver_fraction;
    snapshot.record_loss(400, USA, 0.4);
    snapshot.settle(&mut w);
    near(
        w.nation(USA).mil_strength,
        opening - deployed * 0.4 * maneuver,
    );
    let holdings = &w.nation(USA).arsenal.held;
    let parked = holdings.iter().find(|h| h.design_id.is_some()).unwrap();
    assert_eq!(parked.units, 1000.0);
    assert_eq!(parked.loss_remainder, 0.0);
    let legacy = holdings.iter().find(|h| h.design_id.is_none()).unwrap();
    near(
        legacy.units,
        1000.0 * (1.0 - deployed / opening * 0.4 * 0.5 * 0.5),
    );
}
