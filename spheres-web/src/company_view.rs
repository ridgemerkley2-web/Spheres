// Company ownership, quotes and work are simulated in spheres_sim::companies.
// This adapter supplies labels and review forms to the equipment bureau.
use spheres_sim::companies::{self, CompanyOrder};

pub(super) fn parse_company_order(v: &Value) -> Option<CompanyOrder> {
    let text = |key: &str| v.get(key)?.as_str().map(str::to_string);
    let integer = |key: &str| v.get(key)?.as_u64()?.try_into().ok();
    let money = |key: &str| v.get(key)?.as_f64().filter(|x| x.is_finite()).map(|x| x / 1000.0);
    // An initial preview has no token. Its returned confirmation action carries
    // the simulator's exact quote; actual orders never silently refresh it.
    let token = || match v.get("quote") {
        None => Some(String::new()),
        Some(value) => value.as_str().filter(|s| s.len() <= 256).map(str::to_string),
    };
    Some(match v.get("kind")?.as_str()? {
        "company_establish" => CompanyOrder::Establish {
            name: text("name")?, district: text("district")?,
            capitalization_bn: money("capital_mn")?, quote: token()?,
        },
        "company_capitalize" => CompanyOrder::Capitalize {
            company: integer("company")?, amount_bn: money("amount_mn")?, quote: token()?,
        },
        "company_develop" => CompanyOrder::Develop {
            company: integer("company")?, name: text("name")?,
            spec: serde_json::from_value(json!({"platform":v.get("platform"),"components":v.get("components")})).ok()?,
            daily_budget_bn: money("daily_budget_mn")?, stock_target: integer("stock_target")?, quote: token()?,
        },
        "company_purchase" => CompanyOrder::Purchase {
            company: integer("company")?, product: integer("product")?,
            quantity: integer("quantity")?, quote: token()?,
        },
        "company_funding" => CompanyOrder::Funding {
            company: integer("company")?, product: integer("product")?, daily_budget_bn: money("daily_budget_mn")?,
        },
        "company_inventory" => CompanyOrder::Inventory {
            company: integer("company")?, product: integer("product")?, stock_target: integer("stock_target")?,
        },
        "company_cancel" => CompanyOrder::CancelDevelopment {
            company: integer("company")?, product: integer("product")?,
        },
        _ => return None,
    })
}

fn company_quote(w: &WorldState, me: NationId, order: &CompanyOrder) -> Option<Value> {
    match order {
        CompanyOrder::Establish {name,district,capitalization_bn,..} =>
            serde_json::to_value(companies::establishment_quote(w,me,name,district,*capitalization_bn)).ok(),
        CompanyOrder::Capitalize {company,amount_bn,..} =>
            serde_json::to_value(companies::capitalization_quote(w,me,*company,*amount_bn)).ok(),
        CompanyOrder::Develop {company,name,spec,daily_budget_bn,stock_target,..} =>
            serde_json::to_value(companies::development_quote(w,me,*company,name,spec,*daily_budget_bn,*stock_target)).ok(),
        CompanyOrder::Purchase {company,product,quantity,..} =>
            serde_json::to_value(companies::purchase_quote(w,me,*company,*product,*quantity)).ok(),
        _ => None,
    }
}

