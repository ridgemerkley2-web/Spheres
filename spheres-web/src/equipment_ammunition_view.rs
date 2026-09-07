fn ammo_quantity(value:f64)->String {
    let value=if value==0.0{0.0}else{value};
    let fixed=if value>0.0&&value<0.001 {format!("{value:.6}")}else{format!("{value:.3}")};
    let trimmed=fixed.trim_end_matches('0').trim_end_matches('.');
    let (whole,fraction)=trimmed.split_once('.').unwrap_or((trimmed,""));
    let mut grouped=String::new();
    for (i,c) in whole.chars().enumerate() {
        if i>0&&(whole.len()-i)%3==0 {grouped.push(',');}
        grouped.push(c);
    }
    if !fraction.is_empty(){grouped.push('.');grouped.push_str(fraction);}
    grouped
}

fn ammo_order_action(w:&WorldState,me:NationId,family:&str,quantity:u32)->Value {
    if companies::ammo_supplier_active(w,me,family) {
        return nav("Review company ammunition stock",json!({"action":"equipment","tab":"ammunition"}));
    }
    let options=sites(w,me);
    let site=options.iter().find(|o|o["value"].as_str().is_some_and(|d|eq::ammo_order_quote(w,me,family,d,quantity,0.0001).valid))
        .or_else(||options.first()).map(|o|o["value"].clone()).unwrap_or(json!(""));
    let mut action=intent("Review ammunition production",json!({"kind":"equipment_ammo_order","family":family,"district":site,"quantity":quantity,"daily_budget_mn":0.1}),vec![
        json!({"key":"district","label":"Arms plant province","type":"select","value":site,"options":options}),
        json!({"key":"quantity","label":if family.starts_with("air_bomb_"){"Mission stores to manufacture"}else{"Rounds to manufacture"},"type":"number","value":quantity,"min":1,"max":eq::MAX_AMMO_ORDER,"step":1}),
        budget_input(0.0001),
    ]);
    action["detail"]=json!("Manufacture a finite batch of this exact ammunition family. Each day consumes raw inputs and remaining Defense maintenance funds, after fleet upkeep. Completed rounds enter national stores; starting the order creates none.");
    action
}

