use crate::world::*;

/// Annual growth a sanctions regime costs its target when the *entire* world
/// economy has shut its doors. Charged in proportion to the sanctioners' share
/// of world output, so a coalition weighing half the world costs half of it.
///
/// This replaced `sanctioned_by_count() as f64 * 0.006`, which counted flags.
/// The two rules agree only when every sanctioner weighs 30% of the world
/// economy: the old one charged Luxembourg joining an embargo exactly what it
/// charged the United States joining it, and the total bill rose without limit
/// as the roster grew. `oil_blockade` in world.rs has always weighed output
/// rather than counting signatures; this is the same quantity, and the comment
/// there — "an embargo bites in proportion to the demand that closes its doors"
/// — was already the correct statement of what this term means.
///
/// WHY THIS AND NOT THE CATCHUP COEFFICIENT. `china_growth_miracle` went red at
/// 10.13x when the roster grew 31 -> 108, and the suspicion recorded on
/// `golden_hash_of_a_known_run` was that the catchup coefficient below had been
/// fitted against a world GDP that was ~18% too small. It had not. Measured
/// with `ai_aggression = 0.0`, so that China simply grows for thirty years:
///
/// ```text
///     China 30-year multiple, at peace, 108 nations, seeds 0..=9:
///       14.15 14.03 13.68 13.80 14.34 14.40 13.68 13.69 14.47 14.02
///       median 14.02x, against the real 14.33x (World Bank NY.GDP.MKTP.KD)
/// ```
///
/// The growth model is right to within 2% and the catchup coefficient is not
/// mistuned. Raising it to lift the median would have pushed a peaceful China
/// past 19x and every developing economy with it — fitting a constant to a test
/// while the constant was already correct. The affordability denominator in
/// `tech::tick` was swept 3.2x and did not respond either; the numbers are in
/// the comment there.
///
/// What the roster actually changed is how often China is *sanctioned*. At 108
/// nations China has fourteen land neighbours instead of two, so it goes to war
/// in 6 of 10 seeds rather than 4 — and the old rule then charged the G5
/// coalition that forms (5 flags, ~52% of world output) a flat 3.0 points of
/// annual growth for fifteen years and more. That single term is the whole of
/// the bimodality `nations.rs` documents at its East Asia block: China either
/// stays at peace and finishes at 13-18x, or fights and finishes at 6-10x, with
/// almost nothing in between, so the median is decided by which side five or
/// six of ten seeds fell on. At bite 0.000 the ten seeds span 10.01-15.52; at
/// 0.030 they span 8.77-17.10. The spread IS this coefficient.
///
/// ANCHORED ON REALITY, NOT ON THE TEST. Growth lost per year by a target under
/// a held regime, measured against an otherwise identical control over 20 years
/// with `ai_aggression = 0.0` (`sanction_cost_calibration`, ignored):
///
/// ```text
///     bite   USA alone -> China   G5 -> China
///            (share 0.24)         (share 0.52)
///     0.000        0.20pt              0.46pt   <- the three count-based
///     0.010        0.48pt              1.09pt      channels still unconverted
///     0.015        0.47pt              1.37pt
///     0.020        0.59pt              1.53pt   <- shipped
///     0.025        0.72pt              1.70pt
///     0.030        0.87pt              1.92pt
///     0.040        1.11pt              2.55pt
/// ```
///
/// The real regimes of the period, non-oil channel only:
///   - China 2018-19, United States alone, ~24% of world output: growth 6.7% ->
///     6.0%, about 0.6pt. Model at 0.020: 0.59pt.
///   - South Africa 1985-93, near-universal, ~80%: ~1.0% growth against a ~3.5%
///     trend, about 2.5pt. Model at 0.020 scaled to 0.80: 2.35pt.
///   - Russia 2014-21 and Iran 2012-15 are the two other large regimes and are
///     DELIBERATELY NOT USED. Both targets are petro-states whose measured loss
///     ran mostly through oil, and this model prices that separately in
///     `embargo_drag` and `oil_blockade`. Calibrating the non-oil growth drag on
///     them would count the same barrel twice. Taken at face value they would
///     argue for 0.010-0.015.
///
/// So the two clean anchors bracket 0.020-0.025, and 0.020 is taken rather than
/// 0.025 because three further sanction channels still count flags rather than
/// weighing output and add 0.46pt on top at G5 weight (the 0.000 row above):
/// `research_output` and `absorptive_capacity` in tech/mod.rs, and the stability
/// term below. Converting those is the follow-up this commit deliberately does
/// not do, so that the movement here is attributable to one line.
///
/// **THAT FOLLOW-UP IS NOW DONE, AND THIS CONSTANT DID NOT MOVE WITH IT.** All
/// four surviving count-based channels have been converted — the stability term
/// below, `research_output` and `absorptive_capacity` in tech/mod.rs, and the
/// Business pillar in government.rs — each on the same `c / 0.30` carry-across
/// the paragraph above describes. So the 0.46pt the 0.000 row measures is gone,
/// and the reasoning quoted immediately above would now argue for 0.025 rather
/// than 0.020.
///
/// It is deliberately NOT moved. Re-deriving it here would be a coefficient
/// tuned inside a change whose whole purpose is to make four other coefficients
/// attributable, and 0.020 remains inside the 0.020-0.025 bracket the two clean
/// anchors give. It is recorded as an open calibration question, with the
/// measurement that would settle it already named — re-run
/// `sanction_cost_calibration` now that the three parasitic channels are off it,
/// and the USA-alone and G5 columns should be read again from scratch.
const SANCTION_BITE: f64 = 0.020;

/// Points of annual growth a sanctions regime of this weight takes off its
/// target, which is `SANCTION_BITE` applied to a share of world output.
///
/// A FUNCTION RATHER THAN A BARE MULTIPLICATION IN `tick`, so that the one other
/// place that has to know this number can be given it instead of guessing.
/// spheres-web's policy panel prints the drag under the ledger, and it was
/// computing `sanctioned_by_count * 0.006` — the rule this channel was converted
/// AWAY from when it stopped counting flags and started weighing output. A count
/// and a share are not the same shape, so the two answers do not merely differ by
/// a factor: measured over four seeds and thirty years the browser's figure ran
/// from 0.09x to 313x the sim's, and its worst reading was outside anything the
/// sim can produce, because a count is unbounded and a share is not.
///
/// Takes the weight rather than the world, deliberately: `sanction_weight` is an
/// O(roster) scan and `tick` already has the value in hand, so this must not be
/// the thing that makes it run twice a month per nation.
pub fn growth_drag_of_sanctions(sanction_weight: f64) -> f64 {
    sanction_weight * SANCTION_BITE
}

/// The inflation an economy settles on when demand is at potential — and
/// therefore, necessarily, the number its central bank is aiming at. It was
/// written twice: 0.02 in the price equation here and 0.025 in the Taylor rule
/// in politics.rs. Two constants naming one object, and the half-point
/// disagreement was paid out as permanent free growth to every nation on the
/// board.
///
/// Solve the closed loop the two of them make — `r* = 0.010 + 1.6π`,
/// `real* = 0.010 + 0.6π`, `g = 0.55(0.015 - 0.6π)`, `π = 0.02 + 1.6g` — and it
/// has a fixed point at `1.528g = 0.00165`, i.e. `g* = +0.00108`, `π* = 0.02173`,
/// `r* = 0.04477`. The six mature economies sat on all three to three
/// significant figures in the final year: demand_gap 0.1069-0.1079 pt,
/// inflation 0.0217-0.0218, rate 0.0448-0.0449. A demand gap is by definition a
/// deviation from potential and must average zero; a cyclical term with a
/// non-zero mean is BIBLE §8's error class. One constant, and the fixed point is
/// zero by algebra rather than by tuning.
///
/// There are genuinely two objects across these two files and only one of them
/// disagreed: the neutral real rate is 0.025 in both (`neutral` below, the
/// nominal intercept in politics.rs) and always agreed. The inflation anchor is
/// the one that did not.
///
/// 2.0% and not 2.5% because 2.0% is the announced target of every central bank
/// on this panel — the Federal Reserve, the ECB, the Bank of England and the
/// Bank of Japan. politics.rs's 0.025 was the unsourced half, so politics.rs
/// moved and this did not.
pub const INFLATION_ANCHOR: f64 = 0.020;

/// A readable labor-market result derived from the same conditions that drive
/// output. It is deliberately a rate, not a second labor simulation: the
/// arcade layer needs jobs to tell the player what growth means for people,
/// while the calibrated production model still owns the underlying economy.
pub fn unemployment_rate(n: &Nation, at_war: bool) -> f64 {
    let gdp_pc = n.gdp * 1000.0 / n.population.max(0.1);
    let dev = (gdp_pc / 24000.0).min(1.0);
    let natural = 0.045
        + (1.0 - dev) * 0.035
        + if n.system == EconomySystem::Command { 0.012 } else { 0.0 };
    let cyclical = (0.020 - n.growth_last) * 0.90;
    let disorder = (50.0 - n.stability).max(0.0) * 0.001;
    let war = if at_war { 0.020 } else { 0.0 };
    let balance_sheet = (-n.bubble).max(0.0) * 0.035;
    // PENSIONS ALONE OWNS THIS ARM. Health 0.12, education 0.16,
    // infrastructure 0.28, industry 0.24 and science 0.08 were removed from
    // this expression in the ministry collapse: five ministries were pushing
    // the same rate, none of them was calibrated, and between them they reached
    // output twice — once here and once through `potential`.
    //
    // LABOUR-FORCE WITHDRAWAL, which is the honest reason a pension budget
    // moves the unemployment rate and the only one: a state pension that is
    // worth taking is a state pension people take, and the people who take
    // it leave the measured labour force. This is why the design DROPS
    // pensions' demand arm rather than resizing it — `demand_gap` forks into
    // both output and inflation, so a pension rise routed through demand
    // would reach the same GDP twice.
    //
    // INVENTED, and labelled as the design requires: the 0.20 slope. The
    // design sizes it at "+0.5% of GDP is about -0.1pp of unemployment",
    // and 0.005 * 0.20 = 0.001 is exactly that. The existing
    // `clamp(0.02, 0.35)` on the result bounds it in both directions.
    let ministry_jobs = crate::ministries::pensions_jobs(n.budget_gap(BUDGET_PENSIONS));
    (natural + cyclical + disorder + war + balance_sheet - ministry_jobs).clamp(0.02, 0.35)
}

/// Monthly economic tick for every living nation, plus the global oil market.
/// The floor under a monthly output collapse. Module-level rather than
/// function-local since `growth_terms` and `tick` both apply it — the first
/// to a sum with no noise in it, the second to the same sum with the month's
/// draw added. The value is untouched; only its scope moved.
const WORST_ANNUAL_COLLAPSE: f64 = -0.95;

/// What the world is doing to one nation this month, gathered once so that the
/// arithmetic below can be a pure function of a [`Nation`].
///
/// Every field is a world-level query `tick` was already making per nation. It
/// is a type so that the one other place that has to price a policy — the
/// browser's panel — can build the same four out of the same world and be given
/// the sim's answer instead of writing its own.
pub struct Conditions {
    pub oil_price: f64,
    /// Share of world output held by the states sanctioning this one.
    pub sanction_share: f64,
    pub at_war: bool,
    /// The fraction of this nation's oil that can actually leave.
    pub export_share: f64,
}

impl Conditions {
    pub fn of(w: &WorldState, id: NationId) -> Conditions {
        Conditions {
            oil_price: w.oil_price,
            sanction_share: w.sanction_weight(id),
            at_war: w.at_war(id),
            export_share: w.oil_export_share(id),
        }
    }
}

// ---------------------------------------------------------------------------
// THE TREASURY
//
// Money as a stock rather than a ratio. Two `Option<f64>` fields on `Nation`
// hold billions of 1990 dollars; `None` is the pre-treasury world and runs the
// arithmetic it always ran, to the bit. Everything in this section is dead code
// on the default board and is asserted to be, by
// `the_treasury_is_inert_while_the_books_are_closed`.
// ---------------------------------------------------------------------------

