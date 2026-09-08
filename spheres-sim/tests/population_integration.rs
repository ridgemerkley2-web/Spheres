//! Causal population contracts across a full education delay and a border
//! change. These are accounting invariants and controlled interventions, not
//! historical calibration statistics; no sampled outcome bars are claimed.
use spheres_sim::{clock, init::world_1990, population::{self, Course},
    world::{GameRules, NationId, WorldState, BUDGET_EDUCATION}};

fn world() -> WorldState {
    let mut w = world_1990(GameRules {
        daily_simulation: true, ai_aggression: 0.0, ..GameRules::default()
    });
    w.player = Some(NationId::USA);
    population::enable(&mut w);
    w
}

/// Keep one existing province and one government so a full eighteen-month
/// course can be exercised without unrelated world events or migration.
fn training_world() -> (WorldState, String) {
    let mut w = world();
    let id = NationId::USA;
    let district = w.population_system.provinces.keys()
        .find(|d| w.districts.get(*d) == Some(&id)).unwrap().clone();
    w.districts.retain(|d, _| d == &district);
    for n in &mut w.nations { n.alive = n.id == id; }
    w.population_system.provinces.retain(|d, _| d == &district);
    w.population_system.unallocated.clear();
    w.population_system.nations.retain(|n, _| *n == id);
    w.production.provinces.clear();
    w.production.projects.clear();
    w.production.industry.sites.clear();
    w.production.industry.modules.clear();
    let p = w.population_system.provinces.get_mut(&district).unwrap();
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
    p.filled = [[0.0; 3]; 8];
    p.filled[7][2] = 0.04; // Teachers are present before the first lesson.
    p.school_coverage = 1.0;
    p.school_reference = 1.0;
    p.courses = vec![Course { from: 2, to: 3, people_m: 0.02,
        started_day: -4000, required_days: 548.0, funded_days: 0.0 }];
    w.nation_mut(id).population = 1.0;
    w.nation_mut(id).mil_strength = 0.0;
    w.nation_mut(id).growth_last = 0.02;
    w.population_system.nations.get_mut(&id).unwrap().opening_population_m = 1.0;
    (w, district)
}

fn maintain_education(w: &mut WorldState, funded: bool) {
    let id = NationId::USA;
    let mut budget = w.nation(id).budget_for(w.year);
    budget.allocations[BUDGET_EDUCATION] = if funded {
        w.population_system.nations[&id].opening_education_share
    } else { 0.0 };
    w.nation_mut(id).annual_budget = Some(budget);
}

#[test]
fn opening_sources_are_respected_and_income_does_not_invent_qualifications() {
    let w = world();
    let sources: serde_json::Value = serde_json::from_str(
        include_str!("../data/population_1990.json")).unwrap();
    for id in [NationId::USA, NationId::India, NationId::USSR] {
        let row = population::snapshot(&w, id).unwrap();
        let source = &sources["countries"][format!("{id:?}")];
        for (actual, key) in [
            (row.children_m, "population_age_0_14"),
            (row.working_age_m, "population_age_15_64"),
            (row.retirees_m, "population_age_65_plus"),
        ] {
            assert!((actual / row.population_m - source[key]["value"].as_f64().unwrap()).abs() < 1e-12,
                "{id:?} must retain the sourced opening age composition");
        }
    }
    assert!(sources["countries"]["Taiwan"]["population_age_15_64"].is_null());
    assert!(population::snapshot(&w, NationId::Taiwan).unwrap().notes.iter()
        .any(|n| n.contains("population_age_15_64")),
        "missing historical observations must be disclosed as modeled starting values");
    assert!(sources["countries"].as_object().unwrap().values()
        .all(|row| row["class_shares"].is_null()));
    let mut poorer = world_1990(GameRules {
        daily_simulation: true, ..GameRules::default()
    });
    poorer.nation_mut(NationId::USA).gdp *= 0.01;
    population::enable(&mut poorer);
    let normal = population::snapshot(&w, NationId::USA).unwrap();
    let changed = population::snapshot(&poorer, NationId::USA).unwrap();
    for (a, b) in normal.education.iter().zip(&changed.education) {
        assert_eq!(a.key, b.key);
        assert!((a.share - b.share).abs() < 1e-12,
            "GDP must not be used as a historical education census");
    }
}

#[test]
fn technical_training_fills_jobs_after_its_calendar_delay_and_needs_funding() {
    let (mut funded, district) = training_world();
    let mut unfunded = funded.clone();
    for day in 0..548 {
        maintain_education(&mut funded, true);
        maintain_education(&mut unfunded, false);
        population::tick(&mut funded);
        population::tick(&mut unfunded);
        if day == 0 || day == 546 {
            let course = funded.population_system.provinces[&district].courses.iter()
                .find(|c| c.started_day == -4000).expect("degree awarded before the paid calendar delay");
            assert!(course.funded_days < course.required_days);
        }
        clock::advance_date(&mut funded);
        clock::advance_date(&mut unfunded);
    }
    let trained = &funded.population_system.provinces[&district];
    let idle = &unfunded.population_system.provinces[&district];
    assert!(!trained.courses.iter().any(|c| c.started_day == -4000));
    let waiting = idle.courses.iter().find(|c| c.started_day == -4000).unwrap();
    assert_eq!(waiting.funded_days, 0.0);
    assert!(trained.working[3] > idle.working[3] + 0.015,
        "funded technical graduates must join the technical qualification stock");
    assert!(trained.filled[2][1] > idle.filled[2][1],
        "technical graduates must help fill the existing technician shortage");
    assert!((trained.population_m() - idle.population_m()).abs() < 1e-9,
        "education must move qualifications without creating people");
}

