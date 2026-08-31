//! AUDIT INSTRUMENT — not a calibration test, not asserted, `#[ignore]`d.
//!
//! Recomputes every term of `economy.rs`'s `growth_annual` from public state,
//! month by month, for the six `MATURE_1990` economies, and prints the
//! time-average of each. It samples the world BEFORE `tick_month`, and
//! `economy::tick` is the first entry in `SYSTEMS`, so the values it recomputes
//! are the values the sim used that month — except `noise`, which is a
//! zero-mean draw this cannot reproduce.
//!
//! The last column is the point: `applied` is the CAGR implied by compounding
//! the recomputed terms, `actual` is the CAGR of the GDP series. Their
//! difference is output arriving from somewhere that is not the growth
//! equation. Two things now land there by design and not by accident: the
//! aggregate trade level gain in `statecraft::trade_level_gain`, and the
//! capital-deepening level payment in `economy::tick`. Both are LEVELS, so
//! neither belongs in a table of annual rates; `off_equation_output` is where
//! they are read.
//!
//! Run:
//!   cargo test --release -p spheres-sim --test growth_decomposition -- --ignored --nocapture

use spheres_sim::init::world_1990;
use spheres_sim::world::{EconomySystem, GameRules, NationId, WorldState};
use spheres_sim::{tech, tick_month};

const MATURE: [NationId; 6] = [
    NationId::USA,
    NationId::Japan,
    NationId::Germany,
    NationId::France,
    NationId::UK,
    NationId::Italy,
];

const NTERM: usize = 12;
const NAMES: [&str; NTERM] = [
    "tfp", "invest", "catchup", "labour", "command", "demand", "bubble", "oil", "sanct", "war",
    "debt", "instab",
];

// `population_growth` is no longer a function of income alone: it is the
// nation's own transcribed 1990 rate moved by the transition, and the sim's own
// definition is used here rather than a copy so the two cannot drift apart.
use spheres_sim::economy::population_growth;

/// The terms of `growth_annual`, signed as they enter the sum.
fn terms(w: &WorldState, id: NationId) -> [f64; NTERM] {
    let n = w.nation(id);
    let oil_price = w.oil_price;
    let sanction_share = w.sanction_weight(id);
    let at_war = w.at_war(id);
    let export_share = w.oil_export_share(id);

    let invest = n.state_invest_gdp + n.priv_invest_gdp;
    let gdp_pc = n.gdp * 1000.0 / n.population;
    let dev = (gdp_pc / 24000.0).min(1.0);
    let intensity = spheres_sim::exact::powf((invest / 0.20).max(0.0), 0.55) * 0.20;
    let invest_effect = intensity * 0.080 * (1.0 - dev);
    let catchup = (1.0 - dev) * 0.020;
    let labour = population_growth(n) * 0.60;
    let command = if n.system == EconomySystem::Command {
        -(0.004 + 0.010 * (gdp_pc / 24000.0).min(1.0))
    } else {
        0.0
    };

    let real_rate = n.interest_rate - n.inflation;
    let mut demand_gap = (0.025 - real_rate) * 0.55;
    if demand_gap > 0.0 {
        let room_to_cut = (n.interest_rate / 0.04).clamp(0.0, 1.0);
        let willing_to_borrow = 1.0 - (-n.bubble).clamp(0.0, 1.0) * 0.75;
        demand_gap *= 0.25 + 0.75 * room_to_cut.min(willing_to_borrow);
    }
    // `MAX_DEMAND_GAP` in economy.rs — symmetric, and read off the bust side's
    // own bound. See the comment there.
    let demand_gap = demand_gap.clamp(-0.35, 0.35);
    // `money_works` in economy.rs gates the OUTPUT arm of the demand term and
    // only the output arm; the price arm keeps the ungated gap. This table is a
    // table of what reaches `growth_annual`, so it carries the gated one.
    let money_works = 1.0 / (1.0 + (n.inflation.max(0.0) / 0.40 - 1.0).max(0.0).powi(2));
    let demand_output = if demand_gap > 0.0 { demand_gap * money_works } else { demand_gap };

    let bubble_boost = if n.bubble > 0.0 {
        n.bubble * 0.012
    } else if n.bubble < 0.0 {
        n.bubble * 0.022
    } else {
        0.0
    };

    let oil_revenue_gdp = (n.oil_mbd * export_share * oil_price * 0.365 / n.gdp).min(2.0);
    let exposure = tech::energy_exposure(n);
    // The producer arm is still a RATE and is still wrong; see the block above
    // the producer arm in economy.rs. It stays in this table because this table
    // must recompute what the sim actually does, not what it ought to do.
    let oil_effect = if n.oil_mbd > 0.5 {
        (oil_price - 20.0) / 20.0 * oil_revenue_gdp * 0.5
    } else {
        -(oil_price - 20.0) / 20.0 * 0.006 * exposure
    };
    let embargo_drag = if n.oil_mbd > 0.5 {
        ((1.0 - export_share) * n.oil_mbd * oil_price * 0.365 / n.gdp * 0.30).min(0.12)
    } else {
        0.0
    };

    [
        n.tfp_trend,
        invest_effect,
        catchup,
        labour,
        command,
        demand_output,
        bubble_boost,
        // embargo_drag is an oil term and is folded in here
        oil_effect - embargo_drag,
        -(sanction_share * 0.020),
        -(if at_war { 0.020 + n.war_exhaustion * 0.03 } else { 0.0 }),
        -(if n.debt_gdp > 0.9 { (n.debt_gdp - 0.9) * 0.02 } else { 0.0 }),
        -(if n.stability < 40.0 { (40.0 - n.stability) * 0.0009 } else { 0.0 }),
    ]
}

#[test]
#[ignore]
fn growth_decomposition() {
    let years = 35usize;
    let seeds = 0..10u64;

    // acc[nation][term], plus applied-product and gdp endpoints
    let mut acc = vec![[0.0f64; NTERM]; MATURE.len()];
    let mut applied = vec![0.0f64; MATURE.len()]; // sum of ln(1+g/12)
    let mut ratio = vec![0.0f64; MATURE.len()]; // sum of ln(gdp_end/gdp_0)
    let mut months = vec![0.0f64; MATURE.len()];
    let mut alive_seeds = vec![0.0f64; MATURE.len()];
    let mut oilp = 0.0f64;
    let mut oilp_n = 0.0f64;

    for seed in seeds.clone() {
        let mut w = world_1990(GameRules { seed, ..GameRules::default() });
        let g0: Vec<f64> = MATURE.iter().map(|id| w.nation(*id).gdp).collect();
        for _ in 0..(12 * years) {
            oilp += w.oil_price;
            oilp_n += 1.0;
            for (k, id) in MATURE.iter().enumerate() {
                if !w.nation(*id).alive {
                    continue;
                }
                let t = terms(&w, *id);
                let s: f64 = t.iter().sum::<f64>().max(-0.95);
                for i in 0..NTERM {
                    acc[k][i] += t[i];
                }
                applied[k] += (1.0 + s / 12.0).ln();
                months[k] += 1.0;
            }
            tick_month(&mut w, &[]);
        }
        for (k, id) in MATURE.iter().enumerate() {
            if w.nation(*id).alive {
                ratio[k] += (w.nation(*id).gdp / g0[k]).ln();
                alive_seeds[k] += 1.0;
            }
        }
    }

    println!("\nmean oil price over the panel: {:.2}", oilp / oilp_n);
    println!(
        "\n{:>8} {}  {:>7} {:>7} {:>7}",
        "nation",
        NAMES.iter().map(|s| format!("{:>7}", s)).collect::<Vec<_>>().join(" "),
        "SUM",
        "applied",
        "actual"
    );
    for (k, id) in MATURE.iter().enumerate() {
        let m = months[k].max(1.0);
        let row: Vec<String> =
            (0..NTERM).map(|i| format!("{:>7.3}", acc[k][i] / m * 100.0)).collect();
        let sum: f64 = (0..NTERM).map(|i| acc[k][i] / m).sum::<f64>() * 100.0;
        // CAGR implied by compounding the recomputed terms
        let app = ((applied[k] / months[k] * 12.0).exp() - 1.0) * 100.0;
        let act = ((ratio[k] / alive_seeds[k].max(1.0) / years as f64).exp() - 1.0) * 100.0;
        println!(
            "{:>8} {}  {:>7.3} {:>7.3} {:>7.3}",
            format!("{:?}", id),
            row.join(" "),
            sum,
            app,
            act
        );
    }
    println!("\nall figures are annual percentage points, averaged over months and seeds.");
    println!("applied = CAGR from compounding the recomputed monthly rate.");
    println!("actual  = CAGR of the GDP series. actual - applied is off-equation output.\n");
}

