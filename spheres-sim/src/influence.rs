//! Influence projection — the system the game is named after.
//!
//! `statecraft.rs` gave a patron four instruments. Each of them was an END:
//! you signed a pact, or you didn't; you paid aid, or you didn't. Nothing
//! accumulated, nothing wore off, and a sphere of influence was therefore
//! something you BOUGHT rather than something you HELD.
//!
//! This module is the stock those four instruments now feed, and the decay that
//! makes holding it a standing bill:
//!
//! * **A stake** is what one patron has invested in one client, 0..100. Aid,
//!   arms, trade dependency, a defence guarantee and plain diplomatic effort all
//!   pay into the same number. Nothing else about them changes.
//! * **It decays every month, unconditionally.** That single line is the whole
//!   mechanic: a sphere held at 60 points bleeds about two points a month and
//!   has to be topped up forever. Stop paying and it is gone in three years.
//!   Spend-to-hold, not spend-to-buy.
//! * **Alignment has hysteresis.** A client leans toward whoever holds the most,
//!   but a rival has to beat the incumbent by a clear margin and HOLD that lead
//!   for half a year before anything flips. Drifting out of an orbit is slow,
//!   quiet and reversible. Being taken is sudden, public, and costs the loser
//!   two thirds of everything it built.
//! * **Two patrons contesting one client is the expensive state**, because each
//!   side's work partly cancels the other's: contest raises decay for both and
//!   roughly doubles the monthly bill. That is the actual texture of the period
//!   and it now has a price tag rather than a flavour text.
//!
//! Nothing here rolls a die. A programme in force is a bill, in exactly the
//! sense `statecraft::tick` means it; the flips are thresholds, not gambles.

use crate::world::*;

// ---------------------------------------------------------------------------
// The constants, and what each of them means in months
// ---------------------------------------------------------------------------

/// Monthly fractional decay of a stake nobody is refreshing. 0.030 is about 31%
/// a year: an abandoned sphere is halved in under two years and is below the
/// alignment floor in three. Diplomats rotate out, the scholarships lapse, the
/// officers who trained with you retire.
pub const BASE_DECAY: f64 = 0.030;

/// The incumbent's advantage, and the first half of the hysteresis. The patron a
/// client is already aligned to decays slower, because what it built is
/// institutional: the staff college syllabus, the spare parts, the language of
/// the officer corps. Holding is cheaper than taking, which is why spheres are
/// stable for decades and then are not.
pub const INCUMBENT_RELIEF: f64 = 0.45;

/// Extra decay a contested stake suffers, per unit of rival pressure. Two
/// powers working the same capital spend much of their effort undoing each
/// other's, and both of them pay for it.
pub const CONTEST_DECAY: f64 = 0.045;

/// Points a month bought by a full-effort diplomatic programme — embassies,
/// visits, scholarships, broadcasting, the party-to-party channel. Alone, at
/// full effort into a friendly poor country and with nobody contesting it, this
/// settles a stake near 55: comfortably past the alignment floor, comfortably
/// short of what it takes to prise a well-funded client off a rival. That gap is
/// deliberate and it is the point of the whole redesign — diplomacy alone gets
/// you an unclaimed capital, and taking a held one needs an instrument.
pub const PROJECTION_PER_EFFORT: f64 = 2.4;

/// What the instruments pay in, per month, at their maximum. These are the
/// numbers that make the four instruments means rather than ends.
///
/// Aid is the strongest because it is the one a government cannot replace: it
/// is this month's salaries. Arms buy the officer corps but not the street, so
/// they pay about two thirds. Trade binds a whole economy but slowly and
/// impersonally. A guarantee is worth a steady, modest amount forever.
const AID_WEIGHT: f64 = 6.0; // x infusion, capped at MAX_INFUSION 0.25 -> 1.50/mo
const ARMS_WEIGHT: f64 = 4.0; // -> 1.00/mo
const TRADE_WEIGHT: f64 = 1.2; // x dependency 0..1
const PACT_WEIGHT: f64 = 1.0; // flat, while it is in force

/// Below this a stake is not a sphere, it is an embassy. Nobody is anybody's
/// client under it and the alignment simply lapses.
pub const ALIGN_FLOOR: f64 = 25.0;

/// The second half of the hysteresis, and the part that makes a flip HARD. A
/// challenger has to be this far ahead of the incumbent, and stay there for
/// `FLIP_MONTHS` running, before the client changes sides. Fall back for one
/// month and the count starts again.
pub const FLIP_MARGIN: f64 = 12.0;
pub const FLIP_MONTHS: u32 = 6;

