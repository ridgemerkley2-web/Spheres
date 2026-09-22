use spheres_sim::{init::world_1990, world::{GameRules,WorldState}, nations::NationId, government::{self,Bloc,Pillar}, blocs, tick_month, data};
fn snapshot(w:&WorldState,seed:u64,month:usize) {
 let world_income=w.nations.iter().filter(|n|n.alive).map(|n|n.gdp).sum::<f64>()*1000.0/w.nations.iter().filter(|n|n.alive).map(|n|n.population).sum::<f64>();
 println!("STATE seed={seed} month={month} world_income={world_income:.0}");
 for id in [NationId::Poland,NationId::Hungary,NationId::Bulgaria,NationId::Albania,NationId::Mongolia,NationId::Lithuania,NationId::Afghanistan,NationId::Zambia,NationId::Zaire,NationId::Cambodia,NationId::Thailand,NationId::Pakistan] {
  let Some(n)=w.nation_opt(id) else {continue}; let Some(g)=government::state(w,id) else {continue};
  let spend=data::army_personnel_1990(id).map(|p|n.mil_spend_gdp*n.gdp*1e9/p);
  println!("{:?} alive={} electoral={} ruling={:?} coalition={:?} support={:?} movements={:?} auth={:.3} stab={:.1} inflation={:.3} growth={:.3} war={:.3} D={:.3} army={:.3} party={:.3} weakest={:?} challenger={:?} round_table={} militia_can_win={} mil_share={:.4} spend_per_member={:?}",id,n.alive,government::is_electoral(w,id),blocs::ruling_bloc(w,id),g.coalition,g.support,g.movements,n.authoritarianism,n.stability,n.inflation,n.growth_last,n.war_exhaustion,blocs::discontent(w,id),g.loyalty(Pillar::Army),g.loyalty(Pillar::Party),g.weakest_armed(),blocs::strongest_challenger(w,id),blocs::round_table_armed(w,id),blocs::bloc_can_win(w,id,Bloc::Islamist),n.mil_spend_gdp,spend);
 }
}
fn main(){for seed in [0,37] {let mut w=world_1990(GameRules{seed,ideology_blocs:true,ideology_takeover:true,..GameRules::default()}); for m in 0..=132 {if [0,24,60,84,132].contains(&m){snapshot(&w,seed,m);} if m<132{tick_month(&mut w,&[]);}}}}
