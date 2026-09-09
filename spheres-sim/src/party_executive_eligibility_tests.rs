//! Role integration fixtures load staged research without importing runtime facts.
use super::*;
use executive_eligibility::{Basis, Grant, OfficeRole, Policy};

fn staged_roster() -> Roster {
    let mut r = roster().unwrap().clone();
    for text in [include_str!("../../docs/research/usa-party-roles-1990-starter.json"),
        include_str!("../../docs/research/france-party-leaders-1990-2026.json")]
    {
        let staged: Value = serde_json::from_str(text).unwrap();
        for person in staged["people"].as_array().unwrap() {
            let p: Person = serde_json::from_value(person.clone()).unwrap();
            if !r.people.iter().any(|old|old.id==p.id) { r.people.push(p); }
        }
        for party in staged["parties"].as_array().unwrap() {
            let p: Party = serde_json::from_value(party.clone()).unwrap();
            let index=r.parties.iter().position(|old|old.nation==p.nation && old.party==p.party).unwrap();
            r.parties[index] = p;
        }
    }
    validate_roster(&r).unwrap();
    r
}
fn campaign(r: &Roster, date: &str) -> WorldState {
    let mut w = crate::init::world_1990(GameRules {
        ideology_blocs:true,historical_party_leadership:true,daily_simulation:true,..Default::default()
    });
    (w.year,w.month,w.day)=strict_date(date).unwrap();
    w.party_leadership = None;
    ensure_with(&mut w,r);
    validate_with(&w,r).unwrap();
    w
}
fn seat(w: &mut WorldState, r: &Roster, policy: &Policy, nation: NationId, how: gov::Succession) {
    let (next,_) = gov::succession_seat(w,nation,&how).expect("fixture must cause a real seating event");
    on_succession_with_policy(w,r,Some(policy),nation,&how,next.party.as_deref());
    let o = w.leadership.as_mut().unwrap().iter_mut().find(|o|o.nation==nation).unwrap();
    o.name=None; o.native=None; o.tie=None; o.bloc_override=None; o.must_leave_by=None; o.also.clear();
    o.emergent=Some(next);
    validate_with(w,r).unwrap();
}
fn party_holder<'a>(w: &'a WorldState,n: NationId,party: &str)-> &'a Holder {
    &w.party_leadership.as_ref().unwrap().assignments.iter()
        .find(|a|a.nation==n&&a.party==party&&!a.holders.is_empty()).unwrap().holders[0]
}

#[test]
fn staged_us_chairs_and_french_party_officers_render_without_becoming_presidents() {
    let r=staged_roster();
    let policy=executive_eligibility::policy().unwrap();
    let cases=[
        (NationId::USA,"us_dem","ron_brown",gov::Succession::Election{leader:"us_dem".into()}),
        (NationId::USA,"us_rep","lee_atwater",gov::Succession::Death),
        (NationId::France,"fr_ps","pierre_mauroy",gov::Succession::Death),
        (NationId::France,"fr_rpr","jacques_chirac",gov::Succession::Election{leader:"fr_rpr".into()}),
    ];
    for (nation,party,chair,event) in cases {
        let mut w=campaign(&r,"1990-01-01");
        assert_ne!(executive_id_with(&w,&r,nation).as_deref(),Some(chair));
        assert_eq!(party_holder(&w,nation,party).person,chair);
        let original=executive_id_with(&w,&r,nation).unwrap();
        let before=serde_json::to_value(&w).unwrap();
        let display=view_with(&w,nation,0,"1990-01-01",&r);
        let row=display["parties"].as_array().unwrap().iter().find(|p|p["party_id"]==party).unwrap();
        assert!(row["historical"].as_array().unwrap().iter().any(|h|h["person"]["id"]==chair));
        assert_eq!(row["campaign"][0]["person"]["id"],chair);
        assert_eq!(row["campaign"][0]["executive_eligibility"]["role"],"party_only");
        assert_eq!(display["executive_person"]["id"],original);
        assert_eq!(serde_json::to_value(&w).unwrap(),before,"read-only policy view cannot reseat anybody");
        seat(&mut w,&r,policy,nation,event);
        assert_eq!(party_holder(&w,nation,party).person,chair);
        assert!(executive_id_with(&w,&r,nation).is_none(),"a chair/party president is not a national president");
    }
}

