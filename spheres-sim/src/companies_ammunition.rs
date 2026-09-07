// Existing certified ammunition recipes, manufactured with corporate cash on
// the same leased plant slot. Nothing here invents a stock, weapon or factory.
pub const MAX_AMMO_STOCK: u32 = equipment::MAX_AMMO_ORDER;
pub const MAX_AMMO_DELIVERIES: usize = 100_000;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AmmoProduct {
    pub id: u32,
    pub family: String,
    pub source_revision: String,
    pub stock_target: u32,
    pub stock: u32,
    pub stock_cost_bn: f64,
    pub produced_units: u32,
    pub sold_units: u32,
    pub materials_expense_bn: f64,
    pub fabrication_expense_bn: f64,
    pub resources_used: [f64; 12],
    pub started_day: i32,
    pub last_work_day: Option<i32>,
    pub status: String,
    pub reason: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AmmoDelivery {
    pub id: u32,
    pub company: u32,
    pub buyer: NationId,
    pub product: u32,
    pub family: String,
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

pub fn ammo_supplier_active(w: &WorldState, n: NationId, family: &str) -> bool {
    w.companies
        .firms
        .iter()
        .filter(|c| c.nation == n)
        .any(|c| c.ammunition_products.iter().any(|p| p.family == family))
}
pub fn ammo_inbound_units(w: &WorldState, n: NationId, family: &str) -> u64 {
    w.companies
        .ammunition_deliveries
        .iter()
        .filter(|d| d.buyer == n && d.family == family && d.delivered_day.is_none())
        .map(|d| d.quantity as u64)
        .sum()
}
fn ammo_source<'a>(
    w: &'a WorldState,
    n: NationId,
    family: &str,
) -> Option<&'a equipment::DesignRevision> {
    w.nation_opt(n)
        .and_then(|n| n.equipment.as_ref())
        .and_then(|s| {
            s.revisions.values().find(|r| {
                r.certified_day.is_some_and(|d| d <= clock::absolute_day(w))
                    && equipment::ammunition_family(&r.spec) == Some(family)
            })
        })
}
fn ammo_refusal(w: &WorldState, n: NationId, family: &str) -> Option<String> {
    actor(w, n)
        .or_else(|| equipment::ammo_actor_refusal(w, n))
        .or_else(|| equipment::ammo_def(family).is_none().then(|| "Choose a supported ammunition family.".into()))
        .or_else(|| ammo_source(w, n, family).is_none().then(|| "Certify a compatible weapon configuration before licensing this ammunition family.".into()))
}
fn ammo_indexes(
    w: &WorldState,
    n: NationId,
    id: u32,
    product: u32,
) -> Result<(usize, usize), String> {
    let i = w
        .companies
        .firms
        .iter()
        .position(|c| c.id == id && c.nation == n)
        .ok_or("This domestic company is missing.")?;
    let j = w.companies.firms[i]
        .ammunition_products
        .iter()
        .position(|p| p.id == product)
        .ok_or("This ammunition supply product is missing.")?;
    Ok((i, j))
}
fn equipment_pending(p: &Product) -> bool {
    p.cancelled_day.is_none()
        && (p.stock < p.stock_target
            || p.unit_work_days > 0.0
            || p.unit_inputs.iter().any(|x| *x > 0.0))
}
/// The one development job takes priority, followed by paid refit contracts.
/// Then one globally ordered equipment/ammunition product gets the work packet.
fn scheduled_product(c: &Company) -> Option<u32> {
    c.products
        .iter()
        .find(|p| p.cancelled_day.is_none() && p.certified_day.is_none())
        .map(|p| p.id)
        .or_else(|| {
            c.refits
                .iter()
                .filter(|p| refit_remaining(p) > 0)
                .map(|p| p.id)
                .min()
        })
        .or_else(|| {
            c.products
                .iter()
                .filter(|p| equipment_pending(p))
                .map(|p| p.id)
                .chain(
                    c.ammunition_products
                        .iter()
                        .filter(|p| p.stock < p.stock_target)
                        .map(|p| p.id),
                )
                .min()
        })
}
/// Money still needed for the actual maintenance plan on this open date is
/// reserved before a reviewed ammunition purchase, including prepaid money.
pub fn ammo_purchase_funding(w: &WorldState, n: NationId) -> (f64, f64) {
    let available = programs::available_bn(w, n, BUDGET_DEFENSE, 2);
    let day = clock::absolute_day(w);
    let protected = w
        .nation_opt(n)
        .and_then(|n| n.equipment.as_ref())
        .and_then(|s| s.maintenance_plan.as_ref())
        .map_or(0.0, |p| {
            if p.from_day > day || p.receipt.as_ref().is_some_and(|r| r.day >= day) {
                0.0
            } else {
                (equipment::fleet_maintenance_requirement(w.nation(n))
                    + equipment::legacy_maintenance_requirement(w.nation(n)))
                .min(p.daily_limit_bn)
                .min(available)
                .max(0.0)
            }
        });
    (protected, (available - protected).max(0.0))
}
pub fn ammo_supply_quote(w: &WorldState, n: NationId, id: u32, family: &str, target: u32) -> Quote {
    let c = company(w, n, id);
    let mut q = Quote::default();
    q.reason = ammo_refusal(w, n, family)
        .or_else(|| (!(1..=MAX_AMMO_STOCK).contains(&target)).then(|| format!("Choose an initial ammunition stock buffer from 1 to {MAX_AMMO_STOCK}.")))
        .or_else(|| c.is_none().then(|| "Establish a domestic contractor first.".into()))
        .or_else(|| c.and_then(|c| facility_blocker(w, c)))
        .or_else(|| c.and_then(|c| c.ammunition_products.iter().any(|p| p.family == family).then(|| "The company already supplies this ammunition family. Adjust its stock buffer instead.".into())));
    if let Some(def) = equipment::ammo_def(family) {
        let cost = material_cost(w, &def.recipe) + def.fabrication_bn;
        q.unit_price_bn = cost * (1.0 + MARGIN);
        q.company_cash_needed_bn = cost * target as f64;
        q.minimum_days = (target as f64 / def.rounds_per_day).ceil() as u32;
        if let Some(c) =
            c.filter(|c| facility_blocker(w, c).is_none() && scheduled_product(c).is_none())
        {
            if c.cash_bn >= cost
                && resources::ALL
                    .iter()
                    .all(|r| resources::stockpile(w, n, *r) >= def.recipe[r.idx()])
            {
                q.first_stock_days = Some(1);
            }
            if c.cash_bn >= q.company_cash_needed_bn
                && resources::ALL
                    .iter()
                    .all(|r| resources::stockpile(w, n, *r) >= def.recipe[r.idx()] * target as f64)
            {
                q.eta_days = Some(q.minimum_days);
            }
        }
    }
    q.note = "Authorizes a finite company-owned stock buffer using an already certified ammunition family. No public fee, rounds or activation are granted. The company buys actual domestic raw inputs and pays the existing fabrication cost at the existing plant rate. This product shares the contractor's one leased Arms Plant slot with equipment. Converting this family stops new automatic public reserve batches; existing targets and already commissioned public batches remain. Government purchases require a separate review and seven accessible delivery days after fiscal settlement. Rates, margin and delivery timing are game assumptions.".into();
    finish_quote(
        w,
        n,
        serde_json::json!(["ammo_supply", id, family, target]),
        q,
    )
}
pub fn ammo_purchase_quote(
    w: &WorldState,
    n: NationId,
    id: u32,
    product: u32,
    quantity: u32,
) -> Quote {
    let (protected, available) = ammo_purchase_funding(w, n);
    let mut q = Quote {
        minimum_days: DELIVERY_DAYS,
        eta_days: Some(DELIVERY_DAYS),
        protected_maintenance_bn: protected,
        purchase_available_bn: available,
        ..Quote::default()
    };
    q.reason = actor(w, n)
        .or_else(|| {
            (!(1..=MAX_AMMO_STOCK).contains(&quantity))
                .then(|| format!("Choose 1 to {MAX_AMMO_STOCK} finished rounds or stores."))
        })
        .or_else(|| {
            (w.companies.ammunition_deliveries.len() >= MAX_AMMO_DELIVERIES).then(|| {
                "The ammunition delivery ledger is full; no further purchases can be recorded."
                    .into()
            })
        });
    if let Some(c) = company(w, n, id) {
        q.reason = q.reason.or_else(|| facility_blocker(w, c));
        if let Some(p) = c.ammunition_products.iter().find(|p| p.id == product) {
            q.available_units = p.stock;
            q.unit_price_bn = if p.stock == 0 {
                0.0
            } else {
                p.stock_cost_bn / p.stock as f64 * (1.0 + MARGIN)
            };
            q.cost_bn = q.unit_price_bn * quantity as f64;
            q.reason = q.reason.or_else(|| ammo_refusal(w,n,&p.family))
                .or_else(|| (!equipment::maintenance_plan_on(w.nation(n), clock::absolute_day(w))).then(|| "Wait for the actual fleet maintenance plan to take effect before buying ammunition.".into()))
                .or_else(|| (quantity > p.stock).then(|| "The requested ammunition exceeds finite finished company stock. No new manufacturing is purchased here.".into()))
                .or_else(|| (q.cost_bn > available).then(|| "Defense maintenance and supply authority is insufficient after protecting today's actual fleet upkeep. Let authority accrue or adjust the budget.".into()));
        } else {
            q.reason = Some("This ammunition supply product is missing.".into());
        }
    } else {
        q.reason = Some("This domestic company is missing.".into());
    }
    q.note = "Purchase existing finished company ammunition with Defense maintenance and supply authority after protecting today's actual fleet upkeep. Price is average paid input and fabrication cost plus the existing 15% margin. Company revenue settles once with the public fiscal day. Purchased stock becomes usable only after seven accessible transit days; no physical-ammunition activation, weapon model or public fabrication order is created.".into();
    finish_quote(
        w,
        n,
        serde_json::json!(["ammo_purchase", id, product, quantity]),
        q,
    )
}
fn start_ammo_supply(
    w: &mut WorldState,
    n: NationId,
    id: u32,
    family: &str,
    target: u32,
    reviewed: &str,
) -> Result<(), String> {
    accept(ammo_supply_quote(w, n, id, family, target), reviewed)?;
    let source_revision = ammo_source(w, n, family).unwrap().id.clone();
    let day = clock::absolute_day(w);
    let product = next_id(w);
    w.companies.version = w.companies.version.max(AMMUNITION_VERSION);
    let c = w
        .companies
        .firms
        .iter_mut()
        .find(|c| c.id == id && c.nation == n)
        .unwrap();
    c.ammunition_products.push(AmmoProduct {
        id: product,
        family: family.into(),
        source_revision,
        stock_target: target,
        stock: 0,
        stock_cost_bn: 0.0,
        produced_units: 0,
        sold_units: 0,
        materials_expense_bn: 0.0,
        fabrication_expense_bn: 0.0,
        resources_used: [0.0; 12],
        started_day: day,
        last_work_day: None,
        status: "queued".into(),
        reason:
            "Awaiting a future plant work date. No ammunition or public purchase has been created."
                .into(),
    });
    Ok(())
}
fn set_ammo_inventory(
    w: &mut WorldState,
    n: NationId,
    id: u32,
    product: u32,
    target: u32,
) -> Result<(), String> {
    if target > MAX_AMMO_STOCK {
        return Err(format!(
            "Choose an ammunition stock buffer from 0 to {MAX_AMMO_STOCK}."
        ));
    }
    let (i, j) = ammo_indexes(w, n, id, product)?;
    w.companies.firms[i].ammunition_products[j].stock_target = target;
    Ok(())
}
fn buy_ammo_stock(
    w: &mut WorldState,
    n: NationId,
    company: u32,
    product: u32,
    quantity: u32,
    reviewed: &str,
) -> Result<(), String> {
    let q = accept(
        ammo_purchase_quote(w, n, company, product, quantity),
        reviewed,
    )?;
    // apply() rolls back even fiscal-day initialization on an exceptional error.
    charge(w, n, 2, q.cost_bn)?;
    let (i, j) = ammo_indexes(w, n, company, product)?;
    let id = next_id(w);
    let day = clock::absolute_day(w);
    let c = &mut w.companies.firms[i];
    let p = &mut c.ammunition_products[j];
    let cost = if quantity == p.stock {
        p.stock_cost_bn
    } else {
        p.stock_cost_bn / p.stock as f64 * quantity as f64
    };
    p.stock -= quantity;
    p.stock_cost_bn = (p.stock_cost_bn - cost).max(0.0);
    p.sold_units = p
        .sold_units
        .checked_add(quantity)
        .ok_or("The ammunition sale ledger is full.")?;
    w.companies.ammunition_deliveries.push(AmmoDelivery { id, company, buyer:n, product, family:p.family.clone(), district:c.district.clone(), quantity, unit_price_bn:q.unit_price_bn, total_price_bn:q.cost_bn, cost_basis_bn:cost, purchased_day:day, settled_day:None, due_day:None, delivered_day:None, status:"awaiting_settlement".into(), reason:"Purchased ammunition is reserved to the government; shipping begins after fiscal settlement.".into() });
    receipt(w, i, "ammo_sale", q.cost_bn, Some(product), Some(id));
    Ok(())
}
fn update_ammo_queue(w: &mut WorldState, i: usize, selected: Option<u32>) {
    for p in &mut w.companies.firms[i].ammunition_products {
        if Some(p.id) != selected {
            p.status = if p.stock > 0 { "in_stock" } else { "idle" }.into();
            p.reason=if p.stock>=p.stock_target {"The finite ammunition stock buffer is met. Purchases require government review."}else{"Waiting for the contractor's shared plant work packet; equipment and ammunition use the same slot."}.into();
        }
    }
}
fn ammo_blocked(w: &mut WorldState, i: usize, j: usize, reason: String) {
    let p = &mut w.companies.firms[i].ammunition_products[j];
    p.status = "blocked".into();
    p.reason = reason;
}
fn tick_ammo_manufacturing(
    w: &mut WorldState,
    i: usize,
    j: usize,
    day: i32,
    facility: Option<String>,
) {
    let c = w.companies.firms[i].clone();
    let p = &c.ammunition_products[j];
    if day <= p.started_day || p.last_work_day.is_some_and(|d| d >= day) {
        return;
    }
    if let Some(reason) = facility.or_else(|| ammo_refusal(w, c.nation, &p.family)) {
        ammo_blocked(w, i, j, reason);
        return;
    }
    let def = equipment::ammo_def(&p.family).unwrap();
    let raw_unit = material_cost(w, &def.recipe);
    let unit = raw_unit + def.fabrication_bn;
    let wanted = p
        .stock_target
        .saturating_sub(p.stock)
        .min(def.rounds_per_day.floor() as u32)
        .min(u32::MAX - p.produced_units);
    // Whole paid rounds only. Every recipe and fabrication dollar belongs to
    // the company, and all purchases happen after the full packet preflight.
    let mut quantity = wanted.min((c.cash_bn / unit).floor().clamp(0.0, u32::MAX as f64) as u32);
    for r in resources::ALL {
        if def.recipe[r.idx()] > 0.0 {
            quantity = quantity.min(
                (resources::stockpile(w, c.nation, r) / def.recipe[r.idx()])
                    .floor()
                    .clamp(0.0, u32::MAX as f64) as u32,
            );
        }
    }
    if quantity == 0 {
        ammo_blocked(w,i,j,"Company cash and actual domestic raw inputs must cover at least one complete round or store. No partial or free ammunition is created.".into());
        return;
    }
    let raw = def.recipe.map(|v| v * quantity as f64);
    let materials = material_cost(w, &raw);
    let fabrication = def.fabrication_bn * quantity as f64;
    if materials + fabrication > c.cash_bn {
        ammo_blocked(
            w,
            i,
            j,
            "The exact paid ammunition packet exceeds company working capital.".into(),
        );
        return;
    }
    if let Err((r, _, _)) = resources::consume_stockpile_atomic(w, c.nation, &raw) {
        ammo_blocked(
            w,
            i,
            j,
            format!(
                "The domestic warehouse lacks {} for this ammunition packet.",
                r.name()
            ),
        );
        return;
    }
    let gdp = w.nation(c.nation).gdp.max(0.1);
    crate::economy::charge(w, c.nation, -materials, -materials / gdp);
    let c = &mut w.companies.firms[i];
    c.cash_bn -= materials + fabrication;
    c.materials_expense_bn += materials;
    c.fabrication_expense_bn += fabrication;
    transaction(c, day, "ammo_materials", materials, Some(p.id), quantity);
    transaction(
        c,
        day,
        "ammo_fabrication",
        fabrication,
        Some(p.id),
        quantity,
    );
    let p = &mut c.ammunition_products[j];
    p.stock += quantity;
    p.produced_units += quantity;
    p.stock_cost_bn += materials + fabrication;
    p.materials_expense_bn += materials;
    p.fabrication_expense_bn += fabrication;
    p.last_work_day = Some(day);
    for k in 0..12 {
        p.resources_used[k] += raw[k];
    }
    p.status = "in_stock".into();
    p.reason="Paid ammunition is finished company-owned stock. It enters no national ammunition book until a reviewed purchase settles and arrives.".into();
}
fn ammo_delivery_blocker(w: &WorldState, d: &AmmoDelivery) -> Option<String> {
    if !w.nation_opt(d.buyer).is_some_and(|n| n.alive) {
        return Some(
            "The buyer government is inactive; purchased ammunition remains owned in transit."
                .into(),
        );
    }
    if w.districts.get(&d.district) != Some(&d.buyer) {
        return Some("Domestic ammunition shipping is paused while the source province is outside government control.".into());
    }
    crate::control::blocker(w, d.buyer, &d.district)
}
fn tick_ammo_deliveries(w: &mut WorldState, day: i32) {
    for i in 0..w.companies.ammunition_deliveries.len() {
        let d = w.companies.ammunition_deliveries[i].clone();
        if d.delivered_day.is_some() || d.settled_day.is_none() || d.settled_day == Some(day) {
            continue;
        }
        if let Some(reason) = ammo_delivery_blocker(w, &d) {
            let d = &mut w.companies.ammunition_deliveries[i];
            d.status = "blocked".into();
            d.reason = reason;
            d.due_day = d.due_day.map(|due| due.saturating_add(1));
            continue;
        }
        if d.due_day.is_some_and(|due| day >= due) {
            let result = equipment::receive_supplier_ammunition(
                w.nation_mut(d.buyer),
                equipment::AmmoSupplierReceipt {
                    delivery: d.id,
                    company: d.company,
                    product: d.product,
                    family: d.family.clone(),
                    quantity: d.quantity,
                    delivered_day: day,
                },
            );
            let d = &mut w.companies.ammunition_deliveries[i];
            match result {
                Ok(()) => {
                    d.delivered_day = Some(day);
                    d.status = "delivered".into();
                    d.reason="Received into the existing national ammunition store with a matching paid supplier receipt. Ordinary compatibility and consumption rules apply.".into();
                }
                Err(reason) => {
                    d.status = "blocked".into();
                    d.reason = reason;
                }
            }
        } else {
            let d = &mut w.companies.ammunition_deliveries[i];
            d.status = "in_transit".into();
            d.reason = "Domestic ammunition delivery is progressing.".into();
        }
    }
}

