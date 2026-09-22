//! Campaign outcomes and explicit continuation. Presentation metadata belongs to
//! the campaign envelope; goal evaluation and sovereignty remain in the sim.
use crate::Game;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use spheres_sim::{campaign_aims, domination, world::NationId};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub(crate) struct Journey {
    pub beyond_2035: bool,
    pub observing: bool,
    pub transitions: Vec<Transition>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub(crate) struct Transition {
    pub from: NationId,
    pub to: NationId,
    pub date: String,
}

fn alive(g: &Game) -> bool {
    g.world.player.is_some_and(|id| g.world.nation_opt(id).is_some_and(|n| n.alive))
}
pub(crate) fn pause_reason(g: &Game) -> Option<String> {
    if g.world.player.is_some() && g.world.year > 2035 && !g.journey.beyond_2035 {
        return Some("The 2035 campaign horizon is complete. Open Campaign to review your result or continue beyond 2035 in sandbox.".into());
    }
    if g.world.player.is_some() && !alive(g) && !g.journey.observing {
        return Some("Your government no longer exists. Open Campaign to review its history and available continuation choices.".into());
    }
    None
}

fn successors(g: &Game) -> Vec<NationId> {
    let w = &g.world;
    if alive(g) { return vec![]; }
    // These are the simulation's authored dissolution families, not arbitrary
    // country switching. Flags AND living seats must both exist in this world.
    let family: &[NationId] = match w.player {
        Some(NationId::USSR) if w.has_flag("ussr_dissolved") => &[NationId::Russia, NationId::Ukraine, NationId::Belarus, NationId::Moldova,
            NationId::Georgia, NationId::Armenia, NationId::Azerbaijan, NationId::Kazakhstan, NationId::Uzbekistan, NationId::Turkmenistan,
            NationId::Kyrgyzstan, NationId::Tajikistan, NationId::Lithuania, NationId::Latvia, NationId::Estonia],
        Some(NationId::Yugoslavia) if w.has_flag("yugoslavia_dissolved") =>
            &[NationId::Serbia, NationId::Croatia, NationId::Slovenia, NationId::Bosnia, NationId::Macedonia, NationId::Montenegro],
        _ => &[],
    };
    family.iter().copied().filter(|id| w.nation_opt(*id).is_some_and(|n| n.alive)).collect()
}

pub(crate) fn is_command(v: &Value) -> bool { v["kind"] == "continue_campaign" }

pub(crate) fn validate_player(g: &Game, payload: &Value) -> Result<(), String> {
    if (!g.journey.transitions.is_empty() || payload.get("player_context").is_some())
        && payload["player_context"] != json!(g.world.player) {
        return Err("The player government changed. Refresh the campaign before issuing orders for the successor.".into());
    }
    Ok(())
}

/// Runs through the existing immediate-command receipt channel. Requires the
/// exact displayed date and player, including on legacy receipt-less clients.
pub(crate) fn apply(g: &mut Game, payload: &Value) -> Result<(), String> {
    let list = payload["commands"].as_array().filter(|a| a.len() == 1)
        .ok_or("Review one campaign continuation at a time.")?;
    let c = &list[0];
    if payload["session_id"].as_str() != Some(&g.session_id)
        || c["date"].as_str() != Some(g.world.date_str().as_str())
        || c["player"] != json!(g.world.player) {
        return Err("The campaign changed. Review the current result before continuing.".into());
    }
    match c["action"].as_str() {
        Some("beyond_2035") if g.world.year > 2035 && !g.journey.beyond_2035 => {
            g.journey.beyond_2035 = true;
            g.record("CAMPAIGN HORIZON: 2035 is complete. The player chooses open-ended sandbox play; this does not award campaign certification or a victory.".into());
        }
        Some("observe") if !alive(g) && !g.journey.observing => {
            g.journey.observing = true;
            g.record(format!("CAMPAIGN CONTINUATION: {}'s government has ended. The player follows the surviving world as an observer.", g.world.player.unwrap().name()));
        }
        Some("successor") => {
            let target = c["target"].as_str().and_then(NationId::parse)
                .filter(|id| successors(g).contains(id))
                .ok_or("Choose a living successor of this dissolved government.")?;
            let from = g.world.player.ok_or("Choose a country first.")?;
            if let Some(goal) = g.world.campaign_aims.active.take() {
                let outcome = if goal.completed_day.is_some() { "achieved" } else { "government ended" };
                g.world.campaign_aims.history.push(campaign_aims::Record {
                    goal, ended_day: spheres_sim::clock::absolute_day(&g.world), outcome: outcome.into(),
                });
            }
            g.journey.transitions.push(Transition { from, to: target, date: g.world.date_str() });
            g.journey.observing = false;
            g.world.player = Some(target);
            g.world.player_set_rate = false;
            g.record(format!("CAMPAIGN SUCCESSION: The player continues from {} as {}. The successor keeps its existing economy, government, military and obligations; no resources or achievements are awarded.", from.name(), target.name()));
        }
        _ => return Err("That continuation is not available in the current campaign.".into()),
    }
    Ok(())
}

pub(crate) fn summary(g: &Game) -> Value {
    let w = &g.world;
    let Some(me) = w.player else { return Value::Null; };
    let family = successors(g);
    let goal = w.campaign_aims.active.as_ref().filter(|goal| goal.nation == me);
    let achieved = goal.is_some_and(|goal| goal.completed_day.is_some());
    let victory = alive(g) && domination::status(w, me).victory;
    let overlord = domination::direct_overlord(w, me);
    let (status, title, detail) = if !alive(g) {
        if family.is_empty() { ("government_ended", "Your government's chapter has ended", "This government no longer has a living seat. Its results and history remain available. You can follow the surviving world as an observer or start a new campaign from the menu.") }
        else { ("succession", "A new chapter after dissolution", "The former government has dissolved. Choose a surviving successor to govern its actual current state. Earlier achievements stay with the country that earned them.") }
    } else if w.year > 2035 && !g.journey.beyond_2035 {
        ("horizon", "The 2035 campaign horizon is complete", "All days through 31 December 2035 have settled. Review your goals and national record, save this result, or choose open-ended sandbox play. Reaching this date does not by itself mean victory or certification.")
    } else if victory { ("victory", "World domination achieved", "Every surviving sovereign government is formally subordinate to yours. The campaign can keep running. No economic or military bonuses are awarded.") }
    else if achieved { ("aim_achieved", "Your campaign aim is achieved", "This result stays recorded even if conditions later change. Continue playing, or set the completed aim aside to choose a new direction.") }
    else if overlord.is_some() { ("subordinate", "Your government is subordinate", "A surviving government can still be governed. Formal subordination is recorded by the sovereignty system; it is not a deleted country or an automatic campaign restart.") }
    else { ("active", "Write your country's next chapter", "Choose an aim, follow the conditions it requires, and review what actually changes in your country. Goals do not award invented resources or bonuses.") };
    let mut actions = vec![];
    let mut action = |kind: &str, label: String, target: Option<NationId>| {
        actions.push(json!({"label":label,"command":{"kind":"continue_campaign","action":kind,
            "target":target,"player":me,"date":w.date_str()}}));
    };
    if w.year > 2035 && !g.journey.beyond_2035 { action("beyond_2035", "Continue beyond 2035 · sandbox".into(), None); }
    if !alive(g) {
        for id in family { action("successor", format!("Continue as {}", id.name()), Some(id)); }
        if !g.journey.observing { action("observe", "Follow the world as an observer".into(), None); }
    }
    json!({"status":status,"title":title,"detail":detail,"nation":me,"nation_name":me.name(),
        "date":w.date_str(),"paused_reason":pause_reason(g),"actions":actions,
        "alive":alive(g),"overlord":overlord,"observing":g.journey.observing,
        "beyond_2035":g.journey.beyond_2035,"transitions":g.journey.transitions,
        "horizon":"31 Dec 2035","horizon_reached":w.year>2035})
}

/// Indexed archive paging keeps old events reachable even after MAX_LOG. The
/// cursor is an append-stable log offset, so new dispatches cannot shift pages.
pub(crate) fn view(g: &Game, url: &str) -> Value {
    let query = url.split_once('?').map(|(_, q)| q).unwrap_or("");
    let param = |key| query.split('&').find_map(|s| s.split_once('=').filter(|(k, _)| *k == key).map(|(_, v)| v));
    let category = param("category").filter(|v| matches!(*v, "economy" | "politics" | "war" | "diplomacy" | "other"));
    let end = param("before").and_then(|v| v.parse::<usize>().ok()).unwrap_or(g.log.len()).min(g.log.len());
    let mut nations: Vec<NationId> = g.journey.transitions.iter().map(|t| t.from).collect();
    nations.extend(g.world.player);
    let matches = |e: &&crate::history::Event| (category.is_none() || category == Some(e.cat.as_str()))
        && e.tags.iter().any(|n| nations.contains(n));
    let total = g.log.iter().filter(matches).count();
    let mut entries = vec![];
    let mut cursor = 0;
    for i in (0..end).rev() {
        if matches(&&g.log[i]) { entries.push(&g.log[i]); }
        if entries.len() == 40 { cursor = i; break; }
    }
    let mut metrics = vec![];
    if let Some(me) = g.world.player {
        let first = g.history.iter().find_map(|s| s.rows.iter().find(|(id, _)| *id == me).map(|(_, row)| (s, row)));
        let last = g.history.iter().rev().find_map(|s| s.rows.iter().find(|(id, _)| *id == me).map(|(_, row)| (s, row)));
        if let (Some((start, a)), Some((finish, b))) = (first, last) {
            for (id, label, before, after, unit) in [
                ("gdp", "Real annual output", a.gdp, b.gdp, "billion 1990 dollars"),
                ("debt", "Public debt / GDP", a.debt*100., b.debt*100., "%"),
                ("stability", "Government stability", a.stability, b.stability, " / 100"),
                ("mil", "Military strength", a.mil, b.mil, "model index"),
            ] { metrics.push(json!({"id":id,"label":label,"before":before,"after":after,"unit":unit,"from":start.date_label(),"to":finish.date_label()})); }
        }
    }
    let aims = g.world.player.map(|me| campaign_aims::view(&g.world, me));
    json!({"session_id":g.session_id,"summary":summary(g),"metrics":metrics,"aims":aims,
        "legacy_aims":g.world.campaign_aims.history.iter().filter(|r| nations.contains(&r.goal.nation)).collect::<Vec<_>>(),
        "events":entries,"before":cursor,"total":total,"category":category,
        "history_note":"Recorded changes describe outcomes, not a causal attribution. Dispatches below explain events the simulation actually recorded. Older simulation-only saves cannot reconstruct missing history."})
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{save, storage, transport};
    fn daily(id: NationId) -> Game {
        let mut g = Game::new(1990, Some(id));
        g.world.rules.daily_simulation = true;
        g.world.day = 1;
        g.history.clear(); g.snapshot(); g
    }
    fn order(g: &Game, action: &str, target: Option<NationId>) -> Value {
        json!({"session_id":g.session_id,"client_id":"journey-test","request_seq":1,
            "commands":[{"kind":"continue_campaign","action":action,"target":target,
                "player":g.world.player,"date":g.world.date_str()}]})
    }
    fn dissolved(id: NationId) -> Game {
        let mut g = daily(id);
        campaign_aims::choose(&mut g.world, id, campaign_aims::Aim::Prosperity).unwrap();
        g.world.nation_mut(id).stability = 0.;
        g.world.nation_mut(id).separatism = 1.;
        spheres_sim::politics::tick(&mut g.world);
        for h in g.world.headlines.clone() {g.record(h);}
        assert!(!alive(&g)); g
    }
    #[test]
    fn s21_every_day_of_2035_settles_then_waits_for_explicit_sandbox() {
        let mut g = daily(NationId::France);
        g.world.year = 2035; g.world.month = 12; g.world.day = 30;
        g.history.clear();g.snapshot();
        let mut count=0;
        while g.world.year==2035 {
            g.advance_days(1,vec![]);count+=1;assert!(count<4);
        }
        assert_eq!(count,2);assert_eq!((g.world.month,g.world.day),(1,1));
        assert_eq!(summary(&g)["status"],"horizon");
        let frozen=save(&g.world);
        let payload=json!({"days":1,"commands":[]});
        assert!(transport::advance_request(&mut g,&payload).is_err());
        assert_eq!(g.advance_days(10,vec![]).0,true);
        assert_eq!(save(&g.world),frozen);
        let p=order(&g,"beyond_2035",None);
        transport::immediate_request(&mut g,&p).unwrap();
        assert_eq!(save(&g.world),frozen,"continuation adds no simulation rewards");
        assert_eq!(transport::immediate_request(&mut g,&p).unwrap()["command_replayed"],true);
        assert_eq!(g.log.iter().filter(|e|e.text.starts_with("CAMPAIGN HORIZON:")).count(),1);
        transport::advance_request(&mut g,&payload).unwrap();
        assert_eq!(g.world.day,2);
        let loaded=storage::decode(&storage::encode(&g).unwrap()).unwrap();
        assert!(loaded.journey.beyond_2035);assert!(pause_reason(&loaded).is_none());
        assert_eq!(g.log,loaded.log);assert_eq!(g.history,loaded.history);
    }
    #[test]
    fn s21_successors_keep_existing_state_and_old_aims_stay_with_old_country() {
        for (from,to) in [(NationId::USSR,NationId::Russia),(NationId::Yugoslavia,NationId::Serbia)] {
            let mut g=dissolved(from);
            assert_eq!(summary(&g)["status"],"succession");
            let p=order(&g,"successor",Some(to));
            let before=g.world.clone();
            transport::immediate_request(&mut g,&p).unwrap();
            let mut expected=before;
            expected.player=Some(to);expected.player_set_rate=false;
            expected.campaign_aims=g.world.campaign_aims.clone();
            assert_eq!(save(&g.world),save(&expected),"only control and the aim record may change");
            assert!(g.world.campaign_aims.active.is_none());
            assert_eq!(g.world.campaign_aims.history[0].goal.nation,from);
            assert_eq!(g.world.campaign_aims.history[0].outcome,"government ended");
            assert_eq!(g.journey.transitions.len(),1);
            transport::immediate_request(&mut g,&p).unwrap();
            assert_eq!(g.journey.transitions.len(),1,"lost response cannot switch twice");
            let stale=json!({"player_context":from,"commands":[{"kind":"improve","target":"Japan"}]});
            let frozen=save(&g.world);
            assert!(transport::immediate_request(&mut g,&stale).is_err());
            assert!(transport::advance_request(&mut g,&json!({"days":1,"player_context":from})).is_err());
            assert_eq!(save(&g.world),frozen,"another tab cannot command the successor as its former player");
            let loaded=storage::decode(&storage::encode(&g).unwrap()).unwrap();
            assert_eq!(loaded.journey,g.journey);assert_eq!(loaded.log,g.log);
            assert_eq!(loaded.world.player,Some(to));
        }
    }
    #[test]
    fn s21_stale_unrelated_unborn_and_batched_continuations_are_atomic_refusals() {
        let mut g=dissolved(NationId::USSR);
        g.world.nation_mut(NationId::Ukraine).alive=false;
        let good=order(&g,"successor",Some(NationId::Russia));
        let mut stale=good.clone();stale["commands"][0]["date"]="stale".into();
        let mut session=good.clone();session["session_id"]="old-session".into();
        let mut batch=good.clone();batch["commands"].as_array_mut().unwrap().push(json!({"kind":"improve","target":"France"}));
        for p in [stale,session,batch,order(&g,"successor",Some(NationId::France)),order(&g,"successor",Some(NationId::Ukraine)),order(&g,"beyond_2035",None)] {
            let before=save(&g.world);let log=g.log.clone();let journey=g.journey.clone();
            assert!(transport::immediate_request(&mut g,&p).is_err());
            assert_eq!(save(&g.world),before);assert_eq!(g.log,log);assert_eq!(g.journey,journey);
        }
    }
    #[test]
    fn s21_defeat_without_successor_can_be_followed_but_cannot_issue_orders() {
        let mut g=daily(NationId::France);g.world.nation_mut(NationId::France).alive=false;
        assert_eq!(summary(&g)["status"],"government_ended");
        assert!(successors(&g).is_empty());assert!(pause_reason(&g).is_some());
        let p=order(&g,"observe",None);transport::immediate_request(&mut g,&p).unwrap();
        assert!(pause_reason(&g).is_none());
        assert!(transport::immediate_request(&mut g,&json!({"commands":[{"kind":"improve","target":"Japan"}]})).is_err());
        assert!(transport::advance_request(&mut g,&json!({"days":1,"commands":[{"kind":"improve","target":"Japan"}]})).is_err());
        let restored=storage::decode(&storage::encode(&g).unwrap()).unwrap();
        assert!(restored.journey.observing);assert!(pause_reason(&restored).is_none());
    }
    #[test]
    fn s21_completed_peaceful_and_military_aims_change_only_the_record() {
        for aim in [campaign_aims::Aim::Prosperity,campaign_aims::Aim::Domination] {
            let mut g=daily(NationId::France);
            campaign_aims::choose(&mut g.world,NationId::France,aim).unwrap();
            if aim==campaign_aims::Aim::Prosperity {
                g.world.nation_mut(NationId::France).gdp *= 1.3;
                g.world.campaign_aims.active.as_mut().unwrap().held_days=89;
            } else {
                for n in &mut g.world.nations { if n.id!=NationId::France {n.alive=false;} }
            }
            let mut expected=g.world.clone();
            campaign_aims::tick(&mut g.world);
            assert!(g.world.campaign_aims.active.as_ref().unwrap().completed_day.is_some());
            expected.campaign_aims=g.world.campaign_aims.clone();expected.headlines=g.world.headlines.clone();
            assert_eq!(save(&g.world),save(&expected));
            assert!(matches!(summary(&g)["status"].as_str(),Some("aim_achieved"|"victory")));
            let before=save(&g.world);view(&g,"?");assert_eq!(save(&g.world),before);
        }
    }
    #[test]
    fn s21_archive_filters_old_events_and_survives_save_without_fabricated_history() {
        let mut g=daily(NationId::France);
        for i in 0..4105 {g.log.push(crate::history::Event{t:0,date:"1 Jan 1990".into(),cat:if i%2==0{"economy"}else{"war"}.into(),tags:vec![NationId::France],text:format!("Authored archive fixture {i}")});}
        let a=view(&g,"?category=economy");assert_eq!(a["total"],2053);assert_eq!(a["events"].as_array().unwrap().len(),40);
        let mut before=a["before"].as_u64().unwrap();let mut seen=40;
        while before>0 {let v=view(&g,&format!("?category=economy&before={before}"));seen+=v["events"].as_array().unwrap().len();before=v["before"].as_u64().unwrap();}
        assert_eq!(seen,2053);
        let mut restored=storage::decode(&storage::encode(&g).unwrap()).unwrap();
        assert_eq!(restored.log,g.log);assert_eq!(restored.history,g.history);
        restored.history.clear();assert_eq!(view(&restored,"?")["metrics"],json!([]));
        let mut file:Value=serde_json::from_str(&storage::encode(&g).unwrap()).unwrap();file.as_object_mut().unwrap().remove("journey");
        assert_eq!(storage::decode(&file.to_string()).unwrap().journey,Journey::default());
    }
    #[test]
    #[ignore="Exports authored S21 boundary fixtures to a NEW SPHERES_S21_FIXTURE_DIR"]
    fn s21_export_review_fixtures() {
        let path=std::path::PathBuf::from(std::env::var_os("SPHERES_S21_FIXTURE_DIR").unwrap());
        assert!(path.is_absolute()&&!path.exists());std::fs::create_dir(&path).unwrap();
        let mut g=Game::new(1990,Some(NationId::France));crate::fresh_play_rules(&mut g).unwrap();g.history.clear();g.snapshot();
        campaign_aims::choose(&mut g.world,NationId::France,campaign_aims::Aim::Prosperity).unwrap();
        for h in g.world.headlines.clone(){g.record(h);}
        std::fs::write(path.join("active.json"),storage::encode(&g).unwrap()).unwrap();
        g.world.year=2035;g.world.month=12;g.world.day=31;g.history.clear();g.snapshot();
        std::fs::write(path.join("endpoint.json"),storage::encode(&g).unwrap()).unwrap();
        let mut successor=Game::new(1990,Some(NationId::USSR));crate::fresh_play_rules(&mut successor).unwrap();
        successor.world.nation_mut(NationId::USSR).stability=0.;successor.world.nation_mut(NationId::USSR).separatism=1.;
        spheres_sim::politics::tick(&mut successor.world);for h in successor.world.headlines.clone(){successor.record(h);}
        successor.history.clear();successor.snapshot();
        std::fs::write(path.join("succession.json"),storage::encode(&successor).unwrap()).unwrap();
    }
}