#[test]
fn transfer_keeps_resident_qualifications_courses_and_household_history() {
    let mut w = world();
    let from = NationId::USA;
    let to = NationId::Canada;
    let district = w.population_system.provinces.keys()
        .find(|d| w.districts.get(*d) == Some(&from)).unwrap().clone();
    let before = w.population_system.provinces[&district].clone();
    let original = population::snapshot(&w, from).unwrap().population_m
        + population::snapshot(&w, to).unwrap().population_m;
    let to_before = population::snapshot(&w, to).unwrap().population_m;
    spheres_sim::districts::transfer_district(&mut w, from, to, &district).unwrap();
    let transferred = population::snapshot(&w, to).unwrap();
    let remaining = population::snapshot(&w, from).unwrap();
    let after_transfer = &w.population_system.provinces[&district];
    assert_eq!(after_transfer.working, before.working);
    assert_eq!(after_transfer.children, before.children);
    assert_eq!(after_transfer.retirees_m, before.retirees_m);
    assert_eq!(after_transfer.courses, before.courses);
    assert_eq!(after_transfer.class_shares, before.class_shares);
    assert_eq!(after_transfer.class_living, before.class_living);
    assert_eq!(after_transfer.class_risk_reference, before.class_risk_reference);
    assert_eq!(after_transfer.wealth_indices, before.wealth_indices,
        "a border transfer must never reseed resident characteristics");
    assert!((transferred.population_m - to_before - before.population_m()).abs() < 1e-9);
    assert!((transferred.population_m + remaining.population_m - original).abs() < 1e-9);
    let resumed = spheres_sim::load(&spheres_sim::save(&w)).unwrap();
    assert_eq!(resumed.population_system.provinces[&district], *after_transfer);
    population::tick(&mut w);
    let after = &w.population_system.provinces[&district];
    assert_eq!(after.last_owner, to);
    assert!((after.working[4] / after.working_age_m()
        - before.working[4] / before.working_age_m()).abs() < 0.002,
        "one day under a new government cannot replace resident education");
}

#[test]
fn competing_jobs_conserve_workers_across_different_country_sizes() {
    let mut w = world();
    let old_gdp: Vec<_> = w.nations.iter().map(|n| n.gdp).collect();
    let old_rng = w.rng.clone();
    for (d, p) in &mut w.population_system.provinces {
        if ![NationId::USA, NationId::China, NationId::Tonga].contains(&w.districts[d]) { continue; }
        // Simultaneous general and specialist shortages cannot reuse a worker.
        let adults = p.working_age_m();
        for sector in &mut p.jobs_reference { *sector = [adults; 3]; }
    }
    for _ in 0..35 {
        population::tick(&mut w);
        for p in w.population_system.provinces.values().chain(w.population_system.unallocated.values()) {
            let employed = p.employed_m();
            assert!(employed >= 0.0 && employed <= p.labor_force_m + 1e-10);
            assert!((p.employed_by_skill.iter().sum::<f64>() - employed).abs() < 1e-10);
            assert!((p.unemployment_by_skill.iter().sum::<f64>() + employed - p.labor_force_m).abs() < 1e-10);
            assert!(p.students_m() <= p.working_age_m() + 1e-10);
            assert!(p.working.iter().all(|x| x.is_finite() && *x >= 0.0));
        }
        clock::advance_date(&mut w);
    }
    assert_eq!(w.nations.iter().map(|n| n.gdp).collect::<Vec<_>>(), old_gdp,
        "population must publish labor outcomes without posting a second GDP receipt");
    assert_eq!(w.rng, old_rng, "population must not consume or reseed the shared RNG");
}

#[test]
fn a_routine_job_loss_hurts_worker_households_and_class_hardship_reaches_politics() {
    let (mut steady, district) = training_world();
    steady.population_system.provinces.get_mut(&district).unwrap().courses.clear();
    let mut shock = steady.clone();
    let p = shock.population_system.provinces.get_mut(&district).unwrap();
    p.jobs_reference[2][0] = 0.10;
    p.jobs[2][0] = 0.10;
    for _ in 0..90 {
        population::tick(&mut steady);
        population::tick(&mut shock);
        clock::advance_date(&mut steady);
        clock::advance_date(&mut shock);
    }
    let view = population::snapshot(&shock, NationId::USA).unwrap();
    let workers = view.classes.iter().find(|c| c.key == "routine_workers").unwrap();
    let professionals = view.classes.iter().find(|c| c.key == "professionals").unwrap();
    assert!(workers.living_standards < professionals.living_standards - 1.0,
        "a routine job shock must produce different household outcomes, not six copies of a national gauge");
    assert!(population::hardship(&shock, NationId::USA) > population::hardship(&steady, NationId::USA));
    // Isolate the class channel while holding aggregate GDP and unemployment
    // fixed. Merely rendering different class bars must not satisfy this test.
    let mut class_only = steady.clone();
    class_only.population_system.provinces.get_mut(&district).unwrap().class_living[1] -= 40.0;
    assert_eq!(population::unemployment(&class_only, NationId::USA), population::unemployment(&steady, NationId::USA));
    assert!(population::hardship(&class_only, NationId::USA) > population::hardship(&steady, NationId::USA),
        "class-specific hardship must reach the political-pressure interface");
}
