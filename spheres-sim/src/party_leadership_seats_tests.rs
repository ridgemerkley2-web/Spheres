use super::*;

fn world(date:&str)->WorldState {
    let mut w=crate::init::world_1990(GameRules{ideology_blocs:true,historical_party_leadership:true,
        daily_simulation:true,..Default::default()});
    (w.year,w.month,w.day)=strict_date(date).unwrap();
    w.party_leadership=None;
    ensure_all(&mut w);
    validate_state(&w).unwrap();
    w
}
fn row<'a>(w:&'a WorldState,party:&str)->&'a Assignment {
    w.party_leadership.as_ref().unwrap().assignments.iter()
        .find(|a|a.nation==NationId::Germany&&a.party==party).unwrap()
}
fn row_mut<'a>(w:&'a mut WorldState,party:&str)->&'a mut Assignment {
    w.party_leadership.as_mut().unwrap().assignments.iter_mut()
        .find(|a|a.nation==NationId::Germany&&a.party==party).unwrap()
}
fn election(w:&mut WorldState,party:&str) {
    gov::seat_office(w,NationId::Germany,&gov::Succession::Election{leader:party.into()});
    validate_state(w).unwrap();
}
fn legacy_named_executive(w:&mut WorldState,party:&str,holder:&Holder) {
    // A valid existing saved executive remains accepted by the new policy. This
    // fixture exercises the actual mortality hook without granting a new chair
    // an executive role in the production policy.
    let at=today(w); let b=w.party_leadership.as_mut().unwrap();
    b.executives.retain(|e|e.nation!=NationId::Germany);
    b.executives.push(Executive{nation:NationId::Germany,party:party.into(),holder:holder.clone(),
        since_day:at,reason:"election".into()});
    validate_state(w).unwrap();
}
fn model<'a>(v:&'a Value,party:&str)->&'a Value {
    v["parties"].as_array().unwrap().iter().find(|p|p["party_id"]==party).unwrap()
}

#[test]
fn paired_policy_has_exact_qualified_fiction_without_new_components_or_identity_fields() {
    let d=leadership_seats::policy().unwrap();
    leadership_seats::validate_policy(d,roster().unwrap()).unwrap();
    for p in &d.parties {
        assert_eq!(p.target_holders,2);
        let party=party_in(roster().unwrap(),p.nation,&p.party).unwrap();
        assert!(party.components.is_empty());
        assert!(p.authored_woman_candidates.iter().chain(&p.authored_man_candidates)
            .all(|id|future::candidate(id).is_some_and(|c|c.component.is_none())));
    }
    let mut invalid=d.clone();
    invalid.parties[0].authored_woman_candidates[0]=future::for_party(NationId::UK,"uk_lab")[0].person.id.clone();
    assert!(leadership_seats::validate_policy(&invalid,roster().unwrap()).is_err());
    let mut invalid=d.clone(); invalid.parties[0].target_holders=3;
    assert!(leadership_seats::validate_policy(&invalid,roster().unwrap()).is_err());
}

#[test]
fn date_advancement_preserves_historical_trio_single_chair_and_all_existing_metadata() {
    let mut w=world("1990-01-01");
    assert_eq!(row(&w,"de_gruene").holders.len(),3);
    assert_eq!(row(&w,"de_spd").holders.len(),1);
    let old=json!(w.party_leadership);
    (w.year,w.month,w.day)=strict_date("2030-01-01").unwrap();
    ensure_all(&mut w);
    let _=view(&w,NationId::Germany);
    assert_eq!(json!(w.party_leadership),old);
    let green=json!(row(&w,"de_gruene"));
    election(&mut w,"de_gruene");
    assert_eq!(json!(row(&w,"de_gruene")),green,"an event cannot evict one of the 1990 trio");
    let spd=json!(row(&w,"de_spd"));
    election(&mut w,"de_spd");
    assert_eq!(json!(row(&w,"de_spd")),spd,"a surviving historical single chair has no newly inferred vacancy");
    let v=view(&w,NationId::Germany);
    assert_eq!(model(&v,"de_spd")["future_leadership_seats"]["historical_single_chair_retained"],true);
}

#[test]
fn empty_future_pair_fills_only_on_an_event_and_survives_save_reload() {
    for party in ["de_gruene","de_spd"] {
        let mut w=world("2027-01-01");
        assert!(row(&w,party).holders.is_empty());
        let empty=json!(row(&w,party));
        ensure_all(&mut w); let _=view(&w,NationId::Germany);
        assert_eq!(json!(row(&w,party)),empty);
        election(&mut w,party);
        let holders=&row(&w,party).holders;
        assert_eq!(holders.len(),2);
        assert_ne!(holders[0].person,holders[1].person);
        assert!(holders.iter().all(|h|future::candidate(&h.person).unwrap().component.is_none()));
        assert!(executive_person(&w,NationId::Germany).is_none(),"paired chairs still need executive role review");
        let saved=crate::save(&w); let restored=crate::load(&saved).unwrap();
        assert_eq!(crate::save(&restored),saved);
        let v=view(&restored,NationId::Germany);
        assert!(model(&v,party)["campaign"].as_array().unwrap().iter().all(|h|h["kind"]=="co_leader"));
        assert_eq!(model(&v,party)["future_leadership_seats"]["vacancies"],0);
    }
}

