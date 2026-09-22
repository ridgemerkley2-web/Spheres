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
    /// Shown to the player. Free to reword; unlike `id`, nothing keys off it.
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
    /// What taking it does to the world. Runs through the command queue's
    /// pricing like any other act, so a stratagem is never a free lunch.
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

/// The legacy package keeps its existing proportional cuts when a detailed
/// plan owns expenditure. The package's one political price buys this change;
/// dispatch preserves department receipts, already granted funds and cash.
fn legacy_consolidation_budget(w: &WorldState, id: NationId) -> Option<(crate::Command, bool)> {
    let n = w.nation(id);
    if n.annual_budget.is_none() && n.program_budget.is_none() {
        return None;
    }
    let current = n.budget_for(w.year);
    let mut allocations = current.allocations;
    let investment = current.investment_total();
    if investment > 0.0 {
        let reduced = (investment * 0.7).max(0.02_f64.min(investment));
        for ministry in [BUDGET_INFRASTRUCTURE, BUDGET_INDUSTRY, BUDGET_SCIENCE] {
            allocations[ministry] *= reduced / investment;
        }
    }
    let defense = current.defense();
    allocations[BUDGET_DEFENSE] = (defense * 0.8).max(0.005_f64.min(defense));
    let changed = allocations != current.allocations;
    let command = if let Some(program) = &n.program_budget {
        crate::Command::SetProgramBudget {
            nation: id, fiscal_year: w.year, allocations, departments: program.departments,
        }
    } else {
        crate::Command::SetAnnualBudget { nation: id, fiscal_year: w.year, allocations }
    };
    Some((command, changed))
}
fn consolidation_available(w: &WorldState, id: NationId) -> bool {
    if crate::fiscal_recovery::enabled(w) {
        crate::fiscal_recovery::assessment(w, id).is_some_and(|v| v.recovery_required)
            && (n_of(w, id).tax_rate < 0.60
                || crate::fiscal_recovery_ai::budget_adjustment(w, id, 0.20, 0.02).is_some())
    } else {
        n_of(w, id).debt_gdp > 0.90
            && legacy_consolidation_budget(w, id).map_or_else(
                || { let n = n_of(w, id); n.tax_rate < 0.60
                    || n.state_invest_gdp > 0.02 || n.mil_spend_gdp > 0.005 },
                |(command, changed)| {
                let allocations = match &command {
                    crate::Command::SetAnnualBudget { allocations, .. }
                    | crate::Command::SetProgramBudget { allocations, .. } => allocations,
                    _ => unreachable!(),
                };
                // SetAnnualBudget enforces these bounds at dispatch; quote
                // them here too so invalid old plans cannot partially enact.
                (n_of(w, id).tax_rate < 0.60 || changed)
                    && (!changed || (allocations.iter().enumerate().all(|(i, v)|
                    v.is_finite() && *v >= 0.0 && *v <= BUDGET_CAPS[i])
                    && allocations.iter().sum::<f64>() <= 0.70
                    && crate::world_refusal(w, &command).is_none()))
            })
    }
}

fn restructure_available(w: &WorldState, id: NationId) -> bool {
    n_of(w, id).debt_gdp > 1.10
        && (!crate::fiscal_recovery::enabled(w)
            || crate::fiscal_recovery_ai::restructuring_ready(w, id))
}

