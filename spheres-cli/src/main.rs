use spheres_sim::init::world_1990;
use spheres_sim::world::*;
use spheres_sim::{load, save, tick_month, Command};
use std::io::{self, BufRead, Write};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("run") => {
            let years: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(30);
            let seed: u64 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(1990);
            headless(years, seed);
        }
        Some("play") => {
            let seed: u64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(1990);
            play(seed);
        }
        Some("resume") => {
            let path = args.get(2).map(|s| s.as_str()).unwrap_or("save.json");
            let s = std::fs::read_to_string(path).expect("read save");
            let w = load(&s).expect("parse save");
            play_loop(w);
        }
        _ => {
            println!("SPHERES v0.5 — usage:");
            println!("  spheres-cli run [years] [seed]   headless world report");
            println!("  spheres-cli play [seed]          interactive game from Jan 1990");
            println!("  spheres-cli resume [save.json]   continue a saved game");
        }
    }
}

fn headless(years: usize, seed: u64) {
    let mut rules = GameRules::default();
    rules.seed = seed;
    let mut w = world_1990(rules);
    for _ in 0..years * 12 {
        let headlines = tick_month(&mut w, &[]);
        for h in headlines {
            println!("[{}] {}", w.date_str(), h);
        }
    }
    println!("\n=== World in {} ===", w.year);
    report(&w);
}

fn report(w: &WorldState) {
    let mut rows: Vec<&Nation> = w.nations.iter().filter(|n| n.alive).collect();
    rows.sort_by(|a, b| b.gdp.partial_cmp(&a.gdp).unwrap());
    println!("{:<16} {:>10} {:>7} {:>7} {:>6} {:>6} {:>6} {:>6}", "Nation", "GDP $bn", "Grow%", "Infl%", "Debt%", "Stab", "PolCap", "Mil");
    for n in rows {
        println!(
            "{:<16} {:>10.0} {:>6.1}% {:>6.1}% {:>5.0}% {:>6.0} {:>6.0} {:>6.0}",
            n.id.name(), n.gdp, n.growth_last * 100.0, n.inflation * 100.0,
            n.debt_gdp * 100.0, n.stability, n.political_capital, n.mil_strength
        );
    }
    println!("Oil: ${:.0}/bbl   Wars: {}   Sanction pairs: {}", w.oil_price, w.conflicts.len(), w.sanctions.len());
}

fn play(seed: u64) {
    println!("=== SPHERES — January 1990 ===\n");
    println!("Playable nations:");
    for id in ALL_START_NATIONS {
        println!("  {}", id.name());
    }
    let nation = loop {
        print!("\nChoose your nation: ");
        io::stdout().flush().unwrap();
        let line = read_line();
        if let Some(id) = NationId::parse(&line) {
            break id;
        }
        println!("Unrecognized. Try e.g. 'USA', 'China', 'Iraq'.");
    };
    let mut rules = GameRules::default();
    rules.seed = seed;
    let mut w = world_1990(rules);
    w.player = Some(nation);
    println!("\nYou govern {}. The Cold War is ending. History is unwritten.\n", nation.name());
    play_loop(w);
}

