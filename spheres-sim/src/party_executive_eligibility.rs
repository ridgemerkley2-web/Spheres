//! Permissions for NEW named executive selections, separate from sourced party
//! offices and saved identity hashes. A party title alone grants no authority.
use super::{future, party_in, Holder, NationId, Party, Roster, Term};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{collections::BTreeSet, sync::OnceLock};

pub const VERSION: u32 = 1;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OfficeRole {
    ParliamentaryGovernmentContender,
    PresidentialContender,
    NationalExecutiveContender,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Basis {
    ExistingParliamentaryGameplay,
    ResearchedExecutiveRole,
    AuthoredFictionalExecutiveRole,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Grant {
    pub nation: NationId,
    pub party: String,
    pub person: String,
    pub term: String,
    pub component: Option<String>,
    /// Exact sourced party title for historical grants; no substring inference.
    pub source_role: String,
    pub executive_role: OfficeRole,
    pub basis: Basis,
    pub sources: Vec<String>,
    pub note: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Institution {
    pub nation: NationId,
    pub fact: String,
    pub sources: Vec<String>,
    pub gameplay_assumption: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Policy {
    pub version: u32,
    pub reviewed_at: String,
    pub historical_default: String,
    pub institutions: Vec<Institution>,
    pub historical_grants: Vec<Grant>,
    /// Explicitly authored invented contenders; empty until individual review.
    /// A grant changes permission, never the underlying fictional identity.
    pub future_office_grants: Vec<Grant>,
}

fn presidential_review_required(n: NationId) -> bool {
    matches!(n, NationId::USA | NationId::France | NationId::Brazil)
}
fn reviewed_office_role(n: NationId) -> Option<OfficeRole> {
    match n {
        NationId::USA | NationId::France | NationId::Brazil => Some(OfficeRole::PresidentialContender),
        NationId::Germany | NationId::Italy | NationId::India | NationId::Canada | NationId::Australia => Some(OfficeRole::ParliamentaryGovernmentContender),
        NationId::China | NationId::SouthAfrica => Some(OfficeRole::NationalExecutiveContender),
        _ => None,
    }
}

impl Policy {
    fn historical_grant(&self, p: &Party, t: &Term) -> Option<&Grant> {
        self.historical_grants.iter().find(|g|
            g.nation == p.nation && g.party == p.party && g.person == t.person
                && g.term == t.id && g.component == t.component && g.source_role == t.role)
    }
    fn future_grant(&self, c: &future::FutureCandidate) -> Option<&Grant> {
        self.future_office_grants.iter().find(|g|
            g.nation == c.nation && g.party == c.party && g.person == c.person.id
                && g.term == c.term_id && g.component == c.component)
    }
    pub(super) fn allows_historical(&self, p: &Party, t: &Term) -> bool {
        self.historical_grant(p, t).is_some()
    }
    pub(super) fn allows_future(&self, c: &future::FutureCandidate) -> bool {
        self.future_grant(c).is_some() || reviewed_office_role(c.nation).is_none()
    }
    pub(super) fn allows_holder(&self, p: &Party, h: &Holder) -> bool {
        if let Some(c) = future::candidate(&h.person) {
            return c.nation == p.nation && c.party == p.party && c.term_id == h.term
                && self.allows_future(c);
        }
        p.terms.iter().find(|t| t.id == h.term && t.person == h.person)
            .is_some_and(|t| self.allows_historical(p, t))
    }
    fn info(&self, n: NationId, grant: Option<&Grant>, legacy_future: bool) -> Value {
        let authorized = grant.is_some() || legacy_future;
        json!({"policy_version":self.version,"authorized":authorized,
            "role":grant.map(|g|serde_json::to_value(&g.executive_role).unwrap())
                .unwrap_or_else(||json!(if legacy_future {"legacy_gameplay_contender"} else {"party_only"})),
            "basis":grant.map(|g|serde_json::to_value(&g.basis).unwrap())
                .unwrap_or_else(||json!(if legacy_future {"existing_fictional_gameplay"} else {"executive_role_unreviewed"})),
            "sources":grant.map(|g|g.sources.as_slice()).unwrap_or_default(),
            "requires_presidential_role_review":presidential_review_required(n) && grant.is_none(),
            "requires_executive_role_review":!authorized,
            "note":grant.map(|g|g.note.as_str()).unwrap_or(if legacy_future {
                "Existing fictional gameplay selection is retained; this is not a researched claim about national nomination rules."
            } else {
                "This record identifies a party role only. A separately reviewed executive role is required before a new national-office selection."
            }),
            "condition":"Role permission only: an actual campaign succession, eligible identity, and unambiguous selection are still required. Existing saved office holders are preserved."})
    }
    pub(super) fn historical_info(&self, p: &Party, t: &Term) -> Value {
        self.info(p.nation, self.historical_grant(p, t), false)
    }
    pub(super) fn future_info(&self, c: &future::FutureCandidate) -> Value {
        self.info(c.nation, self.future_grant(c), reviewed_office_role(c.nation).is_none())
    }
}

pub fn validate_policy(p: &Policy, r: &Roster) -> Result<(), String> {
    if p.version != VERSION || p.reviewed_at != future::HISTORICAL_THROUGH
        || p.historical_default != "party_only"
    { return Err("Unsupported executive eligibility policy or reference cutoff.".into()); }
    let sourced = |sources: &[String]| !sources.is_empty()
        && sources.iter().all(|s| s.starts_with("https://") && s.len() > 8);
    let mut nations = BTreeSet::new();
    for i in &p.institutions {
        if !nations.insert(i.nation) || i.fact.trim().is_empty()
            || i.gameplay_assumption.trim().is_empty() || !sourced(&i.sources)
        { return Err("Invalid or unsourced executive institution policy.".into()); }
    }
    for n in [NationId::USA, NationId::France, NationId::UK, NationId::Japan,
        NationId::Germany, NationId::Italy, NationId::India, NationId::China,
        NationId::Brazil, NationId::SouthAfrica, NationId::Canada, NationId::Australia] {
        if !nations.contains(&n) { return Err("Missing reviewed executive institution distinction.".into()); }
    }
    let mut keys = BTreeSet::new();
    for g in &p.historical_grants {
        let party = party_in(r, g.nation, &g.party).ok_or("Unknown executive-policy party.")?;
        let term = party.terms.iter().find(|t|t.id == g.term).ok_or("Unknown executive-policy term.")?;
        if !keys.insert((g.nation, &g.party, &g.term)) || g.person != term.person
            || g.component != term.component || g.source_role != term.role
            || !sourced(&g.sources) || g.note.trim().is_empty()
            || g.basis == Basis::AuthoredFictionalExecutiveRole
            || (g.basis == Basis::ExistingParliamentaryGameplay
                && (!matches!(g.nation, NationId::UK | NationId::Japan)
                    || g.executive_role != OfficeRole::ParliamentaryGovernmentContender))
            || reviewed_office_role(g.nation).is_some_and(|role|
                g.basis != Basis::ResearchedExecutiveRole || g.executive_role != role)
        { return Err("Invalid, ambiguous or unsourced historical executive grant.".into()); }
    }
    keys.clear();
    for g in &p.future_office_grants {
        let c = future::candidate(&g.person).ok_or("Unknown future executive-policy identity.")?;
        if !keys.insert((g.nation, &g.party, &g.term)) || g.nation != c.nation
            || g.party != c.party || g.term != c.term_id || g.component != c.component
            || g.source_role != "fictional party successor"
            || g.basis != Basis::AuthoredFictionalExecutiveRole
            || !sourced(&g.sources) || g.note.trim().is_empty()
            || reviewed_office_role(g.nation).is_some_and(|role|g.executive_role != role)
        { return Err("Invalid or unreviewed fictional executive grant.".into()); }
    }
    Ok(())
}

pub fn policy() -> Result<&'static Policy, &'static str> {
    static POLICY: OnceLock<Result<Policy, String>> = OnceLock::new();
    POLICY.get_or_init(|| {
        let p: Policy = serde_json::from_str(include_str!("../data/party_executive_eligibility.json"))
            .map_err(|e|format!("Invalid executive eligibility data: {e}"))?;
        validate_policy(&p, super::roster().map_err(str::to_owned)?)?;
        Ok(p)
    }).as_ref().map_err(String::as_str)
}

pub(super) fn historical_info(p: &Party, t: &Term) -> Value {
    policy().map(|policy|policy.historical_info(p,t)).unwrap_or_else(|e|
        json!({"authorized":false,"role":"party_only","error":e}))
}
pub fn future_info(c: &future::FutureCandidate) -> Value {
    policy().map(|policy|policy.future_info(c)).unwrap_or_else(|e|
        json!({"authorized":false,"role":"party_only","error":e}))
}
pub(super) fn holder_info(p: &Party, h: &Holder) -> Value {
    if let Some(c) = future::candidate(&h.person) { return future_info(c); }
    p.terms.iter().find(|t| t.id == h.term && t.person == h.person)
        .map(|t|historical_info(p,t)).unwrap_or_else(||json!({"authorized":false,"role":"party_only"}))
}
pub fn view(n: NationId) -> Value {
    match policy() {
        Ok(p) => json!({"version":p.version,"historical_default":p.historical_default,
            "existing_bindings_preserved":true,"applies_to":"new_executive_selections_only",
            "presidential_role_review_required":presidential_review_required(n),
            "future_executive_role_review_required":reviewed_office_role(n).is_some(),
            "institution":p.institutions.iter().find(|i| i.nation == n),
            "note":"Party leadership and national office are separate roles. Calendar references and party chairs never schedule a presidency."}),
        Err(e) => json!({"error":e,"historical_default":"party_only"}),
    }
}

#[cfg(test)]
pub(super) fn fixture_policy(r: &Roster) -> Policy {
    let mut p = policy().unwrap().clone();
    p.historical_grants = r.parties.iter().flat_map(|party|party.terms.iter().map(move |t|Grant {
        nation:party.nation,party:party.party.clone(),person:t.person.clone(),term:t.id.clone(),
        component:t.component.clone(),source_role:t.role.clone(),
        executive_role:OfficeRole::ParliamentaryGovernmentContender,
        basis:Basis::ExistingParliamentaryGameplay,
        sources:vec!["https://example.invalid/test-only-executive-role".into()],
        note:"Explicit test fixture permission, never a production identity or historical fact.".into(),
    })).collect();
    p
}
