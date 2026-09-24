use spheres_sim::world::{GameRules, WorldState, NationId};
use spheres_sim::init::world_1990;
use spheres_sim::government::{Pillar, GovState, ai_army_funding_floor, tick};
fn roads_rules(seed:u64)->GameRules { GameRules{seed,ideology_blocs:true,ideology_takeover:true,ai_aggression:0.0,..GameRules::default()} }
fn state_mut(w:&mut WorldState,id:NationId)->Option<&mut GovState> { w.governments.states.iter_mut().find(|g|g.nation==id) }
    #[test]
    fn an_explicit_legacy_plan_does_not_let_the_army_ai_erase_its_owner() {
        let id = NationId::Pakistan;
        let mut legacy = world_1990(roads_rules(7));
        legacy.player = None;
        let n = legacy.nation_mut(id);
        n.mil_spend_gdp = 0.001;
        n.state_invest_gdp = 0.02;
        n.social_spend_gdp = Some(0.08);
        n.tax_rate = 0.55;
        n.political_capital = 100.0;
        n.stability = 85.0;
        n.inflation = 0.02;
        n.growth_last = 0.03;
        n.war_exhaustion = 0.0;
        n.separatism = 0.0;
        n.annual_budget = None;
        n.treasury_bn = None;
        n.debt_bn = None;
        let g = state_mut(&mut legacy,id).unwrap();
        for (p,loyalty) in &mut g.pillars { *loyalty = if *p == Pillar::Army {0.20} else {0.80}; }
        let floor = ai_army_funding_floor(&legacy,id).unwrap();
        assert!(floor >= legacy.nation(id).mil_spend_gdp + 0.001);
        assert!(spheres_sim::affordable(&legacy,&spheres_sim::Command::SetMilSpend {nation:id,share:floor}));
        let plan = legacy.nation(id).budget_for(legacy.year);
        let mut planned = legacy.clone();
        planned.nation_mut(id).annual_budget = Some(plan.clone());
        assert!(!planned.nation(id).on_the_books(), "valid legacy plan, before dollar stocks existed");
        let saved = spheres_sim::save(&planned);
        let loaded = spheres_sim::load(&saved).unwrap();
        for mut w in [planned,loaded] {
            let before_floor = ai_army_funding_floor(&w,id);
            tick(&mut w);
            assert_eq!(w.nation(id).annual_budget,Some(plan.clone()),
                "the public government tick must not erase another fiscal owner's plan");
            assert_eq!(w.nation(id).mil_spend_gdp,0.001);
            assert_eq!(w.nation(id).treasury_bn,None);
            assert_eq!(w.nation(id).debt_bn,None);
            assert_eq!(before_floor,None,"no legacy appropriation policy for an explicit plan");
        }
        // The same actual resource problem still buys an affordable increase
        // when there is no explicit fiscal owner. This is not a disabled AI.
        let standing = legacy.nation(id).political_capital;
        tick(&mut legacy);
        assert!(legacy.nation(id).mil_spend_gdp > 0.001);
        assert!(legacy.nation(id).political_capital < standing);
        assert!(legacy.nation(id).annual_budget.is_none());
    }
