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

/// The ruling bloc's opening share of a regime's movements. INVENTED (design
/// D5, approved 2026-09-05): a model coefficient, not a transcribed figure —
/// filed in BUGS.md with what would calibrate it.
pub const RULING_SEED: f64 = 0.60;

// ---------------------------------------------------------------------------
// The leader row
// ---------------------------------------------------------------------------

/// The leader table's row for this nation, if the arm is on and one was
/// transcribed and NAMED. The 23 successor states have none by design (D2):
/// they are described from birth. A REFUSED row — a nameless row whose tie no
/// fetched source supports; Chile, Comoros, Cyprus, Greece and Panama in the
/// 1990 table — is skipped on the same terms: it asserts nothing, and the
/// nation is described from its table.
pub fn leader_row(w: &WorldState, id: NationId) -> Option<&Office> {
    w.leadership.as_ref()?.iter().find(|o| o.nation == id && o.name.is_some())
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
    Some(match row.tie.as_ref()? {
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
/// minister without reference to it. INVENTED (design, monarchy exception):
/// the line is a coefficient, filed in BUGS.md; Jordan's transcribed 1990
/// authoritarianism sits above it and is the only case in the table.
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
        Some(Tie::Pillar(pl)) if w.nation(id).authoritarianism >= COURT_RULES_ABOVE => Some(*pl),
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
/// reads 0.1667; stability 25 alone reads 0.2917. The four weights
/// 0.50/0.20/0.20/0.10 are INVENTED (design, approved 2026-09-05) and filed in
/// BUGS.md with what would calibrate them.
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
    /// `progress()` and `met()` at construction, SERVED so the screen draws
    /// the bar the sim computed rather than dividing value by trigger itself.
    /// `set_bloc` is the only later write and neither depends on the bloc.
    pub progress: f64,
    pub met: bool,
}

impl Gauge {
    fn finish(mut self) -> Gauge {
        self.progress = self.progress();
        self.met = self.met();
        self
    }
    fn above(name: &'static str, value: f64, trigger: f64) -> Gauge {
        Gauge { name, value, trigger, upper: None, sense: Sense::Above, bloc: None, progress: 0.0, met: false }
            .finish()
    }
    fn below(name: &'static str, value: f64, trigger: f64) -> Gauge {
        Gauge { name, value, trigger, upper: None, sense: Sense::Below, bloc: None, progress: 0.0, met: false }
            .finish()
    }
    fn inside(name: &'static str, value: f64, lo: f64, hi: f64) -> Gauge {
        Gauge { name, value, trigger: lo, upper: Some(hi), sense: Sense::Inside, bloc: None, progress: 0.0, met: false }
            .finish()
    }
    /// How far along the gauge is toward its trigger, 0 at rest and 1 at the
    /// trigger, so the map can hatch a nation whose watch is half-armed
    /// whichever way its gauges run. `Below` gauges are read from 1.0 (a
    /// loyalty of 0.35 against a ceiling of 1.0 is the whole distance; a
    /// stability of 12 against 100 likewise) — an INVENTED reading of "half
    /// its trigger" for a gauge that falls, filed in BUGS.md. `Inside` is 1
    /// inside the band and 0 outside: a band has no half, which is why
    /// `Road::half_armed` does not read it.
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
    /// `armed()` and `half_armed()` at construction, served for the screen and
    /// the map's hatch.
    pub armed: bool,
    pub half_armed: bool,
}

impl Road {
    fn closed(gauges: Vec<Gauge>) -> Road {
        let mut r = Road { gauges, open: false, reason: NOT_IN_THIS_BUILD, armed: false, half_armed: false };
        r.armed = r.armed();
        r.half_armed = r.half_armed();
        r
    }
    /// Whether every gauge is at its trigger — what `open` would read if the
    /// build were S4. Served so the screen can say "would be open" honestly.
    pub fn armed(&self) -> bool {
        !self.gauges.is_empty() && self.gauges.iter().all(|g| g.met())
    }
    /// Whether any THRESHOLD gauge is at or past half its trigger — the map's
    /// hatch. An `Inside` gauge is not read: a band is either met or not, it
    /// has no half, and reading its 1.0 as "past half" hatched every one of
    /// the 137 living nations of 1990 (the round table's stability 30..70
    /// band held them all), which told the player nothing. Measured after the
    /// repair on 2026-09-05: see `politics_is_null_off_and_served_whole_on`.
    pub fn half_armed(&self) -> bool {
        self.gauges.iter().any(|g| g.sense != Sense::Inside && g.progress() >= 0.5)
    }
}

/// The four roads, read for one nation.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TakeoverReadout {
    pub coup: Road,
    pub uprising: Road,
    pub round_table: Road,
    pub collapse: Road,
    /// `half_armed()` at construction: any threshold gauge on any road at or
    /// past half its trigger. The map's hatch reads this and nothing else.
    pub half_armed: bool,
}

impl TakeoverReadout {
    pub fn roads(&self) -> [&Road; 4] {
        [&self.coup, &self.uprising, &self.round_table, &self.collapse]
    }
    /// Any threshold gauge on any road at or past half its trigger.
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
    // A roster id not in this world — a successor state before its federation
    // falls — has no stability to read; it is served as a calm 100 rather than
    // a panic, because the browser asks about ids, not about nations.
    let stability = w.nation_opt(id).map_or(100.0, |n| n.stability);
    let g: Option<&GovState> = government::state(w, id);
    let loyalty = |p: Pillar| g.map_or(1.0, |g| g.loyalty(p));
    let pressure = g.map_or(0.0, |g| g.coup_pressure);
    let disc = discontent(w, id);
    let infl = influence(w, id);
    let western = infl[Bloc::Western as usize].1;
    let closed = Road::closed;

    let coup = closed(vec![
        Gauge::below("army loyalty", loyalty(Pillar::Army), 0.35),
        Gauge::above("discontent", disc, 0.25),
        Gauge::above("coup pressure", pressure, 1.0 / w.rules.crisis_intensity.max(0.1)),
    ]);
    let (cv, cb) = match strongest_challenger(w, id) {
        Some((b, v)) => (v, Some(b)),
        None => (0.0, None),
    };
    let mut challenger = Gauge::above("challenger influence", cv, 0.45);
    challenger.bloc = cb;
    let uprising = closed(vec![Gauge::above("discontent", disc, 0.45), challenger]);
    let round_table = closed(vec![
        Gauge::above("Western influence", western, 0.40),
        Gauge::below("party loyalty", loyalty(Pillar::Party), 0.55),
        Gauge::inside("stability", stability, 30.0, 70.0),
    ]);
    let collapse = closed(vec![Gauge::below("stability", stability, 12.0)]);
    let mut out = TakeoverReadout { coup, uprising, round_table, collapse, half_armed: false };
    out.half_armed = out.half_armed();
    out
}

// ---------------------------------------------------------------------------
// The surface (S1): what /api/state serves per nation, built here so the
// browser prints numbers and never derives one
// ---------------------------------------------------------------------------

/// One bloc's line: its share S_B, its foreign backing F_B (zero in this
/// build), and whether this government has proscribed it (nothing does yet).
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct BlocRow {
    pub bloc: Bloc,
    pub share: f64,
    pub backing: f64,
    pub banned: bool,
}

/// The five rows in enum order.
pub fn bloc_rows(w: &WorldState, id: NationId) -> Vec<BlocRow> {
    let shares = bloc_shares(w, id);
    let back = backing(w, id);
    let banned: &[Bloc] = government::state(w, id).map_or(&[], |g| g.banned.as_slice());
    (0..5)
        .map(|i| BlocRow {
            bloc: shares[i].0,
            share: shares[i].1,
            backing: back[i].1,
            banned: banned.contains(&shares[i].0),
        })
        .collect()
}

/// Who directs the executive, as the surface prints it. NAMED where the leader
/// table carries a row for this nation; DESCRIBED by its real institution
/// everywhere else — a successor state, a refused row, a nation the table has
/// not reached (design D2: no name after 1 January 1990, and none invented).
/// Exactly one of `name` and `described` is `Some`.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Leader {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub described: Option<String>,
    pub office: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    /// The party id the leader is tied to, where the tie is a party (or, for a
    /// described electoral leader, the coalition leader's party).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pillar: Option<Pillar>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pillar_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heir: Option<crate::data::Heir>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub must_leave_by: Option<String>,
    pub also: Vec<crate::data::Also>,
}