fn enact_consolidation(w: &mut WorldState, id: NationId) {
    if crate::fiscal_recovery::enabled(w) {
        // The Cabinet vote already pays the package's 34 PC. Dispatch its
        // constituent tax and budget changes through the ordinary state path
        // without charging the same vote again. Existing enrolled departments
        // retain their money and dated receipts; only future accrual changes.
        let budget = crate::fiscal_recovery_ai::budget_adjustment(w, id, 0.20, 0.02);
        let tax = (w.nation(id).tax_rate + 0.05).min(0.60);
        crate::dispatch(w, &crate::Command::SetTaxRate { nation: id, rate: tax })
            .expect("a finite consolidation tax is a valid ordinary command");
        if let Some(budget) = budget {
            crate::dispatch(w, &budget.command)
                .expect("the consolidation quote retains a valid current budget");
        }
    } else if let Some((budget, changed)) = legacy_consolidation_budget(w, id) {
        // Availability checked the plan owner's ordinary command authority.
        // The 34-PC package price is charged by the outer command exactly once.
        if changed {
            crate::dispatch(w, &budget)
                .expect("the legacy consolidation quote retains a valid owned budget");
        }
        let tax = (w.nation(id).tax_rate + 0.05).min(0.60);
        crate::dispatch(w, &crate::Command::SetTaxRate { nation: id, rate: tax })
            .expect("a finite consolidation tax is a valid ordinary command");
    } else {
        let n = w.nation_mut(id);
        n.tax_rate = (n.tax_rate + 0.05).min(0.60);
        // Floors limit cuts; an austerity vote cannot invent new expenditure.
        n.state_invest_gdp = (n.state_invest_gdp * 0.7).max(0.02_f64.min(n.state_invest_gdp));
        n.mil_spend_gdp = (n.mil_spend_gdp * 0.8).max(0.005_f64.min(n.mil_spend_gdp));
    }
    let n = w.nation_mut(id);
    n.stability = (n.stability - 9.0).max(0.0);
    if crate::fiscal_recovery::enabled(w) {
        w.headline(format!("{} enacts a fiscal consolidation budget.", id.name()));
    } else {
        w.headline(format!("{} announces an austerity budget.", id.name()));
    }
}

