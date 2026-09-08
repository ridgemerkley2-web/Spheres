// The sim owns target quantities, prices, payment and freight. These values
// are already formatted for the view; the browser never computes a purchase.
fn replenishment_money(bn:f64)->String { format!("${:.6}m",bn*1000.0) }
fn replenishment_board(w:&WorldState,me:NationId)->Value {
    let cash=spheres_sim::resources::raw_purchase_cash(w,me);
    let mut action=intent("Review material purchase",json!({"kind":"equipment_supply","horizon_days":30,"spending_cap_mn":1.0}),vec![
        json!({"key":"horizon_days","label":"Supply horizon","type":"select","value":30,"options":[{"value":30,"label":"30 days"},{"value":90,"label":"90 days"},{"value":365,"label":"365 days"}]}),
        json!({"key":"spending_cap_mn","label":"Maximum purchase spending ($m)","type":"number","value":1.0,"min":0,"max":1000000,"step":0.1}),
    ]);
    action["detail"]=json!("Preview exact materials, suppliers, payment and estimated arrival before buying. This is a one-time cap, not a recurring budget.");
    json!({"title":"Purchase missing materials","status":"Review before purchase",
        "detail":"Build a finite purchase for funded vehicle, refit and ammunition production shortages. The plan counts shared stock, domestic supply, contracts and already-paid cargo first. It spends only available cash within your limit, then sends real shipments through the resource market. Paused programmes do not request purchases.",
        "metrics":[metric("Cash available for a reviewed purchase",replenishment_money(cash)),metric("Purchase repeats","Only when you review and confirm again")],
        "warnings":["This cap authorizes a material purchase only. It does not raise programme funding, reserve all future inputs, or guarantee an arrival date."],
        "actions":[action,nav("Inspect suppliers and cargo",json!({"action":"resources"}))]})
}
fn replenishment_preview(w:&WorldState,me:NationId,session:&str,command:&Value,horizon_days:u32,spending_cap_bn:f64)->Value {
    let q=eq::replenishment_quote(w,me,horizon_days,spending_cap_bn);
    let mut action=checked(w,me,"Confirm material purchase",command.clone());
    let mut blockers:Vec<String>=q.reason.clone().into_iter().collect();
    if let Some(reason)=action["reason"].as_str(){if !blockers.iter().any(|b|b==reason){blockers.push(reason.into());}}
    let valid=q.valid&&blockers.is_empty();action["enabled"]=json!(valid);
    let mut rows=vec![];
    for c in spheres_sim::resources::ALL {let i=c.idx();if q.equipment_demand[i]<=1e-9&&q.requested[i]<=1e-9{continue;}
        rows.push(json!({"label":c.name(),"value":format!("{:.3} {} requested · {:.3} purchased · {:.3} left",q.requested[i],c.unit(),q.receipt.purchased[i],(q.requested[i]-q.receipt.purchased[i]).max(0.0))}));}
    for fill in &q.receipt.fills {rows.push(json!({"label":format!("{} from {}",fill.commodity.name(),fill.seller.name()),"value":format!("{:.3} {} · {} · {}",fill.quantity,fill.commodity.unit(),replenishment_money(fill.cost_bn),if fill.arrival_days==0{"Immediate warehouse delivery".to_string()}else{format!("estimated {} days in transit",fill.arrival_days)})}));}
    let mut warnings=q.receipt.warnings.clone();
    warnings.push("The displayed payment is the full current spot charge. This model has no additional freight invoice; physical routes and shared transport capacity still apply.".into());
    warnings.push("Paid cargo is not usable warehouse stock until arrival. Route closures may delay it. Purchases are shared national materials, not an exclusive equipment reservation.".into());
    warnings.push("Confirmation recalculates the bundle against live supply and cash. The chosen cap is a maximum; no loan, recurring order or unused-cap charge is created.".into());
    let effect=json!({"title":"Material purchase review","status":if valid{"Ready for your confirmation"}else{"No purchase available"},
        "detail":q.note,"metrics":[metric("Planning horizon",format!("{horizon_days} days")),metric("Maximum authorized payment",replenishment_money(spending_cap_bn)),metric("Actual purchase payment",replenishment_money(q.receipt.spent_bn)),metric("Shipments",q.receipt.fills.len())],"roles_title":"Materials and suppliers","roles":rows,"warnings":warnings});
    json!({"session_id":session,"nation":me,"valid":valid,"blockers":blockers,"metrics":[],"costs":[],"timing":[],"requirements":[],"service_effects":effect,"actions":[action],
        "detail":"Review the exact purchase and remaining shortfall before confirming. Opening this quote changes no stock, cash, freight or programme."})
}

