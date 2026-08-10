use crate::world::*;

/// Monthly military & war tick.
pub fn tick(w: &mut WorldState) {
    // ---- Strength accumulation from spending ----
    let ids: Vec<NationId> = w.nations.iter().filter(|n| n.alive).map(|n| n.id).collect();
    for id in &ids {
        let n = w.nation_mut(*id);
        let budget = n.gdp * n.mil_spend_gdp; // $bn/yr
        // Strength drifts toward what the budget sustains
        let sustained = (budget * 0.30).sqrt() * 8.0;
        n.mil_strength += (sustained - n.mil_strength) * 0.02;
        // Exhaustion decays in peace
        n.war_exhaustion = (n.war_exhaustion - 0.01).max(0.0);
    }

    // ---- Resolve wars ----
    let mut ended: Vec<(War, bool)> = vec![]; // (war, attacker_won)
    let mut continuing: Vec<War> = vec![];
    let wars = std::mem::take(&mut w.wars);

    for mut war in wars {
        let att = side_strength(w, &war, true);
        let def = side_strength(w, &war, false);
        let ratio = att / def.max(1.0);
        let push = (ratio.ln()) * 6.0 + w.rng.range(-2.0, 2.0);
        war.progress = (war.progress + push).clamp(-100.0, 100.0);

        // Exhaustion accrues, faster for the losing side
        for id in war_participants(&war) {
            let losing = match war.side_of(id) {
                Some(true) => war.progress < 0.0,
                Some(false) => war.progress > 0.0,
                None => false,
            };
            let n = w.nation_mut(id);
            n.war_exhaustion = (n.war_exhaustion + if losing { 0.030 } else { 0.018 }).min(1.0);
            // Attrition
            n.mil_strength *= if losing { 0.975 } else { 0.985 };
        }

        if war.progress >= 100.0 {
            ended.push((war, true));
        } else if war.progress <= -100.0 {
            ended.push((war, false));
        } else {
            // White peace when both sides are spent
            let a_ex = w.nation(war.attacker).war_exhaustion;
            let d_ex = w.nation(war.defender).war_exhaustion;
            if a_ex > 0.75 && d_ex > 0.75 {
                w.headline(format!(
                    "Exhausted, {} and {} sign a white peace.",
                    war.attacker.name(), war.defender.name()
                ));
                // drop the war
            } else {
                continuing.push(war);
            }
        }
    }
    w.wars = continuing;

    for (war, attacker_won) in ended {
        if attacker_won {
            let (winner, loser) = (war.attacker, war.defender);
            conquer(w, winner, loser);
        } else {
            // Defender victory: attacker's regime is humiliated, and the lesson sticks
            w.set_flag(&format!("burned_{:?}_{:?}", war.attacker, war.defender));
            let a = w.nation_mut(war.attacker);
            a.stability = (a.stability - 12.0).max(0.0);
            w.headline(format!(
                "{} repels {}'s invasion — the aggressor's regime totters.",
                war.defender.name(), war.attacker.name()
            ));
        }
    }
}

fn conquer(w: &mut WorldState, winner: NationId, loser: NationId) {
    let (lgdp, lpop, loil) = {
        let l = w.nation(loser);
        (l.gdp, l.population, l.oil_mbd)
    };
    // Annexing a whole large nation is beyond anyone's digestion — only small
    // states can be swallowed. Larger defeated nations are subjugated instead.
    if lpop < 8.0 {
        {
            let l = w.nation_mut(loser);
            l.alive = false;
            l.gdp = 0.0;
        }
        let n = w.nation_mut(winner);
        n.gdp += lgdp * 0.75; // war-damaged
        n.population += lpop;
        n.oil_mbd += loil * 0.85;
        n.stability = (n.stability + 6.0).min(100.0);
        n.separatism = (n.separatism + 0.15).min(1.0);
        w.headline(format!("{} has annexed {}.", winner.name(), loser.name()));
        w.wars.retain(|war| !war.involves(loser));
    } else {
        // Subjugation: reparations, ceded industry, a broken military, a shaken regime
        {
            let l = w.nation_mut(loser);
            l.gdp *= 0.85;
            l.mil_strength *= 0.4;
            l.stability = (l.stability - 20.0).max(5.0);
            l.war_exhaustion = 0.6;
        }
        {
            let n = w.nation_mut(winner);
            n.gdp += lgdp * 0.05; // reparations
            n.stability = (n.stability + 8.0).min(100.0);
        }
        w.set_relation(winner, loser, -80.0);
        w.headline(format!(
            "{} capitulates to {} — reparations, disarmament, humiliation.",
            loser.name(), winner.name()
        ));
    }
}

fn side_strength(w: &WorldState, war: &War, attacker: bool) -> f64 {
    let members: Vec<NationId> = if attacker {
        std::iter::once(war.attacker).chain(war.attacker_allies.iter().copied()).collect()
    } else {
        std::iter::once(war.defender).chain(war.defender_allies.iter().copied()).collect()
    };
    members
        .iter()
        .filter(|id| w.nation(**id).alive)
        .map(|id| {
            let n = w.nation(*id);
            n.mil_strength * (1.0 - n.war_exhaustion * 0.6)
        })
        .sum()
}

pub fn war_participants(war: &War) -> Vec<NationId> {
    let mut v = vec![war.attacker, war.defender];
    v.extend(&war.attacker_allies);
    v.extend(&war.defender_allies);
    v
}

/// Declare war, triggering coalition sanctions and possible interventions.
pub fn declare_war(w: &mut WorldState, attacker: NationId, defender: NationId) -> Result<(), String> {
    if attacker == defender {
        return Err("A nation cannot declare war on itself.".into());
    }
    if !w.nation(attacker).alive || !w.nation(defender).alive {
        return Err("Nation no longer exists.".into());
    }
    if w.wars.iter().any(|war| war.involves(attacker) && war.involves(defender)) {
        return Err("Already at war.".into());
    }
    // Nuclear taboo: no direct wars between nuclear powers
    if w.nation(attacker).nuclear && w.nation(defender).nuclear {
        return Err("Deterrence holds — direct war between nuclear powers is unthinkable.".into());
    }

    w.headline(format!("WAR: {} invades {}!", attacker.name(), defender.name()));
    w.shift_relation(attacker, defender, -60.0);

    let mut war = War {
        attacker,
        defender,
        attacker_allies: vec![],
        defender_allies: vec![],
        start_year: w.year,
        start_month: w.month,
        progress: 0.0,
    };

    // Coalition response: majors sanction the aggressor...
    let majors = [NationId::USA, NationId::UK, NationId::France, NationId::Germany, NationId::Japan];
    for m in majors {
        if m == attacker || !w.nation(m).alive {
            continue;
        }
        if !w.is_sanctioning(m, attacker) {
            w.sanctions.push((m, attacker));
        }
        w.shift_relation(m, attacker, -25.0);
    }
    w.headline(format!("Coalition sanctions slam {}.", attacker.name()));

    // ...and friends of the victim may intervene (never against a nuclear attacker directly
    // unless they're nuclear too — abstracted: majors intervene if relation with victim high).
    for m in majors {
        if m == attacker || !w.nation(m).alive {
            continue;
        }
        let rel = w.relation(m, defender);
        let att_nuclear = w.nation(attacker).nuclear;
        let m_nuclear = w.nation(m).nuclear;
        if rel >= 40.0 && (!att_nuclear || m_nuclear) {
            war.defender_allies.push(m);
            w.headline(format!("{} joins the war in defense of {}.", m.name(), defender.name()));
        }
    }

    w.wars.push(war);
    Ok(())
}
