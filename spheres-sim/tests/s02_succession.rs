//! Combined S02 ownership/replay check, not historical calibration. The actual
//! politics dissolution creates all fifteen authored republics. Opening cash,
//! component stock, a completed office and ongoing courses are explicit
//! synthetic fixtures; the infrastructure queue advances through real paid work.
//! The inherited rules do not transfer the dead government's national cash,
//! debt, stock or uncompleted project contracts to its successors. They preserve
//! that historical property while locating residents and completed sites by land.
use spheres_sim::{clock, connected_economy, districts, fiscal_recovery as fiscal,
    init::world_1990, load, politics, population::{self, Course}, production,
    programs, save, tick_day};
use spheres_sim::world::{GameRules, NationId as N, WorldState};

const HEIRS:[N;15]=[N::Russia,N::Ukraine,N::Belarus,N::Kazakhstan,N::Uzbekistan,
    N::Georgia,N::Armenia,N::Azerbaijan,N::Lithuania,N::Latvia,N::Estonia,
    N::Moldova,N::Kyrgyzstan,N::Tajikistan,N::Turkmenistan];
fn near(a:f64,b:f64){assert!((a-b).abs()<1e-7,"{a:.12} != {b:.12}");}
fn residents(w:&WorldState)->f64{w.nations.iter().filter(|n|n.alive).map(|n|n.population).sum()}
fn stocks(w:&WorldState,id:N)->(Option<f64>,Option<f64>){let n=w.nation(id);(n.treasury_bn,n.debt_bn)}

