//! Deterministic controlled scenarios, not statistical historical calibration.
use spheres_sim::{arsenal, manufacturing, production, war};
use spheres_sim::init::world_1990;
use spheres_sim::world::{Belligerent, Conflict, GameRules, NationId as N, Objective, WorldState};

fn world(daily: bool) -> WorldState {
    let mut w = world_1990(GameRules { daily_simulation: daily,
        military_operations: true, ..GameRules::default() });
    w.player = Some(N::France);
    for n in &mut w.nations { n.political_capital = 0.0; }
    w
}
fn conflict(w: &WorldState, id: u32, a: N, b: N) -> Conflict {
    Conflict { id, theatre: war::theatre_between(w, a, b), side_a: vec![a], side_b: vec![b],
        posture: vec![Belligerent::new(a, 8, Objective::Seize), Belligerent::new(b, 8, Objective::Hold)],
        control: 0.0, months: 0, quiet_months: 0, frozen_since: None, start_year: 1990, start_month: 1,
        origin_attacker: a, invasion_declared: true, front: Default::default(), pockets: vec![], aim: None }
}

#[test]
fn a_zero_deployment_ally_loses_no_force_or_equipment_in_either_cadence() {
    for daily in [false, true] {
        let mut peace = world(daily);
        let mut fight = peace.clone();
        let mut c = conflict(&fight, 1, N::Iraq, N::Kuwait);
        c.side_b.push(N::France);
        c.posture.push(Belligerent::new(N::France, 1, Objective::Hold));
        assert_eq!(war::committed_force(&fight, &c, N::France), 0.0);
        fight.conflicts.push(c);
        war::tick(&mut peace);
        war::tick(&mut fight);
        assert_eq!(fight.nation(N::France).mil_strength, peace.nation(N::France).mil_strength,
            "zero battlefield exposure must not cost national force, daily={daily}");
        assert_eq!(arsenal::book_value(fight.nation(N::France)), arsenal::book_value(peace.nation(N::France)));
    }
}

#[test]
fn simultaneous_home_conflicts_cannot_duplicate_the_national_army() {
    let mut w = world(true);
    w.conflicts = vec![conflict(&w, 1, N::Iraq, N::Kuwait), conflict(&w, 2, N::Iraq, N::SaudiArabia)];
    let total: f64 = w.conflicts.iter().map(|c| war::committed_force(&w, c, N::Iraq)).sum();
    assert!(total <= w.nation(N::Iraq).mil_strength + 1e-10, "{total} duplicates the national army");
}

#[test]
fn occupation_stops_an_arms_line_and_recapture_resumes_without_transfer() {
    let mut w = world(true);
    w.rules.resource_market = true;
    w.rules.manufacturing_system = true;
    w.rules.resource_gates = false; // isolate physical control from material scarcity
    let d = w.districts.iter().find(|(_, o)| **o == N::France).unwrap().0.clone();
    w.production.provinces.push(production::ProvinceCapabilities { district: d.clone(),
        infrastructure: 0, civilian_industry: 0, power_grid: 0, research_centers: 0, arms_plants: 1 });
    w.production.provinces.sort_by(|a, b| a.district.cmp(&b.district));
    let kit = arsenal::DECK[arsenal::available(w.nation(N::France))[0] as usize].id;
    let id = manufacturing::start_line(&mut w, N::France, &d, kit).unwrap();
    let mut c = conflict(&w, 3, N::Germany, N::France);
    c.front.insert(d.clone(), 1.0);
    w.conflicts.push(c);
    let line = w.manufacturing.lines.iter().find(|l| l.id == id).unwrap();
    assert!(manufacturing::line_blocker(&w, line).is_some(), "enemy-held factory cannot operate");
    arsenal::tick(&mut w);
    assert_eq!(w.manufacturing.lines[0].ordered_bn, 0.0);
    assert_eq!(w.districts.get(&d), Some(&N::France), "occupation is not sovereignty");
    w.conflicts[0].front.insert(d.clone(), -1.0);
    spheres_sim::clock::advance_date(&mut w);
    arsenal::tick(&mut w);
    assert!(w.manufacturing.lines[0].ordered_bn > 0.0);
}

#[test]
fn an_overseas_ceiling_is_shared_and_player_allocation_releases_reserves() {
    let mut w = world(true);
    w.player = Some(N::USA);
    w.conflicts = vec![conflict(&w, 1, N::USA, N::Iraq), conflict(&w, 2, N::USA, N::Vietnam),
        conflict(&w, 3, N::USA, N::Iran)];
    let before = spheres_sim::operations::view(&w, N::USA);
    assert!(before.overseas_deployed <= before.overseas_limit + 1e-10);
    assert!(before.deployed <= before.structure + 1e-10);
    for cid in 1..=3 {
        spheres_sim::apply_command(&mut w, &spheres_sim::Command::SetForceAllocation {
            conflict: cid, nation: N::USA, share_bp: Some(0) }).unwrap();
    }
    let after = spheres_sim::operations::view(&w, N::USA);
    assert_eq!(after.deployed, 0.0);
    assert_eq!(after.reserve, after.structure);
    assert!(after.deployments.iter().all(|r| r.burn_monthly == 0.0));
    let save = spheres_sim::save(&w);
    assert!(spheres_sim::apply_command(&mut w, &spheres_sim::Command::SetForceAllocation {
        conflict: 1, nation: N::USA, share_bp: Some(10001) }).is_err());
    assert_eq!(save, spheres_sim::save(&w), "rejection cannot spend or mutate");
    let mut loaded = spheres_sim::load(&save).unwrap();
    war::tick(&mut w);
    war::tick(&mut loaded);
    assert_eq!(spheres_sim::save(&w), spheres_sim::save(&loaded));
}

