//! Independent integration invariants found during the war supply audit.
use spheres_sim::world::{Belligerent, Conflict, GameRules, NationId as N, Objective, WorldState};
use spheres_sim::{
    apply_command,
    campaign_peace::{self, PeaceOrder, Terms},
    init::world_1990,
    resources, war, Command,
};

fn world() -> WorldState {
    let mut w = world_1990(GameRules {
        daily_simulation: true,
        military_operations: true,
        operational_warfare: 1,
        resource_market: true,
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
        months: 0,
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
fn net_cash(w: &WorldState, n: N) -> f64 {
    let x = w.nation(n);
    x.treasury_bn
        .unwrap_or_else(|| resources::market_cash_bn(w, n))
        - x.debt_bn.unwrap_or(x.debt_gdp * x.gdp)
}
#[test]
fn debt_free_closed_books_recipient_keeps_every_reparation_dollar() {
    let mut w = world();
    {
        let n = w.nation_mut(N::Iran);
        n.treasury_bn = Some(100.0);
        n.debt_bn = Some(0.0);
        n.debt_gdp = 0.0;
    }
    {
        let n = w.nation_mut(N::Iraq);
        n.treasury_bn = None;
        n.debt_bn = None;
        n.debt_gdp = 0.0;
    }
    let before = net_cash(&w, N::Iraq) + net_cash(&w, N::Iran);
    let expected = w.nation(N::Iran).gdp * 0.01;
    let received = net_cash(&w, N::Iraq);
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
        (net_cash(&w, N::Iraq) - received - expected).abs() < 1e-8,
        "closed-book debt floor cannot destroy reparations"
    );
    assert!((net_cash(&w, N::Iraq) + net_cash(&w, N::Iran) - before).abs() < 1e-8);
}
#[test]
fn changing_coalition_sides_requires_fresh_consent_even_with_same_nations() {
    let mut w = world();
    let c = w.conflict_mut(1).unwrap();
    c.side_a.push(N::Syria);
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
        N::Syria,
        PeaceOrder::Respond {
            offer: 1,
            accept: true,
        },
    )
    .unwrap();
    let c = w.conflict_mut(1).unwrap();
    c.side_a.retain(|n| *n != N::Syria);
    c.side_b.push(N::Syria);
    let before = spheres_sim::save(&w);
    assert!(
        command(
            &mut w,
            N::Iran,
            PeaceOrder::Respond {
                offer: 1,
                accept: true
            }
        )
        .is_err(),
        "same membership does not preserve changed coalition consent"
    );
    assert_eq!(spheres_sim::save(&w), before);
}
#[test]
fn changing_the_principal_requires_fresh_terms_even_with_unchanged_coalitions() {
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
            terms: Terms::Reparations { share_bp: 100 },
        },
    )
    .unwrap();
    command(
        &mut w,
        N::Syria,
        PeaceOrder::Respond {
            offer: 1,
            accept: true,
        },
    )
    .unwrap();
    w.conflict_mut(1).unwrap().side_b.swap(0, 1);
    let before = spheres_sim::save(&w);
    assert!(
        command(
            &mut w,
            N::Iran,
            PeaceOrder::Respond {
                offer: 1,
                accept: true
            }
        )
        .is_err(),
        "a different principal cannot inherit the old payer's terms"
    );
    assert_eq!(spheres_sim::save(&w), before);
}
#[test]
fn occupation_belongs_to_its_actual_front_not_first_matching_war() {
    let mut a = world();
    let district = a
        .districts
        .iter()
        .find(|(_, n)| **n == N::Iran)
        .unwrap()
        .0
        .clone();
    a.conflicts[0].id = 10;
    let mut second = a.conflicts[0].clone();
    second.id = 20;
    second.front.insert(district.clone(), 1.0);
    a.conflicts.push(second);
    let mut b = a.clone();
    b.conflicts.reverse();
    campaign_peace::tick(&mut a);
    campaign_peace::tick(&mut b);
    assert_eq!(a.campaign_peace.occupation[&district].conflict, 20);
    assert_eq!(b.campaign_peace.occupation[&district].conflict, 20);
    assert_eq!(
        a.campaign_peace.occupation[&district].coverage,
        b.campaign_peace.occupation[&district].coverage
    );
}
