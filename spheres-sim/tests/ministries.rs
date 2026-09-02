//! THE MINISTRY COLLAPSE — one or two named arms per ministry, and the thirty
//! scattered addends gone.
//!
//! Stage 2 of the ministry economy (design approved by Ridge 2026-09-02). The
//! rule the whole thing is held to, in the design's own words: *no ministry gap
//! may reach `n.gdp` by more than one route, and no two ministries may write
//! the same arm.* This file is that rule as a test.
//!
//! HOW EVERY BAR HERE IS BUILT, because the construction is what makes it
//! exact. Two worlds are built from one seed. Both enact the INHERITED plan
//! through the real `Command::SetAnnualBudget`, which the shipped
//! `enacting_the_inherited_budget_unchanged_is_a_no_op` already proves is a
//! no-op on all three aggregates — so after that step the two worlds are
//! identical and both have their books open. Then ONE allocation is moved on
//! one of them, and only the allocation: `social_spend_gdp`,
//! `state_invest_gdp` and `mil_spend_gdp` are deliberately left alone. That
//! isolates the GAP CHANNEL, which is the only thing this stage touched, and it
//! is why "everything else is bit-identical" can be asserted with `to_bits`
//! rather than with a tolerance.
//!
//! ALMOST EVERYTHING HERE IS AN INVARIANT — a universal claim about one
//! deterministic pair of worlds, which iron rule 7 carves out of the sampling
//! rule because a small sample cannot make such a claim red falsely. The single
//! exception is `a_funded_foreign_service_catches_more_spies`, which reads a
//! rate across seeds and carries its own derived `n` beside the bar.
//!
//! Every test carries the RED CHECK that was actually run against it: the
//! mutation made to the tree, and what the test then said. Iron rules 5 and 6.

use spheres_sim::economy::{self, growth_terms, unemployment_rate, Conditions};
use spheres_sim::init::world_1990;
use spheres_sim::resources::{self, Commodity};
use spheres_sim::world::*;
use spheres_sim::{apply_command, politics, tech, war, Command};

/// The nation every probe is run on. Big enough to have a located non-oil
/// resource base (so infrastructure's arm has something to raise), a real
/// research budget, and a defence budget worth cutting.
const ME: NationId = NationId::Brazil;
/// Somebody to sanction it, so diplomacy's shield has a drag to shield against.
const THEM: NationId = NationId::USA;

/// The move every probe makes: half a point of GDP, which is the size the
/// design sizes its invented slopes against.
const DELTA: f64 = 0.005;

// ---------------------------------------------------------------------------
// The arm vector
// ---------------------------------------------------------------------------

/// Every named arm the design leaves standing, plus every aggregate it says a
/// ministry must NOT reach. One struct, read the same way for all ten
/// ministries, so that "and nothing else moved" is a claim about a closed list
/// rather than about whatever the test author remembered.
#[derive(Clone, Debug)]
struct Arms {
    /// Potential growth. NO MINISTRY MAY REACH THIS. Five did.
    potential: f64,
    /// The demand gap. NO MINISTRY MAY REACH THIS: it forks into output and
    /// into the price impulse, so a ministry here is charged twice. Three did.
    demand_gap: f64,
    /// The price impulse, for the same reason.
    target_inflation: f64,
    /// The month's growth before its noise draw — the aggregate the rule is
    /// ultimately about. Only diplomacy may move it, and only through the
    /// sanction shield.
    growth: f64,
    /// The sanction drag net of DIPLOMACY's shield.
    sanctions: f64,
    /// Unemployment in peace and in war. PENSIONS alone may move these.
    jobs_peace: f64,
    jobs_war: f64,
    /// Research points a month. EDUCATION alone may move this.
    research: f64,
    /// Absorptive capacity — how much of what the world knows this nation can
    /// take up. SCIENCE alone may move this.
    absorb: f64,
    /// What the whole unheld technology tree costs to reach. Science's arm read
    /// through its consequence rather than through the quantity it writes.
    tech_cost: f64,
    /// Force structure after a month of peace. HEALTH's arm is war-only, so
    /// NOTHING may move this.
    strength_peace: f64,
    /// Magazines after a month. INDUSTRY & ENERGY alone may move this.
    munitions: f64,
    /// Political capital after a month. PENSIONS alone may move this.
    standing: f64,
    /// Population after a month. HEALTH and HOUSING may move this.
    population: f64,
    /// Stability after a month. HOUSING, PENSIONS and SECURITY may move this.
    stability: f64,
    /// Separatist strain after a month. SECURITY alone may move this.
    separatism: f64,
    /// Annual non-oil extraction. INFRASTRUCTURE alone may move this.
    extraction: f64,
    /// Oil, which infrastructure is explicitly forbidden to touch.
    oil: f64,
}

impl Arms {
    fn fields(&self) -> [(&'static str, f64); 18] {
        [
            ("potential", self.potential),
            ("demand_gap", self.demand_gap),
            ("target_inflation", self.target_inflation),
            ("growth", self.growth),
            ("sanctions", self.sanctions),
            ("jobs_peace", self.jobs_peace),
            ("jobs_war", self.jobs_war),
            ("research", self.research),
            ("absorb", self.absorb),
            ("tech_cost", self.tech_cost),
            ("strength_peace", self.strength_peace),
            ("munitions", self.munitions),
            ("standing", self.standing),
            ("population", self.population),
            ("stability", self.stability),
            ("separatism", self.separatism),
            ("extraction", self.extraction),
            ("oil", self.oil),
        ]
    }

    /// The fields that differ, by name, `to_bits` exact.
    fn differences(&self, other: &Arms) -> Vec<&'static str> {
        let mine = self.fields();
        let theirs = other.fields();
        mine.iter()
            .zip(theirs.iter())
            .filter(|((_, a), (_, b))| a.to_bits() != b.to_bits())
            .map(|((name, _), _)| *name)
            .collect()
    }
}

// ---------------------------------------------------------------------------
// The two worlds
// ---------------------------------------------------------------------------

/// A 1990 world with the probe nation's books open, something to shield against
/// and something to suppress, and its magazines half empty so a refill is
/// visible against the 1.0 clamp.
fn staged(seed: u64) -> WorldState {
    let rules = GameRules { seed, ..GameRules::default() };
    let mut w = world_1990(rules);
    w.player = Some(ME);
    for id in [ME, THEM] {
        w.nation_mut(id).political_capital = 100.0;
    }
    // A live sanction, so `growth_drag_of_sanctions` is non-zero and diplomacy's
    // shield has something to bite on. Without one the shield multiplies zero
    // and the arm is invisible — which is a fact about the drag, not about the
    // ministry, and a bar that could not see the arm would be decorative.
    apply_command(&mut w, &Command::Sanction { imposer: THEM, target: ME })
        .expect("the sanction is imposed");
    {
        let n = w.nation_mut(ME);
        // Half-empty magazines: `munitions` clamps at 1.0 and every nation
        // starts full, so a faster refill is otherwise unobservable.
        n.munitions = 0.5;
        // A separatist movement that already exists, because SECURITY's
        // cohesion arm reads a positive gap against a live strain and must
        // never conjure one.
        n.separatism = 0.30;
        n.political_capital = 100.0;
    }
    // Open the books through the real command, with the plan the nation already
    // ran. A no-op on all three aggregates; it only seats the plan.
    let year = w.year;
    let allocations = w.nation(ME).budget_for(year).allocations;
    apply_command(
        &mut w,
        &Command::SetAnnualBudget { nation: ME, fiscal_year: year, allocations },
    )
    .expect("the inherited budget is enacted");
    w
}

