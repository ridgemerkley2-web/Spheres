//! Statecraft — everything a power does to another power short of shooting at it.
//!
//! Four instruments, all of them commands like any other, all of them available
//! to the AI on exactly the terms the player gets:
//!
//! * **Defence pacts** turn a vague affinity into a written commitment that
//!   drags you into other people's wars — and that you can break, at a price.
//! * **Aid and arms** buy a government's survival, and with it its alignment.
//! * **Covert action** breaks a rival's client without a declaration, until the
//!   day it is caught and the whole thing rebounds.
//! * **Trade agreements** make two economies grow together and leave the smaller
//!   one holding a knife by the blade.

use crate::world::*;

/// What full integration with a market of overwhelming size is worth as a
/// permanent uplift to output. Paid once, as integration deepens, rather than
/// every month forever. A quarter is a large number and it is meant to be: it
/// is the gain to a small open economy that gets access to a market a hundred
/// times its size, and it is scaled down by relative size for everyone else, so
/// the giant on the other side of the same agreement collects a fraction of a
/// percent.
const TRADE_LEVEL_GAIN: f64 = 0.25;

/// Annual share of GDP a guarantee costs both signatories: garrisons, exercises,
/// and forces sized for somebody else's border.
const PACT_UPKEEP: f64 = 0.003;

/// The most output any patron can promise to clients. The Soviet Union, the
/// most profligate patron of the era, disbursed roughly $6.9bn to the Third
/// World in 1985 against a GNP near $1.5tn — under half a percent.
/// https://en.wikipedia.org/wiki/Economy_of_the_Soviet_Union
pub const MAX_AID_SHARE: f64 = 0.010;

/// ...and no single client can absorb more than this share of it. A sphere of
/// influence is several countries or it is not a sphere.
pub const MAX_CLIENT_SHARE: f64 = 0.004;

/// A patron's transfer is small at home and enormous at the other end: Soviet
/// subsidies to Cuba averaged $4.3bn a year over 1986-90 and came to 21% of
/// Cuban GNP. That asymmetry is the whole mechanism, and this is where it stops.
/// https://en.wikipedia.org/wiki/Cuba%E2%80%93Soviet_Union_relations
const MAX_INFUSION: f64 = 0.25;

/// Monthly upkeep of the standing arrangements. Nothing here rolls dice: a
/// treaty in force is not a gamble, it is a bill.
pub fn tick(w: &mut WorldState) {
    reputations_recover(w);
    pacts_upkeep(w);
    aid_flows(w);
    trade_deepens(w);
    covert_channels_cool(w);
    // A seller's refusal is remembered, then gradually not (resources.rs,
    // spec section 6.3). Free while the memory is empty, which is every
    // world the market switch is off in.
    crate::resources::cool_refusals(w);
}

/// A broken promise is remembered, then gradually not. Nobody climbs back above
/// the baseline by waiting; that has to be earned in a war.
fn reputations_recover(w: &mut WorldState) {
    for (_, v) in w.statecraft.reputation.iter_mut() {
        if *v < BASE_REPUTATION {
            *v = (*v + 0.15).min(BASE_REPUTATION);
        }
    }
}

fn pacts_upkeep(w: &mut WorldState) {
    let pairs: Vec<(NationId, NationId)> =
        w.statecraft.pacts.iter().map(|p| (p.a, p.b)).collect();
    let mut lapsed: Vec<(NationId, NationId)> = vec![];
    for (a, b) in pairs {
        let both_alive = w.nation_opt(a).is_some_and(|n| n.alive)
            && w.nation_opt(b).is_some_and(|n| n.alive);
        if !both_alive {
            lapsed.push((a, b));
            continue;
        }
        for id in [a, b] {
            w.nation_mut(id).debt_gdp += PACT_UPKEEP / 12.0;
        }
        if w.relation(a, b) < 85.0 {
            w.shift_relation(a, b, 0.20);
        }
        // A guarantee between states that have come to loathe each other is a
        // dead letter long before anyone bothers to renounce it.
        if w.relation(a, b) < -25.0 {
            lapsed.push((a, b));
            w.headline(format!(
                "The defence pact between {} and {} lapses, unmourned.",
                a.name(),
                b.name()
            ));
        }
    }
    w.statecraft.pacts.retain(|p| !lapsed.contains(&(p.a, p.b)));
}

