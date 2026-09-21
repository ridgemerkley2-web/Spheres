//! Bounded, disclosed S16 scenarios: paid fighter procurement, a quiet patrol,
//! explicit authored opposition, and depleted defense. This is not an AI run.
use super::*;
use serde_json::{json, Value};
use spheres_sim::{
    airbases as ab, airmissions as am, arsenal, aviation as av, clock, commitment, equipment as eq,
};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};
const ME: NationId = NationId::France;
const ENEMY: NationId = NationId::Italy;
const MISSILES: &str = "air_missile_short_range";
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
    let before = save(&g.world);
    let q = equipment_view::preview(&g.world, ME, &g.session_id, &json!({"command":raw})).unwrap();
    assert_eq!(q["valid"], true, "{raw}: {q}");
    assert_eq!(save(&g.world), before);
    q["actions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|a| a["enabled"] != false && a["command"]["kind"] == raw["kind"])
        .unwrap()["command"]
        .clone()
}
fn setup_command(g: &mut Game, raw: Value) {
    let c = parse_command(&g.world, &confirmation(g, &raw), ME).unwrap();
    apply_command(&mut g.world, &c).unwrap();
}
fn setup_day(w: &mut WorldState) {
    equipment_view::company_refit_view_tests::day(w);
}
fn mission(squadron: u32, conflict: u32, target: &str) -> Value {
    json!({"kind":"air_mission","order":am::MissionCommand::Queue{squadron,conflict,kind:am::MissionKind::DefendSkies,target:target.into()}})
}
fn saved(g: &Game) -> Game {
    storage::decode(&storage::encode(g).unwrap()).unwrap()
}
fn report(g: &Game, key: u32) -> am::MissionReport {
    g.world
        .air_missions
        .as_ref()
        .unwrap()
        .orders
        .iter()
        .find(|o| o.id == key)
        .unwrap()
        .report
        .clone()
        .unwrap()
}
fn last_order(g: &Game) -> u32 {
    g.world
        .air_missions
        .as_ref()
        .unwrap()
        .orders
        .last()
        .unwrap()
        .id
}
fn authored_base(w: &mut WorldState, id: NationId, district: &str) {
    w.airbases
        .get_or_insert_with(Default::default)
        .bases
        .push(ab::Airbase {
            id: district.into(),
            district: district.into(),
            name: format!("S16 {id:?} starting airfield"),
            sponsor: id,
            capacity_level: 1,
            support_level: 1,
            protection_level: 0,
            project: None,
            history: vec![],
        });
}
fn author_opponent(g: &mut Game, base: &str) -> u32 {
    let w = &mut g.world;
    let today = clock::absolute_day(w);
    let state = w
        .nation_mut(ENEMY)
        .equipment
        .get_or_insert_with(Default::default);
    state
        .learned
        .extend(eq::RESEARCH.iter().map(|r| r.id.into()));
    let spec = eq::default_spec("air_light_attack");
    let p = eq::design_preview(w, ENEMY, &spec);
    assert!(p.valid, "{:?}", p.blockers);
    w.nation_mut(ENEMY)
        .equipment
        .as_mut()
        .unwrap()
        .revisions
        .insert(
            "s16-opposing-attack".into(),
            eq::DesignRevision {
                id: "s16-opposing-attack".into(),
                name: "Authored Italian attack flight".into(),
                specification_key: eq::specification_key(&spec),
                spec,
                profile: p.profile.unwrap(),
                created_day: today,
                certified_day: Some(today),
            },
        );
    arsenal::deliver_design(w.nation_mut(ENEMY), "s16-opposing-attack", 4, 0.0).unwrap();
    w.player = Some(ENEMY);
    w.nation_mut(ENEMY).treasury_bn = Some(1000.0);
    programs::set_construction_budget(w, ENEMY, 0.0).unwrap();
    eq::set_maintenance_plan(w, ENEMY, 1.0).unwrap();
    av::apply(
        w,
        ENEMY,
        &av::SquadronCommand::Create {
            name: "Authored Italian opposition".into(),
            revision: "s16-opposing-attack".into(),
            quantity: 4,
        },
    )
    .unwrap();
    let sq = w
        .nation_mut(ENEMY)
        .aviation
        .as_mut()
        .unwrap()
        .squadrons
        .last_mut()
        .unwrap();
    sq.base = Some(base.into());
    let key = sq.id;
    w.player = Some(ME);
    // A balanced, completed authored batch makes the opponent's starting stock
    // load-valid. It is disclosed rather than represented as browser procurement.
    let district = w
        .districts
        .iter()
        .find(|(_, n)| **n == ENEMY)
        .unwrap()
        .0
        .clone();
    let def = eq::ammo_def("air_bomb_unguided").unwrap();
    let quantity = 200;
    let a = w
        .nation_mut(ENEMY)
        .equipment
        .as_mut()
        .unwrap()
        .ammunition
        .get_or_insert_with(Default::default);
    let id = a.next_id;
    a.next_id += 1;
    a.orders.push(eq::AmmoOrder {
        id,
        family: def.id.into(),
        district,
        quantity,
        completed_rounds: quantity,
        work_rounds: quantity as f64,
        cost_bn: def.fabrication_bn * quantity as f64,
        spent_bn: def.fabrication_bn * quantity as f64,
        daily_limit_bn: 1.0,
        recipe_per_round: def.recipe,
        resources_used: def.recipe.map(|v| v * quantity as f64),
        rounds_per_day: def.rounds_per_day,
        started_day: today - 1,
        completed_day: Some(today),
        last_day: None,
        last_spent_bn: 0.0,
        paused: false,
        status: eq::ProjectStatus::Complete,
        reason: "Disclosed completed opponent starting ammunition batch.".into(),
    });
    a.stocks.insert(def.id.into(), quantity as f64);
    key
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
        let c = confirmation(g, &raw);
        let mut expected = g.world.clone();
        apply_command(&mut expected, &parse_command(&g.world, &c, ME).unwrap()).unwrap();
        resources::warm(&mut expected);
        self.seq += 1;
        let payload = json!({"session_id":g.session_id,"client_id":"s16-native-fixture","request_seq":self.seq,"commands":[c]});
        let r = transport::immediate_request(g, &payload).unwrap();
        assert_eq!(r["errors"], json!([]), "{id}: {r}");
        assert_eq!(save(&g.world), save(&expected), "{id}");
        let once = save(&g.world);
        assert_eq!(
            transport::immediate_request(g, &payload).unwrap()["command_replayed"],
            true
        );
        assert_eq!(save(&g.world), once);
        self.commands.insert(id.into(), raw);
        self.confirmations.push(json!({"id":id,"command":c}));
        self.steps.push(json!({"command":id}));
    }
    fn days(&mut self, g: &mut Game, count: usize) {
        let mut resumed = saved(g);
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
        let round = saved(g);
        assert_eq!(save(&g.world), save(&round.world));
        assert_eq!(g.history, round.history);
        assert_eq!(g.log, round.log);
        let file = format!("expected-{id}.json");
        exclusive(
            &self.destination.join(&file),
            storage::encode(g).unwrap().as_bytes(),
        );
        self.stages.insert(id.into(), json!(file));
        self.steps.push(json!({"checkpoint":id}));
        println!("S16_STAGE={id} DATE={}", g.world.date_str());
    }
    fn imported(&mut self, g: &Game, id: &str) {
        let file = format!("{id}.json");
        exclusive(
            &self.destination.join(&file),
            storage::encode(g).unwrap().as_bytes(),
        );
        self.steps.push(json!({"load":id,"file":file}));
        self.checkpoint(g, id);
    }
}

