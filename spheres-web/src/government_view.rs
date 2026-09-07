//! Presentation-only government briefing and exact immediate command reviews.
//! The existing government action list is the allowlist; simulation commands
//! remain the only writers, and reviews execute against a disposable clone.

use serde_json::{json, Value};
use spheres_sim::blocs;
use spheres_sim::government::{self as gov, Bloc, Pillar};
use spheres_sim::world::{Nation, NationId, WorldState};

fn percent(v: f64) -> String {
    format!("{:.1}%", v * 100.0)
}
fn points(v: f64) -> String {
    format!("{v:.1}")
}
fn money(v: f64) -> String {
    format!("${:.2} million", v * 1000.0)
}

fn party_name(id: NationId, party: &str) -> String {
    gov::party_spec(id, party).map_or_else(|| party.to_string(), |p| p.name.to_string())
}

fn cabinet(w: &WorldState, id: NationId) -> String {
    gov::state(w, id)
        .map(|g| {
            g.coalition
                .iter()
                .map(|p| party_name(id, p))
                .collect::<Vec<_>>()
                .join(" + ")
        })
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "No seated cabinet".into())
}

fn election(w: &WorldState, id: NationId) -> String {
    gov::state(w, id)
        .filter(|_| gov::is_electoral(w, id))
        .map(|g| g.next_election)
        .filter(|(y, m)| *y != 0 && *m != 0)
        .map_or_else(|| "Not scheduled".into(), |(y, m)| format!("{y:04}-{m:02}"))
}

fn officeholder(w: &WorldState, id: NationId) -> String {
    blocs::leader(w, id).map_or_else(
        || "No officeholder record".into(),
        |leader| {
            let holder = leader
                .name
                .or(leader.described)
                .unwrap_or_else(|| "The officeholder".into());
            format!("{}: {}", leader.office, holder)
        },
    )
}

fn institution_name(w: &WorldState, id: NationId, pillar: Pillar) -> String {
    gov::polity_in(w, id)
        .and_then(|p| p.pillars.iter().find(|s| s.pillar == pillar))
        .map_or_else(|| pillar.key().to_string(), |p| p.name.to_string())
}

fn category(kind: &str) -> &'static str {
    match kind {
        "invite" | "expel" | "call_election" => "coalition",
        "secure_pillar" => "institutions",
        _ => "reform",
    }
}

fn action_key(a: &Value) -> String {
    let c = &a["command"];
    let kind = c["kind"].as_str().unwrap_or("unknown");
    let target = ["party", "pillar", "bloc", "id"]
        .iter()
        .find_map(|key| c[*key].as_str());
    target.map_or_else(|| kind.to_string(), |target| format!("{kind}:{target}"))
}

fn action_index(actions: &[Value], kind: &str) -> Option<usize> {
    actions
        .iter()
        .position(|a| a["kind"] == kind && a["refusal"].is_null())
}

