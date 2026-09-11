//! S02 fiscal invariants, not historical calibration. Opening balances,
//! authority and the single Arms Plant below are explicitly synthetic fixtures.
//! Construction and company capitalization then use their real payment paths.
use spheres_sim::{clock, companies, economy, fiscal_recovery as fiscal, init::world_1990,
    production, programs, resources, Command};
use spheres_sim::world::{GameRules, NationId as N, WorldState, BUDGET_DEFENSE, BUDGET_INDUSTRY};

const HOME:N=N::France;
fn near(a:f64,b:f64){assert!((a-b).abs()<1e-10,"{a:.15} != {b:.15}");}
fn money(w:&WorldState)->(f64,f64){let n=w.nation(HOME);(n.treasury_bn.unwrap(),n.debt_bn.unwrap())}
fn base()->WorldState{
    let mut w=world_1990(GameRules{daily_simulation:true,ai_aggression:0.0,crisis_intensity:0.0,..Default::default()});
    w.player=Some(HOME);
    let n=w.nation_mut(HOME);
    n.gdp=100.0;n.treasury_bn=Some(20.0);n.debt_bn=Some(80.0);n.debt_gdp=0.8;
    n.growth_last=0.0;n.interest_rate=0.03;n.inflation=0.02;n.political_capital=1000.0;
    fiscal::enable(&mut w);
    w
}
fn cash_day(w:&mut WorldState,revenue:f64,spending:f64,other:f64){
    fiscal::prepare(w);
    let day=clock::absolute_day(w);let yf=clock::year_fraction(w);
    let n=w.nation(HOME);let gdp=n.gdp;
    let r=revenue*gdp*yf;let s=spending*gdp*yf;let i=economy::interest_gdp(n)*gdp*yf;
    w.nation_mut(HOME).tax_rate=revenue;
    fiscal::record_fiscal(&mut w.fiscal_recovery,HOME,day,r,s,i);
    economy::charge(w,HOME,s+i-r+other*gdp*yf,(s+i-r)/gdp+other*yf);
    let paid=money(w);
    fiscal::tick(w);
    assert_eq!(money(w),paid,"fiscal observation must not pay a second bill");
    clock::advance_date(w);
}
fn cash_month(w:&mut WorldState,revenue:f64,spending:f64,other:f64){
    let month=clock::month_index(w);
    while clock::month_index(w)==month{cash_day(w,revenue,spending,other);}
}

#[test]
fn s02_default_fiscal_methods_are_serialization_inert(){
    let mut w=world_1990(GameRules::default());
    let before=spheres_sim::save(&w);
    fiscal::prepare(&mut w);fiscal::tick(&mut w);
    fiscal::record_fiscal(&mut w.fiscal_recovery,HOME,0,1.0,2.0,3.0);
    assert!(fiscal::assessment(&w,HOME).is_none());
    assert_eq!(fiscal::stability_pressure(&w,HOME),0.0);
    assert_eq!(spheres_sim::save(&w),before);
}

#[test]
fn s02_enrollment_preserves_existing_stocks_and_creates_no_reserve(){
    let mut w=world_1990(GameRules{daily_simulation:true,..Default::default()});
    w.player=Some(HOME);w.day=17;
    let original:Vec<_>=w.nations.iter().filter(|n|n.alive).map(|n|(n.id,n.gdp,n.debt_gdp,n.debt_bn,n.treasury_bn)).collect();
    let n=w.nation_mut(HOME);n.treasury_bn=Some(7.25);n.debt_bn=Some(123.0);n.debt_gdp=123.0/n.gdp;
    fiscal::enable(&mut w);
    assert_eq!(money(&w),(7.25,123.0));
    for (id,gdp,ratio,debt,cash) in original{
        if id==HOME{continue;}
        let n=w.nation(id);assert_eq!(n.gdp,gdp);assert_eq!(n.debt_gdp,ratio);
        assert_eq!(n.treasury_bn,Some(cash.unwrap_or(0.0)));
        assert_eq!(n.debt_bn,Some(debt.unwrap_or(ratio*gdp)));
    }
    let before=spheres_sim::save(&w);fiscal::enable(&mut w);fiscal::prepare(&mut w);
    assert_eq!(spheres_sim::save(&w),before,"enrollment cannot reset the observation clock");
}

