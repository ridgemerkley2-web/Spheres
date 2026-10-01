//! Non-party Tonga offices. Historical observations never schedule appointments.
//! The elected premier is separate from the crown; fictional people have no
//! hereditary eligibility. Player recommendations are an explicit arcade model.
use crate::{
    government::Succession,
    party_leadership::{self, Person},
    world::{NationId, WorldState},
    Command,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{collections::BTreeSet, sync::OnceLock};

pub const FIRST: &str = "2026-09-08";
pub const UNTIL: &str = "2036-01-01";

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Candidate {
    pub person: Person,
    pub nation: NationId,
    pub origin: String,
    pub role: String,
    pub source_draft: String,
    pub appearance_seed: String,
    pub presentation: String,
    pub fictional_biography: String,
    pub eligible_from: String,
    pub eligible_until_exclusive: String,
    pub sources: Vec<String>,
    pub restriction: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Opening {
    pub king_person_id: String,
    pub king_since: String,
    pub prime_minister_person_id: String,
    pub heir_person_id: String,
    pub heir_name: String,
    pub heir_since: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Catalogue {
    pub version: u32,
    pub nation: NationId,
    pub historical_reference_through: String,
    pub fictional_from: String,
    pub fictional_until_exclusive: String,
    pub opening: Opening,
    pub institution_sources: Vec<String>,
    pub gameplay_assumption: String,
    pub historical_bindings: Vec<Value>,
    pub future_candidates: Vec<Candidate>,
}
pub fn catalogue() -> &'static Catalogue {
    static DATA: OnceLock<Catalogue> = OnceLock::new();
    DATA.get_or_init(|| {
        let d: Catalogue =
            serde_json::from_str(include_str!("../data/tonga_institutional_leadership.json"))
                .expect("validated Tonga institutional catalogue");
        assert_eq!((d.version, d.nation), (1, NationId::Tonga));
        assert_eq!(d.historical_reference_through, "2026-09-07");
        assert_eq!(
            (
                d.fictional_from.as_str(),
                d.fictional_until_exclusive.as_str()
            ),
            (FIRST, UNTIL)
        );
        let mut ids = BTreeSet::new();
        for c in &d.future_candidates {
            assert!(ids.insert(&c.person.id) && c.person.id.starts_with("fictional_to_"));
            assert!(
                c.person.sources.is_empty()
                    && c.nation == NationId::Tonga
                    && c.origin == "fictional_successor"
            );
            assert_eq!(
                (
                    c.eligible_from.as_str(),
                    c.eligible_until_exclusive.as_str()
                ),
                (FIRST, UNTIL)
            );
            assert!(matches!(
                c.role.as_str(),
                "peoples_representative"
                    | "prime_minister"
                    | "nonelected_minister"
                    | "party_organizer"
            ));
            assert!(
                c.appearance_seed.len() == 16
                    && c.appearance_seed.bytes().all(|b| b.is_ascii_hexdigit())
            );
            assert!(
                !c.sources.is_empty()
                    && !c.fictional_biography.is_empty()
                    && !c.restriction.is_empty()
            );
        }
        d
    })
}
pub fn candidate(id: &str) -> Option<&'static Candidate> {
    catalogue()
        .future_candidates
        .iter()
        .find(|c| c.person.id == id)
}
pub fn person(id: &str) -> Option<&'static Person> {
    candidate(id).map(|c| &c.person)
}
pub fn person_view(id: &str) -> Value {
    if let Some(c) = candidate(id) {
        let mut value = json!(c.person);
        value["fiction"] = json!({"origin":c.origin,"role":c.role,"appearance_seed":c.appearance_seed,
            "fictional_biography":c.fictional_biography,"restriction":c.restriction,
            "eligible_from":FIRST,"eligible_until_exclusive":UNTIL});
        value
    } else {
        party_leadership::roster()
            .ok()
            .and_then(|r| r.people.iter().find(|p| p.id == id))
            .map_or(Value::Null, |p| json!(p))
    }
}
/// Same flat portrait catalogue shape; no fake party ID is attached.
pub fn portrait_catalogue() -> Vec<Value> {
    catalogue()
        .future_candidates
        .iter()
        .map(|c| {
            json!({"person_id":c.person.id,"name":c.person.name,
        "nation":c.nation,"origin":c.origin,"role":c.role,"party":Value::Null,
        "institution":"tonga_civilian_institutions","component":Value::Null,
        "appearance_seed":c.appearance_seed,"presentation":c.presentation,
        "fictional_biography":c.fictional_biography,"eligible_from":FIRST,
        "eligible_until_exclusive":UNTIL,"restriction":c.restriction})
        })
        .collect()
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum Action {
    Reform,
    HoldElection,
    RecommendPrimeMinister { person_id: String },
    AppointMinister { person_id: String },
    Vacate { office: String, person_id: String },
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Appointment {
    pub person_id: String,
    pub selected_day: i32,
    pub reason: String,
    pub identity_hash: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Event {
    pub day: i32,
    pub action: Action,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CrownIdentity {
    pub person_id: String,
    pub since: String,
    pub identity_hash: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub version: u32,
    pub initialized_day: i32,
    pub opening_prime_minister: bool,
    pub reformed_on: Option<i32>,
    pub election_day: Option<i32>,
    pub recommendation_due: bool,
    pub representatives: Vec<Appointment>,
    pub prime_minister: Option<Appointment>,
    pub ministers: Vec<Appointment>,
    pub events: Vec<Event>,
    pub inherited_monarch: Option<CrownIdentity>,
}
fn day(s: &str) -> Option<i32> {
    let (y, m, d) = crate::data::parse_date(s)?;
    if format!("{y:04}-{m:02}-{d:02}") != s
        || !(1..=12).contains(&m)
        || d == 0
        || d > crate::world::days_in_month(y, m)
    {
        return None;
    }
    Some(crate::clock::date_day(y, m, d))
}
fn event_day(w: &WorldState) -> u32 {
    if crate::clock::is_daily(w) {
        w.day.max(1)
    } else {
        1
    }
}
fn today(w: &WorldState) -> i32 {
    crate::clock::date_day(w.year, w.month, event_day(w))
}
fn label(w: &WorldState) -> String {
    format!("{:04}-{:02}-{:02}", w.year, w.month, event_day(w))
}
fn in_future(at: i32) -> bool {
    (day(FIRST).unwrap()..day(UNTIL).unwrap()).contains(&at)
}
fn hash(value: Value) -> String {
    let h = value
        .to_string()
        .bytes()
        .fold(0xcbf29ce484222325u64, |h, b| {
            (h ^ u64::from(b)).wrapping_mul(0x100000001b3)
        });
    format!("{h:016x}")
}
fn identity_hash(id: &str) -> String {
    if let Some(c) = candidate(id) {
        hash(json!([c.person, c.role, c.appearance_seed, FIRST, UNTIL]))
    } else {
        party_leadership::roster()
            .ok()
            .and_then(|r| r.people.iter().find(|p| p.id == id))
            .map_or_else(String::new, |p| {
                hash(json!([p.id, p.name, p.native, p.born, p.died]))
            })
    }
}
fn appointment(id: &str, at: i32, reason: &str) -> Appointment {
    Appointment {
        person_id: id.into(),
        selected_day: at,
        reason: reason.into(),
        identity_hash: identity_hash(id),
    }
}
fn crown_active(w: &WorldState) -> bool {
    crate::blocs::leader_row(w, NationId::Tonga).is_some_and(|o| {
        o.emergent
            .as_ref()
            .map_or(o.office == "King", |e| e.office == "King")
    })
}
fn opening_pm_present(w: &WorldState) -> bool {
    let d = &catalogue().opening;
    let Some(p) = party_leadership::roster()
        .ok()
        .and_then(|r| r.people.iter().find(|p| p.id == d.prime_minister_person_id))
    else {
        return false;
    };
    crate::blocs::leader_row(w, NationId::Tonga).is_some_and(|o| {
        o.emergent.is_none()
            && o.since == d.king_since
            && o.also
                .iter()
                .any(|a| a.name == p.name && a.office == "Prime Minister")
    })
}
fn initial(at: i32, opening_pm: bool) -> State {
    State {
        version: 1,
        initialized_day: at,
        opening_prime_minister: opening_pm,
        reformed_on: None,
        election_day: None,
        recommendation_due: false,
        representatives: vec![],
        prime_minister: opening_pm.then(|| {
            appointment(
                &catalogue().opening.prime_minister_person_id,
                at,
                "opening_reference",
            )
        }),
        ministers: vec![],
        events: vec![],
        inherited_monarch: None,
    }
}
fn state(w: &WorldState) -> State {
    w.institutional_leadership
        .clone()
        .unwrap_or_else(|| initial(today(w), opening_pm_present(w)))
}
fn local_refusal(s: &State, a: &Action, at: i32) -> Option<String> {
    let no = |message: &str| Some(message.into());
    if at < s.initialized_day || s.events.last().is_some_and(|e| e.day > at) {
        return no("Institutional event dates cannot move backwards.");
    }
    if matches!(a, Action::Reform) {
        return s
            .reformed_on
            .map(|_| "The separate Assembly model is already active.".into());
    }
    if s.reformed_on.is_none() {
        return no("Adopt the Assembly reform before changing its offices.");
    }
    match a {
        Action::Reform => None,
        Action::HoldElection => {
            if !in_future(at) {
                no("No reviewed election candidate pool is available on this date; historical appointments are not invented.")
            } else if s.election_day == Some(at) {
                no("An Assembly election has already been held today.")
            } else {
                None
            }
        }
        Action::RecommendPrimeMinister { person_id } => {
            if !in_future(at) {
                no("New fictional appointments are available only from 8 September 2026 through 2035.")
            } else if !s.recommendation_due {
                no("A new election or actual premiership vacancy is required.")
            } else if !candidate(person_id).is_some_and(|c| c.role == "prime_minister") {
                no("This person has no reviewed independent-premier eligibility.")
            } else if !s.representatives.iter().any(|a| a.person_id == *person_id) {
                no("The nominee must first hold a people's seat won in this campaign.")
            } else {
                None
            }
        }
        Action::AppointMinister { person_id } => {
            if !in_future(at) {
                no("New fictional appointments are available only from 8 September 2026 through 2035.")
            } else if s.prime_minister.is_none() || s.recommendation_due {
                no("A confirmed Prime Minister must nominate the minister.")
            } else if !candidate(person_id).is_some_and(|c| c.role == "nonelected_minister") {
                no("This person has no reviewed non-elected Cabinet eligibility.")
            } else if s.ministers.len() >= 4 {
                no("No more than four non-elected ministers may serve together.")
            } else if s.ministers.iter().any(|a| a.person_id == *person_id) {
                no("This person already holds a Cabinet appointment.")
            } else {
                None
            }
        }
        Action::Vacate { office, person_id } => {
            let exists = match office.as_str() {
                "prime_minister" => s
                    .prime_minister
                    .as_ref()
                    .is_some_and(|a| a.person_id == *person_id),
                "nonelected_minister" => s.ministers.iter().any(|a| a.person_id == *person_id),
                "peoples_representative" => {
                    s.representatives.iter().any(|a| a.person_id == *person_id)
                }
                _ => false,
            };
            if exists {
                None
            } else {
                no("Only an existing civilian appointment can be vacated here; the Crown and hereditary offices are separate.")
            }
        }
    }
}
fn reduce(s: &mut State, a: &Action, at: i32) -> Result<(), String> {
    if let Some(why) = local_refusal(s, a, at) {
        return Err(why);
    }
    match a {
        Action::Reform => s.reformed_on = Some(at),
        Action::HoldElection => {
            s.election_day = Some(at);
            s.representatives = catalogue()
                .future_candidates
                .iter()
                .filter(|c| matches!(c.role.as_str(), "peoples_representative" | "prime_minister"))
                .map(|c| appointment(&c.person.id, at, "assembly_election"))
                .collect();
            s.recommendation_due = true;
        }
        Action::RecommendPrimeMinister { person_id } => {
            s.prime_minister = Some(appointment(
                person_id,
                at,
                "assembly_recommendation_and_royal_appointment",
            ));
            s.recommendation_due = false;
        }
        Action::AppointMinister { person_id } => s.ministers.push(appointment(
            person_id,
            at,
            "premier_nomination_and_royal_appointment",
        )),
        Action::Vacate { office, person_id } => match office.as_str() {
            "prime_minister" => {
                s.prime_minister = None;
                s.recommendation_due = true
            }
            "nonelected_minister" => s.ministers.retain(|a| a.person_id != *person_id),
            "peoples_representative" => {
                s.representatives.retain(|a| a.person_id != *person_id);
                if s.prime_minister
                    .as_ref()
                    .is_some_and(|a| a.person_id == *person_id)
                {
                    s.prime_minister = None;
                    s.recommendation_due = true
                }
            }
            _ => unreachable!(),
        },
    }
    s.events.push(Event {
        day: at,
        action: a.clone(),
    });
    Ok(())
}
pub fn price(a: &Action) -> f64 {
    match a {
        Action::Reform => 35.0,
        Action::HoldElection => 25.0,
        Action::RecommendPrimeMinister { .. } => 12.0,
        Action::AppointMinister { .. } => 10.0,
        Action::Vacate { .. } => 0.0,
    }
}
pub fn refusal(w: &WorldState, n: NationId, a: &Action) -> Option<String> {
    if n != NationId::Tonga {
        return Some("This institutional model is specific to Tonga.".into());
    }
    if !w.rules.ideology_blocs
        || !w.rules.historical_party_leadership
        || w.party_leadership.is_none()
    {
        return Some("Enable campaign leadership before using these institutions.".into());
    }
    if !w.nation_opt(n).is_some_and(|n| n.alive) {
        return Some("This country no longer exists.".into());
    }
    if !crown_active(w) {
        return Some("This campaign no longer has the constitutional Crown required by these appointment rules.".into());
    }
    local_refusal(&state(w), a, today(w))
}
pub fn apply(w: &mut WorldState, n: NationId, a: &Action) -> Result<(), String> {
    if let Some(why) = refusal(w, n, a) {
        return Err(why);
    }
    let mut next = state(w);
    reduce(&mut next, a, today(w))?;
    w.institutional_leadership = Some(next);
    Ok(())
}

/// Called before the engine consumes the *authored* opening heir. No lookup by
/// a new name, no inferred second heir, and no historical accession schedule.
pub(crate) fn on_crown_succession(
    w: &mut WorldState,
    n: NationId,
    heir_used: bool,
    how: &Succession,
) {
    if n != NationId::Tonga || !w.rules.historical_party_leadership {
        return;
    }
    let d = &catalogue().opening;
    let verified = heir_used
        && crate::blocs::leader_row(w, n).is_some_and(|o| {
            o.emergent.is_none()
                && o.since == d.king_since
                && o.office == "King"
                && o.heir
                    .as_ref()
                    .is_some_and(|h| h.name == d.heir_name && h.since == d.heir_since)
        });
    if verified {
        let mut s = state(w);
        s.inherited_monarch = Some(CrownIdentity {
            person_id: d.heir_person_id.clone(),
            since: label(w),
            identity_hash: identity_hash(&d.heir_person_id),
        });
        w.institutional_leadership = Some(s);
    } else if let Some(s) = w.institutional_leadership.as_mut() {
        // Any later actual office replacement ends this exact inherited link.
        s.inherited_monarch = None;
    }
    let _ = how; // The existing engine decides whether and why the heir is seated.
}
pub fn monarch(w: &WorldState, n: NationId) -> Option<&'static Person> {
    if n != NationId::Tonga || !w.rules.historical_party_leadership {
        return None;
    }
    let b = w
        .institutional_leadership
        .as_ref()?
        .inherited_monarch
        .as_ref()?;
    let row = crate::blocs::leader_row(w, n)?;
    if !row.emergent.as_ref().is_some_and(|e| {
        e.office == "King" && e.since == b.since && e.described == catalogue().opening.heir_name
    }) {
        return None;
    }
    party_leadership::roster()
        .ok()?
        .people
        .iter()
        .find(|p| p.id == b.person_id)
}
pub fn validate(w: &WorldState) -> Result<(), String> {
    let Some(s) = &w.institutional_leadership else {
        return Ok(());
    };
    if s.version != 1
        || !w.rules.historical_party_leadership
        || !w.rules.ideology_blocs
        || s.initialized_day < day("1990-01-01").unwrap()
        || s.initialized_day > today(w)
    {
        return Err("Invalid institutional leadership version, date or rules.".into());
    }
    let mut replay = initial(s.initialized_day, s.opening_prime_minister);
    for e in &s.events {
        if e.day > today(w) {
            return Err("Institutional event lies in the future.".into());
        }
        reduce(&mut replay, &e.action, e.day)?;
    }
    replay.inherited_monarch = s.inherited_monarch.clone();
    if &replay != s {
        return Err(
            "Institutional appointments do not match their event journal or exact identity hashes."
                .into(),
        );
    }
    if let Some(b) = &s.inherited_monarch {
        if b.person_id != catalogue().opening.heir_person_id
            || b.identity_hash != identity_hash(&b.person_id)
            || day(&b.since).is_none_or(|d| d < s.initialized_day || d > today(w))
            || monarch(w, NationId::Tonga).is_none()
        {
            return Err(
                "Invalid inherited Crown identity; an exact saved appointment is required.".into(),
            );
        }
    }
    Ok(())
}
fn action_view(w: &WorldState, action: Action) -> Value {
    let c = Command::TongaInstitutions {
        nation: NationId::Tonga,
        action: action.clone(),
    };
    json!({"command":{"kind":"tonga_institutions","action":action},"price_pc":price(&action),"refusal":crate::refusal_of(w,&c)})
}
pub fn view(w: &WorldState, n: NationId) -> Value {
    if n != NationId::Tonga {
        return Value::Null;
    }
    let s = state(w);
    let office = |a: &Appointment| {
        let (year, month, day) = crate::clock::date_from_day(a.selected_day);
        let mut record = json!(a);
        // Initializing a reference record does not establish an appointment date.
        record["selected_on"] = if a.reason == "opening_reference" {
            Value::Null
        } else {
            json!(format!("{year:04}-{month:02}-{day:02}"))
        };
        json!({"person":person_view(&a.person_id),"appointment":record,
            "opening_reference":a.reason=="opening_reference"})
    };
    let active = crown_active(w);
    let assembly_active = active && s.reformed_on.is_some();
    let candidates=catalogue().future_candidates.iter().map(|c|{
        let actions=match c.role.as_str() {
            "prime_minister"=>vec![action_view(w,Action::RecommendPrimeMinister{person_id:c.person.id.clone()})],
            "nonelected_minister"=>vec![action_view(w,Action::AppointMinister{person_id:c.person.id.clone()})],_=>vec![],
        };
        json!({"person":person_view(&c.person.id),"role":c.role,"origin":c.origin,"restriction":c.restriction,
            "eligible_by_date":in_future(today(w)),"actions":actions})
    }).collect::<Vec<_>>();
    let mut vacancies = vec![];
    for (name, items) in [
        ("peoples_representative", s.representatives.as_slice()),
        ("nonelected_minister", s.ministers.as_slice()),
    ] {
        for a in items {
            vacancies.push(action_view(
                w,
                Action::Vacate {
                    office: name.into(),
                    person_id: a.person_id.clone(),
                },
            ));
        }
    }
    if let Some(a) = &s.prime_minister {
        vacancies.push(action_view(
            w,
            Action::Vacate {
                office: "prime_minister".into(),
                person_id: a.person_id.clone(),
            },
        ));
    }
    json!({"nation":n,"date":label(w),"version":1,"active":active,"reformed":s.reformed_on.is_some(),
        "monarch":crate::blocs::leader(w,n),"prime_minister":if active{s.prime_minister.as_ref().map(office)}else{None},
        "people_representatives":if active{s.representatives.iter().map(office).collect::<Vec<_>>()}else{vec![]},
        "nonelected_ministers":if active{s.ministers.iter().map(office).collect::<Vec<_>>()}else{vec![]},
        "peoples_seats":assembly_active.then_some(17),"nobles_seats":assembly_active.then_some(9),
        "unnamed_peoples_seats":assembly_active.then_some(17usize.saturating_sub(s.representatives.len())),
        "proposed_model_seats":{"peoples":17,"nobles":9},
        "nobles_eligibility":"Hereditary eligibility; no invented candidates.",
        "recommendation_due":active && s.recommendation_due,"events":s.events,
        "future_preview":candidates,"historical_bindings":catalogue().historical_bindings,
        "actions":{"reform":action_view(w,Action::Reform),"hold_election":action_view(w,Action::HoldElection),"vacancies":vacancies},
        "historical_reference_through":"2026-09-07","fictional_from":FIRST,"fictional_until_exclusive":UNTIL,
        "institution_sources":catalogue().institution_sources,"gameplay_assumption":catalogue().gameplay_assumption,
        "notice":"Historical references and missing dates do not appoint a campaign holder. Future cards are fictional previews. The separate premier never replaces the monarch."})
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::government;
    fn world() -> WorldState {
        let mut w = crate::init::world_1990(crate::world::GameRules {
            ideology_blocs: true,
            historical_party_leadership: true,
            daily_simulation: true,
            ..Default::default()
        });
        party_leadership::ensure_all(&mut w);
        w.nation_mut(NationId::Tonga).political_capital = 500.0;
        w
    }
    fn act(w: &mut WorldState, a: Action) -> Result<(), String> {
        crate::apply_command(
            w,
            &Command::TongaInstitutions {
                nation: NationId::Tonga,
                action: a,
            },
        )
    }
    fn future(w: &mut WorldState) {
        w.year = 2026;
        w.month = 9;
        w.day = 8;
    }
    #[test]
    fn dates_and_read_only_views_never_seat_a_candidate() {
        let mut w = world();
        let before = crate::save(&w);
        view(&w, NationId::Tonga);
        assert_eq!(crate::save(&w), before);
        assert!(view(&w, NationId::Tonga)["peoples_seats"].is_null());
        assert!(view(&w, NationId::Tonga)["nobles_seats"].is_null());
        assert!(
            view(&w, NationId::Tonga)["prime_minister"]["appointment"]["selected_on"].is_null()
        );
        future(&mut w);
        view(&w, NationId::Tonga);
        assert!(w.institutional_leadership.is_none());
        assert!(act(&mut w, Action::HoldElection).is_err());
    }
    #[test]
    fn election_recommendation_and_minister_are_separate_from_crown() {
        let mut w = world();
        let crown = crate::blocs::leader(&w, NationId::Tonga).unwrap().name;
        act(&mut w, Action::Reform).unwrap();
        assert_eq!(view(&w, NationId::Tonga)["peoples_seats"], 17);
        assert!(act(&mut w, Action::HoldElection).is_err());
        future(&mut w);
        act(&mut w, Action::HoldElection).unwrap();
        let pm = "fictional_to_sitani_lolohea";
        act(
            &mut w,
            Action::RecommendPrimeMinister {
                person_id: pm.into(),
            },
        )
        .unwrap();
        act(
            &mut w,
            Action::AppointMinister {
                person_id: "fictional_to_pisila_tukuafu".into(),
            },
        )
        .unwrap();
        assert_eq!(
            w.institutional_leadership
                .as_ref()
                .unwrap()
                .prime_minister
                .as_ref()
                .unwrap()
                .person_id,
            pm
        );
        assert_eq!(
            crate::blocs::leader(&w, NationId::Tonga).unwrap().name,
            crown
        );
        assert!(act(
            &mut w,
            Action::RecommendPrimeMinister {
                person_id: pm.into()
            }
        )
        .is_err());
        validate(&w).unwrap();
        let resumed = crate::load(&crate::save(&w)).unwrap();
        assert_eq!(w.institutional_leadership, resumed.institutional_leadership);
    }
    #[test]
    fn unreviewed_party_office_and_hereditary_people_cannot_get_civilian_roles() {
        let mut w = world();
        future(&mut w);
        act(&mut w, Action::Reform).unwrap();
        act(&mut w, Action::HoldElection).unwrap();
        for id in [
            "fictional_to_kalolo_matalehu",
            "taufaahau_tupou_iv",
            "fictional_to_lesieli_fotu",
        ] {
            let before = crate::save(&w);
            assert!(act(
                &mut w,
                Action::RecommendPrimeMinister {
                    person_id: id.into()
                }
            )
            .is_err());
            assert_eq!(crate::save(&w), before);
        }
        assert!(act(
            &mut w,
            Action::Vacate {
                office: "king".into(),
                person_id: "taufaahau_tupou_iv".into()
            }
        )
        .is_err());
    }
    #[test]
    fn authored_heir_survives_actual_succession_and_reload() {
        let mut w = world();
        government::seat_office(&mut w, NationId::Tonga, &Succession::Death);
        assert_eq!(
            monarch(&w, NationId::Tonga).unwrap().id,
            "siaosi_taufaahau_manumataongo"
        );
        assert_eq!(
            party_leadership::executive_person(&w, NationId::Tonga)
                .unwrap()
                .id,
            "siaosi_taufaahau_manumataongo"
        );
        validate(&w).unwrap();
        let mut resumed = crate::load(&crate::save(&w)).unwrap();
        assert_eq!(
            monarch(&resumed, NationId::Tonga).unwrap().id,
            "siaosi_taufaahau_manumataongo"
        );
        government::seat_office(&mut resumed, NationId::Tonga, &Succession::Death);
        assert!(monarch(&resumed, NationId::Tonga).is_none());
    }
    #[test]
    fn saves_reject_unjournalled_appointments_and_keep_old_absence() {
        let mut w = world();
        assert!(w.institutional_leadership.is_none());
        let old = crate::save(&w);
        assert!(crate::load(&old)
            .unwrap()
            .institutional_leadership
            .is_none());
        future(&mut w);
        act(&mut w, Action::Reform).unwrap();
        act(&mut w, Action::HoldElection).unwrap();
        w.institutional_leadership.as_mut().unwrap().representatives[0].person_id =
            "fictional_to_kalolo_matalehu".into();
        assert!(crate::load(&crate::save(&w)).is_err());
    }
    #[test]
    fn after_endpoint_no_new_fiction_but_saved_incumbents_continue() {
        let mut w = world();
        future(&mut w);
        act(&mut w, Action::Reform).unwrap();
        act(&mut w, Action::HoldElection).unwrap();
        act(
            &mut w,
            Action::RecommendPrimeMinister {
                person_id: "fictional_to_sitani_lolohea".into(),
            },
        )
        .unwrap();
        let pm = w
            .institutional_leadership
            .as_ref()
            .unwrap()
            .prime_minister
            .clone();
        w.year = 2036;
        w.month = 1;
        w.day = 1;
        assert!(act(&mut w, Action::HoldElection).is_err());
        assert_eq!(
            w.institutional_leadership.as_ref().unwrap().prime_minister,
            pm
        );
        validate(&w).unwrap();
    }

    #[test]
    fn reference_cutoff_foreign_actor_and_refused_costs_are_exact() {
        let mut w = world();
        act(&mut w, Action::Reform).unwrap();
        w.year = 2026;
        w.month = 9;
        w.day = 7;
        let saved = crate::save(&w);
        assert!(act(&mut w, Action::HoldElection).is_err());
        assert_eq!(crate::save(&w), saved);
        assert!(crate::apply_command(
            &mut w,
            &Command::TongaInstitutions {
                nation: NationId::France,
                action: Action::Reform
            }
        )
        .is_err());
        assert_eq!(crate::save(&w), saved);
        w.day = 8;
        let pc = w.nation(NationId::Tonga).political_capital;
        act(&mut w, Action::HoldElection).unwrap();
        assert_eq!(w.nation(NationId::Tonga).political_capital, pc - 25.0);
    }

    #[test]
    fn real_vacancy_reopens_recommendation_and_ministers_remain_caretakers() {
        let mut w = world();
        future(&mut w);
        act(&mut w, Action::Reform).unwrap();
        act(&mut w, Action::HoldElection).unwrap();
        let pid = "fictional_to_sitani_lolohea";
        act(
            &mut w,
            Action::RecommendPrimeMinister {
                person_id: pid.into(),
            },
        )
        .unwrap();
        act(
            &mut w,
            Action::AppointMinister {
                person_id: "fictional_to_pisila_tukuafu".into(),
            },
        )
        .unwrap();
        act(
            &mut w,
            Action::Vacate {
                office: "prime_minister".into(),
                person_id: pid.into(),
            },
        )
        .unwrap();
        assert!(w
            .institutional_leadership
            .as_ref()
            .unwrap()
            .prime_minister
            .is_none());
        assert_eq!(
            w.institutional_leadership.as_ref().unwrap().ministers.len(),
            1
        );
        act(
            &mut w,
            Action::RecommendPrimeMinister {
                person_id: pid.into(),
            },
        )
        .unwrap();
        validate(&w).unwrap();
    }

    #[test]
    fn cabinet_limit_and_saved_identity_tampering_fail_closed() {
        let mut w = world();
        future(&mut w);
        act(&mut w, Action::Reform).unwrap();
        act(&mut w, Action::HoldElection).unwrap();
        act(
            &mut w,
            Action::RecommendPrimeMinister {
                person_id: "fictional_to_sitani_lolohea".into(),
            },
        )
        .unwrap();
        let mut s = w.institutional_leadership.clone().unwrap();
        // Synthetic invariant case: a future larger pool still cannot add a fifth.
        s.ministers = (0..4)
            .map(|_| appointment("fictional_to_pisila_tukuafu", today(&w), "fixture"))
            .collect();
        assert!(local_refusal(
            &s,
            &Action::AppointMinister {
                person_id: "fictional_to_pisila_tukuafu".into()
            },
            today(&w)
        )
        .unwrap()
        .contains("four"));
        w.institutional_leadership
            .as_mut()
            .unwrap()
            .prime_minister
            .as_mut()
            .unwrap()
            .identity_hash = "changed".into();
        assert!(crate::load(&crate::save(&w)).is_err());
    }

    #[test]
    fn a_takeover_does_not_inherit_the_crowns_civilian_appointment_authority() {
        let mut w = world();
        act(&mut w, Action::Reform).unwrap();
        future(&mut w);
        act(&mut w, Action::HoldElection).unwrap();
        assert_eq!(view(&w, NationId::Tonga)["recommendation_due"], true);
        government::seat_office(
            &mut w,
            NationId::Tonga,
            &Succession::Coup {
                pillar: government::Pillar::Army,
            },
        );
        assert!(refusal(&w, NationId::Tonga, &Action::HoldElection)
            .unwrap()
            .contains("Crown"));
        assert_eq!(view(&w, NationId::Tonga)["prime_minister"], Value::Null);
        assert!(view(&w, NationId::Tonga)["peoples_seats"].is_null());
        assert_eq!(view(&w, NationId::Tonga)["recommendation_due"], false);
        validate(&w).unwrap();
    }
}
