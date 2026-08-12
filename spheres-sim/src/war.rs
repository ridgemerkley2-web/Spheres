use crate::theatre::{self, TheatreId};
use crate::world::*;

// ---------------------------------------------------------------------------
// The force package: what a rung actually puts in the field
// ---------------------------------------------------------------------------

/// What share of its force structure each rung actually commits. Rung 9 —
/// occupation — fields less combat power than 8 and vastly more garrison-months.
pub const RUNG_COMMIT: [f64; 10] = [0.0, 0.00, 0.00, 0.00, 0.02, 0.05, 0.15, 0.35, 0.85, 0.60];

/// How much of itself a side standing on a rung offers to be found and killed.
/// This is the capability gate, and it is one term in a multiplication rather
/// than a branch: an army in the open at rung 8 is a target set, and a proxy at
/// rung 4 in a city in rough country is not.
pub const RUNG_EXPOSURE: [f64; 10] = [0.0, 0.02, 0.04, 0.08, 0.10, 0.15, 0.45, 0.70, 1.00, 0.55];

/// Only a side that has put boots or hulls on the ground can take any.
pub const SEIZE_BY_RUNG: [f64; 10] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.05, 0.40, 1.00, 0.25];

/// Share of the magazines a month at each rung consumes. Rung 6 — standoff
/// strike — is the hungriest per unit of effect, which is exactly why an air
/// campaign cannot be sustained indefinitely.
pub const BURN_BY_RUNG: [f64; 10] = [0.0, 0.0, 0.0, 0.0, 0.002, 0.005, 0.090, 0.070, 0.140, 0.045];

/// What it costs in political capital to *arrive* at each rung, summed over the
/// rungs crossed. Rung 8 costs 30 in total, which is exactly what declaring war
/// has always cost — that price is preserved rather than reinvented.
pub const ESCALATION_PRICE: [f64; 10] = [0.0, 0.0, 2.0, 6.0, 8.0, 10.0, 14.0, 20.0, 30.0, 40.0];

/// The number BIBLE §6 says does more work than anything else in the model: the
/// share of its force structure a nation can have abroad and sustained at once.
///
/// Derived, never authored. A country-named special case would mean the model is
/// wrong (§7), so this reads only quantities the sim already keeps: how much
/// budget stands behind each point of structure — capital intensity, which is
/// what buys airlift, sealift, tankers and the logistics behind them — lifted by
/// what the nation technologically knows how to do with it.
///
/// Against the transcribed 1990 data this lands the United States near 0.14, the
/// Soviet Union near 0.10, Iraq under 0.04 and Vietnam at the floor. Nobody
/// typed any of those figures.
pub fn deployable_fraction(w: &WorldState, id: NationId) -> f64 {
    let n = match w.nation_opt(id) {
        Some(n) => n,
        None => return 0.02,
    };
    let per_point = (n.gdp * n.mil_spend_gdp) / n.mil_strength.max(1.0);
    let capital = (per_point / 3.0).clamp(0.0, 1.2);
    let lift = 0.30 + 0.70 * ((crate::tech::military_multiplier(n) - 0.5) / 3.5).clamp(0.0, 1.0);
    (0.02 + 0.30 * capital * lift).clamp(0.02, 0.40)
}

/// A dry magazine does not stop an army; it stops it being able to do the thing
/// its rung says it is doing.
pub fn magazine_multiplier(w: &WorldState, id: NationId) -> f64 {
    let m = w.nation_opt(id).map_or(1.0, |n| n.munitions);
    if m >= 0.15 {
        1.0
    } else {
        0.35 + 0.65 * (m / 0.15).clamp(0.0, 1.0)
    }
}