/// The same world with exactly one allocation moved, and NOTHING else — the
/// three aggregates deliberately left where the inherited plan put them, so the
/// only thing that differs between the pair is `budget_gap(ministry)`.
fn gapped(w: &WorldState, ministry: usize, delta: f64) -> WorldState {
    let mut w2 = w.clone();
    let plan = w2.nation_mut(ME).annual_budget.as_mut().expect("the books are open");
    plan.allocations[ministry] += delta;
    assert!(
        plan.allocations[ministry] <= BUDGET_CAPS[ministry],
        "the probe moved ministry {ministry} past its own cap"
    );
    assert!(
        plan.allocations[ministry] >= 0.0,
        "the probe moved ministry {ministry} below zero"
    );
    w2
}

fn probe(w: &WorldState) -> Arms {
    let n = w.nation(ME);
    let c = Conditions::of(w, ME);
    let terms = growth_terms(n, n.state_invest_gdp, n.interest_rate, &c);
    let dev = (n.gdp * 1000.0 / n.population / 24000.0).min(1.0);

    // Each system is ticked on its own clone, so one arm's month cannot carry
    // another arm's month into the reading.
    let mut wq = w.clone();
    economy::tick(&mut wq);
    let after_economy = wq.nation(ME).clone();

    let mut ww = w.clone();
    war::tick(&mut ww);
    let after_war = ww.nation(ME).clone();

    let mut wp = w.clone();
    politics::tick(&mut wp);
    let after_politics = wp.nation(ME).clone();

    let mut wr = w.clone();
    resources::tick(&mut wr);
    let extraction: f64 = resources::ALL
        .iter()
        .filter(|c| **c != Commodity::Oil)
        .map(|c| resources::flow(&wr, ME, *c))
        .sum();

    Arms {
        potential: terms.potential,
        demand_gap: terms.demand_gap,
        target_inflation: terms.target_inflation,
        growth: terms.before_noise,
        sanctions: terms.sanctions,
        jobs_peace: unemployment_rate(n, false),
        jobs_war: unemployment_rate(n, true),
        research: tech::research_output(w, n, dev),
        absorb: tech::absorptive_capacity(w, n, dev),
        tech_cost: unheld_tree_cost(w),
        strength_peace: after_war.mil_strength,
        munitions: after_war.munitions,
        standing: after_politics.political_capital,
        population: after_economy.population,
        stability: after_economy.stability,
        separatism: after_economy.separatism,
        extraction,
        oil: resources::flow(&wr, ME, Commodity::Oil),
    }
}

/// What the whole unheld tree costs this nation, summed. A single technology
/// is the wrong probe for absorptive capacity in either direction: at the
/// frontier `effective_cost`'s `reach` term multiplies capacity by an adopter
/// share near zero, and on the ordinary tier the build floor binds and never
/// reads `absorb` at all — that function's own comment measures the floor
/// binding "for every nation examined from Equatorial Guinea to India from
/// month 120 onward". The sum is where the middle of the distribution shows up.
fn unheld_tree_cost(w: &WorldState) -> f64 {
    let n = w.nation(ME);
    (0..tech::registry().len() as u16)
        .filter(|t| !n.tech.knows_index(*t))
        .map(|t| tech::cost_of(w, ME, t))
        .sum()
}

/// The whole shape of a per-ministry bar: move one dial, and assert that the
/// set of arms that moved is EXACTLY the set named — no more, and no fewer.
fn moves_exactly(ministry: usize, delta: f64, expected: &[&str]) -> (Arms, Arms) {
    let base = staged(1990);
    let with = gapped(&base, ministry, delta);
    let (a, b) = (probe(&base), probe(&with));
    let mut moved = a.differences(&b);
    moved.sort_unstable();
    let mut want: Vec<&str> = expected.to_vec();
    want.sort_unstable();
    assert_eq!(moved, want, "ministry {ministry} moved the wrong set of arms");
    (a, b)
}

// ---------------------------------------------------------------------------
// 0. HEALTH
// ---------------------------------------------------------------------------

/// Health buys births in peace and NOTHING ELSE. Its unemployment 0.12,
/// potential 0.015, demand 0.06 and stability 8.0 addends are gone.
///
/// RED CHECK, run 2026-09-02 and reverted: `ds += budget_gap[BUDGET_HEALTH] *
/// 8.0` put back into the stability block — one of the thirty, restored. RED:
/// "ministry 0 moved the wrong set of arms", `["population", "stability"]`
/// against `["population"]`.
#[test]
fn health_buys_births_in_peace_and_nothing_else() {
    let (base, with) = moves_exactly(BUDGET_HEALTH, DELTA, &["population"]);
    // 0.030 a year on the population GROWTH RATE, applied for one month to the
    // stock the month opened with. Read as a difference of stocks rather than
    // as a ratio of them, because the ratio carries the incumbent growth rate
    // in its denominator and would need a tolerance to hide it.
    let start = staged(1990).nation(ME).population;
    let got = with.population - base.population;
    let want = start * DELTA * 0.030 / 12.0;
    assert!(
        (got - want).abs() < 1e-9,
        "health moved population by {got:.6} a month, not the {want:.6} its 0.030 slope buys"
    );
}

/// Open a shooting conflict on the probe nation, through real player commands.
fn open_war(w: &mut WorldState) {
    // The probe nation OPENS, against a neighbour in its own theatre. Staged
    // the other way round the command is refused for basing -- "no consenting
    // host within range of Latin America" -- which is the theatre model
    // working, not a problem with this bar.
    let (opener, target) = (ME, NationId::Colombia);
    let theatre = spheres_sim::theatre::TheatreId::LatinAmerica;
    for id in [opener, target] {
        w.nation_mut(id).political_capital = 100.0;
    }
    apply_command(w, &Command::OpenConflict { opener, target, theatre })
        .expect("the conflict opens");
    let id = w.conflict_between(opener, target).expect("just opened").id;
    for who in [opener, target] {
        w.nation_mut(who).political_capital = 100.0;
        apply_command(w, &Command::SetCeiling { conflict: id, nation: who, rung: 8 })
            .expect("the ceiling is set");
        w.nation_mut(who).political_capital = 100.0;
        apply_command(w, &Command::SetCommitment { conflict: id, nation: who, rung: 8 })
            .expect("the commitment is set");
    }
}

/// Health's SECOND arm is war-only, and the peacetime series is bit-identical.
///
/// RED CHECK, run 2026-09-02 and reverted: the `if !w.at_war(id) { return 1.0; }`
/// gate deleted from `war::health_retention`, so a health budget rebuilt an army
/// that was not fighting. RED: "health moved the force structure in peacetime",
/// left 4625162129559630705 against right 4625158660784643837 — the two bit
/// patterns of one month of peacetime replacement.
#[test]
fn health_returns_the_wounded_only_in_war() {
    // Peace. The probe above already pins this, and it is restated here because
    // this is the test that owns the claim.
    let peace = staged(1990);
    let peace_gap = gapped(&peace, BUDGET_HEALTH, DELTA);
    let (mut a, mut b) = (peace.clone(), peace_gap);
    war::tick(&mut a);
    war::tick(&mut b);
    assert_eq!(
        a.nation(ME).mil_strength.to_bits(),
        b.nation(ME).mil_strength.to_bits(),
        "health moved the force structure in peacetime"
    );

    // The gate itself, in peace: exactly 1.0 with the budget and without it,
    // which is what makes the peacetime series above bit-identical rather than
    // merely close.
    assert_eq!(war::health_retention(&peace, ME).to_bits(), 1.0f64.to_bits());
    assert_eq!(war::health_retention(&b, ME).to_bits(), 1.0f64.to_bits());

    // War: the same pair, shooting.
    let mut a = staged(1990);
    let mut b = gapped(&a, BUDGET_HEALTH, DELTA);
    for w in [&mut a, &mut b] {
        open_war(w);
        assert!(w.at_war(ME), "the staged conflict is not shooting");
    }
    // The SLOPE is read off the multiplier itself. It cannot be read off
    // `mil_strength` after the tick, because the same tick resolves the
    // fighting and a bigger army takes proportionally bigger casualties: a
    // 1.10x retention showed up as a 0.978x step in held force on this tree,
    // which is the model being right and the measurement being the wrong one.
    assert_eq!(
        war::health_retention(&a, ME).to_bits(),
        1.0f64.to_bits(),
        "an unfunded medical corps is not neutral in war"
    );
    let got = war::health_retention(&b, ME);
    // THE SLOPE IS 6.0, DERIVED, and it was the design's first-draft x20 until
    // the dial was measured. Health's reachable raise runs min 0.09575 / mean
    // 0.10112 / max 0.10725 across the 137 living 1990 nations, and at x20 the
    // 1.60 clamp was met at a gap of 0.030 — so 68.7% to 72.0% of every nation's
    // raise range (mean 70.3%) bought nothing on this arm. At 6.0 the ceiling is
    // met at 0.10000, the measured mean reach to within 1.1%, which is the
    // derivation education's own 15.0 carries.
    //
    // SECOND RED CHECK, run 2026-09-02 and reverted: the slope left at its old
    // 20.0. RED: "a funded medical corps rebuilt at 1.1000x replacement, not the
    // 1.0300x its x6 slope buys".
    let want = 1.0 + DELTA * 6.0;
    assert!(
        (got - want).abs() < 1e-12,
        "a funded medical corps rebuilt at {got:.4}x replacement, not the {want:.4}x \
         its x6 slope buys"
    );
    // And the arm reaches the state: with the fighting resolved on both sides
    // from the same starting force, the two worlds no longer hold the same army.
    let sustained = war::sustained_force(a.nation(ME), a.nation(ME).mil_spend_gdp);
    for w in [&mut a, &mut b] {
        w.nation_mut(ME).mil_strength = sustained * 0.50;
    }
    war::tick(&mut a);
    war::tick(&mut b);
    assert_ne!(
        a.nation(ME).mil_strength.to_bits(),
        b.nation(ME).mil_strength.to_bits(),
        "the wartime arm never reached the force structure"
    );
}

