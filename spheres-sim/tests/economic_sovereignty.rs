use spheres_sim::{
    apply_command, clock, domination,
    init::world_1990,
    sovereignty, state_hash,
    world::{GameRules, NationId, Pact, TradePact, WorldState},
    Command,
};

fn prepared() -> WorldState {
    let mut w = world_1990(GameRules {
        daily_simulation: true,
        economic_competition: true,
        ..GameRules::default()
    });
    let (a, b) = (NationId::USA, NationId::Canada);
    w.player = Some(a);
    w.nation_mut(a).political_capital = 100.0;
    w.nation_mut(b).political_capital = 100.0;
    w.shift_relation(a, b, 100.0);
    w.statecraft.pacts.push(Pact {
        a: a.min(b),
        b: a.max(b),
        since_year: 1990,
        since_month: 1,
    });
    w.statecraft.trade.push(TradePact {
        a: a.min(b),
        b: a.max(b),
        depth: 0.4,
    });
    w
}

#[test]
fn delivered_industrial_value_reaches_the_shared_dependency_and_compact_gate() {
    use spheres_sim::commerce::{Commerce, DeliveredSource};
    let mut w = prepared();
    w.statecraft.trade.clear();
    w.rules.production_system = true;
    w.rules.resource_gates = true;
    w.rules.resource_market = true;
    w.rules.logistics_routes = true;
    w.rules.physical_logistics = true;
    assert_eq!(w.trade_dependency(NationId::Canada, NationId::USA), 0.0);
    let value = w.nation(NationId::Canada).gdp * 0.12;
    w.commerce = Some(Commerce {
        sourcing: vec![DeliveredSource {
            day: clock::absolute_day(&w),
            buyer: NationId::Canada,
            seller: NationId::USA,
            reference_value_bn: value,
        }],
        ..Commerce::default()
    });
    assert!((w.trade_dependency(NationId::Canada, NationId::USA) - 0.12).abs() < 1e-12);
    assert!(sovereignty::quote(&w, NationId::USA, NationId::Canada).ready);
    w.rules.economic_competition = false;
    assert_eq!(w.trade_dependency(NationId::Canada, NationId::USA), 0.0);
}

#[test]
fn economic_compact_requires_trust_protection_and_asymmetric_dependence() {
    let w = prepared();
    assert!(sovereignty::quote(&w, NationId::USA, NationId::Canada).ready);
    for barrier in 0..6 {
        let mut blocked = w.clone();
        match barrier {
            0 => blocked.statecraft.pacts.clear(),
            1 => blocked.statecraft.trade.clear(),
            2 => blocked.shift_relation(NationId::USA, NationId::Canada, -100.0),
            3 => blocked.sanctions.push((NationId::USA, NationId::Canada)),
            4 => blocked.nation_mut(NationId::USA).gdp = blocked.nation(NationId::Canada).gdp,
            _ => blocked.rules.economic_competition = false,
        }
        let before = state_hash(&blocked);
        let q = sovereignty::quote(&blocked, NationId::USA, NationId::Canada);
        assert!(!q.ready, "barrier {barrier}");
        assert!(apply_command(
            &mut blocked,
            &Command::ProposeEconomicUnion {
                patron: NationId::USA,
                partner: NationId::Canada
            }
        )
        .is_err());
        assert_eq!(
            state_hash(&blocked),
            before,
            "Rejected compact spent or mutated state: {barrier}"
        );
    }
}

#[test]
fn consenting_compact_counts_control_without_transferring_output_or_provinces() {
    let mut w = prepared();
    let gdp: Vec<_> = w.nations.iter().map(|n| (n.id, n.gdp)).collect();
    let districts = w.districts.clone();
    let before = domination::status(&w, NationId::USA).subordinate_clients;
    apply_command(
        &mut w,
        &Command::ProposeEconomicUnion {
            patron: NationId::USA,
            partner: NationId::Canada,
        },
    )
    .unwrap();
    assert_eq!(
        domination::status(&w, NationId::USA).subordinate_clients,
        before + 1
    );
    assert_eq!(
        w.nation(NationId::USA).political_capital,
        100.0 - sovereignty::COMPACT_PC
    );
    assert_eq!(w.districts, districts);
    assert_eq!(
        w.nations.iter().map(|n| (n.id, n.gdp)).collect::<Vec<_>>(),
        gdp
    );
    assert!(sovereignty::hostility_blocked(
        &w,
        NationId::Canada,
        NationId::USA
    ));
    assert_eq!(
        spheres_sim::dyads::war_appetite(&w, NationId::Canada, NationId::USA),
        0.0
    );
    for command in [
        Command::Sanction {
            imposer: NationId::Canada,
            target: NationId::USA,
        },
        Command::DeclareWar {
            attacker: NationId::Canada,
            defender: NationId::USA,
        },
        Command::OpenConflict {
            opener: NationId::Canada,
            target: NationId::USA,
            theatre: spheres_sim::war::theatre_between(&w, NationId::Canada, NationId::USA),
        },
    ] {
        let before = state_hash(&w);
        assert!(apply_command(&mut w, &command).is_err());
        assert_eq!(state_hash(&w), before);
    }
}

