//! Explicitly sourced missing conventional military institutions. Personnel
//! totals alone cannot identify an Army: some include police or paramilitaries.
//! These presence-only additions are visible solely under the political lens.
use crate::government::{self, Pillar, PillarSpec, Polity};
use crate::world::NationId;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::sync::OnceLock;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum InstitutionKind { Army }

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArmyInstitution {
    pub nation: NationId,
    pub kind: InstitutionKind,
    pub name: String,
    pub sources: Vec<String>,
    pub note: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Roster { version: u32, as_of: String, rows: Vec<ArmyInstitution> }

pub const EMBEDDED_INSTITUTIONS: &str = include_str!("../data/army_institutions_1990.json");
/// The complete opening-roster audit, including explicit no-inference cases.
/// It documents source scope; it does not assign military behaviour.
pub const EMBEDDED_COVERAGE: &str = include_str!("../data/army_institution_coverage_1990.json");

/// Validate this narrowly scoped overlay, including its immutable base table.
/// A positive personnel entry is a consistency check, never discovery evidence.
pub fn parse_institutions(json: &str) -> Result<Vec<ArmyInstitution>, String> {
    let roster: Roster = serde_json::from_str(json).map_err(|e| e.to_string())?;
    if roster.version != 1 || roster.as_of != "1990-01-01" {
        return Err("expected version 1, as_of 1990-01-01".into());
    }
    let mut seen = BTreeSet::new();
    for row in &roster.rows {
        if !seen.insert(row.nation) { return Err(format!("duplicate institution: {:?}", row.nation)); }
        if row.name.trim().is_empty() || row.note.trim().is_empty()
            || row.sources.is_empty() || row.sources.iter().any(|s| !s.starts_with("https://") || s.len() <= 8)
        { return Err(format!("institution needs name, note and HTTPS sources: {:?}", row.nation)); }
        let pol = government::polity(row.nation).ok_or("institution has no base polity")?;
        if pol.pillars.iter().any(|p| p.pillar == Pillar::Army) {
            return Err(format!("Army already present in base polity: {:?}", row.nation));
        }
        if !crate::data::army_personnel_1990(row.nation).is_some_and(|n| n.is_finite() && n > 0.0) {
            return Err(format!("institution contradicts absent/zero personnel: {:?}", row.nation));
        }
    }
    Ok(roster.rows)
}

/// Public source records support inspection without claiming an estimated
/// headcount, historical loyalty or a predetermined military intervention.
pub fn opening_institutions() -> &'static [ArmyInstitution] {
    static ROWS: OnceLock<Vec<ArmyInstitution>> = OnceLock::new();
    ROWS.get_or_init(|| parse_institutions(EMBEDDED_INSTITUTIONS)
        .unwrap_or_else(|e| panic!("army_institutions_1990.json: {e}")))
}

pub fn institution(id: NationId) -> Option<&'static ArmyInstitution> {
    opening_institutions().iter().find(|r| r.nation == id)
}

