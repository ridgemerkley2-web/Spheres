//! S02 controlled accounting fixtures, not historical-frequency calibration.
//! No browser, server, campaign file or long world warmup is required.
use spheres_sim::{
    clock, districts,
    init::world_1990,
    load,
    population::{self, Course, Policy, PopulationOutcomes},
    production::{Priority, Project, ProjectKind, ProjectStatus},
    save,
    world::{GameRules, NationId as N, WorldState, BUDGET_EDUCATION},
};

fn daily() -> WorldState {
    world_1990(GameRules {
        daily_simulation: true,
        ai_aggression: 0.0,
        ..GameRules::default()
    })
}
fn close(a: f64, b: f64) {
    assert!(
        (a - b).abs() <= 1e-9 * a.abs().max(b.abs()).max(1.0),
        "{a} != {b}"
    );
}
fn next_population_day(w: &mut WorldState) {
    clock::advance_date(w);
    population::tick(w);
}
fn district(w: &WorldState, id: N) -> String {
    w.districts
        .iter()
        .find(|(_, owner)| **owner == id)
        .unwrap()
        .0
        .clone()
}

#[test]
fn disabled_legacy_is_inert_and_orphaned_outcomes_are_rejected() {
    let mut w = world_1990(GameRules::default());
    let original = save(&w);
    population::validate(&w).unwrap();
    population::tick(&mut w);
    population::ai_tick(&mut w);
    assert!(population::snapshot(&w, N::France).is_none());
    assert!(
        population::enable(&mut w).is_err(),
        "daily adoption is explicit"
    );
    assert_eq!(save(&w), original);
    assert_eq!(save(&load(&original).unwrap()), original);
    w.nation_mut(N::France).population_outcomes = Some(PopulationOutcomes {
        labor_growth: 0.0,
        unemployment: 0.0,
    });
    assert!(population::validate(&w).is_err());
    assert!(
        load(&save(&w)).is_err(),
        "orphaned outcomes must never suppress legacy demographic growth"
    );
}

#[test]
fn france_japan_india_enroll_current_midcampaign_residents_without_reseeding_money() {
    for (id, multiplier) in [(N::France, 1.17), (N::Japan, 0.91), (N::India, 1.42)] {
        let mut w = daily();
        w.year = 2005;
        w.month = 6;
        w.day = 15;
        w.player = Some(id);
        w.nation_mut(id).population *= multiplier;
        w.nation_mut(id).gdp *= 1.37;
        w.nation_mut(id).treasury_bn = Some(12.345);
        w.nation_mut(id).debt_bn = Some(w.nation(id).gdp * w.nation(id).debt_gdp);
        let owned: Vec<_> = w
            .districts
            .iter()
            .filter(|(_, owner)| **owner == id)
            .map(|(d, _)| d.clone())
            .collect();
        for d in owned {
            if let Some(p) = w.district_population.get_mut(&d) {
                *p *= multiplier;
            }
        }
        let before: Vec<_> = w
            .nations
            .iter()
            .map(|n| {
                (
                    n.id,
                    n.population,
                    n.gdp,
                    n.treasury_bn,
                    n.debt_bn,
                    n.political_capital,
                )
            })
            .collect();
        let rng = w.rng.clone();
        population::enable(&mut w).unwrap();
        population::validate(&w).unwrap();
        for (id, pop, gdp, cash, debt, pc) in before {
            assert_eq!(w.nation(id).population, pop);
            assert_eq!(
                (
                    w.nation(id).gdp,
                    w.nation(id).treasury_bn,
                    w.nation(id).debt_bn,
                    w.nation(id).political_capital
                ),
                (gdp, cash, debt, pc)
            );
        }
        assert_eq!(w.rng, rng);
        let view = population::snapshot(&w, id).unwrap();
        close(view.population_m, w.nation(id).population);
        assert_eq!(view.annual_graduates_m, 0.0);
        assert!(view.courses.is_empty());
        assert!(view
            .notes
            .iter()
            .any(|note| note.contains("current residents")));
        assert!(w.population_system.provinces.values().all(|p| p
            .children
            .keys()
            .all(|birth| (w.year - 14..=w.year).contains(birth))));
        let enrolled = save(&w);
        population::enable(&mut w).unwrap();
        population::tick(&mut w);
        for n in &w.nations {
            let _ = population::snapshot(&w, n.id);
        }
        assert_eq!(
            save(&w),
            enrolled,
            "adoption, repeat dates and read-only views are pure"
        );
        let mut resumed = load(&enrolled).unwrap();
        next_population_day(&mut w);
        next_population_day(&mut resumed);
        assert_eq!(save(&w), save(&resumed));
        population::validate(&w).unwrap();
    }
}

