//! Controlled operational invariants. These are not historical calibration bars.
use spheres_sim::campaign::{AirMission, Approach, NavalMission, OperationOrder};
use spheres_sim::world::{Belligerent, Conflict, GameRules, NationId as N, Objective, WorldState};
use spheres_sim::{arsenal, campaign, clock, districts, init::world_1990, logistics, war, Command};

fn world() -> WorldState {
    let mut w = world_1990(GameRules {
        daily_simulation: true,
        military_operations: true,
        operational_warfare: 1,
        resource_market: true,
        logistics_routes: true,
        physical_logistics: true,
        ai_aggression: 0.0,
        ..Default::default()
    });
    w.player = Some(N::Iraq);
    for n in &mut w.nations {
        n.political_capital = 0.0;
    }
    w
}
fn conflict(w: &WorldState, id: u32, enemy: N) -> Conflict {
    Conflict {
        id,
        theatre: war::theatre_between(w, N::Iraq, enemy),
        side_a: vec![N::Iraq],
        side_b: vec![enemy],
        posture: vec![
            Belligerent::new(N::Iraq, 8, Objective::Seize),
            Belligerent::new(enemy, 8, Objective::Hold),
        ],
        control: 0.0,
        months: 0,
        quiet_months: 0,
        frozen_since: None,
        start_year: 1990,
        start_month: 1,
        origin_attacker: N::Iraq,
        invasion_declared: true,
        front: Default::default(),
        pockets: vec![],
        aim: None,
    }
}
fn order(approach: Approach) -> OperationOrder {
    OperationOrder {
        conflict: 1,
        nation: N::Iraq,
        target: None,
        approach,
        reserve_bp: 1500,
        air: AirMission::None,
        naval: NavalMission::None,
    }
}
fn step(w: &mut WorldState) {
    logistics::begin_month(w);
    war::tick(w);
    clock::advance_date(w);
}
fn readiness(w: &WorldState) -> f64 {
    let mut numerator = 0.0;
    let mut mass = 0.0;
    for s in w.campaign.sectors.values().filter(|s| s.nation == N::Iraq) {
        numerator += s.strength * s.cohesion;
        mass += s.strength;
    }
    numerator / mass.max(1e-9)
}
fn staged() -> WorldState {
    let mut w = world();
    w.conflicts.push(conflict(&w, 1, N::Iran));
    w
}

#[test]
fn enrollment_only_migrates_existing_wars_even_after_day_zero_save() {
    let mut w = staged();
    campaign::enroll(&mut w);
    w = spheres_sim::load(&spheres_sim::save(&w)).unwrap();
    w.conflicts.push(conflict(&w, 2, N::SaudiArabia));
    campaign::enroll(&mut w);
    step(&mut w);
    assert!(w.campaign.sectors.values().any(|s| s.conflict == 1));
    assert!(w.campaign.sectors.values().all(|s| s.conflict != 2));
    assert!(w.campaign.transfers.iter().any(|t| t.conflict == Some(2)));

    let mut empty = world();
    campaign::enroll(&mut empty);
    empty = spheres_sim::load(&spheres_sim::save(&empty)).unwrap();
    empty.conflicts.push(conflict(&empty, 1, N::Iran));
    campaign::enroll(&mut empty);
    step(&mut empty);
    assert!(empty.campaign.sectors.is_empty());
    assert!(empty
        .campaign
        .transfers
        .iter()
        .any(|t| t.conflict == Some(1)));
}

#[test]
fn empty_campaign_activation_does_not_teleport_the_first_new_deployment() {
    let mut w = world();
    step(&mut w);
    assert!(w.campaign.initialized);
    w.conflicts.push(conflict(&w, 1, N::Iran));
    step(&mut w);
    assert!(
        w.campaign.sectors.values().all(|s| s.nation != N::Iraq),
        "new campaign forces must travel"
    );
    let trips: Vec<_> = w
        .campaign
        .transfers
        .iter()
        .filter(|t| t.nation == N::Iraq && t.conflict == Some(1))
        .collect();
    assert!(!trips.is_empty());
    assert!(trips
        .iter()
        .all(|t| t.arrives_day >= clock::absolute_day(&w)));
    for _ in 0..5 {
        step(&mut w);
    }
    assert!(
        w.campaign.sectors.values().any(|s| s.nation == N::Iraq),
        "real due dates eventually deploy the force"
    );
}

