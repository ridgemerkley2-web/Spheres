//! Population accounting and replay contracts. These are deterministic
//! invariants, not historical-frequency calibration bars.
use spheres_sim::{
    init::world_1990,
    load,
    population::{self, Course, Policy},
    save, tick_day, tick_month,
    world::{GameRules, NationId as N, WorldState},
    Command,
};

fn world() -> WorldState {
    let mut w = world_1990(GameRules {
        daily_simulation: true,
        ai_aggression: 0.0,
        ..GameRules::default()
    });
    w.player = Some(N::USA);
    population::enable(&mut w);
    w
}
fn accounting(w: &WorldState) {
    for n in w.nations.iter().filter(|n| n.alive) {
        let s = population::snapshot(w, n.id).unwrap();
        assert!(s.population_m.is_finite() && s.population_m >= 0.0);
        assert!(
            (s.population_m - n.population).abs() < 1e-7,
            "{} population ledger must close",
            n.id.name()
        );
        assert!((s.children_m + s.working_age_m + s.retirees_m - s.population_m).abs() < 1e-7);
        assert!((s.employed_m + s.unemployed_m - s.labor_force_m).abs() < 1e-7);
        assert!(s.employed_m <= s.labor_force_m + 1e-8);
        assert!(s.labor_force_m + s.military_m <= s.working_age_m + 1e-8);
        assert!((s.classes.iter().map(|c| c.people_m).sum::<f64>() - s.population_m).abs() < 1e-7);
        assert!(
            (s.education.iter().map(|e| e.people_m).sum::<f64>() - s.working_age_m).abs() < 1e-7
        );
        assert!(s.sectors.iter().all(|x| x.employed_m >= 0.0
            && x.employed_m <= x.jobs_m + 1e-8
            && x.staffing.is_finite()));
        assert!(n.gdp.is_finite() && n.gdp > 0.0);
    }
}
#[test]
fn disabled_views_ticks_and_repeated_dates_are_pure() {
    let mut legacy = world_1990(GameRules::default());
    let original = save(&legacy);
    population::tick(&mut legacy);
    population::ai_tick(&mut legacy);
    assert!(population::snapshot(&legacy, N::USA).is_none());
    assert_eq!(save(&legacy), original);
    let mut w = world();
    population::tick(&mut w);
    let first = save(&w);
    population::tick(&mut w);
    population::enable(&mut w);
    for n in &w.nations {
        let _ = population::snapshot(&w, n.id);
    }
    assert_eq!(
        save(&w),
        first,
        "one actual date accrues one population flow; views are pure"
    );
    accounting(&w);
}
#[test]
fn dated_policy_replay_survives_batching_and_midmonth_resume() {
    let mut daily = world();
    let mut batched = daily.clone();
    for _ in 0..19 {
        tick_day(&mut daily, &[]);
        tick_day(&mut batched, &[]);
    }
    let command = Command::SetPopulationPolicy {
        nation: N::USA,
        policy: Policy::TradeSchools,
    };
    tick_day(&mut daily, &[command.clone()]);
    tick_day(&mut batched, &[command]);
    for _ in 0..7 {
        tick_day(&mut daily, &[]);
        tick_day(&mut batched, &[]);
    }
    batched = load(&save(&batched)).unwrap();
    while daily.month == 1 {
        tick_day(&mut daily, &[]);
    }
    tick_month(&mut batched, &[]);
    assert_eq!(save(&daily), save(&batched));
    for _ in 0..2 {
        let month = daily.month;
        while daily.month == month {
            tick_day(&mut daily, &[]);
        }
        tick_month(&mut batched, &[]);
        assert_eq!(
            save(&daily),
            save(&batched),
            "same dated orders must have the same timeline"
        );
        accounting(&daily);
    }
}
#[test]
fn no_teachers_or_researchers_means_no_course_progress_or_research() {
    let mut w = world();
    let d = w.population_system.nations[&N::USA].province_ids[0].clone();
    for key in w.population_system.nations[&N::USA].province_ids.clone() {
        let p = w.population_system.provinces.get_mut(&key).unwrap();
        p.working[1] += p.working[4];
        p.working[4] = 0.0;
        p.filled[7][2] = 0.0;
    }
    let p = w.population_system.provinces.get_mut(&d).unwrap();
    p.courses.push(Course {
        from: 1,
        to: 4,
        people_m: 0.0001,
        started_day: 0,
        required_days: 1461.0,
        funded_days: 0.0,
    });
    population::tick(&mut w);
    assert_eq!(
        w.population_system.provinces[&d].courses[0].funded_days,
        0.0
    );
    assert_eq!(population::research_multiplier(&w, N::USA), Some(0.0));
}
#[test]
fn combat_losses_remove_military_adults_without_killing_students_or_dependents() {
    let mut w = world();
    let before = population::snapshot(&w, N::USA).unwrap();
    let children = before.children_m;
    let retirees = before.retirees_m;
    let gdp = w.nation(N::USA).gdp;
    let rng = w.rng.clone();
    population::record_casualties(&mut w, N::USA, 0.25);
    let after = population::snapshot(&w, N::USA).unwrap();
    assert!(
        (before.population_m - after.population_m - before.military_m * 0.25 * 0.20).abs() < 1e-8
    );
    assert_eq!(after.children_m, children);
    assert_eq!(after.retirees_m, retirees);
    assert_eq!(w.nation(N::USA).gdp, gdp);
    assert_eq!(w.rng, rng);
    accounting(&w);
}
#[test]
#[ignore = "Release census: two independent ten-year daily campaigns; diagnostic balance evidence, not historical calibration"]
fn decade_daily_population_census() {
    run_decade_population_census(&[7, 1990]);
}

