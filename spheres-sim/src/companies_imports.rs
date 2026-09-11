// Foreign purchases use a separate buyer-funded escrow and frozen origin.
// No firm, inventory, research or factory is created by browsing or adoption.
pub const IMPORT_DAYS: u32 = 14;
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ImportBook {
    pub buyers: Vec<ImportBuyer>,
    pub contracts: Vec<ImportContract>,
}
impl ImportBook {
    pub fn is_empty(&self) -> bool {
        self.buyers.is_empty() && self.contracts.is_empty()
    }
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ImportBuyer {
    pub nation: NationId,
    pub adopted_day: i32,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ImportContract {
    pub id: u32,
    pub seller: NationId,
    pub buyer: NationId,
    pub company: u32,
    pub product: u32,
    pub ammunition: bool,
    pub quantity: u32,
    pub district: String,
    pub source_revision: equipment::DesignRevision,
    pub buyer_revision: Option<String>,
    pub family: Option<String>,
    pub route: crate::logistics::RoutePlan,
    pub transit_days: u32,
    pub unit_price_bn: f64,
    pub total_price_bn: f64,
    pub cost_basis_bn: f64,
    pub purchased_day: i32,
    pub settled_day: Option<i32>,
    pub due_day: Option<i32>,
    pub delivered_day: Option<i32>,
    pub cancelled_day: Option<i32>,
    pub refunded_day: Option<i32>,
    pub escrow_bn: f64,
    pub refunded_bn: f64,
    pub status: String,
    pub reason: String,
}
#[derive(Clone, Debug, Serialize)]
pub struct ImportOffer {
    pub seller: NationId,
    pub company: u32,
    pub product: u32,
    pub ammunition: bool,
    pub supplier_name: String,
    pub product_name: String,
    pub platform: Option<String>,
    pub family: Option<String>,
    pub spec: Option<equipment::DesignSpec>,
    pub ready_stock: u32,
    pub unit_price_bn: f64,
    pub purchase_available_bn: f64,
    pub protected_maintenance_bn: f64,
    pub affordable_quantity: u32,
    pub review_quantity: u32,
    pub delivery_days: Option<u32>,
    pub allowed: bool,
    pub reason: Option<String>,
}
#[derive(Clone, Debug, Serialize)]
pub struct ImportCancelQuote {
    pub valid: bool,
    pub reason: Option<String>,
    pub token: String,
    pub refund_bn: f64,
    pub refund_after_settlement: bool,
    pub note: String,
}
pub fn imports_enabled(w: &WorldState, n: NationId) -> bool {
    w.companies.imports.buyers.iter().any(|b| b.nation == n)
}
pub fn imported_revision(w: &WorldState, n: NationId, id: &str) -> bool {
    w.companies.imports.contracts.iter().any(|d| {
        d.buyer == n && d.delivered_day.is_some() && d.buyer_revision.as_deref() == Some(id)
    })
}
pub fn import_enrollment_quote(w: &WorldState, n: NationId) -> Quote {
    let mut q = Quote::default();
    q.reason = actor(w, n)
        .or_else(|| (!w.rules.production_system).then(||
            "Reviewed imports require financial construction for their paid supplier programmes.".into()))
        .or_else(|| crate::supplier_catalogue::validate(w).err())
        .or_else(|| {
            imports_enabled(w, n)
                .then(|| "Reviewed imports are already enabled for this government.".into())
        });
    q.note="Hold future unassigned Defense procurement authority for reviewed purchases, without changing allocations or granting money, factories, stock or component research. Existing paid domestic orders continue. Modeled foreign supplier programmes can start through their governments' ordinary paid construction and development; first stock requires actual funded work.".into();
    finish_quote(w, n, serde_json::json!(["enable_imports"]), q)
}
fn enable_imports(w: &mut WorldState, n: NationId, provided: &str) -> Result<(), String> {
    accept(import_enrollment_quote(w, n), provided)?;
    let day = clock::absolute_day(w);
    equipment::activate(w.nation_mut(n), day);
    w.companies.imports.buyers.push(ImportBuyer {
        nation: n,
        adopted_day: day,
    });
    w.companies.imports.buyers.sort_by_key(|b| b.nation);
    w.companies.version = IMPORT_VERSION;
    crate::supplier_catalogue::enable(w)?;
    Ok(())
}
fn import_source<'a>(
    w: &'a WorldState,
    seller: NationId,
    cid: u32,
    pid: u32,
    ammo: bool,
) -> Result<
    (
        &'a Company,
        &'a equipment::DesignRevision,
        u32,
        f64,
        Option<String>,
    ),
    String,
> {
    let c = company(w, seller, cid).ok_or("The named seller company is missing.")?;
    let (revision, stock, basis, family) = if ammo {
        let p = c
            .ammunition_products
            .iter()
            .find(|p| p.id == pid)
            .ok_or("The seller ammunition product is missing.")?;
        (
            &p.source_revision,
            p.stock,
            p.stock_cost_bn,
            Some(p.family.clone()),
        )
    } else {
        let p = c
            .products
            .iter()
            .find(|p| p.id == pid && p.cancelled_day.is_none())
            .ok_or("The seller's active product is missing.")?;
        (&p.revision_id, p.stock, p.stock_cost_bn, None)
    };
    let r = w
        .nation(seller)
        .equipment
        .as_ref()
        .and_then(|s| s.revisions.get(revision))
        .ok_or("The seller's frozen revision is missing.")?;
    if r.certified_day
        .is_none_or(|day| day > clock::absolute_day(w))
    {
        return Err("The seller has not completed development and certification.".into());
    }
    Ok((c, r, stock, basis, family))
}
fn import_access(
    w: &WorldState,
    buyer: NationId,
    c: &Company,
) -> Result<crate::logistics::RoutePlan, String> {
    if buyer == c.nation {
        return Err("Use the domestic stock purchase for your own country's supplier.".into());
    }
    if w.districts.get(&c.district) != Some(&c.nation) {
        return Err("The seller's shipping province is outside its government's control.".into());
    }
    if let Some(reason) = crate::control::blocker(w, c.nation, &c.district) {
        return Err(reason);
    }
    crate::logistics::plan(w, c.nation, buyer)
}
fn import_terms(
    w: &WorldState,
    buyer: NationId,
    seller: NationId,
    cid: u32,
    pid: u32,
    ammo: bool,
    quantity: u32,
) -> (Quote, Option<crate::logistics::RoutePlan>) {
    let mut q = Quote::default();
    q.reason = (w.companies.next_id >= u32::MAX - 8)
        .then(|| "The company transaction archive is full.".into())
        .or_else(|| actor(w, buyer))
        .or_else(|| {
            (!imports_enabled(w, buyer)).then(|| {
                "Enable reviewed imports to hold future procurement funding before ordering.".into()
            })
        })
        .or_else(|| {
            (w.companies.imports.contracts.len() >= 100_000)
                .then(|| "The import contract archive is full.".into())
        });
    let mut route = None;
    match import_source(w, seller, cid, pid, ammo) {
        Err(reason) => q.reason = q.reason.or(Some(reason)),
        Ok((c, r, stock, basis, family)) => {
            q.available_units = stock;
            q.unit_price_bn = if stock > 0 {
                basis / stock as f64 * (1.0 + MARGIN)
            } else {
                0.0
            };
            q.cost_bn = q.unit_price_bn * quantity as f64;
            let funding = if ammo {
                ammo_purchase_funding(w, buyer)
            } else {
                (0.0, programs::available_bn(w, buyer, BUDGET_DEFENSE, 3))
            };
            q.protected_maintenance_bn = funding.0;
            q.purchase_available_bn = funding.1;
            let max = if ammo { MAX_AMMO_STOCK } else { MAX_STOCK };
            q.reason=q.reason.or_else(||(!(1..=max).contains(&quantity)).then(||format!("Choose 1–{max} whole units.")))
                .or_else(||(stock<quantity).then(||"The requested quantity exceeds the seller's finished stock; this creates no production order.".into()));
            if ammo {
                q.reason=q.reason.or_else(||equipment::ammo_actor_refusal(w,buyer)).or_else(||(!equipment::maintenance_plan_on(w.nation(buyer),clock::absolute_day(w))).then(||"Wait for an actual fleet maintenance plan to take effect before buying ammunition.".into())).or_else(||family.as_deref().and_then(|f|ammo_source(w,buyer,f).is_none().then(||"Receive or certify a compatible weapon model before importing this ammunition family.".into())));
            } else {
                q.maintenance_bn_day = r.profile.maintenance_bn_day * quantity as f64;
                let id = format!("import-{cid}-{pid}");
                if let Some(s) = &w.nation(buyer).equipment {
                    q.reason = q.reason.or_else(|| {
                        (!s.revisions.contains_key(&id)
                            && s.revisions.len() >= equipment::MAX_REVISIONS)
                            .then(|| "The buyer's equipment revision library is full.".into())
                    });
                    if let Some(existing) = s.revisions.get(&id) {
                        let mut expected = r.clone();
                        expected.id = id;
                        q.reason=q.reason.or_else(||(existing!=&expected).then(||"The destination revision identity is already occupied by another model.".into()));
                    }
                }
            }
            q.reason=q.reason.or_else(||(q.cost_bn>funding.1).then(||"The buyer's available Defense authority cannot fund this lot after protected maintenance.".into()));
            match import_access(w, buyer, c) {
                Ok(p) => {
                    let days = p.estimated_days.max(IMPORT_DAYS);
                    q.minimum_days = days;
                    q.eta_days = Some(days);
                    route = Some(p);
                }
                Err(reason) => q.reason = q.reason.or(Some(reason)),
            }
        }
    }
    q.note="Buys existing foreign company stock into buyer-owned transit under a fixed price. The buyer's Defense payment settles once into protected import escrow; the seller earns it only when the exact stock arrives. Transit requires continuing route and shipping-province access. Cancel an undelivered lot when the seller has room to receive it: stock and its original cost basis return once, and the settled buyer payment refunds once without recreating authority. Imports grant use and ordinary maintenance rights, never component research or domestic manufacturing rights. Timings and 15% margin are game assumptions.".into();
    q.valid = q.reason.is_none();
    (q, route)
}
pub fn import_purchase_quote(
    w: &WorldState,
    buyer: NationId,
    seller: NationId,
    cid: u32,
    pid: u32,
    ammo: bool,
    quantity: u32,
) -> Quote {
    let (mut q, route) = import_terms(w, buyer, seller, cid, pid, ammo, quantity);
    q.token = token(
        w,
        buyer,
        serde_json::json!(["import", seller, cid, pid, ammo, quantity, &q, &route]),
    );
    q
}
pub fn import_offers(w: &WorldState, buyer: NationId) -> Vec<ImportOffer> {
    let mut out = Vec::new();
    for c in w.companies.firms.iter().filter(|c| c.nation != buyer) {
        for (pid, ammo) in c
            .products
            .iter()
            .map(|p| (p.id, false))
            .chain(c.ammunition_products.iter().map(|p| (p.id, true)))
        {
            let Ok((_, r, stock, _, family)) = import_source(w, c.nation, c.id, pid, ammo) else {
                continue;
            };
            let (q, _) = import_terms(w, buyer, c.nation, c.id, pid, ammo, 1);
            let mut affordable = if q.unit_price_bn > 0.0 && q.unit_price_bn.is_finite() {
                (q.purchase_available_bn / q.unit_price_bn)
                    .floor()
                    .max(0.0)
                    .min(stock as f64)
                    .min(if ammo { MAX_AMMO_STOCK } else { MAX_STOCK } as f64)
                    as u32
            } else {
                0
            };
            // Division may round up at an exact funding boundary; the purchase
            // owner compares the multiplied cost, so the offer must do the same.
            while affordable > 0 && q.unit_price_bn * affordable as f64 > q.purchase_available_bn {
                affordable -= 1;
            }
            out.push(ImportOffer {
                seller: c.nation,
                company: c.id,
                product: pid,
                ammunition: ammo,
                supplier_name: c.name.clone(),
                product_name: if ammo {
                    family.clone().unwrap()
                } else {
                    r.name.clone()
                },
                platform: (!ammo).then(|| r.spec.platform.clone()),
                family,
                spec: (!ammo).then(|| r.spec.clone()),
                ready_stock: stock,
                unit_price_bn: q.unit_price_bn,
                purchase_available_bn: q.purchase_available_bn,
                protected_maintenance_bn: q.protected_maintenance_bn,
                affordable_quantity: affordable,
                review_quantity: if stock > 0 { 1 } else { 0 },
                delivery_days: q.eta_days,
                allowed: q.valid,
                reason: q.reason,
            });
        }
    }
    out
}
fn purchase_import(
    w: &mut WorldState,
    buyer: NationId,
    seller: NationId,
    cid: u32,
    pid: u32,
    ammo: bool,
    quantity: u32,
    provided: &str,
) -> Result<(), String> {
    let q = accept(
        import_purchase_quote(w, buyer, seller, cid, pid, ammo, quantity),
        provided,
    )?;
    let (c, r, stock, basis, family) = import_source(w, seller, cid, pid, ammo)?;
    let district = c.district.clone();
    let source_revision = r.clone();
    let route = import_access(w, buyer, c)?;
    let cost_basis_bn = basis / stock as f64 * quantity as f64;
    charge(w, buyer, if ammo { 2 } else { 3 }, q.cost_bn)?;
    let id = next_id(w);
    let day = clock::absolute_day(w);
    let c = w.companies.firms.iter_mut().find(|c| c.id == cid).unwrap();
    if ammo {
        let p = c
            .ammunition_products
            .iter_mut()
            .find(|p| p.id == pid)
            .unwrap();
        p.stock -= quantity;
        p.sold_units += quantity;
        p.stock_cost_bn = (p.stock_cost_bn - cost_basis_bn).max(0.0);
    } else {
        let p = c.products.iter_mut().find(|p| p.id == pid).unwrap();
        p.stock -= quantity;
        p.sold_units += quantity;
        p.stock_cost_bn = (p.stock_cost_bn - cost_basis_bn).max(0.0);
    }
    w.companies.imports.contracts.push(ImportContract{id,seller,buyer,company:cid,product:pid,ammunition:ammo,quantity,district,source_revision,buyer_revision:(!ammo).then(||format!("import-{cid}-{pid}")),family,route,transit_days:q.minimum_days,unit_price_bn:q.unit_price_bn,total_price_bn:q.cost_bn,cost_basis_bn,purchased_day:day,settled_day:None,due_day:None,delivered_day:None,cancelled_day:None,refunded_day:None,escrow_bn:0.0,refunded_bn:0.0,status:"awaiting_settlement".into(),reason:"The buyer owns this reserved lot; the original Defense payment must settle before transit starts.".into()});
    Ok(())
}
pub fn import_cancel_quote(w: &WorldState, buyer: NationId, id: u32) -> ImportCancelQuote {
    let d = w
        .companies
        .imports
        .contracts
        .iter()
        .find(|d| d.id == id && d.buyer == buyer);
    let mut reason = actor(w, buyer).or_else(|| {
        d.is_none()
            .then(|| "This buyer's import contract is missing.".into())
    });
    if let Some(d) = d {
        reason = reason.or_else(|| {
            (d.delivered_day.is_some() || d.cancelled_day.is_some())
                .then(|| "Only an undelivered, uncancelled import can be returned.".into())
        });
        if let Ok((_, _, stock, _, _)) =
            import_source(w, d.seller, d.company, d.product, d.ammunition)
        {
            reason=reason.or_else(||stock.checked_add(d.quantity).is_none_or(|q|q>if d.ammunition{MAX_AMMO_STOCK}else{MAX_STOCK}).then(||"The seller's inventory is full; retained buyer property cannot be refunded until the supplier has room for its returned lot.".into()));
        } else {
            reason = reason.or(Some(
                "The original seller product cannot receive the returned stock.".into(),
            ));
        }
    }
    let mut q=ImportCancelQuote{valid:reason.is_none(),reason,token:String::new(),refund_bn:d.map_or(0.0,|d|d.total_price_bn),refund_after_settlement:d.is_some_and(|d|d.settled_day.is_none()),note:"Returns the entire undelivered lot and its original cost basis to the seller. The buyer's exact paid escrow refunds once after its original fiscal settlement; no departmental authority, research or equipment is granted. Delivery cannot also claim a cancelled lot.".into()};
    q.token = token(w, buyer, serde_json::json!(["cancel_import", id, &q]));
    q
}
fn cancel_import(
    w: &mut WorldState,
    buyer: NationId,
    id: u32,
    provided: &str,
) -> Result<(), String> {
    let q = import_cancel_quote(w, buyer, id);
    if !q.valid {
        return Err(q.reason.unwrap());
    }
    if q.token != provided {
        return Err(
            "This cancellation quote is stale; review the retained lot and seller stock again."
                .into(),
        );
    }
    let i = w
        .companies
        .imports
        .contracts
        .iter()
        .position(|d| d.id == id)
        .unwrap();
    let d = w.companies.imports.contracts[i].clone();
    let c = w
        .companies
        .firms
        .iter_mut()
        .find(|c| c.id == d.company)
        .unwrap();
    if d.ammunition {
        let p = c
            .ammunition_products
            .iter_mut()
            .find(|p| p.id == d.product)
            .unwrap();
        p.stock += d.quantity;
        p.sold_units -= d.quantity;
        p.stock_cost_bn += d.cost_basis_bn;
    } else {
        let p = c.products.iter_mut().find(|p| p.id == d.product).unwrap();
        p.stock += d.quantity;
        p.sold_units -= d.quantity;
        p.stock_cost_bn += d.cost_basis_bn;
    }
    let day = clock::absolute_day(w);
    let d = &mut w.companies.imports.contracts[i];
    d.cancelled_day = Some(day);
    d.status = "awaiting_refund_settlement".into();
    d.reason="Returned stock belongs to the seller again; the original buyer payment must settle before its refund.".into();
    settle_imports(w);
    Ok(())
}
fn settle_imports(w: &mut WorldState) {
    if w.companies.imports.contracts.is_empty() {
        return;
    }
    let today = clock::absolute_day(w);
    for i in 0..w.companies.imports.contracts.len() {
        let d = w.companies.imports.contracts[i].clone();
        if d.settled_day.is_none()
            && w.nation_opt(d.buyer)
                .and_then(|n| n.program_budget.as_ref())
                .and_then(|p| p.settled_day)
                == Some(d.purchased_day)
        {
            let live = &mut w.companies.imports.contracts[i];
            live.settled_day = Some(d.purchased_day);
            live.due_day = Some(d.purchased_day.saturating_add(d.transit_days as i32));
            live.escrow_bn = d.total_price_bn;
            live.status = "in_transit".into();
            live.reason="The buyer's settled payment is protected in import escrow until delivery or cancellation.".into();
            crate::fiscal_journal::supplier_settled(
                w,
                d.buyer,
                d.purchased_day,
                if d.ammunition {
                    crate::fiscal_journal::SupplierInclusion::Ammunition
                } else {
                    crate::fiscal_journal::SupplierInclusion::Equipment
                },
                d.total_price_bn,
            );
        }
        let d = &w.companies.imports.contracts[i];
        if d.cancelled_day.is_some() && d.settled_day.is_some() && d.refunded_day.is_none() {
            let buyer = d.buyer;
            let amount = d.total_price_bn;
            let gdp = w.nation(buyer).gdp.max(1e-12);
            crate::economy::charge_for(
                w,
                buyer,
                -amount,
                -amount / gdp,
                crate::fiscal_journal::CashCause::EquipmentImportRefund,
            );
            let d = &mut w.companies.imports.contracts[i];
            d.escrow_bn = 0.0;
            d.refunded_bn = amount;
            d.refunded_day = Some(today);
            d.status = "cancelled".into();
            d.reason="The lot returned to seller stock and the exact settled payment refunded once; spending authority was not recreated.".into();
        }
    }
}
fn tick_imports(w: &mut WorldState, day: i32) {
    for i in 0..w.companies.imports.contracts.len() {
        let d = w.companies.imports.contracts[i].clone();
        if d.delivered_day.is_some()
            || d.cancelled_day.is_some()
            || d.settled_day.is_none_or(|s| s >= day)
        {
            continue;
        }
        let reason = match company(w, d.seller, d.company) {
            None => Some("The original supplier identity is unavailable; the buyer's paid lot and escrow remain held in transit.".into()),
            Some(c) => {
                if w.districts.get(&d.district) != Some(&d.seller) {
                    Some("The seller's shipping province is outside government control.".into())
                } else {
                    crate::control::blocker(w, d.seller, &c.district).or_else(|| {
                        crate::logistics::freight_route_open(w, d.seller, d.buyer, &d.route).err()
                    })
                }
            }
        }
            .or_else(|| {
                (!w.nation_opt(d.buyer).is_some_and(|n| n.alive)).then(|| {
                    "The buyer government is inactive; its paid lot remains owned in transit."
                        .into()
                })
            });
        if let Some(reason) = reason {
            let live = &mut w.companies.imports.contracts[i];
            live.status = "blocked".into();
            live.reason = reason;
            live.due_day = live.due_day.map(|due| due.saturating_add(1));
            continue;
        }
        if d.due_day.is_none_or(|due| day < due) {
            let live = &mut w.companies.imports.contracts[i];
            live.status = "in_transit".into();
            live.reason =
                "The reviewed foreign route remains open; buyer-owned stock is in transit.".into();
            continue;
        }
        let n = w.nation_mut(d.buyer);
        let result = if d.ammunition {
            equipment::receive_supplier_ammunition(
                n,
                equipment::AmmoSupplierReceipt {
                    delivery: d.id,
                    company: d.company,
                    product: d.product,
                    family: d.family.clone().unwrap(),
                    quantity: d.quantity,
                    delivered_day: day,
                },
            )
        } else {
            let mut revision = d.source_revision.clone();
            revision.id = d.buyer_revision.clone().unwrap();
            let s = n
                .equipment
                .as_mut()
                .expect("import adoption activated equipment");
            s.version = equipment::VERSION;
            if s.revisions
                .get(&revision.id)
                .is_some_and(|old| old != &revision)
            {
                Err("The buyer revision identity is occupied by a different model; paid import retained.".into())
            } else if !s.revisions.contains_key(&revision.id)
                && s.revisions.len() >= equipment::MAX_REVISIONS
            {
                Err("The buyer revision library is full; paid import retained.".into())
            } else {
                let id = revision.id.clone();
                let added = s.revisions.insert(id.clone(), revision).is_none();
                let r = crate::arsenal::deliver_design(n, &id, d.quantity, 0.0);
                if r.is_err() && added {
                    n.equipment.as_mut().unwrap().revisions.remove(&id);
                }
                r
            }
        };
        let live = &mut w.companies.imports.contracts[i];
        match result {
            Ok(()) => {
                live.delivered_day = Some(day);
                live.escrow_bn = 0.0;
                live.status = "delivered".into();
                live.reason="The buyer received the exact foreign model or ammunition. Ordinary upkeep and compatibility apply; no research or manufacturing license was granted.".into();
                let c = w
                    .companies
                    .firms
                    .iter_mut()
                    .find(|c| c.id == d.company)
                    .unwrap();
                c.cash_bn += d.total_price_bn;
                c.sales_revenue_bn += d.total_price_bn;
                transaction(
                    c,
                    day,
                    "import_sale",
                    d.total_price_bn,
                    Some(d.product),
                    d.quantity,
                );
            }
            Err(reason) => {
                live.status = "blocked".into();
                live.reason = reason;
            }
        }
    }
}
fn imported_sold(w: &WorldState, c: u32, p: u32, ammo: bool) -> u64 {
    w.companies
        .imports
        .contracts
        .iter()
        .filter(|d| {
            d.company == c && d.product == p && d.ammunition == ammo && d.cancelled_day.is_none()
        })
        .map(|d| d.quantity as u64)
        .sum()
}
fn imported_ammo_receipt(w: &WorldState, n: NationId, r: &equipment::AmmoSupplierReceipt) -> bool {
    w.companies.imports.contracts.iter().any(|d| {
        d.id == r.delivery
            && d.buyer == n
            && d.ammunition
            && d.company == r.company
            && d.product == r.product
            && d.family.as_deref() == Some(r.family.as_str())
            && d.quantity == r.quantity
            && d.delivered_day == Some(r.delivered_day)
            && d.cancelled_day.is_none()
            && d.settled_day.is_some()
    })
}
fn validate_imports(w: &WorldState, ids: &mut BTreeSet<u32>) -> Result<(), String> {
    let book = &w.companies.imports;
    let day = clock::absolute_day(w);
    let fail = || {
        "Invalid imported stock, frozen origin, buyer escrow, transit or refund ownership."
            .to_string()
    };
    if book.is_empty() {
        if w.companies.version == IMPORT_VERSION {
            return Err(fail());
        }
        return Ok(());
    }
    if w.companies.version != IMPORT_VERSION
        || book.buyers.len() > crate::nations::nation_count()
        || book.contracts.len() > 100_000
    {
        return Err(fail());
    }
    let mut buyers = BTreeSet::new();
    for b in &book.buyers {
        if !buyers.insert(b.nation)
            || b.adopted_day > day
            || w.nation_opt(b.nation)
                .and_then(|n| n.equipment.as_ref())
                .is_none()
        {
            return Err(fail());
        }
    }
    let near = |a: f64, b: f64| (a - b).abs() <= 1e-12 + 1e-10 * a.abs().max(b.abs());
    for d in &book.contracts {
        let (c, r, _, _, family) =
            import_source(w, d.seller, d.company, d.product, d.ammunition).map_err(|_| fail())?;
        if d.id == 0
            || !ids.insert(d.id)
            || d.buyer == d.seller
            || !buyers.contains(&d.buyer)
            || d.district != c.district
            || &d.source_revision != r
            || d.family != family
            || d.quantity == 0
            || d.quantity
                > if d.ammunition {
                    MAX_AMMO_STOCK
                } else {
                    MAX_STOCK
                }
            || d.buyer_revision
                != (!d.ammunition).then(|| format!("import-{}-{}", d.company, d.product))
            || ![
                d.unit_price_bn,
                d.total_price_bn,
                d.cost_basis_bn,
                d.escrow_bn,
                d.refunded_bn,
            ]
            .iter()
            .all(|v| v.is_finite() && *v >= 0.0)
            || d.total_price_bn <= 0.0
            || !near(d.total_price_bn, d.unit_price_bn * d.quantity as f64)
            || !near(d.total_price_bn, d.cost_basis_bn * (1.0 + MARGIN))
            || d.purchased_day > day
            || d.source_revision
                .certified_day
                .is_none_or(|t| t > d.purchased_day)
            || d.transit_days != d.route.estimated_days.max(IMPORT_DAYS)
            || !crate::logistics::valid_import_route_geometry(&d.route)
            || d.route.nodes.is_empty()
            || d.route.segments.is_empty()
            || d.route.estimated_days == 0
            || d.transit_days > i32::MAX as u32
            || d.settled_day.is_some_and(|t| t != d.purchased_day)
            || d.settled_day.is_some() != d.due_day.is_some()
            || d.due_day
                .is_some_and(|t| t < d.purchased_day.saturating_add(d.transit_days as i32))
            || d.delivered_day.is_some_and(|t| {
                t > day || d.due_day.is_none_or(|due| t < due) || d.cancelled_day.is_some()
            })
            || d.cancelled_day
                .is_some_and(|t| t < d.purchased_day || t > day)
            || d.refunded_day.is_some_and(|t| {
                t > day || d.cancelled_day.is_none_or(|c| t < c) || d.settled_day.is_none()
            })
            || d.refunded_day.is_some() != (d.refunded_bn > 0.0)
            || d.refunded_day.is_some() && !near(d.refunded_bn, d.total_price_bn)
            || !near(
                d.escrow_bn,
                if d.settled_day.is_some() && d.delivered_day.is_none() && d.refunded_day.is_none()
                {
                    d.total_price_bn
                } else {
                    0.0
                },
            )
        {
            return Err(fail());
        }
        if let Some(delivered) = d.delivered_day {
            if d.ammunition {
                if !w
                    .nation(d.buyer)
                    .equipment
                    .as_ref()
                    .and_then(|s| s.ammunition.as_ref())
                    .is_some_and(|a| {
                        a.supplier_receipts
                            .iter()
                            .any(|r| r.delivery == d.id && imported_ammo_receipt(w, d.buyer, r))
                    })
                {
                    return Err(fail());
                }
            } else {
                let mut expected = d.source_revision.clone();
                expected.id = d.buyer_revision.clone().unwrap();
                if w.nation(d.buyer)
                    .equipment
                    .as_ref()
                    .and_then(|s| s.revisions.get(&expected.id))
                    != Some(&expected)
                    || delivered < d.purchased_day
                {
                    return Err(fail());
                }
            }
        }
        if d.settled_day.is_none() {
            let p = w.nation(d.buyer).program_budget.as_ref().ok_or_else(fail)?;
            if p.day != Some(d.purchased_day) || p.settled_day.is_some_and(|t| t >= d.purchased_day)
            {
                return Err(fail());
            }
        }
    }
    for buyer in buyers {
        for department in [2, 3] {
            let pending = book
                .contracts
                .iter()
                .filter(|d| {
                    d.buyer == buyer
                        && d.settled_day.is_none()
                        && (if d.ammunition { 2 } else { 3 }) == department
                })
                .map(|d| d.total_price_bn)
                .sum::<f64>();
            if pending > 0.0 {
                let p = w.nation(buyer).program_budget.as_ref().ok_or_else(fail)?;
                let domestic = w
                    .companies
                    .firms
                    .iter()
                    .filter(|c| c.nation == buyer)
                    .flat_map(|c| &c.receivables)
                    .filter(|r| {
                        if department == 2 {
                            r.kind == "ammo_sale"
                        } else {
                            matches!(r.kind.as_str(), "capitalization" | "sale" | "refit_advance")
                        }
                    })
                    .map(|r| r.amount_bn)
                    .sum::<f64>();
                if pending + domestic
                    > p.spent_today_bn[BUDGET_DEFENSE][department]
                        + p.prepaid_used_today_bn[BUDGET_DEFENSE][department]
                        + 1e-12
                {
                    return Err(fail());
                }
            }
        }
    }
    Ok(())
}
pub fn domestic_offers(w: &WorldState, buyer: NationId) -> Vec<ImportOffer> {
    let mut out = Vec::new();
    for c in w.companies.firms.iter().filter(|c| c.nation == buyer) {
        for (pid, ammo) in c
            .products
            .iter()
            .map(|p| (p.id, false))
            .chain(c.ammunition_products.iter().map(|p| (p.id, true)))
        {
            let Ok((_, r, stock, _, family)) = import_source(w, buyer, c.id, pid, ammo) else {
                continue;
            };
            let q = if ammo {
                ammo_purchase_terms(w, buyer, c.id, pid, 1)
            } else {
                purchase_terms(w, buyer, c.id, pid, 1)
            };
            let available = if ammo {
                q.purchase_available_bn
            } else {
                programs::available_bn(w, buyer, BUDGET_DEFENSE, 3)
            };
            let mut affordable = if q.unit_price_bn > 0.0 && q.unit_price_bn.is_finite() {
                (available / q.unit_price_bn)
                    .floor()
                    .max(0.0)
                    .min(stock as f64) as u32
            } else {
                0
            };
            while affordable > 0 && q.unit_price_bn * affordable as f64 > available {
                affordable -= 1;
            }
            out.push(ImportOffer {
                seller: buyer,
                company: c.id,
                product: pid,
                ammunition: ammo,
                supplier_name: c.name.clone(),
                product_name: if ammo {
                    family.clone().unwrap()
                } else {
                    r.name.clone()
                },
                platform: (!ammo).then(|| r.spec.platform.clone()),
                family,
                spec: (!ammo).then(|| r.spec.clone()),
                ready_stock: stock,
                unit_price_bn: q.unit_price_bn,
                purchase_available_bn: available,
                protected_maintenance_bn: q.protected_maintenance_bn,
                affordable_quantity: affordable,
                review_quantity: if stock > 0 { 1 } else { 0 },
                delivery_days: q.eta_days,
                allowed: q.valid,
                reason: q.reason,
            });
        }
    }
    out
}