fn company_preview(w: &WorldState, me: NationId, session: &str, command: &Value, order: &CompanyOrder) -> Value {
    let quote = company_quote(w,me,order);
    let mut confirmed = command.clone();
    if let Some(q) = &quote {confirmed["quote"] = q["token"].clone();}
    let (label,detail) = match order {
        CompanyOrder::Establish {..} => ("Establish state contractor", "Create a state-owned tank manufacturer with a separately funded account and rights to one existing arms-plant slot."),
        CompanyOrder::Capitalize {..} => ("Invest in manufacturer", "Transfer this additional investment to the company's account. It supports company-owned tooling and stock; it does not purchase equipment for the government."),
        CompanyOrder::Develop {..} => ("Commission tank development", "Commission the manufacturer to develop and test this exact design. Development funding buys certification and manufacturing rights; finished tanks require a later purchase."),
        CompanyOrder::Purchase {..} => ("Buy available tanks", "Buy existing finished stock at this reviewed price. Purchased tanks enter service only after payment settlement and delivery."),
        CompanyOrder::Funding {..} => ("Apply development funding", "Set the maximum the government can pay each day for actual contracted development work. Zero pauses future work and retains paid progress."),
        CompanyOrder::Inventory {..} => ("Set company stock target", "Set a finite finished-stock target. The state company uses its own cash and facility to replenish it; this does not authorize government equipment purchases."),
        CompanyOrder::CancelDevelopment {..} => ("Cancel unfinished development", "End remaining development work. Completed work and its payment remain recorded; cancellation does not refund spent engineering and trials costs."),
    };
    let mut action = checked(w,me,label,confirmed);
    let mut blockers: Vec<String> = action["reason"].as_str().map(str::to_string).into_iter().collect();
    if let Some(q) = &quote {
        if q["valid"] == false {
            if let Some(reason) = q["reason"].as_str() {
                if !blockers.iter().any(|b| b == reason) {blockers.push(reason.into());}
            }
        }
    }
    if !blockers.is_empty() {action["enabled"]=json!(false);}
    let mut metrics = vec![];
    let mut costs = vec![];
    let mut timing = vec![];
    let mut requirements: Vec<String> = vec![];
    if let Some(parsed)=super::parse_command(w,&action["command"],me) {
        if let Some(price)=spheres_sim::price_of(w,&parsed).filter(|price|*price>0.0) {
            metrics.push(metric("Political capital",format!("{price:.0} PC once")));
        }
    }
    let identity=match order {
        CompanyOrder::Establish{..}=>None,
        CompanyOrder::Capitalize{company,..}=>Some((*company,None)),
        CompanyOrder::Develop{company,..}=>Some((*company,None)),
        CompanyOrder::Purchase{company,product,..}|CompanyOrder::Funding{company,product,..}|
        CompanyOrder::Inventory{company,product,..}|CompanyOrder::CancelDevelopment{company,product}=>Some((*company,Some(*product))),
    };
    let selected=identity.and_then(|(id,product)|companies::company(w,me,id).map(|firm|(firm,product.and_then(|id|firm.products.iter().find(|p|p.id==id)))));
    if let Some((firm,product))=selected {
        metrics.push(metric("Manufacturer",&firm.name));
        if let Some(product)=product {
            let model=w.nation(me).equipment.as_ref().and_then(|s|s.revisions.get(&product.revision_id));
            metrics.push(metric("Exact model",model.map_or(product.revision_id.as_str(),|r|r.name.as_str())));
            metrics.push(metric("Frozen revision",&product.revision_id));
        }
    }
    let mut service_effects=Value::Null;
    match order {
        CompanyOrder::Establish {name,district,capitalization_bn,..} => {
            metrics.push(metric("Manufacturer",name));
            metrics.push(metric("Ownership","State-owned; corporate cash remains separate"));
            metrics.push(metric("Leased production site",spheres_sim::districts::name_of(district).unwrap_or(district)));
            metrics.push(metric("Existing capacity committed","One completed arms-plant slot"));
            costs.push(cost("Initial company investment",*capitalization_bn,"one transfer · Defense procurement"));
            requirements.push("The leased slot cannot simultaneously serve a public manufacturing line, vehicle project or ammunition batch. Building additional facilities remains a separate construction decision.".into());
            requirements.push("This establishes a new modeled company; it is not a historical 1990 corporation or an inherited stock grant.".into());
        },
        CompanyOrder::Capitalize {amount_bn,..} => {
            costs.push(cost("Additional company investment",*amount_bn,"one transfer · Defense procurement"));
        },
        CompanyOrder::Develop {name,spec,daily_budget_bn,stock_target,..} => {
            metrics.push(metric("Model",name));
            metrics.push(metric("Initial company stock target",*stock_target));
            if let Some(profile) = eq::design_preview(w,me,spec).profile {
                metrics.extend(profile_metrics(&profile));
                costs.push(cost("Development and trials",profile.development_cost_bn,"government R&D · paid as work progresses"));
                costs.push(cost("Company tooling commitment",profile.tooling_cost_bn,"company working capital after certification"));
                timing.push(metric("Engineering and trials minimum",format!("{} days",profile.development_days)));
                timing.push(metric("Tooling minimum after certification",format!("{} days",profile.tooling_days)));
                timing.push(metric("Fabrication per tank",format!("{} days after tooling",profile.production_days)));
                costs.push(cost("Maintenance after a later purchase",profile.maintenance_bn_day,"per delivered tank / day"));
            }
            costs.push(cost("Government development ceiling",*daily_budget_bn,"per day · shared R&D authority"));
            requirements.push("The contract freezes this model's specifications. A later design edit needs its own revision and development review.".into());
            requirements.push("After certification the company pays for production and acquires inputs into its own accounts. Its stock target reserves no government purchases.".into());
        },
        CompanyOrder::Purchase {quantity,..} => {
            metrics.push(metric("Tanks to buy",*quantity));
            if let Some((_,Some(product)))=selected {
                let before=eq::fleet_target_plans_world(w,me).into_iter().find(|p|p.revision==product.revision_id);
                let mut after=w.clone();
                let future=if blockers.is_empty() {
                    super::parse_command(w,&action["command"],me).and_then(|parsed|spheres_sim::apply_command(&mut after,&parsed).ok()).and_then(|_|eq::fleet_target_plans_world(&after,me).into_iter().find(|p|p.revision==product.revision_id))
                } else {None};
                let mut effects=vec![metric("Supplier stock before purchase",product.stock),metric("Supplier stock after purchase",product.stock.saturating_sub(*quantity))];
                if let Some(before)=before {
                    effects.push(metric("Delivered and available now",before.available));
                    effects.push(metric("Government-owned incoming before purchase",before.incoming));
                    if let Some(after)=future {effects.push(metric("Government-owned incoming after purchase",after.incoming));
                        if let Some(desired)=before.desired {effects.push(metric("Fleet target",desired));effects.push(metric("Shortfall before purchase",before.shortfall));effects.push(metric("Shortfall after purchase",after.shortfall));}
                    }
                }
                service_effects=json!({"title":"What this purchase changes","detail":"Your usable fleet stays unchanged until delivery. Paid inbound vehicles count toward your exact-model target; the supplier's remaining stock stays outside your Arsenal.","metrics":effects,"warnings":[]});
            }
            requirements.push("Only the reviewed available quantity is purchased. A remaining fleet shortage is not converted into an automatic order or a production commitment.".into());
            requirements.push("The supplier's price does not change the frozen combat profile. Service support and compatible ammunition remain necessary after delivery.".into());
        },
        CompanyOrder::Funding {daily_budget_bn,..} => {
            costs.push(cost("Development ceiling",*daily_budget_bn,"per day · future actual work"));
        },
        CompanyOrder::Inventory {stock_target,..} => {metrics.push(metric("Finished stock target",*stock_target));},
        CompanyOrder::CancelDevelopment {..} => {requirements.push("The frozen design and dated transactions remain part of the record.".into());},
    }
    if matches!(order,CompanyOrder::Establish{..}|CompanyOrder::Capitalize{..}|CompanyOrder::Purchase{..}) {
        requirements.push("The existing Defense procurement account funds this transaction once. Company receipts become spendable only when their corresponding public payment is settled.".into());
    }
    // Simulator quote fields are added below without computing prices or work
    // rates in the web layer. The action carries the exact token being reviewed.
    if let Some(q) = &quote {
        if let Some(eta) = q.get("eta_days").and_then(Value::as_u64) {
            timing.push(metric("Estimate at current conditions",format!("{eta} days")));
        }
        if let Some(note) = q.get("note").and_then(Value::as_str) {requirements.push(note.into());}
        if matches!(order,CompanyOrder::Purchase{..}) {
            if let Some(price)=q.get("unit_price_bn").and_then(Value::as_f64) {costs.push(cost("Supplier price",price,"per tank · fabrication and inputs included"));}
            if let Some(total)=q.get("cost_bn").and_then(Value::as_f64) {costs.push(cost("Total equipment purchase",total,"one purchase · Defense procurement"));}
            if let Some(stock)=q.get("available_units").and_then(Value::as_u64) {metrics.push(metric("Supplier stock before purchase",stock));}
            if let Some(days)=q.get("minimum_days").and_then(Value::as_u64) {timing.push(metric("Delivery after settlement",format!("{days} accessible days")));}
            if let Some(upkeep)=q.get("maintenance_bn_day").and_then(Value::as_f64) {costs.push(cost("Added maintenance requirement",upkeep,"per day after delivery · all purchased tanks"));}
        }
        if matches!(order,CompanyOrder::Develop{..}) {
            if let Some(cash)=q.get("company_cash_needed_bn").and_then(Value::as_f64) {costs.push(cost("Company capital needed for first stock target",cash,"tooling, inputs and fabrication · current prices"));}
            if let Some(price)=q.get("unit_price_bn").and_then(Value::as_f64) {costs.push(cost("Indicative later purchase price",price,"per tank · reviewed again when stock is available"));}
            if let Some(days)=q.get("first_stock_days").and_then(Value::as_u64) {timing.push(metric("Earliest first stock at this funding",format!("About {days} days with company cash, inputs and access")));}
        }
    }
    json!({"session_id":session,"nation":me,"valid":blockers.is_empty(),"blockers":blockers,
        "metrics":metrics,"costs":costs,"timing":timing,"requirements":requirements,"service_effects":service_effects,"actions":[action],"detail":detail})
}

