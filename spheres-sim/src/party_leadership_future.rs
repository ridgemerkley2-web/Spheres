//! Authored future people, kept outside the sourced historical roster.
//! This catalog never advances time, consumes campaign RNG, or seats an office.
use super::{DateBound, Person};
use crate::{government as gov, world::NationId};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{collections::{BTreeMap, BTreeSet}, sync::OnceLock};

pub const HISTORICAL_THROUGH: &str = "2026-09-07";
pub const FROM: &str = "2026-09-08";
pub const UNTIL: &str = "2036-01-01";

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContinuationReview {
    pub nation: NationId,
    pub party: String,
    pub status: String,
    pub sources: Vec<String>,
    pub note: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ContinuationData {
    version: u32,
    reviewed_at: String,
    scope: String,
    parties: Vec<ContinuationReview>,
}
/// Editorial disclosure only: this never controls a surviving campaign party.
pub fn historical_continuation(nation: NationId, party: &str) -> Option<&'static ContinuationReview> {
    static DATA: OnceLock<ContinuationData> = OnceLock::new();
    let data = DATA.get_or_init(|| {
        let d: ContinuationData = serde_json::from_str(include_str!("../data/future_party_continuation.json"))
            .expect("bundled organization-continuation disclosures must parse");
        assert_eq!(d.version, 1);
        assert_eq!(d.reviewed_at, HISTORICAL_THROUGH);
        assert!(!d.scope.trim().is_empty());
        let mut seen = BTreeSet::new();
        for p in &d.parties {
            assert!(seen.insert((p.nation, p.party.as_str())));
            assert!(gov::party_spec(p.nation, &p.party).is_some());
            assert!(matches!(p.status.as_str(), "ceased" | "unverified"));
            assert!(!p.note.trim().is_empty());
            assert!(!p.sources.is_empty() && p.sources.iter().all(|s| s.starts_with("https://")));
        }
        d
    });
    data.parties.iter().find(|p| p.nation == nation && p.party == party)
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct NamePool {
    female: Vec<String>,
    male: Vec<String>,
    surnames: Vec<String>,
    family_first: bool,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Research {
    pub id: String,
    pub url: String,
    pub fact: String,
    pub applies_to: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Profile {
    pub id: String,
    pub label: String,
    pub basis: Vec<String>,
    pub career: String,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Data {
    version: u32,
    historical_reference_through: String,
    fictional_from: String,
    fictional_until_exclusive: String,
    candidates_per_component: usize,
    status: String,
    scope: String,
    research: Vec<Research>,
    assumptions: Vec<String>,
    profiles: Vec<Profile>,
    party_research: Vec<PartyResearch>,
    name_pools: BTreeMap<String, NamePool>,
    country_name_pools: BTreeMap<String, String>,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct PartyResearch {
    nation: NationId,
    party: String,
    research: Vec<Research>,
    biographies: Vec<String>,
}
fn data() -> &'static Data {
    static DATA: OnceLock<Data> = OnceLock::new();
    DATA.get_or_init(|| {
        let d: Data = serde_json::from_str(include_str!("../data/future_party_leadership.json"))
            .expect("bundled fictional leadership profiles must parse");
        assert_eq!(d.version, 1);
        assert_eq!(d.historical_reference_through, HISTORICAL_THROUGH);
        assert_eq!(d.fictional_from, FROM);
        assert_eq!(d.fictional_until_exclusive, UNTIL);
        assert_eq!(d.candidates_per_component, 4);
        for pool in d.name_pools.values() {
            assert!(pool.female.len() >= 4 && pool.male.len() >= 4 && !pool.surnames.is_empty());
        }
        for profile in &d.profiles {
            assert!(!profile.basis.is_empty() && profile.basis.iter().all(|id| d.research.iter().any(|r| r.id == *id)));
        }
        let mut scoped_parties = BTreeSet::new();
        for scoped in &d.party_research {
            assert!(scoped_parties.insert((scoped.nation, scoped.party.as_str())));
            assert!(gov::party_spec(scoped.nation, &scoped.party).is_some());
            assert_eq!(scoped.biographies.len(), d.candidates_per_component);
            assert!(!scoped.research.is_empty());
        }
        d
    })
}

#[derive(Clone, Debug, Serialize)]
pub struct FutureCandidate {
    pub person: Person,
    pub nation: NationId,
    pub party: String,
    pub component: Option<String>,
    pub term_id: String,
    pub origin: String,
    pub profile_id: String,
    pub profile_label: String,
    pub ideology: String,
    pub fictional_biography: String,
    pub eligible_from: String,
    pub eligible_until_exclusive: String,
    pub appearance_seed: String,
    pub presentation: String,
    pub name_pool: String,
    pub editorial_status: String,
    pub research_basis: Vec<Research>,
    pub assumptions: Vec<String>,
}

fn hash(s: &str) -> u64 {
    s.bytes().fold(0xcbf29ce484222325u64, |h, b| (h ^ u64::from(b)).wrapping_mul(0x100000001b3))
}
pub fn eligible_on(at: i32) -> bool {
    at >= super::day(FROM).unwrap() && at < super::day(UNTIL).unwrap()
}
fn profile_id(pol: &gov::Polity, component: Option<&str>) -> &'static str {
    if component.is_some() { "coalition_component" }
    else if pol.parties.len() == 1 && pol.pillars.iter().any(|p| p.pillar == gov::Pillar::Party) {
        "party_institution"
    } else if matches!(pol.system, gov::Electoral::FirstPastThePost | gov::Electoral::TwoRound) {
        "parliamentary_majoritarian"
    } else { "parliamentary_proportional" }
}
fn authored_name(pool: &NamePool, key: &str, female: bool, attempt: u64) -> String {
    let h = hash(&format!("{key}:{attempt}"));
    let first = if female { &pool.female } else { &pool.male };
    let given = &first[(h as usize) % first.len()];
    let mut surname = pool.surnames[((h >> 16) as usize) % pool.surnames.len()].clone();
    // Authored grammatical forms, not demographic rules or identity inference.
    if female && (surname.ends_with("ov") || surname.ends_with("in")) && key.contains(":russian:") {
        surname.push('a');
    }
    if female && key.contains(":polish:") && surname.ends_with("ski") {
        surname.pop(); surname.push('a');
    }
    if surname.is_empty() { given.clone() }
    else if pool.family_first { format!("{surname} {given}") }
    else { format!("{given} {surname}") }
}

/// Immutable IDs derive from the nation, exact party/component, and authored slot.
/// No appearance is inferred from a historical person or another party's avatar.
pub fn catalog() -> &'static [FutureCandidate] {
    static CATALOG: OnceLock<Vec<FutureCandidate>> = OnceLock::new();
    CATALOG.get_or_init(|| {
        let d = data();
        let historical = super::roster().expect("historical roster must validate before fictional catalog");
        let mut seen = BTreeSet::new();
        let mut result = vec![];
        for pol in gov::POLITIES.iter().chain(gov::D4_POLITIES) {
            for party in pol.parties {
                if !seen.insert((pol.nation, party.id)) { continue; }
                let researched = historical.parties.iter().find(|p| p.nation == pol.nation && p.party == party.id);
                let scoped = d.party_research.iter().find(|s| s.nation == pol.nation && s.party == party.id);
                let components = researched.filter(|p| !p.components.is_empty())
                    .map(|p| p.components.iter().map(|c| Some(c.id.as_str())).collect::<Vec<_>>())
                    .unwrap_or_else(|| vec![None]);
                let pool_id = &d.country_name_pools[pol.nation.code()];
                let pool = &d.name_pools[pool_id];
                for component in components {
                    let profile = d.profiles.iter().find(|p| p.id == profile_id(pol, component)).unwrap();
                    let mut names = BTreeSet::new();
                    for slot in 0..d.candidates_per_component {
                        let id = format!("fictional_v1_{}_{}_{}_{:02}", pol.nation.code().to_lowercase(), party.id, component.unwrap_or("main"), slot + 1);
                        let name_key = format!("{id}:{pool_id}:");
                        let female = slot % 2 == 0;
                        let mut name = authored_name(pool, &name_key, female, 0);
                        for attempt in 1..256 {
                            if !names.contains(&name) { break; }
                            name = authored_name(pool, &name_key, female, attempt);
                        }
                        // New historical research must not rename an already-saved fictional person.
                        // A coincidental shared name is not a shared identity: IDs and art stay separate.
                        assert!(names.insert(name.clone()), "fictional name collision within component");
                        let h = hash(&id);
                        result.push(FutureCandidate {
                            person: Person {
                                id: id.clone(), name, native: None,
                                born: Some(DateBound::Day(format!("{:04}-{:02}-{:02}", 1969 + h % 18, 1 + (h >> 8) % 12, 1 + (h >> 16) % 28))),
                                died: None, sources: vec![],
                            },
                            nation: pol.nation, party: party.id.into(), component: component.map(str::to_owned),
                            term_id: format!("{id}_candidacy"), origin: "fictional_successor".into(),
                            profile_id: profile.id.clone(), profile_label: profile.label.clone(),
                            ideology: party.family.label().into(), fictional_biography: scoped.map(|s| s.biographies[slot].clone()).unwrap_or_else(|| profile.career.clone()),
                            eligible_from: FROM.into(), eligible_until_exclusive: UNTIL.into(),
                            appearance_seed: format!("{h:016x}"), presentation: if female { "female" } else { "male" }.into(),
                            name_pool: pool_id.clone(), editorial_status: "authored_fiction_needs_country_review".into(),
                            research_basis: d.research.iter().filter(|r| profile.basis.contains(&r.id)).cloned()
                                .chain(scoped.into_iter().flat_map(|s| s.research.iter().cloned())).collect(),
                            assumptions: d.assumptions.clone(),
                        });
                    }
                }
            }
        }
        result.sort_by(|a, b| a.person.id.cmp(&b.person.id));
        result
    })
}
pub fn candidate(id: &str) -> Option<&'static FutureCandidate> {
    if !id.starts_with("fictional_v1_") { return None; }
    catalog().binary_search_by(|c| c.person.id.as_str().cmp(id)).ok().map(|i| &catalog()[i])
}
pub fn for_party(nation: NationId, party: &str) -> Vec<&'static FutureCandidate> {
    catalog().iter().filter(|c| c.nation == nation && c.party == party).collect()
}
pub fn binding_hash(c: &FutureCandidate) -> String {
    super::identity_hash(json!([c.person.id, c.person.name, c.person.born, c.nation, c.party,
        c.component, c.term_id, c.origin, c.profile_id, c.ideology, c.eligible_from,
        c.eligible_until_exclusive, c.appearance_seed, c.presentation, c.name_pool]))
}
pub fn holder(c: &FutureCandidate, at: i32) -> super::Holder {
    super::Holder { person: c.person.id.clone(), term: c.term_id.clone(), selected_day: at, identity_hash: binding_hash(c) }
}
/// Sequential organizations within one party are not simultaneous coalition seats.
/// The source cutoff chooses the default organization for a future vacancy;
/// an already-bound campaign holder is handled separately and remains in place.
pub fn component_eligible(nation: NationId, party: &str, component: Option<&str>) -> bool {
    let Ok(r) = super::roster() else { return false };
    let Some(p) = super::party_in(r, nation, party) else { return false };
    if p.kind != super::PartyKind::Party { return true; }
    match component {
        None => p.components.is_empty(),
        Some(id) => p.components.iter().find(|c| c.id == id).is_some_and(|c|
            super::life_contains(&c.founded, &c.dissolved, super::day(HISTORICAL_THROUGH).unwrap())),
    }
}
pub fn pick(nation: NationId, party: &str, component: Option<&str>, at: i32, book: &super::CampaignLeadership, office: bool) -> Option<super::Holder> {
    if !eligible_on(at) || !component_eligible(nation, party, component) { return None; }
    for_party(nation, party).into_iter()
        .filter(|c| c.component.as_deref() == component && !super::dead(book, &c.person.id))
        .find(|c| !office || (!book.office_exclusions.iter().any(|e| e.nation == nation && e.person == c.person.id)
            && super::executive_eligibility::policy().is_ok_and(|p|p.allows_future(c))))
        .map(|c| holder(c, at))
}
pub fn policy(at: i32) -> Value {
    let d = data();
    json!({"version":d.version,"historical_reference_through":HISTORICAL_THROUGH,
        "from":FROM,"until_exclusive":UNTIL,"eligible":eligible_on(at),
        "selection":"actual_succession_events_only","incumbents_retained":true,
        "status":d.status,"scope":d.scope,"candidate_count":catalog().len(),
        "profile_count":d.profiles.len(),"candidates_per_component":d.candidates_per_component,
        "research":d.research,"assumptions":d.assumptions})
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn future_profiles_cover_all_game_parties_without_claiming_future_history() {
        let d = data();
        assert_eq!((d.historical_reference_through.as_str(), d.fictional_from.as_str(), d.fictional_until_exclusive.as_str()), (HISTORICAL_THROUGH, FROM, UNTIL));
        let parties: BTreeSet<_> = gov::POLITIES.iter().chain(gov::D4_POLITIES)
            .flat_map(|p| p.parties.iter().map(move |s| (p.nation, s.id))).collect();
        let covered: BTreeSet<_> = catalog().iter().map(|c| (c.nation, c.party.as_str())).collect();
        assert_eq!(parties, covered);
        let mut ids = BTreeSet::new();
        for c in catalog() {
            assert!(ids.insert(&c.person.id));
            assert!(c.person.sources.is_empty());
            assert!(super::super::person_in(super::super::roster().unwrap(), &c.person.id).is_none());
            assert_eq!(c.origin, "fictional_successor");
            assert!(!c.research_basis.is_empty());
            assert!(!c.assumptions.is_empty());
            assert_eq!(candidate(&c.person.id).unwrap().appearance_seed, c.appearance_seed);
        }
        assert_eq!(parties.len(), 624);
        assert!(catalog().len() >= parties.len() * 4);
    }
    #[test]
    fn future_windows_are_exact_and_do_not_extend_historical_references() {
        assert!(!eligible_on(super::super::day("2026-09-07").unwrap()));
        assert!(eligible_on(super::super::day("2026-09-08").unwrap()));
        assert!(eligible_on(super::super::day("2035-12-31").unwrap()));
        assert!(!eligible_on(super::super::day("2036-01-01").unwrap()));
        assert_eq!(super::super::roster().unwrap().reference_through, HISTORICAL_THROUGH);
    }
    #[test]
    fn ldp_pilot_research_is_scoped_and_editorial_details_do_not_change_saved_identities() {
        let pilots = for_party(NationId::Japan, "jp_ldp");
        assert_eq!(pilots.len(), 4);
        assert_eq!(pilots.iter().map(|c| c.fictional_biography.as_str()).collect::<BTreeSet<_>>().len(), 4);
        for c in pilots {
            assert!(c.fictional_biography.contains("Diet member"));
            assert!(c.research_basis.iter().any(|r| r.id == "japan_ldp_presidential_rules"
                && r.url == "https://www.jimin.jp/english/the-president/rules/"));
            assert!(c.person.sources.is_empty());
            let mut revised = c.clone();
            revised.fictional_biography = "Another explicitly invented career sketch.".into();
            revised.research_basis.clear();
            assert_eq!(binding_hash(c), binding_hash(&revised), "editorial grounding must not change a saved identity");
        }
        assert!(catalog().iter().filter(|c| c.nation != NationId::Japan || c.party != "jp_ldp")
            .all(|c| !c.research_basis.iter().any(|r| r.id == "japan_ldp_presidential_rules")));
    }
    #[test]
    fn organization_continuation_disclosure_preserves_counterfactual_candidates_and_campaign() {
        use super::super::reference_view;
        let w = crate::init::world_1990(crate::world::GameRules {
            ideology_blocs: true, historical_party_leadership: true, ..Default::default()
        });
        let before = crate::save(&w);
        let at = super::super::day("2030-01-01").unwrap();
        let book = w.party_leadership.as_ref().unwrap();
        for (nation, party, status) in [(NationId::SouthAfrica,"za_np", "ceased"), (NationId::SouthAfrica,"za_dp", "unverified"),
            (NationId::Canada,"ca_pc","ceased"), (NationId::Canada,"ca_reform","ceased")] {
            let view = reference_view(&w, nation, "2030-01-01");
            let review = historical_continuation(nation, party).unwrap();
            assert_eq!(review.status, status);
            assert!(!review.sources.is_empty());
            assert!(pick(nation, party, None, at, book, false).is_some(),
                "editorial continuity warnings must not disable a surviving campaign party");
            let row = view["parties"].as_array().unwrap().iter().find(|p|p["party_id"]==party).unwrap();
            assert_eq!(row["future_preview"].as_array().unwrap().len(), 4);
            if status == "ceased" {
                assert!(row["future_candidates"].as_array().unwrap().is_empty(),
                    "a future reference must respect researched final registration boundaries");
            }
            for candidate in row["future_preview"].as_array().unwrap() {
                assert_eq!(candidate["historical_continuation_status"], status);
                assert_eq!(candidate["historical_continuation"]["note"], review.note);
                assert_eq!(candidate["eligible"], false);
            }
        }
        assert!(historical_continuation(NationId::SouthAfrica, "za_anc").is_none());
        assert_eq!(crate::save(&w), before);
    }
}
