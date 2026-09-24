//! Observed military leverage over executive removal, distinct from Army
//! resources, strength, physical autonomy and the probability of a coup.
//! Historical inputs never replay. The campaign's current leverage can change
//! through an actual military seizure or a qualifying civilian handover.
use crate::{government::{self, Pillar}, world::{NationId, WorldState}};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, sync::OnceLock};

/// Immutable assessment imported from the named historical source. These are
/// expert response means and executive weights, not observed coup frequencies.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceAssessment {
    pub nation: NationId,
    pub source_key: String,
    pub source_year: i32,
    pub hos_removal: Option<f64>,
    pub hog_removal: Option<f64>,
    pub hos_weight: f64,
    pub hog_weight: f64,
    pub assessment: f64,
}

/// A saved source snapshot and its separate, mutable campaign state. Missing
/// old saves remain unknown rather than receiving fabricated historical data.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArmyAuthority {
    pub source: SourceAssessment,
    pub current_leverage: f64,
    pub updated_on: (i32, u32, u32),
}

#[derive(Deserialize)]
struct Catalog {
    version: u32,
    as_of: String,
    coverage: String,
    rows: Vec<SourceAssessment>,
}

pub const EMBEDDED_AUTHORITY: &str = include_str!("../data/army_authority_1989.json");

fn unit(value: f64) -> bool { value.is_finite() && (0.0..=1.0).contains(&value) }

fn valid_assessment(row: &SourceAssessment) -> bool {
    row.source_year == 1989 && !row.source_key.trim().is_empty()
        && unit(row.hos_weight) && unit(row.hog_weight)
        && (row.hos_weight + row.hog_weight - 1.0).abs() <= 1e-9
        && row.hos_removal.is_none_or(unit) && row.hog_removal.is_none_or(unit)
        && (row.hos_weight == 0.0 || row.hos_removal.is_some())
        && (row.hog_weight == 0.0 || row.hog_removal.is_some())
        && unit(row.assessment)
        && (row.assessment - row.hos_weight * row.hos_removal.unwrap_or(0.0)
            - row.hog_weight * row.hog_removal.unwrap_or(0.0)).abs() <= 1e-9
}

pub fn parse_catalog(json: &str) -> Result<Vec<SourceAssessment>, String> {
    let catalog: Catalog = serde_json::from_str(json).map_err(|e| e.to_string())?;
    if catalog.version != 1 || catalog.as_of != "1990-01-01" || catalog.coverage.trim().is_empty() {
        return Err("authority catalog requires version 1, opening date and coverage".into());
    }
    let opening: BTreeSet<_> = crate::data::parse_nations(crate::data::EMBEDDED_NATIONS)
        .map_err(|_| "invalid opening roster")?.into_iter().map(|row| row.id).collect();
    let mut ids = BTreeSet::new(); let mut keys = BTreeSet::new();
    for row in &catalog.rows {
        if !valid_assessment(row) || !ids.insert(row.nation) || !keys.insert(&row.source_key) {
            return Err(format!("invalid or repeated 1989 authority assessment: {:?}", row.nation));
        }
        if !opening.contains(&row.nation) {
            return Err(format!("authority assessment is not an opening nation: {:?}", row.nation));
        }
    }
    Ok(catalog.rows)
}

pub fn sources() -> &'static [SourceAssessment] {
    static ROWS: OnceLock<Vec<SourceAssessment>> = OnceLock::new();
    ROWS.get_or_init(|| parse_catalog(EMBEDDED_AUTHORITY)
        .unwrap_or_else(|e| panic!("army_authority_1989.json: {e}")))
}

fn model_date(w: &WorldState) -> (i32, u32, u32) {
    (w.year, w.month, if crate::clock::is_daily(w) { w.day.max(1) } else { 1 })
}

