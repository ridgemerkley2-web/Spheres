/// Physical daily support requirement of delivered custom vehicles, including
/// refit reservations. Pending deliveries do not consume this service envelope.
/// This reads current stock; the last settlement's receipt is kept separately.
pub fn fleet_maintenance_requirement(n: &Nation) -> f64 {
    n.arsenal
        .held
        .iter()
        .filter_map(|h| {
            h.design_id.as_deref().and_then(|key| {
                profile(n, key).map(|p| p.maintenance_bn_day * h.units.max(0.0))
            })
        })
        .sum::<f64>()
}

#[derive(Clone, Debug, Serialize)]
pub struct RetirementQuote {
    pub valid: bool,
    pub reason: Option<String>,
    pub revision: String,
    pub quantity: u32,
    pub held_units: u32,
    pub reserved_units: u32,
    pub available_units: u32,
    pub remaining_units: u32,
    /// Whole custom fleet's physical requirement at the next support settlement.
    /// These are not treasury savings, appropriations, or revised past receipts.
    pub maintenance_bn_day_before: f64,
    pub maintenance_bn_day_after: f64,
    pub maintenance_reduction_bn_day: f64,
    pub note: String,
}

/// Review removal from delivered custom inventory without changing the world.
/// Source vehicles reserved by a refit and all pending deliveries are excluded.
pub fn retirement_quote(
    w: &WorldState,
    id: NationId,
    revision: &str,
    quantity: u32,
) -> RetirementQuote {
    let n = w.nation_opt(id);
    let mut reason = actor_refusal(w, id).or_else(|| {
        (!(1..=MAX_BATCH).contains(&quantity))
            .then(|| format!("Choose between one and {MAX_BATCH} vehicles to retire."))
    });
    if reason.is_none() {
        reason = n.and_then(|n| {
            certified(n, revision)
                .err()
                .or_else(|| validate_state(n).err())
        });
    }
    let h = n.and_then(|n| {
        n.arsenal
            .held
            .iter()
            .find(|h| h.design_id.as_deref() == Some(revision))
    });
    let held_units = h.map_or(0, |h| h.units as u32);
    let reserved_units = h.map_or(0, |h| h.refit_reserved);
    let available_units = h.map_or(0, crate::arsenal::available_design_units);
    if reason.is_none() && available_units < quantity {
        reason = Some(
            "There are not enough delivered, unreserved vehicles to retire. Pending deliveries and vehicles reserved for refit cannot be retired.".into(),
        );
    }
    let before = n.map_or(0.0, fleet_maintenance_requirement);
    let valid = reason.is_none();
    let after = if valid {
        let mut stock = n.unwrap().clone();
        remove_retired_holding(&mut stock, revision, quantity);
        fleet_maintenance_requirement(&stock)
    } else {
        before
    };
    RetirementQuote {
        valid,
        reason,
        revision: revision.into(),
        quantity,
        held_units,
        reserved_units,
        available_units,
        remaining_units: if valid { held_units - quantity } else { held_units },
        maintenance_bn_day_before: before,
        maintenance_bn_day_after: after,
        maintenance_reduction_bn_day: (before - after).max(0.0),
        note: "Retirement permanently removes these delivered vehicles without a cash refund or recovered materials. Remaining vehicles keep their average age. Lower physical maintenance demand takes effect at a future support settlement; current receipts and departmental allocations stay unchanged. Production orders and refit reservations continue.".into(),
    }
}

// Called only after the quote validates the entire operation. Remove only the
// requested row, leaving unrelated holdings and their ordering untouched.
fn remove_retired_holding(n: &mut Nation, revision: &str, quantity: u32) {
    let index = n.arsenal.held.iter()
        .position(|h| h.design_id.as_deref() == Some(revision))
        .expect("a valid retirement has delivered source vehicles");
    let h = &mut n.arsenal.held[index];
    h.units -= quantity as f64;
    // A partial retirement does not rejuvenate the survivors or erase accrued
    // attrition. Exhausting a row naturally removes its fractional-loss carry.
    if h.units == 0.0 {
        n.arsenal.held.remove(index);
    }
}

/// Commit a bounded retirement through the equipment command dispatcher.
/// No spending, refunds, material transfer, or support receipt is posted here.
pub fn retire(
    w: &mut WorldState,
    id: NationId,
    revision: &str,
    quantity: u32,
) -> Result<(), String> {
    let q = retirement_quote(w, id, revision, quantity);
    if let Some(reason) = q.reason {
        return Err(reason);
    }
    remove_retired_holding(w.nation_mut(id), revision, quantity);
    Ok(())
}

