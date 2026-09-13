//! Read-only explanations of saved commitments. These projections neither
//! settle money nor decide whether a diplomatic answer is legal.
use serde_json::{json, Value};
use spheres_sim::{agency::{self, Offer, OfferKind}, clock, statecraft, world::{Conflict, NationId, WorldState, rung_name}};

const UPKEEP_NOTE: &str = "Estimate at today's GDP and current simulation cadence, per signatory. Future GDP, diplomacy and the settlement order can change the bill. This view makes no payment.";

fn alive(w: &WorldState, id: NationId) -> bool {
    w.nation_opt(id).is_some_and(|n| n.alive)
}

pub(crate) fn money(bn: f64) -> String {
    if bn == 0.0 { "$0".into() }
    else if bn.abs() >= 1.0 { format!("${bn:.3}bn") }
    else if bn.abs() >= 0.001 { format!("${:.3}m", bn * 1_000.0) }
    else if bn.abs() >= 0.000_001 { format!("${:.3}k", bn * 1_000_000.0) }
    else if bn.abs() >= 0.000_000_000_01 { format!("${:.2}", bn * 1_000_000_000.0) }
    else { "less than $0.01".into() }
}

fn upkeep(w: &WorldState, nation: NationId, count: usize) -> Value {
    let (step, _) = statecraft::pact_upkeep_charge(w, nation);
    // Add one charge per saved living pair, in settlement order. Do not turn
    // this into a booked total or include arrangements with a dead signatory.
    let next_step_bn = (0..count).fold(0.0, |total, _| total + step);
    let annual_share = statecraft::PACT_UPKEEP * count as f64;
    let annual_bn = w.nation(nation).gdp * annual_share;
    json!({"count":count,"annual_share":annual_share,"annual_bn":annual_bn,
        "annual_label":money(annual_bn),"next_step_bn":next_step_bn,
        "next_step_label":format!("{} per {} at today's conditions",money(next_step_bn),if clock::is_daily(w) {"day"} else {"model month"}),
        "cadence":if clock::is_daily(w) {"daily"} else {"monthly"},"note":UPKEEP_NOTE})
}

pub(crate) fn total_upkeep(w: &WorldState, nation: NationId) -> Value {
    let count = w.statecraft.pacts.iter().filter(|p| (p.a == nation || p.b == nation) && alive(w,p.a) && alive(w,p.b)).count();
    upkeep(w,nation,count)
}

fn people(w: &WorldState, ids: &[NationId]) -> Vec<Value> {
    ids.iter().map(|id| json!({"id":id,"name":id.name(),"alive":alive(w,*id)})).collect()
}

fn participation(w: &WorldState, c: &Conflict, nation: NationId) -> Value {
    let side = c.side_of(nation);
    let posture = c.posture_of(nation);
    let allies = match side {Some(true)=>c.side_a.as_slice(),Some(false)=>c.side_b.as_slice(),None=>&[]};
    let opponents = match side {Some(true)=>c.side_b.as_slice(),Some(false)=>c.side_a.as_slice(),None=>&[]};
    json!({"conflict_id":c.id,"theatre":c.theatre.name(),"started":format!("{:04}-{:02}",c.start_year,c.start_month),
        "side":side.map(|a|if a {"attacking"} else {"defending"}),
        "side_label":match side {Some(true)=>"Attacking coalition",Some(false)=>"Defending coalition",None=>"Outside this conflict"},
        "rung":posture.map(|p|p.rung),"rung_name":posture.map(|p|rung_name(p.rung)),
        "shooting":c.shooting(),"shooting_label":if c.shooting() {"Shooting conflict"} else {"Below the shooting threshold"},
        "allies":people(w,allies),"opponents":people(w,opponents)})
}

