//! The front projection — BIBLE §5 as amended 2026-08-30: front lines drawn
//! across the admin-1 district map.
//!
//! Governing idea, stated once: **the legacy scalar equation remains the sole
//! source of monthly movement; the front is that movement projected onto
//! district geography.** `war::resolve_conflicts` computes `dcontrol` exactly
//! as it always has, and this module *spends* it as an area-budget across
//! frontier districts. The aggregate therefore tracks the old scalar by
//! construction, not by tuning — "ground is a projection of capability" is
//! literal code structure. The projection must never throttle the aggregate;
//! it only distributes it.
//!
//! No new combat scalars exist: mass, quality, rung, munitions, deployable
//! fraction, access, home ground, ROE and objective all arrive pre-mixed
//! inside the budget. The constants here — a swing cap terrain can shave, the
//! pocket decay, a held band — are distribution constants only. The module
//! draws nothing from the RNG, iterates only BTreeMaps and sorted vectors,
//! runs entirely inside the war tick, and adds no player command: the front
//! is read-only consequence.
//!
//! Terrain, since the terrain pass: inside the capped phase, the per-district
//! class from `districts::terrain_of` SUPERSEDES the theatre-scalar `rough`
//! factor the cap used to carry — the six tempo constants below, plus
//! `RIVER_CROSS_TEMPO` for ground whose every approach crosses a major river
//! and `TEMPO_FLOOR`, are the complete terrain vocabulary of the projection.
//! `theatre.rough` keeps every legacy role elsewhere (war.rs's exposure gate,
//! the UI readout, theatre seeding), and `theatre.urbanisation` stays a
//! theatre scalar in the shave because no district-level urban figure was
//! transcribed and inventing one fails the transcription bar. All of it
//! lives in the capped phase only; the final uncapped sweep is untouched, so
//! the aggregate spend is preserved by construction.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::districts;
use crate::theatre;
use crate::world::{Conflict, WorldState, SHOOTING_RUNG};

/// A district is *held* by a side when its hold is past this band; between the
/// bands it is contested. Shared by the pocket BFS, the settlement preference
/// and the UI, so the three can never disagree about whose a district is.
pub const HELD_BAND: f64 = 1.0 / 3.0;
/// How far one district's hold can move in one month of budget spending,
/// before terrain shaves it. Soft: a final uncapped pass spends whatever the
/// capped passes could not, because the projection must never throttle the
/// aggregate — terrain shapes tempo (where the movement lands), never
/// throughput (how much there is).
pub const SWING_CAP: f64 = 1.0;
/// How far a pocketed district's hold moves toward its surrounder each month,
/// budget or no budget — a pocket flips without an assault. The one deviation
/// from the legacy aggregate, bounded by pocket area share and monotone
/// toward the side already winning.
pub const POCKET_SWING: f64 = 0.50;
/// Deviations smaller than this are pruned back to the baseline.
pub const FRONT_EPS: f64 = 1e-4;

// ---- Terrain tempo (distribution constants, BIBLE §5: distribution and
// tempo only). Each class multiplies the swing cap inside the capped phase;
// the uncapped sweep ignores them, so terrain shapes where and how fast the
// budget lands, never how much of it there is.
/// The reference going.
pub const TEMPO_LOWLAND: f64 = 1.00;
pub const TEMPO_HIGHLAND: f64 = 0.70;
/// The strongest ground defence.
pub const TEMPO_MOUNTAIN: f64 = 0.40;
/// Open going: manoeuvre outruns the reference. The desert–munitions
/// interaction is emergent, not coded — munitions arrive pre-mixed in the
/// budget (module contract above), dry magazines shrink it, and a
/// multiplier above one means what budget remains travels further in the
/// open. Do not re-read munitions here.
pub const TEMPO_DESERT: f64 = 1.15;
pub const TEMPO_WETLAND: f64 = 0.55;
/// Attrition country: the tempo shave IS the attrition here.
pub const TEMPO_TUNDRA: f64 = 0.55;
/// Extra shave when every qualifying approach to a district crosses a major
/// river (`districts::crosses_river`). Rivers slow a crossing; they never
/// forbid one, and they never disqualify an approach in `frontier`.
pub const RIVER_CROSS_TEMPO: f64 = 0.50;
/// The per-district cap never shaves below this — ground is always takeable,
/// only slower.
pub const TEMPO_FLOOR: f64 = 0.15;

