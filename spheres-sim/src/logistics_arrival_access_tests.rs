//! Exact old owned-key arrival-pass oracle; no qualification timing assertions.
use super::*;
use std::cell::Cell;

thread_local! {
    pub(super) static ORIGINAL: Cell<bool> = const { Cell::new(false) };
    pub(super) static REMOVED_KEYS: Cell<usize> = const { Cell::new(0) };
    pub(super) static REMOVED_STRINGS: Cell<usize> = const { Cell::new(0) };
    pub(super) static REMOVED_BYTES: Cell<usize> = const { Cell::new(0) };
    pub(super) static REUSED: Cell<usize> = const { Cell::new(0) };
}

pub(super) fn original_post_arrivals(w: &mut WorldState, reuse_due_routes: bool) -> bool {
    if !enabled(w) {
        return false;
    }
    let now = resources::month_abs(w);
    let daily = crate::clock::is_daily(w);
    let today = crate::clock::absolute_day(w);
    if if daily { w.logistics.last_day == Some(today) }
        else { w.logistics.last_month == Some(now) } {
        return false;
    }
    w.logistics.last_month = Some(now);
    if daily { w.logistics.last_day = Some(today); }
    w.logistics.usage_tonnes.clear();
    w.logistics.arrivals.clear();
    w.logistics.route_cache.clear();
    let old = std::mem::take(&mut w.logistics.cargo);
    let mut keep = vec![];
    let mut arrivals = vec![];
    // Repeated consignments may share a booked path. Control and permissions
    // do not change during this arrival pass, so one exact path check suffices.
    let mut open_routes: BTreeMap<(NationId, NationId, Vec<String>), Result<(), String>> = BTreeMap::new();
    // Due/held consignments also share pure access checks. Keep this separate
    // from the existing in-transit cache: saved names affect exact refusal
    // text, even when the ordered node IDs are identical. No movement, stock,
    // ownership or permissions change until this arrival pass has finished.
    let mut due_routes: BTreeMap<(NationId, NationId, Vec<(String, String)>), Result<(), String>> = BTreeMap::new();
    for mut c in old {
        if daily && c.due_day.is_none() {
            // Legacy freight arrived at the END of due_month: preserve that
            // known boundary, rather than adding months to the load date.
            // Its exact dispatch day was never stored and stays unknown.
            let year = 1990 + c.due_month.div_euclid(12);
            let month = c.due_month.rem_euclid(12) as u32 + 1;
            c.due_day = Some(crate::clock::date_day(year, month,
                crate::world::days_in_month(year, month)));
        }
        let in_transit = if daily { c.due_day.is_some_and(|due| due > today) }
            else { c.due_month > now };
        if in_transit {
            if w.rules.military_operations {
                let key = (c.seller, c.buyer, c.route.nodes.iter().map(|n| n.id.clone()).collect());
                c.hold_reason = open_routes.entry(key).or_insert_with(|| route_open(w, c.seller, c.buyer, &c.route)).clone().err();
            }
            keep.push(c);
            continue;
        }
        let access = if reuse_due_routes {
            let key = (c.seller, c.buyer, c.route.nodes.iter().map(|n| (n.id.clone(), n.name.clone())).collect());
            due_routes.entry(key).or_insert_with(|| route_open(w, c.seller, c.buyer, &c.route)).clone()
        } else { route_open(w, c.seller, c.buyer, &c.route) };
        match access {
            Ok(()) => {
                c.hold_reason = None;
                arrivals.push(c)
            }
            Err(reason) => {
                c.hold_reason = Some(reason);
                keep.push(c)
            }
        }
    }
    arrivals.sort_by_key(|c| c.id);
    keep.sort_by_key(|c| c.id);
    w.logistics.cargo = keep;
    w.logistics.arrivals = arrivals;
    true
}

