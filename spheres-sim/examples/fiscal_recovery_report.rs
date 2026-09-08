//! Descriptive daily fiscal QA. These runs do not claim historical calibration.
//! cargo run --release -p spheres-sim --example fiscal_recovery_report -- ai 2035 1990 artifacts/fiscal-ai.json
//! Modes: ai (all governments), guided (USA follows priced recovery proposals),
//! passive (USA holds its budget), legacy (passive with recovery disabled).
//! All use normal 1990 data and daily rules. No cash/PC/growth overrides.
use spheres_sim::{apply_command, clock, fiscal_recovery, init::world_1990,
    programs, tick_day, Command, world::{GameRules,NationId as N}};
use serde_json::json;

fn main() {
    let args:Vec<_>=std::env::args().collect();
    let mode=args.get(1).map(String::as_str).unwrap_or("ai");
    assert!(["ai","guided","passive","legacy"].contains(&mode));
    let end:i32=args.get(2).map(|s|s.parse().unwrap()).unwrap_or(2035);
    let seed:u64=args.get(3).map(|s|s.parse().unwrap()).unwrap_or(1990);
    let path=args.get(4).expect("output JSON path required");
    let mut w=world_1990(GameRules{seed,daily_simulation:true,
        resource_market:true,logistics_routes:true,physical_logistics:true,
        military_operations:true,operational_warfare:1,production_system:true,
        industry_rebuild:true,manufacturing_system:true,ideology_blocs:true,
        fiscal_recovery:mode!="legacy",..Default::default()});
    w.player=if mode=="ai" {None} else {Some(N::USA)};
    spheres_sim::starting_industry::enable_new_world(&mut w).unwrap();
    spheres_sim::starting_industry::enrich_new_world(&mut w).unwrap();
    spheres_sim::government::ensure_all(&mut w);
    spheres_sim::campaign::enroll(&mut w);
    spheres_sim::province_economy::enable(&mut w);
    spheres_sim::companies::enable(&mut w);
    spheres_sim::population::enable(&mut w);
    spheres_sim::resources::warm(&mut w);
    let opening=spheres_sim::state_hash(&w);
    let mut rows=vec![];
    let mut actions=vec![];
    let mut days=0usize;
    loop {
        if w.month==1 && w.day==1 {
            for n in w.nations.iter().filter(|n|n.alive) {
                rows.push(json!({"year":w.year,"nation":n.id,"gdp_bn":n.gdp,
                    "debt_gdp":n.debt_gdp,"debt_bn":n.debt_bn,"cash_bn":n.treasury_bn,
                    "tax":n.tax_rate,"stability":n.stability,"inflation":n.inflation,
                    "fiscal":fiscal_recovery::assessment(&w,n.id)}));
            }
            eprintln!("{mode} seed {seed}: {} complete; USA debt {:.1}% stability {:.1}",w.year,
                w.nation(N::USA).debt_gdp*100.0,w.nation(N::USA).stability);
        }
        if w.year>=end {break;}
        if w.player.is_some() && w.nation(N::USA).alive &&
            w.nation(N::USA).program_budget.as_ref().is_none_or(|p|p.fiscal_year!=w.year) {
            let n=w.nation(N::USA);
            let command=Command::SetProgramBudget{nation:N::USA,fiscal_year:w.year,
                allocations:n.budget_for(w.year).allocations,
                departments:n.program_budget.as_ref().map_or_else(programs::default_departments,|p|p.departments)};
            apply_command(&mut w,&command).expect("unchanged annual player budget renewal");
        }
        if mode=="guided" && days%90==0 && w.nation(N::USA).alive {
            if let Some(command)=spheres_sim::fiscal_recovery_ai::proposal(&w,N::USA) {
                let price=spheres_sim::price_of(&w,&command).unwrap_or(0.0);
                if price+8.0<=w.nation(N::USA).political_capital {
                    let result=apply_command(&mut w,&command);
                    actions.push(json!({"day":days,"command":command,"cost":price,"error":result.err()}));
                }
            }
        }
        tick_day(&mut w,&[]);
        days+=1;
        for n in w.nations.iter().filter(|n|n.alive) {
            assert!(n.gdp.is_finite() && n.gdp>0.0 && n.debt_gdp.is_finite() && n.debt_gdp>=0.0);
            assert!(n.stability.is_finite() && (0.0..=100.0).contains(&n.stability));
            if fiscal_recovery::enabled(&w) && n.on_the_books() {
                assert!(n.debt_bn.unwrap().is_finite() && n.debt_bn.unwrap()>=0.0);
                assert!(n.treasury_bn.unwrap().is_finite() && n.treasury_bn.unwrap()>=0.0);
                assert!((n.debt_gdp-n.debt_bn.unwrap()/n.gdp).abs()<1e-9,
                    "{} {:?}: ratio {} differs from debt {} / GDP {} = {}",w.date_str(),n.id,
                    n.debt_gdp,n.debt_bn.unwrap(),n.gdp,n.debt_bn.unwrap()/n.gdp);
            }
        }
    }
    let terminal=spheres_sim::state_hash(&w);
    let mut resumed=spheres_sim::load(&spheres_sim::save(&w)).unwrap();
    for _ in 0..7 {tick_day(&mut w,&[]);tick_day(&mut resumed,&[]);}
    assert_eq!(spheres_sim::state_hash(&w),spheres_sim::state_hash(&resumed));
    let output=json!({"mode":mode,"seed":seed,"days":days,"end_year":end,
        "opening_hash":format!("{opening:016x}"),"terminal_hash":format!("{terminal:016x}"),
        "seven_day_save_replay":true,"last_day":clock::absolute_day(&w),"annual":rows,"player_actions":actions});
    std::fs::write(path,serde_json::to_string_pretty(&output).unwrap()).unwrap();
}