// ---------------------------------------------------------------------------
// 1. EDUCATION
// ---------------------------------------------------------------------------

/// Education owns the research multiplier ALONE, at the re-derived x15, and
/// reaches nothing else. Its unemployment 0.16, potential 0.050 and stability
/// 5.0 addends are gone, and so is science's x35 from the same expression.
///
/// RED CHECK, run 2026-09-02 and reverted: the slope left at its old 20.0. RED:
/// "education bought 1.1000x the research, not the 1.0750x its x15 slope buys"
/// — the bar reading the slope itself and not merely its sign.
#[test]
fn education_owns_the_research_multiplier() {
    let (base, with) = moves_exactly(BUDGET_EDUCATION, DELTA, &["research"]);
    let ratio = with.research / base.research;
    let want = 1.0 + DELTA * 15.0;
    assert!(
        (ratio - want).abs() < 1e-12,
        "education bought {ratio:.4}x the research, not the {want:.4}x its x15 slope buys"
    );
}

// ---------------------------------------------------------------------------
// 2. HOUSING
// ---------------------------------------------------------------------------

/// Housing keeps its two incumbent arms — births and contentment — and its
/// demand 0.28 addend and its half of the separatism cohesion term are gone.
///
/// RED CHECK, run 2026-09-02 and reverted: the housing half of `cohesion` put
/// back, so housing suppressed separatism again. RED: "ministry 2 moved the
/// wrong set of arms", `["population", "separatism", "stability"]` against
/// `["population", "stability"]`.
#[test]
fn housing_buys_births_and_contentment_and_not_demand() {
    let (base, with) = moves_exactly(BUDGET_HOUSING, DELTA, &["population", "stability"]);
    let start = staged(1990).nation(ME).population;
    let pop = with.population - base.population;
    let want_pop = start * DELTA * 0.015 / 12.0;
    assert!(
        (pop - want_pop).abs() < 1e-9,
        "housing's 0.015 population slope moved {pop:.6}, not {want_pop:.6}"
    );
    // `ds` is applied as `stability + ds / 12.0 * 12.0 * 0.25`.
    let ds = with.stability - base.stability;
    let want = DELTA * 14.0 * 0.25;
    assert!(
        (ds - want).abs() < 1e-9,
        "housing's 14.0 stability slope moved {ds:.6}, not {want:.6}"
    );
}

// ---------------------------------------------------------------------------
// 3. PENSIONS
// ---------------------------------------------------------------------------

/// Pensions owns the labour-force arm and bleeds the standing ceiling, keeps
/// its incumbent stability arm, and its demand 0.18 addend is DROPPED rather
/// than resized.
///
/// RED CHECK, run 2026-09-02 and reverted: the 0.20 labour-force slope set to
/// 0.0 — the arm present but dead. RED: "ministry 3 moved the wrong set of
/// arms", `["stability", "standing"]` against `["jobs_peace", "jobs_war",
/// "stability", "standing"]`. The SAME mutation is what
/// `a_fiscal_year_budget_carries_ten_ministries_into_the_sim` was re-expressed
/// against in lib.rs, and it turns that bar red too: "assertion failed:
/// economy::unemployment_rate(n, false) < jobs_before".
#[test]
fn pensions_own_the_labour_force_and_the_standing_ceiling() {
    let (base, with) = moves_exactly(
        BUDGET_PENSIONS,
        DELTA,
        &["jobs_peace", "jobs_war", "standing", "stability"],
    );
    let jobs = with.jobs_peace - base.jobs_peace;
    let want = -(DELTA * 0.20);
    assert!(
        (jobs - want).abs() < 1e-12,
        "pensions moved unemployment {jobs:.5}, not the {want:.5} its 0.20 slope buys"
    );
    // The ceiling moved by 5 points, and the stock walks toward it at the
    // FALLING rate of 0.055 — the probe nation is seated at 100 and its record
    // justifies less, which is the ordinary case for a government that has just
    // spent everything — so the observable step is 0.275 of a point. Standing
    // is slow to build and quicker to lose; this arm is read on the quick side.
    let step = with.standing - base.standing;
    let want_step = DELTA * 1000.0 * 0.055;
    assert!(
        (step - want_step).abs() < 1e-9,
        "the standing stock stepped {step:.6} toward a ceiling raised 5 points, not {want_step:.6}"
    );
}

// ---------------------------------------------------------------------------
// 4. INFRASTRUCTURE
// ---------------------------------------------------------------------------

/// Infrastructure raises non-oil extraction and NOT oil, and its potential
/// 0.025, unemployment 0.28 and business-pressure 0.02 addends are gone. The
/// dollar it spends still reaches growth through `investment_total`, which is
/// why `growth` is not in the moved list: this probe holds the three aggregates
/// still on purpose, and the aggregate route is the calibrated one that
/// predates the ministries.
///
/// RED CHECK, run 2026-09-02 and reverted: the `if c != OIL` guard removed from
/// `have_table`'s stock loop, so the arm raised the oil column too. RED:
/// "ministry 4 moved the wrong set of arms", `["extraction", "oil"]` against
/// `["extraction"]` — the exclusion the design writes out in full, seen.
#[test]
fn infrastructure_raises_the_ground_but_never_oil() {
    let (base, with) = moves_exactly(BUDGET_INFRASTRUCTURE, DELTA, &["extraction"]);
    // One month of phase-in: the target is DELTA * 2.0 = 0.01, inside the
    // 0.02/month step, so the whole of it lands in the first month.
    let ratio = with.extraction / base.extraction;
    let want = 1.0 + DELTA * resources::INFRA_EXTRACTION_SLOPE;
    assert!(
        (ratio - want).abs() < 1e-12,
        "infrastructure raised extraction {ratio:.5}x, not the {want:.5}x its slope buys"
    );
    assert_eq!(with.oil.to_bits(), base.oil.to_bits(), "infrastructure moved the oil ledger");
}

