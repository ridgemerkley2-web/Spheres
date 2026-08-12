//! Local web front end for SPHERES.
//!
//! The simulation stays the single source of truth: this server owns one
//! `WorldState`, applies commands through the same queue the CLI uses, and
//! serves a browser UI that renders it. No game logic lives here.

use spheres_sim::init::world_1990;
use spheres_sim::world::*;
use spheres_sim::{apply_command, load, save, tick_month, Command};
use std::sync::Mutex;
use tiny_http::{Header, Method, Response, Server};

const INDEX: &str = include_str!("../ui/index.html");
/// Baked country outlines — see `src/bin/mapgen.rs`.
const WORLD_JS: &str = include_str!("../ui/world.js");

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
        let mut rules = GameRules::default();
        rules.seed = seed;
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
    for id in ALL_START_NATIONS.iter().chain(SUCCESSOR_NATIONS.iter()) {
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
        || h.contains("repels");
    structural || me.map_or(false, |m| h.contains(&m.name().to_lowercase()))
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
        "sanctioned_by_me": me.map_or(false, |m| w.is_sanctioning(m, n.id)),
        "sanctioning_me": me.map_or(false, |m| w.is_sanctioning(n.id, m)),
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
        "log": log,
        "flags": w.flags,
        "interrupt": interrupt,
    })
}

/// Translate the UI's flat command objects into sim commands.
fn parse_command(v: &serde_json::Value, me: NationId) -> Option<Command> {
    let kind = v.get("kind")?.as_str()?;
    let num = || v.get("value").and_then(|x| x.as_f64());
    let target = || {
        v.get("target")
            .and_then(|x| x.as_str())
            .and_then(NationId::parse)
    };
    Some(match kind {
        "rate" => Command::SetInterestRate { nation: me, rate: num()? },
        "tax" => Command::SetTaxRate { nation: me, rate: num()? },
        "military" => Command::SetMilSpend { nation: me, share: num()? },
        "invest" => Command::SetStateInvest { nation: me, share: num()? },
        "sanction" => Command::Sanction { imposer: me, target: target()? },
        "lift" => Command::LiftSanction { imposer: me, target: target()? },
        "improve" => Command::ImproveRelations { from: me, to: target()? },
        "war" => Command::DeclareWar { attacker: me, defender: target()? },
        _ => return None,
    })
}

fn json_response(v: serde_json::Value) -> Response<std::io::Cursor<Vec<u8>>> {
    let body = serde_json::to_string(&v).unwrap_or_else(|_| "{}".into());
    Response::from_string(body).with_header(
        Header::from_bytes(&b"Content-Type"[..], &b"application/json; charset=utf-8"[..]).unwrap(),
    )
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

        let response = match (&method, url_path.as_str()) {
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
            (Method::Get, "/api/state") => {
                let g = game.lock().unwrap();
                json_response(state_json(&g, None))
            }
            (Method::Get, "/api/history") => {
                let only = request
                    .url()
                    .split_once("nation=")
                    .and_then(|(_, q)| NationId::parse(q.split('&').next().unwrap_or("")));
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
                *g = Game::new(seed, player);
                json_response(state_json(&g, None))
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
                        .map(|a| a.iter().filter_map(|v| parse_command(v, me)).collect())
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
                        if let Some(cmd) = parse_command(v, me) {
                            if let Err(e) = apply_command(&mut g.world, &cmd) {
                                errors.push(e);
                            }
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
}