fn play_loop(mut w: WorldState) {
    let me = w.player.expect("player set");
    briefing(&w, me);
    let mut queued: Vec<Command> = vec![];
    loop {
        print!("\n[{}] {} > ", w.date_str(), me.name());
        io::stdout().flush().unwrap();
        let line = read_line();
        let mut parts = line.trim().splitn(2, ' ');
        let verb = parts.next().unwrap_or("").to_lowercase();
        let rest = parts.next().unwrap_or("").trim().to_string();

        match verb.as_str() {
            "" => {}
            "help" | "?" => help(),
            "quit" | "exit" => {
                println!("The world spins on without you.");
                break;
            }
            "save" => {
                std::fs::write("save.json", save(&w)).expect("write save");
                println!("Saved to save.json (resume with: spheres-cli resume save.json)");
            }
            "status" => briefing(&w, me),
            "world" => report(&w),
            "options" | "stratagems" | "opt" => {
                let opts = spheres_sim::stratagems::available(&w, me);
                let held = w.nation(me).political_capital;
                if opts.is_empty() {
                    println!("  The world is offering you nothing this month.");
                } else {
                    println!("\n  Standing: {:.0} political capital\n", held);
                    for s in opts {
                        let afford = if held >= s.cost { "" } else { "   [cannot afford]" };
                        println!("  {:<22} {:>3.0} pc{}", s.id, s.cost, afford);
                        println!("      {}", s.name);
                        println!("      {}", s.blurb);
                        println!("      Open to you because: {}\n", s.because);
                    }
                    println!("  enact <id>   to take one");
                }
            }
            "enact" => {
                if rest.is_empty() {
                    println!("Usage: enact shock_therapy   (see 'options')");
                } else {
                    let cmd = Command::EnactStratagem { nation: me, id: rest.clone() };
                    match spheres_sim::apply_command(&mut w, &cmd) {
                        Ok(()) => println!("  Done. It takes effect immediately."),
                        Err(e) => println!("  {}", e),
                    }
                }
            }
            "government" | "gov" | "parties" => government_view(&w, me),
            "invite" => {
                if rest.is_empty() {
                    println!("Usage: invite it_psi   (see 'government')");
                } else {
                    let cmd = Command::InviteToGovernment { nation: me, party: rest.clone() };
                    match spheres_sim::apply_command(&mut w, &cmd) {
                        Ok(()) => government_view(&w, me),
                        Err(e) => println!("  {}", e),
                    }
                }
            }
            "expel" => {
                if rest.is_empty() {
                    println!("Usage: expel it_pli   (see 'government')");
                } else {
                    let cmd = Command::ExpelFromGovernment { nation: me, party: rest.clone() };
                    match spheres_sim::apply_command(&mut w, &cmd) {
                        Ok(()) => government_view(&w, me),
                        Err(e) => println!("  {}", e),
                    }
                }
            }
            "election" => {
                println!("Go to the country early? 25 political capital, and the polls are the polls. (yes/no)");
                print!("> ");
                io::stdout().flush().unwrap();
                if read_line().trim().eq_ignore_ascii_case("yes") {
                    match spheres_sim::apply_command(&mut w, &Command::CallElection { nation: me }) {
                        Ok(()) => {
                            for h in w.headlines.clone() {
                                println!("  [{}] {}", w.date_str(), h);
                            }
                            government_view(&w, me);
                        }
                        Err(e) => println!("  {}", e),
                    }
                } else {
                    println!("You stay in office.");
                }
            }
            "secure" | "patronage" => match spheres_sim::government::Pillar::parse(&rest) {
                Some(pillar) => {
                    let cmd = Command::SecurePillar { nation: me, pillar };
                    match spheres_sim::apply_command(&mut w, &cmd) {
                        Ok(()) => {
                            for h in w.headlines.clone() {
                                println!("  {}", h);
                            }
                            government_view(&w, me);
                        }
                        Err(e) => println!("  {}", e),
                    }
                }
                None => println!("Usage: secure army   (army/party/security/business/clergy)"),
            },
            "relations" => {
                let mut rels: Vec<(NationId, f64)> = ALL_START_NATIONS
                    .iter()
                    .chain(std::iter::once(&NationId::Russia))
                    .filter(|x| **x != me && w.nations.iter().any(|n| n.id == **x && n.alive))
                    .map(|x| (*x, w.relation(me, *x)))
                    .collect();
                rels.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                for (x, v) in rels {
                    let tag = if w.is_sanctioning(me, x) { " [sanctioned by you]" }
                        else if w.is_sanctioning(x, me) { " [sanctioning you]" } else { "" };
                    println!("  {:<16} {:>+5.0}{}", x.name(), v, tag);
                }
            }
            "rate" => match rest.parse::<f64>() {
                Ok(v) => {
                    queued.push(Command::SetInterestRate { nation: me, rate: v / 100.0 });
                    println!("Queued: policy rate -> {:.1}%", v);
                }
                Err(_) => println!("Usage: rate 6.5   (percent)"),
            },
            "tax" => match rest.parse::<f64>() {
                Ok(v) => {
                    queued.push(Command::SetTaxRate { nation: me, rate: v / 100.0 });
                    println!("Queued: tax take -> {:.1}% of GDP", v);
                }
                Err(_) => println!("Usage: tax 30"),
            },
            "military" | "mil" => match rest.parse::<f64>() {
                Ok(v) => {
                    queued.push(Command::SetMilSpend { nation: me, share: v / 100.0 });
                    println!("Queued: military spending -> {:.1}% of GDP", v);
                }
                Err(_) => println!("Usage: military 4.5"),
            },
            "invest" => match rest.parse::<f64>() {
                Ok(v) => {
                    queued.push(Command::SetStateInvest { nation: me, share: v / 100.0 });
                    println!("Queued: state investment -> {:.1}% of GDP", v);
                }
                Err(_) => println!("Usage: invest 8"),
            },
            "sanction" => match NationId::parse(&rest) {
                Some(t) => {
                    queued.push(Command::Sanction { imposer: me, target: t });
                    println!("Queued: sanction {}", t.name());
                }
                None => println!("Usage: sanction Iraq"),
            },
            "lift" => match NationId::parse(&rest) {
                Some(t) => {
                    queued.push(Command::LiftSanction { imposer: me, target: t });
                    println!("Queued: lift sanctions on {}", t.name());
                }
                None => println!("Usage: lift Iraq"),
            },
            "improve" => match NationId::parse(&rest) {
                Some(t) => {
                    queued.push(Command::ImproveRelations { from: me, to: t });
                    println!("Queued: diplomatic push toward {}", t.name());
                }
                None => println!("Usage: improve China"),
            },
            "war" => match NationId::parse(&rest) {
                Some(t) => {
                    println!("Declare war on {}? This will have consequences. (yes/no)", t.name());
                    print!("> ");
                    io::stdout().flush().unwrap();
                    if read_line().trim().eq_ignore_ascii_case("yes") {
                        queued.push(Command::DeclareWar { attacker: me, defender: t });
                        println!("Queued: WAR on {}", t.name());
                    } else {
                        println!("Stand down.");
                    }
                }
                None => println!("Usage: war Kuwait"),
            },
            // ---- Getting in. Two verbs, and without them the eight below are
            // unreachable: every one of them needs you to already be a party to
            // something. ----
            "quarrel" => match NationId::parse(&rest) {
                Some(t) => {
                    let theatre = spheres_sim::war::theatre_between(&w, me, t);
                    queued.push(Command::OpenConflict { opener: me, target: t, theatre });
                    println!(
                        "Queued: open a public quarrel with {} over {} — rung 1, rhetoric.",
                        t.name(),
                        theatre.name()
                    );
                }
                None => println!("Usage: quarrel Iraq"),
            },
            "join" => match join_command(&w, me, &rest) {
                Ok(c) => {
                    println!("Queued: {}", describe_ladder(&c));
                    queued.push(c);
                }
                Err(e) => println!("{}", e),
            },
            // ---- The commitment ladder. One conflict at a time, named by the
            // theatre it is fought in, because that is how the briefing prints
            // it and a player should never have to learn an integer id. ----
            "commit" | "objective" | "roe" | "ceiling" | "redline" => {
                match ladder_command(&w, me, &verb, &rest) {
                    Ok(c) => {
                        println!("Queued: {}", describe_ladder(&c));
                        queued.push(c);
                    }
                    Err(e) => println!("{}", e),
                }
            }
            "access" => match access_command(&w, me, &rest) {
                Ok(c) => {
                    println!("Queued: {}", describe_ladder(&c));
                    queued.push(c);
                }
                Err(e) => println!("{}", e),
            },
            "next" | "n" => {
                advance(&mut w, &mut queued, 1);
            }
            "year" | "y" => {
                advance(&mut w, &mut queued, 12);
            }
            _ => {
                // "next 6" style
                if let Ok(k) = verb.parse::<usize>() {
                    advance(&mut w, &mut queued, k.min(120));
                } else {
                    println!("Unknown command. Type 'help'.");
                }
            }
        }
        if !w.nation(me).alive {
            println!("\n*** {} has been destroyed. Your game is over. ***", me.name());
            break;
        }
    }
}

