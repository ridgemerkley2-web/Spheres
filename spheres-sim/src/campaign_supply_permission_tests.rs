use super::*;
use crate::theatre::{Access, TheatreId};
use std::cell::Cell;

thread_local! {
    pub(super) static FRESH: Cell<bool> = const { Cell::new(false) };
    pub(super) static REUSED: Cell<u64> = const { Cell::new(0) };
}

fn fresh_permissions<T>(run: impl FnOnce() -> T) -> T {
    struct Reset(bool);
    impl Drop for Reset {
        fn drop(&mut self) { FRESH.with(|flag|flag.set(self.0)); }
    }
    let _reset=Reset(FRESH.with(|flag|flag.replace(true)));
    run()
}

fn route_bits(result: Result<Route,String>) -> Result<(Vec<String>,Vec<usize>,u32,bool,u64),String> {
    result.map(|r|(r.nodes,r.edges,r.days,r.sea,r.capacity.to_bits()))
}

fn access(w: &mut WorldState,nation: NationId) {
    w.access.push(Access {host:NationId::France,seeker:nation,
        theatre:TheatreId::CentralEurope,since_year:1990,since_month:1});
}

#[test]
fn permission_scope_reuses_only_immutable_queries_and_fresh_scopes_observe_mutations() {
    let (mut w,r)=super::tests::fixture();
    logistics::begin_month(&mut w);
    let mut expected_open=Vec::new();
    for state in 0..8 {
        // These mutations happen AFTER dropping the entire prior query scope,
        // on the same calendar date. No date-based permission cache is allowed.
        match state {
            1=>{w.districts.insert(r.district.clone(),NationId::France);}
            2=>access(&mut w,r.nation),
            3=>w.access.clear(),
            4=>{access(&mut w,r.nation);w.nation_mut(NationId::France).alive=false;}
            5=>w.nation_mut(NationId::France).alive=true,
            6=>{
                let theatre=crate::war::theatre_between(&w,r.nation,NationId::France);
                let id=crate::commitment::open_conflict(&mut w,r.nation,NationId::France,theatre).unwrap();
                w.conflict_mut(id).unwrap().front.clear();
            }
            7=>w.conflicts.clear(),
            _=>{}
        }
        let before=crate::save(&w);
        let mut scope=DeploymentRoutes::new(&w);
        let hits=REUSED.with(Cell::get);
        for district in [r.district.as_str(),"DE-BE",r.district.as_str(),"unknown-district"] {
            let actual=scope.route(r.nation,district);
            let expected=fresh_permissions(||deployment_route(&w,r.nation,district));
            assert_eq!(actual,expected,"state {state}, destination {district}");
        }
        assert!(REUSED.with(Cell::get)>hits,"repeated mapped destinations must reuse actual permissions");
        expected_open.push(scope.route(r.nation,&r.district).is_some());
        drop(scope);
        assert_eq!(crate::save(&w),before,"all quotes remain pure");
    }
    assert_eq!(expected_open,[true,false,true,false,false,true,false,true]);

    // Keep the existing private fresh query useful even when callers retain
    // geometry across same-day permission changes, as its older tests do.
    let g=Graph::new(&w);
    assert!(route(&w,&g,&r,"DE-BE").is_ok());
    w.access.clear();
    assert!(route(&w,&g,&r,"DE-BE").is_err());
}