/// What a flip costs the power that lost. Bases close, advisers are put on
/// aircraft, the contracts are torn up: two thirds of the stake goes in the
/// month it happens, which is what makes buying a client back so much more
/// expensive than holding one.
const LOSS_ON_FLIP: f64 = 0.35;

/// ...and the humiliation at home. Losing a client is a foreign-policy defeat
/// the government has to answer for.
const FLIP_PC_COST: f64 = 6.0;

/// Political capital a month, per POINT of stake held, simply for holding it:
/// the ambassador, the desk, the visits nobody at home wants to pay for, the
/// standing line in the budget. A sixty-point sphere costs 0.07 a month at rest.
///
/// The scale is set by what a government can actually earn. `political_capital`
/// in politics.rs walks the stock 2.8% of the way to its target each month, so a
/// mature power sitting thirty points below target regenerates about 0.8 a
/// month, and that — not a designer's preference — is the whole budget a great
/// power has for everything it does abroad. Ten clients at rest is 0.7 of it.
/// Try to hold twenty and you are insolvent, which is the mechanic.
const UPKEEP_PC_PER_POINT: f64 = 0.0012;
/// ...and what an active programme costs on top of that, at full effort. Three
/// programmes at full effort is most of a great power's monthly income.
const UPKEEP_PC_EFFORT: f64 = 0.22;

/// A stake this small is not worth the paperwork; the desk quietly closes.
const PRUNE_BELOW: f64 = 0.4;

// ---------------------------------------------------------------------------
// Reading the board
// ---------------------------------------------------------------------------

/// A great power is nobody's client. Not a balance decision — a statement about
/// what the word means. Without it the symmetric relations matrix reads "France
/// stands at +80 with the United States" as a French sphere over Washington, and
/// then the orbit rule in `tick` drags Washington's opinion around after Paris's
/// enemies. Measured before this line existed: the United States and China ended
/// a 35-year run at -95, having started at -10, purely through that chain.
///
/// The same membership `politics::best_client` uses for the same reason.
pub fn can_be_client(id: NationId) -> bool {
    !patrons().contains(&id) && !majors().contains(&id)
}

impl WorldState {
    pub fn stake(&self, patron: NationId, client: NationId) -> f64 {
        self.statecraft
            .influence
            .iter()
            .find(|s| s.patron == patron && s.client == client)
            .map(|s| s.stock)
            .unwrap_or(0.0)
    }
    /// The standing diplomatic effort a patron has committed to a client, 0..1.
    /// This is the dial the player and the AI actually turn.
    pub fn effort(&self, patron: NationId, client: NationId) -> f64 {
        self.statecraft
            .influence
            .iter()
            .find(|s| s.patron == patron && s.client == client)
            .map(|s| s.effort)
            .unwrap_or(0.0)
    }
    /// Who this client currently leans to, if anyone. Not simply "who holds the
    /// most": it is the recorded alignment, which resists.
    pub fn aligned_to(&self, client: NationId) -> Option<NationId> {
        self.statecraft
            .alignment
            .iter()
            .find(|a| a.client == client)
            .map(|a| a.patron)
    }
    /// Everyone with a stake worth naming in this client, strongest first and
    /// NationId order on ties, so the readouts and the AI agree.
    pub fn stakeholders(&self, client: NationId) -> Vec<(NationId, f64)> {
        let mut v: Vec<(NationId, f64)> = self
            .statecraft
            .influence
            .iter()
            .filter(|s| s.client == client && s.stock > 1.0)
            .map(|s| (s.patron, s.stock))
            .collect();
        v.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal).then(a.0.cmp(&b.0)));
        v
    }
    /// Every client a patron has a stake in, strongest first.
    pub fn sphere_of(&self, patron: NationId) -> Vec<(NationId, f64)> {
        let mut v: Vec<(NationId, f64)> = self
            .statecraft
            .influence
            .iter()
            .filter(|s| s.patron == patron && s.stock > 1.0)
            .map(|s| (s.client, s.stock))
            .collect();
        v.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal).then(a.0.cmp(&b.0)));
        v
    }
    /// How much two powers working the same capital actually get in each other's
    /// way, 0..1. Britain and France both courting Cairo is two embassies and
    /// costs them a little; Washington and Moscow both courting Cairo is a
    /// contest and costs them both a great deal. Rivalry is a slope rather than
    /// a switch because the alternative measured nothing: after 1991 almost no
    /// pair of patrons in this model sits below the hostility line, so a
    /// threshold made "contested" a permanently empty set and the interesting
    /// state never occurred.
    pub fn rivalry(&self, a: NationId, b: NationId) -> f64 {
        ((20.0 - self.relation(a, b)) / 80.0).clamp(0.0, 1.0)
    }
    /// The rival crowding this stake hardest, and what it holds. Weighted by
    /// rivalry, so the answer is who is actually a problem rather than simply
    /// who is largest.
    pub fn contested_by(&self, patron: NationId, client: NationId) -> Option<(NationId, f64)> {
        let mine = self.stake(patron, client);
        self.stakeholders(client)
            .into_iter()
            .filter(|(q, _)| *q != patron)
            .map(|(q, s)| (q, s, self.rivalry(patron, q) * s / (s + mine + 10.0)))
            .filter(|(_, _, p)| *p > 0.01)
            .max_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(q, s, _)| (q, s))
    }
    /// 0..1 — how much of the room a rival is taking up, hostility included.
    pub fn contest_pressure(&self, patron: NationId, client: NationId) -> f64 {
        let mine = self.stake(patron, client);
        self.stakeholders(client)
            .into_iter()
            .filter(|(q, _)| *q != patron)
            .map(|(q, s)| self.rivalry(patron, q) * s / (s + mine + 10.0))
            .fold(0.0, f64::max)
    }
    /// What one client costs this patron in political capital every month.
    pub fn sphere_upkeep(&self, patron: NationId, client: NationId) -> f64 {
        let stock = self.stake(patron, client);
        let effort = self.effort(patron, client);
        if stock <= PRUNE_BELOW && effort <= 0.0 {
            return 0.0;
        }
        (UPKEEP_PC_PER_POINT * stock + UPKEEP_PC_EFFORT * effort)
            * (1.0 + self.contest_pressure(patron, client))
    }
    /// ...and what the whole sphere costs. The number the briefing leads with.
    pub fn sphere_bill(&self, patron: NationId) -> f64 {
        self.statecraft
            .influence
            .iter()
            .filter(|s| s.patron == patron)
            .map(|s| self.sphere_upkeep(patron, s.client))
            .sum()
    }
}

