//! Actual project activity at constant accounting prices, not a sales ledger.
//!
//! The MODEL pricebook values manufactured packs and internal power; raw inputs
//! use the resource table's documented 1990 unit values. Each producer reports
//! gross output less intermediate consumption. Wages are value added, not an
//! intermediate deduction. There is no treasury receipt, profit or tax write.
//! A daily flow is annualized once at 365 days, never accumulated into GDP as
//! if annual output were a cash stock. The province economy owns aggregation.
use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::{
    clock, industry,
    production::{self, Project, ProjectKind as K},
    resources::{self, Commodity as C, ALL},
    world::{NationId, WorldState},
};

pub const DAYS_PER_ACCOUNTING_YEAR: f64 = 365.0;
/// MODEL prices in constant 1990 $bn. These are industrial accounting packs,
/// not geological quantities or claims about historical factory sale prices.
pub const INTERMEDIATE_PACK_BN: f64 = 0.0001;
pub const CAPITAL_PACK_BN: f64 = 0.00025;
pub const POWER_UNIT_BN: f64 = 0.00001;
/// MODEL extraction intermediate-cost share. The resource layer has no mine
/// operating-cost ledger yet; this is an explicit imputation, not cash spent.
pub const EXTRACTION_INTERMEDIATE_SHARE: f64 = 0.30;

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct FlowLedger {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub day: Option<i32>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub receipts: BTreeMap<String, Contribution>,
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub events: BTreeSet<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Contribution {
    pub id: String,
    pub name: String,
    pub district: String,
    pub kind: String,
    pub sector: String,
    pub classification: String,
    pub status: String,
    pub reason: Option<String>,
    pub counted: bool,
    /// Annualized actual daily value added, constant 1990 $bn.
    pub annual_gdp_bn: f64,
    /// Observed value added already contained in the inherited economy. It is
    /// disclosed as production, but excluded from incremental GDP settlement.
    #[serde(default, skip_serializing_if = "is_zero")]
    pub inherited_annual_gdp_bn: f64,
    pub daily_value_added_bn: f64,
    pub gross_output_daily_bn: f64,
    pub intermediate_inputs_daily_bn: f64,
    pub output_quantity_daily: f64,
    pub output_unit: String,
    /// Informational cash already handled by the existing fiscal mechanism.
    pub payments_daily_bn: f64,
    pub valuation_basis: String,
    /// Most actual daily activity has a stable 365-day run-rate. Extraction's
    /// source is annual and its unchanged physical posting uses equal months.
    pub annualization_days: f64,
}

fn is_zero(value: &f64) -> bool { *value == 0.0 }

/// Actual project output not already represented by the inherited baseline.
/// Keep legacy rows on their exact prior arithmetic when no overlap exists.
pub fn incremental_gdp_bn(row: &Contribution) -> f64 {
    if row.inherited_annual_gdp_bn == 0.0 { row.annual_gdp_bn }
    else { (row.annual_gdp_bn - row.inherited_annual_gdp_bn).max(0.0) }
}

pub fn begin_day(w: &mut WorldState) {
    if !crate::province_economy::active(w) {
        return;
    }
    let day = clock::absolute_day(w);
    let f = &mut w
        .province_economy
        .as_mut()
        .expect("active province ledger")
        .flows;
    if f.day == Some(day) {
        return;
    }
    f.day = Some(day);
    f.receipts.clear();
    f.events.clear();
}

fn event(w: &mut WorldState, key: &str) -> bool {
    if !crate::province_economy::active(w) {
        return false;
    }
    begin_day(w);
    w.province_economy
        .as_mut()
        .unwrap()
        .flows
        .events
        .insert(key.into())
}

fn receipt(id: String, name: String, district: &str, kind: &str, sector: &str) -> Contribution {
    Contribution {
        id,
        name,
        district: district.into(),
        kind: kind.into(),
        sector: sector.into(),
        classification: "incremental_value_added".into(),
        status: "active".into(),
        reason: None,
        counted: true,
        annual_gdp_bn: 0.0,
        inherited_annual_gdp_bn: 0.0,
        daily_value_added_bn: 0.0,
        gross_output_daily_bn: 0.0,
        intermediate_inputs_daily_bn: 0.0,
        output_quantity_daily: 0.0,
        output_unit: String::new(),
        payments_daily_bn: 0.0,
        valuation_basis: "Constant 1990 dollars; modeled project accounting, not sales or profit."
            .into(),
        annualization_days: DAYS_PER_ACCOUNTING_YEAR,
    }
}

fn insert(w: &mut WorldState, mut row: Contribution) {
    if !crate::province_economy::active(w) {
        return;
    }
    if row.counted {
        row.daily_value_added_bn = row.gross_output_daily_bn - row.intermediate_inputs_daily_bn;
        row.annual_gdp_bn = row.daily_value_added_bn * row.annualization_days;
    } else {
        row.daily_value_added_bn = 0.0;
        row.annual_gdp_bn = 0.0;
        row.inherited_annual_gdp_bn = 0.0;
    }
    let f = &mut w.province_economy.as_mut().unwrap().flows;
    if let Some(old) = f.receipts.get_mut(&row.id) {
        old.gross_output_daily_bn += row.gross_output_daily_bn;
        old.intermediate_inputs_daily_bn += row.intermediate_inputs_daily_bn;
        old.daily_value_added_bn += row.daily_value_added_bn;
        old.annual_gdp_bn += row.annual_gdp_bn;
        old.inherited_annual_gdp_bn += row.inherited_annual_gdp_bn;
        old.output_quantity_daily += row.output_quantity_daily;
        old.payments_daily_bn += row.payments_daily_bn;
    } else {
        f.receipts.insert(row.id.clone(), row);
    }
}

fn raw_value(raw: &[f64; 12]) -> Result<f64, C> {
    let mut total = 0.0;
    for c in ALL {
        if raw[c.idx()] > 0.0 {
            total += raw[c.idx()] * resources::unit_price_bn(c).ok_or(c)?;
        }
    }
    Ok(total)
}

pub fn record_construction(
    w: &mut WorldState,
    p: &Project,
    advance_days: f64,
    work_bill_bn: f64,
    paid_today: bool,
) {
    let id = format!("construction:{}", p.id);
    if advance_days <= 0.0 || !work_bill_bn.is_finite() || work_bill_bn < 0.0 || !event(w, &id) {
        return;
    }
    let mut row = receipt(
        id,
        format!("{} construction", production::catalog(p.kind).name),
        &p.district,
        p.kind.key(),
        "construction",
    );
    row.status = "building".into();
    row.gross_output_daily_bn = work_bill_bn;
    if !paid_today {
        row.counted = false;
        row.classification = "attributed_legacy_work".into();
        row.status = "attributed".into();
    }
    row.output_quantity_daily = advance_days;
    row.output_unit = "completed project-days".into();
    row.payments_daily_bn = if paid_today { work_bill_bn } else { 0.0 };
    row.reason=Some(if paid_today {"Actual completed installation and labor. Consumed raw and industrial goods are excluded; completion grants no second GDP bonus."}else{"Completed legacy work valued at the modeled installation/labor schedule. This is not a new treasury charge or a retroactive award."}.into());
    insert(w, row);
}

pub fn record_mine_construction(
    w: &mut WorldState,
    p: &resources::MineProject,
    advance_days: f64,
    work_bill_bn: f64,
    paid_today: bool,
) {
    let id = format!("mine_construction:{}:{}", p.district, p.commodity.key());
    if advance_days <= 0.0 || !work_bill_bn.is_finite() || work_bill_bn < 0.0 || !event(w, &id) {
        return;
    }
    let mut row = receipt(
        id,
        format!("{} mine construction", p.commodity.name()),
        &p.district,
        "mine_development",
        "construction",
    );
    row.status = "building".into();
    row.gross_output_daily_bn = work_bill_bn;
    if !paid_today {
        row.counted = false;
        row.classification = "attributed_legacy_work".into();
        row.status = "attributed".into();
    }
    row.output_quantity_daily = advance_days;
    row.output_unit = "completed project-days".into();
    row.payments_daily_bn = if paid_today { work_bill_bn } else { 0.0 };
    row.reason=Some(if paid_today{"Actual installation/labor work, excluding consumed construction inputs. Extraction is recorded separately after completion."}else{"Current work on a legacy prepaid mine, valued at its installation schedule; no new or retroactive treasury charge."}.into());
    insert(w, row);
}

/// One atomic plant bundle. Its embedded generating coal belongs to the power
/// producer, not the factory as well. Purchased internal power is deducted from
/// the plant and the same gross power value is allocated to dispatching sites.
pub fn record_factory(
    w: &mut WorldState,
    nation: NationId,
    district: &str,
    kind: K,
    output: f64,
    power: f64,
    raw: [f64; 12],
    plant_cash: f64,
    generation_cash: f64,
) {
    let id = format!("site:{district}:{}", kind.key());
    if output <= 0.0 || !output.is_finite() || !power.is_finite() || !event(w, &id) {
        return;
    }
    let fuel = (power * 0.02 * 1e9).round() / 1e9;
    let fuel = fuel.min(raw[C::Coal.idx()]);
    let mut plant_raw = raw;
    plant_raw[C::Coal.idx()] -= fuel;
    let mut row = receipt(
        id,
        production::catalog(kind).name.into(),
        district,
        kind.key(),
        "manufacturing",
    );
    row.status = "producing".into();
    row.output_quantity_daily = output;
    row.payments_daily_bn = plant_cash;
    row.output_unit = if matches!(kind,K::ProcessingPlant|K::StarterIndustry) {
        "intermediate packs"
    } else {
        "capital-goods packs"
    }
    .into();
    row.gross_output_daily_bn = output
        * if matches!(kind,K::ProcessingPlant|K::StarterIndustry) {
            INTERMEDIATE_PACK_BN
        } else {
            CAPITAL_PACK_BN
        };
    match raw_value(&plant_raw) {
        Ok(inputs) => {
            row.intermediate_inputs_daily_bn = inputs
                + power * POWER_UNIT_BN
                + if kind == K::MachineryWorks {
                    output * INTERMEDIATE_PACK_BN
                } else {
                    0.0
                };
        }
        Err(c) => {
            row.counted = false;
            row.classification = "unpriced_output".into();
            row.status = "unpriced".into();
            row.reason=Some(format!("Physical output is active, but {} has no constant-price input valuation; GDP is not guessed.",c.name()));
        }
    }
    if row.reason.is_none() {
        row.reason=Some("Actual packs produced, less consumed raw materials, intermediate packs and internal power. Model prices are not sales proceeds or profit; wages are not deducted from value added.".into());
    }
    insert(w, row);

    record_power_dispatch(w, nation, power, fuel, generation_cash);
}

/// A paid inherited Materials order makes part of background manufacturing
/// observable. Its already-used capacity is a reclassification, not another GDP
/// award; only production beyond that allowance replaces the incremental rate.
/// A blocked/cancelled order posts nothing and cannot suppress background GDP.
pub fn record_materials_operation(
    w: &mut WorldState,
    nation: NationId,
    district: &str,
    order_id: u32,
    output: f64,
    reserved_rate: f64,
    power: f64,
    raw: [f64; 12],
    conversion_cash: f64,
    energy_cash: f64,
) {
    let id = format!("inherited_materials:{order_id}");
    if w.districts.get(district) != Some(&nation)
        || [output, reserved_rate, power, conversion_cash, energy_cash].iter().any(|v| !v.is_finite() || *v < 0.0)
        || output <= 0.0 || reserved_rate <= 0.0
        || raw.iter().any(|v| !v.is_finite() || *v < 0.0)
    { return; }
    let Some(group) = crate::starting_industry::province(w, district)
        .and_then(|p| p.groups.into_iter().find(|g| g.key == "materials")) else { return; };
    if !event(w, &id) { return; }
    let fuel = ((power * 0.02 * 1e9).round() / 1e9).min(raw[C::Coal.idx()]);
    let mut plant_raw = raw;
    plant_raw[C::Coal.idx()] -= fuel;
    let Ok(raw_cost) = raw_value(&plant_raw) else { return; };
    let mut row = receipt(id, "Inherited Materials production".into(), district,
        "inherited_materials", "manufacturing");
    row.status = "producing".into();
    row.output_quantity_daily = output;
    row.output_unit = "intermediate packs".into();
    row.gross_output_daily_bn = output * INTERMEDIATE_PACK_BN;
    row.intermediate_inputs_daily_bn = raw_cost + power * POWER_UNIT_BN;
    row.payments_daily_bn = conversion_cash;
    let observed = (row.gross_output_daily_bn - row.intermediate_inputs_daily_bn).max(0.0);
    let utilization = if group.capacity_annual_bn > 0.0 {
        (group.current_output_annual_bn / group.capacity_annual_bn).clamp(0.0, 1.0)
    } else { 0.0 };
    let reserved = reserved_rate * (observed / output) * DAYS_PER_ACCOUNTING_YEAR;
    row.inherited_annual_gdp_bn = (observed * DAYS_PER_ACCOUNTING_YEAR)
        .min(reserved * utilization).min(group.current_output_annual_bn);
    row.classification = "observed_inherited_materials".into();
    row.reason = Some("Paid actual Materials output. The occupied share of reserved inherited capacity is already in background GDP; only realized output above that allowance is additional. Idle, blocked or cancelled orders leave background output unchanged.".into());
    row.valuation_basis.push_str(" Inherited overlap is disclosed separately; annual GDP settlement uses observed value added minus inherited overlap. Conversion fees are payment information, not a second output award.");
    insert(w, row);
    record_power_dispatch(w, nation, power, fuel, energy_cash);
}

/// Power is an internal transaction: factories deduct its full value and its
/// dispatching producers earn that same gross amount, less generating fuel.
fn record_power_dispatch(w: &mut WorldState, nation: NationId, power: f64, fuel: f64, generation_cash: f64) {
    let generators: Vec<_> = w
        .districts
        .iter()
        .filter(|(d, _)| {
            w.districts.get(*d) == Some(&nation) && !resources::district_contested(w, d)
        })
        .filter_map(|(d, _)| {
            let level = crate::industrial_modules::effective_capacity(w, d, K::Generation);
            (level > 0.0).then(|| (d.clone(), level * 10.0))
        })
        .collect();
    let capacity: f64 = generators.iter().map(|(_, c)| *c).sum();
    if power <= 0.0 || capacity <= 0.0 {
        return;
    }
    let fuel_value =
        fuel * resources::unit_price_bn(C::Coal).expect("coal has a documented 1990 price");
    let mut power_left = power;
    let mut fuel_left = fuel_value;
    let mut cash_left = generation_cash;
    for (i, (d, c)) in generators.iter().enumerate() {
        let last = i + 1 == generators.len();
        let share = *c / capacity;
        let used = if last { power_left } else { power * share };
        let inputs = if last { fuel_left } else { fuel_value * share };
        let cash = if last {
            cash_left
        } else {
            generation_cash * share
        };
        power_left -= used;
        fuel_left -= inputs;
        cash_left -= cash;
        let mut r = receipt(
            format!("site:{d}:generation"),
            "Power Generation".into(),
            d,
            "generation",
            "utilities",
        );
        r.status = "dispatching".into();
        r.output_quantity_daily = used;
        r.output_unit = "modeled power units".into();
        r.gross_output_daily_bn = used * POWER_UNIT_BN;
        r.intermediate_inputs_daily_bn = inputs;
        r.payments_daily_bn = cash;
        r.reason=Some("Only power actually consumed by running factories, apportioned across available generation by capacity. Plants deduct this same internal power value; generating coal is deducted here once.".into());
        insert(w, r);
    }
}

/// Called once when domestic resources post, not from a board/forecast read.
/// Only player-created completed mines are incremental; mapped inherited
/// national extraction belongs to the calibrated baseline, not this adapter.
pub fn record_mines(w: &mut WorldState) {
    if !crate::province_economy::active(w) {
        return;
    }
    for mine in w.resources.mines.clone() {
        let id = format!("mine:{}:{}", mine.district, mine.commodity.key());
        let Some(&owner) = w.districts.get(&mine.district) else {
            continue;
        };
        if !w.nation_opt(owner).is_some_and(|n| n.alive) || !event(w, &id) {
            continue;
        }
        let mut row = receipt(
            id,
            format!("{} extraction", mine.commodity.name()),
            &mine.district,
            mine.commodity.key(),
            "extraction",
        );
        if mine.commodity == C::Oil {
            row.counted = false;
            row.classification = "inherited_activity".into();
            row.status = "attributed".into();
            row.reason=Some("Already reflected in the inherited oil economy. No second GDP addition and no change to oil mechanics.".into());
            insert(w, row);
            continue;
        }
        let quantity = mine.output / 12.0 * clock::month_fraction(w);
        row.annualization_days = 12.0 / clock::month_fraction(w);
        row.valuation_basis="Source annual extraction flow at 1990 resource reference prices, minus MODEL 30% intermediate costs. Daily quantities follow the existing fiscal-month posting calendar; the annual production benchmark is unchanged at month boundaries.".into();
        row.output_quantity_daily = quantity;
        row.output_unit = format!("{} table units", mine.commodity.name());
        if let Some(price) = resources::unit_price_bn(mine.commodity) {
            row.gross_output_daily_bn = quantity * price;
            row.intermediate_inputs_daily_bn =
                row.gross_output_daily_bn * EXTRACTION_INTERMEDIATE_SHARE;
            row.status = "extracting".into();
            row.reason=Some("Only this completed mine's additional extraction. Reference 1990 commodity price minus a MODEL 30% intermediate-cost share; this cost estimate does not charge cash.".into());
        } else {
            row.counted = false;
            row.classification = "unpriced_output".into();
            row.status = "unpriced".into();
            row.reason = Some("Resource flow active—no GDP valuation yet.".into());
        }
        if resources::district_contested(w, &mine.district) {
            row.reason=Some(format!("{} Extraction follows current resource posting; territorial conflict does not currently suspend this completed mine's output.",row.reason.as_deref().unwrap_or("")));
        }
        insert(w, row);
    }
}

pub fn record_manufacturing_commitment(
    w: &mut WorldState,
    line: &crate::manufacturing::ManufacturingLine,
    budget_bn: f64,
    units: f64,
) {
    let id = format!("military_order:{}", line.id);
    if budget_bn <= 0.0 || !event(w, &id) {
        return;
    }
    let mut row = receipt(
        id,
        format!("{} equipment order", line.kit),
        &line.district,
        "military_order",
        "manufacturing",
    );
    row.counted = false;
    row.classification = "pending_order".into();
    row.status = "awaiting_production".into();
    row.payments_daily_bn = budget_bn;
    row.output_quantity_daily = units;
    row.output_unit = "equipment units ordered, not delivered".into();
    row.reason=Some("Paid procurement commitment on the existing long-lead order book—not measured completed production. No GDP credit is fabricated from ordering, and delivery is not credited a second time.".into());
    insert(w, row);
}

fn site_sector(kind: K) -> &'static str {
    match kind {
        K::Generation | K::PowerGrid => "utilities",
        K::Infrastructure | K::FreightTerminal | K::Warehouse => "transport",
        K::ResearchCenter => "public_services",
        _ => "manufacturing",
    }
}