#[test]
fn all_theatres_use_one_opening_force_and_equipment_snapshot() {
    let mut a = world(true);
    a.conflicts = vec![conflict(&a, 1, N::Iraq, N::Kuwait), conflict(&a, 2, N::Iraq, N::SaudiArabia)];
    let mut b = a.clone();
    b.conflicts.reverse();
    war::tick(&mut a);
    war::tick(&mut b);
    for id in [N::Iraq, N::Kuwait, N::SaudiArabia] {
        assert_eq!(a.nation(id).mil_strength, b.nation(id).mil_strength);
        assert_eq!(a.nation(id).munitions, b.nation(id).munitions);
        assert_eq!(arsenal::book_value(a.nation(id)), arsenal::book_value(b.nation(id)));
    }
    for c in &a.conflicts {
        assert_eq!(c.front, b.conflict(c.id).unwrap().front);
    }
}

#[test]
fn battlefield_material_losses_consume_inventory_but_not_pending_orders() {
    let mut peace = world(true);
    let mut fight = peace.clone();
    fight.conflicts = vec![conflict(&fight, 1, N::Iraq, N::Kuwait)];
    let kit = arsenal::index_of("arm_gen2").unwrap();
    for w in [&mut peace, &mut fight] {
        w.nation_mut(N::Iraq).arsenal.orders.push(arsenal::Order { kit, units: 100.0, due: 120, due_days: Some(3600) });
    }
    war::tick(&mut peace);
    war::tick(&mut fight);
    assert!(arsenal::book_value(fight.nation(N::Iraq)) < arsenal::book_value(peace.nation(N::Iraq)));
    assert_eq!(fight.nation(N::Iraq).arsenal.orders[0].units, 100.0);
    assert_eq!(fight.nation(N::Iraq).arsenal.orders[0].due_days, Some(3600));
}

#[test]
fn equal_equipment_value_cannot_substitute_ships_for_land_capability() {
    let mut land = world(true);
    let mut sea = land.clone();
    let armour = arsenal::DECK.iter().position(|d| d.class == arsenal::Class::Armour).unwrap();
    let naval = arsenal::DECK.iter().position(|d| d.class == arsenal::Class::Naval).unwrap();
    for (w, kit) in [(&mut land, armour), (&mut sea, naval)] {
        w.nation_mut(N::Iraq).arsenal.held = vec![arsenal::Holding {
            kit: kit as u16, units: 10.0 / arsenal::DECK[kit].unit_cost, age: 0.0 }];
    }
    assert!((arsenal::book_value(land.nation(N::Iraq)) - arsenal::book_value(sea.nation(N::Iraq))).abs() < 1e-10);
    let a = spheres_sim::operations::capabilities(land.nation(N::Iraq));
    let b = spheres_sim::operations::capabilities(sea.nation(N::Iraq));
    assert!(a.land > b.land && b.lift > a.lift);
    let c = conflict(&land, 1, N::Iraq, N::Kuwait);
    assert!(war::seize_terms(&land, &c, 0.0).0 > war::seize_terms(&sea, &c, 0.0).0);
}

#[test]
fn controller_handles_contested_recaptured_and_conflicting_fronts() {
    let mut w = world(true);
    let d = w.districts.iter().find(|(_, o)| **o == N::France).unwrap().0.clone();
    let mut c = conflict(&w, 1, N::Germany, N::France);
    c.front.insert(d.clone(), 0.0);
    w.conflicts.push(c);
    assert_eq!(spheres_sim::control::controller(&w, &d), None);
    assert!(spheres_sim::resources::district_contested(&w, &d));
    w.conflicts[0].front.insert(d.clone(), -1.0);
    assert_eq!(spheres_sim::control::controller(&w, &d), Some(N::France));
    assert!(!spheres_sim::resources::district_contested(&w, &d));
    w.conflicts[0].front.insert(d.clone(), 1.0);
    let mut other = conflict(&w, 2, N::Italy, N::France);
    other.front.insert(d.clone(), 1.0);
    w.conflicts.push(other);
    assert_eq!(spheres_sim::control::controller(&w, &d), None);
    w.conflicts.reverse();
    assert_eq!(spheres_sim::control::controller(&w, &d), None);
}

#[test]
fn daily_batched_and_saved_schedules_replay_identically() {
    let mut a = world(true);
    a.conflicts = vec![conflict(&a, 1, N::Iraq, N::Kuwait)];
    let mut b = a.clone();
    for _ in 0..10 { spheres_sim::tick_day(&mut a, &[]); }
    a = spheres_sim::load(&spheres_sim::save(&a)).unwrap();
    for _ in 10..31 { spheres_sim::tick_day(&mut a, &[]); }
    spheres_sim::tick_month(&mut b, &[]);
    assert_eq!(spheres_sim::save(&a), spheres_sim::save(&b));
}
