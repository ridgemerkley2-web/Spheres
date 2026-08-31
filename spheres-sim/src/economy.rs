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

/// Monthly economic tick for every living nation, plus the global oil market.
pub fn tick(w: &mut WorldState) {
    oil_market(w);

    let oil_price = w.oil_price;
    let ids: Vec<NationId> = w.nations.iter().filter(|n| n.alive).map(|n| n.id).collect();

    for id in ids {
        let sanction_share = w.sanction_weight(id);
        let at_war = w.at_war(id);
        let export_share = w.oil_export_share(id);
        let noise = w.rng.range(-0.004, 0.004);
        let crisis_mult = w.rules.crisis_intensity;
        let n = w.nation_mut(id);

        // ---- Investment & potential growth (annual rates) ----
        let invest = n.state_invest_gdp + n.priv_invest_gdp;
        let gdp_pc = n.gdp * 1000.0 / n.population; // $ per capita
        let dev = (gdp_pc / 24000.0).min(1.0);
        // Capital deepening has diminishing returns as economies mature — and
        // diminishing returns to itself. The return was linear in the investment
        // share, so a nation could buy growth indefinitely by simply investing
        // more of its output, and Japan's 29% of GDP earned it two-thirds again
        // what America's 17.5% earned forever. Concave around a 20% reference:
        // the same at that point, worth progressively less above it.
        let intensity = crate::exact::powf((invest / 0.20).max(0.0), 0.55) * 0.20;
        // ONLY THE TRANSITIONAL ARM SURVIVES. `0.030` was a floor that never
        // expired: `dev` is pinned at 1.0 for every mature economy from month
        // one and `invest` never moves, so it was a per-nation CONSTANT paid as
        // a growth rate for 420 months — 0.5576 pt/yr to the United States and
        // 0.7360 to Japan, flat to four decimals across all four decades. A
        // constant investment share buys a permanently larger economy and no
        // permanent growth at all: capital contributes α·ĝ_K and ĝ_K -> ĝ_Y on
        // any balanced path. It was ordered against reality besides, handing
        // Italy (0.587) more than the United States (0.554). This is BIBLE §8's
        // error class, and it is fixed the way §8 fixes it: the share now buys a
        // LEVEL, paid as the share actually changes, below.
        //
        // The surviving arm is a different animal and is correct in form —
        // transitional capital deepening that expires as a nation develops. It
        // is what pays China 2.7 points in 1990 and 1.4 by 2025.
        let invest_effect = intensity * 0.080 * (1.0 - dev);
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
            n.tech.tfp_base += (FRONTIER_TFP - n.tfp_trend) * 0.008;
        }

        // Command economies pay an allocation penalty that worsens as they develop
        if n.system == EconomySystem::Command {
            potential -= 0.004 + 0.010 * (gdp_pc / 24000.0).min(1.0);
        }

        // ---- Demand side ----
        // Real rate vs neutral moves demand around potential
        let real_rate = n.interest_rate - n.inflation;
        let neutral = 0.025;
        let mut demand_gap = (neutral - real_rate) * 0.55; // easy money -> above potential
        // ...but only while there is a rate left to cut and somebody willing to
        // borrow. Pushing on a string: Japan ran the policy rate at zero for two
        // decades against a corporate sector repairing its balance sheet, and
        // got almost no growth for it. The naive rule reads that same zero rate
        // as a permanent stimulus, which is why Japan kept compounding at 3% in
        // a model that was otherwise behaving.
        if demand_gap > 0.0 {
            let room_to_cut = (n.interest_rate / 0.04).clamp(0.0, 1.0);
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

        // Bubble dynamics: hot bubbles add demand until they pop
        let mut bubble_boost = 0.0;
        if n.bubble > 0.0 {
            bubble_boost = n.bubble * 0.012;
            // Tight real rates pop bubbles
            if real_rate > 0.025 && n.bubble > 0.5 {
                n.bubble -= 0.06 * crisis_mult;
                if n.bubble < 0.5 {
                    // POP: flip into a debt-overhang hangover (negative bubble)
                    n.bubble = -crisis_mult;
                }
            } else {
                n.bubble = (n.bubble + 0.004).min(1.0);
            }
        } else if n.bubble < 0.0 {
            // Balance-sheet recession. A decade was too kind: Japan's asset
            // prices peaked in 1989 and the corporate sector was still paying
            // down debt rather than investing twenty years later, which is why
            // the lost decade is properly the lost decades. Roughly twenty years
            // to heal, not nine.
            bubble_boost = n.bubble * 0.022;
            n.bubble = (n.bubble + 0.0042).min(0.0);
        }

        // ---- Drags ----
        let sanction_drag = sanction_share * SANCTION_BITE;
        let war_drag = if at_war { 0.020 + n.war_exhaustion * 0.03 } else { 0.0 };
        let debt_drag = if n.debt_gdp > 0.9 { (n.debt_gdp - 0.9) * 0.02 } else { 0.0 };
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
        const WORST_ANNUAL_COLLAPSE: f64 = -0.95;
        // `demand_output`, not `demand_gap`: the output arm of the demand term,
        // gated by whether there is still money. The price arm keeps the
        // ungated `demand_gap` — see the split above.
        let growth_annual = (potential + demand_output + bubble_boost + oil_effect
            - sanction_drag - war_drag - debt_drag - instability_drag - embargo_drag + noise)
            .max(WORST_ANNUAL_COLLAPSE);

        n.gdp *= 1.0 + growth_annual / 12.0;
        n.growth_last = n.growth_last * 0.9 + growth_annual * 0.1;

        // A change in the investment share buys a permanently different level of
        // output per worker, not a permanently different growth rate. With
        // Y/L ∝ (K/Y)^(α/(1-α)) and K/Y ∝ s along a balanced path, moving the
        // share from s0 to s1 moves the level by (s1/s0)^(α/(1-α)). Paid in over
        // roughly four years rather than instantly, because re-equipping an
        // economy takes time; paid back symmetrically when a share is cut,
        // because that is what the same arithmetic says.
        //
        // `None` means the 1990 transcription already reflects the 1990 share —
        // this must never reprice a transcribed starting figure, and it is also
        // what makes an older save load without being paid twice.
        const CAPITAL_ELASTICITY: f64 = 0.49; // α/(1-α) at a capital share of 1/3
        let entitled = CAPITAL_ELASTICITY * crate::exact::ln(invest.max(1e-6) / 0.20);
        match n.capital_level_paid {
            None => n.capital_level_paid = Some(entitled),
            Some(paid) => {
                let step = (entitled - paid) * 0.02;
                n.gdp *= crate::exact::exp(step);
                n.capital_level_paid = Some(paid + step);
            }
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

        // ---- Inflation (annual rate, adjusts monthly) ----
        // Demand pressure plus oil pass-through for importers; tight money disinflates.
        let oil_infl = if n.oil_mbd < 0.5 { ((oil_price - 20.0) / 20.0).max(-0.5) * 0.012 * exposure } else { 0.0 };
        let target_infl =
            INFLATION_ANCHOR + demand_gap * 1.6 + oil_infl + if at_war { 0.015 } else { 0.0 };
        n.inflation += (target_infl - n.inflation) * 0.10;
        n.inflation = n.inflation.clamp(-0.05, 3.0);

        // ---- Budget & debt ----
        let revenue_gdp = n.tax_rate + if n.oil_mbd > 0.5 { oil_revenue_gdp * 0.55 } else { 0.0 };
        let social_spend = 0.17 + (1.0 - n.authoritarianism) * 0.05;
        let spend_gdp = social_spend + n.mil_spend_gdp + n.state_invest_gdp;
        let deficit_gdp = spend_gdp - revenue_gdp;
        // Debt ratio: adds deficit, erodes with growth+inflation
        n.debt_gdp += deficit_gdp / 12.0;
        n.debt_gdp /= 1.0 + (growth_annual + n.inflation) / 12.0;
        n.debt_gdp = n.debt_gdp.max(0.0);

        // ---- Population ----
        n.population *= 1.0 + population_growth(n) / 12.0;

        // ---- Stability ----
        let mut ds = 0.0;
        ds += (n.growth_last - 0.015) * 6.0; // growth legitimizes
        ds -= (n.inflation - 0.05).max(0.0) * 4.0; // high inflation corrodes
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
        ds += (60.0 - n.stability) * 0.01; // slow mean reversion
        n.stability = (n.stability + ds / 12.0 * 12.0 * 0.25).clamp(0.0, 100.0);

        // Separatism strain grows when unstable, decays when stable
        if n.separatism > 0.0 {
            if n.stability < 50.0 {
                n.separatism = (n.separatism + 0.008 * crisis_mult).min(1.0);
            } else {
                n.separatism = (n.separatism - 0.002).max(0.0);
            }
        }
    }
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
    let noise = w.rng.range(-0.6, 0.6);
    w.oil_price += (target - w.oil_price) * 0.18 + noise;
    w.oil_price = w.oil_price.clamp(8.0, 120.0);
}
