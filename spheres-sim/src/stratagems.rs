//! Stratagems — what a government can *decide to do*, and the replacement for
//! national focus trees.
//!
//! BIBLE section 5 refuses focus trees forever, and names this as the thing that
//! has to be good enough to defend that refusal. The distinction it has to earn:
//!
//! A focus tree is a **rail**. The same 1990s happen every run, and it *runs
//! out* — the most cited complaint about Millennium Dawn is that the world goes
//! quiet at year ten or fifteen when the tree is spent.
//!
//! A stratagem is an **option the world offers you because of the state it is
//! in**. It is on the table because your debt is crushing you, or because
//! inflation is running, or because you are poor and open and somebody will buy
//! what you make. It cannot run out, because the world keeps generating
//! conditions. Nothing here is scheduled and nothing is named after a country:
//! every one of these was reached for by several governments in this period, and
//! any nation that finds itself in the same position can reach for it too.
//!
//! Every one costs political capital, and every one has a real cost beyond the
//! price. A stratagem with only an upside is a button, not a decision.

use crate::world::*;

/// One thing a government can decide to do, when the world lets it.
pub struct Stratagem {
    /// Stable identifier. This is what a command carries and what a save would
    /// write, so it must never be renamed once shipped.
    pub id: &'static str,
    pub name: &'static str,
    /// What it does and what it costs you, in the language a briefing would use.
    pub blurb: &'static str,
    /// Why the world is offering it — shown to the player, because a decision
    /// you do not understand the basis of is a lottery.
    pub because: &'static str,
    /// Political capital. The good ones hurt.
    pub cost: f64,
    /// Derived from world state. Never a date, never a nation.
    pub available: fn(&WorldState, NationId) -> bool,
    pub enact: fn(&mut WorldState, NationId),
}

fn n_of(w: &WorldState, id: NationId) -> &Nation {
    w.nation(id)
}

/// Development, the same proxy the growth model uses.
fn dev(w: &WorldState, id: NationId) -> f64 {
    let n = n_of(w, id);
    (n.gdp * 1000.0 / n.population.max(0.001) / 24000.0).min(1.0)
}

