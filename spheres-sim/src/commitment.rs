//! The commitment ladder — BIBLE §6 object 2, and the mechanic the whole war
//! layer exists for.
//!
//! Nine rungs, each side picking its own, monthly. What makes it a ladder rather
//! than a difficulty slider is that the middle of it is *already built*: rungs 2
//! through 5 are the four statecraft instruments SPHERES has had since the
//! spheres-of-influence branch — sanctions, arms to a proxy, advisers,
//! deniable action. This module does not reimplement any of them. It issues the
//! same `Command`s a player issues, through `apply_command`, so they are priced
//! and refusable on exactly the player's terms. A government without the
//! standing to sanction somebody literally cannot climb to rung 2, and that is
//! the political-capital bite arriving for free rather than being bolted on.
//!
//! Above rung 5 there is nothing to bind, because there is no instrument short
//! of force: rungs 6 to 9 are the resolver's own arithmetic in `war.rs`.

use crate::theatre::{self, TheatreId, MAX_RUNG_WITHOUT_ACCESS};
use crate::war::ESCALATION_PRICE;
use crate::world::*;
use crate::{apply_command, Command};

/// What climbing from `from` to `to` asks of the government that does it.
///
/// The table is cumulative — what it costs to *be* standing on a rung — so a
/// climb is a difference and the price of arriving at rung 8 is 30 however many
/// steps you took to get there. That is not a coincidence: 30 is what declaring
/// war has always cost, and the ladder preserves the price rather than
/// reinventing it.
///
/// A democracy pays more. Escalation has to be explained to a parliament and an
/// electorate; a police state can simply do it. Descending is free here and
/// charged in reputation instead — cheap at home, expensive abroad.
///
/// `defending_home` is the one discount, and it is the difference between an
/// expedition and a defence: a government has to be talked into sending an army
/// abroad and does not have to be talked into meeting one at the frontier.
/// Without it the model charged Kuwait the same to resist an invasion as it
/// charged Iraq to mount one, and small states simply could not afford to fight
/// for themselves.
pub const HOME_DEFENCE_DISCOUNT: f64 = 0.30;

/// What it costs a government in political capital to climb from one rung to
/// another, at home or abroad.
///
/// Thin wrapper over [`escalation_cost_in`] for the common case where the
/// question is not about defending your own ground.
pub fn escalation_cost(w: &WorldState, id: NationId, from: u8, to: u8) -> f64 {
    escalation_cost_in(w, id, from, to, false)
}

/// The political price of climbing the ladder from `from` to `to`.
///
/// Zero when `to` is at or below `from`: walking back down costs a government
/// nothing it has not already spent. Defending your own territory is cheaper
/// than projecting force into somebody else's, which is what `defending_home`
/// selects — a parliament that will not fund an expedition will fund a defence.
pub fn escalation_cost_in(
    w: &WorldState,
    id: NationId,
    from: u8,
    to: u8,
    defending_home: bool,
) -> f64 {
    if to <= from {
        return 0.0;
    }
    let lo = ESCALATION_PRICE[(from as usize).min(9)];
    let hi = ESCALATION_PRICE[(to as usize).min(9)];
    let auth = w.nation_opt(id).map_or(0.5, |n| n.authoritarianism);
    let home = if defending_home { HOME_DEFENCE_DISCOUNT } else { 1.0 };
    (hi - lo) * (1.4 - 0.6 * auth) * home
}

/// Whether this belligerent is answering on its own ground rather than
/// projecting onto somebody else's: the theatre is its home, and it did not
/// open the quarrel.
pub fn defending_home(w: &WorldState, c: &Conflict, id: NationId) -> bool {
    c.side_of(id) != c.side_of(c.origin_attacker) && theatre::is_home(w, id, c.theatre)
}

