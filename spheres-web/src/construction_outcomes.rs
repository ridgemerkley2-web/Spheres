//! Construction installs capacity. Workforce matching and dated operating/GDP
//! receipts describe separate events; reading this page must never run them.
use serde_json::{json, Value};
use spheres_sim::{clock, gdp_projects, industrial_modules, industry, industry_operations,
    population, programs, resources, production::{self, ProjectKind as K}, world::{NationId, WorldState, BUDGET_INDUSTRY}};

fn dated(day:i32)->String {
    let (year,month,date)=clock::date_from_day(day);
    format!("{year:04}-{month:02}-{date:02}")
}
pub(crate) fn action(district:&str,kind:K)->Value {
    json!({"action":"industry","district":district,"kind":kind.key(),
        "label":format!("Inspect {} outcome",production::catalog(kind).name)})
}
fn installed(w:&WorldState,district:&str,kind:K)->f64 {
    if matches!(kind,K::CivilianIndustry|K::StarterIndustry|K::Generation|K::PowerGrid) {
        industrial_modules::effective_capacity(w,district,kind)
    } else { production::level(w,district,kind) as f64 }
}
fn passive(kind:K)->bool {
    matches!(kind,K::CivilianIndustry|K::Infrastructure|K::PowerGrid|K::Generation
        |K::FreightTerminal|K::Warehouse|K::Automation|K::Efficiency)
}

/// A full installed day uses the native prospective recipes, before workforce,
/// demand, storage and shared-supply limits. These are not construction inputs
/// or quantities asserted to have been consumed by a past receipt.
fn rated_requirements(w:&WorldState,me:NationId,district:&str,kind:K,capacity:f64)->Vec<Value> {
    let processor=matches!(kind,K::ProcessingPlant|K::StarterIndustry|K::MachineryWorks);
    if !processor && !matches!(kind,K::OfficeDistrict|K::AdvancedIndustry|K::Shipyard) {return vec![];}
    let company=industry::manufacturing_company(w,me,district);
    let (power,raw,intermediates,cash,department)=if processor {
        let rate=industry::plant_rate(w,district,kind);
        let power=rate*industry::power_per_pack(w,district,kind);
        (power,industry::company_operating_recipe(w,me,district,kind,rate,power),
            if kind==K::MachineryWorks{rate*company.input_rate}else{0.0},
            rate*industry_operations::OPERATING_CASH_LEVEL_DAY_BN*(1.0+company.fee_rate),
            if kind==K::MachineryWorks{0}else{2})
    } else {
        let power=capacity*industry_operations::power_per_level(w,district,kind);
        (power,industry_operations::operating_raw_recipe(w,me,district,kind,capacity,power),
            industry_operations::intermediate_requirement(w,me,district,kind,capacity),
            industry_operations::operating_cash_required(w,me,district,kind,capacity),0)
    };
    let mut rows=vec![];
    let mut add=|id:String,label:String,required:f64,available:f64,unit:&str,note:&str| {
        rows.push(json!({"id":id,"label":label,"required":required,"available":available,
            "unit":unit,"ready":available>=required,"note":note}));
    };
    let stock_note="Required for one full operating day at installed capacity; available is shared national inventory, not a daily delivery or a reservation. Actual operation may be lower. Construction does not require this input.";
    for commodity in resources::ALL {
        if raw[commodity.idx()]>0.0 {
            add(format!("raw_{}",commodity.key()),format!("{} operating input",commodity.name()),
                raw[commodity.idx()],resources::stockpile(w,me,commodity),commodity.unit(),stock_note);
        }
    }
    if intermediates>0.0 {
        add("intermediate_packs".into(),"Intermediate operating inputs".into(),intermediates,
            w.production.industry.goods.get(&me).map_or(0.0,|g|g.intermediates),"packs",stock_note);
    }
    let power_note="Required per full installed operating day, compared with total modeled support. Capacity is shared with other consumers; this does not reserve power or report historical dispatch.";
    add("national_power".into(),"National generating support".into(),power,industry::power_capacity(w,me),"power units/day",power_note);
    add("local_grid".into(),"Local grid support".into(),power,industry_operations::grid_capacity(w,district),"power units/day",power_note);
    let authority_note="Rated charge for one full installed operating day at current contractor terms. Available is current department spending authority, not Treasury cash or a payment already made; actual work may be lower.";
    add("operating_authority".into(),"Facility operating authority".into(),cash,
        programs::available_bn(w,me,BUDGET_INDUSTRY,department),"$bn",authority_note);
    let (_,energy_fee)=industry::energy_company_rates(w,me);
    add("energy_authority".into(),"Generating service authority".into(),
        power*industry_operations::ENERGY_CASH_POWER_DAY_BN*(1.0+energy_fee),
        programs::available_bn(w,me,BUDGET_INDUSTRY,1),"$bn",authority_note);
    rows
}

