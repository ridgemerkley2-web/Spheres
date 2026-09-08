fn maintenance_action(w:&WorldState,me:NationId)->Value {
    let n=w.nation(me);let plan=n.equipment.as_ref().and_then(|s|s.maintenance_plan.as_ref());
    let required=eq::fleet_maintenance_requirement(n)+eq::legacy_maintenance_requirement_world(w,me);
    let limit_mn=plan.map_or_else(||(required*1_000_000.0).ceil()/1000.0,|p|p.daily_limit_bn*1000.0);
    let mut action=intent(if plan.is_some(){"Review maintenance limit"}else{"Review actual maintenance plan"},json!({"kind":"equipment_maintenance","daily_budget_mn":limit_mn}),vec![json!({"key":"daily_budget_mn","label":"Maximum maintenance payment","unit":"$m / day","type":"number","value":limit_mn,"min":0,"max":1_000_000,"step":0.001})]);
    action["detail"]=json!("Use Defense maintenance to pay actual fleet work, up to this daily ceiling. Unused authorization remains in that department. A limit below the bill reduces support for both inherited and custom equipment.");
    action
}
fn maintenance_board(w:&WorldState,me:NationId)->Value {
    let n=w.nation(me);let plan=n.equipment.as_ref().and_then(|s|s.maintenance_plan.as_ref());
    let custom=eq::fleet_maintenance_requirement(n);let legacy=eq::legacy_maintenance_requirement_world(w,me);
    let mut metrics=vec![metric("Current custom maintenance need",service_money(custom)),metric("Current inherited maintenance need",service_money(legacy)),metric("Total physical fleet requirement",service_money(custom+legacy))];
    if spheres_sim::industry_operations::enabled(w) {
        let repair=eq::shipyard_repair_support(w,me);
        metrics.push(metric("Operating shipyards",format!("{:.2}",repair.operating_shipyards)));
        metrics.push(metric("Naval repair invoice reduction",format!("{:.1}%",repair.naval_reduction_fraction*100.0)));
        metrics.push(metric("Daily naval repair savings",service_money(repair.savings_daily_bn)));
    }
    let mut warnings=vec![];let mut status="Allocation-based support";
    if let Some(plan)=plan {
        metrics.push(metric("Daily payment ceiling",service_money(plan.daily_limit_bn)));
        status=if eq::maintenance_plan_on(n,spheres_sim::clock::absolute_day(w)){"Actual maintenance plan"}else{"Starts next funding day"};
        if let Some(r)=&plan.receipt {
            let paid=r.custom_paid_bn+r.legacy_paid_bn;let bill=r.custom_required_bn+r.legacy_required_bn;
            metrics.extend([metric("Last invoice date",super::settled_day_json(r.day)["label"].clone()),metric("Last invoice required",service_money(bill)),metric("Last invoice paid",service_money(paid)),metric("Last invoice unfunded",service_money((bill-paid).max(0.0))),metric("Fleet support from maintenance",format!("{:.1}%",if bill>0.0{paid/bill*100.0}else{100.0}))]);
            if paid+1e-12<bill {
                status="Maintenance limits readiness";
                if r.daily_limit_bn+1e-12<bill {warnings.push("The daily maintenance ceiling was below the invoice. Review the limit to support more of the fleet.".to_string());}
                if r.authority_bn+1e-12<bill {warnings.push("Available Defense maintenance funds were below the invoice. Review the departmental allocation and fiscal-year renewal.".to_string());}
            }
        }else{warnings.push("No actual invoice has settled yet. The first payment uses the eligible funding day after confirmation.".to_string());}
    }else{warnings.push("The existing allocation-based system is still active. Review an actual maintenance plan to replace automatic maintenance spending on future funding days.".to_string());}
    let mut readiness=vec![];
    let available: f64=n.arsenal.held.iter().map(|h|if h.design_id.is_some(){spheres_sim::arsenal::available_design_units(h) as f64}else{h.units}).sum();
    let condition:f64=n.arsenal.held.iter().map(|h|if h.design_id.is_some(){spheres_sim::arsenal::available_design_units(h) as f64*spheres_sim::arsenal::holding_condition(n,h)}else{h.units*spheres_sim::arsenal::holding_condition(n,h)}).sum();
    if available>0.0 {
        readiness.push(json!({"label":"Physical condition","value":format!("{:.1}% weighted condition of available equipment",condition/available*100.0),"detail":"Age reduces capability separately from maintenance funding. Review modernization or replacement targets for worn equipment."}));
    }
    let physical_ammo=eq::ammunition_active(n,spheres_sim::clock::absolute_day(w));
    readiness.push(json!({"label":if physical_ammo{"Inherited magazine readiness"}else{"Shared ammunition readiness"},"value":format!("{:.1}%",n.munitions*100.0),"detail":if physical_ammo{"This magazine supplies only the inherited share. Custom ground firing uses compatible physical stores; review the Ammunition tab for stock and consumption."}else{"This is the existing national magazine, shared by operations. Maintenance supports its refill; paying for vehicle manufacture does not refill it."}}));
    if n.munitions<0.5 {warnings.push("The shared ammunition magazine is below half capacity. Sustained operations can be constrained while it recovers.".to_string());}
    let reserved:u64=n.arsenal.held.iter().map(|h|h.refit_reserved as u64).sum();
    readiness.push(json!({"label":"Refit withdrawal","value":format!("{reserved} custom vehicles unavailable"),"detail":"Reserved vehicles return when their conversions finish, or when remaining refit work is cancelled."}));
    json!({"title":"Maintenance and readiness","status":status,"detail":"Actual maintenance bills cover delivered inherited and custom equipment, including refit reservations. Incoming orders are charged after delivery. A funding shortfall reduces supported capability; equipment age, refits and ammunition remain separate constraints.",
        "metrics":metrics,"roles":readiness,"roles_title":"What affects readiness","warnings":warnings,"actions":[maintenance_action(w,me),nav("Review Defense maintenance funding",json!({"action":"budget","ministry":"defense","department":2}))]})
}
fn maintenance_preview(w:&WorldState,me:NationId,session:&str,command:&Value,limit:f64)->Value {
    let n=w.nation(me);let custom=eq::fleet_maintenance_requirement(n);let legacy=eq::legacy_maintenance_requirement_world(w,me);let required=custom+legacy;
    let action=checked(w,me,"Confirm maintenance plan",command.clone());
    let blockers:Vec<_>=action["reason"].as_str().map(str::to_string).into_iter().collect();
    let p=n.equipment.as_ref().and_then(|s|s.maintenance_plan.as_ref());
    json!({"session_id":session,"nation":me,"valid":blockers.is_empty(),"blockers":blockers,"metrics":[metric("Proposed daily payment ceiling",service_money(limit)),metric("Inherited equipment requirement",service_money(legacy)),metric("Custom equipment requirement",service_money(custom)),metric("Total current requirement",service_money(required)),metric("Coverage permitted by this ceiling",format!("{:.1}% before funding constraints",if required>0.0{(limit/required).min(1.0)*100.0}else{100.0}))],
        "costs":[],"timing":[{"label":"Effective","value":if p.is_some(){"Next unsettled invoice"}else{"Next funding day after confirmation"}}],
        "requirements":["This ceiling uses the existing Defense maintenance allocation. No cash is paid merely by setting the plan; actual daily invoices use the shared fiscal ledger once. Unused authorization stays available until normal fiscal expiry.","Invoices are allocated proportionally across inherited and custom equipment when the ceiling or funding is insufficient. More money cannot restore age-related condition or deploy refit reservations.","Inherited equipment upkeep is a game assumption of 4% of its catalog purchase value per year, spread over 365 days. In rebuilt industry, each staffed and supplied shipyard reduces its naval portion by 5%, up to 25%. This quote uses current shipyard support; an electricity, workforce or operating-funding shortage can change the next invoice. Custom revisions use their frozen per-vehicle daily requirement.","Activating this plan replaces the automatic maintenance allocation expense prospectively. Existing receipts and other departments remain unchanged; old campaign saves retain their previous support behavior until activation."],
        "actions":[action],"detail":"Review the expected fleet bill and your daily limit. Actual payment can be lower when departmental funding is insufficient; there is no loan or immediate spending from this confirmation."})
}