/// The floor under the REAL policy rate a government pays on its own debt.
///
/// INVENTED, and carried across from the design unchanged. A government can
/// borrow at a negative real rate -- most of the roster did in the 1970s and
/// several did again after 2010 -- but not without limit, because at some
/// negative real return the lender buys goods instead. -2%/yr is a bound rather
/// than a calibration: nothing the shipped board produces sits near it, and it
/// is here so that interest is provably bounded below rather than bounded by
/// inspection.
const REAL_RATE_FLOOR: f64 = -0.02;

/// Where the sovereign spread starts, as a debt-to-output ratio.
///
/// INVENTED. Two measurements pinned it. The roster's own MEDIAN 1990 debt
/// ratio is 0.52 -- measured across all 137 files in `data/nations/`, mean
/// 0.642, min 0.00 (Brunei), max 3.80 (Nicaragua) -- so a knee at 0.60 leaves
/// the median borrower paying exactly its own policy rate and charges nothing
/// at all to the half of the world below it, which is the design's requirement
/// that a nation at the 1990 median pay close to the policy rate. And 0.60 is
/// the SAME line `dyads.rs` already draws for fiscal desperation in the
/// war-appetite term, `(debt_gdp - 0.6).max(0.0) * 1.5`: the point at which the
/// bond market starts charging a premium is the point at which a government
/// starts behaving as though it is short of money. One line, not two.
const SPREAD_KNEE: f64 = 0.60;

/// How fast the spread widens past the knee, in rate per unit of debt ratio.
///
/// INVENTED. Sized off the one endpoint the design names: a nation at 90% of
/// output "pays a visible premium". At 0.06 that premium is 1.80 percentage
/// points -- visible against policy rates the roster carries between 2.9% and
/// 25%, large enough to be worth consolidating away, and far short of the
/// several hundred basis points that mean a market thinks it will not be paid.
const SPREAD_SLOPE: f64 = 0.06;

/// The most the spread can ever add, whatever the debt ratio.
///
/// INVENTED, and a guard rather than a calibration -- the thing that stops a
/// spiral. Interest is the one term in this model that feeds its own input:
/// more debt buys a wider spread buys more interest buys more debt. Uncapped,
/// that recursion has no fixed point and the `debt_gdp < 6.0` invariant
/// asserted in four places in lib.rs would eventually be a coin toss. At 6
/// percentage points the cap binds from a debt ratio of 1.60 upward, so every
/// borrower on the shipped board sits on the sloped part of the curve and only
/// a state already past 160% of output ever meets the ceiling.
const SPREAD_CAP: f64 = 0.06;

/// What a government actually pays on its debt, as an annual rate.
///
/// RIDGE'S AMENDMENT, 2026-09-02: the rate is not flat. It is the real policy
/// rate plus a sovereign spread that RISES with the debt ratio, so borrowing
/// gets dearer the more of it a state has already done. All three coefficients
/// are invented and each is labelled above with what it does.
///
/// MEASURED on this definition at a 5% policy rate against 3% inflation, so the
/// real rate is 2.00% and the spread is the whole of the difference:
///     debt  30% of GDP -> 2.00%/yr  (+0.00pp; below the knee)
///     debt  60% of GDP -> 2.00%/yr  (+0.00pp; exactly at the knee)
///     debt  90% of GDP -> 3.80%/yr  (+1.80pp)
///     debt 150% of GDP -> 7.40%/yr  (+5.40pp)
///     debt 160% of GDP -> 8.00%/yr  (+6.00pp, the cap, and flat above it)
pub fn effective_interest_rate(policy_rate: f64, inflation: f64, debt_gdp: f64) -> f64 {
    let real = (policy_rate - inflation).max(REAL_RATE_FLOOR);
    let spread = ((debt_gdp - SPREAD_KNEE).max(0.0) * SPREAD_SLOPE).min(SPREAD_CAP);
    real + spread
}

/// Debt service, annualised and expressed as a share of output.
///
/// THE ONE DEFINITION, for the same reason `GrowthTerms` is the one definition
/// of a growth rate: the ledger, the interest row on the budget card and any
/// future growth arm all read this, so they cannot disagree about what the
/// state is paying. `interest_bn * 12 / gdp` reduces to `debt_bn * rate / gdp`,
/// which is what is written.
///
/// Zero while the books are closed, which is what "this nation does not model
/// debt service" must mean to a caller that adds it to something.
pub fn interest_gdp(n: &Nation) -> f64 {
    match n.debt_bn {
        Some(debt) => {
            debt * effective_interest_rate(n.interest_rate, n.inflation, n.debt_gdp)
                / n.gdp.max(0.1)
        }
        None => 0.0,
    }
}

/// The state's books for one month, annualised.
///
/// THE ONE DEFINITION of what a government takes in and what it pays out, and
/// the reason it is a type rather than four expressions: `economy::tick`
/// charges these numbers, and the money card prints them. Every previous time
/// this codebase let the browser own an arithmetic rule, the two copies came
/// apart -- the growth model by four whole terms, the sanction drag by 313x.
/// The browser now reads `Fiscal` and multiplies nothing.
///
/// SIGN CONVENTION, settled here because the browser had two of them: `balance`
/// is REVENUE LESS SPENDING LESS INTEREST, so positive is a surplus, in every
/// unit and on every card. The ledger used to call the same quantity `deficit`
/// with the opposite sign while the budget card called it `balance` with this
/// one, and the two sat four inches apart on the same screen.
///
/// Interest is a share of GDP and a rate, both, because Ridge's amendment asks
/// the card for both: `interest_gdp` is what debt service costs as a share of
/// output, and `effective_rate` is the rate it is charged at, which rises with
/// the debt ratio. `interest_bn` is the annual charge in billions and is what
/// `tick` divides by twelve.
#[derive(Clone, Copy, Debug)]
pub struct Fiscal {
    pub revenue_gdp: f64,
    pub spend_gdp: f64,
    /// Annual debt service in billions. Zero while the books are closed.
    pub interest_bn: f64,
    /// The same charge as a share of output -- `economy::interest_gdp`.
    pub interest_gdp: f64,
    /// The rate that charge is struck at, real policy rate plus the sovereign
    /// spread. Served beside the interest line, which is Ridge's amendment.
    pub effective_rate: f64,
    /// Revenue less spending less interest. POSITIVE IS A SURPLUS.
    pub balance_gdp: f64,
}

impl Fiscal {
    pub fn of(n: &Nation, terms: &GrowthTerms) -> Fiscal {
        let revenue_gdp = n.tax_rate + terms.budget_oil_revenue;
        let spend_gdp = crate::programs::fiscal_spending_share(n,
            terms.social_spend + n.mil_spend_gdp + n.state_invest_gdp);
        let effective_rate =
            effective_interest_rate(n.interest_rate, n.inflation, n.debt_gdp);
        let interest_bn = n.debt_bn.map_or(0.0, |debt| debt * effective_rate);
        let interest_gdp = interest_gdp(n);
        Fiscal {
            revenue_gdp,
            spend_gdp,
            interest_bn,
            interest_gdp,
            effective_rate,
            balance_gdp: revenue_gdp - spend_gdp - interest_gdp,
        }
    }

    /// The same three quantities in billions of 1990 dollars a year.
    pub fn in_billions(&self, gdp: f64) -> (f64, f64, f64) {
        (self.revenue_gdp * gdp, self.spend_gdp * gdp, self.balance_gdp * gdp)
    }
}

/// Move `bn` billions out of a nation's finances, and say where it came from.
///
/// Out: the till first, and a till that would go negative issues debt for the
/// remainder -- the design's rule verbatim. In (a negative `bn`): debt is
/// retired first and only the surplus accumulates. The second half is not
/// decoration. `debt_gdp` is still what `politics.rs` consolidates against and
/// what `dyads.rs` reads for desperation, so a state that runs surpluses has to
/// be able to get its ratio DOWN; if income only ever piled up in the till,
/// debt would be monotonic and consolidation would be unreachable.
///
/// Debt cannot go negative here -- a surplus past the last bond becomes cash --
/// so `debt_gdp` stays non-negative and the browser's floor
/// (`a_nation_with_no_debt_is_not_shown_paying_it_down`) still holds unmoved. A
/// NET creditor is representable through `Nation::net_position_bn`, which is
/// where the design puts it, rather than through a negative ratio.
fn pay(treasury: f64, debt: f64, bn: f64) -> (f64, f64) {
    if bn >= 0.0 {
        let after = treasury - bn;
        if after < 0.0 {
            (0.0, debt - after)
        } else {
            (after, debt)
        }
    } else {
        let inflow = -bn;
        let repaid = inflow.min(debt);
        (treasury + (inflow - repaid), debt - repaid)
    }
}

/// ONE MONEY LEG out of one nation's finances, and the single helper every
/// hand-rolled ratio pusher now goes through. ~~the five~~ ~~NINE~~ — corrected
/// twice: 2026-09-02 on the merge of `origin/feat/hoi4-map-and-tech` 61b388f,
/// which added three legs written after this helper existed, and again the same
/// day when a review found TWO MORE that predate it and were never routed --
/// the debt write-down and the privatisation proceeds in `stratagems.rs`.
/// MEASURED on this tree, `economy::charge(` has ELEVEN call sites:
/// `resources::settle` (both sides), `apply_market_net`'s outflow and receipt
/// arms, `start_mine`'s construction investment, the pact upkeep and the aid
/// and covert legs in `statecraft.rs`, the patronage payment in
/// `government.rs`, and the two stratagems.
///
/// THE STRATAGEM PAIR IS WHY "single channel" IS A CORRECTNESS CLAIM AND NOT
/// TIDINESS. Both pushed `debt_gdp` by hand and left the stock alone, so for a
/// government that had opened its books the fiscal block recomputed the ratio
/// FROM the untouched stock on its next pass and the whole act was erased
/// inside a month. A bar in `spheres-sim/tests/treasury.rs` now holds that
/// module to this channel.
///
/// TWO REPRESENTATIONS OF THE SAME MONEY, and neither may be derived from the
/// other. `share` is EXACTLY the number the caller's pre-treasury line pushed
/// into `debt_gdp`, and is what the `None` arm writes -- so the default board
/// is bit-identical rather than nearly so. Recomputing it as `bn / gdp` would
/// be a rounding away from the shipped timeline at every one of those eleven
/// sites, because `(share * gdp) / gdp` is not `share` in binary floating
/// point. `bn` is the dollars, and is what the `Some` arm moves. The debt
/// write-down is the sharpest case: its `share` is `ratio * 0.55 - ratio`,
/// which Sterbenz makes exact and which the closed arm's addition undoes
/// exactly, where any recomputation from dollars would not.
///
/// The two arms are also where a real defect is repaired, and the defect is not
/// quite the one the design names. A money leg was divided by the PAYER's
/// output going out and by the PAYEE's coming in, which LOOKS like a leak and
/// is not: multiply each side's ratio move back by its own output and the same
/// `bn` comes out of both. Measured, and the mutation that would have proved
/// otherwise was run and left the conservation bar green.
///
/// THE LEAK IS THE FLOOR. `settle`'s payee line ended `.max(0.0)`, so a receipt
/// larger than the payee's debt DESTROYED the remainder: $10bn paid to Kuwait,
/// which owes $1.8bn, retired the debt and annihilated $8.2bn. That is money
/// leaving one state's books and reaching nobody, and it is what
/// `a_money_leg_between_unequal_economies_conserves` is pointed at. On the
/// `Some` arm `pay` retires the debt and keeps the remainder as cash, so the
/// pair conserves.
///
/// The `.max(0.0)` on the `None` arm is the floor `settle`'s payee line already
/// carried, and it is inert at EIGHT of the eleven sites (~~the other four~~,
/// ~~SEVEN of the nine~~, corrected twice 2026-09-02): the seven payments pass a
/// non-negative `share` against a `debt_gdp` that is non-negative by the fiscal
/// block's own floor, so the clamp cannot bind; and the debt write-down, though
/// a receipt, lands on `ratio * 0.55` with `ratio > 1.10` by its own
/// availability rule, so it cannot bind there either. The THREE where it can
/// bind are `settle`'s payee leg, `apply_market_net`'s receipt arm and the
/// privatisation proceeds, and on all three it is the arithmetic those lines
/// already shipped.
///
/// A negative `bn`/`share` is a receipt.
///
/// RETURNS THE UNABSORBED INFLOW: the dollars of a receipt that the nation's
/// debt could not soak up, and zero for any payment out. It is a report, not a
/// second money leg -- on the `Some` arm `pay` has already banked that
/// remainder in the till, so the number is only ever ACTED ON by a caller whose
/// books are closed and which therefore keeps a till of its own. That caller is
/// `resources::apply_market_net`, whose market-cash ledger is the pre-treasury
/// stand-in for exactly this till: while the books are closed the surplus lands
/// there, and while they are open the treasury has it already and the market
/// ledger is not touched. The TEN other sites use it as a statement and
/// nothing about their arithmetic moves (~~four~~, ~~EIGHT~~, corrected twice
/// 2026-09-02, the merge taking the call count from six to nine and the
/// stratagem routing to eleven). The privatisation proceeds are the one place
/// where discarding it is a JUDGEMENT rather than a no-op: a closed-books
/// state that raises more than it owes loses the surplus at the floor, exactly
/// as the shipped ratio line did, while an open-books one banks it in the till
/// -- which is the same repair the payee floor got, arriving by the same route.
///
/// Both arms measure the same quantity against the same denominator the callers
/// use: `debt` in dollars on the `Some` arm, and `debt_gdp.max(0.0) * gdp` on
/// the `None` arm, which is character for character the `debt_bn` that
/// `apply_market_net` computed for itself before this became one channel.
pub fn charge(w: &mut WorldState, id: NationId, bn: f64, share: f64) -> f64 {
    let n = w.nation_mut(id);
    match (n.treasury_bn, n.debt_bn) {
        (Some(treasury), Some(debt)) => {
            let unabsorbed = (-bn - debt).max(0.0);
            let (treasury, debt) = pay(treasury, debt, bn);
            n.treasury_bn = Some(treasury);
            n.debt_bn = Some(debt);
            n.debt_gdp = debt / n.gdp.max(0.1);
            unabsorbed
        }
        _ => {
            let unabsorbed = (-bn - n.debt_gdp.max(0.0) * n.gdp.max(0.1)).max(0.0);
            n.debt_gdp = (n.debt_gdp + share).max(0.0);
            unabsorbed
        }
    }
}

