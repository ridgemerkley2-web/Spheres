//! THE TREASURY — money as a stock of dollars rather than a ratio.
//!
//! Stage 1 of the ministry economy (design approved by Ridge 2026-09-02).
//! Everything asserted here is either an INVARIANT — a universal claim, which a
//! small sample cannot make red falsely (iron rule 7's own carve-out) — or an
//! exact bit-for-bit reproduction of arithmetic that already shipped. Nothing
//! in this file reads a statistic across seeds, so nothing in it needs an n.
//!
//! Every test carries the RED CHECK that was actually run against it: the
//! mutation made to the tree, and what the test then said. Iron rules 5 and 6.

use spheres_sim::economy::{charge, effective_interest_rate, growth_terms, interest_gdp, Conditions};
use spheres_sim::init::world_1990;
use spheres_sim::world::{AidKind, GameRules, NationId};
use spheres_sim::{save, state_hash, tick_month, Command};

// ---------------------------------------------------------------------------
// 1. THE DEFAULT BOARD DOES NOT MOVE
// ---------------------------------------------------------------------------

/// The two golden ACTUALS this tree is deliberately red at, and which the
/// design forbids re-pinning while the endowment bar (BUGS E-3) is red. They
/// are pinned HERE as the proof that the treasury changed nothing, exactly the
/// way `the_resource_layer_is_inert_at_1990` pins them in lib.rs. If either of
/// these moves, the change is not inert and it is wrong.
const START_ACTUAL: u64 = 0xa5c9c5b2306313d8;
const RUN_ACTUAL: u64 = 0x20c24ab0f1581807;

/// With no budget seated anywhere — which is the default board, every AI
/// nation, and every save written before the treasury existed — a 240-month
/// world is bit-identical to the one that shipped, and its save is
/// byte-identical.
///
/// THIS IS THE WHOLE INERTNESS CLAIM and it is made three ways: the start hash,
/// the hash after twenty years of compounding, and the absence of the two new
/// keys from the serialized world. The two hashes are the ones lib.rs's pins
/// already produce, so a single ulp anywhere in `economy::charge`, in the
/// fiscal block's closed-books arm, or in the five call sites that now route
/// through the helper would show up here.
///
/// RED CHECKS, all run 2026-09-02 on this tree, each mutation reverted after:
///   1. `charge`'s closed arm recomputes the ratio from the dollars —
///      `n.debt_gdp + bn / n.gdp.max(0.1)` instead of `n.debt_gdp + share`,
///      which is the obvious simplification and the reason `charge` takes two
///      arguments at all. RED on the 240-month arm, actual 0x1099ddeb26822d83
///      against 0x20c24ab0f1581807.
///   2. `PACT_UPKEEP` 0.003 -> 0.0035, a money leg changing size. RED, actual
///      0xd029d0eb78818491.
///   3. The covert service charge 0.0008 -> 0.00081, one part in eight
///      hundred at a leg that fires a few times a decade. RED, actual
///      0x28b0eccfa538e645.
///
/// AND THE MEASURED LIMIT OF THIS BAR, recorded rather than assumed (iron rule
/// 7's power half). Changing the closed arm's floor from `.max(0.0)` to
/// `.max(1e-18)` left this test GREEN, because no nation's debt ratio ever
/// comes within 1e-18 of zero, so the two floors are the same function on
/// every input the model produces. That is the bar being right, not blind —
/// but it is worth knowing that it sees a changed VALUE and not a changed
/// clamp that never binds.
#[test]
fn the_treasury_is_inert_while_the_books_are_closed() {
    let mut w = world_1990(GameRules::default());

    let text = save(&w);
    for key in ["\"treasury_bn\"", "\"debt_bn\""] {
        assert!(!text.contains(key), "{key} appears in the 1990 save");
    }
    for n in w.nations.iter() {
        assert!(!n.on_the_books(), "{:?} starts with its books open", n.id);
        assert_eq!(n.net_position_bn(), None);
        assert_eq!(interest_gdp(n), 0.0, "{:?} is charged interest at t=0", n.id);
    }
    let start = state_hash(&w);
    assert_eq!(
        start, START_ACTUAL,
        "the 1990 start state moved (actual {start:#018x})"
    );

    for _ in 0..(12 * 20) {
        tick_month(&mut w, &[]);
    }
    let end = state_hash(&w);
    assert_eq!(
        end, RUN_ACTUAL,
        "the twenty-year timeline moved (actual {end:#018x})"
    );

    // And after 240 months of pacts, aid, covert operations, patronage and
    // contract deliveries, still not one nation has opened its books and not
    // one byte of the two new fields has reached the save.
    let text = save(&w);
    for key in ["\"treasury_bn\"", "\"debt_bn\""] {
        assert!(!text.contains(key), "{key} appears after 240 months");
    }
    assert!(w.nations.iter().all(|n| !n.on_the_books()));
}

