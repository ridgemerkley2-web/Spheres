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
        "company_enable_imports" => CompanyOrder::EnableImports { quote: token()? },
        "company_import_purchase" => CompanyOrder::ImportPurchase {
            seller: serde_json::from_value(v.get("seller")?.clone()).ok()?,
            company: integer("company")?, product: integer("product")?,
            ammunition: v.get("ammunition")?.as_bool()?, quantity: integer("quantity")?, quote: token()?,
        },
        "company_import_cancel" => CompanyOrder::CancelImport { contract: integer("contract")?, quote: token()? },
        "company_ammo_supply" => CompanyOrder::AmmoSupply {
            company: integer("company")?, family: text("family")?, stock_target: integer("stock_target")?, quote: token()?,
        },
        "company_ammo_inventory" => CompanyOrder::AmmoInventory {
            company: integer("company")?, product: integer("product")?, stock_target: integer("stock_target")?,
        },
        "company_ammo_purchase" => CompanyOrder::AmmoPurchase {
            company: integer("company")?, product: integer("product")?, quantity: integer("quantity")?, quote: token()?,
        },
        "company_refit" => CompanyOrder::Refit {
            company: integer("company")?, source: text("source")?, product: integer("product")?, quantity: integer("quantity")?, quote: token()?,
        },
        "company_refit_cancel" => CompanyOrder::CancelRefit {
            company: integer("company")?, refit: integer("refit")?, quote: token()?,
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
        CompanyOrder::EnableImports {..} =>
            serde_json::to_value(companies::import_enrollment_quote(w,me)).ok(),
        CompanyOrder::ImportPurchase {seller,company,product,ammunition,quantity,..} =>
            serde_json::to_value(companies::import_purchase_quote(w,me,*seller,*company,*product,*ammunition,*quantity)).ok(),
        CompanyOrder::CancelImport {contract,..} =>
            serde_json::to_value(companies::import_cancel_quote(w,me,*contract)).ok(),
        CompanyOrder::AmmoSupply {company,family,stock_target,..} =>
            serde_json::to_value(companies::ammo_supply_quote(w,me,*company,family,*stock_target)).ok(),
        CompanyOrder::AmmoPurchase {company,product,quantity,..} =>
            serde_json::to_value(companies::ammo_purchase_quote(w,me,*company,*product,*quantity)).ok(),
        CompanyOrder::Refit {company,source,product,quantity,..} =>
            serde_json::to_value(companies::refit_quote(w,me,*company,source,*product,*quantity)).ok(),
        CompanyOrder::CancelRefit {company,refit,..} =>
            serde_json::to_value(companies::refit_cancel_quote(w,me,*company,*refit)).ok(),
        _ => None,
    }
}

fn company_preview(w: &WorldState, me: NationId, session: &str, command: &Value, order: &CompanyOrder) -> Value {
    if matches!(order,CompanyOrder::EnableImports{..}|CompanyOrder::ImportPurchase{..}|CompanyOrder::CancelImport{..}) {
        return company_import_preview(w,me,session,command,order);
    }
    let quote = company_quote(w,me,order);
    let mut confirmed = command.clone();
    if let Some(q) = &quote {confirmed["quote"] = q["token"].clone();}
    let (label,detail) = match order {
        CompanyOrder::Establish {..} => ("Establish state contractor", "Create a state-owned equipment manufacturer with a separately funded account and rights to one existing arms-plant slot."),
        CompanyOrder::Capitalize {..} => ("Invest in manufacturer", "Transfer this additional investment to the company's account. It supports company-owned tooling and stock; it does not purchase equipment for the government."),
        CompanyOrder::Develop {..} => ("Commission vehicle development", "Commission the manufacturer to develop and test this exact design. Development funding buys certification and manufacturing rights; finished vehicles require a later purchase."),
        CompanyOrder::Purchase {..} => ("Buy available equipment", "Buy existing finished stock at this reviewed price. Purchased vehicles enter service only after payment settlement and delivery."),
        CompanyOrder::Funding {..} => ("Apply development funding", "Set the maximum the government can pay each day for actual contracted development work. Zero pauses future work and retains paid progress."),
        CompanyOrder::Inventory {..} => ("Set company stock target", "Set a finite finished-stock target. The state company uses its own cash and facility to replenish it; this does not authorize government equipment purchases."),
        CompanyOrder::CancelDevelopment {..} => ("Cancel unfinished development", "End remaining development work. Completed work and its payment remain recorded; cancellation does not refund spent engineering and trials costs."),
        CompanyOrder::AmmoSupply {..}|CompanyOrder::AmmoInventory {..}|CompanyOrder::AmmoPurchase {..} => return company_ammunition_preview(w,me,session,command,order),
        CompanyOrder::Refit {..}|CompanyOrder::CancelRefit {..} => return company_refit_preview(w,me,session,command,order),
        CompanyOrder::EnableImports {..}|CompanyOrder::ImportPurchase {..}|CompanyOrder::CancelImport {..} => unreachable!("Import reviews return above"),
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
        _=>unreachable!("Ammunition has its own review"),
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
            metrics.push(metric("Platform",eq::PLATFORMS.iter().find(|p|p.id==spec.platform).map_or(spec.platform.as_str(),|p|p.name)));
            metrics.push(metric("Initial company stock target",*stock_target));
            if let Some(profile) = eq::design_preview(w,me,spec).profile {
                metrics.extend(profile_metrics(&profile));
                if profile.aviation.is_some() {
                    requirements.push("The aircraft contract includes no bombs. Manufacture compatible mission stores separately; delivered aircraft also need maintenance and theatre basing access before tactical air raids.".into());
                } else if let Some(def)=eq::ammunition_family(spec).and_then(eq::ammo_def) {
                    metrics.push(metric("Compatible ammunition",def.name));
                }
                costs.push(cost("Development and trials",profile.development_cost_bn,"government R&D · paid as work progresses"));
                costs.push(cost("Company tooling commitment",profile.tooling_cost_bn,"company working capital after certification"));
                timing.push(metric("Engineering and trials minimum",format!("{} days",profile.development_days)));
                timing.push(metric("Tooling minimum after certification",format!("{} days",profile.tooling_days)));
                timing.push(metric("Fabrication per vehicle",format!("{} days after tooling",profile.production_days)));
                costs.push(cost("Maintenance after a later purchase",profile.maintenance_bn_day,"per delivered vehicle / day"));
            }
            costs.push(cost("Government development ceiling",*daily_budget_bn,"per day · shared R&D authority"));
            requirements.push("The contract freezes this model's specifications. A later design edit needs its own revision and development review.".into());
            requirements.push("After certification the company pays for production and acquires inputs into its own accounts. Its stock target reserves no government purchases.".into());
        },
        CompanyOrder::Purchase {quantity,..} => {
            metrics.push(metric("Units to buy",*quantity));
            if let Some((_,Some(product)))=selected {
                if let Some(model)=w.nation(me).equipment.as_ref().and_then(|s|s.revisions.get(&product.revision_id)) {
                    metrics.push(metric("Platform",eq::PLATFORMS.iter().find(|p|p.id==model.spec.platform).map_or(model.spec.platform.as_str(),|p|p.name)));
                    metrics.extend(profile_metrics(&model.profile));
                    if model.profile.aviation.is_some() {
                        metrics.push(metric("Mission stores included",0));
                        requirements.push("This purchase includes no bombs. Acquire the exact compatible mission-store family separately. Delivery alone grants no ready sorties: maintenance, stores and theatre basing access are required.".into());
                    } else if let Some(def)=eq::ammunition_family(&model.spec).and_then(eq::ammo_def) {
                        metrics.push(metric("Compatible ammunition",def.name));
                    }
                }
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
        _=>unreachable!("Ammunition has its own review"),
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
            if let Some(price)=q.get("unit_price_bn").and_then(Value::as_f64) {costs.push(cost("Supplier price",price,"per vehicle · fabrication and inputs included"));}
            if let Some(total)=q.get("cost_bn").and_then(Value::as_f64) {costs.push(cost("Total equipment purchase",total,"one purchase · Defense procurement"));}
            if let Some(stock)=q.get("available_units").and_then(Value::as_u64) {metrics.push(metric("Supplier stock before purchase",stock));}
            if let Some(days)=q.get("minimum_days").and_then(Value::as_u64) {timing.push(metric("Delivery after settlement",format!("{days} accessible days")));}
            if let Some(upkeep)=q.get("maintenance_bn_day").and_then(Value::as_f64) {costs.push(cost("Added maintenance requirement",upkeep,"per day after delivery · all purchased vehicles"));}
        }
        if matches!(order,CompanyOrder::Develop{..}) {
            if let Some(cash)=q.get("company_cash_needed_bn").and_then(Value::as_f64) {costs.push(cost("Company capital needed for first stock target",cash,"tooling, inputs and fabrication · current prices"));}
            if let Some(price)=q.get("unit_price_bn").and_then(Value::as_f64) {costs.push(cost("Indicative later purchase price",price,"per vehicle · reviewed again when stock is available"));}
            if let Some(days)=q.get("first_stock_days").and_then(Value::as_u64) {timing.push(metric("Earliest first stock at this funding",format!("About {days} days with company cash, inputs and access")));}
            else {timing.push(metric("First stock estimate","No dated estimate · review funding and earlier company work"));}
        }
    }
    let mut review_actions=vec![action];
    if matches!(order,CompanyOrder::Purchase{..}) && selected.and_then(|(_,p)|p).and_then(|p|eq::profile(w.nation(me),&p.revision_id)).is_some_and(|p|p.aviation.is_some()) {
        review_actions.push(nav("Prepare aircraft mission stores",json!({"action":"equipment","tab":"ammunition"})));
    }
    json!({"session_id":session,"nation":me,"date":w.date_str(),"as_of_day":spheres_sim::clock::absolute_day(w),"valid":blockers.is_empty(),"blockers":blockers,
        "metrics":metrics,"costs":costs,"timing":timing,"requirements":requirements,"service_effects":service_effects,"actions":review_actions,"detail":detail})
}

