//! Modeled province GDP accounts, not a historical subnational GDP dataset.
//!
//! National GDP is an ANNUAL real-output rate. Actual project value added is
//! another annual rate: replace its previous level, never add annual output on
//! every day. The macro engine grows only inherited output. Explicitly enrolled
//! programme projects no longer also enter its aggregate public-investment arm.
//!
//! Existing peace/cession/successor rules remain the national settlement oracle.
//! Prior project output is reattributed by CURRENT district ownership before
//! replacement, so a GDP transfer already made by those rules is not paid twice.
//! Province weights travel with their land and are normalized into the current
//! owner's inherited residual. This is accounting, not a new settlement formula.
use crate::{
    clock, districts, gdp_projects,
    world::{NationId, WorldState},
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const SECTORS: [&str; 8] = [
    "agriculture",
    "extraction",
    "manufacturing",
    "utilities",
    "construction",
    "transport",
    "services",
    "public_services",
];
pub const SECTOR_NAMES: [&str; 8] = [
    "Agriculture",
    "Extraction",
    "Manufacturing",
    "Utilities",
    "Construction",
    "Transport & logistics",
    "Market services",
    "Public services",
];
/// MODEL PRESET, not transcribed historical sector shares. Deliberately generic
/// until licensed regional accounts replace the allocation proxy. The national
/// GDP itself remains its existing sourced/calibrated number.
pub const MODEL_SECTOR_SHARES: [f64; 8] = [0.08, 0.06, 0.20, 0.04, 0.07, 0.10, 0.30, 0.15];

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct Date {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}
impl Date {
    fn of(w: &WorldState) -> Self {
        Self {
            year: w.year,
            month: w.month,
            day: w.day,
        }
    }
    fn from_day(day: i32) -> Self {
        let (year, month, day) = clock::date_from_day(day);
        Self { year, month, day }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProvinceBasis {
    pub weight: f64,
    pub opening_gdp_bn: f64,
    pub opening_date: Date,
    pub sector_shares: [f64; 8],
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct NationBasis {
    pub opening_gdp_bn: f64,
    pub opening_date: Date,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct ProvinceEconomy {
    pub provinces: BTreeMap<String, ProvinceBasis>,
    pub nations: BTreeMap<NationId, NationBasis>,
    pub flows: gdp_projects::FlowLedger,
    pub posted_contributions: Vec<gdp_projects::Contribution>,
    pub pending_inherited_assets: BTreeMap<String, f64>,
    pub day_opening_gdp: BTreeMap<NationId, f64>,
    /// Persistent book-value reconciliation after the inherited settlement
    /// model values ceded/damaged output below its modeled production price.
    /// Carries with the asset: stopping/restarting cannot undo that haircut.
    pub valuation_scales: BTreeMap<String, f64>,
    pub day: Option<i32>,
    pub settled_day: Option<i32>,
}
fn nonnegative(value: f64) -> f64 {
    if value.is_finite() {
        value.max(0.0)
    } else {
        0.0
    }
}
pub fn active(w: &WorldState) -> bool {
    clock::is_daily(w) && w.province_economy.is_some()
}

/// Explicit browser opt-in. Pending legacy calendars may establish read-only
/// accounts now; their macro behaviour is unchanged until daily play starts.
/// Repeated enable calls never reseed weights, reset history, or award output.
pub fn enable(w: &mut WorldState) {
    if !clock::is_daily(w) && w.daily.activate_after_month.is_none() {
        return;
    }
    if w.province_economy.is_none() {
        let pending_inherited_assets = gdp_projects::asset_scales(w);
        w.province_economy = Some(ProvinceEconomy {
            pending_inherited_assets,
            ..ProvinceEconomy::default()
        });
    }
    ensure_accounts(w);
}

fn ensure_accounts(w: &mut WorldState) {
    if w.province_economy.is_none() {
        return;
    }
    let date = Date::of(w);
    let alive: Vec<_> = w
        .nations
        .iter()
        .filter(|n| n.alive)
        .map(|n| {
            (
                n.id,
                nonnegative(n.gdp),
                crate::programs::effective_investment(n, n.state_invest_gdp),
            )
        })
        .collect();
    let freeze = clock::is_daily(w);
    for (id, total, investment) in alive {
        if freeze && w.nation(id).province_investment_reference.is_none() {
            w.nation_mut(id).province_investment_reference = Some(nonnegative(investment));
        }
        w.province_economy
            .as_mut()
            .unwrap()
            .nations
            .entry(id)
            .or_insert(NationBasis {
                opening_gdp_bn: total,
                opening_date: date,
            });
        let owned: Vec<String> = w
            .districts
            .iter()
            .filter(|(_, owner)| **owner == id)
            .map(|(d, _)| d.clone())
            .collect();
        let masses: Vec<f64> = owned
            .iter()
            .map(|d| nonnegative(districts::population_of(w, d).unwrap_or(0.0)))
            .collect();
        let sum: f64 = masses.iter().sum();
        let mut remaining = total;
        for (i, d) in owned.iter().enumerate() {
            let amount = if i + 1 == owned.len() {
                remaining
            } else if sum > 0.0 {
                total * masses[i] / sum
            } else {
                total / owned.len().max(1) as f64
            };
            let amount = amount.min(remaining).max(0.0);
            remaining = (remaining - amount).max(0.0);
            w.province_economy
                .as_mut()
                .unwrap()
                .provinces
                .entry(d.clone())
                .or_insert(ProvinceBasis {
                    // Dollar-weighted population proxy travels with its district.
                    // Zero-GDP provinces retain a nonnegative fallback weight.
                    weight: if amount > 0.0 {
                        amount
                    } else {
                        masses[i].max(1e-12)
                    },
                    opening_gdp_bn: amount,
                    opening_date: date,
                    sector_shares: MODEL_SECTOR_SHARES,
                });
        }
    }
}

fn prior_project_levels(w: &WorldState) -> BTreeMap<NationId, f64> {
    let mut out = BTreeMap::new();
    if let Some(accounts) = &w.province_economy {
        for row in &accounts.posted_contributions {
            if row.counted {
                if let Some(&owner) = w.districts.get(&row.district) {
                    if w.nation_opt(owner).is_some_and(|n| n.alive) {
                        *out.entry(owner).or_insert(0.0) += nonnegative(row.annual_gdp_bn);
                    }
                }
            }
        }
    }
    out
}
fn adjust_row(row: &mut gdp_projects::Contribution, factor: f64) {
    if factor >= 1.0 {
        return;
    }
    row.annual_gdp_bn *= factor;
    row.daily_value_added_bn *= factor;
    row.gross_output_daily_bn *= factor;
    row.intermediate_inputs_daily_bn *= factor;
    // Cash and physical quantities are receipts, not revaluations, and stay.
    row.valuation_basis.push_str(&format!(" Territorial/national book-value reconciliation applies a {:.6} valuation factor; physical quantities and actual cash are unchanged.",factor));
}
fn reconcile_recorded_values(w: &mut WorldState) {
    let levels = prior_project_levels(w);
    let factors: BTreeMap<NationId, f64> = levels
        .into_iter()
        .filter_map(|(id, value)| {
            let total = nonnegative(w.nation(id).gdp);
            (value > total && value > 0.0).then_some((id, total / value))
        })
        .collect();
    if factors.is_empty() {
        return;
    }
    let owners = &w.districts;
    let accounts = w.province_economy.as_mut().unwrap();
    for row in accounts
        .posted_contributions
        .iter_mut()
        .filter(|r| r.counted)
    {
        if let Some(factor) = owners
            .get(&row.district)
            .and_then(|owner| factors.get(owner))
        {
            adjust_row(row, *factor);
            *accounts
                .valuation_scales
                .entry(row.id.clone())
                .or_insert(1.0) *= *factor;
        }
    }
}
/// None keeps the old multiplication bit for bit. An enabled zero-project
/// world takes exactly that arithmetic too, not an algebraic reconstruction.
pub fn project_level(w: &WorldState, nation: NationId) -> Option<f64> {
    active(w).then(|| {
        prior_project_levels(w)
            .get(&nation)
            .copied()
            .unwrap_or(0.0)
            .min(nonnegative(w.nation(nation).gdp))
    })
}
pub fn apply_macro_factor(gdp: &mut f64, project: Option<f64>, factor: f64) {
    match project {
        Some(level) if level > 0.0 => *gdp = nonnegative((*gdp - level).max(0.0) * factor + level),
        _ => *gdp *= factor,
    }
}

pub fn begin_day(w: &mut WorldState) {
    if !active(w) {
        return;
    }
    ensure_accounts(w);
    reconcile_recorded_values(w);
    let today = clock::absolute_day(w);
    if w.province_economy.as_ref().unwrap().day == Some(today) {
        return;
    }
    w.province_economy.as_mut().unwrap().day = Some(today);
    w.province_economy.as_mut().unwrap().day_opening_gdp = w
        .nations
        .iter()
        .filter(|n| n.alive)
        .map(|n| (n.id, nonnegative(n.gdp)))
        .collect();
    gdp_projects::begin_day(w);
}

pub fn finish_day(w: &mut WorldState) {
    if !active(w) {
        return;
    }
    let today = clock::absolute_day(w);
    let ledger = w.province_economy.as_ref().unwrap();
    if ledger.day != Some(today) || ledger.settled_day == Some(today) {
        return;
    }
    ensure_accounts(w); // Successors may have been born after today's opening.
    reconcile_recorded_values(w);
    gdp_projects::finish_day(w);
    let mut prior = prior_project_levels(w);
    let scales = gdp_projects::asset_scales(w);
    let settled_prior = prior.clone();
    let mut absorbed: BTreeMap<String, (NationId, f64)> = BTreeMap::new();
    let mut rows = gdp_projects::contributions(w);
    rows.sort_by(|a, b| a.district.cmp(&b.district).then_with(|| a.id.cmp(&b.id)));
    let mut next = BTreeMap::new();
    for row in &mut rows {
        if !row.annual_gdp_bn.is_finite() || row.annual_gdp_bn < 0.0 {
            row.counted = false;
            row.annual_gdp_bn = 0.0;
            row.daily_value_added_bn = 0.0;
            row.gross_output_daily_bn = 0.0;
            row.intermediate_inputs_daily_bn = 0.0;
            row.classification = "invalid_valuation".into();
            row.reason=Some("No GDP credited: the accounting receipt does not contain a finite, nonnegative value-added estimate.".into());
        }
        row.annual_gdp_bn = nonnegative(row.annual_gdp_bn);
        let valuation = w
            .province_economy
            .as_ref()
            .unwrap()
            .valuation_scales
            .get(&row.id)
            .copied()
            .unwrap_or(1.0);
        adjust_row(row, valuation);
        if row.counted {
            if let Some(&owner) = w.districts.get(&row.district) {
                if w.nation_opt(owner).is_some_and(|n| n.alive) {
                    if let Some(opening_scale) = if row.annual_gdp_bn > 0.0 {
                        w.province_economy
                            .as_mut()
                            .unwrap()
                            .pending_inherited_assets
                            .remove(&row.id)
                    } else {
                        None
                    } {
                        // A pre-upgrade plant was already inside national GDP.
                        // Its first measured output splits that inherited level
                        // instead of awarding a windfall on loading an old save.
                        let share = scales
                            .get(&row.id)
                            .filter(|scale| **scale > 0.0)
                            .map_or(1.0, |scale| (opening_scale / scale).clamp(0.0, 1.0));
                        absorbed.insert(row.id.clone(), (owner, row.annual_gdp_bn * share));
                        row.classification = if share < 1.0 {
                            "mixed_inherited_and_incremental_value_added"
                        } else {
                            "inherited_value_added"
                        }
                        .into();
                        row.valuation_basis.push_str(&format!(" {:.6} of this first measured activity was already inside opening GDP; only the remainder changes its level.",share));
                    }
                    *next.entry(owner).or_insert(0.0) += row.annual_gdp_bn;
                }
            }
        }
    }
    // First-observation migration is a decomposition of existing GDP, even
    // when a legacy settlement left less national GDP than the site's nominal
    // output. Rebase ONLY the inherited portion: post-enable upgrades still
    // contribute their complete measured incremental value.
    let mut inherited_by_owner: BTreeMap<NationId, f64> = BTreeMap::new();
    for (owner, amount) in absorbed.values() {
        *inherited_by_owner.entry(*owner).or_insert(0.0) += *amount;
    }
    let absorption_factors: BTreeMap<_, _> = inherited_by_owner
        .iter()
        .map(|(owner, amount)| {
            let room = (nonnegative(w.nation(*owner).gdp)
                - settled_prior.get(owner).copied().unwrap_or(0.0))
            .max(0.0);
            (
                *owner,
                if *amount > 0.0 {
                    (room / amount).min(1.0)
                } else {
                    1.0
                },
            )
        })
        .collect();
    for row in &mut rows {
        if let Some((owner, amount)) = absorbed.get(&row.id) {
            let factor = absorption_factors[owner];
            let included = amount * factor;
            *prior.entry(*owner).or_insert(0.0) += included;
            if factor < 1.0 && row.annual_gdp_bn > 0.0 {
                let valuation =
                    (row.annual_gdp_bn - amount + included).max(0.0) / row.annual_gdp_bn;
                adjust_row(row, valuation);
                *w.province_economy
                    .as_mut()
                    .unwrap()
                    .valuation_scales
                    .entry(row.id.clone())
                    .or_insert(1.0) *= valuation;
            }
        }
    }
    next.clear();
    for row in rows.iter().filter(|r| r.counted) {
        if let Some(&owner) = w.districts.get(&row.district) {
            if w.nation_opt(owner).is_some_and(|n| n.alive) {
                *next.entry(owner).or_insert(0.0) += row.annual_gdp_bn;
            }
        }
    }
    let fraction = clock::year_fraction(w);
    let blend = clock::blend(w, 0.10);
    let growth_bases = w.province_economy.as_ref().unwrap().day_opening_gdp.clone();
    for n in w.nations.iter_mut().filter(|n| n.alive) {
        let old = prior
            .get(&n.id)
            .copied()
            .unwrap_or(0.0)
            .min(nonnegative(n.gdp));
        let new = next.get(&n.id).copied().unwrap_or(0.0);
        let before = nonnegative(n.gdp);
        // Stable output replaces itself exactly; no repeated +$GDP each day.
        if old != new {
            // A ceased activity cannot annihilate a living national account;
            // retain a $1 annual-output numerical floor, never a cash award.
            n.gdp = ((before - old).max(0.0) + new).max(1e-9);
            let growth_basis = growth_bases.get(&n.id).copied().unwrap_or(before);
            if growth_basis > 0.0 && fraction > 0.0 {
                // Report only the production-level change, not a map/peace
                // transfer. Match the existing smoothed annual-growth display.
                n.growth_last += (n.gdp - before) / growth_basis / fraction * blend;
            }
            // Same stock/output quotient as fiscal and territorial settlement;
            // only a new output level needs refreshing, and no cash moves.
            crate::economy::refresh_debt_ratio(n);
        }
    }
    let accounts = w.province_economy.as_mut().unwrap();
    accounts.posted_contributions = rows;
    accounts.settled_day = Some(today);
}

#[derive(Clone, Debug, Serialize)]
pub struct SectorSnapshot {
    pub id: &'static str,
    pub name: &'static str,
    pub gdp_bn: f64,
    pub share: f64,
}
#[derive(Clone, Debug, Serialize)]
pub struct ProvinceSnapshot {
    pub id: String,
    pub name: String,
    pub nation: NationId,
    pub total_gdp_bn: f64,
    pub inherited_gdp_bn: f64,
    pub project_gdp_bn: f64,
    pub opening_gdp_bn: f64,
    pub change_since_opening: f64,
    pub change_since_opening_bn: f64,
    pub sectors: Vec<SectorSnapshot>,
    pub projects: Vec<gdp_projects::Contribution>,
    pub current_date: Date,
    pub opening_date: Date,
    pub contribution_date: Option<Date>,
    pub settled_day: Option<i32>,
    pub note: &'static str,
}
#[derive(Clone, Debug, Serialize)]
pub struct NationSnapshot {
    pub nation: NationId,
    pub total_gdp_bn: f64,
    pub inherited_gdp_bn: f64,
    pub project_gdp_bn: f64,
    pub opening_gdp_bn: f64,
    pub change_since_opening: f64,
    pub change_since_opening_bn: f64,
    pub unallocated_gdp_bn: f64,
    pub province_count: usize,
    pub sectors: Vec<SectorSnapshot>,
    pub projects: Vec<gdp_projects::Contribution>,
    pub provinces: Vec<ProvinceSnapshot>,
    pub current_date: Date,
    pub opening_date: Date,
    pub contribution_date: Option<Date>,
    pub settled_day: Option<i32>,
    pub note: &'static str,
}
const NOTE:&str="Modeled allocation, not measured provincial GDP: inherited national output is population-weighted at first observation using a generic eight-sector game preset. Weights follow the land and reconcile to its current owner's economy. Project GDP is actual value added, not sales: daily factories use a fixed 365-day annual equivalent; annual-source mines retain their source-year rate. Intermediate production is not counted twice. Change since opening is a level change, not an annual growth rate.";

fn sectors(total: f64, mut values: [f64; 8]) -> Vec<SectorSnapshot> {
    let mut remaining = total;
    (0..8)
        .map(|i| {
            values[i] = if i == 7 {
                remaining
            } else {
                nonnegative(values[i]).min(remaining)
            };
            remaining = (remaining - values[i]).max(0.0);
            SectorSnapshot {
                id: SECTORS[i],
                name: SECTOR_NAMES[i],
                gdp_bn: values[i],
                share: if total > 0.0 { values[i] / total } else { 0.0 },
            }
        })
        .collect()
}
pub fn snapshot(w: &WorldState, nation: NationId) -> Option<NationSnapshot> {
    let ledger = w.province_economy.as_ref()?;
    let n = w.nation_opt(nation)?;
    if !n.alive {
        return None;
    }
    let basis = ledger.nations.get(&nation)?;
    let total = nonnegative(n.gdp);
    let owned: Vec<_> = w
        .districts
        .iter()
        .filter(|(_, owner)| **owner == nation)
        .map(|(d, _)| d)
        .collect();
    // Show newly queued projects immediately, but all monetary/output figures
    // remain the last POSTED observation. Today's incomplete receipts and pure
    // placeholders are not silently promoted into achieved GDP.
    let mut visible: BTreeMap<String, gdp_projects::Contribution> = BTreeMap::new();
    for mut row in gdp_projects::contributions(w) {
        row.annual_gdp_bn = 0.0;
        row.daily_value_added_bn = 0.0;
        row.gross_output_daily_bn = 0.0;
        row.intermediate_inputs_daily_bn = 0.0;
        row.output_quantity_daily = 0.0;
        row.payments_daily_bn = 0.0;
        visible.insert(row.id.clone(), row);
    }
    for row in &ledger.posted_contributions {
        visible.insert(row.id.clone(), row.clone());
    }
    let mut rows: Vec<_> = visible
        .into_values()
        .filter(|r| w.districts.get(&r.district) == Some(&nation))
        .collect();
    let raw_project: f64 = rows
        .iter()
        .filter(|r| r.counted)
        .map(|r| nonnegative(r.annual_gdp_bn))
        .sum();
    let scale = if raw_project > total && raw_project > 0.0 {
        total / raw_project
    } else {
        1.0
    };
    if scale < 1.0 {
        for r in rows.iter_mut().filter(|r| r.counted) {
            adjust_row(r, scale);
        }
    }
    let project: f64 = rows
        .iter()
        .filter(|r| r.counted)
        .map(|r| r.annual_gdp_bn)
        .sum();
    let inherited = (total - project).max(0.0);
    let weights: Vec<f64> = owned
        .iter()
        .map(|d| {
            ledger
                .provinces
                .get(*d)
                .map_or(0.0, |p| nonnegative(p.weight))
        })
        .collect();
    let weight_sum: f64 = weights.iter().sum();
    let mut remainder = inherited;
    let mut provinces = Vec::new();
    let mut national_sectors = [0.0; 8];
    for (i, d) in owned.iter().enumerate() {
        let fallback = ProvinceBasis {
            weight: 1.0,
            opening_gdp_bn: 0.0,
            opening_date: Date::of(w),
            sector_shares: MODEL_SECTOR_SHARES,
        };
        let p = ledger.provinces.get(*d).unwrap_or(&fallback);
        let base = if i + 1 == owned.len() {
            remainder
        } else if weight_sum > 0.0 {
            inherited * weights[i] / weight_sum
        } else {
            inherited / owned.len().max(1) as f64
        };
        let base = base.min(remainder).max(0.0);
        remainder = (remainder - base).max(0.0);
        let local: Vec<_> = rows
            .iter()
            .filter(|r| r.district.as_str() == d.as_str())
            .cloned()
            .collect();
        let added: f64 = local
            .iter()
            .filter(|r| r.counted)
            .map(|r| r.annual_gdp_bn)
            .sum();
        let value = base + added;
        let mut breakdown = std::array::from_fn(|s| base * p.sector_shares[s]);
        for r in local.iter().filter(|r| r.counted) {
            let s = SECTORS.iter().position(|s| *s == r.sector).unwrap_or(6);
            breakdown[s] += r.annual_gdp_bn;
        }
        let breakdown = sectors(value, breakdown);
        for (s, entry) in breakdown.iter().enumerate() {
            national_sectors[s] += entry.gdp_bn;
        }
        provinces.push(ProvinceSnapshot {
            id: (*d).clone(),
            name: districts::name_of(d).unwrap_or(d).to_string(),
            nation,
            total_gdp_bn: value,
            inherited_gdp_bn: base,
            project_gdp_bn: added,
            opening_gdp_bn: p.opening_gdp_bn,
            change_since_opening: if p.opening_gdp_bn > 0.0 {
                value / p.opening_gdp_bn - 1.0
            } else {
                0.0
            },
            change_since_opening_bn: value - p.opening_gdp_bn,
            sectors: breakdown,
            projects: local,
            current_date: Date::of(w),
            opening_date: p.opening_date,
            contribution_date: ledger.settled_day.map(Date::from_day),
            settled_day: ledger.settled_day,
            note: NOTE,
        });
    }
    let unallocated = if owned.is_empty() { total } else { 0.0 };
    if owned.is_empty() {
        national_sectors = std::array::from_fn(|s| total * MODEL_SECTOR_SHARES[s]);
    }
    Some(NationSnapshot {
        nation,
        total_gdp_bn: total,
        inherited_gdp_bn: inherited,
        project_gdp_bn: project,
        opening_gdp_bn: basis.opening_gdp_bn,
        change_since_opening: if basis.opening_gdp_bn > 0.0 {
            total / basis.opening_gdp_bn - 1.0
        } else {
            0.0
        },
        change_since_opening_bn: total - basis.opening_gdp_bn,
        unallocated_gdp_bn: unallocated,
        province_count: provinces.len(),
        sectors: sectors(total, national_sectors),
        projects: rows,
        provinces,
        current_date: Date::of(w),
        opening_date: basis.opening_date,
        contribution_date: ledger.settled_day.map(Date::from_day),
        settled_day: ledger.settled_day,
        note: NOTE,
    })
}
pub fn province(w: &WorldState, district: &str) -> Option<ProvinceSnapshot> {
    let owner = *w.districts.get(district)?;
    snapshot(w, owner)?
        .provinces
        .into_iter()
        .find(|p| p.id == district)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        init::world_1990,
        production::{Priority, Project, ProjectKind, ProjectStatus},
        world::GameRules,
    };
    fn prepared() -> WorldState {
        world_1990(GameRules {
            daily_simulation: true,
            ..GameRules::default()
        })
    }
    fn district(w: &WorldState, n: NationId) -> String {
        w.districts
            .iter()
            .find(|(_, id)| **id == n)
            .unwrap()
            .0
            .clone()
    }
    fn near(a: f64, b: f64) {
        assert!(
            (a - b).abs() < 1e-9 * a.abs().max(b.abs()).max(1.0),
            "{a} != {b}"
        );
    }
    fn receipt(w: &mut WorldState, d: &str, id: &str, value: f64) {
        let row = gdp_projects::Contribution {
            id: id.into(),
            name: "Measured test activity".into(),
            district: d.into(),
            kind: "test".into(),
            sector: "manufacturing".into(),
            classification: "incremental_value_added".into(),
            status: "producing".into(),
            reason: None,
            counted: true,
            annual_gdp_bn: value,
            daily_value_added_bn: value / 365.0,
            gross_output_daily_bn: value / 365.0,
            intermediate_inputs_daily_bn: 0.0,
            output_quantity_daily: 1.0,
            output_unit: "test packs".into(),
            payments_daily_bn: 0.0,
            valuation_basis: "Accounting invariant fixture".into(),
            annualization_days: 365.0,
        };
        w.province_economy
            .as_mut()
            .unwrap()
            .flows
            .receipts
            .insert(id.into(), row);
    }
    fn reconcile(w: &WorldState, n: NationId) {
        let s = snapshot(w, n).unwrap();
        near(s.total_gdp_bn, w.nation(n).gdp);
        near(s.inherited_gdp_bn + s.project_gdp_bn, s.total_gdp_bn);
        near(
            s.provinces.iter().map(|p| p.total_gdp_bn).sum::<f64>() + s.unallocated_gdp_bn,
            s.total_gdp_bn,
        );
        near(s.sectors.iter().map(|s| s.gdp_bn).sum(), s.total_gdp_bn);
        for p in &s.provinces {
            near(p.sectors.iter().map(|s| s.gdp_bn).sum(), p.total_gdp_bn);
            near(p.inherited_gdp_bn + p.project_gdp_bn, p.total_gdp_bn);
            assert!(p
                .sectors
                .iter()
                .all(|s| s.gdp_bn.is_finite() && s.gdp_bn >= 0.0));
        }
    }
    #[test]
    fn opening_covers_every_live_nation_and_reconciles_without_repricing() {
        let mut w = prepared();
        let before: Vec<_> = w.nations.iter().map(|n| n.gdp.to_bits()).collect();
        let rng = serde_json::to_string(&w.rng).unwrap();
        enable(&mut w);
        assert_eq!(
            before,
            w.nations
                .iter()
                .map(|n| n.gdp.to_bits())
                .collect::<Vec<_>>()
        );
        assert_eq!(rng, serde_json::to_string(&w.rng).unwrap());
        for n in w.nations.iter().filter(|n| n.alive) {
            reconcile(&w, n.id);
        }
        let once = crate::save(&w);
        enable(&mut w);
        assert_eq!(once, crate::save(&w));
    }
    #[test]
    fn disabled_ledger_is_absent_and_preserves_the_known_default_start() {
        let mut w = world_1990(GameRules::default());
        let before = crate::save(&w);
        enable(&mut w);
        begin_day(&mut w);
        finish_day(&mut w);
        assert_eq!(before, crate::save(&w));
        assert!(!before.contains("province_economy"));
        assert!(!before.contains("province_investment_reference"));
        assert_eq!(crate::state_hash(&w), 0xe26e4bf8d6c60066);
        let mut daily = prepared();
        let before = crate::save(&daily);
        begin_day(&mut daily);
        finish_day(&mut daily);
        assert_eq!(before, crate::save(&daily));
    }
    #[test]
    fn annual_output_replaces_itself_and_only_inherited_output_compounds() {
        let mut w = prepared();
        enable(&mut w);
        let d = district(&w, NationId::USA);
        let base = w.nation(NationId::USA).gdp;
        for day in 0..40 {
            begin_day(&mut w);
            if day > 0 {
                let level = project_level(&w, NationId::USA);
                apply_macro_factor(&mut w.nation_mut(NationId::USA).gdp, level, 1.001);
            }
            receipt(&mut w, &d, "stable", 10.0);
            finish_day(&mut w);
            let once = crate::save(&w);
            finish_day(&mut w);
            assert_eq!(once, crate::save(&w));
            near(
                w.nation(NationId::USA).gdp,
                base * 1.001_f64.powi(day) + 10.0,
            );
            reconcile(&w, NationId::USA);
            clock::advance_date(&mut w);
        }
        begin_day(&mut w);
        finish_day(&mut w);
        near(w.nation(NationId::USA).gdp, base * 1.001_f64.powi(39));
        assert_eq!(
            snapshot(&w, NationId::USA).unwrap().project_gdp_bn,
            0.0,
            "a blocked/stopped day produces nothing"
        );
    }

    #[test]
    fn finance_province_output_refreshes_debt_ratio_without_touching_cash() {
        let mut w = prepared();
        let id = NationId::Tonga;
        let n = w.nation_mut(id);
        n.gdp = 0.05;
        n.treasury_bn = Some(1.0);
        n.debt_bn = Some(0.04);
        n.debt_gdp = 0.8;
        enable(&mut w);
        let d = district(&w, id);
        for annual_output in [0.01, 0.01, 0.0] {
            begin_day(&mut w);
            if annual_output > 0.0 {
                receipt(&mut w, &d, "measured_site", annual_output);
            }
            finish_day(&mut w);
            let n = w.nation(id);
            assert!((n.gdp - (0.05 + annual_output)).abs() < 1e-15);
            assert_eq!(n.debt_gdp, n.debt_bn.unwrap() / n.gdp);
            assert_eq!(n.debt_bn, Some(0.04));
            assert_eq!(n.treasury_bn, Some(1.0));
            let saved = crate::save(&w);
            finish_day(&mut w);
            assert_eq!(crate::save(&w), saved, "closing a day twice is inert");
            w = crate::load(&saved).unwrap();
            clock::advance_date(&mut w);
        }
    }
    #[test]
    fn consent_reattributes_project_output_without_transferring_gdp_twice() {
        let mut w = prepared();
        enable(&mut w);
        let d = district(&w, NationId::USA);
        begin_day(&mut w);
        receipt(&mut w, &d, "site", 10.0);
        finish_day(&mut w);
        clock::advance_date(&mut w);
        districts::transfer_district(&mut w, NationId::USA, NationId::Germany, &d).unwrap();
        let a = w.nation(NationId::USA).gdp;
        let b = w.nation(NationId::Germany).gdp;
        let growth = w.nation(NationId::Germany).growth_last;
        begin_day(&mut w);
        receipt(&mut w, &d, "site", 10.0);
        finish_day(&mut w);
        assert_eq!(w.nation(NationId::USA).gdp, a);
        assert_eq!(w.nation(NationId::Germany).gdp, b);
        assert_eq!(w.nation(NationId::Germany).growth_last, growth);
        assert_eq!(
            snapshot(&w, NationId::Germany).unwrap().project_gdp_bn,
            10.0
        );
        assert_eq!(snapshot(&w, NationId::USA).unwrap().project_gdp_bn, 0.0);
        reconcile(&w, NationId::USA);
        reconcile(&w, NationId::Germany);
    }
    #[test]
    fn annexation_keeps_war_damage_and_empty_countries_have_explicit_remainders() {
        let mut w = prepared();
        enable(&mut w);
        let d = district(&w, NationId::Kuwait);
        begin_day(&mut w);
        receipt(&mut w, &d, "mine", 1.0);
        finish_day(&mut w);
        clock::advance_date(&mut w);
        let damaged = w.nation(NationId::USA).gdp + w.nation(NationId::Kuwait).gdp * 0.75;
        w.nation_mut(NationId::USA).gdp = damaged;
        w.nation_mut(NationId::Kuwait).gdp = 0.0;
        w.nation_mut(NationId::Kuwait).alive = false;
        districts::annex_all(&mut w, NationId::USA, NationId::Kuwait);
        begin_day(&mut w);
        receipt(&mut w, &d, "mine", 1.0);
        finish_day(&mut w);
        assert_eq!(w.nation(NationId::USA).gdp, damaged);
        assert!(snapshot(&w, NationId::Kuwait).is_none());
        reconcile(&w, NationId::USA);
        w.districts.retain(|_, owner| *owner != NationId::France);
        let s = snapshot(&w, NationId::France).unwrap();
        assert_eq!(s.province_count, 0);
        assert_eq!(s.unallocated_gdp_bn, s.total_gdp_bn);
        reconcile(&w, NationId::France);
    }
    #[test]
    fn pending_transition_and_save_resume_preserve_accounting_dates() {
        let mut w = world_1990(GameRules::default());
        w.day = 30;
        clock::enable_daily_play(&mut w);
        enable(&mut w);
        assert!(!active(&w));
        assert!(w
            .nations
            .iter()
            .all(|n| n.province_investment_reference.is_none()));
        crate::tick_day(&mut w, &[]);
        crate::tick_day(&mut w, &[]);
        assert!(active(&w));
        let mut copy = crate::load(&crate::save(&w)).unwrap();
        for _ in 0..8 {
            crate::tick_day(&mut w, &[]);
            crate::tick_day(&mut copy, &[]);
        }
        assert_eq!(crate::save(&w), crate::save(&copy));
        let s = snapshot(&w, NationId::USA).unwrap();
        assert_eq!(s.current_date.day, 9);
        assert_eq!(s.contribution_date.unwrap().day, 8);
    }
    #[test]
    fn first_observed_existing_asset_is_absorbed_but_subsequent_upgrades_are_not() {
        let mut w = prepared();
        let d = district(&w, NationId::USA);
        w.production
            .industry
            .sites
            .insert(d.clone(), [0, 0, 1, 0, 0, 0, 0]);
        enable(&mut w);
        let base = w.nation(NationId::USA).gdp;
        let id = format!("site:{d}:processing_plant");
        w.production.industry.sites.get_mut(&d).unwrap()[2] = 2;
        begin_day(&mut w);
        receipt(&mut w, &d, &id, 2.0);
        finish_day(&mut w);
        near(w.nation(NationId::USA).gdp, base + 1.0);
    }
    #[test]
    fn frozen_public_investment_prevents_a_second_project_growth_channel() {
        let mut w = prepared();
        w.player = Some(NationId::USA);
        w.nation_mut(NationId::USA).political_capital = 100.0;
        let allocations = w.nation(NationId::USA).budget_for(w.year).allocations;
        crate::apply_command(
            &mut w,
            &crate::Command::SetProgramBudget {
                nation: NationId::USA,
                fiscal_year: 1990,
                allocations,
                departments: crate::programs::default_departments(),
            },
        )
        .unwrap();
        let old = crate::programs::effective_investment(w.nation(NationId::USA), 0.9);
        enable(&mut w);
        w.nation_mut(NationId::USA)
            .program_budget
            .as_mut()
            .unwrap()
            .realized_investment_share = 0.65;
        assert_eq!(
            crate::programs::effective_investment(w.nation(NationId::USA), 0.9),
            old
        );
        let mut unenrolled = prepared();
        let n = unenrolled.nation_mut(NationId::USA);
        assert_eq!(crate::programs::effective_investment(n, 0.9), 0.9);
    }
    #[test]
    fn new_project_metadata_is_visible_but_unposted_work_has_zero_gdp() {
        let mut w = prepared();
        enable(&mut w);
        let d = district(&w, NationId::USA);
        w.production.projects.push(Project {
            id: 7,
            nation: NationId::USA,
            district: d.clone(),
            kind: ProjectKind::CivilianIndustry,
            priority: Priority::Normal,
            status: ProjectStatus::Building,
            reason: None,
            progress_days: 0.0,
            total_days: 100,
            resources_used: [0.0; 12],
        });
        let before = crate::save(&w);
        let s = province(&w, &d).unwrap();
        assert!(s.projects.iter().any(|r| r.id == "construction:7"));
        assert_eq!(s.project_gdp_bn, 0.0);
        assert_eq!(before, crate::save(&w));
        begin_day(&mut w);
        receipt(&mut w, &d, "today", 2.0);
        assert_eq!(province(&w, &d).unwrap().project_gdp_bn, 0.0);
        finish_day(&mut w);
        let s = province(&w, &d).unwrap();
        near(
            s.change_since_opening,
            s.total_gdp_bn / s.opening_gdp_bn - 1.0,
        );
        assert_eq!(s.project_gdp_bn, 2.0);
    }
    #[test]
    fn nonfinite_receipt_cannot_poison_gdp_and_microstate_debt_uses_actual_output() {
        let mut w = prepared();
        enable(&mut w);
        let d = district(&w, NationId::USA);
        begin_day(&mut w);
        let original = w.nation(NationId::USA).gdp;
        receipt(&mut w, &d, "bad", f64::NAN);
        finish_day(&mut w);
        assert_eq!(w.nation(NationId::USA).gdp, original);
        reconcile(&w, NationId::USA);
        clock::advance_date(&mut w);
        w.nation_mut(NationId::USA).gdp = 0.02;
        // Dollar debt and treasury are seated together; a half-open fixture
        // would exercise the legacy ratio arm, not this open-book invariant.
        w.nation_mut(NationId::USA).treasury_bn = Some(0.0);
        w.nation_mut(NationId::USA).debt_bn = Some(0.01);
        begin_day(&mut w);
        receipt(&mut w, &d, "tiny", 0.01);
        finish_day(&mut w);
        near(w.nation(NationId::USA).debt_gdp, 1.0 / 3.0);
        assert_eq!(w.nation(NationId::USA).debt_bn, Some(0.01));
        assert_eq!(w.nation(NationId::USA).treasury_bn, Some(0.0));
    }
    #[test]
    fn output_heavy_cession_to_small_economy_cannot_mint_gdp_on_transfer_or_restart() {
        let mut w = prepared();
        w.nation_mut(NationId::USA).gdp = 50.0;
        w.nation_mut(NationId::Germany).gdp = 1.0;
        enable(&mut w);
        let d = district(&w, NationId::USA);
        begin_day(&mut w);
        receipt(&mut w, &d, "heavy", 50.0);
        finish_day(&mut w);
        clock::advance_date(&mut w);
        districts::transfer_district(&mut w, NationId::USA, NationId::Germany, &d).unwrap();
        let donor = w.nation(NationId::USA).gdp;
        let receiver = w.nation(NationId::Germany).gdp;
        assert!(receiver < 50.0);
        begin_day(&mut w);
        receipt(&mut w, &d, "heavy", 50.0);
        finish_day(&mut w);
        assert_eq!(w.nation(NationId::USA).gdp, donor);
        assert_eq!(w.nation(NationId::Germany).gdp, receiver);
        let s = snapshot(&w, NationId::Germany).unwrap();
        reconcile(&w, NationId::Germany);
        let row = s.projects.iter().find(|r| r.id == "heavy").unwrap();
        near(
            row.annual_gdp_bn,
            row.daily_value_added_bn * row.annualization_days,
        );
        near(
            row.daily_value_added_bn,
            row.gross_output_daily_bn - row.intermediate_inputs_daily_bn,
        );
        assert!(row.valuation_basis.contains("reconciliation"));
        clock::advance_date(&mut w);
        begin_day(&mut w);
        finish_day(&mut w);
        assert!(w.nation(NationId::Germany).gdp >= 0.0);
        w = crate::load(&crate::save(&w)).unwrap();
        clock::advance_date(&mut w);
        begin_day(&mut w);
        receipt(&mut w, &d, "heavy", 50.0);
        finish_day(&mut w);
        near(w.nation(NationId::Germany).gdp, receiver);
        reconcile(&w, NationId::Germany);
    }
    #[test]
    fn zero_activity_enable_preserves_every_other_daily_state_field() {
        let mut old = prepared();
        let mut accounted = old.clone();
        enable(&mut accounted);
        for _ in 0..90 {
            crate::tick_day(&mut old, &[]);
            crate::tick_day(&mut accounted, &[]);
        }
        let expected = serde_json::to_value(&old).unwrap();
        let mut actual = serde_json::to_value(&accounted).unwrap();
        actual.as_object_mut().unwrap().remove("province_economy");
        for n in actual["nations"].as_array_mut().unwrap() {
            n.as_object_mut()
                .unwrap()
                .remove("province_investment_reference");
        }
        assert_eq!(actual,expected,"accounting-only activation cannot change macro, politics, RNG or money without project activity");
    }
    #[test]
    fn oversized_inherited_asset_first_observation_is_not_an_upgrade_windfall() {
        for upgrade in [false, true] {
            let mut w = prepared();
            w.nation_mut(NationId::USA).gdp = 1.0;
            let d = district(&w, NationId::USA);
            let id = format!("site:{d}:processing_plant");
            w.production
                .industry
                .sites
                .insert(d.clone(), [0, 0, 1, 0, 0, 0, 0]);
            enable(&mut w);
            if upgrade {
                w.production.industry.sites.get_mut(&d).unwrap()[2] = 2;
            }
            let value = if upgrade { 100.0 } else { 50.0 };
            let expected = if upgrade { 51.0 } else { 1.0 };
            begin_day(&mut w);
            receipt(&mut w, &d, &id, value);
            finish_day(&mut w);
            near(w.nation(NationId::USA).gdp, expected);
            reconcile(&w, NationId::USA);
            let row = snapshot(&w, NationId::USA)
                .unwrap()
                .projects
                .into_iter()
                .find(|r| r.id == id)
                .unwrap();
            near(
                row.annual_gdp_bn,
                row.daily_value_added_bn * row.annualization_days,
            );
            clock::advance_date(&mut w);
            begin_day(&mut w);
            receipt(&mut w, &d, &id, value);
            finish_day(&mut w);
            near(w.nation(NationId::USA).gdp, expected);
        }
    }
}