// ---------------------------------------------------------------------------
// 2. THE ESCALATING RATE (Ridge's amendment)
// ---------------------------------------------------------------------------

/// The effective rate rises with the debt ratio over the whole range a nation
/// can reach, and is bounded above by the policy real rate plus the cap. An
/// INVARIANT over a dense sweep of 2,000 points, not a sample.
///
/// THE CURVE HAS THREE SEGMENTS and the bar asserts each one for what it is,
/// because the design asks for two things that cannot both be true everywhere
/// — "strictly increasing" AND "bounded by the cap". A function on a bounded
/// range cannot be strictly increasing forever.
///   - BELOW THE KNEE the spread is exactly zero and the rate is FLAT. That is
///     the half of the design that makes a nation at the 1990 median pay its
///     own policy rate, so asserting a rise here would assert the opposite of
///     what was approved.
///   - BETWEEN KNEE AND CAP it is STRICTLY increasing, which is the amendment.
///   - AT AND ABOVE THE CAP it is flat again, which is the bound, and is the
///     thing that stops the interest-feeds-debt recursion running away.
/// The two breakpoints are found by sweeping the function rather than by
/// copying its literals, so moving a constant moves the segments with it and
/// this test still asserts the shape rather than the numbers.
///
/// RED CHECKS, both run 2026-09-02, both reverted:
///   - `SPREAD_SLOPE` 0.06 -> 0.0, the spread stopping. RED: "the cap is 0" —
///     with no slope the curve has no sloped segment at all and the swept cap
///     collapses to nothing, which is the shape assertion doing its job before
///     the step-by-step sweep is even reached.
///   - `.min(SPREAD_CAP)` deleted, the bound removed. RED: "the cap is
///     59.964" — the swept cap at a debt ratio of 1000 is now unbounded, which
///     is exactly the runaway the constant exists to stop.
#[test]
fn the_effective_rate_rises_with_debt_and_is_bounded() {
    const POLICY: f64 = 0.05;
    const INFLATION: f64 = 0.03;
    let real = POLICY - INFLATION;
    // The cap, read off the function itself at a debt ratio far past where it
    // binds, so this test does not keep its own copy of the literal.
    let cap = effective_interest_rate(POLICY, INFLATION, 1000.0) - real;
    assert!(cap > 0.0 && cap < 1.0, "the cap is {cap}");

    // The two breakpoints, swept off the function to a thousandth of a point of
    // debt: the last ratio still paying exactly the policy rate, and the first
    // ratio already paying the full cap.
    let step = 0.001_f64;
    let mut knee = 0.0_f64;
    let mut capped_at = f64::INFINITY;
    for i in 0..=3000 {
        let d = i as f64 * step;
        let r = effective_interest_rate(POLICY, INFLATION, d);
        if r <= real {
            knee = d;
        }
        if r >= real + cap && capped_at.is_infinite() {
            capped_at = d;
        }
    }
    assert!(knee > 0.0 && knee < 1.0, "the knee is {knee}");
    assert!(capped_at > knee && capped_at.is_finite(), "the cap binds at {capped_at}");
    // MEASURED on this tree: knee 0.600, cap binds at 1.600, cap 0.0600.
    // The sloped segment is a full point of debt wide, which is the whole range
    // any borrower the shipped board produces actually sits in.
    assert!(
        capped_at - knee > 0.5,
        "the sloped segment is only {:.3} of debt wide",
        capped_at - knee
    );

    let mut d = 0.0_f64;
    let mut prev = effective_interest_rate(POLICY, INFLATION, d);
    for i in 1..=3000 {
        let next_d = i as f64 * step;
        let next = effective_interest_rate(POLICY, INFLATION, next_d);
        if next_d <= knee {
            assert_eq!(
                next.to_bits(),
                real.to_bits(),
                "below the knee the rate is not the policy real rate at {next_d:.3}"
            );
        } else if d >= knee && next_d <= capped_at {
            assert!(
                next > prev,
                "the rate is flat from {d:.3} to {next_d:.3}: {prev} then {next}"
            );
        } else if d >= capped_at {
            assert_eq!(
                next.to_bits(),
                prev.to_bits(),
                "the rate is still moving at {next_d:.3}, past the cap at {capped_at:.3}"
            );
        }
        assert!(
            next <= real + cap + 1e-15,
            "rate {next:.4} at debt {next_d:.2} exceeds real {real:.4} + cap {cap:.4}"
        );
        assert!(next >= real, "rate {next:.4} at debt {next_d:.2} is below the policy real rate");
        prev = next;
        d = next_d;
    }

    // The real floor, which is a bound and not a calibration: a policy rate far
    // below inflation cannot make debt free money.
    assert!(
        effective_interest_rate(0.0, 5.0, 0.0) >= -0.02,
        "the real floor does not hold under hyperinflation"
    );
}

