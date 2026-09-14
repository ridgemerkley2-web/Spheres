use super::*;
use crate::{init::world_1990, production, programs, world::GameRules};

const USA: NationId = NationId::USA;
const SOURCE: &str = "s12-light";
const TARGET: &str = "s12-light-extended";
const PENDING: &str = "s12-strike";

fn certify(w: &mut WorldState, key: &str, spec: equipment::DesignSpec) {
    let quote = equipment::design_preview(w, USA, &spec);
    assert!(quote.valid, "{:?}", quote.blockers);
    let today = clock::absolute_day(w);
    w.nation_mut(USA)
        .equipment
        .get_or_insert_with(Default::default)
        .revisions
        .insert(
            key.into(),
            equipment::DesignRevision {
                id: key.into(),
                name: key.into(),
                specification_key: equipment::specification_key(&spec),
                spec,
                profile: quote.profile.unwrap(),
                created_day: today,
                certified_day: Some(today),
            },
        );
}

fn fixture() -> (WorldState, String) {
    let mut w = world_1990(GameRules {
        daily_simulation: true,
        military_operations: true,
        production_system: true,
        manufacturing_system: true,
        resource_market: true,
        resource_gates: true,
        ..GameRules::default()
    });
    w.player = Some(USA);
    programs::set_construction_budget(&mut w, USA, 0.001).unwrap();
    let district = w
        .districts
        .iter()
        .find(|(_, owner)| **owner == USA)
        .unwrap()
        .0
        .clone();
    production::complete_capability(&mut w, &district, production::ProjectKind::ArmsPlant);
    certify(&mut w, SOURCE, equipment::default_spec("air_light_attack"));
    let mut upgraded = equipment::default_spec("air_light_attack");
    upgraded
        .components
        .insert("air_fuel".into(), "air_fuel_extended".into());
    certify(&mut w, TARGET, upgraded);
    certify(
        &mut w,
        PENDING,
        equipment::default_spec("air_tactical_strike"),
    );
    certify(&mut w, "s12-ground", equipment::baseline_spec());
    let n = w.nation_mut(USA);
    arsenal::deliver_design(n, SOURCE, 12, 15.25).unwrap();
    arsenal::deliver_design(n, TARGET, 6, 6.5).unwrap();
    arsenal::deliver_design(n, "s12-ground", 20, 11.0).unwrap();
    arsenal::queue_design_order(n, PENDING, 3, 1, 9.75).unwrap();
    equipment::validate_state(n).unwrap();
    (w, district)
}

fn create(w: &mut WorldState, name: &str, revision: &str, quantity: u32) {
    apply(
        w,
        USA,
        &SquadronCommand::Create {
            name: name.into(),
            revision: revision.into(),
            quantity,
        },
    )
    .unwrap();
}

fn stock(w: &WorldState) -> serde_json::Value {
    let n = w.nation(USA);
    serde_json::json!({
        "arsenal": n.arsenal, "equipment": n.equipment, "treasury": n.treasury_bn,
        "debt": n.debt_bn, "resources": w.resources, "companies": w.companies,
    })
}

fn refused_unchanged(w: &mut WorldState, id: NationId, command: SquadronCommand) {
    let before = crate::save(w);
    assert!(refusal(w, id, &command).is_some(), "{command:?}");
    assert_eq!(crate::save(w), before, "review must be pure");
    assert!(apply(w, id, &command).is_err(), "{command:?}");
    assert_eq!(crate::save(w), before, "refusal must precede mutation");
}

