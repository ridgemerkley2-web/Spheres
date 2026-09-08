// Standing purchase authority and immutable receipts are owned by the simulator.
// A live illustrative bundle is separate from the next dated automatic review.
fn supply_policy_action(w:&WorldState,me:NationId)->Value {
    let plan=w.nation(me).equipment.as_ref().and_then(|s|s.supply_automation.as_ref()).and_then(|s|s.policy.as_ref());
    let automatic=plan.is_some_and(|p|p.automatic);
    let horizon=plan.map_or(30,|p|p.horizon_days);
    let cap=plan.map_or(0.001,|p|p.spending_cap_bn);
    let floor=plan.map_or(0.0,|p|p.cash_floor_bn);
    let interval=plan.map_or(7,|p|p.review_interval_days);
    let mut action=intent(if plan.is_some(){"Review purchasing plan"}else{"Set a purchasing plan"},json!({"kind":"equipment_supply_policy","automatic":automatic,"horizon_days":horizon,"spending_cap_mn":cap*1000.0,"cash_floor_mn":floor*1000.0,"review_interval_days":interval}),vec![
        json!({"key":"automatic","label":"Purchasing mode","type":"select","value":automatic,"options":[{"value":false,"label":"Manual purchases only"},{"value":true,"label":"Automatic purchases within these limits"}]}),
        json!({"key":"horizon_days","label":"Funded production to cover","type":"select","value":horizon,"options":[{"value":30,"label":"Next 30 days"},{"value":90,"label":"Next 90 days"},{"value":365,"label":"Next 365 days"}]}),
        json!({"key":"spending_cap_mn","label":"Maximum per review","unit":"$m","type":"number","value":cap*1000.0,"min":0,"max":1_000_000,"step":0.1}),
        json!({"key":"cash_floor_mn","label":"Cash to leave available","unit":"$m","type":"number","value":floor*1000.0,"min":0,"max":1_000_000,"step":1}),
        json!({"key":"review_interval_days","label":"Review purchases every","type":"select","value":interval,"options":[{"value":1,"label":"Day"},{"value":7,"label":"7 days"},{"value":30,"label":"30 days"}]}),
    ]);
    action["detail"]=json!("Choose whether future material purchases can repeat, how much each review may spend, and the cash this plan must leave available. Saving these settings makes no immediate purchase.");
    action
}

