pub mod economy;
pub mod init;
pub mod politics;
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

    fn seeded(seed: u64) -> WorldState {
        let mut rules = GameRules::default();
        rules.seed = seed;
        world_1990(rules)
    }

    /// Month index at which a given era opened, if it did.
    fn era_onset(w: &mut WorldState, era: TechEra, months: usize) -> Option<usize> {
        for m in 0..months {
            tick_month(w, &[]);
            if w.era == era {
                return Some(m);
            }
        }
        None
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
        assert!(end / start > 6.0, "China grew only {:.1}x in 30y", end / start);
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
    fn the_internet_era_opens_in_the_nineties() {
        // Nothing in the sim knows what year the internet is. The Information era
        // opens when the frontier crosses its onset level, so this is a claim about
        // how fast 1990's frontier compounds — and it should land in the mid-90s.
        let mut in_the_nineties = 0;
        let mut led_by_a_market_economy = 0;
        for seed in 0..10u64 {
            let mut w = seeded(seed);
            if let Some(m) = era_onset(&mut w, TechEra::Information, 360) {
                if (60..=132).contains(&m) {
                    in_the_nineties += 1;
                }
                let leader = w.tech_leader().expect("someone leads");
                if w.nation(leader).system == EconomySystem::Market {
                    led_by_a_market_economy += 1;
                }
            }
        }
        assert!(in_the_nineties >= 8, "internet era mistimed: {}/10 in 1995-2000", in_the_nineties);
        assert!(
            led_by_a_market_economy >= 8,
            "a command economy held the frontier into the internet era {}/10 times",
            10 - led_by_a_market_economy
        );
    }

    #[test]
    fn the_era_waits_for_the_frontier_not_the_calendar() {
        // The proof that the date above is an outcome and not a script: hobble the
        // inputs the frontier runs on and the same era arrives years late. A
        // date-triggered internet would not care.
        let mut normal = seeded(4);
        let quick = era_onset(&mut normal, TechEra::Information, 480).expect("era opened");

        let mut slow_world = seeded(4);
        for n in slow_world.nations.iter_mut() {
            // A third of the schooling and half the capital: a poorer, less
            // educated world invents more slowly, and its paradigms are that much
            // further off. Nothing else about the two worlds differs — same seed,
            // same nations, same calendar.
            n.education *= 0.30;
            n.priv_invest_gdp *= 0.50;
            n.state_invest_gdp *= 0.50;
        }
        // If it never arrives inside forty years, the point is made even harder.
        if let Some(m) = era_onset(&mut slow_world, TechEra::Information, 480) {
            assert!(
                m > quick + 36,
                "hobbling the frontier barely moved the era: {} vs {} months",
                m, quick
            );
        }
    }

    #[test]
    fn a_new_paradigm_accelerates_the_frontier() {
        // The late-90s productivity acceleration: the frontier should grow markedly
        // faster in the years after a paradigm opens than in the years before it,
        // because a fresh paradigm has room in it and an exhausted one does not.
        let mut accelerated = 0;
        for seed in 0..10u64 {
            let mut w = seeded(seed);
            let mut before = 0.0;
            let mut at_onset = 0.0;
            let mut onset_month = usize::MAX;
            for m in 0..180 {
                if m == 48 {
                    before = w.tech_frontier;
                }
                let was = w.era;
                tick_month(&mut w, &[]);
                if w.era != was && w.era == TechEra::Information {
                    at_onset = w.tech_frontier;
                    onset_month = m;
                }
                if onset_month != usize::MAX && m == onset_month + 60 {
                    let pre = (at_onset / before).ln() / ((onset_month - 48) as f64 / 12.0);
                    let post = (w.tech_frontier / at_onset).ln() / 5.0;
                    if post > pre * 1.25 {
                        accelerated += 1;
                    }
                    break;
                }
            }
        }
        assert!(
            accelerated >= 8,
            "the internet era did not accelerate the frontier: {}/10",
            accelerated
        );
    }

    #[test]
    fn open_economies_take_more_of_the_internet_era() {
        // The asymmetry that matters. Two identical Chinas, one of which is allowed
        // to be a market economy; by the end of the information era the open one is
        // measurably further along the same frontier. Machine tools can be bought
        // and bolted down by a state that permits nothing else. A network cannot.
        let mut open_ahead = 0;
        for seed in 0..6u64 {
            let mut closed = seeded(seed);
            let mut open = seeded(seed);
            open.nation_mut(NationId::China).system = EconomySystem::Market;
            run_months(&mut closed, 204); // to 2007, the back end of the era
            run_months(&mut open, 204);
            if open.tech_rel(NationId::China) > closed.tech_rel(NationId::China) * 1.10 {
                open_ahead += 1;
            }
        }
        assert!(
            open_ahead >= 5,
            "openness bought nothing in the information era: {}/6",
            open_ahead
        );
    }

    #[test]
    fn a_command_economy_leads_one_paradigm_and_misses_the_next() {
        // The historically interesting shape: a planner can close a gap in the
        // paradigm that rewards pouring capital into plant, stalls badly in the one
        // that rewards letting people connect to each other, and gets a second wind
        // when the frontier turns back toward manufacturing at scale.
        let mut rotated = 0;
        for seed in 0..8u64 {
            let mut w = seeded(seed);
            let mut rate: Vec<(TechEra, f64)> = vec![];
            let (mut last_rel, mut last_m) = (w.tech_rel(NationId::China), 0usize);
            for m in 1..=420 {
                let was = w.era;
                tick_month(&mut w, &[]);
                if w.era != was {
                    let rel = w.tech_rel(NationId::China);
                    rate.push((was, (rel - last_rel) / ((m - last_m) as f64 / 12.0)));
                    last_rel = rel;
                    last_m = m;
                }
            }
            let of = |e: TechEra| rate.iter().find(|(x, _)| *x == e).map(|(_, r)| *r);
            if let (Some(ind), Some(info), Some(mob)) = (
                of(TechEra::Industrial),
                of(TechEra::Information),
                of(TechEra::Mobile),
            ) {
                if info < ind * 0.75 && mob > info * 1.4 {
                    rotated += 1;
                }
            }
        }
        assert!(
            rotated >= 6,
            "the command economy's fortunes did not turn with the paradigm: {}/8",
            rotated
        );
    }

    #[test]
    fn japan_slips_behind_the_frontier_it_kept_investing_in() {
        // Japan's problem was never that it stopped building. It was that its
        // capital, its firms and its careers were organised around the paradigm
        // that was ending, and a bubble hangover defunded the research that might
        // have carried it into the next one.
        let mut slipped = 0;
        for seed in 0..10u64 {
            let mut w = seeded(seed);
            run_months(&mut w, 66); // 1995
            let before = w.tech_rel(NationId::Japan);
            run_months(&mut w, 132); // 2006
            let after = w.tech_rel(NationId::Japan);
            let jp = w.nation(NationId::Japan);
            let us = w.nation(NationId::USA);
            let still_investing = jp.priv_invest_gdp + jp.state_invest_gdp
                > us.priv_invest_gdp + us.state_invest_gdp;
            if after < before && still_investing {
                slipped += 1;
            }
        }
        assert!(
            slipped >= 8,
            "Japan kept pace through the internet era: {}/10 slipped",
            slipped
        );
    }

    #[test]
    fn the_poor_and_open_close_on_the_frontier() {
        // Catch-up is the main engine, and it should be visible: Korea and China
        // both start far back and both should be markedly closer after 30 years,
        // without either of them overtaking anyone.
        for seed in 0..5u64 {
            let mut w = seeded(seed);
            let k0 = w.tech_rel(NationId::SouthKorea);
            let c0 = w.tech_rel(NationId::China);
            run_months(&mut w, 360);
            let k1 = w.tech_rel(NationId::SouthKorea);
            let c1 = w.tech_rel(NationId::China);
            assert!(k1 > k0 * 1.25, "seed {}: Korea stalled {:.2}->{:.2}", seed, k0, k1);
            assert!(c1 > c0 * 1.40, "seed {}: China stalled {:.2}->{:.2}", seed, c0, c1);
            assert!(k1 < 1.0 && c1 < 1.0, "seed {}: a follower overtook the frontier", seed);
        }
    }

    #[test]
    fn a_technological_lead_is_a_military_lead() {
        // The same budget buys a different army at the frontier than two paradigms
        // behind it, so a lead in the laboratory shows up in what deters.
        let mut base = world_1990(GameRules::default());
        let mut advanced = world_1990(GameRules::default());
        base.rules.ai_aggression = 0.0;
        advanced.rules.ai_aggression = 0.0;
        // Iraq alone gets a frontier-grade technology base. Nothing else differs —
        // same budget, same GDP, same spending share.
        advanced.nation_mut(NationId::Iraq).tech = advanced.tech_frontier;
        run_months(&mut base, 120);
        run_months(&mut advanced, 120);
        let weak = base.nation(NationId::Iraq).mil_strength;
        let strong = advanced.nation(NationId::Iraq).mil_strength;
        assert!(
            strong > weak * 1.10,
            "technology bought no military edge: {:.1} vs {:.1}",
            strong, weak
        );
        assert!(
            base.tech_edge(NationId::USA) > base.tech_edge(NationId::Iraq),
            "the frontier power had no edge over a backward one"
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
}
