pub mod arsenal;
pub mod commitment;
pub mod data;
pub mod districts;
pub mod dyads;
pub mod economy;
pub mod front;
pub mod government;
pub mod exact;
pub mod init;
pub mod nations;
pub mod politics;
pub mod statecraft;
pub mod stratagems;
pub mod tech;
pub mod theatre;
pub mod war;
pub mod world;

use serde::{Deserialize, Serialize};
use world::*;

/// All player and AI actions flow through the command queue.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Command {
    SetInterestRate { nation: NationId, rate: f64 },
    SetTaxRate { nation: NationId, rate: f64 },
    SetMilSpend { nation: NationId, share: f64 },
    SetStateInvest { nation: NationId, share: f64 },
    /// Put a domain's laboratories onto a named technology, or hand the choice
    /// back to them with `None`. Switching away from a project in progress
    /// forfeits half of what was banked against it.
    SetResearchFocus { nation: NationId, domain: tech::Domain, tech: Option<String> },
    /// Declare one domain a national research programme, or stand the last one
    /// down with `None`. Worth `tech::PRIORITY_MULTIPLIER` of its ordinary share
    /// of the budget, taken from the other seven.
    SetResearchPriority { nation: NationId, domain: Option<tech::Domain> },
    Sanction { imposer: NationId, target: NationId },
    LiftSanction { imposer: NationId, target: NationId },
    ImproveRelations { from: NationId, to: NationId },
    DeclareWar { attacker: NationId, defender: NationId },
    /// Offer a mutual defence guarantee. The other government decides.
    ProposeAlliance { from: NationId, to: NationId },
    /// Walk away from one. Cheap in peacetime, ruinous while the ally is under attack.
    BreakAlliance { from: NationId, to: NationId },
    /// A standing transfer of `share_gdp` of the patron's output, until cancelled.
    PledgeAid { patron: NationId, client: NationId, kind: AidKind, share_gdp: f64 },
    EndAid { patron: NationId, client: NationId, kind: AidKind },
    CovertAction { sponsor: NationId, target: NationId, op: CovertOp },
    ProposeTrade { from: NationId, to: NationId },
    AbrogateTrade { from: NationId, to: NationId },
    /// Take one of the options the world is currently offering this government.
    /// Carries the stratagem's stable id, never an index into the deck.
    EnactStratagem { nation: NationId, id: String },

    // --- Government: who holds office, and what holding it costs ---
    /// Bring a party into the cabinet. Carries the party's stable id.
    InviteToGovernment { nation: NationId, party: String },
    /// Throw one out. Cheap, always available, and frequently the end of your
    /// majority.
    ExpelFromGovernment { nation: NationId, party: String },
    /// Go back to the country before you have to.
    CallElection { nation: NationId },
    /// What a regime that does not hold elections does instead: pay one of the
    /// institutions that could remove it.
    SecurePillar { nation: NationId, pillar: government::Pillar },

    // --- The commitment ladder (BIBLE §6) ------------------------------------
    /// Start a quarrel at rung 1. Conflicts begin when somebody climbs, not with
    /// a declaration, and this is deliberately the cheapest thing in the enum.
    OpenConflict { opener: NationId, target: NationId, theatre: theatre::TheatreId },
    /// Take a side in a quarrel that is already running. Entering is entering at
    /// the bottom: rung 1, and the ladder from there.
    JoinConflict { conflict: u32, nation: NationId, side_a: bool, objective: Objective },
    /// The primary click of the whole war layer: pick your rung.
    SetCommitment { conflict: u32, nation: NationId, rung: u8 },
    SetObjective { conflict: u32, nation: NationId, objective: Objective },
    SetRoE { conflict: u32, nation: NationId, roe: Roe },
    /// Announce a limit. It publicly binds you, and the other side reads it.
    SetCeiling { conflict: u32, nation: NationId, rung: u8 },
    /// The acceptable-casualties threshold that terminates by itself.
    SetRedLine { conflict: u32, nation: NationId, resolve_floor: f64 },

    // --- Access: statecraft as a direct military input -----------------------
    RequestAccess { seeker: NationId, host: NationId, theatre: theatre::TheatreId },
    /// Ask again, with the leverage dependency built. Ankara, March 2003.
    PressForAccess { seeker: NationId, host: NationId, theatre: theatre::TheatreId },
    /// The host's own click, and the one that gives a small state agency in a
    /// great power's war.
    GrantAccess { host: NationId, seeker: NationId, theatre: theatre::TheatreId, grant: bool },
    RevokeAccess { host: NationId, seeker: NationId, theatre: theatre::TheatreId },
}

/// What a command asks of the government that issues it, and who it asks.
///
/// Policy is priced on how far it moves, not on being touched: nudging a rate a
/// quarter point is free in a way that doubling it is not. Nothing here is a
/// toll on playing the game — an idle government accrues faster than ordinary
/// play spends — but a government cannot reverse its whole programme twice in a
/// year, and one that has just spent a war's worth of standing cannot do
/// anything at all until it has delivered something.
/// Returns who pays, what it costs, and whether the price can refuse the act.
///
/// Most things a government does need standing it does not have: it cannot
/// simply decide to raise taxes it has no authority to raise. But a government
/// can always break its word. Renouncing a guarantee, cutting a client loose,
/// tearing up a trade treaty — these are always available and merely ruinous,
/// so they are charged to the point of bankruptcy rather than refused. A model
/// that stopped a discredited government from abandoning its allies would have
/// it exactly backwards; that is the government most likely to.
fn command_price(w: &WorldState, c: &Command) -> Option<(NationId, f64, bool)> {
    let swing = |before: f64, after: f64, per_point: f64| (after - before).abs() * per_point;
    /// Needs the standing to act.
    const REFUSABLE: bool = true;
    /// Always available, and charged anyway.
    const ALWAYS: bool = false;
    Some(match c {
        Command::SetInterestRate { nation, rate } => (
            *nation,
            swing(w.nation(*nation).interest_rate, rate.clamp(0.0, 0.60), 90.0),
            REFUSABLE,
        ),
        Command::SetTaxRate { nation, rate } => (
            *nation,
            // Taxes are the most expensive thing a government touches, and
            // raising them costs about three times what cutting them does.
            {
                let before = w.nation(*nation).tax_rate;
                let after = rate.clamp(0.02, 0.60);
                if after > before { swing(before, after, 320.0) } else { swing(before, after, 110.0) }
            },
            REFUSABLE,
        ),
        Command::SetMilSpend { nation, share } => (
            *nation,
            swing(w.nation(*nation).mil_spend_gdp, share.clamp(0.0, 0.35), 150.0),
            REFUSABLE,
        ),
        Command::SetStateInvest { nation, share } => (
            *nation,
            swing(w.nation(*nation).state_invest_gdp, share.clamp(0.0, 0.40), 120.0),
            REFUSABLE,
        ),
        Command::Sanction { imposer, .. } => (*imposer, 6.0, REFUSABLE),
        // Redirecting a laboratory is an ordinary act of government and priced
        // like one. The expensive part is not the announcement, it is the half
        // of the banked progress the switch throws away, which the model charges
        // in months rather than in standing.
        Command::SetResearchFocus { nation, .. } => (*nation, 6.0, REFUSABLE),
        // Declaring a national programme is not an ordinary act. It is a public
        // commitment that seven other domains are going to go short, and every
        // interest behind them knows it.
        Command::SetResearchPriority { nation, domain } => {
            (*nation, if domain.is_some() { 30.0 } else { 10.0 }, REFUSABLE)
        }
        Command::LiftSanction { imposer, .. } => (*imposer, 3.0, REFUSABLE),
        Command::ImproveRelations { from, .. } => (*from, 2.0, REFUSABLE),
        // The most expensive thing a government can decide to do.
        Command::DeclareWar { attacker, .. } => (*attacker, 30.0, REFUSABLE),

        // --- Statecraft: holding a sphere is what the currency is for ---
        // Promising to fight somebody else's war is a commitment made at home
        // before it is made abroad, and a government without standing cannot
        // credibly make it.
        Command::ProposeAlliance { from, .. } => (*from, 12.0, REFUSABLE),
        // Cheap when nothing is happening and the most expensive thing in this
        // list when it is not: abandoning an ally under attack is the act a
        // government is remembered for. Never refused, because a government can
        // always renege — statecraft charges the reputation, this charges what
        // is left of the standing.
        Command::BreakAlliance { from, to } => (
            *from,
            if w.at_war(*to) { 45.0 } else { 8.0 },
            ALWAYS,
        ),
        // Sending output abroad has to be explained to the people it was taxed
        // from, and the bill scales with how much is being sent. A patron near
        // the ceiling of what it can promise spends real standing to stay there,
        // which is the mechanism by which a sphere is expensive to *hold* and
        // not merely expensive to buy.
        Command::PledgeAid { patron, share_gdp, .. } => (
            *patron,
            share_gdp.abs() * 1500.0 + 3.0,
            REFUSABLE,
        ),
        // Cutting a client loose is cheap at home and catastrophic at the other
        // end, which is why the threat works while the money still flows.
        Command::EndAid { patron, .. } => (*patron, 3.0, ALWAYS),
        // Deniable by construction: what a government spends on an operation
        // nobody is supposed to know about is small. The bill for it landing in
        // the newspapers is charged by statecraft, not here.
        Command::CovertAction { sponsor, .. } => (*sponsor, 5.0, REFUSABLE),
        // Opening a market is the popular half of the trade.
        Command::ProposeTrade { from, .. } => (*from, 2.0, REFUSABLE),
        // Closing one is the unpopular half, and it is your own importers who
        // notice first.
        Command::AbrogateTrade { from, .. } => (*from, 10.0, ALWAYS),
        // Each stratagem carries its own price, and they are the largest in this
        // list. Reordering an economy is the most expensive thing a government
        // ever decides to do, and it should cost most of a term's standing.
        Command::EnactStratagem { nation, id } => (
            *nation,
            stratagems::by_id(id).map_or(0.0, |s| s.cost),
            REFUSABLE,
        ),

        // A coalition partner is bought, not persuaded, and the bill is the
        // distance between you: a neighbouring party wants a ministry, one at
        // the other end of the plane wants most of your programme.
        Command::InviteToGovernment { nation, party } => (
            *nation,
            government::invite_price(w, *nation, party),
            REFUSABLE,
        ),
        // Breaking up your own government is always available and never free —
        // the same rule as walking out on an ally.
        Command::ExpelFromGovernment { nation, .. } => (*nation, 12.0, ALWAYS),
        // Going to the country early is a gamble a weak government cannot pay
        // for, which is exactly why weak governments limp on.
        Command::CallElection { nation } => (*nation, 25.0, REFUSABLE),
        // Patronage. Cheaper than an election and it has to be paid again.
        Command::SecurePillar { nation, .. } => (*nation, 14.0, REFUSABLE),

        // --- The ladder. Every rung is a purchase. ---------------------------
        // Opening at rhetoric is nearly free on purpose: the first rung has to
        // be a real option rather than a formality, or nobody ever uses the
        // bottom of the ladder and it becomes a war button with extra steps.
        Command::OpenConflict { opener, .. } => (*opener, 4.0, REFUSABLE),
        // Taking a side in somebody else's war is the most consequential thing a
        // government can do without firing anything, and it is priced between
        // opening a quarrel of your own and the guarantee that would have
        // obliged you to. What it buys is rung 1 — every rung after it is
        // charged again, which is the whole point of the ladder.
        Command::JoinConflict { nation, .. } => (*nation, 14.0, REFUSABLE),
        // Climbing is charged by how far, and by what kind of government has to
        // explain it. Descending is free here and paid in reputation instead —
        // a government can always run away, and it always looks like running.
        Command::SetCommitment { conflict, nation, rung } => (
            *nation,
            w.conflict(*conflict).map_or(0.0, |c| {
                let home = commitment::defending_home(w, c, *nation);
                c.posture_of(*nation).map_or(0.0, |b| {
                    commitment::escalation_cost_in(w, *nation, b.rung, *rung, home)
                })
            }),
            REFUSABLE,
        ),
        Command::SetObjective { nation, .. } => (*nation, 3.0, REFUSABLE),
        // Restraint is free. Taking the gloves off is not, and it is charged
        // twice: here, and again in every parliament that was going to lend you
        // an airfield.
        Command::SetRoE { nation, roe, .. } => (
            *nation,
            if *roe == Roe::Unrestricted { 8.0 } else { 0.0 },
            REFUSABLE,
        ),
        // Announcing a limit is itself a political act.
        Command::SetCeiling { nation, .. } => (*nation, 4.0, REFUSABLE),
        // Deciding in advance what you will not pay costs nothing and is the
        // most valuable thing on this list.
        Command::SetRedLine { nation, .. } => (*nation, 0.0, ALWAYS),

        Command::RequestAccess { seeker, .. } => (*seeker, 6.0, REFUSABLE),
        Command::PressForAccess { seeker, .. } => (*seeker, 15.0, REFUSABLE),
        // The host pays at home, which is exactly why a small state's political
        // capital suddenly matters in somebody else's war. Refusing is free.
        Command::GrantAccess { host, grant, .. } => {
            (*host, if *grant { 5.0 } else { 0.0 }, REFUSABLE)
        }
        // A parliament can always vote, including on the way out — dearer while
        // a superpower is actually flying out of your bases.
        Command::RevokeAccess { host, seeker, theatre } => (
            *host,
            if w.conflicts.iter().any(|c| {
                c.theatre == *theatre
                    && c.posture_of(*seeker).is_some_and(|b| b.rung >= 7)
            }) {
                20.0
            } else {
                4.0
            },
            ALWAYS,
        ),
    })
}

pub fn apply_command(w: &mut WorldState, c: &Command) -> Result<(), String> {
    // Priced before anything happens, so a command that cannot be afforded also
    // cannot take effect — and charged only once the act itself has gone
    // through. A government that asks for something the world refuses it (a
    // rung with no airfield under it, a pact with a state that will not sign)
    // has not spent its standing on the asking, and before this it did: every
    // blocked rung quietly drained the treasury of whoever tried it.
    let bill = command_price(w, c).filter(|(_, price, _)| *price > 0.0);
    if let Some((payer, price, refusable)) = bill {
        let held = w.nation(payer).political_capital;
        if refusable && held < price {
            return Err(format!(
                "{} has not the standing: {:.0} political capital held, {:.0} needed.",
                payer.name(), held, price
            ));
        }
    }
    let outcome = dispatch(w, c);
    if outcome.is_ok() {
        if let Some((payer, price, _)) = bill {
            // A government that reneges past the end of its credit does not get
            // to owe political capital; it simply has none left.
            let held = w.nation(payer).political_capital;
            w.nation_mut(payer).political_capital = (held - price).max(0.0);
        }
    }
    outcome
}

fn dispatch(w: &mut WorldState, c: &Command) -> Result<(), String> {
    match c {
        Command::SetInterestRate { nation, rate } => {
            w.nation_mut(*nation).interest_rate = rate.clamp(0.0, 0.60);
            // The player has taken the wheel; the AI bank stands down for good.
            // Latched here rather than inferred from the rate's value so that
            // deliberately re-setting the rate one already had still counts.
            if Some(*nation) == w.player {
                w.player_set_rate = true;
            }
        }
        Command::SetTaxRate { nation, rate } => {
            w.nation_mut(*nation).tax_rate = rate.clamp(0.02, 0.60);
        }
        Command::SetMilSpend { nation, share } => {
            w.nation_mut(*nation).mil_spend_gdp = share.clamp(0.0, 0.35);
        }
        Command::SetStateInvest { nation, share } => {
            w.nation_mut(*nation).state_invest_gdp = share.clamp(0.0, 0.40);
        }
        Command::SetResearchFocus { nation, domain, tech: want } => {
            let di = domain.index();
            let target = match want {
                None => None,
                Some(id) => {
                    let idx = tech::index_of(id)
                        .ok_or_else(|| format!("No technology called {}.", id))?;
                    let n = w.nation(*nation);
                    if !tech::eligible_projects(n, *domain).iter().any(|d| d.id == id) {
                        return Err(format!(
                            "{} cannot start {} yet: it is either already known or its \
                             prerequisites are not.",
                            nation.name(),
                            id
                        ));
                    }
                    Some(idx)
                }
            };
            let n = w.nation_mut(*nation);
            if n.tech.focus.get(di).copied().flatten() != target {
                // A laboratory redirected does not start from nothing, and does
                // not carry everything across either. Half the bank survives the
                // change of subject; the rest was specific to the old one.
                if let Some(pr) = n.tech.progress.get_mut(di) {
                    *pr *= 0.5;
                }
            }
            if let Some(slot) = n.tech.focus.get_mut(di) {
                *slot = target;
            }
        }
        Command::SetResearchPriority { nation, domain } => {
            w.nation_mut(*nation).tech.priority = *domain;
            match domain {
                Some(d) => w.headline(format!(
                    "{} declares a national {} programme.",
                    nation.name(),
                    d.name().to_lowercase()
                )),
                None => w.headline(format!(
                    "{} stands down its national research programme.",
                    nation.name()
                )),
            }
        }
        Command::Sanction { imposer, target } => {
            if *imposer == *target {
                return Err("Cannot sanction yourself.".into());
            }
            if !w.is_sanctioning(*imposer, *target) {
                w.sanctions.push((*imposer, *target));
                w.shift_relation(*imposer, *target, -15.0);
                w.headline(format!("{} imposes sanctions on {}.", imposer.name(), target.name()));
            }
        }
        Command::LiftSanction { imposer, target } => {
            w.sanctions.retain(|(i, t)| !(i == imposer && t == target));
        }
        Command::ImproveRelations { from, to } => {
            w.shift_relation(*from, *to, 6.0);
            w.headline(format!("{} extends a diplomatic hand to {}.", from.name(), to.name()));
        }
        Command::DeclareWar { attacker, defender } => {
            war::declare_war(w, *attacker, *defender)?;
        }
        Command::ProposeAlliance { from, to } => statecraft::propose_pact(w, *from, *to)?,
        Command::BreakAlliance { from, to } => statecraft::break_pact(w, *from, *to)?,
        Command::PledgeAid { patron, client, kind, share_gdp } => {
            statecraft::pledge_aid(w, *patron, *client, *kind, *share_gdp)?
        }
        Command::EndAid { patron, client, kind } => statecraft::end_aid(w, *patron, *client, *kind)?,
        Command::CovertAction { sponsor, target, op } => {
            statecraft::covert_action(w, *sponsor, *target, *op)?
        }
        Command::ProposeTrade { from, to } => statecraft::propose_trade(w, *from, *to)?,
        Command::AbrogateTrade { from, to } => statecraft::abrogate_trade(w, *from, *to)?,
        Command::EnactStratagem { nation, id } => {
            let s = stratagems::by_id(id)
                .ok_or_else(|| format!("No such stratagem: {}", id))?;
            // Checked again here, not only when the menu was drawn: the world
            // may have moved between a government deciding and acting.
            if !(s.available)(w, *nation) {
                return Err(format!(
                    "{} is no longer open to {}.",
                    s.name,
                    nation.name()
                ));
            }
            (s.enact)(w, *nation);
        }
        Command::InviteToGovernment { nation, party } => {
            government::invite(w, *nation, party)?
        }
        Command::ExpelFromGovernment { nation, party } => {
            government::expel(w, *nation, party)?
        }
        Command::CallElection { nation } => government::call_election(w, *nation)?,
        Command::SecurePillar { nation, pillar } => {
            government::secure_pillar(w, *nation, *pillar)?
        }

        Command::OpenConflict { opener, target, theatre } => {
            commitment::open_conflict(w, *opener, *target, *theatre)?;
        }
        Command::JoinConflict { conflict, nation, side_a, objective } => {
            commitment::join_conflict(w, *nation, *conflict, *side_a, *objective)?
        }
        Command::SetCommitment { conflict, nation, rung } => {
            commitment::set_commitment(w, *conflict, *nation, *rung)?
        }
        Command::SetObjective { conflict, nation, objective } => {
            let c = w.conflict_mut(*conflict).ok_or("No such conflict.")?;
            let b = c.posture_mut(*nation).ok_or("Not a party to that conflict.")?;
            b.objective = *objective;
            w.headline(format!(
                "{}'s objective is now to {}.",
                nation.name(),
                objective.label()
            ));
        }
        Command::SetRoE { conflict, nation, roe } => {
            let c = w.conflict_mut(*conflict).ok_or("No such conflict.")?;
            let b = c.posture_mut(*nation).ok_or("Not a party to that conflict.")?;
            b.roe = *roe;
            if *roe == Roe::Unrestricted {
                // Winning faster and keeping your host's airbase are the same
                // decision, and this is the click where they are traded.
                w.shift_reputation(*nation, -6.0);
                w.headline(format!("{} takes the gloves off.", nation.name()));
            }
        }
        Command::SetCeiling { conflict, nation, rung } => {
            let r = (*rung).clamp(1, 9);
            let c = w.conflict_mut(*conflict).ok_or("No such conflict.")?;
            let b = c.posture_mut(*nation).ok_or("Not a party to that conflict.")?;
            b.ceiling = r;
            if b.rung > r {
                b.rung = r;
            }
            w.headline(format!(
                "{} publicly rules out going beyond rung {} — {}.",
                nation.name(),
                r,
                rung_name(r)
            ));
        }
        Command::SetRedLine { conflict, nation, resolve_floor } => {
            let c = w.conflict_mut(*conflict).ok_or("No such conflict.")?;
            let b = c.posture_mut(*nation).ok_or("Not a party to that conflict.")?;
            b.red_line = resolve_floor.clamp(0.0, 0.95);
        }

        Command::RequestAccess { seeker, host, theatre } => {
            commitment::request_access(w, *seeker, *host, *theatre, false)?
        }
        Command::PressForAccess { seeker, host, theatre } => {
            commitment::request_access(w, *seeker, *host, *theatre, true)?
        }
        Command::GrantAccess { host, seeker, theatre, grant } => {
            commitment::grant_access(w, *host, *seeker, *theatre, *grant)?
        }
        Command::RevokeAccess { host, seeker, theatre } => {
            commitment::revoke_access(w, *host, *seeker, *theatre)?
        }
    }
    Ok(())
}

/// Every subsystem a month runs, in the order it runs them.
///
/// A table rather than eight statements in `tick_month` so that the profiling
/// instrument (`century_run_profile`, below) times exactly what the game runs
/// and cannot drift out of sync with it when a system is added or reordered.
/// The comments that used to sit between the calls are the ones below.
///
/// `government::tick` appears here rather than as the first statement of
/// `politics::tick`, which is where it used to live. Same call in the same
/// position; hoisted only so that its cost is reported separately, because it
/// is the largest single entry in the table.
#[allow(clippy::type_complexity)]
pub const SYSTEMS: &[(&str, fn(&mut WorldState))] = &[
    ("economy", economy::tick),
    // Research is funded out of the output the economy has just produced, and
    // what it unlocks is in the nation's hands before the soldiers and the
    // politicians get their turn with it.
    ("tech", tech::tick),
    // Pacts decide who is obliged to join a war and patronage decides who can
    // still afford one, so the standing arrangements are settled before the
    // fighting is worked out.
    // Money becomes orders years before it becomes strength, so procurement
    // runs with the other standing bills rather than beside the fighting.
    ("arsenal", arsenal::tick),
    ("statecraft", statecraft::tick),
    ("stratagems", stratagems::tick),
    ("ai_stratagems", stratagems::ai_stratagems),
    ("war", war::tick),
    // Who holds office is settled before what their standing is worth: an
    // election held this month, or a coup, has to be reflected in the capital
    // the government wakes up holding.
    ("government", government::tick),
    ("politics", politics::tick),
];

/// Advance the world one month. Commands are applied before systems tick.
pub fn tick_month(w: &mut WorldState, commands: &[Command]) -> Vec<String> {
    w.headlines.clear();
    // The id -> position index, refreshed once a month. A federation coming
    // apart mid-tick appends to `nations` and leaves it stale for the rest of
    // that month, which every lookup already handles by falling back to the
    // scan it used to do unconditionally.
    w.reindex();
    for c in commands {
        if let Err(e) = apply_command(w, c) {
            w.headline(format!("[rejected] {:?}: {}", c, e));
        }
    }
    for (_, system) in SYSTEMS {
        system(w);
    }

    // Calendar
    w.month += 1;
    if w.month > 12 {
        w.month = 1;
        w.year += 1;
    }
    w.headlines.clone()
}

/// FNV-1a over the serialized world — a cheap, stable fingerprint of an entire
/// timeline. Two runs that agree here agree everywhere, which makes this the
/// oracle for determinism tests and, later, for replay verification.
pub fn state_hash(w: &WorldState) -> u64 {
    const OFFSET_BASIS: u64 = 0xcbf2_9ce4_8422_2325;
    const PRIME: u64 = 0x0000_0100_0000_01b3;
    let mut h = OFFSET_BASIS;
    for b in save(w).as_bytes() {
        h ^= *b as u64;
        h = h.wrapping_mul(PRIME);
    }
    h
}

pub fn save(w: &WorldState) -> String {
    serde_json::to_string_pretty(w).expect("serialize")
}
pub fn load(s: &str) -> Result<WorldState, String> {
    let mut w: WorldState = serde_json::from_str(s).map_err(|e| e.to_string())?;
    migrate_legacy_wars(&mut w);
    if w.theatres.is_empty() {
        w.theatres = theatre::default_theatres();
    }
    // A save written before districts existed carries none; reconstruct the
    // best map available — 1990 defaults, then alive successors claim their
    // own ground. See `districts::reseed` for what is and is not recoverable.
    if w.districts.is_empty() {
        districts::reseed(&mut w);
    }
    // ...and a save written before the operational map carries wars with a
    // control scalar and no front. Project the scalar back onto the ground it
    // summarizes — deterministic, no RNG — so the map and the number agree
    // from the first rendered frame. A new-code roundtrip never lands here:
    // its fronts are serialized.
    front::reseed_fronts(&mut w);
    // The position index is derived, so it is not in the save. Built here so a
    // loaded world that is only ever read — the web server rendering a turn —
    // gets the same lookups as one that has ticked.
    w.reindex();
    Ok(w)
}