pub(super) fn record_key(key: &ArrivalAccessKey<'_>, reused: bool) {
    REMOVED_KEYS.with(|n| n.set(n.get() + 1));
    REMOVED_STRINGS.with(|n| n.set(n.get() + key.nodes.len() * if key.names { 2 } else { 1 }));
    let bytes = key.nodes.iter().map(|n| n.id.len() + if key.names { n.name.len() } else { 0 }).sum::<usize>();
    REMOVED_BYTES.with(|n| n.set(n.get() + bytes));
    if reused { REUSED.with(|n| n.set(n.get() + 1)); }
}

fn original<T>(run: impl FnOnce() -> T) -> T {
    struct Reset(bool);
    impl Drop for Reset { fn drop(&mut self) { ORIGINAL.with(|flag| flag.set(self.0)); } }
    let _reset = Reset(ORIGINAL.with(|flag| flag.replace(true))); run()
}

fn world(daily: bool, modern: bool) -> WorldState {
    let mut w = crate::init::world_1990(crate::world::GameRules {
        resource_gates: true, resource_market: true, logistics_routes: true, physical_logistics: true,
        daily_simulation: daily, military_operations: modern, ..Default::default()
    });
    w.conflicts.clear(); w.sanctions.clear(); w
}

#[test]
fn arrival_access_borrowed_order_is_exact_owned_key_order() {
    let w = world(true, true);
    let route = plan(&w, NationId::Germany, NationId::France).unwrap();
    let mut renamed = route.nodes.clone();
    for n in &mut renamed { n.name = format!("Saved α {}", n.name); n.kind = "metadata ignored".into(); n.lon = f64::NAN; }
    let mut changed = route.nodes.clone(); changed[0].id = "missing\0node".into();
    let mut repeated = route.nodes.clone(); repeated.insert(1, repeated[0].clone());
    let mut reversed = route.nodes.clone(); reversed.reverse();
    let lists = [vec![], route.nodes[..1].to_vec(), route.nodes, renamed, changed, repeated, reversed];
    for names in [false, true] { for left in &lists { for right in &lists {
        for (a, b) in [(NationId::Germany, NationId::Germany), (NationId::France, NationId::Germany)] {
            let l = ArrivalAccessKey { seller: a, buyer: NationId::France, nodes: left, names };
            let r = ArrivalAccessKey { seller: b, buyer: NationId::France, nodes: right, names };
            let expected = if names {
                (a, NationId::France, left.iter().map(|n| (n.id.clone(), n.name.clone())).collect::<Vec<_>>())
                    .cmp(&(b, NationId::France, right.iter().map(|n| (n.id.clone(), n.name.clone())).collect::<Vec<_>>()))
            } else {
                (a, NationId::France, left.iter().map(|n| n.id.clone()).collect::<Vec<_>>())
                    .cmp(&(b, NationId::France, right.iter().map(|n| n.id.clone()).collect::<Vec<_>>()))
            };
            assert_eq!(l.cmp(&r), expected); assert_eq!(l == r, expected == Ordering::Equal);
        }
    }}}
}

