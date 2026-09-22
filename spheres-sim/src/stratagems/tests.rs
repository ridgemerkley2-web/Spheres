    use super::*;
    use crate::{government, init::world_1990, world::GameRules};

    #[test]
    fn an_affordable_round_table_reaches_the_real_ai_dispatcher() {
        let id = NationId::Albania;
        let mut w = world_1990(GameRules { ideology_blocs: true, seed: 7, ..Default::default() });
        // Isolate this government's decision opportunities. No monthly
        // standing recovery or other country's draw can pay the bill for it.
        for n in &mut w.nations { n.alive = n.id == id; }
        w.nation_mut(id).political_capital = government::ROUND_TABLE_PC;
        assert!(!government::is_electoral(&w, id));
        assert!(matches!(government::ai_lever(&w, id), Some(crate::Command::ConveneRoundTable { .. })));

        let mut off = w.clone();
        off.rules.ideology_blocs = false;
        let before = crate::save(&off);
        for _ in 0..200 { ai_stratagems(&mut off); }
        assert_eq!(crate::save(&off), before, "disabled levers consume neither standing nor random draws");
        let mut poor = w.clone();
        poor.nation_mut(id).political_capital = government::ROUND_TABLE_PC - 1.0;
        let before = crate::save(&poor);
        for _ in 0..200 { ai_stratagems(&mut poor); }
        assert_eq!(crate::save(&poor), before, "an unfunded lever never reaches the random draw");

        let mut attempts = 0;
        while !government::is_electoral(&w, id) && attempts < 1000 {
            ai_stratagems(&mut w);
            attempts += 1;
        }
        assert!(government::is_electoral(&w, id), "the real dispatch path never offered an affordable negotiation");
        assert_eq!(w.nation(id).political_capital, 0.0);
        assert_eq!(government::state(&w, id).unwrap().next_election, (1990, 7));
    }

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

    #[test]
    fn a_paid_consolidation_keeps_stock_only_aggregate_authority() {
        let id = NationId::Italy;
        let mut w = consolidation_world(false);
        let allocations = w.nation(id).budget_for(w.year).allocations;
        crate::apply_command(&mut w, &crate::Command::SetAnnualBudget {
            nation: id, fiscal_year: 1990, allocations,
        }).unwrap();
        // A legal aggregate vote can exceed an inherited detailed row's cap.
        // A later paid card must not silently convert it to a different owner.
        crate::apply_command(&mut w, &crate::Command::SetStateInvest {
            nation: id, share: 0.40,
        }).unwrap();
        let n = w.nation(id);
        assert!(n.on_the_books() && n.annual_budget.is_none());
        let before = (n.treasury_bn, n.debt_bn, n.debt_gdp, n.social_spend_gdp);
        let pc = n.political_capital;
        buy_consolidation(&mut w);
        let n = w.nation(id);
        assert_eq!((n.treasury_bn, n.debt_bn, n.debt_gdp, n.social_spend_gdp), before);
        assert!(n.annual_budget.is_none() && n.program_budget.is_none());
        assert!((n.state_invest_gdp - 0.28).abs() < 1e-12);
        assert!((n.mil_spend_gdp - 0.032).abs() < 1e-12);
        assert_eq!(n.political_capital, pc - 34.0);
    }

    fn consolidation_owner(w: &mut WorldState, owner: &str) {
        let id = NationId::Italy;
        let allocations = w.nation(id).budget_for(w.year).allocations;
        let command = match owner {
            "aggregate" => return,
            "annual" => crate::Command::SetAnnualBudget { nation: id, fiscal_year: 1990, allocations },
            "program" => crate::Command::SetProgramBudget {
                nation: id, fiscal_year: 1990, allocations,
                departments: crate::programs::default_departments(),
            },
            _ => unreachable!(),
        };
        crate::apply_command(w, &command).unwrap();
    }

    #[test]
    fn a_fiscal_consolidation_with_no_financial_effect_is_refused() {
        let id = NationId::Italy;
        for owner in ["aggregate", "annual", "program"] {
            for player in [false, true] {
                let mut w = consolidation_world(true);
                let n = w.nation_mut(id);
                n.tax_rate = 0.60;
                n.mil_spend_gdp = 0.005;
                n.state_invest_gdp = 0.02;
                consolidation_owner(&mut w, owner);
                if !player {
                    w.player = None;
                    w.rules.economic_competition = true;
                }
                let before = crate::save(&w);
                // The AI reads this same pure deck. It may choose another
                // legitimate action, but never this punitive financial no-op.
                assert!(!available(&w, id).iter().any(|s| s.id == "austerity"));
                assert!(crate::apply_command(&mut w, &crate::Command::EnactStratagem {
                    nation: id, id: "austerity".into(),
                }).is_err());
                assert_eq!(crate::save(&w), before, "{owner}, player={player}");
            }
        }
    }

    #[test]
    fn each_single_financial_lever_keeps_consolidation_available() {
        let id = NationId::Italy;
        for owner in ["aggregate", "annual", "program"] {
            for lever in ["tax", "investment", "defense"] {
                let mut w = consolidation_world(true);
                let n = w.nation_mut(id);
                n.tax_rate = if lever == "tax" { 0.59 } else { 0.60 };
                n.state_invest_gdp = if lever == "investment" { 0.03 } else { 0.02 };
                n.mil_spend_gdp = if lever == "defense" { 0.02 } else { 0.005 };
                consolidation_owner(&mut w, owner);
                let pc = w.nation(id).political_capital;
                assert!(available(&w, id).iter().any(|s| s.id == "austerity"));
                buy_consolidation(&mut w);
                let n = w.nation(id);
                assert_eq!(n.political_capital, pc - 34.0);
                assert_eq!(n.stability, 71.0);
                assert!((n.tax_rate - 0.60).abs() < 1e-12);
                assert!((n.state_invest_gdp - if lever == "investment" { 0.021 } else { 0.02 }).abs() < 1e-12);
                assert!((n.mil_spend_gdp - if lever == "defense" { 0.016 } else { 0.005 }).abs() < 1e-12);
            }
        }
    }