#[test]
fn migration_and_two_wars_conserve_national_force_through_losses_and_transfers() {
    let mut w = staged();
    w.conflicts.push(conflict(&w, 2, N::SaudiArabia));
    for _ in 0..18 {
        step(&mut w);
        for n in [N::Iraq, N::Iran, N::SaudiArabia] {
            let held = campaign::accounted_force(&w, n);
            let actual = w.nation(n).mil_strength;
            assert!(
                (held - actual).abs() < 1e-7,
                "{n:?}: pools {held} differ from national force {actual}"
            );
        }
        assert!(w
            .campaign
            .sectors
            .values()
            .all(|s| s.strength >= 0.0 && (0.0..=1.0).contains(&s.cohesion)));
    }
    let save = spheres_sim::save(&w);
    let loaded = spheres_sim::load(&save).unwrap();
    assert_eq!(
        w.campaign, loaded.campaign,
        "loading cannot redeploy a second migration army"
    );
}

#[test]
fn continuous_breakthrough_exhausts_more_cohesion_than_holding_and_orders_do_not_refresh_it() {
    let mut a = staged();
    step(&mut a);
    let mut b = a.clone();
    spheres_sim::apply_command(
        &mut a,
        &Command::SetOperation {
            order: order(Approach::Breakthrough),
        },
    )
    .unwrap();
    spheres_sim::apply_command(
        &mut b,
        &Command::SetOperation {
            order: order(Approach::Hold),
        },
    )
    .unwrap();
    for _ in 0..10 {
        step(&mut a);
        step(&mut b);
    }
    assert!(
        readiness(&b) > readiness(&a) + 0.08,
        "continuous attack must have a real cohesion cost: hold {}, attack {}",
        readiness(&b),
        readiness(&a)
    );
    let before = a.campaign.sectors.clone();
    campaign::set_order(&mut a, &order(Approach::Hold)).unwrap();
    campaign::set_order(&mut a, &order(Approach::Breakthrough)).unwrap();
    assert_eq!(
        before, a.campaign.sectors,
        "switching orders is not preparation or recovery"
    );
    assert!(
        campaign::consumption_multiplier(&a, 1, N::Iraq)
            > campaign::consumption_multiplier(&b, 1, N::Iraq)
    );
}

#[test]
fn changed_focus_moves_existing_cohorts_on_saved_dates_without_resetting_fatigue() {
    let mut w = staged();
    step(&mut w);
    let sector = w
        .campaign
        .sectors
        .values()
        .find(|s| {
            s.nation == N::Iraq
                && districts::adj_of(&s.district)
                    .iter()
                    .any(|d| w.districts.get(d) == Some(&N::Iran))
        })
        .unwrap()
        .clone();
    let target = districts::adj_of(&sector.district)
        .iter()
        .find(|d| w.districts.get(*d) == Some(&N::Iran))
        .unwrap()
        .clone();
    for s in w
        .campaign
        .sectors
        .values_mut()
        .filter(|s| s.nation == N::Iraq)
    {
        s.cohesion = 0.31;
    }
    // Exhaust the existing reserve cohort too: reserve reinforcements now use
    // real mapped routes and are included among the dated journeys below.
    // Leaving that cohort fresh would legitimately introduce rested troops.
    w.campaign.reserves.get_mut(&N::Iraq).unwrap().cohesion = 0.31;
    let mut o = order(Approach::Advance);
    o.target = Some(target);
    campaign::set_order(&mut w, &o).unwrap();
    step(&mut w);
    let trips: Vec<_> = w
        .campaign
        .transfers
        .iter()
        .filter(|t| t.nation == N::Iraq && t.route.len() > 1)
        .collect();
    assert!(
        !trips.is_empty(),
        "staff focus must move finite cohorts rather than instantly reweight every front"
    );
    assert!(trips
        .iter()
        .all(|t| t.arrives_day >= clock::absolute_day(&w) && t.cohesion < 0.5));
    let saved = spheres_sim::save(&w);
    let mut loaded = spheres_sim::load(&saved).unwrap();
    for _ in 0..8 {
        step(&mut w);
        step(&mut loaded);
    }
    assert_eq!(spheres_sim::save(&w), spheres_sim::save(&loaded));
}

