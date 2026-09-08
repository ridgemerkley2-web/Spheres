use crate::{construction_capacity as capacity, construction_presets, industry, production::{self,ProjectKind as K},
    resources::{self,Commodity as C}, world::{GameRules,NationId,WorldState}, programs};
const USA:NationId=NationId::USA;
fn fixture()->(WorldState,String) {
    let mut w=crate::init::world_1990(GameRules { daily_simulation:true,production_system:true,resource_market:true,industry_rebuild:true,..Default::default() });
    w.player=Some(USA);
    let d=w.districts.iter().find(|(_,n)|**n==USA).unwrap().0.clone();
    programs::set_construction_budget(&mut w,USA,1.0).unwrap();
    programs::begin_day(&mut w);
    (w,d)
}
fn fill(w:&mut WorldState) {
    for c in resources::ALL { if c!=C::Oil { resources::set_stockpile_for_test(w,USA,c,1000.0); } }
}
#[test]
fn preset_is_four_ordinary_jobs_and_refusal_is_atomic() {
    let (mut w,d)=fixture();
    let before=crate::save(&w);
    let quote=construction_presets::preview(&w,USA,&d);
    assert!(quote["can_start"].as_bool().unwrap());
    assert_eq!(before,crate::save(&w));
    let ids=construction_presets::start(&mut w,USA,&d).unwrap();
    assert_eq!(ids.len(),4);
    for k in construction_presets::INDUSTRIAL_STARTER {
        assert_eq!(production::projects_for(&w,USA).filter(|p|p.kind==k).count(),1);
    }
    let queued=crate::save(&w);
    assert!(construction_presets::start(&mut w,USA,&d).is_err());
    assert_eq!(queued,crate::save(&w));
}
#[test]
fn no_iron_pauses_without_spending_or_consuming_other_inputs() {
    let (mut w,d)=fixture();fill(&mut w);
    let iron=resources::stockpile(&w,USA,C::Iron);
    let mut drain=[0.0;12];drain[C::Iron.idx()]=iron;
    resources::consume_stockpile_atomic(&mut w,USA,&drain).unwrap();
    let id=production::start_project(&mut w,USA,&d,K::CivilianIndustry).unwrap();
    let copper=resources::stockpile(&w,USA,C::Copper);
    let cash=programs::construction_available_bn(&w,USA);
    production::tick_day(&mut w);
    let p=w.production.projects.iter().find(|p|p.id==id).unwrap();
    assert_eq!(p.progress_days,0.0);
    assert!(p.reason.as_ref().unwrap().contains("Iron") || p.reason.as_ref().unwrap().contains("iron"));
    assert_eq!(cash,programs::construction_available_bn(&w,USA));
    assert_eq!(copper,resources::stockpile(&w,USA,C::Copper));
}
#[test]
fn prepared_materials_replace_raw_draw_once_and_save_replay_matches() {
    let (mut w,d)=fixture();fill(&mut w);
    let mut drain=[0.0;12];drain[C::Iron.idx()]=resources::stockpile(&w,USA,C::Iron);
    resources::consume_stockpile_atomic(&mut w,USA,&drain).unwrap();
    w.production.industry.goods.entry(USA).or_default().intermediates=5.0;
    let id=production::start_project(&mut w,USA,&d,K::CivilianIndustry).unwrap();
    capacity::set_assignment(&mut w,USA,id,Some(10.0)).unwrap();
    let plan=industry::project_plans(&w).remove(&id).unwrap();
    assert!(plan.advance_days>0.0 && plan.goods.intermediates>0.0);
    assert_eq!(plan.required[C::Iron.idx()],0.0);
    let mut restored=crate::load(&crate::save(&w)).unwrap();
    production::tick_day(&mut w);production::tick_day(&mut restored);
    assert_eq!(crate::save(&w),crate::save(&restored));
    assert!((w.production.industry.goods[&USA].intermediates-(5.0-plan.goods.intermediates)).abs()<1e-9);
    let paid=crate::save(&w);production::tick_day(&mut w);assert_eq!(paid,crate::save(&w));
}
