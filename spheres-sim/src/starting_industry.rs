//! The inherited 1990 manufacturing economy, in explicit GAME equivalents.
//!
//! These are aggregate capacity estimates, not a census of establishments and
//! not the recipe-driven plants in `production`. Their output is ALREADY in
//! macro GDP. This module never grants packs, money, construction work, resource
//! consumption or another growth arm. Only an explicit fresh-world opt-in seeds
//! frozen records; reading, loading, economic growth and conquest never seed.
use crate::{
    clock, districts, province_economy,
    world::{start_nations, NationId, WorldState},
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

/// MODEL conversion, not an observed factory size or utilization statistic.
/// One equivalent has $100m of annual value-added capacity at full utilization.
pub const ANNUAL_CAPACITY_PER_EQUIVALENT_BN: f64 = 0.1;
pub const STARTING_UTILIZATION: f64 = 0.8;
pub const GROUP_KEYS: [&str; 5] = [
    "food_textiles",
    "materials",
    "chemicals",
    "machinery_electronics",
    "other",
];
pub const GROUP_NAMES: [&str; 5] = [
    "Food & textiles",
    "Materials",
    "Chemicals",
    "Machinery & electronics",
    "Other manufacturing",
];
pub const ALLOCATION_BASIS: &str = "Population-weighted game allocation, not historical factory locations. Countries missing map coverage retain an unallocated national account.";
pub const NOTE: &str = "Estimated factory equivalents, not literal establishments. Inherited manufacturing output is already included in GDP; no goods, cash, operating inputs or construction are granted. Capacity stays frozen while the existing macroeconomy changes its output. Utilization above 100% indicates that output has outgrown the opening estimate, not new automatically built factories.";

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SectorWeights {
    pub food_textiles: f64,
    pub materials: f64,
    pub chemicals: f64,
    pub machinery_electronics: f64,
    pub other: f64,
}
impl SectorWeights {
    pub fn values(&self) -> [f64; 5] {
        [
            self.food_textiles,
            self.materials,
            self.chemicals,
            self.machinery_electronics,
            self.other,
        ]
    }
}
/// Persisted provenance: updating the source artifact cannot silently rewrite
/// the mix/quality of an already running campaign.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CountryProfile {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_gdp_usd: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_mva_usd: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_mva_share: Option<f64>,
    pub manufacturing_share: f64,
    pub share_quality: String,
    pub sector_weights: SectorWeights,
    pub mix_quality: String,
    #[serde(deserialize_with = "source_text")]
    pub source: String,
    #[serde(deserialize_with = "notes_text")]
    pub notes: String,
}
// The source artifact keeps structured provenance. Persist its complete JSON
// when it is structured, not merely one URL, so saves retain the observations,
// series, proxy choices and caveats even when the data artifact changes later.
// Already-string fields in a saved profile round-trip without reformatting.
fn source_text<'de, D: serde::Deserializer<'de>>(d: D) -> Result<String, D::Error> {
    let value = serde_json::Value::deserialize(d)?;
    match value {
        serde_json::Value::String(s) => Ok(s),
        serde_json::Value::Null => Err(serde::de::Error::custom("industry source must not be null")),
        other => serde_json::to_string_pretty(&other).map_err(serde::de::Error::custom),
    }
}
fn notes_text<'de, D: serde::Deserializer<'de>>(d: D) -> Result<String, D::Error> {
    let value = serde_json::Value::deserialize(d)?;
    match value {
        serde_json::Value::String(s) => Ok(s),
        serde_json::Value::Array(rows) if rows.iter().all(serde_json::Value::is_string) => {
            Ok(rows.iter().map(|r| r.as_str().unwrap()).collect::<Vec<_>>().join("\n"))
        }
        _ => Err(serde::de::Error::custom("industry notes must be text or a list of text notes")),
    }
}
#[derive(Clone, Debug, Deserialize)]
pub struct Dataset {
    pub schema_version: u32,
    pub countries: BTreeMap<NationId, CountryProfile>,
}

