use spheres_sim::{init::world_1990, world::GameRules, state_hash, tick_month};
fn main() {
    for seed in [1990,0,1,2,3,4,5] {
        let mut w=world_1990(GameRules{seed,..GameRules::default()});
        let start=state_hash(&w);
        for _ in 0..240 {tick_month(&mut w,&[]);}
        println!("{{\"seed\":{seed},\"startup_hash\":\"{start:#018x}\",\"months\":240,\"end_hash\":\"{:#018x}\",\"end_date\":[{},{},{}]}}",state_hash(&w),w.year,w.month,w.day);
    }
}
