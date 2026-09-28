use super::*;
use std::cell::Cell;

thread_local! {
    pub(super) static ORIGINAL: Cell<bool> = const { Cell::new(false) };
    pub(super) static SCANNED: Cell<u64> = const { Cell::new(0) };
    pub(super) static ORIGINAL_COMPARISONS: Cell<u64> = const { Cell::new(0) };
}

fn original<T>(run: impl FnOnce() -> T) -> T {
    struct Reset(bool);
    impl Drop for Reset {
        fn drop(&mut self) { ORIGINAL.with(|flag| flag.set(self.0)); }
    }
    let _reset = Reset(ORIGINAL.with(|flag| flag.replace(true)));
    run()
}

// Literal pre-index add_people: preserve both the course search and all
// surrounding population/class arithmetic as an independent byte oracle.
pub(super) fn add_people_original(to: &mut ProvincePopulation, from: &ProvincePopulation) {
    let old = to.population_m();
    let incoming = from.population_m();
    let weight = ratio(incoming, old + incoming);
    for (&year, &pop) in &from.children {
        *to.children.entry(year).or_default() += pop;
    }
    for i in 0..SKILLS {
        to.working[i] += from.working[i];
    }
    to.retirees_m += from.retirees_m;
    to.military_m += from.military_m;
    for course in &from.courses {
        if let Some(same) = to.courses.iter_mut().find(|x| {
            x.from == course.from
                && x.to == course.to
                && x.started_day == course.started_day
                && x.required_days.to_bits() == course.required_days.to_bits()
                && x.funded_days.to_bits() == course.funded_days.to_bits()
        }) {
            same.people_m += course.people_m;
        } else {
            to.courses.push(course.clone());
        }
    }
    for i in 0..6 {
        let class_weight = ratio(
            incoming * from.class_shares[i],
            old * to.class_shares[i] + incoming * from.class_shares[i],
        );
        to.class_shares[i] += (from.class_shares[i] - to.class_shares[i]) * weight;
        to.wealth_indices[i] += (from.wealth_indices[i] - to.wealth_indices[i]) * class_weight;
        to.class_living[i] += (from.class_living[i] - to.class_living[i]) * class_weight;
        to.class_risk_reference[i] +=
            (from.class_risk_reference[i] - to.class_risk_reference[i]) * class_weight;
    }
}

fn course(started_day: i32, people_m: f64, required_days: f64, funded_days: f64) -> Course {
    Course { from: 0, to: 1, people_m, started_day, required_days, funded_days }
}

fn assert_population(actual: &ProvincePopulation, expected: &ProvincePopulation) {
    assert_eq!(serde_json::to_vec(actual).unwrap(), serde_json::to_vec(expected).unwrap());
    // JSON cannot distinguish signed zero in every consumer, or retain NaN
    // payloads. Check every course field explicitly as well as the full row.
    let bits = |p: &ProvincePopulation| p.courses.iter().map(|c|
        (c.from, c.to, c.started_day, c.people_m.to_bits(),
         c.required_days.to_bits(), c.funded_days.to_bits())).collect::<Vec<_>>();
    assert_eq!(bits(actual), bits(expected));
}

#[test]
fn course_merge_keeps_first_duplicate_append_order_and_exact_key_bits() {
    let w = crate::init::world_1990(crate::world::GameRules::default());
    let base = seed_province(&w, NationId::France, None, 1.0);
    let nan1 = f64::from_bits(0x7ff8_0000_0000_0001);
    let nan2 = f64::from_bits(0x7ff8_0000_0000_0002);
    let destinations = vec![
        course(1, 1e16, 183.0, 0.0),
        course(1, 0.25, 183.0, 0.0), // first match must stay at index zero
        course(2, 0.4, 183.0, -0.0),
        course(3, 0.2, nan1, 10.0),
        course(4, 0.3, 183.0, nan1),
    ];
    let incoming = vec![
        course(1, 1.0, 183.0, 0.0),
        course(5, 0.1, 183.0, 2.0), // new duplicate must merge with this append
        course(1, -1e16, 183.0, 0.0), // additions cannot be combined/reordered
        course(5, 0.2, 183.0, 2.0),
        course(2, 0.1, 183.0, 0.0), // signed zero is a different key
        course(3, 0.1, nan1, 10.0),
        course(3, 0.1, nan2, 10.0), // distinct NaN payload remains distinct
        course(4, 0.1, 183.0, nan1),
        course(4, 0.1, 183.0, nan2),
        course(2, 0.1, -0.0, 0.0),
        course(2, 0.1, 0.0, 0.0),
        Course { from: 1, to: 2, ..course(1, 0.1, 183.0, 0.0) },
        Course { from: 0, to: 2, ..course(1, 0.1, 183.0, 0.0) },
    ];
    for empty_to in [false, true] {
        for empty_from in [false, true] {
            let mut actual = base.clone();
            actual.courses = if empty_to { vec![] } else { destinations.clone() };
            let mut expected = actual.clone();
            let mut from = base.clone();
            from.courses = if empty_from { vec![] } else { incoming.clone() };
            let from_before = from.clone();
            for pass in 0..2 {
                if pass == 1 && !actual.courses.is_empty() {
                    // A separate same-date merge must observe changed funding
                    // identity rather than retaining an index across calls.
                    actual.courses[0].funded_days = 11.0;
                    expected.courses[0].funded_days = 11.0;
                }
                add_people(&mut actual, &from);
                add_people_original(&mut expected, &from);
                assert_population(&actual, &expected);
                assert_population(&from, &from_before);
                if pass == 0 && !empty_to && !empty_from {
                    assert_eq!(actual.courses[0].people_m.to_bits(), 0.0_f64.to_bits());
                    assert_eq!(actual.courses[1].people_m.to_bits(), 0.25_f64.to_bits());
                    assert_eq!(actual.courses[5].started_day, 5);
                    assert_eq!(actual.courses[5].people_m.to_bits(), (0.1_f64+0.2).to_bits());
                }
            }
        }
    }
}

