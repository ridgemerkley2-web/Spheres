//! S03 property boundaries; synthetic funds isolate paid work from calibration.
use spheres_sim::{apply_command, clock, company_network, connected_economy, industry,
    init::world_1990, load, production::{self,ProjectKind as K}, programs, save,
    sector_contractors::{self as contractors,CompanySector as S,CompanyTarget as T}, tick_day, Command};
use spheres_sim::world::{GameRules,NationId as N,WorldState};
const HOME:N=N::France;
fn near(a:f64,b:f64){assert!((a-b).abs()<1e-10,"{a:.12} != {b:.12}");}
fn fixture()->(WorldState,String){
    let mut w=world_1990(GameRules{daily_simulation:true,production_system:true,
        manufacturing_system:true,resource_market:true,resource_gates:true,ai_aggression:0.0,crisis_intensity:0.0,..Default::default()});
    w.player=Some(HOME);w.nation_mut(HOME).political_capital=1000.0;
    programs::set_construction_budget(&mut w,HOME,0.001).unwrap();
    connected_economy::enable(&mut w).unwrap();
    spheres_sim::resources::tick(&mut w);
    let d=w.districts.iter().find(|(_,n)|**n==HOME).unwrap().0.clone();(w,d)
}
fn enroll(w:&mut WorldState){apply_command(w,&Command::EnableCompanies{nation:HOME}).unwrap();}
fn roundtrip(w:&WorldState)->WorldState{
    let once=save(w);let resumed=load(&once).unwrap();
    assert!(save(&resumed)==once,"first roundtrip changed property");
    let twice=load(&save(&resumed)).unwrap();assert!(save(&twice)==once,"second roundtrip changed property");twice
}
fn assign(w:&mut WorldState,project:u32)->u32{
    let id=w.sector_contractors.roster.iter().find(|c|c.nation==HOME&&c.sector==S::Construction&&c.work_bonus>0.0).unwrap().id;
    let target=T::Construction{project};let q=contractors::assignment_quote(w,HOME,id,&target);
    apply_command(w,&Command::AssignSectorContractor{nation:HOME,company:id,target,quote:q.token}).unwrap();id
}
#[test]
fn adoption_is_explicit_atomic_and_never_issues_supplier_property(){
    let mut old=world_1990(GameRules::default());let saved=save(&old);
    assert!(company_network::enable(&mut old).is_err());assert_eq!(save(&old),saved);
    contractors::tick_day(&mut old);assert_eq!(save(&old),saved);
    let(mut w,d)=fixture();let id=production::start_project(&mut w,HOME,&d,K::Infrastructure).unwrap();
    tick_day(&mut w,&[]);let p=w.production.projects.clone();let accounts=w.nation(HOME).program_budget.clone();
    let money=(w.nation(HOME).treasury_bn,w.nation(HOME).debt_bn);
    enroll(&mut w);assert_eq!(serde_json::to_value(&w.production.projects).unwrap(),serde_json::to_value(p).unwrap());
    assert_eq!(serde_json::to_value(&w.nation(HOME).program_budget).unwrap(),serde_json::to_value(accounts).unwrap());
    assert_eq!((w.nation(HOME).treasury_bn,w.nation(HOME).debt_bn),money);
    assert!(w.companies.is_empty());assert!(w.production.projects.iter().any(|p|p.id==id));
    let once=save(&w);enroll(&mut w);assert_eq!(save(&w),once);roundtrip(&w);
    assert!(apply_command(&mut w,&Command::EnableCompanies{nation:N::Japan}).is_err());
    assert_eq!(save(&w),once);
}
#[test]
fn construction_pays_separate_service_fees_within_one_financial_budget(){
    let(mut w,d)=fixture();enroll(&mut w);
    let ids=[K::Infrastructure,K::Warehouse].map(|k|production::start_project(&mut w,HOME,&d,k).unwrap());
    let company=assign(&mut w,ids[0]);programs::begin_day(&mut w);
    let raw=serde_json::to_value(&w.resources).unwrap();let goods=w.production.industry.goods.clone();
    let available=programs::construction_available_bn(&w,HOME);
    let before=w.nation(HOME).program_budget.as_ref().unwrap().spent_today_bn.iter().flatten().sum::<f64>();
    let plans=industry::project_plans(&w);assert!(plans[&ids[0]].company_fee_bn>0.0);
    assert!(plans.values().map(|p|p.cash_bn).sum::<f64>()<=available+1e-12);
    for p in plans.values(){assert_eq!(p.required,[0.0;12]);assert_eq!(p.goods,industry::Goods::default());}
    production::tick_day(&mut w);
    let after=w.nation(HOME).program_budget.as_ref().unwrap().spent_today_bn.iter().flatten().sum::<f64>();
    near(after-before,plans.values().map(|p|p.cash_bn).sum());
    for id in ids{let f=&w.production.industry.projects[&id];near(f.spent_bn,plans[&id].base_cash_bn);near(f.company_fees_bn,plans[&id].company_fee_bn);}
    near(w.sector_contractors.roster.iter().find(|c|c.id==company).unwrap().total_fees_bn,plans[&ids[0]].company_fee_bn);
    assert_eq!(serde_json::to_value(&w.resources).unwrap(),raw);assert_eq!(w.production.industry.goods,goods);
    let once=save(&w);production::tick_day(&mut w);assert_eq!(save(&w),once);roundtrip(&w);
}
#[test]
fn original_master_roster_migrates_without_enable_settlement_or_identity_change(){
    let(mut w,d)=fixture();contractors::enable(&mut w);
    let id=production::start_project(&mut w,HOME,&d,K::Infrastructure).unwrap();assign(&mut w,id);
    programs::begin_day(&mut w);production::tick_day(&mut w);
    // The original master had no separate financial service receipt field;
    // existing paid construction dollars remain authoritative, without repricing.
    let fees=w.production.industry.projects[&id].company_fees_bn;
    let f=w.production.industry.projects.get_mut(&id).unwrap();f.spent_bn+=fees;f.company_fees_bn=0.0;f.contract_cost_bn=None;
    let expected_roster=serde_json::to_value(&w.sector_contractors).unwrap();
    let mut original=serde_json::to_value(&w).unwrap();
    original.as_object_mut().unwrap().remove("sector_contractors");original["companies"]=expected_roster.clone();
    let adopted=load(&original.to_string()).unwrap();
    assert_eq!(serde_json::to_value(&adopted.sector_contractors).unwrap(),expected_roster);
    assert!(adopted.companies.is_empty());assert!(adopted.supplier_operations.is_empty());
    assert_eq!(serde_json::to_value(&adopted).unwrap(),serde_json::to_value(&w).unwrap());
    let mut a=adopted;let mut b=roundtrip(&a);
    tick_day(&mut a,&[]);tick_day(&mut b,&[]);assert!(save(&a)==save(&b));
    let mut mixed=original.clone();mixed["companies"]["firms"]=serde_json::json!([]);assert!(load(&mixed.to_string()).is_err());
    let mut nested=original.clone();nested["companies"]["roster"][0]["lost_property"]=1.into();assert!(load(&nested.to_string()).is_err());
}
#[test]
fn combined_capability_cannot_be_downgraded_or_mislabeled(){
    let(mut w,_)=fixture();enroll(&mut w);let v:serde_json::Value=serde_json::from_str(&save(&w)).unwrap();
    assert_eq!(v["format"],"spheres-companies-save");assert_eq!(v["economy_version"],1);
    assert!(load(&v["world"].to_string()).is_err());
    for field in ["version","equipment_version","party_leadership_version","economy_version","supplier_operations_version"]{
        let mut bad=v.clone();bad[field]=99.into();assert!(load(&bad.to_string()).is_err(),"{field}");
    }
    let mut bad=v.clone();bad["format"]="spheres-economy-save".into();assert!(load(&bad.to_string()).is_err());
    let mut bad=v;bad["world"]["campaign"]=serde_json::json!({"armies":[]});assert!(load(&bad.to_string()).is_err());
}
#[test]
fn reviewed_service_assignment_refuses_stale_state_without_moving_money(){
    let(mut w,d)=fixture();enroll(&mut w);let p=production::start_project(&mut w,HOME,&d,K::Infrastructure).unwrap();
    let c=w.sector_contractors.roster.iter().find(|c|c.nation==HOME&&c.sector==S::Construction).unwrap().id;
    let target=T::Construction{project:p};let quote=contractors::assignment_quote(&w,HOME,c,&target);
    clock::advance_date(&mut w);let before=save(&w);
    assert!(apply_command(&mut w,&Command::AssignSectorContractor{nation:HOME,company:c,target,quote:quote.token}).is_err());
    assert_eq!(save(&w),before);
}
