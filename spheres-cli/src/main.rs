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
    println!("Oil: ${:.0}/bbl   Wars: {}   Sanction pairs: {}", w.oil_price, w.wars.len(), w.sanctions.len());
}

fn play(seed: u64) {
    println!("=== SPHERES — January 1990 ===\n");
    println!("Playable nations:");
    for id in start_nations() {
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
                let mut rels: Vec<(NationId, f64)> = all_nations()
                    .iter()
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

/// Events worth interrupting a multi-month advance for.
fn is_major(headline: &str, me: NationId) -> bool {
    let h = headline.to_lowercase();
    let structural = h.starts_with("war:")
        || h.contains("dissolved")
        || h.contains("has annexed")
        || h.contains("capitulates")
        || h.contains("revolution in")
        || h.contains("repels");
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
    for war in &w.wars {
        let mark = if war.involves(me) { " <-- YOU" } else { "" };
        println!(
            "WAR: {} vs {} (progress {:+.0}){}",
            war.attacker.name(), war.defender.name(), war.progress, mark
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
    println!("  save / quit");
}

fn read_line() -> String {
    let mut s = String::new();
    io::stdin().lock().read_line(&mut s).unwrap_or(0);
    s.trim().to_string()
}
