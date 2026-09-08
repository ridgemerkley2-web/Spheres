fn ammunition_reserve_action(w:&WorldState,me:NationId,family:&str,suggested:u32)->Value {
    let plan=w.nation(me).equipment.as_ref().and_then(|s|s.ammunition_reserves.get(family));
    let target=plan.map_or(suggested.max(1).min(eq::MAX_AMMO_RESERVE_TARGET),|p|p.target_rounds);
    let funding=plan.map_or(0.0001,|p|p.daily_limit_bn);
    let automatic=plan.is_some_and(|p|p.automatic);
    let preferred=sites(w,me).first().and_then(|p|p["value"].as_str()).map(str::to_string);
    let default_district=w.districts.iter().find(|(_,owner)|**owner==me).map(|(id,_)|id.as_str()).unwrap_or("");
    let district=plan.map(|p|p.district.clone()).or(preferred).unwrap_or_else(||default_district.to_string());
    let mut options:Vec<Value>=w.districts.iter().filter(|(_,owner)|**owner==me).map(|(id,_)|json!({"value":id,"label":format!("{} · {} arms-plant slots",spheres_sim::districts::name_of(id).unwrap_or(id),spheres_sim::manufacturing::plant_slots(w,id))})).collect();
    if !district.is_empty()&&!options.iter().any(|o|o["value"]==district) {
        options.insert(0,json!({"value":district,"label":format!("{} · unavailable; choose an owned province",spheres_sim::districts::name_of(&district).unwrap_or(&district))}));
    }
    let mut action=intent(if plan.is_some(){"Review reserve plan"}else{"Set ammunition reserve"},json!({"kind":"equipment_ammo_reserve","family":family,"target_rounds":target,"district":district,"daily_budget_mn":funding*1000.0,"automatic":automatic}),vec![
        json!({"key":"target_rounds","label":if family.starts_with("air_bomb_"){"Target mission stores in reserve"}else{"Target rounds in reserve"},"type":"number","value":target,"min":0,"max":eq::MAX_AMMO_RESERVE_TARGET,"step":1}),
        json!({"key":"automatic","label":"Replenishment","type":"select","value":automatic,"options":[{"value":false,"label":"Manual target · review each order"},{"value":true,"label":"Automatic replenishment · authorize future batches"}]}),
        json!({"key":"district","label":"Preferred production province","type":"select","value":district,"options":options}),
        json!({"key":"daily_budget_mn","label":"New batch spending ceiling","type":"number","value":funding*1000.0,"min":0,"max":1_000_000,"step":0.01,"unit":"$m / day"}),
    ]);
    action["detail"]=json!("Choose the physical reserve you want to maintain. A manual target only tracks the gap. Automatic replenishment authorizes future finite batches at this site and ceiling, after existing batches finish. Existing stock and unfinished work count before another order is created.");
    action
}

