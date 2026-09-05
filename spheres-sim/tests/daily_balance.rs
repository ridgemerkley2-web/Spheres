//! Diagnostic only: daily integration is not expected to equal a monthly
//! left-endpoint step through a changing economy. This instrument moves no
//! calibration bar and does not assert a preferred historical outcome.
//!
//! cargo test --release -p spheres-sim --test daily_balance -- --ignored --nocapture
use spheres_sim::{
    clock, economy,
    init::world_1990,
    tech,
    world::{GameRules, NationId, WorldState},
};

fn funded_research_world(year: i32, month: u32) -> WorldState {
    let mut w = world_1990(GameRules {
        daily_simulation: true,
        ai_aggression: 0.0,
        ..GameRules::default()
    });
    w.year = year;
    w.month = month;
    w.player = Some(NationId::USA);
    let n = w.nation_mut(NationId::USA);
    n.tech.known.clear();
    n.tech.focus.fill(None);
    n.tech.progress.fill(1_000_000.0);
    w
}

fn known_by_domain(w: &WorldState) -> [u8; tech::DOMAIN_COUNT] {
    let mut counts = [0; tech::DOMAIN_COUNT];
    for t in &w.nation(NationId::USA).tech.known {
        counts[tech::registry()[*t as usize].domain.index()] += 1;
    }
    counts
}

#[test]
fn daily_research_never_pays_for_more_than_six_acquisitions_per_domain_per_month() {
    let mut w = funded_research_world(2035, 1);
    let opening = w.nation(NationId::USA).tech.research_total;
    for day in 1..=31 {
        tech::tick(&mut w);
        for (domain, count) in known_by_domain(&w).iter().enumerate() {
            assert!(
                *count <= 6,
                "day{day}, domain{domain}: {count} acquired in one calendar month"
            );
        }
        clock::advance_date(&mut w);
    }
    assert!(
        w.nation(NationId::USA).tech.research_total > opening,
        "research effort keeps accruing after the acquisition ceiling"
    );
}

#[test]
fn research_quota_resets_on_actual_months_including_leap_and_year_boundaries() {
    for (year, month, days) in [(2035, 2, 28), (2036, 2, 29), (2035, 4, 30), (2035, 12, 31)] {
        let mut w = funded_research_world(year, month);
        tech::tick(&mut w);
        let first = known_by_domain(&w);
        let initial = tech::acquisition_quota_status(&w, NationId::USA).unwrap();
        assert_eq!(initial.acquired, first);
        for _ in 1..days {
            clock::advance_date(&mut w);
            tech::tick(&mut w);
            assert_eq!(
                known_by_domain(&w),
                first,
                "a exhausted calendar cap must not refresh every day"
            );
        }
        assert_eq!(w.day, days);
        clock::advance_date(&mut w);
        let before = spheres_sim::save(&w);
        let reset = tech::acquisition_quota_status(&w, NationId::USA).unwrap();
        assert_eq!(reset.acquired, [0; tech::DOMAIN_COUNT]);
        assert_eq!(reset.remaining, [6; tech::DOMAIN_COUNT]);
        assert!(!reset.migration_hold);
        assert_eq!(
            spheres_sim::save(&w),
            before,
            "quota read must not mutate the calendar"
        );
        assert_eq!((w.year, w.month), (initial.reset_year, initial.reset_month));
        tech::tick(&mut w);
        let after = known_by_domain(&w);
        assert!(after.iter().zip(first).any(|(a, b)| *a > b));
        assert!(after.iter().zip(first).all(|(a, b)| *a - b <= 6));
    }
}

#[test]
fn older_midmonth_save_holds_slots_but_never_invents_acquisitions() {
    let mut w = funded_research_world(2036, 2);
    w.day = 15;
    let opening = spheres_sim::save(&w);
    let status = tech::acquisition_quota_status(&w, NationId::USA).unwrap();
    assert!(status.migration_hold);
    assert_eq!(status.acquired, [0; tech::DOMAIN_COUNT]);
    assert_eq!(status.remaining, [0; tech::DOMAIN_COUNT]);
    assert!(status.note.contains("Older mid-month save"));
    assert_eq!(spheres_sim::save(&w), opening);
    let research = w.nation(NationId::USA).tech.research_total;
    while w.month == 2 {
        tech::tick(&mut w);
        assert_eq!(known_by_domain(&w), [0; tech::DOMAIN_COUNT]);
        clock::advance_date(&mut w);
    }
    assert!(w.nation(NationId::USA).tech.research_total > research);
    let ready = tech::acquisition_quota_status(&w, NationId::USA).unwrap();
    assert!(!ready.migration_hold);
    assert_eq!(ready.remaining, [6; tech::DOMAIN_COUNT]);
    tech::tick(&mut w);
    assert!(known_by_domain(&w).iter().any(|count| *count > 0));
}