/// A historical receipt never borrows present capacity, staffing, price or
/// national ownership to fill a missing historical field.
fn operation(w:&WorldState,me:NationId,district:&str,kind:K)->Option<Value> {
    let today=clock::absolute_day(w);
    if let Some(row)=w.production.operations.receipts.iter().find(|r|
        r.district==district && r.kind==kind.key() && r.nation==Some(me)
            && r.recorded_day.is_some_and(|day|day<=today)) {
        let day=row.recorded_day.unwrap();
        return Some(json!({"day":day,"date":dated(day),"label":"Recorded facility operation",
            "status":row.status,"reason":row.reason,"output":row.output_daily,"output_unit":row.output_unit,
            "cash_spent_bn":row.cash_spent_daily_bn,"power_used_daily":row.power_used_daily,
            "installed_capacity":row.installed_capacity,"used_workers":row.jobs_filled,
            "required_workers":row.jobs_required,"assigned_workers":null,
            "actual_operating_capacity":row.actual_operating_capacity,"actual_utilization":row.actual_utilization,
            "owner_verified":true,"value_added_bn":null,
            "note":"Actual output, labor use and operating charges on the recorded date. Current capacity or shortages do not rewrite this receipt. These charges are included in the existing fiscal settlement, not a second bill."}));
    }
    if kind==K::ResearchCenter {
        let row=w.production.industry.research.get(&me)?.operations.iter().find(|r|
            r.district==district && r.nation==me && r.day<=today)?;
        return Some(json!({"day":row.day,"date":dated(row.day),"label":"Recorded prototype service",
            "status":row.status,"reason":row.reason,"output":row.prototype_credit,"output_unit":"prototype credit",
            "cash_spent_bn":row.cash_spent_daily_bn,"power_used_daily":null,
            "installed_capacity":row.level,"used_workers":null,"actual_operating_capacity":null,
            "required_workers":null,"assigned_workers":null,
            "actual_utilization":null,"owner_verified":true,"value_added_bn":null,
            "note":"Recorded testing credit for its sponsoring government and technology. Credit is neither manufactured inventory nor a cash refund. This older receipt does not store workers used."}));
    }
    let row=w.production.industry.operations.iter().find(|r|r.district==district && r.kind==kind)?;
    if row.operation.as_ref().is_some_and(|context|context.nation!=me || context.day>today) {return None;}
    let day=row.operation.as_ref().map(|context|context.day)
        .or(w.production.industry.last_day).filter(|day|*day<=today)?;
    let unit=match kind {K::ProcessingPlant|K::StarterIndustry=>"intermediate packs",
        K::MachineryWorks=>"capital-goods packs",_=>return None};
    // The inherited SiteStatus format names the province, but not the payer or
    // dated workers. Keep those unknown instead of attributing captured output.
    Some(json!({"day":day,"date":dated(day),"label":"Recorded province operation",
        "status":row.status,"reason":row.reason,"output":row.output_daily,"output_unit":unit,
        "cash_spent_bn":row.cash_spent_daily_bn,"power_used_daily":row.power_used_daily,
        "installed_capacity":row.operation.as_ref().map(|context|context.installed_capacity)
            .or_else(||if kind==K::StarterIndustry {row.capacity_micros.map(|v|v as f64/1_000_000.0)}else{Some(row.level as f64)}),
        "used_workers":row.operation.as_ref().map(|context|context.jobs_used),
        "assigned_workers":row.operation.as_ref().and_then(|context|context.jobs_assigned),
        "required_workers":row.operation.as_ref().map(|context|context.jobs_required),
        "actual_operating_capacity":null,"actual_utilization":null,
        "owner_verified":row.operation.is_some(),"value_added_bn":null,
        "note":if row.operation.is_some(){"Actual output, worker use and operating charges on the recorded date. Current capacity or shortages do not rewrite this receipt. Charges belong to the existing fiscal settlement, not a second bill."}
            else{"The legacy province receipt records output and charges, but not its sponsoring government or workers used. National GDP attribution is unavailable from this receipt alone. Present-day capacity is not substituted for its original capacity."}}))
}

