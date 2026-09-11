//! Missing browser seams from S01 S1-S4. Core cadence, movement, consent and
//! property mechanics already have native tests; these keep their saved intent
//! through the browser archive. Original supplier versions use external evidence.
use super::*;
use serde_json::{Value, json};

fn same(a: &Game, b: &Game) {
    assert!(
        save(&a.world) == save(&b.world),
        "Canonical simulation differs"
    );
    assert!(a.history == b.history, "History differs");
    assert!(a.log == b.log, "Dispatch archive differs");
    assert_eq!(a.history_epoch, b.history_epoch);
}

fn owned(w: &WorldState) -> Value {
    json!({"rng":w.rng,"date":[w.year,w.month,w.day],"companies":w.companies,
        "production":w.production,"manufacturing":w.manufacturing,"resources":w.resources,
        "nations":w.nations.iter().map(|n|json!({"id":n.id,"alive":n.alive,
            "population":n.population,"gdp":n.gdp,"treasury":n.treasury_bn,"debt":n.debt_bn,
            "programs":n.program_budget,"arsenal":n.arsenal,"equipment":n.equipment})).collect::<Vec<_>>()})
}

fn party_case(mut game: Game, expected_equipment_version: u32) {
    let assets = owned(&game.world);
    let history = game.history.clone();
    let epoch = game.history_epoch;
    spheres_sim::party_leadership::enable_campaign(&mut game.world).unwrap();
    assert_eq!(
        owned(&game.world),
        assets,
        "Party adoption must not change paid property or RNG"
    );
    // Loaded games already contain this day's snapshot. Party enrollment is
    // not a new day, and appending it would fabricate a duplicate timestamp.
    assert_eq!(game.history, history);
    assert_eq!(game.history_epoch, epoch);
    let text = save(&game.world);
    let envelope: Value = serde_json::from_str(&text).unwrap();
    assert_eq!(envelope["format"], "spheres-party-leadership-save");
    assert_eq!(envelope["version"], 1);
    assert_eq!(envelope["equipment_version"], expected_equipment_version);
    assert!(
        save(&load(&text).unwrap()) == text,
        "Native standalone party envelope changed"
    );
    let mut direct = storage::decode(&text).unwrap();
    assert!(
        save(&direct.world) == text,
        "Browser standalone party envelope changed"
    );
    assert_eq!(owned(&direct.world), assets);
    assert_eq!(
        serde_json::to_value(&direct.world.party_leadership).unwrap(),
        serde_json::to_value(&game.world.party_leadership).unwrap()
    );
    let twice = storage::decode(&save(&direct.world)).unwrap();
    assert!(save(&twice.world) == text);
    let mut archived = storage::decode(&storage::encode(&game).unwrap()).unwrap();
    same(&game, &archived);
    game.advance_days(1, vec![]);
    direct.advance_days(1, vec![]);
    archived.advance_days(1, vec![]);
    same(&game, &archived);
    assert!(
        save(&game.world) == save(&direct.world),
        "Standalone continuation differs from archived continuation"
    );
}

#[test]
fn standalone_party_v1_with_no_equipment_and_native_equipment_v1_uses_both_loaders() {
    for version in 0..=1 {
        let mut game = Game::new(1990, Some(NationId::France));
        play_rules(&mut game);
        if version == 1 {
            // A legal, empty equipment research/design workspace. This is the
            // v1 envelope case, not a fabricated certification or vehicle.
            game.world.nation_mut(NationId::France).equipment = Some(Default::default());
        }
        party_case(game, version);
    }
}

#[test]
#[ignore = "Requires the unchanged original 28-file export manifest in SPHERES_S05_ACTIVE_FIXTURES"]
fn standalone_party_v1_preserves_real_supplier_books_for_equipment_versions_2_through_5() {
    let root = std::path::PathBuf::from(
        std::env::var_os("SPHERES_S05_ACTIVE_FIXTURES")
            .expect("Set the original exporter fixture directory"),
    );
    assert!(root.is_absolute());
    let manifest: Value =
        serde_json::from_slice(&std::fs::read(root.join("active-fixture-manifest.json")).unwrap())
            .unwrap();
    assert_eq!(manifest["format"], "spheres-active-fixture-matrix");
    assert_eq!(
        manifest["source_revision"],
        "5f7f355502f17bd6bd8f0383a2d14f0024fa7884"
    );
    let rows = manifest["fixtures"].as_array().unwrap();
    assert_eq!(rows.len(), 28);
    for (version, file) in [
        (2, "companies/04-transit.json"),
        (3, "companies/09-mixed-transit.json"),
        (4, "ammunition/03-ammunition-transit.json"),
        (5, "refits/ground-04-active-work.json"),
    ] {
        let row = rows.iter().find(|r| r["file"] == file).unwrap();
        let path = root.join(file);
        let bytes = std::fs::read(&path).unwrap();
        assert_eq!(bytes.len() as u64, row["bytes"].as_u64().unwrap());
        // The existing browser migration establishes its documented flags;
        // the test explicitly adds party identities only on this private copy.
        // Save chooses the legal nested version from the real company book.
        let game = storage::decode(std::str::from_utf8(&bytes).unwrap()).unwrap();
        assert!(!game.world.companies.is_empty());
        assert!(game.world.party_leadership.is_none());
        party_case(game, version);
        assert_eq!(
            std::fs::read(&path).unwrap(),
            bytes,
            "Original fixture changed"
        );
        println!(
            "S05_PARTY_ENVELOPE {}",
            json!({"file":file,"source_sha256":row["sha256"],"equipment_version":version,"party_version":1,"loaders":"native, browser standalone, browser archive","continuation_days":1})
        );
    }
}