/// The one map from class to tempo. Public so the calibration tests can
/// compute expected caps without restating the table.
pub fn tempo_of(t: districts::TerrainClass) -> f64 {
    match t {
        districts::TerrainClass::Lowland => TEMPO_LOWLAND,
        districts::TerrainClass::Highland => TEMPO_HIGHLAND,
        districts::TerrainClass::Mountain => TEMPO_MOUNTAIN,
        districts::TerrainClass::Desert => TEMPO_DESERT,
        districts::TerrainClass::Wetland => TEMPO_WETLAND,
        districts::TerrainClass::Tundra => TEMPO_TUNDRA,
    }
}

const BUDGET_EPS: f64 = 1e-12;
const STEP_EPS: f64 = 1e-9;

/// The contested ground of one conflict: the two principals' home-theatre
/// districts, never the whole world. A coalition ally's territory is not in
/// it — adding Saudi Arabia's 2.15M km² to a Kuwait war would make the front
/// absurd, and because the aggregate is budget-driven, the set's composition
/// decides *where* ground moves, never how fast the scalar does.
pub struct Contested {
    /// district id -> (base is side A, area in km²).
    pub k: BTreeMap<String, (bool, f64)>,
    pub area_a: f64,
    pub area_b: f64,
}

pub fn contested_set(w: &WorldState, c: &Conflict) -> Contested {
    let att = c.attacker();
    let def = c.defender();
    let a_home = theatre::is_home(w, att, c.theatre);
    let d_home = att != def && theatre::is_home(w, def, c.theatre);
    let mut k = BTreeMap::new();
    let (mut area_a, mut area_b) = (0.0f64, 0.0f64);
    if !a_home && !d_home {
        return Contested { k, area_a, area_b };
    }
    for (d, &o) in w.districts.iter() {
        let is_a = if a_home && o == att {
            true
        } else if d_home && o == def {
            false
        } else {
            continue;
        };
        let ar = districts::area_of(d);
        if is_a {
            area_a += ar;
        } else {
            area_b += ar;
        }
        k.insert(d.clone(), (is_a, ar));
    }
    Contested { k, area_a, area_b }
}

fn base_of(is_a: bool) -> f64 {
    if is_a {
        1.0
    } else {
        -1.0
    }
}

/// Where one district stands: the stored deviation, or its owner's side.
fn hold_of(c: &Conflict, d: &str, is_a: bool) -> f64 {
    c.front.get(d).map(|v| *v as f64).unwrap_or_else(|| base_of(is_a))
}

/// The single f32 write site. All arithmetic is f64; the store narrows once,
/// so the same value takes the same bytes on every path and every platform.
fn set_hold(c: &mut Conflict, d: &str, v: f64) {
    c.front.insert(d.to_string(), v.clamp(-1.0, 1.0) as f32);
}

/// held (+1/-1) or contested (0), on the shared band.
fn class_of(h: f64) -> i8 {
    if h > HELD_BAND {
        1
    } else if h < -HELD_BAND {
        -1
    } else {
        0
    }
}

/// The aggregate: district control, area-weighted, mapped onto the legacy
/// scalar's semantics. 0 = status quo ante; +1 = side A holds all of B's
/// theatre ground and all of its own. A zero-area denominator zeroes its term,
/// which is the one-sided expeditionary gauge: an attacker with no theatre
/// ground of its own cannot lose ground it never had, and C runs 0..1.
pub fn gauge(c: &Conflict, k: &Contested) -> f64 {
    let (mut gain_a, mut gain_b) = (0.0f64, 0.0f64);
    for (d, &(is_a, area)) in &k.k {
        let h = hold_of(c, d, is_a);
        if is_a {
            gain_b += area * (1.0 - h) / 2.0;
        } else {
            gain_a += area * (h + 1.0) / 2.0;
        }
    }
    let ta = if k.area_b > 0.0 { gain_a / k.area_b } else { 0.0 };
    let tb = if k.area_a > 0.0 { gain_b / k.area_a } else { 0.0 };
    (ta - tb).clamp(-1.0, 1.0)
}

