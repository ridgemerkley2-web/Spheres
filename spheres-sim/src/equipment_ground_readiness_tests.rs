//! S11: servicing controls the existing physical weapon contribution. These
//! authored fleets and paid-store fixtures test accounting, not campaign play.
use super::*;
use crate::{arsenal, operations, programs, world::NationId};

const USA: NationId = NationId::USA;

fn near(a: f64, b: f64) {
    assert!(
        (a - b).abs() <= 1e-10 * a.abs().max(b.abs()).max(1.0),
        "{a} != {b}"
    );
}

fn service(w: &mut WorldState, fraction: f64) {
    // The shared fixture activates this plan and opens its next funding day.
    // Updating the limit preserves from_day, then this is the first invoice.
    assert!(maintenance_plan_on(w.nation(USA), clock::absolute_day(w)));
    let requirement = fleet_maintenance_requirement(w.nation(USA));
    set_maintenance_plan(w, USA, requirement * fraction).unwrap();
    settle_support(w);
    let state = w.nation(USA).equipment.as_ref().unwrap();
    near(state.maintenance_fraction, fraction);
    let receipt = state
        .maintenance_plan
        .as_ref()
        .unwrap()
        .receipt
        .as_ref()
        .unwrap();
    near(receipt.custom_paid_bn, requirement * fraction);
}

fn deployment() -> AmmoDeployment {
    AmmoDeployment {
        ground: true,
        deployed_share: 0.5,
        aircraft_share: 0.0,
        intensity: 0.75,
        air_exposure: 0.4,
    }
}

#[test]
fn s11_ground_weapons_require_paid_support_for_every_family() {
    for platform in [
        "tank_standard",
        "ground_ifv",
        "ground_apc",
        "ground_recon",
        "ground_artillery",
        "ground_air_defense",
    ] {
        for fraction in [0.0, 0.5, 1.0] {
            let mut w = ammunition_operation_tests::fixture(platform);
            ammunition_operation_tests::stock(&mut w, 100_000);
            w.conflicts[0].posture_mut(NationId::Canada).unwrap().rung = 6;
            service(&mut w, fraction);
            let saved = crate::save(&w);
            let d = deployment();
            let plan = plan_ammunition(&w, USA, &[d]);
            let row = plan.families.iter().find(|row| row.vehicles > 0.0).unwrap();
            let anti_air = platform == "ground_air_defense";
            let expected = 1000.0
                * ammo_def(&row.family).unwrap().rounds_per_vehicle_month
                * 0.5
                * 0.75
                * if anti_air { 0.4 } else { 1.0 }
                * fraction
                * clock::month_fraction(&w);
            near(row.vehicles, 1000.0);
            near(row.required, expected);
            near(row.used, expected);
            near(plan.legacy_share, 0.0);
            let caps = operations::capabilities(w.nation(USA));
            let effects = ammunition_effects(&w, USA, &d, caps, &plan);
            near(effects.fire_fraction, if anti_air { 0.0 } else { fraction });
            let ratings = profile(w.nation(USA), "ammo-model")
                .unwrap()
                .ground_roles
                .unwrap_or_default();
            near(
                effects.fire_support,
                (ratings.fire_support * 0.20).min(0.25) * fraction,
            );
            near(
                effects.air_defense,
                (ratings.air_defense * 0.25).min(0.35) * fraction,
            );
            if fraction == 0.0 {
                near(effects.attack_coefficient, 0.0);
                near(effects.fire_support, 0.0);
                near(effects.air_defense, 0.0);
                let actual = operations::view(&w, USA).deployments[0].ammunition.unwrap();
                near(actual.fire_fraction, 0.0);
                near(actual.attack_coefficient, 0.0);
                assert!(ammunition_overview(&w, USA)
                    .families
                    .iter()
                    .all(|r| r.used == 0.0));
            }
            near(effects.maneuver_fraction, 1.0);
            assert_eq!(
                crate::save(&w),
                saved,
                "a readiness quote must not spend or mutate"
            );
            let loaded = crate::load(&saved).unwrap();
            assert_eq!(
                crate::save(&loaded),
                saved,
                "actual support invoices remain loadable"
            );
        }
    }
}

