// Pure research, comparison and modernization read models. This file is included
// by equipment_view; all figures are compiled by the simulation, never the UI.
fn component_branch(slot: &str) -> &'static str {
    match slot {
        "mobility" | "transmission" | "air_engine" | "air_fuel" => "engines",
        "tracks" | "wheels" | "suspension" | "troop_compartment" | "air_wing" => "chassis",
        "turret" | "armament" | "ammunition" | "artillery_loader" | "air_hardpoints" | "air_payload" => "weapons",
        "protection" | "active_protection" | "air_countermeasures" => "armor",
        "communications" => "communications",
        _ => "optics",
    }
}

fn equipment_presets() -> Vec<Value> {
    eq::PLATFORMS.iter().map(|p| {
        let id = match p.id { "tank_standard"=>"balanced", "tank_heavy"=>"protected", "tank_light"=>"light", "tank_destroyer"=>"destroyer", other=>other };
        let s=eq::default_spec(p.id);
        json!({"id":id,"name":p.name,"description":p.detail,"family":eq::platform_role(p.id),"platform":s.platform,"components":s.components,"default":p.id=="tank_standard"})
    }).collect()
}

fn comparison_options(presets:&[Value],designs:&[Value])->Vec<Value> {
    presets.iter().map(|p|json!({"id":format!("preset:{}",p["id"].as_str().unwrap_or("")),"name":format!("{} · starting configuration",p["name"].as_str().unwrap_or("Vehicle")),"platform":p["platform"],"kind":"preset"}))
        .chain(designs.iter().map(|d|json!({"id":d["id"],"name":format!("{} · {}",d["name"].as_str().unwrap_or("Model"),d["status"].as_str().unwrap_or("saved")),"platform":d["spec"]["platform"],"kind":"saved"}))).collect()
}

fn prerequisite_row(n:&Nation,id:&str,equipment:bool)->Value {
    if equipment {
        json!({"id":id,"name":eq::research(id).map(|r|r.name).unwrap_or(id),"known":n.equipment.as_ref().is_some_and(|s|s.learned.contains(id)),"equipment":true,"domain":"Aerospace"})
    } else {
        let t=spheres_sim::tech::registry().iter().find(|t|t.id==id);
        json!({"id":id,"name":t.map(|t|t.name).unwrap_or(id),"known":n.tech.knows(id),"equipment":false,"domain":t.map(|t|format!("{:?}",t.domain)).unwrap_or("Aerospace".into())})
    }
}

// The legacy detailed-tank predicate deliberately excludes ground mission
// components. Research should show every part accepted by a released platform.
fn component_in_designer(c:&eq::ComponentDef)->bool {
    eq::PLATFORMS.iter().any(|p|eq::component_compatible(p.id,c))
}

