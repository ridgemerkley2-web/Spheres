//! Explicit supplier operating inputs. The existing company scheduler owns work;
//! this ledger only qualifies and receipts its paid packets. Coefficients are
//! game assumptions, not historical industrial measurements.
use crate::{
    clock,
    companies::{self, Company},
    equipment, industry_operations,
    production::ProjectKind,
    programs, resources,
    world::{NationId, WorldState, BUDGET_DEFENSE},
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

pub const VERSION: u32 = 1;
pub const COMPONENTS_PER_FABRICATION_BN: f64 = 100.0;
pub const COMPONENT_PRICE_BN: f64 = crate::gdp_projects::ADVANCED_COMPONENT_BN;
pub const POWER_PER_WORK_DAY: f64 = 0.35;
const COAL_PER_POWER: f64 = 0.02;

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, deny_unknown_fields)]
pub struct SupplierOperations {
    pub version: u32,
    pub adopted_day: Option<i32>,
    pub grandfathered_units: BTreeSet<u32>,
    pub grandfathered_programs: BTreeSet<u32>,
    pub grandfathered_refits: BTreeSet<u32>,
    pub contracts: BTreeMap<u32, SupplierContract>,
    pub last_day: Option<i32>,
    pub receipts: Vec<SupplierReceipt>,
}
impl SupplierOperations {
    pub fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SupplierContract {
    pub company: u32,
    pub kind: String,
    pub reference: String,
    pub recipe_version: u32,
    pub booked_day: i32,
    pub baseline_units: u32,
    pub components_per_unit: f64,
    pub component_unit_price_bn: f64,
    pub units_started: u32,
    pub components_used: f64,
    pub component_cost_bn: f64,
    pub power_used: f64,
    pub utility_cost_bn: f64,
    pub raw_used: [f64; 12],
    pub input_cost_bn: f64,
    /// Inputs paid during development/tooling are operating expenses, never
    /// part of the finished unit's inventory cost. This is a subset of the
    /// existing input expense, not another charge.
    #[serde(default, skip_serializing_if = "zero_preproduction_inputs")]
    pub preproduction_inputs_bn: f64,
    pub work_days: f64,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SupplierReceipt {
    pub day: i32,
    pub company: u32,
    pub nation: NationId,
    pub district: String,
    pub work_id: u32,
    pub phase: String,
    pub work_days: f64,
    pub units: u32,
    pub components_used: f64,
    pub power_used: f64,
    pub raw_used: [f64; 12],
    pub cash_paid_bn: f64,
}

fn zero_preproduction_inputs(value: &f64) -> bool {
    *value == 0.0
}

pub(crate) fn preproduction_inputs_bn(w: &WorldState, company: u32) -> f64 {
    w.supplier_operations
        .contracts
        .values()
        .filter(|t| t.company == company)
        .map(|t| t.preproduction_inputs_bn)
        .sum()
}

/// Older paid-work books omitted the preproduction classification. Recover only
/// an absent field whose entire saved input bill is provably utility expense
/// before any physical unit started. No current price, clock, payment or work
/// is consulted. A supplied zero is never treated as a migration request.
pub(crate) fn retain_unclassified_preproduction_inputs(
    w: &mut WorldState,
    absent: &BTreeSet<u32>,
) -> Result<(), String> {
    for id in absent {
        let Some(t) = w.supplier_operations.contracts.get(id) else {
            continue;
        };
        if t.kind != "equipment"
            || w.supplier_operations.grandfathered_programs.contains(id)
            || t.input_cost_bn == 0.0
        {
            continue;
        }
        let c = w
            .companies
            .firms
            .iter()
            .find(|c| c.id == t.company)
            .ok_or("The saved supplier input account has no original company.")?;
        let p = c
            .products
            .iter()
            .find(|p| p.id == *id && p.revision_id == t.reference)
            .ok_or("The saved supplier input account has no original product.")?;
        let proven = t.baseline_units == 0
            && t.units_started == 0
            && p.produced_units == 0
            && p.sold_units == 0
            && p.stock == 0
            && p.stock_cost_bn == 0.0
            && p.unit_work_days == 0.0
            && p.unit_spent_bn == 0.0
            && p.unit_material_cost_bn == 0.0
            && p.unit_inputs.iter().all(|v| *v == 0.0)
            && t.components_used == 0.0
            && t.component_cost_bn == 0.0
            && t.raw_used
                .iter()
                .enumerate()
                .all(|(k, v)| k == resources::Commodity::Coal.idx() || *v == 0.0)
            && t.input_cost_bn.is_finite()
            && t.input_cost_bn > 0.0
            && near(t.input_cost_bn, t.utility_cost_bn);
        if !proven {
            return Err(format!("Saved supplier programme {id} has no preproduction expense classification, and its physical inventory or inputs make that ownership ambiguous. Keep the original archive; this build cannot safely infer those older costs."));
        }
        let amount = t.input_cost_bn;
        w.supplier_operations
            .contracts
            .get_mut(id)
            .unwrap()
            .preproduction_inputs_bn = amount;
    }
    Ok(())
}

pub fn enabled(w: &WorldState) -> bool {
    w.supplier_operations.version == VERSION
}
pub fn enable(w: &mut WorldState) -> Result<(), String> {
    if enabled(w) {
        return validate(w);
    }
    if !w.supplier_operations.is_empty() {
        return Err("The supplier operating ledger has unsupported or incomplete data.".into());
    }
    if !clock::is_daily(w)
        || !industry_operations::enabled(w)
        || !crate::population::active(w)
        || !w.rules.resource_market
        || w.resources.market.is_none()
    {
        return Err("Supplier operations require the daily connected economy and an existing physical resource market.".into());
    }
    let mut state = SupplierOperations {
        version: VERSION,
        adopted_day: Some(clock::absolute_day(w)),
        ..Default::default()
    };
    for c in &w.companies.firms {
        for p in &c.products {
            state.grandfathered_programs.insert(p.id);
            if p.unit_work_days > 0.0 || p.unit_inputs.iter().any(|v| *v > 0.0) {
                state.grandfathered_units.insert(p.id);
            }
        }
        state
            .grandfathered_refits
            .extend(c.refits.iter().map(|p| p.id));
    }
    w.supplier_operations = state;
    Ok(())
}
pub fn power_used(w: &WorldState, n: NationId, district: Option<&str>) -> f64 {
    let day = clock::absolute_day(w);
    w.supplier_operations
        .receipts
        .iter()
        .filter(|r| r.day == day && r.nation == n && district.is_none_or(|d| d == r.district))
        .map(|r| r.power_used)
        .sum()
}
pub fn work_fraction(w: &WorldState, c: &Company) -> f64 {
    if !enabled(w) {
        return 1.0;
    }
    if companies::facility_blocker(w, c).is_some() {
        return 0.0;
    }
    if w.supplier_operations
        .receipts
        .iter()
        .any(|r| r.company == c.id && r.day == clock::absolute_day(w))
    {
        return 0.0;
    }
    let (national, local) = industry_operations::remaining_power(w, c.nation, &c.district);
    let power = (national - power_used(w, c.nation, None))
        .max(0.0)
        .min((local - power_used(w, c.nation, Some(&c.district))).max(0.0));
    industry_operations::district_worker_fraction(w, &c.district, ProjectKind::ArmsPlant)
        .clamp(0.0, 1.0)
        .min(power / POWER_PER_WORK_DAY)
        .clamp(0.0, 1.0)
}
pub fn new_refit_inputs_bn(w: &WorldState, labor: f64, days: u32) -> f64 {
    if !enabled(w) {
        return 0.0;
    }
    let power = POWER_PER_WORK_DAY * days as f64;
    components_for(labor) * COMPONENT_PRICE_BN
        + power * industry_operations::ENERGY_CASH_POWER_DAY_BN
        + resources::market_current_price(w, resources::Commodity::Coal) * COAL_PER_POWER * power
            / 1e9
}
pub fn components_for(fabrication_bn: f64) -> f64 {
    fabrication_bn * COMPONENTS_PER_FABRICATION_BN
}
pub(crate) fn ammunition_inputs_bn(w: &WorldState, family: &str) -> f64 {
    if !enabled(w) {
        return 0.0;
    }
    equipment::ammo_def(family).map_or(0.0, |d| {
        let power = POWER_PER_WORK_DAY / d.rounds_per_day;
        ammo_components(family, d.fabrication_bn) * COMPONENT_PRICE_BN
            + power * industry_operations::ENERGY_CASH_POWER_DAY_BN
            + power
                * COAL_PER_POWER
                * resources::market_current_price(w, resources::Commodity::Coal)
                / 1e9
    })
}
fn ammo_components(id: &str, fabrication_bn: f64) -> f64 {
    // Catalogue identities are explicit: "unguided" contains the substring
    // "guided", but ordinary bombs have no guidance electronics recipe.
    if matches!(
        id,
        "howitzer_122_guided" | "howitzer_155_guided" | "aa_missile" | "air_bomb_guided"
    ) {
        components_for(fabrication_bn)
    } else {
        0.0
    }
}

pub(crate) fn applies(w: &WorldState, c: &Company, id: u32) -> bool {
    if !enabled(w) {
        return false;
    }
    if let Some(p) = c.products.iter().find(|p| p.id == id) {
        if w.supplier_operations.grandfathered_units.contains(&id) {
            return false;
        }
        if w.supplier_operations.grandfathered_programs.contains(&id) {
            if let Some(r) = equipment::profile(w.nation(c.nation), &p.revision_id) {
                if p.certified_day.is_none() || p.tooling_work_days + 1e-9 < r.tooling_days as f64 {
                    return false;
                }
            }
        }
    }
    !w.supplier_operations.grandfathered_refits.contains(&id)
}
pub(crate) fn finish_unit(w: &mut WorldState, id: u32) {
    w.supplier_operations.grandfathered_units.remove(&id);
}

/// A pure, exact proposal for the one work item selected by the existing plant
/// scheduler. A proposal is never a reservation, purchase, output or receipt.
#[derive(Clone, Debug)]
pub(crate) struct Packet {
    pub work_id: u32,
    pub kind: String,
    pub reference: String,
    pub phase: String,
    pub baseline_units: u32,
    pub components_per_unit: f64,
    pub starts: u32,
    pub units: u32,
    pub work_days: f64,
    pub raw: [f64; 12],
    pub components: f64,
    pub power: f64,
    pub inputs_cost_bn: f64,
    pub utility_cost_bn: f64,
    pub labor_bn: f64,
    pub public_payment_bn: f64,
    pub cash_required_bn: f64,
    pub reason: Option<String>,
}
impl Packet {
    fn new(id: u32) -> Self {
        Self {
            work_id: id,
            kind: String::new(),
            reference: String::new(),
            phase: String::new(),
            baseline_units: 0,
            components_per_unit: 0.0,
            starts: 0,
            units: 0,
            work_days: 0.0,
            raw: [0.0; 12],
            components: 0.0,
            power: 0.0,
            inputs_cost_bn: 0.0,
            utility_cost_bn: 0.0,
            labor_bn: 0.0,
            public_payment_bn: 0.0,
            cash_required_bn: 0.0,
            reason: None,
        }
    }
}
pub(crate) fn quote_next_packet(w: &WorldState, c: &Company) -> Option<Packet> {
    let id = companies::scheduled_product(c)?;
    if !applies(w, c, id) {
        return None;
    }
    let day = clock::absolute_day(w);
    let mut q = Packet::new(id);
    let fraction = work_fraction(w, c);
    q.reason = companies::facility_blocker(w, c);
    if w.supplier_operations
        .receipts
        .iter()
        .any(|r| r.company == c.id && r.day == day)
    {
        q.reason=Some("This company has already used its one work packet today; the dated receipt records that work.".into());
    }
    let mut reserve = 0.0;
    if let Some(p) = c.products.iter().find(|p| p.id == id) {
        let profile = equipment::profile(w.nation(c.nation), &p.revision_id)?;
        q.kind = "equipment".into();
        q.reference = p.revision_id.clone();
        q.baseline_units = p.produced_units;
        q.components_per_unit = components_for(profile.fabrication_cost_bn);
        if day <= p.started_day {
            q.reason = Some("Work begins after the commissioning date.".into());
        }
        if p.certified_day.is_none() {
            q.phase = "development".into();
            let per = profile.development_cost_bn / profile.development_days.max(1) as f64;
            let left = (profile.development_cost_bn - p.development_spent_bn).max(0.0);
            q.public_payment_bn = (per * fraction)
                .min(left)
                .min(p.daily_budget_bn)
                .min(programs::available_bn(w, c.nation, BUDGET_DEFENSE, 4));
            q.work_days = if per > 0.0 {
                q.public_payment_bn / per
            } else {
                0.0
            };
        } else if p.tooling_work_days + 1e-9 < profile.tooling_days as f64 {
            q.phase = "tooling".into();
            q.work_days =
                fraction.min((profile.tooling_days as f64 - p.tooling_work_days).max(0.0));
            q.labor_bn = (profile.tooling_cost_bn / profile.tooling_days.max(1) as f64
                * q.work_days)
                .min((profile.tooling_cost_bn - p.tooling_spent_bn).max(0.0));
            reserve = q.labor_bn;
        } else {
            q.phase = "manufacturing".into();
            q.work_days =
                fraction.min((profile.production_days as f64 - p.unit_work_days).max(0.0));
            let left = (profile.fabrication_cost_bn - (p.unit_spent_bn - p.unit_material_cost_bn))
                .max(0.0);
            q.labor_bn = (profile.fabrication_cost_bn / profile.production_days.max(1) as f64
                * q.work_days)
                .min(left);
            q.starts =
                u32::from(p.unit_work_days == 0.0 && p.unit_inputs.iter().all(|x| *x == 0.0));
            q.raw = profile.recipe.map(|x| x * q.starts as f64);
            q.components = q.components_per_unit * q.starts as f64;
            reserve = if q.starts > 0 {
                profile.fabrication_cost_bn
            } else {
                q.labor_bn
            };
            q.units =
                u32::from(p.unit_work_days + q.work_days + 1e-9 >= profile.production_days as f64);
        }
    } else if let Some(p) = c.refits.iter().find(|p| p.id == id) {
        q.kind = "refit".into();
        q.reference = p.target_revision.clone();
        q.phase = "refitting".into();
        q.baseline_units = p.completed_units;
        q.components_per_unit = w
            .supplier_operations
            .contracts
            .get(&id)
            .map_or(components_for(p.unit_fabrication_cost_bn), |t| {
                t.components_per_unit
            });
        q.work_days = fraction.min((p.unit_days as f64 - p.unit_work_days).max(0.0));
        q.starts = u32::from(p.unit_started_day.is_none());
        q.raw = p.recipe_per_unit.map(|x| x * q.starts as f64);
        q.components = q.components_per_unit * q.starts as f64;
        q.labor_bn = (p.unit_fabrication_cost_bn / p.unit_days.max(1) as f64 * q.work_days).min(
            if q.starts > 0 {
                p.unit_fabrication_cost_bn
            } else {
                p.working_capital_locked_bn
            },
        );
        reserve = if q.starts > 0 {
            p.unit_fabrication_cost_bn
        } else {
            0.0
        };
        q.units = u32::from(p.unit_work_days + q.work_days + 1e-9 >= p.unit_days as f64);
        if day <= p.booked_day || p.settled_day.is_none_or(|d| day <= d) {
            q.reason = Some("The fixed service deposit must settle before work begins.".into());
        }
    } else if let Some(p) = c.ammunition_products.iter().find(|p| p.id == id) {
        let def = equipment::ammo_def(&p.family)?;
        q.kind = "ammunition".into();
        q.reference = p.family.clone();
        q.phase = "ammunition".into();
        q.baseline_units = p.produced_units;
        q.components_per_unit = ammo_components(&p.family, def.fabrication_bn);
        let mut recipe = def.recipe;
        let power = POWER_PER_WORK_DAY / def.rounds_per_day;
        recipe[resources::Commodity::Coal.idx()] += COAL_PER_POWER * power;
        let input = companies::material_cost(w, &recipe)
            + q.components_per_unit * COMPONENT_PRICE_BN
            + power * industry_operations::ENERGY_CASH_POWER_DAY_BN;
        let mut units = p
            .stock_target
            .saturating_sub(p.stock)
            .min(u32::MAX - p.produced_units)
            .min((def.rounds_per_day * fraction).floor() as u32)
            .min((c.cash_bn / (input + def.fabrication_bn)).floor().max(0.0) as u32);
        for r in resources::ALL {
            if recipe[r.idx()] > 0.0 {
                units = units.min(
                    (resources::stockpile(w, c.nation, r) / recipe[r.idx()])
                        .floor()
                        .max(0.0) as u32,
                );
            }
        }
        if q.components_per_unit > 0.0 {
            units = units.min(
                (industry_operations::advanced_component_stock(w, c.nation) / q.components_per_unit)
                    .floor()
                    .max(0.0) as u32,
            );
        }
        q.units = units;
        q.starts = units;
        q.work_days = units as f64 / def.rounds_per_day;
        // A blocked quote shows the exact minimum whole round, never a fictitious output.
        let proposed = units.max(1);
        q.raw = def.recipe.map(|v| v * proposed as f64);
        q.components = q.components_per_unit * proposed as f64;
        q.labor_bn = def.fabrication_bn * proposed as f64;
        reserve = q.labor_bn;
        if units == 0 {
            q.reason = Some("Staffed power, settled company cash and physical inputs must cover one whole round or store.".into());
        }
        if day <= p.started_day {
            q.reason = Some("Ammunition work begins after its supply agreement date.".into());
        }
    } else {
        return None;
    }
    q.power = q.work_days * POWER_PER_WORK_DAY;
    q.raw[resources::Commodity::Coal.idx()] += q.power * COAL_PER_POWER;
    q.utility_cost_bn = q.power * industry_operations::ENERGY_CASH_POWER_DAY_BN
        + q.power * COAL_PER_POWER * resources::market_current_price(w, resources::Commodity::Coal)
            / 1e9;
    q.inputs_cost_bn = companies::material_cost(w, &q.raw)
        + q.components * COMPONENT_PRICE_BN
        + q.power * industry_operations::ENERGY_CASH_POWER_DAY_BN;
    q.cash_required_bn = q.inputs_cost_bn + reserve;
    if q.work_days <= 0.0 {
        q.reason = q.reason.or_else(|| {
            Some(
                if fraction <= 0.0 {
                    "The leased plant has no qualified staffed power available today."
                } else {
                    "The development funding ceiling or current Defense R&D authority is exhausted."
                }
                .into(),
            )
        });
    }
    for r in resources::ALL {
        if resources::stockpile(w, c.nation, r) < q.raw[r.idx()] {
            q.reason = q.reason.or_else(|| {
                Some(format!(
                    "The government warehouse lacks {} for this complete paid work packet.",
                    r.name()
                ))
            });
        }
    }
    if industry_operations::advanced_component_stock(w, c.nation) < q.components {
        q.reason = q.reason.or_else(|| Some("The government warehouse lacks advanced components. Production and arrived purchases supply this shared finite stock.".into()));
    }
    if c.cash_bn < q.cash_required_bn {
        q.reason = q.reason.or_else(|| Some("Settled company cash must cover operating inputs and the next unit's protected fabrication capital. Public escrow cannot pay this bill.".into()));
    }
    if !c.cash_bn.is_finite()
        || !industry_operations::advanced_component_stock(w, c.nation).is_finite()
        || resources::ALL
            .iter()
            .any(|r| !resources::stockpile(w, c.nation, *r).is_finite())
        || ![
            q.cash_required_bn,
            q.inputs_cost_bn,
            q.components,
            q.power,
            q.work_days,
            q.labor_bn,
            q.public_payment_bn,
        ]
        .into_iter()
        .chain(q.raw)
        .all(|v| v.is_finite() && v >= 0.0)
    {
        q.reason=Some("The supplier cash or physical input ledger is not finite; reconcile it before spending.".into());
    }
    Some(q)
}

pub(crate) fn book_refit(w: &mut WorldState, company: u32, id: u32, reference: &str, labor: f64) {
    if !enabled(w) {
        return;
    }
    let day = clock::absolute_day(w);
    w.supplier_operations.contracts.insert(
        id,
        empty_contract(company, "refit", reference, day, 0, components_for(labor)),
    );
}
fn empty_contract(
    company: u32,
    kind: &str,
    reference: &str,
    day: i32,
    baseline: u32,
    components: f64,
) -> SupplierContract {
    SupplierContract {
        company,
        kind: kind.into(),
        reference: reference.into(),
        recipe_version: VERSION,
        booked_day: day,
        baseline_units: baseline,
        components_per_unit: components,
        component_unit_price_bn: COMPONENT_PRICE_BN,
        units_started: 0,
        components_used: 0.0,
        component_cost_bn: 0.0,
        power_used: 0.0,
        utility_cost_bn: 0.0,
        raw_used: [0.0; 12],
        input_cost_bn: 0.0,
        preproduction_inputs_bn: 0.0,
        work_days: 0.0,
    }
}
/// Caller has preflighted its public payment or Arsenal return. Nothing else
/// mutates these stocks between this complete preflight and its checked debit.
pub(crate) fn consume(w: &mut WorldState, i: usize, q: &Packet) -> Result<(), String> {
    if let Some(reason) = &q.reason {
        return Err(reason.clone());
    }
    let c = &w.companies.firms[i];
    let (company, nation, district) = (c.id, c.nation, c.district.clone());
    if w.supplier_operations
        .receipts
        .iter()
        .any(|r| r.company == company && r.day == clock::absolute_day(w))
        || !c.cash_bn.is_finite()
        || c.cash_bn < q.cash_required_bn
        || ![
            q.cash_required_bn,
            q.inputs_cost_bn,
            q.components,
            q.power,
            q.work_days,
            q.labor_bn,
            q.public_payment_bn,
        ]
        .into_iter()
        .chain(q.raw)
        .all(|v| v.is_finite() && v >= 0.0)
        || !industry_operations::advanced_component_stock(w, nation).is_finite()
        || industry_operations::advanced_component_stock(w, nation) < q.components
        || resources::ALL.iter().any(|r| {
            !resources::stockpile(w, nation, *r).is_finite()
                || resources::stockpile(w, nation, *r) < q.raw[r.idx()]
        })
    {
        return Err(
            "Supplier inputs changed before the paid work packet; no work was posted.".into(),
        );
    }
    // Both operations now have sufficient finite stock. These helpers neither
    // schedule other consumers nor publish money, so the preflight stays valid.
    resources::consume_stockpile_atomic(w, nation, &q.raw)
        .map_err(|(r, _, _)| format!("Missing {}.", r.name()))?;
    industry_operations::take_advanced_components(w, nation, q.components)?;
    let gdp = w.nation(nation).gdp.max(0.1);
    crate::economy::charge_for(
        w,
        nation,
        -q.inputs_cost_bn,
        -q.inputs_cost_bn / gdp,
        crate::fiscal_journal::CashCause::SupplierInputs,
    );
    let c = &mut w.companies.firms[i];
    c.cash_bn = (c.cash_bn - q.inputs_cost_bn).max(0.0);
    c.materials_expense_bn += q.inputs_cost_bn;
    let day = clock::absolute_day(w);
    let state = &mut w.supplier_operations;
    let terms = state.contracts.entry(q.work_id).or_insert_with(|| {
        empty_contract(
            company,
            &q.kind,
            &q.reference,
            day,
            q.baseline_units,
            q.components_per_unit,
        )
    });
    terms.units_started += q.starts;
    terms.components_used += q.components;
    terms.component_cost_bn += q.components * terms.component_unit_price_bn;
    terms.power_used += q.power;
    terms.utility_cost_bn += q.utility_cost_bn;
    terms.input_cost_bn += q.inputs_cost_bn;
    if matches!(q.phase.as_str(), "development" | "tooling") {
        terms.preproduction_inputs_bn += q.inputs_cost_bn;
    }
    terms.work_days += q.work_days;
    for k in 0..12 {
        terms.raw_used[k] += q.raw[k];
    }
    if state.last_day != Some(day) {
        state.receipts.clear();
        state.last_day = Some(day);
    }
    state.receipts.push(SupplierReceipt {
        day,
        company,
        nation,
        district,
        work_id: q.work_id,
        phase: q.phase.clone(),
        work_days: q.work_days,
        units: q.units,
        components_used: q.components,
        power_used: q.power,
        raw_used: q.raw,
        cash_paid_bn: q.inputs_cost_bn + q.labor_bn,
    });
    Ok(())
}

fn near(a: f64, b: f64) -> bool {
    (a - b).abs() <= 1e-9 * a.abs().max(b.abs()).max(1.0)
}
pub fn validate(w: &WorldState) -> Result<(), String> {
    let s = &w.supplier_operations;
    if s.is_empty() {
        return Ok(());
    }
    let day = clock::absolute_day(w);
    let fail =
        || "Invalid supplier operating ownership, recipe, input or dated work ledger.".to_string();
    if s.version != VERSION
        || s.adopted_day.is_none_or(|d| d > day)
        || !clock::is_daily(w)
        || !industry_operations::enabled(w)
        || !crate::population::active(w)
        || !w.rules.resource_market
        || s.last_day
            .is_some_and(|d| d > day || d < s.adopted_day.unwrap_or(day))
        || (s.last_day.is_none() && !s.receipts.is_empty())
    {
        return Err(fail());
    }
    for id in &s.grandfathered_units {
        if !w.companies.firms.iter().flat_map(|c| &c.products).any(|p| {
            p.id == *id && (p.unit_work_days > 0.0 || p.unit_inputs.iter().any(|x| *x > 0.0))
        }) || s.contracts.contains_key(id)
        {
            return Err(fail());
        }
    }
    for id in &s.grandfathered_programs {
        if !w
            .companies
            .firms
            .iter()
            .flat_map(|c| &c.products)
            .any(|p| p.id == *id && p.started_day <= s.adopted_day.unwrap())
        {
            return Err(fail());
        }
    }
    for id in &s.grandfathered_refits {
        if !w
            .companies
            .firms
            .iter()
            .flat_map(|c| &c.refits)
            .any(|p| p.id == *id && p.booked_day <= s.adopted_day.unwrap())
            || s.contracts.contains_key(id)
        {
            return Err(fail());
        }
    }
    for (id, t) in &s.contracts {
        let c = w
            .companies
            .firms
            .iter()
            .find(|c| c.id == t.company)
            .ok_or_else(fail)?;
        let (expected, started, recipe, expected_work) = match t.kind.as_str() {
            "equipment" => {
                let p = c
                    .products
                    .iter()
                    .find(|p| p.id == *id && p.revision_id == t.reference)
                    .ok_or_else(fail)?;
                let profile =
                    equipment::profile(w.nation(c.nation), &p.revision_id).ok_or_else(fail)?;
                (
                    components_for(profile.fabrication_cost_bn),
                    p.produced_units
                        .checked_sub(t.baseline_units)
                        .and_then(|x| {
                            x.checked_add(u32::from(
                                p.unit_work_days > 0.0 || p.unit_inputs.iter().any(|x| *x > 0.0),
                            ))
                        })
                        .ok_or_else(fail)?,
                    profile.recipe,
                    (p.produced_units - t.baseline_units) as f64 * profile.production_days as f64
                        + p.unit_work_days
                        + if s.grandfathered_programs.contains(id) {
                            0.0
                        } else {
                            p.development_work_days + p.tooling_work_days
                        },
                )
            }
            "ammunition" => {
                let p = c
                    .ammunition_products
                    .iter()
                    .find(|p| p.id == *id && p.family == t.reference)
                    .ok_or_else(fail)?;
                (
                    ammo_components(
                        &p.family,
                        equipment::ammo_def(&p.family)
                            .ok_or_else(fail)?
                            .fabrication_bn,
                    ),
                    p.produced_units
                        .checked_sub(t.baseline_units)
                        .ok_or_else(fail)?,
                    equipment::ammo_def(&p.family).ok_or_else(fail)?.recipe,
                    (p.produced_units - t.baseline_units) as f64
                        / equipment::ammo_def(&p.family)
                            .ok_or_else(fail)?
                            .rounds_per_day,
                )
            }
            "refit" => {
                let p = c
                    .refits
                    .iter()
                    .find(|p| p.id == *id && p.target_revision == t.reference)
                    .ok_or_else(fail)?;
                (
                    components_for(p.unit_fabrication_cost_bn),
                    p.completed_units + u32::from(p.unit_started_day.is_some()),
                    p.recipe_per_unit,
                    p.completed_units as f64 * p.unit_days as f64 + p.unit_work_days,
                )
            }
            _ => return Err(fail()),
        };
        if t.recipe_version != VERSION
            || t.booked_day < s.adopted_day.unwrap()
            || t.booked_day > day
            || !near(t.components_per_unit, expected)
            || t.component_unit_price_bn != COMPONENT_PRICE_BN
            || t.units_started != started
            || !near(t.components_used, t.units_started as f64 * expected)
            || !near(
                t.component_cost_bn,
                t.components_used * t.component_unit_price_bn,
            )
            || !near(t.power_used, t.work_days * POWER_PER_WORK_DAY)
            || !near(t.work_days, expected_work)
            || t.raw_used.iter().enumerate().any(|(k, used)| {
                !near(
                    *used,
                    recipe[k] * t.units_started as f64
                        + if k == resources::Commodity::Coal.idx() {
                            t.power_used * COAL_PER_POWER
                        } else {
                            0.0
                        },
                )
            })
            || t.work_days > (day - t.booked_day + 1).max(0) as f64 + 1e-9
            || t.kind == "refit" && t.baseline_units != 0
            || ![
                t.components_per_unit,
                t.components_used,
                t.component_cost_bn,
                t.power_used,
                t.utility_cost_bn,
                t.input_cost_bn,
                t.preproduction_inputs_bn,
                t.work_days,
            ]
            .into_iter()
            .chain(t.raw_used)
            .all(|x| x.is_finite() && x >= 0.0)
            || t.preproduction_inputs_bn > t.input_cost_bn + 1e-9
            || t.preproduction_inputs_bn > t.utility_cost_bn + 1e-9
            || (t.kind != "equipment" || s.grandfathered_programs.contains(id))
                && t.preproduction_inputs_bn != 0.0
            || t.kind == "equipment"
                && t.units_started == 0
                && !near(t.preproduction_inputs_bn, t.input_cost_bn)
            || t.input_cost_bn + 1e-9 < t.component_cost_bn + t.utility_cost_bn
            || t.input_cost_bn > c.materials_expense_bn + 1e-9
        {
            return Err(fail());
        }
    }
    for c in &w.companies.firms {
        if s.contracts
            .values()
            .filter(|t| t.company == c.id)
            .map(|t| t.input_cost_bn)
            .sum::<f64>()
            > c.materials_expense_bn + 1e-9
        {
            return Err(fail());
        }
        for p in &c.products {
            if (p.unit_work_days > 0.0 || p.unit_inputs.iter().any(|x| *x > 0.0))
                && !s.grandfathered_units.contains(&p.id)
                && !s.contracts.contains_key(&p.id)
            {
                return Err(fail());
            }
            if !s.grandfathered_programs.contains(&p.id)
                && p.development_work_days > 0.0
                && !s.contracts.contains_key(&p.id)
            {
                return Err(fail());
            }
        }
        for p in &c.refits {
            if !s.grandfathered_refits.contains(&p.id) && !s.contracts.contains_key(&p.id) {
                return Err(fail());
            }
        }
    }
    let mut companies = BTreeSet::new();
    for r in &s.receipts {
        let t = s.contracts.get(&r.work_id).ok_or_else(fail)?;
        let c = w
            .companies
            .firms
            .iter()
            .find(|c| c.id == r.company)
            .ok_or_else(fail)?;
        if !companies.insert(r.company)
            || r.day != s.last_day.unwrap_or(i32::MIN)
            || t.company != r.company
            || r.day < t.booked_day
            || r.work_days > t.work_days + 1e-9
            || r.components_used > t.components_used + 1e-9
            || r.power_used > t.power_used + 1e-9
            || r.raw_used
                .iter()
                .zip(t.raw_used)
                .any(|(a, b)| *a > b + 1e-9)
            || !match t.kind.as_str() {
                "equipment" => {
                    ["development", "tooling", "manufacturing"].contains(&r.phase.as_str())
                }
                "ammunition" => r.phase == "ammunition",
                "refit" => r.phase == "refitting",
                _ => false,
            }
            || r.nation != c.nation
            || r.district != c.district
            || r.work_days <= 0.0
            || r.work_days > 1.0 + 1e-9
            || !near(r.power_used, r.work_days * POWER_PER_WORK_DAY)
            || ![r.work_days, r.components_used, r.power_used, r.cash_paid_bn]
                .into_iter()
                .chain(r.raw_used)
                .all(|x| x.is_finite() && x >= 0.0)
        {
            return Err(fail());
        }
    }
    Ok(())
}
fn date(day: i32) -> String {
    let (y, m, d) = clock::date_from_day(day);
    format!("{y:04}-{m:02}-{d:02}")
}
pub fn snapshot(w: &WorldState, n: NationId) -> Value {
    let rows: Vec<Value> = w.companies.firms.iter().filter(|c| c.nation == n).map(|c| {
        let selected = companies::scheduled_product(c);
        let q = quote_next_packet(w,c);
        let (national,local) = industry_operations::remaining_power(w,n,&c.district);
        let available = (national-power_used(w,n,None)).max(0.0).min((local-power_used(w,n,Some(&c.district))).max(0.0));
        let mut inputs = Vec::new();
        if let Some(q) = &q {
            for r in resources::ALL { if q.raw[r.idx()] > 0.0 { let stock=resources::stockpile(w,n,r);
                inputs.push(json!({"key":format!("{:?}",r).to_lowercase(),"name":r.name(),"unit":"resource units","required":q.raw[r.idx()],"available":stock,"missing":(q.raw[r.idx()]-stock).max(0.0)})); } }
            let stock=industry_operations::advanced_component_stock(w,n);
            if q.components > 0.0 { inputs.push(json!({"key":"advanced_components","name":"Advanced components","unit":"components","required":q.components,"available":stock,"missing":(q.components-stock).max(0.0)})); }
        }
        let receipt = w.supplier_operations.receipts.iter().rev().find(|r|r.company==c.id).map(|r| {
            let mut v=serde_json::to_value(r).unwrap(); v["date"]=json!(date(r.day)); v
        });
        json!({"company":c.id,"nation":n.code(),"district":c.district,"work_id":selected,
            "work_kind":q.as_ref().map(|q|&q.kind),"phase":q.as_ref().map(|q|&q.phase),
            "grandfathered":enabled(w)&&selected.is_some_and(|id|!applies(w,c,id)),
            "company_cash_available_bn":c.cash_bn,"locked_working_capital_bn":c.refits.iter().map(|p|p.working_capital_locked_bn).sum::<f64>(),
            "public_refit_escrow_bn":c.refits.iter().map(|p|p.escrow_bn).sum::<f64>(),
            "reason":q.as_ref().and_then(|q|q.reason.clone()).or_else(||if selected.is_none(){Some("The finite stock target is met; no work is scheduled.".into())}else if !enabled(w){Some("Supplier operations have not been adopted.".into())}else if q.is_none(){Some("Existing paid work retains its original input and timing terms.".into())}else{None}),
            "readiness":{"slot_available":companies::facility_blocker(w,c).is_none(),"staffing_fraction":industry_operations::district_worker_fraction(w,&c.district,ProjectKind::ArmsPlant),"power_available":available,"power_required":q.as_ref().map_or(0.0,|q|q.power),"work_fraction":work_fraction(w,c)},
            "next_packet":{"work_days":q.as_ref().map_or(0.0,|q|q.work_days),"units":q.as_ref().map_or(0,|q|q.units),"company_cash_required_bn":q.as_ref().map_or(0.0,|q|q.cash_required_bn),"public_development_required_bn":q.as_ref().map_or(0.0,|q|q.public_payment_bn),"inputs":inputs},"last_receipt":receipt})
    }).collect();
    json!({"enabled":enabled(w),"adopted_day":w.supplier_operations.adopted_day,"adopted_date":w.supplier_operations.adopted_day.map(date),
        "as_of_day":clock::absolute_day(w),"component_unit_price_bn":COMPONENT_PRICE_BN,"companies":rows,
        "note":"Available work is a conditional plant estimate. Only dated receipts are paid work. Government warehouse inputs are purchased with settled company cash; company stock and public service escrow remain separately owned. Existing work is not charged again."})
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unguided_stores_and_ordinary_rounds_have_no_guidance_component_recipe() {
        for id in [
            "air_bomb_unguided",
            "tank_120_mixed",
            "howitzer_155_he",
            "aa_cannon",
        ] {
            let d = equipment::ammo_def(id).unwrap();
            assert_eq!(ammo_components(id, d.fabrication_bn), 0.0, "{id}");
        }
        for id in [
            "air_bomb_guided",
            "howitzer_122_guided",
            "howitzer_155_guided",
            "aa_missile",
        ] {
            let d = equipment::ammo_def(id).unwrap();
            assert_eq!(
                ammo_components(id, d.fabrication_bn),
                d.fabrication_bn * COMPONENTS_PER_FABRICATION_BN,
                "{id}"
            );
            assert!(ammo_components(id, d.fabrication_bn) > 0.0);
        }
    }
}
