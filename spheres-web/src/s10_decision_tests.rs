//! Deliberate transaction boundary fixtures, not campaign advancement evidence.
use super::*;
use serde_json::{json, Value};
use spheres_sim::{agency, clock, government};

fn game() -> Game {
    let mut g = Game::new(13, Some(NationId::France));
    g.world.rules.daily_simulation = true;
    government::ensure_all(&mut g.world);
    g
}
fn policy() -> Value {
    json!({"kind":"set_diplomatic_policy","policy":{"defense_pacts":"decline","trade_treaties":"accept","calls_to_arms":"review"}})
}
fn reviewed(g: &Game, command: Value, kind: &str) -> Value {
    let quote = decision_review::preview(g, g.world.player.unwrap(), kind, &command);
    assert_eq!(quote["valid"], true, "{quote}");
    json!({"session_id":g.session_id,"client_id":"s10-review","request_seq":1,"commands":[command],"review_kind":kind,"review_token":quote["review_token"]})
}

#[test]
fn s10_review_is_pure_and_commits_exact_native_consequences_once() {
    for command in [policy(),json!({"kind":"sanction","target":"Japan"}),json!({"kind":"improve","target":"Japan"})] {
        let mut g = game();
        let before = save(&g.world);
        let payload = reviewed(&g,command.clone(),"decisions");
        assert_eq!(save(&g.world),before);
        let mut actual = g.world.clone();
        let native=parse_command(&g.world,&command,NationId::France).unwrap();
        apply_command(&mut actual,&native).unwrap();
        let response=transport::immediate_request(&mut g,&payload).unwrap();
        assert_eq!(response["errors"],json!([]));
        assert_eq!(save(&g.world),save(&actual));
        let committed=save(&g.world); let log=g.log.clone();
        assert_eq!(transport::immediate_request(&mut g,&payload).unwrap()["command_replayed"],true);
        assert_eq!(save(&g.world),committed); assert_eq!(g.log,log);
        let loaded=load(&committed).unwrap();
        assert_eq!(decision_review::agency_view(&loaded,NationId::France),decision_review::agency_view(&g.world,NationId::France));
    }
}

#[test]
fn s10_review_refuses_other_tab_changes_ticks_and_replacements_before_mutation() {
    for mutation in 0..4 {
        let mut g=game();
        let payload=reviewed(&g,policy(),"decisions");
        match mutation {
            0=>{let other=json!({"commands":[{"kind":"improve","target":"Japan"}]});transport::immediate_request(&mut g,&other).unwrap();},
            1=>clock::advance_date(&mut g.world),
            2=>g.session_id=fresh_session_id(),
            _=>{g.world.nation_mut(NationId::France).political_capital=0.0;},
        }
        let before=save(&g.world);let log=g.log.clone();
        assert!(transport::immediate_request(&mut g,&payload).unwrap_err().requires_review);
        assert_eq!(save(&g.world),before); assert_eq!(g.log,log);
        assert!(!g.command_receipts.contains_key("s10-review"));
    }
}

#[test]
fn s10_review_cannot_move_to_another_command_batch_channel_or_receipt() {
    let mut g=game();
    let original=reviewed(&g,policy(),"decisions");
    for mode in 0..5 {
        let mut p=original.clone();
        match mode {
            0=>p["commands"][0]["policy"]["calls_to_arms"]=json!("accept"),
            1=>p["commands"].as_array_mut().unwrap().push(json!({"kind":"improve","target":"Japan"})),
            2=>p["review_kind"]=json!("government"),
            3=>{p.as_object_mut().unwrap().remove("review_token");},
            _=>p["review_token"]=json!(""),
        }
        let before=save(&g.world);
        assert!(transport::immediate_request(&mut g,&p).unwrap_err().requires_review);
        assert_eq!(save(&g.world),before);
    }
    transport::immediate_request(&mut g,&original).unwrap();
    let mut changed_receipt=original.clone();changed_receipt["review_token"]=json!("different");
    assert!(transport::immediate_request(&mut g,&changed_receipt).is_err());
    assert_eq!(transport::immediate_request(&mut g,&original).unwrap()["command_replayed"],true);
}

#[test]
fn s10_government_review_has_the_same_server_staleness_guard() {
    let mut g=game();
    let command=government_json(&g.world,NationId::France)["actions"].as_array().unwrap().iter()
        .find(|a|a["refusal"].is_null()).expect("a shipped legal decision")["command"].clone();
    let payload=reviewed(&g,command,"government");
    let other=json!({"commands":[policy()]});
    transport::immediate_request(&mut g,&other).unwrap();
    let before=save(&g.world);
    assert!(transport::immediate_request(&mut g,&payload).unwrap_err().requires_review);
    assert_eq!(save(&g.world),before);
}

