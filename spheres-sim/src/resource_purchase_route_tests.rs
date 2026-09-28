//! Monthly buyer-route reuse against the unchanged fresh route predicate.
use super::*;
use std::cell::Cell;

thread_local! {
    pub(super) static ORIGINAL: Cell<bool> = const { Cell::new(false) };
    pub(super) static REUSED: Cell<usize> = const { Cell::new(0) };
}

fn original<T>(run: impl FnOnce() -> T) -> T {
    struct Reset(bool);
    impl Drop for Reset { fn drop(&mut self) { ORIGINAL.with(|flag| flag.set(self.0)); } }
    let _reset = Reset(ORIGINAL.with(|flag| flag.replace(true))); run()
}

fn fixture() -> (WorldState, Vec<Commodity>, Vec<NationId>) {
    let buyer = NationId::France;
    let mut w = crate::init::world_1990(crate::world::GameRules {
        resource_gates: true, resource_market: true, logistics_routes: true,
        physical_logistics: true, military_operations: true, ..Default::default()
    });
    w.reindex(); tick(&mut w);
    w.player = None; w.sanctions.clear(); w.conflicts.clear();
    w.resources.cover.clear(); w.resources.contracts.clear(); w.resources.offers.clear(); w.resources.refusals.clear();
    w.nation_mut(buyer).political_capital = 1_000.0;
    w.nation_mut(buyer).mil_spend_gdp = 0.000001;
    // An explicit synthetic two-material recipe; default staff picks may need
    // only one material and would not prove reuse across a buyer's lines.
    let armour = crate::arsenal::index_of("trophy").unwrap();
    let tech = crate::tech::index_of(crate::arsenal::DECK[armour as usize].tech.unwrap()).unwrap();
    if let Err(index) = w.nation(buyer).tech.known.binary_search(&tech) {
        w.nation_mut(buyer).tech.known.insert(index, tech);
    }
    w.nation_mut(buyer).arsenal.preference = Some("trophy".into());
    w.nation_mut(buyer).arsenal.banked = 0.0;
    let n = w.nation(buyer); let kit = crate::arsenal::pick(n).expect("fixture military line");
    let need = kit_need(kit, crate::arsenal::line_of(n));
    let commodities: Vec<_> = ALL.into_iter().filter(|c| c.tracked() && need[c.idx()] > 0.0).collect();
    assert!(commodities.len() >= 2, "one buyer must actually review several materials");
    w.resource_have.flow[buyer.index()] = [1.0e9; 12];
    let sellers = vec![NationId::USA, NationId::Germany, NationId::Chile];
    for &c in &commodities {
        for id in all_nations() {
            w.resource_have.flow[id.index()][c.idx()] = 0.0;
            w.resource_have.presence[id.index()] &= !c.bit();
        }
        for &seller in &sellers {
            w.resource_have.flow[seller.index()][c.idx()] = 1.0e9;
            w.resource_have.presence[seller.index()] |= c.bit();
            set_stockpile_for_test(&mut w, seller, c, 1.0e9);
            w.set_relation(buyer, seller, 60.0);
        }
        write_cover(&mut w, buyer, c, 0.0, need[c.idx()]);
        assert_eq!(supply(&w, buyer, c, need[c.idx()]).available, 0.0);
    }
    let sellers = producers(&w, commodities[0]);
    (w, commodities, sellers)
}

#[test]
fn purchase_route_memo_matches_full_waves_signings_offers_refusals_and_cooldowns() {
    let (base, commodities, sellers) = fixture();
    let buyer = NationId::France; let first = sellers[0];
    for case in 0..8 {
        let mut actual = base.clone();
        match case {
            1 => { for &c in &commodities { remember_refusal(&mut actual, buyer, first, c, Reason::NoSurplus); } }
            2 => actual.sanctions.push((buyer, first)),
            3 => { for &seller in &sellers { actual.sanctions.push((seller, buyer)); } }
            4 => actual.player = Some(first),
            5 => actual.nation_mut(buyer).political_capital = 0.0,
            6 => { for &seller in &sellers { actual.set_relation(buyer, seller, -99.0); } }
            7 => actual.rules.physical_logistics = false,
            _ => {}
        }
        let mut expected = actual.clone();
        meter::BUY_ROUTE_PLANS.with(|n| n.set(0)); meter::ASKS.with(|n| n.set([0; 5]));
        original(|| buy_pass(&mut expected));
        let old_plans = meter::BUY_ROUTE_PLANS.with(Cell::get); let old_asks = meter::ASKS.with(Cell::get);
        meter::BUY_ROUTE_PLANS.with(|n| n.set(0)); meter::ASKS.with(|n| n.set([0; 5]));
        let before = REUSED.with(Cell::get); buy_pass(&mut actual);
        assert_eq!(meter::ASKS.with(Cell::get), old_asks, "same ordered ask outcomes case{case}");
        assert_eq!(crate::save(&actual), crate::save(&expected), "complete world/refusals/prices/headlines/RNG case{case}");
        if case == 0 {
            assert!(REUSED.with(Cell::get) > before);
            assert!(meter::BUY_ROUTE_PLANS.with(Cell::get) < old_plans, "avoid actual expensive native route searches");
            assert!(actual.resources.contracts.iter().filter(|c| c.from == buyer).count() >= 2, "several ordinary material signatures, with live cap/evaluation after the first");
        }
        if case == 4 { assert!(actual.resources.offers.iter().any(|o| o.from == buyer && o.to == first)); }
        if case == 3 || case == 5 || case == 6 { assert!(actual.resources.contracts.is_empty()); }
        // Every invocation owns a new memo, including retries after fresh
        // refusal memory, offers and newly signed commitments.
        buy_pass(&mut actual); original(|| buy_pass(&mut expected));
        assert_eq!(crate::save(&actual), crate::save(&expected), "fresh complete wave case{case}");
    }
}

