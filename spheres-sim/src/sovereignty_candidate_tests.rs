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

fn quote_bits(q: &CompactQuote) -> [u64; 5] {
    [q.dependency, q.reciprocal_dependency, q.relations, q.size_ratio, q.political_cost]
        .map(f64::to_bits)
}

#[test]
fn compact_scalar_prefilter_keeps_thresholds_nonfinite_values_and_public_quotes_exact() {
    let base = fixture();
    let patron = NationId::USA;
    let partner = NationId::Canada;
    // Explicitly cover the nonfinite refusal asymmetry. The quote rejects a
    // nonfinite computed ratio, but its reputation/relation gates reject only
    // values less than their thresholds. These rows are synthetic edge inputs.
    let cases = [
        ("ratio", 1.499999, true), ("ratio", 1.5, false), ("ratio", 1.500001, false),
        ("ratio", -0.0, true), ("ratio", f64::NAN, true),
        ("ratio", f64::INFINITY, true), ("ratio", f64::NEG_INFINITY, true),
        ("reputation", 49.999999, true), ("reputation", 50.0, false),
        ("reputation", 50.000001, false), ("reputation", -0.0, true),
        ("reputation", f64::NAN, false), ("reputation", f64::INFINITY, false),
        ("reputation", f64::NEG_INFINITY, true),
        ("relations", 54.999999, true), ("relations", 55.0, false),
        ("relations", 55.000001, false), ("relations", -0.0, true),
        ("relations", f64::NAN, false), ("relations", f64::INFINITY, false),
        ("relations", f64::NEG_INFINITY, true),
        // Preserve the exact denominator max rather than inventing a GDP
        // validity gate. NaN, negative and zero partner GDP use the same floor.
        ("partner_gdp", f64::NAN, false), ("partner_gdp", -0.0, false),
        ("partner_gdp", 0.0, false), ("partner_gdp", -1.0, false),
        ("partner_gdp", f64::NEG_INFINITY, false),
        ("partner_gdp", f64::INFINITY, true),
    ];
    let mut new_skips = 0;
    for (field,value,refused) in cases {
        let mut w = base.clone();
        match field {
            "ratio" => w.nation_mut(patron).gdp = value * w.nation(partner).gdp,
            "partner_gdp" => w.nation_mut(partner).gdp = value,
            "reputation" => {
                w.statecraft.reputation.retain(|(id,_)|*id!=patron);
                w.statecraft.reputation.push((patron,value));
                assert_eq!(w.reputation(patron).to_bits(),value.to_bits());
            }
            "relations" => {
                w.relations.set(patron,partner,value);
                assert_eq!(w.relation(patron,partner).to_bits(),value.to_bits());
            }
            _ => unreachable!(),
        }
        let before = crate::save(&w);
        let public_before = quote(&w,patron,partner);
        assert_eq!(candidate_scalar_refusal(&w,patron,partner),refused,"{field}={value}");
        if refused { assert!(!public_before.ready,"every early refusal must already fail the public quote"); }
        if matches!(field,"reputation"|"relations") && value.is_nan() {
            assert!(public_before.ready,"the existing NaN comparison must not be strengthened");
        }
        let before_skips = TEST_SKIPPED_SCALAR_REFUSALS.with(|count|count.get());
        assert_eq!(candidate(&w,patron,true),candidate(&w,patron,false),"{field}={value}");
        let skips=TEST_SKIPPED_SCALAR_REFUSALS.with(|count|count.get())-before_skips;
        if refused { assert!(skips>0,"the protected rejected pair must exercise the new guard"); }
        new_skips += skips;
        let public_after = quote(&w,patron,partner);
        assert_eq!(serde_json::to_vec(&public_after).unwrap(),serde_json::to_vec(&public_before).unwrap());
        assert_eq!(quote_bits(&public_after),quote_bits(&public_before),"exact public float bits");
        assert_eq!(crate::save(&w),before,"all candidate and public reads remain pure");
    }
    assert!(new_skips>0);
}

#[test]
fn compact_scalar_prefilter_reads_subjects_live_without_shortcutting_hostility() {
    let base=fixture();
    let (patron,partner)=(NationId::USA,NationId::Canada);
    for barrier in 0..6 {
        let mut w=base.clone();
        match barrier {
            0=>domination::subjugate(&mut w,NationId::France,patron),
            1=>domination::subjugate(&mut w,patron,partner),
            2=>w.nation_mut(patron).alive=false,
            3=>w.nation_mut(partner).alive=false,
            4=>w.sanctions.push((patron,partner)),
            _=>{
                let theatre=crate::war::theatre_between(&w,patron,NationId::Iraq);
                crate::commitment::open_conflict(&mut w,patron,NationId::Iraq,theatre).unwrap();
            }
        }
        let before=crate::save(&w);
        assert_eq!(candidate_scalar_refusal(&w,patron,partner),barrier<4,
            "bilateral sanctions and conflicts still use the complete quote");
        assert!(!quote(&w,patron,partner).ready);
        assert_eq!(candidate(&w,patron,true),candidate(&w,patron,false),"barrier {barrier}");
        assert_eq!(crate::save(&w),before);
    }
    assert!(candidate_scalar_refusal(&base,patron,patron),"self cannot become a candidate");

    let mut w=base.clone();
    domination::subjugate(&mut w,patron,NationId::Mexico);
    let theatre=crate::war::theatre_between(&w,NationId::Mexico,NationId::Iraq);
    crate::commitment::open_conflict(&mut w,NationId::Mexico,NationId::Iraq,theatre).unwrap();
    assert!(!candidate_scalar_refusal(&w,patron,partner));
    assert!(quote(&w,patron,partner).ready,"an unrelated descendant external war is not a gate");
    assert_eq!(candidate(&w,patron,true),candidate(&w,patron,false));
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
    let before_scalar_skips = TEST_SKIPPED_SCALAR_REFUSALS.with(|count| count.get());
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
    let scalar_skips=TEST_SKIPPED_SCALAR_REFUSALS.with(|count|count.get())-before_scalar_skips;
    assert!(scalar_skips>0,
        "actual native search must exercise the new scalar/subject guard, not only old unprotected skips");
    assert_eq!(std::fs::read_to_string(path).unwrap(), source, "immutable input");
    eprintln!("31 native days matched the complete original compact search; {scalar_skips} protected pairs failed exact scalar/subject prerequisites.");
}