#[test]
fn malformed_archives_and_mismatched_direct_party_books_refuse_clearly() {
    let mut game = Game::new(1990, Some(NationId::France));
    play_rules(&mut game);
    spheres_sim::party_leadership::enable_campaign(&mut game.world).unwrap();
    let standalone: Value = serde_json::from_str(&save(&game.world)).unwrap();
    let archive: Value = serde_json::from_str(&storage::encode(&game).unwrap()).unwrap();
    assert!(storage::decode(&standalone.to_string()).is_ok());
    assert!(storage::decode(&archive.to_string()).is_ok(), "Negative cases must begin with a valid archive");
    let mut cases: Vec<(&str, Value)> = Vec::new();
    for field in ["world", "history", "log", "saved_date"] {
        let mut bad = archive.clone();
        bad.as_object_mut().unwrap().remove(field);
        cases.push((field, bad));
    }
    let mut bad = archive.clone();
    bad["version"] = json!(999);
    cases.push(("future archive", bad));
    let mut bad = archive.clone();
    bad["history"][0]["month"] = json!(13);
    cases.push(("invalid history month", bad));
    let mut bad = archive.clone();
    bad["history"][0]["day"] = json!(32);
    cases.push(("invalid history day", bad));
    let mut bad = archive.clone();
    bad["history"][0]["t"] = json!(1.0);
    cases.push(("future history", bad));
    let mut bad = archive.clone();
    let first = bad["history"][0].clone();
    bad["history"].as_array_mut().unwrap().push(first);
    cases.push(("duplicate history date", bad));
    for (label, value) in cases {
        let error = storage::decode(&value.to_string())
            .err()
            .unwrap_or_else(|| panic!("Accepted {label}"));
        assert!(!error.is_empty(), "Refusal needs an explanation: {label}");
    }
    let text = archive.to_string();
    assert!(
        storage::decode(&text[..text.len() - 1]).is_err(),
        "Truncated closing envelope was accepted"
    );
    for case in 0..5 {
        let mut bad = standalone.clone();
        match case {
            0 => {
                bad.as_object_mut().unwrap().remove("equipment_version");
            }
            1 => {
                bad["equipment_version"] = json!(5);
            } // no corresponding supplier book
            2 => {
                bad["world"]["rules"]["historical_party_leadership"] = json!(false);
            }
            3 => {
                bad["world"].as_object_mut().unwrap().remove("rng");
            }
            _ => {
                bad["version"] = json!(999);
            }
        }
        assert!(
            load(&bad.to_string()).is_err(),
            "Native accepted inconsistent party case {case}"
        );
        assert!(
            storage::decode(&bad.to_string()).is_err(),
            "Direct import accepted inconsistent party case {case}"
        );
        let mut nested = archive.clone();
        nested["world"] = bad;
        assert!(
            storage::decode(&nested.to_string()).is_err(),
            "Archive accepted inconsistent party case {case}"
        );
    }
    assert!(
        storage::decode(&standalone["world"].to_string()).is_err(),
        "Unwrapped party property was silently downgraded"
    );
}

