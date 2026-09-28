use super::*;

fn fixture() -> WorldState {
    // Explicit small test endowment, not an S22 campaign measurement.
    let mut w = init::world_1990(GameRules {
        daily_simulation: true,
        military_operations: true,
        production_system: true,
        manufacturing_system: true,
        resource_market: true,
        economic_competition: true,
        ai_aggression: 0.0,
        crisis_intensity: 0.0,
        ..GameRules::default()
    });
    w.player = Some(NationId::USA);
    for id in [NationId::USA, NationId::France] {
        programs::set_construction_budget(&mut w, id, 0.001).unwrap();
        w.nation_mut(id).political_capital = 0.0;
    }
    w
}

fn order(id: NationId, amount: f64, days: u32, automatic: bool) -> Command {
    Command::Equipment {
        nation: id,
        order: EquipmentOrder::AirSupport {
            daily_budget_mn: amount,
            target_days: days,
            automatic,
        },
    }
}

// Preserve the old command's clone-and-apply validation as an independent
// oracle. Support has no political bill or fiscal-policy journal entry; fail
// loudly if that command contract changes rather than silently skipping it.
fn old_apply(w: &mut WorldState, c: &Command) -> Result<(), String> {
    let Command::Equipment {
        nation,
        order: order @ EquipmentOrder::AirSupport { .. },
    } = c
    else {
        panic!("support-only oracle");
    };
    apply_equipment_order(&mut w.clone(), *nation, order)?;
    assert!(command_price(w, c)
        .filter(|(_, price, _)| *price > 0.0)
        .is_none());
    assert!(fiscal_journal::before_policy(w, c).is_none());
    dispatch(w, c)
}

fn compare(w: &WorldState, c: &Command, refused: bool) -> WorldState {
    let before = serde_json::to_vec(w).unwrap();
    let mut old = w.clone();
    let expected = old_apply(&mut old, c);
    assert_eq!(expected.is_err(), refused, "{c:?}: {expected:?}");
    assert_eq!(world_refusal(w, c), expected.clone().err());
    assert_eq!(refusal_of(w, c), expected.clone().err());
    assert_eq!(
        serde_json::to_vec(w).unwrap(),
        before,
        "preview mutated world"
    );
    let mut actual = w.clone();
    assert_eq!(apply_command(&mut actual, c), expected);
    assert_eq!(
        serde_json::to_vec(&actual).unwrap(),
        serde_json::to_vec(&old).unwrap()
    );
    if refused {
        assert_eq!(
            serde_json::to_vec(&actual).unwrap(),
            before,
            "refusal partially applied"
        );
    }
    actual
}

#[test]
fn air_support_command_refusals_match_old_clone_and_apply_in_order() {
    let base = fixture();
    let id = NationId::USA;
    let malformed = order(id, f64::NAN, 0, true);
    // Earlier guards must still outrank the invalid budget and target.
    for change in [
        (|w: &mut WorldState| w.rules.daily_simulation = false) as fn(&mut WorldState),
        |w| w.rules.military_operations = false,
        |w| w.nation_mut(NationId::USA).alive = false,
        |w| w.nations.retain(|n| n.id != NationId::USA),
        |w| {
            w.rules.economic_competition = false;
            w.player = Some(NationId::France);
        },
    ] {
        let mut w = base.clone();
        change(&mut w);
        compare(&w, &malformed, true);
    }
    for amount in [
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
        -1.0,
        1_000_001.0,
    ] {
        let mut w = base.clone();
        w.nation_mut(id).program_budget = None;
        compare(&w, &order(id, amount, 0, true), true);
    }
    for days in [0, 366, u32::MAX] {
        let mut w = base.clone();
        w.nation_mut(id).program_budget = None;
        compare(&w, &order(id, 1.0, days, true), true);
    }
    let mut corrupt = base.clone();
    corrupt.nation_mut(id).equipment = Some(equipment::EquipmentState::default());
    corrupt.nation_mut(id).equipment.as_mut().unwrap().version = 0;
    compare(&corrupt, &order(id, 1.0, 30, true), true);
    // Missing accounts must outrank the corrupt equipment state.
    corrupt.nation_mut(id).program_budget = None;
    compare(&corrupt, &order(id, 1.0, 30, false), true);

    let mut maintenance = base.clone();
    equipment::set_maintenance_plan(&mut maintenance, id, 1.0).unwrap();
    maintenance
        .nation_mut(id)
        .equipment
        .as_mut()
        .unwrap()
        .maintenance_plan
        .as_mut()
        .unwrap()
        .daily_limit_bn = 1001.0;
    compare(&maintenance, &order(id, 1.0, 30, true), true);
    let mut policy = base.clone();
    equipment::set_air_support(&mut policy, id, 1.0, 30, true).unwrap();
    policy
        .nation_mut(id)
        .equipment
        .as_mut()
        .unwrap()
        .air_support
        .as_mut()
        .unwrap()
        .target_days = 0;
    compare(&policy, &order(id, 1.0, 30, false), true);
}

#[test]
fn air_support_command_preserves_boundaries_pending_edits_and_replay() {
    let base = fixture();
    // Player and opponent governments use the same ordinary command. Cover
    // zero standing, both automatic modes, and exact normalized-budget bounds.
    for id in [NationId::USA, NationId::France] {
        for automatic in [false, true] {
            for amount in [0.0, -0.0, -f64::from_bits(1), 1.0, 1_000_000.0] {
                for days in [1, 365] {
                    let c = order(id, amount, days, automatic);
                    let first = compare(&base, &c, false);
                    let state = first.nation(id).equipment.as_ref().unwrap();
                    assert_eq!(state.maintenance_plan.is_some(), automatic);
                    assert_eq!(first.nation(id).political_capital, 0.0);
                    let repeated = compare(&first, &c, false);
                    assert_eq!(
                        serde_json::to_vec(&first).unwrap(),
                        serde_json::to_vec(&repeated).unwrap()
                    );
                    compare(&repeated, &order(id, 25.0, 30, !automatic), false);
                }
            }
        }
    }
}

#[test]
fn air_support_command_preserves_active_terms_and_settled_receipt() {
    let id = NationId::USA;
    let mut w = compare(&fixture(), &order(id, 2.0, 30, true), false);
    // Two ordinary daily ticks reach and settle the prospective support plan.
    tick_day(&mut w, &[]);
    tick_day(&mut w, &[]);
    equipment::validate_state(w.nation(id)).unwrap();
    let old = w
        .nation(id)
        .equipment
        .as_ref()
        .unwrap()
        .air_support
        .clone()
        .unwrap();
    assert!(
        old.receipt.is_some(),
        "fixture must exercise retained paid-day evidence"
    );
    let edited = compare(&w, &order(id, 5.0, 60, false), false);
    let policy = edited
        .nation(id)
        .equipment
        .as_ref()
        .unwrap()
        .air_support
        .as_ref()
        .unwrap();
    assert_eq!(policy.receipt, old.receipt);
    assert_eq!(
        policy.previous.as_ref().unwrap().daily_budget_bn,
        old.daily_budget_bn
    );
    let replayed = compare(&edited, &order(id, 6.0, 90, true), false);
    let policy = replayed
        .nation(id)
        .equipment
        .as_ref()
        .unwrap()
        .air_support
        .as_ref()
        .unwrap();
    assert_eq!(policy.receipt, old.receipt);
    assert_eq!(
        policy.previous.as_ref().unwrap().daily_budget_bn,
        old.daily_budget_bn
    );
}
