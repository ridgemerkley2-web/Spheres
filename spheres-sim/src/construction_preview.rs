//! Read-only construction decisions. Completion is applied only to a cloned
//! world; quoted output is installed capacity, never an invented GDP return.
use crate::{
    clock, districts, industrial_modules as modules, industry, industry_operations as operations, logistics, manufacturing,
    production::{self, Project, ProjectKind as K},
    programs, resources,
    world::{NationId, WorldState},
};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, sync::OnceLock};

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct Effect {
    pub label: String,
    pub before: Option<f64>,
    pub after: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_label: Option<String>,
    pub unit: String,
    pub detail: String,
}

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct Requirement {
    pub label: String,
    pub value: Option<f64>,
    pub unit: String,
    pub detail: String,
}

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct ConstructionPreview {
    pub name: String,
    pub project_kind: K,
    pub district: String,
    pub capacity_micros: Option<u32>,
    pub cost_bn: f64,
    pub minimum_days: u32,
    pub eta_days: Option<u32>,
    pub can_start: bool,
    pub reason: Option<String>,
    pub province_effects: Vec<Effect>,
    pub national_effects: Vec<Effect>,
    pub operating_requirements: Vec<Requirement>,
    pub notes: Vec<String>,
}

fn effect(label: impl Into<String>, before: f64, after: f64, unit: &str, detail: &str) -> Effect {
    Effect {
        label: label.into(),
        before: Some(before),
        after: Some(after),
        after_label: None,
        unit: unit.into(),
        detail: detail.into(),
    }
}
fn requirement(
    label: impl Into<String>,
    value: Option<f64>,
    unit: &str,
    detail: &str,
) -> Requirement {
    Requirement {
        label: label.into(),
        value,
        unit: unit.into(),
        detail: detail.into(),
    }
}
fn flag(enabled: bool) -> f64 {
    if enabled {
        1.0
    } else {
        0.0
    }
}
fn owned_sum(w: &WorldState, nation: NationId, f: impl Fn(&WorldState, &str) -> f64) -> f64 {
    w.districts
        .iter()
        .filter(|(_, owner)| **owner == nation)
        .map(|(d, _)| f(w, d))
        .sum()
}
fn eligible_estate(w: &WorldState, district: &str) -> f64 {
    flag(modules::effective_capacity(w, district, K::CivilianIndustry) >= 1.0)
}
fn grid(w: &WorldState, district: &str) -> f64 {
    if operations::enabled(w) { operations::grid_capacity(w, district) }
    else { modules::effective_capacity(w, district, K::PowerGrid) * 5.0 }
}
fn research_capacity(w: &WorldState, district: &str) -> f64 {
    if industry::research_enabled(w) && !resources::district_contested(w, district) {
        production::level(w, district, K::ResearchCenter) as f64
            * industry::PROTOTYPE_WORK_PER_LEVEL_DAY
    } else {
        0.0
    }
}

#[derive(Default)]
struct PlantCapacity {
    intermediates: f64,
    capital_goods: f64,
    intermediate_input: f64,
    power: f64,
    raw: [f64; 12],
    cash_bn: f64,
}
fn plant_capacity(w: &WorldState, district: &str) -> PlantCapacity {
    let mut out = PlantCapacity::default();
    for kind in [K::ProcessingPlant, K::StarterIndustry, K::MachineryWorks] {
        let rate = industry::plant_rate(w, district, kind);
        let power = rate * industry::power_per_pack(w, district, kind);
        if kind == K::MachineryWorks {
            out.capital_goods += rate;
            out.intermediate_input += rate;
        } else {
            out.intermediates += rate;
        }
        out.power += power;
        let raw = industry::operating_recipe(kind, rate, power);
        for (total, amount) in out.raw.iter_mut().zip(raw) {
            *total += amount;
        }
        // Exact current civilian-operation and generating-service rates from
        // industry::tick_day. Raw purchases are separate from these cash bills.
        out.cash_bn += rate * 0.00001 + power * 0.000002;
    }
    out
}
fn national_plant(w: &WorldState, nation: NationId, f: impl Fn(&PlantCapacity) -> f64) -> f64 {
    owned_sum(w, nation, |world, district| {
        if resources::district_contested(world, district) {
            0.0
        } else {
            f(&plant_capacity(world, district))
        }
    })
}
fn add_plant_effects(
    out: &mut ConstructionPreview,
    w: &WorldState,
    after: &WorldState,
    nation: NationId,
    district: &str,
) {
    let before = plant_capacity(w, district);
    let next = plant_capacity(after, district);
    let conditional = "Rated output with sufficient operating funds, inputs, shared generation, local grid and storage; not guaranteed production or sales.";
    for (label, a, b, unit) in [
        (
            "Intermediate production capacity",
            before.intermediates,
            next.intermediates,
            "packs/day",
        ),
        (
            "Capital-goods production capacity",
            before.capital_goods,
            next.capital_goods,
            "packs/day",
        ),
        (
            "Power needed at full local output",
            before.power,
            next.power,
            "modeled power units/day",
        ),
        (
            "Operating bill at full local output",
            before.cash_bn,
            next.cash_bn,
            "$bn/day",
        ),
    ] {
        out.province_effects
            .push(effect(label, a, b, unit, conditional));
    }
    out.national_effects.push(effect(
        "National intermediate production capacity",
        national_plant(w, nation, |p| p.intermediates),
        national_plant(after, nation, |p| p.intermediates),
        "packs/day",
        conditional,
    ));
    out.national_effects.push(effect(
        "National capital-goods production capacity",
        national_plant(w, nation, |p| p.capital_goods),
        national_plant(after, nation, |p| p.capital_goods),
        "packs/day",
        conditional,
    ));
    if out.project_kind == K::Efficiency {
        out.province_effects.push(effect("Coal demand at full local output", before.raw[resources::Commodity::Coal.idx()],
            next.raw[resources::Commodity::Coal.idx()], &format!("{}/day",resources::Commodity::Coal.unit()),
            "Efficiency reduces generating fuel, not the coal used directly by processing. Savings occur only while the plant operates."));
        out.national_effects.push(effect(
            "Civilian power demand at full output",
            national_plant(w, nation, |p| p.power),
            national_plant(after, nation, |p| p.power),
            "modeled power units/day",
            "Same installed output with less generating power and fuel; no free extra output.",
        ));
    }
    let detail = if operations::enabled(w) {
        "Change at full rated output after completion. These are recurring operating inputs, separate from physical construction inputs and payments."
    } else { "Change at full rated output after completion. These are operating inputs; construction itself needs only funding." };
    for c in resources::ALL {
        let change = next.raw[c.idx()] - before.raw[c.idx()];
        if change.abs() > 1e-12 {
            out.operating_requirements.push(requirement(
                format!("Change in {} demand", c.name()),
                Some(change),
                &format!("{}/day", c.unit()),
                detail,
            ));
        }
    }
    if (next.intermediate_input - before.intermediate_input).abs() > 1e-12 {
        out.operating_requirements.push(requirement(
            "Change in intermediate-pack demand",
            Some(next.intermediate_input - before.intermediate_input),
            "packs/day",
            detail,
        ));
    }
    out.operating_requirements.push(requirement("Change in operating cash",Some(next.cash_bn-before.cash_bn),"$bn/day",
        "Full-output plant and generating-service bills, paid from Industry operating authority. Raw-stock purchases are additional; idle capacity is not billed."));
    out.operating_requirements.push(requirement("Shared national generation",Some(industry::power_capacity(after,nation)),"modeled power units/day",
        "Shared by civilian lines; having generation does not supply the required local grid, raw stocks or operating funds."));
    out.operating_requirements.push(requirement(
        "Civilian operations enabled",
        Some(flag(clock::is_daily(w) && w.rules.production_system && w.rules.resource_market)),
        "enabled",
        "Operating production requires daily simulation, production and the resource market. Rated capacity is idle while these rules are disabled.",
    ));
    out.operating_requirements.push(requirement(
        "Local grid ceiling after completion",
        Some(grid(after, district)),
        "modeled power units/day",
        conditional,
    ));
}