#[test]
fn browser_archive_retains_pending_midmonth_close_and_resumes_daily_once() {
    let mut old = Game::new(1990, Some(NationId::France));
    play_rules(&mut old);
    old.world.rules.daily_simulation = false;
    old.history.clear();
    old.snapshot();
    old.advance_days(14, vec![]);
    assert_eq!(old.world.date_str(), "15 Jan 1990");
    let mut expected_month = load(&save(&old.world)).unwrap();
    let mut uninterrupted = storage::decode(&storage::encode(&old).unwrap()).unwrap();
    assert_eq!(uninterrupted.history, old.history);
    assert_eq!(uninterrupted.world.daily.activate_after_month, Some(0));
    let opening = uninterrupted.world.nation(NationId::France).gdp;
    for _ in 0..10 {
        uninterrupted.advance_days(1, vec![]);
    }
    assert_eq!(uninterrupted.world.date_str(), "25 Jan 1990");
    assert_eq!(uninterrupted.world.nation(NationId::France).gdp, opening);
    let mut resumed = storage::decode(&storage::encode(&uninterrupted).unwrap()).unwrap();
    same(&uninterrupted, &resumed);
    assert!(!resumed.world.rules.daily_simulation);
    assert_eq!(resumed.world.daily.activate_after_month, Some(0));
    for _ in 0..7 {
        uninterrupted.advance_days(1, vec![]);
        resumed.advance_days(1, vec![]);
        same(&uninterrupted, &resumed);
    }
    tick_month(&mut expected_month, &[]);
    let mut paid_month = resumed.world.clone();
    paid_month.rules.daily_simulation = false;
    assert!(
        save(&paid_month) == save(&expected_month),
        "Archive migration did not post exactly the original owed month"
    );
    assert_eq!(resumed.world.date_str(), "1 Feb 1990");
    assert!(resumed.world.rules.daily_simulation);
    assert_eq!(resumed.world.daily.activate_after_month, None);
    let february_open = resumed.world.nation(NationId::France).gdp;
    uninterrupted.advance_days(1, vec![]);
    resumed.advance_days(1, vec![]);
    same(&uninterrupted, &resumed);
    assert_ne!(resumed.world.nation(NationId::France).gdp, february_open);
    assert!(!spheres_sim::connected_economy::has_state(&resumed.world));
    assert!(!spheres_sim::company_network::has_state(&resumed.world));
    assert!(!spheres_sim::operational_warfare::has_state(&resumed.world));
}

#[test]
fn pending_peace_and_real_movement_survive_archive_then_consent_settles_once() {
    use spheres_sim::campaign_peace::{PeaceOrder, Terms};
    let mut game = Game::new(1990, Some(NationId::Iran));
    play_rules(&mut game);
    game.world.rules.ai_aggression = 0.0;
    game.world.rules.crisis_intensity = 0.0;
    // Explicit technical standing, not free troops, movement or supply records.
    for n in [NationId::Iraq, NationId::Iran] {
        game.world.nation_mut(n).political_capital = 100.0;
    }
    apply_command(
        &mut game.world,
        &Command::EnableOperationalWarfare {
            nation: NationId::Iran,
        },
    )
    .unwrap();
    let theatre = spheres_sim::war::theatre_between(&game.world, NationId::Iraq, NationId::Iran);
    apply_command(
        &mut game.world,
        &Command::OpenConflict {
            opener: NationId::Iraq,
            target: NationId::Iran,
            theatre,
        },
    )
    .unwrap();
    let conflict = game
        .world
        .conflict_between(NationId::Iraq, NationId::Iran)
        .unwrap()
        .id;
    for nation in [NationId::Iraq, NationId::Iran] {
        apply_command(
            &mut game.world,
            &Command::SetCommitment {
                conflict,
                nation,
                rung: 8,
            },
        )
        .unwrap();
    }
    game.history.clear();
    game.snapshot();
    game.advance_days(1, vec![]);
    assert!(
        game.world
            .campaign
            .transfers
            .iter()
            .any(|t| t.conflict == Some(conflict)),
        "New deployment must generate actual finite movement"
    );
    apply_command(
        &mut game.world,
        &Command::WarDiplomacy {
            nation: NationId::Iraq,
            order: PeaceOrder::Propose {
                conflict,
                terms: Terms::Ceasefire,
            },
        },
    )
    .unwrap();
    let offer = game
        .world
        .campaign_peace
        .offers
        .iter()
        .find(|o| o.conflict == conflict)
        .unwrap()
        .id;
    let movement = serde_json::to_value(&game.world.campaign.transfers).unwrap();
    let consent = serde_json::to_value(&game.world.campaign_peace).unwrap();
    let mut resumed = storage::decode(&storage::encode(&game).unwrap()).unwrap();
    same(&game, &resumed);
    assert_eq!(
        serde_json::to_value(&resumed.world.campaign.transfers).unwrap(),
        movement
    );
    assert_eq!(
        serde_json::to_value(&resumed.world.campaign_peace).unwrap(),
        consent
    );
    game.advance_days(1, vec![]);
    resumed.advance_days(1, vec![]);
    same(&game, &resumed);
    assert!(
        game.world
            .campaign_peace
            .offers
            .iter()
            .any(|o| o.id == offer && !o.approved.contains(&NationId::Iran)),
        "AI may not consent for the saved human government"
    );
    let respond = Command::WarDiplomacy {
        nation: NationId::Iran,
        order: PeaceOrder::Respond {
            offer,
            accept: true,
        },
    };
    apply_command(&mut game.world, &respond).unwrap();
    apply_command(&mut resumed.world, &respond).unwrap();
    same(&game, &resumed);
    assert!(game.world.conflict(conflict).is_none());
    assert_eq!(
        game.world
            .campaign_peace
            .history
            .iter()
            .filter(|r| r.offer.id == offer && r.outcome == "accepted")
            .count(),
        1
    );
    let once = save(&game.world);
    assert!(apply_command(&mut game.world, &respond).is_err());
    assert!(
        save(&game.world) == once,
        "Repeated peace response changed settled ownership"
    );
    for _ in 0..8 {
        game.advance_days(1, vec![]);
        resumed.advance_days(1, vec![]);
        same(&game, &resumed);
        for nation in [NationId::Iraq, NationId::Iran] {
            assert!(
                (spheres_sim::campaign::accounted_force(&game.world, nation)
                    - game.world.nation(nation).mil_strength)
                    .abs()
                    < 1e-7,
                "Returning movement duplicated or lost national force"
            );
        }
    }
    assert_eq!(
        game.world
            .campaign_peace
            .history
            .iter()
            .filter(|r| r.offer.id == offer)
            .count(),
        1
    );
    same(
        &game,
        &storage::decode(&storage::encode(&game).unwrap()).unwrap(),
    );
}

