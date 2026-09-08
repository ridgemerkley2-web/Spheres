//! HTTP parser/view regressions for operational campaigns. The simulation's
//! fixtures cover battlefield behavior; these cover the browser's actual door.
use super::*;
use serde_json::{json, Value};
use spheres_sim::campaign::{AirMission, Approach, NavalMission};
use spheres_sim::campaign_peace::{PeaceOrder, WarAim};

fn fixture() -> (Game, u32) {
    let mut g = Game::new(1990, Some(NationId::Iraq));
    play_rules(&mut g);
    g.world.nation_mut(NationId::Iraq).political_capital = 150.0;
    let theatre = spheres_sim::war::theatre_between(&g.world, NationId::Iraq, NationId::Iran);
    let cid = spheres_sim::commitment::open_conflict(
        &mut g.world,
        NationId::Iraq,
        NationId::Iran,
        theatre,
    )
    .unwrap();
    for b in &mut g.world.conflict_mut(cid).unwrap().posture {
        b.rung = 8;
    }
    (g, cid)
}
fn operation(cid: u32) -> Value {
    json!({"kind":"operation","conflict":cid,"nation":"Iran","target":null,
        "approach":"breakthrough","reserve_bp":1234,"air":"reconnaissance","naval":"escort"})
}
fn conflict_value<'a>(state: &'a Value, cid: u32) -> &'a Value {
    state["wars"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["id"] == cid)
        .unwrap()
}

#[test]
fn a_war_declared_before_the_first_browser_tick_still_requires_deployment() {
    let mut g = Game::new(1990, Some(NationId::Iraq));
    play_rules(&mut g);
    g.world.nation_mut(NationId::Iraq).political_capital = 150.0;
    spheres_sim::apply_command(
        &mut g.world,
        &Command::DeclareWar {
            attacker: NationId::Iraq,
            defender: NationId::Iran,
        },
    )
    .unwrap();
    let cid = g
        .world
        .conflicts
        .iter()
        .find(|c| {
            c.side_of(NationId::Iraq) == Some(true) && c.side_of(NationId::Iran) == Some(false)
        })
        .unwrap()
        .id;
    spheres_sim::war::tick(&mut g.world);
    let view = spheres_sim::campaign::view(&g.world, cid, NationId::Iraq).unwrap();
    assert_eq!(
        view.fielded, 0.0,
        "new orders cannot join the old-save migration army"
    );
    assert!(
        view.in_transit > 0.0,
        "first-day reinforcements have a real journey"
    );
}
#[test]
fn operation_parser_binds_the_actor_and_preserves_intent() {
    let (g, cid) = fixture();
    let command = parse_command(&g.world, &operation(cid), NationId::Iraq).unwrap();
    let Command::SetOperation { order } = command else {
        panic!("wrong command");
    };
    assert_eq!(order.nation, NationId::Iraq);
    assert_eq!(order.conflict, cid);
    assert_eq!(order.target, None);
    assert_eq!(order.reserve_bp, 1234);
    assert_eq!(order.approach, Approach::Breakthrough);
    assert_eq!(order.air, AirMission::Reconnaissance);
    assert_eq!(order.naval, NavalMission::Escort);
}

#[test]
fn operation_parser_refuses_invalid_numbers_missing_fields_and_unknown_enums() {
    let (g, cid) = fixture();
    for (field, bad) in [
        ("conflict", json!(-1)),
        ("conflict", json!(4294967296u64)),
        ("reserve_bp", json!(-1)),
        ("reserve_bp", json!(1.5)),
        ("reserve_bp", json!(65536)),
        ("reserve_bp", json!("1500")),
        ("target", json!(42)),
        ("approach", json!("march")),
        ("air", json!("fighter")),
        ("naval", json!("fleet")),
    ] {
        let mut payload = operation(cid);
        payload[field] = bad;
        assert!(
            parse_command(&g.world, &payload, NationId::Iraq).is_none(),
            "accepted {payload}"
        );
    }
    for field in [
        "conflict",
        "reserve_bp",
        "target",
        "approach",
        "air",
        "naval",
    ] {
        let mut payload = operation(cid);
        payload.as_object_mut().unwrap().remove(field);
        assert!(
            parse_command(&g.world, &payload, NationId::Iraq).is_none(),
            "accepted missing {field}"
        );
    }
}

#[test]
fn invalid_or_foreign_operations_leave_the_saved_world_unchanged() {
    let (mut g, cid) = fixture();
    for (actor, field, value) in [
        (NationId::Iraq, "reserve_bp", json!(10001)),
        (NationId::Iraq, "target", json!("not-a-district")),
        (NationId::France, "target", Value::Null),
        (NationId::Iraq, "conflict", json!(u32::MAX)),
    ] {
        let mut payload = operation(cid);
        payload[field] = value;
        let command = parse_command(&g.world, &payload, actor).unwrap();
        let before = save(&g.world);
        assert!(apply_command(&mut g.world, &command).is_err());
        assert_eq!(save(&g.world), before, "refused order changed the world");
    }
}

