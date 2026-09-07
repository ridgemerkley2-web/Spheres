// Pure equipment work and supply planning. This is deliberately not persisted:
// opening a programme never reserves money, materials, or a factory place.
#[derive(Clone, Debug, Serialize)]
pub struct ProjectSupply {
    pub project_id: u32,
    pub stage: String,
    pub plan_day: i32,
    pub earliest_work_day: i32,
    pub remaining: [f64; 12],
    pub planned_day: [f64; 12],
    pub stock: [f64; 12],
    pub shortfall: [f64; 12],
    pub planned_payment_bn: f64,
    pub planned_work_days: f64,
    pub executable_payment_bn: f64,
    pub executable_work_days: f64,
    pub executable_raw: [f64; 12],
    pub remaining_payment_bn: f64,
    pub remaining_work_days: f64,
    pub department: usize,
    pub funding_available_bn: f64,
    pub daily_cap_bn: f64,
    pub blockers: Vec<String>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct EquipmentRawDemand {
    /// Active, eligible contracts only; per-project views retain paused bills.
    pub remaining: [f64; 12],
    pub next_work: [f64; 12],
    pub horizons: [[f64; 3]; 12],
    /// New procurement appropriation used by custom work. Opening carry and
    /// prepaid balances are excluded, so a bank is never a recurring flow.
    pub procurement_claim_bn: [f64; 3],
    pub procurement_months: [f64; 3],
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct NextWorkSupply {
    pub raw: [f64; 12],
    pub procurement_payment_bn: f64,
    pub procurement_available_bn: f64,
}

struct WorkTerms { tooling: bool, rate: f64, stage_remaining: f64 }
fn work_terms(job: &EquipmentProject) -> WorkTerms {
    let tooling = job.kind == ProjectKind::Production
        && job.work_days + 1e-9 < job.tooling_days as f64;
    let (rate, stage_remaining) = if tooling {
        (job.tooling_cost_bn / job.tooling_days.max(1) as f64,
            job.tooling_days as f64 - job.work_days)
    } else if job.kind == ProjectKind::Development {
        (job.cost_bn / job.minimum_days.max(1) as f64,
            job.minimum_days as f64 - job.work_days)
    } else {
        let work = job.minimum_days.saturating_sub(job.tooling_days).max(1) as f64;
        let per_unit = work / job.quantity.max(1) as f64;
        ((job.cost_bn - job.tooling_cost_bn) / work,
            job.tooling_days as f64 + (job.completed_units as f64 + 1.0) * per_unit - job.work_days)
    };
    WorkTerms { tooling, rate, stage_remaining }
}
fn work_inputs(job: &EquipmentProject, step: f64) -> [f64; 12] {
    if work_terms(job).tooling || job.kind == ProjectKind::Development {
        return [0.0; 12];
    }
    let unit_days = job.minimum_days.saturating_sub(job.tooling_days).max(1) as f64
        / job.quantity.max(1) as f64;
    let units_worked = ((job.work_days + step - job.tooling_days as f64).max(0.0)
        / unit_days).min(job.quantity as f64);
    let batch = job.recipe_per_unit.map(|x| x * job.quantity as f64);
    let target = crate::resources::scale_bundle(&batch, units_worked / job.quantity.max(1) as f64);
    std::array::from_fn(|i| (target[i] - job.resources_used[i]).max(0.0))
}
fn supply_ended(job: &EquipmentProject) -> bool {
    matches!(job.status, ProjectStatus::Complete | ProjectStatus::Cancelled)
}
fn supply_remaining(job: &EquipmentProject) -> [f64; 12] {
    if supply_ended(job) || job.kind == ProjectKind::Development { return [0.0; 12]; }
    std::array::from_fn(|i| (job.recipe_per_unit[i] * job.quantity as f64 - job.resources_used[i]).max(0.0))
}
fn supply_department(job: &EquipmentProject) -> usize {
    if job.kind == ProjectKind::Development { 4 } else { 3 }
}
fn supply_static_blocker(w: &WorldState, id: NationId, job: &EquipmentProject) -> Option<String> {
    if supply_ended(job) { return Some("This programme is closed.".into()); }
    if job.paused { return Some("Paused by the government.".into()); }
    if job.daily_budget_bn <= 0.0 { return Some("Daily project funding is zero.".into()); }
    if job.kind != ProjectKind::Development && job.district.is_none() {
        return Some("The production province is missing.".into());
    }
    if let Some(reason) = live_site_blocker(w, id, job) { return Some(reason); }
    if profile(w.nation(id), &job.revision_id).is_none() { return Some("The saved revision is missing.".into()); }
    let terms = work_terms(job);
    if !terms.rate.is_finite() || terms.rate <= 0.0 || terms.stage_remaining <= 0.0 {
        return Some("The saved work contract is invalid.".into());
    }
    None
}

fn supply_start(w: &WorldState, state: &EquipmentState) -> i32 {
    crate::resources::forecast_start_day(w)
        .max(state.last_tick_day.map_or(i32::MIN, |d| d.saturating_add(1)))
}
// Read the programme's opening balances without cloning or advancing a world.
// Existing accrued/prepaid authority is a stock. Fresh authority is a flow.
fn supply_opening_funds(w: &WorldState, id: NationId, start: i32) -> ([f64; 5], [f64; 5]) {
    let mut funds = [0.0; 5]; let mut carry = [0.0; 5];
    let Some(p) = &w.nation(id).program_budget else { return (funds, carry); };
    let year = clock::date_from_day(start).0;
    for d in [3, 4] {
        let prior = if p.authority_year == year { p.available_bn[crate::world::BUDGET_DEFENSE][d] } else { 0.0 };
        funds[d] = prior + p.prepaid_bn[crate::world::BUDGET_DEFENSE][d];
        let current_accrual = if p.day == Some(start) { p.accrued_today_bn[crate::world::BUDGET_DEFENSE][d] } else { 0.0 };
        carry[d] = (funds[d] - current_accrual).max(0.0);
    }
    (funds, carry)
}
fn supply_accrue(w: &WorldState, id: NationId, state: &EquipmentState, day: i32, funds: &mut [f64; 5]) {
    let n = w.nation(id);
    let Some(p) = &n.program_budget else { return; };
    let (year, month, _) = clock::date_from_day(day);
    if p.fiscal_year != year || day < state.finance_from_day || p.day == Some(day) { return; }
    let total = n.budget_for(year).allocations[crate::world::BUDGET_DEFENSE] * n.gdp.max(0.0)
        / 12.0 / crate::world::days_in_month(year, month).max(1) as f64;
    funds[3] += total * p.departments[crate::world::BUDGET_DEFENSE][3] as f64 / 10_000.0;
    let assigned: f64 = (0..4).map(|d| total * p.departments[crate::world::BUDGET_DEFENSE][d] as f64 / 10_000.0).sum();
    funds[4] += (total - assigned).max(0.0);
}
fn supply_payment(job: &EquipmentProject, step: f64) -> f64 {
    if job.work_days + step + 1e-9 >= job.minimum_days as f64 {
        (job.cost_bn - job.spent_bn).max(0.0)
    } else { work_terms(job).rate * step }
}

/// Next-unsettled work, with the same priority, stage boundaries, daily caps
/// and cumulative material quantization as settlement. Only earlier custom
/// projects are netted from stock; no future deliveries are treated as stock.
pub fn supply_plan(w: &WorldState, id: NationId) -> Vec<ProjectSupply> {
    let Some(n) = w.nation_opt(id).filter(|n| n.alive) else { return vec![]; };
    let Some(state) = &n.equipment else { return vec![]; };
    if !clock::is_daily(w) { return vec![]; }
    let day = supply_start(w, state);
    let (mut funds, _) = supply_opening_funds(w, id, day);
    supply_accrue(w, id, state, day, &mut funds);
    let mut stock = std::array::from_fn(|i| crate::resources::stockpile(w, id, crate::resources::ALL[i]));
    let mut jobs: Vec<_> = state.projects.iter().collect();
    jobs.sort_by_key(|p| (p.priority.dispatch_rank(), p.id));
    let mut out = vec![];
    for job in jobs {
        let department = supply_department(job); let terms = work_terms(job);
        let stage = if job.status == ProjectStatus::Complete { "complete" }
            else if job.status == ProjectStatus::Cancelled { "cancelled" }
            else if terms.tooling { "tooling" }
            else { match job.kind { ProjectKind::Development => "development", ProjectKind::Production => "production", ProjectKind::Refit => "refit" } };
        let mut p = ProjectSupply { project_id:job.id, stage:stage.into(), plan_day:day,
            earliest_work_day:job.started_day.saturating_add(1).max(state.finance_from_day),
            remaining:supply_remaining(job), planned_day:[0.0;12], stock, shortfall:[0.0;12],
            planned_payment_bn:0.0, planned_work_days:0.0, executable_payment_bn:0.0,
            executable_work_days:0.0, executable_raw:[0.0;12],
            remaining_payment_bn:if supply_ended(job){0.0}else{(job.cost_bn-job.spent_bn).max(0.0)},
            remaining_work_days:if supply_ended(job){0.0}else{(job.minimum_days as f64-job.work_days).max(0.0)},
            department, funding_available_bn:funds[department], daily_cap_bn:job.daily_budget_bn, blockers:vec![] };
        let blocker = supply_static_blocker(w,id,job)
            .or_else(|| (day < p.earliest_work_day).then(|| "Waiting for the first eligible work date.".into()))
            .or_else(|| (funds[department] <= 0.0).then(|| "No renewed departmental funding is available for this work.".into()));
        if let Some(reason) = blocker { if !supply_ended(job) { p.blockers.push(reason); } out.push(p); continue; }
        p.planned_work_days = 1.0_f64.min(terms.stage_remaining).min(job.daily_budget_bn / terms.rate).min(funds[department] / terms.rate);
        p.planned_payment_bn = supply_payment(job,p.planned_work_days);
        if p.planned_payment_bn > funds[department] || p.planned_payment_bn > job.daily_budget_bn + 1e-12 {
            p.planned_work_days=0.0; p.planned_payment_bn=0.0;
            p.blockers.push("The remaining contract payment exceeds this funding date's authority.".into());
            out.push(p); continue;
        }
        p.planned_day = work_inputs(job,p.planned_work_days);
        p.shortfall = std::array::from_fn(|i| (p.planned_day[i]-stock[i]).max(0.0));
        let mut fraction = 1.0_f64;
        for i in 0..12 { if p.planned_day[i]>1e-12 && stock[i]+1e-12<p.planned_day[i] { fraction=fraction.min((stock[i]/p.planned_day[i]).clamp(0.0,1.0)); } }
        p.executable_work_days = p.planned_work_days * fraction;
        if p.executable_work_days > 1e-12 {
            p.executable_raw = work_inputs(job,p.executable_work_days);
            p.executable_payment_bn = supply_payment(job,p.executable_work_days);
            // Settlement's final atomic preflight is intentionally strict,
            // including a rounding remainder after proportional work.
            if p.executable_raw.iter().zip(stock).any(|(need,have)|*need>1e-12 && *need>have)
                || p.executable_payment_bn>funds[department]
                || p.executable_payment_bn>job.daily_budget_bn+1e-12 {
                p.executable_raw=[0.0;12];p.executable_work_days=0.0;p.executable_payment_bn=0.0;
                p.blockers.push("The exact material or payment preflight cannot execute this fractional work bundle.".into());
            } else {
                funds[department] = (funds[department]-p.executable_payment_bn).max(0.0);
                for i in 0..12 { if p.executable_raw[i]>1e-12 { stock[i]=(((stock[i]-p.executable_raw[i]).max(0.0)*1e9).round()/1e9).max(0.0); } }
            }
        }
        if fraction < 1.0 { p.blockers.push("Current shared stock cannot support all planned work. Supply the missing inputs to restore the funded pace.".into()); }
        out.push(p);
    }
    out
}

/// A jointly funded target for one date, assuming materials are secured.
/// Unlike summing conditional project attempts, each dollar is assigned only
/// once even when today's empty warehouse would let several jobs retry it.
pub fn next_work_supply(w: &WorldState, id: NationId) -> NextWorkSupply {
    let mut out=NextWorkSupply::default();
    let Some(n)=w.nation_opt(id).filter(|n|n.alive) else {return out;};
    let Some(state)=&n.equipment else {return out;};
    if !clock::is_daily(w) {return out;}
    let day=supply_start(w,state);
    let (mut funds,_)=supply_opening_funds(w,id,day);supply_accrue(w,id,state,day,&mut funds);
    out.procurement_available_bn=funds[3];
    let mut jobs:Vec<_>=state.projects.iter().collect();jobs.sort_by_key(|p|(p.priority.dispatch_rank(),p.id));
    for job in jobs {
        if supply_static_blocker(w,id,job).is_some() || day<=job.started_day || day<state.finance_from_day {continue;}
        let d=supply_department(job);let terms=work_terms(job);
        let step=1.0_f64.min(terms.stage_remaining).min(job.daily_budget_bn/terms.rate).min(funds[d]/terms.rate);
        if step<=1e-12 {continue;}
        let payment=supply_payment(job,step);
        if payment>funds[d] || payment>job.daily_budget_bn+1e-12 {continue;}
        funds[d]=(funds[d]-payment).max(0.0);
        if d==3 {out.procurement_payment_bn+=payment;}
        for (total,raw) in out.raw.iter_mut().zip(work_inputs(job,step)) {*total+=raw;}
    }
    out
}

/// Finite, funded horizons at unchanged GDP, allocation and project caps.
/// Inputs are assumed procurable so a shortage does not erase its own demand.
/// Paused/invalid sites are excluded, tooling uses money but no raw materials,
/// and fresh authority expires with the enacted fiscal year. Previously paid
/// procurement balances survive that boundary exactly as settlement does.
pub fn raw_supply_demand(w: &WorldState, id: NationId) -> EquipmentRawDemand {
    let mut out = EquipmentRawDemand::default();
    let Some(n) = w.nation_opt(id).filter(|n|n.alive) else { return out; };
    let Some(state) = &n.equipment else { return out; };
    let Some(budget) = &n.program_budget else { return out; };
    if !clock::is_daily(w) { return out; }
    let start = supply_start(w,state);

    let mut jobs:Vec<_> = state.projects.iter().filter(|p|supply_static_blocker(w,id,p).is_none()).cloned().collect();
    jobs.sort_by_key(|p|(p.priority.dispatch_rank(),p.id));
    for job in &jobs { for (r,q) in out.remaining.iter_mut().zip(supply_remaining(job)) { *r+=q; } }
    out.next_work=next_work_supply(w,id).raw;
    if jobs.is_empty() { return out; }
    let (mut funds,mut carry) = supply_opening_funds(w,id,start);
    let legacy_buys = crate::arsenal::pick(n).is_some()
        || crate::manufacturing::lines_for(w,id).next().is_some();
    let mut used=[0.0;12]; let mut procurement_claim=0.0;let mut procurement_months=0.0;
    let mut prepaid:[f64;5]=std::array::from_fn(|d|budget.prepaid_bn[crate::world::BUDGET_DEFENSE][d]);
    let mut authority_year=clock::date_from_day(start).0;
    for offset in 0..crate::economic_ai::RAW_HORIZON_DAYS[2] {
        let day=start.saturating_add(offset);
        {
            let (year,month,_)=clock::date_from_day(day);
            if year!=authority_year {
                // Unspent annual authority expires. Already-paid balances do
                // not, and are still spent before fresh authority.
                for d in [3,4] { funds[d]=prepaid[d];carry[d]=prepaid[d]; }
                authority_year=year;
            }
            if year==budget.fiscal_year { procurement_months+=1.0/crate::world::days_in_month(year,month).max(1) as f64; }
            supply_accrue(w,id,state,day,&mut funds);
            for job in &mut jobs {
                if supply_ended(job) || day<=job.started_day || day<state.finance_from_day { continue; }
                let d=supply_department(job); let terms=work_terms(job);
                let step=1.0_f64.min(terms.stage_remaining).min(job.daily_budget_bn/terms.rate).min(funds[d]/terms.rate);
                if step<=1e-12 { continue; }
                let payment=supply_payment(job,step);
                if payment>funds[d] || payment>job.daily_budget_bn+1e-12 { continue; }
                let from_prepaid=prepaid[d].min(payment);prepaid[d]-=from_prepaid;
                let from_carry=carry[d].min(payment); carry[d]-=from_carry;
                if d==3 { procurement_claim+=payment-from_carry; }
                funds[d]=(funds[d]-payment).max(0.0);
                let raw=work_inputs(job,step);
                for i in 0..12 { used[i]+=raw[i]; job.resources_used[i]+=raw[i]; }
                job.work_days=(job.work_days+step).min(job.minimum_days as f64); job.spent_bn+=payment;
                if job.kind!=ProjectKind::Development {
                    let each=job.minimum_days.saturating_sub(job.tooling_days).max(1) as f64/job.quantity.max(1) as f64;
                    job.completed_units=(((job.work_days-job.tooling_days as f64).max(0.0)/each+1e-9).floor() as u32).min(job.quantity);
                }
                if job.work_days+1e-9>=job.minimum_days as f64 { job.status=ProjectStatus::Complete; }
            }
            // Legacy procurement follows custom projects in the daily schedule.
            // Its unused appropriation cannot also finance tomorrow's forecast.
            if legacy_buys { funds[3]=0.0; carry[3]=0.0; prepaid[3]=0.0; }
        }
        for (h,days) in crate::economic_ai::RAW_HORIZON_DAYS.iter().enumerate() {
            if offset+1==*days {
                for i in 0..12 { out.horizons[i][h]=used[i].min(out.remaining[i]); }
                out.procurement_claim_bn[h]=procurement_claim;
                out.procurement_months[h]=procurement_months;
            }
        }
    }
    out
}

#[cfg(test)]
mod supply_tests {
    use super::*;
    use crate::{init::world_1990, resources::{self, Commodity}, world::{GameRules,BUDGET_DEFENSE}};
    const USA:NationId=NationId::USA;
    fn fixture()->(WorldState,String,String) {
        let mut w=world_1990(GameRules {daily_simulation:true,military_operations:true,
            production_system:true,manufacturing_system:true,resource_market:true,resource_gates:true,..GameRules::default()});
        w.player=Some(USA);
        crate::programs::set_construction_budget(&mut w,USA,0.001).unwrap();
        let district=w.districts.iter().find(|(_,n)|**n==USA).unwrap().0.clone();
        for _ in 0..3 { crate::production::complete_capability(&mut w,&district,crate::production::ProjectKind::ArmsPlant); }
        for c in resources::ALL { if c!=Commodity::Oil { resources::set_stockpile_for_test(&mut w,USA,c,1_000_000.0); } }
        let revision=start_development(&mut w,USA,"Supply test vehicle",default_spec("ground_apc"),1.0).unwrap();
        let days=profile(w.nation(USA),&revision).unwrap().development_days;
        for _ in 0..days { next(&mut w); }
        assert!(certified(w.nation(USA),&revision).is_ok());
        (w,district,revision)
    }
    fn next(w:&mut WorldState) {
        clock::advance_date(w); crate::programs::begin_day(w); tick_day(w); crate::programs::finish_day(w);
    }
    fn open_next(w:&mut WorldState) { clock::advance_date(w);crate::programs::begin_day(w); }
    fn approx(a:f64,b:f64) { assert!((a-b).abs()<1e-8,"{a} != {b}"); }
    fn job(w:&WorldState,id:u32)->&EquipmentProject { w.nation(USA).equipment.as_ref().unwrap().projects.iter().find(|p|p.id==id).unwrap() }
    fn plan(w:&WorldState,id:u32)->ProjectSupply { supply_plan(w,USA).into_iter().find(|p|p.project_id==id).unwrap() }
    #[test]
    fn supply_tooling_start_dates_and_pauses_are_finite_and_pure() {
        let (mut w,d,r)=fixture();let id=start_production(&mut w,USA,&r,&d,2,1.0).unwrap();
        let before=crate::save(&w);let p=plan(&w,id);let raw=raw_supply_demand(&w,USA);
        assert_eq!(p.stage,"tooling");assert_eq!(p.planned_day,[0.0;12]);
        // The previous day is settled, so the next planned date is tomorrow.
        assert!(p.plan_day>=clock::absolute_day(&w));
        for i in 0..12 { approx(raw.remaining[i],profile(w.nation(USA),&r).unwrap().recipe[i]*2.0);
            assert_eq!(raw.horizons[i][0],0.0,"all first 30 dates are tooling");
            assert!(raw.horizons[i][1]<=raw.remaining[i]);assert!(raw.horizons[i][2]<=raw.remaining[i]); }
        assert!(raw.horizons.iter().any(|row|row[1]>0.0));assert_eq!(crate::save(&w),before);
        set_project_paused(&mut w,USA,id,true).unwrap();
        assert!(plan(&w,id).remaining.iter().any(|q|*q>0.0));
        assert_eq!(raw_supply_demand(&w,USA).horizons,[[0.0;3];12]);
        assert_eq!(raw_supply_demand(&w,USA).remaining,[0.0;12]);
    }
    #[test]
    fn supply_stock_sharing_matches_atomic_settlement_and_partial_conservation() {
        let (mut w,d,r)=fixture();let a=start_production(&mut w,USA,&r,&d,2,1.0).unwrap();
        let b=start_production(&mut w,USA,&r,&d,2,1.0).unwrap();
        let tooling=profile(w.nation(USA),&r).unwrap().tooling_days;
        for _ in 0..tooling { next(&mut w); }
        open_next(&mut w);
        let full=plan(&w,a).planned_day;let ci=Commodity::Iron.idx();assert!(full[ci]>0.0);
        resources::set_stockpile_for_test(&mut w,USA,Commodity::Iron,full[ci]*0.5);
        let pa=plan(&w,a);let pb=plan(&w,b);let before=crate::save(&w);
        assert!(pa.executable_work_days>0.0&&pa.executable_work_days<pa.planned_work_days);
        assert!(pa.shortfall[ci]>0.0);assert!(pb.stock[ci]<pa.stock[ci]);
        assert_eq!(crate::save(&w),before);
        let before_a=job(&w,a).clone();let before_b=job(&w,b).clone();tick_day(&mut w);
        for (id,p,old) in [(a,pa,before_a),(b,pb,before_b)] {
            let j=job(&w,id);approx(j.work_days-old.work_days,p.executable_work_days);
            approx(j.spent_bn-old.spent_bn,p.executable_payment_bn);
            for i in 0..12 { approx(j.resources_used[i]-old.resources_used[i],p.executable_raw[i]); }
        }
        let forecast=raw_supply_demand(&w,USA);
        for i in 0..12 { let remaining=[a,b].iter().map(|id|supply_remaining(job(&w,*id))[i]).sum::<f64>();
            approx(forecast.remaining[i],remaining);assert!(forecast.horizons[i][2]<=remaining+1e-9); }
    }
    #[test]
    fn supply_zero_funding_invalid_sites_and_closed_projects_do_not_create_demand() {
        let (mut w,d,r)=fixture();let id=start_production(&mut w,USA,&r,&d,1,0.0).unwrap();
        open_next(&mut w);
        assert_eq!(plan(&w,id).planned_work_days,0.0);
        assert_eq!(raw_supply_demand(&w,USA).horizons,[[0.0;3];12]);
        set_project_budget(&mut w,USA,id,1.0).unwrap();
        w.districts.insert(d.clone(),NationId::USSR);
        assert!(!plan(&w,id).blockers.is_empty());assert_eq!(raw_supply_demand(&w,USA).remaining,[0.0;12]);
        w.districts.insert(d,USA);cancel_project(&mut w,USA,id).unwrap();
        assert_eq!(plan(&w,id).stage,"cancelled");assert_eq!(plan(&w,id).remaining,[0.0;12]);
    }
    #[test]
    fn supply_horizon_uses_one_shared_funding_pool_and_respects_enacted_year() {
        let (mut w,d,r)=fixture();let a=start_production(&mut w,USA,&r,&d,1,1.0).unwrap();
        let b=start_production(&mut w,USA,&r,&d,1,1.0).unwrap();open_next(&mut w);
        let rate=work_terms(job(&w,a)).rate;
        let p=w.nation_mut(USA).program_budget.as_mut().unwrap();p.available_bn[BUDGET_DEFENSE][3]=rate*0.75;p.prepaid_bn[BUDGET_DEFENSE][3]=0.0;
        let pa=plan(&w,a);let pb=plan(&w,b);
        approx(pa.planned_work_days,0.75);assert_eq!(pb.planned_work_days,0.0);
        assert!(pa.planned_payment_bn+pb.planned_payment_bn<=rate*0.75+1e-12);
        // A programme signed at year end cannot pretend its tooling has already
        // finished or borrow authority from the following fiscal year.
        w.year=1990;w.month=12;w.day=31;
        let state=w.nation_mut(USA).equipment.as_mut().unwrap();state.last_tick_day=None;
        let forecast=raw_supply_demand(&w,USA);
        assert_eq!(forecast.horizons,[[0.0;3];12]);
    }
    #[test]
    fn supply_finished_batch_consumes_its_bill_once_and_reload_preserves_it() {
        let (mut w,d,r)=fixture();let id=start_production(&mut w,USA,&r,&d,1,1.0).unwrap();
        let total=job(&w,id).minimum_days;let recipe=job(&w,id).recipe_per_unit;
        for _ in 0..total { next(&mut w); }
        assert_eq!(job(&w,id).status,ProjectStatus::Complete);
        for i in 0..12 { approx(job(&w,id).resources_used[i],recipe[i]); }
        let p=plan(&w,id);assert_eq!(p.remaining,[0.0;12]);assert_eq!(p.planned_day,[0.0;12]);
        assert_eq!(raw_supply_demand(&w,USA).horizons,[[0.0;3];12]);
        validate_state(w.nation(USA)).unwrap();let save=crate::save(&w);let loaded=crate::load(&save).unwrap();
        assert_eq!(crate::save(&loaded),save);assert_eq!(raw_supply_demand(&loaded,USA).remaining,[0.0;12]);
    }
    #[test]
    fn supply_forecast_nets_custom_appropriation_and_never_repeats_procurement_bank() {
        let (mut w,d,r)=fixture();let id=start_production(&mut w,USA,&r,&d,2,1.0).unwrap();open_next(&mut w);
        let p=w.nation_mut(USA).program_budget.as_mut().unwrap();p.available_bn[BUDGET_DEFENSE][3]=0.0;p.prepaid_bn[BUDGET_DEFENSE][3]=0.0;p.accrued_today_bn[BUDGET_DEFENSE][3]=0.0;
        let raw=raw_supply_demand(&w,USA);assert!(raw.procurement_claim_bn[0]>0.0);
        let forecast=crate::economic_ai::raw_supply_forecast(&w,USA);
        let standing=resources::recurring_procurement_draw(&w,USA);let monthly=crate::arsenal::budget_of(w.nation(USA));
        let funded_months=raw.procurement_months[0];
        for line in &forecast.lines { if line.commodity==Commodity::Oil {continue;} let i=line.commodity.idx();
            let expected=standing[i]*(funded_months-raw.procurement_claim_bn[0]/monthly).max(0.0)+raw.horizons[i][0];
            approx(line.demand[0],expected);approx(line.equipment_remaining,raw.remaining[i]); }
        w.nation_mut(USA).arsenal.banked=100.0;
        assert_eq!(raw_supply_demand(&w,USA).horizons,raw.horizons);
        assert_eq!(plan(&w,id).planned_work_days,0.0,"no authority on this date");
    }
    #[test]
    fn supply_prepaid_balance_can_fund_work_after_annual_authority_expires() {
        let (mut w,d,r)=fixture();let id=start_production(&mut w,USA,&r,&d,1,1.0).unwrap();
        let tooling=job(&w,id).tooling_days;for _ in 0..tooling { next(&mut w); }
        w.year=1991;w.month=1;w.day=1;
        let rate=work_terms(job(&w,id)).rate;
        let p=w.nation_mut(USA).program_budget.as_mut().unwrap();
        p.prepaid_bn[BUDGET_DEFENSE][3]=rate;p.available_bn[BUDGET_DEFENSE][3]=100.0;
        // The old fiscal programme remains 1990; its large unspent normal
        // appropriation must expire, while exactly one prepaid day survives.
        let raw=raw_supply_demand(&w,USA);let planned=plan(&w,id);
        assert!(raw.horizons.iter().any(|row|row[0]>0.0));
        assert_eq!(raw.procurement_claim_bn,[0.0;3]);assert_eq!(raw.procurement_months,[0.0;3]);
        for i in 0..12 { approx(raw.horizons[i][0],planned.planned_day[i]);approx(raw.horizons[i][2],planned.planned_day[i]); }
        crate::programs::begin_day(&mut w);let before=job(&w,id).resources_used;tick_day(&mut w);
        for i in 0..12 { approx(job(&w,id).resources_used[i]-before[i],raw.horizons[i][0]); }
    }
    #[test]
    fn supply_full_custom_calendar_appropriation_leaves_no_invented_legacy_draw() {
        let (mut w,d,r)=fixture();let id=start_production(&mut w,USA,&r,&d,1,1.0).unwrap();
        w.year=1991;w.month=1;w.day=1;
        let p=w.nation_mut(USA).program_budget.as_mut().unwrap();
        p.fiscal_year=1991;p.authority_year=1991;p.day=None;
        p.available_bn[BUDGET_DEFENSE][3]=0.0;p.prepaid_bn[BUDGET_DEFENSE][3]=0.0;
        let freed=p.departments[BUDGET_DEFENSE][3]-1;p.departments[BUDGET_DEFENSE][3]=1;p.departments[BUDGET_DEFENSE][0]+=freed;
        let raw=raw_supply_demand(&w,USA);let monthly=crate::arsenal::budget_of(w.nation(USA));
        assert!(raw.procurement_claim_bn[0]>0.0);approx(raw.procurement_months[0],30.0/31.0);
        approx(raw.procurement_claim_bn[0],monthly*30.0/31.0);
        let forecast=crate::economic_ai::raw_supply_forecast(&w,USA);
        for line in forecast.lines {if line.commodity!=Commodity::Oil {approx(line.demand[0],raw.horizons[line.commodity.idx()][0]);}}
        assert_eq!(plan(&w,id).stage,"tooling");
    }
    #[test]
    fn supply_exact_fractional_preflight_does_not_claim_rejected_work() {
        let (mut w,d,r)=fixture();let id=start_production(&mut w,USA,&r,&d,3,1.0).unwrap();
        let j=w.nation_mut(USA).equipment.as_mut().unwrap().projects.iter_mut().find(|p|p.id==id).unwrap();
        j.minimum_days=150;j.tooling_days=30;j.work_days=30.05109489051095;
        j.recipe_per_unit=[0.0;12];j.recipe_per_unit[Commodity::Iron.idx()]=24.0;
        j.resources_used=[0.0;12];j.resources_used[Commodity::Iron.idx()]=0.030656934;
        resources::set_stockpile_for_test(&mut w,USA,Commodity::Iron,0.043298969);open_next(&mut w);
        let p=plan(&w,id);let before=job(&w,id).clone();
        assert_eq!(p.executable_work_days,0.0);assert!(!p.blockers.is_empty());tick_day(&mut w);
        assert_eq!(job(&w,id).work_days,before.work_days);assert_eq!(job(&w,id).resources_used,before.resources_used);
        assert_eq!(job(&w,id).spent_bn,before.spent_bn);
    }
    #[test]
    fn supply_national_target_cannot_assign_one_dollar_to_two_stock_blocked_projects() {
        let (mut w,d,r)=fixture();let a=start_production(&mut w,USA,&r,&d,2,1.0).unwrap();
        let b=start_production(&mut w,USA,&r,&d,2,1.0).unwrap();
        let tooling=job(&w,a).tooling_days;for _ in 0..tooling {next(&mut w);}open_next(&mut w);
        let rate=work_terms(job(&w,a)).rate;let p=w.nation_mut(USA).program_budget.as_mut().unwrap();
        p.available_bn[BUDGET_DEFENSE][3]=rate;p.prepaid_bn[BUDGET_DEFENSE][3]=0.0;
        resources::set_stockpile_for_test(&mut w,USA,Commodity::Iron,0.0);
        let pa=plan(&w,a);let pb=plan(&w,b);
        approx(pa.planned_work_days,1.0);approx(pb.planned_work_days,1.0);
        assert_eq!(pa.executable_work_days,0.0);assert_eq!(pb.executable_work_days,0.0);
        let before=crate::save(&w);let target=next_work_supply(&w,USA);
        approx(target.procurement_payment_bn,rate);assert_eq!(target.raw,pa.planned_day);
        assert_eq!(raw_supply_demand(&w,USA).next_work,target.raw);
        let reported=resources::tick_draw(&w,USA);
        for i in 0..12 {approx(reported[i],target.raw[i]);}
        assert_eq!(crate::save(&w),before);
    }
}
