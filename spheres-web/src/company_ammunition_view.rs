// Read-only ammunition procurement adapters. Prices, eligibility, payments and
// ownership transitions come from the company simulator, including in reviews.
fn company_ammo_unit(family:&str)->&'static str {
    if family.starts_with("air_bomb_"){ "mission store" }else{ "round" }
}

fn company_ammo_need(w:&WorldState,me:NationId,family:&str)->Value {
    let n=w.nation(me);let state=n.equipment.as_ref();
    let models:Vec<_>=state.into_iter().flat_map(|s|s.revisions.values())
        .filter(|r|r.certified_day.is_some()&&eq::ammunition_family(&r.spec)==Some(family)).map(|r|r.name.as_str()).collect();
    let def=eq::ammo_def(family);
    let reference:f64=n.arsenal.held.iter().filter_map(|h|{
        let r=h.design_id.as_ref().and_then(|id|state.and_then(|s|s.revisions.get(id)))?;
        if eq::ammunition_family(&r.spec)!=Some(family){return None;}
        let units=spheres_sim::arsenal::available_design_units(h) as f64;
        Some(units*r.profile.aviation.as_ref().map_or_else(||def.map_or(0.0,|d|d.rounds_per_vehicle_month),|a|a.sorties_per_aircraft_month*a.stores_per_sortie))
    }).sum();
    let stock=state.and_then(|s|s.ammunition.as_ref()).and_then(|s|s.stocks.get(family)).copied().unwrap_or(0.0);
    let public:u64=state.and_then(|s|s.ammunition.as_ref()).into_iter().flat_map(|s|s.orders.iter())
        .filter(|o|o.family==family&&!matches!(o.status,eq::ProjectStatus::Complete|eq::ProjectStatus::Cancelled))
        .map(|o|o.quantity.saturating_sub(o.completed_rounds) as u64).sum();
    let incoming=companies::ammo_inbound_units(w,me,family);
    let saved=state.and_then(|s|s.ammunition_reserves.get(family)).map(|p|p.target_rounds);
    let target=saved.map_or(reference,|t|t as f64);
    let gap=(target-stock-public as f64-incoming as f64).max(0.0).ceil().min(u32::MAX as f64) as u32;
    json!({"stock":stock,"public":public,"incoming":incoming,"target":target,"saved":saved.is_some(),"gap":gap,"models":models})
}

fn company_ammo_need_metrics(need:&Value,family:&str)->Vec<Value> {
    let unit=company_ammo_unit(family);
    vec![metric("In national stores",format!("{} {unit}s",ammo_quantity(company_amount(need,"stock")))),
        metric(if need["saved"]==true{"Saved national reserve target"}else{"One-month planning reserve"},format!("{} {unit}s",ammo_quantity(company_amount(need,"target")))),
        metric("Unfinished public output",format!("{} {unit}s",ammo_quantity(company_count(need,"public") as f64))),metric("Purchased company stock in transit",format!("{} {unit}s",ammo_quantity(company_count(need,"incoming") as f64))),
        metric("Reserve gap after all commitments",format!("{} {unit}s",ammo_quantity(company_count(need,"gap") as f64)))]
}

fn company_ammo_target_input(target:u64,initial:bool)->Value {
    json!({"key":"stock_target","label":"Company ammunition stock target","type":"number","value":target,"min":if initial{1}else{0},"max":eq::MAX_AMMO_ORDER,"step":1,
        "note":"Company-owned inventory. Every government purchase is reviewed separately."})
}

fn company_ammo_supply_action(w:&WorldState,me:NationId,company:u64,family:&str)->Value {
    let need=company_ammo_need(w,me,family);
    let target=company_count(&need,"gap").max(eq::ammo_def(family).map_or(1,|d|d.rounds_per_day.ceil().max(1.0) as u64)).min(eq::MAX_AMMO_ORDER as u64);
    let mut action=intent("Review company ammunition supply",json!({"kind":"company_ammo_supply","company":company,"family":family,"stock_target":target}),vec![company_ammo_target_input(target,true)]);
    action["detail"]=json!("Authorize this company to manufacture a finite buffer of an already certified ammunition family using its own cash, real materials and existing shared plant slot. This buys no rounds. Future automatic public batches for this family stop; existing batches and reserve preferences remain.");
    action
}

