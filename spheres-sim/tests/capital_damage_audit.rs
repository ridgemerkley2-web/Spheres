//! AUDIT INSTRUMENT — measurement only. Asserts nothing, `#[ignore]`d, adds no
//! bar and moves none. It exists to measure the damage the capital-channel swap
//! did to the economies that have no calibration test watching them.
//!
//! It is the HEAD-side twin of `capital_damage_audit.rs` in the HEAD~1/HEAD~2
//! worktrees: same panel, same seeds, same windows, same printing, so the three
//! runs are directly subtractable. The only thing that differs between the two
//! copies is the recomputation of `growth_annual`'s terms, which must mirror
//! whatever `economy.rs` does at that revision.
//!
//! UPDATED 2026-08-31 by the capital-repair pass: `terms()` now mirrors the
//! REPAIRED `economy.rs`, so a run of this file against the HEAD~1/HEAD~2 twins
//! is a three-way comparison across the swap AND its repair. Nothing else moved.
//!
//! THE CAPITAL CHANNEL IS TWO THINGS AND THEY ARE MEASURED SEPARATELY:
//!   * `invest`  — the RATE arm, now `net_intensity * 0.080 * (1 - dev)` where
//!                 `net_intensity` is investment above the replacement share.
//!   * `caplvl`  — the LEVEL write, read EXACTLY off `Nation::capital_level_paid`
//!                 rather than recomputed. That field is the running log of what
//!                 the nation has been paid, so its endpoint difference IS the
//!                 total multiplier applied to GDP by the level block. No
//!                 transcription risk in that column at all.
//!
//! Run:
//!   cargo test --release -p spheres-sim --test capital_damage_audit -- --ignored --nocapture

use spheres_sim::economy::population_growth;
use spheres_sim::init::world_1990;
use spheres_sim::world::{EconomySystem, GameRules, NationId, WorldState};
use spheres_sim::{tech, tick_month};

const REV: &str = "HEAD + capital-channel repair (2026-08-31)";
const SEEDS: u64 = 100;
const M30: usize = 360;
const M35: usize = 420;

/// Eight developing economies — the six the swap is known to have touched, plus
/// Indonesia and Vietnam, chosen because they are the two large poor Asian
/// economies with the highest 1990 investment shares outside China and Korea,
/// so if a capital-share channel moved anything they are where it shows.
/// Then the six mature economies, as controls.
const PANEL: [(NationId, &str); 14] = [
    (NationId::China, "China"),
    (NationId::India, "India"),
    (NationId::SouthKorea, "SouthKorea"),
    (NationId::Poland, "Poland"),
    (NationId::Brazil, "Brazil"),
    (NationId::Nigeria, "Nigeria"),
    (NationId::Indonesia, "Indonesia"),
    (NationId::Vietnam, "Vietnam"),
    (NationId::USA, "USA"),
    (NationId::Japan, "Japan"),
    (NationId::Germany, "Germany"),
    (NationId::France, "France"),
    (NationId::UK, "UK"),
    (NationId::Italy, "Italy"),
];

const NTERM: usize = 12;
const NAMES: [&str; NTERM] = [
    "tfp", "invest", "catchup", "labour", "command", "demand", "bubble", "oil", "sanct", "war",
    "debt", "instab",
];
/// Index of the investment/capital RATE arm inside `terms`.
const I_INVEST: usize = 1;

/// The terms of `growth_annual` at THIS revision, signed as they enter the sum.
/// Transcribed from `economy::tick` at eb7de26.
fn terms(w: &WorldState, id: NationId) -> [f64; NTERM] {
    let n = w.nation(id);
    let oil_price = w.oil_price;
    let sanction_share = w.sanction_weight(id);
    let at_war = w.at_war(id);
    let export_share = w.oil_export_share(id);

    let invest = n.state_invest_gdp + n.priv_invest_gdp;
    let gdp_pc = n.gdp * 1000.0 / n.population;
    let dev = (gdp_pc / 24000.0).min(1.0);
    // REPAIRED (2026-08-31): net investment above the replacement share, pinned
    // to equal the old concave `intensity` exactly at s = 0.20. Coefficient
    // 0.080 unmoved; the 0.030 floor arm stays gone.
    let net_intensity = (invest.max(0.0) - 0.125) * (0.20 / (0.20 - 0.125));
    let invest_effect = net_intensity * 0.080 * (1.0 - dev);
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
    let demand_gap = demand_gap.clamp(-0.35, 0.35);
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
        oil_effect - embargo_drag,
        -(sanction_share * 0.020),
        -(if at_war { 0.020 + n.war_exhaustion * 0.03 } else { 0.0 }),
        -(if n.debt_gdp > 0.9 { (n.debt_gdp - 0.9) * 0.02 } else { 0.0 }),
        -(if n.stability < 40.0 { (40.0 - n.stability) * 0.0009 } else { 0.0 }),
    ]
}

