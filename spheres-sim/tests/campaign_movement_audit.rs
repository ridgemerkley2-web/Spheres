//! A withdrawal has already left its origin; arrival concerns the remaining
//! corridor and destination, rather than recapture of abandoned ground.
use spheres_sim::{campaign::{self,Transfer},clock,init::world_1990,logistics,war};
use spheres_sim::world::{Belligerent,Conflict,GameRules,NationId as N,Objective};
#[test]
fn a_due_withdrawal_reaches_friendly_fallback_after_leaving_contested_origin() {
    let mut w=world_1990(GameRules {daily_simulation:true,military_operations:true,operational_warfare:1,
        resource_market:true,logistics_routes:true,physical_logistics:true,ai_aggression:0.0,..Default::default()});
    w.player=Some(N::Germany);for n in &mut w.nations {n.political_capital=0.0;}
    w.conflicts.push(Conflict{id:1,theatre:war::theatre_between(&w,N::Germany,N::France),
        side_a:vec![N::Germany],side_b:vec![N::France],
        posture:vec![Belligerent::new(N::Germany,8,Objective::Hold),Belligerent::new(N::France,8,Objective::Hold)],
        control:0.0,months:0,quiet_months:0,frozen_since:None,start_year:1990,start_month:1,
        origin_attacker:N::France,invasion_declared:true,
        front:std::collections::BTreeMap::from([("DE-BE".into(),0.0)]),pockets:vec![],aim:None});
    w.campaign.initialized=true;
    let today=clock::absolute_day(&w);
    w.campaign.transfers.push(Transfer {nation:N::Germany,conflict:Some(1),district:Some("DE-BB".into()),
        strength:1.0,cohesion:0.31,route:vec!["DE-BE".into(),"DE-BB".into()],arrives_day:today,departed_day:today-2});
    assert_eq!(spheres_sim::control::controller(&w,"DE-BE"),None);
    assert_eq!(spheres_sim::control::controller(&w,"DE-BB"),Some(N::Germany));
    logistics::begin_month(&mut w);war::tick(&mut w);
    assert!(!w.campaign.transfers.iter().any(|t|t.nation==N::Germany && t.arrives_day<=today
        && t.route.first().is_some_and(|d|d=="DE-BE")),"retreat cannot require friendly control of ground already abandoned");
    assert!((campaign::accounted_force(&w,N::Germany)-w.nation(N::Germany).mil_strength).abs()<1e-7);
}
