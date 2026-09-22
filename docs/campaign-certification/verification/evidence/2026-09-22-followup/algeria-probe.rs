use serde_json::{json, Value};
use spheres_sim::{init::world_1990, world::{GameRules, WorldState}, nations::NationId, state_hash, tick_month, save};

fn overlay(w: &mut WorldState) {
    assert_eq!((w.year,w.month,w.day), (1990,1,1));
    let g=w.governments.states.iter_mut().find(|g|g.nation==NationId::Algeria).unwrap();
    assert_eq!(g.months_in_office, 0);
    assert_eq!(g.office_month_fraction, 0.0);
    for (id, share) in &mut g.seats { *share=if id=="dz_fln" {1.0} else {0.0}; }
    g.coalition=vec!["dz_fln".to_string()];
}
fn diff(a:&Value,b:&Value,path:&str,out:&mut Vec<Value>) {
    if a==b {return;}
    match (a,b) {
        (Value::Object(x),Value::Object(y)) if x.keys().eq(y.keys()) => {
            for (k,v) in x {diff(v,&y[k],&format!("{path}/{k}"),out);}
        },
        (Value::Array(x),Value::Array(y)) if x.len()==y.len() => {
            for (i,(v,z)) in x.iter().zip(y).enumerate() {diff(v,z,&format!("{path}/{i}"),out);}
        },
        _=>out.push(json!({"path":path,"old":a,"new":b})),
    }
}
fn run(w:&mut WorldState) {for _ in 0..240 {tick_month(w,&[]);}}
fn main() {
    let old=world_1990(GameRules::default());
    let old_start=state_hash(&old);
    assert_eq!(old_start,0xe26e4bf8d6c60066,"cached library is not the old canonical default startup");
    let mut new=old.clone(); overlay(&mut new);
    let new_start=state_hash(&new);
    let a:Value=serde_json::from_str(&save(&old)).unwrap();
    let b:Value=serde_json::from_str(&save(&new)).unwrap();
    let mut changes=vec![];diff(&a,&b,"",&mut changes);
    let index=old.governments.states.iter().position(|g|g.nation==NationId::Algeria).unwrap();
    let prefix=format!("/governments/states/{index}/");
    assert!(changes.iter().all(|d| {
        let p=d["path"].as_str().unwrap();
        p.starts_with(&format!("{prefix}seats/")) || p==format!("{prefix}coalition/0")
    }),"overlay changed fields outside Algeria seats and coalition: {changes:?}");
    println!("{}",json!({"phase":"startup","old_start":format!("{old_start:#018x}"),"new_start":format!("{new_start:#018x}"),"algeria_government_index":index,"all_json_field_differences":changes,"diff_scope_assertion":"passed"}));
    let mut control=old;run(&mut control);
    let old_end=state_hash(&control);
    assert_eq!(old_end,0x0cbd02497c30957c,"cached library does not reproduce the old canonical 240-month timeline");
    run(&mut new);let new_end=state_hash(&new);
    println!("{}",json!({"phase":"default_240_months","seed":1990,"old_hash":format!("{old_end:#018x}"),"new_hash":format!("{new_end:#018x}"),"date":[new.year,new.month,new.day],"old_control_assertion":"passed"}));
    for seed in 0..6u64 {
        let mut w=world_1990(GameRules{seed,..GameRules::default()}); overlay(&mut w);run(&mut w);
        println!("{}",json!({"phase":"seed_240_months","seed":seed,"new_hash":format!("{:#018x}",state_hash(&w)),"date":[w.year,w.month,w.day]}));
    }
    println!("{}",json!({"phase":"complete","default_control_checks":2,"seed_runs":6,"simulation_mode":"default headless monthly, no commands; only opening Algeria seat shares and coalition overlaid"}));
}