fn migration_fixture() -> WorldState {
    let mut w = crate::init::world_1990(crate::world::GameRules {
        daily_simulation: true, ai_aggression: 0.0, ..Default::default()
    });
    w.player = Some(NationId::France);
    enable(&mut w).unwrap();
    let ids = w.population_system.nations[&NationId::France].province_ids.clone();
    assert!(ids.len() >= 3);
    for (i, id) in ids.iter().enumerate() {
        let p = w.population_system.provinces.get_mut(id).unwrap();
        p.children.clear(); p.retirees_m = 0.0; p.military_m = 0.0;
        p.working = [1.0, 0.0, 0.0, 0.0, 0.0]; p.participation = 0.7;
        p.jobs = [[0.0; 3]; 8]; p.project_jobs = [[0.0; 3]; 8];
        if i == 0 { p.jobs[0][0] = 2.0; }
        p.jobs_reference = p.jobs;
        p.courses = (0..if i==0 { 512 } else { 16 }).map(|j|
            course(-1000-j, 1e-8, 1461.0, if i==0 { 20.0 } else { 19.0 })).collect();
        match_jobs(p);
    }
    write_populations(&mut w);
    publish_outcomes(&mut w);
    w
}

#[test]
fn indexed_migration_matches_full_world_and_native_day_outcomes() {
    let mut actual = migration_fixture();
    let mut expected = actual.clone();
    let comparisons = ORIGINAL_COMPARISONS.with(Cell::get);
    let scanned = SCANNED.with(Cell::get);
    let before = crate::save(&actual);
    for pass in 0..2 {
        migrate(&mut actual);
        original(|| migrate(&mut expected));
        assert!(crate::save(&actual)==crate::save(&expected), "full migration world pass {pass}");
        assert_eq!(actual.headlines, expected.headlines);
    }
    assert_ne!(crate::save(&actual), before, "fixture must move real residents and courses");
    assert!(ORIGINAL_COMPARISONS.with(Cell::get)-comparisons > SCANNED.with(Cell::get)-scanned,
        "exercise genuinely avoided repeated destination scans");
    for day in 0..4 {
        let a = crate::tick_day(&mut actual, &[]);
        let b = original(|| crate::tick_day(&mut expected, &[]));
        assert_eq!(a,b,"returned headlines day {day}");
        assert_eq!(actual.headlines,expected.headlines,"retained headlines day {day}");
        assert!(crate::save(&actual)==crate::save(&expected),"complete native world day {day}");
    }
}

#[test]
#[ignore="explicit immutable actual-checkpoint 31-day original course-merge oracle; no timing assertions"]
fn indexed_course_merge_matches_actual_checkpoint_for_31_complete_days() {
    let path = std::env::var("SPHERES_S22_CHECKPOINT").expect("actual campaign checkpoint required");
    let source = std::fs::read_to_string(&path).unwrap();
    let mut value: serde_json::Value = serde_json::from_str(&source).unwrap();
    let world = if value.get("format").and_then(serde_json::Value::as_str)==Some("spheres-campaign") {
        let world=value.get_mut("world").expect("campaign world payload").take();
        drop(value); world
    } else { value };
    let mut actual = crate::load_value(world).unwrap();
    assert!(active(&actual),"actual population workload required");
    let mut expected = actual.clone();
    let comparisons = ORIGINAL_COMPARISONS.with(Cell::get);
    let scanned = SCANNED.with(Cell::get);
    for day in 0..31 {
        let a = crate::tick_day(&mut actual, &[]);
        let b = original(|| crate::tick_day(&mut expected, &[]));
        assert_eq!(a,b,"returned headlines day {day}");
        assert_eq!(actual.headlines,expected.headlines,"retained headlines day {day}");
        assert!(crate::save(&actual)==crate::save(&expected),"complete native world day {day}");
    }
    let comparisons = ORIGINAL_COMPARISONS.with(Cell::get)-comparisons;
    let scanned = SCANNED.with(Cell::get)-scanned;
    assert!(comparisons > scanned,"actual migration must avoid repeated searches: original={comparisons}, scanned={scanned}");
    assert_eq!(std::fs::read_to_string(path).unwrap(),source,"immutable source checkpoint");
    eprintln!("31 native days matched original course merges; {comparisons} original row comparisons replaced by {scanned} destination index probes; no timing claim.");
}