fn ammunition_board(w:&WorldState,me:NationId)->Value {
    let n=w.nation(me);let equipment=n.equipment.as_ref();let state=equipment.and_then(|s|s.ammunition.as_ref());
    let reading=eq::ammunition_overview(w,me);
    let ground_active=eq::ammunition_active(n,spheres_sim::clock::absolute_day(w));
    let has_aircraft=equipment.is_some_and(|s|s.revisions.values().any(|r|r.profile.aviation.is_some()));
    let mut families=vec![];
    for def in eq::ammo_catalog() {
        let models:Vec<_>=equipment.into_iter().flat_map(|s|s.revisions.values())
            .filter(|r|r.certified_day.is_some()&&eq::ammunition_family(&r.spec)==Some(def.id)).map(|r|r.name.clone()).collect();
        let stock=state.and_then(|s|s.stocks.get(def.id)).copied().unwrap_or(0.0);
        let jobs:Vec<_>=state.into_iter().flat_map(|s|s.orders.iter()).filter(|o|o.family==def.id&&!matches!(o.status,eq::ProjectStatus::Complete|eq::ProjectStatus::Cancelled)).collect();
        let incoming:u64=jobs.iter().map(|o|o.quantity.saturating_sub(o.completed_rounds) as u64).sum();
        let company_incoming=companies::ammo_inbound_units(w,me,def.id);
        if models.is_empty()&&stock<=0.0&&jobs.is_empty(){continue;}
        let vehicles:u64=n.arsenal.held.iter().filter(|h|h.design_id.as_ref().and_then(|id|equipment.and_then(|s|s.revisions.get(id))).is_some_and(|r|eq::ammunition_family(&r.spec)==Some(def.id)))
            .map(|h|spheres_sim::arsenal::available_design_units(h) as u64).sum();
        let aviation=def.id.starts_with("air_bomb_");
        let reference=if aviation {n.arsenal.held.iter().filter_map(|h|h.design_id.as_ref().and_then(|id|equipment.and_then(|s|s.revisions.get(id))).and_then(|r|r.profile.aviation.as_ref()).filter(|a|a.store_family==def.id).map(|a|spheres_sim::arsenal::available_design_units(h) as f64*a.sorties_per_aircraft_month*a.stores_per_sortie)).sum()}else{vehicles as f64*def.rounds_per_vehicle_month};
        let reserve=equipment.and_then(|s|s.ammunition_reserves.get(def.id));
        let target=reserve.map_or(reference,|p|p.target_rounds as f64);
        let gap=(target-stock-incoming as f64-company_incoming as f64).max(0.0);
        let suggested=if gap>0.0 {gap.ceil().min(eq::MAX_AMMO_ORDER as f64) as u32}else{def.rounds_per_day.ceil().max(1.0).min(eq::MAX_AMMO_ORDER as f64) as u32};
        let usage=reading.families.iter().find(|f|f.family==def.id);
        let required=usage.map_or(0.0,|f|f.required);let used=usage.map_or(0.0,|f|f.used);
        let unit=if aviation{"stores"}else{"rounds"};
        let mut metrics=vec![metric(if aviation{"Mission stores on hand"}else{"Rounds in stores"},ammo_quantity(stock)),metric(if aviation{"Compatible available aircraft"}else{"Compatible available vehicles"},vehicles),metric(if aviation{"Stores still in manufacture"}else{"Rounds still in manufacture"},incoming),
            metric("Current operations require",format!("{} {unit} this tick",ammo_quantity(required))),metric("Current stores can supply",format!("{} {unit} this tick",ammo_quantity(used))),
            metric("One-month planning reserve",format!("{} {unit}",ammo_quantity(reference))),metric("Reserve gap after all active orders",format!("{} {unit}",ammo_quantity(gap)))];
        if reserve.is_some(){metrics.insert(5,metric("Saved reserve target",ammo_quantity(target)));}
        metrics.push(metric("Purchased company stock in transit",format!("{} {unit}",ammo_quantity(company_incoming as f64))));
        if let Some(u)=usage.filter(|u|u.required>0.0){metrics.push(metric("Firing demand supported",format!("{:.1}%",u.coverage*100.0)));}
        let mut detail=format!("Compatible certified models: {}. The reserve is a modeled month of firing per available vehicle, not a battle forecast. {}",if models.is_empty(){"none currently available".into()}else{models.join(", ")},if ground_active{"Current operation requirements account for the vehicles deployed and their missions."}else{"These stores will supply custom weapons after physical ammunition activation."});
        if aviation {detail=format!("Compatible aircraft: {}. The planning reserve uses each model's frozen sortie rate and stores per sortie. Aircraft always require their exact physical mission stores, without activating ground ammunition. Actual use depends on supported aircraft, assigned air raids and theatre access.",models.join(", "));}
        if incoming>0 {detail.push_str(" Outstanding batches are conditional on funding, raw inputs and plant access; paused work is included in the planned reserve.");}
        let mut actions=vec![];
        if !models.is_empty(){actions.push(ammo_order_action(w,me,def.id,suggested));actions.push(ammunition_reserve_action(w,me,def.id,reference.ceil().max(def.rounds_per_day).min(eq::MAX_AMMO_RESERVE_TARGET as f64) as u32));}
        families.push(json!({"id":def.id,"name":def.name,"status":if required>used+1e-9{"Firing shortage"}else if stock<=0.0{if aviation{"No mission stores"}else{"No rounds in stores"}}else{"Stores available"},"detail":detail,"metrics":metrics,
            "costs":[cost(if aviation{"Fabrication per mission store"}else{"Fabrication per round"},def.fabrication_bn,"raw inputs separate")],"actions":actions}));
    }
    let mut orders=vec![];
    if let Some(state)=state {for o in &state.orders {
        let aviation=o.family.starts_with("air_bomb_");let unit=if aviation{"stores"}else{"rounds"};
        let active=!matches!(o.status,eq::ProjectStatus::Complete|eq::ProjectStatus::Cancelled);
        let mut actions=vec![];
        if active {
            let mut pause=checked(w,me,if o.paused{"Resume ammunition batch"}else{"Pause ammunition batch"},json!({"kind":"equipment_ammo_pause","project":o.id,"paused":!o.paused}));
            pause["requires_preview"]=json!(true);
            actions.push(pause);
            actions.push(intent("Review ammunition funding",json!({"kind":"equipment_ammo_funding","project":o.id,"daily_budget_mn":o.daily_limit_bn*1000.0}),vec![budget_input(o.daily_limit_bn)]));
            let mut cancel=checked(w,me,"Cancel unfinished ammunition",json!({"kind":"equipment_ammo_cancel","project":o.id}));
            cancel["requires_preview"]=json!(true);
            cancel["detail"]=json!("Completed rounds remain in stores. Consumed inputs and paid work are not refunded; incomplete work is abandoned and the plant slot is released.");actions.push(cancel);
        }
        orders.push(json!({"id":o.id,"name":format!("{} · batch {}",eq::ammo_def(&o.family).map_or(o.family.as_str(),|d|d.name),o.id),"status":format!("{:?}",o.status),"detail":o.reason,
            "progress":o.work_rounds/o.quantity as f64,"metrics":[metric(if aviation{"Completed mission stores"}else{"Completed rounds"},format!("{} / {}",ammo_quantity(o.completed_rounds as f64),ammo_quantity(o.quantity as f64))),metric("Province",spheres_sim::districts::name_of(&o.district).unwrap_or(&o.district)),metric("Maximum physical production",format!("{} {unit} / day",ammo_quantity(o.rounds_per_day)))],
            "costs":[cost("Fabrication paid",o.spent_bn,"cumulative"),cost("Finite fabrication bill",o.cost_bn,"raw inputs separate"),cost("Daily spending ceiling",o.daily_limit_bn,"Defense maintenance, after upkeep")],
            "awaiting_receipt":o.completed_day.or(o.last_day).is_none(),"receipt_label":o.completed_day.or(o.last_day).map(|d|super::settled_day_json(d)["label"].clone()),"actions":actions}));
    }}
    let mut overview_metrics=vec![metric("Compatible ammunition families",families.len()),metric("Funding source","Defense · Maintenance, after fleet upkeep")];
    let mut warnings=vec![];let mut actions=vec![];
    if families.is_empty() {
        warnings.push("Certify a custom ground or aircraft design to reveal its compatible ammunition family here. Inherited equipment continues to use the national magazine.".into());
        actions.push(nav("Design your first vehicle",json!({"action":"equipment","tab":"designer"})));
    }
    let activation=state.and_then(|s|s.active_from_day);
    let status=if ground_active{"Physical custom ammunition active"}else if has_aircraft{"Aircraft stores required · ground activation separate"}else if activation.is_some(){"Activation scheduled"}else{"Prepare stores before activation"};
    if let Some(day)=activation {overview_metrics.push(metric("Physical supply from",super::settled_day_json(day)["label"].clone()));}
    if let Some(receipt)=state.and_then(|s|s.last_consumption.as_ref()) {
        overview_metrics.push(metric("Last consumption date",super::settled_day_json(receipt.day)["label"].clone()));
        overview_metrics.push(metric("Last ammunition expended",ammo_quantity(receipt.used.values().sum())));
    }
    if activation.is_none() {
        let mut action=checked(w,me,"Review physical ammunition activation",json!({"kind":"equipment_ammo_activate"}));
        action["requires_preview"]=json!(true);
        action["detail"]=json!("Switch custom ground weapons to compatible physical stores from the next eligible date. This cannot be switched off. No rounds are granted. Prepare stock first: empty or mismatched stores constrain firing. Inherited equipment keeps its own share of the existing magazine.");
        actions.push(action);
        warnings.push("Custom ground vehicles still use the existing shared magazine until you activate physical ground ammunition. Aircraft always require their compatible mission stores.".to_string());
    }
    if equipment.and_then(|s|s.maintenance_plan.as_ref()).is_none() {
        warnings.push("An actual maintenance plan is required before ammunition supply or purchases. Fleet upkeep is protected before the remaining Maintenance & supply funds can pay for ammunition.".into());
        actions.push(nav("Review maintenance plan",json!({"action":"equipment","tab":"service"})));
    }
    warnings.push("Stocks belong to exact weapon families. Wrong-caliber rounds, unfinished batches and refit-reserved vehicles cannot supply firing. Extra rounds do not increase standing force or create extra vehicles.".into());
    actions.push(nav("Review material purchasing plan",json!({"action":"equipment","tab":"production"})));
    actions.push(nav("Review Defense maintenance funding",json!({"action":"budget","ministry":"defense","department":2})));
    let overview=json!({"title":"National ammunition stores","status":status,
        "detail":if has_aircraft{"Acquire compatible aircraft mission stores before launching air raids. Aircraft need physical stores from their first deployment; ground ammunition activation is separate. Company purchases and public batches deliver into these exact-family stores. Actual consumption is shared across operations and recorded once."}else if ground_active{"Your custom ground fleet draws from compatible physical stores. Review manufacturer stock and existing public batches as rounds are used. The national plan shares ammunition across operations and records consumption once; fleet upkeep is protected before ammunition purchases."}else{"Acquire compatible stores, then review physical ground ammunition activation when ready. Company purchases and existing public batches add actual rounds only on arrival or completion. Fleet upkeep is protected before ammunition purchases; inherited equipment keeps its existing magazine."},
        "metrics":overview_metrics,"warnings":warnings,"actions":actions});
    json!({"overview":overview,"supplier_market":company_ammunition_market(w,me),"reserves":ammunition_reserve_cards(w,me),"families":families,"orders":orders})
}

