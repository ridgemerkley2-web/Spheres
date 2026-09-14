use super::*;
use crate::{
    aviation::{AviationState, Squadron},
    equipment,
    world::{GameRules, NationId as N},
};

fn fixture() -> WorldState {
    let mut w = crate::init::world_1990(GameRules {
        daily_simulation: true,
        military_operations: true,
        production_system: true,
        ..Default::default()
    });
    w.player = Some(N::France);
    w.conflicts.clear();
    programs::set_construction_budget(&mut w, N::France, 1.0).unwrap();
    w
}
fn near_district(w: &WorldState, nation: N, lon: f64, lat: f64) -> String {
    let anchor = BaseLocation {
        district: String::new(),
        name: String::new(),
        lon,
        lat,
    };
    w.districts
        .iter()
        .filter(|(_, owner)| **owner == nation)
        .filter_map(|(id, _)| district_location(id).map(|loc| (id, distance_km(&anchor, loc))))
        .min_by(|a, b| a.1.total_cmp(&b.1))
        .unwrap()
        .0
        .clone()
}
fn establish(w: &mut WorldState, id: &str, cap: f64) {
    apply(
        w,
        N::France,
        &AirbaseCommand::Establish {
            district: id.into(),
            name: "Campaign test airfield".into(),
            daily_budget_mn: cap,
        },
    )
    .unwrap();
}
fn step(w: &mut WorldState) -> Vec<String> {
    clock::advance_date(w);
    programs::begin_day(w);
    let events = tick_day(w);
    programs::finish_day(w);
    validate(w).unwrap();
    events
}
fn complete(w: &mut WorldState, id: &str) {
    for _ in 0..200 {
        if base(w, id).unwrap().project.is_none() {
            return;
        }
        step(w);
    }
    panic!("project failed to finish: {:?}", base(w, id));
}
fn completed_fixture_base(w: &mut WorldState, owner: N, id: &str) {
    w.airbases
        .get_or_insert_with(Default::default)
        .bases
        .push(Airbase {
            id: id.into(),
            district: id.into(),
            name: "Authored base fixture".into(),
            sponsor: owner,
            capacity_level: 1,
            support_level: 0,
            protection_level: 0,
            project: None,
            history: vec![],
        });
}
fn squadron(w: &mut WorldState, id: u32, base: &str, quantity: u32) {
    let spec = equipment::default_spec("air_light_attack");
    let profile = equipment::design_preview(w, N::France, &spec)
        .profile
        .unwrap();
    let day = clock::absolute_day(w);
    w.nation_mut(N::France)
        .equipment
        .get_or_insert_with(Default::default)
        .revisions
        .entry("s13-aircraft".into())
        .or_insert(equipment::DesignRevision {
            id: "s13-aircraft".into(),
            name: "Authored S13 aircraft".into(),
            specification_key: equipment::specification_key(&spec),
            spec,
            profile,
            created_day: day,
            certified_day: Some(day),
        });
    let state = w
        .nation_mut(N::France)
        .aviation
        .get_or_insert_with(AviationState::default);
    state.next_id = state.next_id.max(id + 1);
    state.squadrons.push(Squadron {
        id,
        name: format!("Test squadron {id}"),
        revision: "s13-aircraft".into(),
        assigned: quantity,
        base: Some(base.into()),
        transit: None,
        service_days_left: 0,
    });
}
fn sq(w: &WorldState, id: u32) -> &Squadron {
    w.nation(N::France)
        .aviation
        .as_ref()
        .unwrap()
        .squadrons
        .iter()
        .find(|s| s.id == id)
        .unwrap()
}
fn near(a: f64, b: f64) {
    assert!((a - b).abs() < 1e-9, "{a} != {b}");
}

