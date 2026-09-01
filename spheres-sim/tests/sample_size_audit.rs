//! THE SAMPLE-SIZE AUDIT — iron rule 7's instrument. Added 2026-08-31 by the
//! session that wrote that rule, on Ridge's ruling 3 of the same date.
//!
//! MEASUREMENT ONLY. Every test here is `#[ignore]`d, asserts nothing, moves no
//! bar and touches no sim state. It exists because rule 7 requires the sample
//! size of a calibration bar to be derived from THAT TEST'S OWN measured
//! per-seed variance, and a rule whose arithmetic nobody can re-run is a rule
//! that will be inherited on trust and then quietly ignored.
//!
//! What it does: it re-reads, at a much wider sample, exactly the per-seed
//! quantity each tracked calibration bar counts — the same flags, the same
//! headline strings, the same month checkpoints — and prints, for each bar:
//! the measured rate or distribution, the per-seed variance, the false-red
//! probability AT THE BAR'S CURRENT SAMPLE, and the smallest sample that puts
//! that probability under 1% with the bar itself unmoved.
//!
//! It deliberately does NOT print a recommendation. Rule 7 sizes the sample;
//! iron rule 5 still says the bar does not move, and nothing here may be read
//! as licence to move one.
//!
//! ```text
//! cargo test --release -p spheres-sim --test sample_size_audit -- --ignored --nocapture
//! ```
//!
//! ==========================================================================
//! THE TABLE, MEASURED 2026-08-31 ON THE POST-CAPITAL-REPAIR TREE
//! ==========================================================================
//!
//! Rule 7 wants the required n recorded beside each bar. The bars that predate
//! the rule do not carry it yet, so this is where it lives until each one is
//! next touched. `P` is the false-red probability AT THE BAR'S OWN SAMPLE.
//! Sources: `panel_variance` and `statecraft_variance` below,
//! `gulf_war_incidence_scan` and `conquest_size_rule_scan` in lib.rs.
//!
//! ```text
//!   test (bar)                                    n     p / stat    var      P       need  ok
//!   ------------------------------------------------------------------------------------------
//!   gulf_war_emerges (>= n/2)                    200    0.6150    0.2368   0.0004     97   yes
//!   a_large_nation_... (refusals >= 15)          100    0.404/sd  0.5075   0.0002    100   yes
//!   a_dead_nation_... (annexations >= 1)         240    0.0417/sd 0.0399   0.00004   109   yes
//!   a_dead_nation_... (annexations <= 40)        240    mean 10.0 sd 3.1   ~0        240   yes
//!   china_growth_miracle (median in 11.0..19.0)  100    med 14.45 1.935    0.0000     10*  yes
//!   a_burned_aggressor... (burned >= 4)           20    0.5700    0.2451   0.0001     14   yes
//!   a_pact_drags... (dragged >= 3)                12    0.4800    0.2496   0.0267     14   NO
//!   a_pact_drags... (dragged < 12, "not all")     12    0.4800    0.2496   0.0001      7   yes
//!   guarantees... (abandoned >= 1)                40    0.1955    0.1573   0.0002     22   yes
//!   guarantees... (honoured > 2*abandoned)        40    0.1955    0.1573   0.0160     49   NO
//!   a_parliament... (refused >= 4)                40    0.3395    0.2242   0.0001     26   yes
//!   a_parliament... (granted >= 4)                40    0.6605    0.2242   0.0000     11   yes
//!   superpowers_compete... (contested >= 7)       12    0.9600    0.0384   0.000003    9   yes
//!   some_wars_end_at_the_table (settled >= 3)     12    1.0000    0.0000   0.0000      3   yes
//!   ussr_collapses_in_the_nineties (>= 6)         10    1.0000    0.0000   0.0000      1   yes
//!   yugoslavia_comes_apart... (>= 8)              10    1.0000    0.0000   0.0000      1   yes
//!   ukraine_leaves_the_union... (born >= 6)       10    1.0000    0.0000   0.0000      1   yes
//!   brazil_grinds_down (burning at 18m >= 8)      10    1.0000    0.0000   0.0000      1   yes
//!   brazil_grinds_down (tamed by 1999 >= 8)       10    1.0000    0.0000   0.0000      1   yes
//!   desert_storm_is_quick (quick >= 6 of 8)        8    1.0000    0.0000   0.0000      1   yes
//!   slovenia_escapes... (bosnia > slovenia*3)     10    0.63/sd   0.3131   0.0001     10   yes
//!   the_frontier_does_not_run_away (< 4.0)        10    mean 2.00 0.0036   0.0000      1   yes
//!   the_frontier_does_not_run_away (> 0.5)        10    mean 1.31 0.0056   0.0000      1   yes
//! ```
//!
//! `*` china_growth_miracle needs only ten seeds to hold rule 7's false-red
//! ceiling. It carries a hundred because of rule 7's POWER clause and Ridge's
//! ruling 3, which names n >= 100 for it: ten seeds is the sample that hid a
//! 22.5% regression behind a green light, at a detection rate of 37.6%.
//!
//! THE TWO REDS IN THAT COLUMN ARE REPORTED, NOT REPAIRED, and neither is a
//! model defect:
//!
//!   - `a_pact_drags_a_great_power_into_a_war_it_did_not_start` reads
//!     `(3..12).contains(&dragged)` over twelve seeds. The lower arm sits at
//!     2.67%, and the derivation asks for fourteen. It cannot be widened
//!     without rewriting the upper bound, because the literal 12 IS the sample
//!     size and means "not every seed" — re-expressing a bar is Ridge's call
//!     under iron rule 5, not a session's.
//!
//!   - `guarantees_are_usually_but_not_always_honoured` reads
//!     `honoured > abandoned * 2` over forty, at 1.60% against a derivation
//!     asking for forty-nine. It ALSO cannot be widened as written, for a
//!     reason only the wide sample makes visible: `force_pact` proposes until
//!     Kuwait consents and `.unwrap()`s every proposal, and a proposal costs
//!     standing. On 56 of 2000 seeds (2.8%) Washington runs out and the test
//!     PANICS rather than fails. Seeds 0..40 happen to contain none, but any
//!     window past forty has a 1 - 0.972^n chance of hitting one — 68% at
//!     n = 40, 75% at n = 49. The fix is to `force_pact` (replenish the way
//!     `force_access` already does), not to this bar.
//!
//! THREE BARS ARE COMPLIANT AND POWERLESS, which rule 7's power clause says to
//! record rather than believe: `ussr_collapses` (>= 6 of 10 against a measured
//! rate of 1.000), `yugoslavia_comes_apart` (>= 8 of 10, same) and
//! `the_frontier_does_not_run_away`, whose 4.0% ceiling now stands 1.9 points
//! above the fastest mature economy in a hundred seeds and would not notice
//! most of a doubling of the frontier's growth rate. None of them can be
//! tightened by a session — that is a calibration decision.
//!
//! NOT COVERED HERE, and deliberately: the INVARIANTS (`a_century_holds_
//! together`, `stable_democracies_never_hyperinflate`, `economic_invariants_50_
//! years`, `every_nation_has_a_home`, `expanded_roster_holds_the_economic_
//! invariants`, `the_war_layer_holds_its_own_invariants`, `a_settled_peace_
//! moves_the_biggest_district`, `a_dissolution_makes_no_choice_between_its_
//! successors`, `ussr_dissolution_hands_ukraine_its_districts`, `saves_name_
//! nations_rather_than_numbering_them`, `nuclear_taboo_holds`, `mature_
//! economies_do_not_run_hot`, `convergence_outruns_the_frontier`, `an_
//! endowment_cannot_pay_the_diffusion_floor`, `a_poor_nation_still_picks_up_
//! what_everyone_has`), which assert a universal claim per seed and so cannot
//! false-red from a small sample; and the PAIRED CONTRASTS (`aid_props_up_a_
//! client_regime`, `arms_transfers_build_a_client_army`, `a_trade_agreement_
//! lifts_the_smaller_partner`, `sanctions_cost_the_target_real_growth`,
//! `nigeria_has_a_good_gulf_war`, `embargo_starves_the_aggressor`), which run
//! two worlds off the SAME seed with `ai_aggression = 0.0` and the player
//! frozen, so the quantity they read is a difference whose seed variance is
//! suppressed by construction rather than by sample size.

