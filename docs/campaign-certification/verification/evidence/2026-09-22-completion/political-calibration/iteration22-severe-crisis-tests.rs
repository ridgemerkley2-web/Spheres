
    // Counterfactual mechanism fixture, NOT a historical claim about Britain or
    // an empirical reading of V-Dem. The existing UK row supplies a complete
    // party table, a sourced Army and a verified source-zero assessment. Only
    // mutable campaign leverage is changed; immutable historical input stays 0.
    fn fully_provisioned_severe_civilian_crisis_fixture() -> WorldState {
        let id = NationId::UK;
        let mut w = world_1990(roads_rules(7));
        w.player = Some(id);
        {
            let n = w.nation_mut(id);
            n.gdp = 1_000.0; n.population = 100.0;
            n.tax_rate = 0.50; n.debt_gdp = 0.0; n.state_invest_gdp = 0.03;
            n.mil_spend_gdp = 0.001; n.political_capital = 100.0;
            n.authoritarianism = 0.52;
            n.stability = 85.0; n.inflation = 0.02; n.growth_last = 0.03;
            n.war_exhaustion = 0.0; n.separatism = 0.0;
        }
        {
            let g = state_mut(&mut w,id).unwrap();
            g.coalition = vec!["uk_con".into()];
            for (party,share) in &mut g.support {
                *share = if party == "uk_con" { 0.80 } else if party == "uk_lab" { 0.20 } else { 0.0 };
            }
        }
        hold_election(&mut w,id);
        assert_eq!(state(&w,id).unwrap().leader(),Some("uk_con"));
        assert!(state(&w,id).unwrap().elected && state(&w,id).unwrap().unrestricted_mandate);
        state_mut(&mut w,id).unwrap().pillars.iter_mut()
            .find(|(p,_)| *p == Pillar::Army).unwrap().1 = 0.85;
        let n = w.nation(id);
        let full_resources = 2.0 * army_operating_allowance(n.gdp * 1000.0 / n.population);
        let full_share = full_resources * army_personnel_assessment(&w,id).unwrap().members
            / (n.gdp * 1_000_000_000.0);
        let command = crate::Command::SetMilSpend { nation:id, share:full_share };
        let price = crate::price_of(&w,&command).unwrap();
        assert!(price > 0.0 && full_share < 0.35);
        let capital = w.nation(id).political_capital;
        let loyalty = state(&w,id).unwrap().loyalty(Pillar::Army);
        crate::apply_command(&mut w,&command).unwrap();
        assert!((w.nation(id).political_capital - (capital-price)).abs() < 1e-12);
        assert_eq!(state(&w,id).unwrap().loyalty(Pillar::Army),loyalty,
            "paid appropriation allocates resources without granting immediate obedience");
        assert!((army_resources_per_member(&w,id).unwrap()-full_resources).abs() < 1e-8);
        assert!(crate::blocs::backing(&w,id).iter().all(|(_,v)| *v == 0.0));
        assert_eq!(w.nation(id).war_exhaustion,0.0);
        // Explicit counterfactual observed state, not a historical claim or
        // an empirical source estimate. The prior same-party record supplies
        // the stated 24-month/.75-crisis input; the forward loyalty, pressure
        // and government response below must still occur through real ticks.
        {
            let n=w.nation_mut(id);
            // D = .50*(.30+.40) + .20*1 + .20*1 = .75. Separatism is an
            // existing crisis input; there is no war or foreign backing.
            n.stability=42.0;n.inflation=0.18;n.growth_last=-0.04;n.separatism=1.0;
            let g=state_mut(&mut w,id).unwrap();
            for (party,share) in &mut g.support {
                *share=if party=="uk_con" {0.25} else if party=="uk_lab" {0.75} else {0.0};
            }
            g.political_record=Some(PoliticalRecord {
                government:"party:uk_con".into(),months:24.0,performance:-0.75,
            });
            assert_eq!(g.army_authority.as_ref().unwrap().source.assessment,0.0);
            g.army_authority.as_mut().unwrap().current_leverage=0.80;
            assert_eq!(g.coup_pressure,0.0);
        }
        w
    }

    #[test]
    fn a_fully_provisioned_army_can_lose_confidence_after_a_sustained_severe_civilian_crisis() {
        let id = NationId::UK;
        let mut w = fully_provisioned_severe_civilian_crisis_fixture();
        assert!((crate::blocs::discontent(&w,id)-0.75).abs() < 1e-12);
        assert!((civilian_public_mandate(&w,id)-0.25).abs() < 1e-12);
        assert!((army_civilian_confidence_penalty(&w,id)-0.54).abs() < 1e-12,
            "the predeclared 1.20 response is a game-design relationship, not a source estimate");
        assert!((pillar_targets(&w,id,&[Pillar::Army])[0].1-0.31).abs() < 1e-12);
        let resources = army_resources_per_member(&w,id).unwrap();
        let strength = w.nation(id).mil_strength;
        let rng = w.rng.clone();
        assert!(!maybe_electoral_coup(&mut w,id), "the target alone is not a coup");
        let mut first_hostile = None;
        let mut coup = None;
        let mut recovered = None;
        for month in 1..=48 {
            (w.year,w.month) = add_months(1990,1,month-1);
            tick(&mut w);
            assert_eq!(w.rng,rng,"the real government walk remains deterministic");
            if !is_electoral(&w,id) { coup=Some(month); break; }
            assert_eq!(w.nation(id).mil_strength,strength);
            assert_eq!(army_resources_per_member(&w,id),Some(resources));
            if state(&w,id).unwrap().loyalty(Pillar::Army) < ELECTORAL_COUP_ARMY && first_hostile.is_none() {
                first_hostile=Some(month);
                recovered=Some(crate::load(&crate::save(&w)).unwrap());
            }
            if month <= 3 {
                assert_eq!(state(&w,id).unwrap().coup_pressure,0.0,
                    "funded confidence cannot disappear instantly");
            }
        }
        let hostile = first_hostile.expect("sustained crisis must eventually overcome the material cushion");
        let coup = coup.expect("real pressure and government tick must eventually permit the hostile Army to move");
        assert!(hostile > 3 && coup > hostile && coup <= 48,
            "smoothing then pressure, not a forced historical event: hostile={hostile}, coup={coup}");
        assert!(w.headlines.iter().any(|h|h.contains(&format!("COUP IN {}",id.name().to_uppercase()))));
        assert_eq!(state(&w,id).unwrap().army_authority.as_ref().unwrap().source.assessment,0.0,
            "campaign seizure cannot rewrite the source-zero historical assessment");
        // A recovery observed after hostility, before the break, blocks the
        // live trigger even with inherited pressure and a bad remembered term.
        let mut recovery = recovered.unwrap();
        let old_pressure = state(&recovery,id).unwrap().coup_pressure;
        assert!(old_pressure > 0.0);
        {
            let n=recovery.nation_mut(id);
            n.stability=85.0; n.inflation=0.02; n.growth_last=0.03; n.separatism=0.0;
        }
        assert_eq!(army_civilian_confidence_penalty(&recovery,id),0.0);
        assert!(!maybe_electoral_coup(&mut recovery,id));
        let recovered_on=(recovery.year,recovery.month);
        for month in 1..=24 {
            (recovery.year,recovery.month)=add_months(recovered_on.0,recovered_on.1,month);
            tick(&mut recovery);
            assert!(is_electoral(&recovery,id));
        }
        assert_eq!(state(&recovery,id).unwrap().coup_pressure,0.0);
        assert!(state(&recovery,id).unwrap().loyalty(Pillar::Army)>0.60);
        assert_eq!(army_resources_per_member(&recovery,id),Some(resources));
    }

    #[test]
    fn confidence_keeps_quiet_consent_record_and_known_zero_guards() {
        let id=NationId::UK;
        let base=fully_provisioned_severe_civilian_crisis_fixture();
        let source=state(&base,id).unwrap().army_authority.as_ref().unwrap().source.clone();
        let mut quiet=base.clone();
        {
            let n=quiet.nation_mut(id);
            n.stability=85.0;n.inflation=0.02;n.growth_last=0.03;n.separatism=0.0;
        }
        let mut strong=base.clone();
        for (party,share) in &mut state_mut(&mut strong,id).unwrap().support {
            *share=if party=="uk_con" {0.80} else if party=="uk_lab" {0.20} else {0.0};
        }
        assert!((civilian_public_mandate(&strong,id)-0.80).abs()<1e-12);
        assert!((pillar_targets(&strong,id,&[Pillar::Army])[0].1-0.706).abs()<1e-12);
        let mut low=base.clone();
        state_mut(&mut low,id).unwrap().army_authority.as_mut().unwrap().current_leverage=0.10;
        assert!((pillar_targets(&low,id,&[Pillar::Army])[0].1-0.7825).abs()<1e-12);
        let mut zero=base.clone();
        state_mut(&mut zero,id).unwrap().army_authority.as_mut().unwrap().current_leverage=source.assessment;
        assert_eq!(source.assessment,0.0);
        assert_eq!(army_civilian_confidence_penalty(&zero,id),0.0);
        let mut unknown=zero.clone();
        state_mut(&mut unknown,id).unwrap().army_authority=None;
        assert!((civilian_army_executive_leverage(&unknown,id)-0.80).abs()<1e-12);
        assert!((pillar_targets(&unknown,id,&[Pillar::Army])[0].1-0.31).abs()<1e-12,
            "missing source keeps the explicitly documented old-save proxy; known zero is not missing");
        let old=crate::save(&unknown);
        unknown=crate::load(&old).unwrap();ensure(&mut unknown,id);
        assert!(state(&unknown,id).unwrap().army_authority.is_none());
        let mut fresh=base.clone();
        state_mut(&mut fresh,id).unwrap().political_record.as_mut().unwrap().months=0.0;
        for month in 1..=5 {
            (fresh.year,fresh.month)=add_months(1990,1,month-1);
            tick(&mut fresh);
            assert_eq!(army_civilian_confidence_penalty(&fresh,id),0.0);
            assert_eq!(state(&fresh,id).unwrap().coup_pressure,0.0);
        }
        for month in 1..=36 {
            for w in [&mut quiet,&mut low,&mut zero] {
                (w.year,w.month)=add_months(1990,1,month-1);
                tick(w);
                assert!(is_electoral(w,id));
                assert_eq!(state(w,id).unwrap().coup_pressure,0.0);
            }
            // Strong current consent is protection while held, not a promise
            // of immunity after later real electoral support has disappeared.
            if month<=6 {
                (strong.year,strong.month)=add_months(1990,1,month-1);
                tick(&mut strong);
                assert!(is_electoral(&strong,id));
                assert_eq!(state(&strong,id).unwrap().coup_pressure,0.0);
            }
        }
        assert!(state(&quiet,id).unwrap().loyalty(Pillar::Army)>0.80);
        assert!(state(&low,id).unwrap().loyalty(Pillar::Army)>0.65);
        assert_eq!(state(&zero,id).unwrap().army_authority.as_ref().unwrap().source,source);
        // Source-zero blocks only the confidence channel. Actual neglect and
        // war retain the original material target and are never waived.
        zero.nation_mut(id).mil_spend_gdp=0.0;
        zero.nation_mut(id).war_exhaustion=0.20;
        assert!((pillar_targets(&zero,id,&[Pillar::Army])[0].1-0.11).abs()<1e-12);
        // The existing D=.25 eligibility discontinuity is explicit. Test the
        // actual public conditions on both sides, rather than smoothing or
        // silently moving the threshold to suit the stronger response.
        let mut edge=base.clone();
        {
            let n=edge.nation_mut(id);
            n.stability=30.0;n.inflation=0.02;n.growth_last=0.03;
            n.separatism=0.0;n.war_exhaustion=0.0;
        }
        assert!((crate::blocs::discontent(&edge,id)-ELECTORAL_COUP_DISCONTENT).abs()<1e-12);
        assert!((army_civilian_confidence_penalty(&edge,id)-0.36).abs()<1e-12);
        edge.nation_mut(id).stability=30.000001;
        assert!(crate::blocs::discontent(&edge,id)<ELECTORAL_COUP_DISCONTENT);
        assert_eq!(army_civilian_confidence_penalty(&edge,id),0.0);
        // Maximum finite crisis includes the existing war pain, unlike the
        // no-war positive scenario. This bounds the confidence response itself
        // at1.20 and verifies the unchanged final loyalty target clamp.
        let mut maximum=base;
        {
            let n=maximum.nation_mut(id);
            n.stability=0.0;n.inflation=0.18;n.growth_last=-0.04;
            n.separatism=0.0;n.war_exhaustion=1.0;
            let g=state_mut(&mut maximum,id).unwrap();
            for (party,share) in &mut g.support { *share=if party=="uk_lab" {1.0} else {0.0}; }
            g.political_record.as_mut().unwrap().performance=-1.0;
            g.army_authority.as_mut().unwrap().current_leverage=1.0;
        }
        assert!((crate::blocs::discontent(&maximum,id)-1.0).abs()<1e-12);
        assert_eq!(civilian_public_mandate(&maximum,id),0.0);
        assert!((army_civilian_confidence_penalty(&maximum,id)-1.20).abs()<1e-12);
        assert_eq!(pillar_targets(&maximum,id,&[Pillar::Army])[0].1,0.0);
    }

    #[test]
    fn severe_confidence_crisis_does_not_authorize_free_or_useless_military_spending() {
        let id=NationId::UK;
        let mut w=fully_provisioned_severe_civilian_crisis_fixture();
        w.player=None;
        // Exercise an eligible review, not merely the annual loyalty guard.
        // This spending-only control explicitly starts with a hostile pillar;
        // the positive scenario above earns hostility through real time.
        state_mut(&mut w,id).unwrap().pillars.iter_mut()
            .find(|(p,_)| *p==Pillar::Army).unwrap().1=0.34;
        assert!(crate::clock::month_end(&w));
        assert!(state(&w,id).unwrap().loyalty(Pillar::Army)<0.40);
        let full=w.nation(id).mil_spend_gdp;
        assert!((ai_army_funding_floor(&w,id).unwrap()-full).abs()<1e-12,
            "even an unreachable political target must stop at full useful provision");
        let before=crate::save(&w);
        let target=pillar_targets(&w,id,&[Pillar::Army])[0].1;
        let capital=w.nation(id).political_capital;
        ai_government(&mut w);
        assert_eq!(w.nation(id).mil_spend_gdp,full);
        assert_eq!(w.nation(id).political_capital,capital);
        assert_eq!(pillar_targets(&w,id,&[Pillar::Army])[0].1,target);
        assert!(target<ELECTORAL_COUP_ARMY);
        let mut loaded=crate::load(&before).unwrap();
        assert_eq!(ai_army_funding_floor(&loaded,id),ai_army_funding_floor(&w,id));
        let rng=loaded.rng.clone();
        loaded.rules.ideology_blocs=false;loaded.rules.ideology_takeover=false;
        let pillars=state(&loaded,id).unwrap().pillars.clone();
        assert_eq!(army_civilian_confidence_penalty(&loaded,id),0.0);
        assert_eq!(ai_army_funding_floor(&loaded,id),None);
        electoral_army_tick(&mut loaded,id);
        assert_eq!(state(&loaded,id).unwrap().pillars,pillars);
        assert_eq!(loaded.rng,rng);
    }
