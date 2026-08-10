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

/// A year-end snapshot, kept so the UI can draw history rather than a single frame.
struct Snapshot {
    year: i32,
    gdp: Vec<(NationId, f64)>,
    oil: f64,
}

struct Game {
    world: WorldState,
    log: Vec<(String, String)>, // (date, headline)
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
        self.history.push(Snapshot {
            year: self.world.year,
            gdp: self
                .world
                .nations
                .iter()
                .filter(|n| n.alive)
                .map(|n| (n.id, n.gdp))
                .collect(),
            oil: self.world.oil_price,
        });
    }

    /// Advance up to `months`, stopping early on an event worth reacting to.
    /// Returns whether it stopped early and why.
    fn advance(&mut self, months: usize, commands: Vec<Command>) -> (bool, Option<String>) {
        let mut queued = commands;
        for i in 0..months {
            let cmds = std::mem::take(&mut queued);
            let year_before = self.world.year;
            let headlines = tick_month(&mut self.world, &cmds);
            let date = self.world.date_str();
            for h in &headlines {
                self.log.push((date.clone(), h.clone()));
            }
            if self.world.year != year_before {
                self.snapshot();
            }
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
        "debt": n.debt_gdp,
        "stability": n.stability,
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
    })
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
        .wars
        .iter()
        .map(|war| {
            serde_json::json!({
                "attacker": war.attacker.name(),
                "attacker_id": format!("{:?}", war.attacker),
                "defender": war.defender.name(),
                "defender_id": format!("{:?}", war.defender),
                "progress": war.progress,
                "attacker_allies": war.attacker_allies.iter().map(|a| a.name()).collect::<Vec<_>>(),
                "defender_allies": war.defender_allies.iter().map(|a| a.name()).collect::<Vec<_>>(),
                "start": format!("{} {}", war.start_month, war.start_year),
            })
        })
        .collect();
    let history: Vec<serde_json::Value> = g
        .history
        .iter()
        .map(|s| {
            serde_json::json!({
                "year": s.year,
                "oil": s.oil,
                "gdp": s.gdp.iter().map(|(id, v)| (format!("{:?}", id), *v)).collect::<std::collections::BTreeMap<_, _>>(),
            })
        })
        .collect();
    // Most recent first, capped — the UI shows a feed, not an archive.
    let log: Vec<serde_json::Value> = g
        .log
        .iter()
        .rev()
        .take(300)
        .map(|(d, h)| serde_json::json!({ "date": d, "text": h }))
        .collect();

    serde_json::json!({
        "date": w.date_str(),
        "year": w.year,
        "month": w.month,
        "player": w.player.map(|p| format!("{:?}", p)),
        "player_name": w.player.map(|p| p.name()),
        "oil_price": w.oil_price,
        "nations": nations,
        "dead": dead,
        "wars": wars,
        "log": log,
        "history": history,
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
                if let Some(list) = payload.get("commands").and_then(|c| c.as_array()) {
                    for v in list {
                        if let Some(cmd) = parse_command(v, me) {
                            if let Err(e) = apply_command(&mut g.world, &cmd) {
                                errors.push(e);
                            }
                        }
                    }
                }
                let date = g.world.date_str();
                let fresh: Vec<String> = g.world.headlines.clone();
                for h in fresh {
                    g.log.push((date.clone(), h));
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
