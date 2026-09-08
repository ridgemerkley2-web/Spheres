//! Shared-accounting and replay contracts, not historical calibration bands.
use spheres_sim::{apply_command, clock, economy, fiscal_recovery, init::world_1990,
    programs, tick_day, Command, world::{GameRules, NationId as N, WorldState}};

fn daily() -> WorldState {
    let mut w = world_1990(GameRules { daily_simulation:true, ai_aggression:0.0,
        ..GameRules::default() });
    w.player=Some(N::USA);
    w
}

#[test]
fn legacy_save_requires_explicit_upgrade_without_a_cash_or_debt_grant() {
    let mut w=daily();
    w.year=2007;
    w.nation_mut(N::USA).treasury_bn=Some(17.0);
    w.nation_mut(N::USA).debt_bn=Some(901.0);
    economy::refresh_debt_ratio(w.nation_mut(N::USA));
    let before=spheres_sim::save(&w);
    assert!(!before.contains("fiscal_recovery"));
    let mut loaded=spheres_sim::load(&before).unwrap();
    assert!(!fiscal_recovery::enabled(&loaded));
    let opening:Vec<_>=loaded.nations.iter().filter(|n|n.alive)
        .map(|n|(n.id,n.gdp,n.debt_bn.unwrap_or(n.debt_gdp*n.gdp),n.treasury_bn.unwrap_or(0.0))).collect();
    apply_command(&mut loaded,&Command::EnableFiscalRecovery{nation:N::USA}).unwrap();
    for (id,gdp,debt,cash) in opening {
        let n=loaded.nation(id);
        assert_eq!(n.gdp,gdp);
        assert_eq!(n.debt_bn,Some(debt));
        assert_eq!(n.treasury_bn,Some(cash));
    }
    let enabled=spheres_sim::save(&loaded);
    fiscal_recovery::enable(&mut loaded);
    assert_eq!(spheres_sim::save(&loaded),enabled,"upgrade must be idempotent");
}

#[test]
fn unopened_player_and_ai_pay_the_same_explicit_interest_rule() {
    let mut w=daily();
    fiscal_recovery::enable(&mut w);
    // Every nation, including countries outside economic competition, opens
    // the same stock ledger before any policy or operating settlement.
    assert!(w.nations.iter().filter(|n|n.alive).all(|n|n.on_the_books()));
    for id in [N::USA,N::Japan,N::Iraq] {
        let n=w.nation_mut(id);
        n.debt_bn=Some(n.gdp*1.2);
        n.interest_rate=0.06;
        n.inflation=0.02;
        economy::refresh_debt_ratio(n);
        assert!((economy::interest_gdp(n)-1.2*(0.04+0.6*0.06)).abs()<1e-12);
    }
    let mut expected=w.clone();
    // Economy alone proves opening books is not merely a UI label: every
    // ordinary nation incurs its budget plus the interest charge in cash.
    economy::tick(&mut expected);
    for id in [N::USA,N::Japan,N::Iraq] {
        assert_ne!(expected.nation(id).debt_bn,w.nation(id).debt_bn);
        assert_eq!(expected.nation(id).debt_gdp,
            expected.nation(id).debt_bn.unwrap()/expected.nation(id).gdp);
    }
}

#[test]
fn fiscal_state_survives_midmonth_save_and_daily_replay() {
    let mut w=daily();
    fiscal_recovery::enable(&mut w);
    for _ in 0..17 {tick_day(&mut w,&[]);}
    let mut resumed=spheres_sim::load(&spheres_sim::save(&w)).unwrap();
    for _ in 0..80 {
        tick_day(&mut w,&[]);
        tick_day(&mut resumed,&[]);
    }
    assert_eq!(spheres_sim::save(&w),spheres_sim::save(&resumed));
}

#[test]
fn department_receipts_still_post_once_with_recovery_monitoring() {
    let mut w=daily();
    fiscal_recovery::enable(&mut w);
    let allocations=w.nation(N::USA).budget_for(w.year).allocations;
    let fiscal_year=w.year;
    apply_command(&mut w,&Command::SetProgramBudget{nation:N::USA,fiscal_year,
        allocations,departments:programs::default_departments()}).unwrap();
    programs::begin_day(&mut w);
    economy::tick(&mut w);
    programs::finish_day(&mut w);
    fiscal_recovery::tick(&mut w);
    let saved=spheres_sim::save(&w);
    programs::finish_day(&mut w);
    fiscal_recovery::tick(&mut w);
    assert_eq!(spheres_sim::save(&w),saved,"monitor or settlement posted twice");
    assert_eq!(clock::absolute_day(&w),0);
}

