// Finite assistance for committed custom-equipment work. Buying is explicit;
// merely reading a board or quote never reserves stock, cash or freight.
#[derive(Clone, Debug, Serialize)]
pub struct ReplenishmentQuote {
    pub valid: bool,
    pub reason: Option<String>,
    pub horizon_days: u32,
    pub spending_cap_bn: f64,
    pub equipment_demand: [f64; 12],
    pub requested: [f64; 12],
    pub stock: [f64; 12],
    pub paid_inbound: [f64; 12],
    pub receipt: crate::resources::RawPurchaseReceipt,
    pub note: String,
}

pub fn replenishment_quote(w: &WorldState, id: NationId, horizon_days: u32, spending_cap_bn: f64) -> ReplenishmentQuote {
    replenishment_proposal(w,id,horizon_days,spending_cap_bn).0
}

fn replenishment_proposal(w: &WorldState,id:NationId,horizon_days:u32,spending_cap_bn:f64) -> (ReplenishmentQuote,Option<WorldState>) {
    let mut q = ReplenishmentQuote {
        valid:false,reason:actor_refusal(w,id),horizon_days,spending_cap_bn,
        equipment_demand:[0.0;12],requested:[0.0;12],stock:[0.0;12],paid_inbound:[0.0;12],
        receipt:Default::default(),
        note:"This one-time purchase targets the funded custom-equipment material bill after retained shared stock, domestic supply, existing contracts and paid cargo. Paused work is excluded. The warehouse is shared: other consumers can use these inputs, and actual later consumption may create another gap. Current prices, supplier reserves and shared freight capacity determine the exact shipment; future funding, production and arrival remain conditional. Raw warehouse reserve targets are not storage limits. No automatic repeat purchase is created.".into(),
    };
    let h = [30,90,365].iter().position(|d|*d==horizon_days);
    if q.reason.is_none() && (h.is_none() || !spending_cap_bn.is_finite() || spending_cap_bn <= 0.0) {
        q.reason=Some("Choose a 30, 90 or 365 day supply horizon and a positive finite spending limit.".into());
    }
    if q.reason.is_none() { q.reason=w.nation_opt(id).and_then(|n|validate_state(n).err()); }
    if q.reason.is_some() {return(q,None);}
    let h=h.unwrap();
    let forecast=crate::economic_ai::raw_supply_forecast(w,id);
    for line in forecast.lines {
        let i=line.commodity.idx();
        q.equipment_demand[i]=line.equipment_horizon[h];
        q.stock[i]=line.stock;
        let all_pending=crate::logistics::pending(w,id,line.commodity);
        q.paid_inbound[i]=all_pending;
        // Do not buy the same paid material again merely because its route is
        // delayed/held or its ETA falls beyond the selected review window.
        let later_pending=(all_pending-crate::logistics::pending_within_days(w,id,line.commodity,horizon_days as i32)).max(0.0);
        // This finite assistance does not enlarge a custom order to refill
        // unrelated recurring demand. Retained coverage is shared rather than
        // reserved: actual later consumption can create another reviewable gap.
        q.requested[i]=(line.equipment_horizon[h]-line.coverage[h]-later_pending).max(0.0);
    }
    if q.requested.iter().all(|need|*need<=1e-9) {
        q.reason=Some("No additional purchase is needed for funded custom-equipment work in this horizon after shared supply and paid cargo. Review existing deliveries if current work is waiting.".into());
        return(q,None);
    }
    let mut proposed=w.clone();
    match crate::resources::purchase_raw_bundle(&mut proposed,id,q.requested,spending_cap_bn) {
        Ok(receipt)=>{q.receipt=receipt;q.valid=true;(q,Some(proposed))},
        Err(reason)=>{q.reason=Some(reason);(q,None)},
    }
}

/// Recalculate against live supply at confirmation, and commit only the fully
/// prepared result. A failed or stale request leaves every world byte intact.
pub fn replenish(w:&mut WorldState,id:NationId,horizon_days:u32,spending_cap_bn:f64)->Result<(),String> {
    replenish_with_receipt(w,id,horizon_days,spending_cap_bn).map(|_|())
}

// The manual command and optional policy use this same atomic transaction.
// Returning its actual receipt adds audit data, not another payment path.
fn replenish_with_receipt(w:&mut WorldState,id:NationId,horizon_days:u32,spending_cap_bn:f64)->Result<crate::resources::RawPurchaseReceipt,String> {
    let (quote,proposed)=replenishment_proposal(w,id,horizon_days,spending_cap_bn);
    let Some(mut proposed)=proposed else{return Err(quote.reason.unwrap_or_else(||"No material purchase is available.".into()));};
    proposed.headline(format!("EQUIPMENT: {} purchases {} material shipments for ${:.3}m within a ${:.3}m limit; paid freight arrives through the resource ledger.",id.name(),quote.receipt.fills.len(),quote.receipt.spent_bn*1000.0,spending_cap_bn*1000.0));
    *w=proposed;
    Ok(quote.receipt)
}

