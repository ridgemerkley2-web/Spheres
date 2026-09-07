// Manufacturer service reviews use frozen design identity and simulation quotes.
// A refit never purchases another copy of the government's existing vehicles.
fn company_refit_model(w:&WorldState,me:NationId,revision:&str)->Option<Value> {
    w.nation(me).equipment.as_ref().and_then(|s|s.revisions.get(revision))
        .map(|r|json!({"revision":r.id,"name":r.name,"spec":r.spec}))
}

fn company_refit_models(w:&WorldState,me:NationId,source:&str,target:&str)->Value {
    json!({"source":company_refit_model(w,me,source),"target":company_refit_model(w,me,target)})
}

fn company_refit_comparison(w:&WorldState,me:NationId,source:&str,target:&str)->Value {
    let Some(r)=w.nation(me).equipment.as_ref().and_then(|s|s.revisions.get(target)) else{return json!({"available":false});};
    let mut comparison=design_comparison(w,me,&json!({"comparison_id":source}),&r.spec,Some(&r.profile));
    // The service fee is quoted separately. New-model development/tooling and
    // fabrication prices do not describe this already certified conversion.
    if let Some(rows)=comparison["rows"].as_array_mut() {
        rows.retain(|row|!matches!(row["key"].as_str(),Some("fabrication"|"development"|"tooling"|"development_days"|"production_days")));
    }
    comparison["detail"]=json!("Exact certified specifications before and after conversion. Capability changes apply only to each completed vehicle returned to service; its age is preserved. The reviewed service fee appears separately.");
    comparison
}

fn company_refit_action(w:&WorldState,me:NationId,source:&str,target:Option<&str>,requested:u32)->Option<Value> {
    let n=w.nation(me);let state=n.equipment.as_ref()?;let old=state.revisions.get(source)?;
    let available=n.arsenal.held.iter().find(|h|h.design_id.as_deref()==Some(source)).map_or(0,spheres_sim::arsenal::available_design_units).min(eq::MAX_BATCH);
    if available==0{return None;}
    for firm in w.companies.firms.iter().filter(|c|c.nation==me) {
        let options:Vec<Value>=firm.products.iter().filter(|p|p.cancelled_day.is_none()&&p.certified_day.is_some())
            .filter_map(|p|state.revisions.get(&p.revision_id).map(|r|(p,r)))
            .filter(|(_,r)|r.certified_day.is_some()&&r.id!=source&&r.spec.platform==old.spec.platform&&r.spec.components.get("armament")==old.spec.components.get("armament")&&r.spec.components!=old.spec.components)
            .filter(|(_,r)|target.is_none_or(|id|r.id==id))
            .map(|(p,r)|json!({"value":p.id,"label":format!("{} · {}",r.name,firm.name)})).collect();
        let Some(first)=options.first() else{continue;};
        let quantity=requested.max(1).min(available);
        let mut action=intent("Review manufacturer refit",json!({"kind":"company_refit","company":firm.id,"source":source,"product":first["value"],"quantity":quantity}),vec![
            json!({"key":"product","label":"Certified manufacturer upgrade","type":"select","value":first["value"],"options":options}),
            json!({"key":"quantity","label":if old.profile.aviation.is_some(){"Aircraft to withdraw for refit"}else{"Vehicles to withdraw for refit"},"type":"number","value":quantity,"min":1,"max":available,"step":1})]);
        action["detail"]=json!("Review a fixed-price service contract for vehicles you already own. The manufacturer performs the conversion in its existing plant; reserved vehicles cannot deploy until returned. Payment is held separately and earned only as each complete vehicle returns.");
        return Some(action);
    }
    None
}

fn company_service_overview()->Value {
    json!({"title":"Upgrade through your manufacturer","status":"Reviewed service contracts",
        "detail":"Choose a compatible certified upgrade for equipment you already own. The fixed service price is held for the contract. Your manufacturer buys the parts and performs the conversion, then earns its fee as each finished vehicle returns to service.",
        "metrics":[],"warnings":["Refits share the manufacturer's existing plant: development runs first, then refit contracts, then finished-stock replenishment. Downtime depends on earlier work, company capital, real inputs and access.","Cancellation releases and refunds unstarted vehicles. Work already started on a vehicle finishes under its existing contract; completed upgrades and spent materials remain."],
        "actions":[nav("Review vehicles in service",json!({"action":"equipment","tab":"service"})),nav("Review procurement funding",json!({"action":"budget","ministry":"defense","department":3}))]})
}

