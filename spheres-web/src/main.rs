//! Local web front end for SPHERES.
//!
//! The simulation stays the single source of truth: this server owns one
//! `WorldState`, applies commands through the same queue the CLI uses, and
//! serves a browser UI that renders it. No game logic lives here.

use spheres_sim::init::world_1990;
use spheres_sim::stratagems;
use spheres_sim::theatre::TheatreId;
use spheres_sim::world::*;
use spheres_sim::{apply_command, load, save, tick_month, Command};
use std::sync::Mutex;
use tiny_http::{Header, Method, Response, Server};

const INDEX: &str = include_str!("../ui/index.html");
/// Baked country outlines — see `src/bin/mapgen.rs`.
const WORLD_JS: &str = include_str!("../ui/world.js");
/// Baked admin-1 district outlines, same projection and canvas as world.js.
const DISTRICTS_JS: &str = include_str!("../ui/districts.js");
/// Baked hillshade underlay, same Robinson canvas — see tools/terrain/. Kept as the
/// fallback the map falls back to when WebGL2 is unavailable or a context is lost.
const TERRAIN_PNG: &[u8] = include_bytes!("../ui/terrain.png");
/// Baked ETOPO elevation: RGB-packed uint16 + sqrt depth — see tools/terrain/make_relief.py.
const RELIEF_PNG: &[u8] = include_bytes!("../ui/relief.png");
/// Baked signed coastline distance field, same Robinson canvas — see tools/terrain/make_coast.py.
const COAST_PNG: &[u8] = include_bytes!("../ui/coast.png");
/// Baked NE1 vegetation index, half-resolution — see tools/terrain/make_cover.py.
const COVER_PNG: &[u8] = include_bytes!("../ui/cover.png");
/// Baked signed lake-shoreline distance field, same Robinson canvas, same encode and same
/// clip as coast.png — see tools/terrain/make_lakes.py.
const LAKE_PNG: &[u8] = include_bytes!("../ui/lake.png");
/// Baked major rivers + lakes, same projection as world.js.
const RIVERS_JS: &str = include_str!("../ui/rivers.js");
/// Baked per-district terrain classes + feature names, same ids as
/// districts.js — see tools/terrain/classify_districts.py.
const TERRAIN_JS: &str = include_str!("../ui/terrain.js");
/// Baked 1990 resource transcription, same district ids as districts.js — see
/// tools/resources/. Served whole rather than reduced to a render payload: the
/// provenance, the confidence bands, the admission rules and the unlocated
/// producers are the point of the file, and a map that cannot show what is
/// behind a patch is the map this data was cleaned to avoid. The UI fetches it
/// lazily, only when the Resources shading is first opened.
const RESOURCES_JSON: &str = include_str!("../data/district_resources.json");

/// The six per-nation numbers the UI plots. Recorded every month so a decade of
/// stagnation reads as a shape rather than a pair of endpoints.
#[derive(Clone, Copy)]
struct Row {
    gdp: f64,
    growth: f64,
    inflation: f64,
    debt: f64,
    stability: f64,
    mil: f64,
}

/// The world as it stood at the start of one month.
struct Snapshot {
    t: u32, // months since Jan 1990
    year: i32,
    month: u32,
    oil: f64,
    rows: Vec<(NationId, Row)>,
}

/// A headline plus the handles the UI filters on: when it happened, what kind of
/// event it was, and who it was about.
struct Event {
    t: u32,
    date: String,
    text: String,
    cat: &'static str,
    tags: Vec<NationId>,
}

/// Sixty years of monthly history is 720 rows; the caps are there so a player who
/// runs the clock for centuries cannot make the server eat the machine.
const MAX_HISTORY: usize = 3000;
const MAX_LOG: usize = 4000;

struct Game {
    world: WorldState,
    log: Vec<Event>,
    history: Vec<Snapshot>,
}

impl Game {
    fn new(seed: u64, player: Option<NationId>) -> Game {
        let rules = GameRules { seed, ..GameRules::default() };
        let mut world = world_1990(rules);
        world.player = player;
        let mut g = Game { world, log: vec![], history: vec![] };
        g.snapshot();
        g
    }

    fn snapshot(&mut self) {
        let w = &self.world;
        self.history.push(Snapshot {
            t: month_index(w.year, w.month),
            year: w.year,
            month: w.month,
            oil: w.oil_price,
            rows: w
                .nations
                .iter()
                .filter(|n| n.alive)
                .map(|n| {
                    (
                        n.id,
                        Row {
                            gdp: n.gdp,
                            growth: n.growth_last,
                            inflation: n.inflation,
                            debt: n.debt_gdp,
                            stability: n.stability,
                            mil: n.mil_strength,
                        },
                    )
                })
                .collect(),
        });
        if self.history.len() > MAX_HISTORY {
            self.history.remove(0);
        }
    }

    fn record(&mut self, text: String) {
        let w = &self.world;
        self.log.push(Event {
            t: month_index(w.year, w.month),
            date: w.date_str(),
            cat: classify(&text),
            tags: mentioned(&text),
            text,
        });
        if self.log.len() > MAX_LOG {
            self.log.remove(0);
        }
    }

    /// Advance up to `months`, stopping early on an event worth reacting to.
    /// Returns whether it stopped early and why.
    fn advance(&mut self, months: usize, commands: Vec<Command>) -> (bool, Option<String>) {
        let mut queued = commands;
        for i in 0..months {
            let cmds = std::mem::take(&mut queued);
            let headlines = tick_month(&mut self.world, &cmds);
            for h in &headlines {
                self.record(h.clone());
            }
            self.snapshot();
            if let Some(me) = self.world.player {
                if !self.world.nation(me).alive {
                    return (true, Some(format!("{} no longer exists.", me.name())));
                }
            }
            if i + 1 < months {
                if let Some(e) = headlines.iter().find(|h| is_major(h, self.world.player)) {
                    return (true, Some(e.clone()));
                }
            }
        }
        (false, None)
    }
}

fn month_index(year: i32, month: u32) -> u32 {
    (((year - 1990) * 12) + month as i32 - 1).max(0) as u32
}

const MONTH_NAMES: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

fn month_name(month: u32, year: i32) -> String {
    format!("{} {}", MONTH_NAMES[(month.clamp(1, 12) - 1) as usize], year)
}

/// Bucket a headline for the event log's filters. Order matters: a nuclear test
/// that "the world condemns" is politics, not diplomacy.
fn classify(h: &str) -> &'static str {
    let t = h.to_lowercase();
    let war = t.starts_with("war:")
        || t.contains("invades")
        || t.contains("joins the war")
        || t.contains("capitulates")
        || t.contains("annexed")
        || t.contains("sues for peace")
        || t.contains("peace terms")
        || t.contains("white peace")
        || t.contains("repels");
    let politics = t.contains("dissolved")
        || t.contains("revolution")
        || t.contains("nuclear test")
        || t.contains("republics")
        || t.contains("regime");
    let diplomacy = t.contains("sanction") || t.contains("diplomatic hand");
    let economy = t.contains("oil") || t.contains("inflation") || t.contains("recession");
    if war {
        "war"
    } else if politics {
        "politics"
    } else if diplomacy {
        "diplomacy"
    } else if economy {
        "economy"
    } else {
        "other"
    }
}

/// Which nations a headline is about. The sim writes headlines with `id.name()`,
/// so a substring match on the full names is exact rather than a guess.
fn mentioned(h: &str) -> Vec<NationId> {
    let hay = h.to_lowercase(); // dissolution headlines shout in capitals
    let mut out = vec![];
    for id in all_nations() {
        if hay.contains(&id.name().to_lowercase()) && !out.contains(id) {
            out.push(*id);
        }
    }
    out
}

fn is_major(headline: &str, me: Option<NationId>) -> bool {
    let h = headline.to_lowercase();
    let structural = h.starts_with("war:")
        || h.contains("dissolved")
        || h.contains("has annexed")
        || h.contains("capitulates")
        || h.contains("revolution in")
        || h.contains("sues for peace")
        || h.contains("repels")
        || h.contains("escalates to rung")
        || h.contains("grants")
        || h.contains("revokes");
    structural || me.is_some_and(|m| h.contains(&m.name().to_lowercase()))
}

/// Pull `nation=` out of a query string and percent-decode it.
///
/// `NationId::parse` accepts display names, and eight of them contain a space
/// that a browser sends as `%20`. Reading the raw query means "Saudi Arabia"
/// arrives as `Saudi%20Arabia` and resolves to nothing — silently, since the
/// param is optional everywhere it is used.
fn nation_param(url: &str) -> Option<NationId> {
    let raw = url.split_once("nation=")?.1.split('&').next()?;
    let bytes = raw.as_bytes();
    let mut out = String::with_capacity(raw.len());
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'+' => {
                out.push(' ');
                i += 1;
            }
            b'%' if i + 2 < bytes.len() => {
                match u8::from_str_radix(&raw[i + 1..i + 3], 16) {
                    Ok(b) => {
                        out.push(b as char);
                        i += 3;
                    }
                    Err(_) => {
                        out.push('%');
                        i += 1;
                    }
                }
            }
            _ => {
                let c = raw[i..].chars().next()?;
                out.push(c);
                i += c.len_utf8();
            }
        }
    }
    NationId::parse(&out)
}

