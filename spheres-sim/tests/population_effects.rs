//! Existing-system contracts for the enabled People model, alongside legacy
//! replay tests. These assertions catch double posting and dishonest previews.
use spheres_sim::{apply_command, clock, economy, init::world_1990, ministries,
    population::{self, Policy}, price_of, tech, tick_day,
    world::{GameRules, NationId, WorldState, BUDGET_EDUCATION}};

fn campaign() -> WorldState {
    let mut w = world_1990(GameRules { daily_simulation: true,
        ai_aggression: 0.0, ..GameRules::default() });
    w.player = Some(NationId::USA);
    population::enable(&mut w);
    w
}

#[test]
fn economic_readers_quote_counted_unemployment_from_the_opening_day() {
    let mut w = campaign();
    for _ in 0..3 {
        let jobs = population::snapshot(&w, NationId::USA).unwrap();
        assert!((economy::unemployment_rate(w.nation(NationId::USA), false)
            - jobs.unemployed_m / jobs.labor_force_m).abs() < 1e-12);
        tick_day(&mut w, &[]);
    }
}

#[test]
fn a_daily_tick_posts_demography_once_across_population_economy_and_technology() {
    let mut w = campaign();
    let mut demographic_only = w.clone();
    population::tick(&mut demographic_only);
    tick_day(&mut w, &[]);
    let wanted = demographic_only.nation(NationId::USA).population;
    assert!((w.nation(NationId::USA).population - wanted).abs() < 1e-10,
        "economy or technology posted a second population flow");
}

#[test]
fn education_funding_quotes_places_and_cannot_instantly_create_researchers() {
    let mut w = campaign();
    let id = NationId::USA;
    let before = population::research_multiplier(&w, id).unwrap();
    let mut budget = w.nation(id).budget_for(w.year);
    budget.allocations[BUDGET_EDUCATION] += 0.02;
    w.nation_mut(id).annual_budget = Some(budget);
    assert_eq!(population::research_multiplier(&w, id).unwrap(), before);
    let n = w.nation(id);
    let arms = ministries::arms_at(&w, n, BUDGET_EDUCATION,
        n.budget_for(w.year).allocations[BUDGET_EDUCATION]);
    assert_eq!(arms[0].id, "education_capacity");
    assert!(!arms.iter().any(|a| a.id == "research"));
    let terms = tech::research_terms(&w, n, (n.gdp * 1000.0 / n.population / 24000.0).min(1.0));
    assert_eq!(terms.ministry, before);
}

#[test]
fn population_policy_price_is_paid_once_and_cooldown_refusal_is_atomic() {
    let mut w = campaign();
    let id = NationId::USA;
    w.nation_mut(id).political_capital = 100.0;
    let command = spheres_sim::Command::SetPopulationPolicy { nation: id, policy: Policy::TradeSchools };
    let quote = population::policy_quote(&w, id, Policy::TradeSchools);
    assert_eq!(price_of(&w, &command), Some(quote.political_cost));
    apply_command(&mut w, &command).unwrap();
    assert_eq!(w.nation(id).political_capital, 100.0 - quote.political_cost);
    let before = spheres_sim::save(&w);
    let other = spheres_sim::Command::SetPopulationPolicy { nation: id, policy: Policy::Universities };
    assert!(apply_command(&mut w, &other).is_err());
    assert_eq!(spheres_sim::save(&w), before);
}

#[test]
fn old_saves_require_explicit_population_upgrade_and_idempotent_enable() {
    let mut w = world_1990(GameRules::default());
    let before = spheres_sim::save(&w);
    assert!(!before.contains("population_system"));
    assert!(!spheres_sim::load(&before).unwrap().population_system.enabled);
    clock::enable_daily_play(&mut w);
    let pop = w.nation(NationId::USA).population;
    let gdp = w.nation(NationId::USA).gdp;
    apply_command(&mut w, &spheres_sim::Command::EnablePopulation { nation: NationId::USA }).unwrap();
    assert_eq!(w.nation(NationId::USA).population, pop);
    assert_eq!(w.nation(NationId::USA).gdp, gdp);
    let enabled = spheres_sim::save(&w);
    population::enable(&mut w);
    assert_eq!(spheres_sim::save(&w), enabled);
}