/// Every term of one nation's annual growth rate, and the price impulse that
/// travels with it.
///
/// ONE DEFINITION, WHICH IS THE ENTIRE POINT OF THIS TYPE. `tick` builds a
/// nation's year out of these fields, and so does the browser's policy panel,
/// which until now kept its own copy of the arithmetic in JavaScript under a
/// comment claiming the copy "computes nothing the sim does not". By the time
/// anybody checked, the copy was missing: the whole net-of-replacement shape of
/// the capital arm and the `0.030` floor that was deleted from it, the labour
/// term entirely, all three gates on the demand arm (`room_to_cut`,
/// `willing_to_borrow` and `money_works`), `MAX_DEMAND_GAP`, `MAX_OIL_SHARE`,
/// `tech::energy_exposure`, the bubble, and `WORST_ANNUAL_COLLAPSE`. That is
/// what a mirror is worth after a few passes of repair, and it is why the
/// browser is now handed these fields rather than trusted to rebuild them.
pub struct GrowthTerms {
    /// Trend plus capital plus catch-up plus labour, net of the command-economy
    /// allocation penalty.
    pub potential: f64,
    /// The cyclical gap, ungated — this is the arm that sets prices.
    pub demand_gap: f64,
    /// The same gap as OUTPUT, after `money_works`. The two come apart in a
    /// hyperinflation, which is the whole content of the split.
    pub demand_output: f64,
    pub bubble: f64,
    pub oil: f64,
    /// Oil income as a share of output, capped at `MAX_OIL_SHARE` — the budget
    /// reads this too, so the ledger and the growth line cannot disagree.
    pub oil_revenue_gdp: f64,
    pub energy_exposure: f64,
    pub embargo: f64,
    pub sanctions: f64,
    pub war: f64,
    pub debt: f64,
    pub unrest: f64,
    pub oil_inflation: f64,
    /// The rate inflation is being pulled toward this month.
    pub target_inflation: f64,
    /// The one line of the budget the player does not set with a slider.
    pub social_spend: f64,
    /// What oil puts into the budget: `oil_revenue_gdp * 0.55` for a producer
    /// and nothing for anybody else. Carried so that the browser's ledger adds
    /// up the player's own three sliders and two numbers from here, rather than
    /// keeping a copy of the rule about who counts as a producer.
    pub budget_oil_revenue: f64,
    /// The floor `tick` puts under a year. Carried here rather than exported as
    /// a constant, so the one other place that has to assemble this sum is
    /// handed the bound instead of copying its literal.
    pub floor: f64,
    /// The assembled annual rate before `tick`'s RNG draw and before the floor.
    pub before_noise: f64,
    /// The same sum floored, and therefore the honest forecast: everything the
    /// sim will charge except a draw it has not made.
    pub growth: f64,
}

