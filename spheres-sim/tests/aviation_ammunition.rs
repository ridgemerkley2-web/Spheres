//! Tactical aviation consumes paid compatible stores through the existing world.
//! Certified designs and airframes are explicit synthetic fixture inputs; stores
//! are manufactured by normal dated commands, never granted on aircraft delivery.
use spheres_sim::world::{Belligerent, Conflict, GameRules, NationId as N, Objective, WorldState};
use spheres_sim::{
    arsenal, clock, equipment, operations, production, programs, resources, theatre, Command,
    EquipmentOrder,
};

const USA: N = N::USA;
const BOMBS: &str = "air_bomb_unguided";

fn command(order: EquipmentOrder) -> Command {
    Command::Equipment { nation: USA, order }
}
fn step(w: &mut WorldState, commands: &[Command]) {
    let events = spheres_sim::tick_day(w, commands);
    assert!(
        !events.iter().any(|e| e.starts_with("[rejected]")),
        "{events:?}"
    );
    equipment::validate_state(w.nation(USA)).unwrap();
}
fn certify(w: &mut WorldState, id: &str, spec: equipment::DesignSpec) {
    certify_for(w, USA, id, spec);
}
fn certify_for(w: &mut WorldState, nation: N, id: &str, spec: equipment::DesignSpec) {
    let profile = equipment::design_preview(w, nation, &spec).profile.unwrap();
    let day = clock::absolute_day(w);
    w.nation_mut(nation)
        .equipment
        .get_or_insert_with(Default::default)
        .revisions
        .insert(
            id.into(),
            equipment::DesignRevision {
                id: id.into(),
                name: format!("Synthetic {id}"),
                specification_key: equipment::specification_key(&spec),
                spec,
                profile,
                created_day: day,
                certified_day: Some(day),
            },
        );
}
fn fixture() -> (WorldState, String) {
    let mut w = spheres_sim::init::world_1990(GameRules {
        daily_simulation: true,
        military_operations: true,
        production_system: true,
        manufacturing_system: true,
        resource_market: true,
        resource_gates: true,
        ..Default::default()
    });
    w.player = Some(USA);
    w.conflicts.clear();
    programs::set_construction_budget(&mut w, USA, 0.0).unwrap();
    certify(&mut w, "air", equipment::default_spec("air_light_attack"));
    let mut guided = equipment::default_spec("air_light_attack");
    guided
        .components
        .insert("air_payload".into(), "air_payload_guided".into());
    guided
        .components
        .insert("air_avionics".into(), "air_avionics_digital".into());
    certify(&mut w, "guided", guided);
    let mut refit = equipment::default_spec("air_light_attack");
    refit
        .components
        .insert("air_fuel".into(), "air_fuel_extended".into());
    certify(&mut w, "refit", refit);
    certify(&mut w, "ground", equipment::baseline_spec());
    w.nation_mut(USA).arsenal.held.clear();
    w.nation_mut(USA).arsenal.orders.clear();
    let district = w
        .districts
        .iter()
        .find(|(_, owner)| **owner == USA)
        .unwrap()
        .0
        .clone();
    if let Some(p) = w
        .production
        .provinces
        .iter_mut()
        .find(|p| p.district == district)
    {
        p.arms_plants = 3;
    } else {
        w.production
            .provinces
            .push(production::ProvinceCapabilities {
                district: district.clone(),
                infrastructure: 0,
                civilian_industry: 0,
                power_grid: 0,
                research_centers: 0,
                arms_plants: 3,
            });
        w.production
            .provinces
            .sort_by(|a, b| a.district.cmp(&b.district));
    }
    step(
        &mut w,
        &[command(EquipmentOrder::Maintenance {
            daily_budget_bn: 1.0,
        })],
    );
    let market = w.resources.market.as_mut().unwrap();
    for commodity in [
        resources::Commodity::Iron,
        resources::Commodity::Copper,
        resources::Commodity::Coal,
    ] {
        if let Some(s) = market
            .stocks
            .iter_mut()
            .find(|s| s.nation == USA && s.commodity == commodity)
        {
            s.quantity = 10_000.0;
        } else {
            market.stocks.push(resources::Stock {
                nation: USA,
                commodity,
                quantity: 10_000.0,
                reserve_target: 0.0,
            });
        }
    }
    market.stocks.sort_by_key(|s| (s.nation, s.commodity));
    arsenal::deliver_design(w.nation_mut(USA), "air", 10, 0.0).unwrap();
    assert!(w
        .nation(USA)
        .equipment
        .as_ref()
        .unwrap()
        .ammunition
        .is_none());
    step(&mut w, &[]);
    step(
        &mut w,
        &[command(EquipmentOrder::AmmoOrder {
            family: BOMBS.into(),
            district: district.clone(),
            quantity: 60,
            daily_budget_bn: 0.01,
        })],
    );
    step(&mut w, &[]);
    assert_eq!(
        w.nation(USA)
            .equipment
            .as_ref()
            .unwrap()
            .ammunition
            .as_ref()
            .unwrap()
            .stocks[BOMBS],
        60.0
    );
    assert!(!equipment::ammunition_active(
        w.nation(USA),
        clock::absolute_day(&w)
    ));
    assert!(equipment::physical_ammunition_required(
        w.nation(USA),
        clock::absolute_day(&w)
    ));
    w.conflicts.clear();
    (w, district)
}
fn conflict(w: &WorldState, id: u32, opponent: N, rung: u8) -> Conflict {
    Conflict {
        id,
        theatre: spheres_sim::war::theatre_between(w, USA, opponent),
        side_a: vec![USA],
        side_b: vec![opponent],
        posture: vec![
            Belligerent::new(USA, rung, Objective::Seize),
            Belligerent::new(opponent, 8, Objective::Hold),
        ],
        control: 0.0,
        months: 0,
        quiet_months: 0,
        frozen_since: None,
        start_year: w.year,
        start_month: w.month,
        origin_attacker: USA,
        invasion_declared: true,
        front: Default::default(),
        pockets: vec![],
        aim: None,
    }
}
fn deployment(w: &WorldState) -> operations::Deployment {
    operations::view(w, USA).deployments[0].clone()
}
fn need(w: &WorldState, family: &str) -> f64 {
    equipment::ammunition_overview(w, USA)
        .families
        .iter()
        .find(|f| f.family == family)
        .map_or(0.0, |f| f.required)
}
fn near(a: f64, b: f64) {
    assert!(
        (a - b).abs() < 1e-10 * a.abs().max(b.abs()).max(1.0),
        "{a} != {b}"
    );
}
fn remaining(w: &mut WorldState, quantity: f64) {
    // This clone represents already-expended stores. Preserve the paid batch
    // ledger and total delivered=remaining+consumed while isolating coverage.
    let a = w
        .nation_mut(USA)
        .equipment
        .as_mut()
        .unwrap()
        .ammunition
        .as_mut()
        .unwrap();
    a.stocks.insert(BOMBS.into(), quantity);
    a.consumed.insert(BOMBS.into(), 60.0 - quantity);
}

