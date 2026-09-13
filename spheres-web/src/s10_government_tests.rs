//! S10 government and identity integration using the shipped catalogue.
//! Deliberate support/date changes below are boundary fixtures, not historical
//! election outcomes or an assertion of worldwide content completeness.
use super::{government_json, government_view, parse_command, person_portraits};
use serde_json::{json, Value};
use spheres_sim::{blocs, government as gov, party_leadership as parties};
use spheres_sim::world::{GameRules, NationId, WorldState};

const CASES: [(NationId, &str, bool); 4] = [
    (NationId::UK, "margaret_thatcher", true),
    (NationId::France, "francois_mitterrand", true),
    (NationId::China, "jiang_zemin", false),
    (NationId::SaudiArabia, "fahd_bin_abdulaziz_al_saud", false),
];

fn world(id: NationId) -> WorldState {
    let mut w = spheres_sim::init::world_1990(GameRules {
        seed: 13, daily_simulation: true, ideology_blocs: true,
        historical_party_leadership: true, ..GameRules::default()
    });
    w.player = Some(id);
    gov::ensure_all(&mut w);
    parties::ensure_all(&mut w);
    parties::validate_state(&w).unwrap();
    w
}

fn party<'a>(view: &'a Value, id: &str) -> &'a Value {
    view["parties"].as_array().unwrap().iter()
        .find(|p| p["party_id"] == id).unwrap()
}

fn current_id(w: &WorldState, id: NationId) -> Option<String> {
    parties::executive_person(w, id).map(|p| p.id.clone())
}

fn office_text(w: &WorldState, id: NationId) -> String {
    let leader = blocs::leader(w, id).unwrap();
    format!("{}: {}", leader.office, leader.name.or(leader.described).unwrap())
}

#[test]
fn s10_four_government_roles_serve_actual_offices_and_all_authored_parties_purely() {
    for (id, person, electoral) in CASES {
        let w = world(id);
        let saved = spheres_sim::save(&w);
        let board = government_json(&w, id);
        assert_eq!(board["mine"], true);
        assert_eq!(board["electoral"], electoral);
        assert_eq!(board["party_leadership"]["executive_person"]["id"], person);
        assert_eq!(board["leader"], serde_json::to_value(blocs::leader(&w, id)).unwrap());
        let polity = gov::polity_in(&w, id).unwrap();
        let rows = board["party_leadership"]["parties"].as_array().unwrap();
        assert_eq!(rows.len(), polity.parties.len());
        for p in polity.parties { assert!(rows.iter().any(|row| row["party_id"] == p.id)); }
        let actions = board["actions"].as_array().unwrap();
        assert_eq!(actions.iter().any(|a| a["kind"] == "call_election"), electoral);
        assert_eq!(actions.iter().any(|a| a["kind"] == "secure_pillar"), !electoral);
        assert_eq!(board["briefing"]["date_label"], w.date_str());
        if id == NationId::France {
            let ps = party(&board["party_leadership"], "fr_ps");
            assert_eq!(ps["campaign"][0]["person"]["id"], "pierre_mauroy");
            assert_eq!(ps["campaign"][0]["executive_eligibility"]["role"], "party_only");
            assert_ne!(ps["campaign"][0]["person"]["id"], board["party_leadership"]["executive_person"]["id"]);
        }
        if id == NationId::SaudiArabia {
            assert!(rows.is_empty(), "The shipped monarchy has institutions, not invented parties");
            assert!(board["leader"]["party"].is_null());
            assert_eq!(board["leader"]["pillar"], "Party");
        }
        assert_eq!(spheres_sim::save(&w), saved, "{id:?} board changed the campaign");
    }
}