#[test]
fn actual_death_replaces_only_the_vacant_co_seat_and_retains_individual_dates() {
    for party in ["de_gruene","de_spd"] {
        let mut w=world("2027-01-01"); election(&mut w,party);
        let removed=row(&w,party).holders[0].clone();
        let survivor=row(&w,party).holders[1].clone();
        legacy_named_executive(&mut w,party,&removed);
        let saved=crate::save(&w);
        let mut replay=crate::load(&saved).unwrap();
        for state in [&mut w,&mut replay] {
            state.day=2;
            gov::seat_office(state,NationId::Germany,&gov::Succession::Death);
            validate_state(state).unwrap();
            let a=row(state,party);
            assert_eq!(a.holders.len(),2);
            assert_eq!(json!(a.holders.iter().find(|h|h.person==survivor.person).unwrap()),json!(survivor));
            assert!(a.holders.iter().all(|h|h.person!=removed.person));
            assert_eq!(a.since_day,today(state));
            let v=view(state,NationId::Germany);
            let displayed=model(&v,party)["campaign"].as_array().unwrap().iter()
                .find(|h|h["person_id"]==survivor.person).unwrap();
            assert_eq!(displayed["since_day"],survivor.selected_day);
            assert_eq!(displayed["party_succession_day"],today(state));
        }
        assert_eq!(crate::save(&w),crate::save(&replay));
    }
}

#[test]
fn a_historical_co_holder_is_retained_without_inferred_gender_or_rewritten_identity() {
    let mut w=world("2026-09-07");
    assert_eq!(row(&w,"de_spd").holders.len(),2);
    election(&mut w,"de_spd");
    let removed=row(&w,"de_spd").holders[0].clone();
    let survivor=row(&w,"de_spd").holders[1].clone();
    legacy_named_executive(&mut w,"de_spd",&removed);
    (w.year,w.month,w.day)=strict_date("2030-01-01").unwrap();
    gov::seat_office(&mut w,NationId::Germany,&gov::Succession::Death);
    validate_state(&w).unwrap();
    assert_eq!(row(&w,"de_spd").holders.len(),2);
    assert_eq!(json!(row(&w,"de_spd").holders.iter().find(|h|h.person==survivor.person).unwrap()),json!(survivor));
    let v=view(&w,NationId::Germany);
    assert_eq!(model(&v,"de_spd")["future_leadership_seats"]["unclassified_historical_holders"],1);
    assert!(model(&v,"de_spd")["future_leadership_seats"]["qualification_note"].as_str().unwrap().contains("not inferred"));
}

#[test]
fn qualification_exhaustion_and_date_bounds_leave_vacancies_without_reusing_people() {
    let mut w=world("2027-01-01"); election(&mut w,"de_spd");
    let removed=row(&w,"de_spd").holders[0].clone();
    let survivor=row(&w,"de_spd").holders[1].clone();
    legacy_named_executive(&mut w,"de_spd",&removed);
    let other_woman=leadership_seats::policy().unwrap().parties.iter().find(|p|p.party=="de_spd").unwrap()
        .authored_woman_candidates.iter().find(|id|**id!=removed.person).unwrap().clone();
    let at=today(&w);
    w.party_leadership.as_mut().unwrap().deaths.push(Death{person:other_woman,day:at});
    w.day=2;
    gov::seat_office(&mut w,NationId::Germany,&gov::Succession::Death);
    validate_state(&w).unwrap();
    assert_eq!(json!(row(&w,"de_spd").holders),json!([survivor]));
    let p=party_in(roster().unwrap(),NationId::Germany,"de_spd").unwrap();
    assert!(leadership_seats::additions(p,row(&w,"de_spd"),day("2026-09-07").unwrap(),w.party_leadership.as_ref().unwrap()).is_empty());
    assert!(leadership_seats::additions(p,row(&w,"de_spd"),day("2036-01-01").unwrap(),w.party_leadership.as_ref().unwrap()).is_empty());
    let saved=crate::save(&w); assert_eq!(crate::save(&crate::load(&saved).unwrap()),saved);
    let v=view(&w,NationId::Germany);
    assert_eq!(model(&v,"de_spd")["future_leadership_seats"]["vacancies"],1);
    // No date change or repeat event can resurrect a dead qualified candidate.
    w.day=3; election(&mut w,"de_spd");
    assert_eq!(row(&w,"de_spd").holders.len(),1);
    row_mut(&mut w,"de_spd").holders.clear();
    let at=today(&w);
    let remaining=future::for_party(NationId::Germany,"de_spd").into_iter().map(|c|c.person.id.clone()).collect::<Vec<_>>();
    for id in remaining {
        if !dead(w.party_leadership.as_ref().unwrap(),&id) {
            w.party_leadership.as_mut().unwrap().deaths.push(Death{person:id,day:at});
        }
    }
    election(&mut w,"de_spd");
    assert!(row(&w,"de_spd").holders.is_empty());
}

#[test]
fn single_leader_future_parties_and_executive_exclusions_keep_their_existing_semantics() {
    let mut w=world("2027-01-01");
    election(&mut w,"de_fdp");
    assert_eq!(row(&w,"de_fdp").holders.len(),1);
    let id=future::for_party(NationId::Germany,"de_spd")[0].person.id.clone();
    let at=today(&w);
    w.party_leadership.as_mut().unwrap().office_exclusions.push(OfficeExclusion{nation:NationId::Germany,person:id.clone(),day:at});
    election(&mut w,"de_spd");
    assert!(row(&w,"de_spd").holders.iter().any(|h|h.person==id),"executive term limits do not bar party leadership");
}
