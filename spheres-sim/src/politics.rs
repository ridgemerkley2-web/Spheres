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
        let is_yugo = id == NationId::Yugoslavia;
        if is_ussr && (stab < 25.0 || sep > 0.9) && !w.has_flag("ussr_dissolved") {
            dissolve_ussr(w);
        } else if is_yugo && (stab < 25.0 || sep > 0.9) && !w.has_flag("yugoslavia_dissolved") {
            dissolve_yugoslavia(w);
        } else if !is_ussr && !is_yugo && stab < 12.0 && w.rng.chance(0.10 * w.rules.crisis_intensity) {
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

    // ---- Grievances fade; alliances are institutional and do not ----
    let belligerents: Vec<(NationId, NationId)> =
        w.wars.iter().map(|war| (war.attacker, war.defender)).collect();
    for (a, b, v) in w.relations.iter_mut() {
        if *v >= 0.0 {
            continue;
        }
        if belligerents.iter().any(|(x, y)| (x == a && y == b) || (x == b && y == a)) {
            continue; // an active war keeps the wound open
        }
        *v -= *v * 0.008; // ~9%/yr toward indifference
    }

    // ---- Sanctions relief: an embargo outlasts the war that caused it, and
    // lifts only once the grievance behind it has cooled. ----
    let decisions: Vec<((NationId, NationId), bool)> = w
        .sanctions
        .iter()
        .map(|&(imposer, target)| {
            let target_alive = w.nations.iter().any(|n| n.id == target && n.alive);
            // Grievance decay sets the clock: a minor partner's embargo fades in
            // ~5 years, a principal antagonist's holds for a decade.
            let keep = target_alive && (w.at_war(target) || w.relation(imposer, target) < -15.0);
            ((imposer, target), keep)
        })
        .collect();
    let mut kept: Vec<(NationId, NationId)> = vec![];
    let mut lifted: Vec<(NationId, NationId)> = vec![];
    for (pair, keep) in decisions {
        if keep {
            kept.push(pair);
        } else {
            lifted.push(pair);
        }
    }
    w.sanctions = kept;
    // Headline only when a target comes fully in from the cold.
    let freed: Vec<NationId> = lifted
        .iter()
        .map(|(_, t)| *t)
        .filter(|t| w.nations.iter().any(|n| n.id == *t && n.alive))
        .filter(|t| w.sanctioned_by_count(*t) == 0)
        .collect();
    for t in freed {
        if !w.headlines.iter().any(|h| h.contains(&format!("Sanctions on {}", t.name()))) {
            w.headline(format!("Sanctions on {} are lifted.", t.name()));
        }
    }
}