fn aid_flows(w: &mut WorldState) {
    let flows: Vec<AidFlow> = w.statecraft.aid.clone();
    let mut dead: Vec<(NationId, NationId, AidKind)> = vec![];
    for f in flows {
        let patron_ok = w.nation_opt(f.patron).is_some_and(|n| n.alive);
        let client_ok = w.nation_opt(f.client).is_some_and(|n| n.alive);
        if !patron_ok || !client_ok {
            dead.push((f.patron, f.client, f.kind));
            continue;
        }
        let annual = w.nation(f.patron).gdp * f.share_gdp; // $bn/yr
        // Weapons leave on credit that is rarely repaid, so they cost the
        // treasury about half what a cash transfer does.
        let fiscal = match f.kind {
            AidKind::Economic => f.share_gdp,
            AidKind::Arms => f.share_gdp * 0.5,
        };
        w.nation_mut(f.patron).debt_gdp += fiscal / 12.0;

        let (client_gdp, client_mil_share, client_strength) = {
            let c = w.nation(f.client);
            (c.gdp, c.mil_spend_gdp, c.mil_strength)
        };
        let infusion = (annual / client_gdp.max(0.1)).min(MAX_INFUSION);
        match f.kind {
            AidKind::Economic => {
                let c = w.nation_mut(f.client);
                // Most of a subsidy is eaten, not invested; what it reliably
                // buys is a government that can pay its soldiers this month.
                c.gdp *= 1.0 + infusion * 0.15 / 12.0;
                c.stability = (c.stability + infusion * 8.0 / 12.0).min(100.0);
            }
            AidKind::Arms => {
                // Transferred weapons are budget the client never had to raise.
                // Same curve war.rs uses for domestic spending, so a client's
                // army converges on what its patron is willing to fund.
                let sustained = ((client_gdp * client_mil_share + annual) * 0.30).sqrt() * 8.0;
                let c = w.nation_mut(f.client);
                c.mil_strength += (sustained - client_strength) * 0.02;
            }
        }
        // Dependence reads as friendship right up until the money stops.
        if w.relation(f.patron, f.client) < 75.0 {
            w.shift_relation(f.patron, f.client, 0.25);
        }
    }
    w.statecraft
        .aid
        .retain(|f| !dead.contains(&(f.patron, f.client, f.kind)));
}

fn trade_deepens(w: &mut WorldState) {
    let pairs: Vec<(NationId, NationId)> = w.statecraft.trade.iter().map(|t| (t.a, t.b)).collect();
    let mut dead: Vec<(NationId, NationId)> = vec![];
    for (a, b) in pairs {
        let both_alive = w.nation_opt(a).is_some_and(|n| n.alive)
            && w.nation_opt(b).is_some_and(|n| n.alive);
        if !both_alive {
            dead.push((a, b));
            continue;
        }
        let broken = belligerents(w, a, b) || w.is_sanctioning(a, b) || w.is_sanctioning(b, a);
        if broken {
            dead.push((a, b));
            w.headline(format!(
                "Trade between {} and {} collapses; the agreement is a dead letter.",
                a.name(),
                b.name()
            ));
            continue;
        }
        // Integration deepens. THE LEVEL GAIN IS NO LONGER PAID HERE: it used to
        // be paid per pact, per month, on this increment — see `trade_level_gain`
        // below for why summing that over a roster is a growth rate wearing a
        // level's clothes, and what replaces it.
        {
            let t = w
                .statecraft
                .trade
                .iter_mut()
                .find(|t| t.a == a && t.b == b)
                .expect("trade pact");
            t.depth = (t.depth + 0.012).min(1.0);
        }
        if w.relation(a, b) < 70.0 {
            w.shift_relation(a, b, 0.15);
        }
    }
    w.statecraft.trade.retain(|t| !dead.contains(&(t.a, t.b)));
    trade_level_gain(w);
}