fn company_refit_preview(w:&WorldState,me:NationId,session:&str,command:&Value,order:&CompanyOrder)->Value {
    let quote=company_quote(w,me,order).expect("Manufacturer services have simulation quotes");
    let mut confirmed=command.clone();confirmed["quote"]=quote["token"].clone();
    let cancel=matches!(order,CompanyOrder::CancelRefit{..});
    let mut action=checked(w,me,if cancel{"Cancel unstarted vehicles"}else{"Commission manufacturer refit"},confirmed);
    let mut blockers:Vec<String>=action["reason"].as_str().map(str::to_string).into_iter().collect();
    if quote["valid"]==false {if let Some(reason)=quote["reason"].as_str(){if !blockers.iter().any(|r|r==reason){blockers.push(reason.into());}}}
    if !blockers.is_empty(){action["enabled"]=json!(false);}
    let company=company_count(&quote,"company") as u32;
    let raw=companies::view(w,me);
    let contract=if let CompanyOrder::CancelRefit{refit,..}=order {
        raw["refits"].as_array().and_then(|rows|rows.iter().find(|r|company_count(r,"id")==*refit as u64&&company_count(r,"company")==company as u64))
    }else{None};
    let source=contract.map_or(company_text(&quote,"source_revision"),|r|company_text(r,"source_revision"));
    let target=contract.map_or(company_text(&quote,"target_revision"),|r|company_text(r,"target_revision"));
    let models=company_refit_models(w,me,source,target);
    let aircraft=models["source"]["spec"]["platform"].as_str().is_some_and(eq::is_aviation_platform);
    let mut metrics=vec![metric("Manufacturer",companies::company(w,me,company).map(|c|c.name.as_str())),
        metric("Current model",models["source"]["name"].clone()),metric("Certified upgrade",models["target"]["name"].clone())];
    let mut costs=vec![];let mut timing=vec![];
    let mut requirements=vec!["These are your existing vehicles. They remain government-owned, retain their age, and cannot fight or be retired while reserved. The manufacturer does not sell them back as stock.".to_string(),
        "A completed refit changes only the returned vehicle's exact specifications. Compatible ammunition, maintenance and aircraft basing remain necessary; the contract grants no ammunition or new vehicles.".into()];
    let mut effects=vec![];
    if let CompanyOrder::Refit{quantity,..}=order {
        metrics.push(metric(if aircraft{"Aircraft to withdraw"}else{"Vehicles to withdraw"},quantity));
        metrics.push(metric("Unreserved source equipment",company_count(&quote,"available_units")));
        metrics.push(metric("Source equipment already reserved",company_count(&quote,"reserved_units")));
        costs.push(cost("Fixed contract price",company_amount(&quote,"cost_bn"),"one advance · Defense procurement"));
        costs.push(cost(if aircraft{"Fixed service price per aircraft"}else{"Fixed service price per vehicle"},company_amount(&quote,"unit_price_bn"),"parts, conversion work and manufacturer margin included"));
        costs.push(cost("Company capital needed for the next vehicle",company_amount(&quote,"company_cash_needed_bn"),"manufacturer funds inputs and reserves all conversion labor"));
        costs.push(cost("Company cash available",company_amount(&quote,"company_available_cash_bn"),"excludes customer escrow and reserved labor funds"));
        costs.push(cost("Maintenance before refit",company_amount(&quote,"maintenance_bn_day_before"),"all quoted vehicles / day · continues during withdrawal"));
        costs.push(cost("Maintenance after conversion",company_amount(&quote,"maintenance_bn_day_after"),"all quoted vehicles / day · as each returns"));
        timing.push(metric("Conversion work per vehicle",format!("{} days",company_count(&quote,"unit_days"))));
        timing.push(metric("Total minimum conversion work",format!("{} days",company_count(&quote,"minimum_days"))));
        timing.push(metric("First return estimate",quote["first_return_days"].as_u64().map(|d|format!("About {d} days with capital, inputs and access")).unwrap_or("No dated estimate · review company capital and earlier work".into())));
        timing.push(metric("All vehicles returned",quote["eta_days"].as_u64().map(|d|format!("About {d} days if current conditions hold")).unwrap_or("No dated estimate · shared plant and real inputs required".into())));
        requirements.push("Your advance is held for this contract after fiscal settlement. Only the fee for each completed, returned vehicle becomes company revenue. It cannot finance unrelated company stock while held.".into());
        requirements.push("This contract uses the existing company plant. Development runs first, then refit contracts, then equipment and ammunition restocking. Starting a refit can delay stock replenishment.".into());
        requirements.push("Cancel only unstarted vehicles for a refund of their fixed service price. Once work starts on a vehicle, its inputs and full labor funding are committed and its conversion continues.".into());
        if blockers.is_empty() {
            let mut after=w.clone();
            if super::parse_command(w,&action["command"],me).and_then(|c|spheres_sim::apply_command(&mut after,&c).ok()).is_some() {
                let source_available=|world:&WorldState|world.nation(me).arsenal.held.iter().find(|h|h.design_id.as_deref()==Some(source)).map_or(0,spheres_sim::arsenal::available_design_units);
                effects.push(metric("Source vehicles available now",format!("{} → {}",source_available(w),source_available(&after))));
                effects.push(metric("Current maintenance requirement",format!("{} → {}",service_money(eq::fleet_maintenance_requirement(w.nation(me))),service_money(eq::fleet_maintenance_requirement(after.nation(me))))));
                effects.extend(service_role_rows(w.nation(me),Some(after.nation(me))));
                for (revision,label) in [(source,"Source fleet"),(target,"Upgraded fleet")] {
                    let before=eq::fleet_target_plans_world(w,me).into_iter().find(|p|p.revision==revision);
                    let future=eq::fleet_target_plans_world(&after,me).into_iter().find(|p|p.revision==revision);
                    if let (Some(before),Some(future))=(before,future) {
                        if before.desired.is_some(){effects.push(metric(&format!("{label} shortfall after committed work"),format!("{} → {}",before.shortfall,future.shortfall)));}
                    }
                }
            }
        }
    }else{
        metrics.push(metric("Unstarted vehicles released",company_count(&quote,"refundable_units")));
        metrics.push(metric("Started vehicles still in conversion",company_count(&quote,"active_units")));
        metrics.push(metric("Completed upgrades retained",company_count(&quote,"completed_units")));
        costs.push(cost("Refund to treasury",company_amount(&quote,"refund_bn"),"fixed service price for unstarted vehicles only"));
        timing.push(metric("Refund timing",if quote["refund_after_settlement"]==true{"After the original advance settles"}else{"On cancellation"}));
        requirements.push("Only untouched vehicles return to availability now. Any started vehicle remains withdrawn until its conversion finishes using the already reserved labor funds. Completed conversions stay upgraded.".into());
        requirements.push("The refund returns treasury cash or reduces debt once. It does not renew expired budget authority or restore already-used procurement appropriations. A payment awaiting fiscal settlement must settle before its refund can be released.".into());
    }
    if let Some(note)=quote["note"].as_str(){requirements.push(note.into());}
    json!({"session_id":session,"nation":me,"valid":blockers.is_empty(),"blockers":blockers,"metrics":metrics,"costs":costs,"timing":timing,"requirements":requirements,
        "refit":models,"comparison":company_refit_comparison(w,me,source,target),
        "service_effects":if effects.is_empty(){Value::Null}else{json!({"title":"What withdrawal changes now","detail":"The source vehicles leave deployable service immediately. Upgraded capability returns one vehicle at a time as conversion finishes. Reserved vehicles continue to age and require maintenance.","metrics":effects,"warnings":[]})},
        "actions":[action,nav("Review manufacturer and service contracts",json!({"action":"equipment","tab":"companies"})),nav("Review fleet in service",json!({"action":"equipment","tab":"service"}))],
        "detail":if cancel{"Release and refund only the unstarted part of this service contract. Review the vehicles that remain in conversion before confirming."}else{"Commission a fixed-price conversion of equipment you already own. Review the specifications, temporary loss of availability and company work requirements before paying the advance."}})
}