fn ammo_product_view(w: &WorldState, c: &Company, p: &AmmoProduct) -> serde_json::Value {
    let Some(def) = equipment::ammo_def(&p.family) else {
        return serde_json::to_value(p).unwrap();
    };
    let unit = material_cost(w, &def.recipe) + def.fabrication_bn;
    let remaining = p.stock_target.saturating_sub(p.stock);
    let next = remaining.min(def.rounds_per_day as u32);
    let full_cash = unit * remaining as f64;
    let materials_ready = resources::ALL
        .iter()
        .all(|r| resources::stockpile(w, c.nation, *r) >= def.recipe[r.idx()] * remaining as f64);
    let eta = if remaining > 0
        && scheduled_product(c) == Some(p.id)
        && facility_blocker(w, c).is_none()
        && c.cash_bn >= full_cash
        && materials_ready
    {
        Some((remaining as f64 / def.rounds_per_day).ceil() as u32)
    } else {
        None
    };
    let mut v = serde_json::to_value(p).unwrap();
    v["name"] = serde_json::json!(def.name);
    v["unit_label"] = serde_json::json!(if def.unit == "stores" {
        "store"
    } else {
        "round"
    });
    v["rounds_per_day"] = serde_json::json!(def.rounds_per_day);
    v["recipe_per_unit"] = serde_json::json!(def.recipe);
    v["fabrication_bn"] = serde_json::json!(def.fabrication_bn);
    v["unit_price_bn"] = serde_json::json!(if p.stock > 0 {
        p.stock_cost_bn / p.stock as f64 * (1.0 + MARGIN)
    } else {
        unit * (1.0 + MARGIN)
    });
    v["estimated_stock_days"] = serde_json::json!(eta);
    v["company_cash_needed_bn"] = serde_json::json!(unit * next as f64);
    v["national_stock"] = serde_json::json!(w
        .nation(c.nation)
        .equipment
        .as_ref()
        .and_then(|s| s.ammunition.as_ref())
        .and_then(|a| a.stocks.get(&p.family))
        .copied()
        .unwrap_or(0.0));
    v["incoming"] = serde_json::json!(ammo_inbound_units(w, c.nation, &p.family));
    v["reserve_status"] = serde_json::json!(equipment::ammo_reserve_status(w, c.nation, &p.family));
    v["estimate_note"]=serde_json::json!("Stock-buffer timing assumes sufficient settled company cash, all remaining raw inputs, continuous access and no earlier product using the shared slot. This family uses reviewed government purchases; existing reserve targets and previously commissioned public work remain.");
    v
}
fn ammo_deliveries_view(w: &WorldState, n: NationId) -> Vec<serde_json::Value> {
    w.companies
        .ammunition_deliveries
        .iter()
        .filter(|d| d.buyer == n)
        .map(|d| {
            let mut v = serde_json::to_value(d).unwrap();
            let def = equipment::ammo_def(&d.family);
            v["name"] = serde_json::json!(def.map(|d| d.name));
            v["unit_label"] = serde_json::json!(if def.is_some_and(|d| d.unit == "stores") {
                "store"
            } else {
                "round"
            });
            v["remaining_days"] =
                serde_json::json!(d.due_day.map(|due| (due - clock::absolute_day(w)).max(0)));
            v
        })
        .collect()
}