fn ammunition_reserve_cards(w:&WorldState,me:NationId)->Vec<Value> {
    let Some(state)=w.nation(me).equipment.as_ref() else{return vec![];};
    state.ammunition_reserves.iter().map(|(family,p)|{
        let r=eq::ammo_reserve_status(w,me,family);
        let mut metrics=reserve_metrics(&r);
        metrics.push(metric("Preferred province",spheres_sim::districts::name_of(&p.district).unwrap_or(&p.district)));
        metrics.push(metric("Automatic review from",if p.automatic{super::settled_day_json(p.authorized_day+1)["label"].clone()}else{json!("Manual review only")}));
        let mut detail=r.reason.clone();
        if let Some(receipt)=&p.last_review {
            detail.push_str(&format!(" Last review: {}",receipt.reason));
            if let Some(order)=receipt.order_id {detail.push_str(&format!(" Batch {order}: {} {}.",ammo_quantity(receipt.ordered_rounds as f64),if family.starts_with("air_bomb_"){"mission stores"}else{"rounds"}));}
        }
        let mut actions=vec![ammunition_reserve_action(w,me,family,p.target_rounds)];
        if r.gap>0 {
            let mut fill=ammo_order_action(w,me,family,r.gap.min(eq::MAX_AMMO_ORDER));
            fill["command"]["district"]=json!(p.district);
            fill["command"]["daily_budget_mn"]=json!(p.daily_limit_bn*1000.0);
            for input in fill["inputs"].as_array_mut().unwrap() {
                if input["key"]=="district" {
                    input["value"]=json!(p.district);
                    let options=input["options"].as_array_mut().unwrap();
                    if !options.iter().any(|o|o["value"]==p.district) {options.insert(0,json!({"value":p.district,"label":format!("{} · configured site; review availability",spheres_sim::districts::name_of(&p.district).unwrap_or(&p.district))}));}
                }else if input["key"]=="daily_budget_mn" {input["value"]=json!(p.daily_limit_bn*1000.0);}
            }
            fill["label"]=json!("Review a batch toward this target");
            fill["detail"]=json!("This is a separately reviewed finite order. The reserve plan will count it before scheduling more. Existing batches keep their own spending ceilings and controls.");
            actions.push(fill);
        }
        let mut clear=checked(w,me,"Clear reserve plan",json!({"kind":"equipment_ammo_reserve_clear","family":family}));
        clear["requires_preview"]=json!(true);
        clear["detail"]=json!("Remove this target and stop its future automatic orders. Already scheduled batches keep their existing funding and must be paused or canceled separately.");
        actions.push(clear);
        json!({"id":family,"name":format!("{} reserve",eq::ammo_def(family).map_or(family.as_str(),|d|d.name)),"status":if p.automatic{"Automatic replenishment"}else{"Manual target"},"detail":detail,"metrics":metrics,
            "costs":[cost("New batch daily ceiling",p.daily_limit_bn,"existing batches retain their own controls; all share Defense maintenance after upkeep")],
            "receipt_label":p.last_review.as_ref().map(|r|super::settled_day_json(r.day)["label"].clone()),"actions":actions})
    }).collect()
}

fn reserve_metrics(r:&eq::AmmoReserveStatus)->Vec<Value> {
    let aircraft=r.family.starts_with("air_bomb_");
    let mut rows=vec![metric("Target reserve",ammo_quantity(r.target_rounds.unwrap_or(0) as f64)),metric(if aircraft{"Mission stores on hand"}else{"Rounds currently in stores"},ammo_quantity(r.stock)),
        metric("Unfinished batch commitments",ammo_quantity(r.committed as f64)),metric("Stores plus planned output",ammo_quantity(r.projected)),metric(if aircraft{"Additional whole mission stores needed"}else{"Additional whole rounds needed"},ammo_quantity(r.gap as f64))];
    if r.paused_committed>0 {rows.push(metric("Of planned output, paused",ammo_quantity(r.paused_committed as f64)));}
    rows
}