#[cfg(test)]
mod service_tests {
    use super::*;
    use crate::{arsenal, init::world_1990, world::{GameRules, BUDGET_DEFENSE}, Command, EquipmentOrder};
    const USA: NationId = NationId::USA;
    const SOURCE: &str = "service-source";
    const TARGET: &str = "service-upgrade";

    fn certify(w: &mut WorldState, key: &str, spec: DesignSpec) {
        let compiled = design_preview(w, USA, &spec);
        assert!(compiled.valid, "{:?}", compiled.blockers);
        let day = clock::absolute_day(w);
        state_mut(w.nation_mut(USA)).revisions.insert(key.into(), DesignRevision {
            id: key.into(), name: key.into(), specification_key: specification_key(&spec),
            spec, profile: compiled.profile.unwrap(), created_day: day, certified_day: Some(day),
        });
    }

    fn fixture() -> (WorldState, String) {
        let mut w = world_1990(GameRules {
            daily_simulation: true, military_operations: true, production_system: true,
            manufacturing_system: true, resource_market: true, resource_gates: true,
            ..GameRules::default()
        });
        w.player = Some(USA);
        crate::programs::set_construction_budget(&mut w, USA, 0.001).unwrap();
        let district = w.districts.iter().find(|(_, id)| **id == USA).unwrap().0.clone();
        crate::production::complete_capability(&mut w, &district, crate::production::ProjectKind::ArmsPlant);
        for commodity in crate::resources::ALL {
            crate::resources::set_stockpile_for_test(&mut w, USA, commodity, 1_000_000.0);
        }
        certify(&mut w, SOURCE, baseline_spec());
        let mut upgraded = baseline_spec();
        upgraded.components.insert("mobility".into(), "drive_mobile".into());
        certify(&mut w, TARGET, upgraded);
        let day = clock::absolute_day(&w);
        state_mut(w.nation_mut(USA)).finance_from_day = day;
        arsenal::deliver_design(w.nation_mut(USA), SOURCE, 8, 111.25).unwrap();
        arsenal::queue_design_order(w.nation_mut(USA), SOURCE, 5, DELIVERY_DAYS, 0.0).unwrap();
        crate::programs::begin_day(&mut w);
        settle_maintenance(&mut w, USA);
        validate_state(w.nation(USA)).unwrap();
        (w, district)
    }

