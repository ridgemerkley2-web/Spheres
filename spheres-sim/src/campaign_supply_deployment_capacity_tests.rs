use super::*;
use std::cell::Cell;

thread_local! {
    pub(super) static FRESH: Cell<bool> = const { Cell::new(false) };
    pub(super) static REUSED: Cell<u64> = const { Cell::new(0) };
}

fn fresh_capacities<T>(run: impl FnOnce() -> T) -> T {
    struct Reset(bool);
    impl Drop for Reset {
        fn drop(&mut self) { FRESH.with(|flag| flag.set(self.0)); }
    }
    let _reset = Reset(FRESH.with(|flag| flag.replace(true)));
    run()
}

fn route_bits(result: Result<Route, String>) -> Result<(Vec<String>, Vec<usize>, u32, bool, u64), String> {
    result.map(|r| (r.nodes, r.edges, r.days, r.sea, r.capacity.to_bits()))
}

#[test]
fn deployment_capacity_reads_preserve_paths_bits_and_distinct_sea_factors() {
    let (mut w, request) = super::tests::fixture();
    logistics::begin_month(&mut w);
    let mut graph = Graph::new(&w);
    // Nonuniform existing reservations and occupation resistance are part of
    // the original formula, including on land edges of a seagoing request.
    for (index, edge) in graph.edges.iter().enumerate() {
        w.logistics.usage_tonnes.insert(edge.key.clone(),
            edge.capacity_tonnes * ((index % 11) as f64 / 7.0));
    }
    for (index, resistance) in graph.resistance.iter_mut().enumerate() {
        *resistance = if index % 3 == 0 { 0.37 } else { 1.0 };
    }
    let before = crate::save(&w);
    let mut permissions = RoutePermissions::default();
    let mut reads = DeploymentCapacities::default();
    let hits = REUSED.with(Cell::get);
    let mut sea_bits = std::collections::BTreeSet::new();
    for denial in [0.0, 0.4, 1.0, f64::NAN, f64::INFINITY, -0.0] {
        for escort in [0.0, 0.3, 1.0] {
            let mut r = request.clone();
            r.sea_denial = denial;
            r.sea_escort = escort;
            sea_bits.insert(sea_factor(&w, &r).to_bits());
            for district in ["DE-BB", "DE-SN", "DE-BE", "unknown-district", "DE-BB"] {
                r.district = district.into();
                let actual = route_bits(route_with_reads(&w, &graph, &r, "DE-BE",
                    Some(&mut permissions), Some(&mut reads)));
                let expected = route_bits(route(&w, &graph, &r, "DE-BE"));
                assert_eq!(actual, expected, "destination {district}, denial {denial}, escort {escort}");
            }
        }
    }
    assert!(REUSED.with(Cell::get) > hits, "the exact shared edge reads must be exercised");
    assert!(sea_bits.len() > 1, "the fixture must exercise distinct sea multipliers");
    assert_eq!(reads.by_sea_bits.len(), sea_bits.len());
    assert!(reads.by_sea_bits.values().all(|slots| slots.len() == graph.edges.len()));
    assert_eq!(crate::save(&w), before, "quotes remain read-only");
}

