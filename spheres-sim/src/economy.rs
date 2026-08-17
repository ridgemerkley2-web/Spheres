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
///    China 30-year multiple, at peace, 108 nations, seeds 0..=9:
///      14.15 14.03 13.68 13.80 14.34 14.40 13.68 13.69 14.47 14.02
///      median 14.02x, against the real 14.33x (World Bank NY.GDP.MKTP.KD)
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
///    bite   USA alone -> China   G5 -> China
///           (share 0.24)         (share 0.52)
///    0.000        0.20pt              0.46pt   <- the three count-based
///    0.010        0.48pt              1.09pt      channels still unconverted
///    0.015        0.47pt              1.37pt
///    0.020        0.59pt              1.53pt   <- shipped
///    0.025        0.72pt              1.70pt
///    0.030        0.87pt              1.92pt
///    0.040        1.11pt              2.55pt
///
/// The real regimes of the period, non-oil channel only:
///   - China 2018-19, United States alone, ~24% of world output: growth 6.7% ->
///    6.0%, about 0.6pt. Model at 0.020: 0.59pt.
///   - South Africa 1985-93, near-universal, ~80%: ~1.0% growth against a ~3.5%
///    trend, about 2.5pt. Model at 0.020 scaled to 0.80: 2.35pt.
///   - Russia 2014-21 and Iran 2012-15 are the two other large regimes and are
///    DELIBERATELY NOT USED. Both targets are petro-states whose measured loss
///    ran mostly through oil, and this model prices that separately in
///    `embargo_drag` and `oil_blockade`. Calibrating the non-oil growth drag on
///    them would count the same barrel twice. Taken at face value they would
///    argue for 0.010-0.015.
///
/// So the two clean anchors bracket 0.020-0.025, and 0.020 is taken rather than
/// 0.025 because three further sanction channels still count flags rather than
/// weighing output and add 0.46pt on top at G5 weight (the 0.000 row above):
/// `research_output` and `absorptive_capacity` in tech/mod.rs, and the stability
/// term below. Converting those is the follow-up this commit deliberately does
/// not do, so that the movement here is attributable to one line.
const SANCTION_BITE: f64 = 0.020;

/// Monthly economic tick for every living nation, plus the global oil market.
pub fn tick(w: &mut WorldState) {
    oil_market(w);

    let oil_price = w.oil_price;
    let ids: Vec<NationId> = w.nations.iter().filter(|n| n.alive).map(|n| n.id).collect();

    for id in ids {
        let sanction_count = w.sanctioned_by_count(id) as f64;
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
        let invest_effect = intensity * (0.030 + 0.080 * (1.0 - dev));
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
        let labour = population_growth(gdp_pc) * 0.60;
        let mut potential = n.tfp_trend + invest_effect + catchup + labour;

        // The advantage of backwardness expires. A trend rate earned while
        // catching up cannot be held once a nation *is* the frontier: there is
        // nothing left to copy, and everything after that has to be invented.
        // Japan's transcribed 1.8% is a 1980s number it never saw again after
        // the bubble, and nothing in the model was taking it away. US TFP growth
        // averaged about 1.1% a year over the same period, which is the anchor.
        const FRONTIER_TFP: f64 = 0.011;
        if dev >= 1.0 && n.tech.tfp_base > FRONTIER_TFP {
            n.tech.tfp_base += (FRONTIER_TFP - n.tech.tfp_base) * 0.008;
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

        // Bubble dynamics: hot bubbles add demand until they pop
        let mut bubble_boost = 0.0;
        if n.bubble > 0.0 {
            bubble_boost = n.bubble * 0.012;
            // Tight real rates pop bubbles
            if real_rate > 0.025 && n.bubble > 0.5 {
                n.bubble -= 0.06 * crisis_mult;
                if n.bubble < 0.5 {
                    // POP: flip into a debt-overhang hangover (negative bubble)
                    n.bubble = -1.0 * crisis_mult;
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
        let oil_revenue_gdp = n.oil_mbd * export_share * oil_price * 0.365 / n.gdp; // $bn/yr per $bn GDP
        // An importer that has learned to do more with a barrel is hurt less by
        // what the barrel costs.
        let exposure = crate::tech::energy_exposure(n);
        let oil_effect = if n.oil_mbd > 0.5 {
            (oil_price - 20.0) / 20.0 * oil_revenue_gdp * 0.5
        } else {
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

        let growth_annual = potential + demand_gap + bubble_boost + oil_effect
            - sanction_drag - war_drag - debt_drag - instability_drag - embargo_drag + noise;

        // An economy can collapse; it cannot become negative. Nothing in the
        // growth model bounded this, and it took a *player* to find out: the
        // central bank in politics.rs deliberately skips the player's own nation
        // — the rate is theirs to set — so a player who takes the United States
        // and then does nothing for thirty-five years never cuts, deflates, and
        // the demand gap drives growth negative for long enough to walk GDP
        // through zero. Measured on master: -10.98 by June 2016, after which
        // war.rs square-roots a negative budget, mil_strength becomes NaN, serde
        // writes NaN as `null`, and the browser UI dies on it.
        //
        // Every invariant test we had ran with `player = None`, so the one
        // configuration a human actually plays in was the one nothing covered.
        n.gdp = (n.gdp * (1.0 + growth_annual / 12.0)).max(0.001);
        n.growth_last = n.growth_last * 0.9 + growth_annual * 0.1;

        // ---- Inflation (annual rate, adjusts monthly) ----
        // Demand pressure plus oil pass-through for importers; tight money disinflates.
        let oil_infl = if n.oil_mbd < 0.5 { ((oil_price - 20.0) / 20.0).max(-0.5) * 0.012 * exposure } else { 0.0 };
        let target_infl = 0.02 + demand_gap * 1.6 + oil_infl + if at_war { 0.015 } else { 0.0 };
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
        n.population *= 1.0 + population_growth(gdp_pc) / 12.0;

        // ---- Stability ----
        let mut ds = 0.0;
        ds += (n.growth_last - 0.015) * 6.0; // growth legitimizes
        ds -= (n.inflation - 0.05).max(0.0) * 4.0; // high inflation corrodes
        ds -= n.war_exhaustion * 1.2;
        // STILL COUNTS FLAGS, deliberately. One of the three channels named in
        // the SANCTION_BITE comment that should weigh `sanction_weight` rather
        // than counting signatures; left alone so that this commit's movement is
        // attributable to the growth drag alone. The other two are
        // `research_output` and `absorptive_capacity` in tech/mod.rs.
        ds -= sanction_count * 0.15;
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

/// The demographic transition, as an annual rate, read off income per head.
///
/// It runs all the way down: getting rich cuts fertility below replacement and
/// does not stop there. Japan peaked in 2010 and has shrunk every year since;
/// Italy has been below replacement since the late 1970s. One definition, used
/// both to age the population and to price labour's contribution to output —
/// they must not be allowed to disagree.
fn population_growth(gdp_pc: f64) -> f64 {
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