use spheres_sim::nations::patrons;
use spheres_sim::world::{GameRules, NationId};
use spheres_sim::{apply_command, exact, init::world_1990, theatre, tick_month, war, Command};

// ---------------------------------------------------------------------------
// SAMPLE SIZES. Chosen for what the wall clock will bear, and stated so the
// precision of every figure below can be judged rather than assumed.
// ---------------------------------------------------------------------------

/// The long sweep: 35 years each, which is the longest run any bar below reads.
/// A rate measured on this has a standard error of about 5 points.
const PANEL_N: u64 = 100;

/// The war-incidence sweeps, which need their own world setup.
const WAR_N: u64 = 60;

/// The pact and access bars tick the world zero times, so they are free.
const CHEAP_N: u64 = 2000;

// ---------------------------------------------------------------------------
// THE ARITHMETIC OF RULE 7
// ---------------------------------------------------------------------------

/// ln(n!) for 0..=n. `exact::ln`, not the platform's, so a figure printed here
/// is the same figure on every machine — `exact.rs`'s ban only walks `src/`,
/// but a measurement that decides a sample size should be as reproducible as
/// the sim it measures.
fn ln_fact(n: usize) -> Vec<f64> {
    let mut v = vec![0.0f64; n + 1];
    for i in 1..=n {
        v[i] = v[i - 1] + exact::ln(i as f64);
    }
    v
}