/// Add only presentation fields. Existing action order, commands, prices,
/// refusals and the political watch remain intact for every caller.
pub(crate) fn enrich(w: &WorldState, id: NationId, value: &mut Value) {
    if let Some(actions) = value["actions"].as_array_mut() {
        for action in actions {
            action["key"] = json!(action_key(action));
            action["category"] = json!(category(action["kind"].as_str().unwrap_or("")));
        }
    }
    let Some(n) = w.nation_opt(id).filter(|n| n.alive) else {
        value["briefing"] = json!({"date_label":w.date_str(),"summary":"This government is not active in the current campaign.","stats":[],"attention":[],"drivers":[],"links":[]});
        return;
    };
    let Some(g) = gov::state(w, id) else {
        value["briefing"] = json!({"date_label":w.date_str(),"summary":"No government record is available for this country.","stats":[],"attention":[],"drivers":[],"links":[]});
        return;
    };
    let electoral = gov::is_electoral(w, id);
    let actions = value["actions"].as_array().cloned().unwrap_or_default();
    let mut stats = vec![
        json!({"key":"political_capital","label":"Political capital","value":format!("{} / 100",points(n.political_capital)),"detail":"Decisions spend this stock. Growth, stable prices, order, war and your government determine how it recovers."}),
        json!({"key":"stability","label":"National stability","value":format!("{} / 100",points(n.stability)),"detail":"Order affects public support, institutional loyalty and the government's survival."}),
    ];
    if w.rules.ideology_blocs {
        stats.push(json!({"key":"discontent","label":"Public discontent","value":percent(blocs::discontent(w,id)),"detail":"Reads the same order, price, growth and war pressures that move political support."}));
    }
    let summary = if electoral {
        stats.push(json!({"key":"government_seats","label":"Government seats","value":percent(g.government_seats()),"detail":"The cabinet's share of the chamber from the last election; public support changes between votes."}));
        stats.push(json!({"key":"upkeep","label":"Coalition upkeep","value":format!("{} PC / month",points(gov::upkeep(w,id))),"detail":format!("Coalition strain is {}. Government composition changes the standing target by {:+.1} points.",points(gov::strain(w,id)),gov::standing_modifier(w,id))}));
        stats.push(json!({"key":"election","label":"Next election","value":election(w,id),"detail":format!("{} completed months in office. An early election uses today's public support immediately.",g.months_in_office)}));
        format!("{} holds {} of the chamber. Coalition decisions change both your votes and the political cost of governing.",cabinet(w,id),percent(g.government_seats()))
    } else {
        stats.push(json!({"key":"loyalty","label":"Mean institutional loyalty","value":percent(g.mean_loyalty()),"detail":"Institutions respond to the country's policies and conditions; a one-time payment does not permanently change their targets."}));
        stats.push(json!({"key":"coup_pressure","label":"Regime pressure","value":points(g.coup_pressure),"detail":"Recorded pressure from armed institutions. Ordinary regime coups remain part of the simulation."}));
        stats.push(json!({"key":"office","label":"Time in office","value":format!("{} months",g.months_in_office),"detail":"This government rests on institutions rather than an electoral majority."}));
        let institution = gov::polity_in(w, id).map_or("The government", |p| p.ruling);
        format!("{institution} rests on the institutions below. Their loyalty responds to spending, prices, growth, order and war.")
    };
    let mut attention = vec![];
    if electoral {
        if g.government_seats() < 0.5 {
            attention.push(json!({"id":"minority","tone":"attention","title":"Your cabinet lacks a majority","detail":format!("The government holds {} of the chamber. Review a partner's added seats alongside the new coalition upkeep before inviting them.",percent(g.government_seats())),"action_index":action_index(&actions,"invite")}));
        } else {
            attention.push(json!({"id":"majority","tone":"good","title":"Your cabinet holds a majority","detail":format!("{} of the chamber supports the cabinet. Each partner still contributes to its ongoing political cost.",percent(g.government_seats()))}));
        }
        attention.push(json!({"id":"electoral_survival","tone":"info","title":"Elections are tied to the country you govern","detail":format!("The scheduled vote is {}. Cabinet seats and national stability can also bring an ordinary government down early; this remains active even when ideological takeover routes are disabled.",election(w,id)),"action_index":action_index(&actions,"call_election")}));
    } else if let Some((pillar, loyalty)) = g.weakest_armed() {
        let index = actions.iter().position(|a| {
            a["command"]["kind"] == "secure_pillar"
                && a["command"]["pillar"] == pillar.key()
                && a["refusal"].is_null()
        });
        attention.push(json!({"id":"institution_pressure","tone":if g.coup_pressure>0.0 {"attention"} else {"info"},"title":format!("Keep {} in view",institution_name(w,id,pillar)),"detail":format!("Your least loyal armed institution is at {} loyalty; recorded regime pressure is {}. Ordinary regime survival is live. A reviewed loyalty payment costs both political capital and public money.",percent(loyalty),points(g.coup_pressure)),"action_index":index,"route":"budget"}));
    }
    let unaffordable = actions.iter().filter(|a| a["affordable"] == false).count();
    if unaffordable > 0 {
        attention.push(json!({"id":"standing","tone":"attention","title":"Political capital limits your options","detail":format!("{unaffordable} listed decisions exceed your current standing. Their exact costs and refusals are shown below; governing conditions determine recovery."),"route":"budget"}));
    }
    if w.rules.ideology_blocs && !w.rules.ideology_takeover {
        attention.push(json!({"id":"ideology_watch","tone":"info","title":"The ideological takeover watch is inactive","detail":"Its gauges describe current conditions, but those takeover routes remain closed for calibration. This does not disable ordinary elections, cabinet failures or regime coups."}));
    }
    let drivers = vec![
        json!({"id":"prices","label":"Prices","value":percent(n.inflation),"detail":"Inflation changes incumbent support and political standing; business institutions also respond to rising prices.","route":"budget"}),
        json!({"id":"growth","label":"Economic growth","value":percent(n.growth_last),"detail":"Delivered growth affects incumbent support, political standing and business loyalty. Review construction and the economy behind it.","route":"construction"}),
        json!({"id":"order","label":"Order and cohesion","value":format!("{} stability · {} separatism",points(n.stability),percent(n.separatism)),"detail":"Disorder and separatism feed public discontent and the appeal of opposition movements; reforms can change both.","route":"budget"}),
        json!({"id":"war","label":"War exhaustion","value":percent(n.war_exhaustion),"detail":"War exhaustion lowers political standing and army loyalty and shifts support away from the government.","route":"military"}),
    ];
    let links = vec![
        json!({"id":"budget","label":"Ministry budgets","detail":"Review defense, pensions and the services behind standing and loyalty.","route":"budget"}),
        json!({"id":"cash_flow","label":"Public finances","detail":"See the treasury and debt that fund one-time institutional payments.","route":"cash_flow"}),
        json!({"id":"construction","label":"Construction","detail":"Review the investments behind the country's economic performance.","route":"construction"}),
        json!({"id":"diplomacy","label":"Diplomacy","detail":"Political reforms also change relations abroad.","route":"diplomacy"}),
    ];
    value["briefing"] = json!({"date_label":w.date_str(),"summary":summary,"stats":stats,"attention":attention,"drivers":drivers,"links":links});
}

