// Reserve plans are preferences and optional standing scheduling authority.
// They create only ordinary finite batches, never money, materials or rounds.
pub const MAX_AMMO_RESERVE_TARGET: u32 = 10_000_000;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AmmoReservePlan {
    pub target_rounds: u32,
    pub district: String,
    pub daily_limit_bn: f64,
    #[serde(default)]
    pub automatic: bool,
    pub authorized_day: i32,
    pub last_review: Option<AmmoReserveReceipt>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AmmoReserveReceipt {
    pub day: i32,
    pub stock: f64,
    pub committed: u64,
    pub gap: u32,
    pub order_id: Option<u32>,
    pub ordered_rounds: u32,
    pub reason: String,
}
#[derive(Clone, Debug, Serialize)]
pub struct AmmoReserveStatus {
    pub family: String,
    pub target_rounds: Option<u32>,
    pub stock: f64,
    pub committed: u64,
    pub supplier_inbound: u64,
    pub paused_committed: u64,
    pub projected: f64,
    pub gap: u32,
    pub next_quantity: u32,
    pub reason: String,
    pub can_schedule: bool,
    pub batch: Option<AmmoOrderQuote>,
}
#[derive(Clone, Debug, Serialize)]
pub struct AmmoReserveQuote {
    pub valid: bool,
    pub reason: Option<String>,
    pub family: String,
    pub target_rounds: u32,
    pub district: String,
    pub daily_limit_bn: f64,
    pub automatic: bool,
    pub eligible_from_day: i32,
    pub status: AmmoReserveStatus,
}

fn reserve_gap(target: u32, stock: f64, committed: u64) -> u32 {
    (target as f64 - stock - committed as f64)
        .max(0.0)
        .ceil()
        .min(target as f64) as u32
}
fn reserve_configuration_refusal(
    w: &WorldState,
    id: NationId,
    family: &str,
    target: u32,
    district: &str,
    limit: f64,
    automatic: bool,
) -> Option<String> {
    actor_refusal(w, id)
        .or_else(|| budget_refusal(limit))
        .or_else(|| {
            (target > MAX_AMMO_RESERVE_TARGET).then(|| {
                format!("Choose a reserve target from zero to {MAX_AMMO_RESERVE_TARGET} rounds.")
            })
        })
        .or_else(|| {
            ammo_def(family)
                .is_none()
                .then(|| "Choose a supported ammunition family.".into())
        })
        .or_else(|| {
            (w.districts.get(district) != Some(&id)).then(|| {
                "Choose a province owned by this government for future ammunition batches.".into()
            })
        })
        .or_else(|| {
            (!w.nation(id).equipment.as_ref().is_some_and(|s| {
                s.revisions.values().any(|r| {
                    r.certified_day.is_some() && ammunition_family(&r.spec) == Some(family)
                })
            }))
            .then(|| {
                "Certify a compatible weapon configuration before setting this ammunition reserve."
                    .into()
            })
        })
        .or_else(|| {
            if automatic {
                ammo_actor_refusal(w, id)
            } else {
                None
            }
        })
        .or_else(|| validate_state(w.nation(id)).err())
}
fn reserve_status_for(
    w: &WorldState,
    id: NationId,
    family: &str,
    plan: Option<&AmmoReservePlan>,
) -> AmmoReserveStatus {
    let ammunition = w
        .nation_opt(id)
        .and_then(|n| n.equipment.as_ref())
        .and_then(|s| s.ammunition.as_ref());
    let stock = ammunition
        .and_then(|a| a.stocks.get(family))
        .copied()
        .unwrap_or(0.0);
    let mut committed = 0u64;
    let mut paused = 0u64;
    let mut unfinished = 0usize;
    if let Some(a) = ammunition {
        for p in a
            .orders
            .iter()
            .filter(|p| p.family == family && !ammunition_ended(p))
        {
            let remaining = p.quantity.saturating_sub(p.completed_rounds) as u64;
            committed += remaining;
            unfinished += 1;
            if p.paused || p.status == ProjectStatus::Paused {
                paused += remaining;
            }
        }
    }
    let supplier_inbound = crate::companies::ammo_inbound_units(w, id, family);
    committed += supplier_inbound;
    let gap = plan.map_or(0, |p| reserve_gap(p.target_rounds, stock, committed));
    let mut out = AmmoReserveStatus {
        family: family.into(),
        target_rounds: plan.map(|p| p.target_rounds),
        stock,
        committed,
        supplier_inbound,
        paused_committed: paused,
        projected: stock + committed as f64,
        gap,
        next_quantity: gap.min(MAX_AMMO_ORDER),
        reason: "No reserve plan is tracked for this family.".into(),
        can_schedule: false,
        batch: None,
    };
    let Some(plan) = plan else {
        return out;
    };
    if crate::companies::ammo_supplier_active(w, id, family) {
        out.reason = "This family uses reviewed supplier stock purchases. The reserve target remains tracked, and purchased inbound stock and previously commissioned public batches count toward it. New automatic public fabrication is disabled for this family.".into();
        return out;
    }
    if gap > 0 {
        out.batch = Some(ammo_order_quote(
            w,
            id,
            family,
            &plan.district,
            out.next_quantity,
            plan.daily_limit_bn,
        ));
    }
    let day = clock::absolute_day(w);
    out.reason = if !plan.automatic {
        "Manual reserve target. Existing batch commitments are conditional; start additional batches through the ordinary production review.".into()
    } else if gap == 0 {
        "Physical stock and conditional unfinished batches cover this target; no new batch is needed.".into()
    } else if unfinished > 0 {
        "Waiting for existing batch work. Automatic scheduling keeps at most one unfinished batch per family, including paused or blocked work.".into()
    } else if day <= plan.authorized_day {
        "Automatic review starts on the next date after authorization. Saving this plan creates no batch or rounds.".into()
    } else if plan.last_review.as_ref().is_some_and(|r| r.day >= day) {
        "This reserve has already been reviewed today. Changed stock, funds or factory capacity will be reviewed on the next date.".into()
    } else if let Some(reason) = ammo_actor_refusal(w, id) {
        reason
    } else if !maintenance_plan_on(w.nation(id), day) {
        "Waiting for the actual maintenance plan to take effect.".into()
    } else if plan.daily_limit_bn <= 0.0 {
        "The new-batch daily funding ceiling is zero.".into()
    } else if w
        .nation(id)
        .program_budget
        .as_ref()
        .is_none_or(|p| p.day != Some(day) || p.settled_day == Some(day))
    {
        "Waiting for an open maintenance and supply funding date.".into()
    } else if crate::programs::available_bn(w, id, crate::world::BUDGET_DEFENSE, 2) <= 0.0 {
        "No maintenance and supply authority remains after existing fleet servicing and ammunition work.".into()
    } else if let Some(reason) = out.batch.as_ref().and_then(|q| q.reason.clone()) {
        reason
    } else {
        out.can_schedule = true;
        "The next automatic review can schedule one finite batch. Production still needs its existing funding and physical raw inputs; no material purchase is authorized.".into()
    };
    out
}
pub fn ammo_reserve_status(w: &WorldState, id: NationId, family: &str) -> AmmoReserveStatus {
    let plan = w
        .nation_opt(id)
        .and_then(|n| n.equipment.as_ref())
        .and_then(|s| s.ammunition_reserves.get(family));
    reserve_status_for(w, id, family, plan)
}
pub fn ammo_reserve_quote(
    w: &WorldState,
    id: NationId,
    family: &str,
    target: u32,
    district: &str,
    limit: f64,
    automatic: bool,
) -> AmmoReserveQuote {
    let reason = reserve_configuration_refusal(w, id, family, target, district, limit, automatic);
    let day = clock::absolute_day(w);
    let proposed = AmmoReservePlan {
        target_rounds: target,
        district: district.into(),
        daily_limit_bn: limit,
        automatic,
        authorized_day: day,
        last_review: None,
    };
    AmmoReserveQuote {
        valid: reason.is_none(),
        reason,
        family: family.into(),
        target_rounds: target,
        district: district.into(),
        daily_limit_bn: limit,
        automatic,
        eligible_from_day: day.saturating_add(1),
        status: reserve_status_for(w, id, family, Some(&proposed)),
    }
}
pub fn set_ammo_reserve(
    w: &mut WorldState,
    id: NationId,
    family: &str,
    target: u32,
    district: &str,
    limit: f64,
    automatic: bool,
) -> Result<(), String> {
    let q = ammo_reserve_quote(w, id, family, target, district, limit, automatic);
    if let Some(reason) = q.reason {
        return Err(reason);
    }
    let day = clock::absolute_day(w);
    state_mut(w.nation_mut(id)).ammunition_reserves.insert(
        family.into(),
        AmmoReservePlan {
            target_rounds: target,
            district: district.into(),
            daily_limit_bn: limit,
            automatic,
            authorized_day: day,
            last_review: None,
        },
    );
    Ok(())
}
pub fn clear_ammo_reserve(w: &mut WorldState, id: NationId, family: &str) -> Result<(), String> {
    if let Some(reason) = actor_refusal(w, id) {
        return Err(reason);
    }
    validate_state(w.nation(id))?;
    if !w
        .nation(id)
        .equipment
        .as_ref()
        .is_some_and(|s| s.ammunition_reserves.contains_key(family))
    {
        return Err("This ammunition family has no reserve plan to clear.".into());
    }
    state_mut(w.nation_mut(id))
        .ammunition_reserves
        .remove(family);
    Ok(())
}

/// Runs after existing maintenance and fabrication, before the war snapshot.
/// Repeated calls on one date cannot order twice, even when work is blocked.
fn tick_ammunition_reserves(w: &mut WorldState, id: NationId) {
    let day = clock::absolute_day(w);
    let n = w.nation(id);
    if n.equipment
        .as_ref()
        .is_none_or(|s| s.ammunition_reserves.is_empty())
    {
        return;
    }
    if !clock::is_daily(w) || !w.rules.military_operations || validate_state(n).is_err() {
        return;
    }
    let Some(s) = &n.equipment else {
        return;
    };
    if n.program_budget
        .as_ref()
        .is_none_or(|p| p.day != Some(day) || p.settled_day == Some(day))
    {
        return;
    }
    let families: Vec<_> = s
        .ammunition_reserves
        .iter()
        .filter(|(_, p)| {
            p.automatic
                && p.authorized_day < day
                && p.last_review.as_ref().is_none_or(|r| r.day < day)
        })
        .map(|(f, _)| f.clone())
        .collect();
    for family in families {
        let status = ammo_reserve_status(w, id, &family);
        let plan = w.nation(id).equipment.as_ref().unwrap().ammunition_reserves[&family].clone();
        let mut receipt = AmmoReserveReceipt {
            day,
            stock: status.stock,
            committed: status.committed,
            gap: status.gap,
            order_id: None,
            ordered_rounds: 0,
            reason: status.reason,
        };
        if status.can_schedule {
            match start_ammo_order(
                w,
                id,
                &family,
                &plan.district,
                status.next_quantity,
                plan.daily_limit_bn,
            ) {
                Ok(order) => {
                    // Existing fabrication was already reviewed before this
                    // scheduler. A first batch creates its sparse ammo state
                    // afterward; preserve that same-date work guard as well.
                    let ammo = state_mut(w.nation_mut(id)).ammunition.as_mut().unwrap();
                    if ammo.last_work_day.is_none() {
                        ammo.last_work_day = Some(day);
                    }
                    receipt.order_id = Some(order);
                    receipt.ordered_rounds = status.next_quantity;
                    receipt.reason=format!("Scheduled batch #{order}: {} rounds. Existing fabrication rules will pay for work from the next eligible date; no rounds or materials were granted.",status.next_quantity);
                }
                Err(reason) => receipt.reason = reason,
            }
        }
        state_mut(w.nation_mut(id))
            .ammunition_reserves
            .get_mut(&family)
            .unwrap()
            .last_review = Some(receipt);
    }
}

#[cfg(test)]
mod ammunition_reserve_tests {
    use super::*;
    use crate::{
        programs,
        resources::{self, Commodity},
        world::{GameRules, BUDGET_DEFENSE as D},
    };
    const USA: NationId = NationId::USA;
    fn fixture() -> (WorldState, String) {
        let mut w = crate::init::world_1990(GameRules {
            daily_simulation: true,
            military_operations: true,
            production_system: true,
            manufacturing_system: true,
            resource_market: true,
            resource_gates: true,
            ..Default::default()
        });
        w.player = Some(USA);
        let day = clock::absolute_day(&w);
        let spec = default_spec("ground_apc");
        let profile = design_preview(&w, USA, &spec).profile.unwrap();
        state_mut(w.nation_mut(USA)).revisions.insert(
            "reserve-apc".into(),
            DesignRevision {
                id: "reserve-apc".into(),
                name: "Reserve APC".into(),
                specification_key: specification_key(&spec),
                spec,
                profile,
                created_day: day,
                certified_day: Some(day),
            },
        );
        let district = w
            .districts
            .iter()
            .find(|(_, n)| **n == USA)
            .unwrap()
            .0
            .clone();
        for _ in 0..3 {
            crate::production::complete_capability(
                &mut w,
                &district,
                crate::production::ProjectKind::ArmsPlant,
            );
        }
        for c in [Commodity::Iron, Commodity::Copper, Commodity::Coal] {
            resources::set_stockpile_for_test(&mut w, USA, c, 100.0);
        }
        (w, district)
    }
    fn maintenance(w: &mut WorldState) {
        programs::set_construction_budget(w, USA, 0.0).unwrap();
        set_maintenance_plan(w, USA, 0.0).unwrap();
    }
    fn open_next(w: &mut WorldState) {
        clock::advance_date(w);
        programs::begin_day(w);
    }
    fn work_next(w: &mut WorldState) {
        open_next(w);
        settle_support(w);
        programs::finish_day(w);
    }
    fn plan(w: &WorldState) -> &AmmoReservePlan {
        &w.nation(USA)
            .equipment
            .as_ref()
            .unwrap()
            .ammunition_reserves["mg_127"]
    }
    fn orders(w: &WorldState) -> &[AmmoOrder] {
        w.nation(USA)
            .equipment
            .as_ref()
            .unwrap()
            .ammunition
            .as_ref()
            .map_or(&[], |a| &a.orders)
    }
    #[test]
    fn manual_reserve_review_and_save_need_no_maintenance_and_grant_nothing() {
        let (mut w, district) = fixture();
        let before = crate::save(&w);
        let q = ammo_reserve_quote(&w, USA, "mg_127", 123, &district, 0.1, false);
        assert!(q.valid, "{:?}", q.reason);
        assert_eq!(q.status.gap, 123);
        assert_eq!(crate::save(&w), before);
        assert!(
            ammo_reserve_quote(&w, USA, "mg_127", 123, &district, 0.1, true)
                .reason
                .is_some()
        );
        set_ammo_reserve(&mut w, USA, "mg_127", 123, &district, 0.1, false).unwrap();
        assert!(w.nation(USA).program_budget.is_none());
        assert!(w
            .nation(USA)
            .equipment
            .as_ref()
            .unwrap()
            .ammunition
            .is_none());
        assert!(!plan(&w).automatic);
        assert!(plan(&w).last_review.is_none());
        let saved = crate::save(&w);
        assert_eq!(crate::save(&crate::load(&saved).unwrap()), saved);
        for _ in 0..3 {
            clock::advance_date(&mut w);
            settle_support(&mut w);
        }
        assert!(orders(&w).is_empty());
        assert!(plan(&w).last_review.is_none());
        clear_ammo_reserve(&mut w, USA, "mg_127").unwrap();
        assert!(
            !serde_json::to_string(w.nation(USA).equipment.as_ref().unwrap())
                .unwrap()
                .contains("ammunition_reserves")
        );
    }
    #[test]
    fn first_automatic_review_is_prospective_and_repeated_support_is_byte_identical() {
        let (mut w, district) = fixture();
        maintenance(&mut w);
        set_ammo_reserve(&mut w, USA, "mg_127", 100, &district, 0.1, true).unwrap();
        programs::begin_day(&mut w);
        settle_support(&mut w);
        assert!(orders(&w).is_empty());
        assert!(plan(&w).last_review.is_none());
        open_next(&mut w);
        let budget = w.nation(USA).program_budget.clone();
        let stocks: [f64; 12] =
            std::array::from_fn(|i| resources::stockpile(&w, USA, resources::ALL[i]));
        settle_support(&mut w);
        assert_eq!(orders(&w).len(), 1);
        assert_eq!(orders(&w)[0].completed_rounds, 0);
        assert_eq!(orders(&w)[0].spent_bn, 0.0);
        assert_eq!(w.nation(USA).program_budget, budget);
        assert_eq!(plan(&w).last_review.as_ref().unwrap().ordered_rounds, 100);
        assert_eq!(orders(&w)[0].started_day, clock::absolute_day(&w));
        for i in 0..12 {
            assert_eq!(resources::stockpile(&w, USA, resources::ALL[i]), stocks[i]);
        }
        let reviewed = crate::save(&w);
        settle_support(&mut w);
        assert_eq!(crate::save(&w), reviewed);
        work_next(&mut w);
        assert_eq!(orders(&w)[0].completed_rounds, 100);
        assert!(orders(&w)[0].spent_bn > 0.0);
        assert_eq!(orders(&w).len(), 1);
    }
    #[test]
    fn reserve_gap_nets_stock_and_all_unfinished_commitments_including_paused_work() {
        let (mut w, district) = fixture();
        maintenance(&mut w);
        let unit = ammo_def("mg_127").unwrap().fabrication_bn;
        let order = start_ammo_order(&mut w, USA, "mg_127", &district, 10, unit * 1.5).unwrap();
        work_next(&mut w);
        assert_eq!(orders(&w)[0].completed_rounds, 1);
        pause_ammo_order(&mut w, USA, order, true).unwrap();
        set_ammo_reserve(&mut w, USA, "mg_127", 20, &district, 0.2, true).unwrap();
        let status = ammo_reserve_status(&w, USA, "mg_127");
        assert_eq!(status.stock, 1.0);
        assert_eq!(status.committed, 9);
        assert_eq!(status.paused_committed, 9);
        assert_eq!(status.projected, 10.0);
        assert_eq!(status.gap, 10);
        assert!(!status.can_schedule);
        assert!(status.reason.contains("existing batch"));
        work_next(&mut w);
        assert_eq!(orders(&w).len(), 1);
        cancel_ammo_order(&mut w, USA, order).unwrap();
        let before = crate::save(&w);
        settle_support(&mut w);
        assert_eq!(crate::save(&w), before);
        work_next(&mut w);
        assert_eq!(orders(&w).len(), 2);
        assert_eq!(orders(&w)[1].quantity, 19);
        assert_eq!(orders(&w)[1].daily_limit_bn, 0.2);
    }
    #[test]
    fn target_and_batch_are_bounded_and_a_large_plan_never_stacks_new_batch_ceilings() {
        let (mut w, district) = fixture();
        maintenance(&mut w);
        let before = crate::save(&w);
        assert!(set_ammo_reserve(
            &mut w,
            USA,
            "mg_127",
            MAX_AMMO_RESERVE_TARGET + 1,
            &district,
            0.1,
            true
        )
        .is_err());
        assert_eq!(crate::save(&w), before);
        set_ammo_reserve(
            &mut w,
            USA,
            "mg_127",
            MAX_AMMO_RESERVE_TARGET,
            &district,
            0.1,
            true,
        )
        .unwrap();
        work_next(&mut w);
        assert_eq!(orders(&w).len(), 1);
        assert_eq!(orders(&w)[0].quantity, MAX_AMMO_ORDER);
        for _ in 0..3 {
            work_next(&mut w);
        }
        assert_eq!(orders(&w).len(), 1);
        assert!(ammo_reserve_status(&w, USA, "mg_127").gap > 0);
        let frozen = orders(&w)[0].clone();
        set_ammo_reserve(&mut w, USA, "mg_127", 0, &district, 0.0, false).unwrap();
        assert_eq!(orders(&w)[0], frozen);
        clear_ammo_reserve(&mut w, USA, "mg_127").unwrap();
        assert_eq!(orders(&w)[0], frozen);
    }
    #[test]
    fn expired_authority_zero_cap_and_missing_factory_wait_without_granting_or_importing() {
        for blocker in 0..3 {
            let (mut w, district) = fixture();
            maintenance(&mut w);
            set_ammo_reserve(
                &mut w,
                USA,
                "mg_127",
                100,
                &district,
                if blocker == 0 { 0.0 } else { 0.1 },
                true,
            )
            .unwrap();
            if blocker == 1 {
                w.year += 1;
                w.month = 1;
                w.day = 1;
                programs::begin_day(&mut w);
            } else {
                open_next(&mut w);
            }
            if blocker == 2 {
                w.production
                    .provinces
                    .iter_mut()
                    .find(|p| p.district == district)
                    .unwrap()
                    .arms_plants = 0;
            }
            let market = serde_json::to_string(&w.resources).unwrap();
            settle_support(&mut w);
            assert!(orders(&w).is_empty());
            assert_eq!(serde_json::to_string(&w.resources).unwrap(), market);
            let receipt = plan(&w).last_review.as_ref().unwrap();
            assert_eq!(receipt.ordered_rounds, 0);
            assert!(receipt.order_id.is_none());
            let before = crate::save(&w);
            settle_support(&mut w);
            assert_eq!(crate::save(&w), before);
        }
    }
    #[test]
    fn maintenance_and_existing_paid_work_own_authority_before_any_new_reserve_batch() {
        let (mut w, district) = fixture();
        maintenance(&mut w);
        set_maintenance_plan(&mut w, USA, 1.0).unwrap();
        set_ammo_reserve(&mut w, USA, "mg_127", 100, &district, 0.1, true).unwrap();
        open_next(&mut w);
        let b = w.nation_mut(USA).program_budget.as_mut().unwrap();
        b.available_bn[D][2] = 1e-9;
        b.prepaid_bn[D][2] = 0.0;
        settle_support(&mut w);
        assert!(orders(&w).is_empty());
        assert_eq!(
            w.nation(USA).program_budget.as_ref().unwrap().available_bn[D][2],
            0.0
        );
        assert!(plan(&w)
            .last_review
            .as_ref()
            .unwrap()
            .reason
            .contains("authority"));
    }
    #[test]
    fn unscheduled_targets_are_not_raw_purchase_demand_but_a_real_batch_exposes_missing_inputs() {
        let (mut w, district) = fixture();
        maintenance(&mut w);
        resources::set_stockpile_for_test(&mut w, USA, Commodity::Copper, 0.0);
        set_ammo_reserve(&mut w, USA, "mg_127", 100, &district, 0.1, false).unwrap();
        assert_eq!(ammunition_supply_demand(&w, USA).remaining, [0.0; 12]);
        set_ammo_reserve(&mut w, USA, "mg_127", 100, &district, 0.1, true).unwrap();
        work_next(&mut w);
        assert_eq!(orders(&w).len(), 1);
        assert!(ammunition_supply_demand(&w, USA).remaining[Commodity::Copper.idx()] > 0.0);
        work_next(&mut w);
        assert_eq!(orders(&w)[0].completed_rounds, 0);
        assert_eq!(orders(&w)[0].spent_bn, 0.0);
        assert_eq!(orders(&w).len(), 1);
    }
    #[test]
    fn receipt_roundtrip_preserves_continuation_and_rejects_forged_dates_or_orders() {
        let (mut w, district) = fixture();
        maintenance(&mut w);
        set_ammo_reserve(&mut w, USA, "mg_127", 100, &district, 1e-8, true).unwrap();
        work_next(&mut w);
        let saved = crate::save(&w);
        let mut loaded = crate::load(&saved).unwrap();
        assert_eq!(crate::save(&loaded), saved);
        for _ in 0..3 {
            work_next(&mut w);
            work_next(&mut loaded);
        }
        assert_eq!(crate::save(&w), crate::save(&loaded));
        for flaw in 0..5 {
            let mut bad = w.clone();
            let p = state_mut(bad.nation_mut(USA))
                .ammunition_reserves
                .get_mut("mg_127")
                .unwrap();
            let r = p.last_review.as_mut().unwrap();
            match flaw {
                0 => r.day += 10,
                1 => r.gap += 1,
                2 => {
                    r.order_id = Some(u32::MAX);
                    r.ordered_rounds = 1;
                }
                3 => p.automatic = false,
                _ => p.authorized_day = r.day,
            }
            assert!(validate_state(bad.nation(USA)).is_err(), "forgery {flaw}");
            assert!(crate::load(&crate::save(&bad)).is_err());
        }
        let (mut old, _) = fixture();
        old.nation_mut(USA).equipment.as_mut().unwrap().version = 5;
        let old_saved = crate::save(&old);
        assert_eq!(crate::save(&crate::load(&old_saved).unwrap()), old_saved);
    }
}
pub fn validate_ammunition_reserves(n: &Nation) -> Result<(), String> {
    let Some(s) = &n.equipment else {
        return Ok(());
    };
    if s.ammunition_reserves.is_empty() {
        return Ok(());
    }
    let invalid = || "Invalid ammunition reserve plan or dated scheduling receipt.".to_string();
    if s.version < 6 || s.ammunition_reserves.len() > ammo_catalog().len() {
        return Err(invalid());
    }
    let fiscal_day = n.program_budget.as_ref().and_then(|b| b.day);
    for (family, p) in &s.ammunition_reserves {
        if ammo_def(family).is_none()
            || p.target_rounds > MAX_AMMO_RESERVE_TARGET
            || p.district.is_empty()
            || budget_refusal(p.daily_limit_bn).is_some()
        {
            return Err(invalid());
        }
        let earliest = s
            .revisions
            .values()
            .filter(|r| ammunition_family(&r.spec) == Some(family.as_str()))
            .filter_map(|r| r.certified_day)
            .min()
            .ok_or_else(invalid)?;
        if p.authorized_day < earliest
            || fiscal_day.is_some_and(|d| p.authorized_day > d.saturating_add(1))
        {
            return Err(invalid());
        }
        if p.automatic {
            let maintenance = s.maintenance_plan.as_ref().ok_or_else(invalid)?;
            if fiscal_day.is_none() && p.authorized_day > maintenance.from_day.saturating_sub(1) {
                return Err(invalid());
            }
        }
        if let Some(r) = &p.last_review {
            if !p.automatic
                || r.day <= p.authorized_day
                || fiscal_day.is_none_or(|d| r.day > d)
                || !r.stock.is_finite()
                || r.stock < 0.0
                || r.committed
                    > MAX_AMMO_ORDER as u64
                        * (MAX_AMMO_ORDERS as u64 + crate::companies::MAX_AMMO_DELIVERIES as u64)
                || r.gap != reserve_gap(p.target_rounds, r.stock, r.committed)
                || r.ordered_rounds > r.gap.min(MAX_AMMO_ORDER)
                || (r.ordered_rounds > 0) != r.order_id.is_some()
            {
                return Err(invalid());
            }
            if let Some(id) = r.order_id {
                let order = s
                    .ammunition
                    .as_ref()
                    .and_then(|a| a.orders.iter().find(|o| o.id == id))
                    .ok_or_else(invalid)?;
                if order.family != *family
                    || order.district != p.district
                    || order.quantity != r.ordered_rounds
                    || order.started_day != r.day
                {
                    return Err(invalid());
                }
            }
        }
    }
    Ok(())
}
