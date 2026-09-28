use super::*;
use std::cell::Cell;

thread_local! {
    pub(super) static ORIGINAL_CONTRACT_ROUTES: Cell<bool> = const { Cell::new(false) };
    pub(super) static REUSED_CONTRACT_FREIGHT: Cell<u64> = const { Cell::new(0) };
}

fn original_routes<T>(run: impl FnOnce() -> T) -> T {
    struct Reset(bool);
    impl Drop for Reset {
        fn drop(&mut self) { ORIGINAL_CONTRACT_ROUTES.with(|flag| flag.set(self.0)); }
    }
    let _reset = Reset(ORIGINAL_CONTRACT_ROUTES.with(|flag| flag.replace(true)));
    run()
}

fn world() -> WorldState {
    let mut w = crate::init::world_1990(crate::world::GameRules {
        daily_simulation: true, resource_gates: true, resource_market: true,
        logistics_routes: true, physical_logistics: true, military_operations: true,
        ..Default::default()
    });
    w.conflicts.clear(); w.sanctions.clear();
    w
}

fn compare(routes: &mut ContractDispatchRoutes, actual: &mut WorldState, original: &mut WorldState,
    legs: &[(NationId, NationId, Commodity, f64)], stock: f64, id: u32,
) -> Vec<Dispatch> {
    let a = routes.dispatch(actual, legs, stock, id);
    let b = dispatch_bundle(original, legs, stock, id);
    assert_eq!(a.0.to_bits(), b.0.to_bits(), "contract {id}: exact service fraction");
    assert_eq!(a.1, b.1, "contract {id}: exact routes, strings, quantities and order");
    assert!(crate::save(actual) == crate::save(original), "contract {id}: complete world parity");
    assert_eq!(actual.headlines, original.headlines);
    let nominal = routes.nominal.as_ref().unwrap();
    assert!(nominal.capacities.is_empty(), "contract reuse cannot freeze terminal capacities");
    assert_eq!(nominal.search_nodes_left, 32_768, "contract alternates cannot borrow spot search budget");
    assert!(nominal.assembly_segments.iter().enumerate().all(|(i, row)|
        row.is_none() || network().edges[i].kind != "terminal"));
    a.1
}

#[test]
fn contract_nominal_reuse_preserves_barters_zero_invalid_and_congested_dispatches() {
    for policy in [RoutePolicy::Fastest, RoutePolicy::LandOnly, RoutePolicy::AvoidChokepoints] {
        let mut a = world();
        let (seller, buyer) = (NationId::Germany, NationId::France);
        set_policy(&mut a, buyer, policy).unwrap();
        begin_month(&mut a);
        // Make the first nominal route full, so the existing alternate search
        // and shared edge accounting are exercised rather than bypassed.
        let nominal = plan(&a, seller, buyer).unwrap();
        for segment in &nominal.segments {
            let capacity=route_segment_capacity(&a,segment).unwrap();
            a.logistics.usage_tonnes.insert(segment.clone(),capacity);
        }
        let mut b = a.clone();
        let mut routes = ContractDispatchRoutes::default();
        let mut alternate = false;
        for (id, legs, stock) in [
            (1, vec![(seller,buyer,Commodity::Iron,0.0)], 1.0),
            (2, vec![(seller,buyer,Commodity::Iron,1.0), (seller,buyer,Commodity::Copper,0.5)], 1.0),
            (3, vec![(seller,buyer,Commodity::Iron,1_000_000.0)], 0.75),
            (4, vec![(seller,buyer,Commodity::Coal,5.0), (buyer,seller,Commodity::Copper,0.5)], 0.4),
            (5, vec![(seller,NationId::Italy,Commodity::Iron,2.0)], 1.0),
            (6, vec![(seller,buyer,Commodity::Copper,1.0)], 0.0),
            (7, vec![(seller,buyer,Commodity::Copper,-1.0)], 1.0),
            (8, vec![(seller,buyer,Commodity::Copper,f64::NAN)], 1.0),
            (9, vec![(seller,seller,Commodity::Copper,1.0)], 1.0),
        ] {
            let result = compare(&mut routes, &mut a, &mut b, &legs, stock, id);
            alternate |= result.iter().any(|d| d.route.as_ref().is_some_and(|r| r.dispatch_note.is_some()));
        }
        assert!(alternate, "fixture must exercise actual alternate route selection");
        assert!(!routes.nominal.as_ref().unwrap().trees.is_empty());
    }
}