/// The stock is a stock: it is built over years and lost over years, and it
/// never passes its ceiling however long the budget stands.
///
/// RED CHECK, run 2026-09-02 and reverted: the fixed monthly step replaced by
/// `target - held`, i.e. the stock arriving whole the month the budget is
/// enacted. RED: "the stock arrived in 1 months, which is a switch and not a
/// stock", which is exactly the distinction the design asks for.
#[test]
fn the_infrastructure_stock_is_built_and_lost_over_years() {
    let base = staged(1990);
    // The dial pushed to its own cap, which is the largest thing a player can
    // ask for. Note what this measures: the SLOPE is sized so the top of the
    // dial lands just under the ceiling rather than on it, so the stock settles
    // at what the dial justifies and the ceiling is a guard rather than the
    // destination.
    let cap = BUDGET_CAPS[BUDGET_INFRASTRUCTURE];
    let now = base.nation(ME).annual_budget.as_ref().unwrap().allocations[BUDGET_INFRASTRUCTURE];
    let mut w = gapped(&base, BUDGET_INFRASTRUCTURE, cap - now);
    let target = ((cap - now) * resources::INFRA_EXTRACTION_SLOPE)
        .min(resources::INFRA_EXTRACTION_CEILING);
    let mut months_to_target = None;
    for m in 1..=240 {
        resources::tick(&mut w);
        let held = w.nation(ME).infra_extraction.expect("the stock is seated");
        assert!(
            held <= resources::INFRA_EXTRACTION_CEILING + 1e-12,
            "the stock passed its ceiling at {held}"
        );
        if months_to_target.is_none() && (held - target).abs() < 1e-12 {
            months_to_target = Some(m);
        }
    }
    let built = months_to_target.expect("the stock never reached what the dial justifies");
    assert!(
        built >= 12,
        "the stock arrived in {built} months, which is a switch and not a stock"
    );

    // And it is lost at the same rate. Take the budget back to its reference.
    {
        let plan = w.nation_mut(ME).annual_budget.as_mut().unwrap();
        plan.allocations[BUDGET_INFRASTRUCTURE] = plan.reference[BUDGET_INFRASTRUCTURE];
    }
    let mut lost = None;
    for m in 1..=240 {
        resources::tick(&mut w);
        if w.nation(ME).infra_extraction.unwrap() <= 1e-12 {
            lost = Some(m);
            break;
        }
    }
    assert_eq!(
        lost,
        Some(built),
        "the stock was lost in {lost:?} months against the {built} it took to build"
    );
}

// ---------------------------------------------------------------------------
// 5. INDUSTRY & ENERGY
// ---------------------------------------------------------------------------

/// Industry refills the magazines faster and reaches nothing else. Its
/// potential 0.035, unemployment 0.24 and business-pressure 0.04 addends are
/// gone, and there is no energy system: the design says so in as many words.
///
/// RED CHECK, run 2026-09-02 and reverted: the slope set to 0.0 — the arm
/// present but dead. RED: "ministry 5 moved the wrong set of arms", `[]`
/// against `["munitions"]`.
///
/// SECOND RED CHECK, run 2026-09-02 and reverted: the slope left at its old
/// 20.0. RED: "industry refilled 1.1000x, not the 1.0210x its x4.2 slope buys".
#[test]
fn industry_refills_the_magazines() {
    let (base, with) = moves_exactly(BUDGET_INDUSTRY, DELTA, &["munitions"]);
    // Both start at 0.5, so the step is the refill.
    let ratio = (with.munitions - 0.5) / (base.munitions - 0.5);
    // THE SLOPE IS 4.2, DERIVED, and this is the ministry where the design's
    // first-draft x20 cost the most, because INDUSTRY HAS EXACTLY ONE ARM: when
    // it saturates the whole card is dead and the page prints "another point of
    // GDP buys nothing here" for every remaining press. Industry's reachable
    // raise runs min 0.03900 / mean 0.09505 / max 0.11790 across the 137 living
    // 1990 nations, and at x20 the 1.40 clamp was met at a gap of 0.020, so
    // between 48.7% and 83.0% of the raise range (mean 78.0%) bought nothing at
    // all. At 4.2 the ceiling is met at 0.09524, the measured mean reach to
    // within 0.2%.
    let want = 1.0 + DELTA * 4.2;
    assert!(
        (ratio - want).abs() < 1e-9,
        "industry refilled {ratio:.4}x, not the {want:.4}x its x4.2 slope buys"
    );
}

// ---------------------------------------------------------------------------
// 6. SCIENCE
// ---------------------------------------------------------------------------

/// Science moved to the PRICE side. It makes a technology cheaper to reach and
/// no longer inflates the research bank; its potential 0.025, unemployment 0.08
/// and business-pressure 0.02 addends are gone.
///
/// RED CHECK, run 2026-09-02 and reverted: science's x35 restored to
/// `research_output` alongside the new absorptive-capacity arm, so both
/// ministries multiplied the bank again. RED: "ministry 6 moved the wrong set
/// of arms", `["absorb", "research", "tech_cost"]` against `["absorb",
/// "tech_cost"]` — the two-ministries-one-arm defect this stage exists to
/// remove, caught at the arm it used to share.
#[test]
fn science_buys_reach_and_not_a_bigger_bank() {
    let (base, with) = moves_exactly(BUDGET_SCIENCE, DELTA, &["absorb", "tech_cost"]);
    let got = with.absorb - base.absorb;
    let want = DELTA * 6.0;
    assert!(
        (got - want).abs() < 1e-12,
        "science bought {got:.6} of absorptive capacity, not the {want:.6} its 6.0 slope buys"
    );
    assert!(
        with.tech_cost < base.tech_cost,
        "a science programme did not make frontier work cheaper to reach: {} against {}",
        with.tech_cost,
        base.tech_cost
    );
}

// ---------------------------------------------------------------------------
// 7. DEFENSE
// ---------------------------------------------------------------------------

/// THIS ROW'S JOB IS TO ADD NOTHING, and this is the test that says so. Defense
/// has no gap arm anywhere: its allocation IS `mil_spend_gdp`, the priced
/// aggregate the force model already reads, and a gap arm on top would charge
/// the same money twice.
///
/// RED CHECK, run 2026-09-02 and reverted: `ds += budget_gap[BUDGET_DEFENSE] *
/// 2.0` added to the stability block — the plausible-looking arm a later
/// session might add on the grounds that every other ministry has one. RED:
/// "ministry 7 moved the wrong set of arms", `["stability"]` against `[]`.
#[test]
fn defense_has_no_gap_arm_at_all() {
    moves_exactly(BUDGET_DEFENSE, -DELTA, &[]);
    moves_exactly(BUDGET_DEFENSE, DELTA, &[]);
}

// ---------------------------------------------------------------------------
// 8. SECURITY
// ---------------------------------------------------------------------------

/// Security keeps its incumbent stability arm and now owns the cohesion term
/// alone — and reads a POSITIVE gap only, so cutting the police does not
/// conjure a secession that is not already there.
///
/// RED CHECK, run 2026-09-02 and reverted: the `.max(0.0)` dropped from
/// `cohesion`, so a cut added strain. RED on the CUT half: "ministry 8 moved
/// the wrong set of arms", `["separatism", "stability"]` against
/// `["stability"]` — a police cut conjuring secession that was not there.
#[test]
fn security_alone_suppresses_separatism_and_only_upward() {
    let (base, with) = moves_exactly(BUDGET_SECURITY, DELTA, &["stability", "separatism"]);
    let ds = with.stability - base.stability;
    let want_ds = DELTA * 16.0 * 0.25;
    assert!(
        (ds - want_ds).abs() < 1e-9,
        "security's 16.0 stability slope moved {ds:.6}, not {want_ds:.6}"
    );
    let cohesion = base.separatism - with.separatism;
    let want = DELTA * 0.04;
    assert!(
        (cohesion - want).abs() < 1e-12,
        "security suppressed {cohesion:.6} of strain, not the {want:.6} its 0.04 slope buys"
    );

    // A CUT buys no suppression and, crucially, conjures no strain.
    let (cut_base, cut_with) = moves_exactly(BUDGET_SECURITY, -DELTA, &["stability"]);
    assert_eq!(
        cut_with.separatism.to_bits(),
        cut_base.separatism.to_bits(),
        "cutting the police conjured separatism"
    );
}