/// Prototype service is an enabling input to the existing technology system,
/// NOT laboratory sales or a second capitalization of the construction bill.
/// Its cash is already paid through Science; expose the actual receipt without
/// inventing a value-added output price or awarding GDP on research credits.
pub fn record_research_service(w: &mut WorldState, operation: &industry::ResearchOperation) {
    let id = format!("site:{}:research_center", operation.district);
    if !event(w, &format!("research_service:{}:{}", operation.day, operation.district)) {
        return;
    }
    let mut row = receipt(id, "Research Center: prototype & testing".into(),
        &operation.district, K::ResearchCenter.key(), "public_services");
    row.counted = false;
    row.classification = "enabling_asset".into();
    row.status = operation.status.clone();
    row.reason = Some(operation.reason.clone());
    row.output_quantity_daily = operation.prototype_credit;
    row.output_unit = "specific-technology acquisition credit, not research money or GDP".into();
    row.payments_daily_bn = operation.cash_spent_daily_bn;
    row.intermediate_inputs_daily_bn = operation.goods_used.intermediates * INTERMEDIATE_PACK_BN
        + operation.goods_used.capital_goods * CAPITAL_PACK_BN;
    row.valuation_basis = "Modeled prototype/testing service. Supplies were already counted when manufactured; no additional laboratory GDP valuation is asserted.".into();
    insert(w, row);
}
/// Pure, save-neutral read model. Completion-day receipts survive removal of
/// their projects; passive assets and blocked orders remain visible at zero.
pub fn contributions(w: &WorldState) -> Vec<Contribution> {
    if w.province_economy.is_none() {
        return vec![];
    }
    let mut rows = w.province_economy.as_ref().unwrap().flows.receipts.clone();
    for p in &w.production.projects {
        let id = format!("construction:{}", p.id);
        rows.entry(id.clone()).or_insert_with(|| {
            let mut r = receipt(
                id,
                format!("{} construction", production::catalog(p.kind).name),
                &p.district,
                p.kind.key(),
                "construction",
            );
            r.status = p.status.key().into();
            r.reason = p.reason.clone().or_else(|| {
                Some("No completed work has been recorded for this settlement.".into())
            });
            r
        });
    }
    for p in &w.resources.mine_projects {
        let id = format!("mine_construction:{}:{}", p.district, p.commodity.key());
        rows.entry(id.clone()).or_insert_with(||{let mut r=receipt(id,format!("{} mine construction",p.commodity.name()),&p.district,"mine_development","construction");r.status="awaiting_work".into();r.reason=w.production.industry.mines.get(&industry::mine_key(&p.district,p.commodity)).and_then(|f|f.reason.clone()).or_else(||Some("No completed mine-construction work has been recorded for this settlement.".into()));r});
    }
    for mine in &w.resources.mines {
        let id = format!("mine:{}:{}", mine.district, mine.commodity.key());
        rows.entry(id.clone()).or_insert_with(||{let mut r=receipt(id,format!("{} extraction",mine.commodity.name()),&mine.district,mine.commodity.key(),"extraction");r.counted=false;r.classification="awaiting_settlement".into();r.status="awaiting_settlement".into();r.reason=Some("Completed mine; waiting for an actual resource settlement. A production forecast is not credited as realized GDP.".into());if mine.commodity==C::Oil{r.classification="inherited_activity".into();r.status="attributed".into();r.reason=Some("Already reflected in the inherited oil economy. No second GDP addition and no change to oil mechanics.".into());}r});
    }
    let sites: BTreeSet<&String> = w
        .production
        .provinces
        .iter()
        .map(|p| &p.district)
        .chain(w.production.industry.sites.keys())
        .chain(w.production.industry.modules.keys())
        .collect();
    for district in sites {
        if !w.districts.contains_key(district) {
            continue;
        }
        for kind in production::PROJECT_KINDS {
            let capacity=if matches!(kind,K::StarterIndustry|K::Generation) {
                crate::industrial_modules::effective_capacity(w,district,kind)
            } else {production::level(w,district,kind) as f64};
            if capacity <= 0.0 {
                continue;
            }
            let id = format!("site:{district}:{}", kind.key());
            rows.entry(id.clone()).or_insert_with(||{
                let mut r=receipt(id,production::catalog(kind).name.into(),district,kind.key(),site_sector(kind));
                r.counted=false;r.classification="enabling_asset".into();r.status="enabling".into();
                r.reason=Some(format!("{} No independent GDP bonus; only actual downstream output or completed work is counted.",production::catalog(kind).effect));
                if matches!(kind,K::ProcessingPlant|K::StarterIndustry|K::MachineryWorks|K::Generation){r.classification="inactive_capacity".into();r.status="idle".into();r.reason=w.production.industry.operations.iter().find(|s|s.district==*district && s.kind==kind).and_then(|s|s.reason.clone()).or_else(||Some("No actual output or power dispatch was recorded for this settlement; installed capacity alone is not GDP.".into()));}
                r
            });
        }
    }
    for line in &w.manufacturing.lines {
        let id = format!("military_order:{}", line.id);
        rows.entry(id.clone()).or_insert_with(||{let mut r=receipt(id,format!("{} equipment programme",line.kit),&line.district,"military_order","manufacturing");r.counted=false;r.classification="pending_order".into();r.status=line.status.key().into();r.reason=line.reason.clone().or_else(||Some("Procurement is an order commitment, not a measured completed output. It is not added to province GDP.".into()));r});
    }
    rows.into_values().collect()
}

