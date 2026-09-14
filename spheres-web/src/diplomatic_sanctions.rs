//! Read-only sanction directions and bilateral conditions. Saved sanctions have
//! no enactment date; these views report today's state without inventing one.
use serde_json::{json, Value};
use spheres_sim::{economy, logistics, statecraft, world::{NationId, WorldState}};

const GROWTH_NOTE: &str = "Annualized growth drag from all sanctioning economies at today's output shares, including the affected country's diplomacy ministry. This is one growth term, not a forecast of total growth or a payment.";
const FREIGHT_NOTE: &str = "A route check does not book cargo or guarantee a sale, stock, capacity or delivery. Contracts, route access and future conditions still apply.";

fn alive(w: &WorldState, id: NationId) -> bool {
    w.nation_opt(id).is_some_and(|n| n.alive)
}

fn growth_drag(w: &WorldState, nation: NationId) -> Value {
    let n = w.nation(nation);
    let annual_pp = economy::growth_terms(n, n.state_invest_gdp, n.interest_rate,
        &economy::Conditions::of(w, nation)).sanctions * 100.0;
    json!({"annual_pp":annual_pp,"label":format!("{annual_pp:.3} pp/year"),"note":GROWTH_NOTE})
}

/// The board is attached only to the active player's agency, never a foreign
/// inspection. Directional entries retain the native saved record order.
pub(crate) fn view(w: &WorldState, nation: NationId) -> Value {
    if w.player != Some(nation) || !alive(w, nation) { return Value::Null; }
    let row = |imposer: NationId, target: NationId| json!({
        "imposer":imposer,"imposer_name":imposer.name(),"target":target,"target_name":target.name(),
        "imposer_alive":alive(w,imposer),"target_alive":alive(w,target),
        "can_review_lift":imposer==nation && target!=nation && alive(w,target)
    });
    let incoming: Vec<_> = w.sanctions.iter().filter(|(_, target)| *target==nation)
        .map(|(imposer,target)|row(*imposer,*target)).collect();
    let outgoing: Vec<_> = w.sanctions.iter().filter(|(imposer,_)| *imposer==nation)
        .map(|(imposer,target)|row(*imposer,*target)).collect();
    let mut targets: Vec<_> = w.nations.iter().filter(|n|n.alive && n.id!=nation)
        .map(|n|json!({"id":n.id,"name":n.id.name(),"sanctioned_by_player":w.is_sanctioning(nation,n.id)})).collect();
    targets.sort_by_key(|n|n["name"].as_str().unwrap().to_string());
    json!({"nation":nation,"nation_name":nation.name(),"date_label":w.date_str(),
        "incoming":incoming,"outgoing":outgoing,"targets":targets,"growth_drag":growth_drag(w,nation),
        "note":"Each government controls its own sanctions. You can review lifting your restrictions; other governments decide their own policy. The date reflects current campaign conditions."})
}

fn route(w: &WorldState, seller: NationId, buyer: NationId) -> Value {
    if !logistics::enabled(w) {
        return json!({"available":null,"label":"Physical freight routing is not enabled in this campaign."});
    }
    match logistics::plan(w,seller,buyer) {
        Ok(_) => json!({"available":true,"label":"A freight route is available under current conditions."}),
        Err(reason) => json!({"available":false,"label":reason}),
    }
}

fn conditions(w: &WorldState, actor: NationId, target: NationId) -> Value {
    let yours = w.is_sanctioning(actor,target);
    let theirs = w.is_sanctioning(target,actor);
    let opposing = statecraft::belligerents(w,actor,target);
    let mut restrictions = vec![];
    if yours { restrictions.push(format!("{} sanctions {}",actor.name(),target.name())); }
    if theirs { restrictions.push(format!("{} sanctions {}",target.name(),actor.name())); }
    if opposing { restrictions.push("The countries are on opposing sides of a conflict".into()); }
    let restriction_label = if restrictions.is_empty() {
        "No bilateral sanctions or opposing conflict. Other trade and route requirements still apply.".into()
    } else { format!("Bilateral restrictions remain: {}.",restrictions.join("; ")) };
    let other_sanctioners: Vec<_> = w.sanctions.iter().filter(|(imposer,recipient)| *recipient==target && *imposer!=actor)
        .map(|(imposer,_)|json!({"id":imposer,"name":imposer.name(),"alive":alive(w,*imposer)})).collect();
    let treaty = w.statecraft.trade.iter().find(|p| (p.a==actor && p.b==target) || (p.b==actor && p.a==target));
    let disrupted = yours || theirs || opposing || !alive(w,actor) || !alive(w,target);
    let treaty = json!({"saved":treaty.is_some(),"depth":treaty.map(|p|p.depth),
        "status_label":match treaty {None=>"No saved trade agreement.",Some(_) if disrupted=>"Saved trade agreement; restrictions can remove it at the next settlement.",Some(_)=>"Saved trade agreement; integration follows future settlements."}});
    json!({"relation":w.relation(actor,target),"relation_label":format!("{:.1}",w.relation(actor,target)),
        "your_sanctions":yours,"their_sanctions":theirs,"opposing_conflict":opposing,
        "restriction_label":restriction_label,"other_sanctioners":other_sanctioners,
        "target_growth_drag":growth_drag(w,target),"trade_treaty":treaty,
        "freight":{"enabled":logistics::enabled(w),"outbound":route(w,actor,target),"inbound":route(w,target,actor),"note":FREIGHT_NOTE}})
}