#[test]
fn s02_receipt_and_final_cash_are_observed_once(){
    let mut w=base();let day=clock::absolute_day(&w);
    fiscal::record_fiscal(&mut w.fiscal_recovery,HOME,day,0.3,0.4,0.1);
    economy::charge(&mut w,HOME,0.2,0.002);
    // A separately paid tail transaction changes closing cash, not the receipt.
    economy::charge(&mut w,HOME,0.05,0.0005);
    let paid=money(&w);fiscal::tick(&mut w);
    let f=&w.fiscal_recovery.nations[&HOME];let row=f.accumulating.as_ref().unwrap();
    near(row.revenue_bn,0.3);near(row.spending_bn,0.4);near(row.interest_bn,0.1);
    assert_eq!((row.closing_treasury_bn,row.closing_debt_bn),paid);
    let before=spheres_sim::save(&w);
    fiscal::record_fiscal(&mut w.fiscal_recovery,HOME,day,30.0,40.0,10.0);
    fiscal::tick(&mut w);assert_eq!(spheres_sim::save(&w),before);
    assert_eq!(money(&w),paid);
}

#[test]
fn s02_partial_months_and_calendar_jumps_do_not_invent_paid_years(){
    let mut w=base();w.day=31;
    w.fiscal_recovery=Default::default();fiscal::prepare(&mut w);
    cash_day(&mut w,0.25,0.40,0.0);
    near(w.fiscal_recovery.nations[&HOME].observed_month_fraction,1.0/31.0);
    assert_eq!(w.fiscal_recovery.nations[&HOME].months_observed,0);
    assert_eq!(fiscal::assessment(&w,HOME).unwrap().grace_months_remaining,12);
    w.year=1992;w.month=2;w.day=29;
    cash_day(&mut w,0.25,0.40,0.0);
    let f=&w.fiscal_recovery.nations[&HOME];
    near(f.observed_month_fraction,1.0/31.0+1.0/29.0);
    assert_eq!(f.observations.len(),2);assert_eq!(f.months_observed,0);
    assert_eq!(fiscal::stability_pressure(&w,HOME),0.0);
}

fn funded_company_fixture()->WorldState{
    let mut w=base();w.rules.production_system=true;w.rules.manufacturing_system=true;w.rules.resource_market=true;
    w.rules.military_operations=true;
    w.conflicts.clear();resources::tick(&mut w);
    programs::set_construction_budget(&mut w,HOME,0.2).unwrap();programs::begin_day(&mut w);
    let district=w.districts.iter().find(|(_,owner)|**owner==HOME).unwrap().0.clone();
    if let Some(p)=w.production.provinces.iter_mut().find(|p|p.district==district){p.arms_plants=1;}
    else{w.production.provinces.push(production::ProvinceCapabilities{district:district.clone(),arms_plants:1,infrastructure:0,civilian_industry:0,power_grid:0,research_centers:0});w.production.provinces.sort_by(|a,b|a.district.cmp(&b.district));}
    let n=w.nation_mut(HOME);n.treasury_bn=Some(20.0);n.debt_bn=Some(80.0);n.debt_gdp=80.0/n.gdp;
    n.arsenal.held.clear();n.arsenal.orders.clear();n.arsenal.banked=0.0;
    let p=n.program_budget.as_mut().unwrap();
    p.available_bn=programs::ZERO;p.prepaid_bn=programs::ZERO;p.spent_today_bn=programs::ZERO;
    p.prepaid_used_today_bn=programs::ZERO;p.noncapital_spent_today_bn=programs::ZERO;
    p.available_bn[BUDGET_DEFENSE][3]=1.0;p.available_bn[BUDGET_INDUSTRY][0]=0.2;
    p.revenue_today_bn=0.3;p.interest_today_bn=0.1;p.fiscal_staged=true;
    programs::spend_construction(&mut w,HOME,0.2).unwrap();
    let quote=companies::establishment_quote(&w,HOME,"Synthetic S02 Works",&district,1.0);
    assert!(quote.valid,"{:?}",quote.reason);
    spheres_sim::apply_command(&mut w,&Command::Company{nation:HOME,order:companies::CompanyOrder::Establish{
        name:"Synthetic S02 Works".into(),district,capitalization_bn:1.0,quote:quote.token}}).unwrap();
    w
}
fn close_funded_day(w:&mut WorldState){
    companies::settle_receivables(w);programs::finish_day(w);companies::settle_receivables(w);fiscal::tick(w);
}