#[test]
fn inconsistent_enrollment_is_atomic_and_loaded_books_fail_closed() {
    let mut invalid = daily();
    invalid.nation_mut(N::France).population = -1.0;
    let before = save(&invalid);
    assert!(population::enable(&mut invalid).is_err());
    assert_eq!(save(&invalid), before);
    let mut good = daily();
    population::enable(&mut good).unwrap();
    let d = district(&good, N::France);
    let mut bad = good.clone();
    bad.nation_mut(N::France).population_outcomes = None;
    assert!(population::validate(&bad).is_err());
    assert!(load(&save(&bad)).is_err());
    let mut bad = good.clone();
    bad.population_system.last_day = Some(clock::absolute_day(&bad) + 1);
    assert!(population::validate(&bad).is_err());
    let mut bad = good.clone();
    bad.population_system.provinces.get_mut(&d).unwrap().working[2] = f64::INFINITY;
    assert!(population::validate(&bad).is_err());
    let mut bad = good.clone();
    bad.population_system
        .provinces
        .get_mut(&d)
        .unwrap()
        .courses
        .push(Course {
            from: 9,
            to: 10,
            people_m: 0.1,
            started_day: 0,
            required_days: 548.0,
            funded_days: 0.0,
        });
    assert!(
        population::validate(&bad).is_err(),
        "malformed skills cannot become a tick panic"
    );
    let mut bad = good.clone();
    bad.population_system
        .nations
        .get_mut(&N::France)
        .unwrap()
        .province_ids
        .clear();
    assert!(population::validate(&bad).is_err());
}