#[test]
fn a_separate_reviewed_candidate_can_win_without_replacing_the_party_chair() {
    let mut r=staged_roster();
    let mut person=person_in(&r,"ron_brown").unwrap().clone();
    person.id="fixture_presidential_nominee".into(); person.name="Test-only presidential nominee".into();
    r.people.push(person);
    let p=r.parties.iter_mut().find(|p|p.nation==NationId::USA&&p.party=="us_dem").unwrap();
    let mut t=p.terms[0].clone();
    t.id="fixture_presidential_nomination".into(); t.person="fixture_presidential_nominee".into();
    t.kind=TermKind::Candidate; t.role="Test-only presidential nominee".into();
    let mut policy=executive_eligibility::policy().unwrap().clone();
    policy.historical_grants.push(Grant{nation:p.nation,party:p.party.clone(),person:t.person.clone(),
        term:t.id.clone(),component:t.component.clone(),source_role:t.role.clone(),
        executive_role:OfficeRole::PresidentialContender,basis:Basis::ResearchedExecutiveRole,
        sources:vec!["https://example.invalid/test-only-nomination-evidence".into()],
        note:"Synthetic regression fixture, not a claim about a real candidate.".into()});
    p.terms.push(t);
    executive_eligibility::validate_policy(&policy,&r).unwrap();
    let mut w=campaign(&r,"1990-01-01");
    let chair=party_holder(&w,NationId::USA,"us_dem").clone();
    seat(&mut w,&r,&policy,NationId::USA,gov::Succession::Election{leader:"us_dem".into()});
    assert_eq!(json!(party_holder(&w,NationId::USA,"us_dem")),json!(chair));
    assert_eq!(executive_id_with(&w,&r,NationId::USA).as_deref(),Some("fixture_presidential_nominee"));
    let saved=serde_json::to_string(&w).unwrap();
    let restored:WorldState=serde_json::from_str(&saved).unwrap();
    validate_with(&restored,&r).unwrap();
    assert_eq!(executive_id_with(&restored,&r,NationId::USA).as_deref(),Some("fixture_presidential_nominee"),
        "load preserves a valid bound executive even when today's separate policy has no grant for it");
}

#[test]
fn parliamentary_continuity_is_exact_and_new_terms_default_to_party_only() {
    let p=executive_eligibility::policy().unwrap();
    let r=roster().unwrap();
    executive_eligibility::validate_policy(p,r).unwrap();
    assert_eq!(p.historical_grants.len(),93,"new imports require a deliberate policy review");
    for grant in &p.historical_grants {
        assert!(matches!(grant.nation,NationId::UK|NationId::Japan));
        let party=party_in(r,grant.nation,&grant.party).unwrap();
        let term=party.terms.iter().find(|t|t.id==grant.term).unwrap();
        assert!(p.allows_historical(party,term));
        let mut unreviewed=term.clone(); unreviewed.id.push_str("_new_unreviewed_role");
        assert!(!p.allows_historical(party,&unreviewed));
        let mut different_role=term.clone(); different_role.role="party administrator".into();
        assert!(!p.allows_historical(party,&different_role));
    }
    for (nation,party,person) in [(NationId::UK,"uk_lab","neil_kinnock"),(NationId::Japan,"jp_jsp","takako_doi")] {
        let mut w=campaign(r,"1990-01-01");
        seat(&mut w,r,p,nation,gov::Succession::Election{leader:party.into()});
        assert_eq!(executive_id_with(&w,r,nation).as_deref(),Some(person));
    }
}