#[test]
fn fiscal_confidence_uses_one_prorated_stability_arm_without_an_inflation_charge() {
    let mut calm=daily();
    fiscal_recovery::enable(&mut calm);
    let mut stressed=calm.clone();
    stressed.fiscal_recovery.nations.get_mut(&N::USA).unwrap().confidence_pressure=0.20;
    let dt=clock::month_fraction(&calm);
    economy::tick(&mut calm);
    economy::tick(&mut stressed);
    let a=calm.nation(N::USA);
    let b=stressed.nation(N::USA);
    assert!((a.stability-b.stability-economy::stability_flow(0.20,dt)).abs()<1e-12);
    assert_eq!(a.inflation,b.inflation);
    assert_eq!(a.debt_bn,b.debt_bn);
    assert_eq!(a.gdp,b.gdp);
    assert_eq!(calm.conflicts,stressed.conflicts);
}

#[test]
fn draft_tax_forecast_changes_without_rewriting_posted_recovery_results() {
    let mut w=daily();
    fiscal_recovery::enable(&mut w);
    let allocations=w.nation(N::USA).budget_for(w.year).allocations;
    let budget=Command::SetProgramBudget{nation:N::USA,fiscal_year:w.year,
        allocations,departments:programs::default_departments()};
    let before=spheres_sim::save(&w);
    let (_,base)=spheres_sim::fiscal_preview::with_budget(&w,N::USA,&[],&budget).unwrap();
    let policy=Command::SetTaxRate{nation:N::USA,rate:w.nation(N::USA).tax_rate+0.03};
    let (_,raised)=spheres_sim::fiscal_preview::with_budget(&w,N::USA,&[policy],&budget).unwrap();
    assert!(raised.projected_debt_gdp_5y_at_full_use.unwrap()<base.projected_debt_gdp_5y_at_full_use.unwrap());
    assert_eq!(spheres_sim::save(&w),before);
}

#[test]
fn post_economy_gdp_shock_changes_the_ratio_without_erasing_debt_or_charging_again() {
    let mut w=daily();
    fiscal_recovery::enable(&mut w);
    economy::tick(&mut w);
    let debt=w.nation(N::Iraq).debt_bn;
    let cash=w.nation(N::Iraq).treasury_bn;
    // The government coup system uses a GDP level shock after economy. Its
    // unaltered fiscal stock must remain consistent at the closing daily read.
    w.nation_mut(N::Iraq).gdp*=0.97;
    assert_ne!(w.nation(N::Iraq).debt_gdp,debt.unwrap()/w.nation(N::Iraq).gdp);
    fiscal_recovery::tick(&mut w);
    assert_eq!(w.nation(N::Iraq).debt_gdp,debt.unwrap()/w.nation(N::Iraq).gdp);
    assert_eq!(w.nation(N::Iraq).debt_bn,debt);
    assert_eq!(w.nation(N::Iraq).treasury_bn,cash);
    assert_eq!(fiscal_recovery::assessment(&w,N::Iraq).unwrap().debt_gdp,w.nation(N::Iraq).debt_gdp);
}

#[test]
fn government_coup_refreshes_the_ratio_at_the_actual_gdp_mutation() {
    let mut w=daily();
    w.player=Some(N::China);
    w.rules.ideology_blocs=true;
    spheres_sim::government::ensure_all(&mut w);
    fiscal_recovery::enable(&mut w);
    // The existing government-module coup fixture: a settled regime and an
    // army whose accumulated pressure has crossed the deterministic threshold.
    let g=w.governments.states.iter_mut().find(|g|g.nation==N::China).unwrap();
    g.coup_pressure=5.0;
    g.months_in_office=48;
    for (pillar,loyalty) in &mut g.pillars {
        *loyalty=if *pillar==spheres_sim::government::Pillar::Army {0.20} else {0.80};
    }
    let opening=w.nation(N::China);
    let (gdp,debt,cash)=(opening.gdp,opening.debt_bn,opening.treasury_bn);
    spheres_sim::government::tick(&mut w);
    assert!(w.headlines.iter().any(|h|h.starts_with("COUP IN CHINA")),"{:?}",w.headlines);
    let after=w.nation(N::China);
    assert_eq!(after.gdp,gdp*0.97);
    assert_eq!(after.debt_bn,debt);
    assert_eq!(after.treasury_bn,cash);
    // Read immediately after government::tick: the end-of-day safety refresh
    // cannot hide a stale quote at the mutation owner.
    assert_eq!(after.debt_gdp,debt.unwrap()/after.gdp);
}

