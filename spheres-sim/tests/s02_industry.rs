//! S02 ownership and conservation fixtures. Synthetic stocks and appropriations
//! isolate the integration boundary; these are not historical calibration tests.
use spheres_sim::{arsenal, clock, commerce, companies, construction_preview, industry,
    industry_operations as operations, init::world_1990, manufacturing, population,
    production::{self, ProjectKind as K, ProvinceCapabilities}, programs, resources,
    save, load};
use spheres_sim::world::{GameRules, NationId as N, WorldState, BUDGET_DEFENSE, BUDGET_INDUSTRY};
use resources::Commodity as C;
const HOME:N=N::France;
fn near(a:f64,b:f64) { assert!((a-b).abs()<1e-9,"{a:.12} != {b:.12}"); }
fn fixture() -> (WorldState,String) {
    let mut w=world_1990(GameRules {daily_simulation:true,production_system:true,
        manufacturing_system:true,military_operations:true,resource_gates:true,resource_market:true,
        industry_rebuild:true,ai_aggression:0.0,crisis_intensity:0.0,..Default::default()});
    w.player=Some(HOME);
    let n=w.nation_mut(HOME);n.political_capital=1000.0;n.treasury_bn=Some(20.0);n.debt_bn=Some(0.0);n.debt_gdp=0.0;
    programs::set_construction_budget(&mut w,HOME,1.0).unwrap();
    resources::tick(&mut w);
    programs::begin_day(&mut w);
    let d=w.districts.iter().find(|(_,n)|**n==HOME).unwrap().0.clone();
    let budget=w.nation_mut(HOME).program_budget.as_mut().unwrap();
    budget.available_bn[BUDGET_INDUSTRY][0]=10.0;
    budget.available_bn[BUDGET_INDUSTRY][1]=10.0;
    budget.available_bn[BUDGET_INDUSTRY][2]=10.0;
    budget.available_bn[BUDGET_DEFENSE][3]=10.0;
    stock(&mut w,10.0);
    (w,d)
}
fn stock(w:&mut WorldState,quantity:f64) {
    let market=w.resources.market.as_mut().expect("explicit physical-market fixture");
    market.stocks.retain(|s|s.nation!=HOME);
    for c in resources::ALL {market.stocks.push(resources::Stock {nation:HOME,commodity:c,quantity,reserve_target:0.0});}
    market.stocks.sort_by_key(|s|(s.nation,s.commodity));
}
fn raw(w:&WorldState)->[f64;12] {std::array::from_fn(|i|resources::stockpile(w,HOME,C::from_idx(i).unwrap()))}
fn site(w:&mut WorldState,d:&str,arms:u8,levels:[u8;3]) {
    w.production.provinces.retain(|p|p.district!=d);
    w.production.provinces.push(ProvinceCapabilities {district:d.into(),infrastructure:0,
        civilian_industry:0,power_grid:2,research_centers:0,arms_plants:arms});
    w.production.provinces.sort_by(|a,b|a.district.cmp(&b.district));
    w.production.industry.sites.insert(d.into(),[0,1,0,0,0,0,0]);
    w.production.rebuild_sites.insert(d.into(),levels);
    w.production.industry.goods.insert(HOME,industry::Goods {intermediates:10.0,capital_goods:10.0});
}
fn spent(w:&WorldState)->f64 {w.nation(HOME).program_budget.as_ref().unwrap().spent_today_bn.iter().flatten().sum()}

#[test]
fn appended_catalogue_preserves_all_legacy_keys_and_default_inertia() {
    let legacy=["infrastructure","civilian_industry","power_grid","research_center","arms_plant",
        "machinery_works","generation","processing_plant","freight_terminal","warehouse","automation","efficiency","starter_industry"];
    assert_eq!(production::PROJECT_KINDS[..13].iter().map(|k|k.key()).collect::<Vec<_>>(),legacy);
    let mut w=world_1990(GameRules::default());let before=save(&w);
    operations::begin_day(&mut w);operations::tick_day(&mut w);
    assert_eq!(save(&w),before);assert_eq!(commerce::supported_goods(&w).len(),2);
    let (mut w,d)=fixture();w.rules.industry_rebuild=false;
    for k in [K::OfficeDistrict,K::Shipyard,K::AdvancedIndustry] {
        assert!(production::start_project_error(&w,HOME,&d,k).is_some());
    }
}