#[cfg(test)]
mod replenishment_view_tests {
    use super::*;
    fn command()->Value{json!({"kind":"equipment_supply","horizon_days":30,"spending_cap_mn":1.0})}
    pub(super) fn fixture()->super::super::Game{
        let mut g=super::super::Game::new(1990,Some(NationId::France));super::super::play_rules(&mut g);
        let id=NationId::France;spheres_sim::programs::set_construction_budget(&mut g.world,id,0.0).unwrap();
        let spec=eq::default_spec("ground_ifv");let mut p=eq::design_preview(&g.world,id,&spec).profile.unwrap();
        // Synthetic frozen fixture makes shortages deterministic for browser QA.
        p.recipe=[0.0;12];p.recipe[spheres_sim::resources::Commodity::Copper.idx()]=1_000_000.0;
        let day=spheres_sim::clock::absolute_day(&g.world);
        let s=g.world.nation_mut(id).equipment.get_or_insert_with(Default::default);s.finance_from_day=day;
        s.revisions.insert("supply-ifv".into(),eq::DesignRevision{id:"supply-ifv".into(),name:"Supply IFV".into(),specification_key:eq::specification_key(&spec),spec,profile:p,created_day:day,certified_day:Some(day)});
        let district=g.world.districts.iter().find(|(_,n)|**n==id).unwrap().0.clone();
        g.world.production.provinces.push(spheres_sim::production::ProvinceCapabilities{district:district.clone(),infrastructure:0,civilian_industry:0,power_grid:1,research_centers:0,arms_plants:3});
        let generator=spheres_sim::industry::EXTENDED.iter().position(|k|*k==spheres_sim::production::ProjectKind::Generation).unwrap();
        g.world.production.industry.sites.entry(district.clone()).or_default()[generator]=1;
        g.world.production.operations.advanced_components.insert(id,5.0);
        eq::start_production(&mut g.world,id,"supply-ifv",&district,10,0.5).unwrap();g.world.day=20;
        let p=&mut g.world.nation_mut(id).equipment.as_mut().unwrap().projects[0];p.work_days=p.tooling_days as f64;p.spent_bn=p.tooling_cost_bn;
        // Isolate copper purchasing: the factory itself has funded electricity,
        // operating fuel and components, while Germany holds the missing raw input.
        let mut stocks=vec![spheres_sim::resources::Stock{nation:id,commodity:spheres_sim::resources::Commodity::Coal,quantity:1000.0,reserve_target:0.0},spheres_sim::resources::Stock{nation:NationId::Germany,commodity:spheres_sim::resources::Commodity::Copper,quantity:10_000_000.0,reserve_target:0.0}];
        stocks.sort_by_key(|s|(s.nation,s.commodity));
        g.world.resources.market=Some(spheres_sim::resources::MarketState{prices:[1000.0;12],previous_prices:[1000.0;12],cleared_volume:[0.0;12],unmet_orders:[0.0;12],fills:vec![],contract_fills:vec![],shipment_audits:vec![],contract_spend_bn:vec![],stocks,cash:vec![],last_produced:-1,last_cleared:-1,last_produced_day:None,last_cleared_day:None,period_days:None});
        for (nation,cash) in [(id,100.0),(NationId::Germany,0.0)]{let n=g.world.nation_mut(nation);n.treasury_bn=Some(cash);n.debt_bn=Some(0.0);n.debt_gdp=0.0;}
        spheres_sim::programs::begin_day(&mut g.world);
        spheres_sim::industry_operations::begin_day(&mut g.world);
        assert!(spheres_sim::industry_operations::operating_fraction(&g.world,&district,spheres_sim::production::ProjectKind::ArmsPlant)>0.0);
        g
    }
    #[test]
    fn supply_review_exposes_actual_payment_and_partial_shipments_without_mutation(){
        let mut g=fixture();let id=NationId::France;let before=spheres_sim::save(&g.world);
        let board=replenishment_board(&g.world,id);assert_eq!(board["actions"][0]["requires_preview"],true);
        let quoted=preview(&g.world,id,&g.session_id,&json!({"command":command()})).unwrap();
        assert_eq!(quoted["valid"],true,"{quoted}");assert_eq!(quoted["actions"][0]["command"],command());
        assert!(quoted["service_effects"]["roles"].as_array().unwrap().iter().any(|r|r["label"].as_str().is_some_and(|s|s.contains("from Germany"))));
        assert!(quoted["service_effects"]["warnings"].as_array().unwrap().iter().any(|r|r.as_str().unwrap().contains("partial supply")));
        assert_eq!(spheres_sim::save(&g.world),before);
        if let Some(path)=std::env::var_os("SPHERES_REPLENISHMENT_QA_SAVE"){
            let mut qa=g.world.clone();spheres_sim::arsenal::deliver_design(qa.nation_mut(id),"supply-ifv",10,18.0).unwrap();
            std::fs::write(path,spheres_sim::save(&qa)).unwrap();
        }
        let parsed=super::super::parse_command(&g.world,&quoted["actions"][0]["command"],id).unwrap();
        spheres_sim::apply_command(&mut g.world,&parsed).unwrap();
        assert!(g.world.nation(id).treasury_bn.unwrap()>=99.999);assert!(g.world.logistics.cargo.iter().any(|c|c.buyer==id));
    }
    #[test]
    fn malformed_supply_choices_cannot_bypass_the_review_contract(){
        let g=fixture();let before=spheres_sim::save(&g.world);
        for bad in [json!(-1),json!(1.5),json!(4294967296u64),json!("30")]{let mut v=command();v["horizon_days"]=bad;assert!(super::super::parse_command(&g.world,&v,NationId::France).is_none());}
        for bad in [json!("1"),Value::Null]{let mut v=command();v["spending_cap_mn"]=bad;assert!(super::super::parse_command(&g.world,&v,NationId::France).is_none());}
        let mut request=command();request["spending_cap_mn"]=json!(0);
        let q=preview(&g.world,NationId::France,&g.session_id,&json!({"command":request})).unwrap();assert_eq!(q["valid"],false);assert_eq!(q["actions"][0]["enabled"],false);
        assert_eq!(spheres_sim::save(&g.world),before);
    }
}
