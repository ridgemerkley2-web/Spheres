//! Reproducible descriptive daily campaign panel. Does not read live campaigns.
//! cargo run -p spheres-sim --release --example daily_calibration -- --years 30 --seeds 1990,7,42 --output daily.csv
use spheres_sim::{apply_command,arsenal,clock,economic_ai,economy,exact,fiscal_preview,init::world_1990,operations,production,programs,province_economy,resources,starting_industry,tech,tick_day,war,world::*,Command};
use std::{collections::{BTreeMap,BTreeSet},fs::File,io::{BufWriter,Write},path::PathBuf,time::Instant};

const SCENARIOS:[(&str,NationId,&str);11]=[
    ("idle_human",NationId::USA,"USA; no policy commands; floating automatic central bank"),
    ("balanced_budget",NationId::USA,"USA; enact annual department plan with spending authority at 95% of current revenue net of interest"),
    ("fiscal_stress",NationId::Brazil,"Brazil; controlled opening political capital 100 so the stress plan can be enacted; tax rate 10%; renew inherited department plan at 1.5 times its initial allocations, respecting ministry caps"),
    ("commodity_dependence",NationId::SaudiArabia,"Saudi Arabia; no policy overrides; transcribed oil dependence"),
    ("small_state",NationId::Tonga,"Tonga; no policy overrides; inherited small-state economy"),
    ("command_economy",NationId::USSR,"Soviet Union; no policy overrides; observe government survival and successor world separately"),
    ("war_shock",NationId::Kuwait,"Kuwait player; controlled Iraqi opening political capital 100; priced Iraqi declaration on opening date, then no player orders"),
    ("import_dependent_industry",NationId::Japan,"Japan; no policy commands; transcribed industry and import exposure"),
    ("enrolled_budget",NationId::USA,"USA; renew unchanged inherited annual ministry allocations with default department shares through normal priced commands"),
    ("investment_program",NationId::USA,"USA; unchanged inherited annual department plan and one annual project attempt from the existing deterministic investment candidate; normal prices, funding and material gates; no grants"),
    ("supply_disruption",NationId::Japan,"Japan; observe January1990 paid spot imports; on February1 attempt one priced sanction against largest supplier by cumulative paid import value (nation-id tie break); no forced success or grants"),
];
#[derive(Clone,serde::Serialize)]
struct Row {
    scenario:String,seed:u64,elapsed_years:u32,date:String,country:String,alive:bool,
    gdp_bn:f64,gdp_multiple:f64,gdp_cagr:Option<f64>,population_m:f64,gdp_per_person:f64,
    inflation:f64,debt_gdp:f64,interest_gdp:f64,treasury_bn:Option<f64>,debt_bn:Option<f64>,
    stability:f64,political_capital:f64,technologies:usize,war_days:u32,shortage_days:u32,
    demand_days:u32,hyperinflation_days:u32,pending_diplomacy:usize,command_refusals:u32,
    world_gdp_bn:f64,living_countries:usize,
    projects_active:usize,projects_stalled:usize,project_completion_headlines:u32,project_stall_days:u32,
    civilian_capital_spent_bn:f64,civilian_capital_authority_bn:f64,civilian_capital_authority_used_share:Option<f64>,
    inherited_industry_utilization:Option<f64>,industrial_site_days:u64,industrial_active_site_days:u64,
    election_headlines:u32,coup_headlines:u32,world_election_headlines:u32,world_coup_headlines:u32,
    force_structure:f64,force_deployed:f64,force_reserve:f64,deployed_force_days:f64,
    arsenal_book_value_bn:f64,magazine_fraction:f64,magazine_constrained_days:u32,magazine_empty_days:u32,
    cumulative_military_losses:Option<f64>,raw_overdue_cargo_days:u64,goods_overdue_cargo_days:u64,held_cargo_days:u64,
    current_max_shipment_overdue_days:u32,technologies_acquired:usize,first_technology_acquisition_day:Option<i32>,last_technology_acquisition_day:Option<i32>,
    observer_seconds:f64,simulation_seconds:f64,run_wall_seconds:f64,
}
#[derive(Default)]
struct Counts {
    war:u32,shortage:u32,demand:u32,hyperinflation:u32,refusals:u32,
    project_completions:u32,project_stalls:u32,capital_spent:f64,capital_authority:f64,site_days:u64,active_site_days:u64,
    elections:u32,coups:u32,world_elections:u32,world_coups:u32,deployed_days:f64,magazine_constrained:u32,magazine_empty:u32,
    raw_overdue:u64,goods_overdue:u64,held_cargo:u64,known:BTreeSet<u16>,acquisitions:Vec<Acquisition>,
    imports:BTreeMap<NationId,f64>,events:Vec<serde_json::Value>,disruption_attempted:bool,
    observer_seconds:f64,simulation_seconds:f64,run_seconds:f64,
}
#[derive(serde::Serialize)]
struct Acquisition {technology:String,day:i32,date:String}
fn date(day:i32)->String {let(y,m,d)=clock::date_from_day(day);format!("{y:04}-{m:02}-{d:02}")}
fn stalled(p:&production::Project)->bool {matches!(p.status,production::ProjectStatus::Paused|production::ProjectStatus::Blocked|production::ProjectStatus::Slowed)}
fn observe_before(w:&WorldState,id:NationId,c:&mut Counts) {
    let started=Instant::now();
    if w.nation(id).alive {
        if w.at_war(id) {c.war+=1;}
        if resources::tick_draw(w,id).iter().any(|v|*v>1e-9) {c.demand+=1;}
        if resources::action_stalled_mask(w,id).iter().any(|v|*v) {c.shortage+=1;}
        if w.nation(id).inflation>=1.0 {c.hyperinflation+=1;}
        c.deployed_days+=operations::view(w,id).deployed;
        if war::magazine_multiplier(w,id)<1.0 {c.magazine_constrained+=1;}
        if w.nation(id).munitions<=0.0 {c.magazine_empty+=1;}
    }
    c.observer_seconds+=started.elapsed().as_secs_f64();
}
fn observe_after(w:&WorldState,id:NationId,day:i32,headlines:&[String],c:&mut Counts) {
    let started=Instant::now();let name=id.name();
    for h in headlines {
        if h.starts_with(&format!("{name} votes: ")) {c.elections+=1;}
        if h.starts_with(&format!("COUP IN {}: ",name.to_uppercase())) && h.ends_with("removes the government.") {c.coups+=1;}
        if h.split_once(" votes: ").is_some_and(|(country,_)|spheres_sim::nations::all_nations().iter().any(|n|n.name()==country)) {c.world_elections+=1;}
        if h.starts_with("COUP IN ") && h.ends_with("removes the government.") {c.world_coups+=1;}
        if production::PROJECT_KINDS.iter().any(|kind|h.starts_with(&format!("{name} completes {} level ",production::catalog(*kind).name)))
            || (h.starts_with(&format!("{name} completes a ")) && h.contains("% Starter Industry module in ")) {c.project_completions+=1;}
    }
    if production::projects_for(w,id).any(stalled) {c.project_stalls+=1;}
    if let Some(p)=w.nation(id).program_budget.as_ref().filter(|p|p.settled_day==Some(day)) {
        for (m,departments) in [(BUDGET_INFRASTRUCTURE,4),(BUDGET_INDUSTRY,5),(BUDGET_SCIENCE,1)] {
            for d in 0..departments {
                c.capital_spent+=(p.spent_today_bn[m][d]-p.noncapital_spent_today_bn[m][d]).max(0.0);
                c.capital_authority+=p.accrued_today_bn[m][d];
            }
        }
    }
    if w.production.industry.last_day==Some(day) {
        for site in w.production.industry.operations.iter().filter(|s|w.districts.get(&s.district)==Some(&id)) {
            c.site_days+=1;if site.output_daily>0.0 {c.active_site_days+=1;}
        }
    }
    for cargo in w.logistics.cargo.iter().filter(|cargo|cargo.buyer==id) {
        if cargo.due_day.is_some_and(|due|due<=day) {c.raw_overdue+=1;}
        if cargo.hold_reason.is_some() {c.held_cargo+=1;}
    }
    for cargo in w.commerce.iter().flat_map(|c|&c.cargo).filter(|cargo|cargo.buyer==id) {
        if cargo.due_day<=day {c.goods_overdue+=1;}
        if cargo.hold_reason.is_some() {c.held_cargo+=1;}
    }
    for &t in &w.nation(id).tech.known {
        if c.known.insert(t) {c.acquisitions.push(Acquisition{technology:tech::registry()[t as usize].id.into(),day,date:date(day)});}
    }
    if day<31 {
        if let Some(market)=&w.resources.market {
            if market.last_cleared_day==Some(day) {for f in market.fills.iter().filter(|f|f.buyer==id) {*c.imports.entry(f.seller).or_default()+=f.cost_bn;}}
        }
    }
    c.observer_seconds+=started.elapsed().as_secs_f64();
}
fn row(w:&WorldState,scenario:&str,seed:u64,id:NationId,years:u32,start:f64,c:&Counts)->Row {
    let n=w.nation(id);let multiple=n.gdp/start;let force=operations::view(w,id);
    let last_day=clock::absolute_day(w)-1;
    let overdue=w.logistics.cargo.iter().filter(|cargo|cargo.buyer==id).filter_map(|cargo|cargo.due_day)
        .chain(w.commerce.iter().flat_map(|c|&c.cargo).filter(|cargo|cargo.buyer==id).map(|cargo|cargo.due_day))
        .map(|due|(last_day-due+1).max(0) as u32).max().unwrap_or(0);
    Row{scenario:scenario.into(),seed,elapsed_years:years,date:format!("{:04}-{:02}-{:02}",w.year,w.month,w.day),country:id.name().into(),alive:n.alive,
        gdp_bn:n.gdp,gdp_multiple:multiple,gdp_cagr:if n.alive && years>0 {Some(exact::powf(multiple,1.0/years as f64)-1.0)} else {None},
        population_m:n.population,gdp_per_person:n.gdp*1000.0/n.population.max(1e-9),inflation:n.inflation,debt_gdp:n.debt_gdp,
        interest_gdp:economy::interest_gdp(n),treasury_bn:n.treasury_bn,debt_bn:n.debt_bn,stability:n.stability,political_capital:n.political_capital,
        technologies:n.tech.count(),war_days:c.war,shortage_days:c.shortage,demand_days:c.demand,hyperinflation_days:c.hyperinflation,
        pending_diplomacy:w.agency.offers.len(),command_refusals:c.refusals,
        world_gdp_bn:w.nations.iter().filter(|n|n.alive).map(|n|n.gdp).sum(),living_countries:w.nations.iter().filter(|n|n.alive).count(),
        projects_active:production::projects_for(w,id).count(),projects_stalled:production::projects_for(w,id).filter(|p|stalled(p)).count(),
        project_completion_headlines:c.project_completions,project_stall_days:c.project_stalls,
        civilian_capital_spent_bn:c.capital_spent,civilian_capital_authority_bn:c.capital_authority,
        civilian_capital_authority_used_share:(c.capital_authority>0.0).then(||c.capital_spent/c.capital_authority),
        inherited_industry_utilization:starting_industry::snapshot(w,id).map(|s|s.utilization),industrial_site_days:c.site_days,industrial_active_site_days:c.active_site_days,
        election_headlines:c.elections,coup_headlines:c.coups,world_election_headlines:c.world_elections,world_coup_headlines:c.world_coups,
        force_structure:force.structure,force_deployed:force.deployed,force_reserve:force.reserve,deployed_force_days:c.deployed_days,
        arsenal_book_value_bn:arsenal::book_value(n),magazine_fraction:n.munitions,magazine_constrained_days:c.magazine_constrained,magazine_empty_days:c.magazine_empty,
        cumulative_military_losses:None,raw_overdue_cargo_days:c.raw_overdue,goods_overdue_cargo_days:c.goods_overdue,held_cargo_days:c.held_cargo,
        current_max_shipment_overdue_days:overdue,technologies_acquired:c.acquisitions.len(),first_technology_acquisition_day:c.acquisitions.first().map(|a|a.day),last_technology_acquisition_day:c.acquisitions.last().map(|a|a.day),
        observer_seconds:c.observer_seconds,simulation_seconds:c.simulation_seconds,run_wall_seconds:c.run_seconds}
}
fn command(w:&mut WorldState,c:&Command,counts:&mut Counts) {
    let before=w.nation(w.player.unwrap()).political_capital;
    let result=apply_command(w,c);
    counts.events.push(serde_json::json!({"date":date(clock::absolute_day(w)),"command":c,"accepted":result.is_ok(),"refusal":result.as_ref().err(),"political_capital_charged":before-w.nation(w.player.unwrap()).political_capital}));
    if let Err(why)=result {counts.refusals+=1;eprintln!("{} {:04}-{:02}-{:02}: refused {:?}: {}",w.player.unwrap().name(),w.year,w.month,w.day,c,why);}
}
fn balanced_allocations(w:&WorldState,id:NationId,initial:&[f64;BUDGET_MINISTRIES])->[f64;BUDGET_MINISTRIES] {
    // Quote the normal opening-book command on a copy. Closed-book interest is
    // deliberately zero in ordinary reads, but the proposed plan opens debt.
    // Reuse the browser's fiscal basis without granting money or paying early.
    let budget=Command::SetProgramBudget{nation:id,fiscal_year:w.year,allocations:*initial,departments:programs::default_departments()};
    let (_,quote)=fiscal_preview::with_budget(w,id,&[],&budget).expect("The inherited budget must be quotable");
    let spend=(quote.revenue_gdp-quote.interest_gdp).max(0.0)*0.95;
    let mut allocations=*initial;let total=allocations.iter().sum::<f64>();
    for a in &mut allocations {*a*=spend/total.max(1e-9);}
    allocations
}
fn policy(w:&mut WorldState,scenario:&str,initial:&[f64;BUDGET_MINISTRIES],counts:&mut Counts) {
    let id=w.player.unwrap();if !w.nation(id).alive {return;}
    if scenario=="fiscal_stress" && w.year==1990 {command(w,&Command::SetTaxRate{nation:id,rate:0.10},counts);}
    if !["balanced_budget","fiscal_stress","enrolled_budget","investment_program"].contains(&scenario) {return;}
    let mut allocations=*initial;
    if scenario=="balanced_budget" {
        allocations=balanced_allocations(w,id,initial);
    } else if scenario=="fiscal_stress" {for a in &mut allocations {*a*=1.5;}}
    for (a,cap) in allocations.iter_mut().zip(BUDGET_CAPS) {*a=a.min(cap);}
    command(w,&Command::SetProgramBudget{nation:id,fiscal_year:w.year,allocations,departments:programs::default_departments()},counts);
    if scenario=="investment_program" {
        match economic_ai::candidate(w,id) {
            Ok((district,kind,reason))=>{
                counts.events.push(serde_json::json!({"date":date(clock::absolute_day(w)),"investment_recommendation":reason,"district":district,"kind":kind}));
                command(w,&Command::StartProject{nation:id,district,kind},counts);
            }
            Err(reason)=>counts.events.push(serde_json::json!({"date":date(clock::absolute_day(w)),"investment_unavailable":reason})),
        }
    }
}
fn disruption(w:&mut WorldState,scenario:&str,c:&mut Counts) {
    if scenario!="supply_disruption" || c.disruption_attempted || clock::absolute_day(w)!=31 {return;}
    c.disruption_attempted=true;let id=w.player.unwrap();
    let supplier=c.imports.iter().filter(|(_,value)|**value>0.0)
        .max_by(|(a,av),(b,bv)|av.total_cmp(bv).then_with(||b.cmp(a))).map(|(id,value)|(*id,*value));
    c.events.push(serde_json::json!({"date":date(clock::absolute_day(w)),"disruption_supplier":supplier.map(|s|s.0),"observed_january_import_bn":supplier.map(|s|s.1),"selection":"largest paid January spot-import value; lowest nation id on tie; absent supplier means no disruption"}));
    if let Some((target,_))=supplier {command(w,&Command::Sanction{imposer:id,target},c);}
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture(id:NationId)->WorldState {
        let mut w=world_1990(GameRules{daily_simulation:true,resource_market:true,logistics_routes:true,physical_logistics:true,
            production_system:true,manufacturing_system:true,military_operations:true,..GameRules::default()});
        w.player=Some(id);starting_industry::enable_new_world(&mut w).unwrap();starting_industry::enrich_new_world(&mut w).unwrap();
        province_economy::enable(&mut w);resources::warm(&mut w);w
    }
    #[test]
    fn observers_preserve_old_scenario_worlds_rng_and_headlines() {
        for (scenario,id,_) in SCENARIOS.into_iter().take(7) {
            let mut observed=fixture(id);let mut c=Counts::default();c.known=observed.nation(id).tech.known.iter().copied().collect();
            let initial=observed.nation(id).budget_for(observed.year).allocations;
            if scenario=="fiscal_stress" {observed.nation_mut(id).political_capital=100.0;}
            if scenario=="war_shock" {observed.nation_mut(NationId::Iraq).political_capital=100.0;apply_command(&mut observed,&Command::DeclareWar{attacker:NationId::Iraq,defender:NationId::Kuwait}).unwrap();}
            policy(&mut observed,scenario,&initial,&mut c);let mut plain=observed.clone();
            for _ in 0..3 {
                let day=clock::absolute_day(&observed);disruption(&mut observed,scenario,&mut c);
                observe_before(&observed,id,&mut c);let headlines=tick_day(&mut observed,&[]);
                observe_after(&observed,id,day,&headlines,&mut c);let _=row(&observed,scenario,1990,id,0,1.0,&c);
                assert_eq!(headlines,tick_day(&mut plain,&[]));
                assert_eq!(spheres_sim::save(&observed),spheres_sim::save(&plain),"observer changed {scenario}");
                assert_eq!(observed.rng.state,plain.rng.state);
            }
        }
    }
    #[test]
    fn new_enrollment_and_investment_use_inherited_budgets_and_real_prices() {
        for scenario in ["enrolled_budget","investment_program"] {
            let mut w=fixture(NationId::USA);let id=NationId::USA;
            let initial=w.nation(id).budget_for(w.year).allocations;let pc=w.nation(id).political_capital;
            let gdp=w.nation(id).gdp;let stocks=resources::ALL.map(|commodity|resources::stockpile(&w,id,commodity));
            let mut c=Counts::default();policy(&mut w,scenario,&initial,&mut c);
            assert!(programs::enrolled(&w,id));assert_eq!(w.nation(id).budget_for(w.year).allocations,initial);
            assert!(w.nation(id).political_capital<pc);assert_eq!(w.nation(id).gdp,gdp);
            assert_eq!(resources::ALL.map(|commodity|resources::stockpile(&w,id,commodity)),stocks,"policy cannot gift or pre-consume input stocks");
            assert_eq!(c.refusals,0,"opening fixture must actually exercise its commands");
            assert_eq!(production::projects_for(&w,id).count(),usize::from(scenario=="investment_program"));
        }
    }
    #[test]
    fn disruption_targets_observed_import_value_and_pays_once() {
        let mut w=fixture(NationId::Japan);w.month=2;let mut c=Counts::default();
        c.imports.insert(NationId::USA,2.0);c.imports.insert(NationId::Australia,1.0);
        let pc=w.nation(NationId::Japan).political_capital;
        disruption(&mut w,"supply_disruption",&mut c);
        assert!(w.is_sanctioning(NationId::Japan,NationId::USA));
        assert_eq!(w.nation(NationId::Japan).political_capital,pc-6.0);
        let once=spheres_sim::save(&w);disruption(&mut w,"supply_disruption",&mut c);assert_eq!(spheres_sim::save(&w),once);
        let mut empty=Counts::default();let before=spheres_sim::save(&w);disruption(&mut w,"supply_disruption",&mut empty);assert_eq!(spheres_sim::save(&w),before);
    }
    #[test]
    fn observers_classify_outcomes_and_settled_spending_without_double_counting_rates() {
        let mut w=fixture(NationId::USA);let id=NationId::USA;let initial=w.nation(id).budget_for(w.year).allocations;
        let mut c=Counts::default();c.known=w.nation(id).tech.known.iter().copied().collect();policy(&mut w,"enrolled_budget",&initial,&mut c);
        let day=clock::absolute_day(&w);tick_day(&mut w,&[]);
        let plan=w.nation(id).program_budget.as_ref().unwrap();
        let authority=[(BUDGET_INFRASTRUCTURE,4),(BUDGET_INDUSTRY,5),(BUDGET_SCIENCE,1)].into_iter()
            .flat_map(|(m,ds)|(0..ds).map(move |d|plan.accrued_today_bn[m][d])).sum::<f64>();
        let headlines=vec![format!("{} votes: a party takes office.",id.name()),format!("{} sets a date for its first free elections.",id.name()),
            format!("COUP IN {}: an institution removes the government.",id.name().to_uppercase()),
            format!("{} completes Power Grid level 1 in US-TX.",id.name()),format!("{} cancels Power Grid.",id.name())];
        let before=spheres_sim::save(&w);observe_after(&w,id,day,&headlines,&mut c);
        assert_eq!((c.elections,c.coups,c.project_completions),(1,1,1));
        assert_eq!(c.capital_authority,authority);assert_eq!(before,spheres_sim::save(&w));
        let snapshot=row(&w,"enrolled_budget",1990,id,0,1.0,&c);assert_eq!(snapshot.cumulative_military_losses,None);
    }
    #[test]
    fn opening_balanced_plan_reserves_actual_debt_service_without_mutating_quote_source() {
        let mut w=world_1990(GameRules{daily_simulation:true,..GameRules::default()});
        w.player=Some(NationId::USA);let id=NationId::USA;
        let initial=w.nation(id).budget_for(w.year).allocations;
        assert!(!w.nation(id).on_the_books());
        let before=serde_json::to_vec(&w).unwrap();
        let allocations=balanced_allocations(&w,id,&initial);
        assert_eq!(before,serde_json::to_vec(&w).unwrap());
        apply_command(&mut w,&Command::SetProgramBudget{nation:id,fiscal_year:1990,allocations,departments:programs::default_departments()}).unwrap();
        let n=w.nation(id);let terms=economy::growth_terms(n,n.state_invest_gdp,n.interest_rate,&economy::Conditions::of(&w,id));
        let interest=economy::interest_gdp(n);assert!(interest>0.0);
        let expected=(n.tax_rate+terms.budget_oil_revenue-interest).max(0.0)*0.95;
        assert!((allocations.iter().sum::<f64>()-expected).abs()<1e-12);
        assert!(allocations.iter().sum::<f64>()<(n.tax_rate+terms.budget_oil_revenue)*0.95);
    }
}
fn write_row(out:&mut impl Write,row:&Row)->std::io::Result<()> {
    // Derive field order and values from the same serialized object; JSON numeric
    // formatting preserves precision and Option::None writes an empty CSV cell.
    let value=serde_json::to_value(row).unwrap();let object=value.as_object().unwrap();
    let cells:Vec<String>=object.values().map(|v|match v {
        serde_json::Value::Null=>String::new(),serde_json::Value::String(s)=>format!("\"{}\"",s.replace('"',"\"\"")),_=>v.to_string(),
    }).collect();writeln!(out,"{}",cells.join(","))
}
#[derive(serde::Serialize)]
struct Statistic {n:usize,mean:Option<f64>,sample_variance:Option<f64>,min:Option<f64>,max:Option<f64>}
fn stats(values:impl Iterator<Item=f64>)->Statistic {
    let values:Vec<_>=values.filter(|v|v.is_finite()).collect();let n=values.len();
    let mean=if n>0 {Some(values.iter().sum::<f64>()/n as f64)} else {None};
    Statistic{n,mean,sample_variance:if n>1 {Some(values.iter().map(|v|(v-mean.unwrap()).powi(2)).sum::<f64>()/(n-1) as f64)} else {None},
        min:values.iter().copied().reduce(f64::min),max:values.iter().copied().reduce(f64::max)}
}
fn terminal_statistics(rows:&[Row])->BTreeMap<String,Statistic> {
    let objects:Vec<_>=rows.iter().map(|r|serde_json::to_value(r).unwrap()).collect();
    let Some(first)=objects.first().and_then(|v|v.as_object()) else {return BTreeMap::new();};
    first.iter().filter(|(key,value)|key.as_str()!="seed" && key.as_str()!="elapsed_years" && (value.is_number() || value.is_null()))
        .map(|(key,_)|(key.clone(),stats(objects.iter().filter_map(|v|v[key].as_f64())))).collect()
}
fn main()->Result<(),Box<dyn std::error::Error>> {
    let mut years=30u32;let mut seeds=vec![1990,7,42];let mut output=PathBuf::from("daily-calibration.csv");
    let mut physical_logistics=true;let mut economic_competition=false;
    let mut selected:Option<Vec<String>>=None;let args:Vec<String>=std::env::args().skip(1).collect();let mut i=0;
    while i<args.len() {
        if args[i]=="--help" {println!("daily_calibration --years 30 --seeds 1990,7,42 --output daily.csv [--scenarios idle_human,balanced_budget,...] [--physical-logistics true|false] [--economic-competition true|false]\nWrites annual CSV and a .summary.json with per-scenario means and sample variance. Descriptive evidence, no tuned pass/fail growth bars.");return Ok(());}
        let value=args.get(i+1).ok_or("Every option needs a value")?;
        match args[i].as_str() {
            "--years"=>years=value.parse()?,"--seeds"=>seeds=value.split(',').map(str::parse).collect::<Result<Vec<_>,_>>()?,
            "--physical-logistics"=>physical_logistics=value.parse()?,
            "--economic-competition"=>economic_competition=value.parse()?,
            "--output"=>output=PathBuf::from(value),"--scenarios"=>selected=Some(value.split(',').map(String::from).collect()),
            _=>return Err(format!("Unknown option {}",args[i]).into()),
        } i+=2;
    }
    if seeds.iter().copied().collect::<std::collections::BTreeSet<_>>().len()!=seeds.len() {return Err("Seeds must be distinct; repeated worlds are not independent samples".into());}
    if years>100 || seeds.is_empty() {return Err("Use 0–100 years and at least one seed".into());}
    if let Some(names)=&selected {for name in names {if !SCENARIOS.iter().any(|s|s.0==name) {return Err(format!("Unknown scenario {name}").into());}}}
    if let Some(parent)=output.parent().filter(|p|!p.as_os_str().is_empty()) {std::fs::create_dir_all(parent)?;}
    let mut out=BufWriter::new(File::create(&output)?);let mut header=false;let mut finals=BTreeMap::<String,Vec<Row>>::new();let mut rules=vec![];let mut diagnostics=vec![];
    for (scenario,id,description) in SCENARIOS {
        if selected.as_ref().is_some_and(|s|!s.iter().any(|n|n==scenario)) {continue;}
        for seed in &seeds {
            let run_started=Instant::now();
            let mut w=world_1990(GameRules{seed:*seed,daily_simulation:true,resource_market:true,logistics_routes:true,physical_logistics,
                production_system:true,manufacturing_system:true,economic_competition,military_operations:true,..GameRules::default()});
            w.player=Some(id);starting_industry::enable_new_world(&mut w)?;starting_industry::enrich_new_world(&mut w)?;province_economy::enable(&mut w);resources::warm(&mut w);
            let initial=w.nation(id).budget_for(w.year).allocations;let start=w.nation(id).gdp;let mut counts=Counts::default();
            counts.known=w.nation(id).tech.known.iter().copied().collect();
            if scenario=="fiscal_stress" {w.nation_mut(id).political_capital=100.0;}
            if scenario=="war_shock" {w.nation_mut(NationId::Iraq).political_capital=100.0;apply_command(&mut w,&Command::DeclareWar{attacker:NationId::Iraq,defender:NationId::Kuwait})?;}
            policy(&mut w,scenario,&initial,&mut counts);
            rules.push(serde_json::json!({"scenario":scenario,"seed":seed,"rules":w.rules}));
            eprintln!("START {scenario} seed={seed} years={years}: {description}");
            counts.run_seconds=run_started.elapsed().as_secs_f64();
            let mut last=row(&w,scenario,*seed,id,0,start,&counts);
            if !header {let value=serde_json::to_value(&last)?;writeln!(out,"{}",value.as_object().unwrap().keys().cloned().collect::<Vec<_>>().join(","))?;header=true;}
            write_row(&mut out,&last)?;
            while w.year<1990+years as i32 {
                let old_year=w.year;
                disruption(&mut w,scenario,&mut counts);
                observe_before(&w,id,&mut counts);
                let day=clock::absolute_day(&w);let tick_started=Instant::now();
                let headlines=tick_day(&mut w,&[]);
                counts.simulation_seconds+=tick_started.elapsed().as_secs_f64();
                observe_after(&w,id,day,&headlines,&mut counts);
                // Universal invariants have no sampling false-red rate. These
                // fail loudly; descriptive growth/difficulty metrics do not.
                for n in w.nations.iter().filter(|n|n.alive) {
                    assert!(n.gdp.is_finite() && n.gdp>0.0 && n.population.is_finite() && n.population>0.0,"{scenario}/{seed}: invalid physical economy for {:?}",n.id);
                    assert!(n.inflation.is_finite() && n.debt_gdp.is_finite(),"{scenario}/{seed}: non-finite fiscal state for {:?}",n.id);
                    if let Some(cash)=n.treasury_bn {assert!(cash.is_finite() && cash>=-1e-7);}
                    if let Some(debt)=n.debt_bn {assert!(debt.is_finite() && debt>=-1e-7);}
                }
                if w.year!=old_year {
                    counts.run_seconds=run_started.elapsed().as_secs_f64();
                    last=row(&w,scenario,*seed,id,(w.year-1990) as u32,start,&counts);
                    write_row(&mut out,&last)?;out.flush()?;
                    eprintln!("YEAR {scenario} seed={seed} date={} alive={} GDP={:.3}x inflation={:.3} debt={:.3}",last.date,last.alive,last.gdp_multiple,last.inflation,last.debt_gdp);
                    if w.year<1990+years as i32 {policy(&mut w,scenario,&initial,&mut counts);}
                }
            }
            diagnostics.push(serde_json::json!({"scenario":scenario,"seed":seed,"commands_and_fixture_events":counts.events,"technology_acquisitions":counts.acquisitions,"final_world_hash":format!("{:016x}",spheres_sim::state_hash(&w))}));
            finals.entry(scenario.into()).or_default().push(last);
        }
    }
    out.flush()?;
    let groups:Vec<_>=finals.iter().map(|(name,rows)|serde_json::json!({
        "scenario":name,"runs":rows.len(),"surviving_governments":rows.iter().filter(|r|r.alive).count(),
        "cagr_survivors":stats(rows.iter().filter_map(|r|r.gdp_cagr)),"inflation":stats(rows.iter().map(|r|r.inflation)),
        "debt_gdp":stats(rows.iter().map(|r|r.debt_gdp)),"war_days":stats(rows.iter().map(|r|r.war_days as f64)),
        "shortage_days":stats(rows.iter().map(|r|r.shortage_days as f64)),"command_refusals":stats(rows.iter().map(|r|r.command_refusals as f64)),
        "terminal_statistics":terminal_statistics(rows),
        "terminal_rows":rows,
    })).collect();
    let summary=serde_json::json!({"instrument_schema":2,"instrument":"daily_calibration_v2","years":years,"seeds":seeds,"calendar":"actual daily dates including leap years","economic_competition":economic_competition,"physical_logistics":physical_logistics,
        "initialization":"daily simulation with strategic resource market, manufacturing, production, province accounts and modeled inherited capacity; physical freight and Economic Competition are selected flags; scenario policy fixtures are explicitly described below",
        "scope":"Descriptive evidence. Sample variance is n-1 and unavailable for n<2. No historical fit or growth-rate success claim follows from this panel. CAGR excludes ceased governments; their cessation remains reported. War, shortage and hyperinflation days are observed before each settlement.",
        "sampling":"No statistical calibration bar is added. A future bar must name its target regression, derive sample size from its own measured variance and false-red probability below 1%, and verify power; do not widen an existing bar to match this panel.",
        "metric_definitions":{"civilian_capital_authority_used_share":"Freshly expensed civilian capital divided by accrued authority; Infrastructure departments0-3, Industry0-4, Science0; excludes operating expense and prepaid funds; includes mines/prototypes, not only construction projects","project_completion_headlines":"Tracked-country catalog-project level completion or Starter Industry module completion; cancellations are excluded","project_stall_days":"Settled days with at least one tracked-country project Paused, Blocked or Slowed","industrial_active_site_days":"Owned settled site observations with output_daily>0; denominator industrial_site_days; not physical throughput utilization","inherited_industry_utilization":"Existing inherited output/capacity estimate; may exceed1; no new factories implied","election_headlines":"Exact COUNTRY votes: prefix; scheduling and coalition changes excluded","coup_headlines":"Exact COUP IN COUNTRY: prefix and removes the government. suffix","magazine_constrained_days":"Living tracked-country pre-settlement boundaries with war::magazine_multiplier<1; empty_days uses magazine stock<=0","shipment_delay_days":"Each tracked-country inbound cargo remaining after settlement on/after its booked due date contributes one overdue cargo-day; raw and manufactured separated; held includes hold_reason before or after due date","technology_acquisitions":"Newly known stable tech ids after each settlement; dated from1990-01-01; inherited opening stock excluded; method not inferred","observer_seconds":"Pre/post settlement observer collection only; simulation_seconds covers tick_day; run_wall_seconds includes setup/policy/output overhead"},
        "unavailable_metrics":{"cumulative_military_losses":"No cumulative casualty receipt exists in saved state; net force or inventory changes mix regeneration, acquisition, combat and succession and cannot substitute."},
        "rules_by_run":rules,"run_diagnostics":diagnostics,"scenarios":SCENARIOS.iter().map(|(name,nation,description)|serde_json::json!({"id":name,"nation":nation.name(),"fixture":description})).collect::<Vec<_>>(),"results":groups});
    let summary_path=output.with_extension("summary.json");
    std::fs::write(&summary_path,serde_json::to_string_pretty(&summary)?)?;
    eprintln!("WROTE {} and {}",output.display(),summary_path.display());
    Ok(())
}