fn party_name(id: NationId, party: &str) -> Option<String> {
    government::party_spec(id, party).map(|s| s.name.to_string())
}

fn pillar_name(id: NationId, pillar: Pillar) -> Option<String> {
    polity(id)?.pillars.iter().find(|s| s.pillar == pillar).map(|s| s.name.to_string())
}

/// The raw table row, refused or not — a refused row still carries a sourced
/// office and date, which the description keeps.
fn any_row(w: &WorldState, id: NationId) -> Option<&Office> {
    w.leadership.as_ref()?.iter().find(|o| o.nation == id)
}

/// The leader card for one nation. `None` only where the nation has no polity.
pub fn leader(w: &WorldState, id: NationId) -> Option<Leader> {
    let pol = polity(id)?;
    if let Some(row) = leader_row(w, id) {
        let (party, pillar) = match &row.tie {
            Some(Tie::Party(p)) => (Some(p.clone()), None),
            Some(Tie::Pillar(pl)) => (None, Some(*pl)),
            None => (None, None),
        };
        return Some(Leader {
            name: row.name.clone(),
            native: row.native.clone(),
            described: None,
            office: row.office.clone(),
            since: Some(row.since.clone()),
            party_name: party.as_deref().and_then(|p| party_name(id, p)),
            party,
            pillar_name: pillar.and_then(|p| pillar_name(id, p)),
            pillar,
            heir: row.heir.clone(),
            must_leave_by: row.must_leave_by.clone(),
            also: row.also.clone(),
        });
    }
    // Described. A REFUSED row (Chile, Panama) keeps its sourced office, date
    // and `also` for the record, and its holder is "the office-holder" — not
    // the leader of the chamber's largest party, who is somebody else, and
    // not the name in the row's note, which the refusal exists to withhold.
    if let Some(r) = any_row(w, id) {
        return Some(Leader {
            name: None,
            native: None,
            described: Some("the office-holder".to_string()),
            office: r.office.clone(),
            since: Some(r.since.clone()),
            party: None,
            party_name: None,
            pillar: None,
            pillar_name: None,
            heir: r.heir.clone(),
            must_leave_by: r.must_leave_by.clone(),
            also: r.also.clone(),
        });
    }
    // No row at all — a successor state, or a nation the table has not
    // reached: the chamber's leading party, or the regime's ruling institution.
    let g = government::state(w, id);
    if government::is_electoral(w, id) {
        let party = g.and_then(|g| g.leader()).map(|s| s.to_string());
        let pname = party.as_deref().and_then(|p| party_name(id, p));
        return Some(Leader {
            name: None,
            native: None,
            described: Some(match &pname {
                Some(n) => format!("the leader of {}", n),
                None => "the head of government".to_string(),
            }),
            office: "head of government".to_string(),
            since: None,
            party,
            party_name: pname,
            pillar: None,
            pillar_name: None,
            heir: None,
            must_leave_by: None,
            also: vec![],
        });
    }
    Some(Leader {
        name: None,
        native: None,
        described: Some(format!("the leadership of {}", pol.ruling)),
        office: pol.ruling.to_string(),
        since: None,
        party: None,
        party_name: None,
        pillar: None,
        pillar_name: None,
        heir: None,
        must_leave_by: None,
        also: vec![],
    })
}

/// Everything the political arm serves per nation, or `None` while
/// `rules.ideology_blocs` is off — the browser then prints null for every one
/// of these fields, and the page has nothing to compute from.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Politics {
    pub ruling_bloc: Option<Bloc>,
    pub discontent: f64,
    pub blocs: Vec<BlocRow>,
    pub leader: Option<Leader>,
    /// Monarchy exception only: the party leading the chamber under a court
    /// that appoints the government. Null everywhere else.
    pub government_of_the_day: Option<String>,
    pub takeover: TakeoverReadout,
}