/// A numeric conflict id is insufficient: legacy worlds can reuse it. The
/// saved call pins its original aggressor, opening month and requesting side.
fn matched_conflict<'a>(w: &'a WorldState, offer: &Offer) -> Option<&'a Conflict> {
    let OfferKind::CallToArms{conflict,attacker,year,month,..} = offer.kind else {return None};
    w.conflict(conflict).filter(|c| c.origin_attacker==attacker && c.start_year==year && c.start_month==month && c.side_b.contains(&offer.from))
}

pub(crate) fn call_context(w: &WorldState, nation: NationId, offer_id: u64) -> Option<Value> {
    if w.player != Some(nation) || !alive(w,nation) {return None;}
    let offer = w.agency.offers.iter().find(|o|o.id==offer_id && o.to==nation)?;
    let OfferKind::CallToArms{conflict,rung,guaranteed,..} = offer.kind else {return None};
    let matched = matched_conflict(w,offer);
    let blocked = agency::response_error(w,nation,offer_id,true);
    let status = if matched.is_none() {"unmatched"} else if blocked.is_some() {"unavailable"} else {"reviewable"};
    Some(json!({"requester":offer.from,"requester_name":offer.from.name(),"guaranteed":guaranteed,
        "conflict_id":conflict,"requested_rung":rung,"requested_rung_name":rung_name(rung),
        "status":status,"status_label":match status {"unmatched"=>"The original conflict has ended or changed identity.","unavailable"=>"Acceptance is currently unavailable.",_=>"Review the native effects before answering."},
        "blocked":blocked,
        "conflict":matched.map(|c|json!({"conflict_id":c.id,"theatre":c.theatre.name(),"started":format!("{:04}-{:02}",c.start_year,c.start_month),
            "defenders":people(w,&c.side_b),"opponents":people(w,&c.side_a),"shooting":c.shooting(),
            "current_participation":participation(w,c,nation)})),
        "note":"The requested rung is a proposed posture, not an estimate of future war spending or a promise that every instrument will activate immediately. Only confirmation applies the native response."}))
}

/// Changes are read from the same cloned world that native response produced.
/// No synthetic rung transition or military payment is simulated by the view.
pub(crate) fn call_changes(before: &WorldState, after: &WorldState, nation: NationId, offer_id: u64) -> Vec<Value> {
    let Some(offer) = before.agency.offers.iter().find(|o|o.id==offer_id && o.to==nation) else {return vec![]};
    let (Some(first),Some(last)) = (matched_conflict(before,offer),matched_conflict(after,offer)) else {return vec![]};
    let (first,last) = (participation(before,first,nation),participation(after,last,nation));
    let mut out = vec![];
    if first["side"] != last["side"] {
        out.push(json!({"label":format!("Participation in {}",first["theatre"].as_str().unwrap()),"before":first["side_label"],"after":last["side_label"],"detail":"The coalition actually joined by this native response. Conflict participation can begin below the shooting threshold."}));
    }
    if first["rung"] != last["rung"] {
        let label = |v: &Value|v["rung"].as_u64().map_or_else(||"No commitment".into(),|r|format!("{} · {}",r,v["rung_name"].as_str().unwrap_or("Unspecified posture")));
        out.push(json!({"label":format!("Commitment in {}",first["theatre"].as_str().unwrap()),"before":label(&first),"after":label(&last),"detail":"The saved native posture after this answer. Further escalation, access and the ongoing cost of operations follow the existing conflict rules."}));
    }
    out
}