/// The operating areas, and whose consent an outsider needs in each. Sent whole
/// rather than per-conflict, because the access panel is playable on its own:
/// a host state that is in nobody's war still wants to see who is asking.
fn theatres_json(w: &WorldState) -> Vec<serde_json::Value> {
    w.theatres
        .iter()
        .map(|t| {
            serde_json::json!({
                "id": format!("{:?}", t.id),
                "name": t.id.name(),
                "home": t.home.iter().map(|n| format!("{:?}", n)).collect::<Vec<_>>(),
                "hosts": t.access_hosts.iter().map(|n| format!("{:?}", n)).collect::<Vec<_>>(),
                "host_names": t.access_hosts.iter().map(|n| n.name()).collect::<Vec<_>>(),
                "rough": t.rough,
                "urbanisation": t.urbanisation,
            })
        })
        .collect()
}

/// One conflict, with the ladder on it. The legacy keys (`attacker`,
/// `defender`, `progress`, the two ally lists) are kept byte-for-byte so the
/// existing war card keeps rendering while the new ones are added beside them.
fn conflict_json(w: &WorldState, c: &Conflict) -> serde_json::Value {
    let posture: Vec<serde_json::Value> = c
        .posture
        .iter()
        .map(|b| {
            serde_json::json!({
                "id": format!("{:?}", b.nation),
                "name": b.nation.name(),
                "side_a": c.side_of(b.nation) == Some(true),
                "rung": b.rung,
                "rung_name": spheres_sim::world::rung_name(b.rung),
                "ceiling": b.ceiling,
                "objective": b.objective.label(),
                "roe": b.roe.label(),
                "resolve": b.resolve,
                "red_line": b.red_line,
                "stake": b.stake,
                "months_at_rung": b.months_at_rung,
                "munitions": w.nation_opt(b.nation).map(|n| n.munitions),
                "deployable": spheres_sim::war::deployable_fraction(w, b.nation),
                "access": spheres_sim::theatre::has_access(w, b.nation, c.theatre),
                "home": spheres_sim::theatre::is_home(w, b.nation, c.theatre),
                // Whether this belligerent is answering on its own ground, which
                // is what the escalation discount hangs off. Computed by the sim
                // so the price the UI quotes and the price the queue charges
                // cannot drift apart.
                "defending_home": spheres_sim::commitment::defending_home(w, c, b.nation),
                "committed": spheres_sim::war::committed_force(w, c, b.nation),
            })
        })
        .collect();
    serde_json::json!({
        "id": c.id,
        "theatre": format!("{:?}", c.theatre),
        "theatre_name": c.theatre.name(),
        "class": format!("{:?}", c.class()),
        "attacker": c.attacker().name(),
        "attacker_id": format!("{:?}", c.attacker()),
        "defender": c.defender().name(),
        "defender_id": format!("{:?}", c.defender()),
        // Control is the ground held, -1..+1; the old progress bar was the same
        // quantity on a scale of a hundred, so the UI needs no arithmetic change.
        "progress": c.control * 100.0,
        "control": c.control,
        // The front: district -> hold, +1 side A / -1 side B, only the
        // contested ground (deviations plus the base-valued districts along a
        // hard edge — the sim's canonical map, small by construction). The
        // aggregate above IS this map area-weighted, so the two never
        // disagree. Rounded to 2dp; no adjacency ships to the client.
        "front": c
            .front
            .iter()
            .map(|(d, h)| {
                (d.clone(), serde_json::json!(((*h as f64) * 100.0).round() / 100.0))
            })
            .collect::<serde_json::Map<String, serde_json::Value>>(),
        // Encircled groups, each a sorted list of district ids.
        "pockets": c.pockets,
        "months": c.months,
        "frozen_since": c.frozen_since.map(|(y, m)| month_name(m, y)),
        "attacker_allies": c.side_a.iter().skip(1).map(|a| a.name()).collect::<Vec<_>>(),
        "defender_allies": c.side_b.iter().skip(1).map(|a| a.name()).collect::<Vec<_>>(),
        "posture": posture,
        "start": month_name(c.start_month, c.start_year),
    })
}

fn nation_json(w: &WorldState, n: &Nation) -> serde_json::Value {
    let me = w.player;
    serde_json::json!({
        "id": format!("{:?}", n.id),
        "name": n.id.name(),
        "alive": n.alive,
        "gdp": n.gdp,
        "gdp_pc": if n.population > 0.0 { n.gdp * 1000.0 / n.population } else { 0.0 },
        "population": n.population,
        "growth": n.growth_last,
        "inflation": n.inflation,
        "rate": n.interest_rate,
        "tax": n.tax_rate,
        "mil_spend": n.mil_spend_gdp,
        "state_invest": n.state_invest_gdp,
        // The UI's policy readout reproduces the growth arithmetic, and cannot do
        // it without the two terms the player never sets directly.
        "priv_invest": n.priv_invest_gdp,
        "tfp": n.tfp_trend,
        "debt": n.debt_gdp,
        "stability": n.stability,
        "political_capital": n.political_capital,
        "separatism": n.separatism,
        "mil_strength": n.mil_strength,
        "war_exhaustion": n.war_exhaustion,
        "nuclear": n.nuclear,
        "oil_mbd": n.oil_mbd,
        "command_economy": n.system == EconomySystem::Command,
        "authoritarianism": n.authoritarianism,
        "at_war": w.at_war(n.id),
        "relation": me.map(|m| w.relation(m, n.id)),
        "sanctioned_by_me": me.is_some_and(|m| w.is_sanctioning(m, n.id)),
        "sanctioning_me": me.is_some_and(|m| w.is_sanctioning(n.id, m)),
        "sanctioned_by_count": w.sanctioned_by_count(n.id),
        "export_share": if n.oil_mbd > 0.0 { w.oil_export_share(n.id) } else { 1.0 },
        // Every standing it holds, not just the one with the player — the detail
        // view is a dossier on that nation, not on your relationship with it.
        "relations": w
            .nations
            .iter()
            .filter(|o| o.alive && o.id != n.id)
            .map(|o| serde_json::json!({
                "id": format!("{:?}", o.id),
                "name": o.id.name(),
                "value": w.relation(n.id, o.id),
                "sanctioning": w.is_sanctioning(n.id, o.id),
                "sanctioned_by": w.is_sanctioning(o.id, n.id),
            }))
            .collect::<Vec<_>>(),
    })
}

/// What the world is offering one government this month, at the price the sim
/// charges. Availability, the reason, and the cost all come from
/// `spheres_sim::stratagems` — the server invents none of them, and `affordable`
/// is only the same comparison `apply_command` makes when it charges, surfaced
/// early so the button can be honest before it is pressed.
fn stratagems_json(w: &WorldState, id: NationId) -> serde_json::Value {
    let held = w.nation_opt(id).map_or(0.0, |n| n.political_capital);
    let offers: Vec<serde_json::Value> = stratagems::available(w, id)
        .into_iter()
        .map(|s| {
            serde_json::json!({
                "id": s.id,
                "name": s.name,
                "blurb": s.blurb,
                "because": s.because,
                "cost": s.cost,
                "affordable": held >= s.cost,
                "shortfall": (s.cost - held).max(0.0),
            })
        })
        .collect();
    serde_json::json!({
        "nation": format!("{:?}", id),
        "nation_name": id.name(),
        "political_capital": held,
        "offers": offers,
    })
}

fn round(v: f64, places: i32) -> f64 {
    if !v.is_finite() {
        return 0.0;
    }
    let m = 10f64.powi(places);
    (v * m).round() / m
}

/// The recorded time series, column-major. One nation's arrays start at `t0`
/// (successor states appear late) and simply stop when it dies, so a dead power's
/// line ends rather than running flat to the end of the game.
fn history_json(g: &Game, only: Option<NationId>) -> serde_json::Value {
    let mut order: Vec<NationId> = vec![];
    for s in &g.history {
        for (id, _) in &s.rows {
            if !order.contains(id) {
                order.push(*id);
            }
        }
    }
    if let Some(one) = only {
        order.retain(|id| *id == one);
    }

    let mut nations = serde_json::Map::new();
    for id in &order {
        let mut t0: Option<usize> = None;
        let mut gap = 0usize;
        let mut last: Option<Row> = None;
        let (mut gdp, mut growth, mut infl, mut debt, mut stab, mut mil) =
            (vec![], vec![], vec![], vec![], vec![], vec![]);
        let mut push = |r: Row| {
            gdp.push(round(r.gdp, 2));
            growth.push(round(r.growth, 5));
            infl.push(round(r.inflation, 5));
            debt.push(round(r.debt, 4));
            stab.push(round(r.stability, 2));
            mil.push(round(r.mil, 2));
        };
        for (i, s) in g.history.iter().enumerate() {
            match s.rows.iter().find(|(x, _)| x == id).map(|(_, r)| *r) {
                Some(r) => {
                    if t0.is_none() {
                        t0 = Some(i);
                    }
                    for _ in 0..std::mem::take(&mut gap) {
                        if let Some(p) = last {
                            push(p);
                        }
                    }
                    push(r);
                    last = Some(r);
                }
                None => {
                    if t0.is_some() {
                        gap += 1;
                    }
                }
            }
        }
        if let Some(t0) = t0 {
            nations.insert(
                format!("{:?}", id),
                serde_json::json!({
                    "name": id.name(),
                    "t0": t0,
                    "gdp": gdp, "growth": growth, "inflation": infl,
                    "debt": debt, "stability": stab, "mil": mil,
                }),
            );
        }
    }

    serde_json::json!({
        "t": g.history.iter().map(|s| s.t).collect::<Vec<_>>(),
        "labels": g.history.iter().map(|s| s.date_label()).collect::<Vec<_>>(),
        "oil": g.history.iter().map(|s| round(s.oil, 2)).collect::<Vec<_>>(),
        "metrics": ["gdp", "growth", "inflation", "debt", "stability", "mil"],
        "order": order.iter().map(|id| format!("{:?}", id)).collect::<Vec<_>>(),
        "nations": nations,
    })
}

