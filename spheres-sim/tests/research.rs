//! THE RESEARCH DECOMPOSITION — the seven named arms, and the eight shares the
//! government is now allowed to write down.
//!
//! Stage 3 of the ministry economy (design approved by Ridge 2026-09-02). Two
//! things landed and this file is the bar on both.
//!
//! THE INERTNESS CONTRACT is the whole risk and
//! `the_decomposition_is_the_old_scalar_bit_for_bit` is the bar on it.
//! `research_output` was a sequential chain of `out *= ...` with three arms
//! inside `if` branches that skipped the multiply entirely; it is now
//! `ResearchTerms::total()`, seven unconditional multiplies with the dormant
//! arms stored as `1.0`. Floating multiplication is not associative, so this is
//! a claim that has to be MEASURED and not argued: the old chain is carried in
//! this file, verbatim, as `old_scalar`, and the two are compared with `to_bits`
//! — no tolerance, no epsilon — for every living nation in every one of 240
//! months. Not at t=0, where every arm is dormant and the bar would be a
//! tautology.
//!
//! The coverage assertions at the end of that test are what stop it from being
//! one anyway. A board where no nation is ever at war, sanctioned, unstable,
//! command-run or running an education budget would pass a chain of seven ones,
//! so the test counts how many nation-months exercised each arm and fails if any
//! arm was never charged.
//!
//! THE SECOND HALF is `Command::SetResearchAllocation`. `None` is the shipped
//! world and must execute no new arithmetic; `Some` must be what the sim
//! actually spends, not merely what the screen prints. Both are bars here.
//!
//! Every test carries the RED CHECK that was run against it: the mutation made
//! to the tree, and what the test then said. Iron rules 5 and 6.

use spheres_sim::init::world_1990;
use spheres_sim::world::*;
use spheres_sim::{apply_command, price_of, tech, tick_month, Command};

/// What a command costs its government in political capital.
fn cost(w: &WorldState, c: &Command) -> f64 {
    price_of(w, c).expect("the command is priced")
}

/// The nation the allocation probes are run on: a real research budget, eight
/// domains with something left to learn in each, and not so close to the
/// frontier that a domain runs out of eligible projects mid-probe.
const ME: NationId = NationId::Brazil;

// ---------------------------------------------------------------------------
// The old scalar, carried verbatim
// ---------------------------------------------------------------------------

/// `research_output` EXACTLY AS IT STOOD before the decomposition, down to the
/// order of the multiplies and the placement of the three `if` branches.
///
/// Copied out of the tree at 9274baa rather than paraphrased. A paraphrase that
/// happened to regroup two factors would make this bar agree with the new code
/// for the wrong reason, which is the one failure mode a bit-for-bit oracle has.
/// The only edits are the ones the module boundary forces: the `crate::world::`
/// prefix is dropped, and the private `development` is not called because `dev`
/// arrives as an argument in both.
fn old_scalar(w: &WorldState, n: &Nation, dev: f64) -> f64 {
    let invest = n.state_invest_gdp + n.priv_invest_gdp;
    let intensity = (0.008 + 0.017 * dev) * (0.55 + 1.5 * invest);
    let mut out = n.gdp * intensity / 12.0;

    let ministry_multiplier = 1.0 + n.budget_gap(BUDGET_EDUCATION) * 15.0;
    out *= ministry_multiplier.clamp(0.35, 2.25);

    out *= 1.0 + n.tech.bonus.research_rate_eff();

    if n.system == EconomySystem::Command {
        out *= 0.80;
    }
    if n.stability < 40.0 {
        out *= 0.60 + n.stability / 100.0;
    }
    if w.at_war(n.id) {
        out *= 0.85;
    }
    out *= 1.0 - 0.10 * w.sanction_weight(n.id);
    out.max(0.0)
}

/// The development proxy every caller of `research_output` outside the module
/// already builds this way — `tests/ministries.rs` and the browser both.
fn dev_of(n: &Nation) -> f64 {
    (n.gdp * 1000.0 / n.population / 24000.0).min(1.0)
}

