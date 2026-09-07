// Server-owned material readiness, separate from orders and financial quotes.
// Quantities stay in each commodity's physical recipe units throughout a row.
fn equipment_supply_resources(remaining:&[f64;12],planned:&[f64;12],stock:&[f64;12],shortfall:&[f64;12])->Vec<Value> {
    spheres_sim::resources::ALL.iter().filter(|c|remaining[c.idx()]>0.0||planned[c.idx()]>0.0).map(|c|{
        let i=c.idx();json!({"commodity":c.key(),"name":c.name(),"unit":c.unit(),
            "remaining":remaining[i],"planned_day":planned[i],"stock":stock[i],"shortfall":shortfall[i],
            "detail":"Remaining is the unconsumed material requirement, not a purchase order. Stock is shared; this view reserves nothing."})
    }).collect()
}
fn equipment_supply_actions(rows:&[Value])->Vec<Value> {
    rows.iter().map(|r|nav(&format!("Review {} supply",r["name"].as_str().unwrap_or("material")),
        json!({"action":"resources","commodity":r["commodity"]}))).collect()
}
fn equipment_supply_stock(w:&WorldState,me:NationId)->[f64;12] {
    std::array::from_fn(|i|spheres_sim::resources::stockpile(w,me,spheres_sim::resources::ALL[i]))
}
fn equipment_project_supply(_w:&WorldState,_me:NationId,p:&eq::ProjectSupply)->Value {
    let rows=equipment_supply_resources(&p.remaining,&p.planned_day,&p.stock,&p.shortfall);
    let ended=p.stage=="complete"||p.stage=="cancelled";
    let short=p.shortfall.iter().any(|q|*q>1e-9);
    let status=if ended {"Closed"} else if !p.blockers.is_empty(){"Review required"}else if short{"Material gap"}else if p.stage=="tooling"{"Preparing the line"}else{"Next work reviewed"};
    let detail=match p.stage.as_str() {
        "development"=>"Development consumes funding and engineering time. This stage has no raw-material recipe.",
        "tooling"=>"Tooling consumes funding first. The remaining bill below is for later fabrication; zero material use during tooling does not mean the batch is supplied.",
        "complete"=>"This programme has completed its contracted work. No further materials are planned.",
        "cancelled"=>"This programme was cancelled. Its unperformed material requirement is no longer scheduled.",
        _=>"The next work estimate follows project priority, current funding and plant availability. Shared warehouse stock is reduced by earlier custom-project work; other consumers and future deliveries may change it before settlement.",
    };
    let mut warnings=p.blockers.clone();
    if !ended&&!short&&p.remaining.iter().zip(p.stock).any(|(need,have)|*need>have+1e-9) {
        warnings.push("Current stock does not cover the entire remaining material bill. Review supply before later fabrication or refit work; future deliveries are not counted as stock here.".into());
    }
    if p.earliest_work_day>p.plan_day {
        warnings.push(format!("This order cannot work before {}. No work is charged on its issue date.",super::settled_day_json(p.earliest_work_day)["label"].as_str().unwrap_or("its first eligible date")));
    }
    let metrics=vec![metric("Planning date",super::settled_day_json(p.plan_day)["label"].clone()),
        metric("Planned work before material limits",format!("{:.3} work days",p.planned_work_days)),
        metric("Planned payment before material limits",format!("${:.3}m",p.planned_payment_bn*1000.0)),
        metric("Work supported by current stock",format!("{:.3} work days",p.executable_work_days)),
        metric("Remaining programme payment",format!("${:.3}m",p.remaining_payment_bn*1000.0))];
    let mut actions=equipment_supply_actions(&rows);
    if !ended {actions.push(nav("Review programme funding",json!({"action":"budget","ministry":"defense","department":p.department})));}
    json!({"title":"Programme supply readiness","stage":p.stage,"status":status,"detail":detail,"metrics":metrics,"resources":rows,"warnings":warnings,"actions":actions})
}