fn domestic_recovery_policy_fixture(enabled: bool) -> WorldState {
    let mut w = daily();
    if enabled { fiscal_recovery::enable(&mut w); }
    let fiscal_year = w.year;
    let n = w.nation_mut(N::USA);
    n.political_capital = 500.0;
    n.stability = 60.0;
    // A balance-sheet recession makes unemployment an active pressure even
    // after economy replaces growth_last with the current day's result.
    n.bubble = -1.0;
    let mut allocations = n.budget_for(fiscal_year).allocations;
    allocations[spheres_sim::world::BUDGET_HOUSING] *= 0.80;
    apply_command(&mut w, &Command::SetAnnualBudget {
        nation: N::USA, fiscal_year, allocations,
    }).unwrap();
    apply_command(&mut w, &Command::SetTaxRate { nation: N::USA, rate: 0.45 }).unwrap();
    w
}

#[test]
fn enabled_ai_and_player_pay_identical_domestic_recovery_consequences() {
    let mut player = domestic_recovery_policy_fixture(true);
    let mut ai = player.clone();
    ai.player = None;
    let opening_investment = player.nation(N::USA).priv_invest_gdp;
    let human_terms = economy::stability_pressure_terms_of(&player, player.nation(N::USA));
    let ai_terms = economy::stability_pressure_terms_of(&ai, ai.nation(N::USA));
    assert!(human_terms.housing < 0.0, "the enacted housing cut must carry a service cost");
    assert!(human_terms.unemployment_drag > 0.0, "the fixture must have joblessness pressure");
    assert_eq!(ai_terms.housing, human_terms.housing);
    assert_eq!(ai_terms.unemployment_drag, human_terms.unemployment_drag);
    assert_eq!(ai_terms.total, human_terms.total);
    assert_eq!(economy::effective_population_growth(&ai, N::USA),
        economy::effective_population_growth(&player, N::USA));

    economy::tick(&mut player);
    economy::tick(&mut ai);
    assert_eq!(ai.daily.economic_shocks, player.daily.economic_shocks);
    let human = player.nation(N::USA);
    let opponent = ai.nation(N::USA);
    assert_ne!(human.priv_invest_gdp, opening_investment,
        "tax and recession feedback must reach actual private investment");
    assert_eq!(opponent.gdp, human.gdp);
    assert_eq!(opponent.treasury_bn, human.treasury_bn);
    assert_eq!(opponent.debt_bn, human.debt_bn);
    assert_eq!(opponent.priv_invest_gdp, human.priv_invest_gdp);
    assert_eq!(opponent.stability, human.stability);
    assert_eq!(opponent.population, human.population);
}

#[test]
fn legacy_ai_retains_its_original_domestic_policy_gates() {
    let mut player = domestic_recovery_policy_fixture(false);
    let mut ai = player.clone();
    ai.player = None;
    let opening_investment = ai.nation(N::USA).priv_invest_gdp;
    let human_terms = economy::stability_pressure_terms_of(&player, player.nation(N::USA));
    let ai_terms = economy::stability_pressure_terms_of(&ai, ai.nation(N::USA));
    assert!(human_terms.housing < 0.0);
    assert!(human_terms.unemployment_drag > 0.0);
    assert_eq!(ai_terms.housing, 0.0);
    assert_eq!(ai_terms.unemployment_drag, 0.0);
    assert_ne!(economy::effective_population_growth(&ai, N::USA),
        economy::effective_population_growth(&player, N::USA));
    economy::tick(&mut player);
    economy::tick(&mut ai);
    assert_eq!(ai.nation(N::USA).priv_invest_gdp, opening_investment,
        "disabled legacy AI must retain its calibrated investment share");
    assert_ne!(player.nation(N::USA).priv_invest_gdp, opening_investment);
    assert!(player.nation(N::USA).stability < ai.nation(N::USA).stability);
    assert!(player.nation(N::USA).population < ai.nation(N::USA).population);
}