fn company_money(value: f64) -> String {
    if value.abs() < 1e-12 {return "$0".into();}
    if value.abs() >= 1.0 {format!("${value:.3}bn")}
    else if value.abs() >= 0.001 {format!("${:.2}m",value*1000.0)}
    else {format!("${:.2}k",value*1e6)}
}
fn company_amount(row: &Value, key: &str) -> f64 {row[key].as_f64().unwrap_or(0.0)}
fn company_count(row: &Value, key: &str) -> u64 {row[key].as_u64().unwrap_or(0)}
fn company_text<'a>(row: &'a Value, key: &str) -> &'a str {row[key].as_str().unwrap_or("")}
fn company_cash_input(key: &str, label: &str, value: f64) -> Value {
    json!({"key":key,"label":label,"type":"number","value":value,"min":0.001,"max":1_000_000,"step":0.001,"unit":"$m"})
}
fn company_inventory_input(target: u64, initial: bool) -> Value {
    json!({"key":"stock_target","label":"Company finished-stock target","type":"number","value":target,
        "min":if initial{1}else{0},"max":companies::MAX_STOCK,"step":1,
        "note":"The company finances this buffer. This target places no government equipment order."})
}
fn company_development_action(w: &WorldState, me: NationId, name: &str, spec: &eq::DesignSpec) -> Value {
    let options: Vec<_> = w.companies.firms.iter().filter(|c|c.nation==me).map(|c| {
        let reason=companies::facility_blocker(w,c);
        json!({"value":c.id,"label":c.name,"enabled":reason.is_none(),
            "detail":format!("Tank development and manufacture · {} · {} company cash",spheres_sim::districts::name_of(&c.district).unwrap_or(&c.district),company_money(c.cash_bn)),"reason":reason})
    }).collect();
    if options.is_empty() {
        let mut action=nav("Establish a tank manufacturer",json!({"action":"equipment","tab":"companies"}));
        action["detail"]=json!("Save this draft, then establish a funded contractor with an existing arms plant. Return here to commission development of your tank.");
        return action;
    }
    let firm=options[0]["value"].clone();
    let mut action=intent("Choose manufacturer and review development",json!({"kind":"company_develop","company":firm,"name":name,
        "platform":spec.platform,"components":spec.components,"daily_budget_mn":0.5,"stock_target":4}),vec![
        json!({"key":"company","label":"Developing manufacturer","type":"select","value":firm,"options":options}),
        budget_input(0.0005),company_inventory_input(4,true),
    ]);
    action["detail"]=json!("Review the engineering contract, first-stock estimate and company working capital before commissioning this frozen design. You buy completed tanks later.");
    action
}
fn company_supplies_revision(w: &WorldState, me: NationId, revision: &str) -> bool {
    w.companies.firms.iter().any(|c|c.nation==me&&c.products.iter().any(|p|p.revision_id==revision&&p.cancelled_day.is_none()))
}
fn company_design_costs(p: &eq::CompiledProfile) -> Vec<Value> {
    vec![cost("Development and trials",p.development_cost_bn,"government R&D · one model"),
        cost("Manufacturer tooling",p.tooling_cost_bn,"company capital · once per licensed model"),
        cost("Company fabrication",p.fabrication_cost_bn,"per tank · company also purchases its inputs"),
        cost("Maintenance requirement",p.maintenance_bn_day,"per delivered tank / day · government support")]
}