#[test]
fn s12_sparse_legacy_fractional_property_and_capability_survive_without_conversion() {
    let (mut w, _) = fixture();
    let kit = arsenal::index_of("air_gen3").unwrap();
    w.nation_mut(USA).arsenal.held.push(arsenal::Holding {
        kit,
        design_id: None,
        units: 0.375,
        age: 21.125,
        refit_reserved: 0,
        loss_remainder: 0.0,
    });
    assert!(w.nation(USA).aviation.is_none());
    let before = crate::save(&w);
    let summary = legacy_summary(w.nation(USA));
    let fractional = summary.last().unwrap();
    assert_eq!(
        fractional.formation_equivalents.to_bits(),
        0.375f64.to_bits()
    );
    assert_eq!(fractional.age_months.to_bits(), 21.125f64.to_bits());
    assert!(LEGACY_AIRCRAFT_NOTE.contains("No rounding"));
    assert_eq!(unassigned_units(w.nation(USA), "air_gen3"), 0);
    assert_eq!(
        ready_units(w.nation(USA), "air_gen3", clock::absolute_day(&w)),
        0
    );
    assert_eq!(crate::save(&w), before);
    refused_unchanged(
        &mut w,
        USA,
        SquadronCommand::Create {
            name: "Inherited".into(),
            revision: "air_gen3".into(),
            quantity: 1,
        },
    );
    let restored = crate::load(&before).unwrap();
    assert_eq!(crate::save(&restored), before);
    assert_eq!(legacy_summary(restored.nation(USA)), summary);
    assert!(restored.nation(USA).aviation.is_none());
    assert_eq!(
        arsenal::book_value(restored.nation(USA)),
        arsenal::book_value(w.nation(USA))
    );
    assert_eq!(
        serde_json::to_value(crate::operations::capabilities(restored.nation(USA))).unwrap(),
        serde_json::to_value(crate::operations::capabilities(w.nation(USA))).unwrap()
    );
}

#[test]
fn s12_pending_deliveries_cannot_be_assigned_and_receipt_never_fills_a_squadron() {
    let (mut w, _) = fixture();
    create(&mut w, "First", SOURCE, 4);
    let claims = w.nation(USA).aviation.clone();
    refused_unchanged(
        &mut w,
        USA,
        SquadronCommand::Create {
            name: "Pending strike".into(),
            revision: PENDING.into(),
            quantity: 1,
        },
    );
    // Advance only the existing paid delivery book in this authored fixture.
    for n in &mut w.nations {
        n.mil_spend_gdp = 0.0;
        n.arsenal.banked = 0.0;
    }
    programs::begin_day(&mut w);
    arsenal::tick(&mut w);
    reconcile(w.nation_mut(USA));
    assert_eq!(w.nation(USA).aviation, claims);
    assert_eq!(unassigned_units(w.nation(USA), PENDING), 3);
    let holding = w
        .nation(USA)
        .arsenal
        .held
        .iter()
        .find(|h| h.design_id.as_deref() == Some(PENDING))
        .unwrap();
    assert_eq!((holding.units, holding.age), (3.0, 9.75));
    let before = stock(&w);
    create(&mut w, "Received strike", PENDING, 3);
    assert_eq!(assigned_units(w.nation(USA), PENDING), 3);
    assert_eq!(
        stock(&w),
        before,
        "assignment cannot buy equipment or post another receipt"
    );
    validate(&w).unwrap();
}

#[test]
fn s12_two_squadrons_share_one_pool_and_revisions_exchange_claims_without_refitting() {
    let (mut w, _) = fixture();
    let before = stock(&w);
    create(&mut w, "First", SOURCE, 8);
    refused_unchanged(
        &mut w,
        USA,
        SquadronCommand::Create {
            name: "Too many".into(),
            revision: SOURCE.into(),
            quantity: 5,
        },
    );
    create(&mut w, "Second", SOURCE, 4);
    assert_eq!(
        (
            assigned_units(w.nation(USA), SOURCE),
            unassigned_units(w.nation(USA), SOURCE)
        ),
        (12, 0)
    );
    refused_unchanged(
        &mut w,
        USA,
        SquadronCommand::ChangeRevision {
            squadron: 1,
            revision: TARGET.into(),
        },
    );
    apply(
        &mut w,
        USA,
        &SquadronCommand::Resize {
            squadron: 1,
            quantity: 6,
        },
    )
    .unwrap();
    apply(
        &mut w,
        USA,
        &SquadronCommand::Resize {
            squadron: 2,
            quantity: 6,
        },
    )
    .unwrap();
    apply(
        &mut w,
        USA,
        &SquadronCommand::ChangeRevision {
            squadron: 1,
            revision: TARGET.into(),
        },
    )
    .unwrap();
    assert_eq!(assigned_units(w.nation(USA), SOURCE), 6);
    assert_eq!(unassigned_units(w.nation(USA), SOURCE), 6);
    assert_eq!(assigned_units(w.nation(USA), TARGET), 6);
    assert_eq!(unassigned_units(w.nation(USA), TARGET), 0);
    apply(
        &mut w,
        USA,
        &SquadronCommand::Resize {
            squadron: 2,
            quantity: 0,
        },
    )
    .unwrap();
    assert_eq!(assigned_units(w.nation(USA), SOURCE), 0);
    apply(&mut w, USA, &SquadronCommand::Disband { squadron: 1 }).unwrap();
    assert_eq!(unassigned_units(w.nation(USA), TARGET), 6);
    assert_eq!(w.nation(USA).aviation.as_ref().unwrap().next_id, 3);
    create(&mut w, "Third", TARGET, 1);
    assert_eq!(
        w.nation(USA)
            .aviation
            .as_ref()
            .unwrap()
            .squadrons
            .last()
            .unwrap()
            .id,
        3
    );
    assert_eq!(
        stock(&w),
        before,
        "all holdings, ages, components, stores and money stay unchanged"
    );
    validate(&w).unwrap();
    let saved = crate::save(&w);
    assert_eq!(crate::save(&crate::load(&saved).unwrap()), saved);
}

