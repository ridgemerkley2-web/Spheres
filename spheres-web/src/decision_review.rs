//! Immediate, campaign-bound political reviews. These are transport preconditions,
//! not saved authority or a second simulation. Legacy command clients remain valid;
//! the reviewed UI opts into the guard with both envelope fields.
use crate::{government_view, parse_command, Game};
use serde_json::{json, Value};
use spheres_sim::{agency, clock, world::{NationId, WorldState}};
use std::{collections::hash_map::DefaultHasher, hash::Hasher, io::Write};

struct Digest(DefaultHasher);
impl Write for Digest {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.0.write(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> { Ok(()) }
}

/// Hash the serialized world, including RNG, date and identities, without a large
/// intermediate save string. Derived caches are deliberately skipped by serde.
/// The token is an accidental-staleness guard, not an authentication credential.
fn token(g: &Game, kind: &str, command: &Value) -> String {
    let mut digest = Digest(DefaultHasher::new());
    serde_json::to_writer(&mut digest, &(&g.session_id, kind, command, &g.world))
        .expect("the campaign is serializable");
    format!("decision-v1-{:016x}", digest.0.finish())
}

pub(crate) fn preview(g: &Game, nation: NationId, kind: &str, command: &Value) -> Value {
    let mut value = match kind {
        "government" => government_view::preview(&g.world, nation, command),
        "decisions" => decisions(&g.world, nation, command),
        _ => invalid(command, "Choose a supported decision review."),
    };
    value["session_id"] = json!(g.session_id);
    value["nation"] = json!(nation);
    value["date_label"] = json!(g.world.date_str());
    if value["valid"] == true { value["review_token"] = json!(token(g, kind, command)); }
    value
}

/// Call after receipt replay detection but before any mutation, parsing or cost.
/// A reviewed single order cannot be attached to a batch or a different command.
pub(crate) fn validate(g: &Game, payload: &Value) -> Result<(), crate::transport::AdvanceError> {
    let kind = payload.get("review_kind");
    let supplied = payload.get("review_token");
    if kind.is_none() && supplied.is_none() { return Ok(()); }
    let refuse = || crate::transport::AdvanceError {
        message: "This decision review is out of date or no longer matches. Refresh the campaign and review it again; nothing was applied.".into(),
        requires_review: true,
        not_applied: true,
    };
    let kind = kind.and_then(Value::as_str).filter(|v| matches!(*v, "government" | "decisions")).ok_or_else(refuse)?;
    let commands = payload["commands"].as_array().filter(|a| a.len() == 1).ok_or_else(refuse)?;
    if payload["session_id"].as_str() != Some(&g.session_id)
        || supplied.and_then(Value::as_str) != Some(token(g, kind, &commands[0]).as_str()) {
        return Err(refuse());
    }
    // Re-check the page's allowlist and native legality as well as the world token.
    let nation = g.world.player.ok_or_else(refuse)?;
    let valid = match kind {
        "government" => government_view::preview(&g.world, nation, &commands[0]),
        _ => decisions(&g.world, nation, &commands[0]),
    };
    if valid["valid"] != true { return Err(refuse()); }
    Ok(())
}

fn invalid(command: &Value, reason: &str) -> Value {
    json!({"valid":false,"reason":reason,"title":"Review decision","command":command,"description":"","changes":[],"warnings":[]})
}

fn change(rows: &mut Vec<Value>, label: &str, before: impl ToString, after: impl ToString, detail: &str) {
    let (before, after) = (before.to_string(), after.to_string());
    if before != after { rows.push(json!({"label":label,"before":before,"after":after,"detail":detail})); }
}

fn policy_label(p: agency::ResponsePolicy) -> &'static str {
    match p { agency::ResponsePolicy::Review => "Ask me", agency::ResponsePolicy::Accept => "Accept when legal", agency::ResponsePolicy::Decline => "Decline" }
}