fn ammunition_reserve_preview(w:&WorldState,me:NationId,session:&str,command:&Value,order:&EquipmentOrder)->Value {
    let label=match order {EquipmentOrder::AmmoReserve{automatic:true,..}=>"Authorize automatic replenishment",EquipmentOrder::AmmoReserve{..}=>"Save manual reserve target",_=>"Clear this reserve plan"};
    let action=checked(w,me,label,command.clone());
    let blockers:Vec<String>=action["reason"].as_str().map(str::to_string).into_iter().collect();
    let mut metrics=vec![];let mut costs=vec![];let mut timing=vec![];let mut requirements:Vec<String>=vec![];
    let detail=match order {
        EquipmentOrder::AmmoReserve{family,target_rounds,district,daily_budget_bn,automatic}=>{
            let q=eq::ammo_reserve_quote(w,me,family,*target_rounds,district,*daily_budget_bn,*automatic);
            metrics.push(metric("Ammunition family",eq::ammo_def(family).map_or(family.as_str(),|d|d.name)));
            metrics.extend(reserve_metrics(&q.status));
            metrics.push(metric("Current scheduling condition",&q.status.reason));
            costs.push(cost("New batch daily ceiling",*daily_budget_bn,"future finite batches; shared Defense maintenance after fleet upkeep"));
            if let Some(batch)=q.status.batch.as_ref().filter(|_|q.status.next_quantity>0) {
                metrics.push(metric("Next batch size",ammo_quantity(q.status.next_quantity as f64)));
                costs.push(cost("Next batch fabrication estimate",batch.cost_bn,"raw inputs separate; charged only as actual work progresses"));
            }
            timing.push(metric("Automatic review begins",if *automatic{super::settled_day_json(q.eligible_from_day)["label"].clone()}else{json!("Disabled · every batch needs manual review")}));
            requirements.push("Saving a target creates no rounds, immediate bill or raw-material purchase. Stock and unfinished batches, including paused work, count before ordering more.".into());
            requirements.push("Existing batches retain their frozen quantities, sites and individual spending ceilings. Lowering, disabling or clearing this plan never cancels them.".into());
            if *automatic {
                requirements.push("You authorize future finite batches whenever this family is below target and no batch for it remains unfinished. Each batch is at most 1,000,000 rounds. Work begins on a later eligible date and requires real factory access, raw inputs and available Defense maintenance funding.".into());
                requirements.push("This authorization can repeat while the plan remains automatic. It does not activate physical combat ammunition or purchase missing inputs. Review Resources for supplies and the production list to stop an existing batch.".into());
                "Automatic replenishment is a standing production instruction. Review the target, site and new-batch ceiling before authorizing repeated orders."
            }else{
                "A manual reserve target records the stock you want. It will explain the remaining gap, but every production order still needs your review."
            }
        },
        EquipmentOrder::AmmoReserveClear{family}=>{
            metrics.push(metric("Ammunition family",eq::ammo_def(family).map_or(family.as_str(),|d|d.name)));
            requirements.push("This stops future orders from the plan. Existing finite batches continue under their own controls; pause or cancel them separately if desired.".into());
            "Remove this reserve target without spending money, deleting stocks, refunding work or changing existing batches."
        },
        _=>unreachable!(),
    };
    json!({"session_id":session,"nation":me,"valid":blockers.is_empty(),"blockers":blockers,"metrics":metrics,"costs":costs,"timing":timing,"requirements":requirements,"actions":[action],"detail":detail})
}

