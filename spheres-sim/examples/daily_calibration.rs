//! Reproducible descriptive daily campaign panel. Does not read live campaigns.
//! cargo run -p spheres-sim --release --example daily_calibration -- --years 30 --seeds 1990,7,42 --output daily.csv
use spheres_sim::{apply_command,economy,exact,init::world_1990,programs,province_economy,resources,starting_industry,tick_day,world::*,Command};
use std::{collections::BTreeMap,fs::File,io::{BufWriter,Write},path::PathBuf};

const SCENARIOS:[(&str,NationId,&str);7]=[
    ("idle_human",NationId::USA,"USA; no policy commands; floating automatic central bank"),
    ("balanced_budget",NationId::USA,"USA; enact annual department plan with spending authority at 95% of current revenue net of interest"),
    ("fiscal_stress",NationId::Brazil,"Brazil; controlled opening political capital 100 so the stress plan can be enacted; tax rate 10%; renew inherited department plan at 1.5 times its initial allocations, respecting ministry caps"),
    ("commodity_dependence",NationId::SaudiArabia,"Saudi Arabia; no policy overrides; transcribed oil dependence"),
    ("small_state",NationId::Tonga,"Tonga; no policy overrides; inherited small-state economy"),
    ("command_economy",NationId::USSR,"Soviet Union; no policy overrides; observe government survival and successor world separately"),
    ("war_shock",NationId::Kuwait,"Kuwait player; controlled Iraqi opening political capital 100; priced Iraqi declaration on opening date, then no player orders"),
];
#[derive(Clone,serde::Serialize)]
struct Row {
    scenario:String,seed:u64,elapsed_years:u32,date:String,country:String,alive:bool,
    gdp_bn:f64,gdp_multiple:f64,gdp_cagr:Option<f64>,population_m:f64,gdp_per_person:f64,
    inflation:f64,debt_gdp:f64,interest_gdp:f64,treasury_bn:Option<f64>,debt_bn:Option<f64>,
    stability:f64,political_capital:f64,technologies:usize,war_days:u32,shortage_days:u32,
    demand_days:u32,hyperinflation_days:u32,pending_diplomacy:usize,command_refusals:u32,
    world_gdp_bn:f64,living_countries:usize,
}
#[derive(Default)]
struct Counts {war:u32,shortage:u32,demand:u32,hyperinflation:u32,refusals:u32}
fn row(w:&WorldState,scenario:&str,seed:u64,id:NationId,years:u32,start:f64,c:&Counts)->Row {
    let n=w.nation(id);let multiple=n.gdp/start;
    Row{scenario:scenario.into(),seed,elapsed_years:years,date:format!("{:04}-{:02}-{:02}",w.year,w.month,w.day),country:id.name().into(),alive:n.alive,
        gdp_bn:n.gdp,gdp_multiple:multiple,gdp_cagr:if n.alive && years>0 {Some(exact::powf(multiple,1.0/years as f64)-1.0)} else {None},
        population_m:n.population,gdp_per_person:n.gdp*1000.0/n.population.max(1e-9),inflation:n.inflation,debt_gdp:n.debt_gdp,
        interest_gdp:economy::interest_gdp(n),treasury_bn:n.treasury_bn,debt_bn:n.debt_bn,stability:n.stability,political_capital:n.political_capital,
        technologies:n.tech.count(),war_days:c.war,shortage_days:c.shortage,demand_days:c.demand,hyperinflation_days:c.hyperinflation,
        pending_diplomacy:w.agency.offers.len(),command_refusals:c.refusals,
        world_gdp_bn:w.nations.iter().filter(|n|n.alive).map(|n|n.gdp).sum(),living_countries:w.nations.iter().filter(|n|n.alive).count()}
}
fn command(w:&mut WorldState,c:&Command,counts:&mut Counts) {
    if let Err(why)=apply_command(w,c) {counts.refusals+=1;eprintln!("{} {:04}-{:02}-{:02}: refused {:?}: {}",w.player.unwrap().name(),w.year,w.month,w.day,c,why);}
}
fn policy(w:&mut WorldState,scenario:&str,initial:&[f64;BUDGET_MINISTRIES],counts:&mut Counts) {
    let id=w.player.unwrap();if !w.nation(id).alive {return;}
    if scenario=="fiscal_stress" && w.year==1990 {command(w,&Command::SetTaxRate{nation:id,rate:0.10},counts);}
    if scenario!="balanced_budget" && scenario!="fiscal_stress" {return;}
    let mut allocations=*initial;
    if scenario=="balanced_budget" {
        let n=w.nation(id);
        let terms=economy::growth_terms(n,n.state_invest_gdp,n.interest_rate,&economy::Conditions::of(w,id));
        let spend=(n.tax_rate+terms.budget_oil_revenue-economy::interest_gdp(n)).max(0.0)*0.95;
        let total=allocations.iter().sum::<f64>();
        for a in &mut allocations {*a*=spend/total.max(1e-9);}
    } else {for a in &mut allocations {*a*=1.5;}}
    for (a,cap) in allocations.iter_mut().zip(BUDGET_CAPS) {*a=a.min(cap);}
    command(w,&Command::SetProgramBudget{nation:id,fiscal_year:w.year,allocations,departments:programs::default_departments()},counts);
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
    let mut out=BufWriter::new(File::create(&output)?);let mut header=false;let mut finals=BTreeMap::<String,Vec<Row>>::new();let mut rules=vec![];
    for (scenario,id,description) in SCENARIOS {
        if selected.as_ref().is_some_and(|s|!s.iter().any(|n|n==scenario)) {continue;}
        for seed in &seeds {
            let mut w=world_1990(GameRules{seed:*seed,daily_simulation:true,resource_market:true,logistics_routes:true,physical_logistics,
                production_system:true,manufacturing_system:true,economic_competition,military_operations:true,..GameRules::default()});
            w.player=Some(id);starting_industry::enable_new_world(&mut w)?;starting_industry::enrich_new_world(&mut w)?;province_economy::enable(&mut w);resources::warm(&mut w);
            let initial=w.nation(id).budget_for(w.year).allocations;let start=w.nation(id).gdp;let mut counts=Counts::default();
            if scenario=="fiscal_stress" {w.nation_mut(id).political_capital=100.0;}
            if scenario=="war_shock" {w.nation_mut(NationId::Iraq).political_capital=100.0;apply_command(&mut w,&Command::DeclareWar{attacker:NationId::Iraq,defender:NationId::Kuwait})?;}
            policy(&mut w,scenario,&initial,&mut counts);
            rules.push(serde_json::json!({"scenario":scenario,"seed":seed,"rules":w.rules}));
            eprintln!("START {scenario} seed={seed} years={years}: {description}");
            let mut last=row(&w,scenario,*seed,id,0,start,&counts);
            if !header {let value=serde_json::to_value(&last)?;writeln!(out,"{}",value.as_object().unwrap().keys().cloned().collect::<Vec<_>>().join(","))?;header=true;}
            write_row(&mut out,&last)?;
            while w.year<1990+years as i32 {
                let old_year=w.year;
                if w.nation(id).alive {
                    if w.at_war(id) {counts.war+=1;}
                    if resources::tick_draw(&w,id).iter().any(|v|*v>1e-9) {counts.demand+=1;}
                    if resources::action_stalled_mask(&w,id).iter().any(|v|*v) {counts.shortage+=1;}
                    if w.nation(id).inflation>=1.0 {counts.hyperinflation+=1;}
                }
                tick_day(&mut w,&[]);
                // Universal invariants have no sampling false-red rate. These
                // fail loudly; descriptive growth/difficulty metrics do not.
                for n in w.nations.iter().filter(|n|n.alive) {
                    assert!(n.gdp.is_finite() && n.gdp>0.0 && n.population.is_finite() && n.population>0.0,"{scenario}/{seed}: invalid physical economy for {:?}",n.id);
                    assert!(n.inflation.is_finite() && n.debt_gdp.is_finite(),"{scenario}/{seed}: non-finite fiscal state for {:?}",n.id);
                    if let Some(cash)=n.treasury_bn {assert!(cash.is_finite() && cash>=-1e-7);}
                    if let Some(debt)=n.debt_bn {assert!(debt.is_finite() && debt>=-1e-7);}
                }
                if w.year!=old_year {
                    last=row(&w,scenario,*seed,id,(w.year-1990) as u32,start,&counts);
                    write_row(&mut out,&last)?;out.flush()?;
                    eprintln!("YEAR {scenario} seed={seed} date={} alive={} GDP={:.3}x inflation={:.3} debt={:.3}",last.date,last.alive,last.gdp_multiple,last.inflation,last.debt_gdp);
                    if w.year<1990+years as i32 {policy(&mut w,scenario,&initial,&mut counts);}
                }
            }
            finals.entry(scenario.into()).or_default().push(last);
        }
    }
    out.flush()?;
    let groups:Vec<_>=finals.iter().map(|(name,rows)|serde_json::json!({
        "scenario":name,"runs":rows.len(),"surviving_governments":rows.iter().filter(|r|r.alive).count(),
        "cagr_survivors":stats(rows.iter().filter_map(|r|r.gdp_cagr)),"inflation":stats(rows.iter().map(|r|r.inflation)),
        "debt_gdp":stats(rows.iter().map(|r|r.debt_gdp)),"war_days":stats(rows.iter().map(|r|r.war_days as f64)),
        "shortage_days":stats(rows.iter().map(|r|r.shortage_days as f64)),"command_refusals":stats(rows.iter().map(|r|r.command_refusals as f64)),
        "terminal_rows":rows,
    })).collect();
    let summary=serde_json::json!({"years":years,"seeds":seeds,"calendar":"actual daily dates including leap years","economic_competition":economic_competition,"physical_logistics":physical_logistics,
        "initialization":"daily simulation with strategic resource market, manufacturing, production, province accounts and modeled inherited capacity; physical freight and Economic Competition are selected flags; scenario policy fixtures are explicitly described below",
        "scope":"Descriptive evidence. Sample variance is n-1 and unavailable for n<2. No historical fit or growth-rate success claim follows from this panel. CAGR excludes ceased governments; their cessation remains reported. War, shortage and hyperinflation days are observed before each settlement.",
        "sampling":"No statistical calibration bar is added. A future bar must name its target regression, derive sample size from its own measured variance and false-red probability below 1%, and verify power; do not widen an existing bar to match this panel.",
        "rules_by_run":rules,"scenarios":SCENARIOS.iter().map(|(name,nation,description)|serde_json::json!({"id":name,"nation":nation.name(),"fixture":description})).collect::<Vec<_>>(),"results":groups});
    let summary_path=output.with_extension("summary.json");
    std::fs::write(&summary_path,serde_json::to_string_pretty(&summary)?)?;
    eprintln!("WROTE {} and {}",output.display(),summary_path.display());
    Ok(())
}
