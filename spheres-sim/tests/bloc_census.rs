//! THE BLOC CENSUS — S5's instrument for "The Political Arm of SPHERES"
//! (rev 2, approved 2026-09-05), the section "What history it must
//! reproduce". MEASUREMENT ONLY: `#[ignore]`d, asserts nothing, moves no
//! constant. It reads, per seed, every quantity the anchors A1-A10 need, and
//! prints one line per seed plus the summary — the per-seed sample every bar
//! derives its n from (iron rule 7), so the derivation beside a pinned bar
//! can be re-run instead of taken on trust. It sits beside the bars the way
//! `gulf_war_incidence_scan` sits beside `gulf_war_emerges`.
//!
//! The world: both switches on (`ideology_blocs`, `ideology_takeover`),
//! `GameRules::default()` otherwise, the monthly clock, no player, no
//! commands, January 1990 to December 2010 (252 months), seeds
//! `0..SPHERES_CENSUS_SEEDS` (default 60).
//!
//! ```text
//! SPHERES_CENSUS_SEEDS=60 cargo test --release -p spheres-sim --test bloc_census -- --ignored --nocapture
//! ```
//!
//! DEFINITIONS, so the numbers below mean one thing:
//! - A "takeover" is a change of the RULING BLOC (`blocs::ruling_bloc`, read
//!   before and after each month's tick) attributed to that month's headline
//!   for the nation: annulment ("annuls the election"), coup against an
//!   elected government ("removes the elected government"), a regime coup by
//!   a pillar ("removes the government."), an uprising ("Revolution in X: the
//!   B movement takes power."), a programme ("X declares a ... programme"),
//!   or a BALLOT ("X votes:" / "The government of X falls" with the state
//!   electoral before and after). A change with none of those (the seam
//!   seating an interim table, a birth) is "other" and is neither.
//! - An uprising's CLAUSE is the pre-tick reading of what armed it: the
//!   pre-existing stability < 12 collapse, the movement
//!   (`blocs::uprising_armed`), both, or neither ("none": crossed inside
//!   the tick, which the pre-tick reading cannot see).
//! - A5's "ex-communist return by ballot": a Communist-bloc-led coalition
//!   seated by BALLOT (as above) in a nation that was Communist-ruled on
//!   1 January 1990, or an ex-Warsaw-Pact state (Poland, Hungary,
//!   Czechoslovakia, Romania, Bulgaria, the USSR), or a successor state born
//!   after 1990. Counted by end-1996 (the design's date) and over the run.
//! - A6's "route event": annulment, coup against an elected government,
//!   regime coup, suspension, round table, or an uprising the MOVEMENT armed
//!   pre-tick (`blocs::uprising_armed`), in a nation whose 1990
//!   authoritarianism was <= 0.20. An uprising the movement did not arm is
//!   the pre-existing stability < 12 collapse (or its crossing inside the
//!   tick) and is reported beside it, not as a route event.
//! - A9 reads GDP per head (`gdp * 1000 / population`, dollars) pre-tick in
//!   the coup's month over every coup and annulment.
//! - A10: for every non-ballot takeover, the nation's mean discontent over
//!   the twelve pre-tick months before it, less the roster median of the same
//!   twelve-month mean over every nation with a government; the per-seed
//!   figure is the median of those differences.

use spheres_sim::blocs;
use spheres_sim::government::{self, Bloc};
use spheres_sim::init::world_1990;
use spheres_sim::world::*;
use spheres_sim::{tick_month};
use std::collections::{HashMap, HashSet};

const MONTHS: usize = 252;
const END_1996: usize = 84; // ticks through December 1996
const END_2000: usize = 132; // ticks through December 2000

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Route {
    Annul,
    ElCoup,
    RegCoup,
    Uprising,
    Programme,
    Ballot,
    Suspend,
    RoundTable,
    OpensUp,
    Seam,
    Other,
}