pub(crate) fn receipt_label(nation: NationId, command: &Value) -> String {
    let label = match command["kind"].as_str() {
        Some("lift") => format!("lift sanctions on {}",command["target"].as_str().unwrap_or("the selected country")),
        Some("resume_automatic_bank") => "resume automatic central-bank policy".into(),
        _ => "apply the reviewed decision".into(),
    };
    format!("{} confirms: {label}.",nation.name())
}

fn decisions(w: &WorldState, nation: NationId, command: &Value) -> Value {
    if w.player != Some(nation) || w.nation_opt(nation).is_none_or(|n| !n.alive) {
        return invalid(command, "Only your active government can review this decision.");
    }
    let kind = command["kind"].as_str().unwrap_or("");
    let (title, description, fields): (&str, &str, &[&str]) = match kind {
        "respond_diplomacy" => (if command["accept"] == true {"Accept diplomatic request"} else {"Decline diplomatic request"}, "This answer applies immediately to the selected request. Review its deadline and any continuing obligations.", &["kind","offer","accept"]),
        "set_diplomatic_policy" => ("Update standing diplomatic policy", "The policy applies to future requests. Existing requests still need their own answer before their deadline.", &["kind","policy"]),
        "break_currency_peg" => ("Exit currency peg", "The currency floats and the automatic central bank resumes. This has immediate political and economic costs.", &["kind"]),
        "resume_automatic_bank" => ("Resume automatic central bank", "Future policy rates are chosen by the existing central-bank rules.", &["kind"]),
        "sanction" => ("Impose sanctions", "Close sanctioned trade lanes and reduce relations. The target's growth drag follows the sanctioning economies' share of world output.", &["kind","target"]),
        "lift" => ("Lift sanctions", "Remove your sanctions. Other governments' sanctions and the existing relationship remain in place.", &["kind","target"]),
        "improve" => ("Improve relations", "Spend political capital on an immediate diplomatic improvement, within the existing relationship limits.", &["kind","target"]),
        "choose_campaign_aim" => ("Choose campaign aim", "Freeze this aim's target from today's country conditions. Progress is measured as the campaign advances.", &["kind","aim"]),
        "continue_sandbox" => ("Continue in sandbox", "Set aside the active aim. Its record stays in the campaign and the world keeps running.", &["kind"]),
        _ => return invalid(command, "Choose a supported decision from the current page."),
    };
    // No ignored actor, target or nested policy fields in a reviewed command.
    if command.as_object().is_none_or(|m| m.len() != fields.len() || m.keys().any(|k| !fields.contains(&k.as_str()))) {
        return invalid(command, "This decision contains missing or unrecognized fields.");
    }
    if kind == "set_diplomatic_policy" && command["policy"].as_object().is_none_or(|m| m.len() != 3 || m.keys().any(|k| !["defense_pacts","trade_treaties","calls_to_arms"].contains(&k.as_str()))) {
        return invalid(command, "Choose all three standing responses without extra fields.");
    }
    let target = if fields.contains(&"target") {
        let Some(target) = command["target"].as_str().and_then(NationId::parse).filter(|id| *id != nation && w.nation_opt(*id).is_some_and(|n| n.alive)) else {
            return invalid(command, "Choose another active country.");
        };
        Some(target)
    } else { None };
    let Some(native) = parse_command(w, command, nation) else { return invalid(command, "This decision cannot be read."); };
    let mut after = w.clone();
    if let Err(reason) = spheres_sim::apply_command(&mut after, &native) { return invalid(command, &reason); }
    let (before_n, after_n) = (w.nation(nation), after.nation(nation));
    let (before_a, after_a) = (agency::view(w, nation), agency::view(&after, nation));
    let mut rows = vec![];
    change(&mut rows,"Political capital",format!("{:.1}", before_n.political_capital),format!("{:.1}", after_n.political_capital),"Immediate cost after the native limits.");
    change(&mut rows,"Stability",format!("{:.1}", before_n.stability),format!("{:.1}", after_n.stability),"Immediate national stability.");
    change(&mut rows,"Inflation",format!("{:.2}%", before_n.inflation * 100.0),format!("{:.2}%", after_n.inflation * 100.0),"The current annual inflation rate.");
    change(&mut rows,"Reputation",format!("{:.1}", w.reputation(nation)),format!("{:.1}", after.reputation(nation)),"Diplomatic credibility, after its native limits.");
    change(&mut rows,"Central bank",if before_a.automatic_bank {"Automatic"} else {"Manual or pegged"},if after_a.automatic_bank {"Automatic"} else {"Manual or pegged"},"The policy used for future rate decisions.");
    let monetary = |v: &agency::MonetaryRegime| match v { agency::MonetaryRegime::Floating => "Floating".into(), agency::MonetaryRegime::Pegged{rate} => format!("Pegged at {:.2}%",rate*100.0) };
    change(&mut rows,"Currency regime",monetary(&before_a.monetary),monetary(&after_a.monetary),"The saved monetary commitment.");
    for (label, before, next) in [("Defense pacts",before_a.policy.defense_pacts,after_a.policy.defense_pacts),("Trade treaties",before_a.policy.trade_treaties,after_a.policy.trade_treaties),("Calls to arms",before_a.policy.calls_to_arms,after_a.policy.calls_to_arms)] {
        change(&mut rows,label,policy_label(before),policy_label(next),"Future requests only; pending requests keep their deadline.");
    }
    let mut warnings = vec![];
    if let Some(offer) = before_a.offers.iter().find(|o| Some(o.id) == command["offer"].as_u64()) {
        warnings.push(format!("{} · reply by {} ({} days remaining). {}",offer.from_name,offer.expires,offer.days_remaining,offer.consequence));
        change(&mut rows,"Request status","Awaiting your reply",after.agency.history.iter().find(|h|h.offer.id==offer.id).map_or("Pending",|h|h.outcome.as_str()),"Recorded in the saved diplomatic ledger with its resolution date.");
        change(&mut rows,&format!("Trade integration with {}",offer.from_name),format!("{:.1}%",w.trade_depth(nation,offer.from)*100.0),format!("{:.1}%",after.trade_depth(nation,offer.from)*100.0),"The current treaty depth. Integration develops through subsequent economic settlements.");
    }
    if kind == "choose_campaign_aim" {
        if let Some(goal) = after.campaign_aims.active.as_ref() {
            let evaluation = spheres_sim::campaign_aims::evaluate(&after,goal);
            change(&mut rows,"Campaign aim","Not selected",goal.aim.title(),"This optional aim grants no bonuses and does not stop the simulation.");
            change(&mut rows,"Fixed target","Not selected",format!("{:.2} {}",goal.target,evaluation.metric),"Frozen from today's native country conditions.");
            change(&mut rows,"Required duration","Not selected",format!("{} consecutive settled days",goal.hold_days),"Every required condition must hold; a break resets consecutive progress.");
            let board=spheres_sim::campaign_aims::view(&after,nation);
            if let Some(offer)=board.offers.iter().find(|o|o.aim==goal.aim) {warnings.push(offer.description.clone());}
            warnings.extend(evaluation.blockers);
        }
    }
    for other in w.nations.iter().filter(|n| n.alive && n.id != nation) {
        change(&mut rows,&format!("Relations with {}",other.id.name()),format!("{:.1}",w.relation(nation,other.id)),format!("{:.1}",after.relation(nation,other.id)),"Immediate relationship change.");
        change(&mut rows,&format!("Defense pact with {}",other.id.name()),w.allied(nation,other.id),after.allied(nation,other.id),"A mutual-defense commitment; a later call to arms follows your response policy.");
    }
    if let Some(target) = target {
        change(&mut rows,&format!("Sanctions on {}",target.name()),if w.is_sanctioning(nation,target) {"Active"} else {"None"},if after.is_sanctioning(nation,target) {"Active"} else {"None"},"Your sanction status, independent of other countries.");
        let drag = |world: &WorldState| {
            let n=world.nation(target);
            format!("{:.3} pp/year",spheres_sim::economy::growth_terms(n,n.state_invest_gdp,n.interest_rate,&spheres_sim::economy::Conditions::of(world,target)).sanctions*100.0)
        };
        change(&mut rows,&format!("{} sanctions growth drag",target.name()),drag(w),drag(&after),"Annualized growth drag from all sanctioners at today's output shares, including the target's diplomacy ministry. Future output and other decisions can change it.");
    }
    for headline in after.headlines.iter().skip(w.headlines.len()) {
        // The old peg narrative names nominal penalties even at a cap; the
        // numeric rows above are the exact outcome this review must present.
        if kind != "break_currency_peg" {
            rows.push(json!({"label":"Dated result","before":"Pending","after":headline,"detail":w.date_str()}));
        }
    }
    if kind == "set_diplomatic_policy" && rows.is_empty() { warnings.push("These settings already match your standing policy.".into()); }
    let title = target.map_or_else(|| title.to_string(), |target| format!("{title}: {}",target.name()));
    json!({"valid":true,"reason":null,"title":title,"command":command,"description":description,"changes":rows,"warnings":warnings})
}