fn company_refit_rows(w:&WorldState,me:NationId,raw:&Value)->Vec<Value> {
    let empty=vec![];
    raw["refits"].as_array().unwrap_or(&empty).iter().rev().map(|r|{
        let id=company_count(r,"id");let company=company_count(r,"company");
        let source=company_text(r,"source_revision");let target=company_text(r,"target_revision");
        let models=company_refit_models(w,me,source,target);
        let firm=companies::company(w,me,company as u32);
        let quantity=company_count(r,"quantity");let completed=company_count(r,"completed_units");let cancelled=company_count(r,"cancelled_units");
        let remaining=company_count(r,"remaining_units");let active=company_count(r,"active_units");let refundable=company_count(r,"refundable_units");
        let unit_days=company_amount(r,"unit_days");let work=company_amount(r,"unit_work_days");
        let status=if company_amount(r,"refund_pending_bn")>0.0 {"Cancelled · refund awaiting settlement"}else if remaining==0 {if cancelled==quantity{"Cancelled before work"}else if cancelled>0{"Completed · unstarted vehicles cancelled"}else{"Completed and returned"}}
            else if r["settled_day"].is_null(){"Awaiting payment settlement"}else if r["status"]=="blocked"{"Needs attention"}else if cancelled>0&&active>0{"Finishing the started vehicle"}else if active>0{"Conversion underway"}else{"Waiting for company work"};
        let mut actions=vec![];
        if refundable>0 {actions.push(intent("Review cancellation of unstarted vehicles",json!({"kind":"company_refit_cancel","company":company,"refit":id}),vec![]));}
        actions.push(nav("Review vehicles in service",json!({"action":"equipment","tab":"service"})));
        let eta=if remaining==0{"No remaining conversion".into()}else{r["eta_days"].as_u64().map(|days|format!("About {days} days at current conditions")).unwrap_or("No dated estimate · review earlier work, company capital, materials and access".into())};
        json!({"id":format!("refit:{id}"),"refit_id":id,"company":company,"name":format!("{} → {}",models["source"]["name"].as_str().unwrap_or(source),models["target"]["name"].as_str().unwrap_or(target)),
            "supplier_name":firm.map(|c|c.name.as_str()),"family":r["family"],"unit_label":r["unit_label"],"refit":models,"status":status,"phase":if remaining==0{"complete"}else if active>0{"manufacturing"}else{"queued"},"detail":r["reason"],
            "progress":if quantity>cancelled&&unit_days>0.0{Some(((completed as f64*unit_days+work)/((quantity-cancelled) as f64*unit_days)).clamp(0.0,1.0))}else{None},
            "metrics":[metric("Manufacturer",firm.map(|c|c.name.as_str())),metric("Original contract quantity",quantity),metric("Converted and returned",completed),metric("Still withdrawn",remaining),
                metric("In conversion",active),metric("Unstarted and refundable",refundable),metric("Cancelled and released",cancelled),metric("Return estimate",eta),
                metric("Production site",firm.map(|c|spheres_sim::districts::name_of(&c.district).unwrap_or(&c.district)))],
            "costs":[cost("Fixed contract price",company_amount(r,"total_price_bn"),"original advance · Defense procurement"),cost("Payment held for unfinished work",company_amount(r,"escrow_bn"),"separate from company spending cash"),
                cost("Company fee earned",company_amount(r,"earned_revenue_bn"),"only completed, returned conversions"),cost("Refund paid",company_amount(r,"refunded_bn"),"unstarted vehicles only"),cost("Refund awaiting settlement",company_amount(r,"refund_pending_bn"),"released only after the original advance settles"),
                cost("Labor funding reserved",company_amount(r,"working_capital_locked_bn"),"company-owned funds for the started vehicle"),cost("Additional company capital to continue",company_amount(r,"company_cash_needed_bn"),"started-unit costs already reserved; otherwise parts and full conversion labor")],
            "milestones":[{"label":"Government ownership","value":format!("{remaining} reserved"),"detail":"Already-owned equipment; no new stock purchase"},
                {"label":"Current conversion","value":format!("{work:.1} / {unit_days:.0} work days"),"detail":"One company plant packet, shared with existing work"},
                {"label":"Returned to service","value":format!("{completed} converted"),"detail":"Exact target revision; age preserved"}],
            "requirements":["A started vehicle completes under this contract even when unstarted vehicles are cancelled. Refunds do not renew procurement authority.","Development takes priority over refits. Refit contracts precede equipment and ammunition stock replenishment on the same leased plant."],"actions":actions})
    }).collect()
}