pub const DECK: &[Stratagem] = &[
    // Poland's Balcerowicz plan, Jan 1990. Russia 1992. The bargain is always
    // the same: break the inflation, and take the slump that comes with it.
    Stratagem {
        id: "shock_therapy",
        name: "Shock Therapy",
        blurb: "Free prices, end the subsidies, take the recession. Inflation \
                breaks; output and legitimacy break with it.",
        because: "A command economy that has stopped delivering, or prices out of control",
        cost: 42.0,
        available: |w, id| {
            let n = n_of(w, id);
            // Nobody dismantles a system that is working. China grew 8% a year
            // through this window and pointedly did NOT do this; Poland and
            // Russia did it because the plan had stopped producing. The gate is
            // therefore failure, not merely central planning — which is also
            // what stops the AI throwing away a growth miracle it is in the
            // middle of.
            !w.has_flag(&format!("shock_{:?}", id))
                && ((n.system == EconomySystem::Command && n.growth_last < 0.02)
                    || n.inflation > 0.30)
        },
        enact: |w, id| {
            w.set_flag(&format!("shock_{:?}", id));
            let n = w.nation_mut(id);
            n.system = EconomySystem::Market;
            n.inflation *= 0.35;
            n.gdp *= 0.88;
            n.stability = (n.stability - 14.0).max(0.0);
            n.priv_invest_gdp = (n.priv_invest_gdp + 0.03).min(0.40);
            let name = id.name();
            w.headline(format!("{} frees prices and takes the slump.", name));
        },
    },
    // Argentina 1991, Bulgaria 1997, and the thing Yugoslavia's Markovic
    // programme was in January 1990. It works until the fundamentals disagree.
    Stratagem {
        id: "currency_peg",
        name: "Peg the Currency",
        blurb: "Tie the currency to a hard one and import its credibility. \
                Inflation falls fast. You give up the rate as a tool.",
        because: "Inflation above 15% with a central bank nobody believes",
        cost: 26.0,
        available: |w, id| {
            let n = n_of(w, id);
            n.inflation > 0.15 && !w.has_flag(&format!("peg_{:?}", id))
        },
        enact: |w, id| {
            w.set_flag(&format!("peg_{:?}", id));
            // Pegging is a monetary decision, so it counts as governing the
            // rate: without this the default bank in politics.rs — which runs
            // for a player who has not touched monetary policy — would drift
            // the peg away within months, and the player would have paid 26
            // political capital for nothing. Measured before the latch was
            // added here: pinned at 0.055, back to 0.078 six months later.
            if Some(id) == w.player {
                w.player_set_rate = true;
            }
            let n = w.nation_mut(id);
            n.inflation = n.inflation.min(0.06);
            // The rate is no longer yours to set, so it stops floating with you.
            n.interest_rate = 0.055;
            n.stability = (n.stability + 5.0).min(100.0);
            let name = id.name();
            w.headline(format!("{} pegs its currency and imports somebody else's credibility.", name));
        },
    },
    // Korea and Taiwan did it before the window; China, Vietnam, Indonesia and
    // Turkey were doing it inside it. Cheap only if you are still poor.
    Stratagem {
        id: "export_led",
        name: "Export-Led Industrialisation",
        blurb: "Point the whole economy outward: undervalue, subsidise the \
                exporters, suppress consumption. Growth, bought from your own \
                households.",
        because: "Still poor enough for the world to want what you can make",
        cost: 30.0,
        available: |w, id| dev(w, id) < 0.55 && !w.has_flag(&format!("export_{:?}", id)),
        enact: |w, id| {
            w.set_flag(&format!("export_{:?}", id));
            let n = w.nation_mut(id);
            // Calibrated down from +6pp of investment and +0.4pp of trend, which
            // compounded a China already growing 8% a year into 14.5x over three
            // decades against a real ~9x. A stratagem should bend a trajectory,
            // never manufacture one.
            n.priv_invest_gdp = (n.priv_invest_gdp + 0.025).min(0.40);
            n.tech.tfp_base += 0.0015;
            // Suppressed consumption is a real cost paid by real people.
            n.stability = (n.stability - 6.0).max(0.0);
            let name = id.name();
            w.headline(format!("{} turns its economy outward.", name));
        },
    },
    // The IMF programme, and what it costs a government to sign one.
    Stratagem {
        id: "austerity",
        name: "Fiscal Consolidation",
        blurb: "Raise the taxes, cut the spending, stop the debt. The arithmetic \
                works. The government that does it usually loses.",
        because: "Public debt above 90% of output",
        cost: 34.0,
        available: |w, id| n_of(w, id).debt_gdp > 0.90,
        enact: |w, id| {
            let n = w.nation_mut(id);
            n.tax_rate = (n.tax_rate + 0.05).min(0.60);
            n.state_invest_gdp = (n.state_invest_gdp * 0.7).max(0.02);
            n.mil_spend_gdp = (n.mil_spend_gdp * 0.8).max(0.005);
            n.stability = (n.stability - 9.0).max(0.0);
            let name = id.name();
            w.headline(format!("{} announces an austerity budget.", name));
        },
    },
    // Brady bonds, the Paris Club, and Egypt's reward for joining the coalition.
    // Your creditors are the people whose opinion of you it costs.
    Stratagem {
        id: "debt_restructuring",
        name: "Restructure the Debt",
        blurb: "Go to the creditors and write it down. The burden lifts. Every \
                capital that holds your paper remembers.",
        because: "Debt beyond what any budget can service",
        cost: 30.0,
        available: |w, id| n_of(w, id).debt_gdp > 1.10,
        enact: |w, id| {
            {
                let n = w.nation_mut(id);
                n.debt_gdp *= 0.55;
                n.stability = (n.stability + 4.0).min(100.0);
            }
            for m in crate::nations::majors().iter().copied() {
                if m != id {
                    w.shift_relation(m, id, -8.0);
                }
            }
            let name = id.name();
            w.headline(format!("{} restructures its external debt; its creditors take the loss.", name));
        },
    },
    // The Soviet republics, the Warsaw Pact states, and Britain before them.
    Stratagem {
        id: "mass_privatisation",
        name: "Mass Privatisation",
        blurb: "Sell or give away the state's industry. Productivity rises \
                eventually. The people who lose their jobs notice first.",
        because: "A state that owns its industry and cannot afford to run it",
        cost: 36.0,
        available: |w, id| {
            let n = n_of(w, id);
            n.state_invest_gdp > 0.08 && !w.has_flag(&format!("privatised_{:?}", id))
        },
        enact: |w, id| {
            w.set_flag(&format!("privatised_{:?}", id));
            let n = w.nation_mut(id);
            n.state_invest_gdp = (n.state_invest_gdp * 0.45).max(0.02);
            n.priv_invest_gdp = (n.priv_invest_gdp + 0.04).min(0.40);
            n.tech.tfp_base += 0.003;
            n.stability = (n.stability - 8.0).max(0.0);
            n.debt_gdp = (n.debt_gdp - 0.08).max(0.0);
            let name = id.name();
            w.headline(format!("{} sells the state's industry.", name));
        },
    },
    // What an autocrat does instead of an election, and what it costs abroad.
    Stratagem {
        id: "security_crackdown",
        name: "Security Crackdown",
        blurb: "Put the apparatus on the street. Order returns. Everyone who \
                cares about how you govern stops pretending they do not notice.",
        because: "A restive population and a government willing to coerce it",
        cost: 20.0,
        available: |w, id| {
            let n = n_of(w, id);
            n.stability < 55.0 && n.authoritarianism > 0.35
        },
        enact: |w, id| {
            {
                let n = w.nation_mut(id);
                n.stability = (n.stability + 16.0).min(100.0);
                n.separatism = (n.separatism - 0.10).max(0.0);
                n.authoritarianism = (n.authoritarianism + 0.06).min(0.98);
            }
            let democracies: Vec<NationId> = w
                .nations
                .iter()
                .filter(|x| x.alive && x.authoritarianism < 0.30 && x.id != id)
                .map(|x| x.id)
                .collect();
            for d in democracies {
                w.shift_relation(d, id, -6.0);
            }
            let name = id.name();
            w.headline(format!("{} moves against its own streets.", name));
        },
    },
    // Gorbachev's problem exactly: the thing that buys you the world's goodwill
    // is the thing that lets your own union come apart.
    Stratagem {
        id: "liberalisation",
        name: "Political Liberalisation",
        blurb: "Open it up. The world warms to you and your own people find \
                their voice — including the ones who never wanted to be ruled \
                by you.",
        because: "An authoritarian state under pressure it cannot simply crush",
        cost: 28.0,
        available: |w, id| n_of(w, id).authoritarianism > 0.45,
        enact: |w, id| {
            {
                let n = w.nation_mut(id);
                n.authoritarianism = (n.authoritarianism - 0.18).max(0.05);
                n.stability = (n.stability + 6.0).min(100.0);
                // Opening up is how a union discovers what it was holding down.
                n.separatism = (n.separatism + 0.12).min(1.0);
            }
            let democracies: Vec<NationId> = w
                .nations
                .iter()
                .filter(|x| x.alive && x.authoritarianism < 0.30 && x.id != id)
                .map(|x| x.id)
                .collect();
            for d in democracies {
                w.shift_relation(d, id, 10.0);
            }
            let name = id.name();
            w.headline(format!("{} opens up.", name));
        },
    },
    // A professional army costs money and buys quality; conscription costs
    // legitimacy and buys mass. Most of Europe made this choice in the period.
    Stratagem {
        id: "professional_army",
        name: "End Conscription",
        blurb: "Trade a large unwilling army for a small willing one. It costs \
                more per soldier and the country stops resenting you for it.",
        because: "An army built on conscripts and a budget that could pay them",
        cost: 22.0,
        available: |w, id| {
            let n = n_of(w, id);
            n.mil_spend_gdp > 0.02 && dev(w, id) > 0.35 && !w.has_flag(&format!("prof_{:?}", id))
        },
        enact: |w, id| {
            w.set_flag(&format!("prof_{:?}", id));
            let n = w.nation_mut(id);
            n.mil_spend_gdp = (n.mil_spend_gdp * 1.15).min(0.35);
            n.mil_strength *= 1.12;
            n.stability = (n.stability + 7.0).min(100.0);
            let name = id.name();
            w.headline(format!("{} ends conscription.", name));
        },
    },
    // The thing every capital says it is not doing until the day it tests.
    Stratagem {
        id: "nuclear_programme",
        name: "Pursue the Bomb",
        blurb: "Start the programme. Nobody invades a nuclear power. Everybody \
                treats you differently long before you have one.",
        because: "A state with the industry to try and reason to want it",
        cost: 48.0,
        available: |w, id| {
            let n = n_of(w, id);
            !n.nuclear && n.gdp > 90.0 && !w.has_flag(&format!("nuke_prog_{:?}", id))
        },
        enact: |w, id| {
            w.set_flag(&format!("nuke_prog_{:?}", id));
            {
                let n = w.nation_mut(id);
                n.state_invest_gdp = (n.state_invest_gdp + 0.01).min(0.40);
            }
            for m in crate::nations::majors().iter().copied() {
                if m != id {
                    w.shift_relation(m, id, -14.0);
                }
            }
            let name = id.name();
            w.headline(format!("{} is believed to have begun a weapons programme.", name));
        },
    },
];