pub(crate) fn for_site(w:&WorldState,me:NationId,district:&str,kind:K)->Value {
    if w.districts.get(district)!=Some(&me) || !w.nation_opt(me).is_some_and(|n|n.alive) {
        return Value::Null;
    }
    let capacity=installed(w,district,kind);
    let connected=industry_operations::enabled(w);
    let current=connected.then(||industry_operations::site(w,district,kind));
    let staffing=connected.then(||industry_operations::current_site_staffing(w,district,kind)).flatten();
    let required=current.as_ref().map(|s|s.jobs_required);
    let unmatched=population::active(w) && required.is_some_and(|jobs|jobs>0.0) && staffing.is_none();
    let consumer_controlled=matches!(kind,K::ResearchCenter|K::ArmsPlant);
    let available=(!unmatched).then(||current.as_ref().map(|s|s.jobs_required*s.worker_fraction)).flatten();
    let mut actual=operation(w,me,district,kind);
    let mut effects=vec![];
    if let Some(op)=actual.as_mut() {
        if op["owner_verified"]==true {
            let day=op["day"].as_i64().unwrap() as i32;
            let contribution=w.province_economy.as_ref().filter(|p|p.flows.day==Some(day))
                .and_then(|p|p.flows.receipts.get(&format!("site:{district}:{}",kind.key())));
            if let Some(row)=contribution {
                if row.counted {
                    op["value_added_bn"]=json!(row.daily_value_added_bn);
                    effects.push(json!({"label":"Recorded annualized incremental value added",
                        "value":gdp_projects::incremental_gdp_bn(row),"unit":"$bn/year",
                        "note":format!("From the {} operating receipt at constant model prices. This is the same province/national GDP contribution, counted once; it is not profit, sales or a Treasury payment. {}",dated(day),row.valuation_basis)}));
                    if row.inherited_annual_gdp_bn!=0.0 {
                        effects.push(json!({"label":"Value added already in inherited GDP",
                            "value":row.inherited_annual_gdp_bn,"unit":"$bn/year",
                            "note":"Already present in the inherited economy; excluded from incremental GDP. Do not add it again."}));
                    }
                } else {
                    effects.push(json!({"label":"Output valuation unavailable","value":null,"unit":"$bn/year",
                        "note":row.reason.as_deref().unwrap_or("The GDP ledger does not count this receipt.")}));
                }
            }
        }
    }
    let mut requirements=vec![];
    if let Some(row)=current.as_ref().filter(|_|!passive(kind)) {
        for (id,label,fraction,note) in [
            ("workers","Qualified staffing",row.worker_fraction,"Coverage of the required qualifications. Assigned people are shared with other employers; these are not newly created workers."),
            ("power","Power and local grid",row.power_fraction,"Current utility support; actual dispatch is shared with other operating facilities."),
            ("inputs","Operating inputs and storage",row.input_fraction,"Raw materials, intermediate goods and finished-goods room required for operation. They are not construction requirements."),
            ("funding","Operating department funds",row.funding_fraction,"Conditional current operating authorization. Availability is neither cash already spent nor a payment guarantee.")
        ] {
            if consumer_controlled && id!="workers" {continue;}
            let known=!(unmatched && id=="workers");
            requirements.push(json!({"id":id,"label":label,"required":1.0,"available":known.then_some(fraction),"unit":"share",
                "ready":known && fraction>=1.0-1e-9,"note":if known{note}else{"Installed capacity changed after the last workforce match. Qualified coverage is unknown until the normal population system assigns workers."}}));
        }
        requirements.extend(rated_requirements(w,me,district,kind,capacity));
    }
    let used=actual.as_ref().and_then(|o|o["used_workers"].as_f64());
    let readiness=current.as_ref().map_or_else(||json!({"label":"Detailed readiness is not enabled",
        "note":"Adopt the connected economy to read workforce and operating constraints. Reading this panel does not change campaign rules.","fraction":null,"requirements":[]}),|row|json!({
        "label":if passive(kind){"Installed support capability"}else if unmatched{"Awaiting workforce matching"}else if consumer_controlled{"Readiness depends on selected work"}else if row.utilization<=1e-9{"Blocked for current work"}else if row.utilization<1.0-1e-9{"Limited for current work"}else{"Ready for the next operation"},
        "note":if passive(kind){production::catalog(kind).effect.to_string()}else if unmatched{"The installed site has not matched the population ledger's current assignments. The next normal workforce match determines its staffing; this reading does not hire anyone.".into()}else if consumer_controlled{"This facility supports selected research or equipment work. Its owning screen checks the active project's goods, authority, capacity reservations and other prerequisites; installed staffing alone does not prove that work is ready.".into()}else{format!("{} This is current readiness; only a dated receipt proves operation.",row.reason)},
        "fraction":if passive(kind)||unmatched||consumer_controlled{None}else{Some(row.utilization)},"requirements":requirements}));
    json!({"id":format!("site:{district}:{}",kind.key()),"installed_capacity":capacity,
        "installed_label":format!("{capacity:.6} installed level equivalents"),
        "installed_note":"Installed capacity includes all completed additions at this site. Older saves do not retain each building's completion date or paid contract history; none is reconstructed. Finishing construction is not proof of staffed output or a second GDP award.",
        "staffing":{"required":required,"assigned":staffing.as_ref().map(|s|s.jobs_assigned),
            "available":available,"used":used,"matched_day":staffing.as_ref().and_then(|s|s.matched_day),
            "matched_date":staffing.as_ref().and_then(|s|s.matched_day).map(dated),
            "label":"Current workforce and recorded use",
            "note":if staffing.as_ref().is_some_and(|s|s.matched_day.is_some()){"Assigned workers come from the population ledger's dated workforce match. Qualified staffing coverage is current feasible coverage of installed job requirements; workers used belong to the separate operating receipt date."}
                else if staffing.is_some(){"Assigned workers come from the initial population enrollment, which does not simulate or record an operating date. Qualified coverage is a current staffing measure; dated worker use is shown separately only after operation."}
                else if population::active(w){"No matching population assignment is available for this installed capacity yet. Assigned workers stay unknown until the real workforce match; current qualified coverage is not a hiring result. Used workers are shown only when saved in an operating receipt."}
                else{"Detailed population matching is not enabled. Staffing coverage is a model estimate, not a recorded employee count. Historical worker use stays unknown where the receipt did not store it."}},
        "readiness":readiness,"operation":actual,"effects":effects,
        "outcome_action":action(district,kind)})
}