// ---------------------------------------------------------------------------
// What a month does
// ---------------------------------------------------------------------------

/// How receptive a client is to being bought at all. A rich, orderly country
/// that likes you fine is still not for sale; a poor one that already leans your
/// way is.
fn receptivity(w: &WorldState, patron: NationId, client: NationId) -> f64 {
    let c = match w.nation_opt(client) {
        Some(c) => c,
        None => return 0.0,
    };
    let rel = w.relation(patron, client);
    // Hostility is not merely unhelpful, it is a wall: nobody projects into a
    // capital that has expelled their ambassador.
    let warmth = ((rel + 70.0) / 110.0).clamp(0.0, 1.15);
    // A country rich enough to fund its own government is not for sale, which is
    // the same rule `politics::client_score` applies and for the same reason.
    let wealth = (c.gdp * 1000.0 / c.population.max(0.1) / 20000.0).clamp(0.0, 1.0);
    warmth * (1.0 - 0.75 * wealth)
}

/// How well a patron can reach at all: size relative to the client, and whether
/// it is next door. A superpower projects into a small neighbour easily and into
/// a large one across an ocean with difficulty.
fn reach(w: &WorldState, patron: NationId, client: NationId) -> f64 {
    let (p, c) = match (w.nation_opt(patron), w.nation_opt(client)) {
        (Some(p), Some(c)) => (p.gdp, c.gdp),
        _ => return 0.0,
    };
    let size = 0.55 + 0.45 * (p / (p + c).max(1.0));
    size * if adjacent(patron, client) { 1.25 } else { 1.0 }
}

/// Points a month the standing arrangements pay into a stake, before resistance.
/// This is where the four instruments stop being ends in themselves.
fn instruments(w: &WorldState, patron: NationId, client: NationId) -> f64 {
    let cg = w.nation_opt(client).map(|n| n.gdp).unwrap_or(0.0).max(0.1);
    let pg = w.nation_opt(patron).map(|n| n.gdp).unwrap_or(0.0);
    let mut v = 0.0;
    for f in w.statecraft.aid.iter().filter(|f| f.patron == patron && f.client == client) {
        // The same infusion `statecraft::aid_flows` pays the client's economy —
        // what the transfer is worth at the receiving end, not at the sending
        // one, which is the whole asymmetry of patronage.
        let infusion = (pg * f.share_gdp / cg).min(0.25);
        v += infusion
            * match f.kind {
                AidKind::Economic => AID_WEIGHT,
                AidKind::Arms => ARMS_WEIGHT,
            };
    }
    v += w.trade_dependency(client, patron) * TRADE_WEIGHT;
    if w.allied(patron, client) {
        v += PACT_WEIGHT;
    }
    v
}