#[test]
fn s11_ground_condition_scales_rounds_and_fire_once_without_repricing_the_reference() {
    let mut w = ammunition_operation_tests::fixture("tank_standard");
    let life = profile(w.nation(USA), "ammo-model").unwrap().service_months;
    w.nation_mut(USA).arsenal.held[0].age = life as f64 * 0.5;
    ammunition_operation_tests::stock(&mut w, 100_000);
    service(&mut w, 0.5);
    // The existing hull condition is linear to 35% residual over its service
    // life. Half-life hulls therefore supply 67.5%, then half servicing applies.
    let readiness = 0.675 * 0.5;
    let d = deployment();
    let plan = plan_ammunition(&w, USA, &[d]);
    let row = &plan.families[0];
    let expected = 1000.0
        * ammo_def(&row.family).unwrap().rounds_per_vehicle_month
        * 0.5
        * 0.75
        * clock::month_fraction(&w)
        * readiness;
    near(row.required, expected);
    near(row.coverage, 1.0);
    let effects = ammunition_effects(&w, USA, &d, operations::capabilities(w.nation(USA)), &plan);
    near(effects.fire_fraction, readiness);
    near(effects.legacy_share, 0.0);
    near(effects.maneuver_fraction, 1.0);
    let mut repriced = w.clone();
    repriced
        .nation_mut(USA)
        .equipment
        .as_mut()
        .unwrap()
        .revisions
        .get_mut("ammo-model")
        .unwrap()
        .profile
        .unit_cost_bn *= 1000.0;
    let changed = plan_ammunition(&repriced, USA, &[d]);
    near(changed.families[0].required, expected);
    near(
        ammunition_effects(
            &repriced,
            USA,
            &d,
            operations::capabilities(repriced.nation(USA)),
            &changed,
        )
        .fire_fraction,
        readiness,
    );
}

fn add_aircraft(w: &mut WorldState) {
    let spec = default_spec("air_light_attack");
    let p = design_preview(w, USA, &spec).profile.unwrap();
    let day = clock::absolute_day(w);
    state_mut(w.nation_mut(USA)).revisions.insert(
        "parked-aircraft".into(),
        DesignRevision {
            id: "parked-aircraft".into(),
            name: "Parked aircraft".into(),
            specification_key: specification_key(&spec),
            spec,
            profile: p,
            created_day: day,
            certified_day: Some(day),
        },
    );
    arsenal::deliver_design(w.nation_mut(USA), "parked-aircraft", 100, 0.0).unwrap();
}

#[test]
fn s11_ground_mixed_age_specialists_apply_condition_once_beside_legacy_stock() {
    let mut w = ammunition_operation_tests::fixture("ground_artillery");
    let p = profile(w.nation(USA), "ammo-model").unwrap().clone();
    let reference = 1000.0 * p.reference_weight_bn;
    w.nation_mut(USA).arsenal.held[0].age = p.service_months as f64 * 0.5;
    let kit = arsenal::index_of("arm_gen3").unwrap();
    w.nation_mut(USA).arsenal.held.push(arsenal::Holding {
        kit,
        units: reference / arsenal::DECK[kit as usize].unit_cost,
        age: 0.0,
        design_id: None,
        refit_reserved: 0,
        loss_remainder: 0.0,
    });
    ammunition_operation_tests::stock(&mut w, 100_000);
    let n = w.nation(USA);
    let required = fleet_maintenance_requirement(n) + legacy_maintenance_requirement(n);
    set_maintenance_plan(&mut w, USA, required).unwrap();
    settle_support(&mut w);
    near(
        w.nation(USA)
            .equipment
            .as_ref()
            .unwrap()
            .maintenance_fraction,
        1.0,
    );
    near(legacy_maintenance_fraction(w.nation(USA)), 1.0);
    let d = deployment();
    let plan = plan_ammunition(&w, USA, &[d]);
    let effects = ammunition_effects(&w, USA, &d, operations::capabilities(w.nation(USA)), &plan);
    // Equal physical cohorts: 67.5% condition artillery and fresh legacy armor.
    // Only the artillery half contributes specialist support, once at condition.
    near(
        effects.fire_support,
        p.ground_roles.unwrap().fire_support * 0.20 * 0.675 / 2.0,
    );
    near(effects.fire_fraction, 0.5 + 0.5 * 0.675);
}