/// One nation's year, as a pure function of the nation, the policy it is
/// running, and what the world is doing to it.
///
/// `state_invest` and `interest_rate` are arguments rather than fields read off
/// `n`, so that the same function answers both "what is happening" — which is
/// what `tick` asks, passing the nation's own two — and "what would happen if I
/// moved this slider", which is what the policy panel asks.
pub fn growth_terms(
    n: &Nation,
    state_invest: f64,
    interest_rate: f64,
    c: &Conditions,
) -> GrowthTerms {
    let state_invest = crate::programs::effective_investment(n, state_invest);
    let oil_price = c.oil_price;
    let sanction_share = c.sanction_share;
    let at_war = c.at_war;
    let export_share = c.export_share;
    let budget_gap: [f64; BUDGET_MINISTRIES] =
        std::array::from_fn(|i| n.budget_gap(i));
    {
    // ---- Investment & potential growth (annual rates) ----
    let invest = state_invest + n.priv_invest_gdp;
    let gdp_pc = n.gdp * 1000.0 / n.population; // $ per capita
    let dev = (gdp_pc / 24000.0).min(1.0);
    // ===================================================================
    // THE CAPITAL CHANNEL. Repaired 2026-08-31 on Ridge's ruling 1 of that
    // date, taken with the measurement below in front of him. Recorded here
    // the way §8 records the trade-pact correction it cites, because this is
    // the same error class caught twice in a row and the second catch is the
    // one that is easy to get wrong.
    //
    // WHAT CAPITAL ACCUMULATION ACTUALLY DOES TO OUTPUT, WRITTEN AS THE ONE
    // IDENTITY IT IS. With Y = K^α (A·L)^(1-α), rearranging to put the
    // capital-output ratio on the right gives
    //
    //      ln Y = α/(1-α) · ln(K/Y) + ln(A·L)
    //
    // so the whole of what capital does to output is α/(1-α)·ln(K/Y), and
    // the only question this file has to answer is what moves K/Y. K/Y is a
    // STOCK ratio and it has its own law of motion,
    //
    //      d(K/Y)/dt = s - (δ + g)·(K/Y)
    //
    // in which the investment share s is the FLOW INTO the stock and not the
    // stock. Both defects fixed in this pass — this arm, and the level block
    // beside `CAPITAL_ELASTICITY` below — are the same mistake: the code
    // priced the flow s as though it were the stock K.
    //
    // WHAT THIS ARM IS, AND WHAT IT IS NOT. It is deliberately NOT α·ĝ_K.
    // Paying α·ĝ_K here would double-count, because `tfp_trend` and
    // `catchup` below are reduced-form TREND terms for a developing economy
    // rather than a pure Solow residual — measured on the shipped board
    // China draws 3.9 points of `tfp` and 1.9 of `catchup`, which is already
    // most of a growth rate, and α·ĝ_K on top of that would be the same
    // growth counted twice. What this arm is is the rate at which a nation
    // converts capital formation into closing its development gap: the gap
    // is `(1 - dev)`, and how fast it closes depends on how much NEW capital
    // the nation actually lays down. That reading is why the `(1 - dev)`
    // gate belongs here and why the arm is not BIBLE §8's error class — it
    // expires on its own as a nation develops, so it is transitional, and a
    // transitional rate paid on a gap that closes is a level in disguise
    // rather than a permanent rate.
    //
    // ONLY THE TRANSITIONAL ARM SURVIVES, AND THAT PART OF THE PREVIOUS PASS
    // STANDS. `0.030` was a floor that never expired: `dev` is pinned at 1.0
    // for every mature economy from month one and `invest` never moves, so
    // it was a per-nation CONSTANT paid as a growth rate for 420 months —
    // 0.5576 pt/yr to the United States and 0.7360 to Japan, flat to four
    // decimals across all four decades. A constant investment share buys a
    // permanently larger economy and no permanent growth at all. It was
    // ordered against reality besides, handing Italy (0.587) more than the
    // United States (0.554). Removing it was right and it is not coming
    // back; what follows repairs the arm that was left, not that one.
    //
    // THE DEFECT IN THE ARM THAT WAS LEFT IS ITS SHAPE, NOT ITS SIZE.
    // `intensity = (s/0.20)^0.55 · 0.20` HAS NO ZERO. It is strictly
    // positive for every s > 0, so a nation putting 4% of its output into
    // capital was paid capital deepening it was not doing — its capital
    // stock was shrinking. And because the concavity is applied to the whole
    // share rather than to the part of it that is new capital, a 30% economy
    // was paid only 25% more than a 20% one, as though the first twelve
    // points of investment — the ones that merely replace what wore out —
    // were buying growth. They are not. Capital only deepens above the
    // replacement line, and the whole of the difference between a 30%
    // economy and a 20% one is the NET investment above that line: 17.5
    // points against 7.5, a factor of 2.3 rather than 1.25.
    //
    // That is also the answer to the thing the ruling names. A share that
    // drifts 0.300 -> 0.261 is still laying down capital at five times the
    // rate the stock wears out; the channel must not read a mild decline in
    // a high share as a nation destroying capital, and with the zero in the
    // right place it does not — the decline is priced against the 0.175 of
    // net investment that is actually there, not against the 0.300 of gross.
    //
    // WHERE THE LINE IS, DERIVED FROM CONSTANTS THIS FILE ALREADY HOLDS
    // RATHER THAN CHOSEN. `CAPITAL_ELASTICITY` below fixes α/(1-α) = 0.49,
    // and the level block's reference investment share is 0.20. Read that
    // reference back through the same balanced path the level block is built
    // on: a nation at s = 0.20, depreciating at the standard aggregate 5% a
    // year against a 3% trend, holds K/Y = s/(δ+g) = 0.20/0.08 = 2.5 years
    // of output. Replacing 5% of that costs δ·(K/Y) = 0.125 of output every
    // year before one unit of new capital exists. So the replacement share
    // is 0.125, and it is 0.20 and δ restated rather than a new number.
    //
    // AND THIS IS A RESHAPE, NOT A RESCALE — the precedent is `intensity`
    // itself, which was introduced "the same at that point, worth
    // progressively less above it". `net_intensity` is normalised to equal
    // the old `intensity` EXACTLY at s = 0.20, and the 0.080 coefficient
    // beside it does not move by a bit. What changed is the slope through
    // that point, and the slope is what the physics fixes. Nothing here was
    // swept until China read 9.2%: the reference value is the shipped value
    // to the last bit, and the shipped board's China lands where it lands.
    //
    // BOUNDED, at the place where it is computed, in the sense the
    // no-mirroring-ceiling ruling below asks for: s >= 0 makes
    // `net_intensity` >= -0.3333 and the arm >= -2.67 pt/yr, reached only by
    // a nation investing literally nothing, and it is gated to zero at the
    // frontier besides.
    const REPLACEMENT_SHARE: f64 = 0.125; // δ·(K/Y) = 0.05 × 2.5 at the 0.20 reference
    let net_intensity =
        (invest.max(0.0) - REPLACEMENT_SHARE) * (0.20 / (0.20 - REPLACEMENT_SHARE));
    let invest_effect = net_intensity * 0.080 * (1.0 - dev);
    // Income convergence: capital deepening and the reallocation of labour
    // out of subsistence, which is most of what makes a poor country grow
    // fast. This is *not* the technological diffusion the tech tree models,
    // and the two are not one effect counted twice — that was the argument
    // for deleting this line, and it was wrong. Diffusion governs what it
    // costs to acquire a technology; this governs what happens when a
    // country moves its people from a field to a factory. A nation can hold
    // most of the frontier's technologies and still be ten times poorer per
    // head, which is the gap this closes and the tree does not. Deleting it
    // collapsed emerging growth, and it is staying.
    //
    // AND IT IS STAYING AT 0.020. This is the coefficient the roster
    // expansion was expected to have invalidated — `tech::tick` sizes
    // affordability against world GDP, world GDP rose 18% when the roster
    // went 31 -> 108, so every coefficient fitted underneath it was
    // suspect. Re-measured at 108 nations, it was not. Take the wars away
    // with `ai_aggression = 0.0` and let China simply grow for thirty years:
    //
    //      catchup 0.000 -> peaceful China  8.16x
    //      catchup 0.010 -> peaceful China 10.90x
    //      catchup 0.020 -> peaceful China 14.02x   <- real 14.33x
    //      catchup 0.030 -> peaceful China 18.38x
    //
    // 0.020 lands within 2% of the World Bank series at the fuller roster,
    // which is a better fit than it had any right to be and is not a number
    // to move. The ten-seed median that `china_growth_miracle` reads is
    // pinned in both directions at this roster too — 9.59x at 0.010 and
    // 19.77x at 0.030, both outside the 11.0-19.0 band — so the test does
    // constrain this line; it simply was not this line that broke.
    // SANCTION_BITE above is what the roster actually moved.
    let catchup = (1.0 - dev) * 0.020;
    // Growth accounting: output growth is productivity, plus capital's
    // contribution, plus labour's share of the change in the workforce. A
    // shrinking workforce is a headwind no amount of investment offsets,
    // which is the fact about Japan the model was missing entirely.
    let labour = population_growth(n) * 0.60;
    let mut potential = n.tfp_trend + invest_effect + catchup + labour;
    // NO MINISTRY REACHES POTENTIAL GROWTH DIRECTLY, and the ministry collapse
    // removed the five that did — health 0.015, education 0.050,
    // infrastructure 0.025, industry 0.035 and science 0.025. The money a
    // ministry spends already reaches growth through `invest_effect` above,
    // because `investment_total` is the state investment envelope; a second
    // addend here was the same dollar counted twice, and it was the largest of
    // the thirty uncalibrated channels BUGS D-1 filed.

    // Command economies pay an allocation penalty that worsens as they develop
    if n.system == EconomySystem::Command {
        potential -= 0.004 + 0.010 * (gdp_pc / 24000.0).min(1.0);
    }

    // ---- Demand side ----
    // Real rate vs neutral moves demand around potential
    let real_rate = interest_rate - n.inflation;
    let neutral = 0.025;
    let mut demand_gap = (neutral - real_rate) * 0.55; // easy money -> above potential
    // NO MINISTRY REACHES DEMAND. Health 0.06, housing 0.28 and pensions 0.18
    // were removed here: `demand_gap` FORKS, into output above and into the
    // price impulse below, so a ministry routed through it is charged to two
    // aggregates at once and can be read neither way. Pensions' claim on the
    // labour market is now made where it can be read, in `unemployment_rate`.
    //
    // `demand_gap += (n.social_spend() - n.baseline_social_spend()) * 0.15`
    // STOOD HERE AND IS GONE, and it is why the paragraph above was false when
    // it was written. It is the exact aggregate twin of the `ds += social_gap *
    // 12.0` already deleted from the stability block for the same reason: the
    // real `Command::SetAnnualBudget` writes `social_spend_gdp =
    // plan.social_total()`, and `social_total` sums HEALTH, EDUCATION, HOUSING,
    // PENSIONS, SECURITY and DIPLOMACY -- so the three per-ministry demand
    // addends the collapse removed were replaced by one shared addend that fired
    // for all six, at an effective slope of 0.15 for each, and it handed
    // EDUCATION, SECURITY and DIPLOMACY a demand arm the design never gave them.
    //
    // MEASURED before removing it (Brazil, seed 1990, +0.005 of GDP on one dial
    // through the real command): every one of the six moved `demand_gap` by
    // +0.000750000, `target_inflation` by +0.001200000 and before-noise growth
    // by +0.000018011, identical to the last digit, and twelve months on all six
    // carried +0.001348661 of inflation. INFRASTRUCTURE, INDUSTRY and SCIENCE
    // moved demand and inflation by exactly 0.000000000 (they move growth by
    // +0.000951904 through `investment_total`, which is the design's own route),
    // and DEFENSE moved nothing. After removing it all ten read 0.000000000 on
    // demand and inflation.
    //
    // INERT ON EVERY DEFAULT PATH, and the reason is the same one the stability
    // twin carried: `social_spend_gdp` is `None` for all 137 nations at 1990 and
    // `social_spend()` then returns `baseline_social_spend()` by definition, so
    // the addend was exactly 0.0 and `x += 0.0` is exact. Its only two writers
    // are player commands (`lib.rs:550` and `lib.rs:607`).
    // ...but only while there is a rate left to cut and somebody willing to
    // borrow. Pushing on a string: Japan ran the policy rate at zero for two
    // decades against a corporate sector repairing its balance sheet, and
    // got almost no growth for it. The naive rule reads that same zero rate
    // as a permanent stimulus, which is why Japan kept compounding at 3% in
    // a model that was otherwise behaving.
    if demand_gap > 0.0 {
        let room_to_cut = (interest_rate / 0.04).clamp(0.0, 1.0);
        let willing_to_borrow = 1.0 - (-n.bubble).clamp(0.0, 1.0) * 0.75;
        demand_gap *= 0.25 + 0.75 * room_to_cut.min(willing_to_borrow);
    }
    // BOUNDED, AND BOUNDED SYMMETRICALLY. The bust side already had a bound
    // and nobody had noticed it was one-sided.
    //
    // Both of this term's inputs are clamped elsewhere, and they are clamped
    // ASYMMETRICALLY. The policy rate cannot exceed 0.60 (`Command::SetRate`
    // in lib.rs; the AI's Taylor rule stops at 0.45) and inflation cannot
    // fall below -0.05 (the clamp forty lines down), so the deepest gap the
    // bust side can reach is `(0.025 - (0.60 + 0.05)) * 0.55 = -0.344`. Going
    // the other way there is no matching pair: the rate floor is 0.0 but the
    // inflation CEILING is 3.0, so the boom side reaches
    // `(0.025 + 3.0) * 0.55 = +1.664`, and a hyperinflation — which destroys
    // output — was being read by this line as +166% a year of demand-led
    // growth. Measured across all nations, ten seeds, thirty-five years, it
    // reached +0.784 against a bust-side worst of -0.160.
    //
    // WHY SYMMETRY IS THE PRINCIPLE AND NOT A NUMBER I PICKED. A demand gap
    // is by definition a deviation from potential; the `INFLATION_ANCHOR`
    // comment above spends a paragraph establishing that it must therefore
    // average zero, and that is the whole reason politics.rs's 0.025 moved.
    // A zero-mean term whose two tails are bounded at different distances is
    // biased by construction — the asymmetry IS a mean. So the bound has to
    // be the same on both sides, and once that is settled the size is not a
    // choice either: it is read off the side that already had one. 0.35 is
    // the bust-side bound of -0.344 rounded away from zero, so this clamp
    // cannot bind on the bust side and changes nothing there; it binds only
    // where nothing was binding before.
    //
    // Like `WORST_ANNUAL_COLLAPSE` below, this is a guard and not a policy:
    // ±35 points of annual growth from the cyclical term alone is far outside
    // anything this model is meant to produce, and the 99th percentile of the
    // whole board is +0.0067.
    const MAX_DEMAND_GAP: f64 = 0.35;
    let demand_gap = demand_gap.clamp(-MAX_DEMAND_GAP, MAX_DEMAND_GAP);

    // ---- ONE DEMAND GAP WAS DOING TWO JOBS, AND AT HYPERINFLATION THE TWO
    // JOBS HAVE OPPOSITE SIGNS. ----
    //
    // `demand_gap` above is used twice: it is added to `growth_annual` as
    // real output, and it is multiplied by 1.6 into `target_infl` forty lines
    // down as the price impulse. Those are the same object in a normal
    // cycle — a boom raises output and prices together, a slump lowers both —
    // and they come apart completely at the top of the range, which is
    // precisely where this model spends the first half of the nineties in a
    // third of the roster. Monetary financing of a deficit raises the price
    // level with full force and raises output not at all. That is what
    // stagflation IS, and the model could not express it, because it had one
    // variable where the world has two.
    //
    // Read as output, the term was backwards. A real rate of -70% is not
    // cheap credit; it is what a lending market looks like after it has
    // stopped existing. Nobody lends at -70% real, so nothing is financed at
    // that rate and the gap it implies is never collected. Reading the rate
    // alone, the model took the deepest monetary collapse of the period for
    // the largest stimulus of the period: over the nineties, ten seeds, this
    // line paid Kazakhstan +3.55, Belarus +3.11, Ukraine +2.72 and Russia
    // +1.38 points of annual growth, and in the opening months it was pinned
    // against MAX_DEMAND_GAP above — **+35 points a year** — in the same
    // year those economies lost a seventh of their output. That is BIBLE §8's
    // error class with the sign inverted: a permanent RATE paid out for a
    // one-time LEVEL destruction, and paid to the nation it happened to.
    //
    // So `money_works` gates the OUTPUT arm and only the output arm. The
    // price arm above is untouched, which is not a convenience: it is the
    // whole content of the split. `brazil_grinds_down_its_hyperinflation`
    // asserts that 1990 Brazil is still burning above 50% eighteen months
    // later and tamed by 1999, and that behaviour is carried entirely by the
    // price arm's feedback against a policy rate the Taylor rule clamps at
    // 0.45 — inflation cannot be caught until it falls below the ceiling,
    // and then it can. Gating both arms together broke that test in all ten
    // seeds and it was right to break: it is the detector for exactly this
    // confusion, and the fix is to stop conflating the two channels rather
    // than to weaken the one that was correct.
    //
    // ONLY THE POSITIVE SIDE. Tight money in a high-inflation country really
    // does crush output — the stabilisation recession is the most reliable
    // fact in this literature — so the bust arm passes through at full
    // strength and is not multiplied here.
    //
    // THE COEFFICIENT IS A THRESHOLD AND IT IS NOT MINE. 0.40 is the
    // conventional high-inflation line: Bruno and Easterly (1998) define a
    // "high inflation crisis" as an episode above 40% a year, and Fischer,
    // Sahay and Vegh use the same cut for the transition economies. It is
    // where indexation stops keeping up and contracts start shortening.
    //
    // AND IT IS A THRESHOLD, WITH A DEAD ZONE, BECAUSE ITS TWO SIBLINGS ARE.
    // `room_to_cut` is exactly 1.0 at any policy rate above 4% and bites only
    // below it; `willing_to_borrow` is exactly 1.0 for any non-negative
    // bubble and bites only below it. Both are silent in a normal economy and
    // speak only in the pathology they are about. The first draft of this one
    // was `1/(1 + (pi/0.40)^2)`, which is 0.998 at the anchor and 0.88 at
    // 15% — a term with no dead zone at all, quietly taxing every economy
    // that was merely warm, and using the 40% line as a half-power point
    // rather than as the threshold it is cited for. It reached Iraq at 18%
    // inflation, where nothing about the dinar had stopped working, and the
    // Gulf War rate fell from 22/40 seeds to 17/40 against a bar of 20.
    //
    // Subtracting one inside the square makes the citation mean what it says:
    // money works, exactly and entirely, until the crisis line, and degrades
    // past it. 1.000 up to 40%; 0.88 at 55%; 0.39 at 90%; 0.25 at 110%;
    // 0.087 at 170%; 0.024 at 295%. Bounded in (0, 1], monotone, one
    // constant, and the constant is not free to move — it is a citation.
    //
    // IT CANNOT REACH THE MATURE PANEL, and that is measured rather than
    // asserted — `money_works_reach` in tests/growth_decomposition.rs prints
    // the whole roster sorted by what it costs them. All six mature economies
    // sit at exactly 1.0. What it reaches is Yugoslavia, the Latin American
    // hyperinflations and the post-Soviet republics in descending order of
    // their transcribed opening inflation, which is the list of currencies
    // that actually failed in this period, and nothing else.
    //
    // `powi(2)` and not `exact::powf(x, 2.0)`: this runs for every living
    // nation every month, and an integer power is exact multiplication.
    const MONEY_FAILS: f64 = 0.40;
    let past_crisis = (n.inflation.max(0.0) / MONEY_FAILS - 1.0).max(0.0);
    let money_works = 1.0 / (1.0 + past_crisis.powi(2));
    let demand_output =
        if demand_gap > 0.0 { demand_gap * money_works } else { demand_gap };

    // Bubble dynamics: hot bubbles add demand until they pop.
    //
    // THE BOOST IS READ HERE AND THE STOCK IS MOVED IN `tick`. Both arms
    // always read `n.bubble` as the month found it and only then wrote to
    // it, so separating the read from the write changes no value; what it
    // buys is that this function touches nothing, which is what lets the
    // policy panel ask it what a slider would do.
    let bubble_boost = if n.bubble > 0.0 {
        n.bubble * 0.012
    } else if n.bubble < 0.0 {
        // Balance-sheet recession. A decade was too kind: Japan's asset
        // prices peaked in 1989 and the corporate sector was still paying
        // down debt rather than investing twenty years later, which is why
        // the lost decade is properly the lost decades. Roughly twenty years
        // to heal, not nine.
        n.bubble * 0.022
    } else {
        0.0
    };

    // ---- Drags ----
    // DEFENSE HAS NO GAP ARM ANYWHERE IN THIS FILE OR ANY OTHER, AND THAT IS
    // THE DESIGN. Its allocation IS `mil_spend_gdp` — the priced aggregate the
    // force model, the arsenal and `sustained_force` all already read — so a
    // gap arm on top of it would charge the same money a second time. If a
    // later session notices that nine ministries have a named arm and defense
    // does not, this comment is the answer: defense is the one ministry whose
    // whole effect was already modelled before the ministries existed.
    //
    // DIPLOMACY'S FIRST NAMED ARM, kept and unchanged: the sanction shield.
    let diplomatic_shield = crate::ministries::diplomacy_shield(budget_gap[BUDGET_DIPLOMACY]);
    let sanction_drag = growth_drag_of_sanctions(sanction_share) * (1.0 - diplomatic_shield);
    let war_drag = if at_war { 0.020 + n.war_exhaustion * 0.03 } else { 0.0 };
    // DEBT IS CHARGED ONCE, and which charge a nation pays depends on whether
    // it keeps books.
    //
    // This drag is INHERITED and was calibrated when the model had no interest
    // at all: it was the only thing that made debt cost anything, so it had to
    // stand in for debt service, crowding-out and the risk premium together.
    // The treasury now charges the real thing on the `Some` arm of the fiscal
    // block below -- `books.interest_bn / 12.0` in cash, struck at the
    // escalating `effective_interest_rate` -- and `SPREAD_KNEE`'s derivation
    // reasons the 0.60 knee against `dyads.rs` and `politics.rs` without ever
    // noticing this line, so the spread was sized as the market's whole charge
    // for a ratio the model was already charging for here.
    //
    // MEASURED this run, Brazil with the books open at debt_gdp 1.20 (policy
    // rate 2.90, inflation 2.95, both hyperinflationary 1990 figures):
    // effective_rate 0.016000, cash interest_gdp 0.019200 a year, debt_drag
    // 0.006000 of growth. Paying both is 0.025200 against the 0.006000 the same
    // nation pays with the books closed -- 4.20x, which is a double charge and a
    // player-versus-AI asymmetry in one.
    //
    // So the drag is the CLOSED-BOOKS representation of debt service and the
    // cash charge is the open-books one. Gating it here leaves every AI nation
    // and the whole default board bit-identical -- `debt_bn` is `None` for all
    // 137 nations until a player issues a budget -- and stops the one nation
    // that opened its books being billed twice for the same debt.
    let debt_drag = if n.debt_bn.is_none() && n.debt_gdp > 0.9 {
        (n.debt_gdp - 0.9) * 0.02
    } else {
        0.0
    };
    let instability_drag = if n.stability < 40.0 { (40.0 - n.stability) * 0.0009 } else { 0.0 };

    // ---- Oil terms of trade ----
    // Producers gain when oil is dear — but only on the barrels they can
    // actually ship. An embargoed producer watches the price it caused spike
    // while its own revenue collapses.
    // Capped, for the same reason `embargo_drag` below is capped and with
    // more force: this is a ratio whose own denominator it then shrinks.
    // Oil income is a *share* of output, and a share that runs away is not a
    // petro-boom, it is a division by a collapsing GDP. The ceiling is set
    // from what governed play actually reaches — over three seeds and
    // thirty-five years the highest any live producer saw was 1.25 (Kuwait
    // in 1994, its output wrecked and its wells relit), the 99.9th
    // percentile 0.71, the median 0.038. At 2.0 this never binds on a
    // working economy. The runaway it stops had reached 65,700.
    const MAX_OIL_SHARE: f64 = 2.0;
    let oil_revenue_gdp =
        (n.oil_mbd * export_share * oil_price * 0.365 / n.gdp).min(MAX_OIL_SHARE); // $bn/yr per $bn GDP
    // An importer that has learned to do more with a barrel is hurt less by
    // what the barrel costs.
    let exposure = crate::tech::energy_exposure(n);

    // ===================================================================
    // KNOWN DEFECT, DIAGNOSED AND MEASURED, NOT SHIPPED. READ THIS BEFORE
    // TOUCHING THE PRODUCER ARM BELOW — IT IS WRONG, AND THE FIX IS BLOCKED
    // ON AN OWNER RULING, NOT ON ANYBODY WORKING OUT WHAT IT IS.
    //
    // THE DEFECT. The producer arm enters `growth_annual` as an annual RATE,
    // charged every month the price sits above $20 — which is 82% of all
    // months across ten seeds and thirty-five years. Its reachable maximum is
    //
    //     (120 - 20)/20 * MAX_OIL_SHARE * 0.5  =  5.0 * 2.0 * 0.5  =  5.0
    //
    // i.e. **+500% a year, compounded, for as long as the price stays high**.
    // Measured on the live board it pays the four large producers +0.96 pt/yr
    // on average, forever, for a price that has simply not fallen back. That
    // is BIBLE §8's error class — a permanent RATE for a one-time LEVEL
    // change — and it is the fourth instance of it in this file after
    // `invest_effect` above, the labour term, and the trade pacts §8 records.
    // A barrel that is dear this month is dear again next month; that is the
    // SAME windfall arriving again, not a second one.
    //
    // WHAT IT SHOULD DO. A terms-of-trade improvement raises real gross
    // domestic INCOME by roughly (Δp/p) × (oil exports as a share of output)
    // — once. Holding $40 forever does not make a petro-state grow 25% a year
    // forever; it makes it permanently richer by about one windfall and then
    // returns it to trend. So it is a LEVEL, paid the way `capital_level_paid`
    // below and `statecraft::trade_level_gain` are paid: on the CHANGE in the
    // entitlement, not on the entitlement itself. Symmetric rather than a
    // high-water mark, because an oil bust really does take the income back —
    // Kuwait in 1986 and 1998 lost the level, it did not merely stop gaining.
    //
    // THE FIX WAS WRITTEN AND MEASURED. `oil_windfall = (p-20)/20 *
    // oil_revenue_gdp * 0.5` as an entitlement, `ln(1 + oil_windfall)` paid in
    // at 0.04/month against a per-nation `oil_level_paid` tracker, exactly
    // parallel to the capital block below, with the producer arm here set to
    // zero. It is arithmetically correct and it is NOT SHIPPED, because on
    // ten seeds it costs:
    //
    //     china_growth_miracle   11.72x -> 10.86x   RED (floor 11.0x)
    //     mature panel Spearman   0.943 -> 0.771
    //     ordering clause DE < UK  true -> FALSE  (UK 1.90 -> 1.80)
    //     Nigeria 35-yr multiple  11.12x -> 8.21x
    //
    // Buying any of that back means moving a bar or tuning a coefficient, and
    // iron rule 5 forbids both. So it stops here and the owner rules.
    //
    // WHY THE HONEST LEVEL CANNOT REPLACE THE RATE, which is the actual
    // finding and is worth more than the patch. The level is right and it is
    // SMALL: China's oil is 5.2% of its 1990 output, so a 5% price rise is
    // worth 0.125% of output once. The rate was paying that same 0.125% EVERY
    // YEAR FOR THIRTY YEARS. Removing it does not reveal a mis-sized level —
    // it reveals that producers had been living on the rate, and it exposes
    // two residuals this file cannot close:
    //
    //   1. China's growth is already ~0.8pt short of reality (PLAN step 7,
    //      "named, not closed with a coefficient"). The oil rate was masking
    //      part of that shortfall, which is the same thing the previous pass
    //      found for `invest_effect` and `demand_gap`.
    //   2. `oil_market` below has no boom in it. Its target is
    //      `20 * (1 + disruption * 4)` and disruption is capped at 0.6, so the
    //      price reaches $38 over thirty-five years against a real 1990-2025
    //      range of roughly $10-$140. A model whose oil price never booms
    //      cannot pay a producer a real windfall LEVEL, and the runaway rate
    //      was silently standing in for the boom the market cannot produce.
    //      Fix the oil market first, then this.
    //
    // Until then the line below is the shipped behaviour, unchanged, and it
    // is wrong.
    // ===================================================================
    let oil_effect = if n.oil_mbd > 0.5 {
        (oil_price - 20.0) / 20.0 * oil_revenue_gdp * 0.5
    } else {
        // The importer arm is a different animal and is not part of the
        // defect above: an expensive barrel is a standing tax on a country
        // that has to buy one every month, not a one-off transfer. It is
        // bounded too — the price is clamped to [8, 120] and
        // `energy_exposure` to a small factor — and it averaged -0.046 pt/yr
        // over the mature panel. It is also the arm that reaches Italy, whose
        // floor margin on `mature_economies_do_not_run_hot` is +0.0007, so it
        // is not to be touched outside a calibration pass.
        -(oil_price - 20.0) / 20.0 * 0.006 * exposure
    };
    // Barrels that never ship are income that never arrives — the hard edge
    // of an embargo for a petro-economy.
    // Capped: this is a ratio to a shrinking GDP, and an uncapped version
    // would feed on its own collapse.
    let embargo_drag = if n.oil_mbd > 0.5 {
        ((1.0 - export_share) * n.oil_mbd * oil_price * 0.365 / n.gdp * 0.30).min(0.12)
    } else {
        0.0
    };

    // Real output can collapse; it cannot invert. The multiplicative form
    // carries an unstated assumption — that the monthly factor stays
    // positive — and nothing enforced it, so a growth rate under -1200%/yr
    // walked GDP through zero in a single step. That is not an economy
    // shrinking fast, it is arithmetic with no floor, and everything
    // downstream inherits it: war.rs square-roots the budget, mil_strength
    // becomes NaN, serde writes NaN as `null`, and the browser UI dies on it.
    //
    // Bounding the *rate* rather than clamping the result is what makes
    // `gdp > 0` provable instead of merely patched: 1 + (-0.95)/12 = 0.921 is
    // positive for every input, so a positive GDP stays positive by
    // induction, and this is the only site in the sim that scales a living
    // nation's GDP. Clamping the *result* instead — which is what the first
    // pass at this did — leaves an economy pinned at the floor while the
    // terms that drove it there go on compounding, which reads as "finite
    // and positive" to a test and as a $120bn United States holding 100.00
    // stability to a player.
    //
    // -0.95 is a guard, not a policy. The worst real collapse of the period
    // is Kuwait in 1991 at roughly -40%, so it never binds on anything this
    // model is meant to produce.
    // NO MIRRORING CEILING, AND THAT IS A RULING RATHER THAN AN OMISSION.
    // PLAN step 7 asks for a growth ceiling mirroring this floor. It should
    // not exist, for three reasons, and the third is the one that decides it.
    //
    // 1. THE FLOOR IS NOT A SAFETY NET, IT IS A PROOF. It exists because
    //    `1 + g/12` goes NEGATIVE below g = -12, and the paragraph above
    //    spends itself establishing that `gdp > 0` is then provable by
    //    induction rather than patched. There is no matching singularity
    //    above: `1 + g/12` is positive for every g > -12 and stays positive
    //    however large g gets. A ceiling would prove nothing, so it is not a
    //    mirror of this constant — it only looks like one.
    //
    // 2. EVERY POSITIVE TERM IN THE SUM IS ALREADY BOUNDED, and each by
    //    something that means what it says: `tfp_trend` reverts to
    //    FRONTIER_TFP, `invest_effect` and `catchup` carry a `(1 - dev)` that
    //    is zero at the frontier, `labour` is bounded by `transition`,
    //    `bubble_boost` by |bubble| <= 1, `demand_gap` by MAX_DEMAND_GAP
    //    above, `noise` by ±0.004. The one exception is the producer arm of
    //    `oil_effect`, which reaches +500%/yr — and see 3.
    //
    // 3. A CEILING WOULD HIDE EXACTLY THE BUG THIS STEP WAS SENT TO FIND. A
    //    clamp at, say, +0.25 would take the +500%/yr producer arm off every
    //    instrument in the suite while leaving it in the arithmetic: the
    //    petro-states would pin against the clamp, `the_frontier_does_not_run
    //    _away` would stay green, and the term would read as "finite and
    //    positive" to every test while being wrong by two orders of
    //    magnitude. That is the same failure the paragraph above records for
    //    clamping the RESULT instead of the rate, and it is why the audit
    //    that reviewed this asked for the question to be ruled on rather than
    //    implemented. The answer to an unbounded term is to bound the TERM,
    //    at the place where it is computed and where a comment can say why —
    //    which is what MAX_DEMAND_GAP does above and what the producer arm
    //    still needs.
    //
    // The detector the sweep is missing is a MEASUREMENT, not a clamp:
    // `peak_sustained_growth` in tests/growth_decomposition.rs reads the
    // quantity a runaway term actually moves.
    // `demand_output`, not `demand_gap`: the output arm of the demand term,
    // gated by whether there is still money. The price arm keeps the
    // ungated `demand_gap` — see the split above.
    // `demand_output`, not `demand_gap`: the output arm of the demand term,
    // gated by whether there is still money. The price arm keeps the
    // ungated `demand_gap` — see the split above.
    //
    // THE NOISE IS NOT HERE, and the floor that goes with it is applied
    // twice on purpose. `tick` adds its draw and floors the result; this
    // floors the same sum without one, because a forecast cannot know a
    // number the sim has not rolled yet and must not invent one. The
    // addition is associative-identical: `noise` was always the last
    // addend, so `before_noise + noise` is bit-for-bit what this line used
    // to compute.
    let before_noise = potential + demand_output + bubble_boost + oil_effect
        - sanction_drag - war_drag - debt_drag - instability_drag - embargo_drag;

    let oil_infl = if n.oil_mbd < 0.5 { ((oil_price - 20.0) / 20.0).max(-0.5) * 0.012 * exposure } else { 0.0 };
    let target_infl =
        INFLATION_ANCHOR + demand_gap * 1.6 + oil_infl + if at_war { 0.015 } else { 0.0 };

    GrowthTerms {
        potential,
        demand_gap,
        demand_output,
        bubble: bubble_boost,
        oil: oil_effect,
        oil_revenue_gdp,
        energy_exposure: exposure,
        embargo: embargo_drag,
        sanctions: sanction_drag,
        war: war_drag,
        debt: debt_drag,
        unrest: instability_drag,
        oil_inflation: oil_infl,
        target_inflation: target_infl,
        social_spend: n.social_spend(),
        budget_oil_revenue: if n.oil_mbd > 0.5 { oil_revenue_gdp * 0.55 } else { 0.0 },
        floor: WORST_ANNUAL_COLLAPSE,
        before_noise,
        growth: before_noise.max(WORST_ANNUAL_COLLAPSE),
    }
    }
}