/// Why a nation cannot stand where it wants to. Returned as prose because a
/// disabled rung that explains itself is the whole tutorial.
pub fn rung_blocked(w: &WorldState, c: &Conflict, id: NationId, rung: u8) -> Option<String> {
    if !(1..=9).contains(&rung) {
        return Some("There are nine rungs.".into());
    }
    let b = c.posture_of(id)?;
    if rung > b.ceiling {
        return Some(format!(
            "You have publicly bound yourself to rung {} or below.",
            b.ceiling
        ));
    }
    if rung > MAX_RUNG_WITHOUT_ACCESS && !theatre::has_access(w, id, c.theatre) {
        let hosts: Vec<&str> = theatre::theatre(w, c.theatre)
            .access_hosts
            .iter()
            .filter(|h| **h != id)
            .map(|h| h.name())
            .collect();
        return Some(format!(
            "No consenting host within range of {}: {} would have to agree to basing.",
            c.theatre.name(),
            if hosts.is_empty() { "nobody in reach".to_string() } else { hosts.join(" or ") }
        ));
    }
    // The taboo, and it binds harder here than it ever did at the declaration:
    // it is not a war between nuclear powers that is unthinkable, it is anyone
    // choosing to start shooting at one without an arsenal to answer with.
    //
    // It binds the expedition, never the defence. A state fighting on its own
    // ground fights with everything it has and asks nobody's permission —
    // Iraq in 1991, Serbia in 1999, Vietnam for a decade. Deterrence is a
    // reason not to go somewhere, not a reason to stand still while somebody
    // arrives. Written the other way it would make every non-nuclear country on
    // earth unable to resist a great power, which is the opposite of the fact
    // it is meant to encode.
    if rung >= SHOOTING_RUNG && !theatre::is_home(w, id, c.theatre) {
        let mine = w.nation_opt(id).is_some_and(|n| n.nuclear);
        let side = c.side_of(id);
        let armed_enemy = c.participants().iter().any(|o| {
            c.side_of(*o) != side && w.nation_opt(*o).is_some_and(|n| n.alive && n.nuclear)
        });
        if armed_enemy && !mine {
            return Some("Deterrence holds — they have the bomb and we do not.".into());
        }
    }
    None
}

/// Move a belligerent to a rung, and bring the instruments of the rungs it
/// crossed with it.
pub fn set_commitment(
    w: &mut WorldState,
    conflict: u32,
    nation: NationId,
    rung: u8,
) -> Result<(), String> {
    let (old, opponent, ally) = {
        let c = w.conflict(conflict).ok_or("No such conflict.")?;
        if let Some(why) = rung_blocked(w, c, nation, rung) {
            return Err(why);
        }
        let b = c.posture_of(nation).ok_or("Not a party to that conflict.")?;
        (b.rung, primary_opponent(c, nation), proxy_of(w, c, nation))
    };
    if old == rung {
        return Ok(());
    }
    w.daily.counters.remove(&format!("war:{conflict}:rung:{nation:?}"));
    if rung > old { w.daily.counters.remove(&format!("war:{conflict}:quiet")); }
    {
        let c = w.conflict_mut(conflict).expect("checked");
        if rung > old {
            // A quarrel somebody is actively climbing is not a frozen one, and
            // the freeze clock must not be allowed to kill a climb halfway up:
            // eighteen months is less than it takes a poor government to save
            // the standing for seven rungs. Only a climb does this — walking
            // back down leaves the clock running, which is what lets a quarrel
            // nobody is prosecuting finally fall off the board.
            c.quiet_months = 0;
            c.frozen_since = None;
        }
        let b = c.posture_mut(nation).expect("checked");
        b.rung = rung;
        b.months_at_rung = 0;
    }
    if rung > old {
        w.headline(format!(
            "{} escalates to rung {} — {}.",
            nation.name(),
            rung,
            rung_name(rung)
        ));
        let aggressor_side = w
            .conflict(conflict)
            .is_some_and(|c| c.side_of(nation) == c.side_of(c.origin_attacker));

        // A guarantee answers the shooting, not only the border crossing.
        //
        // Before the ladder there was one rung — war — and the guarantors were
        // called at it. The ladder put two rungs of real fire underneath it,
        // standoff strike and blockade, and leaving the call at rung 8 meant a
        // patron watched its client be bombed for years and was never asked.
        // Measured across twelve thirty-year runs it was worse than that: an
        // aggressor now weighs the opposition again at every step of the climb,
        // so a guaranteed state was never even climbed at. 2 of 77 invasions
        // fell on a guaranteed defender, against 13 of 197 wars before the
        // ladder, and `a_pact_drags_a_great_power_into_a_war_it_did_not_start`
        // read 1/12 runs against a floor of 3. Every guarantee had become a
        // border, which is exactly what that test's comment says a guarantee
        // must not be. The call belongs at the rung where force starts.
        if rung >= SHOOTING_RUNG && old < SHOOTING_RUNG && aggressor_side {
            if let Some(idx) = w.conflicts.iter().position(|c| c.id == conflict) {
                // Out of the world and back at the same index, because the
                // guarantors need a mutable conflict and a mutable world at
                // once and conflict order is iterated everywhere.
                let mut c = w.conflicts.remove(idx);
                crate::statecraft::call_the_guarantors(w, &mut c, rung);
                w.conflicts.insert(idx, c);
            }
        }

        // The step that is different in kind from the seven below it. Climbing
        // to a full conventional campaign against somebody who is not in your
        // country is an invasion, and the world answers invasions: sanctions,
        // guarantees called in, friends of the victim arriving. It happens once
        // per quarrel, at the rung, rather than at the moment somebody decided
        // to be annoyed — which is the whole of QA's first finding.
        let crossing = rung >= INVASION_RUNG
            && w.conflict(conflict).is_some_and(|c| !c.invasion_declared)
            && aggressor_side;
        if crossing {
            crate::war::invasion_begins(w, conflict, nation);
        }
    } else {
        // De-escalating is free at home and expensive abroad: the reputation
        // goes, and every client you were paying to fight for you notices.
        let dropped = (old - rung) as f64;
        w.shift_reputation(nation, -4.0 * dropped);
        let clients: Vec<NationId> = w
            .statecraft
            .aid
            .iter()
            .filter(|f| f.patron == nation)
            .map(|f| f.client)
            .collect();
        for cl in clients {
            w.shift_relation(nation, cl, -3.0);
        }
        w.headline(format!(
            "{} steps back to rung {} — {}.",
            nation.name(),
            rung,
            rung_name(rung)
        ));
    }
    bind_instruments(w, nation, opponent, ally, old, rung);
    Ok(())
}