#[test]
fn achieved_goals_survive_more_than_twenty_four_abandoned_goals_and_reload() {
    use spheres_sim::campaign_aims::{self, Aim};
    let nation = NationId::France;
    let mut game = Game::new(1990, Some(nation));
    play_rules(&mut game);
    let mut achieved = Vec::new();
    let mut abandoned = Vec::new();
    for index in 0..36 {
        let completing = matches!(index, 0 | 12 | 25);
        let aim = if completing {
            Aim::Science
        } else {
            Aim::Stability
        };
        apply_command(&mut game.world, &Command::ChooseCampaignAim { nation, aim }).unwrap();
        if completing {
            // Explicit achievement prerequisite: the fixture acquires eight
            // previously unknown technology IDs. This tests history retention,
            // not research funding; the ordinary aim observer declares success.
            let acquired: Vec<_> = (0..spheres_sim::tech::registry().len() as u16)
                .filter(|id| !game.world.nation(nation).tech.known.contains(id))
                .take(campaign_aims::SCIENCE_DISCOVERIES)
                .collect();
            assert_eq!(acquired.len(), campaign_aims::SCIENCE_DISCOVERIES);
            let known = &mut game.world.nation_mut(nation).tech.known;
            known.extend(acquired);
            known.sort();
            campaign_aims::tick(&mut game.world);
            assert!(
                game.world
                    .campaign_aims
                    .active
                    .as_ref()
                    .unwrap()
                    .completed_day
                    .is_some()
            );
        }
        apply_command(&mut game.world, &Command::ContinueSandbox { nation }).unwrap();
        let record = game.world.campaign_aims.history.last().unwrap().clone();
        if completing {
            assert_eq!(record.outcome, "achieved");
            achieved.push(record);
        } else {
            assert_eq!(record.outcome, "set aside");
            abandoned.push(record);
        }
        spheres_sim::clock::advance_date(&mut game.world);
    }
    assert_eq!(
        abandoned.len(),
        33,
        "Exercise abandonment beyond the rolling limit"
    );
    let check = |world: &WorldState, abandoned: &[campaign_aims::Record]| {
        let history = &world.campaign_aims.history;
        let complete: Vec<_> = history
            .iter()
            .filter(|r| r.goal.completed_day.is_some())
            .cloned()
            .collect();
        let set_aside: Vec<_> = history
            .iter()
            .filter(|r| r.goal.completed_day.is_none())
            .cloned()
            .collect();
        assert_eq!(
            complete, achieved,
            "Old and interleaved achievements must never be evicted"
        );
        assert_eq!(
            set_aside,
            abandoned[abandoned.len() - 24..],
            "Keep the newest 24 abandoned records in order"
        );
        assert_eq!(history.len(), achieved.len() + 24);
    };
    check(&game.world, &abandoned);
    let native = load(&save(&game.world)).unwrap();
    assert_eq!(native.campaign_aims, game.world.campaign_aims);
    game.snapshot();
    let mut resumed = storage::decode(&storage::encode(&game).unwrap()).unwrap();
    same(&game, &resumed);
    check(&resumed.world, &abandoned);
    // Trimming still behaves correctly after reload, not just on the first
    // process's in-memory history. No completed goal is re-awarded or lost.
    for world in [&mut game.world, &mut resumed.world] {
        apply_command(
            world,
            &Command::ChooseCampaignAim {
                nation,
                aim: Aim::Stability,
            },
        )
        .unwrap();
        apply_command(world, &Command::ContinueSandbox { nation }).unwrap();
    }
    abandoned.push(game.world.campaign_aims.history.last().unwrap().clone());
    check(&resumed.world, &abandoned);
    same(&game, &resumed);
}