/// Called only by fresh world_1990 initialization, never ensure or load.
pub(crate) fn prepare_opening(w: &mut WorldState) {
    if !w.rules.ideology_blocs || (w.year, w.month, w.day) != (1990, 1, 1) { return; }
    for source in sources() {
        if !w.nation_opt(source.nation).is_some_and(|n| n.alive) { continue; }
        let Some(g) = w.governments.states.iter_mut().find(|g| g.nation == source.nation) else { continue; };
        if g.army_authority.is_none() && g.pillars.iter().any(|(p, _)| *p == Pillar::Army) {
            g.army_authority = Some(ArmyAuthority {
                source: source.clone(), current_leverage: source.assessment,
                updated_on: (1990, 1, 1),
            });
        }
    }
}

/// Read the saved campaign assessment. Absence or malformed saved provenance
/// stays unknown so the caller can keep its explicit legacy proxy.
pub fn current_leverage(w: &WorldState, id: NationId) -> Option<f64> {
    if !w.rules.ideology_blocs { return None; }
    let g = government::state(w, id)?;
    if !g.pillars.iter().any(|(p, _)| *p == Pillar::Army) { return None; }
    let known = g.army_authority.as_ref()?;
    (known.source.nation == id && valid_assessment(&known.source) && unit(known.current_leverage))
        .then_some(known.current_leverage)
}

/// Only a real Army-led seizure calls this, not an office-description change.
/// Unknown historical states stay unknown; no source record is synthesized.
pub(crate) fn army_seizure(w: &mut WorldState, id: NationId) {
    if current_leverage(w, id).is_none() { return; }
    let date = model_date(w);
    let g = w.governments.states.iter_mut().find(|g| g.nation == id).unwrap();
    let known = g.army_authority.as_mut().unwrap();
    known.current_leverage = 1.0;
    known.updated_on = date;
}