// ---------------------------------------------------------------------------
// 9. DIPLOMACY
// ---------------------------------------------------------------------------

/// Diplomacy keeps the sanction shield and gains counter-intelligence. Its
/// business-pressure 0.01 and stability 3.0 addends are gone.
///
/// RED CHECK, run 2026-09-02 and reverted: diplomacy's `ds` 3.0 addend put
/// back. RED: "diplomacy moved stability 0.003787488, which is not the growth
/// route's residual: the removed 3.0 addend was worth 0.003750000 a month" —
/// the residual and the addend, side by side, two orders of magnitude apart.
#[test]
fn diplomacy_keeps_the_shield_and_nothing_else_in_the_economy() {
    // `stability` is in this list and it is NOT a diplomacy gap arm — the
    // removed 3.0 addend is gone. It moves because the shield moves GROWTH, and
    // `economy::tick` writes `growth_last` before it reads it in
    // `ds += (growth_last - 0.015) * 6.0` in the same month. That is the
    // AGGREGATE route, which every ministry is allowed and which predates the
    // ministries; the assertion below proves it is the route taken, because the
    // stability step is exactly the growth step through that term with nothing
    // left over for a resurrected gap addend.
    let (base, with) =
        moves_exactly(BUDGET_DIPLOMACY, DELTA, &["sanctions", "growth", "stability"]);
    // BOUNDED AGAINST THE ADDEND THAT WAS REMOVED, which is the claim being
    // made. Diplomacy's old `ds` addend was `gap * 3.0`, and the stability
    // integrator applies `ds * 0.25`, so at this DELTA it was worth 0.00375 of
    // a point a month. Measured on this tree the residual growth route is worth
    // 0.0000375 — a HUNDREDTH of it — so a bar at a twentieth of the removed
    // addend has the addend's return dead in its sights and leaves the
    // legitimate aggregate route alone.
    let removed_addend = DELTA * 3.0 * 0.25;
    let ds = with.stability - base.stability;
    assert!(
        ds.abs() < removed_addend * 0.05,
        "diplomacy moved stability {ds:.9}, which is not the growth route's residual: \
         the removed 3.0 addend was worth {removed_addend:.9} a month"
    );
    assert!(
        (with.growth > base.growth) == (ds > 0.0),
        "diplomacy's stability residual does not follow its growth"
    );
    // The shield is `(gap * 8.0).clamp(-0.20, 0.40)` on the drag.
    let shield = 1.0 - with.sanctions / base.sanctions;
    let want = DELTA * 8.0;
    assert!(
        (shield - want).abs() < 1e-12,
        "the shield took {shield:.5} off the drag, not the {want:.5} its x8 slope buys"
    );
}

/// One covert operation against the probe nation, answered by whether it was
/// exposed. Exposure is what costs the sponsor relations and reputation, so a
/// fall in the target's opinion of the sponsor is the observable.
fn run_one_covert(mut w: WorldState) -> bool {
    w.nation_mut(THEM).political_capital = 100.0;
    let before = w.relation(ME, THEM);
    let _ = apply_command(
        &mut w,
        &Command::CovertAction {
            sponsor: THEM,
            target: ME,
            op: CovertOp::FundOpposition,
        },
    );
    // The exposed path is the only one that moves the target's opinion of the
    // sponsor; a deniable operation leaves it exactly where it was.
    w.relation(ME, THEM) < before
}

/// DIPLOMACY'S SECOND ARM: a funded foreign service catches more spies.
///
/// THE ONE BAR IN THIS FILE THAT READS A RATE ACROSS SEEDS, so it carries its
/// own `n` per iron rule 7, derived from this test's own measured variance
/// rather than guessed or inherited from a neighbour.
///
/// THE DERIVATION. Both worlds share one RNG stream and consume it identically
/// — `success_p` is unchanged, so its draw is the same draw — and the exposure
/// outcome therefore flips on a seed exactly when that seed's uniform falls
/// between the two thresholds. The gap between them is `DELTA * 10.0 = 0.05`,
/// so the per-seed flip is a Bernoulli with p = 0.05 and its variance IS
/// p(1-p) = 0.0475; there is nothing else in it. For an "at least once" bar the
/// rule's own arithmetic gives n = ln(0.01) / ln(1 - p) = ln(0.01) / ln(0.95)
/// = 89.8, so 90 seeds is the floor and this bar runs 128.
///
/// AND ITS POWER, the half iron rule 7 says to state. This bar exists to catch
/// the arm being DEAD or BACKWARDS. At n = 128 an arm worth nothing flips zero
/// seeds and goes red with certainty; an arm worth half its slope still flips
/// about three and stays green. So it sees "gone", which is what a
/// removal-and-addition stage can plausibly break, and it does not claim to see
/// a mis-sized slope — the slope is pinned by arithmetic in the comment beside
/// the constant, not by this sample. The monotonicity arm inside the loop is an
/// INVARIANT and needs no n at all: funding a foreign service must never make
/// an operation against it harder to expose, on any seed.
///
/// MEASURED ON THIS TREE, 2026-09-02, by forcing the bar red so the counts
/// print: 7 flips in 128 seeds, 18 operations exposed unfunded against 25
/// funded. 7/128 = 0.0547 against the 0.05 the threshold gap predicts, which is
/// the arithmetic above confirming itself rather than a coincidence.
///
/// RED CHECK, run 2026-09-02 and reverted: the counter-intelligence term's 10.0
/// slope set to 0.0. RED: "a funded foreign service caught 0 more spies in 128
/// seeds (base 18, funded 18)" — the arm gone, seen, with the base count
/// unmoved beside it so the reading is unambiguous.
#[test]
fn a_funded_foreign_service_catches_more_spies() {
    const SEEDS: u64 = 128;
    let mut flipped = 0u32;
    let mut caught_base = 0u32;
    let mut caught_with = 0u32;
    for seed in 0..SEEDS {
        let base = staged(seed);
        let with = gapped(&base, BUDGET_DIPLOMACY, DELTA);
        let a = run_one_covert(base);
        let b = run_one_covert(with);
        caught_base += u32::from(a);
        caught_with += u32::from(b);
        if b && !a {
            flipped += 1;
        }
        assert!(
            !(a && !b),
            "seed {seed}: funding the foreign service made an operation HARDER to expose"
        );
    }
    assert!(
        flipped > 0,
        "a funded foreign service caught {flipped} more spies in {SEEDS} seeds \
         (base {caught_base}, funded {caught_with})"
    );
}

// ---------------------------------------------------------------------------
// THE RULE ITSELF
// ---------------------------------------------------------------------------