/// The market a nation's agreements have opened to it, as ONE number for the
/// whole portfolio: depth-weighted access over own output plus the full size of
/// every partner.
///
/// The per-pact form this replaces summed `theirs / (mine + theirs)` over every
/// agreement. That quantity is already a *share* of a nation's trading
/// universe, so N of them add past one and keep going — the United States held
/// 41 agreements on the ten-seed average by 2024 for a summed reach well past
/// two, a permanent uplift still climbing every four years, and measured at
/// 1.17 points of annual growth for thirty-five years. An unbounded stream of
/// level shifts is a rate, which is exactly the error BIBLE §8 records finding
/// in this very function and fixing only per-agreement: `TRADE_LEVEL_GAIN`
/// bounds *one* agreement, and nothing bounded the roster.
///
/// Aggregated, reach is bounded by 1 however many agreements are held, so the
/// whole portfolio is bounded by TRADE_LEVEL_GAIN — the claim the constant's
/// own comment already makes and could not keep. WITH A SINGLE PACT THE TWO
/// FORMS ARE IDENTICAL: `depth * theirs / (mine + theirs)`. The small partner's
/// transformation is untouched; only the giant collecting sixty fractions of a
/// percent is.
fn trade_reach(w: &WorldState, id: NationId) -> f64 {
    let mine = w.nation(id).gdp;
    let (mut access, mut potential) = (0.0, 0.0);
    for t in w.statecraft.trade.iter() {
        let other = if t.a == id {
            t.b
        } else if t.b == id {
            t.a
        } else {
            continue;
        };
        if let Some(o) = w.nation_opt(other).filter(|n| n.alive) {
            access += t.depth * o.gdp;
            potential += o.gdp;
        }
    }
    if potential <= 0.0 {
        return 0.0;
    }
    access / (mine + potential).max(1.0)
}

/// Paid on the RISE in entitlement and never clawed back. Two reasons, both
/// structural rather than convenient. Reach moves when a partner's economy
/// moves relative to yours, and paying on that would hand a nation growth for
/// its partners' growth — the same rate-for-a-level bug through the back door.
/// And a pact that collapses and is re-signed re-enters at depth 0.05, so
/// without a high-water mark sign/collapse/re-sign is an unbounded GDP pump
/// under player action. What losing an agreement costs is priced where it
/// belongs, in `abrogate_trade` and the dependency system; giving the level
/// back here would charge it twice.
fn trade_level_gain(w: &mut WorldState) {
    let mut ids: Vec<NationId> = vec![];
    for t in w.statecraft.trade.iter() {
        for id in [t.a, t.b] {
            if !ids.contains(&id) {
                ids.push(id);
            }
        }
    }
    ids.sort_by_key(|i| i.index()); // one fixed order; no map iteration
    for id in ids {
        if !w.nation_opt(id).is_some_and(|n| n.alive) {
            continue;
        }
        let owed = TRADE_LEVEL_GAIN * trade_reach(w, id);
        let n = w.nation_mut(id);
        match n.trade_level_paid {
            None => n.trade_level_paid = Some(owed), // a loaded save is already paid
            Some(paid) if owed > paid => {
                n.gdp *= 1.0 + (owed - paid);
                n.trade_level_paid = Some(owed);
            }
            _ => {}
        }
    }
}

fn covert_channels_cool(w: &mut WorldState) {
    for (_, _, h) in w.statecraft.covert_heat.iter_mut() {
        *h -= 0.012;
    }
    w.statecraft.covert_heat.retain(|(_, _, h)| *h > 0.0);
}

// ---------------------------------------------------------------------------
// Commands
// ---------------------------------------------------------------------------