    fn command(revision: &str, quantity: u32) -> Command {
        Command::Equipment { nation: USA, order: EquipmentOrder::Retire { revision: revision.into(), quantity } }
    }
    fn holding<'a>(w: &'a WorldState, revision: &str) -> &'a arsenal::Holding {
        w.nation(USA).arsenal.held.iter().find(|h| h.design_id.as_deref() == Some(revision)).unwrap()
    }
    fn close(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-12, "{a} != {b}");
    }

    #[test]
    fn retirement_quote_is_pure_and_quotes_physical_requirement_only() {
        let (w, _) = fixture();
        let saved = crate::save(&w);
        let unit = profile(w.nation(USA), SOURCE).unwrap().maintenance_bn_day;
        let q = retirement_quote(&w, USA, SOURCE, 3);
        assert!(q.valid);
        assert_eq!((q.held_units, q.reserved_units, q.available_units, q.remaining_units), (8, 0, 8, 5));
        close(q.maintenance_bn_day_before, unit * 8.0);
        close(q.maintenance_bn_day_after, unit * 5.0);
        close(q.maintenance_reduction_bn_day, unit * 3.0);
        assert!(q.note.contains("without a cash refund"));
        assert_eq!(crate::save(&w), saved);
    }

    #[test]
    fn retirement_refusals_are_atomic_and_pending_or_legacy_stock_cannot_be_used() {
        let (mut w, _) = fixture();
        for (revision, quantity) in [(SOURCE, 0), (SOURCE, MAX_BATCH + 1), (SOURCE, 9),
            (SOURCE, u32::MAX), (TARGET, 1), ("missing", 1), ("arm_gen3", 1)] {
            let saved = crate::save(&w);
            let q = retirement_quote(&w, USA, revision, quantity);
            assert!(!q.valid, "{revision}: {quantity}");
            assert_eq!(q.maintenance_bn_day_before, q.maintenance_bn_day_after);
            assert!(crate::apply_command(&mut w, &command(revision, quantity)).is_err());
            assert_eq!(crate::save(&w), saved);
        }
        for disabled in 0..4 {
            let mut invalid = w.clone();
            match disabled {
                0 => invalid.rules.daily_simulation = false,
                1 => invalid.rules.military_operations = false,
                2 => invalid.nation_mut(USA).alive = false,
                _ => invalid.player = Some(NationId::USSR),
            }
            let saved = crate::save(&invalid);
            assert!(crate::apply_command(&mut invalid, &command(SOURCE, 1)).is_err());
            assert_eq!(crate::save(&invalid), saved);
        }
    }

    #[test]
    fn partial_retirement_changes_only_units_and_preserves_refits_age_and_losses() {
        let (mut w, district) = fixture();
        let project = start_refit(&mut w, USA, SOURCE, TARGET, &district, 2, 1.0).unwrap();
        w.nation_mut(USA).arsenal.held.iter_mut().find(|h| h.design_id.as_deref() == Some(SOURCE)).unwrap().loss_remainder = 0.625;
        let q = retirement_quote(&w, USA, SOURCE, 3);
        assert_eq!((q.held_units, q.reserved_units, q.available_units, q.remaining_units), (8, 2, 6, 5));
        let mut expected = w.clone();
        expected.nation_mut(USA).arsenal.held.iter_mut().find(|h| h.design_id.as_deref() == Some(SOURCE)).unwrap().units = 5.0;
        crate::apply_command(&mut w, &command(SOURCE, 3)).unwrap();
        assert_eq!(crate::save(&w), crate::save(&expected), "no funding, receipts, orders, profiles, age, or attrition carry may change");
        assert_eq!(holding(&w, SOURCE).refit_reserved, 2);
        assert_eq!(holding(&w, SOURCE).age, 111.25);
        assert_eq!(holding(&w, SOURCE).loss_remainder, 0.625);
        validate_state(w.nation(USA)).unwrap();
        let before = crate::save(&w);
        assert!(crate::apply_command(&mut w, &command(SOURCE, 4)).is_err());
        assert_eq!(crate::save(&w), before);
        cancel_project(&mut w, USA, project).unwrap();
        crate::apply_command(&mut w, &command(SOURCE, 5)).unwrap();
        validate_state(w.nation(USA)).unwrap();
    }

    #[test]
    fn full_retirement_removes_only_delivered_row_and_later_delivery_starts_fresh() {
        let (mut w, _) = fixture();
        w.nation_mut(USA).arsenal.held.iter_mut().find(|h| h.design_id.as_deref() == Some(SOURCE)).unwrap().loss_remainder = 0.75;
        let mut expected = w.clone();
        expected.nation_mut(USA).arsenal.held.retain(|h| h.design_id.as_deref() != Some(SOURCE));
        crate::apply_command(&mut w, &command(SOURCE, 8)).unwrap();
        assert_eq!(crate::save(&w), crate::save(&expected));
        assert!(retirement_quote(&w, USA, SOURCE, 1).reason.unwrap().contains("delivered"));
        assert_eq!(w.nation(USA).arsenal.orders.iter().filter(|o| o.design_id.as_deref() == Some(SOURCE)).map(|o| o.units).sum::<f64>(), 5.0);
        assert_eq!(fleet_maintenance_requirement(w.nation(USA)), 0.0);
        validate_state(w.nation(USA)).unwrap();
        // A subsequent real delivery can still use the frozen revision; the
        // discarded cohort's age and loss carry must not infect fresh vehicles.
        arsenal::deliver_design(w.nation_mut(USA), SOURCE, 2, 0.0).unwrap();
        assert_eq!((holding(&w, SOURCE).units, holding(&w, SOURCE).age, holding(&w, SOURCE).loss_remainder), (2.0, 0.0, 0.0));
        validate_state(w.nation(USA)).unwrap();
    }

    #[test]
    fn retiring_all_unreserved_vehicles_still_allows_the_reserved_refit_to_finish() {
        let (mut w, district) = fixture();
        let project = start_refit(&mut w, USA, SOURCE, TARGET, &district, 2, 1.0).unwrap();
        let days = w.nation(USA).equipment.as_ref().unwrap().projects.iter().find(|p| p.id == project).unwrap().minimum_days;
        crate::apply_command(&mut w, &command(SOURCE, 6)).unwrap();
        assert_eq!((holding(&w, SOURCE).units, holding(&w, SOURCE).refit_reserved), (2.0, 2));
        assert_eq!(retirement_quote(&w, USA, SOURCE, 1).available_units, 0);
        for _ in 0..days {
            clock::advance_date(&mut w);
            crate::programs::begin_day(&mut w);
            tick_day(&mut w);
            crate::programs::finish_day(&mut w);
        }
        let finished = w.nation(USA).equipment.as_ref().unwrap().projects.iter().find(|p| p.id == project).unwrap();
        assert_eq!(finished.status, ProjectStatus::Complete);
        assert_eq!(finished.completed_units, 2);
        assert!(w.nation(USA).arsenal.held.iter().all(|h| h.design_id.as_deref() != Some(SOURCE)));
        assert_eq!((holding(&w, TARGET).units, holding(&w, TARGET).age), (2.0, 111.25));
        assert_eq!(w.nation(USA).arsenal.orders.iter().filter(|o| o.design_id.as_deref() == Some(SOURCE)).map(|o| o.units).sum::<f64>(), 5.0);
        validate_state(w.nation(USA)).unwrap();
    }

    #[test]
    fn retirement_preserves_receipts_and_funding_while_future_support_matches_survivors() {
        let (mut w, _) = fixture();
        let required_before = fleet_maintenance_requirement(w.nation(USA));
        w.nation_mut(USA).program_budget.as_mut().unwrap().spent_today_bn[BUDGET_DEFENSE][2] = required_before / 2.0;
        settle_maintenance(&mut w, USA);
        let before = w.nation(USA).equipment.as_ref().unwrap().clone();
        let budget = w.nation(USA).program_budget.clone();
        let combat_before = arsenal::combat_value(w.nation(USA), holding(&w, SOURCE));
        let q = retirement_quote(&w, USA, SOURCE, 4);
        crate::apply_command(&mut w, &command(SOURCE, 4)).unwrap();
        assert_eq!(w.nation(USA).equipment.as_ref().unwrap(), &before);
        assert_eq!(w.nation(USA).program_budget, budget);
        close(arsenal::combat_value(w.nation(USA), holding(&w, SOURCE)), combat_before / 2.0);
        assert_eq!(before.maintenance_fraction, 0.5);
        clock::advance_date(&mut w);
        crate::programs::begin_day(&mut w);
        let posted = w.nation(USA).program_budget.clone();
        settle_support(&mut w);
        assert_eq!(w.nation(USA).program_budget, posted, "support earmarking cannot post another charge");
        let supported = w.nation(USA).equipment.as_ref().unwrap();
        assert_eq!(supported.maintenance_required_today_bn, q.maintenance_bn_day_after);
        assert_eq!(supported.maintenance_fraction, 1.0);
        validate_state(w.nation(USA)).unwrap();
    }

    #[test]
    fn retiring_specialists_removes_their_realized_operational_role() {
        let (mut w, _) = fixture();
        certify(&mut w, "service-artillery", default_spec("ground_artillery"));
        arsenal::deliver_design(w.nation_mut(USA), "service-artillery", 20, 10.0).unwrap();
        let before = crate::operations::capabilities(w.nation(USA));
        assert!(before.ground_roles.fire_support > 0.0);
        let receipts = w.nation(USA).equipment.as_ref().unwrap().clone();
        crate::apply_command(&mut w, &command("service-artillery", 20)).unwrap();
        assert_eq!(crate::operations::capabilities(w.nation(USA)).ground_roles.fire_support, 0.0);
        assert_eq!(w.nation(USA).equipment.as_ref().unwrap(), &receipts);
        assert_eq!(holding(&w, SOURCE).units, 8.0);
    }

    #[test]
    fn retirement_save_load_continues_the_same_dated_refit_and_delivery_schedule() {
        let (mut uninterrupted, district) = fixture();
        start_refit(&mut uninterrupted, USA, SOURCE, TARGET, &district, 2, 1.0).unwrap();
        crate::apply_command(&mut uninterrupted, &command(SOURCE, 3)).unwrap();
        let saved = crate::save(&uninterrupted);
        let mut resumed = crate::load(&saved).unwrap();
        assert_eq!(crate::save(&resumed), saved);
        for day in 0..10 {
            let commands = if day == 2 { vec![command(SOURCE, 1)] } else { vec![] };
            assert_eq!(crate::tick_day(&mut uninterrupted, &commands), crate::tick_day(&mut resumed, &commands));
            assert_eq!(crate::save(&uninterrupted), crate::save(&resumed), "diverged on continuation day {day}");
        }
        assert_eq!(holding(&resumed, SOURCE).units, 9.0, "8 delivered - 4 retired + 5 pending delivered");
        assert_eq!(holding(&resumed, SOURCE).refit_reserved, 2);
        assert!(holding(&resumed, SOURCE).age < 111.25, "new deliveries correctly blend with old survivors");
        validate_state(resumed.nation(USA)).unwrap();
    }
}