fn company_ammunition_preview(w:&WorldState,me:NationId,session:&str,command:&Value,order:&CompanyOrder)->Value {
    let (company,product,family,label,detail)=match order {
        CompanyOrder::AmmoSupply{company,family,..}=>(*company,None,Some(family.as_str()),"Authorize company ammunition supply","The manufacturer builds its own ammunition inventory. Review the stock target and the switch in future replenishment before authorizing supply."),
        CompanyOrder::AmmoInventory{company,product,..}=>(*company,Some(*product),None,"Set ammunition stock target","Change the manufacturer's finite inventory target. This places no government purchase and creates no national ammunition."),
        CompanyOrder::AmmoPurchase{company,product,..}=>(*company,Some(*product),None,"Buy available ammunition","Buy this exact ammunition family from finished company stock. Paid rounds count toward your reserve but cannot be fired until they arrive."),
        _=>unreachable!(),
    };
    let raw=companies::view(w,me);
    let firm=raw["companies"].as_array().and_then(|rows|rows.iter().find(|f|company_count(f,"id")==company as u64));
    let p=firm.and_then(|f|f["ammunition_products"].as_array()).and_then(|rows|rows.iter().find(|p|Some(company_count(p,"id") as u32)==product));
    let family=family.or_else(||p.and_then(|p|p["family"].as_str())).unwrap_or("");
    let unit=company_ammo_unit(family);
    let quote=company_quote(w,me,order);
    let mut confirmed=command.clone();
    if let Some(q)=&quote {confirmed["quote"]=q["token"].clone();}
    let mut action=checked(w,me,label,confirmed);
    let mut blockers:Vec<String>=action["reason"].as_str().map(str::to_string).into_iter().collect();
    if let Some(q)=&quote {if q["valid"]==false {if let Some(reason)=q["reason"].as_str() {if !blockers.iter().any(|r|r==reason){blockers.push(reason.into());}}}}
    if !blockers.is_empty(){action["enabled"]=json!(false);}
    let need=company_ammo_need(w,me,family);
    let mut metrics=vec![metric("Manufacturer",firm.map(|f|f["name"].clone()).unwrap_or(Value::Null)),metric("Exact ammunition family",eq::ammo_def(family).map_or(family,|d|d.name))];
    metrics.extend(company_ammo_need_metrics(&need,family));
    let mut costs=vec![];let mut timing=vec![];
    let models=need["models"].as_array().map(|a|a.iter().filter_map(Value::as_str).collect::<Vec<_>>().join(", ")).unwrap_or_default();
    let mut requirements=vec![format!("Compatible certified models: {}.",if models.is_empty(){"none"}else{&models}),
        "Ammunition keeps its exact family. It grants no vehicles, strength, extra capacity or ground ammunition activation. Aircraft always require their physical mission stores.".into()];
    let mut effects=Value::Null;
    match order {
        CompanyOrder::AmmoSupply{stock_target,..}|CompanyOrder::AmmoInventory{stock_target,..}=>{
            metrics.push(metric("Company-owned stock target",format!("{} {unit}s",ammo_quantity(*stock_target as f64))));
            requirements.push("The company pays for raw inputs and fabrication from settled working capital. Development and equipment manufacturing share its one existing leased plant slot; earlier work can delay ammunition.".into());
            requirements.push("Government reserve targets and existing public batches remain. Future automatic public batches for this supplied family stop. The company stock target does not authorize automatic government purchases.".into());
            if matches!(order,CompanyOrder::AmmoInventory{stock_target:0,..}) {requirements.push("Zero stops new restocking. Company stock, paid deliveries and any work already in progress remain recorded.".into());}
        },
        CompanyOrder::AmmoPurchase{quantity,..}=>{
            metrics.push(metric("Quantity to buy",format!("{} {unit}s",ammo_quantity(*quantity as f64))));
            requirements.push("Maintenance & supply funds this purchase once, after protecting today's unpaid fleet upkeep. Payment must settle before delivery begins; arrival adds the purchased physical stores once.".into());
            let mut after=w.clone();
            if blockers.is_empty() {
                if super::parse_command(w,&action["command"],me).and_then(|c|spheres_sim::apply_command(&mut after,&c).ok()).is_some() {
                    let future=company_ammo_need(&after,me,family);
                    effects=json!({"title":"What this purchase changes","detail":"Usable ammunition stays unchanged until delivery. Only paid incoming rounds reduce the national reserve gap; unsold company inventory remains outside national stores.",
                        "metrics":[metric("National stores now",ammo_quantity(company_amount(&need,"stock"))),metric("Purchased incoming before",company_count(&need,"incoming")),metric("Purchased incoming after",company_count(&future,"incoming")),metric("Reserve gap before",company_count(&need,"gap")),metric("Reserve gap after",company_count(&future,"gap"))],"warnings":[]});
                }
            }
        },_=>unreachable!(),
    }
    if let Some(q)=&quote {
        if let Some(note)=q["note"].as_str(){requirements.push(note.into());}
        if matches!(order,CompanyOrder::AmmoPurchase{..}) {
            metrics.push(metric("Available finished stock",format!("{} {unit}s",ammo_quantity(company_count(q,"available_units") as f64))));
            costs.push(cost("Supplier unit price",company_amount(q,"unit_price_bn"),&format!("per {unit} · includes materials and fabrication")));
            costs.push(cost("Total ammunition purchase",company_amount(q,"cost_bn"),"one purchase · Defense Maintenance & supply"));
            costs.push(cost("Protected fleet upkeep",company_amount(q,"protected_maintenance_bn"),"unpaid requirement today · retained before this purchase"));
            costs.push(cost("Available after upkeep protection",company_amount(q,"purchase_available_bn"),"shared Maintenance & supply authority"));
            timing.push(metric("Delivery after payment settlement",format!("{} accessible days",company_count(q,"minimum_days"))));
        }else{
            costs.push(cost("Company capital required",company_amount(q,"company_cash_needed_bn"),"current estimated materials and fabrication · company pays"));
            costs.push(cost("Government purchase now",0.0,"no stores purchased by supply authorization"));
        }
    }
    json!({"session_id":session,"nation":me,"valid":blockers.is_empty(),"blockers":blockers,"metrics":metrics,"costs":costs,"timing":timing,"requirements":requirements,"service_effects":effects,
        "actions":[action,nav("Review ammunition stores and reserves",json!({"action":"equipment","tab":"ammunition"})),nav("Review Maintenance & supply funding",json!({"action":"budget","ministry":"defense","department":2}))],"detail":detail})
}

