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

/// Browser-like systems together, including the newly enrolled ministry books
/// and province accounts. These are invariants, not a sampled calibration bar.
#[test]
fn daily_ministries_and_province_accounts_close_and_resume_together() {
    for (seed, nation) in [(7, NationId::USA), (1990, NationId::Tonga)] {
        let mut w = daily_world(seed);
        w.player = Some(nation);
        spheres_sim::province_economy::enable(&mut w);
        let allocations = w.nation(nation).budget_for(w.year).allocations;
        let fiscal_year = w.year;
        spheres_sim::apply_command(&mut w, &Command::SetProgramBudget {
            nation, fiscal_year, allocations,
            departments: spheres_sim::programs::default_departments(),
        }).unwrap();
        let mut resumed = load(&save(&w)).unwrap();
        for day in 0..90 {
            tick_day(&mut w, &[]);
            tick_day(&mut resumed, &[]);
            if day == 44 { resumed = load(&save(&resumed)).unwrap(); }
            assert!(save(&w) == save(&resumed), "replay drift, seed {seed} day {day}");
            for n in w.nations.iter().filter(|n| n.alive) {
                assert!(n.gdp.is_finite() && n.gdp > 0.0, "{:?} GDP", n.id);
                assert!(n.population.is_finite() && n.population > 0.0, "{:?} population", n.id);
                if n.on_the_books() {
                    assert_eq!(n.debt_gdp, n.debt_bn.unwrap() / n.gdp);
                    assert!(n.treasury_bn.unwrap().is_finite() && n.treasury_bn.unwrap() >= 0.0);
                }
            }
            assert!(w.districts.values().all(|id| w.nation(*id).alive), "dead seat holds land");
        }
        for n in w.nations.iter().filter(|n| n.alive) {
            let view = spheres_sim::province_economy::snapshot(&w, n.id).unwrap();
            let sum = view.provinces.iter().map(|p| p.total_gdp_bn).sum::<f64>()
                + view.unallocated_gdp_bn;
            assert!((sum - n.gdp).abs() <= n.gdp.max(1.0) * 1e-10,
                "{:?} province output does not reconcile", n.id);
            assert!((view.sectors.iter().map(|s| s.gdp_bn).sum::<f64>() - n.gdp).abs()
                <= n.gdp.max(1.0) * 1e-10, "{:?} sector output does not reconcile", n.id);
        }
    }
}

/// The AI buy pass asks ONCE A MONTH in daily play, as its refusal memory,
/// its `PATIENCE` and its "a fourth is not asked this month" are all written
/// in months. Found 2026-09-03 by the browser check of this push: run daily,
/// the pass re-asked every day while a freshly signed contract was still at
/// sea (physical freight counts only the day's fills), so France signed SEVEN
/// copper contracts with the United States on seven consecutive January days
/// (seed 7, player Iraq), against one signing in three legacy months. Iron
/// rule 7: an invariant on one seeded board, no statistic.
#[test]
fn the_ai_buy_pass_asks_once_a_month_in_daily_play() {
    let mut w = daily_world(7);
    w.player = Some(NationId::parse("Iraq").unwrap());
    let mut signings = 0usize;
    for _ in 0..90 {
        for line in tick_day(&mut w, &[]) {
            if line.contains("sign a supply contract") { signings += 1; }
        }
    }
    let mut seen = std::collections::BTreeMap::new();
    for k in &w.resources.contracts {
        let legs = |legs: &[spheres_sim::resources::Leg]| legs.iter()
            .filter_map(|l| match l { spheres_sim::resources::Leg::Commodity { c, .. } => Some(*c), _ => None })
            .collect::<Vec<_>>();
        for c in legs(&k.give).into_iter().chain(legs(&k.take)) {
            let n = seen.entry((k.from, k.to, c, k.since)).or_insert(0);
            *n += 1;
            assert_eq!(*n, 1, "{:?} sold {:?} {:?} twice in month {}: the pass asked more than once",
                k.from, k.to, c, k.since);
        }
    }
    assert!(signings <= w.resources.contracts.len() + 3, "{signings} signings for {} contracts", w.resources.contracts.len());
}

/// Year-long replay: a dated command schedule lands on the same world whether
/// the days are stepped one at a time, resumed from a save partway, or batched
/// through `tick_month`. Recorded 2026-09-03 by the daily-push audit as the
/// replay invariant the proration repairs must preserve (seed 1990 read
/// 0xc88ea208970e7b2f before them; the value is not pinned here because the
/// repairs move it, the equalities are what is asserted).
#[test]
fn a_year_of_dated_commands_replays_the_same_stepped_resumed_or_batched() {
    for seed in [1990u64, 7] {
        let schedule = |day: usize| -> Vec<Command> {
            match day {
                1 => vec![Command::SetInterestRate { nation: NationId::USA, rate: 0.07 }],
                45 => vec![Command::SetTaxRate { nation: NationId::USA, rate: 0.40 }],
                200 => vec![Command::SetInterestRate { nation: NationId::USA, rate: 0.05 }],
                _ => vec![],
            }
        };
        let mut stepped = daily_world(seed);
        let mut resumed = daily_world(seed);
        let mut batched = daily_world(seed);
        for day in 1..=365 {
            tick_day(&mut stepped, &schedule(day));
            tick_day(&mut resumed, &schedule(day));
            if day == 100 { resumed = load(&save(&resumed)).unwrap(); }
        }
        let mut day = 1;
        while day <= 365 {
            let commands = schedule(day);
            // Batch a whole month only where no command falls inside it.
            let rest = spheres_sim::world::days_in_month(batched.year, batched.month) - batched.day + 1;
            let quiet = (day + 1..day + rest as usize).all(|d| schedule(d).is_empty());
            if quiet && batched.day == 1 && day + rest as usize - 1 <= 365 {
                tick_month(&mut batched, &commands);
                day += rest as usize;
            } else {
                tick_day(&mut batched, &commands);
                day += 1;
            }
        }
        assert_eq!((stepped.year, stepped.month, stepped.day), (1991, 1, 1));
        let h = state_hash(&stepped);
        assert_eq!(h, state_hash(&resumed), "seed {seed}: resume moved the world");
        assert_eq!(h, state_hash(&batched), "seed {seed}: batching moved the world");
        eprintln!("daily replay seed {seed}: {h:#018x}");
    }
}
