use super::*;
use crate::{
    campaign::{self, AirMission, Approach, OperationOrder},
    equipment::ammunition_operation_tests as ammo,
    programs,
};

const ID: NationId = NationId::USA;
const ENEMY: NationId = NationId::Canada;
const CID: u32 = 400;
const MODEL: &str = "ammo-model";
const FAMILY: &str = "air_bomb_unguided";

/// Authored cross-session fixture: four certified physical aircraft, two saved
/// establishments and a completed geographic airfield. The ammunition fixture
/// retains a completed, paid fabrication receipt; this is no historical grant.
pub(crate) fn fixture() -> (WorldState, String, String) {
    let mut w = ammo::fixture("air_light_attack");
    w.rules.operational_warfare = 1;
    w.conflicts.clear();
    w.conflicts.push(ammo::conflict(&w, CID, ENEMY));
    w.nation_mut(ID).arsenal.held[0].units = 4.0;
    ammo::stock(&mut w, 100);
    campaign::enroll(&mut w);
    let snapshot = operations::Snapshot::new(&w);
    let _ = campaign::prepare(&mut w, &snapshot);
    let contested = front::contested_set(&w, w.conflict(CID).unwrap());
    let mut choices = vec![];
    for (target, (side_a, _)) in &contested.k {
        if *side_a {
            continue;
        }
        let Some(t) = airbases::district_location(target) else {
            continue;
        };
        for s in w.campaign.sectors.values().filter(|s| {
            s.nation == ID
                && s.conflict == CID
                && s.strength > 0.0
                && crate::districts::adj_of(&s.district).contains(target)
        }) {
            let Some(b) = airbases::district_location(&s.district) else {
                continue;
            };
            choices.push((
                airbases::distance_km(b, t),
                s.district.clone(),
                target.clone(),
            ));
        }
    }
    choices.sort_by(|a, b| {
        a.0.total_cmp(&b.0)
            .then_with(|| a.1.cmp(&b.1))
            .then_with(|| a.2.cmp(&b.2))
    });
    let (_, base, target) = choices
        .into_iter()
        .next()
        .expect("Native US/Canada frontier contact");
    w.airbases = Some(airbases::AirbaseState {
        bases: vec![airbases::Airbase {
            id: base.clone(),
            district: base.clone(),
            name: "Authored campaign airfield".into(),
            sponsor: ID,
            capacity_level: 1,
            support_level: 0,
            protection_level: 0,
            project: None,
            history: vec![],
        }],
        ..Default::default()
    });
    for name in ["First flight", "Second flight"] {
        aviation::apply(
            &mut w,
            ID,
            &aviation::SquadronCommand::Create {
                name: name.into(),
                revision: MODEL.into(),
                quantity: 2,
            },
        )
        .unwrap();
    }
    for sq in &mut w.nation_mut(ID).aviation.as_mut().unwrap().squadrons {
        sq.base = Some(base.clone());
    }
    equipment::settle_support(&mut w);
    equipment::set_air_support(&mut w, ID, 1.0, 30, true).unwrap();
    programs::stage_fiscal(w.nation_mut(ID), 0.0, 0.0);
    programs::finish_day(&mut w);
    let q = quote(&w, ID, 1, CID, MissionKind::SupportArmy, &target);
    assert!(
        q.valid,
        "Authored contact must support both mission kinds: {:?}",
        q.reason
    );
    airbases::validate(&w).unwrap();
    aviation::validate(&w).unwrap();
    (w, base, target)
}

fn queue(w: &mut WorldState, squadron: u32, kind: MissionKind, target: &str) {
    apply(
        w,
        ID,
        &MissionCommand::Queue {
            squadron,
            conflict: CID,
            kind,
            target: target.into(),
        },
    )
    .unwrap();
}

fn open_next(w: &mut WorldState) {
    clock::advance_date(w);
    programs::begin_day(w);
    equipment::settle_support(w);
    equipment::tick_air_support(w);
}

fn campaign_day(w: &mut WorldState) {
    open_next(w);
    crate::war::tick(w);
    programs::stage_fiscal(w.nation_mut(ID), 0.0, 0.0);
    programs::finish_day(w);
    validate(w).unwrap();
    aviation::validate(w).unwrap();
    equipment::validate_state(w.nation(ID)).unwrap();
}