// Logistics exposes route plans but keeps edge helpers private. Read the same
// bundled graph and reproduce its segment_capacity formula, including the
// weaker district endpoint and terrain factors. Do not add edge ceilings and
// label the sum as achievable route throughput.
#[derive(Deserialize)]
struct MapNode {
    id: String,
    name: String,
    kind: String,
}
#[derive(Deserialize)]
struct MapEdge {
    a: String,
    b: String,
    kind: String,
}
#[derive(Deserialize)]
struct FreightMap {
    nodes: Vec<MapNode>,
    edges: Vec<MapEdge>,
}
fn freight_map() -> &'static FreightMap {
    static MAP: OnceLock<FreightMap> = OnceLock::new();
    MAP.get_or_init(|| {
        serde_json::from_str(logistics::EMBEDDED_NETWORK).expect("bundled logistics graph")
    })
}
fn terrain_factor(district: &str) -> f64 {
    match districts::terrain_of(district) {
        districts::TerrainClass::Mountain => 0.55,
        districts::TerrainClass::Highland => 0.78,
        districts::TerrainClass::Desert => 0.82,
        districts::TerrainClass::Wetland => 0.72,
        districts::TerrainClass::Tundra => 0.65,
        districts::TerrainClass::Lowland => 1.0,
    }
}
fn corridor_capacity(w: &WorldState, edge: &MapEdge, nodes: &BTreeMap<&str, &MapNode>) -> f64 {
    if !logistics::enabled(w) {
        return 0.0;
    }
    let ends: Vec<_> = [&edge.a, &edge.b]
        .into_iter()
        .filter(|id| nodes[id.as_str()].kind == "district")
        .collect();
    let level = ends
        .iter()
        .map(|id| production::level(w, id, K::Infrastructure))
        .min()
        .unwrap_or(0) as f64;
    let terrain = ends
        .iter()
        .map(|id| terrain_factor(id))
        .fold(1.0_f64, f64::min);
    25_000.0 * (1.0 + level * 0.20) * terrain * clock::month_fraction(w)
}
fn gateway_capacity(w: &WorldState, district: &str) -> f64 {
    if !logistics::enabled(w) || !logistics::has_terminal(district) {
        return 0.0;
    }
    45_000.0
        * (1.0 + production::level(w, district, K::FreightTerminal) as f64 * 0.25)
        * clock::month_fraction(w)
}
fn add_freight_effects(
    out: &mut ConstructionPreview,
    w: &WorldState,
    after: &WorldState,
    nation: NationId,
    district: &str,
) {
    let map = freight_map();
    let nodes: BTreeMap<_, _> = map.nodes.iter().map(|n| (n.id.as_str(), n)).collect();
    if out.project_kind == K::Infrastructure {
        let mut improved = 0.0;
        for edge in map.edges.iter().filter(|e| {
            (e.a == district || e.b == district) && e.kind != "sea" && e.kind != "terminal"
        }) {
            let a = corridor_capacity(w, edge, &nodes);
            let b = corridor_capacity(after, edge, &nodes);
            if b > a {
                improved += 1.0;
            }
            let other = if edge.a == district { &edge.b } else { &edge.a };
            out.province_effects.push(effect(format!("Corridor to {}",nodes[other.as_str()].name),a,b,"modeled tonnes/day",
                "Current-date edge ceiling. The weaker endpoint and terrain determine capacity; a route remains limited by its tightest segment and access rights."));
        }
        out.national_effects.push(effect("Mapped freight links improved",0.0,improved,"links",
            "Links incident to this province whose actual modeled capacity rises. Other links, sea capacity, route access and travel time are unchanged."));
        if improved == 0.0 {
            out.notes.push("This level does not increase a mapped land-corridor ceiling today: an adjacent endpoint, missing mapped link or disabled freight model limits the result.".into());
        }
    } else {
        out.province_effects.push(effect("Gateway freight ceiling",gateway_capacity(w,district),gateway_capacity(after,district),"modeled tonnes/day",
            "Each level adds 25% of the base gateway ceiling. Shared cargo, connecting land routes and sea bottlenecks can limit actual shipments."));
        out.national_effects.push(effect("Sum of owned gateway ceilings",owned_sum(w,nation,gateway_capacity),owned_sum(after,nation,gateway_capacity),"modeled tonnes/day",
            "Parallel gateway capacity only, not a promise of national shipment throughput; connecting routes can remain bottlenecks."));
    }
    out.operating_requirements.push(requirement("Physical freight model enabled",Some(flag(logistics::enabled(w))),"enabled",
        "Freight benefits require resource gates, the resource market, freight routes and physical logistics. This investment has no separate modeled facility upkeep."));
}