/// Open a nation's books on the plan it already runs, then move EDUCATION by
/// `delta`. Enacting the inherited budget is a no-op on all three aggregates —
/// `enacting_the_inherited_budget_unchanged_is_a_no_op` is the shipped proof —
/// so the only thing this changes is `budget_gap(BUDGET_EDUCATION)`, which is
/// the only thing the ministry arm reads.
fn fund_education(w: &mut WorldState, id: NationId, delta: f64) {
    let year = w.year;
    let mut allocations = w.nation(id).budget_for(year).allocations;
    allocations[BUDGET_EDUCATION] =
        (allocations[BUDGET_EDUCATION] + delta).clamp(0.0, BUDGET_CAPS[BUDGET_EDUCATION]);
    apply_command(w, &Command::SetAnnualBudget { nation: id, fiscal_year: year, allocations })
        .expect("the budget is enacted");
}

// ---------------------------------------------------------------------------
// 1. The contract
// ---------------------------------------------------------------------------

/// THE BAR THIS STAGE EXISTS TO PASS. `ResearchTerms::total()` is the old chain
/// and not merely close to it, on every nation, for twenty years.
///
/// RED CHECK, run: the last two multiplies in `ResearchTerms::total` were
/// swapped — `out *= self.sanctions;` before `out *= self.war;`, the same seven
/// factors in a different order, and the change a reader would call harmless.
/// The test went red at month 15 on Iran, 0.052751848188205594 against
/// 0.0527518481882056: bit patterns 4587763163558786678 and ...679, one unit in
/// the last place. Green at t=0, and green for fourteen months, which is why
/// this bar sweeps the board rather than sampling the start.
#[test]
fn the_decomposition_is_the_old_scalar_bit_for_bit() {
    let mut w = world_1990(GameRules::default());

    // Two governments with their books open and education moved in opposite
    // directions, so the ministry arm is exercised on BOTH sides of 1.0 rather
    // than sitting dormant on a board where nobody has enacted anything.
    w.nation_mut(ME).political_capital = 200.0;
    w.nation_mut(NationId::USA).political_capital = 200.0;
    fund_education(&mut w, ME, 0.02);
    fund_education(&mut w, NationId::USA, -0.02);

    let mut checked = 0u64;
    // One counter per arm, so "the bar was exercised" is a measurement.
    let (mut ministry, mut tools, mut system, mut disorder, mut war, mut sanctions) =
        (0u64, 0u64, 0u64, 0u64, 0u64, 0u64);

    for month in 0..240u32 {
        for i in 0..w.nations.len() {
            let n = &w.nations[i];
            if !n.alive {
                continue;
            }
            let dev = dev_of(n);
            let t = tech::research_terms(&w, n, dev);
            let want = old_scalar(&w, n, dev);
            assert_eq!(
                t.total().to_bits(),
                want.to_bits(),
                "month {month}, {}: the decomposition charged {} where the old \
                 chain charged {} — arms base {} ministry {} tools {} system {} \
                 disorder {} war {} sanctions {}",
                n.id.name(),
                t.total(),
                want,
                t.base,
                t.ministry,
                t.tools,
                t.system,
                t.disorder,
                t.war,
                t.sanctions
            );
            checked += 1;
            if t.ministry != 1.0 {
                ministry += 1;
            }
            if t.tools != 1.0 {
                tools += 1;
            }
            if t.system != 1.0 {
                system += 1;
            }
            if t.disorder != 1.0 {
                disorder += 1;
            }
            if t.war != 1.0 {
                war += 1;
            }
            if t.sanctions != 1.0 {
                sanctions += 1;
            }
        }
        tick_month(&mut w, &[]);
    }

    println!(
        "nation-months checked {checked}; arms charged: ministry {ministry}, \
         tools {tools}, system {system}, disorder {disorder}, war {war}, \
         sanctions {sanctions}"
    );
    // WITHOUT THESE THE TEST IS A TAUTOLOGY. Seven ones multiply to one in any
    // order, so a board that never charged an arm would pass a decomposition
    // that had got that arm wrong.
    // 240 months against the roster this board actually carries, successor
    // states included: 36,709 nation-months, measured. The bar sits below that
    // and above anything a board collapsed to a handful of survivors could give.
    assert!(checked > 30_000, "only {checked} nation-months were checked");
    assert!(ministry > 0, "the EDUCATION arm was never charged");
    assert!(tools > 0, "the tools arm was never charged");
    assert!(system > 0, "the command-economy arm was never charged");
    assert!(disorder > 0, "the disorder arm was never charged");
    assert!(war > 0, "the war arm was never charged");
    assert!(sanctions > 0, "the sanctions arm was never charged");
}

