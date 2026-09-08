// Reviewed actual maintenance invoices. No balance outside ProgramBudget.
// Legacy maintenance is an explicit game assumption: 4% of catalog purchase
// value annually, spread over 365 days. Custom revisions retain frozen upkeep.
pub const LEGACY_MAINTENANCE_ANNUAL_RATE:f64=0.04;

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq)]
#[serde(deny_unknown_fields)]
pub struct FleetMaintenance {
    pub from_day:i32,
    pub daily_limit_bn:f64,
    pub receipt:Option<MaintenanceReceipt>,
}
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq)]
#[serde(deny_unknown_fields)]
pub struct MaintenanceReceipt {
    pub day:i32,
    pub custom_required_bn:f64,
    pub legacy_required_bn:f64,
    pub custom_paid_bn:f64,
    pub legacy_paid_bn:f64,
    pub authority_bn:f64,
    pub daily_limit_bn:f64,
}
pub fn maintenance_plan_on(n:&Nation,day:i32)->bool {
    n.equipment.as_ref().and_then(|s|s.maintenance_plan.as_ref()).is_some_and(|p|day>=p.from_day)
}
pub fn legacy_maintenance_requirement(n:&Nation)->f64 {
    n.arsenal.held.iter().filter(|h|h.design_id.is_none()).filter_map(|h|crate::arsenal::DECK.get(h.kit as usize).map(|d|h.units.max(0.0)*d.unit_cost*LEGACY_MAINTENANCE_ANNUAL_RATE/365.0)).sum::<f64>()+0.0
}
/// MODEL dockside repair efficiency: each staffed, supplied shipyard reduces
/// the inherited naval maintenance invoice by 5%, capped at 25%. It buys no
/// vessels, restores no age condition, and leaves frozen custom upkeep intact.
pub const SHIPYARD_REPAIR_REDUCTION_PER_LEVEL:f64=0.05;
pub const SHIPYARD_REPAIR_MAX_REDUCTION:f64=0.25;
#[derive(Clone,Debug,Serialize,PartialEq)]
pub struct ShipyardRepairSupport {
    pub operating_shipyards:f64,
    pub naval_requirement_before_bn:f64,
    pub naval_reduction_fraction:f64,
    pub savings_daily_bn:f64,
    pub legacy_requirement_after_bn:f64,
}
pub fn shipyard_repair_support(w:&WorldState,id:NationId)->ShipyardRepairSupport {
    let n=w.nation(id);
    let base=legacy_maintenance_requirement(n);
    let naval=n.arsenal.held.iter().filter(|h|h.design_id.is_none())
        .filter_map(|h|crate::arsenal::DECK.get(h.kit as usize)
            .filter(|kit|kit.class==crate::arsenal::Class::Naval)
            .map(|kit|h.units.max(0.0)*kit.unit_cost*LEGACY_MAINTENANCE_ANNUAL_RATE/365.0)).sum::<f64>();
    let operating=crate::industry_operations::naval_capacity_bonus(w,id);
    let fraction=(operating*SHIPYARD_REPAIR_REDUCTION_PER_LEVEL).clamp(0.0,SHIPYARD_REPAIR_MAX_REDUCTION);
    let savings=(naval*fraction).min(base);
    ShipyardRepairSupport {operating_shipyards:operating,naval_requirement_before_bn:naval,
        naval_reduction_fraction:fraction,savings_daily_bn:savings,legacy_requirement_after_bn:(base-savings).max(0.0)}
}
/// The actual invoice, its review, and ammunition's shared-funding forecast all
/// use this one world-aware arm. Legacy rules retain the original arithmetic.
pub fn legacy_maintenance_requirement_world(w:&WorldState,id:NationId)->f64 {
    if !crate::industry_operations::enabled(w) {return legacy_maintenance_requirement(w.nation(id));}
    shipyard_repair_support(w,id).legacy_requirement_after_bn
}
pub fn legacy_maintenance_fraction(n:&Nation)->f64 {
    n.equipment.as_ref().and_then(|s|s.maintenance_plan.as_ref()).and_then(|p|p.receipt.as_ref()).map_or(1.0,|r|if r.legacy_required_bn>0.0{(r.legacy_paid_bn/r.legacy_required_bn).clamp(0.0,1.0)}else{1.0})
}
pub fn maintenance_plan_refusal(w:&WorldState,id:NationId,daily_limit_bn:f64)->Option<String> {
    actor_refusal(w,id).or_else(||budget_refusal(daily_limit_bn)).or_else(||{
        if w.nation(id).program_budget.is_none(){Some("Activate a departmental budget before setting a fleet maintenance plan.".into())}
        else{validate_state(w.nation(id)).err()}
    })
}
pub fn set_maintenance_plan(w:&mut WorldState,id:NationId,daily_limit_bn:f64)->Result<(),String> {
    if let Some(reason)=maintenance_plan_refusal(w,id,daily_limit_bn){return Err(reason);}
    let day=clock::absolute_day(w);
    let s=state_mut(w.nation_mut(id));
    if let Some(plan)=&mut s.maintenance_plan {plan.daily_limit_bn=daily_limit_bn;}
    else{s.maintenance_plan=Some(FleetMaintenance{from_day:day.saturating_add(1),daily_limit_bn,receipt:None});}
    Ok(())
}
fn settle_fleet_maintenance(w:&mut WorldState,id:NationId) {
    let day=clock::absolute_day(w);let n=w.nation(id);
    let Some(plan)=n.equipment.as_ref().and_then(|s|s.maintenance_plan.as_ref())else{return;};
    if day<plan.from_day||plan.receipt.as_ref().is_some_and(|r|r.day>=day){return;}
    let Some(budget)=n.program_budget.as_ref().filter(|p|p.day==Some(day)&&p.settled_day!=Some(day))else{return;};
    let custom=fleet_maintenance_requirement(n)+0.0;let legacy=legacy_maintenance_requirement_world(w,id);
    let authority=budget.available_bn[crate::world::BUDGET_DEFENSE][2]+budget.prepaid_bn[crate::world::BUDGET_DEFENSE][2];
    let limit=plan.daily_limit_bn;
    let paid=(custom+legacy).min(authority.max(0.0)).min(limit);
    if crate::programs::spend_operating(w,id,crate::world::BUDGET_DEFENSE,2,paid).is_err(){return;}
    let fraction=if custom+legacy>0.0{(paid/(custom+legacy)).clamp(0.0,1.0)}else{1.0};
    let custom_paid=(custom*fraction).min(paid);let legacy_paid=(paid-custom_paid).max(0.0);
    let s=state_mut(w.nation_mut(id));
    s.maintenance_required_today_bn=custom;
    s.maintenance_allocated_today_bn=custom_paid;
    s.maintenance_fraction=if custom>0.0{fraction}else{1.0};
    s.maintenance_plan.as_mut().unwrap().receipt=Some(MaintenanceReceipt{day,custom_required_bn:custom,legacy_required_bn:legacy,custom_paid_bn:custom_paid,legacy_paid_bn:legacy_paid,authority_bn:authority,daily_limit_bn:limit});
}
fn validate_maintenance_plan(n:&Nation)->Result<(),String> {
    let Some(s)=&n.equipment else{return Ok(());};
    let Some(plan)=&s.maintenance_plan else{return Ok(());};
    if s.version<4||n.program_budget.is_none()||budget_refusal(plan.daily_limit_bn).is_some(){return Err("Invalid fleet maintenance plan or funding envelope.".into());}
    if let Some(r)=&plan.receipt {
        if r.day<plan.from_day||[r.custom_required_bn,r.legacy_required_bn,r.custom_paid_bn,r.legacy_paid_bn,r.authority_bn,r.daily_limit_bn].iter().any(|v|!v.is_finite()||*v<0.0)
            ||r.custom_paid_bn>r.custom_required_bn+1e-12||r.legacy_paid_bn>r.legacy_required_bn+1e-12
            ||r.custom_paid_bn+r.legacy_paid_bn>r.authority_bn.min(r.daily_limit_bn)+1e-12 {
            return Err("Invalid fleet maintenance invoice.".into());
        }
        let near=|a:f64,b:f64|(a-b).abs()<=1e-12*a.abs().max(b.abs()).max(1.0);
        let required=r.custom_required_bn+r.legacy_required_bn;
        let paid=required.min(r.authority_bn).min(r.daily_limit_bn);
        let fraction=if required>0.0{(paid/required).clamp(0.0,1.0)}else{1.0};
        if n.program_budget.as_ref().and_then(|p|p.day).is_none_or(|d|r.day>d)
            ||!near(r.custom_paid_bn,(r.custom_required_bn*fraction).min(paid))
            ||!near(r.legacy_paid_bn,(paid-r.custom_paid_bn).max(0.0))
            ||!near(s.maintenance_required_today_bn,r.custom_required_bn)
            ||!near(s.maintenance_allocated_today_bn,r.custom_paid_bn)
            ||!near(s.maintenance_fraction,if r.custom_required_bn>0.0{fraction}else{1.0}) {
            return Err("Fleet maintenance receipt disagrees with its dated payment or supported inventory.".into());
        }
    }
    Ok(())
}