/// What `oil_effect` is actually worth to each arm, and what the price does.
#[test]
#[ignore]
fn oil_effect_readout() {
    let years = 35usize;
    let mut hi = 0.0f64;
    let mut n_hi = 0.0f64;
    let mut prod_sum = 0.0f64;
    let mut prod_n = 0.0f64;
    let mut prod_peak = f64::NEG_INFINITY;
    let mut imp_sum = 0.0f64;
    let mut imp_n = 0.0f64;
    let mut price_hist: Vec<f64> = vec![];
    for seed in 0..10u64 {
        let mut w = world_1990(GameRules { seed, ..GameRules::default() });
        for _ in 0..(12 * years) {
            price_hist.push(w.oil_price);
            if w.oil_price > 20.0 {
                hi += 1.0;
            }
            n_hi += 1.0;
            for id in [NationId::Germany, NationId::Japan, NationId::France, NationId::Italy] {
                if !w.nation(id).alive {
                    continue;
                }
                let n = w.nation(id);
                imp_sum += -(w.oil_price - 20.0) / 20.0 * 0.006 * tech::energy_exposure(n);
                imp_n += 1.0;
            }
            for id in [NationId::SaudiArabia, NationId::Kuwait, NationId::Iraq, NationId::USA] {
                if !w.nation(id).alive {
                    continue;
                }
                let n = w.nation(id);
                let es = w.oil_export_share(id);
                let rev = (n.oil_mbd * es * w.oil_price * 0.365 / n.gdp).min(2.0);
                // THE OLD PRODUCER ARM, recomputed on the live board so the size
                // of what was removed stays readable. This was charged as an
                // annual RATE every month; it is now a LEVEL (`oil_windfall` in
                // economy.rs) and the same expression is that level.
                prod_sum += (w.oil_price - 20.0) / 20.0 * rev * 0.5;
                prod_n += 1.0;
                prod_peak = prod_peak.max((w.oil_price - 20.0) / 20.0 * rev * 0.5);
            }
            tick_month(&mut w, &[]);
        }
    }
    price_hist.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!(
        "\noil price: min {:.1} p10 {:.1} median {:.1} p90 {:.1} max {:.1}; above $20 in {:.1}% of months",
        price_hist[0],
        price_hist[price_hist.len() / 10],
        price_hist[price_hist.len() / 2],
        price_hist[price_hist.len() * 9 / 10],
        price_hist[price_hist.len() - 1],
        hi / n_hi * 100.0
    );
    println!(
        "mean oil_effect, mature importers (DE/JP/FR/IT): {:+.4} pp/yr",
        imp_sum / imp_n * 100.0
    );
    println!(
        "mean oil_effect, producers (SA/KW/IQ/US):        {:+.4} pp/yr, peak {:+.4} pp/yr",
        prod_sum / prod_n * 100.0,
        prod_peak * 100.0
    );
    println!(
        "(a RATE, charged every month the price is high. Reachable max is \
         (120-20)/20 * 2.0 * 0.5 = +500%/yr. See economy.rs.)\n"
    );
}