fn invalid(command: &Value, reason: &str) -> Value {
    json!({"valid":false,"reason":reason,"title":"Review government decision","kind":command["kind"],"command":command,"price_pc":0.0,"money_cost_bn":0.0,"description":"","effects":[],"changes":[],"warnings":[]})
}

fn add_change(
    changes: &mut Vec<Value>,
    key: impl Into<String>,
    label: impl Into<String>,
    before: String,
    after: String,
    detail: &str,
) {
    if before != after {
        changes.push(json!({"key":key.into(),"label":label.into(),"before":before,"after":after,"detail":detail}));
    }
}

fn net_public_balance(n: &Nation) -> f64 {
    if n.on_the_books() {
        n.treasury_bn.unwrap_or(0.0) - n.debt_bn.unwrap_or(0.0)
    } else {
        -n.debt_gdp * n.gdp
    }
}

/// Review one exact action that the existing government page currently serves.
/// No simulation date advances, and no caller can smuggle a different command
/// or target nation through this endpoint.
pub(crate) fn preview(w: &WorldState, id: NationId, command: &Value) -> Value {
    if w.player != Some(id) {
        return invalid(command, "Only your own government can review a decision.");
    }
    if w.nation_opt(id).is_none_or(|n| !n.alive) || gov::state(w, id).is_none() {
        return invalid(
            command,
            "This government is not active in the current campaign.",
        );
    }
    let board = super::government_json(w, id);
    let Some(action) = board["actions"]
        .as_array()
        .and_then(|actions| actions.iter().find(|a| a["command"] == *command))
    else {
        return invalid(
            command,
            "Choose an exact decision from the current government page.",
        );
    };
    let Some(cmd) = super::parse_command(w, command, id) else {
        return invalid(command, "This government decision cannot be read.");
    };
    let mut after = w.clone();
    if let Err(reason) = spheres_sim::apply_command(&mut after, &cmd) {
        let mut v = invalid(command, &reason);
        v["title"] = action["label"].clone();
        v["kind"] = action["kind"].clone();
        v["description"] = action["detail"].clone();
        return v;
    }
    let before_n = w.nation(id);
    let after_n = after.nation(id);
    let before_g = gov::state(w, id).unwrap();
    let after_g = gov::state(&after, id).unwrap();
    let price_pc = (before_n.political_capital - after_n.political_capital).max(0.0);
    let money_cost = (net_public_balance(before_n) - net_public_balance(after_n)).max(0.0);
    let mut changes = vec![];
    add_change(
        &mut changes,
        "political_capital",
        "Political capital",
        points(before_n.political_capital),
        points(after_n.political_capital),
        "The actual immediate deduction, after the simulation's limits.",
    );
    if before_n.on_the_books() && after_n.on_the_books() {
        add_change(
            &mut changes,
            "treasury",
            "Treasury",
            money(before_n.treasury_bn.unwrap()),
            money(after_n.treasury_bn.unwrap()),
            "Public cash used immediately by this decision.",
        );
        add_change(
            &mut changes,
            "debt",
            "Public debt",
            money(before_n.debt_bn.unwrap()),
            money(after_n.debt_bn.unwrap()),
            "Any payment beyond available treasury cash is borrowed.",
        );
    } else {
        add_change(
            &mut changes,
            "debt",
            "Debt / GDP",
            percent(before_n.debt_gdp),
            percent(after_n.debt_gdp),
            "This campaign currently records public debt as a share of GDP.",
        );
    }
    for (key,label,b,a,detail) in [
        ("stability","National stability",points(before_n.stability),points(after_n.stability),"Immediate change to order; this feeds the existing political and economic systems."),
        ("authoritarianism","Authoritarianism",percent(before_n.authoritarianism),percent(after_n.authoritarianism),"The political system reads the resulting level."),
        ("separatism","Separatism",percent(before_n.separatism),percent(after_n.separatism),"Immediate change to the country's cohesion pressure."),
        ("system","Government accountability",if gov::is_electoral(w,id) {"Electoral"} else {"Institutional"}.into(),if gov::is_electoral(&after,id) {"Electoral"} else {"Institutional"}.into(),"Who the government must answer to after this decision."),
        ("cabinet","Cabinet",cabinet(w,id),cabinet(&after,id),"The seated coalition after the command; a dormant cabinet remains a record in a regime."),
        ("seats","Government seats",percent(before_g.government_seats()),percent(after_g.government_seats()),"Seat arithmetic from the actual command. Public support and seats are separate stocks."),
        ("strain","Coalition strain",points(gov::strain(w,id)),points(gov::strain(&after,id)),"Strain determines the cost and standing consequences of holding this cabinet together."),
        ("upkeep","Coalition upkeep",format!("{} PC / month",points(gov::upkeep(w,id))),format!("{} PC / month",points(gov::upkeep(&after,id))),"Ongoing upkeep of the resulting cabinet; future country conditions can still change it."),
        ("composition","Government standing modifier",format!("{:+.1}",gov::standing_modifier(w,id)),format!("{:+.1}",gov::standing_modifier(&after,id)),"The current composition's contribution to the standing target, including any electoral honeymoon; not a lump-sum PC award."),
        ("election","Next election",election(w,id),election(&after,id),"The election date recorded immediately by this command."),
        ("coup_pressure","Regime pressure",points(before_g.coup_pressure),points(after_g.coup_pressure),"Recorded pressure changes now; this is not a probability forecast."),
    ] { add_change(&mut changes,key,label,b,a,detail); }
    if w.rules.ideology_blocs {
        add_change(
            &mut changes,
            "officeholder",
            "Officeholder",
            officeholder(w, id),
            officeholder(&after, id),
            "The actual recorded officeholder or institutional description after this decision; no future person is invented.",
        );
        add_change(
            &mut changes,
            "ruling_bloc",
            "Ruling bloc",
            blocs::ruling_bloc(w, id)
                .map_or("None", |b| b.label())
                .into(),
            blocs::ruling_bloc(&after, id)
                .map_or("None", |b| b.label())
                .into(),
            "The bloc that holds power after the decision.",
        );
        add_change(
            &mut changes,
            "discontent",
            "Public discontent",
            percent(blocs::discontent(w, id)),
            percent(blocs::discontent(&after, id)),
            "Re-read immediately from the same country conditions used by the simulation.",
        );
        let old_shares = blocs::bloc_shares(w, id);
        let new_shares = blocs::bloc_shares(&after, id);
        let old_backing = blocs::backing(w, id);
        let new_backing = blocs::backing(&after, id);
        for b in Bloc::ALL {
            add_change(
                &mut changes,
                format!("bloc_share:{}", b.key()),
                format!("{} support", b.label()),
                percent(old_shares[b as usize].1),
                percent(new_shares[b as usize].1),
                "The actual normalized domestic share after the command.",
            );
            add_change(
                &mut changes,
                format!("backing:{}", b.key()),
                format!("{} foreign backing", b.label()),
                percent(old_backing[b as usize].1),
                percent(new_backing[b as usize].1),
                "Foreign backing is separate from voters. Unexposed sponsors remain unnamed.",
            );
        }
    }
    if let Some(pol) = gov::polity_in(w, id) {
        for s in pol.pillars {
            add_change(&mut changes,format!("loyalty:{}",s.pillar.key()),format!("{} loyalty",s.name),percent(before_g.loyalty(s.pillar)),percent(after_g.loyalty(s.pillar)),"The immediate loyalty change; its underlying policy-dependent target is unchanged unless the decision changes those policies.");
        }
        for p in pol.parties {
            add_change(
                &mut changes,
                format!("party_support:{}", p.id),
                format!("{} public support", p.name),
                percent(before_g.support_of(p.id)),
                percent(after_g.support_of(p.id)),
                "The actual normalized support after the decision; these voters are distinct from parliamentary seats.",
            );
            add_change(
                &mut changes,
                format!("party_seats:{}", p.id),
                format!("{} seats", p.name),
                percent(before_g.seat_share(p.id)),
                percent(after_g.seat_share(p.id)),
                "The party's share of the chamber after the actual seat calculation.",
            );
            let was_banned = before_g.banned.iter().any(|s| s == p.id);
            let now_banned = after_g.banned.iter().any(|s| s == p.id);
            add_change(
                &mut changes,
                format!("party_status:{}", p.id),
                format!("{} status", p.name),
                if was_banned { "Banned" } else { "Legal" }.into(),
                if now_banned { "Banned" } else { "Legal" }.into(),
                "A ban changes legal participation; it does not erase voters.",
            );
        }
    }
    for other in w.nations.iter().filter(|n| n.alive && n.id != id) {
        add_change(
            &mut changes,
            format!("relation:{:?}", other.id),
            format!("Relations with {}", other.id.name()),
            points(w.relation(id, other.id)),
            points(after.relation(id, other.id)),
            "The actual diplomatic change, including the existing relationship limits.",
        );
    }
    let kind = command["kind"].as_str().unwrap_or("");
    let mut warnings = vec![];
    if kind == "secure_pillar" {
        warnings.push("This is a one-time public payment. Institutional loyalty continues to respond to budgets and country conditions afterward.".to_string());
    }
    if kind == "call_election" {
        warnings.push("These are the results of calling an election now, using today's support. They are not a forecast of a future scheduled election.".to_string());
    }
    if kind == "expel_from_government" && price_pc + 1e-9 < action["price"].as_f64().unwrap_or(0.0)
    {
        warnings.push(format!("Expulsion remains available below its nominal {} PC price and spends only the {} PC you currently hold.",points(action["price"].as_f64().unwrap_or(0.0)),points(price_pc)));
    }
    if kind == "stratagem"
        && command["id"] == "liberalisation"
        && !gov::is_electoral(w, id)
        && gov::is_electoral(&after, id)
    {
        warnings.push("This opens the electoral system now. The next government tick schedules the first election; this review does not advance that tick.".to_string());
    }
    json!({"valid":true,"reason":null,"title":action["label"],"kind":action["kind"],"command":command,"price_pc":price_pc,"money_cost_bn":money_cost,"description":action["detail"],"effects":action["effects"],"changes":changes,"warnings":warnings})
}