fn company_ammo_rows(w:&WorldState,me:NationId,raw:&Value)->(Vec<Value>,Vec<Value>) {
    let empty=vec![];let mut products=vec![];
    for firm in raw["companies"].as_array().unwrap_or(&empty) {
        let company=company_count(firm,"id");
        for p in firm["ammunition_products"].as_array().unwrap_or(&empty) {
            let product=company_count(p,"id");let family=company_text(p,"family");let unit=company_ammo_unit(family);
            let stock=company_count(p,"stock");let target=company_count(p,"stock_target");let need=company_ammo_need(w,me,family);
            let mut metrics=vec![metric("Available to buy",format!("{} {unit}s",ammo_quantity(stock as f64))),metric("Company stock target",format!("{} {unit}s",ammo_quantity(target as f64))),
                metric("Maximum manufacturing rate",format!("{} {unit}s / day",ammo_quantity(company_amount(p,"rounds_per_day")))),
                metric("Stock target estimate",p["estimated_stock_days"].as_u64().map(|d|format!("About {d} days with cash, inputs and access")).unwrap_or_else(||if stock>=target{"Target met".into()}else{format!("No dated estimate. {}",company_text(p,"estimate_note"))}))];
            metrics.extend(company_ammo_need_metrics(&need,family));
            let gap=company_count(&need,"gap");
            let reason=if gap>0 {format!("Your reserve still needs {} {unit}s after national stocks, paid company deliveries and unfinished public batches.",ammo_quantity(gap as f64))}
                else if company_amount(&need,"target")<=0.0 {"No active fleet demand or reserve target currently calls for a purchase. Set a reserve for the fleet you are preparing.".into()}
                else {"Your reserve is covered by national stores and existing commitments. Additional purchases are optional.".into()};
            let mut actions=vec![];
            if stock>0 {
                let quantity=gap.max(1).min(stock);
                actions.push(intent("Review ammunition purchase",json!({"kind":"company_ammo_purchase","company":company,"product":product,"quantity":quantity}),vec![json!({"key":"quantity","label":format!("Available {unit}s to buy"),"type":"number","value":quantity,"min":1,"max":stock,"step":1})]));
            }
            actions.push(intent("Review ammunition stock target",json!({"kind":"company_ammo_inventory","company":company,"product":product,"stock_target":target}),vec![company_ammo_target_input(target,false)]));
            actions.push(ammunition_reserve_action(w,me,family,company_amount(&need,"target").ceil().max(1.0).min(eq::MAX_AMMO_RESERVE_TARGET as f64) as u32));
            let models=need["models"].as_array().unwrap_or(&empty).iter().filter_map(Value::as_str).collect::<Vec<_>>().join(", ");
            products.push(json!({"id":format!("ammo:{company}:{product}"),"company":company,"product":product,"family":"ammunition","ammo_family":family,"unit_label":unit,"name":eq::ammo_def(family).map_or(family,|d|d.name),"supplier_name":firm["name"],
                "status":if stock>0{"Available to buy"}else if target==0{"Restocking stopped"}else if p["status"]=="blocked"{"Needs attention"}else{"Building company stock"},"phase":"ammunition","detail":format!("{reason} {}",company_text(p,"reason")),
                "metrics":metrics,"costs":[cost("Manufacturer unit price",company_amount(p,"unit_price_bn"),&format!("per {unit} · {}",if stock>0{"reviewed before purchase"}else{"indicative until stock is available"}))],
                "requirements":[format!("Compatible certified models: {models}."),"Company manufacturing shares its existing plant with vehicle development and restocking. Unsold ammunition belongs to the company. Every government purchase needs review."],"actions":actions}));
        }
    }
    let deliveries=raw["ammunition_deliveries"].as_array().unwrap_or(&empty).iter().rev().map(|d|{
        let family=company_text(d,"family");let unit=company_ammo_unit(family);let arrived=d["delivered_day"].is_number();
        let supplier=raw["companies"].as_array().and_then(|firms|firms.iter().find(|f|f["id"]==d["company"])).map(|f|f["name"].clone());
        json!({"id":format!("ammo-delivery:{}",company_count(d,"id")),"family":"ammunition","ammo_family":family,"unit_label":unit,"supplier_name":supplier,"name":format!("{} · purchase #{}",eq::ammo_def(family).map_or(family,|def|def.name),company_count(d,"id")),
            "status":if arrived{"Delivered"}else if d["settled_day"].is_null(){"Awaiting payment settlement"}else if d["status"]=="blocked"{"Delivery paused"}else{"In transit"},"detail":d["reason"],
            "metrics":[metric("Manufacturer",supplier),metric("Purchased ammunition",format!("{} {unit}s",ammo_quantity(company_count(d,"quantity") as f64))),metric("Delivery remaining",if arrived{"Arrived".into()}else if d["settled_day"].is_null(){"Begins after payment settlement".into()}else{d["remaining_days"].as_u64().map(|days|format!("{days} accessible days")).unwrap_or("Awaiting current delivery estimate".into())})],
            "costs":[cost("Reviewed purchase total",company_amount(d,"total_price_bn"),"Maintenance & supply · paid once")],"actions":[nav("Review national ammunition stores",json!({"action":"equipment","tab":"ammunition"}))]})
    }).collect();
    (products,deliveries)
}