fn supply_automation_board(w:&WorldState,me:NationId)->Value {
    let state=w.nation(me).equipment.as_ref().and_then(|s|s.supply_automation.as_ref());
    let plan=state.and_then(|s|s.policy.as_ref());
    let reading=eq::supply_policy_status(w,me);
    let mut metrics=vec![metric("Purchasing mode",if reading.automatic{"Automatic within your limits"}else{"Manual purchases only"}),metric("Cash available now",replenishment_money(reading.cash_available_bn))];
    if let Some(p)=plan {
        metrics.extend([metric("Production horizon",format!("{} days",p.horizon_days)),metric("Maximum per review",replenishment_money(p.spending_cap_bn)),metric("Cash to leave available",replenishment_money(p.cash_floor_bn)),metric("Review interval",if p.review_interval_days==1{"Daily".to_string()}else{format!("Every {} days",p.review_interval_days)}),metric("Purchase limit at current cash",replenishment_money(reading.effective_cap_bn))]);
    }
    if let Some(day)=reading.next_review_day {metrics.push(metric("Next scheduled review",super::settled_day_json(day)["label"].clone()));}
    let mut actions=vec![supply_policy_action(w,me)];
    if plan.is_some() {
        let mut clear=checked(w,me,"Remove purchasing plan",json!({"kind":"equipment_supply_policy_clear"}));
        clear["requires_preview"]=json!(true);
        clear["detail"]=json!("Stop future automatic purchases. Paid shipments, existing production orders and past purchase receipts remain.");
        actions.push(clear);
    }
    actions.push(nav("Inspect materials and shipments",json!({"action":"resources"})));
    let reviews:Vec<Value>=state.into_iter().flat_map(|s|s.reviews.iter().rev()).map(|r|{
        let date=super::settled_day_json(r.day)["label"].as_str().unwrap_or("Recorded day").to_string();
        let mut metrics=vec![metric("Cash at review",replenishment_money(r.cash_available_bn)),metric("Authorized maximum",replenishment_money(r.spending_cap_bn)),metric("Protected cash",replenishment_money(r.cash_floor_bn)),metric("Usable purchase limit",replenishment_money(r.effective_cap_bn)),metric("Production horizon",format!("{} days",r.horizon_days)),metric("Review interval",if r.review_interval_days==1{"1 day".to_string()}else{format!("{} days",r.review_interval_days)})];
        let spent=r.purchase.as_ref().map_or(0.0,|p|p.spent_bn);
        metrics.push(metric("Actual payment",replenishment_money(spent)));
        for c in spheres_sim::resources::ALL {
            let i=c.idx();let bought=r.purchase.as_ref().map_or(0.0,|p|p.purchased[i]);
            if r.requested[i]>1e-9||bought>1e-9 {
                metrics.push(metric(c.name(),format!("{:.3} {} needed · {:.3} bought · {:.3} unfilled",r.requested[i],c.unit(),bought,(r.requested[i]-bought).max(0.0))));
            }
        }
        if let Some(receipt)=&r.purchase {for fill in &receipt.fills {
            metrics.push(metric(&format!("{} from {}",fill.commodity.name(),fill.seller.name()),format!("{:.3} {} · {} · {}",fill.quantity,fill.commodity.unit(),replenishment_money(fill.cost_bn),if fill.arrival_days==0{"delivered at purchase".into()}else{format!("{} days estimated at dispatch",fill.arrival_days)})));
        }}
        json!({"id":format!("supply-review:{}",r.day),"name":format!("Purchasing review · {date}"),"status":if spent>0.0{"Materials purchased"}else{"No purchase"},"detail":r.reason,"metrics":metrics,"receipt_label":date,"blockers":r.purchase.as_ref().map(|p|p.warnings.iter().filter(|text|*text!=&r.reason).cloned().collect::<Vec<_>>()).unwrap_or_default(),"actions":[nav("Inspect current shipments",json!({"action":"resources"}))]})
    }).collect();
    json!({"overview":{"title":"Military supply purchasing","status":if reading.automatic{"Automatic purchasing enabled"}else{"Manual purchases only"},"detail":reading.reason,
        "metrics":metrics,"roles_title":"How this plan works","roles":[
            {"label":"Buy for funded work","value":"Vehicles, refits and ammunition","detail":"The forecast covers finite production orders at their current funding. Reserve targets alone create no raw-material demand. Paused work is excluded."},
            {"label":"Count what is already covered","value":"Stock, expected supply and paid cargo","detail":"Even delayed paid shipments count before buying more. Goods become usable when they arrive and enter shared national stores."},
            {"label":"Spend within your limits","value":"Cash remaining after the day's bills","detail":"Each review uses its own ceiling and cash reserve. Unused limits do not accumulate; missed reviews are not replayed. This cash reserve limits these automatic purchases only."}],
        "warnings":["Material purchases pay the market separately from fabrication funding. This plan cannot guarantee suppliers, transit time or continuous production."],"actions":actions},"reviews":reviews})
}