#[test]
fn s12_existing_refits_are_preserved_and_assigned_aircraft_cannot_be_refitted_or_retired() {
    let (mut w, district) = fixture();
    let refit = equipment::start_refit(&mut w, USA, SOURCE, TARGET, &district, 4, 0.0001).unwrap();
    assert_eq!(unassigned_units(w.nation(USA), SOURCE), 8);
    refused_unchanged(
        &mut w,
        USA,
        SquadronCommand::Create {
            name: "Overlaps refit".into(),
            revision: SOURCE.into(),
            quantity: 9,
        },
    );
    create(&mut w, "Protected", SOURCE, 8);
    let before = crate::save(&w);
    assert!(equipment::start_refit(&mut w, USA, SOURCE, TARGET, &district, 1, 0.0001).is_err());
    assert!(equipment::retire(&mut w, USA, SOURCE, 1).is_err());
    assert_eq!(crate::save(&w), before);
    let resumed = crate::load(&before).unwrap();
    assert_eq!(crate::save(&resumed), before);
    assert_eq!(assigned_units(resumed.nation(USA), SOURCE), 8);
    assert_eq!(unassigned_units(resumed.nation(USA), SOURCE), 0);
    equipment::cancel_project(&mut w, USA, refit).unwrap();
    assert_eq!(assigned_units(w.nation(USA), SOURCE), 8);
    assert_eq!(unassigned_units(w.nation(USA), SOURCE), 4);
    apply(
        &mut w,
        USA,
        &SquadronCommand::Resize {
            squadron: 1,
            quantity: 12,
        },
    )
    .unwrap();
    validate(&w).unwrap();
}

#[test]
fn s12_transit_service_and_unbased_aircraft_never_report_ready_or_escape_work_by_editing() {
    let (mut w, district) = fixture();
    create(&mut w, "First", SOURCE, 4);
    let today = clock::absolute_day(&w);
    assert_eq!(ready_units(w.nation(USA), SOURCE, today), 0);
    w.nation_mut(USA).aviation.as_mut().unwrap().squadrons[0].base = Some(district.clone());
    assert_eq!(ready_units(w.nation(USA), SOURCE, today), 4);
    let destination = w
        .districts
        .keys()
        .find(|id| **id != district)
        .unwrap()
        .clone();
    for service in [false, true] {
        {
            let q = &mut w.nation_mut(USA).aviation.as_mut().unwrap().squadrons[0];
            q.service_days_left = if service { 2 } else { 0 };
            q.transit = if service {
                None
            } else {
                Some(AirTransit {
                    from: Some(district.clone()),
                    to: destination.clone(),
                    departed_day: today - 3,
                    arrival_day: today - 1,
                })
            };
        }
        validate(&w).unwrap();
        assert_eq!(
            ready_units(w.nation(USA), SOURCE, today + 100),
            0,
            "reads cannot finish even overdue work"
        );
        for command in [
            SquadronCommand::Resize {
                squadron: 1,
                quantity: 0,
            },
            SquadronCommand::ChangeRevision {
                squadron: 1,
                revision: TARGET.into(),
            },
            SquadronCommand::Disband { squadron: 1 },
        ] {
            refused_unchanged(&mut w, USA, command);
        }
    }
    let q = &mut w.nation_mut(USA).aviation.as_mut().unwrap().squadrons[0];
    q.service_days_left = 0;
    assert_eq!(ready_units(w.nation(USA), SOURCE, today), 4);
}