fn advance(w: &mut WorldState, queued: &mut Vec<Command>, months: usize) {
    let me = w.player.unwrap();
    for i in 0..months {
        let cmds = if i == 0 { std::mem::take(queued) } else { vec![] };
        let headlines = tick_month(w, &cmds);
        for h in &headlines {
            println!("  [{}] {}", w.date_str(), h);
        }
        if !w.nation(me).alive {
            return;
        }
        // History does not wait politely for your turn to end: a war or a
        // collapse cuts a long advance short so you can actually respond to it.
        if i + 1 < months {
            if let Some(event) = headlines.iter().find(|h| is_major(h, me)) {
                println!("\n  ** {} **", event.trim_end_matches('.'));
                println!("  The world stops for you. ({} months left unrun.)", months - i - 1);
                break;
            }
        }
    }
    briefing(w, me);
}

/// The conflict a player means when they name one — by theatre, or by the other
/// side's name, or by nothing at all when there is only one.
fn find_conflict(w: &WorldState, me: NationId, hint: &str) -> Result<u32, String> {
    let mine: Vec<&Conflict> = w.conflicts.iter().filter(|c| c.involves(me)).collect();
    if mine.is_empty() {
        return Err("You are not party to any conflict.".into());
    }
    if hint.is_empty() {
        if mine.len() == 1 {
            return Ok(mine[0].id);
        }
        return Err(format!(
            "Which one? {}",
            mine.iter().map(|c| c.theatre.name()).collect::<Vec<_>>().join(", ")
        ));
    }
    let want_th = spheres_sim::theatre::TheatreId::parse(hint);
    let want_n = NationId::parse(hint);
    mine.iter()
        .find(|c| {
            Some(c.theatre) == want_th || want_n.map_or(false, |n| c.involves(n))
        })
        .map(|c| c.id)
        .ok_or_else(|| format!("No conflict of yours matches '{}'.", hint))
}

