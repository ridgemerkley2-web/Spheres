//! Bounded measurement of the real Materials-enabled daily world. Timings live
//! only in this observer, never in WorldState or economic decisions. The copied
//! no-command daily sequence is verified against ordinary tick_day afterwards.
//! Usage: materials_profile [DAYS=40] [SEED=42] [REPORT.json]
use spheres_sim::{
    clock, init::world_1990, production, programs, province_economy, save,
    starting_industry, tick_day, world::{GameRules, WorldState}, SYSTEMS,
};
use std::{collections::BTreeMap, time::Instant};

#[derive(Default, serde::Serialize)]
struct Timing {
    calls: u64,
    seconds: f64,
    maximum_call_seconds: f64,
}
fn timed<T>(totals: &mut BTreeMap<String, Timing>, name: &str, operation: impl FnOnce() -> T) -> T {
    let start = Instant::now();
    let result = operation();
    let seconds = start.elapsed().as_secs_f64();
    let entry = totals.entry(name.into()).or_default();
    entry.calls += 1;
    entry.seconds += seconds;
    entry.maximum_call_seconds = entry.maximum_call_seconds.max(seconds);
    result
}
fn profiled_tick(w: &mut WorldState, totals: &mut BTreeMap<String, Timing>) -> Vec<String> {
    // Exact no-command daily arm of spheres_sim::tick_day, including the
    // monthly headline clearing and returned daily headline allocation.
    assert!(clock::is_daily(w));
    timed(totals, "headline_boundary", || {
        if w.day <= 1 { w.headlines.clear(); }
    });
    let before = w.headlines.len();
    timed(totals, "reindex", || w.reindex());
    // Commands are [] in both measured and control worlds.
    timed(totals, "province_begin", || province_economy::begin_day(w));
    timed(totals, "program_begin", || programs::begin_day(w));
    timed(totals, "construction", || production::tick_day(w));
    for (name, system) in SYSTEMS { timed(totals, name, || system(w)); }
    timed(totals, "program_finish", || programs::finish_day(w));
    timed(totals, "province_finish", || province_economy::finish_day(w));
    timed(totals, "calendar", || clock::advance_date(w));
    timed(totals, "headline_result", || w.headlines[before..].to_vec())
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let days = args.get(1).map(|s| s.parse::<u32>().expect("DAYS must be an integer")).unwrap_or(40);
    let seed = args.get(2).map(|s| s.parse::<u64>().expect("SEED must be an integer")).unwrap_or(42);
    assert!(days > 0);
    let mut world = world_1990(GameRules {
        seed, daily_simulation: true, economic_competition: true, production_system: true,
        resource_market: true, manufacturing_system: true, physical_logistics: true,
        logistics_routes: true, ..GameRules::default()
    });
    assert!(world.player.is_none());
    assert_eq!(world.nations.iter().filter(|n| n.alive).count(), 137);
    assert!(world.production.is_empty() && world.resources.is_empty());
    starting_industry::enable_new_world(&mut world).unwrap();
    province_economy::enable(&mut world);
    let mut control = world.clone();
    let mut totals = BTreeMap::new();
    let mut headlines = Vec::new();
    let started = Instant::now();
    for day in 1..=days {
        headlines.push(profiled_tick(&mut world, &mut totals));
        if day % 10 == 0 || day == days {
            eprintln!("profile seed={seed} day={day}/{days} date={} elapsed_s={:.3}", world.date_str(), started.elapsed().as_secs_f64());
        }
    }
    let wall_seconds = started.elapsed().as_secs_f64();
    let control_started = Instant::now();
    for expected in &headlines { assert_eq!(*expected, tick_day(&mut control, &[]), "daily headline result differs"); }
    let control_seconds = control_started.elapsed().as_secs_f64();
    assert_eq!(save(&world), save(&control), "Instrumented sequence must exactly equal ordinary tick_day");
    assert_eq!(world.rng.state, control.rng.state);
    let accounted: f64 = totals.values().map(|t| t.seconds).sum();
    let mut ranking = totals.iter().map(|(name, t)| serde_json::json!({
        "phase": name, "seconds": t.seconds, "percent_of_measured": t.seconds / accounted * 100.0,
        "mean_ms_per_day": t.seconds * 1000.0 / days as f64,
        "maximum_call_ms": t.maximum_call_seconds * 1000.0, "calls": t.calls,
    })).collect::<Vec<_>>();
    ranking.sort_by(|a,b| b["seconds"].as_f64().unwrap().total_cmp(&a["seconds"].as_f64().unwrap()));
    let report = serde_json::json!({
        "days": days, "seed": seed, "initial_living_countries": 137, "date": world.date_str(),
        "wall_seconds": wall_seconds, "measured_seconds": accounted, "ordinary_control_seconds": control_seconds,
        "mean_wall_ms_per_day": wall_seconds * 1000.0 / days as f64,
        "final_save_and_rng_match_ordinary_tick_day": true, "daily_headlines_match": true,
        "final_hash": format!("{:016x}", spheres_sim::state_hash(&world)),
        "basis": "Unmodified 1990 daily Economic Competition world with inherited industry. No gifts, forced Materials adoption or overridden wars. One bounded measured pass, not a statistically calibrated performance threshold.",
        "ranking": ranking,
    });
    let json = serde_json::to_string_pretty(&report).unwrap();
    if let Some(path) = args.get(3) { std::fs::write(path, &json).expect("write profile artifact"); }
    println!("{json}");
}