/// THE STANDING PRESSURE ON A NATION'S ORDER: everything the stability
/// integrator adds in a month EXCEPT the mean reversion.
///
/// Factored out of `tick` so the browser can be told where a budget takes a
/// nation without a second copy of this sum. It is not a system and it writes
/// nothing: `tick` calls it, adds `(60 - stability) * MEAN_REVERSION`, and
/// integrates. The fixed point is therefore `60 + pressure / MEAN_REVERSION`
/// BOUNDED TO 0..100 by the integrator's own clamp, which is what
/// `ministries::stability_settles_at` computes and what every stability CARD is
/// now quoted through.
///
/// `budget_gap` is passed in rather than read off the nation because `tick`
/// gates the ministry channels on the nation being the PLAYER (the design's
/// point 5); a function reading `n.budget_gap` directly would quietly hand the
/// three arms to every AI government that ever seats a plan.
pub fn stability_pressure(
    n: &Nation,
    budget_gap: &[f64; BUDGET_MINISTRIES],
    unemployment: f64,
    is_player: bool,
    sanction_share: f64,
) -> f64 {
    let mut ds = 0.0;
    ds += (n.growth_last - 0.015) * 6.0; // growth legitimizes
    ds -= (n.inflation - 0.05).max(0.0) * 4.0; // high inflation corrodes
    if is_player {
        ds -= (unemployment - 0.06).max(0.0) * 1.5;
    }
    // `ds += social_gap * 12.0` STOOD HERE AND IS GONE. It was the
    // pre-ministry route from the social aggregate to stability, and the
    // three surviving addends below are the same claim made per ministry:
    // a nation that raises housing raises `social_total`, so both fired,
    // and the aggregate one could not be attributed to anything a player
    // had done. Inert on every default path — `social_spend_gdp` is `None`
    // for all 137 nations at 1990 and `social_spend()` then returns
    // `baseline_social_spend()` by definition, so `social_gap` was exactly
    // 0.0 — and a live channel for a player who had moved the old slider.
    //
    // HEALTH 8.0 and EDUCATION 5.0 are removed with it: neither ministry
    // keeps a stability arm, because neither one's real claim is about
    // order. DIPLOMACY 3.0 is removed for the same reason. What is left is
    // the three ministries whose whole subject is whether the public is
    // content: HOUSING, PENSIONS and SECURITY.
    ds += crate::ministries::housing_stability(budget_gap[BUDGET_HOUSING])
        + crate::ministries::pensions_stability(budget_gap[BUDGET_PENSIONS])
        + crate::ministries::security_stability(budget_gap[BUDGET_SECURITY]);
    ds -= n.war_exhaustion * 1.2;
    // CONVERTED. This was `sanction_count * 0.15` — the first of the four
    // surviving flag-counting sanction channels, and the one the
    // SANCTION_BITE comment above named as "the stability term below". It is
    // now weighed the way that comment says the honest measure is weighed.
    //
    // The coefficient is not re-derived and is not free: it is the old one
    // carried across the SAME conversion the shipped growth drag used.
    // `0.006 * count -> 0.020 * weight` is exactly `c / 0.30`, i.e. the two
    // rules agree when a sanctioner weighs 30% of world output — that
    // sentence is written out in the SANCTION_BITE comment. Applying it here,
    // `0.15 / 0.30 = 0.50`. So a single sanctioner of the size the shipped
    // conversion was checked against costs precisely what one flag cost, and
    // the only thing that changes is what a coalition costs: at G5 weight
    // (~0.52) the bill falls from 0.75 to 0.26 a month, and it can no longer
    // rise without limit as the roster grows.
    ds -= sanction_share * 0.50;
    if n.system == EconomySystem::Command && n.growth_last < 0.0 {
        ds -= 0.5; // command legitimacy is growth-bought
    }
    ds
}