#[test]
fn deployment_capacity_scope_observes_same_day_changes_and_drops_before_service() {
    let (base, request) = super::tests::fixture();
    for case in 0..4 {
        let mut actual = base.clone();
        logistics::begin_month(&mut actual);
        // Synthetic boundary inputs exercise same-date changed reservations,
        // installed infrastructure and exhausted capacity; no date cache.
        if case == 1 {
            for row in &mut actual.production.provinces { row.infrastructure = 5; }
        }
        if case >= 2 {
            for (index, edge) in Graph::new(&actual).edges.into_iter().enumerate() {
                actual.logistics.usage_tonnes.insert(edge.key,
                    edge.capacity_tonnes * if case == 3 { 2.0 } else { (index % 7) as f64 / 8.0 });
            }
        }
        let before = crate::save(&actual);
        let mut quotes = DeploymentRoutes::new(&actual);
        for district in ["DE-BB", "DE-SN", "DE-BE", "DE-BB"] {
            assert_eq!(quotes.route(request.nation, district),
                fresh_capacities(|| deployment_route(&actual, request.nation, district)));
        }
        let opening = quotes.into_supply_graph();
        assert_eq!(crate::save(&actual), before);
        let mut original = actual.clone();
        let mut large = request.clone();
        actual.nation_mut(large.nation).mil_strength = 1_000_000.0;
        original.nation_mut(large.nation).mil_strength = 1_000_000.0;
        large.deployed = 100_000.0;
        let mut second = large.clone();
        second.key = "later-capacity-contender".into();
        let requests = [large, second];
        // The game reinstalls only its detached campaign book at this boundary.
        // Strength changes above do not affect graph capacities or permissions.
        let delivered = prepare_with_graph(&mut actual, &requests, opening);
        let expected = fresh_capacities(|| prepare(&mut original, &requests));
        assert_eq!(delivered, expected, "live deliveries case {case}");
        assert!(crate::save(&actual) == crate::save(&original), "all reservations and receipts case {case}");
        assert_eq!(actual.headlines, original.headlines);
    }

    let mut changed = base.clone();
    logistics::begin_month(&mut changed);
    assert!(deployment_route(&changed, request.nation, &request.district).is_some());
    for edge in Graph::new(&changed).edges {
        changed.logistics.usage_tonnes.insert(edge.key, edge.capacity_tonnes * 2.0);
    }
    assert!(deployment_route(&changed, request.nation, &request.district).is_none(),
        "a new scope on the same date must see exhausted freight capacity");
}

#[test]
fn deployment_capacity_reuse_matches_complete_native_days() {
    let mut actual = graph_handoff_tests::campaign_fixture();
    let mut original = actual.clone();
    let before = REUSED.with(Cell::get);
    for day in 0..8 {
        let a = crate::tick_day(&mut actual, &[]);
        let b = fresh_capacities(|| crate::tick_day(&mut original, &[]));
        assert_eq!(a, b, "returned headlines day {day}");
        assert_eq!(actual.headlines, original.headlines, "retained headlines day {day}");
        assert!(crate::save(&actual) == crate::save(&original), "complete world day {day}");
    }
    assert!(REUSED.with(Cell::get) > before, "complete days must exercise capacity reuse");
}

#[test]
#[ignore = "immutable actual-checkpoint 31-day original capacity reads; no timing assertions"]
fn deployment_capacity_reuse_matches_actual_checkpoint_for_31_complete_days() {
    let path = std::env::var("SPHERES_S22_CHECKPOINT").expect("actual campaign checkpoint required");
    let source = std::fs::read_to_string(&path).unwrap();
    let mut value: serde_json::Value = serde_json::from_str(&source).unwrap();
    let world = if value.get("format").and_then(serde_json::Value::as_str) == Some("spheres-campaign") {
        let world = value.get_mut("world").expect("campaign world").take();
        drop(value);
        world
    } else { value };
    let mut actual = crate::load_value(world).unwrap();
    assert!(crate::campaign::enabled(&actual));
    let mut original = actual.clone();
    let before = REUSED.with(Cell::get);
    for day in 0..31 {
        let a = crate::tick_day(&mut actual, &[]);
        let b = fresh_capacities(|| crate::tick_day(&mut original, &[]));
        assert_eq!(a, b, "returned headlines day {day}");
        assert_eq!(actual.headlines, original.headlines, "retained headlines day {day}");
        assert!(crate::save(&actual) == crate::save(&original), "complete world day {day}");
    }
    let reused = REUSED.with(Cell::get) - before;
    assert!(reused > 0, "actual input must exercise repeated immutable edge reads");
    assert_eq!(std::fs::read_to_string(path).unwrap(), source, "immutable source checkpoint");
    eprintln!("31 complete native days matched original capacity reads; {reused} repeated edge calculations avoided.");
}
