//! Read-only: roster-median discontent vs 12-month mean discontent of named
//! quiet high-leverage electoral countries (Claude scratch diagnostic).
use spheres_sim::blocs;
use spheres_sim::government;
use spheres_sim::init::world_1990;
use spheres_sim::tick_month;
use spheres_sim::world::*;
use std::collections::BTreeMap;
fn median(v: &mut Vec<f64>) -> f64 { v.sort_by(|a,b| a.partial_cmp(b).unwrap()); let n=v.len(); if n%2==1 {v[n/2]} else {(v[n/2-1]+v[n/2])/2.0} }
fn main() {
    let watch = ["Pakistan","Thailand","Ghana","Gabon","Paraguay","Lesotho","Cameroon","Central African Republic","El Salvador","Honduras","Fiji","Turkey","South Korea","Philippines","Peru"];
    let mut leads: BTreeMap<&str, Vec<f64>> = BTreeMap::new();
    for seed in 0..12u64 {
        let mut w = world_1990(GameRules { seed, ideology_blocs: true, ideology_takeover: true, ..GameRules::default() });
        let mut hist: BTreeMap<String, Vec<f64>> = BTreeMap::new();
        for _m in 0..252 {
            let mut roster = vec![];
            for n in &w.nations { if n.alive && government::state(&w, n.id).is_some() {
                let d = blocs::discontent(&w, n.id); roster.push(d);
                let h = hist.entry(n.id.name().to_string()).or_default(); h.push(d); if h.len() > 12 { h.remove(0); } } }
            // roster median of 12-month means (as A10 computes it)
            let mut means: Vec<f64> = hist.values().filter(|h| h.len() >= 6).map(|h| h.iter().sum::<f64>() / h.len() as f64).collect();
            if means.len() > 10 {
                let rm = median(&mut means);
                for c in watch { if let Some(h) = hist.get(c) { if h.len() >= 6 {
                    let n = w.nations.iter().find(|n| n.id.name() == c).unwrap();
                    if government::is_electoral(&w, n.id) {
                        leads.entry(c).or_default().push(h.iter().sum::<f64>() / h.len() as f64 - rm); } } } }
            }
            let _ = tick_month(&mut w, &[]);
        }
    }
    for (c, mut v) in leads { let n=v.len(); let med = median(&mut v); println!("{c:26} electoral-months {n:5} lead(12m mean D - roster median) min {:+.3} median {med:+.3} max {:+.3}", v[0], v[n-1]); }
}