#[test]
fn s13_foundation_spends_shared_construction_once_and_grants_capacity_only_on_completion() {
    let mut w = fixture();
    let id = near_district(&w, N::France, 2.0, 48.0);
    let authority_before = w.nation(N::France).program_budget.clone();
    establish(&mut w, &id, 2.0);
    assert_eq!(
        w.nation(N::France).program_budget,
        authority_before,
        "ordering creates no spending authority or payment"
    );
    assert_eq!(base(&w, &id).unwrap().capacity(), 0);
    assert!(tick_day(&mut w).is_empty());
    near(
        base(&w, &id).unwrap().project.as_ref().unwrap().paid_bn,
        0.0,
    );
    for _ in 0..11 {
        step(&mut w);
        assert_eq!(base(&w, &id).unwrap().capacity(), 0);
    }
    let saved = serde_json::to_string(&w).unwrap();
    let mut resumed: WorldState = serde_json::from_str(&saved).unwrap();
    let events = step(&mut w);
    step(&mut resumed);
    assert_eq!(w.airbases, resumed.airbases);
    assert_eq!(
        w.nation(N::France).program_budget,
        resumed.nation(N::France).program_budget
    );
    assert_eq!(base(&w, &id).unwrap().capacity(), 12);
    assert_eq!(events.len(), 1);
    near(base(&w, &id).unwrap().history[0].paid_bn, 0.024);
    near(
        w.nation(N::France)
            .program_budget
            .as_ref()
            .unwrap()
            .construction_spent_ytd_bn,
        0.024,
    );
    let before = serde_json::to_string(&w).unwrap();
    assert!(tick_day(&mut w).is_empty());
    assert_eq!(
        serde_json::to_string(&w).unwrap(),
        before,
        "same-day retry cannot post or finish work twice"
    );
}

#[test]
fn s13_daily_project_cap_and_other_construction_share_one_remaining_envelope() {
    let mut w = fixture();
    let id = near_district(&w, N::France, 2.0, 48.0);
    programs::set_construction_budget(&mut w, N::France, 0.0015).unwrap();
    establish(&mut w, &id, 1.0);
    clock::advance_date(&mut w);
    programs::begin_day(&mut w);
    programs::spend_construction(&mut w, N::France, 0.001).unwrap();
    tick_day(&mut w);
    near(
        base(&w, &id).unwrap().project.as_ref().unwrap().paid_bn,
        0.0005,
    );
    near(
        w.nation(N::France)
            .program_budget
            .as_ref()
            .unwrap()
            .construction_spent_today_bn,
        0.0015,
    );
    assert!(base(&w, &id)
        .unwrap()
        .project
        .as_ref()
        .unwrap()
        .reason
        .as_deref()
        .unwrap()
        .contains("Partial"));
    programs::finish_day(&mut w);
    step(&mut w);
    near(
        base(&w, &id).unwrap().project.as_ref().unwrap().paid_bn,
        0.0015,
    );
    near(
        w.nation(N::France)
            .program_budget
            .as_ref()
            .unwrap()
            .construction_spent_today_bn,
        0.001,
    );
    assert_eq!(base(&w, &id).unwrap().capacity(), 0);
}

#[test]
fn s13_all_three_improvements_apply_only_after_paid_work_and_resume() {
    let mut w = fixture();
    let id = near_district(&w, N::France, 2.0, 48.0);
    establish(&mut w, &id, 100.0);
    complete(&mut w, &id);
    for track in [
        UpgradeTrack::Capacity,
        UpgradeTrack::Support,
        UpgradeTrack::Protection,
    ] {
        let before = base(&w, &id).unwrap().level(track);
        let q = improvement_quote(base(&w, &id), track, 100.0);
        apply(
            &mut w,
            N::France,
            &AirbaseCommand::Upgrade {
                base: id.clone(),
                track,
                daily_budget_mn: 100.0,
            },
        )
        .unwrap();
        step(&mut w);
        assert_eq!(base(&w, &id).unwrap().level(track), before);
        let partial = base(&w, &id).unwrap().project.as_ref().unwrap().paid_bn;
        apply(
            &mut w,
            N::France,
            &AirbaseCommand::PauseUpgrade {
                base: id.clone(),
                paused: true,
            },
        )
        .unwrap();
        step(&mut w);
        near(
            base(&w, &id).unwrap().project.as_ref().unwrap().paid_bn,
            partial,
        );
        apply(
            &mut w,
            N::France,
            &AirbaseCommand::PauseUpgrade {
                base: id.clone(),
                paused: false,
            },
        )
        .unwrap();
        complete(&mut w, &id);
        assert_eq!(base(&w, &id).unwrap().level(track), before + 1);
        near(
            base(&w, &id).unwrap().history.last().unwrap().paid_bn,
            q.total_cost_bn,
        );
    }
    assert_eq!(base(&w, &id).unwrap().capacity(), 24);
    assert!(refusal(
        &w,
        N::France,
        &AirbaseCommand::Upgrade {
            base: id.clone(),
            track: UpgradeTrack::Support,
            daily_budget_mn: 100.0
        }
    )
    .unwrap()
    .contains("maximum level"));
    assert!(
        improvement_quote(base(&w, &id), UpgradeTrack::Support, 100.0)
            .effect
            .contains("one funded post-flight service day")
    );
    assert!(
        improvement_quote(base(&w, &id), UpgradeTrack::Protection, 100.0)
            .effect
            .contains("sortie aircraft losses")
    );
    validate(&w).unwrap();
}