/// `after` is exclusively the cloned native command result. A refused review
/// retains its current bilateral identity and state, with no invented outcome.
pub(crate) fn context(before: &WorldState, after: Option<&WorldState>, actor: NationId, target: NationId) -> Value {
    json!({"actor":actor,"actor_name":actor.name(),"target":target,"target_name":target.name(),
        "before":conditions(before,actor,target),"after":after.map(|w|conditions(w,actor,target)),
        "note":"These are current conditions and, when the decision is legal, its immediate result. Lifting your sanction leaves reverse and third-party sanctions in place. Improving relations does not lift sanctions, end a conflict or restore a removed treaty. Future settlement and diplomacy can change these conditions."})
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{decision_review, parse_command, transport, Game};
    use spheres_sim::{apply_command, load, save, world::{NationId as N, TradePact}};

    fn game() -> Game {
        let mut g=Game::new(13,Some(N::France));
        g.world.rules.daily_simulation=true;
        g.world.nation_mut(N::France).political_capital=100.0;
        g.world.sanctions.clear();
        g.world.statecraft.trade.clear();
        g.world.statecraft.pacts.clear();
        g.world.conflicts.clear();
        g
    }

    #[test]
    fn s10g_sanctions_board_is_pure_directional_and_has_no_foreign_lift_authority() {
        let mut g=game();
        g.world.sanctions=vec![(N::France,N::Japan),(N::Japan,N::France),(N::Brazil,N::France)];
        let original=save(&g.world);
        let board=view(&g.world,N::France);
        assert_eq!(board["nation"],"France");
        assert_eq!(board["incoming"].as_array().unwrap().len(),2);
        assert!(board["incoming"].as_array().unwrap().iter().all(|r|r["can_review_lift"]==false && r["target"]=="France"));
        assert_eq!(board["outgoing"][0]["imposer"],"France");
        assert_eq!(board["outgoing"][0]["target"],"Japan");
        assert_eq!(board["outgoing"][0]["can_review_lift"],true);
        assert!(board["targets"].as_array().unwrap().iter().all(|n|n["id"]!="France"));
        assert!(view(&g.world,N::Japan).is_null());
        assert_eq!(save(&g.world),original);
        assert_eq!(view(&load(&original).unwrap(),N::France),board);
        g.world.nation_mut(N::Japan).alive=false;
        let dead=view(&g.world,N::France);
        assert_eq!(dead["outgoing"][0]["target_alive"],false);
        assert_eq!(dead["outgoing"][0]["can_review_lift"],false);
        assert!(dead["targets"].as_array().unwrap().iter().all(|n|n["id"]!="Japan"));
    }

    #[test]
    fn s10g_lift_keeps_reverse_and_other_sanctions_and_uses_exact_native_freight_and_growth() {
        let mut g=game();
        g.world.rules.resource_gates=true;g.world.rules.resource_market=true;
        g.world.rules.logistics_routes=true;g.world.rules.physical_logistics=true;
        g.world.sanctions=vec![(N::France,N::Japan),(N::Japan,N::France),(N::India,N::Japan)];
        let command=json!({"kind":"lift","target":"Japan"});
        let original=save(&g.world);
        let quote=decision_review::preview(&g,N::France,"decisions",&command);
        assert_eq!(quote["valid"],true);
        let mut native=g.world.clone();
        apply_command(&mut native,&parse_command(&g.world,&command,N::France).unwrap()).unwrap();
        let c=&quote["bilateral_context"];
        assert_eq!(c["before"]["your_sanctions"],true);
        assert_eq!(c["after"]["your_sanctions"],false);
        assert_eq!(c["after"]["their_sanctions"],true);
        assert_eq!(c["after"]["other_sanctioners"][0]["id"],"India");
        assert_eq!(c["after"]["freight"]["outbound"]["label"],logistics::plan(&native,N::France,N::Japan).unwrap_err());
        assert_eq!(c["after"]["freight"]["inbound"]["label"],logistics::plan(&native,N::Japan,N::France).unwrap_err());
        assert_eq!(c["after"]["freight"]["outbound"]["available"],false);
        assert_eq!(c["after"]["target_growth_drag"],growth_drag(&native,N::Japan));
        assert!(c["after"]["target_growth_drag"]["annual_pp"].as_f64().unwrap()>0.0);
        assert_eq!(native.relation(N::France,N::Japan),g.world.relation(N::France,N::Japan));
        assert_eq!(save(&g.world),original);
        assert_eq!(context(&load(&original).unwrap(),Some(&load(&save(&native)).unwrap()),N::France,N::Japan),*c);
    }

    #[test]
    fn s10g_sanction_quotes_saved_treaty_loss_as_later_settlement_not_immediate_deletion() {
        let mut g=game();
        let (a,b)=if N::France<N::Japan {(N::France,N::Japan)} else {(N::Japan,N::France)};
        g.world.statecraft.trade.push(TradePact{a,b,depth:0.425});
        let command=json!({"kind":"sanction","target":"Japan"});
        let original=save(&g.world);
        let quote=decision_review::preview(&g,N::France,"decisions",&command);
        assert_eq!(quote["valid"],true);
        let treaty=&quote["bilateral_context"]["after"]["trade_treaty"];
        assert_eq!(treaty["saved"],true);assert_eq!(treaty["depth"],0.425);
        assert!(treaty["status_label"].as_str().unwrap().contains("next settlement"));
        let mut native=g.world.clone();
        apply_command(&mut native,&parse_command(&g.world,&command,N::France).unwrap()).unwrap();
        assert_eq!(native.statecraft.trade,g.world.statecraft.trade);
        statecraft::tick(&mut native);
        assert!(native.statecraft.trade.is_empty());
        assert_eq!(save(&g.world),original);
    }

    #[test]
    fn s10g_improvement_quotes_actual_caps_and_full_price_even_without_a_relation_change() {
        for (before,after) in [(98.0,100.0),(100.0,100.0)] {
            let mut g=game();g.world.set_relation(N::France,N::Japan,before);
            let original=save(&g.world);
            let quote=decision_review::preview(&g,N::France,"decisions",&json!({"kind":"improve","target":"Japan"}));
            assert_eq!(quote["valid"],true);
            assert_eq!(quote["bilateral_context"]["before"]["relation"],before);
            assert_eq!(quote["bilateral_context"]["after"]["relation"],after);
            let rows=quote["changes"].as_array().unwrap();
            let relation=rows.iter().find(|r|r["label"]=="Relations with Japan").unwrap();
            assert_eq!(relation["before"],format!("{before:.1}"));assert_eq!(relation["after"],format!("{after:.1}"));
            let pc=rows.iter().find(|r|r["label"]=="Political capital").unwrap();
            assert_eq!(pc["before"],"100.0");assert_eq!(pc["after"],"98.0");
            if before==after {assert!(quote["warnings"].as_array().unwrap().iter().any(|w|w.as_str().unwrap().contains("will not increase")));}
            assert_eq!(save(&g.world),original);
        }
    }

    #[test]
    fn s10g_native_refusal_retains_bilateral_identity_and_has_no_after_or_authority() {
        let mut g=game();g.world.nation_mut(N::France).political_capital=0.0;
        let original=save(&g.world);
        let command=json!({"kind":"sanction","target":"Japan"});
        let quote=decision_review::preview(&g,N::France,"decisions",&command);
        assert_eq!(quote["valid"],false);assert!(quote["review_token"].is_null());
        assert_eq!(quote["bilateral_context"]["actor"],"France");assert_eq!(quote["bilateral_context"]["target"],"Japan");
        assert!(quote["bilateral_context"]["before"].is_object());assert!(quote["bilateral_context"]["after"].is_null());
        assert_eq!(decision_review::preview(&g,N::Japan,"decisions",&command)["valid"],false);
        assert!(decision_review::preview(&g,N::France,"decisions",&json!({"kind":"lift","target":"France"}))["bilateral_context"].is_null());
        assert_eq!(save(&g.world),original);
    }

    #[test]
    fn s10g_reverse_policy_change_invalidates_lift_review_before_any_mutation() {
        let mut g=game();g.world.sanctions.push((N::France,N::Japan));
        g.world.nation_mut(N::Japan).political_capital=100.0;
        let command=json!({"kind":"lift","target":"Japan"});
        let quote=decision_review::preview(&g,N::France,"decisions",&command);
        let payload=json!({"session_id":g.session_id,"commands":[command],"review_kind":"decisions","review_token":quote["review_token"]});
        apply_command(&mut g.world,&spheres_sim::Command::Sanction{imposer:N::Japan,target:N::France}).unwrap();
        let original=save(&g.world);let log=g.log.clone();
        let error=transport::immediate_request(&mut g,&payload).unwrap_err();
        assert!(error.requires_review && error.not_applied);
        assert_eq!(save(&g.world),original);assert_eq!(g.log,log);
    }

    #[test]
    fn s10g_disabled_freight_is_not_reported_as_an_open_route() {
        let mut g=game();g.world.rules.physical_logistics=false;
        let c=context(&g.world,None,N::France,N::Japan);
        assert_eq!(c["before"]["freight"]["enabled"],false);
        assert!(c["before"]["freight"]["outbound"]["available"].is_null());
        assert!(c["before"]["freight"]["inbound"]["available"].is_null());
    }

    #[test]
    fn s10g_lifting_the_last_bilateral_sanction_does_not_end_opposing_conflict() {
        let mut g=game();
        g.world.rules.resource_gates=true;g.world.rules.resource_market=true;
        g.world.rules.logistics_routes=true;g.world.rules.physical_logistics=true;
        let theatre=spheres_sim::war::theatre_between(&g.world,N::France,N::Japan);
        spheres_sim::commitment::open_conflict(&mut g.world,N::France,N::Japan,theatre).unwrap();
        g.world.sanctions=vec![(N::France,N::Japan)];
        let command=json!({"kind":"lift","target":"Japan"});
        let original=save(&g.world);
        let quote=decision_review::preview(&g,N::France,"decisions",&command);
        assert_eq!(quote["valid"],true);
        let c=&quote["bilateral_context"]["after"];
        assert_eq!(c["your_sanctions"],false);assert_eq!(c["their_sanctions"],false);
        assert_eq!(c["opposing_conflict"],true);
        let mut native=g.world.clone();
        apply_command(&mut native,&parse_command(&g.world,&command,N::France).unwrap()).unwrap();
        assert_eq!(c["freight"]["outbound"]["label"],logistics::plan(&native,N::France,N::Japan).unwrap_err());
        assert_eq!(c["freight"]["inbound"]["available"],false);
        assert_eq!(save(&g.world),original);
    }

    #[test]
    fn s10g_repeated_sanction_or_empty_lift_discloses_no_change_and_no_charge() {
        for kind in ["sanction","lift"] {
            let mut g=game();
            if kind=="sanction" {g.world.sanctions.push((N::France,N::Japan));}
            let original=save(&g.world);
            let quote=decision_review::preview(&g,N::France,"decisions",&json!({"kind":kind,"target":"Japan"}));
            assert_eq!(quote["valid"],true);
            assert!(quote["warnings"].as_array().unwrap().iter().any(|w|w.as_str().unwrap().contains("political capital charge")));
            assert!(!quote["changes"].as_array().unwrap().iter().any(|r|r["label"]=="Political capital"));
            assert_eq!(quote["bilateral_context"]["before"],quote["bilateral_context"]["after"]);
            assert_eq!(save(&g.world),original);
        }
    }

    #[test]
    #[ignore = "Requires an explicitly named, read-only mature campaign archive"]
    fn s10g_mature_open_route_preview_latency() {
        let path=std::env::var("SPHERES_S10_PERF_SAVE").expect("name the read-only mature archive");
        let source=std::fs::read_to_string(&path).unwrap();
        let mut g=crate::storage::decode(&source).unwrap();
        let original_world=save(&g.world);
        let log=g.log.clone();
        let history=serde_json::to_value(&g.history).unwrap();
        let history_epoch=g.history_epoch;
        spheres_sim::resources::warm(&mut g.world);
        assert_eq!(save(&g.world),original_world,"Warming derived resources cannot change the source world");
        let actor=g.world.player.expect("the archive must have a player");
        assert!(alive(&g.world,actor),"The archive's government must still be active");
        assert!(logistics::enabled(&g.world),"Use a mature archive with physical freight enabled");
        // Discover a real open pair without deleting sanctions, changing access,
        // ending wars or fabricating a route. Discovery is outside the timings.
        let target=g.world.nations.iter().filter(|n|n.alive && n.id!=actor)
            .map(|n|n.id).find(|target| !g.world.is_sanctioning(actor,*target)
                && !g.world.is_sanctioning(*target,actor)
                && !statecraft::belligerents(&g.world,actor,*target)
                && logistics::plan(&g.world,actor,*target).is_ok()
                && logistics::plan(&g.world,*target,actor).is_ok())
            .expect("The archive must contain a living foreign partner with actual routes in both directions");
        let command=json!({"kind":"improve","target":target});
        let native=parse_command(&g.world,&command,actor).unwrap();
        let mut authored_preconditions=vec![];
        // Prefer the unmodified saved campaign. If this particular archive has
        // spent its standing, alter only a private world clone and disclose the
        // native command-price floor; never change the source file or routes.
        if !spheres_sim::affordable(&g.world,&native) {
            let before=g.world.nation(actor).political_capital;
            let after=spheres_sim::price_of(&g.world,&native).expect("the unaffordable command has a price");
            let mut trial=g.world.clone();
            trial.nation_mut(actor).political_capital=after;
            g.world=trial;
            authored_preconditions.push(json!({"kind":"political_capital_floor","nation":actor,
                "before":before,"after":after,"reason":"Only the private benchmark clone was raised to the native command price so the valid four-route review path can be measured."}));
        }
        let benchmark_world=save(&g.world);
        let mut samples=vec![];
        let mut route_facts=Value::Null;
        for i in 0..24 {
            let start=std::time::Instant::now();
            let quote=decision_review::preview(&g,actor,"decisions",&command);
            let elapsed=start.elapsed().as_secs_f64()*1000.0;
            assert_eq!(quote["valid"],true,"{quote}");
            let context=&quote["bilateral_context"];
            assert_eq!(context["actor"],json!(actor));assert_eq!(context["target"],json!(target));
            for when in ["before","after"] {
                assert_eq!(context[when]["your_sanctions"],false);
                assert_eq!(context[when]["their_sanctions"],false);
                assert_eq!(context[when]["opposing_conflict"],false);
                assert_eq!(context[when]["freight"]["enabled"],true);
                for direction in ["outbound","inbound"] {
                    assert_eq!(context[when]["freight"][direction]["available"],true,"The benchmark must execute real open-route searches: {quote}");
                }
            }
            if i==0 {route_facts=json!({"before":context["before"]["freight"],"after":context["after"]["freight"]});}
            if i>=3 {samples.push(elapsed);}
        }
        let mut sorted=samples.clone();sorted.sort_by(f64::total_cmp);
        let p95=sorted[19];let maximum=sorted[20];
        assert_eq!(save(&g.world),benchmark_world);
        assert_eq!(g.log,log);assert_eq!(serde_json::to_value(&g.history).unwrap(),history);
        assert_eq!(g.history_epoch,history_epoch);
        assert_eq!(std::fs::read_to_string(&path).unwrap(),source);
        let passed=p95<=300.0 && maximum<=750.0;
        println!("S10G_SANCTIONS_PERF {}",json!({"passed":passed,"source":path,
            "nation":actor,"nation_name":actor.name(),"target":target,"target_name":target.name(),
            "date":g.world.date_str(),"command":command,"warmups":3,"samples":21,
            "samples_ms":samples,"p95_ms":p95,"max_ms":maximum,"p95_limit_ms":300,"max_limit_ms":750,
            "route_facts":route_facts,"authored_preconditions":authored_preconditions,
            "source_file_unchanged":true,"benchmark_world_unchanged":true,"log_history_unchanged":true,"days_advanced":0,
            "scope":"Native diplomatic preview generation including a cloned command, four native route checks and the complete-world review token. Excludes HTTP, browser and target discovery. No decision is applied and no campaign time advances."}));
        assert!(passed,"Predeclared open-route preview latency limits failed");
    }
}
