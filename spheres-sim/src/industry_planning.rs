//! One pure national capacity association for AI and the player read model.
//! This is planning, not an extra consumption/GDP channel. Inherited industrial
//! estimates use a separate annual-value-added ledger, never usable pack stock.
use crate::{
    clock,
    commerce::{self, Good},
    industrial_modules, industry,
    production::{self, ProjectKind as K},
    resources,
    world::{NationId, WorldState},
};
use serde::Serialize;

/// MODEL policy: 25% spare capacity, and no new lines while existing stock plus
/// paid inbound lots cover 90 days of current use. These are not spending flows.
pub const CAPACITY_HEADROOM: f64 = 1.25;
pub const STOCK_COVER_DAYS: f64 = 90.0;
pub const EXPORT_LOOKBACK_DAYS: i32 = 90;
const EPS: f64 = 1e-9;
fn is_zero(value: &f64) -> bool { *value == 0.0 }

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct GoodsBalance {
    pub good: Good,
    pub installed_daily: f64,
    /// Additional output already ordered, not output currently available.
    pub committed_daily: f64,
    pub domestic_daily: f64,
    pub export_daily: f64,
    pub demand_daily: f64,
    pub stock: f64,
    pub incoming: f64,
    /// Finite domestic orders, not installed factories or paid inbound freight.
    #[serde(skip_serializing_if = "is_zero")]
    pub contracted_daily: f64,
    #[serde(skip_serializing_if = "is_zero")]
    pub contracted_remaining: f64,
    pub expansion_daily: f64,
    pub status: String,
    pub reason: String,
}
#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct ProvinceCapacity {
    pub district: String,
    pub estate: f64,
    pub estate_committed: f64,
    pub processing_daily: f64,
    pub processing_committed_daily: f64,
    pub machinery_daily: f64,
    pub machinery_committed_daily: f64,
    pub grid_daily: f64,
    pub grid_committed_daily: f64,
    pub power_required_daily: f64,
    /// Demand on the same grid/generation, not a second power endowment.
    #[serde(skip_serializing_if = "is_zero")]
    pub materials_power_required_daily: f64,
    pub contested: bool,
}
#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct CapacityPlan {
    pub goods: Vec<GoodsBalance>,
    pub provinces: Vec<ProvinceCapacity>,
    pub generation_daily: f64,
    pub generation_committed_daily: f64,
    pub power_required_daily: f64,
    pub storage: f64,
    pub storage_committed: f64,
    /// Structural industrial estimates, not additional pack availability.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub inherited_sectors: Vec<SectorBalance>,
}

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct SectorBalance {
    pub key: String,
    pub name: String,
    pub inherited_factory_equivalents: f64,
    pub inherited_capacity_annual_bn: f64,
    pub funded_capacity_annual_bn: f64,
    pub committed_capacity_annual_bn: f64,
    pub output_annual_bn: f64,
    pub total_capacity_annual_bn: f64,
    pub expansion_annual_bn: f64,
    pub pressure: f64,
    pub status: String,
    pub reason: String,
}

/// The same constant-price gross output less inputs used by factory receipts.
/// Potential throughput is not realized GDP or sales. Generating coal belongs
/// to the power producer: deduct purchased power here, not that coal twice.
fn factory_value_added_per_pack(w: &WorldState, district: &str, kind: K) -> Option<f64> {
    let power = industry::power_per_pack(w, district, kind);
    let mut raw = industry::operating_recipe(kind, 1.0, power);
    raw[resources::Commodity::Coal.idx()] =
        (raw[resources::Commodity::Coal.idx()] - power * 0.02).max(0.0);
    let mut inputs = power * crate::gdp_projects::POWER_UNIT_BN;
    for c in resources::ALL {
        if raw[c.idx()] > 0.0 {
            inputs += raw[c.idx()] * resources::unit_price_bn(c)?;
        }
    }
    let gross = if kind == K::MachineryWorks {
        inputs += crate::gdp_projects::INTERMEDIATE_PACK_BN;
        crate::gdp_projects::CAPITAL_PACK_BN
    } else {
        crate::gdp_projects::INTERMEDIATE_PACK_BN
    };
    Some((gross - inputs).max(0.0))
}

