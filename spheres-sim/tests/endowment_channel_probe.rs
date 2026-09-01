//! AUDIT INSTRUMENT — measurement only, `#[ignore]`d, asserts nothing, moves no
//! bar. Added 2026-09-01 by the sim-and-web fixer, investigating BUGS.md E-2.
//!
//! `endowment_margin_probe` beside it says WHICH nation is worst and by how
//! much. This says WHICH TERM the difference is in, which is the question E-2
//! could not answer and the reason it recorded a mechanism it had not measured.
//!
//! `apply_bonuses` assembles the trend from exactly three terms
//!
//!     tfp_trend = tfp_base + (saturated_tech_tfp - reference) + min(adoption, MAX)
//!
//! and `economy::tick` turns that trend into growth alongside the capital,
//! catchup, labour, demand and drag channels. This probe runs the SAME A/B the
//! tracked test runs — same twenty nations, same rebase order, same twelve
//! months, same `ai_aggression = 0.0` — and prints each term separately for both
//! worlds, so the residual can be attributed instead of guessed at.
//!
//! `cargo test --release -p spheres-sim --test endowment_channel_probe -- --ignored --nocapture`

use spheres_sim::init::world_1990;
use spheres_sim::tech;
use spheres_sim::world::{GameRules, Nation, NationId, WorldState};

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

/// The three terms `apply_bonuses` assembles, read off the nation rather than
/// recomputed, plus the world quantities they are measured against.
struct Terms {
    tfp_base: f64,
    sat: f64,
    reference: f64,
    /// What is left of the trend once the first two terms are removed: this is
    /// `min(adoption, ADOPTION_MAX)` by construction, without this file needing
    /// a copy of ADOPTION_PER_TECH, TACIT or ADOPTION_MAX.
    adoption: f64,
    trend: f64,
    count: usize,
    absorption: f64,
}

fn terms(w: &WorldState, n: &Nation) -> Terms {
    let reference = tech::world_reference(&w.nations);
    let sat = tech::saturated_tech_tfp(n);
    let tfp_base = n.tech.tfp_base;
    Terms {
        tfp_base,
        sat,
        reference,
        adoption: n.tfp_trend - tfp_base - (sat - reference),
        trend: n.tfp_trend,
        count: n.tech.count(),
        absorption: n.tech.absorption_rate,
    }
}

fn find<'a>(w: &'a WorldState, id: NationId) -> &'a Nation {
    w.nations.iter().find(|n| n.id == id).expect("seated in 1990")
}

