//! Partial, sourced opening parliamentary authority, separate from elections
//! completed by the simulation. No vote percentages or democratic-control
//! bonuses are inferred here. The historical source can end, never regenerate.
use crate::{data, government, world::{NationId, WorldState}};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, sync::OnceLock};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OpeningMandate {
    pub source_key: String,
    pub ballot_on: String,
    pub governing_parties: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceMandate {
    pub key: String,
    pub nation: NationId,
    pub ballot_on: String,
    pub governing_parties: Vec<String>,
    pub opening_holder: String,
    pub opening_office: String,
    pub opening_party: String,
    pub authority: String,
    pub sources: Vec<String>,
    pub note: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct NotImported { nation: NationId, reason: String }
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Catalog {
    version: u32, as_of: String, coverage: String,
    reviewed_not_imported: Vec<NotImported>, rows: Vec<SourceMandate>,
}

pub const EMBEDDED_MANDATES: &str = include_str!("../data/opening_mandates_1990.json");

pub fn parse_catalog(json: &str) -> Result<Vec<SourceMandate>, String> {
    let catalog: Catalog = serde_json::from_str(json).map_err(|e| e.to_string())?;
    if catalog.version != 1 || catalog.as_of != "1990-01-01" || catalog.coverage.trim().is_empty() {
        return Err("opening mandate catalog needs version 1, 1990-01-01 and explicit coverage".into());
    }
    let leaders = data::parse_leaders(&data::EMBEDDED_LEADERS).map_err(|_| "invalid opening leaders")?;
    let mut nations = BTreeSet::new(); let mut keys = BTreeSet::new();
    for row in &catalog.rows {
        if !nations.insert(row.nation) || !keys.insert(&row.key) || row.key.trim().is_empty() {
            return Err("duplicate or missing opening mandate identity".into());
        }
        if !data::parse_date(&row.ballot_on).is_some_and(|date| date < (1990, 1, 1)) {
            return Err(format!("opening ballot must precede the campaign: {:?}", row.nation));
        }
        if row.authority != "assembly_elected_executive" || row.note.trim().is_empty()
            || row.sources.len() < 2 || row.sources.iter().any(|s| !s.starts_with("https://") || s.len() <= 8) {
            return Err("opening mandate needs attested parliamentary authority and sources".into());
        }
        let pol = government::polity(row.nation).ok_or("opening mandate needs a polity")?;
        let mut parties = BTreeSet::new();
        if row.governing_parties.is_empty() || row.governing_parties.iter().any(|party|
            !parties.insert(party) || !pol.parties.iter().any(|p| p.id == party))
            || !row.governing_parties.contains(&row.opening_party) {
            return Err("opening mandate needs distinct known governing parties".into());
        }
        let leader = leaders.iter().find(|r| r.nation == row.nation).ok_or("opening mandate needs an actual office")?;
        if leader.name.as_deref() != Some(row.opening_holder.as_str()) || leader.office != row.opening_office
            || leader.tie_now() != Some(data::Tie::Party(row.opening_party.clone())) {
            return Err("opening mandate disagrees with the sourced incumbent's office or party".into());
        }
    }
    for row in &catalog.reviewed_not_imported {
        if !nations.insert(row.nation) || row.reason.trim().is_empty() {
            return Err("duplicate or unexplained not-imported audit row".into());
        }
    }
    Ok(catalog.rows)
}

pub fn sources() -> &'static [SourceMandate] {
    static SOURCES: OnceLock<Vec<SourceMandate>> = OnceLock::new();
    SOURCES.get_or_init(|| parse_catalog(EMBEDDED_MANDATES)
        .unwrap_or_else(|e| panic!("opening_mandates_1990.json: {e}")))
}

/// Used only by fresh world_1990 construction, never ensure or save loading.
/// This does not set elected, age an office, seed a vote anchor, or grant an
/// unrestricted ballot or institutional consolidation.
pub(crate) fn prepare_opening(w: &mut WorldState) {
    if !w.rules.ideology_blocs || (w.year,w.month,w.day) != (1990,1,1) { return; }
    for row in sources() {
        if !government::is_electoral(w,row.nation) { continue; }
        let actual = crate::blocs::leader_row(w,row.nation).is_some_and(|office|
            office.name.as_deref() == Some(row.opening_holder.as_str()) && office.office == row.opening_office
                && office.tie_now() == Some(data::Tie::Party(row.opening_party.clone())));
        if !actual { continue; }
        let Some(g) = w.governments.states.iter_mut().find(|g| g.nation == row.nation) else { continue; };
        if g.elected || g.awaiting_first_election || !g.banned.is_empty()
            || g.leader() != Some(row.opening_party.as_str()) { continue; }
        g.opening_mandate = Some(OpeningMandate { source_key: row.key.clone(), ballot_on: row.ballot_on.clone(),
            governing_parties: row.governing_parties.clone() });
    }
}

/// Only the attested parties can supply this prior authority. A new coalition
/// partner gets no inherited votes; changed national support still changes the
/// magnitude read by government. Unknown/edited source markers fail closed.
pub(crate) fn attested_parties(w: &WorldState, id: NationId) -> Option<&'static [String]> {
    if !w.rules.ideology_blocs || !government::is_electoral(w,id) { return None; }
    let g = government::state(w,id)?;
    if g.elected || g.awaiting_first_election || !g.banned.is_empty() { return None; }
    let marker = g.opening_mandate.as_ref()?;
    let row = sources().iter().find(|r| r.nation == id && r.key == marker.source_key)?;
    if marker.ballot_on != row.ballot_on || marker.governing_parties != row.governing_parties { return None; }
    let leader = g.leader()?;
    if !row.governing_parties.iter().any(|p| p == leader) { return None; }
    let office = crate::blocs::leader_row(w,id)?;
    let current_office = office.emergent.as_ref().map_or(office.office.as_str(), |e| e.office.as_str());
    if current_office != row.opening_office || office.tie_now() != Some(data::Tie::Party(leader.to_string())) { return None; }
    Some(&row.governing_parties)
}

pub(crate) fn clear(w: &mut WorldState, id: NationId) {
    if !w.rules.ideology_blocs { return; }
    if let Some(g) = w.governments.states.iter_mut().find(|g| g.nation == id) { g.opening_mandate = None; }
}

/// Invalidated authority cannot return simply because a party, legality or
/// government shape later changes back. This never imports missing old data.
pub(crate) fn discard_incompatible(w: &mut WorldState, id: NationId) {
    if !w.rules.ideology_blocs || government::state(w,id).is_none_or(|g| g.opening_mandate.is_none()) { return; }
    if attested_parties(w,id).is_none() { clear(w,id); }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn opening_mandate_catalog_is_partial_dated_and_bound_to_the_actual_office() {
        let rows = parse_catalog(EMBEDDED_MANDATES).unwrap();
        assert_eq!(rows.len(),1);
        let row=&rows[0];
        assert_eq!(row.nation,NationId::Suriname);
        assert_eq!(row.ballot_on,"1987-11-25");
        assert_eq!(row.governing_parties,["sr_fdo"]);
        for (from,to) in [("1987-11-25","1990-05-27"),("Ramsewak Shankar","another holder"),
            ("\"opening_party\": \"sr_fdo\"","\"opening_party\": \"sr_ndp\""),
            ("\"governing_parties\": [\"sr_fdo\"]","\"governing_parties\": [\"unknown_party\"]")]
        {
            let changed=EMBEDDED_MANDATES.replace(from,to);
            assert_ne!(changed,EMBEDDED_MANDATES);
            assert!(parse_catalog(&changed).is_err(),"{from} -> {to}");
        }
    }
}