/// The same pressure, read off the world for a nation that is not mid-tick.
///
/// What the ministry CARDS use. It reproduces `tick`'s player gate exactly,
/// including that a nation which is not the player carries no ministry arms at
/// all -- so its card shows a flat zero rather than a promise the sim will not
/// keep.
pub fn stability_pressure_of(w: &WorldState, n: &Nation) -> f64 {
    let is_player = w.player == Some(n.id);
    let budget_gap: [f64; BUDGET_MINISTRIES] =
        std::array::from_fn(|i| if is_player { n.budget_gap(i) } else { 0.0 });
    stability_pressure(
        n,
        &budget_gap,
        unemployment_rate(n, w.at_war(n.id)),
        is_player,
        w.sanction_weight(n.id),
    )
}

pub fn tick(w: &mut WorldState) {
    let daily = crate::clock::is_daily(w);
    let dt = crate::clock::month_fraction(w);
    let trend_blend = crate::clock::blend(w, 0.008);
    let growth_blend = crate::clock::blend(w, 0.10);
    let investment_blend = crate::clock::blend(w, 0.06);
    let inflation_blend = crate::clock::blend(w, 0.10);
    if daily {
        prepare_daily_shocks(w);
    }
    oil_market(w);

    let oil_price = w.oil_price;
    let player = w.player;
    let ids: Vec<NationId> = w.nations.iter().filter(|n| n.alive).map(|n| n.id).collect();
    let mut district_growth: Vec<(NationId, f64)> = Vec::with_capacity(ids.len());

    for id in ids {
        let sanction_share = w.sanction_weight(id);
        let at_war = w.at_war(id);
        let export_share = w.oil_export_share(id);
        let noise = if daily {
            match w.daily.economic_shocks.get(&id).copied() {
                Some(shock) => shock,
                None => {
                    // A successor born after the month's opening needs its own
                    // persisted shock. Never redraw it on each daily tick.
                    let shock = w.rng.range(-0.004, 0.004);
                    w.daily.economic_shocks.insert(id, shock);
                    shock
                }
            }
        } else {
            w.rng.range(-0.004, 0.004)
        };
        let crisis_mult = w.rules.crisis_intensity;
        let budget_gap: [f64; BUDGET_MINISTRIES] = std::array::from_fn(|i| {
            if Some(id) == player { w.nation(id).budget_gap(i) } else { 0.0 }
        });
        let project_level = crate::province_economy::project_level(w, id);
        let n = w.nation_mut(id);

        // The whole of this nation's year, priced by `growth_terms` — the one
        // definition, which the browser's policy panel is now handed rather
        // than reimplementing. What stays here is only what MUTATES.
        let invest = crate::programs::effective_investment(n, n.state_invest_gdp) + n.priv_invest_gdp;
        let terms = growth_terms(
            n,
            n.state_invest_gdp,
            n.interest_rate,
            &Conditions { oil_price, sanction_share, at_war, export_share },
        );

        let gdp_pc = n.gdp * 1000.0 / n.population; // $ per capita
        let dev = (gdp_pc / 24000.0).min(1.0);
        // The advantage of backwardness expires. A trend rate earned while
        // catching up cannot be held once a nation *is* the frontier: there is
        // nothing left to copy, and everything after that has to be invented.
        // Japan's transcribed 1.8% is a 1980s number it never saw again after
        // the bubble, and nothing in the model was taking it away. US TFP growth
        // averaged about 1.1% a year over the same period, which is the anchor.
        const FRONTIER_TFP: f64 = 0.011;
        // READ THE TRANSCRIBED TREND, NOT `tfp_base`. Since the 1990 endowment
        // landed, `tfp_base` is the transcribed trend with the value of that
        // endowment already subtracted out of it (see
        // `tech::rebase_to_transcribed`), and this anchor was calibrated against
        // the transcribed figure — Japan's 1.8%, against a US frontier of 1.1%.
        // Comparing the rebased base instead makes the predicate a function of
        // how much technology a nation was handed, which is not what "the
        // advantage of backwardness expires" is about: measured on the shipped
        // roster under a top-28 endowment, the reversion switched ON for the
        // United Arab Emirates, where today it is off, and Luxembourg picked up
        // an extra 4.1e-4 of downward drag in twelve months. Both were pure
        // artefacts of the arithmetic.
        //
        // It also puts the fixed point in the right place. Driving
        // `tfp_base + offset` toward 0.011 means the *assembled* trend converges
        // on `0.011 + (s - reference)`: the frontier anchor applies to the trend
        // net of a nation's technological position, and its position then adds
        // on top, which is what the anchor means.
        //
        // On a board where nothing was granted the offset is exactly +0.0 and
        // this is the line that was here before, bit for bit. The additive
        // nudges in `stratagems.rs` need no such change and must not get one:
        // they add to `tfp_base` rather than compare against it, and addition is
        // offset-invariant.
        // REVERT THE ASSEMBLED TREND, NOT THE TRANSCRIBED ONE — and this
        // replaces the argument written directly above, which is why it is
        // spelled out rather than merely changed.
        //
        // Driving `tfp_base + offset` to the anchor leaves the terminal trend
        // equal to `0.011 - s_1990 + reference_1990 + s_2025 - reference_2025`.
        // By 2025 every major holds all 167 technologies, so `s_2025` is common
        // to all of them and the ONLY thing still separating their trends is
        // `-s_1990`: the nation authored to know the most in 1990 is permanently
        // penalised for it. Measured 2025 trends were USA 0.910, Japan 1.064,
        // UK 1.124, Germany 1.131, Italy 1.147, France 1.156 — the United
        // States, authored with 40 technologies against Italy's 16, ends with
        // the LOWEST trend of the six while its real per-capita growth is the
        // HIGHEST of the six. Two nations at an identical 2025 technological
        // position hold permanently different trends, differing only by a 1990
        // starting condition, inside the one term whose entire purpose is that
        // such advantages EXPIRE.
        //
        // The offset-invariance argument the replaced comment makes is correct
        // about `stratagems.rs`, which ADDS to `tfp_base`, and addition really is
        // offset-invariant. It does not carry to a PREDICATE and a target, which
        // is what this is: comparing against, and reverting toward, a level.
        if dev >= 1.0 && n.tfp_trend > FRONTIER_TFP {
            n.tech.tfp_base += (FRONTIER_TFP - n.tfp_trend) * trend_blend;
        }

        // The bubble's own stock, moved here from beside the boost it produces.
        // Both arms read `n.bubble` as the month found it before writing to it,
        // so the boost `growth_terms` returned is the value this block used to
        // compute for itself.
        let real_rate = n.interest_rate - n.inflation;
        if n.bubble > 0.0 {
            // Tight real rates pop bubbles
            if real_rate > 0.025 && n.bubble > 0.5 {
                n.bubble -= 0.06 * crisis_mult * dt;
                if n.bubble < 0.5 {
                    // POP: flip into a debt-overhang hangover (negative bubble)
                    n.bubble = -crisis_mult;
                }
            } else {
                n.bubble = (n.bubble + 0.004 * dt).min(1.0);
            }
        } else if n.bubble < 0.0 {
            n.bubble = (n.bubble + 0.0042 * dt).min(0.0);
        }

        let growth_annual = (terms.before_noise + noise).max(WORST_ANNUAL_COLLAPSE);
        let opening_output = n.gdp;
        let previous_reported_growth = n.growth_last;

        crate::province_economy::apply_macro_factor(&mut n.gdp, project_level,
            1.0 + growth_annual / 12.0 * dt);
        n.growth_last = if daily {
            n.growth_last * (1.0 - growth_blend) + growth_annual * growth_blend
        } else {
            n.growth_last * 0.9 + growth_annual * 0.1
        };

        // A change in the investment share buys a permanently different level of
        // output per worker, not a permanently different growth rate. With
        // Y/L ∝ (K/Y)^(α/(1-α)) and K/Y ∝ s along a balanced path, moving the
        // share from s0 to s1 moves the level by (s1/s0)^(α/(1-α)).
        //
        // `None` means the 1990 transcription already reflects the 1990 share —
        // this must never reprice a transcribed starting figure, and it is also
        // what makes an older save load without being paid twice.
        //
        // THE FORM OF THIS BLOCK IS RIGHT AND IS KEPT: it pays on the CHANGE in
        // the entitlement rather than on the entitlement itself, so a constant
        // share pays a bounded one-time level and then exactly nothing forever.
        // That is BIBLE §8 satisfied by construction. Two things in it were not
        // derived, and both are the flow-priced-as-a-stock mistake the arm above
        // records.
        //
        // 1. THE SPEED WAS A FREE CONSTANT. `0.02` a month, glossed as "roughly
        //    four years", is a 2.9-year half-life. A capital-output ratio does
        //    not converge at 24% a year; it converges at δ + g, which is the
        //    coefficient on K/Y in its own law of motion — about 8% a year, a
        //    nine-year half-life. The shipped speed chased every transient dip
        //    in a policy variable three times faster than a capital stock can
        //    physically respond, so a share that fell for a decade and came back
        //    was charged most of a permanent level loss on the way down.
        //
        // 2. THE STEP WAS THE LINEARISATION, AND THE LINEARISATION IS SYMMETRIC
        //    WHEN THE PHYSICS IS NOT. `(entitled - paid)/CAPITAL_ELASTICITY` is
        //    exactly ln(K*/K) — the log gap between the capital-output ratio the
        //    current share supports and the one the nation has — so the true law
        //    of motion in logs is
        //
        //        d ln(K/Y)/dt = s/(K/Y) - (δ + g) = (δ + g)·(K*/K - 1)
        //
        //    and `exp(gap) - 1` is that, where `gap` alone is its first-order
        //    approximation. The difference is the whole asymmetry: a stock that
        //    is being added to can rise as fast as the investment allows, but it
        //    can only FALL at the rate it wears out, because gross investment
        //    cannot go below zero. exp(x)-1 > x for x < 0 says precisely that,
        //    and it is why a mild decline in a high share can no longer produce
        //    a large sustained negative. The linearisation had no such floor.
        //
        // Measured, 30 seeds, off the shipped board's own share series before
        // anything was changed: the two corrections together are worth about
        // +0.01 pt/yr to China over thirty years and EXACTLY 0.000 to Japan,
        // Germany, France, the UK and Italy, whose shares are flat for 420
        // months. This block was never where the damage was — that is the arm
        // above — and it is repaired here because it was wrong, not because it
        // was load-bearing.
        const CAPITAL_ELASTICITY: f64 = 0.49; // α/(1-α) at a capital share of 1/3
        const DEPRECIATION: f64 = 0.05; // aggregate capital, the standard rate
        const TREND_GROWTH: f64 = 0.03; // the reference balanced path's g
        const CONVERGENCE: f64 = (DEPRECIATION + TREND_GROWTH) / 12.0;
        let entitled = CAPITAL_ELASTICITY * crate::exact::ln(invest.max(1e-6) / 0.20);
        match n.capital_level_paid {
            None => n.capital_level_paid = Some(entitled),
            Some(paid) => {
                // A guard and not a policy, in the sense the WORST_ANNUAL_COLLAPSE
                // paragraph above uses: |gap| = 2 is a capital-output ratio 7.4x
                // away from what the share supports, which nothing this model
                // produces reaches. It is here so the exponential is provably
                // bounded rather than bounded by inspection.
                let gap = ((entitled - paid) / CAPITAL_ELASTICITY).clamp(-2.0, 2.0);
                let step = CAPITAL_ELASTICITY * CONVERGENCE * (crate::exact::exp(gap) - 1.0) * dt;
                crate::province_economy::apply_macro_factor(&mut n.gdp, project_level,
                    crate::exact::exp(step));
                n.capital_level_paid = Some(paid + step);
            }
        }
        if project_level.is_some_and(|level|level>0.0) && opening_output > 0.0 {
            // Report the base-only macro multiplier plus the capital-level
            // adjustment actually realized. Project replacement adds its own
            // separate level delta at the end of this same daily tick.
            let actual_annual = (n.gdp / opening_output - 1.0) / (dt / 12.0);
            n.growth_last = previous_reported_growth * (1.0 - growth_blend)
                + actual_annual * growth_blend;
        }

        // THE OIL TERMS-OF-TRADE LEVEL BELONGS HERE, beside the other two level
        // writes, and is deliberately absent. Its shape is settled — the same
        // `match Option<f64>` tracker as the block above, paying `ln(1 +
        // oil_windfall)` in at about 0.04 a month, which is a two-year budget lag
        // rather than the four years re-equipping an economy takes, and fast
        // enough that the ±$0.6 monthly noise in `oil_market` does not become
        // ±5 pt/yr of producer growth. What blocks it is not its design. See the
        // block above the producer arm for the measurement and the ruling it
        // needs.

        // In the governed economy, private investment is an outcome rather than
        // a frozen roster number. AI economies keep their calibrated structural
        // shares until they receive the same player policy surface.
        let unemployment = unemployment_rate(n, at_war);
        if Some(id) == player {
            let neutral = 0.025;
            let mut business_pressure = (n.growth_last - 0.020) * 0.20
                + (neutral - real_rate) * 0.10
                + (n.stability - 60.0) * 0.00010
                + (0.28 - n.tax_rate) * 0.06
                + (0.07 - unemployment) * 0.05
                + n.bubble * 0.012
                - sanction_share * 0.025
                - if at_war { 0.020 } else { 0.0 };
            // NO MINISTRY REACHES PRIVATE INVESTMENT. Infrastructure 0.02,
            // industry 0.04, science 0.02 and diplomacy 0.01 were removed:
            // private investment is a THIRD route from the same budget line to
            // the same output, on top of `investment_total` and the potential
            // addend above, and three routes is two too many.
            if n.system == EconomySystem::Command {
                business_pressure -= 0.003 + dev * 0.002;
            }
            let private_target = (n.priv_invest_gdp + business_pressure).clamp(0.01, 0.35);
            n.priv_invest_gdp += (private_target - n.priv_invest_gdp) * investment_blend;
        }

        // ---- Inflation (annual rate, adjusts monthly) ----
        // Demand pressure plus oil pass-through for importers; tight money disinflates.
        n.inflation += (terms.target_inflation - n.inflation) * inflation_blend;
        n.inflation = n.inflation.clamp(-0.05, 3.0);

        // ---- Budget & debt ----
        // ONE DEFINITION of what a state takes in and what it pays out, for the
        // same reason `GrowthTerms` is one definition of a growth rate: the
        // money card, the ledger and this block all read `Fiscal::of` and
        // cannot disagree about the books. The two lines this replaces were
        // character for character what `Fiscal::of` computes.
        let books = Fiscal::of(n, &terms);
        let revenue_gdp = books.revenue_gdp;
        let spend_gdp = books.spend_gdp;
        match (n.treasury_bn, n.debt_bn) {
            _ if n.program_budget.is_some() => {
                // Actual programme disbursements are not all known until
                // procurement/industry finish. One end-of-day fiscal posting
                // replaces, rather than supplements, the blanket ministry bill.
                crate::programs::stage_fiscal(n, revenue_gdp, books.interest_bn);
            }
            // THE BOOKS ARE OPEN. The two stocks are the state's finances and
            // this block is their only writer, `debt_gdp` becoming their
            // quotient at the end of the month rather than a thing pushed
            // around by a dozen callers.
            //
            // TWO LINES ARE DELIBERATELY ABSENT and their absence is the point.
            // The growth+inflation renormalisation is gone because a stock of
            // dollars does not erode when output grows -- the RATIO falls, and
            // it falls out of the division below, which is the same statement
            // made once instead of twice. The `.max(0.0)` floor is gone because
            // `pay` cannot drive debt negative anyway, and because a state that
            // is owed money is now a positive `net_position_bn` rather than an
            // impossible negative ratio.
            //
            // Interest is charged at the escalating rate (Ridge's amendment,
            // 2026-09-02) against the debt ratio as the month FOUND it, before
            // this month's borrowing, so a month cannot charge itself a premium
            // for the debt it is about to issue.
            (Some(treasury), Some(debt)) => {
                let revenue_bn = revenue_gdp * n.gdp / 12.0 * dt;
                let spend_bn = spend_gdp * n.gdp / 12.0 * dt;
                // `books.interest_bn` is the ANNUAL charge, `debt *
                // effective_rate`, so this is character for character the
                // `debt * effective_interest_rate(..) / 12.0` it replaces --
                // not the reporting share multiplied back up by GDP, which
                // would not be the same float.
                let interest_bn = books.interest_bn / 12.0 * dt;
                let (treasury, debt) = pay(treasury, debt, spend_bn + interest_bn - revenue_bn);
                n.treasury_bn = Some(treasury);
                n.debt_bn = Some(debt);
                n.debt_gdp = debt / n.gdp.max(0.1);
            }
            // THE BOOKS ARE CLOSED, which is every AI nation and the whole
            // default board. Not one new multiplication happens here.
            _ => {
                let deficit_gdp = spend_gdp - revenue_gdp;
                // Debt ratio: adds deficit, erodes with growth+inflation
                n.debt_gdp += deficit_gdp / 12.0 * dt;
                n.debt_gdp /= 1.0 + (growth_annual + n.inflation) / 12.0 * dt;
                n.debt_gdp = n.debt_gdp.max(0.0);
            }
        }

        // ---- Population ----
        let demographic_support = crate::ministries::health_population(budget_gap[BUDGET_HEALTH])
            + crate::ministries::housing_population(budget_gap[BUDGET_HOUSING]);
        let population_before = n.population;
        n.population *= 1.0 + (population_growth(n) + demographic_support) / 12.0 * dt;
        district_growth.push((id, n.population / population_before));

        // ---- Stability ----
        // The whole of the month's push, less the mean reversion, is
        // `stability_pressure` -- factored out so `ministries` can tell a card
        // where the nation SETTLES without keeping a second copy of this sum.
        // See that function for every term and why the gaps are passed in.
        let mut ds =
            stability_pressure(n, &budget_gap, unemployment, Some(id) == player, sanction_share);
        // The 0.01 is `ministries::MEAN_REVERSION`, named there because it is
        // the reciprocal every stability CARD is quoted through: a standing
        // pressure of x settles the nation at `(60 + x/0.01).clamp(0.0, 100.0)`,
        // and the clamp is as load-bearing as the reciprocal.
        ds += (60.0 - n.stability) * crate::ministries::MEAN_REVERSION; // slow mean reversion
        n.stability = (n.stability + ds / 12.0 * 12.0 * 0.25 * dt).clamp(0.0, 100.0);

        // Separatism strain grows when unstable, decays when stable
        if n.separatism > 0.0 {
            if n.stability < 50.0 {
                n.separatism = (n.separatism + 0.008 * crisis_mult * dt).min(1.0);
            } else {
                n.separatism = (n.separatism - 0.002 * dt).max(0.0);
            }
            // SECURITY ALONE OWNS THE COHESION TERM. Housing's half was
            // removed: two ministries suppressing the same separatism made the
            // effect a function of the budget's shape rather than of policing,
            // and housing already answers for contentment through `ds` above.
            //
            // A POSITIVE GAP ONLY, and that is deliberate rather than
            // inherited. Reading a cut here would CONJURE secession that is not
            // in the nation's `separatism` stock — a player who trimmed the
            // police budget of a country with no separatist movement would
            // watch one appear — so a cut simply stops buying suppression.
            let cohesion = crate::ministries::security_cohesion(budget_gap[BUDGET_SECURITY]);
            n.separatism = (n.separatism - cohesion * dt).max(0.0);
        }
    }

    // Technology applies its own population channel later in this same month.
    // Hold this exact first multiplier transiently so both can be paid in one
    // province-map pass, in the same arithmetic order.
    w.district_population_growth = district_growth;
}