fn validate_ammo_supplier_state(w: &WorldState, ids: &mut BTreeSet<u32>) -> Result<(), String> {
    let fail =
        || "Invalid company ammunition work, ownership, payment or supplier receipt.".to_string();
    let day = clock::absolute_day(w);
    let state = &w.companies;
    let finite = |v: f64| v.is_finite() && v >= 0.0;
    let near = |a: f64, b: f64| (a - b).abs() <= 1e-12 + 1e-10 * a.abs().max(b.abs());
    let has_products = state
        .firms
        .iter()
        .any(|c| !c.ammunition_products.is_empty());
    let has_receipts = w.nations.iter().any(|n| {
        n.equipment
            .as_ref()
            .and_then(|s| s.ammunition.as_ref())
            .is_some_and(|a| !a.supplier_receipts.is_empty())
    });
    if state.version < AMMUNITION_VERSION
        && (has_products || has_receipts || !state.ammunition_deliveries.is_empty())
        || state.version == AMMUNITION_VERSION && !has_products
        || state.ammunition_deliveries.len() > MAX_AMMO_DELIVERIES
    {
        return Err(fail());
    }
    for c in &state.firms {
        if c.ammunition_products.len() > equipment::ammo_catalog().len() {
            return Err(fail());
        }
        let mut families = BTreeSet::new();
        for p in &c.ammunition_products {
            let def = equipment::ammo_def(&p.family).ok_or_else(fail)?;
            let revision = w
                .nation(c.nation)
                .equipment
                .as_ref()
                .and_then(|s| s.revisions.get(&p.source_revision))
                .ok_or_else(fail)?;
            if p.id == 0
                || !ids.insert(p.id)
                || !families.insert(&p.family)
                || revision.certified_day.is_none_or(|d| d > p.started_day)
                || equipment::ammunition_family(&revision.spec) != Some(p.family.as_str())
                || p.started_day < c.established_day
                || p.started_day > day
                || p.stock_target > MAX_AMMO_STOCK
                || p.stock > MAX_AMMO_STOCK
                || p.stock.checked_add(p.sold_units) != Some(p.produced_units)
                || ![
                    p.stock_cost_bn,
                    p.materials_expense_bn,
                    p.fabrication_expense_bn,
                ]
                .into_iter()
                .all(finite)
                || !p.resources_used.into_iter().all(finite)
                || p.last_work_day
                    .is_some_and(|d| d <= p.started_day || d > day)
                || p.produced_units > 0 && p.last_work_day.is_none()
                || p.produced_units == 0
                    && (p.last_work_day.is_some() || p.materials_expense_bn != 0.0)
                || p.produced_units > 0 && p.materials_expense_bn <= 0.0
                || p.produced_units as f64
                    > p.last_work_day
                        .map_or(0.0, |d| (d - p.started_day) as f64 * def.rounds_per_day)
                || !near(
                    p.fabrication_expense_bn,
                    def.fabrication_bn * p.produced_units as f64,
                )
                || p.stock == 0 && p.stock_cost_bn > 1e-12
                || p.stock_cost_bn + 1e-12 < p.stock as f64 * def.fabrication_bn
                || p.resources_used
                    .iter()
                    .zip(def.recipe)
                    .any(|(v, per)| !near(*v, per * p.produced_units as f64))
            {
                return Err(fail());
            }
            let sold = state
                .ammunition_deliveries
                .iter()
                .filter(|d| d.company == c.id && d.product == p.id);
            if sold.clone().map(|d| d.quantity as u64).sum::<u64>() != p.sold_units as u64
                || !near(
                    p.stock_cost_bn + sold.map(|d| d.cost_basis_bn).sum::<f64>(),
                    p.materials_expense_bn + p.fabrication_expense_bn,
                )
            {
                return Err(fail());
            }
        }
        if !c.ammunition_products.is_empty() {
            let equipment_fabrication = c
                .products
                .iter()
                .map(|p| {
                    let x = &w.nation(c.nation).equipment.as_ref().unwrap().revisions
                        [&p.revision_id]
                        .profile;
                    x.fabrication_cost_bn * p.produced_units as f64
                        + (p.unit_spent_bn - p.unit_material_cost_bn).max(0.0)
                })
                .sum::<f64>();
            let ammunition_fabrication = c
                .ammunition_products
                .iter()
                .map(|p| p.fabrication_expense_bn)
                .sum::<f64>();
            let ammunition_materials = c
                .ammunition_products
                .iter()
                .map(|p| p.materials_expense_bn)
                .sum::<f64>();
            if !near(
                c.fabrication_expense_bn,
                equipment_fabrication + ammunition_fabrication + refit_fabrication_cost(c),
            ) || c.materials_expense_bn + 1e-12 < ammunition_materials
            {
                return Err(fail());
            }
        }
    }
    for d in &state.ammunition_deliveries {
        let c = state
            .firms
            .iter()
            .find(|c| c.id == d.company)
            .ok_or_else(fail)?;
        let p = c
            .ammunition_products
            .iter()
            .find(|p| p.id == d.product)
            .ok_or_else(fail)?;
        if d.id == 0
            || !ids.insert(d.id)
            || d.buyer != c.nation
            || d.family != p.family
            || d.district != c.district
            || !(1..=MAX_AMMO_STOCK).contains(&d.quantity)
            || ![d.unit_price_bn, d.total_price_bn, d.cost_basis_bn]
                .into_iter()
                .all(|v| finite(v) && v > 0.0)
            || !near(d.total_price_bn, d.unit_price_bn * d.quantity as f64)
            || !near(d.total_price_bn, d.cost_basis_bn * (1.0 + MARGIN))
            || d.purchased_day <= p.started_day
            || d.purchased_day > day
            || d.settled_day
                .is_some_and(|settled| settled < d.purchased_day || settled > day)
            || d.due_day
                .zip(d.settled_day)
                .is_some_and(|(due, settled)| due < settled.saturating_add(DELIVERY_DAYS as i32))
            || d.settled_day.is_none() && (d.due_day.is_some() || d.delivered_day.is_some())
            || d.settled_day.is_some() && d.due_day.is_none()
            || d.delivered_day.is_some_and(|arrived| arrived > day)
            || d.delivered_day
                .zip(d.due_day)
                .is_some_and(|(arrived, due)| arrived < due)
        {
            return Err(fail());
        }
        let invoices = c
            .receivables
            .iter()
            .filter(|r| r.kind == "ammo_sale" && r.delivery == Some(d.id))
            .count();
        if invoices != usize::from(d.settled_day.is_none()) {
            return Err(fail());
        }
        let receipts = w
            .nation(d.buyer)
            .equipment
            .as_ref()
            .and_then(|s| s.ammunition.as_ref())
            .map_or(0, |a| {
                a.supplier_receipts
                    .iter()
                    .filter(|r| r.delivery == d.id)
                    .count()
            });
        if receipts != usize::from(d.delivered_day.is_some()) {
            return Err(fail());
        }
    }
    let mut received = BTreeSet::new();
    for n in &w.nations {
        if let Some(a) = n.equipment.as_ref().and_then(|s| s.ammunition.as_ref()) {
            for r in &a.supplier_receipts {
                let d = state
                    .ammunition_deliveries
                    .iter()
                    .find(|d| d.id == r.delivery)
                    .ok_or_else(fail)?;
                if !received.insert(r.delivery)
                    || d.buyer != n.id
                    || d.company != r.company
                    || d.product != r.product
                    || d.family != r.family
                    || d.quantity != r.quantity
                    || d.delivered_day != Some(r.delivered_day)
                    || d.settled_day.is_none()
                {
                    return Err(fail());
                }
            }
        }
    }
    Ok(())
}
