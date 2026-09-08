use spheres_sim::world::{Belligerent, Conflict, GameRules, NationId as N, Objective, WorldState};
use spheres_sim::{
    apply_command,
    campaign_peace::{self, OccupationPolicy, PeaceOrder, Terms, WarAim},
    init::world_1990,
    war, Command,
};

fn world() -> WorldState {
    let mut w = world_1990(GameRules {
        daily_simulation: true,
        military_operations: true,
        operational_warfare: 1,
        ..Default::default()
    });
    w.player = Some(N::Iran);
    for n in &mut w.nations {
        n.political_capital = 100.0;
    }
    w.conflicts.push(Conflict {
        id: 1,
        theatre: war::theatre_between(&w, N::Iraq, N::Iran),
        side_a: vec![N::Iraq],
        side_b: vec![N::Iran],
        posture: vec![
            Belligerent::new(N::Iraq, 8, Objective::Seize),
            Belligerent::new(N::Iran, 8, Objective::Hold),
        ],
        control: 0.0,
        months: 12,
        quiet_months: 0,
        frozen_since: None,
        start_year: 1990,
        start_month: 1,
        origin_attacker: N::Iraq,
        invasion_declared: true,
        front: Default::default(),
        pockets: vec![],
        aim: None,
    });
    w
}
fn command(w: &mut WorldState, n: N, order: PeaceOrder) -> Result<(), String> {
    apply_command(w, &Command::WarDiplomacy { nation: n, order })
}