/// How much a nation's population growth MOVES as it gets rich, and nothing
/// about where it starts. Unchanged, coefficient for coefficient, from the
/// function that used to be the whole answer.
///
/// The demographic transition is real and this is its shape: getting rich cuts
/// fertility below replacement and does not stop there. Japan peaked in 2010 and
/// has shrunk every year since; Italy has been below replacement since the late
/// 1970s. What was wrong was using it as a LEVEL. Read as one, it made
/// population growth a decreasing function of income across the whole board,
/// and among rich countries that is simply not true — population growth there is
/// set by migration, and Switzerland, Australia and Canada, three of the richest
/// on the panel, had three of the fastest-growing populations. Driven off income
/// alone the United States' labour term ran +0.254 pt/yr in the 1990s and
/// **-0.217 by the 2020s**: a population shrinking 0.36%/yr against a real
/// +0.7%, with the term declining toward its floor fastest for the richest.
///
/// So this supplies the CHANGE and `Nation::pop_growth_offset` supplies the
/// level, from the nation's own transcribed 1990 rate. See
/// `data::EconomyRecord::pop_growth_1990`.
pub fn transition(gdp_pc: f64) -> f64 {
    if gdp_pc < 3000.0 {
        0.021
    } else if gdp_pc < 12000.0 {
        0.012
    } else if gdp_pc < 25000.0 {
        0.005
    } else {
        (0.005 - (gdp_pc - 25000.0) / 25000.0 * 0.007).max(-0.004)
    }
}