/// The binding itself. Everything here is an existing command; nothing here is a
/// new system. Climbing installs, descending lifts.
fn bind_instruments(
    w: &mut WorldState,
    nation: NationId,
    opponent: Option<NationId>,
    proxy: Option<NationId>,
    old: u8,
    new: u8,
) {
    let opp = match opponent {
        Some(o) => o,
        None => return,
    };
    let climbing = new > old;
    if climbing {
        if new >= 2 && old < 2 {
            let _ = apply_command(w, &Command::Sanction { imposer: nation, target: opp });
        }
        // Rung 3 is arms to a proxy and rung 4 is arms plus the people who know
        // how to use them, which is the same aid flow at a heavier rate.
        if new >= 3 && old < 3 {
            if let Some(p) = proxy {
                let share = if new >= 4 { 0.003 } else { 0.002 };
                let _ = apply_command(
                    w,
                    &Command::PledgeAid { patron: nation, client: p, kind: AidKind::Arms, share_gdp: share },
                );
            }
        } else if new >= 4 && old < 4 {
            if let Some(p) = proxy {
                let _ = apply_command(
                    w,
                    &Command::PledgeAid { patron: nation, client: p, kind: AidKind::Arms, share_gdp: 0.003 },
                );
            }
        }
    } else {
        if new < 3 && old >= 3 {
            if let Some(p) = proxy {
                let _ = apply_command(
                    w,
                    &Command::EndAid { patron: nation, client: p, kind: AidKind::Arms },
                );
            }
        }
        if new < 2 && old >= 2 {
            let _ = apply_command(w, &Command::LiftSanction { imposer: nation, target: opp });
        }
    }
}

