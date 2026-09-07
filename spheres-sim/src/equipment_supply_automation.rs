// Optional standing permission to run the ordinary finite material purchase.
// This state records review settings/outcomes; all money, stock and cargo stay
// with their existing fiscal, Resources and Logistics owners.
pub const MAX_SUPPLY_POLICY_REVIEWS: usize = 64;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct EquipmentSupplyPolicy {
    #[serde(default)]
    pub automatic: bool,
    pub horizon_days: u32,
    pub spending_cap_bn: f64,
    pub cash_floor_bn: f64,
    pub review_interval_days: u32,
    pub authorized_day: i32,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct EquipmentSupplyReview {
    pub day: i32,
    pub authorized_day: i32,
    pub horizon_days: u32,
    pub spending_cap_bn: f64,
    pub cash_floor_bn: f64,
    pub review_interval_days: u32,
    pub cash_available_bn: f64,
    pub effective_cap_bn: f64,
    pub requested: [f64; 12],
    pub purchase: Option<crate::resources::RawPurchaseReceipt>,
    pub reason: String,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct EquipmentSupplyAutomation {
    pub policy: Option<EquipmentSupplyPolicy>,
    pub reviews: Vec<EquipmentSupplyReview>,
}
#[derive(Clone, Debug, Serialize)]
pub struct EquipmentSupplyPolicyStatus {
    pub automatic: bool,
    pub next_review_day: Option<i32>,
    pub can_review: bool,
    pub cash_available_bn: f64,
    pub effective_cap_bn: f64,
    pub reason: String,
    pub quote: Option<ReplenishmentQuote>,
}
#[derive(Clone, Debug, Serialize)]
pub struct EquipmentSupplyPolicyQuote {
    pub valid: bool,
    pub reason: Option<String>,
    pub automatic: bool,
    pub horizon_days: u32,
    pub spending_cap_bn: f64,
    pub cash_floor_bn: f64,
    pub review_interval_days: u32,
    pub eligible_from_day: i32,
    pub status: EquipmentSupplyPolicyStatus,
}

fn supply_policy_values_refusal(
    horizon: u32,
    cap: f64,
    floor: f64,
    interval: u32,
) -> Option<String> {
    if ![30, 90, 365].contains(&horizon) {
        Some("Choose a material planning horizon of 30, 90 or 365 days.".into())
    } else if ![1, 7, 30].contains(&interval) {
        Some("Review material purchases every 1, 7 or 30 simulation days.".into())
    } else if !cap.is_finite() || cap < 0.0 || !floor.is_finite() || floor < 0.0 {
        Some("The per-review cash limit and minimum cash reserve must be finite, non-negative amounts.".into())
    } else {
        None
    }
}
fn supply_policy_actor_refusal(w: &WorldState, id: NationId, automatic: bool) -> Option<String> {
    actor_refusal(w,id).or_else(||if automatic{
        if !(w.rules.resource_market&&w.rules.resource_gates){Some("Automatic material purchasing requires the physical Resources market.".into())}
        else if w.nation(id).treasury_bn.is_none()||w.nation(id).program_budget.is_none(){Some("Open departmental fiscal accounts before authorizing automatic material purchases.".into())}
        else{None}
    }else{None})
}
fn supply_policy_due(policy: &EquipmentSupplyPolicy, reviews: &[EquipmentSupplyReview]) -> i32 {
    policy
        .authorized_day
        .saturating_add(1)
        .max(reviews.last().map_or(i32::MIN, |r| {
            r.day.saturating_add(policy.review_interval_days as i32)
        }))
}
fn supply_policy_effective_cap(cash: f64, cap: f64, floor: f64) -> f64 {
    cap.min((cash - floor).max(0.0))
}
fn supply_policy_status_for(
    w: &WorldState,
    id: NationId,
    policy: Option<&EquipmentSupplyPolicy>,
    reviews: &[EquipmentSupplyReview],
) -> EquipmentSupplyPolicyStatus {
    let raw_cash = crate::resources::raw_purchase_cash(w, id);
    let valid_cash = raw_cash.is_finite() && raw_cash >= 0.0;
    let cash = if valid_cash { raw_cash } else { 0.0 };
    let mut out = EquipmentSupplyPolicyStatus {
        automatic: policy.is_some_and(|p| p.automatic),
        next_review_day: policy
            .filter(|p| p.automatic)
            .map(|p| supply_policy_due(p, reviews)),
        can_review: false,
        cash_available_bn: cash,
        effective_cap_bn: 0.0,
        reason: "Automatic material purchasing has not been configured.".into(),
        quote: None,
    };
    let Some(policy) = policy else {
        return out;
    };
    if let Some(reason) = supply_policy_values_refusal(
        policy.horizon_days,
        policy.spending_cap_bn,
        policy.cash_floor_bn,
        policy.review_interval_days,
    ) {
        out.reason = reason;
        return out;
    }
    out.effective_cap_bn =
        supply_policy_effective_cap(cash, policy.spending_cap_bn, policy.cash_floor_bn);
    // An illustration is still useful before authorization takes effect or
    // while automatic reviews are off. The ordinary quote remains pure.
    if valid_cash && out.effective_cap_bn >= 1e-9 {
        out.quote = Some(replenishment_quote(
            w,
            id,
            policy.horizon_days,
            out.effective_cap_bn,
        ));
    }
    let day = clock::absolute_day(w);
    out.reason = if !policy.automatic {
        "Automatic purchasing is off. These settings authorize no future purchases.".into()
    } else if let Some(reason) = supply_policy_actor_refusal(w, id, true) {
        reason
    } else if !valid_cash {
        "Available purchasing cash is invalid; no policy purchase can run.".into()
    } else if supply_policy_due(policy, reviews) > day {
        "Waiting for the next authorized review date. Review limits do not accumulate or create catch-up purchases.".into()
    } else if w
        .nation(id)
        .program_budget
        .as_ref()
        .is_none_or(|p| p.day != Some(day) || p.settled_day != Some(day))
    {
        "Waiting for today's fiscal settlement. Material purchases use cash left after the day's existing bills.".into()
    } else {
        out.can_review = true;
        if out.effective_cap_bn < 1e-9 {
            "The spending ceiling or minimum cash reserve leaves less than one dollar available. This review will buy nothing and will not borrow.".into()
        } else if let Some(reason) = out.quote.as_ref().and_then(|q| q.reason.clone()) {
            reason
        } else {
            "This due review can execute one bounded material purchase using the ordinary supplier, stock, cash and freight rules.".into()
        }
    };
    out
}
pub fn supply_policy_status(w: &WorldState, id: NationId) -> EquipmentSupplyPolicyStatus {
    let state = w
        .nation_opt(id)
        .and_then(|n| n.equipment.as_ref())
        .and_then(|s| s.supply_automation.as_ref());
    supply_policy_status_for(
        w,
        id,
        state.and_then(|s| s.policy.as_ref()),
        state.map_or(&[], |s| s.reviews.as_slice()),
    )
}
pub fn supply_policy_quote(
    w: &WorldState,
    id: NationId,
    horizon: u32,
    cap: f64,
    floor: f64,
    interval: u32,
    automatic: bool,
) -> EquipmentSupplyPolicyQuote {
    let reason = supply_policy_actor_refusal(w, id, automatic)
        .or_else(|| supply_policy_values_refusal(horizon, cap, floor, interval))
        .or_else(|| validate_state(w.nation(id)).err());
    let day = clock::absolute_day(w);
    let policy = EquipmentSupplyPolicy {
        automatic,
        horizon_days: horizon,
        spending_cap_bn: cap,
        cash_floor_bn: floor,
        review_interval_days: interval,
        authorized_day: day,
    };
    let reviews: &[EquipmentSupplyReview] = w
        .nation_opt(id)
        .and_then(|n| n.equipment.as_ref())
        .and_then(|s| s.supply_automation.as_ref())
        .map_or(&[], |s| s.reviews.as_slice());
    EquipmentSupplyPolicyQuote {
        valid: reason.is_none(),
        reason,
        automatic,
        horizon_days: horizon,
        spending_cap_bn: cap,
        cash_floor_bn: floor,
        review_interval_days: interval,
        eligible_from_day: supply_policy_due(&policy, reviews),
        status: supply_policy_status_for(w, id, Some(&policy), reviews),
    }
}
pub fn set_supply_policy(
    w: &mut WorldState,
    id: NationId,
    horizon: u32,
    cap: f64,
    floor: f64,
    interval: u32,
    automatic: bool,
) -> Result<(), String> {
    let quote = supply_policy_quote(w, id, horizon, cap, floor, interval, automatic);
    if let Some(reason) = quote.reason {
        return Err(reason);
    }
    let day = clock::absolute_day(w);
    let state = state_mut(w.nation_mut(id))
        .supply_automation
        .get_or_insert_with(Default::default);
    state.policy = Some(EquipmentSupplyPolicy {
        automatic,
        horizon_days: horizon,
        spending_cap_bn: cap,
        cash_floor_bn: floor,
        review_interval_days: interval,
        authorized_day: day,
    });
    Ok(())
}
pub fn clear_supply_policy(w: &mut WorldState, id: NationId) -> Result<(), String> {
    if let Some(reason) = actor_refusal(w, id) {
        return Err(reason);
    }
    validate_state(w.nation(id))?;
    if !w
        .nation(id)
        .equipment
        .as_ref()
        .and_then(|s| s.supply_automation.as_ref())
        .is_some_and(|s| s.policy.is_some())
    {
        return Err("There is no material-purchasing policy to clear.".into());
    }
    state_mut(w.nation_mut(id))
        .supply_automation
        .as_mut()
        .unwrap()
        .policy = None;
    Ok(())
}

/// Post-fiscal daily tail, after reserve scheduling and both fiscal/province
/// settlement. This tail never opens an early funding or transportation day.
pub fn tick_supply_automation(w: &mut WorldState) {
    if !clock::is_daily(w) || !w.rules.military_operations {
        return;
    }
    let mut ids: Vec<_> = w
        .nations
        .iter()
        .filter(|n| {
            n.alive
                && n.equipment
                    .as_ref()
                    .and_then(|s| s.supply_automation.as_ref())
                    .and_then(|s| s.policy.as_ref())
                    .is_some_and(|p| p.automatic)
        })
        .map(|n| n.id)
        .collect();
    ids.sort();
    for id in ids {
        // UI reads may request an illustrative quote at any time. The tick
        // need not clone/price a market bundle before its review is due.
        let day = clock::absolute_day(w);
        let n = w.nation(id);
        let state = n
            .equipment
            .as_ref()
            .unwrap()
            .supply_automation
            .as_ref()
            .unwrap();
        if supply_policy_due(state.policy.as_ref().unwrap(), &state.reviews) > day
            || n.program_budget
                .as_ref()
                .is_none_or(|p| p.day != Some(day) || p.settled_day != Some(day))
        {
            continue;
        }
        if validate_state(w.nation(id)).is_err() {
            continue;
        }
        let status = supply_policy_status(w, id);
        if !status.can_review {
            continue;
        }
        let day = clock::absolute_day(w);
        let policy = w
            .nation(id)
            .equipment
            .as_ref()
            .unwrap()
            .supply_automation
            .as_ref()
            .unwrap()
            .policy
            .as_ref()
            .unwrap()
            .clone();
        let mut review = EquipmentSupplyReview {
            day,
            authorized_day: policy.authorized_day,
            horizon_days: policy.horizon_days,
            spending_cap_bn: policy.spending_cap_bn,
            cash_floor_bn: policy.cash_floor_bn,
            review_interval_days: policy.review_interval_days,
            cash_available_bn: status.cash_available_bn,
            effective_cap_bn: status.effective_cap_bn,
            requested: status.quote.as_ref().map_or([0.0; 12], |q| q.requested),
            purchase: None,
            reason: status.reason,
        };
        if status.effective_cap_bn >= 1e-9 && status.quote.as_ref().is_some_and(|q| q.valid) {
            match replenish_with_receipt(w, id, policy.horizon_days, status.effective_cap_bn) {
                Ok(receipt) => {
                    review.requested = receipt.requested;
                    review.reason = if receipt.warnings.is_empty() {
                        "Purchased the available funded material gap within this review's cash ceiling and reserve.".into()
                    } else {
                        receipt.warnings.join(" ")
                    };
                    review.purchase = Some(receipt);
                }
                Err(reason) => review.reason = reason,
            }
        }
        let state = state_mut(w.nation_mut(id))
            .supply_automation
            .as_mut()
            .unwrap();
        state.reviews.push(review);
        if state.reviews.len() > MAX_SUPPLY_POLICY_REVIEWS {
            state.reviews.remove(0);
        }
    }
}

pub fn validate_supply_automation(n: &Nation) -> Result<(), String> {
    let Some(s) = &n.equipment else {
        return Ok(());
    };
    let Some(state) = &s.supply_automation else {
        return Ok(());
    };
    let invalid = || "Invalid material-purchasing policy or retained review receipt.".to_string();
    if s.version < 7 || state.reviews.len() > MAX_SUPPLY_POLICY_REVIEWS {
        return Err(invalid());
    }
    let fiscal_day = n.program_budget.as_ref().and_then(|p| p.day);
    if let Some(p) = &state.policy {
        if supply_policy_values_refusal(
            p.horizon_days,
            p.spending_cap_bn,
            p.cash_floor_bn,
            p.review_interval_days,
        )
        .is_some()
            || fiscal_day.is_some_and(|d| p.authorized_day > d.saturating_add(1))
            || p.automatic && n.program_budget.is_none()
        {
            return Err(invalid());
        }
    }
    let near = |a: f64, b: f64| (a - b).abs() <= 1e-12 + 1e-12 * a.abs().max(b.abs());
    let mut last = None;
    for r in &state.reviews {
        if supply_policy_values_refusal(
            r.horizon_days,
            r.spending_cap_bn,
            r.cash_floor_bn,
            r.review_interval_days,
        )
        .is_some()
            || r.day <= r.authorized_day
            || fiscal_day.is_none_or(|d| r.day > d)
            || last.is_some_and(|d: i32| r.day < d.saturating_add(r.review_interval_days as i32))
            || [r.cash_available_bn, r.effective_cap_bn]
                .iter()
                .any(|v| !v.is_finite() || *v < 0.0)
            || !near(
                r.effective_cap_bn,
                supply_policy_effective_cap(
                    r.cash_available_bn,
                    r.spending_cap_bn,
                    r.cash_floor_bn,
                ),
            )
            || r.requested.iter().any(|v| !v.is_finite() || *v < 0.0)
            || r.requested[crate::resources::Commodity::Oil.idx()] != 0.0
        {
            return Err(invalid());
        }
        if let Some(p) = &r.purchase {
            if p.fills.is_empty()
                || p.fills.len() > 12
                || p.requested != r.requested
                || !near(p.cash_available_bn, r.cash_available_bn)
                || !p.spent_bn.is_finite()
                || p.spent_bn < 1e-9
                || p.spent_bn > r.effective_cap_bn + 1e-12
                || r.cash_available_bn - p.spent_bn + 1e-12 < r.cash_floor_bn
            {
                return Err(invalid());
            }
            let mut purchased = [0.0; 12];
            let mut spent = 0.0;
            let mut keys = BTreeSet::new();
            for fill in &p.fills {
                if fill.seller == n.id
                    || fill.commodity == crate::resources::Commodity::Oil
                    || !fill.commodity.tracked()
                    || !fill.quantity.is_finite()
                    || fill.quantity <= 1e-9
                    || !fill.cost_bn.is_finite()
                    || fill.cost_bn < 1e-9
                    || !keys.insert((fill.commodity.idx(), fill.seller))
                {
                    return Err(invalid());
                }
                purchased[fill.commodity.idx()] += fill.quantity;
                spent += fill.cost_bn;
            }
            if !near(spent, p.spent_bn) {
                return Err(invalid());
            }
            for i in 0..12 {
                if !p.purchased[i].is_finite()
                    || p.purchased[i] < 0.0
                    || !near(p.purchased[i], purchased[i])
                    || p.purchased[i] > r.requested[i] + 1e-9
                {
                    return Err(invalid());
                }
            }
        }
        last = Some(r.day);
    }
    Ok(())
}

#[cfg(test)]
mod supply_automation_tests {
    use super::*;
    use crate::{programs, resources};
    const ID: NationId = NationId::France;
    fn fixture() -> WorldState {
        super::replenishment_tests::fixture()
    }
    fn state(w: &WorldState) -> &EquipmentSupplyAutomation {
        w.nation(ID)
            .equipment
            .as_ref()
            .unwrap()
            .supply_automation
            .as_ref()
            .unwrap()
    }
    fn close_next(w: &mut WorldState) {
        clock::advance_date(w);
        programs::begin_day(w);
        // These unit fixtures isolate the purchasing owner from fiscal policy;
        // integration tests exercise real daily revenue and bills together.
        let bill: f64 = w
            .nation(ID)
            .program_budget
            .as_ref()
            .unwrap()
            .spent_today_bn
            .iter()
            .flatten()
            .sum();
        let budget = w.nation(ID).program_budget.as_ref().unwrap();
        let revenue_share = bill / (budget.basis_gdp * budget.fraction);
        programs::stage_fiscal(w.nation_mut(ID), revenue_share, 0.0);
        programs::finish_day(w);
        w.resources.last_tick_day = Some(clock::absolute_day(w));
    }
    fn review_next(w: &mut WorldState) {
        close_next(w);
        tick_supply_automation(w);
    }
    #[test]
    fn policy_illustrations_and_configuration_are_pure_until_a_later_review() {
        let mut w = fixture();
        let before = crate::save(&w);
        let q = supply_policy_quote(&w, ID, 30, 0.001, 0.0, 7, true);
        assert!(q.valid, "{:?}", q.reason);
        assert!(q.status.quote.as_ref().is_some_and(|q| q.valid));
        assert_eq!(crate::save(&w), before);
        for (horizon, cap, floor, interval) in [
            (31, 0.001, 0.0, 7),
            (30, -1.0, 0.0, 7),
            (30, 0.001, -1.0, 7),
            (30, f64::NAN, 0.0, 7),
            (30, 0.001, f64::INFINITY, 7),
            (30, 0.001, 0.0, 2),
        ] {
            assert!(set_supply_policy(&mut w, ID, horizon, cap, floor, interval, true).is_err());
            assert_eq!(crate::save(&w), before);
        }
        let resources = serde_json::to_string(&w.resources).unwrap();
        let logistics = w.logistics.clone();
        let cash = w.nation(ID).treasury_bn;
        set_supply_policy(&mut w, ID, 30, 0.001, 0.0, 7, true).unwrap();
        assert!(state(&w).reviews.is_empty());
        assert_eq!(w.nation(ID).treasury_bn, cash);
        assert_eq!(serde_json::to_string(&w.resources).unwrap(), resources);
        assert_eq!(w.logistics, logistics);
        let saved = crate::save(&w);
        tick_supply_automation(&mut w);
        assert_eq!(crate::save(&w), saved);
    }
    #[test]
    fn automatic_purchase_is_the_same_transaction_as_manual_with_exact_cash_floor() {
        let mut w = fixture();
        w.nation_mut(ID).treasury_bn = Some(0.003);
        set_supply_policy(&mut w, ID, 30, 0.01, 0.002, 7, true).unwrap();
        close_next(&mut w);
        let status = supply_policy_status(&w, ID);
        assert!(status.can_review);
        assert!((status.effective_cap_bn - 0.001).abs() < 1e-15);
        assert!(status.quote.as_ref().unwrap().valid);
        let mut manual = w.clone();
        let budget = w.nation(ID).program_budget.clone();
        let debt = w.nation(ID).debt_bn;
        replenish(&mut manual, ID, 30, status.effective_cap_bn).unwrap();
        tick_supply_automation(&mut w);
        assert_eq!(
            serde_json::to_string(&w.resources).unwrap(),
            serde_json::to_string(&manual.resources).unwrap()
        );
        assert_eq!(w.logistics, manual.logistics);
        for n in &w.nations {
            assert_eq!(n.treasury_bn, manual.nation(n.id).treasury_bn);
            assert_eq!(n.debt_bn, manual.nation(n.id).debt_bn);
        }
        assert_eq!(w.nation(ID).program_budget, budget);
        assert_eq!(w.nation(ID).debt_bn, debt);
        assert!(resources::raw_purchase_cash(&w, ID) >= 0.002 - 1e-12);
        assert_eq!(state(&w).reviews.len(), 1);
        let r = &state(&w).reviews[0];
        let purchase = r.purchase.as_ref().unwrap();
        assert!(purchase.spent_bn <= r.effective_cap_bn);
        assert_eq!(purchase, state_purchase(&status));
        let saved = crate::save(&w);
        tick_supply_automation(&mut w);
        assert_eq!(crate::save(&w), saved);
        assert!(validate_state(w.nation(ID)).is_ok());
    }
    fn state_purchase(status: &EquipmentSupplyPolicyStatus) -> &resources::RawPurchaseReceipt {
        &status.quote.as_ref().unwrap().receipt
    }
    #[test]
    fn failed_reviews_advance_cadence_and_edits_do_not_create_catch_up_or_same_day_credit() {
        let mut w = fixture();
        set_supply_policy(&mut w, ID, 30, 0.001, 1000.0, 7, true).unwrap();
        review_next(&mut w);
        assert_eq!(state(&w).reviews.len(), 1);
        assert!(state(&w).reviews[0].purchase.is_none());
        assert_eq!(state(&w).reviews[0].effective_cap_bn, 0.0);
        let reviewed_day = state(&w).reviews[0].day;
        set_supply_policy(&mut w, ID, 30, 0.001, 0.0, 7, true).unwrap();
        assert_eq!(
            supply_policy_status(&w, ID).next_review_day,
            Some(reviewed_day + 7)
        );
        tick_supply_automation(&mut w);
        assert_eq!(state(&w).reviews.len(), 1);
        for _ in 0..6 {
            review_next(&mut w);
        }
        assert_eq!(state(&w).reviews.len(), 1);
        review_next(&mut w);
        assert_eq!(state(&w).reviews.len(), 2);
        assert!(state(&w).reviews[1].purchase.is_some());
        // Skipped dates authorize one present review, never every missed cap.
        for _ in 0..20 {
            close_next(&mut w);
        }
        tick_supply_automation(&mut w);
        assert_eq!(state(&w).reviews.len(), 3);
        assert_eq!(
            supply_policy_status(&w, ID).next_review_day,
            Some(clock::absolute_day(&w) + 7)
        );
    }
    #[test]
    fn paused_work_and_expired_fabrication_authority_create_no_purchase_demand() {
        for expired in [false, true] {
            let mut w = fixture();
            set_supply_policy(&mut w, ID, 30, 0.1, 0.0, 1, true).unwrap();
            if expired {
                w.year += 1;
                w.month = 1;
                w.day = 1;
            } else {
                w.nation_mut(ID).equipment.as_mut().unwrap().projects[0].paused = true;
            }
            review_next(&mut w);
            let r = &state(&w).reviews[0];
            assert!(r.purchase.is_none());
            assert_eq!(r.requested, [0.0; 12]);
            assert!(r.reason.contains("No additional purchase"));
        }
    }
    #[test]
    fn disable_and_clear_preserve_cargo_and_frozen_history_while_stopping_future_purchases() {
        let mut w = fixture();
        set_supply_policy(&mut w, ID, 30, 0.001, 0.0, 1, true).unwrap();
        review_next(&mut w);
        let old_reviews = state(&w).reviews.clone();
        let cargo = w.logistics.clone();
        set_supply_policy(&mut w, ID, 365, 0.0, 1000.0, 30, false).unwrap();
        assert_eq!(state(&w).reviews, old_reviews);
        assert_eq!(w.logistics, cargo);
        review_next(&mut w);
        assert_eq!(state(&w).reviews, old_reviews);
        clear_supply_policy(&mut w, ID).unwrap();
        assert!(state(&w).policy.is_none());
        assert_eq!(state(&w).reviews, old_reviews);
        assert_eq!(w.logistics, cargo);
        assert!(validate_state(w.nation(ID)).is_ok());
        let saved = crate::save(&w);
        assert!(clear_supply_policy(&mut w, ID).is_err());
        assert_eq!(crate::save(&w), saved);
        let mut loaded = crate::load(&saved).unwrap();
        assert_eq!(crate::save(&loaded), saved);
        for _ in 0..3 {
            review_next(&mut w);
            review_next(&mut loaded);
        }
        assert_eq!(crate::save(&w), crate::save(&loaded));
    }
    #[test]
    fn bounded_audit_history_retains_latest_guard_and_does_not_exhaust_policy_lifetime() {
        let mut w = fixture();
        set_supply_policy(&mut w, ID, 30, 0.0, 0.0, 1, true).unwrap();
        for _ in 0..MAX_SUPPLY_POLICY_REVIEWS + 3 {
            review_next(&mut w);
        }
        assert_eq!(state(&w).reviews.len(), MAX_SUPPLY_POLICY_REVIEWS);
        let day = clock::absolute_day(&w);
        assert_eq!(
            state(&w).reviews.first().unwrap().day,
            day - (MAX_SUPPLY_POLICY_REVIEWS as i32 - 1)
        );
        clear_supply_policy(&mut w, ID).unwrap();
        set_supply_policy(&mut w, ID, 30, 0.0, 0.0, 1, true).unwrap();
        let saved = crate::save(&w);
        tick_supply_automation(&mut w);
        assert_eq!(crate::save(&w), saved);
        review_next(&mut w);
        assert_eq!(state(&w).reviews.len(), MAX_SUPPLY_POLICY_REVIEWS);
        assert_eq!(state(&w).reviews.last().unwrap().day, day + 1);
        assert!(validate_state(w.nation(ID)).is_ok());
    }
    #[test]
    fn persisted_receipt_rejects_forged_costs_floors_fills_and_dates_after_policy_changes() {
        let mut w = fixture();
        set_supply_policy(&mut w, ID, 30, 0.001, 0.0, 7, true).unwrap();
        review_next(&mut w);
        set_supply_policy(&mut w, ID, 365, 0.0, 1000.0, 30, false).unwrap();
        assert!(validate_state(w.nation(ID)).is_ok());
        for flaw in 0..8 {
            let mut bad = w.clone();
            let r = &mut state_mut(bad.nation_mut(ID))
                .supply_automation
                .as_mut()
                .unwrap()
                .reviews[0];
            match flaw {
                0 => r.day += 10,
                1 => r.cash_floor_bn = r.cash_available_bn + 1.0,
                2 => r.purchase.as_mut().unwrap().spent_bn = 0.0,
                3 => r.purchase.as_mut().unwrap().fills[0].seller = ID,
                4 => {
                    r.purchase.as_mut().unwrap().purchased[resources::Commodity::Copper.idx()] +=
                        1.0
                }
                5 => r.purchase.as_mut().unwrap().requested = [0.0; 12],
                6 => r.effective_cap_bn += 1.0,
                _ => r.authorized_day = r.day,
            }
            assert!(
                validate_state(bad.nation(ID)).is_err(),
                "forged receipt {flaw}"
            );
            assert!(crate::load(&crate::save(&bad)).is_err());
        }
        let saved = crate::save(&w);
        assert_eq!(crate::save(&crate::load(&saved).unwrap()), saved);
        let mut old = fixture();
        old.nation_mut(ID).equipment.as_mut().unwrap().version = 6;
        let old_saved = crate::save(&old);
        assert_eq!(crate::save(&crate::load(&old_saved).unwrap()), old_saved);
    }
}