#[test]
fn permission_scope_preserves_each_search_live_reservations_and_complete_supply() {
    let (base,request)=super::tests::fixture();
    for case in 0..6 {
        let mut actual=base.clone();
        let mut r=request.clone();
        match case {
            1=>{actual.nation_mut(r.nation).mil_strength=1_000_000.0;r.deployed=100_000.0;}
            2=>{actual.districts.insert(r.district.clone(),NationId::France);}
            3=>{actual.districts.insert(r.district.clone(),NationId::France);access(&mut actual,r.nation);}
            4=>{actual.districts.insert(r.district.clone(),NationId::France);access(&mut actual,r.nation);
                let theatre=crate::war::theatre_between(&actual,r.nation,NationId::France);
                let conflict=crate::commitment::open_conflict(&mut actual,r.nation,NationId::France,theatre).unwrap();
                // Isolate belligerency from contested-hub selection in this
                // synthetic permission fixture; operational front tests cover
                // contested control separately.
                actual.conflict_mut(conflict).unwrap().front.clear();}
            5=>{actual.campaign_supply.sources.get_mut(&r.nation).unwrap().district="DE-SN".into();}
            _=>{}
        }
        logistics::begin_month(&mut actual);
        let g=Graph::new(&actual);
        let mut permissions=RoutePermissions::default();
        // An immutable access vector may be retained across military capacity
        // reservations, but every heap traversal and bottleneck reads live use.
        let first=route_bits(route_with_permissions(&actual,&g,&r,"DE-BE",Some(&mut permissions)));
        assert_eq!(first,route_bits(route(&actual,&g,&r,"DE-BE")),"first quote {case}");
        let mut capacity_probe=actual.clone();
        for edge in &g.edges {
            capacity_probe.logistics.usage_tonnes.insert(edge.key.clone(),edge.capacity_tonnes*2.0);
        }
        let saturated=route_bits(route_with_permissions(&capacity_probe,&g,&r,"DE-BE",Some(&mut permissions)));
        assert_eq!(saturated,route_bits(route(&capacity_probe,&g,&r,"DE-BE")),"live capacity {case}");
        if case==0 {assert!(first.is_ok()&&saturated.is_err(),"do not cache a previously viable route");}

        let mut second=r.clone();
        second.key=format!("second-{}",r.key);
        second.conflict=2;
        second.sea_escort=0.0;
        second.sea_denial=1.0;
        let requests=[second,r.clone()];
        let mut original=actual.clone();
        let hits=REUSED.with(Cell::get);
        let mut quote=DeploymentRoutes::new(&actual);
        let _=quote.route(r.nation,&r.district);
        let graph=quote.into_supply_graph();
        // Exactly the handoff's permitted mutation: reinstallation of the
        // detached campaign book, not owners, access, conflicts or inventory.
        actual.campaign.initialized=true;original.campaign.initialized=true;
        let delivered=prepare_with_graph(&mut actual,&requests,graph);
        let expected=fresh_permissions(||prepare(&mut original,&requests));
        assert_eq!(delivered,expected,"deliveries {case}");
        assert_eq!(crate::save(&actual),crate::save(&original),"full native state {case}");
        assert_eq!(actual.headlines,original.headlines);
        assert!(REUSED.with(Cell::get)>hits,"handoff/service requests exercise reuse {case}");
        if case==1 {assert!(!actual.logistics.usage_tonnes.is_empty(),"real capacity contention fixture");}
        let saved=crate::save(&actual);
        let hits=REUSED.with(Cell::get);
        assert_eq!(prepare(&mut actual,&requests),expected);
        assert_eq!(crate::save(&actual),saved,"same-day receipt stays inert");
        assert_eq!(REUSED.with(Cell::get),hits,"same-day receipt does not even ask for permissions");
    }
}

#[test]
fn permission_scope_matches_complete_native_days_with_original_access_reads() {
    let mut actual=graph_handoff_tests::campaign_fixture();
    let mut original=actual.clone();
    let before=REUSED.with(Cell::get);
    for day in 0..8 {
        let a=crate::tick_day(&mut actual,&[]);
        let b=fresh_permissions(||crate::tick_day(&mut original,&[]));
        assert_eq!(a,b,"returned headlines day {day}");
        assert_eq!(actual.headlines,original.headlines,"retained headlines day {day}");
        assert!(crate::save(&actual)==crate::save(&original),"complete world day {day}");
    }
    assert!(REUSED.with(Cell::get)>before,"complete days must exercise actual repeated permissions");
}

#[test]
#[ignore="explicit immutable actual-checkpoint 31-day original permission reads; no timing assertions"]
fn permission_scope_matches_actual_checkpoint_for_31_complete_days() {
    let path=std::env::var("SPHERES_S22_CHECKPOINT").expect("actual campaign checkpoint required");
    let source=std::fs::read_to_string(&path).unwrap();
    let mut value:serde_json::Value=serde_json::from_str(&source).unwrap();
    let world=if value.get("format").and_then(serde_json::Value::as_str)==Some("spheres-campaign") {
        let world=value.get_mut("world").expect("campaign world payload").take();
        drop(value);world
    } else {value};
    let mut actual=crate::load_value(world).unwrap();
    assert!(crate::campaign::enabled(&actual));
    let mut original=actual.clone();
    let before=REUSED.with(Cell::get);
    // This equivalence oracle makes no new orders or budget renewal and is
    // separate from the qualification workload and all performance claims.
    for day in 0..31 {
        let a=crate::tick_day(&mut actual,&[]);
        let b=fresh_permissions(||crate::tick_day(&mut original,&[]));
        assert_eq!(a,b,"returned headlines day {day}");
        assert_eq!(actual.headlines,original.headlines,"retained headlines day {day}");
        assert!(crate::save(&actual)==crate::save(&original),"complete native world day {day}");
    }
    let reused=REUSED.with(Cell::get)-before;
    assert!(reused>0,"actual input must exercise repeated immutable permission reads");
    assert_eq!(std::fs::read_to_string(path).unwrap(),source,"immutable source checkpoint");
    eprintln!("31 native days matched original permission reads; {reused} permission vectors reused.");
}