#[test]
fn unpaid_future_and_rejected_prerequisite_gates_do_not_spend_quota() {
    let mut w = funded_research_world(1990, 1);
    w.nation_mut(NationId::USA).gdp = 1e-6;
    w.nation_mut(NationId::USA).tech.progress.fill(0.0);
    tech::tick(&mut w);
    assert_eq!(
        tech::acquisition_quota_status(&w, NationId::USA)
            .unwrap()
            .acquired,
        [0; tech::DOMAIN_COUNT]
    );
    let future = tech::registry()
        .iter()
        .enumerate()
        .find(|(_, d)| d.earliest_year > 1990)
        .unwrap()
        .0 as u16;
    let domain = tech::registry()[future as usize].domain;
    let n = w.nation_mut(NationId::USA);
    n.tech.known = tech::prereqs_of(future).to_vec();
    n.tech.known.sort_unstable();
    n.tech.focus.fill(None);
    n.tech.focus[domain.index()] = Some(future);
    n.tech.progress[domain.index()] = 1_000_000.0;
    n.tech.allocation = Some(std::array::from_fn(|di| {
        if di == domain.index() {
            1.0
        } else {
            0.0
        }
    }));
    clock::advance_date(&mut w);
    tech::tick(&mut w);
    assert!(!w.nation(NationId::USA).tech.knows_index(future));
    assert_eq!(
        tech::acquisition_quota_status(&w, NationId::USA)
            .unwrap()
            .acquired,
        [0; tech::DOMAIN_COUNT]
    );
    assert!(w.nation(NationId::USA).tech.progress[domain.index()] >= 1_000_000.0);
    let missing = tech::registry()
        .iter()
        .enumerate()
        .find(|(i, _)| !tech::prereqs_of(*i as u16).is_empty())
        .unwrap();
    w.nation_mut(NationId::USA).tech.known.clear();
    let before = spheres_sim::save(&w);
    assert!(spheres_sim::apply_command(
        &mut w,
        &spheres_sim::Command::SetResearchFocus {
            nation: NationId::USA,
            domain: missing.1.domain,
            tech: Some(missing.1.id.into()),
        }
    )
    .is_err());
    assert_eq!(
        spheres_sim::save(&w),
        before,
        "rejected prerequisite must not charge either money or slots"
    );
}

#[test]
fn focus_switch_and_repeated_tick_do_not_refresh_calendar_quota() {
    let mut w = funded_research_world(2035, 1);
    tech::tick(&mut w);
    let first = known_by_domain(&w);
    let quota = w.nation(NationId::USA).tech.acquisition_quota;
    let choices = tech::eligible_projects(w.nation(NationId::USA), tech::Domain::Materials);
    let next = choices
        .iter()
        .find(|d| {
            Some(tech::index_of(d.id).unwrap())
                != w.nation(NationId::USA).tech.focus[tech::Domain::Materials.index()]
        })
        .unwrap()
        .id;
    spheres_sim::apply_command(
        &mut w,
        &spheres_sim::Command::SetResearchFocus {
            nation: NationId::USA,
            domain: tech::Domain::Materials,
            tech: Some(next.into()),
        },
    )
    .unwrap();
    assert_eq!(w.nation(NationId::USA).tech.acquisition_quota, quota);
    for _ in 0..3 {
        tech::tick(&mut w);
    }
    assert_eq!(known_by_domain(&w), first);
    assert_eq!(w.nation(NationId::USA).tech.acquisition_quota, quota);
}

#[test]
fn research_quota_survives_reload_and_daily_batching() {
    let mut day_by_day = funded_research_world(2036, 2);
    // Keep only the technology subsystem in this schedule: the test compares
    // the same dated work, not noisy macroeconomic paths or AI choices.
    let mut grouped = day_by_day.clone();
    for day in 0..35 {
        tech::tick(&mut day_by_day);
        clock::advance_date(&mut day_by_day);
        if day == 11 {
            day_by_day = spheres_sim::load(&spheres_sim::save(&day_by_day)).unwrap();
        }
    }
    for count in [3, 9, 17, 6] {
        for _ in 0..count {
            tech::tick(&mut grouped);
            clock::advance_date(&mut grouped);
        }
        grouped = spheres_sim::load(&spheres_sim::save(&grouped)).unwrap();
    }
    assert_eq!(spheres_sim::save(&grouped), spheres_sim::save(&day_by_day));
    assert_eq!(
        tech::acquisition_quota_status(&grouped, NationId::USA)
            .unwrap()
            .acquired,
        tech::acquisition_quota_status(&day_by_day, NationId::USA)
            .unwrap()
            .acquired
    );
}