#[test]
fn arrival_access_borrowing_preserves_due_transit_holds_duplicate_order_and_fresh_dates() {
    for daily in [false, true] { for modern in [false, true] {
        let mut base = world(daily, modern);
        let route = plan(&base, NationId::Germany, NationId::France).unwrap();
        let transit = base.districts.iter().find(|(_, owner)| **owner == NationId::Netherlands).unwrap().0.clone();
        let mut named_a = route.clone();
        let mut node = route.nodes[0].clone(); node.id = transit; node.name = "Saved transit A".into();
        named_a.nodes.insert(1, node);
        let mut named_b = named_a.clone(); named_b.nodes[1].name = "Saved transit B".into();
        let mut missing = route.clone(); missing.nodes[0].id = "unknown booked node".into();
        let mut empty = route.clone(); empty.nodes.clear();
        let today = crate::clock::absolute_day(&base); let now = resources::month_abs(&base);
        for (r, route) in [route, named_a, named_b, missing, empty].into_iter().enumerate() {
            for copy in 0..4 {
                // Duplicate IDs exercise the stable ordering within each final
                // partition; differently named transit paths share only ID keys.
                base.logistics.cargo.push(Cargo { id: r as u64, seller: NationId::Germany, buyer: NationId::France,
                    commodity: Commodity::Iron, quantity: 1.25 + copy as f64, source: ShipmentSource::Spot,
                    contract: None, route: route.clone(), dispatched_month: now - 2,
                    due_month: if copy >= 2 { now + 1 } else { now - 1 }, dispatched_day: None,
                    due_day: match copy { 0 => None, 1 => Some(today), _ => Some(today + 1) },
                    hold_reason: Some("Old hold must be reevaluated".into()) });
            }
        }
        base.logistics.cargo.reverse();
        for case in 0..7 {
            let mut actual = base.clone();
            match case {
                1 => actual.sanctions.push((NationId::Netherlands, NationId::Germany)),
                2 => actual.sanctions.push((NationId::Germany, NationId::France)),
                3 => actual.nation_mut(NationId::France).alive = false,
                4 => actual.rules.physical_logistics = false,
                5 => actual.logistics.cargo.clear(),
                6 => {
                    actual.logistics.arrivals = actual.logistics.cargo.clone();
                    actual.logistics.last_month = Some(now); actual.logistics.last_day = Some(today);
                }
                _ => {}
            }
            let mut expected = actual.clone();
            let a = begin_month(&mut actual); let b = original(|| begin_month(&mut expected));
            assert_eq!(a, b, "complete public arrivals daily={daily} modern={modern} case={case}");
            assert_eq!(crate::save(&actual), crate::save(&expected), "all routes, names, dates, quantities and holds");
            assert!(begin_month(&mut actual).is_empty()); assert!(original(|| begin_month(&mut expected)).is_empty());
            assert_eq!(crate::save(&actual), crate::save(&expected));
            for w in [&mut actual, &mut expected] {
                w.sanctions.clear(); w.nation_mut(NationId::France).alive = true;
                if daily { crate::clock::advance_date(w); } else { w.month += 1; }
            }
            assert_eq!(begin_month(&mut actual), original(|| begin_month(&mut expected)), "new date reads current access");
            assert_eq!(crate::save(&actual), crate::save(&expected));
        }
    }}
}

#[test]
#[ignore = "actual immutable 2015/2035 checkpoint old owned-key freight pass for31days; no timing claim"]
fn arrival_access_borrowed_keys_match_actual_checkpoint_for_31_complete_days() {
    let path = std::env::var("SPHERES_S22_CHECKPOINT").expect("actual checkpoint required");
    let source = std::fs::read_to_string(&path).unwrap();
    let mut value: serde_json::Value = serde_json::from_str(&source).unwrap();
    let saved = if value["format"] == "spheres-campaign" { value["world"].take() } else { value.take() };
    drop(value);
    let mut actual = crate::load_value(saved).unwrap();
    assert!(actual.year == 2015 || actual.year == 2035); assert!(enabled(&actual));
    let mut expected = actual.clone();
    let counts = || (REMOVED_KEYS.with(Cell::get), REMOVED_STRINGS.with(Cell::get), REMOVED_BYTES.with(Cell::get), REUSED.with(Cell::get));
    let before = counts();
    // No new orders/budget renewals; this is a complete simulation equivalence
    // scenario, separate from qualification timing/control commands.
    for day in 0..31 {
        let a = crate::tick_day(&mut actual, &[]); let b = original(|| crate::tick_day(&mut expected, &[]));
        assert_eq!(a, b, "returned headlines day{day}"); assert_eq!(actual.headlines, expected.headlines);
        assert!(crate::save(&actual) == crate::save(&expected), "complete world day{day}");
    }
    let after = counts(); let delta = (after.0-before.0, after.1-before.1, after.2-before.2, after.3-before.3);
    assert!(delta.0 > 0 && delta.1 > 0 && delta.2 > 0 && delta.3 > 0, "actual nonempty paths must reuse cache entries without owned key copies");
    assert_eq!(std::fs::read_to_string(path).unwrap(), source);
    eprintln!("31 exact native days: avoided {} owned route-key vectors, {} node-string copies/{} content bytes; {} repeated-key hits (not timing/allocator measurements).",delta.0,delta.1,delta.2,delta.3);
}