fn structural_sectors(w: &WorldState, nation: NationId, plan: &CapacityPlan) -> Vec<SectorBalance> {
    let Some(inherited) = crate::starting_industry::snapshot(w, nation) else {
        return vec![];
    };
    // Read the reconciled public ledger, not pre-transfer saved receipts that
    // may exceed a new owner's GDP before the next settlement rebases them.
    let projects = crate::province_economy::snapshot(w, nation)
        .map(|ledger| ledger.projects).unwrap_or_default();
    inherited.groups.iter().map(|group| {
        // Only these two products have commissioned/queued physical recipes.
        // Food, chemicals and other factories remain inherited sector records.
        let kind = match group.key {
            "materials" => Some(K::ProcessingPlant),
            "machinery_electronics" => Some(K::MachineryWorks),
            _ => None,
        };
        let mut funded = 0.0;
        let mut committed = 0.0;
        let mut actual = 0.0;
        if let Some(kind) = kind {
            for site in &plan.provinces {
                if let Some(value) = factory_value_added_per_pack(w, &site.district, kind) {
                    let (installed, queued) = if kind == K::ProcessingPlant {
                        (site.processing_daily, site.processing_committed_daily)
                    } else {
                        (site.machinery_daily, site.machinery_committed_daily)
                    };
                    funded += installed * value * crate::gdp_projects::DAYS_PER_ACCOUNTING_YEAR;
                    committed += queued * value * crate::gdp_projects::DAYS_PER_ACCOUNTING_YEAR;
                }
            }
                actual = projects.iter().filter(|r| r.counted && r.sector == "manufacturing"
                    && (r.kind == kind.key() || (kind == K::ProcessingPlant
                        && (r.kind == K::StarterIndustry.key() || r.kind == "inherited_materials"))))
                    .map(crate::gdp_projects::incremental_gdp_bn).sum();
        }
        let capacity = group.capacity_annual_bn + funded + committed;
        let output = group.current_output_annual_bn + actual;
        let raw_gap = (output * CAPACITY_HEADROOM - capacity).max(0.0);
        let gap = if raw_gap <= EPS * capacity.max(1.0) { 0.0 } else { raw_gap };
        SectorBalance {
            key: group.key.into(), name: group.name.into(),
            inherited_factory_equivalents: group.factory_equivalents,
            inherited_capacity_annual_bn: group.capacity_annual_bn,
            funded_capacity_annual_bn: funded, committed_capacity_annual_bn: committed,
            output_annual_bn: output, total_capacity_annual_bn: capacity,
            expansion_annual_bn: gap,
            pressure: if capacity > EPS { gap / capacity } else if gap > EPS { 1.0 } else { 0.0 },
            status: if gap > 0.0 { "structural_pressure" } else { "structural_capacity_covered" }.into(),
            reason: if gap > 0.0 {
                "Output has grown beyond the estimated inherited and funded capacity buffer. This is a sector priority signal, not an order or proof of pack demand."
            } else {
                "Estimated inherited, funded and queued capacity covers current sector output with a 25% buffer. Pack-producing projects still need actual demand, inputs, power and funds."
            }.into(),
        }
    }).collect()
}

/// Inherited capacity ranks two ALREADY justified physical investments. It may
/// never masquerade as a warehouse, consume raw inputs, or suppress a necessary
/// first pack supply chain. Equal/missing signals preserve the legacy order.
pub fn expansion_order(plan: &CapacityPlan) -> [K; 2] {
    let pressure = |key: &str| plan.inherited_sectors.iter()
        .find(|s| s.key == key).map_or(0.0, |s| s.pressure);
    if pressure("machinery_electronics") > pressure("materials") + EPS {
        [K::MachineryWorks, K::ProcessingPlant]
    } else {
        [K::ProcessingPlant, K::MachineryWorks]
    }
}

/// A legacy contract accepted within the window is conservative evidence:
/// every unit already delivered from it necessarily arrived within the window.
/// New dated rows capture late deliveries from older contracts too. Subtract
/// all known dated units from the legacy fallback so none are counted twice.
pub fn delivered_daily(w: &WorldState, nation: NationId, good: Good) -> f64 {
    let Some(c) = &w.commerce else {
        return 0.0;
    };
    let today = clock::absolute_day(w);
    let mut quantity: f64 = c
        .goods_deliveries
        .iter()
        .filter(|r| {
            r.seller == nation
                && r.good == good
                && r.day > today - EXPORT_LOOKBACK_DAYS
                && r.day <= today
        })
        .map(|r| r.quantity)
        .sum();
    for contract in c.contracts.iter().filter(|r| {
        r.seller == nation
            && r.good == good
            && r.accepted_day > today - EXPORT_LOOKBACK_DAYS
            && r.accepted_day <= today
    }) {
        let known: f64 = c
            .goods_deliveries
            .iter()
            .filter(|r| r.contract == contract.id)
            .map(|r| r.quantity)
            .sum();
        quantity += (contract.delivered_quantity - known).max(0.0);
    }
    quantity / EXPORT_LOOKBACK_DAYS as f64
}

