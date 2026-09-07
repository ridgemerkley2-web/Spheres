//! Paid, player-established domestic contractors. No historical company assets
//! are invented. Corporate cash, work and unsold stock are separate from the
//! government; departmental settlement is the only public expenditure posting.
use crate::{
    clock, equipment, programs, resources,
    world::{NationId, WorldState, BUDGET_DEFENSE},
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// Version one contains tanks; version two adds existing ground/air platforms;
/// version three adds finite company ammunition and its delivery provenance.
/// Version four adds prepaid refit services with separately locked funds.
/// Older books retain their versions until the new feature is actually used.
pub const VERSION: u32 = 4;
pub const TANK_VERSION: u32 = 1;
pub const EQUIPMENT_VERSION: u32 = 2;
pub const AMMUNITION_VERSION: u32 = 3;
pub const MAX_STOCK: u32 = 12;
pub const MAX_PRODUCTS: usize = 32;
pub const DELIVERY_DAYS: u32 = 7;
pub const MARGIN: f64 = 0.15;

include!("companies_ammunition.rs");
include!("companies_refits.rs");

pub fn supported_platform(platform: &str) -> bool {
    equipment::PLATFORMS.iter().any(|p| p.id == platform)
}
pub fn product_family(platform: &str) -> &'static str {
    if equipment::is_aviation_platform(platform) {
        "aircraft"
    } else {
        "ground"
    }
}
/// Singular label for stock quantities; aircraft is intentionally invariant.
pub fn unit_label(platform: &str) -> &'static str {
    if equipment::is_aviation_platform(platform) {
        "aircraft"
    } else {
        "vehicle"
    }
}
pub fn platform_name(platform: &str) -> &str {
    equipment::PLATFORMS
        .iter()
        .find(|p| p.id == platform)
        .map_or(platform, |p| p.name)
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum CompanyOrder {
    Establish {
        name: String,
        district: String,
        capitalization_bn: f64,
        quote: String,
    },
    Capitalize {
        company: u32,
        amount_bn: f64,
        quote: String,
    },
    Develop {
        company: u32,
        name: String,
        spec: equipment::DesignSpec,
        daily_budget_bn: f64,
        stock_target: u32,
        quote: String,
    },
    Purchase {
        company: u32,
        product: u32,
        quantity: u32,
        quote: String,
    },
    Funding {
        company: u32,
        product: u32,
        daily_budget_bn: f64,
    },
    Inventory {
        company: u32,
        product: u32,
        stock_target: u32,
    },
    CancelDevelopment {
        company: u32,
        product: u32,
    },
    AmmoSupply {
        company: u32,
        family: String,
        stock_target: u32,
        quote: String,
    },
    AmmoInventory {
        company: u32,
        product: u32,
        stock_target: u32,
    },
    AmmoPurchase {
        company: u32,
        product: u32,
        quantity: u32,
        quote: String,
    },
    Refit {
        company: u32,
        source: String,
        product: u32,
        quantity: u32,
        quote: String,
    },
    CancelRefit {
        company: u32,
        refit: u32,
        quote: String,
    },
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Companies {
    pub version: u32,
    pub next_id: u32,
    pub firms: Vec<Company>,
    pub deliveries: Vec<Delivery>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ammunition_deliveries: Vec<AmmoDelivery>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_tick_day: Option<i32>,
}
impl Companies {
    pub fn is_empty(&self) -> bool {
        self.firms.is_empty()
            && self.deliveries.is_empty()
            && self.ammunition_deliveries.is_empty()
            && self.next_id == 0
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Company {
    pub id: u32,
    pub nation: NationId,
    pub name: String,
    pub ownership: String,
    /// Exclusive use of one existing completed plant slot. No new factory.
    pub district: String,
    pub established_day: i32,
    pub cash_bn: f64,
    pub capital_received_bn: f64,
    pub development_revenue_bn: f64,
    pub sales_revenue_bn: f64,
    pub development_expense_bn: f64,
    pub tooling_expense_bn: f64,
    pub materials_expense_bn: f64,
    pub fabrication_expense_bn: f64,
    pub receivables: Vec<Receivable>,
    pub products: Vec<Product>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ammunition_products: Vec<AmmoProduct>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub refits: Vec<RefitContract>,
    #[serde(default, skip_serializing_if = "zero_refit_money")]
    pub refit_advances_received_bn: f64,
    #[serde(default, skip_serializing_if = "zero_refit_money")]
    pub refit_revenue_bn: f64,
    #[serde(default, skip_serializing_if = "zero_refit_money")]
    pub refit_refunds_bn: f64,
    pub transactions: Vec<Transaction>,
    pub last_tick_day: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Product {
    pub id: u32,
    pub design_nation: NationId,
    pub revision_id: String,
    pub status: String,
    pub reason: String,
    pub daily_budget_bn: f64,
    pub stock_target: u32,
    pub development_work_days: f64,
    pub development_spent_bn: f64,
    pub tooling_work_days: f64,
    pub tooling_spent_bn: f64,
    pub unit_work_days: f64,
    pub unit_spent_bn: f64,
    pub unit_material_cost_bn: f64,
    /// One recipe transferred from the sovereign warehouse into this WIP unit.
    pub unit_inputs: [f64; 12],
    pub stock: u32,
    pub stock_cost_bn: f64,
    pub produced_units: u32,
    pub sold_units: u32,
    pub started_day: i32,
    pub certified_day: Option<i32>,
    pub cancelled_day: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Receivable {
    pub id: u32,
    pub day: i32,
    pub kind: String,
    pub amount_bn: f64,
    pub product: Option<u32>,
    pub delivery: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refit: Option<u32>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Transaction {
    pub day: i32,
    pub kind: String,
    pub amount_bn: f64,
    pub product: Option<u32>,
    pub quantity: u32,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Delivery {
    pub id: u32,
    pub company: u32,
    pub buyer: NationId,
    pub product: u32,
    pub revision_id: String,
    pub district: String,
    pub quantity: u32,
    pub unit_price_bn: f64,
    pub total_price_bn: f64,
    pub cost_basis_bn: f64,
    pub purchased_day: i32,
    pub settled_day: Option<i32>,
    pub due_day: Option<i32>,
    pub delivered_day: Option<i32>,
    pub status: String,
    pub reason: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct Quote {
    pub valid: bool,
    pub reason: Option<String>,
    pub token: String,
    pub cost_bn: f64,
    pub minimum_days: u32,
    pub eta_days: Option<u32>,
    pub first_stock_days: Option<u32>,
    pub unit_price_bn: f64,
    pub available_units: u32,
    pub maintenance_bn_day: f64,
    pub company_cash_needed_bn: f64,
    pub protected_maintenance_bn: f64,
    pub purchase_available_bn: f64,
    pub note: String,
}
impl Default for Quote {
    fn default() -> Self {
        Self {valid:false,reason:None,token:String::new(),cost_bn:0.0,minimum_days:0,eta_days:None,first_stock_days:None,unit_price_bn:0.0,available_units:0,maintenance_bn_day:0.0,company_cash_needed_bn:0.0,protected_maintenance_bn:0.0,purchase_available_bn:0.0,note:"Explicit game assumptions. Company capital, development fees and finished-stock purchases are separate payments. Estimates require funding, inputs and continuous facility access.".into()}
    }
}

fn actor(w: &WorldState, n: NationId) -> Option<String> {
    equipment::actor_refusal(w, n)
        .or_else(|| {
            (!w.rules.resource_market || !w.rules.manufacturing_system)
                .then(|| "Companies require the resource market and military manufacturing.".into())
        })
        .or_else(|| {
            w.nation_opt(n)
                .and_then(|n| equipment::activation_refusal(n))
        })
}
fn finite_money(value: f64, positive: bool) -> bool {
    value.is_finite() && value >= if positive { 0.000001 } else { 0.0 } && value <= 1000.0
}
pub fn company(w: &WorldState, n: NationId, id: u32) -> Option<&Company> {
    w.companies
        .firms
        .iter()
        .find(|c| c.id == id && c.nation == n)
}
fn indexes(w: &WorldState, n: NationId, c: u32, p: u32) -> Result<(usize, usize), String> {
    let i = w
        .companies
        .firms
        .iter()
        .position(|f| f.id == c && f.nation == n)
        .ok_or("This domestic company is missing.")?;
    let j = w.companies.firms[i]
        .products
        .iter()
        .position(|x| x.id == p)
        .ok_or("This licensed product is missing.")?;
    Ok((i, j))
}
pub fn reserved_slots(w: &WorldState, n: NationId, district: &str) -> usize {
    w.companies
        .firms
        .iter()
        .filter(|c| c.nation == n && c.district == district)
        .count()
}
/// Establishing a company explicitly turns unassigned procurement authority
/// into money held for reviewed purchases, without cancelling named public work.
pub fn procurement_active(w: &WorldState, n: NationId) -> bool {
    clock::is_daily(w) && w.companies.firms.iter().any(|c| c.nation == n)
}
pub fn licensed_revision(w: &WorldState, n: NationId, revision: &str) -> bool {
    w.companies.firms.iter().filter(|c| c.nation == n).any(|c| {
        c.products
            .iter()
            .any(|p| p.revision_id == revision && p.cancelled_day.is_none())
    })
}
pub fn facility_blocker(w: &WorldState, c: &Company) -> Option<String> {
    if !w.nation_opt(c.nation).is_some_and(|n| n.alive) {
        return Some("The commissioning government is inactive; property remains recorded.".into());
    }
    if let Some(e) = crate::control::blocker(w, c.nation, &c.district) {
        return Some(e);
    }
    if w.districts.get(&c.district) != Some(&c.nation) {
        return Some("The company's leased plant is outside government control.".into());
    }
    let old = crate::manufacturing::lines_for(w, c.nation)
        .filter(|l| l.district == c.district)
        .count()
        + equipment::occupied_site_slots_today(
            w.nation(c.nation),
            &c.district,
            clock::absolute_day(w),
        );
    let rank = w
        .companies
        .firms
        .iter()
        .filter(|f| f.nation == c.nation && f.district == c.district && f.id < c.id)
        .count();
    if old + rank >= crate::manufacturing::plant_slots(w, &c.district) as usize {
        return Some("The leased Arms Plant slot is unavailable; existing paid public work retains priority.".into());
    }
    None
}
fn next_id(w: &mut WorldState) -> u32 {
    if w.companies.version == 0 {
        w.companies.version = TANK_VERSION;
    }
    let x = w.companies.next_id.max(1);
    w.companies.next_id = x
        .checked_add(1)
        .expect("preflighted company identity limit");
    x
}
fn token(w: &WorldState, n: NationId, params: serde_json::Value) -> String {
    // A deterministic review fingerprint, not authentication. Include relevant
    // national rights/fiscal state and the complete sparse supplier book so a
    // replay, second buyer, facility change or amended design cannot reuse it.
    let nstate = w.nation_opt(n);
    let bytes = serde_json::to_vec(&(
        clock::absolute_day(w),
        params,
        &w.companies,
        &w.districts,
        &w.manufacturing,
        &w.production,
        nstate.map(|n| {
            (
                &n.program_budget,
                &n.equipment,
                &n.arsenal,
                &n.treasury_bn,
                &n.debt_bn,
            )
        }),
        w.rules.resource_market,
        w.rules.production_system,
        w.rules.manufacturing_system,
        w.rules.military_operations,
    ))
    .unwrap_or_default();
    let hash = bytes.iter().fold(0xcbf29ce484222325u64, |h, b| {
        (h ^ *b as u64).wrapping_mul(0x100000001b3)
    });
    format!("company-v1-{hash:016x}")
}
fn finish_quote(w: &WorldState, n: NationId, params: serde_json::Value, mut q: Quote) -> Quote {
    if w.companies.next_id >= u32::MAX - 8 {
        q.reason = Some("This company's transaction library is full.".into());
    }
    q.valid = q.reason.is_none();
    q.token = token(
        w,
        n,
        serde_json::json!([
            params,
            q.cost_bn,
            q.unit_price_bn,
            q.available_units,
            q.company_cash_needed_bn,
            q.protected_maintenance_bn,
            q.purchase_available_bn,
            q.eta_days,
            q.first_stock_days
        ]),
    );
    q
}
fn sufficient(w: &WorldState, n: NationId, department: usize, cost: f64) -> Option<String> {
    (programs::available_bn(w,n,BUDGET_DEFENSE,department)<cost).then(||if department==4 {"Defense R&D funding is exhausted.".into()} else {"Defense procurement authority is insufficient. Let funding accrue or adjust the departmental budget.".into()})
}
pub fn establishment_quote(
    w: &WorldState,
    n: NationId,
    name: &str,
    district: &str,
    capital: f64,
) -> Quote {
    let mut q = Quote {
        cost_bn: capital.max(0.0),
        ..Quote::default()
    };
    q.reason=actor(w,n).or_else(||equipment::name_refusal(name)).or_else(||(!finite_money(capital,true)).then(||"Choose positive initial capital between $1,000 and $1,000bn.".into()))
        .or_else(||w.companies.firms.iter().any(|c|c.nation==n).then(||"This first release supports one player-established state contractor per country.".into()))
        .or_else(||crate::control::blocker(w,n,district))
        .or_else(||(w.districts.get(district)!=Some(&n)).then(||"Choose a domestic province under government control.".into()))
        .or_else(||(crate::manufacturing::used_slots(w,n,district)>=crate::manufacturing::plant_slots(w,district) as usize).then(||"Choose a free completed Arms Plant slot; construction creates facilities, companies do not.".into()))
        .or_else(||sufficient(w,n,3,capital));
    q.note="Establish a modeled state-owned contractor, transfer the reviewed capital from Defense procurement, and grant exclusive use of one existing free Arms Plant slot. Capital becomes spendable after fiscal settlement. Background automatic catalogue purchases stop so unassigned procurement authority can accrue for reviewed stock purchases; explicit public lines, projects, ammunition and paid deliveries continue. No historical company, cash or equipment is invented.".into();
    finish_quote(
        w,
        n,
        serde_json::json!(["establish", name, district, capital]),
        q,
    )
}
pub fn capitalization_quote(w: &WorldState, n: NationId, id: u32, amount: f64) -> Quote {
    let q = Quote {
        cost_bn: amount.max(0.0),
        reason: actor(w, n)
            .or_else(|| {
                company(w, n, id)
                    .is_none()
                    .then(|| "This domestic company is missing.".into())
            })
            .or_else(|| {
                (!finite_money(amount, true))
                    .then(|| "Choose a positive, finite capital transfer up to $1,000bn.".into())
            })
            .or_else(|| sufficient(w, n, 3, amount)),
        ..Quote::default()
    };
    finish_quote(w, n, serde_json::json!(["capitalize", id, amount]), q)
}
fn material_cost(w: &WorldState, recipe: &[f64; 12]) -> f64 {
    resources::ALL
        .iter()
        .map(|c| recipe[c.idx()] * resources::market_current_price(w, *c) / 1e9)
        .sum()
}
pub fn development_quote(
    w: &WorldState,
    n: NationId,
    id: u32,
    name: &str,
    spec: &equipment::DesignSpec,
    budget: f64,
    target: u32,
) -> Quote {
    let old = equipment::development_quote(w, n, name, spec, budget);
    let profile = equipment::design_preview(w, n, spec).profile;
    let c = company(w, n, id);
    let reason=actor(w,n).or(old.reason)
        .or_else(||(!supported_platform(&spec.platform)).then(||"Choose one of the implemented ground vehicles or tactical aircraft in the designer.".into()))
        .or_else(||(!(1..=MAX_STOCK).contains(&target)).then(||format!("Choose an initial stock target of 1–{MAX_STOCK} complete units.")))
        .or_else(||c.is_none().then(||"Establish a domestic contractor first.".into()))
        .or_else(||c.and_then(|c|facility_blocker(w,c)))
        .or_else(||c.and_then(|c|(c.products.len()>=MAX_PRODUCTS).then(||"The company's product library is full.".into())))
        .or_else(||c.and_then(|c|c.products.iter().any(|p|p.cancelled_day.is_none()&&p.certified_day.is_none()).then(||"Finish or cancel the company's current development contract first.".into())))
        .or_else(||c.and_then(|c|c.products.iter().find(|p|p.cancelled_day.is_none()&&w.nation(n).equipment.as_ref().and_then(|s|s.revisions.get(&p.revision_id)).is_some_and(|r|r.spec==*spec)).map(|_|"This exact revision is already licensed to the contractor.".into())));
    let mut q = Quote {
        reason,
        cost_bn: old.cost_bn,
        minimum_days: old.minimum_days,
        eta_days: old.eta_days,
        ..Quote::default()
    };
    if let Some(p) = profile {
        let raw = material_cost(w, &p.recipe);
        q.unit_price_bn = (p.fabrication_cost_bn + raw) * (1.0 + MARGIN);
        q.company_cash_needed_bn =
            p.tooling_cost_bn + (p.fabrication_cost_bn + raw) * target as f64;
        q.maintenance_bn_day = p.maintenance_bn_day;
        q.first_stock_days = q.eta_days.map(|d| {
            d.saturating_add(p.tooling_days)
                .saturating_add(p.production_days)
                .saturating_add(2)
        });
        // Development takes the next work packet, but production returns to
        // earlier licensed products before starting this one's tooling. Future
        // sales and their restock buffers are not a fixed, payable order book.
        if c.is_some_and(|c| {
            c.refits.iter().any(|p| refit_remaining(p) > 0)
                || c.ammunition_products
                    .iter()
                    .any(|p| p.stock < p.stock_target)
                || c.products.iter().any(|p| {
                    p.cancelled_day.is_none()
                        && (p.stock < p.stock_target
                            || p.unit_work_days > 0.0
                            || p.unit_inputs.iter().any(|v| *v > 0.0))
                })
        }) {
            q.first_stock_days = None;
            q.note.push_str(" First stock has no dated estimate while earlier licensed products have unfinished work or unmet stock targets, or a contracted refit is pending. All work shares this plant slot; development completion remains separately estimated.");
        }
    }
    finish_quote(
        w,
        n,
        serde_json::json!(["develop", id, name, spec, budget, target]),
        q,
    )
}
pub fn purchase_quote(w: &WorldState, n: NationId, id: u32, product: u32, quantity: u32) -> Quote {
    let mut q = Quote {
        minimum_days: DELIVERY_DAYS,
        eta_days: Some(DELIVERY_DAYS),
        ..Quote::default()
    };
    q.reason = actor(w, n).or_else(|| {
        (!(1..=MAX_STOCK).contains(&quantity))
            .then(|| format!("Choose 1–{MAX_STOCK} complete units from available stock."))
    });
    if let Some(c) = company(w, n, id) {
        q.reason = q.reason.or_else(|| facility_blocker(w, c));
        if let Some(p) = c.products.iter().find(|p| p.id == product) {
            q.available_units = p.stock;
            q.unit_price_bn = if p.stock > 0 {
                p.stock_cost_bn / p.stock as f64 * (1.0 + MARGIN)
            } else {
                0.0
            };
            q.cost_bn = q.unit_price_bn * quantity as f64;
            q.reason=q.reason.or_else(||(p.certified_day.is_none()||p.cancelled_day.is_some()).then(||"This product has not completed development and certification.".into()))
                .or_else(||(p.stock<quantity).then(||"The requested quantity exceeds the company's finished stock. This creates no production order.".into()));
            if let Some(r) = w
                .nation(n)
                .equipment
                .as_ref()
                .and_then(|s| s.revisions.get(&p.revision_id))
            {
                q.maintenance_bn_day = r.profile.maintenance_bn_day * quantity as f64;
            }
            q.reason = q.reason.or_else(|| sufficient(w, n, 3, q.cost_bn));
        } else {
            q.reason = Some("This licensed product is missing.".into());
        }
    } else {
        q.reason = Some("This domestic company is missing.".into());
    }
    q.note="Buys only finished company-owned stock. Price includes paid materials and fabrication plus a 15% modeled margin; development and tooling are not charged again. Domestic delivery is seven days after fiscal settlement. Transit pauses if the shipping province is unavailable; paid property is retained and resumes on restored access.".into();
    finish_quote(
        w,
        n,
        serde_json::json!(["purchase", id, product, quantity]),
        q,
    )
}

fn accept(q: Quote, provided: &str) -> Result<Quote, String> {
    if !q.valid {
        return Err(q
            .reason
            .unwrap_or_else(|| "This order is unavailable.".into()));
    }
    if q.token != provided {
        return Err("This reviewed quote is stale. Review the latest funding, facility and stock before confirming.".into());
    }
    Ok(q)
}
fn charge(w: &mut WorldState, n: NationId, department: usize, amount: f64) -> Result<(), String> {
    settle_receivables(w);
    programs::begin_day(w);
    programs::spend_operating(w, n, BUDGET_DEFENSE, department, amount)
}
fn receipt(
    w: &mut WorldState,
    i: usize,
    kind: &str,
    amount: f64,
    product: Option<u32>,
    delivery: Option<u32>,
) {
    let id = next_id(w);
    let day = clock::absolute_day(w);
    w.companies.firms[i].receivables.push(Receivable {
        id,
        day,
        kind: kind.into(),
        amount_bn: amount,
        product,
        delivery,
        refit: None,
    });
}
pub fn apply(w: &mut WorldState, n: NationId, order: &CompanyOrder) -> Result<(), String> {
    // Opening a fiscal day itself mutates ledgers. Preserve full atomicity even
    // if an exceptional funding refusal happens after an otherwise valid quote.
    let before = w.clone();
    let result = apply_inner(w, n, order);
    if result.is_err() {
        *w = before;
    }
    result
}
fn apply_inner(w: &mut WorldState, n: NationId, order: &CompanyOrder) -> Result<(), String> {
    if let Some(reason) = actor(w, n) {
        return Err(reason);
    }
    match order {
        CompanyOrder::Establish {
            name,
            district,
            capitalization_bn,
            quote,
        } => {
            accept(
                establishment_quote(w, n, name, district, *capitalization_bn),
                quote,
            )?;
            charge(w, n, 3, *capitalization_bn)?;
            // Materialize only the exact pre-company opening warehouse if it
            // was still represented by legacy cover. No draw, grant or import.
            resources::consume_stockpile_atomic(w, n, &[0.0; 12])
                .map_err(|_| "The opening national warehouse could not be seated.")?;
            let day = clock::absolute_day(w);
            equipment::activate(w.nation_mut(n), day);
            let id = next_id(w);
            w.companies.firms.push(Company {
                id,
                nation: n,
                name: name.trim().into(),
                ownership: "state".into(),
                district: district.clone(),
                established_day: day,
                cash_bn: 0.0,
                capital_received_bn: 0.0,
                development_revenue_bn: 0.0,
                sales_revenue_bn: 0.0,
                development_expense_bn: 0.0,
                tooling_expense_bn: 0.0,
                materials_expense_bn: 0.0,
                fabrication_expense_bn: 0.0,
                receivables: vec![],
                products: vec![],
                ammunition_products: vec![],
                refits: vec![],
                refit_advances_received_bn: 0.0,
                refit_revenue_bn: 0.0,
                refit_refunds_bn: 0.0,
                transactions: vec![],
                last_tick_day: None,
            });
            let i = w.companies.firms.len() - 1;
            receipt(w, i, "capitalization", *capitalization_bn, None, None);
        }
        CompanyOrder::Capitalize {
            company,
            amount_bn,
            quote,
        } => {
            accept(capitalization_quote(w, n, *company, *amount_bn), quote)?;
            charge(w, n, 3, *amount_bn)?;
            let i = w
                .companies
                .firms
                .iter()
                .position(|c| c.id == *company && c.nation == n)
                .unwrap();
            receipt(w, i, "capitalization", *amount_bn, None, None);
        }
        CompanyOrder::Develop {
            company,
            name,
            spec,
            daily_budget_bn,
            stock_target,
            quote,
        } => {
            accept(
                development_quote(w, n, *company, name, spec, *daily_budget_bn, *stock_target),
                quote,
            )?;
            let preview = equipment::design_preview(w, n, spec);
            let day = clock::absolute_day(w);
            equipment::activate(w.nation_mut(n), day);
            let s = w.nation_mut(n).equipment.as_mut().unwrap();
            s.version = equipment::VERSION;
            let number = s.next_id;
            s.next_id += 1;
            let revision_id = format!("{}-design-{number}", n.code());
            s.revisions.insert(
                revision_id.clone(),
                equipment::DesignRevision {
                    id: revision_id.clone(),
                    name: name.trim().into(),
                    spec: spec.clone(),
                    profile: preview.profile.unwrap(),
                    created_day: day,
                    certified_day: None,
                    specification_key: preview.specification_key,
                },
            );
            if !spec.platform.starts_with("tank_") {
                w.companies.version = w.companies.version.max(EQUIPMENT_VERSION);
            }
            let id = next_id(w);
            let c = w
                .companies
                .firms
                .iter_mut()
                .find(|c| c.id == *company && c.nation == n)
                .unwrap();
            c.products.push(Product{id,design_nation:n,revision_id,status:"development".into(),reason:"Engineering begins on the next funded day; prototypes are not saleable equipment.".into(),daily_budget_bn:*daily_budget_bn,stock_target:*stock_target,development_work_days:0.0,development_spent_bn:0.0,tooling_work_days:0.0,tooling_spent_bn:0.0,unit_work_days:0.0,unit_spent_bn:0.0,unit_material_cost_bn:0.0,unit_inputs:[0.0;12],stock:0,stock_cost_bn:0.0,produced_units:0,sold_units:0,started_day:day,certified_day:None,cancelled_day:None});
        }
        CompanyOrder::Purchase {
            company,
            product,
            quantity,
            quote,
        } => {
            let q = accept(purchase_quote(w, n, *company, *product, *quantity), quote)?;
            let (i, j) = indexes(w, n, *company, *product)?;
            let day = clock::absolute_day(w);
            let id = next_id(w);
            // The ID is checked and all payment/stock conditions were quoted.
            // Funding failure is still atomic (next_id is restored below).
            if let Err(e) = charge(w, n, 3, q.cost_bn) {
                w.companies.next_id = id;
                return Err(e);
            }
            let c = &mut w.companies.firms[i];
            let p = &mut c.products[j];
            let basis = p.stock_cost_bn / p.stock as f64 * (*quantity as f64);
            p.stock -= *quantity;
            p.stock_cost_bn = (p.stock_cost_bn - basis).max(0.0);
            p.sold_units += *quantity;
            w.companies.deliveries.push(Delivery{id,company:*company,buyer:n,product:*product,revision_id:p.revision_id.clone(),district:c.district.clone(),quantity:*quantity,unit_price_bn:q.unit_price_bn,total_price_bn:q.cost_bn,cost_basis_bn:basis,purchased_day:day,settled_day:None,due_day:None,delivered_day:None,status:"awaiting_settlement".into(),reason:"Paid stock is reserved to the government; shipping begins after the fiscal day settles.".into()});
            receipt(w, i, "sale", q.cost_bn, Some(*product), Some(id));
        }
        CompanyOrder::Funding {
            company,
            product,
            daily_budget_bn,
        } => {
            if !finite_money(*daily_budget_bn, false) {
                return Err(
                    "Choose a finite daily development budget between zero and $1,000bn.".into(),
                );
            }
            let (i, j) = indexes(w, n, *company, *product)?;
            let p = &mut w.companies.firms[i].products[j];
            if p.certified_day.is_some() || p.cancelled_day.is_some() {
                return Err(
                    "Only an active development contract has a public funding ceiling.".into(),
                );
            }
            p.daily_budget_bn = *daily_budget_bn;
        }
        CompanyOrder::Inventory {
            company,
            product,
            stock_target,
        } => {
            if *stock_target > MAX_STOCK {
                return Err(format!(
                    "Choose a finished-stock buffer between 0 and {MAX_STOCK}."
                ));
            }
            let (i, j) = indexes(w, n, *company, *product)?;
            let p = &mut w.companies.firms[i].products[j];
            if p.cancelled_day.is_some() {
                return Err("This development contract was cancelled.".into());
            }
            p.stock_target = *stock_target;
        }
        CompanyOrder::CancelDevelopment { company, product } => {
            let day = clock::absolute_day(w);
            let (i, j) = indexes(w, n, *company, *product)?;
            let p = &mut w.companies.firms[i].products[j];
            if p.certified_day.is_some() || p.cancelled_day.is_some() {
                return Err("Only an unfinished development contract can be cancelled.".into());
            }
            p.cancelled_day = Some(day);
            p.status = "cancelled".into();
            p.reason="Future development charges stopped. Previously performed and invoiced work remains payable; no prototypes become saleable units.".into();
        }
        CompanyOrder::AmmoSupply {
            company,
            family,
            stock_target,
            quote,
        } => {
            start_ammo_supply(w, n, *company, family, *stock_target, quote)?;
        }
        CompanyOrder::AmmoInventory {
            company,
            product,
            stock_target,
        } => {
            set_ammo_inventory(w, n, *company, *product, *stock_target)?;
        }
        CompanyOrder::AmmoPurchase {
            company,
            product,
            quantity,
            quote,
        } => {
            buy_ammo_stock(w, n, *company, *product, *quantity, quote)?;
        }
        CompanyOrder::Refit {
            company,
            source,
            product,
            quantity,
            quote,
        } => {
            start_company_refit(w, n, *company, source, *product, *quantity, quote)?;
        }
        CompanyOrder::CancelRefit {
            company,
            refit,
            quote,
        } => {
            cancel_company_refit(w, n, *company, *refit, quote)?;
        }
    }
    Ok(())
}

fn transaction(
    c: &mut Company,
    day: i32,
    kind: &str,
    amount: f64,
    product: Option<u32>,
    quantity: u32,
) {
    c.transactions.push(Transaction {
        day,
        kind: kind.into(),
        amount_bn: amount,
        product,
        quantity,
    });
    // Cumulative accounts retain all money; this is only the recent activity
    // window, bounded so a long campaign does not grow without limit.
    if c.transactions.len() > 256 {
        c.transactions.remove(0);
    }
}

/// Department authority creates a receivable, never immediate corporate cash.
/// Even prepaid authority waits for its public day's close, conservatively.
/// Repeating this hook or loading between invoice and settlement pays once.
pub fn settle_receivables(w: &mut WorldState) {
    if w.companies.is_empty() {
        return;
    }
    for i in 0..w.companies.firms.len() {
        let n = w.companies.firms[i].nation;
        let settled = w
            .nation_opt(n)
            .and_then(|n| n.program_budget.as_ref())
            .and_then(|p| p.settled_day);
        let Some(day) = settled else { continue };
        let receipts: Vec<_> = w.companies.firms[i]
            .receivables
            .iter()
            .filter(|r| r.day == day)
            .cloned()
            .collect();
        for r in receipts {
            if r.kind == "refit_advance" {
                settle_refit_advance(w, i, &r, day);
                continue;
            }
            let c = &mut w.companies.firms[i];
            c.cash_bn += r.amount_bn;
            match r.kind.as_str() {
                "capitalization" => c.capital_received_bn += r.amount_bn,
                "development" => {
                    c.development_revenue_bn += r.amount_bn;
                    c.development_expense_bn += r.amount_bn;
                    c.cash_bn -= r.amount_bn;
                }
                "sale" | "ammo_sale" => c.sales_revenue_bn += r.amount_bn,
                _ => continue,
            }
            transaction(c, day, &r.kind, r.amount_bn, r.product, 0);
            c.receivables.retain(|x| x.id != r.id);
            if let Some(id) = r.delivery {
                if r.kind == "ammo_sale" {
                    if let Some(d) = w
                        .companies
                        .ammunition_deliveries
                        .iter_mut()
                        .find(|d| d.id == id && d.settled_day.is_none())
                    {
                        d.settled_day = Some(day);
                        d.due_day = Some(day.saturating_add(DELIVERY_DAYS as i32));
                        d.status = "in_transit".into();
                        d.reason = "Purchased ammunition is in domestic transit; it cannot be fired before arrival.".into();
                    }
                    continue;
                }
                if let Some(d) = w
                    .companies
                    .deliveries
                    .iter_mut()
                    .find(|d| d.id == id && d.settled_day.is_none())
                {
                    d.settled_day = Some(day);
                    d.due_day = Some(day.saturating_add(DELIVERY_DAYS as i32));
                    d.status = "in_transit".into();
                    d.reason="Purchased equipment is in domestic transit; no military strength or upkeep before delivery.".into();
                }
            }
        }
    }
}

pub fn inbound_units(w: &WorldState, n: NationId, revision: &str) -> u32 {
    w.companies
        .deliveries
        .iter()
        .filter(|d| d.buyer == n && d.revision_id == revision && d.delivered_day.is_none())
        .map(|d| d.quantity)
        .fold(0, u32::saturating_add)
}

fn delivery_blocker(w: &WorldState, d: &Delivery) -> Option<String> {
    if !w.nation_opt(d.buyer).is_some_and(|n| n.alive) {
        return Some(
            "The buyer government is inactive; paid equipment remains owned in transit.".into(),
        );
    }
    if w.districts.get(&d.district) != Some(&d.buyer) {
        return Some(
            "Domestic shipping is paused while the source province is outside government control."
                .into(),
        );
    }
    crate::control::blocker(w, d.buyer, &d.district)
}

fn blocked(w: &mut WorldState, i: usize, j: usize, why: String) {
    let p = &mut w.companies.firms[i].products[j];
    p.status = "blocked".into();
    p.reason = why;
}

pub fn tick_day(w: &mut WorldState) {
    if !clock::is_daily(w) || w.companies.is_empty() {
        return;
    }
    let day = clock::absolute_day(w);
    if w.companies.last_tick_day == Some(day) {
        return;
    }
    w.companies.last_tick_day = Some(day);
    tick_ammo_deliveries(w, day);
    // Deliveries own their units until this one transfer into Arsenal. They
    // never also occupy the ordinary arsenal order book.
    for i in 0..w.companies.deliveries.len() {
        let d = w.companies.deliveries[i].clone();
        if d.delivered_day.is_some() || d.settled_day.is_none() || d.settled_day == Some(day) {
            continue;
        }
        if let Some(reason) = delivery_blocker(w, &d) {
            let d = &mut w.companies.deliveries[i];
            d.status = "blocked".into();
            d.reason = reason;
            d.due_day = d.due_day.map(|due| due.saturating_add(1));
            continue;
        }
        if d.due_day.is_some_and(|due| day >= due) {
            match crate::arsenal::deliver_design(
                w.nation_mut(d.buyer),
                &d.revision_id,
                d.quantity,
                0.0,
            ) {
                Ok(()) => {
                    let d = &mut w.companies.deliveries[i];
                    d.delivered_day = Some(day);
                    d.status = "delivered".into();
                    d.reason="Received into the exact-revision Arsenal; ordinary maintenance and ammunition rules now apply.".into();
                }
                Err(reason) => {
                    let d = &mut w.companies.deliveries[i];
                    d.status = "blocked".into();
                    d.reason = reason;
                }
            }
        } else {
            let d = &mut w.companies.deliveries[i];
            d.status = "in_transit".into();
            d.reason = "Domestic delivery is progressing.".into();
        }
    }
    for i in 0..w.companies.firms.len() {
        let c = w.companies.firms[i].clone();
        if c.last_tick_day == Some(day) {
            continue;
        }
        w.companies.firms[i].last_tick_day = Some(day);
        let facility = facility_blocker(w, &c);
        // One leased slot, one daily work packet. A development commission
        // temporarily takes priority over refits and finite inventory. Refit
        // services take the slot before unpurchased equipment/ammunition stock.
        let selected_id = scheduled_product(&c);
        let selected = c.products.iter().position(|p| Some(p.id) == selected_id);
        for j in 0..c.products.len() {
            if c.products[j].cancelled_day.is_some() {
                continue;
            }
            if selected != Some(j) {
                let p = &mut w.companies.firms[i].products[j];
                p.status = if p.stock > 0 { "in_stock" } else { "idle" }.into();
                p.reason = if p.stock >= p.stock_target {
                    "The finite stock target is met. No public purchase is implied."
                } else {
                    "Waiting for the leased plant's current product."
                }
                .into();
            }
        }
        update_ammo_queue(w, i, selected_id);
        update_refit_queue(w, i, selected_id);
        if let Some(j) = c.refits.iter().position(|p| Some(p.id) == selected_id) {
            tick_company_refit(w, i, j, day, facility);
            continue;
        }
        if let Some(j) = c
            .ammunition_products
            .iter()
            .position(|p| Some(p.id) == selected_id)
        {
            tick_ammo_manufacturing(w, i, j, day, facility);
            continue;
        }
        let Some(j) = selected else { continue };
        let p = c.products[j].clone();
        if let Some(reason) = facility {
            blocked(w, i, j, reason);
            continue;
        }
        if day <= p.started_day {
            continue;
        }
        let Some(r) = w
            .nation(c.nation)
            .equipment
            .as_ref()
            .and_then(|s| s.revisions.get(&p.revision_id))
            .cloned()
        else {
            blocked(
                w,
                i,
                j,
                "The immutable licensed revision is missing.".into(),
            );
            continue;
        };
        let profile = &r.profile;
        if p.certified_day.is_none() {
            if w.companies.next_id == u32::MAX {
                blocked(w,i,j,"The corporate transaction identity limit is reached; no new work can be invoiced.".into());
                continue;
            }
            let per_day = profile.development_cost_bn / profile.development_days.max(1) as f64;
            let available = programs::available_bn(w, c.nation, BUDGET_DEFENSE, 4);
            let remaining = (profile.development_cost_bn - p.development_spent_bn).max(0.0);
            let daily_cost =
                if p.development_work_days + 1.0 + 1e-9 >= profile.development_days as f64 {
                    remaining
                } else {
                    per_day.min(remaining)
                };
            let pay = daily_cost
                .min(p.daily_budget_bn)
                .min(available)
                .min(remaining);
            if pay <= 0.0 {
                blocked(
                    w,
                    i,
                    j,
                    if p.daily_budget_bn <= 0.0 {
                        "Development is paused at a zero funding ceiling."
                    } else {
                        "Waiting for Defense R&D authority; no fee or work is posted today."
                    }
                    .into(),
                );
                continue;
            }
            if let Err(reason) = programs::spend_operating(w, c.nation, BUDGET_DEFENSE, 4, pay) {
                blocked(w, i, j, reason);
                continue;
            }
            receipt(w, i, "development", pay, Some(p.id), None);
            let p = &mut w.companies.firms[i].products[j];
            p.development_spent_bn += pay;
            p.development_work_days =
                (p.development_work_days + pay / per_day).min(profile.development_days as f64);
            let progress = p.development_work_days / profile.development_days.max(1) as f64;
            p.status = if progress < 0.5 {
                "engineering"
            } else if progress < 0.8 {
                "prototype"
            } else {
                "trials"
            }
            .into();
            p.reason="Actual engineering and trials are invoiced as work progresses. No prototype is fielded or for sale.".into();
            if p.development_spent_bn >= profile.development_cost_bn
                && p.development_work_days + 1e-8 >= profile.development_days as f64
            {
                p.certified_day = Some(day);
                p.status = "tooling".into();
                p.reason="Certified. The company now funds tooling and finite inventory from its own settled cash.".into();
                w.nation_mut(c.nation)
                    .equipment
                    .as_mut()
                    .unwrap()
                    .revisions
                    .get_mut(&r.id)
                    .unwrap()
                    .certified_day = Some(day);
            }
            continue;
        }
        if p.tooling_work_days + 1e-9 < profile.tooling_days as f64 {
            let left = (profile.tooling_cost_bn - p.tooling_spent_bn).max(0.0);
            let pay = if p.tooling_work_days + 1.0 + 1e-9 >= profile.tooling_days as f64 {
                left
            } else {
                (profile.tooling_cost_bn / profile.tooling_days.max(1) as f64).min(left)
            };
            if c.cash_bn < pay {
                blocked(
                    w,
                    i,
                    j,
                    "The company needs settled working capital for production tooling.".into(),
                );
                continue;
            }
            let c = &mut w.companies.firms[i];
            c.cash_bn -= pay;
            c.tooling_expense_bn += pay;
            transaction(c, day, "tooling", pay, Some(p.id), 0);
            let p = &mut c.products[j];
            p.tooling_work_days = (p.tooling_work_days + 1.0).min(profile.tooling_days as f64);
            p.tooling_spent_bn += pay;
            p.status = "tooling".into();
            p.reason = "The company is paying for tooling at its leased Arms Plant.".into();
            continue;
        }
        let mut inputs_cost = 0.0;
        if p.unit_inputs.iter().all(|x| *x == 0.0) && p.unit_work_days == 0.0 {
            inputs_cost = material_cost(w, &profile.recipe);
            // Reserve the full unit's fabrication liquidity before acquiring
            // inputs, so a cash-starved firm does not accumulate unlimited WIP.
            if c.cash_bn < inputs_cost + profile.fabrication_cost_bn {
                blocked(w,i,j,"Company cash cannot cover one complete unit's inputs and fabrication. Add capital or await settled sales.".into());
                continue;
            }
            if let Some(c) = resources::ALL.into_iter().find(|commodity| {
                resources::stockpile(w, c.nation, *commodity) < profile.recipe[commodity.idx()]
            }) {
                blocked(w,i,j,format!("The domestic warehouse lacks {} for a complete equipment recipe. Company purchases cannot create materials.",c.name()));
                continue;
            }
            if let Err((commodity, _, _)) =
                resources::consume_stockpile_atomic(w, c.nation, &profile.recipe)
            {
                blocked(
                    w,
                    i,
                    j,
                    format!("Waiting for domestic {} stock.", commodity.name()),
                );
                continue;
            }
            let cash = w.nation(c.nation).gdp.max(0.1);
            crate::economy::charge(w, c.nation, -inputs_cost, -inputs_cost / cash);
            let c = &mut w.companies.firms[i];
            c.cash_bn -= inputs_cost;
            c.materials_expense_bn += inputs_cost;
            transaction(c, day, "materials", inputs_cost, Some(p.id), 0);
            let p = &mut c.products[j];
            p.unit_inputs = profile.recipe;
            p.unit_spent_bn = inputs_cost;
            p.unit_material_cost_bn = inputs_cost;
        }
        let fabrication_paid = (p.unit_spent_bn - p.unit_material_cost_bn).max(0.0);
        // Daily fabricated work uses the frozen physical duration. Paid input
        // price belongs to cost basis and never changes vehicle performance.
        let remainder = (profile.fabrication_cost_bn - fabrication_paid).max(0.0);
        let payment = if p.unit_work_days + 1.0 + 1e-9 >= profile.production_days as f64 {
            remainder
        } else {
            (profile.fabrication_cost_bn / profile.production_days.max(1) as f64).min(remainder)
        };
        let c = &mut w.companies.firms[i];
        if c.cash_bn < payment {
            blocked(w,i,j,"Company working capital is exhausted; purchased WIP inputs remain company property.".into());
            continue;
        }
        c.cash_bn -= payment;
        c.fabrication_expense_bn += payment;
        transaction(c, day, "fabrication", payment, Some(p.id), 0);
        let p = &mut c.products[j];
        p.unit_work_days = (p.unit_work_days + 1.0).min(profile.production_days as f64);
        p.unit_spent_bn += payment;
        p.status = "manufacturing".into();
        p.reason =
            "The company is building finite unsold stock with its own inputs and working capital."
                .into();
        if p.unit_work_days + 1e-9 >= profile.production_days as f64 {
            p.stock += 1;
            p.produced_units = p.produced_units.saturating_add(1);
            p.stock_cost_bn += p.unit_spent_bn;
            p.unit_work_days = 0.0;
            p.unit_spent_bn = 0.0;
            p.unit_material_cost_bn = 0.0;
            p.unit_inputs = [0.0; 12];
            p.status = "in_stock".into();
            p.reason="Finished company-owned equipment is available to buy. It adds no government capability or upkeep.".into();
        }
        let _ = inputs_cost;
    }
}

/// Server-owned supplier read model. Display code need not guess phase timing,
/// price, departmental authority or who owns the visible stock.
pub fn view(w: &WorldState, n: NationId) -> serde_json::Value {
    let day = clock::absolute_day(w);
    let firms:Vec<_>=w.companies.firms.iter().filter(|c|c.nation==n).map(|c|{
        let scheduled=scheduled_product(c);
        let mut value=serde_json::to_value(c).unwrap();
        value["facility_blocker"]=serde_json::json!(facility_blocker(w,c));
        value["receivable_bn"]=serde_json::json!(c.receivables.iter().map(|r|r.amount_bn).sum::<f64>());
        value["expenses_bn"]=serde_json::json!(c.development_expense_bn+c.tooling_expense_bn+c.materials_expense_bn+c.fabrication_expense_bn);
        value["refit_escrow_bn"]=serde_json::json!(c.refits.iter().map(|p|p.escrow_bn).sum::<f64>());
        value["refit_working_capital_locked_bn"]=serde_json::json!(refit_locked_capital(c));
        value["inventory_cost_bn"]=serde_json::json!(c.products.iter().map(|p|p.stock_cost_bn+p.unit_spent_bn).sum::<f64>()+c.ammunition_products.iter().map(|p|p.stock_cost_bn).sum::<f64>());
        value["ammunition_products"]=serde_json::json!(c.ammunition_products.iter().map(|p|ammo_product_view(w,c,p)).collect::<Vec<_>>());
        value["products"]=serde_json::json!(c.products.iter().map(|p|{
            let mut v=serde_json::to_value(p).unwrap();
            if let Some(r)=w.nation(n).equipment.as_ref().and_then(|s|s.revisions.get(&p.revision_id)) {
                let x=&r.profile;let inputs=material_cost(w,&x.recipe);let new_cost=x.fabrication_cost_bn+inputs;
                let development_left=(x.development_cost_bn-p.development_spent_bn).max(0.0);
                let dev_days=if p.certified_day.is_some(){Some(0)}else if p.daily_budget_bn>0.0{Some(((development_left/p.daily_budget_bn).ceil() as u32).max((x.development_days as f64-p.development_work_days).max(0.0).ceil() as u32))}else{None};
                let tool_days=(x.tooling_days as f64-p.tooling_work_days).max(0.0).ceil() as u32;
                let unit_days=(x.production_days as f64-p.unit_work_days).max(0.0).ceil() as u32;
                let cash_needed=(x.tooling_cost_bn-p.tooling_spent_bn).max(0.0)+if p.unit_work_days>0.0 {(x.fabrication_cost_bn-(p.unit_spent_bn-p.unit_material_cost_bn)).max(0.0)}else{new_cost};
                let materials_ready=p.unit_work_days>0.0||resources::ALL.into_iter().all(|commodity|resources::stockpile(w,n,commodity)>=x.recipe[commodity.idx()]);
                let earlier_backlog=p.certified_day.is_none()&&(c.refits.iter().any(|p|refit_remaining(p)>0)||c.ammunition_products.iter().any(|earlier|earlier.id<p.id&&earlier.stock<earlier.stock_target)||c.products.iter().take_while(|earlier|earlier.id!=p.id).any(|earlier|equipment_pending(earlier)));
                let eta=if scheduled!=Some(p.id)||earlier_backlog||p.cancelled_day.is_some()||facility_blocker(w,c).is_some()||c.cash_bn<cash_needed||!materials_ready {None}else{dev_days.map(|d|d.saturating_add(tool_days).saturating_add(unit_days))};
                v["name"]=serde_json::json!(r.name);v["spec"]=serde_json::json!(r.spec);v["profile"]=serde_json::json!(x);v["source_revision"]=serde_json::json!(r.id);
                v["platform"]=serde_json::json!(r.spec.platform);v["platform_name"]=serde_json::json!(platform_name(&r.spec.platform));v["family"]=serde_json::json!(product_family(&r.spec.platform));v["unit_label"]=serde_json::json!(unit_label(&r.spec.platform));
                v["development_cost_bn"]=serde_json::json!(x.development_cost_bn);v["development_days"]=serde_json::json!(x.development_days);v["development_remaining_days"]=serde_json::json!(dev_days);v["tooling_cost_bn"]=serde_json::json!(x.tooling_cost_bn);v["tooling_days"]=serde_json::json!(x.tooling_days);v["production_days"]=serde_json::json!(x.production_days);
                v["unit_price_bn"]=serde_json::json!(if p.stock>0 {p.stock_cost_bn/p.stock as f64*(1.0+MARGIN)}else{new_cost*(1.0+MARGIN)});v["estimated_stock_days"]=serde_json::json!(eta);v["company_cash_needed_bn"]=serde_json::json!(cash_needed);v["maintenance_bn_day"]=serde_json::json!(x.maintenance_bn_day);v["estimate_note"]=serde_json::json!(if earlier_backlog {"First stock has no dated estimate while earlier licensed products need unfinished work or restocking on this same plant slot. Development completion remains separately estimated."}else{"Estimate assumes renewed Defense R&D authority, the current daily funding ceiling, sufficient settled company cash and continuing materials/facility access. Other licensed products wait for the current work packet."});
                let target=equipment::fleet_target_plans_world(w,n).into_iter().find(|target|target.revision==p.revision_id);
                v["fleet_target"]=serde_json::json!(target);
            }
            v
        }).collect::<Vec<_>>());value
    }).collect();
    let deliveries: Vec<_> = w
        .companies
        .deliveries
        .iter()
        .filter(|d| d.buyer == n)
        .map(|d| {
            let mut value = serde_json::to_value(d).unwrap();
            value["remaining_days"] = serde_json::json!(d.due_day.map(|due| (due - day).max(0)));
            if let Some(r) = w
                .nation(d.buyer)
                .equipment
                .as_ref()
                .and_then(|s| s.revisions.get(&d.revision_id))
            {
                value["platform"] = serde_json::json!(r.spec.platform);
                value["platform_name"] = serde_json::json!(platform_name(&r.spec.platform));
                value["family"] = serde_json::json!(product_family(&r.spec.platform));
                value["unit_label"] = serde_json::json!(unit_label(&r.spec.platform));
            }
            value
        })
        .collect();
    let sites:Vec<_>=w.districts.iter().filter(|(_,owner)|**owner==n).filter_map(|(district,_)|{let slots=crate::manufacturing::plant_slots(w,district);if slots==0{return None}let used=crate::manufacturing::used_slots(w,n,district);let reason=crate::control::blocker(w,n,district).or_else(||(used>=slots as usize).then(||"All completed Arms Plant slots are assigned.".to_string()));Some(serde_json::json!({"district":district,"slots":slots,"used_slots":used,"available":reason.is_none(),"reason":reason}))}).collect();
    let (protected, ammo_available) = ammo_purchase_funding(w, n);
    serde_json::json!({"enabled":actor(w,n).is_none(),"reason":actor(w,n),"day":day,"nation":n.code(),"companies":firms,"refits":refits_view(w,n),"max_active_refits":MAX_ACTIVE_REFITS,"deliveries":deliveries,"ammunition_deliveries":ammo_deliveries_view(w,n),"ammunition_catalog":equipment::ammo_catalog().iter().map(|d|serde_json::json!({"id":d.id,"name":d.name,"unit_label":if d.unit=="stores"{"store"}else{"round"},"rounds_per_day":d.rounds_per_day,"eligible":ammo_refusal(w,n,d.id).is_none(),"reason":ammo_refusal(w,n,d.id),"converted":ammo_supplier_active(w,n,d.id)})).collect::<Vec<_>>(),"protected_maintenance_bn":protected,"ammo_purchase_available_bn":ammo_available,"max_ammo_stock":MAX_AMMO_STOCK,"sites":sites,"procurement_available_bn":programs::available_bn(w,n,BUDGET_DEFENSE,3),"development_available_bn":programs::available_bn(w,n,BUDGET_DEFENSE,4),"legacy_automatic_procurement":!procurement_active(w,n),"max_stock":MAX_STOCK,"supported_platforms":equipment::PLATFORMS.iter().map(|p|serde_json::json!({"id":p.id,"name":p.name,"family":product_family(p.id),"unit_label":unit_label(p.id)})).collect::<Vec<_>>(),"note":"Domestic equipment and ammunition suppliers share one explicitly capitalized state contractor and its existing Arms Plant. Companies pay for raw inputs and finite stock; equipment also requires paid development and tooling. Government stock purchases settle and arrive before military use. Establishing a contractor stops background automatic catalogue purchases, and licensing an ammunition family stops new automatic public reserve batches for that family. Explicit public work and already paid deliveries continue. Prices and lead times are game assumptions; no historical firm, balance or extra factory is invented."})
}

/// Older saves omit this sparse book. Present state is strict: a malformed
/// company cannot create money, licensed designs, raw inputs or delivered units.
pub fn validate_state(w: &WorldState) -> Result<(), String> {
    if w.companies.is_empty() {
        if w.nations.iter().any(|n| {
            n.equipment
                .as_ref()
                .is_some_and(|s| !s.company_refits.is_empty())
        }) {
            return Err(
                "Company refit reservations require their matching service contracts.".into(),
            );
        }
        if w.nations.iter().any(|n| {
            n.equipment
                .as_ref()
                .and_then(|s| s.ammunition.as_ref())
                .is_some_and(|a| !a.supplier_receipts.is_empty())
        }) {
            return Err(
                "Supplier ammunition receipts require their matching company property book.".into(),
            );
        }
        return Ok(());
    }
    let state = &w.companies;
    let day = clock::absolute_day(w);
    if !(TANK_VERSION..=VERSION).contains(&state.version)
        || state.firms.len() > crate::nations::nation_count()
        || state.deliveries.len() > 100_000
    {
        return Err("Unsupported or oversized company save state.".into());
    }
    let mut identities = BTreeSet::new();
    let mut nations = BTreeSet::new();
    let mut revisions = BTreeSet::new();
    let finite = |v: f64| v.is_finite() && v >= 0.0;
    let near = |a: f64, b: f64| (a - b).abs() <= 1e-12 + 1e-10 * a.abs().max(b.abs());
    for c in &state.firms {
        if c.id == 0
            || !identities.insert(c.id)
            || !nations.insert(c.nation)
            || w.nation_opt(c.nation).is_none()
            || equipment::name_refusal(&c.name).is_some()
            || c.ownership != "state"
            || crate::districts::name_of(&c.district).is_none()
            || c.established_day > day
            || c.products.len() > MAX_PRODUCTS
            || c.transactions.len() > 256
        {
            return Err("Invalid company identity, ownership or facility rights.".into());
        }
        if ![
            c.cash_bn,
            c.capital_received_bn,
            c.development_revenue_bn,
            c.sales_revenue_bn,
            c.development_expense_bn,
            c.tooling_expense_bn,
            c.materials_expense_bn,
            c.fabrication_expense_bn,
            c.refit_advances_received_bn,
            c.refit_revenue_bn,
            c.refit_refunds_bn,
        ]
        .into_iter()
        .all(finite)
            || !near(c.development_revenue_bn, c.development_expense_bn)
            || !near(
                c.cash_bn + refit_locked_capital(c),
                c.capital_received_bn + c.sales_revenue_bn + c.refit_revenue_bn
                    - c.tooling_expense_bn
                    - c.materials_expense_bn
                    - c.fabrication_expense_bn,
            )
        {
            return Err("Corporate cash and settled financial accounts do not reconcile.".into());
        }
        for p in &c.products {
            let Some(r) = w
                .nation(c.nation)
                .equipment
                .as_ref()
                .and_then(|s| s.revisions.get(&p.revision_id))
            else {
                return Err("A company license names a missing frozen design revision.".into());
            };
            let x = &r.profile;
            if p.id == 0
                || !identities.insert(p.id)
                || p.design_nation != c.nation
                || !revisions.insert((p.design_nation, p.revision_id.clone()))
                || !supported_platform(&r.spec.platform)
                || state.version == TANK_VERSION && !r.spec.platform.starts_with("tank_")
                || p.started_day < c.established_day
                || p.started_day > day
                || p.certified_day != r.certified_day
                || p.stock_target > MAX_STOCK
                || p.stock > MAX_STOCK
                || p.stock.checked_add(p.sold_units) != Some(p.produced_units)
                || p.cancelled_day.is_some() && p.certified_day.is_some()
                || ![
                    p.daily_budget_bn,
                    p.development_work_days,
                    p.development_spent_bn,
                    p.tooling_work_days,
                    p.tooling_spent_bn,
                    p.unit_work_days,
                    p.unit_spent_bn,
                    p.unit_material_cost_bn,
                    p.stock_cost_bn,
                ]
                .into_iter()
                .all(finite)
                || !p.unit_inputs.into_iter().all(finite)
                || p.development_spent_bn > x.development_cost_bn + 1e-8
                || p.development_work_days > x.development_days as f64 + 1e-8
                || p.tooling_spent_bn > x.tooling_cost_bn + 1e-8
                || p.tooling_work_days > x.tooling_days as f64 + 1e-8
                || p.unit_work_days > x.production_days as f64 + 1e-8
                || p.certified_day.is_some() && !near(p.development_spent_bn, x.development_cost_bn)
                || !near(
                    p.development_spent_bn,
                    p.development_work_days / x.development_days.max(1) as f64
                        * x.development_cost_bn,
                )
                || p.unit_inputs.iter().any(|v| *v > 0.0) && p.unit_inputs != x.recipe
                || p.unit_spent_bn + 1e-12 < p.unit_material_cost_bn
                || p.stock == 0 && p.stock_cost_bn > 1e-8
            {
                return Err(
                    "Invalid company development, inventory or frozen-revision accounting.".into(),
                );
            }
            if p.certified_day.is_none()
                && (p.tooling_work_days > 0.0
                    || p.tooling_spent_bn > 0.0
                    || p.unit_work_days > 0.0
                    || p.unit_inputs.iter().any(|v| *v > 0.0)
                    || p.produced_units > 0
                    || p.unit_spent_bn > 0.0)
                || p.produced_units > 0 && !near(p.tooling_spent_bn, x.tooling_cost_bn)
                || !near(
                    p.tooling_spent_bn,
                    p.tooling_work_days / x.tooling_days.max(1) as f64 * x.tooling_cost_bn,
                )
                || !near(
                    p.unit_spent_bn - p.unit_material_cost_bn,
                    p.unit_work_days / x.production_days.max(1) as f64 * x.fabrication_cost_bn,
                )
                || p.unit_work_days > 0.0 && p.unit_inputs != x.recipe
                || p.unit_inputs.iter().all(|v| *v == 0.0) && p.unit_material_cost_bn > 0.0
                || p.stock_cost_bn + 1e-8 < p.stock as f64 * x.fabrication_cost_bn
            {
                return Err(
                    "Company inventory is not supported by certified, paid physical work.".into(),
                );
            }
            let sold = state
                .deliveries
                .iter()
                .filter(|d| d.company == c.id && d.product == p.id)
                .map(|d| d.quantity as u64)
                .sum::<u64>();
            if sold != p.sold_units as u64 {
                return Err(
                    "Company sold units do not reconcile with purchased delivery ownership.".into(),
                );
            }
        }
        let mut purchase_invoices = BTreeSet::new();
        for r in &c.receivables {
            if r.id == 0
                || !identities.insert(r.id)
                || !finite(r.amount_bn)
                || r.amount_bn == 0.0
                || r.day > day
                || !matches!(
                    r.kind.as_str(),
                    "capitalization" | "development" | "sale" | "ammo_sale" | "refit_advance"
                )
                || r.product.is_some_and(|id| {
                    if r.kind == "ammo_sale" {
                        !c.ammunition_products.iter().any(|p| p.id == id)
                    } else {
                        !c.products.iter().any(|p| p.id == id)
                    }
                })
            {
                return Err("Invalid corporate receivable.".into());
            }
            let plan = w
                .nation(c.nation)
                .program_budget
                .as_ref()
                .ok_or("Corporate invoices require their fiscal ledger.")?;
            if plan.day != Some(r.day) {
                return Err(
                    "A corporate invoice has lost its corresponding public fiscal day.".into(),
                );
            }
            let linked = match r.kind.as_str() {
                "capitalization" => {
                    r.product.is_none() && r.delivery.is_none() && r.refit.is_none()
                }
                "development" => r.product.is_some() && r.delivery.is_none() && r.refit.is_none(),
                "sale" => {
                    r.product.is_some()
                        && r.refit.is_none()
                        && r.delivery.is_some_and(|id| {
                            state.deliveries.iter().any(|d| {
                                d.id == id
                                    && d.company == c.id
                                    && Some(d.product) == r.product
                                    && d.purchased_day == r.day
                                    && d.settled_day.is_none()
                                    && near(d.total_price_bn, r.amount_bn)
                            })
                        })
                }
                "ammo_sale" => {
                    r.product.is_some()
                        && r.refit.is_none()
                        && r.delivery.is_some_and(|id| {
                            state.ammunition_deliveries.iter().any(|d| {
                                d.id == id
                                    && d.company == c.id
                                    && Some(d.product) == r.product
                                    && d.purchased_day == r.day
                                    && d.settled_day.is_none()
                                    && near(d.total_price_bn, r.amount_bn)
                            })
                        })
                }
                "refit_advance" => {
                    r.delivery.is_none()
                        && r.refit.is_some_and(|id| {
                            c.refits.iter().any(|p| {
                                p.id == id
                                    && Some(p.product) == r.product
                                    && p.booked_day == r.day
                                    && p.settled_day.is_none()
                                    && near(p.total_price_bn, r.amount_bn)
                            })
                        })
                }
                _ => false,
            };
            if !linked
                || matches!(r.kind.as_str(), "sale" | "ammo_sale")
                    && !purchase_invoices.insert(r.delivery.unwrap())
            {
                return Err(
                    "Corporate receivable identity does not match its contract or owned purchase."
                        .into(),
                );
            }
        }
        if let Some(plan) = w.nation(c.nation).program_budget.as_ref() {
            for department in [2, 3, 4] {
                let pending = c
                    .receivables
                    .iter()
                    .filter(|r| {
                        if department == 2 {
                            r.kind == "ammo_sale"
                        } else if department == 4 {
                            r.kind == "development"
                        } else {
                            r.kind == "sale"
                                || r.kind == "capitalization"
                                || r.kind == "refit_advance"
                        }
                    })
                    .map(|r| r.amount_bn)
                    .sum::<f64>();
                let posted = plan.spent_today_bn[BUDGET_DEFENSE][department]
                    + plan.prepaid_used_today_bn[BUDGET_DEFENSE][department];
                if pending > posted + 1e-12 {
                    return Err(
                        "Corporate invoices exceed actual corresponding departmental expenditure."
                            .into(),
                    );
                }
            }
        }
        for t in &c.transactions {
            if !finite(t.amount_bn) || t.day > day {
                return Err("Invalid corporate transaction history.".into());
            }
        }
        let pending_development = c
            .receivables
            .iter()
            .filter(|r| r.kind == "development")
            .map(|r| r.amount_bn)
            .sum::<f64>();
        let settled_sales = state
            .deliveries
            .iter()
            .filter(|d| d.company == c.id && d.settled_day.is_some())
            .map(|d| d.total_price_bn)
            .sum::<f64>()
            + state
                .ammunition_deliveries
                .iter()
                .filter(|d| d.company == c.id && d.settled_day.is_some())
                .map(|d| d.total_price_bn)
                .sum::<f64>();
        let sold_cost = state
            .deliveries
            .iter()
            .filter(|d| d.company == c.id)
            .map(|d| d.cost_basis_bn)
            .sum::<f64>()
            + state
                .ammunition_deliveries
                .iter()
                .filter(|d| d.company == c.id)
                .map(|d| d.cost_basis_bn)
                .sum::<f64>();
        let inventory_cost = c
            .products
            .iter()
            .map(|p| p.stock_cost_bn + p.unit_spent_bn)
            .sum::<f64>()
            + c.ammunition_products
                .iter()
                .map(|p| p.stock_cost_bn)
                .sum::<f64>();
        if !near(
            c.development_revenue_bn + pending_development,
            c.products.iter().map(|p| p.development_spent_bn).sum(),
        ) || !near(
            c.tooling_expense_bn,
            c.products.iter().map(|p| p.tooling_spent_bn).sum(),
        ) || !near(c.sales_revenue_bn, settled_sales)
            || !near(
                c.materials_expense_bn + c.fabrication_expense_bn,
                inventory_cost + sold_cost + refit_incurred_cost(c),
            )
        {
            return Err("Company cumulative accounts do not reconcile with work, inventory and settled sales.".into());
        }
    }
    for d in &state.deliveries {
        let c = state
            .firms
            .iter()
            .find(|c| c.id == d.company)
            .ok_or("A purchase names a missing company.")?;
        let p = c
            .products
            .iter()
            .find(|p| p.id == d.product)
            .ok_or("A purchase names a missing product.")?;
        if d.id == 0
            || !identities.insert(d.id)
            || d.buyer != c.nation
            || d.revision_id != p.revision_id
            || d.district != c.district
            || d.quantity == 0
            || d.quantity > MAX_STOCK
            || !finite(d.unit_price_bn)
            || !finite(d.total_price_bn)
            || !finite(d.cost_basis_bn)
            || !near(d.total_price_bn, d.unit_price_bn * d.quantity as f64)
            || !near(d.total_price_bn, d.cost_basis_bn * (1.0 + MARGIN))
            || d.purchased_day > day
            || p.certified_day
                .is_none_or(|certified| d.purchased_day < certified)
            || d.settled_day
                .is_some_and(|settled| settled < d.purchased_day || settled > day)
            || d.due_day
                .zip(d.settled_day)
                .is_some_and(|(due, settled)| due < settled.saturating_add(DELIVERY_DAYS as i32))
            || d.settled_day.is_none() && (d.due_day.is_some() || d.delivered_day.is_some())
            || d.settled_day.is_some() && d.due_day.is_none()
            || d.delivered_day.is_some_and(|d| d > day)
            || d.delivered_day
                .zip(d.due_day)
                .is_some_and(|(arrived, due)| arrived < due)
        {
            return Err("Invalid company purchase ownership or delivery.".into());
        }
        if d.settled_day.is_none()
            && !c.receivables.iter().any(|r| {
                r.delivery == Some(d.id) && r.kind == "sale" && near(r.amount_bn, d.total_price_bn)
            })
        {
            return Err("An unsettled purchase has no matching company invoice.".into());
        }
    }
    validate_ammo_supplier_state(w, &mut identities)?;
    validate_company_refits(w, &mut identities)?;
    if identities.last().is_some_and(|id| *id >= state.next_id) {
        return Err("Company identity sequence would overwrite recorded property.".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const HOME: NationId = NationId::France;
    fn supplier_fixture() -> WorldState {
        let mut w = crate::init::world_1990(crate::world::GameRules {
            daily_simulation: true,
            military_operations: true,
            production_system: true,
            manufacturing_system: true,
            resource_market: true,
            ..Default::default()
        });
        w.player = Some(HOME);
        programs::set_construction_budget(&mut w, HOME, 0.0).unwrap();
        // A synthetic test plant and appropriation. Every later corporate
        // dollar still goes through the ordinary reviewed transfer command.
        let district = w
            .districts
            .iter()
            .find(|(_, owner)| **owner == HOME)
            .unwrap()
            .0
            .clone();
        w.production
            .provinces
            .push(crate::production::ProvinceCapabilities {
                district: district.clone(),
                arms_plants: 1,
                infrastructure: 0,
                civilian_industry: 0,
                power_grid: 0,
                research_centers: 0,
            });
        w.production
            .provinces
            .sort_by(|a, b| a.district.cmp(&b.district));
        w.nation_mut(HOME)
            .program_budget
            .as_mut()
            .unwrap()
            .available_bn[BUDGET_DEFENSE][3] = 1.0;
        let q = establishment_quote(&w, HOME, "Synthetic supplier", &district, 0.01);
        assert!(q.valid, "{:?}", q.reason);
        apply(
            &mut w,
            HOME,
            &CompanyOrder::Establish {
                name: "Synthetic supplier".into(),
                district,
                capitalization_bn: 0.01,
                quote: q.token,
            },
        )
        .unwrap();
        w
    }
    #[test]
    fn ammunition_license_is_sparse_free_of_stock_and_preserves_old_equipment_schema() {
        let mut w = supplier_fixture();
        let company = w.companies.firms[0].id;
        let day = clock::absolute_day(&w);
        let spec = equipment::default_spec("ground_apc");
        let preview = equipment::design_preview(&w, HOME, &spec);
        w.nation_mut(HOME)
            .equipment
            .as_mut()
            .unwrap()
            .revisions
            .insert(
                "synthetic-certified-apc".into(),
                equipment::DesignRevision {
                    id: "synthetic-certified-apc".into(),
                    name: "Synthetic starting certification".into(),
                    spec,
                    profile: preview.profile.unwrap(),
                    created_day: day,
                    certified_day: Some(day),
                    specification_key: preview.specification_key,
                },
            );
        let missing_plan = ammo_supply_quote(&w, HOME, company, "mg_127", 100);
        assert!(!missing_plan.valid);
        equipment::set_maintenance_plan(&mut w, HOME, 0.01).unwrap();
        let national = serde_json::to_value(&w.nations).unwrap();
        let quote = ammo_supply_quote(&w, HOME, company, "mg_127", 100);
        assert!(quote.valid, "{:?}", quote.reason);
        assert_eq!(quote.cost_bn, 0.0);
        apply(
            &mut w,
            HOME,
            &CompanyOrder::AmmoSupply {
                company,
                family: "mg_127".into(),
                stock_target: 100,
                quote: quote.token,
            },
        )
        .unwrap();
        assert_eq!(serde_json::to_value(&w.nations).unwrap(), national);
        assert_eq!(w.companies.version, AMMUNITION_VERSION);
        let p = &w.companies.firms[0].ammunition_products[0];
        let product = p.id;
        assert_eq!((p.stock, p.produced_units, p.sold_units), (0, 0, 0));
        assert_eq!(p.resources_used, [0.0; 12]);
        assert!(w
            .nation(HOME)
            .equipment
            .as_ref()
            .unwrap()
            .ammunition
            .is_none());
        let saved = crate::save(&w);
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&saved).unwrap()["version"],
            4
        );
        assert_eq!(crate::save(&crate::load(&saved).unwrap()), saved);
        let duplicate = ammo_supply_quote(&w, HOME, company, "mg_127", 100);
        assert!(!duplicate.valid);
        assert!(apply(
            &mut w,
            HOME,
            &CompanyOrder::AmmoSupply {
                company,
                family: "mg_127".into(),
                stock_target: 100,
                quote: duplicate.token
            }
        )
        .is_err());
        assert_eq!(crate::save(&w), saved);
        apply(
            &mut w,
            HOME,
            &CompanyOrder::AmmoInventory {
                company,
                product,
                stock_target: 0,
            },
        )
        .unwrap();
        assert_eq!(scheduled_product(&w.companies.firms[0]), None);
        assert_eq!(w.companies.firms[0].ammunition_products[0].stock_target, 0);
        assert_eq!(serde_json::to_value(&w.nations).unwrap(), national);
    }
    #[test]
    fn all_implemented_families_freeze_the_existing_profile_without_another_factory_or_free_stock()
    {
        let template = supplier_fixture();
        for platform in equipment::PLATFORMS {
            let mut w = template.clone();
            let company = w.companies.firms[0].id;
            let spec = equipment::default_spec(platform.id);
            let expected = equipment::design_preview(&w, HOME, &spec).profile.unwrap();
            let arsenal = serde_json::to_string(&w.nation(HOME).arsenal).unwrap();
            let q = development_quote(&w, HOME, company, platform.name, &spec, 0.01, 1);
            assert!(q.valid, "{}: {:?}", platform.id, q.reason);
            apply(
                &mut w,
                HOME,
                &CompanyOrder::Develop {
                    company,
                    name: platform.name.into(),
                    spec: spec.clone(),
                    daily_budget_bn: 0.01,
                    stock_target: 1,
                    quote: q.token,
                },
            )
            .unwrap();
            let c = &w.companies.firms[0];
            let product = &c.products[0];
            let revision =
                &w.nation(HOME).equipment.as_ref().unwrap().revisions[&product.revision_id];
            assert_eq!(revision.profile, expected);
            assert_eq!(revision.spec, spec);
            assert_eq!(
                (product.stock, product.produced_units, product.sold_units),
                (0, 0, 0)
            );
            assert_eq!(product.development_spent_bn, 0.0);
            assert_eq!(product.unit_inputs, [0.0; 12]);
            assert_eq!(crate::manufacturing::used_slots(&w, HOME, &c.district), 1);
            assert_eq!(
                serde_json::to_string(&w.nation(HOME).arsenal).unwrap(),
                arsenal
            );
            assert_eq!(
                w.companies.version,
                if platform.id.starts_with("tank_") {
                    TANK_VERSION
                } else {
                    EQUIPMENT_VERSION
                }
            );
            assert_eq!(
                product_family(platform.id),
                if expected.aviation.is_some() {
                    "aircraft"
                } else {
                    "ground"
                }
            );
            assert_eq!(
                unit_label(platform.id),
                if expected.aviation.is_some() {
                    "aircraft"
                } else {
                    "vehicle"
                }
            );
            let bytes = crate::save(&w);
            assert_eq!(crate::save(&crate::load(&bytes).unwrap()), bytes);
        }
    }
    #[test]
    fn ordinary_transactions_do_not_upgrade_old_tank_books_and_unsupported_designs_are_atomic() {
        let mut w = supplier_fixture();
        let company = w.companies.firms[0].id;
        let original = crate::save(&w);
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&original).unwrap()["version"],
            2
        );
        assert_eq!(crate::save(&crate::load(&original).unwrap()), original);
        let q = capitalization_quote(&w, HOME, company, 0.02);
        apply(
            &mut w,
            HOME,
            &CompanyOrder::Capitalize {
                company,
                amount_bn: 0.02,
                quote: q.token,
            },
        )
        .unwrap();
        assert_eq!(w.companies.version, TANK_VERSION);
        let old = crate::save(&w);
        let mut spec = equipment::default_spec("tank_standard");
        spec.platform = "naval_unimplemented".into();
        let q = development_quote(&w, HOME, company, "Unsupported", &spec, 0.01, 1);
        assert!(!q.valid);
        assert!(apply(
            &mut w,
            HOME,
            &CompanyOrder::Develop {
                company,
                name: "Unsupported".into(),
                spec,
                daily_budget_bn: 0.01,
                stock_target: 1,
                quote: q.token
            }
        )
        .is_err());
        assert_eq!(crate::save(&w), old);
    }
    #[test]
    fn unused_company_book_is_sparse_and_does_not_move_the_world() {
        for daily in [false, true] {
            let mut w = crate::init::world_1990(crate::world::GameRules {
                daily_simulation: daily,
                ..Default::default()
            });
            let before = crate::save(&w);
            tick_day(&mut w);
            settle_receivables(&mut w);
            validate_state(&w).unwrap();
            assert_eq!(crate::save(&w), before);
            assert!(!before.contains("\"companies\""));
            assert!(!procurement_active(&w, NationId::France));
        }
    }
    #[test]
    fn unavailable_quotes_and_refused_orders_do_not_enroll_the_company_layer() {
        let mut w = crate::init::world_1990(Default::default());
        w.player = Some(NationId::France);
        let before = crate::save(&w);
        let q = establishment_quote(
            &w,
            NationId::France,
            "A contractor",
            "not-a-province",
            f64::INFINITY,
        );
        assert!(!q.valid);
        let order = CompanyOrder::Establish {
            name: "A contractor".into(),
            district: "not-a-province".into(),
            capitalization_bn: 0.01,
            quote: q.token,
        };
        assert!(apply(&mut w, NationId::France, &order).is_err());
        assert_eq!(crate::save(&w), before);
    }
}
