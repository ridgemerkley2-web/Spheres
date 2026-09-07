//! Custom ground and tactical-strike design lifecycles. Ratings and prices are
//! explicit GAME assumptions, not specifications of historical vehicles.
//! Component knowledge consumes existing Aerospace effort. Equipment itself
//! always remains in `Nation::arsenal`, never in this project ledger.
use crate::{
    clock,
    production::Priority,
    world::{Nation, NationId, WorldState},
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const VERSION: u32 = 8;
pub const MAX_REVISIONS: usize = 128;
pub const MAX_PROJECTS: usize = 128;
pub const MAX_BATCH: u32 = 1000;
pub const DELIVERY_DAYS: u32 = 7;
pub const SLOTS: [&str; 5] = [
    "mobility",
    "protection",
    "armament",
    "sensors",
    "communications",
];

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct DesignSpec {
    pub platform: String,
    pub components: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct PlatformDef {
    pub id: &'static str,
    pub name: &'static str,
    pub capacity: u32,
    pub cost_bn: f64,
    pub mobility: f64,
    pub protection: f64,
    pub detail: &'static str,
}
pub const PLATFORMS: &[PlatformDef] = &[
    PlatformDef { id:"tank_standard", name:"Main battle tank", capacity:12,
        cost_bn:0.0011, mobility:1.0, protection:1.0,
        detail:"Balanced protection, mobility and installation room. A flexible starting point for general-purpose forces." },
    PlatformDef { id:"tank_heavy", name:"Heavy tank", capacity:16,
        cost_bn:0.0017, mobility:0.85, protection:1.15,
        detail:"Modeled larger chassis: more installation room and protection, with greater expense and lower mobility." },
    PlatformDef { id:"tank_light", name:"Light tank", capacity:10,
        cost_bn:0.00065, mobility:1.25, protection:0.70,
        detail:"Light, economical reconnaissance-oriented tank. Limited armor, engine and gun installation choices." },
    PlatformDef { id:"tank_destroyer", name:"Tank destroyer", capacity:14,
        cost_bn:0.00095, mobility:0.95, protection:1.05,
        detail:"Tracked vehicle with a fixed casemate and a powerful gun. Trades turret flexibility for a lower-cost weapon mounting." },
    PlatformDef { id:"ground_ifv", name:"Infantry fighting vehicle", capacity:14,
        cost_bn:0.00060, mobility:1.12, protection:0.78,
        detail:"Tracked infantry carrier with an autocannon and separate troop compartment. Provides ground fire support and protected maneuver." },
    PlatformDef { id:"ground_apc", name:"Armored personnel carrier", capacity:12,
        cost_bn:0.00032, mobility:1.18, protection:0.68,
        detail:"Wheeled protected transport with a defensive weapon station. Improves ground maneuver and troop protection; does not create overseas lift." },
    PlatformDef { id:"ground_recon", name:"Reconnaissance vehicle", capacity:12,
        cost_bn:0.00040, mobility:1.30, protection:0.60,
        detail:"Wheeled scout with a dedicated observation package and optional autocannon. Helps committed ground forces find exposed targets." },
    PlatformDef { id:"ground_artillery", name:"Self-propelled artillery", capacity:18,
        cost_bn:0.00070, mobility:0.95, protection:0.70,
        detail:"Tracked howitzer with separate loading and ammunition specifications. Adds indirect fire support to committed ground operations." },
    PlatformDef { id:"ground_air_defense", name:"Mobile air defense", capacity:17,
        cost_bn:0.00065, mobility:1.02, protection:0.68,
        detail:"Tracked gun or missile defense with separate search radar. Reduces air-strike damage to the forces it accompanies." },
    PlatformDef { id:"air_light_attack", name:"Light attack aircraft", capacity:18,
        cost_bn:0.0040, mobility:1.0, protection:1.0,
        detail:"Economical tactical-strike aircraft with two-store installations. Needs actual maintenance, available theatre basing and manufactured bombs for strike operations." },
    PlatformDef { id:"air_tactical_strike", name:"Tactical strike aircraft", capacity:21,
        cost_bn:0.0100, mobility:1.0, protection:1.0,
        detail:"Larger strike airframe with optional twin engines and four-store installations. Provides tactical air strikes; no interception, strategic lift or ground combat capability." },
];

#[derive(Clone, Debug, Serialize)]
pub struct ComponentDef {
    pub id: &'static str,
    pub name: &'static str,
    pub slot: &'static str,
    pub load: u32,
    pub cost_bn: f64,
    pub upkeep_bn_day: f64,
    pub land: f64,
    pub protection: f64,
    pub mobility: f64,
    pub recon: f64,
    pub research: Option<&'static str>,
    pub technology: Option<&'static str>,
    pub detail: &'static str,
}
macro_rules! component {
    ($id:literal,$name:literal,$slot:literal,$load:expr,$cost:expr,$upkeep:expr,$land:expr,$protection:expr,$mobility:expr,$recon:expr,$research:expr,$tech:expr,$detail:literal) => {
        ComponentDef {
            id: $id,
            name: $name,
            slot: $slot,
            load: $load,
            cost_bn: $cost,
            upkeep_bn_day: $upkeep,
            land: $land,
            protection: $protection,
            mobility: $mobility,
            recon: $recon,
            research: $research,
            technology: $tech,
            detail: $detail,
        }
    };
}
pub const COMPONENTS: &[ComponentDef] = &[
    component!("drive_standard","Standard powertrain","mobility",2,0.00035,0.00000008,0.0,0.0,0.0,0.0,None,None,"Legacy package. Lower cost and support demand."),
    component!("drive_mobile","High-mobility powertrain","mobility",3,0.00065,0.00000015,0.0,0.0,0.20,0.0,None,None,"More mobility, at higher purchase and maintenance cost."),
    component!("protection_standard","Standard armor","protection",2,0.00040,0.00000003,0.0,0.0,0.0,0.0,None,None,"Baseline armor. Active protection is selected separately in detailed designs."),
    component!("protection_heavy","Reinforced armor","protection",4,0.00085,0.00000006,0.0,0.25,-0.12,0.0,None,None,"Greater protection occupies room and reduces mobility. Can accompany a separate active-protection system."),
    component!("protection_active","Integrated active protection","protection",4,0.00115,0.00000013,0.0,0.35,-0.05,0.0,None,Some("aero_active_protection_system"),"Requires the existing active-protection discovery; a new design or paid refit must install it."),
    component!("armament_standard","General-purpose tank weapon package","armament",3,0.00055,0.00000006,0.0,0.0,0.0,0.0,None,None,"Legacy package mapped to the modeled 105 mm mixed ammunition family. Compatible physical rounds are manufactured separately."),
    component!("armament_heavy","Heavy tank weapon package","armament",4,0.00090,0.00000010,0.20,0.0,-0.06,0.0,None,None,"Higher land contribution, installation and support burden. Mapped to modeled 120 mm mixed rounds, manufactured separately."),
    component!("sensors_optical","Optical observation and control","sensors",1,0.00015,0.00000002,0.0,0.0,0.0,0.0,None,None,"Legacy observation package."),
    component!("sensors_integrated","Integrated observation and fire control","sensors",2,0.00048,0.00000006,0.12,0.0,0.0,0.25,Some("tank_fire_control_1990"),None,"A modeled vehicle integration programme, enabled by electronics knowledge. Research alone changes no existing vehicle."),
    component!("comms_radio","Field radio package","communications",1,0.00008,0.00000001,0.0,0.0,0.0,0.0,None,None,"Legacy radio integration."),
    component!("comms_data","Tactical data integration","communications",2,0.00022,0.00000004,0.04,0.0,0.0,0.10,None,Some("aero_tactical_datalink"),"Requires existing tactical data-link knowledge and installation on a compatible revision."),
];

include!("equipment_specs.rs");
include!("equipment_ground.rs");
include!("equipment_aviation.rs");
include!("equipment_supply.rs");
include!("equipment_service.rs");
include!("equipment_maintenance.rs");
include!("equipment_targets.rs");
include!("equipment_replenishment.rs");
include!("equipment_ammunition_production.rs");
include!("equipment_ammunition_operations.rs");
include!("equipment_ammunition_reserves.rs");
include!("equipment_supply_automation.rs");

#[derive(Clone, Debug, Serialize)]
pub struct ResearchDef {
    pub id: &'static str,
    pub name: &'static str,
    pub points: f64,
    pub earliest_year: i32,
    pub prerequisite: &'static str,
    pub detail: &'static str,
}
pub const RESEARCH: &[ResearchDef] = &[ResearchDef {
    id:"tank_fire_control_1990", name:"Vehicle observation and fire-control integration",
    points:32.0, earliest_year:1990, prerequisite:"core_cmos_submicron",
    detail:"Modeled integration programme using submicron electronics. Consumes Aerospace research effort; grants only compatible tank components, with no macroeconomic discovery or national military bonus.",
}, ResearchDef { id:"tank_powerpack", name:"Advanced engine and powerpack integration", points:36.0, earliest_year:1990, prerequisite:"core_cmos_submicron", detail:"Electronic engine management and vehicle integration unlock the turbine engine specification. Uses Aerospace effort." },
ResearchDef { id:"tank_running_gear", name:"Controlled suspension integration", points:24.0, earliest_year:1990, prerequisite:"core_cmos_submicron", detail:"Vehicle control integration unlocks hydropneumatic suspension. Uses Aerospace effort." },
ResearchDef { id:"tank_autoloader", name:"Autoloading turret integration", points:40.0, earliest_year:1990, prerequisite:"core_cmos_submicron", detail:"Vehicle automation integration unlocks a separate autoloading turret. Uses Aerospace effort." },
ResearchDef { id:"ground_wheeled_chassis", name:"Advanced wheeled chassis", points:26.0, earliest_year:1990, prerequisite:"core_cmos_submicron", detail:"Builds on controlled suspension to unlock eight-wheel run-flat drive for carriers and scouts." },
ResearchDef { id:"ground_engine_management", name:"Managed ground powertrains", points:28.0, earliest_year:1990, prerequisite:"core_cmos_submicron", detail:"Builds on powerpack integration to unlock a managed compact diesel and electronic transmission." },
ResearchDef { id:"ground_medium_weapons", name:"Medium weapons integration", points:28.0, earliest_year:1990, prerequisite:"core_cmos_submicron", detail:"Unlocks a 35 mm autocannon and a 155 mm howitzer. Leads to automation and guided-weapon branches." },
ResearchDef { id:"ground_artillery_automation", name:"Artillery loading automation", points:34.0, earliest_year:1990, prerequisite:"core_cmos_submicron", detail:"Combines turret automation and medium weapons integration to unlock assisted artillery loading." },
ResearchDef { id:"ground_guided_weapons", name:"Guided ground weapons", points:42.0, earliest_year:1990, prerequisite:"core_cmos_submicron", detail:"Combines medium weapons and fire-control knowledge to unlock guided artillery loads and mobile air-defense missiles." },
ResearchDef { id:"ground_modular_armor", name:"Modular ground protection", points:26.0, earliest_year:1990, prerequisite:"core_cmos_submicron", detail:"Unlocks modular specialist-vehicle armor and a protected infantry compartment. Both require installation." },
ResearchDef { id:"ground_sensor_fusion", name:"Ground sensor fusion", points:34.0, earliest_year:1990, prerequisite:"core_cmos_submicron", detail:"Builds on observation integration to unlock elevated scout sensors and tracking radar." },
ResearchDef { id:"ground_secure_radios", name:"Secure ground communications", points:20.0, earliest_year:1990, prerequisite:"core_cmos_submicron", detail:"Unlocks secure radio installations and begins the ground command-network branch." },
ResearchDef { id:"ground_battlefield_network", name:"Networked ground command", points:38.0, earliest_year:1990, prerequisite:"core_cmos_submicron", detail:"Combines secure radios and sensor fusion to unlock networked reconnaissance and specialist coordination." },
ResearchDef { id:"air_propulsion_integration", name:"Aircraft propulsion and flight-control integration", points:40.0, earliest_year:1990, prerequisite:"core_cmos_submicron", detail:"Unlocks managed aircraft engines and stabilized attack wings. Installed components change sustained sortie output; research grants no aircraft or national modifier." },
ResearchDef { id:"air_mission_systems", name:"Tactical aircraft mission systems", points:42.0, earliest_year:1990, prerequisite:"core_cmos_submicron", detail:"Unlocks ground-mapping radar, digital attack avionics and integrated countermeasures. Effects require a certified aircraft revision and supplied strike mission." },
ResearchDef { id:"air_guided_strike", name:"Guided air-to-ground stores integration", points:48.0, earliest_year:1990, prerequisite:"core_cmos_submicron", detail:"Builds on aircraft mission systems to certify guided-bomb interfaces. Bombs remain finite manufactured consumables; no starting stores are granted." },
];

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CompiledProfile {
    pub rules_version: u32,
    /// Fabrication payment excludes raw materials acquired through the market.
    pub unit_cost_bn: f64,
    pub fabrication_cost_bn: f64,
    pub development_cost_bn: f64,
    pub development_days: u32,
    pub tooling_cost_bn: f64,
    pub tooling_days: u32,
    pub production_days: u32,
    pub service_months: u32,
    pub maintenance_bn_day: f64,
    pub land: f64,
    pub protection: f64,
    pub mobility: f64,
    pub recon: f64,
    pub land_factor: f64,
    /// Physical normalization, independent of selected component prices.
    pub reference_weight_bn: f64,
    pub recipe: [f64; 12],
    pub component_costs: BTreeMap<String, f64>,
    pub installation_used: u32,
    pub installation_capacity: u32,
    /// Absent on old tank profiles so loading never changes frozen revisions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ground_roles: Option<GroundRoles>,
    /// Sparse on all earlier frozen profiles. Tactical strike has no land role.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aviation: Option<AviationProfile>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DesignRevision {
    pub id: String,
    pub name: String,
    pub spec: DesignSpec,
    pub profile: CompiledProfile,
    pub created_day: i32,
    pub certified_day: Option<i32>,
    pub specification_key: String,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProjectKind {
    Development,
    Production,
    Refit,
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProjectStatus {
    Working,
    Paused,
    Blocked,
    Complete,
    Cancelled,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct EquipmentProject {
    pub id: u32,
    pub kind: ProjectKind,
    pub revision_id: String,
    pub source_design_id: Option<String>,
    pub district: Option<String>,
    pub quantity: u32,
    pub completed_units: u32,
    pub priority: Priority,
    pub status: ProjectStatus,
    pub paused: bool,
    pub reason: String,
    pub started_day: i32,
    pub completed_day: Option<i32>,
    pub minimum_days: u32,
    pub work_days: f64,
    pub cost_bn: f64,
    pub spent_bn: f64,
    pub tooling_days: u32,
    pub tooling_cost_bn: f64,
    pub daily_budget_bn: f64,
    pub recipe_per_unit: [f64; 12],
    pub resources_used: [f64; 12],
    pub last_spent_bn: f64,
    pub last_day: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DesignDraft {
    pub name: String,
    pub spec: DesignSpec,
    pub updated_day: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct EquipmentState {
    pub version: u32,
    pub next_id: u32,
    pub revisions: BTreeMap<String, DesignRevision>,
    pub drafts: BTreeMap<String, DesignDraft>,
    pub projects: Vec<EquipmentProject>,
    pub learned: BTreeSet<String>,
    pub active_research: Option<String>,
    pub research_progress: f64,
    pub last_research_day: Option<i32>,
    pub last_research_completed_day: Option<i32>,
    pub finance_from_day: i32,
    pub force_support_scale: f64,
    pub development_daily_limit_bn: Option<f64>,
    pub procurement_daily_limit_bn: Option<f64>,
    pub maintenance_allocated_today_bn: f64,
    pub maintenance_required_today_bn: f64,
    pub maintenance_fraction: f64,
    pub last_tick_day: Option<i32>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub fleet_targets: BTreeMap<String, u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maintenance_plan: Option<FleetMaintenance>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ammunition: Option<AmmunitionState>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub ammunition_reserves: BTreeMap<String, AmmoReservePlan>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supply_automation: Option<EquipmentSupplyAutomation>,
}
impl Default for EquipmentState {
    fn default() -> Self {
        Self {
            version: VERSION,
            next_id: 1,
            revisions: BTreeMap::new(),
            drafts: BTreeMap::new(),
            projects: vec![],
            learned: BTreeSet::new(),
            active_research: None,
            research_progress: 0.0,
            last_research_day: None,
            last_research_completed_day: None,
            finance_from_day: i32::MAX,
            force_support_scale: 1.0,
            development_daily_limit_bn: None,
            procurement_daily_limit_bn: None,
            maintenance_allocated_today_bn: 0.0,
            maintenance_required_today_bn: 0.0,
            maintenance_fraction: 1.0,
            last_tick_day: None,
            fleet_targets: BTreeMap::new(),
            maintenance_plan: None,
            ammunition: None,
            ammunition_reserves: BTreeMap::new(),
            supply_automation: None,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct DesignPreview {
    pub valid: bool,
    pub blockers: Vec<String>,
    pub profile: Option<CompiledProfile>,
    pub specification_key: String,
    pub notes: Vec<String>,
}

pub fn baseline_spec() -> DesignSpec {
    DesignSpec {
        platform: "tank_standard".into(),
        components: [
            ("mobility", "drive_standard"),
            ("protection", "protection_standard"),
            ("armament", "armament_standard"),
            ("sensors", "sensors_optical"),
            ("communications", "comms_radio"),
        ]
        .into_iter()
        .map(|(k, v)| (k.into(), v.into()))
        .collect(),
    }
}
pub fn component(id: &str) -> Option<&'static ComponentDef> {
    all_components().find(|c| c.id == id)
}
pub fn research(id: &str) -> Option<&'static ResearchDef> {
    RESEARCH.iter().find(|r| r.id == id)
}
pub fn profile<'a>(n: &'a Nation, id: &str) -> Option<&'a CompiledProfile> {
    n.equipment.as_ref()?.revisions.get(id).map(|r| &r.profile)
}
pub fn specification_key(spec: &DesignSpec) -> String {
    // Canonical, length-delimited identity. This intentionally is not a hash of
    // floating ratings or a platform-dependent DefaultHasher result.
    let mut key = format!("v{}|{}:{}", spec_version(spec), spec.platform.len(), spec.platform);
    for (slot, id) in &spec.components {
        key.push_str(&format!("|{}:{}:{}:{}", slot.len(), slot, id.len(), id));
    }
    key
}
pub fn component_known(n: &Nation, c: &ComponentDef) -> bool {
    c.technology.is_none_or(|id| n.tech.knows(id))
        && c.research
            .is_none_or(|id| n.equipment.as_ref().is_some_and(|s| s.learned.contains(id)))
}

pub fn design_preview(w: &WorldState, nation: NationId, spec: &DesignSpec) -> DesignPreview {
    let mut blockers = vec![];
    let Some(n) = w.nation_opt(nation).filter(|n| n.alive) else {
        return DesignPreview {
            valid: false,
            blockers: vec!["This government is not active.".into()],
            profile: None,
            specification_key: specification_key(spec),
            notes: vec![],
        };
    };
    let platform = PLATFORMS.iter().find(|p| p.id == spec.platform);
    let required = slots_for(spec);
    blockers.extend(configuration_refusals(spec));
    if platform.is_none() {
        blockers.push("Choose a supported vehicle chassis or aircraft airframe.".into());
    }
    for key in spec.components.keys() {
        if !required.contains(&key.as_str()) {
            blockers.push(format!("Unsupported component slot: {key}."));
        }
    }
    let mut selected = vec![];
    for &slot in required {
        match spec.components.get(slot).and_then(|id| component(id)) {
            None => blockers.push(format!("Choose a valid {slot} component.")),
            Some(c) if c.slot != slot => {
                blockers.push(format!("{} does not fit the {slot} slot.", c.name))
            }
            Some(c) => {
                if !component_known(n, c) {
                    blockers.push(format!(
                        "Research {} before developing this configuration.",
                        c.research
                            .and_then(research)
                            .map(|r| r.name)
                            .unwrap_or(c.technology.unwrap_or(c.name))
                    ));
                }
                selected.push(c);
            }
        }
    }
    let compiled = platform.filter(|_| selected.len() == required.len()).map(|p| {
        if is_aviation_platform(p.id) {
            let profile = compile_aviation_model(spec).expect("all required aircraft slots validated");
            if profile.installation_used > profile.installation_capacity {
                blockers.push(format!("Installation load {} exceeds this airframe's {} capacity.",
                    profile.installation_used, profile.installation_capacity));
            }
            return profile;
        }
        let capacity = p.capacity + if detailed_spec(spec) { 8 } else { 0 };
        let used = selected.iter().map(|c| c.load).sum();
        if used > capacity {
            blockers.push(format!(
                "Installation load {used} exceeds this chassis's {} capacity.",
                capacity
            ));
        }
        let cost = p.cost_bn + selected.iter().map(|c| c.cost_bn).sum::<f64>();
        let mut recipe = [0.0; 12];
        // MODEL input equivalents in the existing raw table's units: iron t,
        // coal kt and copper t. Fabrication cost deliberately excludes these.
        recipe[crate::resources::Commodity::Iron.idx()] =
            match p.id { "tank_heavy" => 75.0, "tank_light" => 30.0, "tank_destroyer" => 50.0,
                "ground_ifv" => 24.0, "ground_apc" => 15.0, "ground_recon" => 12.0,
                "ground_artillery" => 32.0, "ground_air_defense" => 26.0, _ => 55.0 };
        recipe[crate::resources::Commodity::Coal.idx()] = 0.025;
        recipe[crate::resources::Commodity::Copper.idx()] = 0.30
            + selected
                .iter()
                .filter(|c| c.research.is_some() || c.technology.is_some())
                .count() as f64
                * 0.15;
        let land = 1.0 + selected.iter().map(|c| c.land).sum::<f64>();
        let protection = p.protection + selected.iter().map(|c| c.protection).sum::<f64>();
        let mobility = p.mobility + selected.iter().map(|c| c.mobility).sum::<f64>();
        let recon = 1.0 + selected.iter().map(|c| c.recon).sum::<f64>();
        CompiledProfile {
            rules_version: spec_version(spec),
            unit_cost_bn: cost,
            fabrication_cost_bn: cost,
            development_cost_bn: cost * 30.0,
            development_days: 180 + used * 5,
            tooling_cost_bn: cost * 5.0,
            tooling_days: 30,
            production_days: 20 + used,
            service_months: 360,
            maintenance_bn_day: 0.00000015 + selected.iter().map(|c| c.upkeep_bn_day).sum::<f64>(),
            land,
            protection,
            mobility,
            recon,
            land_factor: ((land + protection + mobility + recon) / 4.0).clamp(0.75, 1.25),
            reference_weight_bn: match p.id { "tank_light" => 0.0021, "tank_destroyer" => 0.0028,
                "ground_ifv" => 0.0018, "ground_apc" => 0.0011, "ground_recon" => 0.0012,
                "ground_artillery" => 0.0022, "ground_air_defense" => 0.0020, _ => 0.0032 },
            recipe,
            component_costs: selected
                .iter()
                .map(|c| (c.slot.into(), c.cost_bn))
                .collect(),
            installation_used: used,
            installation_capacity: capacity,
            ground_roles: compile_ground_roles(spec, land, protection, mobility, recon),
            aviation: None,
        }
    });
    DesignPreview {valid:blockers.is_empty(),blockers,profile:compiled,specification_key:specification_key(spec),notes:vec![
        "Prices, installation ratings and input quantities are explicit game-model assumptions, not historical vehicle specifications.".into(),
        "One unit is one complete new vehicle or aircraft. Inherited Arsenal formations remain legacy equivalents.".into(),
        "Fabrication payments exclude separately acquired raw inputs. Research and drafts create no equipment or force bonus.".into()]}
}

pub(crate) fn actor_refusal(w: &WorldState, id: NationId) -> Option<String> {
    if !clock::is_daily(w) {
        return Some("Equipment design requires daily play.".into());
    }
    if !w.rules.military_operations {
        return Some("Equipment design requires conserved military operations so fielded vehicles have supported roles and losses.".into());
    }
    if !w.nation_opt(id).is_some_and(|n| n.alive) {
        return Some("This government is not active.".into());
    }
    if !crate::economic_ai::may_direct(w, id) {
        return Some("You cannot direct this government's equipment programme.".into());
    }
    None
}
pub(crate) fn name_refusal(name: &str) -> Option<String> {
    (name.trim().is_empty() || name.chars().count() > 80 || name.chars().any(char::is_control))
        .then(|| "Use a model name of 1–80 characters without control characters.".into())
}
fn budget_refusal(bn: f64) -> Option<String> {
    (!bn.is_finite() || !(0.0..=1000.0).contains(&bn))
        .then(|| "Daily funding must be a finite amount between zero and $1,000bn.".into())
}
fn state_mut(n: &mut Nation) -> &mut EquipmentState {
    let state = n.equipment.get_or_insert_with(EquipmentState::default);
    state.version = VERSION;
    state
}
fn room_refusal(n: &Nation) -> Option<String> {
    n.equipment.as_ref().and_then(|s| {
        if s.revisions.len() >= MAX_REVISIONS
            || s.projects.len() >= MAX_PROJECTS
            || s.next_id == u32::MAX
        {
            Some("This campaign's equipment project library is full.".into())
        } else {
            None
        }
    })
}
pub(crate) fn activation_refusal(n: &Nation) -> Option<String> {
    let Some(p) = &n.program_budget else {
        return Some("Activate a departmental budget before funding equipment development.".into());
    };
    if p.departments[crate::world::BUDGET_DEFENSE][0] as u32
        + p.departments[crate::world::BUDGET_DEFENSE][1] as u32
        == 0
    {
        Some("Allocate Defense funding to Personnel & training or Operations before activating equipment development.".into())
    } else {
        None
    }
}
pub(crate) fn activate(n: &mut Nation, today: i32) {
    if n.equipment
        .as_ref()
        .is_some_and(|s| s.finance_from_day != i32::MAX)
    {
        return;
    }
    let p = n
        .program_budget
        .as_ref()
        .expect("activation was preflighted");
    let base = p.departments[crate::world::BUDGET_DEFENSE][0] as f64
        + p.departments[crate::world::BUDGET_DEFENSE][1] as f64;
    let scale = (base + p.departments[crate::world::BUDGET_DEFENSE][4] as f64) / base;
    let s = state_mut(n);
    s.finance_from_day = today.saturating_add(1);
    s.force_support_scale = scale;
}
pub fn save_draft(
    w: &mut WorldState,
    id: NationId,
    name: &str,
    spec: DesignSpec,
) -> Result<(), String> {
    if let Some(e) = actor_refusal(w, id).or_else(|| name_refusal(name)) {
        return Err(e);
    }
    if spec.components.len() > 16
        || spec.platform.len() > 80
        || spec
            .components
            .iter()
            .any(|(k, v)| k.len() > 80 || v.len() > 120)
    {
        return Err("This draft contains oversized component identifiers.".into());
    }
    if w.nation(id)
        .equipment
        .as_ref()
        .is_some_and(|s| s.drafts.len() >= MAX_REVISIONS && !s.drafts.contains_key(name.trim()))
    {
        return Err("The design draft library is full.".into());
    }
    let day = clock::absolute_day(w);
    state_mut(w.nation_mut(id)).drafts.insert(
        name.trim().into(),
        DesignDraft {
            name: name.trim().into(),
            spec,
            updated_day: day,
        },
    );
    Ok(())
}
pub fn research_refusal(w: &WorldState, id: NationId, component_id: &str) -> Option<String> {
    if let Some(e) = actor_refusal(w, id) {
        return Some(e);
    }
    let n = w.nation(id);
    if let Err(reason) = research_available(n, component_id) { return Some(reason); }
    if n.equipment
        .as_ref()
        .is_some_and(|s| s.active_research.as_deref() == Some(component_id))
    {
        return Some("This equipment research project is already selected.".into());
    }
    None
}
pub fn start_research(w: &mut WorldState, id: NationId, component_id: &str) -> Result<(), String> {
    if let Some(e) = research_refusal(w, id, component_id) {
        return Err(e);
    }
    let s = state_mut(w.nation_mut(id));
    if s.active_research.is_some() {
        s.research_progress *= 0.5;
    }
    s.active_research = Some(component_id.into());
    s.last_research_completed_day = None;
    Ok(())
}
/// Called by the existing Aerospace grant, after its shared acquisition-quota
/// check. `true` means those points belong to this project and cannot also be
/// assigned to ordinary technology. Root tech integration owns domain switching
/// and the shared monthly acquisition counter.
pub fn research_step(n: &mut Nation, points: f64, year: i32, day: i32) -> bool {
    let Some(id) = n.equipment.as_ref().and_then(|s| s.active_research.clone()) else {
        return false;
    };
    let Some(def) = research(&id) else {
        return true;
    };
    if research_available(n, &id).is_err() {
        return true;
    }
    let s = state_mut(n);
    if s.last_research_day == Some(day) {
        return true;
    }
    s.last_research_day = Some(day);
    if points.is_finite() && points > 0.0 {
        s.research_progress += points;
    }
    if year >= def.earliest_year && s.research_progress + 1e-12 >= def.points {
        s.research_progress = (s.research_progress - def.points).max(0.0);
        s.learned.insert(id);
        s.active_research = None;
        s.last_research_completed_day = Some(day);
    }
    true
}

#[derive(Clone, Debug, Serialize)]
pub struct ProjectQuote {
    pub valid: bool,
    pub reason: Option<String>,
    pub cost_bn: f64,
    pub minimum_days: u32,
    pub daily_budget_bn: f64,
    pub eta_days: Option<u32>,
    pub recipe: [f64; 12],
    pub available_units: Option<u32>,
    pub note: String,
}
fn quoted(
    cost: f64,
    days: u32,
    budget: f64,
    reason: Option<String>,
    recipe: [f64; 12],
) -> ProjectQuote {
    let eta = (budget.is_finite() && budget > 0.0)
        .then(|| ((cost / budget).ceil().min(u32::MAX as f64) as u32).max(days));
    ProjectQuote {valid:reason.is_none(),reason,cost_bn:cost,minimum_days:days,daily_budget_bn:budget,
        eta_days:eta,recipe,available_units:None,note:"Timing assumes renewed departmental authority, continuous funding and available inputs. Raw materials are acquired separately; completed units take seven days to enter service.".into()}
}
pub fn development_quote(
    w: &WorldState,
    id: NationId,
    name: &str,
    spec: &DesignSpec,
    daily_budget_bn: f64,
) -> ProjectQuote {
    let v = design_preview(w, id, spec);
    let reason = actor_refusal(w, id)
        .or_else(|| name_refusal(name))
        .or_else(|| budget_refusal(daily_budget_bn))
        .or_else(|| v.blockers.first().cloned())
        .or_else(|| activation_refusal(w.nation(id)))
        .or_else(|| room_refusal(w.nation(id)))
        .or_else(|| {
            w.nation(id).equipment.as_ref().and_then(|s| {
                s.revisions
                    .values()
                    .find(|r| {
                        r.specification_key == v.specification_key
                            && (r.certified_day.is_some()
                                || s.projects.iter().any(|p| {
                                    p.revision_id == r.id
                                        && !matches!(
                                            p.status,
                                            ProjectStatus::Cancelled | ProjectStatus::Complete
                                        )
                                }))
                    })
                    .map(|r| {
                        format!(
                            "This configuration already exists as {}. Use its existing revision.",
                            r.name
                        )
                    })
            })
        });
    let (cost, days) = v
        .profile
        .map_or((0.0, 0), |p| (p.development_cost_bn, p.development_days));
    quoted(cost, days, daily_budget_bn, reason, [0.0; 12])
}
pub fn start_development(
    w: &mut WorldState,
    id: NationId,
    name: &str,
    spec: DesignSpec,
    daily_budget_bn: f64,
) -> Result<String, String> {
    let q = development_quote(w, id, name, &spec, daily_budget_bn);
    if let Some(e) = q.reason {
        return Err(e);
    }
    let v = design_preview(w, id, &spec);
    let today = clock::absolute_day(w);
    activate(w.nation_mut(id), today);
    let s = state_mut(w.nation_mut(id));
    let number = s.next_id;
    s.next_id += 1;
    let revision_id = format!("{}-design-{number}", id.code());
    s.revisions.insert(
        revision_id.clone(),
        DesignRevision {
            id: revision_id.clone(),
            name: name.trim().into(),
            spec,
            profile: v.profile.unwrap(),
            created_day: today,
            certified_day: None,
            specification_key: v.specification_key,
        },
    );
    s.projects.push(new_project(
        number,
        ProjectKind::Development,
        &revision_id,
        today,
        q.minimum_days,
        q.cost_bn,
        daily_budget_bn,
    ));
    Ok(revision_id)
}
fn new_project(
    id: u32,
    kind: ProjectKind,
    revision: &str,
    day: i32,
    minimum_days: u32,
    cost_bn: f64,
    budget: f64,
) -> EquipmentProject {
    EquipmentProject {
        id,
        kind,
        revision_id: revision.into(),
        source_design_id: None,
        district: None,
        quantity: 1,
        completed_units: 0,
        priority: Priority::Normal,
        status: ProjectStatus::Working,
        paused: false,
        reason: "Awaiting the next funding day.".into(),
        started_day: day,
        completed_day: None,
        minimum_days,
        work_days: 0.0,
        cost_bn,
        spent_bn: 0.0,
        tooling_days: 0,
        tooling_cost_bn: 0.0,
        daily_budget_bn: budget,
        recipe_per_unit: [0.0; 12],
        resources_used: [0.0; 12],
        last_spent_bn: 0.0,
        last_day: None,
    }
}
pub fn reserved_site_slots(n: &Nation, district: &str) -> usize {
    n.equipment.as_ref().map_or(0, |s| {
        s.projects
            .iter()
            .filter(|p| {
                p.district.as_deref() == Some(district)
                    && !matches!(p.status, ProjectStatus::Complete | ProjectStatus::Cancelled)
            })
            .count() + reserved_ammunition_slots(n,district)
    })
}

/// A completed job still consumed its plant's work packet on its completion
/// date. Lower-priority corporate work cannot reuse it later that same day.
pub(crate) fn occupied_site_slots_today(n:&Nation,district:&str,day:i32)->usize {
    n.equipment.as_ref().map_or(0,|s|s.projects.iter().filter(|p|p.district.as_deref()==Some(district)
        &&(!matches!(p.status,ProjectStatus::Complete|ProjectStatus::Cancelled)||p.completed_day==Some(day)&&p.status==ProjectStatus::Complete)).count()
        +s.ammunition.as_ref().map_or(0,|a|a.orders.iter().filter(|p|p.district==district&&(!ammunition_ended(p)||p.completed_day==Some(day)&&p.status==ProjectStatus::Complete)).count()))
}
pub fn maintenance_allocated_bn(n: &Nation) -> f64 {
    n.equipment
        .as_ref()
        .map_or(0.0, |s| s.maintenance_allocated_today_bn)
}

fn site_refusal(w: &WorldState, id: NationId, district: &str) -> Option<String> {
    if let Some(e) = crate::control::blocker(w, id, district) {
        return Some(e);
    }
    if w.districts.get(district) != Some(&id) {
        return Some("Choose a province controlled by this government.".into());
    }
    let slots = crate::manufacturing::plant_slots(w, district) as usize;
    let used = crate::manufacturing::used_slots(w, id, district);
    if used >= slots {
        return Some("This province needs a free completed arms-plant slot.".into());
    }
    None
}
fn certified<'a>(n: &'a Nation, id: &str) -> Result<&'a DesignRevision, String> {
    let r = n
        .equipment
        .as_ref()
        .and_then(|s| s.revisions.get(id))
        .ok_or("This design revision is missing.")?;
    if r.certified_day.is_none() {
        return Err(
            "Complete development and certification before producing this revision.".into(),
        );
    }
    Ok(r)
}
pub fn production_quote(
    w: &WorldState,
    id: NationId,
    revision: &str,
    district: &str,
    quantity: u32,
    budget: f64,
) -> ProjectQuote {
    let mut reason = actor_refusal(w, id).or_else(|| budget_refusal(budget));
    if reason.is_none() && !(1..=MAX_BATCH).contains(&quantity) {
        reason = Some(format!(
            "Choose between one and {MAX_BATCH} complete vehicles."
        ));
    }
    if reason.is_none() && !w.rules.resource_market {
        reason = Some("Equipment manufacturing requires the resource market.".into());
    }
    if reason.is_none() && crate::companies::licensed_revision(w,id,revision) {
        reason=Some("This equipment revision is licensed to its manufacturer. Buy the company's finished stock instead of creating a second public production line.".into());
    }
    if reason.is_none() {
        reason = site_refusal(w, id, district).or_else(|| room_refusal(w.nation(id)));
    }
    let r = w.nation_opt(id).and_then(|n| certified(n, revision).ok());
    if reason.is_none() && r.is_none() {
        reason =
            Some("Complete development and certification before producing this revision.".into());
    }
    let (cost, days, recipe) = r.map_or((0.0, 0, [0.0; 12]), |r| {
        let p = &r.profile;
        (
            p.tooling_cost_bn + p.fabrication_cost_bn * quantity as f64,
            p.tooling_days + p.production_days.saturating_mul(quantity),
            p.recipe.map(|x| x * quantity as f64),
        )
    });
    quoted(cost, days, budget, reason, recipe)
}
pub fn start_production(
    w: &mut WorldState,
    id: NationId,
    revision: &str,
    district: &str,
    quantity: u32,
    budget: f64,
) -> Result<u32, String> {
    let q = production_quote(w, id, revision, district, quantity, budget);
    if let Some(e) = q.reason {
        return Err(e);
    }
    let p = profile(w.nation(id), revision).unwrap().clone();
    let day = clock::absolute_day(w);
    let s = state_mut(w.nation_mut(id));
    let number = s.next_id;
    s.next_id += 1;
    let mut job = new_project(
        number,
        ProjectKind::Production,
        revision,
        day,
        q.minimum_days,
        q.cost_bn,
        budget,
    );
    job.quantity = quantity;
    job.district = Some(district.into());
    job.tooling_days = p.tooling_days;
    job.tooling_cost_bn = p.tooling_cost_bn;
    job.recipe_per_unit = p.recipe;
    s.projects.push(job);
    Ok(number)
}
fn refit_terms(
    source: &DesignRevision,
    target: &DesignRevision,
) -> Result<(f64, u32, [f64; 12]), String> {
    if source.id == target.id {
        return Err("Choose a different certified revision.".into());
    }
    if source.spec.platform != target.spec.platform
        || source.spec.components.get("armament") != target.spec.components.get("armament")
    {
        return Err(
            "This change replaces the chassis, airframe or main ground weapon. Manufacture new equipment instead."
                .into(),
        );
    }
    let changed: Vec<_> = target
        .spec
        .components
        .iter()
        .filter(|(slot, id)| source.spec.components.get(*slot) != Some(*id))
        .collect();
    if changed.is_empty() {
        return Err("These revisions have no equipment changes to refit.".into());
    }
    let cost = changed
        .iter()
        .map(|(slot, _)| {
            target
                .profile
                .component_costs
                .get(*slot)
                .copied()
                .unwrap_or(0.0)
        })
        .sum::<f64>()
        * 1.15;
    let scale = changed.len() as f64 / slots_for(&target.spec).len() as f64 * 0.35;
    Ok((
        cost,
        30 + changed.len() as u32 * 10,
        target.profile.recipe.map(|x| x * scale),
    ))
}
pub fn refit_quote(
    w: &WorldState,
    id: NationId,
    source: &str,
    target: &str,
    district: &str,
    quantity: u32,
    budget: f64,
) -> ProjectQuote {
    let mut reason = actor_refusal(w, id).or_else(|| budget_refusal(budget));
    if reason.is_none() && !w.rules.resource_market {
        reason = Some("Equipment refits require the resource market.".into());
    }
    if reason.is_none() && !(1..=MAX_BATCH).contains(&quantity) {
        reason = Some(format!(
            "Choose between one and {MAX_BATCH} vehicles to refit."
        ));
    }
    if reason.is_none() {
        reason = site_refusal(w, id, district).or_else(|| room_refusal(w.nation(id)));
    }
    let n = w.nation_opt(id);
    let available = n
        .and_then(|n| {
            n.arsenal
                .held
                .iter()
                .find(|h| h.design_id.as_deref() == Some(source))
        })
        .map_or(0, crate::arsenal::available_design_units);
    if reason.is_none() && available < quantity {
        reason = Some("There are not enough unreserved source vehicles for this refit.".into());
    }
    let terms = n
        .ok_or_else(|| "This government is missing.".into())
        .and_then(|n| {
            certified(n, source).and_then(|s| certified(n, target).and_then(|t| refit_terms(s, t)))
        });
    let (cost, days, recipe) = match terms {
        Ok(v) => v,
        Err(e) => {
            if reason.is_none() {
                reason = Some(e);
            }
            (0.0, 0, [0.0; 12])
        }
    };
    let mut q = quoted(
        cost * quantity as f64,
        days.saturating_mul(quantity),
        budget,
        reason,
        recipe.map(|x| x * quantity as f64),
    );
    q.available_units = Some(available);
    q
}
pub fn start_refit(
    w: &mut WorldState,
    id: NationId,
    source: &str,
    target: &str,
    district: &str,
    quantity: u32,
    budget: f64,
) -> Result<u32, String> {
    let q = refit_quote(w, id, source, target, district, quantity, budget);
    if let Some(e) = q.reason {
        return Err(e);
    }
    let (_, _, recipe) = refit_terms(
        certified(w.nation(id), source)?,
        certified(w.nation(id), target)?,
    )?;
    crate::arsenal::reserve_refit(w.nation_mut(id), source, quantity)?;
    let day = clock::absolute_day(w);
    let s = state_mut(w.nation_mut(id));
    let number = s.next_id;
    s.next_id += 1;
    let mut job = new_project(
        number,
        ProjectKind::Refit,
        target,
        day,
        q.minimum_days,
        q.cost_bn,
        budget,
    );
    job.quantity = quantity;
    job.source_design_id = Some(source.into());
    job.district = Some(district.into());
    job.recipe_per_unit = recipe;
    s.projects.push(job);
    Ok(number)
}
fn project_index(w: &WorldState, id: NationId, project: u32) -> Result<usize, String> {
    if let Some(e) = actor_refusal(w, id) {
        return Err(e);
    }
    let s = w
        .nation(id)
        .equipment
        .as_ref()
        .ok_or("No equipment projects exist.")?;
    let index = s
        .projects
        .iter()
        .position(|p| p.id == project)
        .ok_or("This equipment project is missing.")?;
    if matches!(
        s.projects[index].status,
        ProjectStatus::Complete | ProjectStatus::Cancelled
    ) {
        return Err("This equipment project has already ended.".into());
    }
    Ok(index)
}
pub fn set_project_paused(
    w: &mut WorldState,
    id: NationId,
    project: u32,
    paused: bool,
) -> Result<(), String> {
    let i = project_index(w, id, project)?;
    let p = &mut state_mut(w.nation_mut(id)).projects[i];
    p.paused = paused;
    p.status = if paused {
        ProjectStatus::Paused
    } else {
        ProjectStatus::Working
    };
    p.reason = if paused {
        "Paused by the government."
    } else {
        "Awaiting the next funding day."
    }
    .into();
    Ok(())
}
pub fn set_project_budget(
    w: &mut WorldState,
    id: NationId,
    project: u32,
    budget: f64,
) -> Result<(), String> {
    if let Some(e) = budget_refusal(budget) {
        return Err(e);
    }
    let i = project_index(w, id, project)?;
    state_mut(w.nation_mut(id)).projects[i].daily_budget_bn = budget;
    Ok(())
}
pub fn set_project_priority(
    w: &mut WorldState,
    id: NationId,
    project: u32,
    priority: Priority,
) -> Result<(), String> {
    let i = project_index(w, id, project)?;
    state_mut(w.nation_mut(id)).projects[i].priority = priority;
    Ok(())
}
pub fn cancel_project(w: &mut WorldState, id: NationId, project: u32) -> Result<(), String> {
    let i = project_index(w, id, project)?;
    let p = w.nation(id).equipment.as_ref().unwrap().projects[i].clone();
    if p.kind == ProjectKind::Refit {
        let remaining = p.quantity - p.completed_units;
        if remaining > 0 {
            crate::arsenal::release_refit(
                w.nation_mut(id),
                p.source_design_id
                    .as_deref()
                    .ok_or("The source revision is missing.")?,
                remaining,
            )?;
        }
    }
    let today = clock::absolute_day(w);
    let p = &mut state_mut(w.nation_mut(id)).projects[i];
    p.status = ProjectStatus::Cancelled;
    p.completed_day = Some(today);
    p.reason="Cancelled. Paid work remains spent; completed vehicles and deliveries remain in the Arsenal.".into();
    Ok(())
}

fn live_site_blocker(w: &WorldState, id: NationId, job: &EquipmentProject) -> Option<String> {
    let district = job.district.as_deref()?;
    if let Some(e) = crate::control::blocker(w, id, district) {
        return Some(e);
    }
    if w.districts.get(district) != Some(&id) {
        return Some("The production province is no longer owned by this government.".into());
    }
    let mut jobs: Vec<_> = w
        .nation(id)
        .equipment
        .as_ref()?
        .projects
        .iter()
        .filter(|p| {
            p.district.as_deref() == Some(district)
                && !matches!(p.status, ProjectStatus::Complete | ProjectStatus::Cancelled)
        })
        .collect();
    jobs.sort_by_key(|p| (p.priority.dispatch_rank(), p.id));
    // Existing directed lines retain their occupied plant places. New project
    // starts preflight all reservations; malformed/reduced-capacity saves use
    // stable priority order rather than running more work than the site holds.
    let legacy = crate::manufacturing::lines_for(w, id)
        .filter(|l| l.district == district)
        .count();
    let remaining =
        (crate::manufacturing::plant_slots(w, district) as usize).saturating_sub(legacy);
    if jobs
        .iter()
        .position(|p| p.id == job.id)
        .is_none_or(|index| index >= remaining)
    {
        return Some("No completed arms-plant slot is available for this project.".into());
    }
    None
}
fn set_blocker(w: &mut WorldState, id: NationId, index: usize, reason: String, paused: bool) {
    let p = &mut state_mut(w.nation_mut(id)).projects[index];
    p.status = if paused {
        ProjectStatus::Paused
    } else {
        ProjectStatus::Blocked
    };
    p.reason = reason;
}

/// One funding date, one physical work day per project at most. Every charge
/// is proportional to work, bounded by the frozen contract, and posted by the
/// existing fiscal settlement. Finished products enter only the Arsenal.
pub fn tick_day(w: &mut WorldState) {
    if !clock::is_daily(w) {
        return;
    }
    let today = clock::absolute_day(w);
    let ids: Vec<_> = w
        .nations
        .iter()
        .filter(|n| {
            n.alive
                && n.equipment
                    .as_ref()
                    .is_some_and(|s| s.finance_from_day <= today && s.last_tick_day != Some(today))
        })
        .map(|n| n.id)
        .collect();
    for id in ids {
        let mut jobs: Vec<_> = w
            .nation(id)
            .equipment
            .as_ref()
            .unwrap()
            .projects
            .iter()
            .enumerate()
            .filter(|(_, p)| {
                !matches!(p.status, ProjectStatus::Complete | ProjectStatus::Cancelled)
            })
            .map(|(i, p)| (p.priority.dispatch_rank(), p.id, i))
            .collect();
        jobs.sort();
        {
            let s = state_mut(w.nation_mut(id));
            s.last_tick_day = Some(today);
            for p in &mut s.projects {
                p.last_spent_bn = 0.0;
                p.last_day = Some(today);
            }
        }
        for (_, _, index) in jobs {
            let job = w.nation(id).equipment.as_ref().unwrap().projects[index].clone();
            if job.started_day >= today {
                continue;
            }
            if job.paused || job.daily_budget_bn <= 0.0 {
                set_blocker(
                    w,
                    id,
                    index,
                    if job.paused {
                        "Paused by the government."
                    } else {
                        "Daily project funding is zero."
                    }
                    .into(),
                    true,
                );
                continue;
            }
            if let Some(e) = live_site_blocker(w, id, &job) {
                set_blocker(w, id, index, e, false);
                continue;
            }
            if profile(w.nation(id), &job.revision_id).is_none() {
                set_blocker(w, id, index, "The saved revision is missing.".into(), false);
                continue;
            }
            let department = if job.kind == ProjectKind::Development {
                4
            } else {
                3
            };
            let available =
                crate::programs::available_bn(w, id, crate::world::BUDGET_DEFENSE, department);
            if available <= 0.0 {
                set_blocker(
                    w,
                    id,
                    index,
                    "No renewed departmental funding is available for this work.".into(),
                    true,
                );
                continue;
            }
            let terms = work_terms(&job);
            let tooling = terms.tooling;
            let rate = terms.rate;
            let stage_remaining = terms.stage_remaining;
            if !rate.is_finite() || rate <= 0.0 || stage_remaining <= 0.0 {
                set_blocker(w, id, index, "The saved work contract is invalid.".into(), false);
                continue;
            }
            let mut advance = 1.0_f64.min(stage_remaining)
                .min(job.daily_budget_bn / rate).min(available / rate);
            let input_bundle = |step: f64| work_inputs(&job, step);
            let desired = input_bundle(advance);
            // Custom manufacture always needs its recipe; the legacy gate flag
            // cannot turn newly designed vehicles into free physical inputs.
            let supply = if desired.iter().any(|x| *x > 0.0) {
                crate::resources::bundle_throughput(w, id, &desired)
            } else {
                1.0
            };
            advance *= supply.clamp(0.0, 1.0);
            if advance <= 1e-12 {
                set_blocker(
                    w,
                    id,
                    index,
                    "Waiting for the physical inputs shown in this project's recipe.".into(),
                    true,
                );
                continue;
            }
            let required = input_bundle(advance);
            let finishing = job.work_days + advance + 1e-9 >= job.minimum_days as f64;
            let payment = if finishing {
                (job.cost_bn - job.spent_bn).max(0.0)
            } else {
                rate * advance
            };
            if payment > available || payment > job.daily_budget_bn + 1e-12 {
                set_blocker(
                    w,
                    id,
                    index,
                    "The remaining contract payment exceeds today's funding.".into(),
                    true,
                );
                continue;
            }
            // Restore only this tiny funding ledger if an exact atomic material
            // preflight refuses. Failed consumption itself is write-free.
            let before = w.nation(id).program_budget.clone();
            let paid = if department == 4 {
                crate::programs::spend_operating(w, id, crate::world::BUDGET_DEFENSE, 4, payment)
            } else {
                crate::programs::spend(w, id, crate::world::BUDGET_DEFENSE, 3, payment)
            };
            if let Err(e) = paid {
                set_blocker(w, id, index, e, true);
                continue;
            }
            if required.iter().any(|x| *x > 0.0) {
                if let Err((c, need, have)) =
                    crate::resources::consume_stockpile_atomic(w, id, &required)
                {
                    w.nation_mut(id).program_budget = before;
                    set_blocker(
                        w,
                        id,
                        index,
                        format!(
                            "Waiting for {}: {:.3} required, {:.3} available.",
                            c.name(),
                            need,
                            have
                        ),
                        true,
                    );
                    continue;
                }
            }
            let work = (job.work_days + advance).min(job.minimum_days as f64);
            let new_units = if job.kind == ProjectKind::Development {
                0
            } else {
                let each =
                    (job.minimum_days - job.tooling_days).max(1) as f64 / job.quantity as f64;
                (((work - job.tooling_days as f64).max(0.0) / each + 1e-9).floor() as u32)
                    .min(job.quantity)
            };
            let units = new_units.saturating_sub(job.completed_units);
            if units > 0 {
                let result = if job.kind == ProjectKind::Refit {
                    crate::arsenal::complete_refit(
                        w.nation_mut(id),
                        job.source_design_id.as_deref().expect("refit has a source"),
                        &job.revision_id,
                        units,
                    )
                } else {
                    // Arsenal settles later on this same date. Reserve that
                    // first decrement so seven full subsequent days remain.
                    crate::arsenal::queue_design_order(
                        w.nation_mut(id),
                        &job.revision_id,
                        units,
                        DELIVERY_DAYS + 1,
                        0.0,
                    )
                };
                // Starts reserve valid source stock and immutable identities.
                // A contradictory imported save must fail visibly, never mint
                // a second product or silently lose the completed contract.
                result.expect(
                    "preflighted equipment stock conversion remains valid during one settlement",
                );
            }
            let s = state_mut(w.nation_mut(id));
            let p = &mut s.projects[index];
            p.work_days = work;
            p.spent_bn += payment;
            p.last_spent_bn = payment;
            p.completed_units = new_units;
            for (used, amount) in p.resources_used.iter_mut().zip(required) {
                *used += amount;
            }
            p.status = if finishing {
                ProjectStatus::Complete
            } else {
                ProjectStatus::Working
            };
            p.reason = if finishing {
                "Completed. Vehicles and pending deliveries are recorded in the Arsenal."
            } else if tooling {
                "Preparing the production line."
            } else {
                "Funded work is progressing."
            }
            .into();
            if finishing {
                p.completed_day = Some(today);
                if job.kind == ProjectKind::Development {
                    s.revisions.get_mut(&job.revision_id).unwrap().certified_day = Some(today);
                }
            }
        }
        settle_maintenance(w, id);
    }
}

fn settle_maintenance(w: &mut WorldState, id: NationId) {
    // Actual invoices settle once after today's deliveries. The old earmark
    // remains the compatibility path until a reviewed plan takes effect.
    if maintenance_plan_on(w.nation(id),clock::absolute_day(w)){return;}
    let required = fleet_maintenance_requirement(w.nation(id));
    // Department2 already records this service payment. Earmarking its limited
    // envelope supports vehicles and reduces legacy magazine refill through
    // the shared integration helper; it never creates another treasury charge.
    let envelope = w
        .nation(id)
        .program_budget
        .as_ref()
        .map_or(0.0, |p| p.spent_today_bn[crate::world::BUDGET_DEFENSE][2]);
    let s = state_mut(w.nation_mut(id));
    s.maintenance_required_today_bn = required;
    s.maintenance_allocated_today_bn = required.min(envelope.max(0.0));
    s.maintenance_fraction = if required > 0.0 {
        (envelope / required).clamp(0.0, 1.0)
    } else {
        1.0
    };
}

/// Refresh support after Arsenal delivery/ageing, before battlefield snapshots.
/// This only earmarks the already-posted service envelope; calling it twice
/// makes no additional expenditure and newly delivered vehicles are included.
pub fn settle_support(w: &mut WorldState) {
    if !clock::is_daily(w) {
        return;
    }
    let day = clock::absolute_day(w);
    let ids: Vec<_> = w
        .nations
        .iter()
        .filter(|n| {
            n.alive
                && n.equipment
                    .as_ref()
                    .is_some_and(|s| s.finance_from_day <= day || s.maintenance_plan.as_ref().is_some_and(|p|p.from_day<=day))
                && n.program_budget
                    .as_ref()
                    .is_some_and(|p| p.day == Some(day))
        })
        .map(|n| n.id)
        .collect();
    for id in ids {
        if maintenance_plan_on(w.nation(id),day){settle_fleet_maintenance(w,id);}else{settle_maintenance(w, id);}
        tick_ammunition_work(w,id);
        tick_ammunition_reserves(w,id);
    }
}

/// Validate persisted identities and conservation before any daily work. New
/// saves never repair malformed stock by inventing missing models or vehicles.
pub fn validate_state(n: &Nation) -> Result<(), String> {
    let Some(s) = &n.equipment else {
        if n.arsenal.held.iter().any(|h| h.design_id.is_some())
            || n.arsenal.orders.iter().any(|o| o.design_id.is_some())
        {
            return Err("Custom Arsenal equipment is missing its design library.".into());
        }
        return Ok(());
    };
    let fail = |detail: &str| format!("Invalid equipment state for {}: {detail}", n.id.name());
    if !(1..=VERSION).contains(&s.version) {
        return Err(fail("unsupported equipment version"));
    }
    validate_fleet_targets(n)?;
    validate_maintenance_plan(n)?;
    validate_ammunition(n)?;
    validate_ammunition_reserves(n)?;
    validate_supply_automation(n)?;
    if s.revisions.len() > MAX_REVISIONS
        || s.projects.len() > MAX_PROJECTS
        || s.drafts.len() > MAX_REVISIONS
    {
        return Err(fail("library limit exceeded"));
    }
    if !s.force_support_scale.is_finite()
        || s.force_support_scale < 1.0
        || !s.research_progress.is_finite()
        || s.research_progress < 0.0
    {
        return Err(fail("invalid reference support or research progress"));
    }
    for x in [s.development_daily_limit_bn, s.procurement_daily_limit_bn]
        .into_iter()
        .flatten()
    {
        if budget_refusal(x).is_some() {
            return Err(fail("invalid funding limit"));
        }
    }
    if [
        s.maintenance_allocated_today_bn,
        s.maintenance_required_today_bn,
        s.maintenance_fraction,
    ]
    .iter()
    .any(|v| !v.is_finite() || *v < 0.0)
        || s.maintenance_fraction > 1.0
        || s.maintenance_allocated_today_bn > s.maintenance_required_today_bn + 1e-12
    {
        return Err(fail("invalid maintenance receipt"));
    }
    if s.active_research
        .as_deref()
        .is_some_and(|id| research(id).is_none())
        || s.learned.iter().any(|id| research(id).is_none())
    {
        return Err(fail("unknown component research reference"));
    }
    if s.learned.iter().chain(s.active_research.iter()).any(|id| {
        research_prerequisites(id).iter().any(|required| !s.learned.contains(*required))
    }) {
        return Err(fail("missing prerequisite component research"));
    }
    for (key, r) in &s.revisions {
        let p = &r.profile;
        if key != &r.id
            || r.id.is_empty()
            || name_refusal(&r.name).is_some()
            || r.specification_key != specification_key(&r.spec)
            || p.rules_version != spec_version(&r.spec)
            || (s.version == 1 && detailed_spec(&r.spec))
            || (s.version < 3 && is_ground_platform(&r.spec.platform))
            || (s.version < 8 && is_aviation_platform(&r.spec.platform))
            || !PLATFORMS.iter().any(|p| p.id == r.spec.platform)
            || !configuration_refusals(&r.spec).is_empty()
            || r.spec.components.len() != slots_for(&r.spec).len()
            || slots_for(&r.spec).iter().any(|slot| r.spec.components.get(*slot).and_then(|id| component(id)).is_none_or(|c| c.slot != *slot))
        {
            return Err(fail("invalid immutable design identity"));
        }
        if [
            p.unit_cost_bn,
            p.fabrication_cost_bn,
            p.development_cost_bn,
            p.tooling_cost_bn,
            p.maintenance_bn_day,
            p.land,
            p.protection,
            p.mobility,
            p.recon,
            p.land_factor,
            p.reference_weight_bn,
        ]
        .iter()
        .any(|v| !v.is_finite() || *v <= 0.0)
            || p.recipe.iter().any(|v| !v.is_finite() || *v < 0.0)
            || !(0.75..=1.25).contains(&p.land_factor)
            || p.ground_roles.is_some() != is_ground_platform(&r.spec.platform)
            || p.ground_roles.is_some_and(|roles| !roles.valid())
            || !aviation_frozen_profile_valid(&r.spec, p)
            || p.service_months == 0
            || p.production_days == 0
            || p.development_days == 0
            || p.tooling_days == 0
            || p.installation_used > p.installation_capacity
            || slots_for(&r.spec).iter().any(|slot| {
                !p.component_costs
                    .get(*slot)
                    .is_some_and(|v| v.is_finite() && *v > 0.0)
            })
        {
            return Err(fail("invalid frozen design profile"));
        }
    }
    let mut ids = BTreeSet::new();
    let mut reserved: BTreeMap<&str, u32> = BTreeMap::new();
    for p in &s.projects {
        if !ids.insert(p.id)
            || p.id >= s.next_id
            || !s.revisions.contains_key(&p.revision_id)
            || p.quantity == 0
            || p.quantity > MAX_BATCH
            || p.completed_units > p.quantity
            || p.minimum_days == 0
            || p.minimum_days > 1_000_000
            || p.tooling_days >= p.minimum_days
            || [
                p.work_days,
                p.cost_bn,
                p.spent_bn,
                p.tooling_cost_bn,
                p.last_spent_bn,
            ]
            .iter()
            .any(|v| !v.is_finite() || *v < 0.0)
            || p.cost_bn <= 0.0
            || p.spent_bn > p.cost_bn + 1e-9
            || p.work_days > p.minimum_days as f64 + 1e-9
            || p.recipe_per_unit
                .iter()
                .chain(p.resources_used.iter())
                .any(|v| !v.is_finite() || *v < 0.0)
            || budget_refusal(p.daily_budget_bn).is_some()
        {
            return Err(fail("invalid project contract, progress or identity"));
        }
        if p.kind != ProjectKind::Development && p.district.as_deref().is_none_or(str::is_empty) {
            return Err(fail("production/refit site is missing"));
        }
        let revision = &s.revisions[&p.revision_id];
        let (
            expected_cost,
            expected_days,
            expected_tooling,
            expected_tooling_days,
            expected_recipe,
        ) = match p.kind {
            ProjectKind::Development => (
                revision.profile.development_cost_bn,
                revision.profile.development_days,
                0.0,
                0,
                [0.0; 12],
            ),
            ProjectKind::Production => {
                if revision.certified_day.is_none() {
                    return Err(fail("production revision is not certified"));
                }
                let r = &revision.profile;
                (
                    r.tooling_cost_bn + r.fabrication_cost_bn * p.quantity as f64,
                    r.tooling_days + r.production_days * p.quantity,
                    r.tooling_cost_bn,
                    r.tooling_days,
                    r.recipe,
                )
            }
            ProjectKind::Refit => {
                let source = p
                    .source_design_id
                    .as_deref()
                    .and_then(|id| s.revisions.get(id))
                    .ok_or_else(|| fail("refit source revision is missing"))?;
                if source.certified_day.is_none() || revision.certified_day.is_none() {
                    return Err(fail("refit revisions are not certified"));
                }
                let (cost, days, recipe) = refit_terms(source, revision)
                    .map_err(|_| fail("incompatible refit contract"))?;
                (cost * p.quantity as f64, days * p.quantity, 0.0, 0, recipe)
            }
        };
        let near = |a: f64, b: f64| (a - b).abs() <= 1e-9_f64.max(b.abs() * 1e-12);
        if !near(p.cost_bn, expected_cost)
            || p.minimum_days != expected_days
            || !near(p.tooling_cost_bn, expected_tooling)
            || p.tooling_days != expected_tooling_days
            || p.recipe_per_unit
                .iter()
                .zip(expected_recipe)
                .any(|(a, b)| !near(*a, b))
        {
            return Err(fail("project differs from its frozen revision contract"));
        }
        let manufacturing_work = (p.work_days - p.tooling_days as f64).max(0.0);
        let work_fraction = manufacturing_work / (p.minimum_days - p.tooling_days) as f64;
        let expected_spent = if p.tooling_days > 0 && p.work_days <= p.tooling_days as f64 {
            p.tooling_cost_bn * p.work_days / p.tooling_days as f64
        } else {
            p.tooling_cost_bn + (p.cost_bn - p.tooling_cost_bn) * work_fraction
        };
        let units_worked = if p.kind == ProjectKind::Development {
            0.0
        } else {
            work_fraction * p.quantity as f64
        };
        let expected_units = (units_worked + 1e-9).floor() as u32;
        let batch = p.recipe_per_unit.map(|x| x * p.quantity as f64);
        let expected_inputs =
            crate::resources::scale_bundle(&batch, units_worked / p.quantity as f64);
        if !near(p.spent_bn, expected_spent)
            || p.completed_units != expected_units
            || p.resources_used
                .iter()
                .zip(expected_inputs)
                .any(|(a, b)| !near(*a, b))
        {
            return Err(fail(
                "project money, materials or completed units disagree with paid work",
            ));
        }
        let finished = near(p.work_days, p.minimum_days as f64);
        if (p.status == ProjectStatus::Complete) != finished
            || (p.status == ProjectStatus::Complete && p.completed_day.is_none())
            || (p.kind == ProjectKind::Development
                && p.status == ProjectStatus::Complete
                && revision.certified_day.is_none())
        {
            return Err(fail("project completion state is inconsistent"));
        }
        if p.kind == ProjectKind::Refit {
            let source = p
                .source_design_id
                .as_deref()
                .ok_or_else(|| fail("refit source is missing"))?;
            if !s.revisions.contains_key(source) {
                return Err(fail("refit source revision is missing"));
            }
            if !matches!(p.status, ProjectStatus::Complete | ProjectStatus::Cancelled) {
                let amount = reserved.entry(source).or_default();
                *amount = amount
                    .checked_add(p.quantity - p.completed_units)
                    .ok_or_else(|| fail("refit reservation overflow"))?;
            }
        }
    }
    for h in &n.arsenal.held {
        if let Some(id) = h.design_id.as_deref() {
            if !s.revisions.contains_key(id)
                || s.revisions
                    .get(id)
                    .is_some_and(|r| r.certified_day.is_none())
                || Some(h.kit) != s.revisions.get(id).and_then(|r| crate::arsenal::index_of(design_base_kit(&r.spec)))
                || !h.units.is_finite()
                || h.units < 0.0
                || h.units.fract() != 0.0
                || h.units > u32::MAX as f64
                || h.refit_reserved as f64 > h.units
                || !h.age.is_finite()
                || h.age < 0.0
                || !h.loss_remainder.is_finite()
                || !(0.0..1.0).contains(&h.loss_remainder)
            {
                return Err(fail("invalid custom holding"));
            }
            if reserved.remove(id).unwrap_or(0) != h.refit_reserved {
                return Err(fail("held refit reservations do not match active projects"));
            }
        }
    }
    if reserved.values().any(|x| *x > 0) {
        return Err(fail("a refit has no source holding"));
    }
    for o in &n.arsenal.orders {
        if let Some(id) = o.design_id.as_deref() {
            if !s.revisions.contains_key(id)
                || s.revisions
                    .get(id)
                    .is_some_and(|r| r.certified_day.is_none())
                || Some(o.kit) != s.revisions.get(id).and_then(|r| crate::arsenal::index_of(design_base_kit(&r.spec)))
                || !o.units.is_finite()
                || o.units <= 0.0
                || o.units.fract() != 0.0
                || o.units > u32::MAX as f64
                || o.due_days.is_none_or(|d| d == 0)
                || o.delivery_age.is_none_or(|x| !x.is_finite() || x < 0.0)
            {
                return Err(fail("invalid custom delivery order"));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        init::world_1990,
        resources::{self, Commodity},
        world::{GameRules, BUDGET_DEFENSE},
    };
    const USA: NationId = NationId::USA;
    pub(super) fn fixture() -> (WorldState, String) {
        let mut w = world_1990(GameRules {
            daily_simulation: true,
            military_operations: true,
            production_system: true,
            manufacturing_system: true,
            resource_market: true,
            resource_gates: true,
            ..GameRules::default()
        });
        w.player = Some(USA);
        crate::programs::set_construction_budget(&mut w, USA, 0.001).unwrap();
        let district = w
            .districts
            .iter()
            .find(|(_, n)| **n == USA)
            .unwrap()
            .0
            .clone();
        for _ in 0..3 {
            crate::production::complete_capability(
                &mut w,
                &district,
                crate::production::ProjectKind::ArmsPlant,
            );
        }
        for c in [Commodity::Iron, Commodity::Coal, Commodity::Copper] {
            resources::set_stockpile_for_test(&mut w, USA, c, 10_000.0);
        }
        (w, district)
    }
    fn next_work_day(w: &mut WorldState) {
        let (year, month, day) = clock::date_from_day(clock::absolute_day(w) + 1);
        w.year = year;
        w.month = month;
        w.day = day;
        crate::programs::begin_day(w);
        tick_day(w);
        crate::programs::finish_day(w);
    }
    fn finish_development(w: &mut WorldState, name: &str, spec: DesignSpec) -> String {
        let id = start_development(w, USA, name, spec, 1.0).unwrap();
        let min = profile(w.nation(USA), &id).unwrap().development_days;
        for _ in 0..min {
            next_work_day(w);
        }
        assert!(certified(w.nation(USA), &id).is_ok());
        id
    }
    #[test]
    fn ground_families_complete_paid_development_and_share_physical_production() {
        for platform in PLATFORMS.iter().filter(|p| is_ground_platform(p.id)) {
            let (mut w, district) = fixture();
            let revision = finish_development(&mut w, platform.name, default_spec(platform.id));
            let frozen = profile(w.nation(USA), &revision).unwrap().clone();
            assert!(frozen.ground_roles.is_some());
            let stock: [f64; 12] = std::array::from_fn(|i| resources::stockpile(&w,USA,crate::resources::ALL[i]));
            let project = start_production(&mut w,USA,&revision,&district,2,1.0).unwrap();
            let days = production_quote(&w,USA,&revision,&district,2,1.0).minimum_days;
            assert_eq!(crate::manufacturing::used_slots(&w,USA,&district),1);
            for _ in 0..days {
                clock::advance_date(&mut w);
                if w.nation(USA).program_budget.as_ref().unwrap().fiscal_year != w.year {
                    crate::programs::set_construction_budget(&mut w,USA,0.001).unwrap();
                }
                crate::programs::begin_day(&mut w); tick_day(&mut w); crate::programs::finish_day(&mut w);
            }
            let job = w.nation(USA).equipment.as_ref().unwrap().projects.iter().find(|p|p.id==project).unwrap();
            assert_eq!(job.status,ProjectStatus::Complete,"{}",platform.id);
            assert_eq!(job.completed_units,2);
            assert_eq!(job.spent_bn,job.cost_bn);
            for (i, amount) in frozen.recipe.iter().enumerate() {
                assert!((job.resources_used[i]-amount*2.0).abs()<1e-9);
                assert!((stock[i]-resources::stockpile(&w,USA,crate::resources::ALL[i])-amount*2.0).abs()<1e-8);
            }
            assert_eq!(crate::manufacturing::used_slots(&w,USA,&district),0);
            let n=w.nation_mut(USA);
            assert_eq!(n.arsenal.orders.iter().filter(|o|o.design_id.as_deref()==Some(&revision)).map(|o|o.units).sum::<f64>(),2.0);
            assert!(n.arsenal.held.iter().all(|h|h.design_id.as_deref()!=Some(&revision)));
            n.mil_spend_gdp=0.0; n.arsenal.banked=0.0;
            for _ in 0..=DELIVERY_DAYS {
                clock::advance_date(&mut w); crate::programs::begin_day(&mut w);
                crate::arsenal::tick(&mut w); crate::programs::finish_day(&mut w);
            }
            assert_eq!(w.nation(USA).arsenal.held.iter().filter(|h|h.design_id.as_deref()==Some(&revision)).map(|h|h.units).sum::<f64>(),2.0);
            assert_eq!(profile(w.nation(USA),&revision).unwrap(),&frozen);
            validate_state(w.nation(USA)).unwrap();
            let raw=crate::save(&w); assert_eq!(crate::save(&crate::load(&raw).unwrap()),raw);
        }
    }
    #[test]
    fn previews_and_unfunded_drafts_are_pure_and_locked_components_are_not_developed() {
        let (mut w, _) = fixture();
        let before = crate::save(&w);
        let spec = baseline_spec();
        let v = design_preview(&w, USA, &spec);
        assert!(v.valid);
        assert_eq!(crate::save(&w), before);
        let mut locked = spec.clone();
        locked
            .components
            .insert("sensors".into(), "sensors_integrated".into());
        let v = design_preview(&w, USA, &locked);
        assert!(!v.valid);
        assert!(v.profile.is_some());
        assert!(start_development(&mut w, USA, "Too soon", locked.clone(), 1.0).is_err());
        assert_eq!(crate::save(&w), before);
        let funding = w.nation(USA).program_budget.clone();
        let strength = crate::war::sustained_force(w.nation(USA), w.nation(USA).mil_spend_gdp);
        save_draft(&mut w, USA, "Future model", locked).unwrap();
        assert_eq!(
            w.nation(USA).equipment.as_ref().unwrap().finance_from_day,
            i32::MAX
        );
        assert_eq!(w.nation(USA).program_budget, funding);
        assert_eq!(
            crate::war::sustained_force(w.nation(USA), w.nation(USA).mil_spend_gdp),
            strength
        );
        let mut invalid = spec;
        invalid
            .components
            .insert("armament".into(), "drive_standard".into());
        assert!(!design_preview(&w, USA, &invalid).valid);
    }

    #[test]
    fn separate_specifications_have_valid_distinct_types_and_require_every_slot() {
        let (w,_) = fixture();let before=crate::save(&w);let mut prices=vec![];
        for platform in PLATFORMS.iter().filter(|p| p.id.starts_with("tank_")) {
            let spec=tank_spec(platform.id);let preview=design_preview(&w,USA,&spec);
            assert!(preview.valid,"{}: {:?}",platform.id,preview.blockers);
            let p=preview.profile.unwrap();assert_eq!(p.rules_version,2);assert_eq!(p.component_costs.len(),12);prices.push(p.unit_cost_bn);
            for slot in DESIGN_SLOTS {let mut missing=spec.clone();missing.components.remove(slot);assert!(!design_preview(&w,USA,&missing).valid,"{slot}");}
        }
        prices.sort_by(f64::total_cmp);prices.dedup();assert_eq!(prices.len(),4);assert_eq!(crate::save(&w),before);
    }

    #[test]
    fn gun_mounting_and_light_chassis_limits_are_enforced_before_spending() {
        let (mut w,_) = fixture();let mut spec=tank_spec("tank_light");spec.components.insert("armament".into(),"gun_120".into());
        let before=crate::save(&w);let p=design_preview(&w,USA,&spec);assert!(!p.valid);assert!(p.blockers.iter().any(|s|s.contains("light chassis")));
        assert!(start_development(&mut w,USA,"Invalid light",spec,1.0).is_err());assert_eq!(crate::save(&w),before);
        let mut spec=tank_spec("tank_standard");spec.components.insert("turret".into(),"turret_casemate".into());assert!(!design_preview(&w,USA,&spec).valid);
        let mut spec=tank_spec("tank_destroyer");spec.components.insert("turret".into(),"turret_standard".into());assert!(!design_preview(&w,USA,&spec).valid);
        let mut spec=tank_spec("tank_heavy");spec.components.insert("armament".into(),"gun_125".into());assert!(!design_preview(&w,USA,&spec).valid);
    }

    #[test]
    fn each_new_specification_has_a_separate_price_or_rating_and_research_gate() {
        let (mut w,_) = fixture();let base=tank_spec("tank_standard");let original=design_preview(&w,USA,&base).profile.unwrap();
        for c in SPEC_COMPONENTS {
            if base.components.get(c.slot).is_some_and(|id|id==c.id){continue;}
            let mut spec=base.clone();spec.components.insert(c.slot.into(),c.id.into());let p=design_preview(&w,USA,&spec);
            assert_ne!(p.profile.as_ref().unwrap(),&original,"{} must change the review",c.id);
            if c.research.is_some() || c.technology.is_some(){assert!(!p.valid,"{} requires research",c.id);}
        }
        let mut spec=tank_spec("tank_heavy");spec.components.insert("active_protection".into(),"aps_hard".into());
        let index=crate::tech::index_of("aero_active_protection_system").unwrap();w.nation_mut(USA).tech.known.push(index);w.nation_mut(USA).tech.known.sort();
        let p=design_preview(&w,USA,&spec);assert!(p.valid,"{:?}",p.blockers);assert!(p.profile.unwrap().protection>1.6);
    }

    #[test]
    fn old_revisions_keep_identity_and_prices_when_a_detailed_draft_is_saved() {
        let (mut w,_) = fixture();let id=start_development(&mut w,USA,"Legacy",baseline_spec(),0.0).unwrap();
        w.nation_mut(USA).equipment.as_mut().unwrap().version=1;
        let raw=crate::save(&w);let mut restored=crate::load(&raw).unwrap();assert_eq!(crate::save(&restored),raw);
        let old=restored.nation(USA).equipment.as_ref().unwrap().revisions[&id].clone();assert!(old.specification_key.starts_with("v1|"));
        save_draft(&mut restored,USA,"Detailed",tank_spec("tank_light")).unwrap();
        let state=restored.nation(USA).equipment.as_ref().unwrap();assert_eq!(state.version,VERSION);assert_eq!(state.revisions[&id],old);
        let raw=crate::save(&restored);assert_eq!(crate::save(&crate::load(&raw).unwrap()),raw);
        let expanded=editable_spec(&old.spec);assert_eq!(expanded.components.len(),12);assert!(design_preview(&restored,USA,&expanded).valid);
    }

    #[test]
    fn detailed_revision_development_and_refit_quotes_survive_reload() {
        let (mut w,_) = fixture();let source=finish_development(&mut w,"Detailed main",tank_spec("tank_standard"));
        let mut spec=tank_spec("tank_standard");spec.components.insert("tracks".into(),"tracks_padded".into());spec.components.insert("ammunition".into(),"ammo_support".into());
        let target=start_development(&mut w,USA,"Revised main",spec,0.0001).unwrap();let state=w.nation(USA).equipment.as_ref().unwrap();
        let (cost,days,recipe)=refit_terms(&state.revisions[&source],&state.revisions[&target]).unwrap();assert!(cost>0.0);assert_eq!(days,50);assert!(recipe.iter().any(|v|*v>0.0));
        validate_state(w.nation(USA)).unwrap();let raw=crate::save(&w);assert_eq!(crate::save(&crate::load(&raw).unwrap()),raw);
    }
    #[test]
    fn component_research_consumes_effort_but_changes_no_stock_or_macro_knowledge() {
        let (mut w, _) = fixture();
        let known = w.nation(USA).tech.count();
        let stock = serde_json::to_string(&w.nation(USA).arsenal).unwrap();
        let force = crate::war::sustained_force(w.nation(USA), w.nation(USA).mil_spend_gdp);
        start_research(&mut w, USA, "tank_fire_control_1990").unwrap();
        let day = clock::absolute_day(&w);
        assert!(research_step(w.nation_mut(USA), 16.0, 1990, day));
        assert!(research_step(w.nation_mut(USA), 16.0, 1990, day));
        assert!(!w
            .nation(USA)
            .equipment
            .as_ref()
            .unwrap()
            .learned
            .contains("tank_fire_control_1990"));
        assert!(research_step(w.nation_mut(USA), 16.0, 1990, day + 1));
        assert!(w
            .nation(USA)
            .equipment
            .as_ref()
            .unwrap()
            .learned
            .contains("tank_fire_control_1990"));
        assert_eq!(w.nation(USA).tech.count(), known);
        assert_eq!(
            serde_json::to_string(&w.nation(USA).arsenal).unwrap(),
            stock
        );
        assert_eq!(
            crate::war::sustained_force(w.nation(USA), w.nation(USA).mil_spend_gdp),
            force
        );
        let mut spec = baseline_spec();
        spec.components
            .insert("sensors".into(), "sensors_integrated".into());
        assert!(design_preview(&w, USA, &spec).valid);
    }
    #[test]
    fn research_focus_switches_charge_one_half_bank_and_shared_quota_survives_reload() {
        let (mut w, _) = fixture();
        let di = crate::tech::Domain::Aerospace.index();
        w.nation_mut(USA).political_capital = 1000.0;
        w.nation_mut(USA).tech.progress[di] = 20.0;
        let select = crate::Command::Equipment {
            nation: USA,
            order: crate::EquipmentOrder::Research {
                component: "tank_fire_control_1990".into(),
            },
        };
        crate::apply_command(&mut w, &select).unwrap();
        assert_eq!(
            w.nation(USA).equipment.as_ref().unwrap().research_progress,
            10.0
        );
        assert_eq!(w.nation(USA).tech.progress[di], 0.0);
        crate::apply_command(
            &mut w,
            &crate::Command::SetResearchFocus {
                nation: USA,
                domain: crate::tech::Domain::Aerospace,
                tech: None,
            },
        )
        .unwrap();
        assert_eq!(w.nation(USA).tech.progress[di], 5.0);
        assert_eq!(
            w.nation(USA).equipment.as_ref().unwrap().research_progress,
            0.0
        );
        crate::apply_command(&mut w, &select).unwrap();
        assert_eq!(
            w.nation(USA).equipment.as_ref().unwrap().research_progress,
            2.5
        );
        w.day = 31;
        let month = clock::month_index(&w);
        w.nation_mut(USA).tech.acquisition_quota = Some(crate::tech::AcquisitionQuota {
            month_index: month,
            acquired: [6; crate::tech::DOMAIN_COUNT],
            migration_hold: false,
        });
        w.nation_mut(USA)
            .equipment
            .as_mut()
            .unwrap()
            .research_progress = 100.0;
        crate::tech::tick(&mut w);
        assert!(w
            .nation(USA)
            .equipment
            .as_ref()
            .unwrap()
            .active_research
            .is_some());
        assert_eq!(
            w.nation(USA).tech.acquisition_quota.unwrap().acquired[di],
            6
        );
        let mut loaded = crate::load(&crate::save(&w)).unwrap();
        clock::advance_date(&mut w);
        clock::advance_date(&mut loaded);
        crate::tech::tick(&mut w);
        crate::tech::tick(&mut loaded);
        assert!(w
            .nation(USA)
            .equipment
            .as_ref()
            .unwrap()
            .learned
            .contains("tank_fire_control_1990"));
        assert_eq!(
            w.nation(USA).tech.acquisition_quota.unwrap().acquired[di],
            1
        );
        assert_eq!(crate::save(&w), crate::save(&loaded));
    }
    #[test]
    fn disabled_military_rules_refuse_design_work_without_mutation() {
        let (mut w, _) = fixture();
        w.rules.military_operations = false;
        let before = crate::save(&w);
        assert!(
            start_development(&mut w, USA, "Unsupported", baseline_spec(), 1.0)
                .unwrap_err()
                .contains("military operations")
        );
        assert_eq!(crate::save(&w), before);
    }
    #[test]
    fn development_obeys_activation_budget_calendar_and_exact_contract_once() {
        let (mut w, _) = fixture();
        crate::programs::begin_day(&mut w);
        let funding = w.nation(USA).program_budget.clone();
        let today = clock::absolute_day(&w);
        let revision = start_development(&mut w, USA, "Model A", baseline_spec(), 0.0).unwrap();
        let p = w.nation(USA).equipment.as_ref().unwrap().projects[0].clone();
        assert_eq!(w.nation(USA).program_budget, funding);
        assert_eq!(
            w.nation(USA).equipment.as_ref().unwrap().finance_from_day,
            today + 1
        );
        next_work_day(&mut w);
        assert_eq!(
            w.nation(USA).equipment.as_ref().unwrap().projects[0].spent_bn,
            0.0
        );
        let rate = p.cost_bn / p.minimum_days as f64;
        set_project_budget(&mut w, USA, p.id, rate / 2.0).unwrap();
        next_work_day(&mut w);
        let first = &w.nation(USA).equipment.as_ref().unwrap().projects[0];
        assert!((first.work_days - 0.5).abs() < 1e-12);
        assert!((first.spent_bn - rate / 2.0).abs() < 1e-12);
        let settled = crate::save(&w);
        tick_day(&mut w);
        assert_eq!(crate::save(&w), settled);
        set_project_budget(&mut w, USA, p.id, 1000.0).unwrap();
        for _ in 0..p.minimum_days - 1 {
            next_work_day(&mut w);
        }
        assert!(
            w.nation(USA).equipment.as_ref().unwrap().revisions[&revision]
                .certified_day
                .is_none()
        );
        next_work_day(&mut w);
        let completed = &w.nation(USA).equipment.as_ref().unwrap().projects[0];
        assert_eq!(completed.status, ProjectStatus::Complete);
        assert_eq!(completed.spent_bn, p.cost_bn);
        assert!(completed.work_days <= p.minimum_days as f64);
        let receipt = w.nation(USA).program_budget.as_ref().unwrap();
        assert_eq!(
            receipt.spent_today_bn[BUDGET_DEFENSE][4],
            completed.last_spent_bn
        );
        assert_eq!(
            receipt.noncapital_spent_today_bn[BUDGET_DEFENSE][4],
            completed.last_spent_bn
        );
        validate_state(w.nation(USA)).unwrap();
    }
    #[test]
    fn production_requires_materials_and_delivers_only_whole_revisioned_orders() {
        let (mut w, d) = fixture();
        let revision = finish_development(&mut w, "Model A", baseline_spec());
        let id = start_production(&mut w, USA, &revision, &d, 1, 1.0).unwrap();
        let days = profile(w.nation(USA), &revision).unwrap().tooling_days;
        for _ in 0..days {
            next_work_day(&mut w);
        }
        resources::set_stockpile_for_test(&mut w, USA, Commodity::Copper, 0.0);
        let before = w
            .nation(USA)
            .equipment
            .as_ref()
            .unwrap()
            .projects
            .iter()
            .find(|p| p.id == id)
            .unwrap()
            .clone();
        next_work_day(&mut w);
        let paused = w
            .nation(USA)
            .equipment
            .as_ref()
            .unwrap()
            .projects
            .iter()
            .find(|p| p.id == id)
            .unwrap();
        assert_eq!(paused.spent_bn, before.spent_bn);
        assert_eq!(paused.work_days, before.work_days);
        assert!(w
            .nation(USA)
            .arsenal
            .orders
            .iter()
            .all(|o| o.design_id.is_none()));
        resources::set_stockpile_for_test(&mut w, USA, Commodity::Copper, 10.0);
        for _ in 0..profile(w.nation(USA), &revision).unwrap().production_days - 1 {
            next_work_day(&mut w);
        }
        assert!(w
            .nation(USA)
            .arsenal
            .orders
            .iter()
            .all(|o| o.design_id.is_none()));
        next_work_day(&mut w);
        let order = w
            .nation(USA)
            .arsenal
            .orders
            .iter()
            .find(|o| o.design_id.as_deref() == Some(&revision))
            .unwrap();
        assert_eq!(order.units, 1.0);
        assert_eq!(order.due_days, Some(DELIVERY_DAYS + 1));
        assert!(w
            .nation(USA)
            .arsenal
            .held
            .iter()
            .all(|h| h.design_id.is_none()));
        let job = w
            .nation(USA)
            .equipment
            .as_ref()
            .unwrap()
            .projects
            .iter()
            .find(|p| p.id == id)
            .unwrap();
        assert_eq!(job.spent_bn, job.cost_bn);
        for (used, want) in job
            .resources_used
            .iter()
            .zip(profile(w.nation(USA), &revision).unwrap().recipe)
        {
            assert!((used - want).abs() < 1e-9);
        }
        validate_state(w.nation(USA)).unwrap();
    }
    #[test]
    fn refit_reserves_real_stock_cancel_releases_it_and_completion_keeps_age() {
        let (mut w, d) = fixture();
        state_mut(w.nation_mut(USA))
            .learned
            .insert("tank_fire_control_1990".into());
        let source = start_development(&mut w, USA, "Original", baseline_spec(), 1.0).unwrap();
        let mut target_spec = baseline_spec();
        target_spec
            .components
            .insert("sensors".into(), "sensors_integrated".into());
        let target =
            start_development(&mut w, USA, "Observation upgrade", target_spec, 1.0).unwrap();
        for _ in 0..profile(w.nation(USA), &target).unwrap().development_days {
            next_work_day(&mut w);
        }
        crate::arsenal::deliver_design(w.nation_mut(USA), &source, 2, 96.0).unwrap();
        w.rules.resource_market = false;
        let before = crate::save(&w);
        assert!(start_refit(&mut w, USA, &source, &target, &d, 1, 1.0)
            .unwrap_err()
            .contains("resource market"));
        assert_eq!(crate::save(&w), before);
        w.rules.resource_market = true;
        let id = start_refit(&mut w, USA, &source, &target, &d, 1, 1.0).unwrap();
        let mut corrupt = w.clone();
        corrupt
            .nation_mut(USA)
            .equipment
            .as_mut()
            .unwrap()
            .projects
            .iter_mut()
            .find(|p| p.id == id)
            .unwrap()
            .revision_id = source.clone();
        assert!(validate_state(corrupt.nation(USA))
            .unwrap_err()
            .contains("incompatible refit"));
        assert_eq!(
            w.nation(USA)
                .arsenal
                .held
                .iter()
                .find(|h| h.design_id.as_deref() == Some(&source))
                .unwrap()
                .refit_reserved,
            1
        );
        next_work_day(&mut w);
        let sunk = w
            .nation(USA)
            .equipment
            .as_ref()
            .unwrap()
            .projects
            .iter()
            .find(|p| p.id == id)
            .unwrap()
            .spent_bn;
        assert!(sunk > 0.0);
        cancel_project(&mut w, USA, id).unwrap();
        assert_eq!(
            w.nation(USA)
                .equipment
                .as_ref()
                .unwrap()
                .projects
                .iter()
                .find(|p| p.id == id)
                .unwrap()
                .spent_bn,
            sunk
        );
        let id = start_refit(&mut w, USA, &source, &target, &d, 1, 1.0).unwrap();
        let days = w
            .nation(USA)
            .equipment
            .as_ref()
            .unwrap()
            .projects
            .iter()
            .find(|p| p.id == id)
            .unwrap()
            .minimum_days;
        // Model ordinary ageing of the held platform while it is in the shop.
        w.nation_mut(USA)
            .arsenal
            .held
            .iter_mut()
            .find(|h| h.design_id.as_deref() == Some(&source))
            .unwrap()
            .age = 99.0;
        for _ in 0..days {
            next_work_day(&mut w);
        }
        let custom: Vec<_> = w
            .nation(USA)
            .arsenal
            .held
            .iter()
            .filter(|h| h.design_id.is_some())
            .collect();
        assert_eq!(custom.iter().map(|h| h.units).sum::<f64>(), 2.0);
        assert_eq!(
            custom
                .iter()
                .find(|h| h.design_id.as_deref() == Some(&target))
                .unwrap()
                .age,
            99.0
        );
        assert!(custom.iter().all(|h| h.refit_reserved == 0));
        validate_state(w.nation(USA)).unwrap();
    }
    #[test]
    fn saved_midwork_continues_identically_and_revision_prices_are_frozen() {
        let (mut w, _) = fixture();
        let revision =
            start_development(&mut w, USA, "Stable model", baseline_spec(), 0.0002).unwrap();
        for _ in 0..7 {
            next_work_day(&mut w);
        }
        let mut loaded = crate::load(&crate::save(&w)).unwrap();
        assert_eq!(crate::save(&loaded), crate::save(&w));
        for _ in 0..9 {
            next_work_day(&mut w);
            next_work_day(&mut loaded);
        }
        assert_eq!(crate::save(&loaded), crate::save(&w));
        let before = profile(w.nation(USA), &revision).unwrap().clone();
        let mut draft = baseline_spec();
        draft
            .components
            .insert("mobility".into(), "drive_mobile".into());
        save_draft(&mut w, USA, "Stable model", draft).unwrap();
        assert_eq!(profile(w.nation(USA), &revision), Some(&before));
    }
    #[test]
    fn actual_system_order_keeps_seven_full_delivery_days_and_supports_arrivals() {
        let (mut w, d) = fixture();
        let revision = finish_development(&mut w, "Delivered model", baseline_spec());
        let id = start_production(&mut w, USA, &revision, &d, 1, 1.0).unwrap();
        let days = w
            .nation(USA)
            .equipment
            .as_ref()
            .unwrap()
            .projects
            .iter()
            .find(|p| p.id == id)
            .unwrap()
            .minimum_days;
        for _ in 0..days - 1 {
            next_work_day(&mut w);
        }
        clock::advance_date(&mut w);
        crate::programs::begin_day(&mut w);
        tick_day(&mut w);
        let completed_day = clock::absolute_day(&w);
        crate::arsenal::tick(&mut w);
        settle_support(&mut w);
        crate::programs::finish_day(&mut w);
        assert_eq!(
            w.nation(USA)
                .arsenal
                .orders
                .iter()
                .find(|o| o.design_id.as_deref() == Some(&revision))
                .unwrap()
                .due_days,
            Some(DELIVERY_DAYS)
        );
        for elapsed in 1..=DELIVERY_DAYS {
            clock::advance_date(&mut w);
            crate::programs::begin_day(&mut w);
            tick_day(&mut w);
            crate::arsenal::tick(&mut w);
            let expenses = w
                .nation(USA)
                .program_budget
                .as_ref()
                .unwrap()
                .spent_today_bn;
            settle_support(&mut w);
            settle_support(&mut w);
            assert_eq!(
                w.nation(USA)
                    .program_budget
                    .as_ref()
                    .unwrap()
                    .spent_today_bn,
                expenses
            );
            let holding = w
                .nation(USA)
                .arsenal
                .held
                .iter()
                .find(|h| h.design_id.as_deref() == Some(&revision));
            if elapsed < DELIVERY_DAYS {
                assert!(holding.is_none());
            } else {
                assert_eq!(holding.unwrap().units, 1.0);
                assert_eq!(
                    clock::absolute_day(&w) - completed_day,
                    DELIVERY_DAYS as i32
                );
                let support = w.nation(USA).equipment.as_ref().unwrap();
                assert!(support.maintenance_required_today_bn > 0.0);
                assert!(support.maintenance_allocated_today_bn > 0.0);
            }
            crate::programs::finish_day(&mut w);
        }
    }
    #[test]
    fn every_vehicle_in_a_batch_consumes_its_recipe_and_false_paid_work_is_rejected() {
        let (mut w, d) = fixture();
        let revision = finish_development(&mut w, "Batch model", baseline_spec());
        let profile = profile(w.nation(USA), &revision).unwrap().clone();
        resources::set_stockpile_for_test(
            &mut w,
            USA,
            Commodity::Copper,
            profile.recipe[Commodity::Copper.idx()] * 1.5,
        );
        let id = start_production(&mut w, USA, &revision, &d, 3, 1.0).unwrap();
        let mut corrupt = w.clone();
        corrupt
            .nation_mut(USA)
            .equipment
            .as_mut()
            .unwrap()
            .projects
            .iter_mut()
            .find(|p| p.id == id)
            .unwrap()
            .resources_used = profile.recipe;
        assert!(validate_state(corrupt.nation(USA))
            .unwrap_err()
            .contains("paid work"));
        for _ in 0..profile.tooling_days + profile.production_days * 2 {
            next_work_day(&mut w);
        }
        let job = w
            .nation(USA)
            .equipment
            .as_ref()
            .unwrap()
            .projects
            .iter()
            .find(|p| p.id == id)
            .unwrap();
        assert_eq!(
            job.completed_units, 1,
            "two later vehicles must not bypass the depleted copper pile"
        );
        assert!(job.work_days < job.minimum_days as f64);
        validate_state(w.nation(USA)).unwrap();
        let mut loaded = crate::load(&crate::save(&w)).unwrap();
        resources::set_stockpile_for_test(&mut w, USA, Commodity::Copper, 10.0);
        resources::set_stockpile_for_test(&mut loaded, USA, Commodity::Copper, 10.0);
        for _ in 0..profile.production_days * 2 {
            next_work_day(&mut w);
            next_work_day(&mut loaded);
        }
        assert_eq!(crate::save(&w), crate::save(&loaded));
        let job = w
            .nation(USA)
            .equipment
            .as_ref()
            .unwrap()
            .projects
            .iter()
            .find(|p| p.id == id)
            .unwrap();
        assert_eq!(job.completed_units, 3);
        assert_eq!(job.spent_bn, job.cost_bn);
        for (used, per_unit) in job.resources_used.iter().zip(profile.recipe) {
            assert!((used - per_unit * 3.0).abs() < 1e-9);
        }
        validate_state(w.nation(USA)).unwrap();
    }
    #[test]
    fn load_validation_refuses_missing_designs_and_invented_reservations() {
        let (mut w, _) = fixture();
        let revision = finish_development(&mut w, "Model A", baseline_spec());
        crate::arsenal::deliver_design(w.nation_mut(USA), &revision, 1, 0.0).unwrap();
        validate_state(w.nation(USA)).unwrap();
        w.nation_mut(USA)
            .arsenal
            .held
            .iter_mut()
            .find(|h| h.design_id.is_some())
            .unwrap()
            .refit_reserved = 1;
        assert!(validate_state(w.nation(USA))
            .unwrap_err()
            .contains("reservations"));
        w.nation_mut(USA)
            .equipment
            .as_mut()
            .unwrap()
            .revisions
            .clear();
        assert!(validate_state(w.nation(USA)).is_err());
    }
}
