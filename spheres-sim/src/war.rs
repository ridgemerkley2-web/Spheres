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
/// strike — is the hungriest, and that is the point: it is all ordnance and no
/// ground, so it spends its whole weight out of the magazine, which is exactly
/// why an air campaign cannot be sustained indefinitely. A campaign and an
/// occupation are heavier overall and spend more of that weight on fuel, people
/// and time, so they burn ordnance more slowly than the thing that is only
/// ordnance.
pub const BURN_BY_RUNG: [f64; 10] = [0.0, 0.0, 0.0, 0.0, 0.002, 0.005, 0.090, 0.055, 0.070, 0.035];

/// What it costs in political capital to *arrive* at each rung, summed over the
/// rungs crossed. Rung 8 costs 30 in total, which is exactly what declaring war
/// has always cost — that price is preserved rather than reinvented.
pub const ESCALATION_PRICE: [f64; 10] = [0.0, 0.0, 2.0, 6.0, 8.0, 10.0, 14.0, 20.0, 30.0, 40.0];

/// How much budget stands behind each point of force structure, normalised so
/// that 1.0 is the United States in 1990.
///
/// The single most load-bearing derived number in the war model, and three
/// different things read it: what share of an army can be abroad at once, what
/// that army can do when it gets there, and how fast its magazines refill. That
/// is not overloading — they are the same fact. An army with $3.3bn behind each
/// point has tankers, optics, precision munitions and the industry that makes
/// more of them; one with $0.46bn has men and rifles. Iraq's 1990 figure is
/// 0.15 against the United States' 1.10, and nobody typed either.
pub fn capital_intensity(w: &WorldState, id: NationId) -> f64 {
    match w.nation_opt(id) {
        Some(n) => ((n.gdp * n.mil_spend_gdp) / n.mil_strength.max(1.0) / 3.0).clamp(0.0, 1.2),
        None => 0.0,
    }
}

/// The number BIBLE §6 says does more work than anything else in the model: the
/// share of its force structure a nation can have abroad and sustained at once.
///
/// Derived, never authored. A country-named special case would mean the model is
/// wrong (§7), so this reads only quantities the sim already keeps: capital
/// intensity, which is what buys airlift, sealift, tankers and the logistics
/// behind them, lifted by what the nation technologically knows how to do with
/// it. Against the transcribed 1990 data that lands the United States near 0.15,
/// the Soviet Union near 0.10, Iraq under 0.04 and Vietnam at the floor.
pub fn deployable_fraction(w: &WorldState, id: NationId) -> f64 {
    let n = match w.nation_opt(id) {
        Some(n) => n,
        None => return 0.02,
    };
    let capital = capital_intensity(w, id);
    let lift = 0.30 + 0.70 * ((crate::tech::military_multiplier(n) - 0.5) / 3.5).clamp(0.0, 1.0);
    (0.02 + 0.30 * capital * lift).clamp(0.02, 0.40)
}

/// How fast a month of peace puts ordnance back in the magazines, at a nation of
/// average capital intensity. Scaled by that intensity at the call site, because
/// what refills a magazine is not the size of an army but the industry standing
/// behind it — and that ratio is why a poor state's arsenal is a one-shot weapon
/// while a rich one's is a tap.
pub const MAGAZINE_REBUILD: f64 = 0.030;

/// The highest rung a belligerent with nothing left in the magazines can be
/// forced down to. Below this the ladder does not consume ordnance in quantity,
/// so a dry arsenal stops a campaign without ending the quarrel.
pub const MAX_SUSTAINABLE_DRY: u8 = 5;

