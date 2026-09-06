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

/// HOW LIKELY A COVERT OPERATION IS TO BE EXPOSED, in one place.
///
/// Factored out so the diplomacy CARD can quote the REALISED change in this
/// probability instead of the raw `gap * 10.0` the arm adds to it. The 0.85
/// ceiling is the whole reason: the arm's slope is 10.0 and the largest
/// reachable diplomacy gap is about 0.076, so an unclamped card offers up to
/// +76 percentage points of exposure into a probability that has at most 75
/// points of room from a cold channel against a pure democracy, and far less in
/// practice -- covert heat accrues 0.18 per operation, so a second operation
/// against the same target starts from about 0.24 and the ceiling binds sooner
/// with every one after that, and sooner again the more authoritarian the
/// target already is.
pub fn exposure_probability(heat: f64, authoritarianism: f64, counterintel: f64) -> f64 {
    (0.10 + heat * 0.55 + authoritarianism * 0.15 + counterintel).clamp(0.05, 0.85)
}


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

// ---------------------------------------------------------------------------
// Foreign backing (the political arm, S3). Every number below is INVENTED —
// a model coefficient of the approved design, not a transcribed figure — and
// filed in BUGS.md with what would calibrate it.
// ---------------------------------------------------------------------------

/// What one clean `BackBloc` operation adds to a sponsor's weight behind a
/// bloc. FIXED: the op draws no third die the way the other three do.
pub const BACKING_STEP: f64 = 0.06;
/// The most one sponsor can hold behind one bloc in one target: two clean
/// operations' worth.
pub const BACKING_SPONSOR_CAP: f64 = 0.12;
/// The most a bloc's F_B can total from every source, sponsors and patronage
/// gravity together (`blocs::backing` applies the same cap to the view).
pub const BACKING_TOTAL_CAP: f64 = 0.25;
/// Monthly cooling of every backing entry, at half the rate covert heat
/// cools (0.012): money that stops arriving is spent within a year or two.
pub const BACKING_DECAY: f64 = 0.006;
/// What exposure costs the backed bloc's own support, in share points.
pub const EXPOSURE_TAINT: f64 = 0.02;

/// Monthly upkeep of the standing arrangements. Nothing here rolls dice: a
/// treaty in force is not a gamble, it is a bill.
pub fn tick(w: &mut WorldState) {
    reputations_recover(w);
    pacts_upkeep(w);
    aid_flows(w);
    trade_deepens(w);
    covert_channels_cool(w);
    backing_cools(w);
    // A seller's refusal is remembered, then gradually not (resources.rs,
    // spec section 6.3). Free while the memory is empty, which is every
    // world the market switch is off in. ONCE A CALENDAR MONTH: the memory
    // cools one step of twenty-fourths per call and the lattice cannot be
    // prorated, so daily play takes its step on the month's last day, the
    // day the legacy settlement took it. Called every day, a refusal was
    // forgotten in 24 days instead of 24 months (2026-09-03, measured
    // 1990-01-25 against 1992-01-01).
    if crate::clock::month_end(w) {
        crate::resources::cool_refusals(w);
    }
}

/// A broken promise is remembered, then gradually not. Nobody climbs back above
/// the baseline by waiting; that has to be earned in a war.
fn reputations_recover(w: &mut WorldState) {
    let dt = crate::clock::month_fraction(w);
    for (_, v) in w.statecraft.reputation.iter_mut() {
        if *v < BASE_REPUTATION {
            *v = (*v + 0.15 * dt).min(BASE_REPUTATION);
        }
    }
}