pub fn validate_data(data: &Dataset) -> Result<(), String> {
    if data.schema_version != 1 {
        return Err("1990 industry data requires schema_version 1".into());
    }
    let expected: BTreeSet<_> = start_nations().iter().copied().collect();
    if data.countries.keys().copied().collect::<BTreeSet<_>>() != expected {
        return Err("1990 industry data must cover exactly the canonical starting roster".into());
    }
    for (id, row) in &data.countries {
        if [row.source_gdp_usd, row.source_mva_usd, row.source_mva_share]
            .into_iter().flatten().any(|v| !v.is_finite() || v < 0.0)
        {
            return Err(format!("{}: historical source observations must be finite nonnegative values or null", id.code()));
        }
        if !row.manufacturing_share.is_finite() || !(0.0..1.0).contains(&row.manufacturing_share) {
            return Err(format!(
                "{}: manufacturing share must be finite and between zero and one",
                id.code()
            ));
        }
        let weights = row.sector_weights.values();
        if weights
            .iter()
            .any(|x| !x.is_finite() || *x < 0.0 || *x > 1.0)
            || (weights.iter().sum::<f64>() - 1.0).abs() > 1e-9
        {
            return Err(format!(
                "{}: non-overlapping manufacturing group weights must sum to one",
                id.code()
            ));
        }
        if [
            &row.share_quality,
            &row.mix_quality,
            &row.source,
            &row.notes,
        ]
        .iter()
        .any(|s| s.trim().is_empty())
        {
            return Err(format!(
                "{}: source, quality and method notes are required",
                id.code()
            ));
        }
    }
    Ok(())
}
pub fn data() -> &'static Dataset {
    static DATA: OnceLock<Dataset> = OnceLock::new();
    DATA.get_or_init(|| {
        let data: Dataset = serde_json::from_str(include_str!("../data/industry_1990.json"))
            .expect("parse sourced/labelled 1990 game-capacity dataset");
        validate_data(&data).expect("validate sourced/labelled 1990 game-capacity dataset");
        data
    })
}

/// Five aggregate asset records per location, represented compactly. Array
/// ordering is GROUP_KEYS and is versioned with the saved schema. Neither
/// quantities nor opening output are recomputed from later GDP/population.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct InheritedAssets {
    pub origin: NationId,
    pub opening_gdp_bn: f64,
    pub factory_equivalents: [f64; 5],
    pub opening_output_annual_bn: [f64; 5],
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct StartingIndustry {
    pub schema_version: u32,
    pub annual_capacity_per_equivalent_bn: f64,
    pub starting_utilization: f64,
    pub profiles: BTreeMap<NationId, CountryProfile>,
    pub provinces: BTreeMap<String, InheritedAssets>,
    pub unallocated: BTreeMap<NationId, InheritedAssets>,
}

fn finite_nonnegative(v: f64) -> f64 {
    if v.is_finite() {
        v.max(0.0)
    } else {
        0.0
    }
}

fn allocated_assets(
    origin: NationId,
    opening_gdp_bn: f64,
    profile: &CountryProfile,
) -> InheritedAssets {
    let manufacturing = opening_gdp_bn * profile.manufacturing_share;
    let weights = profile.sector_weights.values();
    let mut remaining = manufacturing;
    let opening_output_annual_bn = std::array::from_fn(|i| {
        let value = if i == 4 {
            remaining
        } else {
            (manufacturing * weights[i]).min(remaining)
        };
        remaining = (remaining - value).max(0.0);
        value
    });
    InheritedAssets {
        origin,
        opening_gdp_bn,
        factory_equivalents: opening_output_annual_bn
            .map(|output| output / (ANNUAL_CAPACITY_PER_EQUIVALENT_BN * STARTING_UTILIZATION)),
        opening_output_annual_bn,
    }
}

