pub mod economy;
pub mod finance;
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
    /// Fix, manage or free the currency. Pegging at today's rate is free to
    /// announce and expensive to keep.
    SetFxStance { nation: NationId, stance: FxStance },
    /// Break the rate deliberately, before the reserves make the decision.
    Devalue { nation: NationId },
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
        Command::SetFxStance { nation, stance } => {
            if w.fin_opt(*nation).is_none() {
                return Err("That state has no currency of its own.".into());
            }
            let f = w.fin_mut(*nation);
            if f.stance == *stance {
                return Ok(());
            }
            f.stance = *stance;
            f.peg_rate = f.fx_index;
            w.headline(format!(
                "{} moves to a {} exchange rate.",
                nation.name(),
                stance.label()
            ));
        }
        Command::Devalue { nation } => {
            match w.fin_opt(*nation) {
                None => return Err("That state has no currency of its own.".into()),
                Some(f) if f.stance == FxStance::Floating => {
                    return Err("A floating currency has nothing to devalue from.".into())
                }
                Some(_) => {}
            }
            finance::break_peg(w, *nation);
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
    finance::tick(w);
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

    /// The trap has to close on its own: a country that pegs, grows fast and
    /// borrows abroad while the peg holds must sometimes find out that the three
    /// of those are the same decision. Nothing here names a country or a year.
    #[test]
    fn a_boom_on_a_peg_sometimes_breaks_it() {
        let mut seeds_with_a_boom_bust = 0;
        for seed in 0..16u64 {
            let mut rules = GameRules::default();
            rules.seed = seed;
            let mut w = world_1990(rules);
            let mut saw = false;
            for _ in 0..(12 * 30) {
                // Who was riding a boom on a fixed rate before the month began.
                let exposed: Vec<NationId> = w
                    .nations
                    .iter()
                    .filter(|n| n.alive && n.growth_last > 0.035)
                    .filter(|n| {
                        w.fin_opt(n.id).map_or(false, |f| {
                            f.stance != FxStance::Floating && f.fx_debt_gdp > 0.08
                        })
                    })
                    .map(|n| n.id)
                    .collect();
                let headlines = tick_month(&mut w, &[]);
                for h in &headlines {
                    if h.contains("CURRENCY CRISIS")
                        && exposed.iter().any(|id| h.contains(id.name()))
                    {
                        saw = true;
                    }
                }
            }
            if saw {
                seeds_with_a_boom_bust += 1;
            }
        }
        assert!(
            seeds_with_a_boom_bust >= 9,
            "a pegged boom never ends badly: {}/16 seeds",
            seeds_with_a_boom_bust
        );
    }

    /// The centrepiece. When a currency breaks, the money does not reassess the
    /// country that broke — it reassesses everyone who looked like it. So the
    /// repricing has to land measurably harder on economies with the same
    /// regime, the same neighbourhood or the same foreign-currency leverage than
    /// on everyone else in the world that month.
    #[test]
    fn a_devaluation_is_repriced_onto_its_lookalikes() {
        let (mut alike_sum, mut alike_n) = (0.0, 0usize);
        let (mut other_sum, mut other_n) = (0.0, 0usize);
        for seed in 0..16u64 {
            let mut rules = GameRules::default();
            rules.seed = seed;
            let mut w = world_1990(rules);
            for _ in 0..(12 * 30) {
                let before: Vec<(NationId, FxStance, Region, f64, f64)> = w
                    .nations
                    .iter()
                    .filter(|n| n.alive)
                    .filter_map(|n| {
                        w.fin_opt(n.id)
                            .map(|f| (n.id, f.stance, n.id.region(), f.fx_debt_gdp, f.risk))
                    })
                    .collect();
                let headlines = tick_month(&mut w, &[]);
                let broke: Vec<NationId> = before
                    .iter()
                    .map(|b| b.0)
                    .filter(|id| {
                        headlines
                            .iter()
                            .any(|h| h.contains("CURRENCY CRISIS") && h.contains(id.name()))
                    })
                    .collect();
                let source = match broke.first() {
                    Some(id) => *before.iter().find(|b| b.0 == *id).unwrap(),
                    None => continue,
                };
                for b in before.iter().filter(|b| !broke.contains(&b.0)) {
                    // Resembling the casualty: fixed like it, and either next
                    // door to it or leveraged in the same foreign money.
                    let alike = b.1 != FxStance::Floating
                        && source.1 != FxStance::Floating
                        && (b.2 == source.2 || b.3.min(source.3) > 0.15);
                    let repriced = w.fin(b.0).risk - b.4;
                    if alike {
                        alike_sum += repriced;
                        alike_n += 1;
                    } else {
                        other_sum += repriced;
                        other_n += 1;
                    }
                }
            }
        }
        assert!(alike_n > 40, "too few lookalikes to judge: {}", alike_n);
        let alike = alike_sum / alike_n as f64;
        let other = other_sum / other_n.max(1) as f64;
        assert!(
            alike > other * 1.3,
            "contagion did not discriminate: lookalikes +{:.4} vs everyone else +{:.4}",
            alike,
            other
        );
    }

    /// The transmission channel, isolated. Two identical worlds; in one, the
    /// country that devalues owes its money to foreigners. The devaluation is
    /// the same size in both — what differs is whose balance sheet it lands on.
    #[test]
    fn foreign_currency_debt_turns_a_devaluation_into_a_depression() {
        let victim = NationId::France;
        let mut indebted = world_1990(GameRules::default());
        let mut clean = world_1990(GameRules::default());
        indebted.rules.ai_aggression = 0.0;
        clean.rules.ai_aggression = 0.0;
        indebted.fin_mut(victim).fx_debt_gdp = 0.60;
        clean.fin_mut(victim).fx_debt_gdp = 0.0;

        let cmd = [Command::Devalue { nation: victim }];
        tick_month(&mut indebted, &cmd);
        tick_month(&mut clean, &cmd);
        run_months(&mut indebted, 36);
        run_months(&mut clean, 36);

        let a = indebted.nation(victim).gdp;
        let b = clean.nation(victim).gdp;
        assert!(
            a < b * 0.90,
            "foreign-currency debt cost nothing: {:.0} vs {:.0}",
            a,
            b
        );
        assert!(
            indebted.nation(victim).stability < clean.nation(victim).stability,
            "a banking collapse left the regime untouched"
        );
    }

    /// A peg is cheap when the fundamentals agree with it and ruinous when they
    /// do not, and the roster starts with one of each: a riyal backed by the
    /// largest oil revenues on earth, and a zloty fixed against 550% inflation
    /// with $2.5bn in the vault. Neither outcome is written anywhere.
    #[test]
    fn a_peg_is_only_worth_what_backs_it() {
        let (mut riyal_broke, mut zloty_broke) = (0, 0);
        for seed in 0..12u64 {
            let mut rules = GameRules::default();
            rules.seed = seed;
            let mut w = world_1990(rules);
            for _ in 0..(12 * 8) {
                let headlines = tick_month(&mut w, &[]);
                for h in &headlines {
                    if !h.contains("CURRENCY CRISIS") {
                        continue;
                    }
                    if h.contains("Saudi Arabia") {
                        riyal_broke += 1;
                    }
                    if h.contains("Poland") {
                        zloty_broke += 1;
                    }
                }
            }
        }
        assert!(zloty_broke >= 8, "the zloty peg survived: {}/12", zloty_broke);
        assert!(
            riyal_broke <= 2,
            "the riyal peg broke {} times despite the reserves behind it",
            riyal_broke
        );
    }

    /// The knob has to move the thing it names.
    #[test]
    fn financial_volatility_changes_how_often_money_runs() {
        let count = |vol: f64| {
            let mut breaks = 0;
            for seed in 0..8u64 {
                let mut rules = GameRules::default();
                rules.seed = seed;
                rules.financial_volatility = vol;
                let mut w = world_1990(rules);
                for _ in 0..(12 * 25) {
                    breaks += tick_month(&mut w, &[])
                        .iter()
                        .filter(|h| h.contains("CURRENCY CRISIS"))
                        .count();
                }
            }
            breaks
        };
        let calm = count(0.35);
        let wild = count(2.0);
        assert!(
            wild > calm,
            "volatility knob did nothing: {} breaks calm vs {} wild",
            calm,
            wild
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