fn company_board(w: &WorldState, me: NationId) -> Value {
    let raw=companies::view(w,me);
    let empty=Vec::new();
    let firms=raw["companies"].as_array().unwrap_or(&empty);
    let site_options: Vec<_>=raw["sites"].as_array().unwrap_or(&empty).iter().filter(|s|company_count(s,"slots")>0).map(|s| {
        let district=company_text(s,"district");
        json!({"value":district,"label":format!("{} · {} of {} slots in use",spheres_sim::districts::name_of(district).unwrap_or(district),company_count(s,"used_slots"),company_count(s,"slots")),
            "enabled":s["available"],"reason":s["reason"]})
    }).collect();
    let mut actions=vec![];
    if firms.is_empty() {
        let site=site_options.iter().find(|s|s["enabled"]==true).or(site_options.first()).map(|s|s["value"].clone()).unwrap_or(json!(""));
        let name=format!("{} Defence Industries",me.name());
        let mut establish=intent("Establish a state contractor",json!({"kind":"company_establish","name":name,"district":site,"capital_mn":25.0}),vec![
            json!({"key":"name","label":"Company name","type":"text","value":name,"maxlength":64}),
            json!({"key":"district","label":"Existing arms-plant province","type":"select","value":site,"options":site_options}),
            company_cash_input("capital_mn","Initial working capital",25.0),
        ]);
        establish["detail"]=json!("A new state contractor leases one existing completed plant slot. Choose an explicit initial investment; its own account pays for tooling and finished stock.");
        actions.push(establish);
    } else {actions.push(nav("Design a tank",json!({"action":"equipment","tab":"designer"})));}
    actions.push(nav("Review procurement funding",json!({"action":"budget","ministry":"defense","department":3})));
    actions.push(nav("Review development funding",json!({"action":"budget","ministry":"defense","department":4})));
    actions.push(nav("Build an arms plant",json!({"action":"construction","kind":"arms_plant"})));
    let mut product_rows=vec![];
    let mut firm_rows=vec![];
    for firm in firms {
        let id=company_count(firm,"id");
        let mut firm_actions=vec![intent("Review additional investment",json!({"kind":"company_capitalize","company":id,"amount_mn":25.0}),vec![company_cash_input("amount_mn","Additional company capital",25.0)])];
        firm_actions.push(nav("Design a tank for this company",json!({"action":"equipment","tab":"designer"})));
        firm_actions.push(nav("Inspect domestic material availability",json!({"action":"resources"})));
        let district=company_text(firm,"district");
        let block=firm["facility_blocker"].as_str();
        let stock: u64=firm["products"].as_array().unwrap_or(&empty).iter().map(|p|company_count(p,"stock")).sum();
        firm_rows.push(json!({"id":id,"name":firm["name"],"status":if block.is_some(){"Facility unavailable"}else{"State contractor"},
            "detail":block.unwrap_or("A separate state-owned business. Its unsold stock adds no military strength and incurs no government fleet maintenance."),
            "metrics":[metric("Specialty","Tank development and manufacture"),metric("Company cash",company_money(company_amount(firm,"cash_bn"))),
                metric("Awaiting public settlement",company_money(company_amount(firm,"receivable_bn"))),metric("Available tanks",stock),metric("Company inventory at cost",company_money(company_amount(firm,"inventory_cost_bn"))),
                metric("Leased site",spheres_sim::districts::name_of(district).unwrap_or(district)),metric("Physical capacity","One existing arms-plant slot")],
            "costs":[cost("Capital received",company_amount(firm,"capital_received_bn"),"cumulative investment"),
                cost("Development receipts",company_amount(firm,"development_revenue_bn"),"cumulative engineering revenue"),
                cost("Equipment sales",company_amount(firm,"sales_revenue_bn"),"settled sales revenue"),
                cost("Operating and inventory costs",company_amount(firm,"expenses_bn"),"cumulative company payments; includes unsold inventory")],
            "actions":firm_actions}));
        for p in firm["products"].as_array().unwrap_or(&empty) {
            let pid=company_count(p,"id");let stock=company_count(p,"stock");let target=company_count(p,"stock_target");
            let revision=company_text(p,"revision_id");
            let certified=p["certified_day"].is_number();let cancelled=p["cancelled_day"].is_number();
            let dev_days=company_amount(p,"development_days");let dev_work=company_amount(p,"development_work_days");
            let restock=p["estimated_stock_days"].as_u64().map(|d|format!("About {d} days if cash, inputs and access hold")).unwrap_or_else(||company_text(p,"reason").to_string());
            let mut product_actions=vec![];
            let plan=eq::fleet_target_plans_world(w,me).into_iter().find(|f|f.revision==revision);
            if stock>0 && !cancelled {
                let quantity=plan.as_ref().filter(|p|p.desired.is_some()&&p.shortfall>0).map_or(1,|p|p.shortfall.min(stock));
                product_actions.push(intent("Review stock purchase",json!({"kind":"company_purchase","company":id,"product":pid,"quantity":quantity}),vec![
                    json!({"key":"quantity","label":"Available tanks to buy","type":"number","value":quantity,"min":1,"max":stock,"step":1})]));
            }
            if !cancelled && !certified {
                let budget=company_amount(p,"daily_budget_bn");
                product_actions.push(intent("Adjust development funding",json!({"kind":"company_funding","company":id,"product":pid,"daily_budget_mn":budget*1000.0}),vec![budget_input(budget)]));
                product_actions.push(intent("Cancel unfinished development",json!({"kind":"company_cancel","company":id,"product":pid}),vec![]));
            }
            if !cancelled {
                product_actions.push(intent("Review company stock target",json!({"kind":"company_inventory","company":id,"product":pid,"stock_target":target}),vec![company_inventory_input(target,false)]));
            }
            if certified {product_actions.push(target_action(w,me,revision));}
            let status=match company_text(p,"status") {"development"|"engineering"=>"Engineering","prototype"=>"Prototype development","trials"=>"Vehicle trials","tooling"=>"Preparing production","manufacturing"=>"Building company stock","stock"|"in_stock"=>"In stock","idle"=>"Stock target met","cancelled"=>"Development cancelled","blocked"=>"Needs attention",other=>other};
            let phase=match company_text(p,"status") {"development"|"engineering"|"prototype"|"trials"=>"development","in_stock"|"stock"=>"stock",other=>other};
            let mut metrics=vec![metric("Government development paid",company_money(company_amount(p,"development_spent_bn"))),
                metric("Company tooling paid",company_money(company_amount(p,"tooling_spent_bn"))),metric("Tanks manufactured",company_count(p,"produced_units")),metric("Tanks sold",company_count(p,"sold_units")),
                metric("Company stock target",target),metric("Company cash needed for next stock",company_money(company_amount(p,"company_cash_needed_bn"))),metric("Exact model revision",revision)];
            if let Some(plan)=&plan {metrics.push(metric("Government-owned incoming",plan.incoming));metrics.push(metric("Still needed for fleet target",plan.shortfall));}
            product_rows.push(json!({"id":format!("{}:{}",id,pid),"company":id,"product":pid,"name":p["name"],"supplier_name":firm["name"],
                "source_revision":revision,"spec":p["spec"],"phase":phase,"status":status,"detail":p["reason"],
                "progress":if !certified&&dev_days>0.0{Some((dev_work/dev_days).clamp(0.0,1.0))}else{None},
                "milestones":[{"label":"Engineering and trials","value":if certified{"Certified".to_string()}else{format!("{dev_work:.1} / {dev_days:.0} work days")},"detail":"Government-funded development"},
                    {"label":"Production readiness","value":format!("{:.1} / {} tooling days",company_amount(p,"tooling_work_days"),company_count(p,"tooling_days")),"detail":"Company-funded tooling"},
                    {"label":"Finished inventory","value":format!("{stock} available"),"detail":restock}],
                "availability":{"ready_stock":stock,"unit_price_bn":p["unit_price_bn"],"delivery_days":companies::DELIVERY_DAYS,
                    "restock":restock,"maintenance_bn_per_year":company_amount(p,"maintenance_bn_day")*365.0,
                    "fleet_need":plan.as_ref().and_then(|plan|plan.desired.map(|_|plan.shortfall))},
                "metrics":metrics,"costs":[cost("Development contract",company_amount(p,"development_cost_bn"),"government R&D · total"),
                    cost("Manufacturer unit price",company_amount(p,"unit_price_bn"),if stock>0{"per available tank · reviewed before purchase"}else{"indicative per tank · stock not yet available"})],"actions":product_actions}));
        }
    }
    let deliveries: Vec<_>=raw["deliveries"].as_array().unwrap_or(&empty).iter().rev().map(|d| {
        let revision=company_text(d,"revision_id");
        let name=w.nation(me).equipment.as_ref().and_then(|s|s.revisions.get(revision)).map_or(revision,|r|r.name.as_str());
        let delivered=d["delivered_day"].is_number();
        json!({"id":d["id"],"name":format!("{} · purchase #{}",name,company_count(d,"id")),
            "status":if delivered{"Delivered"}else if d["settled_day"].is_null(){"Awaiting payment settlement"}else if d["status"]=="blocked"{"Delivery paused"}else{"In transit"},
            "detail":d["reason"],"metrics":[metric("Purchased tanks",company_count(d,"quantity")),metric("Exact model",revision),
                metric("Delivery remaining",if delivered{"Arrived".into()}else if d["settled_day"].is_null(){"Begins after payment settlement".into()}else{d["remaining_days"].as_u64().map(|days|format!("{days} accessible days")).unwrap_or("Awaiting a current delivery estimate".into())})],
            "costs":[cost("Paid equipment price",company_amount(d,"total_price_bn"),"reviewed purchase total")],
            "actions":[nav("Review fleet in service",json!({"action":"equipment","tab":"service"}))]})
    }).collect();
    let total_stock:u64=product_rows.iter().map(|p|p["availability"]["ready_stock"].as_u64().unwrap_or(0)).sum();
    let total_cash:f64=firms.iter().map(|f|company_amount(f,"cash_bn")).sum();
    let warnings:Vec<_>=raw["reason"].as_str().filter(|r|!r.is_empty()).map(str::to_string).into_iter().collect();
    json!({"overview":{"title":"Companies & Procurement","status":if firms.is_empty(){"Establish your first manufacturer"}else{"Domestic tank procurement"},
        "detail":"Design the tank. Commission its development. Buy finished stock when your manufacturer has it ready.",
        "metrics":[metric("Manufacturers",firms.len()),metric("Tanks available to buy",total_stock),metric("Company working capital",company_money(total_cash)),
            metric("Government procurement available",company_money(company_amount(&raw,"procurement_available_bn"))),metric("Government R&D available",company_money(company_amount(&raw,"development_available_bn"))),
            metric("Background catalogue buying",if firms.is_empty(){"Existing procurement mode"}else{"Off · save funds for reviewed purchases"})],
        "roles_title":"Each payment has a purpose","roles":[
            {"label":"Development","value":"Government funds engineering and trials","detail":"The design becomes certified; prototypes do not enter service."},
            {"label":"Manufacturing","value":"Company funds its own finite stock","detail":"Capacity, cash and inputs constrain restocking."},
            {"label":"Purchase","value":"Government buys completed tanks","detail":"Ownership transfers once, then delivery makes them available for service."}],
        "warnings":warnings,"actions":actions},"firms":firm_rows,"products":product_rows,"deliveries":deliveries})
}

