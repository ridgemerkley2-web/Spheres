pub use spheres_sim::*;
pub use spheres_sim::world::*;
use spheres_sim::init::world_1990;
    fn consolidation_world(daily: bool) -> WorldState {
        let mut w = world_1990(GameRules { daily_simulation: daily, ..Default::default() });
        let id = NationId::Italy;
        w.player = Some(id);
        let n = w.nation_mut(id);
        n.debt_gdp = 1.30;
        n.tax_rate = 0.30;
        n.mil_spend_gdp = 0.04;
        n.state_invest_gdp = 0.08;
        n.stability = 80.0;
        n.political_capital = 100.0;
        w
    }

    fn buy_consolidation(w: &mut WorldState) {
        crate::apply_command(w, &crate::Command::EnactStratagem {
            nation: NationId::Italy, id: "austerity".into(),
        }).unwrap();
    }

    #[test]
    fn the_paid_consolidation_card_cuts_without_inventing_minimum_spending() {
        let id = NationId::Italy;
        for daily in [false, true] {
            for (mil, investment, expected_mil, expected_investment) in [
                (0.0, 0.0, 0.0, 0.0),
                (0.004, 0.014, 0.004, 0.014),
                (0.005, 0.020, 0.005, 0.020),
                (0.04, 0.08, 0.032, 0.056),
            ] {
                let mut w = consolidation_world(daily);
                let n = w.nation_mut(id);
                n.mil_spend_gdp = mil;
                n.state_invest_gdp = investment;
                buy_consolidation(&mut w);
                let n = w.nation(id);
                assert!((n.tax_rate - 0.35).abs() < 1e-12);
                assert!((n.mil_spend_gdp - expected_mil).abs() < 1e-12);
                assert!((n.state_invest_gdp - expected_investment).abs() < 1e-12);
                assert!(n.mil_spend_gdp <= mil && n.state_invest_gdp <= investment);
                assert_eq!(n.political_capital, 66.0);
                assert_eq!(n.stability, 71.0);
                assert!(!n.on_the_books() && n.annual_budget.is_none());
            }
        }
        let mut capped = consolidation_world(false);
        capped.nation_mut(id).tax_rate = 0.60;
        buy_consolidation(&mut capped);
        assert_eq!(capped.nation(id).tax_rate, 0.60);
        let mut refused = consolidation_world(false);
        refused.nation_mut(id).political_capital = 33.99;
        let before = crate::save(&refused);
        assert!(crate::apply_command(&mut refused, &crate::Command::EnactStratagem {
            nation: id, id: "austerity".into(),
        }).is_err());
        assert_eq!(crate::save(&refused), before);
    }

    #[test]
    fn the_paid_consolidation_card_updates_owned_plans_and_charges_once() {
        let id = NationId::Italy;
        for program in [false, true] {
            for small in [false, true] {
                let mut w = consolidation_world(true);
                if small {
                    w.nation_mut(id).mil_spend_gdp = 0.004;
                    w.nation_mut(id).state_invest_gdp = 0.014;
                }
                let allocations = w.nation(id).budget_for(w.year).allocations;
                let command = if program {
                    // A pre-existing legacy prepaid entitlement, deliberately
                    // not a cash grant from this consolidation package.
                    w.nation_mut(id).arsenal.banked = 2.0;
                    crate::Command::SetProgramBudget {
                        nation: id, fiscal_year: 1990, allocations,
                        departments: crate::programs::default_departments(),
                    }
                } else {
                    crate::Command::SetAnnualBudget { nation: id, fiscal_year: 1990, allocations }
                };
                crate::apply_command(&mut w, &command).unwrap();
                if program {
                    crate::programs::begin_day(&mut w);
                    let available = crate::programs::available_bn(&w, id, BUDGET_INDUSTRY, 0);
                    assert!(available > 0.0);
                    crate::programs::spend(&mut w, id, BUDGET_INDUSTRY, 0, available / 4.0).unwrap();
                }
                let n = w.nation(id);
                let before = n.budget_for(w.year);
                let ledger = n.program_budget.clone();
                let stocks = (n.treasury_bn, n.debt_bn, n.debt_gdp);
                let pc = n.political_capital;
                buy_consolidation(&mut w);
                let n = w.nation(id);
                let after = n.annual_budget.as_ref().unwrap();
                assert_eq!(after.reference, before.reference);
                assert_eq!(after.social_total(), before.social_total());
                assert_eq!(n.program_budget, ledger, "authority, prepaid funds and dated receipts remain exact");
                assert_eq!((n.treasury_bn, n.debt_bn, n.debt_gdp), stocks);
                assert_eq!(n.political_capital, pc - 34.0, "the card has one political price");
                assert_eq!(n.mil_spend_gdp, after.defense());
                assert_eq!(n.state_invest_gdp, after.investment_total());
                assert!((after.defense() - if small { 0.004 } else { 0.032 }).abs() < 1e-12);
                assert!((after.investment_total() - if small { 0.014 } else { 0.056 }).abs() < 1e-12);
                for ministry in [BUDGET_INFRASTRUCTURE, BUDGET_INDUSTRY, BUDGET_SCIENCE] {
                    let expected = before.allocations[ministry] * if small { 1.0 } else { 0.7 };
                    assert!((after.allocations[ministry] - expected).abs() < 1e-12);
                }
            }
        }
    }

    #[test]
    fn an_unowned_program_consolidation_is_refused_before_any_effect() {
        let id = NationId::Italy;
        let mut w = consolidation_world(true);
        let allocations = w.nation(id).budget_for(w.year).allocations;
        crate::apply_command(&mut w, &crate::Command::SetProgramBudget {
            nation: id, fiscal_year: 1990, allocations,
            departments: crate::programs::default_departments(),
        }).unwrap();
        w.player = None;
        assert!(!crate::economic_ai::enabled(&w));
        let before = crate::save(&w);
        assert!(crate::apply_command(&mut w, &crate::Command::EnactStratagem {
            nation: id, id: "austerity".into(),
        }).is_err());
        assert_eq!(crate::save(&w), before);
        w.rules.economic_competition = true;
        let pc = w.nation(id).political_capital;
        buy_consolidation(&mut w);
        assert_eq!(w.nation(id).political_capital, pc - 34.0);
        assert!((w.nation(id).mil_spend_gdp - 0.032).abs() < 1e-12);
    }