fn balance(
    w: &WorldState,
    nation: NationId,
    good: Good,
    installed: f64,
    committed: f64,
    domestic: f64,
) -> GoodsBalance {
    let exports = delivered_daily(w, nation, good);
    let demand = domestic + exports;
    let stock = commerce::stock(w, nation, good);
    let incoming = commerce::pending(w, nation, good);
    let contracted_daily = if good == Good::Intermediates { crate::materials::reserved_daily(w,nation) } else { 0.0 };
    let contracted_remaining = if good == Good::Intermediates { crate::materials::pending(w,nation) } else { 0.0 };
    let gap = (demand * CAPACITY_HEADROOM - installed - committed - contracted_daily).max(0.0);
    let (status, reason, expansion) = if demand <= EPS {
        ("no_demand", "No current use or delivered export demand. Use or sell existing output before adding more capacity.".to_string(),0.0)
    } else if stock + incoming + contracted_remaining + EPS >= demand * STOCK_COVER_DAYS {
        ("stock_covered", "Stock, paid incoming goods and finite domestic orders already cover at least 90 days of current demand. Use that supply before adding another line.".into(),0.0)
    } else if gap <= EPS && contracted_daily > EPS {
        ("contract_covered", "Existing factories and finite Materials contracts cover current use. Orders still require inputs, power and funding; review capacity again when they expire.".into(),0.0)
    } else if gap <= EPS && committed > EPS {
        ("already_committed", "Existing capacity plus projects already under construction cover demand with the planning buffer. Finish paid work first.".into(),0.0)
    } else if gap <= EPS {
        ("capacity_covered", "Existing factories cover demand with the planning buffer. If output is limited or waiting, restore inputs, power or funding instead of duplicating the factory.".into(),0.0)
    } else {
        ("room_to_grow",format!("Current demand supports up to {:.4} additional packs/day, after counting existing and queued capacity and a 25% planning buffer. A build still needs inputs, power and funding.",gap),gap)
    };
    GoodsBalance {
        good,
        installed_daily: installed,
        committed_daily: committed,
        domestic_daily: domestic,
        export_daily: exports,
        demand_daily: demand,
        stock,
        incoming,
        contracted_daily,
        contracted_remaining,
        expansion_daily: expansion,
        status: status.into(),
        reason,
    }
}