fn research_board(w:&WorldState,me:NationId)->Vec<Value> {
    let n=w.nation(me);let state=n.equipment.as_ref();let mut rows=vec![];
    for (branch,name) in [("chassis","Established vehicle platforms"),("engines","Established powertrains"),("weapons","Established weapon systems"),("armor","Established protection"),("optics","Established observation"),("communications","Established communications")] {
        let parts:Vec<_>=eq::all_components().filter(|c|component_in_designer(c)&&c.research.is_none()&&c.technology.is_none()&&component_branch(c.slot)==branch).map(|c|json!({"id":c.id,"name":c.name,"slot":c.slot})).collect();
        rows.push(json!({"id":format!("foundation:{branch}"),"name":name,"branch":branch,"state":"known","status":"Known foundation","detail":"Available starting systems. These are existing design choices, not a new research reward.","prerequisites":[],"unlock_components":parts,"unlocks":parts.iter().filter_map(|p|p["name"].as_str()).collect::<Vec<_>>(),"actions":[]}));
    }
    for r in eq::RESEARCH {
        let known=state.is_some_and(|s|s.learned.contains(r.id));let active=state.is_some_and(|s|s.active_research.as_deref()==Some(r.id));
        let bank=if active{state.unwrap().research_progress}else{0.0};
        let mut prerequisites=vec![prerequisite_row(n,r.prerequisite,false)];
        prerequisites.extend(eq::research_prerequisites(r.id).iter().map(|id|prerequisite_row(n,id,true)));
        let available=eq::research_available(n,r.id).is_ok();
        let mut actions=vec![];
        if !known&&!active {actions.push(checked(w,me,"Review research focus",json!({"kind":"equipment_research","component":r.id})));}
        for p in prerequisites.iter().filter(|p|p["known"]==false&&p["equipment"]==false) {
            actions.push(nav("Open prerequisite research",json!({"action":"research","id":p["id"],"domain":p["domain"]})));
        }
        let parts:Vec<_>=eq::all_components().filter(|c|component_in_designer(c)&&c.research==Some(r.id)).map(|c|json!({"id":c.id,"name":c.name,"slot":c.slot})).collect();
        let status=if known{"known"}else if active{"researching"}else if available{"available"}else{"locked"};
        rows.push(json!({"id":r.id,"name":r.name,"branch":eq::research_branch(r.id),"state":status,"status":if known{"Known"}else if active{"Researching"}else if available{"Available integration"}else{"Prerequisites needed"},
            "detail":r.detail,"progress":bank/r.points,"prerequisites":prerequisites,"unlock_components":parts,"unlocks":parts.iter().filter_map(|p|p["name"].as_str()).collect::<Vec<_>>(),
            "metrics":[metric("Research effort",format!("{:.2} / {} points",bank,r.points)),metric("Earliest completion",r.earliest_year),metric("Focus change","6 political capital; half the previous Aerospace progress is retained")],"actions":actions}));
    }
    let mut seen=std::collections::BTreeSet::new();
    for c in eq::all_components().filter(|c|c.technology.is_some()&&component_in_designer(c)) {
        let id=c.technology.unwrap();if !seen.insert(id){continue;}
        let t=spheres_sim::tech::registry().iter().find(|t|t.id==id);let known=n.tech.knows(id);
        let parts:Vec<_>=eq::all_components().filter(|p|p.technology==Some(id)&&component_in_designer(p)).map(|p|json!({"id":p.id,"name":p.name,"slot":p.slot})).collect();
        rows.push(json!({"id":id,"name":t.map(|t|t.name).unwrap_or(c.name),"branch":component_branch(c.slot),"state":if known{"known"}else{"locked"},"status":if known{"Known"}else{"Technology research"},"detail":c.detail,
            "prerequisites":[],"unlock_components":parts,"unlocks":parts.iter().filter_map(|p|p["name"].as_str()).collect::<Vec<_>>(),"actions":[nav("Open technology research",json!({"action":"research","id":id,"domain":"Aerospace"}))]}));
    }
    rows
}

fn profile_rows(p:&eq::CompiledProfile)->Vec<(&'static str,&'static str,f64,&'static str,&'static str)> {
    let mut rows=vec![
        ("land_factor","Land contribution",p.land_factor,"×","higher"),("firepower","Firepower",p.land,"rating","higher"),
        ("protection","Protection",p.protection,"rating","higher"),("mobility","Mobility",p.mobility,"rating","higher"),("observation","Observation",p.recon,"rating","higher"),
        ("fabrication","Fabrication per vehicle",p.fabrication_cost_bn,"bn","lower"),("development","Development programme",p.development_cost_bn,"bn","lower"),
        ("tooling","Tooling per batch",p.tooling_cost_bn,"bn","lower"),("maintenance","Maintenance per vehicle / day",p.maintenance_bn_day,"bn","lower"),
        ("development_days","Minimum development",p.development_days as f64,"days","lower"),("production_days","Minimum fabrication",p.production_days as f64,"days","lower"),
        ("load","Installation load",p.installation_used as f64,"points","neutral"),
    ];
    if let Some(g)=&p.ground_roles {rows.extend([
        ("fire_support","Fire support",g.fire_support,"rating","higher"),("protected_mobility","Protected mobility",g.protected_mobility,"rating","higher"),
        ("reconnaissance","Reconnaissance support",g.reconnaissance,"rating","higher"),("air_defense","Air defense",g.air_defense,"rating","higher")]);}
    if let Some(a)=&p.aviation {
        rows.retain(|r|!matches!(r.0,"land_factor"|"firepower"|"protection"|"mobility"|"observation"));
        rows.extend([("air_strike","Supported strike effectiveness",a.strike_factor,"×","higher"),("sorties","Supported sorties / aircraft / month",a.sorties_per_aircraft_month,"sorties","higher"),("stores_per_sortie","Mission stores per sortie",a.stores_per_sortie,"stores","neutral")]);
    }
    rows
}