/// Every stratagem in the game, in a fixed order.
///
/// Fixed because the order decides what a player is offered when several are
/// available at once, and a shuffling menu is a non-deterministic one.
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
            crate::economy::refresh_debt_ratio(n);
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
            crate::agency::establish_peg(w,id,0.055);
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
        blurb: "Raise taxes by up to five points and trim discretionary spending. \
                Essential services and maintenance are protected in recovery campaigns. \
                The vote costs 9 stability; debt only improves when the budget delivers.",
        because: "A debt trajectory that requires recovery, with room for a fiscal adjustment (legacy campaigns: debt above 90%)",
        cost: 34.0,
        available: consolidation_available,
        enact: enact_consolidation,
    },
    // Brady bonds, the Paris Club, and Egypt's reward for joining the coalition.
    // Your creditors are the people whose opinion of you it costs.
    Stratagem {
        id: "debt_restructuring",
        name: "Restructure the Debt",
        blurb: "Write down 45% of the debt: creditors lose money and major-country \
                relations fall by 8. Recovery campaigns allow another restructuring \
                only after five years. Ongoing deficits still need a funded response.",
        because: "Debt beyond what any budget can service",
        cost: 30.0,
        available: restructure_available,
        enact: |w, id| {
            if crate::fiscal_recovery::enabled(w) {
                let today = crate::clock::absolute_day(w);
                w.fiscal_recovery.nations.entry(id).or_default().last_restructuring_day = Some(today);
            }
            {
                // THROUGH THE ONE FISCAL CHANNEL, and this leg reached it late
                // (routed 2026-09-02, after the merge). A write-down is money
                // the government stops owing, so it is a receipt -- a negative
                // `bn` -- and it goes through `economy::charge` like every
                // other money leg rather than pushing the ratio by hand.
                //
                // WHAT WAS WRONG WITH THE HAND-ROLLED FORM, measured: it moved
                // the ratio and left the stock alone, so for a government that
                // had opened its books the fiscal block's next pass recomputed
                // the ratio FROM the untouched stock and the restructuring was
                // erased inside a month. The creditors took the loss and the
                // debtor kept the debt.
                //
                // TWO REPRESENTATIONS OF THE SAME 45%. `bn` is the dollars a
                // government keeping a treasury stops owing. `share` is the
                // ratio move the shipped line made, and it is written as
                // `ratio * 0.55 - ratio` rather than `-0.45 * ratio` because
                // the closed-books arm writes `debt_gdp + share`, and only this
                // form reproduces the old scaling BIT FOR BIT: `0.55 * r` and
                // `r` are within a factor of two, so Sterbenz makes the
                // subtraction exact and the addition undoes it exactly. That
                // matters here and not only in principle -- `ai_stratagems`
                // reaches for this deck on the DEFAULT board, so a single ulp
                // would move the golden run.
                let ratio = n_of(w, id).debt_gdp;
                let gdp = n_of(w, id).gdp;
                let owed_bn = n_of(w, id).debt_bn.unwrap_or(ratio * gdp);
                let share = ratio * 0.55 - ratio;
                crate::economy::charge_for(w, id, -(owed_bn * 0.45), share, crate::fiscal_journal::CashCause::DebtRestructuring);
                let n = w.nation_mut(id);
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
            // The proceeds of the sale, through the same one channel and for
            // the same reason as the restructuring above: a ratio pushed by
            // hand is erased by the next fiscal pass for a government on the
            // books. `share` is `-0.08` and the closed arm writes
            // `(debt_gdp + share).max(0.0)`, which is the shipped line
            // character for character -- IEEE makes `r + (-0.08)` and
            // `r - 0.08` the same bits, and the floor is the same floor.
            //
            // ON THE BOOKS THE FLOOR STOPS DESTROYING THE REMAINDER, which is
            // the same repair the payee leg got: a state that raises 8% of
            // output selling its industry and owes less than that retires the
            // debt and BANKS the rest, where the ratio arithmetic could only
            // clamp at zero and lose it.
            let gdp = n_of(w, id).gdp;
            crate::economy::charge_for(w, id, -(0.08 * gdp), -0.08, crate::fiscal_journal::CashCause::AssetSale);
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
            // The political arm (S3): the same crackdown halves the foreign
            // money behind every non-ruling movement. Gated inside the arm,
            // which returns before reading anything with the arm off.
            crate::statecraft::halve_foreign_backing(w, id);
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
    if w.nation_opt(id).is_none_or(|n| !n.alive) {
        return vec![];
    }
    DECK.iter().filter(|s| (s.available)(w, id)).collect()
}

/// Why enacting a stratagem would be refused by the world, read without
/// touching it: the deck does not carry the id, or the condition that opened
/// the option has closed. The one place the prose lives: `dispatch` asks it
/// and the government screen serves it.
pub fn closed_reason(w: &WorldState, id: NationId, stratagem: &str) -> Option<String> {
    let s = match by_id(stratagem) {
        Some(s) => s,
        None => return Some(format!("No such stratagem: {}", stratagem)),
    };
    if stratagem == "debt_restructuring" && crate::fiscal_recovery::enabled(w)
        && !crate::fiscal_recovery_ai::restructuring_ready(w, id)
    {
        let last = w.fiscal_recovery.nations.get(&id).and_then(|n| n.last_restructuring_day).unwrap();
        let left = (last + crate::fiscal_recovery_ai::RESTRUCTURING_COOLDOWN_DAYS
            - crate::clock::absolute_day(w)).max(0);
        return Some(format!("Creditors will not accept another restructuring for {left} days."));
    }
    if !(s.available)(w, id) {
        return Some(format!("{} is no longer open to {}.", s.name, id.name()));
    }
    None
}

/// Look a stratagem up by its stable `id`, or `None` if nothing carries it.
///
/// Saves store the id, so this is also the load path — which is why an id must
/// never be renamed once it has shipped.
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
    for id in actors.iter().copied() {
        let held = w.nation(id).political_capital;
        let lever = crate::government::ai_lever(w, id);
        // Routine cards keep their standing reserve. A legally available
        // political lever already checks its own price: the 30-point round
        // table must not be silently replaced by a 55-point dispatch gate.
        // With the political layer off, ai_lever is None and this is exactly
        // the previous card gate, including its random-draw behavior.
        if held < 55.0 && lever.is_none() {
            continue;
        }
        let options: Vec<_> = available(w, id).into_iter().filter(|s|
            !crate::fiscal_recovery::enabled(w)
                || !matches!(s.id, "austerity" | "debt_restructuring")
        ).collect();
        // The political arm's five levers (S3) ride THIS draw — the design's
        // "on the existing 0.02 monthly stratagem draw" — so the government
        // module keeps drawing nothing and a month holds one decision, a
        // lever or a card, never both. `government::ai_lever` is pure and
        // answers `None` before reading anything with the arm off, so the
        // off world's stream is untouched, and a switched-on world parts from
        // it only in a month a government has a lever and no card (the same
        // finding as the sponsors' draw in `politics`). The lever comes
        // first because its conditions are the narrower crisis.
        if options.is_empty() && lever.is_none() {
            continue;
        }
        // Deck order is the priority order, so the choice is deterministic:
        // the first thing it can afford that its condition genuinely calls for.
        let choice = options.iter().find(|s| s.cost <= held - 20.0).map(|s| s.id);
        let cmd = match (lever, choice) {
            (Some(cmd), _) => cmd,
            (None, Some(sid)) => crate::Command::EnactStratagem {
                nation: id,
                id: sid.to_string(),
            },
            (None, None) => continue,
        };
        // Rare, because these are decisions of a whole term, not a month.
        if w.rng.chance(crate::clock::chance(w, 0.02)) {
            let _ = crate::apply_command(w, &cmd);
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
        let ready = w.nation(id).stability > 35.0 && w.rng.chance(crate::clock::chance(w, 0.008));
        if ready {
            w.nation_mut(id).nuclear = true;
            w.headline(format!("{} tests a nuclear device.", id.name()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{government, init::world_1990, world::GameRules};

    #[test]
    fn an_affordable_round_table_reaches_the_real_ai_dispatcher() {
        let id = NationId::Albania;
        let mut w = world_1990(GameRules { ideology_blocs: true, seed: 7, ..Default::default() });
        // Isolate this government's decision opportunities. No monthly
        // standing recovery or other country's draw can pay the bill for it.
        for n in &mut w.nations { n.alive = n.id == id; }
        w.nation_mut(id).political_capital = government::ROUND_TABLE_PC;
        assert!(!government::is_electoral(&w, id));
        assert!(matches!(government::ai_lever(&w, id), Some(crate::Command::ConveneRoundTable { .. })));

        let mut off = w.clone();
        off.rules.ideology_blocs = false;
        let before = crate::save(&off);
        for _ in 0..200 { ai_stratagems(&mut off); }
        assert_eq!(crate::save(&off), before, "disabled levers consume neither standing nor random draws");
        let mut poor = w.clone();
        poor.nation_mut(id).political_capital = government::ROUND_TABLE_PC - 1.0;
        let before = crate::save(&poor);
        for _ in 0..200 { ai_stratagems(&mut poor); }
        assert_eq!(crate::save(&poor), before, "an unfunded lever never reaches the random draw");

        let mut attempts = 0;
        while !government::is_electoral(&w, id) && attempts < 1000 {
            ai_stratagems(&mut w);
            attempts += 1;
        }
        assert!(government::is_electoral(&w, id), "the real dispatch path never offered an affordable negotiation");
        assert_eq!(w.nation(id).political_capital, 0.0);
        assert_eq!(government::state(&w, id).unwrap().next_election, (1990, 7));
    }

    fn consolidation_world(daily: bool) -> WorldState {
        let mut w = world_1990(GameRules { daily_simulation: daily, ..Default::default() });
        let id = NationId::Italy;
        w.player = Some(id);
        let n = w.nation_mut(id);
        n.debt_gdp = 1.30;
        n.tax_rate = 0.30;
        n.mil_spend_gdp = 0.04;
        n.state_invest_gdp = 0.08;
        n.stability = 80.0;
        n.political_capital = 100.0;
        w
    }

    fn buy_consolidation(w: &mut WorldState) {
        crate::apply_command(w, &crate::Command::EnactStratagem {
            nation: NationId::Italy, id: "austerity".into(),
        }).unwrap();
    }

    #[test]
    fn the_paid_consolidation_card_cuts_without_inventing_minimum_spending() {
        let id = NationId::Italy;
        for daily in [false, true] {
            for (mil, investment, expected_mil, expected_investment) in [
                (0.0, 0.0, 0.0, 0.0),
                (0.004, 0.014, 0.004, 0.014),
                (0.005, 0.020, 0.005, 0.020),
                (0.04, 0.08, 0.032, 0.056),
            ] {
                let mut w = consolidation_world(daily);
                let n = w.nation_mut(id);
                n.mil_spend_gdp = mil;
                n.state_invest_gdp = investment;
                buy_consolidation(&mut w);
                let n = w.nation(id);
                assert!((n.tax_rate - 0.35).abs() < 1e-12);
                assert!((n.mil_spend_gdp - expected_mil).abs() < 1e-12);
                assert!((n.state_invest_gdp - expected_investment).abs() < 1e-12);
                assert!(n.mil_spend_gdp <= mil && n.state_invest_gdp <= investment);
                assert_eq!(n.political_capital, 66.0);
                assert_eq!(n.stability, 71.0);
                assert!(!n.on_the_books() && n.annual_budget.is_none());
            }
        }
        let mut capped = consolidation_world(false);
        capped.nation_mut(id).tax_rate = 0.60;
        buy_consolidation(&mut capped);
        assert_eq!(capped.nation(id).tax_rate, 0.60);
        let mut refused = consolidation_world(false);
        refused.nation_mut(id).political_capital = 33.99;
        let before = crate::save(&refused);
        assert!(crate::apply_command(&mut refused, &crate::Command::EnactStratagem {
            nation: id, id: "austerity".into(),
        }).is_err());
        assert_eq!(crate::save(&refused), before);
    }

    #[test]
    fn the_paid_consolidation_card_updates_owned_plans_and_charges_once() {
        let id = NationId::Italy;
        for program in [false, true] {
            for small in [false, true] {
                let mut w = consolidation_world(true);
                if small {
                    w.nation_mut(id).mil_spend_gdp = 0.004;
                    w.nation_mut(id).state_invest_gdp = 0.014;
                }
                let allocations = w.nation(id).budget_for(w.year).allocations;
                let command = if program {
                    // A pre-existing legacy prepaid entitlement, deliberately
                    // not a cash grant from this consolidation package.
                    w.nation_mut(id).arsenal.banked = 2.0;
                    crate::Command::SetProgramBudget {
                        nation: id, fiscal_year: 1990, allocations,
                        departments: crate::programs::default_departments(),
                    }
                } else {
                    crate::Command::SetAnnualBudget { nation: id, fiscal_year: 1990, allocations }
                };
                crate::apply_command(&mut w, &command).unwrap();
                if program {
                    crate::programs::begin_day(&mut w);
                    let available = crate::programs::available_bn(&w, id, BUDGET_INDUSTRY, 0);
                    assert!(available > 0.0);
                    crate::programs::spend(&mut w, id, BUDGET_INDUSTRY, 0, available / 4.0).unwrap();
                }
                let n = w.nation(id);
                let before = n.budget_for(w.year);
                let ledger = n.program_budget.clone();
                let stocks = (n.treasury_bn, n.debt_bn, n.debt_gdp);
                let pc = n.political_capital;
                buy_consolidation(&mut w);
                let n = w.nation(id);
                let after = n.annual_budget.as_ref().unwrap();
                assert_eq!(after.reference, before.reference);
                assert_eq!(after.social_total(), before.social_total());
                assert_eq!(n.program_budget, ledger, "authority, prepaid funds and dated receipts remain exact");
                assert_eq!((n.treasury_bn, n.debt_bn, n.debt_gdp), stocks);
                assert_eq!(n.political_capital, pc - 34.0, "the card has one political price");
                assert_eq!(n.mil_spend_gdp, after.defense());
                assert_eq!(n.state_invest_gdp, after.investment_total());
                assert!((after.defense() - if small { 0.004 } else { 0.032 }).abs() < 1e-12);
                assert!((after.investment_total() - if small { 0.014 } else { 0.056 }).abs() < 1e-12);
                for ministry in [BUDGET_INFRASTRUCTURE, BUDGET_INDUSTRY, BUDGET_SCIENCE] {
                    let expected = before.allocations[ministry] * if small { 1.0 } else { 0.7 };
                    assert!((after.allocations[ministry] - expected).abs() < 1e-12);
                }
            }
        }
    }

    #[test]
    fn an_unowned_program_consolidation_is_refused_before_any_effect() {
        let id = NationId::Italy;
        let mut w = consolidation_world(true);
        let allocations = w.nation(id).budget_for(w.year).allocations;
        crate::apply_command(&mut w, &crate::Command::SetProgramBudget {
            nation: id, fiscal_year: 1990, allocations,
            departments: crate::programs::default_departments(),
        }).unwrap();
        w.player = None;
        assert!(!crate::economic_ai::enabled(&w));
        let before = crate::save(&w);
        assert!(crate::apply_command(&mut w, &crate::Command::EnactStratagem {
            nation: id, id: "austerity".into(),
        }).is_err());
        assert_eq!(crate::save(&w), before);
        w.rules.economic_competition = true;
        let pc = w.nation(id).political_capital;
        buy_consolidation(&mut w);
        assert_eq!(w.nation(id).political_capital, pc - 34.0);
        assert!((w.nation(id).mil_spend_gdp - 0.032).abs() < 1e-12);
    }

    #[test]
    fn a_paid_consolidation_keeps_stock_only_aggregate_authority() {
        let id = NationId::Italy;
        let mut w = consolidation_world(false);
        let allocations = w.nation(id).budget_for(w.year).allocations;
        crate::apply_command(&mut w, &crate::Command::SetAnnualBudget {
            nation: id, fiscal_year: 1990, allocations,
        }).unwrap();
        // A legal aggregate vote can exceed an inherited detailed row's cap.
        // A later paid card must not silently convert it to a different owner.
        crate::apply_command(&mut w, &crate::Command::SetStateInvest {
            nation: id, share: 0.40,
        }).unwrap();
        let n = w.nation(id);
        assert!(n.on_the_books() && n.annual_budget.is_none());
        let before = (n.treasury_bn, n.debt_bn, n.debt_gdp, n.social_spend_gdp);
        let pc = n.political_capital;
        buy_consolidation(&mut w);
        let n = w.nation(id);
        assert_eq!((n.treasury_bn, n.debt_bn, n.debt_gdp, n.social_spend_gdp), before);
        assert!(n.annual_budget.is_none() && n.program_budget.is_none());
        assert!((n.state_invest_gdp - 0.28).abs() < 1e-12);
        assert!((n.mil_spend_gdp - 0.032).abs() < 1e-12);
        assert_eq!(n.political_capital, pc - 34.0);
    }

    fn consolidation_owner(w: &mut WorldState, owner: &str) {
        let id = NationId::Italy;
        let allocations = w.nation(id).budget_for(w.year).allocations;
        let command = match owner {
            "aggregate" => return,
            "annual" => crate::Command::SetAnnualBudget { nation: id, fiscal_year: 1990, allocations },
            "program" => crate::Command::SetProgramBudget {
                nation: id, fiscal_year: 1990, allocations,
                departments: crate::programs::default_departments(),
            },
            _ => unreachable!(),
        };
        crate::apply_command(w, &command).unwrap();
    }

    #[test]
    fn a_fiscal_consolidation_with_no_financial_effect_is_refused() {
        let id = NationId::Italy;
        for owner in ["aggregate", "annual", "program"] {
            for player in [false, true] {
                let mut w = consolidation_world(true);
                let n = w.nation_mut(id);
                n.tax_rate = 0.60;
                n.mil_spend_gdp = 0.005;
                n.state_invest_gdp = 0.02;
                consolidation_owner(&mut w, owner);
                if !player {
                    w.player = None;
                    w.rules.economic_competition = true;
                }
                let before = crate::save(&w);
                // The AI reads this same pure deck. It may choose another
                // legitimate action, but never this punitive financial no-op.
                assert!(!available(&w, id).iter().any(|s| s.id == "austerity"));
                assert!(crate::apply_command(&mut w, &crate::Command::EnactStratagem {
                    nation: id, id: "austerity".into(),
                }).is_err());
                assert_eq!(crate::save(&w), before, "{owner}, player={player}");
            }
        }
    }

    #[test]
    fn each_single_financial_lever_keeps_consolidation_available() {
        let id = NationId::Italy;
        for owner in ["aggregate", "annual", "program"] {
            for lever in ["tax", "investment", "defense"] {
                let mut w = consolidation_world(true);
                let n = w.nation_mut(id);
                n.tax_rate = if lever == "tax" { 0.59 } else { 0.60 };
                n.state_invest_gdp = if lever == "investment" { 0.03 } else { 0.02 };
                n.mil_spend_gdp = if lever == "defense" { 0.02 } else { 0.005 };
                consolidation_owner(&mut w, owner);
                let pc = w.nation(id).political_capital;
                assert!(available(&w, id).iter().any(|s| s.id == "austerity"));
                buy_consolidation(&mut w);
                let n = w.nation(id);
                assert_eq!(n.political_capital, pc - 34.0);
                assert_eq!(n.stability, 71.0);
                assert!((n.tax_rate - 0.60).abs() < 1e-12);
                assert!((n.state_invest_gdp - if lever == "investment" { 0.021 } else { 0.02 }).abs() < 1e-12);
                assert!((n.mil_spend_gdp - if lever == "defense" { 0.016 } else { 0.005 }).abs() < 1e-12);
            }
        }
    }

}
