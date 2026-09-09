//! Reviewed future party seats use the existing collective-holder save shape.
//! Seat rules never replace an incumbent or infer a historical person's gender.
use super::{future, Assignment, CampaignLeadership, Holder, NationId, Party, PartyKind, Roster, TermKind};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{collections::BTreeSet, sync::OnceLock};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all="snake_case")]
pub enum PairRule { AtLeastOneAuthoredWoman, AuthoredWomanAndMan }

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Seats {
    pub nation:NationId,
    pub party:String,
    pub target_holders:usize,
    pub fictional_pair_rule:PairRule,
    pub authored_woman_candidates:Vec<String>,
    pub authored_man_candidates:Vec<String>,
    pub sources:Vec<String>,
    pub institutional_fact:String,
    pub gameplay_assumption:String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SeatPolicy {
    pub version:u32,
    pub reviewed_at:String,
    pub from:String,
    pub until_exclusive:String,
    pub parties:Vec<Seats>,
}

pub fn reviewed_pair(nation:NationId,party:&str)->bool {
    nation==NationId::Germany && matches!(party,"de_gruene"|"de_spd")
}
pub fn validate_policy(d:&SeatPolicy,r:&Roster)->Result<(),String> {
    if d.version!=1 || d.reviewed_at!=future::HISTORICAL_THROUGH
        || d.from!=future::FROM || d.until_exclusive!=future::UNTIL || d.parties.len()!=2 {
        return Err("Unsupported future leadership seat policy.".into());
    }
    let mut parties=BTreeSet::new();
    for p in &d.parties {
        if !reviewed_pair(p.nation,&p.party) || !parties.insert(p.party.as_str()) || p.target_holders!=2
            || p.sources.is_empty() || p.sources.iter().any(|s|!s.starts_with("https://"))
            || p.institutional_fact.trim().is_empty() || p.gameplay_assumption.trim().is_empty() {
            return Err("Invalid or unsourced future paired-leadership policy.".into());
        }
        let party=super::party_in(r,p.nation,&p.party).ok_or("Unknown future paired party.")?;
        if party.kind!=PartyKind::Party || !party.components.is_empty() {
            return Err("Future leadership seats must not become party components.".into());
        }
        if (p.party=="de_gruene" && p.fictional_pair_rule!=PairRule::AtLeastOneAuthoredWoman)
            || (p.party=="de_spd" && p.fictional_pair_rule!=PairRule::AuthoredWomanAndMan) {
            return Err("Changed reviewed fictional team assumption requires explicit review.".into());
        }
        let mut ids=BTreeSet::new();
        for list in [&p.authored_woman_candidates,&p.authored_man_candidates] {
            if list.len()!=2 {return Err("Each reviewed fictional qualification needs its two authored candidates.".into());}
            for id in list {
                let c=future::candidate(id).ok_or("Unknown authored seat candidate.")?;
                if c.nation!=p.nation || c.party!=p.party || c.component.is_some() || !ids.insert(id.as_str()) {
                    return Err("Invalid or duplicate fictional seat qualification.".into());
                }
            }
        }
    }
    Ok(())
}
pub fn policy()->Result<&'static SeatPolicy,&'static str> {
    static POLICY:OnceLock<Result<SeatPolicy,String>>=OnceLock::new();
    POLICY.get_or_init(||{
        let d:SeatPolicy=serde_json::from_str(include_str!("../data/future_party_leadership_seats.json"))
            .map_err(|e|format!("Invalid future leadership seat data: {e}"))?;
        validate_policy(&d,super::roster().map_err(str::to_owned)?)?;
        Ok(d)
    }).as_ref().map_err(String::as_str)
}
fn seats(nation:NationId,party:&str)->Option<&'static Seats> {
    policy().ok()?.parties.iter().find(|p|p.nation==nation&&p.party==party)
}
fn continuing_historical_single(p:&Party,holders:&[Holder])->bool {
    holders.len()==1 && future::candidate(&holders[0].person).is_none()
        && p.terms.iter().find(|t|t.id==holders[0].term&&t.person==holders[0].person)
            .is_some_and(|t|t.kind!=TermKind::CoLeader)
}

/// Return only additions for a real vacancy. Existing holders and their hashes
/// are never rewritten; the caller records the actual latest succession date.
pub(super) fn additions(p:&Party,a:&Assignment,at:i32,book:&CampaignLeadership)->Vec<Holder> {
    let Some(seats)=seats(p.nation,&p.party) else {return vec![]};
    if !future::eligible_on(at) || a.component.is_some() || !p.components.is_empty()
        || !future::component_eligible(p.nation,&p.party,None)
        || a.holders.len()>=seats.target_holders || continuing_historical_single(p,&a.holders) {
        return vec![];
    }
    let mut occupied=a.holders.iter().map(|h|h.person.as_str()).collect::<BTreeSet<_>>();
    let available=future::for_party(p.nation,&p.party).into_iter()
        .filter(|c|c.component.is_none() && !super::dead(book,&c.person.id)).collect::<Vec<_>>();
    let mut women=a.holders.iter().filter(|h|seats.authored_woman_candidates.contains(&h.person)).count();
    let mut men=a.holders.iter().filter(|h|seats.authored_man_candidates.contains(&h.person)).count();
    let mut selected=vec![];
    while a.holders.len()+selected.len()<seats.target_holders {
        // Existing real people are unclassified. Choosing a known fictional
        // woman beside an unknown holder avoids guessing from a name/portrait.
        let require_woman=women==0;
        let require_man=!require_woman && seats.fictional_pair_rule==PairRule::AuthoredWomanAndMan && men==0;
        let next=available.iter().copied().find(|c|!occupied.contains(c.person.id.as_str())
            && (!require_woman || seats.authored_woman_candidates.contains(&c.person.id))
            && (!require_man || seats.authored_man_candidates.contains(&c.person.id)));
        let Some(c)=next else {break}; // Qualification exhaustion leaves a vacancy.
        occupied.insert(c.person.id.as_str());
        women+=usize::from(seats.authored_woman_candidates.contains(&c.person.id));
        men+=usize::from(seats.authored_man_candidates.contains(&c.person.id));
        selected.push(future::holder(c,at));
    }
    selected
}

pub fn view(p:&Party,assignments:&[&Assignment])->Value {
    if !reviewed_pair(p.nation,&p.party) {return Value::Null;}
    let Some(seats)=seats(p.nation,&p.party) else {
        return json!({"target_holders":2,"status":"policy_unavailable","selection":"vacancies_only"});
    };
    let holders=assignments.iter().flat_map(|a|a.holders.iter()).cloned().collect::<Vec<_>>();
    let historical_single=continuing_historical_single(p,&holders);
    let unclassified=holders.iter().filter(|h|future::candidate(&h.person).is_none()).count();
    json!({"version":1,"target_holders":seats.target_holders,"current_holders":holders.len(),
        "vacancies":if historical_single{0}else{seats.target_holders.saturating_sub(holders.len())},
        "historical_single_chair_retained":historical_single,"unclassified_historical_holders":unclassified,
        "selection":"actual_future_vacancies_only","existing_holders_preserved":true,
        "from":future::FROM,"until_exclusive":future::UNTIL,"fictional_pair_rule":seats.fictional_pair_rule,
        "sources":seats.sources,"institutional_fact":seats.institutional_fact,"gameplay_assumption":seats.gameplay_assumption,
        "qualification_note":if unclassified>0{"Historical incumbent qualifications are not inferred; mixed-team quota eligibility is not claimed as fully researched."}
            else{"Future qualifications are separately authored and never derived from a portrait or name."}})
}