/// What a soldier of this nation can do with what his country put in his hands.
///
/// Two terms, both derived. Technology is what the tree already scores. Capital
/// intensity — how much budget stands behind each point of force structure — is
/// what actually bought the optics, the tankers, the precision munitions and the
/// people who maintain them, and in January 1990 it is the *only* term that
/// separates anybody, because nobody has researched anything yet.
///
/// Against the transcribed data that gives the United States 1.37 and Iraq 0.67:
/// a hair over two to one, which is roughly the gap the Gulf turned out to be
/// once it was measured. Nobody typed either number.
pub fn quality(w: &WorldState, id: NationId) -> f64 {
    let n = match w.nation_opt(id) {
        Some(n) => n,
        None => return 1.0,
    };
    crate::tech::military_multiplier(n) * (0.55 + 0.75 * capital_intensity(w, id))
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

// ---------------------------------------------------------------------------
// What an objective and a set of rules of engagement are actually worth
// ---------------------------------------------------------------------------

/// Objectives are not decoration. Each is a different pair of multipliers on
/// killing and on taking ground, and choosing between them is choosing which of
/// the two tracks you are playing.
fn obj_kill(o: Objective) -> f64 {
    match o {
        Objective::Deny => 0.85,
        Objective::Degrade => 1.40,
        Objective::Seize => 1.00,
        Objective::Hold => 0.95,
        Objective::Stabilise => 0.90,
        Objective::Withdraw => 0.50,
    }
}
/// Denying them the ground is not the same as taking it, but a force in the
/// field that means to stop you does contest it — a fractional pull, not none,
/// which is why a coalition that arrives to deny can still push an occupier out.
fn obj_seize(o: Objective) -> f64 {
    match o {
        Objective::Deny => 0.35,
        Objective::Degrade => 0.30,
        Objective::Seize => 1.00,
        Objective::Hold => 0.20,
        Objective::Stabilise => 0.25,
        Objective::Withdraw => 0.00,
    }
}
/// How fast what you hold slips through your fingers. Holding is the objective
/// that is *about* not letting it.
fn obj_hold(o: Objective) -> f64 {
    match o {
        Objective::Hold => 0.25,
        Objective::Stabilise => 0.50,
        Objective::Withdraw => 4.00,
        _ => 1.00,
    }
}
fn obj_recover(o: Objective) -> f64 {
    match o {
        Objective::Hold => 1.20,
        Objective::Withdraw => 0.60,
        _ => 1.00,
    }
}
fn roe_kill(r: Roe) -> f64 {
    match r {
        Roe::Restrained => 0.75,
        Roe::Standard => 1.00,
        Roe::Unrestricted => 1.35,
    }
}
fn roe_seize(r: Roe) -> f64 {
    match r {
        Roe::Restrained => 0.90,
        Roe::Standard => 1.00,
        Roe::Unrestricted => 1.15,
    }
}
fn roe_burn(r: Roe) -> f64 {
    match r {
        Roe::Restrained => 0.80,
        Roe::Standard => 1.00,
        Roe::Unrestricted => 1.30,
    }
}

/// The rate at which committed force finds and kills exposed force, per month.
pub const KILL_RATE: f64 = 0.055;
/// Nobody's fires are perfectly efficient against a vanishingly small target,
/// and without this a rounding error becomes an infinite kill rate.
pub const FLOOR_MASS: f64 = 4.0;
/// How fast ground held slips away from a force that is not holding it down.
pub const CONTROL_DECAY: f64 = 0.010;
/// How fast the ground itself changes hands when one side is taking it and the
/// other is not. Large, because at a monthly tick it has to be: Kuwait fell in
/// two days and Iraq's army was destroyed in a hundred hours, and a model whose
/// front can only move six percent a month cannot express either. It is the
/// *ratio* that is bounded, not the speed — near-parity still grinds.
pub const SEIZE_PUSH: f64 = 0.35;
/// Keeps a rout from dividing by nothing without flattening a real mismatch.
pub const SEIZE_FLOOR: f64 = 2.0;
/// What "they hold the ground" means as a number. Control cannot reach a clean
/// 1.0 — the saturating term that stops a lead running away also stops it
/// arriving — so the threshold has to be named rather than assumed, and a war
/// that pins here for a year has been decided whatever the last hundredth says.
pub const CONTROL_SATURATED: f64 = 0.97;
/// The share of the gap to its sustainable size a nation's force structure
/// closes each month. The strength loop walks at exactly this rate, and the
/// resolve model reads the same number, because they are the same fact: what
/// you have to out-kill to be getting anywhere is what they can put back.
pub const REPLACEMENT_RATE: f64 = 0.020;

/// How much faster than replacement you have to be killing them before a
/// government can tell itself this is going to end.
///
/// Measured against replacement, not in absolute terms, and that distinction is
/// the difference between the two wars this model exists to tell apart. An
/// occupier killing 2.3% of an insurgency a month is not "three-quarters of the
/// way to decisive"; it is treading water against an enemy recruiting at 2%, and
/// its own government works that out within four years. A coalition killing 6% a
/// month of an army under embargo is finishing it. Same line, same constants.
pub const DECISIVE_KILL: f64 = 0.030;

/// One coalition's month, reduced to the six numbers the resolution needs. Every
/// per-belligerent quantity is weighted by what that belligerent actually
/// committed, so a great power that turns up with 2% of itself does not drag a
/// local army's numbers around.
struct Side {
    mass: f64,
    quality: f64,
    kill_mult: f64,
    seize_mult: f64,
    top_rung: u8,
    /// How much of this side has arrived meaning to deny rather than to take —
    /// which is how much of the *other* side's seizing it cancels.
    deny_share: f64,
    unrestricted: bool,
}

fn side_profile(w: &WorldState, c: &Conflict, side_a: bool) -> Side {
    let members = if side_a { &c.side_a } else { &c.side_b };
    let mut mass = 0.0;
    let (mut q, mut k, mut s, mut deny) = (0.0, 0.0, 0.0, 0.0);
    let mut top = 1u8;
    let mut unrestricted = false;
    for id in members {
        if w.nation_opt(*id).map_or(true, |n| !n.alive) {
            continue;
        }
        let b = match c.posture_of(*id) {
            Some(b) => b,
            None => continue,
        };
        top = top.max(b.rung);
        if b.roe == Roe::Unrestricted {
            unrestricted = true;
        }
        let m = committed_force(w, c, *id);
        if m <= 0.0 {
            continue;
        }
        // Advisers and intelligence are what rung 4 *is*: somebody else's army
        // fights better because your people are standing next to it.
        let advised = c.posture.iter().any(|o| {
            o.nation != *id
                && c.side_of(o.nation) == Some(side_a)
                && (4..=5).contains(&o.rung)
                && b.rung <= 5
        });
        mass += m;
        q += m * quality(w, *id) * if advised { 1.25 } else { 1.0 };
        k += m * obj_kill(b.objective) * roe_kill(b.roe);
        s += m * obj_seize(b.objective) * roe_seize(b.roe);
        if b.objective == Objective::Deny {
            deny += m;
        }
    }
    if mass <= 0.0 {
        return Side {
            mass: 0.0,
            quality: 1.0,
            kill_mult: 1.0,
            seize_mult: 0.0,
            top_rung: top,
            deny_share: 0.0,
            unrestricted,
        };
    }
    Side {
        mass,
        quality: q / mass,
        kill_mult: k / mass,
        seize_mult: s / mass,
        top_rung: top,
        deny_share: deny / mass,
        unrestricted,
    }
}

/// The capability gate, and it is a multiplication rather than a branch.
///
/// How much of itself the target offers to be found is set by the rung it stands
/// on, cut by terrain and buildings, and then scaled by whether the shooter can
/// see better than the target can hide. An army in the open at rung 8 facing a
/// force with twice its sensors is a target set; a proxy at rung 4 in a city in
/// the mountains is not, and no amount of mass changes that.
fn exposure(hunter: &Side, prey: &Side, th: &crate::theatre::Theatre) -> f64 {
    let terrain = (1.0 - 0.55 * th.urbanisation) * (1.0 - 0.45 * th.rough);
    let find = (hunter.quality / prey.quality.max(0.01)).clamp(0.30, 3.00);
    RUNG_EXPOSURE[(prey.top_rung as usize).min(9)] * terrain * find
}

fn kill_rate(hunter: &Side, prey: &Side, th: &crate::theatre::Theatre) -> f64 {
    if hunter.mass <= 0.0 {
        return 0.0;
    }
    (KILL_RATE * hunter.quality * hunter.kill_mult * hunter.mass * exposure(hunter, prey, th)
        / (prey.mass + FLOOR_MASS))
        .clamp(0.0, 0.50)
}

/// Monthly military & war tick.
pub fn tick(w: &mut WorldState) {
    // ---- Strength accumulation from spending, and magazines refilling ----
    let ids: Vec<NationId> = w.nations.iter().filter(|n| n.alive).map(|n| n.id).collect();
    for id in &ids {
        // Munitions rebuild out of the industry standing behind the army, not
        // out of the army. A state with tankers and a machine-tool base refills
        // in two and a half years what it can shoot off in eighteen months; one
        // without spends a decade catching up with a single campaign, and that
        // mismatch is the whole of BIBLE §6's second stock.
        let refill = MAGAZINE_REBUILD * capital_intensity(w, *id);
        let n = w.nation_mut(*id);
        let budget = n.gdp * n.mil_spend_gdp; // $bn/yr
        // Strength drifts toward what the budget sustains — and what a budget
        // sustains depends on what the arsenal it buys is made of.
        let sustained = (budget * 0.30).sqrt() * 8.0 * crate::tech::military_multiplier(n)
            + crate::tech::military_floor(n);
        n.mil_strength += (sustained - n.mil_strength) * REPLACEMENT_RATE;
        n.munitions = (n.munitions + refill).clamp(0.0, 1.0);
        // Exhaustion decays in peace
        n.war_exhaustion = (n.war_exhaustion - 0.01).max(0.0);
    }

    // Standing on rung 5 is a thing a service keeps doing, not a state it enters.
    crate::commitment::deniable_forces_upkeep(w);
    // Each side picks its own rung before the month is fought, not after.
    crate::commitment::ai_ladder(w);

    resolve_conflicts(w);
}

/// One outcome per conflict that stopped being one.
enum Ending {
    /// Side A took the ground and stayed at a rung that can hold it.
    Conquest,
    /// The opener was thrown back. The lesson sticks, which is what stops the
    /// same aggressor trying the same thing forever.
    Repelled,
    /// Both governments ran out of will in the same month.
    White,
    /// Terms: reparations always, territory only from a state that cannot answer
    /// with annihilation.
    Settled { winner: NationId, loser: NationId },
    /// Nobody is standing on a rung worth a headline any more.
    Lapsed,
}

fn resolve_conflicts(w: &mut WorldState) {
    let mut continuing: Vec<Conflict> = vec![];
    let mut ended: Vec<(Conflict, Ending)> = vec![];
    let conflicts = std::mem::take(&mut w.conflicts);

    for mut c in conflicts {
        // A conflict outlives its parties only as paperwork.
        c.side_a.retain(|id| w.nation_opt(*id).map_or(false, |n| n.alive));
        c.side_b.retain(|id| w.nation_opt(*id).map_or(false, |n| n.alive));
        c.posture.retain(|b| c.side_a.contains(&b.nation) || c.side_b.contains(&b.nation));
        if c.side_a.is_empty() || c.side_b.is_empty() {
            ended.push((c, Ending::Lapsed));
            continue;
        }

        c.months += 1;
        for b in c.posture.iter_mut() {
            b.months_at_rung += 1;
        }

        let th = crate::theatre::theatre(w, c.theatre).clone();
        let a = side_profile(w, &c, true);
        let b = side_profile(w, &c, false);

        // ---- STEP 2: the gate ----
        let kill_ab = kill_rate(&a, &b, &th);
        let kill_ba = kill_rate(&b, &a, &th);

        // ---- STEP 3: control, saturating so it cannot run away ----
        let seize_a = a.mass * a.quality * SEIZE_BY_RUNG[(a.top_rung as usize).min(9)]
            * a.seize_mult
            * (1.0 - 0.5 * b.deny_share);
        let seize_b = b.mass * b.quality * SEIZE_BY_RUNG[(b.top_rung as usize).min(9)]
            * b.seize_mult
            * (1.0 - 0.5 * a.deny_share);
        let hold_mult = {
            let side = c.control >= 0.0;
            let members = if side { &c.side_a } else { &c.side_b };
            members
                .iter()
                .filter_map(|id| c.posture_of(*id))
                .map(|x| obj_hold(x.objective))
                .fold(f64::INFINITY, f64::min)
                .min(4.0)
        };
        let push = SEIZE_PUSH * (seize_a - seize_b) / (seize_a + seize_b + SEIZE_FLOOR);
        let dcontrol = push * (1.0 - c.control * c.control) - CONTROL_DECAY * c.control * hold_mult;
        c.control = (c.control + dcontrol).clamp(-1.0, 1.0);

        // ---- STEP 4/5/6: resolve, magazines, and the national residue ----
        let mut exits: Vec<NationId> = vec![];
        let mut headlines: Vec<String> = vec![];
        let mut structure_hits: Vec<(NationId, f64)> = vec![];
        let mut exhaustion: Vec<(NationId, f64)> = vec![];
        let mut burns: Vec<(NationId, f64)> = vec![];
        for i in 0..c.posture.len() {
            let (nation, side) = {
                let x = &c.posture[i];
                (x.nation, c.side_of(x.nation))
            };
            let side_a = match side {
                Some(s) => s,
                None => continue,
            };
            let (kill_in, kill_out) = if side_a { (kill_ba, kill_ab) } else { (kill_ab, kill_ba) };
            let opp_unrestricted = if side_a { b.unrestricted } else { a.unrestricted };
            let mine = if side_a { c.control } else { -c.control };
            let enemy_top = c.top_rung(!side_a);
            let at_home = theatre::is_home(w, nation, c.theatre);

            let stake = stake_of(w, &c, nation, opp_unrestricted);
            let (rung, objective, red_line) = {
                let x = &c.posture[i];
                (x.rung, x.objective, x.red_line)
            };
            let (gdp, milshare, strength) = match w.nation_opt(nation) {
                Some(n) => (n.gdp, n.mil_spend_gdp, n.mil_strength),
                None => continue,
            };

            let decisive = ((kill_out - REPLACEMENT_RATE) / DECISIVE_KILL).clamp(0.0, 1.0);
            let held = mine.clamp(0.0, 1.0);
            let held_against = (-mine).clamp(0.0, 1.0);
            let withdrawing = objective == Objective::Withdraw;
            let mut casualty_pain = 0.55 * kill_in / stake.max(0.05);
            if objective == Objective::Hold {
                casualty_pain *= 0.6;
            }
            let mut treasure_pain = 0.06
                * (RUNG_COMMIT[(rung as usize).min(9)] * strength * 0.9)
                / (gdp * milshare + 8.0);
            // Holding ground you cannot convert into an ending is the thing that
            // actually loses modern wars, and it is the only term here that gets
            // *worse* the better the position looks on the map.
            let mut futility = 0.022 * held * (1.0 - decisive);
            // ...and winning fast is what keeps a government in a war it has no
            // business at home being in.
            let recovery =
                (0.008 + 0.020 * decisive) * (1.0 - held_against) * obj_recover(objective);
            if withdrawing {
                casualty_pain *= 0.30;
                treasure_pain *= 0.50;
                futility = 0.0;
            }

            let new_resolve =
                (c.posture[i].resolve + recovery - casualty_pain - treasure_pain - futility)
                    .clamp(0.0, 1.0);
            {
                let x = &mut c.posture[i];
                x.resolve = new_resolve;
                x.stake = stake;
            }

            // The acceptable-casualties threshold BIBLE §6 names, and the thing
            // that saves a government destroyed by a war it forgot to leave.
            if new_resolve <= red_line && !withdrawing && red_line > 0.0 {
                c.posture[i].objective = Objective::Withdraw;
                headlines.push(format!(
                    "{} crosses its own red line and orders a withdrawal.",
                    nation.name()
                ));
            }

            // Withdrawal is a schedule, not a decision taken again each month.
            if c.posture[i].objective == Objective::Withdraw && c.posture[i].rung > 1 {
                c.posture[i].rung -= 1;
                c.posture[i].months_at_rung = 0;
            }

            structure_hits.push((nation, kill_in));
            exhaustion.push((
                nation,
                0.036 * RUNG_COMMIT[(rung as usize).min(9)] * (1.0 + 2.0 * casualty_pain),
            ));
            burns.push((
                nation,
                BURN_BY_RUNG[(rung as usize).min(9)] * roe_burn(c.posture[i].roe),
            ));

            if new_resolve <= 0.0 {
                // Whether a government that has run out of will steps back or
                // gives up depends on whether the other side is in a position to
                // press it, and that distinction is QA's second finding.
                //
                // An expedition that has become unbearable comes home one rung
                // at a time; there is always a lower rung to stand on and a
                // border between you and them. A state whose own ground is held
                // by somebody else's army has nothing to step back TO, and a
                // government that has stopped being able to defend its own
                // country has stopped, whatever rung the paperwork says. Without
                // this every beaten defender slid quietly down the ladder,
                // reset its resolve to a quarter, and the war simply went quiet
                // — 82 conflicts across eight seeds and one capitulation.
                let pressed = mine <= -0.55 && enemy_top >= INVASION_RUNG && at_home;
                if c.posture[i].rung > 2 && !pressed {
                    c.posture[i].rung -= 1;
                    c.posture[i].months_at_rung = 0;
                    c.posture[i].resolve = 0.25;
                    headlines.push(format!(
                        "{} cannot sustain the commitment and falls back to rung {}.",
                        nation.name(),
                        c.posture[i].rung
                    ));
                } else {
                    if pressed {
                        headlines.push(format!(
                            "{} can no longer defend its own ground.",
                            nation.name()
                        ));
                    }
                    exits.push(nation);
                }
            }
        }

        for (id, rate) in structure_hits {
            let n = w.nation_mut(id);
            n.mil_strength = (n.mil_strength * (1.0 - rate)).max(0.0);
        }
        for (id, d) in exhaustion {
            let n = w.nation_mut(id);
            n.war_exhaustion = (n.war_exhaustion + d).min(1.0);
        }
        for (id, burn) in burns {
            let dry = {
                let n = w.nation_mut(id);
                n.munitions = (n.munitions - burn).clamp(0.0, 1.0);
                n.munitions <= 0.0 && burn > 0.0
            };
            // A dry magazine ends a campaign for logistical rather than
            // political reasons, and it is the guard against two spent armies
            // deadlocked at rung 8 forever.
            //
            // It stops at rung 5, not at rung 1: an empty magazine ends a
            // shooting campaign, it does not end the quarrel. A state whose
            // arsenal is spent still has sanctions, still has proxies, and still
            // has a service — it simply cannot conduct fires any more.
            if dry {
                if let Some(x) = c.posture_mut(id) {
                    if x.rung > MAX_SUSTAINABLE_DRY {
                        x.rung -= 1;
                        x.months_at_rung = 0;
                        headlines.push(format!(
                            "{}'s magazines are empty. The tempo falls to rung {}.",
                            id.name(),
                            x.rung
                        ));
                    }
                }
            }
        }
        for h in headlines {
            w.headline(h);
        }
        for id in &exits {
            w.headline(format!("{} quits the fight.", id.name()));
            c.side_a.retain(|x| x != id);
            c.side_b.retain(|x| x != id);
            c.posture.retain(|x| x.nation != *id);
        }

        // ---- STEP 7: termination, with no capitulation and no declaration ----
        let (lead_a, lead_b) = (c.attacker(), c.defender());
        if c.side_b.is_empty() {
            let e = if c.control >= 0.35 && c.top_rung(true) >= 8 {
                Ending::Conquest
            } else if c.control >= 0.35 {
                Ending::Settled { winner: lead_a, loser: lead_b }
            } else {
                Ending::Lapsed
            };
            ended.push((c, e));
            continue;
        }
        if c.side_a.is_empty() {
            ended.push((c, Ending::Repelled));
            continue;
        }
        // Holding the ground is not winning, and this pair of conditions is the
        // whole difference between Kuwait in 1990 and Afghanistan for twenty
        // years. Both are an occupier at control +1 with a rung-8 army in the
        // field. What separates them is whether the other side's government
        // still has anything left: Kuwait's did not, and an insurgency at home
        // taking no casualties never runs out. Take the resolve condition away
        // and every occupation resolves as an annexation inside a year.
        let spent = |side_a: bool| -> bool {
            let m = if side_a { &c.side_a } else { &c.side_b };
            m.iter()
                .filter_map(|id| c.posture_of(*id))
                .map(|x| x.resolve)
                .fold(0.0f64, f64::max)
                <= 0.05
        };
        if c.control >= CONTROL_SATURATED && c.top_rung(true) >= 8 && spent(false) {
            ended.push((c, Ending::Conquest));
            continue;
        }
        // ...and an aggressor is thrown back when it has lost the ground *and*
        // can no longer prosecute a campaign to get it back.
        if c.control <= -CONTROL_SATURATED
            && (c.top_rung(true) < SHOOTING_RUNG || spent(true))
        {
            ended.push((c, Ending::Repelled));
            continue;
        }
        let a_ex = w.nation(lead_a).war_exhaustion;
        let d_ex = w.nation(lead_b).war_exhaustion;
        if a_ex > 0.75 && d_ex > 0.75 {
            ended.push((c, Ending::White));
            continue;
        }
        if let Some((winner, loser)) = settlement_ripe(w, &c) {
            ended.push((c, Ending::Settled { winner, loser }));
            continue;
        }

        // Conflicts do not end; they go quiet. A conflict nobody is shooting in
        // freezes, keeps its rungs and its grievance, and stays on the board.
        let quiet = c.posture.iter().all(|x| x.rung < SHOOTING_RUNG);
        if quiet {
            c.quiet_months = c.quiet_months.saturating_add(1);
            if c.quiet_months >= 18 && c.frozen_since.is_none() {
                c.frozen_since = Some((w.year, w.month));
                w.headline(format!(
                    "The quarrel between {} and {} freezes over {}.",
                    lead_a.name(),
                    lead_b.name(),
                    c.theatre.name()
                ));
            }
        } else {
            c.quiet_months = 0;
            c.frozen_since = None;
        }

        // ...with one exception, and it is the difference between a world that
        // is consequential and one that merely churns. A quarrel that never
        // became a war may fade out; a quarrel in which somebody CROSSED A
        // BORDER IN FORCE may not. Half a year after the guns stop, an invasion
        // has a verdict, and which one it is is written on the map: if the
        // aggressor is holding the ground it took, it keeps it at a table; if it
        // is not, it has been thrown back, with everything that costs it at
        // home. Before this rule the aggressor could simply walk back down the
        // ladder and the whole thing evaporated — 150 conflicts across eight
        // seeds and two settlements between them.
        if c.invasion_declared && c.quiet_months >= 6 {
            let e = if c.control >= 0.35 {
                Ending::Settled { winner: lead_a, loser: lead_b }
            } else {
                Ending::Repelled
            };
            ended.push((c, e));
            continue;
        }

        // ...and a frozen quarrel that nobody has bothered to renew in two more
        // years is finally just history.
        if c.frozen_since.is_some()
            && c.quiet_months >= 42
            && c.posture.iter().all(|x| x.rung <= 1)
        {
            ended.push((c, Ending::Lapsed));
            continue;
        }

        continuing.push(c);
    }
    w.conflicts = continuing;

    for (c, e) in ended {
        match e {
            Ending::Conquest => {
                let (winner, loser) = (c.attacker(), c.defender());
                conquer(w, winner, loser);
                // The claim has been pressed to a conclusion. It survives as a
                // grievance and stops being a war aim — the counterpart of the
                // `burned_` lesson below, and the thing whose absence had states
                // relaunching the same invasion every second year for a century.
                // `origin_attacker` rather than `attacker()`, because after a
                // coalition joins, the lead of side A may be somebody who was
                // defending a client — the claim collected is the opener's.
                w.set_flag(&crate::dyads::settled_flag(c.origin_attacker, loser));
                w.set_flag(&crate::dyads::settled_flag(winner, loser));
            }
            Ending::Repelled => {
                // The lesson sticks, which is what stops the same aggressor
                // trying the same thing every year for a century.
                w.set_flag(&format!("burned_{:?}_{:?}", c.origin_attacker, c.defender()));
                if w.nation_opt(c.origin_attacker).map_or(false, |n| n.alive) {
                    let a = w.nation_mut(c.origin_attacker);
                    a.stability = (a.stability - 12.0).max(0.0);
                }
                w.headline(format!(
                    "{} repels {}'s invasion — the aggressor's regime totters.",
                    c.defender().name(),
                    c.origin_attacker.name()
                ));
            }
            Ending::White => {
                w.headline(format!(
                    "Exhausted, {} and {} sign a white peace.",
                    c.attacker().name(),
                    c.defender().name()
                ));
            }
            Ending::Settled { winner, loser } => {
                negotiated_peace(w, winner, loser);
                if winner == c.origin_attacker {
                    // The opener got what it came for, so its claim is
                    // collected and it has no war aim left.
                    w.set_flag(&crate::dyads::settled_flag(winner, loser));
                } else {
                    // The defender won terms. Marking ITS claim collected
                    // disarmed the wrong side entirely — the state that was
                    // invaded came away with its own grievance written off —
                    // and the aggressor learned nothing, which is the repeat
                    // invasion this flag exists to stop.
                    w.set_flag(&format!(
                        "burned_{:?}_{:?}",
                        c.origin_attacker,
                        c.defender()
                    ));
                }
            }
            Ending::Lapsed => {}
        }
    }
}

/// What losing here would mean at home. Home ground is everything; somebody
/// else's war is worth what your commitment to them is worth, and civilian
/// casualties harden whoever is on the receiving end of them.
fn stake_of(w: &WorldState, c: &Conflict, id: NationId, opp_unrestricted: bool) -> f64 {
    let mut s: f64 = if crate::theatre::is_home(w, id, c.theatre) {
        1.0
    } else {
        let side = c.side_of(id);
        let committed_to_someone = c
            .participants()
            .iter()
            .filter(|o| c.side_of(**o) == side && **o != id)
            .any(|o| w.allied(id, *o) || w.aid_share_to(id, *o) > 0.0);
        if committed_to_someone {
            0.45
        } else {
            0.20
        }
    };
    if opp_unrestricted {
        s += 0.15;
    }
    s.min(1.15)
}

/// Most wars end at a table, not in a capital. A side losing the ground clearly
/// but not yet catastrophically will buy its way out — if its government has run
/// low enough on will to swallow terms and its enemy has run low enough to offer
/// them instead of pressing for the whole prize.
fn settlement_ripe(w: &mut WorldState, c: &Conflict) -> Option<(NationId, NationId)> {
    let lead = c.control.abs();
    if !(0.35..CONTROL_SATURATED).contains(&lead) {
        return None;
    }
    let (winner, loser) = if c.control > 0.0 {
        (c.attacker(), c.defender())
    } else {
        (c.defender(), c.attacker())
    };
    let side_resolve = |id: NationId| c.posture_of(id).map_or(1.0, |b| b.resolve);
    let (lr, wr) = (side_resolve(loser), side_resolve(winner));
    if lr > 0.45 {
        return None; // still has fight in it
    }
    let p = 0.10 * lead * (1.0 - lr) * (1.5 - wr);
    if w.rng.chance(p.clamp(0.0, 0.5)) {
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

pub fn conflict_participants(c: &Conflict) -> Vec<NationId> {
    c.participants()
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

/// Where a war between these two is fought: the defender's own ground if it has
/// any, otherwise the attacker's, otherwise the sea lanes.
pub fn theatre_between(w: &WorldState, attacker: NationId, defender: NationId) -> TheatreId {
    theatre::home_theatre(w, defender)
        .or_else(|| theatre::home_theatre(w, attacker))
        .unwrap_or(TheatreId::Maritime)
}

/// The moment a quarrel becomes an invasion, and the only place the world
/// mobilises about one.
///
/// This used to live inside `declare_war` and fire the instant a conflict was
/// created, which is why every conflict in the world was born at rung 8: the
/// coalition, the guarantors and the interveners were all welded to the act of
/// starting a quarrel rather than to the act of crossing a border in force.
/// They belong here, hanging off the rung, so that a state can climb to an
/// invasion over years and the world answers at the step that deserves it.
///
/// Reached from two directions and identical down both: a player or an AI
/// climbing to rung 8 on the ladder, and `declare_war`, which is now sugar for
/// starting at the top.
pub fn invasion_begins(w: &mut WorldState, conflict: u32, attacker: NationId) {
    let idx = match w.conflicts.iter().position(|c| c.id == conflict) {
        Some(i) => i,
        None => return,
    };
    if w.conflicts[idx].invasion_declared {
        return;
    }
    // Taken out of the world for the duration because the guarantors need a
    // mutable conflict and a mutable world at the same time, and put back at the
    // same index so conflict order — which the whole sim iterates — cannot move.
    let mut c = w.conflicts.remove(idx);
    c.invasion_declared = true;
    let defender = match crate::commitment::primary_opponent(&c, attacker) {
        Some(d) => d,
        None => {
            w.conflicts.insert(idx, c);
            return;
        }
    };

    w.headline(format!("WAR: {} invades {}!", attacker.name(), defender.name()));
    w.shift_relation(attacker, defender, -60.0);
    // A state whose border has just been crossed fights for it with everything
    // it has, and it does not have to be talked into it.
    let at_home = theatre::is_home(w, defender, c.theatre);
    if let Some(b) = c.posture_mut(defender) {
        b.stake = 1.0;
        if b.rung < INVASION_RUNG && at_home {
            b.rung = INVASION_RUNG;
            b.months_at_rung = 0;
        }
    }

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
    let refused = crate::statecraft::call_the_guarantors(w, &mut c);

    // ...and friends of the victim may intervene (never against a nuclear attacker directly
    // unless they're nuclear too — abstracted: majors intervene if relation with victim high).
    for m in majors.iter().copied() {
        if c.involves(m) || refused.contains(&m) {
            continue;
        }
        if would_intervene(w, m, defender, attacker) {
            join_side(&mut c, m, false, 8, Objective::Deny);
            w.headline(format!("{} joins the war in defense of {}.", m.name(), defender.name()));
        }
    }

    let th = c.theatre;
    let expeditionary: Vec<NationId> = c
        .participants()
        .into_iter()
        .filter(|id| !theatre::is_home(w, *id, th))
        .collect();
    w.conflicts.insert(idx, c);

    // Everyone who has just committed to a war on somebody else's continent goes
    // round the neighbours asking for an airfield. This is the whole of BIBLE
    // §6's third object arriving at the only moment it matters: without a
    // consenting host, a coalition at rung 8 lands a quarter of the fraction of
    // itself it could deploy, and Desert Storm becomes a gesture.
    for id in expeditionary {
        crate::commitment::seek_access(w, id, th);
    }
}

/// Declare war, triggering coalition sanctions and possible interventions.
///
/// Kept as a public entry point and as sugar over the ladder: it opens a
/// conflict with the attacker already standing on rung 8, a full conventional
/// campaign, objective Seize. Nothing else in the sim starts here any more —
/// the AI opens quarrels at the bottom and buys its way up — but a player who
/// wants to skip the whole climb can still pay for the top rung in one act, and
/// the tests that stage a war outright use it.
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

    let th = theatre_between(w, attacker, defender);
    let id = w.next_conflict_id();
    w.conflicts.push(Conflict {
        id,
        theatre: th,
        side_a: vec![attacker],
        side_b: vec![defender],
        posture: vec![
            Belligerent::new(attacker, INVASION_RUNG, Objective::Seize),
            Belligerent::new(defender, INVASION_RUNG, Objective::Hold),
        ],
        control: 0.0,
        months: 0,
        quiet_months: 0,
        frozen_since: None,
        start_year: w.year,
        start_month: w.month,
        origin_attacker: attacker,
        invasion_declared: false,
    });
    invasion_begins(w, id, attacker);
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