pub fn finish_day(_w: &mut WorldState) {}

/// Stable identifiers present when the GDP ledger is enabled. The aggregator
/// may absorb their first measured operating flow into the inherited baseline;
/// unfinished projects are deliberately absent, so their future work is new.
pub fn inherited_asset_ids(w: &WorldState) -> BTreeSet<String> {
    let mut ids = BTreeSet::new();
    for (d, _) in &w.districts {
        for kind in [K::ProcessingPlant, K::StarterIndustry, K::MachineryWorks, K::Generation] {
            let capacity=if matches!(kind,K::StarterIndustry|K::Generation) {
                crate::industrial_modules::effective_capacity(w,d,kind)
            } else {production::level(w,d,kind) as f64};
            if capacity > 0.0 {
                ids.insert(format!("site:{d}:{}", kind.key()));
            }
        }
    }
    for mine in &w.resources.mines {
        ids.insert(format!("mine:{}:{}", mine.district, mine.commodity.key()));
    }
    ids
}

/// Installed productive scale used ONLY to apportion migration's first
/// measured receipt between an existing asset and subsequent upgrades. This
/// is never a forecast, a receipt or a GDP award. Actual money, inputs, power
/// and utilization remain mandatory at settlement.
pub fn asset_scales(w: &WorldState) -> BTreeMap<String, f64> {
    let mut scales = BTreeMap::new();
    let districts=w.production.industry.sites.keys().chain(w.production.industry.modules.keys()).collect::<BTreeSet<_>>();
    for d in districts {
        for kind in [K::ProcessingPlant, K::StarterIndustry, K::MachineryWorks] {
            let rate = industry::plant_rate(w, d, kind);
            if rate <= 0.0 {
                continue;
            }
            let power = industry::power_per_pack(w, d, kind);
            let mut raw = industry::operating_recipe(kind, 1.0, power);
            raw[C::Coal.idx()] -= (power * 0.02 * 1e9).round() / 1e9;
            let gross = if matches!(kind,K::ProcessingPlant|K::StarterIndustry) {
                INTERMEDIATE_PACK_BN
            } else {
                CAPITAL_PACK_BN
            };
            if let Ok(cost) = raw_value(&raw) {
                let va = gross
                    - cost
                    - power * POWER_UNIT_BN
                    - if kind == K::MachineryWorks {
                        INTERMEDIATE_PACK_BN
                    } else {
                        0.0
                    };
                scales.insert(format!("site:{d}:{}", kind.key()), rate * va);
            }
        }
        let generation = crate::industrial_modules::effective_capacity(w, d, K::Generation) * 10.0;
        if generation > 0.0 {
            scales.insert(
                format!("site:{d}:generation"),
                generation
                    * (POWER_UNIT_BN - 0.02 * resources::unit_price_bn(C::Coal).unwrap_or(0.0)),
            );
        }
    }
    for mine in &w.resources.mines {
        if let Some(price) = resources::unit_price_bn(mine.commodity) {
            scales.insert(
                format!("mine:{}:{}", mine.district, mine.commodity.key()),
                mine.output * price * (1.0 - EXTRACTION_INTERMEDIATE_SHARE),
            );
        }
    }
    scales
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{init::world_1990, programs, province_economy, world::GameRules};
    const USA: NationId = NationId::USA;
    fn near(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-10, "{a} != {b}");
    }
    fn prepared() -> WorldState {
        let mut w = world_1990(GameRules {
            daily_simulation: true,
            production_system: true,
            resource_market: true,
            manufacturing_system: true,
            ..GameRules::default()
        });
        w.player = Some(USA);
        let year = w.year;
        programs::install(&mut w, USA, year, programs::default_departments());
        programs::begin_day(&mut w);
        province_economy::enable(&mut w);
        province_economy::begin_day(&mut w);
        for c in ALL {
            if c != C::Oil {
                resources::set_stockpile_for_test(&mut w, USA, c, 1e6);
            }
        }
        w
    }
    fn district(w: &WorldState) -> String {
        w.districts
            .iter()
            .find(|(_, n)| **n == USA)
            .unwrap()
            .0
            .clone()
    }
    fn chain(w: &mut WorldState, d: &str) {
        for kind in [
            K::CivilianIndustry,
            K::PowerGrid,
            K::Generation,
            K::ProcessingPlant,
            K::MachineryWorks,
        ] {
            production::complete_capability(w, d, kind);
        }
    }
    #[test]
    fn disabled_adapter_is_byte_inert_and_readers_never_settle() {
        let mut w = world_1990(GameRules::default());
        let before = crate::save(&w);
        begin_day(&mut w);
        record_mines(&mut w);
        finish_day(&mut w);
        assert!(contributions(&w).is_empty());
        assert_eq!(crate::save(&w), before);
        let w = prepared();
        let before = crate::save(&w);
        let _ = contributions(&w);
        let _ = asset_scales(&w);
        assert_eq!(crate::save(&w), before);
    }
    #[test]
    fn actual_factory_chain_conserves_value_added_and_internal_power() {
        let mut w = prepared();
        let d = district(&w);
        chain(&mut w, &d);
        let gdp = w.nation(USA).gdp;
        let treasury = w.nation(USA).treasury_bn;
        industry::tick_day(&mut w);
        let rows = contributions(&w);
        let total = rows.iter().map(|r| r.daily_value_added_bn).sum::<f64>();
        let mut raw = [0.0; 12];
        raw[C::Iron.idx()] = 1.0;
        raw[C::Bauxite.idx()] = 0.2;
        raw[C::Coal.idx()] = 0.29;
        raw[C::Copper.idx()] = 0.05;
        let final_and_inventory = 0.5 * INTERMEDIATE_PACK_BN + 0.5 * CAPITAL_PACK_BN;
        near(total, final_and_inventory - raw_value(&raw).unwrap());
        let energy = rows.iter().find(|r| r.kind == "generation").unwrap();
        near(energy.output_quantity_daily, 2.0);
        near(energy.gross_output_daily_bn, 2.0 * POWER_UNIT_BN);
        assert!(rows
            .iter()
            .filter(|r| r.counted)
            .all(|r| r.daily_value_added_bn >= 0.0));
        assert_eq!(w.nation(USA).gdp, gdp);
        assert_eq!(
            w.nation(USA).treasury_bn,
            treasury,
            "GDP valuation is not a sale or another cash charge"
        );
        let once = crate::save(&w);
        industry::tick_day(&mut w);
        begin_day(&mut w);
        assert_eq!(crate::save(&w), once);
        let mut restored = crate::load(&once).unwrap();
        industry::tick_day(&mut restored);
        begin_day(&mut restored);
        assert_eq!(
            crate::save(&restored),
            once,
            "saving/reloading cannot repost receipts"
        );
    }
    #[test]
    fn blocked_plants_and_enablers_do_not_create_output() {
        let mut w = prepared();
        let d = district(&w);
        chain(&mut w, &d);
        resources::set_stockpile_for_test(&mut w, USA, C::Coal, 0.0);
        for kind in production::PROJECT_KINDS {
            if kind==K::StarterIndustry {
                w.production.industry.modules.insert(d.clone(),10_000);
            } else if production::level(&w, &d, kind) == 0 {
                production::complete_capability(&mut w, &d, kind);
            }
        }
        industry::tick_day(&mut w);
        let rows = contributions(&w);
        for kind in production::PROJECT_KINDS {
            assert!(
                rows.iter().any(|r| r.kind == kind.key()),
                "missing {:?}",
                kind
            );
        }
        assert!(rows.iter().all(|r| r.annual_gdp_bn == 0.0));
        assert!(rows.iter().all(|r| r.reason.is_some()));
    }
    #[test]
    fn fractional_workshop_records_only_real_intermediates_and_balances_internal_power() {
        let mut w=prepared();
        let d=district(&w);
        w.production.industry.modules.insert(d.clone(),10_000);
        assert_eq!(production::level(&w,&d,K::ProcessingPlant),0);
        assert!(contributions(&w).iter().all(|r|r.daily_value_added_bn==0.0));
        let before=crate::save(&w);
        let scales=asset_scales(&w);
        assert!(scales[&format!("site:{d}:starter_industry")]>0.0);
        assert_eq!(before,crate::save(&w),"Reading module scale is not production");
        industry::tick_day(&mut w);
        let rows=contributions(&w);
        let factory=rows.iter().find(|r|r.kind=="starter_industry").unwrap();
        let generator=rows.iter().find(|r|r.kind=="generation").unwrap();
        near(factory.output_quantity_daily,0.01);
        assert_eq!(factory.output_unit,"intermediate packs");
        near(factory.gross_output_daily_bn,0.01*INTERMEDIATE_PACK_BN);
        near(generator.output_quantity_daily,0.01);
        let mut raw=[0.0;12];
        raw[C::Iron.idx()]=0.01;
        raw[C::Bauxite.idx()]=0.002;
        raw[C::Coal.idx()]=0.0027;
        near(rows.iter().map(|r|r.daily_value_added_bn).sum(),0.01*INTERMEDIATE_PACK_BN-raw_value(&raw).unwrap());
        assert_eq!(rows.iter().filter(|r|r.counted).count(),2,"Only actual processing and dispatched power are value-added receipts");
        let once=crate::save(&w);
        industry::tick_day(&mut w);
        assert_eq!(once,crate::save(&w));
        let mut loaded=crate::load(&once).unwrap();
        industry::tick_day(&mut loaded);
        assert_eq!(once,crate::save(&loaded));
    }
    #[test]
    fn a_migrated_module_expansion_changes_capacity_without_a_gdp_completion_award() {
        let mut w=prepared();
        let d=district(&w);
        w.production.industry.modules.insert(d.clone(),10_000);
        let first=asset_scales(&w);
        let inherited=inherited_asset_ids(&w);
        assert!(inherited.contains(&format!("site:{d}:starter_industry")));
        assert!(inherited.contains(&format!("site:{d}:generation")));
        let gdp=w.nation(USA).gdp;
        w.production.industry.modules.insert(d.clone(),20_000);
        let larger=asset_scales(&w);
        near(larger[&format!("site:{d}:starter_industry")],2.0*first[&format!("site:{d}:starter_industry")]);
        near(larger[&format!("site:{d}:generation")],2.0*first[&format!("site:{d}:generation")]);
        assert_eq!(w.nation(USA).gdp,gdp);
        assert!(contributions(&w).iter().all(|r|r.daily_value_added_bn==0.0));
    }
    #[test]
    fn power_dispatch_is_shared_across_generator_provinces_not_credited_twice() {
        let mut w = prepared();
        let ds: Vec<_> = w
            .districts
            .iter()
            .filter(|(_, n)| **n == USA)
            .take(2)
            .map(|(d, _)| d.clone())
            .collect();
        chain(&mut w, &ds[0]);
        production::complete_capability(&mut w, &ds[1], K::Generation);
        production::complete_capability(&mut w, &ds[1], K::Generation);
        industry::tick_day(&mut w);
        let rows = contributions(&w);
        let generators: Vec<_> = rows.iter().filter(|r| r.kind == "generation").collect();
        assert_eq!(generators.len(), 2);
        near(
            generators.iter().map(|r| r.output_quantity_daily).sum(),
            2.0,
        );
        near(
            generators
                .iter()
                .find(|r| r.district == ds[0])
                .unwrap()
                .output_quantity_daily,
            2.0 / 3.0,
        );
        near(
            generators
                .iter()
                .find(|r| r.district == ds[1])
                .unwrap()
                .output_quantity_daily,
            4.0 / 3.0,
        );
    }
    #[test]
    fn completion_keeps_actual_work_receipt_and_legacy_work_is_not_incremental() {
        let mut w = prepared();
        let d = district(&w);
        let kind = K::CivilianIndustry;
        let id = production::start_project(&mut w, USA, &d, kind).unwrap();
        let p = &mut w.production.projects[0];
        p.progress_days = p.total_days as f64 - 0.5;
        let fraction = p.progress_fraction();
        p.resources_used = production::catalog(kind).recipe.map(|r| r * fraction);
        w.production
            .industry
            .projects
            .get_mut(&id)
            .unwrap()
            .spent_bn = industry::work_cost_bn(kind) * fraction;
        production::tick_day(&mut w);
        assert!(w.production.projects.is_empty());
        let rows = contributions(&w);
        let receipt = rows
            .iter()
            .find(|r| r.id == format!("construction:{id}"))
            .unwrap();
        assert!(receipt.counted && receipt.annual_gdp_bn > 0.0);
        assert!(rows.iter().any(|r| r.kind == kind.key() && !r.counted));
        let legacy = Project {
            id: 99,
            nation: USA,
            district: d,
            kind: K::Infrastructure,
            priority: production::Priority::Normal,
            status: production::ProjectStatus::Building,
            reason: None,
            progress_days: 0.0,
            total_days: 360,
            resources_used: [0.0; 12],
            capacity_micros: None,
            started_day: None,
        };
        record_construction(&mut w, &legacy, 1.0, 0.001, false);
        let rows = contributions(&w);
        let r = rows.iter().find(|r| r.id == "construction:99").unwrap();
        assert!(!r.counted);
        near(r.annual_gdp_bn, 0.0);
        near(r.payments_daily_bn, 0.0);
    }
    #[test]
    fn all_thirteen_construction_kinds_use_paid_work_not_completion_bonuses() {
        let mut w = prepared();
        let d = district(&w);
        for (i, kind) in production::PROJECT_KINDS.into_iter().enumerate() {
            let p = Project {
                id: 100 + i as u32,
                nation: USA,
                district: d.clone(),
                kind,
                priority: production::Priority::Normal,
                status: production::ProjectStatus::Building,
                reason: None,
                progress_days: 0.0,
                total_days: production::catalog(kind).total_days,
                resources_used: [0.0; 12],
                capacity_micros: None,
                started_day: None,
            };
            record_construction(&mut w, &p, 1.0, 0.001, true);
        }
        let rows = contributions(&w);
        assert_eq!(
            rows.iter()
                .filter(|r| r.sector == "construction" && r.counted)
                .count(),
            13
        );
        near(rows.iter().map(|r| r.daily_value_added_bn).sum(), 0.013);
    }
    #[test]
    fn new_mine_receipts_follow_actual_posting_without_january_february_gdp_jumps() {
        let mut w = prepared();
        let d = district(&w);
        w.resources.mines.push(resources::Mine {
            district: d.clone(),
            commodity: C::Iron,
            output: 365.0,
            completed: 0,
        });
        assert!(contributions(&w)
            .iter()
            .any(|r| r.kind == "iron" && !r.counted));
        w.day = 31;
        begin_day(&mut w);
        resources::tick(&mut w);
        let jan = contributions(&w)
            .into_iter()
            .find(|r| r.kind == "iron")
            .unwrap();
        near(jan.output_quantity_daily, 365.0 / 12.0 / 31.0);
        near(
            jan.annual_gdp_bn,
            365.0 * resources::unit_price_bn(C::Iron).unwrap() * 0.70,
        );
        let once = crate::save(&w);
        record_mines(&mut w);
        assert_eq!(crate::save(&w), once);
        clock::advance_date(&mut w);
        begin_day(&mut w);
        resources::tick(&mut w);
        let feb = contributions(&w)
            .into_iter()
            .find(|r| r.kind == "iron")
            .unwrap();
        near(feb.annual_gdp_bn, jan.annual_gdp_bn);
        assert!(feb.daily_value_added_bn > jan.daily_value_added_bn);
        w.resources.mines.push(resources::Mine {
            district: d,
            commodity: C::Oil,
            output: 100.0,
            completed: 0,
        });
        record_mines(&mut w);
        let oil = contributions(&w)
            .into_iter()
            .find(|r| r.kind == "oil")
            .unwrap();
        assert!(!oil.counted);
        near(oil.annual_gdp_bn, 0.0);
        assert!(oil.reason.unwrap().contains("inherited oil"));
    }
    #[test]
    fn mine_construction_hook_records_paid_work_but_not_inherited_extraction() {
        let mut baseline = prepared();
        resources::tick(&mut baseline);
        assert!(
            contributions(&baseline)
                .iter()
                .all(|r| r.annual_gdp_bn == 0.0),
            "inherited resource flow is already in the national baseline"
        );
        let mut w = prepared();
        let (d, c) = w
            .districts
            .iter()
            .filter(|(_, n)| **n == USA)
            .flat_map(|(d, _)| ALL.map(|c| (d.clone(), c)))
            .find(|(d, c)| resources::mine_refusal(&w, USA, d, *c).is_none())
            .unwrap();
        resources::start_mine(&mut w, USA, &d, c).unwrap();
        resources::tick(&mut w);
        let rows = contributions(&w);
        let work = rows.iter().find(|r| r.kind == "mine_development").unwrap();
        assert!(work.counted && work.payments_daily_bn > 0.0);
        near(work.daily_value_added_bn, work.payments_daily_bn);
        assert!(
            rows.iter().all(|r| r.sector != "extraction"),
            "an unfinished mine has no extra extraction"
        );
    }
    #[test]
    fn military_order_is_a_payment_not_fabricated_delivered_production() {
        let mut w = prepared();
        let d = district(&w);
        let line = crate::manufacturing::ManufacturingLine {
            id: 4,
            nation: USA,
            district: d,
            kit: "arm_gen3".into(),
            priority: production::Priority::Normal,
            status: crate::manufacturing::LineStatus::Producing,
            reason: None,
            ordered_bn: 1.0,
            resources_used: [0.0; 12],
            ordered_today_bn: 1.0,
            throughput_today: 1.0,
            settled_day: None,
        };
        record_manufacturing_commitment(&mut w, &line, 1.0, 12.0);
        let once = crate::save(&w);
        record_manufacturing_commitment(&mut w, &line, 1.0, 12.0);
        assert_eq!(once, crate::save(&w));
        let row = contributions(&w)
            .into_iter()
            .find(|r| r.kind == "military_order")
            .unwrap();
        assert!(!row.counted);
        near(row.gross_output_daily_bn, 0.0);
        near(row.payments_daily_bn, 1.0);
        near(row.annual_gdp_bn, 0.0);
    }
    #[test]
    fn installed_migration_scale_accounts_for_subsequent_upgrades_without_awarding_output() {
        let mut w = prepared();
        let d = district(&w);
        chain(&mut w, &d);
        let before = asset_scales(&w);
        production::complete_capability(&mut w, &d, K::ProcessingPlant);
        production::complete_capability(&mut w, &d, K::Automation);
        production::complete_capability(&mut w, &d, K::Efficiency);
        let after = asset_scales(&w);
        assert!(
            after[&format!("site:{d}:processing_plant")]
                > before[&format!("site:{d}:processing_plant")] * 2.0
        );
        assert!(contributions(&w).iter().all(|r| r.annual_gdp_bn == 0.0));
        assert!(asset_scales(&w).values().all(|v| *v > 0.0));
        let saved = crate::save(&w);
        let restored = crate::load(&saved).unwrap();
        assert_eq!(contributions(&w), contributions(&restored));
    }
}