/// The design's rule, as one table: seating a plan changes exactly the arms the
/// design says it changes and nothing else. Every ministry, in one place, so
/// that a future session adding an arm has to come here and say so.
///
/// This is the bar that would catch an arm added quietly somewhere else in the
/// tree — the thirty scattered addends were added exactly that way, one at a
/// time, each of them locally reasonable.
///
/// RED CHECK, run 2026-09-02 and reverted: `potential += budget_gap[
/// BUDGET_EDUCATION] * 0.050` restored — one of the thirty, put back where it
/// stood. RED on the education row: `["growth", "potential", "research",
/// "stability"]` against `["research"]`, which is one addend reaching four
/// readings and is the whole argument for removing it.
#[test]
fn the_ministry_map_is_exactly_this() {
    let map: [(usize, &str, &[&str]); BUDGET_MINISTRIES] = [
        (BUDGET_HEALTH, "health", &["population"]),
        (BUDGET_EDUCATION, "education", &["research"]),
        (BUDGET_HOUSING, "housing", &["population", "stability"]),
        (
            BUDGET_PENSIONS,
            "pensions",
            &["jobs_peace", "jobs_war", "standing", "stability"],
        ),
        (BUDGET_INFRASTRUCTURE, "infrastructure", &["extraction"]),
        (BUDGET_INDUSTRY, "industry", &["munitions"]),
        (BUDGET_SCIENCE, "science", &["absorb", "tech_cost"]),
        (BUDGET_DEFENSE, "defense", &[]),
        (BUDGET_SECURITY, "security", &["stability", "separatism"]),
        (BUDGET_DIPLOMACY, "diplomacy", &["sanctions", "growth", "stability"]),
    ];
    let base = staged(1990);
    let reference = probe(&base);
    let mut owners: std::collections::BTreeMap<&str, Vec<&str>> = Default::default();
    for (ministry, name, expected) in map {
        let with = gapped(&base, ministry, DELTA);
        let mut moved = reference.differences(&probe(&with));
        moved.sort_unstable();
        let mut want: Vec<&str> = expected.to_vec();
        want.sort_unstable();
        assert_eq!(moved, want, "{name} moved the wrong set of arms");
        for arm in expected {
            owners.entry(arm).or_default().push(name);
        }
    }
    // The arms that legitimately have more than one owner, and the reason each
    // is allowed: `population` and `stability` are STOCKS with several
    // contributors by design — the design names health and housing on one and
    // housing, pensions and security on the other. Everything else must have
    // exactly one ministry behind it, which is the second half of the rule.
    let shared = ["population", "stability"];
    for (arm, ministries) in &owners {
        if shared.contains(arm) {
            continue;
        }
        assert_eq!(
            ministries.len(),
            1,
            "{arm} is written by {ministries:?} — two ministries, one arm"
        );
    }
}


// ---------------------------------------------------------------------------
// THE RULE ITSELF, MOVED THE WAY A PLAYER MOVES IT
// ---------------------------------------------------------------------------

/// The same plan, enacted through the real command.
///
/// `gapped` above writes the allocation straight into the stored `AnnualBudget`
/// and deliberately leaves the three aggregates where the inherited plan put
/// them. That isolates the GAP CHANNEL and is the right instrument for it — but
/// it is a mutation the game cannot perform. `Command::SetAnnualBudget`
/// (lib.rs) writes `social_spend_gdp`, `state_invest_gdp` and `mil_spend_gdp`
/// from the plan as well, so a real press opens LEVEL routes beside the gap
/// ones and the bar above is structurally blind to every one of them. That
/// blindness is how `demand_gap += (social_spend() - baseline) * 0.15` survived
/// the collapse: six ministries reached demand and inflation through the social
/// aggregate, under a comment in `economy.rs` saying none did.
///
/// NO OFFSETTING CUT: only the one dial moves, which is the press a player
/// actually makes when they raise a ministry.
///
/// The command's own PRICE is not a ministry arm. `SetAnnualBudget` is priced
/// against political capital as a function of the swing, so a moved plan and an
/// unmoved one leave different STOCKS behind and `standing` would move for all
/// ten for a reason that has nothing to do with pensions. Both worlds are
/// re-seated to the same stock after the command, which leaves only the TARGET
/// free — and the target is where PENSIONS' arm lives.
fn enacted(w: &WorldState, ministry: usize, delta: f64) -> WorldState {
    let mut w2 = w.clone();
    w2.nation_mut(ME).political_capital = 100.0;
    let year = w2.year;
    let mut allocations = w2.nation(ME).annual_budget.as_ref().expect("the books are open").allocations;
    allocations[ministry] += delta;
    assert!(
        allocations[ministry] <= BUDGET_CAPS[ministry] && allocations[ministry] >= 0.0,
        "the probe moved ministry {ministry} outside its own range"
    );
    apply_command(
        &mut w2,
        &Command::SetAnnualBudget { nation: ME, fiscal_year: year, allocations },
    )
    .expect("the plan is enacted");
    // The stock, re-seated after the price was charged. See the doc above.
    w2.nation_mut(ME).political_capital = 100.0;
    w2
}

/// THE MAP AGAIN, FOR THE PRESS A PLAYER CAN ACTUALLY MAKE.
///
/// `the_ministry_map_is_exactly_this` proves the map for the gap channel. This
/// is the same table proved for the whole press, and the two lists differ by
/// exactly the routes the design KEEPS as aggregates and names as such:
///
/// * INFRASTRUCTURE, INDUSTRY and SCIENCE additionally move `potential`,
///   `growth`, `research` and `stability`, because their dollar enters
///   `investment_total` — "Science STAYS in investment_total" and "the dollar
///   keeps entering investment_total" are the design's own words, and
///   `stability` follows from `growth` through `growth_last` in the integrator,
///   not from any budget addend.
/// * DEFENSE moves `munitions` and `strength_peace`, because defense's
///   allocation IS `mil_spend_gdp`; that is the row whose job is to add nothing
///   NEW.
/// * The six SOCIAL ministries move NOTHING beyond their own gap arms. That is
///   the claim this bar exists for, and it was false until `economy.rs`'s
///   `demand_gap += (social_spend() - baseline_social_spend()) * 0.15` was
///   removed.
///
/// RED CHECK, run 2026-09-02 and reverted: that line restored to `economy.rs`.
/// RED on the first row — "health moved the wrong set of arms", left
/// `["demand_gap", "growth", "population", "stability", "target_inflation"]`
/// against right `["population"]`, one addend reaching five readings. Every
/// other bar in this file stayed GREEN in the same run, INCLUDING
/// `the_ministry_map_is_exactly_this`, which is the whole argument for this one
/// existing: the gap bar cannot see a level route by construction.
#[test]
fn the_enacted_ministry_map_is_exactly_this() {
    let map: [(usize, &str, &[&str]); BUDGET_MINISTRIES] = [
        (BUDGET_HEALTH, "health", &["population"]),
        (BUDGET_EDUCATION, "education", &["research"]),
        (BUDGET_HOUSING, "housing", &["population", "stability"]),
        (
            BUDGET_PENSIONS,
            "pensions",
            &["jobs_peace", "jobs_war", "standing", "stability"],
        ),
        (
            BUDGET_INFRASTRUCTURE,
            "infrastructure",
            &["extraction", "growth", "potential", "research", "stability"],
        ),
        (
            BUDGET_INDUSTRY,
            "industry",
            &["growth", "munitions", "potential", "research", "stability"],
        ),
        (
            BUDGET_SCIENCE,
            "science",
            &["absorb", "growth", "potential", "research", "stability", "tech_cost"],
        ),
        (BUDGET_DEFENSE, "defense", &["munitions", "strength_peace"]),
        (BUDGET_SECURITY, "security", &["separatism", "stability"]),
        (BUDGET_DIPLOMACY, "diplomacy", &["growth", "sanctions", "stability"]),
    ];
    let base = staged(1990);
    for (ministry, name, expected) in map {
        // The reference is the SAME command with a zero delta, so the pair
        // differs in the delta and in nothing else — not in the command, not in
        // the aggregates it re-seats, not in the price it charged.
        let reference = probe(&enacted(&base, ministry, 0.0));
        let mut moved = reference.differences(&probe(&enacted(&base, ministry, DELTA)));
        moved.sort_unstable();
        let mut want: Vec<&str> = expected.to_vec();
        want.sort_unstable();
        assert_eq!(moved, want, "{name} moved the wrong set of arms");
        // Stated separately as well as being implied by the list above, because
        // this is the specific claim `economy.rs` makes in a comment and the one
        // that was false: `demand_gap` forks into output AND into the price
        // impulse, so a ministry that reaches it is charged to two aggregates at
        // once and can be read neither way.
        assert!(
            !moved.contains(&"demand_gap") && !moved.contains(&"target_inflation"),
            "{name} reaches demand through a level route: {moved:?}"
        );
    }
}