#[test]
fn aircraft_require_exact_paid_stores_and_partial_coverage_gates_strike_once() {
    let (mut full, _) = fixture();
    full.conflicts.push(conflict(&full, 1, N::Canada, 6));
    let required = need(&full, BOMBS);
    assert!(required > 0.0 && required < 60.0);
    let before = spheres_sim::save(&full);
    let ready = deployment(&full).ammunition.unwrap();
    assert!(ready.attack_coefficient > 0.0);
    let valuation = (
        arsenal::book_value(full.nation(USA)),
        arsenal::adequacy(full.nation(USA)),
        operations::capabilities(full.nation(USA)),
    );
    let mut partial = full.clone();
    remaining(&mut partial, required / 2.0);
    let limited = deployment(&partial).ammunition.unwrap();
    near(limited.attack_coefficient, ready.attack_coefficient / 2.0);
    near(limited.fire_fraction, ready.fire_fraction / 2.0);
    let mut dry = full.clone();
    remaining(&mut dry, 0.0);
    let empty = deployment(&dry).ammunition.unwrap();
    near(empty.attack_coefficient, 0.0);
    near(empty.maneuver_fraction, 0.0);
    near(empty.legacy_share, 0.0);
    assert!(empty.physical_dry);
    assert_eq!(arsenal::book_value(dry.nation(USA)), valuation.0);
    assert_eq!(arsenal::adequacy(dry.nation(USA)), valuation.1);
    assert_eq!(
        serde_json::to_value(operations::capabilities(dry.nation(USA))).unwrap(),
        serde_json::to_value(valuation.2).unwrap()
    );
    let mut wrong = full.clone();
    wrong.nation_mut(USA).arsenal.held[0].design_id = Some("guided".into());
    assert!(need(&wrong, "air_bomb_guided") > 0.0);
    near(need(&wrong, BOMBS), 0.0);
    near(
        deployment(&wrong).ammunition.unwrap().attack_coefficient,
        0.0,
    );
    assert_eq!(
        spheres_sim::save(&full),
        before,
        "Operational previews must not spend stores."
    );
}