fn company_money(value: f64) -> String {
    if value.abs() < 1e-12 {return "$0".into();}
    if value.abs() >= 1.0 {format!("${value:.3}bn")}
    else if value.abs() >= 0.001 {format!("${:.2}m",value*1000.0)}
    else {format!("${:.2}k",value*1e6)}
}

fn company_import_preview(w:&WorldState,me:NationId,session:&str,command:&Value,order:&CompanyOrder)->Value {
    let quote=company_quote(w,me,order).expect("Import orders have authoritative simulation quotes");
    let (label,detail)=match order {
        CompanyOrder::EnableImports{..}=>("Enable reviewed equipment imports",
            "Open explicit purchases from foreign supplier stock. This also permits the modeled supplier programs to build and commission products using their own countries' existing budgets, cash and inputs. It grants no immediate manufacturer, equipment, research or inventory to you."),
        CompanyOrder::ImportPurchase{..}=>("Buy this foreign stock lot",
            "Purchase only the reviewed finished stock. The native contract retains the seller, exact equipment or ammunition, price and delivery terms. Equipment is usable after actual arrival; no component research or domestic manufacturing license is granted."),
        CompanyOrder::CancelImport{..}=>("Cancel undelivered imported stock",
            "Review the native contract's cancellation and refund terms. Delivered equipment remains owned; cancellation cannot create a second refund or renew spent budget authority."),
        _=>unreachable!("Only import orders enter this review"),
    };
    let mut confirmed=command.clone();confirmed["quote"]=quote["token"].clone();
    let mut action=checked(w,me,label,confirmed);
    let mut blockers:Vec<String>=action["reason"].as_str().map(str::to_owned).into_iter().collect();
    if quote["valid"]==false {
        if let Some(reason)=quote["reason"].as_str() {
            if !blockers.iter().any(|r|r==reason){blockers.push(reason.into());}
        }
    }
    if !blockers.is_empty(){action["enabled"]=json!(false);}
    let mut metrics=vec![];let mut costs=vec![];let mut timing=vec![];
    let mut requirements=vec![company_text(&quote,"note").to_owned()];
    match order {
        CompanyOrder::EnableImports{..}=>{
            metrics.push(metric("Equipment bought now",0));
            metrics.push(metric("Research granted",0));
            costs.push(cost("Quoted enrollment cost",company_amount(&quote,"cost_bn"),"current native enrollment quote"));
            requirements.push("No arms plant is required in the buying country. Foreign companies still need actual paid construction, legal designs, development, tooling and manufacturing before stock can be sold; their first product can take years.".into());
            requirements.push("Existing owned manufacturers, public production, completed assets and saved contracts retain their property. This is a prospective campaign choice, not an imported-stock grant.".into());
            requirements.push("Activate a departmental budget before enabling reviewed imports. Defense procurement funds vehicles; Maintenance & supply funds compatible ammunition. Applying the current allocations creates no extra cash or stock.".into());
        },
        CompanyOrder::ImportPurchase{seller,company,product,ammunition,quantity,..}=>{
            let firm=w.companies.firms.iter().find(|f|f.id==*company&&f.nation==*seller);
            metrics.push(metric("Supplier country",seller.name()));
            metrics.push(metric("Manufacturer",firm.map(|f|f.name.as_str()).unwrap_or("Unavailable supplier")));
            metrics.push(metric("Reviewed quantity",quantity));
            metrics.push(metric("Finished seller stock",company_count(&quote,"available_units")));
            metrics.push(metric("Component research granted",0));
            if *ammunition {
                if let Some(p)=firm.and_then(|f|f.ammunition_products.iter().find(|p|p.id==*product)) {
                    metrics.push(metric("Exact ammunition family",eq::ammo_def(&p.family).map_or(p.family.as_str(),|d|d.name)));
                    metrics.extend(company_ammo_need_metrics(&company_ammo_need(w,me,&p.family),&p.family));
                }
                costs.push(cost("Protected fleet upkeep",company_amount(&quote,"protected_maintenance_bn"),"unpaid upkeep protected before this purchase"));
                requirements.push("This lot does not include vehicles or activate ground ammunition rules. Physical stores remain unavailable until arrival and retain their exact compatible family.".into());
            } else if let Some(p)=firm.and_then(|f|f.products.iter().find(|p|p.id==*product)) {
                if let Some(model)=w.nation_opt(p.design_nation).and_then(|n|n.equipment.as_ref()).and_then(|s|s.revisions.get(&p.revision_id)) {
                    metrics.push(metric("Exact supplier model",&model.name));
                    metrics.push(metric("Frozen supplier revision",&model.id));
                    metrics.extend(profile_metrics(&model.profile));
                    let ammo=model.profile.aviation.as_ref().map(|a|a.store_family.as_str())
                        .or_else(||eq::ammunition_family(&model.spec));
                    if let Some(family)=ammo {metrics.push(metric("Compatible ammunition",eq::ammo_def(family).map_or(family,|d|d.name)));}
                }
                costs.push(cost("Additional fleet maintenance requirement",company_amount(&quote,"maintenance_bn_day"),"per day for the complete lot after delivery; not an advance charge"));
                requirements.push("The purchase buys finished equipment use, not the seller's component technologies or manufacturing rights. Compatible stores and normal maintenance remain separate needs; aircraft also require appropriate basing and mission access.".into());
            }
            costs.push(cost("Quoted supplier unit price",company_amount(&quote,"unit_price_bn"),"one finished vehicle, aircraft or ammunition unit"));
            costs.push(cost("Reviewed lot purchase",company_amount(&quote,"cost_bn"),if *ammunition{"one purchase · Maintenance & supply"}else{"one purchase · Defense procurement"}));
            costs.push(cost("Available purchase authority",company_amount(&quote,"purchase_available_bn"),"current native spending allowance, not company or Treasury cash"));
            timing.push(metric("Delivery after settlement",quote["eta_days"].as_u64().map(|d|format!("{d} accessible days if access holds")).unwrap_or("No current delivery estimate".into())));
            requirements.push("A stale price, changed stock or changed access requires a new review. A displayed market offer is not a reservation; an accepted contract records the actual purchased lot once.".into());
            requirements.push("Sanctions and access changes can block a new purchase or pause undelivered property. Use the contract's native reviewed cancellation quote for its exact refund timing.".into());
        },
        CompanyOrder::CancelImport{contract,..}=>{
            metrics.push(metric("Import contract",contract));
            costs.push(cost("Quoted cancellation refund",company_amount(&quote,"refund_bn"),"native refundable amount only; original spending authority is not replenished"));
            timing.push(metric("Refund timing",if quote["refund_after_settlement"]==true{"After the original payment settles"}else{"As specified by the current cancellation quote"}));
            requirements.push("The native cancellation returns the contract's undelivered property and refundable amount once. A delivered or previously cancelled lot cannot be purchased, returned or refunded again through this action.".into());
        },
        _=>unreachable!(),
    }
    json!({"session_id":session,"nation":me,"date":w.date_str(),"as_of_day":spheres_sim::clock::absolute_day(w),
        "valid":blockers.is_empty(),"blockers":blockers,"quote":quote,"metrics":metrics,"costs":costs,
        "timing":timing,"requirements":requirements,"detail":detail,
        "actions":[action,nav("Review and activate Defense funding",json!({"action":"budget","ministry":"defense","department":3})),
            nav("Review suppliers and deliveries",json!({"action":"equipment","tab":"companies"})),
            nav("Review delivered fleet and upkeep",json!({"action":"equipment","tab":"service"})),
            nav("Review compatible ammunition",json!({"action":"equipment","tab":"ammunition"}))]})
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

fn company_supplier_market(w:&WorldState,me:NationId)->Value {
    let enabled=companies::imports_enabled(w,me);
    let mut native_offers=companies::domestic_offers(w,me);
    native_offers.extend(companies::import_offers(w,me));
    let raw=serde_json::to_value(native_offers).unwrap_or_else(|_|json!([]));
    let mut offers=vec![];
    for row in raw.as_array().into_iter().flatten() {
        let company=company_count(row,"company");let product=company_count(row,"product");
        let ammunition=row["ammunition"]==true;let stock=company_count(row,"ready_stock");
        let review_quantity=company_count(row,"review_quantity");
        let platform=row["platform"].as_str().unwrap_or("");
        let product_name=if ammunition {row["family"].as_str().and_then(eq::ammo_def).map(|d|json!(d.name)).unwrap_or_else(||row["product_name"].clone())}else{row["product_name"].clone()};
        let unit=if ammunition {company_ammo_unit(row["family"].as_str().unwrap_or(""))}else{companies::unit_label(platform)};
        let seller=row["seller"].clone();
        let seller_name=serde_json::from_value::<NationId>(seller.clone()).map(|n|n.name().to_string()).unwrap_or_else(|_|seller.as_str().unwrap_or("Unknown country").to_string());
        let origin=if seller==json!(me){"domestic"}else{"foreign"};
        let mut actions=vec![];
        if review_quantity>0&&stock>0 {
            let command=if origin=="foreign" {json!({"kind":"company_import_purchase","seller":seller,
                "company":company,"product":product,"ammunition":ammunition,"quantity":review_quantity})}
                else {json!({"kind":if ammunition{"company_ammo_purchase"}else{"company_purchase"},
                    "company":company,"product":product,"quantity":review_quantity})};
            actions.push(intent("Review this stock lot",command,vec![json!({"key":"quantity","label":"Finished stock to review",
                "type":"number","value":review_quantity,"min":1,"max":stock,"step":1})]));
        }
        actions.push(nav("Review purchase funding",json!({"action":"budget","ministry":"defense","department":if ammunition{2}else{3}})));
        actions.push(nav("Review fleet upkeep and service",json!({"action":"equipment","tab":"service"})));
        let reason=row["reason"].as_str().unwrap_or(if stock>0{"This stock is available for a fresh native purchase review."}else{"The company has no finished units for sale yet. Paid company work must finish first."});
        let status=if stock==0{"Awaiting finished stock"}else if row["allowed"]==false{"Purchase currently blocked"}else{"Available for review"};
        offers.push(json!({"id":format!("supplier:{}:{company}:{}:{product}",seller.as_str().unwrap_or("unknown"),if ammunition{"ammo"}else{"equipment"}),
            "seller_nation":seller,"seller_name":seller_name,"company":company,"product":product,"origin":origin,
            "ammunition":ammunition,"supplier_name":row["supplier_name"],"name":product_name,
            "family":if ammunition{"ammunition"}else{companies::product_family(platform)},"ammo_family":row["family"],
            "platform":row["platform"],"platform_name":companies::platform_name(platform),"unit_label":unit,"spec":row["spec"],
            "phase":if stock>0{"stock"}else{"preparing"},"status":status,"detail":reason,
            "availability":{"ready_stock":stock,"unit_price_bn":if stock>0{row["unit_price_bn"].clone()}else{Value::Null},
                "unit":unit,
                "affordable_quantity":row["affordable_quantity"],"review_quantity":review_quantity,"delivery_days":row["delivery_days"],
                "purchase_available_bn":row["purchase_available_bn"],"protected_maintenance_bn":row["protected_maintenance_bn"]},
            "access":{"allowed":row["allowed"],"label":if row["allowed"]==true{"Purchase checks satisfied"}else{"Review current restriction"},"reason":row["reason"]},
            "metrics":[metric("Supplier country",seller_name),metric("Finished supplier stock",format!("{stock} {}",if unit=="aircraft"{unit.to_string()}else{format!("{unit}s")})),
                metric("Native affordable lot",row["affordable_quantity"].clone()),metric("Current purchase authority",company_money(company_amount(row,"purchase_available_bn")))],
            "costs":if stock>0{vec![cost("Native stock unit price",company_amount(row,"unit_price_bn"),"current finished inventory; final quantity and access are reviewed before purchase")]}else{vec![]},
            "service":{"label":"Ownership and service after arrival","detail":if ammunition{"Exact-family stores arrive separately; no equipment, new research or ground-ammunition activation is included."}else{"Normal maintenance and compatible stores remain necessary. A foreign purchase grants finished equipment use, not component research or domestic manufacturing rights."}},
            "actions":actions}));
    }
    let imports=serde_json::to_value(&w.companies.imports).unwrap_or(Value::Null);
    let deliveries:Vec<Value>=imports["contracts"].as_array().into_iter().flatten().filter(|row|row["buyer"]==json!(me)).rev().map(|row|{
        let seller_name=serde_json::from_value::<NationId>(row["seller"].clone()).map(|n|n.name().to_string()).unwrap_or_else(|_|company_text(row,"seller").to_string());
        let id=company_count(row,"id");let ammunition=row["ammunition"]==true;
        let delivered=row["delivered_day"].is_number();let cancelled=row["cancelled_day"].is_number();
        let refunded=row["refunded_day"].is_number();let settled=row["settled_day"].is_number();
        let model=&row["source_revision"];let family=row["family"].as_str().unwrap_or("");
        let name=if ammunition {eq::ammo_def(family).map_or(family,|d|d.name)}else{model["name"].as_str().unwrap_or("Imported equipment")};
        let mut actions=vec![];
        if !delivered&&!cancelled {actions.push(intent("Review cancellation and refund",json!({"kind":"company_import_cancel","contract":id}),vec![]));}
        actions.push(nav("Review delivered equipment and service",json!({"action":"equipment","tab":if ammunition{"ammunition"}else{"service"}})));
        json!({"id":format!("import-delivery:{id}"),"contract":id,"origin":"foreign","seller_nation":row["seller"],"seller_name":seller_name,
            "name":format!("{name} · import #{id}"),"family":if ammunition{"ammunition"}else{companies::product_family(model["spec"]["platform"].as_str().unwrap_or(""))},
            "spec":model["spec"],"ammo_family":row["family"],"source_revision":model["id"],"buyer_revision":row["buyer_revision"],
            "status":if delivered{"Delivered"}else if refunded{"Cancelled and refunded"}else if cancelled{"Cancelled · refund pending settlement"}else if !settled{"Awaiting payment settlement"}else if row["status"]=="blocked"{"Delivery paused"}else{"In transit"},
            "detail":row["reason"],"metrics":[metric("Seller country",seller_name),metric("Purchased lot",row["quantity"].clone()),
                metric("Current scheduled arrival",row["due_day"].as_i64().map(|day|super::settled_day_json(day as i32)["label"].clone()).unwrap_or(json!("No current date; access and settlement still apply")))],
            "costs":[cost("Reviewed import total",company_amount(row,"total_price_bn"),if settled{"original purchase; not a new payment"}else{"accepted purchase awaiting its original fiscal settlement"}),
                cost("Unreleased contract escrow",company_amount(row,"escrow_bn"),"already held for this contract, not a new bill"),
                cost("Refund recorded",company_amount(row,"refunded_bn"),"cumulative return for this contract only")],"actions":actions})
    }).collect();
    let mut actions=vec![];
    if !enabled {actions.push(intent("Enable reviewed imports",json!({"kind":"company_enable_imports"}),vec![]));}
    if w.supplier_catalogue.enabled && !w.rules.economic_competition {
        let mut action=nav("Review economic competition",json!({"action":"trade"}));
        action["detail"]=json!("Enable economic competition in the Exchange to schedule paid foreign supplier work. Importing stock that already exists is separate.");
        actions.push(action);
    }
    actions.push(nav("Review Defense procurement funding",json!({"action":"budget","ministry":"defense","department":3})));
    let catalogue=spheres_sim::supplier_catalogue::view(w);
    let programs:Vec<Value>=catalogue["programs"].as_array().into_iter().flatten().map(|row|{
        let status_name=|id:&str|match id {
            "construction"=>"Plant construction","capital_settlement"=>"Working capital settling",
            "development"|"engineering"=>"Development","prototype"=>"Prototype development","trials"=>"Trials and certification",
            "tooling"=>"Production tooling","manufacturing"|"producing"=>"Producing stock",
            "stock_available"|"in_stock"=>"Available stock","blocked"=>"Programme blocked","paused"=>"Economic competition paused",
            "player_managed"=>"Player managed","locally_managed"=>"Locally managed","not_enabled"=>"Not enabled",
            "inactive"=>"Government inactive","planned"=>"Awaiting scheduled review","ammunition_production"=>"Producing ammunition",
            "maintenance_setup"=>"Maintenance plan adopted",
            "cancelled"=>"Cancelled development","idle"=>"No work scheduled",
            _=>"Supplier work in progress"};
        let programme_status=status_name(row["status"].as_str().unwrap_or(""));
        let administration_status=if row["last_review_day"].is_number() {
            status_name(row["last_review_status"].as_str().unwrap_or(company_text(row,"status")))
        }else{"No review yet"};
        let live=row["products"].as_array().and_then(|p|p.first());
        let status=live.map(|p|status_name(company_text(p,"status"))).unwrap_or(programme_status);
        let mut milestones:Vec<Value>=row["facilities"].as_array().into_iter().flatten().map(|facility|json!({
            "label":facility["name"],"value":format!("{} completed levels",company_count(facility,"level")),
            "detail":if let Some(progress)=facility["progress"].as_f64(){format!("Funded construction {:.1}% complete. {}",progress*100.0,company_text(facility,"reason"))}
                else{company_text(facility,"reason").to_string()}})).collect();
        for product in row["products"].as_array().into_iter().flatten() {
            let work=|actual:&str,total:&str|match product[total].as_f64() {
                Some(total)=>format!("{:.2} / {total:.2} paid work-days",company_amount(product,actual)),
                None=>format!("{:.2} recorded paid work-days",company_amount(product,actual))};
            milestones.push(json!({"label":format!("Model #{} · development",company_count(product,"id")),
                "value":work("development_work_days","development_total_days"),
                "detail":format!("{} actual R&D paid. {}",company_money(company_amount(product,"development_spent_bn")),
                    product["certified_day"].as_i64().map(|d|format!("Certified {}.",super::settled_day_json(d as i32)["label"].as_str().unwrap_or("on its recorded day")))
                        .unwrap_or("Certification still requires actual work; this is not elapsed calendar time or an arrival promise.".into()))}));
            milestones.push(json!({"label":"Factory tooling","value":work("tooling_work_days","tooling_total_days"),
                "detail":"Actual funded tooling for this frozen model; stock production follows completed tooling."}));
            milestones.push(json!({"label":"Current next unit","value":work("production_work_days","production_total_days"),
                "detail":format!("{} units actually produced across this model's history; {} finished units still belong to the supplier.",company_count(product,"produced_units"),company_count(product,"ready_stock"))}));
        }
        milestones.push(json!({"label":"Last administrative review","value":administration_status,
            "detail":format!("{} · {}",row["last_review_day"].as_i64().map(|d|super::settled_day_json(d as i32)["label"].as_str().unwrap_or("Recorded day").to_string()).unwrap_or("No review yet".into()),row["last_review_reason"].as_str().unwrap_or(company_text(row,"reason")))}));
        json!({"id":format!("supplier-program:{}:{}",company_text(row,"nation"),company_text(row,"platform")),
            "seller_nation":row["nation"],"seller_name":row["nation_name"],"origin":if row["nation"].as_str()==Some(me.code()){"domestic"}else{"foreign"},
            "name":format!("{} · {}",company_text(row,"nation_name"),company_text(row,"platform_name")),
            "status":status,"status_id":live.map(|p|p["status"].clone()).unwrap_or_else(||row["status"].clone()),
            "detail":live.map(|p|p["reason"].clone()).unwrap_or_else(||row["reason"].clone()),
            "phase":if company_count(row,"ready_stock")>0{"stock"}else{"preparing"},"platform":row["platform"],
            "company":row["company"],"district":row["district"],"last_review_day":row["last_review_day"],
            "metrics":[metric("Finished supplier stock",row["ready_stock"].clone()),
                metric("Programme country",row["nation_name"].clone()),
                metric("Last actual programme review",row["last_review_day"].as_i64().map(|d|super::settled_day_json(d as i32)["label"].clone()).unwrap_or(json!("No review yet")))],"milestones":milestones,
            "products":row["products"],"actions":[]})
    }).collect();
    json!({"nation":me,"date":w.date_str(),"as_of_day":spheres_sim::clock::absolute_day(w),"enabled":enabled,
        "overview":{"title":"Supplier stock market","status":if enabled{"Reviewed domestic and foreign choice"}else{"Review import access"},
            "detail":"Compare actual company-owned inventory and the native affordable lot. A buyer needs no domestic arms plant. Suppliers must complete their own funded work before anything is available to purchase.",
            "metrics":[metric("Foreign import access",if enabled{"Enabled"}else{"Not yet adopted"})],
            "warnings":["Preparing suppliers and empty stock are not equipment offers with a promised arrival date. Enabling the market creates no inventory.","An offer is not reserved. Price, stock, funding and access are checked again by the native review and confirmation."],"actions":actions},
        "offers":offers,"deliveries":deliveries,"programs":programs,"program_note":catalogue["note"]})
}
fn company_development_action(w: &WorldState, me: NationId, name: &str, spec: &eq::DesignSpec) -> Value {
    let options: Vec<_> = w.companies.firms.iter().filter(|c|c.nation==me).map(|c| {
        let reason=companies::facility_blocker(w,c);
        json!({"value":c.id,"label":c.name,"enabled":reason.is_none(),
            "detail":format!("Ground vehicles and tactical aircraft · {} · {} company cash",spheres_sim::districts::name_of(&c.district).unwrap_or(&c.district),company_money(c.cash_bn)),"reason":reason})
    }).collect();
    if options.is_empty() {
        let mut action=nav("Establish an equipment manufacturer",json!({"action":"equipment","tab":"companies"}));
        action["detail"]=json!("Save this draft, then establish a funded contractor with an existing arms plant. Return here to commission development of this vehicle.");
        return action;
    }
    let firm=options[0]["value"].clone();
    let stock_target=if eq::is_aviation_platform(&spec.platform){2}else{4};
    let mut action=intent("Choose manufacturer and review development",json!({"kind":"company_develop","company":firm,"name":name,
        "platform":spec.platform,"components":spec.components,"daily_budget_mn":0.5,"stock_target":stock_target}),vec![
        json!({"key":"company","label":"Developing manufacturer","type":"select","value":firm,"options":options}),
        budget_input(0.0005),company_inventory_input(stock_target,true),
    ]);
    action["detail"]=json!("Review the engineering contract, first-stock estimate and company working capital before commissioning this frozen design. You buy completed vehicles later.");
    action
}
fn company_supplies_revision(w: &WorldState, me: NationId, revision: &str) -> bool {
    w.companies.firms.iter().any(|c|c.nation==me&&c.products.iter().any(|p|p.revision_id==revision&&p.cancelled_day.is_none()))
}
fn company_design_costs(p: &eq::CompiledProfile) -> Vec<Value> {
    vec![cost("Development and trials",p.development_cost_bn,"government R&D · one model"),
        cost("Manufacturer tooling",p.tooling_cost_bn,"company capital · once per licensed model"),
        cost("Company fabrication",p.fabrication_cost_bn,"per vehicle · company also purchases its inputs"),
        cost("Maintenance requirement",p.maintenance_bn_day,"per delivered vehicle / day · government support")]
}

fn company_board(w: &WorldState, me: NationId) -> Value {
    let raw=companies::view(w,me);
    let market=company_supplier_market(w,me);
    company_board_from_reads(w,me,&raw,&market)
}
fn company_board_from_reads(w:&WorldState,me:NationId,raw:&Value,market:&Value)->Value {
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
    } else {actions.push(nav("Design a vehicle",json!({"action":"equipment","tab":"designer"})));}
    actions.push(nav("Review procurement funding",json!({"action":"budget","ministry":"defense","department":3})));
    actions.push(nav("Review development funding",json!({"action":"budget","ministry":"defense","department":4})));
    actions.push(nav("Build an arms plant",json!({"action":"construction","kind":"arms_plant"})));
    let mut product_rows=vec![];
    let mut firm_rows=vec![];
    for firm in firms {
        let id=company_count(firm,"id");
        let mut firm_actions=vec![intent("Review additional investment",json!({"kind":"company_capitalize","company":id,"amount_mn":25.0}),vec![company_cash_input("amount_mn","Additional company capital",25.0)])];
        firm_actions.push(nav("Design a vehicle for this company",json!({"action":"equipment","tab":"designer","company":id})));
        firm_actions.push(nav("Inspect domestic material availability",json!({"action":"resources"})));
        let district=company_text(firm,"district");
        let block=firm["facility_blocker"].as_str();
        let stock: u64=firm["products"].as_array().unwrap_or(&empty).iter().map(|p|company_count(p,"stock")).sum();
        let service_escrow:f64=firm["refits"].as_array().unwrap_or(&empty).iter().map(|r|company_amount(r,"escrow_bn")).sum();
        let service_labor:f64=firm["refits"].as_array().unwrap_or(&empty).iter().map(|r|company_amount(r,"working_capital_locked_bn")).sum();
        firm_rows.push(json!({"id":id,"name":firm["name"],"status":if block.is_some(){"Facility unavailable"}else{"State contractor"},
            "detail":block.unwrap_or("A separate state-owned business. Its unsold stock adds no military strength and incurs no government fleet maintenance."),
            "metrics":[metric("Specialty","Ground vehicles, tactical aircraft and ammunition"),metric("Company cash",company_money(company_amount(firm,"cash_bn"))),
                metric("Awaiting public settlement",company_money(company_amount(firm,"receivable_bn"))),metric("Available equipment",stock),metric("Company inventory at cost",company_money(company_amount(firm,"inventory_cost_bn"))),
                metric("Leased site",spheres_sim::districts::name_of(district).unwrap_or(district)),metric("Physical capacity","One existing arms-plant slot"),
                metric("Customer refit payments held",company_money(service_escrow)),metric("Company labor capital reserved",company_money(service_labor))],
            "costs":[cost("Capital received",company_amount(firm,"capital_received_bn"),"cumulative investment"),
                cost("Development receipts",company_amount(firm,"development_revenue_bn"),"cumulative engineering revenue"),
                cost("Equipment and ammunition sales",company_amount(firm,"sales_revenue_bn"),"settled sales revenue"),
                cost("Refit service fees earned",company_amount(firm,"refit_revenue_bn"),"only completed conversions · held advances excluded"),
                cost("Operating and inventory costs",company_amount(firm,"expenses_bn"),"cumulative company payments; includes unsold inventory")],
            "actions":firm_actions}));
        for p in firm["products"].as_array().unwrap_or(&empty) {
            let pid=company_count(p,"id");let stock=company_count(p,"stock");let target=company_count(p,"stock_target");
            let revision=company_text(p,"revision_id");
            let platform=company_text(&p["spec"],"platform");
            let certified=p["certified_day"].is_number();let cancelled=p["cancelled_day"].is_number();
            let dev_days=company_amount(p,"development_days");let dev_work=company_amount(p,"development_work_days");
            let restock=p["estimated_stock_days"].as_u64().map(|d|format!("About {d} days if cash, inputs and access hold")).unwrap_or_else(||if !cancelled&&stock<target {format!("No dated estimate. {}",company_text(p,"estimate_note"))}else{company_text(p,"reason").to_string()});
            let mut product_actions=vec![];
            let plan=eq::fleet_target_plans_world(w,me).into_iter().find(|f|f.revision==revision);
            if stock>0 && !cancelled {
                let quantity=plan.as_ref().filter(|p|p.desired.is_some()&&p.shortfall>0).map_or(1,|p|p.shortfall.min(stock));
                product_actions.push(intent("Review stock purchase",json!({"kind":"company_purchase","company":id,"product":pid,"quantity":quantity}),vec![
                    json!({"key":"quantity","label":"Available units to buy","type":"number","value":quantity,"min":1,"max":stock,"step":1})]));
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
            if eq::is_aviation_platform(platform) {product_actions.push(nav("Prepare aircraft mission stores",json!({"action":"equipment","tab":"ammunition"})));}
            let status=match company_text(p,"status") {"development"|"engineering"=>"Engineering","prototype"=>"Prototype development","trials"=>"Vehicle trials","tooling"=>"Preparing production","manufacturing"=>"Building company stock","stock"|"in_stock"=>"In stock","idle"=>"Stock target met","cancelled"=>"Development cancelled","blocked"=>"Needs attention",other=>other};
            let phase=match company_text(p,"status") {"development"|"engineering"|"prototype"|"trials"=>"development","in_stock"|"stock"=>"stock",other=>other};
            let mut metrics=vec![metric("Platform",eq::PLATFORMS.iter().find(|def|def.id==platform).map_or(platform,|def|def.name)),metric("Government development paid",company_money(company_amount(p,"development_spent_bn"))),
                metric("Company tooling paid",company_money(company_amount(p,"tooling_spent_bn"))),metric("Units manufactured",company_count(p,"produced_units")),metric("Units sold",company_count(p,"sold_units")),
                metric("Company stock target",target),metric("Company cash needed for next stock",company_money(company_amount(p,"company_cash_needed_bn"))),metric("Exact model revision",revision)];
            if let Some(plan)=&plan {metrics.push(metric("Government-owned incoming",plan.incoming));metrics.push(metric("Still needed for fleet target",plan.shortfall));}
            if let Some(air)=eq::profile(w.nation(me),revision).and_then(|profile|profile.aviation.as_ref()) {
                metrics.push(metric("Compatible mission stores",eq::ammo_def(&air.store_family).map_or(air.store_family.as_str(),|def|def.name)));
                metrics.push(metric("Mission stores included","None · acquire separately"));
            }
            product_rows.push(json!({"id":format!("{}:{}",id,pid),"company":id,"product":pid,"name":p["name"],"supplier_name":firm["name"],
                "family":p["family"],"platform_name":p["platform_name"],"unit_label":p["unit_label"],
                "source_revision":revision,"spec":p["spec"],"phase":phase,"status":status,"detail":p["reason"],
                "progress":if !certified&&dev_days>0.0{Some((dev_work/dev_days).clamp(0.0,1.0))}else{None},
                "milestones":[{"label":"Engineering and trials","value":if certified{"Certified".to_string()}else{format!("{dev_work:.1} / {dev_days:.0} work days")},"detail":"Government-funded development"},
                    {"label":"Production readiness","value":format!("{:.1} / {} tooling days",company_amount(p,"tooling_work_days"),company_count(p,"tooling_days")),"detail":"Company-funded tooling"},
                    {"label":"Finished inventory","value":format!("{stock} available"),"detail":restock}],
                "availability":{"ready_stock":stock,"unit_price_bn":p["unit_price_bn"],"delivery_days":companies::DELIVERY_DAYS,
                    "restock":restock,"maintenance_bn_per_year":company_amount(p,"maintenance_bn_day")*365.0,
                    "fleet_need":plan.as_ref().and_then(|plan|plan.desired.map(|_|plan.shortfall))},
                "metrics":metrics,"costs":[cost("Development contract",company_amount(p,"development_cost_bn"),"government R&D · total"),
                    cost("Manufacturer unit price",company_amount(p,"unit_price_bn"),if stock>0{"per available vehicle · reviewed before purchase"}else{"indicative per vehicle · stock not yet available"})],"actions":product_actions}));
        }
    }
    let mut deliveries: Vec<_>=raw["deliveries"].as_array().unwrap_or(&empty).iter().rev().map(|d| {
        let revision=company_text(d,"revision_id");
        let model=w.nation(me).equipment.as_ref().and_then(|s|s.revisions.get(revision));
        let name=model.map_or(revision,|r|r.name.as_str());
        let delivered=d["delivered_day"].is_number();
        json!({"id":d["id"],"name":format!("{} · purchase #{}",name,company_count(d,"id")),
            "family":d["family"],"platform_name":d["platform_name"],"unit_label":d["unit_label"],"spec":model.map(|r|&r.spec),"source_revision":revision,
            "status":if delivered{"Delivered"}else if d["settled_day"].is_null(){"Awaiting payment settlement"}else if d["status"]=="blocked"{"Delivery paused"}else{"In transit"},
            "detail":d["reason"],"metrics":[metric("Purchased units",company_count(d,"quantity")),metric("Exact model",revision),
                metric("Delivery remaining",if delivered{"Arrived".into()}else if d["settled_day"].is_null(){"Begins after payment settlement".into()}else{d["remaining_days"].as_u64().map(|days|format!("{days} accessible days")).unwrap_or("Awaiting a current delivery estimate".into())})],
            "costs":[cost("Paid equipment price",company_amount(d,"total_price_bn"),"reviewed purchase total")],
            "actions":[nav("Review fleet in service",json!({"action":"equipment","tab":"service"}))]})
    }).collect();
    let total_stock:u64=product_rows.iter().map(|p|p["availability"]["ready_stock"].as_u64().unwrap_or(0)).sum();
    let (ammunition_products,ammunition_deliveries)=company_ammo_rows(w,me,&raw);
    product_rows.extend(ammunition_products);deliveries.extend(ammunition_deliveries);
    actions.push(nav("Buy ammunition and manage reserves",json!({"action":"equipment","tab":"ammunition"})));
    let total_cash:f64=firms.iter().map(|f|company_amount(f,"cash_bn")).sum();
    let warnings:Vec<_>=raw["reason"].as_str().filter(|r|!r.is_empty()).map(str::to_string).into_iter().collect();
    json!({"nation":me,"date":w.date_str(),"as_of_day":spheres_sim::clock::absolute_day(w),
        "overview":{"title":"Companies & Procurement","status":if firms.is_empty(){"Establish your first manufacturer"}else{"Domestic equipment procurement"},
        "detail":"Design the vehicle and commission its development. Buy finished equipment and compatible ammunition from your manufacturer's stock.",
        "metrics":[metric("Manufacturers",firms.len()),metric("Equipment available to buy",total_stock),metric("Company working capital",company_money(total_cash)),
            metric("Government procurement available",company_money(company_amount(&raw,"procurement_available_bn"))),metric("Government R&D available",company_money(company_amount(&raw,"development_available_bn"))),
            metric("Background catalogue buying",if firms.is_empty(){"Existing procurement mode"}else{"Off · save funds for reviewed purchases"})],
        "roles_title":"Each payment has a purpose","roles":[
            {"label":"Development","value":"Government funds engineering and trials","detail":"The design becomes certified; prototypes do not enter service."},
            {"label":"Manufacturing","value":"Company funds its own finite stock","detail":"Capacity, cash and inputs constrain restocking."},
            {"label":"Purchase","value":"Government buys finished equipment and ammunition","detail":"Vehicle purchases use procurement; ammunition uses Maintenance & supply after fleet upkeep. Delivery makes each purchase available."}],
        "warnings":warnings,"actions":actions},"firms":firm_rows,"products":product_rows,"deliveries":deliveries,"market":market,
        "service_overview":company_service_overview(),"services":company_refit_rows(w,me,&raw)})
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
    fn enroll_current_budget(g:&mut super::super::Game,me:NationId) {
        let daily_budget_bn=spheres_sim::programs::construction_daily_budget_bn(&g.world,me);
        let allocations=g.world.nation(me).budget_for(g.world.year).allocations;
        let departments=g.world.nation(me).program_budget.as_ref().map(|p|p.departments);
        spheres_sim::apply_command(&mut g.world,&Command::SetConstructionBudget{nation:me,daily_budget_bn}).unwrap();
        assert_eq!(g.world.nation(me).budget_for(g.world.year).allocations,allocations);
        if let Some(shares)=departments {assert_eq!(g.world.nation(me).program_budget.as_ref().unwrap().departments,shares);}
        assert_eq!(spheres_sim::programs::construction_daily_budget_bn(&g.world,me),daily_budget_bn);
    }
    fn review_tonga_procurement_priority(g:&mut super::super::Game)->Value {
        use spheres_sim::world::BUDGET_INFRASTRUCTURE;
        let me=NationId::Tonga;let n=g.world.nation(me);
        let original=n.budget_for(g.world.year).allocations;
        let old_plan=n.program_budget.clone().expect("Activate the ordinary departmental plan first");
        let mut allocations=original;allocations[BUDGET_INFRASTRUCTURE]-=0.025;allocations[BUDGET_DEFENSE]+=0.025;
        let mut departments=old_plan.departments;departments[BUDGET_DEFENSE]=[600,600,700,8000,100];
        assert_eq!(allocations.iter().sum::<f64>(),original.iter().sum::<f64>(),"This declared policy reallocates a fixed total budget");
        for d in 0..3 {assert!(allocations[BUDGET_DEFENSE]*f64::from(departments[BUDGET_DEFENSE][d])
            >=original[BUDGET_DEFENSE]*f64::from(old_plan.departments[BUDGET_DEFENSE][d]),"Personnel, operations and maintenance shares of GDP must be retained");}
        let mut payload=json!({"kind":"program_budget","fiscal_year":g.world.year,"departments":departments});
        for (i,value) in allocations.iter().enumerate(){payload[super::super::ministry_key(i)]=json!(value);}
        let before=spheres_sim::save(&g.world);
        let review=super::super::program_preview_json(&g.world,me,&payload).unwrap();
        let native=spheres_sim::programs::preview_with_plan(&g.world,me,allocations,departments).unwrap();
        let annual_procurement_bn=native.rows.iter().find(|r|r.ministry==BUDGET_DEFENSE&&r.department==3).unwrap().annual_bn;
        assert!(spheres_sim::save(&g.world)==before,"The actual budget review must remain pure");
        let money=(n.treasury_bn,n.debt_bn);let political=n.political_capital;
        let owned=serde_json::to_value((&n.arsenal,&n.equipment,&n.tech,&g.world.companies,&g.world.production)).unwrap();
        let command=super::super::parse_command(&g.world,&payload,me).unwrap();
        spheres_sim::apply_command(&mut g.world,&command).unwrap();
        let n=g.world.nation(me);let after=n.program_budget.as_ref().unwrap();
        assert_eq!((n.treasury_bn,n.debt_bn),money);
        assert_eq!(serde_json::to_value((&n.arsenal,&n.equipment,&n.tech,&g.world.companies,&g.world.production)).unwrap(),owned);
        assert_eq!(after.available_bn,old_plan.available_bn);assert_eq!(after.prepaid_bn,old_plan.prepaid_bn);
        assert_eq!(after.spent_today_bn,old_plan.spent_today_bn);assert_eq!(after.spent_ytd_bn,old_plan.spent_ytd_bn);
        assert_eq!(political-n.political_capital,native.political_cost);
        json!({"command":payload,"native_preview":review,"political_cost":native.political_cost,
            "total_before":original.iter().sum::<f64>(),"total_after":allocations.iter().sum::<f64>(),
            "old_allocations":original,"chosen_allocations":allocations,"chosen_departments":departments,
            "annual_procurement_bn_at_review":annual_procurement_bn,
            "no_immediate_cash_debt_authority_property_or_research_grant":true,
            "tradeoff":"Transfer 2.5 percentage points of GDP from Infrastructure to Defense, prioritising procurement while preserving absolute personnel, operations and maintenance funding. This deliberately reduces infrastructure appropriations; future authority must accrue through actual days."})
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
    #[test]
    fn every_current_platform_commissions_its_exact_model_through_a_company() {
        let (mut g,site)=fixture();let c=reviewed(&g,establish_command(&site));confirm(&mut g,&c);
        let held_before=serde_json::to_value(&g.world.nation(ME).arsenal).unwrap();
        let mut counts=(0,0);
        for platform in eq::PLATFORMS {
            let spec=eq::default_spec(platform.id);
            let draft=json!({"name":format!("Supplier {}",platform.name),"platform":spec.platform,"components":spec.components});
            let before=spheres_sim::save(&g.world);
            let proposed=preview(&g.world,ME,&g.session_id,&draft).unwrap();
            assert_eq!(proposed["valid"],true,"{}: {proposed}",platform.name);
            let action=&proposed["actions"][1];
            assert_eq!(action["command"]["kind"],"company_develop");
            assert_eq!(action["command"]["components"],draft["components"]);
            let quote=preview(&g.world,ME,&g.session_id,&json!({"command":action["command"]})).unwrap();
            assert_eq!(quote["valid"],true,"{}: {quote}",platform.name);
            let profile=eq::design_preview(&g.world,ME,&spec).profile.unwrap();
            assert_eq!(quote["costs"][0]["amount_bn"],profile.development_cost_bn);
            assert_eq!(spheres_sim::save(&g.world),before,"Review changes no money, assets or work");
            confirm(&mut g,&quote["actions"][0]["command"]);
            let product=g.world.companies.firms[0].products.last().unwrap();
            let revision=product.revision_id.clone();let product_id=product.id;
            let board=company_board(&g.world,ME);
            let row=board["products"].as_array().unwrap().iter().find(|p|p["source_revision"]==revision).unwrap();
            assert_eq!(row["spec"],serde_json::to_value(&spec).unwrap());
            assert_eq!(row["platform_name"],platform.name);
            assert_eq!(row["availability"]["ready_stock"],0);
            assert_eq!(row["availability"]["maintenance_bn_per_year"],profile.maintenance_bn_day*365.0);
            if eq::is_aviation_platform(platform.id) {
                counts.1+=1;assert_eq!(row["family"],"aircraft");assert_eq!(row["unit_label"],"aircraft");
                assert_eq!(action["command"]["stock_target"],2);
                assert!(quote["metrics"].as_array().unwrap().iter().any(|m|m["label"]=="Compatible mission stores"));
                assert!(quote["requirements"].as_array().unwrap().iter().any(|r|r.as_str().unwrap().contains("includes no bombs")));
                assert!(row["actions"].as_array().unwrap().iter().any(|a|a["navigate"]["tab"]=="ammunition"));
                assert!(aviation_board(&g.world,ME)["actions"].as_array().unwrap().iter().any(|a|a["navigate"]["tab"]=="companies"));
            } else {counts.0+=1;assert_eq!(row["family"],"ground");}
            assert_eq!(serde_json::to_value(&g.world.nation(ME).arsenal).unwrap(),held_before,"Development grants no usable equipment");
            assert!(g.world.nation(ME).equipment.as_ref().unwrap().ammunition.is_none(),"A vehicle contract grants no ammunition");
            let cancelled=json!({"kind":"company_cancel","company":g.world.companies.firms[0].id,"product":product_id});
            confirm(&mut g,&cancelled);
        }
        assert_eq!(counts,(9,2));
    }

    fn foreign_stock_fixture()->(super::super::Game,u32,u32) {
        let (mut g,site)=fixture();let formed=reviewed(&g,establish_command(&site));confirm(&mut g,&formed);
        let company=g.world.companies.firms[0].id;
        let spec=eq::default_spec("ground_apc");
        let development=reviewed(&g,json!({"kind":"company_develop","company":company,
            "name":"Recorded seller APC","platform":spec.platform,"components":spec.components,
            "daily_budget_mn":0.1,"stock_target":2}));
        confirm(&mut g,&development);
        let today=spheres_sim::clock::absolute_day(&g.world);
        let product=g.world.companies.firms[0].products[0].id;
        let revision=g.world.companies.firms[0].products[0].revision_id.clone();
        // Explicit recorded-stock fixture isolates adapter identity and reviews.
        // Native S08 tests and browser journeys own paid stock construction.
        g.world.nation_mut(ME).equipment.as_mut().unwrap().revisions.get_mut(&revision).unwrap().certified_day=Some(today);
        let p=&mut g.world.companies.firms[0].products[0];
        p.certified_day=Some(today);p.stock=2;p.produced_units=2;p.stock_cost_bn=0.002;
        let buyer=NationId::Tonga;g.world.player=Some(buyer);
        enroll_current_budget(&mut g,buyer);
        g.world.nation_mut(buyer).program_budget.as_mut().unwrap().available_bn[BUDGET_DEFENSE][3]=0.01;
        let q=companies::import_enrollment_quote(&g.world,buyer);assert!(q.valid,"{:?}",q.reason);
        companies::apply(&mut g.world,buyer,&CompanyOrder::EnableImports{quote:q.token}).unwrap();
        (g,company,product)
    }

    #[test]
    fn s08_request_local_supplier_reads_preserve_both_board_sections() {
        let (mut g,company,product)=foreign_stock_fixture();
        for phase in ["stock","purchased"] {
            if phase=="purchased" {
                let q=companies::import_purchase_quote(&g.world,NationId::Tonga,ME,company,product,false,1);
                assert!(q.valid);
                spheres_sim::apply_command(&mut g.world,&Command::Company {nation:NationId::Tonga,
                    order:CompanyOrder::ImportPurchase {seller:ME,company,product,ammunition:false,quantity:1,quote:q.token}}).unwrap();
            }
            let before=spheres_sim::save(&g.world);
            for me in [ME,NationId::Tonga] {
                // Independent standalone reads reconstruct the two former
                // call paths; the response now shares their inputs once.
                let independent_company=company_board(&g.world,me);
                let independent_ammunition=ammunition_board(&g.world,me);
                let combined=view(&g.world,me,&g.session_id);
                assert_eq!(combined["companies"],independent_company,"{phase} {me:?}");
                assert_eq!(combined["ammunition"],independent_ammunition,"{phase} {me:?}");
            }
            assert_eq!(spheres_sim::save(&g.world),before,"Shared supplier reads changed {phase}");
        }
    }
    #[test]
    fn s08_no_factory_country_can_review_import_access_without_any_opening_stock_grant() {
        let mut g=super::super::Game::new(1990,Some(NationId::Tonga));
        super::super::fresh_play_rules(&mut g).unwrap();
        let me=NationId::Tonga;let before=spheres_sim::save(&g.world);
        let market=company_supplier_market(&g.world,me);
        assert_eq!(market["nation"],json!(me));assert_eq!(market["date"],g.world.date_str());
        assert_eq!(market["as_of_day"],spheres_sim::clock::absolute_day(&g.world));
        assert!(market["offers"].as_array().unwrap().is_empty());
        assert!(market["deliveries"].as_array().unwrap().is_empty());
        assert!(!market["programs"].as_array().unwrap().is_empty());
        assert!(market["programs"].as_array().unwrap().iter().all(|p|p["status"]=="Economic competition paused"));
        assert!(market["overview"]["actions"].as_array().unwrap().iter().any(|a|a["navigate"]["action"]=="trade"));
        let blocked=preview(&g.world,me,&g.session_id,&json!({"command":{"kind":"company_enable_imports"}})).unwrap();
        assert_eq!(blocked["valid"],false,"An unenrolled budget must not be invented by a preview");
        assert!(blocked["actions"].as_array().unwrap().iter().any(|a|a["navigate"]["action"]=="budget"));
        assert_eq!(spheres_sim::save(&g.world),before);
        enroll_current_budget(&mut g,me);
        let before=spheres_sim::save(&g.world);
        let review=preview(&g.world,me,&g.session_id,&json!({"command":{"kind":"company_enable_imports"}})).unwrap();
        assert_eq!(review["valid"],true,"{review}");assert_eq!(review["date"],g.world.date_str());
        assert_eq!(review["quote"]["token"],companies::import_enrollment_quote(&g.world,me).token);
        assert_eq!(spheres_sim::save(&g.world),before,"Market and enrollment review may not perform supplier work");
        let arsenal=serde_json::to_value(&g.world.nation(me).arsenal).unwrap();
        let tech=serde_json::to_value(&g.world.nation(me).tech).unwrap();
        let day=spheres_sim::clock::absolute_day(&g.world);
        let command=super::super::parse_command(&g.world,&review["actions"][0]["command"],me).unwrap();
        spheres_sim::apply_command(&mut g.world,&command).unwrap();
        assert!(companies::imports_enabled(&g.world,me));assert!(g.world.companies.firms.is_empty());
        assert!(!g.world.rules.economic_competition,"Import adoption must not silently turn on other economic AI");
        assert!(g.world.companies.imports.contracts.is_empty());
        assert_eq!(serde_json::to_value(&g.world.nation(me).arsenal).unwrap(),arsenal);
        assert_eq!(serde_json::to_value(&g.world.nation(me).tech).unwrap(),tech);
        assert_eq!(spheres_sim::clock::absolute_day(&g.world),day);
        assert!(g.world.districts.iter().filter(|(_,n)|**n==me).all(|(d,_)|spheres_sim::production::level(&g.world,d,spheres_sim::production::ProjectKind::ArmsPlant)==0));
    }

    #[test]
    fn s08_import_parser_keeps_foreign_seller_separate_from_authenticated_buyer() {
        let (g,_)=fixture();let buyer=NationId::Tonga;
        let command=json!({"kind":"company_import_purchase","nation":"France","seller":"France",
            "company":12,"product":19,"ammunition":false,"quantity":2,"quote":"reviewed"});
        assert!(matches!(super::super::parse_command(&g.world,&command,buyer),
            Some(Command::Company{nation,order:CompanyOrder::ImportPurchase{seller:ME,company:12,product:19,ammunition:false,quantity:2,quote}})
                if nation==buyer&&quote=="reviewed"));
        for (key,bad) in [("seller",json!("No such nation")),("company",json!(-1)),("product",json!(4294967296u64)),
            ("ammunition",json!("false")),("quantity",json!(1.5)),("quote",json!("x".repeat(257)))] {
            let mut malformed=command.clone();malformed[key]=bad;
            assert!(parse_company_order(&malformed).is_none(),"Malformed {key} was accepted");
        }
    }

    #[test]
    fn s08_market_preserves_native_stock_affordability_and_foreign_identity_in_pure_reads() {
        let (g,company,product)=foreign_stock_fixture();let buyer=NationId::Tonga;
        let before=spheres_sim::save(&g.world);let native=companies::import_offers(&g.world,buyer);
        let expected=native.iter().find(|row|row.company==company&&row.product==product&&!row.ammunition).unwrap();
        let market=company_supplier_market(&g.world,buyer);
        let row=market["offers"].as_array().unwrap().iter().find(|row|row["company"]==company&&row["product"]==product).unwrap();
        assert_eq!(row["origin"],"foreign");assert_eq!(row["seller_nation"],json!(ME));
        assert_eq!(row["availability"]["ready_stock"],expected.ready_stock);
        assert_eq!(row["availability"]["unit_price_bn"],expected.unit_price_bn);
        assert_eq!(row["availability"]["affordable_quantity"],expected.affordable_quantity);
        assert_eq!(row["availability"]["review_quantity"],expected.review_quantity);
        assert_eq!(row["access"]["allowed"],expected.allowed);
        assert_eq!(row["spec"],serde_json::to_value(&expected.spec).unwrap());
        assert_eq!(row["actions"][0]["command"]["kind"],"company_import_purchase");
        let domestic=company_supplier_market(&g.world,ME);
        assert!(domestic["offers"].as_array().unwrap().iter().any(|row|row["origin"]=="domestic"&&row["company"]==company));
        assert_eq!(spheres_sim::save(&g.world),before);
    }

    #[test]
    fn s08_import_purchase_and_cancellation_reviews_retain_exact_native_quotes_and_no_research_grant() {
        let (mut g,company,product)=foreign_stock_fixture();let buyer=NationId::Tonga;
        let command=json!({"kind":"company_import_purchase","seller":"France","company":company,
            "product":product,"ammunition":false,"quantity":1});
        let before=spheres_sim::save(&g.world);
        let quote=companies::import_purchase_quote(&g.world,buyer,ME,company,product,false,1);
        let reviewed=preview(&g.world,buyer,&g.session_id,&json!({"command":command})).unwrap();
        assert_eq!(reviewed["valid"],quote.valid);assert!(quote.valid,"{:?}",quote.reason);
        assert_eq!(reviewed["quote"]["cost_bn"],quote.cost_bn);
        assert_eq!(reviewed["actions"][0]["command"]["quote"],quote.token);
        assert_eq!(spheres_sim::save(&g.world),before);
        let tech=serde_json::to_value(&g.world.nation(buyer).tech).unwrap();
        let arsenal=serde_json::to_value(&g.world.nation(buyer).arsenal).unwrap();
        let parsed=super::super::parse_command(&g.world,&reviewed["actions"][0]["command"],buyer).unwrap();
        spheres_sim::apply_command(&mut g.world,&parsed).unwrap();
        assert_eq!(g.world.companies.firms[0].products[0].stock,1);
        assert_eq!(serde_json::to_value(&g.world.nation(buyer).tech).unwrap(),tech);
        assert_eq!(serde_json::to_value(&g.world.nation(buyer).arsenal).unwrap(),arsenal);
        let id=g.world.companies.imports.contracts[0].id;
        let market=company_supplier_market(&g.world,buyer);let delivery=&market["deliveries"][0];
        assert_eq!(delivery["contract"],id);assert_eq!(delivery["origin"],"foreign");
        assert_eq!(delivery["status"],"Awaiting payment settlement");
        let before=spheres_sim::save(&g.world);
        let native=companies::import_cancel_quote(&g.world,buyer,id);
        let cancellation=preview(&g.world,buyer,&g.session_id,&json!({"command":delivery["actions"][0]["command"]})).unwrap();
        assert_eq!(cancellation["quote"]["refund_bn"],native.refund_bn);
        assert_eq!(cancellation["actions"][0]["command"]["quote"],native.token);
        assert_eq!(spheres_sim::save(&g.world),before);
    }

    #[test]
    fn s08_changed_stock_or_access_cannot_reuse_a_reviewed_import() {
        let (mut g,company,product)=foreign_stock_fixture();let buyer=NationId::Tonga;
        let command=json!({"kind":"company_import_purchase","seller":"France","company":company,
            "product":product,"ammunition":false,"quantity":1});
        let reviewed=preview(&g.world,buyer,&g.session_id,&json!({"command":command.clone()})).unwrap();
        assert_eq!(reviewed["valid"],true);
        let p=&mut g.world.companies.firms[0].products[0];p.stock=1;p.produced_units=1;p.stock_cost_bn=0.001;
        let before=spheres_sim::save(&g.world);
        let parsed=super::super::parse_command(&g.world,&reviewed["actions"][0]["command"],buyer).unwrap();
        assert!(spheres_sim::apply_command(&mut g.world,&parsed).unwrap_err().to_lowercase().contains("stale"));
        assert_eq!(spheres_sim::save(&g.world),before);
        g.world.sanctions.push((buyer,ME));let before=spheres_sim::save(&g.world);
        let blocked=preview(&g.world,buyer,&g.session_id,&json!({"command":command})).unwrap();
        assert_eq!(blocked["valid"],false);assert_eq!(blocked["actions"][0]["enabled"],false);
        assert!(!blocked["blockers"].as_array().unwrap().is_empty());
        assert_eq!(spheres_sim::save(&g.world),before);
    }

    #[test]
    fn s08_live_paid_product_work_is_distinct_from_last_capital_review() {
        let (mut g,company,_)=foreign_stock_fixture();let buyer=NationId::Tonga;
        let day=spheres_sim::clock::absolute_day(&g.world);
        let firm=&mut g.world.companies.firms[0];let district=firm.district.clone();
        let p=&mut firm.products[0];p.status="engineering".into();p.reason="Actual paid engineering receipt.".into();
        p.stock=0;p.produced_units=0;p.stock_cost_bn=0.0;p.certified_day=None;
        p.development_work_days=8.5;p.development_spent_bn=0.001;
        g.world.supplier_catalogue.plans.insert(ME,spheres_sim::supplier_catalogue::Plan{
            district:Some(district),company:Some(company),last_review_day:Some(day),
            status:"capital_settlement".into(),reason:"Earlier reviewed capitalization.".into()});
        let before=spheres_sim::save(&g.world);
        let market=company_supplier_market(&g.world,buyer);
        let program=market["programs"].as_array().unwrap().iter().find(|p|p["company"]==company).unwrap();
        assert_eq!(program["status"],"Development");assert_eq!(program["detail"],"Actual paid engineering receipt.");
        let milestones=program["milestones"].as_array().unwrap();
        assert!(milestones.iter().any(|m|m["value"].as_str().is_some_and(|v|v.starts_with("8.50 / ")&&v.ends_with("paid work-days"))));
        assert!(milestones.iter().any(|m|m["label"]=="Last administrative review"&&m["value"]=="Working capital settling"));
        assert!(spheres_sim::save(&g.world)==before,"Reading current product work cannot settle administrative receipts");
    }

    #[test]
    fn s08_declared_small_country_purchase_policy_uses_the_actual_priced_budget_review() {
        let mut g=super::super::Game::new(1990,Some(NationId::Tonga));
        super::super::fresh_play_rules(&mut g).unwrap();enroll_current_budget(&mut g,NationId::Tonga);
        let policy=review_tonga_procurement_priority(&mut g);
        assert_eq!(policy["no_immediate_cash_debt_authority_property_or_research_grant"],true);
        assert_eq!(policy["political_cost"],15.085);
        assert_eq!(policy["annual_procurement_bn_at_review"],0.0030096);
    }

    /// A separate genuine startup journey, never the synthetic card fixture.
    /// Output is disposable evidence; existing files and player saves are refused.
    #[test]
    #[ignore="Requires an empty SPHERES_S08_SUPPLIER_EXPORT directory; advances up to 3000 real fresh-campaign days"]
    fn s08_fresh_tonga_paid_supplier_market_export() {
        use std::{fs,io::Write,path::Path};
        let out=std::path::PathBuf::from(std::env::var_os("SPHERES_S08_SUPPLIER_EXPORT")
            .expect("Set SPHERES_S08_SUPPLIER_EXPORT to a new disposable evidence directory"));
        fs::create_dir_all(&out).unwrap();
        assert!(fs::read_dir(&out).unwrap().next().is_none(),"Exporter refuses a nonempty output directory");
        let source_root=Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
        let git=|args:&[&str]|std::process::Command::new("git").current_dir(source_root).args(args).output().ok()
            .filter(|r|r.status.success()).map(|r|String::from_utf8_lossy(&r.stdout).trim().to_string());
        let source_files=["spheres-web/src/company_view.rs","spheres-sim/src/supplier_catalogue.rs",
            "spheres-sim/src/companies_imports.rs","spheres-sim/src/companies.rs","spheres-web/src/main.rs"];
        let source_hashes:Vec<_>=source_files.iter().map(|p|json!({"path":p,"git_blob_sha1":git(&["hash-object",p])})).collect();
        let provenance=json!({"format":"spheres-s08-genuine-supplier-journey","build":super::super::build_info(),
            "source_head":git(&["rev-parse","HEAD"]),"source_status":git(&["-c","core.longpaths=true","status","--porcelain"]),
            "source_files":source_hashes,"executable":std::env::current_exe().ok(),
            "qualification":"Developmental unless separately bound to a clean recorded candidate and executable hash; no synthetic stock, plant, research or budget grants.",
            "declared_policy":"Review and enact a fixed-total Tonga budget reallocation from Infrastructure to Defense procurement, preserving absolute personnel, operations and maintenance. Renew the chosen policy unchanged annually; no purchase funds are granted upfront.",
            "maximum_actual_days":3000});
        fs::write(out.join("provenance.json"),serde_json::to_vec_pretty(&provenance).unwrap()).unwrap();
        let mut journal=fs::File::create(out.join("checkpoints.jsonl")).unwrap();
        fn checkpoint(g:&super::super::Game,out:&Path,journal:&mut fs::File,label:&str,advanced:usize,archive:bool) {
            let seller=NationId::France;let buyer=NationId::Tonga;
            let raw:Vec<_>=spheres_sim::resources::ALL.iter().map(|r|json!({"commodity":r.name(),
                "quantity":spheres_sim::resources::stock_quantity(&g.world,seller,*r)})).collect();
            let companies:Vec<_>=g.world.companies.firms.iter().filter(|c|c.nation==seller).collect();
            let projects:Vec<_>=spheres_sim::production::projects_for(&g.world,seller).collect();
            let project_finance:Vec<_>=projects.iter().filter_map(|p|g.world.production.industry.projects.get(&p.id).map(|f|json!({"project":p.id,"finance":f}))).collect();
            let facts=json!({"label":label,"actual_days_advanced":advanced,"date":g.world.date_str(),
                "as_of_day":spheres_sim::clock::absolute_day(&g.world),"catalogue":spheres_sim::supplier_catalogue::view(&g.world),
                "seller_projects":projects,"seller_project_finance":project_finance,"seller_companies":companies,"seller_raw_stock":raw,
                "seller_components":spheres_sim::industry_operations::advanced_component_stock(&g.world,seller),
                "seller_operations":spheres_sim::supplier_operations::snapshot(&g.world,seller),
                "seller_cash_bn":g.world.nation(seller).treasury_bn,"seller_program_budget":g.world.nation(seller).program_budget,
                "buyer_cash_bn":g.world.nation(buyer).treasury_bn,"buyer_program_budget":g.world.nation(buyer).program_budget,
                "buyer_imports":g.world.companies.imports,"offers":companies::import_offers(&g.world,buyer)});
            writeln!(journal,"{}",facts).unwrap();journal.flush().unwrap();
            if archive {
                let encoded=super::super::storage::encode(g).unwrap();
                // Retain the exact failing state too if archive qualification
                // discovers a new issue after years of genuine daily work.
                fs::write(out.join(format!("{advanced:04}-{label}.campaign.json")),&encoded).unwrap();
                if matches!(label,"first-finished-stock"|"ready-before-purchase"|"purchased"|"delivered") {
                    let restored=super::super::storage::decode(&encoded).expect("Genuinely earned supplier property must reload through the actual archive decoder");
                    assert!(spheres_sim::save(&restored.world)==spheres_sim::save(&g.world),"{label}: archive changed the genuine supplier world");
                }
            }
            let status=g.world.supplier_catalogue.plans.get(&seller);
            println!("S08 {advanced} {} {label}: France {} / {}",g.world.date_str(),status.map_or("unreviewed",|p|p.status.as_str()),status.map_or("",|p|p.reason.as_str()));
        }
        let buyer=NationId::Tonga;let seller=NationId::France;
        let mut g=super::super::Game::new(1990,Some(buyer));
        super::super::fresh_play_rules(&mut g).unwrap();
        g.history.clear();g.log.clear();g.snapshot();
        checkpoint(&g,&out,&mut journal,"fresh-before-decisions",0,true);
        enroll_current_budget(&mut g,buyer);
        let policy=review_tonga_procurement_priority(&mut g);
        fs::write(out.join("declared-funding-policy.json"),serde_json::to_vec_pretty(&policy).unwrap()).unwrap();
        let original_budget=serde_json::to_value(&g.world.nation(buyer).program_budget).unwrap();
        let original_stock=serde_json::to_value(&g.world.nation(buyer).arsenal).unwrap();
        let enrollment=preview(&g.world,buyer,&g.session_id,&json!({"command":{"kind":"company_enable_imports"}})).unwrap();
        assert_eq!(enrollment["valid"],true,"{enrollment}");
        let cmd=super::super::parse_command(&g.world,&enrollment["actions"][0]["command"],buyer).unwrap();
        spheres_sim::apply_command(&mut g.world,&cmd).unwrap();
        assert_eq!(serde_json::to_value(&g.world.nation(buyer).program_budget).unwrap(),original_budget,"Import enrollment cannot change procurement appropriations");
        assert_eq!(serde_json::to_value(&g.world.nation(buyer).arsenal).unwrap(),original_stock);
        assert!(g.world.companies.firms.is_empty());assert!(g.world.production.projects.is_empty());
        // The existing explicit Exchange decision, separate from import access.
        let competition=super::super::parse_command(&g.world,&json!({"kind":"enable_economic_competition"}),buyer).unwrap();
        spheres_sim::apply_command(&mut g.world,&competition).unwrap();
        assert_eq!(serde_json::to_value(&g.world.nation(buyer).program_budget).unwrap(),original_budget);
        checkpoint(&g,&out,&mut journal,"opening",0,true);
        let start=spheres_sim::clock::absolute_day(&g.world);
        let mut purchase=None;
        let mut first_stock=false;
        for advanced in 1..=3000usize {
            if g.world.nation(buyer).program_budget.as_ref().is_some_and(|p|p.fiscal_year!=g.world.year) {
                let cash=g.world.nation(buyer).treasury_bn;
                enroll_current_budget(&mut g,buyer);
                assert_eq!(g.world.nation(buyer).treasury_bn,cash,"Renewing unchanged annual authority must not grant cash");
                checkpoint(&g,&out,&mut journal,"unchanged-annual-funding-renewal",advanced-1,false);
            }
            g.advance_days(1,vec![]);
            assert_eq!(spheres_sim::clock::absolute_day(&g.world),start+advanced as i32);
            if advanced%30==0 {checkpoint(&g,&out,&mut journal,"progress",advanced,advanced%360==0);}
            if purchase.is_none() {
                let offers=companies::import_offers(&g.world,buyer);
                if !first_stock {
                    if let Some(offer)=offers.iter().find(|o|o.seller==seller&&!o.ammunition&&o.platform.as_deref()==Some("ground_apc")&&o.ready_stock>0) {
                        fs::write(out.join("first-finished-stock-offer.json"),serde_json::to_vec_pretty(offer).unwrap()).unwrap();
                        checkpoint(&g,&out,&mut journal,"first-finished-stock",advanced,true);
                        first_stock=true;
                    }
                }
                let offer=offers.into_iter().find(|o|o.seller==seller&&!o.ammunition
                    &&o.platform.as_deref()==Some("ground_apc")&&o.allowed&&o.review_quantity>0
                    &&(o.affordable_quantity<2||o.ready_stock>=2));
                if let Some(offer)=offer {
                    checkpoint(&g,&out,&mut journal,"ready-before-purchase",advanced,true);
                    let quantity=if offer.affordable_quantity>=2&&offer.ready_stock>=2 {2}else{offer.review_quantity};
                    let command=json!({"kind":"company_import_purchase","seller":seller,"company":offer.company,
                        "product":offer.product,"ammunition":false,"quantity":quantity});
                    let review=preview(&g.world,buyer,&g.session_id,&json!({"command":command})).unwrap();
                    assert_eq!(review["valid"],true,"{review}");
                    let tech=serde_json::to_value(&g.world.nation(buyer).tech).unwrap();
                    let held=serde_json::to_value(&g.world.nation(buyer).arsenal).unwrap();
                    let parsed=super::super::parse_command(&g.world,&review["actions"][0]["command"],buyer).unwrap();
                    spheres_sim::apply_command(&mut g.world,&parsed).unwrap();
                    assert_eq!(g.world.companies.firms.iter().find(|c|c.id==offer.company).unwrap().products.iter().find(|p|p.id==offer.product).unwrap().stock,offer.ready_stock-quantity);
                    assert_eq!(serde_json::to_value(&g.world.nation(buyer).tech).unwrap(),tech);
                    assert_eq!(serde_json::to_value(&g.world.nation(buyer).arsenal).unwrap(),held);
                    purchase=Some(g.world.companies.imports.contracts.last().unwrap().id);
                    fs::write(out.join("purchase-review.json"),serde_json::to_vec_pretty(&review).unwrap()).unwrap();
                    checkpoint(&g,&out,&mut journal,"purchased",advanced,true);
                }
            }
            if let Some(id)=purchase {
                let d=g.world.companies.imports.contracts.iter().find(|d|d.id==id).unwrap();
                if d.delivered_day.is_some() {
                    assert!(d.settled_day.is_some());assert_eq!(d.escrow_bn,0.0);assert_eq!(d.refunded_bn,0.0);
                    let revision=d.buyer_revision.as_ref().expect("Actual arrival must retain its frozen imported model");
                    assert_eq!(g.world.nation(buyer).equipment.as_ref().unwrap().revisions[revision].spec,d.source_revision.spec);
                    checkpoint(&g,&out,&mut journal,"delivered",advanced,true);
                    fs::write(out.join("result.json"),serde_json::to_vec_pretty(&json!({"passed":true,"build":super::super::build_info(),
                        "actual_days_advanced":advanced,"date":g.world.date_str(),"contract":d,"source":provenance,
                        "no_synthetic_endowments":true,"funding_policy":policy,"note":"Ordinary fresh campaign with a declared fixed-total procurement-priority budget tradeoff, unchanged annual renewals, explicit reviewed import enrollment and purchase; all funds accrue and all supplier facilities, development, inputs and stock are earned through native daily work."})).unwrap()).unwrap();
                    return;
                }
            }
        }
        checkpoint(&g,&out,&mut journal,"blocked-final",3000,true);
        fs::write(out.join("result.json"),serde_json::to_vec_pretty(&json!({"passed":false,"build":super::super::build_info(),
            "actual_days_advanced":3000,"date":g.world.date_str(),"purchase":purchase,
            "catalogue":spheres_sim::supplier_catalogue::view(&g.world),"seller_operations":spheres_sim::supplier_operations::snapshot(&g.world,seller),
            "reason":"No genuinely produced French APC lot reached Tonga within the bounded journey. No financial or input blockers were bypassed."})).unwrap()).unwrap();
        panic!("Genuine supplier journey did not deliver by {}: inspect {}",g.world.date_str(),out.display());
    }
}
