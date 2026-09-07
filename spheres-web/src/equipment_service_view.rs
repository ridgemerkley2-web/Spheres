// Inventory and service read models. Physical retirement is owned by the sim.
fn service_money(value:f64)->String {
    let value=if value==0.0 {0.0}else{value};
    let amount=if value.abs()>=1.0 {format!("${value:.3}bn")}else if value.abs()>=0.001 {format!("${:.3}m",value*1000.0)}else if value.abs()>=0.000001 {format!("${:.3}k",value*1_000_000.0)}else {format!("${:.2}",value*1_000_000_000.0)};
    format!("{amount} / day")
}
fn service_role_rows(n:&Nation,after:Option<&Nation>)->Vec<Value> {
    let before=spheres_sim::operations::capabilities(n);
    let next=after.map(spheres_sim::operations::capabilities);
    let rows=[
        ("Land composition factor",before.land,next.map(|c|c.land),false),
        ("Strike composition factor",before.strike,next.map(|c|c.strike),false),
        ("Fire support bonus",before.ground_roles.fire_support,next.map(|c|c.ground_roles.fire_support),true),
        ("Protected mobility bonus",before.ground_roles.protected_mobility,next.map(|c|c.ground_roles.protected_mobility),true),
        ("Reconnaissance bonus",before.ground_roles.reconnaissance,next.map(|c|c.ground_roles.reconnaissance),true),
        ("Air defense bonus",before.ground_roles.air_defense,next.map(|c|c.ground_roles.air_defense),true),
    ];
    rows.into_iter().map(|(label,current,proposed,percent)|{
        let show=|v:f64|if percent{format!("{:.3}%",v*100.0)}else{format!("{v:.3}×")};
        json!({"label":label,"value":proposed.map(|v|format!("{} → {}",show(current),show(v))).unwrap_or_else(||show(current))})
    }).collect()
}
fn equipment_service_board(w:&WorldState,me:NationId)->Value {
    let n=w.nation(me);let s=n.equipment.as_ref();
    let held:Vec<_>=n.arsenal.held.iter().filter(|h|h.design_id.as_deref().is_some_and(|id|eq::profile(n,id).is_some())&&h.units>0.0).collect();
    let delivered=held.iter().map(|h|h.units).sum::<f64>()+0.0;
    let available: u64=held.iter().map(|h|spheres_sim::arsenal::available_design_units(h) as u64).sum();
    let reserved:u64=held.iter().map(|h|h.refit_reserved as u64).sum();
    let deliveries:Vec<_>=n.arsenal.orders.iter().filter(|o|o.design_id.is_some()).collect();
    let incoming=deliveries.iter().map(|o|o.units).sum::<f64>()+0.0;
    let fabrication:u64=s.map_or(0,|s|s.projects.iter().filter(|p|p.kind==eq::ProjectKind::Production&&!matches!(p.status,eq::ProjectStatus::Complete|eq::ProjectStatus::Cancelled)).map(|p|p.quantity.saturating_sub(p.completed_units) as u64).sum());
    let requirement=eq::fleet_maintenance_requirement(n);
    let recorded=s.filter(|s|s.last_tick_day.is_some()||s.maintenance_plan.as_ref().is_some_and(|p|p.receipt.is_some()));
    let mut warnings=vec![];
    if delivered>0.0&&recorded.is_none(){warnings.push("A support settlement has not yet been recorded for this custom fleet. Review the maintenance allocation before advancing.".to_string());}
    if delivered>0.0&&recorded.is_some_and(|s|s.maintenance_fraction<0.95){warnings.push("The last maintenance settlement did not fully support this fleet. Review Defense maintenance before expanding it; a higher allocation leaves less for other departments.".to_string());}
    if reserved>0 {warnings.push(format!("{reserved} delivered vehicles are withdrawn for refit. They cannot deploy or be retired until conversion completes or their refit is cancelled."));}
    let mut metrics=vec![metric("Delivered custom vehicles",delivered),metric("Available for operations",available),metric("Withdrawn for refit",reserved),metric("Completed vehicles in transit",incoming),metric("New vehicles awaiting fabrication",fabrication),metric("Current custom maintenance requirement",service_money(requirement))];
    if let Some(s)=recorded {
        metrics.push(metric("Last settled maintenance coverage",format!("{:.1}%",s.maintenance_fraction*100.0)));
        metrics.push(metric(if s.maintenance_plan.as_ref().is_some_and(|p|p.receipt.is_some()){"Last custom maintenance payment"}else{"Last maintenance earmark"},service_money(s.maintenance_allocated_today_bn)));
    }
    if let Some(days)=deliveries.iter().map(|o|o.due_days.unwrap_or_else(||spheres_sim::clock::days_for_months(w,o.due))).min(){metrics.push(metric("Next delivery",format!("{days} days remaining on its delivery counter")));}
    let mut actions=vec![nav("Review maintenance funding",json!({"action":"budget","ministry":"defense","department":2}))];
    if incoming>0.0||fabrication>0||reserved>0{actions.push(nav("Follow production and deliveries",json!({"action":"equipment","tab":"production"})));}
    actions.push(nav(if s.is_some_and(|s|!s.revisions.is_empty()){"Open equipment library"}else{"Design your first vehicle"},json!({"action":"equipment","tab":if s.is_some_and(|s|!s.revisions.is_empty()){"library"}else{"designer"}})));
    json!({"title":"Your fleet in service","status":if delivered<=0.0&&incoming>0.0{"Awaiting delivery"}else if delivered<=0.0{"No custom vehicles delivered"}else if available==0{"Fleet withdrawn for refit"}else if !warnings.is_empty(){"Review fleet support"}else{"Fleet in service"},
        "detail":"Delivered holdings include refit reservations; available vehicles exclude them. Transit and unfinished production add no fielded capability. Maintenance requirements reflect current holdings, while coverage and payments describe the last settlement. National composition includes inherited equipment and recorded support; these are operational role effects, not per-vehicle ratings or battle predictions.",
        "metrics":metrics,"roles":if delivered>0.0{service_role_rows(n,None)}else{vec![]},"warnings":warnings,"actions":actions})
}
fn retirement_action(w:&WorldState,me:NationId,revision:&str)->Value {
    let q=eq::retirement_quote(w,me,revision,1);
    let mut action=intent("Review vehicle retirement",json!({"kind":"equipment_retire","revision":revision,"quantity":1}),vec![
        json!({"key":"quantity","label":"Vehicles to retire","type":"number","value":1,"min":1,"max":q.available_units.min(eq::MAX_BATCH).max(1),"step":1})]);
    action["enabled"]=json!(q.valid);action["reason"]=json!(q.reason);
    action["detail"]=json!("Permanently remove selected delivered vehicles from the Arsenal. Review the remaining force and maintenance requirement first; there is no cash or material refund.");
    action
}
fn equipment_retirement_preview(w:&WorldState,me:NationId,session:&str,command:&Value,parsed:&Command,revision:&str,quantity:u32)->Value {
    let q=eq::retirement_quote(w,me,revision,quantity);
    let mut action=checked(w,me,"Confirm vehicle retirement",command.clone());
    let mut blockers=vec![];
    if let Some(reason)=action["reason"].as_str(){blockers.push(reason.to_string());}
    let mut proposed=w.clone();
    let valid=q.valid&&spheres_sim::apply_command(&mut proposed,parsed).is_ok()&&blockers.is_empty();
    if !valid&&blockers.is_empty(){blockers.push(q.reason.clone().unwrap_or("This retirement cannot be performed.".into()));}
    action["enabled"]=json!(valid);
    let mut metrics=vec![metric("Vehicles requested for retirement",quantity),metric("Delivered vehicles in this model",q.held_units),metric("Available for operations",q.available_units),metric("Reserved for refit",q.reserved_units)];
    if valid {metrics=vec![metric("Vehicles retired",quantity),metric("Delivered vehicles remaining",format!("{} → {}",q.held_units,q.remaining_units)),
        metric("Available for operations",format!("{} → {}",q.available_units,q.available_units.saturating_sub(quantity))),metric("Refit reservations retained",q.reserved_units),
        metric("Custom fleet maintenance requirement",format!("{} → {}",service_money(q.maintenance_bn_day_before),service_money(q.maintenance_bn_day_after))),
        metric("Maintenance requirement removed",service_money(q.maintenance_reduction_bn_day)),metric("Cash and material refund","None")];}
    let effects=json!({"title":"Fleet after retirement","status":if valid{"Review permanent withdrawal"}else{"Retirement unavailable"},
        "detail":"Retirement removes physical vehicles immediately. Survivors keep their age and condition. National composition includes inherited equipment at the current recorded support level; normal settlement recalculates support later. Small changes may round to the same displayed value.",
        "metrics":metrics,"roles":if valid{service_role_rows(w.nation(me),Some(proposed.nation(me)))}else{vec![]},
        "warnings":["Retirement cannot be undone by the game. Replacing these vehicles requires new production or delivery.",if w.nation(me).equipment.as_ref().is_some_and(|s|s.maintenance_plan.is_some()){"Lower maintenance need can reduce future invoices. Existing budget allocations and already-recorded payments stay unchanged."}else{"Reduced maintenance need is not a cash saving or a budget cut. Existing appropriations and already-recorded payments stay unchanged."},"Refit reservations, pending deliveries, production contracts and saved design revisions are retained."]});
    json!({"session_id":session,"nation":me,"valid":valid,"blockers":blockers,"metrics":[],"costs":[],"timing":[],"requirements":[q.note],"service_effects":effects,
        "actions":[action],"detail":"Review the quantity and loss of available equipment before confirming retirement. Opening this comparison changes nothing."})
}