/// An offer, not a fact: the other government has to want it. What it weighs is
/// how it feels about you, what your word has been worth to others, and whether
/// signing would put it on the wrong side of a power it cannot afford to annoy.
pub fn propose_pact(w: &mut WorldState, from: NationId, to: NationId) -> Result<(), String> {
    if from == to {
        return Err("A nation cannot guarantee itself.".into());
    }
    if !alive(w, from) || !alive(w, to) {
        return Err("Nation no longer exists.".into());
    }
    if w.allied(from, to) {
        return Err("Already allied.".into());
    }
    if belligerents(w, from, to) {
        return Err("Cannot ally with a nation you are fighting.".into());
    }
    // Blocs are exclusive, and that exclusivity is what makes them blocs. Nobody
    // underwrites a state a rival has already underwritten, because the pact
    // would oblige the two guarantors to fight each other for it.
    for g in w.pact_partners(to) {
        if w.relation(from, g) < -25.0 {
            return Err("A rival has already guaranteed them.".into());
        }
    }
    for g in w.pact_partners(from) {
        if w.relation(to, g) < -25.0 {
            return Err("They will not sign alongside our enemies.".into());
        }
    }
    // A guarantee everyone has given is a guarantee nobody has given.
    if w.pact_partners(to).len() >= 3 || w.pact_partners(from).len() >= 3 {
        return Err("Too many standing commitments already.".into());
    }

    let rel = w.relation(from, to);
    let mut p = ((rel + 10.0) / 140.0).clamp(0.0, 0.85);
    p *= (w.reputation(from) / BASE_REPUTATION).clamp(0.35, 1.20);
    // A guarantee from someone your patron hates is a liability, not a shield.
    for q in w.patrons_of(to) {
        if q != from && w.relation(from, q) < -25.0 {
            p *= 0.4;
        }
    }
    // The more lopsided the pair, the easier the signature: the strong side is
    // granting a favour it does not expect to have to pay for.
    let (fs, ts) = (w.nation(from).mil_strength, w.nation(to).mil_strength);
    let lopsided = (fs.max(ts) / fs.min(ts).max(1.0)).min(6.0);
    p *= 1.0 + (lopsided - 1.0) * 0.06;

    if w.rng.chance(p.min(0.9)) {
        let (a, b) = if from <= to { (from, to) } else { (to, from) };
        w.statecraft.pacts.push(Pact {
            a,
            b,
            since_year: w.year,
            since_month: w.month,
        });
        w.shift_relation(from, to, 8.0);
        w.headline(format!(
            "{} and {} sign a mutual defence pact.",
            from.name(),
            to.name()
        ));
    } else {
        w.headline(format!(
            "{} declines {}'s offer of a defence pact.",
            to.name(),
            from.name()
        ));
    }
    Ok(())
}

/// Renouncing a guarantee in peacetime is merely embarrassing. Renouncing one
/// while the other party is being invaded is the thing everyone remembers.
pub fn break_pact(w: &mut WorldState, from: NationId, to: NationId) -> Result<(), String> {
    if !w.allied(from, to) {
        return Err("No pact to break.".into());
    }
    dissolve_pact(w, from, to);
    if w.at_war(to) {
        w.shift_reputation(from, -30.0);
        w.shift_relation(from, to, -50.0);
        w.headline(format!(
            "{} renounces its guarantee to {} with {} under attack. The word of {} is worth less everywhere.",
            from.name(), to.name(), to.name(), from.name()
        ));
        cheapened_guarantees(w, from, to);
    } else {
        w.shift_reputation(from, -10.0);
        w.shift_relation(from, to, -20.0);
        w.headline(format!(
            "{} withdraws from its defence pact with {}.",
            from.name(),
            to.name()
        ));
    }
    Ok(())
}

/// Open a standing transfer from `patron` to `client`, of `share_gdp` of the
/// patron's output, in the currency named by `kind`.
///
/// A pledge is an ongoing obligation rather than a one-off gift: the patron
/// keeps paying it every month until somebody withdraws it, which is what makes
/// a sphere cost something to hold. Errors if the two are the same nation or
/// the terms are outside what a patron can actually sustain.
pub fn pledge_aid(
    w: &mut WorldState,
    patron: NationId,
    client: NationId,
    kind: AidKind,
    share_gdp: f64,
) -> Result<(), String> {
    if patron == client {
        return Err("A nation cannot be its own client.".into());
    }
    if !alive(w, patron) || !alive(w, client) {
        return Err("Nation no longer exists.".into());
    }
    if belligerents(w, patron, client) {
        return Err("Cannot bankroll a nation you are fighting.".into());
    }
    let share = share_gdp.clamp(0.0, MAX_AID_SHARE);
    if share <= 0.0 {
        return end_aid(w, patron, client, kind);
    }
    let this_flow = w.aid_flow(patron, client, kind).map(|f| f.share_gdp).unwrap_or(0.0);
    if w.aid_share_committed(patron) - this_flow + share > MAX_AID_SHARE + 1e-9 {
        return Err("The budget will not stretch to another client.".into());
    }
    if w.aid_share_to(patron, client) - this_flow + share > MAX_CLIENT_SHARE + 1e-9 {
        return Err("One client cannot absorb any more.".into());
    }

    let annual = w.nation(patron).gdp * share;
    let existing = w.aid_flow(patron, client, kind).map(|f| f.share_gdp);
    match w.statecraft.aid.iter_mut().find(|f| {
        f.patron == patron && f.client == client && f.kind == kind
    }) {
        Some(f) => f.share_gdp = share,
        None => w.statecraft.aid.push(AidFlow {
            patron,
            client,
            kind,
            share_gdp: share,
            since_year: w.year,
        }),
    }
    w.shift_relation(patron, client, 4.0);
    w.headline(match (kind, existing) {
        (AidKind::Economic, None) => format!(
            "{} commits ${:.0}bn a year in economic aid to {}.",
            patron.name(), annual, client.name()
        ),
        (AidKind::Economic, Some(_)) => format!(
            "{} raises its aid to {} to ${:.0}bn a year.",
            patron.name(), client.name(), annual
        ),
        (AidKind::Arms, None) => format!(
            "{} approves ${:.0}bn a year in arms sales to {}.",
            patron.name(), annual, client.name()
        ),
        (AidKind::Arms, Some(_)) => format!(
            "{} expands arms transfers to {} to ${:.0}bn a year.",
            patron.name(), client.name(), annual
        ),
    });
    // Bankrolling a country's enemy is noticed by the country.
    let offended: Vec<NationId> = w
        .nations
        .iter()
        .filter(|n| n.alive && n.id != patron && n.id != client)
        .map(|n| n.id)
        .filter(|x| w.relation(client, *x) < -30.0)
        .collect();
    for x in offended {
        w.shift_relation(patron, x, -4.0);
    }
    Ok(())
}