/// P(Binomial(n, p) <= k), summed in log space so a sample of two thousand
/// does not underflow the first term.
fn binom_le(n: usize, k: isize, p: f64, lf: &[f64]) -> f64 {
    if k < 0 {
        return 0.0;
    }
    let k = (k as usize).min(n);
    if p <= 0.0 {
        return 1.0;
    }
    if p >= 1.0 {
        return if k >= n { 1.0 } else { 0.0 };
    }
    let (lp, lq) = (exact::ln(p), exact::ln(1.0 - p));
    let mut acc = 0.0;
    for i in 0..=k {
        let l = lf[n] - lf[i] - lf[n - i] + i as f64 * lp + (n - i) as f64 * lq;
        acc += exact::exp(l);
    }
    acc.min(1.0)
}

fn binom_ge(n: usize, k: usize, p: f64, lf: &[f64]) -> f64 {
    1.0 - binom_le(n, k as isize - 1, p, lf)
}

/// Smallest n for which a FIXED floor of `k` reds under 1% of healthy worlds.
fn need_fixed_floor(p: f64, k: usize, cap: usize) -> Option<usize> {
    let lf = ln_fact(cap);
    (1..=cap).find(|&n| binom_le(n, k as isize - 1, p, &lf) < 0.01)
}

/// Smallest n for which a floor that SCALES with the sample — "a majority", a
/// fixed fraction `frac` of the seeds — reds under 1% of healthy worlds.
fn need_scaled_floor(p: f64, frac: f64, cap: usize) -> Option<usize> {
    let lf = ln_fact(cap);
    (1..=cap).find(|&n| {
        let bar = (frac * n as f64).ceil() as isize;
        binom_le(n, bar - 1, p, &lf) < 0.01
    })
}