#[cfg(test)]
mod service_view_tests {
    use super::*;
    use spheres_sim::arsenal;
    fn fixture()->super::super::Game {
        let mut g=super::super::Game::new(1990,Some(NationId::USA));super::super::play_rules(&mut g);
        spheres_sim::programs::set_construction_budget(&mut g.world,NationId::USA,0.0).unwrap();
        let today=spheres_sim::clock::absolute_day(&g.world);
        for (id,name,wide) in [("fleet-ifv","Fleet IFV",false),("fleet-ifv-upgrade","Fleet IFV upgrade",true)] {
            let mut spec=eq::default_spec("ground_ifv");if wide{spec.components.insert("tracks".into(),"tracks_wide".into());}
            let preview=eq::design_preview(&g.world,NationId::USA,&spec);assert!(preview.valid,"{:?}",preview.blockers);
            g.world.nation_mut(NationId::USA).equipment.get_or_insert_with(Default::default).revisions.insert(id.into(),eq::DesignRevision {
                id:id.into(),name:name.into(),specification_key:eq::specification_key(&spec),spec,profile:preview.profile.unwrap(),created_day:today,certified_day:Some(today),
            });
        }
        g.world.nation_mut(NationId::USA).equipment.as_mut().unwrap().finance_from_day=today;
        arsenal::deliver_design(g.world.nation_mut(NationId::USA),"fleet-ifv",6,36.0).unwrap();
        arsenal::queue_design_order(g.world.nation_mut(NationId::USA),"fleet-ifv",3,7,0.0).unwrap();
        let district=g.world.districts.iter().find(|(_,n)|**n==NationId::USA).unwrap().0.clone();
        g.world.production.provinces.push(spheres_sim::production::ProvinceCapabilities{district:district.clone(),infrastructure:0,civilian_industry:0,power_grid:0,research_centers:0,arms_plants:3});
        eq::start_refit(&mut g.world,NationId::USA,"fleet-ifv","fleet-ifv-upgrade",&district,2,0.0001).unwrap();
        g
    }
    fn command(quantity:Value)->Value {json!({"kind":"equipment_retire","revision":"fleet-ifv","quantity":quantity})}
    fn metric_value<'a>(service:&'a Value,label:&str)->&'a Value {&service["metrics"].as_array().unwrap().iter().find(|r|r["label"]==label).unwrap()["value"]}
    #[test]
    fn service_reading_separates_available_reserved_and_incoming_without_changing_world() {
        let g=fixture();let before=spheres_sim::save(&g.world);let data=view(&g.world,NationId::USA,&g.session_id);
        assert_eq!(metric_value(&data["service"],"Delivered custom vehicles"),6.0);
        assert_eq!(metric_value(&data["service"],"Available for operations"),4);
        assert_eq!(metric_value(&data["service"],"Withdrawn for refit"),2);
        assert_eq!(metric_value(&data["service"],"Completed vehicles in transit"),3.0);
        assert_eq!(metric_value(&data["service"],"Current custom maintenance requirement"),"$2.478k / day");
        assert_eq!(metric_value(&data["lots"][0],"Last settled maintenance coverage"),"Not yet settled");
        assert!(!data["service"]["roles"].as_array().unwrap().is_empty());
        assert!(data["lots"][0]["actions"].as_array().unwrap().iter().any(|a|a["command"]["kind"]=="equipment_retire"&&a["requires_preview"]==true));
        assert_eq!(before,spheres_sim::save(&g.world));
        let loaded=spheres_sim::load(&before).unwrap();assert_eq!(spheres_sim::save(&loaded),before);
        if let Some(path)=std::env::var_os("SPHERES_FLEET_QA_SAVE"){std::fs::write(path,before).unwrap();}
    }
    #[test]
    fn retirement_quote_is_pure_matches_sim_and_only_confirmed_command_removes_available_stock() {
        let mut g=fixture();let before=spheres_sim::save(&g.world);let c=command(json!(2));
        let result=preview(&g.world,NationId::USA,&g.session_id,&json!({"command":c})).unwrap();
        assert_eq!(result["valid"],true);assert_eq!(result["actions"][0]["label"],"Confirm vehicle retirement");
        let effects=&result["service_effects"];
        assert_eq!(metric_value(effects,"Delivered vehicles remaining"),"6 → 4");
        assert_eq!(metric_value(effects,"Available for operations"),"4 → 2");
        let sim=eq::retirement_quote(&g.world,NationId::USA,"fleet-ifv",2);
        assert_eq!(metric_value(effects,"Maintenance requirement removed").as_str(),Some(service_money(sim.maintenance_reduction_bn_day).as_str()));
        assert!(effects["warnings"].as_array().unwrap().iter().any(|s|s.as_str().unwrap().contains("not a cash saving")));
        assert_eq!(before,spheres_sim::save(&g.world));
        let orders=serde_json::to_string(&g.world.nation(NationId::USA).arsenal.orders).unwrap();
        let parsed=super::super::parse_command(&g.world,&result["actions"][0]["command"],NationId::USA).unwrap();
        spheres_sim::apply_command(&mut g.world,&parsed).unwrap();
        let h=g.world.nation(NationId::USA).arsenal.held.iter().find(|h|h.design_id.as_deref()==Some("fleet-ifv")).unwrap();
        assert_eq!((h.units,h.refit_reserved,h.age),(4.0,2,36.0));
        assert_eq!(eq::fleet_maintenance_requirement(g.world.nation(NationId::USA)),sim.maintenance_bn_day_after);
        assert_eq!(serde_json::to_string(&g.world.nation(NationId::USA).arsenal.orders).unwrap(),orders);
    }
    #[test]
    fn retirement_rejects_reserved_or_stale_quantities_and_malformed_commands() {
        let mut g=fixture();let before=spheres_sim::save(&g.world);
        let rejected=preview(&g.world,NationId::USA,&g.session_id,&json!({"command":command(json!(5))})).unwrap();
        assert_eq!(rejected["valid"],false);assert_eq!(rejected["actions"][0]["enabled"],false);
        assert!(rejected["service_effects"]["roles"].as_array().unwrap().is_empty());
        assert_eq!(spheres_sim::save(&g.world),before);
        for bad in [json!(-1),json!(1.5),json!("2"),json!(4294967296u64)]{assert!(super::super::parse_command(&g.world,&command(bad),NationId::USA).is_none());}
        let c=super::super::parse_command(&g.world,&command(json!(3)),NationId::USA).unwrap();
        spheres_sim::apply_command(&mut g.world,&c).unwrap();let retired=spheres_sim::save(&g.world);
        assert!(spheres_sim::apply_command(&mut g.world,&c).is_err());assert_eq!(spheres_sim::save(&g.world),retired);
    }
    #[test]
    fn empty_fleet_explains_next_step_without_inventing_service_coverage() {
        let mut g=super::super::Game::new(1990,Some(NationId::USA));super::super::play_rules(&mut g);
        let data=equipment_service_board(&g.world,NationId::USA);
        assert_eq!(metric_value(&data,"Delivered custom vehicles").to_string(),"0.0");
        assert_eq!(metric_value(&data,"Completed vehicles in transit").to_string(),"0.0");
        assert_eq!(metric_value(&data,"Current custom maintenance requirement"),"$0.00 / day");
        assert_eq!(data["status"],"No custom vehicles delivered");assert!(data["roles"].as_array().unwrap().is_empty());
        assert!(data["actions"].as_array().unwrap().iter().any(|a|a["navigate"]["tab"]=="designer"));
        assert!(data["metrics"].as_array().unwrap().iter().all(|m|m["label"]!="Last settled maintenance coverage"));
    }
}
