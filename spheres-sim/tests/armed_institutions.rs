//! Each recorded institution receives one loyalty update per simulation step.
//! Sudan legitimately records two Army institutions; neither is a spare copy.
use spheres_sim::{government::{self, Pillar}, init::world_1990, world::{GameRules, NationId}};

#[test]
fn parallel_armed_institutions_each_receive_one_update() {
    let id = NationId::Sudan;
    for daily in [false, true] {
        for prior in [[0.65, 0.65], [0.20, 0.90]] {
            let mut w = world_1990(GameRules {
                seed: 0, ideology_blocs: true, ideology_takeover: true,
                daily_simulation: daily, ..GameRules::default()
            });
            w.player = Some(id); // no AI appropriation or patronage in this probe
            w.nation_mut(id).political_capital = 0.0;
            let g = w.governments.states.iter_mut().find(|g| g.nation == id).unwrap();
            let army: Vec<_> = g.pillars.iter().enumerate()
                .filter(|(_, (p, _))| *p == Pillar::Army).map(|(i, _)| i).collect();
            assert_eq!(army.len(), 2, "both sourced Sudanese institutions remain recorded");
            for (index, loyalty) in army.iter().zip(prior) { g.pillars[*index].1 = loyalty; }

            let saved = spheres_sim::save(&w);
            w = spheres_sim::load(&saved).unwrap();
            assert_eq!(spheres_sim::save(&w), saved, "loading must preserve both institutions");
            let mut expected = Vec::new();
            for keep in &army {
                // Independent control: the same institution, same inputs and
                // same public tick, with the other Army institution absent.
                let mut control = w.clone();
                let g = control.governments.states.iter_mut().find(|g| g.nation == id).unwrap();
                g.pillars = g.pillars.iter().enumerate()
                    .filter(|(i, (p, _))| *p != Pillar::Army || i == keep)
                    .map(|(_, row)| *row).collect();
                government::tick(&mut control);
                expected.push(government::state(&control, id).unwrap().loyalty(Pillar::Army));
            }
            government::tick(&mut w);
            let actual: Vec<_> = government::state(&w, id).unwrap().pillars.iter()
                .filter(|(p, _)| *p == Pillar::Army).map(|(_, v)| *v).collect();
            assert_eq!(actual.len(), 2);
            for (index, (actual, expected)) in actual.iter().zip(expected).enumerate() {
                assert!((actual - expected).abs() < 1e-12,
                    "daily={daily}, prior={prior:?}, institution={index}: {actual} != {expected}");
            }
        }
    }
}
