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

#[test]
fn construction_summary_and_markers_follow_live_inputs_and_assignments_without_settling() {
    let me=NationId::France;
    let mut game=Game::new(1990,Some(me));
    fresh_play_rules(&mut game).unwrap();
    tick_day(&mut game.world,&[]);
    programs::set_construction_budget(&mut game.world,me,0.01).unwrap();
    programs::begin_day(&mut game.world);
    resources::warm(&mut game.world);
    let district=game.world.districts.iter().find(|(id,owner)| **owner==me
        && spheres_sim::districts::name_of(id).is_some_and(|name|name.contains("le-de-France")))
        .expect("France has its capital province").0.clone();
    let market=game.world.resources.market.as_mut().unwrap();
    for commodity in ALL {
        let quantity=if commodity==Commodity::RareEarths {0.0}else{1000.0};
        if let Some(stock)=market.stocks.iter_mut().find(|stock|stock.nation==me&&stock.commodity==commodity) {
            stock.quantity=quantity;
        } else {
            market.stocks.push(resources::Stock{nation:me,commodity,quantity,reserve_target:0.0});
        }
    }
    market.stocks.sort_by_key(|stock|(stock.nation,stock.commodity));
    let id=production::start_project(&mut game.world,me,&district,ProjectKind::CivilianIndustry).unwrap();
    assert_eq!(game.world.production.projects[0].status,ProjectStatus::Building,
        "Newly queued status has not yet received a settlement");
    let assert_views=|game:&Game,expected:&str| {
        let before=spheres_sim::save(&game.world);
        let board=production_json(&game.world,me);
        let summary=production_summary_json(&game.world,me);
        assert_eq!(summary,board["summary"]);
        assert_eq!(board["queue"][0]["status"],expected);
        assert_eq!(summary[expected],1);
        assert_eq!(summary["building"],u64::from(expected=="building"));
        assert_eq!(summary["attention"],u64::from(expected!="building"));
        assert_eq!(summary["attention_ids"],if expected=="building" {serde_json::json!([])}else{serde_json::json!([id])});
        let marker=board["markers"].as_array().unwrap().iter().find(|marker|marker["district"]==district).unwrap();
        assert_eq!(marker["status"],expected);
        assert_eq!(state_json(game,None)["production_summary"],summary);
        assert_eq!(spheres_sim::save(&game.world),before,"Status reads must not settle work or alter RNG");
    };
    assert_views(&game,"paused");
    assert!(production_json(&game.world,me)["queue"][0]["reason"].as_str().unwrap().contains("rare"));

    // Receiving the missing input resumes the displayed work immediately;
    // no day must elapse for the summary to agree with the construction card.
    game.world.resources.market.as_mut().unwrap().stocks.iter_mut()
        .find(|stock|stock.nation==me&&stock.commodity==Commodity::RareEarths).unwrap().quantity=1000.0;
    assert_views(&game,"building");
    spheres_sim::construction_capacity::set_assignment(&mut game.world,me,id,Some(0.0)).unwrap();
    assert_views(&game,"paused");
    spheres_sim::construction_capacity::set_assignment(&mut game.world,me,id,None).unwrap();
    assert_views(&game,"building");
    assert_eq!(game.world.production.projects[0].progress_days,0.0);
}
