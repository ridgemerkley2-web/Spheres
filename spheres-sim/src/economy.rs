use crate::world::*;

/// Monthly economic tick for every living nation, plus the global oil market.
pub fn tick(w: &mut WorldState) {
    oil_market(w);
    technology(w);

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
        // Capital deepening has diminishing returns as economies mature
        let invest_effect = invest * (0.030 + 0.080 * (1.0 - dev));
        // Productivity is mostly the technology a nation has actually absorbed.
        // The convergence bonus poorer nations used to get for being poor now
        // comes from where it really comes from: the distance to the frontier,
        // and whether they are equipped to close it.
        let mut potential = n.tfp_trend * TFP_RESIDUAL + n.tech_growth * TECH_TO_TFP + invest_effect;

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
        let oil_effect = if n.oil_mbd > 0.5 {
            (oil_price - 20.0) / 20.0 * oil_revenue_gdp * 0.5
        } else {
            -(oil_price - 20.0) / 20.0 * 0.006
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
        let oil_infl = if n.oil_mbd < 0.5 { ((oil_price - 20.0) / 20.0).max(-0.5) * 0.012 } else { 0.0 };
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
        let pop_growth = if gdp_pc < 3000.0 { 0.021 } else if gdp_pc < 12000.0 { 0.012 } else { 0.005 };
        n.population *= 1.0 + pop_growth / 12.0;

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

/// How much of a nation's measured productivity growth the technology system
/// accounts for. The rest is `tfp_trend` — sectoral mix, institutions, residual.
const TECH_TO_TFP: f64 = 0.55;
const TFP_RESIDUAL: f64 = 0.35;
/// Scale of original invention at the frontier, annual.
const INVENTION: f64 = 0.024;
/// Scale of adoption per unit of distance to the frontier, annual. Copying is far
/// easier than inventing, and that asymmetry is the entire engine of convergence.
const DIFFUSION: f64 = 0.022;
/// Adoption is worse than linear in the log-gap: the first nine tenths of a gap
/// is machinery and manuals that can be bought, and the last tenth is tacit and
/// is not for sale. A nation that has copied everything copyable has to start
/// inventing, and most of them discover they cannot.
const TACIT: f64 = 1.35;

/// The technological frontier, and everyone's distance from it.
///
/// Two things move a nation's technology. It can invent, which only really
/// happens near the frontier and only in proportion to what the current paradigm
/// rewards. Or it can adopt, which is cheap and scales with how far behind it is
/// — the further back, the more there is lying around to be copied. For everyone
/// but the leader the second term dominates, and that is why poor countries can
/// grow faster than rich ones without being cleverer than them.
fn technology(w: &mut WorldState) {
    // The frontier ratchets. A state can dissolve; what it knew stays known.
    let peak = w.nations.iter().filter(|n| n.alive).map(|n| n.tech).fold(0.0, f64::max);
    w.tech_frontier = w.tech_frontier.max(peak);

    // A paradigm opens when the frontier reaches it — no calendar, no decree.
    while let Some(next) = w.era.next() {
        if w.tech_frontier < next.onset() {
            break;
        }
        w.era = next;
        let leader = w.tech_leader().map(|l| l.name()).unwrap_or("no one");
        w.headline(format!(
            "A NEW TECHNOLOGICAL ERA: the {} age opens, with {} at the frontier.",
            next.name(), leader
        ));
    }

    // A fresh paradigm is fertile and an exhausted one is not. This is what puts
    // an acceleration at the start of an era and a slowdown at the end of it,
    // without anyone scheduling either.
    let progress = ((w.tech_frontier - w.era.onset()) / w.era.span()).clamp(0.0, 1.0);
    let vigor = 0.70 + 0.55 * (1.0 - progress);
    let era = w.era;
    let frontier = w.tech_frontier.max(1e-6);

    let ids: Vec<NationId> = w.nations.iter().filter(|n| n.alive).map(|n| n.id).collect();
    for id in ids {
        let n = w.nation_mut(id);
        let open = openness(n);
        // Schooling measured against the 1990 frontier's ~13 years.
        let school = (n.education / 12.0).clamp(0.10, 1.20);
        // How much of the economy is committed to building things. It is what the
        // industrial paradigm paid for, and what the intelligence one will pay for
        // again once compute has to be bought by the acre.
        let capital = ((n.state_invest_gdp + n.priv_invest_gdp) / 0.30).min(1.40);
        let rel = (n.tech / frontier).clamp(0.0, 1.0);
        // Sunk paradigm. Heavy capital is only an anchor if it is *modern* heavy
        // capital: a nation near the frontier has its plant, its firms and its
        // careers organised around the paradigm that is ending, and cannot simply
        // decide to be organised around the next one. A nation still three decades
        // behind has nothing to be stranded by — it is not choosing the fax
        // machine over the internet, it is buying both at once.
        let incumbency = capital * rel;
        // New ventures are funded out of a growing economy and starved by a
        // balance-sheet hangover. Japan's lost decades were a technology story as
        // much as a monetary one, and this is the link: stagnation defunds the
        // research that would have ended it.
        let finance = (0.60 + 13.0 * n.growth_last).clamp(0.60, 1.25)
            * (1.0 + n.bubble.min(0.0) * 0.20)
            * (1.0 - (n.debt_gdp - 1.0).max(0.0) * 0.10).max(0.75);

        // Invention needs a frontier to stand on: you cannot discover what is
        // already two paradigms ahead of your best laboratory.
        let invention =
            INVENTION * invention_capture(era, open, school, capital, incumbency)
                * era.frontier_yield() * vigor * finance * rel.powf(0.75);
        let gap = (frontier / n.tech.max(1e-6)).max(1.0).ln();
        let adoption = DIFFUSION
            * adoption_capture(era, open, school, capital, incumbency)
            * gap.powf(TACIT);

        let growth = invention + adoption;
        n.tech *= 1.0 + growth / 12.0;
        // Technology reaches production with a lag; the smoothing is that lag.
        n.tech_growth += (growth - n.tech_growth) * 0.06;

        // Schooling is the slowest input there is. A country cannot buy the
        // education of its adult population, only wait for the cohorts.
        n.education += 0.14 * (1.0 - n.education / 14.0).max(0.0) / 12.0;
    }
}

/// Openness to the ideas and enterprise a frontier runs on. Deliberately not a
/// new datum: it falls out of the political and economic system already on the
/// nation. A regime that closes is choosing to give up technology, and the choice
/// costs it the moment the paradigm turns to one that needs openness.
fn openness(n: &Nation) -> f64 {
    let political = 1.0 - n.authoritarianism;
    let economic = match n.system {
        EconomySystem::Market => 1.0,
        EconomySystem::Command => 0.15,
    };
    (0.15 + 0.50 * political + 0.45 * economic).min(1.0)
}

/// What each paradigm pays for at the frontier.
fn invention_capture(era: TechEra, open: f64, school: f64, capital: f64, incumbency: f64) -> f64 {
    let v = match era {
        // Steel, chemicals, machine tools: capital poured into plant, which a
        // planner can order done and did.
        TechEra::Industrial => 0.30 + 0.55 * capital + 0.25 * school + 0.20 * open,
        // Networked computing rewards the opposite of everything the last paradigm
        // rewarded — open enterprise and schooling — and it strands whoever was
        // best at the last one.
        TechEra::Information => 0.15 + 0.85 * open + 0.55 * school - 1.30 * incumbency,
        // Handsets are manufacturing again, so the industrial economies get a turn.
        TechEra::Mobile => 0.25 + 0.50 * capital + 0.40 * school + 0.35 * open,
        // Compute is capital, and a great deal of it, alongside deep schooling.
        TechEra::Intelligence => 0.05 + 0.60 * capital + 0.80 * school + 0.35 * open,
    };
    v.max(0.05)
}

/// What each paradigm demands of a nation that only wants to *use* it. The
/// information era is the harsh one: a machine tool can be bought and bolted down
/// by a state that permits nothing else, but a network is only worth having if
/// people are allowed to connect to it. That asymmetry is why an open economy
/// takes more out of this era than a closed one of the same wealth.
fn adoption_capture(era: TechEra, open: f64, school: f64, capital: f64, incumbency: f64) -> f64 {
    let v = match era {
        TechEra::Industrial => 0.25 + 0.30 * open + 0.30 * school + 0.55 * capital,
        TechEra::Information => {
            0.10 + 0.85 * open + 0.35 * school + 0.50 * capital - 0.85 * incumbency
        }
        // The one paradigm that reached the poor world faster than the rich one:
        // cheap, mass-market, and it asks almost nothing of the buyer.
        TechEra::Mobile => 0.55 + 0.35 * open + 0.25 * school + 0.55 * capital,
        TechEra::Intelligence => 0.15 + 0.50 * open + 0.55 * school + 0.60 * capital,
    };
    v.max(0.05)
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