/// The four figures the design asks for, at a 5% policy rate against 3%
/// inflation — a 2.00% real rate, so the whole of the difference is spread.
///
/// MEASURED by this test on this tree:
///     debt  30% of GDP -> 2.0000%/yr   (+0.00pp)
///     debt  60% of GDP -> 2.0000%/yr   (+0.00pp)
///     debt  90% of GDP -> 3.8000%/yr   (+1.80pp)
///     debt 150% of GDP -> 7.4000%/yr   (+5.40pp)
/// and the roster's own median 1990 debt ratio, 0.52 across all 137 files in
/// `data/nations/`, pays 2.0000%/yr — the policy rate exactly, which is the
/// design's requirement that the median nation pay close to it.
///
/// RED CHECKS, both run 2026-09-02, both reverted:
///   - `SPREAD_KNEE` 0.60 -> 0.40, the knee moving off the 1990 median. RED:
///     "the 1990 median borrower pays 0.02720, not the policy real rate
///     0.02000".
///   - `SPREAD_SLOPE` 0.06 -> 0.0, the escalation removed. RED: "90%:
///     0.020000000000000004" — a nation at 90% of output paying the same as
///     one at 30%, which is the amendment being undone.
#[test]
fn the_median_1990_borrower_pays_the_policy_rate_and_a_debtor_pays_more() {
    const POLICY: f64 = 0.05;
    const INFLATION: f64 = 0.03;
    let real = POLICY - INFLATION;

    // Measured across every file in spheres-sim/data/nations/ on 2026-09-02:
    // median 0.52, mean 0.642, min 0.00 (Brunei), max 3.80 (Nicaragua).
    const MEDIAN_1990_DEBT_GDP: f64 = 0.52;
    let at_median = effective_interest_rate(POLICY, INFLATION, MEDIAN_1990_DEBT_GDP);
    assert_eq!(
        at_median.to_bits(),
        real.to_bits(),
        "the 1990 median borrower pays {at_median:.5}, not the policy real rate {real:.5}"
    );

    let at = |d: f64| effective_interest_rate(POLICY, INFLATION, d);
    assert!((at(0.30) - 0.0200).abs() < 1e-12, "30%: {}", at(0.30));
    assert!((at(0.60) - 0.0200).abs() < 1e-12, "60%: {}", at(0.60));
    assert!((at(0.90) - 0.0380).abs() < 1e-12, "90%: {}", at(0.90));
    assert!((at(1.50) - 0.0740).abs() < 1e-12, "150%: {}", at(1.50));

    // "A heavily indebted nation visibly pays more" — at least a full
    // percentage point over the median borrower by 90% of output, which is
    // where politics.rs's own consolidation rule (debt_gdp > 0.85) has already
    // fired.
    assert!(
        at(0.90) - at_median >= 0.01,
        "a nation at 90% of output pays only {:.4}pp over the median",
        (at(0.90) - at_median) * 100.0
    );
    assert!(at(1.50) > at(0.90) && at(0.90) > at(0.60));
}