/// What a belligerent actually has in the fight this month.
///
/// `mil_strength` is force structure and no longer decides anything by itself.
/// Everything between it and combat power is a decision somebody made: which
/// rung to stand on, whether the fight is at home, whether a third state has
/// consented to host you, and whether the magazines are still full.
pub fn committed_force(w: &WorldState, c: &Conflict, id: NationId) -> f64 {
    let b = match c.posture_of(id) {
        Some(b) => b,
        None => return 0.0,
    };
    let structure = match w.nation_opt(id) {
        Some(n) if n.alive => n.mil_strength,
        _ => return 0.0,
    };
    let proj = if theatre::is_home(w, id, c.theatre) {
        1.0
    } else {
        deployable_fraction(w, id)
    };
    // No consenting host in range does not mean nothing arrives; it means very
    // little does, and by sea.
    let acc = if theatre::has_access(w, id, c.theatre) { 1.0 } else { 0.25 };
    structure * RUNG_COMMIT[b.rung.min(9) as usize] * proj * acc * magazine_multiplier(w, id)
}

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

    // ---- Resolve conflicts ----
    let mut ended: Vec<(Conflict, bool)> = vec![]; // (conflict, side A won)
    let mut continuing: Vec<Conflict> = vec![];
    let conflicts = std::mem::take(&mut w.conflicts);

    for mut c in conflicts {
        let att = side_strength(w, &c, true);
        let def = side_strength(w, &c, false);
        let ratio = att / def.max(1.0);
        let push = (ratio.ln()) * 6.0 + w.rng.range(-2.0, 2.0);
        let progress = (c.control * 100.0 + push).clamp(-100.0, 100.0);
        c.control = progress / 100.0;
        c.months += 1;

        // Exhaustion accrues, faster for the losing side
        for id in c.participants() {
            let losing = match c.side_of(id) {
                Some(true) => progress < 0.0,
                Some(false) => progress > 0.0,
                None => false,
            };
            let n = w.nation_mut(id);
            n.war_exhaustion = (n.war_exhaustion + if losing { 0.030 } else { 0.018 }).min(1.0);
            // Attrition
            n.mil_strength *= if losing { 0.975 } else { 0.985 };
        }

        if progress >= 100.0 {
            ended.push((c, true));
        } else if progress <= -100.0 {
            ended.push((c, false));
        } else {
            // White peace when both sides are spent
            let a_ex = w.nation(c.attacker()).war_exhaustion;
            let d_ex = w.nation(c.defender()).war_exhaustion;
            if a_ex > 0.75 && d_ex > 0.75 {
                w.headline(format!(
                    "Exhausted, {} and {} sign a white peace.",
                    c.attacker().name(), c.defender().name()
                ));
                // drop the conflict
            } else if let Some((winner, loser)) = settlement_ripe(w, &c) {
                negotiated_peace(w, winner, loser);
            } else {
                continuing.push(c);
            }
        }
    }
    w.conflicts = continuing;

    for (c, side_a_won) in ended {
        if side_a_won {
            let (winner, loser) = (c.attacker(), c.defender());
            conquer(w, winner, loser);
        } else {
            // Defender victory: attacker's regime is humiliated, and the lesson sticks
            w.set_flag(&format!("burned_{:?}_{:?}", c.attacker(), c.defender()));
            let a = w.nation_mut(c.attacker());
            a.stability = (a.stability - 12.0).max(0.0);
            w.headline(format!(
                "{} repels {}'s invasion — the aggressor's regime totters.",
                c.defender().name(), c.attacker().name()
            ));
        }
    }
}