#[test]
fn s10_foreign_malformed_self_and_unaffordable_decisions_refuse_purely() {
    let mut g=game();
    g.world.nation_mut(NationId::France).political_capital=0.0;
    let before=save(&g.world);
    for command in [json!({"kind":"sanction","target":"Japan"}),json!({"kind":"sanction","target":"France"}),
        json!({"kind":"improve","target":"Unrecognized"}),json!({"kind":"tax","value":0.2}),
        json!({"kind":"respond_diplomacy","offer":-1,"accept":true}),
        json!({"kind":"respond_diplomacy","offer":1,"accept":true,"nation":"Japan"}),
        json!({"kind":"set_diplomatic_policy","policy":{"defense_pacts":"review","trade_treaties":"review","calls_to_arms":"review","other":"accept"}})] {
        let q=decision_review::preview(&g,NationId::France,"decisions",&command);
        assert_eq!(q["valid"],false,"{q}");assert!(q["review_token"].is_null());
    }
    assert_eq!(decision_review::preview(&g,NationId::Japan,"decisions",&policy())["valid"],false);
    assert_eq!(save(&g.world),before);
}

fn treaty(g:&mut Game)->u64 {
    agency::offer_treaty(&mut g.world,NationId::Japan,NationId::France,agency::OfferKind::TradeTreaty).unwrap();
    g.world.agency.offers.last().unwrap().id
}

#[test]
fn s10_standing_policy_preserves_pending_offers_and_deadlines_survive_reload() {
    let mut g=game();let id=treaty(&mut g);
    let old=g.world.agency.offers[0].clone();
    let payload=reviewed(&g,policy(),"decisions");
    transport::immediate_request(&mut g,&payload).unwrap();
    assert_eq!(g.world.agency.offers[0],old);
    let native=load(&save(&g.world)).unwrap();
    assert_eq!(native.agency,g.world.agency);
    let (y,m,d)=clock::date_from_day(old.expires_day);
    g.world.year=y;g.world.month=m;g.world.day=d;
    let command=json!({"kind":"respond_diplomacy","offer":id,"accept":true});
    let q=decision_review::preview(&g,NationId::France,"decisions",&command);
    assert_eq!(q["valid"],false);
    assert!(q["reason"].as_str().unwrap().contains("deadline"));
    agency::tick(&mut g.world);
    let view=decision_review::agency_view(&g.world,NationId::France);
    assert!(view["offers"].as_array().unwrap().is_empty());
    assert_eq!(view["history"][0]["date_label"],format!("{y:04}-{m:02}-{d:02}"));
    assert_eq!(view["history"][0]["title"],"Trade treaty");
    assert_eq!(view["history"][0]["from_name"],"Japan");
    assert_eq!(view["history"][0]["outcome"],"expired: declined");
    assert_eq!(decision_review::agency_view(&load(&save(&g.world)).unwrap(),NationId::France),view);
}

#[test]
fn s10_answered_offer_records_actual_date_once_and_late_confirmation_is_pure() {
    let mut g=game();let id=treaty(&mut g);
    let payload=reviewed(&g,json!({"kind":"respond_diplomacy","offer":id,"accept":false}),"decisions");
    transport::immediate_request(&mut g,&payload).unwrap();
    assert_eq!(g.world.agency.history.len(),1);
    assert_eq!(g.world.agency.history[0].resolved_day,clock::absolute_day(&g.world));
    transport::immediate_request(&mut g,&payload).unwrap();
    assert_eq!(g.world.agency.history.len(),1);
    let mut late=payload.clone();late["request_seq"]=json!(2);
    let saved=save(&g.world);
    assert!(transport::immediate_request(&mut g,&late).unwrap_err().requires_review);
    assert_eq!(save(&g.world),saved);
}

#[test]
fn s10_reads_and_derived_caches_do_not_invalidate_a_review() {
    let mut g=game();let payload=reviewed(&g,policy(),"decisions");
    resources::warm(&mut g.world);
    let _=state_json(&g,None); let _=government_json(&g.world,NationId::France);
    decision_review::validate(&g,&payload).unwrap();
    for path in ["/api/government/preview","/api/decisions/preview"] {
        assert!(exchange_read_path(path));
        assert!(!exchange_session_matches(&Method::Post,path,&json!({}),&g.session_id));
        assert!(exchange_session_matches(&Method::Post,path,&json!({"session_id":g.session_id}),&g.session_id));
    }
}

