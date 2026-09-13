//! Opt-in, modeled export programmes. Every asset is earned through ordinary
//! financial construction, company development and production commands. These
//! policies are not historical firms or opening military endowments.
use crate::{clock, companies::{self, CompanyOrder as O}, economic_ai, equipment,
    production::{self, ProjectKind as K}, programs, world::{NationId, WorldState, BUDGET_DEFENSE}, Command};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const REVIEW_DAYS: i32 = 30;
const ROSTER: [(&str, &str, u32); 7] = [
    ("France", "ground_apc", 4), ("UK", "ground_recon", 4),
    ("USA", "tank_standard", 2), ("USSR", "tank_heavy", 2),
    ("China", "tank_light", 3), ("SouthAfrica", "ground_artillery", 2),
    ("Japan", "air_light_attack", 1),
];

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Catalogue {
    pub enabled: bool,
    pub plans: BTreeMap<NationId, Plan>,
}
impl Catalogue { pub fn is_empty(&self) -> bool { self == &Self::default() } }

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Plan {
    pub district: Option<String>,
    pub company: Option<u32>,
    pub last_review_day: Option<i32>,
    pub status: String,
    pub reason: String,
}

pub fn enable(w: &mut WorldState) -> Result<(), String> {
    if !clock::is_daily(w) || !w.rules.production_system || !w.rules.manufacturing_system
        || !w.rules.resource_market || !w.rules.military_operations {
        return Err("Supplier programmes require daily construction, manufacturing, resources and military operations.".into());
    }
    validate(w)?;
    w.supplier_catalogue.enabled = true;
    Ok(())
}

pub fn validate(w: &WorldState) -> Result<(), String> {
    let state=&w.supplier_catalogue;
    if state.enabled && (!clock::is_daily(w) || !w.rules.production_system || !w.rules.manufacturing_system
        || !w.rules.resource_market || !w.rules.military_operations) {
        return Err("Enabled supplier programmes require daily construction, manufacturing, resources and military operations.".into());
    }
    if !state.enabled && !state.plans.is_empty() {
        return Err("Disabled supplier programmes cannot retain managed firms.".into());
    }
    for (nation, p) in &state.plans {
        if !ROSTER.iter().any(|(id,_,_)| NationId::parse(id)==Some(*nation))
            || p.last_review_day.is_some_and(|d|d>clock::absolute_day(w))
            || p.status.len()>100 || p.reason.len()>2000 {
            return Err("Invalid saved supplier programme.".into());
        }
        if let Some(id)=p.company {
            let c=companies::company(w,*nation,id).ok_or("The managed supplier is missing from its original government.")?;
            if p.district.as_deref()!=Some(c.district.as_str()) {
                return Err("The supplier programme must retain its actual company site.".into());
            }
        }
        if p.district.as_ref().is_some_and(|d| !w.districts.contains_key(d)) {
            return Err("The supplier programme references an unknown province.".into());
        }
    }
    Ok(())
}

fn report(w:&mut WorldState,n:NationId,status:&str,reason:impl Into<String>) {
    let p=w.supplier_catalogue.plans.entry(n).or_default();
    p.status=status.into(); p.reason=reason.into();
}
fn act(w:&mut WorldState,n:NationId,order:O)->Result<(),String> {
    crate::apply_command(w,&Command::Company{nation:n,order})
}
fn district(w:&WorldState,n:NationId)->Option<String> {
    // Keep a managed site's identity after loss; access is a blocker, never a
    // licence to teleport the company or its stock to a different province.
    if let Some(d)=w.supplier_catalogue.plans.get(&n).and_then(|p|p.district.clone()) {return Some(d);}
    w.districts.iter().filter(|(d,owner)|**owner==n && crate::control::blocker(w,n,d).is_none())
        .max_by(|(a,_),(b,_)| {
            let free=|d:&str| crate::manufacturing::plant_slots(w,d) as usize>crate::manufacturing::used_slots(w,n,d);
            let pop=|d:&str|w.population_system.provinces.get(d).map_or(0.0,|p|p.population_m());
            free(a).cmp(&free(b)).then_with(||pop(a).total_cmp(&pop(b))).then_with(||b.cmp(a))
        }).map(|(d,_)|d.clone())
}

