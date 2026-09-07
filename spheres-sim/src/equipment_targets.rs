/// Desired quantities are planning preferences, never orders or appropriations.
pub const MAX_FLEET_TARGET: u32 = 1_000_000;

pub fn validate_fleet_targets(n: &Nation) -> Result<(), String> {
    let Some(s) = &n.equipment else {
        return Ok(());
    };
    if !s.fleet_targets.is_empty() && s.version < 4 {
        return Err("Fleet targets require equipment save version four.".into());
    }
    if s.fleet_targets.len() > MAX_REVISIONS
        || s.fleet_targets
            .iter()
            .any(|(id, quantity)| *quantity > MAX_FLEET_TARGET || certified(n, id).is_err())
    {
        return Err(
            "A fleet target must name a certified revision and a supported whole-vehicle quantity."
                .into(),
        );
    }
    Ok(())
}

/// Called by EquipmentOrder::Target. An explicit zero is a goal of no vehicles;
/// None stops tracking. Both preserve every vehicle, order and financial entry.
pub fn set_fleet_target(
    w: &mut WorldState,
    id: NationId,
    revision: &str,
    quantity: Option<u32>,
) -> Result<(), String> {
    if let Some(reason) = actor_refusal(w, id) {
        return Err(reason);
    }
    if quantity.is_some_and(|value| value > MAX_FLEET_TARGET) {
        return Err(format!(
            "Choose a whole-vehicle target from zero to {MAX_FLEET_TARGET}."
        ));
    }
    let n = w.nation(id);
    certified(n, revision)?;
    validate_state(n)?;
    let s = w.nation_mut(id).equipment.as_mut().unwrap();
    if let Some(quantity) = quantity {
        s.fleet_targets.insert(revision.into(), quantity);
        s.version = VERSION;
    } else {
        s.fleet_targets.remove(revision);
    }
    Ok(())
}

#[derive(Clone, Debug, Serialize)]
pub struct FleetTargetPlan {
    pub revision: String,
    pub desired: Option<u32>,
    pub delivered: u64,
    pub available: u64,
    pub incoming: u64,
    pub production_remaining: u64,
    pub refit_incoming: u64,
    pub refit_outgoing: u64,
    /// Incoming production/refit units that still depend on funded work.
    pub conditional_incoming: u64,
    /// Subset of conditional incoming on paused, blocked or unfunded projects.
    pub stalled_incoming: u64,
    pub stalled_outgoing: u64,
    /// Delivered + transit + unfinished fabrication + refit in - refit out.
    pub projected: u64,
    pub shortfall: u64,
    pub excess: u64,
}

/// All certified revisions, including untracked ones so the library can offer
/// a target. Completed units are already in Arsenal stock/orders and are never
/// added from the project ledger a second time. Cancelled work contributes zero.
pub fn fleet_target_plans(n: &Nation) -> Vec<FleetTargetPlan> {
    let Some(s) = &n.equipment else {
        return vec![];
    };
    s.revisions
        .values()
        .filter(|r| r.certified_day.is_some())
        .map(|r| {
            let delivered = n
                .arsenal
                .held
                .iter()
                .filter(|h| h.design_id.as_deref() == Some(r.id.as_str()))
                .map(|h| h.units.max(0.0) as u64)
                .sum::<u64>();
            let available = n
                .arsenal
                .held
                .iter()
                .filter(|h| h.design_id.as_deref() == Some(r.id.as_str()))
                .map(|h| crate::arsenal::available_design_units(h) as u64)
                .sum();
            let incoming = n
                .arsenal
                .orders
                .iter()
                .filter(|o| o.design_id.as_deref() == Some(r.id.as_str()))
                .map(|o| o.units.max(0.0) as u64)
                .sum::<u64>();
            let mut production_remaining = 0;
            let mut refit_incoming = 0;
            let mut refit_outgoing = 0;
            let mut stalled_incoming = 0;
            let mut stalled_outgoing = 0;
            for p in s
                .projects
                .iter()
                .filter(|p| !matches!(p.status, ProjectStatus::Complete | ProjectStatus::Cancelled))
            {
                let remaining = p.quantity.saturating_sub(p.completed_units) as u64;
                let stalled = p.paused
                    || p.daily_budget_bn <= 0.0
                    || matches!(p.status, ProjectStatus::Paused | ProjectStatus::Blocked);
                if p.revision_id == r.id
                    && matches!(p.kind, ProjectKind::Production | ProjectKind::Refit)
                {
                    if p.kind == ProjectKind::Production {
                        production_remaining += remaining;
                    } else {
                        refit_incoming += remaining;
                    }
                    if stalled {
                        stalled_incoming += remaining;
                    }
                }
                if p.kind == ProjectKind::Refit
                    && p.source_design_id.as_deref() == Some(r.id.as_str())
                {
                    refit_outgoing += remaining;
                    if stalled {
                        stalled_outgoing += remaining;
                    }
                }
            }
            let desired = s.fleet_targets.get(&r.id).copied();
            let projected = delivered.saturating_sub(refit_outgoing)
                + incoming
                + production_remaining
                + refit_incoming;
            FleetTargetPlan {
                revision: r.id.clone(),
                desired,
                delivered,
                available,
                incoming,
                production_remaining,
                refit_incoming,
                refit_outgoing,
                conditional_incoming: production_remaining + refit_incoming,
                stalled_incoming,
                stalled_outgoing,
                projected,
                shortfall: desired.map_or(0, |goal| (goal as u64).saturating_sub(projected)),
                excess: desired.map_or(0, |goal| projected.saturating_sub(goal as u64)),
            }
        })
        .collect()
}