/// Rung 5 — deniable forces — is not a state you enter once; it is a thing a
/// service keeps doing, and keeps being able to be caught doing. Run quarterly
/// from the war tick so that standing there shows up in `covert_heat` exactly as
/// a player's own covert action would.
pub fn deniable_forces_upkeep(w: &mut WorldState) {
    if !crate::clock::is_daily(w) && w.month % 3 != 1 {
        return;
    }
    let mut jobs: Vec<(NationId, NationId)> = w
        .conflicts
        .iter()
        .flat_map(|c| {
            c.posture
                .iter()
                .filter(|b| b.rung == 5)
                .filter_map(|b| primary_opponent(c, b.nation).map(|o| (b.nation, o)))
                .collect::<Vec<_>>()
        })
        .collect();
    if crate::clock::is_daily(w) {
        jobs.sort();
        jobs.dedup();
        let active: Vec<_> = jobs.iter().map(|(a,b)| format!("deniable:{a:?}:{b:?}")).collect();
        w.daily.counters.retain(|key, _| !key.starts_with("deniable:") || active.contains(key));
    }
    for (sponsor, target) in jobs {
        if Some(sponsor) == w.player {
            continue; // the player's own services do what the player tells them
        }
        if !crate::clock::interval_due(w, format!("deniable:{sponsor:?}:{target:?}"), 3.0) { continue; }
        let _ = apply_command(
            w,
            &Command::CovertAction { sponsor, target, op: CovertOp::StirSeparatists },
        );
    }
}

/// Who this belligerent is actually fighting: the first living state on the
/// other side, which is the one everything downstream names.
pub fn primary_opponent(c: &Conflict, id: NationId) -> Option<NationId> {
    let side = c.side_of(id)?;
    let others = if side { &c.side_b } else { &c.side_a };
    others.first().copied()
}

/// Whose war this actually is on your own side — the local party a great power
/// arms instead of fighting itself. Home ground and the smallest army: the
/// client, not the patron.
fn proxy_of(w: &WorldState, c: &Conflict, id: NationId) -> Option<NationId> {
    let side = c.side_of(id)?;
    let mine = if side { &c.side_a } else { &c.side_b };
    mine.iter()
        .copied()
        .filter(|x| *x != id)
        .filter(|x| w.nation_opt(*x).is_some_and(|n| n.alive))
        .filter(|x| theatre::is_home(w, *x, c.theatre))
        .min_by(|a, b| {
            let (x, y) = (w.nation(*a).mil_strength, w.nation(*b).mil_strength);
            x.partial_cmp(&y).unwrap_or(std::cmp::Ordering::Equal)
        })
}

// ---------------------------------------------------------------------------
// Opening a conflict, which is the cheapest thing in this file on purpose
// ---------------------------------------------------------------------------

/// Conflicts begin when somebody climbs, not with a declaration. Opening one at
/// rung 1 — rhetoric — costs almost nothing, and that cheapness is what makes
/// the first rung a real option rather than a formality.
pub fn open_conflict(
    w: &mut WorldState,
    opener: NationId,
    target: NationId,
    th: TheatreId,
) -> Result<u32, String> {
    if opener == target {
        return Err("A nation cannot open a conflict with itself.".into());
    }
    if w.nation_opt(opener).is_none_or(|n| !n.alive)
        || w.nation_opt(target).is_none_or(|n| !n.alive)
    {
        return Err("Nation no longer exists.".into());
    }
    if let Some(c) = w.conflict_between(opener, target) {
        return Ok(c.id);
    }
    let id = w.next_conflict_id();
    let mut b_def = Belligerent::new(target, 1, Objective::Hold);
    b_def.stake = if theatre::is_home(w, target, th) { 1.0 } else { 0.45 };
    let c = Conflict {
        id,
        theatre: th,
        side_a: vec![opener],
        side_b: vec![target],
        posture: vec![Belligerent::new(opener, 1, Objective::Deny), b_def],
        control: 0.0,
        months: 0,
        quiet_months: 0,
        frozen_since: None,
        start_year: w.year,
        start_month: w.month,
        origin_attacker: opener,
        invasion_declared: false,
        front: std::collections::BTreeMap::new(),
        pockets: vec![],
        aim: None,
    };
    w.conflicts.push(c);
    w.headline(format!(
        "{} opens a public quarrel with {} over {}.",
        opener.name(),
        target.name(),
        th.name()
    ));
    Ok(id)
}

