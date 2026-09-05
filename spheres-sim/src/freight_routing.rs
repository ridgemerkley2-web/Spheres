//! Congestion-aware dispatch, deliberately separate from nominal route previews.
//! Searches use integer travel costs and stable graph order, never random ties.
use super::*;

fn remaining(w: &WorldState, route: &RoutePlan, used: &BTreeMap<String, f64>, capacities: Option<&[f64]>) -> f64 {
    route.segments.iter().map(|edge| (route_segment_capacity_with(w, edge, capacities)
        .unwrap_or(route.capacity_tonnes) - used.get(edge).copied().unwrap_or(0.0)).max(0.0))
        .fold(f64::INFINITY, f64::min)
}

/// Preserve the nominal fastest route when it fits. Otherwise try the fastest
/// route that can fit the whole lot, then the fastest route with strictly more
/// room than the nominal bottleneck. No split cargo, new capacity or departure
/// before the settlement. Route policy and physical control still constrain it.
pub(super) fn select(w: &WorldState, seller: NationId, buyer: NationId, nominal: RoutePlan,
    wanted_tonnes: f64, used: &BTreeMap<String, f64>, capacities: Option<&[f64]>,
    mut search_budget: Option<&mut usize>) -> (RoutePlan, bool) {
    if !w.rules.military_operations || wanted_tonnes <= 0.0
        || search_budget.as_ref().is_some_and(|left| **left == 0) {
        return (nominal, false);
    }
    let room = remaining(w, &nominal, used, capacities);
    if wanted_tonnes <= room { return (nominal, false); }
    let first = plan_search(w, seller, buyer, true, Some((used, wanted_tonnes, capacities)), search_budget.as_deref_mut());
    let alternate = first.or_else(|_| plan_search(w, seller, buyer, true,
        Some((used, room + 1e-7, capacities)), search_budget.as_deref_mut()));
    match alternate {
        Ok(mut route) if remaining(w, &route, used, capacities) > room => {
            route.dispatch_note = Some(format!("Congestion required an alternate route: {} days of travel instead of {}.",
                route.estimated_days, nominal.estimated_days));
            (route, true)
        }
        _ => (nominal, false),
    }
}

pub(super) fn dispatch_reason(w: &WorldState, alternate: bool, partial: bool, route: &RoutePlan) -> Option<String> {
    if alternate {
        Some(format!("Congestion diverted this shipment to an alternate route ({} days).{}",
            route.estimated_days, if partial { " The remaining goods wait for capacity at the next settlement." } else { "" }))
    } else if partial {
        Some(if w.rules.military_operations {
            "The selected route cannot carry the whole lot; the remaining goods wait for the next settlement."
        } else { "Shared freight capacity is committed; the remaining lot waits for another day." }.into())
    } else { None }
}

pub(super) struct Bundle {
    pub routes: Vec<Result<RoutePlan, String>>,
    pub demand: BTreeMap<String, f64>,
    pub ratio: f64,
}

/// Freeze every leg against the same opening edge ledger. The subsequent
/// dispatch uses these exact routes; recomputing after the first leg could
/// choose a new route and break the common service fraction of a barter deal.
pub(super) fn prepare_bundle(w: &WorldState, legs: &[(NationId, NationId, Commodity, f64)],
    used: &BTreeMap<String, f64>) -> Bundle {
    let mut result = Bundle { routes: vec![], demand: BTreeMap::new(), ratio: 1.0 };
    for &(seller, buyer, commodity, quantity) in legs {
        let route = if !quantity.is_finite() || quantity < 0.0 {
            Err("Requested freight must be finite and non-negative.".into())
        } else {
            plan(w, seller, buyer).map(|p| select(w, seller, buyer, p, quantity * tonnes_per_unit(commodity), used, None, None).0)
        };
        if quantity > 0.0 {
            if let Ok(route) = &route {
                for edge in &route.segments {
                    *result.demand.entry(edge.clone()).or_default() += quantity * tonnes_per_unit(commodity);
                }
            } else { result.ratio = 0.0; }
        }
        if !quantity.is_finite() || quantity < 0.0 { result.ratio = 0.0; }
        result.routes.push(route);
    }
    for (edge, wanted) in &result.demand {
        let room = (route_segment_capacity(w, edge).unwrap_or(0.0) - used.get(edge).copied().unwrap_or(0.0)).max(0.0);
        result.ratio = result.ratio.min((room / wanted.max(EPS)).clamp(0.0, 1.0));
    }
    result
}

pub(super) fn fresh_contract_ratios(w: &WorldState,
    bundles: &[(u32, Vec<(NationId, NationId, Commodity, f64)>)]) -> BTreeMap<u32, f64> {
    let mut ordered: Vec<_> = bundles.iter().collect();
    ordered.sort_by_key(|(id, _)| *id);
    let mut used = BTreeMap::new();
    let mut ratios = BTreeMap::new();
    for (id, legs) in ordered {
        let bundle = prepare_bundle(w, legs, &used);
        for (edge, wanted) in bundle.demand {
            *used.entry(edge).or_default() += wanted * bundle.ratio;
        }
        ratios.insert(*id, bundle.ratio);
    }
    ratios
}