/// Publicly owned supplier deliveries count toward the goal; unsold company
/// stock and company work in progress do not. No second stock ledger is made.
pub fn fleet_target_plans_world(w:&WorldState,nation:NationId)->Vec<FleetTargetPlan> {
    let mut plans=fleet_target_plans(w.nation(nation));
    for p in &mut plans {
        let purchased=crate::companies::inbound_units(w,nation,&p.revision) as u64;
        p.incoming=p.incoming.saturating_add(purchased);
        p.projected=p.projected.saturating_add(purchased);
        let (refit_in,refit_out,blocked_in,blocked_out)=crate::companies::refit_target_flow(w,nation,&p.revision);
        p.refit_incoming=p.refit_incoming.saturating_add(refit_in);
        p.refit_outgoing=p.refit_outgoing.saturating_add(refit_out);
        p.conditional_incoming=p.conditional_incoming.saturating_add(refit_in);
        p.stalled_incoming=p.stalled_incoming.saturating_add(blocked_in);
        p.stalled_outgoing=p.stalled_outgoing.saturating_add(blocked_out);
        p.projected=p.projected.saturating_sub(refit_out).saturating_add(refit_in);
        p.shortfall=p.desired.map_or(0,|goal|(goal as u64).saturating_sub(p.projected));
        p.excess=p.desired.map_or(0,|goal|p.projected.saturating_sub(goal as u64));
    }
    plans
}

#[derive(Clone, Debug, Serialize)]
pub struct FleetRefitCandidate {
    pub source_revision: String,
    pub quantity: u32,
}

/// Conservative alternatives to new manufacture: use only physically present,
/// unreserved source vehicles above that source revision's own desired count.
/// Future deliveries cannot be withdrawn, and no suggestion cancels another plan.
pub fn fleet_target_refits(n: &Nation, revision: &str) -> Vec<FleetRefitCandidate> {
    let Ok(target) = certified(n, revision) else {
        return vec![];
    };
    let plans = fleet_target_plans(n);
    let Some(plan) = plans.iter().find(|p| p.revision == revision) else {
        return vec![];
    };
    if plan.shortfall == 0 {
        return vec![];
    }
    plans
        .iter()
        .filter_map(|source| {
            let count = source
                .available
                .saturating_sub(source.desired.unwrap_or(0) as u64)
                .min(plan.shortfall)
                .min(MAX_BATCH as u64) as u32;
            if count == 0 || refit_terms(certified(n, &source.revision).ok()?, target).is_err() {
                return None;
            }
            Some(FleetRefitCandidate {
                source_revision: source.revision.clone(),
                quantity: count,
            })
        })
        .collect()
}

pub fn fleet_target_refits_world(w:&WorldState,nation:NationId,revision:&str)->Vec<FleetRefitCandidate> {
    let need=fleet_target_plans_world(w,nation).into_iter().find(|p|p.revision==revision).map_or(0,|p|p.shortfall);
    fleet_target_refits(w.nation(nation),revision).into_iter().filter_map(|mut c|{c.quantity=(c.quantity as u64).min(need) as u32;(c.quantity>0).then_some(c)}).collect()
}