#[test]
fn funded_construction_ignores_empty_raw_goods_power_and_workforce() {
    let (mut w,d)=fixture();stock(&mut w,0.0);
    population::enable(&mut w).unwrap();
    for p in w.population_system.provinces.values_mut() {p.filled=[[0.0;3];8];}
    let ids=[K::OfficeDistrict,K::AdvancedIndustry,K::Warehouse].map(|k|production::start_project(&mut w,HOME,&d,k).unwrap());
    let before_raw=raw(&w);let before_goods=w.production.industry.goods.clone();
    let plans=industry::project_plans(&w);let before_spent=spent(&w);
    for id in ids {let p=&plans[&id];assert!(p.advance_days>0.0);assert_eq!(p.required,[0.0;12]);assert_eq!(p.goods,industry::Goods::default());}
    production::tick_day(&mut w);
    for id in ids {near(w.production.projects.iter().find(|p|p.id==id).unwrap().progress_days,plans[&id].advance_days);}
    near(spent(&w)-before_spent,plans.values().map(|p|p.cash_bn).sum());
    assert_eq!(raw(&w),before_raw);assert_eq!(w.production.industry.goods,before_goods);
    let after=save(&w);production::tick_day(&mut w);assert_eq!(save(&w),after);
}

#[test]
fn zero_budget_preserves_frozen_contract_and_historical_input_receipts() {
    let (mut w,d)=fixture();let id=production::start_project(&mut w,HOME,&d,K::Infrastructure).unwrap();
    production::tick_day(&mut w);
    w.production.projects.iter_mut().find(|p|p.id==id).unwrap().resources_used[C::Iron.idx()]=3.0;
    let paid=w.production.industry.projects[&id].clone();let progress=w.production.projects[0].progress_days;
    assert!(progress>0.0 && paid.spent_bn>0.0);
    clock::advance_date(&mut w);programs::set_construction_budget(&mut w,HOME,0.0).unwrap();
    production::tick_day(&mut w);
    let p=&w.production.projects[0];assert_eq!(p.progress_days,progress);assert_eq!(p.resources_used[C::Iron.idx()],3.0);
    let f=&w.production.industry.projects[&id];assert_eq!(f.spent_bn,paid.spent_bn);assert_eq!(f.contract_cost_bn,paid.contract_cost_bn);
}

#[test]
fn commissioned_operations_consume_real_complete_bundles_once_and_survive_save() {
    let (mut w,d)=fixture();site(&mut w,&d,0,[1,0,1]);
    spheres_sim::province_economy::enable(&mut w);
    let before_raw=raw(&w);let before_spent=spent(&w);let treasury=w.nation(HOME).treasury_bn;
    operations::begin_day(&mut w);operations::tick_day(&mut w);
    near(operations::advanced_component_stock(&w,HOME),0.2);
    near(w.production.industry.goods[&HOME].intermediates,9.59);
    near(before_raw[C::Copper.idx()]-raw(&w)[C::Copper.idx()],0.02);
    near(before_raw[C::RareEarths.idx()]-raw(&w)[C::RareEarths.idx()],0.002);
    near(before_raw[C::Coal.idx()]-raw(&w)[C::Coal.idx()],0.021);
    near(spent(&w)-before_spent,2.0*operations::OPERATING_CASH_LEVEL_DAY_BN+1.05*operations::ENERGY_CASH_POWER_DAY_BN);
    assert_eq!(w.nation(HOME).treasury_bn,treasury,"fiscal settlement alone owns public cash");
    assert_eq!(w.production.operations.receipts.len(),2);operations::validate(&w).unwrap();
    let receipts=&w.province_economy.as_ref().unwrap().flows.receipts;
    assert!(receipts.contains_key(&format!("site:{d}:office_district")));
    assert!(receipts.contains_key(&format!("site:{d}:advanced_industry")));
    let once=save(&w);operations::begin_day(&mut w);operations::tick_day(&mut w);assert_eq!(save(&w),once);
    let mut resumed=load(&once).unwrap();operations::tick_day(&mut resumed);assert!(save(&resumed)==once,"save/load and same-date operations must preserve every serialized byte");
}