/// Explicit INITIALIZATION operation, invoked by a new browser campaign only.
/// Not a migration, not called by `world_1990`, load, enable or the daily tick.
/// Returns false for an already-seeded campaign without changing it.
pub fn enable_new_world(w: &mut WorldState) -> Result<bool, String> {
    if w.starting_industry.is_some() {
        return Ok(false);
    }
    if !clock::is_daily(w) || (w.year, w.month, w.day) != (1990, 1, 1) {
        return Err("Inherited 1990 capacity can only seed a new daily campaign on 1 January 1990; existing campaigns are not backfilled.".into());
    }
    if w.province_economy.is_some()
        || !w.production.is_empty()
        || !w.manufacturing.is_empty()
        || !w.resources.is_empty()
        || !w.logistics.is_empty()
        || w.commerce.is_some()
        || w.materials.is_some()
        || !w.daily.is_empty()
        || !w.economic_ai.is_empty()
        || w.districts != districts::ownership_1990()
        || w.nations
            .iter()
            .any(|n| n.program_budget.is_some() || n.annual_budget.is_some())
    {
        return Err("Inherited capacity must be registered before a new campaign opens its accounts or performs work; existing activity is preserved.".into());
    }
    let expected: BTreeSet<_> = start_nations().iter().copied().collect();
    if w.nations
        .iter()
        .filter(|n| n.alive)
        .map(|n| n.id)
        .collect::<BTreeSet<_>>()
        != expected
    {
        return Err(
            "Inherited capacity requires the canonical 1990 country roster, not successor states."
                .into(),
        );
    }
    let profiles = data().countries.clone();
    let mut state = StartingIndustry {
        schema_version: 1,
        annual_capacity_per_equivalent_bn: ANNUAL_CAPACITY_PER_EQUIVALENT_BN,
        starting_utilization: STARTING_UTILIZATION,
        profiles,
        provinces: BTreeMap::new(),
        unallocated: BTreeMap::new(),
    };
    for &id in start_nations() {
        let gdp = w.nation(id).gdp;
        if !gdp.is_finite() || gdp < 0.0 {
            return Err(format!("{} has invalid opening GDP", id.code()));
        }
        let profile = &state.profiles[&id];
        let owned: Vec<_> = w
            .districts
            .iter()
            .filter(|(_, owner)| **owner == id)
            .map(|(d, _)| d)
            .collect();
        if owned.is_empty() {
            state
                .unallocated
                .insert(id, allocated_assets(id, gdp, profile));
            continue;
        }
        let masses: Vec<_> = owned
            .iter()
            .map(|d| finite_nonnegative(districts::population_of(w, d).unwrap_or(0.0)))
            .collect();
        let mass: f64 = masses.iter().sum();
        let mut remaining = gdp;
        for (i, district) in owned.iter().enumerate() {
            let amount = if i + 1 == owned.len() {
                remaining
            } else if mass > 0.0 {
                gdp * masses[i] / mass
            } else {
                gdp / owned.len() as f64
            };
            let amount = amount.min(remaining).max(0.0);
            remaining = (remaining - amount).max(0.0);
            state
                .provinces
                .insert((*district).clone(), allocated_assets(id, amount, profile));
        }
    }
    w.starting_industry = Some(state);
    Ok(true)
}