/// Smallest n for which a CEILING that scales — "fewer than `frac` of them" —
/// reds under 1% of healthy worlds.
fn need_scaled_ceiling(p: f64, frac: f64, cap: usize) -> Option<usize> {
    let lf = ln_fact(cap);
    (1..=cap).find(|&n| {
        let bar = (frac * n as f64).ceil() as usize;
        binom_ge(n, bar, p, &lf) < 0.01
    })
}

fn mean_var(v: &[f64]) -> (f64, f64) {
    let n = v.len() as f64;
    let m = v.iter().sum::<f64>() / n;
    (m, v.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / n)
}

/// A Bernoulli bar, reported in full. `p` is measured, so the variance IS
/// p(1-p) and nothing about it is assumed.
fn report_bernoulli(label: &str, hits: usize, n: usize, bar: Bar, current_n: usize) {
    let p = hits as f64 / n as f64;
    let var = p * (1.0 - p);
    let lf = ln_fact(current_n.max(1));
    let (false_red, need) = match bar {
        Bar::Floor(k) => (
            binom_le(current_n, k as isize - 1, p, &lf),
            need_fixed_floor(p, k, 20000),
        ),
        Bar::ScaledFloor(f) => {
            let k = (f * current_n as f64).ceil() as isize;
            (binom_le(current_n, k - 1, p, &lf), need_scaled_floor(p, f, 20000))
        }
        Bar::ScaledCeiling(f) => {
            let k = (f * current_n as f64).ceil() as usize;
            (binom_ge(current_n, k, p, &lf), need_scaled_ceiling(p, f, 20000))
        }
        Bar::NotAll => (
            exact::exp(current_n as f64 * exact::ln(p.max(1e-12))),
            (1..20000).find(|&n| exact::exp(n as f64 * exact::ln(p.max(1e-12))) < 0.01),
        ),
    };
    println!(
        "  {:<52} p {:.4} ({:>4}/{:<4})  var {:.4}  bar {:<22} at n={:<4} P(false red) {:>7.4}  needs n >= {}",
        label,
        p,
        hits,
        n,
        var,
        bar.show(),
        current_n,
        false_red,
        need.map(|x| x.to_string()).unwrap_or_else(|| ">20000".into())
    );
}

#[derive(Clone, Copy)]
enum Bar {
    /// "at least k of them", k fixed however wide the sample is
    Floor(usize),
    /// "at least this FRACTION of them" — a majority, three quarters
    ScaledFloor(f64),
    /// "fewer than this fraction of them"
    ScaledCeiling(f64),
    /// "not every one of them"
    NotAll,
}

impl Bar {
    fn show(&self) -> String {
        match self {
            Bar::Floor(k) => format!(">= {}", k),
            Bar::ScaledFloor(f) => format!(">= {:.0}% of n", f * 100.0),
            Bar::ScaledCeiling(f) => format!("< {:.0}% of n", f * 100.0),
            Bar::NotAll => "not all".into(),
        }
    }
}

/// A deterministic bootstrap, for the bars that read a median rather than a
/// count. Local SplitMix64 — this is a test instrument and touches no sim
/// state, so it is not the sim's RNG and iron rule 1 is not in play.
struct Boot(u64);
impl Boot {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
}

/// P(the median of `k` draws from the measured sample leaves `lo..hi`), and the
/// smallest k that puts it under 1%. The convention is the one the tested code
/// uses: the mean of the two middle order statistics for an even k.
fn median_band_risk(sample: &[f64], lo: f64, hi: f64, ks: &[usize]) {
    for &k in ks {
        let mut boot = Boot(0x5EED_5A11_1E00_0001);
        let trials = 20000;
        let mut out = 0usize;
        let mut buf = vec![0.0f64; k];
        for _ in 0..trials {
            for slot in buf.iter_mut() {
                *slot = sample[(boot.next() % sample.len() as u64) as usize];
            }
            buf.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let med = if k % 2 == 0 { (buf[k / 2 - 1] + buf[k / 2]) / 2.0 } else { buf[k / 2] };
            if med < lo || med >= hi {
                out += 1;
            }
        }
        println!(
            "    median band [{:.1}, {:.1}) at n={:<4}: P(false red) {:.4}",
            lo,
            hi,
            k,
            out as f64 / trials as f64
        );
    }
}