#[test]
fn s13_cancelled_foundation_preserves_spent_money_and_creates_no_capacity() {
    let mut w = fixture();
    let id = near_district(&w, N::France, 2.0, 48.0);
    establish(&mut w, &id, 2.0);
    step(&mut w);
    let paid = base(&w, &id).unwrap().project.as_ref().unwrap().paid_bn;
    apply(
        &mut w,
        N::France,
        &AirbaseCommand::CancelUpgrade { base: id.clone() },
    )
    .unwrap();
    assert_eq!(base(&w, &id).unwrap().capacity(), 0);
    near(base(&w, &id).unwrap().history[0].paid_bn, paid);
    assert!(base(&w, &id).unwrap().history[0].cancelled_day.is_some());
    establish(&mut w, &id, 2.0);
    near(
        base(&w, &id).unwrap().project.as_ref().unwrap().paid_bn,
        0.0,
    );
    assert_eq!(base(&w, &id).unwrap().history.len(), 1);
    validate(&w).unwrap();
}

#[test]
fn s13_timed_rebase_reserves_inbound_capacity_and_excludes_all_transit_sorties() {
    let mut w = fixture();
    let origin = near_district(&w, N::France, 2.0, 48.0);
    let destination = near_district(&w, N::France, 5.0, 44.0);
    assert_ne!(origin, destination);
    completed_fixture_base(&mut w, N::France, &origin);
    completed_fixture_base(&mut w, N::France, &destination);
    squadron(&mut w, 1, &origin, 8);
    squadron(&mut w, 2, &origin, 4);
    apply(
        &mut w,
        N::France,
        &AirbaseCommand::Rebase {
            squadron: 1,
            base: destination.clone(),
        },
    )
    .unwrap();
    assert_eq!(occupied_capacity(&w, &origin), 4);
    assert_eq!(occupied_capacity(&w, &destination), 8);
    assert_eq!(free_capacity(&w, &destination), 4);
    assert!(assignment_refusal(&w, N::France, None, &destination, 5)
        .unwrap()
        .contains("4 aircraft spaces"));
    assert!(squadron_blocker(&w, N::France, sq(&w, 1))
        .unwrap()
        .contains("transit"));
    assert!(target_in_range(&w, N::France, sq(&w, 1), &destination)
        .unwrap_err()
        .contains("transit"));
    let transit = sq(&w, 1).transit.as_ref().unwrap();
    assert!(transit.arrival_day > transit.departed_day);
    let mut resumed: WorldState =
        serde_json::from_str(&serde_json::to_string(&w).unwrap()).unwrap();
    step(&mut w);
    step(&mut resumed);
    assert_eq!(w.airbases, resumed.airbases);
    assert_eq!(
        w.nation(N::France).aviation,
        resumed.nation(N::France).aviation
    );
    assert_eq!(sq(&w, 1).base.as_deref(), Some(destination.as_str()));
    assert!(sq(&w, 1).transit.is_none());
    assert_eq!(occupied_capacity(&w, &destination), 8);
}