#[derive(Default, Clone)]
struct Seed {
    seed: u64,
    coup_el: u32,
    annul: u32,
    coup_reg: u32,
    algeria_annulled: bool,
    algeria_islamist: bool,
    /// uprisings by winner bloc (enum order); [5] = no winner ("old regime falls")
    upr: [u32; 6],
    upr_clause: [u32; 4], // stab12, movement, both, none
    /// non-ballot takeovers (ruling bloc changed) by NEW bloc, enum order
    take: [u32; 5],
    take_total: u32,
    ballot_flips: u32,
    /// changes of the leading party by ballot (a bloc change or not)
    ballot_party_flips: u32,
    islamist_states_by_2000: u32,
    islamist_states: u32,
    communist_states: u32,
    opened_1996: u32,
    opened_attr: [u32; 4], // drift, uprising, lever, card
    excomm_1996: u32,
    excomm_run: u32,
    a6_route: u32,
    a6_collapse: u32,
    big8_keep: u32,
    coups_poor: u32,
    coups_all: u32,
    a10_median: f64,
    a10_n: u32,
    deaths: u32,
    rt_lever: u32,
    suspend: u32,
    programme: u32,
    max_islamist_i: f64,
    max_islamist_where: &'static str,
    elcoup_nations: Vec<&'static str>,
    top3_share: f64,
}

fn median(v: &[f64]) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    let mut s = v.to_vec();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = s.len();
    if n % 2 == 1 { s[n / 2] } else { (s[n / 2 - 1] + s[n / 2]) / 2.0 }
}

fn stats(v: &[f64]) -> String {
    let mut s = v.to_vec();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mean = s.iter().sum::<f64>() / s.len().max(1) as f64;
    let var = s.iter().map(|x| (x - mean) * (x - mean)).sum::<f64>() / (s.len().max(2) - 1) as f64;
    format!(
        "min {:.2} med {:.2} max {:.2} mean {:.3} sd {:.3}",
        s[0],
        median(&s),
        s[s.len() - 1],
        mean,
        var.sqrt()
    )
}

/// A deterministic bootstrap of the median: P(median lands outside lo..=hi)
/// over 2000 resamples with a fixed LCG, so a band bar can size its n.
fn bootstrap_median_outside(v: &[f64], lo: f64, hi: f64, n: usize) -> f64 {
    let mut x: u64 = 0x9E3779B97F4A7C15;
    let mut bad = 0usize;
    const R: usize = 2000;
    for _ in 0..R {
        let mut sample = Vec::with_capacity(n);
        for _ in 0..n {
            x = x.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            sample.push(v[((x >> 33) as usize) % v.len()]);
        }
        let m = median(&sample);
        if m < lo || m > hi {
            bad += 1;
        }
    }
    bad as f64 / R as f64
}

fn name_of(w: &WorldState, i: usize) -> &'static str {
    w.nations[i].id.name()
}

struct Snap {
    alive: bool,
    gov: bool,
    electoral: bool,
    ruling: Option<Bloc>,
    stab: f64,
    d: f64,
    armed: bool,
    per_head: f64,
    leader: Option<String>,
    /// pre-tick readings for the verbose trace: challenger (bloc, I_W),
    /// weakest armed loyalty, mean loyalty, authoritarianism
    chall: Option<(Bloc, f64)>,
    weakest: f64,
    mean: f64,
    auth: f64,
}

fn snapshot(w: &WorldState) -> Vec<Snap> {
    w.nations
        .iter()
        .map(|n| {
            let id = n.id;
            let gov = n.alive && government::state(w, id).is_some();
            Snap {
                alive: n.alive,
                gov,
                electoral: gov && government::is_electoral(w, id),
                ruling: if gov { blocs::ruling_bloc(w, id) } else { None },
                stab: n.stability,
                d: if gov { blocs::discontent(w, id) } else { 0.0 },
                armed: gov && blocs::uprising_armed(w, id),
                per_head: n.gdp * 1000.0 / n.population.max(0.001),
                leader: government::state(w, id).and_then(|g| g.leader()).map(|s| s.to_string()),
                chall: if gov { blocs::challenger(w, id) } else { None },
                weakest: government::state(w, id).and_then(|g| g.weakest_armed()).map_or(1.0, |(_, v)| v),
                mean: government::state(w, id).map_or(1.0, |g| g.mean_loyalty()),
                auth: n.authoritarianism,
            }
        })
        .collect()
}