#[test]
fn s10_reviewed_elections_and_institution_payments_match_native_results_and_reload() {
    for (id, _, electoral) in CASES {
        let mut w = world(id);
        // Isolate a legal action and a definite electoral change of government.
        // These are explicit test setup conditions, not granted campaign progress.
        w.nation_mut(id).political_capital = 80.0;
        let command = if electoral {
            let winner = if id == NationId::UK { "uk_lab" } else { "fr_rpr" };
            let g = w.governments.states.iter_mut().find(|g| g.nation == id).unwrap();
            g.months_in_office = 12;
            let rest = 0.1 / (g.support.len() - 1) as f64;
            for (party, share) in &mut g.support { *share = if party == winner { 0.9 } else { rest }; }
            json!({"kind":"call_election"})
        } else {
            json!({"kind":"secure_pillar","pillar":"army"})
        };
        let saved = spheres_sim::save(&w);
        let quote = government_view::preview(&w, id, &command);
        assert_eq!(quote["valid"], true, "{id:?}: {quote}");
        let native = parse_command(&w, &command, id).unwrap();
        let mut actual = w.clone();
        spheres_sim::apply_command(&mut actual, &native).unwrap();
        assert_eq!(quote["price_pc"].as_f64().unwrap().to_bits(),
            (w.nation(id).political_capital - actual.nation(id).political_capital).max(0.0).to_bits());
        assert_eq!(actual.date_str(), w.date_str(), "A reviewed command does not advance the clock");
        assert!(actual.headlines.len() > w.headlines.len(), "The actual command must report its result");
        let changes = quote["changes"].as_array().unwrap();
        if electoral {
            let next = gov::state(&actual, id).unwrap().next_election;
            assert_eq!(changes.iter().find(|x| x["key"] == "election").unwrap()["after"], format!("{:04}-{:02}", next.0, next.1));
            assert_eq!(changes.iter().find(|x| x["key"] == "officeholder").unwrap()["after"], office_text(&actual, id));
            assert_eq!(quote["money_cost_bn"], 0.0);
            if id == NationId::UK { assert_eq!(current_id(&actual, id).as_deref(), Some("neil_kinnock")); }
            else {
                assert!(current_id(&actual, id).is_none(), "A party president has no automatic national executive grant");
                assert!(blocs::leader(&actual, id).unwrap().name.is_none());
            }
        } else {
            assert!(changes.iter().any(|x| x["key"] == "loyalty:army"));
            assert!(quote["money_cost_bn"].as_f64().unwrap() > 0.0);
            assert_eq!(current_id(&actual, id), current_id(&w, id));
        }
        assert_eq!(spheres_sim::save(&w), saved, "Preview spent, voted, or changed a leader");
        let mut loaded = spheres_sim::load(&spheres_sim::save(&actual)).unwrap();
        assert_eq!(government_json(&loaded, id), government_json(&actual, id));
        // The following actual government update must replay identically too.
        gov::tick(&mut actual); gov::tick(&mut loaded);
        assert_eq!(spheres_sim::save(&loaded), spheres_sim::save(&actual));
    }
}

#[test]
fn s10_historical_browsing_and_future_eligibility_never_replace_saved_incumbents() {
    for (id, original, _) in CASES {
        let mut w = world(id);
        let saved = spheres_sim::save(&w);
        for date in ["1990-01-01", "1990-11-27", "2026-09-07"] {
            let reference = person_portraits::reference_view(&w, id, date).unwrap();
            assert_eq!(reference["date"], date);
            assert!(reference["executive_person"].is_null());
            for p in reference["parties"].as_array().unwrap() {
                assert_eq!(p["status"], "reference_only");
                for key in ["campaign", "future_candidates", "future_preview"] {
                    assert!(p[key].as_array().unwrap().is_empty());
                }
            }
            if id == NationId::UK && date == "1990-11-27" {
                assert!(party(&reference, "uk_con")["historical"].as_array().unwrap().iter()
                    .any(|h| h["person"]["id"] == "john_major"));
            }
        }
        assert!(person_portraits::reference_view(&w, id, "2026-09-08").is_err());
        assert_eq!(spheres_sim::save(&w), saved);
        let identities = serde_json::to_value(&w.party_leadership).unwrap();
        let offices = serde_json::to_value(&w.leadership).unwrap();
        // Date-only boundaries deliberately do not simulate intervening elections,
        // deaths, or history. The purpose is to isolate candidate-read eligibility.
        for (year, month, day) in [(2026,9,7), (2026,9,8), (2035,12,31)] {
            (w.year,w.month,w.day) = (year,month,day);
            parties::ensure_all(&mut w);
            let before_read = spheres_sim::save(&w);
            let board = government_json(&w, id);
            assert_eq!(board["party_leadership"]["executive_person"]["id"], original);
            assert_eq!(serde_json::to_value(&w.party_leadership).unwrap(), identities);
            assert_eq!(serde_json::to_value(&w.leadership).unwrap(), offices);
            if id == NationId::UK {
                let candidates = party(&board["party_leadership"], "uk_con")["future_candidates"].as_array().unwrap();
                assert_eq!(candidates.is_empty(), (year,month,day) == (2026,9,7));
                assert!(candidates.iter().all(|c| c["person"]["fiction"]["origin"] == "fictional_successor"));
            }
            let loaded = spheres_sim::load(&before_read).unwrap();
            assert_eq!(government_json(&loaded, id), board);
            assert_eq!(spheres_sim::save(&w), before_read);
        }
    }
}