// ---------------------------------------------------------------------------
// 3. `charge` — exact on the closed arm, conserving on the open one
// ---------------------------------------------------------------------------

/// The five money legs still do, to the bit, what they did before the helper
/// existed, and each is driven through the surface a player actually uses
/// rather than by reaching into the function.
///
/// The expected value at each site is the pre-treasury line written out by
/// hand, and the comparison is `to_bits`, so a rounding is a failure.
///
/// RED CHECKS, both run 2026-09-02, both reverted:
///   - `PACT_UPKEEP` 0.003 -> 0.0035. RED: "USA did not pay the pact upkeep
///     exactly", 0x3FE0000... left 4603762310182998464 against right
///     4603761934883029516.
///   - The covert service charge 0.0008 -> 0.00081. RED: "the covert service
///     charge is no longer 0.0008 of output exactly", left 4603766978914612171
///     against right 4603766888842619624.
///
/// AND THE MEASURED LIMIT, recorded rather than assumed. Recomputing the ratio
/// from the dollars inside `charge` — the mutation
/// `the_treasury_is_inert_while_the_books_are_closed` catches — leaves THIS
/// test green, because every site here builds `bn` as `share * gdp` and
/// `(share * gdp) / gdp` happens to round back to `share` at these particular
/// magnitudes. The rounding is real and shows up over 240 months of compounding
/// (it moves the golden), not in a single call. So the two bars are not
/// redundant: this one sees a leg's SIZE change, the inertness bar sees a leg's
/// ROUNDING change, and neither sees the other's.
#[test]
fn the_closed_books_arm_is_the_arithmetic_that_shipped() {
    // -- the helper itself, at its contract: `share` and not `bn / gdp`.
    {
        let mut w = world_1990(GameRules::default());
        let before = w.nation(NationId::USA).debt_gdp;
        let share = 0.0008_f64;
        let bn = w.nation(NationId::USA).gdp * share;
        charge(&mut w, NationId::USA, bn, share);
        assert_eq!(
            w.nation(NationId::USA).debt_gdp.to_bits(),
            (before + share).to_bits(),
            "the closed arm did not write `debt_gdp + share`"
        );
        // A receipt is the same statement with the sign turned round, and it is
        // `a - b`, which IEEE 754 defines as `a + (-b)` exactly.
        let mid = w.nation(NationId::USA).debt_gdp;
        charge(&mut w, NationId::USA, -bn, -share);
        assert_eq!(
            w.nation(NationId::USA).debt_gdp.to_bits(),
            (mid - share).to_bits()
        );
    }

    // -- site 2: the pact upkeep, both signatories. `statecraft::tick` is
    //    called directly rather than through `tick_month`, so the fiscal block
    //    is not in the way and the assertion can be exact.
    {
        let mut w = world_1990(GameRules::default());
        let (a, b) = (NationId::USA, NationId::UK);
        spheres_sim::statecraft::propose_pact(&mut w, a, b).expect("the pact is signed");
        assert!(
            w.statecraft.pacts.iter().any(|p| (p.a, p.b) == (a, b) || (p.a, p.b) == (b, a)),
            "no pact stands to charge for"
        );
        let before = [w.nation(a).debt_gdp, w.nation(b).debt_gdp];
        spheres_sim::statecraft::tick(&mut w);
        // PACT_UPKEEP, statecraft.rs, is 0.003 of output a year and is private.
        // A guarantee costs both signatories the same SHARE of their own
        // output, which is the claim the pre-treasury line made and the one a
        // recomputation from dollars would round away.
        const PACT_UPKEEP_MONTHLY: f64 = 0.003 / 12.0;
        for (id, b0) in [a, b].iter().zip(before.iter()) {
            assert_eq!(
                w.nation(*id).debt_gdp.to_bits(),
                (b0 + PACT_UPKEEP_MONTHLY).to_bits(),
                "{id:?} did not pay the pact upkeep exactly"
            );
        }
    }

    // -- site 3: the aid leg. Economic aid costs the patron `share_gdp / 12`
    //    of its output a month, and arms half of that. Same direct call.
    {
        for (kind, factor) in [(AidKind::Economic, 1.0_f64), (AidKind::Arms, 0.5_f64)] {
            let mut w = world_1990(GameRules::default());
            let (patron, client) = (NationId::USA, NationId::Israel);
            let share = 0.004_f64;
            spheres_sim::statecraft::pledge_aid(&mut w, patron, client, kind, share)
                .expect("the pledge is made");
            let before = w.nation(patron).debt_gdp;
            spheres_sim::statecraft::tick(&mut w);
            assert_eq!(
                w.nation(patron).debt_gdp.to_bits(),
                (before + share * factor / 12.0).to_bits(),
                "the {kind:?} aid leg is no longer the share it was"
            );
        }
    }

    // -- site 4: the covert service charge, 0.0008 of output, exactly.
    {
        let mut w = world_1990(GameRules::default());
        let before = w.nation(NationId::USA).debt_gdp;
        spheres_sim::statecraft::covert_action(
            &mut w,
            NationId::USA,
            NationId::Iraq,
            spheres_sim::world::CovertOp::FundOpposition,
        )
        .expect("the operation is authorised");
        assert_eq!(
            w.nation(NationId::USA).debt_gdp.to_bits(),
            (before + 0.0008_f64).to_bits(),
            "the covert service charge is no longer 0.0008 of output exactly"
        );
    }

    // -- site 5: the patronage envelope, 0.008 of output, exactly.
    {
        let mut w = world_1990(GameRules::default());
        let id = NationId::Iraq;
        spheres_sim::government::ensure(&mut w, id);
        let before = w.nation(id).debt_gdp;
        let cmd = Command::SecurePillar { nation: id, pillar: spheres_sim::government::Pillar::Army };
        spheres_sim::apply_command(&mut w, &cmd).expect("the pillar is bought");
        assert_eq!(
            w.nation(id).debt_gdp.to_bits(),
            (before + 0.008_f64).to_bits(),
            "the patronage envelope is no longer 0.008 of output exactly"
        );
    }
}

