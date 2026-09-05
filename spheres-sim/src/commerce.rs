//! Manufactured goods are distinct from the twelve mapped raw commodities.
//!
//! A standing export policy is the seller's explicit consent. Negotiation buys
//! one finite lot, not invented consumer demand. Acceptance moves actual goods
//! and cash into escrow; dispatch reserves the *same* physical edge ledger as
//! raw freight and pays the seller once. Arrival alone releases usable goods.
//! Import cash is a separately reported acquisition ledger, not ministry labor
//! or installation spending. Purchases require existing cash: no implicit debt,
//! program-authority duplication, GDP-on-sale, or second charge on consumption.
//!
//! Price and mass are named game benchmarks, not historical pack observations.
//! Delivery_days is the agreed loading window; dispatched cargo retains its
//! actual route arrival date even after cancellation or expiry. Closure/war or
//! a full warehouse holds buyer-owned freight; it does not destroy/refund it.
//! Extinct endpoints do not silently transfer national inventories to a
//! successor: undispatched lots refund to the original parties; paid freight
//! remains held for its named buyer. General successor inventory inheritance is
//! a separate game-wide policy, not an exception invented by this market.
//! Counteroffers last seven days and honor their quoted price when the ask
//! changes; explicit export disable/reserve changes and lost stock are checked
//! again before acceptance. Cancelling undispatched work costs two reputation.