#[test]
fn s10_actual_future_succession_respects_executive_review_and_recorded_monarchy_heir() {
    for (id, original, _) in CASES {
        let mut w = world(id);
        (w.year,w.month,w.day) = (2030,1,2);
        let mut replay = spheres_sim::load(&spheres_sim::save(&w)).unwrap();
        let rng = serde_json::to_value(&w.rng).unwrap();
        let heir = blocs::leader(&w, id).unwrap().heir.map(|h| h.name);
        // Exercise the native succession event directly; no death is predicted.
        gov::seat_office(&mut w, id, &gov::Succession::Death);
        gov::seat_office(&mut replay, id, &gov::Succession::Death);
        assert_eq!(spheres_sim::save(&w), spheres_sim::save(&replay));
        assert_eq!(serde_json::to_value(&w.rng).unwrap(), rng);
        assert_ne!(current_id(&w, id).as_deref(), Some(original));
        assert_eq!(blocs::leader(&w, id).unwrap().since.as_deref(), Some("2030-01-02"));
        let board = government_json(&w, id);
        if id == NationId::UK {
            let successor = current_id(&w, id).expect("Reviewed UK parliamentary successor");
            assert!(parties::fictional_person(&successor).is_some());
            assert_eq!(board["party_leadership"]["executive_person"]["fiction"]["origin"], "fictional_successor");
        } else {
            assert!(current_id(&w, id).is_none(), "Unreviewed party candidates cannot become named executives");
            assert!(board["leader"]["name"].is_null());
            if id == NationId::SaudiArabia {
                assert_eq!(board["leader"]["described"], heir.unwrap());
                assert!(board["leader"]["heir"].is_null());
                gov::seat_office(&mut w, id, &gov::Succession::Death);
                assert_eq!(blocs::leader(&w, id).unwrap().described.as_deref(), Some("the ruling house"));
            } else {
                let party_id = if id == NationId::France { "fr_ps" } else { "cn_cpc" };
                let candidates = party(&board["party_leadership"], party_id)["future_candidates"].as_array().unwrap();
                assert!(!candidates.is_empty());
                assert!(candidates.iter().all(|c| c["executive_eligibility"]["authorized"] == false));
            }
        }
        parties::validate_state(&w).unwrap();
        let loaded = spheres_sim::load(&spheres_sim::save(&w)).unwrap();
        assert_eq!(government_json(&loaded, id), government_json(&w, id));
    }
}

#[test]
fn s10_foreign_and_newly_unaffordable_reviews_refuse_without_touching_either_country() {
    let mut w = world(NationId::SaudiArabia);
    let command = json!({"kind":"secure_pillar","pillar":"army"});
    let before = spheres_sim::save(&w);
    assert_eq!(government_view::preview(&w, NationId::SaudiArabia, &command)["valid"], true);
    let foreign = government_json(&w, NationId::China);
    assert_eq!(foreign["mine"], false);
    assert_eq!(government_view::preview(&w, NationId::China, &command)["valid"], false);
    assert_eq!(spheres_sim::save(&w), before);
    // Standing changed after the valid review: both the renewed review and the
    // actual dispatcher must now refuse the same command at the current price.
    w.nation_mut(NationId::SaudiArabia).political_capital = 0.0;
    let before = spheres_sim::save(&w);
    let quote = government_view::preview(&w, NationId::SaudiArabia, &command);
    assert_eq!(quote["valid"], false);
    let native = parse_command(&w, &command, NationId::SaudiArabia).unwrap();
    let refusal = spheres_sim::apply_command(&mut w, &native).unwrap_err();
    assert_eq!(quote["reason"], refusal);
    assert_eq!(spheres_sim::save(&w), before);
}