#[test]
fn contract_nominal_reuse_reads_paid_terminal_experience_after_each_bundle() {
    use crate::sector_contractors::{self as companies, CompanySector, CompanyTarget};
    for boundary in [180.0, 540.0] {
        let mut a = world(); companies::enable(&mut a);
        let (seller,buyer) = (NationId::Japan,NationId::USA);
        let route = plan(&a,seller,buyer).unwrap();
        let edge_id = *route.segments.iter().filter_map(|key| network().edge_index.get(key))
            .find(|&&i| {
                let edge = &network().edges[i];
                edge.kind == "terminal" && [&edge.a,&edge.b].iter()
                    .any(|district| a.districts.get(*district) == Some(&buyer))
            }).unwrap();
        let edge = &network().edges[edge_id];
        let district = [&edge.a,&edge.b].into_iter()
            .find(|district| a.districts.get(*district) == Some(&buyer)).unwrap().clone();
        production::complete_capability(&mut a,&district,production::ProjectKind::FreightTerminal);
        let company_id = a.sector_contractors.roster.iter()
            .find(|c| c.nation == buyer && c.sector == CompanySector::Logistics).unwrap().id;
        companies::assign(&mut a,buyer,company_id,CompanyTarget::Facility {
            district, sector:CompanySector::Logistics,
        }).unwrap();
        // Disclosed synthetic XP boundary. Ordinary paid cargo must cross it.
        let company = a.sector_contractors.roster.iter_mut().find(|c|c.id==company_id).unwrap();
        company.experience=boundary-0.5; company.experience_day=None; company.experience_today=0.0;
        company.work_bonus=0.20; company.fee_rate=0.02;
        a.nation_mut(buyer).treasury_bn=Some(100.0); a.nation_mut(buyer).debt_bn=Some(0.0);
        begin_month(&mut a);
        let capacity_before=segment_capacity(&a,edge).0;
        let mut b=a.clone(); let mut routes=ContractDispatchRoutes::default();
        let first=compare(&mut routes,&mut a,&mut b,&[(seller,buyer,Commodity::Iron,100.0)],1.0,10);
        assert!(first[0].quantity>0.5);
        let company=a.sector_contractors.roster.iter().find(|c|c.id==company_id).unwrap();
        assert!(company.experience>=boundary && company.total_fees_bn>0.0);
        assert!(segment_capacity(&a,edge).0>capacity_before);
        // Reassembly must observe the newly earned capacity instead of the
        // nominal RoutePlan retained before the first cargo crossed the port.
        assert_eq!(routes.nominal.as_mut().unwrap().plan(&a,seller,buyer),plan(&b,seller,buyer));
        compare(&mut routes,&mut a,&mut b,&[(seller,buyer,Commodity::Copper,100.0)],1.0,11);
        compare(&mut routes,&mut a,&mut b,&[(seller,buyer,Commodity::Coal,100.0)],1.0,12);
    }
}

#[test]
fn contract_posting_rebuilds_context_each_day_and_keeps_full_ledger_identical() {
    let mut a=world();
    // Synthetic contracts, preserved in their original order. Physical goods
    // and counterpayments still use the real production/dispatch/ledger path.
    for id in 1..=12 {
        a.resources.contracts.push(resources::Contract {
            id, from:NationId::USA, to:if id<=8 {NationId::Pakistan} else {NationId::France},
            give:vec![resources::Leg::Commodity {c:Commodity::Copper,per_month:10.0},
                resources::Leg::Commodity {c:Commodity::Iron,per_month:3.0}],
            take:vec![resources::Leg::Money {bn_per_year:0.001}],
            months_left:24, months_total:24, days_left:Some(730), since:0, depth:0.0,
        });
    }
    a.resources.next_id=13;
    let district=a.districts.iter().find(|(_,owner)|**owner==NationId::France).unwrap().0.clone();
    let mut b=a.clone();
    let before=REUSED_CONTRACT_FREIGHT.with(|count|count.get());
    for day in 0..6 {
        for w in [&mut a,&mut b] {
            match day {
                1=>w.sanctions.push((NationId::USA,NationId::Pakistan)),
                2=>{w.sanctions.clear();set_policy(w,NationId::Pakistan,RoutePolicy::LandOnly).unwrap();},
                3=>{set_policy(w,NationId::Pakistan,RoutePolicy::Fastest).unwrap();w.nation_mut(NationId::France).alive=false;},
                4=>{w.nation_mut(NationId::France).alive=true;w.districts.insert(district.clone(),NationId::Germany);w.districts_epoch+=1;},
                5=>{w.districts.insert(district.clone(),NationId::France);w.districts_epoch+=1;
                    production::complete_capability(w,&district,production::ProjectKind::Infrastructure);
                    w.month=2;w.day=1;},
                _=>{},
            }
        }
        resources::tick(&mut a);
        original_routes(||resources::tick(&mut b));
        assert!(crate::save(&a)==crate::save(&b),"full resource posting after context invalidation day {day}");
        assert_eq!(a.headlines,b.headlines);
        crate::clock::advance_date(&mut a);crate::clock::advance_date(&mut b);
    }
    assert!(REUSED_CONTRACT_FREIGHT.with(|count|count.get())>before+1,
        "fixture must actually dispatch multiple real contract freight legs");
}

#[test]
#[ignore="explicit actual-checkpoint original-contract routing parity; no timing assertions"]
fn contract_nominal_reuse_matches_actual_checkpoint_for_31_complete_days() {
    let path=std::env::var("SPHERES_S22_CHECKPOINT").expect("actual checkpoint path required");
    let source=std::fs::read_to_string(&path).unwrap();
    let mut actual=crate::load(&source).unwrap();
    assert!(enabled(&actual) && actual.rules.military_operations);
    assert!(actual.resources.contracts.len()>1,"actual checkpoint must retain recurring contracts");
    let before=REUSED_CONTRACT_FREIGHT.with(|count|count.get());
    let mut original=actual.clone();
    // Complete native ticks, without authored state or added player budget
    // orders. This is exact-equivalence evidence, not web timing qualification.
    for day in 0..31 {
        let a=crate::tick_day(&mut actual,&[]);
        let b=original_routes(||crate::tick_day(&mut original,&[]));
        assert_eq!(a,b,"returned headlines day {day}");
        assert_eq!(actual.headlines,original.headlines,"retained headlines day {day}");
        assert!(crate::save(&actual)==crate::save(&original),"complete native world day {day}");
    }
    let dispatched=REUSED_CONTRACT_FREIGHT.with(|count|count.get())-before;
    assert!(dispatched>1,"actual checkpoint must exercise multiple reused contract freight legs");
    assert_eq!(std::fs::read_to_string(path).unwrap(),source,"source checkpoint remains immutable");
    eprintln!("31 complete native days matched the original per-leg router; {dispatched} nonzero contract dispatches exercised the shared nominal search.");
}
