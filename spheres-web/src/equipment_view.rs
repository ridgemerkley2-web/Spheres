//! Read models and pure quotes for the equipment bureau. The simulator owns every rule.
use serde_json::{json, Value};
use spheres_sim::{equipment as eq, world::*, Command, EquipmentOrder};
include!("equipment_planning.rs");
include!("equipment_supply_view.rs");
include!("equipment_service_view.rs");

fn metric(label:&str,value:impl serde::Serialize)->Value {json!({"label":label,"value":value})}
fn cost(label:&str,amount:f64,period:&str)->Value {json!({"label":label,"amount_bn":amount,"period":period})}
fn nav(label:&str,action:Value)->Value {json!({"label":label,"navigate":action,"enabled":true})}
fn budget_input(amount:f64)->Value {json!({"key":"daily_budget_mn","label":"Project funding limit","type":"number","value":amount*1000.0,"min":0,"max":1_000_000,"step":0.01,"unit":"$m / day"})}
fn checked(w:&WorldState,me:NationId,label:&str,command:Value)->Value {
    let reason=super::parse_command(w,&command,me).ok_or_else(||"Invalid equipment order.".to_string())
        .and_then(|c|spheres_sim::apply_command(&mut w.clone(),&c)).err();
    let detail=(command["kind"]=="equipment_research").then_some("Changing focus costs 6 political capital and retains half the previous Aerospace progress. This uses existing research effort; it creates no vehicle or national military bonus.");
    json!({"label":label,"command":command,"enabled":reason.is_none(),"reason":reason,"detail":detail})
}
fn intent(label:&str,command:Value,inputs:Vec<Value>)->Value {
    json!({"label":label,"command":command,"inputs":inputs,"requires_preview":true,"enabled":true})
}
fn profile_metrics(p:&eq::CompiledProfile)->Vec<Value> {let mut rows=vec![
    metric("Land contribution",format!("{:.3}×",p.land_factor)),
    metric("Firepower rating",p.land),metric("Protection rating",p.protection),
    metric("Mobility rating",p.mobility),metric("Observation rating",p.recon),
    metric("Installation load",format!("{} / {}",p.installation_used,p.installation_capacity)),
];
if let Some(g)=&p.ground_roles {rows.extend([metric("Fire support",g.fire_support),metric("Protected mobility",g.protected_mobility),metric("Reconnaissance support",g.reconnaissance),metric("Air defense",g.air_defense)]);}
rows}
fn profile_costs(p:&eq::CompiledProfile)->Vec<Value> {vec![
    cost("Development and trials",p.development_cost_bn,"one programme"),
    cost("Site tooling",p.tooling_cost_bn,"each production batch"),
    cost("Vehicle fabrication",p.fabrication_cost_bn,"per vehicle; raw inputs separate"),
    cost("Maintenance requirement",p.maintenance_bn_day,"per vehicle / day; existing Defense maintenance allocation"),
]}
fn sites(w:&WorldState,me:NationId)->Vec<Value> {
    w.districts.iter().filter(|(_,d)|**d==me).filter_map(|(id,_)|{
        let total=spheres_sim::manufacturing::plant_slots(w,id);
        if total==0{return None;}
        let used=spheres_sim::manufacturing::used_slots(w,me,id);
        Some(json!({"value":id,"label":format!("{} — {} of {} slots occupied",spheres_sim::districts::name_of(id).unwrap_or(id),used,total),"enabled":true}))
    }).collect()
}
fn production_action(w:&WorldState,me:NationId,r:&eq::DesignRevision)->Value {
    let options=sites(w,me);let site=options.first().map(|o|o["value"].clone()).unwrap_or(json!(""));
    intent("Review production order",json!({"kind":"equipment_produce","revision":r.id,"district":site,"quantity":10,"daily_budget_mn":0.2}),vec![
        json!({"key":"district","label":"Arms plant province","type":"select","value":site,"options":options}),
        json!({"key":"quantity","label":"Vehicles to build","type":"number","value":10,"min":1,"max":eq::MAX_BATCH,"step":1}),budget_input(0.0002)])
}
pub fn view(w:&WorldState,me:NationId,session:&str)->Value {
    let n=w.nation(me);let state=n.equipment.as_ref();
    let supply_plans=eq::supply_plan(w,me);
    let supply=equipment_national_supply(w,me,&supply_plans);
    let platforms:Vec<_>=eq::PLATFORMS.iter().map(|p|json!({"id":p.id,"name":p.name,"description":p.detail,
        "role":eq::platform_role(p.id),"family":if p.id.starts_with("tank_"){"tanks"}else{p.id},"family_name":if p.id.starts_with("tank_"){"Tanks"}else{p.name},"default_spec":eq::default_spec(p.id),
        "slots":eq::platform_slots(p.id).iter().map(|s|json!({"id":s,"name":eq::slot_name(s),"required":true,
            "components":eq::all_components().filter(|c|c.slot==*s && eq::component_compatible(p.id,c)).map(|c|c.id).collect::<Vec<_>>()})).collect::<Vec<_>>()})).collect();
    let components:Vec<_>=eq::all_components().map(|c|{
        let known=eq::component_known(n,c);
        let tech=c.research.map(|id|json!({"id":id,"domain":"Aerospace","name":eq::research(id).map(|r|r.name),"equipment":true}))
            .or_else(||c.technology.map(|id|json!({"id":id,"domain":"Aerospace","name":id})));
        json!({"id":c.id,"name":c.name,"family":c.slot,"description":c.detail,"known":known,
            "reason":if known {None} else {Some("Research this system before funding a model that uses it.")},"tech":tech,
            "tradeoffs":[format!("{} installation points · ${:.0}k fabrication · ${:.0}/day upkeep",c.load,c.cost_bn*1e6,c.upkeep_bn_day*1e9),
                format!("Rating changes: firepower {:+.2}, protection {:+.2}, mobility {:+.2}, observation {:+.2}",c.land,c.protection,c.mobility,c.recon)]})
    }).collect();
    let presets=equipment_presets();
    let mut designs=vec![];let mut development=vec![];let mut production=vec![];let mut lots=vec![];
    if let Some(s)=state {
        for (id,d) in &s.drafts {designs.push(json!({"id":format!("draft:{id}"),"name":d.name,"spec":d.spec,"editable_spec":eq::editable_spec(&d.spec),"status":"Draft","detail":"Editable concept. No money spent and no vehicles created.","actions":[]}));}
        for r in s.revisions.values() {
            designs.push(json!({"id":r.id,"name":r.name,"spec":r.spec,"editable_spec":eq::editable_spec(&r.spec),"status":if r.certified_day.is_some(){"Certified"}else{"Under development"},
                "detail":format!("Revision {} is fixed. Use it as a starting point to develop a different configuration.",r.id),"metrics":profile_metrics(&r.profile),"costs":profile_costs(&r.profile),
                "actions":if r.certified_day.is_some(){vec![production_action(w,me,r)]}else{vec![]}}));
        }
        for p in &s.projects {
            let r=s.revisions.get(&p.revision_id);let active=!matches!(p.status,eq::ProjectStatus::Complete|eq::ProjectStatus::Cancelled);
            let mut actions=vec![];
            if active {
                actions.push(checked(w,me,if p.paused{"Resume programme"}else{"Pause programme"},json!({"kind":"equipment_pause","project":p.id,"paused":!p.paused})));
                actions.push(intent("Adjust project funding",json!({"kind":"equipment_funding","project":p.id,"daily_budget_mn":p.daily_budget_bn*1000.0}),vec![budget_input(p.daily_budget_bn)]));
                let mut cancel=checked(w,me,"Cancel remaining work",json!({"kind":"equipment_cancel","project":p.id}));
                cancel["detail"]=json!("Spent money and consumed inputs are not refunded. Delivered vehicles remain. Unconverted vehicles reserved for a refit return to service.");actions.push(cancel);
            }
            let mut costs=vec![cost("Programme spending",p.spent_bn,"cumulative"),cost("Planned fabrication / development",p.cost_bn,"raw inputs separate"),cost("Project limit",p.daily_budget_bn,"per day")];
            if p.last_day.is_some(){costs.push(cost("Latest payment",p.last_spent_bn,"latest programme day"));}
            let completion=if p.kind==eq::ProjectKind::Development{metric("Certification",if r.is_some_and(|r|r.certified_day.is_some()){"Certified"}else{"Pending trials"})}else{metric("Completed vehicles",format!("{} / {}",p.completed_units,p.quantity))};
            let row=json!({"id":p.id,"name":format!("{} · programme {}",r.map(|r|r.name.as_str()).unwrap_or("Model"),p.id),
                "status":format!("{:?} · {:?}",p.kind,p.status),"detail":p.reason,"progress":if p.cost_bn>0.0{p.spent_bn/p.cost_bn}else{0.0},
                "metrics":[completion,metric("Work days",format!("{:.1} / {}",p.work_days,p.minimum_days)),metric("Province",p.district.as_deref().unwrap_or("National development"))],
                "costs":costs,
                "supply":supply_plans.iter().find(|plan|plan.project_id==p.id).map(|plan|equipment_project_supply(w,me,plan)),
                "awaiting_receipt":p.last_day.is_none(),"receipt_label":p.last_day.map(|d|super::settled_day_json(d)["label"].clone()),"actions":actions});
            if p.kind==eq::ProjectKind::Development{development.push(row);}else{production.push(row);}
        }
        for h in &n.arsenal.held {
            let Some(id)=h.design_id.as_deref() else{continue;};let Some(r)=s.revisions.get(id)else{continue;};
            let options:Vec<_>=s.revisions.values().filter(|t|t.id!=r.id&&t.certified_day.is_some()&&t.spec.platform==r.spec.platform&&t.spec.components.get("armament")==r.spec.components.get("armament"))
                .map(|t|json!({"value":t.id,"label":t.name,"enabled":true})).collect();
            let mut actions=vec![];
            if let Some(target)=options.first(){let site_options=sites(w,me);let site=site_options.first().map(|s|s["value"].clone()).unwrap_or(json!(""));
                actions.push(intent("Review compatible refit",json!({"kind":"equipment_refit","source":id,"target":target["value"],"district":site,"quantity":1,"daily_budget_mn":0.1}),vec![
                    json!({"key":"target","label":"Target revision","type":"select","value":target["value"],"options":options}),
                    json!({"key":"district","label":"Arms plant province","type":"select","value":site,"options":site_options}),
                    json!({"key":"quantity","label":"Vehicles to withdraw for refit","type":"number","value":1,"min":1,"max":eq::MAX_BATCH,"step":1}),budget_input(0.0001)]));
            }
            actions.push(retirement_action(w,me,id));
            lots.push(json!({"id":id,"name":r.name,"status":"In service","detail":"Only delivered vehicles contribute. Reserved refit vehicles are withdrawn until conversion or cancellation.",
                "metrics":[metric("Vehicles held",h.units),metric("Available for operations",spheres_sim::arsenal::available_design_units(h)),metric("Reserved for refit",h.refit_reserved),metric("Age",format!("{:.1} months",h.age)),metric("Condition",format!("{:.1}%",spheres_sim::arsenal::holding_condition(n,h)*100.0)),metric("Last settled maintenance coverage",if s.last_tick_day.is_some(){format!("{:.0}%",s.maintenance_fraction*100.0)}else{"Not yet settled".into()})],
                "costs":[cost("Maintenance requirement",r.profile.maintenance_bn_day*h.units,"per day")],"actions":actions}));
        }
        for o in &n.arsenal.orders {if let Some(id)=o.design_id.as_deref(){production.push(json!({"id":format!("delivery:{id}:{}",o.due),"name":s.revisions.get(id).map(|r|r.name.as_str()).unwrap_or(id),"status":"Delivery","detail":"Fabrication is paid. These vehicles enter service when delivery completes.","metrics":[metric("Vehicles in transit",o.units),metric("Delivery remaining",format!("{} days",o.due_days.unwrap_or(o.due*30)))],"actions":[]}));}}
    }
    let research=research_board(w,me);
    let comparison_options=comparison_options(&presets,&designs);
    let modernization=modernization_board(w,me);
    json!({"session_id":session,"nation":me,"name":me.name(),"date":w.date_str(),"enabled":spheres_sim::clock::is_daily(w)&&w.rules.military_operations,"reason":"Equipment programmes require daily time and military operations.",
        "platforms":platforms,"components":components,"presets":presets,"designs":designs,"development":development,"production":production,"lots":lots,"research":research,"comparison_options":comparison_options,"modernization":modernization,"supply":supply,"service":equipment_service_board(w,me),
        "funding":{"metrics":[metric("Development","Defense · Research & development"),metric("Production and refit","Defense · Procurement"),metric("Service support","Defense · Maintenance, within the existing allocation"),metric("Unused development funds",format!("${:.3}m",spheres_sim::programs::available_bn(w,me,BUDGET_DEFENSE,4)*1000.0)),metric("Unused procurement funds",format!("${:.3}m",spheres_sim::programs::available_bn(w,me,BUDGET_DEFENSE,3)*1000.0))]},
        "actions":[nav("Development funding",json!({"action":"budget","ministry":"defense","department":4})),nav("Procurement funding",json!({"action":"budget","ministry":"defense","department":3})),nav("Build an arms plant",json!({"action":"construction","kind":"arms_plant"})),nav("Review raw inputs",json!({"action":"resources"}))]})
}