/// EXACT, not recomputed: how much log-output the capital LEVEL block has paid
/// this nation so far. At revisions without the block this returns 0.0 and the
/// column is zero by construction.
fn capital_level_paid(w: &WorldState, id: NationId) -> f64 {
    w.nation(id).capital_level_paid.unwrap_or(0.0)
}

fn med(v: &mut Vec<f64>) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len() / 2]
}

#[test]
#[ignore]
fn capital_damage_audit() {
    let np = PANEL.len();
    // per-nation, per-seed
    let mut mult30: Vec<Vec<f64>> = vec![vec![]; np];
    let mut mult35: Vec<Vec<f64>> = vec![vec![]; np];
    let mut caplvl30: Vec<Vec<f64>> = vec![vec![]; np]; // ln paid over 0..360
    let mut caplvl35: Vec<Vec<f64>> = vec![vec![]; np]; // ln paid over 0..420
    let mut inv90: Vec<Vec<f64>> = vec![vec![]; np];
    let mut inv20: Vec<Vec<f64>> = vec![vec![]; np];
    let mut inv25: Vec<Vec<f64>> = vec![vec![]; np];
    let mut dev25: Vec<Vec<f64>> = vec![vec![]; np];
    // term accumulators over the whole 35 years, and the invest arm over 30
    let mut acc = vec![[0.0f64; NTERM]; np];
    let mut acc_inv30 = vec![0.0f64; np];
    let mut months = vec![0.0f64; np];
    let mut months30 = vec![0.0f64; np];
    // total off-equation level writes (trade + capital), in logs
    let mut resid = vec![0.0f64; np];
    let mut deaths = vec![0.0f64; np];

    for seed in 0..SEEDS {
        let mut w = world_1990(GameRules { seed, ..GameRules::default() });
        let g0: Vec<f64> = PANEL.iter().map(|(id, _)| w.nation(*id).gdp).collect();
        for (k, (id, _)) in PANEL.iter().enumerate() {
            let n = w.nation(*id);
            inv90[k].push(n.state_invest_gdp + n.priv_invest_gdp);
        }
        // the level tracker seeds itself on the first tick, so the baseline is
        // read after month one, not before it
        let mut cap_base: Vec<Option<f64>> = vec![None; np];
        let mut cap_at30: Vec<f64> = vec![0.0; np];

        for month in 0..M35 {
            let before: Vec<f64> = PANEL.iter().map(|(id, _)| w.nation(*id).gdp).collect();
            let g: Vec<f64> = PANEL
                .iter()
                .map(|(id, _)| terms(&w, *id).iter().sum::<f64>().max(-0.95))
                .collect();
            for (k, (id, _)) in PANEL.iter().enumerate() {
                if !w.nation(*id).alive {
                    continue;
                }
                let t = terms(&w, *id);
                for i in 0..NTERM {
                    acc[k][i] += t[i];
                }
                months[k] += 1.0;
                if month < M30 {
                    acc_inv30[k] += t[I_INVEST];
                    months30[k] += 1.0;
                }
            }
            tick_month(&mut w, &[]);
            for (k, (id, _)) in PANEL.iter().enumerate() {
                if !w.nation(*id).alive {
                    continue;
                }
                resid[k] += (w.nation(*id).gdp / before[k]).ln() - (1.0 + g[k] / 12.0).ln();
                if cap_base[k].is_none() {
                    cap_base[k] = Some(capital_level_paid(&w, *id));
                }
            }
            if month + 1 == M30 {
                for (k, (id, _)) in PANEL.iter().enumerate() {
                    if w.nation(*id).alive {
                        mult30[k].push(w.nation(*id).gdp / g0[k]);
                        cap_at30[k] = capital_level_paid(&w, *id);
                        inv20[k].push(
                            w.nation(*id).state_invest_gdp + w.nation(*id).priv_invest_gdp,
                        );
                    }
                }
            }
        }
        for (k, (id, _)) in PANEL.iter().enumerate() {
            let n = w.nation(*id);
            if !n.alive {
                deaths[k] += 1.0;
                continue;
            }
            mult35[k].push(n.gdp / g0[k]);
            inv25[k].push(n.state_invest_gdp + n.priv_invest_gdp);
            dev25[k].push((n.gdp * 1000.0 / n.population / 24000.0).min(1.0));
            let base = cap_base[k].unwrap_or(0.0);
            caplvl30[k].push(cap_at30[k] - base);
            caplvl35[k].push(capital_level_paid(&w, *id) - base);
        }
    }

    let ann = |ln: f64, yrs: f64| ((ln / yrs).exp() - 1.0) * 100.0;

    println!("\n################ REV {} — {} seeds ################", REV, SEEDS);

    println!("\n=== TABLE A: outcome. medians over {} seeds ===", SEEDS);
    println!(
        "{:>11} {:>10} {:>10} {:>10} {:>10} {:>7}",
        "nation", "30y mult", "30y CAGR", "35y mult", "35y CAGR", "deaths"
    );
    for (k, (_, name)) in PANEL.iter().enumerate() {
        let m30 = med(&mut mult30[k].clone());
        let m35 = med(&mut mult35[k].clone());
        println!(
            "{:>11} {:>10.4} {:>10.4} {:>10.4} {:>10.4} {:>7.0}",
            name,
            m30,
            (spheres_sim::exact::powf(m30, 1.0 / 30.0) - 1.0) * 100.0,
            m35,
            (spheres_sim::exact::powf(m35, 1.0 / 35.0) - 1.0) * 100.0,
            deaths[k]
        );
    }

    println!("\n=== TABLE B: the capital channel, pt/yr ===");
    println!(
        "{:>11} {:>10} {:>10} {:>10} {:>10} {:>12} {:>9} {:>9} {:>9} {:>7}",
        "nation",
        "inv30",
        "inv35",
        "caplvl30",
        "caplvl35",
        "CAP TOTAL35",
        "s1990",
        "s2020",
        "s2025",
        "dev25"
    );
    for (k, (_, name)) in PANEL.iter().enumerate() {
        let m = months[k].max(1.0);
        let m3 = months30[k].max(1.0);
        let i30 = acc_inv30[k] / m3 * 100.0;
        let i35 = acc[k][I_INVEST] / m * 100.0;
        let c30 = ann(med(&mut caplvl30[k].clone()), 30.0);
        let c35 = ann(med(&mut caplvl35[k].clone()), 35.0);
        println!(
            "{:>11} {:>+10.4} {:>+10.4} {:>+10.4} {:>+10.4} {:>+12.4} {:>9.4} {:>9.4} {:>9.4} {:>7.3}",
            name,
            i30,
            i35,
            c30,
            c35,
            i35 + c35,
            med(&mut inv90[k].clone()),
            med(&mut inv20[k].clone()),
            med(&mut inv25[k].clone()),
            med(&mut dev25[k].clone())
        );
    }
    println!("inv* = the RATE arm in growth_annual. caplvl* = the LEVEL write, read");
    println!("exactly off Nation::capital_level_paid, shown as the annual rate it averages");
    println!("to. s* = state+private investment share of output. CAP TOTAL35 = inv35+caplvl35.");

    println!("\n=== TABLE C: full decomposition, 35 years, annual pct points ===");
    println!(
        "{:>11} {}  {:>8} {:>8} {:>8}",
        "nation",
        NAMES.iter().map(|s| format!("{:>7}", s)).collect::<Vec<_>>().join(" "),
        "SUM",
        "level",
        "actual"
    );
    for (k, (_, name)) in PANEL.iter().enumerate() {
        let m = months[k].max(1.0);
        let row: Vec<String> =
            (0..NTERM).map(|i| format!("{:>7.3}", acc[k][i] / m * 100.0)).collect();
        let sum: f64 = (0..NTERM).map(|i| acc[k][i] / m).sum::<f64>() * 100.0;
        let lvl = ann(resid[k] / m * 12.0, 1.0);
        let act = ann(med(&mut mult35[k].clone()).ln(), 35.0);
        println!("{:>11} {}  {:>8.3} {:>8.3} {:>8.3}", name, row.join(" "), sum, lvl, act);
    }
    println!("`level` is ALL off-equation level writes (trade + capital) as an annual rate.");
    println!("`actual` is the CAGR of the median 35-year multiple.\n");
}