#[test]
#[ignore = "Release census: standalone second ordinary browser campaign, sharing the decade invariants"]
fn second_seed_daily_population_census() {
    run_decade_population_census(&[1990]);
}

fn run_decade_population_census(seeds: &[u64]) {
    for &seed in seeds {
        let mut w = world_1990(GameRules {
            seed,
            daily_simulation: true,
            ..GameRules::default()
        });
        w.player = Some(N::USA);
        // Fresh browser order from fresh_play_rules/play_rules: historical
        // estimates first, then runtime feature flags, then companies/people.
        spheres_sim::starting_industry::enable_new_world(&mut w).unwrap();
        spheres_sim::starting_industry::enrich_new_world(&mut w).unwrap();
        w.rules.resource_market = true;
        w.rules.ideology_blocs = true;
        w.rules.ideology_takeover = false;
        spheres_sim::government::ensure_all(&mut w);
        w.rules.logistics_routes = true;
        w.rules.physical_logistics = true;
        w.rules.military_operations = true;
        w.rules.production_system = true;
        w.rules.industry_rebuild = true;
        w.rules.manufacturing_system = true;
        spheres_sim::province_economy::enable(&mut w);
        spheres_sim::companies::enable(&mut w);
        population::enable(&mut w);
        spheres_sim::resources::warm(&mut w);
        // Optional Economic Competition has a separate bounded timing smoke
        // below, so these long campaigns match ordinary browser defaults.
        let starts: Vec<_> = [N::USA, N::China, N::India, N::Japan, N::Tonga]
            .into_iter()
            .map(|id| (id, w.nation(id).gdp, w.nation(id).population))
            .collect();
        while w.year < 2000 {
            tick_day(&mut w, &[]);
            if w.day == 1 && w.month == 1 {
                accounting(&w);
                println!("census seed={seed} year={} alive={} provinces={} companies={} population_m={:.3}",w.year,w.nations.iter().filter(|n|n.alive).count(),w.population_system.provinces.len(),w.companies.enabled,w.nations.iter().filter(|n|n.alive).map(|n|n.population).sum::<f64>());
            }
        }
        for (id, gdp, pop) in starts {
            if !w.nation(id).alive {
                continue;
            }
            let s = population::snapshot(&w, id).unwrap();
            println!("seed={seed} {} GDPx={:.3} populationx={:.3} unemployment={:.3} participation={:.3} living={:.2} researchers={:.3}",id.name(),w.nation(id).gdp/gdp,s.population_m/pop,s.unemployment_rate,s.participation_rate,s.living_standards,s.research_multiplier);
        }
    }
}