fn spec(v:&Value)->Result<eq::DesignSpec,String>{serde_json::from_value(json!({"platform":v.get("platform"),"components":v.get("components")})).map_err(|_|"Choose a chassis and valid component selections.".into())}
pub fn preview(w:&WorldState,me:NationId,session:&str,v:&Value)->Result<Value,String> {
    if let Some(command)=v.get("command") {
        let parsed=super::parse_command(w,command,me).ok_or("The equipment order is malformed.")?;
        let Command::Equipment {ref order,..}=parsed else{return Err("This preview accepts equipment orders only.".into());};
        if let EquipmentOrder::Retire{revision,quantity}=order {
            return Ok(equipment_retirement_preview(w,me,session,command,&parsed,revision,*quantity));
        }
        let action=checked(w,me,"Confirm this order",command.clone());let mut blockers=vec![];
        if let Some(reason)=action["reason"].as_str(){blockers.push(reason.to_string());}
        let quote=match order {
            EquipmentOrder::Develop{name,spec,daily_budget_bn}=>Some(eq::development_quote(w,me,name,spec,*daily_budget_bn)),
            EquipmentOrder::Produce{revision,district,quantity,daily_budget_bn}=>Some(eq::production_quote(w,me,revision,district,*quantity,*daily_budget_bn)),
            EquipmentOrder::Refit{source,target,district,quantity,daily_budget_bn}=>Some(eq::refit_quote(w,me,source,target,district,*quantity,*daily_budget_bn)),_=>None,
        };
        let supply=equipment_order_supply(w,me,&parsed,order,quote.as_ref());
        let mut costs=vec![];let mut timing=vec![];let mut requirements=vec![];
        if let Some(q)=quote {
            costs=vec![cost("Programme cost",q.cost_bn,"paid as work progresses; raw inputs separate"),cost("Project funding limit",q.daily_budget_bn,"per day; shared department funds may constrain it")];
            timing=vec![json!({"label":"Minimum work time","value":format!("{} days",q.minimum_days)}),json!({"label":"Estimate at this funding limit","value":q.eta_days.map(|d|format!("{d} days + delivery where applicable")).unwrap_or("Paused at zero funding".into())})];
            requirements.push(q.note);
            for (i,amount) in q.recipe.iter().enumerate(){if *amount>0.0{requirements.push(format!("Raw input {}: {:.3} {}. Source this through Resources; the batch waits when its inputs run short.",spheres_sim::resources::ALL[i].name(),amount,spheres_sim::resources::ALL[i].unit()));}}
        } else if let EquipmentOrder::Funding{daily_budget_bn,..}=order {costs.push(cost("New project funding limit",*daily_budget_bn,"per day"));}
        return Ok(json!({"session_id":session,"nation":me,"valid":blockers.is_empty(),"blockers":blockers,"metrics":[],"costs":costs,"timing":timing,"requirements":requirements,"supply":supply,"actions":[action],"detail":"Starting a programme reserves its work and funding priority. Spending occurs on future daily ticks. All figures are modeled game assumptions."}));
    }
    let spec=spec(v)?;let name=v.get("name").and_then(Value::as_str).unwrap_or("New vehicle");let p=eq::design_preview(w,me,&spec);
    let mut actions=vec![checked(w,me,"Save design draft",json!({"kind":"equipment_save","name":name,"platform":spec.platform,"components":spec.components}))];
    if p.valid {actions.push(intent("Review development funding",json!({"kind":"equipment_develop","name":name,"platform":spec.platform,"components":spec.components,"daily_budget_mn":0.5}),vec![budget_input(0.0005)]));}
    let mut metrics=p.profile.as_ref().map(profile_metrics).unwrap_or_default();
    if let Some(old)=v.get("source_revision").and_then(Value::as_str).and_then(|id|eq::profile(w.nation(me),id)){if let Some(new)=&p.profile{
        metrics.push(metric("Change from original land contribution",format!("{:+.3}×",new.land_factor-old.land_factor)));
        metrics.push(metric("Fabrication cost change",format!("{:+.3}m per vehicle",(new.fabrication_cost_bn-old.fabrication_cost_bn)*1000.0)));
    }}
    let comparison=design_comparison(w,me,v,&spec,p.profile.as_ref());
    Ok(json!({"session_id":session,"nation":me,"valid":p.valid,"blockers":p.blockers,"metrics":metrics,"costs":p.profile.as_ref().map(profile_costs).unwrap_or_default(),
        "timing":p.profile.as_ref().map(|p|vec![json!({"label":"Development minimum","value":format!("{} days",p.development_days)}),json!({"label":"Per-vehicle production minimum","value":format!("{} days after {} tooling days",p.production_days,p.tooling_days)})]).unwrap_or_default(),
        "comparison":comparison,"requirements":p.notes,"actions":actions,"detail":"Research unlocks components. Paid development certifies this exact revision. Only delivered vehicles affect the country's land forces."}))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture()->super::super::Game {
        let mut g=super::super::Game::new(1990,Some(NationId::USA));
        super::super::play_rules(&mut g);
        spheres_sim::programs::set_construction_budget(&mut g.world,NationId::USA,0.0).unwrap();
        g
    }
    fn design()->Value {let s=eq::baseline_spec();json!({"name":"Test model","platform":s.platform,"components":s.components})}
    #[test]
    fn equipment_reads_and_order_quotes_are_pure_and_use_simulation_prices() {
        let g=fixture();let w=&g.world;let before=spheres_sim::save(w);
        let board=view(w,NationId::USA,&g.session_id);
        assert_eq!(board["session_id"],g.session_id);assert_eq!(board["platforms"].as_array().unwrap().len(),eq::PLATFORMS.len());
        let draft=preview(w,NationId::USA,&g.session_id,&design()).unwrap();assert_eq!(draft["valid"],true);
        let action=&draft["actions"][1];assert_eq!(action["requires_preview"],true);
        let final_quote=preview(w,NationId::USA,&g.session_id,&json!({"command":action["command"]})).unwrap();
        assert_eq!(final_quote["valid"],true);assert!(final_quote["actions"][0]["inputs"].is_null());
        assert_eq!(final_quote["costs"][0]["amount_bn"],eq::design_preview(w,NationId::USA,&eq::baseline_spec()).profile.unwrap().development_cost_bn);
        assert_eq!(spheres_sim::save(w),before,"Opening and quoting cannot create state or spend funding");
    }
    #[test]
    fn equipment_drafts_can_keep_unknown_components_but_cannot_develop_them() {
        let g=fixture();let mut draft=design();draft["components"]["sensors"]=json!("sensors_integrated");
        let p=preview(&g.world,NationId::USA,&g.session_id,&draft).unwrap();
        assert_eq!(p["valid"],false);assert_eq!(p["actions"].as_array().unwrap().len(),1);assert_eq!(p["actions"][0]["enabled"],true);
        let mut w=g.world.clone();let command=super::super::parse_command(&w,&p["actions"][0]["command"],NationId::USA).unwrap();
        spheres_sim::apply_command(&mut w,&command).unwrap();assert_eq!(w.nation(NationId::USA).equipment.as_ref().unwrap().finance_from_day,i32::MAX);
    }
    #[test]
    fn detailed_catalogue_prices_every_vehicle_type_with_its_independent_slots() {
        let g=fixture();let before=spheres_sim::save(&g.world);let board=view(&g.world,NationId::USA,&g.session_id);
        for p in board["platforms"].as_array().unwrap(){assert_eq!(p["slots"].as_array().unwrap().len(),eq::platform_slots(p["id"].as_str().unwrap()).len());}
        for preset in board["presets"].as_array().unwrap(){
            assert_eq!(preset["components"].as_object().unwrap().len(),eq::platform_slots(preset["platform"].as_str().unwrap()).len());
            let quote=preview(&g.world,NationId::USA,&g.session_id,preset).unwrap();assert_eq!(quote["valid"],true,"{}",preset["name"]);
            assert_eq!(quote["actions"][1]["command"]["components"],preset["components"]);
        }
        assert_eq!(spheres_sim::save(&g.world),before);
    }
    #[test]
    fn equipment_order_parser_keeps_nation_authority_and_rejects_fractional_units() {
        let g=fixture();let mut c=json!({"kind":"equipment_produce","nation":"China","revision":"missing","district":"unknown","quantity":1,"daily_budget_mn":2.5});
        assert!(matches!(super::super::parse_command(&g.world,&c,NationId::USA),Some(Command::Equipment{nation:NationId::USA,order:EquipmentOrder::Produce{daily_budget_bn,..}}) if daily_budget_bn==0.0025));
        for q in [json!(1.5),json!(-1),json!(4294967296u64),json!("1")] {c["quantity"]=q;assert!(super::super::parse_command(&g.world,&c,NationId::USA).is_none());}
        assert!(preview(&g.world,NationId::USA,&g.session_id,&json!({"command":{"kind":"rate","value":5}})).is_err());
    }
    #[test]
    fn equipment_routes_require_current_campaign_on_get_and_post() {
        use tiny_http::Method;
        for path in ["/api/equipment","/api/equipment-preview"]{assert!(super::super::exchange_read_path(path));}
        assert!(super::super::exchange_session_matches(&Method::Get,"/api/equipment?session_id=123-4",&json!({}),"123-4"));
        assert!(!super::super::exchange_session_matches(&Method::Get,"/api/equipment",&json!({}),"123-4"));
        assert!(super::super::exchange_session_matches(&Method::Post,"/api/equipment-preview",&json!({"session_id":"123-4"}),"123-4"));
        assert!(!super::super::exchange_session_matches(&Method::Post,"/api/equipment-preview",&json!({"session_id":"old"}),"123-4"));
    }
    #[test]
    fn equipment_archives_roundtrip_and_old_world_shape_refuses_them() {
        let mut g=fixture();eq::save_draft(&mut g.world,NationId::USA,"Saved concept",eq::baseline_spec()).unwrap();
        let raw=spheres_sim::save(&g.world);assert!(serde_json::from_str::<WorldState>(&raw).is_err(),"Pre-designer raw loaders must refuse the envelope");
        assert_eq!(spheres_sim::save(&spheres_sim::load(&raw).unwrap()),raw);
        let archive=super::super::storage::encode(&g).unwrap();let restored=super::super::storage::decode(&archive).unwrap();
        assert_eq!(spheres_sim::save(&restored.world),raw);
        let mut invalid:Value=serde_json::from_str(&raw).unwrap();invalid["version"]=json!(999);assert!(spheres_sim::load(&invalid.to_string()).is_err());
        let mut invalid:Value=serde_json::from_str(&raw).unwrap();let nations=invalid["world"]["nations"].as_array_mut().unwrap();
        nations.iter_mut().find(|n|n["equipment"].is_object()).unwrap()["equipment"]["version"]=json!(999);assert!(spheres_sim::load(&invalid.to_string()).is_err());
    }
    #[test]
    fn equipment_research_appears_in_current_research_without_adding_technology_nodes() {
        let mut g=fixture();let t=spheres_sim::tech::index_of("core_cmos_submicron").unwrap();
        if !g.world.nation(NationId::USA).tech.knows_index(t){g.world.nation_mut(NationId::USA).tech.known.push(t);g.world.nation_mut(NationId::USA).tech.known.sort();}
        let before=g.world.nation(NationId::USA).tech.count();
        spheres_sim::apply_command(&mut g.world,&Command::Equipment{nation:NationId::USA,order:EquipmentOrder::Research{component:"tank_fire_control_1990".into()}}).unwrap();
        let r=super::super::research_json(&g.world,NationId::USA);let d=r["domains"].as_array().unwrap().iter().find(|d|d["domain"]=="Aerospace").unwrap();
        assert_eq!(d["project"]["equipment"],true);assert_eq!(g.world.nation(NationId::USA).tech.count(),before);
        let tree=super::super::tech_tree_json(&g.world,NationId::USA,spheres_sim::tech::Domain::Aerospace);
        assert!(tree["nodes"].as_array().unwrap().iter().all(|n|n["focus"]==false));
    }
}