/// On the open arm a money leg between two differently-sized economies moves
/// the SAME dollars on both sides, so nothing is created and nothing destroyed.
///
/// This is the defect the design names. Before the helper, `settle` divided the
/// leg by the payer's output going out and by the payee's coming in: with the
/// two stocks that is a leak, because the payer's dollar debt and the payee's
/// do not move by the same money. Asserted here directly, on a pair chosen for
/// how far apart they are — the United States at $5,980bn against Kuwait at
/// $18bn, a factor of 332.
///
/// RED CHECK, run 2026-09-02, reverted. `pay`'s inflow arm was changed to
/// `(treasury, (debt - inflow).max(0.0))` — the receipt floored at zero the way
/// the pre-treasury payee line floored it. RED: "the leg leaked $8.200bn of
/// 10.000", which is Kuwait's whole debt stock of $1.8bn subtracted from a
/// $10bn receipt and the remainder evaporating.
///
/// AND A NEGATIVE RESULT WORTH RECORDING, because it corrects the design's own
/// account of the defect. Re-inflating the ratio by the charged nation's own
/// output — `pay(treasury, debt, share * n.gdp)` — leaves this test GREEN, and
/// it leaves the arithmetic right: `settle` divides by the payer's output going
/// out and the payee's coming in, and multiplying each side back by its own
/// output returns the same `bn` to both. The leak was never the two divisors.
/// It is the FLOOR, which destroys any receipt larger than the payee's debt,
/// and that is what this test is pointed at.
#[test]
fn a_money_leg_between_unequal_economies_conserves() {
    let mut w = world_1990(GameRules::default());
    let (payer, payee) = (NationId::USA, NationId::Kuwait);

    // Open both sets of books the way the game does: the player's own budget
    // command. Enacting the inherited plan unchanged is a no-op on every
    // aggregate, so the only thing this does is seat the two stocks.
    for id in [payer, payee] {
        let year = w.year;
        let plan = w.nation(id).budget_for(year);
        spheres_sim::apply_command(
            &mut w,
            &Command::SetAnnualBudget {
                nation: id,
                fiscal_year: year,
                allocations: plan.allocations,
            },
        )
        .expect("the budget is enacted");
        assert!(w.nation(id).on_the_books(), "{id:?} did not open its books");
    }

    let before: Vec<f64> = [payer, payee]
        .iter()
        .map(|id| w.nation(*id).net_position_bn().unwrap())
        .collect();
    let world_before: f64 = before.iter().sum();

    // $10bn, moved once, in the two halves `settle` moves it in.
    const BN: f64 = 10.0;
    let g = w.nation(payer).gdp.max(0.1);
    charge(&mut w, payer, BN, BN / g);
    let g = w.nation(payee).gdp.max(0.1);
    charge(&mut w, payee, -BN, -(BN / g));

    let after: Vec<f64> = [payer, payee]
        .iter()
        .map(|id| w.nation(*id).net_position_bn().unwrap())
        .collect();
    let world_after: f64 = after.iter().sum();

    assert!(
        (world_after - world_before).abs() < 1e-9,
        "the leg leaked ${:.3}bn of {BN:.3}",
        (world_after - world_before).abs()
    );
    assert!(
        ((before[0] - after[0]) - BN).abs() < 1e-9,
        "the payer parted with ${:.3}bn, not {BN:.3}",
        before[0] - after[0]
    );
    assert!(
        ((after[1] - before[1]) - BN).abs() < 1e-9,
        "the payee received ${:.3}bn, not {BN:.3}",
        after[1] - before[1]
    );
}

