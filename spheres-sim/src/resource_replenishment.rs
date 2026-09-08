// Reviewed, finite spot purchases use the existing raw stock, fiscal and
// freight owners. This file is included in resources.rs; it is not a ledger.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RawPurchaseFill {
    pub seller: NationId,
    pub commodity: Commodity,
    pub quantity: f64,
    pub cost_bn: f64,
    pub arrival_days: u32,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RawPurchaseReceipt {
    pub requested: [f64; 12],
    pub purchased: [f64; 12],
    pub spent_bn: f64,
    pub cash_available_bn: f64,
    pub fills: Vec<RawPurchaseFill>,
    pub warnings: Vec<String>,
}

/// The transport clock is the sole owner of arrivals. Both a reviewed spot
/// dispatch and ordinary production posting may open it; the same-day guard
/// ensures the second caller neither resets freight capacity nor credits twice.
fn begin_raw_freight(w: &mut WorldState, market: &mut MarketState) {
    if crate::logistics::enabled(w) {
        for cargo in crate::logistics::begin_month(w) {
            change_market_stock(market, cargo.buyer, cargo.commodity, cargo.quantity);
        }
    }
}

pub fn raw_purchase_cash(w: &WorldState, buyer: NationId) -> f64 {
    w.nation_opt(buyer).map_or(0.0, |n| {
        n.treasury_bn.unwrap_or(0.0).max(0.0) + market_cash_bn(w, buyer)
    })
}

/// Dispatch a bounded finite bundle, with no recurring purchase instruction.
/// Opening stocks are the only goods offered: domestic output is never created
/// by a command. Seller operating reserves, manual Materials inputs and the
/// next funded work bundle remain protected exactly as in automatic clearing.
/// The caller uses a cloned world for atomic review/commit of the whole bundle.
pub(crate) fn purchase_raw_bundle(
    w: &mut WorldState, buyer: NationId, requested: [f64; 12], spending_cap_bn: f64,
) -> Result<RawPurchaseReceipt, String> {
    if !(w.rules.resource_gates && w.rules.resource_market && crate::clock::is_daily(w)) {
        return Err("Reviewed material purchases require the daily physical resource market.".into());
    }
    if !w.nation_opt(buyer).is_some_and(|n| n.alive && n.treasury_bn.is_some()) {
        return Err("Open the national fiscal accounts before purchasing materials.".into());
    }
    if !spending_cap_bn.is_finite() || spending_cap_bn <= 0.0
        || requested.iter().any(|q| !q.is_finite() || *q < 0.0)
        || requested[Commodity::Oil.idx()] > 0.0 {
        return Err("Choose a positive finite spending limit and physical material quantities.".into());
    }
    let cash_available_bn = raw_purchase_cash(w, buyer);
    let limit = spending_cap_bn.min(cash_available_bn);
    let mut out = RawPurchaseReceipt { requested, cash_available_bn, ..Default::default() };
    if limit < 1e-9 { return Err("No available treasury or retained trade cash can fund this purchase. This order does not borrow.".into()); }
    let mut market = w.resources.market.clone().unwrap_or_else(|| new_market_state(w));
    begin_raw_freight(w, &mut market);
    w.resources.market = Some(market.clone());
    // Limit work and presentation to twelve dispatches, in commodity then
    // NationId order. A later reviewed purchase can seek further suppliers.
    let mut finance: BTreeMap<NationId, f64> = BTreeMap::new();
    for c in ALL.into_iter().filter(|c| *c != Commodity::Oil && c.tracked()) {
        let i = c.idx();
        let price = if market.prices[i].is_finite() && market.prices[i] > 0.0 {
            market.prices[i]
        } else { reference_price_usd(w, c) };
        if requested[i] <= 1e-9 || price <= 0.0 { continue; }
        for seller in all_nations().iter().copied() {
            if out.fills.len() >= 12 || out.spent_bn >= limit { break; }
            if seller == buyer || !w.nation_opt(seller).is_some_and(|n|n.alive) || !open_to(w,buyer,seller) { continue; }
            let need = draw_inner(w, seller, false)[i];
            let next = if crate::programs::enrolled(w,seller) { need.max(tick_draw_inner(w,seller,false)[i]) } else { need };
            let manual = crate::materials::resource_reserve(w,seller)[i];
            let reserve = need * BUFFER_MONTHS + manual + next;
            let sellable = (market_stock(&market,seller,c) - reserve).max(0.0);
            let remaining = (requested[i] - out.purchased[i]).max(0.0);
            let cash_left = (limit - out.spent_bn).max(0.0);
            // Resource money posts at a whole-dollar quantum. Floor the
            // affordable budget before sizing goods; rounding its eventual
            // invoice must never turn a fractional last dollar into a loan.
            let payable = (cash_left * 1e9).floor() / 1e9;
            let mut proposed = (remaining.min(sellable).min(payable * 1e9 / price) * 1e9).floor() / 1e9;
            if round_market(proposed * price / 1e9) > cash_left {
                proposed = ((proposed * (1.0 - f64::EPSILON)) * 1e9).floor() / 1e9;
            }
            if proposed <= 1e-9 || round_market(proposed * price / 1e9) <= 0.0 { continue; }
            let opening_logistics = w.logistics.clone();
            let dispatch = crate::logistics::enabled(w).then(|| crate::logistics::dispatch(w,seller,buyer,c,proposed,ShipmentSource::Spot,None));
            let quantity = dispatch.as_ref().map_or(proposed, |d| d.quantity);
            let cost_bn = round_market(quantity * price / 1e9);
            if quantity <= 1e-9 || cost_bn <= 0.0 { w.logistics = opening_logistics; continue; }
            if cost_bn > cash_left { return Err("The exact shipment payment exceeds the remaining reviewed spending limit.".into()); }
            change_market_stock(&mut market,seller,c,-quantity);
            if dispatch.is_none() { change_market_stock(&mut market,buyer,c,quantity); }
            *finance.entry(buyer).or_default() += cost_bn;
            *finance.entry(seller).or_default() -= cost_bn;
            out.spent_bn += cost_bn;
            out.purchased[i] += quantity;
            let arrival_days = dispatch.as_ref().and_then(|d|d.route.as_ref()).map_or(0,|r|r.estimated_days.max(1));
            out.fills.push(RawPurchaseFill { seller, commodity:c, quantity, cost_bn, arrival_days });
            market.fills.push(SpotFill { buyer,seller,commodity:c,quantity,unit_price:price,cost_bn });
            market.shipment_audits.push(ShipmentAudit {
                source:ShipmentSource::Spot,contract:None,seller,buyer,commodity:c,requested:proposed,
                delivered:if dispatch.is_some(){0.0}else{quantity},unit_price:Some(price),cost_bn:Some(cost_bn),months_left:None,
                status:if dispatch.is_some(){ShipmentStatus::InTransit}else{ShipmentStatus::Delivered},cause:None,
                dispatched:dispatch.as_ref().map(|_|quantity),route:dispatch.and_then(|d|d.route),
            });
            market.cleared_volume[i] = round_market(market.cleared_volume[i] + quantity);
            w.resources.market = Some(market.clone());
            if out.purchased[i] + 1e-9 >= requested[i] { break; }
        }
    }
    if out.fills.is_empty() { return Err("No payable shipment is available within this limit. Review supplier surplus, open routes, shared freight capacity and cash.".into()); }
    for (id,cost) in finance { apply_market_net(w,&mut market,id,cost); }
    market.fills.sort_by_key(|f|(f.buyer,f.seller,f.commodity));
    sort_shipment_audits(&mut market.shipment_audits);
    w.resources.market = Some(market);
    if out.purchased.iter().zip(requested).any(|(bought,want)|*bought + 1e-9 < want) {
        out.warnings.push("This is a partial supply purchase. Cash, the spending limit, supplier reserves, routes or the twelve-shipment limit leave part of the requirement unfilled.".into());
    }
    Ok(out)
}