#[test]
fn monthly_research_keeps_old_loop_and_never_writes_a_quota() {
    let mut w = funded_research_world(2035, 1);
    w.rules.daily_simulation = false;
    let before = spheres_sim::save(&w);
    assert!(tech::acquisition_quota_status(&w, NationId::USA).is_none());
    assert_eq!(spheres_sim::save(&w), before);
    tech::tick(&mut w);
    let first = known_by_domain(&w);
    assert!(first.iter().all(|count| *count <= 6));
    tech::tick(&mut w);
    assert!(
        known_by_domain(&w).iter().zip(first).any(|(a, b)| *a > b),
        "legacy monthly tick still gets its own six-attempt loop"
    );
    assert!(w.nations.iter().all(|n| n.tech.acquisition_quota.is_none()));
    assert!(!spheres_sim::save(&w).contains("\"acquisition_quota\""));
}

fn rate(w: &WorldState, id: NationId) -> f64 {
    let n = w.nation(id);
    economy::growth_terms(
        n,
        n.state_invest_gdp,
        n.interest_rate,
        &economy::Conditions {
            oil_price: w.oil_price,
            sanction_share: w.sanction_weight(id),
            at_war: w.at_war(id),
            export_share: w.oil_export_share(id),
        },
    )
    .before_noise
}

fn median(mut values: Vec<f64>) -> f64 {
    values.sort_by(f64::total_cmp);
    values[values.len() / 2]
}

#[test]
fn additive_private_investment_pressure_conserves_28_29_30_31_day_months() {
    for (year, month, expected_days) in [(1990, 2, 28), (2000, 2, 29), (1990, 4, 30), (1990, 1, 31)]
    {
        let mut w = world_1990(GameRules {
            daily_simulation: true,
            ..GameRules::default()
        });
        w.year = year;
        w.month = month;
        w.day = 1;
        let mut fraction = 0.0;
        let mut positive_flow = 0.0;
        let mut negative_flow = 0.0;
        for _ in 0..expected_days {
            let coefficient = economy::private_investment_flow_fraction(&w);
            fraction += coefficient;
            positive_flow += 0.015 * coefficient;
            negative_flow += -0.012 * coefficient;
            clock::advance_date(&mut w);
        }
        assert_eq!(w.day, 1);
        assert_ne!(w.month, month);
        assert!(
            (fraction - 0.06).abs() < 1e-14,
            "{year}-{month}: a month of daily pressure sums to {fraction}, not .06"
        );
        assert!((positive_flow - 0.015 * 0.06).abs() < 1e-14);
        assert!((negative_flow - -0.012 * 0.06).abs() < 1e-14);
    }
}

#[test]
fn monthly_private_investment_pressure_keeps_the_original_arithmetic() {
    let mut w = world_1990(GameRules::default());
    for month in 1..=12 {
        w.month = month;
        let coefficient = economy::private_investment_flow_fraction(&w);
        assert_eq!(coefficient.to_bits(), 0.06_f64.to_bits());
        for share in [0.01_f64, 0.06, 0.15, 0.30, 0.35] {
            for pressure in [-0.1_f64, -0.012, 0.0, 0.015, 0.1] {
                let target = (share + pressure).clamp(0.01, 0.35);
                let old = share + (target - share) * 0.06;
                let actual = share + (target - share) * coefficient;
                assert_eq!(
                    actual.to_bits(),
                    old.to_bits(),
                    "monthly clamp and signed flow must remain bit-identical"
                );
            }
        }
    }
}

#[test]
fn actual_economy_settlement_uses_the_additive_pressure_fraction() {
    for daily in [false, true] {
        for (year, month) in [(1990, 2), (2000, 2), (1990, 4), (1990, 1)] {
            for id in [NationId::USA, NationId::China] {
                let mut w = world_1990(GameRules {
                    daily_simulation: daily,
                    ..GameRules::default()
                });
                w.year = year;
                w.month = month;
                w.player = Some(id);
                let before = w.nation(id).clone();
                let at_war = w.at_war(id);
                let sanction = w.sanction_weight(id);
                economy::tick(&mut w);
                let after = w.nation(id);
                // Reconstruct the engine's investment-phase inputs: growth
                // and output have settled, population/stability have not yet.
                // This independently checks that tick actually uses the new
                // coefficient; a helper-only test would miss a disconnected fix.
                let mut phase = after.clone();
                phase.population = before.population;
                phase.stability = before.stability;
                let unemployment = economy::unemployment_rate(&phase, at_war);
                let dev = (before.gdp * 1000.0 / before.population / 24000.0).min(1.0);
                let mut pressure = (after.growth_last - 0.020) * 0.20
                    + (0.025 - (before.interest_rate - before.inflation)) * 0.10
                    + (before.stability - 60.0) * 0.00010
                    + (0.28 - before.tax_rate) * 0.06
                    + (0.07 - unemployment) * 0.05
                    + after.bubble * 0.012
                    - sanction * 0.025
                    - if at_war { 0.020 } else { 0.0 };
                if before.system == spheres_sim::world::EconomySystem::Command {
                    pressure -= 0.003 + dev * 0.002;
                }
                let coefficient = if daily {
                    0.06 * (1.0 / spheres_sim::world::days_in_month(year, month) as f64)
                } else {
                    0.06
                };
                let target = (before.priv_invest_gdp + pressure).clamp(0.01, 0.35);
                let expected =
                    before.priv_invest_gdp + (target - before.priv_invest_gdp) * coefficient;
                assert_eq!(
                    after.priv_invest_gdp.to_bits(),
                    expected.to_bits(),
                    "{id:?} {year}-{month} daily={daily}"
                );
            }
        }
    }
}