// ---------------------------------------------------------------------------
// 4. A NET CREDITOR
// ---------------------------------------------------------------------------

/// A state that has retired its debt and piled up cash reads as UN-DESPERATE,
/// not as panicking.
///
/// The risk the design's audit names. `dyads.rs` derives war appetite from
/// `(debt_gdp - 0.6).max(0.0) * 1.5`, and it was calibrated against a ratio
/// that was floored at zero. The treasury has to leave that reading intact:
/// `pay` retires debt before it accumulates cash, so `debt_bn` cannot go
/// negative, `debt_gdp` cannot either, and a creditor sits at the bottom of the
/// term rather than off the end of it. Asserted rather than assumed, which is
/// what the design asks for.
///
/// This also pins the browser's floor, which the same audit flags as
/// load-bearing: `a_nation_with_no_debt_is_not_shown_paying_it_down` in
/// spheres-web relies on `debt_gdp >= 0.0` for every nation every month, and it
/// still holds for a nation on the books.
///
/// RED CHECK, run 2026-09-02, reverted. `pay`'s inflow arm was changed to
/// accumulate everything in the till without retiring debt —
/// `(treasury + inflow, debt)`, which is the design sentence "the balance flows
/// to treasury_bn" read literally and is why that reading was rejected. RED:
/// "debt was not retired first", left `Some(3707.6)` against right `Some(0.0)`
/// — the United States still owing $3,707.6bn after paying in twice what it
/// owed, and therefore still reading as fiscally desperate to `dyads.rs`.
#[test]
fn a_net_creditor_reads_as_un_desperate() {
    let mut w = world_1990(GameRules::default());
    let id = NationId::USA;
    let year = w.year;
    let plan = w.nation(id).budget_for(year);
    spheres_sim::apply_command(
        &mut w,
        &Command::SetAnnualBudget { nation: id, fiscal_year: year, allocations: plan.allocations },
    )
    .expect("the budget is enacted");

    let debt0 = w.nation(id).debt_bn.expect("the books are open");
    assert!(debt0 > 0.0, "the United States starts owing money");
    let ratio0 = w.nation(id).debt_gdp;
    let appetite = |d: f64| (d - 0.6).max(0.0) * 1.5; // dyads.rs, verbatim
    assert!(appetite(ratio0) > 0.0, "the pre-condition is a nation the term can see");

    // Pay in twice what it owes, as one receipt.
    charge(&mut w, id, -(debt0 * 2.0), -(ratio0 * 2.0));

    let n = w.nation(id);
    assert_eq!(n.debt_bn, Some(0.0), "debt was not retired first");
    assert!(n.debt_gdp >= 0.0, "the debt ratio went negative: {}", n.debt_gdp);
    assert_eq!(n.debt_gdp, 0.0, "the ratio is not the retired stock: {}", n.debt_gdp);
    assert!(
        n.net_position_bn().unwrap() > 0.0,
        "a net creditor is not representable: {:?}",
        n.net_position_bn()
    );
    assert_eq!(
        appetite(n.debt_gdp),
        0.0,
        "a net creditor still reads as fiscally desperate"
    );
    assert_eq!(interest_gdp(n), 0.0, "a nation owing nothing is still charged interest");

    // And it stays that way through a year of ordinary ticking: the ratio is
    // the quotient of two stocks and neither the browser's floor nor the war
    // term ever sees a negative.
    for _ in 0..12 {
        tick_month(&mut w, &[]);
        for n in w.nations.iter().filter(|n| n.alive) {
            assert!(n.debt_gdp >= 0.0, "{:?} holds negative debt {}", n.id, n.debt_gdp);
        }
    }
}