fn stores(w: &WorldState) -> f64 {
    w.nation(ID)
        .equipment
        .as_ref()
        .unwrap()
        .ammunition
        .as_ref()
        .unwrap()
        .stocks[FAMILY]
}

fn reports(w: &WorldState) -> Vec<&MissionReport> {
    w.air_missions
        .as_ref()
        .unwrap()
        .orders
        .iter()
        .filter_map(|o| o.report.as_ref())
        .collect()
}

fn near(a: f64, b: f64) {
    assert!((a - b).abs() <= 1e-9, "{a} != {b}");
}

#[test]
fn s15_reviews_reserve_real_squadrons_and_refuse_busy_or_unavailable_flights() {
    let (mut w, base, target) = fixture();
    let before = crate::save(&w);
    assert!(quote(&w, ID, 1, CID, MissionKind::StrikeTarget, &target).valid);
    assert_eq!(
        crate::save(&w),
        before,
        "Review cannot mutate holdings, money or stores"
    );
    queue(&mut w, 1, MissionKind::StrikeTarget, &target);
    assert!(busy(&w, ID, 1));
    let booked = crate::save(&w);
    assert!(apply(
        &mut w,
        ID,
        &MissionCommand::Queue {
            squadron: 1,
            conflict: CID,
            kind: MissionKind::SupportArmy,
            target: target.clone(),
        }
    )
    .is_err());
    assert_eq!(crate::save(&w), booked);
    assert!(aviation::refusal(
        &w,
        ID,
        &aviation::SquadronCommand::Resize {
            squadron: 1,
            quantity: 1
        }
    )
    .is_some());
    assert!(airbases::refusal(
        &w,
        ID,
        &airbases::AirbaseCommand::Rebase { squadron: 1, base }
    )
    .is_some());
    apply(&mut w, ID, &MissionCommand::Cancel { mission: 1 }).unwrap();
    assert!(!busy(&w, ID, 1));
    assert_eq!(
        w.air_missions.as_ref().unwrap().orders[0].status,
        MissionStatus::Cancelled
    );
    near(stores(&w), 100.0);
    w.nation_mut(ID).aviation.as_mut().unwrap().squadrons[0].service_days_left = 2;
    assert!(quote(&w, ID, 1, CID, MissionKind::StrikeTarget, &target)
        .reason
        .unwrap()
        .contains("support"));
}

#[test]
fn s15_two_missions_share_finite_stores_and_replay_the_same_campaign_result() {
    let (mut w, _, target) = fixture();
    let q = quote(&w, ID, 1, CID, MissionKind::StrikeTarget, &target);
    let limited_stock = q.stores_required * 0.5;
    let a = w
        .nation_mut(ID)
        .equipment
        .as_mut()
        .unwrap()
        .ammunition
        .as_mut()
        .unwrap();
    a.stocks.insert(FAMILY.into(), limited_stock);
    a.consumed.insert(FAMILY.into(), 100.0 - limited_stock);
    queue(&mut w, 1, MissionKind::SupportArmy, &target);
    queue(&mut w, 2, MissionKind::StrikeTarget, &target);
    let saved = crate::save(&w);
    let mut loaded = crate::load(&saved).unwrap();
    let mut baseline = w.clone();
    for mission in [1, 2] {
        apply(&mut baseline, ID, &MissionCommand::Cancel { mission }).unwrap();
    }
    campaign_day(&mut baseline);
    campaign_day(&mut w);
    campaign_day(&mut loaded);
    assert_eq!(crate::save(&w), crate::save(&loaded));
    assert_eq!(reports(&w).len(), 2);
    assert!(w
        .air_missions
        .as_ref()
        .unwrap()
        .orders
        .iter()
        .all(|o| o.status == MissionStatus::Flown));
    let consumed: f64 = reports(&w).iter().map(|r| r.stores_used).sum();
    assert!(consumed > 0.0);
    assert!(consumed <= limited_stock + 1e-9);
    near(stores(&w), limited_stock - consumed);
    assert!(reports(&w)
        .iter()
        .any(|r| r.contacted && r.applied_power > 0.0));
    assert!(
        w.nation(ENEMY).mil_strength < baseline.nation(ENEMY).mil_strength,
        "Paid strike power must affect the shared campaign casualty path"
    );
    let sq = &w.nation(ID).aviation.as_ref().unwrap().squadrons;
    assert!(sq.iter().all(|s| s.service_days_left > 0));
    let lost: u32 = reports(&w).iter().map(|r| r.aircraft_lost).sum();
    near(w.nation(ID).arsenal.held[0].units, 4.0 - lost as f64);
    assert_eq!(sq.iter().map(|s| s.assigned).sum::<u32>(), 4 - lost);
    let settled = crate::save(&w);
    prepare(&mut w);
    settle(&mut w);
    assert_eq!(
        crate::save(&w),
        settled,
        "Repeated mission hooks cannot debit or report twice"
    );
}