use crate::world::{NationId, WorldState};
use crate::{clock, economy, industry, logistics, production::ProjectKind};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const NEGOTIATION_PC: f64 = 2.0;
const MAX_LOT: f64 = 1_000_000.0;
const MIN_LOT: f64 = 1e-9;
const MAX_ACTIVE: usize = 64;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum Good {
    Intermediates,
    CapitalGoods,
}
impl Good {
    pub fn key(self) -> &'static str {
        match self {
            Self::Intermediates => "intermediates",
            Self::CapitalGoods => "capital_goods",
        }
    }
    pub fn parse(s: &str) -> Option<Self> {
        match s
            .trim()
            .to_ascii_lowercase()
            .replace([' ', '-'], "_")
            .as_str()
        {
            "intermediates" | "intermediate" | "intermediate_packs" => Some(Self::Intermediates),
            "capital_goods" | "capital" => Some(Self::CapitalGoods),
            _ => None,
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Self::Intermediates => "Intermediate packs",
            Self::CapitalGoods => "Capital goods",
        }
    }
    /// MODELED shipping tonnes per industrial pack, not a geological unit.
    pub fn tonnes(self) -> f64 {
        match self {
            Self::Intermediates => 1.0,
            Self::CapitalGoods => 2.0,
        }
    }
}
pub const GOODS: [Good; 2] = [Good::Intermediates, Good::CapitalGoods];
pub fn reference_price_bn(good: Good) -> f64 {
    match good {
        Good::Intermediates => crate::gdp_projects::INTERMEDIATE_PACK_BN,
        Good::CapitalGoods => crate::gdp_projects::CAPITAL_PACK_BN,
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SalePolicy {
    pub nation: NationId,
    pub good: Good,
    pub reserve: f64,
    pub ask_multiplier: f64,
    pub enabled: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Offer {
    pub id: u64,
    pub buyer: NationId,
    pub seller: NationId,
    pub good: Good,
    pub quantity: f64,
    pub unit_price_bn: f64,
    pub delivery_days: u32,
    pub created_day: i32,
    pub expires_day: i32,
    pub reason: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Contract {
    pub id: u64,
    pub buyer: NationId,
    pub seller: NationId,
    pub good: Good,
    pub quantity: f64,
    pub unit_price_bn: f64,
    pub remaining_quantity: f64,
    pub escrow_bn: f64,
    pub delivered_quantity: f64,
    pub cancelled_quantity: f64,
    pub paid_bn: f64,
    pub accepted_day: i32,
    pub expires_day: i32,
    pub status: String,
    pub reason: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Cargo {
    pub id: u64,
    pub contract: u64,
    pub buyer: NationId,
    pub seller: NationId,
    pub good: Good,
    pub quantity: f64,
    pub route: logistics::RoutePlan,
    pub dispatched_day: i32,
    pub due_day: i32,
    pub hold_reason: Option<String>,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Account {
    pub imports_reserved_bn: f64,
    pub imports_refunded_bn: f64,
    pub exports_received_bn: f64,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DeliveredSource {
    pub day: i32,
    pub buyer: NationId,
    pub seller: NationId,
    pub reference_value_bn: f64,
}
/// Dated physical deliveries for capacity planning, separate from monetary
/// dependency. An offer, dispatch or escrow is not demonstrated final demand.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct GoodsDelivery {
    pub contract: u64,
    pub day: i32,
    pub buyer: NationId,
    pub seller: NationId,
    pub good: Good,
    pub quantity: f64,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Commerce {
    pub policies: Vec<SalePolicy>,
    pub offers: Vec<Offer>,
    pub contracts: Vec<Contract>,
    pub cargo: Vec<Cargo>,
    pub accounts: BTreeMap<NationId, Account>,
    pub next_id: u64,
    pub last_day: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sourcing: Vec<DeliveredSource>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub goods_deliveries: Vec<GoodsDelivery>,
}
#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct Quote {
    pub buyer: NationId,
    pub seller: NationId,
    pub good: Good,
    pub quantity: f64,
    pub available_quantity: f64,
    pub unit_price_bn: f64,
    pub total_price_bn: f64,
    pub estimated_days: u32,
    pub accepted: bool,
    pub reason: String,
}
#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct ProposalResult {
    pub id: u64,
    pub kind: String,
}
#[derive(Clone, Debug, Serialize)]
pub struct GoodsView {
    pub good: Good,
    pub name: String,
    pub stock: f64,
    pub capacity: f64,
    pub demand: f64,
    pub shortage: f64,
    pub incoming: f64,
    pub exportable: f64,
    pub reference_price_bn: f64,
    pub sale: Option<SalePolicy>,
}
#[derive(Clone, Debug, Serialize)]
pub struct Snapshot {
    pub enabled: bool,
    pub nation: NationId,
    pub goods: Vec<GoodsView>,
    pub offers: Vec<Offer>,
    pub contracts: Vec<Contract>,
    pub cargo: Vec<Cargo>,
    pub account: Account,
    pub escrow_bn: f64,
    pub note: String,
}

pub fn active(w: &WorldState) -> bool {
    w.rules.economic_competition
        && clock::is_daily(w)
        && w.rules.production_system
        && logistics::enabled(w)
}
pub fn enabled(w: &WorldState) -> bool {
    active(w)
}
fn amount(goods: &industry::Goods, good: Good) -> f64 {
    match good {
        Good::Intermediates => goods.intermediates,
        Good::CapitalGoods => goods.capital_goods,
    }
}
fn change_stock(w: &mut WorldState, nation: NationId, good: Good, delta: f64) {
    let goods = w.production.industry.goods.entry(nation).or_default();
    let value = match good {
        Good::Intermediates => &mut goods.intermediates,
        Good::CapitalGoods => &mut goods.capital_goods,
    };
    *value = (*value + delta).max(0.0);
}
pub fn stock(w: &WorldState, nation: NationId, good: Good) -> f64 {
    w.production
        .industry
        .goods
        .get(&nation)
        .map_or(0.0, |g| amount(g, good))
        .max(0.0)
}
pub fn sale(w: &WorldState, nation: NationId, good: Good) -> Option<&SalePolicy> {
    w.commerce
        .as_ref()?
        .policies
        .iter()
        .find(|p| p.nation == nation && p.good == good)
}
pub fn available_to_sell(w: &WorldState, nation: NationId, good: Good) -> f64 {
    sale(w, nation, good)
        .filter(|p| p.enabled)
        .map_or(0.0, |p| (stock(w, nation, good) - p.reserve).max(0.0))
}
pub fn pending(w: &WorldState, nation: NationId, good: Good) -> f64 {
    w.commerce.as_ref().map_or(0.0, |c| {
        c.contracts
            .iter()
            .filter(|c| c.buyer == nation && c.good == good)
            .map(|c| c.remaining_quantity)
            .sum::<f64>()
            + c.cargo
                .iter()
                .filter(|c| c.buyer == nation && c.good == good)
                .map(|c| c.quantity)
                .sum::<f64>()
    })
}
/// Current operating-goods use at today's installed rate. This is a planning
/// read, not an inventory debit; construction remains a finite requirement in
/// `demand` rather than being multiplied as if it recurred forever.
pub fn recurring_demand_daily(w: &WorldState, nation: NationId, good: Good) -> f64 {
    let factories = if good == Good::Intermediates {
        w.districts
            .iter()
            .filter(|(_, owner)| **owner == nation)
            .map(|(district, _)| {
                industry::plant_rate(w, district, ProjectKind::MachineryWorks)
            })
            .sum()
    } else {
        0.0
    };
    factories + amount(&industry::research_goods_demand(w, nation), good)
}
/// Only actual queued construction and installed consumers create demand.
/// Thirty days is the existing operating horizon; supply policy can extend the
/// recurring arm without multiplying a project's one-off remaining recipe.
pub fn demand(w: &WorldState, nation: NationId, good: Good) -> f64 {
    let projects = w
        .production
        .projects
        .iter()
        .filter(|p| p.nation == nation && w.districts.get(&p.district) == Some(&nation))
        .map(|p| {
            let used = w
                .production
                .industry
                .projects
                .get(&p.id)
                .map_or(0.0, |f| amount(&f.goods_used, good));
            (amount(&industry::goods_recipe(p.kind), good) - used).max(0.0)
        })
        .sum::<f64>();
    projects + recurring_demand_daily(w, nation, good) * 30.0
}
pub fn shortage(w: &WorldState, nation: NationId, good: Good) -> f64 {
    (demand(w, nation, good) - stock(w, nation, good) - pending(w, nation, good)).max(0.0)
}
/// Delivered reference-value imports over the trailing 365 days, divided by
/// actual national annual GDP, capped at 12%. No signature, unpaid escrow,
/// over-priced token pack or annualized one-day purchase can create leverage.
pub fn dependency(w: &WorldState, buyer: NationId, seller: NationId) -> f64 {
    if !active(w) {
        return 0.0;
    }
    let Some(n) = w
        .nation_opt(buyer)
        .filter(|n| n.alive && n.gdp.is_finite() && n.gdp > 0.0)
    else {
        return 0.0;
    };
    let day = clock::absolute_day(w);
    let value = w.commerce.as_ref().map_or(0.0, |c| {
        c.sourcing
            .iter()
            .filter(|r| r.buyer == buyer && r.seller == seller && r.day <= day && r.day > day - 365)
            .map(|r| r.reference_value_bn)
            .sum::<f64>()
    });
    (value / n.gdp).clamp(0.0, 0.12)
}
fn government(w: &WorldState, nation: NationId) -> Result<(), String> {
    let n = w
        .nation_opt(nation)
        .filter(|n| n.alive)
        .ok_or("That government is not on the board.")?;
    if !n.on_the_books()
        || !n.treasury_bn.is_some_and(|v| v.is_finite() && v >= 0.0)
        || !n.debt_bn.is_some_and(|v| v.is_finite() && v >= 0.0)
    {
        return Err("Open the annual treasury books before trading manufactured goods.".into());
    }
    Ok(())
}
pub fn sale_refusal(
    w: &WorldState,
    nation: NationId,
    _good: Good,
    reserve: f64,
    ask_multiplier: f64,
    _enabled: bool,
) -> Option<String> {
    if !active(w) {
        return Some(
            "Enable the daily economic competition and physical logistics systems first.".into(),
        );
    }
    if let Err(e) = government(w, nation) {
        return Some(e);
    }
    if !reserve.is_finite() || !(0.0..=MAX_LOT).contains(&reserve) {
        return Some(
            "Export reserve must be finite and between zero and one million packs.".into(),
        );
    }
    if !ask_multiplier.is_finite() || !(0.25..=4.0).contains(&ask_multiplier) {
        return Some(
            "Ask price must be between 25% and 400% of the modeled reference price.".into(),
        );
    }
    None
}
pub fn set_sale(
    w: &mut WorldState,
    nation: NationId,
    good: Good,
    reserve: f64,
    ask_multiplier: f64,
    enabled: bool,
) -> Result<(), String> {
    if let Some(e) = sale_refusal(w, nation, good, reserve, ask_multiplier, enabled) {
        return Err(e);
    }
    let c = w.commerce.get_or_insert_with(Commerce::default);
    c.policies.retain(|p| p.nation != nation || p.good != good);
    c.policies.push(SalePolicy {
        nation,
        good,
        reserve,
        ask_multiplier,
        enabled,
    });
    c.policies.sort_by_key(|p| (p.nation, p.good));
    Ok(())
}
fn check(
    w: &WorldState,
    buyer: NationId,
    seller: NationId,
    good: Good,
    quantity: f64,
    unit_price_bn: f64,
    delivery_days: u32,
) -> Result<logistics::RoutePlan, String> {
    if !active(w) {
        return Err("Enable daily economic competition and physical logistics first.".into());
    }
    if buyer == seller {
        return Err("An import needs a different seller.".into());
    }
    government(w, buyer)?;
    government(w, seller)?;
    if !quantity.is_finite() || !(MIN_LOT..=MAX_LOT).contains(&quantity) {
        return Err("A lot must contain between 0.000000001 and one million finite packs.".into());
    }
    if !unit_price_bn.is_finite()
        || unit_price_bn <= 0.0
        || unit_price_bn > reference_price_bn(good) * 100.0
        || !(quantity * unit_price_bn).is_finite()
        || quantity * unit_price_bn <= 0.0
    {
        return Err(
            "Price must be positive, finite and at most 100 times the reference price.".into(),
        );
    }
    if !(1..=365).contains(&delivery_days) {
        return Err("The dispatch window must be between one and 365 days.".into());
    }
    if !sale(w, seller, good).is_some_and(|p| p.enabled) {
        return Err("The seller has not consented to export this good.".into());
    }
    if available_to_sell(w, seller, good) < MIN_LOT {
        return Err("The seller has no stock above its export reserve.".into());
    }
    if w.commerce.as_ref().is_some_and(|c| c.next_id == u64::MAX) {
        return Err("Goods ledger identifier capacity reached.".into());
    }
    if w.commerce.as_ref().is_some_and(|ledger| {
        ledger
            .contracts
            .iter()
            .filter(|c| {
                c.buyer == buyer
                    && (c.remaining_quantity > 0.0
                        || ledger.cargo.iter().any(|s| s.contract == c.id))
            })
            .count()
            >= MAX_ACTIVE
    }) {
        return Err("Finish or cancel an existing goods order before opening another.".into());
    }
    logistics::plan(w, seller, buyer)
}
pub fn quote(
    w: &WorldState,
    buyer: NationId,
    seller: NationId,
    good: Good,
    quantity: f64,
    unit_price_bn: f64,
    delivery_days: u32,
) -> Quote {
    let checked = check(
        w,
        buyer,
        seller,
        good,
        quantity,
        unit_price_bn,
        delivery_days,
    );
    let available = available_to_sell(w, seller, good);
    let ask = sale(w, seller, good).map_or(reference_price_bn(good), |p| {
        reference_price_bn(good) * p.ask_multiplier
    });
    let proposed_quantity = if quantity.is_finite() {
        quantity.min(available).max(0.0)
    } else {
        0.0
    };
    let proposed_price = if unit_price_bn.is_finite() {
        unit_price_bn.max(ask)
    } else {
        ask
    };
    let affordable = w
        .nation_opt(buyer)
        .and_then(|n| n.treasury_bn)
        .unwrap_or(0.0)
        >= proposed_quantity * proposed_price;
    let accepted = checked.is_ok() && quantity <= available && unit_price_bn >= ask && affordable;
    let reason=match checked.as_ref() { Err(e)=>e.clone(), Ok(_) if !affordable=>"Insufficient treasury cash for full-lot escrow; imports never borrow automatically.".into(),
        Ok(_) if !accepted=>"Counteroffer: the seller quotes its available surplus at its posted ask price.".into(),
        Ok(_)=>"Accepted by the seller's standing export policy; cash and goods reserve on confirmation.".into() };
    Quote {
        buyer,
        seller,
        good,
        quantity: proposed_quantity,
        available_quantity: available,
        unit_price_bn: proposed_price,
        total_price_bn: proposed_quantity * proposed_price,
        estimated_days: checked.as_ref().map_or(0, |r| r.estimated_days),
        accepted,
        reason,
    }
}
pub fn proposal_refusal(
    w: &WorldState,
    buyer: NationId,
    seller: NationId,
    good: Good,
    quantity: f64,
    unit_price_bn: f64,
    delivery_days: u32,
) -> Option<String> {
    if let Err(e) = check(
        w,
        buyer,
        seller,
        good,
        quantity,
        unit_price_bn,
        delivery_days,
    ) {
        return Some(e);
    }
    let q = quote(
        w,
        buyer,
        seller,
        good,
        quantity,
        unit_price_bn,
        delivery_days,
    );
    if w.nation(buyer).treasury_bn.unwrap() < q.total_price_bn {
        return Some(q.reason);
    }
    if !q.accepted
        && w.commerce.as_ref().is_some_and(|c| {
            c.offers.iter().filter(|o| o.buyer == buyer).count() >= MAX_ACTIVE
                && !c
                    .offers
                    .iter()
                    .any(|o| o.buyer == buyer && o.seller == seller && o.good == good)
        })
    {
        return Some("Resolve an existing counteroffer before opening another.".into());
    }
    None
}
fn accept_lot(
    w: &mut WorldState,
    buyer: NationId,
    seller: NationId,
    good: Good,
    quantity: f64,
    unit_price_bn: f64,
    delivery_days: u32,
) -> Result<u64, String> {
    check(
        w,
        buyer,
        seller,
        good,
        quantity,
        unit_price_bn,
        delivery_days,
    )?;
    if quantity > available_to_sell(w, seller, good) {
        return Err("The seller's available surplus has changed; request a new quote.".into());
    }
    let total = quantity * unit_price_bn;
    if total > w.nation(buyer).treasury_bn.unwrap() {
        return Err("Insufficient treasury cash for full-lot escrow; no debt was issued.".into());
    }
    let day = clock::absolute_day(w);
    // Every fallible preflight precedes all mutations.
    let c = w.commerce.get_or_insert_with(Commerce::default);
    let id = c.next_id;
    c.next_id = c
        .next_id
        .checked_add(1)
        .ok_or("Goods contract identifier capacity reached.")?;
    c.contracts.push(Contract {
        id,
        buyer,
        seller,
        good,
        quantity,
        unit_price_bn,
        remaining_quantity: quantity,
        escrow_bn: total,
        delivered_quantity: 0.0,
        cancelled_quantity: 0.0,
        paid_bn: 0.0,
        accepted_day: day,
        expires_day: day + delivery_days as i32,
        status: "awaiting_dispatch".into(),
        reason: None,
    });
    c.accounts.entry(buyer).or_default().imports_reserved_bn += total;
    change_stock(w, seller, good, -quantity);
    economy::charge(w, buyer, total, 0.0);
    Ok(id)
}
pub fn propose(
    w: &mut WorldState,
    buyer: NationId,
    seller: NationId,
    good: Good,
    quantity: f64,
    unit_price_bn: f64,
    delivery_days: u32,
) -> Result<ProposalResult, String> {
    if let Some(e) = proposal_refusal(
        w,
        buyer,
        seller,
        good,
        quantity,
        unit_price_bn,
        delivery_days,
    ) {
        return Err(e);
    }
    let q = quote(
        w,
        buyer,
        seller,
        good,
        quantity,
        unit_price_bn,
        delivery_days,
    );
    if q.accepted {
        return Ok(ProposalResult {
            id: accept_lot(
                w,
                buyer,
                seller,
                good,
                quantity,
                unit_price_bn,
                delivery_days,
            )?,
            kind: "contract".into(),
        });
    }
    let day = clock::absolute_day(w);
    let c = w.commerce.get_or_insert_with(Commerce::default);
    if c.offers.iter().filter(|o| o.buyer == buyer).count() >= MAX_ACTIVE
        && !c
            .offers
            .iter()
            .any(|o| o.buyer == buyer && o.seller == seller && o.good == good)
    {
        return Err("Resolve an existing counteroffer before opening another.".into());
    }
    // One current counter per buyer/seller/good prevents unbounded spam.
    c.offers
        .retain(|o| !(o.buyer == buyer && o.seller == seller && o.good == good));
    let id = c.next_id;
    c.next_id = c
        .next_id
        .checked_add(1)
        .ok_or("Goods offer identifier capacity reached.")?;
    c.offers.push(Offer {
        id,
        buyer,
        seller,
        good,
        quantity: q.quantity,
        unit_price_bn: q.unit_price_bn,
        delivery_days,
        created_day: day,
        expires_day: day + 7,
        reason: q.reason,
    });
    Ok(ProposalResult {
        id,
        kind: "counter_offer".into(),
    })
}
pub fn offer_refusal(w: &WorldState, nation: NationId, offer: u64) -> Option<String> {
    let Some(o) = w
        .commerce
        .as_ref()
        .and_then(|c| c.offers.iter().find(|o| o.id == offer))
    else {
        return Some("That counteroffer is no longer open.".into());
    };
    if o.buyer != nation {
        return Some("Only the named buyer can accept this counteroffer.".into());
    }
    if clock::absolute_day(w) >= o.expires_day {
        return Some("That counteroffer has expired.".into());
    }
    if let Err(e) = check(
        w,
        o.buyer,
        o.seller,
        o.good,
        o.quantity,
        o.unit_price_bn,
        o.delivery_days,
    ) {
        return Some(e);
    }
    if o.quantity > available_to_sell(w, o.seller, o.good) {
        return Some("The seller's surplus has changed; request a new quote.".into());
    }
    if o.quantity * o.unit_price_bn > w.nation(nation).treasury_bn.unwrap() {
        return Some("Not enough cash to reserve this entire lot.".into());
    }
    None
}
pub fn accept_offer(w: &mut WorldState, nation: NationId, offer: u64) -> Result<u64, String> {
    if let Some(e) = offer_refusal(w, nation, offer) {
        return Err(e);
    }
    let o = w
        .commerce
        .as_ref()
        .unwrap()
        .offers
        .iter()
        .find(|o| o.id == offer)
        .unwrap()
        .clone();
    let id = accept_lot(
        w,
        o.buyer,
        o.seller,
        o.good,
        o.quantity,
        o.unit_price_bn,
        o.delivery_days,
    )?;
    w.commerce
        .as_mut()
        .unwrap()
        .offers
        .retain(|o| o.id != offer);
    Ok(id)
}
pub fn cancel_refusal(w: &WorldState, nation: NationId, contract: u64) -> Option<String> {
    let Some(c) = w
        .commerce
        .as_ref()
        .and_then(|c| c.contracts.iter().find(|c| c.id == contract))
    else {
        return Some("That goods contract does not exist.".into());
    };
    if nation != c.buyer && nation != c.seller {
        return Some("Only a party to this contract may cancel it.".into());
    }
    if c.remaining_quantity <= 0.0 {
        return Some("Nothing remains to cancel; dispatched cargo belongs to the buyer.".into());
    }
    None
}
fn refund(w: &mut WorldState, index: usize, status: &str, reason: &str) {
    let c = &mut w.commerce.as_mut().unwrap().contracts[index];
    let (buyer, seller, good, qty, cash) =
        (c.buyer, c.seller, c.good, c.remaining_quantity, c.escrow_bn);
    c.remaining_quantity = 0.0;
    c.escrow_bn = 0.0;
    c.cancelled_quantity += qty;
    c.status = status.into();
    c.reason = Some(reason.into());
    w.commerce
        .as_mut()
        .unwrap()
        .accounts
        .entry(buyer)
        .or_default()
        .imports_refunded_bn += cash;
    // Returns may temporarily exceed warehouse capacity: lossless, and future
    // production/import unloading pauses until the excess is consumed.
    change_stock(w, seller, good, qty);
    economy::charge(w, buyer, -cash, 0.0);
}
pub fn cancel(w: &mut WorldState, nation: NationId, contract: u64) -> Result<(), String> {
    if let Some(e) = cancel_refusal(w, nation, contract) {
        return Err(e);
    }
    let index = w
        .commerce
        .as_ref()
        .unwrap()
        .contracts
        .iter()
        .position(|c| c.id == contract)
        .unwrap();
    refund(w,index,"cancelled","Undispatched goods and escrow returned; already-dispatched cargo still belongs to the buyer.");
    w.shift_reputation(nation, -2.0);
    Ok(())
}
pub fn market_quotes(
    w: &WorldState,
    buyer: NationId,
    good: Good,
    quantity: f64,
    delivery_days: u32,
) -> Vec<Quote> {
    if !active(w) || !quantity.is_finite() || quantity < MIN_LOT || government(w, buyer).is_err() {
        return vec![];
    }
    let cash = w.nation(buyer).treasury_bn.unwrap();
    let mut quotes: Vec<_> = w
        .commerce
        .as_ref()
        .into_iter()
        .flat_map(|c| c.policies.iter())
        .filter(|p| p.nation != buyer && p.good == good && p.enabled)
        .filter_map(|p| {
            let price = reference_price_bn(good) * p.ask_multiplier;
            // Leave a representable downward margin only when cash is the binding
            // limit; exact cash equal to a requested lot remains fully spendable.
            let mut qty = quantity
                .min(MAX_LOT)
                .min(available_to_sell(w, p.nation, good))
                .min(cash / price);
            if qty * price > cash {
                qty *= 1.0 - f64::EPSILON;
            }
            if qty < MIN_LOT {
                return None;
            }
            let q = quote(w, buyer, p.nation, good, qty, price, delivery_days);
            q.accepted.then_some(q)
        })
        .collect();
    quotes.sort_by(|a, b| {
        a.unit_price_bn
            .total_cmp(&b.unit_price_bn)
            .then(a.seller.cmp(&b.seller))
    });
    quotes
}

pub fn tick_day(w: &mut WorldState) {
    if !active(w) || w.commerce.is_none() {
        return;
    }
    let today = clock::absolute_day(w);
    if w.commerce.as_ref().unwrap().last_day == Some(today) {
        return;
    }
    w.commerce.as_mut().unwrap().last_day = Some(today);
    w.commerce
        .as_mut()
        .unwrap()
        .offers
        .retain(|o| o.expires_day > today);
    let cargo = std::mem::take(&mut w.commerce.as_mut().unwrap().cargo);
    for mut shipment in cargo {
        if shipment.due_day > today {
            w.commerce.as_mut().unwrap().cargo.push(shipment);
            continue;
        }
        let open =
            logistics::freight_route_open(w, shipment.seller, shipment.buyer, &shipment.route);
        if let Err(reason) = open {
            shipment.hold_reason = Some(reason);
            w.commerce.as_mut().unwrap().cargo.push(shipment);
            continue;
        }
        let free = (industry::goods_capacity(w, shipment.buyer)
            - stock(w, shipment.buyer, shipment.good))
        .max(0.0);
        let delivered = shipment.quantity.min(free);
        if delivered > 0.0 {
            change_stock(w, shipment.buyer, shipment.good, delivered);
            if let Some(c) = w
                .commerce
                .as_mut()
                .unwrap()
                .contracts
                .iter_mut()
                .find(|c| c.id == shipment.contract)
            {
                c.delivered_quantity += delivered;
            }
            let source = &mut w.commerce.as_mut().unwrap().sourcing;
            let value = delivered * reference_price_bn(shipment.good);
            if let Some(row) = source.iter_mut().find(|r| {
                r.day == today && r.buyer == shipment.buyer && r.seller == shipment.seller
            }) {
                row.reference_value_bn += value;
            } else {
                source.push(DeliveredSource {
                    day: today,
                    buyer: shipment.buyer,
                    seller: shipment.seller,
                    reference_value_bn: value,
                });
            }
            shipment.quantity = (shipment.quantity - delivered).max(0.0);
            let deliveries = &mut w.commerce.as_mut().unwrap().goods_deliveries;
            if let Some(row) = deliveries.iter_mut().find(|r|
                r.contract == shipment.contract && r.day == today) {
                row.quantity += delivered;
            } else {
                deliveries.push(GoodsDelivery {contract:shipment.contract,day:today,
                    buyer:shipment.buyer,seller:shipment.seller,good:shipment.good,quantity:delivered});
            }
        }
        if shipment.quantity > 0.0 {
            shipment.hold_reason =
                Some("The buyer's warehouse is full; paid cargo waits without loss.".into());
            w.commerce.as_mut().unwrap().cargo.push(shipment);
        }
    }
    let count = w.commerce.as_ref().unwrap().contracts.len();
    for index in 0..count {
        let c = w.commerce.as_ref().unwrap().contracts[index].clone();
        if c.remaining_quantity <= 0.0 {
            continue;
        }
        if today >= c.expires_day
            || !w.nation_opt(c.buyer).is_some_and(|n| n.alive)
            || !w.nation_opt(c.seller).is_some_and(|n| n.alive)
        {
            refund(w,index,"expired","Dispatch window closed or government ceased operating; undispatched escrow and stock returned.");
            continue;
        }
        if w.commerce.as_ref().unwrap().next_id == u64::MAX {
            w.commerce.as_mut().unwrap().contracts[index].reason = Some(
                "Goods ledger identifier capacity reached; remaining escrow is refundable.".into(),
            );
            continue;
        }
        let dispatch =
            logistics::reserve_freight(w, c.seller, c.buyer, c.remaining_quantity, c.good.tonnes());
        let row = &mut w.commerce.as_mut().unwrap().contracts[index];
        row.reason = dispatch.reason;
        if dispatch.quantity <= 0.0 {
            row.status = "awaiting_dispatch".into();
            continue;
        }
        let quantity = dispatch.quantity;
        let payment = if quantity == c.remaining_quantity {
            c.escrow_bn
        } else {
            (quantity * c.unit_price_bn).min(c.escrow_bn)
        };
        row.remaining_quantity = (row.remaining_quantity - quantity).max(0.0);
        row.escrow_bn = (row.escrow_bn - payment).max(0.0);
        row.paid_bn += payment;
        row.status = if row.remaining_quantity > 0.0 {
            "partially_dispatched"
        } else {
            "in_transit"
        }
        .into();
        let route = dispatch.route.unwrap();
        let ledger = w.commerce.as_mut().unwrap();
        let id = ledger.next_id;
        ledger.next_id = ledger.next_id.saturating_add(1);
        ledger.cargo.push(Cargo {
            id,
            contract: c.id,
            buyer: c.buyer,
            seller: c.seller,
            good: c.good,
            quantity,
            due_day: today + route.estimated_days.max(1) as i32,
            dispatched_day: today,
            route,
            hold_reason: None,
        });
        ledger
            .accounts
            .entry(c.seller)
            .or_default()
            .exports_received_bn += payment;
        economy::charge(w, c.seller, -payment, 0.0);
    }
    let ledger = w.commerce.as_mut().unwrap();
    ledger.sourcing.retain(|r| r.day > today - 365);
    ledger.goods_deliveries.retain(|r| r.day > today - 365);
    for c in &mut ledger.contracts {
        if c.remaining_quantity == 0.0
            && c.cancelled_quantity == 0.0
            && !ledger.cargo.iter().any(|s| s.contract == c.id)
        {
            c.status = "delivered".into();
            c.reason = None;
        }
    }
    // Retain live commitments plus the latest 256 settled rows. Lifetime cash
    // totals remain in Account, so pruning display history never changes money.
    let mut settled = 0;
    ledger.contracts.reverse();
    ledger.contracts.retain(|c| {
        if c.remaining_quantity > 0.0 || ledger.cargo.iter().any(|s| s.contract == c.id) {
            true
        } else {
            settled += 1;
            settled <= 256
        }
    });
    ledger.contracts.reverse();
}
pub fn snapshot(w: &WorldState, nation: NationId) -> Snapshot {
    let c = w.commerce.as_ref();
    Snapshot {enabled:active(w),nation,goods:GOODS.into_iter().map(|good|GoodsView{good,name:good.name().into(),stock:stock(w,nation,good),capacity:industry::goods_capacity(w,nation),demand:demand(w,nation,good),shortage:shortage(w,nation,good),incoming:pending(w,nation,good),exportable:available_to_sell(w,nation,good),reference_price_bn:reference_price_bn(good),sale:sale(w,nation,good).cloned()}).collect(),
        offers:c.map_or(vec![],|c|c.offers.iter().filter(|o|o.buyer==nation || o.seller==nation).cloned().collect()),
        contracts:c.map_or(vec![],|c|c.contracts.iter().filter(|o|o.buyer==nation || o.seller==nation).cloned().collect()),
        cargo:c.map_or(vec![],|c|c.cargo.iter().filter(|o|o.buyer==nation || o.seller==nation).cloned().collect()),
        account:c.and_then(|c|c.accounts.get(&nation)).cloned().unwrap_or_default(),
        escrow_bn:c.map_or(0.0,|c|c.contracts.iter().filter(|c|c.buyer==nation).map(|c|c.escrow_bn).sum()),
        note:"Cash-only foreign-goods purchases, separate from ministry installation budgets. Escrow funds undispatched lots; sellers are paid at loading, usable stock arrives later. No GDP is created by resale. Dispatch windows are not arrival guarantees; blocked routes and full warehouses hold paid cargo.".into()}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        init::world_1990,
        resources::{Commodity, ShipmentSource},
        world::GameRules,
    };
    const BUYER: NationId = NationId::Belgium;
    const SELLER: NationId = NationId::Netherlands;
    fn world() -> WorldState {
        let mut w = world_1990(GameRules {
            daily_simulation: true,
            economic_competition: true,
            production_system: true,
            resource_gates: true,
            resource_market: true,
            logistics_routes: true,
            physical_logistics: true,
            ..GameRules::default()
        });
        for id in [BUYER, SELLER] {
            let n = w.nation_mut(id);
            n.treasury_bn = Some(1.0);
            n.debt_bn = Some(0.0);
            n.debt_gdp = 0.0;
        }
        change_stock(&mut w, SELLER, Good::Intermediates, 100.0);
        change_stock(&mut w, SELLER, Good::CapitalGoods, 100.0);
        set_sale(&mut w, SELLER, Good::Intermediates, 10.0, 1.0, true).unwrap();
        set_sale(&mut w, SELLER, Good::CapitalGoods, 10.0, 1.0, true).unwrap();
        w
    }
    fn buy(w: &mut WorldState, good: Good, qty: f64, days: u32) -> u64 {
        let p = propose(w, BUYER, SELLER, good, qty, reference_price_bn(good), days).unwrap();
        assert_eq!(p.kind, "contract");
        p.id
    }
    fn settle(w: &mut WorldState) {
        assert!(logistics::begin_month(w).is_empty());
        tick_day(w);
    }
    fn next(w: &mut WorldState) {
        clock::advance_date(w);
        settle(w);
    }
    fn near(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-12, "{a} != {b}");
    }
    fn goods_total(w: &WorldState, good: Good) -> f64 {
        stock(w, BUYER, good)
            + stock(w, SELLER, good)
            + w.commerce.as_ref().map_or(0.0, |c| {
                c.contracts
                    .iter()
                    .filter(|c| c.good == good)
                    .map(|c| c.remaining_quantity)
                    .sum::<f64>()
                    + c.cargo
                        .iter()
                        .filter(|c| c.good == good)
                        .map(|c| c.quantity)
                        .sum::<f64>()
            })
    }
    fn money_total(w: &WorldState) -> f64 {
        [BUYER, SELLER]
            .into_iter()
            .map(|id| w.nation(id).treasury_bn.unwrap() - w.nation(id).debt_bn.unwrap())
            .sum::<f64>()
            + w.commerce.as_ref().map_or(0.0, |c| {
                c.contracts.iter().map(|c| c.escrow_bn).sum::<f64>()
            })
    }
    #[test]
    fn manufactured_lot_conserves_cash_stock_and_gdp_through_exact_delivery_once() {
        for good in GOODS {
            let mut w = world();
            let total = goods_total(&w, good);
            let money = money_total(&w);
            let gdp = (w.nation(BUYER).gdp, w.nation(SELLER).gdp);
            let id = buy(&mut w, good, 10.0, 30);
            near(stock(&w, BUYER, good), 0.0);
            near(stock(&w, SELLER, good), 90.0);
            near(w.nation(SELLER).treasury_bn.unwrap(), 1.0);
            near(goods_total(&w, good), total);
            near(money_total(&w), money);
            settle(&mut w);
            near(stock(&w, BUYER, good), 0.0);
            assert!(w.commerce.as_ref().unwrap().goods_deliveries.is_empty(),
                "escrow and dispatch are not delivered demand");
            near(
                w.nation(SELLER).treasury_bn.unwrap(),
                1.0 + 10.0 * reference_price_bn(good),
            );
            let due = w.commerce.as_ref().unwrap().cargo[0].due_day;
            let once = crate::save(&w);
            tick_day(&mut w);
            assert_eq!(once, crate::save(&w));
            while clock::absolute_day(&w) < due {
                next(&mut w);
                if clock::absolute_day(&w) < due {
                    near(stock(&w, BUYER, good), 0.0);
                }
            }
            near(stock(&w, BUYER, good), 10.0);
            near(goods_total(&w, good), total);
            let deliveries=&w.commerce.as_ref().unwrap().goods_deliveries;
            assert_eq!(deliveries.len(),1);
            assert_eq!(deliveries[0].good,good);
            assert_eq!(deliveries[0].day,due);
            near(deliveries[0].quantity,10.0);
            near(crate::industry_planning::delivered_daily(&w,SELLER,good),10.0/90.0);
            near(money_total(&w), money);
            assert!(w.commerce.as_ref().unwrap().cargo.is_empty());
            assert_eq!(
                w.commerce
                    .as_ref()
                    .unwrap()
                    .contracts
                    .iter()
                    .find(|c| c.id == id)
                    .unwrap()
                    .status,
                "delivered"
            );
            next(&mut w);
            near(stock(&w, BUYER, good), 10.0);
            assert_eq!(w.commerce.as_ref().unwrap().goods_deliveries.len(),1);
            near(money_total(&w), money);
            assert_eq!(
                gdp,
                (w.nation(BUYER).gdp, w.nation(SELLER).gdp),
                "trade resells output; never new GDP"
            );
        }
    }
    #[test]
    fn cash_only_handles_tiny_exact_cash_and_insufficient_funds_without_new_debt() {
        for qty in [1e-6, 1.0, 10.0] {
            let mut w = world();
            let cost = qty * reference_price_bn(Good::Intermediates);
            w.nation_mut(BUYER).treasury_bn = Some(cost);
            buy(&mut w, Good::Intermediates, qty, 30);
            near(w.nation(BUYER).treasury_bn.unwrap(), 0.0);
            near(w.nation(BUYER).debt_bn.unwrap(), 0.0);
            settle(&mut w);
            near(w.commerce.as_ref().unwrap().cargo[0].quantity, qty);
        }
        let mut w = world();
        w.nation_mut(BUYER).treasury_bn = Some(0.00001);
        let before = crate::save(&w);
        assert!(propose(
            &mut w,
            BUYER,
            SELLER,
            Good::Intermediates,
            1.0,
            reference_price_bn(Good::Intermediates),
            30
        )
        .is_err());
        assert_eq!(before, crate::save(&w));
        let q = market_quotes(&w, BUYER, Good::Intermediates, 10.0, 30);
        assert_eq!(q.len(), 1);
        assert!(q[0].total_price_bn <= 0.00001);
        assert!(q[0].quantity < 1.0);
    }
    #[test]
    fn cancel_refunds_only_undispatched_and_debt_retirement_conserves_net_money() {
        let mut w = world();
        let id = buy(&mut w, Good::CapitalGoods, 10.0, 30);
        // Subsequent government borrowing does not turn a refund into cash plus
        // a second debt credit. Existing charge convention retires debt first.
        w.nation_mut(BUYER).debt_bn = Some(0.5);
        let total = money_total(&w);
        cancel(&mut w, BUYER, id).unwrap();
        near(money_total(&w), total);
        near(goods_total(&w, Good::CapitalGoods), 100.0);
        near(w.nation(BUYER).debt_bn.unwrap(), 0.5 - 0.0025);
        let after = crate::save(&w);
        assert!(cancel(&mut w, BUYER, id).is_err());
        assert_eq!(after, crate::save(&w));
        let id = buy(&mut w, Good::Intermediates, 1.0, 30);
        settle(&mut w);
        let paid = crate::save(&w);
        assert!(cancel(&mut w, BUYER, id).is_err());
        assert_eq!(paid, crate::save(&w));
    }
    #[test]
    fn counteroffer_has_no_escrow_until_acceptance_and_revalidates_consent_stock_expiry() {
        let mut w = world();
        let cash = w.nation(BUYER).treasury_bn;
        let p = propose(
            &mut w,
            BUYER,
            SELLER,
            Good::Intermediates,
            95.0,
            reference_price_bn(Good::Intermediates) * 0.5,
            30,
        )
        .unwrap();
        assert_eq!(p.kind, "counter_offer");
        near(stock(&w, SELLER, Good::Intermediates), 100.0);
        assert_eq!(w.nation(BUYER).treasury_bn, cash);
        assert!(accept_offer(&mut w, SELLER, p.id).is_err());
        set_sale(&mut w, SELLER, Good::Intermediates, 10.0, 1.0, false).unwrap();
        assert!(accept_offer(&mut w, BUYER, p.id).is_err());
        set_sale(&mut w, SELLER, Good::Intermediates, 10.0, 1.0, true).unwrap();
        let id = accept_offer(&mut w, BUYER, p.id).unwrap();
        near(stock(&w, SELLER, Good::Intermediates), 10.0);
        assert!(w.commerce.as_ref().unwrap().offers.is_empty());
        cancel(&mut w, BUYER, id).unwrap();
        let p = propose(
            &mut w,
            BUYER,
            SELLER,
            Good::Intermediates,
            1.0,
            reference_price_bn(Good::Intermediates) * 0.5,
            30,
        )
        .unwrap();
        for _ in 0..7 {
            clock::advance_date(&mut w);
        }
        let before = crate::save(&w);
        assert!(accept_offer(&mut w, BUYER, p.id).is_err());
        assert_eq!(before, crate::save(&w));
    }
    #[test]
    fn invalid_inputs_and_identifier_overflow_are_atomic() {
        let mut w = world();
        for qty in [f64::NAN, f64::INFINITY, -1.0, 0.0, MAX_LOT + 1.0] {
            let before = crate::save(&w);
            assert!(propose(&mut w, BUYER, SELLER, Good::Intermediates, qty, 0.0001, 30).is_err());
            assert_eq!(before, crate::save(&w));
        }
        for price in [f64::NAN, f64::INFINITY, -1.0, 0.0, 1.0] {
            let before = crate::save(&w);
            assert!(propose(&mut w, BUYER, SELLER, Good::Intermediates, 1.0, price, 30).is_err());
            assert_eq!(before, crate::save(&w));
        }
        for days in [0, 366, u32::MAX] {
            assert!(propose(
                &mut w,
                BUYER,
                SELLER,
                Good::Intermediates,
                1.0,
                0.0001,
                days
            )
            .is_err());
        }
        w.commerce.as_mut().unwrap().next_id = u64::MAX;
        let before = crate::save(&w);
        assert!(propose(&mut w, BUYER, SELLER, Good::Intermediates, 1.0, 0.00001, 30).is_err());
        assert_eq!(before, crate::save(&w));
    }
    #[test]
    fn raw_and_manufactured_freight_compete_for_the_same_edge_capacity() {
        let mut w = world();
        logistics::begin_month(&mut w);
        let raw = logistics::dispatch(
            &mut w,
            SELLER,
            BUYER,
            Commodity::Copper,
            1e9,
            ShipmentSource::Spot,
            None,
        );
        assert!(raw.quantity > 0.0);
        let goods = logistics::reserve_freight(&mut w, SELLER, BUYER, 1.0, 1.0);
        assert!(
            goods.quantity < 1e-6,
            "raw cargo consumed the common bottleneck"
        );
        next(&mut w);
        let goods = logistics::reserve_freight(&mut w, SELLER, BUYER, 1e9, 1.0);
        assert!(goods.quantity > 0.0);
        let raw = logistics::dispatch(
            &mut w,
            SELLER,
            BUYER,
            Commodity::Copper,
            1.0,
            ShipmentSource::Spot,
            None,
        );
        assert!(
            raw.quantity < 1e-6,
            "manufactured cargo consumed the common bottleneck"
        );
    }
    #[test]
    fn partial_dispatch_cancellation_preserves_paid_cargo_and_remaining_stock() {
        let mut w = world();
        let id = buy(&mut w, Good::Intermediates, 10.0, 30);
        logistics::begin_month(&mut w);
        let route = logistics::plan(&w, SELLER, BUYER).unwrap();
        // Pre-reserve all but three tonnes on every shared route segment.
        for edge in &route.segments {
            w.logistics
                .usage_tonnes
                .insert(edge.clone(), route.capacity_tonnes - 3.0);
        }
        tick_day(&mut w);
        let shipped = w
            .commerce
            .as_ref()
            .unwrap()
            .cargo
            .iter()
            .map(|c| c.quantity)
            .sum::<f64>();
        assert!(
            shipped > 0.0 && shipped < 10.0,
            "partial freight = {shipped}"
        );
        let money = money_total(&w);
        cancel(&mut w, BUYER, id).unwrap();
        near(goods_total(&w, Good::Intermediates), 100.0);
        near(money_total(&w), money);
        let due = w.commerce.as_ref().unwrap().cargo[0].due_day;
        while clock::absolute_day(&w) < due {
            next(&mut w);
        }
        near(stock(&w, BUYER, Good::Intermediates), shipped);
        near(money_total(&w), money);
    }
    #[test]
    fn expiry_and_dead_government_return_only_undispatched_escrow() {
        for dead in [false, true] {
            let mut w = world();
            let id = buy(&mut w, Good::CapitalGoods, 10.0, 1);
            let money = money_total(&w);
            if dead {
                w.nation_mut(SELLER).alive = false;
            } else {
                clock::advance_date(&mut w);
            }
            settle(&mut w);
            near(money_total(&w), money);
            near(stock(&w, SELLER, Good::CapitalGoods), 100.0);
            let c = w
                .commerce
                .as_ref()
                .unwrap()
                .contracts
                .iter()
                .find(|c| c.id == id)
                .unwrap();
            assert_eq!(c.status, "expired");
            near(c.escrow_bn, 0.0);
        }
        let mut w = world();
        buy(&mut w, Good::CapitalGoods, 10.0, 30);
        settle(&mut w);
        w.nation_mut(BUYER).alive = false;
        for _ in 0..40 {
            next(&mut w);
        }
        assert_eq!(w.commerce.as_ref().unwrap().cargo.len(), 1);
        near(stock(&w, BUYER, Good::CapitalGoods), 0.0);
        near(goods_total(&w, Good::CapitalGoods), 100.0);
    }
    #[test]
    fn closure_full_warehouse_and_reload_hold_then_release_without_duplicate_payment() {
        let mut w = world();
        buy(&mut w, Good::Intermediates, 10.0, 30);
        settle(&mut w);
        w.sanctions.push((BUYER, SELLER));
        let paid = money_total(&w);
        for _ in 0..35 {
            next(&mut w);
        }
        assert!(w.commerce.as_ref().unwrap().cargo[0].hold_reason.is_some());
        near(stock(&w, BUYER, Good::Intermediates), 0.0);
        let mut loaded = crate::load(&crate::save(&w)).unwrap();
        assert_eq!(loaded.commerce, w.commerce);
        loaded.sanctions.clear();
        change_stock(&mut loaded, BUYER, Good::Intermediates, 250.0);
        next(&mut loaded);
        assert!(loaded.commerce.as_ref().unwrap().cargo[0]
            .hold_reason
            .as_ref()
            .unwrap()
            .contains("warehouse"));
        change_stock(&mut loaded, BUYER, Good::Intermediates, -5.0);
        next(&mut loaded);
        near(loaded.commerce.as_ref().unwrap().cargo[0].quantity, 5.0);
        change_stock(&mut loaded, BUYER, Good::Intermediates, -5.0);
        next(&mut loaded);
        assert!(loaded.commerce.as_ref().unwrap().cargo.is_empty());
        near(money_total(&loaded), paid);
        let before = crate::save(&loaded);
        tick_day(&mut loaded);
        assert_eq!(before, crate::save(&loaded));
    }
    #[test]
    fn dependency_requires_delivery_and_scales_reference_value_not_price() {
        let mut w = world();
        let qty = 1e-6;
        propose(
            &mut w,
            BUYER,
            SELLER,
            Good::Intermediates,
            qty,
            reference_price_bn(Good::Intermediates) * 100.0,
            30,
        )
        .unwrap();
        near(dependency(&w, BUYER, SELLER), 0.0);
        settle(&mut w);
        near(dependency(&w, BUYER, SELLER), 0.0);
        for _ in 0..35 {
            next(&mut w);
        }
        near(
            dependency(&w, BUYER, SELLER),
            qty * reference_price_bn(Good::Intermediates) / w.nation(BUYER).gdp,
        );
        assert!(dependency(&w, BUYER, SELLER) < 1e-8);
        near(dependency(&w, SELLER, BUYER), 0.0);
        for _ in 0..365 {
            clock::advance_date(&mut w);
        }
        near(dependency(&w, BUYER, SELLER), 0.0);
    }
    #[test]
    fn thirty_day_demand_never_invents_consumers_or_double_orders() {
        let mut w = world();
        near(demand(&w, BUYER, Good::Intermediates), 0.0);
        near(shortage(&w, BUYER, Good::CapitalGoods), 0.0);
        let district = w
            .districts
            .iter()
            .find(|(_, owner)| **owner == BUYER)
            .unwrap()
            .0
            .clone();
        let mut levels = [0; 7];
        levels[0] = 1;
        w.production.industry.sites.insert(district, levels);
        near(demand(&w, BUYER, Good::Intermediates), 15.0);
        buy(&mut w, Good::Intermediates, 15.0, 30);
        near(shortage(&w, BUYER, Good::Intermediates), 0.0);
        settle(&mut w);
        near(shortage(&w, BUYER, Good::Intermediates), 0.0);
    }
    #[test]
    fn disabled_legacy_save_stays_byte_identical_and_views_are_pure() {
        let mut w = world_1990(GameRules::default());
        let before = crate::save(&w);
        tick_day(&mut w);
        let _ = snapshot(&w, BUYER);
        let _ = market_quotes(&w, BUYER, Good::Intermediates, 1.0, 30);
        assert_eq!(before, crate::save(&w));
        assert!(!before.contains("\"commerce\""));
        assert!(!before.contains("\"economic_competition\""));
        let loaded = crate::load(&before).unwrap();
        assert!(loaded.commerce.is_none());
    }
    #[test]
    fn fully_paid_inflight_orders_still_count_toward_outstanding_limit() {
        let mut w = world();
        for _ in 0..MAX_ACTIVE {
            buy(&mut w, Good::Intermediates, 0.1, 30);
        }
        settle(&mut w);
        assert_eq!(w.commerce.as_ref().unwrap().cargo.len(), MAX_ACTIVE);
        assert!(w
            .commerce
            .as_ref()
            .unwrap()
            .contracts
            .iter()
            .all(|c| c.remaining_quantity == 0.0));
        let before = crate::save(&w);
        assert!(
            proposal_refusal(&w, BUYER, SELLER, Good::Intermediates, 0.1, 0.0001, 30)
                .unwrap()
                .contains("existing")
        );
        assert!(propose(&mut w, BUYER, SELLER, Good::Intermediates, 0.1, 0.0001, 30).is_err());
        assert_eq!(before, crate::save(&w));
    }
    #[test]
    fn accepted_counter_honors_signed_price_but_not_stock_already_sold() {
        let mut w = world();
        let p = propose(&mut w, BUYER, SELLER, Good::CapitalGoods, 2.0, 0.0001, 30).unwrap();
        assert_eq!(p.kind, "counter_offer");
        set_sale(&mut w, SELLER, Good::CapitalGoods, 10.0, 2.0, true).unwrap();
        let id = accept_offer(&mut w, BUYER, p.id).unwrap();
        near(
            w.commerce
                .as_ref()
                .unwrap()
                .contracts
                .iter()
                .find(|c| c.id == id)
                .unwrap()
                .unit_price_bn,
            0.00025,
        );
        let p = propose(&mut w, BUYER, SELLER, Good::CapitalGoods, 88.0, 0.0001, 30).unwrap();
        // A seller's remaining stock is not reserved by an unanswered offer.
        change_stock(&mut w, SELLER, Good::CapitalGoods, -1.0);
        let before = crate::save(&w);
        assert!(accept_offer(&mut w, BUYER, p.id).is_err());
        assert_eq!(before, crate::save(&w));
    }
    #[test]
    fn leap_day_routes_and_daily_save_resume_use_absolute_dates() {
        let mut w = world();
        w.year = 1992;
        w.month = 2;
        w.day = 28;
        buy(&mut w, Good::CapitalGoods, 10.0, 30);
        settle(&mut w);
        let cargo = &w.commerce.as_ref().unwrap().cargo[0];
        assert_eq!(
            cargo.due_day - cargo.dispatched_day,
            cargo.route.estimated_days.max(1) as i32
        );
        let mut loaded = crate::load(&crate::save(&w)).unwrap();
        tick_day(&mut loaded);
        assert_eq!(w.commerce, loaded.commerce);
        for _ in 0..40 {
            next(&mut w);
            next(&mut loaded);
            assert_eq!(w.commerce, loaded.commerce);
            assert_eq!(
                w.production.industry.goods,
                loaded.production.industry.goods
            );
        }
        near(stock(&w, BUYER, Good::CapitalGoods), 10.0);
    }
    #[test]
    fn real_system_order_delivers_goods_before_industry_and_replays() {
        let mut w = world();
        buy(&mut w, Good::CapitalGoods, 10.0, 30);
        crate::tick_day(&mut w, &[]);
        assert!(!w.commerce.as_ref().unwrap().cargo.is_empty());
        let mut loaded = crate::load(&crate::save(&w)).unwrap();
        for _ in 0..12 {
            crate::tick_day(&mut w, &[]);
            crate::tick_day(&mut loaded, &[]);
        }
        assert_eq!(crate::save(&w), crate::save(&loaded));
        near(stock(&w, BUYER, Good::CapitalGoods), 10.0);
        assert!(w.commerce.as_ref().unwrap().cargo.is_empty());
    }
}
