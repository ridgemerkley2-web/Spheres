//! Automatic coalition membership must obey the same sphere gate as a player
//! joining a conflict. The opposing bloc member need not be its original opener.
use spheres_sim::{commitment, domination, init::world_1990, sovereignty, statecraft, state_hash, war,
    world::{GameRules,NationId,Objective,Pact,WorldState}};

fn prepared(usa_attacks:bool)->(WorldState,u32) {
    let mut w=world_1990(GameRules{daily_simulation:true,economic_competition:true,..GameRules::default()});
    domination::subjugate(&mut w,NationId::USA,NationId::UK);
    w.shift_relation(NationId::UK,NationId::Kuwait,100.0);
    w.statecraft.pacts.clear();
    let theatre=war::theatre_between(&w,NationId::Iraq,NationId::Kuwait);
    let conflict=commitment::open_conflict(&mut w,NationId::Iraq,NationId::Kuwait,theatre).unwrap();
    commitment::join_conflict(&mut w,NationId::USA,conflict,usa_attacks,Objective::Deny).unwrap();
    assert!(sovereignty::hostility_blocked(&w,NationId::USA,NationId::UK));
    (w,conflict)
}

#[test]
fn spontaneous_intervention_cannot_oppose_a_later_joining_sphere_member() {
    let (mut w,conflict)=prepared(true);
    war::invasion_begins(&mut w,conflict,NationId::Iraq);
    let c=w.conflict(conflict).unwrap();
    assert_eq!(c.side_of(NationId::USA),Some(true));
    assert_eq!(c.side_of(NationId::UK),None,
        "UK may not enter opposite its own overlord merely because Iraq opened the quarrel");
    assert_eq!(domination::direct_overlord(&w,NationId::UK),Some(NationId::USA));
}

#[test]
fn a_guarantee_cannot_oppose_any_member_of_its_sphere_or_roll_loyalty() {
    let (mut w,conflict)=prepared(true);
    w.statecraft.pacts.push(Pact {a:NationId::UK.min(NationId::Kuwait),b:NationId::UK.max(NationId::Kuwait),since_year:1990,since_month:1});
    let mut c=w.conflict(conflict).unwrap().clone();
    let before=state_hash(&w);
    let refused=statecraft::call_the_guarantors(&mut w,&mut c,8);
    assert!(refused.contains(&NationId::UK),"An incompatible guarantee is explicitly declined");
    assert_eq!(c.side_of(NationId::UK),None);
    assert_eq!(state_hash(&w),before,
        "Sphere gate precedes RNG, reputation penalties and pact dissolution");
}

#[test]
fn joining_alongside_a_sphere_member_is_still_allowed() {
    let (mut w,conflict)=prepared(false);
    war::invasion_begins(&mut w,conflict,NationId::Iraq);
    let c=w.conflict(conflict).unwrap();
    assert_eq!(c.side_of(NationId::USA),Some(false));
    assert_eq!(c.side_of(NationId::UK),Some(false));
}