fn run_seed(seed: u64, verbose: bool, by_nation: &mut HashMap<&'static str, HashMap<&'static str, u32>>) -> Seed {
    let rules = GameRules { seed, ideology_blocs: true, ideology_takeover: true, ..GameRules::default() };
    let mut w = world_1990(rules);
    let mut out = Seed { seed, ..Default::default() };
    let start = snapshot(&w);
    let regimes_1990: Vec<usize> = (0..w.nations.len()).filter(|&i| start[i].gov && !start[i].electoral).collect();
    let low_auth: HashSet<usize> =
        (0..w.nations.len()).filter(|&i| start[i].alive && w.nations[i].authoritarianism <= 0.20).collect();
    let warsaw = ["Poland", "Hungary", "Czechoslovakia", "Romania", "Bulgaria", "USSR"];
    let mut excomm: HashSet<usize> = (0..w.nations.len())
        .filter(|&i| {
            let id = w.nations[i].id;
            start[i].alive
                && (government::regime_is_communist(id)
                    || start[i].ruling == Some(Bloc::Communist)
                    || warsaw.contains(&id.code()))
        })
        .collect();
    let big8 = ["China", "Vietnam", "Cuba", "NorthKorea", "SaudiArabia", "Syria", "Egypt", "Libya"];
    let big8_idx: Vec<(usize, Option<Bloc>)> = (0..w.nations.len())
        .filter(|&i| big8.contains(&w.nations[i].id.code()))
        .map(|i| (i, start[i].ruling))
        .collect();
    let mut opened_month: Vec<Option<usize>> = vec![None; w.nations.len()];
    let mut d_hist: Vec<Vec<f64>> = vec![vec![]; w.nations.len()];
    let mut a10: Vec<f64> = vec![];
    let mut islamist_set: HashSet<usize> = HashSet::new();
    let mut islamist_set_2000: HashSet<usize> = HashSet::new();
    let mut communist_set: HashSet<usize> = HashSet::new();
    let mut elcoup_by: HashMap<&'static str, u32> = HashMap::new();
    let mut alive_before = start.len();

    let mut bump = |table: &'static str, nation: &'static str| {
        *by_nation.entry(table).or_default().entry(nation).or_insert(0) += 1;
    };

    for m in 0..MONTHS {
        let pre = snapshot(&w);
        // A nation born this month has no pre-tick row; treat it as "other".
        if pre.len() > d_hist.len() {
            d_hist.resize(pre.len(), vec![]);
            opened_month.resize(pre.len(), None);
        }
        for (i, s) in pre.iter().enumerate() {
            if s.gov {
                d_hist[i].push(s.d);
                if d_hist[i].len() > 12 {
                    d_hist[i].remove(0);
                }
            }
        }
        let news = tick_month(&mut w, &[]);
        let post = snapshot(&w);
        if post.len() > alive_before {
            for i in alive_before..post.len() {
                excomm.insert(i);
            }
            alive_before = post.len();
            d_hist.resize(post.len(), vec![]);
            opened_month.resize(post.len(), None);
        }
        // Name maps for this month's headlines.
        let mut by_name: HashMap<String, usize> = HashMap::new();
        for i in 0..w.nations.len() {
            by_name.insert(name_of(&w, i).to_string(), i);
        }
        let mut events: HashMap<usize, Vec<Route>> = HashMap::new();
        let mut clauses: Vec<(usize, Option<Bloc>)> = vec![];
        for h in &news {
            if h.ends_with("dies in office.") {
                out.deaths += 1;
                continue;
            }
            let find = |name: &str| by_name.get(name).copied();
            if let Some(rest) = h.strip_prefix("COUP IN ") {
                let (nm, tail) = match rest.split_once(':') {
                    Some(x) => x,
                    None => continue,
                };
                let idx = by_name.iter().find(|(k, _)| k.to_uppercase() == nm).map(|(_, v)| *v);
                let idx = match idx {
                    Some(i) => i,
                    None => continue,
                };
                let r = if tail.contains("annuls the election") {
                    out.annul += 1;
                    bump("annul", name_of(&w, idx));
                    if name_of(&w, idx) == "Algeria" {
                        out.algeria_annulled = true;
                    }
                    Route::Annul
                } else if tail.contains("removes the elected government") {
                    out.coup_el += 1;
                    *elcoup_by.entry(name_of(&w, idx)).or_insert(0) += 1;
                    bump("elcoup", name_of(&w, idx));
                    Route::ElCoup
                } else {
                    out.coup_reg += 1;
                    bump("regcoup", name_of(&w, idx));
                    Route::RegCoup
                };
                out.coups_all += 1;
                if pre.get(idx).map_or(false, |s| s.per_head < 8000.0) {
                    out.coups_poor += 1;
                }
                if low_auth.contains(&idx) {
                    out.a6_route += 1;
                    bump("a6", name_of(&w, idx));
                }
                events.entry(idx).or_default().push(r);
            } else if let Some(rest) = h.strip_prefix("Revolution in ") {
                let (nm, winner) = if let Some((nm, tail)) = rest.split_once(": the ") {
                    let b = tail.split(" movement").next().unwrap_or("");
                    (nm, Bloc::ALL.iter().copied().find(|x| x.label() == b))
                } else {
                    (rest.split(" \u{2014}").next().unwrap_or(rest), None)
                };
                let idx = match find(nm) {
                    Some(i) => i,
                    None => continue,
                };
                match winner {
                    Some(b) => out.upr[b as usize] += 1,
                    None => out.upr[5] += 1,
                }
                let (stab12, armed) = pre.get(idx).map_or((false, false), |s| (s.stab < 12.0, s.armed));
                let c = match (stab12, armed) {
                    (true, false) => 0,
                    (false, true) => 1,
                    (true, true) => 2,
                    _ => 3,
                };
                out.upr_clause[c] += 1;
                clauses.push((idx, winner));
                if low_auth.contains(&idx) {
                    // A route event only where the MOVEMENT armed it pre-tick
                    // (clause movement or both); the stability < 12 collapse,
                    // and a "none" that crossed 12 inside the tick, are the
                    // pre-existing collapse and are reported beside it.
                    if armed {
                        out.a6_route += 1;
                        bump("a6", name_of(&w, idx));
                    } else {
                        out.a6_collapse += 1;
                        bump("a6_collapse", name_of(&w, idx));
                    }
                }
                events.entry(idx).or_default().push(Route::Uprising);
            } else if let Some(nm) = h.split(" votes: ").next().filter(|_| h.contains(" votes: ")) {
                if let Some(i) = find(nm) {
                    events.entry(i).or_default().push(Route::Ballot);
                }
            } else if let Some(rest) = h.strip_prefix("The government of ") {
                if let Some(nm) = rest.split(" falls;").next() {
                    if let Some(i) = find(nm) {
                        events.entry(i).or_default().push(Route::Ballot);
                    }
                }
            } else if let Some(nm) = h.split(" sets a date for its first free elections").next().filter(|_| h.contains(" sets a date for its first free elections")) {
                if let Some(i) = find(nm) {
                    events.entry(i).or_default().push(Route::Seam);
                }
            } else if let Some(nm) = h.strip_suffix(" opens up.") {
                if let Some(i) = find(nm) {
                    events.entry(i).or_default().push(Route::OpensUp);
                }
            } else if let Some(nm) = h.split(" convenes a round table").next().filter(|_| h.contains(" convenes a round table")) {
                out.rt_lever += 1;
                if let Some(i) = find(nm) {
                    events.entry(i).or_default().push(Route::RoundTable);
                    if low_auth.contains(&i) {
                        out.a6_route += 1;
                    }
                }
            } else if let Some(nm) = h.split(" suspends its constitution").next().filter(|_| h.contains(" suspends its constitution")) {
                out.suspend += 1;
                if let Some(i) = find(nm) {
                    events.entry(i).or_default().push(Route::Suspend);
                    if low_auth.contains(&i) {
                        out.a6_route += 1;
                        bump("a6", name_of(&w, i));
                    }
                }
            } else if let Some(nm) = h.split(" declares a ").next().filter(|_| h.contains(" declares a ") && h.ends_with("programme.")) {
                out.programme += 1;
                if let Some(i) = find(nm) {
                    events.entry(i).or_default().push(Route::Programme);
                }
            }
        }
        // Ruling-bloc changes, attributed.
        for i in 0..post.len() {
            let (before, after) = match (pre.get(i), &post[i]) {
                (Some(b), a) if b.gov && a.gov => (b, a),
                _ => continue,
            };
            if before.electoral
                && after.electoral
                && before.leader != after.leader
                && events.get(&i).map_or(false, |ev| ev.contains(&Route::Ballot))
            {
                out.ballot_party_flips += 1;
            }
            if before.ruling == after.ruling || after.ruling.is_none() {
                // An opening without a colour change still counts for A4.
                if !before.electoral && after.electoral && opened_month[i].is_none() {
                    opened_month[i] = Some(m);
                    let ev = events.get(&i).cloned().unwrap_or_default();
                    let attr = if ev.contains(&Route::Uprising) {
                        1
                    } else if ev.contains(&Route::RoundTable) {
                        2
                    } else if ev.contains(&Route::OpensUp) {
                        3
                    } else {
                        0
                    };
                    out.opened_attr[attr] += 1;
                    if m < END_1996 && regimes_1990.contains(&i) {
                        bump("opened_1996", name_of(&w, i));
                    }
                }
                continue;
            }
            let ev = events.get(&i).cloned().unwrap_or_default();
            let route = [Route::Annul, Route::ElCoup, Route::RegCoup, Route::Uprising, Route::Programme, Route::Seam, Route::Ballot, Route::Suspend, Route::RoundTable, Route::OpensUp]
                .iter()
                .copied()
                .find(|r| ev.contains(r))
                .unwrap_or(Route::Other);
            let new = after.ruling.unwrap();
            let nm = name_of(&w, i);
            match route {
                Route::Ballot if before.electoral && after.electoral => {
                    out.ballot_flips += 1;
                    if new == Bloc::Communist && excomm.contains(&i) {
                        out.excomm_run += 1;
                        bump("excomm_run", nm);
                        if m < END_1996 {
                            out.excomm_1996 += 1;
                            bump("excomm_1996", nm);
                        }
                    }
                }
                Route::Annul | Route::ElCoup | Route::RegCoup | Route::Uprising | Route::Programme => {
                    out.take[new as usize] += 1;
                    out.take_total += 1;
                    if new == Bloc::Islamist {
                        islamist_set.insert(i);
                        if m < END_2000 {
                            islamist_set_2000.insert(i);
                        }
                        bump("islamist_take", nm);
                        if nm == "Algeria" {
                            out.algeria_islamist = true;
                        }
                    }
                    if new == Bloc::Communist {
                        communist_set.insert(i);
                        bump("communist_take", nm);
                    }
                    // A10: the year before.
                    let hist = &d_hist[i];
                    if hist.len() >= 6 {
                        let mine = hist.iter().sum::<f64>() / hist.len() as f64;
                        let roster: Vec<f64> = d_hist
                            .iter()
                            .enumerate()
                            .filter(|(j, h)| h.len() >= 6 && pre.get(*j).map_or(false, |s| s.gov))
                            .map(|(_, h)| h.iter().sum::<f64>() / h.len() as f64)
                            .collect();
                        a10.push(mine - median(&roster));
                    }
                }
                _ => {
                    if nm == "Algeria" && new == Bloc::Islamist {
                        out.algeria_islamist = true;
                    }
                }
            }
            if !before.electoral && after.electoral && opened_month[i].is_none() {
                opened_month[i] = Some(m);
                let attr = match route {
                    Route::Uprising => 1,
                    Route::RoundTable => 2,
                    Route::OpensUp => 3,
                    _ => 0,
                };
                out.opened_attr[attr] += 1;
                if m < END_1996 && regimes_1990.contains(&i) {
                    bump("opened_1996", nm);
                }
            }
            if verbose {
                println!(
                    "  s{seed} m{m} {nm}: {:?} -> {:?} by {:?} | pre-tick stab {:.1} D {:.3} auth {:.2} chall {:?} weakest {:.3} mean {:.3} armed {} electoral {}",
                    before.ruling, after.ruling, route, before.stab, before.d, before.auth,
                    before.chall.map(|(b, v)| (b, (v * 1000.0).round() / 1000.0)), before.weakest, before.mean, before.armed, before.electoral
                );
            }
        }
        let _ = clauses;
        if m == END_1996 - 1 {
            out.opened_1996 = regimes_1990.iter().filter(|&&i| post[i].gov && post[i].electoral).count() as u32;
        }
        // Islamist ceiling: the largest Islamist influence in a nation the
        // bloc does not rule and could win.
        for i in 0..post.len() {
            if !post[i].gov || post[i].ruling == Some(Bloc::Islamist) {
                continue;
            }
            let id = w.nations[i].id;
            if !blocs::bloc_can_win(&w, id, Bloc::Islamist) {
                continue;
            }
            let v = blocs::influence(&w, id)[Bloc::Islamist as usize].1;
            if v > out.max_islamist_i {
                out.max_islamist_i = v;
                out.max_islamist_where = id.name();
            }
        }
    }
    let end = snapshot(&w);
    out.big8_keep = big8_idx.iter().filter(|(i, b)| end[*i].gov && end[*i].ruling == *b && b.is_some()).count() as u32;
    for (i, b) in &big8_idx {
        if !(end[*i].gov && end[*i].ruling == *b && b.is_some()) {
            bump("big8_lost", name_of(&w, *i));
        }
    }
    out.islamist_states = islamist_set.len() as u32;
    out.islamist_states_by_2000 = islamist_set_2000.len() as u32;
    out.communist_states = communist_set.len() as u32;
    out.a10_n = a10.len() as u32;
    out.a10_median = median(&a10);
    let mut v: Vec<(&'static str, u32)> = elcoup_by.into_iter().collect();
    v.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    out.top3_share = if out.coup_el > 0 {
        v.iter().take(3).map(|(_, c)| *c).sum::<u32>() as f64 / out.coup_el as f64
    } else {
        0.0
    };
    out.elcoup_nations = v.iter().map(|(n, _)| *n).collect();
    out
}

#[test]
#[ignore]
fn bloc_census() {
    let n: u64 = std::env::var("SPHERES_CENSUS_SEEDS").ok().and_then(|s| s.parse().ok()).unwrap_or(60);
    let verbose = std::env::var("SPHERES_CENSUS_VERBOSE").is_ok();
    let mut rows: Vec<Seed> = vec![];
    let mut by_nation: HashMap<&'static str, HashMap<&'static str, u32>> = HashMap::new();
    println!("bloc census: both switches on, monthly, {MONTHS} months, seeds 0..{n}");
    println!("seed | coupEl annul regCoup | upr W/C/N/I/NA/none | clause s12/mov/both/none | take W/C/N/I/NA ballot(bloc/party) | isl<=2000 isl comm | open96 (drift/upr/lever/card) | excomm96 run | a6 route/collapse | big8 | poor/coups | a10 med (n) | deaths rt susp prog | maxIslI | elcoup top3 share nations");
    for seed in 0..n {
        let r = run_seed(seed, verbose, &mut by_nation);
        println!(
            "s{:>3} | {:>2} {:>2} {:>2} | {:>2}/{:>2}/{:>2}/{:>2}/{:>2}/{:>2} | {:>3}/{:>3}/{:>2}/{:>2} | {:>2}/{:>2}/{:>2}/{:>2}/{:>2} {:>3}/{:>3} | {} {} {} | {:>2} ({}/{}/{}/{}) | {} {} | {} {} | {} | {}/{} | {:+.3} ({:>2}) | {:>2} {} {} {} | {:.3} {} | {:.2} {:?}",
            r.seed, r.coup_el, r.annul, r.coup_reg,
            r.upr[0], r.upr[1], r.upr[2], r.upr[3], r.upr[4], r.upr[5],
            r.upr_clause[0], r.upr_clause[1], r.upr_clause[2], r.upr_clause[3],
            r.take[0], r.take[1], r.take[2], r.take[3], r.take[4], r.ballot_flips, r.ballot_party_flips,
            r.islamist_states_by_2000, r.islamist_states, r.communist_states,
            r.opened_1996, r.opened_attr[0], r.opened_attr[1], r.opened_attr[2], r.opened_attr[3],
            r.excomm_1996, r.excomm_run,
            r.a6_route, r.a6_collapse,
            r.big8_keep,
            r.coups_poor, r.coups_all,
            r.a10_median, r.a10_n,
            r.deaths, r.rt_lever, r.suspend, r.programme,
            r.max_islamist_i, r.max_islamist_where,
            r.top3_share, &r.elcoup_nations[..r.elcoup_nations.len().min(5)]
        );
        rows.push(r);
    }
    let nf = rows.len() as f64;
    let col = |f: &dyn Fn(&Seed) -> f64| -> Vec<f64> { rows.iter().map(|r| f(r)).collect() };
    println!();
    println!("SUMMARY over {} seeds", rows.len());
    let coup_el = col(&|r| r.coup_el as f64);
    println!("A1 coups against elected governments per seed: {}", stats(&coup_el));
    println!("   band 4..14 on the median: P(median outside | n={}) by bootstrap = {:.4}", rows.len(), bootstrap_median_outside(&coup_el, 4.0, 14.0, rows.len()));
    println!("   top-3 nation share of a seed's coups: {}", stats(&col(&|r| r.top3_share)));
    println!("   annulments per seed: {}", stats(&col(&|r| r.annul as f64)));
    println!("   regime coups by a pillar per seed: {}", stats(&col(&|r| r.coup_reg as f64)));
    let isl_majority = rows.iter().filter(|r| r.islamist_states_by_2000 >= 1).count();
    println!("A2 seeds with >= 1 Islamist non-ballot takeover by end-2000: {}/{} = {:.3} (over the run: {}/{}); Algeria Islamist {}/{} annulled {}/{}",
        isl_majority, rows.len(), isl_majority as f64 / nf,
        rows.iter().filter(|r| r.islamist_states >= 1).count(), rows.len(),
        rows.iter().filter(|r| r.algeria_islamist).count(), rows.len(),
        rows.iter().filter(|r| r.algeria_annulled).count(), rows.len());
    println!("   Islamist uprisings per seed: {}; max Islamist influence (winnable, non-ruling): {}", stats(&col(&|r| r.upr[3] as f64)), stats(&col(&|r| r.max_islamist_i)));
    let comm = rows.iter().filter(|r| r.communist_states >= 1).count();
    println!("A3 seeds with >= 1 Communist non-ballot takeover over the run: {}/{} = {:.3}; Communist takeovers per seed: {}; Communist-won uprisings per seed: {}",
        comm, rows.len(), comm as f64 / nf, stats(&col(&|r| r.take[1] as f64)), stats(&col(&|r| r.upr[1] as f64)));
    let opened = col(&|r| r.opened_1996 as f64);
    println!("A4 1990 regimes electoral at end-1996 per seed: {}; band 15..40 on the median: P(outside) = {:.4}", stats(&opened), bootstrap_median_outside(&opened, 15.0, 40.0, rows.len()));
    println!("   openings over the run by attribution drift/uprising/lever/card: {} / {} / {} / {}",
        stats(&col(&|r| r.opened_attr[0] as f64)), stats(&col(&|r| r.opened_attr[1] as f64)), stats(&col(&|r| r.opened_attr[2] as f64)), stats(&col(&|r| r.opened_attr[3] as f64)));
    let ex96 = rows.iter().filter(|r| r.excomm_1996 >= 1).count();
    let ex96_3 = rows.iter().filter(|r| r.excomm_1996 >= 3).count();
    let exrun = rows.iter().filter(|r| r.excomm_run >= 1).count();
    println!("A5 ex-communist return by ballot: seeds with >= 1 by end-1996 {}/{} = {:.3}; >= 3 by end-1996 {}/{}; >= 1 over the run {}/{}; per seed by 1996 {}",
        ex96, rows.len(), ex96 as f64 / nf, ex96_3, rows.len(), exrun, rows.len(), stats(&col(&|r| r.excomm_1996 as f64)));
    println!("A6 route events in 1990 auth <= 0.20 nations per seed: {} (pre-existing stability<12 collapses beside: {}); seeds with zero route events {}/{}",
        stats(&col(&|r| r.a6_route as f64)), stats(&col(&|r| r.a6_collapse as f64)), rows.iter().filter(|r| r.a6_route == 0).count(), rows.len());
    let ratio: Vec<f64> = rows.iter().map(|r| r.ballot_flips as f64 / (r.take_total.max(1)) as f64).collect();
    let ratio_party: Vec<f64> = rows.iter().map(|r| r.ballot_party_flips as f64 / (r.take_total.max(1)) as f64).collect();
    println!("A7 ballot BLOC flips per seed {}; ballot PARTY flips per seed {}; non-ballot takeovers (bloc changes) per seed {}",
        stats(&col(&|r| r.ballot_flips as f64)), stats(&col(&|r| r.ballot_party_flips as f64)), stats(&col(&|r| r.take_total as f64)));
    println!("   ratio bloc-flips/takeovers per seed {}; ratio party-flips/takeovers per seed {} (bar: median >= 3)", stats(&ratio), stats(&ratio_party));
    let big = rows.iter().filter(|r| r.big8_keep >= 6).count();
    println!("A8 seeds with >= 6 of the big eight keeping their 1990 bloc to end-2010: {}/{} = {:.3}; per seed {}", big, rows.len(), big as f64 / nf, stats(&col(&|r| r.big8_keep as f64)));
    let poor: u32 = rows.iter().map(|r| r.coups_poor).sum();
    let all: u32 = rows.iter().map(|r| r.coups_all).sum();
    println!("A9 coups (all kinds) in nations under $8,000 per head: {}/{} = {:.3}", poor, all, poor as f64 / all.max(1) as f64);
    let a10: Vec<f64> = rows.iter().filter(|r| r.a10_n > 0).map(|r| r.a10_median).collect();
    println!("A10 per-seed median of (D in the year before a non-ballot takeover - roster median): {} (bar >= 0.25; seeds with data {})", if a10.is_empty() { "none".to_string() } else { stats(&a10) }, a10.len());
    println!("deaths in office per seed {}; round-table levers {}; suspensions {}; programmes {}",
        stats(&col(&|r| r.deaths as f64)), stats(&col(&|r| r.rt_lever as f64)), stats(&col(&|r| r.suspend as f64)), stats(&col(&|r| r.programme as f64)));
    println!("uprisings by winner W/C/N/I/NA/none: {} / {} / {} / {} / {} / {}",
        stats(&col(&|r| r.upr[0] as f64)), stats(&col(&|r| r.upr[1] as f64)), stats(&col(&|r| r.upr[2] as f64)), stats(&col(&|r| r.upr[3] as f64)), stats(&col(&|r| r.upr[4] as f64)), stats(&col(&|r| r.upr[5] as f64)));
    println!("uprising clause stab12/movement/both/none: {} / {} / {} / {}",
        stats(&col(&|r| r.upr_clause[0] as f64)), stats(&col(&|r| r.upr_clause[1] as f64)), stats(&col(&|r| r.upr_clause[2] as f64)), stats(&col(&|r| r.upr_clause[3] as f64)));
    println!();
    println!("BY NATION over all seeds (top 14 each)");
    let mut tables: Vec<&&str> = by_nation.keys().collect();
    tables.sort();
    for t in tables {
        let mut v: Vec<(&str, u32)> = by_nation[*t].iter().map(|(k, v)| (*k, *v)).collect();
        v.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
        let total: u32 = v.iter().map(|(_, c)| *c).sum();
        println!("  {:<14} total {:>4} distinct {:>3}: {:?}", t, total, v.len(), &v[..v.len().min(14)]);
    }
}