/// The monthly bill and the monthly dividend, for every stake on the board.
///
/// Order is the stake vector's own order, which is append order and therefore
/// fixed by construction; nothing here iterates a map and nothing draws.
pub fn tick(w: &mut WorldState) {
    // ---- 0. Open a ledger line wherever an instrument is already in force.
    // Without this, the four instruments could only feed a stake somebody had
    // already opened by hand, so Moscow's post-1992 guarantees to Armenia,
    // Kazakhstan, Uzbekistan and Belarus — the Tashkent treaty, which
    // politics.rs writes down as four real pacts — bought Moscow no influence
    // whatsoever and Russia finished a 35-year run holding nothing at all. An
    // arrangement that is in force IS a position; this is where it becomes one.
    // Trade needs real integration first: a fresh treaty is a gesture. ----
    let arrivals: Vec<(NationId, NationId)> = {
        let mut v = vec![];
        for p in patrons().iter().copied() {
            if w.nation_opt(p).map_or(true, |n| !n.alive) {
                continue;
            }
            for c in all_nations().iter().copied() {
                if c == p || !can_be_client(c) || w.nation_opt(c).map_or(true, |n| !n.alive) {
                    continue;
                }
                if w.stake(p, c) > 0.0 || w.effort(p, c) > 0.0 {
                    continue;
                }
                let arranged = w.aid_share_to(p, c) > 0.0
                    || w.allied(p, c)
                    || w.trade_depth(p, c) > 0.25;
                if arranged {
                    v.push((p, c));
                }
            }
        }
        v
    };
    for (p, c) in arrivals {
        w.statecraft.influence.push(Stake { patron: p, client: c, stock: 0.0, effort: 0.0 });
    }

    // ---- 1. Decay, then accumulation, on every stake. ----
    let stakes: Vec<(NationId, NationId)> = w
        .statecraft
        .influence
        .iter()
        .map(|s| (s.patron, s.client))
        .collect();
    for (p, c) in stakes {
        let alive = w.nation_opt(p).map_or(false, |n| n.alive)
            && w.nation_opt(c).map_or(false, |n| n.alive);
        if !alive {
            if let Some(s) = stake_mut(w, p, c) {
                s.stock = 0.0;
                s.effort = 0.0;
            }
            continue;
        }
        let incumbent = w.aligned_to(c) == Some(p);
        let pressure = w.contest_pressure(p, c);
        let decay = BASE_DECAY * if incumbent { 1.0 - INCUMBENT_RELIEF } else { 1.0 }
            + CONTEST_DECAY * pressure;
        let gain = {
            let effort = w.effort(p, c);
            (PROJECTION_PER_EFFORT * effort + instruments(w, p, c))
                * receptivity(w, p, c)
                * reach(w, p, c)
        };
        if let Some(s) = stake_mut(w, p, c) {
            s.stock = (s.stock * (1.0 - decay) + gain).clamp(0.0, 100.0);
        }
    }

    // ---- 2. The bill, in the currency every decision is charged against. A
    // patron that cannot pay does not go into debt; its programmes are cut,
    // which is how a sphere is lost to trouble at home rather than abroad. ----
    let payers: Vec<NationId> = {
        let mut v: Vec<NationId> = w.statecraft.influence.iter().map(|s| s.patron).collect();
        v.sort();
        v.dedup();
        v
    };
    for p in payers {
        if w.nation_opt(p).map_or(true, |n| !n.alive) {
            continue;
        }
        let bill = w.sphere_bill(p);
        if bill <= 0.0 {
            continue;
        }
        let held = w.nation(p).political_capital;
        if held >= bill {
            w.nation_mut(p).political_capital = (held - bill).max(0.0);
            continue;
        }
        w.nation_mut(p).political_capital = 0.0;
        // Retrenchment, and its order is the argument. A government that has run
        // out of standing at home does not abandon its most valuable client and
        // does not abandon its cheapest; it abandons the one it is already
        // LOSING — the weakest stake on its books. Fixed order, no draw.
        //
        // This is what makes the whole thing self-limiting. A power can inherit
        // or acquire more clients than its politics can pay for, and it will
        // then shed them, one a month, until the bill fits what the government
        // can actually earn. Nobody has to take them off it.
        let weakest = w.sphere_of(p).last().map(|(c, _)| *c).or_else(|| {
            w.statecraft.influence.iter().find(|s| s.patron == p).map(|s| s.client)
        });
        let Some(c) = weakest else { continue };
        let effort = w.effort(p, c);
        if effort > 0.0 {
            if let Some(s) = stake_mut(w, p, c) {
                s.effort = (effort - 0.34).max(0.0);
            }
            if w.effort(p, c) <= 0.0 {
                w.headline(format!(
                    "{} quietly winds up its programmes in {} — the standing will not stretch.",
                    p.name(),
                    c.name()
                ));
            }
        } else {
            // Nothing left to cut but the position itself.
            let mattered = w.stake(p, c) >= ALIGN_FLOOR;
            if let Some(s) = stake_mut(w, p, c) {
                s.stock = 0.0;
            }
            if mattered {
                w.headline(format!(
                    "{} closes its mission in {}; the commitment outlived what {} could pay for it.",
                    p.name(),
                    c.name(),
                    p.name()
                ));
            }
        }
    }

    // ---- 3. Alignment: resist, then flip. ----
    let clients: Vec<NationId> = {
        let mut v: Vec<NationId> = w.statecraft.influence.iter().map(|s| s.client).collect();
        v.sort();
        v.dedup();
        v
    };
    for c in clients {
        if w.nation_opt(c).map_or(true, |n| !n.alive) {
            w.statecraft.alignment.retain(|a| a.client != c);
            continue;
        }
        settle_alignment(w, c);
    }

    // ---- 4. What alignment is FOR. A client in somebody's orbit drifts toward
    // its patron and away from its patron's enemies, month on month, bounded so
    // it pulls the world's opinion rather than pinning it. Relations are what
    // every other system reads, so this is where influence reaches war
    // appetite, pacts, sanctions and trade without touching any of them. ----
    let orbits: Vec<(NationId, NationId)> = w
        .statecraft
        .alignment
        .iter()
        .map(|a| (a.client, a.patron))
        .collect();
    for (c, p) in orbits {
        if w.relation(p, c) < 65.0 {
            w.shift_relation(p, c, 0.20);
        }
        let enemies: Vec<NationId> = w
            .nations
            .iter()
            .filter(|n| n.alive && n.id != c && n.id != p)
            .map(|n| n.id)
            .filter(|q| w.relation(p, *q) < -30.0)
            .collect();
        for q in enemies {
            if w.relation(c, q) > -55.0 {
                w.shift_relation(c, q, -0.10);
            }
        }
    }

    // ---- 5. Close the dead desks. ----
    w.statecraft
        .influence
        .retain(|s| s.stock > PRUNE_BELOW || s.effort > 0.0);
}