fn supply_automation_preview(w:&WorldState,me:NationId,session:&str,command:&Value,order:&EquipmentOrder)->Value {
    let label=match order {EquipmentOrder::SupplyPolicy{automatic:true,..}=>"Authorize automatic purchases",EquipmentOrder::SupplyPolicy{..}=>"Save manual purchasing plan",_=>"Remove purchasing plan"};
    let action=checked(w,me,label,command.clone());
    let blockers:Vec<String>=action["reason"].as_str().map(str::to_string).into_iter().collect();
    let mut metrics=vec![];let mut timing=vec![];let mut requirements=vec![];let mut roles=vec![];let mut warnings=vec![];
    let detail=match order {
        EquipmentOrder::SupplyPolicy{horizon_days,spending_cap_bn,cash_floor_bn,review_interval_days,automatic}=>{
            let q=eq::supply_policy_quote(w,me,*horizon_days,*spending_cap_bn,*cash_floor_bn,*review_interval_days,*automatic);
            metrics.extend([metric("Purchasing mode",if *automatic{"Automatic future purchases"}else{"Manual purchases only"}),metric("Maximum per review",replenishment_money(*spending_cap_bn)),metric("Cash to leave available",replenishment_money(*cash_floor_bn)),metric("Production horizon",format!("{horizon_days} days")),metric("Review interval",if *review_interval_days==1{"Daily".to_string()}else{format!("Every {review_interval_days} days")}),metric("Cash available now",replenishment_money(q.status.cash_available_bn)),metric("Purchase limit at current cash",replenishment_money(q.status.effective_cap_bn))]);
            timing.push(metric("First automatic review",if *automatic{super::settled_day_json(q.status.next_review_day.unwrap_or(q.eligible_from_day))["label"].clone()}else{json!("Disabled") }));
            if let Some(example)=&q.status.quote {
                metrics.push(metric("Illustrative purchase at today's prices",replenishment_money(example.receipt.spent_bn)));
                for c in spheres_sim::resources::ALL {let i=c.idx();if example.requested[i]>1e-9 {
                    roles.push(json!({"label":c.name(),"value":format!("{:.3} {} needed · {:.3} could be bought now",example.requested[i],c.unit(),example.receipt.purchased[i]),"detail":format!("{:.3} {} already in paid shipments",example.paid_inbound[i].max(0.0),c.unit())}));
                }}
                if let Some(reason)=&example.reason {warnings.push(reason.clone());}
                warnings.extend(example.receipt.warnings.iter().cloned());
            }else if q.valid&&q.status.effective_cap_bn<1e-9 {
                warnings.push("At current cash, this spending ceiling or cash reserve allows no purchase. You can save the plan; future reviews will check the cash then available.".into());
            }else{warnings.push(q.status.reason.clone());}
            requirements.push("Saving this plan creates no immediate payment or shipment. Future reviews recalculate finite shortages, prices, cash and routes; today's example is not a reserved quote.".to_string());
            requirements.push("Your cash reserve applies only to this automatic purchasing plan. Ordinary bills and separately confirmed manual purchases can still use that cash. No borrowing is authorized by this plan.".to_string());
            requirements.push("Editing, disabling or removing this plan preserves paid cargo, production orders and dated receipts. Each review has a fresh ceiling; unused limits do not accumulate and missed reviews are never caught up.".to_string());
            if *automatic {
                requirements.push(format!("You authorize repeated purchases {}, up to the per-review limit, after daily fiscal settlement. Reviews that cannot buy anything also wait for the next scheduled date.",if *review_interval_days==1{"daily".to_string()}else{format!("every {review_interval_days} days")}));
                "Authorize a standing material-purchasing plan. Each future payment is bounded by your review limit, remaining cash above the reserve, and real available supply."
            }else{
                "Save these planning settings with automatic purchasing disabled. Continue using Review material purchase for separately confirmed one-time orders."
            }
        },
        EquipmentOrder::SupplyPolicyClear=>{
            requirements.push("Future automatic purchases stop. Paid shipments continue their journeys, existing production retains its funding, and past reviews remain visible.".into());
            "Remove the purchasing instruction. Existing orders, cargo and payment history are preserved."
        },
        _=>unreachable!(),
    };
    json!({"session_id":session,"nation":me,"valid":blockers.is_empty(),"blockers":blockers,"metrics":metrics,"costs":[],"timing":timing,"requirements":requirements,
        "service_effects":matches!(order,EquipmentOrder::SupplyPolicy{..}).then(||json!({"title":"Current material outlook","detail":"An illustration from today's world. Future automatic reviews use the supply, cash and prices then available.","metrics":[],"roles_title":"Finite production shortages","roles":roles,"warnings":warnings})),"actions":[action],"detail":detail})
}

