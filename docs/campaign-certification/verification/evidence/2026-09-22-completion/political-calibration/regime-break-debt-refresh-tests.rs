    /// A change of government changes output, not the state's dollar debt or
    /// treasury. Every actual caller shares the same settlement invariant.
    #[test]
    fn coups_refresh_open_book_debt_ratios_without_rewriting_legacy_debt() {
        for route in 0..4 {
            for books in [false, true] {
                let id = match route {
                    0 => NationId::Pakistan,
                    2 => NationId::Jordan,
                    _ => NationId::China,
                };
                let rules = if route == 3 {
                    GameRules { seed: 7, ..GameRules::default() }
                } else { roads_rules(7) };
                let mut w = world_1990(rules);
                w.player = Some(id);
                if books {
                    let allocations = w.nation(id).budget_for(w.year).allocations;
                    crate::apply_command(&mut w, &crate::Command::SetAnnualBudget {
                        nation: id, fiscal_year: 1990, allocations,
                    }).unwrap();
                }
                {
                    let n = w.nation_mut(id);
                    n.stability = 25.0;
                    n.inflation = 0.03;
                    n.growth_last = 0.01;
                    n.war_exhaustion = 0.0;
                    n.separatism = 0.0;
                    n.debt_gdp = 0.65;
                    if books {
                        n.debt_bn = Some(n.gdp * 0.65);
                        n.treasury_bn = Some(4.0);
                        crate::economy::refresh_debt_ratio(n);
                    }
                    if route == 2 { n.authoritarianism = 0.38; }
                }
                {
                    let g = state_mut(&mut w, id).unwrap();
                    g.months_in_office = 48;
                    g.coup_pressure = 5.0;
                    for (pillar, loyalty) in &mut g.pillars {
                        *loyalty = if *pillar == Pillar::Army { 0.20 } else { 0.80 };
                    }
                }
                assert_eq!(w.nation(id).on_the_books(), books);
                let before = w.nation(id);
                let output = before.gdp;
                let debt = before.debt_bn;
                let cash = before.treasury_bn;
                let legacy_ratio = before.debt_gdp;
                let rng = w.rng.clone();
                match route {
                    0 => {
                        assert!(maybe_electoral_coup(&mut w, id));
                        assert!(w.headlines.iter().any(|h| h.contains("removes the elected government")));
                    }
                    2 => {
                        assert_eq!(annulment_check(&w, id).as_deref(), Some("jo_ikhwan"));
                        hold_election(&mut w, id);
                        assert!(w.headlines.iter().any(|h| h.contains("the army annuls the election")));
                        assert!(state(&w, id).unwrap().banned.contains(&"jo_ikhwan".to_string()));
                    }
                    _ => {
                        maybe_coup(&mut w, id);
                        assert!(w.headlines.iter().any(|h| h.contains("removes the government")));
                    }
                }
                let after = w.nation(id);
                assert_eq!(after.gdp.to_bits(), (output * 0.97).to_bits(), "route {route}");
                assert_eq!(after.debt_bn, debt, "a coup does not discharge dollar debt");
                assert_eq!(after.treasury_bn, cash, "a coup does not change cash");
                let expected = if books { debt.unwrap() / after.gdp } else { legacy_ratio };
                assert_eq!(after.debt_gdp.to_bits(), expected.to_bits(), "route {route}, books {books}");
                assert_eq!(w.rng, rng, "account synchronization never consumes a draw");
                let saved = crate::save(&w);
                let resumed = crate::load(&saved).unwrap();
                assert!(crate::save(&resumed) == saved,
                    "loading must not silently repair debt after route {route}, books {books}");
            }
        }
    }

    #[test]
    fn a_coup_tick_has_identical_fiscal_state_with_and_without_a_save_boundary() {
        let id = NationId::Pakistan;
        for books in [false, true] {
            let mut w = world_1990(roads_rules(7));
            w.player = Some(id);
            if books {
                let allocations = w.nation(id).budget_for(w.year).allocations;
                crate::apply_command(&mut w, &crate::Command::SetAnnualBudget {
                    nation: id, fiscal_year: 1990, allocations,
                }).unwrap();
            }
            {
                let n = w.nation_mut(id);
                n.stability = 25.0;
                n.inflation = 0.03;
                n.growth_last = 0.01;
                n.war_exhaustion = 0.0;
                n.separatism = 0.0;
                let g = state_mut(&mut w, id).unwrap();
                g.months_in_office = 48;
                g.coup_pressure = 5.0;
                for (pillar, loyalty) in &mut g.pillars {
                    *loyalty = if *pillar == Pillar::Army { 0.20 } else { 0.80 };
                }
            }
            let debt = w.nation(id).debt_bn;
            let cash = w.nation(id).treasury_bn;
            let output = w.nation(id).gdp;
            let legacy_ratio = w.nation(id).debt_gdp;
            tick(&mut w);
            assert!(w.headlines.iter().any(|h| h.starts_with("COUP IN PAKISTAN:")
                && h.contains("removes the elected government")));
            assert_eq!(w.nation(id).gdp.to_bits(), (output * 0.97).to_bits());
            assert_eq!(w.nation(id).debt_bn, debt);
            assert_eq!(w.nation(id).treasury_bn, cash);
            let expected = if books { debt.unwrap() / w.nation(id).gdp } else { legacy_ratio };
            assert_eq!(w.nation(id).debt_gdp.to_bits(), expected.to_bits());
            let saved = crate::save(&w);
            let mut resumed = crate::load(&saved).unwrap();
            assert!(crate::save(&resumed) == saved, "books {books}");
            for _ in 0..2 {
                let uninterrupted_news = crate::tick_month(&mut w, &[]);
                let resumed_news = crate::tick_month(&mut resumed, &[]);
                assert_eq!(uninterrupted_news, resumed_news);
                assert_eq!(crate::state_hash(&w), crate::state_hash(&resumed),
                    "the load boundary changed the campaign, books {books}");
            }
        }
    }