#[test]
fn s12_loss_reconciliation_preserves_busy_assignments_is_idempotent_and_never_refills() {
    let (mut w, district) = fixture();
    w.airbases.get_or_insert_with(Default::default).bases.push(crate::airbases::Airbase {
        id: district.clone(),
        district: district.clone(),
        name: "Authored reconciliation test airfield".into(),
        sponsor: USA,
        capacity_level: 1,
        support_level: 0,
        protection_level: 0,
        project: None,
        history: vec![],
    });
    create(&mut w, "Operating", SOURCE, 6);
    create(&mut w, "Servicing", SOURCE, 4);
    {
        let n = w.nation_mut(USA);
        for q in &mut n.aviation.as_mut().unwrap().squadrons {
            q.base = Some(district.clone());
        }
        n.aviation.as_mut().unwrap().squadrons[1].service_days_left = 3;
        let h = n
            .arsenal
            .held
            .iter_mut()
            .find(|h| h.design_id.as_deref() == Some(SOURCE))
            .unwrap();
        h.units = 8.0;
        h.loss_remainder = 0.375;
    }
    let physical = stock(&w);
    reconcile(w.nation_mut(USA));
    let state = w.nation(USA).aviation.as_ref().unwrap();
    assert_eq!(
        (state.squadrons[0].assigned, state.squadrons[1].assigned),
        (4, 4)
    );
    assert_eq!(
        stock(&w),
        physical,
        "reconciliation cannot post a second physical casualty"
    );
    validate(&w).unwrap();
    let saved = crate::save(&w);
    let mut resumed = crate::load(&saved).unwrap();
    reconcile(w.nation_mut(USA));
    reconcile(resumed.nation_mut(USA));
    assert_eq!(crate::save(&w), saved);
    assert_eq!(crate::save(&resumed), saved);
    for world in [&mut w, &mut resumed] {
        arsenal::deliver_design(world.nation_mut(USA), SOURCE, 3, 0.0).unwrap();
        reconcile(world.nation_mut(USA));
        assert_eq!(assigned_units(world.nation(USA), SOURCE), 8);
        assert_eq!(unassigned_units(world.nation(USA), SOURCE), 3);
    }
    assert_eq!(crate::save(&w), crate::save(&resumed));
}

#[test]
fn s12_loss_reconciliation_uses_stable_ids_and_retains_empty_establishments() {
    let (mut w, _) = fixture();
    create(&mut w, "Oldest", SOURCE, 4);
    create(&mut w, "Middle", SOURCE, 4);
    create(&mut w, "Newest", SOURCE, 4);
    let n = w.nation_mut(USA);
    n.aviation.as_mut().unwrap().squadrons.swap(0, 2);
    n.arsenal
        .held
        .iter_mut()
        .find(|h| h.design_id.as_deref() == Some(SOURCE))
        .unwrap()
        .units = 5.0;
    reconcile(n);
    let claims: BTreeMap<_, _> = n
        .aviation
        .as_ref()
        .unwrap()
        .squadrons
        .iter()
        .map(|q| (q.id, q.assigned))
        .collect();
    assert_eq!(claims, BTreeMap::from([(1, 4), (2, 1), (3, 0)]));
    assert_eq!(n.aviation.as_ref().unwrap().next_id, 4);
    validate(&w).unwrap();
}