#[test]
fn s13_lost_exact_host_access_holds_transit_without_losing_aircraft_and_can_redirect() {
    let mut w = fixture();
    let origin = near_district(&w, N::France, 2.0, 48.0);
    let foreign = near_district(&w, N::UK, 0.0, 51.0);
    completed_fixture_base(&mut w, N::France, &origin);
    completed_fixture_base(&mut w, N::UK, &foreign);
    squadron(&mut w, 1, &origin, 6);
    let cmd = AirbaseCommand::Rebase {
        squadron: 1,
        base: foreign.clone(),
    };
    assert!(refusal(&w, N::France, &cmd)
        .unwrap()
        .contains("has not granted"));
    let theatre = crate::theatre::home_theatre(&w, N::UK).unwrap();
    w.access.push(crate::theatre::Access {
        host: N::UK,
        seeker: N::France,
        theatre,
        since_year: w.year,
        since_month: w.month,
    });
    apply(&mut w, N::France, &cmd).unwrap();
    w.access
        .retain(|a| a.host != N::UK || a.seeker != N::France);
    step(&mut w);
    assert!(sq(&w, 1).transit.is_some());
    assert_eq!(sq(&w, 1).assigned, 6);
    assert!(squadron_blocker(&w, N::France, sq(&w, 1))
        .unwrap()
        .contains("Destination unavailable"));
    apply(
        &mut w,
        N::France,
        &AirbaseCommand::Rebase {
            squadron: 1,
            base: origin.clone(),
        },
    )
    .unwrap();
    step(&mut w);
    assert_eq!(sq(&w, 1).base.as_deref(), Some(origin.as_str()));
    assert!(sq(&w, 1).transit.is_none());
    assert_eq!(sq(&w, 1).assigned, 6);
}

#[test]
fn s13_captured_destination_cannot_finish_arrival_or_construction() {
    let mut w = fixture();
    let origin = near_district(&w, N::France, 2.0, 48.0);
    let destination = near_district(&w, N::France, 5.0, 44.0);
    completed_fixture_base(&mut w, N::France, &origin);
    completed_fixture_base(&mut w, N::France, &destination);
    squadron(&mut w, 1, &origin, 6);
    apply(
        &mut w,
        N::France,
        &AirbaseCommand::Upgrade {
            base: destination.clone(),
            track: UpgradeTrack::Capacity,
            daily_budget_mn: 100.0,
        },
    )
    .unwrap();
    apply(
        &mut w,
        N::France,
        &AirbaseCommand::Rebase {
            squadron: 1,
            base: destination.clone(),
        },
    )
    .unwrap();
    w.districts.insert(destination.clone(), N::USSR);
    step(&mut w);
    assert!(sq(&w, 1).transit.is_some());
    assert_eq!(sq(&w, 1).assigned, 6);
    near(
        base(&w, &destination)
            .unwrap()
            .project
            .as_ref()
            .unwrap()
            .paid_bn,
        0.0,
    );
    assert_eq!(base(&w, &destination).unwrap().capacity(), 12);
    assert!(base(&w, &destination)
        .unwrap()
        .project
        .as_ref()
        .unwrap()
        .reason
        .as_deref()
        .unwrap()
        .contains("has not granted"));
}

#[test]
fn s13_geographic_range_uses_installed_fuel_and_useful_refusals() {
    let mut w = fixture();
    let origin = near_district(&w, N::France, 2.0, 48.0);
    completed_fixture_base(&mut w, N::France, &origin);
    squadron(&mut w, 1, &origin, 6);
    near(
        target_in_range(&w, N::France, sq(&w, 1), &origin).unwrap(),
        0.0,
    );
    let distant = near_district(&w, N::USA, -120.0, 36.0);
    let reason = target_in_range(&w, N::France, sq(&w, 1), &distant).unwrap_err();
    assert!(reason.contains("access") || reason.contains("radius"));
    let spec = equipment::default_spec("air_light_attack");
    let mut extended = spec.clone();
    extended
        .components
        .insert("air_fuel".into(), "air_fuel_extended".into());
    near(range_km(&spec), 700.0);
    near(range_km(&extended), 945.0);
    let close = BaseLocation {
        district: String::new(),
        name: String::new(),
        lon: 179.0,
        lat: 0.0,
    };
    let across_dateline = BaseLocation {
        lon: -179.0,
        ..close.clone()
    };
    assert!(distance_km(&close, &across_dateline) < 225.0);
    assert_eq!(district_location(&origin).unwrap().district, origin);
}

#[test]
fn s13_validation_rejects_duplicate_projects_and_early_benefits() {
    let mut w = fixture();
    let id = near_district(&w, N::France, 2.0, 48.0);
    establish(&mut w, &id, 2.0);
    validate(&w).unwrap();
    let mut broken = w.clone();
    broken.airbases.as_mut().unwrap().bases[0].capacity_level = 1;
    assert!(validate(&broken).is_err());
    let mut broken = w.clone();
    broken.airbases.as_mut().unwrap().bases[0]
        .project
        .as_mut()
        .unwrap()
        .paid_bn = 0.001;
    assert!(validate(&broken).is_err());
    let mut broken = w.clone();
    broken
        .airbases
        .as_mut()
        .unwrap()
        .bases
        .push(base(&w, &id).unwrap().clone());
    assert!(validate(&broken).is_err());
    let raw = serde_json::to_string(&AirbaseCommand::Upgrade {
        base: id,
        track: UpgradeTrack::Support,
        daily_budget_mn: 2.0,
    })
    .unwrap();
    assert_eq!(
        serde_json::from_str::<AirbaseCommand>(&raw).unwrap(),
        AirbaseCommand::Upgrade {
            base: near_district(&w, N::France, 2.0, 48.0),
            track: UpgradeTrack::Support,
            daily_budget_mn: 2.0
        }
    );
}