#[test]
fn s15_prepared_plan_save_and_resume_consumes_one_shared_ammunition_receipt() {
    let (mut w, _, target) = fixture();
    queue(&mut w, 1, MissionKind::StrikeTarget, &target);
    open_next(&mut w);
    prepare(&mut w);
    assert_eq!(w.air_missions.as_ref().unwrap().plans.len(), 1);
    let saved = crate::save(&w);
    let mut loaded = crate::load(&saved).unwrap();
    for state in [&mut w, &mut loaded] {
        let c = state.conflict(CID).unwrap().clone();
        record_contact(state, &c, &target, &[(ENEMY, 0.35)]);
        let snapshot = operations::Snapshot::new(state);
        snapshot.settle(state);
        settle(state);
    }
    assert_eq!(crate::save(&w), crate::save(&loaded));
    assert_eq!(reports(&w).len(), 1);
    let report = reports(&w)[0];
    let ammo = w
        .nation(ID)
        .equipment
        .as_ref()
        .unwrap()
        .ammunition
        .as_ref()
        .unwrap();
    near(
        ammo.last_consumption.as_ref().unwrap().used[FAMILY],
        report.stores_used,
    );
    near(stores(&w), 100.0 - report.stores_used);
    assert_eq!(report.day, clock::absolute_day(&w));
}

#[test]
fn s15_access_lost_before_launch_blocks_without_ammunition_or_aircraft_loss() {
    let (mut w, base, target) = fixture();
    queue(&mut w, 1, MissionKind::StrikeTarget, &target);
    // Authored control loss between order and launch. The base remains a real
    // geographic asset; its aircraft cannot launch under obsolete permission.
    w.districts.insert(base, ENEMY);
    open_next(&mut w);
    let held = serde_json::to_string(&w.nation(ID).arsenal.held).unwrap();
    let stock = stores(&w);
    prepare(&mut w);
    assert!(w.air_missions.as_ref().unwrap().plans.is_empty());
    let o = &w.air_missions.as_ref().unwrap().orders[0];
    assert_eq!(o.status, MissionStatus::Blocked);
    assert_eq!(o.report.as_ref().unwrap().stores_used, 0.0);
    settle(&mut w);
    near(stores(&w), stock);
    assert_eq!(
        serde_json::to_string(&w.nation(ID).arsenal.held).unwrap(),
        held
    );
}

#[test]
fn s15_enrolled_aircraft_do_not_run_a_second_implicit_campaign_air_raid() {
    let (mut w, _, _) = fixture();
    w.conflict_mut(CID).unwrap().posture_mut(ID).unwrap().rung = 6;
    let mut order = OperationOrder::automatic(w.conflict(CID).unwrap(), ID);
    order.approach = Approach::Advance;
    order.air = AirMission::GroundSupport;
    order.reserve_bp = 0;
    campaign::set_order(&mut w, &order).unwrap();
    let stock = stores(&w);
    let overview = equipment::ammunition_overview(&w, ID);
    assert_eq!(
        overview
            .families
            .iter()
            .filter(|f| f.family == FAMILY)
            .map(|f| f.required)
            .sum::<f64>(),
        0.0
    );
    let mut snapshot = operations::Snapshot::new(&w);
    let held = serde_json::to_string(&w.nation(ID).arsenal.held).unwrap();
    snapshot.record_loss(CID, ID, 0.5);
    snapshot.settle(&mut w);
    near(stores(&w), stock);
    assert_eq!(
        serde_json::to_string(&w.nation(ID).arsenal.held).unwrap(),
        held
    );
}

#[test]
fn s15_closed_shared_store_settlement_cannot_fund_another_launch() {
    let (mut w, _, target) = fixture();
    queue(&mut w, 1, MissionKind::StrikeTarget, &target);
    open_next(&mut w);
    let snapshot = operations::Snapshot::new(&w);
    snapshot.settle(&mut w);
    let before = stores(&w);
    prepare(&mut w);
    settle(&mut w);
    assert_eq!(
        w.air_missions.as_ref().unwrap().orders[0].status,
        MissionStatus::Blocked
    );
    near(stores(&w), before);
    assert_eq!(
        w.nation(ID).aviation.as_ref().unwrap().squadrons[0].service_days_left,
        0
    );
}

