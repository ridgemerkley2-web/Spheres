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
use crate::government::{self, bloc_of, pillar_bloc, polity, polity_in, Bloc, GovState, Pillar};
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
    w.leadership.as_ref()?.iter().find(|o| o.nation == id && o.holds())
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
    Some(match row.tie_now()? {
        Tie::Party(p) => bloc_of(id, &p),
        Tie::Pillar(pl) => pillar_bloc(id, pl),
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
    let pol = polity_in(w, id)?;
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

/// The foreign backing F_B at or over which a bloc is PRESENT in a polity
/// whose transcribed shape carries no party and no pillar of it — the
/// design's third presence clause (D5, "foreign backing already behind
/// it"), built 2026-09-06 on Ridge's ruling M1. One clean `BackBloc`
/// operation puts `statecraft::BACKING_STEP` = 0.06 behind a bloc, so THE
/// FIRST SUCCESSFUL OPERATION CREATES THE PRESENCE, and it lapses when the
/// money stops (the stock cools `BACKING_DECAY` a month). Historical
/// anchor: the Afghan mujahideen parties existed inside Afghanistan only as
/// Pakistani- and Saudi-funded organisations run from Peshawar; Sudan's NIF
/// grew on Gulf money; Nicaragua's contras were a movement a sponsor
/// created. The line is the ruling's own number.
pub const PRESENCE_BACKING: f64 = 0.05;

/// Whether a bloc has any standing presence in a polity's transcribed shape: a
/// party of that bloc in the (possibly dormant) table, or the bloc's installing
/// pillar in the pillar list — or, since M1, foreign backing of at least
/// [`PRESENCE_BACKING`] behind it (`backing`, the stock plus patronage
/// gravity). The backing clause reads the world, which is why the world is
/// an argument; with the arm off the stock is always empty (`BackBloc` is
/// refused) and nothing the tick reads asks, so the answer is the table's.
pub fn bloc_present(w: &WorldState, id: NationId, bloc: Bloc) -> bool {
    bloc_in_table(w, id, bloc) || bloc_backed(w, id, bloc)
}

/// The table half of [`bloc_present`]: a party or an installing pillar of the
/// bloc in the polity's transcribed shape — the table the switch serves
/// (`polity_in`: D4's block under the lens, R1), so it reads the world too.
pub fn bloc_in_table(w: &WorldState, id: NationId, bloc: Bloc) -> bool {
    let pol = match polity_in(w, id) {
        Some(p) => p,
        None => return false,
    };
    pol.parties.iter().any(|s| bloc_of(id, s.id) == bloc)
        || pol.pillars.iter().any(|s| pillar_bloc(id, s.pillar) == bloc)
}

/// The backing half of [`bloc_present`]: foreign backing of at least
/// [`PRESENCE_BACKING`] behind the bloc in this nation.
pub fn bloc_backed(w: &WorldState, id: NationId, bloc: Bloc) -> bool {
    backing(w, id)[bloc as usize].1 >= PRESENCE_BACKING
}

/// The flat seed of a regime's movements (design D5): the ruling bloc at
/// [`RULING_SEED`], the remainder split equally over the non-ruling blocs
/// present in the polity, every bloc floored at [`SHARE_FLOOR`], normalised to
/// one. Pure in the world: the same table, the same backing and the same
/// ruling bloc give the same seed (a bloc present through backing takes an
/// equal split, M1).
pub fn flat_seed(w: &WorldState, id: NationId, ruling: Bloc) -> [(Bloc, f64); 5] {
    let mut shares = zero_shares();
    let present: Vec<Bloc> =
        Bloc::ALL.iter().copied().filter(|b| *b != ruling && bloc_present(w, id, *b)).collect();
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
        Some(r) => flat_seed(w, id, r),
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
pub(crate) fn court_pillar(w: &WorldState, id: NationId) -> Option<Pillar> {
    if !government::is_electoral(w, id) {
        return None;
    }
    let row = leader_row(w, id)?;
    match row.tie_now() {
        Some(Tie::Pillar(pl)) if w.nation(id).authoritarianism >= COURT_RULES_ABOVE => Some(pl),
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

/// The STOCK of foreign backing per bloc: what sponsors have put in through
/// `CovertOp::BackBloc`, summed from `Statecraft.backing`, uncapped.
pub fn backing_stock(w: &WorldState, id: NationId) -> [(Bloc, f64); 5] {
    let mut out = zero_shares();
    for b in &w.statecraft.backing {
        if b.target == id {
            *slot(&mut out, b.bloc) += b.weight;
        }
    }
    out
}

/// The weight patronage gravity puts behind a patron's bloc in a client that
/// takes a tenth of its output from it; the full weight is reached at an
/// infusion of [`GRAVITY_FULL_AT`]. INVENTED (design S3).
pub const GRAVITY_WEIGHT: f64 = 0.10;
pub const GRAVITY_FULL_AT: f64 = 0.10;

/// Patronage gravity (S3), a VIEW and not a stock: for each live aid flow into
/// this nation, the patron's ruling bloc counts `GRAVITY_WEIGHT * min(1,
/// infusion / GRAVITY_FULL_AT)`, where infusion is the flow's annual value
/// over the client's output — the same ratio `statecraft::aid_flows` reads.
/// Gone the month the flow stops, because nothing is stored. Reads with the
/// switch off as every readout here does; nothing the tick reads calls it
/// unless the arm is on.
pub fn gravity(w: &WorldState, id: NationId) -> [(Bloc, f64); 5] {
    let mut out = zero_shares();
    let client_gdp = match w.nation_opt(id) {
        Some(n) => n.gdp.max(0.1),
        None => return out,
    };
    for f in &w.statecraft.aid {
        if f.client != id {
            continue;
        }
        let patron = match w.nation_opt(f.patron) {
            Some(p) if p.alive => p,
            _ => continue,
        };
        let bloc = match ruling_bloc(w, f.patron) {
            Some(b) => b,
            None => continue,
        };
        let infusion = patron.gdp * f.share_gdp / client_gdp;
        *slot(&mut out, bloc) += GRAVITY_WEIGHT * (infusion / GRAVITY_FULL_AT).min(1.0);
    }
    out
}

/// F_B: foreign backing of each bloc in this state — the stock plus
/// patronage gravity, capped together at `statecraft::BACKING_TOTAL_CAP`.
/// Zero everywhere nothing has been put in and nobody is paid.
pub fn backing(w: &WorldState, id: NationId) -> [(Bloc, f64); 5] {
    let stock = backing_stock(w, id);
    let grav = gravity(w, id);
    let mut out = zero_shares();
    for i in 0..5 {
        out[i].1 = (stock[i].1 + grav[i].1).min(crate::statecraft::BACKING_TOTAL_CAP);
    }
    out
}

/// Effective army loyalty (S3): the Army pillar's loyalty less the
/// Nationalist bloc's foreign backing — an army with money behind its own
/// bloc is that much less the government's. 1.0 where there is no
/// government; the pillar reads 1.0 where there is no Army pillar.
pub fn effective_army_loyalty(w: &WorldState, id: NationId) -> f64 {
    let loyalty = government::state(w, id).map_or(1.0, |g| g.loyalty(Pillar::Army));
    loyalty - backing(w, id)[Bloc::Nationalist as usize].1
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
// The uprising's arithmetic (S4, route 3), pure — the mechanic in
// `government::uprising` and the draw in `politics::tick` read these.
// ---------------------------------------------------------------------------

/// Whether a bloc could WIN an uprising in this polity: present, and not
/// only through Regionalist parties (a regional list is not a movement that
/// takes a capital), and — for the Western bloc — carried by a party table,
/// because a Western winner has nothing to govern through in a party-less
/// polity until a table is transcribed (design S4: "closed until a table
/// exists"). A bloc present through foreign backing alone (M1,
/// [`bloc_backed`]) could win on the same terms as one carried by a pillar:
/// the Peshawar parties took Kabul in April 1992 with no seat and no
/// institution inside the country — but a Western one still needs a table.
pub fn bloc_can_win(w: &WorldState, id: NationId, bloc: Bloc) -> bool {
    let pol = match polity_in(w, id) {
        Some(p) => p,
        None => return false,
    };
    let by_party = pol
        .parties
        .iter()
        .any(|s| bloc_of(id, s.id) == bloc && s.family != government::Family::Regionalist);
    let by_pillar = pol.pillars.iter().any(|s| pillar_bloc(id, s.pillar) == bloc);
    if bloc == Bloc::Western {
        return by_party;
    }
    by_party || by_pillar || bloc_backed(w, id, bloc)
}

/// W: the strongest non-ruling bloc by influence among those that could
/// win (`bloc_can_win`), ties in enum order. `None` where nothing rules or
/// nothing could.
pub fn challenger(w: &WorldState, id: NationId) -> Option<(Bloc, f64)> {
    let ruling = ruling_bloc(w, id)?;
    let mut best: Option<(Bloc, f64)> = None;
    for (b, v) in influence(w, id) {
        if b == ruling || !bloc_can_win(w, id, b) {
            continue;
        }
        if best.map_or(true, |(_, bv)| v > bv) {
            best = Some((b, v));
        }
    }
    best
}

/// Whether the state could not put the crowd down (design S4, route 3). A
/// regime: its weakest armed institution under `COERCION_ARMED` or its mean
/// loyalty under `COERCION_MEAN`. An electoral state: only where the winner
/// is non-Western or authoritarianism is at or over `COERCION_AUTH` — a
/// democracy is not overthrown by its own liberals.
///
/// CALIBRATED 2026-09-06 (the bloc census, anchor A3). The design's lines
/// were 0.50 (weakest) and 0.55 (mean), INVENTED; both now read the pillar
/// model's own UNPAID line, 0.35 — where `regime_tick` starts counting coup
/// pressure and `ai_government` starts buying — so a regime cannot put the
/// crowd down exactly where one of its armed institutions is at the point
/// of moving against it, or where its institutions on average are. At
/// 0.50 / 0.55 every junta that had just removed a government failed the
/// test within two years of the coup, because its pillars walk from
/// 0.90 / 0.72 to budget targets that sit under 0.50 in a poor state, and
/// the deposed colour — still the largest movement — retook the capital:
/// the census read Belarus (Party pillar 0.28-0.51, mean 0.47-0.58) and
/// Cambodia (Army 0.46-0.52, mean 0.67) returning to the Communist colour
/// in 60/60 and 42/60 seeds. A3 asks for at most 10%.
pub fn coercion_fails(w: &WorldState, id: NationId, winner: Bloc) -> bool {
    let g = match government::state(w, id) {
        Some(g) => g,
        None => return false,
    };
    if government::is_electoral(w, id) {
        return winner != Bloc::Western || w.nation(id).authoritarianism >= COERCION_AUTH;
    }
    g.weakest_armed().map_or(1.0, |(_, v)| v) < COERCION_ARMED || g.mean_loyalty() < COERCION_MEAN
}

pub const COERCION_ARMED: f64 = 0.35;
pub const COERCION_MEAN: f64 = 0.35;
pub const COERCION_AUTH: f64 = 0.40;
/// The uprising's two lines: discontent and the challenger's influence.
pub const UPRISING_DISCONTENT: f64 = 0.45;
pub const UPRISING_INFLUENCE: f64 = 0.45;

/// Whether route 3 is ARMED by the movement (the stability-under-12
/// collapse beside it is the pre-arm mechanic and is read by the caller):
/// discontent at or over 0.45, the challenger's influence at or over 0.45,
/// and coercion failing. Pure; the draw that decides whether it fires lives
/// in `politics::tick`.
pub fn uprising_armed(w: &WorldState, id: NationId) -> bool {
    let (winner, i_w) = match challenger(w, id) {
        Some(x) => x,
        None => return false,
    };
    discontent(w, id) >= UPRISING_DISCONTENT && i_w >= UPRISING_INFLUENCE && coercion_fails(w, id, winner)
}

/// Why the uprising road is closed, or `None` where it is open.
pub fn uprising_closed(w: &WorldState, id: NationId) -> Option<&'static str> {
    if !w.rules.ideology_takeover {
        return Some(CALIBRATION_PENDING);
    }
    if challenger(w, id).is_some() {
        return None;
    }
    match strongest_challenger(w, id) {
        Some((Bloc::Western, _)) if polity_in(w, id).is_some_and(|p| p.parties.is_empty()) => Some(NO_TABLE_FOR_WESTERN),
        _ => Some(NO_CHALLENGER),
    }
}

// ---------------------------------------------------------------------------
// The round table's arithmetic (S4, route 4), pure.
// ---------------------------------------------------------------------------

/// The loyalty the round table reads: the Party pillar's, or the weakest
/// armed institution's where the regime has no Party pillar. 1.0 where
/// there is no government.
pub fn round_table_loyalty(w: &WorldState, id: NationId) -> f64 {
    let g = match government::state(w, id) {
        Some(g) => g,
        None => return 1.0,
    };
    if g.pillars.iter().any(|(p, _)| *p == Pillar::Party) {
        return g.loyalty(Pillar::Party);
    }
    g.weakest_armed().map_or(1.0, |(_, v)| v)
}

pub const ROUND_TABLE_WESTERN: f64 = 0.40;
pub const ROUND_TABLE_LOYALTY: f64 = 0.55;
pub const ROUND_TABLE_STABILITY: (f64, f64) = (30.0, 70.0);
/// The floor the drift walks authoritarianism to: under the electoral
/// ceiling (0.60), so a polity with a table opens on the way down and a
/// party-less one stops here as a regime. INVENTED (design S4).
pub const ROUND_TABLE_FLOOR: f64 = 0.55;

/// Whether route 4 is armed this month: a regime, Western influence at or
/// over 0.40, stability inside 30..70 (the upper bound exclusive), the
/// round-table loyalty under 0.55. Pure.
pub fn round_table_armed(w: &WorldState, id: NationId) -> bool {
    if government::is_electoral(w, id) || government::state(w, id).is_none() {
        return false;
    }
    let n = match w.nation_opt(id) {
        Some(n) => n,
        None => return false,
    };
    influence(w, id)[Bloc::Western as usize].1 >= ROUND_TABLE_WESTERN
        && n.stability >= ROUND_TABLE_STABILITY.0
        && n.stability < ROUND_TABLE_STABILITY.1
        && round_table_loyalty(w, id) < ROUND_TABLE_LOYALTY
}

/// Why the round-table road is closed, or `None` where it is open.
pub fn round_table_closed(w: &WorldState, id: NationId) -> Option<&'static str> {
    if !w.rules.ideology_takeover {
        return Some(CALIBRATION_PENDING);
    }
    if government::is_electoral(w, id) {
        return Some(ALREADY_ELECTORAL);
    }
    None
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
    /// Whether the road can be taken in this world: the mechanic behind it
    /// (S4) is switched on and the polity has the shape it needs. `reason`
    /// says why not, in the sim's words, and is empty on an open road.
    pub open: bool,
    pub reason: &'static str,
    /// `armed()` and `half_armed()` at construction, served for the screen and
    /// the map's hatch.
    pub armed: bool,
    pub half_armed: bool,
}

impl Road {
    /// A road with its gauges and the REAL reason it is closed, or `None`
    /// for one that is open.
    fn new(gauges: Vec<Gauge>, closed: Option<&'static str>) -> Road {
        let mut r = Road { gauges, open: closed.is_none(), reason: closed.unwrap_or(""), armed: false, half_armed: false };
        r.armed = r.armed();
        r.half_armed = r.half_armed();
        r
    }
    #[cfg(test)]
    fn closed(gauges: Vec<Gauge>) -> Road {
        Road::new(gauges, Some(CALIBRATION_PENDING))
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

/// The reason every road gives while `rules.ideology_takeover` is off — the
/// browser's state in this build: the mechanics are landed (S4) and gated,
/// their calibration against the 1989-92 episodes pending (BUGS.md), so the
/// gauges read live and the roads stay shut.
pub const CALIBRATION_PENDING: &str = "calibration pending";
/// A road whose mechanic has not landed. No road reads it after S4 route 4;
/// kept for the readout's history and the screen's vocabulary.
pub const NOT_IN_THIS_BUILD: &str = "not in this build";
/// The coup road in a polity whose state carries no Army pillar: nobody to
/// stage one.
pub const NO_ARMY: &str = "no army pillar in this polity";
/// The uprising road where no non-ruling bloc could win.
pub const NO_CHALLENGER: &str = "no movement that could take power";
/// The uprising road where the only challenger is Western in a party-less
/// polity: closed until a table is transcribed.
pub const NO_TABLE_FOR_WESTERN: &str = "no party table to seat a Western winner";
/// The round-table road in a state that already votes.
pub const ALREADY_ELECTORAL: &str = "already answers to an electorate";

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
    let pressure = g.map_or(0.0, |g| g.coup_pressure);
    let disc = discontent(w, id);
    let infl = influence(w, id);
    let western = infl[Bloc::Western as usize].1;
    let on = w.rules.ideology_takeover;
    let has_army = g.is_some_and(|g| g.pillars.iter().any(|(p, _)| *p == Pillar::Army));

    // Route 2 (and the regime's own coup): closed without the switch, closed
    // where nobody could stage one.
    let coup = Road::new(
        vec![
            Gauge::below("army loyalty", effective_army_loyalty(w, id), 0.35),
            Gauge::above("discontent", disc, 0.25),
            Gauge::above("coup pressure", pressure, 1.0 / w.rules.crisis_intensity.max(0.1)),
        ],
        if !on {
            Some(CALIBRATION_PENDING)
        } else if !has_army {
            Some(NO_ARMY)
        } else {
            None
        },
    );
    // Route 3: W where one could win, else the strongest challenger for the
    // display with the road closed on it.
    let (cv, cb) = match challenger(w, id).or_else(|| strongest_challenger(w, id)) {
        Some((b, v)) => (v, Some(b)),
        None => (0.0, None),
    };
    let mut challenger = Gauge::above("challenger influence", cv, UPRISING_INFLUENCE);
    challenger.bloc = cb;
    let uprising = Road::new(
        vec![Gauge::above("discontent", disc, UPRISING_DISCONTENT), challenger],
        uprising_closed(w, id),
    );
    // Route 4: the loyalty gauge reads what the drift reads — the Party
    // pillar, or the weakest armed institution where there is none.
    let round_table = Road::new(
        vec![
            Gauge::above("Western influence", western, ROUND_TABLE_WESTERN),
            Gauge::below("party loyalty", round_table_loyalty(w, id), ROUND_TABLE_LOYALTY),
            Gauge::inside("stability", stability, ROUND_TABLE_STABILITY.0, ROUND_TABLE_STABILITY.1),
        ],
        round_table_closed(w, id),
    );
    let collapse = Road::new(
        vec![Gauge::below("stability", stability, 12.0)],
        if !on { Some(CALIBRATION_PENDING) } else { Some(NOT_IN_THIS_BUILD) },
    );
    let mut out = TakeoverReadout { coup, uprising, round_table, collapse, half_armed: false };
    out.half_armed = out.half_armed();
    out
}

// ---------------------------------------------------------------------------
// The surface (S1): what /api/state serves per nation, built here so the
// browser prints numbers and never derives one
// ---------------------------------------------------------------------------

/// One bloc's line: its share S_B, its foreign backing F_B, and whether this
/// government has proscribed it — every party of the bloc in the table
/// banned (`bloc_banned`).
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct BlocRow {
    pub bloc: Bloc,
    pub share: f64,
    pub backing: f64,
    pub banned: bool,
}

/// Whether a bloc is proscribed as a whole: the polity's table carries at
/// least one party of it and this government has banned every one (S3,
/// `BanParty`). A bloc present through a pillar alone is never banned.
pub fn bloc_banned(w: &WorldState, id: NationId, bloc: Bloc) -> bool {
    let banned: &[String] = government::state(w, id).map_or(&[], |g| g.banned.as_slice());
    let members: Vec<&str> = polity_in(w, id)
        .map(|pol| pol.parties.iter().filter(|s| bloc_of(id, s.id) == bloc).map(|s| s.id).collect())
        .unwrap_or_default();
    !members.is_empty() && members.iter().all(|p| banned.iter().any(|q| q == p))
}

/// The five rows in enum order.
pub fn bloc_rows(w: &WorldState, id: NationId) -> Vec<BlocRow> {
    let shares = bloc_shares(w, id);
    let back = backing(w, id);
    (0..5)
        .map(|i| BlocRow {
            bloc: shares[i].0,
            share: shares[i].1,
            backing: back[i].1,
            banned: bloc_banned(w, id, shares[i].0),
        })
        .collect()
}

/// Who directs the executive, as the surface prints it. NAMED where the leader
/// table carries a row for this nation; DESCRIBED by its real institution
/// everywhere else — a successor state, a refused row, a nation the table has
/// not reached. The optional party roster may name a durably bound campaign
/// successor; an unverified or collective identity keeps its description.
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
    let pol = polity_in(w, id)?;
    if let Some(row) = leader_row(w, id) {
        let person = crate::party_leadership::executive_person(w, id);
        // The office has changed hands (S4): described by institution, the
        // remaining heir kept, nothing of the transcribed person.
        if let Some(e) = &row.emergent {
            return Some(Leader {
                name: person.map(|p| p.name.clone()),
                native: person.and_then(|p| p.native.clone()),
                described: person.is_none().then(|| e.described.clone()),
                office: e.office.clone(),
                since: Some(e.since.clone()),
                party_name: e.party.as_deref().and_then(|p| party_name(id, p)),
                party: e.party.clone(),
                pillar_name: e.pillar.and_then(|p| pillar_name(id, p)),
                pillar: e.pillar,
                heir: row.heir.clone(),
                must_leave_by: None,
                also: vec![],
            });
        }
        let (party, pillar) = match &row.tie {
            Some(Tie::Party(p)) => (Some(p.clone()), None),
            Some(Tie::Pillar(pl)) => (None, Some(*pl)),
            None => (None, None),
        };
        return Some(Leader {
            name: person.map(|p| p.name.clone()).or_else(|| row.name.clone()),
            native: person.and_then(|p| p.native.clone()).or_else(|| row.native.clone()),
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
    use crate::{load, save, state_hash, tick_month, Command};

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
    /// first commit of this branch. E-3 has since been repaired and assessed
    /// in HEADLESS_BASELINE_2026-09-04.md; its correction leaves this initial
    /// hash unchanged. Watched red by removing `skip_serializing_if` from
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
            "\"surging\"",
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
            assert!(g.surging.is_empty());
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
    /// pre-merge tree produces. The S1 constants predated the approved E-3
    /// productivity-reference correction (TECH_REFERENCE_REPAIR.md), whose
    /// credited-revelation ledger and corrected later economy change the
    /// monthly baseline. At integration on 2026-09-07, cdfc6c5 was built from
    /// `git archive` in a separate CARGO_TARGET_DIR; all six complete saves,
    /// plus the seed-1990 golden, were BYTE-IDENTICAL to the merged world after
    /// 240 ticks. These strict constants are that independently measured E-3
    /// baseline, not a relaxation or a change to the political arm. That is
    /// the inertness proof: the OFF world is the world the goldens pin.
    ///
    /// Then the switch ON against OFF on seed 1990, Libya in the player's
    /// seat in both (so the one sponsor with a movement to back in January
    /// 1990 — Libya over Chad's army — is not an AI actor): every quantity
    /// the tick reads is bit-identical nation by nation, the RNG stream
    /// identical, UNTIL THE FIRST MONTH THE ARM ACTS. S3 gives the switched-on
    /// world three deliberate writes into the model — an AI sponsor's
    /// BackBloc on the patrons' existing covert draw (a different op on the
    /// same roll), the ideological sponsors' own draw, and the liberalisation
    /// seam — and each leaves a stem in the headlines. The first month the
    /// two worlds part is PINNED as measured (2026-09-06, S3 commit 3): month
    /// 35, France's backing of the Western movement in Libya, caught by Libya
    /// on the same roll that catches France's ordinary op in the OFF world.
    /// Every month before it is bit-identical, and the parting month's
    /// on-only news must carry an arm stem: a leak of the arm into the model
    /// anywhere else, or earlier, moves the pin or fails the stem. Under S1
    /// the bar ran all 240 months with nothing to act; the ON half was
    /// re-expressed here, not widened, because the approved S3 design makes
    /// the switched-on world act by construction. Watched red with a 1e-9
    /// leak of the drift into support: "parted at month 0 with no stem".
    ///
    /// M1, 2026-09-06 (S3-7 / S4-8 continued): the first-act pin moved from
    /// (35, "backing the Western movement in Libya") to month 0, because
    /// Ridge's ruling M1 — "a sponsor's AI covert arm may back its OWN bloc
    /// in a target where that bloc is ABSENT when the target is hostile to
    /// the sponsor (relation at or below -30) or a rival's client - the
    /// first successful operation creates the presence" — gives Pakistan
    /// (at -70 with Kabul) a choice in Afghanistan from January 1990, and
    /// the sponsors' standing programme (`politics::SPONSOR_PROGRAMME_HEAT`)
    /// sends it that month. MEASURED on this seed before the pin moved:
    /// "the first act moved: [A covert operation against Afghanistan comes
    /// to nothing., Afghanistan exposes Pakistan backing the Islamist
    /// movement in Afghanistan …, A covert operation against Afghanistan
    /// comes to nothing., Money and organisers reach the Islamist movement
    /// in Afghanistan …, A covert operation against Chile comes to
    /// nothing.]" — Pakistan, Saudi Arabia and Iran in Afghanistan, Cuba in
    /// Chile, in registry order. The six OFF hashes are untouched; the
    /// on-world now tracks the off-world for the month before the arm's
    /// first act rather than thirty-five, which is what the ruling costs
    /// this bar and is recorded here rather than hidden.
    ///
    /// D4 WIRED 2026-09-06: the government clause compares `support` for
    /// every nation except the two whose ON-world table is the D4 block
    /// (`polity_in` differs from `polity`), because seating a table the OFF
    /// world cannot see is the wiring itself and not a leak; coalition and
    /// pillars are still compared for them. MEASURED before the clause:
    /// "parted at month 0 with no stem: [rng, Nepal government, Haiti
    /// government, news [...]]"; with it the six OFF hashes are unmoved.
    /// R1 (2026-09-06, Ridge's ruling): "WIRE their transcribed tables
    /// (D4_POLITIES beside POLITIES, commit 5086cfa reverted in 62af57b and
    /// cherry-pickable) under the ideology_blocs switch so the default path
    /// is byte-identical; the ONE existing test clause that must narrow to
    /// admit it (the government clause of the_bloc_layer_is_inert_over_time,
    /// for those two nations, switched-on state only) is re-expressed and
    /// the narrowing is recorded in the test comment and BUGS with this
    /// ruling quoted." On this branch the first act is month 0 (M1 above),
    /// so the clause is reached only in that month; it stands as ruled.
    #[test]
    fn the_bloc_layer_is_inert_over_time() {
        const BASE: [u64; 6] = [
            0x8834ad709d4bf805,
            0xbd3f3e335fb6161c,
            0x9120a2ff805b184f,
            0xde17f4fdef2c0d7f,
            0xeb92ae6a6418b12e,
            0xef75c8dcbe4335b5,
        ];
        for seed in 0..6u64 {
            let mut w = world_1990(GameRules { seed, ..GameRules::default() });
            run_months(&mut w, 240);
            let h = state_hash(&w);
            assert_eq!(h, BASE[seed as usize], "seed {seed} moved (actual {h:#018x})");
        }
        const FIRST_ACT: (usize, &str) = (0, "the Islamist movement in Afghanistan");
        const STEMS: [&str; 4] =
            ["Money and organisers reach", "backing the", "has no party to carry it", "comes to nothing"];
        let mut off = world_1990(GameRules::default());
        let mut on = world_1990(on(1990));
        off.player = Some(NationId::Libya);
        on.player = Some(NationId::Libya);
        let mut parted: Option<usize> = None;
        for month in 0..240 {
            let off_news = tick_month(&mut off, &[]);
            let on_news = tick_month(&mut on, &[]);
            let mut why: Vec<String> = vec![];
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
                    if x.to_bits() != y.to_bits() {
                        why.push(format!("{} {name}: {x} vs {y}", a.id.code()));
                    }
                }
            }
            if off.rng.state != on.rng.state {
                why.push("rng".into());
            }
            for (a, b) in off.governments.states.iter().zip(&on.governments.states) {
                // D4 (wired 2026-09-06): the ON world seats the Nepal and
                // Haiti tables that `polity_in` serves only under the
                // switch, so their dormant `support` differs from the OFF
                // world's empty one by construction — a record, not an act,
                // until the seam makes them electoral (auth < 0.60), which
                // is an act with its own stem. Coalition and pillars are
                // still compared for them; support is compared everywhere
                // else.
                let d4 = government::polity_in(&on, a.nation).map(|p| p as *const government::Polity)
                    != government::polity(a.nation).map(|p| p as *const government::Polity);
                if (!d4 && a.support != b.support) || a.coalition != b.coalition || a.pillars != b.pillars {
                    why.push(format!("{} government", a.nation.code()));
                }
            }
            let on_only: Vec<&String> = on_news.iter().filter(|h| !off_news.contains(h)).collect();
            if !on_only.is_empty() {
                why.push(format!("news {on_only:?}"));
            }
            if why.is_empty() {
                continue;
            }
            // Succession (D2, S4) is the arm's RECORD and not an act on the
            // model: a term limit reached (Honduras's 1990-01-27 in month 1)
            // seats a description and prints "is led by", and nothing the
            // model reads moves. A month whose only difference is such
            // lines is not a parting. Watched: with the lines counted as
            // acts, "parted at month 1 with no stem".
            let record_only = why.len() == 1
                && why[0].starts_with("news ")
                && on_only.iter().all(|h| h.contains(" is led by "));
            if record_only {
                continue;
            }
            // Mortality (D1, S4) is the arm's one draw: every January the
            // switched-on world draws once per living transcribed leader,
            // LAST in the month, so the only thing that differs at the
            // month's end is the RNG state (and, on a death, the record).
            // The stream is re-synced here so the comparison goes on; a
            // leak of the draw into the model — anything but the RNG and
            // those lines — is still a parting. Watched: without the
            // re-sync, "parted at month 0 with no stem: [rng, news [Honduras
            // is led by ..., Yitzhak Shamir dies in office., ...]]".
            let january = month % 12 == 0;
            let draw_only = january
                && why.iter().all(|x| x == "rng" || x.starts_with("news "))
                && on_only.iter().all(|h| h.contains(" dies in office.") || h.contains(" is led by "));
            if draw_only {
                on.rng.state = off.rng.state;
                continue;
            }
            let stems: Vec<&&String> =
                on_only.iter().filter(|h| STEMS.iter().any(|s| h.contains(s))).collect();
            assert!(!stems.is_empty(), "parted at month {month} with no stem: {why:?}");
            assert_eq!(month, FIRST_ACT.0, "the first act moved: {stems:?}");
            assert!(stems.iter().any(|h| h.contains(FIRST_ACT.1)), "the first act is another's: {stems:?}");
            parted = Some(month);
            break;
        }
        assert!(parted.is_some(), "the arm never acted in twenty years");
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
        assert_eq!(g.movements, flat_seed(&w, NationId::China, Bloc::Communist).to_vec());
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
                    // Influence is shares plus backing by definition; with
                    // nothing put in, backing is patronage gravity alone,
                    // zero at the start and a view of the aid flows later.
                    let infl = influence(&w, id);
                    let back = backing(&w, id);
                    for i in 0..5 {
                        assert_eq!(infl[i].1.to_bits(), (shares[i].1 + back[i].1).to_bits(), "{}", id.code());
                        assert!(back[i].1 >= 0.0 && back[i].1 <= crate::statecraft::BACKING_TOTAL_CAP);
                        if pass == 0 {
                            assert_eq!(back[i].1, 0.0, "{}: backing at the start", id.code());
                        }
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
            emergent: None,
        });
        assert_eq!(leader_row(&w, id), None);
        assert_eq!(leader_bloc(&w, id), None);
        assert_eq!(ruling_bloc(&w, id), Some(Bloc::Islamist));
    }

    /// Every road reads closed with "calibration pending" while
    /// `rules.ideology_takeover` is off — the lens on or off — and the
    /// gauges carry the design's triggers. Re-expressed from "every road
    /// reads closed in this build" when S4 landed the mechanics behind the
    /// switch: the OFF reading is the same bar under the design's new word,
    /// and the ON readings have their own test below. Watched red (S1) with
    /// the collapse road served open; watched red (S4) with the coup road's
    /// `None` reason served regardless of the switch: "USA: a road is open".
    #[test]
    fn every_road_reads_closed_while_takeover_is_off() {
        for rules in [GameRules::default(), on(1990)] {
            let w = world_1990(rules);
            assert!(!w.rules.ideology_takeover);
            for id in alive(&w) {
                let t = takeover_readout(&w, id);
                for road in t.roads() {
                    assert!(!road.open, "{}: a road is open", id.code());
                    assert_eq!(road.reason, CALIBRATION_PENDING);
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
                assert!(!r.open && r.reason == CALIBRATION_PENDING, "{}: the takeover switch is off", id.code());
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

    // -----------------------------------------------------------------------
    // Foreign backing (S3)
    // -----------------------------------------------------------------------

    fn back(w: &WorldState, id: NationId, b: Bloc) -> f64 {
        backing(w, id)[b as usize].1
    }
    fn stock(w: &WorldState, id: NationId, b: Bloc) -> f64 {
        backing_stock(w, id)[b as usize].1
    }

    /// The arm: two clean operations reach the per-sponsor cap (0.06, 0.12,
    /// then nothing), and three sponsors reach the bloc's total cap (USA
    /// 0.12 + USSR 0.12 + UK 0.01 = 0.25, the UK's second op adds nothing).
    /// The stock stays sorted by (sponsor, target, bloc), influence reads
    /// shares plus backing, and `backing_room` quotes exactly what
    /// `add_backing` then charges. Then through the command: a `BackBloc`
    /// draws exactly the two `chance` rolls the other ops draw and no
    /// third, and a clean one lands 0.06 with its headline. Watched red with
    /// the sponsor cap dropped from `backing_room`: the third USA op read
    /// 0.18.
    #[test]
    fn two_clean_ops_reach_the_sponsor_cap_and_three_sponsors_the_total_cap() {
        use crate::statecraft::{add_backing, backing_room, BACKING_SPONSOR_CAP, BACKING_STEP, BACKING_TOTAL_CAP};
        let mut w = world_1990(on(7));
        let (usa, ussr, uk, pl) = (NationId::USA, NationId::USSR, NationId::UK, NationId::Poland);
        let b = Bloc::Communist;
        assert_eq!(ruling_bloc(&w, pl), Some(Bloc::Western));
        assert_eq!(back(&w, pl, b), 0.0);
        for expected in [BACKING_STEP, BACKING_SPONSOR_CAP, BACKING_SPONSOR_CAP] {
            let quoted = backing_room(&w, usa, pl, b);
            let added = add_backing(&mut w, usa, pl, b);
            assert_eq!(quoted, added, "the card and the arm disagree");
            assert!((w.backing_of(usa, pl, b) - expected).abs() < 1e-12, "{}", w.backing_of(usa, pl, b));
        }
        assert_eq!(add_backing(&mut w, usa, pl, b), 0.0, "a capped sponsor adds nothing");
        add_backing(&mut w, ussr, pl, b);
        add_backing(&mut w, ussr, pl, b);
        assert!((back(&w, pl, b) - 0.24).abs() < 1e-12);
        let last = add_backing(&mut w, uk, pl, b);
        assert!((last - 0.01).abs() < 1e-12, "the total cap clipped the UK to {last}");
        assert_eq!(add_backing(&mut w, uk, pl, b), 0.0);
        assert!((back(&w, pl, b) - BACKING_TOTAL_CAP).abs() < 1e-12);
        assert!((stock(&w, pl, b) - BACKING_TOTAL_CAP).abs() < 1e-12);
        let keys: Vec<(NationId, NationId, Bloc)> =
            w.statecraft.backing.iter().map(|e| (e.sponsor, e.target, e.bloc)).collect();
        let mut sorted = keys.clone();
        sorted.sort();
        assert_eq!(keys, sorted, "the stock is not sorted");
        assert_eq!(keys.len(), 3);
        let shares = bloc_shares(&w, pl);
        let infl = influence(&w, pl);
        for i in 0..5 {
            assert_eq!(infl[i].1.to_bits(), (shares[i].1 + back(&w, pl, shares[i].0)).to_bits());
        }
        // The command. Exactly two draws, a fixed effect, and the headline.
        let mut w = world_1990(on(7));
        w.rules.ai_aggression = 0.0;
        let mut landed = 0;
        for _ in 0..40 {
            let before = w.backing_of(usa, pl, b);
            let mut probe = w.rng.clone();
            probe.next_u64();
            probe.next_u64();
            w.headlines.clear();
            crate::statecraft::covert_action(&mut w, usa, pl, CovertOp::BackBloc(b)).expect("not refused");
            assert_eq!(w.rng.state, probe.state, "a BackBloc drew other than the two rolls");
            let after = w.backing_of(usa, pl, b);
            let clean = w.headlines.iter().any(|h| {
                h == "Money and organisers reach the Communist movement in Poland; nobody can say from where."
            });
            // The two rolls are independent: an op can land AND be caught in
            // the same month, in which case the step lands and is then halved.
            let caught = w.headlines.iter().any(|h| h.contains("exposes United States"));
            let step = if before >= BACKING_SPONSOR_CAP - 1e-12 { 0.0 } else { BACKING_STEP.min(BACKING_SPONSOR_CAP - before) };
            let expected = match (clean, caught) {
                (true, false) => before + step,
                (true, true) => (before + step) * 0.5,
                (false, true) => before * 0.5,
                (false, false) => before,
            };
            if clean {
                landed += 1;
            }
            assert!((after - expected).abs() < 1e-12, "{before} -> {after} (clean {clean}, caught {caught})");
        }
        assert!(landed > 0, "forty operations and none landed");
        assert!(w.backing_of(usa, pl, b) <= BACKING_SPONSOR_CAP + 1e-12);
    }

    /// Backing never enters support. Poland (electoral) and China (a regime)
    /// carry 0.12 of Communist and Western backing respectively; every party's
    /// support and every movement is bit-identical to an unbacked twin before
    /// and after 36 calls of the two drift functions — the only writers of
    /// support and movements in the tick — while influence differs by exactly
    /// the backing. Watched red with `bloc_shares` adding `backing` for a
    /// regime: China's Communist share moved in the first call.
    #[test]
    fn backing_never_enters_support() {
        use crate::government::{drift_movements, drift_support};
        use crate::statecraft::add_backing;
        let (usa, pl, cn) = (NationId::USA, NationId::Poland, NationId::China);
        let mut backed = world_1990(on(7));
        let clean = backed.clone();
        add_backing(&mut backed, usa, pl, Bloc::Communist);
        add_backing(&mut backed, usa, pl, Bloc::Communist);
        add_backing(&mut backed, usa, cn, Bloc::Western);
        add_backing(&mut backed, usa, cn, Bloc::Western);
        assert!((back(&backed, pl, Bloc::Communist) - 0.12).abs() < 1e-12);
        assert!((back(&backed, cn, Bloc::Western) - 0.12).abs() < 1e-12);
        let bits = |w: &WorldState, id: NationId| -> Vec<(String, u64)> {
            let g = government::state(w, id).unwrap();
            let mut v: Vec<(String, u64)> = g.support.iter().map(|(p, s)| (p.clone(), s.to_bits())).collect();
            v.extend(g.movements.iter().map(|(b, s)| (format!("{b:?}"), s.to_bits())));
            v
        };
        let mut backed = backed;
        let mut clean = clean;
        for _ in 0..36 {
            assert_eq!(bits(&backed, pl), bits(&clean, pl), "Poland's support parted");
            assert_eq!(bits(&backed, cn), bits(&clean, cn), "China's movements parted");
            for id in [pl, cn] {
                let sb = bloc_shares(&backed, id);
                let sc = bloc_shares(&clean, id);
                let ib = influence(&backed, id);
                let ic = influence(&clean, id);
                for i in 0..5 {
                    assert_eq!(sb[i].1.to_bits(), sc[i].1.to_bits(), "{} shares", id.code());
                    let expected = ic[i].1 + back(&backed, id, sb[i].0);
                    assert!((ib[i].1 - expected).abs() < 1e-15, "{} influence", id.code());
                }
            }
            drift_support(&mut backed, pl);
            drift_support(&mut clean, pl);
            drift_movements(&mut backed, cn);
            drift_movements(&mut clean, cn);
        }
        assert!(back(&backed, cn, Bloc::Western) > 0.0, "the backing was still there");
    }

    /// Exposure halves every entry of the sponsor's backing in the target and
    /// marks it exposed, leaves other sponsors' entries alone, and taints the
    /// bloc 0.02 of support: off Poland's Communist parties by size, off
    /// China's Western movement — both then renormalised, so the bloc reads
    /// (s - 0.02) / 0.98. Then through the command, until the target catches
    /// the sponsor: that month the halving and the naming happen and the
    /// headline names sponsor and bloc. Watched red with `expose_backing`
    /// not called from the exposure branch: the caught month read 0.054
    /// against the 0.024 a halving and one month's cooling leave.
    #[test]
    fn exposure_halves_backing_and_taints_the_bloc() {
        use crate::statecraft::{add_backing, expose_backing, EXPOSURE_TAINT};
        let (usa, ussr, pl, cn) = (NationId::USA, NationId::USSR, NationId::Poland, NationId::China);
        let mut w = world_1990(on(7));
        add_backing(&mut w, usa, pl, Bloc::Communist);
        add_backing(&mut w, usa, pl, Bloc::Communist);
        add_backing(&mut w, usa, pl, Bloc::NonAligned);
        add_backing(&mut w, ussr, pl, Bloc::Communist);
        add_backing(&mut w, usa, cn, Bloc::Western);
        let pl_before = bloc_shares(&w, pl)[Bloc::Communist as usize].1;
        let cn_before = bloc_shares(&w, cn)[Bloc::Western as usize].1;
        let bits_before: Vec<u64> = government::state(&w, cn).unwrap().support.iter().map(|(_, s)| s.to_bits()).collect();
        expose_backing(&mut w, usa, pl, Bloc::Communist);
        assert!((w.backing_of(usa, pl, Bloc::Communist) - 0.06).abs() < 1e-12);
        assert!((w.backing_of(usa, pl, Bloc::NonAligned) - 0.03).abs() < 1e-12, "every entry of the sponsor halves");
        assert!((w.backing_of(ussr, pl, Bloc::Communist) - 0.06).abs() < 1e-12, "another sponsor's entry moved");
        assert!((w.backing_of(usa, cn, Bloc::Western) - 0.06).abs() < 1e-12, "another target's entry moved");
        for e in &w.statecraft.backing {
            assert_eq!(e.exposed, e.sponsor == usa && e.target == pl, "{e:?}");
        }
        let pl_after = bloc_shares(&w, pl)[Bloc::Communist as usize].1;
        assert!((pl_after - (pl_before - EXPOSURE_TAINT) / (1.0 - EXPOSURE_TAINT)).abs() < 1e-9, "{pl_before} -> {pl_after}");
        let sum: f64 = government::state(&w, pl).unwrap().support.iter().map(|(_, s)| *s).sum();
        assert!((sum - 1.0).abs() < 1e-9);
        expose_backing(&mut w, usa, cn, Bloc::Western);
        let cn_after = bloc_shares(&w, cn)[Bloc::Western as usize].1;
        assert!((cn_after - (cn_before - EXPOSURE_TAINT) / (1.0 - EXPOSURE_TAINT)).abs() < 1e-9, "{cn_before} -> {cn_after}");
        let bits_after: Vec<u64> = government::state(&w, cn).unwrap().support.iter().map(|(_, s)| s.to_bits()).collect();
        assert_eq!(bits_before, bits_after, "a regime's dormant table moved");
        // Through the command, until caught.
        let mut w = world_1990(on(7));
        w.rules.ai_aggression = 0.0;
        w.player = Some(usa);
        // Until the target catches the sponsor with a stock to halve: an
        // exposure with nothing yet landed has nothing to halve or to mark.
        let mut caught_at: Option<(f64, f64, bool)> = None;
        for _ in 0..120 {
            let before = w.backing_of(usa, pl, Bloc::Communist);
            let hl = crate::tick_month(
                &mut w,
                &[Command::CovertAction { sponsor: usa, target: pl, op: CovertOp::BackBloc(Bloc::Communist) }],
            );
            let caught = hl.iter().any(|h| h.contains("exposes United States backing the Communist movement in Poland"));
            let clean = hl.iter().any(|h| h.contains("Money and organisers reach the Communist movement in Poland"));
            if caught && (before > 0.0 || clean) {
                caught_at = Some((before, w.backing_of(usa, pl, Bloc::Communist), clean));
                break;
            }
        }
        let (before, after, clean) = caught_at.expect("a hundred and twenty operations and never once caught with a stock");
        // Halved after this month's own clean step, if any, then cooled once
        // by the month's own decay.
        let stepped = if clean { (before + 0.06).min(0.12) } else { before };
        let expected = stepped * 0.5 - crate::statecraft::BACKING_DECAY;
        assert!((after - expected).abs() < 1e-9, "{before} -> {after} (clean {clean}), expected {expected}");
        assert!(w.statecraft.backing.iter().any(|e| e.sponsor == usa && e.target == pl && e.exposed));
    }

    /// Patronage gravity is a view: an aid flow puts 0.10 * min(1,
    /// infusion / 0.10) of the patron's ruling bloc behind the client, read
    /// off the same output ratio the flow itself reads; it is there the month
    /// after; and it is gone the month the flow stops. The stock plus gravity
    /// is capped at the bloc's 0.25 together. Watched red with `gravity`
    /// returning zero shares: the small pledge read 0 against 0.0278.
    #[test]
    fn gravity_appears_with_an_aid_flow_and_vanishes_the_month_it_stops() {
        use crate::statecraft::{add_backing, BACKING_TOTAL_CAP};
        use crate::{apply_command, Command};
        let (usa, eg) = (NationId::USA, NationId::Egypt);
        let mut w = world_1990(on(7));
        w.rules.ai_aggression = 0.0;
        w.player = Some(usa);
        assert_eq!(ruling_bloc(&w, usa), Some(Bloc::Western));
        assert_eq!(back(&w, eg, Bloc::Western), 0.0);
        apply_command(&mut w, &Command::PledgeAid { patron: usa, client: eg, kind: AidKind::Economic, share_gdp: 0.0002 })
            .expect("pledged");
        let infusion = w.nation(usa).gdp * 0.0002 / w.nation(eg).gdp.max(0.1);
        assert!(infusion < GRAVITY_FULL_AT, "the small pledge is meant to sit under the knee: {infusion}");
        let expected = GRAVITY_WEIGHT * (infusion / GRAVITY_FULL_AT);
        assert!((back(&w, eg, Bloc::Western) - expected).abs() < 1e-12, "{} vs {expected}", back(&w, eg, Bloc::Western));
        assert_eq!(stock(&w, eg, Bloc::Western), 0.0, "gravity is not a stock");
        apply_command(&mut w, &Command::PledgeAid { patron: usa, client: eg, kind: AidKind::Economic, share_gdp: 0.004 })
            .expect("raised");
        assert!((back(&w, eg, Bloc::Western) - GRAVITY_WEIGHT).abs() < 1e-12, "past the knee the weight is the full 0.10");
        tick_month(&mut w, &[]);
        assert!((back(&w, eg, Bloc::Western) - GRAVITY_WEIGHT).abs() < 1e-12, "the month after");
        // M1 (2026-09-06): was `w.statecraft.backing.is_empty()`. The
        // sponsors' standing programme now acts in month 0 (Pakistan in
        // Afghanistan), so the WHOLE store is no longer empty after a
        // month on this seed; the clause's own claim — that the flow's
        // gravity was not stored — is what is asserted. Narrowed, not
        // widened: MEASURED red before the change: "nothing was stored".
        assert_eq!(stock(&w, eg, Bloc::Western), 0.0, "nothing was stored for the flow");
        assert!(!w.statecraft.backing.iter().any(|e| e.target == eg), "nothing was stored in Egypt");
        // Stock and gravity share one cap.
        for _ in 0..4 {
            add_backing(&mut w, NationId::UK, eg, Bloc::Western);
            add_backing(&mut w, NationId::France, eg, Bloc::Western);
        }
        assert!((stock(&w, eg, Bloc::Western) - 0.24).abs() < 1e-12);
        assert!((back(&w, eg, Bloc::Western) - BACKING_TOTAL_CAP).abs() < 1e-12, "{}", back(&w, eg, Bloc::Western));
        w.statecraft.backing.clear();
        apply_command(&mut w, &Command::EndAid { patron: usa, client: eg, kind: AidKind::Economic }).expect("ended");
        assert_eq!(back(&w, eg, Bloc::Western), 0.0, "gone the moment the flow stops");
        tick_month(&mut w, &[]);
        assert_eq!(back(&w, eg, Bloc::Western), 0.0);
    }

    /// The refusals, from the one place the prose lives, read by
    /// `refusal_of` and given by `apply_command` word for word: the switch
    /// first (before any state, and no state or RNG touched by the refusal),
    /// then the target's own ruling bloc. `CovertOp::parse` gains
    /// `back:<bloc>` and keeps `coup` on FundOpposition. Watched red with
    /// the `BackBloc` arm dropped from `world_refusal`: the off-world read
    /// `None` against the sim's sentence.
    #[test]
    fn back_bloc_is_refused_off_and_against_the_ruling_bloc() {
        use crate::{apply_command, refusal_of, Command};
        let (usa, pl) = (NationId::USA, NationId::Poland);
        let off = world_1990(GameRules::default());
        let c = Command::CovertAction { sponsor: usa, target: pl, op: CovertOp::BackBloc(Bloc::Communist) };
        let before = (state_hash(&off), off.rng.state);
        let read = refusal_of(&off, &c);
        assert_eq!(read.as_deref(), Some("This world does not model ideological movements."));
        let mut trial = off.clone();
        assert_eq!(apply_command(&mut trial, &c).err(), read);
        assert_eq!((state_hash(&trial), trial.rng.state), before, "a refused op touched the world");
        let on = world_1990(on(7));
        let c = Command::CovertAction { sponsor: usa, target: pl, op: CovertOp::BackBloc(Bloc::Western) };
        let read = refusal_of(&on, &c);
        assert_eq!(read.as_deref(), Some("You cannot back a government covertly — send aid."));
        assert_eq!(apply_command(&mut on.clone(), &c).err(), read);
        let c = Command::CovertAction { sponsor: usa, target: pl, op: CovertOp::BackBloc(Bloc::Communist) };
        assert_eq!(refusal_of(&on, &c), None);
        assert!(apply_command(&mut on.clone(), &c).is_ok());
        assert_eq!(CovertOp::parse("back:communist"), Some(CovertOp::BackBloc(Bloc::Communist)));
        assert_eq!(CovertOp::parse("back:non-aligned"), Some(CovertOp::BackBloc(Bloc::NonAligned)));
        assert_eq!(CovertOp::parse("back:martian"), None);
        assert_eq!(CovertOp::parse("coup"), Some(CovertOp::FundOpposition));
        assert_eq!(CovertOp::BackBloc(Bloc::Islamist).label(), "backing the Islamist movement");
        // The card's arms quote the clamped room.
        let arms = crate::statecraft::back_bloc_effects(&on, usa, pl, Bloc::Communist);
        assert!(arms[0].contains("0.06 realised now"), "{}", arms[0]);
    }

    /// The AI covert arm backs a bloc only under its conditions, read off
    /// the pure `ai_back_bloc_choice`: `None` with the switch off whatever
    /// the state; never the target's ruling bloc; never a bloc under 0.15 of
    /// influence (Egypt's Islamist reads 0.1327 on the flat seed, and 0.06
    /// of backing lifts it over); a patron backs its own bloc anywhere, and
    /// any qualifying bloc only in a rival's client; an ideological sponsor
    /// backs its transcribed bloc and nothing else, even in a rival's client
    /// with a stronger movement. The flag is on exactly five nations and is
    /// not in `patrons()`. Then a twenty-year run on seed 7 with the switch
    /// on, ai_aggression default: every stored entry is inside both caps and
    /// its sponsor is a patron or an ideological sponsor, printed as
    /// measured; the same run with the switch off stores nothing. Watched
    /// red with the 0.15 line removed: Egypt read Some(Islamist) at 0.1327.
    #[test]
    fn the_ai_backs_a_bloc_only_under_its_conditions() {
        use crate::nations::{ideological_sponsors, patrons};
        use crate::politics::ai_back_bloc_choice;
        use crate::statecraft::{add_backing, pledge_aid};
        let (usa, ussr, iran, pl, eg) = (NationId::USA, NationId::USSR, NationId::Iran, NationId::Poland, NationId::Egypt);
        assert_eq!(
            ideological_sponsors().iter().map(|n| n.code()).collect::<Vec<_>>(),
            ["Pakistan", "SaudiArabia", "Iran", "Cuba", "Libya"],
            "registry order"
        );
        assert_eq!(iran.def().ideological_sponsor, Some(Bloc::Islamist));
        assert_eq!(NationId::Libya.def().ideological_sponsor, Some(Bloc::Nationalist));
        assert_eq!(NationId::Cuba.def().ideological_sponsor, Some(Bloc::Communist));
        assert_eq!(usa.def().ideological_sponsor, None);
        for s in ideological_sponsors() {
            assert!(!patrons().contains(s), "{} leaked into patrons()", s.code());
        }
        // Off: nothing, whatever the state.
        let off = world_1990(GameRules::default());
        assert_eq!(ai_back_bloc_choice(&off, ussr, pl), None);
        // On. Poland rules Western with the Communists at 0.22: the USSR
        // (Communist) backs its own bloc there; the USA (Western) has no bloc
        // to back — Western rules and Poland is nobody's client.
        let mut w = world_1990(on(7));
        assert_eq!(ruling_bloc(&w, pl), Some(Bloc::Western));
        assert!(influence(&w, pl)[Bloc::Communist as usize].1 >= 0.15);
        assert_eq!(ai_back_bloc_choice(&w, ussr, pl), Some(Bloc::Communist));
        assert_eq!(ai_back_bloc_choice(&w, usa, pl), None, "the ruling bloc is never backed");
        // Make Poland a Soviet client the USA is at odds with: now any
        // qualifying bloc will do for the USA, and the Communists qualify.
        pledge_aid(&mut w, ussr, pl, AidKind::Economic, 0.002).expect("pledged");
        w.set_relation(usa, ussr, -40.0);
        assert!(w.patrons_of(pl).contains(&ussr));
        assert_eq!(ai_back_bloc_choice(&w, usa, pl), Some(Bloc::Communist));
        // Egypt: Non-Aligned rules, Islamist present at 0.1327 — under the
        // line for Iran until backing lifts it.
        let islamist = influence(&w, eg)[Bloc::Islamist as usize].1;
        assert!((islamist - 0.1327).abs() < 1e-3, "{islamist}");
        assert_eq!(ai_back_bloc_choice(&w, iran, eg), None, "under the line at {islamist}");
        add_backing(&mut w, NationId::SaudiArabia, eg, Bloc::Islamist);
        assert!(influence(&w, eg)[Bloc::Islamist as usize].1 >= 0.15);
        assert_eq!(ai_back_bloc_choice(&w, iran, eg), Some(Bloc::Islamist));
        // An ideological sponsor backs its own bloc only: Egypt as a US
        // client Iran is at odds with, its Western movement backed to the
        // cap, still reads Islamist for Iran — and Western for the USSR, a
        // patron in a rival's client, whose own bloc is absent there.
        pledge_aid(&mut w, usa, eg, AidKind::Economic, 0.002).expect("pledged");
        w.set_relation(iran, usa, -60.0);
        w.set_relation(ussr, usa, -60.0);
        for _ in 0..4 {
            add_backing(&mut w, NationId::UK, eg, Bloc::Western);
            add_backing(&mut w, NationId::France, eg, Bloc::Western);
        }
        let west = influence(&w, eg)[Bloc::Western as usize].1;
        assert!(west > influence(&w, eg)[Bloc::Islamist as usize].1, "{west}");
        assert_eq!(ai_back_bloc_choice(&w, iran, eg), Some(Bloc::Islamist));
        assert_eq!(ai_back_bloc_choice(&w, ussr, eg), Some(Bloc::Western), "the strongest qualifying bloc in a rival's client");
        // The invariant over a run, and the count for the record.
        let mut on_w = world_1990(on(7));
        let mut landed = 0usize;
        for month in 0..240 {
            let news = tick_month(&mut on_w, &[]);
            landed += news.iter().filter(|h| h.starts_with("Money and organisers reach")).count();
            for e in &on_w.statecraft.backing {
                assert!(e.weight > 0.0 && e.weight <= crate::statecraft::BACKING_SPONSOR_CAP + 1e-12, "month {month}: {e:?}");
                assert!(
                    patrons().contains(&e.sponsor) || ideological_sponsors().contains(&e.sponsor),
                    "month {month}: {e:?}"
                );
            }
            for id in alive(&on_w) {
                for (b, v) in backing_stock(&on_w, id) {
                    assert!(v <= crate::statecraft::BACKING_TOTAL_CAP + 1e-12, "month {month} {} {b:?} {v}", id.code());
                }
            }
        }
        println!("AI backings landed in twenty years on seed 7: {landed}; entries at the end: {}", on_w.statecraft.backing.len());
        let mut off_w = world_1990(GameRules { seed: 7, ..GameRules::default() });
        run_months(&mut off_w, 240);
        assert!(off_w.statecraft.backing.is_empty(), "the switch was off");
    }

    /// M1 (Ridge's ruling, 2026-09-06). Afghanistan's transcribed polity
    /// carries no Religious party and no Clergy pillar, so the Islamist bloc
    /// is ABSENT there by table and, before M1, nothing could ever put it
    /// there (BUGS S5-5: the design's own Afghan case measured 0/200). Now:
    /// Pakistan (an Islamist sponsor at -70 with Kabul) and Saudi Arabia
    /// (-55) have a choice there under the creating clause; a sponsor the
    /// target is not hostile to does not; one clean Pakistani operation
    /// (`BACKING_STEP` 0.06 ≥ `PRESENCE_BACKING`) makes the bloc PRESENT and
    /// one that COULD WIN; the created presence keeps receiving (absent from
    /// the table is what the clause reads) until the sponsor's cap holds it;
    /// and the AI arm's own target choice names Afghanistan for Pakistan on
    /// the 1990 board. Off: absent, and no choice. Watched red with the
    /// creating clause removed from `ai_back_bloc_choice` ("the sponsor has
    /// no choice in a target where its bloc is absent") and with the backing
    /// clause removed from `bloc_present` ("one operation did not create
    /// the presence").
    #[test]
    fn an_absent_islamist_bloc_becomes_present_after_one_sponsor_operation() {
        use crate::politics::{ai_back_bloc_choice, best_covert_target, HOSTILE_TARGET};
        use crate::statecraft::{add_backing, BACKING_SPONSOR_CAP, BACKING_STEP};
        let (af, pk, sa) = (NationId::Afghanistan, NationId::Pakistan, NationId::SaudiArabia);
        assert!(!bloc_in_table(&world_1990(on(7)), af, Bloc::Islamist), "the table carries no Islamist party or pillar");
        let off = world_1990(GameRules::default());
        assert!(!bloc_present(&off, af, Bloc::Islamist));
        assert_eq!(ai_back_bloc_choice(&off, pk, af), None, "off: no choice");
        let mut w = world_1990(on(7));
        assert_eq!(ruling_bloc(&w, af), Some(Bloc::Communist));
        assert!(!bloc_present(&w, af, Bloc::Islamist));
        assert!(!bloc_can_win(&w, af, Bloc::Islamist));
        assert!(w.relation(pk, af) <= HOSTILE_TARGET, "{}", w.relation(pk, af));
        assert!(w.relation(sa, af) <= HOSTILE_TARGET, "{}", w.relation(sa, af));
        assert_eq!(ai_back_bloc_choice(&w, pk, af), Some(Bloc::Islamist), "the creating clause");
        assert_eq!(ai_back_bloc_choice(&w, sa, af), Some(Bloc::Islamist));
        // Not hostile, not a rival's client: no clause.
        let was = w.relation(pk, af);
        w.set_relation(pk, af, 0.0);
        assert_eq!(ai_back_bloc_choice(&w, pk, af), None, "a sponsor Kabul is not hostile to has no choice");
        w.set_relation(pk, af, was);
        // The arm's own target choice on the 1990 board.
        let target = best_covert_target(&w, pk);
        println!("Pakistan's best covert target in January 1990: {target:?}");
        assert_eq!(target, Some(af), "the arm would take the first operation in Afghanistan");
        // One clean operation creates the presence.
        let added = add_backing(&mut w, pk, af, Bloc::Islamist);
        assert_eq!(added, BACKING_STEP);
        assert!(added >= PRESENCE_BACKING);
        assert!(bloc_backed(&w, af, Bloc::Islamist));
        assert!(bloc_present(&w, af, Bloc::Islamist), "one operation created the presence");
        assert!(bloc_can_win(&w, af, Bloc::Islamist));
        assert!(!bloc_in_table(&w, af, Bloc::Islamist), "the table is untouched");
        // One operation makes it a bloc that could win, not yet the
        // challenger: the Nationalist flat-seed share (0.133) outweighs 0.06.
        let (chal, ci) = challenger(&w, af).unwrap();
        assert_eq!(chal, Bloc::Nationalist, "{ci}");
        // The created presence keeps receiving until the sponsor's cap.
        assert_eq!(ai_back_bloc_choice(&w, pk, af), Some(Bloc::Islamist), "the sponsor keeps paying");
        assert_eq!(add_backing(&mut w, pk, af, Bloc::Islamist), BACKING_SPONSOR_CAP - BACKING_STEP);
        assert_eq!(add_backing(&mut w, pk, af, Bloc::Islamist), 0.0, "the sponsor's cap holds");
        assert_eq!(ai_back_bloc_choice(&w, sa, af), Some(Bloc::Islamist), "a second sponsor still has room");
        add_backing(&mut w, sa, af, Bloc::Islamist);
        add_backing(&mut w, sa, af, Bloc::Islamist);
        add_backing(&mut w, NationId::Iran, af, Bloc::Islamist);
        let i = influence(&w, af)[Bloc::Islamist as usize].1;
        assert!(i > ci, "{i} against the Nationalist {ci}");
        assert_eq!(challenger(&w, af).map(|(b, _)| b), Some(Bloc::Islamist), "three sponsors at the bloc cap make it the challenger");
        // The flat seed the drift reverts to now carries the bloc.
        let seed = flat_seed(&w, af, Bloc::Communist);
        assert!(seed[Bloc::Islamist as usize].1 > SHARE_FLOOR, "{seed:?}");
        // And the readout, over a run: how often the arm lands it.
        let mut on_w = world_1990(on(7));
        let mut afghan = 0usize;
        let mut first: Option<usize> = None;
        for month in 0..240 {
            for h in tick_month(&mut on_w, &[]) {
                if h == "Money and organisers reach the Islamist movement in Afghanistan; nobody can say from where." {
                    afghan += 1;
                    first.get_or_insert(month);
                }
            }
        }
        println!("seed 7: Islamist backings landed in Afghanistan in twenty years {afghan}, first in month {first:?}");
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

    /// The Security Crackdown's gated arm. Poland (Western) carries USSR
    /// 0.12 and China 0.06 behind the Communists, the UK 0.06 behind the
    /// Non-Aligned and — hand-written, since the command refuses it — the
    /// USA 0.06 behind the ruling Western bloc. The crackdown halves the
    /// three non-ruling entries (0.06, 0.03, 0.03) and leaves the ruling
    /// one at 0.06; the card quoted exactly the realised F_B before and
    /// after (0.180 → 0.090 Communist, 0.060 → 0.030 Non-Aligned, no
    /// Western line); the enactment draws no RNG; and the off world's
    /// crackdown carries no such arm and no such card. Watched red with
    /// the arm's call dropped from the stratagem: the USSR entry's 0.06 bar
    /// failed, the entry not having been halved.
    #[test]
    fn a_crackdown_halves_the_foreign_backing_of_every_non_ruling_bloc() {
        use crate::statecraft::{add_backing, crackdown_backing_effects};
        use crate::{apply_command, refusal_of, Command};
        let (usa, ussr, china, uk, pl) = (NationId::USA, NationId::USSR, NationId::China, NationId::UK, NationId::Poland);
        let crack = Command::EnactStratagem { nation: pl, id: "security_crackdown".into() };
        let mut w = world_1990(on(7));
        assert_eq!(ruling_bloc(&w, pl), Some(Bloc::Western));
        add_backing(&mut w, ussr, pl, Bloc::Communist);
        add_backing(&mut w, ussr, pl, Bloc::Communist);
        add_backing(&mut w, china, pl, Bloc::Communist);
        add_backing(&mut w, uk, pl, Bloc::NonAligned);
        add_backing(&mut w, usa, pl, Bloc::Western);
        assert!((back(&w, pl, Bloc::Communist) - 0.18).abs() < 1e-12);
        {
            let n = w.nation_mut(pl);
            n.stability = 40.0;
            n.authoritarianism = 0.40;
            n.political_capital = 100.0;
        }
        assert_eq!(refusal_of(&w, &crack), None);
        let card = crackdown_backing_effects(&w, pl);
        assert_eq!(
            card,
            vec![
                "Foreign backing of the Communist movement halved: 0.180 → 0.090.".to_string(),
                "Foreign backing of the Non-Aligned movement halved: 0.060 → 0.030.".to_string(),
            ]
        );
        let rng_before = w.rng.state;
        apply_command(&mut w, &crack).expect("enacted");
        assert_eq!(w.rng.state, rng_before, "the crackdown drew");
        assert!((w.backing_of(ussr, pl, Bloc::Communist) - 0.06).abs() < 1e-12);
        assert!((w.backing_of(china, pl, Bloc::Communist) - 0.03).abs() < 1e-12);
        assert!((w.backing_of(uk, pl, Bloc::NonAligned) - 0.03).abs() < 1e-12);
        assert!((w.backing_of(usa, pl, Bloc::Western) - 0.06).abs() < 1e-12, "the ruling bloc's money is not raided");
        assert!((back(&w, pl, Bloc::Communist) - 0.09).abs() < 1e-12);
        assert!((back(&w, pl, Bloc::NonAligned) - 0.03).abs() < 1e-12);
        assert_eq!(w.nation(pl).stability, 56.0, "the old arms still land");
        assert!(w.headlines.iter().any(|h| h == "Poland moves against its own streets."));
        // Nothing to halve, nothing to say.
        let quiet = world_1990(on(7));
        assert!(crackdown_backing_effects(&quiet, pl).is_empty());
        // Off: no arm, no card, and the crackdown as it always was.
        let mut off = world_1990(GameRules::default());
        {
            let n = off.nation_mut(pl);
            n.stability = 40.0;
            n.authoritarianism = 0.40;
            n.political_capital = 100.0;
        }
        assert!(crackdown_backing_effects(&off, pl).is_empty());
        apply_command(&mut off, &crack).expect("enacted");
        assert!(off.statecraft.backing.is_empty());
        assert_eq!(off.nation(pl).stability, 56.0);
    }
    /// Repair (2026-09-06, the inertness skeptic's note): the backing stock
    /// cools on the switch, not on emptiness alone. A stock written into a
    /// world with the lens OFF (a save carried across a rules change) is
    /// left exactly as it was by twelve months of `statecraft::tick`; the
    /// same stock in the ON world cools 0.006 a month and is gone within
    /// ten. Watched red with the switch dropped from `backing_cools`: the
    /// off world's 0.06 read 0.0 after the year.
    #[test]
    fn a_backing_stock_does_not_cool_with_the_lens_off() {
        use crate::statecraft::BACKING_DECAY;
        use crate::world::Backing;
        let entry = Backing { sponsor: NationId::USSR, target: NationId::Poland, bloc: Bloc::Communist, weight: 0.06, exposed: false };
        let mut off = world_1990(GameRules::default());
        off.statecraft.backing.push(entry.clone());
        for _ in 0..12 {
            crate::statecraft::tick(&mut off);
        }
        assert_eq!(off.statecraft.backing, vec![entry.clone()], "the lens is off and the stock moved");
        let mut on_w = world_1990(on(7));
        on_w.statecraft.backing.push(entry.clone());
        crate::statecraft::tick(&mut on_w);
        assert!((on_w.statecraft.backing[0].weight - (0.06 - BACKING_DECAY)).abs() < 1e-12);
        for _ in 0..12 {
            crate::statecraft::tick(&mut on_w);
        }
        assert!(on_w.statecraft.backing.is_empty(), "{:?}", on_w.statecraft.backing);
    }
}