/// Take a side in somebody else's quarrel.
///
/// The verb that was missing, and its absence was QA's third finding: playing
/// the United States, watching Iraq climb toward Kuwait, there was no way to
/// become a party to it, so every ladder command answered "not a party to that
/// conflict" and the whole war layer was unreachable from the only seat a
/// player ever sits in. The AI got in through `invasion_begins`; nobody else
/// could.
///
/// Joining is entering at the BOTTOM — rung 1, rhetoric — because that is what
/// taking a side actually is. Everything after it is the ladder, bought a rung
/// at a time, which is the mechanic working rather than being bypassed.
pub fn join_conflict(
    w: &mut WorldState,
    joiner: NationId,
    conflict: u32,
    side_a: bool,
    objective: Objective,
) -> Result<(), String> {
    if w.nation_opt(joiner).is_none_or(|n| !n.alive) {
        return Err("Nation no longer exists.".into());
    }
    let (already, th, friends, foes) = {
        let c = w.conflict(conflict).ok_or("No such conflict.")?;
        let (mine, theirs) = if side_a { (&c.side_a, &c.side_b) } else { (&c.side_b, &c.side_a) };
        (c.involves(joiner), c.theatre, mine.clone(), theirs.clone())
    };
    if already {
        return Err("You are already a party to that conflict.".into());
    }
    let stake = if theatre::is_home(w, joiner, th) { 1.0 } else { 0.45 };
    {
        let c = w.conflict_mut(conflict).expect("checked");
        crate::war::join_side(c, joiner, side_a, 1, objective);
        if let Some(b) = c.posture_mut(joiner) {
            b.stake = stake;
        }
    }
    // Taking a side is read as taking a side. Everyone on it warms to you and
    // everyone across from it does not, before a shot is fired or a rung climbed.
    for f in friends {
        w.shift_relation(joiner, f, 8.0);
    }
    for e in foes {
        w.shift_relation(joiner, e, -25.0);
    }
    let against = foes_name(w, conflict, joiner);
    w.headline(format!(
        "{} takes a side against {} over {}.",
        joiner.name(),
        against,
        th.name()
    ));
    // An expeditionary power that has just committed itself goes round the
    // neighbours the same month, because everything above rung 5 depends on it.
    if !theatre::is_home(w, joiner, th) {
        seek_access(w, joiner, th);
    }
    Ok(())
}

fn foes_name(w: &WorldState, conflict: u32, joiner: NationId) -> String {
    w.conflict(conflict)
        .and_then(|c| primary_opponent(c, joiner))
        .map_or_else(|| "them".to_string(), |o| o.name().to_string())
}

// ---------------------------------------------------------------------------
// Access: the diplomatic quantity that is a direct military input
// ---------------------------------------------------------------------------

/// Ask a third state for basing and overflight. Their parliament answers once.
/// A refusal is public, and it leaves the rung cap where it was.
pub fn request_access(
    w: &mut WorldState,
    seeker: NationId,
    host: NationId,
    th: TheatreId,
    pressed: bool,
) -> Result<(), String> {
    if seeker == host {
        return Err("You do not need your own permission.".into());
    }
    if w.nation_opt(host).is_none_or(|n| !n.alive) {
        return Err("Nation no longer exists.".into());
    }
    if !theatre::theatre(w, th).access_hosts.contains(&host) {
        return Err(format!("{} has nothing within range of {}.", host.name(), th.name()));
    }
    if theatre::already_granted(w, host, seeker, th) {
        return Err("They have already agreed.".into());
    }
    let target = w
        .conflicts
        .iter()
        .find(|c| c.involves(seeker) && c.theatre == th)
        .and_then(|c| primary_opponent(c, seeker));
    let unrestricted = w
        .conflicts
        .iter()
        .find(|c| c.involves(seeker) && c.theatre == th)
        .and_then(|c| c.posture_of(seeker))
        .is_some_and(|b| b.roe == Roe::Unrestricted);
    let mut p = theatre::consent_probability(w, host, seeker, target, unrestricted);
    if pressed {
        // Leverage is the whole point of having built dependency, and this is
        // where it is spent. What Washington did to Ankara in March 2003.
        p = (p * (1.0 + 1.2 * w.trade_dependency(host, seeker)) + 0.12).clamp(0.0, 0.95);
    }
    if w.rng.chance(p) {
        theatre::grant(w, host, seeker, th);
        w.shift_relation(host, seeker, 4.0);
        w.headline(format!(
            "{} grants {} basing and overflight for {}.",
            host.name(),
            seeker.name(),
            th.name()
        ));
    } else {
        w.shift_reputation(seeker, -4.0);
        if pressed {
            w.shift_relation(host, seeker, -12.0);
            w.headline(format!(
                "{}'s parliament refuses {} a second time. Pressure has bought nothing but resentment.",
                host.name(),
                seeker.name()
            ));
        } else {
            w.headline(format!(
                "{}'s parliament refuses {} the use of its bases.",
                host.name(),
                seeker.name()
            ));
        }
    }
    Ok(())
}