/// Step 0 of the monthly update: the aggregate is recomputed from the ground
/// before the legacy equation reads it. Self-healing — a district another
/// conflict's settlement moved mid-month leaves the contested set here, and
/// the control every downstream reader consumes is what the map actually
/// says.
pub fn sync(c: &mut Conflict, k: &Contested) {
    c.front.retain(|d, _| k.k.contains_key(d));
    c.control = gauge(c, k);
}

/// The whole monthly projection: pockets on last month's holds, the budget
/// spend, the pocket floor, then the canonical prune and the aggregate.
/// `budget` is the legacy equation's `dcontrol`, verbatim.
pub fn project(
    w: &WorldState,
    c: &mut Conflict,
    k: &Contested,
    budget: f64,
    th: &theatre::Theatre,
) {
    // ---- Pockets, from the holds the month opened with ----
    let (comps_a, decays_a) = pockets_of(w, c, k, true);
    let (comps_b, decays_b) = pockets_of(w, c, k, false);
    let mut all = comps_a.clone();
    all.extend(comps_b.clone());
    all.sort();
    c.pockets = all;

    // ---- Spend the budget. Retreat is not a separate rule: the decay term
    // realizes as the disadvantaged side advancing back over its own ground.
    // The per-district terrain class supersedes the theatre `rough` factor
    // the cap used to carry; urbanisation stays theatre-scalar (module doc).
    if budget.abs() > BUDGET_EPS {
        let advancing_a = budget > 0.0;
        let urban_shave = 1.0 - 0.25 * th.urbanisation;
        let enemy_pockets: BTreeSet<String> = if advancing_a { &comps_b } else { &comps_a }
            .iter()
            .flatten()
            .cloned()
            .collect();
        allocate(w, c, k, advancing_a, budget.abs(), Some(urban_shave), &enemy_pockets);
    }

    // ---- Pocket floor: a surrounded garrison degrades whatever the budget
    // did, and can flip without an assault — but only to a surrounder still
    // actually shooting. A frozen conflict does not digest pockets.
    if c.top_rung(false) >= SHOOTING_RUNG {
        for (d, start) in &decays_a {
            let target = (start - POCKET_SWING * crate::clock::month_fraction(w)).max(-1.0);
            if hold_of(c, d, true) > target {
                set_hold(c, d, target);
            }
        }
    }
    if c.top_rung(true) >= SHOOTING_RUNG {
        for (d, start) in &decays_b {
            let target = (start + POCKET_SWING * crate::clock::month_fraction(w)).min(1.0);
            if hold_of(c, d, false) < target {
                set_hold(c, d, target);
            }
        }
    }

    finish(c, k);
}

/// The canonical form: deviations, plus the base-valued districts adjacent to
/// one — kept so the map's front line never vanishes along an edge where the
/// uncapped pass captured a district exactly to the pole (the seam rule the
/// referee required, recorded here and in the UI spec). Then the aggregate.
fn finish(c: &mut Conflict, k: &Contested) {
    let mut deviated: BTreeSet<&str> = BTreeSet::new();
    for (d, &(is_a, _)) in &k.k {
        if let Some(&h) = c.front.get(d) {
            if (h as f64 - base_of(is_a)).abs() >= FRONT_EPS {
                deviated.insert(d.as_str());
            }
        }
    }
    let mut keep: BTreeMap<String, f32> = BTreeMap::new();
    for &d in &deviated {
        if let Some(&h) = c.front.get(d) {
            keep.insert(d.to_string(), h);
        }
    }
    for &d in &deviated {
        for n in districts::adj_of(d) {
            if deviated.contains(n.as_str()) {
                continue;
            }
            if let Some(&(is_a, _)) = k.k.get(n.as_str()) {
                keep.insert(n.clone(), base_of(is_a) as f32);
            }
        }
    }
    c.front = keep;
    c.control = gauge(c, k);
}