fn stake_mut<'a>(
    w: &'a mut WorldState,
    patron: NationId,
    client: NationId,
) -> Option<&'a mut Stake> {
    w.statecraft
        .influence
        .iter_mut()
        .find(|s| s.patron == patron && s.client == client)
}

/// The hysteresis, in one function.
///
/// Three outcomes and they are deliberately not symmetric:
///
/// * Nobody holds enough — the alignment lapses quietly. This is DRIFT: slow,
///   cheap, and reversible by simply starting to pay again.
/// * The incumbent still leads, or a challenger leads by too little, or has not
///   led for long enough — nothing happens at all, and the challenge counter
///   moves. This is the resistance.
/// * A challenger has led by `FLIP_MARGIN` for `FLIP_MONTHS` running — it takes
///   the client, and the incumbent loses two thirds of everything it built plus
///   a chunk of its government's standing at home. This is the hard flip.
fn settle_alignment(w: &mut WorldState, client: NationId) {
    let board = w.stakeholders(client);
    let leader = board.first().copied();
    let current = w.aligned_to(client);

    let Some((top, top_stock)) = leader else {
        drop_alignment(w, client, current);
        return;
    };
    if top_stock < ALIGN_FLOOR {
        drop_alignment(w, client, current);
        return;
    }

    let Some(holder) = current else {
        // An unaligned client falls in with whoever is clearly ahead. There is
        // no incumbent to resist, so this is the one cheap acquisition in the
        // model — and it is exactly the one every power races for.
        w.statecraft.alignment.push(Alignment {
            client,
            patron: top,
            since_year: w.year,
            since_month: w.month,
            challenger: None,
            challenge_months: 0,
        });
        w.shift_relation(top, client, 6.0);
        w.headline(format!(
            "{} moves into {}'s orbit.",
            client.name(),
            top.name()
        ));
        return;
    };

    let held = w.stake(holder, client);
    // The incumbent is still on top, or the lead is inside the margin: nothing
    // moves, and any challenge that was building is reset.
    if top == holder || top_stock < held + FLIP_MARGIN {
        if let Some(a) = w.statecraft.alignment.iter_mut().find(|a| a.client == client) {
            a.challenger = None;
            a.challenge_months = 0;
        }
        return;
    }

    let months = {
        let Some(a) = w.statecraft.alignment.iter_mut().find(|a| a.client == client) else {
            return;
        };
        if a.challenger == Some(top) {
            a.challenge_months += 1;
        } else {
            a.challenger = Some(top);
            a.challenge_months = 1;
        }
        a.challenge_months
    };
    if months == 1 {
        w.headline(format!(
            "{} is drifting: {} now outweighs {} in {}.",
            client.name(),
            top.name(),
            holder.name(),
            client.name()
        ));
    }
    if months < FLIP_MONTHS {
        return;
    }

    // The flip. Everything below is the price of it.
    if let Some(a) = w.statecraft.alignment.iter_mut().find(|a| a.client == client) {
        a.patron = top;
        a.since_year = w.year;
        a.since_month = w.month;
        a.challenger = None;
        a.challenge_months = 0;
    }
    if let Some(s) = stake_mut(w, holder, client) {
        s.stock *= LOSS_ON_FLIP;
    }
    if let Some(s) = stake_mut(w, top, client) {
        s.stock = (s.stock + 8.0).min(100.0);
    }
    w.shift_relation(holder, client, -22.0);
    w.shift_relation(top, client, 14.0);
    {
        let n = w.nation_mut(holder);
        n.political_capital = (n.political_capital - FLIP_PC_COST).max(0.0);
    }
    w.headline(format!(
        "{} LEAVES {}'s SPHERE FOR {}'s. The mission is expelled and the contracts torn up.",
        client.name(),
        holder.name(),
        top.name()
    ));
}