fn ammunition_preview(w:&WorldState,me:NationId,session:&str,command:&Value,order:&EquipmentOrder)->Value {
    let label=match order {EquipmentOrder::AmmoActivate=>"Activate physical ammunition",EquipmentOrder::AmmoOrder{..}=>"Start ammunition batch",EquipmentOrder::AmmoFunding{..}=>"Save ammunition funding",EquipmentOrder::AmmoPause{paused:true,..}=>"Pause this batch",EquipmentOrder::AmmoPause{..}=>"Resume this batch",_=>"Cancel unfinished batch"};
    let action=checked(w,me,label,command.clone());let blockers:Vec<String>=action["reason"].as_str().map(str::to_string).into_iter().collect();
    let mut metrics=vec![];let mut costs=vec![];let mut timing=vec![];let mut requirements=vec![];
    let detail=match order {
        EquipmentOrder::AmmoOrder{family,district,quantity,daily_budget_bn}=>{
            let aviation=family.starts_with("air_bomb_");
            let q=eq::ammo_order_quote(w,me,family,district,*quantity,*daily_budget_bn);
            metrics.push(metric("Ammunition family",eq::ammo_def(family).map_or(family.as_str(),|d|d.name)));
            metrics.push(metric(if aviation{"Mission stores to manufacture"}else{"Rounds to manufacture"},quantity));
            costs.push(cost("Finite fabrication bill",q.cost_bn,"paid as work progresses; raw inputs separate"));
            costs.push(cost("Maximum daily payment",*daily_budget_bn,"remaining Defense maintenance funds may constrain it"));
            timing.push(metric("Minimum physical work",format!("{} {}",q.minimum_days,if q.minimum_days==1{"day"}else{"days"})));
            requirements.push(if aviation{"Consumes one shared arms-plant slot. Work begins on an eligible future funding day; completed whole mission stores enter inventory. Aircraft require compatible stores from their first mission, independently of ground ammunition activation."}else{"Consumes one shared arms-plant slot. Work begins on an eligible future funding day; completed whole rounds enter stores. This order does not activate physical ammunition for combat."}.to_string());
            for c in spheres_sim::resources::ALL {let amount=q.recipe[c.idx()];if amount>0.0{requirements.push(format!("{}: {} {} for the entire batch; acquire these through the shared Resources market.",c.name(),ammo_quantity(amount),c.unit()));}}
            "Review the exact family, finite fabrication bill and raw inputs. Starting work does not create free rounds or immediately charge the full bill."
        },
        EquipmentOrder::AmmoActivate=>{
            metrics.push(metric("Effective","Next eligible date after confirmation"));
            metrics.push(metric("Starting ammunition grant","None"));
            requirements.push("Activation changes how custom ground weapons receive ammunition. It cannot be switched back to free shared supply; prepare compatible stocks first. Inherited equipment retains its share of the existing magazine.".into());
            requirements.push("Consumption is planned across all conserved deployments once. Empty stores limit firing; mobility and observation retain their separate operational roles. Extra stock grants no standing-force bonus.".into());
            "Your custom ground fleet will depend on compatible physical stores. Review the Ammunition tab before activating: manufacturing orders that have not completed cannot supply combat."
        },
        EquipmentOrder::AmmoFunding{daily_budget_bn,..}=>{costs.push(cost("New daily spending ceiling",*daily_budget_bn,"does not change already paid work"));"Change the future daily ceiling for this finite batch. Fleet upkeep still has first call on shared Defense maintenance funds."},
        EquipmentOrder::AmmoPause{..}=>"Change whether this batch can work on future funding days. Completed stock and paid work remain recorded; a paused batch keeps its plant reservation.",
        _=>"Cancel only the unfinished batch. Completed rounds remain in stores. Spent money and consumed raw inputs are not refunded; the plant slot is released.",
    };
    json!({"session_id":session,"nation":me,"valid":blockers.is_empty(),"blockers":blockers,"metrics":metrics,"costs":costs,"timing":timing,"requirements":requirements,"actions":[action],"detail":detail})
}