/// Cutting a client loose is cheap for the patron and catastrophic for the
/// client, which is exactly why the threat works while the money still flows.
pub fn end_aid(
    w: &mut WorldState,
    patron: NationId,
    client: NationId,
    kind: AidKind,
) -> Result<(), String> {
    if w.aid_flow(patron, client, kind).is_none() {
        return Err("No such aid programme.".into());
    }
    w.statecraft
        .aid
        .retain(|f| !(f.patron == patron && f.client == client && f.kind == kind));
    w.shift_relation(patron, client, -12.0);
    if alive(w, client) {
        let c = w.nation_mut(client);
        c.stability = (c.stability - 3.0).max(0.0);
    }
    w.headline(format!(
        "{} cuts off {} to {}.",
        patron.name(),
        kind.label(),
        client.name()
    ));
    Ok(())
}

/// Deniable, probabilistic, and worse than useless once it is caught. Two rolls
/// in a fixed order: whether the operation worked, and whether it stayed secret.
pub fn covert_action(
    w: &mut WorldState,
    sponsor: NationId,
    target: NationId,
    op: CovertOp,
) -> Result<(), String> {
    if sponsor == target {
        return Err("A service does not run operations against its own state.".into());
    }
    if !alive(w, sponsor) || !alive(w, target) {
        return Err("Nation no longer exists.".into());
    }

    // Running a service costs money whether or not anything comes of it.
    w.nation_mut(sponsor).debt_gdp += 0.0008;

    let (s_gdp, t_gdp) = (w.nation(sponsor).gdp, w.nation(target).gdp);
    let (t_stab, t_sep, t_auth) = {
        let t = w.nation(target);
        (t.stability, t.separatism, t.authoritarianism)
    };
    let heat = w.covert_heat(sponsor, target);
    // You cannot subvert a country that has nothing wrong with it, and reach
    // still matters: a large sponsor can buy more of the local politics.
    let reach = (s_gdp / (s_gdp + t_gdp).max(1.0)).clamp(0.0, 1.0);
    let success_p = (0.12
        + (60.0 - t_stab).max(0.0) / 100.0 * 0.55
        + t_sep * 0.30
        + reach * 0.20)
        .clamp(0.05, 0.80);
    // A police state catches spies; a well-worn channel gets rolled up.
    let expose_p = (0.10 + heat * 0.55 + t_auth * 0.15).clamp(0.05, 0.85);

    let worked = w.rng.chance(success_p);
    let exposed = w.rng.chance(expose_p);
    w.add_covert_heat(sponsor, target, 0.18);

    if worked {
        match op {
            CovertOp::FundOpposition => {
                let hit = w.rng.range(5.0, 10.0);
                let t = w.nation_mut(target);
                t.stability = (t.stability - hit).max(0.0);
                w.headline(format!(
                    "Strikes and street protest paralyse {}. Nobody can prove whose money is behind it.",
                    target.name()
                ));
            }
            CovertOp::StirSeparatists => {
                let hit = w.rng.range(0.05, 0.11);
                let t = w.nation_mut(target);
                t.separatism = (t.separatism + hit).min(1.0);
                w.headline(format!(
                    "Separatist fighters in {} turn up with weapons nobody will account for.",
                    target.name()
                ));
            }
            CovertOp::SabotageIndustry => {
                let hit = w.rng.range(0.006, 0.018);
                let t = w.nation_mut(target);
                t.gdp *= 1.0 - hit;
                t.stability = (t.stability - 1.5).max(0.0);
                w.headline(format!(
                    "A run of accidents wrecks {}'s industrial plant. The inquiry finds nothing.",
                    target.name()
                ));
            }
        }
    } else {
        w.headline(format!(
            "A covert operation against {} comes to nothing.",
            target.name()
        ));
    }

    if exposed {
        w.shift_relation(sponsor, target, -35.0);
        w.shift_reputation(sponsor, -12.0);
        w.add_covert_heat(sponsor, target, 0.25);
        {
            let t = w.nation_mut(target);
            // Caught red-handed, a foreign hand is the best thing that can
            // happen to an unpopular government.
            t.stability = (t.stability + 6.0).min(100.0);
            t.separatism = (t.separatism - 0.03).max(0.0);
        }
        let friends: Vec<NationId> = w
            .nations
            .iter()
            .filter(|n| n.alive && n.id != sponsor && n.id != target)
            .map(|n| n.id)
            .filter(|x| w.relation(target, *x) >= 40.0)
            .collect();
        for x in friends {
            w.shift_relation(sponsor, x, -5.0);
        }
        w.headline(format!(
            "{} exposes {} {} in {} — the scandal rallies the country behind its government.",
            target.name(),
            sponsor.name(),
            op.label(),
            target.name()
        ));
    }
    Ok(())
}