#[cfg(test)]
mod supply_automation_view_tests {
    use super::*;
    const ID:NationId=NationId::France;
    fn command(automatic:bool)->Value {json!({"kind":"equipment_supply_policy","automatic":automatic,"horizon_days":30,"spending_cap_mn":0.1,"cash_floor_mn":10.0,"review_interval_days":7})}
    fn apply(g:&mut super::super::Game,c:&Value) {let parsed=super::super::parse_command(&g.world,c,ID).unwrap();spheres_sim::apply_command(&mut g.world,&parsed).unwrap();}
    #[test]
    fn purchasing_policy_review_is_pure_and_manual_is_the_default() {
        let mut g=super::replenishment_view_tests::fixture();let before=spheres_sim::save(&g.world);
        let board=supply_automation_board(&g.world,ID);
        assert_eq!(board["overview"]["status"],"Manual purchases only");
        assert_eq!(board["overview"]["actions"][0]["command"]["automatic"],false);
        assert!(board["reviews"].as_array().unwrap().is_empty());
        for automatic in [false,true] {
            let c=command(automatic);let q=preview(&g.world,ID,&g.session_id,&json!({"command":c})).unwrap();
            assert_eq!(q["valid"],true,"{q}");assert_eq!(q["actions"][0]["command"],c);
            assert_eq!(q["actions"][0]["label"],if automatic{"Authorize automatic purchases"}else{"Save manual purchasing plan"});
            assert!(q["metrics"].as_array().unwrap().iter().any(|m|m["label"]=="Maximum per review"&&m["value"]=="$0.100000m"));
            assert!(q["requirements"].as_array().unwrap().iter().any(|r|r.as_str().unwrap().contains("no immediate payment")));
            assert!(!q["service_effects"].to_string().contains("-0.000"));
            assert_eq!(spheres_sim::save(&g.world),before);
        }
        if let Some(path)=std::env::var_os("SPHERES_SUPPLY_AUTOMATION_QA_SAVE"){std::fs::write(path,&before).unwrap();}
        let market=serde_json::to_string(&g.world.resources).unwrap();let cargo=serde_json::to_string(&g.world.logistics).unwrap();let cash=g.world.nation(ID).treasury_bn;
        apply(&mut g,&command(false));
        assert_eq!(supply_automation_board(&g.world,ID)["overview"]["status"],"Manual purchases only");
        assert_eq!(serde_json::to_string(&g.world.resources).unwrap(),market);assert_eq!(serde_json::to_string(&g.world.logistics).unwrap(),cargo);assert_eq!(g.world.nation(ID).treasury_bn,cash);
    }
    #[test]
    fn purchasing_policy_parser_and_semantic_limits_fail_closed() {
        let g=super::replenishment_view_tests::fixture();let before=spheres_sim::save(&g.world);
        for key in ["automatic","horizon_days","spending_cap_mn","cash_floor_mn","review_interval_days"] {
            let mut c=command(true);c.as_object_mut().unwrap().remove(key);assert!(super::super::parse_command(&g.world,&c,ID).is_none());
            for bad in [Value::Null,json!("1")] {let mut c=command(true);c[key]=bad;assert!(super::super::parse_command(&g.world,&c,ID).is_none(),"{c}");}
        }
        for (key,bad) in [("automatic",json!(1)),("automatic",json!("false")),("horizon_days",json!(30.5)),("review_interval_days",json!(4294967296u64))] {
            let mut c=command(true);c[key]=bad;assert!(super::super::parse_command(&g.world,&c,ID).is_none());
        }
        for (key,bad) in [("horizon_days",json!(60)),("spending_cap_mn",json!(-1)),("cash_floor_mn",json!(-1)),("review_interval_days",json!(0)),("review_interval_days",json!(5))] {
            let mut c=command(true);c[key]=bad;let q=preview(&g.world,ID,&g.session_id,&json!({"command":c})).unwrap();assert_eq!(q["valid"],false,"{q}");assert_eq!(q["actions"][0]["enabled"],false);
        }
        assert_eq!(spheres_sim::save(&g.world),before);
    }
    #[test]
    fn empty_material_outlook_does_not_block_a_valid_future_policy() {
        let mut g=super::replenishment_view_tests::fixture();g.world.nation_mut(ID).equipment.as_mut().unwrap().projects.clear();
        let q=preview(&g.world,ID,&g.session_id,&json!({"command":command(true)})).unwrap();assert_eq!(q["valid"],true,"{q}");
        assert!(!q["service_effects"]["warnings"].as_array().unwrap().is_empty());
        let mut c=command(true);c["spending_cap_mn"]=json!(0);c["cash_floor_mn"]=json!(1000000);
        let q=preview(&g.world,ID,&g.session_id,&json!({"command":c})).unwrap();assert_eq!(q["valid"],true,"{q}");
        assert!(q["metrics"].as_array().unwrap().iter().any(|m|m["label"]=="Purchase limit at current cash"&&m["value"]=="$0.000000m"));
        assert!(q["service_effects"]["warnings"].as_array().unwrap().iter().any(|v|v.as_str().unwrap().contains("allows no purchase")));
    }
    #[test]
    fn dated_purchase_receipts_keep_their_original_limits_after_edit_and_clear() {
        let mut g=super::replenishment_view_tests::fixture();let c=command(true);apply(&mut g,&c);
        // Execute the fiscal staging as well as its settlement, as browser play
        // does; an unstaged invoice is deliberately ineligible for purchasing.
        spheres_sim::tick_day(&mut g.world,&[]);spheres_sim::tick_day(&mut g.world,&[]);
        let board=supply_automation_board(&g.world,ID);let history=board["reviews"].clone();
        assert_eq!(history.as_array().unwrap().len(),1);assert_eq!(history[0]["status"],"Materials purchased");
        assert!(history[0]["metrics"].as_array().unwrap().iter().any(|m|m["label"]=="Authorized maximum"&&m["value"]=="$0.100000m"));
        let cash=g.world.nation(ID).treasury_bn;let cargo=serde_json::to_string(&g.world.logistics).unwrap();
        let mut edit=command(false);edit["spending_cap_mn"]=json!(0.025);edit["horizon_days"]=json!(90);apply(&mut g,&edit);
        assert_eq!(supply_automation_board(&g.world,ID)["reviews"],history);
        let clear=json!({"kind":"equipment_supply_policy_clear"});let before=spheres_sim::save(&g.world);
        let q=preview(&g.world,ID,&g.session_id,&json!({"command":clear})).unwrap();assert_eq!(q["valid"],true,"{q}");assert_eq!(q["actions"][0]["label"],"Remove purchasing plan");assert!(q["service_effects"].is_null());assert_eq!(spheres_sim::save(&g.world),before);
        apply(&mut g,&clear);let board=supply_automation_board(&g.world,ID);assert_eq!(board["overview"]["status"],"Manual purchases only");assert_eq!(board["reviews"],history);assert_eq!(g.world.nation(ID).treasury_bn,cash);assert_eq!(serde_json::to_string(&g.world.logistics).unwrap(),cargo);
        let saved=spheres_sim::save(&g.world);let loaded=spheres_sim::load(&saved).unwrap();assert_eq!(supply_automation_board(&loaded,ID),board);
    }
}
