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
        // Was the player's nation ALREADY gone when this call arrived? The
        // interrupt below is a piece of news, and news is told once. Without
        // this latch it fired on the first month of every later run too, so a
        // player whose nation had dissolved got one month per request for the
        // rest of the game however many they asked for — measured at 337 calls
        // to reach 337 months, and there is no gesture in the UI that skips it.
        // The `i + 1 < months` line below is the same idea for the ordinary
        // interrupt: do not stop on a condition the caller cannot act on.
        //
        // `nation_opt`, not `nation`: a dissolution may take the row out of the
        // world entirely, and the accessor with the `expect` in it has already
        // cost this server one process today.
        let gone = |g: &Game, me: NationId| !g.world.nation_opt(me).is_some_and(|n| n.alive);
        let already_gone = self.world.player.is_some_and(|me| gone(self, me));
        let mut queued = commands;
        for i in 0..months {
            let cmds = std::mem::take(&mut queued);
            let headlines = tick_month(&mut self.world, &cmds);
            for h in &headlines {
                self.record(h.clone());
            }
            self.snapshot();
            if !already_gone {
                if let Some(me) = self.world.player {
                    if gone(self, me) {
                        return (true, Some(format!("{} no longer exists.", me.name())));
                    }
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
///
/// EXTENDED 2026-09-01, against the corpus rather than against a guess. The four
/// named buckets between them reached one dispatch in ten: measured over four
/// seeds and thirty years, **11,285 of 12,552 headlines — 89.9% — landed in
/// "other"**, and the Economy filter matched **exactly nothing**, so its chip
/// never appeared on the filter row at all. `classify_corpus` beside this is the
/// measurement, and re-runs it on demand.
///
/// The keywords below are the sim's own vocabulary, read off the `headline`
/// format strings in spheres-sim rather than invented. The commitment ladder
/// ("escalates to rung", "steps back to rung") is the single largest unreachable
/// group at 1,197 of the corpus; then patronage, then the pact and trade
/// headlines, then election results.
///
/// EVERY ADDITION IS STRICTLY A MOVE OUT OF "other". Nothing that already
/// matched has been moved, reordered or removed, and no added keyword occurs in
/// any headline a lower-priority bucket was already reaching —
/// `the_log_filters_reach_the_world_the_sim_writes` pins the whole table so a
/// later edit cannot quietly steal one bucket's headlines for another.
///
/// STILL DELIBERATELY IN "other": `{} is first to field {}`, 339 of the corpus.
/// A technology milestone is not war, politics, diplomacy or economy, and the
/// bucket it wants does not exist. Adding one is a change to the filter row in
/// ui/index.html, which is a UI decision and not this function's to make.
fn classify(h: &str) -> &'static str {
    let t = h.to_lowercase();
    let war = t.starts_with("war:")
        // Widened from `invades`, which missed "tears up its own guarantee to
        // {} to invade it" — the one headline where a war opens in the middle
        // of a sentence about a broken pact.
        || t.contains("invade")
        || t.contains("joins the war")
        || t.contains("enters the war")
        || t.contains("capitulates")
        || t.contains("annexed")
        || t.contains("sues for peace")
        || t.contains("peace terms")
        || t.contains("white peace")
        || t.contains("repels")
        // The commitment ladder. Every rung headline the sim writes carries the
        // word: "escalates to rung", "steps back to rung", "publicly rules out
        // going beyond rung", "falls back to rung".
        || t.contains("rung")
        || t.contains("magazines are empty")
        || t.contains("quits the fight")
        || t.contains("takes a side against")
        || t.contains("freezes over")
        || t.contains("objective is now")
        || t.contains("defend its own ground");
    let politics = t.contains("dissolved")
        || t.contains("revolution")
        || t.contains("nuclear test")
        // "{} tests a nuclear device" is the OTHER proliferation headline and
        // the substring "nuclear test" does not occur in it.
        || t.contains("nuclear device")
        || t.contains("weapons programme")
        || t.contains("republics")
        || t.contains("regime")
        || t.contains("coup in")
        // Election results are written as "{} votes: {Party} ...".
        || t.contains("votes:")
        || t.contains("elections")
        || t.contains("goes to the polls")
        || t.contains("goes to the country")
        || t.contains("the government")
        || t.contains("loses its majority")
        || t.contains("parliament refuses")
        || t.contains("its own streets")
        || t.contains("street protest")
        || t.contains("gloves off")
        || t.contains("ends conscription")
        || t.contains("scandal")
        // The two dissolution-aftermath lines, which are the only headlines the
        // sim writes with no `{}` in them at all and were the last strays.
        || t.contains("inherits the arsenal")
        || t.contains("remain in belgrade's hands");
    let diplomacy = t.contains("sanction")
        || t.contains("diplomatic hand")
        || t.contains("defence pact")
        || t.contains("abandons its pact")
        || t.contains("guarantee")
        || t.contains("trade agreement")
        || t.contains("trade pact")
        || t.contains("trade talks")
        || t.contains("trade between")
        || t.contains("buys the loyalty")
        || t.contains("basing")
        || t.contains("overflight")
        || t.contains("opens its bases")
        || t.contains("use of its territory")
        || t.contains("use of its bases")
        || t.contains("public quarrel")
        // Covert action, both outcomes: "A covert operation against {} comes to
        // nothing" and the one that did not, "Separatist fighters in {} turn up
        // with weapons nobody will account for".
        || t.contains("covert operation")
        || t.contains("turn up with weapons")
        // Patronage, which the sim writes four ways and all of them end in a
        // sum per year: arms sales, economic aid, raised aid, expanded transfers.
        || t.contains("arms sales")
        || t.contains("arms transfers")
        || t.contains("economic aid")
        || t.contains("aid to")
        // "{} cuts off {} to {}" takes AidKind::label(), so the economic arm
        // already matched on "economic aid" and only the arms arm was adrift.
        // Deliberately NOT the bare "cuts off", which would take
        // "{} cuts off oil to {}" out of the economy bucket it already reaches.
        || t.contains("cuts off arms");
    let economy = t.contains("oil")
        || t.contains("inflation")
        || t.contains("recession")
        || t.contains("economy outward")
        || t.contains("opens up.")
        || t.contains("state's industry")
        || t.contains("frees prices")
        || t.contains("austerity")
        || t.contains("external debt")
        || t.contains("creditors")
        || t.contains("pegs its currency")
        || t.contains("industrial plant");
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

/// Does `hay` NAME this nation, rather than merely contain its letters?
///
/// The bare `contains` this replaces reads "Romania" as a mention of **Oman**,
/// and across the 137-nation roster that is the one collision — checked, not
/// assumed: no other nation's name is a substring of another's. One is enough.
/// Measured on the live server, governing Oman on seed 1990 for 300 months, the
/// player's own "You" filter held **sixteen dispatches, of which fifteen were
/// about Romania** — its elections, its street protests — and one was about
/// Oman. The same tags drive the chart's per-nation marks, and the same match
/// in `is_major` stopped an Omani player's advance for Romanian election
/// results.
///
/// A boundary is "not flanked by a letter", which keeps every real mention:
/// possessives ("Iraq's magazines"), punctuation ("invades Kuwait!"), and the
/// hyphenated names on the roster all end at a non-letter.
fn names_nation(hay_lower: &str, name_lower: &str) -> bool {
    if name_lower.is_empty() {
        return false;
    }
    let mut from = 0;
    while let Some(i) = hay_lower[from..].find(name_lower) {
        let start = from + i;
        let end = start + name_lower.len();
        let letter_before =
            hay_lower[..start].chars().next_back().is_some_and(|c| c.is_alphabetic());
        let letter_after = hay_lower[end..].chars().next().is_some_and(|c| c.is_alphabetic());
        if !letter_before && !letter_after {
            return true;
        }
        from = end;
    }
    false
}

/// Which nations a headline is about. The sim writes headlines with `id.name()`,
/// so matching on the full names is exact rather than a guess — provided the
/// match respects word boundaries. See [`names_nation`] for the one roster pair
/// that proves it must.
fn mentioned(h: &str) -> Vec<NationId> {
    let hay = h.to_lowercase(); // dissolution headlines shout in capitals
    let mut out = vec![];
    for id in all_nations() {
        if names_nation(&hay, &id.name().to_lowercase()) && !out.contains(id) {
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
    // `names_nation`, not `contains`, for the reason recorded there: the bare
    // test read every Romanian headline as news about Oman, and stopped an
    // Omani player's advance for Romanian election results.
    structural || me.is_some_and(|m| names_nation(&h, &m.name().to_lowercase()))
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
        // A nation that no longer exists is not standing on a rung. The sim
        // keeps a dissolved state's posture for the month it takes the war
        // systems to notice; see the filter in `state_json` for what was
        // measured. A three-cornered war can outlive one of its parties, so the
        // conflict is still served while the row for the dead one is not.
        .filter(|b| w.nation_opt(b.nation).is_some_and(|n| n.alive))
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
        // The COUNT is still served, because two readouts legitimately want a
        // count: the map's ⊘ mark and the dossier's "Sanctioned by N nations".
        "sanctioned_by_count": w.sanctioned_by_count(n.id),
        // The DRAG is served because the policy panel wants growth, and a count
        // has not been how this model prices sanctions since the four channels
        // were converted to weigh output. The browser was still multiplying the
        // count by the pre-conversion coefficient; `economy::growth_drag_of_
        // sanctions` is the sim's own expression and `tick` computes the number
        // it charges from the same function.
        "sanction_drag": spheres_sim::economy::growth_drag_of_sanctions(
            w.sanction_weight(n.id),
        ),
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

/// Round to `digits` SIGNIFICANT figures rather than to a fixed decimal place.
///
/// For the magnitudes in the history payload there is no decimal place that is
/// right for the whole roster: the United States opens at $5,800bn and Sao Tome
/// and Principe at $0.12bn, six orders of magnitude apart in the same array of
/// the same response. Two places was right for the superpowers and destroyed
/// everything small — measured on the live server, Sao Tome's ninety-five-month
/// GDP series came back holding exactly TWO distinct values for an economy that
/// had moved continuously through a 7% decline.
///
/// How many figures is set by what a chart has to resolve, which is the series'
/// RANGE and not its level. Sao Tome moves 1.3% of its own output across a
/// decade; six significant figures put roughly sixteen hundred levels inside
/// that movement, which is more than a chart has pixels, and four would put
/// only sixteen. Six also leaves the large nations exactly where they were —
/// `round(5800.123456, 2)` and `round_sig(5800.123456, 6)` are both 5800.12 —
/// so this is a strict improvement rather than a trade, at a cost of at most
/// two characters a number.
///
/// Not for rates. `growth`, `inflation`, `debt` and `stability` are bounded
/// quantities where a fixed place IS the right precision and where significant
/// figures would spend digits on a number near zero; they keep `round`.
fn round_sig(v: f64, digits: i32) -> f64 {
    if !v.is_finite() || v == 0.0 {
        return 0.0;
    }
    let magnitude = v.abs().log10().floor() as i32;
    // Clamped so a denormal cannot ask for 10^300 and come back as an infinity.
    let places = (digits - 1 - magnitude).clamp(-30, 30);
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
        // The two columns that are MAGNITUDES go out at four significant
        // figures; the four that are RATES or bounded scores keep a fixed
        // decimal place, which is the right precision for a number that lives
        // near zero. See `round_sig` for why the distinction matters here and
        // nowhere else in this file.
        let mut push = |r: Row| {
            gdp.push(round_sig(r.gdp, 6));
            growth.push(round(r.growth, 5));
            infl.push(round(r.inflation, 5));
            debt.push(round(r.debt, 4));
            stab.push(round(r.stability, 2));
            mil.push(round_sig(r.mil, 6));
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
    // A conflict is a thing BETWEEN parties, so one that no longer has a living
    // party on each side is not one, and a belligerent that no longer exists is
    // not standing on a rung. The sim keeps a dissolved state's posture for the
    // month it takes the war systems to notice, and this payload was serving
    // that month as fact — the same response listed the Soviet Union under
    // `dead` and under `wars[].posture` at rung 6, "standoff strike", stake
    // 0.45. Measured over twelve seeds and thirty years: three occurrences,
    // each lasting exactly one month, all at the dissolution.
    //
    // Filtered HERE and nowhere else. The sim's own conflict list is untouched
    // and prunes itself on the following tick exactly as it did; this decides
    // only what the browser is told, which is the half that can be got wrong
    // without changing what the model asserts about history.
    let alive = |id: &NationId| w.nation_opt(*id).is_some_and(|n| n.alive);
    let wars: Vec<serde_json::Value> = w
        .conflicts
        .iter()
        .filter(|c| c.side_a.iter().any(alive) && c.side_b.iter().any(alive))
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
        // WHO IS RUNNING MONETARY POLICY, which is real, permanent, and was
        // invisible. `politics::tick` runs the player's central bank on their
        // behalf until they first issue a rate command, and skips their seat
        // for the rest of the game afterwards. The interest-rate slider is
        // therefore a one-way door and looked like every other slider: a player
        // could not tell whether the rate on screen was their policy or the
        // bank's, nor that moving it dismisses the bank for good.
        "player_set_rate": w.player_set_rate,
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
        "policy": w.player.map(|p| policy_json(w, p)),
        "interrupt": interrupt,
    })
}

/// What the policy sliders will actually buy, answered by the sim.
///
/// THE ONE THING HERE IS THE FORCE CURVE, and it is a curve rather than a
/// coefficient because the quantity has no closed form. `war::sustained_force`
/// is
///
///     sqrt(gdp · share · 0.30) · 8 · military_multiplier · adequacy_at(share)
///       + military_floor
///
/// and `adequacy_at` FALLS as the share rises, so the whole thing is not
/// `k·sqrt(share)` however much it looks like it. The browser was computing
/// `sqrt(gdp · share · 0.30) · 8` — the first factor only — and printing the
/// answer as "sustains a force of N" under the military slider. Three of the
/// four factors were missing: the technology multiplier (0.5x to 4.0x), the
/// equipment adequacy (0.55x to 1.0x), and the flat floor a modern arsenal
/// carries whatever the budget. Wrong by -38% to +42% on the first screen a
/// player sees, with no input from them at all.
///
/// SAMPLED, NOT SOLVED. The slider takes values in thousandths, so the sim
/// evaluates its own function at every thousandth from 0 to [`FORCE_CURVE_MAX`]
/// and serves the lot. The browser indexes; it does not interpolate and it does
/// not re-derive. 401 floats for the player's nation alone, on a payload that
/// already carries a 137x136 relation matrix.
///
/// The RANGE is the server's own and is deliberately wider than the slider's
/// 0..0.35, so this is not a second copy of a UI bound. A page whose slider ever
/// goes past the last sample clamps to it, and
/// `the_force_line_is_the_force_the_sim_sustains` asserts the curve covers what
/// the shipped page can actually ask for.
fn policy_json(w: &WorldState, me: NationId) -> serde_json::Value {
    let n = w.nation(me);
    let curve: Vec<f64> = (0..=FORCE_CURVE_STEPS)
        .map(|i| {
            let share = i as f64 / FORCE_CURVE_STEPS as f64 * FORCE_CURVE_MAX;
            round(spheres_sim::war::sustained_force(n, share), 3)
        })
        .collect();
    serde_json::json!({
        "force_curve": curve,
        "force_curve_max": FORCE_CURVE_MAX,
        // What the nation is actually spending buys, so the standing line needs
        // no lookup at all and cannot be a sample out.
        "sustained": spheres_sim::war::sustained_force(n, n.mil_spend_gdp),
    })
}

/// The widest military share the force curve is served for, and how many samples
/// it is cut into — a thousandth apiece, which is the resolution a range input
/// stepping in thousandths can actually select.
const FORCE_CURVE_MAX: f64 = 0.40;
const FORCE_CURVE_STEPS: usize = 400;

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
            let (project, banked, cost, fields_in) = match spheres_sim::tech::project_of(w, me, *d)
            {
                Some((def, banked, cost)) => (
                    serde_json::json!({ "id": def.id, "name": def.name, "year": def.earliest_year }),
                    banked,
                    cost,
                    Some(def.earliest_year),
                ),
                None => (serde_json::Value::Null, n.tech.progress[di], 0.0, None),
            };
            // A projection, and a projection is only worth serving while its
            // one assumption holds: that this month's research rate is the rate
            // for the whole wait. That is fair over a few years and a fiction
            // over a century — the rate moves with output, development and the
            // domain weights every single month. Past the horizon the division
            // does not become imprecise, it becomes meaningless, and the screen
            // printed the meaninglessness to the month: a microstate's Aerospace
            // board came back at 626,193 (fifty-two thousand years) and
            // microstate-04 saw ten digits of it.
            //
            // 1200 months is the span this server will already talk about at
            // once — the cap /api/advance puts on a single request — so it is
            // the longest wait a player can put a number against. Beyond it the
            // payload says nothing rather than something false.
            //
            // Taken FROM that cap rather than restated beside it. The sentence
            // above says the two are the same number for the same reason, and a
            // number two places have to agree on is one that will eventually
            // stop agreeing.
            const PROJECTION_HORIZON: f64 = MAX_ADVANCE as f64;
            let months_left = if cost > banked && rate > 1e-9 {
                let m = ((cost - banked) / rate).ceil();
                (m.is_finite() && m <= PROJECTION_HORIZON).then_some(m as i64)
            } else {
                None
            };
            // WHY THERE IS NO NUMBER, when there is no number. `months_left` was
            // the whole of what the payload said about a wait, and the browser
            // rendered every one of its four `null`s as the single word
            // "stalled". Only one of the four is a stall.
            //
            // The one that is simply FALSE is `banked >= cost`: the project is
            // paid for. It is waiting on the calendar, because the spend loop in
            // tech::tick will not field a technology before its `earliest_year`
            // however much is banked against it — and until then the board told
            // the player the programme they had fully funded had stopped. A
            // government reading "stalled" moves money to it, and there is
            // nothing the money can do.
            //
            // Served as a REASON rather than as a phrase, so the page keeps the
            // wording and the server keeps the fact.
            let wait = if months_left.is_some() {
                "months"
            } else if project.is_null() {
                // No project, so no wait to describe. The board prints "no
                // project chosen" here and never reaches the eta.
                "none"
            } else if banked >= cost {
                match fields_in {
                    // Paid for, and the year it can be fielded has not arrived.
                    Some(y) if y > w.year => "year",
                    // Paid for, and it lands on the next advance.
                    _ => "funded",
                }
            } else if rate > 1e-9 {
                // Funded, moving, and further off than this server will project.
                "beyond"
            } else {
                // The only real stall: nothing is reaching this domain at all.
                "stalled"
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
                "wait": wait,
                "fields_in": fields_in,
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
        "open_conflict" => {
            let target = target()?;
            Command::OpenConflict {
                opener: me,
                target,
                // Absent is the documented default the paragraph above
                // describes. PRESENT BUT UNUSABLE is not the same thing, and it
                // used to fall into the same branch: `theatre().unwrap_or_else`
                // could not tell a field that was not carried from one the
                // server could not read, so a typo took the default silently.
                // Measured on the live server, all with 200 and `errors: []` —
                // "Balkans" opened in the Balkans, and "Gluf", "", and 42 all
                // opened in the Gulf, indistinguishable from asking for it.
                theatre: match v.get("theatre") {
                    None | Some(serde_json::Value::Null) => {
                        spheres_sim::war::theatre_between(w, me, target)
                    }
                    Some(x) => TheatreId::parse(x.as_str()?)?,
                },
            }
        }
        // Both fields split the same way the theatre above does, and for the
        // same reason: not carrying one is a default, carrying one the server
        // cannot read is a refusal. Measured before that split, with 200 and
        // `errors: []` on every line — `side_a: 1` and `side_a: "true"` both
        // enrolled the player on the side they were asking to fight, and
        // `objective: "siez"` bought Deny, which is the one objective that
        // seizes nothing. Fourteen political capital charged either way.
        "join" => Command::JoinConflict {
            conflict: conflict()?,
            nation: me,
            side_a: match v.get("side_a") {
                None | Some(serde_json::Value::Null) => false,
                Some(x) => x.as_bool()?,
            },
            objective: match v.get("objective") {
                None | Some(serde_json::Value::Null) => Objective::Deny,
                Some(x) => Objective::parse(x.as_str()?)?,
            },
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

/// The body of a POST, or the reason it cannot be used.
///
/// A body that is THERE but does not parse is a FAILED request, not an empty
/// one, and the difference is the whole of this function. It used to be
/// `from_str(&body).unwrap_or(Value::Null)`, so a truncated or corrupt body
/// became the same thing as no body at all and every route then read its own
/// default out of nothing and reported success. Measured on the live server
/// against a body cut off mid-object:
///
///   POST /api/command  -> 200, "errors": []          (having read no commands)
///   POST /api/advance  -> 200, moved ONE month       (the body asked for sixty)
///   POST /api/new      -> 200, a fresh 1990 world    (the game in progress gone)
///
/// The first of those contradicts this file's own stated intent, written into
/// the /api/command arm: a command this build cannot parse is reported rather
/// than dropped, "because from the player's side [that] is a button that does
/// nothing and says nothing". A body this build cannot parse is the same thing
/// one level up. The last is the dangerous one — a malformed request silently
/// destroying the world the player was in.
///
/// An ABSENT body still means "no arguments": /api/save is posted empty and is
/// entitled to be.
fn parse_body(body: &str) -> Result<serde_json::Value, String> {
    if body.trim().is_empty() {
        return Ok(serde_json::Value::Null);
    }
    serde_json::from_str(body).map_err(|e| format!("That request body is not JSON: {}", e))
}

/// The seed the server boots into and the one `/api/new` uses when the request
/// does not ask for another. Matches `GameRules::default()`.
const DEFAULT_SEED: u64 = 1990;

/// The seed the request asked for.
///
/// "same seed, same history" is printed on the setup screen beside the box, and
/// it is the whole contract of a deterministic sim — so a seed the server
/// cannot use has to be said out loud rather than quietly replaced. It was
/// replaced: `as_u64().unwrap_or(1990)` turned a string, a negative number and
/// a fraction alike into the default, and the player was handed 1990's history
/// with no indication it was not theirs.
///
/// Measured on the live server by fingerprinting the state six months in:
///
///   {"seed":1990,...}    -> 6B60D853FEC58666
///   {"seed":"12345",...} -> 6B60D853FEC58666   <- asked for 12345
///   {"seed":-1,...}      -> 6B60D853FEC58666
///   {"seed":3.5,...}     -> 6B60D853FEC58666
///   {"seed":12345,...}   -> 3267FEB6F4A4A872   <- what 12345 actually is
///
/// Three requests asking for three different worlds, all silently given a
/// fourth. An ABSENT seed is still the default, because that is a request that
/// did not ask.
fn asked_seed(payload: &serde_json::Value) -> Result<u64, String> {
    match payload.get("seed") {
        None | Some(serde_json::Value::Null) => Ok(DEFAULT_SEED),
        Some(v) => v.as_u64().ok_or_else(|| {
            format!(
                "{} is not a seed. A seed is a whole number from 0 to {}, \
                 and the same one always gives the same history.",
                v,
                u64::MAX
            )
        }),
    }
}

/// The longest run of months this server will advance in one request, and the
/// same span the research board will project a wait across.
const MAX_ADVANCE: u64 = 1200;

/// How far the request asked the clock to move.
///
/// Time is the one thing in this game that cannot be given back — there is no
/// un-advance — so a request the server cannot read must not be answered by
/// moving the world some other distance. It was: `as_u64().unwrap_or(1)` turned
/// every unusable value into one month, and one month is also what a request
/// that asked for nothing gets, so the two were indistinguishable in the answer.
///
/// Measured on the live server, Poland on seed 7, from a fresh 1990 each time:
///
///   {"months":12}                     -> Jun 1990   (5, stopped by an event)
///   {"months":-5}                     -> Feb 1990   (1)
///   {"months":"12"}                   -> Feb 1990   (1)
///   {"months":3.5}                    -> Feb 1990   (1)
///   {"months":999999999999999999999}  -> Feb 1990   (1)
///   {"months":[12]}                   -> Feb 1990   (1)
///   {}                                -> Feb 1990   (1)   <- the real default
///
/// A client asking for five years and given one month is out by sixty, and the
/// 200 it gets back looks exactly like success. An ABSENT or null `months` is
/// still one month, because that is a request that did not ask.
///
/// The CLAMP is a different thing and is left alone: a request for more than
/// [`MAX_ADVANCE`] is answered with [`MAX_ADVANCE`] months of history, which is
/// a limit on the work rather than a substitution of the question.
fn asked_months(payload: &serde_json::Value) -> Result<u64, String> {
    match payload.get("months") {
        None | Some(serde_json::Value::Null) => Ok(1),
        Some(v) => v
            .as_u64()
            .map(|m| m.min(MAX_ADVANCE))
            .ok_or_else(|| {
                format!(
                    "{} is not a number of months. Months are a whole number \
                     from 0 to {}, and the clock only moves forwards.",
                    v, MAX_ADVANCE
                )
            }),
    }
}

/// Who the request asked to govern.
///
/// `Ok(None)` is an OBSERVER, and it is a real answer: the server boots into
/// exactly that state and the map is worth watching without a seat. But an
/// observer is what you get when you ASK for nothing — no `nation` key, or an
/// explicit null. It is not what you should get when you ask for something the
/// roster does not know.
///
/// It was. `payload.get("nation").and_then(as_str).and_then(NationId::parse)`
/// folds "unknown nation" into "no nation", so POST /api/new
/// {"nation":"Polnad"} answered 200 and started a game with no player in it:
/// no dashboard, no research board, no orders, nothing to spend political
/// capital on, and no word anywhere about why. Measured on the live server —
/// both "Atlantis" and the far likelier typo "Polnad" left `player` empty.
///
/// A refusal here is also the only place a misspelling can be CAUGHT, because
/// `NationId::parse` already accepts codes, display names and aliases
/// case-insensitively; anything it rejects is a name no spelling of which is
/// on the board.
fn asked_player(payload: &serde_json::Value) -> Result<Option<NationId>, String> {
    match payload.get("nation") {
        None | Some(serde_json::Value::Null) => Ok(None),
        Some(v) => {
            let asked = v.as_str().unwrap_or_default().trim();
            NationId::parse(asked).map(Some).ok_or_else(|| {
                format!(
                    "There is no nation called {} on the board. \
                     Names, codes and common aliases all work — try \"Poland\" or \"POL\".",
                    if asked.is_empty() { v.to_string() } else { format!("\"{asked}\"") }
                )
            })
        }
    }
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
        let payload: serde_json::Value = match parse_body(&body) {
            Ok(v) => v,
            Err(e) => {
                let _ = request.respond(json_error(400, serde_json::json!({ "error": e })));
                continue;
            }
        };

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
                let asked = asked_seed(&payload).and_then(|s| Ok((s, asked_player(&payload)?)));
                let (seed, player) = match asked {
                    Ok(pair) => pair,
                    Err(e) => {
                        let _ =
                            request.respond(json_error(400, serde_json::json!({ "error": e })));
                        continue;
                    }
                };
                let mut g = game.lock().unwrap();
                match new_game(&mut g, seed, player) {
                    (v, true) => json_response(v),
                    (v, false) => json_error(400, v),
                }
            }
            (Method::Post, "/api/advance") => {
                let months = match asked_months(&payload) {
                    Ok(m) => m as usize,
                    Err(e) => {
                        let _ = request
                            .respond(json_error(400, serde_json::json!({ "error": e })));
                        continue;
                    }
                };
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

    /// MEASUREMENT INSTRUMENT for the event log's filters, `#[ignore]`d and
    /// asserting nothing. Runs a real thirty-year world and prints what fraction
    /// of its headlines each filter can actually reach, with the commonest
    /// unreachable ones named so the next session can extend `classify` against
    /// the corpus rather than against a guess.
    ///
    /// `cargo test --release -p spheres-web classify_corpus -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn classify_corpus() {
        let mut counts: std::collections::BTreeMap<&'static str, usize> = Default::default();
        let mut other: std::collections::BTreeMap<String, usize> = Default::default();
        let mut total = 0usize;
        for seed in [0u64, 7, 42, 1990] {
            let mut w = world_1990(GameRules { seed, ..GameRules::default() });
            for _ in 0..360 {
                for h in tick_month(&mut w, &[]) {
                    total += 1;
                    let cat = classify(&h);
                    *counts.entry(cat).or_default() += 1;
                    if cat == "other" {
                        // Key on the shape, not the nation: the sim writes names
                        // into every headline and the raw strings never repeat.
                        let shape: String =
                            h.split_whitespace().skip(1).take(4).collect::<Vec<_>>().join(" ");
                        *other.entry(shape).or_default() += 1;
                    }
                }
            }
        }
        println!("\n=== {} headlines over four seeds x thirty years ===", total);
        for (k, v) in &counts {
            println!("{:>10}  {:>6}  {:>5.1}%", k, v, *v as f64 / total as f64 * 100.0);
        }
        println!("\n=== the twenty commonest shapes landing in \"other\" ===");
        let mut rows: Vec<(usize, String)> = other.into_iter().map(|(k, v)| (v, k)).collect();
        rows.sort_by(|a, b| b.0.cmp(&a.0));
        for (n, shape) in rows.iter().take(20) {
            println!("{:>6}  {}", n, shape);
        }
    }

    /// The event log's filter row could reach one dispatch in ten. Measured
    /// over four seeds and thirty years of real play, 11,285 of 12,552 headlines
    /// — 89.9% — fell through `classify` into "other", and the Economy filter
    /// matched EXACTLY NOTHING, so `renderLog`'s "only offer a filter the world
    /// has actually produced" rule meant its chip never appeared at all. A
    /// player filtering for Economy could not; a player filtering for War saw
    /// 85 of the 2,721 war dispatches that world contained.
    ///
    /// After: 2.7% in "other", and every one of those is the technology
    /// milestone this function's comment says is deliberately left there.
    ///
    /// TWO ARMS, and they check different things.
    ///
    /// The TABLE is the exact one, and it carries every headline that already
    /// matched before this was extended. That is what makes the additions
    /// provably additive: `classify` is an ordered if-chain, so a later keyword
    /// added to `war` can silently steal a headline `diplomacy` was reaching,
    /// and three rows below exist only to pin that — a defence pact honoured by
    /// entering a war, a guarantee torn up to invade, and an invasion repelled
    /// by a regime that totters are war, war and war, though each also carries a
    /// lower bucket's keyword.
    ///
    /// The CORPUS arm is the one that would have caught the original defect. A
    /// table can only test the headlines whoever wrote it thought of, and the
    /// reason `classify` rotted is that the sim grew a vocabulary nobody
    /// re-read it against.
    #[test]
    fn the_log_filters_reach_the_world_the_sim_writes() {
        // Real headline text, with the format arguments filled in as the sim
        // fills them. Left column is what the filter row must put it under.
        for (want, headline) in [
            // --- war, including the three that carry another bucket's keyword
            ("war", "WAR: Iraq invades Kuwait!"),
            ("war", "United States joins the war in defense of Kuwait."),
            ("war", "Iraq has annexed Kuwait."),
            ("war", "Iran capitulates to Iraq — reparations, disarmament, humiliation."),
            ("war", "Exhausted, Iran and Iraq sign a white peace."),
            ("war", "Iran and Iraq agree peace terms — reparations, no territory."),
            ("war", "Iraq sues for peace, ceding territory to Iran."),
            ("war", "Iraq escalates to rung 6 — standoff strike."),
            ("war", "United States steps back to rung 3 — arms to a proxy."),
            ("war", "Iraq publicly rules out going beyond rung 5 — deniable forces."),
            ("war", "Iraq cannot sustain a campaign it has no base for and falls back to rung 2."),
            ("war", "Iraq's magazines are empty. The tempo falls to rung 4."),
            ("war", "Kuwait quits the fight."),
            ("war", "United States takes a side against Iraq over Kuwait."),
            ("war", "The quarrel between India and Pakistan freezes over Kashmir."),
            ("war", "Iraq's objective is now to deny."),
            ("war", "Serbia can no longer defend its own ground."),
            // ...and the three that must not be stolen by a lower bucket.
            ("war", "Kuwait repels Iraq's invasion — the aggressor's regime totters."),
            ("war", "France honours its defence pact with Poland and enters the war."),
            ("war", "Iraq tears up its own guarantee to Kuwait to invade it."),
            // --- politics
            ("politics", "THE SOVIET UNION HAS DISSOLVED. Fifteen republics take up their own seats."),
            ("politics", "Revolution in Romania — the old regime falls."),
            ("politics", "COUP IN Chile: the junta removes the government."),
            ("politics", "India conducts nuclear tests. The world condemns; deterrence descends on the subcontinent."),
            ("politics", "Pakistan tests a nuclear device."),
            ("politics", "Israel is believed to have begun a weapons programme."),
            ("politics", "Poland votes: Solidarity takes office with 51% of the seats and no partners."),
            ("politics", "Poland sets a date for its first free elections."),
            ("politics", "Cuba does not hold elections."),
            ("politics", "The government of Moldova falls; the country goes to the polls."),
            ("politics", "Hungary goes to the country early."),
            ("politics", "Poland brings the Peasant Party into the government."),
            ("politics", "Chile moves against its own streets."),
            ("politics", "Brazil takes the gloves off."),
            ("politics", "France ends conscription."),
            ("politics", "Russia inherits the arsenal; Ukraine's warheads go back east under the Budapest assurances."),
            ("politics", "The JNA's divisions, and its arsenal, remain in Belgrade's hands."),
            // --- diplomacy
            ("diplomacy", "United States imposes sanctions on Iraq."),
            ("diplomacy", "Coalition sanctions slam Iraq."),
            ("diplomacy", "Sanctions on Iraq are lifted."),
            ("diplomacy", "France extends a diplomatic hand to Germany."),
            ("diplomacy", "France and Germany sign a mutual defence pact."),
            ("diplomacy", "France and Germany sign a trade agreement."),
            ("diplomacy", "France withdraws from its defence pact with Poland."),
            ("diplomacy", "Soviet Union buys the loyalty of Cuba."),
            ("diplomacy", "Turkey grants United States basing and overflight for the Gulf."),
            ("diplomacy", "Soviet Union commits $3bn a year in economic aid to Cuba."),
            ("diplomacy", "United States approves $2bn a year in arms sales to Israel."),
            ("diplomacy", "Soviet Union expands arms transfers to Syria to $4bn a year."),
            ("diplomacy", "Soviet Union raises its aid to Cuba to $5bn a year."),
            ("diplomacy", "Soviet Union cuts off arms to Somalia."),
            ("diplomacy", "A covert operation against Chile comes to nothing."),
            ("diplomacy", "Separatist fighters in Bosnia turn up with weapons nobody will account for."),
            // --- economy, the bucket that reached nothing at all
            ("economy", "Poland frees prices and takes the slump."),
            ("economy", "Argentina announces an austerity budget."),
            ("economy", "Brazil restructures its external debt; its creditors take the loss."),
            ("economy", "United Kingdom sells the state's industry."),
            ("economy", "China turns its economy outward."),
            ("economy", "Hungary opens up."),
            ("economy", "Argentina pegs its currency and imports somebody else's credibility."),
            ("economy", "A run of accidents wrecks Iraq's industrial plant. The inquiry finds nothing."),
            // --- and what is deliberately still uncategorised
            ("other", "United States is first to field integrated circuits."),
        ] {
            assert_eq!(
                classify(headline),
                want,
                "the event log would file this under {:?}: {}",
                classify(headline),
                headline
            );
        }

        // The corpus arm. A table only covers what its author thought of, and
        // `classify` fell behind precisely because nobody re-read it against the
        // sim's growing vocabulary. Four seeds and thirty years, ~12.5k
        // dispatches; measured 2.7% in "other", all of them "is first to field".
        let mut other = 0usize;
        let mut economy = 0usize;
        let mut total = 0usize;
        let mut stray: Vec<String> = vec![];
        for seed in [0u64, 7, 42, 1990] {
            let mut w = world_1990(GameRules { seed, ..GameRules::default() });
            for _ in 0..360 {
                for h in tick_month(&mut w, &[]) {
                    total += 1;
                    match classify(&h) {
                        "other" => {
                            other += 1;
                            if !h.contains("is first to field") && stray.len() < 10 {
                                stray.push(h.clone());
                            }
                        }
                        "economy" => economy += 1,
                        _ => {}
                    }
                }
            }
        }
        assert!(total > 5_000, "only {total} headlines — the corpus arm is not exercising anything");
        // The bar is 10% against a measured 2.7%, so it is a rot detector and
        // not a fit: it goes red if a WHOLE CLASS of headline stops being
        // reachable again, which is the defect this test exists for, and it
        // does not go red because the sim added one more phrasing.
        assert!(
            other * 10 < total,
            "{} of {} dispatches ({:.1}%) are unreachable from the filter row; \
             the first few that are not technology milestones: {:?}",
            other,
            total,
            other as f64 / total as f64 * 100.0,
            stray
        );
        // The Economy chip is only drawn when the world has produced an economy
        // headline, so a zero here is a filter the player can never even see.
        assert!(economy > 0, "the Economy filter still matches nothing in {total} dispatches");
    }

    /// The policy panel's ledger printed a sanctions drag the sim does not
    /// charge, and had done since the four sanction channels were converted from
    /// counting flags to weighing the coalition's share of world output.
    /// `dragsOf` in ui/index.html still read `sanctioned_by_count * 0.006`,
    /// which is the pre-conversion rule and the pre-conversion coefficient.
    ///
    /// These are not two estimates of one number. A COUNT is unbounded and
    /// blind to size — one signature from Luxembourg weighs what one from the
    /// United States weighs — while a SHARE is bounded by 1 and weighs output.
    /// So the browser could be an order of magnitude low against a coalition
    /// that mattered and an order of magnitude high against a crowd of small
    /// signatories, and its worst readings were outside the range the sim can
    /// produce at all.
    #[test]
    fn the_panel_prices_sanctions_the_way_the_sim_charges_them() {
        // The old browser rule and the sim's, on the same worlds, so the size of
        // the divergence is measured here rather than asserted from memory.
        const OLD_BROWSER_RULE: f64 = 0.006;
        let mut worst_ratio: f64 = 1.0;
        let mut worst: String = String::new();
        let mut compared = 0usize;

        for seed in [0u64, 7, 42] {
            let mut w = world_1990(GameRules { seed, ..GameRules::default() });
            for _ in 0..240 {
                tick_month(&mut w, &[]);
                for n in w.nations.iter().filter(|n| n.alive) {
                    let count = w.sanctioned_by_count(n.id);
                    if count == 0 {
                        continue;
                    }
                    compared += 1;
                    let served = spheres_sim::economy::growth_drag_of_sanctions(
                        w.sanction_weight(n.id),
                    );
                    // The payload must carry exactly what the sim charges.
                    let paid = nation_json(&w, n)["sanction_drag"].as_f64().expect("served");
                    assert_eq!(paid, served, "{:?}: the payload is not the sim's number", n.id);

                    let browser = count as f64 * OLD_BROWSER_RULE;
                    let ratio = if served > 0.0 { browser / served } else { f64::INFINITY };
                    if ratio.is_finite() && ratio > worst_ratio {
                        worst_ratio = ratio;
                        worst = format!(
                            "{:?} in {}: {} sanctioners, browser {:.4} vs sim {:.6} ({:.0}x)",
                            n.id,
                            w.date_str(),
                            count,
                            browser,
                            served,
                            ratio
                        );
                    }
                }
            }
        }
        assert!(compared > 500, "only {compared} sanctioned nation-months — nothing was tested");
        println!("{compared} sanctioned nation-months; worst divergence — {worst}");
        // The measurement, kept as the evidence: the two rules are not close.
        assert!(
            worst_ratio > 10.0,
            "the count rule and the share rule came within 10x over {compared} \
             nation-months, so this test is no longer measuring the defect it \
             was written for (worst seen: {worst})"
        );

        // And the page must READ the served number rather than recompute it.
        assert!(
            INDEX.contains("sanctions: n.sanction_drag"),
            "the policy panel no longer reads the served sanctions drag"
        );
        // Matched on the ASSIGNMENT, not on the bare expression: the comment
        // that replaced the old line quotes it verbatim, and a check that a
        // fix's own explanation trips is a check that invites its removal.
        assert!(
            !INDEX.contains("sanctions: n.sanctioned_by_count"),
            "the policy panel is still pricing sanctions by counting flags"
        );
        // The count itself is still served and still used — the map's mark and
        // the dossier line are honest uses of a count — so this must not be
        // "fixed" by deleting the field.
        assert!(INDEX.contains("sanctioned_by_count > 0"), "the ⊘ map mark reads the count");
    }

    /// "sustains a force of N" under the military slider was the browser's own
    /// arithmetic, and it kept one of `war::sustained_force`'s four factors.
    /// Missing: the technology multiplier (0.5x-4.0x), the equipment adequacy
    /// (0.55x-1.0x), and the flat floor a modern arsenal carries whatever the
    /// budget. Wrong on the FIRST screen with no player input.
    #[test]
    fn the_force_line_is_the_force_the_sim_sustains() {
        // What the page used to compute, kept so the error is measured here and
        // not remembered from a bug report.
        let old_browser_rule = |n: &Nation, share: f64| (n.gdp * share * 0.30).sqrt() * 8.0;

        let mut worst = 0.0f64;
        let mut worst_line = String::new();
        let mut checked = 0usize;
        let mut w = world_1990(GameRules::default());
        for _ in 0..120 {
            tick_month(&mut w, &[]);
            for n in w.nations.iter().filter(|n| n.alive) {
                let truth = spheres_sim::war::sustained_force(n, n.mil_spend_gdp);
                let guess = old_browser_rule(n, n.mil_spend_gdp);
                // Only forces big enough for the error to be about the formula
                // rather than about the last digit of a microstate's militia.
                // The seeder normalises every nation to 1.0 in January 1990, so
                // 1.0 is "an army at all" on this scale, not "a small one".
                if truth < 1.0 {
                    continue;
                }
                checked += 1;
                let err = (guess / truth - 1.0).abs();
                if err > worst {
                    worst = err;
                    worst_line =
                        format!("{:?}: page {:.1} against the sim's {:.1}", n.id, guess, truth);
                }
            }
        }
        assert!(checked > 1_000, "only {checked} nation-months");
        assert!(
            worst > 0.20,
            "the page's old formula came within 20% of the sim everywhere, so \
             this test no longer measures the defect it was written for ({worst_line})"
        );
        println!("worst error of the formula this replaced: {:.1}% — {}", worst * 100.0, worst_line);

        // THE CURVE IS THE SIM'S, sample for sample, at every position the
        // slider can select. This is the assertion that would have caught the
        // original defect, and it is exact rather than approximate because the
        // sampling step and the slider step are the same thousandth.
        let mut g = Game::new(1990, Some(NationId::USA));
        for _ in 0..24 {
            tick_month(&mut g.world, &[]);
        }
        let me = g.world.player.expect("seated");
        let pol = state_json(&g, None)["policy"].clone();
        let curve = pol["force_curve"].as_array().expect("a curve is served");
        assert_eq!(curve.len(), FORCE_CURVE_STEPS + 1);
        let n = g.world.nation(me);
        for (i, sample) in curve.iter().enumerate() {
            let share = i as f64 / FORCE_CURVE_STEPS as f64 * FORCE_CURVE_MAX;
            let want = round(spheres_sim::war::sustained_force(n, share), 3);
            assert_eq!(
                sample.as_f64().expect("finite"),
                want,
                "the curve disagrees with the sim at share {share:.3}"
            );
        }
        // Every sample is a real force, including at share zero where only the
        // arsenal's floor remains.
        assert!(curve.iter().all(|v| v.as_f64().is_some_and(|x| x.is_finite() && x >= 0.0)));
        assert_eq!(
            pol["sustained"].as_f64().expect("finite"),
            spheres_sim::war::sustained_force(n, n.mil_spend_gdp),
        );

        // The curve has to cover what the shipped slider can actually ask for,
        // or the page silently clamps. Read off the page rather than retyped, so
        // widening the slider without widening the curve goes red here.
        let hi: f64 = INDEX
            .split_once("sliderHtml(\"military\", \"Military spending\", m.mil_spend, 0, ")
            .expect("the page still offers a military slider")
            .1
            .split(')')
            .next()
            .expect("a closing paren")
            .trim()
            .parse()
            .expect("a numeric upper bound");
        assert!(
            hi <= FORCE_CURVE_MAX,
            "the military slider reaches {hi} but the force curve stops at {FORCE_CURVE_MAX}"
        );

        // And the page must READ the curve rather than recompute it.
        assert!(
            INDEX.contains("function sustainedForce(m, share)"),
            "the page no longer reads the served force curve"
        );
        assert!(
            !INDEX.contains("const force = Math.sqrt(m.gdp * p.military * 0.30) * 8"),
            "the page is still computing the sustained force itself"
        );
    }

    /// The research board called a fully funded project "stalled". `months_left`
    /// was the whole of what the payload said about a wait, and the browser
    /// rendered every one of its four `null`s as that one word — but a project
    /// whose banked points already cover its cost is not stopped, it is waiting
    /// on the calendar, because `tech::tick` will not field a technology before
    /// its `earliest_year` however much is banked against it.
    ///
    /// That is the reading that costs the player something. A government told
    /// its programme has stalled moves money to it, and there is nothing the
    /// money can do.
    #[test]
    fn a_funded_project_is_not_reported_as_stalled() {
        let mut funded = 0usize;
        let mut stalled = 0usize;
        let mut example = String::new();

        for name in ["United States", "Japan", "Sao Tome and Principe", "India"] {
            let id = NationId::parse(name).expect("on the roster");
            let mut g = Game::new(1990, Some(id));
            for _ in 0..180 {
                tick_month(&mut g.world, &[]);
                let r = research_json(&g.world, id);
                for d in r["domains"].as_array().expect("eight domains") {
                    let wait = d["wait"].as_str().expect("every domain says why");
                    // The payload's own consistency: a reason and a number are
                    // exclusive, and every reason is one the page can render.
                    assert_eq!(
                        d["months_left"].is_null(),
                        wait != "months",
                        "{name}: months_left and wait disagree — {d}"
                    );
                    assert!(
                        ["months", "none", "funded", "year", "beyond", "stalled"].contains(&wait),
                        "{name}: unrenderable wait reason {wait:?}"
                    );
                    match wait {
                        "funded" | "year" => {
                            funded += 1;
                            let banked = d["banked"].as_f64().expect("finite");
                            let cost = d["cost"].as_f64().expect("finite");
                            assert!(
                                banked >= cost,
                                "{name}: {d} claims to be funded on {banked} of {cost}"
                            );
                            if wait == "year" {
                                let y = d["fields_in"].as_i64().expect("a fielding year");
                                assert!(
                                    y > g.world.year as i64,
                                    "{name}: waiting for {y} in {}",
                                    g.world.year
                                );
                                if example.is_empty() {
                                    example = format!(
                                        "{name} {} — {} is {:.0}% funded and fields in {}",
                                        g.world.date_str(),
                                        d["project"]["name"],
                                        banked / cost * 100.0,
                                        y
                                    );
                                }
                            }
                        }
                        "stalled" => stalled += 1,
                        _ => {}
                    }
                }
            }
        }
        // The defect has to be reachable or this test is decoration. Every one
        // of these used to print the word "stalled".
        assert!(
            funded > 0,
            "no funded-but-unquoted project in 60 years of four very different \
             economies, so this test is not exercising the case it was written for"
        );
        println!("{funded} funded-but-unquoted domain-months (all said \"stalled\" before), \
                  {stalled} genuinely stalled; e.g. {example}");

        // And the page must say the four apart rather than collapsing them.
        assert!(INDEX.contains("function etaText(d, tilde)"), "the page lost its eta helper");
        for phrase in ["lands next month", "fields in", "beyond a century", "nothing is funding"] {
            assert!(INDEX.contains(phrase), "the board cannot say {phrase:?}");
        }
        assert!(
            !INDEX.contains(r#"months_left == null ? "stalled""#),
            "the research board still calls every missing number a stall"
        );
    }

    /// The interest-rate slider is a ONE-WAY DOOR and looked like every other
    /// slider. `politics::tick` runs the player's central bank on their behalf
    /// until they first issue a rate command; `WorldState::player_set_rate`
    /// latches on that command and the bank is skipped for the rest of the game.
    /// Nothing on the page said so, so a player could not tell whether the rate
    /// in front of them was their policy or the bank's, and could not know that
    /// touching it dismissed the bank permanently.
    ///
    /// The latch is pinned by tests in the sim and is NOT touched here. This
    /// covers only the half that was missing: saying out loud what it does.
    #[test]
    fn the_page_can_see_who_is_running_the_central_bank() {
        let mut g = Game::new(1990, Some(NationId::USA));
        // Before: unlatched, and it stays unlatched across an advance — the
        // player is idle, not governing.
        assert_eq!(state_json(&g, None)["player_set_rate"], serde_json::json!(false));
        for _ in 0..6 {
            tick_month(&mut g.world, &[]);
        }
        assert_eq!(
            state_json(&g, None)["player_set_rate"],
            serde_json::json!(false),
            "advancing time is not governing"
        );
        // The bank was actually running the seat, or there is nothing to say.
        let drifted = g.world.nation(NationId::USA).interest_rate;
        assert!(
            (drifted - 0.08).abs() > 1e-9,
            "the AI bank never moved the rate, so this test is not exercising the \
             state it describes (still {drifted})"
        );

        // After one rate command it latches, and the payload says so.
        let rate = g.world.nation(NationId::USA).interest_rate;
        apply_command(&mut g.world, &Command::SetInterestRate { nation: NationId::USA, rate })
            .expect("the player may always set their own rate");
        assert_eq!(
            state_json(&g, None)["player_set_rate"],
            serde_json::json!(true),
            "re-setting the rate one already had is still governing"
        );
        // And it never goes back.
        for _ in 0..12 {
            tick_month(&mut g.world, &[]);
        }
        assert_eq!(state_json(&g, None)["player_set_rate"], serde_json::json!(true));

        // The page must read it and say both halves out loud.
        assert!(INDEX.contains("function rateSeat()"), "the page lost its rate-seat line");
        assert!(INDEX.contains("S.player_set_rate"), "the page does not read the latch");
        assert!(
            INDEX.contains(r#"sliderHtml("rate", "Interest rate", m.rate, 0, 0.40, rateSeat())"#),
            "the rate slider no longer says who is holding it"
        );
        assert!(INDEX.contains("the central bank is setting this for you"));
        assert!(INDEX.contains("takes the wheel for good"), "the door is one-way; say so");
    }

    /// A headline was read as being about every nation whose NAME'S LETTERS it
    /// contained. Across the roster that is one pair — "Romania" contains
    /// "Oman" — and one pair was enough to make an Omani player's personal news
    /// feed somebody else's.
    ///
    /// Measured on the live server, governing Oman on seed 1990 for 300 months:
    /// the "You" filter held SIXTEEN dispatches, of which FIFTEEN were about
    /// Romania — its elections, its street protests — and one was about Oman.
    /// The same tags drive the chart's per-nation event marks, and the same
    /// match in `is_major` stopped an Omani player's advance to tell them about
    /// a Romanian election.
    #[test]
    fn a_headline_is_only_about_the_nations_it_names() {
        // The pair this exists for, in both directions.
        assert_eq!(
            mentioned("Romania votes: National Salvation Front takes office."),
            vec![NationId::Romania],
            "Romania is not news about Oman"
        );
        assert_eq!(mentioned("Romania moves against its own streets."), vec![NationId::Romania]);
        assert!(mentioned("Oman opens up.").contains(&NationId::Oman));
        assert!(!mentioned("Oman opens up.").contains(&NationId::Romania));
        assert!(!is_major("Romania moves against its own streets.", Some(NationId::Oman)));
        assert!(is_major("Oman moves against its own streets.", Some(NationId::Oman)));

        // Every real mention still lands: possessives, punctuation, capitals,
        // multi-word names, and the hyphenated names on the roster.
        for (h, want) in [
            ("Iraq's magazines are empty. The tempo falls to rung 4.", NationId::Iraq),
            ("WAR: Iraq invades Kuwait!", NationId::Kuwait),
            ("United States is first to field integrated circuits.", NationId::USA),
            ("THE SOVIET UNION HAS DISSOLVED.", NationId::USSR),
            // The roster's one hyphenated name, and the boundary rule has to
            // let a hyphen close a name the way a space or a full stop does.
            ("Congo-Brazzaville opens up.", NationId::Congo),
            ("Equatorial Guinea opens up.", NationId::EquatorialGuinea),
            ("Oman pegs its currency and imports somebody else's credibility.", NationId::Oman),
        ] {
            assert!(mentioned(h).contains(&want), "{want:?} is not read out of {h:?}");
            assert!(is_major(h, Some(want)), "{want:?} is not told about {h:?}");
        }

        // THE GENERAL CLAIM, not just the one pair: no nation is ever read out
        // of a headline that names only some other nation. This is what stops a
        // future roster addition reopening the defect silently — add "Congo"
        // beside "Congo-Brazzaville" and this goes red.
        for a in all_nations() {
            let h = format!("{} opens up.", a.name());
            let read = mentioned(&h);
            assert_eq!(
                read,
                vec![*a],
                "{:?} is read as being about {:?} as well",
                a,
                read.iter().filter(|x| *x != a).collect::<Vec<_>>()
            );
        }

        // And the boundary rule itself, so its edges are pinned rather than
        // inferred from the cases above.
        assert!(names_nation("romania votes", "romania"));
        assert!(!names_nation("romania votes", "oman"));
        assert!(names_nation("oman votes", "oman"));
        assert!(names_nation("it was oman", "oman"));
        assert!(names_nation("oman's fleet", "oman"));
        assert!(!names_nation("omani forces", "oman"), "an adjective is not the nation");
        assert!(!names_nation("", "oman"));
        assert!(!names_nation("oman", ""), "an empty name matches nothing and must not hang");
    }

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

    /// Outliving your own state is a legitimate ending — you watch the rest of
    /// the century from the stands. It was not survivable: the interrupt that
    /// announces it fired again on the first month of EVERY later advance, so
    /// the clock moved one month per request forever however many were asked
    /// for. Measured against the live server as the Soviet Union on seed 1990:
    /// dissolution in Sep 1993, and then advance after advance asking for 120
    /// months delivered Sep->Oct, Oct->Nov, Nov->Dec ... each repeating
    /// "Soviet Union no longer exists." Reaching 2020 from there is 315 clicks.
    ///
    /// One seed and one nation is enough here, and iron rule 7 says why: this
    /// is an INVARIANT, not a statistic. The latch either exists or it does
    /// not, and a single world where the player dies exercises it completely —
    /// more seeds would buy power against a regression that cannot be
    /// intermittent. The Soviet Union on seed 1990 is chosen only because it is
    /// the shortest path to a dead player.
    #[test]
    fn the_clock_still_moves_after_your_nation_is_gone() {
        let mut g = Game::new(1990, Some(NationId::USSR));

        // Run until the Soviet Union goes, and check the news is delivered once
        // — on the advance it happens.
        let mut told = 0;
        let mut guard = 0;
        loop {
            let (_, why) = g.advance(12, vec![]);
            if why.as_deref().is_some_and(|w| w.contains("no longer exists")) {
                told += 1;
                break;
            }
            guard += 1;
            assert!(guard < 60, "the Soviet Union outlived sixty years on seed 1990");
        }
        assert_eq!(told, 1, "the dissolution must be announced");
        assert!(
            !g.world.nation_opt(NationId::USSR).is_some_and(|n| n.alive),
            "precondition: the player's nation is gone"
        );

        // From here the player is a spectator, and a spectator can still watch.
        // Twelve asked for is twelve delivered — unless some OTHER major event
        // interrupts, which is the ordinary behaviour and not this defect, so
        // the bar is that the clock moves by more than the single month the
        // repeated interrupt used to allow.
        let before = month_index(g.world.year, g.world.month);
        let (_, why) = g.advance(12, vec![]);
        let moved = month_index(g.world.year, g.world.month) - before;
        assert!(
            why.as_deref().is_none_or(|w| !w.contains("no longer exists")),
            "the death must not be re-announced on every later advance: {:?}",
            why
        );
        assert!(
            moved > 1,
            "asked for 12 months after the player died and got {}",
            moved
        );

        // Ten more advances, and the death is never the reason any of them
        // stops. What DOES stop them is the ordinary major-event interrupt —
        // measured here as revolutions in Tajikistan and Georgia and half a
        // dozen escalations across 1994 — which is that interrupt working, not
        // this defect. The bar is therefore the shape of the defect and not the
        // pace of the world: before the fix ten advances delivered exactly ten
        // months, one per call, and no world event could change that number.
        let before = month_index(g.world.year, g.world.month);
        for _ in 0..10 {
            let (_, why) = g.advance(120, vec![]);
            assert!(
                why.as_deref().is_none_or(|w| !w.contains("no longer exists")),
                "a spectator was told again that their nation is gone: {:?}",
                why
            );
        }
        let moved = month_index(g.world.year, g.world.month) - before;
        assert!(
            moved > 10,
            "ten advances after the player died moved {} months — one per call              is the signature of the interrupt firing every time",
            moved
        );
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

    /// Starting a game is the one action with no screen behind it to fall back
    /// on, so it is the one that must never fail in silence — and it did. `api`
    /// called `r.json()` without reading the status, so a refusal arrived as an
    /// ordinary object nothing looked at, and `#startBtn.onclick` had no
    /// try/catch, so a server that was not there left the handler's promise
    /// rejected and the setup screen byte-for-byte as it was: same button, same
    /// caption, no message. Measured before the fix, with the server stopped
    /// between picking Poland and pressing GOVERN: nothing on screen changed and
    /// the console carried `Uncaught (in promise) TypeError: Failed to fetch at
    /// api ... at $.onclick`.
    ///
    /// A substring check against the served HTML, for the reason
    /// `every_nation_on_the_board_has_somewhere_to_be_drawn` gives: this file
    /// ships by `include_str!` and has no build step, so the thing to assert on
    /// is the thing that reaches the browser.
    #[test]
    fn a_refused_start_says_so_instead_of_freezing_the_setup_screen() {
        // `api` must read the status before it reads the body, and must carry
        // the server's own sentence out when there is one.
        assert!(INDEX.contains("if (!r.ok)"), "api() must check the response status");
        assert!(
            INDEX.contains("throw new Error((data && data.error)"),
            "a refusal must surface the server's own message"
        );
        // A dead server is a caught failure, not an unhandled rejection.
        assert!(
            INDEX.contains("The SPHERES server is not answering"),
            "an unreachable server must have a sentence of its own"
        );
        // The hand that presses START must catch, say, and give the button back.
        let start = INDEX
            .split_once("$(\"#startBtn\").onclick")
            .expect("the setup screen still has a start button")
            .1
            .split_once("\n};")
            .expect("the start handler is still brace-terminated")
            .0;
        assert!(start.contains("catch"), "the start handler must catch");
        assert!(start.contains("banner("), "and must say what went wrong");
        assert!(
            start.contains("b.disabled = false"),
            "and must hand the button back so the player can try again"
        );
        // And the route it calls answers a refusal with a status worth reading —
        // the half of this that lives in Rust. Poland is on the board; the
        // successor states are not.
        let mut g = Game::new(1990, None);
        let (_, ok) = new_game(&mut g, 1990, Some(NationId::Poland));
        assert!(ok);
        let refused = spheres_sim::world::successor_nations()[0];
        let (v, ok) = new_game(&mut g, 1990, Some(refused));
        assert!(!ok);
        assert!(
            v["error"].is_string(),
            "the browser reads `error` off the refusal; it must be there"
        );
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

    /// The map's click handler must resolve its pick from the POINTERDOWN
    /// target, never from the click's own.
    ///
    /// `#pane-map` calls `setPointerCapture` on pointerdown so a fast drag that
    /// leaves the pane keeps panning. Pointer capture also retargets the
    /// subsequent `click` at the capture element, so `e.target` in the click
    /// handler is the pane `<div>` and `e.target.closest(".nodeg")` is always
    /// null. Measured in the browser: pointerdown landed on a `<circle>` inside
    /// `.nodeg[data-id=Brazil]`, and the click that followed it targeted
    /// `DIV#pane-map`. The result was that clicking a nation on the map opened
    /// nothing — the control the page advertises in three separate places (the
    /// opening banner, the map legend, and the keyboard card's "click a
    /// country").
    ///
    /// The repair records the press's target and reads the pick off that, so
    /// this test pins the three `closest` calls to the recorded target rather
    /// than to `e.target`. Reverting any one of them silently kills that pick
    /// path again and nothing else in the suite would notice.
    #[test]
    fn the_map_resolves_a_click_from_the_press_not_the_click() {
        assert!(
            INDEX.contains("downTarget = e.target;"),
            "the map's pointerdown must record what the press landed on — pointer \
             capture retargets the click and the pick cannot be read off it"
        );
        assert!(
            INDEX.contains("const t = downTarget && downTarget.isConnected ? downTarget : e.target;"),
            "the map's click handler must prefer the recorded press target"
        );
        for pick in ["t.closest(\".nodeg\")", "t.closest(\".rchip\")", "t.closest(\"#dhit\")"] {
            assert!(
                INDEX.contains(pick),
                "the map's click handler no longer resolves {pick} from the press \
                 target; with pointer capture set, e.target is #pane-map and this \
                 pick can never match"
            );
        }
        // And the capture that makes all of the above necessary is still there:
        // if it ever goes away this test is measuring nothing, and the comment
        // above becomes a lie about why the indirection exists.
        assert!(
            INDEX.contains("pane.setPointerCapture(e.pointerId)"),
            "the pan's pointer capture is gone — re-read whether the press-target \
             indirection is still the right shape before deleting this test"
        );
    }

    /// The shortcut gate must ask whether the key is being CONSUMED as text,
    /// not merely whether the target is an `<input>`.
    ///
    /// The policy sliders are `<input type="range">`. A range consumes the
    /// arrows, Home/End and PageUp/PageDown and nothing else — no letter and no
    /// digit, and the page binds none of what it does consume. But the old
    /// `typing()` returned true for every `<input>`, so the global keydown
    /// handler returned early and EVERY shortcut died while a slider had focus.
    ///
    /// The moment that matters: `noteQueued()` writes "… · R to revert" into
    /// the header only once an order is queued, and the only way to queue one
    /// with the keyboard-free hand is to move a slider — which leaves that
    /// slider holding focus. Measured in the browser: after clicking the tax
    /// slider, `document.activeElement` was `INPUT[range]`, the header read
    /// "1 order takes effect next month · R to revert", and pressing R changed
    /// nothing. Escape (which blurs) then R worked.
    ///
    /// Escape deliberately keeps the WIDE test — `focused()` — so that
    /// behaviour is unchanged; only the shortcut gate narrowed.
    #[test]
    fn a_focused_slider_does_not_kill_every_shortcut() {
        assert!(
            INDEX.contains("const TYPELESS_INPUT = new Set(["),
            "the shortcut gate no longer distinguishes inputs that consume text \
             from inputs that do not"
        );
        assert!(
            INDEX.contains("\"range\", \"checkbox\", \"radio\","),
            "type=range must stay in the set of inputs that are NOT typing — the \
             policy sliders are ranges and the shortcuts must survive them"
        );
        assert!(
            INDEX.contains("return t.tagName !== \"INPUT\" || !TYPELESS_INPUT.has((t.type || \"text\").toLowerCase());"),
            "typing() must consult the input's type"
        );
        // Escape keeps the wide test. If this flips to typing(), a focused
        // slider stops being blurrable by keyboard and that is a regression of
        // its own.
        assert!(
            INDEX.contains("if (focused(e)) { e.target.blur(); return; }"),
            "Escape must still blur any focused control, range included"
        );
        // ...and the shortcut gate must be the narrow one.
        assert!(
            INDEX.contains("if (typing(e) || e.ctrlKey || e.metaKey || e.altKey) return;"),
            "the shortcut gate must ask typing(), not focused()"
        );
        // The advertisement this defect was breaking.
        assert!(
            INDEX.contains("R to revert"),
            "the header no longer offers R; re-read whether this test still \
             describes a real promise"
        );
    }

    /// The war decoration must never take a pointer event.
    ///
    /// Each belligerent gets a `<circle r="34 * PX">` filled with `url(#glow)` —
    /// 81.6 world units of gradient — plus, for a war with no front to draw, a
    /// dashed line between the two. Both are scenery: nothing reads them and
    /// there is nothing to click. But a gradient fill is still a fill, so under
    /// SVG's default `pointer-events: visiblePainted` the whole disc is a hit
    /// target regardless of its alpha, and it is painted above the district mesh
    /// and the ocean.
    ///
    /// Measured over the Gulf in Oct 1990: of 441 points sampled inside one
    /// disc, 134 (30.4%) hit the disc itself rather than anything the map can
    /// open, and fourteen nation anchors — Iraq, Kuwait, Saudi Arabia, Iran,
    /// Israel, Turkey, Syria, Jordan, Lebanon, UAE, Qatar, Oman, Bahrain,
    /// Cyprus — sat under the two discs. A click at CSS (678, 273), squarely on
    /// Egypt's painted territory, opened nothing.
    ///
    /// Asserted as the wrapper because the wrapper is the fix: adding a mark to
    /// this layer must inherit the rule rather than have to remember it.
    #[test]
    fn the_war_decoration_takes_no_pointer_events() {
        assert!(
            INDEX.contains("s += `<g id=\"warg\" pointer-events=\"none\">`;"),
            "the war glow and its dashed abstraction must stay inside a \
             pointer-events:none wrapper — they are scenery painted over the \
             district mesh, and a gradient fill is a hit target whatever its alpha"
        );
        // The layer it wraps, so a wrapper left behind around nothing cannot
        // pass this test while the marks have been moved back out of it.
        let at = INDEX
            .find("s += `<g id=\"warg\" pointer-events=\"none\">`;")
            .expect("the war layer's wrapper is gone");
        let close = INDEX[at..]
            .find("s += `</g>`;")
            .expect("the war layer's wrapper is never closed");
        let inside = &INDEX[at..at + close];
        assert!(
            inside.contains("fill=\"url(#glow)\""),
            "the war glow has left the pointer-events:none wrapper"
        );
        assert!(
            inside.contains("stroke-dasharray="),
            "the dashed war abstraction has left the pointer-events:none wrapper"
        );
    }

    /// The technology graph has the same capture as the map and needs the same
    /// repair: `#techViewport` sets pointer capture to keep a pan alive, and the
    /// click that follows is retargeted at the viewport, so
    /// `e.target.closest("g.node")` is always null and clicking a technology
    /// does nothing.
    ///
    /// This one is louder than the map's, because the graph advertises the
    /// control on the node itself: the hover tooltip's last line reads
    /// "click — routes & research". Measured in the browser: pointerdown landed
    /// on a `<rect>` inside `g.node[data-i="2"]` (GPU Deep Learning), the click
    /// targeted `DIV#techViewport`, and `#techDock` stayed `display:none` while
    /// the tooltip was still promising the click would do something.
    #[test]
    fn the_tech_graph_resolves_a_click_from_the_press_not_the_click() {
        assert!(
            INDEX.contains("st.target = e.target;"),
            "the tech viewport's pointerdown must record what the press landed on"
        );
        assert!(
            INDEX.contains("const t = st.target && st.target.isConnected ? st.target : e.target;"),
            "the tech viewport's click handler must prefer the recorded press target"
        );
        for pick in ["t.closest(\"g.stub\")", "t.closest(\"g.node\")"] {
            assert!(
                INDEX.contains(pick),
                "the tech viewport's click handler no longer resolves {pick} from \
                 the press target; with pointer capture set, e.target is \
                 #techViewport and this pick can never match"
            );
        }
        assert!(
            INDEX.contains("vp.setPointerCapture(e.pointerId)"),
            "the tech pan's pointer capture is gone — re-read whether the \
             press-target indirection is still the right shape before deleting \
             this test"
        );
        // The promise the defect was breaking. If this line ever goes away the
        // control is no longer advertised, and the argument above needs redoing
        // rather than quietly weakening.
        assert!(
            INDEX.contains("click &mdash; routes &amp; research")
                || INDEX.contains("click — routes &amp; research"),
            "the node tooltip no longer offers the click this test protects"
        );
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

    /// The policy ledger printed a debt still falling out of a country that has
    /// none. Measured in the browser as Poland on seed 5 in January 2015: the
    /// nation card read "Debt 0% of GDP" and the ledger directly beside it read
    /// "Debt drift -2.0pp/yr". Not a corner case — 41 of that world's nations
    /// sat at exactly zero debt, because the sim floors the ratio there.
    #[test]
    fn a_nation_with_no_debt_is_not_shown_paying_it_down() {
        // The page applies the floor.
        assert!(
            INDEX.contains("Math.max(led.deficit - m.debt * (expected + m.inflation), -m.debt)"),
            "the drift line no longer carries the sim's floor"
        );

        // And the sim really does floor it, which is the only thing that makes
        // the line above true rather than merely tidy. Run a world and assert
        // the invariant the panel is now allowed to rely on — for every nation,
        // every month, not for a sampled few (iron rule 7: this is an invariant,
        // so one world exercises it completely).
        let mut g = Game::new(5, Some(NationId::Poland));
        let mut ever_zero = 0usize;
        for _ in 0..300 {
            tick_month(&mut g.world, &[]);
            for n in g.world.nations.iter().filter(|n| n.alive) {
                assert!(
                    n.debt_gdp >= 0.0,
                    "{:?} holds negative debt {}",
                    n.id,
                    n.debt_gdp
                );
                if n.debt_gdp == 0.0 {
                    ever_zero += 1;
                }
            }
        }
        assert!(
            ever_zero > 0,
            "no nation reached the floor in 300 months, so this says nothing"
        );
    }

    /// "same seed, same history" is printed on the setup screen beside the box,
    /// and a seed the server could not use was quietly replaced with 1990
    /// instead of being refused. Measured on the live server by fingerprinting
    /// the state six months into each run: {"seed":"12345"}, {"seed":-1} and
    /// {"seed":3.5} all produced 6B60D853FEC58666, byte-identical to
    /// {"seed":1990}, while {"seed":12345} produced 3267FEB6F4A4A872. Three
    /// requests asking for three different worlds, all given a fourth.
    #[test]
    fn a_seed_the_server_cannot_use_is_refused_not_replaced() {
        // Not asking is still the default — this is how the server boots.
        assert_eq!(asked_seed(&serde_json::json!({})), Ok(DEFAULT_SEED));
        assert_eq!(asked_seed(&serde_json::json!({ "seed": null })), Ok(DEFAULT_SEED));
        assert_eq!(
            DEFAULT_SEED,
            spheres_sim::world::GameRules::default().seed,
            "the route's default must be the sim's default"
        );

        // Every seed a player can actually ask for still arrives intact —
        // including 0, which the browser's own `|| 1990` used to swallow.
        for s in [0u64, 1, 1990, 12345, u64::MAX] {
            assert_eq!(asked_seed(&serde_json::json!({ "seed": s })), Ok(s));
        }

        // And the three measured substitutions.
        for bad in [
            serde_json::json!({ "seed": "12345" }),
            serde_json::json!({ "seed": -1 }),
            serde_json::json!({ "seed": 3.5 }),
            serde_json::json!({ "seed": [1990] }),
        ] {
            let e = asked_seed(&bad).expect_err("must be refused");
            assert!(e.contains("is not a seed"), "unhelpful refusal: {e}");
        }

        // The browser half. Its box used to be `parseInt(v, 10) || 1990`, which
        // read "12abc" as 12, replaced everything else with 1990, and made 0
        // unreachable because 0 is falsy.
        assert!(
            !INDEX.contains("parseInt($(\"#seed\").value, 10) || 1990"),
            "the seed box still substitutes a seed the player did not ask for"
        );
        assert!(INDEX.contains("function seedFromBox()"));
        assert!(INDEX.contains("if (!/^\\d+$/.test(raw)) return null;"));
        assert!(INDEX.contains("Number.isSafeInteger(n) ? n : null"));
        assert!(
            INDEX.contains("is not a seed — a seed is a whole number"),
            "a box that refuses must say why"
        );
    }

    /// Asking to govern a nation the roster does not know used to start a game
    /// with nobody in it. Measured on the live server: POST /api/new
    /// {"seed":7,"nation":"Polnad"} answered 200, and GET /api/state came back
    /// with `player` empty — a board with no dashboard, no research, no orders
    /// and no explanation. "Atlantis" did the same. A typed nation name is the
    /// one field on the setup screen a player can get wrong, and it was the one
    /// field that failed silently.
    #[test]
    fn a_nation_the_roster_does_not_know_is_refused_not_ignored() {
        // Not asking is still a legitimate answer — the server boots this way.
        assert_eq!(asked_player(&serde_json::json!({})), Ok(None));
        assert_eq!(asked_player(&serde_json::json!({ "nation": null })), Ok(None));
        assert_eq!(
            asked_player(&serde_json::json!({ "seed": 7 })),
            Ok(None),
            "a body with no nation key is an observer"
        );

        // Everything NationId::parse accepts still gets through: name, code,
        // alias, any case, surrounding space.
        for asked in ["Poland", "POL", "pol", "  Poland  ", "united states", "usa"] {
            assert!(
                matches!(asked_player(&serde_json::json!({ "nation": asked })), Ok(Some(_))),
                "{asked} must still be playable"
            );
        }

        // And what used to be silence.
        for asked in ["Polnad", "Atlantis", "", "   "] {
            let e = asked_player(&serde_json::json!({ "nation": asked }))
                .expect_err("must be refused");
            assert!(e.contains("no nation called"), "unhelpful refusal: {e}");
        }
        // A `nation` that is not even a string is a refusal too, not an
        // observer — as_str() used to swallow it.
        assert!(asked_player(&serde_json::json!({ "nation": 42 })).is_err());
        assert!(asked_player(&serde_json::json!({ "nation": ["Poland"] })).is_err());
    }

    /// A request body that does not parse used to be indistinguishable from no
    /// body at all, so every route read its own default out of nothing and
    /// answered 200. Measured on the live server with a body cut off
    /// mid-object: /api/command answered `errors: []` having read no commands,
    /// /api/advance moved one month against a body asking for sixty, and
    /// /api/new threw away the game in progress and started a fresh 1990 world.
    #[test]
    fn a_body_that_is_not_json_is_a_failed_request_not_an_empty_one() {
        // No body is still no arguments — /api/save posts empty on purpose.
        assert_eq!(parse_body(""), Ok(serde_json::Value::Null));
        assert_eq!(parse_body("   \n"), Ok(serde_json::Value::Null));

        // Real bodies still arrive intact.
        assert_eq!(parse_body("{}"), Ok(serde_json::json!({})));
        assert_eq!(
            parse_body(r#"{"months":60}"#),
            Ok(serde_json::json!({ "months": 60 }))
        );

        // And the three shapes that used to be silently read as `{}`.
        for bad in [
            r#"{"commands": [{"kind":"war","target":"Iraq"}"#, // truncated
            "this is not json",
            "{",
        ] {
            let e = parse_body(bad).expect_err("must be refused");
            assert!(e.contains("not JSON"), "unhelpful refusal: {e}");
        }

        // The point of the refusal: a route must never see a default it can
        // act on. `months` is the one that moved the clock the wrong distance.
        let refused = parse_body(r#"{"months": 60"#);
        assert!(refused.is_err());
        // What the old code handed the route instead, and what it did with it.
        let old = serde_json::Value::Null;
        assert_eq!(old.get("months").and_then(|m| m.as_u64()).unwrap_or(1), 1);
    }

    /// Ten nations on the picker read "$0bn · 0m", and a player who chose one
    /// governed a country whose every headline figure was zero: measured in the
    /// browser as Sao Tome and Principe, the header read "GDP $0bn" and the
    /// dashboard "GDP $0bn / Population 0m" for a transcribed $120m economy of
    /// 119,000 people. Eighteen more nations read "0m" beside a correct GDP —
    /// Luxembourg was "$13bn · 0m".
    ///
    /// The roster is the reason this cannot be one unit: it spans six orders of
    /// magnitude, and a formatter with a fixed unit is wrong at one end of it
    /// whichever end you pick. This test therefore asserts on the DATA, not on
    /// a list of nations: for every nation seated in 1990, the figures the
    /// picker card is built from must be ones the formatters can state without
    /// rounding to nothing — and the served page must carry formatters that
    /// change unit rather than lose the figure.
    #[test]
    fn no_nation_on_the_board_is_shown_as_nothing() {
        // The formatters the page ships. A substring check, for the reason
        // every_nation_on_the_board_has_somewhere_to_be_drawn gives.
        for needle in [
            // money and its flow twin drop to millions below a billion
            "return \"$\" + (v * 1000).toFixed(a >= 0.01 ? 0 : 1) + \"m\";",
            // population drops to thousands below a million
            "return Math.round(m * 1000) + \"k\";",
            // and the picker and the dashboards go through them
            "fmt.pop(n.population)",
            "statRow(\"Population\", fmt.pop(m.population))",
            "statRow(\"Population\", fmt.pop(n.population))",
            // the GDP chart's axis is the same ladder, which is why both of a
            // microstate's axis labels used to read "0bn"
            "fmtY: (v) => fmt.money(v).slice(1)",
        ] {
            assert!(INDEX.contains(needle), "the page no longer carries: {needle}");
        }
        assert!(
            !INDEX.contains("population.toFixed(0) + \"m\""),
            "a population is still being printed straight to the nearest million"
        );

        // And the world the picker is built from. `renderPick` reads /api/state,
        // so these are exactly the numbers it formats.
        let g = Game::new(1990, None);
        let s = state_json(&g, None);
        let mut zero_money = vec![];
        let mut zero_pop = vec![];
        for n in s["nations"].as_array().expect("a roster") {
            let name = n["name"].as_str().unwrap_or("?").to_string();
            let gdp = n["gdp"].as_f64().expect("gdp");
            let pop = n["population"].as_f64().expect("population");
            assert!(gdp > 0.0 && pop > 0.0, "{name} is seated with nothing");
            // What the OLD formatters did, kept here as the thing being
            // guarded against rather than as a description of the fix.
            if gdp.round() == 0.0 {
                zero_money.push(name.clone());
            }
            if pop.round() == 0.0 {
                zero_pop.push(name);
            }
        }
        assert!(
            !zero_money.is_empty() && !zero_pop.is_empty(),
            "this test is only meaningful while the roster still holds nations \
             a whole-billion formatter would erase; it holds {} and {}",
            zero_money.len(),
            zero_pop.len()
        );
        // The count is the measurement this was found by, recorded rather than
        // asserted on: pinning it would make adding a small nation a red test.
        println!(
            "{} nations would read $0bn and {} would read 0m under a fixed unit",
            zero_money.len(),
            zero_pop.len()
        );
    }

    /// The research board's "N mo" is a projection that holds this month's rate
    /// constant for the whole wait. That is fair over a few years; over a
    /// century it is a fiction, and the board printed the fiction to the month.
    /// Measured on the live server: Equatorial Guinea in January 1991 was shown
    /// "626193 mo" against its Aerospace project — fifty-two thousand years,
    /// stated to the month — and Sao Tome "248598 mo" against its own.
    /// microstate-04 reported ten digits of the same thing.
    ///
    /// The bar is the whole roster, because this is an invariant: no nation, in
    /// any state the sim can put it in, may be handed a schedule longer than the
    /// span this server will talk about at once.
    #[test]
    fn the_research_board_never_quotes_a_schedule_in_millennia() {
        // Two of the smallest economies on the board, which is where the rate
        // is small enough for the division to run away.
        for name in ["Sao Tome and Principe", "Equatorial Guinea"] {
            let id = NationId::parse(name).expect("on the roster");
            let mut g = Game::new(1990, Some(id));
            let mut ever_quoted = false;
            for _ in 0..36 {
                tick_month(&mut g.world, &[]);
                let r = research_json(&g.world, id);
                for d in r["domains"].as_array().expect("eight domains") {
                    match d["months_left"].as_i64() {
                        None => {}
                        Some(m) => {
                            ever_quoted = true;
                            assert!(
                                (1..=1200).contains(&m),
                                "{} was quoted {} months ({} years) for {}",
                                name,
                                m,
                                m / 12,
                                d["name"]
                            );
                        }
                    }
                }
            }
            // And the guard has not simply blanked the board: somewhere in three
            // years at least one domain still carries a number a player can use.
            assert!(ever_quoted, "{} was never quoted any schedule at all", name);
        }

        // The same guard must not touch a nation that can actually finish
        // things: a superpower's board keeps its numbers.
        let usa = NationId::parse("United States").expect("on the roster");
        let mut g = Game::new(1990, Some(usa));
        for _ in 0..12 {
            tick_month(&mut g.world, &[]);
        }
        let r = research_json(&g.world, usa);
        let quoted = r["domains"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|d| d["months_left"].as_i64().is_some())
            .count();
        assert!(quoted >= 4, "the United States was quoted only {} schedules", quoted);
    }

    /// A microstate's chart must be a chart, not a staircase. The history
    /// payload rounded GDP to two decimal places of a billion — $10m — which is
    /// a sixty-thousandth of the United States and a sixth of Sao Tome and
    /// Principe. Measured on the live server before the fix: Sao Tome's
    /// ninety-five-month GDP series came back holding exactly TWO distinct
    /// values, 0.11 and 0.12, and its military series four, for a nation whose
    /// output moved continuously the whole time. Indexed to 100 at the start —
    /// which is how the comparison chart draws it — that is a two-step
    /// staircase standing in for a decade of history.
    ///
    /// The bar is stated as a share of the series rather than as a count of
    /// values, so it means the same thing whatever the run length: a series
    /// that resolves its own movement has many more levels than it has steps.
    #[test]
    fn a_microstates_history_is_not_flattened_into_steps() {
        let small = NationId::parse("Sao Tome and Principe").expect("on the roster");
        let mut g = Game::new(1990, Some(small));
        for _ in 0..95 {
            tick_month(&mut g.world, &[]);
            g.snapshot();
        }
        let h = history_json(&g, Some(small));
        let series = &h["nations"][format!("{:?}", small)];

        // Precondition: the nation is alive and its output actually moved, so a
        // flat series would be the payload's fault and not the world's.
        let live = g.world.nation(small);
        assert!(live.alive);
        // 0.12 is its transcribed 1990 GDP. The movement is small in level —
        // about a sixth of ONE step of the old $10m grid — which is exactly why
        // the old payload could not show it at all.
        let moved = (live.gdp - 0.12).abs() / 0.12;
        assert!(moved > 0.01, "the world must have moved it: {}", live.gdp);

        for metric in ["gdp", "mil"] {
            let vals: Vec<f64> = series[metric]
                .as_array()
                .unwrap_or_else(|| panic!("{} series missing", metric))
                .iter()
                .filter_map(|v| v.as_f64())
                .collect();
            assert!(vals.len() > 90, "{}: {} points", metric, vals.len());
            let mut sorted = vals.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
            sorted.dedup();
            assert!(
                sorted.len() * 4 > vals.len(),
                "{} came back as {} distinct values across {} months — the \
                 series has been rounded away, not compressed",
                metric,
                sorted.len(),
                vals.len()
            );
        }

        // And the same precision is still there for a superpower, whose
        // figures are six orders of magnitude larger in the same response.
        let usa_id = NationId::parse("United States").expect("on the roster");
        let h = history_json(&g, Some(usa_id));
        let usa: Vec<f64> = h["nations"][format!("{:?}", usa_id)]["gdp"]
            .as_array()
            .expect("the United States is in the history")
            .iter()
            .filter_map(|v| v.as_f64())
            .collect();
        let big = g.world.nation(usa_id).gdp;
        let last = *usa.last().unwrap();
        assert!(
            (last - big).abs() / big < 1e-3,
            "a superpower's last point {} is not its GDP {}",
            last,
            big
        );
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

    /// Opening a quarrel in a theatre the server could not read used to open it
    /// somewhere else and say nothing. Measured on the live server as Iraq on
    /// seed 7, POST /api/command {"kind":"open_conflict","target":"Kuwait",
    /// "theatre":X}, reading back the theatre the conflict actually landed in:
    ///
    ///   X = "Balkans"  -> 200, errors [] -> the Balkans
    ///   X = "Gluf"     -> 200, errors [] -> the Gulf
    ///   X = "Nonsense" -> 200, errors [] -> the Gulf
    ///   X = ""         -> 200, errors [] -> the Gulf
    ///   X = 42         -> 200, errors [] -> the Gulf
    ///
    /// A player who asked for one operating area was given another, with the
    /// same answer they would have got had they been obeyed. The theatre is not
    /// cosmetic: it decides whose consent the escalation ladder needs above rung
    /// 5, who is defending home ground and at what discount, and which districts
    /// the front is fought over.
    ///
    /// The cause was that `unwrap_or_else` cannot tell a field that was not
    /// carried from one that could not be read, and only the first of those is
    /// a default.
    #[test]
    fn a_theatre_the_server_cannot_use_is_refused_not_replaced() {
        let g = Game::new(7, Some(NationId::Iraq));
        let me = NationId::Iraq;
        let theatre_of = |v: &serde_json::Value| match parse_command(&g.world, v, me) {
            Some(Command::OpenConflict { theatre, .. }) => Ok(theatre),
            Some(other) => panic!("wrong command: {:?}", other),
            None => Err(()),
        };

        // Not carrying the field is still the documented default, and it is the
        // path the browser itself takes — index.html posts `open_conflict` with
        // no theatre at all, so this arm must keep working exactly as it did.
        let asked = serde_json::json!({ "kind": "open_conflict", "target": "Kuwait" });
        let default = spheres_sim::war::theatre_between(&g.world, me, NationId::Kuwait);
        assert_eq!(theatre_of(&asked), Ok(default));
        assert_eq!(
            theatre_of(&serde_json::json!({
                "kind": "open_conflict", "target": "Kuwait", "theatre": null
            })),
            Ok(default)
        );
        assert!(
            INDEX.contains(r#"{ kind: "open_conflict", target }"#),
            "the browser must still be posting open_conflict without a theatre, \
             or the default arm above is no longer the one it uses"
        );

        // Every theatre a client can name still arrives intact, asked of the
        // whole table rather than a sample. `conflictCmd` posts `w.theatre`,
        // which is the Debug spelling the state payload carries, so a strict
        // parse that rejected any of these would break the war sheet.
        for t in spheres_sim::theatre::ALL_THEATRES {
            let debug = format!("{:?}", t);
            assert_eq!(
                theatre_of(&serde_json::json!({
                    "kind": "open_conflict", "target": "Kuwait", "theatre": debug
                })),
                Ok(t),
                "the payload's own spelling of {:?} must parse back",
                t
            );
        }

        // And the four measured substitutions are refused rather than replaced.
        for bad in [
            serde_json::json!("Gluf"),
            serde_json::json!("Nonsense"),
            serde_json::json!(""),
            serde_json::json!(42),
        ] {
            assert_eq!(
                theatre_of(&serde_json::json!({
                    "kind": "open_conflict", "target": "Kuwait", "theatre": bad
                })),
                Err(()),
                "{} was read as a theatre",
                bad
            );
        }
    }

    /// Taking a side in somebody else's war used to rewrite what you asked for
    /// rather than refuse it, in both of the two fields that say what joining
    /// means. Measured on the live server as the United States on seed 7,
    /// advanced until Iraq/Kuwait was on the board, then POST /api/command
    /// {"kind":"join","conflict":N,...} — every line 200 with `errors: []`:
    ///
    ///   objective "seize" -> seize      side_a true  -> side A
    ///   objective "siez"  -> DENY       side_a 1     -> side B
    ///   objective ""      -> DENY       side_a "true"-> side B
    ///   objective 3       -> DENY
    ///
    /// Neither substitution is cosmetic and neither is refundable. Deny is the
    /// one objective that seizes nothing — a player who asked to take ground
    /// bought a war fought to stop somebody else having it. `side_a` is worse:
    /// a client that said `1` instead of `true` was enrolled AGAINST the side it
    /// asked to fight for. Both cost the same fourteen political capital that
    /// asking correctly does, and the join is not undoable.
    #[test]
    fn a_join_the_server_cannot_read_is_refused_not_rewritten() {
        let g = Game::new(7, Some(NationId::USA));
        let me = NationId::USA;
        let join = |v: &serde_json::Value| match parse_command(&g.world, v, me) {
            Some(Command::JoinConflict { side_a, objective, .. }) => Ok((side_a, objective)),
            Some(other) => panic!("wrong command: {:?}", other),
            None => Err(()),
        };
        let asked = |o: serde_json::Value, s: serde_json::Value| {
            serde_json::json!({ "kind": "join", "conflict": 1, "objective": o, "side_a": s })
        };

        // What the browser posts must still go through untouched: conflictCmd
        // sends a literal "deny" and a real boolean.
        assert!(
            INDEX.contains(r#"cmd.side_a = value === 1; cmd.objective = "deny";"#),
            "the browser's join shape has moved; re-measure what it now sends"
        );
        assert_eq!(join(&asked("deny".into(), true.into())), Ok((true, Objective::Deny)));

        // Every objective a client can name still arrives intact, asked of the
        // whole set rather than a sample, in the spelling the state payload
        // itself uses for them.
        for o in [
            Objective::Deny,
            Objective::Degrade,
            Objective::Seize,
            Objective::Hold,
            Objective::Stabilise,
            Objective::Withdraw,
        ] {
            assert_eq!(
                join(&asked(o.label().into(), false.into())),
                Ok((false, o)),
                "the payload's own spelling of {:?} must parse back",
                o
            );
        }

        // Not carrying a field is left exactly as it was — this commit is about
        // a value the client DID supply being replaced by another one.
        assert_eq!(
            join(&serde_json::json!({ "kind": "join", "conflict": 1 })),
            Ok((false, Objective::Deny))
        );

        // And the measured substitutions, in both fields.
        for bad in [
            serde_json::json!("siez"),
            serde_json::json!(""),
            serde_json::json!("attack"),
            serde_json::json!(3),
        ] {
            assert_eq!(
                join(&asked(bad.clone(), false.into())),
                Err(()),
                "{} was read as an objective",
                bad
            );
        }
        for bad in [
            serde_json::json!(1),
            serde_json::json!(0),
            serde_json::json!("true"),
            serde_json::json!("A"),
        ] {
            assert_eq!(
                join(&asked("deny".into(), bad.clone())),
                Err(()),
                "{} was read as a side",
                bad
            );
        }
    }

    /// The payload used to contradict itself for a month after a federation
    /// dissolved: the same response listed the Soviet Union under `dead` AND
    /// under `wars[].posture`, standing at rung 6 — "standoff strike" — with a
    /// stake of 0.45 against a state that no longer existed. Reported by
    /// yugoslavia-04 as F-18.
    ///
    /// Measured before the fix by walking twelve seeds for thirty years and
    /// cross-checking the two lists in every monthly payload: three
    /// occurrences, each lasting exactly one month, all at the Soviet
    /// dissolution in September 1993 —
    ///
    ///   seed 1: conflict 4, USSR(dead)@5 vs Poland@1          [Frozen]
    ///   seed 8: conflict 1, USSR(dead)@1 vs China@1           [Frozen]
    ///   seed 2: conflict 6, South Africa@6, Mozambique@2,
    ///           USSR(dead)@6, Angola@6                        [Conventional]
    ///
    /// The third is why this is two rules and not one. A three-cornered war
    /// outlives one of its parties, so the conflict is still real and must
    /// still be served — it is only the dead row inside it that must go. The
    /// first two have nobody left on one side, and a conflict with nobody on
    /// one side of it is not a conflict.
    ///
    /// The sim is NOT pruned. It keeps its own conflict list exactly as it did
    /// and clears these itself on the following tick; this test therefore
    /// asserts on the payload while asserting that the world behind it is
    /// unchanged, which is the line between a view fix and a model change.
    #[test]
    fn a_dissolved_state_is_not_served_as_a_live_belligerent() {
        let mut g = Game::new(1, None);
        let mut checked = 0;
        let mut wars_seen = 0;
        let mut sim_held_a_dead_belligerent = 0;
        for _ in 0..(30 * 12) {
            tick_month(&mut g.world, &[]);
            g.snapshot();
            let s = state_json(&g, None);
            let dead: std::collections::HashSet<&str> = s["dead"]
                .as_array()
                .unwrap()
                .iter()
                .map(|d| d["id"].as_str().unwrap())
                .collect();
            for war in s["wars"].as_array().unwrap() {
                wars_seen += 1;
                let rows = war["posture"].as_array().unwrap();
                for b in rows {
                    assert!(
                        !dead.contains(b["id"].as_str().unwrap()),
                        "{}: conflict {} is standing {} on rung {} in the same \
                         payload that lists it as dead",
                        s["date"].as_str().unwrap(),
                        war["id"],
                        b["id"],
                        b["rung"]
                    );
                }
                // ...and what is left is still a conflict, with somebody on
                // each side of it. This is the half the row filter alone
                // cannot give: dropping the dead must not leave a war being
                // fought by one party.
                assert!(
                    rows.iter().any(|b| b["side_a"] == true)
                        && rows.iter().any(|b| b["side_a"] == false),
                    "{}: conflict {} is served with nobody on one side of it",
                    s["date"].as_str().unwrap(),
                    war["id"]
                );
            }
            // What the sim is holding underneath, this same month. Seed 1 is
            // one of the three measured worlds, so this counter must not be
            // zero — if it were, the loop above would be proving nothing and
            // the test would pass on a world where the defect cannot occur.
            if g
                .world
                .conflicts
                .iter()
                .flat_map(|c| c.posture.iter())
                .any(|b| !g.world.nation_opt(b.nation).is_some_and(|n| n.alive))
            {
                sim_held_a_dead_belligerent += 1;
            }
            checked += 1;
        }
        assert_eq!(checked, 360);
        assert!(wars_seen > 0, "thirty years produced no conflicts to check");
        assert!(
            sim_held_a_dead_belligerent > 0,
            "the sim never held a dead belligerent in this world, so the filter \
             above was never exercised and the assertions in it mean nothing"
        );
    }

    /// Time is the one thing this game cannot give back, and the route that
    /// spends it was reading its own argument with `unwrap_or(1)`. Every
    /// `months` the server could not use silently advanced the world by one
    /// month and answered 200 — the same answer, and the same distance, as a
    /// request that asked for nothing at all.
    ///
    /// Measured on the live server, Poland on seed 7, from a fresh 1990 each
    /// time. Before:
    ///
    ///   {"months":12}                    -> Jun 1990  (5, stopped by an event)
    ///   {"months":-5}                    -> Feb 1990  (1)
    ///   {"months":"12"}                  -> Feb 1990  (1)
    ///   {"months":3.5}                   -> Feb 1990  (1)
    ///   {"months":999999999999999999999} -> Feb 1990  (1)
    ///   {"months":[12]}                  -> Feb 1990  (1)
    ///   {}                               -> Feb 1990  (1)  <- the real default
    ///
    /// A client asking for five years and given one month is out by sixty, and
    /// nothing in the answer says so. This is the last field of the four the
    /// route family used to substitute — the seed (F-22), the nation (F-23), the
    /// theatre and the join (F-20, F-21) — and the one the body-parse fix
    /// (F-17) named in its own test as "the one that moved the clock the wrong
    /// distance".
    #[test]
    fn a_span_the_server_cannot_use_does_not_move_the_clock_some_other_distance() {
        // Not asking is still one month. This is how the browser's Enact
        // button and the space bar both behave, so it has to stay.
        assert_eq!(asked_months(&serde_json::json!({})), Ok(1));
        assert_eq!(asked_months(&serde_json::json!({ "months": null })), Ok(1));

        // Every span a client can actually ask for arrives intact.
        for m in [0u64, 1, 6, 12, 60, MAX_ADVANCE] {
            assert_eq!(asked_months(&serde_json::json!({ "months": m })), Ok(m));
        }
        // The browser's four buttons, read off the page rather than retyped, so
        // this cannot pass while the page asks for something else.
        for span in ["1", "6", "12", "60"] {
            assert!(
                INDEX.contains(&format!("data-adv=\"{}\"", span)),
                "the page no longer offers a {}-month advance; re-derive this list",
                span
            );
            let m: u64 = span.parse().unwrap();
            assert_eq!(asked_months(&serde_json::json!({ "months": m })), Ok(m));
        }

        // The clamp is a limit on the work, not a substitution of the question,
        // and it stays a clamp rather than becoming a refusal.
        assert_eq!(
            asked_months(&serde_json::json!({ "months": MAX_ADVANCE + 1 })),
            Ok(MAX_ADVANCE)
        );
        assert_eq!(asked_months(&serde_json::json!({ "months": u64::MAX })), Ok(MAX_ADVANCE));

        // And the five measured substitutions.
        for bad in [
            serde_json::json!(-5),
            serde_json::json!("12"),
            serde_json::json!(3.5),
            serde_json::json!(999999999999999999999u128 as f64),
            serde_json::json!([12]),
        ] {
            let e = asked_months(&serde_json::json!({ "months": bad }))
                .expect_err("must be refused");
            assert!(e.contains("is not a number of months"), "unhelpful refusal: {e}");
        }
    }
}
