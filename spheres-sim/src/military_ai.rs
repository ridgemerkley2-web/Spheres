//! Deterministic staff policy, not a second equipment or finance owner.
//! Every order passes the same native command/quote boundary as a player order.
use crate::{
    airbases as ab, airmissions as am, arsenal, aviation as av, clock, companies as co,
    economic_ai, equipment as eq, programs,
    world::{NationId, WorldState, BUDGET_DEFENSE as DEF},
    Command, EquipmentOrder,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const REVIEW_DAYS: i32 = 30;
pub const NOTE: &str = "Staff review acquisition, research, support and basing every 30 days; combat orders are reviewed daily. Countries buy finished stock, wait for delivery and pay ordinary upkeep and ammunition costs. No difficulty multiplier, free equipment, free research, instant transfers or privileged access is applied. Small economies use smaller fleet targets. Existing supplier programmes remain separate paid owners. Player countries are never directed by this staff.";
const AIR: [&str; 3] = ["air_light_attack", "air_fighter", "air_tactical_strike"];
const RESERVE_ORDERS: usize = 256;

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct MilitaryAi {
    pub enabled: bool,
    pub plans: BTreeMap<NationId, Plan>,
}
impl MilitaryAi {
    pub fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Plan {
    pub last_review_day: Option<i32>,
    pub last_operations_day: Option<i32>,
    pub reviews: u32,
    pub procurement: String,
    pub development: String,
    pub support: String,
    pub basing: String,
    pub operations: String,
    pub purchase_limit_bn: f64,
    pub committed_bn: f64,
}

fn requirements(w: &WorldState) -> Option<String> {
    (!clock::is_daily(w) || !w.rules.production_system || !w.rules.manufacturing_system
        || !w.rules.resource_market || !w.rules.military_operations
        || w.rules.operational_warfare != 1)
        .then(|| "Military staff require the daily construction, resource, manufacturing and operational campaign rules.".into())
}
pub fn enable(w: &mut WorldState) -> Result<(), String> {
    if let Some(why) = requirements(w) {
        return Err(why);
    }
    w.military_ai.enabled = true;
    Ok(())
}
pub fn refusal(w: &WorldState, nation: NationId, enabled: bool) -> Option<String> {
    (w.player != Some(nation))
        .then(|| "Only the player can change campaign military staff settings.".into())
        .or_else(|| enabled.then(|| requirements(w)).flatten())
}
pub fn configure(w: &mut WorldState, nation: NationId, enabled: bool) -> Result<(), String> {
    if let Some(why) = refusal(w, nation, enabled) {
        return Err(why);
    }
    w.military_ai.enabled = enabled;
    Ok(())
}
pub fn validate(w: &WorldState) -> Result<(), String> {
    if w.military_ai.enabled {
        if let Some(why) = requirements(w) {
            return Err(why);
        }
    }
    let today = clock::absolute_day(w);
    for (id, p) in &w.military_ai.plans {
        if w.nation_opt(*id).is_none()
            || [p.last_review_day, p.last_operations_day]
                .into_iter()
                .flatten()
                .any(|d| d > today)
            || [p.purchase_limit_bn, p.committed_bn]
                .into_iter()
                .any(|v| !v.is_finite() || v < 0.0)
            || p.committed_bn > p.purchase_limit_bn + 1e-9
            || [
                &p.procurement,
                &p.development,
                &p.support,
                &p.basing,
                &p.operations,
            ]
            .into_iter()
            .any(|s| s.len() > 4000)
        {
            return Err("Invalid saved military staff review.".into());
        }
    }
    Ok(())
}
fn directed(w: &WorldState, n: NationId) -> bool {
    w.military_ai.enabled
        && economic_ai::enabled(w)
        && w.player != Some(n)
        && w.nation_opt(n).is_some_and(|n| n.alive)
}
fn act(w: &mut WorldState, n: NationId, order: co::CompanyOrder) -> Result<(), String> {
    crate::apply_command(w, &Command::Company { nation: n, order })
}
fn equipment(w: &mut WorldState, n: NationId, order: EquipmentOrder) -> Result<(), String> {
    crate::apply_command(w, &Command::Equipment { nation: n, order })
}
fn upkeep(w: &WorldState, n: NationId) -> f64 {
    eq::fleet_maintenance_requirement(w.nation(n)) + eq::legacy_maintenance_requirement(w.nation(n))
}
fn pending_upkeep(w: &WorldState, n: NationId) -> f64 {
    w.companies
        .deliveries
        .iter()
        .filter(|d| d.buyer == n && d.delivered_day.is_none())
        .filter_map(|d| {
            eq::profile(w.nation(n), &d.revision_id)
                .map(|p| p.maintenance_bn_day * d.quantity as f64)
        })
        .sum::<f64>()
        + w.companies
            .imports
            .contracts
            .iter()
            .filter(|d| {
                d.buyer == n
                    && !d.ammunition
                    && d.delivered_day.is_none()
                    && d.cancelled_day.is_none()
            })
            .map(|d| d.source_revision.profile.maintenance_bn_day * d.quantity as f64)
            .sum::<f64>()
}
fn support_authority(w: &WorldState, n: NationId) -> f64 {
    let nat = w.nation(n);
    nat.program_budget.as_ref().map_or(0.0, |p| {
        nat.gdp.max(0.0) * nat.budget_for(w.year).allocations[DEF] * p.departments[DEF][2] as f64
            / 10000.0
            / 365.0
    })
}
fn target(w: &WorldState, n: NationId) -> u32 {
    // GDP changes only ambition. Every actual purchase must still be affordable.
    if w.nation(n).gdp >= 100.0 {
        4
    } else {
        1
    }
}
fn cash_available(w: &WorldState, n: NationId) -> f64 {
    let nat = w.nation(n);
    let pending_bill = nat.program_budget.as_ref().map_or(0.0, |p| {
        (p.spent_today_bn.iter().flatten().sum::<f64>() + p.interest_today_bn - p.revenue_today_bn)
            .max(0.0)
    });
    let reserve = (upkeep(w, n) * 30.0).max(nat.gdp.max(0.0) * 0.00001);
    (nat.treasury_bn.unwrap_or(0.0) - pending_bill - reserve).max(0.0)
}
fn review_limit(w: &WorldState, n: NationId) -> f64 {
    (programs::available_bn(w, n, DEF, 3) * 0.25).min(cash_available(w, n))
}
fn category(platform: &str) -> &str {
    if eq::is_aviation_platform(platform) {
        platform
    } else {
        "ground"
    }
}
fn committed_units(w: &WorldState, n: NationId, class: &str) -> u32 {
    let nation = w.nation(n);
    let mut total = 0u32;
    if let Some(s) = &nation.equipment {
        for (id, r) in &s.revisions {
            if category(&r.spec.platform) != class {
                continue;
            }
            total = total.saturating_add(
                nation
                    .arsenal
                    .held
                    .iter()
                    .filter(|h| h.design_id.as_deref() == Some(id))
                    // Refit reservations remain owned and count once here.
                    .map(|h| h.units.max(0.0) as u32)
                    .fold(0u32, u32::saturating_add),
            );
        }
    }
    for d in w
        .companies
        .deliveries
        .iter()
        .filter(|d| d.buyer == n && d.delivered_day.is_none())
    {
        if nation
            .equipment
            .as_ref()
            .and_then(|s| s.revisions.get(&d.revision_id))
            .is_some_and(|r| category(&r.spec.platform) == class)
        {
            total = total.saturating_add(d.quantity);
        }
    }
    for d in w.companies.imports.contracts.iter().filter(|d| {
        d.buyer == n && !d.ammunition && d.delivered_day.is_none() && d.cancelled_day.is_none()
    }) {
        if category(&d.source_revision.spec.platform) == class {
            total = total.saturating_add(d.quantity);
        }
    }
    total
}
fn rebalance_support(w: &mut WorldState, n: NationId) -> Result<bool, String> {
    let need = (upkeep(w, n) + pending_upkeep(w, n)) * 1.05;
    if support_authority(w, n) + 1e-10 >= need {
        return Ok(false);
    }
    let nat = w.nation(n);
    let Some(plan) = &nat.program_budget else {
        return Ok(false);
    };
    let allocations = nat.budget_for(w.year).allocations;
    let annual = nat.gdp.max(0.0) * allocations[DEF];
    if annual <= 0.0 {
        return Ok(false);
    }
    let mut departments = plan.departments;
    // Protect personnel and operations, retain a research/procurement floor,
    // and never raise the ministry's total allocation or today's cash authority.
    let desired = (need * 365.0 / annual * 10000.0).ceil().min(5000.0) as u16;
    let mut missing = desired.saturating_sub(departments[DEF][2]);
    for source in [3, 4] {
        let moved = missing.min(departments[DEF][source].saturating_sub(500));
        departments[DEF][source] -= moved;
        departments[DEF][2] += moved;
        missing -= moved;
    }
    if departments == plan.departments {
        return Ok(false);
    }
    let command = Command::SetProgramBudget {
        nation: n,
        fiscal_year: w.year,
        allocations,
        departments,
    };
    if let Some((_, cost, _)) = crate::command_price(w, &command) {
        if cost > 0.0 && nat.political_capital < cost + 8.0 {
            return Err(format!("Maintenance needs a Defense reallocation; saving political capital for its {:.1} cost plus 8 reserve. Current support remains limited by paid upkeep.", cost));
        }
    }
    crate::apply_command(w, &command)?;
    Ok(true)
}

fn support(w: &mut WorldState, n: NationId) -> Result<String, String> {
    let rebalanced = rebalance_support(w, n)?;
    let cap = support_authority(w, n).min(1000.0);
    let current = w
        .nation(n)
        .equipment
        .as_ref()
        .and_then(|s| s.air_support.as_ref());
    if current.is_none_or(|p| {
        !p.automatic || (p.daily_budget_bn - cap).abs() > 1e-9 || p.target_days != 30
    }) {
        equipment(
            w,
            n,
            EquipmentOrder::AirSupport {
                daily_budget_mn: cap * 1000.0,
                target_days: 30,
                automatic: true,
            },
        )?;
    }
    let has_ground = w.nation(n).arsenal.held.iter().any(|h| {
        arsenal::available_design_units(h) > 0
            && h.design_id
                .as_ref()
                .and_then(|id| w.nation(n).equipment.as_ref()?.revisions.get(id))
                .is_some_and(|r| {
                    !eq::is_aviation_platform(&r.spec.platform)
                        && eq::ammunition_family(&r.spec).is_some()
                })
    });
    if has_ground
        && w.nation(n)
            .equipment
            .as_ref()
            .and_then(|s| s.ammunition.as_ref())
            .is_none_or(|a| a.active_from_day.is_none())
    {
        equipment(w, n, EquipmentOrder::AmmoActivate)?;
    }
    let result: String = if cap + 1e-10 < upkeep(w, n) {
        "Existing Maintenance & supply allocation cannot cover the whole fleet; new acquisitions wait. Readiness remains limited by actual payment.".into()
    } else {
        "Ordinary next-day support cap: fleet upkeep first, then eligible finished aircraft stores for 30 days. Purchases await stock, access and delivery.".into()
    };
    Ok(if rebalanced {
        format!("Shifted future Defense funding toward maintenance; total spending and today's authority are unchanged. {result}")
    } else {
        result
    })
}

fn ground_stores(w: &mut WorldState, n: NationId, allowance: &mut f64) -> Result<String, String> {
    let mut families = BTreeMap::<String, f64>::new();
    for h in &w.nation(n).arsenal.held {
        let Some(r) = h
            .design_id
            .as_ref()
            .and_then(|id| w.nation(n).equipment.as_ref()?.revisions.get(id))
        else {
            continue;
        };
        if eq::is_aviation_platform(&r.spec.platform) {
            continue;
        }
        if let Some(family) = eq::ammunition_family(&r.spec) {
            *families.entry(family.into()).or_default() += arsenal::available_design_units(h)
                as f64
                * eq::ammo_def(family).unwrap().rounds_per_vehicle_month;
        }
    }
    if families.is_empty() {
        return Ok("No custom ground ammunition requirement.".into());
    }
    let mut offers = co::domestic_offers(w, n);
    offers.extend(co::import_offers(w, n));
    offers.retain(|o| o.ammunition && o.ready_stock > 0);
    offers.sort_by(|a, b| {
        a.unit_price_bn
            .total_cmp(&b.unit_price_bn)
            .then_with(|| (a.seller, a.company, a.product).cmp(&(b.seller, b.company, b.product)))
    });
    let mut why="Ground stores wait for compatible finished stock; depleted vehicles receive no substitute rounds.".to_string();
    for o in offers {
        let Some(family) = o.family.as_ref() else {
            continue;
        };
        let Some(target) = families.get(family) else {
            continue;
        };
        let status = eq::ammo_reserve_status(w, n, family);
        let gap = (*target - status.stock - status.committed as f64)
            .ceil()
            .max(0.0)
            .min(co::MAX_AMMO_STOCK as f64) as u32;
        if gap == 0 {
            why="Ground ammunition target includes physical stock, inbound deliveries and already commissioned batches.".into();
            continue;
        }
        if !o.allowed {
            why = o.reason.unwrap_or_default();
            continue;
        }
        if o.unit_price_bn <= 0.0 {
            continue;
        }
        let cap = (*allowance).min(co::ammo_purchase_funding(w, n).1 * 0.25);
        let quantity = gap
            .min(o.ready_stock)
            .min((cap / o.unit_price_bn).floor().max(0.0) as u32);
        if quantity == 0 {
            why="Ground ammunition purchase waits for funds after protected upkeep and the review cash reserve.".into();
            continue;
        }
        let q = if o.seller == n {
            co::ammo_purchase_quote(w, n, o.company, o.product, quantity)
        } else {
            co::import_purchase_quote(w, n, o.seller, o.company, o.product, true, quantity)
        };
        if !q.valid {
            why = q.reason.unwrap_or_default();
            continue;
        }
        if q.cost_bn > cap {
            continue;
        }
        let order = if o.seller == n {
            co::CompanyOrder::AmmoPurchase {
                company: o.company,
                product: o.product,
                quantity,
                quote: q.token,
            }
        } else {
            co::CompanyOrder::ImportPurchase {
                seller: o.seller,
                company: o.company,
                product: o.product,
                ammunition: true,
                quantity,
                quote: q.token,
            }
        };
        act(w, n, order)?;
        *allowance = (*allowance - q.cost_bn).max(0.0);
        return Ok(format!(
            "Purchased {quantity} {} from finite stock; normal ammunition delivery is pending.",
            eq::ammo_def(family).unwrap().name
        ));
    }
    Ok(why)
}

fn procure(w: &mut WorldState, n: NationId, allowance: &mut f64) -> Result<String, String> {
    let mut offers = co::domestic_offers(w, n);
    offers.extend(co::import_offers(w, n));
    offers.retain(|o| !o.ammunition && o.ready_stock > 0);
    offers.sort_by(|a, b| {
        // Meet missing roles before growing an existing one, then prefer domestic
        // stock at an equal price. No fabricated supplier or special price.
        let have = |o: &co::ImportOffer| {
            committed_units(w, n, category(o.platform.as_deref().unwrap_or("")))
        };
        have(a)
            .cmp(&have(b))
            .then_with(|| a.unit_price_bn.total_cmp(&b.unit_price_bn))
            .then_with(|| (a.seller != n).cmp(&(b.seller != n)))
            .then_with(|| (a.seller, a.company, a.product).cmp(&(b.seller, b.company, b.product)))
    });
    let mut why="No finished eligible company stock is available. Development and tooling cannot be bypassed.".to_string();
    for o in offers {
        let class = category(o.platform.as_deref().unwrap_or(""));
        let goal = if class == "ground" {
            target(w, n) * 4
        } else {
            target(w, n)
        };
        let gap = goal.saturating_sub(committed_units(w, n, class));
        if gap == 0 {
            why="Fleet targets include owned, refitting and inbound units; no duplicate order is needed.".into();
            continue;
        }
        if !o.allowed {
            why = o
                .reason
                .unwrap_or_else(|| "Supplier access is blocked.".into());
            continue;
        }
        if o.unit_price_bn <= 0.0 {
            continue;
        }
        let quantity = gap
            .min(o.ready_stock)
            .min((*allowance / o.unit_price_bn).floor().max(0.0) as u32);
        if quantity == 0 {
            why="Finished stock exists, but the review spending cap and protected cash reserve cannot afford a whole unit.".into();
            continue;
        }
        let q = if o.seller == n {
            co::purchase_quote(w, n, o.company, o.product, quantity)
        } else {
            co::import_purchase_quote(w, n, o.seller, o.company, o.product, false, quantity)
        };
        if !q.valid {
            why = q.reason.unwrap_or_default();
            continue;
        }
        if upkeep(w, n) + pending_upkeep(w, n) + q.maintenance_bn_day
            > support_authority(w, n) + 1e-10
        {
            why="Purchase deferred: current Maintenance & supply funding cannot sustain this fleet addition.".into();
            continue;
        }
        if q.cost_bn > *allowance {
            continue;
        }
        let order = if o.seller == n {
            co::CompanyOrder::Purchase {
                company: o.company,
                product: o.product,
                quantity,
                quote: q.token,
            }
        } else {
            co::CompanyOrder::ImportPurchase {
                seller: o.seller,
                company: o.company,
                product: o.product,
                ammunition: false,
                quantity,
                quote: q.token,
            }
        };
        act(w, n, order)?;
        *allowance = (*allowance - q.cost_bn).max(0.0);
        return Ok(format!("Bought {quantity} {} from {}. Paid stock is in ordinary delivery; it adds no combat capability before arrival.",o.product_name,o.supplier_name));
    }
    Ok(why)
}

fn rotate_supplier_work(w: &mut WorldState, n: NationId) -> Result<(), String> {
    let Some(cid) = w.supplier_catalogue.plans.get(&n).and_then(|p| p.company) else {
        return Ok(());
    };
    let company = co::company(w, n, cid).ok_or("Managed supplier is missing.")?;
    // A stock target is an ordinary request, not extra capacity. Rotate buffers
    // so an older continuously sold vehicle cannot starve every later product.
    // Native development, paid refits and in-progress units retain priority.
    let mut jobs: Vec<(u32, bool, u32, u32)> = company
        .products
        .iter()
        .filter(|p| p.cancelled_day.is_none())
        .filter_map(|p| {
            w.nation(n)
                .equipment
                .as_ref()?
                .revisions
                .get(&p.revision_id)
                .map(|r| {
                    (
                        p.id,
                        false,
                        crate::supplier_catalogue::stock_target(n, &r.spec.platform)
                            .unwrap_or(target(w, n)),
                        p.stock_target,
                    )
                })
        })
        .chain(company.ammunition_products.iter().filter_map(|p| {
            eq::ammo_def(&p.family).map(|d| {
                (
                    p.id,
                    true,
                    (d.rounds_per_day * 30.0)
                        .ceil()
                        .clamp(1.0, co::MAX_AMMO_STOCK as f64) as u32,
                    p.stock_target,
                )
            })
        }))
        .collect();
    jobs.sort_by_key(|p| p.0);
    if jobs.len() < 2 {
        return Ok(());
    }
    let selected =
        (clock::absolute_day(w).div_euclid(REVIEW_DAYS)).rem_euclid(jobs.len() as i32) as usize;
    for (index, (product, ammunition, goal, current)) in jobs.into_iter().enumerate() {
        let stock_target = if index == selected { goal } else { 0 };
        if stock_target == current {
            continue;
        }
        act(
            w,
            n,
            if ammunition {
                co::CompanyOrder::AmmoInventory {
                    company: cid,
                    product,
                    stock_target,
                }
            } else {
                co::CompanyOrder::Inventory {
                    company: cid,
                    product,
                    stock_target,
                }
            },
        )?;
    }
    Ok(())
}

fn develop(w: &mut WorldState, n: NationId, allowance: &mut f64) -> Result<String, String> {
    // Only a firm created and owned by an autonomous supplier programme may
    // receive staff projects. Never take over an inherited/player-created firm.
    let Some(cid) = w.supplier_catalogue.plans.get(&n).and_then(|p| p.company) else {
        return Ok(
            "Acquire eligible domestic or imported stock; no staff-managed supplier firm exists."
                .into(),
        );
    };
    let company = co::company(w, n, cid).ok_or("The managed supplier is unavailable.")?;
    if let Some(why) = co::facility_blocker(w, company) {
        return Err(why);
    }
    // Finish one added air programme before starting another; never redesign it.
    if let Some(p) = company.products.iter().find(|p| {
        p.cancelled_day.is_none()
            && w.nation(n)
                .equipment
                .as_ref()
                .and_then(|s| s.revisions.get(&p.revision_id))
                .is_some_and(|r| eq::is_aviation_platform(&r.spec.platform))
            && (p.certified_day.is_none() || p.stock < p.stock_target)
    }) {
        let profile = eq::profile(w.nation(n), &p.revision_id).unwrap();
        let needed = co::development_working_capital(w, profile, p.stock_target);
        let covered = company.cash_bn
            + company
                .receivables
                .iter()
                .filter(|r| r.kind == "capitalization")
                .map(|r| r.amount_bn)
                .sum::<f64>();
        if covered + 1e-9 < needed {
            return capitalize(w, n, cid, (needed - covered).min(*allowance), allowance);
        }
        return Ok(format!("Existing programme: {}. {}", p.status, p.reason));
    }
    // Supply compatible ammunition for every certified aircraft product. The
    // catalogue's original ground product remains under its existing policy.
    for p in &company.products {
        let Some(r) = w
            .nation(n)
            .equipment
            .as_ref()
            .and_then(|s| s.revisions.get(&p.revision_id))
        else {
            continue;
        };
        if p.cancelled_day.is_some()
            || p.certified_day.is_none()
            || !eq::is_aviation_platform(&r.spec.platform)
        {
            continue;
        }
        let Some(family) = eq::ammunition_family(&r.spec) else {
            continue;
        };
        let existing = company
            .ammunition_products
            .iter()
            .find(|a| a.family == family);
        if existing.is_some_and(|a| a.stock >= a.stock_target) {
            continue;
        }
        let count = existing.map_or_else(
            || {
                (eq::ammo_def(family).unwrap().rounds_per_day * 30.0)
                    .ceil()
                    .clamp(1.0, co::MAX_AMMO_STOCK as f64) as u32
            },
            |a| a.stock_target,
        );
        let q = co::ammo_supply_quote(w, n, cid, family, count);
        let covered = company.cash_bn
            + company
                .receivables
                .iter()
                .filter(|r| r.kind == "capitalization")
                .map(|r| r.amount_bn)
                .sum::<f64>();
        if covered + 1e-9 < q.company_cash_needed_bn {
            return capitalize(
                w,
                n,
                cid,
                (q.company_cash_needed_bn - covered).min(*allowance),
                allowance,
            );
        }
        if existing.is_none() {
            if !q.valid {
                return Err(q.reason.unwrap_or_default());
            }
            act(
                w,
                n,
                co::CompanyOrder::AmmoSupply {
                    company: cid,
                    family: family.into(),
                    stock_target: count,
                    quote: q.token,
                },
            )?;
            return Ok(
                "Commissioned finite compatible aircraft stores through ordinary paid fabrication."
                    .into(),
            );
        }
    }
    let Some(platform) = AIR.iter().find(|platform| {
        !company.products.iter().any(|p| {
            p.cancelled_day.is_none()
                && w.nation(n)
                    .equipment
                    .as_ref()
                    .and_then(|s| s.revisions.get(&p.revision_id))
                    .is_some_and(|r| r.spec.platform == **platform)
        })
    }) else {
        return Ok("Supported aircraft portfolio is complete. Existing paid stock targets replenish without daily redesigns.".into());
    };
    let spec = eq::default_spec(platform);
    let preview = eq::design_preview(w, n, &spec);
    if !preview.valid {
        if w.nation(n)
            .equipment
            .as_ref()
            .is_some_and(|s| s.active_research.is_some())
        {
            return Ok("Waiting for the selected component research; staff do not switch an unfinished project.".into());
        }
        for key in [
            "air_propulsion_integration",
            "air_mission_systems",
            "air_fighter_integration",
        ] {
            if eq::research_refusal(w, n, key).is_none() {
                if w.nation(n).political_capital < 14.0 {
                    return Err(
                        "Preserving 8 political capital before selecting component research."
                            .into(),
                    );
                }
                equipment(
                    w,
                    n,
                    EquipmentOrder::Research {
                        component: key.into(),
                    },
                )?;
                return Ok(format!("Selected {}. Existing funded Aerospace research must earn every point; no component is granted.",eq::research(key).unwrap().name));
            }
        }
        return Err(preview.blockers.join(" "));
    }
    let profile = preview.profile.unwrap();
    let stock_target = target(w, n);
    let needed = co::development_working_capital(w, &profile, stock_target);
    let covered = company.cash_bn
        + company
            .receivables
            .iter()
            .filter(|r| r.kind == "capitalization")
            .map(|r| r.amount_bn)
            .sum::<f64>();
    if covered + 1e-9 < needed {
        return capitalize(w, n, cid, (needed - covered).min(*allowance), allowance);
    }
    let name = format!("{} {} programme", n.code(), co::platform_name(platform));
    let budget =
        (profile.development_cost_bn / profile.development_days.max(1) as f64).max(0.000001);
    let q = co::development_quote(w, n, cid, &name, &spec, budget, stock_target);
    if !q.valid {
        return Err(q.reason.unwrap_or_default());
    }
    act(
        w,
        n,
        co::CompanyOrder::Develop {
            company: cid,
            name,
            spec,
            daily_budget_bn: budget,
            stock_target,
            quote: q.token,
        },
    )?;
    Ok("Ordered one fixed baseline air design. Paid development, certification, tooling and production precede any sale.".into())
}
fn capitalize(
    w: &mut WorldState,
    n: NationId,
    company: u32,
    amount: f64,
    allowance: &mut f64,
) -> Result<String, String> {
    if amount < 0.000001 {
        return Err("Waiting for the bounded procurement allowance to fund actual supplier working capital.".into());
    }
    let q = co::capitalization_quote(w, n, company, amount);
    if !q.valid {
        return Err(q.reason.unwrap_or_default());
    }
    act(
        w,
        n,
        co::CompanyOrder::Capitalize {
            company,
            amount_bn: amount,
            quote: q.token,
        },
    )?;
    *allowance = (*allowance - amount).max(0.0);
    Ok("Committed limited working capital; the supplier receives it only through fiscal settlement.".into())
}

fn targets(w: &WorldState, n: NationId) -> Vec<(u32, String)> {
    w.conflicts
        .iter()
        .filter(|c| c.posture_of(n).is_some_and(|p| p.rung >= 6))
        .flat_map(|c| {
            crate::front::contested_set(w, c)
                .k
                .into_keys()
                .map(move |d| (c.id, d))
        })
        .filter(|(_, d)| ab::district_location(d).is_some())
        .collect()
}
fn distance_to_front(w: &WorldState, n: NationId, base: &str) -> f64 {
    let Some(b) = ab::district_location(base) else {
        return f64::MAX;
    };
    targets(w, n)
        .iter()
        .filter_map(|(_, d)| ab::district_location(d))
        .map(|t| ab::distance_km(b, t))
        .min_by(f64::total_cmp)
        .unwrap_or(0.0)
}
fn basing(w: &mut WorldState, n: NationId) -> Result<String, String> {
    let revisions: Vec<_> = w
        .nation(n)
        .equipment
        .as_ref()
        .into_iter()
        .flat_map(|s| s.revisions.values())
        .filter(|r| r.certified_day.is_some() && eq::is_aviation_platform(&r.spec.platform))
        .map(|r| r.id.clone())
        .collect();
    for revision in revisions {
        let free = av::unassigned_units(w.nation(n), &revision);
        if free == 0 {
            continue;
        }
        let existing = w.nation(n).aviation.as_ref().and_then(|s| {
            s.squadrons
                .iter()
                .find(|q| q.revision == revision && q.assigned < target(w, n))
        });
        if let Some(s) = existing {
            if s.transit.is_some() || am::busy(w, n, s.id) {
                continue;
            }
            let order = av::SquadronCommand::Resize {
                squadron: s.id,
                quantity: s.assigned + free.min(target(w, n) - s.assigned),
            };
            crate::apply_command(w, &Command::AirSquadron { nation: n, order })?;
            return Ok("Assigned delivered replacement aircraft to an existing squadron; no new holding was created.".into());
        }
        if w.nation(n)
            .aviation
            .as_ref()
            .is_some_and(|a| a.squadrons.len() >= 3)
        {
            continue;
        }
        crate::apply_command(
            w,
            &Command::AirSquadron {
                nation: n,
                order: av::SquadronCommand::Create {
                    name: format!(
                        "{} flight {}",
                        n.code(),
                        w.nation(n)
                            .aviation
                            .as_ref()
                            .map_or(1, |a| a.squadrons.len() + 1)
                    ),
                    revision,
                    quantity: free.min(target(w, n)),
                },
            },
        )?;
        return Ok("Formed a squadron from delivered aircraft. A geographic base and completed transfer are still required.".into());
    }
    let squadrons = w
        .nation(n)
        .aviation
        .as_ref()
        .map(|s| s.squadrons.clone())
        .unwrap_or_default();
    if squadrons.is_empty() {
        return Ok("No delivered aircraft need an airfield; procurement must finish first.".into());
    }
    for sq in &squadrons {
        if sq.assigned == 0 || sq.transit.is_some() || am::busy(w, n, sq.id) {
            continue;
        }
        let mut bases: Vec<_> = w
            .airbases
            .as_ref()
            .into_iter()
            .flat_map(|s| s.bases.iter())
            .filter(|b| ab::assignment_refusal(w, n, Some(sq.id), &b.id, sq.assigned).is_none())
            .map(|b| b.id.clone())
            .collect();
        bases.sort_by(|a, b| {
            distance_to_front(w, n, a)
                .total_cmp(&distance_to_front(w, n, b))
                .then_with(|| a.cmp(b))
        });
        if let Some(best) = bases.first() {
            let current_ok = sq.base.as_ref().is_some_and(|b| {
                ab::assignment_refusal(w, n, Some(sq.id), b, sq.assigned).is_none()
            });
            // Move only for lost access/capacity or a material (>100 km) gain.
            let improve = sq.base.as_ref().is_some_and(|b| {
                distance_to_front(w, n, b) > distance_to_front(w, n, best) + 100.0
            });
            if sq.base.as_ref() != Some(best) && (!current_ok || improve) {
                crate::apply_command(
                    w,
                    &Command::AirBase {
                        nation: n,
                        order: ab::AirbaseCommand::Rebase {
                            squadron: sq.id,
                            base: best.clone(),
                        },
                    },
                )?;
                return Ok("Ordered one transfer to an accessible airfield. Capacity, transit and post-arrival service remain real.".into());
            }
            if current_ok {
                continue;
            }
        }
        if w.airbases.as_ref().is_some_and(|s| {
            s.bases.iter().any(|b| {
                b.project.as_ref().is_some_and(|p| {
                    p.sponsor == n && p.completed_day.is_none() && p.cancelled_day.is_none()
                })
            })
        }) {
            return Ok("Waiting for the existing paid airfield project; no duplicate construction was ordered.".into());
        }
        let cap = (programs::construction_daily_budget_bn(w, n) * 0.25).min(0.002);
        if cap <= 0.0 || cash_available(w, n) < 0.024 {
            return Err(
                "Airfield work waits for construction funding and a protected $24m cash allowance."
                    .into(),
            );
        }
        let mut districts: Vec<_> = w
            .districts
            .iter()
            .filter(|(d, owner)| {
                **owner == n
                    && ab::site_access_refusal(w, n, d).is_none()
                    && ab::district_location(d).is_some()
            })
            .map(|(d, _)| d.clone())
            .collect();
        districts.sort_by(|a, b| {
            distance_to_front(w, n, a)
                .total_cmp(&distance_to_front(w, n, b))
                .then_with(|| a.cmp(b))
        });
        for district in districts {
            let order = if let Some(b) = ab::base(w, &district) {
                if b.capacity_level >= ab::MAX_LEVEL {
                    continue;
                }
                ab::AirbaseCommand::Upgrade {
                    base: district.clone(),
                    track: ab::UpgradeTrack::Capacity,
                    daily_budget_mn: cap * 1000.0,
                }
            } else {
                ab::AirbaseCommand::Establish {
                    district: district.clone(),
                    name: format!(
                        "{} airfield",
                        crate::districts::name_of(&district).unwrap_or(&district)
                    ),
                    daily_budget_mn: cap * 1000.0,
                }
            };
            if let Some(why) = ab::refusal(w, n, &order) {
                let _ = why;
                continue;
            }
            crate::apply_command(w, &Command::AirBase { nation: n, order })?;
            return Ok("Ordered one funded airfield project. No capacity is usable before paid work finishes.".into());
        }
        return Err("No accessible domestic site can house this squadron.".into());
    }
    Ok("Squadrons retain their bases. Transfers, service and access are checked again for every mission.".into())
}

fn review(w: &mut WorldState, n: NationId) {
    let today = clock::absolute_day(w);
    let mut plan = w.military_ai.plans.get(&n).cloned().unwrap_or_default();
    plan.last_review_day = Some(today);
    plan.reviews = plan.reviews.saturating_add(1);
    let setup = (|| -> Result<(), String> {
        if !programs::enrolled(w, n)
            || w.nation(n)
                .program_budget
                .as_ref()
                .is_some_and(|p| p.fiscal_year != w.year)
        {
            crate::apply_command(
                w,
                &Command::SetConstructionBudget {
                    nation: n,
                    daily_budget_bn: programs::construction_daily_budget_bn(w, n),
                },
            )?;
        }
        if !co::imports_enabled(w, n) {
            let q = co::import_enrollment_quote(w, n);
            if !q.valid {
                return Err(q.reason.unwrap_or_default());
            }
            act(w, n, co::CompanyOrder::EnableImports { quote: q.token })?;
        }
        Ok(())
    })();
    if let Err(why) = setup {
        plan.procurement = why;
        w.military_ai.plans.insert(n, plan);
        return;
    }
    plan.support = support(w, n).unwrap_or_else(|why| why);
    let mut allowance = review_limit(w, n);
    plan.purchase_limit_bn = allowance;
    // Delivered stock is useful before starting another long development.
    plan.procurement = procure(w, n, &mut allowance).unwrap_or_else(|why| why);
    let stores = ground_stores(w, n, &mut allowance).unwrap_or_else(|why| why);
    plan.support.push(' ');
    plan.support.push_str(&stores);
    plan.development = rotate_supplier_work(w, n)
        .and_then(|_| develop(w, n, &mut allowance))
        .unwrap_or_else(|why| why);
    plan.committed_bn = (plan.purchase_limit_bn - allowance).max(0.0);
    plan.basing = basing(w, n).unwrap_or_else(|why| why);
    w.military_ai.plans.insert(n, plan);
}

fn operations(w: &mut WorldState, n: NationId, defense: bool) -> String {
    if w.air_missions
        .as_ref()
        .is_some_and(|s| s.orders.len() >= am::MAX_ORDERS - RESERVE_ORDERS)
    {
        return "Automatic missions paused at the shared mission-ledger limit; 256 entries remain reserved for player orders.".into();
    }
    let mut targets = targets(w, n);
    if targets.is_empty() {
        return "No authorized air conflict; staff do not declare or escalate wars.".into();
    }
    if defense {
        // Explicit queued attacks are visible operational orders, not forecasts
        // of future dice rolls. Defend their actual areas before quiet patrols.
        targets.sort_by_key(|(cid, d)| {
            let incoming = w.air_missions.as_ref().is_some_and(|s| {
                s.orders.iter().any(|o| {
                    o.conflict == *cid
                        && o.target == *d
                        && o.status == am::MissionStatus::Queued
                        && o.kind != am::MissionKind::DefendSkies
                        && w.conflict(*cid)
                            .is_some_and(|c| c.side_of(o.nation) != c.side_of(n))
                })
            });
            (!incoming, *cid, d.clone())
        });
    }
    let squadrons = w
        .nation(n)
        .aviation
        .as_ref()
        .map(|s| s.squadrons.clone())
        .unwrap_or_default();
    let mut why = "No delivered squadron of this role is ready.".to_string();
    let mut reason_rank = 0;
    for sq in squadrons {
        let fighter = w
            .nation(n)
            .equipment
            .as_ref()
            .and_then(|s| s.revisions.get(&sq.revision))
            .is_some_and(|r| eq::is_fighter_platform(&r.spec.platform));
        if fighter != defense || sq.assigned == 0 {
            continue;
        }
        for (conflict, target) in &targets {
            let kinds = if defense {
                vec![am::MissionKind::DefendSkies]
            } else {
                vec![am::MissionKind::SupportArmy, am::MissionKind::StrikeTarget]
            };
            for kind in kinds {
                if w.conflict(*conflict)
                    .is_none_or(|c| am::target_reason(w, n, c, kind, target).is_some())
                {
                    continue;
                }
                let q = am::quote(w, n, sq.id, *conflict, kind, target);
                if !q.valid {
                    let rank = if ab::target_in_range(w, n, &sq, target).is_ok() {
                        2
                    } else {
                        1
                    };
                    if rank > reason_rank {
                        why = q.reason.unwrap_or_default();
                        reason_rank = rank;
                    }
                    continue;
                }
                match crate::apply_command(w,&Command::AirMission {nation:n,order:am::MissionCommand::Queue {squadron:sq.id,conflict:*conflict,kind,target:target.clone()}}) {
                    Ok(())=>return format!("{} ordered at {} for the next day. Launch rechecks paid readiness, access, range and finite stores.",kind.name(),crate::districts::name_of(target).unwrap_or(target)),
                    Err(reason)=>why=reason,
                }
            }
        }
    }
    why
}

pub fn tick(w: &mut WorldState) {
    if !w.military_ai.enabled || !economic_ai::enabled(w) {
        return;
    }
    let today = clock::absolute_day(w);
    let mut ids: Vec<_> = w
        .nations
        .iter()
        .filter(|n| directed(w, n.id))
        .map(|n| n.id)
        .collect();
    ids.sort();
    // Stable nation slots distribute expensive catalogue reviews over the month.
    let mut all_ids: Vec<_> = w.nations.iter().map(|n| n.id).collect();
    all_ids.sort();
    let due: Vec<_> = all_ids
        .iter()
        .enumerate()
        .filter_map(|(slot, n)| {
            let prior = w.military_ai.plans.get(n).and_then(|p| p.last_review_day);
            let scheduled = match prior {
                Some(day) => today.saturating_sub(day) >= REVIEW_DAYS,
                None => {
                    w.nation(*n)
                        .aviation
                        .as_ref()
                        .is_some_and(|s| s.squadrons.iter().any(|s| s.assigned > 0))
                        || today.rem_euclid(REVIEW_DAYS) == slot as i32 % REVIEW_DAYS
                }
            };
            (directed(w, *n) && scheduled).then_some(*n)
        })
        .collect();
    for n in due {
        review(w, n);
    }
    if !ids.is_empty() {
        let offset = today.rem_euclid(ids.len() as i32) as usize;
        ids.rotate_left(offset);
    }
    let due: Vec<_> = ids
        .into_iter()
        .filter(|n| {
            w.military_ai
                .plans
                .get(n)
                .is_none_or(|p| p.last_operations_day != Some(today))
        })
        .filter(|n| {
            w.nation(*n)
                .aviation
                .as_ref()
                .is_some_and(|s| !s.squadrons.is_empty())
        })
        .collect();
    for n in &due {
        let result = operations(w, *n, false);
        let p = w.military_ai.plans.entry(*n).or_default();
        p.last_operations_day = Some(today);
        p.operations = result;
    }
    // Both sides' attack orders exist before defensive staff choose an area.
    for n in due {
        if !w
            .nation(n)
            .aviation
            .as_ref()
            .unwrap()
            .squadrons
            .iter()
            .any(|sq| {
                w.nation(n)
                    .equipment
                    .as_ref()
                    .and_then(|s| s.revisions.get(&sq.revision))
                    .is_some_and(|r| eq::is_fighter_platform(&r.spec.platform))
            })
        {
            continue;
        }
        let result = operations(w, n, true);
        let p = w.military_ai.plans.get_mut(&n).unwrap();
        p.operations.push_str(" Defense: ");
        p.operations.push_str(&result);
    }
}

/// Pure observation; refreshing the bureau cannot enroll or order anything.
pub fn view(w: &WorldState) -> serde_json::Value {
    let rows:Vec<_>=w.military_ai.plans.iter().map(|(id,p)|serde_json::json!({
        "nation":id.code(),"nation_name":id.name(),"active":directed(w,*id),"plan":p,
        "next_review_day":p.last_review_day.map(|d|d.saturating_add(REVIEW_DAYS)),"aircraft_target_per_role":target(w,*id)
    })).collect();
    serde_json::json!({"enabled":w.military_ai.enabled,"active":w.military_ai.enabled && economic_ai::enabled(w),"note":NOTE,"countries":rows})
}

#[cfg(test)]
#[path = "military_ai_tests.rs"]
mod tests;
