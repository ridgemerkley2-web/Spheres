use crate::world::*;

/// Monthly military & war tick.
pub fn tick(w: &mut WorldState) {
    // ---- Strength accumulation from spending ----
    let ids: Vec<NationId> = w.nations.iter().filter(|n| n.alive).map(|n| n.id).collect();
    for id in &ids {
        let n = w.nation_mut(*id);
        let budget = n.gdp * n.mil_spend_gdp; // $bn/yr
        // Strength drifts toward what the budget sustains — and what a budget
        // sustains depends on what the arsenal it buys is made of.
        let sustained = (budget * 0.30).sqrt() * 8.0 * crate::tech::military_multiplier(n)
            + crate::tech::military_floor(n);
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
            } else if let Some((winner, loser)) = settlement_ripe(w, &war) {
                negotiated_peace(w, winner, loser);
                w.set_flag(&crate::dyads::settled_flag(winner, loser));
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
            // The claim has been pressed to a conclusion. It survives as a
            // grievance and stops being a war aim — the counterpart of the
            // `burned_` lesson below, and the thing whose absence had states
            // relaunching the same invasion every second year for a century.
            w.set_flag(&crate::dyads::settled_flag(winner, loser));
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

/// Most wars end at a table, not in a capital. A side losing clearly but not yet
/// catastrophically will buy its way out — if it is weary enough to swallow terms
/// and its enemy is weary enough to offer them instead of pressing for the whole prize.
fn settlement_ripe(w: &mut WorldState, war: &War) -> Option<(NationId, NationId)> {
    let lead = war.progress.abs();
    if !(35.0..100.0).contains(&lead) {
        return None;
    }
    let (winner, loser) = if war.progress > 0.0 {
        (war.attacker, war.defender)
    } else {
        (war.defender, war.attacker)
    };
    let loser_ex = w.nation(loser).war_exhaustion;
    let winner_ex = w.nation(winner).war_exhaustion;
    if loser_ex < 0.55 {
        return None; // still has fight in it
    }
    // The riper the position and the wearier both sides, the likelier the deal.
    let p = 0.10 * (lead / 100.0) * loser_ex * (0.5 + winner_ex);
    if w.rng.chance(p) {
        Some((winner, loser))
    } else {
        None
    }
}

/// Terms short of conquest: reparations always, territory only from a state that
/// cannot answer with annihilation.
fn negotiated_peace(w: &mut WorldState, winner: NationId, loser: NationId) {
    let (lgdp, lpop, loil, lnuclear) = {
        let l = w.nation(loser);
        (l.gdp, l.population, l.oil_mbd, l.nuclear)
    };
    let cede = if lnuclear { 0.0 } else { 0.12 };
    {
        let l = w.nation_mut(loser);
        l.gdp -= lgdp * (0.03 + cede * 0.5);
        l.population -= lpop * cede;
        l.oil_mbd -= loil * cede;
        l.stability = (l.stability - 10.0).max(5.0);
        l.mil_strength *= 0.80;
    }
    {
        let n = w.nation_mut(winner);
        n.gdp += lgdp * (0.02 + cede * 0.4);
        n.population += lpop * cede;
        n.oil_mbd += loil * cede;
        n.stability = (n.stability + 4.0).min(100.0);
        if cede > 0.0 {
            // Swallowed land comes with people who did not choose you.
            n.separatism = (n.separatism + 0.05).min(1.0);
        }
    }
    w.set_relation(winner, loser, -55.0);
    if cede > 0.0 {
        w.headline(format!(
            "{} sues for peace, ceding territory to {}.",
            loser.name(), winner.name()
        ));
    } else {
        w.headline(format!(
            "{} and {} agree peace terms — reparations, no territory.",
            loser.name(), winner.name()
        ));
    }
}

fn conquer(w: &mut WorldState, winner: NationId, loser: NationId) {
    let (lgdp, lpop, loil) = {
        let l = w.nation(loser);
        (l.gdp, l.population, l.oil_mbd)
    };
    // Annexing a whole nation is beyond anyone's digestion. Only small states
    // can be swallowed — and only quiet ones: a territory that is all minorities
    // is an occupation, not an acquisition. Larger or angrier nations are
    // subjugated instead, and survive to resent it.
    let lsep = w.nation(loser).separatism;
    if lpop < 8.0 && lsep < 0.6 {
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

// The powers that sanction an aggressor and may intervene for its victim are a
// flag on the roster row now — see `nations::majors`. `dyads::war_appetite`
// reads that same list when it works out what an aggressor expects to face; if
// the two ever diverge, aggressors invade into coalitions they never saw coming
// and never learn better.

/// Would `m` come to `victim`'s defence?
pub fn would_intervene(w: &WorldState, m: NationId, victim: NationId, attacker: NationId) -> bool {
    if m == attacker || m == victim || w.nation_opt(m).map_or(true, |n| !n.alive) {
        return false;
    }
    let att_nuclear = w.nation(attacker).nuclear;
    let m_nuclear = w.nation(m).nuclear;
    w.relation(m, victim) >= 40.0 && (!att_nuclear || m_nuclear)
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
    let majors: Vec<NationId> = majors().to_vec();
    for m in majors.iter().copied() {
        if m == attacker || !w.nation(m).alive {
            continue;
        }
        if !w.is_sanctioning(m, attacker) {
            w.sanctions.push((m, attacker));
        }
        w.shift_relation(m, attacker, -25.0);
    }
    w.headline(format!("Coalition sanctions slam {}.", attacker.name()));

    // Written guarantees are called in first. A pact is a harder claim than
    // affinity and, unlike affinity, it can be publicly broken — so anyone who
    // walks away here must not be quietly walked back in by the looser rule below.
    let refused = crate::statecraft::call_the_guarantors(w, &mut war);

    // ...and friends of the victim may intervene (never against a nuclear attacker directly
    // unless they're nuclear too — abstracted: majors intervene if relation with victim high).
    for m in majors.iter().copied() {
        if war.defender_allies.contains(&m) || refused.contains(&m) {
            continue;
        }
        if would_intervene(w, m, defender, attacker) {
            war.defender_allies.push(m);
            w.headline(format!("{} joins the war in defense of {}.", m.name(), defender.name()));
        }
    }

    w.wars.push(war);
    Ok(())
}