fn unpaid_contracts(w: &WorldState, nation: NationId) -> f64 {
    production::projects_for(w, nation)
        .map(|p| {
            let paid = w
                .production
                .industry
                .projects
                .get(&p.id)
                .map_or(industry::project_cost_bn(p) * p.progress_fraction(), |f| {
                    f.spent_bn
                });
            (industry::contract_cost_bn(w, p) - paid).max(0.0)
        })
        .sum::<f64>()
        + w.resources
            .mine_projects
            .iter()
            .filter(|p| p.started_by == nation)
            .filter_map(|p| {
                w.production
                    .industry
                    .mines
                    .get(&industry::mine_key(&p.district, p.commodity))
                    .map(|f| (p.investment_bn - f.spent_bn).max(0.0))
            })
            .sum::<f64>()
}

/// Preview this order at normal priority, with today's fiscal state and queue.
/// Before/after capacities assume successful completion, not instant output.
pub fn preview(
    w: &WorldState,
    nation: NationId,
    district: &str,
    kind: K,
    capacity_micros: Option<u32>,
) -> ConstructionPreview {
    let spec = production::catalog(kind);
    let chosen = (kind == K::StarterIndustry).then(|| {
        capacity_micros.unwrap_or_else(|| modules::recommended_capacity_micros(w, nation))
    });
    let scale = chosen.map_or(1.0, |v| v as f64 / modules::STANDARD_MICROS as f64);
    let cost = industry::work_cost_bn(kind) * scale;
    let legacy_minimum = if kind == K::StarterIndustry {
        (spec.total_days as f64 * scale)
            .ceil()
            .max(modules::MIN_CALENDAR_DAYS as f64) as u32
    } else {
        spec.total_days
    };
    let minimum = if crate::construction_capacity::enabled(w) {
        let multiplier = crate::construction_capacity::MAX_PER_PROJECT
            / crate::construction_capacity::CAPACITY_PER_WORK_DAY
            * (1.0 + production::level(w, district, K::Infrastructure) as f64
                * crate::construction_capacity::INFRASTRUCTURE_WORK_BONUS);
        let days=(spec.total_days as f64 * scale / multiplier).ceil().max(1.0) as u32;
        if kind==K::StarterIndustry { days.max(modules::MIN_CALENDAR_DAYS) } else { days }
    } else { legacy_minimum };
    let mut queued = w.clone();
    if operations::enabled(w) { queued.production.construction_day=None; }
    let started = if !clock::is_daily(w) {
        Err("This funding preview requires daily construction.".into())
    } else if let Some(micros) = chosen {
        let id = queued.production.next_id.max(1);
        modules::start(&mut queued, nation, district, micros).map(|()| id)
    } else {
        production::start_project(&mut queued, nation, district, kind)
    };
    let can_start = started.is_ok();
    let reason = started.as_ref().err().cloned();
    let mut eta = None;
    let mut scheduled_cash = 0.0;
    let mut schedule_reason = None;
    if let Ok(id) = started {
        if let Some(p) = queued.production.projects.iter().find(|p| p.id == id) {
            let quote = industry::project_finance(&queued, p).expect("daily construction quote");
            scheduled_cash = quote.daily_request_bn;
            schedule_reason = quote.reason;
            if quote.next_work_days > 1e-9 {
                eta =
                    Some(((p.total_days as f64 / quote.next_work_days).ceil() as u32).max(minimum));
            }
        }
    }
    let mut after = w.clone();
    if operations::enabled(w) {
        after.production.construction_day=None;
        after.production.operations.support_day=None;
        after.production.operations.last_day=None;
        after.production.operations.receipts.clear();
    }
    // Hypothetical completion can still inform a technology/prerequisite
    // decision. Invalid ownership, size and full sites must never overflow.
    let owned = w.districts.get(district) == Some(&nation);
    let valid_size = chosen.is_none_or(|v| (1..=modules::STANDARD_MICROS).contains(&v));
    let room = if let Some(micros) = chosen {
        modules::COMPONENTS.iter().all(|k| {
            modules::reserved_capacity(w, district, *k)
                .checked_add(micros)
                .is_some_and(|v| v <= modules::MAX_MICROS)
        })
    } else {
        production::level(w, district, kind) < production::MAX_PROVINCE_LEVEL
            && (!modules::COMPONENTS.contains(&kind)
                || modules::reserved_capacity(w, district, kind)
                    <= modules::MAX_MICROS - modules::STANDARD_MICROS)
    };
    if owned && valid_size && room {
        if let Some(micros) = chosen {
            let p = Project {
                id: 0,
                nation,
                district: district.into(),
                kind,
                priority: production::Priority::Normal,
                status: production::ProjectStatus::Building,
                reason: None,
                progress_days: spec.total_days as f64,
                total_days: spec.total_days,
                resources_used: [0.0; 12],
                capacity_micros: Some(micros),
                started_day: Some(clock::absolute_day(w)),
            };
            modules::complete(&mut after, &p);
        } else {
            production::complete_capability(&mut after, district, kind);
        }
    }
    let mut out=ConstructionPreview {name:spec.name.into(),project_kind:kind,district:district.into(),capacity_micros:chosen,
        cost_bn:cost,minimum_days:minimum,eta_days:eta,can_start,reason,province_effects:vec![],national_effects:vec![],operating_requirements:vec![],notes:vec![
            "After values describe completed facilities at the current rules, prices and ownership. Rated output requires operating inputs and funding; it is not guaranteed production, GDP, tax income or profit.".into(),
            "Construction is paid as work progresses. No raw materials, manufactured goods or political capital are charged to start this building; paid work is sunk if cancelled.".into(),
            "ETA holds today's allocation to this new normal-priority project constant. Queue completions, budget changes and fiscal renewals can change the finish date.".into(),
        ]};
    if let Some(why) = schedule_reason {
        out.notes.push(why);
    }
    if !can_start {
        out.notes.push("Construction is currently refused. Any changed capacity values are conditional on meeting the stated start requirements.".into());
    }
    out.province_effects.push(effect(
        if kind == K::StarterIndustry {
            "Installed starter capacity"
        } else {
            "Completed facility levels"
        },
        if kind == K::StarterIndustry {
            modules::capacity(w, district)
        } else {
            production::level(w, district, kind) as f64
        },
        if kind == K::StarterIndustry {
            modules::capacity(&after, district)
        } else {
            production::level(&after, district, kind) as f64
        },
        if kind == K::StarterIndustry {
            "standard packages"
        } else {
            "levels"
        },
        "Completed physical assets only; other queued projects are not assumed complete.",
    ));
    let committed = unpaid_contracts(w, nation);
    out.national_effects.push(effect("Unpaid construction commitments",committed,committed+if can_start {cost}else{0.0},"$bn","Remaining cash bills for this country's funded buildings and mines. No lump-sum treasury debit at order time."));
    out.national_effects.push(Effect {label:"Quoted contract / current annual GDP".into(),before:None,
        after:w.nation_opt(nation).filter(|n|n.gdp>0.0).map(|n|cost/n.gdp*100.0),after_label:None,unit:"% of annual GDP".into(),
        detail:"A scale comparison for the total contract, not an annual tax increase or projected GDP return.".into()});
    let cap = programs::construction_daily_budget_bn(w, nation);
    out.national_effects.push(effect(
        "National daily construction budget",
        cap,
        cap,
        "$bn/day",
        "Starting a project shares this budget; it does not increase the budget automatically.",
    ));
    out.national_effects.push(effect("Today's funding for this proposed project",0.0,scheduled_cash,"$bn/day","Exact opening-queue quote at normal priority, bounded by remaining capital authority and the shared daily cap."));

    let financial_rows = out.national_effects.len();
    match kind {
        K::Infrastructure | K::FreightTerminal => {
            add_freight_effects(&mut out, w, &after, nation, district)
        }
        K::CivilianIndustry if operations::enabled(w) => {},
        K::CivilianIndustry => {
            out.province_effects.push(effect("Full industrial plant prerequisite",eligible_estate(w,district),eligible_estate(&after,district),"eligible",
                "One whole paid estate allows Machinery Works and Materials Processing here, subject to their other start conditions."));
            out.national_effects.push(effect("Provinces meeting the estate prerequisite",owned_sum(w,nation,eligible_estate),owned_sum(&after,nation,eligible_estate),"provinces",
                "Enables future plant sites; estates themselves create no packs, construction-workforce bonus or automatic GDP."));
            out.operating_requirements.push(requirement("Separate estate operating bill",Some(0.0),"$bn/day","This enabling asset has no standalone modeled production or upkeep; installed plants have their own operating inputs."));
        }
        K::PowerGrid => {
            out.province_effects.push(effect("Local industrial grid ceiling",grid(w,district),grid(&after,district),"modeled power units/day","Local delivery capacity for civilian lines; national generation and operating funding are still required."));
            out.national_effects.push(effect("Sum of local industrial grid ceilings",owned_sum(w,nation,grid),owned_sum(&after,nation,grid),"modeled power units/day","Grid capacity stays in its province; unused local capacity cannot supply another province."));
            out.operating_requirements.push(requirement("National generation available",Some(industry::power_capacity(&after,nation)),"modeled power units/day","This is a delivery asset, not a generator. Operating plants pay for the power and fuel they actually use."));
        }
        K::Generation => {
            out.province_effects.push(effect(
                "Installed generation contribution",
                modules::effective_capacity(w, district, kind) * 10.0,
                modules::effective_capacity(&after, district, kind) * 10.0,
                "modeled power units/day",
                "Adds to the shared national civilian generation pool.",
            ));
            out.national_effects.push(effect("National civilian generation",industry::power_capacity(w,nation),industry::power_capacity(&after,nation),"modeled power units/day","Dispatchable modeled capacity; actual output is demand-, grid-, fuel- and funding-limited."));
            let delta = (industry::power_capacity(&after, nation)
                - industry::power_capacity(w, nation))
            .max(0.0);
            out.operating_requirements.push(requirement("Coal if all added generation is used",Some(delta*0.02),&format!("{}/day",resources::Commodity::Coal.unit()),"Charged only when civilian plants use this power; idle generation consumes no fuel."));
            out.operating_requirements.push(requirement("Generating service bill at full added output",Some(delta*0.000002),"$bn/day","Industry / Electricity operating authority; no bill when idle. Fuel purchases are separate."));
        }
        K::Warehouse => {
            for label in [
                "National intermediate-pack storage",
                "National capital-goods storage",
            ] {
                out.national_effects.push(effect(label,industry::goods_capacity(w,nation),industry::goods_capacity(&after,nation),"packs","Each warehouse level adds 250 to each shared national goods store. Existing inventory is preserved and no stock is granted."));
            }
            out.operating_requirements.push(requirement("Separate warehouse upkeep",Some(0.0),"$bn/day","No recurring warehouse invoice in the current model. More storage helps producers only when storage is their constraint."));
        }
        K::ArmsPlant => {
            out.province_effects.push(effect("Military manufacturing line slots",manufacturing::plant_slots(w,district) as f64,manufacturing::plant_slots(&after,district) as f64,"slots","One slot hosts one separately ordered equipment production line; the building itself delivers no equipment."));
            out.national_effects.push(effect("National military manufacturing slots",owned_sum(w,nation,|w,d|manufacturing::plant_slots(w,d) as f64),owned_sum(&after,nation,|w,d|manufacturing::plant_slots(w,d) as f64),"slots","All active lines still share the procurement budget. Extra slots do not create procurement funds or guarantee extra equipment."));
            out.operating_requirements.push(requirement("Equipment programme inputs",None,"programme-specific","A separately ordered, technologically eligible line needs procurement funding, its physical resource recipe and production lead time. Empty slots consume nothing."));
            out.operating_requirements.push(requirement("Military manufacturing enabled",Some(flag(w.rules.manufacturing_system&&w.rules.resource_market)),"enabled","Military manufacturing and the physical resource market must be enabled before a line can operate."));
        }
        K::ResearchCenter => {
            out.province_effects.push(effect("Prototype credit ceiling",research_capacity(w,district),research_capacity(&after,district),"acquisition-cost units/day","Conditional on useful focused research, Science operating authority and manufactured supplies; one target per center per day."));
            out.national_effects.push(effect("National prototype credit ceiling",owned_sum(w,nation,research_capacity),owned_sum(&after,nation,research_capacity),"acquisition-cost units/day","All centers also share each target's daily cap (25% of domain effort) and lifetime cap (20% of its current base research cost)."));
            out.operating_requirements.push(requirement("Prototype operations enabled",Some(flag(industry::research_enabled(w))),"enabled","Current research operations require daily simulation, the active industry system or Economic Competition, production and the resource market. Disabled operations generate zero prototype credit."));
            out.operating_requirements.push(requirement("Science operating cash per added level",Some(industry::PROTOTYPE_CASH_PER_LEVEL_DAY_BN),"$bn/day","Maximum daily bill while a full added level performs useful prototype work; no generic research-output or GDP bonus."));
            out.operating_requirements.push(requirement(
                "Intermediate packs per added level",
                Some(industry::PROTOTYPE_INTERMEDIATES_PER_LEVEL_DAY),
                "packs/day",
                "Consumed only by useful, funded prototype work.",
            ));
            out.operating_requirements.push(requirement(
                "Capital-goods packs per added level",
                Some(industry::PROTOTYPE_CAPITAL_PER_LEVEL_DAY),
                "packs/day",
                "Consumed only by useful, funded prototype work.",
            ));
        }
        K::MachineryWorks
        | K::ProcessingPlant
        | K::Automation
        | K::Efficiency
        | K::StarterIndustry => {
            add_plant_effects(&mut out, w, &after, nation, district);
            if kind == K::StarterIndustry {
                out.province_effects.push(effect(
                    "Full industrial plant prerequisite", eligible_estate(w,district), eligible_estate(&after,district), "eligible",
                    "The package contributes fractional estate capacity. Full Machinery Works and Processing Plant orders need one whole estate equivalent.",
                ));
                out.national_effects.push(effect(
                    "Provinces meeting the estate prerequisite", owned_sum(w,nation,eligible_estate), owned_sum(&after,nation,eligible_estate), "provinces",
                    "A partial starter workshop runs its own fractional processing line without granting a full-estate unlock prematurely.",
                ));
                out.province_effects.push(effect(
                    "Local industrial grid ceiling",
                    grid(w, district),
                    grid(&after, district),
                    "modeled power units/day",
                    "The paid package includes matching fractional local grid capacity.",
                ));
                out.national_effects.push(effect("National civilian generation",industry::power_capacity(w,nation),industry::power_capacity(&after,nation),"modeled power units/day","The paid package includes matching fractional generation; no operating inventory is granted."));
            }
        }
        K::OfficeDistrict | K::Shipyard | K::AdvancedIndustry => {
            if !operations::enabled(w) {
                out.operating_requirements.push(requirement("Industry rebuild enabled",Some(0.0),"enabled",
                    "This project operates with the daily industry rebuild. Its worker, power, operating-input and budget model is disabled in this campaign."));
            }
        },
    }
    if operations::enabled(w) {
        add_rebuild_effects(&mut out, w, &after, nation, district, kind);
    }
    // Lead the decision with this building's physical benefit; cost and budget
    // context follow the project-specific national capacity rows.
    for row in out.national_effects.iter_mut().take(financial_rows) {
        row.after_label = Some("If queued".into());
    }
    out.national_effects.rotate_left(financial_rows);
    out
}