/// P(at least one of k draws breaks a per-seed universal arm) — the arm that
/// gets STRICTER as the sample grows, which is rule 7's trap.
fn per_seed_arm_risk(sample: &[f64], breaks: impl Fn(f64) -> bool, ks: &[usize]) {
    let rate = sample.iter().filter(|x| breaks(**x)).count() as f64 / sample.len() as f64;
    for &k in ks {
        let p_ok = exact::exp(k as f64 * exact::ln((1.0 - rate).max(1e-12)));
        println!(
            "    per-seed arm at n={:<4}: breach rate {:.4} measured, P(at least one) {:.4}",
            k,
            rate,
            1.0 - p_ok
        );
    }
}

// ---------------------------------------------------------------------------
// THE LONG SWEEP — every bar that reads a plain `world_1990(seed)` run
// ---------------------------------------------------------------------------

#[derive(Default)]
struct Row {
    china_30y: f64,
    ussr_by_132: bool,
    ussr_by_180: bool,
    yugo_by_120: bool,
    slovenia_wars: usize,
    bosnia_wars: usize,
    settled_by_360: bool,
    brazil_burning_18m: bool,
    brazil_tamed_120: bool,
    pact_drag_by_360: bool,
    burned_iraq_kuwait: bool,
    contested_by_240: bool,
    mature_fastest: f64,
    mature_slowest: f64,
}

const MATURE: [NationId; 6] = [
    NationId::USA,
    NationId::Japan,
    NationId::Germany,
    NationId::France,
    NationId::UK,
    NationId::Italy,
];

fn sweep(n: u64) -> Vec<Row> {
    let mut rows = Vec::new();
    for seed in 0..n {
        let mut w = world_1990(GameRules { seed, ..GameRules::default() });
        let china0 = w.nation(NationId::China).gdp;
        let mature0: Vec<f64> = MATURE.iter().map(|id| w.nation(*id).gdp).collect();
        let mut r = Row::default();
        for m in 0..420usize {
            let headlines = tick_month(&mut w, &[]);
            if m < 120 {
                for h in &headlines {
                    if h.starts_with("WAR:") {
                        if h.contains("Slovenia") {
                            r.slovenia_wars += 1;
                        }
                        if h.contains("Bosnia") {
                            r.bosnia_wars += 1;
                        }
                    }
                }
            }
            if m < 360 {
                if headlines
                    .iter()
                    .any(|h| h.contains("sues for peace") || h.contains("agree peace terms"))
                {
                    r.settled_by_360 = true;
                }
                for h in &headlines {
                    if h.contains("honours its defence pact")
                        && patrons().iter().any(|p| h.starts_with(p.name()))
                    {
                        r.pact_drag_by_360 = true;
                    }
                }
            }
            if m < 240 && !r.contested_by_240 {
                r.contested_by_240 = w.nations.iter().filter(|x| x.alive).any(|x| {
                    let backers = w.patrons_of(x.id);
                    backers.iter().any(|a| backers.iter().any(|b| w.relation(*a, *b) < -20.0))
                });
            }
            if m == 17 && w.nation(NationId::Brazil).inflation > 0.50 {
                r.brazil_burning_18m = true;
            }
            if m == 119 {
                r.yugo_by_120 = w.has_flag("yugoslavia_dissolved");
                r.brazil_tamed_120 = w.nation(NationId::Brazil).inflation < 0.10;
            }
            if m == 131 {
                r.ussr_by_132 = w.has_flag("ussr_dissolved");
            }
            if m == 179 {
                r.ussr_by_180 = w.has_flag("ussr_dissolved");
            }
            if m == 359 {
                r.china_30y = w.nation(NationId::China).gdp / china0;
            }
            if w.has_flag("burned_Iraq_Kuwait") {
                r.burned_iraq_kuwait = true;
            }
        }
        let cagr: Vec<f64> = MATURE
            .iter()
            .enumerate()
            .filter(|(_, id)| w.nation(**id).alive)
            .map(|(k, id)| (exact::powf(w.nation(*id).gdp / mature0[k], 1.0 / 35.0) - 1.0) * 100.0)
            .collect();
        r.mature_fastest = cagr.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        r.mature_slowest = cagr.iter().cloned().fold(f64::INFINITY, f64::min);
        rows.push(r);
    }
    rows
}

