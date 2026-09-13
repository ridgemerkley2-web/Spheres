//! Congestion-aware dispatch, deliberately separate from nominal route previews.
//! Searches use integer travel costs and stable graph order, never random ties.
use super::*;

/// Boolean validity of the nominal routes already present in the live freight
/// cache, scoped to one spot clearing. That operation changes stock, usage,
/// cash and contractor experience, but cannot change route access. Congestion
/// alternatives and capacity are deliberately absent from this memo.
pub(super) struct ClearingRouteAccess {
    paths: BTreeMap<(NationId, NationId, RoutePolicy), (Vec<String>, bool)>,
    cache_limit: usize,
}

impl Default for ClearingRouteAccess {
    fn default() -> Self { Self { paths: BTreeMap::new(), cache_limit: 4_096 } }
}

impl ClearingRouteAccess {
    pub(super) fn is_open(&mut self, w: &WorldState, seller: NationId, buyer: NationId,
        policy: RoutePolicy, route: &RoutePlan) -> bool {
        let key=(seller,buyer,policy);
        if let Some((nodes,open))=self.paths.get(&key) {
            if nodes.iter().map(String::as_str).eq(route.nodes.iter().map(|node|node.id.as_str())) {
                return *open;
            }
        }
        let open=route_open(w,seller,buyer,route).is_ok();
        if self.paths.len()<self.cache_limit || self.paths.contains_key(&key) {
            self.paths.insert(key,(route.nodes.iter().map(|node|node.id.clone()).collect(),open));
        }
        open
    }
}

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
    prepare_bundle_with_context(w, legs, used, None)
}

