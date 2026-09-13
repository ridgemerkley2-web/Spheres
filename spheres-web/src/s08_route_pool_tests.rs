//! Runtime ownership and full-tick parity, using explicit synthetic calendar
//! fixtures. These checks do not establish earned supplier stock or campaign
//! availability. The genuine imported campaign has a separate 31-day driver.
use super::*;

fn fixture(daily: bool, year: i32, month: u32, day: u32) -> Game {
    let mut world = world_1990(GameRules {
        seed: 1990, resource_gates: true, resource_market: true,
        logistics_routes: true, physical_logistics: true,
        military_operations: true, daily_simulation: daily,
        ..GameRules::default()
    });
    world.player = Some(NationId::France);
    world.year = year; world.month = month; world.day = day;
    loaded_play_game(world)
}

fn compare_day(game: &mut Game, cold: &mut WorldState, commands: Vec<Command>) {
    let before_log = game.log.len();
    // The public cold driver retains a fresh search context every clearing.
    // Only the real Game path can access the separately owned retained pool.
    let headlines = tick_day(cold, &commands);
    game.advance_days(1, commands);
    assert!(save(&game.world) == save(cold),
        "Game-owned routes changed the complete native world on {}", cold.date_str());
    assert_eq!(game.log[before_log..].iter().map(|event| event.text.as_str()).collect::<Vec<_>>(),
        headlines.iter().map(String::as_str).collect::<Vec<_>>());
    assert!(game.freight_routes.entry_count() <= 256);
}

#[test]
fn s08_game_route_pool_preserves_cold_full_ticks_and_command_order_across_calendars() {
    assert_eq!(spheres_sim::SYSTEMS.iter().filter(|(name, _)| *name == "arsenal").count(), 1);
    for daily in [false, true] {
        for (year, month, day) in [(1990, 1, 30), (1992, 2, 28), (1990, 12, 31)] {
            let mut game = fixture(daily, year, month, day);
            assert!(game.freight_routes.is_empty());
            let mut cold = game.world.clone();
            let mut used_pool = false;
            for offset in 0..4 {
                let commands = match offset {
                    1 => vec![Command::SetLogisticsPolicy { nation: NationId::France, policy: RoutePolicy::LandOnly }],
                    2 => vec![Command::SetLogisticsPolicy { nation: NationId::France, policy: RoutePolicy::AvoidChokepoints }],
                    _ => vec![],
                };
                compare_day(&mut game, &mut cold, commands);
                used_pool |= !game.freight_routes.is_empty();
            }
            assert!(used_pool, "the fixture must perform actual nominal searches");
        }
    }
}

#[test]
fn s08_game_route_pool_is_private_to_campaign_and_resets_on_save_load() {
    let mut original = fixture(true, 1990, 1, 30);
    let mut cold = original.world.clone();
    compare_day(&mut original, &mut cold, vec![]);
    assert!(!original.freight_routes.is_empty());
    let retained = original.freight_routes.entry_count();
    let before = save(&original.world);

    let encoded = storage::encode(&original).unwrap();
    let envelope: serde_json::Value = serde_json::from_str(&encoded).unwrap();
    assert!(envelope.get("freight_routes").is_none());
    assert!(envelope["world"].get("freight_routes").is_none());
    let mut resumed = storage::decode(&encoded).unwrap();
    assert!(resumed.freight_routes.is_empty(), "a restored campaign must not inherit process search state");
    assert!(save(&resumed.world) == before, "restoring the archive changed the complete world");
    assert_eq!(serde_json::to_vec(&resumed.history).unwrap(), serde_json::to_vec(&original.history).unwrap());
    assert_eq!(serde_json::to_vec(&resumed.log).unwrap(), serde_json::to_vec(&original.log).unwrap());

    let cloned_world = original.world.clone();
    let raw_restored = loaded_play_game(cloned_world);
    assert!(raw_restored.freight_routes.is_empty());
    assert!(Game::new(1990, Some(NationId::France)).freight_routes.is_empty());
    assert_eq!(original.freight_routes.entry_count(), retained, "loading another Game cannot take this Game's pool");
    assert!(save(&original.world) == before, "a second Game's load changed the first Game");

    for _ in 0..3 {
        // Two live Games with matching nation IDs and independent caches.
        // The resumed one starts cold; both must retain the same complete
        // world, archives and command-free progression as the cold driver.
        compare_day(&mut original, &mut cold, vec![]);
        resumed.advance_days(1, vec![]);
        assert!(save(&resumed.world) == save(&original.world));
        assert_eq!(serde_json::to_vec(&resumed.history).unwrap(), serde_json::to_vec(&original.history).unwrap());
        assert_eq!(serde_json::to_vec(&resumed.log).unwrap(), serde_json::to_vec(&original.log).unwrap());
    }
}