#[test]
fn legacy_peace_resident_estimates_reconcile_only_on_initial_enrollment() {
    let mut w = daily();
    w.year = 2000;
    w.month = 2;
    w.day = 1;
    // Reproduce the legacy owner's real mismatch: peace changes the national
    // population by 12%, but transfers one whole, geographically ranked
    // province. The archived year-10 profile has KW-JA held by Iraq and Kuwait
    // at 2.7777589574380315m nationally versus 2.9105141907285716m mapped.
    let growth = 1.5031163189599712;
    districts::grow_populations_compounded(&mut w, &[(N::Kuwait, growth)], &[]);
    w.nation_mut(N::Kuwait).population *= growth;
    let ceded = w.nation(N::Kuwait).population * 0.12;
    w.nation_mut(N::Kuwait).population -= ceded;
    w.nation_mut(N::Iraq).population += ceded;
    let transferred = districts::cede_share_preferring(
        &mut w,
        N::Iraq,
        N::Kuwait,
        0.12,
        &std::collections::BTreeSet::from(["KW-JA".to_string()]),
    );
    assert_eq!(transferred, vec!["KW-JA".to_string()]);
    w.nation_mut(N::Kuwait).gdp = 27.319304594919576;
    w.nation_mut(N::Kuwait).treasury_bn = Some(7.25);
    w.nation_mut(N::Kuwait).debt_bn = Some(12.75);
    w.nation_mut(N::Kuwait).debt_gdp = 12.75 / w.nation(N::Kuwait).gdp;
    // An under-mapped country has real unlocated residents; do not scale its
    // known provinces upward or erase that separate residual to fix Kuwait.
    w.nation_mut(N::France).population += 2.0;
    let mapped: Vec<_> = w.districts.iter().filter(|(_, id)| **id == N::Kuwait)
        .map(|(d, _)| (d.clone(), districts::population_of(&w, d).unwrap())).collect();
    let mapped_total: f64 = mapped.iter().map(|(_, p)| p).sum();
    let national = w.nation(N::Kuwait).population;
    assert!(mapped_total > national + 0.1, "fixture must expose the real legacy overshoot");
    close(national, 2.7777589574380315);
    close(mapped_total, 2.9105141907285716);
    let transferred_people = districts::population_of(&w, "KW-JA").unwrap();
    let before = w.clone();
    let legacy = save(&w);
    let mut resumed = load(&legacy).unwrap();
    assert!(!resumed.population_system.enabled);
    assert_eq!(save(&resumed), legacy, "loading alone cannot repair estimates or enroll");
    population::enable(&mut w).unwrap();
    population::enable(&mut resumed).unwrap();
    population::validate(&w).unwrap();
    assert_eq!(save(&w), save(&resumed));
    for (d, amount) in mapped {
        close(districts::population_of(&w, &d).unwrap(), amount * national / mapped_total);
        close(w.population_system.provinces[&d].population_m(), amount * national / mapped_total);
    }
    close(w.population_system.unallocated[&N::France].population_m(), 2.0);
    assert_eq!(districts::population_of(&w, "KW-JA").unwrap(), transferred_people);
    for (d, owner) in &w.districts {
        if *owner != N::Kuwait {
            assert_eq!(w.district_population.get(d), before.district_population.get(d));
        }
    }
    let snapshot = population::snapshot(&w, N::Kuwait).unwrap();
    close(snapshot.population_m, national);
    assert!(snapshot.courses.is_empty());
    assert_eq!(snapshot.annual_graduates_m, 0.0);
    assert!(snapshot.notes.iter().any(|note| note.contains("legacy province population estimates were proportionally reconciled")));
    assert!(!population::snapshot(&w, N::France).unwrap().notes.iter().any(|note| note.contains("proportionally reconciled")));
    // Whitelist the only adoption changes. In particular this compares every
    // national population/GDP/cash/debt record, province ownership, arsenal,
    // other property and RNG instead of merely checking an aggregate total.
    let mut outside_population = w.clone();
    outside_population.population_system = before.population_system.clone();
    outside_population.district_population = before.district_population.clone();
    for n in &mut outside_population.nations {
        n.population_outcomes = before.nation(n.id).population_outcomes.clone();
    }
    assert_eq!(save(&outside_population), legacy);
    let once = save(&w);
    population::enable(&mut w).unwrap();
    assert_eq!(save(&w), once, "enrollment adjustment and note are one-time only");
    resumed = load(&once).unwrap();
    next_population_day(&mut w);
    next_population_day(&mut resumed);
    population::validate(&w).unwrap();
    assert_eq!(save(&w), save(&resumed));

    // Once cohorts own residents, either side of a mismatched book is an error;
    // enable/load may not use initial migration to hide corruption.
    for alter_mapping in [false, true] {
        let mut corrupt = w.clone();
        if alter_mapping {
            *corrupt.district_population.get_mut("KW-KU").unwrap() *= 1.1;
        } else {
            corrupt.nation_mut(N::Kuwait).population *= 0.9;
        }
        let before = save(&corrupt);
        assert!(population::enable(&mut corrupt).is_err());
        assert_eq!(save(&corrupt), before);
        assert!(load(&before).is_err());
    }
}

#[test]
fn initial_population_reconciliation_refuses_nonfinite_or_negative_resident_estimates_atomically() {
    for amount in [-1.0, f64::INFINITY, f64::NAN] {
        let mut w = daily();
        w.district_population.insert("KW-KU".into(), amount);
        let before = save(&w);
        assert!(population::enable(&mut w).is_err());
        assert_eq!(save(&w), before);
    }
    let mut w = daily();
    w.district_population.insert("KW-KU".into(), f64::MAX);
    w.district_population.insert("KW-HA".into(), f64::MAX);
    let before = save(&w);
    assert!(population::enable(&mut w).is_err(), "a nonfinite mapped total is not a scaling estimate");
    assert_eq!(save(&w), before);
}

