//! Cross-system S02 adoption/replay contracts. These are controlled fixtures,
//! not historical calibration or evidence of a certified full campaign.
use spheres_sim::{apply_command, clock, connected_economy, init::world_1990,
    load, population, production, programs, save, tick_day, tick_month, Command};
use spheres_sim::world::{GameRules, NationId as N, WorldState};

fn daily() -> WorldState {
    let mut w=world_1990(GameRules {daily_simulation:true, production_system:true,
        manufacturing_system:true, ai_aggression:0.0, crisis_intensity:0.0, ..Default::default()});
    w.player=Some(N::France);
    w.nation_mut(N::France).political_capital=1000.0;
    w
}
fn evidence(name: &str, value: serde_json::Value) {
    if let Some(root)=std::env::var_os("SPHERES_S02_EVIDENCE") {
        let root=std::path::PathBuf::from(root);std::fs::create_dir_all(&root).unwrap();
        std::fs::write(root.join(name),serde_json::to_string_pretty(&value).unwrap()).unwrap();
    }
}
fn adoption() -> Command { Command::EnableConnectedEconomy {nation:N::France} }

#[test]
fn explicit_adoption_preserves_paid_work_and_current_accounts() {
    let mut w=daily();
    programs::set_construction_budget(&mut w,N::France,0.001).unwrap();
    let district=w.districts.iter().find(|(_,n)|**n==N::France).unwrap().0.clone();
    let id=production::start_project(&mut w,N::France,&district,production::ProjectKind::Infrastructure).unwrap();
    production::set_priority(&mut w,N::France,id,production::Priority::High).unwrap();
    tick_day(&mut w,&[]);
    let before=w.clone();
    apply_command(&mut w,&adoption()).unwrap();
    assert_eq!(serde_json::to_value(&w.production.projects).unwrap(),serde_json::to_value(&before.production.projects).unwrap());
    assert_eq!(serde_json::to_value(&w.production.industry).unwrap(),serde_json::to_value(&before.production.industry).unwrap());
    assert_eq!(serde_json::to_value(&w.nation(N::France).program_budget).unwrap(),serde_json::to_value(&before.nation(N::France).program_budget).unwrap());
    for n in &before.nations {
        let after=w.nation(n.id);
        assert_eq!((after.gdp,after.population,after.debt_gdp),(n.gdp,n.population,n.debt_gdp));
        if n.on_the_books() {assert_eq!((after.treasury_bn,after.debt_bn),(n.treasury_bn,n.debt_bn));}
    }
    evidence("adoption.json",serde_json::json!({
        "fixture":"Financial construction already in progress; explicitly upgraded daily France",
        "date":w.date_str(),"before":{"population":before.nation(N::France).population,"gdp":before.nation(N::France).gdp,
            "cash":before.nation(N::France).treasury_bn,"debt":before.nation(N::France).debt_bn,
            "projects":before.production.projects,"contracts":before.production.industry.projects},
        "after":{"population":w.nation(N::France).population,"gdp":w.nation(N::France).gdp,
            "cash":w.nation(N::France).treasury_bn,"debt":w.nation(N::France).debt_bn,
            "projects":w.production.projects,"contracts":w.production.industry.projects},
        "population_enabled":w.population_system.enabled,"fiscal_enabled":w.rules.fiscal_recovery,
        "industry_enabled":w.rules.industry_rebuild
    }));
    let enrolled=save(&w);
    apply_command(&mut w,&adoption()).unwrap();
    assert_eq!(save(&w),enrolled,"adoption cannot restart dated training or fiscal observation");
    assert_eq!(save(&load(&enrolled).unwrap()),enrolled);
}

#[test]
fn refused_enrollment_is_atomic_and_player_scoped() {
    let mut w=daily();let before=save(&w);
    assert!(apply_command(&mut w,&Command::EnableConnectedEconomy{nation:N::Japan}).is_err());
    assert_eq!(save(&w),before);
    w.nation_mut(N::France).population=-1.0;let bad=save(&w);
    assert!(apply_command(&mut w,&adoption()).is_err());assert_eq!(save(&w),bad);
    let mut legacy=world_1990(GameRules::default());legacy.player=Some(N::France);
    let before=save(&legacy);assert!(apply_command(&mut legacy,&adoption()).is_err());
    assert_eq!(save(&legacy),before);
    let mut no_production=daily();no_production.rules.production_system=false;
    let before=save(&no_production);
    let refusal=connected_economy::enrollment_refusal(&no_production,N::France).unwrap();
    assert!(refusal.contains("production and construction"));
    assert_eq!(apply_command(&mut no_production,&adoption()).unwrap_err(),refusal);
    assert_eq!(save(&no_production),before,"refused command cannot silently enable production or open accounts");
    assert_eq!(connected_economy::enable(&mut no_production).unwrap_err(),refusal);
    assert_eq!(save(&no_production),before,"direct adoption has the same atomic precondition");
}