#[cfg(test)]
mod company_refit_view_tests {
    use super::*;
    const ME:NationId=NationId::France;
    const SOURCE:&str="ammo-demo";
    fn review(g:&super::super::Game,c:Value)->Value {
        let q=preview(&g.world,ME,&g.session_id,&json!({"command":c})).unwrap();assert_eq!(q["valid"],true,"{q}");q
    }
    fn apply(g:&mut super::super::Game,c:&Value) {
        let parsed=super::super::parse_command(&g.world,c,ME).unwrap();spheres_sim::apply_command(&mut g.world,&parsed).unwrap();
    }
    fn day(w:&mut WorldState) {
        if w.nation(ME).program_budget.as_ref().unwrap().fiscal_year!=w.year {
            let departments=w.nation(ME).program_budget.as_ref().unwrap().departments;let allocations=w.nation(ME).budget_for(w.year).allocations;
            spheres_sim::apply_command(w,&Command::SetProgramBudget{nation:ME,fiscal_year:w.year,allocations,departments}).unwrap();
        }
        spheres_sim::programs::begin_day(w);eq::tick_day(w);companies::tick_day(w);
        let p=w.nation_mut(ME).program_budget.as_mut().unwrap();p.revenue_today_bn=0.0;p.interest_today_bn=0.0;p.fiscal_staged=true;
        spheres_sim::programs::finish_day(w);companies::settle_receivables(w);spheres_sim::clock::advance_date(w);
    }
    fn ready()->(super::super::Game,u32,u32,String) {
        // Synthetic source vehicles, cash, raw materials and one factory make
        // the interface fixture explicit. The target license is actually earned
        // through a reviewed development command and daily company work.
        let mut g=super::ammunition_view_tests::fixture();
        let site=g.world.production.provinces.iter_mut().find(|p|p.arms_plants>0).unwrap();site.arms_plants=1;let district=site.district.clone();
        let n=g.world.nation_mut(ME);n.political_capital=1000.0;
        let p=n.program_budget.as_mut().unwrap();p.available_bn[BUDGET_DEFENSE][3]=2.0;p.available_bn[BUDGET_DEFENSE][4]=2.0;
        let q=review(&g,json!({"kind":"company_establish","name":"Interface Refit Works","district":district,"capital_mn":25.0}));apply(&mut g,&q["actions"][0]["command"]);
        let company=g.world.companies.firms[0].id;day(&mut g.world);
        let mut spec=eq::default_spec("ground_ifv");spec.components.insert("sensors".into(),"optics_night".into());
        let q=review(&g,json!({"kind":"company_develop","company":company,"name":"Night observation IFV","platform":spec.platform,"components":spec.components,"daily_budget_mn":10.0,"stock_target":1}));apply(&mut g,&q["actions"][0]["command"]);
        let product=g.world.companies.firms[0].products[0].id;let target=g.world.companies.firms[0].products[0].revision_id.clone();
        apply(&mut g,&json!({"kind":"company_inventory","company":company,"product":product,"stock_target":0}));
        for _ in 0..1500 {if g.world.companies.firms[0].products[0].certified_day.is_some(){break;}day(&mut g.world);}
        assert!(g.world.companies.firms[0].products[0].certified_day.is_some());
        g.world.nation_mut(ME).program_budget.as_mut().unwrap().available_bn[BUDGET_DEFENSE][3]=1.0;
        (g,company,product,target)
    }
    #[test]
    fn manufacturer_refit_review_is_pure_and_compares_exact_upgrade_without_new_model_costs(){
        let (g,company,product,target)=ready();let before=spheres_sim::save(&g.world);
        let q=review(&g,json!({"kind":"company_refit","company":company,"source":SOURCE,"product":product,"quantity":2}));
        assert_eq!(q["refit"]["source"]["revision"],SOURCE);assert_eq!(q["refit"]["target"]["revision"],target);
        assert_eq!(q["refit"]["target"]["spec"]["components"]["sensors"],"optics_night");
        assert!(q["comparison"]["rows"].as_array().unwrap().iter().any(|r|r["key"]=="observation"&&r["delta"].as_f64().unwrap()>0.0));
        assert!(!q["comparison"]["rows"].as_array().unwrap().iter().any(|r|r["key"]=="development"||r["key"]=="fabrication"));
        let cost=q["costs"].as_array().unwrap().iter().find(|r|r["label"]=="Fixed contract price").unwrap();
        assert_eq!(cost["amount_bn"],companies::refit_quote(&g.world,ME,company,SOURCE,product,2).cost_bn);
        let upkeep=q["costs"].as_array().unwrap().iter().find(|r|r["label"]=="Maintenance before refit").unwrap();
        assert_eq!(upkeep["amount_bn"],eq::profile(g.world.nation(ME),SOURCE).unwrap().maintenance_bn_day*2.0);
        assert!(q["service_effects"]["metrics"].as_array().unwrap().iter().any(|m|m["label"]=="Source vehicles available now"&&m["value"]=="10 → 8"));
        let lots=&view(&g.world,ME,&g.session_id)["lots"];
        assert_eq!(lots[0]["actions"][0]["command"]["kind"],"company_refit");assert_eq!(lots[0]["actions"][0]["command"]["product"],product);
        assert!(!lots[0]["actions"].as_array().unwrap().iter().any(|a|a["command"]["kind"]=="equipment_refit"));
        assert_eq!(spheres_sim::save(&g.world),before);
    }
    #[test]
    fn refit_booking_replay_preserves_one_deposit_reservation_and_target_projection(){
        let (mut g,company,product,target)=ready();
        apply(&mut g,&json!({"kind":"equipment_target","revision":target,"quantity":2}));
        let q=review(&g,json!({"kind":"company_refit","company":company,"source":SOURCE,"product":product,"quantity":2}));
        let request=json!({"session_id":g.session_id,"client_id":"company-refit-api","request_seq":1,"commands":[q["actions"][0]["command"]]});
        let result=super::super::transport::immediate_request(&mut g,&request).unwrap();assert_eq!(result["errors"],json!([]));
        let before=spheres_sim::save(&g.world);let retry=super::super::transport::immediate_request(&mut g,&request).unwrap();assert_eq!(retry["command_replayed"],true);assert_eq!(spheres_sim::save(&g.world),before);
        let board=company_board(&g.world,ME);assert_eq!(board["services"].as_array().unwrap().len(),1);assert_eq!(board["services"][0]["refit"]["target"]["revision"],target);
        assert_eq!(board["services"][0]["status"],"Awaiting payment settlement");
        assert_eq!(g.world.nation(ME).arsenal.held[0].units,10.0);assert_eq!(g.world.nation(ME).arsenal.held[0].refit_reserved,2);
        assert_eq!(eq::fleet_target_plans_world(&g.world,ME).into_iter().find(|p|p.revision==target).unwrap().shortfall,0);
        assert!(g.world.companies.firms[0].products.iter().all(|p|p.stock==0));
    }
    #[test]
    fn cancellation_review_and_card_explain_refund_pending_original_settlement(){
        let (mut g,company,product,_)=ready();
        let q=review(&g,json!({"kind":"company_refit","company":company,"source":SOURCE,"product":product,"quantity":2}));apply(&mut g,&q["actions"][0]["command"]);
        let board=company_board(&g.world,ME);let c=board["services"][0]["actions"][0]["command"].clone();
        let before=spheres_sim::save(&g.world);let q=review(&g,c);assert_eq!(q["actions"][0]["label"],"Cancel unstarted vehicles");
        assert!(q["timing"].as_array().unwrap().iter().any(|t|t["value"]=="After the original advance settles"));assert_eq!(spheres_sim::save(&g.world),before);
        apply(&mut g,&q["actions"][0]["command"]);
        let board=company_board(&g.world,ME);assert_eq!(board["services"][0]["status"],"Cancelled · refund awaiting settlement");
        assert!(board["services"][0]["costs"].as_array().unwrap().iter().any(|c|c["label"]=="Refund awaiting settlement"&&c["amount_bn"].as_f64().unwrap()>0.0));
        assert_eq!(g.world.nation(ME).arsenal.held[0].refit_reserved,0);
    }
    #[test]
    fn refit_parser_is_strict_and_keeps_player_authority(){
        let g=super::ammunition_view_tests::fixture();let before=spheres_sim::save(&g.world);
        let mut c=json!({"kind":"company_refit","company":1,"source":SOURCE,"product":2,"quantity":1,"quote":"review-token","nation":"USA"});
        assert!(matches!(super::super::parse_command(&g.world,&c,ME),Some(Command::Company{nation:ME,order:CompanyOrder::Refit{quote,..}}) if quote=="review-token"));
        for key in ["company","product","quantity"] {for value in [json!(-1),json!(0.5),json!(4294967296u64),json!("1")] {let mut bad=c.clone();bad[key]=value;assert!(super::super::parse_command(&g.world,&bad,ME).is_none());}}
        c["kind"]=json!("company_refit_cancel");c["refit"]=json!(3);assert!(super::super::parse_command(&g.world,&c,ME).is_some());
        c["refit"]=json!(1.5);assert!(super::super::parse_command(&g.world,&c,ME).is_none());assert_eq!(spheres_sim::save(&g.world),before);
    }
}
