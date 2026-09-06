//! The political arm's lens — stage S1 of "The Political Arm of SPHERES",
//! revision 2, approved by Ridge on 2026-09-05.
//!
//! Everything here is a PURE READ of the world: bloc shares, the ruling bloc,
//! the discontent gauge, influence, and the takeover watch. Nothing in this file
//! writes state or draws the RNG, and every function answers with the switch
//! off as well as on, so the browser can call them on any world and a test can
//! prove they leave the timeline alone. The one write the arm makes in this
//! build — seeding a regime's movements from the leader row — lives beside the
//! other seating code in `government::seed_blocs`, and calls back into
//! [`flat_seed`] here for the arithmetic.
//!
//! What is NOT here, and where it is filed: the roads to power as mechanics,
//! movement drift, the Ban and Back commands, the census, and foreign backing
//! (S3-S5). Their READOUTS are here, closed, so the screen that lands in S2 has
//! every gauge to draw from the first day.

use crate::data::{Office, Tie};
use crate::government::{self, bloc_of, pillar_bloc, polity, Bloc, GovState, Pillar};
use crate::world::*;
use serde::Serialize;

/// Shares below this are floored before a regime's seed is normalised, so an
/// absent bloc is a rounding error rather than an impossibility.
pub const SHARE_FLOOR: f64 = 0.002;

/// The ruling bloc's opening share of a regime's movements.
pub const RULING_SEED: f64 = 0.60;

// ---------------------------------------------------------------------------
// The leader row
// ---------------------------------------------------------------------------

/// The leader table's row for this nation, if the arm is on and one was
/// transcribed. The 23 successor states have none by design (D2): they are
/// described from birth.
pub fn leader_row(w: &WorldState, id: NationId) -> Option<&Office> {
    w.leadership.as_ref()?.iter().find(|o| o.nation == id)
}

/// The bloc the leader row puts in power: a sourced `bloc_override` first
/// (Sudan's Bashir heads the army and governs as an Islamist), else the bloc of
/// the party the leader is tied to, else the installing bloc of the pillar they
/// head. `None` where there is no row.
pub fn leader_bloc(w: &WorldState, id: NationId) -> Option<Bloc> {
    let row = leader_row(w, id)?;
    if let Some(b) = row.bloc_override {
        return Some(b);
    }
    Some(match &row.tie {
        Tie::Party(p) => bloc_of(id, p),
        Tie::Pillar(pl) => pillar_bloc(id, *pl),
    })
}

/// The ruling bloc of a regime as the table describes it, without reading any
/// stored state: the leader row where there is one, else — a successor state,
/// or a nation the table has not reached — the bloc of the largest party in
/// its dormant table, else the installing bloc of the first pillar in its
/// list. This is what `government::seed_blocs` writes into `regime_bloc`, and
/// what `ruling_bloc` falls back to when nothing was stored.
pub fn described_ruling_bloc(w: &WorldState, id: NationId) -> Option<Bloc> {
    if let Some(b) = leader_bloc(w, id) {
        return Some(b);
    }
    let pol = polity(id)?;
    let mut best: Option<&government::PartySpec> = None;
    for s in pol.parties {
        if best.map_or(true, |b| s.start > b.start) {
            best = Some(s);
        }
    }
    if let Some(s) = best {
        return Some(bloc_of(id, s.id));
    }
    pol.pillars.first().map(|s| pillar_bloc(id, s.pillar))
}

// ---------------------------------------------------------------------------
// Shares
// ---------------------------------------------------------------------------

fn zero_shares() -> [(Bloc, f64); 5] {
    [
        (Bloc::Western, 0.0),
        (Bloc::Communist, 0.0),
        (Bloc::Nationalist, 0.0),
        (Bloc::Islamist, 0.0),
        (Bloc::NonAligned, 0.0),
    ]
}

fn slot(shares: &mut [(Bloc, f64); 5], bloc: Bloc) -> &mut f64 {
    &mut shares[bloc as usize].1
}

/// Whether a bloc has any standing presence in a polity's transcribed shape: a
/// party of that bloc in the (possibly dormant) table, or the bloc's installing
/// pillar in the pillar list. Foreign backing as a presence is S3.
pub fn bloc_present(id: NationId, bloc: Bloc) -> bool {
    let pol = match polity(id) {
        Some(p) => p,
        None => return false,
    };
    pol.parties.iter().any(|s| bloc_of(id, s.id) == bloc)
        || pol.pillars.iter().any(|s| pillar_bloc(id, s.pillar) == bloc)
}

