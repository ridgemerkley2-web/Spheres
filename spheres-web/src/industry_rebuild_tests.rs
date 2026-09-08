use super::*;

fn fixture()->Game {
    let mut game=Game::new(1990,Some(NationId::USA));
    fresh_play_rules(&mut game).unwrap();
    programs::set_construction_budget(&mut game.world,NationId::USA,0.01).unwrap();
    game
}

#[test]
fn rebuild_catalog_roles_and_preset_preview_are_pure_and_actor_scoped() {
    let mut game=fixture();let me=NationId::USA;
    let before=spheres_sim::save(&game.world);
    let view=production_json(&game.world,me);
    assert_eq!(view["industry_rebuild"]["enabled"],true);
    for kind in ["civilian_industry","office_district","arms_plant","shipyard","generation","processing_plant","advanced_industry","industry_preset"] {
        let row=view["catalog"].as_array().unwrap().iter().find(|row|row["kind"]==kind).unwrap();
        assert!(!row["purpose"].as_str().unwrap().is_empty());
        assert!(!row["tradeoff"].as_str().unwrap().is_empty());
    }
    let d=game.world.districts.iter().find(|(_,n)|**n==me).unwrap().0.clone();
    let payload=serde_json::json!({"project_kind":"industry_preset","district":d});
    let quote=construction_preview_json(&game.world,me,&payload).unwrap();
    assert_eq!(quote["can_start"],true);assert_eq!(quote["components"].as_array().unwrap().len(),4);
    assert_eq!(before,spheres_sim::save(&game.world));
    assert!(construction_preview_json(&game.world,NationId::Japan,&payload).is_err());
    let cmd=parse_command(&game.world,&serde_json::json!({"kind":"start_industry_preset","district":d}),me).unwrap();
    spheres_sim::apply_command(&mut game.world,&cmd).unwrap();
    assert_eq!(game.world.production.projects.len(),4);
    assert!(construction_preview_json(&game.world,me,&payload).unwrap()["can_start"]==false);
}

#[test]
fn allocations_parse_validate_and_render_the_same_grants() {
    let mut game=fixture();let me=NationId::USA;
    let d=game.world.districts.iter().find(|(_,n)|**n==me).unwrap().0.clone();
    let id=production::start_project(&mut game.world,me,&d,ProjectKind::Infrastructure).unwrap();
    for invalid in [serde_json::json!(-1),serde_json::json!(21),serde_json::json!("10")] {
        let before=spheres_sim::save(&game.world);
        let cmd=parse_command(&game.world,&serde_json::json!({"kind":"set_project_allocation","project":id,"capacity":invalid}),me);
        assert!(cmd.is_none() || spheres_sim::apply_command(&mut game.world,&cmd.unwrap()).is_err());
        assert_eq!(before,spheres_sim::save(&game.world));
    }
    let cmd=parse_command(&game.world,&serde_json::json!({"kind":"set_project_allocation","project":id,"capacity":0}),me).unwrap();
    spheres_sim::apply_command(&mut game.world,&cmd).unwrap();
    let view=production_json(&game.world,me);
    assert_eq!(view["queue"][0]["allocation"]["assigned"],0.0);
    assert_eq!(view["queue"][0]["allocation"]["mode"],"manual");
    let cmd=parse_command(&game.world,&serde_json::json!({"kind":"set_project_allocation","project":id,"capacity":null}),me).unwrap();
    spheres_sim::apply_command(&mut game.world,&cmd).unwrap();
    assert!(production_json(&game.world,me)["queue"][0]["allocation"]["assigned"].as_f64().unwrap()>0.0);
}

#[test]
fn component_trade_is_an_explicit_third_good_only_in_rebuild() {
    let mut game=fixture();let me=NationId::USA;
    assert!(spheres_sim::commerce::snapshot(&game.world,me).goods.iter().any(|g|g.good==spheres_sim::commerce::Good::AdvancedComponents));
    game.world.rules.industry_rebuild=false;
    assert_eq!(spheres_sim::commerce::snapshot(&game.world,me).goods.len(),2);
    assert!(industry_rebuild_json(&game.world,me).is_null());
}
