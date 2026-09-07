// Tactical aircraft share the equipment lifecycle and physical ammunition book.
fn aviation_board(w:&WorldState,me:NationId)->Value {
    let n=w.nation(me);
    let Some(state)=n.equipment.as_ref() else {return Value::Null;};
    let models:Vec<_>=state.revisions.values().filter(|r|r.profile.aviation.is_some()).collect();
    if models.is_empty(){return Value::Null;}
    let mut held=0.0;let mut available=0u64;let mut supported_total=0.0;let mut rows=vec![];
    let usage=eq::ammunition_overview(w,me);
    for r in &models {
        let a=r.profile.aviation.as_ref().unwrap();
        let lots:Vec<_>=n.arsenal.held.iter().filter(|h|h.design_id.as_deref()==Some(r.id.as_str())).collect();
        let count:u64=lots.iter().map(|h|spheres_sim::arsenal::available_design_units(h) as u64).sum();
        let supported=lots.iter().map(|h|spheres_sim::arsenal::combat_value(n,h)/r.profile.reference_weight_bn).sum::<f64>()+0.0;
        held+=lots.iter().map(|h|h.units).sum::<f64>();available+=count;supported_total+=supported;
        let family=eq::ammo_def(&a.store_family).map_or(a.store_family.as_str(),|d|d.name);
        rows.push(json!({"label":r.name,"value":format!("{count} available · {supported:.2} supported aircraft equivalents"),"detail":format!("{} · {:.1} sorties / aircraft / month before deployment limits · {} stores per sortie · {:.3}× supported strike effectiveness. Stores are manufactured separately.",family,a.sorties_per_aircraft_month,a.stores_per_sortie,a.strike_factor)}));
    }
    let required:f64=usage.families.iter().filter(|f|f.family.starts_with("air_bomb_")).map(|f|f.required).sum();
    let supplied:f64=usage.families.iter().filter(|f|f.family.starts_with("air_bomb_")).map(|f|f.used).sum();
    let force=spheres_sim::operations::view(w,me);
    let raids=force.deployments.iter().filter(|d|d.rung==6).count();
    let mut accessible=0;
    for d in force.deployments.iter().filter(|d|d.rung==6) {
        let Some(c)=w.conflict(d.conflict)else{continue;};
        let access=spheres_sim::theatre::has_access(w,me,c.theatre);if access{accessible+=1;}
        rows.push(json!({"label":format!("Air raid · {}",c.theatre.name()),"value":if access{"Theatre access available"}else{"No basing access · aircraft cannot launch"},"detail":format!("{:.1}% of national force allocated to this operation. Mission stores are apportioned with other air raids before use.",if n.mil_strength>0.0{100.0*d.deployed/n.mil_strength}else{0.0})}));
    }
    let mut warnings=vec![];
    if held>0.0&&available==0 {warnings.push("Your aircraft are withdrawn for refit. They launch no sorties until they return to available service.".to_string());}
    if available>0&&supported_total<=1e-9{warnings.push("Maintenance currently supports no aircraft. Fund fleet upkeep before launching sorties.".into());}
    if raids>accessible{warnings.push("One or more air raids lack theatre basing access. Resolve access through diplomacy before these aircraft can launch there.".into());}
    if required>supplied+1e-9 {warnings.push("Compatible aircraft stores are below current mission demand. Manufacture the exact bomb family in Ammunition; other ammunition cannot substitute.".into());}
    if state.maintenance_plan.is_none(){warnings.push("Set an actual maintenance plan before manufacturing mission stores. Aircraft upkeep and store fabrication share Defense maintenance funding.".into());}
    json!({"title":"Tactical aviation readiness","status":if required>supplied+1e-9{"Mission stores needed"}else if available==0{"Awaiting available aircraft"}else if supported_total<=1e-9{"Aircraft maintenance needed"}else if raids==0{"No air raids assigned"}else if accessible==0{"Theatre access needed"}else{"Review current air operations"},
        "detail":"Delivered tactical aircraft serve rung-6 air raids with theatre basing access and a share of the national deployment. Their supported sortie rate and selected payload determine physical store use. These aircraft provide no ground fire, air-superiority mission or transport lift. Ground ammunition activation remains a separate choice.",
        "metrics":[metric("Aircraft models",models.len()),metric("Delivered aircraft",held),metric("Available aircraft",available),metric("Air raids with theatre access",format!("{accessible} / {raids}")),metric("Mission stores required this tick",ammo_quantity(required)),metric("Mission stores available for this tick",ammo_quantity(supplied))],
        "roles_title":"Aircraft and mission loadouts","roles":rows,"warnings":warnings,
        "actions":[nav("Prepare aircraft mission stores",json!({"action":"equipment","tab":"ammunition"})),nav("Follow aircraft production",json!({"action":"equipment","tab":"production"})),nav("Review Defense maintenance funding",json!({"action":"budget","ministry":"defense","department":2}))]})
}

