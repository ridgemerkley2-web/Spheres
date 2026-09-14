// S14 standing permission: existing fleet upkeep first, then finished aircraft
// stores. ProgramBudget, company stock and their delivery ledgers retain ownership.
pub const AIR_SUPPORT_DETAIL: &str = "Fleet upkeep + aircraft stores: from the next day, this single cap funds the actual whole-fleet maintenance invoice first, then compatible finished stores for owned aircraft. The previous fleet-maintenance limit resumes when automatic support is disabled. Separately ordered work is outside this policy. Existing Defense authority still limits all payments. This policy creates no aircraft, manufacturing contract, import enrollment or spending authority. Store targets use a 30-day planning month; inbound purchases and unfinished public batches count toward the target.";

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AirSupportPolicy {
    pub from_day: i32,
    pub daily_budget_bn: f64,
    pub target_days: u32,
    pub automatic: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous: Option<AirSupportTerms>,
    pub receipt: Option<AirSupportReceipt>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AirSupportTerms {
    pub from_day: i32,
    pub daily_budget_bn: f64,
    pub target_days: u32,
    pub automatic: bool,
}

fn active_air_support(p: &AirSupportPolicy, day: i32) -> Option<AirSupportTerms> {
    if p.from_day <= day {
        Some(AirSupportTerms {
            from_day: p.from_day,
            daily_budget_bn: p.daily_budget_bn,
            target_days: p.target_days,
            automatic: p.automatic,
        })
    } else {
        p.previous.clone().filter(|p| p.from_day <= day)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AirSupportReceipt {
    pub day: i32,
    pub authorized_from_day: i32,
    pub daily_budget_bn: f64,
    pub maintenance_paid_bn: f64,
    pub stores_paid_bn: f64,
    pub purchases: Vec<AirSupportPurchase>,
    pub families: Vec<AirSupportFamily>,
    pub reason: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AirSupportPurchase {
    pub seller: NationId,
    pub company: u32,
    pub product: u32,
    pub delivery: u32,
    pub imported: bool,
    pub family: String,
    pub quantity: u32,
    pub cost_bn: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AirSupportFamily {
    pub family: String,
    pub aircraft: u64,
    pub target_stores: u32,
    pub stock: f64,
    pub inbound: u64,
    pub public_committed: u64,
    pub gap: u32,
    pub bought: u32,
    pub reason: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct AirSupportStatus {
    /// Today's effective permission, including an explicitly disabled policy.
    pub active: Option<AirSupportTerms>,
    /// Reviewed settings that have not reached their effective date yet.
    pub pending: Option<AirSupportTerms>,
    pub automatic: bool,
    pub from_day: Option<i32>,
    pub daily_budget_bn: f64,
    pub target_days: u32,
    pub maintenance_required_bn: f64,
    pub maintenance_paid_bn: f64,
    pub stores_paid_bn: f64,
    pub remaining_bn: f64,
    pub families: Vec<AirSupportFamily>,
    pub reason: String,
    pub detail: &'static str,
}

fn air_support_values_refusal(daily_budget_mn: f64, target_days: u32) -> Option<String> {
    budget_refusal(daily_budget_mn / 1000.0).or_else(|| {
        (!(1..=365).contains(&target_days))
            .then(|| "Choose an aircraft-store planning target from 1 to 365 days.".into())
    })
}

pub fn air_support_refusal(
    w: &WorldState,
    id: NationId,
    daily_budget_mn: f64,
    target_days: u32,
    _automatic: bool,
) -> Option<String> {
    actor_refusal(w, id)
        .or_else(|| air_support_values_refusal(daily_budget_mn, target_days))
        .or_else(|| {
            w.nation(id).program_budget.is_none().then(|| {
                "Open departmental fiscal accounts before funding routine aircraft support.".into()
            })
        })
        .or_else(|| validate_state(w.nation(id)).err())
}

pub fn set_air_support(
    w: &mut WorldState,
    id: NationId,
    daily_budget_mn: f64,
    target_days: u32,
    automatic: bool,
) -> Result<(), String> {
    if let Some(reason) = air_support_refusal(w, id, daily_budget_mn, target_days, automatic) {
        return Err(reason);
    }
    let cap = daily_budget_mn / 1000.0;
    // An initial ordinary maintenance plan is prospective too. The retained
    // normal cap resumes when this separately reviewed policy is disabled.
    if automatic
        && w.nation(id)
            .equipment
            .as_ref()
            .is_none_or(|s| s.maintenance_plan.is_none())
    {
        set_maintenance_plan(w, id, cap)?;
    }
    let day = clock::absolute_day(w);
    let from_day = day.saturating_add(1);
    let s = state_mut(w.nation_mut(id));
    let receipt = s.air_support.as_ref().and_then(|p| p.receipt.clone());
    let previous = s
        .air_support
        .as_ref()
        .and_then(|p| active_air_support(p, day));
    s.air_support = Some(AirSupportPolicy {
        from_day,
        daily_budget_bn: cap,
        target_days,
        automatic,
        previous,
        receipt,
    });
    Ok(())
}

/// The maintenance owner applies this expressly reviewed effective ceiling
/// without a second invoice. Editing cannot reopen a settled day.
pub fn air_support_maintenance_limit(n: &Nation, day: i32) -> Option<f64> {
    active_air_support(n.equipment.as_ref()?.air_support.as_ref()?, day)
        .filter(|p| p.automatic)
        .map(|p| p.daily_budget_bn)
}

fn air_support_maintenance_paid(n: &Nation, day: i32) -> f64 {
    n.equipment
        .as_ref()
        .and_then(|s| s.maintenance_plan.as_ref())
        .and_then(|p| p.receipt.as_ref())
        .filter(|r| r.day == day)
        .map_or(0.0, |r| r.custom_paid_bn + r.legacy_paid_bn)
}

fn air_support_families(w: &WorldState, id: NationId, target_days: u32) -> Vec<AirSupportFamily> {
    let n = w.nation(id);
    let mut targets: BTreeMap<String, (u64, f64)> = BTreeMap::new();
    for h in &n.arsenal.held {
        let Some(air) = h
            .design_id
            .as_deref()
            .and_then(|r| profile(n, r))
            .and_then(|p| p.aviation.as_ref())
        else {
            continue;
        };
        let aircraft = crate::arsenal::available_design_units(h) as u64;
        if aircraft == 0 {
            continue;
        }
        let target = targets.entry(air.store_family.clone()).or_default();
        target.0 = target.0.saturating_add(aircraft);
        target.1 += aircraft as f64
            * air.sorties_per_aircraft_month
            * air.stores_per_sortie
            * target_days as f64
            / 30.0;
    }
    let ammunition = n.equipment.as_ref().and_then(|s| s.ammunition.as_ref());
    targets.into_iter().map(|(family, (aircraft, target))| {
        let stock = ammunition.and_then(|s| s.stocks.get(&family)).copied().unwrap_or(0.0);
        let inbound = crate::companies::ammo_inbound_units(w, id, &family);
        let public_committed = ammunition.map_or(0, |s| s.orders.iter()
            .filter(|p| p.family == family && !ammunition_ended(p))
            .map(|p| p.quantity.saturating_sub(p.completed_rounds) as u64).sum());
        let target_stores = target.ceil().min(MAX_AMMO_RESERVE_TARGET as f64) as u32;
        let gap = reserve_gap(target_stores, stock, inbound.saturating_add(public_committed));
        AirSupportFamily {
            family, aircraft, target_stores, stock, inbound, public_committed, gap, bought: 0,
            reason: if gap == 0 {
                "Stock and already commissioned deliveries cover this planning target.".into()
            } else {
                "Compatible finished stores are needed; only existing allowed supplier stock can be purchased.".into()
            },
        }
    }).collect()
}

pub fn air_support_status(w: &WorldState, id: NationId) -> AirSupportStatus {
    let n = w.nation(id);
    let day = clock::absolute_day(w);
    let policy = n.equipment.as_ref().and_then(|s| s.air_support.as_ref());
    let receipt = policy
        .and_then(|p| p.receipt.as_ref())
        .filter(|r| r.day == day);
    let cap = policy.map_or(0.0, |p| p.daily_budget_bn);
    let active = policy.and_then(|p| active_air_support(p, day));
    let pending = policy
        .filter(|p| p.from_day > day)
        .map(|p| AirSupportTerms {
            from_day: p.from_day,
            daily_budget_bn: p.daily_budget_bn,
            target_days: p.target_days,
            automatic: p.automatic,
        });
    let active_cap = active
        .as_ref()
        .filter(|p| p.automatic)
        .map_or(0.0, |p| p.daily_budget_bn);
    let maintenance = air_support_maintenance_paid(n, day);
    let stores = receipt.map_or(0.0, |r| r.stores_paid_bn);
    AirSupportStatus {
        active,
        pending,
        automatic: policy.is_some_and(|p| p.automatic),
        from_day: policy.map(|p| p.from_day), daily_budget_bn: cap,
        target_days: policy.map_or(30, |p| p.target_days),
        maintenance_required_bn: fleet_maintenance_requirement(n) + legacy_maintenance_requirement(n),
        maintenance_paid_bn: maintenance, stores_paid_bn: stores,
        remaining_bn: (active_cap - maintenance - stores).max(0.0),
        families: air_support_families(w, id, policy.map_or(30, |p| p.target_days)),
        reason: match policy {
            None => "Routine aircraft support is not configured.".into(),
            Some(p) if p.from_day > day => "The reviewed support settings take effect on the next simulation day. Today's prior authorization and recorded payments remain in force.".into(),
            Some(p) if !p.automatic => "Automatic aircraft-store purchases are off. Existing maintenance and paid deliveries continue under their recorded instructions.".into(),
            Some(_) => receipt.map_or_else(|| "Awaiting today's actual fleet-maintenance invoice and bounded store review.".into(), |r| r.reason.clone()),
        },
        detail: AIR_SUPPORT_DETAIL,
    }
}

// Turnaround repairs are dated work, not purchases or rejuvenation. A full
// paid fleet invoice advances at most one service day, even on repeated hooks.
fn tick_aircraft_service(w: &mut WorldState, id: NationId, day: i32) {
    let n = w.nation(id);
    if n.aviation
        .as_ref()
        .is_none_or(|a| a.last_service_day.is_some_and(|d| d >= day))
    {
        return;
    }
    let fully_funded = n
        .equipment
        .as_ref()
        .and_then(|s| s.maintenance_plan.as_ref())
        .and_then(|p| p.receipt.as_ref())
        .is_some_and(|r| {
            let required = r.custom_required_bn + r.legacy_required_bn;
            r.day == day
                && required > 0.0
                && r.custom_paid_bn + r.legacy_paid_bn > 0.0
                && r.custom_paid_bn + r.legacy_paid_bn + 1e-12 >= required
        });
    let eligible: BTreeSet<u32> = n
        .aviation
        .as_ref()
        .unwrap()
        .squadrons
        .iter()
        .filter(|s| {
            if s.service_days_left == 0 || s.transit.is_some() {
                return false;
            }
            let mut geographic = (*s).clone();
            geographic.service_days_left = 0;
            crate::airbases::squadron_blocker(w, id, &geographic).is_none()
        })
        .map(|s| s.id)
        .collect();
    let a = w.nation_mut(id).aviation.as_mut().unwrap();
    a.last_service_day = Some(day);
    if fully_funded {
        for squadron in &mut a.squadrons {
            if eligible.contains(&squadron.id) {
                squadron.service_days_left = squadron.service_days_left.saturating_sub(1);
            }
        }
    }
}

/// Pre-fiscal hook, after the existing upkeep and public fabrication owners.
/// Each future active date permits one bounded review with no catch-up credit.
pub fn tick_air_support(w: &mut WorldState) {
    if !clock::is_daily(w) || !w.rules.military_operations {
        return;
    }
    let day = clock::absolute_day(w);
    let ids: Vec<_> = w
        .nations
        .iter()
        .filter(|n| {
            n.alive
                && n.program_budget
                    .as_ref()
                    .is_some_and(|p| p.day == Some(day) && p.settled_day != Some(day))
        })
        .map(|n| n.id)
        .collect();
    for id in ids {
        tick_aircraft_service(w, id, day);
        let Some(policy) = w
            .nation(id)
            .equipment
            .as_ref()
            .and_then(|s| s.air_support.as_ref())
            .filter(|p| p.receipt.as_ref().is_none_or(|r| r.day < day))
            .and_then(|p| active_air_support(p, day))
            .filter(|p| p.automatic)
        else {
            continue;
        };
        if validate_state(w.nation(id)).is_err() {
            continue;
        }
        let maintenance_receipt = w
            .nation(id)
            .equipment
            .as_ref()
            .and_then(|s| s.maintenance_plan.as_ref())
            .and_then(|p| p.receipt.as_ref());
        if maintenance_receipt.is_none_or(|r| r.day != day) {
            continue;
        }
        let maintenance = air_support_maintenance_paid(w.nation(id), day);
        let upkeep_required =
            maintenance_receipt.map_or(0.0, |r| r.custom_required_bn + r.legacy_required_bn);
        let upkeep_funded = maintenance + 1e-12 >= upkeep_required;
        let mut remaining = if upkeep_funded {
            (policy.daily_budget_bn - maintenance).max(0.0)
        } else {
            0.0
        };
        let mut receipt = AirSupportReceipt {
            day,
            authorized_from_day: policy.from_day,
            daily_budget_bn: policy.daily_budget_bn,
            maintenance_paid_bn: maintenance,
            stores_paid_bn: 0.0,
            purchases: vec![],
            families: air_support_families(w, id, policy.target_days),
            reason: String::new(),
        };
        for row in &mut receipt.families {
            if row.gap == 0 {
                continue;
            }
            if remaining <= 0.0 {
                row.reason = if upkeep_funded { "The daily support cap is exhausted after fleet upkeep; no additional store purchase is authorized." } else { "Fleet upkeep remains underfunded. Routine support will buy no stores until the dated maintenance invoice is fully funded." }.into();
                continue;
            }
            let mut offers = crate::companies::domestic_offers(w, id);
            offers.extend(crate::companies::import_offers(w, id));
            offers.retain(|o| o.ammunition && o.family.as_deref() == Some(row.family.as_str()));
            offers.sort_by(|a, b| {
                a.unit_price_bn.total_cmp(&b.unit_price_bn).then_with(|| {
                    (a.seller, a.company, a.product).cmp(&(b.seller, b.company, b.product))
                })
            });
            let mut blocker = None;
            for offer in offers {
                if row.bought >= row.gap {
                    break;
                }
                if !offer.allowed {
                    if blocker.is_none() {
                        blocker = offer.reason;
                    }
                    continue;
                }
                if !offer.unit_price_bn.is_finite() || offer.unit_price_bn <= 0.0 {
                    continue;
                }
                let current_available = crate::companies::ammo_purchase_funding(w, id).1;
                let allowed = remaining.min(current_available);
                let mut quantity =
                    (allowed / offer.unit_price_bn).floor().min(u32::MAX as f64) as u32;
                quantity = quantity
                    .min(row.gap - row.bought)
                    .min(offer.ready_stock)
                    .min(offer.affordable_quantity)
                    .min(crate::companies::MAX_AMMO_STOCK);
                while quantity > 0 && offer.unit_price_bn * quantity as f64 > allowed {
                    quantity -= 1;
                }
                if quantity == 0 {
                    blocker = Some("The remaining support cap or Defense authority cannot pay for one compatible store.".into());
                    continue;
                }
                let imported = offer.seller != id;
                let quote = if imported {
                    crate::companies::import_purchase_quote(
                        w,
                        id,
                        offer.seller,
                        offer.company,
                        offer.product,
                        true,
                        quantity,
                    )
                } else {
                    crate::companies::ammo_purchase_quote(
                        w,
                        id,
                        offer.company,
                        offer.product,
                        quantity,
                    )
                };
                if !quote.valid || quote.cost_bn > remaining {
                    blocker = quote.reason.or_else(|| {
                        Some("The latest supplier quote exceeds the remaining support cap.".into())
                    });
                    continue;
                }
                let cost = quote.cost_bn;
                let order = if imported {
                    crate::companies::CompanyOrder::ImportPurchase {
                        seller: offer.seller,
                        company: offer.company,
                        product: offer.product,
                        ammunition: true,
                        quantity,
                        quote: quote.token,
                    }
                } else {
                    crate::companies::CompanyOrder::AmmoPurchase {
                        company: offer.company,
                        product: offer.product,
                        quantity,
                        quote: quote.token,
                    }
                };
                match crate::companies::apply(w, id, &order) {
                    Err(reason) => blocker = Some(reason),
                    Ok(()) => {
                        let delivery = if imported {
                            w.companies.imports.contracts.last().unwrap().id
                        } else {
                            w.companies.ammunition_deliveries.last().unwrap().id
                        };
                        receipt.purchases.push(AirSupportPurchase {
                            seller: offer.seller,
                            company: offer.company,
                            product: offer.product,
                            delivery,
                            imported,
                            family: row.family.clone(),
                            quantity,
                            cost_bn: cost,
                        });
                        row.bought += quantity;
                        receipt.stores_paid_bn += cost;
                        remaining = (policy.daily_budget_bn - maintenance - receipt.stores_paid_bn)
                            .max(0.0);
                    }
                }
            }
            row.reason = if row.bought == row.gap {
                "Purchased the uncovered compatible-store target; the ordinary paid delivery must arrive before use.".into()
            } else if let Some(reason) = blocker {
                reason
            } else {
                "No allowed supplier has finished compatible stock. Review suppliers or existing public ammunition work; this policy creates no new manufacturing contract.".into()
            };
        }
        receipt.reason = if receipt.families.iter().any(|r| r.bought < r.gap) {
            "The support review retained an aircraft-store shortage; inspect each family for funding, supplier or delivery limits.".into()
        } else if receipt.purchases.is_empty() {
            "The dated fleet invoice is counted once; available stores and existing commitments need no further purchase.".into()
        } else {
            "Routine support purchased only the compatible finished-store gap within the cap remaining after fleet upkeep.".into()
        };
        state_mut(w.nation_mut(id))
            .air_support
            .as_mut()
            .unwrap()
            .receipt = Some(receipt);
    }
}

pub fn validate_air_support(n: &Nation) -> Result<(), String> {
    let Some(s) = &n.equipment else {
        return Ok(());
    };
    let Some(p) = &s.air_support else {
        return Ok(());
    };
    let invalid = || "Invalid routine aircraft-support policy or dated receipt.".to_string();
    let day = n.program_budget.as_ref().and_then(|p| p.day);
    if n.program_budget.is_none()
        || p.automatic && s.maintenance_plan.is_none()
        || air_support_values_refusal(p.daily_budget_bn * 1000.0, p.target_days).is_some()
        || day.is_some_and(|d| p.from_day > d.saturating_add(1))
    {
        return Err(invalid());
    }
    if let Some(previous) = &p.previous {
        if previous.from_day >= p.from_day
            || air_support_values_refusal(previous.daily_budget_bn * 1000.0, previous.target_days)
                .is_some()
            || previous.automatic && s.maintenance_plan.is_none()
        {
            return Err(invalid());
        }
    }
    let Some(r) = &p.receipt else {
        return Ok(());
    };
    if r.day < r.authorized_from_day
        || day.is_none_or(|d| r.day > d)
        || [r.daily_budget_bn, r.maintenance_paid_bn, r.stores_paid_bn]
            .iter()
            .any(|v| !v.is_finite() || *v < 0.0)
        || r.maintenance_paid_bn + r.stores_paid_bn > r.daily_budget_bn + 1e-12
        || r.families.len() > AMMUNITION_CATALOG.len()
        || r.purchases.len() > crate::companies::MAX_AMMO_DELIVERIES
    {
        return Err(invalid());
    }
    let mut ids = BTreeSet::new();
    let mut bought: BTreeMap<String, u64> = BTreeMap::new();
    let mut paid = 0.0;
    for purchase in &r.purchases {
        if !ids.insert(purchase.delivery)
            || purchase.quantity == 0
            || purchase.quantity > crate::companies::MAX_AMMO_STOCK
            || !purchase.cost_bn.is_finite()
            || purchase.cost_bn <= 0.0
            || purchase.imported != (purchase.seller != n.id)
            || !is_aviation_store(&purchase.family)
        {
            return Err(invalid());
        }
        *bought.entry(purchase.family.clone()).or_default() += purchase.quantity as u64;
        paid += purchase.cost_bn;
    }
    if (paid - r.stores_paid_bn).abs() > 1e-12 {
        return Err(invalid());
    }
    let mut families = BTreeSet::new();
    for row in &r.families {
        if !families.insert(&row.family)
            || row.aircraft == 0
            || !is_aviation_store(&row.family)
            || row.target_stores > MAX_AMMO_RESERVE_TARGET
            || !row.stock.is_finite()
            || row.stock < 0.0
            || row.gap
                != reserve_gap(
                    row.target_stores,
                    row.stock,
                    row.inbound.saturating_add(row.public_committed),
                )
            || row.bought > row.gap
            || row.bought as u64 != bought.remove(&row.family).unwrap_or(0)
        {
            return Err(invalid());
        }
    }
    if !bought.is_empty() {
        return Err(invalid());
    }
    Ok(())
}

/// Policy receipts only refer to transactions owned by the ordinary supplier
/// books. A malformed save cannot substitute an unrelated delivery or payment.
pub fn validate_air_support_world(w: &WorldState) -> Result<(), String> {
    for n in &w.nations {
        validate_air_support(n)?;
        let Some(receipt) = n
            .equipment
            .as_ref()
            .and_then(|s| s.air_support.as_ref())
            .and_then(|p| p.receipt.as_ref())
        else {
            continue;
        };
        for p in &receipt.purchases {
            let matches = if p.imported {
                w.companies.imports.contracts.iter().any(|d| {
                    d.id == p.delivery
                        && d.buyer == n.id
                        && d.seller == p.seller
                        && d.company == p.company
                        && d.product == p.product
                        && d.ammunition
                        && d.family.as_deref() == Some(p.family.as_str())
                        && d.quantity == p.quantity
                        && d.purchased_day == receipt.day
                        && d.total_price_bn == p.cost_bn
                })
            } else {
                w.companies.ammunition_deliveries.iter().any(|d| {
                    d.id == p.delivery
                        && d.buyer == n.id
                        && p.seller == n.id
                        && d.company == p.company
                        && d.product == p.product
                        && d.family == p.family
                        && d.quantity == p.quantity
                        && d.purchased_day == receipt.day
                        && d.total_price_bn == p.cost_bn
                })
            };
            if !matches {
                return Err(
                    "Routine aircraft-support receipt has no matching paid supplier delivery."
                        .into(),
                );
            }
        }
    }
    Ok(())
}

#[cfg(test)]
include!("equipment_air_support_tests.rs");