fn company_ammunition_market(w:&WorldState,me:NationId)->Value {
    let raw=companies::view(w,me);let (mut offers,deliveries)=company_ammo_rows(w,me,&raw);
    let mut actions=vec![];
    if let Some(firm)=raw["companies"].as_array().and_then(|rows|rows.first()) {
        for def in eq::ammo_catalog() {
            let need=company_ammo_need(w,me,def.id);
            if need["models"].as_array().is_none_or(Vec::is_empty)||companies::ammo_supplier_active(w,me,def.id){continue;}
            offers.push(json!({"id":format!("supply:{}",def.id),"family":"ammunition","ammo_family":def.id,"unit_label":company_ammo_unit(def.id),"name":def.name,"supplier_name":firm["name"],"status":"Supply authorization available",
                "detail":"A certified model uses this family. Authorize company-funded inventory, then review purchases when finished stock is available. Existing public batches continue.","metrics":company_ammo_need_metrics(&need,def.id),
                "actions":[company_ammo_supply_action(w,me,company_count(firm,"id"),def.id)]}));
        }
    }else{actions.push(nav("Establish an equipment manufacturer",json!({"action":"equipment","tab":"companies"})));}
    actions.push(nav("Review Maintenance & supply funding",json!({"action":"budget","ministry":"defense","department":2})));
    json!({"overview":{"title":"Buy ammunition from manufacturers","status":if offers.is_empty(){"Prepare a supplier and compatible design"}else{"Company stock and reviewed purchases"},
        "detail":"Companies pay to manufacture compatible ammunition using their existing plant and working capital. Buy finished stock with Maintenance & supply funds after fleet upkeep, then follow its delivery into national stores. Supplier inventory and your national reserve are separate targets.","metrics":[],"warnings":["Company supply stops future automatic public batches only for the supplied family. Existing paid batches and reserve targets remain. Purchases and ground ammunition activation are separate decisions."],"actions":actions},"offers":offers,"deliveries":deliveries})
}