fn ensure_facility(w:&mut WorldState,n:NationId,d:&str,k:K)->Result<bool,String> {
    if production::level(w,d,k)>0 {return Ok(true);}
    let pending=production::projects_for(w,n).find(|p|p.district==d && p.kind==k)
        .map(|p|format!("{}: {:.0}% paid work. {}",production::catalog(k).name,p.progress_fraction()*100.0,p.reason.as_deref().unwrap_or("Funding and paid work continue.")));
    if let Some(reason)=pending {
        report(w,n,"construction",reason);
        return Ok(false);
    }
    crate::apply_command(w,&Command::StartProject{nation:n,district:d.into(),kind:k})?;
    // Prioritise only this programme's newly ordered facility. The ordinary
    // common construction ceiling and appropriation still limit its progress.
    let project=production::projects_for(w,n).find(|p|p.district==d && p.kind==k).map(|p|p.id);
    if let Some(id)=project {
        crate::apply_command(w,&Command::SetProjectPriority{nation:n,project:id,priority:production::Priority::High})?;
    }
    report(w,n,"construction",format!("Ordered {} through the existing financial construction budget; stock requires completed work.",production::catalog(k).name));
    Ok(false)
}

fn ensure_ammunition_support(w:&mut WorldState,n:NationId)->Result<bool,String> {
    if w.nation(n).equipment.as_ref().and_then(|s|s.maintenance_plan.as_ref()).is_none() {
        // The supply owner requires an explicit fleet upkeep plan. Adopt the
        // current fleet's actual requirement through the ordinary command; its
        // next-day invoices remain limited by real maintenance authority.
        let limit=(equipment::fleet_maintenance_requirement(w.nation(n))
            +equipment::legacy_maintenance_requirement(w.nation(n))).clamp(0.0,1000.0);
        crate::apply_command(w,&Command::Equipment{nation:n,order:crate::EquipmentOrder::Maintenance{daily_budget_bn:limit}})?;
        report(w,n,"maintenance_setup","Adopted ordinary upkeep for the existing fleet before commissioning compatible ammunition; invoices begin next day under existing Defense funding.");
        return Ok(false);
    }
    Ok(true)
}

fn review(w:&mut WorldState,n:NationId,platform:&str,target:u32)->Result<(),String> {
    review_impl(w,n,platform,target,true)
}