#[test]
fn s15_support_requires_ground_contact_and_aircraft_use_the_exact_store_family() {
    let (mut w, _, target) = fixture();
    w.campaign.sectors.retain(|_, s| s.nation != ID);
    assert!(quote(&w, ID, 1, CID, MissionKind::SupportArmy, &target)
        .reason
        .unwrap()
        .contains("contact"));
    let a = w
        .nation_mut(ID)
        .equipment
        .as_mut()
        .unwrap()
        .ammunition
        .as_mut()
        .unwrap();
    a.stocks.insert(FAMILY.into(), 0.0);
    a.consumed.insert(FAMILY.into(), 100.0);
    equipment::seed_test_ammunition(&mut w, ID, "air_bomb_guided", 100);
    let q = quote(&w, ID, 1, CID, MissionKind::StrikeTarget, &target);
    assert!(!q.valid);
    assert!(q.reason.unwrap().contains("compatible"));
}

#[test]
fn s15_malformed_prepared_plan_references_families_and_shared_claims_are_rejected() {
    let (mut w, _, target) = fixture();
    queue(&mut w, 1, MissionKind::StrikeTarget, &target);
    open_next(&mut w);
    prepare(&mut w);
    validate(&w).unwrap();
    for flaw in 0..10 {
        let mut wrong = w.clone();
        let s = wrong.air_missions.as_mut().unwrap();
        match flaw {
            0 => s.plans[0].revision = "missing-aircraft".into(),
            1 => s.plans[0].family = "air_bomb_guided".into(),
            2 => s.plans[0].base = "missing-airfield".into(),
            3 => s.plans[0].aircraft += 1,
            4 => s.plans[0].required += 1.0,
            5 => s.plans[0].power *= 2.0,
            6 => {
                let p = s.plans[0].clone();
                s.plans.push(p);
            }
            7 => {
                let mut o = s.orders[0].clone();
                o.id = s.next_id;
                s.next_id += 1;
                s.orders.push(o);
            }
            8 => s.plans[0].order = s.next_id + 1,
            _ => s.plans.clear(),
        }
        assert!(
            validate(&wrong).is_err(),
            "Malformed prepared flight {flaw} must be refused"
        );
        assert!(
            crate::load(&crate::save(&wrong)).is_err(),
            "Malformed saved flight {flaw} must be refused"
        );
    }
}

#[test]
fn s15_frozen_launch_cannot_be_cancelled_reassigned_or_replayed_after_resume() {
    let (mut w, _, target) = fixture();
    queue(&mut w, 1, MissionKind::StrikeTarget, &target);
    open_next(&mut w);
    prepare(&mut w);
    let saved = crate::save(&w);
    assert!(apply(&mut w, ID, &MissionCommand::Cancel { mission: 1 }).is_err());
    assert!(aviation::apply(
        &mut w,
        ID,
        &aviation::SquadronCommand::Disband { squadron: 1 }
    )
    .is_err());
    assert_eq!(crate::save(&w), saved);
    let mut loaded = crate::load(&saved).unwrap();
    for state in [&mut w, &mut loaded] {
        prepare(state);
        let c = state.conflict(CID).unwrap().clone();
        record_contact(state, &c, &target, &[(ENEMY, 0.7)]);
        let contacted = crate::save(state);
        record_contact(state, &c, &target, &[(ENEMY, 0.7)]);
        assert_eq!(
            crate::save(state),
            contacted,
            "The same campaign contact cannot add a second flight loss"
        );
        operations::Snapshot::new(state).settle(state);
        settle(state);
        let settled = crate::save(state);
        prepare(state);
        settle(state);
        assert!(apply(state, ID, &MissionCommand::Cancel { mission: 1 }).is_err());
        assert_eq!(crate::save(state), settled);
    }
    assert_eq!(crate::save(&w), crate::save(&loaded));
}