#[test]
fn future_party_templates_need_a_distinct_presidential_grant_without_identity_changes() {
    let r=roster().unwrap();
    let policy=executive_eligibility::policy().unwrap();
    assert!(policy.future_office_grants.is_empty());
    for (nation,party,event) in [
        (NationId::USA,"us_dem",gov::Succession::Election{leader:"us_dem".into()}),
        (NationId::France,"fr_ps",gov::Succession::Death),
    ] {
        let mut w=campaign(r,"2030-01-01");
        seat(&mut w,r,policy,nation,event);
        let c=future::candidate(&party_holder(&w,nation,party).person).unwrap();
        assert!(executive_id_with(&w,r,nation).is_none());
        assert!(!policy.allows_future(c));
        assert_eq!(executive_eligibility::future_info(c)["role"],"party_only");
        let preview=view_with(&w,nation,today(&w),"2030-01-01",r);
        let row=preview["parties"].as_array().unwrap().iter().find(|p|p["party_id"]==party).unwrap();
        assert!(row["future_candidates"].as_array().unwrap().iter().any(|c|c["eligible"]==true));
        assert!(row["future_preview"].as_array().unwrap().iter().all(|c|
            c["eligible"]==false&&c["executive_eligibility"]["authorized"]==false));
        assert!(future::pick(nation,party,None,today(&w),w.party_leadership.as_ref().unwrap(),true).is_none());
    }
    // Authorize slot two only. The selection must skip the party chair in slot
    // one instead of either promoting it or abandoning the reviewed contender.
    let c=future::for_party(NationId::USA,"us_dem")[1];
    let identity_hash=future::binding_hash(c);
    let mut reviewed=policy.clone();
    reviewed.future_office_grants.push(Grant{nation:c.nation,party:c.party.clone(),person:c.person.id.clone(),
        term:c.term_id.clone(),component:c.component.clone(),source_role:"fictional party successor".into(),
        executive_role:OfficeRole::PresidentialContender,basis:Basis::AuthoredFictionalExecutiveRole,
        sources:vec!["https://www.usa.gov/election".into()],
        note:"Test-only authored fictional presidential contender; source describes institutions, not an invented person's biography or future outcome.".into()});
    executive_eligibility::validate_policy(&reviewed,r).unwrap();
    let mut w=campaign(r,"2030-01-01");
    seat(&mut w,r,&reviewed,NationId::USA,gov::Succession::Election{leader:"us_dem".into()});
    assert_eq!(executive_id_with(&w,r,NationId::USA).as_deref(),Some(c.person.id.as_str()));
    assert_ne!(party_holder(&w,NationId::USA,"us_dem").person,c.person.id);
    assert_eq!(future::binding_hash(c),identity_hash);
    validate_state(&crate::load(&crate::save(&w)).unwrap()).unwrap();
}

#[test]
fn executive_policy_rejects_implicit_presidencies_and_unsourced_or_mismatched_grants() {
    let r=staged_roster();
    let mut policy=executive_eligibility::policy().unwrap().clone();
    let party=party_in(&r,NationId::USA,"us_dem").unwrap(); let t=&party.terms[0];
    let g=Grant{nation:party.nation,party:party.party.clone(),person:t.person.clone(),term:t.id.clone(),
        component:t.component.clone(),source_role:t.role.clone(),executive_role:OfficeRole::ParliamentaryGovernmentContender,
        basis:Basis::ExistingParliamentaryGameplay,sources:t.sources.clone(),note:"A chair is not a nominee.".into()};
    policy.historical_grants.push(g);
    assert!(executive_eligibility::validate_policy(&policy,&r).is_err());
    let g=policy.historical_grants.last_mut().unwrap(); g.basis=Basis::ResearchedExecutiveRole;
    assert!(executive_eligibility::validate_policy(&policy,&r).is_err());
    let g=policy.historical_grants.last_mut().unwrap(); g.executive_role=OfficeRole::PresidentialContender; g.sources.clear();
    assert!(executive_eligibility::validate_policy(&policy,&r).is_err());
    let g=policy.historical_grants.last_mut().unwrap(); g.sources.push("https://example.invalid/test-only".into()); g.person="lee_atwater".into();
    assert!(executive_eligibility::validate_policy(&policy,&r).is_err());
}