#[test]
#[ignore = "requires unchanged SPHERES_POPULATION_LEGACY_CHECKPOINT input; no campaign warmup"]
fn archived_legacy_checkpoint_adoption_preserves_accounts_and_replays() {
    let path = std::env::var_os("SPHERES_POPULATION_LEGACY_CHECKPOINT")
        .expect("provide the original archived legacy checkpoint path");
    let input = std::fs::read_to_string(&path).unwrap();
    // The performance archives use the browser presentation envelope. Follow
    // storage::decode's simulation boundary without editing the input archive;
    // this test does not claim to validate its presentation history or log.
    let envelope: serde_json::Value = serde_json::from_str(&input).unwrap();
    let mut direct = if envelope["format"] == "spheres-campaign" {
        assert_eq!(envelope["version"], 1);
        load(&serde_json::to_string(&envelope["world"]).unwrap()).unwrap()
    } else {
        load(&input).unwrap()
    };
    assert!(!direct.population_system.enabled);
    let nation = direct
        .player
        .expect("archived checkpoint has an actual player");
    let before = direct.clone();
    let command = spheres_sim::Command::EnableConnectedEconomy { nation };
    let input_hash = spheres_sim::state_hash(&direct);
    spheres_sim::apply_command(&mut direct, &command).unwrap();
    spheres_sim::connected_economy::validate(&direct).unwrap();
    assert_eq!(direct.districts, before.districts);
    assert_eq!(
        serde_json::to_value(&direct.production).unwrap(),
        serde_json::to_value(&before.production).unwrap()
    );
    assert_eq!(
        serde_json::to_value(&direct.companies).unwrap(),
        serde_json::to_value(&before.companies).unwrap()
    );
    assert_eq!(
        serde_json::to_value(&direct.manufacturing).unwrap(),
        serde_json::to_value(&before.manufacturing).unwrap()
    );
    assert_eq!(direct.rng, before.rng);
    for prior in &before.nations {
        let after = direct.nation(prior.id);
        assert_eq!(
            (after.population, after.gdp, after.debt_gdp),
            (prior.population, prior.gdp, prior.debt_gdp)
        );
        if prior.on_the_books() {
            assert_eq!(
                (after.treasury_bn, after.debt_bn),
                (prior.treasury_bn, prior.debt_bn)
            );
        }
        assert_eq!(
            serde_json::to_value(&after.arsenal).unwrap(),
            serde_json::to_value(&prior.arsenal).unwrap()
        );
        assert_eq!(
            serde_json::to_value(&after.equipment).unwrap(),
            serde_json::to_value(&prior.equipment).unwrap()
        );
        if prior.alive {
            let mapped: f64 = before
                .districts
                .iter()
                .filter(|(_, owner)| **owner == prior.id)
                .map(|(d, _)| districts::population_of(&before, d).unwrap_or(0.0))
                .sum();
            let residual = prior.population - mapped;
            if residual > 0.0 {
                let preserved = direct.population_system.unallocated[&prior.id].population_m();
                assert!(
                    preserved > 0.0,
                    "{} positive residual must not disappear",
                    prior.id.name()
                );
                assert!(
                    (preserved - residual).abs() <= residual * 1e-12,
                    "{} residual changed: {residual:e} -> {preserved:e}",
                    prior.id.name()
                );
            }
        }
    }
    let enrolled = save(&direct);
    spheres_sim::apply_command(&mut direct, &command).unwrap();
    assert_eq!(save(&direct), enrolled);
    let mut resumed = load(&enrolled).unwrap();
    assert_eq!(save(&resumed), enrolled);
    for _ in 0..2 {
        spheres_sim::tick_day(&mut direct, &[]);
        spheres_sim::tick_day(&mut resumed, &[]);
        spheres_sim::connected_economy::validate(&direct).unwrap();
        spheres_sim::connected_economy::validate(&resumed).unwrap();
        assert_eq!(save(&direct), save(&resumed));
        resumed = load(&save(&resumed)).unwrap();
    }
    eprintln!("Archived population adoption: input={}; date={}; canonical_input_hash={input_hash:016x}; final_hash={:016x}",
        std::path::Path::new(&path).display(), before.date_str(), spheres_sim::state_hash(&resumed));
}