fn drop_alignment(w: &mut WorldState, client: NationId, current: Option<NationId>) {
    if let Some(p) = current {
        w.statecraft.alignment.retain(|a| a.client != client);
        w.headline(format!(
            "{} drifts out of {}'s orbit; nobody is paying for it any more.",
            client.name(),
            p.name()
        ));
    }
}

// ---------------------------------------------------------------------------
// The board in January 1990
// ---------------------------------------------------------------------------

/// The opening spheres, and where they come from.
///
/// A player who sits down in January 1990 must be able to see where their
/// influence stands on the first screen. An empty board would be a lie about the
/// most heavily spheres-of-influence year in modern history.
///
/// THIS IS NOT A SECOND TRANSCRIPTION AND IT INVENTS NOTHING. It is a reading of
/// the one that already exists: `data/relations_1990.json` states, with sources,
/// that the Soviet Union stood at +85 with Afghanistan, +80 with Angola, +75
/// with Ethiopia and +70 with Cuba, and that the United States stood at +80 with
/// Israel, +65 with South Korea and the Philippines and +60 with Egypt. Those
/// numbers ARE the sphere map of 1990; nobody has to type it twice. Every stake
/// below is that number, run through one formula, with no per-country term and
/// no exceptions — the moment this file needs a special case named after a
/// country, BIBLE section 7 says the model is wrong.
///
/// Two things the formula says out loud:
///
/// * Warmth below +30 is diplomacy, not patronage. A relation has to be an
///   alignment before it counts as one.
/// * A rich country is not HELD, it agrees. Canada and Japan sit inside the
///   American system and always did, but they are not clients in the sense
///   Angola was, and the same wealth term the rest of the module uses says so.
///
/// Everything after this is the model's problem. Nothing is pinned in place: the
/// stakes decay from the first tick like any other, so a superpower that stops
/// paying for Angola loses Angola, and both superpowers open the game holding
/// more than their politics can afford — which is, as it happens, what happened.
pub fn seat_1990(w: &mut WorldState) {
    for p in patrons().iter().copied() {
        if w.nation_opt(p).map_or(true, |n| !n.alive) {
            continue;
        }
        let clients: Vec<NationId> = all_nations()
            .iter()
            .copied()
            .filter(|c| *c != p && can_be_client(*c))
            .filter(|c| w.nation_opt(*c).map_or(false, |n| n.alive))
            .collect();
        for c in clients {
            let rel = w.relation(p, c);
            if rel < 30.0 {
                continue;
            }
            let n = w.nation(c);
            let wealth = (n.gdp * 1000.0 / n.population.max(0.1) / 20000.0).clamp(0.0, 1.0);
            let stock = ((rel - 30.0) * 1.7).min(90.0) * (1.0 - 0.55 * wealth);
            if stock < ALIGN_FLOOR * 0.8 {
                continue;
            }
            w.statecraft.influence.push(Stake { patron: p, client: c, stock, effort: 0.0 });
        }
    }
    // Settle who leans where before the first month, so the opening briefing is
    // the world as it stood and not the world as it will be in February.
    let clients: Vec<NationId> = {
        let mut v: Vec<NationId> = w.statecraft.influence.iter().map(|s| s.client).collect();
        v.sort();
        v.dedup();
        v
    };
    for c in clients {
        settle_alignment(w, c);
    }
    // The headlines above belong to a month that has not started yet.
    w.headlines.clear();
}