/// The flat seed of a regime's movements (design D5): the ruling bloc at
/// [`RULING_SEED`], the remainder split equally over the non-ruling blocs
/// present in the polity, every bloc floored at [`SHARE_FLOOR`], normalised to
/// one. Pure: the same table and the same ruling bloc give the same seed.
pub fn flat_seed(id: NationId, ruling: Bloc) -> [(Bloc, f64); 5] {
    let mut shares = zero_shares();
    let present: Vec<Bloc> =
        Bloc::ALL.iter().copied().filter(|b| *b != ruling && bloc_present(id, *b)).collect();
    *slot(&mut shares, ruling) = RULING_SEED;
    if !present.is_empty() {
        let each = (1.0 - RULING_SEED) / present.len() as f64;
        for b in present {
            *slot(&mut shares, b) = each;
        }
    }
    for e in shares.iter_mut() {
        e.1 = e.1.max(SHARE_FLOOR);
    }
    normalise(&mut shares);
    shares
}

fn normalise(shares: &mut [(Bloc, f64); 5]) {
    let total: f64 = shares.iter().map(|(_, v)| *v).sum();
    if total > 0.0 {
        for e in shares.iter_mut() {
            e.1 /= total;
        }
    }
}

/// S_B for every bloc, in enum order, summing to one for every nation that has
/// a government. Electoral: the sum of party support over the parties of each
/// bloc — nothing stored, so a vote and a drift move it live. Regime: the
/// stored movements, or the flat seed the arm would write if it has not been
/// switched on, so the readout is the same either way.
pub fn bloc_shares(w: &WorldState, id: NationId) -> [(Bloc, f64); 5] {
    let g = match government::state(w, id) {
        Some(g) => g,
        None => return zero_shares(),
    };
    if government::is_electoral(w, id) {
        let mut shares = zero_shares();
        for (party, s) in &g.support {
            *slot(&mut shares, bloc_of(id, party)) += *s;
        }
        // Support is normalised at seating and after every drift; this only
        // guards a hand-edited save.
        normalise(&mut shares);
        return shares;
    }
    if g.movements.len() == 5 {
        let mut shares = zero_shares();
        for (b, v) in &g.movements {
            *slot(&mut shares, *b) = *v;
        }
        normalise(&mut shares);
        return shares;
    }
    match described_ruling_bloc(w, id) {
        Some(r) => flat_seed(id, r),
        None => zero_shares(),
    }
}

/// The authoritarianism at and above which a crowned head who appoints the
/// government outranks the chamber he lets sit. Jordan in 1990: a Chamber of
/// Deputies elected two months earlier, and a King who chose the prime
/// minister without reference to it.
pub const COURT_RULES_ABOVE: f64 = 0.40;

/// The monarchy exception (design, ruling bloc): in an electoral polity whose
/// leader row ties to a PILLAR, that pillar rules while authoritarianism is at
/// or above [`COURT_RULES_ABOVE`]. Returns the pillar when it applies.
fn court_pillar(w: &WorldState, id: NationId) -> Option<Pillar> {
    if !government::is_electoral(w, id) {
        return None;
    }
    let row = leader_row(w, id)?;
    match &row.tie {
        Tie::Pillar(pl) if w.nation(id).authoritarianism >= COURT_RULES_ABOVE => Some(*pl),
        _ => None,
    }
}

/// Who holds power, as a bloc. Electoral: the coalition leader's bloc,
/// recomputed from whatever `form_government` last wrote — except under the
/// monarchy exception, where it is the court pillar's. Regime: the stored
/// `regime_bloc`, else the described one.
pub fn ruling_bloc(w: &WorldState, id: NationId) -> Option<Bloc> {
    let g = government::state(w, id)?;
    if government::is_electoral(w, id) {
        if let Some(pl) = court_pillar(w, id) {
            return leader_row(w, id)
                .and_then(|r| r.bloc_override)
                .or(Some(pillar_bloc(id, pl)));
        }
        return g.leader().map(|p| bloc_of(id, p));
    }
    g.regime_bloc.or_else(|| described_ruling_bloc(w, id))
}

/// Under the monarchy exception only: the party leading the chamber, which
/// governs day to day under a court that can dismiss it. `None` everywhere
/// else, so the surface can tell the two cases apart.
pub fn government_of_the_day(w: &WorldState, id: NationId) -> Option<String> {
    court_pillar(w, id)?;
    government::state(w, id)?.leader().map(|s| s.to_string())
}