pub(crate) fn view(w: &WorldState, nation: NationId) -> Value {
    let mut defense = vec![];
    for pact in &w.statecraft.pacts {
        let partner = if pact.a==nation {pact.b} else if pact.b==nation {pact.a} else {continue};
        let both_alive = alive(w,nation) && alive(w,partner);
        let relationship = w.relation(nation,partner);
        let status = if !both_alive {"awaiting_lapse"} else {"in_force"};
        let mut warnings = vec![];
        if !both_alive {warnings.push("A signatory no longer exists. The saved pact is awaiting removal and is excluded from the upkeep estimate.");}
        else if relationship < -25.0 {warnings.push("Relations are below the lapse threshold. The next native settlement can end this pact; a final upkeep charge can occur before that check.");}
        let pending:Vec<_> = w.agency.offers.iter().filter(|o|o.to==nation && o.from==partner && matches!(o.kind,OfferKind::CallToArms{guaranteed:true,..})).map(|o|o.id).collect();
        defense.push(json!({"partner":partner,"partner_name":partner.name(),"since":format!("{:04}-{:02}",pact.since_year,pact.since_month),
            "status":status,"status_label":if both_alive {"Saved defense pact"} else {"Awaiting lapse"},"relationship":relationship,
            "upkeep":upkeep(w,nation,usize::from(both_alive)),"warnings":warnings,"pending_offer_ids":pending}));
    }
    defense.sort_by_key(|p|p["partner_name"].as_str().unwrap().to_string());
    let mut trade = vec![];
    for pact in &w.statecraft.trade {
        let partner = if pact.a==nation {pact.b} else if pact.b==nation {pact.a} else {continue};
        let dead = !alive(w,nation) || !alive(w,partner);
        let disrupted = statecraft::belligerents(w,nation,partner) || w.is_sanctioning(nation,partner) || w.is_sanctioning(partner,nation);
        let mut warnings = vec![];
        if dead {warnings.push("A signatory no longer exists. This saved agreement awaits removal.");}
        else if disrupted {warnings.push("Sanctions or opposing conflict participation currently block this agreement. Native settlement can remove it before integration grows.");}
        warnings.push("Overall dependency also includes supply contracts and delivered goods; it is not solely caused by this treaty.");
        trade.push(json!({"partner":partner,"partner_name":partner.name(),"depth":pact.depth,"dependency":w.trade_dependency(nation,partner),
            "partner_dependency":w.trade_dependency(partner,nation),"status":if dead || disrupted {"awaiting_collapse"} else {"in_force"},
            "status_label":if dead || disrupted {"Awaiting possible removal"} else {"Saved trade agreement"},"warnings":warnings}));
    }
    trade.sort_by_key(|p|p["partner_name"].as_str().unwrap().to_string());
    let conflicts:Vec<_> = w.conflicts.iter().filter(|c|c.involves(nation)).map(|c|participation(w,c,nation)).collect();
    json!({"defense_pacts":defense,"trade_agreements":trade,"conflicts":conflicts,"upkeep":total_upkeep(w,nation),
        "note":"These are the campaign's saved commitments. Opening this overview neither signs nor cancels an agreement. Calls to arms still follow their own deadline and standing response policy."})
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{decision_review, Game};
    use spheres_sim::{apply_command, commitment, init::world_1990, war, Command, world::{GameRules, Pact, TradePact}};
    use spheres_sim::world::NationId as N;

    fn add_pact(w: &mut WorldState, first: N, second: N) {
        let (a,b)=if first<second {(first,second)} else {(second,first)};
        w.statecraft.pacts.push(Pact{a,b,since_year:1990,since_month:1});
    }

    fn call(guaranteed: bool, rung: u8) -> (Game,u64,u32) {
        let mut g=Game::new(13,Some(N::France));
        g.world.rules.daily_simulation=true;
        g.world.statecraft.pacts.clear();
        if guaranteed {add_pact(&mut g.world,N::France,N::Kuwait);}
        let theatre=war::theatre_between(&g.world,N::Iraq,N::Kuwait);
        let cid=commitment::open_conflict(&mut g.world,N::Iraq,N::Kuwait,theatre).unwrap();
        let mut conflict=g.world.conflicts.pop().unwrap();
        agency::offer_call(&mut g.world,&mut conflict,N::France,rung,guaranteed);
        g.world.conflicts.push(conflict);
        let id=g.world.agency.offers.last().unwrap().id;
        (g,id,cid)
    }

    #[test]
    fn s10e_saved_overview_is_pure_and_preserves_real_dates_and_disruptions() {
        let (mut g,id,_) = call(true,2);
        add_pact(&mut g.world,N::France,N::Japan);
        add_pact(&mut g.world,N::France,N::Brazil);
        g.world.nation_mut(N::Brazil).alive=false;
        g.world.statecraft.trade.clear();
        let (a,b)=if N::France<N::Japan {(N::France,N::Japan)} else {(N::Japan,N::France)};
        g.world.statecraft.trade.push(TradePact{a,b,depth:0.35});
        g.world.sanctions.push((N::France,N::Japan));
        let saved=spheres_sim::save(&g.world);
        let board=view(&g.world,N::France);
        assert_eq!(board["defense_pacts"].as_array().unwrap().len(),3);
        assert_eq!(board["upkeep"]["count"],2,"Dead-signatory obligations are not charged");
        let pacts=board["defense_pacts"].as_array().unwrap();
        let kuwait=pacts.iter().find(|p|p["partner"]=="Kuwait").unwrap();
        assert_eq!(kuwait["since"],"1990-01");
        assert_eq!(kuwait["pending_offer_ids"],json!([id]));
        assert_eq!(pacts.iter().find(|p|p["partner"]=="Brazil").unwrap()["status"],"awaiting_lapse");
        let trade=&board["trade_agreements"][0];
        assert_eq!(trade["depth"],0.35);
        assert_eq!(trade["dependency"],g.world.trade_dependency(N::France,N::Japan));
        assert_eq!(trade["partner_dependency"],g.world.trade_dependency(N::Japan,N::France));
        assert_eq!(trade["status"],"awaiting_collapse");
        assert!(trade.get("since").is_none(),"Trade signing dates are not saved");
        assert!(board["conflicts"].as_array().unwrap().is_empty(),"An unanswered call is not participation");
        assert_eq!(view(&g.world,N::France),board);
        assert_eq!(spheres_sim::save(&g.world),saved);
        assert_eq!(view(&spheres_sim::load(&saved).unwrap(),N::France),board);
    }

    #[test]
    fn s10e_upkeep_estimate_matches_the_real_charge_in_both_cadences_and_book_modes() {
        for (daily,year,month,days) in [(false,1990,1,31),(true,1990,1,31),(true,1990,2,28),(true,1992,2,29)] {
            for treasury in [false,true] {
                let mut w=world_1990(GameRules{daily_simulation:daily,..Default::default()});
                w.year=year;w.month=month;w.day=1;
                w.statecraft.pacts.clear();w.statecraft.aid.clear();w.statecraft.trade.clear();
                add_pact(&mut w,N::France,N::Kuwait);
                add_pact(&mut w,N::France,N::Japan);
                let n=w.nation_mut(N::France);
                n.gdp=1_234.56789;n.treasury_bn=treasury.then_some(100.0);n.debt_bn=treasury.then_some(0.0);n.debt_gdp=0.0;
                let saved=spheres_sim::save(&w);
                let quote=total_upkeep(&w,N::France);
                let dt=if daily {1.0/days as f64} else {1.0};
                let legacy_bn=w.nation(N::France).gdp*0.003/12.0*dt;
                let legacy_share=0.003/12.0*dt;
                assert_eq!(statecraft::pact_upkeep_charge(&w,N::France),(legacy_bn,legacy_share),"Original operation order");
                assert_eq!(quote["next_step_bn"].as_f64().unwrap(),legacy_bn+legacy_bn);
                assert_eq!(spheres_sim::save(&w),saved);
                statecraft::tick(&mut w);
                let n=w.nation(N::France);
                if treasury {assert_eq!(n.treasury_bn,Some((100.0-legacy_bn)-legacy_bn));}
                else {assert_eq!(n.debt_gdp,legacy_share+legacy_share);}
            }
        }
    }

    #[test]
    fn s10e_call_acceptance_quotes_and_saves_actual_posture_without_inventing_instruments() {
        for guaranteed in [false,true] {
            let (mut g,id,cid)=call(guaranteed,2);
            let command=json!({"kind":"respond_diplomacy","offer":id,"accept":true});
            let before=spheres_sim::save(&g.world);
            let pc=g.world.nation(N::France).political_capital;
            let sanctions=g.world.sanctions.clone();
            let quote=decision_review::preview(&g,N::France,"decisions",&command);
            assert_eq!(quote["valid"],true,"{quote}");
            assert_eq!(quote["call_context"]["guaranteed"],guaranteed);
            assert_eq!(quote["call_context"]["requested_rung_name"],"sanctions");
            assert_eq!(quote["call_context"]["conflict"]["current_participation"]["side"],Value::Null);
            let rows=quote["changes"].as_array().unwrap();
            assert!(rows.iter().any(|r|r["before"]=="Outside this conflict" && r["after"]=="Defending coalition"));
            assert!(rows.iter().any(|r|r["before"]=="No commitment" && r["after"]=="2 · sanctions"));
            assert_eq!(spheres_sim::save(&g.world),before);
            let mut expected=g.world.clone();
            apply_command(&mut expected,&Command::RespondDiplomacy{nation:N::France,offer:id,accept:true}).unwrap();
            let payload=json!({"session_id":g.session_id,"client_id":"s10e-native-posture","request_seq":1,
                "commands":[command],"review_kind":"decisions","review_token":quote["review_token"]});
            assert_eq!(crate::transport::immediate_request(&mut g,&payload).unwrap()["errors"],json!([]));
            assert_eq!(spheres_sim::save(&g.world),spheres_sim::save(&expected));
            assert_eq!(g.world.nation(N::France).political_capital,pc,"Responding costs zero PC");
            assert_eq!(g.world.sanctions,sanctions,"Joining at rung 2 does not execute the sanctions instrument");
            assert_eq!(g.world.conflict(cid).unwrap().posture_of(N::France).unwrap().rung,2);
            assert!(!g.world.at_war(N::France),"Below-threshold participation is not a shooting war");
            let committed=spheres_sim::save(&g.world);
            let log=g.log.clone();
            assert_eq!(crate::transport::immediate_request(&mut g,&payload).unwrap()["command_replayed"],true);
            assert_eq!(spheres_sim::save(&g.world),committed);assert_eq!(g.log,log);
            let board=view(&g.world,N::France);
            assert_eq!(board["conflicts"][0]["side"],"defending");
            assert_eq!(board["conflicts"][0]["shooting"],false);
            assert_eq!(board,view(&spheres_sim::load(&committed).unwrap(),N::France));
        }
    }

    #[test]
    fn s10e_blocked_and_foreign_call_reviews_never_project_an_acceptance() {
        let (mut g,id,_)=call(true,8);
        g.world.access.clear();
        let command=json!({"kind":"respond_diplomacy","offer":id,"accept":true});
        let before=spheres_sim::save(&g.world);
        let quote=decision_review::preview(&g,N::France,"decisions",&command);
        assert_eq!(quote["valid"],false,"The expedition requires native basing access");
        assert_eq!(quote["call_context"]["status"],"unavailable");
        assert!(quote.get("review_token").is_none());assert_eq!(quote["changes"],json!([]));
        assert_eq!(spheres_sim::save(&g.world),before);
        assert_eq!(call_context(&g.world,N::Japan,id),None);
        let foreign=decision_review::preview(&g,N::Japan,"decisions",&command);
        assert_eq!(foreign["valid"],false);assert!(foreign.get("call_context").is_none());
    }

    #[test]
    fn s10e_nuclear_refusal_remains_native_even_when_basing_is_available() {
        let (mut g,id,cid)=call(false,6);
        g.world.nation_mut(N::France).nuclear=false;
        g.world.nation_mut(N::Iraq).nuclear=true;
        let theatre=g.world.conflict(cid).unwrap().theatre;
        let host=spheres_sim::theatre::theatre(&g.world,theatre).access_hosts[0];
        g.world.access.push(spheres_sim::theatre::Access{theatre,host,seeker:N::France,since_year:1990,since_month:1});
        assert!(spheres_sim::theatre::has_access(&g.world,N::France,theatre));
        let before=spheres_sim::save(&g.world);
        let quote=decision_review::preview(&g,N::France,"decisions",&json!({"kind":"respond_diplomacy","offer":id,"accept":true}));
        assert_eq!(quote["valid"],false);
        assert_eq!(quote["reason"],"Deterrence holds — they have the bomb and we do not.");
        assert_eq!(quote["call_context"]["blocked"],quote["reason"]);
        assert_eq!(quote["changes"],json!([]));assert!(quote.get("review_token").is_none());
        assert_eq!(spheres_sim::save(&g.world),before);
    }

    #[test]
    fn s10e_expired_or_reused_conflict_identity_is_context_only_and_never_authority() {
        let (g,id,cid)=call(true,2);
        let command=json!({"kind":"respond_diplomacy","offer":id,"accept":true});
        let mut expired=g.world.clone();
        let end=expired.agency.offers.iter().find(|o|o.id==id).unwrap().expires_day;
        let (year,month,day)=clock::date_from_day(end);expired.year=year;expired.month=month;expired.day=day;
        let context=call_context(&expired,N::France,id).unwrap();
        assert_eq!(context["status"],"unavailable");
        assert_eq!(context["blocked"],"The reply deadline has passed.");
        let mut expired_game=Game::new(13,Some(N::France));expired_game.world=expired;
        let saved=spheres_sim::save(&expired_game.world);
        let quote=decision_review::preview(&expired_game,N::France,"decisions",&command);
        assert_eq!(quote["valid"],false);assert!(quote.get("review_token").is_none());
        assert_eq!(quote["changes"],json!([]));assert_eq!(quote["call_context"]["status"],"unavailable");
        assert_eq!(spheres_sim::save(&expired_game.world),saved);
        let mut ended=g.world.clone();ended.conflicts.clear();
        assert_eq!(call_context(&ended,N::France,id).unwrap()["conflict"],Value::Null);
        for mismatch in 0..4 {
            let mut stale=g.world.clone();
            let c=stale.conflict_mut(cid).unwrap();
            match mismatch {0=>c.start_year+=1,1=>c.start_month+=1,2=>c.origin_attacker=N::Japan,_=>{c.side_b.retain(|n|*n!=N::Kuwait);}}
            let context=call_context(&stale,N::France,id).unwrap();
            assert_eq!(context["status"],"unmatched");assert_eq!(context["conflict"],Value::Null);
            assert!(call_changes(&stale,&stale,N::France,id).is_empty());
            let mut game=Game::new(13,Some(N::France));game.world=stale;
            let before=spheres_sim::save(&game.world);
            let quote=decision_review::preview(&game,N::France,"decisions",&command);
            assert_eq!(quote["valid"],false);assert!(quote.get("review_token").is_none());
            assert_eq!(quote["call_context"]["conflict"],Value::Null);
            assert_eq!(spheres_sim::save(&game.world),before);
        }
    }

    #[test]
    fn s10e_changed_call_decline_does_not_invent_participation_or_upkeep_changes() {
        let (mut g,id,_)=call(true,2);
        g.world.rules.economic_competition=true;
        spheres_sim::domination::subjugate(&mut g.world,N::Iraq,N::France);
        let quote=decision_review::preview(&g,N::France,"decisions",&json!({"kind":"respond_diplomacy","offer":id,"accept":false}));
        assert_eq!(quote["valid"],true);assert_eq!(quote["call_context"]["status"],"unavailable");
        assert!(quote["changes"].as_array().unwrap().iter().all(|r| {
            let label=r["label"].as_str().unwrap();
            !label.starts_with("Participation in ") && !label.starts_with("Commitment in ") && label!="Defense pact upkeep estimate"
        }));
    }
}