fn pacts_upkeep(w: &mut WorldState) {
    let dt = crate::clock::month_fraction(w);
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
            // `PACT_UPKEEP / 12.0` is the exact share the pre-treasury line
            // pushed into `debt_gdp` and is what the closed-books arm writes;
            // the dollars beside it are what a nation keeping a treasury pays.
            let bn = w.nation(id).gdp * PACT_UPKEEP / 12.0 * dt;
            crate::economy::charge(w, id, bn, PACT_UPKEEP / 12.0 * dt);
        }
        if w.relation(a, b) < 85.0 {
            w.shift_relation(a, b, 0.20 * dt);
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
    let dt = crate::clock::month_fraction(w);
    let arms_blend = crate::clock::blend(w, 0.02);
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
        // Same two representations as the pact upkeep above: the ratio is
        // the pre-treasury line unchanged, the dollars are the same money.
        let fiscal_bn = w.nation(f.patron).gdp * fiscal / 12.0 * dt;
        crate::economy::charge(w, f.patron, fiscal_bn, fiscal / 12.0 * dt);

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
                c.gdp *= 1.0 + infusion * 0.15 / 12.0 * dt;
                crate::economy::refresh_debt_ratio(c);
                c.stability = (c.stability + infusion * 8.0 / 12.0 * dt).min(100.0);
            }
            AidKind::Arms => {
                // Transferred weapons are budget the client never had to raise.
                // Same curve war.rs uses for domestic spending, so a client's
                // army converges on what its patron is willing to fund.
                let sustained = ((client_gdp * client_mil_share + annual) * 0.30).sqrt() * 8.0;
                let c = w.nation_mut(f.client);
                c.mil_strength += (sustained - client_strength) * arms_blend;
            }
        }
        // Dependence reads as friendship right up until the money stops.
        if w.relation(f.patron, f.client) < 75.0 {
            w.shift_relation(f.patron, f.client, 0.25 * dt);
        }
    }
    w.statecraft
        .aid
        .retain(|f| !dead.contains(&(f.patron, f.client, f.kind)));
}

fn trade_deepens(w: &mut WorldState) {
    let dt = crate::clock::month_fraction(w);
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
            t.depth = (t.depth + 0.012 * dt).min(1.0);
        }
        if w.relation(a, b) < 70.0 {
            w.shift_relation(a, b, 0.15 * dt);
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
                crate::economy::refresh_debt_ratio(n);
                n.trade_level_paid = Some(owed);
            }
            _ => {}
        }
    }
}

fn covert_channels_cool(w: &mut WorldState) {
    let dt = crate::clock::month_fraction(w);
    for (_, _, h) in w.statecraft.covert_heat.iter_mut() {
        *h -= 0.012 * dt;
    }
    w.statecraft.covert_heat.retain(|(_, _, h)| *h > 0.0);
}

/// Where covert channels cool, backing cools too: `BACKING_DECAY` a month,
/// retained only while positive. Returns before touching anything while the
/// stock is empty — every world the switch is off in — and draws no RNG.
fn backing_cools(w: &mut WorldState) {
    if w.statecraft.backing.is_empty() {
        return;
    }
    let dt = crate::clock::month_fraction(w);
    for b in w.statecraft.backing.iter_mut() {
        b.weight -= BACKING_DECAY * dt;
    }
    w.statecraft.backing.retain(|b| b.weight > 0.0);
}

/// Why a `BackBloc` would be refused, read without touching the world — the
/// ONE place the prose lives, asked by `covert_action` and by
/// `lib::world_refusal` before any state is read (the switch first, so a
/// world without the arm refuses before it looks at a ruling bloc).
pub fn back_bloc_refusal(w: &WorldState, target: NationId, bloc: crate::government::Bloc) -> Option<String> {
    if !w.rules.ideology_blocs {
        return Some("This world does not model ideological movements.".into());
    }
    if crate::blocs::ruling_bloc(w, target) == Some(bloc) {
        return Some("You cannot back a government covertly — send aid.".into());
    }
    None
}