#[cfg(test)]
mod tests {
    use super::*;
    use spheres_sim::world::GameRules;

    fn world(id: NationId, lens: bool) -> WorldState {
        let mut w = spheres_sim::init::world_1990(GameRules {
            seed: 7,
            ideology_blocs: lens,
            ..GameRules::default()
        });
        w.player = Some(id);
        gov::ensure_all(&mut w);
        w
    }

    #[test]
    fn government_briefing_is_pure_and_preserves_existing_action_contracts() {
        for id in [NationId::Poland, NationId::Iraq] {
            let w = world(id, true);
            let before = spheres_sim::save(&w);
            let mut board = super::super::government_json(&w, id);
            let commands: Vec<Value> = board["actions"]
                .as_array()
                .unwrap()
                .iter()
                .map(|a| a["command"].clone())
                .collect();
            enrich(&w, id, &mut board);
            assert_eq!(
                commands,
                board["actions"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|a| a["command"].clone())
                    .collect::<Vec<_>>()
            );
            assert!(board["briefing"]["stats"].as_array().unwrap().len() >= 5);
            assert!(board["briefing"]["attention"]
                .as_array()
                .unwrap()
                .iter()
                .any(|a| a["id"] == "ideology_watch"));
            let mut keys = std::collections::BTreeSet::new();
            for a in board["actions"].as_array().unwrap() {
                assert!(keys.insert(a["key"].as_str().unwrap()));
            }
            assert_eq!(spheres_sim::save(&w), before);
        }
    }