/// The spend allocator: walk the frontier in priority order, move each
/// district toward the advancing side's pole at its own terrain-shaved cap,
/// and when a pass ends with budget left, recompute the frontier — newly
/// held ground opens neighbours — and go again. If the capped passes cannot
/// place it all, one uncapped sweep (same order) spends the rest: the budget
/// is always fully spent while any capacity remains, and capacity exhausts
/// exactly as |C| -> 1, coinciding with the legacy saturation.
///
/// `urban_shave`: Some(shave) runs the terrain-capped phase (per-district
/// class tempo × the theatre urbanisation shave, halved again for ground
/// reachable only across a river, floored at `TEMPO_FLOOR`) and then the
/// uncapped sweep; None is the uncapped sweep alone — the save-reseed path,
/// byte-identical to the pre-terrain code.
fn allocate(
    w: &WorldState,
    c: &mut Conflict,
    k: &Contested,
    advancing_a: bool,
    mut budget: f64,
    urban_shave: Option<f64>,
    enemy_pockets: &BTreeSet<String>,
) {
    let pole = base_of(advancing_a);
    let dir = if advancing_a { 1.0 } else { -1.0 };
    let mut moved: BTreeMap<String, f64> = BTreeMap::new();
    let phases: &[Option<f64>] =
        if urban_shave.is_some() { &[urban_shave, None] } else { &[None] };
    for &phase_shave in phases {
        for _pass in 0..=k.k.len() {
            if budget <= BUDGET_EPS {
                return;
            }
            let (f, river_only) = frontier(w, c, k, advancing_a, enemy_pockets);
            if f.is_empty() {
                return;
            }
            let mut progress = false;
            for d in &f {
                if budget <= BUDGET_EPS {
                    break;
                }
                let (is_a, area) = k.k[d.as_str()];
                let h = hold_of(c, d, is_a);
                let dist = (pole - h) * dir;
                if dist <= STEP_EPS {
                    continue;
                }
                let cap_d = phase_shave.map(|us| {
                    let mut cd = SWING_CAP * tempo_of(districts::terrain_of(d)) * us;
                    if river_only.get(d.as_str()).copied().unwrap_or(false) {
                        cd *= RIVER_CROSS_TEMPO;
                    }
                    cd.max(TEMPO_FLOOR) * crate::clock::month_fraction(w)
                });
                let room = match cap_d {
                    Some(cp) => (cp - moved.get(d.as_str()).copied().unwrap_or(0.0)).min(dist),
                    None => dist,
                };
                if room <= STEP_EPS {
                    continue;
                }
                // The exchange rate: what a unit of hold on this district
                // costs in aggregate control.
                let side_area = if is_a { k.area_a } else { k.area_b };
                let price = if side_area > 0.0 { area / (2.0 * side_area) } else { 0.0 };
                let afford = if price > 0.0 { budget / price } else { f64::INFINITY };
                let step = room.min(afford);
                if step <= STEP_EPS {
                    continue;
                }
                let nh = if step >= dist - STEP_EPS { pole } else { h + step * dir };
                set_hold(c, d, nh);
                budget -= step * price;
                *moved.entry(d.clone()).or_insert(0.0) += step;
                progress = true;
            }
            if !progress {
                break;
            }
        }
    }
}

