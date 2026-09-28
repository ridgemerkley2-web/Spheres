use super::*;
use crate::world::GameRules;

fn original<T>(run: impl FnOnce() -> T) -> T {
    struct Reset(bool);
    impl Drop for Reset {
        fn drop(&mut self) { TEST_ORIGINAL_CONTRACT_DEPENDENCY.with(|flag| flag.set(self.0)); }
    }
    let _reset = Reset(TEST_ORIGINAL_CONTRACT_DEPENDENCY.with(|flag| flag.replace(true)));
    run()
}

#[test]
fn dependency_pair_gate_matches_original_for_barter_self_pairs_and_nonfinite_legs() {
    let base = crate::init::world_1990(GameRules { daily_simulation: true, ..Default::default() });
    let ids = [NationId::France, NationId::USA, NationId::Canada, NationId::UK];
    for case in 0..9 {
        let mut w = base.clone();
        w.resources.contracts.clear();
        for (i, (from, to)) in [(ids[0], ids[1]), (ids[1], ids[0]), (ids[1], ids[2]), (ids[2], ids[2])].into_iter().enumerate() {
            let amount = match case { 2 => 0.0, 3 => -10.0, 4 => f64::NAN, 5 => f64::INFINITY, _ => 10.0 };
            w.resources.contracts.push(Contract {
                id: i as u32, from, to,
                give: vec![Leg::Commodity { c: Commodity::Iron, per_month: amount },
                    Leg::Commodity { c: Commodity::Iron, per_month: -1.0 },
                    Leg::Commodity { c: Commodity::Oil, per_month: 20.0 },
                    Leg::Money { bn_per_year: 2.0 }],
                take: vec![Leg::Commodity { c: Commodity::Bauxite, per_month: amount }],
                months_left: 0, months_total: 12, days_left: Some(0), since: 0,
                depth: match case { 6 => f64::NAN, 7 => -0.0, _ => 0.4 },
            });
        }
        if case == 0 { w.resources.contracts.clear(); }
        if case == 8 {
            w.nation_mut(ids[1]).alive = false;
            w.resources.contracts.reverse();
            w.resources.contracts.push(w.resources.contracts[0].clone());
        }
        for cached in [false, true] {
            if cached { warm(&mut w); }
            let before = crate::save(&w);
            for id in ids { for partner in ids {
                assert_eq!(contract_dependency(&w, id, partner).to_bits(),
                    contract_dependency_original(&w, id, partner).to_bits(),
                    "case {case}, cached {cached}, {id:?}/{partner:?}");
            }}
            assert_eq!(crate::save(&w), before, "dependency remains a pure read");
        }
    }
}

#[test]
#[ignore = "actual checkpoint, original dependency oracle; no timing claims"]
fn dependency_pair_gate_matches_actual_checkpoint_for_31_complete_days() {
    let path = std::env::var("SPHERES_S22_CHECKPOINT").expect("actual checkpoint path required");
    let source = std::fs::read_to_string(&path).unwrap();
    let mut value: serde_json::Value = serde_json::from_str(&source).unwrap();
    let world = if value["format"] == "spheres-campaign" { value["world"].take() } else { value.take() };
    drop(value);
    let mut actual = crate::load_value(world).unwrap();
    assert!(actual.year == 2015 || actual.year == 2035);
    assert!(!actual.resources.contracts.is_empty());
    let mut expected = actual.clone();
    let before = TEST_UNRELATED_CONTRACT_DEPENDENCY.with(|count| count.get());
    for day in 0..31 {
        let a = crate::tick_day(&mut actual, &[]);
        let b = original(|| crate::tick_day(&mut expected, &[]));
        assert_eq!(a, b, "returned headlines day {day}");
        assert_eq!(actual.headlines, expected.headlines, "retained headlines day {day}");
        assert!(crate::save(&actual) == crate::save(&expected), "complete native world day {day}");
    }
    let skipped = TEST_UNRELATED_CONTRACT_DEPENDENCY.with(|count| count.get()) - before;
    assert!(skipped > 0, "actual checkpoint must exercise unrelated contract pairs");
    assert_eq!(std::fs::read_to_string(path).unwrap(), source, "immutable actual source");
    eprintln!("31 complete days match original dependency; {skipped} unrelated pairs skipped.");
}