#[cfg(test)]
mod maintenance_view_tests {
    use super::*;
    fn fixture()->super::super::Game {let mut g=super::super::Game::new(1990,Some(NationId::USA));super::super::play_rules(&mut g);spheres_sim::programs::set_construction_budget(&mut g.world,NationId::USA,0.0).unwrap();g}
    #[test]
    fn maintenance_review_is_pure_and_confirmed_limit_starts_a_dated_invoice() {
        let mut g=fixture();let before=spheres_sim::save(&g.world);
        let command=json!({"kind":"equipment_maintenance","daily_budget_mn":0.001});
        let quote=preview(&g.world,NationId::USA,&g.session_id,&json!({"command":command})).unwrap();assert_eq!(quote["valid"],true);assert_eq!(quote["actions"][0]["label"],"Confirm maintenance plan");
        assert_eq!(spheres_sim::save(&g.world),before);assert!(g.world.nation(NationId::USA).equipment.is_none());
        let order=super::super::parse_command(&g.world,&quote["actions"][0]["command"],NationId::USA).unwrap();spheres_sim::apply_command(&mut g.world,&order).unwrap();
        assert_eq!(maintenance_board(&g.world,NationId::USA)["status"],"Starts next funding day");
        spheres_sim::clock::advance_date(&mut g.world);spheres_sim::programs::begin_day(&mut g.world);eq::settle_support(&mut g.world);
        let reading=maintenance_board(&g.world,NationId::USA);assert_eq!(reading["status"],"Maintenance limits readiness");
        assert!(reading["metrics"].as_array().unwrap().iter().any(|row|row["label"]=="Last invoice paid"&&row["value"]=="$1.000k / day"));
        assert!(reading["warnings"].as_array().unwrap().iter().any(|text|text.as_str().unwrap().contains("ceiling")));
        assert_eq!(reading["roles_title"],"What affects readiness");let saved=spheres_sim::save(&g.world);view(&g.world,NationId::USA,&g.session_id);assert_eq!(spheres_sim::save(&g.world),saved);
    }
    #[test]
    fn malformed_maintenance_values_are_refused_and_unconfigured_campaign_remains_legacy() {
        let g=fixture();let before=spheres_sim::save(&g.world);let board=maintenance_board(&g.world,NationId::USA);assert_eq!(board["status"],"Allocation-based support");
        for value in [json!(null),json!("1"),json!(-1),json!(1e12)] {
            let c=json!({"kind":"equipment_maintenance","daily_budget_mn":value});
            let q=preview(&g.world,NationId::USA,&g.session_id,&json!({"command":c}));assert!(q.is_err()||q.unwrap()["valid"]==false);
        }
        assert_eq!(spheres_sim::save(&g.world),before);
    }
    #[test]
    fn shipyard_repair_review_uses_the_exact_invoice_requirement() {
        let mut g=fixture();
        let nation=NationId::USA;
        let district=g.world.districts.iter().find(|(d,n)|**n==nation&&spheres_sim::logistics::has_terminal(d)).unwrap().0.clone();
        g.world.production.rebuild_sites.insert(district.clone(),[0,1,0]);
        g.world.production.provinces.push(spheres_sim::production::ProvinceCapabilities {
            district:district.clone(),infrastructure:0,civilian_industry:0,power_grid:1,research_centers:0,arms_plants:0,
        });
        g.world.production.provinces.sort_by(|a,b|a.district.cmp(&b.district));
        let generator=spheres_sim::industry::EXTENDED.iter().position(|k|*k==spheres_sim::production::ProjectKind::Generation).unwrap();
        g.world.production.industry.sites.entry(district).or_default()[generator]=1;
        // Seed the operating fuel explicitly: Game::new has no inherited
        // physical stock rows until campaign setup/production materializes them.
        g.world.resources.market=Some(spheres_sim::resources::MarketState {
            prices:[1000.0;12],previous_prices:[1000.0;12],cleared_volume:[0.0;12],unmet_orders:[0.0;12],
            fills:vec![],contract_fills:vec![],shipment_audits:vec![],contract_spend_bn:vec![],
            stocks:vec![spheres_sim::resources::Stock {nation,commodity:spheres_sim::resources::Commodity::Coal,quantity:1000.0,reserve_target:0.0}],
            cash:vec![],last_produced:-1,last_cleared:-1,last_produced_day:None,last_cleared_day:None,period_days:None,
        });
        // The review must use an actual support receipt from a funded date.
        spheres_sim::programs::begin_day(&mut g.world);
        spheres_sim::industry_operations::begin_day(&mut g.world);
        let support=eq::shipyard_repair_support(&g.world,nation);
        assert!(support.savings_daily_bn>0.0,"a staffed, supplied shipyard must lower the reviewed naval invoice: {:?}",g.world.production.operations.receipts);
        let expected=eq::legacy_maintenance_requirement_world(&g.world,nation);
        let before=spheres_sim::save(&g.world);
        let command=json!({"kind":"equipment_maintenance","daily_budget_mn":1000.0});
        let quote=preview(&g.world,nation,&g.session_id,&json!({"command":command})).unwrap();
        assert!(quote["metrics"].as_array().unwrap().iter().any(|m|m["label"]=="Inherited equipment requirement"&&m["value"]==service_money(expected)));
        let board=maintenance_board(&g.world,nation);
        assert!(board["metrics"].as_array().unwrap().iter().any(|m|m["label"]=="Daily naval repair savings"&&m["value"]==service_money(support.savings_daily_bn)));
        assert_eq!(spheres_sim::save(&g.world),before);
    }
}
