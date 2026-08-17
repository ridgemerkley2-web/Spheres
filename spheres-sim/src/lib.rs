pub mod data;
pub mod dyads;
pub mod economy;
pub mod government;
pub mod exact;
pub mod init;
pub mod nations;
pub mod politics;
pub mod statecraft;
pub mod stratagems;
pub mod tech;
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
    })
}

pub fn apply_command(w: &mut WorldState, c: &Command) -> Result<(), String> {
    // Priced and charged before anything happens, so a command that cannot be
    // afforded also cannot take effect.
    if let Some((payer, price, refusable)) = command_price(w, c) {
        if price > 0.0 {
            let held = w.nation(payer).political_capital;
            if refusable && held < price {
                return Err(format!(
                    "{} has not the standing: {:.0} political capital held, {:.0} needed.",
                    payer.name(), held, price
                ));
            }
            // A government that reneges past the end of its credit does not get
            // to owe political capital; it simply has none left.
            w.nation_mut(payer).political_capital = (held - price).max(0.0);
        }
    }
    match c {
        Command::SetInterestRate { nation, rate } => {
            w.nation_mut(*nation).interest_rate = rate.clamp(0.0, 0.60);
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
    }
    Ok(())
}

/// Advance the world one month. Commands are applied before systems tick.
pub fn tick_month(w: &mut WorldState, commands: &[Command]) -> Vec<String> {
    w.headlines.clear();
    for c in commands {
        if let Err(e) = apply_command(w, c) {
            w.headline(format!("[rejected] {:?}: {}", c, e));
        }
    }
    economy::tick(w);
    // Research is funded out of the output the economy has just produced, and
    // what it unlocks is in the nation's hands before the soldiers and the
    // politicians get their turn with it.
    tech::tick(w);
    // Pacts decide who is obliged to join a war and patronage decides who can
    // still afford one, so the standing arrangements are settled before the
    // fighting is worked out.
    statecraft::tick(w);
    stratagems::tick(w);
    stratagems::ai_stratagems(w);
    war::tick(w);
    politics::tick(w);

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
    serde_json::from_str(s).map_err(|e| e.to_string())
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

    #[test]
    fn the_frontier_does_not_run_away() {
        // The guard that was missing, and whose absence let the world run at
        // twice its real size undetected for weeks. Every other calibration test
        // asserts a *relative* outcome — China grows faster than Japan, Slovenia
        // escapes what Bosnia does not — so a world where everyone doubles
        // together passes all of them. This one is absolute.
        //
        // Real 35-year growth for these economies runs about 0.9%/yr (Japan) to
        // 2.5%/yr (USA). The ceiling here is 4.0% rather than 3.0% because Japan
        // is a known outstanding gap at ~3.0% (see ROADMAP), and a test that is
        // red on arrival teaches nothing. It is still tight enough to have
        // caught the bug that prompted it: trade agreements paying a permanent
        // growth rate put the USA at 4.8%/yr.
        let mature = [
            NationId::USA, NationId::Japan, NationId::Germany,
            NationId::France, NationId::UK, NationId::Italy,
        ];
        let start: Vec<(NationId, f64)> = {
            let w = world_1990(GameRules::default());
            mature.iter().map(|id| (*id, w.nation(*id).gdp)).collect()
        };
        let mut w = world_1990(GameRules::default());
        run_months(&mut w, 12 * 35);
        for (id, gdp_1990) in start {
            let n = w.nation(id);
            if !n.alive {
                continue;
            }
            let cagr = exact::powf(n.gdp / gdp_1990, 1.0 / 35.0) - 1.0;
            assert!(
                cagr < 0.040,
                "{:?} compounded {:.1}%/yr over 35 years — the frontier is running away",
                id, cagr * 100.0
            );
            assert!(
                cagr > 0.005,
                "{:?} compounded {:.1}%/yr — a developed economy has stalled",
                id, cagr * 100.0
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
        let target = NationId::Brazil;
        let coalition =
            [NationId::USA, NationId::UK, NationId::France, NationId::Germany, NationId::Japan];

        let mut control = world_1990(GameRules::default());
        control.rules.ai_aggression = 0.0;
        let c0 = control.nation(target).gdp;
        run_months(&mut control, 240);
        let base = exact::powf(control.nation(target).gdp / c0, 1.0 / 20.0) - 1.0;

        let mut treated = world_1990(GameRules::default());
        treated.rules.ai_aggression = 0.0;
        let t0 = treated.nation(target).gdp;
        let mut share_acc = 0.0;
        for _ in 0..240 {
            for i in &coalition {
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
        let mean_share = share_acc / 240.0;
        let lost = (base - after) * 100.0;

        assert!(
            (0.45..0.60).contains(&mean_share),
            "the G5 stopped weighing half the world ({:.3}); the anchors below are \
             quoted at that weight and no longer apply",
            mean_share
        );
        assert!(
            lost > 1.2,
            "half the world economy shut its doors to {} for twenty years and cost it \
             {:.2} points of annual growth ({:.2}% against {:.2}%). Sanctions have \
             stopped being an economic instrument.",
            target.name(), lost, after * 100.0, base * 100.0
        );
        assert!(
            lost < 2.5,
            "a sanctions regime cost {} {:.2} points of annual growth ({:.2}% against \
             {:.2}%) — more than the near-universal embargo of South Africa managed, \
             from a coalition weighing {:.0}% of world output",
            target.name(), lost, after * 100.0, base * 100.0, mean_share * 100.0
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
        let w = world_1990(GameRules::default());
        let h = state_hash(&w);
        assert_eq!(
            h, 0x1bb3d0e7c7919e2eu64,
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
        const GOLDEN: u64 = 0xef3e968249846a49;
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
            let mut rules = GameRules::default();
            rules.seed = seed;
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

    #[test]
    fn gulf_war_emerges() {
        // Iraq should invade Kuwait in a majority of early-90s runs.
        let mut invasions = 0;
        for seed in 0..10u64 {
            let mut rules = GameRules::default();
            rules.seed = seed;
            let mut w = world_1990(rules);
            let mut saw = false;
            for _ in 0..48 {
                let headlines = tick_month(&mut w, &[]);
                if headlines.iter().any(|h| h.contains("Iraq invades Kuwait")) {
                    saw = true;
                }
            }
            if saw || !w.nation(NationId::Kuwait).alive {
                invasions += 1;
            }
        }
        assert!(invasions >= 5, "Gulf War too rare: {}/10", invasions);
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
        let mut xs: Vec<f64> = Vec::new();
        for seed in 0..10u64 {
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
        let median = (xs[4] + xs[5]) / 2.0;
        assert!(
            (11.0..19.0).contains(&median),
            "China's median 30-year growth across ten seeds is {:.2}x \
             ({:.2}%/yr), outside the 11.0x-19.0x band anchored on the real \
             14.33x. Seeds: {:?}",
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
    fn a_poor_nation_still_picks_up_what_everyone_has() {
        // The frontier is supposed to be out of a poor nation's reach. The
        // ordinary is not. For a long time both were: the cost floor stood for
        // having to build the thing and took no account of whether the thing was
        // a bespoke fab or a shipping container, so it bound long before the
        // copying discount could bite and the smallest economies were shut out
        // of the whole tree. Vietnam finished a thirty-year run knowing nothing
        // whatsoever, and no test in the suite objected.
        for seed in [1990u64, 7, 42] {
            let mut rules = GameRules::default();
            rules.seed = seed;
            let mut w = world_1990(rules);
            run_months(&mut w, 360);
            let frontier = w.nations.iter().filter(|n| n.alive).map(|n| n.tech.count()).max().unwrap();
            assert!(frontier > 60, "seed {}: nobody got anywhere: frontier {}", seed, frontier);
            for n in w.nations.iter().filter(|n| n.alive) {
                assert!(
                    n.tech.count() >= 5,
                    "seed {}: {:?} knows {} technologies after thirty years while the frontier holds {}",
                    seed, n.id, n.tech.count(), frontier
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
            let mut rules = GameRules::default();
            rules.seed = seed;
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
            let mut rules = GameRules::default();
            rules.seed = seed;
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
    fn yugoslavia_comes_apart_in_the_nineties() {
        let mut broke = 0;
        for seed in 0..10u64 {
            let mut rules = GameRules::default();
            rules.seed = seed;
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
            let mut rules = GameRules::default();
            rules.seed = seed;
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
            let mut rules = GameRules::default();
            rules.seed = seed;
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
        let mut rules = GameRules::default();
        rules.seed = seed;
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

    #[test]
    fn a_trade_agreement_lifts_the_smaller_partner_and_then_binds_it() {
        let (mut base, mut open) = (seeded(2), seeded(2));
        for w in [&mut base, &mut open] {
            w.rules.ai_aggression = 0.0;
            w.player = Some(NationId::USA);
        }
        force_trade(&mut open, NationId::USA, NationId::Poland);
        run_months(&mut base, 240);
        run_months(&mut open, 240);
        let (b, o) = (base.nation(NationId::Poland).gdp, open.nation(NationId::Poland).gdp);
        assert!(o > b * 1.20, "twenty years of integration bought nothing: {:.0} vs {:.0}", o, b);

        // ...and the growth is the leash. Tearing the agreement up costs the
        // small partner an order of magnitude more than the large one.
        let (p0, u0) = (
            open.nation(NationId::Poland).gdp,
            open.nation(NationId::USA).gdp,
        );
        apply_command(
            &mut open,
            &Command::AbrogateTrade { from: NationId::USA, to: NationId::Poland },
        )
        .unwrap();
        let warsaw = 1.0 - open.nation(NationId::Poland).gdp / p0;
        let washington = 1.0 - open.nation(NationId::USA).gdp / u0;
        assert!(warsaw > 0.02, "the dependent partner shrugged it off: {:.4}", warsaw);
        assert!(
            warsaw > washington * 10.0,
            "dependency was symmetric: {:.4} vs {:.4}",
            warsaw,
            washington
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
            let mut rules = GameRules::default();
            rules.seed = seed;
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
            let mut rules = GameRules::default();
            rules.seed = seed;
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

    #[test]
    fn ukraine_leaves_the_union_without_the_bomb() {
        // Ukraine is not on the board in 1990 and has to be produced by the
        // union's collapse. When it is, it is the second economy of the wreck and
        // the one that disarmed: Russia keeps the arsenal, Kyiv hands its share
        // back under the Budapest assurances of 1994.
        let mut born = 0;
        for seed in 0..10u64 {
            let mut rules = GameRules::default();
            rules.seed = seed;
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
}