#[test]
#[ignore]
fn endowment_channel_probe() {
    let transcribed = transcribed_trends();
    let mut control = world_1990(GameRules::default());
    control.rules.ai_aggression = 0.0;
    let mut granted = world_1990(GameRules::default());
    granted.rules.ai_aggression = 0.0;
    endow_top(&mut granted, &transcribed, 20);

    // Belgium is the nation the tracked test goes red on; the USA is the
    // frontier nation whose credit is zero and whose residual is a fifth of
    // everyone else's; Switzerland, Japan and Sweden sit at dev = 1.0, so their
    // capital rate arm `net_intensity * 0.080 * (1 - dev)` is IDENTICALLY ZERO
    // and they are the controls for E-2's claim that the residual is amplified
    // in proportion to that arm; China and India carry the largest arms on the
    // board and are the other end of the same test.
    let watch = [
        NationId::Belgium,
        NationId::Switzerland,
        NationId::Japan,
        NationId::Sweden,
        NationId::China,
        NationId::India,
        NationId::USA,
    ];
    // The month-1 readings, kept so the twelve-month movement can be attributed
    // to the terms that actually moved rather than to the levels they sit at.
    let mut first: Vec<(NationId, f64, f64, f64)> = vec![];

    println!("\n=== PER-MONTH, granted MINUS ungranted, by term ===");
    println!(
        "{:>6} {:>12} {:>11} {:>11} {:>11} {:>11} {:>11} {:>11}",
        "month", "nation", "d.tfp_base", "d.sat", "d.reference", "d.adoption", "d.trend", "d.growth"
    );
    for m in 1..=12 {
        spheres_sim::tick_month(&mut control, &[]);
        spheres_sim::tick_month(&mut granted, &[]);
        if m == 1 {
            for id in watch {
                let a = terms(&control, find(&control, id));
                let b = terms(&granted, find(&granted, id));
                first.push((
                    id,
                    b.trend - a.trend,
                    b.reference - a.reference,
                    b.adoption - a.adoption,
                ));
            }
        }
        if m != 1 && m != 2 && m != 6 && m != 12 {
            continue;
        }
        for id in watch {
            let a = terms(&control, find(&control, id));
            let b = terms(&granted, find(&granted, id));
            println!(
                "{:>6} {:>12} {:>11.3e} {:>11.3e} {:>11.3e} {:>11.3e} {:>11.3e} {:>11.3e}",
                m,
                format!("{:?}", id),
                b.tfp_base - a.tfp_base,
                b.sat - a.sat,
                b.reference - a.reference,
                b.adoption - a.adoption,
                b.trend - a.trend,
                find(&granted, id).growth_last - find(&control, id).growth_last,
            );
        }
        println!();
    }

    println!("=== LEVELS AT MONTH 12 ===");
    println!(
        "{:>12} {:>4} {:>9} {:>10} {:>10} {:>10} {:>10} {:>10} {:>8}",
        "nation", "wld", "count", "tfp_base", "sat", "reference", "adoption", "trend", "absorb"
    );
    for id in watch {
        for (label, w) in [("ctrl", &control), ("gran", &granted)] {
            let t = terms(w, find(w, id));
            println!(
                "{:>12} {:>4} {:>9} {:>10.6} {:>10.6} {:>10.6} {:>10.6} {:>10.6} {:>8.3}",
                format!("{:?}", id),
                label,
                t.count,
                t.tfp_base,
                t.sat,
                t.reference,
                t.adoption,
                t.trend,
                t.absorption,
            );
        }
    }

    // The world reference is a GDP-weighted mean over the WHOLE roster, so it is
    // one number per world per month and every nation on the board reads it.
    // If the residual is the reference drifting, it is the same subtraction for
    // all 137 of them and it is not a per-nation double payment at all.
    println!(
        "\nreference at month 12: control {:.8}  granted {:.8}  d {:.4e}",
        tech::world_reference(&control.nations),
        tech::world_reference(&granted.nations),
        tech::world_reference(&granted.nations) - tech::world_reference(&control.nations),
    );

    // THE ACCOUNTING. `d.trend` starts at ~0 by construction — that is what the
    // rebase buys — so the whole of the residual the tracked test reads at month
    // twelve is the MOVEMENT in `d.trend`, and this says which term moved.
    // `d.tfp_base + d.sat` is a t=0 constant preserved by the revelation
    // machinery, so what is left is the reference and the adoption term.
    println!("\n=== WHAT MOVED, month 1 -> month 12 ===");
    println!(
        "{:>12} {:>12} {:>12} {:>12} {:>12} {:>10}",
        "nation", "d.trend m1", "d.trend m12", "-D(d.ref)", "D(d.adopt)", "explained"
    );
    for (id, t1, r1, a1) in &first {
        let a = terms(&control, find(&control, *id));
        let b = terms(&granted, find(&granted, *id));
        let t12 = b.trend - a.trend;
        let dref = -((b.reference - a.reference) - r1);
        let dad = (b.adoption - a.adoption) - a1;
        println!(
            "{:>12} {:>12.3e} {:>12.3e} {:>12.3e} {:>12.3e} {:>9.1}%",
            format!("{:?}", id),
            t1,
            t12,
            dref,
            dad,
            (dref + dad) / (t12 - t1) * 100.0,
        );
    }

    // What the capital arm is doing, since E-2 names it as the amplifier. A
    // nation at dev = 1.0 has `invest_effect = net_intensity * 0.080 * (1 - dev)`
    // identically zero, so on E-2's mechanism its residual should be ~zero too.
    println!("\n=== THE CAPITAL CHANNEL, month 12 ===");
    println!(
        "{:>12} {:>10} {:>10} {:>12} {:>12} {:>10} {:>11}",
        "nation", "1-dev", "s", "cap_paid_c", "cap_paid_g", "identical", "dgrowth"
    );
    for id in watch {
        let a = find(&control, id);
        let b = find(&granted, id);
        let dev = (a.gdp * 1000.0 / a.population / 24000.0).min(1.0);
        let (pc, pg) =
            (a.capital_level_paid.unwrap_or(f64::NAN), b.capital_level_paid.unwrap_or(f64::NAN));
        println!(
            "{:>12} {:>10.4} {:>10.4} {:>12.8} {:>12.8} {:>10} {:>11.3e}",
            format!("{:?}", id),
            1.0 - dev,
            a.state_invest_gdp + a.priv_invest_gdp,
            pc,
            pg,
            pc == pg,
            b.growth_last - a.growth_last,
        );
    }

    // And the arithmetic the tracked test asserts, restated so this file's
    // reading can be checked against that one without running both.
    let a = find(&control, NationId::Belgium);
    let b = find(&granted, NationId::Belgium);
    println!(
        "\nBelgium dgrowth {:.4e} (bar 1.0e-4)   dgdp {:.4e} (bar 2.0e-4)\n",
        b.growth_last - a.growth_last,
        b.gdp / a.gdp - 1.0,
    );
}