#[test]
fn secondary_catch_up_connects_basic_adults_to_delayed_technical_training() {
    let mut w = world();
    let id = N::USA;
    let district = w.population_system.nations[&id].province_ids[0].clone();
    w.districts.retain(|d, _| *d == district);
    for nation in &mut w.nations {
        nation.alive = nation.id == id;
    }
    w.population_system.provinces.retain(|d, _| *d == district);
    w.population_system.unallocated.clear();
    w.population_system.nations.retain(|n, _| *n == id);
    let p = w.population_system.provinces.get_mut(&district).unwrap();
    p.children.clear();
    p.retirees_m = 0.0;
    p.working = [0.0, 0.70, 0.10, 0.10, 0.10];
    p.birth_rate_per_adult = 0.0;
    p.reference_population_m = 1.0;
    p.military_m = 0.0;
    p.participation = 0.8;
    p.participation_reference = 0.8;
    p.jobs_reference = [[0.0; 3]; 8];
    p.jobs_reference[2] = [0.30, 0.13, 0.0];
    p.jobs_reference[7] = [0.0, 0.0, 0.035];
    p.jobs = p.jobs_reference;
    p.courses.clear();
    w.nation_mut(id).mil_strength = 0.0;
    w.nation_mut(id).growth_last = 0.02;
    population::reconcile_ownership(&mut w);
    w.nation_mut(id).political_capital = 100.0;
    spheres_sim::apply_command(
        &mut w,
        &Command::SetPopulationPolicy {
            nation: id,
            policy: Policy::BackToWork,
        },
    )
    .unwrap();
    population::tick(&mut w);
    let p = &w.population_system.provinces[&district];
    let first = p
        .courses
        .iter()
        .find(|c| c.from == 1 && c.to == 2)
        .expect("basic adults need a real secondary catch-up route")
        .clone();
    assert_eq!(first.required_days, 730.0);
    assert_eq!(first.funded_days, 0.0);
    assert!(population::snapshot(&w, id)
        .unwrap()
        .courses
        .iter()
        .any(|c| c.name == "Secondary catch-up" && c.days_remaining == 730));
    let mut unpaid = w.clone();
    let mut budget = unpaid.nation(id).budget_for(unpaid.year);
    budget.allocations[spheres_sim::world::BUDGET_EDUCATION] = 0.0;
    unpaid.nation_mut(id).annual_budget = Some(budget);
    for day in 1..=730 {
        spheres_sim::clock::advance_date(&mut w);
        spheres_sim::clock::advance_date(&mut unpaid);
        population::tick(&mut w);
        population::tick(&mut unpaid);
        if day == 729 {
            assert!(
                w.population_system.provinces[&district]
                    .courses
                    .iter()
                    .any(|c| c.started_day == first.started_day && c.from == 1 && c.to == 2),
                "secondary education cannot complete before its funded deadline"
            );
        }
    }
    let p = &w.population_system.provinces[&district];
    assert!(!p
        .courses
        .iter()
        .any(|c| c.started_day == first.started_day && c.from == 1 && c.to == 2));
    assert!(p.working[2] > unpaid.population_system.provinces[&district].working[2] + 0.002);
    // After catching up, the SAME ordinary intake can offer technical study.
    // No command grants a qualification and no special case bypasses schooling.
    let count_before = p
        .courses
        .iter()
        .filter(|c| c.to == 3)
        .map(|c| c.people_m)
        .sum::<f64>();
    w.nation_mut(id).political_capital = 100.0;
    spheres_sim::apply_command(
        &mut w,
        &Command::SetPopulationPolicy {
            nation: id,
            policy: Policy::TradeSchools,
        },
    )
    .unwrap();
    for _ in 0..32 {
        spheres_sim::clock::advance_date(&mut w);
        spheres_sim::clock::advance_date(&mut unpaid);
        population::tick(&mut w);
        population::tick(&mut unpaid);
    }
    let p = &w.population_system.provinces[&district];
    assert!(
        p.courses
            .iter()
            .filter(|c| c.from == 2 && c.to == 3)
            .map(|c| c.people_m)
            .sum::<f64>()
            > count_before,
        "secondary catch-up must open the existing technical education route"
    );
    assert!(
        (p.population_m() - unpaid.population_system.provinces[&district].population_m()).abs()
            < 1e-9,
        "qualifications move people between cohorts rather than create new people"
    );
}

#[test]
#[ignore = "Diagnostic timing only: compare 32 Economic Competition days including the first monthly review"]
fn economic_ai_population_timing() {
    for population_enabled in [false, true] {
        let mut w = world_1990(GameRules {
            seed: 1990,
            daily_simulation: true,
            ..GameRules::default()
        });
        w.player = Some(N::USA);
        spheres_sim::starting_industry::enable_new_world(&mut w).unwrap();
        spheres_sim::starting_industry::enrich_new_world(&mut w).unwrap();
        w.rules.resource_market = true;
        w.rules.ideology_blocs = true;
        spheres_sim::government::ensure_all(&mut w);
        w.rules.logistics_routes = true;
        w.rules.physical_logistics = true;
        w.rules.military_operations = true;
        w.rules.production_system = true;
        w.rules.industry_rebuild = true;
        w.rules.manufacturing_system = true;
        spheres_sim::province_economy::enable(&mut w);
        spheres_sim::companies::enable(&mut w);
        if population_enabled {
            population::enable(&mut w);
        }
        spheres_sim::resources::warm(&mut w);
        let enabling = std::time::Instant::now();
        spheres_sim::apply_command(
            &mut w,
            &Command::EnableEconomicCompetition { nation: N::USA },
        )
        .unwrap();
        println!(
            "economic-ai population={population_enabled} enable_ms={:.2}",
            enabling.elapsed().as_secs_f64() * 1000.0
        );
        for day in 1..=32 {
            let started = std::time::Instant::now();
            tick_day(&mut w, &[]);
            println!(
                "economic-ai population={population_enabled} day={day} tick_ms={:.2} projects={} mine_projects={}",
                started.elapsed().as_secs_f64() * 1000.0,
                w.production.projects.len(),
                w.resources.mine_projects.len()
            );
            if population_enabled {
                accounting(&w);
            }
        }
    }
}
