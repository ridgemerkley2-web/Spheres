//! Sourced historical party candidates, separate from campaign office outcomes.
//! Data never schedules an election winner or removes an incumbent by date.
use crate::{government as gov, world::*};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{collections::BTreeSet, sync::OnceLock};

#[path = "party_leadership_future.rs"]
pub mod future;
pub use future::{candidate as fictional_person, catalog as future_catalog};
#[path = "party_executive_eligibility.rs"]
pub mod executive_eligibility;
#[path = "party_leadership_seats.rs"]
pub mod leadership_seats;
#[cfg(test)]
#[path = "party_executive_eligibility_tests.rs"]
mod executive_eligibility_tests;
#[cfg(test)]
#[path = "party_leadership_seats_tests.rs"]
mod leadership_seats_tests;

pub const VERSION: u32 = 1;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum DateBound {
    Day(String),
    Month(String),
    Year(String),
    Unknown,
    Open,
}

impl DateBound {
    fn bounds(&self) -> Option<(i32, i32)> {
        let (y, m, d, last) = match self {
            Self::Day(s) => {
                let (y, m, d) = strict_date(s)?;
                (y, m, d, d)
            }
            Self::Month(s) => {
                if s.len() != 7
                    || s.as_bytes()[4] != b'-'
                    || !s
                        .bytes()
                        .enumerate()
                        .all(|(i, b)| i == 4 || b.is_ascii_digit())
                {
                    return None;
                }
                let (y, m) = s.split_once('-')?;
                let y = y.parse().ok()?;
                let m = m.parse().ok()?;
                if !(1..=9999).contains(&y) || !(1..=12).contains(&m) {
                    return None;
                }
                (y, m, 1, crate::world::days_in_month(y, m))
            }
            Self::Year(s) => {
                if s.len() != 4 || !s.bytes().all(|b| b.is_ascii_digit()) {
                    return None;
                }
                let y = s.parse().ok()?;
                if !(1..=9999).contains(&y) {
                    return None;
                }
                return Some((
                    crate::clock::date_day(y, 1, 1),
                    crate::clock::date_day(y, 12, 31),
                ));
            }
            _ => return None,
        };
        Some((
            crate::clock::date_day(y, m, d),
            crate::clock::date_day(y, m, last),
        ))
    }
    fn valid(&self, end: bool) -> bool {
        match self {
            Self::Unknown => true,
            Self::Open => end,
            _ => self.bounds().is_some(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Person {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub native: Option<String>,
    #[serde(default)]
    pub born: Option<DateBound>,
    #[serde(default)]
    pub died: Option<DateBound>,
    pub sources: Vec<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Component {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub sources: Vec<String>,
    #[serde(default)]
    pub founded: Option<DateBound>,
    #[serde(default)]
    pub dissolved: Option<DateBound>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PartyKind {
    Unknown,
    Party,
    Coalition,
    Collective,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Coverage {
    Gap,
    Partial,
    Verified,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TermKind {
    Leader,
    CoLeader,
    Acting,
    Candidate,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Term {
    pub id: String,
    pub person: String,
    #[serde(default)]
    pub component: Option<String>,
    pub role: String,
    pub kind: TermKind,
    pub from: DateBound,
    pub until: DateBound,
    pub affiliation_from: DateBound,
    pub affiliation_until: DateBound,
    pub sources: Vec<String>,
    #[serde(default)]
    pub note: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Gap {
    pub from: DateBound,
    pub until: DateBound,
    pub reason: String,
    #[serde(default)]
    pub sources: Vec<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Party {
    pub nation: NationId,
    pub party: String,
    pub kind: PartyKind,
    pub coverage: Coverage,
    #[serde(default)]
    pub sources: Vec<String>,
    #[serde(default)]
    pub identity_note: Option<String>,
    #[serde(default)]
    pub founded: Option<DateBound>,
    #[serde(default)]
    pub dissolved: Option<DateBound>,
    #[serde(default)]
    pub components: Vec<Component>,
    #[serde(default)]
    pub terms: Vec<Term>,
    #[serde(default)]
    pub gaps: Vec<Gap>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OfficeLink {
    pub nation: NationId,
    pub person: String,
    pub since: String,
    pub sources: Vec<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Roster {
    pub version: u32,
    pub reference_from: String,
    pub reference_through: String,
    pub people: Vec<Person>,
    pub parties: Vec<Party>,
    #[serde(default)]
    pub office_links: Vec<OfficeLink>,
}

fn sourced(s: &[String]) -> bool {
    !s.is_empty() && s.iter().all(|s| !s.trim().is_empty())
}
fn identity(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 160
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"_-.:".contains(&b))
}
fn strict_date(s: &str) -> Option<(i32, u32, u32)> {
    let (y, m, d) = crate::data::parse_date(s)?;
    if !(1..=9999).contains(&y) || d > crate::world::days_in_month(y, m) {
        return None;
    }
    Some((y, m, d))
}
fn day(s: &str) -> Option<i32> {
    let (y, m, d) = strict_date(s)?;
    Some(crate::clock::date_day(y, m, d))
}
fn window_valid(from: &DateBound, until: &DateBound) -> bool {
    from.valid(false)
        && until.valid(true)
        && match (from.bounds(), until.bounds()) {
            (Some((a, _)), Some((_, b))) => a < b,
            _ => true,
        }
}
fn life_valid(from: &Option<DateBound>, until: &Option<DateBound>) -> bool {
    from.as_ref().is_none_or(|d| d.valid(false))
        && until.as_ref().is_none_or(|d| d.valid(false))
        && match (
            from.as_ref().and_then(DateBound::bounds),
            until.as_ref().and_then(DateBound::bounds),
        ) {
            (Some((a, _)), Some((_, b))) => a <= b,
            _ => true,
        }
}

pub fn validate_roster(r: &Roster) -> Result<(), String> {
    let fail = |s: &str| Err(format!("Party leadership roster: {s}"));
    if r.version != VERSION
        || r.reference_through != future::HISTORICAL_THROUGH
        || day(&r.reference_from)
            .zip(day(&r.reference_through))
            .is_none_or(|(a, b)| a > b)
    {
        return fail("invalid version or reference period");
    }
    let mut people = BTreeSet::new();
    for p in &r.people {
        if !identity(&p.id)
            || !people.insert(p.id.as_str())
            || p.name.trim().is_empty()
            || !sourced(&p.sources)
            || !life_valid(&p.born, &p.died)
        {
            return fail("duplicate, unsourced or invalid person");
        }
    }
    let mut parties = BTreeSet::new();
    let mut terms = BTreeSet::new();
    for p in &r.parties {
        if !parties.insert((p.nation, p.party.as_str()))
            || gov::party_spec(p.nation, &p.party).is_none()
            || !life_valid(&p.founded, &p.dissolved)
        {
            return fail("invalid party identity");
        }
        if p.coverage == Coverage::Verified
            && (p.kind == PartyKind::Unknown
                || !sourced(&p.sources)
                || p.terms.is_empty()
                || !p.gaps.is_empty())
        {
            return fail("verified party lacks identity, terms or complete coverage");
        }
        let mut components = BTreeSet::new();
        for c in &p.components {
            if !identity(&c.id)
                || !components.insert(c.id.as_str())
                || c.name.trim().is_empty()
                || !life_valid(&c.founded, &c.dissolved)
            {
                return fail("invalid component identity");
            }
        }
        for t in &p.terms {
            if !identity(&t.id)
                || !terms.insert(t.id.as_str())
                || !people.contains(t.person.as_str())
                || t.role.trim().is_empty()
                || !sourced(&t.sources)
                || !window_valid(&t.from, &t.until)
                || !window_valid(&t.affiliation_from, &t.affiliation_until)
                || t.component
                    .as_ref()
                    .is_some_and(|c| !components.contains(c.as_str()))
                || (!p.components.is_empty() && t.component.is_none())
            {
                return fail("invalid, duplicate, unlinked or unsourced leadership term");
            }
        }
        for g in &p.gaps {
            if g.reason.trim().is_empty() || !window_valid(&g.from, &g.until) {
                return fail("invalid coverage gap");
            }
        }
    }
    let originals: crate::data::LeadersFile =
        serde_json::from_str(crate::data::embedded::EMBEDDED_LEADERS.json)
            .map_err(|e| e.to_string())?;
    let mut links = BTreeSet::new();
    for l in &r.office_links {
        if !people.contains(l.person.as_str())
            || !originals
                .rows
                .iter()
                .any(|o| o.nation == l.nation && o.since == l.since && o.name.is_some())
            || day(&l.since).is_none()
            || !sourced(&l.sources)
            || !links.insert((l.nation, l.since.as_str()))
        {
            return fail("invalid executive identity link");
        }
    }
    Ok(())
}

pub fn roster() -> Result<&'static Roster, &'static str> {
    static DATA: OnceLock<Result<Roster, String>> = OnceLock::new();
    DATA.get_or_init(|| {
        let r: Roster = serde_json::from_str(include_str!("../data/party_leaders.json"))
            .map_err(|e| e.to_string())?;
        validate_roster(&r)?;
        Ok(r)
    })
    .as_ref()
    .map_err(|e| e.as_str())
}
/// Explicit opt-in; reading historical references never calls this function.
pub fn enable_campaign(w: &mut WorldState) -> Result<(), String> {
    if !w.rules.ideology_blocs {
        return Err("Party leadership requires the political institutions rule.".into());
    }
    let r = roster().map_err(str::to_string)?;
    if w.party_leadership.is_some() {
        return validate_state(w);
    }
    w.rules.historical_party_leadership = true;
    ensure_with(w, r);
    Ok(())
}
pub fn person(id: &str) -> Option<&'static Person> {
    person_in(roster().ok()?, id).or_else(|| fictional_person(id).map(|c| &c.person))
}
/// Display metadata keeps invented future biographies separate from source facts.
pub fn person_view(id: &str) -> Value {
    let Ok(r) = roster() else { return Value::Null };
    display_person_in(r, id)
}
fn display_person_in(r: &Roster, id: &str) -> Value {
    if let Some(p) = person_in(r, id) { return json!(p); }
    let Some(c) = fictional_person(id) else { return Value::Null };
    let mut value = json!(c.person);
    let mut fiction = json!(c);
    fiction.as_object_mut().unwrap().remove("person");
    value["fiction"] = fiction;
    value
}
fn person_in<'a>(r: &'a Roster, id: &str) -> Option<&'a Person> {
    r.people.iter().find(|p| p.id == id)
}
fn party_in<'a>(r: &'a Roster, n: NationId, id: &str) -> Option<&'a Party> {
    r.parties.iter().find(|p| p.nation == n && p.party == id)
}
fn today(w: &WorldState) -> i32 {
    crate::clock::date_day(
        w.year,
        w.month,
        if crate::clock::is_daily(w) {
            w.day.max(1)
        } else {
            1
        },
    )
}
fn within(r: &Roster, from: &DateBound, until: &DateBound, at: i32) -> bool {
    let Some((_, start)) = from.bounds() else {
        return false;
    };
    let end = match until {
        DateBound::Open => day(&r.reference_through).map(|d| d.saturating_add(1)),
        _ => until.bounds().map(|(d, _)| d),
    };
    start <= at && end.is_some_and(|end| at < end)
}
fn life_contains(from: &Option<DateBound>, until: &Option<DateBound>, at: i32) -> bool {
    from.as_ref()
        .and_then(DateBound::bounds)
        .is_none_or(|(_, latest)| at >= latest)
        && until
            .as_ref()
            .and_then(DateBound::bounds)
            .is_none_or(|(earliest, _)| at < earliest)
}
fn historical_on(r: &Roster, p: &Party, t: &Term, at: i32) -> bool {
    let Some(person) = person_in(r, &t.person) else {
        return false;
    };
    let in_reference = day(&r.reference_from)
        .zip(day(&r.reference_through))
        .is_some_and(|(a, b)| a <= at && at <= b);
    in_reference
        && p.kind != PartyKind::Unknown
        && within(r, &t.from, &t.until, at)
        && life_contains(&person.born, &person.died, at)
        && life_contains(&p.founded, &p.dissolved, at)
        && t.component.as_ref().is_none_or(|id| {
            p.components
                .iter()
                .find(|c| c.id == *id)
                .is_some_and(|c| life_contains(&c.founded, &c.dissolved, at))
        })
}

fn date_label(at: i32) -> String {
    let (y, m, d) = crate::clock::date_from_day(at);
    format!("{y:04}-{m:02}-{d:02}")
}
fn possibly_historical_on(r: &Roster, p: &Party, t: &Term, at: i32) -> bool {
    if t.kind == TermKind::Candidate || historical_on(r, p, t, at) || p.kind == PartyKind::Unknown {
        return false;
    }
    let Some((first, last)) = day(&r.reference_from).zip(day(&r.reference_through)) else {
        return false;
    };
    if at < first || at > last {
        return false;
    }
    let start = t
        .from
        .bounds()
        .map(|(earliest, _)| earliest)
        .unwrap_or(first);
    let end = t
        .until
        .bounds()
        .map(|(_, latest)| latest)
        .unwrap_or(last + 1);
    let possible_life = |a: &Option<DateBound>, b: &Option<DateBound>| {
        a.as_ref()
            .and_then(DateBound::bounds)
            .is_none_or(|(earliest, _)| at >= earliest)
            && b.as_ref()
                .and_then(DateBound::bounds)
                .is_none_or(|(_, latest)| at < latest)
    };
    start <= at
        && at < end
        && person_in(r, &t.person).is_some_and(|person| possible_life(&person.born, &person.died))
        && possible_life(&p.founded, &p.dissolved)
        && t.component.as_ref().is_none_or(|id| {
            p.components
                .iter()
                .find(|c| c.id == *id)
                .is_some_and(|c| possible_life(&c.founded, &c.dissolved))
        })
}

fn eligible(r: &Roster, p: &Party, t: &Term, at: i32) -> bool {
    historical_on(r, p, t, at) && within(r, &t.affiliation_from, &t.affiliation_until, at)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Holder {
    pub person: String,
    pub term: String,
    pub selected_day: i32,
    pub identity_hash: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Assignment {
    pub nation: NationId,
    pub party: String,
    pub component: Option<String>,
    pub holders: Vec<Holder>,
    pub since_day: i32,
    pub reason: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Executive {
    pub nation: NationId,
    pub party: String,
    pub holder: Holder,
    pub since_day: i32,
    pub reason: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Death {
    pub person: String,
    pub day: i32,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OfficeExclusion {
    pub nation: NationId,
    pub person: String,
    pub day: i32,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OfficeIdentity {
    pub nation: NationId,
    pub person: String,
    pub since: String,
    pub identity_hash: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CampaignLeadership {
    pub version: u32,
    pub roster_version: u32,
    pub started_day: i32,
    pub assignments: Vec<Assignment>,
    pub office_identities: Vec<OfficeIdentity>,
    pub executives: Vec<Executive>,
    pub deaths: Vec<Death>,
    pub office_exclusions: Vec<OfficeExclusion>,
}

fn identity_hash(v: Value) -> String {
    let mut h = 0xcbf29ce484222325u64;
    for b in v.to_string().bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3)
    }
    format!("{h:016x}")
}
fn binding_hash(p: &Person, t: &Term) -> String {
    identity_hash(json!([
        p.id,
        p.name,
        p.native,
        p.born,
        p.died,
        t.id,
        t.person,
        t.component,
        t.role,
        t.kind,
        t.from,
        t.until,
        t.affiliation_from,
        t.affiliation_until
    ]))
}
fn office_hash(p: &Person, since: &str) -> String {
    identity_hash(json!([p.id, p.name, p.native, p.born, p.died, since]))
}
fn holder(r: &Roster, t: &Term, at: i32) -> Holder {
    Holder {
        person: t.person.clone(),
        term: t.id.clone(),
        selected_day: at,
        identity_hash: binding_hash(person_in(r, &t.person).unwrap(), t),
    }
}
fn dead(book: &CampaignLeadership, id: &str) -> bool {
    book.deaths.iter().any(|d| d.person == id)
}
fn rank(t: &Term) -> u8 {
    match t.kind {
        TermKind::Leader | TermKind::CoLeader => 0,
        TermKind::Acting => 1,
        TermKind::Candidate => 2,
    }
}
fn choose(
    r: &Roster,
    p: &Party,
    component: Option<&str>,
    at: i32,
    book: &CampaignLeadership,
    initial: bool,
) -> Vec<Holder> {
    let mut choices: Vec<&Term> = p
        .terms
        .iter()
        .filter(|t| {
            t.component.as_deref() == component
                && eligible(r, p, t, at)
                && !dead(book, &t.person)
                && (!initial || t.kind != TermKind::Candidate)
        })
        .collect();
    choices.sort_by(|a, b| rank(a).cmp(&rank(b)).then(a.id.cmp(&b.id)));
    let Some(best) = choices.first().map(|t| rank(t)) else {
        return vec![];
    };
    let mut seen = BTreeSet::new();
    choices
        .into_iter()
        .filter(|t| rank(t) == best && seen.insert(t.person.as_str()))
        .map(|t| holder(r, t, at))
        .collect()
}

fn ensure_with(w: &mut WorldState, r: &Roster) {
    if !w.rules.historical_party_leadership || !w.rules.ideology_blocs {
        return;
    }
    let at = today(w);
    let mut book = w.party_leadership.take().unwrap_or_else(|| {
        let office_identities = w
            .leadership
            .as_ref()
            .into_iter()
            .flatten()
            .filter(|o| o.name.is_some() && o.emergent.is_none())
            .filter_map(|o| {
                let l = r
                    .office_links
                    .iter()
                    .find(|l| l.nation == o.nation && l.since == o.since)?;
                let p = person_in(r, &l.person)?;
                Some(OfficeIdentity {
                    nation: l.nation,
                    person: l.person.clone(),
                    since: l.since.clone(),
                    identity_hash: office_hash(p, &l.since),
                })
            })
            .collect();
        CampaignLeadership {
            version: VERSION,
            roster_version: r.version,
            started_day: at,
            assignments: vec![],
            office_identities,
            executives: vec![],
            deaths: vec![],
            office_exclusions: vec![],
        }
    });
    for n in w.nations.iter().filter(|n| n.alive) {
        if let Some(pol) = gov::polity_in(w, n.id) {
            for spec in pol.parties {
                let p = party_in(r, n.id, spec.id);
                let components: Vec<Option<&str>> = match p {
                    Some(p) if !p.components.is_empty() => {
                        p.components.iter().map(|c| Some(c.id.as_str())).collect()
                    }
                    _ => vec![None],
                };
                for component in components {
                    if book.assignments.iter().any(|a| {
                        a.nation == n.id
                            && a.party == spec.id
                            && a.component.as_deref() == component
                    }) {
                        continue;
                    }
                    let holders = p
                        .map(|p| choose(r, p, component, at, &book, true))
                        .unwrap_or_default();
                    book.assignments.push(Assignment {
                        nation: n.id,
                        party: spec.id.into(),
                        component: component.map(str::to_string),
                        holders,
                        since_day: at,
                        reason: "initial_reference".into(),
                    });
                }
            }
        }
    }
    w.party_leadership = Some(book);
}
pub fn ensure_all(w: &mut WorldState) {
    if !w.rules.historical_party_leadership {
        return;
    }
    if let Ok(r) = roster() {
        ensure_with(w, r)
    }
}

fn executive_id_with(w: &WorldState, _r: &Roster, n: NationId) -> Option<String> {
    if !w.rules.historical_party_leadership {
        return None;
    }
    if let Some(e) = w
        .party_leadership
        .as_ref()?
        .executives
        .iter()
        .find(|e| e.nation == n)
    {
        return Some(e.holder.person.clone());
    }
    let o = w
        .leadership
        .as_ref()?
        .iter()
        .find(|o| o.nation == n && o.name.is_some() && o.emergent.is_none())?;
    w.party_leadership
        .as_ref()?
        .office_identities
        .iter()
        .find(|l| l.nation == n && l.since == o.since)
        .map(|l| l.person.clone())
}
pub fn executive_person(w: &WorldState, n: NationId) -> Option<&'static Person> {
    if !w.rules.historical_party_leadership {
        return None;
    }
    let r = roster().ok()?;
    person(&executive_id_with(w, r, n)?)
}

/// Called only when the existing government engine actually seats an office.
/// It consumes no RNG, changes no votes, and never interprets a date as a win.
pub(crate) fn on_succession(
    w: &mut WorldState,
    n: NationId,
    how: &gov::Succession,
    target_party: Option<&str>,
) {
    if !w.rules.historical_party_leadership {
        return;
    }
    let Ok(r) = roster() else { return };
    on_succession_with(w, r, n, how, target_party)
}
fn on_succession_with(
    w: &mut WorldState,
    r: &Roster,
    n: NationId,
    how: &gov::Succession,
    target_party: Option<&str>,
) {
    on_succession_with_policy(w, r, executive_eligibility::policy().ok(), n, how, target_party)
}
fn on_succession_with_policy(
    w: &mut WorldState,
    r: &Roster,
    policy: Option<&executive_eligibility::Policy>,
    n: NationId,
    how: &gov::Succession,
    target_party: Option<&str>,
) {
    ensure_with(w, r);
    let old = executive_id_with(w, r, n);
    let at = today(w);
    let Some(mut book) = w.party_leadership.take() else {
        return;
    };
    let reason = match how {
        gov::Succession::Election { .. } => "election",
        gov::Succession::Coup { .. } => "coup",
        gov::Succession::Takeover { .. } => "takeover",
        gov::Succession::TermLimit => "term_limit",
        gov::Succession::Death => "death",
        gov::Succession::Programme => "programme",
    };
    let mut affected = BTreeSet::new();
    if let Some(old) = old {
        match how {
            gov::Succession::Death => {
                if !dead(&book, &old) {
                    book.deaths.push(Death {
                        person: old.clone(),
                        day: at,
                    })
                }
                for a in &mut book.assignments {
                    if a.holders.iter().any(|h| h.person == old) {
                        affected.insert((a.nation, a.party.clone()));
                    }
                    a.holders.retain(|h| h.person != old)
                }
                book.executives.retain(|e| e.holder.person != old);
            }
            gov::Succession::TermLimit => {
                if !book
                    .office_exclusions
                    .iter()
                    .any(|e| e.nation == n && e.person == old)
                {
                    book.office_exclusions.push(OfficeExclusion {
                        nation: n,
                        person: old,
                        day: at,
                    })
                }
            }
            _ => {}
        }
    }
    book.executives.retain(|e| e.nation != n);
    if let Some(id) = target_party {
        affected.insert((n, id.to_string()));
    }
    for (party_nation, party_id) in affected {
        let Some(p) = party_in(r, party_nation, &party_id) else {
            continue;
        };
        let indices: Vec<usize> = book
            .assignments
            .iter()
            .enumerate()
            .filter(|(_, a)| a.nation == party_nation && a.party == p.party)
            .map(|(i, _)| i)
            .collect();
        let continuing_single_party = future::eligible_on(at) && p.kind == PartyKind::Party
            && indices.iter().any(|i| !book.assignments[*i].holders.is_empty());
        for i in indices {
            if future::eligible_on(at) && leadership_seats::reviewed_pair(party_nation,&party_id) {
                let additions=leadership_seats::additions(p,&book.assignments[i],at,&book);
                if !additions.is_empty() {
                    book.assignments[i].holders.extend(additions);
                    book.assignments[i].since_day=at;
                    book.assignments[i].reason=reason.into();
                }
                continue;
            }
            if book.assignments[i].holders.is_empty() {
                if continuing_single_party { continue; }
                let component = book.assignments[i].component.clone();
                let selected = if future::eligible_on(at) {
                    future::pick(party_nation, &party_id, component.as_deref(), at, &book, false).into_iter().collect()
                } else { choose(r, p, component.as_deref(), at, &book, false) };
                book.assignments[i].holders = selected;
                book.assignments[i].since_day = at;
                book.assignments[i].reason = reason.into();
            }
        }
    }
    if let Some(p) = target_party.and_then(|id| party_in(r, n, id)) {
        let blocked = |id: &str| {
            book.office_exclusions
                .iter()
                .any(|e| e.nation == n && e.person == id)
        };
        let mut held_people = BTreeSet::new();
        let held: Vec<Holder> = book
            .assignments
            .iter()
            .filter(|a| a.nation == n && a.party == p.party)
            .flat_map(|a| a.holders.iter())
            .filter(|h| {
                !dead(&book, &h.person)
                    && !blocked(&h.person)
                    && policy.is_some_and(|policy|policy.allows_holder(p,h))
                    && held_people.insert(h.person.as_str())
            })
            .cloned()
            .collect();
        let picked = if held.len() == 1 {
            held.into_iter().next()
        } else if held.is_empty() && future::eligible_on(at) {
            let components = if p.components.is_empty() { vec![None] }
                else { p.components.iter().map(|c| Some(c.id.as_str())).collect() };
            let mut choices = components.into_iter()
                .filter(|c|future::component_eligible(n, &p.party, *c))
                .filter_map(|component|future::for_party(n,&p.party).into_iter()
                    .find(|c|c.component.as_deref()==component && !dead(&book,&c.person.id)
                        && !blocked(&c.person.id) && policy.is_some_and(|policy|policy.allows_future(c)))
                    .map(|c|future::holder(c,at))).collect::<Vec<_>>();
            if choices.len() == 1 { choices.pop() } else { None }
        } else if held.is_empty() {
            let mut terms = p
                .terms
                .iter()
                .filter(|t| eligible(r, p, t, at) && !dead(&book, &t.person) && !blocked(&t.person)
                    && policy.is_some_and(|policy|policy.allows_historical(p,t)))
                .collect::<Vec<_>>();
            terms.sort_by_key(|t| rank(t));
            let best = terms.first().map(|t| rank(t));
            let mut ids = BTreeSet::new();
            let choices = terms
                .into_iter()
                .filter(|t| Some(rank(t)) == best && ids.insert(t.person.as_str()))
                .map(|t| holder(r, t, at))
                .collect::<Vec<_>>();
            if choices.len() == 1 {
                choices.into_iter().next()
            } else {
                None
            }
        } else {
            None
        };
        if let Some(holder) = picked {
            book.executives.push(Executive {
                nation: n,
                party: p.party.clone(),
                holder,
                since_day: at,
                reason: reason.into(),
            });
        }
    }
    w.party_leadership = Some(book);
}

pub fn validate_state(w: &WorldState) -> Result<(), String> {
    if !w.rules.historical_party_leadership {
        return if w.party_leadership.is_none() {
            Ok(())
        } else {
            Err("Party leadership state requires its campaign rule.".into())
        };
    }
    let r = roster().map_err(str::to_string)?;
    validate_with(w, r)
}

/// Structural upgrades for explicitly reviewed organization inventories.
/// Only an empty legacy slot can expand. This never selects or replaces people.
/// Validate all affected maps before applying any changes, then run normal strict
/// state validation in the loader. Saving writes the expanded form thereafter.
pub(crate) fn migrate_legacy_empty_components(w: &mut WorldState) -> Result<bool, String> {
    if !w.rules.historical_party_leadership { return Ok(false); }
    let r = roster().map_err(str::to_owned)?;
    migrate_legacy_empty_components_with(w,r)
}
fn migrate_legacy_empty_components_with(w: &mut WorldState, r: &Roster) -> Result<bool, String> {
    if !w.rules.historical_party_leadership { return Ok(false); }
    let Some(book) = w.party_leadership.as_ref() else { return Ok(false) };
    const UPGRADES: &[(NationId, &str, &[&str])] = &[
        (NationId::Japan, "jp_jsp", &["jp_jsp_1945", "jp_sdp_1996"]),
        (NationId::Japan, "jp_komeito", &["jp_komeito_1964", "jp_komei_1994", "jp_komeito_1998"]),
        (NationId::France, "fr_udf", &["fr_udf_federation", "fr_udf_pr", "fr_udf_dl", "fr_udf_cds",
            "fr_udf_fd", "fr_udf_radical", "fr_udf_psd", "fr_udf_perspectives", "fr_udf_ppdf", "fr_udf_direct", "fr_udf_pril"]),
        (NationId::Germany, "de_union", &["de_union_cdu", "de_union_csu"]),
    ];
    let mut replacements = Vec::new();
    for &(nation, id, expected) in UPGRADES {
        let rows = book.assignments.iter().enumerate()
            .filter(|(_, a)| a.nation == nation && a.party == id)
            .collect::<Vec<_>>();
        if rows.is_empty() { continue; } // Ordinary validation handles missing required slots.
        let schema = party_in(r, nation, id).ok_or("Component upgrade has no matching researched party.")?;
        // These two reviewed imports are staged separately. Before ingestion the
        // original empty schema remains valid and has nothing to expand.
        if schema.components.is_empty() && matches!(nation,NationId::France|NationId::Germany) { continue; }
        if schema.components.iter().map(|c| c.id.as_str()).collect::<Vec<_>>() != expected {
            return Err(format!("{id} component upgrade requires its exact documented organization schema."));
        }
        if rows.len() == 1 && rows[0].1.component.is_none() {
            let (index, old) = rows[0];
            if !old.holders.is_empty() {
                return Err(format!("Cannot upgrade {id}: a populated legacy component binding requires explicit identity migration."));
            }
            let expanded = expected.iter().map(|component| {
                let mut a = old.clone();
                a.component = Some((*component).into());
                a
            }).collect::<Vec<_>>();
            replacements.push((index, expanded));
        } else {
            let keys = rows.iter().filter_map(|(_, a)| a.component.as_deref()).collect::<BTreeSet<_>>();
            if rows.iter().any(|(_, a)| a.component.is_none())
                || rows.len() != expected.len() || keys.len() != expected.len()
                || expected.iter().any(|id| !keys.contains(id))
            {
                return Err(format!("Cannot upgrade {id}: mixed, duplicate, unknown or incomplete component assignments."));
            }
        }
    }
    if replacements.is_empty() { return Ok(false); }
    let book = w.party_leadership.as_mut().unwrap();
    // Descending indices keep every unrelated assignment in its original order.
    replacements.sort_by(|a, b| b.0.cmp(&a.0));
    for (index, expanded) in replacements {
        book.assignments.splice(index..=index, expanded);
    }
    Ok(true)
}
fn validate_with(w: &WorldState, r: &Roster) -> Result<(), String> {
    let b = w
        .party_leadership
        .as_ref()
        .ok_or("Party leadership rule requires its saved campaign book.")?;
    let at = today(w);
    if !w.rules.ideology_blocs
        || b.version != VERSION
        || b.roster_version != r.version
        || b.started_day > at
    {
        return Err("Unsupported party leadership book, date or political rules.".into());
    }
    let check_holder = |h: &Holder,
                        n: NationId,
                        pid: &str,
                        component: Option<&str>|
     -> Result<(), String> {
        if let Some(c) = fictional_person(&h.person) {
            if c.nation != n || c.party != pid || c.component.as_deref() != component
                || c.term_id != h.term || h.selected_day < b.started_day || h.selected_day > at
                || !future::eligible_on(h.selected_day) || dead(b, &h.person)
                || h.identity_hash != future::binding_hash(c)
            {
                return Err("Invalid or changed fictional successor identity, affiliation or eligibility date.".into());
            }
            return Ok(());
        }
        let p = party_in(r, n, pid).ok_or("Unknown campaign party leadership record.")?;
        let t = p
            .terms
            .iter()
            .find(|t| t.id == h.term && t.person == h.person && t.component.as_deref() == component)
            .ok_or("Unknown person, party affiliation or leadership term.")?;
        if h.selected_day < b.started_day
            || h.selected_day > at
            || !eligible(r, p, t, h.selected_day)
            || dead(b, &h.person)
            || h.identity_hash
                != binding_hash(
                    person_in(r, &h.person).ok_or("Unknown incumbent person.")?,
                    t,
                )
        {
            return Err("Invalid or changed campaign leadership identity; explicit roster migration is required.".into());
        }
        Ok(())
    };
    let valid_reason = |reason: &str| {
        matches!(
            reason,
            "initial_reference"
                | "election"
                | "coup"
                | "takeover"
                | "term_limit"
                | "death"
                | "programme"
        )
    };
    let mut originals = BTreeSet::new();
    for o in &b.office_identities {
        let p = person_in(r, &o.person).ok_or("Unknown saved original executive identity.")?;
        if !originals.insert(o.nation)
            || !r
                .office_links
                .iter()
                .any(|l| l.nation == o.nation && l.person == o.person && l.since == o.since)
            || o.identity_hash != office_hash(p, &o.since)
        {
            return Err("Invalid or changed original executive identity; explicit roster migration is required.".into());
        }
        if w.leadership.as_ref().is_some_and(|rows| {
            rows.iter().any(|row| {
                row.nation == o.nation
                    && row.name.is_some()
                    && row.emergent.is_none()
                    && row.since == o.since
            })
        }) && dead(b, &o.person)
        {
            return Err("Deceased campaign person still holds the original office.".into());
        }
    }
    for row in w
        .leadership
        .as_ref()
        .into_iter()
        .flatten()
        .filter(|o| o.name.is_some() && o.emergent.is_none())
    {
        if let Some(link) = r
            .office_links
            .iter()
            .find(|l| l.nation == row.nation && l.since == row.since)
        {
            if !b.office_identities.iter().any(|o| {
                o.nation == link.nation && o.person == link.person && o.since == link.since
            }) {
                return Err("Original executive identity is missing; explicit roster migration is required.".into());
            }
        }
    }
    let mut keys = BTreeSet::new();
    for a in &b.assignments {
        if !keys.insert((a.nation, a.party.as_str(), a.component.as_deref()))
            || gov::party_spec(a.nation, &a.party).is_none()
            || a.since_day < b.started_day
            || a.since_day > at
            || !valid_reason(&a.reason)
            || w.nation_opt(a.nation).is_none()
        {
            return Err("Invalid or duplicate party leadership assignment.".into());
        }
        let components = party_in(r, a.nation, &a.party).map(|p| &p.components);
        if a.component
            .as_ref()
            .is_some_and(|id| components.is_none_or(|cs| !cs.iter().any(|c| c.id == *id)))
            || a.component.is_none() && components.is_some_and(|cs| !cs.is_empty())
        {
            return Err("Unknown or missing campaign party component.".into());
        }
        let mut holders = BTreeSet::new();
        for h in &a.holders {
            if fictional_person(&h.person).is_some() && a.reason == "initial_reference" {
                return Err("A fictional successor requires an actual campaign succession event.".into());
            }
            if h.selected_day > a.since_day {
                return Err(
                    "Incumbent selection occurs after its recorded party succession.".into(),
                );
            }
            if !holders.insert(h.person.as_str()) {
                return Err("Duplicate collective leader.".into());
            }
            check_holder(h, a.nation, &a.party, a.component.as_deref())?
        }
    }
    for n in w.nations.iter().filter(|n| n.alive) {
        if let Some(pol) = gov::polity_in(w, n.id) {
            for p in pol.parties {
                let source = party_in(r, n.id, p.id);
                let components = source
                    .filter(|p| !p.components.is_empty())
                    .map(|p| {
                        p.components
                            .iter()
                            .map(|c| Some(c.id.as_str()))
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_else(|| vec![None]);
                for component in components {
                    if !keys.contains(&(n.id, p.id, component)) {
                        return Err(
                            "Campaign leadership book is missing a party or component assignment."
                                .into(),
                        );
                    }
                }
            }
        }
    }
    let mut heads = BTreeSet::new();
    for e in &b.executives {
        if !heads.insert(e.nation)
            || e.since_day < b.started_day
            || e.since_day > at
            || e.holder.selected_day > e.since_day
            || !valid_reason(&e.reason)
            || w.nation_opt(e.nation).is_none()
            || b.office_exclusions
                .iter()
                .any(|x| x.nation == e.nation && x.person == e.holder.person)
        {
            return Err("Invalid or duplicate campaign executive.".into());
        }
        let p = party_in(r, e.nation, &e.party).ok_or("Unknown executive party.")?;
        let component = if let Some(c) = fictional_person(&e.holder.person) {
            if e.reason == "initial_reference" {
                return Err("A fictional executive requires an actual campaign succession event.".into());
            }
            c.component.as_deref()
        } else {
            p.terms.iter().find(|t| t.id == e.holder.term)
                .ok_or("Unknown executive term.")?.component.as_deref()
        };
        check_holder(&e.holder, e.nation, &e.party, component)?;
        let tie = w
            .leadership
            .as_ref()
            .and_then(|rows| rows.iter().find(|o| o.nation == e.nation))
            .and_then(|o| o.tie_now());
        let actual_since = w
            .leadership
            .as_ref()
            .and_then(|rows| rows.iter().find(|o| o.nation == e.nation))
            .and_then(|o| o.emergent.as_ref())
            .and_then(|e| day(&e.since));
        if actual_since != Some(e.since_day) {
            return Err("Campaign executive date does not match its actual seating event.".into());
        }
        if tie != Some(crate::data::Tie::Party(e.party.clone())) {
            return Err(
                "Campaign executive no longer matches the actual government office.".into(),
            );
        }
    }
    let mut deaths = BTreeSet::new();
    for d in &b.deaths {
        if !deaths.insert(d.person.as_str())
            || person_in(r, &d.person).is_none() && fictional_person(&d.person).is_none()
            || d.day < b.started_day
            || d.day > at
            || fictional_person(&d.person).is_some() && d.day < day(future::FROM).unwrap()
        {
            return Err("Invalid campaign person death.".into());
        }
    }
    let mut exits = BTreeSet::new();
    for e in &b.office_exclusions {
        if !exits.insert((e.nation, e.person.as_str()))
            || person_in(r, &e.person).is_none() && fictional_person(&e.person).is_none()
            || w.nation_opt(e.nation).is_none()
            || e.day < b.started_day
            || e.day > at
            || fictional_person(&e.person).is_some() && e.day < day(future::FROM).unwrap()
        {
            return Err("Invalid campaign office exclusion.".into());
        }
    }
    Ok(())
}

/// Whole read model, including every small party and explicit data gaps.
pub fn view(w: &WorldState, n: NationId) -> Value {
    view_on(w, n, today(w), &date_label(today(w)))
}
/// Read historical references on an exact date without advancing campaign time.
/// Campaign bindings are still explicitly shown with their own current date.
pub fn reference_view(w: &WorldState, n: NationId, date: &str) -> Value {
    let Some(at) = day(date) else {
        return json!({"error":"Use a valid Gregorian date in YYYY-MM-DD format.","parties":[]});
    };
    view_on(w, n, at, date)
}
fn view_on(w: &WorldState, n: NationId, at: i32, date: &str) -> Value {
    let Ok(r) = roster() else {
        return json!({"enabled":w.rules.historical_party_leadership,"error":"Party leadership catalogue is invalid.","parties":[]});
    };
    view_with(w, n, at, date, r)
}
fn view_with(w: &WorldState, n: NationId, at: i32, date: &str, r: &Roster) -> Value {
    let rows=gov::polity_in(w,n).map(|pol|pol.parties.iter().map(|s|{
        let p=party_in(r,n,s.id);let assignments=w.party_leadership.as_ref().map(|b|b.assignments.iter().filter(|a|a.nation==n&&a.party==s.id).collect::<Vec<_>>()).unwrap_or_default();
        let historical=p.map(|p|p.terms.iter().filter(|t|historical_on(r,p,t,at)&&t.kind!=TermKind::Candidate).map(|t|json!({"term":t,"person":person_in(r,&t.person),"executive_eligibility":executive_eligibility::historical_info(p,t)})).collect::<Vec<_>>()).unwrap_or_default();
        let uncertain=p.map(|p|p.terms.iter().filter(|t|possibly_historical_on(r,p,t,at)).map(|t|json!({"term":t,"person":person_in(r,&t.person),"executive_eligibility":executive_eligibility::historical_info(p,t),"reason":"The sourced date bounds may overlap this date, but do not establish an incumbent or eligible candidate."})).collect::<Vec<_>>()).unwrap_or_default();
        let candidates=p.map(|p|p.terms.iter().filter(|t|eligible(r,p,t,at)&&(at!=today(w)||w.party_leadership.as_ref().is_none_or(|b|!dead(b,&t.person)))).map(|t|json!({"term":t,"person":person_in(r,&t.person),"executive_eligibility":executive_eligibility::historical_info(p,t)})).collect::<Vec<_>>()).unwrap_or_default();
        let holders=assignments.iter().flat_map(|a|a.holders.iter().map(move|h|json!({"person":display_person_in(r,&h.person),"person_id":h.person,"term_id":h.term,"component":a.component,"executive_eligibility":p.map(|p|executive_eligibility::holder_info(p,h)),"since_day":h.selected_day,"since_label":date_label(h.selected_day),"party_succession_day":a.since_day,"role":if fictional_person(&h.person).is_some(){Some(if leadership_seats::reviewed_pair(n,s.id){"party co-leader (fictional)"}else{"party leader (fictional)"})}else{p.and_then(|p|p.terms.iter().find(|t|t.id==h.term).map(|t|t.role.as_str()))},"kind":if fictional_person(&h.person).is_some(){Some(if leadership_seats::reviewed_pair(n,s.id){TermKind::CoLeader}else{TermKind::Leader})}else{p.and_then(|p|p.terms.iter().find(|t|t.id==h.term).map(|t|t.kind.clone()))},"reason":a.reason,"historical_reference_continues":p.is_some_and(|p|p.terms.iter().find(|t|t.id==h.term).is_some_and(|t|eligible(r,p,t,at)))}))).collect::<Vec<_>>();
        let future_candidates=if future::eligible_on(at) { future::for_party(n,s.id).into_iter().filter(|c|future::component_eligible(n,s.id,c.component.as_deref())).filter(|c|at==today(w)||future_reference_party_exists(p,c.component.as_deref(),at)).map(|c|json!({"person":display_person_in(r,&c.person.id),"term_id":c.term_id,"component":c.component,"eligible":at!=today(w)||w.party_leadership.as_ref().is_none_or(|b|!dead(b,&c.person.id)),"origin":"fictional_successor","role":"fictional party successor candidate","executive_eligibility":executive_eligibility::future_info(c),"historical_continuation_status":future::historical_continuation(n,s.id).map(|r|r.status.as_str()),"historical_continuation":future::historical_continuation(n,s.id)})).collect::<Vec<_>>() } else {vec![]};
        let future_preview=future::for_party(n,s.id).into_iter().map(|c|json!({"person":display_person_in(r,&c.person.id),"term_id":c.term_id,"component":c.component,"eligible":false,"component_available":future::component_eligible(n,s.id,c.component.as_deref()),"origin":"fictional_successor","role":"fictional party successor preview","executive_eligibility":executive_eligibility::future_info(c),"historical_continuation_status":future::historical_continuation(n,s.id).map(|r|r.status.as_str()),"historical_continuation":future::historical_continuation(n,s.id),"condition":if future::component_eligible(n,s.id,c.component.as_deref()){"This party or component must remain active in the campaign; previews never appoint a leader or predict party survival."}else{"Archived organization template. New future vacancies use the surviving organization; existing campaign incumbents remain unchanged."}})).collect::<Vec<_>>();
        let status=if !w.rules.historical_party_leadership{"reference_only"}else if !holders.is_empty(){"campaign_incumbent"}else if future_candidates.iter().any(|c|c["eligible"]==true){"fictional_candidates"}else if candidates.is_empty(){"coverage_gap"}else{"eligible_candidates"};
        json!({"party_id":s.id,"party_name":s.name,"kind":p.map(|p|&p.kind),"coverage":p.map(|p|&p.coverage),"identity_note":p.and_then(|p|p.identity_note.as_ref()),"sources":p.map(|p|&p.sources),"components":p.map(|p|&p.components),"status":status,"historical":historical,"uncertain_historical":uncertain,"campaign":holders,"eligible":candidates,"future_candidates":future_candidates,"future_preview":future_preview,"future_leadership_seats":p.map(|p|leadership_seats::view(p,&assignments)),"gaps":p.map(|p|&p.gaps),"reason":if p.is_none(){Some("No researched leadership roster for this simulation party.")}else if status=="coverage_gap"{Some("No eligible person is available for this date. Historical gaps are not filled with fictional people.")}else{None}})
    }).collect::<Vec<_>>()).unwrap_or_default();
    json!({"enabled":w.rules.historical_party_leadership,"roster_version":r.version,"reference_from":r.reference_from,"reference_through":r.reference_through,"date":date,"campaign_date":date_label(today(w)),"eligibility_context":if at==today(w)&&w.rules.historical_party_leadership{"campaign"}else if future::eligible_on(at){"fictional_future_reference"}else{"historical_reference"},"nation":n,"parties":rows,"executive_person":executive_id_with(w,r,n).map(|id|display_person_in(r,&id)),"executive_mortality_supported":executive_person(w,n).is_some_and(|p|matches!(&p.born,Some(DateBound::Day(_)))),"future_policy":future::policy(at),"executive_policy":executive_eligibility::view(n),"note":"Historical references end on 7 September 2026. Fictional successor candidates are available from 8 September 2026 through 2035. Saved incumbents change only through actual campaign events; no historical or future election outcome is scheduled."})
}
fn future_reference_party_exists(p: Option<&Party>, component: Option<&str>, at: i32) -> bool {
    let Some(p) = p else { return false };
    let exists = |f: &Option<DateBound>, d: &Option<DateBound>| {
        f.as_ref().and_then(DateBound::bounds).is_none_or(|(first, _)| first <= at)
            && d.as_ref().and_then(DateBound::bounds).is_none_or(|(_, last)| last > at)
    };
    exists(&p.founded, &p.dissolved) && component.is_none_or(|id| p.components.iter()
        .find(|c| c.id == id).is_some_and(|c| exists(&c.founded, &c.dissolved)))
}
#[cfg(test)]
mod tests {
    use super::*;
    fn bound(s: &str) -> DateBound {
        DateBound::Day(s.into())
    }
    fn fixture() -> (WorldState, Roster) {
        let people = (0..4)
            .map(|i| Person {
                id: format!("fixture_{i}"),
                name: format!("Test-only candidate {i}"),
                native: None,
                born: Some(bound("1930-01-01")),
                died: None,
                sources: vec!["test-only fixture".into()],
            })
            .collect();
        let term = |i: usize, kind: TermKind| Term {
            id: format!("test_term_{i}"),
            person: format!("fixture_{i}"),
            component: None,
            role: "party leader".into(),
            kind,
            from: bound("1980-01-01"),
            until: DateBound::Open,
            affiliation_from: bound("1980-01-01"),
            affiliation_until: DateBound::Open,
            sources: vec!["test-only fixture".into()],
            note: None,
        };
        let party = |id: &str, terms: Vec<Term>| Party {
            nation: NationId::UK,
            party: id.into(),
            kind: PartyKind::Party,
            coverage: Coverage::Partial,
            sources: vec!["test-only identity".into()],
            identity_note: None,
            founded: None,
            dissolved: None,
            components: vec![],
            terms,
            gaps: vec![],
        };
        let r = Roster {
            version: VERSION,
            reference_from: "1990-01-01".into(),
            reference_through: "2026-09-07".into(),
            people,
            parties: vec![
                party(
                    "uk_con",
                    vec![term(0, TermKind::Leader), term(1, TermKind::Candidate)],
                ),
                party(
                    "uk_lab",
                    vec![term(2, TermKind::CoLeader), term(3, TermKind::CoLeader)],
                ),
            ],
            office_links: vec![OfficeLink {
                nation: NationId::UK,
                person: "fixture_0".into(),
                since: "1979-05-04".into(),
                sources: vec!["test-only explicit executive binding".into()],
            }],
        };
        validate_roster(&r).unwrap();
        let mut w = crate::init::world_1990(GameRules {
            ideology_blocs: true,
            ..Default::default()
        });
        w.rules.historical_party_leadership = true;
        ensure_with(&mut w, &r);
        validate_with(&w, &r).unwrap();
        (w, r)
    }
    fn assignment<'a>(w: &'a WorldState, id: &str) -> &'a Assignment {
        w.party_leadership
            .as_ref()
            .unwrap()
            .assignments
            .iter()
            .find(|a| a.nation == NationId::UK && a.party == id)
            .unwrap()
    }
    fn seat(w: &mut WorldState, r: &Roster, how: gov::Succession) {
        let (next, _) = gov::succession_seat(w, NationId::UK, &how).unwrap();
        let policy = executive_eligibility::fixture_policy(r);
        on_succession_with_policy(w, r, Some(&policy), NationId::UK, &how, next.party.as_deref());
        let o = w
            .leadership
            .as_mut()
            .unwrap()
            .iter_mut()
            .find(|o| o.nation == NationId::UK)
            .unwrap();
        o.name = None;
        o.native = None;
        o.tie = None;
        o.bloc_override = None;
        o.must_leave_by = None;
        o.also.clear();
        o.emergent = Some(next);
    }
    fn future_world(date: &str) -> WorldState {
        let mut w = crate::init::world_1990(GameRules {
            ideology_blocs: true, historical_party_leadership: true,
            daily_simulation: true, ..Default::default()
        });
        (w.year, w.month, w.day) = strict_date(date).unwrap();
        w
    }
    fn legacy_empty_japanese_components() -> WorldState {
        let mut w = future_world("1990-02-01");
        let b = w.party_leadership.as_mut().unwrap();
        let mut seen = BTreeSet::new();
        b.assignments = b.assignments.iter().filter_map(|a| {
            if a.nation != NationId::Japan || !matches!(a.party.as_str(), "jp_jsp" | "jp_komeito") {
                return Some(a.clone());
            }
            if !seen.insert(a.party.clone()) { return None; }
            let mut old = a.clone();
            old.component = None;
            old.holders.clear();
            old.since_day = 20;
            old.reason = "death".into();
            Some(old)
        }).collect();
        w
    }
    #[test]
    fn legacy_japanese_empty_slots_upgrade_without_changing_campaign_state() {
        let mut w = legacy_empty_japanese_components();
        // Saved order is arbitrary: neither the migration nor its index math
        // may assume that the JSP row precedes Komeito.
        w.party_leadership.as_mut().unwrap().assignments.reverse();
        assert!(validate_state(&w).is_err());
        let upgraded = crate::load(&crate::save(&w)).unwrap();
        validate_state(&upgraded).unwrap();
        for (party, expected) in [("jp_jsp", 2), ("jp_komeito", 3)] {
            let rows = upgraded.party_leadership.as_ref().unwrap().assignments.iter()
                .filter(|a| a.nation == NationId::Japan && a.party == party).collect::<Vec<_>>();
            assert_eq!(rows.len(), expected);
            assert!(rows.iter().all(|a| a.component.is_some() && a.holders.is_empty()
                && a.since_day == 20 && a.reason == "death"));
        }
        let strip_upgraded_slots = |world: &WorldState| {
            let mut value = serde_json::to_value(world).unwrap();
            value["party_leadership"]["assignments"].as_array_mut().unwrap().retain(|a| {
                a["nation"] != "Japan" || !matches!(a["party"].as_str(), Some("jp_jsp" | "jp_komeito"))
            });
            value
        };
        assert_eq!(strip_upgraded_slots(&w), strip_upgraded_slots(&upgraded));
        let reloaded = crate::load(&crate::save(&upgraded)).unwrap();
        assert_eq!(crate::save(&upgraded), crate::save(&reloaded));
        let mut modern = upgraded.clone();
        assert!(!migrate_legacy_empty_components(&mut modern).unwrap());
        assert_eq!(crate::save(&modern), crate::save(&upgraded));
    }
    #[test]
    fn japanese_component_upgrade_rejects_populated_and_mixed_maps_atomically() {
        let legacy = legacy_empty_japanese_components();
        let index = legacy.party_leadership.as_ref().unwrap().assignments.iter()
            .position(|a| a.nation == NationId::Japan && a.party == "jp_komeito").unwrap();
        let mut populated = legacy.clone();
        populated.party_leadership.as_mut().unwrap().assignments[index].holders.push(Holder {
            person: "legacy_named_person".into(), term: "legacy_term".into(),
            selected_day: 0, identity_hash: "legacy_hash".into(),
        });
        let mut mixed = legacy.clone();
        let mut new_row = mixed.party_leadership.as_ref().unwrap().assignments[index].clone();
        new_row.component = Some("jp_komeito_1964".into());
        mixed.party_leadership.as_mut().unwrap().assignments.push(new_row);
        let mut duplicate = legacy.clone();
        let repeated = duplicate.party_leadership.as_ref().unwrap().assignments[index].clone();
        duplicate.party_leadership.as_mut().unwrap().assignments.push(repeated);
        let modern = crate::load(&crate::save(&legacy)).unwrap();
        let mut incomplete = modern.clone();
        incomplete.party_leadership.as_mut().unwrap().assignments.retain(|a| a.component.as_deref() != Some("jp_sdp_1996"));
        let mut unknown = modern;
        unknown.party_leadership.as_mut().unwrap().assignments.iter_mut()
            .find(|a| a.component.as_deref() == Some("jp_sdp_1996")).unwrap().component = Some("unsupported_organization".into());
        for mut invalid in [populated, mixed, duplicate, incomplete, unknown] {
            let before = crate::save(&invalid);
            assert!(migrate_legacy_empty_components(&mut invalid).is_err());
            assert_eq!(crate::save(&invalid), before, "failed upgrades are atomic");
            assert!(crate::load(&before).is_err());
        }
        let mut bad_metadata = legacy;
        bad_metadata.party_leadership.as_mut().unwrap().assignments[index].reason = "invented_reason".into();
        assert!(crate::load(&crate::save(&bad_metadata)).is_err(), "normal validation must still reject invalid metadata");
    }
    #[test]
    fn future_cutoff_never_replaces_incumbents_without_a_succession_event() {
        let mut w = future_world("2026-09-07");
        let original = executive_person(&w, NationId::UK).unwrap().id.clone();
        let before = serde_json::to_string(&w.party_leadership).unwrap();
        w.day = 8;
        ensure_all(&mut w);
        assert_eq!(serde_json::to_string(&w.party_leadership).unwrap(), before);
        assert_eq!(executive_person(&w, NationId::UK).unwrap().id, original);
        let saved = crate::save(&w);
        let preview = reference_view(&w, NationId::UK, "2035-12-31");
        assert_eq!(preview["reference_through"], "2026-09-07");
        assert!(preview["parties"][0]["historical"].as_array().unwrap().is_empty());
        assert_eq!(preview["parties"][0]["future_candidates"][0]["person"]["fiction"]["origin"], "fictional_successor");
        assert_eq!(crate::save(&w), saved);

        let mut before_cutoff = future_world("2026-09-07");
        gov::seat_office(&mut before_cutoff, NationId::UK, &gov::Succession::Death);
        assert!(executive_person(&before_cutoff, NationId::UK).is_none_or(|p| fictional_person(&p.id).is_none()));
        gov::seat_office(&mut w, NationId::UK, &gov::Succession::Death);
        let next = executive_person(&w, NationId::UK).unwrap();
        assert!(fictional_person(&next.id).is_some());
        assert!(next.sources.is_empty());
        assert_eq!(view(&w, NationId::UK)["executive_person"]["fiction"]["origin"], "fictional_successor");
        validate_state(&w).unwrap();
    }
    #[test]
    fn japanese_future_vacancies_use_one_surviving_organization_without_replacing_incumbents() {
        for (party, active) in [("jp_jsp", "jp_sdp_1996"), ("jp_komeito", "jp_komeito_1998")] {
            let mut w = future_world("2027-01-01");
            let before = w.party_leadership.as_ref().unwrap().assignments.iter()
                .filter(|a| a.nation == NationId::Japan && a.party == party)
                .flat_map(|a| a.holders.iter()).map(|h| h.person.clone()).collect::<Vec<_>>();
            assert_eq!(before.len(), 1, "one 1990 incumbent in {party}");
            gov::seat_office(&mut w, NationId::Japan, &gov::Succession::Election { leader: party.into() });
            assert_eq!(executive_person(&w, NationId::Japan).unwrap().id, before[0]);
            let mut term_limited = w.clone();
            term_limited.day = 2;
            gov::seat_office(&mut term_limited, NationId::Japan, &gov::Succession::TermLimit);
            let next = executive_person(&term_limited, NationId::Japan).unwrap();
            assert_eq!(fictional_person(&next.id).unwrap().component.as_deref(), Some(active));
            assert!(term_limited.party_leadership.as_ref().unwrap().assignments.iter()
                .any(|a| a.holders.iter().any(|h| h.person == before[0])), "term limit preserves party incumbent");
            validate_state(&term_limited).unwrap();
            w.day = 2;
            gov::seat_office(&mut w, NationId::Japan, &gov::Succession::Death);
            let next = executive_person(&w, NationId::Japan).unwrap();
            assert_eq!(fictional_person(&next.id).unwrap().component.as_deref(), Some(active));
            let slots = w.party_leadership.as_ref().unwrap().assignments.iter()
                .filter(|a| a.nation == NationId::Japan && a.party == party).collect::<Vec<_>>();
            assert_eq!(slots.iter().map(|a| a.holders.len()).sum::<usize>(), 1);
            assert!(slots.iter().filter(|a| a.component.as_deref() != Some(active)).all(|a| a.holders.is_empty()));
            validate_state(&w).unwrap();
            let reloaded = crate::load(&crate::save(&w)).unwrap();
            assert_eq!(crate::save(&w), crate::save(&reloaded));
            let model = view(&w, NationId::Japan);
            let row = model["parties"].as_array().unwrap().iter().find(|p| p["party_id"] == party).unwrap();
            assert_eq!(row["future_candidates"].as_array().unwrap().len(), 4);
            assert!(row["future_candidates"].as_array().unwrap().iter().all(|c| c["component"] == active));
        }
    }
    #[test]
    fn future_coalition_components_remain_collective() {
        let mut w = future_world("2027-01-01");
        // A specific opposition coalition guarantees an actual election seating
        // event as more countries gain researched coalition rows.
        let p = roster().unwrap().parties.iter().find(|p|
            p.nation == NationId::UK && p.kind == PartyKind::Coalition).unwrap();
        assert!(p.components.len() > 1);
        for a in &mut w.party_leadership.as_mut().unwrap().assignments {
            if a.nation == p.nation && a.party == p.party { a.holders.clear(); }
        }
        gov::seat_office(&mut w, p.nation, &gov::Succession::Election { leader: p.party.clone() });
        let holders = w.party_leadership.as_ref().unwrap().assignments.iter()
            .filter(|a| a.nation == p.nation && a.party == p.party).flat_map(|a| a.holders.iter()).collect::<Vec<_>>();
        assert_eq!(holders.len(), p.components.len());
        assert!(holders.iter().all(|h| fictional_person(&h.person).is_some()));
        assert!(executive_person(&w, p.nation).is_none());
        validate_state(&w).unwrap();
    }
    #[test]
    fn fictional_successor_save_replay_death_and_term_limits_keep_stable_identities() {
        let mut w = future_world("2027-01-01");
        gov::seat_office(&mut w, NationId::UK, &gov::Succession::Death);
        let first = executive_person(&w, NationId::UK).unwrap().id.clone();
        let mut loaded = crate::load(&crate::save(&w)).unwrap();
        assert_eq!(crate::save(&w), crate::save(&loaded));
        for state in [&mut w, &mut loaded] {
            state.day = 2;
            gov::seat_office(state, NationId::UK, &gov::Succession::TermLimit);
            let second = executive_person(state, NationId::UK).unwrap();
            assert_ne!(second.id, first);
            assert!(fictional_person(&second.id).is_some());
            assert_eq!(assignment(state, "uk_con").holders[0].person, first);
            state.day = 3;
            gov::seat_office(state, NationId::UK, &gov::Succession::Death);
            validate_state(state).unwrap();
        }
        assert_eq!(crate::save(&w), crate::save(&loaded));
        w.year = 2036;
        let incumbent = executive_person(&w, NationId::UK).unwrap().id.clone();
        ensure_all(&mut w);
        assert_eq!(executive_person(&w, NationId::UK).unwrap().id, incumbent);
        validate_state(&crate::load(&crate::save(&w)).unwrap()).unwrap();
    }
    #[test]
    fn forged_fictional_dates_affiliations_and_initial_assignments_are_rejected() {
        let mut w = future_world("2027-01-01");
        gov::seat_office(&mut w, NationId::UK, &gov::Succession::Death);
        validate_state(&w).unwrap();
        let index = w.party_leadership.as_ref().unwrap().assignments.iter()
            .position(|a| a.nation == NationId::UK && a.party == "uk_con").unwrap();
        let mut bad = w.clone();
        bad.party_leadership.as_mut().unwrap().assignments[index].holders[0].selected_day = day("2026-09-07").unwrap();
        assert!(validate_state(&bad).is_err());
        let mut bad = w.clone();
        bad.party_leadership.as_mut().unwrap().assignments[index].reason = "initial_reference".into();
        assert!(validate_state(&bad).is_err());
        let mut bad = w.clone();
        bad.party_leadership.as_mut().unwrap().assignments[index].holders[0].identity_hash = "changed".into();
        assert!(validate_state(&bad).is_err());
        let mut bad = w.clone();
        let other = future::for_party(NationId::Japan, "jp_ldp")[0];
        bad.party_leadership.as_mut().unwrap().assignments[index].holders[0] = future::holder(other, today(&w));
        assert!(validate_state(&bad).is_err());
    }
    #[test]
    fn future_previews_are_read_only_and_dissolved_reference_parties_stay_distinct() {
        let (w, mut r) = fixture();
        let old = crate::save(&w);
        let model = view_with(&w, NationId::UK, today(&w), "1990-01-01", &r);
        assert!(model["parties"][0]["future_candidates"].as_array().unwrap().is_empty());
        assert!(model["parties"][0]["future_preview"].as_array().unwrap().iter().all(|c| c["eligible"] == false));
        assert_eq!(crate::save(&w), old);
        r.parties[0].dissolved = Some(bound("2000-01-01"));
        let model = view_with(&w, NationId::UK, day("2030-01-01").unwrap(), "2030-01-01", &r);
        assert!(model["parties"][0]["future_candidates"].as_array().unwrap().is_empty());
        assert_eq!(model["parties"][0]["future_preview"].as_array().unwrap().len(), 4);
        let mut diverged = w.clone();
        diverged.year = 2030;
        let model = view_with(&diverged, NationId::UK, today(&diverged), "2030-01-01", &r);
        assert_eq!(model["parties"][0]["future_candidates"].as_array().unwrap().len(), 4);
    }
    #[test]
    fn exact_gregorian_dates_and_conservative_partial_bounds() {
        for invalid in [
            "1900-02-29",
            "1990-02-30",
            "1990-04-31",
            "0000-01-01",
            "1990-1-01",
        ] {
            assert!(day(invalid).is_none(), "{invalid}")
        }
        assert!(day("2000-02-29").is_some());
        for invalid in ["0000", "+990", "a990"] {
            assert!(DateBound::Year(invalid.into()).bounds().is_none())
        }
        let (_, r) = fixture();
        let mut t = r.parties[0].terms[0].clone();
        t.from = DateBound::Year("1990".into());
        t.until = DateBound::Month("1992-03".into());
        assert!(!eligible(&r, &r.parties[0], &t, day("1990-12-30").unwrap()));
        assert!(eligible(&r, &r.parties[0], &t, day("1990-12-31").unwrap()));
        assert!(!eligible(&r, &r.parties[0], &t, day("1992-03-01").unwrap()));
        t.from = DateBound::Unknown;
        assert!(!eligible(&r, &r.parties[0], &t, 0));
        t.from = bound("1990-01-01");
        t.until = DateBound::Open;
        assert!(!eligible(&r, &r.parties[0], &t, day("2026-09-08").unwrap()));
    }
    #[test]
    fn roster_rejects_duplicates_unlinked_people_and_unsourced_claims() {
        let (_, r) = fixture();
        let mut bad = r.clone();
        bad.people.push(bad.people[0].clone());
        assert!(validate_roster(&bad).is_err());
        let mut bad = r.clone();
        bad.parties[0].terms[0].person = "invented".into();
        assert!(validate_roster(&bad).is_err());
        let mut bad = r.clone();
        bad.parties[0].terms[0].sources.clear();
        assert!(validate_roster(&bad).is_err());
        let mut bad = r.clone();
        bad.office_links[0].since = "1980-01-01".into();
        assert!(validate_roster(&bad).is_err());
        let mut bad = r.clone();
        bad.parties[0].kind = PartyKind::Unknown;
        bad.parties[0].coverage = Coverage::Verified;
        assert!(validate_roster(&bad).is_err());
    }
    #[test]
    fn eligibility_honors_person_party_component_and_membership_dates() {
        let (_, mut r) = fixture();
        let at = 0;
        r.people[0].died = Some(bound("1990-01-01"));
        assert!(!eligible(&r, &r.parties[0], &r.parties[0].terms[0], at));
        r.people[0].died = None;
        r.parties[0].founded = Some(bound("1991-01-01"));
        assert!(!eligible(&r, &r.parties[0], &r.parties[0].terms[0], at));
        r.parties[0].founded = None;
        r.parties[0].terms[0].affiliation_until = DateBound::Unknown;
        assert!(!eligible(&r, &r.parties[0], &r.parties[0].terms[0], at));
        assert!(
            historical_on(&r, &r.parties[0], &r.parties[0].terms[0], at),
            "known office history remains visible when wider affiliation is unknown"
        );
        r.parties[0].components.push(Component {
            id: "component".into(),
            name: "Test component".into(),
            sources: vec!["fixture".into()],
            founded: Some(bound("1991-01-01")),
            dissolved: None,
        });
        r.parties[0].terms[0].component = Some("component".into());
        assert!(!historical_on(
            &r,
            &r.parties[0],
            &r.parties[0].terms[0],
            at
        ));
    }
    #[test]
    fn incumbent_survives_reference_expiry_without_votes_cash_or_rng_changes() {
        let (mut w, mut r) = fixture();
        r.parties[0].terms[0].until = bound("1990-02-01");
        r.parties[0].terms[0].affiliation_until = bound("1990-02-01");
        w.party_leadership = None;
        ensure_with(&mut w, &r);
        w.year = 2000;
        let before = serde_json::to_value(&w).unwrap();
        ensure_with(&mut w, &r);
        assert_eq!(serde_json::to_value(&w).unwrap(), before);
        assert_eq!(assignment(&w, "uk_con").holders[0].person, "fixture_0");
        validate_with(&w, &r).unwrap();
        assert!(!eligible(
            &r,
            &r.parties[0],
            &r.parties[0].terms[0],
            today(&w)
        ));
    }
    #[test]
    fn collective_and_components_never_invent_one_executive() {
        let (mut w, r) = fixture();
        assert_eq!(assignment(&w, "uk_lab").holders.len(), 2);
        seat(
            &mut w,
            &r,
            gov::Succession::Election {
                leader: "uk_lab".into(),
            },
        );
        assert!(w.party_leadership.as_ref().unwrap().executives.is_empty());
        validate_with(&w, &r).unwrap();
        let (mut w, mut r) = fixture();
        w.party_leadership = None;
        r.parties[1].kind = PartyKind::Coalition;
        r.parties[1].components = (2..4)
            .map(|i| Component {
                id: format!("org_{i}"),
                name: format!("Test organization {i}"),
                sources: vec!["fixture".into()],
                founded: None,
                dissolved: None,
            })
            .collect();
        for (i, t) in r.parties[1].terms.iter_mut().enumerate() {
            t.component = Some(format!("org_{}", i + 2));
        }
        validate_roster(&r).unwrap();
        ensure_with(&mut w, &r);
        assert_eq!(
            w.party_leadership
                .as_ref()
                .unwrap()
                .assignments
                .iter()
                .filter(|a| a.nation == NationId::UK && a.party == "uk_lab")
                .count(),
            2
        );
        validate_with(&w, &r).unwrap();
    }
    #[test]
    fn actual_death_excludes_person_and_binds_eligible_replacement_without_rng() {
        let (mut w, r) = fixture();
        let rng = serde_json::to_value(&w.rng).unwrap();
        seat(&mut w, &r, gov::Succession::Death);
        assert_eq!(
            w.party_leadership.as_ref().unwrap().deaths[0].person,
            "fixture_0"
        );
        assert_eq!(assignment(&w, "uk_con").holders[0].person, "fixture_1");
        assert_eq!(
            executive_id_with(&w, &r, NationId::UK).as_deref(),
            Some("fixture_1")
        );
        assert_eq!(serde_json::to_value(&w.rng).unwrap(), rng);
        validate_with(&w, &r).unwrap();
        seat(&mut w, &r, gov::Succession::Death);
        assert!(assignment(&w, "uk_con").holders.is_empty());
        assert!(executive_id_with(&w, &r, NationId::UK).is_none());
        validate_with(&w, &r).unwrap();
    }
    #[test]
    fn term_limit_excludes_office_holder_but_preserves_party_leadership() {
        let (mut w, r) = fixture();
        seat(&mut w, &r, gov::Succession::TermLimit);
        assert_eq!(assignment(&w, "uk_con").holders[0].person, "fixture_0");
        assert_eq!(
            executive_id_with(&w, &r, NationId::UK).as_deref(),
            Some("fixture_1")
        );
        assert!(w.party_leadership.as_ref().unwrap().deaths.is_empty());
        validate_with(&w, &r).unwrap();
        seat(
            &mut w,
            &r,
            gov::Succession::Election {
                leader: "uk_lab".into(),
            },
        );
        seat(
            &mut w,
            &r,
            gov::Succession::Election {
                leader: "uk_con".into(),
            },
        );
        assert_ne!(
            executive_id_with(&w, &r, NationId::UK).as_deref(),
            Some("fixture_0")
        );
    }
    #[test]
    fn saved_identity_requires_original_source_affiliation_and_chronology() {
        let (w, r) = fixture();
        let mut bad = w.clone();
        bad.party_leadership.as_mut().unwrap().assignments[0].since_day = 1;
        assert!(validate_with(&bad, &r).is_err());
        let mut bad = w.clone();
        bad.party_leadership.as_mut().unwrap().assignments.pop();
        assert!(validate_with(&bad, &r).is_err());
        let mut bad = w.clone();
        bad.party_leadership.as_mut().unwrap().office_identities[0].person = "fixture_1".into();
        assert!(validate_with(&bad, &r).is_err());
        let mut bad = w.clone();
        let a = bad
            .party_leadership
            .as_mut()
            .unwrap()
            .assignments
            .iter_mut()
            .find(|a| !a.holders.is_empty())
            .unwrap();
        a.holders[0].person = "fixture_3".into();
        assert!(validate_with(&bad, &r).is_err());
        let mut changed = r.clone();
        changed.people[0].name = "Silently replaced person".into();
        assert!(validate_with(&w, &changed).is_err());
        let mut cited = r.clone();
        cited.people[0]
            .sources
            .push("Additional corroboration".into());
        assert!(validate_with(&w, &cited).is_ok());
        let roundtrip: WorldState =
            serde_json::from_str(&serde_json::to_string(&w).unwrap()).unwrap();
        validate_with(&roundtrip, &r).unwrap();
    }
    #[test]
    fn published_catalogue_has_every_authored_party_and_explicit_gaps() {
        let r = roster().unwrap();
        validate_roster(r).unwrap();
        let expected = gov::POLITIES
            .iter()
            .chain(gov::D4_POLITIES)
            .flat_map(|p| p.parties.iter().map(move |s| (p.nation, s.id)))
            .collect::<BTreeSet<_>>();
        let actual = r
            .parties
            .iter()
            .map(|p| (p.nation, p.party.as_str()))
            .collect::<BTreeSet<_>>();
        assert_eq!(expected, actual);
        assert_eq!(actual.len(), 624);
        for p in &r.parties {
            if p.terms.is_empty() {
                // A researched organization can have a documented lifecycle but
                // no sufficiently dated office holder yet (for example PDS).
                assert_ne!(p.coverage, Coverage::Verified);
                if p.coverage == Coverage::Partial {
                    assert!(!p.sources.is_empty());
                    assert!(p.identity_note.as_ref().is_some_and(|n| !n.trim().is_empty()));
                }
                assert!(!p.gaps.is_empty());
            }
        }
    }
    #[test]
    fn canadian_ndp_reference_retains_mclaughlin_through_the_researched_handover() {
        let w = crate::init::world_1990(GameRules { ideology_blocs: true, ..Default::default() });
        let original = crate::save(&w);
        for (date, expected) in [("1994-04-15", "audrey_mclaughlin"),
            ("1995-10-13", "audrey_mclaughlin"), ("1995-10-14", "alexa_mcdonough")] {
            let view = reference_view(&w, NationId::Canada, date);
            let party = view["parties"].as_array().unwrap().iter().find(|p| p["party_id"] == "ca_ndp").unwrap();
            let historical = party["historical"].as_array().unwrap();
            assert_eq!(historical.len(), 1, "one researched NDP leader at {date}");
            assert_eq!(historical[0]["person"]["id"], expected);
            assert!(party["uncertain_historical"].as_array().unwrap().is_empty(),
                "the unresolved announced-resignation/interim label must not erase a continuously serving person");
        }
        assert_eq!(crate::save(&w), original);
    }
    #[test]
    fn default_off_is_sparse_and_reference_reads_do_not_change_world() {
        let mut w = crate::init::world_1990(GameRules::default());
        let before = crate::save(&w);
        ensure_all(&mut w);
        let _ = reference_view(&w, NationId::UK, "1990-01-01");
        assert_eq!(crate::save(&w), before);
        assert!(!before.contains("party_leadership"));
        assert_eq!(crate::save(&crate::load(&before).unwrap()), before);
        assert!(reference_view(&w, NationId::UK, "1900-02-29")
            .get("error")
            .is_some());
        let outside = reference_view(&w, NationId::UK, "2027-01-01");
        for p in outside["parties"].as_array().unwrap() {
            assert!(p["historical"].as_array().unwrap().is_empty());
            assert!(p["eligible"].as_array().unwrap().is_empty());
        }
    }
    #[test]
    fn campaign_envelope_roundtrip_and_downgrade_are_fail_closed() {
        let mut w = crate::init::world_1990(GameRules {
            ideology_blocs: true,
            ..Default::default()
        });
        enable_campaign(&mut w).unwrap();
        let text = crate::save(&w);
        let shape: Value = serde_json::from_str(&text).unwrap();
        assert_eq!(shape["format"], "spheres-party-leadership-save");
        assert_eq!(shape["equipment_version"], 0);
        assert_eq!(crate::save(&crate::load(&text).unwrap()), text);
        assert!(crate::load(&shape["world"].to_string()).is_err());
        let mut bad = shape.clone();
        bad["world"]
            .as_object_mut()
            .unwrap()
            .remove("party_leadership");
        assert!(crate::load(&bad.to_string()).is_err());
        let mut bad = shape.clone();
        bad["world"]["rules"]["historical_party_leadership"] = json!(false);
        assert!(crate::load(&bad.to_string()).is_err());
        let mut bad = shape.clone();
        bad["equipment_version"] = json!(5);
        assert!(crate::load(&bad.to_string()).is_err());
        let mut bad = shape;
        bad["world"]["party_leadership"]["roster_version"] = json!(999);
        assert!(crate::load(&bad.to_string()).is_err());
    }
    #[test]
    fn monthly_succession_and_reference_uncertainty_are_explicit() {
        let (mut w, mut r) = fixture();
        w.month = 2;
        w.day = 28;
        seat(&mut w, &r, gov::Succession::Death);
        let e = &w.party_leadership.as_ref().unwrap().executives[0];
        assert_eq!(date_label(e.since_day), "1990-02-01");
        assert_eq!(e.holder.selected_day, e.since_day);
        let o = w
            .leadership
            .as_ref()
            .unwrap()
            .iter()
            .find(|o| o.nation == NationId::UK)
            .unwrap();
        assert_eq!(o.emergent.as_ref().unwrap().since, date_label(e.since_day));
        validate_with(&w, &r).unwrap();
        r.parties[1].terms[0].until = DateBound::Year("1990".into());
        r.parties[1].terms[1].until = DateBound::Unknown;
        let view = view_with(&w, NationId::UK, 0, "1990-01-01", &r);
        let labour = view["parties"]
            .as_array()
            .unwrap()
            .iter()
            .find(|p| p["party_id"] == "uk_lab")
            .unwrap();
        assert_eq!(labour["historical"].as_array().unwrap().len(), 0);
        assert_eq!(labour["uncertain_historical"].as_array().unwrap().len(), 2);
        assert_eq!(labour["eligible"].as_array().unwrap().len(), 0);
    }
    #[test]
    fn erased_original_identity_and_rewritten_executive_dates_are_rejected() {
        let (mut w, r) = fixture();
        w.party_leadership
            .as_mut()
            .unwrap()
            .office_identities
            .clear();
        assert!(validate_with(&w, &r).is_err());
        let (mut w, r) = fixture();
        w.month = 2;
        seat(&mut w, &r, gov::Succession::Death);
        w.party_leadership.as_mut().unwrap().executives[0].since_day = 0;
        assert!(validate_with(&w, &r).is_err());
    }
    #[test]
    fn real_government_hook_and_named_successor_mortality_replay() {
        let mut w = crate::init::world_1990(GameRules {
            ideology_blocs: true,
            historical_party_leadership: true,
            ..Default::default()
        });
        let r = roster().unwrap();
        let opposition = party_in(r, NationId::UK, "uk_lab").unwrap();
        let leaders = opposition
            .terms
            .iter()
            .filter(|t| eligible(r, opposition, t, 0) && t.kind != TermKind::Candidate)
            .collect::<Vec<_>>();
        assert_eq!(
            leaders.len(),
            1,
            "researched fixture must contain an unambiguous 1990 Labour leader"
        );
        let target = leaders[0].person.clone();
        gov::seat_office(
            &mut w,
            NationId::UK,
            &gov::Succession::Election {
                leader: "uk_lab".into(),
            },
        );
        assert_eq!(executive_person(&w, NationId::UK).unwrap().id, target);
        let card = crate::blocs::leader(&w, NationId::UK).unwrap();
        assert_eq!(
            card.name.as_deref(),
            Some(person(&target).unwrap().name.as_str())
        );
        assert!(card.described.is_none());
        assert_eq!(card.since.as_deref(), Some("1990-01-01"));
        assert_eq!(card.party.as_deref(), Some("uk_lab"));
        validate_state(&w).unwrap();
        // This returns a previously seated party incumbent through the real
        // election hook; their independently sourced birthday supports the
        // mortality draw without inventing one for the Labour reference.
        gov::seat_office(
            &mut w,
            NationId::UK,
            &gov::Succession::Election {
                leader: "uk_con".into(),
            },
        );
        let target = executive_person(&w, NationId::UK).unwrap().id.clone();
        assert!(matches!(
            person(&target).unwrap().born,
            Some(DateBound::Day(_))
        ));
        // Put the already-bound incumbent beyond the certain-age end of the
        // existing hazard. This exercises that engine event, not historical
        // death-date scripting, and the reference roster supplies no successor.
        w.year = 2150;
        w.month = 1;
        w.day = 1;
        let mut resumed = crate::load(&crate::save(&w)).unwrap();
        crate::politics::mortality(&mut w);
        crate::politics::mortality(&mut resumed);
        assert_eq!(crate::save(&w), crate::save(&resumed));
        assert!(w
            .party_leadership
            .as_ref()
            .unwrap()
            .deaths
            .iter()
            .any(|d| d.person == target));
        assert!(executive_person(&w, NationId::UK).is_none());
        validate_state(&w).unwrap();
    }
    #[test]
    fn one_person_leading_multiple_components_is_one_executive_identity() {
        let (mut w, mut r) = fixture();
        w.party_leadership = None;
        let p = &mut r.parties[1];
        p.kind = PartyKind::Coalition;
        p.components = (2..4)
            .map(|i| Component {
                id: format!("organization_{i}"),
                name: format!("Test organization {i}"),
                sources: vec!["fixture".into()],
                founded: None,
                dissolved: None,
            })
            .collect();
        for (i, t) in p.terms.iter_mut().enumerate() {
            t.component = Some(format!("organization_{}", i + 2));
            t.person = "fixture_2".into();
        }
        validate_roster(&r).unwrap();
        ensure_with(&mut w, &r);
        seat(
            &mut w,
            &r,
            gov::Succession::Election {
                leader: "uk_lab".into(),
            },
        );
        assert_eq!(
            executive_id_with(&w, &r, NationId::UK).as_deref(),
            Some("fixture_2")
        );
        assert_eq!(
            w.party_leadership
                .as_ref()
                .unwrap()
                .assignments
                .iter()
                .filter(|a| a.nation == NationId::UK && a.party == "uk_lab")
                .flat_map(|a| &a.holders)
                .count(),
            2
        );
        validate_with(&w, &r).unwrap();
    }
    #[test]
    fn leader_card_keeps_original_role_and_unmapped_identity_distinctions() {
        let mut w = crate::init::world_1990(GameRules {
            ideology_blocs: true,
            ..Default::default()
        });
        let uk = crate::blocs::leader(&w, NationId::UK).unwrap();
        let usa = crate::blocs::leader(&w, NationId::USA).unwrap();
        enable_campaign(&mut w).unwrap();
        assert_eq!(crate::blocs::leader(&w, NationId::UK).unwrap(), uk);
        assert_eq!(crate::blocs::leader(&w, NationId::USA).unwrap(), usa);
        gov::seat_office(
            &mut w,
            NationId::UK,
            &gov::Succession::Election {
                leader: "uk_con".into(),
            },
        );
        assert_eq!(
            crate::blocs::leader(&w, NationId::UK).unwrap(),
            uk,
            "the same-party election keeps the actual original executive and office date"
        );
        gov::seat_office(
            &mut w,
            NationId::USA,
            &gov::Succession::Election {
                leader: "us_dem".into(),
            },
        );
        let changed = crate::blocs::leader(&w, NationId::USA).unwrap();
        assert!(changed.name.is_none());
        assert!(changed.described.is_some());
        assert!(
            executive_person(&w, NationId::USA).is_none(),
            "an unresearched office never borrows a party or national avatar identity"
        );
    }
}