/// The annual rate a nation's population is growing at: its own transcribed 1990
/// rate, moved by the transition since. One definition, used both to age the
/// population and to price labour's contribution to output — they must not be
/// allowed to disagree.
pub fn population_growth(n: &Nation) -> f64 {
    transition(n.gdp * 1000.0 / n.population) + n.pop_growth_offset
}

/// The annual demographic pace the next monthly tick will apply to one nation,
/// including the player's health and housing budget choices. Exposed so the
/// province dossier can state the same rate the simulation uses.
///
/// The two arms are `ministries::health_population` and
/// `ministries::housing_population`, which is where the tick's own
/// `demographic_support` comes from -- not a second copy of `* 0.030` and
/// `* 0.015` beside it. The ministry named FAMILIES when this was written is
/// HOUSING now; the index is the same slot and the coefficients are unmoved.
pub fn effective_population_growth(w: &WorldState, id: NationId) -> Option<f64> {
    let n = w.nation_opt(id)?;
    let policy = if w.player == Some(id) {
        crate::ministries::health_population(n.budget_gap(BUDGET_HEALTH))
            + crate::ministries::housing_population(n.budget_gap(BUDGET_HOUSING))
    } else {
        0.0
    };
    let economy = population_growth(n) + policy;
    let technology = crate::tech::demographic_growth(n);
    // Both channels are applied as monthly multipliers. Return the equivalent
    // annual pace of their product, not a browser-side approximation.
    Some(((1.0 + economy / 12.0) * (1.0 + technology / 12.0) - 1.0) * 12.0)
}

/// A shock describes conditions during a calendar month, not a fresh lottery
/// ticket for every daily click. Keep it in the save so a mid-month reload and
/// a fast-forward consume exactly the same shocks. The flows themselves still
/// settle every day and react immediately to policy, war and ownership changes.
fn prepare_daily_shocks(w: &mut WorldState) {
    let month = w.year * 12 + w.month as i32 - 1;
    if w.daily.shock_month == Some(month) {
        return;
    }
    w.daily.shock_month = Some(month);
    w.daily.oil_shock = Some(w.rng.range(-0.6, 0.6));
    w.daily.economic_shocks.clear();
    for n in w.nations.iter().filter(|n| n.alive) {
        w.daily.economic_shocks.insert(n.id, w.rng.range(-0.004, 0.004));
    }
}

fn oil_market(w: &mut WorldState) {
    // Supply disruption: whatever a producer cannot ship — because its terminals
    // are a war zone, or because the world's buyers have shut it out — is supply
    // the market does not get.
    let producers: Vec<(NationId, f64)> = w
        .nations
        .iter()
        .filter(|n| n.alive && n.oil_mbd > 0.0)
        .map(|n| (n.id, n.oil_mbd))
        .collect();
    let mut disrupted = 0.0;
    let mut total = 0.0;
    for (id, mbd) in producers {
        total += mbd;
        disrupted += mbd * (1.0 - w.oil_export_share(id));
    }
    let disruption_share = if total > 0.0 { (disrupted / total).min(0.6) } else { 0.0 };
    // Oil demand is famously inelastic: a tenth of supply off the market moves
    // the price far more than a tenth.
    let target = 20.0 * (1.0 + disruption_share * 4.0);
    let dt = crate::clock::month_fraction(w);
    let rate = crate::clock::blend(w, 0.18);
    let noise = if crate::clock::is_daily(w) {
        // Integrating this constant forcing with the same retention as the
        // target reproduces a full monthly oil innovation at the month end.
        w.daily.oil_shock.unwrap_or(0.0) * rate / 0.18
    } else {
        w.rng.range(-0.6, 0.6) * dt
    };
    w.oil_price += (target - w.oil_price) * rate + noise;
    w.oil_price = w.oil_price.clamp(8.0, 120.0);
}

#[cfg(test)]
mod daily_tests {
    use super::*;
    use crate::init::world_1990;

    #[test]
    fn daily_economy_posts_growth_population_and_the_days_cash_only() {
        let mut w = world_1990(GameRules { daily_simulation: true, ..GameRules::default() });
        let id = NationId::France;
        {
            let n = w.nation_mut(id);
            n.treasury_bn = Some(100.0);
            n.debt_bn = Some(0.0);
            n.debt_gdp = 0.0;
            n.oil_mbd = 0.0;
            n.tax_rate = 0.40;
            n.social_spend_gdp = Some(0.10);
            n.mil_spend_gdp = 0.02;
            n.state_invest_gdp = 0.03;
        }
        let before = w.nation(id).clone();
        tick(&mut w);
        let after = w.nation(id);
        assert_ne!(after.gdp, before.gdp, "GDP must change before month end");
        assert_ne!(after.population, before.population, "population must change each day");
        let expected_receipt = (0.40 - (0.10 + 0.02 + 0.03)) * after.gdp / 12.0 / 31.0;
        assert!((after.treasury_bn.unwrap() - 100.0 - expected_receipt).abs() < 1e-12,
            "one January day must post 1/31 of the monthly fiscal flow");
        assert_eq!(after.debt_bn, Some(0.0));
    }

    #[test]
    fn daily_economic_shocks_are_saved_and_are_not_redrawn_each_day() {
        let mut w = world_1990(GameRules { daily_simulation: true, ..GameRules::default() });
        tick(&mut w);
        let shocks = w.daily.economic_shocks.clone();
        let oil = w.daily.oil_shock;
        let rng = w.rng.clone();
        w.day = 2;
        let mut resumed = crate::load(&crate::save(&w)).unwrap();
        tick(&mut w);
        tick(&mut resumed);
        assert_eq!(w.daily.economic_shocks, shocks);
        assert_eq!(w.daily.oil_shock, oil);
        assert_eq!(format!("{rng:?}"), format!("{:?}", w.rng), "no new within-month shock draw");
        assert_eq!(crate::save(&w), crate::save(&resumed));
    }
}
