use super::*;
use crate::{campaign::{self, AirMission, Approach, OperationOrder}, equipment::ammunition_operation_tests as ammo};

const USA: NationId = NationId::USA;

fn fixture(platform: &str, rung: u8) -> WorldState {
    let mut w = ammo::fixture(platform);
    w.rules.operational_warfare = 1;
    w.conflicts.clear();
    let mut c = ammo::conflict(&w, 400, NationId::Canada);
    c.posture_mut(USA).unwrap().rung = rung;
    w.conflicts.push(c);
    ammo::stock(&mut w, 100_000);
    order(&mut w, 400, Approach::Advance, 0, AirMission::GroundSupport);
    w
}

fn order(w: &mut WorldState, cid: u32, approach: Approach, reserve_bp: u16, air: AirMission) {
    let mut o = OperationOrder::automatic(w.conflict(cid).unwrap(), USA);
    o.approach = approach;
    o.reserve_bp = reserve_bp;
    o.air = air;
    campaign::set_order(w, &o).unwrap();
}

fn required(w: &WorldState) -> f64 {
    ammunition_overview(w, USA).families.iter().map(|r| r.required).sum()
}

#[test]
fn campaign_orders_change_physical_rounds_and_full_reserve_does_not_fire() {
    for (platform, rung) in [("tank_heavy", 8), ("air_light_attack", 6)] {
        let mut w = fixture(platform, rung);
        order(&mut w, 400, Approach::Hold, 0, AirMission::GroundSupport);
        let hold = required(&w);
        order(&mut w, 400, Approach::Breakthrough, 0, AirMission::GroundSupport);
        let breakthrough = required(&w);
        assert!(hold > 0.0 && breakthrough > hold * 2.0,
            "physical rounds must follow the selected approach: {platform}: {hold} -> {breakthrough}");
        order(&mut w, 400, Approach::Breakthrough, 10_000, AirMission::GroundSupport);
        assert_eq!(required(&w), 0.0, "reserve forces do not fire physical ammunition");
    }
}

#[test]
fn custom_tactical_aircraft_pay_only_for_the_supported_ordered_strike() {
    let mut w = fixture("air_light_attack", 6);
    assert!(required(&w) > 0.0);
    for mission in [AirMission::None, AirMission::Reconnaissance, AirMission::Interception] {
        order(&mut w, 400, Approach::Advance, 0, mission);
        assert_eq!(required(&w), 0.0, "unsupported missions cannot launch custom strike aircraft");
        assert_eq!(Snapshot::new(&w).rows[&(400,USA)].ammunition.unwrap().maneuver_fraction, 0.0);
    }
    w.rules.operational_warfare = 0;
    assert!(required(&w) > 0.0, "legacy operations preserve their existing rung-six launch rule");
}

#[test]
fn custom_ammunition_does_not_erase_the_same_operations_support_demand() {
    let mut w = fixture("tank_heavy", 8);
    order(&mut w, 400, Approach::Breakthrough, 0, AirMission::None);
    let row = Snapshot::new(&w).rows[&(400,USA)].clone();
    assert_eq!(row.ammunition.unwrap().legacy_share, 0.0);
    assert_eq!(row.burn_monthly, 0.0, "physical rounds replace the scalar magazine debit");
    assert!(row.support_burn_monthly > 0.0, "transport and maintenance still serve this force");
    let request = crate::campaign_supply::SupplyRequest {
        key: "400:usa:US-NY".into(), nation: USA, conflict: 400, district: "US-NY".into(),
        deployed: row.deployed, burn_monthly: row.support_burn_monthly, sea_escort: 0.0, sea_denial: 0.0,
    };
    crate::campaign_supply::prepare(&mut w, &[request.clone()]);
    let demand = w.campaign_supply.buffers[&request.key].demand;
    assert!(demand > row.deployed * 0.65, "physical weapons must retain their operation-intensity services");

    let mut scalar = w.clone();
    scalar.nation_mut(USA).equipment.as_mut().unwrap().ammunition.as_mut().unwrap().active_from_day = None;
    scalar.campaign_supply = Default::default();
    let scalar_row = Snapshot::new(&scalar).rows[&(400,USA)].clone();
    assert!(scalar_row.ammunition.is_none());
    assert_eq!(scalar_row.burn_monthly, row.support_burn_monthly);
    let mut scalar_request = request;
    scalar_request.burn_monthly = scalar_row.support_burn_monthly;
    crate::campaign_supply::prepare(&mut scalar, &[scalar_request.clone()]);
    assert_eq!(scalar.campaign_supply.buffers[&scalar_request.key].demand, demand);
}