// ---------------------------------------------------------------------------
// Discontent and influence
// ---------------------------------------------------------------------------

/// How close the country is to acting against its government, 0..1: half of it
/// is order, a fifth each prices and growth, a tenth the war. Read off the same
/// `pains` the party model moves support with, so the gauge and the electorate
/// cannot disagree. Worked values, measured in `tests`: stability 40 alone
/// reads 0.1667; stability 25 alone reads 0.2917.
pub fn discontent(w: &WorldState, id: NationId) -> f64 {
    if w.nation_opt(id).is_none() {
        return 0.0;
    }
    let p = government::pains(w, id);
    (0.50 * p.order.min(1.0) + 0.20 * p.prices + 0.20 * p.growth + 0.10 * p.war).clamp(0.0, 1.0)
}

/// F_B: foreign backing of each bloc in this state, summed from
/// `Statecraft.backing`. Empty in this build (S3 fills it), so this reads zero
/// everywhere — served rather than omitted so the surface is complete.
pub fn backing(w: &WorldState, id: NationId) -> [(Bloc, f64); 5] {
    let mut out = zero_shares();
    for b in &w.statecraft.backing {
        if b.target == id {
            *slot(&mut out, b.bloc) += b.weight;
        }
    }
    out
}

/// I_B = S_B + F_B, per bloc in enum order. Not normalised: backing is added
/// on top of a share, which is the point of it.
pub fn influence(w: &WorldState, id: NationId) -> [(Bloc, f64); 5] {
    let shares = bloc_shares(w, id);
    let back = backing(w, id);
    let mut out = zero_shares();
    for i in 0..5 {
        out[i].1 = shares[i].1 + back[i].1;
    }
    out
}

/// The strongest bloc other than the ruling one, by influence, ties broken in
/// enum order. `None` where nothing rules.
pub fn strongest_challenger(w: &WorldState, id: NationId) -> Option<(Bloc, f64)> {
    let ruling = ruling_bloc(w, id)?;
    let mut best: Option<(Bloc, f64)> = None;
    for (b, v) in influence(w, id) {
        if b == ruling {
            continue;
        }
        if best.map_or(true, |(_, bv)| v > bv) {
            best = Some((b, v));
        }
    }
    best
}

// ---------------------------------------------------------------------------
// The takeover watch
// ---------------------------------------------------------------------------

/// Which way a gauge has to move to arm its road.
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub enum Sense {
    /// The road needs `value >= trigger`.
    Above,
    /// The road needs `value <= trigger`.
    Below,
    /// The road needs `trigger.0 <= value <= trigger.1`; `trigger` carries the
    /// lower bound and `upper` the upper.
    Inside,
}

/// One reading on the takeover watch.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Gauge {
    pub name: &'static str,
    pub value: f64,
    pub trigger: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper: Option<f64>,
    pub sense: Sense,
    /// The bloc this gauge names, where it names one (the uprising's
    /// challenger).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bloc: Option<Bloc>,
}

impl Gauge {
    fn above(name: &'static str, value: f64, trigger: f64) -> Gauge {
        Gauge { name, value, trigger, upper: None, sense: Sense::Above, bloc: None }
    }
    fn below(name: &'static str, value: f64, trigger: f64) -> Gauge {
        Gauge { name, value, trigger, upper: None, sense: Sense::Below, bloc: None }
    }
    fn inside(name: &'static str, value: f64, lo: f64, hi: f64) -> Gauge {
        Gauge { name, value, trigger: lo, upper: Some(hi), sense: Sense::Inside, bloc: None }
    }
    /// How far along the gauge is toward its trigger, 0 at rest and 1 at the
    /// trigger, so the map can hatch a nation whose watch is half-armed
    /// whichever way its gauges run. `Below` gauges are read from 1.0 (a
    /// loyalty of 0.35 against a ceiling of 1.0 is the whole distance; a
    /// stability of 12 against 100 likewise). `Inside` is 1 inside the band
    /// and 0 outside.
    pub fn progress(&self) -> f64 {
        match self.sense {
            Sense::Above => {
                if self.trigger <= 0.0 {
                    1.0
                } else {
                    (self.value / self.trigger).max(0.0)
                }
            }
            Sense::Below => {
                let ceiling = if self.trigger > 1.0 { 100.0 } else { 1.0 };
                let span = ceiling - self.trigger;
                if span <= 0.0 {
                    1.0
                } else {
                    ((ceiling - self.value) / span).max(0.0)
                }
            }
            Sense::Inside => {
                let hi = self.upper.unwrap_or(self.trigger);
                if self.value >= self.trigger && self.value <= hi {
                    1.0
                } else {
                    0.0
                }
            }
        }
    }
    /// Whether the gauge is at or past its trigger.
    pub fn met(&self) -> bool {
        match self.sense {
            Sense::Above => self.value >= self.trigger,
            Sense::Below => self.value <= self.trigger,
            Sense::Inside => self.progress() >= 1.0,
        }
    }
}