/// Use the existing lawful-transfer consolidation amount once, on the same
/// normalized scale as the old (authoritarianism-.20)/.40 proxy. This is a
/// game transition assumption, not a V-Dem estimate of institutional change.
pub(crate) fn consolidate_transfer(w: &mut WorldState, id: NationId, consolidation: f64) {
    let Some(current) = current_leverage(w, id) else { return; };
    if !consolidation.is_finite() || consolidation <= 0.0 { return; }
    let next = (current - consolidation / 0.40).max(0.0);
    if next == current { return; }
    let date = model_date(w);
    let g = w.governments.states.iter_mut().find(|g| g.nation == id).unwrap();
    let known = g.army_authority.as_mut().unwrap();
    known.current_leverage = next;
    known.updated_on = date;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{init::world_1990, GameRules};

    fn rules() -> GameRules {
        GameRules { ideology_blocs: true, ideology_takeover: true, ..GameRules::default() }
    }

    #[test]
    fn authority_catalog_is_lagged_weighted_and_rejects_invented_coverage() {
        let original: serde_json::Value = serde_json::from_str(EMBEDDED_AUTHORITY).unwrap();
        assert_eq!(parse_catalog(EMBEDDED_AUTHORITY).unwrap().len(), 130);
        for case in 0..9 {
            let mut raw = original.clone();
            let row = &mut raw["rows"][0];
            match case {
                0 => row["source_year"] = 1990.into(),
                1 => row["source_key"] = "".into(),
                2 => row["hos_weight"] = 2.0.into(),
                3 => row["hos_removal"] = (-0.1).into(),
                4 => row["assessment"] = 2.0.into(),
                5 => { row["hos_weight"] = 1.0.into(); row["hog_weight"] = 0.0.into(); row["hos_removal"] = serde_json::Value::Null; }
                6 => row["nation"] = "Georgia".into(),
                7 => row["unexpected_runtime_modifier"] = 0.9.into(),
                _ => { let duplicate = row.clone(); raw["rows"].as_array_mut().unwrap().push(duplicate); }
            }
            assert!(parse_catalog(&raw.to_string()).is_err(), "invalid authority source case {case}");
        }
        // A nonexistent second executive is neither fabricated nor given weight.
        let mut raw = original;
        let row = &mut raw["rows"][0];
        row["hos_weight"] = 1.0.into(); row["hog_weight"] = 0.0.into();
        row["hos_removal"] = 0.25.into(); row["hog_removal"] = serde_json::Value::Null;
        row["assessment"] = 0.25.into();
        assert!(parse_catalog(&raw.to_string()).is_ok());
    }

    #[test]
    fn authority_seeds_only_fresh_existing_armies_and_never_backfills_old_saves() {
        let mut w = world_1990(rules());
        for g in &w.governments.states {
            let source = sources().iter().find(|row| row.nation == g.nation);
            let eligible = source.is_some() && g.pillars.iter().any(|(p, _)| *p == Pillar::Army);
            assert_eq!(g.army_authority.is_some(), eligible, "{:?}", g.nation);
            if let Some(known) = &g.army_authority {
                assert_eq!(known.source, *source.unwrap());
                assert_eq!(known.current_leverage, known.source.assessment);
                assert_eq!(known.updated_on, (1990, 1, 1));
            }
        }
        let saved = crate::save(&w); let rng = w.rng.clone();
        w = crate::load(&saved).unwrap();
        government::ensure_all(&mut w);
        assert_eq!(crate::save(&w), saved); assert_eq!(w.rng, rng);
        let mut old: serde_json::Value = serde_json::from_str(&saved).unwrap();
        for g in old["governments"]["states"].as_array_mut().unwrap() {
            g.as_object_mut().unwrap().remove("army_authority");
        }
        let mut resumed = crate::load(&old.to_string()).unwrap();
        government::ensure_all(&mut resumed);
        assert!(resumed.governments.states.iter().all(|g| g.army_authority.is_none()));
        let mut late = crate::data::load_world(crate::data::EMBEDDED_NATIONS,
            &crate::data::EMBEDDED_RELATIONS, rules()).unwrap();
        late.year = 1995;
        government::ensure_all(&mut late);
        prepare_opening(&mut late);
        assert!(late.governments.states.iter().all(|g| g.army_authority.is_none()));
        let mut off = world_1990(GameRules::default());
        let before = crate::save(&off);
        prepare_opening(&mut off);
        army_seizure(&mut off, NationId::Pakistan);
        consolidate_transfer(&mut off, NationId::Pakistan, 0.05);
        assert_eq!(crate::save(&off), before);
        assert!(!before.contains("army_authority"));
    }

    #[test]
    fn authority_lifecycle_keeps_the_source_and_uses_the_model_clock() {
        let id = NationId::Pakistan;
        let base = world_1990(rules());
        let source = government::state(&base, id).unwrap().army_authority.as_ref().unwrap().source.clone();
        let mut receipts = Vec::new();
        for (daily, day, expected) in [(false, 1, 1), (false, 31, 1), (true, 12, 12)] {
            let mut w = base.clone();
            w.rules.daily_simulation = daily; (w.year, w.month, w.day) = (1992, 3, day);
            let rng = w.rng.clone();
            army_seizure(&mut w, id);
            assert_eq!(current_leverage(&w, id), Some(1.0));
            consolidate_transfer(&mut w, id, 0.05);
            assert_eq!(current_leverage(&w, id), Some(0.875));
            let known = government::state(&w, id).unwrap().army_authority.clone().unwrap();
            assert_eq!(known.source, source);
            assert_eq!(known.updated_on, (1992, 3, expected));
            assert_eq!(w.rng, rng);
            let saved = crate::save(&w);
            assert_eq!(crate::save(&crate::load(&saved).unwrap()), saved);
            receipts.push(known);
        }
        assert_eq!(receipts[0], receipts[1], "legacy day wrapper cannot change a monthly observation");
        assert_ne!(receipts[1].updated_on, receipts[2].updated_on);
    }
}