/// `commit 8`, `commit gulf 8`, `objective hold`, `roe unrestricted`,
/// `ceiling 5`, `redline 0.3`. The theatre is optional whenever it is obvious.
fn ladder_command(w: &WorldState, me: NationId, verb: &str, rest: &str) -> Result<Command, String> {
    let parts: Vec<&str> = rest.split_whitespace().collect();
    let (hint, arg) = match parts.len() {
        0 => return Err(format!("Usage: {} <value>   (or: {} <theatre> <value>)", verb, verb)),
        1 => ("", parts[0]),
        _ => (parts[0], parts[1]),
    };
    let conflict = find_conflict(w, me, hint)?;
    Ok(match verb {
        "commit" => Command::SetCommitment {
            conflict,
            nation: me,
            rung: arg.parse::<u8>().map_err(|_| "Rungs are 1 to 9.".to_string())?,
        },
        "ceiling" => Command::SetCeiling {
            conflict,
            nation: me,
            rung: arg.parse::<u8>().map_err(|_| "Rungs are 1 to 9.".to_string())?,
        },
        "objective" => Command::SetObjective {
            conflict,
            nation: me,
            objective: Objective::parse(arg)
                .ok_or("deny | degrade | seize | hold | stabilise | withdraw")?,
        },
        "roe" => Command::SetRoE {
            conflict,
            nation: me,
            roe: Roe::parse(arg).ok_or("restrained | standard | unrestricted")?,
        },
        _ => Command::SetRedLine {
            conflict,
            nation: me,
            resolve_floor: arg.parse::<f64>().map_err(|_| "A resolve floor, 0 to 0.95.".to_string())?,
        },
    })
}

/// `access request Turkey levant`, `access grant USA gulf`, `access press ...`,
/// `access revoke ...`.
fn access_command(w: &WorldState, me: NationId, rest: &str) -> Result<Command, String> {
    let p: Vec<&str> = rest.split_whitespace().collect();
    if p.len() < 2 {
        return Err("Usage: access request|press|grant|refuse|revoke <nation> [theatre]".into());
    }
    let other = NationId::parse(p[1]).ok_or_else(|| format!("Who is '{}'?", p[1]))?;
    let theatre = match p.get(2) {
        Some(t) => spheres_sim::theatre::TheatreId::parse(t)
            .ok_or_else(|| format!("Where is '{}'?", t))?,
        None => w
            .conflicts
            .iter()
            .find(|c| c.involves(me))
            .map(|c| c.theatre)
            .ok_or("Name a theatre — you are in no conflict to infer one from.")?,
    };
    Ok(match p[0] {
        "request" | "ask" => Command::RequestAccess { seeker: me, host: other, theatre },
        "press" => Command::PressForAccess { seeker: me, host: other, theatre },
        "grant" => Command::GrantAccess { host: me, seeker: other, theatre, grant: true },
        "refuse" | "deny" => Command::GrantAccess { host: me, seeker: other, theatre, grant: false },
        "revoke" => Command::RevokeAccess { host: me, seeker: other, theatre },
        _ => return Err("request | press | grant | refuse | revoke".into()),
    })
}