#[test]
fn a_losing_ai_accepts_return_of_its_land_without_accepting_regime_change() {
    for (terms, accepted) in [(Terms::Ceasefire, true), (Terms::Transition, false)] {
        let mut w = world();
        w.player = Some(N::Iraq);
        let c = w.conflict_mut(1).unwrap();
        c.months = 1;
        c.control = 0.6;
        for b in &mut c.posture {
            b.resolve = 1.0;
        }
        command(&mut w, N::Iraq, PeaceOrder::Propose { conflict: 1, terms }).unwrap();
        spheres_sim::clock::advance_date(&mut w);
        campaign_peace::tick(&mut w);
        assert_eq!(w.conflict(1).is_none(), accepted);
        assert_eq!(
            w.campaign_peace.history.last().unwrap().outcome,
            if accepted { "accepted" } else { "rejected" }
        );
    }
}
#[test]
fn human_and_every_ally_must_consent_and_replaying_response_is_inert() {
    let mut w = world();
    let c = w.conflict_mut(1).unwrap();
    c.side_b.push(N::Syria);
    c.posture
        .push(Belligerent::new(N::Syria, 4, Objective::Hold));
    command(
        &mut w,
        N::Iraq,
        PeaceOrder::Propose {
            conflict: 1,
            terms: Terms::Ceasefire,
        },
    )
    .unwrap();
    command(
        &mut w,
        N::Iran,
        PeaceOrder::Respond {
            offer: 1,
            accept: true,
        },
    )
    .unwrap();
    assert!(w.conflict(1).is_some(), "ally still has not consented");
    command(
        &mut w,
        N::Syria,
        PeaceOrder::Respond {
            offer: 1,
            accept: true,
        },
    )
    .unwrap();
    assert!(w.conflict(1).is_none());
    let before = spheres_sim::save(&w);
    assert!(command(
        &mut w,
        N::Iran,
        PeaceOrder::Respond {
            offer: 1,
            accept: true
        }
    )
    .is_err());
    assert_eq!(before, spheres_sim::save(&w));
}
#[test]
fn peace_tick_never_accepts_for_the_human() {
    let mut w = world();
    w.conflict_mut(1)
        .unwrap()
        .posture_mut(N::Iran)
        .unwrap()
        .resolve = 0.0;
    command(
        &mut w,
        N::Iraq,
        PeaceOrder::Propose {
            conflict: 1,
            terms: Terms::Ceasefire,
        },
    )
    .unwrap();
    w.day = 2;
    campaign_peace::tick(&mut w);
    assert!(w.conflict(1).is_some());
    assert!(!w.campaign_peace.offers[0].approved.contains(&N::Iran));
}
#[test]
fn cash_reparations_transfer_once_without_a_gdp_reward() {
    let mut w = world();
    for id in [N::Iran, N::Iraq] {
        let n = w.nation_mut(id);
        n.treasury_bn = Some(100.0);
        n.debt_bn = Some(0.0);
    }
    let gdp = (w.nation(N::Iran).gdp, w.nation(N::Iraq).gdp);
    command(
        &mut w,
        N::Iraq,
        PeaceOrder::Propose {
            conflict: 1,
            terms: Terms::Reparations { share_bp: 100 },
        },
    )
    .unwrap();
    command(
        &mut w,
        N::Iran,
        PeaceOrder::Respond {
            offer: 1,
            accept: true,
        },
    )
    .unwrap();
    assert!(
        (w.nation(N::Iran).treasury_bn.unwrap() + w.nation(N::Iraq).treasury_bn.unwrap() - 200.0)
            .abs()
            < 1e-10
    );
    assert_eq!(gdp, (w.nation(N::Iran).gdp, w.nation(N::Iraq).gdp));
    assert!((w.nation(N::Iran).treasury_bn.unwrap() - (100.0 - gdp.0 * 0.01)).abs() < 1e-10);
}
#[test]
fn unheld_or_duplicate_cession_and_invalid_garrison_cost_nothing() {
    let mut w = world();
    let district = w
        .districts
        .iter()
        .find(|(_, n)| **n == N::Iran)
        .unwrap()
        .0
        .clone();
    let before = spheres_sim::save(&w);
    assert!(command(
        &mut w,
        N::Iraq,
        PeaceOrder::Propose {
            conflict: 1,
            terms: Terms::Cede {
                districts: vec![district.clone()]
            }
        }
    )
    .is_err());
    assert!(command(
        &mut w,
        N::Iraq,
        PeaceOrder::Garrison {
            conflict: 1,
            share_bp: 5001,
            policy: OccupationPolicy::Security
        }
    )
    .is_err());
    assert!(command(
        &mut w,
        N::Iraq,
        PeaceOrder::SetAim {
            conflict: 1,
            aim: WarAim::Recover {
                districts: vec![district.clone(), district]
            }
        }
    )
    .is_err());
    assert_eq!(before, spheres_sim::save(&w));
}
#[test]
fn proposal_cannot_outlive_changed_coalition_or_save_roundtrip() {
    let mut w = world();
    command(
        &mut w,
        N::Iraq,
        PeaceOrder::Propose {
            conflict: 1,
            terms: Terms::Ceasefire,
        },
    )
    .unwrap();
    let saved = spheres_sim::save(&w);
    let mut resumed = spheres_sim::load(&saved).unwrap();
    assert_eq!(saved, spheres_sim::save(&resumed));
    let c = resumed.conflict_mut(1).unwrap();
    c.side_b.push(N::Syria);
    c.posture
        .push(Belligerent::new(N::Syria, 4, Objective::Hold));
    let before = spheres_sim::save(&resumed);
    assert!(command(
        &mut resumed,
        N::Iran,
        PeaceOrder::Respond {
            offer: 1,
            accept: true
        }
    )
    .is_err());
    assert_eq!(before, spheres_sim::save(&resumed));
}
#[test]
fn occupation_is_idempotent_and_does_not_award_inventory_or_ownership() {
    let mut w = world();
    let district = w
        .districts
        .iter()
        .find(|(_, n)| **n == N::Iran)
        .unwrap()
        .0
        .clone();
    w.conflict_mut(1)
        .unwrap()
        .front
        .insert(district.clone(), 1.0);
    command(
        &mut w,
        N::Iraq,
        PeaceOrder::Garrison {
            conflict: 1,
            share_bp: 2500,
            policy: OccupationPolicy::Restraint,
        },
    )
    .unwrap();
    let inventory = serde_json::to_string(&w.nation(N::Iraq).arsenal).unwrap();
    campaign_peace::tick(&mut w);
    assert_eq!(w.districts[&district], N::Iran);
    assert_eq!(
        inventory,
        serde_json::to_string(&w.nation(N::Iraq).arsenal).unwrap()
    );
    assert_eq!(w.campaign_peace.occupation[&district].days, 1);
    let once = spheres_sim::save(&w);
    campaign_peace::tick(&mut w);
    assert_eq!(once, spheres_sim::save(&w));
}
#[test]
fn old_rules_omit_operational_state_and_unknown_versions_are_refused() {
    let w = world_1990(GameRules::default());
    let save = spheres_sim::save(&w);
    let parsed: serde_json::Value = serde_json::from_str(&save).unwrap();
    assert!(parsed.get("campaign").is_none());
    assert!(parsed["rules"].get("operational_warfare").is_none());
    let mut bad = parsed;
    bad["rules"]["operational_warfare"] = serde_json::json!(2);
    assert!(spheres_sim::load(&bad.to_string()).is_err());
}