/// One road to power on the watch.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Road {
    pub gauges: Vec<Gauge>,
    /// Whether the road can be taken. FALSE ON EVERY ROAD IN THIS BUILD: the
    /// roads are S4 and `rules.ideology_takeover` is off everywhere.
    pub open: bool,
    pub reason: &'static str,
}

impl Road {
    /// Whether every gauge is at its trigger — what `open` would read if the
    /// build were S4. Served so the screen can say "would be open" honestly.
    pub fn armed(&self) -> bool {
        !self.gauges.is_empty() && self.gauges.iter().all(|g| g.met())
    }
    /// Whether any gauge is at or past half its trigger — the map's hatch.
    pub fn half_armed(&self) -> bool {
        self.gauges.iter().any(|g| g.progress() >= 0.5)
    }
}

/// The four roads, read for one nation.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TakeoverReadout {
    pub coup: Road,
    pub uprising: Road,
    pub round_table: Road,
    pub collapse: Road,
}

impl TakeoverReadout {
    pub fn roads(&self) -> [&Road; 4] {
        [&self.coup, &self.uprising, &self.round_table, &self.collapse]
    }
    /// Any gauge on any road at or past half its trigger.
    pub fn half_armed(&self) -> bool {
        self.roads().iter().any(|r| r.half_armed())
    }
}

/// The one reason every road gives in this build.
pub const NOT_IN_THIS_BUILD: &str = "not in this build";

/// The takeover watch: per road, the gauges the S4 mechanics will read and
/// whether the road is open. In this build every road is closed with
/// [`NOT_IN_THIS_BUILD`] because `rules.ideology_takeover` is false and S4 has
/// not landed; the numbers are served so the screen can show them.
///
/// Triggers, from the design: coup — army loyalty at or under 0.35, discontent
/// at or over 0.25, coup pressure at or over 1.0 / crisis_intensity; uprising —
/// discontent at or over 0.45 and the strongest non-ruling bloc's influence at
/// or over 0.45; round table — Western influence at or over 0.40, Party loyalty
/// at or under 0.55, stability inside 30..70; collapse — stability at or under
/// 12.
pub fn takeover_readout(w: &WorldState, id: NationId) -> TakeoverReadout {
    let n = w.nation(id);
    let g: Option<&GovState> = government::state(w, id);
    let loyalty = |p: Pillar| g.map_or(1.0, |g| g.loyalty(p));
    let pressure = g.map_or(0.0, |g| g.coup_pressure);
    let disc = discontent(w, id);
    let infl = influence(w, id);
    let western = infl[Bloc::Western as usize].1;
    let closed = |gauges: Vec<Gauge>| Road { gauges, open: false, reason: NOT_IN_THIS_BUILD };

    let coup = closed(vec![
        Gauge::below("army loyalty", loyalty(Pillar::Army), 0.35),
        Gauge::above("discontent", disc, 0.25),
        Gauge::above("coup pressure", pressure, 1.0 / w.rules.crisis_intensity.max(0.1)),
    ]);
    let mut challenger = Gauge::above("challenger influence", 0.0, 0.45);
    if let Some((b, v)) = strongest_challenger(w, id) {
        challenger.value = v;
        challenger.bloc = Some(b);
    }
    let uprising = closed(vec![Gauge::above("discontent", disc, 0.45), challenger]);
    let round_table = closed(vec![
        Gauge::above("Western influence", western, 0.40),
        Gauge::below("party loyalty", loyalty(Pillar::Party), 0.55),
        Gauge::inside("stability", n.stability, 30.0, 70.0),
    ]);
    let collapse = closed(vec![Gauge::below("stability", n.stability, 12.0)]);
    TakeoverReadout { coup, uprising, round_table, collapse }
}