    #[test]
    fn government_preview_requires_own_exact_current_action() {
        let w = world(NationId::Iraq, true);
        let before = spheres_sim::save(&w);
        for payload in [
            json!(null),
            json!([]),
            json!({}),
            json!({"kind":"secure_pillar","pillar":"navy"}),
            json!({"kind":"secure_pillar","pillar":"army","nation":"Poland"}),
            json!({"kind":"tax","value":20}),
            json!({"kind":"secure_pillar","pillar":"army","extra":true}),
        ] {
            assert_eq!(preview(&w, NationId::Iraq, &payload)["valid"], false);
        }
        assert_eq!(
            preview(&w, NationId::Poland, &json!({"kind":"call_election"}))["valid"],
            false
        );
        assert_eq!(spheres_sim::save(&w), before);
    }

    #[test]
    fn government_institution_review_shows_real_cash_debt_and_pc_without_spending() {
        for cash in [None, Some(0.0), Some(1000.0)] {
            let mut w = world(NationId::Iraq, true);
            if let Some(cash) = cash {
                let n = w.nation_mut(NationId::Iraq);
                n.treasury_bn = Some(cash);
                n.debt_bn = Some(n.debt_gdp * n.gdp);
            }
            let saved = spheres_sim::save(&w);
            let quoted = preview(
                &w,
                NationId::Iraq,
                &json!({"kind":"secure_pillar","pillar":"army"}),
            );
            assert_eq!(quoted["valid"], true, "{quoted}");
            assert!((quoted["price_pc"].as_f64().unwrap() - 14.0).abs() < 1e-9);
            assert!(
                (quoted["money_cost_bn"].as_f64().unwrap() - w.nation(NationId::Iraq).gdp * 0.008)
                    .abs()
                    < 1e-9
            );
            assert!(quoted["changes"]
                .as_array()
                .unwrap()
                .iter()
                .any(|c| c["key"] == "loyalty:army"));
            assert_eq!(spheres_sim::save(&w), saved);
        }
    }