/// The dormant arms really are stored as exactly `1.0`, and not as something
/// that rounds to it. The bit-for-bit bar above rests on `x * 1.0 == x`, which
/// is a fact about the literal one and about nothing else.
///
/// RED CHECK, run: the war arm's dormant value was changed from `1.0` to
/// `0.999_999_999_999_999_9`, which prints as 1 and is not one. This test went
/// red on the United States, 4607182418800017407 against ...408, and
/// `the_decomposition_is_the_old_scalar_bit_for_bit` went red in month 0.
#[test]
fn a_dormant_arm_is_exactly_one() {
    let w = world_1990(GameRules::default());
    let mut seen = 0;
    for n in w.nations.iter().filter(|n| n.alive) {
        let t = tech::research_terms(&w, n, dev_of(n));
        // Nobody has enacted a budget on a fresh board and nobody is at war in
        // January 1990, so these two are dormant for every nation on it.
        assert_eq!(t.ministry.to_bits(), 1.0f64.to_bits(), "{}", n.id.name());
        assert_eq!(t.war.to_bits(), 1.0f64.to_bits(), "{}", n.id.name());
        seen += 1;
    }
    assert!(seen > 100, "only {seen} nations were on the board");
}

// ---------------------------------------------------------------------------
// 2. The allocation
// ---------------------------------------------------------------------------

/// `None` is the shipped world. Setting an allocation and standing it down
/// again must return the nation to the very same eight numbers it had — not
/// nearly, exactly — because `None` is the path every nation on a default board
/// is on, and the golden hashes are the claim that it did not move.
///
/// RED CHECK, run: the `None` arm of `Command::SetResearchAllocation`'s apply
/// was changed to leave `n.tech.allocation` standing instead of clearing it.
/// The test went red on Computing, reading a bit pattern of 0 — the ordered
/// share of 0.0 — against the read-off 4592314810234841091.
#[test]
fn an_allocation_of_none_reads_the_computed_weights() {
    let mut w = world_1990(GameRules::default());
    w.nation_mut(ME).political_capital = 200.0;
    let dev = dev_of(w.nation(ME));
    let before = tech::domain_weights_of(&w, w.nation(ME), dev);

    let mut ordered = [0.0f64; tech::DOMAIN_COUNT];
    ordered[tech::Domain::Aerospace.index()] = 3.0;
    ordered[tech::Domain::Agriculture.index()] = 1.0;
    apply_command(&mut w, &Command::SetResearchAllocation { nation: ME, weights: Some(ordered) })
        .expect("the allocation is enacted");

    // Taken as ratios and divided by their own sum, which is the one place the
    // division happens.
    let during = tech::domain_weights_of(&w, w.nation(ME), dev);
    assert_eq!(during[tech::Domain::Aerospace.index()], 0.75);
    assert_eq!(during[tech::Domain::Agriculture.index()], 0.25);
    assert_eq!(during[tech::Domain::Computing.index()], 0.0);
    assert_ne!(
        during[tech::Domain::Computing.index()],
        before[tech::Domain::Computing.index()]
    );

    apply_command(&mut w, &Command::SetResearchAllocation { nation: ME, weights: None })
        .expect("the allocation is stood down");
    let after = tech::domain_weights_of(&w, w.nation(ME), dev);
    for d in tech::DOMAINS {
        assert_eq!(
            after[d.index()].to_bits(),
            before[d.index()].to_bits(),
            "{} did not come back to the weight it was read off with",
            d.name()
        );
    }
    assert!(w.nation(ME).tech.allocation.is_none());
}

