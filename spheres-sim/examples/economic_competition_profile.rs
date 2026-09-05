//! Bounded instrument of the registered daily systems. Timing is observational
//! and outside the simulation; no threshold or decision reads wall-clock time.
use spheres_sim::{
    clock,
    init::world_1990,
    production, programs, province_economy,
    world::{GameRules, WorldState},
    SYSTEMS,
};
use std::{collections::BTreeMap, time::Instant};
fn timed<F: FnOnce(&mut WorldState)>(
    w: &mut WorldState,
    times: &mut BTreeMap<String, f64>,
    name: &str,
    f: F,
) {
    let at = Instant::now();
    f(w);
    *times.entry(name.into()).or_default() += at.elapsed().as_secs_f64();
}
fn main() {
    let days = std::env::args()
        .nth(1)
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(3);
    let mut w = world_1990(GameRules {
        seed: 42,
        daily_simulation: true,
        economic_competition: true,
        production_system: true,
        resource_market: true,
        manufacturing_system: true,
        physical_logistics: true,
        logistics_routes: true,
        ..GameRules::default()
    });
    province_economy::enable(&mut w);
    let mut totals = BTreeMap::new();
    for _ in 0..days {
        let mut times = BTreeMap::new();
        timed(&mut w, &mut times, "reindex", |w| w.reindex());
        timed(
            &mut w,
            &mut times,
            "province_begin",
            province_economy::begin_day,
        );
        timed(&mut w, &mut times, "program_begin", programs::begin_day);
        timed(&mut w, &mut times, "construction", production::tick_day);
        for (name, system) in SYSTEMS {
            timed(&mut w, &mut times, name, *system);
        }
        timed(&mut w, &mut times, "program_finish", programs::finish_day);
        timed(
            &mut w,
            &mut times,
            "province_finish",
            province_economy::finish_day,
        );
        clock::advance_date(&mut w);
        for (name, time) in &times {
            *totals.entry(name.clone()).or_insert(0.0) += time;
        }
        eprintln!(
            "{} {}",
            w.date_str(),
            serde_json::to_string(&times).unwrap()
        );
    }
    println!("{}", serde_json::to_string_pretty(&totals).unwrap());
}