#[test]
fn s12_invalid_names_nonair_stock_foreign_commands_and_oversized_claims_are_atomic() {
    let (mut w, _) = fixture();
    for (name, revision, quantity) in [
        ("", SOURCE, 1),
        ("\nSquadron", SOURCE, 1),
        ("Empty", SOURCE, 0),
        ("Too large", SOURCE, u32::MAX),
        ("Ground", "s12-ground", 1),
        ("Missing", "missing", 1),
    ] {
        refused_unchanged(
            &mut w,
            USA,
            SquadronCommand::Create {
                name: name.into(),
                revision: revision.into(),
                quantity,
            },
        );
    }
    refused_unchanged(
        &mut w,
        NationId::France,
        SquadronCommand::Create {
            name: "Foreign".into(),
            revision: SOURCE.into(),
            quantity: 1,
        },
    );
    create(&mut w, "Valid", SOURCE, 1);
    refused_unchanged(
        &mut w,
        USA,
        SquadronCommand::Resize {
            squadron: 999,
            quantity: 1,
        },
    );
    refused_unchanged(
        &mut w,
        USA,
        SquadronCommand::Resize {
            squadron: 1,
            quantity: u32::MAX,
        },
    );
    refused_unchanged(
        &mut w,
        USA,
        SquadronCommand::ChangeRevision {
            squadron: 1,
            revision: "s12-ground".into(),
        },
    );
}

#[test]
fn s12_malformed_saved_claims_dates_and_custom_fractions_are_rejected_without_repair() {
    let (mut valid, district) = fixture();
    create(&mut valid, "Saved", SOURCE, 4);
    let today = clock::absolute_day(&valid);
    for case in 0..9 {
        let mut w = valid.clone();
        let state = w.nation_mut(USA).aviation.as_mut().unwrap();
        match case {
            0 => state.squadrons.push(state.squadrons[0].clone()),
            1 => state.squadrons[0].assigned = 13,
            2 => state.squadrons[0].revision = "air_gen3".into(),
            3 => state.next_id = 1,
            4 => state.last_service_day = Some(today + 1),
            5 => state.squadrons[0].base = Some("unknown-province".into()),
            6 => {
                state.squadrons[0].transit = Some(AirTransit {
                    from: None,
                    to: district.clone(),
                    departed_day: today + 1,
                    arrival_day: today + 3,
                })
            }
            7 => {
                state.squadrons[0].transit = Some(AirTransit {
                    from: Some(district.clone()),
                    to: district.clone(),
                    departed_day: today,
                    arrival_day: today,
                })
            }
            _ => {
                w.nation_mut(USA)
                    .arsenal
                    .held
                    .iter_mut()
                    .find(|h| h.design_id.as_deref() == Some(SOURCE))
                    .unwrap()
                    .units = 12.5;
            }
        }
        let saved = crate::save(&w);
        assert!(validate(&w).is_err(), "malformed case {case}");
        assert!(
            crate::load(&saved).is_err(),
            "loader must reject malformed case {case}"
        );
        assert_eq!(crate::save(&w), saved);
    }
    let mut unknown = serde_json::to_value(valid.nation(USA).aviation.as_ref().unwrap()).unwrap();
    unknown["free_aircraft"] = serde_json::json!(100);
    assert!(serde_json::from_value::<AviationState>(unknown).is_err());
}

#[test]
fn s13_squadron_resize_checks_real_shared_base_capacity_before_claiming_spares() {
    let (mut w, district) = fixture();
    create(&mut w, "First", SOURCE, 6);
    create(&mut w, "Second", TARGET, 6);
    w.airbases = Some(crate::airbases::AirbaseState {
        bases: vec![crate::airbases::Airbase {
            id: district.clone(),
            district: district.clone(),
            name: "Capacity fixture".into(),
            sponsor: USA,
            capacity_level: 1,
            support_level: 0,
            protection_level: 0,
            project: None,
            history: vec![],
        }],
        ..Default::default()
    });
    for q in &mut w.nation_mut(USA).aviation.as_mut().unwrap().squadrons {
        q.base = Some(district.clone());
    }
    assert_eq!(unassigned_units(w.nation(USA), SOURCE), 6);
    let command = SquadronCommand::Resize {
        squadron: 1,
        quantity: 8,
    };
    assert!(refusal(&w, USA, &command).unwrap().contains("spaces"));
    refused_unchanged(&mut w, USA, command.clone());
    apply(&mut w, USA, &SquadronCommand::Disband { squadron: 2 }).unwrap();
    apply(&mut w, USA, &command).unwrap();
    assert_eq!(crate::airbases::occupied_capacity(&w, &district), 8);
    validate(&w).unwrap();
}