#[test]
fn versioned_economy_refuses_downgrade_and_separate_master_property() {
    let mut w=daily();connected_economy::enable(&mut w).unwrap();
    let v:serde_json::Value=serde_json::from_str(&save(&w)).unwrap();
    assert_eq!(v["format"],"spheres-economy-save");assert_eq!(v["version"],1);
    assert!(load(&v["world"].to_string()).is_err(),"raw serde must not discard the new ownership contract");
    for (field,value) in [("version",9),("equipment_version",5),("party_leadership_version",1)] {
        let mut bad=v.clone();bad[field]=value.into();assert!(load(&bad.to_string()).is_err(),"{field}");
    }
    for (key,value) in [("companies",serde_json::json!({"enabled":true,"roster":[]})),("campaign",serde_json::json!({"armies":[{"owner":"France"}]}))] {
        let mut bad=v.clone();bad["world"][key]=value;
        assert!(load(&bad.to_string()).is_err(),"refuse unsupported {key} before dropping data");
    }
    for value in [serde_json::json!(true),serde_json::json!(1)] {
        let mut bad=v.clone();bad["world"]["rules"]["operational_warfare"]=value;
        assert!(load(&bad.to_string()).is_err());
    }
    // A malformed or alternate ownership shape must not pass as an absent
    // military book merely because it is not a JSON object.
    for key in ["campaign","campaign_supply","campaign_peace"] {
        for value in [serde_json::json!([{"armies":[{"owner":"France"}]}]),
            serde_json::json!([]),serde_json::json!("unrecognized book"),
            serde_json::json!(false),serde_json::json!(0)] {
            let mut bad=v.clone();bad["world"][key]=value;
            assert!(load(&bad.to_string()).is_err(),"refuse unrecognized {key} shape");
        }
        for empty in [serde_json::Value::Null,serde_json::json!({})] {
            let mut blank=v.clone();blank["world"][key]=empty;
            assert_eq!(save(&load(&blank.to_string()).unwrap()),save(&w),
                "an explicitly empty {key} has no military property to discard");
        }
    }
    let legacy=daily();let raw=save(&legacy);
    let adopted=load(&raw).unwrap();assert!(!connected_economy::has_state(&adopted));
    assert_eq!(save(&adopted),raw,"loading never upgrades economic rules");
}

#[test]
fn integrated_daily_batch_and_resume_have_one_identical_timeline() {
    let mut a=daily();a.year=2000;a.month=2;a.day=15;
    connected_economy::enable(&mut a).unwrap();
    let mut b=load(&save(&a)).unwrap();
    tick_month(&mut a,&[]);
    while b.month==2 {
        tick_day(&mut b,&[]);
        if let Err(error)=connected_economy::validate(&b) { panic!("{error}; date={}; USSR={:?}",b.date_str(),b.fiscal_recovery.nations.get(&N::USSR)); }
        b=load(&save(&b)).unwrap();
    }
    assert_eq!(save(&a),save(&b));
    evidence("daily-replay.json",serde_json::json!({"fixture":"2000 leap February, adoption on day15, save/load after every date",
        "final_date":a.date_str(),"direct_hash":format!("{:016x}",spheres_sim::state_hash(&a)),
        "resumed_hash":format!("{:016x}",spheres_sim::state_hash(&b)),"equal":true,
        "fiscal":a.fiscal_recovery.nations.get(&N::France),"population":population::snapshot(&a,N::France)}));
    // A new system re-entered on the settled date cannot age residents or
    // observe another tax receipt. The ordinary world driver owns date advance.
    let day=clock::absolute_day(&b);population::tick(&mut b);
    let observed=save(&b);population::tick(&mut b);
    assert_eq!(clock::absolute_day(&b),day);assert_eq!(save(&b),observed);
}