pub(super) fn prepare_bundle_with_context(w: &WorldState, legs: &[(NationId, NationId, Commodity, f64)],
    used: &BTreeMap<String, f64>, mut context: Option<&mut ContractForecastRoutes<'_>>) -> Bundle {
    let mut result = Bundle { routes: vec![], demand: BTreeMap::new(), ratio: 1.0 };
    for &(seller, buyer, commodity, quantity) in legs {
        let route = if !quantity.is_finite() || quantity < 0.0 {
            Err("Requested freight must be finite and non-negative.".into())
        } else {
            let nominal = match context.as_deref_mut() {
                Some(read) => read.plan(seller, buyer),
                None => plan(w, seller, buyer),
            };
            let capacities = context.as_ref().and_then(|read| read.capacities.as_deref());
            nominal.map(|p| select(w, seller, buyer, p, quantity * tonnes_per_unit(commodity), used, capacities, None).0)
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
        let capacities = context.as_ref().and_then(|read| read.capacities.as_deref());
        let room = (route_segment_capacity_with(w, edge, capacities).unwrap_or(0.0) - used.get(edge).copied().unwrap_or(0.0)).max(0.0);
        result.ratio = result.ratio.min((room / wanted.max(EPS)).clamp(0.0, 1.0));
    }
    result
}

pub(super) fn fresh_contract_ratios(w: &WorldState,
    bundles: &[(u32, Vec<(NationId, NationId, Commodity, f64)>)]) -> BTreeMap<u32, f64> {
    fresh_contract_ratios_impl(w, bundles, None)
}

pub(super) fn fresh_contract_ratios_with_context(context: &mut ContractForecastRoutes<'_>,
    bundles: &[(u32, Vec<(NationId, NationId, Commodity, f64)>)]) -> BTreeMap<u32, f64> {
    let world = context.world;
    fresh_contract_ratios_impl(world, bundles, Some(context))
}

fn fresh_contract_ratios_impl(w: &WorldState,
    bundles: &[(u32, Vec<(NationId, NationId, Commodity, f64)>)],
    mut context: Option<&mut ContractForecastRoutes<'_>>) -> BTreeMap<u32, f64> {
    let mut ordered: Vec<_> = bundles.iter().collect();
    ordered.sort_by_key(|(id, _)| *id);
    let mut used = BTreeMap::new();
    let mut ratios = BTreeMap::new();
    for (id, legs) in ordered {
        let bundle = prepare_bundle_with_context(w, legs, &used, context.as_deref_mut());
        for (edge, wanted) in bundle.demand {
            *used.entry(edge).or_default() += wanted * bundle.ratio;
        }
        ratios.insert(*id, bundle.ratio);
    }
    ratios
}

#[cfg(test)]
mod access_read_tests {
    use super::*;

    fn world() -> WorldState {
        crate::init::world_1990(crate::world::GameRules {
            daily_simulation: true, resource_market: true, logistics_routes: true,
            physical_logistics: true, military_operations: true, ..Default::default()
        })
    }

    #[test]
    fn clearing_access_reuse_preserves_dispatches_invalid_cache_and_alternates() {
        let mut cached=world();
        let (seller,buyer)=(NationId::Germany,NationId::France);
        set_policy(&mut cached,buyer,RoutePolicy::LandOnly).unwrap();
        begin_month(&mut cached);
        let nominal=plan(&cached,seller,buyer).unwrap();
        // The first lookup must reject an invalid preexisting nominal cache.
        // Reserving the real nominal path also makes a later lot divert.
        let mut invalid=nominal.clone(); invalid.nodes[0].id="missing-cached-node".into();
        cached.logistics.route_cache.insert((seller,buyer,RoutePolicy::LandOnly),invalid);
        for segment in &nominal.segments {
            let capacity=route_segment_capacity(&cached,segment).unwrap();
            cached.logistics.usage_tonnes.insert(segment.clone(),capacity);
        }
        let mut plain=cached.clone();
        let mut cached_routes=ClearingRoutes::new(&cached);
        let mut plain_routes=ClearingRoutes::new(&plain);
        // Native access validation with the identical existing search trees,
        // frozen capacity inputs and shared congestion-search budget.
        plain_routes.access.cache_limit=0;
        let mut diverted=false;
        for (commodity,quantity) in [(Commodity::Iron,0.0),(Commodity::Copper,0.125),
            (Commodity::Coal,100_000.0),(Commodity::Iron,0.25),(Commodity::Copper,1.0)] {
            let actual=dispatch_in_clearing(&mut cached,seller,buyer,commodity,quantity,&mut cached_routes);
            let expected=dispatch_in_clearing(&mut plain,seller,buyer,commodity,quantity,&mut plain_routes);
            assert_eq!(actual,expected,"{commodity:?} {quantity}: exact route, quantity and refusal");
            assert_eq!(cached_routes.search_nodes_left,plain_routes.search_nodes_left,"access caching cannot create search budget");
            assert_eq!(crate::save(&cached),crate::save(&plain),"all capacity, cargo, cash and contractor receipts stay exact");
            diverted|=actual.quantity>0.0 && actual.route.as_ref().is_some_and(|route|route.dispatch_note.is_some());
        }
        assert!(diverted,"the fixture must actually dispatch an alternate route");
        assert_eq!(cached_routes.access.paths.len(),1);
        assert!(plain_routes.access.paths.is_empty());
    }

    #[test]
    fn clearing_access_reuse_checks_changed_paths_and_new_clearing_closures() {
        let mut w=world();
        let (seller,buyer)=(NationId::Germany,NationId::France);
        let route=plan(&w,seller,buyer).unwrap();
        let mut invalid=route.clone(); invalid.nodes[0].id="missing-cached-node".into();
        let mut renamed=route.clone();
        for node in &mut renamed.nodes {node.name="A different display name".into();}
        let before=crate::save(&w);
        let mut access=ClearingRouteAccess::default();
        for candidate in [&invalid,&route,&renamed,&invalid,&route] {
            assert_eq!(access.is_open(&w,seller,buyer,RoutePolicy::Fastest,candidate),
                route_open(&w,seller,buyer,candidate).is_ok());
            assert_eq!(access.paths.len(),1,"the exact changed path replaces only its endpoint/policy entry");
        }
        assert_eq!(crate::save(&w),before);
        w.sanctions.push((seller,buyer));
        let mut next_clearing=ClearingRouteAccess::default();
        assert!(!next_clearing.is_open(&w,seller,buyer,RoutePolicy::Fastest,&route));
        assert!(!next_clearing.is_open(&w,seller,buyer,RoutePolicy::Fastest,&route));
        w.sanctions.pop();
        let mut reopened=ClearingRouteAccess::default();
        assert!(reopened.is_open(&w,seller,buyer,RoutePolicy::Fastest,&route));
    }

    #[test]
    fn clearing_access_reuse_bounded_fallback_keeps_native_answers() {
        let w=world();
        let route=plan(&w,NationId::Germany,NationId::France).unwrap();
        let mut access=ClearingRouteAccess {cache_limit:1,..Default::default()};
        for (seller,buyer) in [(NationId::Germany,NationId::France),(NationId::France,NationId::Germany),
            (NationId::Germany,NationId::Germany),(NationId::France,NationId::France)] {
            for _ in 0..2 {
                assert_eq!(access.is_open(&w,seller,buyer,RoutePolicy::Fastest,&route),
                    route_open(&w,seller,buyer,&route).is_ok());
            }
        }
        assert_eq!(access.paths.len(),1);
    }
}