/// Manufacturing receives its sourced/proxied national share. Other sectors
/// retain their relative game presets, rescaled into the remaining GDP; this
/// does not claim that agriculture/services/etc. have historical shares.
fn shares(profile: &CountryProfile) -> [f64; 8] {
    let mut out = province_economy::MODEL_SECTOR_SHARES;
    let scale = (1.0 - profile.manufacturing_share) / (1.0 - out[2]);
    for (i, value) in out.iter_mut().enumerate() {
        *value = if i == 2 {
            profile.manufacturing_share
        } else {
            *value * scale
        };
    }
    out
}
pub fn sector_shares(w: &WorldState, nation: NationId, district: Option<&str>) -> [f64; 8] {
    let Some(state) = &w.starting_industry else {
        return province_economy::MODEL_SECTOR_SHARES;
    };
    let origin = match district {
        Some(d) => state.provinces.get(d).map(|a| a.origin),
        None => state.unallocated.get(&nation).map(|a| a.origin),
    };
    origin
        .and_then(|id| state.profiles.get(&id))
        .map_or(province_economy::MODEL_SECTOR_SHARES, shares)
}
/// If an initially unmapped nation later acquires a mapped district, retain its
/// unlocated inherited output as a national share instead of silently relocating
/// all old factories to the conquered province.
pub fn unallocated_weight(w: &WorldState, nation: NationId) -> f64 {
    w.starting_industry
        .as_ref()
        .and_then(|s| s.unallocated.get(&nation))
        .map_or(0.0, |a| a.opening_gdp_bn)
}

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct GroupSnapshot {
    pub key: &'static str,
    pub name: &'static str,
    pub factory_equivalents: f64,
    pub capacity_annual_bn: f64,
    pub opening_output_annual_bn: f64,
    pub current_output_annual_bn: f64,
    pub utilization: f64,
}
#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct SourceSnapshot {
    pub origin: NationId,
    pub source_gdp_usd: Option<f64>,
    pub source_mva_usd: Option<f64>,
    pub source_mva_share: Option<f64>,
    pub manufacturing_share: f64,
    pub share_quality: String,
    pub mix_quality: String,
    pub source: String,
    pub notes: String,
}
#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct ProvinceSnapshot {
    pub district: String,
    pub nation: NationId,
    pub origin: NationId,
    pub factory_equivalents: f64,
    pub capacity_annual_bn: f64,
    pub opening_output_annual_bn: f64,
    pub current_output_annual_bn: f64,
    pub utilization: f64,
    pub groups: Vec<GroupSnapshot>,
    pub source: SourceSnapshot,
    pub allocation_basis: &'static str,
    pub note: &'static str,
}
#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct NationSnapshot {
    pub nation: NationId,
    pub factory_equivalents: f64,
    pub capacity_annual_bn: f64,
    pub opening_output_annual_bn: f64,
    pub current_output_annual_bn: f64,
    pub utilization: f64,
    pub unallocated_factory_equivalents: f64,
    pub province_count: usize,
    pub annual_capacity_per_equivalent_bn: f64,
    pub starting_utilization: f64,
    pub groups: Vec<GroupSnapshot>,
    pub sources: Vec<SourceSnapshot>,
    pub allocation_basis: &'static str,
    pub note: &'static str,
}
fn source(origin: NationId, profile: &CountryProfile) -> SourceSnapshot {
    SourceSnapshot {
        origin,
        source_gdp_usd: profile.source_gdp_usd,
        source_mva_usd: profile.source_mva_usd,
        source_mva_share: profile.source_mva_share,
        manufacturing_share: profile.manufacturing_share,
        share_quality: profile.share_quality.clone(),
        mix_quality: profile.mix_quality.clone(),
        source: profile.source.clone(),
        notes: profile.notes.clone(),
    }
}
fn utilization(output: f64, capacity: f64) -> f64 {
    if capacity > 0.0 {
        output / capacity
    } else {
        0.0
    }
}
fn groups(
    state: &StartingIndustry,
    assets: &InheritedAssets,
    inherited_gdp: f64,
) -> Vec<GroupSnapshot> {
    let profile = &state.profiles[&assets.origin];
    let current = finite_nonnegative(inherited_gdp) * profile.manufacturing_share;
    let weights = profile.sector_weights.values();
    let mut remaining = current;
    (0..5)
        .map(|i| {
            let output = if i == 4 {
                remaining
            } else {
                (current * weights[i]).min(remaining)
            };
            remaining = (remaining - output).max(0.0);
            let capacity = assets.factory_equivalents[i] * state.annual_capacity_per_equivalent_bn;
            GroupSnapshot {
                key: GROUP_KEYS[i],
                name: GROUP_NAMES[i],
                factory_equivalents: assets.factory_equivalents[i],
                capacity_annual_bn: capacity,
                opening_output_annual_bn: assets.opening_output_annual_bn[i],
                current_output_annual_bn: output,
                utilization: utilization(output, capacity),
            }
        })
        .collect()
}
fn empty_groups() -> Vec<GroupSnapshot> {
    (0..5)
        .map(|i| GroupSnapshot {
            key: GROUP_KEYS[i],
            name: GROUP_NAMES[i],
            factory_equivalents: 0.0,
            capacity_annual_bn: 0.0,
            opening_output_annual_bn: 0.0,
            current_output_annual_bn: 0.0,
            utilization: 0.0,
        })
        .collect()
}
fn add_groups(total: &mut [GroupSnapshot], rows: &[GroupSnapshot]) {
    for (sum, row) in total.iter_mut().zip(rows) {
        sum.factory_equivalents += row.factory_equivalents;
        sum.capacity_annual_bn += row.capacity_annual_bn;
        sum.opening_output_annual_bn += row.opening_output_annual_bn;
        sum.current_output_annual_bn += row.current_output_annual_bn;
        sum.utilization = utilization(sum.current_output_annual_bn, sum.capacity_annual_bn);
    }
}

