//! Exclusive authored S11 journey export. Starting property is disclosed;
//! every action after before.json uses normal reviewed commands or game days.
use super::*;
use serde_json::{json, Value};
use spheres_sim::{arsenal, clock, commitment, companies, equipment as eq};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

const ME: NationId = NationId::France;

fn exclusive(path: &Path, bytes: &[u8]) {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .unwrap();
    file.write_all(bytes).unwrap();
    file.sync_all().unwrap();
}

fn confirmation(g: &Game, raw: &Value) -> Value {
    if raw["kind"] == "force_allocation" {
        return raw.clone();
    }
    let saved = save(&g.world);
    let quote =
        equipment_view::preview(&g.world, ME, &g.session_id, &json!({"command":raw})).unwrap();
    assert_eq!(quote["valid"], true, "{raw}: {quote}");
    assert_eq!(
        save(&g.world),
        saved,
        "native preview changes no campaign field"
    );
    quote["actions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|a| a["enabled"] != false && a["command"]["kind"] == raw["kind"])
        .unwrap()["command"]
        .clone()
}

fn setup_command(g: &mut Game, raw: Value) {
    let command = confirmation(g, &raw);
    let parsed = parse_command(&g.world, &command, ME).unwrap();
    apply_command(&mut g.world, &parsed).unwrap();
}

fn setup_day(w: &mut WorldState) {
    if w.nation(ME).program_budget.as_ref().unwrap().fiscal_year != w.year {
        let departments = w.nation(ME).program_budget.as_ref().unwrap().departments;
        let allocations = w.nation(ME).budget_for(w.year).allocations;
        apply_command(
            w,
            &Command::SetProgramBudget {
                nation: ME,
                fiscal_year: w.year,
                allocations,
                departments,
            },
        )
        .unwrap();
    }
    programs::begin_day(w);
    eq::tick_day(w);
    companies::tick_day(w);
    eq::settle_support(w);
    let budget = w.nation_mut(ME).program_budget.as_mut().unwrap();
    budget.revenue_today_bn = 0.0;
    budget.interest_today_bn = 0.0;
    budget.fiscal_staged = true;
    programs::finish_day(w);
    companies::settle_receivables(w);
    clock::advance_date(w);
}

fn authored_revision(g: &mut Game, platform: &str, key: &str, quantity: u32) {
    let spec = eq::default_spec(platform);
    let preview = eq::design_preview(&g.world, ME, &spec);
    assert!(preview.valid, "{platform}: {:?}", preview.blockers);
    let day = clock::absolute_day(&g.world);
    let n = g.world.nation_mut(ME);
    n.equipment.as_mut().unwrap().revisions.insert(
        key.into(),
        eq::DesignRevision {
            id: key.into(),
            name: format!("S11 {}", eq::platform_role(platform)),
            specification_key: eq::specification_key(&spec),
            spec,
            profile: preview.profile.unwrap(),
            created_day: day,
            certified_day: Some(day),
        },
    );
    arsenal::deliver_design(n, key, quantity, 0.0).unwrap();
}

struct Journey {
    destination: PathBuf,
    commands: serde_json::Map<String, Value>,
    stages: serde_json::Map<String, Value>,
    steps: Vec<Value>,
    reviews: Vec<Value>,
    request_seq: u64,
    days: usize,
    saw_loss: bool,
}

impl Journey {
    fn archive(&mut self, g: &Game, id: &str) {
        let archive = storage::encode(g).unwrap();
        let loaded = storage::decode(&archive).unwrap();
        assert_eq!(
            save(&loaded.world),
            save(&g.world),
            "stage {id} must preserve all native world fields on load"
        );
        assert_eq!(loaded.history, g.history);
        assert_eq!(loaded.log, g.log);
        let file = format!("expected-{id}.json");
        exclusive(&self.destination.join(&file), archive.as_bytes());
        self.stages.insert(id.into(), json!(file));
        println!("S11_STAGE={id} DATE={}", g.world.date_str());
        if let Some(r) = &g
            .world
            .nation(ME)
            .equipment
            .as_ref()
            .unwrap()
            .ground_operations_receipt
        {
            self.saw_loss |= r.conflicts.len() == 2 && r.revisions.iter().any(|r| r.lost > 0);
        }
    }

    fn command(&mut self, g: &mut Game, id: &str, raw: Value) {
        let confirmed = confirmation(g, &raw);
        let mut expected = g.world.clone();
        let parsed = parse_command(&expected, &confirmed, ME).unwrap();
        apply_command(&mut expected, &parsed).unwrap();
        resources::warm(&mut expected);
        self.request_seq += 1;
        let payload = json!({"session_id":g.session_id,"client_id":"s11-native-fixture",
            "request_seq":self.request_seq,"commands":[confirmed]});
        let response = transport::immediate_request(g, &payload).unwrap();
        assert_eq!(response["errors"], json!([]), "{id}: {response}");
        assert_eq!(
            save(&g.world),
            save(&expected),
            "{id}: protected transport must match independent native execution"
        );
        self.reviews
            .push(json!({"id":id,"confirmed_command":confirmed}));
        self.commands.insert(id.into(), raw);
        self.steps.push(json!({"id":id,"command":id}));
        self.archive(g, id);
    }

    fn day(&mut self, g: &mut Game, id: &str) {
        let mut continued = storage::decode(&storage::encode(g).unwrap()).unwrap();
        let day = clock::absolute_day(&g.world);
        g.advance_days(1, vec![]);
        continued.advance_days(1, vec![]);
        assert_eq!(
            clock::absolute_day(&g.world),
            day + 1,
            "{id}: exactly one ordinary day"
        );
        assert_eq!(
            save(&continued.world),
            save(&g.world),
            "{id}: reload and ordinary continuation agree"
        );
        assert_eq!(continued.history, g.history);
        assert_eq!(continued.log, g.log);
        self.days += 1;
        self.steps.push(json!({"id":id,"days":1}));
        self.archive(g, id);
    }
}

#[test]
#[ignore = "Exports only to a NEW absolute SPHERES_S11_FIXTURE_DIR"]
fn s11_export_disposable_ground_operations_fixture() {
    let destination = PathBuf::from(
        std::env::var_os("SPHERES_S11_FIXTURE_DIR").expect("Set SPHERES_S11_FIXTURE_DIR"),
    );
    assert!(destination.is_absolute());
    assert!(
        !destination.exists(),
        "Never overwrite a fixture or campaign directory"
    );
    let (mut g, company, product, target) = equipment_view::company_refit_view_tests::ready();
    let setup_start = clock::absolute_day(&g.world);
    let district = g
        .world
        .companies
        .firms
        .iter()
        .find(|c| c.id == company)
        .unwrap()
        .district
        .clone();
    g.world
        .production
        .provinces
        .iter_mut()
        .find(|p| p.district == district)
        .unwrap()
        .arms_plants = 2;
    for (platform, key, quantity) in [
        ("tank_standard", "s11-tank", 5000),
        ("ground_apc", "s11-apc", 1000),
        ("ground_recon", "s11-recon", 1000),
        ("ground_artillery", "s11-artillery", 1000),
        ("ground_air_defense", "s11-air-defense", 1000),
    ] {
        authored_revision(&mut g, platform, key, quantity);
    }
    eq::set_maintenance_plan(&mut g.world, ME, 0.1).unwrap();
    setup_command(
        &mut g,
        json!({"kind":"company_inventory","company":company,"product":product,"stock_target":1}),
    );
    for _ in 0..400 {
        if g.world
            .companies
            .firms
            .iter()
            .find(|c| c.id == company)
            .unwrap()
            .products
            .iter()
            .any(|p| p.id == product && p.stock >= 1)
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
            .find(|c| c.id == company)
            .unwrap()
            .products
            .iter()
            .find(|p| p.id == product)
            .unwrap()
            .stock,
        1
    );
    setup_command(
        &mut g,
        json!({"kind":"company_inventory","company":company,"product":product,"stock_target":0}),
    );
    let families = g
        .world
        .nation(ME)
        .equipment
        .as_ref()
        .unwrap()
        .revisions
        .values()
        .filter_map(|r| eq::ammunition_family(&r.spec))
        .map(str::to_owned)
        .collect::<std::collections::BTreeSet<_>>();
    for family in &families {
        setup_command(
            &mut g,
            json!({"kind":"equipment_ammo_order","family":family,"district":district,"quantity":20_000,"daily_budget_mn":10.0}),
        );
        // One public slot beside the leased company slot: finish each paid
        // family before commissioning the next, never overbook the factory.
        let maximum_days =
            (20_000.0 / eq::ammo_def(family).unwrap().rounds_per_day).ceil() as usize + 30;
        for _ in 0..maximum_days {
            if g.world
                .nation(ME)
                .equipment
                .as_ref()
                .unwrap()
                .ammunition
                .as_ref()
                .is_some_and(|a| a.stocks.get(family).copied().unwrap_or(0.0) >= 20_000.0)
            {
                break;
            }
            setup_day(&mut g.world);
        }
        assert_eq!(
            g.world
                .nation(ME)
                .equipment
                .as_ref()
                .unwrap()
                .ammunition
                .as_ref()
                .unwrap()
                .stocks
                .get(family)
                .copied(),
            Some(20_000.0),
            "paid public fabrication must finish: {family}"
        );
    }
    let stores = g
        .world
        .nation(ME)
        .equipment
        .as_ref()
        .unwrap()
        .ammunition
        .as_ref()
        .unwrap();
    for family in &families {
        assert_eq!(
            stores.stocks.get(family).copied(),
            Some(20_000.0),
            "paid opening ammunition: {family}"
        );
    }
    setup_command(
        &mut g,
        json!({"kind":"company_ammo_supply","company":company,"family":"autocannon_25","stock_target":1000}),
    );
    for _ in 0..100 {
        if g.world
            .companies
            .firms
            .iter()
            .find(|c| c.id == company)
            .unwrap()
            .ammunition_products
            .iter()
            .any(|p| p.family == "autocannon_25" && p.stock >= 1000)
        {
            break;
        }
        setup_day(&mut g.world);
    }
    let ammo_product = g
        .world
        .companies
        .firms
        .iter()
        .find(|c| c.id == company)
        .unwrap()
        .ammunition_products
        .iter()
        .find(|p| p.family == "autocannon_25" && p.stock >= 1000)
        .unwrap()
        .id;
    setup_command(
        &mut g,
        json!({"kind":"company_ammo_inventory","company":company,"product":ammo_product,"stock_target":0}),
    );
    // Mature adoption uses ordinary enrollment and preserves all earned stock.
    // These disclosed utility facilities and components are initial authored
    // property, supplied before any protected journey command or daily oracle.
    spheres_sim::connected_economy::enable(&mut g.world).unwrap();
    g.world
        .production
        .provinces
        .iter_mut()
        .find(|p| p.district == district)
        .unwrap()
        .power_grid = 4;
    let generation = spheres_sim::industry::EXTENDED
        .iter()
        .position(|k| *k == ProjectKind::Generation)
        .unwrap();
    g.world
        .production
        .industry
        .sites
        .entry(district.clone())
        .or_default()[generation] = 4;
    g.world
        .production
        .operations
        .advanced_components
        .insert(ME, 100.0);
    spheres_sim::company_network::enable(&mut g.world).unwrap();
    spheres_sim::supplier_catalogue::enable(&mut g.world).unwrap();
    spheres_sim::party_leadership::enable_campaign(&mut g.world).unwrap();
    g.world.conflicts.clear();
    g.world.statecraft.pacts.clear();
    g.world.sanctions.clear();
    let mut conflicts = Vec::new();
    for enemy in [NationId::Italy, NationId::Spain] {
        let cid =
            commitment::open_conflict(&mut g.world, ME, enemy, TheatreId::WesternEurope).unwrap();
        let c = g.world.conflict_mut(cid).unwrap();
        c.invasion_declared = true;
        for p in &mut c.posture {
            p.rung = 8;
            p.objective = if p.nation == ME {
                Objective::Seize
            } else {
                Objective::Hold
            };
        }
        conflicts.push(cid);
    }
    spheres_sim::operational_warfare::enable(&mut g.world).unwrap();
    // Opening reserves, aim and missions are authored native operation orders;
    // their force still comes from the shared national pool during migration.
    for cid in &conflicts {
        let mut order =
            spheres_sim::campaign::OperationOrder::automatic(g.world.conflict(*cid).unwrap(), ME);
        order.reserve_bp = 0;
        order.approach = spheres_sim::campaign::Approach::Advance;
        spheres_sim::campaign::set_order(&mut g.world, &order).unwrap();
    }
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
    let mut journey = Journey {
        destination,
        commands: Default::default(),
        stages: Default::default(),
        steps: vec![],
        reviews: vec![],
        request_seq: 0,
        days: 0,
        saw_loss: false,
    };
    journey.command(
        &mut g,
        "allocation_zero",
        json!({"kind":"force_allocation","conflict":conflicts[0],"share_bp":0}),
    );
    journey.command(
        &mut g,
        "allocation_auto",
        json!({"kind":"force_allocation","conflict":conflicts[0],"share_bp":null}),
    );
    journey.command(
        &mut g,
        "maintenance",
        json!({"kind":"equipment_maintenance","daily_budget_mn":10.0}),
    );
    journey.command(
        &mut g,
        "ammunition_activate",
        json!({"kind":"equipment_ammo_activate"}),
    );
    journey.command(&mut g,"refit",json!({"kind":"company_refit","company":company,"source":"ammo-demo","product":product,"quantity":1}));
    let refit = g
        .world
        .companies
        .firms
        .iter()
        .find(|c| c.id == company)
        .unwrap()
        .refits
        .last()
        .unwrap()
        .id;
    journey.command(
        &mut g,
        "purchase",
        json!({"kind":"company_purchase","company":company,"product":product,"quantity":1}),
    );
    let vehicle_delivery = g.world.companies.deliveries.last().unwrap().id;
    journey.command(&mut g,"resupply",json!({"kind":"company_ammo_purchase","company":company,"product":ammo_product,"quantity":500}));
    let ammunition_delivery = g.world.companies.ammunition_deliveries.last().unwrap().id;
    // Activation is prospective. Advance through genuine native contacts until
    // a whole inventory loss is observed, with a bounded 20-day failure guard.
    for index in 1..=20 {
        journey.day(&mut g, &format!("combat_day_{index}"));
        if journey.saw_loss
            && eq::ammunition_active(g.world.nation(ME), clock::absolute_day(&g.world))
            && g.world
                .nation(ME)
                .equipment
                .as_ref()
                .unwrap()
                .ammunition
                .as_ref()
                .unwrap()
                .consumed
                .values()
                .sum::<f64>()
                > 0.0
        {
            break;
        }
    }
    assert!(
        journey.saw_loss,
        "Two simultaneous fronts must produce an actual whole vehicle loss"
    );
    assert!(
        g.world
            .nation(ME)
            .equipment
            .as_ref()
            .unwrap()
            .ammunition
            .as_ref()
            .unwrap()
            .consumed
            .values()
            .sum::<f64>()
            > 0.0
    );
    journey.command(
        &mut g,
        "reserve_front_one",
        json!({"kind":"force_allocation","conflict":conflicts[0],"share_bp":0}),
    );
    journey.command(
        &mut g,
        "reserve_front_two",
        json!({"kind":"force_allocation","conflict":conflicts[1],"share_bp":0}),
    );
    for index in 1..=120 {
        let service = g
            .world
            .companies
            .firms
            .iter()
            .find(|c| c.id == company)
            .unwrap()
            .refits
            .iter()
            .find(|r| r.id == refit)
            .unwrap();
        let received = g
            .world
            .companies
            .deliveries
            .iter()
            .find(|d| d.id == vehicle_delivery)
            .unwrap()
            .delivered_day
            .is_some()
            && g.world
                .companies
                .ammunition_deliveries
                .iter()
                .find(|d| d.id == ammunition_delivery)
                .unwrap()
                .delivered_day
                .is_some();
        if service.completed_units == 1 && received {
            break;
        }
        journey.day(&mut g, &format!("service_day_{index}"));
    }
    let firm = g
        .world
        .companies
        .firms
        .iter()
        .find(|c| c.id == company)
        .unwrap();
    assert_eq!(
        firm.refits
            .iter()
            .find(|r| r.id == refit)
            .unwrap()
            .completed_units,
        1
    );
    assert!(g
        .world
        .companies
        .deliveries
        .iter()
        .find(|d| d.id == vehicle_delivery)
        .unwrap()
        .delivered_day
        .is_some());
    assert!(g
        .world
        .companies
        .ammunition_deliveries
        .iter()
        .find(|d| d.id == ammunition_delivery)
        .unwrap()
        .delivered_day
        .is_some());
    assert!(
        g.world
            .nation(ME)
            .arsenal
            .held
            .iter()
            .find(|h| h.design_id.as_deref() == Some(&target))
            .unwrap()
            .units
            >= 2.0
    );
    let tanks = g
        .world
        .nation(ME)
        .arsenal
        .held
        .iter()
        .find(|h| h.design_id.as_deref() == Some("s11-tank"))
        .unwrap()
        .units;
    journey.command(
        &mut g,
        "retire",
        json!({"kind":"equipment_retire","revision":"s11-tank","quantity":1}),
    );
    assert_eq!(
        g.world
            .nation(ME)
            .arsenal
            .held
            .iter()
            .find(|h| h.design_id.as_deref() == Some("s11-tank"))
            .unwrap()
            .units,
        tanks - 1.0
    );
    let manifest = json!({"version":1,"fixture":"s11-authored-ground-equipment-loop","compiled_revision":env!("SPHERES_REVISION"),
        "player":"France","before_file":"before.json","opening_day":opening_day,"days_advanced":journey.days,
        "scope":"Explicit authored France military and supplier scenario; ordinary reviewed controls and full daily continuation after import. Not historical equipment holdings, an unassisted campaign or final campaign certification.",
        "authored_preconditions":["France seed 1990 with daily military, manufacturing, logistics and departmental budgets enabled",
            "Existing native manufacturer fixture grants 10 baseline IFVs, $100bn treasury, no debt, 1000 political capital, raw inventories of one million units per commodity, an arms plant and bounded opening departmental authority",
            "The manufacturer and night-optics IFV revision were established and certified through reviewed commands and partial daily economic work before this archive",
            "Second arms-plant slot authored for public ammunition fabrication; baseline certified holdings authored: 5000 tanks and 1000 each APC, reconnaissance, artillery and air defense",
            "One supplier IFV and 1000 supplier autocannon rounds were actually manufactured, then stock targets set to zero; 20000 rounds of each compatible national family came from paid public fabrication orders",
            "Connected economy, population, fiscal recovery, company network, supplier catalogue and campaign leadership adopted through existing mature-save enrollment APIs before export; four local grid and generation levels and 100 advanced components are authored opening support property",
            "Starting conflicts, defense pacts and sanctions cleared; native France-Italy and France-Spain conflicts authored in Western Europe, at invasion posture with French seize and opposing hold objectives",
            "Operational migration enabled on those fronts; France advance orders use zero tactical reserve. No force, stock, money, grants, dates or outcomes are authored after before.json"],
        "preparation_days_after_manufacturer_ready":opening_day-setup_start,"commands":journey.commands,"steps":journey.steps,
        "expected_stages":journey.stages,"native_confirmations":journey.reviews,"require_losses":true,
        "required_outcomes":{"company":company,"refit":refit,"vehicle_delivery":vehicle_delivery,"ammunition_delivery":ammunition_delivery,
            "target_revision":target,"retired_revision":"s11-tank","retired_quantity":1,"conflicts":conflicts}});
    exclusive(
        &journey.destination.join("manifest.json"),
        serde_json::to_string_pretty(&manifest).unwrap().as_bytes(),
    );
    println!(
        "S11_FIXTURE_MANIFEST={}",
        journey.destination.join("manifest.json").display()
    );
}