/// Offer `to` a trade agreement, which it may refuse.
///
/// Accepted agreements deepen into dependency over years rather than at once,
/// and dependency is what later becomes leverage. Errors if a nation proposes
/// to itself or the pair cannot trade.
pub fn propose_trade(w: &mut WorldState, from: NationId, to: NationId) -> Result<(), String> {
    if from == to {
        return Err("A nation cannot trade with itself.".into());
    }
    if !alive(w, from) || !alive(w, to) {
        return Err("Nation no longer exists.".into());
    }
    if w.trade_depth(from, to) > 0.0 {
        return Err("A trade agreement is already in force.".into());
    }
    if w.is_sanctioning(from, to) || w.is_sanctioning(to, from) {
        return Err("Sanctions bar an agreement.".into());
    }
    if belligerents(w, from, to) {
        return Err("Cannot open markets to a nation you are fighting.".into());
    }

    let rel = w.relation(from, to);
    // Nobody signs away tariff protection to a country they distrust, but the
    // prospect of a much larger market makes a government swallow a lot.
    let (gf, gt) = (w.nation(from).gdp, w.nation(to).gdp);
    let reach = gf / (gf + gt).max(1.0);
    let p = (((rel + 20.0) / 130.0) * (0.55 + reach)).clamp(0.0, 0.85);
    if w.rng.chance(p) {
        let (a, b) = if from <= to { (from, to) } else { (to, from) };
        w.statecraft.trade.push(TradePact { a, b, depth: 0.05 });
        w.shift_relation(from, to, 5.0);
        w.headline(format!(
            "{} and {} sign a trade agreement.",
            from.name(),
            to.name()
        ));
    } else {
        w.headline(format!(
            "Trade talks between {} and {} break down.",
            from.name(),
            to.name()
        ));
    }
    Ok(())
}

/// The leverage that dependency created, finally used. It costs the dependent
/// party a slice of its economy and costs the wielder its reputation.
pub fn abrogate_trade(w: &mut WorldState, from: NationId, to: NationId) -> Result<(), String> {
    if w.trade_depth(from, to) <= 0.0 {
        return Err("No trade agreement to tear up.".into());
    }
    let dep_to = w.trade_dependency(to, from);
    let dep_from = w.trade_dependency(from, to);
    let (a, b) = if from <= to { (from, to) } else { (to, from) };
    w.statecraft.trade.retain(|t| !(t.a == a && t.b == b));

    {
        let n = w.nation_mut(to);
        n.gdp *= 1.0 - dep_to * 0.06;
        n.stability = (n.stability - dep_to * 8.0).max(0.0);
    }
    w.nation_mut(from).gdp *= 1.0 - dep_from * 0.06;
    w.shift_relation(from, to, -25.0);
    w.shift_reputation(from, -6.0);
    w.headline(format!(
        "{} tears up its trade agreement with {}; {}'s exporters reel.",
        from.name(),
        to.name(),
        to.name()
    ));
    Ok(())
}