#[test]
#[ignore]
fn panel_variance() {
    let rows = sweep(PANEL_N);
    let n = rows.len();
    println!("\n=== THE LONG SWEEP: {} seeds, 35 years each ===\n", n);

    let count = |f: &dyn Fn(&Row) -> bool| rows.iter().filter(|r| f(r)).count();

    println!("-- per-seed Bernoulli bars --");
    report_bernoulli(
        "ussr_collapses_in_the_nineties (dissolved by 2001)",
        count(&|r| r.ussr_by_132),
        n,
        Bar::ScaledFloor(0.6),
        10,
    );
    report_bernoulli(
        "yugoslavia_comes_apart_in_the_nineties (by 2000)",
        count(&|r| r.yugo_by_120),
        n,
        Bar::ScaledFloor(0.8),
        10,
    );
    report_bernoulli(
        "ukraine_leaves_the_union_without_the_bomb (born)",
        count(&|r| r.ussr_by_180),
        n,
        Bar::ScaledFloor(0.6),
        10,
    );
    report_bernoulli(
        "some_wars_end_at_the_table (a settlement by 2020)",
        count(&|r| r.settled_by_360),
        n,
        Bar::Floor(3),
        12,
    );
    report_bernoulli(
        "brazil_grinds_down (still burning at 18m)",
        count(&|r| r.brazil_burning_18m),
        n,
        Bar::ScaledFloor(0.8),
        10,
    );
    report_bernoulli(
        "brazil_grinds_down (tamed by 1999)",
        count(&|r| r.brazil_tamed_120),
        n,
        Bar::ScaledFloor(0.8),
        10,
    );
    report_bernoulli(
        "a_pact_drags_a_great_power... (lower arm)",
        count(&|r| r.pact_drag_by_360),
        n,
        Bar::Floor(3),
        12,
    );
    report_bernoulli(
        "a_pact_drags_a_great_power... (upper arm, not all 12)",
        count(&|r| r.pact_drag_by_360),
        n,
        Bar::NotAll,
        12,
    );
    report_bernoulli(
        "superpowers_compete_for_the_same_clients",
        count(&|r| r.contested_by_240),
        n,
        Bar::Floor(7),
        12,
    );
    report_bernoulli(
        "a_burned_aggressor... (burned_Iraq_Kuwait written)",
        count(&|r| r.burned_iraq_kuwait),
        n,
        Bar::Floor(4),
        10,
    );

    println!("\n-- slovenia_escapes_the_wars_that_consume_bosnia (a ratio of totals, not a rate) --");
    let (sm, sv) = mean_var(&rows.iter().map(|r| r.slovenia_wars as f64).collect::<Vec<_>>());
    let (bm, bv) = mean_var(&rows.iter().map(|r| r.bosnia_wars as f64).collect::<Vec<_>>());
    println!("  slovenia wars/seed mean {:.4} var {:.4}   bosnia wars/seed mean {:.4} var {:.4}", sm, sv, bm, bv);
    {
        let mut boot = Boot(0x5EED_5A11_1E00_0002);
        for k in [10usize, 20, 40, 100] {
            let trials = 20000;
            let mut red = 0;
            for _ in 0..trials {
                let (mut s, mut b) = (0usize, 0usize);
                for _ in 0..k {
                    let r = &rows[(boot.next() % n as u64) as usize];
                    s += r.slovenia_wars;
                    b += r.bosnia_wars;
                }
                if b <= s * 3 {
                    red += 1;
                }
            }
            println!("    bar `bosnia > slovenia*3` at n={:<4}: P(false red) {:.4}", k, red as f64 / trials as f64);
        }
    }

    println!("\n-- china_growth_miracle (a median band and a per-seed floor) --");
    let china: Vec<f64> = rows.iter().map(|r| r.china_30y).collect();
    let mut sorted = china.clone();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let (cm, cv) = mean_var(&china);
    println!(
        "  30y multiple: mean {:.3} var {:.3} sd {:.3}  min {:.2} p10 {:.2} median {:.2} p90 {:.2} max {:.2}",
        cm,
        cv,
        cv.sqrt(),
        sorted[0],
        sorted[n / 10],
        (sorted[n / 2 - 1] + sorted[n / 2]) / 2.0,
        sorted[n * 9 / 10],
        sorted[n - 1]
    );
    println!("  seeds below the 11.0 band floor: {}   at or above reality's 14.33x: {}",
        china.iter().filter(|x| **x < 11.0).count(),
        china.iter().filter(|x| **x >= 14.33).count());
    median_band_risk(&china, 11.0, 19.0, &[10, 20, 40, 100, 200]);
    println!("  the per-seed floor of 6.0x, which gets STRICTER with n:");
    per_seed_arm_risk(&china, |x| x <= 6.0, &[10, 100, 200]);

    println!("\n-- the_frontier_does_not_run_away (a median AND a per-seed arm) --");
    let fastest: Vec<f64> = rows.iter().map(|r| r.mature_fastest).collect();
    let slowest: Vec<f64> = rows.iter().map(|r| r.mature_slowest).collect();
    let mut fs = fastest.clone();
    fs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut ss = slowest.clone();
    ss.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let (fm, fv) = mean_var(&fastest);
    let (slm, slv) = mean_var(&slowest);
    println!("  fastest mature CAGR: mean {:.3} var {:.4} sd {:.3}  min {:.2} median {:.2} max {:.2} (ceiling 4.0)", fm, fv, fv.sqrt(), fs[0], fs[n / 2], fs[n - 1]);
    println!("  slowest mature CAGR: mean {:.3} var {:.4} sd {:.3}  min {:.2} median {:.2} max {:.2} (floor 0.5)", slm, slv, slv.sqrt(), ss[0], ss[n / 2], ss[n - 1]);
    println!("  the ceiling's per-seed arm:");
    per_seed_arm_risk(&fastest, |x| x >= 4.0, &[10, 40, 100]);
    println!("  the floor's per-seed arm:");
    per_seed_arm_risk(&slowest, |x| x <= 0.5, &[10, 40, 100]);
}