fn training_world(id: N) -> (WorldState, String) {
    let mut w = daily();
    let d = district(&w, id);
    w.year = 2005;
    w.month = 12;
    w.day = 30;
    w.player = Some(id);
    for n in &mut w.nations {
        n.alive = n.id == id;
    }
    w.districts.retain(|key, _| *key == d);
    w.district_population.clear();
    w.district_population.insert(d.clone(), 1.0);
    w.nation_mut(id).population = 1.0;
    w.nation_mut(id).mil_strength = 0.0;
    w.nation_mut(id).growth_last = 0.02;
    population::enable(&mut w).unwrap();
    let started = clock::absolute_day(&w);
    let p = w.population_system.provinces.get_mut(&d).unwrap();
    p.children.clear();
    p.working = [0.0, 0.0, 0.60, 0.20, 0.20];
    p.retirees_m = 0.0;
    p.birth_rate_per_adult = 0.0;
    p.reference_population_m = 1.0;
    p.military_m = 0.0;
    p.participation_reference = 0.80;
    p.participation = 0.80;
    p.jobs_reference = [[0.0; 3]; 8];
    p.jobs_reference[2] = [0.40, 0.30, 0.0];
    p.jobs_reference[7] = [0.0, 0.0, 0.04];
    p.jobs = p.jobs_reference;
    p.project_jobs = [[0.0; 3]; 8];
    p.school_coverage = 1.0;
    p.school_reference = 1.0;
    p.courses = vec![Course {
        from: 2,
        to: 3,
        people_m: 0.02,
        started_day: started,
        required_days: 548.0,
        funded_days: 0.0,
    }];
    population::reconcile_ownership(&mut w);
    population::validate(&w).unwrap();
    (w, d)
}
fn fund_education(w: &mut WorldState, id: N, funded: bool) {
    let mut budget = w.nation(id).budget_for(w.year);
    budget.allocations[BUDGET_EDUCATION] = if funded {
        w.population_system.nations[&id].opening_education_share
    } else {
        0.0
    };
    w.nation_mut(id).annual_budget = Some(budget);
}

#[test]
fn training_requires_paid_calendar_days_and_keeps_qualifications_on_save_resume() {
    let id = N::France;
    let (mut funded, d) = training_world(id);
    let mut unfunded = funded.clone();
    let start = clock::absolute_day(&funded);
    let money = (funded.nation(id).gdp, funded.nation(id).treasury_bn);
    for elapsed in 1..=548 {
        clock::advance_date(&mut funded);
        clock::advance_date(&mut unfunded);
        fund_education(&mut funded, id, true);
        fund_education(&mut unfunded, id, false);
        population::tick(&mut funded);
        population::tick(&mut unfunded);
        if elapsed == 1 || elapsed == 547 {
            let c = funded.population_system.provinces[&d]
                .courses
                .iter()
                .find(|c| c.started_day == start)
                .unwrap();
            close(c.funded_days, elapsed as f64);
            assert!(c.funded_days < c.required_days, "no instant qualifications");
        }
        if elapsed == 2 || elapsed == 300 {
            let snapshot = save(&funded);
            funded = load(&snapshot).unwrap();
            population::tick(&mut funded);
            assert_eq!(
                save(&funded),
                snapshot,
                "same-date resume must not age or train twice"
            );
        }
    }
    let qualified = &funded.population_system.provinces[&d];
    let waiting = &unfunded.population_system.provinces[&d];
    assert!(!qualified.courses.iter().any(|c| c.started_day == start));
    assert_eq!(
        waiting
            .courses
            .iter()
            .find(|c| c.started_day == start)
            .unwrap()
            .funded_days,
        0.0
    );
    assert!(qualified.working[3] > waiting.working[3] + 0.015);
    assert!(qualified.filled[2][1] > waiting.filled[2][1]);
    close(qualified.population_m(), waiting.population_m());
    assert_eq!(
        (funded.nation(id).gdp, funded.nation(id).treasury_bn),
        money
    );
    population::validate(&funded).unwrap();
    population::validate(&unfunded).unwrap();
}