/// HOW MUCH OF `china_growth_miracle`'s GREEN IS THE SEED WINDOW.
///
/// That test reads the median 30-year multiple of seeds 0..9 against a floor of
/// 11.0 and a per-seed floor of 6.0. This prints the same quantity on a fair
/// sample, and then asks the only question that matters about a ten-seed
/// window: over every disjoint ten-seed block in the sample, how often would the
/// bar have been red? Nothing is asserted and no bar is touched.
///
/// `cargo test --release -p spheres-sim --test capital_damage_audit china_floor_exposure -- --ignored --nocapture`
#[test]
#[ignore]
fn china_floor_exposure() {
    const N: u64 = 300;
    let mut x: Vec<f64> = vec![];
    for seed in 0..N {
        let mut w = world_1990(GameRules { seed, ..GameRules::default() });
        let g0 = w.nation(NationId::China).gdp;
        for _ in 0..M30 {
            tick_month(&mut w, &[]);
        }
        x.push(w.nation(NationId::China).gdp / g0);
    }
    let mut s = x.clone();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = s.len();
    println!("\n=== china_growth_miracle's quantity on {} seeds ===", N);
    println!("  30-year multiple: median {:.4}x   mean {:.4}x", s[n / 2], x.iter().sum::<f64>() / n as f64);
    println!("  p05 {:.3}  p25 {:.3}  p75 {:.3}  p95 {:.3}  min {:.3}  max {:.3}", s[n / 20], s[n / 4], s[n * 3 / 4], s[n * 19 / 20], s[0], s[n - 1]);
    println!("  reality 14.33x (World Bank NY.GDP.MKTP.KD, 9.28%/yr)");
    println!("  seeds below the 11.0 floor: {} of {} ({:.1}%)", s.iter().filter(|v| **v < 11.0).count(), n, s.iter().filter(|v| **v < 11.0).count() as f64 / n as f64 * 100.0);
    println!("  seeds below the 6.0 per-seed floor: {}", s.iter().filter(|v| **v < 6.0).count());
    println!("  seeds reaching reality (>= 14.33x): {}", s.iter().filter(|v| **v >= 14.33).count());
    // every disjoint ten-seed block, read exactly as the test reads seeds 0..9
    let mut red = 0usize;
    let mut blocks = 0usize;
    let mut first = f64::NAN;
    for b in 0..(n / 10) {
        let mut w: Vec<f64> = x[b * 10..b * 10 + 10].to_vec();
        w.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let m = w[w.len() / 2];
        if b == 0 {
            first = m;
        }
        blocks += 1;
        if m < 11.0 || w[0] < 6.0 {
            red += 1;
        }
    }
    println!("\n  seeds 0..9 (what the test actually reads): median {:.4}x", first);
    println!("  disjoint ten-seed blocks that would be RED: {} of {} ({:.1}%)", red, blocks, red as f64 / blocks as f64 * 100.0);
    println!("  (a bar whose verdict is a coin-flip on the seed window is not measuring growth)\n");
}

