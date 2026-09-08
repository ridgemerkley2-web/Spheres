//! Named construction presets expand into ordinary, independently managed jobs.
use crate::{production::{self, ProjectKind as K}, world::{NationId, WorldState}};

pub const INDUSTRIAL_STARTER: [K;4] = [K::CivilianIndustry,K::Generation,K::PowerGrid,K::ProcessingPlant];

pub fn refusal(w:&WorldState,nation:NationId,district:&str)->Option<String> {
    if !crate::construction_capacity::enabled(w) { return Some("Industrial presets require daily industry.".into()); }
    INDUSTRIAL_STARTER.into_iter().find_map(|kind| production::start_project_error(w,nation,district,kind))
}

pub fn preview(w:&WorldState,nation:NationId,district:&str)->serde_json::Value {
    let mut queued=w.clone();
    queued.production.construction_day=None;
    let ordered=start(&mut queued,nation,district);
    let mut after=w.clone();
    after.production.construction_day=None;
    after.production.operations.support_day=None;
    after.production.operations.receipts.clear();
    if ordered.is_ok() { for kind in INDUSTRIAL_STARTER { production::complete_capability(&mut after,district,kind); } }
    let components:Vec<_>=INDUSTRIAL_STARTER.into_iter().map(|kind|serde_json::json!({
        "kind":kind.key(),"name":production::catalog(kind).name,"capacity":1.0,
        "cost_bn":crate::industry::work_cost_bn(kind),
        "purpose":crate::construction_roles::role(kind).purpose,
    })).collect();
    let estimates:Vec<_>=ordered.as_ref().ok().into_iter().flatten().map(|id| {
        let p=queued.production.projects.iter().find(|p|p.id==*id).unwrap();
        production::estimated_days_left(&queued,p)
    }).collect();
    let eta=if !estimates.is_empty() && estimates.iter().all(Option::is_some) { estimates.into_iter().flatten().max() } else { None };
    serde_json::json!({
        "name":"Industrial starter preset","project_kind":"industry_preset","district":district,"capacity_micros":null,
        "cost_bn":INDUSTRIAL_STARTER.into_iter().map(crate::industry::work_cost_bn).sum::<f64>(),
        "minimum_days":null,"eta_days":eta,"can_start":ordered.is_ok(),"reason":ordered.err(),"components":components,
        "province_effects":[{"label":"Separate projects queued","before":0,"after":4,"unit":"projects",
            "detail":"Civilian Factory, Power Plant, Power Grid and Materials Plant. Assign capacity and priorities separately."}],
        "national_effects":[
            {"label":"Usable construction capacity","before":crate::construction_capacity::pool(w,nation).total_capacity,
                "after":crate::construction_capacity::pool(&after,nation).total_capacity,"unit":"capacity",
                "detail":"Current worker, power and operating-fund conditions after all four facilities finish."},
            {"label":"Generation capacity","before":crate::industry::power_capacity(w,nation),
                "after":crate::industry::power_capacity(&after,nation),"unit":"modeled power units/day",
                "detail":"Generation still needs fuel and an operating budget when dispatched."}],
        "operating_requirements":[{"label":"Factory operations","value":null,"unit":"",
            "detail":"Workers, ore, generating fuel and industry operating funds are needed after commissioning."}],
        "notes":["Four ordinary project contracts are queued atomically. Each keeps its own progress, funding and allocation.",
            "Completion depends on shared construction capacity, physical installation inputs and the daily budget."]
    })
}

pub fn start(w: &mut WorldState, nation: NationId, district: &str) -> Result<Vec<u32>,String> {
    if !crate::construction_capacity::enabled(w) {
        return Err("Industrial presets require the daily industry system.".into());
    }
    // Validate the complete queue change on one copy. A refused fourth project
    // cannot leave the first three ordered or charge any partial commitment.
    let mut planned = w.clone();
    let mut ids = Vec::new();
    for kind in INDUSTRIAL_STARTER {
        ids.push(production::start_project(&mut planned,nation,district,kind)?);
    }
    *w = planned;
    Ok(ids)
}