#[test]
fn s02_construction_and_company_capital_share_one_public_bill(){
    let mut w=funded_company_fixture();
    assert_eq!(money(&w),(20.0,80.0));assert_eq!(w.companies.firms[0].cash_bn,0.0);
    fiscal::tick(&mut w);
    assert!(w.fiscal_recovery.nations[&HOME].last_day.is_none(),"unsettled authorization is not observed spending");
    close_funded_day(&mut w);
    let n=w.nation(HOME);let p=n.program_budget.as_ref().unwrap();
    let spent=p.spent_today_bn.iter().flatten().sum::<f64>();near(spent,1.2);
    near(n.treasury_bn.unwrap()-n.debt_bn.unwrap(),20.0-80.0-(spent+0.1-0.3));
    near(w.companies.firms[0].cash_bn,1.0);near(w.companies.firms[0].capital_received_bn,1.0);
    assert!(w.companies.firms[0].receivables.is_empty());
    let row=w.fiscal_recovery.nations[&HOME].accumulating.as_ref().unwrap();
    near(row.spending_bn,spent);near(row.closing_treasury_bn,n.treasury_bn.unwrap());
    let before=spheres_sim::save(&w);close_funded_day(&mut w);
    assert_eq!(spheres_sim::save(&w),before,"neither public bill nor company receivable may settle twice");
}

#[test]
fn s02_pending_company_payment_and_fiscal_close_survive_reload_exactly(){
    let mut direct=funded_company_fixture();
    let mut resumed=spheres_sim::load(&spheres_sim::save(&direct)).unwrap();
    close_funded_day(&mut direct);close_funded_day(&mut resumed);
    assert_eq!(spheres_sim::state_hash(&direct),spheres_sim::state_hash(&resumed));
    let mut again=spheres_sim::load(&spheres_sim::save(&resumed)).unwrap();
    close_funded_day(&mut again);
    assert_eq!(spheres_sim::state_hash(&direct),spheres_sim::state_hash(&again));
    for _ in 0..3{
        spheres_sim::tick_day(&mut direct,&[]);spheres_sim::tick_day(&mut resumed,&[]);
        assert_eq!(spheres_sim::state_hash(&direct),spheres_sim::state_hash(&resumed));
    }
}

#[test]
fn s02_pending_ordinary_receipt_and_partial_year_continue_deterministically(){
    let mut direct=base();direct.month=12;direct.day=31;
    direct.fiscal_recovery=Default::default();fiscal::prepare(&mut direct);
    let day=clock::absolute_day(&direct);
    fiscal::record_fiscal(&mut direct.fiscal_recovery,HOME,day,0.3,0.4,0.1);
    economy::charge(&mut direct,HOME,0.2,0.002);
    let mut resumed=spheres_sim::load(&spheres_sim::save(&direct)).unwrap();
    for w in [&mut direct,&mut resumed]{fiscal::tick(w);clock::advance_date(w);cash_month(w,0.25,0.30,0.0);}
    assert_eq!(spheres_sim::state_hash(&direct),spheres_sim::state_hash(&resumed));
    assert_eq!(direct.fiscal_recovery.nations[&HOME].months_observed,1);
    near(direct.fiscal_recovery.nations[&HOME].observed_month_fraction,1.0+1.0/31.0);
}