#[test]
fn forecasts_are_pure_and_read_saved_observations_not_exact_enemy_inventories() {
    let mut w = staged();
    step(&mut w);
    let before = spheres_sim::save(&w);
    let expected = serde_json::to_value(campaign::view(&w, 1, N::Iraq).unwrap()).unwrap();
    assert_eq!(
        before,
        spheres_sim::save(&w),
        "a preview must not alter RNG, supply or orders"
    );
    let mut hidden = w.clone();
    hidden.nation_mut(N::Iran).mil_strength *= 100.0;
    hidden.nation_mut(N::Iran).munitions = 0.0;
    hidden.nation_mut(N::Iran).arsenal.held.clear();
    assert_eq!(
        expected,
        serde_json::to_value(campaign::view(&hidden, 1, N::Iraq).unwrap()).unwrap(),
        "unobserved live enemy data leaked into the viewer's forecast"
    );
    assert!(
        campaign::view(&w, 1, N::USA).is_none(),
        "nonparticipants cannot read national operational detail"
    );
}

#[test]
fn unsupported_missions_buy_no_capability_and_standoff_cannot_occupy_ground() {
    let mut w = staged();
    w.nation_mut(N::Iraq).arsenal.held.retain(|h| {
        !matches!(
            arsenal::DECK[h.kit as usize].class,
            arsenal::Class::Air | arsenal::Class::Naval
        )
    });
    step(&mut w);
    let mut mission = w.clone();
    let mut o = order(Approach::Advance);
    o.air = AirMission::GroundSupport;
    o.naval = NavalMission::SeaDenial;
    campaign::set_order(&mut mission, &o).unwrap();
    step(&mut w);
    step(&mut mission);
    assert_eq!(
        w.nation(N::Iran).mil_strength,
        mission.nation(N::Iran).mil_strength
    );
    assert_eq!(
        w.conflict(1).unwrap().front,
        mission.conflict(1).unwrap().front
    );
    let mut standoff = staged();
    standoff
        .conflict_mut(1)
        .unwrap()
        .posture_mut(N::Iraq)
        .unwrap()
        .rung = 6;
    let mut strike = order(Approach::Breakthrough);
    strike.air = AirMission::GroundSupport;
    campaign::set_order(&mut standoff, &strike).unwrap();
    for _ in 0..8 {
        step(&mut standoff);
    }
    assert_eq!(
        standoff.conflict(1).unwrap().control,
        0.0,
        "air-only commitment cannot occupy a district"
    );
}

#[test]
fn invalid_orders_are_atomic_and_default_legacy_saves_stay_unenrolled() {
    let mut w = staged();
    let saved = spheres_sim::save(&w);
    let mut invalid = order(Approach::Advance);
    invalid.target = Some("US-HI".into());
    assert!(campaign::set_order(&mut w, &invalid).is_err());
    invalid.target = None;
    invalid.reserve_bp = 10_001;
    assert!(campaign::set_order(&mut w, &invalid).is_err());
    assert_eq!(saved, spheres_sim::save(&w));
    let old = world_1990(GameRules::default());
    let serialized = spheres_sim::save(&old);
    assert!(!serialized.contains("operational_warfare"));
    assert!(!serialized.contains("\"campaign\":"));
    assert!(!campaign::enabled(&old));
}