#[test]
fn diplomatic_parser_binds_the_actor_and_refuses_forged_nested_fields() {
    let (g, cid) = fixture();
    let payload = json!({"kind":"war_diplomacy","nation":"Iran","order":{"kind":"set_aim","conflict":cid,"aim":{"kind":"expel"}}});
    assert!(matches!(
        parse_command(&g.world, &payload, NationId::Iraq),
        Some(Command::WarDiplomacy {
            nation: NationId::Iraq,
            order: PeaceOrder::SetAim {
                aim: WarAim::Expel,
                ..
            }
        })
    ));
    for order in [
        json!({"kind":"set_aim","conflict":cid,"nation":"Iran","aim":{"kind":"expel"}}),
        json!({"kind":"garrison","conflict":cid,"share_bp":1.5,"policy":"security"}),
        json!({"kind":"garrison","conflict":cid,"share_bp":500,"policy":"annex"}),
        json!({"kind":"respond","offer":1,"accept":"true"}),
        json!({"kind":"propose","conflict":cid,"terms":{"kind":"reparations","share_bp":-1}}),
    ] {
        assert!(parse_command(
            &g.world,
            &json!({"kind":"war_diplomacy","order":order}),
            NationId::Iraq
        )
        .is_none());
    }
}

#[test]
fn campaign_views_are_pure_and_orders_survive_the_browser_load_path() {
    let (mut g, cid) = fixture();
    let command = parse_command(&g.world, &operation(cid), NationId::Iraq).unwrap();
    apply_command(&mut g.world, &command).unwrap();
    let before = save(&g.world);
    let first = state_json(&g, None);
    for _ in 0..3 {
        assert_eq!(state_json(&g, None), first);
    }
    assert_eq!(
        save(&g.world),
        before,
        "view consumed RNG or mutated campaign state"
    );
    let loaded = loaded_play_game(load(&before).unwrap());
    let restored = state_json(&loaded, None);
    assert_eq!(
        conflict_value(&first, cid)["operation"],
        conflict_value(&restored, cid)["operation"]
    );
    assert_eq!(
        conflict_value(&restored, cid)["operation"]["order"]["reserve_bp"],
        1234
    );
}

#[test]
fn protected_aim_retry_charges_political_capital_once() {
    let (mut g, cid) = fixture();
    let before = g.world.nation(NationId::Iraq).political_capital;
    let payload = json!({"session_id":g.session_id,"client_id":"campaign-api-test","request_seq":1,
        "commands":[{"kind":"war_diplomacy","order":{"kind":"set_aim","conflict":cid,"aim":{"kind":"concession"}}}]});
    let first = immediate_request(&mut g, &payload).unwrap();
    assert_eq!(first["errors"], json!([]));
    assert_eq!(
        g.world.nation(NationId::Iraq).political_capital,
        before - 3.0
    );
    let saved = save(&g.world);
    let retry = immediate_request(&mut g, &payload).unwrap();
    assert_eq!(retry["command_replayed"], true);
    assert_eq!(save(&g.world), saved);
}

#[test]
fn operational_enemy_views_hide_exact_posture_and_label_the_public_force_estimate() {
    let (mut g, cid) = fixture();
    g.world.nation_mut(NationId::Iran).mil_strength = 17.37542;
    g.world.nation_mut(NationId::Iraq).mil_strength = 127.37542;
    let state = state_json(&g, None);
    let war = conflict_value(&state, cid);
    let foe = war["posture"]
        .as_array()
        .unwrap()
        .iter()
        .find(|b| b["id"] == "Iran")
        .unwrap();
    for field in [
        "committed",
        "munitions",
        "resolve",
        "deployable",
        "red_line",
        "stake",
        "force_share_bp",
    ] {
        assert!(
            foe[field].is_null(),
            "exact enemy {field} leaked: {}",
            foe[field]
        );
    }
    let own = war["posture"]
        .as_array()
        .unwrap()
        .iter()
        .find(|b| b["id"] == "Iraq")
        .unwrap();
    assert!(own["resolve"].is_number());
    assert!(own["committed"].is_number());
    let iran = state["nations"]
        .as_array()
        .unwrap()
        .iter()
        .find(|n| n["id"] == "Iran")
        .unwrap();
    assert_eq!(iran["mil_strength"], 20.0);
    assert_eq!(iran["military_estimated"], true);
    let iraq = state["nations"]
        .as_array()
        .unwrap()
        .iter()
        .find(|n| n["id"] == "Iraq")
        .unwrap();
    assert_eq!(iraq["mil_strength"], 127.37542);
}