#[test]
fn s02_late_first_program_enrollment_preserves_the_paid_ordinary_bill_and_resumes(){
    let mut direct=base();let day=clock::absolute_day(&direct);let fraction=clock::year_fraction(&direct);
    let opening=money(&direct);
    // Real ordinary fiscal settlement occurs before a late daily policy order.
    // This is the economic_ai first-enrollment sequence, with an explicit
    // command here so the regression does not depend on AI opportunity scores.
    economy::tick(&mut direct);
    let receipt=direct.fiscal_recovery.nations[&HOME].pending.clone().unwrap();
    let paid=money(&direct);assert_eq!(receipt.day,day);
    near(paid.0-paid.1,opening.0-opening.1+receipt.revenue_bn-receipt.spending_bn-receipt.interest_bn);
    assert!(direct.nation(HOME).program_budget.is_none());
    let allocations=direct.nation(HOME).budget_for(direct.year).allocations;
    let command=Command::SetProgramBudget{nation:HOME,fiscal_year:direct.year,allocations,departments:programs::default_departments()};
    spheres_sim::apply_command(&mut direct,&command).unwrap();
    let plan=direct.nation(HOME).program_budget.as_ref().unwrap();
    assert!(plan.day.is_none());assert!(plan.settled_day.is_none());assert_eq!(money(&direct),paid);
    let mut resumed=spheres_sim::load(&spheres_sim::save(&direct)).unwrap();
    for w in [&mut direct,&mut resumed] {
        programs::finish_day(w);assert_eq!(money(w),paid);
        fiscal::tick(w);assert_eq!(money(w),paid,"observation cannot charge or refund an already paid ordinary bill");
        let f=&w.fiscal_recovery.nations[&HOME];assert_eq!(f.last_day,Some(day));assert!(f.pending.is_none());
        let row=f.accumulating.as_ref().unwrap();
        near(row.revenue_bn,receipt.revenue_bn);near(row.spending_bn,receipt.spending_bn);near(row.interest_bn,receipt.interest_bn);
        near(row.year_fraction,fraction);assert_eq!((row.closing_treasury_bn,row.closing_debt_bn),paid);
        let observed=spheres_sim::save(w);fiscal::tick(w);assert_eq!(spheres_sim::save(w),observed);
        fiscal::validate(w).unwrap();clock::advance_date(w);
    }
    for _ in 0..2 {
        resumed=spheres_sim::load(&spheres_sim::save(&resumed)).unwrap();
        spheres_sim::tick_day(&mut direct,&[]);spheres_sim::tick_day(&mut resumed,&[]);
        fiscal::validate(&direct).unwrap();fiscal::validate(&resumed).unwrap();
        assert_eq!(spheres_sim::save(&direct),spheres_sim::save(&resumed));
        assert!(resumed.fiscal_recovery.nations[&HOME].pending.is_none());
    }
    assert_eq!(spheres_sim::save(&spheres_sim::load(&spheres_sim::save(&resumed)).unwrap()),spheres_sim::save(&direct));
}

#[test]
fn s02_fiscal_save_validation_is_pure_and_refuses_corrupt_or_future_receipts(){
    let mut w=base();cash_day(&mut w,0.25,0.30,0.0);
    let before=spheres_sim::save(&w);fiscal::validate(&w).unwrap();assert_eq!(spheres_sim::save(&w),before);
    let mut broken=w.clone();broken.rules.fiscal_recovery=false;assert!(fiscal::validate(&broken).is_err());
    let mut broken=w.clone();broken.rules.daily_simulation=false;assert!(fiscal::validate(&broken).is_err());
    broken.fiscal_recovery=Default::default();assert!(fiscal::validate(&broken).is_err());
    let mut broken=w.clone();broken.fiscal_recovery.nations.get_mut(&HOME).unwrap().confidence_pressure=f64::NAN;
    assert!(fiscal::validate(&broken).is_err());
    let mut broken=w.clone();let tomorrow=clock::absolute_day(&broken)+1;
    broken.fiscal_recovery.nations.get_mut(&HOME).unwrap().pending=Some(fiscal::Receipt{day:tomorrow,revenue_bn:1.0,spending_bn:1.0,interest_bn:0.0});
    assert!(fiscal::validate(&broken).is_err());
    let mut broken=w.clone();broken.fiscal_recovery.nations.get_mut(&HOME).unwrap().months_observed=12;
    assert!(fiscal::validate(&broken).is_err());
    let mut broken=w.clone();broken.fiscal_recovery.nations.remove(&N::Japan);
    assert!(fiscal::validate(&broken).is_err(),"dropping an open government's book must not reset its grace period");
    // Synthetic successor row with the exact authored financial pattern:
    // a new living Russia has its own debt ratio and both stock fields None.
    // Only the next prepare opens those books; validation cannot enroll it.
    assert!(w.nation_opt(N::Russia).is_none());
    let mut successor=w.nation(N::USSR).clone();successor.id=N::Russia;
    successor.debt_gdp=0.35;successor.treasury_bn=None;successor.debt_bn=None;
    w.nations.push(successor);w.reindex();let before=spheres_sim::save(&w);
    fiscal::validate(&w).unwrap();assert_eq!(spheres_sim::save(&w),before);
}