#[test]
fn inaccessible_grounded_in_transit_and_refitting_aircraft_cannot_fly() {
    let (mut w, district) = fixture();
    w.conflicts.push(conflict(&w, 1, N::Canada, 6));
    let required = need(&w, BOMBS);
    arsenal::queue_design_order(w.nation_mut(USA), "air", 10, 7, 0.0).unwrap();
    near(need(&w, BOMBS), required);
    let mut no_base = w.clone();
    no_base.access.clear();
    no_base.conflicts.clear();
    no_base.conflicts.push(conflict(&no_base, 2, N::Iraq, 6));
    assert!(!theatre::has_access(
        &no_base,
        USA,
        no_base.conflicts[0].theatre
    ));
    near(need(&no_base, BOMBS), 0.0);
    near(
        deployment(&no_base).ammunition.unwrap().attack_coefficient,
        0.0,
    );
    let mut grounded = w.clone();
    equipment::set_maintenance_plan(&mut grounded, USA, 0.0).unwrap();
    programs::begin_day(&mut grounded);
    equipment::settle_support(&mut grounded);
    near(need(&grounded, BOMBS), 0.0);
    near(
        deployment(&grounded).ammunition.unwrap().attack_coefficient,
        0.0,
    );
    equipment::validate_state(grounded.nation(USA)).unwrap();
    equipment::start_refit(&mut w, USA, "air", "refit", &district, 10, 0.001).unwrap();
    assert!(equipment::physical_ammunition_required(
        w.nation(USA),
        clock::absolute_day(&w)
    ));
    near(need(&w, BOMBS), 0.0);
    let out = deployment(&w).ammunition.unwrap();
    near(out.attack_coefficient, 0.0);
    near(out.legacy_share, 0.0);
    equipment::validate_state(w.nation(USA)).unwrap();
}

#[test]
fn aviation_does_not_activate_ground_stores_or_supply_ground_firing() {
    let (mut w, _) = fixture();
    w.conflicts.push(conflict(&w, 1, N::Canada, 8));
    let air_only = deployment(&w).ammunition.unwrap();
    near(air_only.attack_coefficient, 0.0);
    near(air_only.maneuver_fraction, 0.0);
    near(need(&w, BOMBS), 0.0);
    arsenal::deliver_design(w.nation_mut(USA), "ground", 10, 0.0).unwrap();
    let mixed = deployment(&w);
    assert!(mixed.ammunition.unwrap().legacy_share > 0.0);
    assert!(mixed.ammunition.unwrap().attack_coefficient > 0.0);
    assert!(mixed.burn_monthly > 0.0);
    near(need(&w, "tank_105_mixed"), 0.0);
    assert!(!equipment::ammunition_active(
        w.nation(USA),
        clock::absolute_day(&w)
    ));
    step(&mut w, &[command(EquipmentOrder::AmmoActivate)]);
    assert!(equipment::ammunition_active(
        w.nation(USA),
        clock::absolute_day(&w)
    ));
    assert!(need(&w, "tank_105_mixed") > 0.0);
    near(need(&w, BOMBS), 0.0);
    near(deployment(&w).ammunition.unwrap().attack_coefficient, 0.0);
    near(deployment(&w).burn_monthly, 0.0);
}