/// The shares the browser is served are the shares the spend loop charges.
///
/// Structurally they cannot differ — `domain_weights_of` and `tech::tick` call
/// one function — but "cannot" is what the growth model said about its
/// JavaScript mirror. This measures it: a nation ordered to put everything into
/// one domain banks research in that domain and in no other.
///
/// RED CHECK, run: the allocation branch was deleted from `domain_weights`, so
/// an enacted allocation reached neither the screen nor the spend loop. The test
/// went red on Computing — "was served a share it was not given" — and
/// `an_allocation_of_none_reads_the_computed_weights` went red with it.
#[test]
fn the_served_weights_are_the_spent_weights() {
    let mut w = world_1990(GameRules::default());
    w.nation_mut(ME).political_capital = 200.0;

    let mut ordered = [0.0f64; tech::DOMAIN_COUNT];
    ordered[tech::Domain::Materials.index()] = 1.0;
    apply_command(&mut w, &Command::SetResearchAllocation { nation: ME, weights: Some(ordered) })
        .expect("the allocation is enacted");

    let dev = dev_of(w.nation(ME));
    let served = tech::domain_weights_of(&w, w.nation(ME), dev);
    let output = tech::research_output(&w, w.nation(ME), dev);
    let banked_before = w.nation(ME).tech.research_total;
    let progress_before = w.nation(ME).tech.progress.clone();
    let known_before = w.nation(ME).tech.count();

    tech::tick(&mut w);

    let n = w.nation(ME);
    // The month's points were generated and banked whole.
    assert!(
        (n.tech.research_total - banked_before - output).abs() < 1e-9,
        "the month banked {} against an output of {output}",
        n.tech.research_total - banked_before
    );
    // And they all went to Materials. A domain served 0.0 must have received
    // 0.0: its bank can only fall, when a project already funded is fielded.
    for d in tech::DOMAINS {
        let di = d.index();
        if d == tech::Domain::Materials {
            continue;
        }
        assert_eq!(served[di], 0.0, "{} was served a share it was not given", d.name());
        assert!(
            n.tech.progress[di] <= progress_before[di],
            "{} banked research on a share of 0.0: {} -> {}",
            d.name(),
            progress_before[di],
            n.tech.progress[di]
        );
    }
    assert!(n.tech.count() >= known_before);
}

/// A government that puts its entire research budget into one domain does not
/// get to empty that domain's tree in a month. The spend loop fields at most six
/// technologies per domain per month — "several cheap adoptions can land in one
/// month, but invention never comes in floods" — and an allocation is a way of
/// moving money, not a way past that cap.
///
/// RED CHECK, run: the acquisition loop's `for _ in 0..6` was widened to
/// `for _ in 0..60`. The test went red in month 0 of the probe, 7 Materials
/// technologies fielded in a single month against the cap of 6 — which is also
/// the measurement that says this bar has power at all.
#[test]
fn a_max_concentration_allocation_does_not_flood_past_the_cap() {
    let mut w = world_1990(GameRules::default());
    w.nation_mut(ME).political_capital = 200.0;

    // MATERIALS, and the domain is chosen from a measurement rather than
    // arbitrarily. With the bank filled the acquisition loop is capped at six a
    // month, and a probe on a domain that never reaches six is a bar with no
    // power — the cap could be deleted and the test would still pass. Measured
    // across six nations and all eight domains with the bank filled, worst month
    // each: Aerospace 3, Computing 4, Transport 4, Energy 5, Biotech 5-6,
    // Communications 5-6, Materials 6, Agriculture 6-7. Only the last two press
    // the cap, and Materials presses it on every nation measured.
    let mut ordered = [0.0f64; tech::DOMAIN_COUNT];
    ordered[tech::Domain::Materials.index()] = 1.0;
    apply_command(&mut w, &Command::SetResearchAllocation { nation: ME, weights: Some(ordered) })
        .expect("the allocation is enacted");
    let reg = tech::registry();
    let held = |w: &WorldState| -> usize {
        w.nation(ME)
            .tech
            .known
            .iter()
            .filter(|t| reg[**t as usize].domain == tech::Domain::Materials)
            .count()
    };

    let mut worst = 0usize;
    let mut fielded_total = 0usize;
    for month in 0..120u32 {
        // THE MONEY IS TAKEN OUT OF THE QUESTION. An allocation only moves the
        // month's points around, and Brazil's whole month buys a fraction of one
        // Materials project — so a probe that only reallocated would measure
        // affordability, and would pass a cap that had been removed. The bank is
        // filled directly instead, which is the only way to put the acquisition
        // loop itself under the bar. Reached around the command queue on purpose
        // and only in the probe: no command can seat a bank like this, which is
        // exactly why the cap has to be measured rather than argued.
        w.nation_mut(ME).tech.progress[tech::Domain::Materials.index()] = 1.0e6;
        let before = held(&w);
        tick_month(&mut w, &[]);
        let gained = held(&w).saturating_sub(before);
        fielded_total += gained;
        worst = worst.max(gained);
        assert!(
            gained <= 6,
            "month {month}: {gained} Materials technologies fielded in one month, past the per-domain cap of 6"
        );
    }
    println!("worst month {worst}, {fielded_total} Materials technologies over 120 months");
    // THE POWER HALF OF THE BAR, iron rule 7. The probe has to have PRESSED the
    // cap, or the assertion above is decorative: with the loop capped at six and
    // the bank effectively infinite, a worst month of six is the cap binding and
    // anything less is some other constraint doing the work.
    assert!(worst >= 6, "the probe never pressed the cap — the worst month fielded only {worst}");
}