/// How much a clean operation would actually add for this sponsor behind
/// this bloc in this target: `BACKING_STEP`, clipped by the sponsor's own
/// cap and the bloc's total cap on the STORED stock. Pure; the arm
/// `add_backing` charges exactly this, and the card quotes it (iron rule 8:
/// an arm clamped downstream is quoted at its clamped value).
pub fn backing_room(w: &WorldState, sponsor: NationId, target: NationId, bloc: crate::government::Bloc) -> f64 {
    let mine = w.backing_of(sponsor, target, bloc);
    let total: f64 = w
        .statecraft
        .backing
        .iter()
        .filter(|b| b.target == target && b.bloc == bloc)
        .map(|b| b.weight)
        .sum();
    BACKING_STEP
        .min(BACKING_SPONSOR_CAP - mine)
        .min(BACKING_TOTAL_CAP - total)
        .max(0.0)
}

/// The arm of a clean `BackBloc`: `backing_room` added to the sponsor's
/// entry, the stock kept sorted by (sponsor, target, bloc). Returns what was
/// added. Writes nothing when the room is zero.
pub fn add_backing(w: &mut WorldState, sponsor: NationId, target: NationId, bloc: crate::government::Bloc) -> f64 {
    let room = backing_room(w, sponsor, target, bloc);
    if room <= 0.0 {
        return 0.0;
    }
    match w
        .statecraft
        .backing
        .iter_mut()
        .find(|b| b.sponsor == sponsor && b.target == target && b.bloc == bloc)
    {
        Some(b) => b.weight += room,
        None => w.statecraft.backing.push(Backing { sponsor, target, bloc, weight: room, exposed: false }),
    }
    w.statecraft.backing.sort_by_key(|b| (b.sponsor, b.target, b.bloc));
    room
}

/// What exposure of a `BackBloc` does beyond the existing costs of being
/// caught: every backing entry this sponsor holds in this target is halved
/// and marked exposed, and the backed bloc is tainted `EXPOSURE_TAINT` of
/// support — in an electoral target off its parties in proportion to their
/// size, in a regime off `movements[bloc]` — then renormalised. Draws no RNG.
pub fn expose_backing(w: &mut WorldState, sponsor: NationId, target: NationId, bloc: crate::government::Bloc) {
    for b in w.statecraft.backing.iter_mut() {
        if b.sponsor == sponsor && b.target == target {
            b.weight *= 0.5;
            b.exposed = true;
        }
    }
    crate::government::taint_bloc(w, target, bloc, EXPOSURE_TAINT);
}

/// The Security Crackdown's gated arm (S3): every stored backing entry
/// behind a NON-ruling bloc in this nation is halved — the apparatus on the
/// street rolls up the foreign money behind the opposition. The ruling
/// bloc's entries are untouched (a government is not raiding its own
/// friends), and patronage gravity, a view of the aid flows, is not a stock
/// and cannot be halved. Returns before reading anything while the arm is
/// off or the stock is empty. Draws no RNG.
pub fn halve_foreign_backing(w: &mut WorldState, id: NationId) {
    if !w.rules.ideology_blocs || w.statecraft.backing.is_empty() {
        return;
    }
    let ruling = crate::blocs::ruling_bloc(w, id);
    for b in w.statecraft.backing.iter_mut() {
        if b.target == id && Some(b.bloc) != ruling {
            b.weight *= 0.5;
        }
    }
}

/// The crackdown arm's card, from the same rule `halve_foreign_backing`
/// applies (iron rule 8): per non-ruling bloc with backing, F_B before and
/// after, each quoted at its CLAMPED value — the stock halved, the gravity
/// kept, the total capped as `blocs::backing` caps it. Empty with the arm
/// off, and empty where there is nothing to halve.
pub fn crackdown_backing_effects(w: &WorldState, id: NationId) -> Vec<String> {
    if !w.rules.ideology_blocs {
        return vec![];
    }
    let ruling = crate::blocs::ruling_bloc(w, id);
    let stock = crate::blocs::backing_stock(w, id);
    let grav = crate::blocs::gravity(w, id);
    let before = crate::blocs::backing(w, id);
    let mut out = vec![];
    for i in 0..5 {
        let (bloc, s) = stock[i];
        if s <= 0.0 || Some(bloc) == ruling {
            continue;
        }
        let after = (s * 0.5 + grav[i].1).min(BACKING_TOTAL_CAP);
        out.push(format!(
            "Foreign backing of the {} movement halved: {:.3} → {:.3}.",
            bloc.label(), before[i].1, after
        ));
    }
    out
}