#[cfg(test)]
mod fleet_target_tests {
    use super::*;
    use crate::{arsenal, init::world_1990, world::GameRules, Command, EquipmentOrder};
    const USA: NationId = NationId::USA;
    const SOURCE: &str = "target-source";
    const DEST: &str = "target-upgrade";

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
        crate::programs::set_construction_budget(&mut w, USA, 0.001).unwrap();
        let district = w
            .districts
            .iter()
            .find(|(_, id)| **id == USA)
            .unwrap()
            .0
            .clone();
        for _ in 0..4 {
            crate::production::complete_capability(
                &mut w,
                &district,
                crate::production::ProjectKind::ArmsPlant,
            );
        }
        for commodity in crate::resources::ALL {
            crate::resources::set_stockpile_for_test(&mut w, USA, commodity, 1_000_000.0);
        }
        let day = clock::absolute_day(&w);
        for (key, upgraded) in [(SOURCE, false), (DEST, true)] {
            let mut spec = baseline_spec();
            if upgraded {
                spec.components
                    .insert("mobility".into(), "drive_mobile".into());
            }
            let profile = design_preview(&w, USA, &spec).profile.unwrap();
            state_mut(w.nation_mut(USA)).revisions.insert(
                key.into(),
                DesignRevision {
                    id: key.into(),
                    name: key.into(),
                    specification_key: specification_key(&spec),
                    spec,
                    profile,
                    created_day: day,
                    certified_day: Some(day),
                },
            );
        }
        state_mut(w.nation_mut(USA)).finance_from_day = day;
        arsenal::deliver_design(w.nation_mut(USA), SOURCE, 8, 27.5).unwrap();
        arsenal::queue_design_order(w.nation_mut(USA), SOURCE, 5, DELIVERY_DAYS, 0.0).unwrap();
        (w, district)
    }
    fn command(revision: &str, quantity: Option<u32>) -> Command {
        Command::Equipment {
            nation: USA,
            order: EquipmentOrder::Target {
                revision: revision.into(),
                quantity,
            },
        }
    }
    fn plan(w: &WorldState, revision: &str) -> FleetTargetPlan {
        fleet_target_plans(w.nation(USA))
            .into_iter()
            .find(|p| p.revision == revision)
            .unwrap()
    }
    fn next_work_day(w: &mut WorldState) {
        let (year, month, day) = clock::date_from_day(clock::absolute_day(w) + 1);
        w.year = year;
        w.month = month;
        w.day = day;
        crate::programs::begin_day(w);
        tick_day(w);
        crate::programs::finish_day(w);
    }

    #[test]
    fn fleet_target_setting_changes_only_preference_and_roundtrips() {
        let (mut w, _) = fixture();
        w.nation_mut(USA).equipment.as_mut().unwrap().version = 3;
        let before = crate::save(&w);
        assert!(!before.contains("fleet_targets"));
        let mut expected = w.clone();
        let state = expected.nation_mut(USA).equipment.as_mut().unwrap();
        state.version = VERSION;
        state.fleet_targets.insert(DEST.into(), 25);
        crate::apply_command(&mut w, &command(DEST, Some(25))).unwrap();
        assert_eq!(crate::save(&w), crate::save(&expected));
        let restored = crate::load(&crate::save(&w)).unwrap();
        assert_eq!(crate::save(&w), crate::save(&restored));
        assert_eq!(plan(&restored, DEST).shortfall, 25);
        crate::apply_command(&mut w, &command(DEST, Some(0))).unwrap();
        assert_eq!(plan(&w, DEST).desired, Some(0));
        crate::apply_command(&mut w, &command(DEST, None)).unwrap();
        assert_eq!(plan(&w, DEST).desired, None);
        assert!(!crate::save(&w).contains("fleet_targets"));
    }

    #[test]
    fn fleet_target_refusals_and_invalid_saves_are_atomic() {
        let (mut w, _) = fixture();
        for (revision, quantity) in [("missing", Some(1)), (DEST, Some(MAX_FLEET_TARGET + 1))] {
            let saved = crate::save(&w);
            assert!(crate::apply_command(&mut w, &command(revision, quantity)).is_err());
            assert_eq!(crate::save(&w), saved);
        }
        w.nation_mut(USA)
            .equipment
            .as_mut()
            .unwrap()
            .revisions
            .get_mut(DEST)
            .unwrap()
            .certified_day = None;
        assert!(crate::apply_command(&mut w, &command(DEST, Some(2))).is_err());
        let mut corrupt = fixture().0;
        corrupt
            .nation_mut(USA)
            .equipment
            .as_mut()
            .unwrap()
            .fleet_targets
            .insert("missing".into(), 1);
        assert!(crate::load(&crate::save(&corrupt)).is_err());
        let s = corrupt.nation_mut(USA).equipment.as_mut().unwrap();
        s.fleet_targets.clear();
        s.fleet_targets.insert(DEST.into(), 1);
        s.version = 3;
        assert!(crate::load(&crate::save(&corrupt)).is_err());
    }

    #[test]
    fn fleet_target_refit_transfers_conserve_counts_and_cancellation_releases_source() {
        let (mut w, district) = fixture();
        set_fleet_target(&mut w, USA, SOURCE, Some(13)).unwrap();
        set_fleet_target(&mut w, USA, DEST, Some(7)).unwrap();
        let project = start_refit(&mut w, USA, SOURCE, DEST, &district, 3, 0.001).unwrap();
        let before = crate::save(&w);
        let source = plan(&w, SOURCE);
        let dest = plan(&w, DEST);
        assert_eq!(
            (
                source.delivered,
                source.available,
                source.incoming,
                source.refit_outgoing,
                source.projected,
                source.shortfall
            ),
            (8, 5, 5, 3, 10, 3)
        );
        assert_eq!(
            (dest.refit_incoming, dest.projected, dest.shortfall),
            (3, 3, 4)
        );
        assert_eq!(
            source.projected + dest.projected,
            13,
            "refits move stock rather than cloning it"
        );
        assert_eq!(crate::save(&w), before, "planning reads are pure");
        set_project_paused(&mut w, USA, project, true).unwrap();
        assert_eq!(plan(&w, DEST).stalled_incoming, 3);
        assert_eq!(plan(&w, SOURCE).stalled_outgoing, 3);
        assert_eq!(plan(&w, DEST).conditional_incoming, 3);
        cancel_project(&mut w, USA, project).unwrap();
        assert_eq!(
            (plan(&w, SOURCE).projected, plan(&w, DEST).projected),
            (13, 0)
        );
        assert_eq!(plan(&w, SOURCE).available, 8);
    }

    #[test]
    fn fleet_target_completion_counts_orders_once_and_cancelled_work_not_at_all() {
        let (mut w, district) = fixture();
        set_fleet_target(&mut w, USA, DEST, Some(10)).unwrap();
        let profile = profile(w.nation(USA), DEST).unwrap().clone();
        let project = start_production(&mut w, USA, DEST, &district, 4, 1.0).unwrap();
        for _ in 0..profile.tooling_days + profile.production_days * 2 {
            next_work_day(&mut w);
        }
        validate_state(w.nation(USA)).unwrap();
        let p = plan(&w, DEST);
        assert_eq!(
            (p.incoming, p.production_remaining, p.projected, p.shortfall),
            (2, 2, 4, 6)
        );
        let mut restored = crate::load(&crate::save(&w)).unwrap();
        next_work_day(&mut w);
        next_work_day(&mut restored);
        assert_eq!(
            crate::save(&w),
            crate::save(&restored),
            "saved targets preserve work and financial continuity"
        );
        cancel_project(&mut w, USA, project).unwrap();
        validate_state(w.nation(USA)).unwrap();
        let p = plan(&w, DEST);
        assert_eq!(
            (p.incoming, p.production_remaining, p.projected, p.shortfall),
            (2, 0, 2, 8)
        );
    }

    #[test]
    fn fleet_target_partial_refit_preserves_projection_and_cancel_keeps_completed_conversion() {
        let (mut w, district) = fixture();
        set_fleet_target(&mut w, USA, DEST, Some(5)).unwrap();
        let quote = refit_quote(&w, USA, SOURCE, DEST, &district, 3, 1.0);
        let project = start_refit(&mut w, USA, SOURCE, DEST, &district, 3, 1.0).unwrap();
        for _ in 0..quote.minimum_days / 3 {
            next_work_day(&mut w);
        }
        validate_state(w.nation(USA)).unwrap();
        assert_eq!(
            (plan(&w, SOURCE).delivered, plan(&w, SOURCE).refit_outgoing),
            (7, 2)
        );
        assert_eq!(
            (plan(&w, DEST).delivered, plan(&w, DEST).refit_incoming),
            (1, 2)
        );
        assert_eq!(plan(&w, SOURCE).projected + plan(&w, DEST).projected, 13);
        cancel_project(&mut w, USA, project).unwrap();
        assert_eq!(
            (plan(&w, SOURCE).projected, plan(&w, DEST).projected),
            (12, 1)
        );
        assert_eq!(
            (plan(&w, SOURCE).available, plan(&w, DEST).shortfall),
            (7, 4)
        );
        validate_state(w.nation(USA)).unwrap();
    }

    #[test]
    fn fleet_target_refit_suggestions_protect_source_goals_and_use_only_delivered_stock() {
        let (mut w, _) = fixture();
        set_fleet_target(&mut w, USA, DEST, Some(20)).unwrap();
        let candidates = fleet_target_refits(w.nation(USA), DEST);
        assert_eq!(
            (
                candidates[0].source_revision.as_str(),
                candidates[0].quantity
            ),
            (SOURCE, 8)
        );
        set_fleet_target(&mut w, USA, SOURCE, Some(6)).unwrap();
        assert_eq!(fleet_target_refits(w.nation(USA), DEST)[0].quantity, 2);
        set_fleet_target(&mut w, USA, SOURCE, Some(13)).unwrap();
        assert!(
            fleet_target_refits(w.nation(USA), DEST).is_empty(),
            "pending deliveries cannot be withdrawn for refit"
        );
    }
}