#[test]
fn local_loss_quotes_match_national_settlement_with_shared_partial_aircraft_stores() {
    for coverage in [0.0, 0.5, 1.0] {
        let mut w = fixture("air_light_attack", 6);
        let mut other = ammo::conflict(&w, 401, NationId::Mexico);
        other.theatre = w.conflicts[0].theatre;
        other.posture_mut(USA).unwrap().rung = 6;
        w.conflicts.push(other);
        order(&mut w, 401, Approach::Hold, 0, AirMission::GroundSupport);
        w.nation_mut(USA).equipment.as_mut().unwrap().maintenance_fraction = 0.5;
        let plan = ammunition_overview(&w, USA);
        let family = plan.families.iter().find(|r| r.required > 0.0).unwrap();
        let opening_stock = family.required * coverage;
        let stores = w.nation_mut(USA).equipment.as_mut().unwrap().ammunition.as_mut().unwrap();
        stores.stocks.insert(family.family.clone(), opening_stock);
        stores.consumed.insert(family.family.clone(), 100_000.0 - opening_stock);
        let opening_force = w.nation(USA).mil_strength;
        let mut snapshot = Snapshot::new(&w);
        let mut actual = 0.0;
        for (cid, fraction) in [(400,0.2),(401,0.3)] {
            let row = &snapshot.rows[&(cid,USA)];
            let raw = row.deployed * fraction;
            let loss = row.actual_loss(raw);
            assert_eq!(snapshot.actual_loss(cid,USA,raw),loss);
            let quoted = view(&w,USA).deployments.into_iter().find(|r|r.conflict==cid).unwrap();
            assert_eq!(quoted.actual_loss(raw),loss, "pure forecast and settlement use the same exposure");
            assert!(loss <= raw);
            actual += loss;
            snapshot.record_loss(cid,USA,fraction);
        }
        if coverage == 0.0 { assert_eq!(actual,0.0); } else { assert!(actual > 0.0); }
        let consumed = snapshot.ammunition[&USA].families.iter().map(|r|r.used).sum::<f64>();
        assert!(consumed <= opening_stock + 1e-9, "two wars share one finite compatible-store stock");
        snapshot.settle(&mut w);
        assert!((w.nation(USA).mil_strength - (opening_force-actual)).abs() < 1e-9,
            "the local quote must remove exactly the force lost nationally");
        let stores = w.nation(USA).equipment.as_ref().unwrap().ammunition.as_ref().unwrap();
        assert!((stores.stocks[&family.family] - (opening_stock-consumed)).abs() < 1e-9);
        let saved = serde_json::to_string(&w).unwrap();
        let restored: WorldState = serde_json::from_str(&saved).unwrap();
        assert_eq!(serde_json::to_string(&restored).unwrap(),saved);
    }
}

#[test]
fn air_defense_rounds_follow_actual_enemy_missions_and_reserves() {
    let mut w = fixture("ground_air_defense", 8);
    let enemy = NationId::Canada;
    let kit = arsenal::DECK.iter().position(|d| d.class == Class::Air).unwrap() as u16;
    w.nation_mut(enemy).arsenal.held = vec![arsenal::Holding {
        kit, design_id: None, units: 1000.0, age: 0.0, refit_reserved: 0, loss_remainder: 0.0,
    }];
    for rung in [6, 8] {
        let posture = w.conflict_mut(400).unwrap().posture_mut(enemy).unwrap();
        posture.rung = rung;
        posture.force_share_bp = Some(100);
        let mut order = OperationOrder::automatic(w.conflict(400).unwrap(), enemy);
        order.air = AirMission::GroundSupport;
        order.reserve_bp = 0;
        campaign::set_order(&mut w, &order).unwrap();
        let active = required(&w);
        assert!(active > 0.0, "actual supported enemy sorties expose local air defense at rung {rung}");
        order.reserve_bp = 5000;
        campaign::set_order(&mut w, &order).unwrap();
        assert!((required(&w) - active * 0.5).abs() < 1e-9);
        order.reserve_bp = 10_000;
        campaign::set_order(&mut w, &order).unwrap();
        assert_eq!(required(&w),0.0, "parked reserves cannot drain hostile anti-air ammunition");
        order.reserve_bp = 0;
        for mission in [AirMission::None,AirMission::Reconnaissance,AirMission::Interception] {
            order.air = mission;
            campaign::set_order(&mut w, &order).unwrap();
            assert_eq!(required(&w),0.0, "non-strike missions cannot create attacking sorties");
        }
    }
}

#[test]
fn garrison_service_never_launches_or_pays_for_parked_aircraft() {
    let mut w = fixture("air_light_attack", 6);
    let full = required(&w);
    assert!(full > 0.0);
    w.campaign_peace.garrisons.insert(format!("400:{USA:?}"),crate::campaign_peace::GarrisonPolicy {
        share_bp: 5000, ..Default::default()
    });
    assert!((required(&w)-full*0.5).abs()<1e-9, "garrison duty is not a strike mission");
    order(&mut w,400,Approach::Advance,5000,AirMission::GroundSupport);
    assert!(campaign::consumption_multiplier(&w,400,USA)>0.0,
        "garrison ground services retain their own activity");
    assert_eq!(required(&w),0.0);
    let row=Snapshot::new(&w).rows[&(400,USA)].clone();
    assert_eq!(row.ammunition.unwrap().maneuver_fraction,0.0);
    assert_eq!(row.actual_loss(row.deployed*0.5),0.0);
}