#[test]
#[ignore = "dated calibration diagnosis; not a balance target or census"]
fn daily_integrator_channel_probe() {
    println!("DAILY CALIBRATION PROBE 2026-09-03; economic-only, identical monthly shocks; seeds 1990,42,7.");
    println!("seed,month,lower_gdp,living,median_gdp_relative,median_stability_delta,usa_monthly_gdp,usa_daily_gdp");
    for seed in [1990, 42, 7] {
        let mut monthly = world_1990(GameRules {
            seed,
            ai_aggression: 0.0,
            ..GameRules::default()
        });
        let mut daily = monthly.clone();
        daily.rules.daily_simulation = true;
        let opening_rates: Vec<_> = daily
            .nations
            .iter()
            .filter(|n| n.alive)
            .map(|n| (n.id, rate(&daily, n.id)))
            .collect();
        for month in 1..=12 {
            let current_month = daily.month;
            economy::tick(&mut monthly);
            // Economy alone does not advance the clock in either mode.
            monthly.day = spheres_sim::world::days_in_month(monthly.year, monthly.month);
            clock::advance_date(&mut monthly);
            while daily.month == current_month {
                economy::tick(&mut daily);
                clock::advance_date(&mut daily);
            }
            assert_eq!(
                monthly.rng, daily.rng,
                "the probe must not confuse different random shocks with an integrator effect"
            );
            assert_eq!(
                (monthly.year, monthly.month, monthly.day),
                (daily.year, daily.month, daily.day)
            );
            for n in daily.nations.iter().filter(|n| n.alive) {
                assert!(n.gdp.is_finite() && n.gdp > 0.0);
                assert!(n.stability.is_finite());
            }
            if month == 1 || month == 12 {
                let rows: Vec<_> = monthly
                    .nations
                    .iter()
                    .filter(|n| n.alive)
                    .map(|n| {
                        let d = daily.nation(n.id);
                        (d.gdp / n.gdp - 1.0, d.stability - n.stability)
                    })
                    .collect();
                println!(
                    "{seed},{month},{},{},{:.9},{:.9},{:.9},{:.9}",
                    rows.iter().filter(|(g, _)| *g < 0.0).count(),
                    rows.len(),
                    median(rows.iter().map(|(g, _)| *g).collect()),
                    median(rows.iter().map(|(_, s)| *s).collect()),
                    monthly.nation(NationId::USA).gdp,
                    daily.nation(NationId::USA).gdp
                );
                if month == 1 {
                    let falling = opening_rates
                        .iter()
                        .filter(|(id, start)| rate(&daily, *id) < *start)
                        .count();
                    println!("seed={seed} first-month declining endogenous annual growth rates={falling}/{}", opening_rates.len());
                    for id in [NationId::USA, NationId::Vietnam, NationId::Argentina] {
                        let start = opening_rates.iter().find(|(n, _)| *n == id).unwrap().1;
                        println!(
                            "seed={seed} {id:?} annual-rate opening={start:.9} month-end={:.9}",
                            rate(&daily, id)
                        );
                    }
                }
            }
        }
    }
    let mut w = world_1990(GameRules {
        daily_simulation: true,
        ..GameRules::default()
    });
    for month in [1, 2, 4] {
        w.month = month;
        let days = spheres_sim::world::days_in_month(w.year, month) as f64;
        println!("Y6 month={month}: monthly=.06 former-compounded-daily-sum={:.9} repaired-additive-daily-sum={:.9}; Y7 former-daily-opportunities={} repaired-calendar-month-limit={}",
            days * clock::blend(&w, 0.06), days * economy::private_investment_flow_fraction(&w),
            days as u32 * 6, tech::ACQUISITIONS_PER_DOMAIN_MONTH);
    }
}