/// Where the advancing side can push this month. A district qualifies when it
/// is short of the advancing pole and reachable: from adjacent held ground,
/// across the border from any side member's national territory, or — for an
/// island — by the air and sea reach a theatre's access rules and the upper
/// rungs already model. When nothing qualifies and the side holds nothing,
/// the enemy ground's periphery is the way in: an expeditionary landing.
///
/// Alongside the frontier itself, returns the river-only flag per member: a
/// district is river-only when it has at least one qualifying land approach
/// and EVERY qualifying land approach crosses a major river. Rivers never
/// disqualify an approach (rivers are crossable — the capped phase merely
/// shaves the tempo), and sea approaches — islands via `reach`, the
/// periphery entry rule — are never river-only.
fn frontier(
    w: &WorldState,
    c: &Conflict,
    k: &Contested,
    advancing_a: bool,
    enemy_pockets: &BTreeSet<String>,
) -> (Vec<String>, BTreeMap<String, bool>) {
    let pole = base_of(advancing_a);
    let dir = if advancing_a { 1.0 } else { -1.0 };
    let sign: i8 = if advancing_a { 1 } else { -1 };
    let members = if advancing_a { &c.side_a } else { &c.side_b };
    let side_owns = |d: &str| -> bool {
        w.districts.get(d).is_some_and(|o| members.contains(o))
    };
    let reach = || -> bool {
        c.top_rung(advancing_a) >= 7
            || members.iter().any(|id| theatre::has_access(w, *id, c.theatre))
    };
    let mut out: Vec<String> = vec![];
    let mut river_only: BTreeMap<String, bool> = BTreeMap::new();
    for (d, &(is_a, _)) in &k.k {
        if (pole - hold_of(c, d, is_a)) * dir <= STEP_EPS {
            continue; // already at the pole
        }
        let adj = districts::adj_of(d);
        if adj.is_empty() {
            if reach() {
                out.push(d.clone());
                river_only.insert(d.clone(), false); // a sea approach
            }
            continue;
        }
        let (mut qualifying, mut crossed) = (0usize, 0usize);
        for n in adj {
            let q = k
                .k
                .get(n.as_str())
                .is_some_and(|&(na, _)| class_of(hold_of(c, n, na)) == sign)
                || side_owns(n);
            if q {
                qualifying += 1;
                if districts::crosses_river(d, n) {
                    crossed += 1;
                }
            }
        }
        if qualifying > 0 {
            out.push(d.clone());
            river_only.insert(d.clone(), crossed == qualifying);
        }
    }
    if out.is_empty() {
        // Entry rule: the enemy ground's periphery — a district with a
        // neighbour not owned by the enemy side, or no land neighbour at all.
        // A landing from the sea, so never river-only.
        let enemy = if advancing_a { &c.side_b } else { &c.side_a };
        for (d, &(is_a, _)) in &k.k {
            if (pole - hold_of(c, d, is_a)) * dir <= STEP_EPS {
                continue;
            }
            let adj = districts::adj_of(d);
            let peripheral = adj.is_empty()
                || adj.iter().any(|n| {
                    !w.districts.get(n.as_str()).is_some_and(|o| enemy.contains(o))
                });
            if peripheral {
                out.push(d.clone());
                river_only.insert(d.clone(), false);
            }
        }
    }
    // Priority: enemy pockets first (area desc, id asc); then own base ground
    // retaken before enemy ground; then land-reachable ground ahead of
    // river-only ground; then the pinched — most held neighbours first,
    // which is where encirclement-adjacent behaviour falls out; then the big
    // border districts; then the id, so the order is total.
    let held_neighbours = |d: &str| -> usize {
        districts::adj_of(d)
            .iter()
            .filter(|n| {
                k.k.get(n.as_str())
                    .is_some_and(|&(na, _)| class_of(hold_of(c, n, na)) == sign)
            })
            .count()
    };
    // The aim (resources.rs): the district the opener's quarrel is for sorts
    // first after enemy pockets, on the opener's side only. No aim, no term —
    // the comparator below is then the one it always was.
    let aim: Option<&str> = match &c.aim {
        Some(a) if members.contains(&c.origin_attacker) => Some(a.district.as_str()),
        _ => None,
    };
    out.sort_by(|x, y| {
        let (px, py) = (enemy_pockets.contains(x), enemy_pockets.contains(y));
        match (px, py) {
            (true, false) => return std::cmp::Ordering::Less,
            (false, true) => return std::cmp::Ordering::Greater,
            (true, true) => {
                return districts::area_of(y)
                    .total_cmp(&districts::area_of(x))
                    .then_with(|| x.cmp(y));
            }
            (false, false) => {}
        }
        if let Some(aim) = aim {
            match (x.as_str() == aim, y.as_str() == aim) {
                (true, false) => return std::cmp::Ordering::Less,
                (false, true) => return std::cmp::Ordering::Greater,
                _ => {}
            }
        }
        let (ox, oy) = (k.k[x.as_str()].0 == advancing_a, k.k[y.as_str()].0 == advancing_a);
        oy.cmp(&ox)
            .then_with(|| river_only[x.as_str()].cmp(&river_only[y.as_str()]))
            .then_with(|| held_neighbours(y).cmp(&held_neighbours(x)))
            .then_with(|| districts::area_of(y).total_cmp(&districts::area_of(x)))
            .then_with(|| x.cmp(y))
    });
    (out, river_only)
}

