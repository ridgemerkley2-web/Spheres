//! Sourced organization presence outside the formal parliamentary party table.
//! Existence is distinct from popularity: this schema deliberately has no vote
//! share or territorial-control field. The generic political model estimates
//! constituencies and lets campaigning and government performance change them.
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::sync::OnceLock;
use crate::government::Bloc;
use crate::world::NationId;
use super::{LoadError, Source};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OpeningMovement {
    pub nation: NationId,
    pub bloc: Bloc,
    pub organization: String,
    pub sources: Vec<String>,
    pub note: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Roster {
    version: u32,
    as_of: String,
    rows: Vec<OpeningMovement>,
}

pub const EMBEDDED_MOVEMENTS: Source<'static> = Source {
    file: "movements_1990.json",
    json: include_str!("../../data/movements_1990.json"),
};

pub fn parse_opening_movements(source: &Source<'_>) -> Result<Vec<OpeningMovement>, Vec<LoadError>> {
    let roster: Roster = serde_json::from_str(source.json)
        .map_err(|e| vec![LoadError::file_level(source.file, e.to_string())])?;
    let mut errors = vec![];
    if roster.version != 1 || roster.as_of != "1990-01-01" {
        errors.push(LoadError::file_level(source.file, "expected version 1, as_of 1990-01-01"));
    }
    let mut seen = BTreeSet::new();
    for row in &roster.rows {
        if !seen.insert((row.nation, row.bloc)) {
            errors.push(LoadError::nation_level(source.file, row.nation.code(), "duplicate organization-presence bloc"));
        }
        if row.organization.trim().is_empty() || row.note.trim().is_empty()
            || row.sources.is_empty() || row.sources.iter().any(|s| !s.starts_with("https://")) {
            errors.push(LoadError::nation_level(source.file, row.nation.code(), "organization presence needs a name, sourcing note and HTTPS sources"));
        }
    }
    if errors.is_empty() { Ok(roster.rows) } else { Err(errors) }
}

pub fn opening_movements_1990() -> &'static [OpeningMovement] {
    static ROWS: OnceLock<Vec<OpeningMovement>> = OnceLock::new();
    ROWS.get_or_init(|| parse_opening_movements(&EMBEDDED_MOVEMENTS)
        .unwrap_or_else(|e| panic!("{}", super::render_errors(&e))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opening_organizations_have_sources_without_invented_vote_shares() {
        let rows = parse_opening_movements(&EMBEDDED_MOVEMENTS).unwrap();
        assert!(rows.iter().any(|r| r.nation == NationId::Afghanistan && r.bloc == Bloc::Islamist));
        let mut raw: serde_json::Value = serde_json::from_str(EMBEDDED_MOVEMENTS.json).unwrap();
        raw["rows"][0]["support"] = serde_json::json!(0.5);
        let json = raw.to_string();
        assert!(parse_opening_movements(&Source { file: "invented-share", json: &json }).is_err());
    }

    #[test]
    fn opening_organizations_reject_duplicates_unknown_nations_and_unsourced_rows() {
        let original: serde_json::Value = serde_json::from_str(EMBEDDED_MOVEMENTS.json).unwrap();
        for case in 0..4 {
            let mut raw = original.clone();
            match case {
                0 => { let row = raw["rows"][0].clone(); raw["rows"].as_array_mut().unwrap().push(row); },
                1 => raw["rows"][0]["nation"] = serde_json::json!("UnknownCountry"),
                2 => raw["rows"][0]["sources"] = serde_json::json!([]),
                _ => raw["as_of"] = serde_json::json!("1995-01-01"),
            }
            let json = raw.to_string();
            assert!(parse_opening_movements(&Source { file: "invalid", json: &json }).is_err(), "case {case}");
        }
    }
}