/// `join Kuwait` — take the side the named nation is on, in the conflict it is
/// fighting. Naming a belligerent rather than a side or an id is the only form
/// a player thinks in: you do not side with "side B", you side with Kuwait.
fn join_command(w: &WorldState, me: NationId, rest: &str) -> Result<Command, String> {
    let who = NationId::parse(rest.trim())
        .ok_or_else(|| "Usage: join Kuwait   (whoever you mean to stand beside)".to_string())?;
    let c = w
        .conflicts
        .iter()
        .find(|c| c.involves(who))
        .ok_or_else(|| format!("{} is not in anybody's conflict.", who.name()))?;
    if c.involves(me) {
        return Err("You are already a party to that conflict.".into());
    }
    Ok(Command::JoinConflict {
        conflict: c.id,
        nation: me,
        side_a: c.side_of(who) == Some(true),
        objective: Objective::Deny,
    })
}

fn describe_ladder(c: &Command) -> String {
    match c {
        Command::JoinConflict { side_a, .. } => format!(
            "take the {} side, at rung 1 — the ladder starts where everybody else's does",
            if *side_a { "attacking" } else { "defending" }
        ),
        Command::SetCommitment { rung, .. } => {
            format!("rung {} — {}", rung, spheres_sim::world::rung_name(*rung))
        }
        Command::SetCeiling { rung, .. } => format!("a public ceiling at rung {}", rung),
        Command::SetObjective { objective, .. } => format!("objective: {}", objective.label()),
        Command::SetRoE { roe, .. } => format!("rules of engagement: {}", roe.label()),
        Command::SetRedLine { resolve_floor, .. } => format!("red line at resolve {:.2}", resolve_floor),
        Command::RequestAccess { host, theatre, .. } => {
            format!("ask {} for basing in {}", host.name(), theatre.name())
        }
        Command::PressForAccess { host, theatre, .. } => {
            format!("press {} for basing in {}", host.name(), theatre.name())
        }
        Command::GrantAccess { seeker, grant, theatre, .. } => format!(
            "{} {} the use of our bases in {}",
            if *grant { "grant" } else { "refuse" }, seeker.name(), theatre.name()
        ),
        Command::RevokeAccess { seeker, theatre, .. } => {
            format!("revoke {}'s basing rights in {}", seeker.name(), theatre.name())
        }
        _ => format!("{:?}", c),
    }
}

/// Events worth interrupting a multi-month advance for.
fn is_major(headline: &str, me: NationId) -> bool {
    let h = headline.to_lowercase();
    let structural = h.starts_with("war:")
        || h.contains("dissolved")
        || h.contains("has annexed")
        || h.contains("capitulates")
        || h.contains("revolution in")
        || h.contains("repels")
        || h.contains("escalates to rung")
        || h.contains("grants")
        || h.contains("revokes");
    // Anything naming you is your business, whoever it happened to.
    structural || h.contains(&me.name().to_lowercase())
}