/// The other end of the same table: the host's own click. This is where a small
/// state discovers it has agency in a great power's war.
pub fn grant_access(
    w: &mut WorldState,
    host: NationId,
    seeker: NationId,
    th: TheatreId,
    grant: bool,
) -> Result<(), String> {
    if host == seeker {
        return Err("You do not need your own permission.".into());
    }
    if !theatre::theatre(w, th).access_hosts.contains(&host) {
        return Err(format!("{} has nothing within range of {}.", host.name(), th.name()));
    }
    if grant {
        if theatre::already_granted(w, host, seeker, th) {
            return Err("Already granted.".into());
        }
        theatre::grant(w, host, seeker, th);
        w.shift_relation(host, seeker, 6.0);
        w.headline(format!(
            "{} opens its bases to {} for {}.",
            host.name(),
            seeker.name(),
            th.name()
        ));
    } else {
        w.shift_relation(host, seeker, -6.0);
        w.headline(format!(
            "{} declines {} the use of its territory.",
            host.name(),
            seeker.name()
        ));
    }
    Ok(())
}

/// A parliament can always vote, including on the way out. A model that stopped
/// a discredited government from betraying a superpower mid-campaign would have
/// it exactly backwards.
pub fn revoke_access(
    w: &mut WorldState,
    host: NationId,
    seeker: NationId,
    th: TheatreId,
) -> Result<(), String> {
    if !theatre::revoke(w, host, seeker, th) {
        return Err("No such agreement.".into());
    }
    w.shift_relation(host, seeker, -20.0);
    w.shift_reputation(host, -15.0);
    w.headline(format!(
        "{} revokes {}'s basing rights in the middle of a campaign.",
        host.name(),
        seeker.name()
    ));
    // Anyone left standing above the access line has to come down.
    let forced: Vec<(u32, NationId, u8)> = w
        .conflicts
        .iter()
        .filter(|c| c.theatre == th && c.involves(seeker))
        .filter(|_| !theatre::has_access(w, seeker, th))
        .filter_map(|c| {
            c.posture_of(seeker)
                .filter(|b| b.rung > MAX_RUNG_WITHOUT_ACCESS)
                .map(|_| (c.id, seeker, MAX_RUNG_WITHOUT_ACCESS))
        })
        .collect();
    for (cid, who, rung) in forced {
        w.daily.counters.remove(&format!("war:{cid}:rung:{who:?}"));
        if let Some(c) = w.conflict_mut(cid) {
            if let Some(b) = c.posture_mut(who) {
                b.rung = rung;
                b.months_at_rung = 0;
            }
        }
        w.headline(format!(
            "{} cannot sustain a campaign it has no base for and falls back to rung {}.",
            who.name(),
            rung
        ));
    }
    Ok(())
}

/// What an expeditionary power does the month it finds itself in somebody
/// else's war: goes round the neighbours asking for an airfield. Routed through
/// `apply_command` so the AI pays for it on the player's terms.
pub fn seek_access(w: &mut WorldState, seeker: NationId, th: TheatreId) {
    if theatre::has_access(w, seeker, th) {
        return;
    }
    for host in theatre::unasked_hosts(w, seeker, th) {
        let _ = apply_command(w, &Command::RequestAccess { seeker, host, theatre: th });
        if theatre::has_access(w, seeker, th) {
            return;
        }
    }
}

// ---------------------------------------------------------------------------
// The one AI rule this phase ships
// ---------------------------------------------------------------------------