fn reviewed_component_roster() -> Roster {
    let mut r=staged_roster();
    let p=r.parties.iter_mut().find(|p|p.nation==NationId::Germany&&p.party=="de_union").unwrap();
    // Exact identifiers supplied by the Germany research owner; this fixture
    // tests the schema only and invents no party office or person.
    p.kind=PartyKind::Coalition;
    p.components=[("de_union_cdu","CDU"),("de_union_csu","CSU")].into_iter().map(|(id,name)|Component{
        id:id.into(),name:name.into(),sources:vec!["test-only documented component schema".into()],
        founded:None,dissolved:None}).collect();
    r
}
fn empty_legacy_european_slots(r:&Roster)->WorldState {
    let mut w=campaign(r,"1990-01-01");
    w.month=2;
    let b=w.party_leadership.as_mut().unwrap(); let mut seen=BTreeSet::new();
    b.assignments=b.assignments.iter().filter_map(|a|{
        if !matches!((a.nation,a.party.as_str()),(NationId::France,"fr_udf")|(NationId::Germany,"de_union")) {
            return Some(a.clone());
        }
        if !seen.insert(a.party.clone()) {return None;}
        let mut old=a.clone(); old.component=None; old.holders.clear(); old.since_day=20; old.reason="death".into(); Some(old)
    }).collect();
    w
}
#[test]
fn all_reviewed_country_batches_require_explicit_future_executive_review() {
    let policy=executive_eligibility::policy().unwrap();
    for n in [NationId::USA,NationId::France,NationId::Germany,NationId::Italy,NationId::India,NationId::China,
        NationId::Brazil,NationId::SouthAfrica,NationId::Canada,NationId::Australia] {
        let candidates=future::catalog().iter().filter(|c|c.nation==n).collect::<Vec<_>>();
        assert!(!candidates.is_empty());
        for c in candidates {
            assert!(!policy.allows_future(c));
            let info=executive_eligibility::future_info(c);
            assert_eq!(info["role"],"party_only");
            assert_eq!(info["requires_executive_role_review"],true);
        }
        assert_eq!(executive_eligibility::view(n)["future_executive_role_review_required"],true);
    }
    for n in [NationId::UK,NationId::Japan] {
        assert!(future::catalog().iter().filter(|c|c.nation==n).all(|c|policy.allows_future(c)));
    }
}
#[test]
fn france_and_germany_expand_only_exact_empty_legacy_slots_without_any_other_changes() {
    let r=reviewed_component_roster(); let w=empty_legacy_european_slots(&r); let before=serde_json::to_value(&w).unwrap();
    let mut upgraded=w.clone(); assert!(migrate_legacy_empty_components_with(&mut upgraded,&r).unwrap());
    validate_with(&upgraded,&r).unwrap();
    for (n,pid,count) in [(NationId::France,"fr_udf",11),(NationId::Germany,"de_union",2)] {
        let rows=upgraded.party_leadership.as_ref().unwrap().assignments.iter().filter(|a|a.nation==n&&a.party==pid).collect::<Vec<_>>();
        assert_eq!(rows.len(),count);
        assert!(rows.iter().all(|a|a.holders.is_empty()&&a.since_day==20&&a.reason=="death"));
    }
    let strip=|mut v:Value|{v["party_leadership"]["assignments"].as_array_mut().unwrap().retain(|a|
        !((a["nation"]=="France"&&a["party"]=="fr_udf")||(a["nation"]=="Germany"&&a["party"]=="de_union"))); v};
    assert_eq!(strip(before),strip(serde_json::to_value(&upgraded).unwrap()));
    let saved=serde_json::to_string(&upgraded).unwrap();
    assert!(!migrate_legacy_empty_components_with(&mut upgraded,&r).unwrap());
    assert_eq!(serde_json::to_string(&upgraded).unwrap(),saved);
}
#[test]
fn european_component_upgrade_rejects_populated_future_and_mixed_or_changed_schemas_atomically() {
    let r=reviewed_component_roster(); let w=empty_legacy_european_slots(&r);
    for (n,pid) in [(NationId::France,"fr_udf"),(NationId::Germany,"de_union")] {
        let index=w.party_leadership.as_ref().unwrap().assignments.iter().position(|a|a.nation==n&&a.party==pid).unwrap();
        let mut populated=w.clone();
        populated.party_leadership.as_mut().unwrap().assignments[index].holders.push(Holder{
            person:format!("fictional_v1_{}_{}_main_01",format!("{n:?}").to_lowercase(),pid),
            term:"saved_future_main_term".into(),selected_day:0,identity_hash:"saved_identity_must_not_be_guessed".into()});
        let before=serde_json::to_value(&populated).unwrap();
        assert!(migrate_legacy_empty_components_with(&mut populated,&r).unwrap_err().contains("populated legacy"));
        assert_eq!(serde_json::to_value(&populated).unwrap(),before);
        let mut mixed=w.clone();
        let mut extra=mixed.party_leadership.as_ref().unwrap().assignments[index].clone();
        extra.component=Some(party_in(&r,n,pid).unwrap().components[0].id.clone());
        mixed.party_leadership.as_mut().unwrap().assignments.push(extra);
        let before=serde_json::to_value(&mixed).unwrap();
        assert!(migrate_legacy_empty_components_with(&mut mixed,&r).is_err());
        assert_eq!(serde_json::to_value(&mixed).unwrap(),before);
        let mut changed=r.clone(); changed.parties.iter_mut().find(|p|p.nation==n&&p.party==pid).unwrap().components.pop();
        let mut state=w.clone();
        assert!(migrate_legacy_empty_components_with(&mut state,&changed).unwrap_err().contains("exact documented"));
        assert_eq!(serde_json::to_value(&state).unwrap(),serde_json::to_value(&w).unwrap());
    }
}
