
use spheres_sim::{programs, Command};
use spheres_sim::world::{NationId as N, BUDGET_DEFENSE as D, BUDGET_INFRASTRUCTURE as I};
fn main() {
    let input = std::env::args().nth(1).expect("unchanged campaign archive");
    let raw=std::fs::read_to_string(&input).unwrap();
    let archive:serde_json::Value=serde_json::from_str(&raw).unwrap();
    let encoded=archive["world"].as_str().map(str::to_string).unwrap_or_else(||archive["world"].to_string());
    let mut w=spheres_sim::load(&encoded).expect("original archive native load");
    let n=w.nation(N::Tonga);
    let old=n.budget_for(w.year).allocations;
    let mut allocations=old;
    allocations[I]-=0.025;
    allocations[D]+=0.025;
    let mut departments=n.program_budget.as_ref().map_or_else(programs::default_departments,|p|p.departments);
    departments[D]=[600,600,700,8000,100];
    let before=spheres_sim::save(&w);
    let validation=programs::validation(&w,N::Tonga,w.year,&allocations,&departments);
    assert!(validation.is_none(),"{:?}",validation);
    let preview=programs::preview_with_plan(&w,N::Tonga,allocations,departments).unwrap();
    assert_eq!(spheres_sim::save(&w),before,"preview must not mutate");
    let money=(n.treasury_bn,n.debt_bn);
    let assets=serde_json::to_value((&n.arsenal,&n.equipment,&w.companies,&w.production)).unwrap();
    let old_plan=n.program_budget.clone();
    let pc=n.political_capital;
    let command=Command::SetProgramBudget {nation:N::Tonga,fiscal_year:w.year,allocations,departments};
    spheres_sim::apply_command(&mut w,&command).expect("real approved budget command");
    let n=w.nation(N::Tonga);
    assert_eq!((n.treasury_bn,n.debt_bn),money,"no cash or borrowing at vote");
    assert_eq!(serde_json::to_value((&n.arsenal,&n.equipment,&w.companies,&w.production)).unwrap(),assets,"no property or research at vote");
    if let Some(old)=old_plan {
        let new=n.program_budget.as_ref().unwrap();
        assert_eq!(new.available_bn,old.available_bn,"no retroactive authority");
        assert_eq!(new.prepaid_bn,old.prepaid_bn,"no prepaid grant");
        assert_eq!(new.spent_today_bn,old.spent_today_bn);
        assert_eq!(new.spent_ytd_bn,old.spent_ytd_bn);
    }
    assert!((old.iter().sum::<f64>()-allocations.iter().sum::<f64>()).abs()<f64::EPSILON);
    let spec=spheres_sim::equipment::default_spec("ground_apc");
    let profile=spheres_sim::equipment::design_preview(&w,N::France,&spec).profile.unwrap();
    let estimate=spheres_sim::companies::development_quote(&w,N::France,0,"Prospective APC",&spec,profile.development_cost_bn/profile.development_days as f64,4);
    println!("{}",serde_json::json!({"france_apc_profile":profile,"france_apc_prospective_estimate":estimate,"input":input,"year":w.year,"gdp_bn":n.gdp,"old_allocations":old,"new_allocations":allocations,"total_before":old.iter().sum::<f64>(),"total_after":allocations.iter().sum::<f64>(),"departments":departments,"validation":validation,"political_before":pc,"political_cost_preview":preview.political_cost,"political_after":n.political_capital,"unchanged_cash_and_debt":money,"annual_procurement_bn":n.gdp*allocations[D]*0.8,"native_preview":preview,"command_success":true,"mutations":"budget settings and normal political cost only; existing money/property/research/authority unchanged"}));
}