impl Snapshot {
    fn date_label(&self) -> String {
        month_name(self.month, self.year)
    }
}

fn state_json(g: &Game, interrupt: Option<String>) -> serde_json::Value {
    let w = &g.world;
    let nations: Vec<serde_json::Value> = w
        .nations
        .iter()
        .filter(|n| n.alive)
        .map(|n| nation_json(w, n))
        .collect();
    let dead: Vec<serde_json::Value> = w
        .nations
        .iter()
        .filter(|n| !n.alive)
        .map(|n| serde_json::json!({ "id": format!("{:?}", n.id), "name": n.id.name() }))
        .collect();
    let wars: Vec<serde_json::Value> = w
        .conflicts
        .iter()
        .map(|c| conflict_json(w, c))
        .collect();
    // Newest first, and the whole archive — the event log is meant to be scrolled
    // back through, not just glanced at.
    let log: Vec<serde_json::Value> = g
        .log
        .iter()
        .rev()
        .map(|e| {
            serde_json::json!({
                "date": e.date,
                "t": e.t,
                "text": e.text,
                "cat": e.cat,
                "tags": e.tags.iter().map(|id| format!("{:?}", id)).collect::<Vec<_>>(),
            })
        })
        .collect();

    serde_json::json!({
        "date": w.date_str(),
        "year": w.year,
        "month": w.month,
        "t": month_index(w.year, w.month),
        "player": w.player.map(|p| format!("{:?}", p)),
        "player_name": w.player.map(|p| p.name()),
        "oil_price": w.oil_price,
        "nations": nations,
        "dead": dead,
        "wars": wars,
        // The sim's held/contested threshold for per-district front control,
        // served so the browser never re-derives it (its literal is only a
        // fallback for a server that predates this key).
        "front_held_band": spheres_sim::front::HELD_BAND,
        "theatres": theatres_json(w),
        "access": w.access.iter().map(|a| serde_json::json!({
            "theatre": format!("{:?}", a.theatre),
            "host": format!("{:?}", a.host),
            "host_name": a.host.name(),
            "seeker": format!("{:?}", a.seeker),
            "seeker_name": a.seeker.name(),
            "since": month_name(a.since_month, a.since_year),
        })).collect::<Vec<_>>(),
        "log": log,
        "flags": w.flags,
        // Delta-encoded: only districts whose owner differs from the 1990
        // default, keyed by district id, value the owner's nation code.
        // Usually empty early game; the browser composes default owners from
        // districts.js grouping and overlays these. Computed sim-side like
        // everything else here.
        "districts": spheres_sim::districts::deltas(w).into_iter()
            .map(|(d, o)| (d, serde_json::Value::String(format!("{:?}", o))))
            .collect::<serde_json::Map<String, serde_json::Value>>(),
        // Carried on every state payload rather than fetched separately, because
        // the offer list changes with the world: a month that breaks the
        // inflation closes the peg, and the panel must not be a frame behind.
        "stratagems": w.player.map(|p| stratagems_json(w, p)),
        "research": w.player.map(|p| research_json(w, p)),
        "interrupt": interrupt,
    })
}

/// Translate the UI's flat command objects into sim commands.
/// The research board: what each of the eight domains is working on, how far in,
/// and what else it could be doing instead.
///
/// Everything here is computed sim-side — the cost of a project, the months left
/// at the current rate, which technologies are startable. The browser renders it
/// and does no arithmetic of its own, which is the lesson from the growth model
/// it used to mirror in JavaScript and got wrong three ways.
fn research_json(w: &WorldState, me: NationId) -> serde_json::Value {
    let n = w.nation(me);
    let dev = (n.gdp * 1000.0 / n.population / 24000.0).min(1.0);
    let monthly = spheres_sim::tech::research_output(w, n, dev);
    let weights = spheres_sim::tech::domain_weights_of(w, n, dev);

    let domains: Vec<serde_json::Value> = spheres_sim::tech::DOMAINS
        .iter()
        .map(|d| {
            let di = d.index();
            let rate = monthly * weights[di];
            let (project, banked, cost) = match spheres_sim::tech::project_of(w, me, *d) {
                Some((def, banked, cost)) => (
                    serde_json::json!({ "id": def.id, "name": def.name, "year": def.earliest_year }),
                    banked,
                    cost,
                ),
                None => (serde_json::Value::Null, n.tech.progress[di], 0.0),
            };
            let months_left = if cost > banked && rate > 1e-9 {
                Some(((cost - banked) / rate).ceil() as i64)
            } else {
                None
            };
            let options: Vec<serde_json::Value> = spheres_sim::tech::eligible_projects(n, *d)
                .iter()
                .map(|def| serde_json::json!({
                    "id": def.id, "name": def.name, "year": def.earliest_year,
                }))
                .collect();
            serde_json::json!({
                "domain": format!("{:?}", d),
                "name": d.name(),
                "share": weights[di],
                "rate": rate,
                "project": project,
                "banked": banked,
                "cost": cost,
                "months_left": months_left,
                "known": spheres_sim::tech::registry().iter().enumerate()
                    .filter(|(i, def)| def.domain == *d && n.tech.knows_index(*i as u16))
                    .count(),
                "total": spheres_sim::tech::registry().iter()
                    .filter(|def| def.domain == *d).count(),
                "options": options,
            })
        })
        .collect();

    serde_json::json!({
        "nation": n.id.name(),
        "monthly": monthly,
        "priority": n.tech.priority.map(|d| format!("{:?}", d)),
        "priority_multiplier": spheres_sim::tech::PRIORITY_MULTIPLIER,
        "domains": domains,
    })
}

/// One domain's whole tree: every technology in it, what it costs this nation,
/// what it needs first, and whether that is already held.
///
/// Served on its own route rather than on the state payload because it is 30-odd
/// nodes a domain and the state is polled on every advance. The screen fetches a
/// domain when it opens one.
fn tech_tree_json(w: &WorldState, me: NationId, domain: spheres_sim::tech::Domain) -> serde_json::Value {
    use spheres_sim::tech;
    let n = w.nation(me);
    let reg = tech::registry();
    let focus = n.tech.focus[domain.index()];

    let nodes: Vec<serde_json::Value> = reg
        .iter()
        .enumerate()
        .filter(|(_, def)| def.domain == domain)
        .map(|(i, def)| {
            let idx = i as u16;
            let known = n.tech.knows_index(idx);
            let pre: Vec<&u16> = tech::prereqs_of(idx).iter().collect();
            let open = !known && pre.iter().all(|q| n.tech.knows_index(**q));
            serde_json::json!({
                "id": def.id,
                "name": def.name,
                "year": def.earliest_year,
                "era": format!("{:?}", def.era),
                "cost": tech::cost_of(w, me, idx),
                "list_cost": def.cost,   // static list price; cost < list_cost ⇒ diffusion discount
                "state": if known { "known" } else if open { "open" } else { "locked" },
                "focus": focus == Some(idx),
                // What holding it actually does, and what it opens. Without
                // these a tree is a list of names with prices on them.
                "effects": def.effects.iter().map(tech::describe_effect).collect::<Vec<_>>(),
                "unlocks": tech::unlocked_by(idx).iter().map(|u| {
                    let d = &reg[*u as usize];
                    serde_json::json!({
                        "name": d.name,
                        "domain": format!("{:?}", d.domain),
                        "year": d.earliest_year,
                    })
                }).collect::<Vec<_>>(),
                // Prerequisites carry their own domain, because a few cross it
                // and a node the screen cannot draw still has to be nameable.
                "prereqs": pre.iter().map(|q| {
                    let d = &reg[**q as usize];
                    serde_json::json!({
                        "id": d.id,
                        "name": d.name,
                        "domain": format!("{:?}", d.domain),
                        "known": n.tech.knows_index(**q),
                    })
                }).collect::<Vec<_>>(),
            })
        })
        .collect();

    serde_json::json!({
        "domain": format!("{:?}", domain),
        "name": domain.name(),
        "year": w.year,
        "priority": n.tech.priority.map(|d| format!("{:?}", d)),
        "nodes": nodes,
    })
}

