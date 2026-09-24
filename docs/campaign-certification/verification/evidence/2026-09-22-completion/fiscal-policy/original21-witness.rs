pub use spheres_sim::*;
pub use spheres_sim::world::*;
use spheres_sim::init::world_1990;
use spheres_sim::politics::tick;
    fn fiscal_boundary_world(daily: bool, lens: bool, id: NationId) -> WorldState {
        let mut w = world_1990(GameRules {
            daily_simulation: daily, ideology_blocs: lens,
            crisis_intensity: 0.0, ai_aggression: 0.0,
            ..GameRules::default()
        });
        // This is a policy-phase unit fixture. Other systems do not settle
        // revenue or change GDP here, and no foreign actor can intervene.
        for n in &mut w.nations { n.alive = n.id == id; }
        let n = w.nation_mut(id);
        n.stability = 90.0;
        n.separatism = 0.0;
        n.debt_gdp = 1.20;
        n.tax_rate = 0.22;
        n.mil_spend_gdp = 0.06;
        n.state_invest_gdp = 0.10;
        n.state_invest_1990 = Some(0.10);
        n.political_capital = 100.0;
        w
    }

    #[test]
    fn fiscal_consolidation_never_increases_spending_or_reduces_tax() {
        // Samoa has no Army: a military minimum cannot create a force budget.
        // Both modes use the same legacy fiscal policy, not separate fixes.
        let id = NationId::Samoa;
        for lens in [false, true] {
            for (mil, investment, tax, expected_mil, expected_investment, expected_tax) in [
                (0.0, 0.0, 0.60, 0.0, 0.0, 0.60),
                (0.008, 0.014, 0.22, 0.008, 0.014, 0.222),
                (0.010, 0.020, 0.55, 0.010, 0.020, 0.55),
                (0.060, 0.100, 0.54, 0.0597, 0.0995, 0.542),
                (0.060, 0.100, 0.5499, 0.0597, 0.0995, 0.55),
            ] {
                let mut w = fiscal_boundary_world(false, lens, id);
                let n = w.nation_mut(id);
                n.mil_spend_gdp = mil;
                n.state_invest_gdp = investment;
                n.tax_rate = tax;
                assert_eq!(crate::government::ai_army_funding_floor(&w, id), None);
                tick(&mut w);
                let n = w.nation(id);
                assert!(n.mil_spend_gdp <= mil && n.state_invest_gdp <= investment);
                assert!(n.tax_rate >= tax);
                assert!((n.mil_spend_gdp - expected_mil).abs() < 1e-12);
                assert!((n.state_invest_gdp - expected_investment).abs() < 1e-12);
                assert!((n.tax_rate - expected_tax).abs() < 1e-12);
            }
        }
    }

    #[test]
    fn daily_fiscal_boundaries_preserve_calendar_scaled_rates() {
        let id = NationId::Samoa;
        // Actual daily integrator flags and Gregorian month lengths, not 30
        // calls to the monthly rate. The isolated policy phase is the subject.
        for (year, month) in [(1990, 1), (1990, 2), (2000, 2)] {
            for lens in [false, true] {
                for low in [false, true] {
                    let mut w = fiscal_boundary_world(true, lens, id);
                    w.year = year; w.month = month;
                    if low {
                        let n = w.nation_mut(id);
                        n.mil_spend_gdp = 0.008;
                        n.state_invest_gdp = 0.014;
                    }
                    for day in 1..=days_in_month(year, month) {
                        w.day = day;
                        tick(&mut w);
                        if low {
                            assert_eq!(w.nation(id).mil_spend_gdp, 0.008);
                            assert_eq!(w.nation(id).state_invest_gdp, 0.014);
                        }
                    }
                    let n = w.nation(id);
                    assert!((n.tax_rate - 0.222).abs() < 1e-12);
                    if !low {
                        assert!((n.mil_spend_gdp - 0.0597).abs() < 1e-12);
                        assert!((n.state_invest_gdp - 0.0995).abs() < 1e-12);
                    }
                }
            }
        }
    }

    #[test]
    fn fiscal_recovery_moves_toward_bounds_without_reversing_direction() {
        let id = NationId::Samoa;
        for daily in [false, true] {
            for (investment, tax, expected_investment, expected_tax) in [
                (0.20, 0.30, 0.20, 0.30),
                (0.10, 0.29, 0.10, 0.29),
                (0.0999, 0.3001, 0.10, 0.30),
                (0.08, 0.40, 0.0804, 0.399),
            ] {
                let mut w = fiscal_boundary_world(daily, false, id);
                let n = w.nation_mut(id);
                n.debt_gdp = 0.10;
                n.state_invest_gdp = investment;
                n.tax_rate = tax;
                let count = if daily { days_in_month(w.year, w.month) } else { 1 };
                for day in 1..=count {
                    w.day = day;
                    let before = w.nation(id).state_invest_gdp;
                    tick(&mut w);
                    assert!(w.nation(id).state_invest_gdp >= before);
                    assert!(w.nation(id).tax_rate <= tax);
                }
                assert!((w.nation(id).state_invest_gdp - expected_investment).abs() < 1e-12);
                assert!((w.nation(id).tax_rate - expected_tax).abs() < 1e-12);
            }
            let mut missing = fiscal_boundary_world(daily, false, id);
            missing.nation_mut(id).debt_gdp = 0.10;
            missing.nation_mut(id).state_invest_1990 = None;
            tick(&mut missing);
            assert_eq!(missing.nation(id).state_invest_gdp, 0.10);
        }
    }

    #[test]
    fn legacy_fiscal_policy_respects_player_program_and_recovery_owners() {
        let id = NationId::Italy;
        for owner in ["player", "program", "recovery"] {
            let mut w = fiscal_boundary_world(true, true, id);
            match owner {
                "player" => w.player = Some(id),
                "program" => {
                    // Enroll through the real priced command, then release the
                    // player seat so the program guard is independently tested.
                    w.player = Some(id);
                    let allocations = w.nation(id).budget_for(w.year).allocations;
                    crate::apply_command(&mut w, &crate::Command::SetProgramBudget {
                        nation: id, fiscal_year: 1990, allocations,
                        departments: crate::programs::default_departments(),
                    }).unwrap();
                    w.player = None;
                    assert!(crate::programs::enrolled(&w, id));
                }
                "recovery" => crate::fiscal_recovery::enable(&mut w),
                _ => unreachable!(),
            }
            let n = w.nation(id);
            let before = (n.tax_rate, n.mil_spend_gdp, n.state_invest_gdp);
            tick(&mut w);
            let n = w.nation(id);
            assert_eq!((n.tax_rate, n.mil_spend_gdp, n.state_invest_gdp), before, "{owner}");
        }
    }

    #[test]
    fn an_explicit_annual_plan_keeps_one_fiscal_owner_after_player_handoff() {
        let id = NationId::Italy;
        for daily in [false, true] {
            let mut w = fiscal_boundary_world(daily, true, id);
            w.player = Some(id);
            let mut allocations = w.nation(id).budget_for(w.year).allocations;
            allocations[BUDGET_DEFENSE] += 0.004;
            let command = crate::Command::SetAnnualBudget { nation: id, fiscal_year: 1990, allocations };
            let price = crate::price_of(&w, &command).unwrap();
            assert!(price > 0.0);
            crate::apply_command(&mut w, &command).unwrap();
            assert!((w.nation(id).political_capital - (100.0 - price)).abs() < 1e-12);
            w.player = None;
            assert!(w.nation(id).on_the_books());
            assert!(!crate::programs::enrolled(&w, id));
            assert_eq!(crate::government::ai_army_funding_floor(&w, id), None);
            let mut resumed = crate::load(&crate::save(&w)).unwrap();
            let before = (w.nation(id).tax_rate, w.nation(id).mil_spend_gdp, w.nation(id).state_invest_gdp);
            for candidate in [&mut w, &mut resumed] {
                tick(candidate);
                let n = candidate.nation(id);
                assert_eq!((n.tax_rate, n.mil_spend_gdp, n.state_invest_gdp), before);
                let plan = n.annual_budget.as_ref().unwrap();
                assert_eq!(plan.allocations, allocations);
                assert_eq!(plan.defense(), n.mil_spend_gdp);
                assert_eq!(plan.investment_total(), n.state_invest_gdp);
            }
        }
    }

    #[test]
    fn opening_low_budget_nations_are_not_given_unvoted_spending() {
        let mut w = world_1990(GameRules { crisis_intensity: 0.0, ai_aggression: 0.0, ..GameRules::default() });
        let ids = [NationId::Guyana, NationId::Jamaica, NationId::Samoa, NationId::SaoTome, NationId::Belgium];
        let before: Vec<_> = ids.iter().map(|id| {
            let n = w.nation(*id);
            assert!(n.debt_gdp > 0.85);
            (*id, n.mil_spend_gdp, n.state_invest_gdp)
        }).collect();
        tick(&mut w);
        for (id, military, investment) in before {
            let n = w.nation(id);
            assert!(n.mil_spend_gdp <= military, "{} received unvoted defence", id.name());
            assert!(n.state_invest_gdp <= investment, "{} received unvoted investment", id.name());
        }
        assert_eq!(w.nation(NationId::Samoa).mil_spend_gdp, 0.0);
    }

    #[test]
    fn legacy_fiscal_policy_respects_stock_and_plan_ownership_independently() {
        let id = NationId::Italy;
        for plan_only in [false, true] {
            let mut w = fiscal_boundary_world(false, true, id);
            let allocations = w.nation(id).budget_for(w.year).allocations;
            crate::apply_command(&mut w, &crate::Command::SetAnnualBudget {
                nation: id, fiscal_year: 1990, allocations,
            }).unwrap();
            if plan_only {
                // Shape of an older explicit-plan save before dollar stocks.
                // An absent treasury must not revoke its adopted allocation.
                let n = w.nation_mut(id);
                n.treasury_bn = None;
                n.debt_bn = None;
                assert!(!n.on_the_books() && n.annual_budget.is_some());
            } else {
                // Explicit stock accounting can outlive an aggregate-budget
                // command, which deliberately clears the detailed plan.
                crate::apply_command(&mut w, &crate::Command::SetMilSpend {
                    nation: id, share: 0.06,
                }).unwrap();
                assert!(w.nation(id).on_the_books());
                assert!(w.nation(id).annual_budget.is_none());
            }
            assert!(!crate::programs::enrolled(&w, id));
            let n = w.nation(id);
            let before = (n.tax_rate, n.mil_spend_gdp, n.state_invest_gdp);
            tick(&mut w);
            let n = w.nation(id);
            assert_eq!((n.tax_rate, n.mil_spend_gdp, n.state_invest_gdp), before);
        }
    }
