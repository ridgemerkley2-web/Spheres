use spheres_sim::{apply_command, commitment, init::world_1990, statecraft, tick_day, war, Command};
use spheres_sim::world::{GameRules, NationId as N, Rng, WorldState};

fn fixture(player: N) -> WorldState {
    let mut w = world_1990(GameRules { seed: 1990, daily_simulation: true, ..GameRules::default() });
    w.player = Some(player);
    w.nation_mut(player).political_capital = 100.0;
    w
}

#[test]
fn player_sanction_survives_neutral_relations_and_duplicate_is_free() {
    let mut w = fixture(N::USA);
    w.set_relation(N::USA, N::Japan, 0.0);
    w.sanctions.clear();
    let command = Command::Sanction { imposer: N::USA, target: N::Japan };
    apply_command(&mut w, &command).unwrap();
    let held = w.nation(N::USA).political_capital;
    apply_command(&mut w, &command).unwrap();
    assert_eq!(w.nation(N::USA).political_capital, held, "an existing sanction is not another decision");
    tick_day(&mut w, &[]);
    assert!(w.is_sanctioning(N::USA, N::Japan), "automatic grievance recovery must not repeal the player's policy");
}

#[test]
fn peg_requires_an_explicit_exit_before_changing_rate() {
    let mut w = fixture(N::Iraq);
    w.nation_mut(N::Iraq).inflation = 0.30;
    apply_command(&mut w, &Command::EnactStratagem { nation: N::Iraq, id: "currency_peg".into() }).unwrap();
    let before = spheres_sim::save(&w);
    assert!(apply_command(&mut w, &Command::SetInterestRate { nation: N::Iraq, rate: 0.01 }).is_err());
    assert_eq!(spheres_sim::save(&w), before, "a refused rate change neither spends nor mutates");
}

#[test]
fn human_treaties_and_defense_decisions_wait_for_consent() {
    let mut w = fixture(N::USA);
    w.statecraft.pacts.clear();
    w.statecraft.aid.clear();
    w.set_relation(N::Kuwait, N::USA, 100.0);
    w.nation_mut(N::Kuwait).political_capital = 100.0;
    w.rng = Rng::new(1);
    apply_command(&mut w, &Command::ProposeAlliance { from: N::Kuwait, to: N::USA }).unwrap();
    assert!(!w.allied(N::Kuwait, N::USA), "an AI proposal is not human acceptance");
    // A pre-existing, explicitly accepted obligation still needs a human answer.
    let (a,b) = if N::USA < N::Kuwait {(N::USA,N::Kuwait)} else {(N::Kuwait,N::USA)};
    w.statecraft.pacts.push(spheres_sim::world::Pact { a,b,since_year:1990,since_month:1 });
    let th = war::theatre_between(&w, N::Iraq, N::Kuwait);
    commitment::open_conflict(&mut w, N::Iraq, N::Kuwait, th).unwrap();
    let mut c = w.conflicts.pop().unwrap();
    let rep = w.reputation(N::USA);
    w.rng = Rng::new(0);
    statecraft::call_the_guarantors(&mut w, &mut c, 8);
    assert!(!c.involves(N::USA));
    assert!(w.allied(N::USA, N::Kuwait));
    assert_eq!(w.reputation(N::USA),rep, "the player has not repudiated the guarantee");
}

#[test]
fn treaty_answers_revalidate_and_survive_save_resume_without_random_acceptance() {
    let mut w=fixture(N::USA);
    w.statecraft.pacts.clear(); w.statecraft.trade.clear();
    w.nation_mut(N::Japan).political_capital=100.0;
    let rng=w.rng.state;
    apply_command(&mut w,&Command::ProposeTrade{from:N::Japan,to:N::USA}).unwrap();
    assert_eq!(w.rng.state,rng);
    assert_eq!(w.trade_depth(N::Japan,N::USA),0.0);
    let id=w.agency.offers[0].id;
    let mut loaded=spheres_sim::load(&spheres_sim::save(&w)).unwrap();
    let answer=Command::RespondDiplomacy{nation:N::USA,offer:id,accept:true};
    for game in [&mut w,&mut loaded] {apply_command(game,&answer).unwrap();}
    assert_eq!(spheres_sim::save(&w),spheres_sim::save(&loaded));
    assert!(w.trade_depth(N::Japan,N::USA)>0.0);
    let before=spheres_sim::save(&w);
    assert!(apply_command(&mut w,&answer).is_err());
    assert_eq!(before,spheres_sim::save(&w));
    // A second country's pending terms cannot bypass a new sanction.
    apply_command(&mut w,&Command::ProposeTrade{from:N::UK,to:N::USA}).unwrap();
    let id=w.agency.offers[0].id;
    w.sanctions.push((N::USA,N::UK));
    let before=spheres_sim::save(&w);
    assert!(apply_command(&mut w,&Command::RespondDiplomacy{nation:N::USA,offer:id,accept:true}).is_err());
    assert_eq!(before,spheres_sim::save(&w));
}

