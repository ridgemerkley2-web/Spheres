use spheres_sim::{apply_command,campaign_aims::{self,Aim},clock,init::world_1990,load,save,tech,Command};
use spheres_sim::world::{GameRules,NationId as N,WorldState};
fn fixture()->WorldState {
    let mut w=world_1990(GameRules{daily_simulation:true,..GameRules::default()});
    w.player=Some(N::USA);w.conflicts.clear();w
}
fn choose(w:&mut WorldState,aim:Aim) {apply_command(w,&Command::ChooseCampaignAim{nation:N::USA,aim}).unwrap();}
fn observe_days(w:&mut WorldState,days:u32) {for _ in 0..days {campaign_aims::tick(w);clock::advance_date(w);}}

#[test]
fn optional_aim_views_and_absent_ticks_are_pure_and_legacy_safe() {
    let mut w=world_1990(GameRules::default());let before=save(&w);
    let view=campaign_aims::view(&w,N::USA);
    assert_eq!(view.offers.len(),6);assert!(view.offers.iter().all(|o|o.unavailable.is_some()));
    campaign_aims::tick(&mut w);assert_eq!(save(&w),before);
    assert!(!before.contains("\"campaign_aims\""));
    w.player=Some(N::USA);let before=save(&w);
    assert!(apply_command(&mut w,&Command::ChooseCampaignAim{nation:N::USA,aim:Aim::Science}).is_err());
    assert_eq!(save(&w),before);
}

#[test]
fn prosperity_requires_fixed_real_per_person_gain_and_sustained_conditions() {
    let mut w=fixture();choose(&mut w,Aim::Prosperity);
    let target=w.campaign_aims.active.as_ref().unwrap().target;
    let n=w.nation_mut(N::USA);n.gdp*=1.21;n.population*=1.21;
    assert!(!campaign_aims::view(&w,N::USA).evaluation.unwrap().eligible,"population growth is not per-person prosperity");
    w.nation_mut(N::USA).population/=1.21;
    let n=w.nation_mut(N::USA);n.inflation=0.02;n.stability=70.0;
    observe_days(&mut w,89);
    assert!(w.campaign_aims.active.as_ref().unwrap().completed_day.is_none());
    // A new view cannot move the frozen target to the now-larger economy.
    assert_eq!(campaign_aims::view(&w,N::USA).active.unwrap().target,target);
    w.nation_mut(N::USA).inflation=0.11;
    observe_days(&mut w,1);assert_eq!(w.campaign_aims.active.as_ref().unwrap().held_days,0);
    w.nation_mut(N::USA).inflation=0.02;
    observe_days(&mut w,90);assert!(w.campaign_aims.active.as_ref().unwrap().completed_day.is_some());
    let rng=w.rng.clone();let n=w.nation(N::USA).clone();
    apply_command(&mut w,&Command::ContinueSandbox{nation:N::USA}).unwrap();
    assert!(w.campaign_aims.active.is_none());assert_eq!(w.campaign_aims.history[0].outcome,"achieved");
    assert_eq!(w.rng,rng);assert_eq!(serde_json::to_value(w.nation(N::USA)).unwrap(),serde_json::to_value(&n).unwrap(),"an achievement gives no invented reward");
}

#[test]
fn stability_counts_each_real_date_once_and_roundtrips_mid_streak() {
    let mut w=fixture();choose(&mut w,Aim::Stability);
    w.nation_mut(N::USA).stability=95.0;w.nation_mut(N::USA).inflation=0.02;
    campaign_aims::tick(&mut w);campaign_aims::tick(&mut w);
    assert_eq!(w.campaign_aims.active.as_ref().unwrap().held_days,1);
    clock::advance_date(&mut w);observe_days(&mut w,99);
    let mut resumed=load(&save(&w)).unwrap();
    for game in [&mut w,&mut resumed] {observe_days(game,265);}
    assert_eq!(save(&w),save(&resumed));
    assert!(w.campaign_aims.active.as_ref().unwrap().completed_day.is_some());
    let count=w.headlines.iter().filter(|h|h.contains("CAMPAIGN AIM ACHIEVED")).count();
    observe_days(&mut w,10);
    assert_eq!(w.headlines.iter().filter(|h|h.contains("CAMPAIGN AIM ACHIEVED")).count(),count);
}