fn conditional_construction_capacity(w: &WorldState, nation: NationId) -> f64 {
    operations::inherited_construction_capacity(w, nation).max(crate::construction_capacity::STARTER_CAPACITY)
        + owned_sum(w, nation, |w, d| operations::site(w, d, K::CivilianIndustry).operating_capacity)
            * crate::construction_capacity::CIVILIAN_CAPACITY_PER_LEVEL
}
fn add_rebuild_effects(out: &mut ConstructionPreview, w: &WorldState, after: &WorldState,
    nation: NationId, district: &str, kind: K) {
    let before = operations::site(w, district, kind);
    let next = operations::site(after, district, kind);
    let conditional = "Current-worker, power, operating-input and budget estimate after commissioning. The actual dated operating receipt can differ as competing activity uses shared supplies.";
    if next.jobs_required > 0.0 {
        out.province_effects.push(effect("Jobs required", before.jobs_required, next.jobs_required, "modeled jobs", "New sites share available hires. Skilled office, advanced-industry and research jobs also need the modeled qualified-worker pool."));
        out.province_effects.push(effect("Jobs currently supportable", before.jobs_filled, next.jobs_filled, "modeled jobs", conditional));
        out.province_effects.push(effect("Currently supportable operating levels", before.operating_capacity, next.operating_capacity, "levels", conditional));
        out.operating_requirements.push(requirement("Additional power at full operation", Some(next.power_required_daily - before.power_required_daily), "modeled power units/day", "Paid generating service consumes coal; national supply and the local grid must both cover the load."));
        out.operating_requirements.push(requirement("Current operating constraint", None, "condition", &next.reason));
    }
    match kind {
        K::CivilianIndustry | K::StarterIndustry => {
            out.national_effects.push(effect("Usable national construction capacity", conditional_construction_capacity(w, nation), conditional_construction_capacity(after, nation), "construction capacity", "Every fully staffed civilian factory supplies 10 allocatable capacity. Ten capacity perform one physical work-day; shared funds and construction inputs can still slow the assigned project."));
            out.operating_requirements.push(requirement("Operating bill per added civilian factory", Some(operations::OPERATING_CASH_LEVEL_DAY_BN), "$bn/day", "Industry operating funds pay factory services; electricity has its own service and fuel bill. The starting construction service lets small countries build before commissioning their first factory."));
        },
        K::OfficeDistrict => {
            let a = operations::office_annual_value_added(w, district, before.operating_capacity);
            let b = operations::office_annual_value_added(after, district, next.operating_capacity);
            out.province_effects.push(effect("Expected annual service value added", a, b, "$bn/year", "Actual staffed service output less purchased intermediates and power. Demand is capped against the existing economy; only settled output enters province GDP."));
            out.national_effects.push(effect("Expected annual tax from these offices", operations::annual_tax_estimate(w, nation, a), operations::annual_tax_estimate(after, nation, b), "$bn/year", "The existing fiscal tax share applied to additional service value added. GDP is an annual output rate; this estimate is neither immediate cash nor a separate treasury payment."));
            out.operating_requirements.push(requirement("Intermediate packs per added office level", Some(operations::OFFICE_INTERMEDIATES_DAY), "packs/day", "Purchased business inputs are consumed only with a complete staffed and funded service bundle."));
            out.operating_requirements.push(requirement("Office operating cash per added level", Some(operations::OPERATING_CASH_LEVEL_DAY_BN), "$bn/day", "Industry operating funding; generating services and purchased raw inputs are additional."));
        },
        K::Shipyard => {
            out.province_effects.push(effect("Naval production line slots", manufacturing::naval_slots(w, district) as f64, manufacturing::naval_slots(after, district) as f64, "slots", "Coastal shipyards host assigned naval equipment lines. A naval programme still needs technology, procurement money, physical inputs and production lead time."));
            out.national_effects.push(effect("National naval production line slots", owned_sum(w, nation, |w,d| manufacturing::naval_slots(w,d) as f64), owned_sum(after, nation, |w,d| manufacturing::naval_slots(w,d) as f64), "slots", "Extra shipbuilding capacity does not automatically order a ship or increase the procurement budget."));
        },
        K::AdvancedIndustry => {
            out.province_effects.push(effect("Advanced component capacity", operations::advanced_output_daily(w,district,before.installed_capacity), operations::advanced_output_daily(after,district,next.installed_capacity), "advanced components/day", "Rated component output uses the currently assigned manufacturing company's work rate. Advanced military programmes require these components; machinery remains the source of capital-goods packs."));
            out.province_effects.push(effect("Currently supportable advanced component output", operations::advanced_output_daily(w,district,before.operating_capacity), operations::advanced_output_daily(after,district,next.operating_capacity), "advanced components/day", conditional));
            out.national_effects.push(effect("National advanced component capacity", owned_sum(w,nation,|w,d|operations::advanced_output_daily(w,d,production::level(w,d,kind) as f64)), owned_sum(after,nation,|w,d|operations::advanced_output_daily(w,d,production::level(w,d,kind) as f64)), "advanced components/day", "Rated commissioned output at current contractor terms. Staffing, power, input shortages and operating funds still constrain actual production."));
            out.operating_requirements.push(requirement("Intermediates per added advanced level", Some(operations::intermediate_requirement(after,nation,district,kind,1.0)), "packs/day", "Current contractor work and input rates apply to the full output recipe. Missing supplies reduce actual production without erasing this requirement."));
            out.operating_requirements.push(requirement("Advanced operating cash per added level", Some(operations::operating_cash_required(after,nation,district,kind,1.0)), "$bn/day", "Includes the assigned manufacturing company's service fee. Electricity and its operator fees are funded separately; charges follow actual supplied work."));
            let raw = operations::company_raw_recipe(after,nation,district,kind,1.0,operations::power_per_level(after,district,kind));
            for c in resources::ALL {
                if raw[c.idx()] > 0.0 { out.operating_requirements.push(requirement(c.name(), Some(raw[c.idx()]), &format!("{}/day",c.unit()), "Required raw input at full added output; supplies can be produced domestically or imported through Resources.")); }
            }
        },
        K::Warehouse => {
            out.national_effects.push(effect("National advanced-component storage", operations::advanced_component_capacity(w,nation), operations::advanced_component_capacity(after,nation), "components", "Warehouse capacity applies separately to advanced components; this upgrade adds no inventory."));
        },
        K::Generation | K::PowerGrid | K::Efficiency => {
            let usable = |world: &WorldState| owned_sum(world,nation,|w,d| production::PROJECT_KINDS.iter()
                .filter(|k| operations::jobs_per_level(**k)>0.0)
                .map(|k| {let s=operations::site(w,d,*k);s.installed_capacity*s.worker_fraction.min(s.power_fraction)}).sum());
            out.national_effects.push(effect("Factory and office levels supported by workers and power", usable(w), usable(after), "operating levels", "This isolates the power/workforce improvement. Operating inputs, department funding and demand may still prevent that potential output from being realized."));
        },
        K::Infrastructure => {
            let multiplier=|w:&WorldState|1.0+production::level(w,district,K::Infrastructure) as f64*crate::construction_capacity::INFRASTRUCTURE_WORK_BONUS;
            out.province_effects.push(effect("Local construction work multiplier", multiplier(w), multiplier(after), "times assigned work", "Infrastructure increases physical work per assigned capacity at this location. Payments and installation materials rise only with work actually performed."));
        },
        _ => {},
    }
    out.national_effects.push(effect("Power demand from commissioned activity", operations::power_required(w,nation), operations::power_required(after,nation), "modeled power units/day", "Opening inherited activity is already served and reserved; only explicitly estimated spare inherited utility capacity and built generators serve additional demand."));
    out.notes.retain(|s| !s.starts_with("Construction is paid as work progresses."));
    out.notes.push("Construction shares the national capacity allocation, daily capital budget, raw materials and manufactured inputs. Missing inputs reduce physical work and cash payments together; import supplies or expand domestic production. Paid work remains sunk on cancellation.".into());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{init::world_1990, save, world::GameRules};
    const USA: NationId = NationId::USA;
    fn prepared() -> (WorldState, String) {
        let mut w = world_1990(GameRules {
            daily_simulation: true,
            economic_competition: true,
            production_system: true,
            resource_market: true,
            resource_gates: true,
            physical_logistics: true,
            logistics_routes: true,
            manufacturing_system: true,
            ..Default::default()
        });
        w.player = Some(USA);
        programs::set_construction_budget(&mut w, USA, 1.0).unwrap();
        let d = w
            .districts
            .iter()
            .find(|(_, n)| **n == USA)
            .unwrap()
            .0
            .clone();
        (w, d)
    }
    fn row<'a>(rows: &'a [Effect], label: &str) -> &'a Effect {
        rows.iter().find(|r| r.label == label).expect(label)
    }
    fn near(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-10, "{a} != {b}");
    }
    #[test]
    fn advanced_company_forecasts_match_output_recipe_and_keep_unfunded_shortages() {
        use crate::companies::{self,CompanySector,CompanyTarget};
        use crate::commerce::Good;
        for saving in [false,true] {
            let (mut w,d)=prepared();
            w.rules.industry_rebuild=true;
            production::complete_capability(&mut w,&d,K::AdvancedIndustry);
            companies::enable(&mut w);
            let id=w.companies.roster.iter().filter(|c|c.nation==USA&&c.sector==CompanySector::Manufacturing)
                .max_by(|a,b|if saving {a.input_saving.total_cmp(&b.input_saving)}else{a.work_bonus.total_cmp(&b.work_bonus)}).unwrap().id;
            let target=CompanyTarget::Facility{district:d.clone(),sector:CompanySector::Manufacturing};
            companies::assign(&mut w,USA,id,target.clone()).unwrap();
            let company=companies::modifiers(&w,USA,&target);
            let output=operations::ADVANCED_OUTPUT_DAY*company.work_rate;
            let inputs=operations::ADVANCED_INTERMEDIATES_DAY*company.work_rate*company.input_rate;
            let before=save(&w);
            let quoted=preview(&w,USA,&d,K::AdvancedIndustry,None);
            let capacity=row(&quoted.province_effects,"Advanced component capacity");
            near(capacity.before.unwrap(),output);
            near(capacity.after.unwrap(),output*2.0);
            near(row(&quoted.national_effects,"National advanced component capacity").after.unwrap(),output*2.0);
            near(row(&quoted.province_effects,"Currently supportable advanced component output").after.unwrap(),0.0);
            let requirement_value=|label:&str|quoted.operating_requirements.iter().find(|r|r.label==label).unwrap().value.unwrap();
            near(requirement_value("Intermediates per added advanced level"),inputs);
            near(requirement_value("Advanced operating cash per added level"),operations::OPERATING_CASH_LEVEL_DAY_BN*company.work_rate*(1.0+company.fee_rate));
            assert!((requirement_value(resources::Commodity::Copper.name())-0.02*company.work_rate*company.input_rate).abs()<1e-9);
            assert_eq!(save(&w),before,"company previews are pure");
            production::start_project(&mut w,USA,&d,K::AdvancedIndustry).unwrap();
            let before_plan=save(&w);
            let plan=crate::industry_planning::plan(&w,USA);
            let components=plan.goods.iter().find(|g|g.good==Good::AdvancedComponents).unwrap();
            let intermediates=plan.goods.iter().find(|g|g.good==Good::Intermediates).unwrap();
            near(components.installed_daily,output);
            near(components.committed_daily,output);
            near(intermediates.domestic_daily,inputs*2.0);
            assert!(intermediates.expansion_daily>0.0,"missing inputs and operating cash must not erase demand");
            assert_eq!(save(&w),before_plan,"company capacity planning is pure");
        }
    }
    #[test]
    fn every_kind_preview_is_pure_and_has_cost_effects_and_operating_requirements() {
        let (w, d) = prepared();
        let before = save(&w);
        for kind in production::PROJECT_KINDS {
            let p = preview(&w, USA, &d, kind, Some(10_000));
            assert_eq!(save(&w), before, "{kind:?} mutated the source world");
            assert_eq!(p.project_kind, kind);
            assert!(
                !p.province_effects.is_empty()
                    && !p.national_effects.is_empty()
                    && !p.operating_requirements.is_empty()
            );
            near(
                p.cost_bn,
                industry::work_cost_bn(kind)
                    * if kind == K::StarterIndustry {
                        0.01
                    } else {
                        1.0
                    },
            );
            assert!(p.minimum_days > 0);
            assert_eq!(row(&p.national_effects,"Unpaid construction commitments").after_label.as_deref(),Some("If queued"));
            assert!(p.province_effects.iter().all(|row|row.after_label.is_none()));
            serde_json::to_value(&p).unwrap();
        }
    }
    #[test]
    fn foreign_province_refuses_without_awarding_national_capacity() {
        let (w, _) = prepared();
        let d = w.districts.iter().find(|(_, n)| **n != USA).unwrap().0;
        let before = save(&w);
        let p = preview(&w, USA, d, K::Generation, None);
        assert!(!p.can_start && p.reason.is_some() && p.eta_days.is_none());
        let r = row(&p.national_effects, "National civilian generation");
        assert_eq!(r.before, r.after);
        assert_eq!(save(&w), before);
    }
    #[test]
    fn module_preview_matches_real_fractional_completion_and_lead_time() {
        let (w, d) = prepared();
        let p = preview(&w, USA, &d, K::StarterIndustry, Some(10_000));
        let mut completed = w.clone();
        modules::start(&mut completed, USA, &d, 10_000).unwrap();
        let project = completed.production.projects.last().unwrap().clone();
        modules::complete(&mut completed, &project);
        assert!(p.can_start);
        assert_eq!(p.minimum_days, 90);
        assert_eq!(p.eta_days, Some(90));
        near(
            row(&p.national_effects, "National civilian generation")
                .after
                .unwrap(),
            industry::power_capacity(&completed, USA),
        );
        near(
            row(&p.province_effects, "Intermediate production capacity")
                .after
                .unwrap(),
            industry::plant_rate(&completed, &d, K::StarterIndustry),
        );
        near(
            row(&p.province_effects, "Local industrial grid ceiling")
                .after
                .unwrap(),
            grid(&completed, &d),
        );
    }
    #[test]
    fn all_standard_capability_deltas_match_actual_completion_helpers() {
        let (w, d) = prepared();
        for kind in production::PROJECT_KINDS
            .into_iter()
            .filter(|k| *k != K::StarterIndustry)
        {
            let p = preview(&w, USA, &d, kind, None);
            let mut completed = w.clone();
            production::complete_capability(&mut completed, &d, kind);
            near(
                row(&p.province_effects, "Completed facility levels")
                    .after
                    .unwrap(),
                production::level(&completed, &d, kind) as f64,
            );
            match kind {
                K::PowerGrid => near(
                    row(&p.province_effects, "Local industrial grid ceiling")
                        .after
                        .unwrap(),
                    grid(&completed, &d),
                ),
                K::Generation => near(
                    row(&p.national_effects, "National civilian generation")
                        .after
                        .unwrap(),
                    industry::power_capacity(&completed, USA),
                ),
                K::Warehouse => near(
                    row(&p.national_effects, "National intermediate-pack storage")
                        .after
                        .unwrap(),
                    industry::goods_capacity(&completed, USA),
                ),
                K::ArmsPlant => near(
                    row(&p.province_effects, "Military manufacturing line slots")
                        .after
                        .unwrap(),
                    manufacturing::plant_slots(&completed, &d) as f64,
                ),
                K::ResearchCenter => near(
                    row(&p.province_effects, "Prototype credit ceiling")
                        .after
                        .unwrap(),
                    research_capacity(&completed, &d),
                ),
                _ => {}
            }
        }
    }
    #[test]
    fn zero_budget_has_no_eta_and_no_invented_gdp_gain() {
        let (mut w, d) = prepared();
        programs::set_construction_budget(&mut w, USA, 0.0).unwrap();
        let p = preview(&w, USA, &d, K::Infrastructure, None);
        assert!(p.can_start);
        assert_eq!(p.eta_days, None);
        assert_eq!(
            row(
                &p.national_effects,
                "Today's funding for this proposed project"
            )
            .after,
            Some(0.0)
        );
        near(
            row(&p.national_effects, "Quoted contract / current annual GDP")
                .after
                .unwrap(),
            p.cost_bn / w.nation(USA).gdp * 100.0,
        );
        assert!(p
            .notes
            .iter()
            .any(|s| s.contains("not guaranteed production, GDP")));
    }
    #[test]
    fn automation_and_efficiency_use_the_installed_plant_operating_recipe() {
        let (mut w, d) = prepared();
        for k in [
            K::CivilianIndustry,
            K::PowerGrid,
            K::Generation,
            K::ProcessingPlant,
            K::MachineryWorks,
        ] {
            production::complete_capability(&mut w, &d, k);
        }
        for kind in [K::Automation, K::Efficiency] {
            let p = preview(&w, USA, &d, kind, None);
            let mut a = w.clone();
            production::complete_capability(&mut a, &d, kind);
            let b = plant_capacity(&w, &d);
            let after = plant_capacity(&a, &d);
            near(
                row(&p.province_effects, "Power needed at full local output")
                    .before
                    .unwrap(),
                b.power,
            );
            near(
                row(&p.province_effects, "Power needed at full local output")
                    .after
                    .unwrap(),
                after.power,
            );
            near(
                row(&p.province_effects, "Operating bill at full local output")
                    .after
                    .unwrap(),
                after.cash_bn,
            );
            if kind == K::Efficiency {
                assert!(after.power < b.power);
                near(after.intermediates, b.intermediates);
            } else {
                assert!(after.intermediates > b.intermediates && after.power > b.power);
            }
        }
    }
    #[test]
    fn invalid_or_full_module_capacity_never_overflows_or_mutates() {
        let (mut w, d) = prepared();
        w.production
            .industry
            .modules
            .insert(d.clone(), modules::MAX_MICROS);
        let before = save(&w);
        for size in [0, 1, 1_000_001, u32::MAX] {
            let p = preview(&w, USA, &d, K::StarterIndustry, Some(size));
            assert!(!p.can_start);
            let r = row(&p.province_effects, "Installed starter capacity");
            assert_eq!(r.before, r.after);
            assert_eq!(save(&w), before);
        }
    }

    #[test]
    fn infrastructure_respects_the_weaker_mapped_endpoint() {
        let (mut w, _) = prepared();
        let map = freight_map();
        let nodes: BTreeMap<_, _> = map.nodes.iter().map(|n| (n.id.as_str(), n)).collect();
        let edge = map
            .edges
            .iter()
            .find(|e| {
                e.kind != "sea"
                    && e.kind != "terminal"
                    && nodes[e.a.as_str()].kind == "district"
                    && nodes[e.b.as_str()].kind == "district"
            })
            .unwrap();
        w.districts.insert(edge.a.clone(), USA);
        w.districts.insert(edge.b.clone(), USA);
        for _ in 0..2 {
            production::complete_capability(&mut w, &edge.a, K::Infrastructure);
        }
        let label = format!("Corridor to {}", nodes[edge.b.as_str()].name);
        let blocked = preview(&w, USA, &edge.a, K::Infrastructure, None);
        let old = row(&blocked.province_effects, &label);
        assert_eq!(
            old.before, old.after,
            "Improving the stronger endpoint cannot bypass the weaker neighbor"
        );
        for _ in 0..3 {
            production::complete_capability(&mut w, &edge.b, K::Infrastructure);
        }
        let improves = preview(&w, USA, &edge.a, K::Infrastructure, None);
        let new = row(&improves.province_effects, &label);
        assert!(
            new.after.unwrap() > new.before.unwrap(),
            "The formerly weak endpoint now improves the corridor"
        );
    }

    #[test]
    fn disabled_research_pilot_is_explicit_and_promises_no_credit() {
        let (mut w, d) = prepared();
        w.rules.economic_competition = false;
        let p = preview(&w, USA, &d, K::ResearchCenter, None);
        assert!(p.can_start);
        let r = row(&p.province_effects, "Prototype credit ceiling");
        assert_eq!(r.before, Some(0.0));
        assert_eq!(r.after, Some(0.0));
        assert_eq!(
            p.operating_requirements
                .iter()
                .find(|r| r.label == "Prototype operations enabled")
                .unwrap()
                .value,
            Some(0.0)
        );
    }
}