#[test]
fn japan_and_india_training_cross_year_save_and_policy_dates_without_early_award() {
    for id in [N::Japan, N::India] {
        let (mut w, d) = training_world(id);
        let start = clock::absolute_day(&w);
        w.nation_mut(id).political_capital = 100.0;
        population::apply_policy(&mut w, id, Policy::TradeSchools).unwrap();
        let policy_day = w.population_system.nations[&id].last_policy_day;
        for _ in 0..3 {
            next_population_day(&mut w);
        }
        assert_eq!(w.year, 2006);
        let resumed = load(&save(&w)).unwrap();
        assert_eq!(
            resumed.population_system.nations[&id].last_policy_day,
            policy_day
        );
        let course = resumed.population_system.provinces[&d]
            .courses
            .iter()
            .find(|c| c.started_day == start)
            .unwrap();
        assert!(course.funded_days <= 3.0 && course.funded_days < course.required_days);
        assert!(!population::policy_quote(&resumed, id, Policy::Universities).available);
        population::validate(&resumed).unwrap();
    }
}

#[test]
fn funding_only_construction_does_not_fabricate_domestic_workforce_demand() {
    let (mut idle, d) = training_world(N::India);
    let mut queued = idle.clone();
    queued.production.projects.push(Project {
        id: 1,
        nation: N::India,
        district: d.clone(),
        kind: ProjectKind::Infrastructure,
        priority: Priority::Normal,
        status: ProjectStatus::Building,
        reason: None,
        progress_days: 0.0,
        total_days: 180,
        resources_used: [0.0; 12],
        capacity_micros: None,
        started_day: Some(clock::absolute_day(&idle)),
    });
    next_population_day(&mut idle);
    next_population_day(&mut queued);
    assert_eq!(
        idle.population_system, queued.population_system,
        "a queue is neither a domestic crew requirement nor paid economic output"
    );
}

#[test]
fn mapped_transfer_preserves_residents_qualifications_and_course_progress() {
    let mut w = daily();
    population::enable(&mut w).unwrap();
    let from = N::France;
    let to = N::Japan;
    let d = district(&w, from);
    let before = w.population_system.provinces[&d].clone();
    let total = w.nation(from).population + w.nation(to).population;
    let cash = (w.nation(from).treasury_bn, w.nation(to).treasury_bn);
    districts::transfer_district(&mut w, from, to, &d).unwrap();
    let after = &w.population_system.provinces[&d];
    assert_eq!(after.working, before.working);
    assert_eq!(after.children, before.children);
    assert_eq!(after.courses, before.courses);
    assert_eq!(after.wealth_indices, before.wealth_indices);
    assert_eq!(after.last_owner, to);
    close(w.nation(from).population + w.nation(to).population, total);
    assert_eq!((w.nation(from).treasury_bn, w.nation(to).treasury_bn), cash);
    population::validate(&w).unwrap();
}

#[test]
fn unallocated_successor_transfer_preserves_distinct_training_progress_and_people() {
    let mut w = daily();
    // Explicit technical fixture: these two nations have no mapped residents.
    w.districts
        .retain(|_, owner| *owner != N::France && *owner != N::Japan);
    population::enable(&mut w).unwrap();
    let day = clock::absolute_day(&w);
    for (id, progress) in [(N::France, 10.0), (N::Japan, 20.0)] {
        let p = w.population_system.unallocated.get_mut(&id).unwrap();
        p.courses.push(Course {
            from: 2,
            to: 3,
            people_m: 0.001,
            started_day: day - 30,
            required_days: 548.0,
            funded_days: progress,
        });
    }
    population::reconcile_ownership(&mut w);
    let total = w.nation(N::France).population + w.nation(N::Japan).population;
    population::transfer_unallocated(&mut w, N::France, N::Japan);
    w.nation_mut(N::France).alive = false;
    population::reconcile_ownership(&mut w);
    assert!(!w.population_system.unallocated.contains_key(&N::France));
    close(w.nation(N::Japan).population, total);
    let p = &w.population_system.unallocated[&N::Japan];
    assert_eq!(
        p.courses.len(),
        2,
        "merging different funded progress must not delay or accelerate either group"
    );
    assert!(p.courses.iter().any(|c| c.funded_days == 10.0));
    assert!(p.courses.iter().any(|c| c.funded_days == 20.0));
    assert!(w.nation(N::France).population_outcomes.is_none());
    population::validate(&w).unwrap();
    let saved = save(&w);
    assert_eq!(save(&load(&saved).unwrap()), saved);
}