/// What this belligerent could actually put into this theatre if it stood in the
/// open: structure, times the share of itself it can get there, times what its
/// soldiers can do when they arrive. Not committed force — that depends on the
/// rung, and this is the thing being used to *choose* the rung.
fn fieldable(w: &WorldState, c: &Conflict, id: NationId) -> f64 {
    let structure = match w.nation_opt(id) {
        Some(n) if n.alive => n.mil_strength,
        _ => return 0.0,
    };
    let proj = if theatre::is_home(w, id, c.theatre) {
        1.0
    } else {
        crate::war::deployable_fraction(w, id)
    };
    structure * proj * crate::war::quality(w, id)
}

/// The rung this belligerent is trying to reach — the top of its climb, not its
/// next step.
///
/// Three terms and no country names: what it says it wants, what the other side
/// could meet it with, and what it has publicly bound itself to. A state that
/// cannot win in the open does not go into the open; it stops at the rungs where
/// somebody else's people do the dying, which is why most quarrels in the world
/// live between 2 and 5 and only a few climb past 6.
pub fn ambition(w: &WorldState, c: &Conflict, b: &Belligerent) -> u8 {
    let side = match c.side_of(b.nation) {
        Some(s) => s,
        None => return 1,
    };
    // A quarrel nobody has prosecuted in eighteen months is not an ambition any
    // more, it is a standing expense. Without this the wind-down and the climb
    // fight each other every month and a frozen conflict oscillates on one rung
    // for the rest of the century.
    if c.frozen_since.is_some() {
        return 1;
    }
    // What the other side is offering. The defensive objectives are answers
    // rather than intentions — they mirror it, and no more than mirror it, which
    // is what keeps a quarrel at rhetoric until somebody decides otherwise and
    // makes the ladder a dialogue instead of two independent slides.
    let opp = c
        .posture
        .iter()
        .filter(|x| c.side_of(x.nation) != Some(side))
        .map(|x| x.rung)
        .max()
        .unwrap_or(1);
    // ...and what an answer answers is SHOOTING. A state does not have to run a
    // deniable service because somebody else is running one at it; it can simply
    // endure the thing and let the other government keep paying for it. Mirror
    // the bottom of the ladder as well and every quarrel in the world ratchets:
    // two sides holding each other at rung 5 with neither able to stop, which
    // measured at three quarters of every belligerent-month on the board.
    let answer = if opp >= SHOOTING_RUNG { opp } else { 1 };
    let want = match b.objective {
        Objective::Seize => INVASION_RUNG,
        Objective::Stabilise => 9,
        Objective::Degrade => SHOOTING_RUNG,
        Objective::Deny | Objective::Hold => answer,
        Objective::Withdraw => 1,
    };
    let mine = fieldable(w, c, b.nation);
    let theirs: f64 = c
        .participants()
        .iter()
        .filter(|o| c.side_of(**o) != Some(side))
        .map(|o| fieldable(w, c, *o))
        .sum();
    // A state on its own ground answers whatever is offered it, and does not
    // consult the balance of forces first — that is what home ground means.
    if defending_home(w, c, b.nation) {
        return want.max(answer).min(b.ceiling);
    }
    let ratio = mine / theirs.max(0.001);
    // How much of the ladder the balance of forces lets it contemplate. The
    // bottom of this table is the whole reason the bottom of the ladder gets
    // used: a state that cannot meet the other army in the field arms somebody
    // who can, or shouts, and those are rungs 2 to 4.
    let dare = if ratio >= 1.6 {
        9
    } else if ratio >= 1.0 {
        INVASION_RUNG
    } else if ratio >= 0.7 {
        SHOOTING_RUNG
    } else if ratio >= 0.4 {
        4
    } else {
        2
    };
    want.min(dare).min(b.ceiling)
}