fn dissolve_ussr(w: &mut WorldState) {
    w.set_flag("ussr_dissolved");
    let (gdp, pop, oil, strength, inherited_tech) = {
        let u = w.nation(NationId::USSR);
        (u.gdp, u.population, u.oil_mbd, u.mil_strength, u.tech.clone())
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
        // The institutes and the engineers do not vanish with the flag over the
        // Kremlin; the research programmes they were working to do.
        tech: crate::tech::TechState::inherit(&inherited_tech, 0.008),
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

/// Yugoslavia comes apart into republics of unequal wealth and — the part that
/// decides everything — unequal ethnic homogeneity. Each successor inherits a
/// separatism value drawn from its real 1991 census: Slovenia is nearly all
/// Slovene and leaves almost intact, while Bosnia inherits a state with no
/// majority at all. Nothing here schedules a war; the strain is simply handed to
/// the successors, and the existing war machinery does what it does with it.
fn dissolve_yugoslavia(w: &mut WorldState) {
    w.set_flag("yugoslavia_dissolved");
    let (gdp, pop, oil, strength, infl, debt, inherited_tech) = {
        let y = w.nation(NationId::Yugoslavia);
        (y.gdp, y.population, y.oil_mbd, y.mil_strength, y.inflation, y.debt_gdp, y.tech.clone())
    };
    {
        let y = w.nation_mut(NationId::Yugoslavia);
        y.alive = false;
        y.gdp = 0.0;
    }

    // (id, GDP share, pop share, JNA share, separatism, authoritarianism, stability, tfp)
    // Macedonia's ~5% of output and ~9% of the people leave with it, unsimulated:
    // it seceded without a shot and never fought anyone.
    let parts: [(NationId, f64, f64, f64, f64, f64, f64, f64); 4] = [
        // Belgrade keeps the army, and Kosovo and Vojvodina keep Belgrade busy.
        (NationId::Serbia,   0.36, 0.42, 0.70, 0.45, 0.75, 40.0, 0.002),
        // A twelve percent Serb minority concentrated in the Krajina.
        (NationId::Croatia,  0.25, 0.20, 0.12, 0.35, 0.45, 45.0, 0.012),
        // ~88% Slovene, no minority worth a war, and the richest republic.
        (NationId::Slovenia, 0.20, 0.085, 0.08, 0.05, 0.25, 62.0, 0.020),
        // 44% Bosniak, 31% Serb, 17% Croat — a republic that is all minorities.
        (NationId::Bosnia,   0.13, 0.19, 0.05, 0.85, 0.40, 30.0, 0.006),
    ];
    for (id, g, p, m, sep, auth, stab, tfp) in parts {
        w.nations.push(Nation {
            id,
            alive: true,
            system: EconomySystem::Market, // the plan dies with the federation
            authoritarianism: auth,
            gdp: gdp * g,
            population: pop * p,
            tfp_trend: tfp,
            inflation: infl,
            interest_rate: 0.25,
            tax_rate: 0.33,
            mil_spend_gdp: 0.05,
            state_invest_gdp: 0.05,
            priv_invest_gdp: 0.10,
            debt_gdp: debt,
            oil_mbd: oil * g,
            bubble: 0.0,
            growth_last: -0.06,
            stability: stab,
            separatism: sep,
            mil_strength: strength * m,
            war_exhaustion: 0.0,
            nuclear: false,
            // Each republic keeps the federation's technical base and starts its
            // own research from nothing.
            tech: crate::tech::TechState::inherit(&inherited_tech, tfp),
        });
    }

    // Successors inherit the federation's standing abroad, thinned out.
    let inherited: Vec<(NationId, f64)> = ALL_START_NATIONS
        .iter()
        .filter(|x| **x != NationId::Yugoslavia)
        .map(|x| (*x, w.relation(NationId::Yugoslavia, *x) * 0.6))
        .collect();
    for (id, _, _, _, _, _, _, _) in parts {
        for (other, v) in &inherited {
            w.set_relation(id, *other, *v);
        }
    }
    // Bonn recognised Slovenia and Croatia early and over everyone's objections.
    w.shift_relation(NationId::Germany, NationId::Slovenia, 25.0);
    w.shift_relation(NationId::Germany, NationId::Croatia, 25.0);

    // How the republics regard each other on day one. Belgrade's claim on the
    // Serb minorities abroad is the fault line; Ljubljana has no such quarrel.
    let between: &[(NationId, NationId, f64)] = &[
        (NationId::Serbia, NationId::Croatia, -45.0),
        (NationId::Serbia, NationId::Bosnia, -35.0),
        (NationId::Serbia, NationId::Slovenia, -20.0),
        (NationId::Croatia, NationId::Bosnia, -10.0),
        (NationId::Croatia, NationId::Slovenia, 20.0),
        (NationId::Bosnia, NationId::Slovenia, 5.0),
    ];
    for (a, b, v) in between {
        w.set_relation(*a, *b, *v);
    }

    w.headline("YUGOSLAVIA HAS DISSOLVED. Slovenia, Croatia, Bosnia and Serbia stand alone.".into());
    w.headline("The JNA's divisions, and its arsenal, remain in Belgrade's hands.".into());
}

fn ai_wars(w: &mut WorldState) {
    // Iraq's Kuwait calculus: debt-strained oil state eyeing a rich, weak neighbor.
    let candidates: Vec<(NationId, NationId, f64)> = {
        let mut v = vec![];
        let aggressors = [
            NationId::Iraq, NationId::Iran, NationId::Pakistan, NationId::India,
            NationId::Serbia, NationId::Croatia,
        ];
        // Successor states are not in ALL_START_NATIONS, so consider everyone alive.
        let living: Vec<NationId> = w.nations.iter().filter(|n| n.alive).map(|n| n.id).collect();
        for a in aggressors {
            let an = match w.nation_opt(a) {
                Some(n) => n,
                None => continue, // has not been born yet, if it ever is
            };
            if !an.alive || Some(a) == w.player || w.at_war(a) || an.war_exhaustion > 0.3 {
                continue;
            }
            for t in living.iter().copied() {
                if t == a {
                    continue;
                }
                let tn = match w.nation_opt(t) {
                    Some(n) => n,
                    None => continue,
                };
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
                    // Wars of succession: neighbours of a state that just came
                    // apart, with kin on the wrong side of the new borders.
                    (NationId::Serbia, NationId::Croatia) => 0.022,
                    (NationId::Serbia, NationId::Bosnia) => 0.030,
                    (NationId::Serbia, NationId::Slovenia) => 0.010,
                    (NationId::Croatia, NationId::Bosnia) => 0.014,
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
                // Read the same list the coalition actually forms from, or the
                // lesson of a repelled invasion is never available to be learned.
                for m in war::MAJORS {
                    if war::would_intervene(w, m, t, a) {
                        expected_def += w.nation(m).mil_strength * coalition_discount;
                    }
                }
                let strength_ratio = an.mil_strength / expected_def.max(1.0);
                if strength_ratio < 0.8 {
                    continue; // deterred
                }
                // Fiscal desperation raises appetite (Iraq 1990); bad relations too
                let desperation = (an.debt_gdp - 0.6).max(0.0) * 1.5 + (0.4 - an.growth_last * 10.0).max(0.0) * 0.2;
                // A neighbour that cannot hold itself together is an invitation:
                // its own minorities are a lever, and its army is busy at home.
                let mut p = base
                    * w.rules.ai_aggression
                    * (1.0 + desperation)
                    * (1.0 + tn.separatism * 1.5)
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
