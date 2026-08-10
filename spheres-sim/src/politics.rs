use crate::war;
use crate::world::*;

/// Monthly politics & AI tick: central banks, elections, collapses, AI wars, peace.
pub fn tick(w: &mut WorldState) {
    let ids: Vec<NationId> = w.nations.iter().filter(|n| n.alive).map(|n| n.id).collect();

    // ---- Proliferation: India & Pakistan test in 1998 (if alive and unconquered) ----
    if w.year == 1998 && w.month == 5 && !w.has_flag("sasia_nuclear") {
        w.set_flag("sasia_nuclear");
        for id in [NationId::India, NationId::Pakistan] {
            if w.nation(id).alive {
                w.nation_mut(id).nuclear = true;
                w.headline(format!("{} conducts nuclear tests. The world condemns; deterrence descends on the subcontinent.", id.name()));
            }
        }
    }

    // ---- Central banks (AI-controlled nations only) ----
    for id in &ids {
        if Some(*id) == w.player {
            continue;
        }
        let n = w.nation_mut(*id);
        // Taylor-lite: respond to inflation above 2-3% band
        let target = 0.025;
        let desired = (0.025 + n.inflation + (n.inflation - target) * 0.6).clamp(0.0, 0.45);
        n.interest_rate += (desired - n.interest_rate) * 0.15;
    }

    // ---- Fiscal AI: consolidate when debt runs hot ----
    for id in &ids {
        if Some(*id) == w.player {
            continue;
        }
        let n = w.nation_mut(*id);
        if n.debt_gdp > 0.85 {
            n.tax_rate = (n.tax_rate + 0.002).min(0.55);
            n.mil_spend_gdp = (n.mil_spend_gdp * 0.995).max(0.01);
            n.state_invest_gdp = (n.state_invest_gdp * 0.995).max(0.02);
        } else if n.debt_gdp < 0.3 && n.tax_rate > 0.30 {
            n.tax_rate -= 0.001;
        }
    }

    // ---- Elections in democracies (every 4 years, Nov) ----
    if w.month == 11 && (w.year % 4) == 2 % 4 {
        for id in &ids {
            let n = w.nation_mut(*id);
            if n.authoritarianism < 0.35 {
                // Bad times throw the bums out; new government resets some legitimacy
                if n.growth_last < 0.005 || n.inflation > 0.07 {
                    n.stability = (n.stability + 8.0).min(85.0);
                } else {
                    n.stability = (n.stability + 3.0).min(95.0);
                }
            }
        }
    }

    // ---- Regime collapse & USSR dissolution ----
    for id in ids.clone() {
        let (stab, sep, is_ussr, alive) = {
            let n = w.nation(id);
            (n.stability, n.separatism, id == NationId::USSR, n.alive)
        };
        if !alive {
            continue;
        }
        if is_ussr && (stab < 25.0 || sep > 0.9) && !w.has_flag("ussr_dissolved") {
            dissolve_ussr(w);
        } else if !is_ussr && stab < 12.0 && w.rng.chance(0.10 * w.rules.crisis_intensity) {
            // Generic regime collapse: chaos, then a new regime
            let auth_shift = w.rng.range(-0.3, 0.2);
            let n = w.nation_mut(id);
            n.stability = 45.0;
            n.gdp *= 0.93;
            n.authoritarianism = (n.authoritarianism + auth_shift).clamp(0.05, 0.95);
            w.headline(format!("Revolution in {} — the old regime falls.", id.name()));
        }
    }

    // ---- AI war decisions ----
    ai_wars(w);

    // ---- AI sanctions relief: relations mend slowly, sanctions lift when wars end ----
    let war_free: Vec<NationId> = w
        .nations
        .iter()
        .filter(|n| n.alive && !w.at_war(n.id))
        .map(|n| n.id)
        .collect();
    w.sanctions.retain(|(imposer, target)| {
        // Keep sanctions while target is at war or relations are dire
        if !war_free.contains(target) {
            return true;
        }
        // lifted probabilistically? Determinism: use relations threshold instead
        false // lift once at peace — simple v0.5 rule
    });
    let _ = war_free;
}