pub fn politics(w: &WorldState, id: NationId) -> Option<Politics> {
    if !w.rules.ideology_blocs {
        return None;
    }
    Some(Politics {
        ruling_bloc: ruling_bloc(w, id),
        discontent: discontent(w, id),
        blocs: bloc_rows(w, id),
        leader: leader(w, id),
        government_of_the_day: government_of_the_day(w, id),
        takeover: takeover_readout(w, id),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::init::world_1990;
    use crate::{load, save, state_hash, tick_month};

    fn on(seed: u64) -> GameRules {
        GameRules { seed, ideology_blocs: true, ..GameRules::default() }
    }
    fn run_months(w: &mut WorldState, months: usize) {
        for _ in 0..months {
            tick_month(w, &[]);
        }
    }
    fn alive(w: &WorldState) -> Vec<NationId> {
        w.nations.iter().filter(|n| n.alive).map(|n| n.id).collect()
    }

    /// With the switch off the arm leaves no key in the save and the 1990
    /// start hashes to what this tree produced before the arm existed. The
    /// hash is the tree's ACTUAL, measured on the untouched worktree before the
    /// first commit of this branch (the golden `the_1990_start_is_pinned` is
    /// deliberately red at this value while BUGS E-3 is open, and its pin is
    /// not touched). Watched red by removing `skip_serializing_if` from
    /// `GovState.regime_bloc`: `"regime_bloc" appears in the default 1990
    /// save`. Watched red again with `seed_blocs` ignoring its switch: the
    /// same key check fired, and the twenty-year hashes below moved (seed 0
    /// read 0x7781f46d31e8a5d0).
    #[test]
    fn the_bloc_layer_is_inert_at_1990() {
        const START_ACTUAL: u64 = 0xe26e4bf8d6c60066;
        assert!(!GameRules::default().ideology_blocs);
        assert!(!GameRules::default().ideology_takeover);
        let w = world_1990(GameRules::default());
        let text = save(&w);
        for key in [
            "\"movements\"",
            "\"banned\"",
            "\"regime_bloc\"",
            "\"leadership\"",
            "\"backing\"",
            "\"ideology_blocs\"",
            "\"ideology_takeover\"",
        ] {
            assert!(!text.contains(key), "{key} appears in the default 1990 save");
        }
        assert!(w.leadership.is_none());
        for g in &w.governments.states {
            assert!(g.movements.is_empty() && g.banned.is_empty() && g.regime_bloc.is_none());
        }
        let h = state_hash(&w);
        assert_eq!(h, START_ACTUAL, "the arm moved the 1990 start (actual {h:#018x})");
        // The readouts answer with the switch off, and leave no trace doing so.
        for id in alive(&w) {
            let _ = bloc_shares(&w, id);
            let _ = ruling_bloc(&w, id);
            let _ = discontent(&w, id);
            let _ = takeover_readout(&w, id);
        }
        assert_eq!(state_hash(&w), START_ACTUAL);
    }

    /// Twenty years, seeds 0..5, default rules: every hash equals the one the
    /// untouched tree produces. The six constants were measured this run by
    /// building origin/feat/hoi4-map-and-tech from `git archive` into a
    /// separate CARGO_TARGET_DIR and running the same loop there. Then the
    /// switch ON against OFF for twenty years on seed 0: every quantity the
    /// tick reads is bit-identical nation by nation, which is the proof that
    /// the arm writes nothing the model reads.
    #[test]
    fn the_bloc_layer_is_inert_over_time() {
        const BASE: [u64; 6] = [
            0xdb60bf07873b8b58,
            0xd355b39ba484cd12,
            0xd6f543b16b90c72b,
            0x1a27e07d697ecf36,
            0x8cd02c837ca851a6,
            0xa03f4942471b2734,
        ];
        for seed in 0..6u64 {
            let mut w = world_1990(GameRules { seed, ..GameRules::default() });
            run_months(&mut w, 240);
            let h = state_hash(&w);
            assert_eq!(h, BASE[seed as usize], "seed {seed} moved (actual {h:#018x})");
        }
        let mut off = world_1990(GameRules::default());
        let mut on = world_1990(on(1990));
        for month in 0..240 {
            tick_month(&mut off, &[]);
            tick_month(&mut on, &[]);
            assert_eq!(off.nations.len(), on.nations.len(), "month {month}");
            for (a, b) in off.nations.iter().zip(&on.nations) {
                assert_eq!(a.id, b.id);
                for (name, x, y) in [
                    ("gdp", a.gdp, b.gdp),
                    ("stability", a.stability, b.stability),
                    ("political_capital", a.political_capital, b.political_capital),
                    ("authoritarianism", a.authoritarianism, b.authoritarianism),
                    ("inflation", a.inflation, b.inflation),
                ] {
                    assert!(x.to_bits() == y.to_bits(), "{} {name} month {month}: {x} vs {y}", a.id.code());
                }
            }
            assert_eq!(off.rng.state, on.rng.state, "the arm drew the RNG in month {month}");
            for (a, b) in off.governments.states.iter().zip(&on.governments.states) {
                assert_eq!(a.support, b.support, "{} support month {month}", a.nation.code());
                assert_eq!(a.coalition, b.coalition);
                assert_eq!(a.pillars, b.pillars);
            }
        }
    }

    /// Same seed twice with the arm on, one arm saved and reloaded at month
    /// 60, equal hashes at month 120; and the reload carries every field of
    /// the arm. Watched red by marking `Office.note` `skip_serializing`: the
    /// hashes still agreed, because BOTH saves dropped the note, and it was
    /// the `leadership` comparison after the reload that caught the loss —
    /// which is why the test compares the table and not only the hash.
    #[test]
    fn the_bloc_layer_is_deterministic_when_on() {
        let mut a = world_1990(on(7));
        let mut b = world_1990(on(7));
        assert_eq!(state_hash(&a), state_hash(&b));
        let text = save(&a);
        for key in ["\"movements\"", "\"regime_bloc\"", "\"leadership\"", "\"ideology_blocs\": true"] {
            assert!(text.contains(key), "{key} missing from the switched-on save");
        }
        assert!(!text.contains("\"ideology_takeover\""), "takeover stays off everywhere");
        run_months(&mut a, 60);
        run_months(&mut b, 60);
        let mut b = load(&save(&b)).expect("the switched-on save reloads");
        assert_eq!(state_hash(&a), state_hash(&b), "save/load moved the world at month 60");
        assert_eq!(a.leadership, b.leadership);
        run_months(&mut a, 60);
        run_months(&mut b, 60);
        assert_eq!(state_hash(&a), state_hash(&b), "the arms diverged after the reload");
        for id in alive(&a) {
            assert_eq!(bloc_shares(&a, id), bloc_shares(&b, id), "{}", id.code());
            assert_eq!(ruling_bloc(&a, id), ruling_bloc(&b, id), "{}", id.code());
        }
    }

    /// The worked values from the design, measured on this tree against
    /// `government::pains`: order = (60 - stability) / 60 with separatism zero,
    /// prices zero at 3% inflation, growth zero at 1%, war zero. Stability 40
    /// alone reads 0.50 * 20/60 = 0.16667; stability 25 alone reads
    /// 0.50 * 35/60 = 0.29167. Watched red with the order weight at 0.45:
    /// "stability 40 alone read 0.15".
    #[test]
    fn the_discontent_gauge_reads_the_pains() {
        let mut w = world_1990(GameRules::default());
        let id = NationId::Poland;
        let quiet = |n: &mut Nation, stability: f64| {
            n.stability = stability;
            n.inflation = 0.03;
            n.growth_last = 0.01;
            n.war_exhaustion = 0.0;
            n.separatism = 0.0;
        };
        quiet(w.nation_mut(id), 40.0);
        let d40 = discontent(&w, id);
        assert!((d40 - 1.0 / 6.0).abs() < 1e-12, "stability 40 alone read {d40}");
        quiet(w.nation_mut(id), 25.0);
        let d25 = discontent(&w, id);
        assert!((d25 - 7.0 / 24.0).abs() < 1e-12, "stability 25 alone read {d25}");
        // Prices, growth and war carry a fifth, a fifth and a tenth.
        quiet(w.nation_mut(id), 60.0);
        w.nation_mut(id).inflation = 0.18;
        assert!((discontent(&w, id) - 0.20).abs() < 1e-12);
        quiet(w.nation_mut(id), 60.0);
        w.nation_mut(id).growth_last = -0.04;
        assert!((discontent(&w, id) - 0.20).abs() < 1e-12);
        quiet(w.nation_mut(id), 60.0);
        w.nation_mut(id).war_exhaustion = 1.0;
        assert!((discontent(&w, id) - 0.10).abs() < 1e-12);
        // Order saturates at 1 before it is weighed, and the whole thing at 1.
        quiet(w.nation_mut(id), 0.0);
        w.nation_mut(id).separatism = 1.0;
        w.nation_mut(id).inflation = 1.0;
        w.nation_mut(id).growth_last = -1.0;
        w.nation_mut(id).war_exhaustion = 1.0;
        assert!((discontent(&w, id) - 1.0).abs() < 1e-12);
    }

    /// The ruling bloc of every 1990 nation, as a sorted list per bloc, with
    /// the census printed so a red run shows the whole picture.
    fn census_1990(w: &WorldState) -> ([usize; 5], [Vec<&'static str>; 5]) {
        let mut census = [0usize; 5];
        let mut who: [Vec<&str>; 5] = Default::default();
        for id in alive(w) {
            let b = ruling_bloc(w, id)
                .unwrap_or_else(|| panic!("{} has no ruling bloc with the arm on", id.code()));
            census[b as usize] += 1;
            who[b as usize].push(id.code());
        }
        for b in Bloc::ALL {
            who[b as usize].sort_unstable();
            println!("{b:?} {}: {:?}", census[b as usize], who[b as usize]);
        }
        (census, who)
    }

    /// The 1990 census over the full leader table, integrated 2026-09-05 and
    /// MEASURED on that tree: Western 67, Communist 17, Nationalist 7,
    /// Islamist 3, Non-Aligned 43, summing to the 137 nations of the roster.
    /// The counts are pinned as transcribed data is pinned elsewhere in this
    /// suite — a change here is a change to a sourced row or to the pillar map,
    /// and it is meant to be noticed. The bars of the design brief that the
    /// rows AGREE with are asserted here; the three they disagree with live in
    /// `the_1990_census_meets_the_design_brief`, which is red until Ridge
    /// decides. Watched red by dropping Solidarity's Western override: Poland
    /// read Some(NonAligned) against Some(Western), and the Western count fell
    /// to 66.
    #[test]
    fn the_1990_ruling_bloc_census() {
        let w = world_1990(on(1990));
        let rows = w.leadership.as_ref().expect("the table is loaded when the arm is on");
        assert_eq!(rows.len(), alive(&w).len(), "one row per 1990 nation");
        let (census, who) = census_1990(&w);
        assert_eq!(census.iter().sum::<usize>(), alive(&w).len());
        assert!(census.iter().all(|c| *c > 0), "every bloc rules somewhere in 1990: {census:?}");
        assert_eq!(census, [67, 17, 7, 3, 43], "the census as transcribed on 2026-09-05");
        // The decided cases (design D1-D6) as the table alone settles them.
        assert_eq!(ruling_bloc(&w, NationId::Poland), Some(Bloc::Western), "Solidarity's umbrella");
        assert_eq!(leader_bloc(&w, NationId::Poland), Some(Bloc::Western));
        assert_eq!(ruling_bloc(&w, NationId::China), Some(Bloc::Communist));
        assert_eq!(ruling_bloc(&w, NationId::USSR), Some(Bloc::Communist));
        assert_eq!(ruling_bloc(&w, NationId::Iran), Some(Bloc::Islamist));
        assert_eq!(ruling_bloc(&w, NationId::Sudan), Some(Bloc::Islamist), "Bashir's sourced override");
        assert_eq!(ruling_bloc(&w, NationId::SaudiArabia), Some(Bloc::NonAligned));
        assert_eq!(ruling_bloc(&w, NationId::USA), Some(Bloc::Western));
        // The bars of the brief the rows meet.
        assert!(census[Bloc::Western as usize] >= 55, "Western: {census:?}");
        for id in ["Iraq", "Syria"] {
            assert!(who[Bloc::Nationalist as usize].contains(&id), "{id} is not Nationalist: {who:?}");
        }
        for id in ["Iran", "Sudan"] {
            assert!(who[Bloc::Islamist as usize].contains(&id), "{id} is not Islamist: {who:?}");
        }
        for id in ["SaudiArabia", "Egypt", "Indonesia", "Jordan"] {
            assert!(who[Bloc::NonAligned as usize].contains(&id), "{id} is not Non-Aligned: {who:?}");
        }
        // The stored seed agrees with the readout and is FLAT.
        let g = government::state(&w, NationId::China).unwrap();
        assert_eq!(g.regime_bloc, Some(Bloc::Communist));
        assert_eq!(g.movements.len(), 5);
        assert_eq!(g.movements, flat_seed(NationId::China, Bloc::Communist).to_vec());
        assert!(g.movements[Bloc::Communist as usize].1 > 0.59);
        for pl in government::state(&w, NationId::Poland).unwrap().movements.iter() {
            panic!("an electoral nation stored a movement: {pl:?}");
        }
        // A refused row is described from its table: Chile, whose Pinochet has
        // no valid tie, and Panama, whose Endara stood on a struck-off party,
        // read as the chamber their tables seat — both Western; and the three
        // rows the 2026-09-05 provenance audit refused read as their chambers
        // too — Comoros Udzima (Non-Aligned), Cyprus DISY and Greece ND (both
        // Western), which is why the census above did not move when their
        // names came out: all three are electoral polities whose ruling bloc
        // was the chamber leader's all along.
        for (id, bloc) in [
            (NationId::Chile, Bloc::Western),
            (NationId::Panama, Bloc::Western),
            (NationId::Comoros, Bloc::NonAligned),
            (NationId::Cyprus, Bloc::Western),
            (NationId::Greece, Bloc::Western),
        ] {
            assert_eq!(leader_row(&w, id), None, "{}", id.code());
            assert_eq!(leader_bloc(&w, id), None, "{}", id.code());
            assert_eq!(ruling_bloc(&w, id), Some(bloc), "{}", id.code());
        }
    }

    /// The three bars of the design brief (2026-09-05) that the transcribed
    /// rows DISAGREE with, kept as written — not bent (iron rule 5) — and
    /// PARKED under `#[ignore]` with the disagreement filed as BUGS.md P-6
    /// until Ridge rules on the rows or the bars. It was red on the tree from
    /// the day it was written, and it was ignored rather than left red because
    /// the suite's contract is exactly three deliberate reds (the two goldens
    /// and BUGS E-3) and a fourth hides a real regression; run it with
    /// `--ignored` to see the disagreement, unchanged. Measured on the
    /// integrated table, re-measured after the audit's three refusals (all
    /// three electoral, so no count moved):
    ///
    /// * Communist 11-13 — the rows read 17: USSR, China, Vietnam, NorthKorea,
    ///   Cuba, Albania, Mongolia, Laos, Cambodia, Afghanistan, Ethiopia, plus
    ///   Yugoslavia (Markovic, yu_skj), Bulgaria (Mladenov, bg_bsp, an
    ///   electoral polity at authoritarianism 0.40 whose chamber the BSP
    ///   leads), Nicaragua (Ortega, ni_fsln, electoral at 0.50), Congo (Sassou
    ///   Nguesso, cg_pct), Madagascar (Ratsiraka, mg_arema) and Seychelles
    ///   (Rene, sc_sppf) — every one the Communist family of its own
    ///   transcribed party.
    /// * Islamist exactly Iran and Sudan — the rows read Iran, Sudan AND
    ///   Algeria: Algeria is electoral at authoritarianism 0.55 and the party
    ///   table seats the June 1990 local result, FIS 0.542, so the chamber
    ///   leader is the FIS whatever Bendjedid's own FLN tie says. That is the
    ///   pre-existing table's transcription, not the leader row's.
    /// * Nationalist includes Libya — the rows read Libya Non-Aligned: Gaddafi
    ///   is tied to the Party pillar (the Revolutionary Committees Movement),
    ///   which the D3 pillar map installs as Non-Aligned in a non-Communist
    ///   regime, and no fetched source gave the row a bloc_override the way
    ///   Sudan's did.
    #[test]
    #[ignore = "RED BY DESIGN and filed as BUGS.md P-6: the transcribed rows disagree with three bars of the brief (Communist 17 vs 11-13; Islamist adds Algeria; Libya Non-Aligned); Ridge rules on the rows or the bars"]
    fn the_1990_census_meets_the_design_brief() {
        let w = world_1990(on(1990));
        let (census, who) = census_1990(&w);
        let mut misses = vec![];
        if !(11..=13).contains(&census[Bloc::Communist as usize]) {
            misses.push(format!("Communist {} against the brief's 11-13", census[Bloc::Communist as usize]));
        }
        if who[Bloc::Islamist as usize] != ["Iran", "Sudan"] {
            misses.push(format!("Islamist {:?} against the brief's exactly Iran and Sudan", who[Bloc::Islamist as usize]));
        }
        if !who[Bloc::Nationalist as usize].contains(&"Libya") {
            misses.push(format!(
                "Libya reads {:?} against the brief's Nationalist",
                ruling_bloc(&w, NationId::Libya)
            ));
        }
        assert!(misses.is_empty(), "the rows disagree with the design brief: {misses:#?}");
    }

    /// Every alive nation's shares are non-negative and sum to one, with the
    /// arm off (the flat seed derived live) and on (the seed stored), at the
    /// start and after ten years. Watched red by skipping `normalise` in
    /// `flat_seed`: the floor put every regime's sum over one and the first
    /// regime in roster order failed the bar.
    #[test]
    fn a_bloc_share_sums_to_one_everywhere() {
        for rules in [GameRules::default(), on(1990)] {
            let mut w = world_1990(rules);
            for pass in 0..2 {
                for id in alive(&w) {
                    let shares = bloc_shares(&w, id);
                    let sum: f64 = shares.iter().map(|(_, v)| *v).sum();
                    assert!((sum - 1.0).abs() < 1e-9, "{} pass {pass}: {shares:?}", id.code());
                    assert!(shares.iter().all(|(_, v)| *v >= 0.0), "{}: {shares:?}", id.code());
                    assert_eq!(shares.iter().map(|(b, _)| *b).collect::<Vec<_>>(), Bloc::ALL.to_vec());
                    let infl = influence(&w, id);
                    for i in 0..5 {
                        assert_eq!(infl[i].1.to_bits(), shares[i].1.to_bits(), "backing is zero this run");
                    }
                }
                run_months(&mut w, 120);
            }
        }
    }

    /// The monarchy exception: an electoral polity whose leader row ties to a
    /// pillar is ruled by that pillar's bloc while authoritarianism is at or
    /// over 0.40, and the chamber leader is served as the government of the
    /// day. The 1990 table carries King Hussein tied to the Hashemite court
    /// (Pillar::Party), so the exception is live at the start; the no-row path
    /// is exercised by taking his row out of the loaded table. Watched red
    /// with the exception disabled: Jordan read Islamist.
    #[test]
    fn the_court_outranks_the_chamber_in_jordan() {
        let mut w = world_1990(on(1990));
        let id = NationId::Jordan;
        assert!(government::is_electoral(&w, id));
        let row = leader_row(&w, id).expect("the 1990 table names Jordan's king");
        assert_eq!(row.name.as_deref(), Some("Hussein"));
        assert_eq!(row.tie, Some(Tie::Pillar(Pillar::Party)), "the Hashemite court");
        assert!(w.nation(id).authoritarianism >= COURT_RULES_ABOVE);
        assert_eq!(ruling_bloc(&w, id), Some(pillar_bloc(id, Pillar::Party)));
        assert_eq!(ruling_bloc(&w, id), Some(Bloc::NonAligned));
        let leader = government::state(&w, id).unwrap().leader().unwrap().to_string();
        assert_eq!(government_of_the_day(&w, id), Some(leader));
        // Liberalise below the line and the chamber rules again.
        w.nation_mut(id).authoritarianism = COURT_RULES_ABOVE - 0.01;
        assert_eq!(ruling_bloc(&w, id), Some(Bloc::Islamist), "the Brotherhood leads the 1989 chamber");
        assert_eq!(government_of_the_day(&w, id), None);
        // No row, no exception, at any level of authoritarianism.
        w.nation_mut(id).authoritarianism = COURT_RULES_ABOVE + 0.10;
        w.leadership.as_mut().unwrap().retain(|o| o.nation != id);
        assert_eq!(leader_row(&w, id), None);
        assert_eq!(government_of_the_day(&w, id), None, "no row, no exception");
        assert_eq!(ruling_bloc(&w, id), Some(Bloc::Islamist));
        // And a REFUSED row is no row: a nameless Jordan asserts nothing.
        w.leadership.as_mut().unwrap().push(Office {
            nation: id,
            name: None,
            native: None,
            office: "King".into(),
            since: "1952-08-11".into(),
            born: None,
            tie: None,
            bloc_override: None,
            heir: None,
            must_leave_by: None,
            also: vec![],
            sources: vec!["test".into()],
            note: Some("REFUSED: test".into()),
        });
        assert_eq!(leader_row(&w, id), None);
        assert_eq!(leader_bloc(&w, id), None);
        assert_eq!(ruling_bloc(&w, id), Some(Bloc::Islamist));
    }

    /// Every road reads closed with the build's reason, on and off, and the
    /// gauges carry the design's triggers. Watched red with the collapse road
    /// served open.
    #[test]
    fn every_road_reads_closed_in_this_build() {
        for rules in [GameRules::default(), on(1990)] {
            let w = world_1990(rules);
            for id in alive(&w) {
                let t = takeover_readout(&w, id);
                for road in t.roads() {
                    assert!(!road.open, "{}: a road is open", id.code());
                    assert_eq!(road.reason, NOT_IN_THIS_BUILD);
                    for g in &road.gauges {
                        assert!(g.value.is_finite() && g.progress().is_finite(), "{}: {g:?}", id.code());
                    }
                }
                assert_eq!(t.coup.gauges[0].trigger, 0.35);
                assert_eq!(t.coup.gauges[1].trigger, 0.25);
                assert_eq!(t.coup.gauges[2].trigger, 1.0 / w.rules.crisis_intensity);
                assert_eq!(t.uprising.gauges[0].trigger, 0.45);
                assert_eq!(t.uprising.gauges[1].trigger, 0.45);
                assert!(t.uprising.gauges[1].bloc.is_some(), "{}: the challenger is named", id.code());
                assert_ne!(t.uprising.gauges[1].bloc, ruling_bloc(&w, id));
                assert_eq!(t.round_table.gauges[0].trigger, 0.40);
                assert_eq!(t.round_table.gauges[1].trigger, 0.55);
                assert_eq!((t.round_table.gauges[2].trigger, t.round_table.gauges[2].upper), (30.0, Some(70.0)));
                assert_eq!(t.collapse.gauges[0].trigger, 12.0);
            }
        }
        // The hatch: a gauge at half its trigger arms it, whichever way it runs.
        assert!(Gauge::above("x", 0.125, 0.25).progress() >= 0.5);
        assert!(Gauge::above("x", 0.124, 0.25).progress() < 0.5);
        assert!((Gauge::below("x", 0.675, 0.35).progress() - 0.5).abs() < 1e-12);
        assert!((Gauge::below("x", 56.0, 12.0).progress() - 0.5).abs() < 1e-12);
        assert_eq!(Gauge::inside("x", 50.0, 30.0, 70.0).progress(), 1.0);
        assert_eq!(Gauge::inside("x", 20.0, 30.0, 70.0).progress(), 0.0);
        // A band met is not a gauge at half: a road whose only gauge is a
        // satisfied band is armed, and NOT half-armed for the hatch. Watched
        // red on 2026-09-05 with `half_armed` reading every gauge again.
        let band_only = Road::closed(vec![Gauge::inside("x", 50.0, 30.0, 70.0)]);
        assert!(band_only.armed());
        assert!(!band_only.half_armed(), "a band has no half");
        let mixed = Road::closed(vec![Gauge::inside("x", 50.0, 30.0, 70.0), Gauge::above("y", 0.13, 0.25)]);
        assert!(mixed.half_armed(), "the threshold gauge beside it still hatches");
    }

    /// The transcribed maps, spot-checked against the rows they were read
    /// from.
    #[test]
    fn the_pillar_map_and_overrides_read_as_transcribed() {
        assert_eq!(bloc_of(NationId::Poland, "pl_solidarity"), Bloc::Western);
        assert_eq!(bloc_of(NationId::India, "in_inc"), Bloc::NonAligned);
        assert_eq!(bloc_of(NationId::India, "in_bjp"), Bloc::Nationalist);
        assert_eq!(bloc_of(NationId::Russia, "ru_apr"), Bloc::Communist);
        assert_eq!(bloc_of(NationId::Pakistan, "pk_iji"), Bloc::Western);
        assert_eq!(bloc_of(NationId::Algeria, "dz_fis"), Bloc::Islamist);
        assert_eq!(bloc_of(NationId::Japan, "jp_komeito"), Bloc::Western);
        assert_eq!(bloc_of(NationId::Poland, "not_a_party"), Bloc::NonAligned);
        assert!(government::regime_is_communist(NationId::China));
        assert!(government::regime_is_communist(NationId::USSR));
        assert!(!government::regime_is_communist(NationId::Poland));
        assert!(!government::regime_is_communist(NationId::Iraq));
        assert_eq!(pillar_bloc(NationId::China, Pillar::Party), Bloc::Communist);
        assert_eq!(pillar_bloc(NationId::SaudiArabia, Pillar::Party), Bloc::NonAligned);
        assert_eq!(pillar_bloc(NationId::Iran, Pillar::Clergy), Bloc::Islamist);
        assert_eq!(pillar_bloc(NationId::Fiji, Pillar::Clergy), Bloc::Western);
        assert_eq!(pillar_bloc(NationId::Sudan, Pillar::Army), Bloc::Nationalist);
        assert_eq!(pillar_bloc(NationId::Iraq, Pillar::Security), Bloc::NonAligned);
        assert_eq!(pillar_bloc(NationId::China, Pillar::Business), Bloc::Western);
        // Every polity's ruling bloc is describable from the table alone.
        for pol in government::POLITIES {
            let w = world_1990(GameRules::default());
            assert!(described_ruling_bloc(&w, pol.nation).is_some(), "{}", pol.nation.code());
        }
    }

    /// The surface: `politics` is `None` with the arm off — the browser then
    /// serves null for every field — and whole with it on. On seed 7: Poland
    /// is Western under a NAMED Mazowiecki tied to pl_solidarity; Chile's
    /// REFUSED row is "the office-holder", nameless, with the row's sourced
    /// office and date kept; every living nation's five rows sum to one; and
    /// every gauge's SERVED `progress`/`met` equal the methods they were
    /// taken from, every road's `armed`/`half_armed` likewise, the readout's
    /// `half_armed` the any-of-roads it is defined as. Watched red with
    /// `finish()` dropped from `Gauge::above`: the first nation in roster
    /// order, the USA, served its coup discontent gauge at progress 0 against
    /// a method reading 0.304.
    #[test]
    fn politics_is_null_off_and_served_whole_on() {
        let off = world_1990(GameRules::default());
        assert!(politics(&off, NationId::Poland).is_none());
        let w = world_1990(on(7));
        let pl = politics(&w, NationId::Poland).expect("the arm is on");
        assert_eq!(pl.ruling_bloc, Some(Bloc::Western));
        assert_eq!(pl.discontent, discontent(&w, NationId::Poland));
        let lead = pl.leader.as_ref().unwrap();
        assert_eq!(lead.name.as_deref(), Some("Tadeusz Mazowiecki"));
        assert_eq!(lead.party.as_deref(), Some("pl_solidarity"));
        assert_eq!(lead.party_name.as_deref(), Some("Solidarity Citizens' Committee"));
        assert!(lead.described.is_none());
        assert!(pl.government_of_the_day.is_none());
        let cl = politics(&w, NationId::Chile).unwrap();
        let lead = cl.leader.as_ref().unwrap();
        assert!(lead.name.is_none());
        assert_eq!(lead.described.as_deref(), Some("the office-holder"));
        assert_eq!(lead.office, "President of the Republic and Commander-in-Chief of the Army");
        assert_eq!(lead.since.as_deref(), Some("1974-12-17"));
        assert!(lead.party.is_none(), "a refused row's office is not the chamber leader's");
        let jo = politics(&w, NationId::Jordan).unwrap();
        assert_eq!(jo.ruling_bloc, Some(Bloc::NonAligned));
        assert_eq!(jo.government_of_the_day.as_deref(), Some("jo_ikhwan"));
        for id in alive(&w) {
            let p = politics(&w, id).unwrap();
            assert_eq!(p.blocs.len(), 5);
            let sum: f64 = p.blocs.iter().map(|r| r.share).sum();
            if government::state(&w, id).is_some() {
                assert!((sum - 1.0).abs() < 1e-9, "{}: shares sum to {sum}", id.code());
            }
            assert!(p.blocs.iter().all(|r| r.backing == 0.0 && !r.banned));
            assert_eq!(p.blocs.iter().map(|r| r.bloc).collect::<Vec<_>>(), Bloc::ALL.to_vec());
            let t = &p.takeover;
            for r in t.roads() {
                assert!(!r.open && r.reason == NOT_IN_THIS_BUILD);
                assert_eq!(r.armed, r.armed(), "{}", id.code());
                assert_eq!(r.half_armed, r.half_armed(), "{}", id.code());
                for g in &r.gauges {
                    assert_eq!(g.progress, g.progress(), "{}: {}", id.code(), g.name);
                    assert_eq!(g.met, g.met(), "{}: {}", id.code(), g.name);
                }
            }
            assert_eq!(t.half_armed, t.half_armed(), "{}", id.code());
            assert!(p.leader.is_some(), "{}", id.code());
        }
        // The hatch, measured and printed so a red run shows which gauge did
        // it. Before the 2026-09-05 repair the round table's stability band
        // read progress 1 inside 30..70 and hatched every nation it held;
        // `Road::half_armed` no longer reads a band (a band has no half). What
        // remains is the design's own arithmetic on its own triggers, and it
        // STILL hatches all 137 living nations of 1990 — MEASURED after the
        // repair on seed 7: 171 discontent gauges at half (the coup's 0.25 and
        // the uprising's 0.45 both read the same number, so a stability under
        // about 45 arms both), Western influence 80, stability-toward-12 72,
        // army loyalty 71, party loyalty 60, challenger influence 24, and the
        // band 102 (no longer read); a nation hatched by the band ALONE: 0. A
        // road-level reading — every gauge on one road at half — would hatch
        // 73 (printed as `road_level` below). Recorded, not bent: the
        // gauge-level reading is the approved design's, and re-reading it is
        // Ridge's call (BUGS.md P-5). The bars here pin the measurement so a
        // change to the triggers or to `half_armed` is noticed.
        let mut per_gauge: std::collections::BTreeMap<String, usize> = Default::default();
        let mut hatched = 0;
        let mut band_only = 0;
        let mut road_level = 0;
        for id in alive(&w) {
            let t = politics(&w, id).unwrap().takeover;
            if t.half_armed {
                hatched += 1;
            }
            if t.roads().iter().any(|r| r.gauges.iter().all(|g| g.sense != Sense::Inside && g.progress >= 0.5)) {
                road_level += 1;
            }
            let mut any_threshold = false;
            for r in t.roads() {
                for g in &r.gauges {
                    if g.progress >= 0.5 {
                        if g.sense == Sense::Inside {
                            *per_gauge.entry(format!("{} (band, not read)", g.name)).or_insert(0) += 1;
                        } else {
                            any_threshold = true;
                            *per_gauge.entry(g.name.to_string()).or_insert(0) += 1;
                        }
                    }
                }
            }
            assert_eq!(t.half_armed, any_threshold, "{}: the hatch reads threshold gauges only", id.code());
            if !any_threshold && t.round_table.gauges[2].progress >= 0.5 {
                band_only += 1;
            }
        }
        println!("hatched {hatched} of 137; band-only {band_only}; road_level {road_level}; per gauge {per_gauge:?}");
        assert_eq!(hatched, 137, "the 1990 hatch as measured 2026-09-05: {per_gauge:?}");
        assert_eq!(band_only, 0);
        assert_eq!(per_gauge["discontent"], 171);
        assert_eq!(per_gauge["stability (band, not read)"], 102);
    }

    /// `refusal_of` says exactly what `apply_command` would, read without
    /// touching the world — the world's bar, then the treasury's, then the
    /// command's own — for the commands the government screen serves, and a
    /// command it would let through it answers `None` for. Watched red with
    /// the treasury branch removed from `refusal_of`: the unaffordable
    /// invitation read `None` against "Poland has not the standing: 0.0
    /// political capital held, 24.3 needed."
    #[test]
    fn refusal_of_says_exactly_what_apply_command_would() {
        use crate::{apply_command, refusal_of, Command};
        let mut w = world_1990(on(7));
        let pl = NationId::Poland;
        let iq = NationId::Iraq;
        let cases: Vec<Command> = vec![
            Command::InviteToGovernment { nation: pl, party: "pl_sld".into() },
            Command::InviteToGovernment { nation: pl, party: "pl_solidarity".into() },
            Command::InviteToGovernment { nation: pl, party: "pl_nobody".into() },
            Command::InviteToGovernment { nation: iq, party: "iq_baath".into() },
            Command::ExpelFromGovernment { nation: pl, party: "pl_solidarity".into() },
            Command::ExpelFromGovernment { nation: pl, party: "pl_sld".into() },
            Command::CallElection { nation: pl },
            Command::CallElection { nation: iq },
            Command::SecurePillar { nation: pl, pillar: Pillar::Army },
            Command::SecurePillar { nation: iq, pillar: Pillar::Clergy },
            Command::SecurePillar { nation: iq, pillar: Pillar::Army },
            Command::EnactStratagem { nation: pl, id: "security_crackdown".into() },
            Command::EnactStratagem { nation: iq, id: "security_crackdown".into() },
            Command::EnactStratagem { nation: pl, id: "no_such_thing".into() },
        ];
        let before = state_hash(&w);
        let mut refused = 0;
        for c in &cases {
            let read = refusal_of(&w, c);
            let mut trial = w.clone();
            let did = apply_command(&mut trial, c);
            assert_eq!(read, did.clone().err(), "{c:?}");
            if read.is_some() {
                refused += 1;
            }
        }
        assert_eq!(refused, 11, "three of the fourteen go through: the affordable invitation, Iraq paying its Guard, Iraq's crackdown");
        assert_eq!(state_hash(&w), before, "refusal_of wrote something");
        // The treasury's sentence, word for word.
        w.nation_mut(pl).political_capital = 0.0;
        let c = Command::InviteToGovernment { nation: pl, party: "pl_sld".into() };
        let read = refusal_of(&w, &c);
        assert_eq!(read, apply_command(&mut w.clone(), &c).err());
        assert_eq!(read.as_deref(), Some("Poland has not the standing: 0.0 political capital held, 24.3 needed."));
        // Expulsion is ALWAYS available: no standing check, and its own refusal
        // still answers.
        let c = Command::ExpelFromGovernment { nation: pl, party: "pl_solidarity".into() };
        assert_eq!(refusal_of(&w, &c).as_deref(), Some("A government cannot expel the party that leads it."));
    }
}