/// WHAT DRIVES THE INVESTMENT SHARE, which is what the capital LEVEL block is
/// paid on. The block's FORM is a level and is not in question; what it is
/// handed is. Median over 40 seeds, five-yearly.
///
/// `cargo test --release -p spheres-sim --test capital_damage_audit invest_share_trajectory -- --ignored --nocapture`
#[test]
#[ignore]
fn invest_share_trajectory() {
    const N: u64 = 40;
    let cuts = [0usize, 60, 120, 180, 240, 300, 360, 420];
    let labels = ["1990", "1995", "2000", "2005", "2010", "2015", "2020", "2025"];
    let watch: Vec<(NationId, &str)> = PANEL.to_vec();
    let mut s: Vec<Vec<Vec<f64>>> = vec![vec![vec![]; cuts.len()]; watch.len()];
    let mut d: Vec<Vec<Vec<f64>>> = vec![vec![vec![]; cuts.len()]; watch.len()];
    let mut has_base = vec![0.0f64; watch.len()];
    for seed in 0..N {
        let mut w = world_1990(GameRules { seed, ..GameRules::default() });
        let mut ci = 0usize;
        for month in 0..=M35 {
            if ci < cuts.len() && month == cuts[ci] {
                for (k, (id, _)) in watch.iter().enumerate() {
                    if w.nation(*id).alive {
                        let n = w.nation(*id);
                        s[k][ci].push(n.state_invest_gdp + n.priv_invest_gdp);
                        d[k][ci].push(n.debt_gdp);
                        if month == 0 && n.state_invest_1990.is_some() {
                            has_base[k] += 1.0;
                        }
                    }
                }
                ci += 1;
            }
            if month < M35 {
                tick_month(&mut w, &[]);
            }
        }
    }
    println!("\n=== investment share s = state+private, median of {} seeds ===", N);
    print!("{:>11}", "nation");
    for l in labels {
        print!(" {:>7}", l);
    }
    println!("   {:>8} {:>12}", "d ln s", "cap lvl pt/yr");
    for (k, (_, name)) in watch.iter().enumerate() {
        print!("{:>11}", name);
        for c in 0..cuts.len() {
            print!(" {:>7.4}", med(&mut s[k][c].clone()));
        }
        let a = med(&mut s[k][0].clone());
        let b = med(&mut s[k][cuts.len() - 1].clone());
        let dl = 0.49 * (b / a).ln();
        println!("   {:>+8.4} {:>+12.4}", (b / a).ln(), ((dl / 35.0).exp() - 1.0) * 100.0);
    }
    println!("\n=== debt/GDP, median. politics.rs cuts state_invest 0.5%/mo above 0.85 ===");
    print!("{:>11}", "nation");
    for l in labels {
        print!(" {:>7}", l);
    }
    println!("   {:>12}", "has 1990 base");
    for (k, (_, name)) in watch.iter().enumerate() {
        print!("{:>11}", name);
        for c in 0..cuts.len() {
            print!(" {:>7.3}", med(&mut d[k][c].clone()));
        }
        println!("   {:>11.0}%", has_base[k] / N as f64 * 100.0);
    }
    println!("\n`d ln s` x CAPITAL_ELASTICITY(0.49), annualised over 35y, is the whole");
    println!("capital LEVEL payment. It is a LEVEL and not a rate; what is in question");
    println!("is the SIGN of the share series it is handed.\n");
}