fn parse_command(w: &WorldState, v: &serde_json::Value, me: NationId) -> Option<Command> {
    let kind = v.get("kind")?.as_str()?;
    let num = || v.get("value").and_then(|x| x.as_f64());
    let target = || {
        v.get("target")
            .and_then(|x| x.as_str())
            .and_then(NationId::parse)
    };
    let theatre = || {
        v.get("theatre")
            .and_then(|x| x.as_str())
            .and_then(TheatreId::parse)
    };
    let conflict = || {
        v.get("conflict")
            .and_then(|x| x.as_u64())
            .map(|x| x as u32)
    };
    let domain = || {
        v.get("domain")
            .and_then(|x| x.as_str())
            .and_then(spheres_sim::tech::Domain::parse)
    };
    Some(match kind {
        // Choosing what a domain's laboratories work on. A missing or empty
        // "tech" hands the choice back to them.
        "research_focus" => Command::SetResearchFocus {
            nation: me,
            domain: domain()?,
            tech: v
                .get("tech")
                .and_then(|x| x.as_str())
                .filter(|x| !x.is_empty())
                .map(|x| x.to_string()),
        },
        // Declaring, or standing down, the national programme.
        "research_priority" => Command::SetResearchPriority { nation: me, domain: domain() },
        "rate" => Command::SetInterestRate { nation: me, rate: num()? },
        "tax" => Command::SetTaxRate { nation: me, rate: num()? },
        "military" => Command::SetMilSpend { nation: me, share: num()? },
        "invest" => Command::SetStateInvest { nation: me, share: num()? },
        "sanction" => Command::Sanction { imposer: me, target: target()? },
        "lift" => Command::LiftSanction { imposer: me, target: target()? },
        "improve" => Command::ImproveRelations { from: me, to: target()? },
        "war" => Command::DeclareWar { attacker: me, defender: target()? },
        // The stratagem carries an id rather than a value or a target; the sim
        // re-checks availability and charges the political capital itself.
        "stratagem" => Command::EnactStratagem {
            nation: me,
            id: v.get("id")?.as_str()?.to_string(),
        },

        // --- The commitment ladder. Flat objects, mapped exactly the way
        // `rate` and `sanction` are: the UI never constructs a sim type. ---
        // The theatre is optional here and it is the difference between the
        // verb being reachable and not: a player picking a quarrel with a
        // neighbour should not first have to know which operating area the sim
        // files it under. Left out, it is the one a war between the two would
        // be fought in — the defender's own ground.
        "open_conflict" => Command::OpenConflict {
            opener: me,
            target: target()?,
            theatre: theatre()
                .unwrap_or_else(|| spheres_sim::war::theatre_between(w, me, target().unwrap())),
        },
        "join" => Command::JoinConflict {
            conflict: conflict()?,
            nation: me,
            side_a: v.get("side_a").and_then(|x| x.as_bool()).unwrap_or(false),
            objective: v
                .get("objective")
                .and_then(|x| x.as_str())
                .and_then(Objective::parse)
                .unwrap_or(Objective::Deny),
        },
        "commit" => Command::SetCommitment {
            conflict: conflict()?,
            nation: me,
            rung: num()? as u8,
        },
        "objective" => Command::SetObjective {
            conflict: conflict()?,
            nation: me,
            objective: Objective::parse(v.get("objective")?.as_str()?)?,
        },
        "roe" => Command::SetRoE {
            conflict: conflict()?,
            nation: me,
            roe: Roe::parse(v.get("roe")?.as_str()?)?,
        },
        "ceiling" => Command::SetCeiling {
            conflict: conflict()?,
            nation: me,
            rung: num()? as u8,
        },
        "red_line" => Command::SetRedLine {
            conflict: conflict()?,
            nation: me,
            resolve_floor: num()?,
        },

        // --- Access. `target` is the other party in every case; who is host
        // and who is seeker depends on which side of the table you sit. ---
        "request_access" => Command::RequestAccess {
            seeker: me,
            host: target()?,
            theatre: theatre()?,
        },
        "press_access" => Command::PressForAccess {
            seeker: me,
            host: target()?,
            theatre: theatre()?,
        },
        "grant_access" => Command::GrantAccess {
            host: me,
            seeker: target()?,
            theatre: theatre()?,
            grant: v.get("grant").and_then(|x| x.as_bool()).unwrap_or(true),
        },
        "revoke_access" => Command::RevokeAccess {
            host: me,
            seeker: target()?,
            theatre: theatre()?,
        },
        _ => return None,
    })
}

fn json_response(v: serde_json::Value) -> Response<std::io::Cursor<Vec<u8>>> {
    let body = serde_json::to_string(&v).unwrap_or_else(|_| "{}".into());
    Response::from_string(body).with_header(
        Header::from_bytes(&b"Content-Type"[..], &b"application/json; charset=utf-8"[..]).unwrap(),
    )
}

/// A refusal. Same JSON shape as every other answer, but carrying the status
/// code that says the request was the problem — a 200 with an `error` key is a
/// thing only the browser that wrote it knows how to read.
fn json_error(code: u16, v: serde_json::Value) -> Response<std::io::Cursor<Vec<u8>>> {
    json_response(v).with_status_code(code)
}