/// Cache the verified extended static slices once; every other field is the original
/// transcribed table. No party id, seat share, election date or ruler is copied
/// into a competing historical record.
pub(crate) fn polity_overlay(id: NationId) -> Option<&'static Polity> {
    static TABLE: OnceLock<Vec<Polity>> = OnceLock::new();
    TABLE.get_or_init(|| opening_institutions().iter().map(|row| {
        let base = government::polity(row.nation).unwrap();
        let mut pillars: Vec<PillarSpec> = base.pillars.iter()
            .map(|p| PillarSpec { pillar: p.pillar, name: p.name }).collect();
        pillars.push(PillarSpec { pillar: Pillar::Army, name: row.name.as_str() });
        Polity { nation: base.nation, system: base.system, term_months: base.term_months,
            next: base.next, parties: base.parties, ruling: base.ruling,
            pillars: Box::leak(pillars.into_boxed_slice()) }
    }).collect()).iter().find(|p| p.nation == id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{init::world_1990, GameRules};

    const COVERED: [NationId; 57] = [
        NationId::Peru, NationId::Guatemala, NationId::Honduras, NationId::Chile,
        NationId::ElSalvador, NationId::Nicaragua, NationId::SriLanka, NationId::Colombia,
        NationId::Guyana, NationId::Senegal, NationId::Zimbabwe, NationId::Mexico,
        NationId::India, NationId::Malaysia, NationId::Bulgaria, NationId::Poland,
        NationId::Bolivia, NationId::DominicanRepublic, NationId::Ecuador, NationId::Singapore,
        NationId::Argentina, NationId::Australia, NationId::Austria, NationId::Belgium,
        NationId::Belize, NationId::Botswana, NationId::Brazil, NationId::Canada,
        NationId::CapeVerde, NationId::Cyprus, NationId::Czechoslovakia, NationId::Denmark,
        NationId::Finland, NationId::France, NationId::Germany, NationId::Greece,
        NationId::Hungary, NationId::Ireland, NationId::Israel, NationId::Italy,
        NationId::Jamaica, NationId::Japan, NationId::Luxembourg, NationId::Malta,
        NationId::Netherlands, NationId::NewZealand, NationId::Norway, NationId::PapuaNewGuinea,
        NationId::Portugal, NationId::Spain, NationId::Sweden, NationId::Switzerland,
        NationId::TrinidadTobago, NationId::UK, NationId::Uruguay, NationId::USA,
        NationId::Venezuela,
    ];
    const EXCLUDED: [NationId; 9] = [
        NationId::Bahamas, NationId::CostaRica, NationId::Iceland, NationId::Maldives,
        NationId::Mauritius, NationId::Panama, NationId::Samoa, NationId::SolomonIslands,
        NationId::Vanuatu,
    ];

    #[test]
    fn army_presence_audit_covers_every_opening_polity_with_explicit_exceptions() {
        let expected: BTreeSet<_> = COVERED.into_iter().collect();
        let actual: BTreeSet<_> = opening_institutions().iter().map(|r| r.nation).collect();
        assert_eq!(actual, expected, "all independently sourced omissions are represented");
        let audit: serde_json::Value = serde_json::from_str(EMBEDDED_COVERAGE).unwrap();
        assert_eq!(audit["version"], 1);
        assert_eq!(audit["as_of"], "1990-01-01");
        let exceptions = audit["excluded"].as_array().unwrap();
        let excluded: BTreeSet<NationId> = exceptions.iter()
            .map(|r| serde_json::from_value(r["nation"].clone()).unwrap()).collect();
        assert_eq!(exceptions.len(), EXCLUDED.len());
        assert_eq!(excluded, EXCLUDED.into_iter().collect());
        let newly_audited = audit["new_rows"].as_array().unwrap();
        assert_eq!(newly_audited.len(), 37);
        let new_ids: BTreeSet<NationId> = newly_audited.iter()
            .map(|r| serde_json::from_value(r["nation"].clone()).unwrap()).collect();
        assert_eq!(new_ids, COVERED[20..].iter().copied().collect());
        for row in exceptions.iter().chain(newly_audited) {
            assert!(!row["country_section"].as_str().unwrap().is_empty());
            assert!(row["source_anchor"].as_str().unwrap().starts_with("id"));
        }
        let mut counts = [0; 3];
        for src in crate::data::EMBEDDED_NATIONS {
            let record: crate::data::NationRecord = serde_json::from_str(src.json).unwrap();
            let base = government::polity(record.id).unwrap();
            let present = base.pillars.iter().any(|p| p.pillar == Pillar::Army);
            let overlay = actual.contains(&record.id);
            let exception = excluded.contains(&record.id);
            assert_eq!(present as usize + overlay as usize + exception as usize, 1,
                "opening country needs exactly one reviewed disposition: {:?}", record.id);
            counts[0] += present as usize;
            counts[1] += overlay as usize;
            counts[2] += exception as usize;
        }
        assert_eq!(counts, [71, 57, 9]);
    }

    #[test]
    fn army_presence_rejects_invalid_sources_kinds_duplicates_and_no_force_records() {
        let original: serde_json::Value = serde_json::from_str(EMBEDDED_INSTITUTIONS).unwrap();
        assert_eq!(parse_institutions(EMBEDDED_INSTITUTIONS).unwrap().len(), COVERED.len());
        for case in 0..8 {
            let mut raw = original.clone();
            match case {
                0 => { let row = raw["rows"][0].clone(); raw["rows"].as_array_mut().unwrap().push(row); }
                1 => raw["rows"][0]["kind"] = serde_json::json!("Police"),
                2 => raw["rows"][0]["sources"] = serde_json::json!([]),
                3 => raw["rows"][0]["sources"] = serde_json::json!(["http://unsourced"]),
                4 => raw["rows"][0]["nation"] = serde_json::json!("CostaRica"),
                5 => raw["rows"][0]["nation"] = serde_json::json!("Pakistan"),
                6 => raw["rows"][0]["nation"] = serde_json::json!("UnknownCountry"),
                _ => raw["rows"][0]["loyalty"] = serde_json::json!(0.1),
            }
            assert!(parse_institutions(&raw.to_string()).is_err(), "invalid case {case}");
        }
    }

    #[test]
    fn army_presence_overlay_preserves_parties_and_is_not_inferred_from_personnel() {
        let mut rules = GameRules::default();
        rules.ideology_blocs = true;
        let on = world_1990(rules);
        let off = world_1990(GameRules::default());
        for row in opening_institutions() {
            let base = government::polity(row.nation).unwrap();
            let live = government::polity_in(&on, row.nation).unwrap();
            assert!(std::ptr::eq(base.parties, live.parties));
            assert_eq!((base.system, base.term_months, base.next, base.ruling),
                (live.system, live.term_months, live.next, live.ruling));
            assert!(std::ptr::eq(base, government::polity_in(&off, row.nation).unwrap()));
            assert_eq!(live.pillars.iter().filter(|p| p.pillar == Pillar::Army).count(), 1);
            assert_eq!(live.pillars.last().unwrap().name, row.name);
            assert_eq!(government::state(&on, row.nation).unwrap().loyalty(Pillar::Army), 0.65);
            assert!(!government::state(&off, row.nation).unwrap().pillars.iter().any(|(p, _)| *p == Pillar::Army));
        }
        // Positive force totals can describe police/paramilitaries. Countries
        // with absent/unknown Army evidence stay unchanged, as do existing armies.
        for id in EXCLUDED.into_iter().chain([NationId::Pakistan]) {
            assert!(institution(id).is_none());
            assert!(std::ptr::eq(government::polity(id).unwrap(), government::polity_in(&on, id).unwrap()));
        }
    }

    #[test]
    fn army_presence_old_saves_preserve_recorded_institutions_and_live_loyalty() {
        let mut off = world_1990(GameRules::default());
        let original = crate::save(&off);
        off = crate::load(&original).unwrap();
        government::ensure_all(&mut off);
        assert_eq!(crate::save(&off), original, "default/off saves are byte stable");

        let mut rules = GameRules::default(); rules.ideology_blocs = true;
        let mut w = world_1990(rules);
        // Older saves recorded no Army for these countries. Empty and other-
        // pillar vectors are both authoritative, not missing histories.
        for row in opening_institutions() {
            let g = w.governments.states.iter_mut().find(|g| g.nation == row.nation).unwrap();
            g.pillars.retain(|(p, _)| *p != Pillar::Army);
            g.army_authority = None;
        }
        w.governments.states.iter_mut().find(|g| g.nation == NationId::UK).unwrap()
            .pillars.push((Pillar::Security, 0.37));
        let before = serde_json::to_value(&w.governments).unwrap();
        let rng = serde_json::to_value(&w.rng).unwrap();
        let saved = crate::save(&w);
        w = crate::load(&saved).unwrap();
        government::ensure_all(&mut w);
        government::ensure_all(&mut w);
        assert_eq!(serde_json::to_value(&w.governments).unwrap(), before,
            "existing saved governments must not gain an invented loyalty stock");
        assert_eq!(serde_json::to_value(&w.rng).unwrap(), rng);
        assert_eq!(crate::save(&w), saved);
        // A genuinely absent government may still be constructed lazily.
        w.governments.states.retain(|g| g.nation != NationId::USA);
        government::ensure(&mut w, NationId::USA);
        assert_eq!(government::state(&w, NationId::USA).unwrap().loyalty(Pillar::Army), 0.65);
        let usa = w.governments.states.iter_mut().find(|g| g.nation == NationId::USA).unwrap();
        usa.pillars.iter_mut().find(|(p, _)| *p == Pillar::Army).unwrap().1 = 0.72;
        government::ensure(&mut w, NationId::USA);
        assert_eq!(government::state(&w, NationId::USA).unwrap().loyalty(Pillar::Army), 0.72);
    }
}
