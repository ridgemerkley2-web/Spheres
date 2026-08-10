pub mod economy;
pub mod init;
pub mod politics;
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
}

pub fn apply_command(w: &mut WorldState, c: &Command) -> Result<(), String> {
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
        let mut w = world_1990(GameRules::default());
        let start = w.nation(NationId::China).gdp;
        run_months(&mut w, 360); // 30 years
        let end = w.nation(NationId::China).gdp;
        let x = end / start;
        // Bounded on both sides. The floor is the miracle; the ceiling is there
        // because an unbounded assertion cannot tell a miracle from a runaway,
        // and a one-sided bound let the tree's arrival move this figure by a
        // third without anything noticing.
        assert!(x > 6.0, "China grew only {:.1}x in 30y", x);
        assert!(x < 14.0, "China ran away: {:.1}x in 30y", x);
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
    fn embargoes_eventually_lift() {
        // ...but they are not eternal. Grievance cools, and the market reopens.
        let mut w = world_1990(GameRules::default());
        w.rules.ai_aggression = 0.0;
        war::declare_war(&mut w, NationId::Iraq, NationId::Kuwait).unwrap();
        run_months(&mut w, 12 * 25);
        assert_eq!(
            w.sanctioned_by_count(NationId::Iraq),
            0,
            "Iraq still embargoed 25 years on"
        );
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