#[test]
fn missing_rare_earths_never_consumes_an_incomplete_operating_bundle() {
    let (mut w,d)=fixture();site(&mut w,&d,0,[0,0,1]);
    spheres_sim::province_economy::enable(&mut w);
    w.resources.market.as_mut().unwrap().stocks.iter_mut().find(|s|s.nation==HOME&&s.commodity==C::RareEarths).unwrap().quantity=0.0;
    let before_raw=raw(&w);let goods=w.production.industry.goods.clone();let budget=spent(&w);
    operations::tick_day(&mut w);
    assert_eq!(raw(&w),before_raw);assert_eq!(w.production.industry.goods,goods);assert_eq!(spent(&w),budget);
    assert_eq!(operations::advanced_component_stock(&w,HOME),0.0);
    assert_eq!(w.production.operations.receipts[0].status,"blocked");operations::validate(&w).unwrap();
    assert!(!w.province_economy.as_ref().unwrap().flows.receipts.contains_key(&format!("site:{d}:advanced_industry")));
}

#[test]
fn qualified_worker_shortage_caps_operations_without_inventing_employment() {
    let (mut w,d)=fixture();site(&mut w,&d,0,[0,0,1]);population::enable(&mut w).unwrap();
    w.population_system.provinces.get_mut(&d).unwrap().filled=[[0.0;3];8];
    let people=w.population_system.clone();let before_raw=raw(&w);let budget=spent(&w);
    operations::tick_day(&mut w);
    assert_eq!(operations::advanced_component_stock(&w,HOME),0.0);assert_eq!(spent(&w),budget);assert_eq!(raw(&w),before_raw);
    assert_eq!(w.production.operations.receipts[0].jobs_filled,0.0);
    assert_eq!(serde_json::to_value(&w.population_system).unwrap(),serde_json::to_value(&people).unwrap());
}

#[test]
fn old_naval_line_and_company_lease_keep_their_arms_plant_entitlements() {
    let (mut w,d)=fixture();site(&mut w,&d,2,[0,0,0]);w.production.rebuild_sites.clear();w.rules.industry_rebuild=false;
    let legacy=manufacturing::start_line(&mut w,HOME,&d,"nav_patrol").unwrap();
    let q=companies::establishment_quote(&w,HOME,"Synthetic supplier",&d,0.01);assert!(q.valid,"{:?}",q.reason);
    companies::apply(&mut w,HOME,&companies::CompanyOrder::Establish{name:"Synthetic supplier".into(),district:d.clone(),capitalization_bn:0.01,quote:q.token}).unwrap();
    assert_eq!(companies::reserved_slots(&w,HOME,&d),1);assert_eq!(manufacturing::used_slots(&w,HOME,&d),2);
    let opening=w.manufacturing.lines.clone();w.rules.industry_rebuild=true;
    assert_eq!(w.manufacturing.lines,opening);assert!(!w.manufacturing.shipyard_lines.contains(&legacy));
    assert!(manufacturing::line_blocker(&w,&opening[0]).is_none());
    w.production.rebuild_sites.insert(d.clone(),[0,1,0]);
    let dock=manufacturing::start_line(&mut w,HOME,&d,"nav_escort").unwrap();
    assert!(w.manufacturing.shipyard_lines.contains(&dock));assert_eq!(manufacturing::used_slots(&w,HOME,&d),2);
    assert_eq!(manufacturing::used_naval_slots(&w,HOME,&d),1);
    let paid_orders=w.nation(HOME).arsenal.orders.clone();manufacturing::stop_line(&mut w,HOME,dock).unwrap();
    assert_eq!(serde_json::to_value(&w.nation(HOME).arsenal.orders).unwrap(),serde_json::to_value(&paid_orders).unwrap());assert!(w.manufacturing.shipyard_lines.is_empty());
    assert_eq!(companies::reserved_slots(&w,HOME,&d),1);assert_eq!(manufacturing::used_slots(&w,HOME,&d),2);
}