// ---------------------------------------------------------------------------
// 7. DEBT IS CHARGED ONCE
// ---------------------------------------------------------------------------

/// A nation pays for its debt through EXACTLY ONE channel, and which one
/// depends on whether it keeps books.
///
/// `growth_terms`' inherited `debt_drag` — `(debt_gdp - 0.9) * 0.02` subtracted
/// from growth — was calibrated when the model charged no interest at all, so
/// it stood in for debt service, crowding-out and the risk premium together.
/// The treasury then added a second, independent charge for the same debt: cash
/// out of the till at the escalating `effective_interest_rate`. `SPREAD_KNEE`'s
/// own derivation reasons the 0.60 knee against `dyads.rs`'s war-appetite line
/// and `politics.rs`'s 0.85 consolidation trigger and never mentions the drag,
/// so the spread was sized as the market's WHOLE charge for a ratio the model
/// was already charging for elsewhere. A nation that opened its books paid both.
///
/// MEASURED on this tree, Brazil seed 1990 with debt_gdp forced to 1.20 (policy
/// rate 2.90 and inflation 2.95, the transcribed 1990 hyperinflation figures):
/// effective_rate 0.016000, cash `interest_gdp` 0.019200 a year, `debt_drag`
/// 0.006000 of growth. Before the gate the open arm paid 0.025200 against the
/// 0.006000 the identical nation pays with its books closed, 4.20x apart at the
/// same ratio — a double charge and a player-versus-AI asymmetry in one.
///
/// AN INVARIANT, so iron rule 7's sampling arm does not apply: the claim is
/// universal over the roster and over the debt range, and the sweep below is a
/// budget question rather than a correctness one.
///
/// RED CHECK, run 2026-09-02 and reverted: the `n.debt_bn.is_none() &&` gate
/// removed from `economy.rs`'s `debt_drag`, which is the tree exactly as it
/// stood. RED: "Brazil on the books pays a growth drag as well as cash
/// interest", left 4573567551181324024 against right 0 — the bit pattern of
/// 0.006 against the bit pattern of zero.
#[test]
fn debt_is_charged_once_and_not_twice() {
    let drag_of = |w: &spheres_sim::world::WorldState, id: NationId| {
        let n = w.nation(id);
        let c = Conditions::of(w, id);
        growth_terms(n, n.state_invest_gdp, n.interest_rate, &c).debt
    };

    // The anchor: one nation, one ratio, both sets of books.
    let mut open = world_1990(GameRules { seed: 1990, ..GameRules::default() });
    open.player = Some(NationId::Brazil);
    open.nation_mut(NationId::Brazil).political_capital = 100.0;
    open.nation_mut(NationId::Brazil).debt_gdp = 1.20;
    let closed = open.clone();
    let year = open.year;
    let allocations = open.nation(NationId::Brazil).budget_for(year).allocations;
    spheres_sim::apply_command(
        &mut open,
        &Command::SetAnnualBudget { nation: NationId::Brazil, fiscal_year: year, allocations },
    )
    .expect("the books open");

    let n = open.nation(NationId::Brazil);
    assert!(n.on_the_books(), "the probe did not open the books");
    let rate = effective_interest_rate(n.interest_rate, n.inflation, n.debt_gdp);
    assert!(
        (rate - 0.016000).abs() < 1e-9,
        "the escalating rate at 1.20 of GDP is {rate:.6}, not the measured 0.016000"
    );
    assert!(
        (interest_gdp(n) - 0.019200).abs() < 1e-9,
        "cash debt service is {:.6} of GDP, not the measured 0.019200",
        interest_gdp(n)
    );
    assert_eq!(
        drag_of(&open, NationId::Brazil).to_bits(),
        0.0f64.to_bits(),
        "Brazil on the books pays a growth drag as well as cash interest"
    );

    // ...and the closed-books nation is UNTOUCHED: the drag it always paid, and
    // no cash charge, which is the inertness half of the same claim.
    assert!(
        (drag_of(&closed, NationId::Brazil) - 0.006000).abs() < 1e-9,
        "the closed-books drag moved: {:.6} rather than the incumbent 0.006000",
        drag_of(&closed, NationId::Brazil)
    );
    assert_eq!(interest_gdp(closed.nation(NationId::Brazil)).to_bits(), 0.0f64.to_bits());

    // The invariant, over the whole roster and the whole range the drag can
    // see: no nation ever carries both charges at once.
    let mut both = Vec::new();
    let mut drag_seen = 0u32;
    let mut cash_seen = 0u32;
    for ratio in [0.0, 0.5, 0.85, 0.9, 1.0, 1.5, 2.0, 3.8] {
        for books_open in [false, true] {
            let mut w = world_1990(GameRules { seed: 1990, ..GameRules::default() });
            let ids: Vec<NationId> = w.nations.iter().filter(|n| n.alive).map(|n| n.id).collect();
            for id in &ids {
                w.nation_mut(*id).debt_gdp = ratio;
            }
            if books_open {
                for id in &ids {
                    w.nation_mut(*id).political_capital = 100.0;
                    let allocations = w.nation(*id).budget_for(year).allocations;
                    spheres_sim::apply_command(
                        &mut w,
                        &Command::SetAnnualBudget {
                            nation: *id,
                            fiscal_year: year,
                            allocations,
                        },
                    )
                    .expect("the books open");
                }
            }
            for id in &ids {
                let drag = drag_of(&w, *id);
                let cash = interest_gdp(w.nation(*id));
                if drag > 0.0 {
                    drag_seen += 1;
                }
                if cash > 0.0 {
                    cash_seen += 1;
                }
                if drag > 0.0 && cash > 0.0 {
                    both.push((*id, ratio, drag, cash));
                }
            }
        }
    }
    assert!(
        both.is_empty(),
        "{} nation-ratios pay for the same debt twice, e.g. {:?}",
        both.len(),
        both.first()
    );
    // POWER, recorded rather than assumed: the sweep has to be able to SEE both
    // charges, or an empty `both` would be proof of nothing.
    assert!(drag_seen > 0 && cash_seen > 0, "the sweep saw drag {drag_seen}, cash {cash_seen}");
    println!("no double charge in {drag_seen} drag readings and {cash_seen} cash readings");
}