// ---------------------------------------------------------------------------
// THE WAR SWEEPS — bars that need their own setup
// ---------------------------------------------------------------------------

#[test]
#[ignore]
fn desert_storm_variance() {
    let mut quick = 0usize;
    let mut months_all: Vec<f64> = vec![];
    for seed in 0..WAR_N {
        let mut w = world_1990(GameRules { seed, ..GameRules::default() });
        w.rules.ai_aggression = 0.0;
        war::declare_war(&mut w, NationId::Iraq, NationId::Kuwait).unwrap();
        let mut months = 0;
        for m in 0..120 {
            tick_month(&mut w, &[]);
            if w.conflict_between(NationId::Iraq, NationId::Kuwait).is_none() {
                months = m + 1;
                break;
            }
        }
        months_all.push(months as f64);
        if months > 0 && months <= 36 {
            quick += 1;
        }
    }
    println!("\n=== desert_storm_is_quick_when_they_stand_and_fight: {} seeds ===", WAR_N);
    let (m, v) = mean_var(&months_all);
    println!("  months to end: mean {:.2} var {:.2}  never-ended seeds: {}", m, v, months_all.iter().filter(|x| **x == 0.0).count());
    report_bernoulli("quick (<= 36 months)", quick, WAR_N as usize, Bar::ScaledFloor(0.75), 8);
}

