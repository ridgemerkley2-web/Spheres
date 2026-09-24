pub use spheres_sim::*;
pub use spheres_sim::world::*;
use spheres_sim::init::world_1990;
use spheres_sim::government::{tick, GovState, Pillar};
fn roads_rules(seed:u64)->GameRules{GameRules{seed,ideology_blocs:true,ideology_takeover:true,..Default::default()}}
fn state_mut(w:&mut WorldState,id:NationId)->Option<&mut GovState>{w.governments.states.iter_mut().find(|g|g.nation==id)}
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