/// Encirclement, from adjacency alone: one BFS per side per month over the
/// contested set. Passable ground for a side is what it holds plus what is
/// contested; anchors are wherever that ground touches the side's own
/// national territory outside the set — or the open sea, which is why an
/// island never pockets — and when neither exists (Kuwait: the whole country
/// IS the set) the component around the largest held district is deemed the
/// main mass. An unreached component whose every external neighbour is
/// enemy-held is a pocket; one touching a neutral third nation is cut off but
/// not pocketed. Every member of a pocket decays — held and contested alike,
/// because a component that stopped qualifying the moment decay carried its
/// garrison inside the contested band would stall there forever, and §5's
/// behaviour is stated outright: a full pocket flips in four months, crosses
/// the band sooner, reclassifies enemy-held and collapses inward. Returns
/// the components, and the members with their opening holds.
#[allow(clippy::type_complexity)]
fn pockets_of(
    w: &WorldState,
    c: &Conflict,
    k: &Contested,
    side_a: bool,
) -> (Vec<Vec<String>>, Vec<(String, f64)>) {
    let sign: i8 = if side_a { 1 } else { -1 };
    let members = if side_a { &c.side_a } else { &c.side_b };
    let mut passable: BTreeSet<&str> = BTreeSet::new();
    let mut held: BTreeSet<&str> = BTreeSet::new();
    for (d, &(is_a, _)) in &k.k {
        let cl = class_of(hold_of(c, d, is_a));
        if cl == sign {
            held.insert(d.as_str());
            passable.insert(d.as_str());
        } else if cl == 0 {
            passable.insert(d.as_str());
        }
    }
    if passable.is_empty() {
        return (vec![], vec![]);
    }
    let mut anchors: BTreeSet<&str> = BTreeSet::new();
    for &d in &passable {
        let adj = districts::adj_of(d);
        if adj.is_empty() {
            anchors.insert(d);
            continue;
        }
        if adj.iter().any(|n| {
            !k.k.contains_key(n.as_str())
                && w.districts.get(n.as_str()).is_some_and(|o| members.contains(o))
        }) {
            anchors.insert(d);
        }
    }
    if anchors.is_empty() && !held.is_empty() {
        // The main mass by declaration: largest held district, id ascending
        // on a tie, so the choice is total and identical on every machine.
        // A side holding nothing at all has no main mass to declare — its
        // contested ground stands or falls on the sealed test alone.
        let seed = held
            .iter()
            .copied()
            .min_by(|a, b| {
                districts::area_of(b)
                    .total_cmp(&districts::area_of(a))
                    .then_with(|| a.cmp(b))
            })
            .expect("held is nonempty");
        anchors.insert(seed);
    }
    // BFS from the anchors through passable ground.
    let mut reached: BTreeSet<&str> = BTreeSet::new();
    let mut queue: VecDeque<&str> = VecDeque::new();
    for &a in &anchors {
        if reached.insert(a) {
            queue.push_back(a);
        }
    }
    while let Some(d) = queue.pop_front() {
        for n in districts::adj_of(d) {
            if passable.contains(n.as_str()) && reached.insert(n.as_str()) {
                queue.push_back(n.as_str());
            }
        }
    }
    // Unreached components, and the sealed ones are pockets.
    let enemy = if side_a { &c.side_b } else { &c.side_a };
    let mut comps: Vec<Vec<String>> = vec![];
    let mut decays: Vec<(String, f64)> = vec![];
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for &start in &passable {
        if reached.contains(start) || seen.contains(start) {
            continue;
        }
        let mut comp: BTreeSet<&str> = BTreeSet::new();
        let mut q: VecDeque<&str> = VecDeque::new();
        comp.insert(start);
        seen.insert(start);
        q.push_back(start);
        while let Some(d) = q.pop_front() {
            for n in districts::adj_of(d) {
                if passable.contains(n.as_str())
                    && !reached.contains(n.as_str())
                    && comp.insert(n.as_str())
                {
                    seen.insert(n.as_str());
                    q.push_back(n.as_str());
                }
            }
        }
        // Sealed means surrounded BY the enemy: every external neighbour
        // enemy-held, and at least one of them — a component with no external
        // neighbours at all (Honshu, Great Britain, a continent reached only
        // through its islands' sea anchors) has the sea at its back, not an
        // army, and is cut off on the land graph without being pocketed.
        let mut sealed = true;
        let mut externals = 0usize;
        for &m in &comp {
            for n in districts::adj_of(m) {
                if comp.contains(n.as_str()) {
                    continue;
                }
                externals += 1;
                if let Some(&(na, _)) = k.k.get(n.as_str()) {
                    if class_of(hold_of(c, n, na)) != -sign {
                        sealed = false;
                    }
                } else if !w.districts.get(n.as_str()).is_some_and(|o| enemy.contains(o)) {
                    sealed = false;
                }
            }
        }
        if sealed && externals > 0 {
            for &m in &comp {
                let (is_a, _) = k.k[m];
                decays.push((m.to_string(), hold_of(c, m, is_a)));
            }
            comps.push(comp.iter().map(|s| s.to_string()).collect());
        }
    }
    comps.sort();
    decays.sort_by(|a, b| a.0.cmp(&b.0));
    (comps, decays)
}