// The eager arm retains the original complete quote at its original position.
fn review_impl(w:&mut WorldState,n:NationId,platform:&str,target:u32,defer_development_quote:bool)->Result<(),String> {
    if !programs::enrolled(w,n) || w.nation(n).program_budget.as_ref().is_some_and(|p|p.fiscal_year!=w.year) {
        let daily=programs::construction_daily_budget_bn(w,n);
        crate::apply_command(w,&Command::SetConstructionBudget{nation:n,daily_budget_bn:daily})?;
    }
    // Reserve real future procurement authority before trying to capitalise a
    // company. Otherwise the legacy automatic catalogue spends it first.
    if !companies::imports_enabled(w,n) {
        let q=companies::import_enrollment_quote(w,n);
        if !q.valid {return Err(q.reason.unwrap_or_default());}
        act(w,n,O::EnableImports{quote:q.token})?;
    }
    let d=district(w,n).ok_or("No accessible domestic province for the supplier programme.")?;
    w.supplier_catalogue.plans.entry(n).or_default().district=Some(d.clone());
    if let Some(why)=crate::control::blocker(w,n,&d) {return Err(why);}
    if w.districts.get(&d)!=Some(&n) {return Err("The supplier's original province is no longer domestic; property is retained in place.".into());}
    let managed=w.supplier_catalogue.plans.get(&n).and_then(|p|p.company);
    if managed.is_none() && w.companies.firms.iter().any(|c|c.nation==n) {
        report(w,n,"locally_managed","An existing domestic company remains under its owner's management.");
        return Ok(());
    }
    if !ensure_facility(w,n,&d,K::ArmsPlant)? {return Ok(());}
    // Finite components are produced by a financially built precision works;
    // missing intermediates, raw inputs, workers or power remain real blockers.
    if crate::supplier_operations::enabled(w) && !ensure_facility(w,n,&d,K::AdvancedIndustry)? {return Ok(());}
    let id=if let Some(id)=managed {id} else {
        let name=format!("{} Export Works",n.code());
        let capital=0.0001;
        let q=companies::establishment_quote(w,n,&name,&d,capital);
        if !q.valid {return Err(q.reason.unwrap_or_default());}
        act(w,n,O::Establish{name,district:d.clone(),capitalization_bn:capital,quote:q.token})?;
        let id=w.companies.firms.iter().find(|c|c.nation==n).ok_or("The paid supplier establishment was not recorded.")?.id;
        w.supplier_catalogue.plans.get_mut(&n).unwrap().company=Some(id);
        report(w,n,"capital_settlement","Initial capital is awaiting the ordinary fiscal settlement.");
        return Ok(());
    };
    let c=companies::company(w,n,id).ok_or("The managed company is missing.")?;
    if let Some(why)=companies::facility_blocker(w,c) {return Err(why);}
    let spec=equipment::default_spec(platform);
    let preview=equipment::design_preview(w,n,&spec);
    let profile=preview.profile.ok_or_else(||preview.blockers.join(" "))?;
    if !preview.valid {return Err(preview.blockers.join(" "));}
    let budget=(profile.development_cost_bn/profile.development_days.max(1) as f64).max(0.000001);
    let name=format!("{} {}",n.code(),companies::platform_name(platform));
    let eager_quote=(!defer_development_quote).then(||companies::development_quote(w,n,id,&name,&spec,budget,target));
    // Capitalise a bounded buffer, not an unlimited subsidy. Outstanding
    // receivables are already paid obligations and must not be charged again.
    let needed=eager_quote.as_ref().map_or_else(
        ||companies::development_working_capital(w,&profile,target),|q|q.company_cash_needed_bn).max(0.0001);
    let covered=c.cash_bn+c.receivables.iter().filter(|r|r.kind=="capitalization").map(|r|r.amount_bn).sum::<f64>();
    if covered+1e-9<needed && (c.products.is_empty() || c.products.iter().any(|p|p.stock<p.stock_target)) {
        let amount=(needed-covered).min(programs::available_bn(w,n,BUDGET_DEFENSE,3));
        if amount<0.000001 {return Err("Waiting for actual Defense procurement authority to fund the company's working capital.".into());}
        let cap=companies::capitalization_quote(w,n,id,amount);
        if !cap.valid {return Err(cap.reason.unwrap_or_default());}
        act(w,n,O::Capitalize{company:id,amount_bn:amount,quote:cap.token})?;
        report(w,n,"capital_settlement","Reviewed working capital is awaiting ordinary fiscal settlement.");
        return Ok(());
    }
    if c.products.is_empty() {
        let q=eager_quote.unwrap_or_else(||companies::development_quote(w,n,id,&name,&spec,budget,target));
        if !q.valid {return Err(q.reason.unwrap_or_default());}
        act(w,n,O::Develop{company:id,name,spec,daily_budget_bn:budget,stock_target:target,quote:q.token})?;
        report(w,n,"development","The exact baseline design is under paid development; no equipment is available until certification, tooling and production finish.");
        return Ok(());
    }
    let p=c.products[0].clone();
    if p.certified_day.is_some() && p.stock>=p.stock_target {
        if let Some(family)=w.nation(n).equipment.as_ref().and_then(|s|s.revisions.get(&p.revision_id)).and_then(|r|equipment::ammunition_family(&r.spec)) {
            if !ensure_ammunition_support(w,n)? {return Ok(());}
            let c=companies::company(w,n,id).ok_or("The managed company is missing.")?;
            let def=equipment::ammo_def(family).ok_or("The certified weapon's ammunition recipe is missing.")?;
            // Thirty actual recipe work-days is a bounded catalogue buffer,
            // not a stock grant or a promise of combat endurance.
            let ammunition=c.ammunition_products.iter().find(|p|p.family==family);
            let rounds=ammunition.map_or_else(||(def.rounds_per_day*30.0).ceil().clamp(1.0,companies::MAX_AMMO_STOCK as f64) as u32,|p|p.stock_target);
            let ammo=companies::ammo_supply_quote(w,n,id,family,rounds);
            let pending=ammunition.is_none_or(|p|p.stock<p.stock_target);
            if pending && covered+1e-9<ammo.company_cash_needed_bn {
                let amount=(ammo.company_cash_needed_bn-covered).min(programs::available_bn(w,n,BUDGET_DEFENSE,3));
                if amount<0.000001 {return Err("Waiting for procurement authority to capitalise compatible ammunition stock.".into());}
                let cap=companies::capitalization_quote(w,n,id,amount);
                if !cap.valid {return Err(cap.reason.unwrap_or_default());}
                act(w,n,O::Capitalize{company:id,amount_bn:amount,quote:cap.token})?;
                report(w,n,"capital_settlement","Compatible ammunition working capital awaits fiscal settlement.");
                return Ok(());
            }
            if ammunition.is_none() {
                if !ammo.valid {return Err(ammo.reason.unwrap_or_default());}
                act(w,n,O::AmmoSupply{company:id,family:family.into(),stock_target:rounds,quote:ammo.token})?;
                report(w,n,"ammunition_production","Authorised a finite compatible ammunition buffer; every round still requires paid factory work and inputs.");
                return Ok(());
            }
        }
    }
    let status=if p.stock>0 {"stock_available".to_string()} else {p.status.clone()};
    let reason=p.reason.clone();
    report(w,n,&status,&reason);
    Ok(())
}