#[test]
fn air_only_consumption_is_conserved_across_conflicts_and_saved_continuations() {
    let (mut w, _) = fixture();
    w.conflicts.push(conflict(&w, 1, N::Canada, 6));
    w.conflicts.push(conflict(&w, 2, N::Mexico, 6));
    let view = operations::view(&w, USA);
    assert_eq!(view.deployments.len(), 2);
    assert!(view.deployments.iter().map(|d| d.deployed).sum::<f64>() <= w.nation(USA).mil_strength);
    assert!(view.deployments.iter().all(|d| d.burn_monthly == 0.0));
    near(equipment::ammunition_legacy_refill_share(&w, USA), 0.0);
    let before_magazine = w.nation(USA).munitions;
    let mut reversed = w.clone();
    reversed.conflicts.reverse();
    let expected = equipment::ammunition_overview(&w, USA);
    assert_eq!(
        serde_json::to_value(&expected).unwrap(),
        serde_json::to_value(equipment::ammunition_overview(&reversed, USA)).unwrap()
    );
    step(&mut w, &[]);
    step(&mut reversed, &[]);
    let a = w
        .nation(USA)
        .equipment
        .as_ref()
        .unwrap()
        .ammunition
        .as_ref()
        .unwrap();
    let reversed_ammo = reversed
        .nation(USA)
        .equipment
        .as_ref()
        .unwrap()
        .ammunition
        .as_ref()
        .unwrap();
    assert_eq!(a, reversed_ammo);
    assert!(a.active_from_day.is_none());
    assert!(a.consumed[BOMBS] > 0.0);
    near(a.stocks[BOMBS] + a.consumed[BOMBS], 60.0);
    near(w.nation(USA).munitions, before_magazine);
    let saved = spheres_sim::save(&w);
    let mut resumed = spheres_sim::load(&saved).unwrap();
    assert_eq!(saved, spheres_sim::save(&resumed));
    for _ in 0..3 {
        step(&mut w, &[]);
        step(&mut resumed, &[]);
        assert_eq!(
            spheres_sim::state_hash(&w),
            spheres_sim::state_hash(&resumed)
        );
        let a = w
            .nation(USA)
            .equipment
            .as_ref()
            .unwrap()
            .ammunition
            .as_ref()
            .unwrap();
        near(a.stocks[BOMBS] + a.consumed[BOMBS], 60.0);
    }
}

#[test]
fn only_actually_launched_custom_raids_create_defender_air_defense_demand() {
    let (mut w, _) = fixture();
    w.player = Some(N::Canada);
    programs::set_construction_budget(&mut w, N::Canada, 0.0).unwrap();
    certify_for(
        &mut w,
        N::Canada,
        "aa",
        equipment::default_spec("ground_air_defense"),
    );
    w.nation_mut(N::Canada).arsenal.held.clear();
    w.nation_mut(N::Canada).arsenal.orders.clear();
    arsenal::deliver_design(w.nation_mut(N::Canada), "aa", 10, 0.0).unwrap();
    equipment::set_maintenance_plan(&mut w, N::Canada, 1.0).unwrap();
    equipment::activate_ammunition(&mut w, N::Canada).unwrap();
    w.player = Some(USA);
    step(&mut w, &[]);
    step(&mut w, &[]);
    w.conflicts.clear();
    w.conflicts.push(conflict(&w, 1, N::Canada, 6));
    let aa_need = |world: &WorldState| {
        equipment::ammunition_overview(world, N::Canada)
            .families
            .iter()
            .filter(|f| f.family.starts_with("aa_"))
            .map(|f| f.required)
            .sum::<f64>()
    };
    assert!(aa_need(&w) > 0.0);
    let mut dry = w.clone();
    remaining(&mut dry, 0.0);
    near(aa_need(&dry), 0.0);
    let mut wrong = w.clone();
    wrong.nation_mut(USA).arsenal.held[0].design_id = Some("guided".into());
    near(aa_need(&wrong), 0.0);
    let mut denied = w.clone();
    denied.access.clear();
    denied.conflicts[0].theatre = spheres_sim::war::theatre_between(&denied, USA, N::Iraq);
    assert!(!theatre::has_access(
        &denied,
        USA,
        denied.conflicts[0].theatre
    ));
    near(aa_need(&denied), 0.0);
}
