use spheres_sim::{init::world_1990,world::*,Command,apply_command,theatre::TheatreId};
fn main(){
 let mut w=world_1990(GameRules {seed:1,..Default::default()});
 for id in [NationId::USSR,NationId::Iraq]{w.nation_mut(id).political_capital=500.;}
 apply_command(&mut w,&Command::OpenConflict{opener:NationId::USSR,target:NationId::China,theatre:TheatreId::EastAsia}).unwrap();
 let a=w.conflict_between(NationId::USSR,NationId::China).unwrap().id;
 apply_command(&mut w,&Command::OpenConflict{opener:NationId::Iraq,target:NationId::Israel,theatre:TheatreId::Levant}).unwrap();
 let b=w.conflict_between(NationId::Iraq,NationId::Israel).unwrap().id;
 apply_command(&mut w,&Command::JoinConflict{conflict:b,nation:NationId::USSR,side_a:true,objective:Objective::Deny}).unwrap();
 assert_eq!(w.conflict(a).unwrap().posture.len(),2);assert_eq!(w.conflict(b).unwrap().posture.len(),3);
 w.nation_mut(NationId::USSR).separatism=1.;spheres_sim::politics::tick(&mut w);
 assert!(w.has_flag("ussr_dissolved"));assert!(!w.nation(NationId::USSR).alive);
 for id in [a,b]{assert!(w.conflict(id).unwrap().posture.iter().any(|p|p.nation==NationId::USSR));}
 println!("PASS real commands and dissolution: two_party={} three_party={} raw_participants=2,3",a,b);
}