pub fn snapshot(w: &WorldState, nation: NationId) -> Option<NationSnapshot> {
    let state = w.starting_industry.as_ref()?;
    let economy = province_economy::snapshot(w, nation)?;
    let mut result = empty_groups();
    let mut origins = BTreeSet::new();
    let mut province_count = 0;
    for province in &economy.provinces {
        if let Some(assets) = state.provinces.get(&province.id) {
            add_groups(
                &mut result,
                &groups(state, assets, province.inherited_gdp_bn),
            );
            origins.insert(assets.origin);
            province_count += 1;
        }
    }
    let mut unallocated_factory_equivalents = 0.0;
    if let Some(assets) = state.unallocated.get(&nation) {
        add_groups(
            &mut result,
            &groups(state, assets, economy.unallocated_gdp_bn),
        );
        unallocated_factory_equivalents = assets.factory_equivalents.iter().sum();
        origins.insert(assets.origin);
    }
    let capacity: f64 = result.iter().map(|g| g.capacity_annual_bn).sum();
    let current: f64 = result.iter().map(|g| g.current_output_annual_bn).sum();
    Some(NationSnapshot {
        nation,
        factory_equivalents: result.iter().map(|g| g.factory_equivalents).sum(),
        capacity_annual_bn: capacity,
        opening_output_annual_bn: result.iter().map(|g| g.opening_output_annual_bn).sum(),
        current_output_annual_bn: current,
        utilization: utilization(current, capacity),
        unallocated_factory_equivalents,
        province_count,
        annual_capacity_per_equivalent_bn: state.annual_capacity_per_equivalent_bn,
        starting_utilization: state.starting_utilization,
        groups: result,
        sources: origins
            .into_iter()
            .map(|id| source(id, &state.profiles[&id]))
            .collect(),
        allocation_basis: ALLOCATION_BASIS,
        note: NOTE,
    })
}
pub fn province(w: &WorldState, district: &str) -> Option<ProvinceSnapshot> {
    let state = w.starting_industry.as_ref()?;
    let assets = state.provinces.get(district)?;
    let economy = province_economy::province(w, district)?;
    let groups = groups(state, assets, economy.inherited_gdp_bn);
    let capacity = groups.iter().map(|g| g.capacity_annual_bn).sum();
    let current = groups.iter().map(|g| g.current_output_annual_bn).sum();
    Some(ProvinceSnapshot {
        district: district.into(),
        nation: economy.nation,
        origin: assets.origin,
        factory_equivalents: groups.iter().map(|g| g.factory_equivalents).sum(),
        capacity_annual_bn: capacity,
        opening_output_annual_bn: groups.iter().map(|g| g.opening_output_annual_bn).sum(),
        current_output_annual_bn: current,
        utilization: utilization(current, capacity),
        groups,
        source: source(assets.origin, &state.profiles[&assets.origin]),
        allocation_basis: ALLOCATION_BASIS,
        note: NOTE,
    })
}
