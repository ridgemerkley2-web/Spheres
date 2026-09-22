//! Authored S12–S15 journey. Hardware and conflict starting conditions are
//! disclosed; exported steps use ordinary reviewed commands and whole days.
use super::*;
use serde_json::{json, Value};
use spheres_sim::{
    airbases as ab, airmissions as am, arsenal, aviation as av, clock, commitment,
    equipment as eq,
};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};
const ME: NationId = NationId::France;

fn exclusive(path: &Path, bytes: &[u8]) {
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .unwrap();
    f.write_all(bytes).unwrap();
    f.sync_all().unwrap();
}
fn confirmation(g: &Game, raw: &Value) -> Value {
    if raw["kind"] == "force_allocation" {
        return raw.clone();
    }
    let before = save(&g.world);
    let q = equipment_view::preview(&g.world, ME, &g.session_id, &json!({"command":raw})).unwrap();
    assert_eq!(q["valid"], true, "{raw}: {q}");
    assert_eq!(save(&g.world), before, "review is read-only");
    q["actions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|a| a["enabled"] != false && a["command"]["kind"] == raw["kind"])
        .unwrap()["command"]
        .clone()
}
fn setup_command(g: &mut Game, raw: Value) {
    let confirmed = confirmation(g, &raw);
    let c = parse_command(&g.world, &confirmed, ME).unwrap();
    apply_command(&mut g.world, &c).unwrap();
}
fn setup_day(w: &mut WorldState) {
    equipment_view::company_refit_view_tests::day(w);
}
fn near_district(w: &WorldState, id: NationId, lon: f64, lat: f64) -> String {
    let point = ab::BaseLocation {
        district: String::new(),
        name: String::new(),
        lon,
        lat,
    };
    w.districts
        .iter()
        .filter(|(_, owner)| **owner == id)
        .filter_map(|(d, _)| ab::district_location(d).map(|p| (d, ab::distance_km(&point, p))))
        .min_by(|a, b| a.1.total_cmp(&b.1))
        .unwrap()
        .0
        .clone()
}
fn authored_aircraft(g: &mut Game, key: &str, extended: bool) {
    let mut spec = eq::default_spec("air_light_attack");
    if extended {
        spec.components
            .insert("air_fuel".into(), "air_fuel_extended".into());
    }
    let preview = eq::design_preview(&g.world, ME, &spec);
    assert!(preview.valid, "{:?}", preview.blockers);
    let day = clock::absolute_day(&g.world);
    let n = g.world.nation_mut(ME);
    n.equipment.as_mut().unwrap().revisions.insert(
        key.into(),
        eq::DesignRevision {
            id: key.into(),
            name: if extended {
                "S15 Extended attack"
            } else {
                "S15 Light attack"
            }
            .into(),
            specification_key: eq::specification_key(&spec),
            spec,
            profile: preview.profile.unwrap(),
            created_day: day,
            certified_day: Some(day),
        },
    );
    arsenal::deliver_design(n, key, 6, 0.0).unwrap();
}
struct Journey {
    destination: PathBuf,
    commands: serde_json::Map<String, Value>,
    stages: serde_json::Map<String, Value>,
    steps: Vec<Value>,
    confirmations: Vec<Value>,
    seq: u64,
    days: usize,
}
impl Journey {
    fn command(&mut self, g: &mut Game, id: &str, raw: Value) {
        let confirmed = confirmation(g, &raw);
        let mut expected = g.world.clone();
        let c = parse_command(&expected, &confirmed, ME).unwrap();
        apply_command(&mut expected, &c).unwrap();
        resources::warm(&mut expected);
        self.seq += 1;
        let payload = json!({"session_id":g.session_id,"client_id":"s15-native-fixture","request_seq":self.seq,"commands":[confirmed]});
        let response = transport::immediate_request(g, &payload).unwrap();
        assert_eq!(response["errors"], json!([]), "{id}: {response}");
        assert_eq!(
            save(&g.world),
            save(&expected),
            "{id}: protected command matches independent dispatch"
        );
        let once = save(&g.world);
        let retry = transport::immediate_request(g, &payload).unwrap();
        assert_eq!(retry["command_replayed"], true);
        assert_eq!(
            save(&g.world),
            once,
            "lost-response retry cannot duplicate this action"
        );
        self.commands.insert(id.into(), raw);
        self.confirmations
            .push(json!({"id":id,"command":confirmed}));
        self.steps.push(json!({"command":id}));
    }
    fn days(&mut self, g: &mut Game, count: usize) {
        let mut resumed = storage::decode(&storage::encode(g).unwrap()).unwrap();
        let start = clock::absolute_day(&g.world);
        for _ in 0..count {
            g.advance_days(1, vec![]);
            resumed.advance_days(1, vec![]);
        }
        assert_eq!(clock::absolute_day(&g.world), start + count as i32);
        assert_eq!(save(&g.world), save(&resumed.world));
        assert_eq!(g.history, resumed.history);
        assert_eq!(g.log, resumed.log);
        self.days += count;
        self.steps.push(json!({"days":count}));
    }
    fn checkpoint(&mut self, g: &Game, id: &str) {
        let encoded = storage::encode(g).unwrap();
        let decoded = storage::decode(&encoded).unwrap();
        assert_eq!(save(&g.world), save(&decoded.world));
        assert_eq!(g.history, decoded.history);
        assert_eq!(g.log, decoded.log);
        let file = format!("expected-{id}.json");
        exclusive(&self.destination.join(&file), encoded.as_bytes());
        self.stages.insert(id.into(), json!(file));
        self.steps.push(json!({"checkpoint":id}));
        println!("S15_STAGE={id} DATE={}", g.world.date_str());
    }
    fn complete_base(&mut self, g: &mut Game, id: &str) {
        let mut count = 0;
        while ab::base(&g.world, id).unwrap().project.is_some() {
            assert!(count < 80, "base project must finish within funded bound");
            self.days(g, 1);
            count += 1;
        }
    }
}
fn base_order(order: ab::AirbaseCommand) -> Value {
    json!({"kind":"air_base","order":order})
}
fn squadron_order(order: av::SquadronCommand) -> Value {
    json!({"kind":"air_squadron","order":order})
}
fn mission_order(order: am::MissionCommand) -> Value {
    json!({"kind":"air_mission","order":order})
}

#[test]
#[ignore = "Exports only to a NEW absolute SPHERES_S15_FIXTURE_DIR"]
fn s15_export_disposable_flight_operations_fixture() {
    let destination = PathBuf::from(
        std::env::var_os("SPHERES_S15_FIXTURE_DIR").expect("Set SPHERES_S15_FIXTURE_DIR"),
    );
    assert!(destination.is_absolute() && !destination.exists());
    let (mut g, company, _, _) = equipment_view::company_refit_view_tests::ready();
    let setup_start = clock::absolute_day(&g.world);
    let district = g
        .world
        .companies
        .firms
        .iter()
        .find(|f| f.id == company)
        .unwrap()
        .district
        .clone();
    g.world
        .production
        .provinces
        .iter_mut()
        .find(|p| p.district == district)
        .unwrap()
        .arms_plants = 3;
    authored_aircraft(&mut g, "s15-light", false);
    authored_aircraft(&mut g, "s15-extended", true);
    setup_command(
        &mut g,
        json!({"kind":"company_capitalize","company":company,"amount_mn":100.0}),
    );
    setup_day(&mut g.world);
    let spec = eq::default_spec("air_tactical_strike");
    setup_command(
        &mut g,
        json!({"kind":"company_develop","company":company,"name":"S15 Supplier attack","platform":spec.platform,"components":spec.components,"daily_budget_mn":10.0,"stock_target":2}),
    );
    let supplier_product = g
        .world
        .companies
        .firms
        .iter()
        .find(|f| f.id == company)
        .unwrap()
        .products
        .last()
        .unwrap()
        .id;
    let supplier_revision = g
        .world
        .companies
        .firms
        .iter()
        .find(|f| f.id == company)
        .unwrap()
        .products
        .last()
        .unwrap()
        .revision_id
        .clone();
    for _ in 0..1500 {
        if g.world
            .companies
            .firms
            .iter()
            .find(|f| f.id == company)
            .unwrap()
            .products
            .iter()
            .find(|p| p.id == supplier_product)
            .unwrap()
            .stock
            >= 2
        {
            break;
        }
        setup_day(&mut g.world);
    }
    assert_eq!(
        g.world
            .companies
            .firms
            .iter()
            .find(|f| f.id == company)
            .unwrap()
            .products
            .iter()
            .find(|p| p.id == supplier_product)
            .unwrap()
            .stock,
        2
    );
    setup_command(
        &mut g,
        json!({"kind":"company_inventory","company":company,"product":supplier_product,"stock_target":0}),
    );
    setup_command(
        &mut g,
        json!({"kind":"company_ammo_supply","company":company,"family":"air_bomb_unguided","stock_target":200}),
    );
    for _ in 0..200 {
        if g.world
            .companies
            .firms
            .iter()
            .find(|f| f.id == company)
            .unwrap()
            .ammunition_products
            .iter()
            .any(|p| p.family == "air_bomb_unguided" && p.stock >= 200)
        {
            break;
        }
        setup_day(&mut g.world);
    }
    let ammunition_product = g
        .world
        .companies
        .firms
        .iter()
        .find(|f| f.id == company)
        .unwrap()
        .ammunition_products
        .iter()
        .find(|p| p.family == "air_bomb_unguided" && p.stock >= 200)
        .unwrap()
        .id;
    setup_command(
        &mut g,
        json!({"kind":"company_ammo_inventory","company":company,"product":ammunition_product,"stock_target":0}),
    );
    spheres_sim::connected_economy::enable(&mut g.world).unwrap();
    spheres_sim::company_network::enable(&mut g.world).unwrap();
    spheres_sim::supplier_catalogue::enable(&mut g.world).unwrap();
    spheres_sim::party_leadership::enable_campaign(&mut g.world).unwrap();
    g.world.conflicts.clear();
    g.world.statecraft.pacts.clear();
    g.world.sanctions.clear();
    let conflict =
        commitment::open_conflict(&mut g.world, ME, NationId::Italy, TheatreId::WesternEurope)
            .unwrap();
    {
        let c = g.world.conflict_mut(conflict).unwrap();
        c.invasion_declared = true;
        for p in &mut c.posture {
            p.rung = 8;
            p.objective = Objective::Hold;
        }
    }
    spheres_sim::operational_warfare::enable(&mut g.world).unwrap();
    let mut ground =
        spheres_sim::campaign::OperationOrder::automatic(g.world.conflict(conflict).unwrap(), ME);
    ground.reserve_bp = 0;
    ground.approach = spheres_sim::campaign::Approach::Hold;
    spheres_sim::campaign::set_order(&mut g.world, &ground).unwrap();
    let home = near_district(&g.world, ME, 6.8, 45.5);
    let abroad = near_district(&g.world, NationId::UK, 0.0, 51.0);
    g.world.access.push(spheres_sim::theatre::Access {
        host: NationId::UK,
        seeker: ME,
        theatre: TheatreId::WesternEurope,
        since_year: g.world.year,
        since_month: g.world.month,
    });
    programs::set_construction_budget(&mut g.world, ME, 0.01).unwrap();
    programs::begin_day(&mut g.world);
    resources::warm(&mut g.world);
    g.history.clear();
    g.snapshot();
    let mut g = storage::decode(&storage::encode(&g).unwrap()).unwrap();
    let opening_day = clock::absolute_day(&g.world);
    fs::create_dir(&destination).unwrap();
    exclusive(
        &destination.join("before.json"),
        storage::encode(&g).unwrap().as_bytes(),
    );
    let mut j = Journey {
        destination,
        commands: Default::default(),
        stages: Default::default(),
        steps: vec![],
        confirmations: vec![],
        seq: 0,
        days: 0,
    };
    j.command(&mut g,"purchase_aircraft",json!({"kind":"company_purchase","company":company,"product":supplier_product,"quantity":2}));
    j.command(&mut g,"fund_routine_support",json!({"kind":"equipment_air_support","daily_budget_mn":100.0,"target_days":30,"automatic":true}));
    j.command(
        &mut g,
        "form_first",
        squadron_order(av::SquadronCommand::Create {
            name: "S15 Alpine wing".into(),
            revision: "s15-light".into(),
            quantity: 6,
        }),
    );
    let first = g.world.nation(ME).aviation.as_ref().unwrap().squadrons[0].id;
    j.checkpoint(&g, "purchased_and_assigned");
    j.command(
        &mut g,
        "resize_first",
        squadron_order(av::SquadronCommand::Resize {
            squadron: first,
            quantity: 4,
        }),
    );
    j.command(
        &mut g,
        "change_revision",
        squadron_order(av::SquadronCommand::ChangeRevision {
            squadron: first,
            revision: "s15-extended".into(),
        }),
    );
    j.command(
        &mut g,
        "foundation_home",
        base_order(ab::AirbaseCommand::Establish {
            district: home.clone(),
            name: "S15 Alpine base".into(),
            daily_budget_mn: 2.0,
        }),
    );
    j.command(
        &mut g,
        "foundation_host",
        base_order(ab::AirbaseCommand::Establish {
            district: abroad.clone(),
            name: "S15 Consenting host base".into(),
            daily_budget_mn: 2.0,
        }),
    );
    j.checkpoint(&g, "funded_foundations");
    j.complete_base(&mut g, &home);
    j.complete_base(&mut g, &abroad);
    assert_eq!(ab::base(&g.world, &home).unwrap().capacity(), 12);
    assert_eq!(ab::base(&g.world, &abroad).unwrap().capacity(), 12);
    j.checkpoint(&g, "bases_completed");
    j.command(
        &mut g,
        "form_purchased",
        squadron_order(av::SquadronCommand::Create {
            name: "S15 Purchased wing".into(),
            revision: supplier_revision.clone(),
            quantity: 2,
        }),
    );
    let second = g
        .world
        .nation(ME)
        .aviation
        .as_ref()
        .unwrap()
        .squadrons
        .last()
        .unwrap()
        .id;
    j.command(
        &mut g,
        "stage_first",
        base_order(ab::AirbaseCommand::Rebase {
            squadron: first,
            base: home.clone(),
        }),
    );
    j.command(
        &mut g,
        "stage_purchased",
        base_order(ab::AirbaseCommand::Rebase {
            squadron: second,
            base: home.clone(),
        }),
    );
    j.checkpoint(&g, "initial_transit");
    j.days(&mut g, 2);
    j.command(
        &mut g,
        "move_to_host",
        base_order(ab::AirbaseCommand::Rebase {
            squadron: first,
            base: abroad.clone(),
        }),
    );
    j.checkpoint(&g, "foreign_transit");
    j.days(&mut g, 2);
    j.command(
        &mut g,
        "return_home",
        base_order(ab::AirbaseCommand::Rebase {
            squadron: first,
            base: home.clone(),
        }),
    );
    j.days(&mut g, 2);
    j.checkpoint(&g, "returned_ready");
    j.command(
        &mut g,
        "improve_support",
        base_order(ab::AirbaseCommand::Upgrade {
            base: home.clone(),
            track: ab::UpgradeTrack::Support,
            daily_budget_mn: 2.0,
        }),
    );
    j.complete_base(&mut g, &home);
    j.command(
        &mut g,
        "improve_protection",
        base_order(ab::AirbaseCommand::Upgrade {
            base: home.clone(),
            track: ab::UpgradeTrack::Protection,
            daily_budget_mn: 2.0,
        }),
    );
    j.complete_base(&mut g, &home);
    j.checkpoint(&g, "completed_improvements");
    let target = g
        .world
        .districts
        .keys()
        .find(|d| {
            am::quote(
                &g.world,
                ME,
                first,
                conflict,
                am::MissionKind::SupportArmy,
                d,
            )
            .valid
                && am::quote(
                    &g.world,
                    ME,
                    second,
                    conflict,
                    am::MissionKind::StrikeTarget,
                    d,
                )
                .valid
        })
        .cloned()
        .expect("Ordinary deployed army must have an eligible nearby target");
    j.command(
        &mut g,
        "queue_cancel_example",
        mission_order(am::MissionCommand::Queue {
            squadron: first,
            conflict,
            kind: am::MissionKind::StrikeTarget,
            target: target.clone(),
        }),
    );
    let cancelled = g
        .world
        .air_missions
        .as_ref()
        .unwrap()
        .orders
        .last()
        .unwrap()
        .id;
    j.command(
        &mut g,
        "cancel_queued",
        mission_order(am::MissionCommand::Cancel { mission: cancelled }),
    );
    j.checkpoint(&g, "cancelled_mission");
    j.command(
        &mut g,
        "support_army",
        mission_order(am::MissionCommand::Queue {
            squadron: first,
            conflict,
            kind: am::MissionKind::SupportArmy,
            target: target.clone(),
        }),
    );
    j.command(
        &mut g,
        "strike_target",
        mission_order(am::MissionCommand::Queue {
            squadron: second,
            conflict,
            kind: am::MissionKind::StrikeTarget,
            target: target.clone(),
        }),
    );
    j.checkpoint(&g, "missions_queued");
    j.days(&mut g, 2);
    j.checkpoint(&g, "missions_flown");
    let flown: Vec<_> = g
        .world
        .air_missions
        .as_ref()
        .unwrap()
        .orders
        .iter()
        .filter(|o| o.status == am::MissionStatus::Flown)
        .collect();
    assert_eq!(flown.len(), 2);
    assert!(flown.iter().all(|o| o
        .report
        .as_ref()
        .is_some_and(|r| r.stores_used > 0.0 && r.contacted && r.applied_power > 0.0)));
    j.days(&mut g, 2);
    j.checkpoint(&g, "funded_recovery");
    assert!(g
        .world
        .nation(ME)
        .aviation
        .as_ref()
        .unwrap()
        .squadrons
        .iter()
        .all(|s| s.service_days_left == 0));
    let manifest = json!({"version":1,"fixture":"s15-authored-flight-operations","compiled_revision":env!("SPHERES_REVISION"),"player":"France","before_file":"before.json","opening_day":opening_day,"days_advanced":j.days,
      "scope":"Authored France aircraft, supplier and military scenario. Ordinary reviewed browser controls and full daily continuation after import; not an unassisted or historically exhaustive campaign.",
      "authored_preconditions":["Native ready manufacturer fixture: 10 IFVs, $100bn treasury, no debt, 1000 political capital, one million units of tracked raw inventories and bounded opening departmental authority; existing IFV development was paid before export.","Three arms-plant slots and two certified light-attack revisions with six delivered aircraft each are authored starting conditions. Aircraft under the supplier revision are not granted.","A reviewed $100m additional company capital transfer funded two distinct tactical-strike supplier aircraft and 200 compatible unguided stores, actually company-developed/manufactured through ordinary setup contracts and daily work; inventory targets then set to zero. No initial national aircraft stores are authored.","France–Italy ground conflict at Hold posture, operational migration, zero French tactical reserve, UK host basing consent and national $10m/day construction limit are explicit pre-export conditions. All campaign adoption uses normal migration APIs.",ab::FOREIGN_SPONSORSHIP_NOTE],
      "preparation_days_after_manufacturer_ready":opening_day-setup_start,"commands":j.commands,"steps":j.steps,"expected_stages":j.stages,"native_confirmations":j.confirmations,
      "required_outcomes":{"company":company,"aircraft_product":supplier_product,"aircraft_revision":supplier_revision,"ammunition_product":ammunition_product,"home_base":home,"foreign_base":abroad,"first_squadron":first,"purchased_squadron":second,"conflict":conflict,"target":target,"cancelled_mission":cancelled}});
    exclusive(
        &j.destination.join("manifest.json"),
        serde_json::to_string_pretty(&manifest).unwrap().as_bytes(),
    );
    println!(
        "S15_FIXTURE_MANIFEST={}",
        j.destination.join("manifest.json").display()
    );
}