/// Add human-readable ledger dates without changing the native save schema.
pub(crate) fn agency_view(w: &WorldState, nation: NationId) -> Value {
    let mut view = serde_json::to_value(agency::view(w, nation)).unwrap();
    for row in view["history"].as_array_mut().unwrap() {
        let day = row["resolved_day"].as_i64().unwrap() as i32;
        let (y,m,d) = clock::date_from_day(day);
        row["date_label"] = json!(format!("{y:04}-{m:02}-{d:02}"));
        row["from_name"] = row["offer"]["from"].as_str().and_then(NationId::parse).map(|n| json!(n.name())).unwrap_or(json!("Former government"));
        row["title"] = json!(match row["offer"]["kind"]["kind"].as_str() {Some("defense_pact")=>"Defense pact",Some("trade_treaty")=>"Trade treaty",_=>"Call to arms"});
    }
    view
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanction_review_reads_the_actual_mitigated_growth_term() {
        let mut g=Game::new(13,Some(NationId::France));
        let n=g.world.nation_mut(NationId::Japan);
        let mut budget=n.budget_for(1990);
        budget.allocations[spheres_sim::world::BUDGET_DIPLOMACY]+=0.04;
        n.annual_budget=Some(budget);
        let command=json!({"kind":"sanction","target":"Japan"});
        let q=preview(&g,NationId::France,"decisions",&command);
        assert_eq!(q["valid"],true);
        let mut after=g.world.clone();
        spheres_sim::apply_command(&mut after,&parse_command(&g.world,&command,NationId::France).unwrap()).unwrap();
        let n=after.nation(NationId::Japan);
        let terms=spheres_sim::economy::growth_terms(n,n.state_invest_gdp,n.interest_rate,&spheres_sim::economy::Conditions::of(&after,n.id));
        assert!(terms.sanctions<spheres_sim::economy::growth_drag_of_sanctions(after.sanction_weight(n.id)));
        let row=q["changes"].as_array().unwrap().iter().find(|r|r["label"]=="Japan sanctions growth drag").unwrap();
        assert_eq!(row["after"],format!("{:.3} pp/year",terms.sanctions*100.0));
    }

    #[test]
    fn campaign_aim_review_shows_the_exact_frozen_target_and_duration() {
        let mut g=Game::new(13,Some(NationId::France));
        g.world.rules.daily_simulation=true;
        let command=json!({"kind":"choose_campaign_aim","aim":"prosperity"});
        let q=preview(&g,NationId::France,"decisions",&command);
        let mut after=g.world.clone();
        spheres_sim::apply_command(&mut after,&parse_command(&g.world,&command,NationId::France).unwrap()).unwrap();
        let goal=after.campaign_aims.active.as_ref().unwrap();
        let rows=q["changes"].as_array().unwrap();
        assert_eq!(rows.iter().find(|r|r["label"]=="Fixed target").unwrap()["after"],format!("{:.2} 1990 dollars per person",goal.target));
        assert_eq!(rows.iter().find(|r|r["label"]=="Required duration").unwrap()["after"],format!("{} consecutive settled days",goal.hold_days));
        assert!(q["warnings"].as_array().unwrap().iter().any(|r|r.as_str().unwrap().contains("inflation")));
        assert!(g.world.campaign_aims.active.is_none());
    }

    #[test]
    fn proven_review_refusal_and_uncertain_older_session_are_distinct() {
        let mut g=Game::new(13,Some(NationId::France));
        let command=json!({"kind":"improve","target":"Japan"});
        let q=preview(&g,NationId::France,"decisions",&command);
        let payload=json!({"commands":[command],"session_id":g.session_id,"review_kind":"decisions","review_token":q["review_token"]});
        g.world.nation_mut(NationId::France).political_capital=0.0;
        let error=crate::transport::immediate_request(&mut g,&payload).unwrap_err();
        assert!(error.requires_review && error.not_applied);
        g.session_id=crate::fresh_session_id();
        let error=crate::transport::immediate_request(&mut g,&payload).unwrap_err();
        assert!(error.requires_review && !error.not_applied);
    }

    #[test]
    fn reviewed_bank_receipt_has_one_persistent_dated_dispatch() {
        let mut g=Game::new(13,Some(NationId::France));g.world.player_set_rate=true;
        let command=json!({"kind":"resume_automatic_bank"});
        let q=preview(&g,NationId::France,"decisions",&command);
        let payload=json!({"commands":[command],"session_id":g.session_id,"client_id":"s10-bank","request_seq":1,"review_kind":"decisions","review_token":q["review_token"]});
        let before=g.log.len();
        crate::transport::immediate_request(&mut g,&payload).unwrap();
        assert_eq!(g.log.len(),before+1);
        assert!(g.log.last().unwrap().text.contains("central-bank"));
        assert_eq!(g.log.last().unwrap().date,g.world.date_str());
        crate::transport::immediate_request(&mut g,&payload).unwrap();
        assert_eq!(g.log.len(),before+1);
    }

    #[test]
    #[ignore = "Requires an explicitly named, read-only mature campaign archive"]
    fn mature_review_latency() {
        let path=std::env::var("SPHERES_S10_PERF_SAVE").expect("name the archive");
        let source=std::fs::read_to_string(&path).unwrap();
        let mut g=crate::storage::decode(&source).unwrap();
        // Loading warms only derived resources. The benchmark sends no command.
        spheres_sim::resources::warm(&mut g.world);
        let saved=spheres_sim::save(&g.world);
        let nation=g.world.player.expect("saved player");
        let policy=json!({"kind":"set_diplomatic_policy","policy":{"defense_pacts":"review","trade_treaties":"review","calls_to_arms":"review"}});
        let board=crate::government_json(&g.world,nation);
        let government=board["actions"].as_array().unwrap().iter().find(|a|a["refusal"].is_null()).expect("native legal action")["command"].clone();
        let mut reports=vec![];
        for (kind,command) in [("decisions",policy),("government",government)] {
            let mut samples=vec![];
            for i in 0..24 {
                let now=std::time::Instant::now();
                let q=preview(&g,nation,kind,&command);
                assert_eq!(q["valid"],true,"{q}");
                if i>=3 {samples.push(now.elapsed().as_secs_f64()*1000.0);}
            }
            let mut sorted=samples.clone();sorted.sort_by(f64::total_cmp);
            let p95=sorted[19];let max=sorted[20];
            let report=json!({"kind":kind,"samples_ms":samples,"p95_ms":p95,"max_ms":max,"p95_limit_ms":300,"max_limit_ms":750});
            println!("S10_REVIEW_LATENCY {report}");
            reports.push(report);
            assert!(p95<=300.0&&max<=750.0,"Predeclared native review latency limits failed");
        }
        assert_eq!(spheres_sim::save(&g.world),saved);
        assert_eq!(std::fs::read_to_string(&path).unwrap(),source);
        println!("S10_REVIEW_CONTEXT {}",json!({"source":path,"nation":nation,"date":g.world.date_str(),"warmups":3,"samples":21,"scope":"native preview generation including cloned command and complete-world token; excludes HTTP and browser","pure":true,"rows":reports}));
    }
}