#[test]
fn science_counts_new_acquisitions_and_goal_changes_require_explicit_sandbox() {
    let mut w=fixture();choose(&mut w,Aim::Science);
    campaign_aims::tick(&mut w);assert_eq!(w.campaign_aims.active.as_ref().unwrap().held_days,0);
    let before=save(&w);
    assert!(apply_command(&mut w,&Command::ChooseCampaignAim{nation:N::USA,aim:Aim::Domination}).is_err());
    assert_eq!(save(&w),before);
    let add:Vec<u16>=(0..tech::registry().len() as u16).filter(|i|!w.nation(N::USA).tech.known.contains(i)).take(8).collect();
    w.nation_mut(N::USA).tech.known.extend(add);w.nation_mut(N::USA).tech.known.sort();
    clock::advance_date(&mut w);campaign_aims::tick(&mut w);
    assert!(w.campaign_aims.active.as_ref().unwrap().completed_day.is_some());
    assert!(!spheres_sim::domination::status(&w,N::USA).victory,"scientific success cannot subjugate another country");
}

#[test]
fn supply_does_not_reward_switches_off_or_zero_demand() {
    let mut w=fixture();let before=save(&w);
    assert!(apply_command(&mut w,&Command::ChooseCampaignAim{nation:N::USA,aim:Aim::Supply}).is_err());
    assert_eq!(before,save(&w));
    w.rules.resource_market=true;w.rules.resource_gates=true;
    choose(&mut w,Aim::Supply);
    w.nation_mut(N::USA).mil_spend_gdp=0.0;
    assert!(spheres_sim::resources::tick_draw(&w,N::USA).iter().all(|x|*x<=1e-9));
    observe_days(&mut w,100);
    assert_eq!(w.campaign_aims.active.as_ref().unwrap().held_days,0);
    assert!(campaign_aims::view(&w,N::USA).evaluation.unwrap().blockers.iter().any(|s|s.contains("positive scheduled")));
}

#[test]
fn diplomacy_counts_distinct_trusted_relationships_and_loses_sanctioned_partners() {
    let mut w=fixture();w.statecraft.pacts.clear();w.statecraft.trade.clear();w.domination.compacts.clear();
    choose(&mut w,Aim::DiplomaticLeadership);
    for partner in [N::Canada,N::Japan] {
        w.set_relation(N::USA,partner,80.0);
        let (a,b)=if N::USA<partner {(N::USA,partner)} else {(partner,N::USA)};
        w.statecraft.pacts.push(spheres_sim::world::Pact{a,b,since_year:1990,since_month:1});
    }
    observe_days(&mut w,89);
    assert_eq!(w.campaign_aims.active.as_ref().unwrap().held_days,89);
    w.sanctions.push((N::USA,N::Japan));observe_days(&mut w,1);
    assert_eq!(w.campaign_aims.active.as_ref().unwrap().held_days,0);
    w.sanctions.clear();observe_days(&mut w,90);
    assert!(w.campaign_aims.active.as_ref().unwrap().completed_day.is_some());
    assert!(w.domination.subjects.is_empty(),"peaceful leadership never fabricates subordination");
}

#[test]
fn actual_daily_tick_observes_after_settlement_and_save_resume_matches() {
    let mut w=fixture();choose(&mut w,Aim::Stability);
    w.nation_mut(N::USA).stability=95.0;w.nation_mut(N::USA).inflation=0.02;
    let mut resumed=load(&save(&w)).unwrap();
    spheres_sim::tick_day(&mut w,&[]);spheres_sim::tick_day(&mut resumed,&[]);
    assert_eq!(save(&w),save(&resumed));
    let goal=w.campaign_aims.active.as_ref().unwrap();
    assert_eq!(goal.last_day,Some(clock::absolute_day(&w)-1));
    assert_eq!(goal.held_days,1);
}