/// An allocation that cannot be normalised is refused at the gate, and refusing
/// it leaves the nation exactly as it was.
///
/// RED CHECK, run: the `sum <= 0.0` guard was deleted from the apply arm. Case 2
/// — eight zeroes — was then accepted and the test went red. Worth stating what
/// the sim did with it: `domain_weights`' own guard caught the unnormalisable
/// array and the nation went on spending the read-off weights, so nothing
/// divided by zero. The state was simply lying — `tech.allocation` was `Some`
/// while the screen showed shares nobody had ordered — which is the divergence
/// this stage exists to forbid.
#[test]
fn an_unnormalisable_allocation_is_refused() {
    let mut w = world_1990(GameRules::default());
    w.nation_mut(ME).political_capital = 200.0;

    let cases: [[f64; tech::DOMAIN_COUNT]; 3] = [
        [f64::NAN, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
        [-1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
        [0.0; tech::DOMAIN_COUNT],
    ];
    for (k, c) in cases.iter().enumerate() {
        let out =
            apply_command(&mut w, &Command::SetResearchAllocation { nation: ME, weights: Some(*c) });
        assert!(out.is_err(), "case {k} was accepted: {c:?}");
        assert!(w.nation(ME).tech.allocation.is_none(), "case {k} left an allocation standing");
    }
}

/// Writing the eight shares down costs what naming one domain costs. The
/// detailed form must not be the cheap way to do the expensive thing.
///
/// RED CHECK, run: the allocation was priced at 6.0, the focus-change price.
/// The test went red reading 6 against 30.
#[test]
fn an_allocation_is_priced_like_the_preset() {
    let w = world_1990(GameRules::default());
    let one = [0.125f64; tech::DOMAIN_COUNT];
    assert_eq!(
        cost(&w, &Command::SetResearchAllocation { nation: ME, weights: Some(one) }),
        cost(&w, &Command::SetResearchPriority { nation: ME, domain: Some(tech::Domain::Computing) })
    );
    assert_eq!(
        cost(&w, &Command::SetResearchAllocation { nation: ME, weights: None }),
        cost(&w, &Command::SetResearchPriority { nation: ME, domain: None })
    );
}

/// A default board writes no allocation to disk. That is what makes the golden
/// hashes evidence that this stage is inert rather than an assertion that it is.
///
/// RED CHECK, run: `skip_serializing_if` was dropped from the field, leaving
/// `#[serde(default)]`. The test went red: a default board wrote an allocation
/// to disk for every nation on it.
#[test]
fn the_default_board_serializes_no_allocation() {
    let w = world_1990(GameRules::default());
    let json = serde_json::to_string(&w).expect("the world serializes");
    assert!(!json.contains("\"allocation\""), "a default board wrote an allocation to disk");
}