#[test]
fn purchase_route_memo_is_local_and_later_waves_read_changed_access_and_policy() {
    let (mut w, _, _) = fixture();
    let (seller, buyer) = (NationId::Germany, NationId::France);
    let mut scope = PurchaseRoutes::default();
    assert!(scope.is_open(&w, seller, buyer));
    w.set_relation(seller, buyer, -100.0);
    assert_eq!(scope.is_open(&w, seller, buyer), purchase_route_open(&w, seller, buyer), "route permission does not use relation");
    drop(scope);
    for state in 0..5 {
        match state {
            0 => w.sanctions.push((seller, buyer)),
            1 => w.sanctions.clear(),
            2 => w.nation_mut(seller).alive = false,
            3 => w.nation_mut(seller).alive = true,
            4 => { crate::logistics::set_policy(&mut w, buyer, crate::logistics::RoutePolicy::LandOnly).unwrap(); }
            _ => {}
        }
        let before = crate::save(&w); let mut next = PurchaseRoutes::default();
        let expected = purchase_route_open(&w, seller, buyer);
        assert_eq!(next.is_open(&w, seller, buyer), expected); assert_eq!(next.is_open(&w, seller, buyer), expected);
        assert_eq!(crate::save(&w), before);
        if state == 0 || state == 2 { assert!(!expected); }
    }
}

#[test]
#[ignore = "actual immutable 2015/2035 checkpoint fresh monthly route bools for31days; no timing claim"]
fn purchase_route_memo_matches_actual_checkpoint_for_31_complete_days() {
    let path = std::env::var("SPHERES_S22_CHECKPOINT").expect("actual checkpoint required");
    let source = std::fs::read_to_string(&path).unwrap();
    let mut value: serde_json::Value = serde_json::from_str(&source).unwrap();
    let saved = if value["format"] == "spheres-campaign" { value["world"].take() } else { value.take() }; drop(value);
    let mut actual = crate::load_value(saved).unwrap();
    assert!(actual.year == 2015 || actual.year == 2035); assert!(crate::logistics::enabled(&actual));
    let mut expected = actual.clone(); let reused = REUSED.with(Cell::get); let mut new_plans = 0u32; let mut old_plans = 0u32;
    // No new orders or budget renewal: this full simulation equivalence run
    // is independent of qualification control commands and timing evidence.
    for day in 0..31 {
        meter::BUY_ROUTE_PLANS.with(|n| n.set(0)); let a = crate::tick_day(&mut actual, &[]);
        new_plans += meter::BUY_ROUTE_PLANS.with(Cell::get);
        meter::BUY_ROUTE_PLANS.with(|n| n.set(0)); let b = original(|| crate::tick_day(&mut expected, &[]));
        old_plans += meter::BUY_ROUTE_PLANS.with(Cell::get);
        assert_eq!(a, b, "returned headlines day{day}"); assert_eq!(actual.headlines, expected.headlines);
        assert!(crate::save(&actual) == crate::save(&expected), "complete world day{day}");
    }
    let reused = REUSED.with(Cell::get) - reused;
    assert!(reused > 0 && new_plans < old_plans, "actual monthly reviews must reuse expensive route searches");
    assert_eq!(std::fs::read_to_string(path).unwrap(), source);
    eprintln!("31 exact native days: {new_plans} route searches vs{old_plans} original; {reused} repeated route-bool reads (not timing measurements).");
}