#[test]
fn s11_ground_support_gate_also_applies_beside_parked_aircraft() {
    for fraction in [0.0, 0.5, 1.0] {
        let mut w = ammunition_operation_tests::fixture("tank_standard");
        add_aircraft(&mut w);
        ammunition_operation_tests::stock(&mut w, 100_000);
        service(&mut w, fraction);
        let n = w.nation(USA);
        let ground = 1000.0 * profile(n, "ammo-model").unwrap().reference_weight_bn;
        let air = 100.0 * profile(n, "parked-aircraft").unwrap().reference_weight_bn;
        let d = deployment();
        let plan = plan_ammunition(&w, USA, &[d]);
        let ground_row = plan
            .families
            .iter()
            .find(|r| r.family == "tank_105_mixed")
            .unwrap();
        let expected = 1000.0
            * ammo_def("tank_105_mixed").unwrap().rounds_per_vehicle_month
            * 0.5
            * 0.75
            * clock::month_fraction(&w)
            * fraction;
        near(ground_row.required, expected);
        near(plan.legacy_share, 0.0);
        let effects = ammunition_effects(&w, USA, &d, operations::capabilities(n), &plan);
        near(effects.fire_fraction, ground / (ground + air) * fraction);
        near(effects.maneuver_fraction, ground / (ground + air));
        assert!(plan
            .families
            .iter()
            .filter(|r| r.family != "tank_105_mixed")
            .all(|r| r.used == 0.0));
        if fraction == 0.0 {
            near(effects.attack_coefficient, 0.0);
            let actual = operations::view(&w, USA).deployments[0].ammunition.unwrap();
            near(actual.attack_coefficient, 0.0);
        }
        let saved = crate::save(&w);
        assert_eq!(crate::save(&crate::load(&saved).unwrap()), saved);
    }
}

fn settle_two_fronts(w: &mut WorldState) {
    let mut snapshot = operations::Snapshot::new(w);
    snapshot.record_loss(1, USA, 0.2);
    snapshot.record_loss(2, USA, 0.3);
    snapshot.settle(w);
}

#[test]
fn s11_ground_partial_service_shares_stores_and_losses_across_fronts_and_reload() {
    let mut w = ammunition_operation_tests::fixture("tank_standard");
    let mut other = ammunition_operation_tests::conflict(&w, 2, NationId::Mexico);
    other.theatre = w.conflicts[0].theatre;
    w.conflicts.push(other);
    for conflict in [1, 2] {
        crate::apply_command(
            &mut w,
            &crate::Command::SetForceAllocation {
                conflict,
                nation: USA,
                share_bp: Some(5000),
            },
        )
        .unwrap();
    }
    ammunition_operation_tests::stock(&mut w, 250);
    service(&mut w, 0.5);
    let opening = w.nation(USA).mil_strength;
    let view = operations::view(&w, USA);
    near(view.deployed, opening);
    for row in &view.deployments {
        near(row.deployed, opening * 0.5);
    }
    let plan = ammunition_overview(&w, USA);
    let required = 1000.0
        * ammo_def("tank_105_mixed").unwrap().rounds_per_vehicle_month
        * clock::month_fraction(&w)
        * 0.5;
    near(plan.families[0].required, required);
    let used = required.min(250.0);
    near(plan.families[0].used, used);
    for row in &view.deployments {
        near(row.ammunition.unwrap().fire_fraction, 0.5 * used / required);
    }
    let saved = crate::save(&w);
    let mut loaded = crate::load(&saved).unwrap();
    let mut reversed = w.clone();
    reversed.conflicts.reverse();
    settle_two_fronts(&mut w);
    settle_two_fronts(&mut loaded);
    settle_two_fronts(&mut reversed);
    // Two half-force commitments lose 20% and 30% locally: 25% of national
    // force, with half of that fraction debited from the same tank inventory.
    near(w.nation(USA).mil_strength, opening * 0.75);
    near(w.nation(USA).arsenal.held[0].units, 875.0);
    near(w.nation(USA).arsenal.held[0].loss_remainder, 0.0);
    let stores = w
        .nation(USA)
        .equipment
        .as_ref()
        .unwrap()
        .ammunition
        .as_ref()
        .unwrap();
    near(stores.stocks["tank_105_mixed"], 250.0 - used);
    near(stores.consumed["tank_105_mixed"], used);
    assert_eq!(crate::save(&loaded), crate::save(&w));
    assert_eq!(
        serde_json::to_value(reversed.nation(USA)).unwrap(),
        serde_json::to_value(w.nation(USA)).unwrap()
    );
    let settled = crate::save(&w);
    settle_ammunition(&mut w, USA, &plan);
    assert_eq!(
        crate::save(&w),
        settled,
        "one dated store settlement cannot pay twice"
    );
    loaded = crate::load(&settled).unwrap();
    for world in [&mut w, &mut loaded] {
        clock::advance_date(world);
        programs::begin_day(world);
        service(world, 0.5);
        settle_two_fronts(world);
    }
    assert_eq!(crate::save(&loaded), crate::save(&w));
    validate_state(w.nation(USA)).unwrap();
}