#[cfg(test)]
mod ammunition_reserve_view_tests {
    use super::*;
    const ID:NationId=NationId::France;
    fn command(g:&super::super::Game,automatic:bool)->Value {
        let district=g.world.districts.iter().find(|(_,owner)|**owner==ID).unwrap().0;
        json!({"kind":"equipment_ammo_reserve","family":eq::ammunition_family(&eq::default_spec("ground_ifv")).unwrap(),"target_rounds":5000,"district":district,"daily_budget_mn":0.1,"automatic":automatic})
    }
    fn apply(g:&mut super::super::Game,c:&Value){let parsed=super::super::parse_command(&g.world,c,ID).unwrap();spheres_sim::apply_command(&mut g.world,&parsed).unwrap();}
    #[test]
    fn reserve_reviews_are_pure_and_targets_create_no_work_or_rounds() {
        let mut g=super::ammunition_view_tests::fixture();
        let family=eq::ammunition_family(&eq::default_spec("ground_ifv")).unwrap();
        let mut c=command(&g,false);c["family"]=json!(family);
        c["daily_budget_mn"]=json!(0.025);
        c["district"]=json!(g.world.districts.iter().filter(|(_,owner)|**owner==ID).nth(1).unwrap().0);
        let before=spheres_sim::save(&g.world);
        let q=preview(&g.world,ID,&g.session_id,&json!({"command":c})).unwrap();
        assert_eq!(q["valid"],true,"{q}");assert_eq!(q["actions"][0]["command"],c);
        assert!(q["detail"].as_str().unwrap().contains("manual reserve target"));
        let board=ammunition_board(&g.world,ID);
        assert!(board["reserves"].as_array().unwrap().is_empty());
        let action=&board["families"][0]["actions"][1];
        assert_eq!(action["command"]["automatic"],false);
        assert_eq!(action["inputs"][1]["options"][0]["value"],false);
        assert_eq!(action["inputs"][1]["options"][1]["value"],true);
        assert_eq!(spheres_sim::save(&g.world),before);
        if let Some(path)=std::env::var_os("SPHERES_RESERVES_QA_SAVE"){std::fs::write(path,&before).unwrap();}
        let treasury=g.world.nation(ID).treasury_bn;
        apply(&mut g,&c);
        let state=g.world.nation(ID).equipment.as_ref().unwrap();
        assert!(!state.ammunition_reserves[family].automatic);
        assert_eq!(state.ammunition_reserves[family].target_rounds,5000);
        assert!(state.ammunition.as_ref().is_none_or(|a|a.orders.is_empty()&&a.stocks.is_empty()));
        assert_eq!(g.world.nation(ID).treasury_bn,treasury);
        let row=&ammunition_board(&g.world,ID)["reserves"][0];
        assert_eq!(row["status"],"Manual target");
        assert!(row["metrics"].as_array().unwrap().iter().any(|m|m["label"]=="Target reserve"&&m["value"]=="5,000"));
        let fill=&row["actions"][1];assert_eq!(fill["command"]["district"],c["district"]);assert_eq!(fill["command"]["daily_budget_mn"],c["daily_budget_mn"]);
        assert!(fill["inputs"].as_array().unwrap().iter().any(|i|i["key"]=="district"&&i["value"]==c["district"]));
        assert!(fill["inputs"].as_array().unwrap().iter().any(|i|i["key"]=="daily_budget_mn"&&i["value"]==c["daily_budget_mn"]));
    }
    #[test]
    fn automatic_reserve_authorization_and_clearing_explain_retained_orders() {
        let mut g=super::ammunition_view_tests::fixture();
        let family=eq::ammunition_family(&eq::default_spec("ground_ifv")).unwrap();
        let mut c=command(&g,true);c["family"]=json!(family);
        let q=preview(&g.world,ID,&g.session_id,&json!({"command":c})).unwrap();
        assert_eq!(q["valid"],true,"{q}");
        assert_eq!(q["actions"][0]["label"],"Authorize automatic replenishment");
        assert!(q["requirements"].as_array().unwrap().iter().any(|v|v.as_str().is_some_and(|s|s.contains("can repeat"))));
        apply(&mut g,&c);
        let district=c["district"].as_str().unwrap();
        eq::start_ammo_order(&mut g.world,ID,family,district,1000,0.0001).unwrap();
        let batches=g.world.nation(ID).equipment.as_ref().unwrap().ammunition.clone();
        let clear=json!({"kind":"equipment_ammo_reserve_clear","family":family});
        let before=spheres_sim::save(&g.world);
        let q=preview(&g.world,ID,&g.session_id,&json!({"command":clear})).unwrap();
        assert_eq!(q["valid"],true,"{q}");
        assert!(q["requirements"].as_array().unwrap().iter().any(|v|v.as_str().is_some_and(|s|s.contains("Existing finite batches continue"))));
        assert_eq!(spheres_sim::save(&g.world),before);
        apply(&mut g,&clear);
        let state=g.world.nation(ID).equipment.as_ref().unwrap();
        assert!(state.ammunition_reserves.is_empty());assert_eq!(state.ammunition,batches);
    }
    #[test]
    fn reserve_parser_rejects_ambiguous_automation_and_invalid_targets_without_mutation() {
        let g=super::ammunition_view_tests::fixture();let before=spheres_sim::save(&g.world);
        for v in [json!("true"),json!("false"),json!(1),json!(null)] {
            let mut c=command(&g,false);c["automatic"]=v;assert!(super::super::parse_command(&g.world,&c,ID).is_none());
        }
        for v in [json!(-1),json!(0.5),json!(4294967296u64),json!("5000")] {
            let mut c=command(&g,false);c["target_rounds"]=v;assert!(super::super::parse_command(&g.world,&c,ID).is_none());
        }
        for (key,value) in [("target_rounds",json!(eq::MAX_AMMO_RESERVE_TARGET+1)),("family",json!("unknown")),("district",json!("unknown")),("daily_budget_mn",json!(-1))] {
            let mut c=command(&g,false);c[key]=value;
            let q=preview(&g.world,ID,&g.session_id,&json!({"command":c})).unwrap();assert_eq!(q["valid"],false,"{q}");
        }
        assert_eq!(spheres_sim::save(&g.world),before);
    }
}