fn design_comparison(w:&WorldState,me:NationId,input:&Value,target:&eq::DesignSpec,new:Option<&eq::CompiledProfile>)->Value {
    let Some(new)=new else{return json!({"available":false,"detail":"Complete the required specifications to compare this design."});};
    let explicit=input.get("comparison_id").and_then(Value::as_str).filter(|v|!v.is_empty());
    let id=explicit.or_else(||input.get("source_revision").and_then(Value::as_str));
    let n=w.nation(me);let presets=equipment_presets();
    let baseline=if let Some(id)=id {
        if let Some(preset)=id.strip_prefix("preset:") {
            presets.iter().find(|p|p["id"]==preset).and_then(|p|spec(p).ok().map(|s|(id.to_string(),p["name"].as_str().unwrap_or("Starting configuration").to_string(),s,None)))
        } else if let Some(draft_id)=id.strip_prefix("draft:") {
            n.equipment.as_ref().and_then(|s|s.drafts.get(draft_id)).map(|d|(id.to_string(),d.name.clone(),d.spec.clone(),None))
        } else {n.equipment.as_ref().and_then(|s|s.revisions.get(id)).map(|r|(id.to_string(),r.name.clone(),r.spec.clone(),Some(r.profile.clone())))}
    } else {presets.iter().find(|p|p["platform"]==target.platform).and_then(|p|spec(p).ok().map(|s|(format!("preset:{}",p["id"].as_str().unwrap()),p["name"].as_str().unwrap().to_string(),s,None)))};
    let Some((id,name,old_spec,frozen))=baseline else{return json!({"available":false,"detail":"The selected comparison model is no longer available. Choose another baseline."});};
    let Some(old)=frozen.or_else(||eq::design_preview(w,me,&old_spec).profile) else{return json!({"available":false,"detail":"This unfinished baseline cannot yet be priced."});};
    let old_rows=profile_rows(&old);let new_rows=profile_rows(new);
    let mut keys:Vec<_>=old_rows.iter().map(|r|r.0).collect();for r in &new_rows {if !keys.contains(&r.0){keys.push(r.0);}}
    let rows:Vec<_>=keys.into_iter().map(|key|{
        let before=old_rows.iter().find(|r|r.0==key);let after=new_rows.iter().find(|r|r.0==key);let meta=after.or(before).unwrap();
        let b=before.map(|r|r.2).unwrap_or(0.0);let a=after.map(|r|r.2).unwrap_or(0.0);
        json!({"key":key,"label":meta.1,"before":b,"after":a,"delta":a-b,"unit":meta.3,"better_when":meta.4})
    }).collect();
    let mut slots:std::collections::BTreeSet<_>=old_spec.components.keys().cloned().collect();slots.extend(target.components.keys().cloned());
    let label=|id:Option<&String>|id.map(|s|eq::component(s).map(|c|c.name.to_string()).unwrap_or_else(||s.clone())).unwrap_or("Not installed".into());
    let changes:Vec<_>=slots.into_iter().filter(|s|old_spec.components.get(s)!=target.components.get(s)).map(|s|json!({"slot":s,"label":eq::slot_name(&s),"before":label(old_spec.components.get(&s)),"after":label(target.components.get(&s))})).collect();
    let same=eq::platform_role(&old_spec.platform)==eq::platform_role(&target.platform);
    json!({"available":true,"id":id,"name":name,"same_role":same,"rows":rows,"changes":changes,"detail":if same{"Per-vehicle design ratings and programme costs. These are prospective capabilities; your forces change only after funded development, delivery and support."}else{"Different vehicle roles: compare mission ratings and cost together. A higher rating in one role does not make a vehicle a replacement for another role."}})
}