#[test]
fn s15_malformed_flown_reports_cannot_invent_stores_sorties_or_another_loadout() {
    let (mut w, _, target) = fixture();
    queue(&mut w, 1, MissionKind::StrikeTarget, &target);
    open_next(&mut w);
    prepare(&mut w);
    let c = w.conflict(CID).unwrap().clone();
    record_contact(&mut w, &c, &target, &[(ENEMY, 0.35)]);
    operations::Snapshot::new(&w).settle(&mut w);
    settle(&mut w);
    validate(&w).unwrap();
    for flaw in 0..7 {
        let mut wrong = w.clone();
        let o = &mut wrong.air_missions.as_mut().unwrap().orders[0];
        let r = o.report.as_mut().unwrap();
        match flaw {
            0 => r.family = "air_bomb_guided".into(),
            1 => r.aircraft += 1,
            2 => r.sorties += 1.0,
            3 => r.stores_used += 1.0,
            4 => r.applied_power += 1.0,
            5 => r.contacted = false,
            _ => o.status = MissionStatus::Cancelled,
        }
        assert!(
            validate(&wrong).is_err(),
            "Malformed flown receipt {flaw} must be refused"
        );
        assert!(crate::load(&crate::save(&wrong)).is_err());
    }
}

#[test]
fn s15_whole_aircraft_loss_debits_only_the_flight_and_preserves_other_claims() {
    let (mut w, base, target) = fixture();
    // Extra authored aircraft: a second squadron in service, a third in
    // transit, one unassigned aircraft in paid refit, and free reserve stock.
    arsenal::deliver_design(w.nation_mut(ID), MODEL, 8, 0.0).unwrap();
    aviation::apply(
        &mut w,
        ID,
        &aviation::SquadronCommand::Create {
            name: "Transit fixture".into(),
            revision: MODEL.into(),
            quantity: 2,
        },
    )
    .unwrap();
    let destination = w
        .districts
        .iter()
        .find(|(d, n)| **n == ID && **d != base && airbases::district_location(d).is_some())
        .unwrap()
        .0
        .clone();
    w.airbases.as_mut().unwrap().bases.push(airbases::Airbase {
        id: destination.clone(),
        district: destination.clone(),
        name: "Authored transit destination".into(),
        sponsor: ID,
        capacity_level: 1,
        support_level: 0,
        protection_level: 0,
        project: None,
        history: vec![],
    });
    let day = clock::absolute_day(&w);
    let a = w.nation_mut(ID).aviation.as_mut().unwrap();
    a.squadrons[1].service_days_left = 2;
    a.squadrons[2].transit = Some(aviation::AirTransit {
        from: Some(base.clone()),
        to: destination,
        departed_day: day,
        arrival_day: day + 7,
    });
    let mut spec = equipment::default_spec("air_light_attack");
    spec.components
        .insert("air_fuel".into(), "air_fuel_extended".into());
    let profile = equipment::design_preview(&w, ID, &spec).profile.unwrap();
    w.nation_mut(ID)
        .equipment
        .as_mut()
        .unwrap()
        .revisions
        .insert(
            "refit-target".into(),
            equipment::DesignRevision {
                id: "refit-target".into(),
                name: "Authored paid refit target".into(),
                specification_key: equipment::specification_key(&spec),
                spec,
                profile,
                created_day: day,
                certified_day: Some(day),
            },
        );
    crate::production::complete_capability(
        &mut w,
        &base,
        crate::production::ProjectKind::ArmsPlant,
    );
    let refit = equipment::start_refit(&mut w, ID, MODEL, "refit-target", &base, 1, 0.01).unwrap();
    w.nation_mut(ID).arsenal.held[0].loss_remainder = 0.999999;
    // Refresh the actual maintenance invoice on the next date before review.
    open_next(&mut w);
    queue(&mut w, 1, MissionKind::StrikeTarget, &target);
    // A separate authored prior flight places the unrelated squadron back
    // into two-day turnaround immediately before this launch date.
    w.nation_mut(ID).aviation.as_mut().unwrap().squadrons[1].service_days_left = 2;
    open_next(&mut w);
    let unrelated: Vec<_> = w
        .nation(ID)
        .aviation
        .as_ref()
        .unwrap()
        .squadrons
        .iter()
        .filter(|s| s.id != 1)
        .cloned()
        .collect();
    let reserve = aviation::unassigned_units(w.nation(ID), MODEL);
    let project = w
        .nation(ID)
        .equipment
        .as_ref()
        .unwrap()
        .projects
        .iter()
        .find(|p| p.id == refit)
        .unwrap()
        .clone();
    prepare(&mut w);
    let c = w.conflict(CID).unwrap().clone();
    let saved = crate::save(&w);
    let mut loaded = crate::load(&saved).unwrap();
    for state in [&mut w, &mut loaded] {
        record_contact(state, &c, &target, &[(ENEMY, 0.7)]);
        assert!(state.air_missions.as_ref().unwrap().plans[0].expected_loss > 0.000001);
        let mut snapshot = operations::Snapshot::new(state);
        // Even a simultaneous ground force loss cannot touch parked aircraft.
        snapshot.record_loss(CID, ID, 0.5);
        snapshot.settle(state);
        settle(state);
    }
    assert_eq!(crate::save(&w), crate::save(&loaded));
    let r = reports(&w)[0];
    assert_eq!(r.aircraft_lost, 1);
    let h = &w.nation(ID).arsenal.held[0];
    assert_eq!(h.units, 11.0);
    assert_eq!(h.refit_reserved, 1);
    assert!(h.loss_remainder > 0.0 && h.loss_remainder < 1.0);
    assert_eq!(
        w.nation(ID).aviation.as_ref().unwrap().squadrons[0].assigned,
        1
    );
    assert_eq!(
        w.nation(ID)
            .aviation
            .as_ref()
            .unwrap()
            .squadrons
            .iter()
            .filter(|s| s.id != 1)
            .cloned()
            .collect::<Vec<_>>(),
        unrelated
    );
    assert_eq!(aviation::unassigned_units(w.nation(ID), MODEL), reserve);
    assert_eq!(
        serde_json::to_value(
            w.nation(ID)
                .equipment
                .as_ref()
                .unwrap()
                .projects
                .iter()
                .find(|p| p.id == refit)
                .unwrap()
        )
        .unwrap(),
        serde_json::to_value(project).unwrap()
    );
    aviation::validate(&w).unwrap();
    validate(&w).unwrap();
}

