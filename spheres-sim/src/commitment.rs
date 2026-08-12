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
pub fn escalation_cost(w: &WorldState, id: NationId, from: u8, to: u8) -> f64 {
    if to <= from {
        return 0.0;
    }
    let lo = ESCALATION_PRICE[(from as usize).min(9)];
    let hi = ESCALATION_PRICE[(to as usize).min(9)];
    let auth = w.nation_opt(id).map_or(0.5, |n| n.authoritarianism);
    (hi - lo) * (1.4 - 0.6 * auth)
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
    if rung >= MAX_RUNG_WITHOUT_ACCESS + 1 && !theatre::has_access(w, id, c.theatre) {
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
        let mine = w.nation_opt(id).map_or(false, |n| n.nuclear);
        let side = c.side_of(id);
        let armed_enemy = c.participants().iter().any(|o| {
            c.side_of(*o) != side && w.nation_opt(*o).map_or(false, |n| n.alive && n.nuclear)
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
    {
        let c = w.conflict_mut(conflict).expect("checked");
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
    if w.month % 3 != 1 {
        return;
    }
    let jobs: Vec<(NationId, NationId)> = w
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
    for (sponsor, target) in jobs {
        if Some(sponsor) == w.player {
            continue; // the player's own services do what the player tells them
        }
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
        .filter(|x| w.nation_opt(*x).map_or(false, |n| n.alive))
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
    if w.nation_opt(opener).map_or(true, |n| !n.alive)
        || w.nation_opt(target).map_or(true, |n| !n.alive)
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
    if w.nation_opt(host).map_or(true, |n| !n.alive) {
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
        .map_or(false, |b| b.roe == Roe::Unrestricted);
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

/// Deliberately minimal, and deliberately routed through `apply_command`, so
/// that the AI buys its escalation with the same currency at the same price the
/// player does. Climb when you are losing the ground and still have the will;
/// step back when you have not. Anything richer is a later branch — see
/// ROADMAP.
pub fn ai_ladder(w: &mut WorldState) {
    #[derive(Clone, Copy)]
    struct Move {
        conflict: u32,
        nation: NationId,
        rung: u8,
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
            if b.months_at_rung < 2 {
                continue; // a government that changes its mind monthly has none
            }
            let side = match c.side_of(b.nation) {
                Some(s) => s,
                None => continue,
            };
            let against = if side { -c.control } else { c.control };
            if b.resolve < 0.20 && b.rung > 1 {
                moves.push(Move { conflict: c.id, nation: b.nation, rung: b.rung - 1 });
            } else if against > 0.10 && b.resolve > 0.50 && b.rung < b.ceiling.min(9) {
                moves.push(Move { conflict: c.id, nation: b.nation, rung: b.rung + 1 });
            } else if c.frozen_since.is_some() && b.months_at_rung >= 12 && b.rung > 1 {
                // A frozen quarrel is one nobody is prosecuting, and nobody keeps
                // paying for advisers and an embargo forever over a border that
                // has not moved in a year. This is what lets a conflict wind
                // down to rhetoric and finally fall off the board — without it
                // the vector only ever grows.
                moves.push(Move { conflict: c.id, nation: b.nation, rung: b.rung - 1 });
            }
        }
    }
    for m in moves {
        let cmd = Command::SetCommitment { conflict: m.conflict, nation: m.nation, rung: m.rung };
        let _ = apply_command(w, &cmd);
    }
}