fn modernization_board(w:&WorldState,me:NationId)->Vec<Value> {
    let n=w.nation(me);let mut rows=vec![];let Some(state)=n.equipment.as_ref() else {
        let s=eq::default_spec("tank_standard");
        return vec![json!({"id":"start-programme","name":"Establish your first vehicle programme","status":"Starting point","reason":"Your inherited arsenal is in service, but there is no custom vehicle programme yet.","detail":"Start from an affordable general-purpose configuration, compare its bill, then decide whether to fund development.","tradeoff":"A new programme adds development, tooling and ongoing maintenance commitments.","spec":s,"actions":[]})];
    };
    let held:Vec<_>=n.arsenal.held.iter().filter(|h|h.units>h.refit_reserved as f64&&h.design_id.is_some()).collect();
    if !held.is_empty()&&state.maintenance_fraction<0.95 {
        rows.push(json!({"id":"restore-support","name":"Restore fleet maintenance coverage","status":"Support priority","reason":format!("Current custom-fleet maintenance coverage is {:.0}%.",state.maintenance_fraction*100.0),"detail":"Improve support for the vehicles you already own before adding a larger maintenance burden.","tradeoff":"Moving Defense funds into maintenance leaves less for the other departments.","actions":[nav("Review maintenance funding",json!({"action":"budget","ministry":"defense","department":2}))]}));
    }
    for h in held.iter().take(8) {
        let id=h.design_id.as_ref().unwrap();let Some(source)=state.revisions.get(id) else{continue;};
        let base=eq::editable_spec(&source.spec);
        let primary=|p:&eq::CompiledProfile|->f64 {if let Some(a)=&p.aviation{return a.strike_factor;}match (base.platform.as_str(),&p.ground_roles) {
            ("ground_artillery",Some(g))=>g.fire_support,("ground_air_defense",Some(g))=>g.air_defense,("ground_recon",Some(g))=>g.reconnaissance,
            ("ground_apc",Some(g))=>g.protected_mobility,("ground_ifv",Some(g))=>(g.fire_support+g.protected_mobility)*0.5,_=>p.land_factor,
        }};
        let mut best:Option<(f64,eq::DesignSpec,eq::CompiledProfile,String)>=None;
        for slot in eq::platform_slots(&base.platform) {
            for c in eq::all_components().filter(|c|c.slot==*slot&&eq::component_compatible(&base.platform,c)&&eq::component_known(n,c)) {
                if base.components.get(*slot).is_some_and(|v|v==c.id){continue;}
                let mut candidate=base.clone();candidate.components.insert((*slot).into(),c.id.into());
                let preview=eq::design_preview(w,me,&candidate);if !preview.valid{continue;}let Some(p)=preview.profile else{continue;};
                let gain=primary(&p)-primary(&source.profile);if gain<0.025{continue;}
                let cost_change=(p.fabrication_cost_bn/source.profile.fabrication_cost_bn.max(1e-9)-1.0).max(0.0);
                let support_change=(p.maintenance_bn_day/source.profile.maintenance_bn_day.max(1e-12)-1.0).max(0.0);
                let score=gain/(1.0+cost_change+support_change);
                if best.as_ref().is_none_or(|b|score>b.0) {best=Some((score,candidate,p,c.name.into()));}
            }
        }
        if let Some((_,target,p,part))=best {
            let refit=target.platform==source.spec.platform&&target.components.get("armament")==source.spec.components.get("armament");
            rows.push(json!({"id":format!("modernize:{id}"),"name":format!("{} · {}",source.name,part),"status":if refit{"Upgrade candidate"}else{"Replacement candidate"},
                "reason":format!("{:.0} available vehicles use {}. This researched component improves the design's supported role rating.",h.units-h.refit_reserved as f64,source.name),
                "detail":if refit{"Compare this proposed revision, then develop it. A compatible refit can be reviewed after certification."}else{"The main weapon changes, so this candidate requires new manufacture. Your existing vehicles remain in service."},
                "tradeoff":format!("Fabrication changes by ${:+.0}k per vehicle; maintenance changes by ${:+.0} per vehicle per day. Development and tooling still apply.",(p.fabrication_cost_bn-source.profile.fabrication_cost_bn)*1e6,(p.maintenance_bn_day-source.profile.maintenance_bn_day)*1e9),
                "metrics":[metric("Source role rating",primary(&source.profile)),metric("Candidate role rating",primary(&p))],"spec":target,"source_revision":id,"actions":[]}));
        }
    }
    if rows.is_empty() {
        rows.push(json!({"id":"research-next","name":"Review the next research opportunity","status":"No immediate upgrade","reason":"No compatible, known single-component change currently offers a meaningful improvement for the delivered custom fleet.","detail":"Keep the current designs, or explore the research branches before committing to another programme.","tradeoff":"Research takes existing Aerospace effort and does not upgrade equipment already in service.","actions":[]}));
    }
    rows.truncate(5);rows
}