// =====================================================================
// REPAIR-PASS PROBE. Added 2026-08-31 by the capital-repair session.
// Asserts nothing, moves no bar, changes no model code. It integrates
// SEVERAL CANDIDATE capital-output-ratio dynamics off the LIVE board's own
// `s` and `g` series, so the choice between them is measured rather than
// argued. Nothing here feeds back into the sim.
//
//   ln Y = a/(1-a) * ln(K/Y) + ln(A*L)     <- the identity the channel serves
//   d(K/Y)/dt = s - (delta + g)*(K/Y)      <- the law of motion, candidates
//                                             differ only in what `g` is
//
// `cargo test --release -p spheres-sim --test capital_damage_audit capital_shape_probe -- --ignored --nocapture`
// =====================================================================
const NCAND: usize = 6;
const CAND: [&str; NCAND] =
    ["g=0", "g=.03", "g=frnt+n", "g=pot_exK", "g=real", "g=.03,x2"];

#[test]
#[ignore]
fn capital_shape_probe() {
    const N: u64 = 30;
    const DELTA: f64 = 0.05;
    const ALPHA_OVER: f64 = 0.49;
    const FRONTIER: f64 = 0.011;
    let np = PANEL.len();

    // per nation, per candidate: median over seeds of 0.49*ln(kappa_T/kappa_0)
    let mut l30: Vec<Vec<Vec<f64>>> = vec![vec![vec![]; NCAND]; np];
    let mut l35: Vec<Vec<Vec<f64>>> = vec![vec![vec![]; NCAND]; np];
    // what HEAD actually paid, same seeds
    let mut h30: Vec<Vec<f64>> = vec![vec![]; np];
    let mut h35: Vec<Vec<f64>> = vec![vec![]; np];
    // decade shape of realised growth
    let mut dec: Vec<Vec<Vec<f64>>> = vec![vec![vec![]; 4]; np];

    for seed in 0..N {
        let mut w = world_1990(GameRules { seed, ..GameRules::default() });
        let mut kap = vec![[0.0f64; NCAND]; np];
        let mut kap0 = vec![[0.0f64; NCAND]; np];
        let mut seeded = vec![false; np];
        let mut cap_base: Vec<Option<f64>> = vec![None; np];
        let mut decacc = vec![[0.0f64; 4]; np];
        let mut decn = vec![[0.0f64; 4]; np];

        for month in 0..M35 {
            for (k, (id, _)) in PANEL.iter().enumerate() {
                if !w.nation(*id).alive {
                    continue;
                }
                let n = w.nation(*id);
                let s = n.state_invest_gdp + n.priv_invest_gdp;
                let gdp_pc = n.gdp * 1000.0 / n.population;
                let dev = (gdp_pc / 24000.0).min(1.0);
                let pop = population_growth(n);
                let gs = [
                    0.0,
                    0.03,
                    FRONTIER + pop,
                    n.tfp_trend + (1.0 - dev) * 0.020 + pop * 0.60,
                    n.growth_last,
                    0.03,
                ];
                if !seeded[k] {
                    for c in 0..NCAND {
                        kap[k][c] = s / (DELTA + gs[c]).max(0.005);
                        kap0[k][c] = kap[k][c];
                    }
                    seeded[k] = true;
                }
                for c in 0..NCAND {
                    // candidate 5 doubles the speed of the ratio's response,
                    // a control on how much of the answer is the LAG
                    let mult = if c == 5 { 2.0 } else { 1.0 };
                    let dk = (s - (DELTA + gs[c]).max(0.005) * kap[k][c]) * mult / 12.0;
                    kap[k][c] = (kap[k][c] + dk).max(0.05);
                }
                let d = (month / 120).min(3);
                decacc[k][d] += n.growth_last;
                decn[k][d] += 1.0;
            }
            tick_month(&mut w, &[]);
            for (k, (id, _)) in PANEL.iter().enumerate() {
                if w.nation(*id).alive && cap_base[k].is_none() {
                    cap_base[k] = Some(w.nation(*id).capital_level_paid.unwrap_or(0.0));
                }
            }
            if month + 1 == M30 {
                for (k, (id, _)) in PANEL.iter().enumerate() {
                    if !w.nation(*id).alive {
                        continue;
                    }
                    for c in 0..NCAND {
                        l30[k][c].push(ALPHA_OVER * (kap[k][c] / kap0[k][c]).ln());
                    }
                    h30[k].push(
                        w.nation(*id).capital_level_paid.unwrap_or(0.0)
                            - cap_base[k].unwrap_or(0.0),
                    );
                }
            }
        }
        for (k, (id, _)) in PANEL.iter().enumerate() {
            if !w.nation(*id).alive {
                continue;
            }
            for c in 0..NCAND {
                l35[k][c].push(ALPHA_OVER * (kap[k][c] / kap0[k][c]).ln());
            }
            h35[k].push(w.nation(*id).capital_level_paid.unwrap_or(0.0) - cap_base[k].unwrap_or(0.0));
            for d in 0..4 {
                if decn[k][d] > 0.0 {
                    dec[k][d].push(decacc[k][d] / decn[k][d] * 100.0);
                }
            }
        }
    }

    let ann = |ln: f64, yrs: f64| ((ln / yrs).exp() - 1.0) * 100.0;
    println!("\n############ CAPITAL-SHAPE PROBE — {} seeds ############", N);
    println!("all figures are the LEVEL the capital channel would have paid,");
    println!("shown as the annual rate it averages to over the window (pt/yr).\n");

    for (win, tbl, yrs) in [("30-YEAR", &l30, 30.0), ("35-YEAR", &l35, 35.0)] {
        let head = if yrs == 30.0 { &h30 } else { &h35 };
        println!("=== {} WINDOW ===", win);
        print!("{:>11} {:>9}", "nation", "HEAD");
        for c in CAND.iter() {
            print!(" {:>9}", c);
        }
        println!();
        for (k, (_, name)) in PANEL.iter().enumerate() {
            print!("{:>11} {:>+9.3}", name, ann(med(&mut head[k].clone()), yrs));
            for c in 0..NCAND {
                print!(" {:>+9.3}", ann(med(&mut tbl[k][c].clone()), yrs));
            }
            println!();
        }
        println!();
    }

    println!("=== realised growth by decade, %/yr (median over seeds) ===");
    println!("{:>11} {:>9} {:>9} {:>9} {:>9}", "nation", "1990s", "2000s", "2010s", "2020-25");
    for (k, (_, name)) in PANEL.iter().enumerate() {
        print!("{:>11}", name);
        for d in 0..4 {
            print!(" {:>9.3}", med(&mut dec[k][d].clone()));
        }
        println!();
    }
    println!();
}

