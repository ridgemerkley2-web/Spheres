use spheres_sim::{*, world::*, init::world_1990, government::Bloc};
fn diff(a:&serde_json::Value,b:&serde_json::Value,path:String,out:&mut Vec<serde_json::Value>){
 if a==b{return}
 match(a,b){
  (serde_json::Value::Object(x),serde_json::Value::Object(y))=>{for(k,v)in x{diff(v,y.get(k).unwrap_or(&serde_json::Value::Null),format!("{path}/{k}"),out)}for(k,v)in y{if !x.contains_key(k){diff(&serde_json::Value::Null,v,format!("{path}/{k}"),out)}}},
  (serde_json::Value::Array(x),serde_json::Value::Array(y)) if x.len()==y.len()=>{for(i,(v,w))in x.iter().zip(y).enumerate(){diff(v,w,format!("{path}/{i}"),out)}},
  _=>out.push(serde_json::json!({"path":path,"monthly":a,"legacy_day_stepped":b}))
 }
}
fn restive(w:&mut WorldState,id:NationId){let n=w.nation_mut(id);n.stability=30.;n.inflation=0.18;n.political_capital=100.;if let Some(g)=w.governments.states.iter_mut().find(|g|g.nation==id){if g.movements.len()==5{let r=g.regime_bloc.unwrap();for e in &mut g.movements{e.1=if e.0==Bloc::Western{0.30}else if e.0==r{0.50}else{0.20/3.};}}}}
fn main(){
 let rules=GameRules{ideology_blocs:true,ideology_takeover:true,..Default::default()};let player=NationId::Indonesia;let target=NationId::Malaysia;
 let mut monthly=world_1990(rules);monthly.player=Some(player);monthly.nation_mut(player).political_capital=100.;let mut daily=monthly.clone();
 for m in 0..24usize{
  let cmds=match m{3=>vec![Command::CovertAction{sponsor:player,target,op:CovertOp::BackBloc(Bloc::Western)}],5=>vec![Command::BanParty{nation:player,party:"id_pdi".into()}],11=>{restive(&mut monthly,player);restive(&mut daily,player);vec![Command::ConveneRoundTable{nation:player}]},_=>vec![]};
  let mr=tick_month(&mut monthly,&cmds);let days=world::days_in_month(daily.year,daily.month);let issue=match m{3=>9,5=>19,11=>days-1,_=>0};let mut dr=vec![];
  for d in 0..days{dr.extend(tick_day(&mut daily,if d==issue{&cmds[..]}else{&[]}));if m==13&&d==14{daily=load(&save(&daily)).unwrap()}}
  assert_eq!(mr,dr);if state_hash(&monthly)!=state_hash(&daily){let mut out=vec![];diff(&serde_json::from_str(&save(&monthly)).unwrap(),&serde_json::from_str(&save(&daily)).unwrap(),String::new(),&mut out);println!("{}",serde_json::json!({"first_month":m,"date":monthly.date_str(),"differences":out}));return}
 }
 println!("No difference in24 months");
}