#[test]
fn s13_foreign_foundation_and_improvement_use_sponsor_money_with_exact_live_access() {
    let mut w = fixture();
    let id = near_district(&w, N::UK, 0.0, 51.0);
    let theatre = crate::theatre::home_theatre(&w, N::UK).unwrap();
    let order = AirbaseCommand::Establish {
        district: id.clone(),
        name: "Sponsored host airfield".into(),
        daily_budget_mn: 100.0,
    };
    w.access.push(crate::theatre::Access {
        host: N::USA,
        seeker: N::France,
        theatre,
        since_year: w.year,
        since_month: w.month,
    });
    assert!(
        refusal(&w, N::France, &order)
            .unwrap()
            .contains("has not granted"),
        "another host in the same theatre cannot consent for this province"
    );
    w.access.push(crate::theatre::Access {
        host: N::UK,
        seeker: N::France,
        theatre,
        since_year: w.year,
        since_month: w.month,
    });
    w.sanctions.push((N::France, N::UK));
    assert!(refusal(&w, N::France, &order)
        .unwrap()
        .contains("Sanctions"));
    w.sanctions.retain(|s| *s != (N::France, N::UK));
    let ownership = w.districts.clone();
    let host_budget = w.nation(N::UK).program_budget.clone();
    let agreements = w.access.clone();
    let acknowledgement = apply(&mut w, N::France, &order).unwrap();
    assert!(acknowledgement.contains("Model assumption"));
    step(&mut w);
    let paid = base(&w, &id).unwrap().project.as_ref().unwrap().paid_bn;
    assert!(paid > 0.0);
    w.access
        .retain(|a| a.host != N::UK || a.seeker != N::France);
    step(&mut w);
    near(
        base(&w, &id).unwrap().project.as_ref().unwrap().paid_bn,
        paid,
    );
    assert_eq!(base(&w, &id).unwrap().capacity(), 0);
    assert!(base(&w, &id)
        .unwrap()
        .project
        .as_ref()
        .unwrap()
        .reason
        .as_deref()
        .unwrap()
        .contains("has not granted"));
    w.access = agreements.clone();
    complete(&mut w, &id);
    apply(
        &mut w,
        N::France,
        &AirbaseCommand::Upgrade {
            base: id.clone(),
            track: UpgradeTrack::Support,
            daily_budget_mn: 100.0,
        },
    )
    .unwrap();
    complete(&mut w, &id);
    assert_eq!(base(&w, &id).unwrap().support_level, 1);
    assert_eq!(base(&w, &id).unwrap().sponsor, N::France);
    near(
        w.nation(N::France)
            .program_budget
            .as_ref()
            .unwrap()
            .construction_spent_ytd_bn,
        0.036,
    );
    assert_eq!(w.nation(N::UK).program_budget, host_budget);
    assert_eq!(w.districts, ownership);
    assert_eq!(
        w.access, agreements,
        "paid work grants no political agreement"
    );
    validate(&w).unwrap();
}

