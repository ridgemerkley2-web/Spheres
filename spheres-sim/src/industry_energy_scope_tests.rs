use super::*;
use std::cell::Cell;
use crate::world::{Belligerent, Conflict, Objective};

thread_local! {
    pub(super) static ORIGINAL: Cell<bool> = const { Cell::new(false) };
    pub(super) static REUSED: Cell<u64> = const { Cell::new(0) };
}
fn original<T>(run: impl FnOnce() -> T) -> T {
    struct Reset(bool);
    impl Drop for Reset { fn drop(&mut self) { ORIGINAL.with(|flag|flag.set(self.0)); } }
    let _reset=Reset(ORIGINAL.with(|flag|flag.replace(true)));
    run()
}
fn rates_bits(r: (f64,f64))->(u64,u64) { (r.0.to_bits(),r.1.to_bits()) }
fn dispatch_bits(rows:Vec<(CompanyTarget,f64,companies::CompanyModifiers)>)
    ->Vec<(CompanyTarget,u64,Option<u32>,u64,u64,u64)> {
    rows.into_iter().map(|(t,c,m)|(t,c.to_bits(),m.company_id,m.work_rate.to_bits(),
        m.input_rate.to_bits(),m.fee_rate.to_bits())).collect()
}
fn power_bits(rows:Vec<(String,f64,f64,f64)>)->Vec<(String,u64,u64,u64)> {
    rows.into_iter().map(|(d,c,i,f)|(d,c.to_bits(),i.to_bits(),f.to_bits())).collect()
}
// Literal original GDP generator read; notably its capacity multiplication
// order differs from industry::energy_dispatch for fractional modules.
fn original_power_generators(w:&WorldState,nation:NationId)->Vec<(String,f64,f64,f64)> {
    w.districts.iter().filter(|(d,owner)|**owner==nation&&!resources::district_contested(w,d))
        .filter_map(|(d,_)| {
            let level=crate::industrial_modules::effective_capacity(w,d,K::Generation);
            let operator=companies::modifiers(w,nation,&CompanyTarget::Facility {
                district:d.clone(),sector:CompanySector::Energy });
            (level>0.0).then(||(d.clone(),level*10.0*operator.work_rate,operator.input_rate,operator.fee_rate))
        }).collect()
}
fn fixture(experience:f64)->(WorldState,u32) {
    let (mut w,energy)=super::tests::s08_line_energy_world(experience);
    // The isolated-line helper installs only its department ledger. Complete
    // native days also require the annual plan owned by a real enacted budget.
    let fiscal_year=w.year;
    let allocations=w.nation(NationId::USA).budget_for(fiscal_year).allocations;
    crate::apply_command(&mut w,&crate::Command::SetProgramBudget {
        nation:NationId::USA,fiscal_year,allocations,departments:programs::default_departments(),
    }).expect("fixture enacts the complete annual and department budget");
    programs::begin_day(&mut w);
    // Synthetic fixture combines existing funded-line setup with canonical
    // new-world inherited assets. Actual-input tests perform no such setup.
    let mut opening=crate::init::world_1990(crate::world::GameRules {
        daily_simulation:true,..Default::default() });
    crate::starting_industry::enable_new_world(&mut opening).unwrap();
    w.starting_industry=opening.starting_industry;
    w.rules.industry_rebuild=true;
    crate::province_economy::enable(&mut w);
    let sites:Vec<_>=w.production.industry.sites.keys().cloned().collect();
    w.production.industry.modules.insert(sites[0].clone(),333_333);
    (w,energy)
}

#[test]
fn energy_scope_keeps_live_modifier_bits_and_rebuilds_changed_geometry() {
    let (mut w,energy)=fixture(179.5);
    let nation=NationId::USA;
    let first=w.production.industry.sites.keys().next().unwrap().clone();
    let second=w.production.industry.sites.keys().nth(1).unwrap().clone();
    for case in 0..9 {
        // Every geometry/control change occurs after dropping the prior scope;
        // the production scope never escapes one tick_day call.
        match case {
            1=>{w.production.industry.modules.insert(first.clone(),777_777);}
            2=>complete_site(&mut w,&first,K::Generation),
            3=>{w.districts.insert(second.clone(),NationId::Canada);}
            4=>{
                w.conflicts.push(Conflict {id:998,theatre:crate::war::theatre_between(&w,nation,NationId::Canada),
                    side_a:vec![nation],side_b:vec![NationId::Canada],posture:vec![
                        Belligerent::new(nation,8,Objective::Seize),Belligerent::new(NationId::Canada,8,Objective::Hold)],
                    control:0.0,months:0,quiet_months:0,frozen_since:None,start_year:w.year,start_month:w.month,
                    origin_attacker:nation,invasion_declared:true,front:BTreeMap::from([(first.clone(),0.0)]),
                    pockets:vec![],aim:None});
            }
            5=>{w.conflicts.clear();w.districts.insert(second.clone(),nation);}
            6=>w.sector_contractors.enabled=false,
            7=>{w.sector_contractors.enabled=true;w.rules.industry_rebuild=false;}
            8=>{w.rules.industry_rebuild=true;w.sector_contractors.assignments.clear();}
            _=>{}
        }
        let mut scope=OperatingEnergy::default();
        for experience in [179.5,180.0,539.5,540.0] {
            // Actual production can cross either XP threshold while the
            // layout stays fixed; no whole-day modifier/rate reuse is valid.
            w.sector_contractors.roster.iter_mut().find(|c|c.id==energy).unwrap().experience=experience;
            let before=crate::save(&w);
            assert_eq!(rates_bits(scope.rates(&w,nation)),rates_bits(energy_company_rates(&w,nation)),"rates {case}/{experience}");
            assert_eq!(dispatch_bits(scope.dispatch(&w,nation)),dispatch_bits(energy_dispatch(&w,nation)),"dispatch {case}/{experience}");
            assert_eq!(power_bits(scope.power_generators(&w,nation)),power_bits(original_power_generators(&w,nation)),"GDP {case}/{experience}");
            assert_eq!(scope.inherited(&w,nation).to_bits(),crate::industry_operations::inherited_power_headroom(&w,nation).to_bits());
            assert_eq!(crate::save(&w),before,"geometry/rate queries are pure");
        }
    }
}