/// Deliberately minimal, and deliberately routed through `apply_command`, so
/// that the AI buys its escalation with the same currency at the same price the
/// player does.
///
/// This is where the climb happens, and it is the answer to QA's first finding.
/// Before, the only reason to escalate was to be losing ground you were already
/// fighting for — which cannot happen at rung 1, where nobody is fighting — so
/// no conflict ever went up and the ladder ran in one direction. Now a state
/// prosecuting a quarrel walks up it a rung at a time, paying at every step,
/// stopping when it runs out of standing, of will, or of nerve about what is on
/// the other side.
/// How fast a government walks up a ladder it has decided to walk up, before
/// the two things that actually pace it: how far it still has to go, and
/// whether it can pay for the next step. A state that means to invade does not
/// spend a year on rhetoric first — it moves quickly through the rungs that are
/// beneath its intention and slowly through the ones near it.
pub const CLIMB_CHANCE: f64 = 0.45;
/// How hard the AI leans toward the next rung each month once it has decided
/// it wants to be higher up the ladder than it is.
pub const CLIMB_URGENCY: f64 = 0.12;
/// A government that changes its mind monthly has none. Two months on a rung
/// before the next step.
pub const RUNG_DWELL: u32 = 2;

/// Move every AI belligerent one considered step along the commitment ladder.
///
/// Each government picks its own rung in each conflict it is party to, pays for
/// the climb, and holds the new rung long enough to mean it — see
/// [`CLIMB_URGENCY`] and the two-month dwell beside it.
pub fn ai_ladder(w: &mut WorldState) {
    #[derive(Clone, Copy)]
    struct Move {
        conflict: u32,
        nation: NationId,
        rung: u8,
        chance: f64,
    }
    let mut moves: Vec<Move> = vec![];
    for c in &w.conflicts {
        for b in &c.posture {
            if Some(b.nation) == w.player {
                continue;
            }
            if b.objective == Objective::Withdraw {
                continue; // the withdrawal schedule owns this belligerent
            }
            if b.months_at_rung < RUNG_DWELL {
                continue;
            }
            let side = match c.side_of(b.nation) {
                Some(s) => s,
                None => continue,
            };
            let against = if side { -c.control } else { c.control };
            let (exhaustion, stability) = match w.nation_opt(b.nation) {
                Some(n) => (n.war_exhaustion, n.stability),
                None => continue,
            };
            let want = ambition(w, c, b);
            if b.resolve < 0.20 && b.rung > 1 {
                moves.push(Move { conflict: c.id, nation: b.nation, rung: b.rung - 1, chance: 1.0 });
            } else if b.rung > want && b.rung > 1 {
                // What it is prepared to hold has fallen below what it is
                // standing on — because the balance moved, or because it has
                // said out loud that it is getting out.
                moves.push(Move { conflict: c.id, nation: b.nation, rung: b.rung - 1, chance: 0.5 });
            } else if b.rung < want {
                // The climb. Slower for a government that is exhausted or
                // unsteady at home, and never instant: the ladder is a sequence
                // of decisions taken over years, which is the point of it.
                let steady = (1.0 - exhaustion) * (0.35 + 0.65 * (stability / 100.0).clamp(0.0, 1.0));
                let urgency = CLIMB_CHANCE + CLIMB_URGENCY * (want - b.rung) as f64;
                let chance = (urgency * steady * (0.6 + 0.4 * b.resolve)).min(0.9);
                moves.push(Move { conflict: c.id, nation: b.nation, rung: b.rung + 1, chance });
            } else if against > 0.10 && b.resolve > 0.50 && b.rung < b.ceiling.min(9) {
                // Losing the ground it is already fighting for: the one reason
                // to go past what it set out to do.
                moves.push(Move { conflict: c.id, nation: b.nation, rung: b.rung + 1, chance: 0.5 });
            } else if c.frozen_since.is_some() && b.months_at_rung >= 12 && b.rung > 1 {
                // A frozen quarrel is one nobody is prosecuting, and nobody keeps
                // paying for advisers and an embargo forever over a border that
                // has not moved in a year. This is what lets a conflict wind
                // down to rhetoric and finally fall off the board — without it
                // the vector only ever grows.
                moves.push(Move { conflict: c.id, nation: b.nation, rung: b.rung - 1, chance: 1.0 });
            }
        }
    }
    // Rolled in a second pass, in the order the conflicts and their postures sit
    // in the vectors, because the first pass is holding the world immutably and
    // the RNG is the world's.
    for m in moves {
        if m.chance < 1.0 && !w.rng.chance(crate::clock::chance(w, m.chance)) {
            continue;
        }
        let cmd = Command::SetCommitment { conflict: m.conflict, nation: m.nation, rung: m.rung };
        let _ = apply_command(w, &cmd);
    }
}