#[test]
fn standing_policy_is_explicit_and_does_not_retroactively_answer_requests() {
    use spheres_sim::agency::{StandingPolicy,ResponsePolicy as P};
    let mut w=fixture(N::USA);w.statecraft.trade.clear();
    apply_command(&mut w,&Command::ProposeTrade{from:N::Japan,to:N::USA}).unwrap();
    apply_command(&mut w,&Command::SetDiplomaticPolicy{nation:N::USA,policy:StandingPolicy{trade_treaties:P::Accept,..Default::default()}}).unwrap();
    assert_eq!(w.trade_depth(N::Japan,N::USA),0.0);
    apply_command(&mut w,&Command::ProposeTrade{from:N::UK,to:N::USA}).unwrap();
    assert!(w.trade_depth(N::UK,N::USA)>0.0);
    assert_eq!(w.agency.offers.len(),1);
    assert_eq!(w.agency.offers[0].from,N::Japan);
}

fn defense_fixture()->(WorldState,u32) {
    let mut w=fixture(N::USA);w.statecraft.pacts.clear();
    let (a,b)=if N::USA<N::Kuwait {(N::USA,N::Kuwait)} else {(N::Kuwait,N::USA)};
    w.statecraft.pacts.push(spheres_sim::world::Pact{a,b,since_year:1990,since_month:1});
    let th=war::theatre_between(&w,N::Iraq,N::Kuwait);
    let cid=commitment::open_conflict(&mut w,N::Iraq,N::Kuwait,th).unwrap();
    let mut c=w.conflicts.pop().unwrap();
    statecraft::call_the_guarantors(&mut w,&mut c,2);
    w.conflicts.push(c);
    (w,cid)
}
#[test]
fn defense_acceptance_and_expiry_have_exactly_one_explicit_consequence() {
    let (mut w,cid)=defense_fixture();let mut late=w.clone();
    let id=w.agency.offers[0].id;
    let rep=w.reputation(N::USA);
    apply_command(&mut w,&Command::RespondDiplomacy{nation:N::USA,offer:id,accept:true}).unwrap();
    assert!(w.conflict(cid).unwrap().involves(N::USA));
    assert_eq!(w.reputation(N::USA),rep+5.0);
    assert!(w.agency.offers.is_empty());
    // Actual seven-day deadline, not seven months or a new RNG roll.
    for _ in 0..7 {spheres_sim::clock::advance_date(&mut late);}
    let mut resumed=spheres_sim::load(&spheres_sim::save(&late)).unwrap();
    for game in [&mut late,&mut resumed] {spheres_sim::agency::tick(game);spheres_sim::agency::tick(game);}
    assert!(!late.allied(N::USA,N::Kuwait));
    assert_eq!(late.reputation(N::USA),rep-25.0);
    assert_eq!(spheres_sim::save(&late),spheres_sim::save(&resumed));
    assert_eq!(late.agency.history.len(),1);
}

#[test]
fn reused_conflict_id_cannot_consume_an_old_defense_answer() {
    let (mut w,cid)=defense_fixture();let old=w.agency.offers[0].id;
    w.conflicts.clear();
    let th=war::theatre_between(&w,N::Iraq,N::Kuwait);
    let new=commitment::open_conflict(&mut w,N::Iraq,N::Kuwait,th).unwrap();
    assert_eq!(cid,new,"exercise the same-month reuse");
    assert!(w.agency.offers.is_empty());
    assert!(apply_command(&mut w,&Command::RespondDiplomacy{nation:N::USA,offer:old,accept:true}).is_err());
    assert!(!w.conflict(new).unwrap().involves(N::USA));
}

#[test]
fn automatic_bank_and_ai_peg_use_the_same_explicit_regime() {
    let mut w=fixture(N::USA);
    w.nation_mut(N::Iraq).inflation=0.30;w.nation_mut(N::Iraq).political_capital=100.0;
    apply_command(&mut w,&Command::EnactStratagem{nation:N::Iraq,id:"currency_peg".into()}).unwrap();
    spheres_sim::politics::tick(&mut w);
    assert_eq!(w.nation(N::Iraq).interest_rate,0.055);
    apply_command(&mut w,&Command::SetInterestRate{nation:N::USA,rate:0.11}).unwrap();
    assert!(w.player_set_rate);
    apply_command(&mut w,&Command::ResumeAutomaticBank{nation:N::USA}).unwrap();
    assert!(!w.player_set_rate);
    spheres_sim::politics::tick(&mut w);
    assert_ne!(w.nation(N::USA).interest_rate,0.11);
    w.player=Some(N::Iraq);let rate=w.nation(N::Iraq).interest_rate;
    let before=w.nation(N::Iraq).political_capital;
    apply_command(&mut w,&Command::BreakCurrencyPeg{nation:N::Iraq}).unwrap();
    assert!(spheres_sim::agency::pegged_rate(&w,N::Iraq).is_none());
    assert_eq!(w.nation(N::Iraq).interest_rate,rate);
    assert!((before-w.nation(N::Iraq).political_capital-12.0).abs()<1e-10);
    assert!(!w.player_set_rate);
    let saved=spheres_sim::save(&w);
    assert!(apply_command(&mut w,&Command::BreakCurrencyPeg{nation:N::Iraq}).is_err());
    assert_eq!(saved,spheres_sim::save(&w));
}

#[test]
fn untouched_legacy_world_and_read_only_decision_view_create_no_state() {
    let mut w=world_1990(GameRules::default());
    let before=spheres_sim::save(&w);
    let _=spheres_sim::agency::view(&w,N::USA);
    spheres_sim::agency::tick(&mut w);
    assert_eq!(before,spheres_sim::save(&w));
    assert!(!before.contains("\"agency\""));
}
