//! Read-only A1 diagnostic (Claude scratch, not repository code).
//! Replicates bloc_census::run_seed's world exactly: both switches on,
//! GameRules::default otherwise, monthly clock, no player, no commands,
//! seeds 0..N (default 12), 252 months. Reads public state only; issues no
//! command and draws nothing itself.
use spheres_sim::blocs;
use spheres_sim::government::{self, Pillar};
use spheres_sim::init::world_1990;
use spheres_sim::tick_month;
use spheres_sim::world::*;
use std::collections::BTreeMap;

#[derive(Default, Clone)]
struct Cty {
    electoral_army_months: u32,
    crisis_months: u32,
    hostile_months: u32,
    both_months: u32,
    quiet_funded_months: u32,
    max_d: f64,
    min_eff: f64,
    max_penalty: f64,
    max_pressure: f64,
    leverage_first: Option<f64>,
    elcoups: u32,
    regime_months: u32,
}

fn leverage(w: &WorldState, id: NationId) -> Option<f64> {
    let g = government::state(w, id)?;
    if !g.pillars.iter().any(|(p, _)| *p == Pillar::Army) || !government::is_electoral(w, id) {
        return None;
    }
    Some(spheres_sim::army_authority::current_leverage(w, id)
        .unwrap_or_else(|| ((w.nation(id).authoritarianism - 0.20) / 0.40).clamp(0.0, 1.0)))
}

fn main() {
    let n: u64 = std::env::var("SEEDS").ok().and_then(|s| s.parse().ok()).unwrap_or(12);
    let months: usize = std::env::var("MONTHS").ok().and_then(|s| s.parse().ok()).unwrap_or(252);
    let mut pooled: BTreeMap<String, Cty> = BTreeMap::new();
    for seed in 0..n {
        let rules = GameRules { seed, ideology_blocs: true, ideology_takeover: true, ..GameRules::default() };
        let mut w = world_1990(rules);
        let mut per: BTreeMap<String, Cty> = BTreeMap::new();
        let mut coup_list: Vec<String> = vec![];
        let mut seen: BTreeMap<String, u32> = BTreeMap::new();
        for m in 0..months {
            // pre-tick readings per nation
            let mut pre: BTreeMap<String, (f64, f64, Option<f64>, f64, f64, u32, bool)> = BTreeMap::new();
            for i in 0..w.nations.len() {
                let nat = &w.nations[i];
                if !nat.alive { continue; }
                let id = nat.id;
                let name = id.name().to_string();
                let Some(g) = government::state(&w, id) else { continue; };
                let has_army = g.pillars.iter().any(|(p, _)| *p == Pillar::Army);
                let electoral = government::is_electoral(&w, id);
                let d = blocs::discontent(&w, id);
                let e = per.entry(name.clone()).or_insert_with(|| Cty { min_eff: 1.0, ..Default::default() });
                if !electoral { e.regime_months += 1; }
                if electoral && has_army {
                    let eff = blocs::effective_army_loyalty(&w, id);
                    let pen = government::army_civilian_confidence_penalty(&w, id);
                    let lev = leverage(&w, id);
                    e.electoral_army_months += 1;
                    if d >= government::ELECTORAL_COUP_DISCONTENT { e.crisis_months += 1; }
                    if eff < government::ELECTORAL_COUP_ARMY { e.hostile_months += 1; }
                    if d >= government::ELECTORAL_COUP_DISCONTENT && eff < government::ELECTORAL_COUP_ARMY { e.both_months += 1; }
                    if d < government::ELECTORAL_COUP_DISCONTENT && eff >= government::ELECTORAL_COUP_ARMY { e.quiet_funded_months += 1; }
                    e.max_d = e.max_d.max(d);
                    e.min_eff = e.min_eff.min(eff);
                    e.max_penalty = e.max_penalty.max(pen);
                    e.max_pressure = e.max_pressure.max(g.coup_pressure);
                    if e.leverage_first.is_none() { e.leverage_first = lev; }
                    pre.insert(name.clone(), (d, eff, lev, pen, g.coup_pressure,
                        government::electoral_coup_settled_months(&w, id), true));
                }
            }
            let news = tick_month(&mut w, &[]);
            for h in &news {
                if let Some(rest) = h.strip_prefix("COUP IN ") {
                    if !rest.contains("removes the elected government") { continue; }
                    let up = rest.split(':').next().unwrap_or("");
                    let name = w.nations.iter().map(|n| n.id.name()).find(|nm| nm.to_uppercase() == up).unwrap_or(up).to_string();
                    let k = seen.entry(name.clone()).or_insert(0);
                    *k += 1;
                    per.entry(name.clone()).or_insert_with(|| Cty { min_eff: 1.0, ..Default::default() }).elcoups += 1;
                    let p = pre.get(&name).cloned();
                    let (y, mo) = (1990 + (m / 12) as i32, (m % 12) + 1);
                    coup_list.push(match p {
                        Some((d, eff, lev, pen, pr, settled, _)) => format!(
                            "{name} {y}-{mo:02} #{k} preD {d:.3} preEff {eff:.3} lev {} pen {pen:.3} prePressure {pr:.3} settled {settled}",
                            lev.map_or("unknown".into(), |l| format!("{l:.3}"))),
                        None => format!("{name} {y}-{mo:02} #{k} (no pre-tick electoral-army row)"),
                    });
                }
            }
        }
        let mut counts: Vec<(String, u32)> = seen.into_iter().collect();
        counts.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        let total: u32 = counts.iter().map(|c| c.1).sum();
        let top3: u32 = counts.iter().take(3).map(|c| c.1).sum();
        println!("SEED {seed} elcoups {total} top3 {top3}/{total} = {:.4} distinct {} counts {:?}",
            if total > 0 { top3 as f64 / total as f64 } else { 0.0 }, counts.len(), counts);
        for c in &coup_list { println!("  COUP s{seed} {c}"); }
        for (k, v) in per {
            let p = pooled.entry(k).or_insert_with(|| Cty { min_eff: 1.0, ..Default::default() });
            p.electoral_army_months += v.electoral_army_months;
            p.crisis_months += v.crisis_months;
            p.hostile_months += v.hostile_months;
            p.both_months += v.both_months;
            p.quiet_funded_months += v.quiet_funded_months;
            p.max_d = p.max_d.max(v.max_d);
            p.min_eff = p.min_eff.min(v.min_eff);
            p.max_penalty = p.max_penalty.max(v.max_penalty);
            p.max_pressure = p.max_pressure.max(v.max_pressure);
            if p.leverage_first.is_none() { p.leverage_first = v.leverage_first; }
            p.elcoups += v.elcoups;
            p.regime_months += v.regime_months;
        }
    }
    println!("POOLED over {n} seeds x {months} months (country: elArmyMonths crisisD>=.25 hostileEff<.35 both quietFunded | maxD minEff maxPenalty maxPressure lev0 | elcoups regimeMonths)");
    for (k, v) in &pooled {
        println!("  CTY {k}: {} {} {} {} {} | {:.3} {:.3} {:.3} {:.3} {} | {} {}",
            v.electoral_army_months, v.crisis_months, v.hostile_months, v.both_months, v.quiet_funded_months,
            v.max_d, v.min_eff, v.max_penalty, v.max_pressure,
            v.leverage_first.map_or("unknown".into(), |l| format!("{l:.3}")), v.elcoups, v.regime_months);
    }
}