#[cfg(test)]
mod planning_tests {
    use super::*;
    fn game()->super::super::Game {
        let mut g=super::super::Game::new(1990,Some(NationId::USA));
        super::super::play_rules(&mut g);g
    }
    fn insert_fleet(g:&mut super::super::Game) {
        let spec=eq::default_spec("tank_standard");let profile=eq::design_preview(&g.world,NationId::USA,&spec).profile.unwrap();
        let n=g.world.nation_mut(NationId::USA);let s=n.equipment.get_or_insert_with(Default::default);
        s.maintenance_fraction=1.0;
        s.revisions.insert("fielded".into(),eq::DesignRevision{id:"fielded".into(),name:"First generation".into(),specification_key:eq::specification_key(&spec),spec,profile,created_day:0,certified_day:Some(0)});
        spheres_sim::arsenal::deliver_design(n,"fielded",20,12.0).unwrap();
    }
    #[test]
    fn comparison_reports_real_deltas_and_is_pure() {
        let g=game();let mut target=eq::default_spec("tank_standard");target.components.insert("sensors".into(),"optics_night".into());
        let before=spheres_sim::save(&g.world);
        let q=preview(&g.world,NationId::USA,&g.session_id,&json!({"name":"Night model","platform":target.platform,"components":target.components,"comparison_id":"preset:balanced"})).unwrap();
        assert_eq!(q["comparison"]["available"],true);
        let rows=q["comparison"]["rows"].as_array().unwrap();
        for r in rows {let b=r["before"].as_f64().unwrap();let a=r["after"].as_f64().unwrap();assert!((r["delta"].as_f64().unwrap()-(a-b)).abs()<1e-12);}
        assert!(rows.iter().find(|r|r["key"]=="observation").unwrap()["delta"].as_f64().unwrap()>0.0);
        assert!(rows.iter().find(|r|r["key"]=="fabrication").unwrap()["delta"].as_f64().unwrap()>0.0);
        assert_eq!(q["comparison"]["changes"].as_array().unwrap().len(),1);
        assert_eq!(spheres_sim::save(&g.world),before);
    }
    #[test]
    fn comparisons_use_frozen_revision_prices_and_explain_role_changes() {
        let mut g=game();insert_fleet(&mut g);
        g.world.nation_mut(NationId::USA).equipment.as_mut().unwrap().revisions.get_mut("fielded").unwrap().profile.fabrication_cost_bn=0.123;
        let target=eq::default_spec("ground_air_defense");let p=eq::design_preview(&g.world,NationId::USA,&target).profile.unwrap();
        let q=design_comparison(&g.world,NationId::USA,&json!({"comparison_id":"fielded"}),&target,Some(&p));
        assert_eq!(q["same_role"],false);
        let rows=q["rows"].as_array().unwrap();assert_eq!(rows.iter().find(|r|r["key"]=="fabrication").unwrap()["before"],0.123);
        assert_eq!(rows.iter().find(|r|r["key"]=="air_defense").unwrap()["before"],0.0);
        assert!(rows.iter().find(|r|r["key"]=="air_defense").unwrap()["after"].as_f64().unwrap()>0.0);
        let missing=design_comparison(&g.world,NationId::USA,&json!({"comparison_id":"not-a-model"}),&target,Some(&p));assert_eq!(missing["available"],false);
    }
    #[test]
    fn research_tree_reflects_dependencies_and_never_changes_knowledge() {
        let mut g=game();let t=spheres_sim::tech::index_of("core_cmos_submicron").unwrap();
        let n=g.world.nation_mut(NationId::USA);if !n.tech.knows_index(t){n.tech.known.push(t);n.tech.known.sort();}
        let before=spheres_sim::save(&g.world);let rows=research_board(&g.world,NationId::USA);
        let advanced=rows.iter().find(|r|r["id"]=="ground_battlefield_network").unwrap();assert_eq!(advanced["state"],"locked");
        assert!(advanced["prerequisites"].as_array().unwrap().iter().any(|p|p["id"]=="ground_secure_radios"&&p["known"]==false));
        for r in eq::RESEARCH {assert!(rows.iter().any(|v|v["id"]==r.id));}
        assert_eq!(spheres_sim::save(&g.world),before);
    }
    #[test]
    fn ground_research_lists_its_unlocks_and_established_starter_components() {
        let g=game();let rows=research_board(&g.world,NationId::USA);
        for research in eq::RESEARCH.iter().filter(|r|r.id.starts_with("ground_")) {
            let row=rows.iter().find(|r|r["id"]==research.id).unwrap();
            let actual:std::collections::BTreeSet<_>=row["unlock_components"].as_array().unwrap().iter().map(|c|c["id"].as_str().unwrap()).collect();
            let expected:std::collections::BTreeSet<_>=eq::GROUND_COMPONENTS.iter().filter(|c|c.research==Some(research.id)).map(|c|c.id).collect();
            assert!(!expected.is_empty(),"{} must unlock an installed component",research.id);
            assert_eq!(actual,expected,"{} must explain every part it unlocks",research.id);
        }
        let foundations:std::collections::BTreeSet<_>=rows.iter().filter(|r|r["id"].as_str().unwrap().starts_with("foundation:"))
            .flat_map(|r|r["unlock_components"].as_array().unwrap()).map(|c|c["id"].as_str().unwrap()).collect();
        for platform in eq::PLATFORMS.iter().filter(|p|eq::is_ground_platform(p.id)) {
            for id in eq::default_spec(platform.id).components.values() {
                let c=eq::component(id).unwrap();assert!(c.research.is_none()&&c.technology.is_none());
                assert!(foundations.contains(id.as_str()),"{} starter part {id} must be represented among known systems",platform.id);
            }
        }
    }
    #[test]
    fn modernization_proposes_valid_known_upgrades_without_touching_the_fleet() {
        let mut g=game();insert_fleet(&mut g);let before=spheres_sim::save(&g.world);
        let rows=modernization_board(&g.world,NationId::USA);let suggestion=rows.iter().find(|r|r["source_revision"]=="fielded").expect("baseline has known upgrades");
        let target: eq::DesignSpec=serde_json::from_value(suggestion["spec"].clone()).unwrap();assert!(eq::design_preview(&g.world,NationId::USA,&target).valid);
        assert!(target.components.values().all(|id|eq::component_known(g.world.nation(NationId::USA),eq::component(id).unwrap())));
        assert!(suggestion["tradeoff"].as_str().unwrap().contains("maintenance"));assert_eq!(spheres_sim::save(&g.world),before);
        g.world.nation_mut(NationId::USA).equipment.as_mut().unwrap().maintenance_fraction=0.4;
        let rows=modernization_board(&g.world,NationId::USA);assert_eq!(rows[0]["id"],"restore-support");assert_eq!(rows[0]["actions"][0]["navigate"]["department"],2);
    }
}
