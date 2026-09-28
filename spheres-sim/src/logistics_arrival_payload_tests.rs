//! Exact freight payload/read parity; synthetic fixture changes are not campaign evidence.
use super::*;
use std::cell::Cell;

thread_local! {
    pub(super) static ORIGINAL: Cell<bool> = const { Cell::new(false) };
    static ELIMINATED_CARGO: Cell<usize> = const { Cell::new(0) };
    static ELIMINATED_NODES: Cell<usize> = const { Cell::new(0) };
    static ELIMINATED_STRINGS: Cell<usize> = const { Cell::new(0) };
}

fn original<T>(run: impl FnOnce() -> T) -> T {
    struct Reset(bool);
    impl Drop for Reset { fn drop(&mut self) { ORIGINAL.with(|flag| flag.set(self.0)); } }
    let _reset = Reset(ORIGINAL.with(|flag| flag.replace(true)));
    run()
}

pub(super) fn record_eliminated_payload(cargo: &Cargo) {
    let route = &cargo.route;
    let bytes = route.nodes.iter().map(|n| n.id.len() + n.name.len() + n.kind.len()).sum::<usize>()
        + route.segments.iter().map(String::len).sum::<usize>()
        + route.chokepoints.iter().map(String::len).sum::<usize>()
        + route.mode.len() + route.bottleneck.len()
        + route.dispatch_note.as_ref().map_or(0, String::len)
        + cargo.hold_reason.as_ref().map_or(0, String::len);
    ELIMINATED_CARGO.with(|n| n.set(n.get() + 1));
    ELIMINATED_NODES.with(|n| n.set(n.get() + route.nodes.len()));
    ELIMINATED_STRINGS.with(|n| n.set(n.get() + bytes));
}

fn counts() -> (usize, usize, usize) {
    (ELIMINATED_CARGO.with(Cell::get), ELIMINATED_NODES.with(Cell::get), ELIMINATED_STRINGS.with(Cell::get))
}

fn fixture(daily: bool, modern: bool) -> WorldState {
    let mut w = crate::init::world_1990(crate::world::GameRules {
        daily_simulation: daily, resource_gates: true, resource_market: true,
        logistics_routes: true, physical_logistics: true, military_operations: modern,
        ..Default::default()
    });
    w.conflicts.clear(); w.sanctions.clear();
    let route = plan(&w, NationId::Germany, NationId::France).unwrap();
    let now = resources::month_abs(&w); let today = crate::clock::absolute_day(&w);
    for (i, commodity) in [Commodity::Copper, Commodity::Iron, Commodity::Copper].into_iter().enumerate() {
        w.logistics.cargo.push(Cargo { id: (3 - i) as u64, seller: NationId::Germany, buyer: NationId::France,
            commodity, quantity: 0.123456789 + i as f64, source: ShipmentSource::Contract,
            contract: Some(772), route: route.clone(), dispatched_month: now - 1,
            due_month: now, dispatched_day: Some(today - 10), due_day: Some(today), hold_reason: None });
    }
    w.logistics.usage_tonnes.insert("opening usage must clear".into(), 5.0);
    w
}

#[test]
fn arrival_payload_callbacks_preserve_public_results_order_and_all_guard_paths() {
    for daily in [false, true] { for modern in [false, true] { for case in 0..7 {
        let mut actual = fixture(daily, modern);
        match case {
            1 => actual.rules.physical_logistics = false,
            2 => actual.logistics.cargo.clear(),
            3 => {
                // A retained nonempty arrival ledger must not be credited again.
                actual.logistics.arrivals = actual.logistics.cargo.clone();
                actual.logistics.last_month = Some(resources::month_abs(&actual));
                actual.logistics.last_day = Some(crate::clock::absolute_day(&actual));
            }
            4 => actual.sanctions.push((NationId::France, NationId::Germany)),
            5 => actual.nation_mut(NationId::France).alive = false,
            6 => actual.logistics.cargo[1].route.nodes[0].id = "missing-booked-node".into(),
            _ => {}
        }
        let mut expected = actual.clone();
        let legacy = begin_month(&mut expected);
        let mut credited = vec![];
        for_new_arrivals(&mut actual, |n, c, q| credited.push((n, c, q.to_bits())));
        assert_eq!(credited, legacy.iter().map(|c| (c.buyer, c.commodity, c.quantity.to_bits())).collect::<Vec<_>>());
        assert_eq!(crate::save(&actual), crate::save(&expected), "complete freight ledger daily={daily} modern={modern} case={case}");
        if case == 0 {
            assert_eq!(actual.logistics.arrivals.iter().map(|c| c.id).collect::<Vec<_>>(), [1, 2, 3]);
            let mut independently_owned = legacy;
            independently_owned[0].quantity = 500.0;
            assert_ne!(independently_owned[0], expected.logistics.arrivals[0], "public result remains independently owned");
        }
        let before = crate::save(&actual);
        let mut replayed = 0;
        for_new_arrivals(&mut actual, |_, _, _| replayed += 1);
        assert_eq!(replayed, 0); assert!(begin_month(&mut expected).is_empty());
        assert_eq!(crate::save(&actual), before, "read/retry must not clear receipts or replay deliveries");
        assert_eq!(crate::save(&actual), crate::save(&expected));
        let mut loaded = crate::load(&before).unwrap();
        for_new_arrivals(&mut loaded, |_, _, _| panic!("loaded same-date freight cannot credit twice"));
        assert_eq!(crate::save(&loaded), before);
    }}}
}