// ---------------------------------------------------------------------------
// The moment a pact is actually worth something
// ---------------------------------------------------------------------------

/// Called when somebody starts shooting at a state that was guaranteed. Every
/// state that guaranteed the defender is asked, one at a time and in a fixed
/// order, whether it meant it. Returns the ones that did not, so the looser
/// relation-based intervention rule does not quietly walk them back in.
///
/// `at` is the rung the guarantor arrives on, and it is the rung the aggressor
/// is standing on: a guarantee answers the war it was called to, so a standoff
/// strike on a client is met with standoff strike and an invasion is met with a
/// campaign. Before the ladder there was one rung and this was always 8.
///
/// A defensive pact obliges nobody to join a war of aggression, so the
/// attacker's own guarantors are not called at all — the whole asymmetry is
/// what makes a guarantee cheap to give and expensive to keep.
pub fn call_the_guarantors(w: &mut WorldState, c: &mut Conflict, at: u8) -> Vec<NationId> {
    let (attacker, defender) = (c.origin_attacker, c.defender());
    let mut refused: Vec<NationId> = vec![];
    for g in w.pact_partners(defender) {
        if g == attacker {
            // The guarantor is the invader. There is no roll to make.
            dissolve_pact(w, g, defender);
            w.shift_reputation(g, -30.0);
            w.headline(format!(
                "{} tears up its own guarantee to {} to invade it.",
                g.name(),
                defender.name()
            ));
            cheapened_guarantees(w, g, defender);
            continue;
        }
        if !alive(w, g) || c.involves(g) {
            continue;
        }
        let att_nuclear = w.nation(attacker).nuclear;
        let (g_nuclear, g_ex, g_strength) = {
            let n = w.nation(g);
            (n.nuclear, n.war_exhaustion, n.mil_strength)
        };
        let mut p = 0.80;
        // Nobody has ever gone to a nuclear war for a third party.
        if att_nuclear && !g_nuclear {
            p = 0.05;
        }
        p *= 1.0 - g_ex * 0.5;
        if w.at_war(g) {
            p *= 0.5;
        }
        let odds = w.nation(attacker).mil_strength / g_strength.max(1.0);
        if odds > 2.0 {
            p *= 0.6;
        }
        p *= (w.reputation(g) / BASE_REPUTATION).clamp(0.55, 1.15);
        // What the guarantee is for, and what it has cost so far, both argue
        // for honouring it.
        if w.relation(g, defender) > 55.0 {
            p *= 1.1;
        }

        if w.rng.chance(p.clamp(0.0, 0.97)) {
            crate::war::join_side(c, g, false, at.clamp(1, 9), Objective::Deny);
            w.shift_reputation(g, 5.0);
            w.shift_relation(g, defender, 10.0);
            w.headline(format!(
                "{} honours its defence pact with {} and enters the war.",
                g.name(),
                defender.name()
            ));
        } else {
            refused.push(g);
            dissolve_pact(w, g, defender);
            w.shift_reputation(g, -25.0);
            w.shift_relation(g, defender, -45.0);
            w.headline(format!(
                "{} abandons its pact with {}. The guarantee proves worthless.",
                g.name(),
                defender.name()
            ));
            cheapened_guarantees(w, g, defender);
        }
    }
    refused
}

/// A guarantee that failed once is worth less everywhere it was given.
fn cheapened_guarantees(w: &mut WorldState, breaker: NationId, betrayed: NationId) {
    for other in w.pact_partners(breaker) {
        if other == betrayed {
            continue;
        }
        w.shift_relation(breaker, other, -8.0);
    }
}

fn dissolve_pact(w: &mut WorldState, a: NationId, b: NationId) {
    let (x, y) = if a <= b { (a, b) } else { (b, a) };
    w.statecraft.pacts.retain(|p| !(p.a == x && p.b == y));
}

fn alive(w: &WorldState, id: NationId) -> bool {
    w.nation_opt(id).is_some_and(|n| n.alive)
}

/// Fighting *each other*, as opposed to fighting alongside each other. Two
/// nations in the same war on the same side are the likeliest pair in the world
/// to sign something.
pub fn belligerents(w: &WorldState, a: NationId, b: NationId) -> bool {
    w.conflict_between(a, b).is_some()
}