#[cfg(test)]
mod tests {
    use super::*;
    use spheres_sim::{industry, init::world_1990, save, world::GameRules};

    // Disclosed installed assets/receipts exercise presentation only. Native
    // completion and browser command journeys own the construction mechanics.
    fn base()->(WorldState,String) {
        let mut w=world_1990(GameRules{daily_simulation:true,production_system:true,
            resource_market:true,industry_rebuild:true,..Default::default()});
        w.player=Some(NationId::France);
        let district=w.districts.iter().find(|(_,owner)|**owner==NationId::France).unwrap().0.clone();
        (w,district)
    }
    fn processor_receipt(w:&mut WorldState,district:&str,stamped:bool) {
        let day=clock::absolute_day(w);
        w.production.industry.last_day=Some(day);
        w.production.industry.operations.push(industry::SiteStatus{
            district:district.into(),kind:K::StarterIndustry,level:0,capacity_micros:Some(10_000),
            operation:stamped.then_some(industry::OperationContext{nation:NationId::France,day,
                installed_capacity:0.01,jobs_required:40.0,jobs_assigned:Some(30.0),jobs_used:10.0}),
            status:"limited".into(),reason:Some("Recorded raw-input shortage".into()),
            output_daily:0.0025,power_used_daily:0.0025,cash_spent_daily_bn:0.00000003});
    }
    #[test]
    fn s07_new_capacity_waits_for_matching_and_never_invents_completed_output() {
        let (mut w,district)=base();
        population::enable(&mut w).unwrap();
        w.production.industry.modules.insert(district.clone(),10_000);
        let before=save(&w);let v=for_site(&w,NationId::France,&district,K::StarterIndustry);
        assert_eq!(v["installed_capacity"],0.01);assert_eq!(v["staffing"]["required"],40.0);
        assert!(v["staffing"]["assigned"].is_null());assert!(v["staffing"]["available"].is_null());
        assert!(v["staffing"]["used"].is_null());assert!(v["operation"].is_null());
        assert!(v["readiness"]["fraction"].is_null());
        assert_eq!(v["readiness"]["label"],"Awaiting workforce matching");
        assert!(v["effects"].as_array().unwrap().is_empty());
        assert_eq!(save(&w),before);assert_eq!(for_site(&w,NationId::France,&district,K::StarterIndustry),v);
    }
    #[test]
    fn s07_expansion_keeps_dated_capacity_workers_and_charges_separate_from_current_jobs() {
        let (mut w,district)=base();w.production.industry.modules.insert(district.clone(),10_000);
        population::enable(&mut w).unwrap();
        let matching=industry_operations::current_site_staffing(&w,&district,K::StarterIndustry).unwrap();
        let before=for_site(&w,NationId::France,&district,K::StarterIndustry);
        assert_eq!(before["staffing"]["assigned"],matching.jobs_assigned);
        processor_receipt(&mut w,&district,true);
        w.production.industry.modules.insert(district.clone(),20_000);
        let saved=save(&w);let v=for_site(&w,NationId::France,&district,K::StarterIndustry);
        assert_eq!(v["installed_capacity"],0.02);assert_eq!(v["staffing"]["required"],80.0);
        assert!(v["staffing"]["assigned"].is_null());assert!(v["staffing"]["available"].is_null());
        assert_eq!(v["operation"]["installed_capacity"],0.01);
        assert_eq!(v["operation"]["required_workers"],40.0);
        assert_eq!(v["operation"]["assigned_workers"],30.0);
        assert_eq!(v["operation"]["used_workers"],10.0);
        assert_eq!(v["operation"]["output"],0.0025);
        assert_eq!(v["operation"]["cash_spent_bn"],0.00000003);
        assert_eq!(v["staffing"]["used"],10.0);assert_eq!(save(&w),saved);
    }
    #[test]
    fn s07_legacy_and_captured_receipts_do_not_gain_new_national_or_worker_attribution() {
        let (mut w,district)=base();w.production.industry.modules.insert(district.clone(),10_000);
        processor_receipt(&mut w,&district,false);
        let legacy=for_site(&w,NationId::France,&district,K::StarterIndustry);
        assert_eq!(legacy["operation"]["output"],0.0025);
        assert_eq!(legacy["operation"]["owner_verified"],false);
        assert!(legacy["operation"]["used_workers"].is_null());
        assert!(legacy["operation"]["value_added_bn"].is_null());
        w.production.industry.operations.clear();processor_receipt(&mut w,&district,true);
        w.districts.insert(district.clone(),NationId::Japan);
        assert!(for_site(&w,NationId::France,&district,K::StarterIndustry).is_null());
        let captured=for_site(&w,NationId::Japan,&district,K::StarterIndustry);
        assert!(captured["operation"].is_null(),"A captured building does not transfer the former owner's receipt");
    }
    #[test]
    fn s07_new_industry_receipts_match_gdp_date_and_do_not_double_count_inherited_output() {
        let (mut w,district)=base();w.production.rebuild_sites.insert(district.clone(),[1,0,0]);
        w.province_economy=Some(Default::default());
        let mut receipt=industry_operations::site(&w,&district,K::OfficeDistrict);
        receipt.recorded_day=Some(clock::absolute_day(&w));receipt.status="running".into();
        receipt.reason="Recorded paid operation".into();receipt.output_daily=0.0001;
        receipt.jobs_filled=4000.0;receipt.cash_spent_daily_bn=0.000004;
        receipt.power_used_daily=0.1;
        receipt.actual_operating_capacity=Some(0.4);receipt.actual_utilization=Some(0.4);
        w.production.operations.receipts.push(receipt);
        gdp_projects::record_office(&mut w,NationId::France,&district,0.4,0.1,[0.0;12],0.000004,0.0000002);
        let key=format!("site:{district}:office_district");
        let flow=w.province_economy.as_mut().unwrap().flows.receipts.get_mut(&key).unwrap();
        // An overlap can be saved after national/territorial reconciliation.
        flow.inherited_annual_gdp_bn=flow.annual_gdp_bn*0.25;
        let incremental=gdp_projects::incremental_gdp_bn(flow);let actual_daily=flow.daily_value_added_bn;
        let saved=save(&w);let board=crate::industry_json(&w,NationId::France);
        let site=board["sites"].as_array().unwrap().iter().find(|r|r["kind"]=="office_district").unwrap();
        assert_eq!(site["has_receipt"],true);assert_eq!(site["output_daily"],0.0001);
        assert_eq!(site["cash_spent_daily_bn"],0.000004);
        assert_eq!(board["power"]["used_daily"],0.1);
        assert_eq!(board["power"]["recorded_facility_count"],1);
        assert_eq!(site["lifecycle"]["operation"]["value_added_bn"],actual_daily);
        assert_eq!(site["lifecycle"]["effects"][0]["value"],incremental);
        assert_eq!(site["lifecycle"]["effects"].as_array().unwrap().len(),2);
        assert_eq!(save(&w),saved);
        let next_day=clock::absolute_day(&w)+1;
        w.province_economy.as_mut().unwrap().flows.day=Some(next_day);
        let mismatched=for_site(&w,NationId::France,&district,K::OfficeDistrict);
        assert!(mismatched["operation"]["value_added_bn"].is_null());
        assert!(mismatched["effects"].as_array().unwrap().is_empty());
    }
    #[test]
    fn s07_starter_package_has_one_producer_and_completed_construction_links_to_it() {
        let (mut w,district)=base();w.production.industry.modules.insert(district.clone(),10_000);
        processor_receipt(&mut w,&district,true);
        let before=save(&w);let board=crate::industry_json(&w,NationId::France);
        let sites=board["sites"].as_array().unwrap();
        assert_eq!(sites.len(),1,"Bundled estate, grid and generation are not additional pack producers");
        assert_eq!(sites[0]["kind"],"starter_industry");assert_eq!(sites[0]["output_daily"],0.0025);
        let construction=crate::production_json(&w,NationId::France);
        let completed=construction["completed"].as_array().unwrap().iter()
            .find(|r|r["province"]["id"]==district).unwrap();
        let actions=completed["outcome_actions"].as_array().unwrap();assert_eq!(actions.len(),1);
        assert_eq!(actions[0],action(&district,K::StarterIndustry));
        assert_eq!(construction["as_of_day"],clock::absolute_day(&w));assert_eq!(construction["date"],w.date_str());
        assert_eq!(save(&w),before);
    }
    #[test]
    fn s07_rated_operating_recipe_is_native_scaled_and_separate_from_recorded_consumption() {
        let (mut w,district)=base();w.production.industry.modules.insert(district.clone(),10_000);
        processor_receipt(&mut w,&district,true);
        let saved=save(&w);let lifecycle=for_site(&w,NationId::France,&district,K::StarterIndustry);
        let rows=lifecycle["readiness"]["requirements"].as_array().unwrap();
        let rate=industry::plant_rate(&w,&district,K::StarterIndustry);
        let power=rate*industry::power_per_pack(&w,&district,K::StarterIndustry);
        let raw=industry::company_operating_recipe(&w,NationId::France,&district,K::StarterIndustry,rate,power);
        for commodity in resources::ALL.into_iter().filter(|c|raw[c.idx()]>0.0) {
            let row=rows.iter().find(|r|r["id"]==format!("raw_{}",commodity.key())).unwrap();
            assert_eq!(row["required"],raw[commodity.idx()]);
            assert_eq!(row["available"],resources::stockpile(&w,NationId::France,commodity));
            assert_eq!(row["unit"],commodity.unit());
        }
        let authority=rows.iter().find(|r|r["id"]=="operating_authority").unwrap();
        assert_eq!(authority["available"],programs::available_bn(&w,NationId::France,BUDGET_INDUSTRY,2));
        assert_eq!(rows.iter().find(|r|r["id"]=="national_power").unwrap()["required"],power);
        assert_eq!(lifecycle["operation"]["output"],0.0025);
        assert_eq!(lifecycle["operation"]["power_used_daily"],0.0025);
        assert_eq!(save(&w),saved);
    }
    #[test]
    fn s07_power_summary_never_combines_different_receipt_dates() {
        let (mut w,district)=base();w.production.industry.modules.insert(district.clone(),10_000);
        processor_receipt(&mut w,&district,true);
        w.production.industry.operations[0].operation.as_mut().unwrap().day-=1;
        w.production.rebuild_sites.insert(district.clone(),[1,0,0]);
        let mut office=industry_operations::site(&w,&district,K::OfficeDistrict);
        office.recorded_day=Some(clock::absolute_day(&w));office.power_used_daily=0.125;
        w.production.operations.receipts.push(office);
        let before=save(&w);let board=crate::industry_json(&w,NationId::France);
        assert_eq!(board["power"]["used_daily"],0.125);
        assert_eq!(board["power"]["recorded_facility_count"],1);
        assert_eq!(board["power"]["receipt_day"],clock::absolute_day(&w));
        let processor=board["sites"].as_array().unwrap().iter().find(|s|s["kind"]=="starter_industry").unwrap();
        assert_eq!(processor["power_used_daily"],0.0025,"Older individual receipt remains visible with its own date");
        assert_eq!(save(&w),before);
    }
    #[test]
    fn s07_shipyard_separates_dock_operation_energy_and_equipment_funding_actions() {
        let (mut w,district)=base();w.production.rebuild_sites.insert(district.clone(),[0,1,0]);
        let before=save(&w);let board=crate::industry_json(&w,NationId::France);
        let dock=board["sites"].as_array().unwrap().iter().find(|s|s["kind"]=="shipyard").unwrap();
        let actions=dock["actions"].as_array().unwrap();
        for (ministry,department,label) in [("industry",0,"Review dock operating funds"),
            ("industry",1,"Review energy funding"),("defense",3,"Review equipment funding")] {
            let matching:Vec<_>=actions.iter().filter(|a|a["action"]=="budget"
                && a["ministry"]==ministry && a["department"]==department).collect();
            assert_eq!(matching.len(),1);assert_eq!(matching[0]["label"],label);
        }
        assert!(actions.iter().any(|a|a["action"]=="resources"));
        assert!(actions.iter().any(|a|a["action"]=="manufacture" && a["district"]==district));
        assert_eq!(save(&w),before);
    }
}