// ---------------------------------------------------------------------------
// NO CARD PROMISES WHAT THE SIM CLAMPS AWAY
// ---------------------------------------------------------------------------

/// The stability integrator run to its fixed point with everything else frozen.
///
/// DELIBERATELY NOT `ministries::stability_settles_at`. That function is the
/// closed form and is the thing under test; this is the loop the closed form
/// claims to solve, written out from `economy::tick`'s own two lines, so the bar
/// below compares two independent computations rather than comparing a function
/// to itself. spheres-web's `a_stability_arm_is_quoted_as_where_order_settles`
/// is the tautology this replaces: it pinned the card against `arms_at` and was
/// green while the card promised +168 points on a 0..100 scale.
///
/// THE LIMIT OF THIS BAR, recorded rather than assumed (iron rule 7's power
/// half): the loop is a TRANSCRIPTION of the integrator, so a change to the
/// integrator's shape makes it stale rather than red. What it can see, and what
/// it exists for, is the card and the fixed point disagreeing -- which is what
/// they did.
fn settle(pressure: f64, start: f64) -> f64 {
    let mut s = start;
    for _ in 0..6000 {
        let ds = pressure + (60.0 - s) * 0.01;
        s = (s + ds / 12.0 * 12.0 * 0.25).clamp(0.0, 100.0);
    }
    s
}

/// A STABILITY ARM IS WHERE THE NATION ACTUALLY SETTLES, bound included.
///
/// `stability_destination` used to be `ds / MEAN_REVERSION` and stopped there,
/// but `economy::tick` clamps the integrator to 0..100 and the integrator does
/// not merely approach that bound, it stops at it. MEASURED on this tree before
/// the repair, by running `settle` to its fixed point: the USA settles at 48.933
/// with no gap; a security gap of +0.030 was quoted +48.0 and delivered +48.000,
/// but +0.050 was quoted +80.0 and delivered +51.067, and at the top of the dial
/// (+0.10495) it was quoted +167.9 and delivered +51.067 -- a displacement that
/// is not merely wrong but impossible on a 0..100 scale for a player sitting at
/// 78. India, base 29.380, was quoted +169.0 at the top of security and
/// delivered +70.620.
///
/// MEASURED AFTER, this run, over four nations x three ministries x six dial
/// positions each including the cap: every card value agrees with `settle`'s
/// fixed point to within 1.4e-5, which is the residual of a 6000-step approach
/// and not a disagreement about the answer.
///
/// AN INVARIANT (iron rule 7's carve-out): a universal claim about deterministic
/// arithmetic, so the four nations are a budget question and not a sampling one.
/// They cover the three regimes -- the USA and Belgium settle mid-scale, India
/// low, and BRAZIL AT 1990 SETTLES AT EXACTLY 0.000 because hyperinflation
/// swamps everything else, so for Brazil the whole stability dial honestly buys
/// nothing and the card now says so instead of promising +168.
///
/// RED CHECK, run 2026-09-02 and reverted: `stability_destination` restored to
/// `ds_contribution / MEAN_REVERSION`, the shipped body. RED at the first
/// clamped sample -- "USA ministry 2 at gap 0.05000 is quoted 70.0000000 where
/// the integrator settles at 51.0674913".
#[test]
fn a_stability_arm_is_the_integrators_own_fixed_point() {
    // The loop's own sanity, so a broken loop cannot silently agree with a
    // broken card: with no pressure at all the integrator's attractor is 60.
    // 1e-4 is the loop's own residual and not a slackened bar: the approach is
    // geometric at 0.0025 a step, so 6000 steps from 12.0 leave
    // 48 * exp(-15) = 1.5e-5 on the table. Measured, not guessed.
    assert!((settle(0.0, 12.0) - 60.0).abs() < 1e-4, "the integrator's attractor is not 60");

    let mut checked = 0usize;
    let mut clamped_samples = 0usize;
    for me in [NationId::USA, NationId::India, NationId::Brazil, NationId::Belgium] {
        let mut w = staged(1990);
        w.player = Some(me);
        w.nation_mut(me).political_capital = 100.0;
        let year = w.year;
        let allocations = w.nation(me).budget_for(year).allocations;
        apply_command(
            &mut w,
            &Command::SetAnnualBudget { nation: me, fiscal_year: year, allocations },
        )
        .expect("the books open");
        let n = w.nation(me);
        let pressure = economy::stability_pressure_of(&w, n);
        let base = settle(pressure, n.stability);

        for m in [BUDGET_HOUSING, BUDGET_PENSIONS, BUDGET_SECURITY] {
            let reference = n.budget_for(year).reference[m];
            let top = BUDGET_CAPS[m] - reference;
            for gap in [0.005, 0.010, 0.020, 0.030, 0.050, top] {
                let arm = spheres_sim::ministries::stability_arm(m, gap)
                    .expect("these three own a stability arm");
                let want = settle(pressure + arm, n.stability) - base;
                let card = spheres_sim::ministries::arms_at(&w, n, m, reference + gap)
                    .into_iter()
                    .find(|a| a.id == "stability")
                    .expect("the card carries a stability arm")
                    .value;
                assert!(
                    (card - want).abs() < 1e-4,
                    "{me:?} ministry {m} at gap {gap:.5} is quoted {card:.7} where the \
                     integrator settles at {want:.7}"
                );
                if (want - arm / spheres_sim::ministries::MEAN_REVERSION).abs() > 1e-4 {
                    clamped_samples += 1;
                }
                checked += 1;
            }
        }
    }
    // POWER: the sweep has to CONTAIN samples where the clamp binds, or it would
    // be green against a card that ignored the bound entirely.
    assert!(
        clamped_samples > 0,
        "{clamped_samples} of {checked} samples were clamped -- this bar saw nothing"
    );
    println!("{checked} stability samples agree, {clamped_samples} of them past the bound");
}

/// PENSIONS' STANDING ARM DIES WHERE THE 100-POINT CEILING BINDS, and the card
/// dies with it.
///
/// `politics.rs` adds `pensions_standing(gap)` to the standing target and clamps
/// the target to 0..100 on the next line; the arm's slope is 1000.0 and the
/// largest reachable pensions gap is about 0.145, so the ceiling saturates long
/// before the dial runs out. The source comment beside the slope already SAID
/// so, and the card denied it.
///
/// MEASURED on the tree as it stood, Brazil with the books open, one
/// `politics::tick`, seed 1990: no gap -> political_capital 20.7002; +0.005 ->
/// 20.9285; +0.020 -> 21.3485; +0.040 -> 21.9085; +0.060 -> 22.4685; +0.100 ->
/// 23.0954; +0.14120, the top of the dial -> 23.0954, identical. Bisected, the
/// ceiling binds at gap 0.08239, so the top 41.7% of the dial bought ZERO
/// standing while the served `per_point` stayed at +10.000000 for Brazil,
/// Belgium and the USA alike.
///
/// AN INVARIANT. RED CHECK, run 2026-09-02 and reverted: the pensions `standing`
/// arm in `arms_at` restored to the bare `pensions_standing(gap)`. RED -- "the
/// card still charges for a press past the ceiling: 141.2 then 151.2", two
/// displacements of a 100-point ceiling that a player sitting anywhere on the
/// scale can never be paid.
#[test]
fn the_pensions_standing_arm_dies_with_the_ceiling() {
    let mut w = staged(1990);
    w.nation_mut(ME).political_capital = 100.0;
    let year = w.year;
    let allocations = w.nation(ME).budget_for(year).allocations;
    apply_command(&mut w, &Command::SetAnnualBudget { nation: ME, fiscal_year: year, allocations })
        .expect("the books open");
    let reference = w.nation(ME).budget_for(year).reference[BUDGET_PENSIONS];
    let top = BUDGET_CAPS[BUDGET_PENSIONS] - reference;

    // What one month of `politics::tick` actually does to the stock, per gap.
    let after = |gap: f64| {
        let mut w2 = w.clone();
        w2.nation_mut(ME).annual_budget.as_mut().unwrap().allocations[BUDGET_PENSIONS] =
            reference + gap;
        w2.nation_mut(ME).political_capital = 20.0;
        politics::tick(&mut w2);
        w2.nation(ME).political_capital
    };
    // The ceiling is genuinely saturated at the top of the dial: two very
    // different gaps produce the SAME stock, bit for bit.
    assert_eq!(
        after(0.100).to_bits(),
        after(top).to_bits(),
        "the pensions ceiling is not saturated at the top of the dial, so this bar is pointed \
         at nothing"
    );
    // ...and the card agrees, rather than promising ten points a percent there.
    let arm_at = |alloc: f64| {
        spheres_sim::ministries::arms_at(&w, w.nation(ME), BUDGET_PENSIONS, alloc)
            .into_iter()
            .find(|a| a.id == "standing")
            .expect("pensions carries a standing arm")
            .value
    };
    let at_top = arm_at(reference + top);
    let a_point_further = arm_at(reference + top + 0.01);
    assert_eq!(
        at_top.to_bits(),
        a_point_further.to_bits(),
        "the card still charges for a press past the ceiling: {at_top} then {a_point_further}"
    );
    // The realised displacement is bounded by the scale it lives on.
    assert!(
        at_top <= 100.0 && at_top > 0.0,
        "the card promises {at_top} points of a 100-point ceiling"
    );
    // And the arm is still LIVE at the bottom of the dial, so the repair did not
    // simply zero it: half a point of GDP is the design's own sizing, "+0.5% of
    // GDP is about +5 points".
    let small = arm_at(reference + DELTA);
    assert!(
        (small - 5.0).abs() < 1e-9,
        "half a point of GDP buys {small} points of ceiling, not the design's 5.0"
    );
    println!("pensions standing: {small:.3} at +0.5% of GDP, {at_top:.3} at the top of the dial");
}