/// Start a fresh world for `player`, or refuse and leave `g` alone.
///
/// THE REFUSAL IS THE POINT, and it is not hypothetical. `NationId::parse`
/// resolves every id on the roster, and the roster includes the successor
/// states that do not exist on the start date — Namibia, and the republics that
/// only appear if a federation comes apart. `world_1990` does not seat them,
/// because `data::load_world` rejects a data file for a nation that is not a
/// starter. Handing one to `Game::new` therefore built a world whose `player`
/// pointed at nobody, and `state_json` on the very next line asked
/// `WorldState::nation` for it and hit `expect("nation")` — on the main thread,
/// which is where tiny-http's request loop lives, so the panic did not fail the
/// request, it killed the process. Every player on that server lost their game
/// because one of them clicked the wrong card.
///
/// `nation_opt` is the accessor for an id the world may not be holding, and its
/// own doc comment names this exact case. Ask it before committing, not after:
/// the guard belongs here, in the route that accepts player input, and NOT in
/// `WorldState::nation`, whose `expect` is a real invariant everywhere else.
///
/// Returns the payload and whether a game was actually started.
fn new_game(g: &mut Game, seed: u64, player: Option<NationId>) -> (serde_json::Value, bool) {
    let fresh = Game::new(seed, player);
    if let Some(id) = player {
        // Asked of the world that was just built rather than of `start_1990`,
        // so this stays true if the roster ever seats a nation it does not
        // today. A world is cheap enough to build and throw away once.
        if !fresh.world.nation_opt(id).is_some_and(|n| n.alive) {
            return (
                serde_json::json!({
                    "error": format!(
                        "{} is not on the board in January 1990 — it exists only \
                         if the state it succeeds comes apart. Choose another nation.",
                        id.name()
                    ),
                }),
                false,
            );
        }
    }
    *g = fresh;
    (state_json(g, None), true)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let port: u16 = args
        .iter()
        .position(|a| a == "--port")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(7777);

    let game: Mutex<Game> = Mutex::new(Game::new(1990, None));

    let addr = format!("127.0.0.1:{}", port);
    let server = match Server::http(&addr) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Could not bind {}: {}", addr, e);
            eprintln!("Another SPHERES may already be running. Try --port 7778.");
            std::process::exit(1);
        }
    };
    let url = format!("http://{}", addr);
    println!("SPHERES is running at {}", url);
    println!("Press Ctrl-C to stop the server.");
    open_browser(&url);

    for mut request in server.incoming_requests() {
        let url_path = request.url().split('?').next().unwrap_or("/").to_string();
        let method = request.method().clone();

        let mut body = String::new();
        if method == Method::Post {
            let _ = request.as_reader().read_to_string(&mut body);
        }
        let payload: serde_json::Value =
            serde_json::from_str(&body).unwrap_or(serde_json::Value::Null);

        // HEAD is routed like GET: tiny-http suppresses the response body for
        // HEAD requests on its own, so `curl -I` sees the same status and
        // headers as a GET instead of falling through to the 404 arm.
        let route_method = if method == Method::Head { Method::Get } else { method.clone() };
        let response = match (&route_method, url_path.as_str()) {
            (Method::Get, "/") | (Method::Get, "/index.html") => {
                let r = Response::from_string(INDEX).with_header(
                    Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..])
                        .unwrap(),
                );
                let _ = request.respond(r);
                continue;
            }
            (Method::Get, "/world.js") => {
                let r = Response::from_string(WORLD_JS).with_header(
                    Header::from_bytes(
                        &b"Content-Type"[..],
                        &b"application/javascript; charset=utf-8"[..],
                    )
                    .unwrap(),
                );
                let _ = request.respond(r);
                continue;
            }
            (Method::Get, "/districts.js") => {
                let r = Response::from_string(DISTRICTS_JS).with_header(
                    Header::from_bytes(
                        &b"Content-Type"[..],
                        &b"application/javascript; charset=utf-8"[..],
                    )
                    .unwrap(),
                );
                let _ = request.respond(r);
                continue;
            }
            (Method::Get, "/rivers.js") => {
                let r = Response::from_string(RIVERS_JS).with_header(
                    Header::from_bytes(
                        &b"Content-Type"[..],
                        &b"application/javascript; charset=utf-8"[..],
                    )
                    .unwrap(),
                );
                let _ = request.respond(r);
                continue;
            }
            (Method::Get, "/terrain.js") => {
                let r = Response::from_string(TERRAIN_JS).with_header(
                    Header::from_bytes(
                        &b"Content-Type"[..],
                        &b"application/javascript; charset=utf-8"[..],
                    )
                    .unwrap(),
                );
                let _ = request.respond(r);
                continue;
            }
            (Method::Get, "/resources.json") => {
                // Cacheable for a day on the same rationale as the baked
                // rasters: it is static transcription compiled into this
                // binary, and the Resources shading should not re-pull it on
                // every reload.
                let r = Response::from_string(RESOURCES_JSON)
                    .with_header(
                        Header::from_bytes(
                            &b"Content-Type"[..],
                            &b"application/json; charset=utf-8"[..],
                        )
                        .unwrap(),
                    )
                    .with_header(
                        Header::from_bytes(&b"Cache-Control"[..], &b"max-age=86400"[..]).unwrap(),
                    );
                let _ = request.respond(r);
                continue;
            }
            (Method::Get, "/terrain.png") => {
                // Identity encoding so the PNG ships with a Content-Length
                // (tiny-http otherwise chunks bodies over 32 KiB).
                //
                // Cacheable for a day: the underlay is static transcription
                // baked into this binary, and the UI's retained <image> node
                // only guards against re-requests within one page's life — a
                // reload should not pull 600 KB again either.
                let r = Response::from_data(TERRAIN_PNG.to_vec())
                    .with_chunked_threshold(usize::MAX)
                    .with_header(
                        Header::from_bytes(&b"Content-Type"[..], &b"image/png"[..]).unwrap(),
                    )
                    .with_header(
                        Header::from_bytes(
                            &b"Cache-Control"[..],
                            &b"public, max-age=86400"[..],
                        )
                        .unwrap(),
                    );
                let _ = request.respond(r);
                continue;
            }
            // The four GL terrain textures, on the same terms as /terrain.png above:
            // identity encoding for the Content-Length, and a day of cache because they are
            // static transcription baked into this binary. relief.png carries packed uint16
            // elevation plus baked sky occlusion, and coast.png and lake.png signed distance
            // fields, so all three are sampled as NUMBERS rather than looked at — the
            // generators assert that none ships a gAMA/sRGB/iCCP chunk, because a decoder
            // that gamma-corrected them would move the terrain and both shorelines.
            (Method::Get, "/relief.png") => {
                let r = Response::from_data(RELIEF_PNG.to_vec())
                    .with_chunked_threshold(usize::MAX)
                    .with_header(
                        Header::from_bytes(&b"Content-Type"[..], &b"image/png"[..]).unwrap(),
                    )
                    .with_header(
                        Header::from_bytes(
                            &b"Cache-Control"[..],
                            &b"public, max-age=86400"[..],
                        )
                        .unwrap(),
                    );
                let _ = request.respond(r);
                continue;
            }
            (Method::Get, "/coast.png") => {
                let r = Response::from_data(COAST_PNG.to_vec())
                    .with_chunked_threshold(usize::MAX)
                    .with_header(
                        Header::from_bytes(&b"Content-Type"[..], &b"image/png"[..]).unwrap(),
                    )
                    .with_header(
                        Header::from_bytes(
                            &b"Cache-Control"[..],
                            &b"public, max-age=86400"[..],
                        )
                        .unwrap(),
                    );
                let _ = request.respond(r);
                continue;
            }
            (Method::Get, "/cover.png") => {
                let r = Response::from_data(COVER_PNG.to_vec())
                    .with_chunked_threshold(usize::MAX)
                    .with_header(
                        Header::from_bytes(&b"Content-Type"[..], &b"image/png"[..]).unwrap(),
                    )
                    .with_header(
                        Header::from_bytes(
                            &b"Cache-Control"[..],
                            &b"public, max-age=86400"[..],
                        )
                        .unwrap(),
                    );
                let _ = request.respond(r);
                continue;
            }
            (Method::Get, "/lake.png") => {
                let r = Response::from_data(LAKE_PNG.to_vec())
                    .with_chunked_threshold(usize::MAX)
                    .with_header(
                        Header::from_bytes(&b"Content-Type"[..], &b"image/png"[..]).unwrap(),
                    )
                    .with_header(
                        Header::from_bytes(
                            &b"Cache-Control"[..],
                            &b"public, max-age=86400"[..],
                        )
                        .unwrap(),
                    );
                let _ = request.respond(r);
                continue;
            }
            (Method::Get, "/api/state") => {
                let g = game.lock().unwrap();
                json_response(state_json(&g, None))
            }
            (Method::Get, "/api/stratagems") => {
                // Defaults to the player; `?nation=Poland` asks what the world is
                // offering somebody else, which is the same question the map
                // already answers for every other quantity.
                let asked = request
                    .url()
                    .split_once("nation=")
                    .and_then(|(_, q)| NationId::parse(q.split('&').next().unwrap_or("")));
                let g = game.lock().unwrap();
                let r = match asked.or(g.world.player) {
                    Some(id) => json_response(stratagems_json(&g.world, id)),
                    None => json_response(serde_json::json!({
                        "error": "no nation chosen",
                        "political_capital": 0.0,
                        "offers": [],
                    })),
                };
                let _ = request.respond(r);
                continue;
            }
            (Method::Get, "/api/tech") => {
                let g = game.lock().unwrap();
                let asked = request
                    .url()
                    .split_once("domain=")
                    .and_then(|(_, q)| {
                        spheres_sim::tech::Domain::parse(q.split('&').next().unwrap_or(""))
                    });
                match (g.world.player, asked) {
                    (Some(me), Some(d)) => json_response(tech_tree_json(&g.world, me, d)),
                    _ => json_response(serde_json::json!({ "nodes": [] })),
                }
            }
            // Where a nation's opening figures came from. Static start-of-game
            // provenance, so it needs neither the lock nor the world — and must
            // not be served from the live Nation, whose numbers have moved.
            (Method::Get, "/api/sources") => {
                let id = nation_param(request.url());
                match id {
                    Some(id) => json_response(serde_json::json!({
                        "id": format!("{:?}", id),
                        "name": id.name(),
                        "sources": spheres_sim::data::sources_for(id),
                    })),
                    None => json_response(serde_json::json!({
                        "error": "unknown nation",
                    })),
                }
            }
            (Method::Get, "/api/history") => {
                let only = nation_param(request.url());
                let g = game.lock().unwrap();
                json_response(history_json(&g, only))
            }
            (Method::Post, "/api/new") => {
                let seed = payload.get("seed").and_then(|s| s.as_u64()).unwrap_or(1990);
                let player = payload
                    .get("nation")
                    .and_then(|s| s.as_str())
                    .and_then(NationId::parse);
                let mut g = game.lock().unwrap();
                match new_game(&mut g, seed, player) {
                    (v, true) => json_response(v),
                    (v, false) => json_error(400, v),
                }
            }
            (Method::Post, "/api/advance") => {
                let months = payload
                    .get("months")
                    .and_then(|m| m.as_u64())
                    .unwrap_or(1)
                    .min(1200) as usize;
                let mut g = game.lock().unwrap();
                let me = g.world.player;
                let cmds: Vec<Command> = match me {
                    Some(me) => payload
                        .get("commands")
                        .and_then(|c| c.as_array())
                        .map(|a| a.iter().filter_map(|v| parse_command(&g.world, v, me)).collect())
                        .unwrap_or_default(),
                    None => vec![],
                };
                let (_stopped, why) = g.advance(months, cmds);
                json_response(state_json(&g, why))
            }
            (Method::Post, "/api/command") => {
                // Apply a command immediately without advancing time — used for
                // diplomacy the player expects to take effect the moment they click.
                let mut g = game.lock().unwrap();
                let me = match g.world.player {
                    Some(m) => m,
                    None => {
                        let _ = request.respond(json_response(
                            serde_json::json!({ "error": "no nation chosen" }),
                        ));
                        continue;
                    }
                };
                let mut errors: Vec<String> = vec![];
                // The tick's own headlines are still sitting in the world and are
                // already in the log; only what these commands add is news.
                let before = g.world.headlines.len();
                if let Some(list) = payload.get("commands").and_then(|c| c.as_array()) {
                    for v in list {
                        // A command this build cannot parse used to be dropped in
                        // silence, which from the player's side is a button that
                        // does nothing and says nothing. Say it.
                        match parse_command(&g.world, v, me) {
                            Some(cmd) => {
                                if let Err(e) = apply_command(&mut g.world, &cmd) {
                                    errors.push(e);
                                }
                            }
                            None => errors.push(format!(
                                "That order did not make sense: {}",
                                serde_json::to_string(v).unwrap_or_default()
                            )),
                        }
                    }
                }
                let fresh: Vec<String> = g.world.headlines[before..].to_vec();
                for h in fresh {
                    g.record(h);
                }
                let mut out = state_json(&g, None);
                out["errors"] = serde_json::json!(errors);
                json_response(out)
            }
            (Method::Post, "/api/save") => {
                let g = game.lock().unwrap();
                match std::fs::write("save.json", save(&g.world)) {
                    Ok(_) => json_response(serde_json::json!({ "ok": true, "path": "save.json" })),
                    Err(e) => json_response(serde_json::json!({ "ok": false, "error": e.to_string() })),
                }
            }
            (Method::Post, "/api/load") => {
                let mut g = game.lock().unwrap();
                match std::fs::read_to_string("save.json").map_err(|e| e.to_string()).and_then(|s| load(&s)) {
                    Ok(w) => {
                        *g = Game { world: w, log: vec![], history: vec![] };
                        g.snapshot();
                        json_response(state_json(&g, None))
                    }
                    Err(e) => json_response(serde_json::json!({ "error": e })),
                }
            }
            _ => Response::from_string("not found").with_status_code(404),
        };
        let _ = request.respond(response);
    }
}