/// Most wars end at a table, not in a capital. A side losing clearly but not yet
/// catastrophically will buy its way out — if it is weary enough to swallow terms
/// and its enemy is weary enough to offer them instead of pressing for the whole prize.
fn settlement_ripe(w: &mut WorldState, c: &Conflict) -> Option<(NationId, NationId)> {
    let progress = c.control * 100.0;
    let lead = progress.abs();
    if !(35.0..100.0).contains(&lead) {
        return None;
    }
    let (winner, loser) = if progress > 0.0 {
        (c.attacker(), c.defender())
    } else {
        (c.defender(), c.attacker())
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
        w.conflicts.retain(|c| !c.involves(loser));
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

fn side_strength(w: &WorldState, c: &Conflict, side_a: bool) -> f64 {
    let members = if side_a { &c.side_a } else { &c.side_b };
    members
        .iter()
        .filter(|id| w.nation_opt(**id).map_or(false, |n| n.alive))
        .map(|id| {
            let n = w.nation(*id);
            n.mil_strength * (1.0 - n.war_exhaustion * 0.6)
        })
        .sum()
}

pub fn conflict_participants(c: &Conflict) -> Vec<NationId> {
    c.participants()
}

/// The powers that sanction an aggressor and may intervene for its victim.
/// The AI's expectations in `politics::ai_wars` read from this same list — if
/// they diverge, aggressors invade into coalitions they never saw coming and
/// never learn better.
pub const MAJORS: [NationId; 5] = [
    NationId::USA, NationId::UK, NationId::France, NationId::Germany, NationId::Japan,
];

/// Would `m` come to `victim`'s defence?
pub fn would_intervene(w: &WorldState, m: NationId, victim: NationId, attacker: NationId) -> bool {
    if m == attacker || m == victim || w.nation_opt(m).map_or(true, |n| !n.alive) {
        return false;
    }
    let att_nuclear = w.nation(attacker).nuclear;
    let m_nuclear = w.nation(m).nuclear;
    w.relation(m, victim) >= 40.0 && (!att_nuclear || m_nuclear)
}

/// Where a war between these two is fought: the defender's own ground if it has
/// any, otherwise the attacker's, otherwise the sea lanes.
pub fn theatre_between(w: &WorldState, attacker: NationId, defender: NationId) -> TheatreId {
    theatre::home_theatre(w, defender)
        .or_else(|| theatre::home_theatre(w, attacker))
        .unwrap_or(TheatreId::Maritime)
}

/// Declare war, triggering coalition sanctions and possible interventions.
///
/// Kept as a public entry point and as sugar over the ladder: it opens a
/// conflict with the attacker already standing on rung 8, a full conventional
/// campaign, objective Seize. The headline is byte-identical to the one three
/// emergent-history tests match on, and that is deliberate.
pub fn declare_war(w: &mut WorldState, attacker: NationId, defender: NationId) -> Result<(), String> {
    if attacker == defender {
        return Err("A nation cannot declare war on itself.".into());
    }
    if !w.nation(attacker).alive || !w.nation(defender).alive {
        return Err("Nation no longer exists.".into());
    }
    if w.conflicts.iter().any(|c| c.involves(attacker) && c.involves(defender)) {
        return Err("Already at war.".into());
    }
    // Nuclear taboo: no direct wars between nuclear powers
    if w.nation(attacker).nuclear && w.nation(defender).nuclear {
        return Err("Deterrence holds — direct war between nuclear powers is unthinkable.".into());
    }

    w.headline(format!("WAR: {} invades {}!", attacker.name(), defender.name()));
    w.shift_relation(attacker, defender, -60.0);

    let th = theatre_between(w, attacker, defender);
    let mut c = Conflict {
        id: w.next_conflict_id(),
        theatre: th,
        side_a: vec![attacker],
        side_b: vec![defender],
        posture: vec![
            Belligerent::new(attacker, 8, Objective::Seize),
            Belligerent::new(defender, 8, Objective::Hold),
        ],
        control: 0.0,
        months: 0,
        frozen_since: None,
        start_year: w.year,
        start_month: w.month,
        origin_attacker: attacker,
    };
    if let Some(b) = c.posture_mut(defender) {
        b.stake = 1.0;
    }

    // Coalition response: majors sanction the aggressor...
    let majors = MAJORS;
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

    // Written guarantees are called in first. A pact is a harder claim than
    // affinity and, unlike affinity, it can be publicly broken — so anyone who
    // walks away here must not be quietly walked back in by the looser rule below.
    let refused = crate::statecraft::call_the_guarantors(w, &mut c);

    // ...and friends of the victim may intervene (never against a nuclear attacker directly
    // unless they're nuclear too — abstracted: majors intervene if relation with victim high).
    for m in majors {
        if c.side_b.contains(&m) || refused.contains(&m) {
            continue;
        }
        if would_intervene(w, m, defender, attacker) {
            join_side(&mut c, m, false, 8, Objective::Deny);
            w.headline(format!("{} joins the war in defense of {}.", m.name(), defender.name()));
        }
    }

    w.conflicts.push(c);
    Ok(())
}

/// Put a nation into a conflict on one side, with its own opening posture. The
/// one place a coalition grows, so the posture vector can never drift out of
/// step with the two side lists.
pub fn join_side(c: &mut Conflict, id: NationId, side_a: bool, rung: u8, objective: Objective) {
    if c.involves(id) {
        return;
    }
    if side_a {
        c.side_a.push(id);
    } else {
        c.side_b.push(id);
    }
    c.posture.push(Belligerent::new(id, rung, objective));
}