#[test]
fn s13_foreign_permission_never_bypasses_host_physical_control() {
    use crate::world::{Belligerent, Conflict, Objective};
    let mut w = fixture();
    let id = near_district(&w, N::UK, 0.0, 51.0);
    let theatre = crate::theatre::home_theatre(&w, N::UK).unwrap();
    w.access.push(crate::theatre::Access {
        host: N::UK,
        seeker: N::France,
        theatre,
        since_year: w.year,
        since_month: w.month,
    });
    w.conflicts.push(Conflict {
        id: 913,
        theatre,
        side_a: vec![N::UK],
        side_b: vec![N::USSR],
        posture: vec![
            Belligerent::new(N::UK, 8, Objective::Hold),
            Belligerent::new(N::USSR, 8, Objective::Hold),
        ],
        control: 0.0,
        months: 0,
        quiet_months: 0,
        frozen_since: None,
        start_year: w.year,
        start_month: w.month,
        origin_attacker: N::USSR,
        invasion_declared: true,
        front: [(id.clone(), -1.0)].into_iter().collect(),
        pockets: vec![],
        aim: None,
    });
    let reason = site_access_refusal(&w, N::France, &id).unwrap();
    assert!(reason.contains("holds this province militarily"));
    assert!(refusal(
        &w,
        N::France,
        &AirbaseCommand::Establish {
            district: id.clone(),
            name: "Unavailable host".into(),
            daily_budget_mn: 100.0
        }
    )
    .is_some());
    w.conflicts[0].front.insert(id.clone(), 0.0);
    assert!(site_access_refusal(&w, N::France, &id)
        .unwrap()
        .contains("contested"));
    w.conflicts.clear();
    assert!(site_access_refusal(&w, N::France, &id).is_none());
}

#[test]
fn s13_queued_mission_must_be_cancelled_before_rebasing() {
    use crate::airmissions::{
        AirMissionsState, MissionCommand, MissionKind, MissionOrder, MissionStatus,
    };
    let mut w = fixture();
    let origin = near_district(&w, N::France, 2.0, 48.0);
    let destination = near_district(&w, N::France, 5.0, 44.0);
    completed_fixture_base(&mut w, N::France, &origin);
    completed_fixture_base(&mut w, N::France, &destination);
    squadron(&mut w, 1, &origin, 6);
    let day = clock::absolute_day(&w);
    w.air_missions = Some(AirMissionsState {
        next_id: 2,
        orders: vec![MissionOrder {
            id: 1,
            nation: N::France,
            squadron: 1,
            conflict: 913,
            kind: MissionKind::StrikeTarget,
            target: destination.clone(),
            issued_day: day,
            launch_day: day + 1,
            status: MissionStatus::Queued,
            report: None,
        }],
        ..Default::default()
    });
    let order = AirbaseCommand::Rebase {
        squadron: 1,
        base: destination,
    };
    let before = w.nation(N::France).aviation.clone();
    assert!(refusal(&w, N::France, &order)
        .unwrap()
        .contains("Cancel the mission"));
    assert!(apply(&mut w, N::France, &order).is_err());
    assert_eq!(w.nation(N::France).aviation, before);
    crate::airmissions::apply(&mut w, N::France, &MissionCommand::Cancel { mission: 1 }).unwrap();
    apply(&mut w, N::France, &order).unwrap();
    assert!(sq(&w, 1).transit.is_some());
}

#[test]
fn s13_load_rejects_nonexistent_overcapacity_and_underpriced_base_claims() {
    let mut w = fixture();
    let id = near_district(&w, N::France, 2.0, 48.0);
    squadron(&mut w, 1, &id, 6);
    assert!(validate(&w)
        .unwrap_err()
        .contains("without an airbase ledger"));
    w.nation_mut(N::France).aviation.as_mut().unwrap().squadrons[0].base = None;
    validate(&w).unwrap();
    completed_fixture_base(&mut w, N::France, &id);
    w.nation_mut(N::France).aviation.as_mut().unwrap().squadrons[0].base = Some(id.clone());
    squadron(&mut w, 2, &id, 7);
    assert!(validate(&w).unwrap_err().contains("exceed completed"));
    w.nation_mut(N::France)
        .aviation
        .as_mut()
        .unwrap()
        .squadrons
        .pop();
    validate(&w).unwrap();
    apply(
        &mut w,
        N::France,
        &AirbaseCommand::Upgrade {
            base: id.clone(),
            track: UpgradeTrack::Capacity,
            daily_budget_mn: 100.0,
        },
    )
    .unwrap();
    let mut forged = w.clone();
    forged.airbases.as_mut().unwrap().bases[0]
        .project
        .as_mut()
        .unwrap()
        .total_cost_bn *= 0.5;
    assert!(validate(&forged)
        .unwrap_err()
        .contains("price or lead time"));
    let mut forged = w.clone();
    forged.airbases.as_mut().unwrap().bases[0]
        .project
        .as_mut()
        .unwrap()
        .total_days = 1;
    assert!(validate(&forged)
        .unwrap_err()
        .contains("price or lead time"));
}