#[test]
fn s02_authored_ussr_succession_conserves_cohorts_and_paid_property_across_save_replay(){
    let mut direct=world_1990(GameRules{daily_simulation:true,production_system:true,
        manufacturing_system:true,ai_aggression:0.0,crisis_intensity:0.0,..Default::default()});
    direct.player=Some(N::USSR);direct.conflicts.clear();
    let district=districts::list_of(N::Russia).iter()
        .find(|d|direct.districts.get(*d)==Some(&N::USSR)).unwrap().clone();
    // Keep the mapped census; two additional million existing residents have
    // no mapped province. This is a technical residual, not a demographic claim.
    let mapped=direct.districts.iter().filter(|(_,owner)|**owner==N::USSR)
        .map(|(d,_)|districts::population_of(&direct,d).unwrap()).sum::<f64>();
    let parent=direct.nation_mut(N::USSR);
    parent.population=mapped+2.0;parent.treasury_bn=Some(20.0);parent.debt_bn=Some(720.0);
    parent.debt_gdp=720.0/parent.gdp;parent.political_capital=1000.0;
    connected_economy::enable(&mut direct).unwrap();
    near(direct.population_system.unallocated[&N::USSR].population_m(),2.0);
    programs::set_construction_budget(&mut direct,N::USSR,0.01).unwrap();
    let project=production::start_project(&mut direct,N::USSR,&district,production::ProjectKind::Infrastructure).unwrap();
    tick_day(&mut direct,&[]);
    let paid_project=direct.production.projects.iter().find(|p|p.id==project).unwrap().clone();
    assert!(paid_project.progress_days>0.0&&paid_project.progress_days<paid_project.total_days as f64);
    let plan=direct.nation(N::USSR).program_budget.as_ref().unwrap();
    assert!(plan.construction_spent_ytd_bn>0.0);assert!(plan.settled_day.is_some());
    assert!(direct.fiscal_recovery.nations[&N::USSR].last_day.is_some());
    // Previously owned opening fixtures, deliberately not a second build or
    // grant made by the succession path under test.
    direct.production.rebuild_sites.insert(district.clone(),[1,0,0]);
    direct.production.operations.advanced_components.insert(N::USSR,0.75);
    let today=clock::absolute_day(&direct);
    for (unallocated,progress) in [(false,10.0),(true,20.0)] {
        let p=if unallocated {direct.population_system.unallocated.get_mut(&N::USSR).unwrap()}
            else {direct.population_system.provinces.get_mut(&district).unwrap()};
        let people=p.working[2].min(0.01)*0.01;assert!(people>0.0);
        p.courses.push(Course{from:2,to:3,people_m:people,started_day:today-30,
            required_days:548.0,funded_days:progress});
    }
    population::reconcile_ownership(&mut direct);
    direct.nation_mut(N::USSR).stability=0.0;direct.nation_mut(N::USSR).separatism=1.0;
    connected_economy::validate(&direct).unwrap();
    let population= residents(&direct);
    let cohorts:Vec<_>=direct.population_system.provinces.iter().filter(|(_,p)|p.last_owner==N::USSR)
        .map(|(d,p)|(d.clone(),p.children.clone(),p.working,p.courses.clone(),p.retirees_m)).collect();
    let unlocated=direct.population_system.unallocated[&N::USSR].clone();
    let parent_money=stocks(&direct,N::USSR);
    let parent_fiscal=direct.fiscal_recovery.nations[&N::USSR].clone();
    let physical=direct.production.rebuild_sites.clone();
    let components=direct.production.operations.advanced_components.clone();
    let mut resumed=load(&save(&direct)).unwrap();

    for w in [&mut direct,&mut resumed] {
        politics::tick(w);
        assert!(w.has_flag("ussr_dissolved"));assert!(!w.nation(N::USSR).alive);
        assert!(HEIRS.iter().all(|id|w.nation_opt(*id).is_some_and(|n|n.alive)));
        near(residents(w),population);
        for (d,children,working,courses,retired) in &cohorts {
            let p=&w.population_system.provinces[d];
            assert_eq!(&p.children,children);assert_eq!(&p.working,working);
            assert_eq!(&p.courses,courses);assert_eq!(p.retirees_m,*retired);
            assert!(HEIRS.contains(&p.last_owner));assert_eq!(w.districts[d],p.last_owner);
        }
        assert!(!w.population_system.unallocated.contains_key(&N::USSR));
        let p=&w.population_system.unallocated[&N::Russia];
        assert_eq!(p.children,unlocated.children);assert_eq!(p.working,unlocated.working);
        assert_eq!(p.courses,unlocated.courses);assert_eq!(p.retirees_m,unlocated.retirees_m);
        assert_eq!(w.districts[&district],N::Russia);
        assert_eq!(w.production.rebuild_sites,physical);
        assert_eq!(w.production.operations.advanced_components,components);
        assert_eq!(stocks(w,N::USSR),parent_money);
        assert_eq!(w.fiscal_recovery.nations[&N::USSR],parent_fiscal);
        assert_eq!(serde_json::to_value(w.production.projects.iter().find(|p|p.id==project).unwrap()).unwrap(),
            serde_json::to_value(&paid_project).unwrap());
        for id in HEIRS {
            assert_eq!(stocks(w,id),(None,None),"new republic retains its authored ratio until its books open");
            assert!(!w.fiscal_recovery.nations.contains_key(&id));
        }
        near(w.nation(N::Russia).debt_gdp,0.35);near(w.nation(N::Ukraine).debt_gdp,0.15);
        connected_economy::validate(w).unwrap();
        let before=save(w);assert_eq!(save(&load(&before).unwrap()),before);
        // Opening each successor recognizes only that republic's current ratio.
        // The parent's observation clock and cash are not inherited or reset.
        fiscal::prepare(w);
        for id in HEIRS {
            let n=w.nation(id);assert_eq!(n.treasury_bn,Some(0.0));
            near(n.debt_bn.unwrap(),n.debt_gdp*n.gdp);
            let f=&w.fiscal_recovery.nations[&id];assert_eq!(f.started_day,Some(today));
            assert_eq!(f.months_observed,0);assert!(f.last_day.is_none());
        }
        assert_eq!(w.fiscal_recovery.nations[&N::USSR],parent_fiscal);
        connected_economy::validate(w).unwrap();
    }
    assert_eq!(save(&direct),save(&resumed));
    for _ in 0..2 {
        resumed=load(&save(&resumed)).unwrap();
        tick_day(&mut direct,&[]);tick_day(&mut resumed,&[]);
        connected_economy::validate(&direct).unwrap();connected_economy::validate(&resumed).unwrap();
        assert_eq!(save(&direct),save(&resumed));
        let p=direct.production.projects.iter().find(|p|p.id==project).unwrap();
        assert_eq!(p.nation,N::USSR);assert_eq!(p.progress_days,paid_project.progress_days);
        assert_eq!(p.status,production::ProjectStatus::Blocked);
        assert_eq!(direct.nations.iter().filter(|n|HEIRS.contains(&n.id)).count(),15);
        assert_eq!(direct.fiscal_recovery.nations[&N::USSR],parent_fiscal);
    }
    assert_eq!(save(&load(&save(&resumed)).unwrap()),save(&direct));
}