fn briefing(w: &WorldState, me: NationId) {
    let n = w.nation(me);
    println!("\n--- {} — {} ---", me.name(), w.date_str());
    println!(
        "GDP ${:.0}bn ({:+.1}%/yr)   Inflation {:.1}%   Rate {:.1}%   Debt {:.0}% GDP",
        n.gdp, n.growth_last * 100.0, n.inflation * 100.0,
        n.interest_rate * 100.0, n.debt_gdp * 100.0
    );
    println!(
        "Tax {:.0}%   Mil spend {:.1}% (str {:.0})   State invest {:.1}%   Stability {:.0}/100",
        n.tax_rate * 100.0, n.mil_spend_gdp * 100.0, n.mil_strength,
        n.state_invest_gdp * 100.0, n.stability
    );
    println!(
        "Political capital {:.0}/100 — what you can still spend on what they will not thank you for.",
        n.political_capital
    );
    {
        use spheres_sim::government as gov;
        if let Some(g) = gov::state(w, me) {
            if gov::is_electoral(w, me) {
                let lead = g.leader().and_then(|l| {
                    gov::polity(me)
                        .and_then(|p| p.parties.iter().find(|s| s.id == l))
                        .map(|s| s.name)
                });
                let strain = gov::strain(w, me);
                println!(
                    "Government: {} - {} part{}, {:.0}% of the chamber{}   ('government' for detail)",
                    lead.unwrap_or("none formed"),
                    g.coalition.len(),
                    if g.coalition.len() == 1 { "y" } else { "ies" },
                    g.government_seats() * 100.0,
                    if strain > 0.0 {
                        format!(", costing {:.1} pc a month", gov::upkeep(w, me))
                    } else {
                        String::new()
                    }
                );
            } else if let Some((pillar, loyalty)) = g.weakest_pillar() {
                println!(
                    "Regime: {} loyalty {:.0}%, coup pressure {:.0}%   ('government' for detail)",
                    pillar.key(),
                    loyalty * 100.0,
                    g.coup_pressure * 100.0
                );
            }
        }
    }
    if n.war_exhaustion > 0.01 {
        println!("War exhaustion: {:.0}%", n.war_exhaustion * 100.0);
    }
    let sanctioners = w.sanctions.iter().filter(|(_, t)| *t == me).count();
    if sanctioners > 0 {
        println!("Under sanction by {} nations.", sanctioners);
    }
    println!("Oil ${:.0}/bbl", w.oil_price);
    for c in &w.conflicts {
        let mark = if c.involves(me) { " <-- YOU" } else { "" };
        // One line per conflict: where, who, how far each side has climbed, who
        // holds the ground, and what is left of the will to go on.
        let mine = c.posture_of(me);
        let theirs = c
            .posture
            .iter()
            .find(|b| c.side_of(b.nation) != c.side_of(me));
        let ladder = match (mine, theirs) {
            (Some(a), Some(b)) => format!(
                " · you rung {} ({}, {}) vs {} rung {} · resolve {:.2} vs {:.2}",
                a.rung, a.objective.label(), a.roe.label(),
                b.nation.name(), b.rung, a.resolve, b.resolve
            ),
            _ => format!(
                " · rung {} vs {}",
                c.top_rung(true), c.top_rung(false)
            ),
        };
        println!(
            "{}: {} vs {} · ground {:+.2}{}{}",
            c.theatre.name().to_uppercase(),
            c.attacker().name(), c.defender().name(), c.control, ladder, mark
        );
    }
}

fn month_name(m: u32) -> &'static str {
    const M: [&str; 12] = ["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"];
    M[((m.max(1) - 1) % 12) as usize]
}

