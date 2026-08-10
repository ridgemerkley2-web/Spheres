use crate::world::*;

/// Monthly economic tick for every living nation, plus the global oil market.
pub fn tick(w: &mut WorldState) {
    oil_market(w);

    let oil_price = w.oil_price;
    let ids: Vec<NationId> = w.nations.iter().filter(|n| n.alive).map(|n| n.id).collect();

    for id in ids {
        let sanction_count = w.sanctioned_by_count(id) as f64;
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
        let intensity = (invest / 0.20).max(0.0).powf(0.55) * 0.20;
        let invest_effect = intensity * (0.030 + 0.080 * (1.0 - dev));
        // Income convergence: capital deepening and the reallocation of labour
        // out of subsistence, which is most of what makes a poor country grow
        // fast. This is *not* the technological diffusion the tech tree models,
        // and the two are not the same effect counted twice — that was the
        // argument for deleting this line, and it was wrong. Diffusion governs
        // what it costs to acquire a technology; this governs what happens when
        // a country moves its people from a field to a factory. A nation can
        // hold most of the frontier's technologies and still be ten times poorer
        // per head, which is exactly the gap this closes and the tree does not.
        // Deleting it collapsed emerging growth; it is staying.
        let catchup = (1.0 - dev) * 0.020;
        // Growth accounting: output growth is productivity, plus capital's
        // contribution, plus labour's share of the change in the workforce. A
        // shrinking workforce is a headwind no amount of investment offsets,
        // which is the fact about Japan the model was missing entirely.
        let labour = population_growth(gdp_pc) * 0.60;
        let mut potential = n.tfp_trend + invest_effect + catchup + labour;

        // Command economies pay an allocation penalty that worsens as they develop
        if n.system == EconomySystem::Command {
            potential -= 0.004 + 0.010 * (gdp_pc / 24000.0).min(1.0);
        }

        // ---- Demand side ----
        // Real rate vs neutral moves demand around potential
        let real_rate = n.interest_rate - n.inflation;
        let neutral = 0.025;
        let demand_gap = (neutral - real_rate) * 0.55; // easy money -> above potential

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
            // Balance-sheet recession: drag fades over ~a decade
            bubble_boost = n.bubble * 0.022;
            n.bubble = (n.bubble + 0.009).min(0.0);
        }

        // ---- Drags ----
        let sanction_drag = sanction_count * 0.006;
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

        n.gdp *= 1.0 + growth_annual / 12.0;
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