#[test]
fn casualties_remove_only_exposed_adults_and_ignore_nonfinite_loss() {
    let mut w = daily();
    population::enable(&mut w).unwrap();
    let id = N::France;
    let before = population::snapshot(&w, id).unwrap();
    let gdp = w.nation(id).gdp;
    population::record_casualties(&mut w, id, 0.25);
    let after = population::snapshot(&w, id).unwrap();
    close(
        before.population_m - after.population_m,
        before.military_m * 0.25 * 0.20,
    );
    assert_eq!(before.children_m, after.children_m);
    assert_eq!(before.retirees_m, after.retirees_m);
    assert_eq!(w.nation(id).gdp, gdp);
    let valid = save(&w);
    population::record_casualties(&mut w, id, f64::NAN);
    assert_eq!(save(&w), valid);
    population::validate(&w).unwrap();
}

#[test]
fn actual_daily_pipeline_keeps_districts_nations_and_outcomes_coherent() {
    for id in [N::France, N::Japan, N::India] {
        let mut w = daily();
        w.player = Some(id);
        population::enable(&mut w).unwrap();
        spheres_sim::tick_day(&mut w, &[]);
        population::validate(&w).unwrap();
        let mut resumed = load(&save(&w)).unwrap();
        spheres_sim::tick_day(&mut w, &[]);
        spheres_sim::tick_day(&mut resumed, &[]);
        assert_eq!(save(&w), save(&resumed));
        population::validate(&w).unwrap();
    }
}

#[test]
fn authored_succession_moves_existing_cohorts_without_reseeding_successor_totals() {
    let mut w = daily();
    population::enable(&mut w).unwrap();
    let parent = N::USSR;
    let total: f64 = w
        .nations
        .iter()
        .filter(|n| n.alive)
        .map(|n| n.population)
        .sum();
    let cohorts: Vec<_> = w
        .population_system
        .provinces
        .iter()
        .filter(|(_, p)| p.last_owner == parent)
        .map(|(d, p)| (d.clone(), p.children.clone(), p.working, p.courses.clone()))
        .collect();
    let original = w.nation(parent).clone();
    w.nation_mut(parent).alive = false;
    for id in [N::Russia, N::Ukraine] {
        // Explicit caller fixture, as in politics succession: successor
        // government records exist before district inheritance is reconciled.
        let mut successor = original.clone();
        successor.id = id;
        successor.alive = true;
        successor.population = 1.0;
        successor.population_outcomes = None;
        if let Some(index) = w.nations.iter().position(|n| n.id == id) {
            w.nations[index] = successor;
        } else {
            w.nations.push(successor);
        }
    }
    w.reindex();
    districts::dissolve_to(&mut w, parent, &[N::Russia, N::Ukraine]);
    let after: f64 = w
        .nations
        .iter()
        .filter(|n| n.alive)
        .map(|n| n.population)
        .sum();
    close(after, total);
    for (d, children, working, courses) in cohorts {
        let p = &w.population_system.provinces[&d];
        assert_eq!(p.children, children);
        assert_eq!(p.working, working);
        assert_eq!(p.courses, courses);
        assert!(p.last_owner == N::Russia || p.last_owner == N::Ukraine);
    }
    assert!(w.nation(parent).population_outcomes.is_none());
    for id in [N::Russia, N::Ukraine] {
        close(
            w.population_system.nations[&id].opening_population_m,
            w.nation(id).population,
        );
        assert_eq!(w.nation(id).gdp, original.gdp);
        assert_eq!(w.nation(id).treasury_bn, original.treasury_bn);
        assert_eq!(w.nation(id).debt_bn, original.debt_bn);
    }
    population::validate(&w).unwrap();
    let once = save(&w);
    population::reconcile_ownership(&mut w);
    assert_eq!(
        save(&w),
        once,
        "reconciliation is not another settlement or enrollment"
    );
}