#[test]
fn s15_shared_family_coverage_cannot_be_forged_to_reuse_the_same_stores() {
    let (mut w, _, target) = fixture();
    let one_flight = quote(&w, ID, 1, CID, MissionKind::StrikeTarget, &target).stores_required;
    let a = w
        .nation_mut(ID)
        .equipment
        .as_mut()
        .unwrap()
        .ammunition
        .as_mut()
        .unwrap();
    a.stocks.insert(FAMILY.into(), one_flight);
    a.consumed.insert(FAMILY.into(), 100.0 - one_flight);
    queue(&mut w, 1, MissionKind::StrikeTarget, &target);
    queue(&mut w, 2, MissionKind::StrikeTarget, &target);
    open_next(&mut w);
    prepare(&mut w);
    validate(&w).unwrap();
    assert_eq!(w.air_missions.as_ref().unwrap().plans.len(), 2);
    assert!(w
        .air_missions
        .as_ref()
        .unwrap()
        .plans
        .iter()
        .all(|p| p.coverage < 1.0));
    let mut forged = w.clone();
    for p in &mut forged.air_missions.as_mut().unwrap().plans {
        p.coverage = 1.0;
    }
    assert!(validate(&forged).is_err());
    assert!(crate::load(&crate::save(&forged)).is_err());
    let c = w.conflict(CID).unwrap().clone();
    record_contact(&mut w, &c, &target, &[(ENEMY, 0.7)]);
    operations::Snapshot::new(&w).settle(&mut w);
    // A deliberately inconsistent partial shared receipt cannot pay both
    // flights just because each separately fits the same remaining amount.
    let r = w
        .nation_mut(ID)
        .equipment
        .as_mut()
        .unwrap()
        .ammunition
        .as_mut()
        .unwrap()
        .last_consumption
        .as_mut()
        .unwrap();
    *r.used.get_mut(FAMILY).unwrap() *= 0.6;
    let before = serde_json::to_string(&w.nation(ID).arsenal.held).unwrap();
    settle(&mut w);
    assert!(w
        .air_missions
        .as_ref()
        .unwrap()
        .orders
        .iter()
        .all(|o| o.status == MissionStatus::Blocked));
    assert!(reports(&w)
        .iter()
        .all(|r| r.aircraft_lost == 0 && r.stores_used == 0.0));
    assert_eq!(
        serde_json::to_string(&w.nation(ID).arsenal.held).unwrap(),
        before
    );
}