// ---------------------------------------------------------------------------
// Commands
// ---------------------------------------------------------------------------

/// Open, widen or narrow a standing programme of influence. The effort is a
/// posture, not a purchase: it costs political capital every month it stands and
/// it stops paying the moment it is withdrawn.
pub fn project(
    w: &mut WorldState,
    patron: NationId,
    client: NationId,
    effort: f64,
) -> Result<(), String> {
    if patron == client {
        return Err("A nation does not project influence into itself.".into());
    }
    if w.nation_opt(patron).map_or(true, |n| !n.alive) || w.nation_opt(client).map_or(true, |n| !n.alive) {
        return Err("Nation no longer exists.".into());
    }
    if crate::statecraft::belligerents(w, patron, client) {
        return Err("You are at war with them. Influence is not the instrument.".into());
    }
    if !can_be_client(client) {
        return Err(format!(
            "{} is a great power in its own right. It is courted, not held.",
            client.name()
        ));
    }
    let effort = effort.clamp(0.0, 1.0);
    if effort <= 0.0 {
        return abandon(w, patron, client);
    }
    let before = w.effort(patron, client);
    match stake_mut(w, patron, client) {
        Some(s) => s.effort = effort,
        None => w.statecraft.influence.push(Stake {
            patron,
            client,
            stock: 0.0,
            effort,
        }),
    }
    let held = w.stake(patron, client);
    let rival = w.contested_by(patron, client);
    if before <= 0.0 {
        w.headline(match rival {
            Some((q, _)) => format!(
                "{} opens a mission in {} — where {} is already at work.",
                patron.name(),
                client.name(),
                q.name()
            ),
            None => format!(
                "{} begins courting {} in earnest.",
                patron.name(),
                client.name()
            ),
        });
    } else if effort > before + 0.15 {
        w.headline(format!(
            "{} redoubles its efforts in {} (holding {:.0}).",
            patron.name(),
            client.name(),
            held
        ));
    }
    Ok(())
}