/// The districts a settlement's winner actually holds of the loser's ground,
/// past the held band — what `negotiated_peace` prefers to move.
pub fn held_by(
    w: &WorldState,
    c: &Conflict,
    winner: crate::world::NationId,
    loser: crate::world::NationId,
) -> BTreeSet<String> {
    let side_a = match c.side_of(winner) {
        Some(s) => s,
        None => return BTreeSet::new(),
    };
    c.front
        .iter()
        .filter(|(d, &h)| {
            w.districts.get(d.as_str()) == Some(&loser)
                && if side_a { (h as f64) > HELD_BAND } else { (h as f64) < -HELD_BAND }
        })
        .map(|(d, _)| d.clone())
        .collect()
}

/// Reconstruct a front for a save written before the operational map: for
/// each live conflict with real control, contested ground and no front, spend
/// the saved control as an uncapped budget through the same allocator — zero
/// RNG, deterministic, and the aggregate the front then expresses is the
/// control the save carried (a new-code roundtrip never lands here, because
/// its front is serialized). Runs in `load()` beside `districts::reseed`.
pub fn reseed_fronts(w: &mut WorldState) {
    let mut conflicts = std::mem::take(&mut w.conflicts);
    for c in &mut conflicts {
        if !c.front.is_empty() || c.control.abs() <= 0.001 {
            continue;
        }
        if c.side_a.is_empty() || c.side_b.is_empty() {
            continue;
        }
        let k = contested_set(w, c);
        if k.k.is_empty() {
            continue;
        }
        let advancing_a = c.control > 0.0;
        let none = BTreeSet::new();
        allocate(w, c, &k, advancing_a, c.control.abs(), None, &none);
        finish(c, &k);
    }
    w.conflicts = conflicts;
}