#[test]
#[ignore]
fn statecraft_variance() {
    println!("\n=== the zero-tick bars: {} seeds ===", CHEAP_N);

    // guarantees_are_usually_but_not_always_honoured
    //
    // `force_pact` proposes until Kuwait says yes and `.unwrap()`s every
    // proposal, and a proposal costs standing. Over forty seeds that never runs
    // out; over two thousand it does, and a seed where it does is a seed on
    // which the TRACKED test would panic rather than fail — so the top-ups are
    // counted and reported, because that count is the real obstacle to widening
    // this bar and it is not visible at n=40.
    let (mut honoured, mut abandoned, mut topups) = (0usize, 0usize, 0usize);
    for seed in 0..CHEAP_N {
        let mut w = world_1990(GameRules { seed, ..GameRules::default() });
        let start_pc = w.nation(NationId::USA).political_capital;
        let mut topped = false;
        for _ in 0..300 {
            if w.allied(NationId::USA, NationId::Kuwait) {
                break;
            }
            if apply_command(
                &mut w,
                &Command::ProposeAlliance { from: NationId::USA, to: NationId::Kuwait },
            )
            .is_err()
            {
                w.nation_mut(NationId::USA).political_capital = start_pc;
                topped = true;
            }
        }
        if topped {
            topups += 1;
        }
        w.headlines.clear();
        war::declare_war(&mut w, NationId::Iraq, NationId::Kuwait).unwrap();
        if w.headlines.iter().any(|h| h.contains("United States honours its defence pact")) {
            honoured += 1;
        }
        if w.headlines.iter().any(|h| h.contains("United States abandons its pact")) {
            abandoned += 1;
        }
    }
    println!("-- guarantees_are_usually_but_not_always_honoured --");
    println!("  honoured {} abandoned {} of {}", honoured, abandoned, CHEAP_N);
    println!(
        "  seeds where forcing the pact exhausted Washington's standing (the tracked \
         test would PANIC on these): {} of {} = {:.4}",
        topups,
        CHEAP_N,
        topups as f64 / CHEAP_N as f64
    );
    report_bernoulli(
        "abandoned >= 1 (the rare arm)",
        abandoned,
        CHEAP_N as usize,
        Bar::Floor(1),
        40,
    );
    report_bernoulli(
        "honoured > 2*abandoned, i.e. abandoned < n/3",
        abandoned,
        CHEAP_N as usize,
        Bar::ScaledCeiling(1.0 / 3.0),
        40,
    );

    // a_parliament_can_refuse_a_superpower
    let (mut granted, mut refused) = (0usize, 0usize);
    for seed in 0..CHEAP_N {
        let mut w = world_1990(GameRules { seed, ..GameRules::default() });
        w.nation_mut(NationId::USA).political_capital = 100.0;
        apply_command(
            &mut w,
            &Command::RequestAccess {
                seeker: NationId::USA,
                host: NationId::Turkey,
                theatre: theatre::TheatreId::Levant,
            },
        )
        .unwrap();
        if theatre::has_access(&w, NationId::USA, theatre::TheatreId::Levant) {
            granted += 1;
        } else {
            refused += 1;
        }
    }
    println!("-- a_parliament_can_refuse_a_superpower --");
    println!("  granted {} refused {} of {}", granted, refused, CHEAP_N);
    report_bernoulli("refused >= 4", refused, CHEAP_N as usize, Bar::Floor(4), 40);
    report_bernoulli("granted >= 4", granted, CHEAP_N as usize, Bar::Floor(4), 40);
}
