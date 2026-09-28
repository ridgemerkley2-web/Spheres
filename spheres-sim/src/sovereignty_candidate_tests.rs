use super::*;
use crate::world::{GameRules, Pact, TradePact};

fn fixture() -> WorldState {
    let mut w = crate::init::world_1990(GameRules {
        daily_simulation: true,
        economic_competition: true,
        ..GameRules::default()
    });
    w.player = Some(NationId::France);
    w.statecraft.pacts.clear();
    w.statecraft.trade.clear();
    for n in &mut w.nations { n.political_capital = 0.0; }
    w.nation_mut(NationId::USA).political_capital = 100.0;
    w.nation_mut(NationId::USA).gdp = 1000.0;
    for partner in [NationId::Canada, NationId::UK] {
        w.nation_mut(partner).gdp = 100.0;
        w.shift_relation(NationId::USA, partner, 100.0);
        w.statecraft.pacts.push(Pact {
            a: NationId::USA.min(partner), b: NationId::USA.max(partner),
            since_year: 1990, since_month: 1,
        });
        w.statecraft.trade.push(TradePact {
            a: NationId::USA.min(partner), b: NationId::USA.max(partner), depth: 0.4,
        });
    }
    w
}

#[test]
fn compact_candidate_prefilter_preserves_ties_refusals_and_live_gates() {
    let base = fixture();
    let expected = NationId::Canada.min(NationId::UK);
    assert_eq!(candidate(&base, NationId::USA, false), Some(expected));
    for barrier in 0..16 {
        let mut w = base.clone();
        match barrier {
            0 => {},
            1 => w.statecraft.pacts.clear(),
            2 => w.statecraft.trade.clear(),
            3 => w.shift_relation(NationId::USA, expected, -100.0),
            4 => w.sanctions.push((NationId::USA, expected)),
            5 => w.nation_mut(NationId::USA).gdp = f64::NAN,
            6 => w.nation_mut(expected).alive = false,
            7 => w.player = Some(expected),
            8 => domination::subjugate(&mut w, NationId::France, NationId::USA),
            9 => domination::subjugate(&mut w, NationId::USA, expected),
            10 => w.rules.economic_competition = false,
            11 => w.statecraft.pacts.reverse(),
            12 => { let pact = w.statecraft.pacts[0].clone(); w.statecraft.pacts.push(pact); },
            13 => { for pact in &mut w.statecraft.pacts { std::mem::swap(&mut pact.a, &mut pact.b); } },
            14 => w.statecraft.pacts.push(Pact { a: NationId::USA, b: NationId::USA, since_year: 1990, since_month: 1 }),
            _ => w.statecraft.pacts.push(Pact { a: NationId::France, b: NationId::UK, since_year: 1990, since_month: 1 }),
        }
        let before = crate::save(&w);
        for patron in [NationId::USA, NationId::Canada, NationId::UK, NationId::France] {
            assert_eq!(candidate(&w, patron, true), candidate(&w, patron, false),
                "barrier {barrier}, patron {patron:?}");
        }
        assert_eq!(crate::save(&w), before, "candidate filtering must be pure");
    }
}

#[test]
fn compact_candidate_prefilter_preserves_sequential_proposals_and_same_day_reentry() {
    let mut actual = fixture();
    let mut original = actual.clone();
    for day in 0..33 {
        tick_impl(&mut actual, true);
        tick_impl(&mut original, false);
        assert_eq!(crate::save(&actual), crate::save(&original), "day {day}");
        assert_eq!(actual.headlines, original.headlines, "day {day}");
        if day == 0 { assert_eq!(actual.domination.compacts.len(), 1); }
        let before = crate::save(&actual);
        tick_impl(&mut actual, true);
        assert_eq!(crate::save(&actual), before, "same-date review remains idle");
        // Both paths observe live pact/relationship changes at their next review.
        if day == 10 {
            actual.statecraft.pacts.clear(); original.statecraft.pacts.clear();
        }
        clock::advance_date(&mut actual);
        clock::advance_date(&mut original);
    }
}

fn with_original_candidates<T>(run: impl FnOnce() -> T) -> T {
    struct Reset(bool);
    impl Drop for Reset {
        fn drop(&mut self) { TEST_ORIGINAL_CANDIDATES.with(|flag| flag.set(self.0)); }
    }
    let _reset = Reset(TEST_ORIGINAL_CANDIDATES.with(|flag| flag.replace(true)));
    run()
}

#[test]
#[ignore = "actual checkpoint, original compact-search oracle; no timing claims"]
fn compact_candidates_match_actual_checkpoint_for_31_complete_days() {
    let path = std::env::var("SPHERES_S22_CHECKPOINT").expect("actual checkpoint path required");
    let source = std::fs::read_to_string(&path).unwrap();
    let mut value: serde_json::Value = serde_json::from_str(&source).unwrap();
    // Accept the complete campaign envelope without serializing its world again.
    let world = if value["format"] == "spheres-campaign" { value["world"].take() } else { value.take() };
    drop(value);
    let mut actual = crate::load_value(world).unwrap();
    assert!(enabled(&actual));
    assert!(actual.year == 2015 || actual.year == 2035);
    let mut original = actual.clone();
    let before_skips = TEST_SKIPPED_UNPROTECTED.with(|count| count.get());
    let mut monthly_reviews = 0;
    // Ordinary full ticks, no new orders or player-budget renewal. This proves
    // equivalence to the original candidate search, not web workload performance.
    for day in 0..31 {
        monthly_reviews += usize::from(actual.day == 1);
        let received = crate::tick_day(&mut actual, &[]);
        let expected = with_original_candidates(|| crate::tick_day(&mut original, &[]));
        assert_eq!(received, expected, "returned headlines day {day}");
        assert_eq!(actual.headlines, original.headlines, "retained headlines day {day}");
        assert_eq!(crate::save(&actual), crate::save(&original), "complete world day {day}");
    }
    assert!(monthly_reviews > 0);
    assert!(TEST_SKIPPED_UNPROTECTED.with(|count| count.get()) > before_skips,
        "actual native monthly search must exercise the prefilter");
    assert_eq!(std::fs::read_to_string(path).unwrap(), source, "immutable input");
}