#[cfg(test)]
mod company_view_tests {
    use super::*;
    const ME:NationId=NationId::France;
    fn fixture()->(super::super::Game,String) {
        let mut g=super::super::Game::new(1990,Some(ME));
        super::super::play_rules(&mut g);
        spheres_sim::programs::set_construction_budget(&mut g.world,ME,0.0).unwrap();
        let site=g.world.districts.iter().find(|(_,owner)|**owner==ME).unwrap().0.clone();
        if let Some(p)=g.world.production.provinces.iter_mut().find(|p|p.district==site) {p.arms_plants=1;}
        else {g.world.production.provinces.push(spheres_sim::production::ProvinceCapabilities {district:site.clone(),arms_plants:1,infrastructure:0,civilian_industry:0,power_grid:0,research_centers:0});}
        // Explicit synthetic authority tests the interface, not opening balances.
        let n=g.world.nation_mut(ME);
        n.program_budget.as_mut().unwrap().available_bn[BUDGET_DEFENSE][3]=2.0;
        (g,site)
    }
    fn establish_command(site:&str)->Value {
        json!({"kind":"company_establish","name":"Interface Test Works","district":site,"capital_mn":25.0})
    }
    fn reviewed(g:&super::super::Game,command:Value)->Value {
        let p=preview(&g.world,ME,&g.session_id,&json!({"command":command})).unwrap();
        assert_eq!(p["valid"],true,"{}",p);
        assert!(p["actions"][0]["command"]["quote"].as_str().is_some_and(|t|!t.is_empty()));
        p["actions"][0]["command"].clone()
    }
    fn confirm(g:&mut super::super::Game,command:&Value) {
        let parsed=super::super::parse_command(&g.world,command,ME).unwrap();
        spheres_sim::apply_command(&mut g.world,&parsed).unwrap();
    }
    #[test]
    fn company_board_and_review_are_pure_and_do_not_grant_opening_assets() {
        let (g,site)=fixture();let before=spheres_sim::save(&g.world);
        let board=view(&g.world,ME,&g.session_id);
        assert!(board["companies"]["firms"].as_array().unwrap().is_empty());
        assert!(board["companies"]["products"].as_array().unwrap().is_empty());
        assert_eq!(board["companies"]["overview"]["actions"][0]["command"]["kind"],"company_establish");
        let c=reviewed(&g,establish_command(&site));
        assert_eq!(c["capital_mn"],25.0);
        assert_eq!(spheres_sim::save(&g.world),before);
    }
    #[test]
    fn company_confirmation_owns_one_slot_and_only_one_capital_receivable() {
        let (mut g,site)=fixture();let c=reviewed(&g,establish_command(&site));
        confirm(&mut g,&c);
        let firm=&g.world.companies.firms[0];
        assert_eq!(firm.cash_bn,0.0,"New public receipts are not spendable before settlement");
        assert_eq!(firm.receivables.len(),1);
        assert_eq!(firm.receivables[0].amount_bn,0.025);
        assert_eq!(spheres_sim::manufacturing::used_slots(&g.world,ME,&site),1);
        let before=spheres_sim::save(&g.world);
        let parsed=super::super::parse_command(&g.world,&c,ME).unwrap();
        assert!(spheres_sim::apply_command(&mut g.world,&parsed).is_err());
        assert_eq!(spheres_sim::save(&g.world),before);
        let board=view(&g.world,ME,&g.session_id);
        assert_eq!(board["companies"]["firms"].as_array().unwrap().len(),1);
        assert!(board["companies"]["products"].as_array().unwrap().is_empty());
    }
    #[test]
    fn company_parser_uses_player_authority_and_preserves_review_token() {
        let (g,site)=fixture();let mut c=reviewed(&g,establish_command(&site));
        c["nation"]=json!("USA");
        assert!(matches!(super::super::parse_command(&g.world,&c,ME),Some(Command::Company{nation:ME,order:CompanyOrder::Establish{capitalization_bn,quote,..}}) if capitalization_bn==0.025&&!quote.is_empty()));
        for value in [json!(1.5),json!(-1),json!(u64::MAX),json!("1")] {
            let p=json!({"kind":"company_purchase","company":value,"product":1,"quantity":1});
            assert!(super::super::parse_command(&g.world,&p,ME).is_none());
            let p=json!({"kind":"company_purchase","company":1,"product":1,"quantity":value});
            assert!(super::super::parse_command(&g.world,&p,ME).is_none());
        }
        c["quote"]=json!({"token":"forged"});
        assert!(super::super::parse_command(&g.world,&c,ME).is_none());
    }
    #[test]
    fn company_quote_token_expires_when_funding_or_capital_changes() {
        let (mut g,site)=fixture();let c=reviewed(&g,establish_command(&site));confirm(&mut g,&c);
        let id=g.world.companies.firms[0].id;
        let a=reviewed(&g,json!({"kind":"company_capitalize","company":id,"amount_mn":10.0}));
        let b=reviewed(&g,json!({"kind":"company_capitalize","company":id,"amount_mn":5.0}));
        confirm(&mut g,&b);let before=spheres_sim::save(&g.world);
        let parsed=super::super::parse_command(&g.world,&a,ME).unwrap();
        let error=spheres_sim::apply_command(&mut g.world,&parsed).unwrap_err();
        assert!(error.to_lowercase().contains("stale"),"{error}");
        assert_eq!(spheres_sim::save(&g.world),before);
    }
    #[test]
    fn tank_designer_commissions_supplier_and_retains_original_independent_specs() {
        let (mut g,site)=fixture();let s=eq::default_spec("tank_standard");
        let draft=json!({"name":"New domestic model","platform":s.platform,"components":s.components});
        let empty=preview(&g.world,ME,&g.session_id,&draft).unwrap();
        assert_eq!(empty["actions"][1]["navigate"]["tab"],"companies");
        let c=reviewed(&g,establish_command(&site));confirm(&mut g,&c);
        let priced=preview(&g.world,ME,&g.session_id,&draft).unwrap();
        let action=&priced["actions"][1];assert_eq!(action["command"]["kind"],"company_develop");
        assert_eq!(action["command"]["components"],draft["components"]);
        assert_eq!(action["inputs"][0]["type"],"select");
        let quote=preview(&g.world,ME,&g.session_id,&json!({"command":action["command"]})).unwrap();
        assert_eq!(quote["valid"],true,"{quote}");
        assert!(quote["costs"].as_array().unwrap().iter().any(|c|c["label"]=="Company capital needed for first stock target"));
        assert!(quote["timing"].as_array().unwrap().iter().any(|t|t["label"]=="Earliest first stock at this funding"));
    }
    #[test]
    fn lost_company_response_replays_receipt_without_creating_another_company() {
        let (mut g,site)=fixture();let c=reviewed(&g,establish_command(&site));
        let request=json!({"session_id":g.session_id,"client_id":"company-api-test","request_seq":1,"commands":[c]});
        let response=super::super::transport::immediate_request(&mut g,&request).unwrap();
        assert_eq!(response["errors"],json!([]));
        let before=spheres_sim::save(&g.world);
        let replay=super::super::transport::immediate_request(&mut g,&request).unwrap();
        assert_eq!(replay["command_replayed"],true);
        assert_eq!(spheres_sim::save(&g.world),before);
        assert_eq!(g.world.companies.firms.len(),1);
    }
}