#[test]
fn s02_negative_real_interest_is_observed_and_reloaded_without_repricing(){
    let mut w=base();w.nation_mut(HOME).interest_rate=0.0;w.nation_mut(HOME).inflation=0.20;
    let annual=economy::interest_gdp(w.nation(HOME));assert!(annual<0.0);
    let day=clock::absolute_day(&w);let exact=annual*w.nation(HOME).gdp*clock::year_fraction(&w);
    fiscal::record_fiscal(&mut w.fiscal_recovery,HOME,day,0.0,0.0,exact);
    fiscal::validate(&w).unwrap();
    let mut resumed=spheres_sim::load(&spheres_sim::save(&w)).unwrap();
    for world in [&mut w,&mut resumed] {
        economy::charge(world,HOME,exact,exact/100.0);
        let paid=money(world);fiscal::tick(world);assert_eq!(money(world),paid);
        near(world.fiscal_recovery.nations[&HOME].accumulating.as_ref().unwrap().interest_bn,exact);
        fiscal::validate(world).unwrap();clock::advance_date(world);
        cash_month(world,0.25,0.30,0.0);fiscal::validate(world).unwrap();
        assert!(world.fiscal_recovery.nations[&HOME].observations[0].interest_bn<0.0);
    }
    assert_eq!(spheres_sim::save(&w),spheres_sim::save(&resumed));
    assert_eq!(spheres_sim::save(&spheres_sim::load(&spheres_sim::save(&w)).unwrap()),spheres_sim::save(&w));
}

#[test]
fn s02_dissolved_government_retains_valid_history_without_reopening_its_books(){
    let mut w=base();cash_month(&mut w,0.25,0.30,0.0);
    let n=w.nation_mut(HOME);n.alive=false;n.gdp=0.0;n.treasury_bn=None;n.debt_bn=None;
    let before=spheres_sim::save(&w);fiscal::validate(&w).unwrap();
    assert_eq!(spheres_sim::save(&w),before,"history validation must not reopen dead public accounts");
    w.fiscal_recovery.nations.get_mut(&HOME).unwrap().observations[0].closing_debt_bn=-1.0;
    assert!(fiscal::validate(&w).is_err(),"historical balances remain validated after dissolution");
}

#[test]
fn s02_cash_deficits_remain_payable_during_grace_and_unused_authority_is_not_a_saving(){
    let mut w=base();let opening=money(&w);
    for _ in 0..12{cash_month(&mut w,0.25,0.40,0.0);}
    assert!(money(&w).0-money(&w).1<opening.0-opening.1);
    assert_eq!(fiscal::stability_pressure(&w,HOME),0.0);
    for _ in 0..6{cash_month(&mut w,0.25,0.40,0.0);}
    assert!(fiscal::stability_pressure(&w,HOME)<0.0);
    let before=spheres_sim::save(&w);let assessment=fiscal::assessment(&w,HOME).unwrap();
    assert!(assessment.recovery_required);assert_eq!(spheres_sim::save(&w),before);
    let pressure=fiscal::stability_pressure(&w,HOME);
    for _ in 0..18{cash_month(&mut w,0.60,0.20,0.0);}
    assert!(fiscal::stability_pressure(&w,HOME)>pressure);
    assert!(!fiscal::assessment(&w,HOME).unwrap().recovery_required);
}

#[test]
fn s02_recurring_cash_obligations_are_not_the_same_as_one_completed_purchase(){
    let mut recurring=base();let mut once=recurring.clone();
    for month in 0..9{
        cash_month(&mut recurring,0.30,0.25,if month%3==0{0.36}else{0.0});
        cash_month(&mut once,0.30,0.25,if month==0{1.08}else{0.0});
    }
    near(fiscal::assessment(&recurring,HOME).unwrap().other_obligations_gdp,0.12);
    near(fiscal::assessment(&once,HOME).unwrap().other_obligations_gdp,0.0);
}