/// THE DETECTOR A GROWTH CEILING WOULD HAVE HIDDEN.
///
/// PLAN step 7 asks for a ceiling mirroring `WORST_ANNUAL_COLLAPSE`. The ruling
/// against it is written out beside that constant in economy.rs; the short form
/// is that a clamp would take a runaway term off every instrument in the suite
/// while leaving it in the arithmetic. This is what should exist instead: the
/// measurement itself.
///
/// It reads what the 21-seed sweep cannot see. That sweep's detectors are NaN,
/// clamps, debt above 6x and output above 100x — none of which catch a
/// petro-state compounding an extra 6pp/yr for thirty-five years, which is
/// exactly what PLAN step 7 records slipping past a direct challenge. This reads
/// the quantity that actually moves: the highest annual growth any nation
/// SUSTAINS over a full rolling year, which noise cannot fake and a runaway term
/// cannot avoid.
///
/// Deliberately a readout and not an assertion. A bar set today would be set
/// around a known-wrong producer arm, and that is a bar fitted to a bug.
///
/// `cargo test --release -p spheres-sim --test growth_decomposition peak_sustained_growth -- --ignored --nocapture`
#[test]
#[ignore]
fn peak_sustained_growth() {
    use std::collections::BTreeMap;
    let years = 35usize;
    // nation -> (best 12-month annualised growth, the year it happened)
    let mut best: BTreeMap<&'static str, (f64, usize)> = BTreeMap::new();
    for seed in 0..10u64 {
        let mut w = world_1990(GameRules { seed, ..GameRules::default() });
        let ids: Vec<NationId> = w.nations.iter().map(|n| n.id).collect();
        let mut window: Vec<Vec<f64>> = vec![vec![]; ids.len()];
        for month in 0..(12 * years) {
            for (k, id) in ids.iter().enumerate() {
                let n = w.nation(*id);
                window[k].push(if n.alive { n.gdp } else { f64::NAN });
                if window[k].len() > 13 {
                    window[k].remove(0);
                }
                if window[k].len() == 13 {
                    let (a, b) = (window[k][0], window[k][12]);
                    if a.is_finite() && b.is_finite() && a > 0.0 {
                        let g = b / a - 1.0;
                        let e = best.entry(id.code()).or_insert((f64::NEG_INFINITY, 0));
                        if g > e.0 {
                            *e = (g, 1990 + month / 12);
                        }
                    }
                }
            }
            tick_month(&mut w, &[]);
        }
    }
    let mut rows: Vec<(&'static str, f64, usize)> =
        best.iter().map(|(k, v)| (*k, v.0, v.1)).collect();
    rows.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    println!("\nworst twenty: highest growth any nation held for a FULL YEAR, ten seeds, 35 years");
    println!("{:>8} {:>14} {:>8}", "nation", "peak 12m", "year");
    for (code, g, y) in rows.iter().take(20) {
        println!("{:>8} {:>13.1}% {:>8}", code, g * 100.0, y);
    }
    let n_over = |bar: f64| rows.iter().filter(|r| r.1 > bar).count();
    println!(
        "\nnations exceeding: 25%/yr {}   50%/yr {}   100%/yr {}   (of {})",
        n_over(0.25),
        n_over(0.50),
        n_over(1.00),
        rows.len()
    );
    println!("a sustained full-year figure — monthly noise (+/-0.4pp) cannot produce one.\n");
}

/// The reachable range of `demand_gap`, given the clamps that actually bound
/// its inputs, and what it reaches in play.
#[test]
#[ignore]
fn demand_gap_readout() {
    let years = 35usize;
    let mut samples: Vec<f64> = vec![];
    let mut mature: Vec<f64> = vec![];
    // How often the symmetric bound actually binds, and how far past it the
    // unclamped term would have gone.
    let (mut bound_hits, mut bound_n, mut raw_max, mut raw_min) =
        (0.0f64, 0.0f64, f64::NEG_INFINITY, f64::INFINITY);
    for seed in 0..10u64 {
        let mut w = world_1990(GameRules { seed, ..GameRules::default() });
        for _ in 0..(12 * years) {
            let ids: Vec<NationId> =
                w.nations.iter().filter(|n| n.alive).map(|n| n.id).collect();
            for id in ids {
                let n = w.nation(id);
                let mut g = (0.025 - (n.interest_rate - n.inflation)) * 0.55;
                if g > 0.0 {
                    let room = (n.interest_rate / 0.04).clamp(0.0, 1.0);
                    let willing = 1.0 - (-n.bubble).clamp(0.0, 1.0) * 0.75;
                    g *= 0.25 + 0.75 * room.min(willing);
                }
                raw_max = raw_max.max(g);
                raw_min = raw_min.min(g);
                bound_n += 1.0;
                if g.abs() > 0.35 {
                    bound_hits += 1.0;
                }
                let g = g.clamp(-0.35, 0.35);
                samples.push(g);
                if MATURE.contains(&id) {
                    mature.push(g);
                }
            }
            tick_month(&mut w, &[]);
        }
    }
    let f = |v: &mut Vec<f64>, label: &str| {
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        println!(
            "{:>10}: min {:+.4} p01 {:+.4} median {:+.4} p99 {:+.4} max {:+.4}  (annual, fraction)",
            label,
            v[0],
            v[v.len() / 100],
            v[v.len() / 2],
            v[v.len() * 99 / 100],
            v[v.len() - 1]
        );
    };
    println!();
    f(&mut samples, "all nations");
    f(&mut mature, "MATURE_1990");
    println!(
        "\nunclamped reach: min {:+.4} max {:+.4}; |gap| > 0.35 in {:.5}% of nation-months",
        raw_min,
        raw_max,
        bound_hits / bound_n * 100.0
    );
    println!("(bound is +/-0.35, symmetric; the bust side's own bound is -0.344)\n");
}

/// WHERE THE OFF-EQUATION OUTPUT COMES FROM.
///
/// Per month, per mature nation: `resid` is `ln(gdp_after / gdp_before)` minus
/// the `ln(1 + g/12)` the growth equation asked for — everything that reaches
/// GDP without passing through `growth_annual`. There are now exactly two such
/// writes and both are LEVELS by construction: `statecraft::trade_level_gain`
/// and the capital-deepening payment in `economy::tick`. `trade` recomputes the
/// first from `w.statecraft.trade` under the aggregate rule the sim now uses, so
/// `resid - trade` isolates the second.
#[test]
#[ignore]
fn off_equation_output() {
    let years = 35usize;
    let mut resid = vec![0.0f64; MATURE.len()];
    let mut trade = vec![0.0f64; MATURE.len()];
    let mut pacts = vec![0.0f64; MATURE.len()];
    let mut months = vec![0.0f64; MATURE.len()];
    // the running high-water mark the sim keeps in `Nation::trade_level_paid`
    let mut paid = vec![f64::NAN; MATURE.len()];
    for seed in 0..10u64 {
        let mut w = world_1990(GameRules { seed, ..GameRules::default() });
        for p in paid.iter_mut() {
            *p = f64::NAN;
        }
        for _ in 0..(12 * years) {
            let before: Vec<f64> = MATURE.iter().map(|id| w.nation(*id).gdp).collect();
            let g: Vec<f64> =
                MATURE.iter().map(|id| terms(&w, *id).iter().sum::<f64>().max(-0.95)).collect();
            // The aggregate entitlement, priced off pre-tick GDP, exactly as
            // `trade_reach` computes it — depth-weighted partner output over own
            // output plus the full size of every partner.
            let mut t = vec![0.0f64; MATURE.len()];
            for (k, id) in MATURE.iter().enumerate() {
                let mine = w.nation(*id).gdp;
                let (mut access, mut potential) = (0.0, 0.0);
                let mut held = 0.0;
                for p in w.statecraft.trade.iter() {
                    let other = if p.a == *id {
                        p.b
                    } else if p.b == *id {
                        p.a
                    } else {
                        continue;
                    };
                    if let Some(o) = w.nation_opt(other).filter(|n| n.alive) {
                        held += 1.0;
                        access += (p.depth + 0.012).min(1.0) * o.gdp;
                        potential += o.gdp;
                    }
                }
                pacts[k] += held;
                let owed = if potential > 0.0 {
                    0.25 * access / (mine + potential).max(1.0)
                } else {
                    0.0
                };
                if paid[k].is_nan() {
                    paid[k] = owed;
                } else if owed > paid[k] {
                    t[k] += (1.0 + (owed - paid[k])).ln();
                    paid[k] = owed;
                }
            }
            tick_month(&mut w, &[]);
            for (k, id) in MATURE.iter().enumerate() {
                if !w.nation(*id).alive {
                    continue;
                }
                resid[k] += (w.nation(*id).gdp / before[k]).ln() - (1.0 + g[k] / 12.0).ln();
                trade[k] += t[k];
                months[k] += 1.0;
            }
        }
    }
    println!(
        "
{:>8} {:>12} {:>12} {:>14} {:>8}",
        "nation", "resid pp/yr", "trade pp/yr", "capital pp/yr", "pacts"
    );
    for (k, id) in MATURE.iter().enumerate() {
        let m = months[k].max(1.0);
        let r = ((resid[k] / m * 12.0).exp() - 1.0) * 100.0;
        let t = ((trade[k] / m * 12.0).exp() - 1.0) * 100.0;
        println!(
            "{:>8} {:>12.3} {:>12.3} {:>14.3} {:>8.2}",
            format!("{:?}", id),
            r,
            t,
            r - t,
            pacts[k] / m
        );
    }
    println!("
both columns are LEVEL writes shown as the annual rate they average to.
");
}

/// Pact churn: how many trade agreements each mature economy SIGNS over 35
/// years against how many it holds, and how many collapse. A pact re-signed
/// after collapsing starts at depth 0.05 and pays the level gain again, and
/// nothing reverses the gain when it collapses.
#[test]
#[ignore]
fn trade_pact_churn() {
    let years = 35usize;
    let mut signed = vec![0.0f64; MATURE.len()];
    let mut collapsed = vec![0.0f64; MATURE.len()];
    let mut held_end = vec![0.0f64; MATURE.len()];
    let seeds = 10u64;
    for seed in 0..seeds {
        let mut w = world_1990(GameRules { seed, ..GameRules::default() });
        for _ in 0..(12 * years) {
            let hs = tick_month(&mut w, &[]);
            for h in hs {
                for (k, id) in MATURE.iter().enumerate() {
                    if !h.contains(id.name()) {
                        continue;
                    }
                    if h.contains("sign a trade agreement") {
                        signed[k] += 1.0;
                    }
                    if h.contains("dead letter") || h.contains("tears up its trade") {
                        collapsed[k] += 1.0;
                    }
                }
            }
        }
        for (k, id) in MATURE.iter().enumerate() {
            held_end[k] += w
                .statecraft
                .trade
                .iter()
                .filter(|p| p.a == *id || p.b == *id)
                .count() as f64;
        }
    }
    println!("\n{:>8} {:>10} {:>10} {:>12}", "nation", "signed", "collapsed", "held at 2025");
    for (k, id) in MATURE.iter().enumerate() {
        println!(
            "{:>8} {:>10.1} {:>10.1} {:>12.1}",
            format!("{:?}", id),
            signed[k] / seeds as f64,
            collapsed[k] / seeds as f64,
            held_end[k] / seeds as f64
        );
    }
    println!("(per seed, 35 years)\n");
}

/// Do the flag-counting sanction sites touch a mature economy at all?
#[test]
#[ignore]
fn mature_sanction_exposure() {
    let years = 35usize;
    let mut months_sanctioned = vec![0.0f64; MATURE.len()];
    let mut peak = vec![0.0f64; MATURE.len()];
    let mut mean_count = vec![0.0f64; MATURE.len()];
    let mut mean_weight = vec![0.0f64; MATURE.len()];
    let mut m = 0.0f64;
    for seed in 0..10u64 {
        let mut w = world_1990(GameRules { seed, ..GameRules::default() });
        for _ in 0..(12 * years) {
            for (k, id) in MATURE.iter().enumerate() {
                let c = w.sanctioned_by_count(*id) as f64;
                if c > 0.0 {
                    months_sanctioned[k] += 1.0;
                }
                peak[k] = peak[k].max(c);
                mean_count[k] += c;
                mean_weight[k] += w.sanction_weight(*id);
            }
            m += 1.0;
            tick_month(&mut w, &[]);
        }
    }
    println!("\n{:>8} {:>14} {:>10} {:>12} {:>12}", "nation", "% months sanc", "peak flags", "mean flags", "mean weight");
    for (k, id) in MATURE.iter().enumerate() {
        println!(
            "{:>8} {:>14.2} {:>10.0} {:>12.4} {:>12.5}",
            format!("{:?}", id),
            months_sanctioned[k] / m * 100.0,
            peak[k],
            mean_count[k] / m,
            mean_weight[k] / m
        );
    }
    println!();
}

// ---------------------------------------------------------------------------
// PANEL AND CONVERGENCE READOUTS
// ---------------------------------------------------------------------------

/// Real 35-year (1990-2025) GDP CAGR, %/yr, for the six-nation mature panel.
/// World Bank NY.GDP.MKTP.KD, constant prices, same order as `MATURE`.
const REAL_CAGR: [f64; 6] = [2.50, 0.83, 1.28, 1.50, 1.93, 0.76];

/// The upper median, `v[len/2]` — the same convention `mature_cagr` in lib.rs
/// uses, so these tables and the frontier test's readings are the same number.
fn median(v: &mut Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len() / 2]
}

fn ranks(v: &[f64]) -> Vec<f64> {
    let mut idx: Vec<usize> = (0..v.len()).collect();
    idx.sort_by(|a, b| v[*a].partial_cmp(&v[*b]).unwrap());
    let mut r = vec![0.0; v.len()];
    let mut i = 0;
    while i < idx.len() {
        let mut j = i;
        while j + 1 < idx.len() && v[idx[j + 1]] == v[idx[i]] {
            j += 1;
        }
        let avg = ((i + j) as f64) / 2.0 + 1.0;
        for k in i..=j {
            r[idx[k]] = avg;
        }
        i = j + 1;
    }
    r
}

fn spearman(a: &[f64], b: &[f64]) -> f64 {
    let (ra, rb) = (ranks(a), ranks(b));
    let n = a.len() as f64;
    let (ma, mb) = (ra.iter().sum::<f64>() / n, rb.iter().sum::<f64>() / n);
    let mut num = 0.0;
    let (mut da, mut db) = (0.0, 0.0);
    for i in 0..a.len() {
        num += (ra[i] - ma) * (rb[i] - mb);
        da += (ra[i] - ma).powi(2);
        db += (rb[i] - mb).powi(2);
    }
    num / (da * db).sqrt()
}

/// THE PANEL INSTRUMENT: per-nation 35-year CAGR, median of ten seeds, with the
/// error against reality and the ordering checks A2 asks for.
///
/// `cargo test --release -p spheres-sim --test growth_decomposition mature_panel -- --ignored --nocapture`
#[test]
#[ignore]
fn mature_panel() {
    let years = 35usize;
    let mut per: Vec<Vec<f64>> = vec![vec![]; MATURE.len()];
    for seed in 0..10u64 {
        let mut w = world_1990(GameRules { seed, ..GameRules::default() });
        let g0: Vec<f64> = MATURE.iter().map(|id| w.nation(*id).gdp).collect();
        for _ in 0..(12 * years) {
            tick_month(&mut w, &[]);
        }
        for (k, id) in MATURE.iter().enumerate() {
            let n = w.nation(*id);
            if !n.alive {
                continue;
            }
            per[k].push((spheres_sim::exact::powf(n.gdp / g0[k], 1.0 / years as f64) - 1.0) * 100.0);
        }
    }
    let med: Vec<f64> = per.iter_mut().map(|v| median(v)).collect();
    println!("\n{:>8} {:>8} {:>8} {:>8}   seeds", "nation", "model", "real", "error");
    for (k, id) in MATURE.iter().enumerate() {
        let seeds: Vec<String> = per[k].iter().map(|x| format!("{:.2}", x)).collect();
        println!(
            "{:>8} {:>8.2} {:>8.2} {:>+8.2}   [{}]",
            format!("{:?}", id),
            med[k],
            REAL_CAGR[k],
            med[k] - REAL_CAGR[k],
            seeds.join(" ")
        );
    }
    println!("\nSpearman rho vs reality: {:.3}", spearman(&med, &REAL_CAGR));
    let (usa, jpn, ger, fra, uk, ita) = (med[0], med[1], med[2], med[3], med[4], med[5]);
    println!("USA strictly fastest:            {}", usa > jpn && usa > ger && usa > fra && usa > uk && usa > ita);
    println!("Japan below USA/UK/FR/DE:        {}", jpn < usa && jpn < uk && jpn < fra && jpn < ger);
    println!("Italy below USA/UK/FR/DE:        {}", ita < usa && ita < uk && ita < fra && ita < ger);
    println!("Germany < UK:                    {}", ger < uk);
    println!("max |error|: {:.2}\n", med.iter().zip(REAL_CAGR.iter()).map(|(m, r)| (m - r).abs()).fold(0.0f64, f64::max));
}

/// THE CONVERGENCE INSTRUMENT: does the developing world survive the fix?
/// China's 30-year multiple is exactly the quantity `china_growth_miracle`
/// reads; the rest are 35-year multiples and CAGRs.
///
/// `cargo test --release -p spheres-sim --test growth_decomposition developing_panel -- --ignored --nocapture`
#[test]
#[ignore]
fn developing_panel() {
    let poor = [
        NationId::China,
        NationId::SouthKorea,
        NationId::India,
        NationId::Poland,
        NationId::Brazil,
        NationId::Nigeria,
    ];
    let mut m30: Vec<f64> = vec![];
    let mut m35: Vec<Vec<f64>> = vec![vec![]; poor.len()];
    for seed in 0..10u64 {
        let mut w = world_1990(GameRules { seed, ..GameRules::default() });
        let g0: Vec<f64> = poor.iter().map(|id| w.nation(*id).gdp).collect();
        for _ in 0..360 {
            tick_month(&mut w, &[]);
        }
        m30.push(w.nation(NationId::China).gdp / g0[0]);
        for _ in 0..(12 * 5) {
            tick_month(&mut w, &[]);
        }
        for (k, id) in poor.iter().enumerate() {
            if w.nation(*id).alive {
                m35[k].push(w.nation(*id).gdp / g0[k]);
            }
        }
    }
    let mut m30s = m30.clone();
    println!(
        "\nchina_growth_miracle quantity: 30yr median {:.2}x (band 11.0-19.0), per-seed min {:.2}x (floor 6.0), real 14.33x",
        median(&mut m30s),
        m30.iter().cloned().fold(f64::INFINITY, f64::min)
    );
    println!("\n{:>10} {:>12} {:>12}", "nation", "35yr mult", "35yr CAGR%");
    for (k, id) in poor.iter().enumerate() {
        let mut v = m35[k].clone();
        let mu = median(&mut v);
        println!(
            "{:>10} {:>12.2} {:>12.2}",
            format!("{:?}", id),
            mu,
            (spheres_sim::exact::powf(mu, 1.0 / 35.0) - 1.0) * 100.0
        );
    }
    println!();
}

/// The same decomposition, on the convergence side of the board. The mature
/// panel is where the over-run was; this is where the fix has to not break
/// anything, and `china_growth_miracle` reads the first row.
///
/// `cargo test --release -p spheres-sim --test growth_decomposition developing_decomposition -- --ignored --nocapture`
#[test]
#[ignore]
fn developing_decomposition() {
    let panel = [
        NationId::China,
        NationId::SouthKorea,
        NationId::India,
        NationId::Poland,
        NationId::Brazil,
        NationId::Nigeria,
    ];
    let years = 30usize;
    let mut acc = vec![[0.0f64; NTERM]; panel.len()];
    let mut months = vec![0.0f64; panel.len()];
    let mut ratio = vec![0.0f64; panel.len()];
    let mut alive = vec![0.0f64; panel.len()];
    let mut resid = vec![0.0f64; panel.len()];
    let mut popg = vec![0.0f64; panel.len()];
    let mut devs = vec![0.0f64; panel.len()];
    for seed in 0..10u64 {
        let mut w = world_1990(GameRules { seed, ..GameRules::default() });
        let g0: Vec<f64> = panel.iter().map(|id| w.nation(*id).gdp).collect();
        for _ in 0..(12 * years) {
            let before: Vec<f64> = panel.iter().map(|id| w.nation(*id).gdp).collect();
            let g: Vec<f64> = panel
                .iter()
                .map(|id| terms(&w, *id).iter().sum::<f64>().max(-0.95))
                .collect();
            for (k, id) in panel.iter().enumerate() {
                if !w.nation(*id).alive {
                    continue;
                }
                let t = terms(&w, *id);
                for i in 0..NTERM {
                    acc[k][i] += t[i];
                }
                let n = w.nation(*id);
                popg[k] += population_growth(n);
                devs[k] += (n.gdp * 1000.0 / n.population / 24000.0).min(1.0);
                months[k] += 1.0;
            }
            tick_month(&mut w, &[]);
            for (k, id) in panel.iter().enumerate() {
                if w.nation(*id).alive {
                    resid[k] += (w.nation(*id).gdp / before[k]).ln() - (1.0 + g[k] / 12.0).ln();
                }
            }
        }
        for (k, id) in panel.iter().enumerate() {
            if w.nation(*id).alive {
                ratio[k] += (w.nation(*id).gdp / g0[k]).ln();
                alive[k] += 1.0;
            }
        }
    }
    println!(
        "\n{:>10} {}  {:>7} {:>7} {:>7} {:>7} {:>6}",
        "nation",
        NAMES.iter().map(|s| format!("{:>7}", s)).collect::<Vec<_>>().join(" "),
        "SUM",
        "level",
        "actual",
        "popg%",
        "dev"
    );
    for (k, id) in panel.iter().enumerate() {
        let m = months[k].max(1.0);
        let row: Vec<String> =
            (0..NTERM).map(|i| format!("{:>7.3}", acc[k][i] / m * 100.0)).collect();
        let sum: f64 = (0..NTERM).map(|i| acc[k][i] / m).sum::<f64>() * 100.0;
        let lvl = ((resid[k] / m * 12.0).exp() - 1.0) * 100.0;
        let act = ((ratio[k] / alive[k].max(1.0) / years as f64).exp() - 1.0) * 100.0;
        println!(
            "{:>10} {}  {:>7.3} {:>7.3} {:>7.3} {:>7.3} {:>6.3}",
            format!("{:?}", id),
            row.join(" "),
            sum,
            lvl,
            act,
            popg[k] / m * 100.0,
            devs[k] / m
        );
    }
    println!("\n`level` is the off-equation level writes (trade + capital), as an annual rate.");
    println!("30-year window, ten seeds, annual percentage points.\n");
}

/// `the_frontier_does_not_run_away` reads exactly this and prints nothing when
/// it passes. Same panel, same ten seeds, same `v[len/2]` median, same
/// per-seed worst — so the four numbers below are the four the test asserts.
///
/// `cargo test --release -p spheres-sim --test growth_decomposition frontier_readout -- --ignored --nocapture`
#[test]
#[ignore]
fn frontier_readout() {
    for years in [35usize, 0] {
        let (mut fastest, mut slowest) = (vec![], vec![]);
        for seed in 0..10u64 {
            let start: Vec<(NationId, f64)> = {
                let w = world_1990(GameRules { seed, ..GameRules::default() });
                MATURE.iter().map(|id| (*id, w.nation(*id).gdp)).collect()
            };
            let mut w = world_1990(GameRules { seed, ..GameRules::default() });
            for _ in 0..(12 * years) {
                tick_month(&mut w, &[]);
            }
            let mut rates: Vec<f64> = vec![];
            for (id, g0) in start {
                let n = w.nation(id);
                if !n.alive {
                    continue;
                }
                let cagr = if years == 0 {
                    n.gdp / g0 - 1.0
                } else {
                    spheres_sim::exact::powf(n.gdp / g0, 1.0 / years as f64) - 1.0
                };
                rates.push(cagr * 100.0);
            }
            rates.sort_by(|a, b| a.partial_cmp(b).unwrap());
            fastest.push(*rates.last().unwrap());
            slowest.push(rates[0]);
        }
        fastest.sort_by(|a, b| a.partial_cmp(b).unwrap());
        slowest.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let r = |v: &[f64]| v.iter().map(|x| (x * 100.0).round() / 100.0).collect::<Vec<_>>();
        println!("
--- horizon {} years", years);
        println!("  fast_med   {:.2}  (bar < 4.0)   fastest {:?}", fastest[5], r(&fastest));
        println!("  fast_worst {:.2}  (bar < 4.0)", fastest[9]);
        println!("  slow_med   {:.2}  (bar > 0.5)   slowest {:?}", slowest[5], r(&slowest));
        println!("  slow_worst {:.2}  (bar > 0.5)", slowest[0]);
    }
    println!();
}

/// ACCEPTANCE ITEM 11, read out rather than argued: a save written before the
/// four new `Nation` fields existed must load and continue WITHOUT being paid
/// twice. `None` is load-bearing and `0.0` would be wrong — a save carries a GDP
/// that already reflects every level payment made to it, so seeding the
/// high-water marks at zero would re-pay the whole portfolio on the next tick.
///
/// The old save is simulated exactly: serialize a live world, delete the four
/// keys from every nation object, and load that back.
///
/// `cargo test --release -p spheres-sim --test growth_decomposition old_save_is_not_paid_twice -- --ignored --nocapture`
#[test]
#[ignore]
fn old_save_is_not_paid_twice() {
    const NEW: [&str; 4] =
        ["trade_level_paid", "capital_level_paid", "pop_growth_offset", "state_invest_1990"];

    let mut w = world_1990(GameRules { seed: 0, ..GameRules::default() });
    for _ in 0..240 {
        tick_month(&mut w, &[]);
    }

    // ---- the modern save, ticked one more month: the control ----
    let mut modern: WorldState = serde_json::from_str(&spheres_sim::save(&w)).unwrap();
    tick_month(&mut modern, &[]);

    // ---- the same save with the four fields deleted, ticked one month ----
    let mut v: serde_json::Value = serde_json::from_str(&spheres_sim::save(&w)).unwrap();
    let mut stripped_count = 0;
    for n in v["nations"].as_array_mut().unwrap() {
        for k in NEW {
            if n.as_object_mut().unwrap().remove(k).is_some() {
                stripped_count += 1;
            }
        }
    }
    let mut old: WorldState = serde_json::from_str(&v.to_string()).expect("an old save must load");
    println!("\nstripped {} keys from {} nations", stripped_count, v["nations"].as_array().unwrap().len());

    let sample = [
        NationId::USA,
        NationId::Japan,
        NationId::Germany,
        NationId::China,
        NationId::India,
        NationId::Brazil,
    ];
    println!("\n{:>8} {:>12} {:>12} {:>10}", "nation", "gdp before", "gdp after", "paid twice?");
    let mut worst = 0.0f64;
    for id in sample {
        let before = old.nation(id).gdp;
        println!(
            "{:>8} {:>12.2} {:>12} {:>10}",
            format!("{:?}", id),
            before,
            "-",
            match old.nation(id).trade_level_paid {
                None => "None (correct)",
                Some(_) => "SOME — WRONG",
            }
        );
    }
    tick_month(&mut old, &[]);
    println!("\n{:>8} {:>14} {:>14} {:>12}", "nation", "old-save gdp", "modern gdp", "rel diff");
    for id in sample {
        let (o, m) = (old.nation(id).gdp, modern.nation(id).gdp);
        let d = (o / m - 1.0).abs();
        worst = worst.max(d);
        println!("{:>8} {:>14.4} {:>14.4} {:>12.2e}", format!("{:?}", id), o, m, d);
    }
    let mut worst_all = 0.0f64;
    for n in old.nations.iter() {
        if !n.alive {
            continue;
        }
        let m = modern.nation(n.id).gdp;
        if m > 0.0 {
            worst_all = worst_all.max((n.gdp / m - 1.0).abs());
        }
    }
    println!(
        "\nworst relative GDP divergence after one tick, sample {:.3e}, WHOLE ROSTER {:.3e}",
        worst, worst_all
    );
    println!("(a double payment would show as a step of order 1e-2 to 1e-1, not 1e-16)\n");
}

/// `mature_economies_do_not_run_hot` reads `growth_last` at thirty years against
/// a band of [0.008, 0.026] and prints nothing when it passes. The spec's risk
/// note said the binding bar would switch from the ceiling to the FLOOR, so the
/// margin on the floor is the number to watch. Same three seeds, same four
/// nations.
///
/// `cargo test --release -p spheres-sim --test growth_decomposition run_hot_readout -- --ignored --nocapture`
#[test]
#[ignore]
fn run_hot_readout() {
    println!("\n{:>8} {:>10} {:>10} {:>10}   floor margin", "nation", "s1990", "s7", "s42");
    let ids = [NationId::USA, NationId::Germany, NationId::France, NationId::Italy];
    let mut rows = vec![vec![]; ids.len()];
    for seed in [1990u64, 7, 42] {
        let mut w = world_1990(GameRules { seed, ..GameRules::default() });
        for _ in 0..360 {
            tick_month(&mut w, &[]);
        }
        for (k, id) in ids.iter().enumerate() {
            rows[k].push(w.nation(*id).growth_last);
        }
    }
    let mut worst = f64::INFINITY;
    for (k, id) in ids.iter().enumerate() {
        let lo = rows[k].iter().cloned().fold(f64::INFINITY, f64::min);
        worst = worst.min(lo - 0.008);
        println!(
            "{:>8} {:>10.4} {:>10.4} {:>10.4}   {:+.4}",
            format!("{:?}", id),
            rows[k][0],
            rows[k][1],
            rows[k][2],
            lo - 0.008
        );
    }
    println!("\nband [0.008, 0.026]; tightest floor margin {:+.4}\n", worst);
}

/// A6: SHAPE, NOT JUST ENDPOINTS. A panel can hit six correct 35-year averages
/// by running flat at the average, which is not the same model. Decade CAGRs,
/// median of ten seeds, plus how long the worst drawdown from a running peak
/// lasts — Italy is supposed to be able to go a decade without regaining one.
///
/// `cargo test --release -p spheres-sim --test growth_decomposition decade_shape -- --ignored --nocapture`
#[test]
#[ignore]
fn decade_shape() {
    let cuts = [0usize, 120, 240, 360, 420];
    let labels = ["1990s", "2000s", "2010s", "2020-25"];
    let mut dec: Vec<Vec<Vec<f64>>> = vec![vec![vec![]; labels.len()]; MATURE.len()];
    let mut trough: Vec<Vec<f64>> = vec![vec![]; MATURE.len()];
    for seed in 0..10u64 {
        let mut w = world_1990(GameRules { seed, ..GameRules::default() });
        let mut series: Vec<Vec<f64>> = vec![vec![]; MATURE.len()];
        for _ in 0..420 {
            for (k, id) in MATURE.iter().enumerate() {
                series[k].push(w.nation(*id).gdp);
            }
            tick_month(&mut w, &[]);
        }
        for (k, _) in MATURE.iter().enumerate() {
            for (d, w2) in cuts.windows(2).enumerate() {
                let (a, b) = (series[k][w2[0]], series[k][w2[1].min(419)]);
                let yrs = (w2[1] - w2[0]) as f64 / 12.0;
                dec[k][d].push((spheres_sim::exact::powf(b / a, 1.0 / yrs) - 1.0) * 100.0);
            }
            // longest run of months spent below a previous peak
            let (mut peak, mut run, mut worst) = (0.0f64, 0usize, 0usize);
            for v in series[k].iter() {
                if *v >= peak {
                    peak = *v;
                    run = 0;
                } else {
                    run += 1;
                    worst = worst.max(run);
                }
            }
            trough[k].push(worst as f64 / 12.0);
        }
    }
    println!("\n{:>8} {:>9} {:>9} {:>9} {:>9}   {:>16}", "nation", labels[0], labels[1], labels[2], labels[3], "longest drawdown");
    for (k, id) in MATURE.iter().enumerate() {
        let d: Vec<String> =
            (0..labels.len()).map(|i| format!("{:>9.2}", median(&mut dec[k][i].clone()))).collect();
        println!(
            "{:>8} {}   {:>13.1} yr",
            format!("{:?}", id),
            d.join(" "),
            median(&mut trough[k].clone())
        );
    }
    println!("\nannual %, median of ten seeds. drawdown = longest stretch below a previous peak.\n");
}

// ---------------------------------------------------------------------------
// THE TRANSITION PANEL
// ---------------------------------------------------------------------------

/// Every economy that entered 1990 running a plan and had to stop, plus the
/// two Soviet successors big enough to read. Ordered as they are printed.
///
/// Russia and Ukraine do not exist on 1 January 1990 and appear only when the
/// union comes apart, so every accumulator below is guarded on `alive` and the
/// month counts differ per nation by construction.
const TRANSITION: [(NationId, &str); 10] = [
    (NationId::Russia, "Russia"),
    (NationId::Ukraine, "Ukraine"),
    (NationId::Poland, "Poland"),
    (NationId::Czechoslovakia, "Czechoslov"),
    (NationId::Hungary, "Hungary"),
    (NationId::Romania, "Romania"),
    (NationId::Bulgaria, "Bulgaria"),
    (NationId::Belarus, "Belarus"),
    (NationId::Kazakhstan, "Kazakhstan"),
    (NationId::USSR, "USSR"),
];

/// Real 1990 = 100 output index for the transition economies, so the model's
/// series has something to be read against rather than an impression.
///
/// Russia: Rosstat/World Bank NY.GDP.MKTP.KD chained to the Goskomstat RSFSR
/// series - 1990 100, 1995 61.4, 1998 55.8, 2000 64.2, 2010 92.6, 2025 ~112.
/// Poland: 1990 100, 1995 112, 1998 128, 2000 137, 2010 191, 2025 ~276.
/// Ukraine: 1990 100, 1995 47.8, 1998 40.8, 2000 43.2, 2010 63.4, 2025 ~52.
/// Czechoslovakia (Czechia + Slovakia summed): 1990 100, 1995 88, 1998 96,
/// 2000 100, 2010 133, 2025 ~155.
const REAL_INDEX: [(&str, [f64; 6]); 4] = [
    ("Russia", [100.0, 61.4, 55.8, 64.2, 92.6, 112.0]),
    ("Ukraine", [100.0, 47.8, 40.8, 43.2, 63.4, 52.0]),
    ("Poland", [100.0, 112.0, 128.0, 137.0, 191.0, 276.0]),
    ("Czechoslov", [100.0, 88.0, 96.0, 100.0, 133.0, 155.0]),
];

/// THE INSTRUMENT THE TRANSITION PANEL WAS MISSING. Same recomputation as
/// `growth_decomposition` above, on the nations that had to stop running a
/// plan, split at the year 2000 so the collapse decade and the recovery are
/// not averaged into each other.
///
/// `cargo test --release -p spheres-sim --test growth_decomposition transition_decomposition -- --ignored --nocapture`
#[test]
#[ignore]
fn transition_decomposition() {
    let years = 35usize;
    // [nation][era][term]; era 0 is months before Jan 2000, era 1 after.
    let mut acc = vec![[[0.0f64; NTERM]; 2]; TRANSITION.len()];
    let mut months = vec![[0.0f64; 2]; TRANSITION.len()];
    let mut ratio = vec![[0.0f64; 2]; TRANSITION.len()];
    let mut resid = vec![[0.0f64; 2]; TRANSITION.len()];
    let mut sys_cmd = vec![[0.0f64; 2]; TRANSITION.len()];
    for seed in 0..10u64 {
        let mut w = world_1990(GameRules { seed, ..GameRules::default() });
        for month in 0..(12 * years) {
            let era = if month < 120 { 0 } else { 1 };
            let before: Vec<Option<f64>> = TRANSITION
                .iter()
                .map(|(id, _)| w.nation_opt(*id).filter(|n| n.alive).map(|n| n.gdp))
                .collect();
            let g: Vec<f64> = TRANSITION
                .iter()
                .map(|(id, _)| match w.nation_opt(*id) {
                    Some(n) if n.alive => terms(&w, *id).iter().sum::<f64>().max(-0.95),
                    _ => 0.0,
                })
                .collect();
            for (k, (id, _)) in TRANSITION.iter().enumerate() {
                if before[k].is_none() {
                    continue;
                }
                let t = terms(&w, *id);
                for i in 0..NTERM {
                    acc[k][era][i] += t[i];
                }
                if w.nation(*id).system == EconomySystem::Command {
                    sys_cmd[k][era] += 1.0;
                }
                months[k][era] += 1.0;
            }
            tick_month(&mut w, &[]);
            for (k, (id, _)) in TRANSITION.iter().enumerate() {
                let (Some(b), Some(n)) = (before[k], w.nation_opt(*id).filter(|n| n.alive)) else {
                    continue;
                };
                ratio[k][era] += (n.gdp / b).ln();
                resid[k][era] += (n.gdp / b).ln() - (1.0 + g[k] / 12.0).ln();
            }
        }
    }
    for (era, label) in ["1990-1999", "2000-2025"].iter().enumerate() {
        println!(
            "\n=== {} ===\n{:>11} {}  {:>7} {:>7} {:>7} {:>6}",
            label,
            "nation",
            NAMES.iter().map(|s| format!("{:>7}", s)).collect::<Vec<_>>().join(" "),
            "SUM",
            "level",
            "actual",
            "cmd%"
        );
        for (k, (_, name)) in TRANSITION.iter().enumerate() {
            let m = months[k][era];
            if m < 1.0 {
                println!("{:>11}  (not alive in this window)", name);
                continue;
            }
            let row: Vec<String> =
                (0..NTERM).map(|i| format!("{:>7.3}", acc[k][era][i] / m * 100.0)).collect();
            let sum: f64 = (0..NTERM).map(|i| acc[k][era][i] / m).sum::<f64>() * 100.0;
            println!(
                "{:>11} {}  {:>7.3} {:>7.3} {:>7.3} {:>6.0}",
                name,
                row.join(" "),
                sum,
                ((resid[k][era] / m * 12.0).exp() - 1.0) * 100.0,
                ((ratio[k][era] / m * 12.0).exp() - 1.0) * 100.0,
                sys_cmd[k][era] / m * 100.0
            );
        }
    }
    println!("\nannual percentage points, averaged over the months each nation was alive.");
    println!("`level` is the off-equation level writes. `cmd%` is the share of");
    println!("those months spent on EconomySystem::Command.\n");
}

/// THE TRAJECTORY. Output indexed to 1 January 1990 = 100, at the six dates the
/// transition literature reads: the start, the mid-nineties, the 1998 trough,
/// the turn, the oil decade, and today.
///
/// A successor that does not exist in 1990 is indexed against the share of the
/// union it was born holding - which is exactly what `dissolve_ussr` hands it,
/// so the index is 1990 = 100 for a Russia-sized slice of the 1990 Soviet
/// Union, and directly comparable to the Rosstat series in `REAL_INDEX`.
///
/// `cargo test --release -p spheres-sim --test growth_decomposition transition_trajectory -- --ignored --nocapture`
#[test]
#[ignore]
fn transition_trajectory() {
    let cuts = [0usize, 60, 96, 120, 240, 420];
    let labels = ["1990", "1995", "1998", "2000", "2010", "2025"];
    // per nation, per cut, per seed
    let mut idx: Vec<Vec<Vec<f64>>> = vec![vec![vec![]; cuts.len()]; TRANSITION.len()];
    let mut abs: Vec<Vec<Vec<f64>>> = vec![vec![vec![]; cuts.len()]; TRANSITION.len()];
    let mut dissolve_month: Vec<f64> = vec![];
    for seed in 0..10u64 {
        let mut w = world_1990(GameRules { seed, ..GameRules::default() });
        // The base each nation is indexed against: its own 1990 GDP if it has
        // one, otherwise the GDP it is born holding.
        let mut base: Vec<Option<f64>> = TRANSITION
            .iter()
            .map(|(id, _)| w.nation_opt(*id).filter(|n| n.alive).map(|n| n.gdp))
            .collect();
        let mut series: Vec<Vec<f64>> = vec![vec![]; TRANSITION.len()];
        let mut dm = f64::NAN;
        for month in 0..421 {
            for (k, (id, _)) in TRANSITION.iter().enumerate() {
                let live = w.nation_opt(*id).filter(|n| n.alive).map(|n| n.gdp);
                if base[k].is_none() {
                    base[k] = live;
                }
                series[k].push(live.unwrap_or(f64::NAN));
            }
            if dm.is_nan() && w.has_flag("ussr_dissolved") {
                dm = month as f64;
            }
            tick_month(&mut w, &[]);
        }
        if dm.is_finite() {
            dissolve_month.push(dm);
        }
        for (k, _) in TRANSITION.iter().enumerate() {
            let Some(b) = base[k] else { continue };
            for (c, cut) in cuts.iter().enumerate() {
                let v = series[k][*cut];
                if v.is_finite() {
                    idx[k][c].push(v / b * 100.0);
                    abs[k][c].push(v);
                }
            }
        }
    }
    if !dissolve_month.is_empty() {
        let mut d = dissolve_month.clone();
        let med = median(&mut d);
        println!(
            "\nUSSR dissolves in month {:.0} (median of {} seeds), i.e. {}",
            med,
            dissolve_month.len(),
            1990 + (med as usize) / 12
        );
    }
    println!("\nOUTPUT INDEX, 1990 = 100 (median of ten seeds)");
    println!(
        "{:>11} {}",
        "nation",
        labels.iter().map(|s| format!("{:>8}", s)).collect::<Vec<_>>().join(" ")
    );
    for (k, (_, name)) in TRANSITION.iter().enumerate() {
        let row: Vec<String> = (0..cuts.len())
            .map(|c| {
                if idx[k][c].is_empty() {
                    format!("{:>8}", "-")
                } else {
                    format!("{:>8.1}", median(&mut idx[k][c].clone()))
                }
            })
            .collect();
        println!("{:>11} {}", name, row.join(" "));
        if let Some((_, real)) = REAL_INDEX.iter().find(|(n, _)| *n == *name) {
            println!(
                "{:>11} {}   <- REAL",
                "",
                real.iter().map(|v| format!("{:>8.1}", v)).collect::<Vec<_>>().join(" ")
            );
        }
    }
    println!("\nGDP, $bn of 1990 dollars (median of ten seeds)");
    println!(
        "{:>11} {}",
        "nation",
        labels.iter().map(|s| format!("{:>8}", s)).collect::<Vec<_>>().join(" ")
    );
    for (k, (_, name)) in TRANSITION.iter().enumerate() {
        let row: Vec<String> = (0..cuts.len())
            .map(|c| {
                if abs[k][c].is_empty() {
                    format!("{:>8}", "-")
                } else {
                    format!("{:>8.0}", median(&mut abs[k][c].clone()))
                }
            })
            .collect();
        println!("{:>11} {}", name, row.join(" "));
    }
    println!();
}

/// WHO `money_works` IN economy.rs ACTUALLY REACHES.
///
/// A limiter written for a monetary collapse must be shown not to be quietly
/// taxing economies that are merely warm. This prints, for every nation the
/// model runs, the mean and worst value of the factor and the share of its
/// nation-months spent below 0.9 â€” and then the twenty nations it costs the
/// most, so the list can be read against the list of countries whose currencies
/// actually failed in the period.
///
/// `cargo test --release -p spheres-sim --test growth_decomposition money_works_reach -- --ignored --nocapture`
#[test]
#[ignore]
fn money_works_reach() {
    use std::collections::BTreeMap;
    let years = 35usize;
    // code -> (sum factor, months, worst, months below 0.9, peak inflation)
    let mut acc: BTreeMap<&'static str, (f64, f64, f64, f64, f64)> = BTreeMap::new();
    for seed in 0..10u64 {
        let mut w = world_1990(GameRules { seed, ..GameRules::default() });
        for _ in 0..(12 * years) {
            let rows: Vec<(&'static str, f64)> = w
                .nations
                .iter()
                .filter(|n| n.alive)
                .map(|n| (n.id.code(), n.inflation))
                .collect();
            for (code, pi) in rows {
                let f = 1.0 / (1.0 + (pi.max(0.0) / 0.40 - 1.0).max(0.0).powi(2));
                let e = acc.entry(code).or_insert((0.0, 0.0, 1.0, 0.0, 0.0));
                e.0 += f;
                e.1 += 1.0;
                e.2 = e.2.min(f);
                if f < 0.9 {
                    e.3 += 1.0;
                }
                e.4 = e.4.max(pi);
            }
            tick_month(&mut w, &[]);
        }
    }
    let mut rows: Vec<(&'static str, f64, f64, f64, f64)> =
        acc.iter().map(|(k, v)| (*k, v.0 / v.1.max(1.0), v.2, v.3 / v.1.max(1.0), v.4)).collect();
    rows.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    println!("\nthe twenty nations `money_works` costs the most, ten seeds, 35 years");
    println!("{:>8} {:>10} {:>10} {:>12} {:>12}", "nation", "mean", "worst", "% mo < 0.9", "peak infl");
    for (c, mean, worst, below, peak) in rows.iter().take(20) {
        println!("{:>8} {:>10.4} {:>10.4} {:>12.1} {:>12.3}", c, mean, worst, below * 100.0, peak);
    }
    println!("\nand the six-nation mature panel, which it must not be able to reach:");
    for id in MATURE.iter() {
        let c = id.code();
        if let Some((_, mean, worst, below, peak)) = rows.iter().find(|r| r.0 == c) {
            println!(
                "{:>8} {:>10.4} {:>10.4} {:>12.1} {:>12.3}",
                c,
                mean,
                worst,
                below * 100.0,
                peak
            );
        }
    }
    let n_touched = rows.iter().filter(|r| r.3 > 0.01).count();
    println!(
        "\n{} of {} nations spend more than 1% of their months below 0.9.\n",
        n_touched,
        rows.len()
    );
}

/// The ten seeds behind `china_growth_miracle`'s median, so a thin margin can be
/// read as what it is â€” which side of the bimodality each seed fell on â€” rather
/// than guessed at.
///
/// `cargo test --release -p spheres-sim --test growth_decomposition china_seeds -- --ignored --nocapture`
#[test]
#[ignore]
fn china_seeds() {
    let mut m30: Vec<f64> = vec![];
    for seed in 0..10u64 {
        let mut w = world_1990(GameRules { seed, ..GameRules::default() });
        let g0 = w.nation(NationId::China).gdp;
        let mut war_months = 0.0;
        let mut sanc = 0.0;
        for _ in 0..360 {
            if w.at_war(NationId::China) {
                war_months += 1.0;
            }
            sanc += w.sanction_weight(NationId::China);
            tick_month(&mut w, &[]);
        }
        let mult = w.nation(NationId::China).gdp / g0;
        m30.push(mult);
        println!(
            "seed {:>2}: {:>6.2}x   war {:>3.0} months   mean sanction weight {:.4}",
            seed, mult, war_months, sanc / 360.0
        );
    }
    let mut s = m30.clone();
    println!(
        "\nmedian {:.2}x (band 11.0-19.0), min {:.2}x (floor 6.0)\n",
        median(&mut s),
        m30.iter().cloned().fold(f64::INFINITY, f64::min)
    );
}

/// IS `gulf_war_emerges` MEASURING A RATE, OR MEASURING A RESHUFFLE?
///
/// The test's own comment says forty seeds was chosen to get the standard error
/// on a rate near a half down to about 8 points, and that "if the true rate ever
/// sits below [twenty], that is a finding about the model's appetite pass and
/// belongs in a bug entry, not in this literal." This is the instrument that
/// tells those two apart, because at forty seeds a 55% rate and a 45% rate are
/// less than two standard errors apart and every change that touches any
/// nation's state anywhere reshuffles the shared RNG stream underneath the
/// appetite roll.
///
/// Two hundred seeds takes the standard error to about 3.5 points. Same
/// quantity, same 48-month window, same `Iraq invades Kuwait` headline, and the
/// first forty are printed separately so the reading `gulf_war_emerges` takes
/// stays legible beside the wider one.
///
/// `cargo test --release -p spheres-sim --test growth_decomposition gulf_war_wide_scan -- --ignored --nocapture`
#[test]
#[ignore]
fn gulf_war_wide_scan() {
    const N: u64 = 200;
    let mut hits: Vec<u64> = vec![];
    for seed in 0..N {
        let mut w = world_1990(GameRules { seed, ..GameRules::default() });
        let mut saw = false;
        for _ in 0..48 {
            let headlines = tick_month(&mut w, &[]);
            if headlines.iter().any(|h| h.contains("Iraq invades Kuwait")) {
                saw = true;
            }
        }
        if saw || !w.nation(NationId::Kuwait).alive {
            hits.push(seed);
        }
    }
    let n = N as f64;
    let k = hits.len() as f64;
    let p = k / n;
    let se = (p * (1.0 - p) / n).sqrt();
    println!("\nIraq invades Kuwait in {}/{} seeds = {:.1}%", hits.len(), N, p * 100.0);
    println!(
        "  standard error {:.1} points; 95% interval [{:.1}%, {:.1}%]",
        se * 100.0,
        (p - 1.96 * se) * 100.0,
        (p + 1.96 * se) * 100.0
    );
    println!(
        "  first forty (what `gulf_war_emerges` reads, bar 20): {}/40",
        hits.iter().filter(|s| **s < 40).count()
    );
    println!(
        "  first ten  (what it used to read, bar 5):            {}/10\n",
        hits.iter().filter(|s| **s < 10).count()
    );
}

/// THE SAME PANEL, FOUR TIMES THE SEEDS, AND THE PAIRWISE ORDERINGS PRINTED AS
/// FREQUENCIES RATHER THAN AS A SINGLE MEDIAN COMPARISON.
///
/// `mature_panel` above compares six medians of ten seeds, and Spearman's rho on
/// six points moves 0.057 whenever any adjacent pair swaps. Three of those pairs
/// â€” Germany/France/UK â€” sit inside 0.05 of each other with ten-seed
/// distributions that overlap almost completely, so the rho a run reports is
/// partly a statement about the model and partly a coin flip. This separates
/// them: `P(a > b)` over forty seeds says how often the model actually orders a
/// pair, and a pair it orders 52% of the time is not an ordering the rho should
/// be read as measuring.
///
/// `cargo test --release -p spheres-sim --test growth_decomposition mature_panel_wide -- --ignored --nocapture`
#[test]
#[ignore]
fn mature_panel_wide() {
    const N: u64 = 40;
    let years = 35usize;
    let mut per: Vec<Vec<f64>> = vec![vec![]; MATURE.len()];
    for seed in 0..N {
        let mut w = world_1990(GameRules { seed, ..GameRules::default() });
        let g0: Vec<f64> = MATURE.iter().map(|id| w.nation(*id).gdp).collect();
        for _ in 0..(12 * years) {
            tick_month(&mut w, &[]);
        }
        for (k, id) in MATURE.iter().enumerate() {
            let n = w.nation(*id);
            if n.alive {
                per[k].push(
                    (spheres_sim::exact::powf(n.gdp / g0[k], 1.0 / years as f64) - 1.0) * 100.0,
                );
            }
        }
    }
    let med: Vec<f64> = per.iter_mut().map(|v| median(v)).collect();
    println!("\n{:>8} {:>8} {:>8} {:>8} {:>18}", "nation", "model", "real", "error", "p10-p90");
    for (k, id) in MATURE.iter().enumerate() {
        let mut v = per[k].clone();
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        println!(
            "{:>8} {:>8.2} {:>8.2} {:>+8.2} {:>10.2}-{:.2}",
            format!("{:?}", id),
            med[k],
            REAL_CAGR[k],
            med[k] - REAL_CAGR[k],
            v[v.len() / 10],
            v[v.len() * 9 / 10]
        );
    }
    println!("\nSpearman rho vs reality ({} seeds): {:.3}", N, spearman(&med, &REAL_CAGR));
    let (usa, jpn, ger, fra, uk, ita) = (med[0], med[1], med[2], med[3], med[4], med[5]);
    println!("USA strictly fastest:            {}", usa > jpn && usa > ger && usa > fra && usa > uk && usa > ita);
    println!("Japan below USA/UK/FR/DE:        {}", jpn < usa && jpn < uk && jpn < fra && jpn < ger);
    println!("Italy below USA/UK/FR/DE:        {}", ita < usa && ita < uk && ita < fra && ita < ger);
    println!("Germany < UK:                    {}", ger < uk);
    println!(
        "max |error|: {:.2}",
        med.iter().zip(REAL_CAGR.iter()).map(|(m, r)| (m - r).abs()).fold(0.0f64, f64::max)
    );

    // How reliably does the model order each pair? A pair it separates 95% of
    // the time is a claim; a pair it separates 52% of the time is a coin.
    println!("\nP(row faster than column), {} seeds â€” 0.50 means the model does not order the pair", N);
    print!("{:>8}", "");
    for id in MATURE.iter() {
        print!(" {:>8}", format!("{:?}", id));
    }
    println!();
    for (a, ida) in MATURE.iter().enumerate() {
        print!("{:>8}", format!("{:?}", ida));
        for (b, _) in MATURE.iter().enumerate() {
            if a == b {
                print!(" {:>8}", "-");
                continue;
            }
            let n = per[a].len().min(per[b].len());
            let wins = (0..n).filter(|i| per[a][*i] > per[b][*i]).count() as f64;
            print!(" {:>8.2}", wins / n as f64);
        }
        println!();
    }
    println!();
}
