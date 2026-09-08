//! People/operations integration contracts. Casualties are an explicit modeled
//! share of exposed military personnel, not historical battle calibration.
use spheres_sim::{
    campaign::{self, AirMission, Approach, NavalMission, OperationOrder},
    clock,
    init::world_1990,
    logistics, population, war,
    world::{Belligerent, Conflict, GameRules, NationId as N, Objective, WorldState},
};
use std::collections::BTreeMap;

const PARTICIPANTS: [N; 4] = [N::Iraq, N::Iran, N::Syria, N::SaudiArabia];

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
    for nation in &mut w.nations {
        nation.political_capital = 0.0;
    }
    population::enable(&mut w);
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

fn fronts() -> WorldState {
    let mut w = world();
    let mut coalition = conflict(&w, 1, N::Iran);
    coalition.side_b.push(N::Syria);
    coalition
        .posture
        .push(Belligerent::new(N::Syria, 8, Objective::Seize));
    w.conflicts.push(coalition);
    w.conflicts.push(conflict(&w, 2, N::SaudiArabia));
    for c in w.conflicts.clone() {
        for nation in c.participants() {
            campaign::set_order(
                &mut w,
                &OperationOrder {
                    conflict: c.id,
                    nation,
                    target: None,
                    approach: Approach::Advance,
                    reserve_bp: 1500,
                    air: AirMission::None,
                    naval: NavalMission::None,
                },
            )
            .unwrap();
        }
    }
    campaign::enroll(&mut w);
    w
}

fn pools_close(w: &WorldState) {
    for id in PARTICIPANTS {
        assert!(
            (campaign::accounted_force(w, id) - w.nation(id).mil_strength).abs() < 1e-7,
            "{id:?}: sectors, reserves and transit must share the national force"
        );
        let people = population::snapshot(w, id).unwrap();
        assert!(people.population_m.is_finite() && people.population_m >= 0.0);
        assert!((people.population_m - w.nation(id).population).abs() < 1e-9);
        assert!((people.employed_m + people.unemployed_m - people.labor_force_m).abs() < 1e-9);
        assert!(people.labor_force_m + people.military_m <= people.working_age_m + 1e-9);
    }
}

#[test]
fn two_front_coalition_casualties_post_one_national_population_debit() {
    let mut w = fronts();
    let mut witnessed = BTreeMap::<(u32, N), f64>::new();
    for _ in 0..4 {
        let before: BTreeMap<_, _> = PARTICIPANTS
            .into_iter()
            .map(|id| (id, population::snapshot(&w, id).unwrap()))
            .collect();
        // A no-combat control measures this day's shared strength replacement
        // independently of battle receipts and their population conversion.
        let mut control = w.clone();
        control.conflicts.clear();
        logistics::begin_month(&mut control);
        war::tick(&mut control);
        logistics::begin_month(&mut w);
        war::tick(&mut w);
        assert_eq!(
            w.conflicts.len(),
            2,
            "both fronts remain in the controlled observation"
        );
        for id in PARTICIPANTS {
            let losses = w
                .conflicts
                .iter()
                .map(|c| {
                    let loss = w
                        .campaign
                        .reports
                        .get(&campaign::order_key(c.id, id))
                        .map_or(0.0, |r| r.losses);
                    *witnessed.entry((c.id, id)).or_default() += loss;
                    loss
                })
                .sum::<f64>();
            let opening = control.nation(id).mil_strength;
            assert!(
                (opening - w.nation(id).mil_strength - losses).abs() < 1e-9,
                "{id:?}: actual national force losses must equal all front receipts"
            );
            let after = population::snapshot(&w, id).unwrap();
            let expected_deaths = before[&id].military_m * (losses / opening) * 0.20;
            // This tight military-stock check distinguishes one aggregate
            // charge from missing, duplicated or sequential per-front charges.
            assert!(
                (before[&id].military_m - after.military_m - expected_deaths).abs() < 1e-12,
                "{id:?}: military fatalities were not posted exactly once"
            );
            assert!(
                (before[&id].working_age_m - after.working_age_m - expected_deaths).abs() < 1e-10
            );
            assert!(
                (before[&id].population_m - after.population_m - expected_deaths).abs() < 1e-10
            );
            assert_eq!(before[&id].children_m, after.children_m);
            assert_eq!(before[&id].retirees_m, after.retirees_m);
            assert_eq!(before[&id].students_m, after.students_m);
        }
        pools_close(&w);
        clock::advance_date(&mut w);
    }
    assert!(
        witnessed[&(1, N::Iraq)] > 0.0 && witnessed[&(2, N::Iraq)] > 0.0,
        "the shared nation must actually suffer losses on both fronts"
    );
    assert!(
        witnessed[&(1, N::Iran)] > 0.0,
        "the opposing coalition must actually fight"
    );
}

#[test]
fn people_and_operational_war_replay_preserve_the_same_force_and_population_accounts() {
    let mut uninterrupted = fronts();
    spheres_sim::tick_day(&mut uninterrupted, &[]);
    let mut resumed = spheres_sim::load(&spheres_sim::save(&uninterrupted)).unwrap();
    for _ in 0..3 {
        assert_eq!(
            spheres_sim::tick_day(&mut uninterrupted, &[]),
            spheres_sim::tick_day(&mut resumed, &[])
        );
        pools_close(&uninterrupted);
        pools_close(&resumed);
        assert_eq!(
            spheres_sim::save(&uninterrupted),
            spheres_sim::save(&resumed),
            "same dated inputs after save must retain identical People/war outcomes"
        );
    }
}

#[test]
fn empty_military_staffing_blocks_replacements_without_posting_combat_deaths() {
    let mut staffed = world();
    let id = N::Iraq;
    staffed.nation_mut(id).mil_strength =
        war::sustained_force(staffed.nation(id), staffed.nation(id).mil_spend_gdp) * 0.5;
    let mut empty = staffed.clone();
    for (district, p) in &mut empty.population_system.provinces {
        if empty.districts.get(district) == Some(&id) {
            p.military_m = 0.0;
        }
    }
    if let Some(p) = empty.population_system.unallocated.get_mut(&id) {
        p.military_m = 0.0;
    }
    assert_eq!(population::military_staffing(&empty, id), 0.0);
    assert_eq!(population::military_staffing(&staffed, id), 1.0);
    let before = staffed.nation(id).mil_strength;
    let before_population = staffed.nation(id).population;
    war::tick(&mut staffed);
    war::tick(&mut empty);
    assert!(staffed.nation(id).mil_strength > before);
    assert_eq!(
        empty.nation(id).mil_strength,
        before,
        "replacement needs counted military staff"
    );
    assert_eq!(staffed.nation(id).population, before_population);
    assert_eq!(
        empty.nation(id).population,
        before_population,
        "staffing shortage or ordinary force adjustment must not post combat fatalities"
    );
}