#[test]
fn arrival_payload_raw_purchase_and_ordinary_posting_match_original_credits() {
    let mut actual = crate::init::world_1990(crate::world::GameRules {
        daily_simulation: true, resource_gates: true, resource_market: true,
        logistics_routes: true, physical_logistics: true, ..Default::default()
    });
    resources::warm(&mut actual);
    resources::materialize_opening_market(&mut actual).unwrap();
    let market = actual.resources.market.as_mut().unwrap(); market.stocks.clear(); market.prices = [1000.0; 12];
    for (id, cash) in [(NationId::France, 1.0), (NationId::Germany, 0.0)] {
        let n = actual.nation_mut(id); n.treasury_bn = Some(cash); n.debt_bn = Some(0.0); n.debt_gdp = 0.0;
    }
    resources::set_stockpile_for_test(&mut actual, NationId::Germany, Commodity::Copper, 10_000_000.0);
    let mut request = [0.0; 12]; request[Commodity::Copper.idx()] = 100.0;
    let mut expected = actual.clone();
    let a = resources::purchase_raw_bundle(&mut actual, NationId::France, request, 1.0).unwrap();
    let b = original(|| resources::purchase_raw_bundle(&mut expected, NationId::France, request, 1.0)).unwrap();
    assert_eq!(a, b); assert_eq!(crate::save(&actual), crate::save(&expected));
    let due = actual.logistics.cargo[0].due_day.unwrap();
    for w in [&mut actual, &mut expected] {
        let (year, month, day) = crate::clock::date_from_day(due); w.year = year; w.month = month; w.day = day;
    }
    let before = counts();
    let a = resources::purchase_raw_bundle(&mut actual, NationId::France, request, 1.0).unwrap();
    let b = original(|| resources::purchase_raw_bundle(&mut expected, NationId::France, request, 1.0)).unwrap();
    assert_eq!(a, b); assert_eq!(crate::save(&actual), crate::save(&expected));
    assert_eq!(resources::stockpile(&actual, NationId::France, Commodity::Copper), 100.0);
    assert_eq!(resources::stockpile(&expected, NationId::France, Commodity::Copper), 100.0);
    assert!(counts().0 > before.0 && counts().1 > before.1 && counts().2 > before.2, "eliminate a real complete route payload");
    resources::tick(&mut actual); original(|| resources::tick(&mut expected));
    assert_eq!(crate::save(&actual), crate::save(&expected), "ordinary posting after reviewed dispatch does not replay cash or cargo");
    let posted = crate::save(&actual);
    resources::tick(&mut actual); original(|| resources::tick(&mut expected));
    assert_eq!(crate::save(&actual), posted); assert_eq!(crate::save(&actual), crate::save(&expected));
}

#[test]
#[ignore = "actual immutable 2015/2035 checkpoint 31-day full freight payload oracle; no timing claim"]
fn arrival_payload_reuse_matches_actual_checkpoint_for_31_complete_days() {
    let path = std::env::var("SPHERES_S22_CHECKPOINT").expect("actual checkpoint required");
    let source = std::fs::read_to_string(&path).unwrap();
    let mut value: serde_json::Value = serde_json::from_str(&source).unwrap();
    let world = if value["format"] == "spheres-campaign" { value["world"].take() } else { value.take() };
    drop(value);
    let mut actual = crate::load_value(world).unwrap();
    assert!(actual.year == 2015 || actual.year == 2035); assert!(enabled(&actual));
    let mut expected = actual.clone(); let before = counts();
    // Ordinary complete simulation days without new orders or budget renewal;
    // this equivalence scenario is separate from qualification control commands.
    for day in 0..31 {
        let a = crate::tick_day(&mut actual, &[]);
        let b = original(|| crate::tick_day(&mut expected, &[]));
        assert_eq!(a, b, "returned headlines day {day}");
        assert_eq!(actual.headlines, expected.headlines, "retained headlines day {day}");
        assert!(crate::save(&actual) == crate::save(&expected), "complete world day {day}");
    }
    let after = counts(); let eliminated = (after.0 - before.0, after.1 - before.1, after.2 - before.2);
    assert!(eliminated.0 > 0 && eliminated.1 > 0 && eliminated.2 > 0, "actual freight must avoid nonempty copied route payloads");
    assert_eq!(std::fs::read_to_string(path).unwrap(), source, "immutable actual checkpoint");
    eprintln!("31 exact native days: removed {} full Cargo copies, {} route-node copies and {} route/hold string bytes (content bytes, not allocator or timing measurements).", eliminated.0, eliminated.1, eliminated.2);
}