#[cfg(test)]
mod supply_view_tests {
    use super::*;
    fn fixture()->super::super::Game {
        let mut g=super::super::Game::new(1990,Some(NationId::USA));
        super::super::play_rules(&mut g);
        spheres_sim::programs::set_construction_budget(&mut g.world,NationId::USA,0.0).unwrap();
        let spec=eq::default_spec("ground_ifv");
        let profile=eq::design_preview(&g.world,NationId::USA,&spec).profile.unwrap();
        let today=spheres_sim::clock::absolute_day(&g.world);
        let s=g.world.nation_mut(NationId::USA).equipment.get_or_insert_with(Default::default);
        s.finance_from_day=today;
        s.revisions.insert("certified-ifv".into(),eq::DesignRevision{id:"certified-ifv".into(),name:"Certified IFV".into(),
            specification_key:eq::specification_key(&spec),spec,profile,created_day:today,certified_day:Some(today)});
        let district=g.world.districts.iter().find(|(_,n)|**n==NationId::USA).unwrap().0.clone();
        g.world.production.provinces.push(spheres_sim::production::ProvinceCapabilities{
            district,infrastructure:0,civilian_industry:0,power_grid:0,research_centers:0,arms_plants:3});
        g
    }
    fn order(g:&super::super::Game,quantity:u32)->Value {
        json!({"kind":"equipment_produce","revision":"certified-ifv","district":g.world.production.provinces.last().unwrap().district,
            "quantity":quantity,"daily_budget_mn":0.5})
    }
    #[test]
    fn production_quote_discloses_whole_recipe_before_order_without_mutation() {
        let g=fixture();let before=spheres_sim::save(&g.world);
        let first=preview(&g.world,NationId::USA,&g.session_id,&json!({"command":order(&g,2)})).unwrap();
        assert_eq!(first["valid"],true,"{}",first["blockers"]);
        let rows=first["supply"]["resources"].as_array().unwrap();assert!(!rows.is_empty());
        let profile=&g.world.nation(NationId::USA).equipment.as_ref().unwrap().revisions["certified-ifv"].profile;
        for row in rows {
            let c=spheres_sim::resources::Commodity::parse(row["commodity"].as_str().unwrap()).unwrap();
            assert_eq!(row["unit"],c.unit());
            assert!((row["remaining"].as_f64().unwrap()-profile.recipe[c.idx()]*2.0).abs()<1e-9);
            assert_eq!(row["planned_day"].as_f64().unwrap(),0.0,"new order cannot consume materials on issue date");
        }
        assert!(first["supply"]["actions"].as_array().unwrap().iter().any(|a|a["navigate"]["commodity"].is_string()));
        assert_eq!(spheres_sim::save(&g.world),before);
    }
    #[test]
    fn invalid_plant_retains_material_bill_but_cannot_confirm() {
        let g=fixture();let mut request=order(&g,3);request["district"]=json!("missing-province");
        let q=preview(&g.world,NationId::USA,&g.session_id,&json!({"command":request})).unwrap();
        assert_eq!(q["valid"],false);assert_eq!(q["supply"]["stage"],"unavailable");
        assert!(!q["supply"]["resources"].as_array().unwrap().is_empty());
        assert_eq!(q["actions"][0]["enabled"],false);
    }
    #[test]
    fn national_supply_counts_stock_once_and_retains_paused_outstanding_bills() {
        let mut g=fixture();let cmd=super::super::parse_command(&g.world,&order(&g,2),NationId::USA).unwrap();
        spheres_sim::apply_command(&mut g.world,&cmd).unwrap();
        spheres_sim::apply_command(&mut g.world,&cmd).unwrap();
        let second=g.world.nation(NationId::USA).equipment.as_ref().unwrap().projects[1].id;
        eq::set_project_paused(&mut g.world,NationId::USA,second,true).unwrap();
        let before=spheres_sim::save(&g.world);let data=view(&g.world,NationId::USA,&g.session_id);
        let per=&g.world.nation(NationId::USA).equipment.as_ref().unwrap().revisions["certified-ifv"].profile.recipe;
        for row in data["supply"]["resources"].as_array().unwrap() {
            let c=spheres_sim::resources::Commodity::parse(row["commodity"].as_str().unwrap()).unwrap();
            assert!((row["remaining"].as_f64().unwrap()-per[c.idx()]*4.0).abs()<2e-9);
            assert_eq!(row["stock"].as_f64().unwrap(),spheres_sim::resources::stockpile(&g.world,NationId::USA,c));
        }
        assert_eq!(spheres_sim::save(&g.world),before);
    }
    #[test]
    fn fabrication_gap_survives_save_and_is_visible_in_the_project_review() {
        let mut g=fixture();
        let cmd=super::super::parse_command(&g.world,&order(&g,4),NationId::USA).unwrap();
        spheres_sim::apply_command(&mut g.world,&cmd).unwrap();
        g.world.day=20;
        let project=&mut g.world.nation_mut(NationId::USA).equipment.as_mut().unwrap().projects[0];
        project.work_days=project.tooling_days as f64;
        project.spent_bn=project.tooling_cost_bn;
        // A controlled empty warehouse makes a real material constraint visible,
        // independent of opening-cover assumptions for any commodity.
        g.world.resources.market=Some(spheres_sim::resources::MarketState{
            prices:[1.0;12],previous_prices:[1.0;12],cleared_volume:[0.0;12],unmet_orders:[0.0;12],
            fills:vec![],contract_fills:vec![],shipment_audits:vec![],contract_spend_bn:vec![],stocks:vec![],cash:vec![],
            last_produced:-1,last_cleared:-1,last_produced_day:None,last_cleared_day:None,period_days:None,
        });
        let saved=spheres_sim::save(&g.world);
        let loaded=spheres_sim::load(&saved).unwrap();
        let data=view(&loaded,NationId::USA,&g.session_id);
        let supply=&data["production"][0]["supply"];
        assert_eq!(supply["stage"],"production");
        assert!(supply["resources"].as_array().unwrap().iter().any(|r|r["shortfall"].as_f64().unwrap()>0.0),"{supply}");
        let forecast=spheres_sim::economic_ai::raw_supply_forecast(&loaded,NationId::USA);
        let iron=forecast.lines.iter().find(|r|r.commodity==spheres_sim::resources::Commodity::Iron).unwrap();
        let resource=super::super::strategic_resource_json(&loaded,&forecast,iron);
        assert!(resource["drivers"].as_array().unwrap().iter().any(|r|r["label"]=="Equipment production and refit remaining"),"{resource}");
        let cards=super::super::stock_cards_json(&loaded,NationId::USA,spheres_sim::resources::Commodity::Iron);
        assert!(cards["advisor"].as_str().unwrap().starts_with("Custom equipment has "));
        assert!(!cards["trade"]["plus"].as_str().unwrap().contains("nothing needs"));
        assert_eq!(spheres_sim::save(&loaded),saved);
        // Opt-in fixture export supports browser QA without touching a campaign.
        if let Some(path)=std::env::var_os("SPHERES_SUPPLY_QA_SAVE") {
            std::fs::write(path,saved).unwrap();
        }
    }
}
fn equipment_national_supply(w:&WorldState,me:NationId,plans:&[eq::ProjectSupply])->Value {
    let active:Vec<_>=plans.iter().filter(|p|!["development","complete","cancelled"].contains(&p.stage.as_str())).collect();
    let mut remaining=[0.0;12];let planned=eq::next_work_supply(w,me).raw;let stock=equipment_supply_stock(w,me);
    for p in &active {for i in 0..12 {remaining[i]+=p.remaining[i];}}
    let shortfall=std::array::from_fn(|i|(planned[i]-stock[i]).max(0.0));
    let rows=equipment_supply_resources(&remaining,&planned,&stock,&shortfall);
    let blocked=active.iter().filter(|p|!p.blockers.is_empty()||p.shortfall.iter().any(|q|*q>1e-9)).count();
    let has_future_gap=remaining.iter().zip(stock).any(|(need,have)|*need>have+1e-9);
    let mut warnings=vec![];
    if has_future_gap {warnings.push("The warehouse does not cover the full outstanding bill. Review the resource forecast for timed deliveries, domestic supply and other claims before buying more.".to_string());}
    let mut actions=equipment_supply_actions(&rows);
    actions.push(nav("Open national supply forecast",json!({"action":"resources"})));
    if active.is_empty(){actions.push(nav("Review arms-plant construction",json!({"action":"construction","kind":"arms_plant"})));}
    json!({"title":"Production supply plan","stage":"national","status":if active.is_empty(){"No production commitments"}else if blocked>0{"Some programmes need attention"}else{"Plan reviewed"},
        "detail":if active.is_empty(){"Certified designs can be scheduled from the Library. Their funded production and refit material bills will appear here; development and inherited equipment are separate."}else{"Outstanding bills include paused production and refits. Next work allocates shared funding once in project priority order, assuming required inputs can be secured. Stock is counted once and is shared with other consumers. Individual programme reviews also account for earlier stock-limited work. Opening this plan reserves nothing and places no supply orders."},
        "metrics":[metric("Production and refit programmes",active.len()),metric("Programmes needing review",blocked),metric("Planning date",super::settled_day_json(spheres_sim::resources::forecast_start_day(w))["label"].clone())],
        "resources":rows,"warnings":warnings,"actions":actions})
}
fn equipment_order_supply(w:&WorldState,me:NationId,command:&Command,order:&EquipmentOrder,quote:Option<&eq::ProjectQuote>)->Option<Value> {
    if !matches!(order,EquipmentOrder::Develop{..}|EquipmentOrder::Produce{..}|EquipmentOrder::Refit{..}|EquipmentOrder::Funding{..}) {return None;}
    let mut proposed=w.clone();
    if spheres_sim::apply_command(&mut proposed,command).is_ok() {
        let project=if let EquipmentOrder::Funding{project,..}=order{*project}else{proposed.nation(me).equipment.as_ref()?.next_id.checked_sub(1)?};
        if let Some(plan)=eq::supply_plan(&proposed,me).iter().find(|p|p.project_id==project) {
            let mut supply=equipment_project_supply(&proposed,me,plan);
            supply["title"]=json!("Supply before you commit");
            supply["warnings"].as_array_mut().unwrap().push(json!("This is a preview of the proposed order. No materials or funds have been reserved by opening it."));
            return Some(supply);
        }
    }
    // Invalid plant/quantity/research choices cannot create a planning project.
    // Still show the quote's material bill when it is known, with no invented work.
    let q=quote?;let stock=equipment_supply_stock(w,me);let rows=equipment_supply_resources(&q.recipe,&[0.0;12],&stock,&[0.0;12]);
    let mut warnings=vec!["Resolve the order requirements to receive a dated work plan. The displayed material bill does not authorize production.".to_string()];
    if let Some(reason)=&q.reason{warnings.push(reason.clone());}
    Some(json!({"title":"Supply before you commit","stage":"unavailable","status":"Order not ready","detail":"Material quantities use the proposed batch size. Current stock is shared and does not guarantee future availability.","metrics":[],"resources":rows,"warnings":warnings,"actions":equipment_supply_actions(&rows)}))
}