#[test]
#[ignore = "Exports only to a NEW absolute SPHERES_S16_FIXTURE_DIR"]
fn s16_export_disposable_air_defense_fixture() {
    let destination = PathBuf::from(
        std::env::var_os("SPHERES_S16_FIXTURE_DIR").expect("Set SPHERES_S16_FIXTURE_DIR"),
    );
    assert!(destination.is_absolute() && !destination.exists());
    let (mut g, company, _, _) = equipment_view::company_refit_view_tests::ready();
    let setup_start = clock::absolute_day(&g.world);
    g.world
        .nation_mut(ME)
        .equipment
        .as_mut()
        .unwrap()
        .learned
        .extend(
            [
                "air_propulsion_integration",
                "air_mission_systems",
                "air_fighter_integration",
            ]
            .into_iter()
            .map(String::from),
        );
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
    setup_command(
        &mut g,
        json!({"kind":"company_capitalize","company":company,"amount_mn":200.0}),
    );
    setup_day(&mut g.world);
    let spec = eq::default_spec("air_fighter");
    setup_command(
        &mut g,
        json!({"kind":"company_develop","company":company,"name":"S16 Supplier fighter","platform":spec.platform,"components":spec.components,"daily_budget_mn":10.0,"stock_target":4}),
    );
    let product = g
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
    let revision = g
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
            .find(|p| p.id == product)
            .unwrap()
            .stock
            >= 4
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
            .find(|p| p.id == product)
            .unwrap()
            .stock,
        4
    );
    setup_command(
        &mut g,
        json!({"kind":"company_inventory","company":company,"product":product,"stock_target":0}),
    );
    println!(
        "S16_SUPPLIER_BEFORE_MISSILES cash_bn={:?}",
        g.world
            .companies
            .firms
            .iter()
            .find(|f| f.id == company)
            .unwrap()
            .cash_bn
    );
    setup_command(
        &mut g,
        json!({"kind":"company_capitalize","company":company,"amount_mn":100.0}),
    );
    setup_command(
        &mut g,
        json!({"kind":"company_ammo_supply","company":company,"family":MISSILES,"stock_target":200}),
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
            .any(|p| p.family == MISSILES && p.stock >= 200)
        {
            break;
        }
        setup_day(&mut g.world);
    }
    assert!(
        g.world
            .companies
            .firms
            .iter()
            .find(|f| f.id == company)
            .unwrap()
            .ammunition_products
            .iter()
            .any(|p| p.family == MISSILES && p.stock >= 200),
        "Supplier did not finish missiles: {:?}",
        g.world
            .companies
            .firms
            .iter()
            .find(|f| f.id == company)
            .unwrap()
            .ammunition_products
    );
    let ammunition_product = g
        .world
        .companies
        .firms
        .iter()
        .find(|f| f.id == company)
        .unwrap()
        .ammunition_products
        .iter()
        .find(|p| p.family == MISSILES && p.stock >= 200)
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
        commitment::open_conflict(&mut g.world, ME, ENEMY, TheatreId::WesternEurope).unwrap();
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
    let contested =
        spheres_sim::front::contested_set(&g.world, g.world.conflict(conflict).unwrap());
    let home = contested
        .k
        .keys()
        .filter(|d| g.world.districts.get(*d) == Some(&ME))
        .filter_map(|d| ab::district_location(d).map(|l| (d, l)))
        .min_by(|a, b| {
            ((a.1.lon - 6.8).abs() + (a.1.lat - 45.5).abs())
                .total_cmp(&((b.1.lon - 6.8).abs() + (b.1.lat - 45.5).abs()))
        })
        .unwrap()
        .0
        .clone();
    let origin = ab::district_location(&home).unwrap();
    let enemy_base = g
        .world
        .districts
        .iter()
        .filter(|(_, n)| **n == ENEMY)
        .filter_map(|(d, _)| ab::district_location(d).map(|l| (d, ab::distance_km(origin, l))))
        .min_by(|a, b| a.1.total_cmp(&b.1))
        .unwrap()
        .0
        .clone();
    authored_base(&mut g.world, ME, &home);
    authored_base(&mut g.world, ENEMY, &enemy_base);
    let opposing_squadron = author_opponent(&mut g, &enemy_base);
    programs::set_construction_budget(&mut g.world, ME, 0.01).unwrap();
    programs::begin_day(&mut g.world);
    resources::warm(&mut g.world);
    g.history.clear();
    g.snapshot();
    let mut g = saved(&g);
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
    j.command(
        &mut g,
        "purchase_fighters",
        json!({"kind":"company_purchase","company":company,"product":product,"quantity":4}),
    );
    j.command(&mut g,"fund_routine_support",json!({"kind":"equipment_air_support","daily_budget_mn":100.0,"target_days":30,"automatic":true}));
    for _ in 0..12 {
        if av::unassigned_units(g.world.nation(ME), &revision) >= 4 {
            break;
        }
        j.days(&mut g, 1);
    }
    assert!(av::unassigned_units(g.world.nation(ME), &revision) >= 4);
    j.command(&mut g,"form_fighter_squadron",json!({"kind":"air_squadron","order":av::SquadronCommand::Create{name:"S16 Alpine defense".into(),revision:revision.clone(),quantity:4}}));
    let squadron = g
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
        "stage_fighters",
        json!({"kind":"air_base","order":ab::AirbaseCommand::Rebase{squadron,base:home.clone()}}),
    );
    j.checkpoint(&g, "delivery_transit");
    for _ in 0..8 {
        if am::quote(
            &g.world,
            ME,
            squadron,
            conflict,
            am::MissionKind::DefendSkies,
            &home,
        )
        .valid
        {
            break;
        }
        j.days(&mut g, 1);
    }
    j.command(
        &mut g,
        "queue_cancel_example",
        mission(squadron, conflict, &home),
    );
    let cancelled = last_order(&g);
    j.command(
        &mut g,
        "cancel_patrol",
        json!({"kind":"air_mission","order":am::MissionCommand::Cancel{mission:cancelled}}),
    );
    j.command(&mut g, "quiet_patrol", mission(squadron, conflict, &home));
    let quiet = last_order(&g);
    j.days(&mut g, 2);
    j.checkpoint(&g, "quiet_flown");
    let quiet_report = report(&g, quiet);
    assert!(
        quiet_report.stores_used > 0.0
            && !quiet_report.contacted
            && quiet_report.applied_power == 0.0
    );
    assert_eq!(quiet_report.defense.as_ref().unwrap().opposing_missions, 0);
    j.command(&mut g,"hold_store_replenishment",json!({"kind":"equipment_air_support","daily_budget_mn":100.0,"target_days":30,"automatic":false}));
    j.days(&mut g, 2);
    assert!(
        am::quote(
            &g.world,
            ME,
            squadron,
            conflict,
            am::MissionKind::DefendSkies,
            &home
        )
        .valid
    );
    g.world.player = Some(ENEMY);
    let q = am::quote(
        &g.world,
        ENEMY,
        opposing_squadron,
        conflict,
        am::MissionKind::StrikeTarget,
        &home,
    );
    assert!(q.valid, "Opponent: {:?}", q.reason);
    am::apply(
        &mut g.world,
        ENEMY,
        &am::MissionCommand::Queue {
            squadron: opposing_squadron,
            conflict,
            kind: am::MissionKind::StrikeTarget,
            target: home.clone(),
        },
    )
    .unwrap();
    g.world.player = Some(ME);
    let hostile = last_order(&g);
    resources::warm(&mut g.world);
    g = saved(&g);
    let common = saved(&g);
    j.imported(&g, "intercept_before");
    j.command(
        &mut g,
        "defend_against_strike",
        mission(squadron, conflict, &home),
    );
    let defense = last_order(&g);
    j.checkpoint(&g, "defense_queued");
    j.days(&mut g, 2);
    j.checkpoint(&g, "defense_flown");
    let defended_report = report(&g, defense);
    let defended_strike = report(&g, hostile);
    assert_eq!(
        defended_report.defense.as_ref().unwrap().opposing_missions,
        1
    );
    assert!(defended_report.defense.as_ref().unwrap().prevented_power > 0.0);
    assert!(
        defended_strike
            .defense
            .as_ref()
            .unwrap()
            .air_combat_expected_loss
            > 0.0
    );
    let mut g = common;
    let a = g
        .world
        .nation_mut(ME)
        .equipment
        .as_mut()
        .unwrap()
        .ammunition
        .as_mut()
        .unwrap();
    let rounds = a.stocks.insert(MISSILES.into(), 0.0).unwrap_or(0.0);
    *a.consumed.entry(MISSILES.into()).or_default() += rounds;
    resources::warm(&mut g.world);
    g = saved(&g);
    j.imported(&g, "depleted_before");
    let refusal = am::quote(
        &g.world,
        ME,
        squadron,
        conflict,
        am::MissionKind::DefendSkies,
        &home,
    );
    assert!(!refusal.valid);
    j.steps.push(json!({"refusal":"depleted_defense","command":mission(squadron,conflict,&home),"reason":refusal.reason}));
    j.days(&mut g, 2);
    j.checkpoint(&g, "depleted_flown");
    let baseline = report(&g, hostile);
    assert!(
        baseline.applied_power > defended_strike.applied_power,
        "Unopposed {:?}, defended {:?}",
        baseline,
        defended_strike
    );
    assert!(baseline.defense.is_none());
    let manifest = json!({"version":1,"fixture":"s16-authored-air-defense","compiled_revision":env!("SPHERES_REVISION"),"player":"France","before_file":"before.json","opening_day":opening_day,"days_advanced":j.days,"scope":"Three bounded authored France–Italy scenarios. Ordinary player procurement, review, patrol and daily continuation; explicit opposing orders are authored imports. This is not AI opposition or campaign certification beyond G3.","authored_preconditions":["Native ready manufacturer fixture: 10 IFVs, $100bn treasury, no debt, 1000 political capital, abundant tracked opening raw inventories and bounded departmental authority. Three arms-plant slots are authored.","France starts with the three researched fighter integrations. Reviewed company capital transfers of $200m for fighters and $100m for missile stock fund actual fighter development, four manufactured fighters and 200 compatible missiles through normal setup contracts and daily work. National fighters and missiles are purchased after import.","Completed France and Italy airfields with Capacity 1 and Support 1, France–Italy Hold conflict and ordinary operational enrollment are explicit starting conditions.","Italian opposition has four authored certified light-attack aircraft, a load-valid balanced 200-bomb completed starting batch and $1000bn treasury with normal $1bn/day maintenance authority. It has no autonomous explicit mission orders.","The second imported scenario adds one Italian Strike target through native commands to the recovered first scenario. The third clones that same starting world and transfers remaining French missile stocks to the consumed ledger. Store replenishment was disabled normally before both imports. The same hostile strike then provides the unopposed baseline."],"preparation_days_after_manufacturer_ready":opening_day-setup_start,"commands":j.commands,"steps":j.steps,"expected_stages":j.stages,"native_confirmations":j.confirmations,"required_outcomes":{"company":company,"aircraft_product":product,"aircraft_revision":revision,"ammunition_product":ammunition_product,"home_base":home,"enemy_base":enemy_base,"squadron":squadron,"opposing_squadron":opposing_squadron,"conflict":conflict,"target":home,"cancelled_mission":cancelled,"quiet_mission":quiet,"hostile_mission":hostile,"defense_mission":defense},"comparison":{"quiet":quiet_report,"defended":defended_report,"hostile_with_defense":defended_strike,"unopposed_hostile_baseline":baseline}});
    exclusive(
        &j.destination.join("manifest.json"),
        serde_json::to_string_pretty(&manifest).unwrap().as_bytes(),
    );
    println!(
        "S16_FIXTURE_MANIFEST={}",
        j.destination.join("manifest.json").display()
    );
}