/// SCIENCE'S ARM IS THE REALISED CHANGE IN ABSORPTIVE CAPACITY, ceiling
/// included.
///
/// `tech::absorptive_capacity` clamps its sum to 0.05..1.20, and science's arm
/// is largest exactly where a nation has no headroom left -- the 6.0 slope was
/// sized so a maximal programme is worth "roughly the whole of the 0.40
/// development term", and a nation that HAS the development term has already
/// spent that room. MEASURED this run, seed 1990, books open, bisecting to the
/// first gap whose capacity equals the value at the cap: USA base 0.9746 and the
/// ceiling binds at gap 0.03757 of a 0.07475 reachable range, so the top 49.7%
/// of the dial buys nothing; Japan 0.9119 and 33.8%; Germany 0.8435 and 20.0%;
/// Brazil 0.4715 and never. The served `per_point` was +0.060000 at the cap for
/// all three of Brazil, Belgium and the USA.
///
/// This bar reads the card against `tech::absorptive_capacity` itself -- the
/// function the price side calls -- rather than against the arm's own slope.
///
/// RED CHECK, run 2026-09-02 and reverted: science's arm in `arms_at` restored
/// to the bare `science_absorption(gap)`. RED -- "USA science at gap 0.04000 is
/// quoted 0.2400000 where capacity moves 0.2257086".
#[test]
fn the_science_arm_is_the_capacity_the_sim_reaches() {
    let mut checked = 0usize;
    let mut clamped = 0usize;
    for me in [NationId::USA, NationId::Japan, NationId::Germany, NationId::Brazil] {
        let mut w = staged(1990);
        w.player = Some(me);
        w.nation_mut(me).political_capital = 100.0;
        let year = w.year;
        let allocations = w.nation(me).budget_for(year).allocations;
        apply_command(
            &mut w,
            &Command::SetAnnualBudget { nation: me, fiscal_year: year, allocations },
        )
        .expect("the books open");
        let reference = w.nation(me).budget_for(year).reference[BUDGET_SCIENCE];
        let top = BUDGET_CAPS[BUDGET_SCIENCE] - reference;
        let dev = tech::dev_of(w.nation(me));
        let base = tech::absorptive_capacity(&w, w.nation(me), dev);

        for gap in [0.005, 0.010, 0.020, 0.040, top] {
            // What the SIM's capacity actually becomes at that allocation.
            let mut w2 = w.clone();
            w2.nation_mut(me).annual_budget.as_mut().unwrap().allocations[BUDGET_SCIENCE] =
                reference + gap;
            let want = tech::absorptive_capacity(&w2, w2.nation(me), dev) - base;
            let card =
                spheres_sim::ministries::arms_at(&w, w.nation(me), BUDGET_SCIENCE, reference + gap)
                    .into_iter()
                    .find(|a| a.id == "reach")
                    .expect("science carries a reach arm")
                    .value;
            assert!(
                (card - want).abs() < 1e-12,
                "{me:?} science at gap {gap:.5} is quoted {card:.7} where capacity moves {want:.7}"
            );
            if (want - gap * 6.0).abs() > 1e-9 {
                clamped += 1;
            }
            checked += 1;
        }
    }
    assert!(clamped > 0, "no sample met the 1.20 ceiling -- this bar saw nothing");
    println!("{checked} science samples agree, {clamped} of them past the 1.20 ceiling");
}

/// DIPLOMACY'S COUNTER-INTELLIGENCE ARM CARRIES THE 0.85 CEILING.
///
/// `statecraft::exposure_probability` clamps to 0.05..0.85; the arm's slope is
/// 10.0 and the largest reachable diplomacy gap is about 0.076, so the card
/// offered up to +75.8pp of exposure into a probability with at most 75 points
/// of room from a COLD channel against a pure democracy, and far less in
/// practice. MEASURED: the largest reachable gap is 0.07580 for Brazil, 0.07570
/// for Belgium and the USA and 0.07590 for India, and the served `per_point` at
/// the cap was +0.100000 for all of them.
///
/// The card is quoted at zero heat, which is the FIRST operation mounted against
/// the nation and the case where the arm buys the most; heat accrues 0.18 an
/// operation and only ever eats headroom, which the arm's note now says.
///
/// RED CHECK, run 2026-09-02 and reverted: the counterintel arm restored to the
/// bare `diplomacy_counterintel(gap)`. RED -- "the card offers 0.7580000 of
/// exposure where the probability moves 0.7200000".
#[test]
fn the_counterintel_arm_carries_the_exposure_ceiling() {
    let w = staged(1990);
    let n = w.nation(ME);
    let year = w.year;
    let reference = n.budget_for(year).reference[BUDGET_DIPLOMACY];
    let top = BUDGET_CAPS[BUDGET_DIPLOMACY] - reference;
    let cold = spheres_sim::statecraft::exposure_probability(0.0, n.authoritarianism, 0.0);

    let mut clamped = 0usize;
    for gap in [0.005, 0.020, 0.050, top] {
        let want = spheres_sim::statecraft::exposure_probability(
            0.0,
            n.authoritarianism,
            spheres_sim::ministries::diplomacy_counterintel(gap),
        ) - cold;
        let card = spheres_sim::ministries::arms_at(&w, n, BUDGET_DIPLOMACY, reference + gap)
            .into_iter()
            .find(|a| a.id == "counterintel")
            .expect("diplomacy carries a counterintel arm")
            .value;
        assert!(
            (card - want).abs() < 1e-12,
            "the card offers {card:.7} of exposure where the probability moves {want:.7}"
        );
        // The claim that matters: no card value can exceed the room the clamp
        // leaves, whatever the slope.
        assert!(
            card <= 0.85 - cold + 1e-12,
            "the card offers {card:.7} into a probability with {:.7} of room",
            0.85 - cold
        );
        if (want - gap * 10.0).abs() > 1e-9 {
            clamped += 1;
        }
    }
    assert!(clamped > 0, "no sample met the 0.85 ceiling -- this bar saw nothing");
    println!("counterintel bounded: cold base {cold:.4}, {clamped} samples past the ceiling");
}