#[cfg(test)]
mod maintenance_tests {
    use super::*;
    use crate::{arsenal,programs,world::{GameRules,BUDGET_DEFENSE as D}};
    const USA:NationId=NationId::USA;
    fn fixture()->WorldState {
        let mut w=crate::init::world_1990(GameRules{daily_simulation:true,military_operations:true,manufacturing_system:true,..Default::default()});
        w.player=Some(USA);programs::set_construction_budget(&mut w,USA,0.0).unwrap();
        let spec=baseline_spec();let p=design_preview(&w,USA,&spec).profile.unwrap();let day=clock::absolute_day(&w);
        state_mut(w.nation_mut(USA)).revisions.insert("maint-model".into(),DesignRevision{id:"maint-model".into(),name:"Maintenance fixture".into(),specification_key:specification_key(&spec),spec,profile:p,created_day:day,certified_day:Some(day)});
        arsenal::deliver_design(w.nation_mut(USA),"maint-model",10,25.0).unwrap();
        w.nation_mut(USA).treasury_bn=Some(1000.0);w
    }
    fn receipt(w:&WorldState)->&MaintenanceReceipt {w.nation(USA).equipment.as_ref().unwrap().maintenance_plan.as_ref().unwrap().receipt.as_ref().unwrap()}
    fn open_next(w:&mut WorldState){clock::advance_date(w);programs::begin_day(w);}
    fn near(a:f64,b:f64){assert!((a-b).abs()<1e-10,"{a} != {b}");}
    #[test]
    fn activation_is_prospective_and_does_not_reclassify_an_open_day() {
        let mut w=fixture();programs::begin_day(&mut w);let before=w.nation(USA).program_budget.clone();let treasury=w.nation(USA).treasury_bn;
        set_maintenance_plan(&mut w,USA,0.001).unwrap();assert_eq!(w.nation(USA).program_budget,before);assert_eq!(w.nation(USA).treasury_bn,treasury);
        assert!(!programs::is_project_funded(w.nation(USA),D,2));settle_support(&mut w);assert!(w.nation(USA).equipment.as_ref().unwrap().maintenance_plan.as_ref().unwrap().receipt.is_none());
        open_next(&mut w);assert!(programs::is_project_funded(w.nation(USA),D,2));near(w.nation(USA).program_budget.as_ref().unwrap().spent_today_bn[D][2],0.0);
        settle_support(&mut w);assert!(receipt(&w).custom_paid_bn>0.0);assert_eq!(w.nation(USA).treasury_bn,treasury);
    }
    #[test]
    fn invoice_is_prorated_paid_once_and_posted_once_by_the_fiscal_owner() {
        let mut w=fixture();let total=fleet_maintenance_requirement(w.nation(USA))+legacy_maintenance_requirement(w.nation(USA));let cap=total*0.5;
        assert!(legacy_maintenance_requirement(w.nation(USA))>0.0);set_maintenance_plan(&mut w,USA,cap).unwrap();open_next(&mut w);
        let before=w.nation(USA).program_budget.as_ref().unwrap().available_bn[D][2];settle_support(&mut w);let r=receipt(&w).clone();
        near(r.custom_paid_bn+r.legacy_paid_bn,cap);near(r.custom_paid_bn/r.custom_required_bn,0.5);near(r.legacy_paid_bn/r.legacy_required_bn,0.5);
        near(before-w.nation(USA).program_budget.as_ref().unwrap().available_bn[D][2],cap);
        let paid=crate::save(&w);settle_support(&mut w);assert_eq!(crate::save(&w),paid);
        programs::stage_fiscal(w.nation_mut(USA),0.0,0.0);let bill:f64=w.nation(USA).program_budget.as_ref().unwrap().spent_today_bn.iter().flatten().sum();
        let treasury=w.nation(USA).treasury_bn.unwrap();programs::finish_day(&mut w);near(treasury-w.nation(USA).treasury_bn.unwrap(),bill);
        let settled=crate::save(&w);programs::finish_day(&mut w);settle_support(&mut w);assert_eq!(crate::save(&w),settled);
    }
    #[test]
    fn zero_funding_reduces_supported_stock_without_deleting_or_rejuvenating_it() {
        let mut w=fixture();set_maintenance_plan(&mut w,USA,0.0).unwrap();open_next(&mut w);
        let held=serde_json::to_string(&w.nation(USA).arsenal.held).unwrap();settle_support(&mut w);assert_eq!(serde_json::to_string(&w.nation(USA).arsenal.held).unwrap(),held);
        assert_eq!(w.nation(USA).equipment.as_ref().unwrap().maintenance_fraction,0.0);assert_eq!(legacy_maintenance_fraction(w.nation(USA)),0.0);
        assert!(w.nation(USA).arsenal.held.iter().all(|h|arsenal::combat_value(w.nation(USA),h)==0.0));
        assert_eq!(programs::refill_multiplier(w.nation(USA),None),0.0);
    }
    #[test]
    fn delivered_vehicles_are_billed_but_pending_orders_are_not_and_retirement_lowers_future_cost() {
        let mut w=fixture();set_maintenance_plan(&mut w,USA,1000.0).unwrap();
        arsenal::queue_design_order(w.nation_mut(USA),"maint-model",500,7,0.0).unwrap();open_next(&mut w);
        arsenal::deliver_design(w.nation_mut(USA),"maint-model",2,0.0).unwrap();settle_support(&mut w);
        let per=profile(w.nation(USA),"maint-model").unwrap().maintenance_bn_day;near(receipt(&w).custom_required_bn,12.0*per);
        retire(&mut w,USA,"maint-model",2).unwrap();let old=receipt(&w).clone();settle_support(&mut w);assert_eq!(*receipt(&w),old);
        open_next(&mut w);settle_support(&mut w);near(receipt(&w).custom_required_bn,10.0*per);assert!(receipt(&w).custom_paid_bn<old.custom_paid_bn);
    }
    #[test]
    fn fiscal_expiry_limits_invoices_but_eligible_prepaid_funds_are_not_charged_again() {
        let mut w=fixture();set_maintenance_plan(&mut w,USA,1.0).unwrap();w.year=1991;w.month=1;w.day=1;
        programs::begin_day(&mut w);let prepaid=0.000001;w.nation_mut(USA).program_budget.as_mut().unwrap().prepaid_bn[D][2]=prepaid;
        settle_support(&mut w);near(receipt(&w).custom_paid_bn+receipt(&w).legacy_paid_bn,prepaid);
        let p=w.nation(USA).program_budget.as_ref().unwrap();near(p.spent_today_bn[D][2],0.0);near(p.prepaid_used_today_bn[D][2],prepaid);
        assert!(receipt(&w).authority_bn<receipt(&w).custom_required_bn+receipt(&w).legacy_required_bn);
    }
    #[test]
    fn invoice_and_target_envelope_roundtrip_preserves_continuation_and_rejects_forged_receipts() {
        let mut w=fixture();set_maintenance_plan(&mut w,USA,0.001).unwrap();open_next(&mut w);settle_support(&mut w);
        let text=crate::save(&w);let mut loaded=crate::load(&text).unwrap();assert_eq!(crate::save(&loaded),text);
        for _ in 0..3 {crate::tick_day(&mut w,&[]);crate::tick_day(&mut loaded,&[]);}assert_eq!(crate::save(&w),crate::save(&loaded));
        let bad=loaded.nation_mut(USA).equipment.as_mut().unwrap().maintenance_plan.as_mut().unwrap().receipt.as_mut().unwrap();bad.custom_paid_bn=bad.custom_required_bn+1.0;
        assert!(validate_state(loaded.nation(USA)).is_err());assert!(crate::load(&crate::save(&loaded)).is_err());
    }
    #[test]
    fn malformed_limit_and_version_three_plan_are_refused_without_mutation() {
        let mut w=fixture();let saved=crate::save(&w);
        for limit in [-1.0,f64::NAN,f64::INFINITY,1001.0] {assert!(set_maintenance_plan(&mut w,USA,limit).is_err());assert_eq!(crate::save(&w),saved);}
        set_maintenance_plan(&mut w,USA,0.01).unwrap();w.nation_mut(USA).equipment.as_mut().unwrap().version=3;
        assert!(validate_state(w.nation(USA)).is_err());
    }
    #[test]
    fn forged_payment_splits_support_and_future_invoice_dates_are_rejected() {
        let mut w=fixture();set_maintenance_plan(&mut w,USA,0.000001).unwrap();open_next(&mut w);settle_support(&mut w);
        for flaw in 0..3 {
            let mut wrong=w.clone();let s=wrong.nation_mut(USA).equipment.as_mut().unwrap();let r=s.maintenance_plan.as_mut().unwrap().receipt.as_mut().unwrap();
            match flaw {0=>{r.custom_paid_bn+=1e-7;r.legacy_paid_bn-=1e-7;},1=>s.maintenance_fraction=1.0,_=>r.day+=10}
            assert!(validate_state(wrong.nation(USA)).is_err());assert!(crate::load(&crate::save(&wrong)).is_err());
        }
    }
    #[test]
    fn powered_shipyard_repair_quote_matches_the_paid_naval_invoice_and_preserves_inventory() {
        let mut w=fixture();
        w.rules.industry_rebuild=true;w.rules.production_system=true;w.rules.resource_market=true;
        let district=w.districts.iter().find(|(d,n)|**n==USA&&crate::logistics::has_terminal(d)).unwrap().0.clone();
        for kind in [crate::production::ProjectKind::Shipyard,crate::production::ProjectKind::PowerGrid,crate::production::ProjectKind::Generation] {
            crate::production::complete_capability(&mut w,&district,kind);
        }
        crate::resources::set_stockpile_for_test(&mut w,USA,crate::resources::Commodity::Coal,1000.0);
        set_maintenance_plan(&mut w,USA,1000.0).unwrap();open_next(&mut w);
        crate::industry_operations::begin_day(&mut w);
        let quote=shipyard_repair_support(&w,USA);
        assert!(quote.operating_shipyards>0.0);
        assert!(quote.naval_requirement_before_bn>0.0);
        near(quote.naval_reduction_fraction,0.05);
        near(quote.savings_daily_bn,quote.naval_requirement_before_bn*0.05);
        let base=legacy_maintenance_requirement(w.nation(USA));
        near(quote.legacy_requirement_after_bn,base-quote.savings_daily_bn);
        let held=serde_json::to_string(&w.nation(USA).arsenal.held).unwrap();
        let custom=fleet_maintenance_requirement(w.nation(USA));
        // Give the test invoice sufficient appropriation; the fixture measures
        // service efficiency, not the unrelated annual-budget constraint.
        w.nation_mut(USA).program_budget.as_mut().unwrap().available_bn[D][2]=1000.0;
        settle_support(&mut w);
        near(receipt(&w).legacy_required_bn,quote.legacy_requirement_after_bn);
        near(receipt(&w).legacy_paid_bn,quote.legacy_requirement_after_bn);
        near(receipt(&w).custom_required_bn,custom);
        assert_eq!(serde_json::to_string(&w.nation(USA).arsenal.held).unwrap(),held,"repair efficiency cannot fabricate or rejuvenate ships");
        let once=crate::save(&w);settle_support(&mut w);assert_eq!(crate::save(&w),once);
        assert_eq!(crate::save(&crate::load(&once).unwrap()),once);
        // A local outage removes future savings, not previously paid invoices.
        open_next(&mut w);
        w.production.provinces.iter_mut().find(|p|p.district==district).unwrap().power_grid=0;
        crate::industry_operations::begin_day(&mut w);
        near(shipyard_repair_support(&w,USA).savings_daily_bn,0.0);
        near(legacy_maintenance_requirement_world(&w,USA),base);
        settle_support(&mut w);near(receipt(&w).legacy_required_bn,base);
        w.rules.industry_rebuild=false;
        near(legacy_maintenance_requirement_world(&w,USA),legacy_maintenance_requirement(w.nation(USA)));
    }
}