#[test]
fn dock_without_operating_inputs_cannot_spend_or_order_even_with_equipment_raw_inputs() {
    let (mut w,d)=fixture();site(&mut w,&d,0,[0,1,0]);
    let line=manufacturing::start_line(&mut w,HOME,&d,"nav_patrol").unwrap();
    // There is equipment steel, but the dock has no qualified operating staff.
    population::enable(&mut w).unwrap();w.population_system.provinces.get_mut(&d).unwrap().filled=[[0.0;3];8];
    operations::begin_day(&mut w);let before=spent(&w);let orders=w.nation(HOME).arsenal.orders.clone();
    arsenal::tick(&mut w);
    let row=w.manufacturing.lines.iter().find(|l|l.id==line).unwrap();
    assert_eq!(row.ordered_today_bn,0.0);assert_eq!(row.throughput_today,0.0);
    assert_eq!(serde_json::to_value(&w.nation(HOME).arsenal.orders).unwrap(),serde_json::to_value(&orders).unwrap());assert_eq!(spent(&w),before);
}

#[test]
fn new_sparse_save_records_cannot_disappear_or_hide_behind_disabled_rules() {
    let (mut w,_)=fixture();w.production=Default::default();
    w.production.operations.advanced_components.insert(HOME,0.25);
    assert!(!w.production.is_empty());let text=save(&w);let restored=load(&text).unwrap();
    assert_eq!(operations::advanced_component_stock(&restored,HOME),0.25);
    w.rules.industry_rebuild=false;assert!(operations::validate(&w).is_err());w.rules.industry_rebuild=true;
    w.production.operations.support_day=Some(clock::absolute_day(&w)+1);assert!(operations::validate(&w).is_err());
    w.production.operations.support_day=None;w.production.operations.advanced_components.insert(HOME,f64::NAN);assert!(operations::validate(&w).is_err());
    w.production.operations.advanced_components.clear();w.manufacturing.shipyard_lines.insert(999);assert!(operations::validate(&w).is_err());
}

#[test]
fn new_construction_previews_explain_conditional_operations_without_mutating_state() {
    let (w,d)=fixture();let before=save(&w);
    for (k,cost,days) in [(K::OfficeDistrict,0.16,360),(K::Shipyard,0.32,840),(K::AdvancedIndustry,0.24,660)] {
        let p=construction_preview::preview(&w,HOME,&d,k,None);assert_eq!(p.cost_bn,cost);assert_eq!(p.minimum_days,days);
        assert!(!p.province_effects.is_empty());assert!(!p.national_effects.is_empty());assert!(!p.operating_requirements.is_empty());
        assert_eq!(save(&w),before);
    }
    assert_eq!(manufacturing::advanced_components_demand_daily(&w,HOME),0.0,"existing supplier work receives no retroactive component bill");
}

#[test]
fn all_sixteen_construction_contracts_are_financial_only_even_for_tonga() {
    for nation in [N::France,N::Tonga] {
        let (mut base,_)=fixture();base.player=Some(nation);
        base.nation_mut(nation).political_capital=1000.0;
        programs::set_construction_budget(&mut base,nation,0.01).unwrap();
        programs::begin_day(&mut base);
        base.nation_mut(nation).program_budget.as_mut().unwrap().available_bn[BUDGET_INDUSTRY][0]=1.0;
        for row in &mut base.resources.market.as_mut().unwrap().stocks {if row.nation==nation {row.quantity=0.0;}}
        let d=base.districts.iter().find(|(_,n)|**n==nation).unwrap().0.clone();
        for kind in production::PROJECT_KINDS {
            let mut w=base.clone();
            // Valid frozen work may survive changed site eligibility. Exercise
            // the actual allocator for every identity, independent of siting UI.
            w.production.projects.push(production::Project {id:1,nation,district:d.clone(),kind,
                priority:production::Priority::Normal,status:production::ProjectStatus::Building,reason:None,
                progress_days:0.0,total_days:production::catalog(kind).total_days,resources_used:[0.0;12],
                capacity_micros:(kind==K::StarterIndustry).then_some(1_000),
                started_day:(kind==K::StarterIndustry).then(||clock::absolute_day(&w))});
            industry::begin_work_day(&mut w);let plan=industry::project_plans(&w)[&1].clone();
            assert!(plan.advance_days>0.0,"{nation:?} {kind:?}: {:?}",plan.reason);
            assert_eq!(plan.required,[0.0;12]);assert_eq!(plan.goods,industry::Goods::default());
            let before=w.resources.market.as_ref().unwrap().stocks.clone();
            production::tick_day(&mut w);
            assert!(w.production.projects[0].progress_days>0.0,"{nation:?} {kind:?}");
            assert_eq!(w.resources.market.as_ref().unwrap().stocks,before);
        }
    }
}