    #[test]
    fn government_reviews_retain_live_refusals_and_off_world_state() {
        let mut w = world(NationId::Poland, false);
        let saved = spheres_sim::save(&w);
        let early = preview(&w, NationId::Poland, &json!({"kind":"call_election"}));
        assert_eq!(early["valid"], false);
        assert!(early["reason"].as_str().unwrap().contains("six months"));
        let ban = preview(
            &w,
            NationId::Poland,
            &json!({"kind":"ban_party","party":"pl_sld"}),
        );
        assert_eq!(ban["valid"], false);
        assert_eq!(ban["reason"], gov::NO_MOVEMENTS);
        let board = super::super::government_json(&w, NationId::Poland);
        assert!(board["briefing"]["stats"]
            .as_array()
            .unwrap()
            .iter()
            .all(|s| s["key"] != "discontent"));
        assert_eq!(spheres_sim::save(&w), saved);
        w.nation_mut(NationId::Poland).alive = false;
        assert_eq!(
            preview(&w, NationId::Poland, &json!({"kind":"call_election"}))["valid"],
            false
        );
    }

    #[test]
    fn government_election_review_is_an_exact_immediate_result() {
        let mut w = world(NationId::Poland, true);
        w.governments
            .states
            .iter_mut()
            .find(|g| g.nation == NationId::Poland)
            .unwrap()
            .months_in_office = 12;
        let payload = json!({"kind":"call_election"});
        let saved = spheres_sim::save(&w);
        let review = preview(&w, NationId::Poland, &payload);
        assert_eq!(review["valid"], true, "{review}");
        let mut actual = w.clone();
        spheres_sim::apply_command(
            &mut actual,
            &spheres_sim::Command::CallElection {
                nation: NationId::Poland,
            },
        )
        .unwrap();
        let date = review["changes"]
            .as_array()
            .unwrap()
            .iter()
            .find(|c| c["key"] == "election")
            .unwrap();
        assert_eq!(date["after"], election(&actual, NationId::Poland));
        assert!(review["warnings"][0]
            .as_str()
            .unwrap()
            .contains("not a forecast"));
        assert_eq!(spheres_sim::save(&w), saved);
    }

