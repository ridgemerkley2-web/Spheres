//! AUDIT INSTRUMENT — measurement only, `#[ignore]`d, asserts nothing, moves no
//! bar. Added 2026-08-31 by the capital-repair session.
//!
//! It replicates EXACTLY the A/B that `tech::tests::the_1990_endowment_does_not
//! _move_year_one_growth` runs — same twenty nations, same rebase order, same
//! twelve months, same `ai_aggression = 0.0` — and PRINTS the per-nation margin
//! that test asserts at 1.0e-4, instead of asserting it. The tracked test prints
//! nothing when it passes, so there is no other way to see how much headroom it
//! had before a change and how much it has after one.
//!
//! `cargo test --release -p spheres-sim --test endowment_margin_probe -- --ignored --nocapture`

use spheres_sim::init::world_1990;
use spheres_sim::tech;
use spheres_sim::world::{GameRules, Nation, WorldState};

fn pool_1990() -> Vec<u16> {
    tech::registry()
        .iter()
        .enumerate()
        .filter(|(_, d)| d.earliest_year <= 1990)
        .map(|(i, _)| i as u16)
        .collect()
}

fn transcribed_trends() -> Vec<f64> {
    spheres_sim::data::parse_nations(spheres_sim::data::EMBEDDED_NATIONS)
        .expect("the roster parses")
        .iter()
        .map(|r| r.economy.tfp_trend)
        .collect()
}

fn largest(w: &WorldState, top: usize) -> Vec<usize> {
    let mut order: Vec<usize> = (0..w.nations.len()).collect();
    order.sort_by(|a, b| {
        w.nations[*b].gdp.partial_cmp(&w.nations[*a].gdp).expect("finite 1990 GDP").then(a.cmp(b))
    });
    order.into_iter().take(top).collect()
}

fn endow_top(w: &mut WorldState, transcribed: &[f64], top: usize) {
    let pool = pool_1990();
    let take = largest(w, top);
    for (k, n) in w.nations.iter_mut().enumerate() {
        if take.contains(&k) {
            n.tech.grant_1990(&pool);
        }
    }
    let reference = tech::world_reference(&w.nations);
    let frontier_1990 = tech::world_frontier(&w.nations);
    for (k, n) in w.nations.iter_mut().enumerate() {
        tech::rebase_to_transcribed(n, transcribed[k], reference, frontier_1990);
    }
}

fn dev(n: &Nation) -> f64 {
    (n.gdp * 1000.0 / n.population / 24000.0).min(1.0)
}

#[test]
#[ignore]
fn endowment_margin_probe() {
    let transcribed = transcribed_trends();
    let mut control = world_1990(GameRules::default());
    control.rules.ai_aggression = 0.0;
    let mut granted = world_1990(GameRules::default());
    granted.rules.ai_aggression = 0.0;

    let endowed = largest(&granted, 20);
    endow_top(&mut granted, &transcribed, 20);

    for _ in 0..12 {
        spheres_sim::tick_month(&mut control, &[]);
        spheres_sim::tick_month(&mut granted, &[]);
    }
    println!("\nrng streams identical: {}", control.rng == granted.rng);
    println!(
        "\n{:>14} {:>11} {:>11} {:>11} {:>11} {:>8} {:>8}",
        "nation", "ungranted", "granted", "dgrowth", "dgdp", "1-dev", "s"
    );
    let mut worst_g = 0.0f64;
    let mut worst_d = 0.0f64;
    let mut rows: Vec<(f64, String)> = vec![];
    for k in endowed {
        let a = &control.nations[k];
        let b = &granted.nations[k];
        if !a.alive || !b.alive {
            continue;
        }
        let dg = (b.growth_last - a.growth_last).abs();
        let dd = (b.gdp / a.gdp - 1.0).abs();
        worst_g = worst_g.max(dg);
        worst_d = worst_d.max(dd);
        rows.push((
            dg,
            format!(
                "{:>14} {:>11.6} {:>11.6} {:>11.3e} {:>11.3e} {:>8.4} {:>8.4}",
                format!("{:?}", a.id),
                a.growth_last,
                b.growth_last,
                dg,
                dd,
                1.0 - dev(a),
                a.state_invest_gdp + a.priv_invest_gdp
            ),
        ));
    }
    rows.sort_by(|x, y| y.0.partial_cmp(&x.0).unwrap());
    for (_, r) in &rows {
        println!("{}", r);
    }
    println!(
        "\nworst dgrowth {:.4e} against the tracked bar 1.0e-4  ({:.1}% of it)",
        worst_g,
        worst_g / 1.0e-4 * 100.0
    );
    println!(
        "worst dgdp    {:.4e} against the tracked bar 2.0e-4  ({:.1}% of it)\n",
        worst_d,
        worst_d / 2.0e-4 * 100.0
    );
}