#[test]
fn scoped_factory_pass_matches_original_bills_gdp_shortages_and_xp() {
    for experience in [179.5,539.5] {
        let (base,energy)=fixture(experience);
        for case in ["funded","cross_sector","missing_raw","full_storage","closed_budget"] {
            let mut actual=base.clone();
            match case {
                "cross_sector"=>actual.sector_contractors.assignments.iter_mut().find(|a|
                    a.nation==NationId::USA&&a.target.sector()==CompanySector::Manufacturing).unwrap().company_id=energy,
                "missing_raw"=>resources::set_stockpile_for_test(&mut actual,NationId::USA,C::Coal,0.0),
                "full_storage"=>{let capacity=goods_capacity(&actual,NationId::USA);
                    actual.production.industry.goods.insert(NationId::USA,Goods{intermediates:capacity,capital_goods:capacity});}
                "closed_budget"=>actual.nation_mut(NationId::USA).program_budget.as_mut().unwrap().day=None,
                _=>{}
            }
            let mut expected=actual.clone();
            let hits=REUSED.with(Cell::get);
            tick_day(&mut actual);
            original(||tick_day(&mut expected));
            assert!(crate::save(&actual)==crate::save(&expected),"full operating world {case}/{experience}");
            assert_eq!(actual.headlines,expected.headlines);
            if case=="funded"||case=="cross_sector" {
                assert!(REUSED.with(Cell::get)>hits,"must reuse real geometry");
                assert!(actual.production.industry.operations.iter().any(|o|o.output_daily>0.0));
                assert!(!actual.province_economy.as_ref().unwrap().flows.receipts.is_empty(),"GDP adapter is exercised");
            }
            let settled=crate::save(&actual);
            tick_day(&mut actual);
            assert_eq!(crate::save(&actual),settled,"replay is inert");
            let district=actual.production.industry.sites.keys().next().unwrap().clone();
            complete_site(&mut actual,&district,K::Generation);
            complete_site(&mut expected,&district,K::Generation);
            for day in 0..3 {
                let a=crate::tick_day(&mut actual,&[]);
                let b=original(||crate::tick_day(&mut expected,&[]));
                assert_eq!(a,b,"native returned headlines {case}/{day}");
                assert_eq!(actual.headlines,expected.headlines);
                assert!(crate::save(&actual)==crate::save(&expected),"native full world {case}/{day}");
            }
        }
    }
}

#[test]
#[ignore="explicit immutable actual-checkpoint 31-day original energy-layout oracle; no timing assertions"]
fn energy_scope_matches_actual_checkpoint_for_31_complete_days() {
    let path=std::env::var("SPHERES_S22_CHECKPOINT").expect("actual campaign checkpoint required");
    let source=std::fs::read_to_string(&path).unwrap();
    let mut value:serde_json::Value=serde_json::from_str(&source).unwrap();
    let world=if value.get("format").and_then(serde_json::Value::as_str)==Some("spheres-campaign") {
        let world=value.get_mut("world").expect("campaign world payload").take();drop(value);world
    } else {value};
    let mut actual=crate::load_value(world).unwrap();
    assert!(actual.rules.industry_rebuild&&actual.rules.production_system&&actual.rules.resource_market);
    let mut expected=actual.clone();
    let hits=REUSED.with(Cell::get);
    for day in 0..31 {
        let a=crate::tick_day(&mut actual,&[]);
        let b=original(||crate::tick_day(&mut expected,&[]));
        assert_eq!(a,b,"returned headlines day {day}");
        assert_eq!(actual.headlines,expected.headlines,"retained headlines day {day}");
        assert!(crate::save(&actual)==crate::save(&expected),"complete native world day {day}");
    }
    let reused=REUSED.with(Cell::get)-hits;
    assert!(reused>0,"actual input must exercise repeated immutable energy geometry reads");
    assert_eq!(std::fs::read_to_string(path).unwrap(),source,"immutable source checkpoint");
    eprintln!("31 native days matched original energy layout reads; {reused} geometry/headroom reads reused; no timing claim.");
}