#[cfg(test)]
mod company_ammunition_view_tests {
    use super::*;
    const ME:NationId=NationId::France;
    fn apply(g:&mut super::super::Game,c:&Value){
        let parsed=super::super::parse_command(&g.world,c,ME).unwrap();
        spheres_sim::apply_command(&mut g.world,&parsed).unwrap();
    }
    fn review(g:&super::super::Game,c:Value)->Value {
        let p=preview(&g.world,ME,&g.session_id,&json!({"command":c})).unwrap();
        assert_eq!(p["valid"],true,"{p}");p
    }
    fn fixture()->(super::super::Game,String,u32){
        let mut g=super::ammunition_view_tests::fixture();
        let site=g.world.production.provinces.iter().find(|p|p.arms_plants>0&&g.world.districts.get(&p.district)==Some(&ME)).unwrap().district.clone();
        let n=g.world.nation_mut(ME);n.program_budget.as_mut().unwrap().available_bn[BUDGET_DEFENSE][3]=1.0;
        n.program_budget.as_mut().unwrap().available_bn[BUDGET_DEFENSE][2]=0.1;
        let r=review(&g,json!({"kind":"company_establish","name":"Ammunition interface works","district":site,"capital_mn":25.0}));
        apply(&mut g,&r["actions"][0]["command"]);
        let id=g.world.companies.firms[0].id;
        let family=eq::ammunition_family(&eq::default_spec("ground_ifv")).unwrap().to_string();
        (g,family,id)
    }
    fn stocked()->(super::super::Game,String,u32,u32){
        let (mut g,family,id)=fixture();
        let reserve=json!({"kind":"equipment_ammo_reserve","family":family,"target_rounds":100,"district":g.world.companies.firms[0].district,"daily_budget_mn":0.1,"automatic":false});
        apply(&mut g,&reserve);
        let r=review(&g,json!({"kind":"company_ammo_supply","company":id,"family":family,"stock_target":50}));
        apply(&mut g,&r["actions"][0]["command"]);
        let product=g.world.companies.firms[0].ammunition_products[0].id;
        for _ in 0..4 {spheres_sim::tick_day(&mut g.world,&[]);}
        assert_eq!(g.world.companies.firms[0].ammunition_products[0].stock,50);
        (g,family,id,product)
    }
    #[test]
    fn supplier_offer_and_review_preserve_state_and_explain_separate_ownership(){
        let (g,family,id)=fixture();let before=spheres_sim::save(&g.world);
        let market=company_ammunition_market(&g.world,ME);
        let offer=market["offers"].as_array().unwrap().iter().find(|o|o["ammo_family"]==family).unwrap();
        assert_eq!(offer["family"],"ammunition");assert!(offer.get("spec").is_none());
        let r=review(&g,json!({"kind":"company_ammo_supply","company":id,"family":family,"stock_target":50}));
        assert!(r["actions"][0]["command"]["quote"].as_str().is_some_and(|s|!s.is_empty()));
        assert!(r["requirements"].as_array().unwrap().iter().any(|s|s.as_str().unwrap().contains("Future automatic public batches")));
        assert_eq!(spheres_sim::save(&g.world),before);
    }
    #[test]
    fn ammunition_parser_rejects_ambiguous_quantities_and_uses_player_authority(){
        let (g,family,id)=fixture();let before=spheres_sim::save(&g.world);
        for kind in ["company_ammo_supply","company_ammo_inventory","company_ammo_purchase"] {
            let key=if kind=="company_ammo_purchase"{"quantity"}else{"stock_target"};
            let mut c=json!({"kind":kind,"company":id,"family":family,"product":2,"stock_target":10,"quantity":10,"nation":"USA"});
            assert!(matches!(super::super::parse_command(&g.world,&c,ME),Some(Command::Company{nation:ME,..})));
            for value in [json!(-1),json!(0.5),json!(4294967296u64),json!("10"),json!(null)] {
                c[key]=value;assert!(super::super::parse_command(&g.world,&c,ME).is_none(),"{c}");
            }
        }
        assert_eq!(spheres_sim::save(&g.world),before);
    }
    #[test]
    fn purchase_review_nets_only_paid_incoming_and_transport_retries_once(){
        let (mut g,family,id,product)=stocked();let before=spheres_sim::save(&g.world);
        let r=review(&g,json!({"kind":"company_ammo_purchase","company":id,"product":product,"quantity":40}));
        assert_eq!(spheres_sim::save(&g.world),before);
        let effects=r["service_effects"]["metrics"].as_array().unwrap();
        assert!(effects.iter().any(|m|m["label"]=="Reserve gap before"&&m["value"]==100));
        assert!(effects.iter().any(|m|m["label"]=="Reserve gap after"&&m["value"]==60));
        assert!(r["costs"].as_array().unwrap().iter().any(|c|c["label"]=="Protected fleet upkeep"));
        let request=json!({"session_id":g.session_id,"client_id":"ammo-api-test","request_seq":1,"commands":[r["actions"][0]["command"]]});
        let result=super::super::transport::immediate_request(&mut g,&request).unwrap();assert_eq!(result["errors"],json!([]));
        let committed=spheres_sim::save(&g.world);
        let replay=super::super::transport::immediate_request(&mut g,&request).unwrap();assert_eq!(replay["command_replayed"],true);
        assert_eq!(spheres_sim::save(&g.world),committed);
        let market=company_ammunition_market(&g.world,ME);assert_eq!(market["deliveries"].as_array().unwrap().len(),1);
        let offer=&market["offers"][0];assert!(offer.get("spec").is_none());
        assert_eq!(company_ammo_need(&g.world,ME,&family)["gap"],60);
        assert_eq!(eq::ammo_reserve_status(&g.world,ME,&family).gap,60);
        assert_eq!(company_ammo_need(&g.world,ME,&family)["stock"],0.0);
        assert!(company_board(&g.world,ME)["products"].as_array().unwrap().iter().any(|p|p["family"]=="ammunition"&&p["ammo_family"]==family));
    }
    #[test]
    fn converted_reserve_review_keeps_preference_and_removes_public_batch_recommendation(){
        let (mut g,family,id)=fixture();
        let district=g.world.companies.firms[0].district.clone();
        let r=review(&g,json!({"kind":"company_ammo_supply","company":id,"family":family,"stock_target":50}));apply(&mut g,&r["actions"][0]["command"]);
        let c=json!({"kind":"equipment_ammo_reserve","family":family,"target_rounds":100,"district":district,"daily_budget_mn":0.1,"automatic":true});
        let r=review(&g,c);assert_eq!(r["actions"][0]["label"],"Save supplier reserve target");apply(&mut g,&r["actions"][0]["command"]);
        assert!(g.world.nation(ME).equipment.as_ref().unwrap().ammunition_reserves[&family].automatic);
        let board=ammunition_board(&g.world,ME);let row=&board["reserves"][0];
        assert_eq!(row["status"],"Reviewed company purchases");assert_eq!(row["actions"][0]["command"]["automatic"],true);
        assert!(!row["actions"].as_array().unwrap().iter().any(|a|a["command"]["kind"]=="equipment_ammo_order"));
        assert!(row["actions"].as_array().unwrap().iter().any(|a|a["navigate"]["tab"]=="ammunition"));
    }
}