    #[test]
    fn government_election_review_includes_the_actual_successor_office() {
        let mut w = world(NationId::UK, true);
        let g = w
            .governments
            .states
            .iter_mut()
            .find(|g| g.nation == NationId::UK)
            .unwrap();
        g.months_in_office = 12;
        let opposition_share = 0.3 / (g.support.len() - 1) as f64;
        for (party, share) in &mut g.support {
            *share = if party == "uk_lab" {
                0.7
            } else {
                opposition_share
            };
        }
        let saved = spheres_sim::save(&w);
        let review = preview(&w, NationId::UK, &json!({"kind":"call_election"}));
        assert_eq!(review["valid"], true, "{review}");
        let mut actual = w.clone();
        spheres_sim::apply_command(
            &mut actual,
            &spheres_sim::Command::CallElection {
                nation: NationId::UK,
            },
        )
        .unwrap();
        let row = review["changes"]
            .as_array()
            .unwrap()
            .iter()
            .find(|c| c["key"] == "officeholder")
            .unwrap();
        assert_eq!(row["before"], officeholder(&w, NationId::UK));
        assert_eq!(row["after"], officeholder(&actual, NationId::UK));
        assert!(row["after"].as_str().unwrap().contains("Labour"));
        assert_eq!(spheres_sim::save(&w), saved);
    }

    #[test]
    fn government_expulsion_review_quotes_actual_clamped_pc_and_majority_loss() {
        let mut w = world(NationId::Italy, true);
        w.nation_mut(NationId::Italy).political_capital = 3.0;
        let party = gov::state(&w, NationId::Italy).unwrap().coalition[1].clone();
        let payload = json!({"kind":"expel_from_government","party":party});
        let saved = spheres_sim::save(&w);
        let review = preview(&w, NationId::Italy, &payload);
        assert_eq!(review["valid"], true, "{review}");
        assert_eq!(review["price_pc"], 3.0);
        assert_eq!(review["money_cost_bn"], 0.0);
        assert!(review["warnings"]
            .as_array()
            .unwrap()
            .iter()
            .any(|s| s.as_str().unwrap().contains("nominal")));
        assert_eq!(spheres_sim::save(&w), saved);
    }
}
