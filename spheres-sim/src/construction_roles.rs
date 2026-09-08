//! Player-facing purpose, attached to the actual project kind and served by
//! every construction view. Coefficients and effects remain in the operators.
use crate::production::ProjectKind as K;
use serde::Serialize;

#[derive(Clone, Copy, Serialize)]
pub struct Role {
    pub category: &'static str,
    pub purpose: &'static str,
    pub tradeoff: &'static str,
}

pub fn role(kind: K) -> Role {
    let (category,purpose,tradeoff) = match kind {
        K::CivilianIndustry => ("build_faster", "Expand the pool assigned to construction projects.", "Workers, operating funds and reliable electricity are needed to deliver the extra capacity."),
        K::OfficeDistrict => ("earn_income", "Create service jobs and expand the taxable economy.", "Needs skilled workers, service demand, electricity and operating funds. GDP is separate from government revenue."),
        K::ArmsPlant => ("equip_forces", "Open production slots for military equipment.", "Assign a production line and fund its equipment order; workers, power and inputs limit delivery."),
        K::Shipyard => ("equip_forces", "Build naval equipment and support fleet repairs.", "Requires a mapped coastal gateway, workers, power and funded naval production."),
        K::Generation => ("support_economy", "Supply electricity to restore or expand industrial output.", "Fuel and generation funding are required when electricity is dispatched."),
        K::ProcessingPlant => ("support_economy", "Turn raw resources into prepared materials for construction, machinery and trade.", "Consumes ore, fuel and electricity; full storage stops production."),
        K::MachineryWorks => ("support_economy", "Supply machine and tool packs for installation, research and trade.", "Needs prepared materials, copper, electricity and an operating budget."),
        K::AdvancedIndustry => ("support_economy", "Supply advanced components for industrial and equipment production.", "Requires skilled workers, technology metals, prepared materials and reliable power."),
        K::Infrastructure => ("support_economy", "Speed local construction and expand freight throughput.", "Helps projects and shipments in this province; low traffic limits the benefit."),
        K::ResearchCenter => ("support_economy", "Perform prototype work for the assigned research program.", "Requires a funded research program, scientists and prototype inputs."),
        K::FreightTerminal => ("support_economy", "Move more shipments through a coastal gateway.", "Only useful at a mapped gateway whose route can carry the extra freight."),
        K::PowerGrid => ("facility_upgrades", "Deliver more electricity to facilities in this province.", "Distribution does not generate electricity; national generation must also cover demand."),
        K::Warehouse => ("facility_upgrades", "Store more prepared materials and machine packs.", "Useful when storage is limiting a producer or when building a reserve."),
        K::Automation => ("facility_upgrades", "Increase throughput at an existing factory.", "Requires robotics technology and more inputs and operating funds for the additional output."),
        K::Efficiency => ("facility_upgrades", "Reduce the electricity and fuel used by an existing factory.", "Requires process technology; savings depend on the factory actually operating."),
        K::StarterIndustry => ("build_faster", "Build a sized civilian factory, power plant, grid and materials line together.", "All named components are paid for; the package still needs operating inputs after commissioning."),
    };
    Role { category, purpose, tradeoff }
}
