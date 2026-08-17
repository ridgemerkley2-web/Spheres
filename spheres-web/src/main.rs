//! Local web front end for SPHERES.
//!
//! The simulation stays the single source of truth: this server owns one
//! `WorldState`, applies commands through the same queue the CLI uses, and
//! serves a browser UI that renders it. No game logic lives here.

use spheres_sim::init::world_1990;
use spheres_sim::stratagems;
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
    for id in all_nations() {
        if hay.contains(&id.name().to_lowercase()) && !out.contains(id) {
            out.push(*id);
        }
    }
    out
}

fn is_major(headline: &str, me: Option<NationId>) -> bool {
    let h = headline.to_lowercase();
    // The spheres system writes a lot of quiet news — a mission opening, a
    // client drifting out of an orbit nobody was paying for — and most of it
    // names a great power, so without this a player advancing a year would be
    // stopped every single month. Only a client actually CHANGING HANDS is worth
    // interrupting for, and that one is caught by the structural list below.
    let routine = h.contains("drifts out of")
        || h.contains("moves into")
        || h.contains("opens a mission")
        || h.contains("begins courting")
        || h.contains("redoubles its efforts")
        || h.contains("gives up its efforts")
        || h.contains("quietly winds up")
        || h.contains("closes its mission")
        || h.contains("is drifting");
    if routine {
        return false;
    }
    let structural = h.starts_with("war:")
        || h.contains("dissolved")
        || h.contains("has annexed")
        || h.contains("capitulates")
        || h.contains("revolution in")
        || h.contains("sues for peace")
        || h.contains("repels")
        // A sphere changing hands is the loud one, and it shouts.
        || h.contains("leaves");
    structural || me.map_or(false, |m| h.contains(&m.name().to_lowercase()))
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
        // The namesake system, per nation. Everything here is read off
        // spheres_sim::influence — the browser computes none of it, and the
        // price it shows is the price apply_command will charge.
        "influence": influence_json(w, n.id),
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

/// One capital's influence board: who holds a position in it, which way it
/// leans, whether it is being fought over, and — if the player is somebody —
/// what taking it would cost them. Every field comes from
/// `spheres_sim::influence`; nothing here is computed twice.
fn influence_json(w: &WorldState, id: NationId) -> serde_json::Value {
    let me = w.player;
    let board: Vec<serde_json::Value> = w
        .stakeholders(id)
        .into_iter()
        .map(|(p, s)| {
            serde_json::json!({
                "id": format!("{:?}", p),
                "name": p.name(),
                "stock": s,
                "effort": w.effort(p, id),
                "aligned": w.aligned_to(id) == Some(p),
                "is_me": Some(p) == me,
            })
        })
        .collect();
    let quote = me
        .filter(|m| *m != id && spheres_sim::influence::can_be_client(id))
        .map(|m| {
            let q = spheres_sim::influence::quote_take(w, m, id);
            serde_json::json!({
                "my_stock": q.my_stock,
                "their_stock": q.their_stock,
                "target_stock": q.target_stock,
                "net_per_month": q.net_per_month,
                "months": q.months,
                "political_capital": q.political_capital,
                "needs_an_instrument": q.needs_an_instrument,
                "holder": q.holder.map(|h| h.name()),
                "my_effort": w.effort(m, id),
                "upkeep": w.sphere_upkeep(m, id),
                "open_price": spheres_sim::influence::open_price(w, m, id, 1.0),
            })
        });
    serde_json::json!({
        "can_be_client": spheres_sim::influence::can_be_client(id),
        "aligned_to": w.aligned_to(id).map(|p| p.name()),
        "aligned_to_id": w.aligned_to(id).map(|p| format!("{:?}", p)),
        "contested": w.is_contested(id),
        "board": board,
        "quote": quote,
    })
}

/// The player's whole sphere on one payload: what they hold, what it costs every
/// month, and where somebody else's client sits. This is the card the briefing
/// leads with, so it must arrive with the state rather than a fetch behind it.
fn spheres_json(w: &WorldState, me: NationId) -> serde_json::Value {
    let mine: Vec<serde_json::Value> = w
        .sphere_of(me)
        .into_iter()
        .map(|(c, s)| {
            let rival = w.contested_by(me, c);
            serde_json::json!({
                "id": format!("{:?}", c),
                "name": c.name(),
                "stock": s,
                "effort": w.effort(me, c),
                "upkeep": w.sphere_upkeep(me, c),
                "aligned": w.aligned_to(c) == Some(me),
                "held_by": w.aligned_to(c).filter(|h| *h != me).map(|h| h.name()),
                "contested": w.is_contested(c),
                "rival": rival.map(|(q, s)| serde_json::json!({ "name": q.name(), "stock": s })),
            })
        })
        .collect();
    // What everyone else holds, most valuable first, with the price of taking it.
    let mut others: Vec<(NationId, NationId, f64)> = w
        .nations
        .iter()
        .filter(|n| n.alive)
        .filter_map(|n| w.aligned_to(n.id).map(|h| (n.id, h, w.stake(h, n.id))))
        .filter(|(_, h, _)| *h != me)
        .collect();
    others.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
    let targets: Vec<serde_json::Value> = others
        .into_iter()
        .take(14)
        .map(|(c, h, s)| {
            let q = spheres_sim::influence::quote_take(w, me, c);
            serde_json::json!({
                "id": format!("{:?}", c),
                "name": c.name(),
                "holder": h.name(),
                "their_stock": s,
                "my_stock": q.my_stock,
                "months": q.months,
                "political_capital": q.political_capital,
                "needs_an_instrument": q.needs_an_instrument,
                "contested": w.is_contested(c),
            })
        })
        .collect();
    serde_json::json!({
        "nation": format!("{:?}", me),
        "political_capital": w.nation(me).political_capital,
        "bill": w.sphere_bill(me),
        "clients": mine,
        "targets": targets,
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
                "start": month_name(war.start_month, war.start_year),
            })
        })
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
        // Carried on every state payload rather than fetched separately, because
        // the offer list changes with the world: a month that breaks the
        // inflation closes the peg, and the panel must not be a frame behind.
        "stratagems": w.player.map(|p| stratagems_json(w, p)),
        // ...and the sphere, for the same reason: it moves every month, so a
        // panel that fetched it separately would render a world one tick old.
        "spheres": w.player.map(|p| spheres_json(w, p)),
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
        // The stratagem carries an id rather than a value or a target; the sim
        // re-checks availability and charges the political capital itself.
        // The namesake commands. `value` is the effort 0..1; the sim prices it,
        // charges it, and charges it again every month it stands.
        "project" => Command::ProjectInfluence {
            patron: me,
            client: target()?,
            effort: num()?,
        },
        "abandon" => Command::AbandonSphere { patron: me, client: target()? },
        "stratagem" => Command::EnactStratagem {
            nation: me,
            id: v.get("id")?.as_str()?.to_string(),
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
        let cmd = parse_command(&posted, NationId::Poland).expect("the UI's shape must parse");
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
        assert!(s["stratagems"]["offers"].as_array().unwrap().len() >= 1);
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

    /// The namesake system has to be legible from the first screen of a new
    /// game, not after ten years of play. January 1990 opens with a real board —
    /// see `influence::seat_1990` — and this asserts the payload carries it.
    #[test]
    fn the_opening_board_reaches_the_browser() {
        let g = Game::new(1990, Some(NationId::USA));
        let s = state_json(&g, None);
        let sph = &s["spheres"];
        assert_eq!(sph["nation"], "USA");
        let clients = sph["clients"].as_array().unwrap();
        assert!(
            clients.len() >= 10,
            "Washington opens January 1990 holding {} positions",
            clients.len()
        );
        assert!(
            sph["bill"].as_f64().unwrap() > 0.0,
            "a sphere that costs nothing a month is not a spend-to-hold economy"
        );
        // Somebody else's clients, priced.
        let targets = sph["targets"].as_array().unwrap();
        assert!(!targets.is_empty(), "nobody else holds anything in 1990");
        for t in targets {
            assert!(t["their_stock"].as_f64().unwrap() > 0.0);
            assert!(t["holder"].as_str().unwrap() != "United States");
        }
        // Moscow's own sphere is on the board and it is Moscow's.
        let angola = s["nations"]
            .as_array()
            .unwrap()
            .iter()
            .find(|n| n["name"] == "Angola")
            .expect("Angola is on the 1990 roster");
        assert_eq!(angola["influence"]["aligned_to"], "Soviet Union");
        // ...and a spectator with no nation gets null rather than a fabricated one.
        assert!(state_json(&Game::new(1990, None), None)["spheres"].is_null());
    }

    /// A price the player is shown must be the price the sim charges, and a
    /// campaign the arithmetic cannot deliver must say so rather than name a
    /// number it will not honour.
    #[test]
    fn the_quote_the_panel_prints_is_the_quote_the_sim_gives() {
        let g = Game::new(1990, Some(NationId::USA));
        let angola = g
            .world
            .nations
            .iter()
            .find(|n| n.id == NationId::Angola)
            .map(|n| nation_json(&g.world, n))
            .unwrap();
        let q = &angola["influence"]["quote"];
        assert_eq!(q["holder"], "Soviet Union");
        // Eighty-odd points of Soviet position is out of reach of embassy
        // receptions, and the panel must be told that in the payload rather than
        // work it out for itself.
        assert_eq!(q["needs_an_instrument"], true);
        assert!(q["months"].is_null());
        assert!(q["target_stock"].as_f64().unwrap() > q["their_stock"].as_f64().unwrap());

        // A great power is nobody's client, and the payload says so instead of
        // offering a price.
        let japan = g
            .world
            .nations
            .iter()
            .find(|n| n.id == NationId::Japan)
            .map(|n| nation_json(&g.world, n))
            .unwrap();
        assert_eq!(japan["influence"]["can_be_client"], false);
        assert!(japan["influence"]["quote"].is_null());
    }

    /// The verb, end to end: the flat JSON the slider posts, through the command
    /// queue, to the world moving and the political capital being spent. A
    /// mechanic the player cannot reach from their seat is not a mechanic.
    #[test]
    fn the_effort_dial_reaches_the_sim_through_the_command_route() {
        let mut g = Game::new(1990, Some(NationId::USA));
        let before = g.world.nation(NationId::USA).political_capital;
        let posted = serde_json::json!({ "kind": "project", "target": "Angola", "value": 1.0 });
        let cmd = parse_command(&posted, NationId::USA).expect("the UI's shape must parse");
        match &cmd {
            Command::ProjectInfluence { patron, client, effort } => {
                assert_eq!(*patron, NationId::USA);
                assert_eq!(*client, NationId::Angola);
                assert_eq!(*effort, 1.0);
            }
            other => panic!("wrong command: {:?}", other),
        }
        apply_command(&mut g.world, &cmd).unwrap();
        assert_eq!(g.world.effort(NationId::USA, NationId::Angola), 1.0);
        assert!(
            g.world.nation(NationId::USA).political_capital < before,
            "opening a mission cost nothing: {:.1} -> {:.1}",
            before,
            g.world.nation(NationId::USA).political_capital
        );

        // ...and the standing bill, which is the half that makes it spend-to-hold.
        let bill_before = g.world.sphere_bill(NationId::USA);
        let held = g.world.nation(NationId::USA).political_capital;
        tick_month(&mut g.world, &[]);
        assert!(bill_before > 0.0);
        assert!(
            g.world.nation(NationId::USA).political_capital < held + 1.0,
            "a month passed and the sphere was free"
        );

        // Giving up is always available and it is charged.
        let posted = serde_json::json!({ "kind": "abandon", "target": "Angola" });
        let cmd = parse_command(&posted, NationId::USA).expect("abandon must parse");
        apply_command(&mut g.world, &cmd).unwrap();
        assert_eq!(g.world.effort(NationId::USA, NationId::Angola), 0.0);
    }

    /// The surface itself. These strings are the panel's structure: if the markup
    /// or the wiring is renamed away this fails, rather than shipping a namesake
    /// system with no verb attached to it.
    #[test]
    fn the_spheres_panel_exists_and_is_wired_to_the_route() {
        // The card in the left column, and the four things a row must say.
        assert!(INDEX.contains("function spheresHtml"));
        assert!(INDEX.contains("id=\"sphereCard\""));
        assert!(INDEX.contains("class=\"sphbill\""));
        assert!(INDEX.contains("data-sphere="));
        assert!(INDEX.contains("pc/mo"));
        // The dossier's half: the board, the quote, and the refusal.
        assert!(INDEX.contains("function influenceHtml"));
        assert!(INDEX.contains("needs_an_instrument"));
        assert!(INDEX.contains("class=\"quote"));
        // The verbs, the hands that press them, and the route they go down.
        assert!(INDEX.contains("window.project = async"));
        assert!(INDEX.contains("window.abandonSphere = async"));
        assert!(INDEX.contains("id=\"projectBtn\""));
        assert!(INDEX.contains("kind: \"project\""));
        assert!(INDEX.contains("kind: \"abandon\""));
        // Read from the payload the server actually sends.
        assert!(INDEX.contains("S.spheres"));
        assert!(INDEX.contains("n.influence"));
        // No CDN, no build step.
        assert!(!INDEX.contains("https://"), "the UI must stay self-contained");
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