/// SAFETY READOUT for the repaired capital channel, over the WHOLE roster
/// rather than the fourteen-nation panel. Three questions, none of them
/// asserted here: does the level block's `gap` clamp ever bind, how negative
/// does the repaired arm ever get on a real board, and did the reshape kill
/// anybody who used to live.
///
/// `cargo test --release -p spheres-sim --test capital_damage_audit roster_safety_readout -- --ignored --nocapture`
#[test]
#[ignore]
fn roster_safety_readout() {
    const N: u64 = 20;
    const REPL: f64 = 0.125;
    let mut worst_gap = 0.0f64;
    let mut worst_arm = 0.0f64;
    let mut worst_arm_who = String::new();
    let mut lowest_s = 1.0f64;
    let mut lowest_s_who = String::new();
    let mut alive_end = 0usize;
    let mut alive_start = 0usize;
    let mut negative_arm_nations: std::collections::BTreeSet<String> = Default::default();

    for seed in 0..N {
        let mut w = world_1990(GameRules { seed, ..GameRules::default() });
        if seed == 0 {
            alive_start = w.nations.iter().filter(|n| n.alive).count();
        }
        for _ in 0..M35 {
            for n in w.nations.iter() {
                if !n.alive {
                    continue;
                }
                let s = n.state_invest_gdp + n.priv_invest_gdp;
                let dev = (n.gdp * 1000.0 / n.population / 24000.0).min(1.0);
                let arm = (s.max(0.0) - REPL) * (0.20 / (0.20 - REPL)) * 0.080 * (1.0 - dev);
                if arm < worst_arm {
                    worst_arm = arm;
                    worst_arm_who = format!("{:?} s={:.4} dev={:.3}", n.id, s, dev);
                }
                if arm < 0.0 {
                    negative_arm_nations.insert(format!("{:?}", n.id));
                }
                if s < lowest_s {
                    lowest_s = s;
                    lowest_s_who = format!("{:?}", n.id);
                }
                if let Some(paid) = n.capital_level_paid {
                    let entitled = 0.49 * (s.max(1e-6) / 0.20).ln();
                    let gap = (entitled - paid) / 0.49;
                    if gap.abs() > worst_gap {
                        worst_gap = gap.abs();
                    }
                }
            }
            tick_month(&mut w, &[]);
        }
        if seed == 0 {
            alive_end = w.nations.iter().filter(|n| n.alive).count();
        }
    }
    println!("\n=== ROSTER SAFETY, {} seeds x 35 years ===", N);
    println!("worst |gap| reached in the level block : {:.4}   (clamp is 2.0 — binds: {})", worst_gap, worst_gap >= 2.0);
    println!("most negative invest_effect reached    : {:+.5} /yr  ({})", worst_arm, worst_arm_who);
    println!("theoretical floor at s=0              : {:+.5} /yr", (0.0 - REPL) * (0.20 / (0.20 - REPL)) * 0.080);
    println!("lowest investment share seen          : {:.4}  ({})", lowest_s, lowest_s_who);
    println!("nations ever paid a NEGATIVE arm ({}) : {:?}", negative_arm_nations.len(), negative_arm_nations);
    println!("alive at 1990 / alive at 2025, seed 0 : {} / {}\n", alive_start, alive_end);
}