#[cfg(test)]
mod raw_replenishment_tests {
    use super::*;
    use crate::world::GameRules;
    fn fixture()->WorldState {
        let mut w=crate::init::world_1990(GameRules {daily_simulation:true,resource_gates:true,resource_market:true,logistics_routes:true,physical_logistics:true,..Default::default()});
        warm(&mut w);
        let mut market=new_market_state(&w);market.stocks.clear();market.prices=[1000.0;12];w.resources.market=Some(market);
        for (id,cash) in [(NationId::France,1.0),(NationId::Germany,0.0)] {let n=w.nation_mut(id);n.treasury_bn=Some(cash);n.debt_bn=Some(0.0);n.debt_gdp=0.0;}
        set_stockpile_for_test(&mut w,NationId::Germany,Commodity::Copper,10_000_000.0);
        w
    }
    #[test]
    fn reviewed_raw_purchase_conserves_stock_cash_and_existing_freight_under_one_cap() {
        let mut w=fixture();let c=Commodity::Copper;
        let seller_before=stockpile(&w,NationId::Germany,c);let buyer_before=w.nation(NationId::France).treasury_bn.unwrap();
        let mut desired=[0.0;12];desired[c.idx()]=100.0;desired[Commodity::Iron.idx()]=100.0;
        set_stockpile_for_test(&mut w,NationId::Germany,Commodity::Iron,1_000_000_000.0);
        let receipt=purchase_raw_bundle(&mut w,NationId::France,desired,0.00015).unwrap();
        assert_eq!(receipt.purchased[c.idx()],100.0);
        assert!(receipt.purchased[Commodity::Iron.idx()]>0.0&&receipt.purchased[Commodity::Iron.idx()]<100.0);
        assert!(receipt.spent_bn<=0.00015,"the second commodity cannot obtain another spending cap");assert_eq!(receipt.fills.len(),2);
        assert!((seller_before-stockpile(&w,NationId::Germany,c)-receipt.purchased[c.idx()]).abs()<1e-8);
        assert_eq!(stockpile(&w,NationId::France,c),0.0,"purchased freight must not teleport");
        assert_eq!(crate::logistics::pending(&w,NationId::France,c),receipt.purchased[c.idx()]);
        assert!((buyer_before-w.nation(NationId::France).treasury_bn.unwrap()-receipt.spent_bn).abs()<1e-12);
        assert!((w.nation(NationId::Germany).treasury_bn.unwrap()-receipt.spent_bn).abs()<1e-12);
        assert_eq!(w.nation(NationId::France).debt_bn,Some(0.0));
        let saved=crate::save(&w);assert_eq!(crate::save(&crate::load(&saved).unwrap()),saved);
    }
    #[test]
    fn reviewed_raw_purchase_never_borrows_or_takes_a_sellers_operating_reserve() {
        let mut w=fixture();w.nation_mut(NationId::France).treasury_bn=Some(0.0000000016);
        let mut desired=[0.0;12];desired[Commodity::Copper.idx()]=100.0;
        let q=purchase_raw_bundle(&mut w,NationId::France,desired,1.0).unwrap();
        assert_eq!(q.spent_bn,0.000000001);assert_eq!(w.nation(NationId::France).debt_bn,Some(0.0));
        assert!(w.nation(NationId::France).treasury_bn.unwrap()>=0.0);
        let mut w=fixture();let operating=draw_inner(&w,NationId::USA,false)[Commodity::Copper.idx()];assert!(operating>0.0);
        set_stockpile_for_test(&mut w,NationId::Germany,Commodity::Copper,0.0);
        set_stockpile_for_test(&mut w,NationId::USA,Commodity::Copper,operating);
        assert!(purchase_raw_bundle(&mut w.clone(),NationId::France,desired,1.0).is_err());
        let mut w=fixture();
        w.sanctions.push((NationId::France,NationId::Germany));set_stockpile_for_test(&mut w,NationId::Germany,Commodity::Copper,10_000_000.0);
        assert!(purchase_raw_bundle(&mut w.clone(),NationId::France,desired,1.0).is_err());
    }
    #[test]
    fn reviewed_dispatch_opens_freight_without_losing_or_double_crediting_arrivals() {
        let mut w=fixture();let c=Commodity::Copper;let mut desired=[0.0;12];desired[c.idx()]=100.0;
        let first=purchase_raw_bundle(&mut w,NationId::France,desired,1.0).unwrap();
        let due=w.logistics.cargo[0].due_day.unwrap();let (year,month,day)=crate::clock::date_from_day(due);w.year=year;w.month=month;w.day=day;
        let before_produced=w.resources.market.as_ref().unwrap().last_produced_day;
        let second=purchase_raw_bundle(&mut w,NationId::France,desired,1.0).unwrap();
        assert_eq!(stockpile(&w,NationId::France,c),first.purchased[c.idx()]);
        assert_eq!(crate::logistics::pending(&w,NationId::France,c),second.purchased[c.idx()]);
        assert_eq!(w.resources.market.as_ref().unwrap().last_produced_day,before_produced,"buying cannot advance national production");
        let mut market=w.resources.market.take().unwrap();begin_raw_freight(&mut w,&mut market);w.resources.market=Some(market);
        assert_eq!(stockpile(&w,NationId::France,c),first.purchased[c.idx()]);
    }
    #[test]
    fn ordinary_settlement_after_reviewed_purchase_never_replays_its_payment_or_freight() {
        let mut w=fixture();let mut desired=[0.0;12];desired[Commodity::Copper.idx()]=100.0;
        purchase_raw_bundle(&mut w,NationId::France,desired,1.0).unwrap();
        let paid_cargo=w.logistics.cargo.clone();let used=w.logistics.usage_tonnes.clone();
        let mut without_receipts=w.clone();let market=without_receipts.resources.market.as_mut().unwrap();market.fills.clear();market.shipment_audits.clear();
        // Receipts describe a completed payment; none can become a second
        // command merely because the ordinary resource owners run afterward.
        tick(&mut w);clear_spot_market(&mut w);
        tick(&mut without_receipts);clear_spot_market(&mut without_receipts);
        assert_eq!(crate::save(&w),crate::save(&without_receipts));
        for cargo in paid_cargo {
            let retained:Vec<_>=w.logistics.cargo.iter().filter(|c|c.id==cargo.id).collect();
            assert_eq!(retained.len(),1);assert_eq!(retained[0].quantity,cargo.quantity);
        }
        for (segment,quantity) in used {assert!(w.logistics.usage_tonnes[&segment]>=quantity,"ordinary settlement reset a paid freight reservation");}
        let settled=crate::save(&w);tick(&mut w);clear_spot_market(&mut w);assert_eq!(crate::save(&w),settled);
    }
}