/// The one-sentence arms of a `BackBloc`, from the same constants and the
/// same `backing_room` the op charges, for the card (iron rule 8).
pub fn back_bloc_effects(w: &WorldState, sponsor: NationId, target: NationId, bloc: crate::government::Bloc) -> Vec<String> {
    let room = backing_room(w, sponsor, target, bloc);
    vec![
        format!(
            "If it works: +{:.2} backing for the {} movement (sponsor cap {:.2}, bloc cap {:.2}); {:.2} realised now.",
            BACKING_STEP, bloc.label(), BACKING_SPONSOR_CAP, BACKING_TOTAL_CAP, room
        ),
        format!("Backing cools {:.3} a month while covert channels cool.", BACKING_DECAY),
        format!(
            "If exposed: every entry of {}'s backing in {} halved and named, the {} movement loses {:.2} of support, and the usual costs of being caught.",
            sponsor.name(), target.name(), bloc.label(), EXPOSURE_TAINT
        ),
        "Backing never enters support: it counts in influence only.".to_string(),
    ]
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
    if let CovertOp::BackBloc(bloc) = op {
        if let Some(why) = back_bloc_refusal(w, target, bloc) {
            return Err(why);
        }
    }

    // Running a service costs money whether or not anything comes of it.
    // 0.0008 of output is the pre-treasury line unchanged.
    let service_bn = w.nation(sponsor).gdp * 0.0008;
    crate::economy::charge(w, sponsor, service_bn, 0.0008);

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
    // A police state catches spies; a well-worn channel gets rolled up — and,
    // DIPLOMACY'S SECOND NAMED ARM, a foreign service that funds its own
    // counter-intelligence catches them too.
    //
    // Note whose gap this is: the TARGET's, not the sponsor's. Diplomacy is the
    // only ministry on the board whose budget acts on somebody else's decision,
    // and it acts through a path that already exists and is already priced —
    // exposure is what costs the sponsor relations and reputation below, and
    // this arm changes only how often that path is taken. It invents no new
    // consequence.
    //
    // INVENTED, and labelled as the design requires: the 10.0 slope. The design
    // sizes it at "+0.5% of GDP is about +5 percentage points of exposure", and
    // 0.005 * 10.0 = 0.05 is exactly that. The existing `clamp(0.05, 0.85)`
    // bounds it, so no budget can make a service infallible.
    //
    // INERT WITHOUT A PLAN: `budget_gap` is 0.0 for a target with no enacted
    // budget, and adding an exact zero to a positive sum is exact, so the RNG
    // draw below is unchanged on every default path.
    let t_dip = w.nation(target).budget_gap(BUDGET_DIPLOMACY);
    let expose_p =
        exposure_probability(heat, t_auth, crate::ministries::diplomacy_counterintel(t_dip));

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
                crate::economy::refresh_debt_ratio(t);
                t.stability = (t.stability - 1.5).max(0.0);
                w.headline(format!(
                    "A run of accidents wrecks {}'s industrial plant. The inquiry finds nothing.",
                    target.name()
                ));
            }
            // FIXED effect, no third draw: the two `chance` rolls above are
            // the whole of this op's randomness, so the stream is the stream
            // the other three leave.
            CovertOp::BackBloc(bloc) => {
                add_backing(w, sponsor, target, bloc);
                w.headline(format!(
                    "Money and organisers reach the {} movement in {}; nobody can say from where.",
                    bloc.label(),
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
        caught(w, sponsor, target, op);
    }
    Ok(())
}

/// What being caught costs, defined once: the block `covert_action` charges
/// on its exposure roll, verbatim, and — under the roads (S4) — what a
/// takeover charges a sponsor whose channel into the winner was already
/// half-blown (`takeover_payoff`). Draws no RNG.
pub(crate) fn caught(w: &mut WorldState, sponsor: NationId, target: NationId, op: CovertOp) {
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
    if let CovertOp::BackBloc(bloc) = op {
        expose_backing(w, sponsor, target, bloc);
    }
    w.headline(format!(
        "{} exposes {} {} in {} — the scandal rallies the country behind its government.",
        target.name(),
        sponsor.name(),
        op.label(),
        target.name()
    ));
}

// ---------------------------------------------------------------------------
// The foreign payoff on a takeover (S4). Every number INVENTED (design S4)
// and filed in BUGS.md.
// ---------------------------------------------------------------------------

/// The backing at or over which a sponsor is paid by the winner it backed.
pub const PAYOFF_BACKING: f64 = 0.10;
/// What the new regime's gratitude is worth in relation.
pub const PAYOFF_SPONSOR: f64 = 40.0;
/// What the sponsor loses with the target's previous top patron, whose
/// client it has just taken.
pub const PAYOFF_RIVAL: f64 = -25.0;
/// The covert heat at or over which the sponsorship is exposed at the
/// moment of the takeover, with the usual costs (`caught`).
pub const PAYOFF_EXPOSED_HEAT: f64 = 0.50;
/// What the democracies (authoritarianism under `government::
/// DEMOCRACY_BELOW`) feel about a new regime: −8 for a Communist,
/// Nationalist or Islamist one, +8 for a Western one, nothing for a
/// Non-Aligned one.
pub const PAYOFF_DEMOCRACY: f64 = 8.0;
/// What a great-power patron ruling in the LOSER's colour feels.
pub const PAYOFF_LOSER_PATRON: f64 = -10.0;

/// Everything `takeover_payoff` writes, computed once (rule 8: the card
/// quotes this plan).
#[derive(Clone, Debug, PartialEq)]
pub struct Payoff {
    /// (sponsor, backing held, exposed at this moment).
    pub sponsors: Vec<(NationId, f64, bool)>,
    /// The target's previous top patron by aid share, if any.
    pub top_patron: Option<NationId>,
    /// (democracy, shift) — every democracy other than the target.
    pub democracies: Vec<(NationId, f64)>,
    /// Great-power patrons whose own ruling bloc is the loser's.
    pub loser_patrons: Vec<NationId>,
}

/// The payoff plan for a non-ballot takeover of `target` by `winner` from
/// `loser`, or `None` before any read while the takeover switch is off.
pub fn takeover_payoff_plan(
    w: &WorldState,
    target: NationId,
    winner: crate::government::Bloc,
    loser: Option<crate::government::Bloc>,
) -> Option<Payoff> {
    use crate::government::Bloc;
    if !w.rules.ideology_takeover {
        return None;
    }
    let sponsors: Vec<(NationId, f64, bool)> = w
        .statecraft
        .backing
        .iter()
        .filter(|b| b.target == target && b.bloc == winner && b.weight >= PAYOFF_BACKING)
        .filter(|b| w.nation_opt(b.sponsor).is_some_and(|n| n.alive))
        .map(|b| (b.sponsor, b.weight, w.covert_heat(b.sponsor, target) >= PAYOFF_EXPOSED_HEAT))
        .collect();
    let mut top: Option<(NationId, f64)> = None;
    for p in w.patrons_of(target) {
        let share: f64 = w.statecraft.aid.iter().filter(|f| f.patron == p && f.client == target).map(|f| f.share_gdp).sum();
        if top.map_or(true, |(_, s)| share > s) {
            top = Some((p, share));
        }
    }
    let democracy_shift = match winner {
        Bloc::Communist | Bloc::Nationalist | Bloc::Islamist => -PAYOFF_DEMOCRACY,
        Bloc::Western => PAYOFF_DEMOCRACY,
        Bloc::NonAligned => 0.0,
    };
    let democracies: Vec<(NationId, f64)> = if democracy_shift == 0.0 {
        vec![]
    } else {
        crate::government::democracies(w, target).into_iter().map(|d| (d, democracy_shift)).collect()
    };
    let loser_patrons: Vec<NationId> = match loser {
        Some(l) => crate::nations::patrons()
            .iter()
            .copied()
            .filter(|p| *p != target && w.nation_opt(*p).is_some_and(|n| n.alive))
            .filter(|p| crate::blocs::ruling_bloc(w, *p) == Some(l))
            .collect(),
        None => vec![],
    };
    Some(Payoff { sponsors, top_patron: top.map(|(p, _)| p), democracies, loser_patrons })
}

/// The foreign payoff on any non-ballot takeover (S4): every sponsor holding
/// `PAYOFF_BACKING` of the winner +40 with the new regime and −25 with the
/// target's previous top patron, exposed on the spot (the usual costs)
/// where its covert heat is at or over 0.50; the democracies −8 with a new
/// Communist / Nationalist / Islamist regime and +8 with a new Western one;
/// the great-power patrons ruling in the loser's colour −10. Writes
/// nothing while the takeover switch is off. Draws no RNG.
pub fn takeover_payoff(
    w: &mut WorldState,
    target: NationId,
    winner: crate::government::Bloc,
    loser: Option<crate::government::Bloc>,
) {
    let p = match takeover_payoff_plan(w, target, winner, loser) {
        Some(p) => p,
        None => return,
    };
    for (sponsor, _, exposed) in &p.sponsors {
        w.shift_relation(*sponsor, target, PAYOFF_SPONSOR);
        if let Some(top) = p.top_patron {
            if top != *sponsor {
                w.shift_relation(*sponsor, top, PAYOFF_RIVAL);
            }
        }
        if *exposed {
            caught(w, *sponsor, target, CovertOp::BackBloc(winner));
        }
    }
    for (d, shift) in &p.democracies {
        w.shift_relation(*d, target, *shift);
    }
    for patron in &p.loser_patrons {
        w.shift_relation(*patron, target, PAYOFF_LOSER_PATRON);
    }
}

/// The payoff's arms as the screen prints them, from the same plan.
pub fn takeover_payoff_effects(
    w: &WorldState,
    target: NationId,
    winner: crate::government::Bloc,
    loser: Option<crate::government::Bloc>,
) -> Vec<String> {
    let p = match takeover_payoff_plan(w, target, winner, loser) {
        Some(p) => p,
        None => return vec![],
    };
    let mut out = vec![];
    for (sponsor, held, exposed) in &p.sponsors {
        out.push(format!(
            "{} holds {:.3} of the {} movement's backing: relations {:+.0} with the new regime{}{}.",
            sponsor.name(),
            held,
            winner.label(),
            PAYOFF_SPONSOR,
            p.top_patron.filter(|t| t != sponsor).map_or(String::new(), |t| format!(", {:+.0} with {}", PAYOFF_RIVAL, t.name())),
            if *exposed { "; exposed on the spot" } else { "" }
        ));
    }
    if let Some((_, shift)) = p.democracies.first() {
        out.push(format!("Relations {:+.0} with {} democracies.", shift, p.democracies.len()));
    }
    for patron in &p.loser_patrons {
        out.push(format!("Relations {:+.0} with {}, which rules in the old colour.", PAYOFF_LOSER_PATRON, patron.name()));
    }
    out
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
        crate::economy::refresh_debt_ratio(n);
        n.stability = (n.stability - dep_to * 8.0).max(0.0);
    }
    w.nation_mut(from).gdp *= 1.0 - dep_from * 0.06;
    crate::economy::refresh_debt_ratio(w.nation_mut(from));
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
        if c.side_a.iter().any(|foe| crate::sovereignty::hostility_blocked(w,g,*foe)) {
            // The formal sphere outranks an outside guarantee. No hidden
            // opposing coalition entry and no repeated random loyalty roll.
            refused.push(g);
            continue;
        }
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