/// What this nation could decide to do this month, in deck order.
pub fn available(w: &WorldState, id: NationId) -> Vec<&'static Stratagem> {
    if w.nation_opt(id).map_or(true, |n| !n.alive) {
        return vec![];
    }
    DECK.iter().filter(|s| (s.available)(w, id)).collect()
}

pub fn by_id(id: &str) -> Option<&'static Stratagem> {
    DECK.iter().find(|s| s.id == id)
}

/// The AI reaches for the same deck on the same terms, or the world does not
/// move and the player is the only government in it that ever decides anything.
///
/// Deliberately conservative: a government spends most of a term's standing on
/// one of these, so it acts when the condition that opened the option is acute
/// rather than merely present, and never while it has other bills to pay.
pub fn ai_stratagems(w: &mut WorldState) {
    let actors: Vec<NationId> = w
        .nations
        .iter()
        .filter(|n| n.alive && Some(n.id) != w.player)
        .map(|n| n.id)
        .collect();
    for id in actors {
        let held = w.nation(id).political_capital;
        // Keep a reserve. A government that spends to the floor cannot answer
        // the next crisis, and the crisis is what stratagems are for.
        if held < 55.0 {
            continue;
        }
        let options = available(w, id);
        if options.is_empty() {
            continue;
        }
        // Deck order is the priority order, so the choice is deterministic:
        // the first thing it can afford that its condition genuinely calls for.
        let choice = options.iter().find(|s| s.cost <= held - 20.0).map(|s| s.id);
        if let Some(sid) = choice {
            // Rare, because these are decisions of a whole term, not a month.
            if w.rng.chance(0.02) {
                let cmd = crate::Command::EnactStratagem {
                    nation: id,
                    id: sid.to_string(),
                };
                let _ = crate::apply_command(w, &cmd);
            }
        }
    }
}

/// A weapons programme is not a bomb. It takes years, and the world is watching
/// the whole time — which is the cost that makes the stratagem a decision rather
/// than a purchase.
pub fn tick(w: &mut WorldState) {
    let seekers: Vec<NationId> = w
        .nations
        .iter()
        .filter(|n| n.alive && !n.nuclear)
        .map(|n| n.id)
        .filter(|id| w.has_flag(&format!("nuke_prog_{:?}", id)))
        .collect();
    for id in seekers {
        // Roughly a decade of sustained effort, and only while the state holds
        // together well enough to run a programme at all.
        let ready = w.nation(id).stability > 35.0 && w.rng.chance(0.008);
        if ready {
            w.nation_mut(id).nuclear = true;
            w.headline(format!("{} tests a nuclear device.", id.name()));
        }
    }
}