fn dissolve_ussr(w: &mut WorldState) {
    w.set_flag("ussr_dissolved");
    let (gdp, pop, oil, strength) = {
        let u = w.nation(NationId::USSR);
        (u.gdp, u.population, u.oil_mbd, u.mil_strength)
    };
    {
        let u = w.nation_mut(NationId::USSR);
        u.alive = false;
        u.gdp = 0.0;
    }
    // Russia inherits roughly 60% of the economy, half the people, most oil, the arsenal.
    let russia = Nation {
        id: NationId::Russia,
        alive: true,
        system: EconomySystem::Market, // shock-therapy transition
        authoritarianism: 0.45,
        gdp: gdp * 0.55,
        population: pop * 0.51,
        tfp_trend: 0.008,
        inflation: 0.90, // transition price liberalization
        interest_rate: 0.20,
        tax_rate: 0.28,
        mil_spend_gdp: 0.045,
        state_invest_gdp: 0.04,
        priv_invest_gdp: 0.10,
        debt_gdp: 0.35,
        oil_mbd: oil * 0.85,
        bubble: 0.0,
        growth_last: -0.05,
        stability: 38.0,
        separatism: 0.20,
        mil_strength: strength * 0.65,
        war_exhaustion: 0.0,
        nuclear: true,
    };
    w.nations.push(russia);
    // Inherit a thawed version of USSR relations
    let rels: Vec<(NationId, f64)> = ALL_START_NATIONS
        .iter()
        .filter(|x| **x != NationId::USSR)
        .map(|x| (*x, w.relation(NationId::USSR, *x) * 0.5 + 10.0))
        .collect();
    for (other, v) in rels {
        w.set_relation(NationId::Russia, other, v);
    }
    w.headline("THE SOVIET UNION HAS DISSOLVED. Russia emerges as successor state.".into());
    w.headline("Newly independent republics abstracted; Russia inherits the arsenal.".into());
}

fn ai_wars(w: &mut WorldState) {
    // Iraq's Kuwait calculus: debt-strained oil state eyeing a rich, weak neighbor.
    let candidates: Vec<(NationId, NationId, f64)> = {
        let mut v = vec![];
        let aggressors = [NationId::Iraq, NationId::Iran, NationId::Pakistan, NationId::India];
        for a in aggressors {
            let an = w.nation(a);
            if !an.alive || Some(a) == w.player || w.at_war(a) || an.war_exhaustion > 0.3 {
                continue;
            }
            for t in ALL_START_NATIONS {
                if t == a {
                    continue;
                }
                let tn = w.nation(t);
                if !tn.alive || w.at_war(t) {
                    continue;
                }
                // Only historically-plausible dyads have nonzero base appetite (regional gates)
                let base: f64 = match (a, t) {
                    (NationId::Iraq, NationId::Kuwait) => 0.030,
                    (NationId::Iraq, NationId::SaudiArabia) => 0.004,
                    (NationId::Iraq, NationId::Iran) => 0.002,
                    (NationId::Iran, NationId::Iraq) => 0.002,
                    (NationId::Pakistan, NationId::India) => 0.001,
                    (NationId::India, NationId::Pakistan) => 0.001,
                    _ => 0.0,
                };
                if base <= 0.0 {
                    continue;
                }
                let rel = w.relation(a, t);
                // Expected defense includes likely interveners — but a first-time
                // gambler discounts them (Saddam's 1990 misjudgment). After one
                // repelled invasion the lesson is learned permanently.
                let learned = w.has_flag(&format!("burned_{:?}_{:?}", a, t));
                let mut expected_def = tn.mil_strength;
                let coalition_discount = if learned { 1.0 } else { 0.10 };
                for m in [NationId::USA, NationId::UK, NationId::France] {
                    if m != a && w.nation(m).alive && w.relation(m, t) >= 40.0 {
                        expected_def += w.nation(m).mil_strength * coalition_discount;
                    }
                }
                let strength_ratio = an.mil_strength / expected_def.max(1.0);
                if strength_ratio < 0.8 {
                    continue; // deterred
                }
                // Fiscal desperation raises appetite (Iraq 1990); bad relations too
                let desperation = (an.debt_gdp - 0.6).max(0.0) * 1.5 + (0.4 - an.growth_last * 10.0).max(0.0) * 0.2;
                let mut p = base
                    * w.rules.ai_aggression
                    * (1.0 + desperation)
                    * strength_ratio.min(4.0)
                    * if rel < -20.0 { 1.5 } else { 0.5 };
                // Deterrence: never attack a nuclear power without your own arsenal
                if tn.nuclear && !an.nuclear {
                    p = 0.0;
                }
                v.push((a, t, p.min(0.25)));
            }
        }
        v
    };
    for (a, t, p) in candidates {
        if w.at_war(a) {
            continue;
        }
        let roll = w.rng.chance(p);
        if roll {
            let _ = war::declare_war(w, a, t);
        }
    }

    // AI peace offers: badly losing attackers sue for peace (abstract: white peace at high exhaustion handled in war tick)
}