/// Walk away. Always available, never free: the stake collapses at once rather
/// than decaying, because withdrawing is itself an announcement.
pub fn abandon(w: &mut WorldState, patron: NationId, client: NationId) -> Result<(), String> {
    if w.stake(patron, client) <= 0.0 && w.effort(patron, client) <= 0.0 {
        return Err("There is no sphere here to give up.".into());
    }
    let was_holder = w.aligned_to(client) == Some(patron);
    if let Some(s) = stake_mut(w, patron, client) {
        s.effort = 0.0;
        s.stock *= 0.5;
    }
    w.shift_relation(patron, client, -10.0);
    if was_holder {
        w.headline(format!(
            "{} abandons {} to its own devices. The orbit is open.",
            patron.name(),
            client.name()
        ));
    } else {
        w.headline(format!(
            "{} gives up its efforts in {}.",
            patron.name(),
            client.name()
        ));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// The moments the instruments actually move the stock
// ---------------------------------------------------------------------------

/// A discrete shove, used by `statecraft.rs` at the moments an instrument does
/// something rather than merely stands there: a covert operation lands, a client
/// is cut off, a guarantee is renounced, a market is closed.
pub fn shift(w: &mut WorldState, patron: NationId, client: NationId, d: f64) {
    if patron == client || !can_be_client(client) {
        return;
    }
    match stake_mut(w, patron, client) {
        Some(s) => s.stock = (s.stock + d).clamp(0.0, 100.0),
        None => {
            if d > 0.0 {
                w.statecraft.influence.push(Stake {
                    patron,
                    client,
                    stock: d.min(100.0),
                    effort: 0.0,
                });
            }
        }
    }
}

/// Subversion, as an influence instrument rather than a mood. A successful
/// operation inside a country that leans to one of the sponsor's enemies does
/// not build the sponsor's own position much — it wrecks the rival's. That
/// asymmetry is why covert action is the cheap way to break a sphere and a poor
/// way to build one.
pub fn subversion_landed(w: &mut WorldState, sponsor: NationId, target: NationId) {
    let victims: Vec<NationId> = w
        .stakeholders(target)
        .into_iter()
        .map(|(q, _)| q)
        .filter(|q| *q != sponsor && w.relation(sponsor, *q) < -15.0)
        .collect();
    for q in victims {
        shift(w, q, target, -6.0);
    }
    shift(w, sponsor, target, 2.0);
}

/// ...and the bill when it lands in the newspapers. Being caught costs the
/// sponsor its own standing in the country, which is the thing an exposed
/// operation actually destroys.
pub fn subversion_exposed(w: &mut WorldState, sponsor: NationId, target: NationId) {
    shift(w, sponsor, target, -9.0);
}

// ---------------------------------------------------------------------------
// The quote: what holding costs, and what taking would
// ---------------------------------------------------------------------------

/// What it would take for `challenger` to take `client` off whoever holds it.
/// Every field is derived from the same arithmetic `tick` runs, so the number
/// the player is shown is the number they will actually be charged.
#[derive(Clone, Debug, PartialEq)]
pub struct TakeQuote {
    pub client: NationId,
    pub holder: Option<NationId>,
    pub their_stock: f64,
    pub my_stock: f64,
    /// The stake a challenger has to reach: the incumbent's, plus the margin.
    pub target_stock: f64,
    /// Net points a month at full effort, with the instruments currently in
    /// force and the decay this stake is actually suffering. Negative means
    /// diplomacy alone cannot do it.
    pub net_per_month: f64,
    /// Months of full effort to get there, once the lead has to be HELD for
    /// `FLIP_MONTHS` on top. `None` when it is not reachable this way.
    pub months: Option<u32>,
    /// ...and what those months cost in political capital, plus the one-off
    /// price of opening the programme. The headline number.
    pub political_capital: Option<f64>,
    /// True when the arithmetic says no: raise an instrument or forget it.
    pub needs_an_instrument: bool,
}

pub fn quote_take(w: &WorldState, challenger: NationId, client: NationId) -> TakeQuote {
    let holder = w.aligned_to(client).filter(|h| *h != challenger);
    let their_stock = holder.map(|h| w.stake(h, client)).unwrap_or(0.0);
    let my_stock = w.stake(challenger, client);
    // An unheld client only has to be brought over the alignment floor; a held
    // one has to be beaten by the margin, and then held there.
    let target_stock = match holder {
        Some(_) => their_stock + FLIP_MARGIN,
        None => ALIGN_FLOOR,
    };

    let gain_full = (PROJECTION_PER_EFFORT + instruments(w, challenger, client))
        * receptivity(w, challenger, client)
        * reach(w, challenger, client);
    // Decay evaluated at the stake it is trying to reach, which is the honest
    // place: the closer you get the harder the last point is.
    let incumbent = w.aligned_to(client) == Some(challenger);
    let pressure = w.contest_pressure(challenger, client);
    let decay = BASE_DECAY * if incumbent { 1.0 - INCUMBENT_RELIEF } else { 1.0 }
        + CONTEST_DECAY * pressure;
    let net = gain_full - decay * target_stock;

    let gap = (target_stock - my_stock).max(0.0);
    let months: Option<u32> = if gap <= 0.0 {
        // Already there on points; only the six months of holding the lead
        // remain, and against an unheld capital not even that.
        Some(if holder.is_some() { FLIP_MONTHS } else { 0 })
    } else if net <= 0.005 {
        None
    } else {
        Some((gap / net).ceil() as u32 + if holder.is_some() { FLIP_MONTHS } else { 0 })
    };
    let monthly =
        (UPKEEP_PC_PER_POINT * target_stock + UPKEEP_PC_EFFORT) * (1.0 + pressure);
    let political_capital =
        months.map(|m| m as f64 * monthly + open_price(w, challenger, client, 1.0));

    TakeQuote {
        client,
        holder,
        their_stock,
        my_stock,
        target_stock,
        net_per_month: net,
        months,
        political_capital,
        needs_an_instrument: months.is_none(),
    }
}

/// The one-off price of opening or widening a programme, charged by
/// `command_price` in lib.rs and quoted here so the two cannot drift apart.
/// Winding one down is charged nothing here — `AbandonSphere` has its own price.
pub fn open_price(w: &WorldState, patron: NationId, client: NationId, effort: f64) -> f64 {
    let before = w.effort(patron, client);
    let widening = effort.clamp(0.0, 1.0) - before;
    if widening <= 0.0 {
        // Giving ground is the cheap direction, which is precisely why spheres
        // are lost by governments that ran out of standing at home rather than
        // by governments that were outbid.
        return 0.0;
    }
    widening * 14.0 + 2.0
}