#[test]
fn human_consent_and_affordability_cannot_be_bypassed() {
    let mut w = prepared();
    w.player = Some(NationId::Canada);
    let before = state_hash(&w);
    assert!(apply_command(
        &mut w,
        &Command::ProposeEconomicUnion {
            patron: NationId::USA,
            partner: NationId::Canada
        }
    )
    .is_err());
    assert_eq!(state_hash(&w), before);
    w.nation_mut(NationId::Canada).political_capital = 0.0;
    let before = state_hash(&w);
    assert!(apply_command(
        &mut w,
        &Command::JoinEconomicUnion {
            nation: NationId::Canada,
            patron: NationId::USA
        }
    )
    .is_err());
    assert_eq!(state_hash(&w), before);
    w.nation_mut(NationId::Canada).political_capital = 100.0;
    apply_command(
        &mut w,
        &Command::JoinEconomicUnion {
            nation: NationId::Canada,
            patron: NationId::USA,
        },
    )
    .unwrap();
    assert_eq!(w.nation(NationId::Canada).political_capital, 80.0);
}

#[test]
fn departure_is_available_when_poor_and_preserves_descendants() {
    let mut w = prepared();
    domination::subjugate(&mut w, NationId::USA, NationId::Canada);
    domination::subjugate(&mut w, NationId::Canada, NationId::UK);
    w.nation_mut(NationId::Canada).political_capital = 0.0;
    apply_command(
        &mut w,
        &Command::LeaveEconomicUnion {
            nation: NationId::Canada,
        },
    )
    .unwrap();
    assert_eq!(domination::direct_overlord(&w, NationId::Canada), None);
    assert_eq!(
        domination::direct_overlord(&w, NationId::UK),
        Some(NationId::Canada)
    );
    assert_eq!(w.nation(NationId::Canada).political_capital, 0.0);
    assert!(!sovereignty::hostility_blocked(
        &w,
        NationId::USA,
        NationId::Canada
    ));
}

#[test]
fn neglected_compact_unravels_on_three_reviews_not_three_days() {
    let mut w = prepared();
    apply_command(
        &mut w,
        &Command::ProposeEconomicUnion {
            patron: NationId::USA,
            partner: NationId::Canada,
        },
    )
    .unwrap();
    w.statecraft.pacts.clear();
    for _ in 0..90 {
        sovereignty::tick(&mut w);
        let after = state_hash(&w);
        sovereignty::tick(&mut w);
        assert_eq!(
            state_hash(&w),
            after,
            "Duplicate same-day review changed state"
        );
        assert_eq!(
            domination::direct_overlord(&w, NationId::Canada),
            Some(NationId::USA)
        );
        clock::advance_date(&mut w);
    }
    sovereignty::tick(&mut w);
    assert_eq!(domination::direct_overlord(&w, NationId::Canada), None);
}

#[test]
fn compact_save_resume_and_default_off_are_exact() {
    let mut w = prepared();
    apply_command(
        &mut w,
        &Command::ProposeEconomicUnion {
            patron: NationId::USA,
            partner: NationId::Canada,
        },
    )
    .unwrap();
    let mut resumed: WorldState =
        serde_json::from_str(&serde_json::to_string(&w).unwrap()).unwrap();
    for _ in 0..45 {
        sovereignty::tick(&mut w);
        sovereignty::tick(&mut resumed);
        clock::advance_date(&mut w);
        clock::advance_date(&mut resumed);
    }
    assert_eq!(state_hash(&w), state_hash(&resumed));
    let mut legacy = world_1990(GameRules::default());
    let before = state_hash(&legacy);
    sovereignty::tick(&mut legacy);
    assert_eq!(state_hash(&legacy), before);
    let json = serde_json::to_string(&legacy).unwrap();
    assert!(!json.contains("economic_competition"));
    assert!(!json.contains("sovereignty_day"));
    assert!(!json.contains("compacts"));
}

#[test]
fn a_subject_cannot_join_the_opposing_coalition_against_its_sphere() {
    let mut w = prepared();
    domination::subjugate(&mut w, NationId::USA, NationId::Canada);
    let theatre = spheres_sim::war::theatre_between(&w, NationId::USA, NationId::Iraq);
    let c = spheres_sim::commitment::open_conflict(&mut w, NationId::USA, NationId::Iraq, theatre)
        .unwrap();
    let before = state_hash(&w);
    assert!(apply_command(
        &mut w,
        &Command::JoinConflict {
            conflict: c,
            nation: NationId::Canada,
            side_a: false,
            objective: spheres_sim::world::Objective::Hold
        }
    )
    .is_err());
    assert_eq!(state_hash(&w), before);
}

#[test]
fn a_voluntary_merger_cannot_hide_descendants_war_or_sanctions() {
    let mut w = prepared();
    domination::subjugate(&mut w, NationId::USA, NationId::France);
    domination::subjugate(&mut w, NationId::Canada, NationId::Mexico);
    let th = spheres_sim::war::theatre_between(&w, NationId::Mexico, NationId::France);
    spheres_sim::commitment::open_conflict(&mut w, NationId::Mexico, NationId::France, th).unwrap();
    let before = state_hash(&w);
    assert!(!sovereignty::quote(&w, NationId::USA, NationId::Canada).ready);
    assert!(apply_command(
        &mut w,
        &Command::ProposeEconomicUnion {
            patron: NationId::USA,
            partner: NationId::Canada
        }
    )
    .is_err());
    assert_eq!(state_hash(&w), before);
    w.conflicts.clear();
    w.sanctions.push((NationId::Mexico, NationId::France));
    assert!(!sovereignty::quote(&w, NationId::USA, NationId::Canada).ready);
    w.sanctions.clear();
    let th = spheres_sim::war::theatre_between(&w, NationId::Mexico, NationId::Iraq);
    spheres_sim::commitment::open_conflict(&mut w, NationId::Mexico, NationId::Iraq, th).unwrap();
    assert!(
        sovereignty::quote(&w, NationId::USA, NationId::Canada).ready,
        "An unrelated descendant's external quarrel does not veto the two peaceful capitals"
    );
}