#[cfg(test)]
mod ammunition_view_tests {
    use super::*;
    const ID:NationId=NationId::France;
    pub(super) fn fixture()->super::super::Game {
        let mut g=super::super::Game::new(1990,Some(ID));super::super::play_rules(&mut g);
        spheres_sim::programs::set_construction_budget(&mut g.world,ID,0.0).unwrap();
        let spec=eq::default_spec("ground_ifv");let profile=eq::design_preview(&g.world,ID,&spec).profile.unwrap();
        let day=spheres_sim::clock::absolute_day(&g.world);
        let n=g.world.nation_mut(ID);n.arsenal.held.clear();n.arsenal.orders.clear();
        n.equipment.get_or_insert_with(Default::default).revisions.insert("ammo-demo".into(),eq::DesignRevision{id:"ammo-demo".into(),name:"Ammunition demonstration IFV".into(),specification_key:eq::specification_key(&spec),spec,profile,created_day:day,certified_day:Some(day)});
        spheres_sim::arsenal::deliver_design(n,"ammo-demo",10,0.0).unwrap();
        eq::set_maintenance_plan(&mut g.world,ID,0.0001).unwrap();
        let district=g.world.districts.iter().find(|(_,n)|**n==ID).unwrap().0.clone();
        g.world.production.provinces.push(spheres_sim::production::ProvinceCapabilities{district,infrastructure:0,civilian_industry:0,power_grid:0,research_centers:0,arms_plants:3});
        g.world.resources.market=Some(spheres_sim::resources::MarketState{prices:[1000.0;12],previous_prices:[1000.0;12],cleared_volume:[0.0;12],unmet_orders:[0.0;12],fills:vec![],contract_fills:vec![],shipment_audits:vec![],contract_spend_bn:vec![],stocks:spheres_sim::resources::ALL.iter().map(|c|spheres_sim::resources::Stock{nation:ID,commodity:*c,quantity:1_000_000.0,reserve_target:0.0}).collect(),cash:vec![],last_produced:-1,last_cleared:-1,last_produced_day:None,last_cleared_day:None,period_days:None});
        let n=g.world.nation_mut(ID);n.treasury_bn=Some(100.0);n.debt_bn=Some(0.0);n.debt_gdp=0.0;
        g
    }
    fn order(g:&super::super::Game,quantity:u32)->Value {
        let family=eq::ammunition_family(&g.world.nation(ID).equipment.as_ref().unwrap().revisions["ammo-demo"].spec).unwrap();
        let district=g.world.districts.iter().find(|(_,n)|**n==ID).unwrap().0;
        json!({"kind":"equipment_ammo_order","family":family,"district":district,"quantity":quantity,"daily_budget_mn":0.1})
    }
    #[test]
    fn ammunition_reading_and_order_review_are_pure_and_do_not_grant_rounds() {
        let mut g=fixture();let before=spheres_sim::save(&g.world);
        let data=view(&g.world,ID,&g.session_id);
        assert_eq!(data["ammunition"]["families"].as_array().unwrap().len(),1);
        assert_eq!(data["ammunition"]["overview"]["status"],"Prepare stores before activation");
        let command=order(&g,1000);let quote=preview(&g.world,ID,&g.session_id,&json!({"command":command})).unwrap();
        assert_eq!(quote["valid"],true,"{quote}");assert_eq!(quote["actions"][0]["command"],command);
        assert!(quote["requirements"].as_array().unwrap().iter().any(|v|v.as_str().is_some_and(|s|s.contains("entire batch"))));
        assert_eq!(spheres_sim::save(&g.world),before);
        if let Some(path)=std::env::var_os("SPHERES_AMMUNITION_QA_SAVE"){std::fs::write(path,&before).unwrap();}
        let parsed=super::super::parse_command(&g.world,&command,ID).unwrap();spheres_sim::apply_command(&mut g.world,&parsed).unwrap();
        let state=g.world.nation(ID).equipment.as_ref().unwrap().ammunition.as_ref().unwrap();
        assert_eq!(state.orders.len(),1);assert_eq!(state.orders[0].spent_bn,0.0);assert!(state.stocks.values().all(|v|*v==0.0));assert_eq!(state.active_from_day,None);
        let board=ammunition_board(&g.world,ID);assert_eq!(board["orders"].as_array().unwrap().len(),1);
        assert_eq!(board["orders"][0]["actions"][2]["command"]["kind"],"equipment_ammo_cancel");
    }
    #[test]
    fn ammunition_activation_is_reviewed_prospective_and_has_no_starting_stock() {
        let mut g=fixture();let command=json!({"kind":"equipment_ammo_activate"});let before=spheres_sim::save(&g.world);
        let quote=preview(&g.world,ID,&g.session_id,&json!({"command":command})).unwrap();assert_eq!(quote["valid"],true,"{quote}");
        assert!(quote["requirements"].as_array().unwrap().iter().any(|v|v.as_str().is_some_and(|s|s.contains("cannot be switched back"))));
        assert_eq!(spheres_sim::save(&g.world),before);
        let parsed=super::super::parse_command(&g.world,&command,ID).unwrap();spheres_sim::apply_command(&mut g.world,&parsed).unwrap();
        let state=g.world.nation(ID).equipment.as_ref().unwrap().ammunition.as_ref().unwrap();
        assert!(state.active_from_day.unwrap()>spheres_sim::clock::absolute_day(&g.world));assert!(state.stocks.is_empty());
        assert_eq!(ammunition_board(&g.world,ID)["overview"]["status"],"Activation scheduled");
    }
    #[test]
    fn malformed_ammunition_commands_cannot_mutate_or_bypass_compatibility() {
        let g=fixture();let before=spheres_sim::save(&g.world);
        for quantity in [json!(-1),json!(1.5),json!(4294967296u64),json!("100")]{let mut c=order(&g,100);c["quantity"]=quantity;assert!(super::super::parse_command(&g.world,&c,ID).is_none());}
        for (key,value) in [("family",json!("not-a-family")),("quantity",json!(0)),("daily_budget_mn",json!(-1)),("district",json!("not-a-province"))] {
            let mut c=order(&g,100);c[key]=value;let q=preview(&g.world,ID,&g.session_id,&json!({"command":c})).unwrap();assert_eq!(q["valid"],false,"{q}");assert_eq!(q["actions"][0]["enabled"],false);
        }
        let c=json!({"kind":"equipment_ammo_pause","project":1,"paused":"true"});assert!(super::super::parse_command(&g.world,&c,ID).is_none());
        assert_eq!(spheres_sim::save(&g.world),before);
    }
}