#[cfg(test)]
mod aviation_view_tests {
    use super::*;
    const ID:NationId=NationId::France;
    fn fixture()->super::super::Game {
        let mut g=ammunition_view_tests::fixture();
        let n=g.world.nation_mut(ID);n.arsenal.held.clear();
        let state=n.equipment.as_mut().unwrap();state.revisions.clear();
        for id in ["air_propulsion_integration","air_mission_systems","air_guided_strike"]{state.learned.insert(id.into());}
        let day=spheres_sim::clock::absolute_day(&g.world);
        for (id,name,platform,guided) in [("air-light","Lark light attack","air_light_attack",false),("air-strike","Kestrel tactical strike","air_tactical_strike",false),("air-guided","Kestrel guided strike","air_tactical_strike",true)] {
            let mut spec=eq::default_spec(platform);
            if guided {spec.components.insert("air_payload".into(),"air_payload_guided".into());spec.components.insert("air_avionics".into(),"air_avionics_digital".into());}
            let p=eq::design_preview(&g.world,ID,&spec);assert!(p.valid,"{:?}",p.blockers);
            let n=g.world.nation_mut(ID);
            n.equipment.as_mut().unwrap().revisions.insert(id.into(),eq::DesignRevision{id:id.into(),name:name.into(),specification_key:eq::specification_key(&spec),spec,profile:p.profile.unwrap(),created_day:day,certified_day:Some(day)});
            if !guided{spheres_sim::arsenal::deliver_design(n,id,2,0.0).unwrap();}
        }
        g
    }
    #[test]
    fn aviation_catalogue_and_previews_show_real_role_costs_and_are_pure() {
        let g=ammunition_view_tests::fixture();let before=spheres_sim::save(&g.world);let board=view(&g.world,ID,&g.session_id);
        assert!(board["aviation"].is_null());
        let aircraft:Vec<_>=board["platforms"].as_array().unwrap().iter().filter(|p|p["family"]=="aviation").collect();
        assert_eq!(aircraft.len(),2);assert!(aircraft.iter().all(|p|p["slots"].as_array().unwrap().len()==8));
        for p in aircraft {
            let mut draft=p["default_spec"].clone();draft["name"]=json!("Air preview");
            let q=preview(&g.world,ID,&g.session_id,&draft).unwrap();assert_eq!(q["valid"],true,"{q}");
            assert!(q["metrics"].as_array().unwrap().iter().any(|m|m["label"]=="Supported strike effectiveness"));
            assert!(!q["metrics"].as_array().unwrap().iter().any(|m|m["label"]=="Land contribution"));
            assert!(q["detail"].as_str().unwrap().contains("mission stores"));
            let command=&q["actions"][1]["command"];
            let paid=preview(&g.world,ID,&g.session_id,&json!({"command":command})).unwrap();
            assert_eq!(paid["valid"],true,"{paid}");assert_eq!(paid["costs"][0]["amount_bn"],eq::design_preview(&g.world,ID,&eq::default_spec(p["id"].as_str().unwrap())).profile.unwrap().development_cost_bn);
        }
        assert_eq!(spheres_sim::save(&g.world),before);
    }
    #[test]
    fn aviation_mismatched_guidance_is_blocked_and_refit_comparison_uses_strike() {
        let g=fixture();let mut draft=serde_json::to_value(eq::default_spec("air_tactical_strike")).unwrap();
        draft["components"]["air_payload"]=json!("air_payload_guided");
        let q=preview(&g.world,ID,&g.session_id,&draft).unwrap();assert_eq!(q["valid"],false);assert_eq!(q["actions"].as_array().unwrap().len(),1);
        draft["components"]["air_avionics"]=json!("air_avionics_digital");draft["source_revision"]=json!("air-strike");
        let q=preview(&g.world,ID,&g.session_id,&draft).unwrap();assert_eq!(q["valid"],true,"{q}");
        assert!(q["metrics"].as_array().unwrap().iter().any(|m|m["label"]=="Change from original supported strike effectiveness"));
        let board=view(&g.world,ID,&g.session_id);let lot=board["lots"].as_array().unwrap().iter().find(|r|r["id"]=="air-strike").unwrap();
        assert_eq!(lot["actions"][0]["command"]["target"],"air-guided");
        assert_eq!(preview(&g.world,ID,&g.session_id,&json!({"command":lot["actions"][0]["command"]})).unwrap()["valid"],true);
    }
    #[test]
    fn aviation_store_reviews_keep_ground_activation_separate_and_archive_exactly() {
        let mut g=fixture();let before=spheres_sim::save(&g.world);
        let board=view(&g.world,ID,&g.session_id);assert!(!board["aviation"].is_null());
        assert!(!board["aviation"].to_string().contains("-0.00 supported"));
        assert_eq!(board["aviation"]["status"],"No air raids assigned");
        assert_eq!(board["ammunition"]["overview"]["status"],"Aircraft stores required · ground activation separate");
        let family=board["ammunition"]["families"].as_array().unwrap().iter().find(|f|f["id"]=="air_bomb_unguided").unwrap();
        let guided=board["ammunition"]["families"].as_array().unwrap().iter().find(|f|f["id"]=="air_bomb_guided").unwrap();
        assert!(guided["metrics"].as_array().unwrap().iter().any(|m|m["label"]=="One-month planning reserve"&&m["value"]=="0 stores"));
        let reference=2.0*12.0*2.0+2.0*10.0*4.0;
        assert!(family["metrics"].as_array().unwrap().iter().any(|m|m["label"]=="One-month planning reserve"&&m["value"]==format!("{} stores",ammo_quantity(reference))));
        let command=&family["actions"][0]["command"];
        let q=preview(&g.world,ID,&g.session_id,&json!({"command":command})).unwrap();assert_eq!(q["valid"],true,"{q}");
        assert!(q["metrics"].as_array().unwrap().iter().any(|m|m["label"]=="Mission stores to manufacture"));
        let reserve=preview(&g.world,ID,&g.session_id,&json!({"command":family["actions"][1]["command"]})).unwrap();assert_eq!(reserve["valid"],true,"{reserve}");
        assert_eq!(spheres_sim::save(&g.world),before);
        if let Some(path)=std::env::var_os("SPHERES_AVIATION_QA_SAVE"){std::fs::write(path,&before).unwrap();}
        let parsed=super::super::parse_command(&g.world,command,ID).unwrap();spheres_sim::apply_command(&mut g.world,&parsed).unwrap();
        let ammo=g.world.nation(ID).equipment.as_ref().unwrap().ammunition.as_ref().unwrap();
        assert_eq!(ammo.active_from_day,None);assert!(ammo.stocks.is_empty());assert_eq!(ammo.orders[0].spent_bn,0.0);
        let archive=super::super::storage::encode(&g).unwrap();let restored=super::super::storage::decode(&archive).unwrap();
        assert_eq!(spheres_sim::save(&g.world),spheres_sim::save(&restored.world));
    }
}