#[cfg(test)]
mod replenishment_tests {
    use super::*;
    use crate::{resources::{self,Commodity},world::GameRules};
    pub(super) fn fixture()->WorldState {
        let id=NationId::France;
        let mut w=crate::init::world_1990(GameRules {daily_simulation:true,military_operations:true,production_system:true,manufacturing_system:true,resource_market:true,resource_gates:true,logistics_routes:true,physical_logistics:true,..Default::default()});
        w.player=Some(id);crate::programs::set_construction_budget(&mut w,id,0.0).unwrap();resources::warm(&mut w);
        let spec=default_spec("ground_ifv");let mut p=design_preview(&w,id,&spec).profile.unwrap();
        // A synthetic frozen test recipe makes the custom contract's shortage
        // unambiguous, independently of national starting-cover assumptions.
        p.recipe=[0.0;12];p.recipe[Commodity::Copper.idx()]=1_000_000.0;
        let today=clock::absolute_day(&w);
        let s=w.nation_mut(id).equipment.get_or_insert_with(Default::default);s.finance_from_day=today;
        s.revisions.insert("supply-ifv".into(),DesignRevision{id:"supply-ifv".into(),name:"Supply IFV".into(),specification_key:specification_key(&spec),spec,profile:p,created_day:today,certified_day:Some(today)});
        let district=w.districts.iter().find(|(_,n)|**n==id).unwrap().0.clone();
        for _ in 0..3{crate::production::complete_capability(&mut w,&district,crate::production::ProjectKind::ArmsPlant);}
        start_production(&mut w,id,"supply-ifv",&district,10,0.5).unwrap();
        w.day=20;
        let p=&mut w.nation_mut(id).equipment.as_mut().unwrap().projects[0];p.work_days=p.tooling_days as f64;p.spent_bn=p.tooling_cost_bn;
        resources::set_stockpile_for_test(&mut w,id,Commodity::Copper,0.0);
        resources::set_stockpile_for_test(&mut w,NationId::Germany,Commodity::Copper,10_000_000.0);
        w.resources.market.as_mut().unwrap().prices[Commodity::Copper.idx()]=1000.0;
        for (nation,cash) in [(id,100.0),(NationId::Germany,0.0)]{let n=w.nation_mut(nation);n.treasury_bn=Some(cash);n.debt_bn=Some(0.0);n.debt_gdp=0.0;}
        w
    }
    #[test]
    fn equipment_replenishment_review_is_pure_and_command_conserves_existing_ledgers() {
        let mut w=fixture();let id=NationId::France;let before=crate::save(&w);
        let q=replenishment_quote(&w,id,30,0.001);assert!(q.valid,"{:?}",q.reason);assert_eq!(crate::save(&w),before);
        assert!(q.receipt.spent_bn<=0.001);assert!(q.requested[Commodity::Copper.idx()]>0.0);
        let expected=100.0-q.receipt.spent_bn;
        replenish(&mut w,id,30,0.001).unwrap();assert_eq!(w.nation(id).treasury_bn,Some(expected));assert_eq!(w.nation(id).debt_bn,Some(0.0));
        let again=replenishment_quote(&w,id,30,0.001);
        assert!(again.requested[Commodity::Copper.idx()] <= q.requested[Commodity::Copper.idx()]-q.receipt.purchased[Commodity::Copper.idx()]+1e-6,"paid cargo must reduce the next purchase request: {:?} {:?}",q.requested,again.requested);
        assert_eq!(crate::save(&crate::load(&crate::save(&w)).unwrap()),crate::save(&w));
    }
    #[test]
    fn paused_work_and_invalid_or_stale_supply_requests_change_nothing() {
        let mut w=fixture();let id=NationId::France;
        for (days,cap) in [(0,1.0),(31,1.0),(30,0.0),(30,-1.0),(30,f64::NAN),(30,f64::INFINITY)]{
            let before=crate::save(&w);assert!(replenish(&mut w,id,days,cap).is_err());assert_eq!(crate::save(&w),before);
        }
        w.nation_mut(id).equipment.as_mut().unwrap().projects[0].paused=true;
        let before=crate::save(&w);let q=replenishment_quote(&w,id,30,1.0);assert!(!q.valid);assert!(q.requested.iter().all(|v|*v==0.0));
        assert!(replenish(&mut w,id,30,1.0).is_err());assert_eq!(crate::save(&w),before);
    }
    #[test]
    fn held_or_late_paid_cargo_is_not_purchased_again() {
        let mut w=fixture();let id=NationId::France;let q=replenishment_quote(&w,id,30,0.001);assert!(q.valid,"{:?}",q.reason);
        replenish(&mut w,id,30,0.001).unwrap();
        let cargo=w.logistics.cargo.iter_mut().find(|c|c.buyer==id&&c.commodity==Commodity::Copper).unwrap();
        cargo.quantity=q.requested[Commodity::Copper.idx()];cargo.due_day=cargo.due_day.map(|d|d+400);cargo.hold_reason=Some("Test route hold".into());
        let before=crate::save(&w);let reviewed=replenishment_quote(&w,id,30,0.001);assert!(!reviewed.valid);assert_eq!(reviewed.requested[Commodity::Copper.idx()],0.0);
        assert!(replenish(&mut w,id,30,0.001).is_err());assert_eq!(crate::save(&w),before);
    }
}