fn open_browser(url: &str) {
    #[cfg(target_os = "windows")]
    let _ = std::process::Command::new("cmd").args(["/C", "start", "", url]).spawn();
    #[cfg(target_os = "macos")]
    let _ = std::process::Command::new("open").arg(url).spawn();
    #[cfg(all(unix, not(target_os = "macos")))]
    let _ = std::process::Command::new("xdg-open").arg(url).spawn();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_nation_on_the_board_has_somewhere_to_be_drawn() {
        // Added when Spain became the first nation appended to the extensible
        // roster, because it exposed the one place where a half-added nation
        // fails SILENTLY. Everything else is loud: a roster row without a data
        // file fails `validate`, a data file without a roster row fails to
        // deserialize its id, a nation without a `Polity` panics
        // `every_government_is_reachable_in_january_1990`. Forget the entry in
        // ui/index.html's TERRITORY map and there is no error anywhere — the
        // nation simply never appears on the map, and its land is drawn as
        // unaligned scenery. With eighty nations arriving across ten branches
        // that is a merge resolution nobody would notice for weeks.
        //
        // Deliberately a substring check against the served HTML rather than a
        // JS parse: this file is shipped by `include_str!` and has no build
        // step, so the thing to assert on is the thing that reaches the
        // browser.
        let map = INDEX
            .split_once("const TERRITORY = {")
            .expect("ui/index.html still declares a TERRITORY map")
            .1
            .split_once("};")
            .expect("the TERRITORY map is still brace-terminated")
            .0;
        for id in spheres_sim::world::all_nations() {
            let key = format!("{:?}:", id);
            assert!(
                map.contains(&key),
                "{:?} is in the roster but not in TERRITORY in ui/index.html, \
                 so it would be drawn as unaligned land and nobody would be told",
                id
            );
        }
    }

    #[test]
    fn a_nation_with_a_space_in_its_name_survives_the_query_string() {
        // The failure this catches is silent: an undecoded "Saudi%20Arabia"
        // parses to None, and every route taking this param treats None as
        // "no filter" rather than as an error, so the UI would quietly show
        // the wrong thing instead of nothing.
        assert_eq!(nation_param("/api/sources?nation=Brazil"), Some(NationId::Brazil));
        assert_eq!(
            nation_param("/api/sources?nation=Saudi%20Arabia"),
            Some(NationId::SaudiArabia)
        );
        assert_eq!(
            nation_param("/api/sources?nation=South+Korea"),
            Some(NationId::SouthKorea)
        );
        assert_eq!(
            nation_param("/api/history?nation=United%20States&x=1"),
            Some(NationId::USA)
        );
        assert_eq!(nation_param("/api/state"), None);
        assert_eq!(nation_param("/api/sources?nation=Atlantis"), None);
    }

    #[test]
    fn a_nation_can_show_where_its_figures_came_from() {
        // The branch's whole point is that the provenance survived the move out
        // of Rust. It only survives as far as somebody can read it.
        let src = spheres_sim::data::sources_for(NationId::Brazil);
        assert!(!src.is_empty());
        assert!(src.join(" ").contains("2948%"));
    }

    /// The picker can offer a nation the world is not holding — every successor
    /// state is on the roster from the first tick but seated only when its
    /// federation comes apart, and the setup grid is built from a live world
    /// that may already have dissolved one. Handing such an id to `Game::new`
    /// used to build a world whose `player` pointed at nobody, and the first
    /// `state_json` after it walked into `WorldState::nation`'s `expect` and
    /// took the whole server process down with it (exit 101) — the browser saw
    /// a dropped connection, and every other player on that server lost their
    /// game too. `/api/new` now refuses the choice and says so, which is why
    /// this asserts on `new_game` rather than on `Game::new`.
    #[test]
    fn a_nation_the_world_is_not_holding_is_refused_not_fatal() {
        let succ = spheres_sim::world::successor_nations();
        assert!(
            !succ.is_empty(),
            "the roster must still carry successor states for this to mean anything"
        );

        let mut g = Game::new(1990, None);
        for id in succ {
            // The precondition: this is exactly the id the picker can offer and
            // the world does not hold.
            assert!(
                g.world.nation_opt(*id).is_none_or(|n| !n.alive),
                "{:?} is seated and alive in January 1990",
                id
            );
            let (v, ok) = new_game(&mut g, 1990, Some(*id));
            assert!(!ok, "{:?} is not on the board and must not be granted", id);
            assert!(
                v["error"].as_str().unwrap_or_default().contains(id.name()),
                "the refusal must name the nation it refused: {}",
                v
            );
            // Refused means refused: the world the player already had is
            // untouched, and in particular nobody has been made an observer.
            assert_eq!(g.world.player, None, "a refused choice must not be seated");
        }

        // The same call with a nation that IS on the board still works, so the
        // guard is a filter and not a wall.
        let (v, ok) = new_game(&mut g, 1990, Some(NationId::Poland));
        assert!(ok, "Poland is seated in 1990 and must be playable: {}", v);
        assert_eq!(g.world.player, Some(NationId::Poland));
        assert!(v["research"].is_object(), "a seated player gets a full payload");
    }

    #[test]
    fn headlines_land_in_the_right_bucket() {
        assert_eq!(classify("WAR: Iraq invades Kuwait!"), "war");
        assert_eq!(classify("Kuwait repels Iraq's invasion — the aggressor's regime totters."), "war");
        assert_eq!(classify("THE SOVIET UNION HAS DISSOLVED. Russia emerges as successor state."), "politics");
        assert_eq!(classify("Revolution in Poland — the old regime falls."), "politics");
        assert_eq!(classify("India conducts nuclear tests. The world condemns; deterrence descends on the subcontinent."), "politics");
        assert_eq!(classify("United States imposes sanctions on Iraq."), "diplomacy");
        assert_eq!(classify("Sanctions on Iraq are lifted."), "diplomacy");
    }

    #[test]
    fn headlines_name_the_nations_they_are_about() {
        assert_eq!(mentioned("WAR: Iraq invades Kuwait!"), vec![NationId::Iraq, NationId::Kuwait]);
        // Dissolution headlines shout, and the tag must survive the capitals.
        assert_eq!(
            mentioned("THE SOVIET UNION HAS DISSOLVED. Russia emerges as successor state."),
            vec![NationId::USSR, NationId::Russia]
        );
        assert!(mentioned("Oil steadies.").is_empty());
    }

    #[test]
    fn month_index_counts_from_january_1990() {
        assert_eq!(month_index(1990, 1), 0);
        assert_eq!(month_index(1990, 12), 11);
        assert_eq!(month_index(1991, 1), 12);
    }

    /// A power that dies must leave a line that ends, not one that runs flat to
    /// the end of the game, and a successor must start where it appeared.
    #[test]
    fn the_series_ends_when_a_nation_does() {
        let mut g = Game::new(1990, None);
        for _ in 0..360 {
            let hs = tick_month(&mut g.world, &[]);
            for h in hs {
                g.record(h);
            }
            g.snapshot();
        }
        let h = history_json(&g, None);
        let n = h["nations"].as_object().unwrap();
        let months = h["t"].as_array().unwrap().len();
        assert_eq!(months, 361, "one row per month plus the opening snapshot");

        let ussr = &n["USSR"];
        let ussr_end = ussr["t0"].as_u64().unwrap() as usize + ussr["gdp"].as_array().unwrap().len();
        assert!(ussr_end < months, "the USSR's line should stop when it does");

        let russia = &n["Russia"];
        assert_eq!(
            russia["t0"].as_u64().unwrap() as usize,
            ussr_end,
            "Russia picks up the month the Union ends"
        );
        for id in n.keys() {
            let s = &n[id];
            let len = s["gdp"].as_array().unwrap().len();
            for m in ["growth", "inflation", "debt", "stability", "mil"] {
                assert_eq!(s[m].as_array().unwrap().len(), len, "{} {} misaligned", id, m);
            }
            assert!(s["t0"].as_u64().unwrap() as usize + len <= months);
        }
    }

    /// The offer list is the world's, not the server's: put a nation in the state
    /// that opens a stratagem and it must appear, priced, with the reason
    /// attached.
    #[test]
    fn the_world_offers_the_player_something_and_says_why() {
        let mut g = Game::new(1990, Some(NationId::Poland));
        {
            let n = g.world.nation_mut(NationId::Poland);
            n.inflation = 0.45;
            n.political_capital = 90.0;
        }
        let j = stratagems_json(&g.world, NationId::Poland);
        assert_eq!(j["nation_name"], "Poland");
        assert_eq!(j["political_capital"], 90.0);
        let offers = j["offers"].as_array().unwrap();
        let peg = offers
            .iter()
            .find(|o| o["id"] == "currency_peg")
            .expect("inflation above 15% opens the peg");
        assert_eq!(peg["cost"], 26.0);
        assert_eq!(peg["affordable"], true);
        // Every field the panel prints must be non-empty, or the player is asked
        // to spend a term's standing on a blank card.
        for k in ["name", "blurb", "because"] {
            assert!(!peg[k].as_str().unwrap().is_empty(), "{} missing", k);
        }
    }

    /// A price the player cannot pay must read as unaffordable *before* they
    /// press it, and must still be refused if they do.
    #[test]
    fn a_price_the_player_cannot_pay_is_marked_and_refused() {
        let mut g = Game::new(1990, Some(NationId::Poland));
        {
            let n = g.world.nation_mut(NationId::Poland);
            n.inflation = 0.45;
            n.political_capital = 10.0;
        }
        let j = stratagems_json(&g.world, NationId::Poland);
        let peg = j["offers"]
            .as_array()
            .unwrap()
            .iter()
            .find(|o| o["id"] == "currency_peg")
            .unwrap()
            .clone();
        assert_eq!(peg["affordable"], false);
        assert_eq!(peg["shortfall"], 16.0);
        let cmd = Command::EnactStratagem {
            nation: NationId::Poland,
            id: "currency_peg".into(),
        };
        assert!(apply_command(&mut g.world, &cmd).is_err(), "it must refuse");
    }

    /// The verb the panel presses. A mechanic the player cannot reach from their
    /// seat is not a mechanic: this asserts the whole route, from the flat JSON
    /// the button posts to the world actually moving and the capital being spent.
    #[test]
    fn the_button_reaches_the_sim_through_the_command_route() {
        let mut g = Game::new(1990, Some(NationId::Poland));
        {
            let n = g.world.nation_mut(NationId::Poland);
            n.inflation = 0.45;
            n.political_capital = 90.0;
        }
        let posted = serde_json::json!({ "kind": "stratagem", "id": "currency_peg" });
        let cmd = parse_command(&g.world, &posted, NationId::Poland)
            .expect("the UI's shape must parse");
        match &cmd {
            Command::EnactStratagem { nation, id } => {
                assert_eq!(*nation, NationId::Poland);
                assert_eq!(id, "currency_peg");
            }
            other => panic!("wrong command: {:?}", other),
        }
        apply_command(&mut g.world, &cmd).unwrap();
        let n = g.world.nation(NationId::Poland);
        assert!(n.inflation <= 0.06, "the peg must break the inflation");
        assert_eq!(n.political_capital, 64.0, "26 political capital spent");
        // And having been taken, it is no longer on offer.
        let after = stratagems_json(&g.world, NationId::Poland);
        assert!(after["offers"]
            .as_array()
            .unwrap()
            .iter()
            .all(|o| o["id"] != "currency_peg"));
    }

    /// Every state payload carries the offers, so the panel cannot lag the world
    /// by a month after the clock moves.
    #[test]
    fn the_state_payload_carries_the_offers() {
        let mut g = Game::new(1990, Some(NationId::Poland));
        g.world.nation_mut(NationId::Poland).inflation = 0.45;
        let s = state_json(&g, None);
        assert_eq!(s["stratagems"]["nation"], "Poland");
        assert!(!s["stratagems"]["offers"].as_array().unwrap().is_empty());
        // A spectator with no nation gets null rather than a fabricated menu.
        let spectator = Game::new(1990, None);
        assert!(state_json(&spectator, None)["stratagems"].is_null());
    }

    /// The surface itself. These strings are the panel's structure: if the markup
    /// or the wiring is renamed away, this fails rather than shipping a model
    /// with no verb attached to it.
    #[test]
    fn the_panel_exists_and_is_wired_to_the_route() {
        // The card, one row per offer, and the four things a row must say.
        assert!(INDEX.contains("function stratagemsHtml"));
        assert!(INDEX.contains("class=\"strat"));
        assert!(INDEX.contains("data-strat="));
        assert!(INDEX.contains("class=\"why\""));
        // The balance, prominently: in the header and at the head of the panel.
        assert!(INDEX.contains("id=\"hdrPc\""));
        assert!(INDEX.contains("class=\"pcbig\""));
        // The verb, the hand that presses it, and the route it goes down. Match
        // to the delimiter: `contains("window.enact")` also passes for
        // `window.enactAnythingElse`, which is a test that cannot fail.
        assert!(INDEX.contains("window.enact = async"));
        assert!(INDEX.contains("enact(b.dataset.strat"));
        assert!(INDEX.contains("kind: \"stratagem\""));
        assert!(INDEX.contains("/api/command"));
        // Read from the payload the server actually sends.
        assert!(INDEX.contains("S.stratagems"));
        // No CDN, no build step.
        assert!(!INDEX.contains("https://"), "the UI must stay self-contained");
    }

    /// The terrain layer ships baked, like world.js: a real PNG behind
    /// /terrain.png, the generated river layer behind /rivers.js, and the
    /// page actually mounting both — all of it local, because the
    /// self-contained guard above binds every href to this binary.
    #[test]
    fn the_map_ships_terrain_and_rivers() {
        assert!(
            TERRAIN_PNG.starts_with(b"\x89PNG\r\n\x1a\n"),
            "terrain.png is not a PNG"
        );
        // The four GL terrain textures, baked by tools/terrain/make_relief.py,
        // make_coast.py, make_cover.py, make_occlusion.py (which repacks relief.png's
        // B plane on land) and make_lakes.py. Nothing else in this binary would notice a
        // truncated or absent artifact: the routes serve whatever bytes are included.
        for (name, bytes) in [
            ("relief.png", RELIEF_PNG),
            ("coast.png", COAST_PNG),
            ("cover.png", COVER_PNG),
            ("lake.png", LAKE_PNG),
        ] {
            assert!(bytes.starts_with(b"\x89PNG\r\n\x1a\n"), "{name} is not a PNG");
            // relief.png, coast.png and lake.png are sampled as numbers, not looked at. A
            // colour chunk would license a decoder to gamma-correct them, which destroys
            // the packed uint16 elevation outright and moves both shorelines' zero
            // crossings.
            for chunk in [&b"gAMA"[..], &b"sRGB"[..], &b"iCCP"[..]] {
                assert!(
                    !bytes.windows(4).any(|w| w == chunk),
                    "{name} carries a colour-management chunk"
                );
            }
        }
        assert!(RIVERS_JS.starts_with("// Generated by tools/terrain/make_rivers.py"));
        assert!(RIVERS_JS.contains("window.RIVERS="));
        assert!(!RIVERS_JS.contains("https://"), "rivers.js must stay self-contained");
        // The terrain class layer, baked the same way and read via the same
        // guard: `window.TERRAIN || { byId: {} }` in the page means a missing
        // route renders silently, so only this test notices a bad artifact.
        assert!(TERRAIN_JS.starts_with("// Generated by tools/terrain/classify_districts.py"));
        assert!(TERRAIN_JS.contains("window.TERRAIN={byId:"));
        assert!(!TERRAIN_JS.contains("https://"), "terrain.js must stay self-contained");
        // The page mounts the underlay, the river group and the class layer,
        // via the routes.
        assert!(INDEX.contains("src=\"/rivers.js\""));
        assert!(INDEX.contains("src=\"/terrain.js\""));
        // The PNG underlay is BOTH what the map draws today and the fallback the GL layer
        // drops back to on a lost context, so this literal must survive the GL work.
        assert!(INDEX.contains("/terrain.png"));
        assert!(INDEX.contains("id=\"riverg\""));
        // The GL underlay samples all four baked textures through these routes. The
        // page fetches them by literal string, so a renamed route is only caught here.
        for path in ["/relief.png", "/coast.png", "/cover.png", "/lake.png"] {
            assert!(INDEX.contains(path), "the GL layer does not reference {path}");
        }
        // The WebGL2 canvas must never eat pointer events: `pointerdown` gates on
        // `svg.contains(e.target)` (ui/index.html), so a canvas that took them would
        // kill pan, nation clicks and district hover in one stroke.
        assert!(INDEX.contains("#glmap { position: absolute; pointer-events: none;"));
        // `#version 300 es` must be the first bytes of every shader string -- a leading
        // newline is a silent compile failure, and the fallback would hide it.
        assert_eq!(
            INDEX.matches(" = `#version 300 es").count(),
            4,
            "expected four inline GLSL strings, each opening on the version directive"
        );
        // One fallback exit, and the class it removes to restore the PNG underlay.
        assert!(INDEX.contains("function glFail("));
        assert!(INDEX.contains("gl-on"));

        // ---- the svg-side window the ground shows through. Every rule here
        // fails SILENTLY if it is edited away: the map still renders, it just
        // renders the wrong thing, and only this test would notice.

        // `:not(:hover)` is load-bearing, not style. `svg.gl-on .nodeg path` is
        // (0,2,2) and beats the generic `.nodeg:hover path` at (0,2,1) -- which
        // would kill hover feedback outright in Fronts and Terrain while leaving
        // Political (its own :hover rule is (0,3,2)) working, so it would read as
        // "hover is broken in two modes" long after the change that did it.
        assert!(
            INDEX.contains("svg.gl-on .nodeg:not(:hover) path"),
            "the ground's fill-opacity rule must stay mutually exclusive with :hover"
        );
        // The two-variable idiom the rule above consumes: --w is emitted per path
        // by renderMap and --fop stamped per camera frame by applyCam. Written as
        // a plain attribute the ramp would lag a whole tick behind the gesture.
        assert!(INDEX.contains("style=\"--w:${op}\""));
        assert!(INDEX.contains("svg.style.setProperty(\"--fop\""));

        // The ocean and the unaligned world are each TWO paths: a fill that
        // yields to the GL ground, and a non-scaling stroke that always draws
        // because GL cannot rule a line that ignores the camera. Collapse either
        // pair back into one element and the map loses its projection border, or
        // every unaligned country's outline, with nothing else complaining.
        assert!(INDEX.contains("<path class=\"oceanfill\" d=\"${WORLD.frame}\" fill=\"url(#ocean)\"/>"));
        assert!(INDEX.contains(
            "<path d=\"${WORLD.frame}\" fill=\"none\" stroke=\"#38404c\" \
             stroke-width=\"1\" vector-effect=\"non-scaling-stroke\"/>"
        ));
        assert!(INDEX.contains("<path class=\"landfill\" d=\"${scenery}\""));
        assert!(INDEX.contains(
            "<path d=\"${scenery}\" fill=\"none\" stroke=\"#161c25\" \
             stroke-width=\".4\" vector-effect=\"non-scaling-stroke\"/>"
        ));

        // Four modes carry a ground and four deliberately do not: the thematic
        // reads are preserved by the ABSENCE of this key, which is what stands
        // the whole layer down rather than merely turning it to zero.
        //
        // This count was 3 until Resources became its own map mode. Resources is
        // a reading of the PHYSICAL ground -- where the ore is -- so it earns a
        // ground block on exactly the same argument Terrain does, and it was
        // authored with one (a quiet u: [0.30, 0.55] under a wash that must stay
        // the subject). The number moved because a mode was ADDED, not because a
        // mode silently lost its ground, which is the failure this assertion
        // exists to catch: raise it only alongside a new `ground:` block you can
        // name, and never lower it to make a red test green.
        assert_eq!(
            INDEX.matches("\n    ground: {").count(),
            4,
            "exactly Political, Fronts, Terrain and Resources may carry a MAP_MODES ground block"
        );
        // ...and the four thematic modes must still have none. Asserted as a
        // total so the count above cannot be satisfied by a thematic mode
        // gaining a ground while a physical one loses it -- the exact swap a
        // bare count is blind to.
        for mode in ["relations", "stability", "growth", "economy"] {
            let at = INDEX
                .find(&format!("\n  {mode}: {{"))
                .unwrap_or_else(|| panic!("MAP_MODES lost its {mode} mode"));
            // Each MAP_MODES entry closes on a `},` at two-space indent, so that
            // is the delimiter -- NOT the next "\n  ", which every four-space
            // line inside the block also matches.
            let rest = &INDEX[at + 1..];
            let end = rest.find("\n  },").map(|e| e + 1).unwrap_or(rest.len());
            assert!(
                !rest[..end].contains("\n    ground: {"),
                "{mode} is a thematic mode: colour IS the data, so it must carry no ground"
            );
        }
    }

    /// The front seam is drawn twice, each pass clipped by its own SVG mask,
    /// and the two masks must carry DISTINCT ids — a merge that collapses the
    /// `a`/`b` suffixes leaves one mask shadowing the other and the seam
    /// renders one-sided, silently. The literals are asserted as they appear
    /// in the template string, because that is what reaches the browser.
    #[test]
    fn the_front_edge_masks_keep_distinct_ids() {
        assert!(
            INDEX.contains("foem${w.id}a"),
            "side A's front-edge mask id is gone from ui/index.html"
        );
        assert!(
            INDEX.contains("foem${w.id}b"),
            "side B's front-edge mask id is gone from ui/index.html"
        );
    }

    /// The district layer is delta-encoded: an untouched 1990 world sends an
    /// empty object, and a district moved by the sim shows up keyed by its
    /// stable id with the new owner's code — which is all the UI needs to
    /// overlay ownership on the default grouping baked into districts.js.
    #[test]
    fn the_state_payload_carries_district_deltas() {
        let mut g = Game::new(1990, None);
        let s = state_json(&g, None);
        let d = s["districts"].as_object().expect("districts is an object");
        assert!(d.is_empty(), "a fresh 1990 world must send no district deltas");
        // The held/contested band rides along so the browser's front readouts
        // use the sim's threshold, not a client-side copy of it.
        assert_eq!(
            s["front_held_band"].as_f64(),
            Some(spheres_sim::front::HELD_BAND),
            "the state payload must carry the sim's held band"
        );

        // Move Kuwait's capital governorate to Iraq the way an annexation
        // would, and the payload must say exactly that and nothing else.
        spheres_sim::districts::annex_all(&mut g.world, NationId::Iraq, NationId::Kuwait);
        let s = state_json(&g, None);
        let d = s["districts"].as_object().unwrap();
        assert_eq!(d.len(), 6, "all six Kuwaiti governorates moved");
        assert_eq!(d["KW-KU"], "Iraq");
    }

    #[test]
    fn one_nation_can_be_asked_for_alone() {
        let mut g = Game::new(1990, None);
        for _ in 0..24 {
            tick_month(&mut g.world, &[]);
            g.snapshot();
        }
        let h = history_json(&g, Some(NationId::Japan));
        let n = h["nations"].as_object().unwrap();
        assert_eq!(n.len(), 1);
        assert!(n.contains_key("Japan"));
        assert_eq!(h["oil"].as_array().unwrap().len(), 25);
    }

    /// Every tree node carries the static list price beside the per-nation
    /// cost, so the screen can say "Procurement: 84 pts — list 260" without
    /// mirroring the registry client-side. The pair only means something under
    /// two invariants: `list_cost` IS the registry's static cost, identical
    /// for every nation, and the per-nation `cost` never exceeds it —
    /// diffusion only ever discounts.
    #[test]
    fn the_tree_payload_carries_the_list_price() {
        use spheres_sim::tech::{self, Domain, DOMAINS};

        let mut g = Game::new(1990, Some(NationId::Poland));
        let reg = tech::registry();

        // A fresh 1990 world. This used to assert flatly that `cost == list`
        // for every node, on the premise that "nobody has fielded anything, so
        // no discount can exist" — a statement about the roster, not about the
        // payload, and one the 1990 technology endowment falsified: 103 of the
        // 137 nations now open holding something, so Poland reads a real
        // diffusion discount on `core_cmos_submicron` and 47 other nodes before
        // the first tick.
        //
        // The claim is therefore asked per node against what the world actually
        // holds, which is strictly more than the blanket version said: a node
        // nobody holds must still price at list to the digit, and a node
        // somebody holds must not price above it. No tolerance and no threshold
        // is introduced — the condition is read off `w.nations`.
        //
        // Every registry node must still ride in exactly one domain's response,
        // because the screen stitches all eight to cover the whole tree.
        let mut seen = 0;
        let mut discounted = 0;
        for d in DOMAINS {
            let j = tech_tree_json(&g.world, NationId::Poland, d);
            for node in j["nodes"].as_array().unwrap() {
                let id = node["id"].as_str().unwrap();
                let idx = tech::index_of(id).expect("payload ids are registry ids");
                let list = node["list_cost"]
                    .as_f64()
                    .expect("every node carries list_cost");
                assert_eq!(list, reg[idx as usize].cost, "{}: list price is the registry's", id);
                let cost = node["cost"].as_f64().unwrap();
                assert!(cost <= list, "{}: a discount can only cut, never add", id);
                let anyone_holds = g.world.nations.iter().any(|n| n.tech.knows_index(idx));
                if anyone_holds {
                    discounted += 1;
                } else {
                    assert_eq!(cost, list, "{}: nobody holds it, so nothing is discounted", id);
                }
                seen += 1;
            }
        }
        assert_eq!(seen, reg.len(), "the eight domain responses must cover the whole registry");
        assert!(
            discounted > 0,
            "no 1990 technology is held by anybody — the endowment has gone, and \
             the branch above is no longer being exercised"
        );

        // Hand the United States a root technology and the price Poland reads
        // must fall below list — the diffusion discount this field exists to
        // make visible.
        let root = reg
            .iter()
            .enumerate()
            .find(|(i, t)| t.domain == Domain::Computing && tech::prereqs_of(*i as u16).is_empty())
            .map(|(i, _)| i as u16)
            .expect("Computing has a root technology");
        g.world.nation_mut(NationId::USA).tech.known = vec![root];
        let j = tech_tree_json(&g.world, NationId::Poland, Domain::Computing);
        let node = j["nodes"]
            .as_array()
            .unwrap()
            .iter()
            .find(|n| tech::index_of(n["id"].as_str().unwrap()) == Some(root))
            .unwrap();
        let (cost, list) = (node["cost"].as_f64().unwrap(), node["list_cost"].as_f64().unwrap());
        assert!(
            cost < list,
            "once the world holds a technology its per-nation cost must fall below list ({} !< {})",
            cost,
            list
        );
    }
}
