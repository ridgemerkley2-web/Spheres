//! Regression for an armed threat hidden by the other institutions' loyalty.
use super::*;
use crate::{Command, GameRules};

fn threatened_military_government() -> WorldState {
    let id = NationId::Pakistan;
    let mut w = crate::init::world_1990(GameRules {
        seed: 0, ideology_blocs: true, ideology_takeover: true, ..GameRules::default()
    });
    w.nation_mut(id).stability = 25.0;
    let g = state_mut(&mut w, id).unwrap();
    g.months_in_office = 48;
    g.coup_pressure = 1.5;
    for (p, loyalty) in &mut g.pillars {
        *loyalty = if *p == Pillar::Army { 0.20 } else { 0.90 };
    }
    assert!(maybe_electoral_coup(&mut w, id));
    // A real military executive, with an organised civilian alternative.
    // The fresh seizure itself cleared pressure; stage a later live threat.
    assert_eq!(state(&w, id).unwrap().coup_pressure, 0.0);
    let g = state_mut(&mut w, id).unwrap();
    g.months_in_office = 48;
    g.coup_pressure = 0.4;
    for (p, loyalty) in &mut g.pillars {
        *loyalty = if *p == Pillar::Army { 0.20 } else { 0.90 };
    }
    w.nation_mut(id).political_capital = ROUND_TABLE_PC;
    assert!(!ai_party_can_contest_opening(&w, id));
    assert!(franchise_demand(&w, id) >= ROUND_TABLE_FRANCHISE_QUORUM);
    assert_eq!(round_table_refusal(&w, id), None);
    let armed: Vec<_> = state(&w, id).unwrap().pillars.iter()
        .filter(|(p, _)| matches!(p, Pillar::Army | Pillar::Security | Pillar::Party))
        .map(|(_, v)| *v).collect();
    assert!(armed.iter().sum::<f64>() / armed.len() as f64 >= 0.50);
    w
}

#[test]
fn live_armed_threat_is_not_hidden_by_loyal_institutions() {
    let id = NationId::Pakistan;
    let w = threatened_military_government();
    let saved = crate::save(&w);
    let command = Command::ConveneRoundTable { nation: id };
    assert_eq!(ai_lever(&w, id), Some(command.clone()));
    assert_eq!(crate::save(&w), saved, "a preference must not mutate the world or RNG");
    let restored = crate::load(&saved).unwrap();
    assert_eq!(ai_lever(&restored, id), Some(command));
    assert_eq!(crate::save(&restored), saved);
}

#[test]
fn recovered_or_absent_armed_threat_does_not_force_an_opening() {
    let id = NationId::Pakistan;
    for control in 0..5 {
        let mut w = threatened_military_government();
        match control {
            0 => state_mut(&mut w, id).unwrap().coup_pressure = 0.0,
            1 => state_mut(&mut w, id).unwrap().pillars.iter_mut()
                .find(|(p, _)| *p == Pillar::Army).unwrap().1 = 0.35,
            2 => state_mut(&mut w, id).unwrap().pillars.retain(|(p, _)|
                !matches!(p, Pillar::Army | Pillar::Security | Pillar::Party)),
            3 => w.nation_mut(id).political_capital = ROUND_TABLE_PC - 1.0,
            _ => w.rules.ideology_blocs = false,
        }
        assert_eq!(ai_lever(&w, id), None, "control {control}");
    }
}

#[test]
fn armed_threat_does_not_bypass_round_table_legality() {
    let id = NationId::Pakistan;
    let mut w = threatened_military_government();
    // Removing the current regime's organised movements closes the command.
    state_mut(&mut w, id).unwrap().movements.clear();
    assert!(round_table_refusal(&w, id).is_some());
    assert_eq!(ai_lever(&w, id), None);
}

#[test]
fn threatened_opening_keeps_the_real_price_and_first_ballot_in_both_clocks() {
    let id = NationId::Pakistan;
    for daily in [false, true] {
        let mut w = threatened_military_government();
        w.rules.daily_simulation = daily;
        let rng = w.rng.clone();
        let command = ai_lever(&w, id).expect("live threat has an affordable legal response");
        assert_eq!(crate::price_of(&w, &command), Some(ROUND_TABLE_PC));
        crate::apply_command(&mut w, &command).unwrap();
        assert_eq!(w.nation(id).political_capital, 0.0);
        assert!(is_electoral(&w, id));
        let g = state(&w, id).unwrap();
        assert!(g.awaiting_first_election);
        assert_eq!(g.next_election, add_months(w.year, w.month, 6));
        assert_eq!(w.rng, rng);
    }
}
