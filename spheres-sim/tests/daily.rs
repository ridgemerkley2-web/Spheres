//! Daily integration invariants, independent of legacy monthly calibration.
use spheres_sim::{clock, init::world_1990, load, save, state_hash, tick_day, tick_month, Command};
use spheres_sim::world::{GameRules, NationId, WorldState};

fn daily_world(seed: u64) -> WorldState {
    let mut w = world_1990(GameRules { seed, daily_simulation: true,
        resource_market: true, logistics_routes: true, physical_logistics: true,
        production_system: true, manufacturing_system: true, ..GameRules::default() });
    w.player = Some(NationId::USA);
    w
}

#[test]
fn daily_batch_is_the_same_dated_schedule_including_save_resume() {
    let mut a = daily_world(42);
    a.year = 2000;
    a.month = 2;
    let mut b = a.clone();
    let command = Command::SetInterestRate { nation: NationId::USA, rate: 0.07 };
    tick_month(&mut a, &[command.clone()]);
    for day in 1..=29 {
        tick_day(&mut b, if day == 1 { std::slice::from_ref(&command) } else { &[] });
        if day == 15 { b = load(&save(&b)).unwrap(); }
    }
    assert_eq!((a.year,a.month,a.day),(2000,3,1));
    assert_eq!(save(&a), save(&b));
    // A partial-month batch must not replay already settled days.
    for _ in 0..9 { tick_day(&mut a,&[]); tick_day(&mut b,&[]); }
    tick_month(&mut a,&[]);
    while b.month == 3 { tick_day(&mut b,&[]); }
    assert_eq!(state_hash(&a), state_hash(&b));
}

#[test]
fn daily_command_timing_changes_real_outcomes() {
    let mut early = daily_world(77);
    let mut late = early.clone();
    let command = Command::SetTaxRate { nation: NationId::USA, rate: 0.45 };
    for day in 1..=31 {
        tick_day(&mut early, if day==1 { std::slice::from_ref(&command) } else { &[] });
        tick_day(&mut late, if day==20 { std::slice::from_ref(&command) } else { &[] });
    }
    assert_ne!(early.nation(NationId::USA).gdp, late.nation(NationId::USA).gdp);
    assert_ne!(early.nation(NationId::USA).debt_gdp, late.nation(NationId::USA).debt_gdp);
}

#[test]
fn daily_combat_and_diplomacy_progress_without_a_monthly_burst() {
    let mut w = daily_world(10);
    let iraq = NationId::parse("Iraq").unwrap();
    let kuwait = NationId::parse("Kuwait").unwrap();
    spheres_sim::war::declare_war(&mut w, iraq, kuwait).unwrap();
    let cid = w.conflicts.iter().find(|c| c.origin_attacker == iraq).unwrap().id;
    let opening = w.conflict(cid).unwrap().clone();
    let magazines = w.nation(iraq).munitions;
    w.statecraft.reputation.push((NationId::USA, 20.0));
    let rep = w.reputation(NationId::USA);
    spheres_sim::statecraft::tick(&mut w);
    assert!((w.reputation(NationId::USA)-rep-0.15/31.0).abs()<1e-10);
    spheres_sim::war::tick(&mut w);
    let c = w.conflict(cid).unwrap();
    assert_eq!(c.months, 0, "one day cannot age a war by a month");
    assert!(c.posture.iter().all(|b| b.months_at_rung==0));
    assert_ne!(c.control, opening.control, "fronts move on day one");
    assert!((w.nation(iraq).munitions-magazines).abs()<0.02, "no full monthly magazine burn");
}

#[test]
fn daily_mode_survives_a_year_across_several_seeds() {
    for seed in [0, 42, 1990] {
        let mut w = daily_world(seed);
        for _ in 0..365 { tick_day(&mut w,&[]); }
        assert_eq!((w.year,w.month,w.day),(1991,1,1));
        for n in w.nations.iter().filter(|n| n.alive) {
            assert!(n.gdp.is_finite() && n.gdp>0.0, "GDP {:?}",n.id);
            assert!(n.population.is_finite() && n.population>0.0, "population {:?}",n.id);
            assert!(n.stability.is_finite() && (0.0..=100.0).contains(&n.stability));
        }
        let loaded = load(&save(&w)).unwrap();
        assert_eq!(state_hash(&w),state_hash(&loaded));
        assert_eq!(clock::absolute_day(&w),365);
    }
}
