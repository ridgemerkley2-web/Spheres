use spheres_sim::init::world_1990;
use spheres_sim::nations::NationId;
use spheres_sim::tick_month;
use spheres_sim::world::*;

/// Is conquest reachable at all after the influence merge?
#[test]
fn diag_conquest_scan() {
    let mut total = 0;
    let mut seeds_with = vec![];
    for seed in 0..30u64 {
        let mut w = world_1990(GameRules { seed, ..GameRules::default() });
        let mut alive: Vec<(NationId, f64)> =
            w.nations.iter().filter(|n| n.alive).map(|n| (n.id, n.population)).collect();
        let mut here = 0;
        for _ in 0..480 {
            tick_month(&mut w, &[]);
            let mut still = Vec::new();
            for (id, pop) in alive {
                if w.nation_opt(id).is_some_and(|n| n.alive) {
                    still.push((id, w.nation(id).population));
                } else if id != NationId::USSR && id != NationId::Yugoslavia {
                    println!("seed {:>2} {} {:?} annexed at {:.1}m", seed, w.year, id, pop);
                    here += 1;
                    total += 1;
                }
            }
            alive = still;
        }
        if here > 0 {
            seeds_with.push(seed);
        }
    }
    println!("conquests across 30 seeds x 40y: {} (seeds {:?})", total, seeds_with);
}