/// The whole system in one screen: who your parties are, what the country
/// currently thinks of them, who sits in your cabinet, what that cabinet is
/// costing you, and what you can do about it. A mechanic the player cannot
/// reach from their seat is not a mechanic.
fn government_view(w: &WorldState, me: NationId) {
    use spheres_sim::government as gov;
    let g = match gov::state(w, me) {
        Some(g) => g,
        None => {
            println!("  {} has no organised politics in this model.", me.name());
            return;
        }
    };
    let pol = match gov::polity(me) {
        Some(p) => p,
        None => return,
    };
    println!("\n--- Government of {} - {} ---", me.name(), w.date_str());

    if gov::is_electoral(w, me) {
        let (ey, em) = g.next_election;
        println!(
            "{}.   Next election: {}",
            pol.system.label(),
            if ey > 0 { format!("{} {}", month_name(em), ey) } else { "not yet set".to_string() }
        );
        println!();
        println!("  {:<14} {:<42} {:>8} {:>7}  {}", "ID", "PARTY", "SUPPORT", "SEATS", "");
        let mut rows: Vec<&gov::PartySpec> = pol.parties.iter().collect();
        rows.sort_by(|a, b| {
            g.support_of(b.id)
                .partial_cmp(&g.support_of(a.id))
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        for s in rows {
            let mark = if g.leader() == Some(s.id) {
                "IN GOVERNMENT (leads)"
            } else if g.in_government(s.id) {
                "IN GOVERNMENT"
            } else if s.pariah {
                "nobody will govern with them"
            } else {
                ""
            };
            println!(
                "  {:<14} {:<42} {:>7.1}% {:>6.1}%  {}",
                s.id, s.name, g.support_of(s.id) * 100.0, g.seat_share(s.id) * 100.0, mark
            );
            if !s.native.is_empty() && s.native != s.name {
                println!("  {:<14} {} - {}", "", s.native, s.family.label());
            }
        }
        println!();
        let seats = g.government_seats();
        let strain = gov::strain(w, me);
        let names: Vec<&str> = g
            .coalition
            .iter()
            .filter_map(|p| pol.parties.iter().find(|s| s.id == *p).map(|s| s.name))
            .collect();
        println!(
            "  Cabinet: {}",
            if names.is_empty() { "none formed".to_string() } else { names.join(" + ") }
        );
        println!(
            "  It commands {:.0}% of the chamber{}.",
            seats * 100.0,
            if seats < 0.5 { ", a minority - and every vote has to be bought separately" } else { "" }
        );
        if strain > 0.0 {
            println!(
                "  Holding it together costs {:.1} political capital a month, and {:.0} off your ceiling.",
                gov::upkeep(w, me),
                strain * 1.8
            );
        } else {
            println!("  It costs you nothing to hold. This is as free as governing gets.");
        }
        println!();
        println!("  invite <id>   bring a party in       expel <id>   throw one out");
        println!("  election      go back to the country early (25 pc)");
    } else {
        println!("Power here is not voted on. It rests on {}.", pol.ruling);
        println!();
        println!("  {:<10} {:<46} {:>8}", "ID", "INSTITUTION", "LOYALTY");
        for (pillar, loyalty) in &g.pillars {
            let name = pol
                .pillars
                .iter()
                .find(|s| s.pillar == *pillar)
                .map(|s| s.name)
                .unwrap_or("");
            let mood = if *loyalty < 0.35 {
                "   <-- they are not being paid"
            } else if *loyalty < 0.5 {
                "   restive"
            } else {
                ""
            };
            println!("  {:<10} {:<46} {:>7.0}%{}", pillar.key(), name, loyalty * 100.0, mood);
        }
        println!();
        println!(
            "  Coup pressure: {:.0}% of the way to somebody acting on it.",
            (g.coup_pressure * 100.0).min(100.0)
        );
        println!(
            "  What the loyalty you have bought is worth: {:+.1} on your political capital ceiling.",
            gov::standing_modifier(w, me)
        );
        println!();
        println!("  secure <id>   pay an institution to stay loyal (14 pc, and it goes on the debt)");
        println!("  The army reads your defence budget too, and the merchants read your prices.");
    }
}

fn help() {
    println!("Commands:");
    println!("  next / n          advance one month");
    println!("  year / y          advance twelve months");
    println!("  6                 advance six months (any number)");
    println!("  status            your nation briefing");
    println!("  world             global league table");
    println!("  relations         your diplomatic standing");
    println!("  rate 6.5          set policy interest rate (%)");
    println!("  tax 30            set tax take (% of GDP)");
    println!("  military 4.5      set military spending (% of GDP)");
    println!("  invest 8          set state investment (% of GDP)");
    println!("  improve China     diplomatic push (+relations)");
    println!("  sanction Iraq     impose sanctions");
    println!("  lift Iraq         lift sanctions");
    println!("  government / gov  your parties, your coalition, and what it costs");
    println!("  invite <party>    bring a party into your cabinet");
    println!("  expel <party>     throw one out");
    println!("  election          go back to the country early");
    println!("  secure army       (unelected regimes) pay an institution to stay loyal");
    println!("  options           what the world is offering you now");
    println!("  enact <id>        take one of those options");
    println!("  war Kuwait        declare war (confirmed)");
    println!("  war Kuwait        declare war outright — rung 8 in one act (confirmed)");
    println!("  -- the commitment ladder --");
    println!("  quarrel Iraq      open a quarrel at rung 1, and climb from there");
    println!("  join Kuwait       take a side in a conflict already running");
    println!("  commit 8          stand on rung 8 in your only conflict");
    println!("  commit gulf 6     ...or name the theatre when you have several");
    println!("  objective hold    deny|degrade|seize|hold|stabilise|withdraw");
    println!("  roe restrained    restrained|standard|unrestricted");
    println!("  ceiling 5         publicly rule out going any further");
    println!("  redline 0.3       withdraw by itself when resolve falls this far");
    println!("  access request Turkey levant   ask a third state for basing");
    println!("  access grant USA gulf          answer somebody who is asking you");
    println!("  save / quit");
}

fn read_line() -> String {
    let mut s = String::new();
    io::stdin().lock().read_line(&mut s).unwrap_or(0);
    s.trim().to_string()
}