/// A save written before the commitment ladder carries a `wars` array and no
/// conflicts. Every such war was, by construction, a full conventional campaign
/// on both sides, so it reopens at rung 8 with its progress bar reinterpreted as
/// control and its magazines and resolve full.
fn migrate_legacy_wars(w: &mut WorldState) {
    if w.wars.is_empty() {
        return;
    }
    let legacy = std::mem::take(&mut w.wars);
    for old in legacy {
        let mut side_a = vec![old.attacker];
        side_a.extend(old.attacker_allies.iter().copied());
        let mut side_b = vec![old.defender];
        side_b.extend(old.defender_allies.iter().copied());
        let mut posture: Vec<Belligerent> = side_a
            .iter()
            .map(|id| Belligerent::new(*id, 8, Objective::Seize))
            .collect();
        posture.extend(side_b.iter().map(|id| {
            let mut b = Belligerent::new(*id, 8, Objective::Hold);
            b.stake = 1.0;
            b
        }));
        let id = w.next_conflict_id();
        w.conflicts.push(Conflict {
            id,
            theatre: war::theatre_between(w, old.attacker, old.defender),
            side_a,
            side_b,
            posture,
            control: (old.progress / 100.0).clamp(-1.0, 1.0),
            months: 0,
            quiet_months: 0,
            frozen_since: None,
            start_year: old.start_year,
            start_month: old.start_month,
            origin_attacker: old.attacker,
            // A save from before the ladder holds only wars that were already
            // invasions, and the coalition against them has long since formed.
            invasion_declared: true,
            // Reconstructed by `front::reseed_fronts` right after this runs.
            front: std::collections::BTreeMap::new(),
            pockets: vec![],
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::init::world_1990;

    fn run_months(w: &mut WorldState, months: usize) {
        for _ in 0..months {
            tick_month(w, &[]);
        }
    }

    #[test]
    fn determinism_same_seed_same_world() {
        let mut a = world_1990(GameRules::default());
        let mut b = world_1990(GameRules::default());
        run_months(&mut a, 240);
        run_months(&mut b, 240);
        assert_eq!(save(&a), save(&b));
    }

    #[test]
    fn saves_name_technologies_rather_than_numbering_them() {
        // The registry is eight independently-authored files concatenated, so
        // adding one technology renumbers every later one. A save that stored
        // indices would silently reinterpret on the next build — a nation that
        // knew one thing would wake up knowing another, with nothing to detect
        // it. Saves must therefore carry ids, and an id this build cannot
        // resolve must be dropped rather than mapped onto its neighbour.
        let mut w = world_1990(GameRules::default());
        run_months(&mut w, 240);
        let text = save(&w);

        // Whatever anyone knows, it is written down by name.
        let known_by_number = text.contains("\"known\": [\n          0")
            || text.contains("\"known\": [0");
        assert!(!known_by_number, "save is storing raw registry indices");

        // An id from a build that had a technology this one does not must be
        // dropped, leaving a world that still loads and still runs.
        const GHOST: &str = "xx_technology_from_a_later_build";
        let doctored = if text.contains("\"known\": []") {
            text.replace("\"known\": []", &format!("\"known\": [\"{}\"]", GHOST))
        } else {
            text.replace("\"known\": [\n", &format!("\"known\": [\n          \"{}\",\n", GHOST))
        };
        let mut reloaded = load(&doctored).expect("a save with an unknown tech id must still load");
        run_months(&mut reloaded, 12);
        for n in reloaded.nations.iter().filter(|n| n.alive) {
            assert!(n.gdp.is_finite() && n.gdp > 0.0, "{:?} broke after reload", n.id);
        }
    }

    #[test]

    fn saves_name_nations_rather_than_numbering_them() {
        // The same lesson as `saves_name_technologies_rather_than_numbering_them`,
        // and the reason `NationId` serializes as its code. A nation's id is an
        // index into the roster and into the relations matrix; inserting one
        // country in the middle of the roster moves every later index. A save
        // that stored those indices would come back describing a different
        // world, with nothing to detect it.
        let mut w = world_1990(GameRules::default());
        run_months(&mut w, 120);
        let text = save(&w);

        assert!(text.contains("\"id\": \"USA\""), "a nation's id is not written by name");
        // The relations matrix in particular: dense in memory, named on disk.
        assert!(
            text.contains("\"USSR\""),
            "the relations triples are not carrying codes"
        );
        // No nation is written as a bare number anywhere.
        assert!(!text.contains("\"id\": 0"), "save is storing raw roster indices");

        // A code from a build with a country this one does not have must be
        // dropped from the relations matrix rather than resolved onto its
        // neighbour, leaving a world that still loads and still runs.
        const GHOST: &str = "xx_nation_from_a_later_build";
        let doctored = text.replacen(
            "\"relations\": [",
            &format!("\"relations\": [\n    [\n      \"{}\",\n      \"USA\",\n      -40.0\n    ],", GHOST),
            1,
        );
        assert_ne!(doctored, text, "the save has no relations to doctor");
        let mut reloaded =
            load(&doctored).expect("a save naming an unknown nation must still load");
        // The ghost was dropped, not mapped onto whoever holds slot zero.
        assert_eq!(
            reloaded.relation(NationId::USA, NationId::USSR),
            w.relation(NationId::USA, NationId::USSR),
            "an unresolvable code was reinterpreted as a real nation"
        );
        run_months(&mut reloaded, 12);
        for n in reloaded.nations.iter().filter(|n| n.alive) {
            assert!(n.gdp.is_finite() && n.gdp > 0.0, "{:?} broke after reload", n.id);
        }
    }

    /// The six mature 1990 economies, and what each compounded over `years`,
    /// one row per seed. `MATURE_1990` is the panel the frontier band is quoted
    /// against.
    const MATURE_1990: [NationId; 6] = [
        NationId::USA, NationId::Japan, NationId::Germany,
        NationId::France, NationId::UK, NationId::Italy,
    ];

    /// Per seed: the fastest and the slowest mature economy's 35-year CAGR, as
    /// percent. A dead nation is skipped rather than counted as a stall.
    ///
    /// `years` of zero is the control arm: a world nobody runs has compounded
    /// nothing, and every reading must be exactly 0.0%. It is a small guard and
    /// it is aimed at a specific failure — this instrument reads `gdp` twice out
    /// of two separately built worlds, and if the 1990 read and the terminal
    /// read ever stop referring to the same quantity (a rescale at init, a unit
    /// change, a panel member silently replaced) the band would keep passing
    /// while measuring nothing. The zero-horizon arm is the only thing here that
    /// would notice.
    fn mature_cagr(years: usize) -> (Vec<f64>, Vec<f64>) {
        let (mut fastest, mut slowest) = (vec![], vec![]);
        for seed in 0..10u64 {
            let start: Vec<(NationId, f64)> = {
                let w = seeded(seed);
                MATURE_1990.iter().map(|id| (*id, w.nation(*id).gdp)).collect()
            };
            let mut w = seeded(seed);
            run_months(&mut w, 12 * years);
            let mut rates: Vec<f64> = vec![];
            for (id, gdp_1990) in start {
                let n = w.nation(id);
                if !n.alive {
                    continue;
                }
                let cagr = if years == 0 {
                    n.gdp / gdp_1990 - 1.0
                } else {
                    exact::powf(n.gdp / gdp_1990, 1.0 / years as f64) - 1.0
                };
                rates.push(cagr * 100.0);
            }
            rates.sort_by(|a, b| a.partial_cmp(b).unwrap());
            fastest.push(*rates.last().expect("every mature economy died"));
            slowest.push(rates[0]);
        }
        fastest.sort_by(|a, b| a.partial_cmp(b).unwrap());
        slowest.sort_by(|a, b| a.partial_cmp(b).unwrap());
        (fastest, slowest)
    }

    /// The guard that was missing, and whose absence let the world run at twice
    /// its real size undetected for weeks. Every other calibration test asserts
    /// a *relative* outcome — China grows faster than Japan, Slovenia escapes
    /// what Bosnia does not — so a world where everyone doubles together passes
    /// all of them. This one is absolute.
    ///
    /// THE ANCHOR, unchanged and quoted from the version this replaces: "Real
    /// 35-year growth for these economies runs about 0.9%/yr (Japan) to 2.5%/yr
    /// (USA). The ceiling here is 4.0% rather than 3.0% because Japan is a known
    /// outstanding gap at ~3.0% (see ROADMAP), and a test that is red on arrival
    /// teaches nothing." Both bounds are inherited at exactly 4.0% and 0.5%.
    /// Neither was re-derived here, and neither may be re-derived to accommodate
    /// a board that has started compounding — that is what a calibration pass is
    /// for, and this is the instrument that tells one it is needed.
    ///
    /// Converted from one seed to ten 2026-08-31, PLAN step 1, and the reason is
    /// that it was GREEN BY SEED LUCK. It ran the default seed only, and a
    /// ten-seed sweep of the same quantity spread 3.07 .. 3.71 on the board this
    /// conversion was written against — a fifth of a point of headroom under the
    /// ceiling, decided by which seed somebody had typed.
    ///
    /// BOTH THE MEDIAN AND THE WORST SEED ARE ASSERTED, and the second is not
    /// redundant. PLAN step 1 asks for a median, and a median is the honest
    /// reading of "is the typical world compounding". But the test this replaces
    /// was absolute over every mature economy in its one world, so asserting
    /// only a median would QUIETLY WIDEN it from "no advanced economy runs away"
    /// to "the average one does not" — and two hot seeds in ten would then hide
    /// behind eight cold ones. Iron rule 5 forbids buying a conversion with a
    /// loosening, so the per-seed guarantee is kept alongside the median it was
    /// converted to.
    #[test]
    fn the_frontier_does_not_run_away() {
        let (fastest, slowest) = mature_cagr(35);
        let (fast_med, slow_med) = (fastest[fastest.len() / 2], slowest[slowest.len() / 2]);
        let show = |v: &[f64]| v.iter().map(|x| (x * 100.0).round() / 100.0).collect::<Vec<_>>();

        assert!(
            fast_med < 4.0,
            "the fastest mature economy compounded a median {:.2}%/yr over 35 years across ten \
             seeds {:?} — the frontier is running away",
            fast_med, show(&fastest)
        );
        assert!(
            *fastest.last().unwrap() < 4.0,
            "the fastest mature economy compounded {:.2}%/yr over 35 years on the worst of ten \
             seeds {:?} — the frontier runs away in some worlds, and a median of eight quiet \
             ones is not a licence to stop looking",
            fastest.last().unwrap(), show(&fastest)
        );
        assert!(
            slow_med > 0.5,
            "the slowest mature economy compounded a median {:.2}%/yr across ten seeds {:?} — a \
             developed economy has stalled",
            slow_med, show(&slowest)
        );
        assert!(
            slowest[0] > 0.5,
            "the slowest mature economy compounded {:.2}%/yr on the worst of ten seeds {:?} — a \
             developed economy has stalled",
            slowest[0], show(&slowest)
        );

        // The control arm: a world nobody ran compounded nothing.
        let (c_fast, c_slow) = mature_cagr(0);
        let worst = c_fast.iter().chain(c_slow.iter()).fold(0.0f64, |a, b| a.max(b.abs()));
        assert!(
            worst < 1e-9,
            "thirty-five years of growth appeared in a world that was never ticked: {:.9}%",
            worst
        );
    }

    /// THE CONVERGENCE CHANNEL, read end to end on one board: what each nation
    /// was authored to know on 1 January 1990, the opening gap to the frontier
    /// that implies, and the 35-year CAGR that follows from it, across ten
    /// seeds rather than one.
    ///
    /// This exists because the channel was found reading the TRANSCRIPTION'S
    /// INCOMPLETENESS as a nation's IGNORANCE: overpayment ordered exactly by
    /// `gap`, and `gap` ordered by how many technologies a researcher happened
    /// to author. A readout is the only way to tell the two apart, because
    /// every calibration test downstream sees the sum.
    ///
    /// `cargo test --release -p spheres-sim convergence_channel_readout -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn convergence_channel_readout() {
        use std::collections::BTreeMap;

        // ---- the authored board, before a single tick ----
        let w0 = world_1990(GameRules::default());
        let frontier = w0.nations.iter().map(|n| n.tech.count()).max().unwrap_or(0);
        let mut hist: BTreeMap<usize, Vec<&'static str>> = BTreeMap::new();
        for n in w0.nations.iter() {
            hist.entry(n.tech.count()).or_default().push(n.id.code());
        }
        println!("\n=== AUTHORED 1990 BOARD ===");
        println!("nations {}  frontier_known {}  total grants {}",
            w0.nations.len(), frontier,
            w0.nations.iter().map(|n| n.tech.count()).sum::<usize>());
        for (count, codes) in &hist {
            println!("  {:>2} authored : {:>3} nations : {}", count, codes.len(),
                if codes.len() > 12 { format!("{} ...", codes[..12].join(" ")) }
                else { codes.join(" ") });
        }

        // ---- the panel the finding tabulates ----
        let panel: Vec<NationId> = ["USA", "Japan", "Germany", "UK", "France", "Italy",
                                    "Brazil", "Kenya"]
            .iter().map(|s| NationId::parse(s).expect("panel nation is on the roster")).collect();

        let mut cagr: BTreeMap<&'static str, Vec<f64>> = BTreeMap::new();
        for seed in 0..10u64 {
            let mut w = seeded(seed);
            let g0: Vec<(NationId, f64)> = panel.iter().map(|id| (*id, w.nation(*id).gdp)).collect();
            run_months(&mut w, 12 * 35);
            for (id, g) in g0 {
                let n = w.nation(id);
                let v = if n.alive {
                    (exact::powf(n.gdp / g, 1.0 / 35.0) - 1.0) * 100.0
                } else {
                    f64::NAN
                };
                cagr.entry(id.code()).or_default().push(v);
            }
        }

        println!("\n=== CONVERGENCE CHANNEL, TEN SEEDS ===");
        println!("{:<10} {:>8} {:>6} {:>8} {:>8} {:>8}",
            "nation", "authored", "gap", "cagr_lo", "cagr_med", "cagr_hi");
        for id in &panel {
            let held = w0.nation(*id).tech.count();
            let gap = if frontier > 0 {
                ((frontier - held) as f64 / frontier as f64).clamp(0.0, 1.0)
            } else { 0.0 };
            let mut xs = cagr[id.code()].clone();
            xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
            println!("{:<10} {:>8} {:>6.2} {:>8.2} {:>8.2} {:>8.2}   by seed {:?}",
                id.code(), held, gap, xs[0], xs[xs.len() / 2], xs[xs.len() - 1],
                cagr[id.code()].iter().map(|x| (x * 100.0).round() / 100.0)
                    .collect::<Vec<_>>());
        }

        // ---- what the convergence channel actually paid out, integrated ----
        // The gap is a stock that drains as a nation catches up, so reading it
        // at either end says nothing about what it was worth on the way. This
        // sums the adoption term month by month over the whole run: it IS the
        // overpayment, in points of annual trend accumulated, and it is the
        // number that has to fall if closing the edge in the data worked.
        println!("\n=== ADOPTION PAID, summed monthly over 35 years, seed 0 ===");
        let mut w = seeded(0);
        let mut paid: BTreeMap<&'static str, f64> = BTreeMap::new();
        let mut gap_years: BTreeMap<&'static str, f64> = BTreeMap::new();
        for _ in 0..(12 * 35) {
            tick_month(&mut w, &[]);
            let reference = crate::tech::world_reference(&w.nations);
            let front = w.nations.iter().map(|n| n.tech.count()).max().unwrap_or(0);
            for id in &panel {
                let n = w.nation(*id);
                let level = crate::tech::saturated_tech_tfp(n) - reference;
                let adoption = n.tfp_trend - n.tech.tfp_base - level;
                *paid.entry(id.code()).or_insert(0.0) += adoption / 12.0;
                let g = if front > 0 {
                    ((front - n.tech.count()) as f64 / front as f64).clamp(0.0, 1.0)
                } else { 0.0 };
                *gap_years.entry(id.code()).or_insert(0.0) += g / 12.0;
            }
        }
        println!("{:<10} {:>14} {:>12}", "nation", "adoption-years", "gap-years");
        for id in &panel {
            println!("{:<10} {:>14.5} {:>12.2}", id.code(), paid[id.code()],
                gap_years[id.code()]);
        }

        // ---- where the growth actually comes from, decomposed ----
        // `apply_bonuses` assembles the trend as
        //     tfp_trend = tfp_base + (saturated_tech_tfp - reference) + adoption
        // and the loader's rebase subtracts the 1990 endowment's value out of
        // `tfp_base`, so at 1990 the level term cancels exactly and ADOPTION IS
        // THE WHOLE OF WHAT AN AUTHORED BOARD BUYS. Differencing the identity
        // recovers the adoption term without reaching into private constants,
        // and printing all three at both ends is the only way to tell "the
        // overpayment closed" apart from "the overpayment moved".
        let decomp = |w: &WorldState, id: NationId| -> (usize, f64, f64, f64, f64) {
            let reference = crate::tech::world_reference(&w.nations);
            let n = w.nation(id);
            let level = crate::tech::saturated_tech_tfp(n) - reference;
            let adoption = n.tfp_trend - n.tech.tfp_base - level;
            (n.tech.count(), n.tfp_trend, n.tech.tfp_base, level, adoption)
        };
        let mut end = seeded(0);
        run_months(&mut end, 12 * 35);
        println!("\n=== TREND DECOMPOSITION, seed 0 ===");
        println!("{:<10} {:>26} {:>34}", "", "-------- 1990 --------",
            "------------- 2025 -------------");
        println!("{:<10} {:>5} {:>9} {:>9} {:>5} {:>9} {:>9} {:>9}",
            "nation", "techs", "adopt", "trend", "techs", "adopt", "level", "trend");
        for id in &panel {
            let (c0, t0, _b0, _l0, a0) = decomp(&w0, *id);
            let (c1, t1, _b1, l1, a1) = decomp(&end, *id);
            println!("{:<10} {:>5} {:>9.5} {:>9.5} {:>5} {:>9.5} {:>9.5} {:>9.5}",
                id.code(), c0, a0, t0, c1, a1, l1, t1);
        }

        // ---- the sanctions counterfactual the red band is made of ----
        // Exactly `sanction_loss`'s control arm, printed rather than differenced:
        // the band moved because the UNSANCTIONED Brazil moved, not because the
        // sanctions coefficient did, and only the base tells the two apart.
        let target = NationId::Brazil;
        let mut base: Vec<f64> = vec![];
        for seed in 0..10u64 {
            let mut control = seeded(seed);
            control.rules.ai_aggression = 0.0;
            let c0 = control.nation(target).gdp;
            run_months(&mut control, 240);
            base.push((exact::powf(control.nation(target).gdp / c0, 1.0 / 20.0) - 1.0) * 100.0);
        }
        let mut sorted = base.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        println!("\nBrazil UNSANCTIONED 20y CAGR at ai_aggression=0: median {:.2}%  {:?}",
            sorted[sorted.len() / 2],
            base.iter().map(|x| (x * 100.0).round() / 100.0).collect::<Vec<_>>());

        // ...and the instrument built on top of it, so the counterfactual and
        // the band it feeds are read out together. The coefficient is not in
        // this number: the loss is `base - treated`, and a base that moves on
        // its own moves the band without anything about sanctions changing.
        let coalition =
            [NationId::USA, NationId::UK, NationId::France, NationId::Germany, NationId::Japan];
        let (mut losses, mut shares) = sanction_loss(&coalition, target);
        losses.sort_by(|a, b| a.partial_cmp(b).unwrap());
        shares.sort_by(|a, b| a.partial_cmp(b).unwrap());
        println!("sanctions_cost_the_target_real_growth: median loss {:.3}pt \
                  (bar 1.2 < x < 2.5), G5 share {:.3}  {:?}",
            losses[losses.len() / 2], shares[shares.len() / 2],
            losses.iter().map(|x| (x * 100.0).round() / 100.0).collect::<Vec<_>>());
        let med_base = sorted[sorted.len() / 2];
        println!("  fraction of Brazil's own growth removed: {:.1}%",
            losses[losses.len() / 2] / med_base * 100.0);
    }

    /// Is `gulf_war_emerges` measuring a rate, or measuring one seed crossing a
    /// bar? Ten seeds against a bar of five discriminates nothing finer than a
    /// tenth, so when a data change moves it the only honest follow-up is a
    /// wider scan of the same quantity.
    ///
    /// Widened to four hundred seeds on 2026-08-31, alongside the bar's move to
    /// two hundred, and it now prints the per-seed variance the sample size is
    /// derived from as well as the narrower windows the bar used to read, so a
    /// future session can re-derive that size rather than inherit it on trust.
    ///
    /// `cargo test --release -p spheres-sim gulf_war_incidence_scan -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn gulf_war_incidence_scan() {
        const N: u64 = 400;
        let hits = gulf_wars(0..N, 1.0);
        let p = hits.len() as f64 / N as f64;
        println!("Iraq invades Kuwait in {}/{} seeds: p = {:.4}", hits.len(), N, p);
        // Bernoulli, so the per-seed variance IS p(1-p) — measured, not assumed.
        let var = p * (1.0 - p);
        println!("  per-seed variance p(1-p) = {:.4}, sd = {:.4}", var, var.sqrt());
        // What sample the 50% doctrinal bar needs for a false red under 1%:
        // the bar sits (p - 0.5) above the truth, so P(count < N/2) = Phi(
        // -(p - 0.5) * sqrt(N) / sd ), and 2.326 sd is the 1% tail.
        let need = (2.326 * var.sqrt() / (p - 0.5)).powi(2);
        println!("  seeds needed for P(false red) < 1% against the 50% bar: {:.0}", need.ceil());
        for w in [10u64, 40, 100, 200] {
            println!(
                "  first {:>3} (bar {:>3}): {}/{}",
                w,
                w / 2,
                hits.iter().filter(|s| **s < w).count(),
                w
            );
        }
    }

    /// The readout the ten-region integration was judged on, kept so the
    /// calibration commit that follows it does not have to rebuild the
    /// instrument. Every quantity here belongs to a test that moved when the
    /// roster went from 31 nations to 108; printing them across ten seeds is
    /// what separates "this test reads one seed and the seed moved" from "the
    /// model is roster-size dependent". Measured values at 31, 91 and 108
    /// nations are in the comment on `golden_hash_of_a_known_run`.
    ///
    /// `cargo test --release -p spheres-sim roster_scale_readout -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn roster_scale_readout() {
        let world_gdp: f64 =
            world_1990(GameRules::default()).nations.iter().filter(|n| n.alive).map(|n| n.gdp).sum();
        println!("world 1990 GDP ${:.0}bn over {} nations", world_gdp, nations::nation_count());

        for id in [NationId::USA, NationId::Japan, NationId::Germany,
                   NationId::France, NationId::UK, NationId::Italy] {
            let mut xs: Vec<f64> = vec![];
            for seed in 0..10u64 {
                let mut w = world_1990(GameRules { seed, ..GameRules::default() });
                let g0 = w.nation(id).gdp;
                run_months(&mut w, 12 * 35);
                let n = w.nation(id);
                xs.push(if n.alive {
                    (exact::powf(n.gdp / g0, 1.0 / 35.0) - 1.0) * 100.0
                } else {
                    f64::NAN
                });
            }
            let hot = xs.iter().filter(|x| x.is_finite() && **x >= 4.0).count();
            println!("35y CAGR {:<8?} {:?}  seeds at or over 4.0%: {}", id,
                xs.iter().map(|x| (x * 100.0).round() / 100.0).collect::<Vec<_>>(), hot);
        }

        let mut xs: Vec<f64> = vec![];
        for seed in 0..10u64 {
            let mut w = world_1990(GameRules { seed, ..GameRules::default() });
            let g0 = w.nation(NationId::China).gdp;
            run_months(&mut w, 360);
            xs.push(w.nation(NationId::China).gdp / g0);
        }
        let mut sorted = xs.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        println!("China 30y multiple {:?} median {:.2}",
            xs.iter().map(|x| (x * 100.0).round() / 100.0).collect::<Vec<_>>(),
            (sorted[4] + sorted[5]) / 2.0);

        for seed in [0u64, 1, 2, 3, 4, 5, 6, 7, 8, 9, 42, 1990] {
            let mut w = world_1990(GameRules { seed, ..GameRules::default() });
            run_months(&mut w, 360);
            let mut rows: Vec<(&str, usize)> = w.nations.iter().filter(|n| n.alive)
                .map(|n| (n.id.name(), n.tech.count())).collect();
            rows.sort_by_key(|(_, c)| *c);
            println!("tech floor seed {:<4} poorest {:?} frontier {}",
                seed, &rows[..3.min(rows.len())], rows.last().unwrap().1);
        }

        // The two single-seed assertions that moved with this refit, on the one
        // seed each of them actually reads.
        for seed in [1990u64, 42] {
            let mut w = world_1990(GameRules { seed, ..GameRules::default() });
            let g0: Vec<(NationId, f64)> = [
                NationId::USA, NationId::Japan, NationId::Germany,
                NationId::France, NationId::UK, NationId::Italy,
            ].iter().map(|id| (*id, w.nation(*id).gdp)).collect();
            run_months(&mut w, 12 * 35);
            let mut line = format!("mature CAGR seed {:<4} ", seed);
            for (id, g) in g0 {
                let n = w.nation(id);
                if !n.alive { continue; }
                line.push_str(&format!("{}={:.2} ", id.name(),
                    (exact::powf(n.gdp / g, 1.0 / 35.0) - 1.0) * 100.0));
            }
            println!("{}", line);
        }
    }

    /// Points of annual growth a coalition costs its target over twenty years,
    /// and the coalition's mean weight in world output, one entry per seed.
    ///
    /// An EMPTY coalition is the control arm: the treated world then runs the
    /// same 240 ticks on the same seed as the untreated one and must come out
    /// the same world, so the loss must be exactly zero. That is the same guard
    /// `arms_transfers_build_a_client_army` gets from a pledge of nothing.
    fn sanction_loss(coalition: &[NationId], target: NationId) -> (Vec<f64>, Vec<f64>) {
        let mut losses = vec![];
        let mut shares = vec![];
        for seed in 0..10u64 {
            let mut control = seeded(seed);
            control.rules.ai_aggression = 0.0;
            let c0 = control.nation(target).gdp;
            run_months(&mut control, 240);
            let base = exact::powf(control.nation(target).gdp / c0, 1.0 / 20.0) - 1.0;

            let mut treated = seeded(seed);
            treated.rules.ai_aggression = 0.0;
            let t0 = treated.nation(target).gdp;
            let mut share_acc = 0.0;
            for _ in 0..240 {
                for i in coalition {
                    if !treated.is_sanctioning(*i, target) {
                        treated.sanctions.push((*i, target));
                    }
                }
                // Sampled before the tick: economy runs first, politics last, and
                // politics is what lifts the regime.
                share_acc += treated.sanction_weight(target);
                tick_month(&mut treated, &[]);
            }
            let after = exact::powf(treated.nation(target).gdp / t0, 1.0 / 20.0) - 1.0;
            losses.push((base - after) * 100.0);
            shares.push(share_acc / 240.0);
        }
        (losses, shares)
    }

    /// THE PIN THAT DID NOT EXIST. `sanction_drag` in economy.rs was changed
    /// from counting flags to weighing output, and while auditing that change
    /// the whole suite was run at bite 0.000, 0.010, 0.015, 0.020, 0.025 and
    /// 0.030 to find what constrained it. Above 0.020 things go red — see the
    /// table on `china_growth_miracle`. BELOW it, nothing did: at bite 0.000,
    /// with sanctions costing a target no growth whatsoever, the entire suite
    /// passed except the golden hash. `embargo_starves_the_aggressor_and_
    /// outlasts_the_war` does not catch it because Iraq is a petro-state and its
    /// pain arrives through `oil_blockade`, which is a different term.
    ///
    /// So a coefficient could be driven to zero — sanctions made a diplomatic
    /// gesture with no economic content, in a game whose namesake system is
    /// spheres of influence — and every test would have stayed green. This is
    /// that missing guard, and it is deliberately two-sided.
    ///
    /// No war, no oil: Brazil is not a producer, so `embargo_drag` and
    /// `oil_export_share` are both out of the picture and what is left is the
    /// growth drag this test is here to pin. The regime is held in place each
    /// month because `politics.rs` lifts sanctions as grievance decays, and this
    /// measures the price of a regime rather than its lifespan.
    ///
    /// The band is set from reality, not from the model. A coalition weighing
    /// half of world output is the G5 of 1990; the two clean non-oil regimes of
    /// the period are the United States alone against China in 2018-19 (~24% of
    /// world output, growth 6.7% -> 6.0%, about 0.6pt) and the near-universal
    /// embargo of South Africa in 1985-93 (~80%, about 2.5pt against trend).
    /// Scaled to half the world economy those bracket roughly 1.2pt to 1.6pt,
    /// and the ceiling is set at 2.5pt rather than 1.6pt because three further
    /// sanction channels still count flags and add their own cost on top.
    ///
    /// THE CONTROL ARM, added 2026-08-31 with the PLAN step 1 conversion. The
    /// median was already here; what was missing was the other half of the
    /// template — a treatment of nothing that must produce an effect of nothing.
    /// The band above cannot be cleared by an instrument that is quietly
    /// measuring its own perturbation, and until the empty coalition was run
    /// nothing in this test would have noticed.
    ///
    /// Checked red in BOTH directions by moving SANCTION_BITE and running THIS
    /// test, not a proxy for it — points of annual growth lost by Brazil:
    ///      bite 0.000 ->  0.71pt   RED (floor)
    ///      bite 0.010 ->  0.79pt   RED (floor)
    ///      bite 0.015 ->  0.83pt   RED (floor)
    ///      bite 0.020 ->  1.87pt   green  (shipped)
    ///      bite 0.025 ->  1.74pt   green
    ///      bite 0.030 ->  1.73pt   green
    ///      bite 0.035 ->  1.98pt   green
    ///      bite 0.040 ->  2.83pt   RED (ceiling)
    /// So the band admits roughly 0.016..0.038 and rejects outside it, deleting
    /// the term outright included.
    ///
    /// RE-MEASURED 2026-08-31 on the pre-endowment board at 137 nations, with
    /// the control arm added. The table above no longer holds — the tree and the
    /// roster have moved under it and the instrument has got COARSER, which is
    /// worth knowing before anybody quotes the old rows:
    ///      bite 0.000 ->  0.59pt   RED (floor)
    ///      bite 0.010 ->           green
    ///      bite 0.015 ->           green   (was RED in the table above)
    ///      bite 0.020 ->  1.94pt   green  (shipped)
    ///      bite 0.040 ->  2.61pt   RED (ceiling)
    /// It still rejects the change it exists to reject — a coefficient driven to
    /// zero is caught, with 0.59pt against a 1.2pt floor — but it now admits
    /// roughly 0.005..0.035 rather than 0.016..0.038, so it discriminates half a
    /// coefficient and no longer a third of one. That is a loss of resolution in
    /// the world, not in the test, and it is recorded rather than repaired here.
    ///
    /// Note the jump between 0.015 and 0.020 and the dip at 0.025-0.030: this is
    /// a whole-world run and the response is not a clean slope, because a
    /// sanctioned Brazil changes what else happens in the world. The instrument
    /// discriminates a third of a coefficient reliably and a twentieth of one
    /// not at all, which is the resolution this suite generally has. The held
    /// single-nation measurement in `sanction_cost_calibration` is monotone in
    /// the bite and is the cleaner readout of the same quantity; this is the one
    /// that runs on every commit.
    #[test]
    fn sanctions_cost_the_target_real_growth() {
        // Was one seed and an absolute bar, and the influence merge is what
        // finally broke it: the default seed read 0.95 against a floor of 1.2, so
        // the test went red on a model that had just got BETTER. Measured across
        // ten seeds either side of that merge:
        //
        //     before   median 1.93   spread 0.18 .. 3.36
        //     after    median 3.01   spread 0.99 .. 4.16
        //
        // Every figure rose. The influence branch sealed a hole where a
        // sanctioned state could sign fresh trade agreements outside the
        // coalition and collect the full level gain, so that being embargoed
        // could make a target grow FASTER. With that shut, sanctions bite harder.
        // The single seed simply landed on the bottom of a distribution that had
        // moved up underneath it.
        //
        // So it is a median now: the shape `arms_transfers_build_a_client_army`
        // already uses and the remedy ROADMAP section 8 prescribes. This is a
        // STRENGTHENING and not a widening -- the bar stays at 1.2 points, and it
        // now has to be cleared by the median of ten runs rather than by
        // whichever single seed somebody picked.
        //
        // NOTE FOR A LATER CALIBRATION PASS, deliberately not acted on here: 3.0
        // points is above what the anchors predict. The two clean regimes of the
        // period are the US alone against China at ~24% of world output for
        // ~0.6pt, and the near-universal embargo of South Africa at ~80% for
        // ~2.5pt. A G5 at 51% interpolates to roughly 1.5pt and the model reads
        // double that. Sanctions may now be too strong; that is a coefficient
        // question, and moving one to chase it while landing a merge is how
        // calibration gets lost.
        let target = NationId::Brazil;
        let coalition =
            [NationId::USA, NationId::UK, NationId::France, NationId::Germany, NationId::Japan];

        let (mut losses, mut shares) = sanction_loss(&coalition, target);
        losses.sort_by(|a, b| a.partial_cmp(b).unwrap());
        shares.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let lost = losses[losses.len() / 2];
        let mean_share = shares[shares.len() / 2];

        assert!(
            (0.45..0.60).contains(&mean_share),
            "the G5 stopped weighing half the world (median {:.3}); the anchors below are \
             quoted at that weight and no longer apply",
            mean_share
        );
        assert!(
            lost > 1.2,
            "half the world economy shut its doors to {} for twenty years and cost it \
             a median {:.2} points of annual growth across ten seeds {:?}. Sanctions \
             have stopped being an economic instrument.",
            target.name(), lost,
            losses.iter().map(|x| (x * 100.0).round() / 100.0).collect::<Vec<_>>()
        );
        assert!(
            lost < 2.5,
            "a sanctions regime cost {} a median {:.2} points of annual growth across \
             ten seeds {:?} — more than the near-universal embargo of South \
             Africa managed, from a coalition weighing {:.0}% of world output",
            target.name(), lost,
            losses.iter().map(|x| (x * 100.0).round() / 100.0).collect::<Vec<_>>(),
            mean_share * 100.0
        );

        // ...and the control arm, which is what keeps this from being a test that
        // cannot fail. `arms_transfers_build_a_client_army` pledges nothing and
        // requires a ratio of one; the same move here is a coalition of nobody.
        // With an empty coalition the treated world runs the identical tick
        // sequence on the identical seed, so the two worlds are the same world
        // and the loss is exactly zero — not approximately. Anything else means
        // the measurement itself is writing to the world it is measuring.
        let (control_losses, control_shares) = sanction_loss(&[], target);
        let worst = control_losses.iter().fold(0.0f64, |a, b| a.max(b.abs()));
        assert!(
            worst < 1e-9,
            "sanctioning nobody moved {}'s growth by {:.6} points — the instrument is \
             perturbing the world it measures: {:?}",
            target.name(), worst, control_losses
        );
        assert!(
            control_shares.iter().all(|s| *s == 0.0),
            "a coalition of nobody weighed something: {:?}",
            control_shares
        );
    }

    /// Is `china_growth_miracle` measuring growth, or measuring how often China
    /// gets into trouble? This is the readout that answered it, and the answer
    /// is worth keeping the instrument for: at `ai_aggression = 0.0` China
    /// finishes thirty years at a median 14.02x against the real 14.33x, so the
    /// growth model is right and the ten-seed median is a war-incidence figure.
    ///
    /// `cargo test --release -p spheres-sim china_trouble_readout -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn china_trouble_readout() {
        for aggr in [1.0f64, 0.0] {
            let mut xs: Vec<f64> = vec![];
            println!("--- ai_aggression = {:.1}", aggr);
            for seed in 0..10u64 {
                let mut w = world_1990(GameRules { seed, ..GameRules::default() });
                w.rules.ai_aggression = aggr;
                let g0 = w.nation(NationId::China).gdp;
                let mut war_m = 0;
                let mut sanc_m = 0;
                for _ in 0..360 {
                    tick_month(&mut w, &[]);
                    if w.at_war(NationId::China) {
                        war_m += 1;
                    }
                    sanc_m += w.sanctioned_by_count(NationId::China);
                }
                let x = w.nation(NationId::China).gdp / g0;
                println!(
                    "  seed {:<2} mult {:>6.2}  war-months {:>3}  sanction-months {:>4}",
                    seed, x, war_m, sanc_m
                );
                xs.push(x);
            }
            xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
            println!("  median {:.2}", (xs[4] + xs[5]) / 2.0);
        }
    }

    /// Who actually sanctions, and what do they weigh? Kept because the answer
    /// was the whole of the refit: the coalition that forms against China is
    /// always the same five — United States, United Kingdom, France, Germany,
    /// Japan — weighing about half of world output, and the old count-based rule
    /// charged it three full points of annual growth for fifteen years and more.
    ///
    /// `cargo test --release -p spheres-sim sanction_weight_readout -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn sanction_weight_readout() {
        for seed in [0u64, 2, 8, 1990] {
            let mut w = world_1990(GameRules { seed, ..GameRules::default() });
            println!("=== seed {}", seed);
            for m in 0..360 {
                tick_month(&mut w, &[]);
                if m % 60 != 0 || !w.sanctions.iter().any(|(_, t)| *t == NationId::China) {
                    continue;
                }
                let who: Vec<&str> = w
                    .sanctions
                    .iter()
                    .filter(|(_, t)| *t == NationId::China)
                    .map(|(i, _)| i.name())
                    .collect();
                println!(
                    "  m{:<4} count {} weight {:.3} : {:?}",
                    m,
                    w.sanctioned_by_count(NationId::China),
                    w.sanction_weight(NationId::China),
                    who
                );
            }
        }
    }

    #[test]
    fn an_idle_player_cannot_break_the_world() {
        // The configuration a human actually plays in, and the one nothing
        // covered: `a_century_holds_together` and `economic_invariants_50_years`
        // both run with `player = None`, so every central bank in them is on.
        // Seat a player and one switches off — by design, the rate is theirs to
        // set — and before `player_set_rate` a player who set nothing was
        // indistinguishable from one who had chosen. Holding 1990's 8% into a
        // deflation walked the United States to a GDP of -10.98 by June 2016,
        // then to a NaN mil_strength out of war.rs's sqrt, then to a browser
        // that would not load the save.
        //
        // Doing nothing is a thing players do. It has to produce a bad game,
        // not a broken one.
        for seat in [NationId::USA, NationId::Poland, NationId::Iraq] {
            let mut w = world_1990(GameRules::default());
            w.player = Some(seat);
            let start_gdp = w.nation(seat).gdp;
            for _ in 0..420 {
                tick_month(&mut w, &[]);
                for n in w.nations.iter().filter(|n| n.alive) {
                    // Everything `economic_invariants_50_years` holds the
                    // headless world to, held to the played one.
                    assert!(
                        n.gdp.is_finite() && n.gdp > 0.0,
                        "idle {:?}: {:?} gdp {} in {}", seat, n.id, n.gdp, w.year
                    );
                    assert!(n.inflation.is_finite(), "idle {:?}: {:?} inflation NaN", seat, n.id);
                    assert!(
                        n.debt_gdp.is_finite() && n.debt_gdp < 6.0,
                        "idle {:?}: {:?} debt spiral {} in {}", seat, n.id, n.debt_gdp, w.year
                    );
                    assert!(
                        (0.0..=100.0).contains(&n.stability),
                        "idle {:?}: {:?} stability {} in {}", seat, n.id, n.stability, w.year
                    );
                    // The surface the bug actually reached the player on: a NaN
                    // here is what serde writes as `null` and the UI dies on.
                    assert!(
                        n.mil_strength.is_finite() && n.mil_strength >= 0.0,
                        "idle {:?}: {:?} mil_strength {} in {}", seat, n.id, n.mil_strength, w.year
                    );
                    assert!(
                        n.population.is_finite() && n.population > 0.0,
                        "idle {:?}: {:?} population {}", seat, n.id, n.population
                    );
                    assert!(
                        n.political_capital.is_finite(),
                        "idle {:?}: {:?} political capital NaN", seat, n.id
                    );
                }
                // Finiteness alone is too weak to catch this one, and saying so
                // is the point. The first attempt at a fix clamped the *result*
                // — `gdp.max(0.001)` — which held every assertion above while
                // leaving the United States a $120bn economy sitting at 100.00
                // stability, because the terms that drove it to the floor went
                // on compounding underneath the clamp. A seat that has been
                // governed by nobody should be mediocre, not evaporated. The
                // margin is deliberately enormous — all three seats actually
                // trough at ~100% of their 1990 output — so this reads as a
                // shape check, not a growth target somebody has to retune.
                let n = w.nation(seat);
                if n.alive {
                    assert!(
                        n.gdp > start_gdp * 0.10,
                        "idle {:?}: seat has evaporated — gdp {} against {} at start, in {}",
                        seat, n.gdp, start_gdp, w.year
                    );
                }
            }
            // A NaN reaches the browser as a `null` the UI cannot render, so the
            // save is where this surfaced. Checking for the literal string would
            // be wrong — every `Option::None` in the state writes a legitimate
            // `null` — so this asserts the two things that matter instead: the
            // save parses back, and the world it parses back into is the same one.
            let text = save(&w);
            let reloaded = load(&text)
                .unwrap_or_else(|e| panic!("idle {:?}: save will not load: {}", seat, e));
            assert_eq!(
                state_hash(&reloaded), state_hash(&w),
                "idle {:?}: the save did not round-trip", seat
            );
        }
    }

    #[test]
    fn a_player_who_sets_a_rate_keeps_it() {
        // The other half of `player_set_rate`, and the reason it is a latch on
        // the command rather than a comparison against the AI's preferred rate:
        // once the player has governed, the bank must never speak again, even
        // when what they chose is exactly what it would have hated. 12% held
        // through a decade is a policy — a wrong one, probably — and it is not
        // the model's business to quietly walk it back to the Taylor rule.
        let mut w = world_1990(GameRules::default());
        w.player = Some(NationId::USA);
        apply_command(&mut w, &Command::SetInterestRate { nation: NationId::USA, rate: 0.12 })
            .expect("player sets their own rate");
        for _ in 0..120 {
            tick_month(&mut w, &[]);
            assert_eq!(
                w.nation(NationId::USA).interest_rate, 0.12,
                "the AI bank overwrote a rate the player had set, in {}", w.year
            );
        }
        // And a seat the player never touched is still governed for them.
        let mut idle = world_1990(GameRules::default());
        idle.player = Some(NationId::USA);
        for _ in 0..120 {
            tick_month(&mut idle, &[]);
        }
        assert!(
            idle.nation(NationId::USA).interest_rate != 0.08,
            "an unmanned seat kept 1990's rate: {}",
            idle.nation(NationId::USA).interest_rate
        );
    }

    #[test]
    fn a_pegged_rate_is_a_governed_rate() {
        // Setting a rate is not the only way to decide one. The currency peg
        // says in as many words that the rate stops floating, and it costs 26
        // political capital to say it — so a default bank that went on drifting
        // it would be selling the player something it then took back. It did:
        // pinned at 0.055, and 0.078 again six months later.
        let mut w = world_1990(GameRules::default());
        w.player = Some(NationId::Iraq);
        for _ in 0..24 {
            tick_month(&mut w, &[]);
        }
        w.nation_mut(NationId::Iraq).inflation = 0.30;
        w.nation_mut(NationId::Iraq).political_capital = 100.0;
        apply_command(
            &mut w,
            &Command::EnactStratagem { nation: NationId::Iraq, id: "currency_peg".to_string() },
        )
        .expect("the peg is available at 30% inflation");
        for _ in 0..24 {
            tick_month(&mut w, &[]);
            assert_eq!(
                w.nation(NationId::Iraq).interest_rate, 0.055,
                "the peg drifted in {}", w.year
            );
        }
    }

    /// Forty years across twenty-one seeds, looking for the shapes a calibration
    /// test does not look for: a value sitting on its clamp for years, the same
    /// two states at war forever, a dead nation still moving, an economy that
    /// ran away or evaporated.
    ///
    /// A readout, not an assertion — it prints what it finds and BUGS.md records
    /// what each finding turned out to mean. Ignored because it is a survey and
    /// wants release mode:
    ///
    /// ```text
    /// cargo test --release -p spheres-sim anomaly_sweep -- --ignored --nocapture
    /// ```
    #[test]
    #[ignore]
    fn anomaly_sweep() {
        use std::collections::BTreeMap;
        const MONTHS: usize = 480; // forty years
        const PINNED: u32 = 60; // five years on a bound before it is a finding
        const OIL_PINNED: u32 = 24;
        const RUNAWAY: f64 = 100.0; // x 1990 output; China's real miracle is ~14x/30y
        const EVAPORATED: f64 = 0.01; // fraction of 1990 output

        // (seed, category, who, year, detail). Deduped to the first occurrence of
        // each (seed, category, who) at the end, so one sick nation is one line
        // rather than four hundred.
        let mut raw: Vec<(u64, &'static str, String, i32, String)> = Vec::new();

        for seed in 0..=20u64 {
            let mut w = world_1990(GameRules { seed, ..GameRules::default() });
            let start: BTreeMap<NationId, f64> = w.nations.iter().map(|n| (n.id, n.gdp)).collect();
            let mut prev_dead: BTreeMap<NationId, (f64, f64)> = BTreeMap::new();
            let mut infl_lo: BTreeMap<NationId, u32> = BTreeMap::new();
            let mut infl_hi: BTreeMap<NationId, u32> = BTreeMap::new();
            let mut stab_lo: BTreeMap<NationId, u32> = BTreeMap::new();
            let mut stab_hi: BTreeMap<NationId, u32> = BTreeMap::new();
            let mut oil_lo: u32 = 0;
            let mut oil_hi: u32 = 0;
            let mut dyad: BTreeMap<(NationId, NationId), Vec<u32>> = BTreeMap::new();

            for _ in 0..MONTHS {
                tick_month(&mut w, &[]);
                let yr = w.year;

                if !w.oil_price.is_finite() || w.oil_price <= 0.0 {
                    raw.push((seed, "oil-nonfinite", "world".to_string(), yr, format!("{}", w.oil_price)));
                }
                oil_lo = if w.oil_price <= 8.0 { oil_lo + 1 } else { 0 };
                oil_hi = if w.oil_price >= 120.0 { oil_hi + 1 } else { 0 };
                if oil_lo == OIL_PINNED {
                    raw.push((seed, "oil-pinned-low", "world".to_string(), yr, "at the $8 floor for 24mo".to_string()));
                }
                if oil_hi == OIL_PINNED {
                    raw.push((seed, "oil-pinned-high", "world".to_string(), yr, "at the $120 ceiling for 24mo".to_string()));
                }

                for c in &w.conflicts {
                    let d = c.defender();
                    let key = if c.origin_attacker <= d {
                        (c.origin_attacker, d)
                    } else {
                        (d, c.origin_attacker)
                    };
                    let e = dyad.entry(key).or_default();
                    if !e.contains(&c.id) {
                        e.push(c.id);
                    }
                    if c.months == 300 {
                        raw.push((seed, "war-endless", format!("{:?} vs {:?}", c.origin_attacker, d), yr,
                                  "a single conflict running 300 months".to_string()));
                    }
                    for side in [&c.side_a, &c.side_b] {
                        for id in side {
                            if w.nation_opt(*id).is_some_and(|n| !n.alive) {
                                raw.push((seed, "dead-at-war", format!("{:?}", id), yr,
                                          format!("listed in live conflict {}", c.id)));
                            }
                        }
                    }
                }

                for n in w.nations.iter() {
                    let who = format!("{:?}", n.id);
                    if !n.alive {
                        // A dead nation has to be inert. If its numbers move, some
                        // loop is still writing to it.
                        if let Some((g, m)) = prev_dead.get(&n.id) {
                            if *g != n.gdp || *m != n.mil_strength {
                                raw.push((seed, "dead-still-moving", who, yr,
                                          format!("gdp {} -> {}, mil {} -> {}", g, n.gdp, m, n.mil_strength)));
                            }
                        }
                        prev_dead.insert(n.id, (n.gdp, n.mil_strength));
                        continue;
                    }
                    prev_dead.remove(&n.id);

                    if !n.gdp.is_finite() || n.gdp <= 0.0 {
                        raw.push((seed, "gdp-invalid", who.clone(), yr, format!("{}", n.gdp)));
                    }
                    if !n.inflation.is_finite() {
                        raw.push((seed, "inflation-nan", who.clone(), yr, "NaN".to_string()));
                    }
                    if !n.mil_strength.is_finite() || n.mil_strength < 0.0 {
                        raw.push((seed, "mil-invalid", who.clone(), yr, format!("{}", n.mil_strength)));
                    }
                    if !n.population.is_finite() || n.population <= 0.0 {
                        raw.push((seed, "pop-invalid", who.clone(), yr, format!("{}", n.population)));
                    }
                    if !n.political_capital.is_finite() {
                        raw.push((seed, "pc-nan", who.clone(), yr, "NaN".to_string()));
                    }
                    if !n.debt_gdp.is_finite() || n.debt_gdp > 6.0 {
                        raw.push((seed, "debt-spiral", who.clone(), yr, format!("{:.2}", n.debt_gdp)));
                    }
                    if !(0.0..=100.0).contains(&n.stability) {
                        raw.push((seed, "stability-range", who.clone(), yr, format!("{}", n.stability)));
                    }
                    if let Some(s0) = start.get(&n.id) {
                        if *s0 > 0.0 && n.gdp > *s0 * RUNAWAY {
                            raw.push((seed, "gdp-runaway", who.clone(), yr,
                                      format!("{:.0}x 1990 ({:.0} -> {:.0})", n.gdp / *s0, s0, n.gdp)));
                        }
                        if *s0 > 0.0 && n.gdp < *s0 * EVAPORATED {
                            raw.push((seed, "gdp-evaporated", who.clone(), yr,
                                      format!("{:.3}% of 1990 ({:.1} -> {:.3})", n.gdp / *s0 * 100.0, s0, n.gdp)));
                        }
                    }

                    let e = infl_lo.entry(n.id).or_default();
                    *e = if n.inflation <= -0.05 { *e + 1 } else { 0 };
                    if *e == PINNED {
                        raw.push((seed, "inflation-pinned-low", who.clone(), yr, "at the -5% clamp for 5y".to_string()));
                    }
                    let e = infl_hi.entry(n.id).or_default();
                    *e = if n.inflation >= 3.0 { *e + 1 } else { 0 };
                    if *e == PINNED {
                        raw.push((seed, "inflation-pinned-high", who.clone(), yr, "at the 300% clamp for 5y".to_string()));
                    }
                    let e = stab_lo.entry(n.id).or_default();
                    *e = if n.stability <= 0.0 { *e + 1 } else { 0 };
                    if *e == PINNED {
                        raw.push((seed, "stability-pinned-0", who.clone(), yr, "stability 0.0 for 5y".to_string()));
                    }
                    let e = stab_hi.entry(n.id).or_default();
                    *e = if n.stability >= 100.0 { *e + 1 } else { 0 };
                    if *e == PINNED {
                        raw.push((seed, "stability-pinned-100", who, yr, "stability 100.0 for 5y".to_string()));
                    }
                }
            }

            for ((a, b), ids) in &dyad {
                if ids.len() >= 5 {
                    raw.push((seed, "dyad-repeat-war", format!("{:?} vs {:?}", a, b), w.year,
                              format!("{} separate conflicts in 40y", ids.len())));
                }
            }
        }

        // One line per (seed, category, nation), at the year it first appeared.
        let mut seen: BTreeMap<(u64, &'static str, String), (i32, String)> = BTreeMap::new();
        for (seed, cat, who, yr, detail) in raw {
            seen.entry((seed, cat, who)).or_insert((yr, detail));
        }
        let mut by_cat: BTreeMap<&'static str, Vec<String>> = BTreeMap::new();
        for ((seed, cat, who), (yr, detail)) in &seen {
            by_cat.entry(cat).or_default().push(format!("seed {:>2} {} {} — {}", seed, yr, who, detail));
        }

        let roster = world_1990(GameRules::default()).nations.len();
        println!("\n=== ANOMALY SWEEP: seeds 0..=20, 40 years, {} nations ===", roster);
        if by_cat.is_empty() {
            println!("no anomalies found");
        }
        for (cat, mut lines) in by_cat {
            println!("\n[{}]  {} occurrence(s)", cat, lines.len());
            lines.sort();
            for l in lines.iter().take(12) {
                println!("   {}", l);
            }
            if lines.len() > 12 {
                println!("   ... and {} more", lines.len() - 12);
            }
        }
        println!();
    }

    /// Every annexation a seed range produces, as `(seed, victim, population at
    /// the month it died, year)`. Dissolutions are not conquests and are
    /// excluded by name: the USSR and Yugoslavia leave the board in every seed
    /// by a different door.
    ///
    /// `aggression` is the arm. At 1.0 the AI fights; at 0.0 it does not, and a
    /// world where nobody attacks anybody must annex nobody — that is the
    /// control, and it is the same code path rather than a second one.
    fn conquests(seeds: std::ops::Range<u64>, aggression: f64) -> Vec<(u64, NationId, f64, i32)> {
        let mut found = vec![];
        for seed in seeds {
            let mut w = seeded(seed);
            w.rules.ai_aggression = aggression;
            let mut alive: Vec<(NationId, f64)> =
                w.nations.iter().filter(|n| n.alive).map(|n| (n.id, n.population)).collect();
            for _ in 0..480 {
                tick_month(&mut w, &[]);
                let mut still: Vec<(NationId, f64)> = Vec::new();
                for (id, pop) in alive {
                    if w.nation_opt(id).is_some_and(|n| n.alive) {
                        // Carry the live figure forward, so a death is judged
                        // against the population it died at.
                        still.push((id, w.nation(id).population));
                    } else if id != NationId::USSR && id != NationId::Yugoslavia {
                        found.push((seed, id, pop, w.year));
                    }
                }
                alive = still;
            }
        }
        found
    }

    /// One `Ending::Conquest`, with the two figures the size rule weighed.
    struct ConquestEnding {
        seed: u64,
        year: i32,
        loser: NationId,
        /// The loser's population and separatism as `war::conquer` saw them.
        pop: f64,
        sep: f64,
        /// The same two a month earlier. Only the scan reads these: they are
        /// the control on the reading above, not a second opinion about the
        /// world. See the note on `conquest_endings`.
        pop_pre: f64,
        sep_pre: f64,
        /// True when the rule admitted the annexation, false when it refused
        /// and subjugated instead.
        annexed: bool,
    }

    /// Every `Ending::Conquest` a seed range produces. This is the size rule's
    /// whole caseload, not the tail of it that leaves the board.
    ///
    /// WHY HEADLINES ARE THE PROBE. `war::conquer` is reached from exactly one
    /// place — the `Ending::Conquest` arm of `war::tick` — and it writes
    /// exactly one of two headlines on every call: "X has annexed Y." when the
    /// rule admits the annexation, "Y capitulates to X" when it refuses. Their
    /// sum is therefore the count of conquest endings, and their split is the
    /// rule's verdict. `conquest_funnel` already counts them this way and its
    /// own printout labels them as such.
    ///
    /// WHY THE READING IS TAKEN AFTER THE TICK, WHICH MATTERS MORE HERE THAN IT
    /// LOOKS. The separatism boundary is genuinely tight — measured over seeds
    /// 0..240, the angriest nation ever annexed sits at 0.587 and the calmest
    /// ever refused for anger at 0.614, thirteen and fourteen thousandths either
    /// side of the 0.600 clause — so a reading off by a month is a false
    /// verdict, not a rounding error. (The population clause has room to spare
    /// by comparison: largest annexed 5.351m, smallest refused for size 8.735m.) It is exact because of the tick order: `population`
    /// is written by `economy` and `tech`, `separatism` by `economy`,
    /// `statecraft` and `stratagems`, and every one of those runs AHEAD of `war`
    /// in the `SYSTEMS` table while nothing behind it (`government`, `politics`)
    /// writes either. `conquer` itself leaves both untouched on the loser in
    /// both arms — the annex arm sets only `alive` and `gdp`, the subjugation
    /// arm only `gdp`, `mil_strength`, `stability` and `war_exhaustion`. So the
    /// post-tick value IS the decision-time value.
    ///
    /// The pre-tick pair is carried alongside so that claim is checked rather
    /// than asserted: `conquest_size_rule_scan` prints every row where the two
    /// readings disagree about the verdict. Over seeds 0..240 there are none.
    /// The one way they could is a loser that is also the WINNER of a
    /// negotiated peace concluded earlier in the same month's ending list,
    /// which is the only path by which `war.rs` moves a nation's separatism
    /// (+0.05) or population before `conquer` reads it.
    fn conquest_endings(
        seeds: std::ops::Range<u64>,
        aggression: f64,
    ) -> Vec<ConquestEnding> {
        let mut rows = vec![];
        for seed in seeds {
            let mut w = seeded(seed);
            w.rules.ai_aggression = aggression;
            let mut pre: std::collections::BTreeMap<NationId, (f64, f64)> = Default::default();
            for _ in 0..480 {
                pre.clear();
                pre.extend(w.nations.iter().map(|n| (n.id, (n.population, n.separatism))));
                let headlines = tick_month(&mut w, &[]);
                for h in &headlines {
                    let (loser_name, annexed) = if let Some(rest) = h.split(" has annexed ").nth(1)
                    {
                        (rest.trim_end_matches('.').to_string(), true)
                    } else if h.contains(" capitulates to ") {
                        (h.split(" capitulates to ").next().unwrap().to_string(), false)
                    } else {
                        continue;
                    };
                    let loser = w
                        .nations
                        .iter()
                        .map(|n| n.id)
                        .find(|id| id.name() == loser_name)
                        .unwrap_or_else(|| panic!("no nation named {:?}", loser_name));
                    let n = w.nation(loser);
                    let (pop_pre, sep_pre) =
                        pre.get(&loser).copied().unwrap_or((n.population, n.separatism));
                    rows.push(ConquestEnding {
                        seed,
                        year: w.year,
                        loser,
                        pop: n.population,
                        sep: n.separatism,
                        pop_pre,
                        sep_pre,
                        annexed,
                    });
                }
            }
        }
        rows
    }

    /// The size rule's caseload, its margins, and the sample sizes ruling 3
    /// makes the two bars carry. Prints every annexation, the refusals closest
    /// to each bound, and the per-seed variance both floors are derived from,
    /// so "the rule stopped refusing" and "conquest stopped happening" can be
    /// told apart and neither sample size has to be inherited on trust.
    ///
    /// ```text
    /// cargo test --release -p spheres-sim conquest_size_rule_scan -- --ignored --nocapture
    /// ```
    #[test]
    #[ignore]
    fn conquest_size_rule_scan() {
        const N: u64 = 240;
        let rows = conquest_endings(0..N, 1.0);
        let annexed: Vec<_> = rows.iter().filter(|r| r.annexed).collect();
        let refused: Vec<_> = rows.iter().filter(|r| !r.annexed).collect();
        println!("\n=== conquest endings, seeds 0..{}, 40 years ===", N);
        println!("  Ending::Conquest total : {}", rows.len());
        println!("  ...ANNEXED             : {}", annexed.len());
        println!("  ...REFUSED (subjugated): {}", refused.len());
        let seeds_with: std::collections::BTreeSet<u64> = rows.iter().map(|r| r.seed).collect();
        println!("  seeds reaching any     : {}/{}", seeds_with.len(), N);

        // THE CONTROL ON THE READING ITSELF. `conquest_endings` claims the
        // post-tick figures are the ones `conquer` compared against 8.0 and
        // 0.6. Here is that claim tested: the verdict the rule would reach off
        // last month's figures, against the verdict it actually reached.
        let mut drift = 0usize;
        for r in &rows {
            let would = |p: f64, s: f64| p < 8.0 && s < 0.6;
            if would(r.pop, r.sep) != would(r.pop_pre, r.sep_pre) {
                drift += 1;
                println!(
                    "  READING DRIFT seed {} {} {}: post {:.4}m/{:.4} vs pre {:.4}m/{:.4}",
                    r.seed,
                    r.loser.name(),
                    r.year,
                    r.pop,
                    r.sep,
                    r.pop_pre,
                    r.sep_pre
                );
            }
        }
        println!(
            "  rows whose verdict differs pre-tick vs post-tick: {} of {}",
            drift,
            rows.len()
        );
        // Per-seed counts, for the variance ruling 3 wants derived rather than
        // guessed. `need` is the seeds a floor of one needs for a false red
        // under 1%: the event is per-seed Bernoulli at the observed hit rate,
        // so P(none in n) = (1 - rate)^n and n = ln(0.01) / ln(1 - rate).
        let mut per_seed = vec![0usize; N as usize];
        let mut per_seed_annex = vec![0usize; N as usize];
        let mut per_seed_refused = vec![0usize; N as usize];
        for r in &rows {
            per_seed[r.seed as usize] += 1;
            if r.annexed {
                per_seed_annex[r.seed as usize] += 1;
            } else {
                per_seed_refused[r.seed as usize] += 1;
            }
        }
        for (label, v) in [
            ("conquests", &per_seed),
            ("annexations", &per_seed_annex),
            ("refusals", &per_seed_refused),
        ] {
            let n = v.len() as f64;
            let mean = v.iter().sum::<usize>() as f64 / n;
            let var = v.iter().map(|&x| (x as f64 - mean).powi(2)).sum::<f64>() / n;
            let rate = v.iter().filter(|&&x| x > 0).count() as f64 / n;
            // `crate::exact`, not the platform libm: `exact.rs` bans the
            // natural-log method across the crate, and its guard test walks the
            // whole source tree with test modules included.
            let need = if rate > 0.0 && rate < 1.0 {
                (crate::exact::ln(0.01) / crate::exact::ln(1.0 - rate)).ceil()
            } else {
                f64::NAN
            };
            println!(
                "  per-seed {:<12} mean {:.4}  var {:.4}  sd {:.4}  seeds-with {:.4}  \
                 n for P(zero)<1%: {:.0}",
                label,
                mean,
                var,
                var.sqrt(),
                rate,
                need
            );
        }
        // The floors the two bars will carry, and the z-score each stands at,
        // read off the variance above rather than chosen.
        for (label, v, floor) in [
            ("refusals >= 15 over 100 seeds", &per_seed_refused, 15.0f64),
            ("annexations >= 1 over 240 seeds", &per_seed_annex, 1.0),
        ] {
            let window = if floor == 15.0 { 100.0 } else { N as f64 };
            let n = v.len() as f64;
            let mean = v.iter().sum::<usize>() as f64 / n;
            let var = v.iter().map(|&x| (x as f64 - mean).powi(2)).sum::<f64>() / n;
            let (m, sd) = (mean * window, (var * window).sqrt());
            println!("  {:<34} expect {:.1} +/- {:.1}, z to floor {:.2}", label, m, sd, (m - floor) / sd);
        }
        // Sub-windows, so a future session can see what a narrower sample buys.
        for w in [20u64, 100, 200] {
            let sub: Vec<_> = rows.iter().filter(|r| r.seed < w).collect();
            println!(
                "  first {:>3} seeds: {:>3} endings, {:>2} annexed, {:>3} refused",
                w,
                sub.len(),
                sub.iter().filter(|r| r.annexed).count(),
                sub.iter().filter(|r| !r.annexed).count()
            );
        }
        println!("\n  --- every annexation ---");
        for r in &annexed {
            println!(
                "    seed {:>3}  {:<16} {:>7.3}m  sep {:.3}  in {}",
                r.seed,
                r.loser.name(),
                r.pop,
                r.sep,
                r.year
            );
        }
        // The two margins that decide whether this instrument is measuring
        // anything: how close the rule ever came to admitting what it should
        // refuse, on each of its two clauses separately.
        let widest_annexed_pop = annexed.iter().map(|r| r.pop).fold(0.0f64, f64::max);
        let angriest_annexed = annexed.iter().map(|r| r.sep).fold(0.0f64, f64::max);
        let calmest_refused =
            refused.iter().filter(|r| r.pop < 8.0).map(|r| r.sep).fold(f64::MAX, f64::min);
        let smallest_refused =
            refused.iter().filter(|r| r.sep < 0.6).map(|r| r.pop).fold(f64::MAX, f64::min);
        println!(
            "\n  largest annexed {:.3}m  (bound 8.000, margin {:.3}m)",
            widest_annexed_pop,
            8.0 - widest_annexed_pop
        );
        println!(
            "  smallest refused-on-size {:.3}m  (bound 8.000, margin {:+.3}m)",
            smallest_refused,
            smallest_refused - 8.0
        );
        println!(
            "  angriest annexed sep {:.3}  (bound 0.600, margin {:.3})",
            angriest_annexed,
            0.6 - angriest_annexed
        );
        println!(
            "  calmest refused-on-anger sep {:.3}  (bound 0.600, margin {:+.3})",
            calmest_refused,
            calmest_refused - 0.6
        );
        let mut near = refused.clone();
        near.sort_by(|a, b| a.pop.partial_cmp(&b.pop).unwrap());
        println!("\n  --- the ten refusals closest to the 8m bound ---");
        for r in near.iter().take(10) {
            println!(
                "    seed {:>3}  {:<16} {:>7.3}m  sep {:.3}  in {}  (refused because {})",
                r.seed,
                r.loser.name(),
                r.pop,
                r.sep,
                r.year,
                if r.pop >= 8.0 && r.sep >= 0.6 {
                    "both"
                } else if r.pop >= 8.0 {
                    "too big"
                } else {
                    "too angry"
                }
            );
        }
        let by_sep = refused.iter().filter(|r| r.pop < 8.0).count();
        println!(
            "\n  refusals on size alone: {}   on separatism alone: {}",
            refused.len() - by_sep,
            by_sep
        );
        println!();
    }

    /// Which seeds reach the annexation branch at all, and on whom.
    ///
    /// ```text
    /// cargo test --release -p spheres-sim conquest_seed_scan -- --ignored --nocapture
    /// ```
    #[test]
    #[ignore]
    fn conquest_seed_scan() {
        for (label, aggr) in [("war", 1.0), ("control (ai_aggression 0)", 0.0)] {
            let found = conquests(0..120, aggr);
            println!("--- {} : {} annexation(s) in seeds 0..120", label, found.len());
            for (seed, id, pop, year) in &found {
                println!("    seed {:>3}  {:?} at {:.2}m in {}", seed, id, pop, year);
            }
        }
    }

    /// The conquest funnel, gate by gate, so "conquest got rarer" can be told
    /// apart from "wars got rarer" and from "the size rule started refusing".
    ///
    /// `war.rs` reaches `Ending::Conquest` through one conjunction —
    /// `control >= CONTROL_SATURATED && top_rung(true) >= 8 && spent(false)` —
    /// and `conquer` then splits it: a loser under 8m people and under 0.6
    /// separatism is annexed and leaves the board, anything larger or angrier
    /// capitulates and survives. Only the first of those shows up in
    /// `conquests()`, so a collapse in annexations and a collapse in conquest
    /// endings are different findings with different causes.
    ///
    /// ```text
    /// cargo test --release -p spheres-sim conquest_funnel -- --ignored --nocapture
    /// ```
    #[test]
    #[ignore]
    fn conquest_funnel() {
        let (mut opened, mut invaded) = (0usize, 0usize);
        let (mut saturated, mut sat_and_rung, mut all_three) = (0usize, 0usize, 0usize);
        let (mut annexed, mut capitulated) = (0usize, 0usize);
        let mut seen: std::collections::BTreeSet<(u64, u32)> = Default::default();
        let mut inv_seen: std::collections::BTreeSet<(u64, u32)> = Default::default();
        let mut sat_seen: std::collections::BTreeSet<(u64, u32)> = Default::default();
        let mut rung_seen: std::collections::BTreeSet<(u64, u32)> = Default::default();
        let mut three_seen: std::collections::BTreeSet<(u64, u32)> = Default::default();
        for seed in 0..40u64 {
            let mut w = seeded(seed);
            for _ in 0..480 {
                for c in &w.conflicts {
                    if seen.insert((seed, c.id)) {
                        opened += 1;
                    }
                    if c.invasion_declared && inv_seen.insert((seed, c.id)) {
                        invaded += 1;
                    }
                    let sat = c.control >= war::CONTROL_SATURATED;
                    if sat && sat_seen.insert((seed, c.id)) {
                        saturated += 1;
                    }
                    if sat && c.top_rung(true) >= 8 && rung_seen.insert((seed, c.id)) {
                        sat_and_rung += 1;
                    }
                    // `spent(false)` from war.rs, verbatim: the defender side's
                    // highest remaining resolve, at or under 0.05. READ THE
                    // CAVEAT ON THE PRINTOUT — this is sampled before the tick,
                    // and the month the conjunction first holds is the month
                    // war.rs ends the conflict, so a terminal state is never
                    // observable here. The honest count of `Ending::Conquest`
                    // is the two headline counters below, which sum to it.
                    let defender_resolve = c
                        .side_b
                        .iter()
                        .filter_map(|id| c.posture_of(*id))
                        .map(|x| x.resolve)
                        .fold(0.0f64, f64::max);
                    if sat
                        && c.top_rung(true) >= 8
                        && defender_resolve <= 0.05
                        && three_seen.insert((seed, c.id))
                    {
                        all_three += 1;
                    }
                }
                for h in tick_month(&mut w, &[]) {
                    if h.contains("has annexed") {
                        annexed += 1;
                        println!("    ANNEXED  seed {:>2} {}-{:02}  {}", seed, w.year, w.month, h);
                    } else if h.contains("capitulates to") {
                        capitulated += 1;
                    }
                }
            }
        }
        println!("\n=== conquest funnel: seeds 0..40, 40 years ===");
        println!("  conflicts opened                          : {}", opened);
        println!("  ...that ever declared an invasion         : {}", invaded);
        println!("  ...that ever saturated control (>= 0.97)  : {}", saturated);
        println!("  ...saturated AND standing at rung 8       : {}", sat_and_rung);
        println!(
            "  ...AND the defender's resolve spent       : {}  (sampled pre-tick, so a \
             terminal month is invisible here — use the two lines below)",
            all_three
        );
        println!("  Ending::Conquest reached                  : {}", annexed + capitulated);
        println!("  ...that ANNEXED   (loser < 8m and calm)   : {}", annexed);
        println!("  ...that SUBJUGATED (loser too big or angry): {}", capitulated);
        println!();
    }

    /// SPEC section 6, in one line: "No swallowing India whole." `conquer`
    /// annexes only a nation under 8m people that is also quiet enough to hold
    /// (separatism < 0.6); anything larger or angrier is subjugated instead and
    /// survives to resent it.
    ///
    /// THE ANCHOR IS THE RULE ITSELF, not a fitted number: SPEC section 6 says
    /// "small nations (pop < ~8M) can be annexed ... large nations are
    /// *subjugated* instead". The 8m bound is transcribed from that sentence and
    /// is not re-derived here in either direction.
    ///
    /// Converted from one pinned seed to a twenty-seed sweep 2026-08-31, PLAN
    /// step 1, and this is the instrument that most needed it. THE SEED PIN HAD
    /// MOVED SIX TIMES — 9, 18, 0, 17, 9, 93 — because conquest is rare enough
    /// that any change touching the shared RNG stream reshuffles which run
    /// reaches the branch. It had moved a seventh time before this conversion
    /// was written: seed 93 reaches no conquest on the current tree, so the
    /// instrument was ALREADY RED, and red for the one reason its own comment
    /// said was not a finding — the pin had gone stale again. A pin that has to
    /// be re-pinned every time the world changes is not measuring the world.
    ///
    /// ── RE-POINTED 2026-08-31, ON RIDGE'S EXPLICIT AUTHORISATION ────────────
    ///
    /// Recorded here the way a BIBLE amendment is recorded, because doctrine
    /// (iron rule 5) otherwise forbids an agent editing a calibration test in
    /// answer to a red, and a later reader has to be able to tell an authorised
    /// strengthening from a quiet widening. Ridge, 2026-08-31, ruling 2 of
    /// three settled that day — the other two being the capital-channel repair
    /// and the sample-size doctrine now in CLAUDE.md. His words: count
    /// `Ending::Conquest` rather than board deaths, "which puts the size rule
    /// under test where it actually fires".
    ///
    /// NO BAR MOVED AND NO TOLERANCE WIDENED. The 8m and 0.6 bounds below are
    /// the same two literals, still transcribed rather than fitted, and they are
    /// now checked in both directions instead of one. What changed is the
    /// population sampled.
    ///
    /// WHAT WAS MIS-SAMPLED. The sweep read `conquests()`, which counts BOARD
    /// DEATHS — that is, only the cases where the rule said YES. Saying yes is
    /// the rarer arm by an order of magnitude: over seeds 0..240 the rule was
    /// put 107 times and admitted the annexation 10 of them, so at 0.042 per
    /// seed a twenty-seed window expects 0.8 observations and gets its whole
    /// verdict from one or two. Worse, it never looked at a refusal at all —
    /// and a refusal is what SPEC section 6 is a claim ABOUT. "No swallowing
    /// India whole" is a sentence about the nations that are not swallowed.
    /// The old test could not have caught the rule refusing to refuse.
    ///
    /// WHY THE NEW CONSTRUCTION MEASURES THE SAME CLAIM BETTER. It counts every
    /// `Ending::Conquest` — every case put to the rule, whichever way it went —
    /// and checks each verdict against the rule's own two clauses in both
    /// directions: nothing at or over 8m or at or over 0.6 separatism may be
    /// annexed, and nothing under both may be refused. Over seeds 0..100 that
    /// is 62 verdicts rather than 8, of which 54 are refusals: the arm the old
    /// sweep could not see is now the bulk of the evidence.
    ///
    /// THE SAMPLE IS DERIVED, per the same day's ruling 3, from this test's own
    /// measured variance rather than guessed. `conquest_size_rule_scan` reports
    /// refusals at 0.404 per seed with variance 0.508 and at least one in 30.0%
    /// of seeds, so a hundred seeds expect 40.4 +/- 7.1 and the floor of 15
    /// below stands 3.57 sd clear of the mean — a false red under 0.02%,
    /// against ruling 3's 1% ceiling. The floor is deliberately loose: it exists
    /// to stop a vacuous pass, not to police the conquest RATE, which is
    /// `conquest_funnel`'s job and is not a fitted number anybody has signed.
    /// ANNEXATION'S OWN BAR IS NOT HERE — it needs a wider sample than the size
    /// rule does and it lives in `a_dead_nation_holds_no_districts`, which is
    /// the test whose subject annexation actually is.
    ///
    /// IF THE FLOOR EVER GOES RED, conquest has become unreachable and THAT is
    /// the finding rather than a flaky test — recorded as O-1 in BUGS.md, and
    /// borders that never move is a game problem, not a test problem.
    /// `conquest_seed_scan` and `conquest_size_rule_scan` are the tools for the
    /// re-scan.
    #[test]
    fn a_large_nation_is_subjugated_rather_than_swallowed() {
        let endings = conquest_endings(0..100, 1.0);
        let (mut annexed, mut refused) = (0usize, 0usize);
        for r in &endings {
            let small_and_quiet = r.pop < 8.0 && r.sep < 0.6;
            if r.annexed {
                annexed += 1;
                // The arm the old sweep read, unchanged in substance: the 8m
                // bound is SPEC section 6 transcribed, and the separatism bound
                // is `conquer`'s second clause — a territory that is all
                // minorities is an occupation, not an acquisition.
                assert!(
                    small_and_quiet,
                    "{} was ANNEXED at {:.2}m people and {:.3} separatism in {} on seed {} — at \
                     or over 8m, or at or over 0.6 separatism, it is meant to be subjugated and \
                     survive to resent it",
                    r.loser.name(), r.pop, r.sep, r.year, r.seed
                );
            } else {
                refused += 1;
                // The arm the old sweep could not see, and the one SPEC section
                // 6 is really about. A conquest that ended in capitulation must
                // have had a reason to: too big, or too angry to hold.
                assert!(
                    !small_and_quiet,
                    "{} was SUBJUGATED at {:.2}m people and {:.3} separatism in {} on seed {} — \
                     under 8m and under 0.6 separatism it is small enough and quiet enough to \
                     annex, so the size rule refused a case it is meant to admit",
                    r.loser.name(), r.pop, r.sep, r.year, r.seed
                );
            }
        }
        assert!(
            refused >= 15,
            "only {} conquest(s) in a hundred seeds of forty years ended in subjugation ({} \
             annexation(s) alongside), so the size rule was barely exercised — conquest may have \
             become unreachable (BUGS.md O-1). Re-scan with `conquest_size_rule_scan` before \
             touching this test",
            refused,
            annexed
        );

        // The control arm: a world where nobody attacks anybody conquers
        // nobody. This is what stops the sweep above from passing for the wrong
        // reason — it establishes that the verdicts counted are reached BECAUSE
        // wars are fought, on the same code path rather than a second one. It
        // reads conquest endings now rather than board deaths, so it also
        // covers the subjugation arm the assertions above are mostly made of.
        let control = conquest_endings(0..100, 0.0);
        assert!(
            control.is_empty(),
            "{} conquest(s) were reached in a world with the AI's appetite for war set to zero: \
             {:?}",
            control.len(),
            control
                .iter()
                .map(|r| (r.seed, r.loser.name(), r.year, r.annexed))
                .collect::<Vec<_>>()
        );
    }

    /// Where the `burned_` flags actually come from, and whether a border was
    /// ever crossed to earn one. Both `war.rs` and `dyads.rs` describe this
    /// flag in exactly one way — "recorded when an invasion is repelled",
    /// "after one repelled invasion the lesson is learned permanently" — so a
    /// flag written on a quarrel where `invasion_declared` is false is the
    /// documented contract being broken, not a test being fussy.
    ///
    /// ```text
    /// cargo test --release -p spheres-sim burned_flag_provenance -- --ignored --nocapture
    /// ```
    #[test]
    #[ignore]
    fn burned_flag_provenance() {
        let mut with_invasion = 0usize;
        let mut without_invasion = 0usize;
        let mut total_written = 0usize;
        let mut repel_headlines = 0usize;
        let mut repel_headlines_no_invasion = 0usize;
        let mut samples: Vec<String> = Vec::new();
        for seed in 0..20u64 {
            let mut w = seeded(seed);
            for _ in 0..480 {
                // Pre-tick: every live conflict, the key its ending would
                // write, and whether that key is already on the books.
                let before: Vec<(u32, String, bool, NationId, NationId, bool, u8, u8, f64)> = w
                    .conflicts
                    .iter()
                    .map(|c| {
                        let key = format!("burned_{:?}_{:?}", c.origin_attacker, c.defender());
                        let had = w.has_flag(&key);
                        (
                            c.id,
                            key,
                            had,
                            c.origin_attacker,
                            c.defender(),
                            c.invasion_declared,
                            c.top_rung(true),
                            c.top_rung(false),
                            c.control,
                        )
                    })
                    .collect();
                let burned_before =
                    w.flags.iter().filter(|f| f.starts_with("burned_")).count();
                let headlines = tick_month(&mut w, &[]);
                total_written +=
                    w.flags.iter().filter(|f| f.starts_with("burned_")).count() - burned_before;
                // The Repelled arm's own headline, matched against whether a
                // border was ever crossed in the quarrel it closed.
                for h in headlines.iter().filter(|h| h.contains("the aggressor's regime totters")) {
                    repel_headlines += 1;
                    let invaded = before
                        .iter()
                        .any(|(_, _, _, a, _, inv, _, _, _)| *inv && h.contains(a.name()));
                    if !invaded {
                        repel_headlines_no_invasion += 1;
                    }
                }
                for (id, key, had, a, d, inv, ra, rb, ctl) in before {
                    if w.conflicts.iter().any(|c| c.id == id) || had || !w.has_flag(&key) {
                        continue;
                    }
                    if inv {
                        with_invasion += 1;
                    } else {
                        without_invasion += 1;
                        if samples.len() < 15 {
                            samples.push(format!(
                                "seed {:>2}  {}-{:02}  {:?} -> {:?}  invasion_declared=FALSE  \
                                 rungs {}/{}  control {:+.2}",
                                seed, w.year, w.month, a, d, ra, rb, ctl
                            ));
                        }
                    }
                }
            }
        }
        println!("\n=== burned_ flags written, seeds 0..20, 40 years ===");
        println!("  total written               : {}", total_written);
        println!("  attributed, AFTER an invasion: {}", with_invasion);
        println!("  attributed, NO invasion      : {}", without_invasion);
        println!(
            "  \"repels ...'s invasion\" headlines: {} ({} where no invasion was ever declared)",
            repel_headlines, repel_headlines_no_invasion
        );
        for s in &samples {
            println!("    {}", s);
        }
        println!();
    }

    #[test]
    fn a_burned_aggressor_does_not_come_back_for_the_same_prize() {
        // "Saddam doesn't retry Kuwait." Being thrown back sets
        // `burned_<attacker>_<defender>`, and `dyads.rs` reads it when it prices
        // appetite — a first-time gambler discounts the coalition, somebody who
        // has already met it does not.
        //
        // What this asserts is the consequence rather than the flag: once the
        // lesson is on the books, that attacker never crosses that border
        // again. A conflict may still be opened — states go on quarrelling —
        // but it must never reach an invasion a second time.
        //
        // THE SECOND ASSERTION IS THE FLAG'S PROVENANCE, added after this test
        // was misdiagnosed as a test defect. It failed reading "Iraq was burned
        // over Kuwait in 1997 and still invaded 0 times", and the reading was
        // right: `war.rs` was writing `burned_` on endings that never involved
        // an invasion, so the consequence above was being asserted against a
        // lesson nobody had been taught. Both `war.rs` and `dyads.rs` describe
        // this flag in exactly one way — "recorded when an invasion is
        // repelled" — and `dyads::war_appetite` spends it to move the coalition
        // discount 0.10 -> 1.00 permanently. So the contract is checked at the
        // moment of writing, in the same loop, at no extra runtime: every
        // `burned_` flag written anywhere in the world must be written on a
        // quarrel where somebody crossed a border in force.
        // ── SAMPLE WIDENED TEN SEEDS -> TWENTY, 2026-08-31, UNDER IRON RULE 7 ──
        //
        // NO BAR MOVED. `burned_seeds >= 4` and `writes_seen >= 5` are the same
        // two literals; both are FIXED floors rather than fractions of the
        // sample, so widening cannot loosen either. What widening does is give
        // the per-seed assertions above — the ones that are the actual subject
        // of this test — twice the evidence.
        //
        // WHY IT NEEDED WIDENING. `sample_size_audit::panel_variance` measures
        // `burned_Iraq_Kuwait` at 0.57 per seed over a hundred seeds, so the
        // per-seed variance is p(1-p) = 0.2451. Against a floor of four, ten
        // seeds carry a FALSE-RED PROBABILITY OF 8.06% — eight times rule 7's
        // ceiling — for a floor whose entire job is to stop this test passing
        // vacuously. The derivation gives n >= 14 for 1%. Twenty is taken
        // because it holds the ceiling if the rate drifts: 0.0001 at the
        // measured 0.57, 0.0013 at 0.50 and 0.0049 at 0.45, and this rate is
        // evidently something a war-incidence change can move.
        let mut burned_seeds = 0;
        let mut writes_seen = 0;
        for seed in 0..20u64 {
            let mut w = seeded(seed);
            let mut invasions = 0;
            let mut invaded_ids: Vec<u32> = Vec::new();
            let mut burned_at: Option<i32> = None;
            for _ in 0..420 {
                // Pre-tick: the key each live quarrel's ending would write, and
                // whether a border has been crossed in it.
                let pending: Vec<(u32, String, bool, bool)> = w
                    .conflicts
                    .iter()
                    .map(|c| {
                        let key =
                            format!("burned_{:?}_{:?}", c.origin_attacker, c.defender());
                        let had = w.has_flag(&key);
                        (c.id, key, had, c.invasion_declared)
                    })
                    .collect();
                tick_month(&mut w, &[]);
                for (id, key, had, invaded) in pending {
                    if w.conflicts.iter().any(|c| c.id == id) || had || !w.has_flag(&key) {
                        continue;
                    }
                    assert!(
                        invaded,
                        "seed {}: {} was written in {} closing a quarrel in which no border \
                         was ever crossed — `burned_` is the lesson of an invasion repelled, \
                         and there was no invasion to repel",
                        seed, key, w.year
                    );
                    writes_seen += 1;
                }
                for c in &w.conflicts {
                    if c.origin_attacker == NationId::Iraq
                        && c.defender() == NationId::Kuwait
                        && c.invasion_declared
                        && !invaded_ids.contains(&c.id)
                    {
                        invaded_ids.push(c.id);
                        invasions += 1;
                    }
                }
                if burned_at.is_none() && w.has_flag("burned_Iraq_Kuwait") {
                    burned_at = Some(w.year);
                }
            }
            if let Some(yr) = burned_at {
                burned_seeds += 1;
                assert_eq!(
                    invasions, 1,
                    "seed {}: Iraq was burned over Kuwait in {} and still invaded {} times",
                    seed, yr, invasions
                );
            }
        }
        assert!(
            burned_seeds >= 4,
            "the lesson was never learned in twenty seeds ({} burned), so nothing was tested",
            burned_seeds
        );
        assert!(
            writes_seen >= 5,
            "no `burned_` flag was written anywhere in twenty seeds of thirty-five years ({} \
             writes), so the provenance assertion above never ran on anything",
            writes_seen
        );
    }

    #[test]
    fn the_fiscal_ai_arrests_a_debt_spiral() {
        // `politics.rs` consolidates above 0.85 debt: taxes up two basis points
        // a month, military and state investment shaved half a percent. Put an
        // AI government deep in the hole with a structural deficit under it and
        // it has to climb out, rather than ride the debt up until the growth
        // drag does the governing for it.
        let mut w = seeded(3);
        let victim = NationId::Italy;
        {
            let n = w.nation_mut(victim);
            n.debt_gdp = 1.30;
            n.tax_rate = 0.22; // well under what it spends
            n.mil_spend_gdp = 0.06;
            n.state_invest_gdp = 0.10;
        }
        let (tax0, mil0, inv0) = {
            let n = w.nation(victim);
            (n.tax_rate, n.mil_spend_gdp, n.state_invest_gdp)
        };
        let mut peak: f64 = 0.0;
        for _ in 0..240 {
            tick_month(&mut w, &[]);
            let d = w.nation(victim).debt_gdp;
            if d > peak {
                peak = d;
            }
        }
        let n = w.nation(victim);
        assert!(
            n.tax_rate > tax0,
            "the fiscal AI never raised taxes: {:.4} -> {:.4} at {:.2} debt",
            tax0, n.tax_rate, n.debt_gdp
        );
        assert!(
            n.mil_spend_gdp < mil0 && n.state_invest_gdp < inv0,
            "the fiscal AI never trimmed spending: mil {:.4} -> {:.4}, invest {:.4} -> {:.4}",
            mil0, n.mil_spend_gdp, inv0, n.state_invest_gdp
        );
        assert!(
            n.debt_gdp < peak,
            "debt never came off its peak: peak {:.2}, ended {:.2}",
            peak, n.debt_gdp
        );
        assert!(
            peak < 6.0,
            "the consolidation rule did not arrest the spiral: peak {:.2}",
            peak
        );
    }

    #[test]
    fn a_save_taken_mid_war_resumes_the_same_war() {
        // `save_load_roundtrip_continuity` saves a world at peace. A war carries
        // state the quiet world never exercises — conflicts, per-belligerent
        // postures and resolve, munitions, theatre access — and all of it has to
        // survive the round trip, or a player who saves during the one moment
        // they care about resumes a different war.
        let mut a = seeded(0);
        a.rules.ai_aggression = 0.0;
        war::declare_war(&mut a, NationId::Iraq, NationId::Kuwait).unwrap();
        run_months(&mut a, 6);
        let mid = a
            .conflict_between(NationId::Iraq, NationId::Kuwait)
            .expect("the war has to still be running for this test to mean anything");
        assert!(mid.months >= 6, "the staged war did not survive to the save point");
        let postures = mid.posture.len();

        let snapshot = save(&a);
        let mut b = load(&snapshot).expect("a mid-war save must load");
        assert_eq!(
            b.conflict_between(NationId::Iraq, NationId::Kuwait)
                .map(|c| c.posture.len()),
            Some(postures),
            "the reloaded war lost its belligerents"
        );
        run_months(&mut a, 120);
        run_months(&mut b, 120);
        assert_eq!(state_hash(&a), state_hash(&b), "a war diverged across save/load");
        assert_eq!(save(&a), save(&b), "a war diverged across save/load");
    }

    #[test]
    fn player_commands_clamp_to_valid_ranges() {
        // Every policy lever is clamped where it is applied, so a UI that sends
        // a slider value from a stale form — or a player typing into the CLI —
        // cannot put the economy outside the band the model was fitted in. A
        // negative tax rate or a 500% policy rate is not an interesting game
        // state, it is a division waiting to happen.
        let mut w = seeded(0);
        let me = NationId::USA;
        w.player = Some(me);

        // (command builder, absurd low, absurd high, expected floor, expected ceiling)
        let cases: [(&str, f64, f64, f64, f64); 4] = [
            ("rate", -3.0, 5.0, 0.0, 0.60),
            ("tax", -1.0, 0.90, 0.02, 0.60),
            ("mil", -1.0, 2.0, 0.0, 0.35),
            ("invest", -1.0, 1.0, 0.0, 0.40),
        ];
        for (lever, low, high, floor, ceiling) in cases {
            for (input, expected) in [(low, floor), (high, ceiling)] {
                // Deliberately absurd inputs are also expensive ones — a
                // 0.6-point tax swing is priced at 186 standing — and this test
                // is about the clamp, not about affordability. Same act as
                // `bankroll`, at a number no swing can outrun.
                w.nation_mut(me).political_capital = 5_000.0;
                let cmd = match lever {
                    "rate" => Command::SetInterestRate { nation: me, rate: input },
                    "tax" => Command::SetTaxRate { nation: me, rate: input },
                    "mil" => Command::SetMilSpend { nation: me, share: input },
                    _ => Command::SetStateInvest { nation: me, share: input },
                };
                apply_command(&mut w, &cmd).unwrap_or_else(|e| panic!("{} {} refused: {}", lever, input, e));
                let got = match lever {
                    "rate" => w.nation(me).interest_rate,
                    "tax" => w.nation(me).tax_rate,
                    "mil" => w.nation(me).mil_spend_gdp,
                    _ => w.nation(me).state_invest_gdp,
                };
                assert_eq!(
                    got, expected,
                    "{} given {} settled at {} rather than its {} bound",
                    lever, input, got, if input < 0.0 { "lower" } else { "upper" }
                );
            }
        }

        // A clamped world is still a world that ticks.
        run_months(&mut w, 24);
        let n = w.nation(me);
        assert!(n.gdp.is_finite() && n.gdp > 0.0, "clamped levers broke the economy: {}", n.gdp);
        assert!(n.inflation.is_finite(), "clamped levers produced a NaN inflation");
    }

    /// What the four single-seed instruments actually read, across ten seeds.
    ///
    /// ROADMAP section 8 records that each of them takes ONE whole-world run at
    /// ONE seed and asserts an absolute band, so adding any nation re-rolls the
    /// reading even when nothing about the measurement changed. This is the
    /// measurement that has to precede converting them: a threshold carried over
    /// from the single-seed era is a guess, and the trade test's own median sat
    /// at 1.2102 against a threshold of 1.20.
    ///
    /// ```text
    /// cargo test --release -p spheres-sim instrument_spread -- --ignored --nocapture
    /// ```
    #[test]
    #[ignore]
    fn instrument_spread() {
        fn med(v: &mut [f64]) -> f64 {
            v.sort_by(|a, b| a.partial_cmp(b).unwrap());
            v[v.len() / 2]
        }
        fn show(name: &str, v: &mut [f64]) {
            let m = med(v);
            println!(
                "{:<28} median {:>8.4}  min {:>8.4}  max {:>8.4}\n{:>30} {:?}",
                name, m, v[0], v[v.len() - 1], "",
                v.iter().map(|x| (x * 1000.0).round() / 1000.0).collect::<Vec<_>>()
            );
        }

        const SEEDS: u64 = 10;

        // ---- sanctions: growth points lost, and the coalition's weight ----
        let mut lost = vec![];
        let mut shares = vec![];
        {
            let target = NationId::Brazil;
            let coalition =
                [NationId::USA, NationId::UK, NationId::France, NationId::Germany, NationId::Japan];
            for seed in 0..SEEDS {
                let mut control = seeded(seed);
                control.rules.ai_aggression = 0.0;
                let c0 = control.nation(target).gdp;
                run_months(&mut control, 240);
                let base = exact::powf(control.nation(target).gdp / c0, 1.0 / 20.0) - 1.0;

                let mut treated = seeded(seed);
                treated.rules.ai_aggression = 0.0;
                let t0 = treated.nation(target).gdp;
                let mut acc = 0.0;
                for _ in 0..240 {
                    for i in &coalition {
                        if !treated.is_sanctioning(*i, target) {
                            treated.sanctions.push((*i, target));
                        }
                    }
                    acc += treated.sanction_weight(target);
                    tick_month(&mut treated, &[]);
                }
                let after = exact::powf(treated.nation(target).gdp / t0, 1.0 / 20.0) - 1.0;
                lost.push((base - after) * 100.0);
                shares.push(acc / 240.0);
            }
        }
        show("sanctions: pt/yr lost", &mut lost);
        show("sanctions: mean G5 share", &mut shares);

        // ---- trade: the lift, and the leash on each side ----
        let mut lift = vec![];
        let mut warsaw_v = vec![];
        let mut washington_v = vec![];
        for seed in 0..SEEDS {
            let (mut base, mut open) = (seeded(seed), seeded(seed));
            for w in [&mut base, &mut open] {
                w.rules.ai_aggression = 0.0;
                w.player = Some(NationId::USA);
            }
            force_trade(&mut open, NationId::USA, NationId::Poland);
            run_months(&mut base, 240);
            run_months(&mut open, 240);
            let (b, o) = (base.nation(NationId::Poland).gdp, open.nation(NationId::Poland).gdp);
            lift.push(o / b);
            let (p0, u0) =
                (open.nation(NationId::Poland).gdp, open.nation(NationId::USA).gdp);
            let _ = apply_command(
                &mut open,
                &Command::AbrogateTrade { from: NationId::USA, to: NationId::Poland },
            );
            warsaw_v.push(1.0 - open.nation(NationId::Poland).gdp / p0);
            washington_v.push(1.0 - open.nation(NationId::USA).gdp / u0);
        }
        show("trade: Poland lift ratio", &mut lift);
        show("trade: Warsaw loses", &mut warsaw_v);
        show("trade: Washington loses", &mut washington_v);

        // ---- frontier: the fastest mature economy in each seed ----
        let mature = [
            NationId::USA, NationId::Japan, NationId::Germany,
            NationId::France, NationId::UK, NationId::Italy,
        ];
        let mut fastest = vec![];
        for seed in 0..SEEDS {
            let start: Vec<(NationId, f64)> = {
                let w = seeded(seed);
                mature.iter().map(|id| (*id, w.nation(*id).gdp)).collect()
            };
            let mut w = seeded(seed);
            run_months(&mut w, 12 * 35);
            let mut worst: f64 = 0.0;
            for (id, g0) in start {
                let n = w.nation(id);
                if !n.alive {
                    continue;
                }
                let cagr = exact::powf(n.gdp / g0, 1.0 / 35.0) - 1.0;
                if cagr > worst {
                    worst = cagr;
                }
            }
            fastest.push(worst * 100.0);
        }
        show("frontier: fastest mature %/yr", &mut fastest);

        // ---- conquest: how often the annexation branch is reached at all ----
        let mut annex = vec![];
        for seed in 0..SEEDS {
            let mut w = seeded(seed);
            let mut alive: Vec<(NationId, f64)> =
                w.nations.iter().filter(|n| n.alive).map(|n| (n.id, n.population)).collect();
            let mut n_annex = 0.0;
            for _ in 0..480 {
                tick_month(&mut w, &[]);
                let mut still = vec![];
                for (id, pop) in alive {
                    if w.nation_opt(id).is_some_and(|n| n.alive) {
                        still.push((id, w.nation(id).population));
                    } else if id != NationId::USSR && id != NationId::Yugoslavia {
                        n_annex += 1.0;
                        println!("    annexation: seed {} {} {:?} at {:.1}m", seed, w.year, id, pop);
                    }
                }
                alive = still;
            }
            annex.push(n_annex);
        }
        show("conquest: annexations/seed", &mut annex);
    }

    #[test]
    fn a_research_programme_moves_a_technology_forward_faster() {
        use tech::Domain;
        // The point of the feature, stated as a measurement: a government that
        // declares a national programme in a domain must finish more of that
        // domain's tree over a decade than the same government left alone --
        // and must pay for it out of the other seven, not out of nothing.
        let count_in = |w: &WorldState, id: NationId, d: Domain| -> usize {
            let n = w.nation(id);
            tech::registry()
                .iter()
                .enumerate()
                .filter(|(i, def)| def.domain == d && n.tech.knows_index(*i as u16))
                .count()
        };

        let me = NationId::USA;
        let (mut idle, mut driven) = (seeded(0), seeded(0));
        for w in [&mut idle, &mut driven] {
            w.rules.ai_aggression = 0.0;
            w.player = Some(me);
        }
        driven.nation_mut(me).political_capital = 100.0;
        apply_command(
            &mut driven,
            &Command::SetResearchPriority { nation: me, domain: Some(tech::Domain::Computing) },
        )
        .expect("a superpower can afford to announce a programme");
        assert_eq!(driven.nation(me).tech.priority, Some(tech::Domain::Computing));

        run_months(&mut idle, 120);
        run_months(&mut driven, 120);

        let (i_comp, d_comp) = (count_in(&idle, me, Domain::Computing), count_in(&driven, me, Domain::Computing));
        assert!(
            d_comp > i_comp,
            "a national computing programme bought nothing: {} technologies against {}",
            d_comp, i_comp
        );

        // ...and the seven that funded it went short. Total research is not
        // created by announcing a priority, only moved.
        let others = |w: &WorldState| -> usize {
            tech::DOMAINS
                .iter()
                .filter(|d| **d != tech::Domain::Computing)
                .map(|d| count_in(w, me, *d))
                .sum()
        };
        assert!(
            others(&driven) < others(&idle),
            "the priority was free: other domains held {} against {}",
            others(&driven), others(&idle)
        );
    }

    #[test]
    fn redirecting_a_laboratory_forfeits_half_of_what_it_had_banked() {
        // The cost that makes the choice a choice. Political capital prices the
        // announcement; this prices the thrash, and it is the one that stops a
        // player re-picking every month for free.
        let me = NationId::USA;
        let mut w = seeded(0);
        w.player = Some(me);
        run_months(&mut w, 24); // let a programme accumulate

        // Whichever domain actually offers a choice — the tree is deep and
        // uneven, and Computing in particular runs out of reachable projects for
        // a frontier economy inside two years, which is a real property of it
        // rather than something to hardcode around.
        let (domain, di, banked, elsewhere) = tech::DOMAINS
            .iter()
            .find_map(|d| {
                let di = d.index();
                let n = w.nation(me);
                let banked = n.tech.progress[di];
                if banked <= 0.0 {
                    return None;
                }
                let other = tech::eligible_projects(n, *d)
                    .iter()
                    .map(|x| x.id.to_string())
                    .find(|id| tech::index_of(id) != n.tech.focus[di])?;
                Some((*d, di, banked, other))
            })
            .expect("no domain had both banked progress and an alternative project");
        let _ = domain;

        w.nation_mut(me).political_capital = 100.0;
        apply_command(
            &mut w,
            &Command::SetResearchFocus {
                nation: me,
                domain,
                tech: Some(elsewhere.clone()),
            },
        )
        .expect("a legal redirection");

        assert_eq!(
            w.nation(me).tech.focus[di],
            tech::index_of(&elsewhere),
            "the laboratory did not take the new subject"
        );
        assert!(
            (w.nation(me).tech.progress[di] - banked * 0.5).abs() < 1e-9,
            "expected half of {} to survive the switch, found {}",
            banked,
            w.nation(me).tech.progress[di]
        );

        // A technology nobody can start yet is refused rather than silently accepted.
        let bogus = apply_command(
            &mut w,
            &Command::SetResearchFocus {
                nation: me,
                domain,
                tech: Some("not_a_real_technology".into()),
            },
        );
        assert!(bogus.is_err(), "an invented technology was accepted as a project");
    }

    #[test]
    fn a_century_holds_together() {
        // The risk register's top entry is two hundred AI economies spiralling,
        // and its stated mitigation is exactly this: a headless century run as a
        // standing invariant. Cheap enough to keep now that a relation lookup is
        // an index rather than a search — the whole century runs in under a
        // second — and it is the thing that catches the next trade-shaped bug
        // without anyone noticing the world felt wrong.
        for seed in 0..3u64 {
            let mut w = world_1990(GameRules { seed, ..GameRules::default() });
            for _ in 0..1200 {
                tick_month(&mut w, &[]);
                for n in w.nations.iter().filter(|n| n.alive) {
                    assert!(n.gdp.is_finite() && n.gdp > 0.0, "{:?} gdp {} in {}", n.id, n.gdp, w.year);
                    assert!(n.inflation.is_finite(), "{:?} inflation NaN in {}", n.id, w.year);
                    assert!(n.debt_gdp.is_finite() && n.debt_gdp < 6.0, "{:?} debt {:.1} in {}", n.id, n.debt_gdp, w.year);
                    assert!(n.population.is_finite() && n.population > 0.0, "{:?} population {} in {}", n.id, n.population, w.year);
                    assert!((0.0..=100.0).contains(&n.stability), "{:?} stability {} in {}", n.id, n.stability, w.year);
                    assert!((0.0..=100.0).contains(&n.political_capital), "{:?} capital {} in {}", n.id, n.political_capital, w.year);
                }
                assert!(w.oil_price.is_finite() && w.oil_price > 0.0, "oil {} in {}", w.oil_price, w.year);
            }
        }
    }

    /// Where the century run actually spends its time, per subsystem, at
    /// several roster sizes. An instrument, not an assertion — it measures wall
    /// clock, which is exactly the thing iron rule 1 keeps out of the sim, so it
    /// is `#[ignore]`d and may never be depended on by anything that ticks.
    ///
    ///     cargo test --release -p spheres-sim -- --ignored --nocapture profile
    ///
    /// Read the rightmost column first. A subsystem whose share of the tick
    /// *rises* with the roster is the one that is super-linear; a subsystem
    /// whose share falls is merely linear and is being outgrown. That is the
    /// whole diagnostic, and it is the reason this prints a sweep rather than a
    /// single number: the 160-nation profile on its own cannot tell the two
    /// apart, and this project has twice mistaken a big linear cost for a
    /// quadratic one.
    ///
    /// Smaller rosters are made by retiring nations from the bottom of registry
    /// order rather than by loading a smaller data set, so the relations matrix
    /// stays the full width and only the number of *living* nations moves. That
    /// is the variable under test, and the distinction turned out to be the
    /// answer: the largest single cost in the tick did not move with it at all,
    /// because it was O(matrix width) and not O(nations alive).
    ///
    /// Every figure is the BEST of `PASSES` identical runs, not the mean. This
    /// machine reports the same binary anywhere between 6.8 and 11.4 seconds on
    /// a century run, so a mean measures the other processes on the box; the
    /// minimum is the closest thing to the work actually done. Two profiles are
    /// only comparable if both were taken this way.
    #[test]
    #[ignore = "timing instrument; run with --ignored --nocapture"]
    fn century_run_profile() {
        use std::time::{Duration, Instant};

        const MONTHS: usize = 1200;
        const PASSES: usize = 3;
        let sizes = [30usize, 108, 137];

        println!();
        for size in sizes {
            let mut best = vec![Duration::MAX; SYSTEMS.len()];
            let mut best_total = Duration::MAX;
            let mut alive = 0usize;

            for _ in 0..PASSES {
                let mut w = world_1990(GameRules::default());
                let living: Vec<usize> =
                    (0..w.nations.len()).filter(|i| w.nations[*i].alive).collect();
                for i in living.iter().skip(size) {
                    w.nations[*i].alive = false;
                }
                alive = w.nations.iter().filter(|n| n.alive).count();

                // One year of ticks discarded: the first month allocates the
                // per-nation vectors every system grows on its first pass, and
                // measuring that as if it were steady state overstates whichever
                // system happens to run first.
                for _ in 0..12 {
                    tick_month(&mut w, &[]);
                }

                let mut per = vec![Duration::ZERO; SYSTEMS.len()];
                let whole = Instant::now();
                for _ in 0..MONTHS {
                    w.headlines.clear();
                    w.reindex();
                    for (i, (_, system)) in SYSTEMS.iter().enumerate() {
                        let t = Instant::now();
                        system(&mut w);
                        per[i] += t.elapsed();
                    }
                    w.month += 1;
                    if w.month > 12 {
                        w.month = 1;
                        w.year += 1;
                    }
                }
                best_total = best_total.min(whole.elapsed());
                for (b, d) in best.iter_mut().zip(per) {
                    *b = (*b).min(d);
                }
            }

            println!(
                "=== {} living nations, {} months, best of {}: {:.3}s wall, {:.2}ms/month",
                alive,
                MONTHS,
                PASSES,
                best_total.as_secs_f64(),
                best_total.as_secs_f64() * 1000.0 / MONTHS as f64
            );
            let mut rows: Vec<(usize, Duration)> = best.iter().copied().enumerate().collect();
            rows.sort_by_key(|r| std::cmp::Reverse(r.1));
            let accounted: f64 = best.iter().map(|d| d.as_secs_f64()).sum();
            for (i, d) in rows {
                println!(
                    "    {:<14} {:>8.3}s  {:>5.1}%  {:>7.3} ms/month",
                    SYSTEMS[i].0,
                    d.as_secs_f64(),
                    100.0 * d.as_secs_f64() / accounted,
                    d.as_secs_f64() * 1000.0 / MONTHS as f64
                );
            }
            println!("    {:<14} {:>8.3}s", "[measured]", accounted);
        }
    }

    #[test]
    fn the_1990_start_is_pinned() {
        // The starting world before a single month is ticked. The golden run
        // hash below catches a changed digit only after twenty years of
        // compounding have carried it somewhere visible; this catches it in the
        // file it was typed in.
        //
        // It earns its place now that the roster is JSON. A positional
        // constructor at least made a wrong number a compile error when the
        // arity changed; a data file makes it a plausible-looking edit. This
        // number was measured against master at c454c81, with the nations still
        // Rust literals, and it did not move when they became data — which is
        // the proof that the transcription changed nothing.
        //
        // Re-pinned once, 0x2cc32e8ec58365e2 -> 0x92537b6bd76fa632, when
        // government landed. This hash is taken over the whole serialized
        // WorldState, so it moves whenever the state gains a field, and
        // `governments` is now seated at t=0 rather than filling in in
        // February. That is not what this test exists to catch, so the movement
        // was proven harmless before the number was touched:
        //   - `git diff master...feat/government -- spheres-sim/data/` is empty:
        //     not one transcribed 1990 figure changed.
        //   - `government::ensure`/`form_government` write only to
        //     `w.governments`; they never touch a `Nation` field.
        // If this ever fails again, do the same two checks before re-pinning. A
        // move with a dirty data/ directory is a wrong number, not a new field.
        //
        // Re-pinned a second time, 0x92537b6bd76fa632 -> 0x4295ff602fa2497b,
        // when `exact` landed: government seats every parliament at t=0 and its
        // seat formula now goes through exact::powf, so the opening seat shares
        // differ in their last bits. Same two checks done, same result —
        // spheres-sim/data/ is untouched.
        //
        // Re-pinned a third time, 0x4295ff602fa2497b -> 0x68d452c8d3a1ca5a, on
        // adding Spain — the first nation added since the roster became
        // extensible. This hash is taken over the whole serialized WorldState,
        // so a thirty-first nation in it moves the number by construction; the
        // test cannot mean "no nation was ever added". What it still means, and
        // what was checked before the number was touched, is that no EXISTING
        // nation's figures moved:
        //   - `git diff -- spheres-sim/data/` was +16/-0, entirely the new
        //     Spain block appended to relations_1990.json, plus one untracked
        //     file, spain.json. Not one transcribed figure of the other thirty
        //     changed.
        //   - the roster diff is one appended row plus `"Spain"` added to
        //     France's neighbour list, which the symmetry assertion in
        //     `nations.rs` requires and which changes no index.
        // THIS IS THE CHECK EVERY ROSTER AUTHOR OWES. A hash that moved with a
        // dirty data/ directory is a wrong number, not a new nation.
        //
        // Re-pinned a fourth time, 0x68d452c8d3a1ca5a -> 0x1bb3d0e7c7919e2e,
        // at the ten-region integration: 31 nations -> 108, landed one branch
        // at a time and re-pinned ONCE at the end rather than ten times. The
        // check above was run against every merge and the data/ diff against
        // a477687 is +3556/-0: PURE ADDITION, not one figure of the thirty-one
        // nations that were already on the board. One figure did move inside
        // this integration and is recorded here rather than left to be found:
        //   hungary.json gdp_bn 33.1 -> 34.5, corrected after the eastern
        //   European branch landed and therefore invisible in the diff against
        //   master, which is exactly why it is written down. The file cited
        //   NY.GDP.MKTP.CD series HUN 1990 and that series returns
        //   $34,478,360,678.76. 33.1 was 4.0% below the source it named while
        //   the five nations it named as its comparability set all sit inside
        //   1% of the same series. See the commit and the source note in the
        //   file for why the figure moved rather than the citation.
        // Nothing else in spheres-sim/data/ changed for any nation that was on
        // the board before this integration.
        //
        // Re-pinned a fifth time, 0x1bb3d0e7c7919e2e -> 0xaa93baba96ed09b2, for
        // the commitment ladder. `spheres-sim/data/` is byte-for-byte unchanged
        // — `git diff` against the previous pin is empty for the whole directory
        // — and not one transcribed 1990 figure moved. What moved is the SHAPE
        // of the state the hash reads, which is what a new subsystem does:
        //   - `Nation` gains `munitions`, BIBLE §6's second stock, at 1.0.
        //   - `WorldState` gains `theatres`, `conflicts` and `access`, and
        //     `wars` is gone; `Conflict` replaces `War`.
        // The 1990 board is the same board. The hash is a fingerprint of the
        // struct as well as the numbers in it, and this is the struct changing.
        // Re-pinned a sixth time, 0x180bcace7572d8ba -> 0xb9673f3eec091d10, for
        // `WorldState::player_set_rate`. Same story as the fifth: the struct the
        // hash reads gained a field, and not one transcribed 1990 figure moved.
        // `spheres-sim/data/` is byte-for-byte unchanged.
        //
        // The bounds that landed with it — the oil-revenue share cap and the
        // floor under the annual growth rate — are guards that do not bind on a
        // working economy, and that claim was measured rather than asserted:
        // dumping gdp, inflation, mil_strength and debt for every nation after
        // 420 months, over three seeds, gives a file byte-identical to the one
        // the previous pin produces. The simulation did not move. The struct did.
        //
        // Re-pinned for research projects. `TechState` gains `priority`, the one
        // domain a government has declared a national programme. Same story as
        // every entry above and checkable the same way: no data file changed,
        // and the field defaults to `None` while `domain_weights` multiplies
        // only when it is `Some`, so a world where nobody has declared anything
        // computes bit-identical shares to the one before it. The struct moved;
        // the simulation did not.
        //
        // Re-pinned for the procurement layer. `Nation` gains `arsenal`, which
        // is empty at 1990 and accrues only banked money until a nation has a
        // technology to spend it on. No other quantity moves: `arsenal::tick`
        // places no orders while `available()` is empty, and `war.rs` does not
        // read the arsenal yet. Struct changed, simulation did not, and
        // `spheres-sim/data/` is untouched.
        //
        // Re-pinned twice for procurement: once for the inheritance, and again
        // when EQUIP_HORIZON was derived down from 240 months to 200, which is
        // the value that makes a fully-equipped 1990 United States worth about
        // $1.1tn against a BEA gross stock of national-defence equipment near
        // $1.1tn. Re-pinned for the 1990 arsenal inheritance. Every nation now opens
        // holding what thirty years of its own procurement bought it, seeded
        // from the transcribed budget and strength already in the record. No
        // data file changed and no figure was invented: units are solved from
        // money so that book value equals the target by construction, which is
        // why this needed no per-nation tuning across 137 nations. The seeder
        // takes a record rather than a `&mut WorldState`, so it cannot draw from
        // the RNG even by accident, and every downstream draw is unmoved.
        //
        // Re-pinned for district ownership, same evidence as above: a new
        // `WorldState` field (`districts`, the 2,610-entry 1990 ownership
        // map seeded from data/districts.json), no behaviour change. The two
        // checks were run before the number was touched: `git diff --
        // spheres-sim/data/` shows one new untracked file, districts.json,
        // and not one existing nation's figures moved; 117 of 119 tests
        // passed untouched — the only reds were this pair. And this time the
        // "struct moved, simulation did not" claim was proven byte for byte
        // rather than argued: stripping the serialized `districts` block out
        // of the new save reproduces the previous pin 0xbffd89cc8498ffaa
        // exactly, so not one pre-existing byte of the start state moved.
        let w = world_1990(GameRules::default());
        let h = state_hash(&w);
        assert_eq!(
            h, 0xd022d50f43c984dau64,
            "the 1990 start state changed (actual {h:#018x})"
        );
    }

    #[test]
    fn golden_hash_of_a_known_run() {
        // A pinned fingerprint of one exact timeline. The two determinism tests
        // build both worlds in one process against one libm, so neither can see
        // a divergence between machines.
        //
        // That used to be an open hole: the sim called `f64::exp`, `f64::powf`
        // and `f64::ln`, none of which IEEE 754 specifies, so this number was
        // only ever a Windows number. It is now expected to hold on every
        // platform, because every transcendental the tick loop touches goes
        // through `crate::exact`, which is built from IEEE-exact primitives
        // only. If this assertion fails on one platform and passes on another,
        // THAT IS THE FINDING: something has slipped back onto the platform
        // libm, or a target is computing f64 in x87 registers. Record the
        // platform and do not simply re-pin the number.
        //
        // Re-pinned once, deliberately, when `exact` replaced the libm calls
        // (Phase 1.3). The move is a few ulps, not a change of model: after 240
        // months every nation's GDP is bit-identical to the old value, the
        // 35-year headline stream is byte-identical, and the 2025 league table
        // is identical to the digit. The one difference measurable at all was
        // France's `tfp_trend` in its sixteenth significant figure. Was
        // 0xb675826e8941683d.
        //
        // Pinned on: Windows, x86_64-pc-windows-gnu, rustc 1.97.1.
        //
        // Re-pinned on the government branch. Elections, coalitions and regime
        // pillars are a deliberate behaviour change: party support now moves
        // every month, coalition strain is a standing deduction on political
        // capital, and unpaid armies remove governments. The timeline is
        // genuinely different from dabaa08's and this fingerprint must move with
        // it. Previous value: 0x0475_a1ec_bc94_bb31.
        //
        // Re-pinned again on adding Spain, 0x5365360981de0aae ->
        // 0x066f5417343f62f9. THIS IS THE ONE TEST IN THE SUITE AN ADDED NATION
        // LEGITIMATELY MOVES, and it will move once per nation as the roster
        // grows to the ~190 BIBLE section 1 commits to. A thirty-first economy
        // in the world draws from the same single RNG in the same tick loop, so
        // every subsequent draw shifts and the whole timeline is a different —
        // not a worse — one. Do not treat that as a regression, and do not try
        // to make it stop moving; the fix for "the golden hash keeps moving" is
        // to re-pin it deliberately, having first confirmed that
        //   (a) the ONLY failing tests are this one and `the_1990_start_is_pinned`,
        //   (b) `git diff -- spheres-sim/data/` contains no change to any
        //       existing nation's figures, and
        //   (c) the emergent-history calibration tests are still green, which
        //       is what actually protects the model.
        // On Spain all three held: 79 of 81 sim tests passed untouched, the
        // data diff was pure addition, and the cross-seed counters barely
        // stirred (USSR 10/10 -> 10/10, Yugoslavia 10/10 -> 10/10,
        // Gulf War 7/10 -> 6/10, China's median 30-year multiple 15.95x ->
        // 15.62x).
        //
        // Re-pinned a third time, 0x066f5417343f62f9 -> 0xc274968416c655b7, at
        // the ten-region integration. 31 nations became 108 in ten separate
        // merges and this number was re-pinned ONCE, at the end, which is the
        // whole reason the merges were sequenced rather than batched.
        //
        // Condition (a) above DID NOT HOLD at the ten-region integration and was
        // not quietly skipped. Four calibration tests were red at 108 nations
        // besides the two hashes:
        //   the_frontier_does_not_run_away   UK 4.37%/yr on the default seed.
        //   arms_transfers_build_a_client_army  1.42 against a bar of 1.50.
        //   a_poor_nation_still_picks_up_what_everyone_has  Afghanistan holds 4
        //     technologies on seed 42 against a floor of 5.
        //   china_growth_miracle  median 30-year multiple 10.13x against a band
        //     of 11.0..19.0, falling monotonically with roster size.
        //
        // Re-pinned a FOURTH time, 0xc274968416c655b7 -> 0xef3e968249846a49, by
        // the refit that was owed on that last entry. The suspicion recorded
        // here was that the catchup coefficient in economy.rs had been fitted
        // against a world GDP 18% too small. It had not been, and the refit is a
        // different change from the one this comment predicted: see SANCTION_BITE
        // in economy.rs. `sanction_drag` counted the flags in a sanctions
        // coalition rather than weighing its share of world output, so the G5
        // regime that forms against China cost a flat 3.0 points of annual
        // growth however large the world got. It now costs 1.5.
        //
        // ALL OF THE MOVEMENT IS THAT ONE LINE, and that is proven rather than
        // asserted. The commit also lifts `sanction_weight` out of
        // `oil_blockade` in world.rs so both terms read one definition; with
        // that extraction in place and the old `sanction_count * 0.006` restored
        // on top of it, this hash reproduces 0xc274968416c655b7 exactly. The
        // refactor is behaviour-neutral and the calibration is the whole change.
        //
        // THREE OF THE FOUR REDS CLEARED, and the movement is distributional
        // rather than a lucky reshuffle, which is the distinction that matters
        // and is why the numbers are here:
        //   china_growth_miracle  10.13x -> 11.16x. The ten seeds span
        //     8.68..17.25 against 6.64..18.39, and at zero sanction drag they
        //     span only 10.01..15.52 — the bimodality really is this one
        //     coefficient. Green, but by 0.16 against an 11.0 floor; the test's
        //     own comment says plainly that this is still fragile and why.
        //   the_frontier_does_not_run_away  UK 4.37%/yr -> 2.91%/yr on the
        //     default seed. Across seeds 0..9 the UK now reads [2.80..3.50] with
        //     ZERO seeds at or over the 4.0 ceiling, against [2.64..4.69] with
        //     two. Every mature economy tightened, so this is the distribution
        //     moving and not the one seed the test reads.
        //   a_poor_nation_still_picks_up_what_everyone_has  Afghanistan 4 -> 10
        //     on seed 42. The poorest nation across twelve seeds is now 6..11
        //     against 4..10. Improved, and still the thinnest margin in the
        //     suite: two seeds sit at 6 against a floor of 5.
        //
        // ONE STAYS RED, and it is untouched by this commit rather than fixed:
        //   arms_transfers_build_a_client_army  10.9 vs 7.7, the same two
        //     figures it failed on before the refit. A single-seed ratio sitting
        //     at 1.4993 against a bar of 1.50, per the note in nations.rs. It is
        //     not an economic-calibration failure and it is not this commit's.
        // NO TOLERANCE IN THIS SUITE WAS WIDENED AND NO TEST WAS REMOVED. One
        // test was ADDED, `sanctions_cost_the_target_real_growth`, because the
        // audit found that nothing in the suite constrained the coefficient this
        // commit changed from below: at bite 0.000, with sanctions costing a
        // target no growth at all, everything except the hashes stayed green.
        //
        // RE-PINNED FOR THE COMMITMENT LADDER, 0xef3e968249846a49 ->
        // 0x448f87451f44d25d. This is the one thing a deliberate behaviour
        // change is allowed to move, and this is as deliberate as it gets: war
        // stopped being a strength ratio pushing a progress bar and became nine
        // rungs each side picks for itself. Twenty years of timeline cannot
        // possibly hash the same, and a hash that DID survive that would mean
        // the ladder was not reaching the world.
        //
        // What the rest of the suite says about the same change, so the number
        // is not the only evidence:
        //   - 93 tests green, 9 ignored, 0 red besides this pair.
        //   - Not one tolerance widened and not one test deleted. Two tests were
        //     re-expressed and the reasoning is in their own comments and in the
        //     commit; one test was added.
        //   - `determinism_same_seed_same_world` and
        //     `save_load_roundtrip_continuity` are green, so the new state —
        //     conflicts, postures, theatres, access — round-trips and replays.
        // Measured with the same twelve thirty-year runs used to diagnose the
        // pact test: 152 conflicts are born where master fought 197 wars, and
        // 77 of them climb to an invasion. The rest sit at rungs nobody could
        // reach before, which is §6's whole claim about the period.
        // Re-pinned, 0xaa7960badaee3a49 -> 0x57724ed8dd8fc5ef, for the same
        // reason and on the same evidence as `the_1990_start_is_pinned` above:
        // `WorldState` gained `player_set_rate`, so the fingerprint of the state
        // moved while the twenty years of history inside it did not. The
        // trajectory dump described there covers this run too.
        //
        // Re-pinned for research projects, on the same evidence as
        // `the_1990_start_is_pinned` above: a new `TechState` field that is
        // `None` everywhere nobody has declared a programme.
        //
        // Re-pinned for the procurement layer, on the same evidence as
        // `the_1990_start_is_pinned`: a new `Nation` field, no behaviour change.
        //
        // Re-pinned with the 1990 inheritance, same evidence as above.
        //
        // Re-pinned once more, and this one is a deliberate behaviour change
        // rather than a struct move: war.rs now scales sustained strength by
        // arsenal::adequacy. Twenty years of timeline cannot hash the same when
        // what a nation can field depends on what it actually bought.
        //
        // Re-pinned for district ownership, same evidence as
        // `the_1990_start_is_pinned` above: a new `WorldState` field, no
        // behaviour change. The district map DOES move inside this run — the
        // union dissolves and its ground passes to fifteen heirs — but the
        // transfers write only `w.districts`: no helper draws from the RNG
        // or touches a `Nation` field. Proven rather than argued: stripping
        // the serialized `districts` block out of this run's save reproduces
        // the previous pin 0x26e13d8d29a02476 exactly, so all twenty years
        // of timeline are byte-identical and the fingerprint moves only
        // because the save now carries the map.
        // Re-pinned for the front projection — BIBLE §5 as amended 2026-08-30
        // — 0x5853f63c87f05b17 -> 0xe383c17ab4499bfa, a deliberate behaviour
        // change, sanctioned in the change that made it. Wars now carry a
        // district front and pockets in the save, a settlement cedes the
        // ground the winner's front actually holds rather than the pure
        // value ranking, and an encircled garrison degrades — so both the
        // serialized shape and the late-run map genuinely move. The checks
        // the protocol above demands all held before this number moved:
        // every emergent-history calibration test is green untouched, the
        // data diff under spheres-sim/data/ is empty, and the instrument
        // batch measured the aggregate's behaviour unchanged to four decimal
        // places (staged-Gulf trajectory month by month, the desert-storm
        // months-to-end distribution 37/31/19/22/25/26/24/15, invasion count
        // 5/10, settlement rate 12/12 — all identical before and after). The
        // two other movers were the seed-pinned conquest pair, re-scanned to
        // seed 17 per their own comments.
        //
        // Re-pinned for the terrain pass — BIBLE §5's amendment carried to
        // its second half: the per-district class (transcribed from Natural
        // Earth by tools/terrain/) supersedes the theatre `rough` scalar
        // inside the front's capped phase, and ground reachable only across
        // a major river moves at half tempo. A deliberate distribution
        // change, sanctioned in the change that made it: every front in the
        // pinned timeline redistributes, so the serialized `conflict.front`
        // maps — and everything the RNG consumes after them — genuinely
        // move. The protocol's checks all held before this number did:
        // `the_1990_start_is_pinned` DID NOT MOVE (terrain lives in static
        // tables outside WorldState and the hash), the districts.json
        // regeneration was verified purely additive (id/name/area_sqkm/adj
        // byte-for-byte, only t/f/riv added), and every emergent-history
        // calibration test is green untouched —
        // `the_aggregate_tracks_the_scalar_it_replaced` inside its 0.10
        // bound, because the uncapped sweep that preserves the aggregate is
        // untouched code. The two other movers were, once again, the
        // seed-pinned conquest pair, re-scanned to seed 9 per their own
        // comments (Mongolia 2027; the tempo pass dissolved seed 17's
        // Bhutan). Previous value: 0xe383c17ab4499bfa.
        //
        // Re-pinned for the no-party electoral fix, 0x5fc8093ab8c34f53 ->
        // 0xbd5ec0f43c5f2e3b, a deliberate behaviour change. `is_electoral`
        // now requires a non-empty party table: the seven states transcribed
        // with none (Saudi Arabia, the smaller Gulf monarchies, Brunei, the
        // Maldives) used to fall through to the electoral branch when a
        // revolution dropped their authoritarianism under the ceiling, where
        // `hold_election` had nobody to seat and nothing to reset — "the
        // government falls; the country goes to the polls" printed every
        // month for a decade, and the state underneath was frozen. They now
        // stay pillar regimes at any openness, which restores coups,
        // patronage and upkeep to a fifth of the Gulf: their stability, GDP
        // and political-capital paths genuinely move, and everything the RNG
        // consumes after them moves with it. The protocol's checks all held
        // before this number did: `the_1990_start_is_pinned` DID NOT MOVE
        // (all seven start authoritarian, so no Jan 1990 government changes),
        // every emergent-history calibration test is green untouched, no
        // tolerance widened and no test deleted, and one test was ADDED —
        // `a_state_with_no_parties_never_loops_through_the_polls`, red
        // against the loop before the fix. The two other movers were, as
        // ever, the seed-pinned conquest pair, re-scanned to seed 93 per
        // their own comments (Saudi Arabia takes Qatar 2018; the fix
        // dissolved seed 9's Mongolia).
        const GOLDEN: u64 = 0xbd5ec0f43c5f2e3b;
        let mut w = world_1990(GameRules::default());
        run_months(&mut w, 12 * 20);
        let h = state_hash(&w);
        assert_eq!(h, GOLDEN, "timeline fingerprint changed (actual {:#018x})", h);
    }

    #[test]
    fn different_seeds_diverge() {
        // The other half of determinism, and the one that catches a seed being
        // silently ignored: identical rules but a different seed must produce a
        // genuinely different history, not a cosmetic reshuffle.
        let mut a = world_1990(GameRules::default());
        let mut b = world_1990(GameRules { seed: 7, ..GameRules::default() });
        run_months(&mut a, 240);
        run_months(&mut b, 240);
        assert_ne!(state_hash(&a), state_hash(&b), "different seeds produced identical worlds");
    }

    #[test]
    fn state_hash_agrees_with_the_serialized_world() {
        let mut a = world_1990(GameRules::default());
        let mut b = world_1990(GameRules::default());
        run_months(&mut a, 120);
        run_months(&mut b, 120);
        assert_eq!(state_hash(&a), state_hash(&b));
        run_months(&mut b, 1);
        assert_ne!(state_hash(&a), state_hash(&b), "hash blind to a month passing");
    }

    #[test]
    fn japans_bubble_becomes_a_lost_decade() {
        // Japan starts with a bubble at 0.95. It must pop, and the hangover must
        // be a decade of stagnation rather than a single bad year — and Japan
        // must not overtake the United States on the way through.
        let mut w = world_1990(GameRules::default());
        run_months(&mut w, 24);
        let peak = w.nation(NationId::Japan).gdp;
        run_months(&mut w, 120); // through to ~2002
        let japan = w.nation(NationId::Japan);
        let decade_growth = japan.gdp / peak;
        assert!(
            decade_growth < 1.45,
            "no lost decade: Japan grew {:.2}x in the ten years after the peak",
            decade_growth
        );
        assert!(
            japan.gdp < w.nation(NationId::USA).gdp,
            "Japan overtook the USA: {:.0} vs {:.0}",
            japan.gdp,
            w.nation(NationId::USA).gdp
        );
    }

    #[test]
    fn stable_democracies_never_hyperinflate() {
        // A standing invariant, not a calibration: if an open, stable economy
        // can spiral in this model, the monetary framework is broken.
        let democracies = [
            NationId::USA, NationId::Japan, NationId::Germany,
            NationId::UK, NationId::France, NationId::Italy,
        ];
        for seed in 0..5u64 {
            let mut w = world_1990(GameRules { seed, ..GameRules::default() });
            for _ in 0..480 {
                tick_month(&mut w, &[]);
                for id in democracies {
                    let n = w.nation(id);
                    if !n.alive || n.stability < 40.0 || w.at_war(id) {
                        continue; // a state in crisis is allowed to be a mess
                    }
                    assert!(
                        n.inflation < 0.50,
                        "{:?} hyperinflated to {:.0}% at {} on seed {}",
                        id, n.inflation * 100.0, w.date_str(), seed
                    );
                }
            }
        }
    }

    #[test]
    fn rich_democracies_settle_near_target() {
        // The quiet background condition the whole model rests on: absent shocks,
        // a mature economy converges on modest growth and inflation near target.
        let mut w = world_1990(GameRules::default());
        w.rules.ai_aggression = 0.0; // no wars: we are testing the resting state
        run_months(&mut w, 480);
        for id in [NationId::USA, NationId::Germany, NationId::France] {
            let n = w.nation(id);
            // Measured across seeds the resting state is far tighter than the
            // bands these assertions used to allow: growth settles near 2% and
            // inflation within a few basis points of target. Tolerances wide
            // enough to admit anything cannot fail when something moves.
            assert!(
                (0.005..0.035).contains(&n.growth_last),
                "{:?} resting growth {:.1}% is not a mature economy",
                id, n.growth_last * 100.0
            );
            assert!(
                (0.010..0.035).contains(&n.inflation),
                "{:?} resting inflation {:.1}% never converged",
                id, n.inflation * 100.0
            );
        }
    }

    #[test]
    fn save_load_roundtrip_continuity() {
        let mut a = world_1990(GameRules::default());
        run_months(&mut a, 60);
        let snapshot = save(&a);
        let mut b = load(&snapshot).unwrap();
        run_months(&mut a, 120);
        run_months(&mut b, 120);
        assert_eq!(save(&a), save(&b), "diverged after save/load");
    }

    // ---- Districts: political geography, owned at 1990, moved at outcomes ----

    #[test]
    fn every_nation_with_a_polygon_has_districts() {
        let w = world_1990(GameRules::default());
        // The six whose TERRITORY polygon list is empty — the map cannot place
        // them, so the district file ships them an empty list on purpose.
        let no_polygon = [
            NationId::Bahrain,
            NationId::Mauritius,
            NationId::Seychelles,
            NationId::Comoros,
            NationId::CapeVerde,
            NationId::Maldives,
        ];
        for id in nations::start_nations().iter().copied() {
            if no_polygon.contains(&id) {
                continue;
            }
            assert!(
                w.districts.values().any(|&o| o == id),
                "{:?} starts 1990 owning no district at all",
                id
            );
        }
        // Partition property: every start nation's list is fully owned and no
        // district is owned twice or dropped.
        let expected: usize = nations::start_nations()
            .iter()
            .map(|id| districts::list_of(*id).len())
            .sum();
        assert_eq!(
            w.districts.len(),
            expected,
            "the 1990 ownership map does not partition the start nations' lists"
        );
        // Delta encoding starts empty: at 1990 nobody differs from the default.
        assert!(
            districts::deltas(&w).is_empty(),
            "the 1990 start already carries district deltas"
        );
    }

    #[test]
    fn annexation_moves_every_district() {
        // Kuwait: 6 districts, population well under the 8m annexation bar.
        // Two identically-built worlds must move identical maps — this goes
        // red against any HashMap-ordered or partial implementation.
        let kuwait_ids = ["KW-AH", "KW-FA", "KW-HA", "KW-JA", "KW-KU", "KW-MU"];
        let mut worlds = [world_1990(GameRules::default()), world_1990(GameRules::default())];
        for w in &mut worlds {
            let moved = districts::annex_all(w, NationId::Iraq, NationId::Kuwait);
            assert_eq!(moved, 6, "Kuwait holds exactly its six governorates");
            assert!(
                !w.districts.values().any(|&o| o == NationId::Kuwait),
                "annexation left Kuwait holding ground"
            );
            let iraqi: Vec<&String> = w
                .districts
                .iter()
                .filter(|&(_, &o)| o == NationId::Iraq)
                .map(|(d, _)| d)
                .collect();
            assert_eq!(iraqi.len(), 18 + 6, "Iraq's own 18 plus all of Kuwait's 6");
            for k in kuwait_ids {
                assert!(
                    w.districts.get(k) == Some(&NationId::Iraq),
                    "{} did not move to Iraq",
                    k
                );
            }
        }
        let [a, b] = worlds;
        assert_eq!(a.districts, b.districts, "the same annexation moved different maps");
    }

    #[test]
    fn a_settled_peace_moves_the_biggest_district() {
        // End-to-end through the real war machinery: the desert-storm setup
        // with the coalition kept home (no major stands at >= 40 with the
        // victim, so nobody intervenes). Alone against Iraq, Kuwait's resolve
        // collapses and it sues for peace — the SETTLED ending, whose
        // `cede = 0.12` over six districts is ceil(0.72) = one district, and
        // the value ranking says it is Jahra, the big desert governorate.
        let mut settled = 0;
        for seed in 0..3u64 {
            let mut w = world_1990(GameRules { seed, ..GameRules::default() });
            w.rules.ai_aggression = 0.0;
            for m in nations::majors().iter().copied() {
                w.set_relation(m, NationId::Kuwait, 0.0);
            }
            war::declare_war(&mut w, NationId::Iraq, NationId::Kuwait).unwrap();
            let mut sued = false;
            for _ in 0..60 {
                let hs = tick_month(&mut w, &[]);
                if hs.iter().any(|h| h.contains("Kuwait sues for peace, ceding territory to Iraq")) {
                    sued = true;
                    break;
                }
                if w.conflict_between(NationId::Iraq, NationId::Kuwait).is_none()
                    || !w.nation(NationId::Kuwait).alive
                {
                    break;
                }
            }
            if !sued {
                continue;
            }
            settled += 1;
            assert_eq!(
                w.districts.get("KW-JA"),
                Some(&NationId::Iraq),
                "seed {}: the ceded territory headline ran and Jahra never moved",
                seed
            );
            for k in ["KW-AH", "KW-FA", "KW-HA", "KW-KU", "KW-MU"] {
                assert_eq!(
                    w.districts.get(k),
                    Some(&NationId::Kuwait),
                    "seed {}: {} moved in a concession that should take exactly one",
                    seed,
                    k
                );
            }
            assert!(
                districts::deltas(&w)
                    .iter()
                    .any(|(d, o)| d == "KW-JA" && *o == NationId::Iraq),
                "seed {}: the concession is missing from the delta payload",
                seed
            );
        }
        assert!(
            settled >= 1,
            "no seed reached a negotiated concession — the guard never ran"
        );
    }

    /// End-to-end for the ANNEXATION site. Whoever dies by conquest must leave
    /// the map entirely; a dissolution is the other way off the board and hands
    /// its ground to heirs instead.
    ///
    /// SWEPT, NOT PINNED, since 2026-08-31. This rode seed 93 alongside
    /// `a_large_nation_is_subjugated_rather_than_swallowed` and went stale with
    /// it — the pin had been Saudi Arabia/Qatar in 2018, and before that seed
    /// 9's Mongolia (lost to the no-party electoral fix), seed 17's Bhutan (the
    /// terrain tempo pass) and seed 0's Malta (the front projection). Four pins,
    /// four unrelated changes, four re-pins.
    ///
    /// ── RE-SAMPLED 2026-08-31, ON RIDGE'S EXPLICIT AUTHORISATION ────────────
    ///
    /// Recorded the way a BIBLE amendment is, and for the same reason as its
    /// sibling: iron rule 5 forbids an agent editing a calibration test in
    /// answer to a red, so an authorised strengthening has to be legible as one.
    /// Ridge, 2026-08-31, ruling 2, in his own terms: annexation "MUST KEEP ITS
    /// OWN BAR, honestly stated at its real rate of about 4-in-200, rather than
    /// being retired as a policed quantity". That is what the lower bar below
    /// is, and it is why this test did not simply hand its annexation counting
    /// over to the sibling test that now counts every conquest ending.
    ///
    /// WHAT WAS MIS-SAMPLED. `annexations > 0` over twenty seeds. Annexation is
    /// a per-seed event of about 4 in 100 — `conquest_size_rule_scan` measures
    /// 0.0417 over seeds 0..240, and the parent measurement pass read about half
    /// that on the pre-repair tree — so the FALSE-RED PROBABILITY of that bar
    /// was 0.9583^20 = 43% at the measured rate and 0.98^20 = 67% at the
    /// pessimistic one. A bar that reds between two and three times in five when
    /// nothing whatever is wrong is not measuring the model. And the invariant
    /// it guards was going along for the ride: with no annexation in the window
    /// `districts::annex_all` is never reached, so the whole test passed on
    /// dissolutions alone — which is exactly what it does under a perturbation
    /// that switches annexation off, 480 deaths and not one of them a conquest.
    ///
    /// THE SAMPLE IS DERIVED, per the same day's ruling 3, not guessed. The
    /// event is per-seed Bernoulli, so n = ln(0.01) / ln(1 - rate): 109 seeds at
    /// the measured 0.0417, and 228 at the pessimistic 0.02 the parent pass read
    /// before the capital repair. 240 is taken because it clears BOTH — a false
    /// red of 0.0000 at today's rate and 0.0079 at the worse one, inside ruling
    /// 3's 1% ceiling either way — and because the rate is evidently something
    /// a growth change can halve. If it ever halves again, widen the sample by
    /// that formula; do not lower the bar.
    ///
    /// THE BAR IS TWO-SIDED, which it was not before, because "annexation is
    /// rare" is half the claim and "annexation still happens" is the other half.
    /// The ceiling of 40 is sized against what the size rule being deleted
    /// actually looks like, measured rather than projected: with both clauses
    /// widened to admit everything, this sweep read 84 annexations on
    /// 2026-08-31. (The arithmetic null of "every conquest ending annexes" is
    /// 107 — 0.446 per seed over 240 — but deleting the rule changes the world
    /// it is deleted from, since a nation annexed in 1998 fights no wars after,
    /// so 84 is the honest figure and 107 is not.) The ceiling therefore sits
    /// about 4 sd below the deleted-rule reading and 9.7 sd above the honest
    /// mean of 10. It is deliberately not tighter: a partial widening of the
    /// bound to 80m stayed under it, and catching THAT is the sibling test's
    /// job, which it does by naming the annexed nation.
    ///
    /// THE COUNT IS TAKEN FROM `war::conquer`'s OWN HEADLINE rather than from
    /// "died and is not the USSR or Yugoslavia". The old exclusion list was a
    /// standing hazard: any third state that ever leaves the board by
    /// dissolution would have been silently counted as an annexation, and the
    /// list had no way to say so. The headline is written on exactly the annex
    /// branch and nowhere else.
    #[test]
    fn a_dead_nation_holds_no_districts() {
        let mut annexations = 0usize;
        let mut deaths = 0usize;
        for seed in 0..240u64 {
            let mut w = seeded(seed);
            let mut alive: Vec<NationId> =
                w.nations.iter().filter(|n| n.alive).map(|n| n.id).collect();
            for _ in 0..480 {
                let headlines = tick_month(&mut w, &[]);
                annexations += headlines.iter().filter(|h| h.contains(" has annexed ")).count();
                let mut still: Vec<NationId> = Vec::new();
                for id in alive {
                    if w.nation_opt(id).is_some_and(|n| n.alive) {
                        still.push(id);
                        continue;
                    }
                    deaths += 1;
                    assert!(
                        !w.districts.values().any(|&o| o == id),
                        "{:?} is off the board in {} on seed {} and still holds districts",
                        id,
                        w.year,
                        seed
                    );
                }
                alive = still;
            }
        }
        assert!(
            annexations >= 1,
            "no annexation anywhere in 240 seeds of forty years ({} death(s) on the board, all \
             of them dissolutions), so `districts::annex_all` was never reached and the invariant \
             above passed vacuously — conquest may have become unreachable (BUGS.md O-1). \
             Re-scan with `conquest_size_rule_scan` before touching this test",
            deaths
        );
        assert!(
            annexations <= 40,
            "{} annexations in 240 seeds — annexation is meant to be the rare arm of the size \
             rule at roughly four seeds in a hundred, and at this rate it has stopped refusing \
             anybody. Check `war::conquer`'s 8m and 0.6 clauses, and \
             `a_large_nation_is_subjugated_rather_than_swallowed` alongside",
            annexations
        );
    }

    #[test]
    fn concession_cedes_the_value_ranked_subset() {
        // ceil(0.12 * 18) = 3: Iraq's three largest districts by (area desc,
        // id asc), pinned literally from the transcribed data so a change to
        // either the ranking rule or the census goes red here.
        let expected = ["IQ-AN", "IQ-MU", "IQ-NI"];
        let mut worlds = [world_1990(GameRules::default()), world_1990(GameRules::default())];
        let mut returns = vec![];
        for w in &mut worlds {
            let ceded = districts::cede_share(w, NationId::Iran, NationId::Iraq, 0.12);
            assert_eq!(ceded, expected, "the value ranking moved");
            for d in &ceded {
                assert_eq!(w.districts.get(d.as_str()), Some(&NationId::Iran));
            }
            assert_eq!(
                w.districts.values().filter(|&&o| o == NationId::Iraq).count(),
                15,
                "Iraq keeps the other fifteen"
            );
            returns.push(ceded);
        }
        assert_eq!(returns[0], returns[1], "same concession, different subsets");
        let [a, b] = worlds;
        assert_eq!(a.districts, b.districts, "same concession, different maps");

        // A nuclear loser's peace carries cede = 0.0 and moves nothing.
        let mut w = world_1990(GameRules::default());
        assert!(districts::cede_share(&mut w, NationId::Iran, NationId::Iraq, 0.0).is_empty());
        assert!(districts::deltas(&w).is_empty());

        // A loser holding one district survives the peace with it: hand-craft
        // Kuwait down to one governorate first.
        for k in ["KW-FA", "KW-HA", "KW-JA", "KW-KU", "KW-MU"] {
            w.districts.insert(k.into(), NationId::Iraq);
        }
        assert!(
            districts::cede_share(&mut w, NationId::Iran, NationId::Kuwait, 0.12).is_empty(),
            "a one-district loser must cede nothing"
        );
        assert_eq!(w.districts.get("KW-AH"), Some(&NationId::Kuwait));
    }

    #[test]
    fn an_old_save_reseeds_its_districts() {
        // A save from before districts existed has no "districts" key at all;
        // serde defaults it empty and load() must rebuild the 1990 map rather
        // than hand back a world where nobody owns anything.
        let w = world_1990(GameRules::default());
        let text = save(&w);
        let mut v: serde_json::Value = serde_json::from_str(&text).unwrap();
        v.as_object_mut()
            .unwrap()
            .remove("districts")
            .expect("a fresh save carries its districts");
        let stripped = serde_json::to_string(&v).unwrap();
        let old = load(&stripped).expect("a pre-district save must still load");
        assert!(!old.districts.is_empty(), "the reseed hook never fired");
        assert_eq!(
            old.districts, w.districts,
            "the reseeded map differs from the 1990 default"
        );
    }

    #[test]
    fn ussr_dissolution_hands_ukraine_its_districts() {
        // Drive the real dissolution the way ussr_collapses_in_the_nineties
        // does, stop on the month the flag appears, and read the map.
        let mut dissolved = 0;
        for seed in 0..10u64 {
            let mut w = world_1990(GameRules { seed, ..GameRules::default() });
            for _ in 0..132 {
                tick_month(&mut w, &[]);
                if w.has_flag("ussr_dissolved") {
                    break;
                }
            }
            if !w.has_flag("ussr_dissolved") {
                continue;
            }
            dissolved += 1;
            assert!(
                !w.districts.values().any(|&o| o == NationId::USSR),
                "seed {}: the union is gone and still holds ground",
                seed
            );
            for d in districts::list_of(NationId::Ukraine) {
                assert_eq!(
                    w.districts.get(d.as_str()),
                    Some(&NationId::Ukraine),
                    "seed {}: {} did not pass to Ukraine",
                    seed,
                    d
                );
            }
            for d in districts::list_of(NationId::Russia) {
                assert_eq!(
                    w.districts.get(d.as_str()),
                    Some(&NationId::Russia),
                    "seed {}: {} did not pass to Russia",
                    seed,
                    d
                );
            }
            // And the delta payload now says so, by name.
            assert!(
                districts::deltas(&w)
                    .iter()
                    .any(|(d, o)| d == "UA-30" && *o == NationId::Ukraine),
                "seed {}: Kyiv City is missing from the delta",
                seed
            );
        }
        assert!(dissolved >= 1, "no seed dissolved the union inside eleven years");
    }

    #[test]
    fn economic_invariants_50_years() {
        let mut w = world_1990(GameRules::default());
        for _ in 0..600 {
            tick_month(&mut w, &[]);
            for n in w.nations.iter().filter(|n| n.alive) {
                assert!(n.gdp.is_finite() && n.gdp > 0.0, "{:?} gdp broke: {}", n.id, n.gdp);
                assert!(n.inflation.is_finite(), "{:?} inflation NaN", n.id);
                assert!(n.debt_gdp.is_finite() && n.debt_gdp < 6.0, "{:?} debt spiral: {}", n.id, n.debt_gdp);
                assert!((0.0..=100.0).contains(&n.stability));
            }
            assert!(w.oil_price.is_finite() && w.oil_price > 0.0);
        }
    }

    #[test]
    fn ussr_collapses_in_the_nineties() {
        // Historical calibration: across seeds, the USSR should usually dissolve by 2000.
        let mut collapses = 0;
        for seed in 0..10u64 {
            let rules = GameRules { seed, ..GameRules::default() };
            let mut w = world_1990(rules);
            for _ in 0..132 {
                tick_month(&mut w, &[]);
            }
            if w.has_flag("ussr_dissolved") {
                collapses += 1;
            }
        }
        assert!(collapses >= 6, "USSR survived too often: {}/10 collapses", collapses);
    }

    /// The seeds in `seeds` whose first four years produce an Iraqi invasion of
    /// Kuwait. `aggression` is the arm, the same shape `conquests` uses: at 1.0
    /// the AI fights, at 0.0 `dyads.rs:273` multiplies every appetite in the
    /// world by zero and nobody attacks anybody.
    ///
    /// The range became a parameter on 2026-08-31 so the bar and the instrument
    /// read the same code over different widths. Nothing inside changed.
    fn gulf_wars(seeds: std::ops::Range<u64>, aggression: f64) -> Vec<u64> {
        let mut hits = vec![];
        for seed in seeds {
            let mut w = seeded(seed);
            w.rules.ai_aggression = aggression;
            let mut saw = false;
            for _ in 0..48 {
                let headlines = tick_month(&mut w, &[]);
                if headlines.iter().any(|h| h.contains("Iraq invades Kuwait")) {
                    saw = true;
                }
            }
            if saw || !w.nation(NationId::Kuwait).alive {
                hits.push(seed);
            }
        }
        hits
    }

    #[test]
    fn gulf_war_emerges() {
        // Iraq should invade Kuwait in a majority of early-90s runs.
        //
        // WIDENED FROM TEN SEEDS TO FORTY, AND THE BAR IS UNMOVED. It read
        // `invasions >= 5` out of ten; it reads twenty out of forty, which is
        // the identical claim — a majority of worlds — asked of four times the
        // evidence. The bar's meaning is the whole point of the conversion and
        // it is not what was wrong here.
        //
        // WHAT WAS WRONG WAS THE SAMPLE. Ten seeds against a bar of five cannot
        // distinguish a rate of 42% from a rate of 50%: both put the expected
        // count within one seed of the bar, so the test flipped colour on any
        // change that touched the shared RNG stream, and it flipped in both
        // directions without the model's war incidence moving at all. It has
        // been red at 4/10 and green at 6/10 on trees whose forty-seed rate was
        // the same number. A bar that a reshuffle can cross is not measuring
        // the mechanism it names.
        //
        // At forty seeds the standard error on a rate near a half is about 8
        // points rather than 16, which is what it takes to tell those two rates
        // apart. `gulf_war_incidence_scan` is the wider readout, and it prints
        // the first ten alongside so the old reading stays legible.
        //
        // The bar is NOT a fitted number and was not re-derived here: SPEC and
        // this test's own first line say a majority, and a majority of forty is
        // twenty. If the true rate ever sits below it, that is a finding about
        // the model's appetite pass and belongs in a bug entry, not in this
        // literal.
        //
        // ── WIDENED AGAIN 2026-08-31, TO TWO HUNDRED, ON RIDGE'S EXPLICIT
        //    AUTHORISATION ────────────────────────────────────────────────────
        //
        // Recorded here the way a BIBLE amendment is, because iron rule 5
        // otherwise forbids an agent editing a calibration test in answer to a
        // red, and a reader must be able to tell an authorised strengthening
        // from a quiet widening. Ridge, 2026-08-31, ruling 2: "widen the Gulf
        // War sample past 40 seeds", and explicitly — ruling 2 again — "the 50%
        // bar is doctrinal, was never fitted, and the true rate is 61.85% — so
        // the bar does NOT move, only the sample."
        //
        // THE BAR DID NOT MOVE. It was five out of ten, then twenty out of
        // forty, and it is now a hundred out of two hundred. Three literals,
        // one claim, unchanged since the root commit: a majority of worlds.
        //
        // WHAT WAS STILL MIS-SAMPLED AT FORTY. The parent measurement pass put
        // the true rate at 61.85% [59.70, 63.95] over 2000 seeds; this tree
        // reads 246/400 = 61.5%, which is the same number. The model therefore
        // clears a 50% bar by twelve points and always has. But forty seeds
        // against a bar of twenty leaves a standard deviation of 3.1 seeds and
        // a margin of 4.6, so the false-red probability is about 7% — seven
        // times ruling 3's ceiling. It was not theoretical: seeds 0..40 read
        // 21/40 on this tree, ONE SEED above the bar, and the parent pass found
        // 0..40 to be the worst of fifty consecutive blocks. The test was one
        // unlucky reshuffle from a red that said nothing about Iraq.
        //
        // THE SAMPLE IS DERIVED, per the same day's ruling 3, from this test's
        // own measured variance. The event is per-seed Bernoulli, so the
        // variance IS p(1-p) = 0.2368 at the measured p = 0.615, and the bar
        // sits (p - 0.5) below the truth: n = (2.326 * sd / (p - 0.5))^2 = 97
        // seeds for a false red under 1%. `gulf_war_incidence_scan` prints that
        // arithmetic from its own sample so it can be re-derived rather than
        // inherited. Two hundred is taken — ruling 3's own named target for this
        // test — which puts the bar 3.34 sd from the truth, a false red of about
        // 0.04%, and leaves headroom for a rate that drifts several points
        // without this test crying wolf. The measured reading at two hundred is
        // 125/200 against a bar of 100 — twenty-five seeds of margin, where
        // forty seeds gave one.
        const N: u64 = 200;
        let hits = gulf_wars(0..N, 1.0);
        assert!(
            hits.len() >= (N / 2) as usize,
            "Gulf War too rare: {}/{} seeds — {:?}",
            hits.len(),
            N,
            hits
        );

        // The control arm, added with the forty-seed widening and carried
        // across: the count above has to come from the appetite pass rather
        // than from anything else that can put Kuwait off the board in four
        // years. With the AI's appetite for war at zero there is no invasion in
        // any of the two hundred.
        let control = gulf_wars(0..N, 0.0);
        assert!(
            control.is_empty(),
            "Iraq invaded Kuwait in a world where the AI's appetite for war is zero: {:?}",
            control
        );
    }

    #[test]
    fn china_growth_miracle() {
        // A cross-seed band, in the style of `ussr_collapses_in_the_nineties`,
        // replacing what used to be a single run on the default seed asserting
        // `6.0 < x < 14.0`. That assertion was a lottery ticket that happened to
        // be a winner, and the ceiling was in the wrong place besides. Both
        // claims are measured, not asserted:
        //
        // 1. The single sample was not representative. Master's 30-year multiple
        //    on seeds 0..=9 is
        //      11.03 13.30 13.77 14.70 14.86 16.50 16.68 16.74 17.13 17.19
        //    so the old ceiling of 14.0 was breached on SIX of ten seeds. It
        //    passed on the default seed 1990 (13.03x) for a reason that has
        //    nothing to do with growth: China invades Vietnam in Oct 1995 on
        //    that seed and eats coalition sanctions until Apr 2004. Every seed
        //    that came in under 14.0 is a seed where China got sanctioned; every
        //    seed where it stayed at peace — 2, 3, 6, 7, 9 — ran 16.5x to 17.2x.
        //    The test was measuring a dyad with monthly probability 0.0003, not
        //    the growth model.
        //
        // 2. The ceiling was below reality. China's real GDP in constant 2015
        //    US$ was $1.041tn in 1990 and $14.92tn in 2020 — a multiple of
        //    14.33x, or 9.28% a year compounded (World Bank NY.GDP.MKTP.KD,
        //    series CHN). A ceiling of 14.0 therefore excluded the actual
        //    outcome: the old test could only pass if something knocked China
        //    OFF its historical trajectory. That is a broken assertion, not a
        //    tight one.
        //
        // So this is not a widened tolerance — it is a statistical assertion
        // replacing a single-sample one, and it is anchored on a figure the old
        // one contradicted. The band on the median, 11.0x to 19.0x, is 8.3% to
        // 10.2% a year against reality's 9.28%; master's median sits at 15.68x
        // (9.61%/yr), so the model runs about a third of a point hot, which is
        // inside what §8 of BIBLE.md allows a major economy. The width is set to
        // catch the regression class the old comment cared about: a change that
        // moves this figure by a third in either direction leaves the band.
        //
        // The floor of 6.0x is kept, and kept per-seed rather than on the
        // median, because "the miracle happened" should be true in every world,
        // sanctions or no.
        //
        // Checked red in both directions per iron rule 5, by moving the catchup
        // coefficient in economy.rs (the term that actually drives this; note
        // that perturbing `tfp_trend` does NOT, because tech/mod.rs:1103
        // rewrites it every tick). RE-MEASURED AT 108 NATIONS — the table this
        // replaces was taken at 31 and every figure in it had moved:
        //      catchup 0.000 -> median  6.72x  RED (floor)
        //      catchup 0.010 -> median  9.59x  RED (floor)
        //      catchup 0.020 -> median 11.16x  green  (shipped)
        //      catchup 0.030 -> median 19.77x  RED (ceiling)
        // So the band still admits the shipped value and rejects a third either
        // way, including deleting the term outright, which is the exact
        // regression the comment on that line records having happened before.
        //
        // WHAT MOVED THIS TEST WHEN THE ROSTER GREW, AND WHAT DID NOT. The
        // median fell 14.57x -> 13.08x -> 10.13x at 31, 91 and 108 nations and
        // the suspicion was that the catchup coefficient had been fitted against
        // a world GDP 18% too small. It had not been. Two things were measured
        // before anything was changed:
        //
        //   - The affordability denominator in `tech::tick` was swept 3.2x,
        //     spanning the 31-nation world and beyond the 108-nation one. The
        //     median went 13.34, 11.23, 11.89, 10.13, 11.64, 10.11 — non-
        //     monotone noise, not a response. The numbers are in the comment
        //     there and the denominator was left alone.
        //   - With `ai_aggression = 0.0`, so China simply grows, the 108-nation
        //     median is 14.02x against the real 14.33x. The growth model was
        //     never wrong.
        //
        // This figure is a war-incidence measurement wearing a growth test's
        // clothes. At 108 nations China has fourteen land neighbours instead of
        // two and fights in 6 of 10 seeds instead of 4; the old count-based
        // `sanction_drag` then charged the G5 coalition a flat 3.0 points of
        // annual growth for fifteen years and more, which is what produced the
        // bimodality `nations.rs` documents at its East Asia block. Pricing that
        // regime by the sanctioners' share of world output instead — the fix in
        // economy.rs — is what brought this back inside the band.
        //
        // IT IS STILL A FRAGILE TEST AND THIS COMMIT DID NOT MAKE IT ROBUST.
        // The median sits at 11.16x against a floor of 11.0x. The bimodality is
        // narrower than it was (the ten seeds span 8.68-17.25 against master's
        // 6.64-18.39, and at zero sanction drag they span only 10.01-15.52, so
        // the spread really is this one coefficient) but it has not gone. Two
        // named causes remain, both out of scope here and both larger than a
        // constant: China's sanctions regimes run 16-21 years, longer than Iraq
        // gets for annexing a country, which is a grievance-decay question in
        // politics.rs; and the war incidence itself is the dyads question
        // nations.rs already flags as needing a sealift term.
        // ── WIDENED TEN SEEDS -> A HUNDRED, 2026-08-31, ON RIDGE'S EXPLICIT
        //    AUTHORISATION ────────────────────────────────────────────────────
        //
        // Recorded the way a BIBLE amendment is, because iron rule 5 otherwise
        // forbids an agent editing a calibration test in answer to a red and a
        // reader must be able to tell an authorised strengthening from a quiet
        // widening. Ridge, 2026-08-31, ruling 3, which became iron rule 7:
        // "Known targets: Gulf War n>=200, China n>=100".
        //
        // NO BAR MOVED. The band is the same `(11.0..19.0)`, the per-seed floor
        // is the same 6.0, the statistic is the same mean-of-the-two-middle
        // median — written generically only so that it stays that statistic at
        // any n. Every literal in this test is the one that was here before.
        //
        // AND THIS IS NOT A FALSE-RED REPAIR, WHICH IS THE INTERESTING PART.
        // Measured on THIS tree by `sample_size_audit::panel_variance` over a
        // hundred seeds, the 30-year multiple is mean 14.22x, variance 1.935,
        // sd 1.391, min 8.65x, median 14.45x (9.31%/yr against the real
        // 9.28%/yr), max 16.56x, with 57 of 100 seeds at or above reality's
        // 14.33x. The band therefore sits about six sampling-sd from the median
        // even at ten seeds, and the bootstrapped false-red probability at n=10
        // is already 0.0000 — inside rule 7's 1% ceiling without touching
        // anything. The per-seed floor of 6.0 is likewise breached by no seed
        // in a hundred, which matters because that arm gets STRICTER as the
        // sample grows.
        //
        // WHAT TEN SEEDS FAILED AT WAS POWER, WHICH IS THE OTHER HALF OF RULE 7
        // AND THE HALF THIS TEST IS THE EVIDENCE FOR. Before the capital-channel
        // repair of the same day, China's 30-year multiple had fallen 14.290x ->
        // 11.072x — 22.5% of level, 0.93 pt/yr — with 45.8% of a 400-seed sample
        // under the 11.0 floor; and this test was GREEN throughout, because
        // seeds 0..9 were a +1.3% draw. A bootstrap on a fair sample put the
        // chance of it catching that regression at 37.6%: a coin flip, dressed
        // as a green light. At a hundred seeds the sampling sd of the median
        // falls by a factor of ~3.2, and a median sitting on 11.07 against a
        // floor of 11.0 is caught essentially every time. THE SAMPLE IS SIZED
        // FOR THE REGRESSION IT MUST SEE, not for the red it must not produce.
        //
        // COST, stated because rule 7 makes tests slower and that is a real
        // price: ninety more thirty-year runs, about 70 seconds of release-mode
        // wall clock, on a spheres-sim suite that was 220s.
        const N: u64 = 100;
        let mut xs: Vec<f64> = Vec::new();
        for seed in 0..N {
            let mut w = world_1990(GameRules { seed, ..GameRules::default() });
            let start = w.nation(NationId::China).gdp;
            run_months(&mut w, 360); // 30 years
            let x = w.nation(NationId::China).gdp / start;
            assert!(
                x > 6.0,
                "no miracle on seed {}: China grew only {:.2}x in 30y",
                seed, x
            );
            xs.push(x);
        }
        xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        // The mean of the two middle order statistics — the same convention the
        // ten-seed version used as `(xs[4] + xs[5]) / 2.0`, written so that it
        // survives a change of sample size.
        let median = (xs[xs.len() / 2 - 1] + xs[xs.len() / 2]) / 2.0;
        assert!(
            (11.0..19.0).contains(&median),
            "China's median 30-year growth across {} seeds is {:.2}x \
             ({:.2}%/yr), outside the 11.0x-19.0x band anchored on the real \
             14.33x. Seeds: {:?}",
            N,
            median,
            (crate::exact::powf(median, 1.0 / 30.0) - 1.0) * 100.0,
            xs.iter().map(|v| (v * 100.0).round() / 100.0).collect::<Vec<_>>()
        );
    }

    #[test]
    fn a_government_cannot_spend_standing_it_has_not_got() {
        // Political capital is a stock, and the point of a stock is that it runs
        // out. A government may tear up its tax policy once; doing it again the
        // same month is a different kind of act and the model should say so.
        let mut w = world_1990(GameRules::default());
        let before = w.nation(NationId::USA).political_capital;
        assert!(before > 20.0, "a stable 1990 USA should open with standing, not {:.0}", before);

        let hike = Command::SetTaxRate { nation: NationId::USA, rate: 0.45 };
        apply_command(&mut w, &hike).expect("the first hike is affordable");
        let after = w.nation(NationId::USA).political_capital;
        assert!(after < before, "the hike cost nothing: {:.1} -> {:.1}", before, after);

        // Drain the rest, then confirm the next ask is refused rather than
        // silently applied.
        w.nation_mut(NationId::USA).political_capital = 1.0;
        let rate_before = w.nation(NationId::USA).tax_rate;
        let err = apply_command(
            &mut w,
            &Command::SetTaxRate { nation: NationId::USA, rate: 0.60 },
        );
        assert!(err.is_err(), "a bankrupt government got its tax rise anyway");
        assert_eq!(
            w.nation(NationId::USA).tax_rate, rate_before,
            "a refused command still moved the world"
        );
    }

    #[test]
    fn a_war_costs_a_government_at_home() {
        // The other half of the currency: it is earned and lost by what the
        // government's record is, not only spent by what it does.
        //
        // Both worlds put the player in Washington, and that is load-bearing
        // rather than decoration. `politics.rs` skips AI statecraft for any
        // nation with `war_exhaustion > 0.3`, so an exhausted USA quietly stops
        // pledging aid and stops SPENDING its standing. Measured without this
        // line, the strained world ended on 43 against the peaceful world's 40
        // — war appeared to pay, purely because the peaceful USA was busy
        // buying clients with the capital the exhausted one was not spending.
        // The `- n.war_exhaustion * 45.0` term was working the whole time; the
        // AI's spending was larger than it and pointed the other way.
        //
        // Making the USA the player suppresses that same AI route in BOTH
        // worlds — symmetrically, and through a condition already in the model
        // rather than a back door — so what is left is the thing the test
        // names. The gap is 63.6 at peace against 52.4 at war.
        //
        // This is the confound that surfaced when patron precedence was
        // restored on the runtime-ids branch: that fix changed how much the
        // peacetime USA spends, which is what pushed an already-masked
        // assertion over the line. The threshold below is untouched.
        let mut w = world_1990(GameRules::default());
        w.rules.ai_aggression = 0.0;
        w.player = Some(NationId::USA);
        run_months(&mut w, 24);
        let peacetime = w.nation(NationId::USA).political_capital;

        let mut at_war = world_1990(GameRules::default());
        at_war.rules.ai_aggression = 0.0;
        at_war.player = Some(NationId::USA);
        at_war.nation_mut(NationId::USA).war_exhaustion = 0.5;
        run_months(&mut at_war, 24);
        let strained = at_war.nation(NationId::USA).political_capital;

        assert!(
            strained < peacetime - 5.0,
            "war cost the government nothing at home: {:.0} at war against {:.0} at peace",
            strained, peacetime
        );
    }

    #[test]
    fn an_endowment_cannot_pay_the_diffusion_floor() {
        // NEGATIVE CONTROL for `a_poor_nation_still_picks_up_what_everyone_has`,
        // which was re-pointed from an absolute count to post-1990 ACQUISITIONS.
        // Iron rule 5 says a re-pointed test must be checked red against the
        // behaviour it now guards, and the subtraction that rewrite added is a
        // NO-OP TODAY — `tech_1990_for` returns an empty grant for every nation
        // because no file carries a `tech_1990` block yet. Without this control
        // the rewrite would be untested code sitting in a green suite, which is
        // the exact failure mode it was written to prevent.
        //
        // The failure mode, exactly: a nation handed a stock in 1990 satisfies a
        // bare floor of five on the grant alone, having acquired nothing at all.
        // The old predicate cannot see that. The new one must.
        //
        // The grant here is injected through the same `grant_1990` door the
        // loader uses, but WITHOUT the loader's rebasing pass, which is fine for
        // this test and would not be for a growth one — nothing below reads TFP.
        for seed in [1990u64, 7, 42] {
            let rules = GameRules { seed, ..GameRules::default() };
            let mut w = world_1990(rules);
            let stock: Vec<u16> = (0..8u16).collect();
            w.nation_mut(NationId::EquatorialGuinea).tech.grant_1990(&stock);
            let granted = stock.len();
            // Two years, not thirty. The horizon is the control: it is long
            // enough to be a real run of the real systems and short enough that
            // the poorest economy on the board has not yet finished anything.
            run_months(&mut w, 24);
            let held = w.nation(NationId::EquatorialGuinea).tech.count();
            let acquired = held.saturating_sub(granted);

            // The OLD predicate — a bare `count() >= 5` — is satisfied here, and
            // satisfied entirely by the endowment.
            assert!(
                held >= 5,
                "seed {}: the control is mis-set, the grant did not land ({} held)",
                seed, held
            );
            // And nothing whatever has been acquired, so the NEW predicate fails
            // as it is supposed to. If this assertion ever goes red, the model
            // has changed such that a granted nation researches inside two years
            // and this control needs a shorter horizon — not a wider tolerance.
            assert_eq!(
                acquired, 0,
                "seed {}: control expected no acquisitions in 24 months, got {}                  (held {}, granted {})",
                seed, acquired, held, granted
            );
        }
        // MEASURED IN PASSING, and worth knowing before the transcription lands:
        // over the full 360 months this endowment does not SUBSTITUTE for
        // acquisition, it ACCELERATES it. Equatorial Guinea finishes seed 42 on
        // exactly 5 technologies ungranted — the floor, to the unit — and on 17
        // when handed these same 8, i.e. 9 acquired rather than 5. So the
        // re-pointed test is precautionary rather than currently load-bearing at
        // thirty years, and the floor it asserts gets easier under a grant, not
        // harder. That is an argument for keeping the acquisition form, not
        // against it: the absolute count would have stopped measuring anything.
    }

    #[test]
    fn a_poor_nation_still_picks_up_what_everyone_has() {
        // The frontier is supposed to be out of a poor nation's reach. The
        // ordinary is not. For a long time both were: the cost floor stood for
        // having to build the thing and took no account of whether the thing was
        // a bespoke fab or a shipping container, so it bound long before the
        // copying discount could bite and the smallest economies were shut out
        // of the whole tree. Vietnam finished a thirty-year run knowing nothing
        // whatsoever, and no test in the suite objected.
        for seed in [1990u64, 7, 42] {
            let rules = GameRules { seed, ..GameRules::default() };
            let mut w = world_1990(rules);
            run_months(&mut w, 360);
            let frontier = w.nations.iter().filter(|n| n.alive).map(|n| n.tech.count()).max().unwrap();
            assert!(frontier > 60, "seed {}: nobody got anywhere: frontier {}", seed, frontier);
            for n in w.nations.iter().filter(|n| n.alive) {
                // RE-POINTED for the 1990 endowment, and the reason is iron rule
                // 5 rather than arithmetic. This test measures ACQUISITION — what
                // a poor nation manages to pick up over thirty years — and a
                // nation handed technology at the start satisfies a bare floor of
                // five without acquiring anything at all. That is the failure
                // mode where a test stops working while staying green, which is
                // worse than one that goes red. So the endowment comes off the
                // count first.
                //
                // Zero for every nation today, because no file carries a
                // `tech_1990` block yet; the subtraction is a no-op and this test
                // is unchanged in what it currently asserts.
                //
                // KNOWN GAP, stated rather than hidden: successors have no data
                // file of their own, so `tech_1990_for` returns nothing for them
                // while `TechState::inherit` hands them the parent's whole
                // granted set. When Tier A authors the Soviet Union, the fifteen
                // republics will each satisfy this floor on inherited technology
                // and this test will quietly stop measuring them. Closing that
                // needs the inherited grant recorded on the successor, which is
                // work for the change that authors the data.
                let granted = crate::data::tech_1990_for(n.id).granted.len();
                let acquired = n.tech.count().saturating_sub(granted);
                assert!(
                    acquired >= 5,
                    "seed {}: {:?} acquired {} technologies in thirty years (holds {}, \
                     granted {}) while the frontier holds {}",
                    seed, n.id, acquired, n.tech.count(), granted, frontier
                );
            }
        }
    }

    #[test]
    fn convergence_outruns_the_frontier() {
        // The structural claim of the whole growth model: a nation behind the
        // technological frontier closes on it, because copying is cheaper than
        // inventing. Convergence now arrives entirely through the tree — the
        // flat bonus for being poor is gone — so if adoption is ever mistuned
        // to nothing, this is what says so. It is a coarse guard and it is
        // meant to be: it holds the sign of the effect, not its size.
        for seed in [1990u64, 7, 42, 2024] {
            let rules = GameRules { seed, ..GameRules::default() };
            let mut w = world_1990(rules);
            let before: Vec<f64> = [NationId::China, NationId::India, NationId::Japan, NationId::Italy]
                .iter()
                .map(|id| w.nation(*id).gdp)
                .collect();
            run_months(&mut w, 360);
            let after: Vec<f64> = [NationId::China, NationId::India, NationId::Japan, NationId::Italy]
                .iter()
                .map(|id| w.nation(*id).gdp)
                .collect();
            let (china, india) = (after[0] / before[0], after[1] / before[1]);
            let (japan, italy) = (after[2] / before[2], after[3] / before[3]);
            for (name, follower) in [("China", china), ("India", india)] {
                for (fname, frontier) in [("Japan", japan), ("Italy", italy)] {
                    assert!(
                        follower > frontier * 1.5,
                        "seed {}: {} grew {:.2}x against {}'s {:.2}x — the frontier is not being caught",
                        seed, name, follower, fname, frontier
                    );
                }
            }
        }
    }

    #[test]
    fn mature_economies_do_not_run_hot() {
        // The exact quantity the tech tree broke when it landed. Its
        // productivity was added on top of a 1990 trend that already priced the
        // same technology in, and every developed economy gained about a point
        // of annual growth for it: measured on this run the United States went
        // to 3.0%, Germany 3.2%, France 2.9% and Italy 2.7%, all of them pinned
        // against the same cap. Nothing failed, because the tolerances then in
        // the suite were wide enough to admit almost anything.
        //
        // Japan is deliberately not here. It carries the highest transcribed
        // 1990 trend of any nation (0.018, correct for 1990) and the model has
        // no mechanism that ever takes it away, so Japan settles near 2.8% and
        // outgrows the United States for the whole run. That is a real gap —
        // the lost decade is modelled as a bubble hangover rather than the
        // permanent break it was — and it wants a demographic or balance-sheet
        // mechanism, not a wider tolerance here.
        for seed in [1990u64, 7, 42] {
            let rules = GameRules { seed, ..GameRules::default() };
            let mut w = world_1990(rules);
            run_months(&mut w, 360);
            for id in [NationId::USA, NationId::Germany, NationId::France, NationId::Italy] {
                let g = w.nation(id).growth_last;
                assert!(
                    (0.008..0.026).contains(&g),
                    "seed {}: {:?} is growing {:.1}% thirty years in — not a mature economy",
                    seed, id, g * 100.0
                );
            }
        }
    }

    #[test]
    fn nuclear_taboo_holds() {
        let mut w = world_1990(GameRules::default());
        let r = war::declare_war(&mut w, NationId::USA, NationId::USSR);
        assert!(r.is_err(), "nuclear powers went to direct war");
    }

    #[test]
    fn the_nuclear_flag_tracks_a_deterrent_other_capitals_could_see() {
        // THE ROSTER'S NUCLEAR CONVENTION, PINNED SO IT CANNOT DRIFT INTO TASTE.
        // `nuclear` is not a possession flag and it is not a declaration flag.
        // It is an OBSERVABILITY flag: it marks a deterrent other governments
        // could see and plan around, because deterring is the only thing it does
        // in the model — `dyads::war_appetite` returns zero outright against a
        // nuclear power and `war.rs` cedes it no ground when it loses.
        //
        // The two files that fix the reading sit at opposite ends of it, and
        // both are entered on this one rule rather than on separate judgements:
        //
        //   Israel  TRUE  on an arsenal it "has never confirmed and never will",
        //                 because every capital in the region planned around
        //                 Dimona in 1990. So declaration cannot be the test.
        //   S.Africa FALSE on six assembled gun-type devices and a seventh
        //                 part-built, because the programme was covert until
        //                 24 March 1993 and dismantlement was ordered on
        //                 26 February 1990. So possession cannot be the test.
        //
        // A review called South Africa's entry a hard citable error. It is not:
        // it is this rule applied to the hardest case on the board, the facts
        // and dates are transcribed in southafrica.json, and flipping it was
        // measured — it moves both goldens and knocks two seed-pinned scenario
        // tests off their seeds. If you are here because you want to change the
        // convention to possession, it has to change for all four of South
        // Africa, Israel, Pakistan and India together, and the model needs a
        // disarmament path first (see the test below).
        let w = world_1990(GameRules::default());
        let mut armed: Vec<&str> = w
            .nations
            .iter()
            .filter(|n| n.alive && n.nuclear)
            .map(|n| n.id.name())
            .collect();
        armed.sort_unstable();
        assert_eq!(
            armed,
            vec!["China", "France", "Israel", "Soviet Union", "United Kingdom", "United States"],
            "the 1990 deterrent set changed; if this is deliberate, the convention \
             in southafrica.json and pakistan.json has to move with it"
        );
    }

    #[test]
    fn nothing_in_the_model_ever_gives_a_deterrent_up() {
        // A DOCTRINE TRIPWIRE, not a wish. The nuclear flag is monotone: once
        // set, nothing anywhere in the sim sets it back. That limit is what
        // decides South Africa's entry — a possession reading would want the
        // flag true for eighteen months of a thirty-year game and false for the
        // remaining twenty-eight and a half, and the model has no way to say so.
        //
        // So this test exists to fail LOUDLY on the day someone implements
        // disarmament, because that is the day southafrica.json's call is worth
        // reopening. It is not asserting that disarmament would be wrong.
        //
        // Successors are excluded deliberately: Ukraine, Belarus and Kazakhstan
        // are BORN non-nuclear in `dissolve_ussr` rather than disarmed by it,
        // which is the model declining to represent the Lisbon Protocol, not a
        // transition. The test tracks nations that were already alive and armed.
        let mut w = world_1990(GameRules::default());
        let mut armed: Vec<NationId> = w
            .nations
            .iter()
            .filter(|n| n.alive && n.nuclear)
            .map(|n| n.id)
            .collect();
        armed.sort_unstable_by_key(|id| id.name());
        for _ in 0..360 {
            run_months(&mut w, 1);
            for id in &armed {
                if let Some(n) = w.nation_opt(*id) {
                    assert!(
                        !n.alive || n.nuclear,
                        "{:?} is alive and has given up its deterrent in {}/{}. If you have \
                         added a disarmament path, that is a real improvement — now go and \
                         reopen the nuclear convention in southafrica.json, which is entered \
                         false precisely because this could not happen.",
                        id, w.year, w.month
                    );
                }
            }
        }
    }

    #[test]
    fn yugoslavia_comes_apart_in_the_nineties() {
        let mut broke = 0;
        for seed in 0..10u64 {
            let rules = GameRules { seed, ..GameRules::default() };
            let mut w = world_1990(rules);
            run_months(&mut w, 120); // to 2000
            if w.has_flag("yugoslavia_dissolved") {
                broke += 1;
            }
        }
        assert!(broke >= 8, "Yugoslavia held together too often: {}/10", broke);
    }

    #[test]
    fn slovenia_escapes_the_wars_that_consume_bosnia() {
        // The asymmetry of the breakup is the whole point, and it must come from
        // inherited ethnic strain rather than a script: Slovenia is homogeneous
        // and gets out; Bosnia has no majority at all and is fought over.
        let (mut slovenia_wars, mut bosnia_wars) = (0, 0);
        for seed in 0..10u64 {
            let rules = GameRules { seed, ..GameRules::default() };
            let mut w = world_1990(rules);
            for _ in 0..120 {
                let headlines = tick_month(&mut w, &[]);
                for h in &headlines {
                    if h.starts_with("WAR:") {
                        if h.contains("Slovenia") {
                            slovenia_wars += 1;
                        }
                        if h.contains("Bosnia") {
                            bosnia_wars += 1;
                        }
                    }
                }
            }
        }
        assert!(
            bosnia_wars > slovenia_wars * 3,
            "breakup was not asymmetric: Bosnia {} wars vs Slovenia {}",
            bosnia_wars, slovenia_wars
        );
    }

    #[test]
    fn some_wars_end_at_the_table() {
        // Not every war runs to a capital or to mutual collapse. Across seeds,
        // negotiated settlements should be a real way for wars to end.
        let mut settled = 0;
        for seed in 0..12u64 {
            let rules = GameRules { seed, ..GameRules::default() };
            let mut w = world_1990(rules);
            let mut saw = false;
            for _ in 0..360 {
                let headlines = tick_month(&mut w, &[]);
                if headlines
                    .iter()
                    .any(|h| h.contains("sues for peace") || h.contains("agree peace terms"))
                {
                    saw = true;
                }
            }
            if saw {
                settled += 1;
            }
        }
        assert!(settled >= 3, "negotiated peace never happens: {}/12 runs", settled);
    }

    #[test]
    fn embargo_starves_the_aggressor_and_outlasts_the_war() {
        // An embargo must bite through exports, not just as a vague GDP drag: the
        // aggressor loses the revenue from barrels it cannot ship, and the
        // coalition's sanctions do not evaporate the moment the shooting stops.
        let mut base = world_1990(GameRules::default());
        let mut embargoed = world_1990(GameRules::default());
        base.rules.ai_aggression = 0.0;
        embargoed.rules.ai_aggression = 0.0;
        war::declare_war(&mut embargoed, NationId::Iraq, NationId::Kuwait).unwrap();
        run_months(&mut base, 60);
        run_months(&mut embargoed, 60);

        assert!(
            embargoed.sanctioned_by_count(NationId::Iraq) > 0,
            "embargo evaporated with the war"
        );
        let bi = base.nation(NationId::Iraq).gdp;
        let ei = embargoed.nation(NationId::Iraq).gdp;
        assert!(ei < bi * 0.8, "embargoed Iraq barely suffered: {:.0} vs {:.0}", ei, bi);
    }

    #[test]
    fn an_embargo_erodes_before_it_ends() {
        // A coalition does not lift together. The minor partners, whose
        // grievance was never deep, are back at the table within a decade; the
        // principal antagonist holds for a generation, and keeps holding partly
        // because its own covert action against the target renews the injury
        // that the relief rule reads. Iraq's real embargo ran thirteen years,
        // Cuba's is past sixty; both shapes are in range. What must never
        // happen is an embargo with no way out at all.
        let mut w = world_1990(GameRules::default());
        w.rules.ai_aggression = 0.0;
        war::declare_war(&mut w, NationId::Iraq, NationId::Kuwait).unwrap();

        run_months(&mut w, 12 * 5);
        let early = w.sanctioned_by_count(NationId::Iraq);
        assert!(early >= 3, "no coalition formed: {} sanctioners in 1995", early);

        run_months(&mut w, 12 * 10);
        let late = w.sanctioned_by_count(NationId::Iraq);
        assert!(late < early, "the coalition never eroded: {} in 1995, {} in 2005", early, late);

        run_months(&mut w, 12 * 35);
        assert_eq!(
            w.sanctioned_by_count(NationId::Iraq),
            0,
            "the embargo outlived a fifty-year run"
        );
    }
    // ---- Statecraft: pacts, patronage, subversion, trade --------------------

    fn seeded(seed: u64) -> WorldState {
        let rules = GameRules { seed, ..GameRules::default() };
        world_1990(rules)
    }

    /// Keep asking until they say yes. An arrangement a test is about to
    /// interrogate has to exist first, and consent is a die roll.
    fn force_pact(w: &mut WorldState, a: NationId, b: NationId) {
        for _ in 0..300 {
            if w.allied(a, b) {
                return;
            }
            apply_command(w, &Command::ProposeAlliance { from: a, to: b }).unwrap();
        }
        panic!("{:?} and {:?} never signed", a, b);
    }

    fn force_trade(w: &mut WorldState, a: NationId, b: NationId) {
        for _ in 0..300 {
            if w.trade_depth(a, b) > 0.0 {
                return;
            }
            apply_command(w, &Command::ProposeTrade { from: a, to: b }).unwrap();
        }
        panic!("{:?} and {:?} never came to terms", a, b);
    }

    #[test]
    fn superpowers_compete_for_the_same_clients() {
        // The texture of the Cold War is two hostile patrons bankrolling the same
        // government at the same time. It should not need a script: a client the
        // other side is already buying is simply worth more.
        let mut contested = 0;
        for seed in 0..12u64 {
            let mut w = seeded(seed);
            let mut saw = false;
            for _ in 0..240 {
                tick_month(&mut w, &[]);
                saw |= w.nations.iter().filter(|n| n.alive).any(|n| {
                    let backers = w.patrons_of(n.id);
                    backers
                        .iter()
                        .any(|x| backers.iter().any(|y| w.relation(*x, *y) < -20.0))
                });
            }
            if saw {
                contested += 1;
            }
        }
        assert!(
            contested >= 7,
            "the powers never bid against each other: {}/12 runs",
            contested
        );
    }

    #[test]
    fn a_pact_drags_a_great_power_into_a_war_it_did_not_start() {
        // Not every run — a guarantee that is always called is not a guarantee,
        // it is a border. But across seeds, somebody's client gets invaded and
        // its protector has to show up.
        let mut dragged = 0;
        for seed in 0..12u64 {
            let mut w = seeded(seed);
            let mut saw = false;
            for _ in 0..360 {
                for h in tick_month(&mut w, &[]) {
                    if h.contains("honours its defence pact")
                        && patrons().iter().any(|p| h.starts_with(p.name()))
                    {
                        saw = true;
                    }
                }
            }
            if saw {
                dragged += 1;
            }
        }
        assert!(
            (3..12).contains(&dragged),
            "pacts pulled a great power into someone else's war in {}/12 runs",
            dragged
        );
    }

    #[test]
    fn guarantees_are_usually_but_not_always_honoured() {
        // The whole point of making commitment explicit is that it can fail.
        // Across seeds the guarantor mostly turns up, and sometimes does not.
        let (mut honoured, mut abandoned) = (0, 0);
        for seed in 0..40u64 {
            let mut w = seeded(seed);
            force_pact(&mut w, NationId::USA, NationId::Kuwait);
            w.headlines.clear();
            war::declare_war(&mut w, NationId::Iraq, NationId::Kuwait).unwrap();
            if w.headlines.iter().any(|h| h.contains("United States honours its defence pact")) {
                honoured += 1;
            }
            if w.headlines.iter().any(|h| h.contains("United States abandons its pact")) {
                abandoned += 1;
            }
        }
        assert_eq!(honoured + abandoned, 40, "a guarantee went unanswered");
        assert!(
            honoured > abandoned * 2,
            "pacts are worthless: {} kept, {} broken",
            honoured,
            abandoned
        );
        assert!(abandoned >= 1, "no pact was ever broken in 40 invasions");
    }

    #[test]
    fn abandoning_an_ally_is_felt_by_every_other_ally() {
        let mut w = seeded(11);
        w.rules.ai_aggression = 0.0;
        war::declare_war(&mut w, NationId::Iraq, NationId::Kuwait).unwrap();
        force_pact(&mut w, NationId::USA, NationId::SouthKorea);
        force_pact(&mut w, NationId::USA, NationId::Kuwait);
        let rep = w.reputation(NationId::USA);
        let seoul = w.relation(NationId::USA, NationId::SouthKorea);

        apply_command(&mut w, &Command::BreakAlliance { from: NationId::USA, to: NationId::Kuwait })
            .unwrap();

        assert!(!w.allied(NationId::USA, NationId::Kuwait));
        assert!(
            w.reputation(NationId::USA) <= rep - 25.0,
            "walking out on a war cost nothing: {} -> {}",
            rep,
            w.reputation(NationId::USA)
        );
        assert!(
            w.relation(NationId::USA, NationId::SouthKorea) < seoul - 5.0,
            "Seoul did not notice what Washington's word is worth"
        );
    }

    #[test]
    fn aid_props_up_a_client_regime_and_the_patron_pays_for_it() {
        let (mut base, mut aided) = (seeded(4), seeded(4));
        for w in [&mut base, &mut aided] {
            w.rules.ai_aggression = 0.0;
            w.player = Some(NationId::USA); // freeze Washington's own AI in both
        }
        apply_command(
            &mut aided,
            &Command::PledgeAid {
                patron: NationId::USA,
                client: NationId::Poland,
                kind: AidKind::Economic,
                share_gdp: 0.004,
            },
        )
        .unwrap();
        run_months(&mut base, 120);
        run_months(&mut aided, 120);

        let (bp, ap) = (base.nation(NationId::Poland), aided.nation(NationId::Poland));
        assert!(
            ap.stability > bp.stability + 5.0,
            "aid did not hold the regime up: {:.1} vs {:.1}",
            ap.stability,
            bp.stability
        );
        assert!(ap.gdp > bp.gdp * 1.10, "aid bought no growth: {:.0} vs {:.0}", ap.gdp, bp.gdp);
        assert!(
            aided.nation(NationId::USA).debt_gdp > base.nation(NationId::USA).debt_gdp,
            "the patron got its sphere for free"
        );
    }
    /// The ratio of an armed client's forces to an otherwise identical one,
    /// across ten seeds. `share` of 0.0 is the control: a pledge that transfers
    /// nothing, which must produce a ratio of one.
    fn arms_ratio(share: f64) -> Vec<f64> {
        let mut out = vec![];
        for seed in 0..10u64 {
            let (mut base, mut armed) = (seeded(seed), seeded(seed));
            for w in [&mut base, &mut armed] {
                w.rules.ai_aggression = 0.0;
                w.player = Some(NationId::USA);
            }
            let _ = apply_command(
                &mut armed,
                &Command::PledgeAid {
                    patron: NationId::USA,
                    client: NationId::Kuwait,
                    kind: AidKind::Arms,
                    share_gdp: share,
                },
            );
            run_months(&mut base, 96);
            run_months(&mut armed, 96);
            let b = base.nation(NationId::Kuwait).mil_strength.max(0.001);
            out.push(armed.nation(NationId::Kuwait).mil_strength / b);
        }
        out.sort_by(|x, y| x.partial_cmp(y).unwrap());
        out
    }

    #[test]
    fn arms_transfers_build_a_client_army() {
        // This was one seed and an absolute bar, and it broke when the roster
        // tripled — not because the aid stopped working, but because it is a
        // ratio between a treated and an UNTREATED Kuwait, and a filling region
        // arms the control too. Measured at the break: the control arm rose
        // 6.50 -> 7.70 while the treated rose 10.60 -> 10.92. The aid still
        // bought an army; it bought a smaller multiple of a bigger baseline.
        //
        // So it is a cross-seed median now, the same shape china_growth_miracle
        // uses, and it is a comparison against the CONTROL rather than against a
        // remembered number — a pledge of nothing must produce a ratio of one,
        // whatever else the world is doing to Kuwait's army.
        let armed = arms_ratio(0.003);
        let median = armed[armed.len() / 2];
        assert!(
            median > 1.25,
            "arms bought no army: median ratio {:.3} across ten seeds, {:?}",
            median,
            armed.iter().map(|x| (x * 100.0).round() / 100.0).collect::<Vec<_>>()
        );
        // ...and the guard that keeps this from being a test that cannot fail:
        // with nothing transferred, the two worlds must be the same world.
        let control = arms_ratio(0.0);
        let control_median = control[control.len() / 2];
        assert!(
            (control_median - 1.0).abs() < 0.02,
            "an empty pledge moved the client's army: {:.3}",
            control_median
        );
    }

    /// Twenty years of a USA-Poland agreement, one entry per seed: what Poland's
    /// output came to as a multiple of the same Poland without the agreement,
    /// and what tearing it up then costs each side.
    ///
    /// `sign` of false is the control arm: no agreement is ever signed, so the
    /// "open" world IS the base world and the lift must be exactly one — and
    /// abrogating an agreement that does not exist must cost Warsaw nothing.
    /// That second half is a real guard and not a formality: `AbrogateTrade`
    /// reaches into both economies, and a version of it that charged the
    /// reputational or dependency penalty without checking that a pact was
    /// there would pass every assertion in the treated arm.
    /// Advance the world with WARSAW OTHERWISE UNOPEN: every trade agreement
    /// Poland holds with anyone but Washington is struck out the month it is
    /// signed. `keep_usa` decides whether the one agreement under test survives.
    ///
    /// This is the construction that makes the reading match the test's name,
    /// and it is not a formality. `statecraft` ticks BEFORE `politics` in
    /// `SYSTEMS`, so an agreement the AI signs in month M is not seen by
    /// `trade_level_gain` until month M+1 — striking it at the end of month M
    /// means it is never paid at all, and Poland's `trade_level_paid` stays
    /// `None` in the arm that holds nothing. The strike is applied to BOTH arms
    /// and to Washington-Warsaw too when `keep_usa` is false, so the control arm
    /// cannot quietly acquire the very agreement it is supposed to lack.
    fn run_warsaw_unopen(w: &mut WorldState, months: usize, keep_usa: bool) {
        for _ in 0..months {
            tick_month(w, &[]);
            w.statecraft.trade.retain(|t| {
                if t.a != NationId::Poland && t.b != NationId::Poland {
                    return true;
                }
                keep_usa && (t.a == NationId::USA || t.b == NationId::USA)
            });
        }
    }

    fn trade_lift(sign: bool) -> (Vec<f64>, Vec<f64>, Vec<f64>, usize) {
        let (mut lift, mut warsaw_v, mut washington_v) = (vec![], vec![], vec![]);
        let mut tore_up = 0usize;
        for seed in 0..10u64 {
            let (mut base, mut open) = (seeded(seed), seeded(seed));
            for w in [&mut base, &mut open] {
                w.rules.ai_aggression = 0.0;
                w.player = Some(NationId::USA);
            }
            if sign {
                force_trade(&mut open, NationId::USA, NationId::Poland);
            }
            run_warsaw_unopen(&mut base, 240, false);
            run_warsaw_unopen(&mut open, 240, sign);
            lift.push(open.nation(NationId::Poland).gdp / base.nation(NationId::Poland).gdp);

            let (p0, u0) =
                (open.nation(NationId::Poland).gdp, open.nation(NationId::USA).gdp);
            // Not unwrapped: in the control arm there is nothing to tear up and
            // the refusal is the point. Counting the successes is how the test
            // tells "the agreement cost nothing to lose" apart from "there was
            // no agreement", which are very different findings.
            if apply_command(
                &mut open,
                &Command::AbrogateTrade { from: NationId::USA, to: NationId::Poland },
            )
            .is_ok()
            {
                tore_up += 1;
            }
            warsaw_v.push(1.0 - open.nation(NationId::Poland).gdp / p0);
            washington_v.push(1.0 - open.nation(NationId::USA).gdp / u0);
        }
        for v in [&mut lift, &mut warsaw_v, &mut washington_v] {
            v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        }
        (lift, warsaw_v, washington_v, tore_up)
    }

    /// Converted from one seed to a ten-seed median 2026-08-31, PLAN step 1.
    ///
    /// IT WAS A COIN FLIP WEARING A THRESHOLD, and the measurement that says so
    /// is `instrument_spread`: the lift asserted above 1.20 while the ten-seed
    /// spread ran 1.018 .. 1.436. Four seeds in ten sat under the bar. It passed
    /// because somebody had picked seed 2, and any change anywhere that
    /// reshuffled the RNG stream could have flipped it either way without
    /// anything about trade having moved.
    ///
    /// THE BAR IS UNCHANGED AT 1.20 AND THAT IS DELIBERATE. Converting a
    /// one-seed reading to a median of ten is a strengthening only if the
    /// threshold stays put; re-deriving it against what the model now prints
    /// would be the laundering iron rule 5 forbids. So 1.20 is inherited, not
    /// re-fitted, and the median of ten must clear what one lucky seed used to.
    ///
    /// RE-CONSTRUCTED 2026-08-31, AND THE BAR IS STILL UNCHANGED AT 1.20. The
    /// median had fallen to 1.107 and the reason was not trade. Poland holds
    /// agreements of its own: on the ten-seed control arm it finished with
    /// Czechoslovakia and Hungary and a `trade_level_paid` of 0.079 .. 0.137
    /// against the treated arm's 0.246. So the ratio was reading the MARGINAL
    /// worth of one more agreement to a country already open, while the name on
    /// the test claims the STANDALONE worth of the first one. Reach is a share
    /// of a trading universe and is bounded by one, so a portfolio that already
    /// covers most of it leaves little for the next entrant to add — 1.246/1.11
    /// is 1.12, which is the number that was printing.
    ///
    /// `run_warsaw_unopen` fixes the construction rather than the threshold:
    /// Warsaw is kept otherwise unopen in BOTH arms, so the ratio is one
    /// agreement against none, which is what the name says. Confirmed by the
    /// instrument itself — the control arm's `trade_level_paid` is now `None`,
    /// i.e. Poland was never paid a penny of level gain in the arm that holds
    /// nothing, so no part of the lift is leakage.
    ///
    /// MEASURED AFTER: median 1.222, ten-seed spread 1.186 .. 1.231, eight of
    /// ten seeds over the bar. That is a real margin rather than a comfortable
    /// one, and it is stated so the next reader does not mistake it for slack.
    /// It is also a far tighter distribution than the 1.018 .. 1.436 recorded
    /// below, which is the finding underneath all of this: the spread that made
    /// the old reading a coin flip came from Poland's OTHER agreements moving
    /// around under it, not from what an agreement is worth.
    ///
    /// AND THE 1.20 IS UNSOURCED — recorded here rather than quietly repaired.
    /// The asymmetry below has a real anchor stated in the test ("an order of
    /// magnitude", the 10x this repo's CLAUDE.md records reading 8.5x before the
    /// growth model was fixed). The *magnitude* of the lift has none: no note in
    /// this repo says what twenty years of integration was worth to a small
    /// open economy in the period, and `TRADE_LEVEL_GAIN`'s own comment argues
    /// its quarter from market size rather than from a measured episode. A band
    /// invented here to look rigorous would be a fabricated citation, so the
    /// inherited threshold stands and the debt is written down instead. This is
    /// the one instrument of the four still resting on an unsourced number.
    #[test]
    fn a_trade_agreement_lifts_the_smaller_partner_and_then_binds_it() {
        let (lift, warsaw_v, washington_v, tore_up) = trade_lift(true);
        assert_eq!(tore_up, 10, "the treated arm failed to tear up an agreement it had signed");
        let median = lift[lift.len() / 2];
        assert!(
            median > 1.20,
            "twenty years of integration bought nothing: median lift {:.3} across ten seeds {:?}",
            median,
            lift.iter().map(|x| (x * 1000.0).round() / 1000.0).collect::<Vec<_>>()
        );

        // ...and the growth is the leash. Tearing the agreement up costs the
        // small partner an order of magnitude more than the large one.
        let warsaw = warsaw_v[warsaw_v.len() / 2];
        let washington = washington_v[washington_v.len() / 2];
        assert!(warsaw > 0.02, "the dependent partner shrugged it off: {:.4} {:?}", warsaw, warsaw_v);
        assert!(
            warsaw > washington * 10.0,
            "dependency was symmetric: {:.4} vs {:.4}",
            warsaw,
            washington
        );

        // The control arm: never sign, and the two worlds are one world.
        let (c_lift, c_warsaw, _, c_tore_up) = trade_lift(false);
        let c_median = c_lift[c_lift.len() / 2];
        assert!(
            (c_median - 1.0).abs() < 1e-9,
            "an agreement nobody signed lifted Poland: {:.6} {:?}",
            c_median, c_lift
        );
        // Nothing was signed, so nothing can be torn up — which also establishes
        // that the treated arm measured the agreement THIS TEST made, and was
        // not riding on one the AI had signed on its own.
        assert_eq!(
            c_tore_up, 0,
            "Washington tore up a Warsaw agreement it never signed in {} of ten seeds",
            c_tore_up
        );
        let c_worst = c_warsaw.iter().fold(0.0f64, |a, b| a.max(b.abs()));
        assert!(
            c_worst < 1e-9,
            "tearing up an agreement that was never signed cost Warsaw {:.6}",
            c_worst
        );
    }

    #[test]
    fn covert_action_is_deniable_until_it_is_not() {
        // A service that keeps going back to the same well gets rolled up, and
        // the bill lands on the relationship rather than on the operation.
        let mut w = seeded(3);
        w.rules.ai_aggression = 0.0;
        w.player = Some(NationId::USA);
        let start = w.relation(NationId::USA, NationId::Iran);
        let (mut caught, mut clean) = (0, 0);
        for _ in 0..40 {
            let hl = tick_month(
                &mut w,
                &[Command::CovertAction {
                    sponsor: NationId::USA,
                    target: NationId::Iran,
                    op: CovertOp::FundOpposition,
                }],
            );
            if hl.iter().any(|h| h.contains("exposes United States")) {
                caught += 1;
            } else {
                clean += 1;
            }
        }
        assert!(caught > 0, "forty operations and never once caught");
        assert!(clean > 0, "covert action was never actually covert");
        assert!(
            w.relation(NationId::USA, NationId::Iran) < start - 30.0,
            "getting caught cost nothing: {} -> {}",
            start,
            w.relation(NationId::USA, NationId::Iran)
        );
        assert!(w.covert_heat(NationId::USA, NationId::Iran) > 0.5, "the channel never got hot");
    }

    #[test]
    fn oil_shock_propagates_to_importer_inflation() {
        // The open thread from last session, now as a test: a Gulf war must
        // raise oil prices and push importer (Japan) inflation up vs baseline.
        let mut base = world_1990(GameRules::default());
        let mut shocked = world_1990(GameRules::default());
        // Suppress AI wars in baseline by zeroing aggression
        base.rules.ai_aggression = 0.0;
        shocked.rules.ai_aggression = 0.0;
        war::declare_war(&mut shocked, NationId::Iraq, NationId::Kuwait).unwrap();
        for _ in 0..8 {
            tick_month(&mut base, &[]);
            tick_month(&mut shocked, &[]);
        }
        assert!(shocked.oil_price > base.oil_price + 5.0, "oil didn't spike: {} vs {}", shocked.oil_price, base.oil_price);
        let jb = base.nation(NationId::Japan).inflation;
        let js = shocked.nation(NationId::Japan).inflation;
        assert!(js > jb, "Japan inflation didn't rise with oil: {} vs {}", js, jb);
    }

    #[test]
    fn expanded_roster_holds_the_economic_invariants() {
        // The 50-year invariant sweep again, but over the full roster and across
        // seeds — eight more economies means eight more ways for the arithmetic
        // to blow up, and two of them (Brazil at 295% inflation, Vietnam at a
        // hundred dollars a head) sit at the far edges of the model's range.
        for seed in [0u64, 7, 1990] {
            let rules = GameRules { seed, ..GameRules::default() };
            let mut w = world_1990(rules);
            for id in [
                NationId::Brazil, NationId::Indonesia, NationId::Egypt, NationId::Israel,
                NationId::Turkey, NationId::Nigeria, NationId::Vietnam,
            ] {
                assert!(w.nation(id).alive, "{:?} missing from the 1990 roster", id);
            }
            assert!(w.nation_opt(NationId::Ukraine).is_none(), "Ukraine exists before the union falls");

            for _ in 0..600 {
                tick_month(&mut w, &[]);
                for n in w.nations.iter().filter(|n| n.alive) {
                    assert!(n.gdp.is_finite() && n.gdp > 0.0, "seed {} {:?} gdp broke: {}", seed, n.id, n.gdp);
                    assert!(n.population.is_finite() && n.population > 0.0, "seed {} {:?} population broke", seed, n.id);
                    assert!(n.inflation.is_finite(), "seed {} {:?} inflation NaN", seed, n.id);
                    assert!(n.debt_gdp.is_finite() && n.debt_gdp < 6.0, "seed {} {:?} debt spiral: {}", seed, n.id, n.debt_gdp);
                    assert!((0.0..=100.0).contains(&n.stability), "seed {} {:?} stability {}", seed, n.id, n.stability);
                    assert!(n.mil_strength.is_finite() && n.mil_strength >= 0.0, "seed {} {:?} strength broke", seed, n.id);
                    assert!(n.oil_mbd.is_finite() && n.oil_mbd >= 0.0, "seed {} {:?} oil broke", seed, n.id);
                }
                assert!(w.oil_price.is_finite() && w.oil_price > 0.0);
            }
        }
    }

    #[test]
    fn brazil_grinds_down_its_hyperinflation() {
        // Nothing in the sim knows about the Collor Plan or the Real. Brazil is
        // simply handed January 1990 — prices doubling every six weeks, an
        // overnight rate chasing them — and the ordinary central bank machinery
        // has to fight its way out. The claim being tested is the shape of that
        // fight, not its date: no quick cure, and no permanent hyperinflation.
        let (mut still_burning_at_18m, mut tamed_by_1999) = (0, 0);
        for seed in 0..10u64 {
            let rules = GameRules { seed, ..GameRules::default() };
            let mut w = world_1990(rules);
            for m in 0..120 {
                tick_month(&mut w, &[]);
                if m == 17 && w.nation(NationId::Brazil).inflation > 0.50 {
                    still_burning_at_18m += 1;
                }
            }
            if w.nation(NationId::Brazil).inflation < 0.10 {
                tamed_by_1999 += 1;
            }
        }
        assert!(
            still_burning_at_18m >= 8,
            "hyperinflation vanished overnight in too many runs: still burning in {}/10",
            still_burning_at_18m
        );
        assert!(
            tamed_by_1999 >= 8,
            "Brazil never escaped hyperinflation: tamed in {}/10",
            tamed_by_1999
        );
    }

    #[test]
    fn nigeria_has_a_good_gulf_war() {
        // The roster expansion puts producers outside the Gulf into the model for
        // the first time, and that changes who wins when the Gulf catches fire.
        // Lagos ships 1.8 mbd and buys nothing from Iraq: an embargo that takes
        // Iraqi and Kuwaiti barrels off the market is, for Nigeria, a pay rise.
        let mut base = world_1990(GameRules::default());
        let mut shocked = world_1990(GameRules::default());
        base.rules.ai_aggression = 0.0;
        shocked.rules.ai_aggression = 0.0;
        war::declare_war(&mut shocked, NationId::Iraq, NationId::Kuwait).unwrap();
        for _ in 0..24 {
            tick_month(&mut base, &[]);
            tick_month(&mut shocked, &[]);
        }
        let nb = base.nation(NationId::Nigeria).gdp;
        let ns = shocked.nation(NationId::Nigeria).gdp;
        assert!(
            ns > nb * 1.08,
            "Nigeria got no windfall from the Gulf crisis: {:.1} vs {:.1}",
            ns, nb
        );
        // ...while an importer of the same size pays for it. Turkey buys its oil
        // and lost the Kirkuk pipeline transit fees on top.
        let tb = base.nation(NationId::Turkey).inflation;
        let ts = shocked.nation(NationId::Turkey).inflation;
        assert!(ts > tb, "Turkey shrugged off the oil shock: {:.4} vs {:.4}", ts, tb);
    }

    // ---- The commitment ladder ---------------------------------------------

    /// The measurement QA ran by hand, kept in the tree so the next person does
    /// not have to. Not an assertion — a census. Prints, for a run of seeds:
    /// what rung conflicts are BORN on, how many belligerent-months each rung
    /// holds, and how the conflicts that ended actually ended.
    ///
    ///     cargo test -p spheres-sim --release --lib war_census -- --ignored --nocapture
    #[test]
    #[ignore]
    fn war_census() {
        let years = 35;
        let mut born = [0u32; 10];
        let mut peak = [0u32; 10];
        let mut months_at = [0u64; 10];
        let mut endings: Vec<(&str, u32)> = vec![
            ("annexed", 0), ("capitulates", 0), ("repels", 0),
            ("white peace", 0), ("sues for peace", 0), ("agree peace terms", 0),
        ];
        let (mut opened, mut invaded) = (0u32, 0u32);
        for seed in 0..8u64 {
            let mut w = world_1990(GameRules { seed, ..GameRules::default() });
            let mut seen: std::collections::BTreeMap<u32, (u8, u8)> = Default::default();
            for _ in 0..12 * years {
                for h in tick_month(&mut w, &[]) {
                    for e in endings.iter_mut() {
                        if h.contains(e.0) {
                            e.1 += 1;
                        }
                    }
                    if h.starts_with("WAR:") {
                        invaded += 1;
                    }
                }
                for c in &w.conflicts {
                    let top = c.posture.iter().map(|b| b.rung).max().unwrap_or(1);
                    let e = seen.entry(c.id).or_insert_with(|| {
                        opened += 1;
                        born[top as usize] += 1;
                        (top, top)
                    });
                    e.1 = e.1.max(top);
                    for b in &c.posture {
                        months_at[b.rung as usize] += 1;
                    }
                }
            }
            for (_, (_, hi)) in seen {
                peak[hi as usize] += 1;
            }
        }
        println!("\n{} conflicts opened, {} of them became invasions", opened, invaded);
        println!("rung:      1     2     3     4     5     6     7     8     9");
        let row = |name: &str, v: &[u64]| {
            let mut s = format!("{:<9}", name);
            for slot in v.iter().take(10).skip(1) {
                s += &format!("{:>6}", slot);
            }
            println!("{}", s);
        };
        row("born at", &born.iter().map(|x| *x as u64).collect::<Vec<_>>());
        row("peak at", &peak.iter().map(|x| *x as u64).collect::<Vec<_>>());
        row("bel-mths", &months_at);
        println!("endings:");
        for (name, n) in endings {
            println!("  {:<20} {}", name, n);
        }
        let mut who: std::collections::BTreeMap<String, u64> = Default::default();
        for seed in 0..8u64 {
            let mut w = world_1990(GameRules { seed, ..GameRules::default() });
            for _ in 0..12 * years {
                tick_month(&mut w, &[]);
                for c in &w.conflicts {
                    for b in c.posture.iter().filter(|b| b.rung == 5) {
                        *who.entry(format!(
                            "{} in {} (frozen {})",
                            b.nation.name(),
                            c.theatre.name(),
                            c.frozen_since.is_some()
                        ))
                        .or_default() += 1;
                    }
                }
            }
        }
        {
            let mut w = world_1990(GameRules { seed: 0, ..GameRules::default() });
            let mut said = 0;
            for _ in 0..12 * years {
                tick_month(&mut w, &[]);
                let probes: Vec<(NationId, u32, u8, u8, String, f64)> = w
                    .conflicts
                    .iter()
                    .flat_map(|c| {
                        c.posture
                            .iter()
                            .filter(|b| b.rung == 5 && b.months_at_rung > 24)
                            .map(|b| {
                                (
                                    b.nation,
                                    c.id,
                                    b.rung,
                                    commitment::ambition(&w, c, b),
                                    commitment::rung_blocked(&w, c, b.nation, 6)
                                        .unwrap_or_else(|| "-".into()),
                                    w.nation(b.nation).political_capital,
                                )
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect();
                for (n, cid, r, want, why, pc) in probes {
                    if said < 8 {
                        said += 1;
                        println!(
                            "  stuck: {} c{} at {} wants {} pc {:.0} — blocked: {}",
                            n.name(), cid, r, want, pc, why
                        );
                    }
                }
            }
        }
        let mut v: Vec<_> = who.into_iter().collect();
        v.sort_by_key(|(_, n)| std::cmp::Reverse(*n));
        println!("who is standing on rung 5:");
        for (k, n) in v.iter().take(12) {
            println!("  {:<50} {}", k, n);
        }
    }

    /// Hand a government enough standing to buy what the test is about, so that
    /// the assertion is about the war model rather than about affordability.
    fn bankroll(w: &mut WorldState, id: NationId) {
        w.nation_mut(id).political_capital = 100.0;
    }

    /// Keep asking until the parliament says yes. Consent is a die roll, and a
    /// test that wants to interrogate what happens *after* access has to get it.
    fn force_access(w: &mut WorldState, seeker: NationId, host: NationId, th: theatre::TheatreId) {
        for _ in 0..400 {
            if theatre::has_access(w, seeker, th) {
                return;
            }
            bankroll(w, seeker);
            let _ = apply_command(w, &Command::RequestAccess { seeker, host, theatre: th });
        }
        panic!("{:?} never got into {:?}", seeker, th);
    }

    /// Open a conflict and put both sides where the test wants them, bypassing
    /// nothing: every step goes through `apply_command` exactly as a player's
    /// click would.
    fn staged_conflict(
        w: &mut WorldState,
        opener: NationId,
        target: NationId,
        th: theatre::TheatreId,
        opener_rung: u8,
        target_rung: u8,
    ) -> u32 {
        bankroll(w, opener);
        apply_command(w, &Command::OpenConflict { opener, target, theatre: th }).unwrap();
        let id = w.conflict_between(opener, target).expect("just opened").id;
        for (who, rung) in [(opener, opener_rung), (target, target_rung)] {
            bankroll(w, who);
            // A ceiling is what stops the AI wandering off the posture the test
            // is about, and it is a real player command rather than a back door.
            apply_command(w, &Command::SetCeiling { conflict: id, nation: who, rung }).unwrap();
            bankroll(w, who);
            apply_command(w, &Command::SetCommitment { conflict: id, nation: who, rung }).unwrap();
        }
        id
    }

    /// Hold a staged conflict open, so a test of one of BIBLE §6's three stocks
    /// cannot be pre-empted by one of the other two.
    ///
    /// The three stocks are on *deliberately mismatched time constants* — that
    /// is the design, not an accident of tuning. Force structure takes decades,
    /// munitions weeks, resolve months. The consequence for a measurement is
    /// unavoidable: whichever stock empties first ends the conflict and hides
    /// what the other two were doing. A test that means to measure one of them
    /// therefore has to say which, and hold the others still.
    ///
    /// This holds the political stock still. Resolve is pinned above 0.45,
    /// which is `settlement_ripe`'s threshold for a loser having no fight left,
    /// and exhaustion below 0.75, which is the white-peace threshold. Nothing
    /// else is touched: the rung, the burn, the refill, the kills, the control
    /// track and the dry-magazine rule all run exactly as they do in a game.
    ///
    /// It cannot make a bound easier to pass. A drain that is too slow still
    /// reads too slow and one that is too fast still reads too fast; the only
    /// outcome it removes is "the war stopped before the question was asked".
    fn hold_open(w: &mut WorldState, id: u32) {
        let parties = match w.conflict_mut(id) {
            Some(c) => {
                for b in c.posture.iter_mut() {
                    b.resolve = b.resolve.max(0.60);
                }
                c.participants()
            }
            None => return,
        };
        for n in parties {
            let x = w.nation_mut(n);
            x.war_exhaustion = x.war_exhaustion.min(0.50);
        }
    }

    #[test]
    fn a_player_can_get_into_somebody_elses_war() {
        // QA's third finding, as an assertion: playing the United States there
        // was no verb that made you a party to a conflict, so every command on
        // the ladder answered "not a party to that conflict" and the whole war
        // layer was unreachable from the only seat a player ever sits in.
        //
        // The route has to be the ordinary one — commands through the queue,
        // priced, refusable — and it has to end with the player actually
        // standing on a rung of somebody else's war.
        // Iraq and Iran, because it is the case where nobody arrives on their
        // own: the majors turn up for Kuwait by themselves, through
        // `invasion_begins`, and a test that used that pair would be asserting
        // about the AI's route in rather than the player's. Washington backing
        // Baghdad against Tehran is also simply what happened.
        let mut w = seeded(2);
        w.player = Some(NationId::USA);
        w.rules.ai_aggression = 0.0;
        war::declare_war(&mut w, NationId::Iraq, NationId::Iran).unwrap();
        let id = w.conflict_between(NationId::Iraq, NationId::Iran).unwrap().id;
        assert!(
            !w.conflict(id).unwrap().involves(NationId::USA),
            "the test needs a war Washington is not already in"
        );

        // Before joining, the ladder is closed to you and says so.
        let shut = apply_command(
            &mut w,
            &Command::SetCommitment { conflict: id, nation: NationId::USA, rung: 2 },
        );
        assert!(shut.is_err(), "a bystander was allowed to pick a rung");
        assert!(shut.unwrap_err().contains("Not a party"));

        // Taking a side costs standing, and it costs it once.
        w.nation_mut(NationId::USA).political_capital = 60.0;
        let before = w.nation(NationId::USA).political_capital;
        apply_command(
            &mut w,
            &Command::JoinConflict {
                conflict: id,
                nation: NationId::USA,
                side_a: true,
                objective: Objective::Deny,
            },
        )
        .expect("Washington could not take Baghdad's side");
        let c = w.conflict(id).unwrap();
        assert!(c.side_a.contains(&NationId::USA), "the join did not put it on a side");
        assert_eq!(
            c.posture_of(NationId::USA).unwrap().rung,
            1,
            "joining entered above the bottom of the ladder"
        );
        assert!(
            w.nation(NationId::USA).political_capital < before - 10.0,
            "taking a side in somebody else's war was free"
        );

        // ...and from there the ladder is the ordinary ladder.
        apply_command(
            &mut w,
            &Command::SetCommitment { conflict: id, nation: NationId::USA, rung: 2 },
        )
        .expect("a party to the conflict cannot buy rung 2");
        assert!(
            w.is_sanctioning(NationId::USA, NationId::Iran),
            "rung 2 did not bind the instrument it is made of"
        );
        assert!(
            apply_command(
                &mut w,
                &Command::JoinConflict {
                    conflict: id,
                    nation: NationId::USA,
                    side_a: false,
                    objective: Objective::Deny,
                },
            )
            .is_err(),
            "a nation joined the same war twice, on both sides"
        );
    }

    #[test]
    fn a_quarrel_is_not_an_invasion_and_the_world_knows_the_difference() {
        // The distinction QA's first finding is about, pinned so it cannot
        // quietly go away again. Opening a quarrel is cheap, public and
        // consequence-free abroad; it is the CLIMB to a full conventional
        // campaign that brings the coalition, and it is a separate act, priced
        // separately, seven rungs further up.
        let mut w = seeded(6);
        w.rules.ai_aggression = 0.0;
        w.player = Some(NationId::Iraq); // freeze Baghdad's own AI; the test is the player
        bankroll(&mut w, NationId::Iraq);
        let th = theatre::TheatreId::Gulf;
        apply_command(
            &mut w,
            &Command::OpenConflict { opener: NationId::Iraq, target: NationId::Kuwait, theatre: th },
        )
        .unwrap();
        let id = w.conflict_between(NationId::Iraq, NationId::Kuwait).unwrap().id;
        assert_eq!(w.conflict(id).unwrap().posture_of(NationId::Iraq).unwrap().rung, 1);
        assert_eq!(
            w.sanctioned_by_count(NationId::Iraq),
            0,
            "the world sanctioned a state for being annoyed with its neighbour"
        );
        assert!(!w.at_war(NationId::Iraq), "rhetoric is not a war");

        // Climb to the campaign. The rung is where the world answers.
        for r in 2..=8 {
            bankroll(&mut w, NationId::Iraq);
            apply_command(
                &mut w,
                &Command::SetCommitment { conflict: id, nation: NationId::Iraq, rung: r },
            )
            .unwrap_or_else(|e| panic!("rung {} refused: {}", r, e));
        }
        assert!(
            w.sanctioned_by_count(NationId::Iraq) >= 3,
            "no coalition formed against an invasion: {} sanctioners",
            w.sanctioned_by_count(NationId::Iraq)
        );
        assert!(w.at_war(NationId::Iraq));
        assert!(
            w.headlines.iter().any(|h| h.contains("Iraq invades Kuwait")),
            "the invasion never made the news"
        );
        assert_eq!(
            w.conflict(id).unwrap().posture_of(NationId::Kuwait).unwrap().rung,
            8,
            "the defender did not answer an invasion of its own country"
        );
        // ...and it happens once, not once per climb.
        let sanctioners = w.sanctioned_by_count(NationId::Iraq);
        w.headlines.clear();
        bankroll(&mut w, NationId::Iraq);
        apply_command(
            &mut w,
            &Command::SetCommitment { conflict: id, nation: NationId::Iraq, rung: 9 },
        )
        .unwrap();
        assert!(!w.headlines.iter().any(|h| h.starts_with("WAR:")));
        assert_eq!(w.sanctioned_by_count(NationId::Iraq), sanctioners);
    }

    #[test]
    fn a_state_defends_itself_more_cheaply_than_it_invades() {
        // Escalation is explained to a parliament; a defence is not. Without
        // this a small state simply could not afford to resist, because the
        // ladder charged Kuwait exactly what it charged Iraq.
        let mut w = seeded(3);
        let th = theatre::TheatreId::Gulf;
        bankroll(&mut w, NationId::Iraq);
        apply_command(
            &mut w,
            &Command::OpenConflict { opener: NationId::Iraq, target: NationId::Kuwait, theatre: th },
        )
        .unwrap();
        let c = w.conflict_between(NationId::Iraq, NationId::Kuwait).unwrap();
        let attack = commitment::escalation_cost_in(&w, NationId::Kuwait, 1, 6, false);
        let defend = commitment::escalation_cost_in(&w, NationId::Kuwait, 1, 6, true);
        assert!(defend < attack * 0.5, "defending cost {:.1} against {:.1}", defend, attack);
        assert!(commitment::defending_home(&w, c, NationId::Kuwait));
        assert!(!commitment::defending_home(&w, c, NationId::Iraq));
    }

    #[test]
    fn no_power_goes_above_rung_five_without_a_host() {
        // BIBLE §6 object 3, as a single assertion: access is a diplomatic
        // quantity and it is a hard military gate. The United States cannot
        // mount a campaign in South Asia until somebody in range agrees to
        // carry it, however much political capital it is holding.
        let mut w = world_1990(GameRules::default());
        let th = theatre::TheatreId::SouthAsia;
        bankroll(&mut w, NationId::USA);
        apply_command(
            &mut w,
            &Command::OpenConflict { opener: NationId::USA, target: NationId::Pakistan, theatre: th },
        )
        .unwrap();
        let id = w.conflict_between(NationId::USA, NationId::Pakistan).unwrap().id;

        bankroll(&mut w, NationId::USA);
        let err = apply_command(
            &mut w,
            &Command::SetCommitment { conflict: id, nation: NationId::USA, rung: 6 },
        );
        assert!(err.is_err(), "a superpower reached rung 6 with nowhere to fly from");
        assert!(
            err.unwrap_err().contains("host"),
            "the refusal did not say why, which is the whole point of it"
        );
        assert_eq!(w.conflict(id).unwrap().posture_of(NationId::USA).unwrap().rung, 1);

        // Rung 5 and below need nobody's permission — that is what makes the
        // bottom of the ladder the thing a power without access actually does.
        bankroll(&mut w, NationId::USA);
        apply_command(
            &mut w,
            &Command::SetCommitment { conflict: id, nation: NationId::USA, rung: 5 },
        )
        .expect("deniable forces need no airfield");

        // ...and with a consenting host the same command goes through.
        force_access(&mut w, NationId::USA, NationId::India, th);
        bankroll(&mut w, NationId::USA);
        apply_command(
            &mut w,
            &Command::SetCommitment { conflict: id, nation: NationId::USA, rung: 8 },
        )
        .expect("with Delhi's consent the campaign is possible");
    }

    #[test]
    fn a_parliament_can_refuse_a_superpower() {
        // Turkey, March 2003. A host's answer is its own, it is not always yes,
        // and a refusal is not a formality: it leaves the seeker capped.
        let (mut granted, mut refused) = (0, 0);
        for seed in 0..40u64 {
            let mut w = seeded(seed);
            bankroll(&mut w, NationId::USA);
            apply_command(
                &mut w,
                &Command::RequestAccess {
                    seeker: NationId::USA,
                    host: NationId::Turkey,
                    theatre: theatre::TheatreId::Levant,
                },
            )
            .unwrap();
            if theatre::has_access(&w, NationId::USA, theatre::TheatreId::Levant) {
                granted += 1;
            } else {
                refused += 1;
            }
        }
        assert_eq!(granted + refused, 40, "a request went unanswered");
        assert!(refused >= 4, "Ankara never once said no: {}/40 refusals", refused);
        assert!(granted >= 4, "Ankara never once said yes: {}/40 grants", granted);
    }

    #[test]
    fn revoking_a_base_brings_a_superpower_down_the_ladder() {
        // The other end of the same table, and the reason a small state's
        // political capital matters in a great power's war.
        let mut w = seeded(5);
        let th = theatre::TheatreId::SouthAsia;
        force_access(&mut w, NationId::USA, NationId::India, th);
        let id = staged_conflict(&mut w, NationId::USA, NationId::Pakistan, th, 8, 4);
        assert_eq!(w.conflict(id).unwrap().posture_of(NationId::USA).unwrap().rung, 8);

        bankroll(&mut w, NationId::India);
        apply_command(
            &mut w,
            &Command::RevokeAccess { host: NationId::India, seeker: NationId::USA, theatre: th },
        )
        .unwrap();
        assert!(!theatre::has_access(&w, NationId::USA, th));
        assert_eq!(
            w.conflict(id).unwrap().posture_of(NationId::USA).unwrap().rung,
            theatre::MAX_RUNG_WITHOUT_ACCESS,
            "Washington kept its campaign after Delhi closed the airfields"
        );
    }

    #[test]
    fn the_ladder_binds_the_instruments_rather_than_duplicating_them() {
        // Rungs 2 to 5 are the statecraft systems that already exist. Climbing
        // has to *issue* them, through the same commands a player uses — if it
        // reimplements them instead, this is what says so.
        let mut w = seeded(9);
        w.rules.ai_aggression = 0.0;
        let th = theatre::TheatreId::Gulf;
        // The United States, with Kuwait beside it as the local party it arms.
        bankroll(&mut w, NationId::USA);
        apply_command(
            &mut w,
            &Command::OpenConflict {
                opener: NationId::USA,
                target: NationId::Iraq,
                theatre: th,
            },
        )
        .unwrap();
        let id = w.conflict_between(NationId::USA, NationId::Iraq).unwrap().id;
        war::join_side(w.conflict_mut(id).unwrap(), NationId::Kuwait, true, 1, Objective::Hold);

        assert!(!w.is_sanctioning(NationId::USA, NationId::Iraq));
        bankroll(&mut w, NationId::USA);
        apply_command(&mut w, &Command::SetCommitment { conflict: id, nation: NationId::USA, rung: 2 })
            .unwrap();
        assert!(
            w.is_sanctioning(NationId::USA, NationId::Iraq),
            "rung 2 is sanctions and it did not produce a sanctions row"
        );

        bankroll(&mut w, NationId::USA);
        apply_command(&mut w, &Command::SetCommitment { conflict: id, nation: NationId::USA, rung: 3 })
            .unwrap();
        assert!(
            w.aid_flow(NationId::USA, NationId::Kuwait, AidKind::Arms).is_some(),
            "rung 3 is arms to a proxy and no arms flow appeared"
        );

        // ...and coming back down lifts what climbing installed.
        bankroll(&mut w, NationId::USA);
        apply_command(&mut w, &Command::SetCommitment { conflict: id, nation: NationId::USA, rung: 1 })
            .unwrap();
        assert!(w.aid_flow(NationId::USA, NationId::Kuwait, AidKind::Arms).is_none());
        assert!(!w.is_sanctioning(NationId::USA, NationId::Iraq));
    }

    #[test]
    fn desert_storm_is_quick_when_they_stand_and_fight() {
        // Both sides in the open at rung 8 in flat desert, one of them with
        // twice the other's quality: the gate is wide open, the kill rates are
        // lopsided, the loser's structure is destroyed, and the ground changes
        // hands. This must be decisive and it must be quick.
        let mut quick = 0;
        for seed in 0..8u64 {
            let mut w = seeded(seed);
            w.rules.ai_aggression = 0.0;
            war::declare_war(&mut w, NationId::Iraq, NationId::Kuwait).unwrap();
            let mut months = 0;
            for m in 0..120 {
                tick_month(&mut w, &[]);
                if w.conflict_between(NationId::Iraq, NationId::Kuwait).is_none() {
                    months = m + 1;
                    break;
                }
            }
            assert!(months > 0, "seed {}: the Gulf war never ended in ten years", seed);
            assert!(
                w.nation(NationId::Kuwait).alive,
                "seed {}: the coalition turned up and Kuwait was annexed anyway",
                seed
            );
            if months <= 36 {
                quick += 1;
            }
        }
        assert!(quick >= 6, "a conventional war against a coalition dragged: {}/8 inside 3 years", quick);
    }

    #[test]
    fn afghanistan_does_not_end() {
        // The case the whole model exists for, and the one a progress bar cannot
        // express. An expeditionary power at rung 8 against a local party at
        // rung 4, in rough and built-up country: it takes the ground within a
        // year and holds it indefinitely, its kill rate against a target that
        // barely exposes itself is nowhere near decisive, and futility eats its
        // government's resolve while the defender — at home, taking almost no
        // casualties — never runs out.
        //
        // Nothing here is named after a country. It is the same six lines that
        // produce Desert Storm, with the rungs mismatched and the terrain rough.
        let mut w = seeded(3);
        w.rules.ai_aggression = 0.0;
        w.player = Some(NationId::USA); // Washington's posture is the test's, not the AI's
        let th = theatre::TheatreId::SouthAsia;
        force_access(&mut w, NationId::USA, NationId::India, th);
        // Rung 9, occupation: less combat power in the field than a campaign
        // and vastly more garrison-months, and — the part that matters — an
        // ordnance burn a rich state's industry can very nearly sustain forever.
        // An occupation is affordable in magazines and unaffordable in politics,
        // which is the entire shape of the thing.
        let id = staged_conflict(&mut w, NationId::USA, NationId::Pakistan, th, 9, 4);

        for _ in 0..60 {
            tick_month(&mut w, &[]);
        }
        let c = w.conflict(id).expect("an occupation does not resolve itself in five years");
        let usa = c.posture_of(NationId::USA).expect("still there");
        let pak = c.posture_of(NationId::Pakistan).expect("still there");
        assert!(
            c.control > 0.80,
            "the occupier does not hold the ground: control {:+.2}",
            c.control
        );
        assert!(
            usa.resolve < 0.35,
            "holding ground it cannot convert into an ending cost the occupier nothing: resolve {:.2}",
            usa.resolve
        );
        assert!(
            pak.resolve > 0.55,
            "the defender's will collapsed anyway: resolve {:.2}",
            pak.resolve
        );
        assert_eq!(c.class(), ConflictClass::Irregular, "mismatched rungs are not reading as irregular");

        // ...and the same occupier against the same country standing and
        // fighting in the open is a different war entirely.
        let mut open = seeded(3);
        open.rules.ai_aggression = 0.0;
        open.player = Some(NationId::USA);
        force_access(&mut open, NationId::USA, NationId::India, th);
        let oid = staged_conflict(&mut open, NationId::USA, NationId::Pakistan, th, 8, 8);
        for _ in 0..60 {
            tick_month(&mut open, &[]);
        }
        let their_resolve = open
            .conflict(oid)
            .and_then(|c| c.posture_of(NationId::Pakistan))
            .map_or(0.0, |b| b.resolve);
        assert!(
            open.conflict(oid).is_none() || their_resolve < pak.resolve,
            "standing in the open cost the defender no more than hiding did: {:.2} vs {:.2}",
            their_resolve,
            pak.resolve
        );
    }

    #[test]
    fn magazines_run_dry() {
        // BIBLE §6's second stock, on its own time constant. A campaign empties
        // the magazines faster than any budget refills them, and what stops is
        // the shooting — not the quarrel.
        //
        // The opponent is Iran and it used to be Saudi Arabia. Not a widened
        // bound — every assertion below is the one that was here — but the
        // staging had stopped being able to ask the question. Climbing to rung 8
        // now summons the coalition that a rung-8 climb deserves, and against a
        // client the majors actually like, the war Iraq gets is eleven months
        // long and settled at a table before any magazine could empty. Iran is
        // the case the assertion was always really about: eight years of exactly
        // this, both sides shooting off more than their industry could replace
        // and neither able to interest a great power in coming.
        //
        // Iran stopped being enough on its own, and this is the second and last
        // time the staging moves. The government module now gives Iran pillars
        // and a coalition that strains, so its resolve is the first of the three
        // stocks to reach bottom: the conflict ends in month 8 with "Iran sues
        // for peace, ceding territory to Iraq", seven months of drain short of
        // the magazine emptying. Iraq's ordnance was going at 0.065 a month
        // exactly as designed and would have been gone near month 15 — inside
        // this test's band, which has NOT moved — but a settlement got there
        // first, and what the measurement was then reporting was a fact about
        // resolve wearing the name of a fact about munitions.
        //
        // So the political stock is held still and the logistical one is left
        // entirely alone. `hold_open` above says exactly what that means and why
        // it cannot buy a pass. The band is still 6..30 and it is still checked
        // in both directions, because `magazines_are_not_a_bottomless_tap`
        // below pins the drain rate itself with no war in the way at all: a burn
        // slow enough to survive thirty months, or fast enough to empty inside
        // six, turns that test red as well as this one.
        let mut w = seeded(1);
        w.rules.ai_aggression = 0.0;
        w.player = Some(NationId::Iraq);
        let th = theatre::TheatreId::Gulf;
        let id = staged_conflict(&mut w, NationId::Iraq, NationId::Iran, th, 8, 8);
        assert_eq!(w.nation(NationId::Iraq).munitions, 1.0);

        let mut dry_at = None;
        let mut rung_after_dry = None;
        for m in 0..60 {
            tick_month(&mut w, &[]);
            hold_open(&mut w, id);
            let empty = w.nation(NationId::Iraq).munitions <= 0.0;
            if empty && dry_at.is_none() {
                dry_at = Some(m + 1);
            }
            // When the shooting stops, counted from the month the magazine
            // empties. Not read at month 60: what happens after the tempo falls
            // is the rest of the model — the quarrel goes quiet, and a quiet
            // invasion is eventually given a verdict — and asking at the end
            // made the assertion conditional on the war still being on the
            // board, which is the same mistake in a second place. A settlement
            // could skip it silently, and did.
            //
            // A dry army comes off the shooting rungs one rung a month, so from
            // rung 8 this is three months and never instant. That gradient is
            // the point: an arsenal running out is a tempo falling away, not a
            // switch.
            if empty && rung_after_dry.is_none() {
                if let Some(b) = w.conflict(id).and_then(|c| c.posture_of(NationId::Iraq)) {
                    if b.rung <= war::MAX_SUSTAINABLE_DRY {
                        rung_after_dry = Some(m + 1 - dry_at.unwrap_or(m + 1));
                    }
                }
            }
        }
        let dry = dry_at.expect("Iraq shot for five years and never ran short");
        assert!(
            (6..30).contains(&dry),
            "a poor state's magazines lasted {} months of full campaign",
            dry
        );
        let stopped = rung_after_dry
            .expect("an army with nothing left to fire never came off the shooting rungs");
        assert!(
            stopped <= 6,
            "it took {} months off an empty magazine to stop shooting",
            stopped
        );
    }

    #[test]
    fn magazines_are_not_a_bottomless_tap() {
        // The drain rate on its own, with nothing in the world able to pre-empt
        // it: no conflict, no resolve, no settlement, no coalition. This is the
        // half of `magazines_run_dry` that a war cannot interrupt, and it is
        // what makes that test's 6..30 band bite in both directions instead of
        // being a band a stopped clock could sit inside.
        //
        // A month of full conventional campaign burns `BURN_BY_RUNG[8]` out of a
        // magazine that holds 1.0, against a refill that scales with the
        // industry standing behind the army. For Iraq in 1990 — $0.46bn of
        // budget per point of force structure against the United States' $3.3bn
        // — that refill is nearly a rounding error, and the arsenal is a
        // one-shot weapon rather than a tap.
        let w = seeded(1);
        let burn = war::BURN_BY_RUNG[8];
        let refill = war::MAGAZINE_REBUILD * war::capital_intensity(&w, NationId::Iraq);
        let months = 1.0 / (burn - refill);
        assert!(
            (6.0..30.0).contains(&months),
            "Iraq's magazine buys {:.1} months of rung-8 fire (burn {:.4}/mo, \
             refill {:.4}/mo) — the same band `magazines_run_dry` asserts",
            months,
            burn,
            refill
        );
        // And the rich state's arsenal IS a tap: the same rung, the same stock,
        // and an order of magnitude more industry behind it. If this ever stops
        // being true, the capital-intensity term has collapsed into a constant
        // and §6's thesis — that the difference between a large army and a power
        // is derived, not typed — has gone with it.
        let us_refill = war::MAGAZINE_REBUILD * war::capital_intensity(&w, NationId::USA);
        assert!(
            us_refill > refill * 4.0,
            "a superpower refills at {:.4}/mo against a poor state's {:.4}/mo",
            us_refill,
            refill
        );
    }

    #[test]
    fn every_nation_has_a_home() {
        // A state that is home to no theatre would be expeditionary in its own
        // capital and would fight for its own cities with the fraction of itself
        // it could have sent abroad. The dissolutions mutate theatre membership,
        // which is a new class of bug, and this is the guard against it.
        for seed in 0..6u64 {
            let mut w = seeded(seed);
            for _ in 0..360 {
                tick_month(&mut w, &[]);
            }
            for n in w.nations.iter().filter(|n| n.alive) {
                let homes = w.theatres.iter().filter(|t| t.home.contains(&n.id)).count();
                assert_eq!(
                    homes, 1,
                    "seed {}: {:?} is home to {} theatres in {}",
                    seed, n.id, homes, w.year
                );
            }
        }
    }

    #[test]
    fn the_war_layer_holds_its_own_invariants() {
        // The economic sweep's counterpart for the force package. Never widened,
        // only added to: every bound here is one the model claims outright.
        for seed in [0u64, 7, 1990] {
            let mut w = world_1990(GameRules { seed, ..GameRules::default() });
            for _ in 0..600 {
                tick_month(&mut w, &[]);
                for n in w.nations.iter().filter(|n| n.alive) {
                    assert!(
                        (0.0..=1.0).contains(&n.munitions),
                        "seed {} {:?} munitions {}",
                        seed, n.id, n.munitions
                    );
                    let d = war::deployable_fraction(&w, n.id);
                    assert!(
                        (0.02..=0.40).contains(&d),
                        "seed {} {:?} can project {:.2} of itself — the divide-by-strength ran away",
                        seed, n.id, d
                    );
                }
                for c in &w.conflicts {
                    for b in &c.posture {
                        assert!((1..=9).contains(&b.rung), "rung {} is not on the ladder", b.rung);
                        assert!(
                            (0.0..=1.0).contains(&b.resolve),
                            "{:?} resolve {}",
                            b.nation, b.resolve
                        );
                        assert!(b.resolve.is_finite() && b.stake.is_finite());
                    }
                    // The zombie-war guard: two dry magazines must not deadlock
                    // a conflict at a shooting rung forever.
                    assert!(
                        c.months < 600 || !c.shooting(),
                        "a shooting war has run {} months in {}",
                        c.months, w.year
                    );
                    // The front holds its own shape: every hold a finite
                    // number on the board, every key a district the census
                    // ships, every pocket a sorted nonempty group of them.
                    for (d, h) in &c.front {
                        assert!(
                            h.is_finite() && (-1.0..=1.0).contains(&(*h as f64)),
                            "seed {} front {} holds {}",
                            seed, d, h
                        );
                        assert!(
                            districts::area_of(d) > 0.0,
                            "seed {} front key {} is not in the census",
                            seed, d
                        );
                    }
                    for p in &c.pockets {
                        assert!(!p.is_empty(), "seed {} an empty pocket group", seed);
                        assert!(
                            p.windows(2).all(|x| x[0] < x[1]),
                            "seed {} pocket group not sorted-unique: {:?}",
                            seed, p
                        );
                        for d in p {
                            assert!(
                                districts::area_of(d) > 0.0,
                                "seed {} pocket member {} is not in the census",
                                seed, d
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn ukraine_leaves_the_union_without_the_bomb() {
        // Ukraine is not on the board in 1990 and has to be produced by the
        // union's collapse. When it is, it is the second economy of the wreck and
        // the one that disarmed: Russia keeps the arsenal, Kyiv hands its share
        // back under the Budapest assurances of 1994.
        let mut born = 0;
        for seed in 0..10u64 {
            let rules = GameRules { seed, ..GameRules::default() };
            let mut w = world_1990(rules);
            run_months(&mut w, 180); // to 2005
            if !w.has_flag("ussr_dissolved") {
                assert!(w.nation_opt(NationId::Ukraine).is_none(), "Ukraine without a dissolution");
                continue;
            }
            born += 1;
            let u = w.nation(NationId::Ukraine);
            let r = w.nation(NationId::Russia);
            assert!(!u.nuclear, "Ukraine kept the arsenal");
            assert!(r.nuclear, "Russia lost the arsenal");
            assert!(
                (40.0..90.0).contains(&u.population),
                "Ukraine's population is not a republic's worth: {:.0}m",
                u.population
            );
            let share = u.gdp / r.gdp;
            assert!(
                (0.10..0.60).contains(&share),
                "Ukraine is not the union's second economy: {:.2} of Russia",
                share
            );
        }
        assert!(born >= 6, "the union held together too often: {}/10", born);
    }

    #[test]
    fn a_dissolution_makes_no_choice_between_its_successors() {
        // WHY THIS EXISTS, stated plainly because the finding that prompted it
        // did not survive contact with the code. A review flagged the "largest
        // population share" tie-break on `TechState::inherit` as a determinism
        // hazard under iron rule 1. THERE IS NO SUCH TIE-BREAK, and there is no
        // such selection: `dissolve_ussr` and `dissolve_yugoslavia` both walk a
        // hard-coded table in a fixed order and hand EVERY successor the parent's
        // entire known set through `inherit`. Nothing chooses, so nothing can tie,
        // and there is nothing for iteration order to decide. (The sim holds no
        // HashMap or HashSet anywhere, and the population shares in the Soviet
        // table are all distinct with Russia largest at 0.51, so even the rule
        // the review had in mind would not tie on this data.)
        //
        // The hazard is real but it is in the FUTURE tense. The succession rule
        // under consideration — largest-population successor is the continuator
        // and keeps everything, the others keep less — would introduce exactly
        // the selection that does not exist today, and "largest share" is not by
        // itself a total order. So this test pins the absence: while every
        // successor inherits identically there is no choice to make, and the day
        // that stops being true is the day an explicit tie-break is owed.
        for seed in 0..8u64 {
            let rules = GameRules { seed, ..GameRules::default() };
            let mut w = world_1990(rules);
            let parent_at_start = w.nation(NationId::USSR).tech.count();
            // Snapshot on the tick the union comes apart, not years later. What
            // the successors do with the inheritance afterwards is research, and
            // research is supposed to move them apart; what this test is about is
            // the single instant where the dissolution decides who gets what.
            let mut dissolved = false;
            for _ in 0..180 {
                run_months(&mut w, 1);
                if w.has_flag("ussr_dissolved") {
                    dissolved = true;
                    break;
                }
            }
            if !dissolved {
                continue;
            }
            // The fifteen the union broke into, named here rather than derived,
            // so that adding a republic to the table makes this test say so.
            let born = [
                NationId::Russia, NationId::Ukraine, NationId::Belarus,
                NationId::Kazakhstan, NationId::Uzbekistan, NationId::Azerbaijan,
                NationId::Georgia, NationId::Lithuania, NationId::Moldova,
                NationId::Latvia, NationId::Armenia, NationId::Tajikistan,
                NationId::Kyrgyzstan, NationId::Turkmenistan, NationId::Estonia,
            ];
            let successors: Vec<NationId> = born
                .iter()
                .copied()
                .filter(|id| w.nation_opt(*id).is_some_and(|n| n.alive))
                .collect();
            assert!(
                successors.len() >= 10,
                "seed {}: only {} successors survived the collapse",
                seed, successors.len()
            );
            // Every one of them holds the same thing, and it is at least what the
            // union held on the day the game opened. They diverge afterwards by
            // researching, which is the model working; what must not differ is
            // what the dissolution ITSELF handed each of them.
            let first = w.nation(successors[0]).tech.known.clone();
            for id in &successors {
                assert_eq!(
                    w.nation(*id).tech.known, first,
                    "seed {}: {:?} came out of the dissolution holding a different set from \
                     {:?}. If that is deliberate, a continuator rule has been introduced and \
                     it now needs an EXPLICIT total order — largest population share alone is \
                     not one, and falling back on table order is iron rule 1.",
                    seed, id, successors[0]
                );
            }
            assert!(
                first.len() >= parent_at_start,
                "seed {}: successors hold {} where the union opened with {}",
                seed, first.len(), parent_at_start
            );
        }
    }

    // ---- The front projection: BIBLE section 5 as amended 2026-08-30 -------

    #[test]
    fn the_front_is_deterministic() {
        // Two same-seed worlds through a staged invasion agree byte for byte
        // every month — the front draws no RNG and iterates no map in
        // arbitrary order — and a different seed fights a different front.
        let stage = |seed: u64| {
            let mut w = seeded(seed);
            w.rules.ai_aggression = 0.0;
            war::declare_war(&mut w, NationId::Iraq, NationId::Kuwait).unwrap();
            w
        };
        let front_of = |w: &WorldState| {
            format!(
                "{:?}",
                w.conflict_between(NationId::Iraq, NationId::Kuwait)
                    .map(|c| (&c.front, &c.pockets))
            )
        };
        let mut a = stage(0);
        let mut b = stage(0);
        let mut fronts_a: Vec<String> = vec![];
        for month in 0..36 {
            tick_month(&mut a, &[]);
            tick_month(&mut b, &[]);
            assert_eq!(
                state_hash(&a),
                state_hash(&b),
                "month {}: same seed, same war, different world",
                month
            );
            fronts_a.push(front_of(&a));
        }
        assert_eq!(save(&a), save(&b), "the timelines agree and the bytes do not");
        let mut c = stage(4);
        let mut fronts_c: Vec<String> = vec![];
        for _ in 0..36 {
            tick_month(&mut c, &[]);
            fronts_c.push(front_of(&c));
        }
        assert_ne!(fronts_a, fronts_c, "two different seeds fought identical fronts");
    }

    #[test]
    fn the_aggregate_tracks_the_scalar_it_replaced() {
        // The projection's calibration defense, held permanently: the front
        // spends the legacy equation's movement, so a shadow scalar
        // integrated from `war::seize_terms` must stay within a dime of the
        // real control while no pocket has fired — a throttled budget, a
        // broken gauge or a double-counted top-up all blow the bound. The
        // shadow reads its terms a tick-phase apart from the real update, so
        // the bound is a dime and not an epsilon; the saturating equation
        // contracts the two together, which is the same property that keeps
        // the projection itself glued to the scalar it replaced.
        let mut w = seeded(0);
        w.rules.ai_aggression = 0.0;
        for m in nations::majors().iter().copied() {
            w.set_relation(m, NationId::Kuwait, 0.0);
        }
        war::declare_war(&mut w, NationId::Iraq, NationId::Kuwait).unwrap();
        let mut shadow = 0.0f64;
        let (mut cross_real, mut cross_shadow) = (None, None);
        let mut months_checked = 0;
        for month in 0..36i64 {
            let terms = w
                .conflict_between(NationId::Iraq, NationId::Kuwait)
                .map(|c| war::seize_terms(&w, c, shadow));
            let (push, hold_mult) = match terms {
                Some(t) => t,
                None => break,
            };
            shadow += push * (1.0 - shadow * shadow) - war::CONTROL_DECAY * shadow * hold_mult;
            shadow = shadow.clamp(-1.0, 1.0);
            tick_month(&mut w, &[]);
            let c = match w.conflict_between(NationId::Iraq, NationId::Kuwait) {
                Some(c) => c,
                None => break,
            };
            if !c.pockets.is_empty() {
                break; // the one sanctioned deviation begins here
            }
            months_checked += 1;
            assert!(
                (c.control - shadow).abs() <= 0.10,
                "month {}: control {:+.4} strayed from the shadow scalar {:+.4}",
                month,
                c.control,
                shadow
            );
            if cross_real.is_none() && c.control.abs() >= 0.35 {
                cross_real = Some(month);
            }
            if cross_shadow.is_none() && shadow.abs() >= 0.35 {
                cross_shadow = Some(month);
            }
        }
        assert!(months_checked >= 12, "the war ended before the bound was exercised");
        let r = cross_real.expect("the invasion never crossed the settlement window");
        let s = cross_shadow.expect("the shadow never crossed the settlement window");
        assert!(
            (r - s).abs() <= 2,
            "the ±0.35 crossing drifted: real month {}, shadow month {}",
            r,
            s
        );
    }

    /// Terrain-distribution probe. Stage `att` against `def` at rungs 8/8
    /// with the AI held still, then run ONE `front::project` with a budget
    /// computed from the projection's public vocabulary to fully cap every
    /// pass-1 frontier district — so each measured district's movement
    /// equals its own terrain cap, and the uncapped sweep gets only a
    /// rounding remainder, which lands on the FIRST district in priority
    /// order (returned, so a test can refuse to measure it). The
    /// frontier/cap replication here is the same shadow idiom
    /// `the_aggregate_tracks_the_scalar_it_replaced` uses for the control
    /// equation.
    ///
    /// `budget_ignores_river`: when true, the budget is computed as if no
    /// approach crossed a river. The river test needs this — a budget that
    /// modelled the shave would hand the (last-ranked) river-only district
    /// exactly its shaved spend, and the measured ratio would then be the
    /// budget arithmetic rather than the implementation's cap. With the
    /// shave un-modelled, the surplus drains to the first-priority district
    /// and the river district's movement is capped by front.rs alone.
    /// Returns (moved-off-base per frontier district, priority-first id).
    fn terrain_probe(
        att: NationId,
        def: NationId,
        th_id: theatre::TheatreId,
        budget_ignores_river: bool,
    ) -> (std::collections::BTreeMap<String, f64>, String) {
        use std::cmp::Reverse;
        let mut w = seeded(0);
        w.rules.ai_aggression = 0.0;
        let id = staged_conflict(&mut w, att, def, th_id, 8, 8);
        let th = theatre::theatre(&w, th_id).clone();
        let mut conflicts = std::mem::take(&mut w.conflicts);
        let c = conflicts.iter_mut().find(|c| c.id == id).expect("staged");
        let k = front::contested_set(&w, c);
        assert!(!k.k.is_empty(), "no contested ground staged");
        let us = 1.0 - 0.25 * th.urbanisation;
        // Pass-1 frontier of the advancing attacker, from the documented
        // rule: enemy ground short of the pole with a neighbour the attacker
        // owns (base ground in the contested set counts as held), or an
        // island reached at rung 8. River-only: every qualifying land
        // approach crossed.
        struct Cand {
            id: String,
            capacity: f64, // budgeted cap * price
            river_only: bool,
            held_n: usize,
            area: f64,
        }
        let mut cands: Vec<Cand> = vec![];
        for (d, &(is_a, area)) in &k.k {
            if is_a {
                continue; // the attacker's base ground opens at its pole
            }
            let adj = districts::adj_of(d);
            let (mut q, mut crossed, mut held_n) = (0usize, 0usize, 0usize);
            for n in adj {
                let base_held = k.k.get(n.as_str()).is_some_and(|&(na, _)| na);
                if base_held {
                    held_n += 1;
                }
                if base_held || w.districts.get(n.as_str()) == Some(&att) {
                    q += 1;
                    if districts::crosses_river(d, n) {
                        crossed += 1;
                    }
                }
            }
            let (qualifies, river_only) = if adj.is_empty() {
                (true, false) // sea reach at rung 8
            } else {
                (q > 0, q > 0 && q == crossed)
            };
            if !qualifies {
                continue;
            }
            let mut cap = front::SWING_CAP * front::tempo_of(districts::terrain_of(d)) * us;
            if river_only && !budget_ignores_river {
                cap *= front::RIVER_CROSS_TEMPO;
            }
            cap = cap.max(front::TEMPO_FLOOR);
            let price = area / (2.0 * k.area_b);
            cands.push(Cand { id: d.clone(), capacity: cap * price, river_only, held_n, area });
        }
        assert!(cands.len() >= 2, "need at least two frontier districts");
        // The projection's priority order among non-pocket enemy ground:
        // (river_only, held desc, area desc, id asc). The first-ranked
        // district absorbs whatever the capped phase could not place.
        cands.sort_by(|x, y| {
            x.river_only
                .cmp(&y.river_only)
                .then_with(|| Reverse(x.held_n).cmp(&Reverse(y.held_n)))
                .then_with(|| y.area.total_cmp(&x.area))
                .then_with(|| x.id.cmp(&y.id))
        });
        let first = cands.first().expect("nonempty").id.clone();
        let budget: f64 = cands.iter().map(|c| c.capacity).sum::<f64>() * (1.0 + 1e-9);
        front::project(&w, c, &k, budget, &th);
        assert!(c.pockets.is_empty(), "a pocket fired inside the probe: {:?}", c.pockets);
        let moved = cands
            .iter()
            .map(|cand| {
                let h = c.front.get(&cand.id).copied().unwrap_or(-1.0) as f64;
                (cand.id.clone(), h + 1.0)
            })
            .collect();
        (moved, first)
    }

    /// The largest `def`-owned district of `want` class with a land approach
    /// from `att`'s national ground that crosses no major river — the clean
    /// measurement target for the probe above.
    fn frontier_pick(
        att: NationId,
        def: NationId,
        want: districts::TerrainClass,
    ) -> Option<String> {
        districts::list_of(def)
            .iter()
            .filter(|d| districts::terrain_of(d) == want)
            .filter(|d| {
                districts::adj_of(d).iter().any(|n| {
                    districts::start_owner_1990(n) == Some(att)
                        && !districts::crosses_river(d, n)
                })
            })
            .max_by(|a, b| {
                districts::area_of(a)
                    .total_cmp(&districts::area_of(b))
                    .then_with(|| b.cmp(a))
            })
            .cloned()
    }

    #[test]
    fn mountain_ground_is_taken_more_slowly() {
        // Two Iranian districts on the Iraqi border, one Zagros mountain and
        // one lowland, each with a land approach crossing no major river,
        // both fully capped by the probe's budget: the mountain moves slower
        // by exactly the tempo ratio. Distribution, never throughput — the
        // budget itself is spent in full either way.
        let m = frontier_pick(NationId::Iraq, NationId::Iran, districts::TerrainClass::Mountain)
            .expect("the Zagros belt borders Iraq");
        let l = frontier_pick(NationId::Iraq, NationId::Iran, districts::TerrainClass::Lowland)
            .expect("the Mesopotamian border has lowland");
        // The 1e-9 rounding surplus is far inside the tolerance below, so
        // no district needs excluding from measurement here.
        let (moved, _first) =
            terrain_probe(NationId::Iraq, NationId::Iran, theatre::TheatreId::Gulf, false);
        let (mm, ml) = (moved[&m], moved[&l]);
        let want = front::TEMPO_MOUNTAIN / front::TEMPO_LOWLAND;
        assert!(
            (mm / ml - want).abs() < 1e-3,
            "{} moved {:.4}, {} moved {:.4}: ratio {:.4} != {:.2}",
            m, mm, l, ml, mm / ml, want
        );
        assert!(mm < ml, "mountain ground did not move more slowly");
    }

    #[test]
    fn a_river_crossing_slows_the_taking() {
        // The Shatt al-Arab, both as data (crossed both ways round, and the
        // Rio Grande beside it) and as behaviour: Basra is reachable from
        // Iranian ground only across the river, so under a staged Iranian
        // invasion it moves at half the tempo of a same-class lowland
        // district with a dry land approach.
        assert!(districts::crosses_river("IQ-BA", "IR-10"));
        assert!(districts::crosses_river("IR-10", "IQ-BA"));
        assert!(districts::crosses_river("MX-TAM", "US-TX"));
        let ba = "IQ-BA";
        assert_eq!(districts::terrain_of(ba), districts::TerrainClass::Lowland);
        let ir_adj: Vec<&String> = districts::adj_of(ba)
            .iter()
            .filter(|n| districts::start_owner_1990(n) == Some(NationId::Iran))
            .collect();
        assert!(!ir_adj.is_empty(), "Basra lost its Iranian border");
        assert!(
            ir_adj.iter().all(|n| districts::crosses_river(ba, n)),
            "every Iranian approach to Basra is the Shatt al-Arab"
        );
        let l = frontier_pick(NationId::Iran, NationId::Iraq, districts::TerrainClass::Lowland)
            .expect("the border has dry lowland approaches");
        let (moved, first) =
            terrain_probe(NationId::Iran, NationId::Iraq, theatre::TheatreId::Gulf, true);
        assert!(ba != first && l != first, "a target absorbs the unshaved surplus");
        let (mb, ml) = (moved[ba], moved[&l]);
        assert!(
            (mb / ml - front::RIVER_CROSS_TEMPO).abs() < 1e-3,
            "river-only {} moved {:.4} vs {} {:.4}: ratio {:.4} != {:.2}",
            ba, mb, l, ml, mb / ml, front::RIVER_CROSS_TEMPO
        );
        assert!(mb < ml, "the river did not slow the taking");
    }

    #[test]
    fn the_cold_and_the_open_read_from_the_map() {
        // The behavioural half (the data half sits in districts.rs beside
        // the loader): desert ground moves FASTER than the reference —
        // manoeuvre outruns it in the open — and tundra slower, both by
        // exactly their tempo constants under the probe's full-cap budget.
        // Desert: Anbar's Syrian Desert against the Euphrates lowland, under
        // a staged Saudi push north.
        let an =
            frontier_pick(NationId::SaudiArabia, NationId::Iraq, districts::TerrainClass::Desert)
                .expect("the Syrian Desert borders Saudi ground");
        let mu = frontier_pick(
            NationId::SaudiArabia,
            NationId::Iraq,
            districts::TerrainClass::Lowland,
        )
        .expect("the Saudi border reaches Iraqi lowland");
        let (moved, _first) =
            terrain_probe(NationId::SaudiArabia, NationId::Iraq, theatre::TheatreId::Gulf, false);
        let (ms, mi) = (moved[&an], moved[&mu]);
        assert!(
            (ms / mi - front::TEMPO_DESERT).abs() < 1e-3,
            "desert {} moved {:.4} vs lowland {} {:.4}: ratio {:.4} != {:.2}",
            an, ms, mu, mi, ms / mi, front::TEMPO_DESERT
        );
        assert!(ms > mi, "open going did not outrun the reference");
        // Tundra: Norrbotten against Sweden's lowland border, under a staged
        // Norwegian push east (both home in Western Europe).
        let tundra =
            frontier_pick(NationId::Norway, NationId::Sweden, districts::TerrainClass::Tundra)
                .expect("Norrbotten borders Norway");
        let low =
            frontier_pick(NationId::Norway, NationId::Sweden, districts::TerrainClass::Lowland)
                .expect("the Scandinavian border has lowland");
        let (moved, _first) = terrain_probe(
            NationId::Norway,
            NationId::Sweden,
            theatre::TheatreId::WesternEurope,
            false,
        );
        let (mt, ml) = (moved[&tundra], moved[&low]);
        assert!(
            (mt / ml - front::TEMPO_TUNDRA).abs() < 1e-3,
            "tundra {} moved {:.4} vs lowland {} {:.4}: ratio {:.4} != {:.2}",
            tundra, mt, low, ml, mt / ml, front::TEMPO_TUNDRA
        );
        assert!(mt < ml, "the cold did not slow the taking");
    }

    /// Stage Iraq against Iran with most of Iran's ground hand-held by the
    /// invader, one interior district `x` left to Iran and sealed inside the
    /// captured ground, and Iran's largest district `y` left as its main
    /// mass. Returns (conflict id, x, y).
    fn staged_pocket(w: &mut WorldState, opener_rung: u8, target_rung: u8) -> (u32, String, String) {
        w.rules.ai_aggression = 0.0;
        w.player = Some(NationId::Iraq);
        let id = staged_conflict(w, NationId::Iraq, NationId::Iran, theatre::TheatreId::Gulf, opener_rung, target_rung);
        let iran: std::collections::BTreeSet<&str> =
            districts::list_of(NationId::Iran).iter().map(|d| d.as_str()).collect();
        let y = iran
            .iter()
            .copied()
            .min_by(|a, b| {
                districts::area_of(b)
                    .total_cmp(&districts::area_of(a))
                    .then_with(|| a.cmp(b))
            })
            .expect("Iran has districts")
            .to_string();
        let x = iran
            .iter()
            .copied()
            .find(|d| {
                let adj = districts::adj_of(d);
                **d != *y
                    && !adj.is_empty()
                    && adj.iter().all(|n| iran.contains(n.as_str()) && *n != y)
            })
            .expect("Iran has an interior district")
            .to_string();
        let c = w.conflict_mut(id).expect("just staged");
        for d in iran {
            if d != x && d != y {
                c.front.insert(d.to_string(), 1.0);
            }
        }
        (id, x, y)
    }

    #[test]
    fn encirclement_emerges_from_the_map() {
        // A held district sealed inside enemy-held ground, with no path to
        // its side's main mass, is found by the BFS and degrades by the
        // pocket swing in one month — while a district far behind the front
        // does not move at all.
        let mut w = seeded(1);
        let (id, x, _y) = staged_pocket(&mut w, 8, 8);
        tick_month(&mut w, &[]);
        let c = w.conflict(id).expect("one month in");
        assert!(
            c.pockets.iter().any(|p| p.contains(&x)),
            "{} is sealed inside enemy ground and no pocket was found: {:?}",
            x,
            c.pockets
        );
        let hx = c.front.get(&x).copied().expect("the pocket moved") as f64;
        assert!(
            hx >= -1.0 + front::POCKET_SWING - 1e-6,
            "the pocket at {} degraded only to {:+.3}",
            x,
            hx
        );
        // Anbar sits on the far side of Iraq, connected to the main mass and
        // nowhere near the fighting: it does not move.
        let anbar = c.front.get("IQ-AN").map_or(1.0, |h| *h as f64);
        assert!(
            anbar > 0.9,
            "a connected rear district moved with the pocket: IQ-AN {:+.3}",
            anbar
        );
    }

    #[test]
    fn a_pocket_collapses_without_an_assault() {
        // Both sides at the blockade rung with identical forces and Hold
        // orders: the push is exactly zero, no budget is advancing anywhere,
        // and the sealed district still flips — encirclement is an outcome of
        // the map, not of an assault the budget paid for.
        let mut w = seeded(1);
        {
            // Identical belligerents, so seize_a == seize_b and push == 0.
            let (gdp, spend, strength, tech) = {
                let n = w.nation(NationId::Iraq);
                (n.gdp, n.mil_spend_gdp, n.mil_strength, n.tech.clone())
            };
            let iran = w.nation_mut(NationId::Iran);
            iran.gdp = gdp;
            iran.mil_spend_gdp = spend;
            iran.mil_strength = strength;
            iran.tech = tech;
        }
        let (id, x, _y) = staged_pocket(&mut w, 6, 6);
        let mut flipped_at = None;
        for month in 0..6 {
            {
                let c = w.conflict_mut(id).expect("staged");
                for b in c.posture.iter_mut() {
                    b.rung = 6;
                    b.objective = Objective::Hold;
                    b.resolve = b.resolve.max(0.6);
                }
            }
            tick_month(&mut w, &[]);
            let c = w.conflict(id).expect("a hold-hold quarrel does not end in six months");
            let hx = c.front.get(&x).map_or(-1.0, |h| *h as f64);
            if flipped_at.is_none() && hx > 1.0 / 3.0 {
                flipped_at = Some(month);
            }
        }
        let c = w.conflict(id).expect("still on");
        assert!(
            flipped_at.is_some(),
            "six months surrounded and {} never flipped: {:+.3}",
            x,
            c.front.get(&x).map_or(-1.0, |h| *h as f64)
        );
        assert!(
            !c.pockets.iter().any(|p| p.contains(&x)),
            "{} flipped and the pocket did not collapse",
            x
        );
        // The rear stayed still: no budget was advancing anywhere.
        let anbar = c.front.get("IQ-AN").map_or(1.0, |h| *h as f64);
        assert!(anbar > 0.9, "a zero-push month moved the rear: IQ-AN {:+.3}", anbar);
    }

    #[test]
    fn an_island_never_pockets() {
        // Kansas surrounded is a pocket; Hawaii surrounded is a garrison
        // with the sea at its back — an empty adjacency list is an anchor,
        // which is the whole of the contract's "islands simply have no land
        // neighbours".
        let mut w = seeded(0);
        w.rules.ai_aggression = 0.0;
        war::declare_war(&mut w, NationId::Canada, NationId::USA).unwrap();
        {
            let c = w
                .conflict_between(NationId::Canada, NationId::USA)
                .expect("just declared")
                .id;
            let c = w.conflict_mut(c).unwrap();
            for d in districts::list_of(NationId::USA) {
                if d != "US-HI" && d != "US-KS" {
                    c.front.insert(d.clone(), 1.0);
                }
            }
        }
        tick_month(&mut w, &[]);
        let c = w
            .conflict_between(NationId::Canada, NationId::USA)
            .expect("one month in");
        assert!(
            c.pockets.iter().any(|p| p.contains(&"US-KS".to_string())),
            "landlocked Kansas is surrounded and did not pocket: {:?}",
            c.pockets
        );
        assert!(
            !c.pockets.iter().any(|p| p.contains(&"US-HI".to_string())),
            "Hawaii pocketed with the sea at its back"
        );
    }

    #[test]
    fn the_last_defender_to_quit_is_the_loser() {
        // When the only defender is driven from the field, the ending must
        // name IT as the loser. Before this test, `defender()` on the emptied
        // side fell back to ORIGIN_ATTACKER: on seed 1990 China capitulated to
        // China in Jan 2016 — the winner disarmed itself and paid itself
        // reparations for winning — while Mongolia, which had just quit the
        // fight it lost, walked away untouched with the settled-claim flag
        // written against the wrong dyad.
        let mut w = seeded(0);
        w.rules.ai_aggression = 0.0;
        war::declare_war(&mut w, NationId::Iraq, NationId::Kuwait).unwrap();
        let id = w
            .conflict_between(NationId::Iraq, NationId::Kuwait)
            .expect("just declared")
            .id;
        let iraq_mil = w.nation(NationId::Iraq).mil_strength;
        let kuwait_gdp = w.nation(NationId::Kuwait).gdp;
        for _ in 0..6 {
            match w.conflict_mut(id) {
                None => break,
                Some(c) => {
                    // The China–Mongolia shape exactly: two principals, no
                    // coalition under arms — strip any joiner the declaration
                    // rallied.
                    c.side_a.retain(|x| *x == NationId::Iraq);
                    c.side_b.retain(|x| *x == NationId::Kuwait);
                    c.posture
                        .retain(|b| b.nation == NationId::Iraq || b.nation == NationId::Kuwait);
                    // Iraq owns the ground outright — the front map is the
                    // authority control is gauged from, so the map is what the
                    // test stages. Kuwait's government has nothing left and, at
                    // home under an army at the invasion rung, nothing to step
                    // back to.
                    for d in districts::list_of(NationId::Kuwait) {
                        c.front.insert(d.clone(), 1.0);
                    }
                    c.control = 0.95;
                    c.invasion_declared = true;
                    for b in c.posture.iter_mut() {
                        b.months_at_rung = 0;
                        if b.nation == NationId::Iraq {
                            b.rung = 8;
                            b.resolve = 1.0;
                        } else {
                            b.resolve = 0.0;
                            b.red_line = 0.0;
                        }
                    }
                }
            }
            tick_month(&mut w, &[]);
        }
        assert!(
            w.conflict(id).is_none(),
            "a defender with no resolve and no ground held out six months"
        );
        // The verdict lands on Kuwait: annexed outright, or alive and visibly
        // poorer for the terms.
        let (k_alive, k_gdp) = {
            let k = w.nation(NationId::Kuwait);
            (k.alive, k.gdp)
        };
        assert!(
            !k_alive || k_gdp < kuwait_gdp * 0.97,
            "the sole defender quit the fight and lost nothing: alive={} gdp {:.1} -> {:.1}",
            k_alive,
            kuwait_gdp,
            k_gdp
        );
        // ...and not on the winner: Iraq must not come out of its own conquest
        // subjugated.
        let a_mil = w.nation(NationId::Iraq).mil_strength;
        assert!(
            a_mil > iraq_mil * 0.55,
            "the winner came out of its own conquest disarmed: {:.1} -> {:.1}",
            iraq_mil,
            a_mil
        );
    }

    #[test]
    fn a_concession_prefers_the_ground_actually_held() {
        // The preferring comparator, held directly against `cede_share`: an
        // empty preference reproduces the value ranking list for list, and a
        // nonempty one moves the held ground first — even a low-value
        // district the value ranking would never have reached.
        let mut a = world_1990(GameRules::default());
        let mut b = world_1990(GameRules::default());
        let empty = std::collections::BTreeSet::new();
        let plain = districts::cede_share(&mut a, NationId::Iran, NationId::Iraq, 0.12);
        let pref0 =
            districts::cede_share_preferring(&mut b, NationId::Iran, NationId::Iraq, 0.12, &empty);
        assert_eq!(plain, pref0, "an empty preference must be cede_share exactly");
        assert_eq!(a.districts, b.districts, "the two paths moved different maps");

        // Iraq's smallest district — dead last in the value ranking.
        let smallest = districts::list_of(NationId::Iraq)
            .iter()
            .min_by(|x, y| {
                districts::area_of(x)
                    .total_cmp(&districts::area_of(y))
                    .then_with(|| x.cmp(y))
            })
            .expect("Iraq has districts")
            .clone();
        let mut w = world_1990(GameRules::default());
        let preferred: std::collections::BTreeSet<String> =
            [smallest.clone()].into_iter().collect();
        let ceded =
            districts::cede_share_preferring(&mut w, NationId::Iran, NationId::Iraq, 0.12, &preferred);
        assert_eq!(ceded.len(), 3, "ceil(0.12 * 18) is three districts");
        assert_eq!(ceded[0], smallest, "the held ground did not cede first");
        assert_eq!(
            &ceded[1..],
            &["IQ-AN".to_string(), "IQ-MU".to_string()],
            "the remainder is not the value ranking"
        );
        assert_eq!(w.districts.get(smallest.as_str()), Some(&NationId::Iran));
    }

    #[test]
    fn an_old_save_reloads_a_front() {
        // Strip "front" and "pockets" from a mid-war save — the shape a
        // pre-front build wrote — and load() must project the saved control
        // back onto the ground it summarizes, then hold it stable through a
        // year of save/load cycles.
        let mut w = seeded(0);
        w.rules.ai_aggression = 0.0;
        for m in nations::majors().iter().copied() {
            w.set_relation(m, NationId::Kuwait, 0.0);
        }
        war::declare_war(&mut w, NationId::Iraq, NationId::Kuwait).unwrap();
        run_months(&mut w, 6);
        let (id0, control0) = {
            let c = w
                .conflict_between(NationId::Iraq, NationId::Kuwait)
                .expect("the staged war must outlive the save point");
            assert!(!c.front.is_empty(), "six months of invasion drew no front");
            assert!(c.control.abs() > 0.001, "no ground changed hands in six months");
            (c.id, c.control)
        };

        let mut v: serde_json::Value = serde_json::from_str(&save(&w)).unwrap();
        let mut stripped_any = false;
        for c in v["conflicts"].as_array_mut().expect("conflicts serialize as an array") {
            let o = c.as_object_mut().unwrap();
            stripped_any |= o.remove("front").is_some();
            o.remove("pockets");
        }
        assert!(stripped_any, "a mid-war save carries its front");
        let stripped = serde_json::to_string(&v).unwrap();
        let mut x = load(&stripped).expect("a pre-front save must load");
        {
            let c = x.conflict(id0).expect("the war survived the surgery");
            assert!(!c.front.is_empty(), "the reseed hook never fired");
            assert!(
                (c.control - control0).abs() <= 0.01,
                "the reseeded aggregate {:+.4} strayed from the saved control {:+.4}",
                c.control,
                control0
            );
        }
        for month in 0..12 {
            tick_month(&mut x, &[]);
            let s = save(&x);
            let y = load(&s).expect("a front save must reload");
            assert_eq!(
                s,
                save(&y),
                "month {}: a reloaded front re-serialized differently",
                month
            );
            assert_eq!(state_hash(&x), state_hash(&y), "month {}: hash drift", month);
        }
    }
}