pub fn tick_day(w:&mut WorldState) {
    if !w.supplier_catalogue.enabled || !economic_ai::enabled(w) {return;}
    let today=clock::absolute_day(w);
    for (slot,(code,platform,target)) in ROSTER.iter().enumerate() {
        if today.rem_euclid(REVIEW_DAYS)!=slot as i32 {continue;}
        let Some(n)=NationId::parse(code) else {continue;};
        if w.player==Some(n) || !w.nation_opt(n).is_some_and(|n|n.alive)
            || w.supplier_catalogue.plans.get(&n).is_some_and(|p|p.last_review_day==Some(today)) {continue;}
        w.supplier_catalogue.plans.entry(n).or_default().last_review_day=Some(today);
        if let Err(why)=review(w,n,platform,*target) {report(w,n,"blocked",why);}
    }
}

/// Pure read model: no quotes, asset enrollment, matching, ticking or funding.
pub fn view(w:&WorldState)->serde_json::Value {
    let rows:Vec<_>=ROSTER.iter().filter_map(|(code,platform,target)| {
        let n=NationId::parse(code)?;
        let p=w.supplier_catalogue.plans.get(&n);
        let c=p.and_then(|p|p.company).and_then(|id|companies::company(w,n,id));
        let facilities:Vec<_>=p.and_then(|p|p.district.as_ref()).map(|d|[K::ArmsPlant,K::AdvancedIndustry].iter().map(|k| {
            let queued=production::projects_for(w,n).find(|p|p.district==*d && p.kind==*k);
            serde_json::json!({"kind":k.key(),"name":production::catalog(*k).name,"level":production::level(w,d,*k),"project":queued.map(|p|p.id),"progress":queued.map(|p|p.progress_fraction()),"status":queued.map(|p|p.status.key()),"reason":queued.and_then(|p|p.reason.as_deref())})
        }).collect()).unwrap_or_default();
        let products:Vec<_>=c.map(|c|c.products.iter().map(|p| {
            let profile=equipment::profile(w.nation(n),&p.revision_id);
            serde_json::json!({"id":p.id,"status":p.status,"reason":p.reason,"ready_stock":p.stock,"target":p.stock_target,
                "development_work_days":p.development_work_days,"development_total_days":profile.map(|p|p.development_days),
                "development_spent_bn":p.development_spent_bn,"certified_day":p.certified_day,
                "tooling_work_days":p.tooling_work_days,"tooling_total_days":profile.map(|p|p.tooling_days),
                "production_work_days":p.unit_work_days,"production_total_days":profile.map(|p|p.production_days),
                "produced_units":p.produced_units})
        }).collect()).unwrap_or_default();
        let (status,reason)=if !w.supplier_catalogue.enabled {("not_enabled","Supplier programmes are not enabled; no autonomous orders are scheduled.")} else if w.player==Some(n) {("player_managed","This government is under player control; autonomous supplier orders are paused.")} else if !w.nation(n).alive {("inactive","This government is no longer active; recorded supplier property is retained.")} else if !economic_ai::enabled(w) {("paused","Economic competition must be enabled for autonomous supplier orders.")} else {p.map_or(("planned","The government will review paid construction on its next scheduled date."),|p|(p.status.as_str(),p.reason.as_str()))};
        Some(serde_json::json!({"nation":n.code(),"nation_name":n.name(),"platform":platform,"platform_name":companies::platform_name(platform),"district":p.and_then(|p|p.district.as_ref()),"company":c.map(|c|c.id),"status":status,"reason":reason,"last_review_day":p.and_then(|p|p.last_review_day),"last_review_status":p.map(|p|p.status.as_str()),"last_review_reason":p.map(|p|p.reason.as_str()),"facilities":facilities,"products":products,"ready_stock":c.map_or(0,|c|c.products.iter().map(|p|p.stock).sum::<u32>()),"stock_target":target}))
    }).collect();
    serde_json::json!({"enabled":w.supplier_catalogue.enabled,"programs":rows,"note":"Modeled export programmes, not historical firms. Governments fund actual facilities, development and limited stock through ordinary budgets. First stock has no guaranteed date; staffing, power, inputs, access and funding can delay work. Player governments retain control of their own companies."})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn s08_deferred_supplier_development_quote_matches_eager_full_world_branches() {
        const N: NationId = NationId::France;
        const PLATFORM: &str = "ground_apc";
        const TARGET: u32 = 4;
        fn compare(w: &WorldState, label: &str, target: u32, status: Option<&str>) -> WorldState {
            let before = crate::save(w);
            let mut deferred = w.clone();
            let mut eager = w.clone();
            let actual = review_impl(&mut deferred, N, PLATFORM, target, true);
            let original = review_impl(&mut eager, N, PLATFORM, target, false);
            assert_eq!(actual, original, "{label}: refusal precedence");
            assert_eq!(crate::save(&deferred), crate::save(&eager), "{label}: complete game state");
            if let Some(status) = status {
                assert!(actual.is_ok(), "{label}: {actual:?}");
                assert_eq!(deferred.supplier_catalogue.plans[&N].status, status, "{label}");
            } else {
                assert!(actual.is_err(), "{label}: fixture must reach a refusal");
            }
            assert_eq!(crate::save(w), before, "{label}: input and quote reads remain immutable");
            deferred
        }

        let mut base = crate::init::world_1990(crate::world::GameRules {
            daily_simulation: true, military_operations: true, production_system: true,
            manufacturing_system: true, resource_market: true, economic_competition: true,
            ..Default::default()
        });
        base.player = Some(N);
        programs::set_construction_budget(&mut base, N, 0.0).unwrap();
        enable(&mut base).unwrap();
        let district = base.districts.iter().find(|(_, owner)| **owner == N).unwrap().0.clone();
        // Synthetic facility and appropriation inputs only. The company,
        // capital obligations and development are ordinary reviewed commands;
        // this fixture is not an earned exporter or campaign qualification.
        base.production.provinces.push(production::ProvinceCapabilities {
            district: district.clone(), arms_plants: 1, infrastructure: 0,
            civilian_industry: 0, power_grid: 0, research_centers: 0,
        });
        base.production.provinces.sort_by(|a, b| a.district.cmp(&b.district));
        base.nation_mut(N).program_budget.as_mut().unwrap().available_bn[BUDGET_DEFENSE][3] = 1.0;
        base.supplier_catalogue.plans.entry(N).or_default().district = Some(district.clone());
        let established = compare(&base, "establishment", TARGET, Some("capital_settlement"));
        assert_eq!(established.companies.firms.len(), 1);
        assert_eq!(established.companies.firms[0].receivables.len(), 1);
        assert!(established.companies.firms[0].products.is_empty());

        let funded = compare(&established, "working capital", TARGET, Some("capital_settlement"));
        assert_eq!(funded.companies.firms[0].receivables.len(), 2,
            "capital review must create exactly one additional paid obligation");
        assert_eq!(funded.companies.firms[0].cash_bn, 0.0,
            "unsettled obligations count as covered capital without becoming spendable cash");
        let developed = compare(&funded, "development order", TARGET, Some("development"));
        assert_eq!(developed.companies.firms[0].products.len(), 1);
        assert_eq!(developed.companies.firms[0].products[0].stock, 0);
        let unchanged = compare(&developed, "existing development", TARGET, Some("development"));
        assert_eq!(unchanged.companies.firms[0].products.len(), 1);
        assert_eq!(unchanged.companies.firms[0].receivables.len(), 2);

        let mut blocked = established.clone();
        blocked.nation_mut(N).program_budget.as_mut().unwrap().available_bn[BUDGET_DEFENSE][3] = 0.0;
        let refused = compare(&blocked, "exhausted procurement", TARGET, None);
        assert_eq!(crate::save(&refused), crate::save(&blocked));
        let mut lost_site = developed.clone();
        lost_site.districts.insert(district, NationId::UK);
        compare(&lost_site, "lost original site", TARGET, None);
        compare(&funded, "invalid empty-catalogue target", 0, None);

        // Explicit synthetic mid-programme states exercise branches without
        // waiting for, or claiming, real certification and paid production.
        let mut short_capital = developed.clone();
        short_capital.companies.firms[0].receivables.clear();
        let id = short_capital.companies.firms[0].id;
        let spec = equipment::default_spec(PLATFORM);
        let q = companies::development_quote(&short_capital, N, id,
            "Repeated development", &spec, 0.01, TARGET);
        assert!(!q.valid, "existing development must make the unused eager quote invalid");
        let recapitalized = compare(&short_capital, "capital precedes unused invalid quote", TARGET, Some("capital_settlement"));
        assert_eq!(recapitalized.companies.firms[0].receivables.len(), 1);

        let mut stocked = developed.clone();
        let today = clock::absolute_day(&stocked);
        let product = &mut stocked.companies.firms[0].products[0];
        product.certified_day = Some(today);
        product.stock = 1;
        product.produced_units = 1;
        let revision = product.revision_id.clone();
        stocked.nation_mut(N).equipment.as_mut().unwrap().revisions.get_mut(&revision).unwrap().certified_day = Some(today);
        let available = compare(&stocked, "existing partial stock", TARGET, Some("stock_available"));
        assert_eq!(available.companies.firms[0].products[0].stock, 1);
        stocked.companies.firms[0].products[0].stock = TARGET;
        stocked.companies.firms[0].products[0].produced_units = TARGET;
        let upkeep = compare(&stocked, "completed buffer upkeep", TARGET, Some("maintenance_setup"));
        assert!(upkeep.nation(N).equipment.as_ref().unwrap().maintenance_plan.is_some());
        assert!(upkeep.companies.firms[0].ammunition_products.is_empty());
    }

    #[test]
    fn supplier_ammunition_support_adopts_only_a_real_next_day_upkeep_plan() {
        let mut w=crate::init::world_1990(crate::world::GameRules {
            daily_simulation:true,military_operations:true,production_system:true,
            manufacturing_system:true,resource_market:true,economic_competition:true,..Default::default()
        });
        let n=NationId::France;
        let daily=programs::construction_daily_budget_bn(&w,n);
        crate::apply_command(&mut w,&Command::SetConstructionBudget{nation:n,daily_budget_bn:daily}).unwrap();
        let q=companies::import_enrollment_quote(&w,n);
        act(&mut w,n,O::EnableImports{quote:q.token}).unwrap();
        let before=crate::save(&w);
        let expected=equipment::fleet_maintenance_requirement(w.nation(n))+equipment::legacy_maintenance_requirement(w.nation(n));
        assert!(!ensure_ammunition_support(&mut w,n).unwrap());
        let plan=w.nation(n).equipment.as_ref().unwrap().maintenance_plan.as_ref().unwrap();
        assert_eq!(plan.from_day,clock::absolute_day(&w)+1);
        assert_eq!(plan.daily_limit_bn,expected.clamp(0.0,1000.0));
        assert!(plan.receipt.is_none());
        assert!(w.companies.firms.is_empty());
        let mut reverted=w.clone();
        reverted.nation_mut(n).equipment.as_mut().unwrap().maintenance_plan=None;
        reverted.supplier_catalogue.plans.clear();
        assert_eq!(crate::save(&reverted),before,"Only the upkeep order and planner explanation change; no spending, rounds, inputs or property are granted.");
        let once=crate::save(&w);
        assert!(ensure_ammunition_support(&mut w,n).unwrap());
        assert_eq!(crate::save(&w),once);
        assert_eq!(crate::save(&crate::load(&once).unwrap()),once);
    }
}