pub fn plan(w: &WorldState, nation: NationId) -> CapacityPlan {
    let mut out = CapacityPlan {
        goods: vec![],
        provinces: vec![],
        generation_daily: 0.0,
        generation_committed_daily: 0.0,
        power_required_daily: 0.0,
        storage: industry::goods_capacity(w, nation),
        storage_committed: 0.0,
        inherited_sectors: vec![],
    };
    let mut construction = industry::Goods::default();
    let mut labs = 0.0;
    let mut labs_committed = 0.0;
    // The allocator walks the global construction queue. Resolve it once,
    // not again for every project/province in this read-only country plan.
    let work_plans = industry::project_plans(w);
    for (d, owner) in &w.districts {
        if *owner != nation {
            continue;
        }
        let contested = resources::district_contested(w, d);
        let mut pending = std::collections::BTreeMap::<K, f64>::new();
        for p in production::projects_for(w, nation).filter(|p| &p.district == d) {
            if p.kind == K::StarterIndustry {
                let scale = industrial_modules::scale(p);
                for kind in industrial_modules::COMPONENTS {
                    *pending.entry(kind).or_default() += scale;
                }
            } else {
                *pending.entry(p.kind).or_default() += 1.0;
            }
            if !contested {
                // Actual next funded request, not the entire lifetime bill
                // mislabelled as a recurring daily factory order.
                if let Some(f) = work_plans.get(&p.id) {
                    construction.intermediates += f.goods.intermediates;
                    construction.capital_goods += f.goods.capital_goods;
                }
            }
        }
        let queued = |k| pending.get(&k).copied().unwrap_or(0.0);
        let effective = |k| industrial_modules::effective_capacity(w, d, k);
        let auto = 1.0 + production::level(w, d, K::Automation) as f64 * 0.2;
        let future_auto = auto + queued(K::Automation) * 0.2;
        let processing = industry::plant_rate(w, d, K::ProcessingPlant)
            + industry::plant_rate(w, d, K::StarterIndustry);
        let machinery = industry::plant_rate(w, d, K::MachineryWorks);
        let process_total =
            (effective(K::ProcessingPlant) + queued(K::ProcessingPlant)) * future_auto;
        let machine_total =
            (effective(K::MachineryWorks) + queued(K::MachineryWorks)) * 0.5 * future_auto;
        let efficiency = (1.0
            - (production::level(w, d, K::Efficiency) as f64 + queued(K::Efficiency)) * 0.1)
            .max(0.5);
        let materials_power = crate::materials::province_reserved_daily(w, nation, d)
            * industry::power_per_pack(w, d, K::ProcessingPlant);
        let power = (process_total + 2.0 * machine_total) * efficiency + materials_power;
        out.generation_daily += effective(K::Generation) * 10.0;
        out.generation_committed_daily += queued(K::Generation) * 10.0;
        out.power_required_daily += power;
        out.storage_committed += queued(K::Warehouse) * 250.0;
        if !contested {
            labs += production::level(w, d, K::ResearchCenter) as f64;
            labs_committed += queued(K::ResearchCenter);
        }
        out.provinces.push(ProvinceCapacity {
            district: d.clone(),
            estate: effective(K::CivilianIndustry),
            estate_committed: queued(K::CivilianIndustry),
            processing_daily: processing,
            processing_committed_daily: (process_total - processing).max(0.0),
            machinery_daily: machinery,
            machinery_committed_daily: (machine_total - machinery).max(0.0),
            grid_daily: effective(K::PowerGrid) * 5.0,
            grid_committed_daily: queued(K::PowerGrid) * 5.0,
            power_required_daily: power,
            materials_power_required_daily: materials_power,
            contested,
        });
    }
    let research = industry::research_goods_demand(w, nation);
    // Pending labs may consume only still-useful work not already covered by
    // installed labs; they are not extra research effort or guaranteed buyers.
    let pending_lab_work =
        (industry::research_work_demand(w, nation) / industry::PROTOTYPE_WORK_PER_LEVEL_DAY - labs)
            .max(0.0)
            .min(labs_committed);
    let capital_domestic = construction.capital_goods
        + research.capital_goods
        + pending_lab_work * industry::PROTOTYPE_CAPITAL_PER_LEVEL_DAY;
    let machine_installed: f64 = out.provinces.iter().map(|p| p.machinery_daily).sum();
    let machine_committed: f64 = out
        .provinces
        .iter()
        .map(|p| p.machinery_committed_daily)
        .sum();
    let capital = balance(
        w,
        nation,
        Good::CapitalGoods,
        machine_installed,
        machine_committed,
        capital_domestic,
    );
    let machine_consumers: f64 = out
        .provinces
        .iter()
        .filter(|p| !p.contested)
        .map(|p| p.machinery_daily + p.machinery_committed_daily)
        .sum();
    // Do not expand upstream solely to feed machinery whose finished product
    // already fills its buffer. Existing machinery may still run normally;
    // this controls new construction, never invents/removes consumption.
    let machine_use = if capital.stock + capital.incoming
        >= (capital.demand_daily * STOCK_COVER_DAYS).max(machine_consumers * 30.0)
    {
        machine_consumers.min(capital.demand_daily)
    } else {
        machine_consumers
    };
    let process_installed = out.provinces.iter().map(|p| p.processing_daily).sum();
    let process_committed = out
        .provinces
        .iter()
        .map(|p| p.processing_committed_daily)
        .sum();
    let domestic = construction.intermediates
        + research.intermediates
        + machine_use
        + pending_lab_work * industry::PROTOTYPE_INTERMEDIATES_PER_LEVEL_DAY;
    out.goods.push(balance(
        w,
        nation,
        Good::Intermediates,
        process_installed,
        process_committed,
        domestic,
    ));
    out.goods.push(capital);
    out.inherited_sectors = structural_sectors(w, nation, &out);
    out
}