#[test]
fn s10_accepted_trade_and_pact_reviews_preserve_native_effects_and_one_receipt() {
    for kind in [agency::OfferKind::TradeTreaty, agency::OfferKind::DefensePact] {
        let mut g = game();
        // Deliberate legal, near-capped relationship fixtures, not historical treaties.
        g.world.statecraft.pacts.clear();
        g.world.statecraft.trade.clear();
        g.world.sanctions.clear();
        g.world.set_relation(NationId::France, NationId::Japan, 98.0);
        assert!(agency::offer_treaty(&mut g.world, NationId::Japan, NationId::France, kind.clone()).unwrap());
        let offer = g.world.agency.offers.last().unwrap().clone();
        let command = json!({"kind":"respond_diplomacy","offer":offer.id,"accept":true});
        let before = save(&g.world);
        let rng = g.world.rng.clone();
        let quote = decision_review::preview(&g, NationId::France, "decisions", &command);
        assert_eq!(quote["valid"], true, "{quote}");
        let payload = reviewed(&g, command.clone(), "decisions");
        assert_eq!(save(&g.world), before, "Review must leave the offer unanswered");
        let mut native = g.world.clone();
        apply_command(&mut native, &parse_command(&g.world, &command, NationId::France).unwrap()).unwrap();
        let response = transport::immediate_request(&mut g, &payload).unwrap();
        assert_eq!(response["errors"], json!([]));
        assert_eq!(save(&g.world), save(&native));
        assert_eq!(g.world.rng, rng, "Human consent does not roll AI acceptance");
        assert_eq!(g.world.relation(NationId::France, NationId::Japan), 100.0);
        let relation = quote["changes"].as_array().unwrap().iter()
            .find(|c| c["label"] == "Relations with Japan").unwrap();
        assert_eq!(relation["before"], "98.0");
        assert_eq!(relation["after"], "100.0", "Review must show the capped native result");
        match kind {
            agency::OfferKind::TradeTreaty => {
                assert_eq!(g.world.trade_depth(NationId::France, NationId::Japan), 0.05);
                assert_eq!(g.world.statecraft.trade.len(), 1);
                assert!(!g.world.allied(NationId::France, NationId::Japan));
            }
            agency::OfferKind::DefensePact => {
                assert!(g.world.allied(NationId::France, NationId::Japan));
                assert_eq!(g.world.statecraft.pacts.len(), 1);
                assert_eq!(g.world.trade_depth(NationId::France, NationId::Japan), 0.0);
                assert!(quote["warnings"].as_array().unwrap().iter()
                    .any(|w| w.as_str().unwrap().contains("0.3% of GDP annually")));
            }
            _ => unreachable!(),
        }
        assert!(g.world.agency.offers.is_empty());
        assert_eq!(g.world.agency.history.len(), 1);
        assert_eq!(g.world.agency.history[0].offer, offer);
        assert_eq!(g.world.agency.history[0].outcome, "accepted");
        assert_eq!(g.world.agency.history[0].resolved_day, clock::absolute_day(&g.world));
        let committed = save(&g.world);
        let log = g.log.clone();
        assert_eq!(transport::immediate_request(&mut g, &payload).unwrap()["command_replayed"], true);
        assert_eq!(save(&g.world), committed);
        assert_eq!(g.log, log);
        assert_eq!(decision_review::agency_view(&load(&committed).unwrap(), NationId::France),
                   decision_review::agency_view(&g.world, NationId::France));
    }
}

