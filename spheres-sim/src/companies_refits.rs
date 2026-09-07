// Fixed-price conversion services. Vehicles remain government property. Public
// unearned escrow and company-funded labor locks are distinct, non-spendable
// pools; only a returned whole conversion earns the manufacturer's service fee.
pub const MAX_REFITS: usize = 100_000;
pub const MAX_ACTIVE_REFITS: usize = 32;
fn zero_refit_money(v: &f64) -> bool {
    *v == 0.0
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RefitContract {
    pub id: u32,
    pub product: u32,
    pub source_revision: String,
    pub target_revision: String,
    pub quantity: u32,
    pub completed_units: u32,
    pub cancelled_units: u32,
    pub unit_days: u32,
    pub unit_fabrication_cost_bn: f64,
    pub unit_material_quote_bn: f64,
    pub unit_price_bn: f64,
    pub total_price_bn: f64,
    pub recipe_per_unit: [f64; 12],
    pub advance_received_bn: f64,
    pub escrow_bn: f64,
    pub earned_revenue_bn: f64,
    pub refunded_bn: f64,
    pub working_capital_locked_bn: f64,
    pub unit_started_day: Option<i32>,
    pub unit_work_days: f64,
    pub unit_inputs: [f64; 12],
    pub unit_material_cost_bn: f64,
    pub materials_expense_bn: f64,
    pub fabrication_expense_bn: f64,
    pub resources_used: [f64; 12],
    pub booked_day: i32,
    pub settled_day: Option<i32>,
    pub cancelled_day: Option<i32>,
    pub refund_day: Option<i32>,
    pub closed_day: Option<i32>,
    pub last_work_day: Option<i32>,
    pub status: String,
    pub reason: String,
}
#[derive(Clone, Debug, Serialize)]
pub struct RefitQuote {
    pub valid: bool,
    pub reason: Option<String>,
    pub token: String,
    pub company: u32,
    pub source_revision: String,
    pub target_revision: String,
    pub product: u32,
    pub quantity: u32,
    pub available_units: u32,
    pub reserved_units: u32,
    pub cost_bn: f64,
    pub unit_price_bn: f64,
    pub unit_days: u32,
    pub minimum_days: u32,
    pub eta_days: Option<u32>,
    pub first_return_days: Option<u32>,
    pub unit_fabrication_cost_bn: f64,
    pub unit_material_cost_bn: f64,
    pub recipe_per_unit: [f64; 12],
    pub company_cash_needed_bn: f64,
    pub company_available_cash_bn: f64,
    pub maintenance_bn_day_before: f64,
    pub maintenance_bn_day_after: f64,
    pub note: String,
}
#[derive(Clone, Debug, Serialize)]
pub struct RefitCancelQuote {
    pub valid: bool,
    pub reason: Option<String>,
    pub token: String,
    pub company: u32,
    pub refit: u32,
    pub refundable_units: u32,
    pub active_units: u32,
    pub completed_units: u32,
    pub refund_bn: f64,
    pub refund_after_settlement: bool,
    pub note: String,
}
pub fn refit_remaining(p: &RefitContract) -> u32 {
    p.quantity
        .saturating_sub(p.completed_units)
        .saturating_sub(p.cancelled_units)
}
fn refit_active(p: &RefitContract) -> u32 {
    u32::from(p.unit_started_day.is_some())
}
fn refit_refundable(p: &RefitContract) -> u32 {
    refit_remaining(p).saturating_sub(refit_active(p))
}
fn refit_locked_capital(c: &Company) -> f64 {
    c.refits.iter().map(|p| p.working_capital_locked_bn).sum()
}
fn refit_incurred_cost(c: &Company) -> f64 {
    c.refits
        .iter()
        .map(|p| p.materials_expense_bn + p.fabrication_expense_bn)
        .sum()
}
fn refit_fabrication_cost(c: &Company) -> f64 {
    c.refits.iter().map(|p| p.fabrication_expense_bn).sum()
}
fn refit_indexes(
    w: &WorldState,
    n: NationId,
    company: u32,
    id: u32,
) -> Result<(usize, usize), String> {
    let i = w
        .companies
        .firms
        .iter()
        .position(|c| c.id == company && c.nation == n)
        .ok_or("This domestic company is missing.")?;
    let j = w.companies.firms[i]
        .refits
        .iter()
        .position(|p| p.id == id)
        .ok_or("This manufacturer refit contract is missing.")?;
    Ok((i, j))
}
fn service_terms(
    w: &WorldState,
    n: NationId,
    c: &Company,
    source: &str,
    product: u32,
) -> Result<(String, f64, u32, [f64; 12]), String> {
    let p = c
        .products
        .iter()
        .find(|p| p.id == product)
        .ok_or("Choose a model licensed to this manufacturer.")?;
    if p.certified_day.is_none() || p.cancelled_day.is_some() {
        return Err(
            "The manufacturer must hold a certified production license for the target revision."
                .into(),
        );
    }
    let s = w
        .nation(n)
        .equipment
        .as_ref()
        .ok_or("The equipment library is missing.")?;
    let source = s
        .revisions
        .get(source)
        .filter(|r| r.certified_day.is_some())
        .ok_or("Choose a certified custom source revision.")?;
    let target = s
        .revisions
        .get(&p.revision_id)
        .filter(|r| r.certified_day.is_some())
        .ok_or("The licensed target revision is not certified.")?;
    let (cost, days, recipe) = equipment::refit_terms(source, target)?;
    Ok((target.id.clone(), cost, days, recipe))
}
pub fn refit_quote(
    w: &WorldState,
    n: NationId,
    company_id: u32,
    source: &str,
    product: u32,
    quantity: u32,
) -> RefitQuote {
    let c = company(w, n, company_id);
    let mut reason = actor(w, n)
        .or_else(|| equipment::validate_state(w.nation(n)).err())
        .or_else(|| {
            (!(1..=equipment::MAX_BATCH).contains(&quantity)).then(|| {
                format!(
                    "Choose 1 to {} delivered vehicles or aircraft.",
                    equipment::MAX_BATCH
                )
            })
        })
        .or_else(|| {
            c.is_none()
                .then(|| "Choose a domestic manufacturer.".into())
        })
        .or_else(|| c.and_then(|c| facility_blocker(w, c)))
        .or_else(|| {
            c.and_then(|c| {
                (c.refits.len() >= MAX_REFITS
                    || c.refits.iter().filter(|p| refit_remaining(p) > 0).count()
                        >= MAX_ACTIVE_REFITS)
                    .then(|| "The manufacturer's refit contract book is full.".into())
            })
        })
        .or_else(|| {
            (w.companies.next_id >= u32::MAX - 8)
                .then(|| "The company transaction library is full.".into())
        });
    let holding = w.nation_opt(n).and_then(|n| {
        n.arsenal
            .held
            .iter()
            .find(|h| h.design_id.as_deref() == Some(source))
    });
    let available = holding.map_or(0, crate::arsenal::available_design_units);
    let reserved = holding.map_or(0, |h| h.refit_reserved);
    reason=reason.or_else(||(available<quantity).then(||"There are not enough delivered, unreserved source units. Pending deliveries and other refit reservations are unavailable.".into()));
    let (target, labor, days, recipe) = match c
        .ok_or_else(|| "This domestic company is missing.".to_string())
        .and_then(|c| service_terms(w, n, c, source, product))
    {
        Ok(terms) => terms,
        Err(r) => {
            reason = reason.or(Some(r));
            (String::new(), 0.0, 0, [0.0; 12])
        }
    };
    let raw = material_cost(w, &recipe);
    let unit = (raw + labor) * (1.0 + MARGIN);
    let total = unit * quantity as f64;
    reason = reason
        .or_else(|| {
            (!total.is_finite() || total <= 0.0 || total > 1000.0).then(|| {
                "Choose a positive fixed service contract within the $1,000bn limit.".into()
            })
        })
        .or_else(|| sufficient(w, n, 3, total));
    let required = equipment::profile(w.nation(n), source)
        .map_or(0.0, |p| p.maintenance_bn_day * quantity as f64);
    let change = equipment::profile(w.nation(n), &target)
        .zip(equipment::profile(w.nation(n), source))
        .map_or(0.0, |(to, from)| {
            (to.maintenance_bn_day - from.maintenance_bn_day) * quantity as f64
        });
    let capital = c.map_or(0.0, |c| c.cash_bn);
    let queue_clear = c.is_some_and(|c| {
        !c.products
            .iter()
            .any(|p| p.certified_day.is_none() && p.cancelled_day.is_none())
            && !c.refits.iter().any(|p| refit_remaining(p) > 0)
    });
    let raw_ready = resources::ALL
        .iter()
        .all(|r| resources::stockpile(w, n, *r) >= recipe[r.idx()]);
    let all_raw = resources::ALL
        .iter()
        .all(|r| resources::stockpile(w, n, *r) >= recipe[r.idx()] * quantity as f64);
    let known = reason.is_none() && queue_clear && capital >= raw + labor;
    let mut q=RefitQuote {valid:reason.is_none(),reason,token:String::new(),company:company_id,source_revision:source.into(),target_revision:target,product,quantity,available_units:available,reserved_units:reserved,cost_bn:total,unit_price_bn:unit,unit_days:days,minimum_days:days.saturating_mul(quantity),eta_days:if known&&all_raw{Some(days.saturating_mul(quantity).saturating_add(1))}else{None},first_return_days:if known&&raw_ready{Some(days.saturating_add(1))}else{None},unit_fabrication_cost_bn:labor,unit_material_cost_bn:raw,recipe_per_unit:recipe,company_cash_needed_bn:raw+labor,company_available_cash_bn:capital,maintenance_bn_day_before:required,maintenance_bn_day_after:(required+change).max(0.0),note:"Fixed-price manufacturer service: the full Defense procurement payment settles into locked public escrow. The company buys real inputs and reserves its own labor capital. Only a completed whole conversion earns the fixed unit fee and returns the unit to the same Arsenal, preserving current cohort age. Source units remain government property, pay source upkeep and cannot fight, retire or be reserved again. Work uses the existing conversion rules and one shared plant slot: active development first, contracted refits next, finite stock afterward. Cancel unstarted units for their full fixed fee; an already started unit remains reserved until its conversion finishes. Cancellation before fiscal close waits for the original payment to settle before refund. Refunds return treasury cash or retire debt; they do not recreate departmental authority. No new delivery stage, ammunition or models are granted. Price, margin and durations are game assumptions.".into()};
    q.token = token(w, n, serde_json::json!(["refit", &q]));
    q
}
pub fn refit_cancel_quote(w: &WorldState, n: NationId, company: u32, id: u32) -> RefitCancelQuote {
    let found = refit_indexes(w, n, company, id);
    let p = found
        .as_ref()
        .ok()
        .map(|(i, j)| &w.companies.firms[*i].refits[*j]);
    let quantity = p.map_or(0, refit_refundable);
    let reason=actor(w,n).or_else(||found.err()).or_else(||(quantity==0).then(||"There are no unstarted units to cancel. A started conversion must finish and already returned units remain converted.".into()));
    let mut q=RefitCancelQuote {valid:reason.is_none(),reason,token:String::new(),company,refit:id,refundable_units:quantity,active_units:p.map_or(0,refit_active),completed_units:p.map_or(0,|p|p.completed_units),refund_bn:p.map_or(0.0,|p|p.unit_price_bn*quantity as f64),refund_after_settlement:p.is_some_and(|p|p.settled_day.is_none()),note:"Cancel only unstarted units. Their source reservations are released immediately and their full fixed unit fees return from locked public escrow once the original fiscal payment has settled. This refunds treasury cash or retires debt without restoring spending authority. Started work retains its separate company-funded labor reserve and completes under the original contract; completed conversions are unchanged.".into()};
    q.token = token(w, n, serde_json::json!(["cancel_refit", &q]));
    q
}
fn accept_service(
    valid: bool,
    reason: Option<String>,
    token: &str,
    provided: &str,
) -> Result<(), String> {
    if !valid {
        return Err(reason.unwrap_or_else(|| "This service is unavailable.".into()));
    }
    if token != provided {
        return Err("This reviewed service quote is stale. Review the latest ownership, company capital, facility and fiscal state.".into());
    }
    Ok(())
}
fn start_company_refit(
    w: &mut WorldState,
    n: NationId,
    company: u32,
    source: &str,
    product: u32,
    quantity: u32,
    reviewed: &str,
) -> Result<(), String> {
    let q = refit_quote(w, n, company, source, product, quantity);
    accept_service(q.valid, q.reason.clone(), &q.token, reviewed)?;
    charge(w, n, 3, q.cost_bn)?;
    crate::arsenal::reserve_refit(w.nation_mut(n), source, quantity)?;
    let id = next_id(w);
    let invoice = next_id(w);
    let day = clock::absolute_day(w);
    w.companies.version = VERSION;
    let c = w
        .companies
        .firms
        .iter_mut()
        .find(|c| c.id == company && c.nation == n)
        .unwrap();
    c.refits.push(RefitContract {id,product,source_revision:source.into(),target_revision:q.target_revision.clone(),quantity,completed_units:0,cancelled_units:0,unit_days:q.unit_days,unit_fabrication_cost_bn:q.unit_fabrication_cost_bn,unit_material_quote_bn:q.unit_material_cost_bn,unit_price_bn:q.unit_price_bn,total_price_bn:q.cost_bn,recipe_per_unit:q.recipe_per_unit,advance_received_bn:0.0,escrow_bn:0.0,earned_revenue_bn:0.0,refunded_bn:0.0,working_capital_locked_bn:0.0,unit_started_day:None,unit_work_days:0.0,unit_inputs:[0.0;12],unit_material_cost_bn:0.0,materials_expense_bn:0.0,fabrication_expense_bn:0.0,resources_used:[0.0;12],booked_day:day,settled_day:None,cancelled_day:None,refund_day:None,closed_day:None,last_work_day:None,status:"awaiting_settlement".into(),reason:"Government source units are reserved; the service deposit must settle into locked escrow before work begins.".into()});
    c.receivables.push(Receivable {
        id: invoice,
        day,
        kind: "refit_advance".into(),
        amount_bn: q.cost_bn,
        product: Some(product),
        delivery: None,
        refit: Some(id),
    });
    w.nation_mut(n)
        .equipment
        .as_mut()
        .unwrap()
        .company_refits
        .insert(
            id,
            equipment::CompanyRefitClaim {
                company,
                source_revision: source.into(),
                target_revision: q.target_revision,
                quantity,
            },
        );
    Ok(())
}
fn sync_refit_claim(w: &mut WorldState, i: usize, j: usize) {
    let c = &w.companies.firms[i];
    let n = c.nation;
    let p = &c.refits[j];
    let id = p.id;
    let remaining = refit_remaining(p);
    let s = w.nation_mut(n).equipment.as_mut().unwrap();
    if remaining == 0 {
        s.company_refits.remove(&id);
    } else if let Some(claim) = s.company_refits.get_mut(&id) {
        claim.quantity = remaining;
    }
}
fn refund_cancelled_refit(w: &mut WorldState, i: usize, j: usize, day: i32) {
    let day = day.max(clock::absolute_day(w));
    let c = &w.companies.firms[i];
    let n = c.nation;
    let p = &c.refits[j];
    if p.settled_day.is_none() {
        return;
    }
    let expected = p.unit_price_bn * p.cancelled_units as f64;
    let refund = (expected - p.refunded_bn).max(0.0).min(p.escrow_bn);
    if refund <= 0.0 {
        return;
    }
    let gdp = w.nation(n).gdp.max(0.1);
    crate::economy::charge(w, n, -refund, -refund / gdp);
    let c = &mut w.companies.firms[i];
    c.refit_refunds_bn += refund;
    let p = &mut c.refits[j];
    p.escrow_bn = (p.escrow_bn - refund).max(0.0);
    p.refunded_bn += refund;
    p.refund_day = Some(day);
    let product = p.product;
    let quantity = p.cancelled_units;
    transaction(c, day, "refit_refund", refund, Some(product), quantity);
}
fn cancel_company_refit(
    w: &mut WorldState,
    n: NationId,
    company: u32,
    id: u32,
    reviewed: &str,
) -> Result<(), String> {
    let q = refit_cancel_quote(w, n, company, id);
    accept_service(q.valid, q.reason.clone(), &q.token, reviewed)?;
    let (i, j) = refit_indexes(w, n, company, id)?;
    let source = w.companies.firms[i].refits[j].source_revision.clone();
    crate::arsenal::release_refit(w.nation_mut(n), &source, q.refundable_units)?;
    let day = clock::absolute_day(w);
    let p = &mut w.companies.firms[i].refits[j];
    p.cancelled_units += q.refundable_units;
    p.cancelled_day = Some(day);
    if refit_remaining(p) == 0 {
        p.closed_day = Some(day);
    }
    p.status = if refit_active(p) > 0 {
        "finishing_started_unit"
    } else if p.settled_day.is_none() {
        "awaiting_refund_settlement"
    } else {
        "cancelled"
    }
    .into();
    p.reason="Unstarted units were released. Their fixed fees are refundable after original fiscal settlement; any started conversion retains its labor reserve and must finish.".into();
    sync_refit_claim(w, i, j);
    refund_cancelled_refit(w, i, j, day);
    Ok(())
}
fn settle_refit_advance(w: &mut WorldState, i: usize, r: &Receivable, day: i32) {
    let Some(j) = w.companies.firms[i]
        .refits
        .iter()
        .position(|p| Some(p.id) == r.refit && p.settled_day.is_none())
    else {
        return;
    };
    let c = &mut w.companies.firms[i];
    c.refit_advances_received_bn += r.amount_bn;
    let p = &mut c.refits[j];
    p.advance_received_bn = r.amount_bn;
    p.escrow_bn = r.amount_bn;
    p.settled_day = Some(day);
    p.status = if refit_remaining(p) > 0 {
        "queued"
    } else {
        "cancelled"
    }
    .into();
    p.reason="The service deposit is held in locked escrow. The company must fund the real conversion from its own working capital.".into();
    transaction(c, day, "refit_advance", r.amount_bn, r.product, 0);
    c.receivables.retain(|x| x.id != r.id);
    refund_cancelled_refit(w, i, j, day);
}

fn update_refit_queue(w: &mut WorldState, i: usize, selected: Option<u32>) {
    for p in &mut w.companies.firms[i].refits {
        if refit_remaining(p) == 0 || Some(p.id) == selected {
            continue;
        }
        p.status = "queued".into();
        p.reason="Waiting for active development or an earlier refit contract on the same leased plant slot. Reserved source units remain national property.".into();
    }
}
fn block_refit(w: &mut WorldState, i: usize, j: usize, reason: String) {
    let p = &mut w.companies.firms[i].refits[j];
    p.status = "blocked".into();
    p.reason = reason;
}
fn tick_company_refit(w: &mut WorldState, i: usize, j: usize, day: i32, facility: Option<String>) {
    let c = w.companies.firms[i].clone();
    let p = c.refits[j].clone();
    if refit_remaining(&p) == 0 || p.last_work_day.is_some_and(|d| d >= day) || day <= p.booked_day
    {
        return;
    }
    if p.settled_day.is_none_or(|d| day <= d) {
        return;
    }
    if let Some(reason) = facility {
        block_refit(w, i, j, reason);
        return;
    }
    if !w
        .nation(c.nation)
        .equipment
        .as_ref()
        .and_then(|s| s.company_refits.get(&p.id))
        .is_some_and(|claim| claim.quantity == refit_remaining(&p))
    {
        block_refit(w,i,j,"The government reservation does not match this service. Property and held funds remain recorded.".into());
        return;
    }
    if p.unit_started_day.is_none() {
        let raw_cost = material_cost(w, &p.recipe_per_unit);
        let needed = raw_cost + p.unit_fabrication_cost_bn;
        if c.cash_bn < needed {
            block_refit(w,i,j,"Company working capital must cover the next unit's real inputs and reserve its entire conversion labor. Public service escrow cannot finance this work.".into());
            return;
        }
        if let Err((r, _, _)) = resources::consume_stockpile_atomic(w, c.nation, &p.recipe_per_unit)
        {
            block_refit(
                w,
                i,
                j,
                format!(
                    "The domestic warehouse lacks {} for the complete conversion recipe.",
                    r.name()
                ),
            );
            return;
        }
        let gdp = w.nation(c.nation).gdp.max(0.1);
        crate::economy::charge(w, c.nation, -raw_cost, -raw_cost / gdp);
        let c = &mut w.companies.firms[i];
        c.cash_bn -= needed;
        c.materials_expense_bn += raw_cost;
        transaction(c, day, "refit_materials", raw_cost, Some(p.product), 1);
        transaction(
            c,
            day,
            "refit_labor_reserved",
            p.unit_fabrication_cost_bn,
            Some(p.product),
            1,
        );
        let p = &mut c.refits[j];
        p.unit_started_day = Some(day);
        p.unit_inputs = p.recipe_per_unit;
        p.unit_material_cost_bn = raw_cost;
        p.materials_expense_bn += raw_cost;
        p.working_capital_locked_bn = p.unit_fabrication_cost_bn;
        for k in 0..12 {
            p.resources_used[k] += p.recipe_per_unit[k];
        }
    }
    let p = w.companies.firms[i].refits[j].clone();
    let next = (p.unit_work_days + 1.0).min(p.unit_days as f64);
    let finishing = next >= p.unit_days as f64;
    let payment = if finishing {
        p.working_capital_locked_bn
    } else {
        (p.unit_fabrication_cost_bn / p.unit_days.max(1) as f64).min(p.working_capital_locked_bn)
    };
    if finishing {
        if let Err(reason) = crate::arsenal::complete_refit(
            w.nation_mut(c.nation),
            &p.source_revision,
            &p.target_revision,
            1,
        ) {
            block_refit(w, i, j, reason);
            return;
        }
    }
    let c = &mut w.companies.firms[i];
    c.fabrication_expense_bn += payment;
    transaction(
        c,
        day,
        "refit_conversion",
        payment,
        Some(p.product),
        u32::from(finishing),
    );
    let p = &mut c.refits[j];
    p.working_capital_locked_bn = (p.working_capital_locked_bn - payment).max(0.0);
    p.fabrication_expense_bn += payment;
    p.unit_work_days = next;
    p.last_work_day = Some(day);
    p.status = "refitting".into();
    p.reason="The company is converting one reserved unit using its paid inputs and locked labor capital. The fixed fee remains unearned public escrow until return.".into();
    if finishing {
        let fee = if refit_remaining(p) == 1 {
            p.escrow_bn
        } else {
            p.unit_price_bn
        };
        p.completed_units += 1;
        p.escrow_bn = (p.escrow_bn - fee).max(0.0);
        p.earned_revenue_bn += fee;
        p.unit_work_days = 0.0;
        p.unit_started_day = None;
        p.unit_inputs = [0.0; 12];
        p.unit_material_cost_bn = 0.0;
        p.working_capital_locked_bn = 0.0;
        let finished = refit_remaining(p) == 0;
        if finished {
            p.closed_day = Some(day);
        }
        p.status = if finished { "complete" } else { "queued" }.into();
        p.reason="A whole conversion returned to the government Arsenal at its existing cohort age. Its fixed service fee was released once; no new vehicle or ammunition was created.".into();
        let product = p.product;
        c.cash_bn += fee;
        c.refit_revenue_bn += fee;
        transaction(c, day, "refit_fee_earned", fee, Some(product), 1);
        sync_refit_claim(w, i, j);
    }
}
/// Counts already owned units conditionally changing revision, not new stock.
pub fn refit_target_flow(w: &WorldState, n: NationId, revision: &str) -> (u64, u64, u64, u64) {
    let mut incoming = 0;
    let mut outgoing = 0;
    let mut blocked_in = 0;
    let mut blocked_out = 0;
    for c in w.companies.firms.iter().filter(|c| c.nation == n) {
        for p in &c.refits {
            let count = refit_remaining(p) as u64;
            let stalled = p.status == "blocked"
                || p.settled_day.is_none()
                || facility_blocker(w, c).is_some();
            if p.target_revision == revision {
                incoming += count;
                if stalled {
                    blocked_in += count;
                }
            }
            if p.source_revision == revision {
                outgoing += count;
                if stalled {
                    blocked_out += count;
                }
            }
        }
    }
    (incoming, outgoing, blocked_in, blocked_out)
}
fn refit_view(w: &WorldState, c: &Company, p: &RefitContract) -> serde_json::Value {
    let mut v = serde_json::to_value(p).unwrap();
    let remaining = refit_remaining(p);
    let unstarted = refit_refundable(p);
    let active = refit_active(p);
    let work = (p.unit_days as f64 * remaining as f64 - p.unit_work_days).max(0.0);
    let next_cost = material_cost(w, &p.recipe_per_unit) + p.unit_fabrication_cost_bn;
    let future_capital = if unstarted == 0 {
        0.0
    } else {
        next_cost + (next_cost - p.unit_price_bn).max(0.0) * unstarted.saturating_sub(1) as f64
    };
    let future_cash = c.cash_bn + if active > 0 { p.unit_price_bn } else { 0.0 };
    let raw_ready = resources::ALL.iter().all(|r| {
        resources::stockpile(w, c.nation, *r) >= p.recipe_per_unit[r.idx()] * unstarted as f64
    });
    let known = remaining > 0
        && scheduled_product(c) == Some(p.id)
        && p.settled_day.is_some()
        && facility_blocker(w, c).is_none();
    let eta = if known && future_cash >= future_capital && raw_ready {
        Some(work.ceil() as u32)
    } else {
        None
    };
    let first = if known
        && (active > 0
            || c.cash_bn >= next_cost
                && resources::ALL
                    .iter()
                    .all(|r| resources::stockpile(w, c.nation, *r) >= p.recipe_per_unit[r.idx()]))
    {
        Some((p.unit_days as f64 - p.unit_work_days).ceil() as u32)
    } else {
        None
    };
    v["company"] = serde_json::json!(c.id);
    v["remaining_units"] = serde_json::json!(remaining);
    v["refundable_units"] = serde_json::json!(unstarted);
    v["active_units"] = serde_json::json!(active);
    v["refund_bn"] = serde_json::json!(unstarted as f64 * p.unit_price_bn);
    v["refund_pending_bn"] =
        serde_json::json!((p.cancelled_units as f64 * p.unit_price_bn - p.refunded_bn).max(0.0));
    v["eta_days"] = serde_json::json!(eta);
    v["first_return_days"] = serde_json::json!(first);
    v["remaining_work_days"] = serde_json::json!(work);
    v["company_cash_needed_bn"] = serde_json::json!(if active > 0 {
        0.0
    } else if remaining > 0 {
        next_cost
    } else {
        0.0
    });
    if let Some(s) = w.nation(c.nation).equipment.as_ref() {
        if let Some(r) = s.revisions.get(&p.target_revision) {
            v["target_name"] = serde_json::json!(r.name);
            v["target_spec"] = serde_json::json!(r.spec);
            v["family"] = serde_json::json!(product_family(&r.spec.platform));
            v["platform"] = serde_json::json!(r.spec.platform);
            v["platform_name"] = serde_json::json!(platform_name(&r.spec.platform));
            v["unit_label"] = serde_json::json!(unit_label(&r.spec.platform));
        }
        if let Some(r) = s.revisions.get(&p.source_revision) {
            v["source_name"] = serde_json::json!(r.name);
            v["source_spec"] = serde_json::json!(r.spec);
        }
    }
    v["estimate_note"]=serde_json::json!("Refits share the manufacturer's single leased slot. Active development takes priority; refits precede finite stock. Estimates require continuing facility and input access. Public escrow is earned only on whole-unit return; separately locked company labor cannot finance other work.");
    v
}
fn refits_view(w: &WorldState, n: NationId) -> Vec<serde_json::Value> {
    w.companies
        .firms
        .iter()
        .filter(|c| c.nation == n)
        .flat_map(|c| c.refits.iter().map(move |p| refit_view(w, c, p)))
        .collect()
}

fn validate_company_refits(w: &WorldState, ids: &mut BTreeSet<u32>) -> Result<(), String> {
    let fail = || {
        "Invalid company refit ownership, fixed terms, escrow, refund or physical work.".to_string()
    };
    let day = clock::absolute_day(w);
    let near = |a: f64, b: f64| (a - b).abs() <= 1e-12 + 1e-10 * a.abs().max(b.abs());
    let finite = |v: f64| v.is_finite() && v >= 0.0;
    let has_refits = w.companies.firms.iter().any(|c| !c.refits.is_empty());
    let has_claims = w.nations.iter().any(|n| {
        n.equipment
            .as_ref()
            .is_some_and(|s| !s.company_refits.is_empty())
    });
    if w.companies.version < VERSION && (has_refits || has_claims)
        || w.companies.version == VERSION && !has_refits
    {
        return Err(fail());
    }
    for c in &w.companies.firms {
        if c.refits.len() > MAX_REFITS
            || c.refits.iter().filter(|p| refit_remaining(p) > 0).count() > MAX_ACTIVE_REFITS
        {
            return Err(fail());
        }
        for p in &c.refits {
            let (target, labor, days, recipe) =
                service_terms(w, c.nation, c, &p.source_revision, p.product).map_err(|_| fail())?;
            let remaining = refit_remaining(p);
            let active = refit_active(p);
            let processed = p.completed_units as f64 + p.unit_work_days / days as f64;
            let known = w.nation(c.nation).equipment.as_ref().unwrap();
            if p.id == 0
                || !ids.insert(p.id)
                || p.target_revision != target
                || !(1..=equipment::MAX_BATCH).contains(&p.quantity)
                || p.completed_units
                    .checked_add(p.cancelled_units)
                    .is_none_or(|v| v > p.quantity)
                || p.unit_days != days
                || p.recipe_per_unit != recipe
                || !near(p.unit_fabrication_cost_bn, labor)
                || ![
                    p.unit_fabrication_cost_bn,
                    p.unit_material_quote_bn,
                    p.unit_price_bn,
                    p.total_price_bn,
                    p.advance_received_bn,
                    p.escrow_bn,
                    p.earned_revenue_bn,
                    p.refunded_bn,
                    p.working_capital_locked_bn,
                    p.unit_work_days,
                    p.unit_material_cost_bn,
                    p.materials_expense_bn,
                    p.fabrication_expense_bn,
                ]
                .into_iter()
                .all(finite)
                || !p
                    .unit_inputs
                    .into_iter()
                    .chain(p.resources_used)
                    .all(finite)
                || p.unit_price_bn <= 0.0
                || p.total_price_bn > 1000.0
                || !near(
                    p.unit_price_bn,
                    (labor + p.unit_material_quote_bn) * (1.0 + MARGIN),
                )
                || !near(p.total_price_bn, p.unit_price_bn * p.quantity as f64)
                || p.booked_day < c.established_day
                || p.booked_day > day
                || [&p.source_revision, &p.target_revision]
                    .into_iter()
                    .any(|id| {
                        known.revisions[id]
                            .certified_day
                            .is_none_or(|d| d > p.booked_day)
                    })
                || p.settled_day.is_some_and(|d| d < p.booked_day || d > day)
                || p.cancelled_day.is_some_and(|d| d < p.booked_day || d > day)
                || (p.cancelled_units > 0) != p.cancelled_day.is_some()
                || p.closed_day.is_some_and(|d| d < p.booked_day || d > day)
                || (remaining == 0) != p.closed_day.is_some()
                || p.last_work_day
                    .is_some_and(|d| d > day || p.settled_day.is_none_or(|settled| d <= settled))
                || (p.completed_units > 0 || active > 0) != p.last_work_day.is_some()
                || p.unit_started_day.is_some_and(|d| {
                    p.settled_day.is_none_or(|s| d <= s)
                        || p.last_work_day.is_none_or(|last| d > last)
                })
                || p.unit_work_days.fract() != 0.0
                || p.unit_work_days >= days as f64
                || active > 0
                    && (remaining == 0
                        || p.unit_work_days == 0.0
                        || p.unit_inputs != recipe
                        || p.unit_material_cost_bn <= 0.0)
                || active == 0
                    && (p.unit_work_days != 0.0
                        || p.unit_inputs != [0.0; 12]
                        || p.unit_material_cost_bn != 0.0
                        || p.working_capital_locked_bn != 0.0)
                || !near(
                    p.working_capital_locked_bn,
                    if active > 0 {
                        labor * (1.0 - p.unit_work_days / days as f64)
                    } else {
                        0.0
                    },
                )
                || !near(p.fabrication_expense_bn, labor * processed)
                || p.materials_expense_bn + 1e-12 < p.unit_material_cost_bn
                || p.completed_units + active > 0 && p.materials_expense_bn <= 0.0
                || p.completed_units + active == 0 && p.materials_expense_bn != 0.0
                || p.resources_used
                    .iter()
                    .zip(recipe)
                    .any(|(used, per)| !near(*used, per * (p.completed_units + active) as f64))
                || p.last_work_day
                    .zip(p.settled_day)
                    .is_some_and(|(last, settled)| {
                        processed * days as f64 > (last - settled) as f64 + 1e-8
                    })
                || !near(
                    p.earned_revenue_bn,
                    p.unit_price_bn * p.completed_units as f64,
                )
                || !near(
                    p.advance_received_bn,
                    if p.settled_day.is_some() {
                        p.total_price_bn
                    } else {
                        0.0
                    },
                )
                || !near(
                    p.escrow_bn + p.earned_revenue_bn + p.refunded_bn,
                    p.advance_received_bn,
                )
                || !near(
                    p.refunded_bn,
                    if p.settled_day.is_some() {
                        p.cancelled_units as f64 * p.unit_price_bn
                    } else {
                        0.0
                    },
                )
                || (p.refunded_bn > 0.0) != p.refund_day.is_some()
                || p.refund_day.is_some_and(|d| {
                    d > day
                        || p.settled_day.is_none_or(|s| d < s)
                        || p.cancelled_day.is_none_or(|s| d < s)
                })
                || p.settled_day.is_none()
                    && (p.completed_units > 0
                        || active > 0
                        || p.materials_expense_bn > 0.0
                        || p.fabrication_expense_bn > 0.0)
                || p.closed_day
                    .zip(p.last_work_day)
                    .is_some_and(|(closed, last)| closed < last)
                || p.closed_day
                    .zip(p.cancelled_day)
                    .is_some_and(|(closed, cancelled)| closed < cancelled)
            {
                return Err(fail());
            }
            let claims = known.company_refits.get(&p.id);
            if remaining > 0 {
                if !claims.is_some_and(|claim| {
                    claim.company == c.id
                        && claim.source_revision == p.source_revision
                        && claim.target_revision == p.target_revision
                        && claim.quantity == remaining
                }) {
                    return Err(fail());
                }
            } else if claims.is_some() {
                return Err(fail());
            }
            let invoices = c
                .receivables
                .iter()
                .filter(|r| r.kind == "refit_advance" && r.refit == Some(p.id))
                .count();
            if invoices != usize::from(p.settled_day.is_none()) {
                return Err(fail());
            }
        }
        if !near(
            c.refit_advances_received_bn,
            c.refits.iter().map(|p| p.advance_received_bn).sum(),
        ) || !near(
            c.refit_revenue_bn,
            c.refits.iter().map(|p| p.earned_revenue_bn).sum(),
        ) || !near(
            c.refit_refunds_bn,
            c.refits.iter().map(|p| p.refunded_bn).sum(),
        ) {
            return Err(fail());
        }
        if !c.refits.is_empty() {
            let equipment_labor = c
                .products
                .iter()
                .map(|p| {
                    w.nation(c.nation).equipment.as_ref().unwrap().revisions[&p.revision_id]
                        .profile
                        .fabrication_cost_bn
                        * p.produced_units as f64
                        + (p.unit_spent_bn - p.unit_material_cost_bn).max(0.0)
                })
                .sum::<f64>();
            let ammunition_labor = c
                .ammunition_products
                .iter()
                .map(|p| p.fabrication_expense_bn)
                .sum::<f64>();
            if !near(
                c.fabrication_expense_bn,
                equipment_labor + ammunition_labor + refit_fabrication_cost(c),
            ) {
                return Err(fail());
            }
        }
    }
    let mut claimed = BTreeSet::new();
    for n in &w.nations {
        if let Some(s) = &n.equipment {
            for (id, claim) in &s.company_refits {
                let c = w
                    .companies
                    .firms
                    .iter()
                    .find(|c| c.id == claim.company && c.nation == n.id)
                    .ok_or_else(fail)?;
                let p = c.refits.iter().find(|p| p.id == *id).ok_or_else(fail)?;
                if !claimed.insert(*id)
                    || refit_remaining(p) == 0
                    || claim.quantity != refit_remaining(p)
                    || claim.source_revision != p.source_revision
                    || claim.target_revision != p.target_revision
                {
                    return Err(fail());
                }
            }
        }
    }
    Ok(())
}