#[test]
fn s10_accepted_defense_call_joins_native_side_and_rung_exactly_once() {
    // Match the native agency defense fixture: an authored test obligation and
    // newly opened conflict, not an assumed war in the opening historical world.
    let mut g = Game::new(13, Some(NationId::USA));
    g.world.rules.daily_simulation = true;
    g.world.nation_mut(NationId::USA).political_capital = 100.0;
    g.world.statecraft.pacts.clear();
    let (a, b) = if NationId::USA < NationId::Kuwait {
        (NationId::USA, NationId::Kuwait)
    } else { (NationId::Kuwait, NationId::USA) };
    g.world.statecraft.pacts.push(spheres_sim::world::Pact { a, b, since_year:1990, since_month:1 });
    let theatre = spheres_sim::war::theatre_between(&g.world, NationId::Iraq, NationId::Kuwait);
    let cid = spheres_sim::commitment::open_conflict(&mut g.world, NationId::Iraq, NationId::Kuwait, theatre).unwrap();
    let mut conflict = g.world.conflicts.pop().unwrap();
    agency::offer_call(&mut g.world, &mut conflict, NationId::USA, 2, true);
    g.world.conflicts.push(conflict);
    let offer = g.world.agency.offers.last().unwrap().clone();
    assert_eq!(offer.to, NationId::USA);
    assert_eq!(offer.from, NationId::Kuwait);
    assert!(!g.world.conflict(cid).unwrap().involves(NationId::USA));
    let command = json!({"kind":"respond_diplomacy","offer":offer.id,"accept":true});
    let before = save(&g.world);
    let quote = decision_review::preview(&g, NationId::USA, "decisions", &command);
    assert_eq!(quote["valid"], true, "{quote}");
    assert!(quote["warnings"].as_array().unwrap().iter()
        .any(|w| w.as_str().unwrap().contains("enter the defending side at rung 2")));
    let payload = reviewed(&g, command.clone(), "decisions");
    assert_eq!(save(&g.world), before);
    let mut native = g.world.clone();
    apply_command(&mut native, &parse_command(&g.world, &command, NationId::USA).unwrap()).unwrap();
    assert_eq!(transport::immediate_request(&mut g, &payload).unwrap()["errors"], json!([]));
    assert_eq!(save(&g.world), save(&native));
    let joined = g.world.conflict(cid).unwrap();
    assert!(!joined.side_a.contains(&NationId::USA));
    assert_eq!(joined.side_b.iter().filter(|id| **id == NationId::USA).count(), 1);
    assert_eq!(joined.posture_of(NationId::USA).unwrap().rung, 2);
    assert_eq!(joined.posture_of(NationId::USA).unwrap().objective, spheres_sim::world::Objective::Deny);
    assert!(g.world.allied(NationId::USA, NationId::Kuwait));
    assert!(g.world.agency.offers.is_empty());
    assert_eq!(g.world.agency.history.len(), 1);
    assert_eq!(g.world.agency.history[0].outcome, "accepted");
    let committed = save(&g.world);
    let log = g.log.clone();
    assert_eq!(transport::immediate_request(&mut g, &payload).unwrap()["command_replayed"], true);
    assert_eq!(save(&g.world), committed);
    assert_eq!(g.log, log);
    let resumed = load(&committed).unwrap();
    let resumed_conflict = resumed.conflict(cid).unwrap();
    let live_conflict = g.world.conflict(cid).unwrap();
    assert_eq!(resumed_conflict.side_a, live_conflict.side_a);
    assert_eq!(resumed_conflict.side_b, live_conflict.side_b);
    assert_eq!(resumed_conflict.posture_of(NationId::USA), live_conflict.posture_of(NationId::USA));
    assert_eq!(decision_review::agency_view(&resumed, NationId::USA),
               decision_review::agency_view(&g.world, NationId::USA));
}

#[test]
fn s10_peg_exit_review_shows_capped_costs_and_does_not_reset_rate_or_recharge() {
    let mut g = game();
    // Deliberately near the native stability floor and inflation ceiling.
    let n = g.world.nation_mut(NationId::France);
    n.political_capital = agency::BREAK_PEG_PC;
    n.stability = 2.0;
    n.inflation = 2.99;
    n.interest_rate = 0.055;
    g.world.player_set_rate = true;
    agency::establish_peg(&mut g.world, NationId::France, 0.055);
    let command = json!({"kind":"break_currency_peg"});
    let before = save(&g.world);
    let quote = decision_review::preview(&g, NationId::France, "decisions", &command);
    assert_eq!(quote["valid"], true, "{quote}");
    for (label, first, last) in [("Political capital", "12.0", "0.0"),
        ("Stability", "2.0", "0.0"), ("Inflation", "299.00%", "300.00%"),
        ("Central bank", "Manual or pegged", "Automatic"),
        ("Currency regime", "Pegged at 5.50%", "Floating")] {
        let row = quote["changes"].as_array().unwrap().iter().find(|c| c["label"] == label).unwrap();
        assert_eq!(row["before"], first, "{label}");
        assert_eq!(row["after"], last, "{label}");
    }
    let payload = reviewed(&g, command.clone(), "decisions");
    assert_eq!(save(&g.world), before);
    let mut native = g.world.clone();
    apply_command(&mut native, &parse_command(&g.world, &command, NationId::France).unwrap()).unwrap();
    assert_eq!(transport::immediate_request(&mut g, &payload).unwrap()["errors"], json!([]));
    assert_eq!(save(&g.world), save(&native));
    let n = g.world.nation(NationId::France);
    assert_eq!(n.political_capital, 0.0);
    assert_eq!(n.stability, 0.0);
    assert_eq!(n.inflation, 3.0);
    assert_eq!(n.interest_rate, 0.055, "Exit resumes future policy; it does not set a new rate now");
    assert!(agency::pegged_rate(&g.world, NationId::France).is_none());
    assert!(!g.world.player_set_rate);
    let committed = save(&g.world);
    let log = g.log.clone();
    assert_eq!(transport::immediate_request(&mut g, &payload).unwrap()["command_replayed"], true);
    assert_eq!(save(&g.world), committed);
    assert_eq!(g.log, log);
    assert_eq!(decision_review::preview(&g, NationId::France, "decisions", &command)["valid"], false);
    assert_eq!(save(&g.world), committed);
    assert_eq!(decision_review::agency_view(&load(&committed).unwrap(), NationId::France),
               decision_review::agency_view(&g.world, NationId::France));
}
