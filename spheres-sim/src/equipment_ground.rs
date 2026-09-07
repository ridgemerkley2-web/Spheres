// Ground-family extensions. Every price, load and relative capability is an
// explicit game assumption, not a claim about a historical vehicle.
pub const GROUND_IFV_SLOTS: [&str; 13] = ["mobility", "transmission", "tracks", "suspension", "turret", "armament", "ammunition", "protection", "active_protection", "sensors", "fire_control", "communications", "troop_compartment"];
pub const GROUND_APC_SLOTS: [&str; 13] = ["mobility", "transmission", "wheels", "suspension", "turret", "armament", "ammunition", "protection", "active_protection", "sensors", "fire_control", "communications", "troop_compartment"];
pub const GROUND_RECON_SLOTS: [&str; 13] = ["mobility", "transmission", "wheels", "suspension", "turret", "armament", "ammunition", "protection", "active_protection", "sensors", "fire_control", "communications", "recon_package"];
pub const GROUND_ARTILLERY_SLOTS: [&str; 13] = ["mobility", "transmission", "tracks", "suspension", "turret", "armament", "ammunition", "protection", "active_protection", "sensors", "fire_control", "communications", "artillery_loader"];
pub const GROUND_AIR_DEFENSE_SLOTS: [&str; 13] = ["mobility", "transmission", "tracks", "suspension", "turret", "armament", "ammunition", "protection", "active_protection", "sensors", "fire_control", "communications", "radar"];

pub fn is_ground_platform(platform: &str) -> bool {
    matches!(platform, "ground_ifv" | "ground_apc" | "ground_recon" | "ground_artillery" | "ground_air_defense")
}
pub fn platform_slots(platform: &str) -> &'static [&'static str] {
    match platform {
        "ground_ifv" => &GROUND_IFV_SLOTS, "ground_apc" => &GROUND_APC_SLOTS,
        "ground_recon" => &GROUND_RECON_SLOTS, "ground_artillery" => &GROUND_ARTILLERY_SLOTS,
        "ground_air_defense" => &GROUND_AIR_DEFENSE_SLOTS,
        "tank_standard" | "tank_heavy" | "tank_light" | "tank_destroyer" => &DESIGN_SLOTS,
        "air_light_attack" | "air_tactical_strike" => &AVIATION_SLOTS,
        _ => &[],
    }
}
pub fn platform_role(platform: &str) -> &'static str {
    match platform {
        "ground_ifv" => "Mechanized infantry support", "ground_apc" => "Protected troop transport",
        "ground_recon" => "Battlefield reconnaissance", "ground_artillery" => "Indirect fire support",
        "ground_air_defense" => "Mobile air defense", "tank_light" => "Light armored combat",
        "tank_heavy" => "Heavy armored assault", "tank_destroyer" => "Anti-armor combat",
        "tank_standard" => "Main battle tank", "air_light_attack" => "Light tactical air strike",
        "air_tactical_strike" => "Tactical air strike", _ => "Unknown platform",
    }
}

pub const GROUND_COMPONENTS: &[ComponentDef] = &[
    component!("ground_engine_750","Managed 750 hp diesel","mobility",2,0.00039,0.000000075,0.0,0.0,0.18,0.0,Some("ground_engine_management"),None,"Electronic management improves mobility without a turbine's support burden."),
    component!("ground_transmission_electric","Electronic automatic gearbox","transmission",2,0.00028,0.00000005,0.0,0.0,0.12,0.0,Some("ground_engine_management"),None,"Managed transmission improves the mobility contribution of a newly installed powertrain."),
    component!("ground_wheels_standard","Six-wheel drive","wheels",1,0.00007,0.00000002,0.0,0.0,0.04,0.0,None,None,"Economical wheeled running gear for carriers and scouts."),
    component!("ground_wheels_runflat","Eight-wheel run-flat drive","wheels",2,0.00018,0.00000004,0.0,0.04,0.12,0.0,Some("ground_wheeled_chassis"),None,"More protected mobility, installation load and maintenance; only wheeled platforms accept it."),
    component!("ground_turret_autocannon","Autocannon turret","turret",2,0.00020,0.000000025,0.0,0.0,0.0,0.0,None,None,"A compact IFV or scout turret for the 25 mm and 35 mm autocannons."),
    component!("ground_station_mg","Protected machine-gun station","turret",1,0.00006,0.00000001,0.0,0.0,0.03,0.0,None,None,"Light mounting for the carrier or scout's 12.7 mm weapon."),
    component!("ground_turret_howitzer","Howitzer turret","turret",3,0.00032,0.00000004,0.0,0.0,-0.04,0.0,None,None,"An enclosed indirect-fire mounting; loading equipment is selected separately."),
    component!("ground_turret_aa","Air-defense weapon mount","turret",2,0.00026,0.000000035,0.0,0.0,0.0,0.0,None,None,"Accepts the mobile gun or missile weapon; radar is a separate installation."),
    component!("ground_gun_25","25 mm autocannon","armament",2,0.00020,0.000000035,-0.12,0.0,0.02,0.0,None,None,"Light direct fire with a mechanized fire-support contribution."),
    component!("ground_gun_35","35 mm autocannon","armament",3,0.00035,0.00000005,0.02,0.0,-0.03,0.0,Some("ground_medium_weapons"),None,"Improves IFV and scout direct fire and their bounded fire-support rating."),
    component!("ground_mg_127","12.7 mm machine gun","armament",1,0.000035,0.00000001,-0.25,0.0,0.03,0.0,None,None,"Carrier self-defense weapon. Does not confer tank firepower or air-defense coverage."),
    component!("ground_howitzer_122","122 mm howitzer","armament",3,0.00030,0.00000004,-0.12,0.0,-0.02,0.0,None,None,"Baseline indirect fire: its support rating improves ground combat, not strategic strike or lift."),
    component!("ground_howitzer_155","155 mm howitzer","armament",4,0.00060,0.000000075,-0.06,0.0,-0.08,0.0,Some("ground_medium_weapons"),None,"Greater indirect-fire support at higher procurement and maintenance cost."),
    component!("ground_aa_gun","Twin air-defense cannon","armament",3,0.00036,0.00000005,-0.18,0.0,-0.02,0.0,None,None,"Local air-defense coverage reduces incoming air-strike damage; does not create aircraft."),
    component!("ground_aa_missiles","Short-range missile launcher","armament",4,0.00075,0.00000008,-0.25,0.0,-0.04,0.0,Some("ground_guided_weapons"),None,"Improved mobile air defense. Requires tracking radar and its matching missile load."),
    component!("ground_ammo_autocannon","Mixed autocannon load","ammunition",1,0.000035,0.000000012,0.0,0.0,0.0,0.0,None,None,"Autocannon load specification. Caliber-matched physical stores are manufactured separately under Ammunition."),
    component!("ground_ammo_ball","Machine-gun ammunition","ammunition",1,0.000012,0.000000006,0.0,0.0,0.0,0.0,None,None,"Self-defense load specification. Compatible 12.7 mm rounds are managed under Ammunition."),
    component!("ground_ammo_he","High-explosive artillery load","ammunition",2,0.00006,0.00000002,0.0,0.0,0.0,0.0,None,None,"Baseline indirect-fire payload for either howitzer."),
    component!("ground_ammo_guided","Guided artillery load","ammunition",3,0.00025,0.00000005,0.04,0.0,0.0,0.04,Some("ground_guided_weapons"),None,"Improves artillery support after weapons and observation integration; requires digital fire control."),
    component!("ground_ammo_aa","Air-defense cannon load","ammunition",2,0.000065,0.000000025,0.0,0.0,0.0,0.0,None,None,"Matched payload for the mobile air-defense cannon."),
    component!("ground_ammo_missiles","Short-range missile load","ammunition",3,0.00030,0.000000055,0.0,0.0,0.0,0.0,Some("ground_guided_weapons"),None,"Matched launcher payload. Compatible missiles are manufactured separately and consumed by physical ammunition operations after activation."),
    component!("ground_armor_light","Light armored hull","protection",1,0.00016,0.000000015,0.0,0.0,0.02,0.0,None,None,"Economical baseline protection for specialist ground vehicles."),
    component!("ground_armor_modular","Modular applique armor","protection",3,0.00040,0.00000004,0.0,0.22,-0.10,0.0,Some("ground_modular_armor"),None,"More protection with a mobility penalty; also improves protected-transport effectiveness."),
    component!("ground_troops_standard","Standard troop compartment","troop_compartment",2,0.00009,0.000000015,0.0,0.0,-0.02,0.0,None,None,"Seats, stowage and egress for a protected infantry element. Contributes to ground maneuver, never overseas lift."),
    component!("ground_troops_protected","Protected troop compartment","troop_compartment",3,0.00022,0.00000003,0.0,0.08,-0.04,0.0,Some("ground_modular_armor"),None,"Improved protected-transport contribution; only IFVs and APCs have this compartment."),
    component!("ground_recon_observer","Scout observation package","recon_package",2,0.00014,0.00000002,0.0,0.0,0.0,0.08,None,None,"Dedicated scouts contribute reconnaissance to the combat exposure gate."),
    component!("ground_recon_mast","Elevated sensor mast","recon_package",3,0.00042,0.000000045,0.0,0.0,-0.02,0.20,Some("ground_sensor_fusion"),None,"Improves the reconnaissance contribution of a new scout model or installed refit."),
    component!("ground_loader_manual","Manual artillery loading","artillery_loader",1,0.00006,0.00000001,0.0,0.0,0.0,0.0,None,None,"Baseline sustained indirect-fire support."),
    component!("ground_loader_assisted","Assisted artillery loading","artillery_loader",3,0.00031,0.000000055,0.04,0.0,-0.03,0.0,Some("ground_artillery_automation"),None,"Raises artillery fire support at additional cost and installation load."),
    component!("ground_radar_search","Search radar","radar",2,0.00032,0.00000004,0.0,0.0,0.0,0.04,None,None,"Baseline target search for mobile air-defense cannon."),
    component!("ground_radar_tracking","Search and tracking radar","radar",3,0.00062,0.00000007,0.0,0.0,0.0,0.12,Some("ground_sensor_fusion"),None,"Improves local air-defense coverage and enables the missile launcher."),
    component!("ground_comms_network","Networked ground command","communications",3,0.00036,0.000000045,0.04,0.0,0.0,0.18,Some("ground_battlefield_network"),None,"Installed coordination raises reconnaissance and specialist support ratings. Learning it alone changes no fielded model."),
    component!("ground_comms_secure","Secure ground radio","communications",2,0.00015,0.000000025,0.0,0.0,0.0,0.06,Some("ground_secure_radios"),None,"Entry ground-communications integration; prerequisite for networked command."),
];

/// This filters individual choices. `configuration_refusals` additionally
/// validates paired weapon/mount/payload choices after all slots are selected.
pub fn component_compatible(platform: &str, c: &ComponentDef) -> bool {
    if is_aviation_platform(platform) { return aviation_component_compatible(platform,c); }
    if !platform_slots(platform).contains(&c.slot) { return false; }
    if !is_ground_platform(platform) {
        if !design_component(c) || GROUND_COMPONENTS.iter().any(|x| x.id == c.id) { return false; }
        return match platform {
            "tank_light" => !matches!(c.id,"gun_120"|"gun_125"|"turret_heavy"|"turret_autoload"|"turret_casemate"|"engine_turbine_1500"|"protection_heavy"),
            "tank_destroyer" => c.slot != "turret" || c.id == "turret_casemate",
            _ => c.id != "turret_casemate",
        };
    }
    match c.slot {
        "mobility" => matches!(c.id,"engine_diesel_600"|"engine_diesel_900"|"ground_engine_750") || (platform == "ground_artillery" && c.id == "engine_diesel_1200"),
        "transmission" => matches!(c.id,"transmission_manual"|"transmission_auto"|"ground_transmission_electric"),
        "tracks" => matches!(c.id,"tracks_standard"|"tracks_wide"|"tracks_padded"),
        "wheels" => matches!(c.id,"ground_wheels_standard"|"ground_wheels_runflat"),
        "suspension" => matches!(c.id,"suspension_torsion"|"suspension_hydro"),
        "turret" => match platform {
            "ground_ifv" => c.id == "ground_turret_autocannon", "ground_apc" => c.id == "ground_station_mg",
            "ground_recon" => matches!(c.id,"ground_station_mg"|"ground_turret_autocannon"),
            "ground_artillery" => c.id == "ground_turret_howitzer", _ => c.id == "ground_turret_aa",
        },
        "armament" => match platform {
            "ground_ifv" => matches!(c.id,"ground_gun_25"|"ground_gun_35"), "ground_apc" => c.id == "ground_mg_127",
            "ground_recon" => matches!(c.id,"ground_mg_127"|"ground_gun_25"|"ground_gun_35"),
            "ground_artillery" => matches!(c.id,"ground_howitzer_122"|"ground_howitzer_155"),
            _ => matches!(c.id,"ground_aa_gun"|"ground_aa_missiles"),
        },
        "ammunition" => match platform {
            "ground_ifv" => c.id == "ground_ammo_autocannon", "ground_apc" => c.id == "ground_ammo_ball",
            "ground_recon" => matches!(c.id,"ground_ammo_ball"|"ground_ammo_autocannon"),
            "ground_artillery" => matches!(c.id,"ground_ammo_he"|"ground_ammo_guided"),
            _ => matches!(c.id,"ground_ammo_aa"|"ground_ammo_missiles"),
        },
        "protection" => matches!(c.id,"ground_armor_light"|"ground_armor_modular"),
        "active_protection" => matches!(c.id,"aps_none"|"aps_soft"|"aps_hard"),
        "sensors" => matches!(c.id,"optics_day"|"optics_night"|"optics_thermal"),
        "fire_control" => matches!(c.id,"fcs_basic"|"fcs_stabilized"|"fcs_digital"),
        "communications" => matches!(c.id,"comms_radio"|"comms_data"|"ground_comms_secure"|"ground_comms_network"),
        "troop_compartment" => matches!(c.id,"ground_troops_standard"|"ground_troops_protected"),
        "recon_package" => matches!(c.id,"ground_recon_observer"|"ground_recon_mast"),
        "artillery_loader" => matches!(c.id,"ground_loader_manual"|"ground_loader_assisted"),
        "radar" => matches!(c.id,"ground_radar_search"|"ground_radar_tracking"), _ => false,
    }
}

pub fn default_spec(platform: &str) -> DesignSpec {
    if is_aviation_platform(platform) { return aviation_default_spec(platform); }
    let mut spec = tank_spec(platform);
    if !is_ground_platform(platform) { return spec; }
    for (slot,id) in [("mobility","engine_diesel_600"),("protection","ground_armor_light")] { spec.components.insert(slot.into(), id.into()); }
    let changes: &[(&str,&str)] = match platform {
        "ground_ifv" => &[("turret","ground_turret_autocannon"),("armament","ground_gun_25"),("ammunition","ground_ammo_autocannon"),("troop_compartment","ground_troops_standard")],
        "ground_apc" => &[("turret","ground_station_mg"),("armament","ground_mg_127"),("ammunition","ground_ammo_ball"),("wheels","ground_wheels_standard"),("troop_compartment","ground_troops_standard")],
        "ground_recon" => &[("turret","ground_station_mg"),("armament","ground_mg_127"),("ammunition","ground_ammo_ball"),("wheels","ground_wheels_standard"),("recon_package","ground_recon_observer")],
        "ground_artillery" => &[("mobility","engine_diesel_900"),("turret","ground_turret_howitzer"),("armament","ground_howitzer_122"),("ammunition","ground_ammo_he"),("artillery_loader","ground_loader_manual")],
        _ => &[("turret","ground_turret_aa"),("armament","ground_aa_gun"),("ammunition","ground_ammo_aa"),("radar","ground_radar_search")],
    };
    if matches!(platform,"ground_apc"|"ground_recon") { spec.components.remove("tracks"); }
    for (slot,id) in changes { spec.components.insert((*slot).into(), (*id).into()); }
    spec
}

fn ground_configuration_refusals(spec: &DesignSpec) -> Vec<String> {
    let mut reasons = vec![];
    let get = |slot:&str| spec.components.get(slot).map(String::as_str).unwrap_or("");
    for id in spec.components.values() {
        if let Some(c) = component(id) { if !component_compatible(&spec.platform,c) { reasons.push(format!("{} is not compatible with this vehicle family.",c.name)); } }
    }
    let gun = get("armament");
    if matches!(gun,"ground_gun_25"|"ground_gun_35") && (get("turret") != "ground_turret_autocannon" || get("ammunition") != "ground_ammo_autocannon") { reasons.push("An autocannon requires its autocannon turret and ammunition load.".into()); }
    if gun == "ground_mg_127" && (get("turret") != "ground_station_mg" || get("ammunition") != "ground_ammo_ball") { reasons.push("The 12.7 mm weapon requires the machine-gun station and matching ammunition.".into()); }
    if gun == "ground_aa_gun" && get("ammunition") != "ground_ammo_aa" { reasons.push("The air-defense cannon requires its cannon ammunition load.".into()); }
    if gun == "ground_aa_missiles" && (get("ammunition") != "ground_ammo_missiles" || get("radar") != "ground_radar_tracking" || get("fire_control") != "fcs_digital") { reasons.push("The missile launcher requires missile ammunition, tracking radar and digital fire control.".into()); }
    if get("ammunition") == "ground_ammo_guided" && get("fire_control") != "fcs_digital" { reasons.push("Guided artillery ammunition requires digital fire control.".into()); }
    reasons
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct GroundRoles {
    pub fire_support: f64,
    pub protected_mobility: f64,
    pub reconnaissance: f64,
    pub air_defense: f64,
}
impl GroundRoles {
    pub fn valid(&self) -> bool { [self.fire_support,self.protected_mobility,self.reconnaissance,self.air_defense].iter().all(|x| x.is_finite() && (0.0..=2.0).contains(x)) }
}

/// Frozen relative role ratings. Operations weights these by serviceable physical
/// inventory, caps the resulting national bonuses, and the war equations consume
/// them. Neither research nor a higher purchase price changes an existing rating.
fn compile_ground_roles(spec: &DesignSpec, land:f64, protection:f64, mobility:f64, recon:f64) -> Option<GroundRoles> {
    if !is_ground_platform(&spec.platform) { return None; }
    let has = |id:&str| spec.components.values().any(|x| x == id);
    let network = if has("ground_comms_network") {1.15} else {1.0};
    let mut r = match spec.platform.as_str() {
        "ground_ifv" => GroundRoles {fire_support:0.35,protected_mobility:0.60,reconnaissance:0.20,air_defense:0.0},
        "ground_apc" => GroundRoles {protected_mobility:1.0,reconnaissance:0.10,..Default::default()},
        "ground_recon" => GroundRoles {fire_support:if has("ground_mg_127") {0.0} else {0.15},reconnaissance:1.0,..Default::default()},
        "ground_artillery" => GroundRoles {fire_support:1.0,reconnaissance:0.10,..Default::default()},
        _ => GroundRoles {reconnaissance:0.10,air_defense:1.0,..Default::default()},
    };
    r.fire_support *= land * network * if has("ground_gun_35") || has("ground_howitzer_155") {1.20} else {1.0}
        * if has("ground_loader_assisted") {1.20} else {1.0} * if has("ground_ammo_guided") {1.20} else {1.0};
    r.protected_mobility *= protection * mobility * if has("ground_troops_protected") {1.25} else {1.0};
    r.reconnaissance *= recon * network * if has("ground_recon_mast") {1.20} else {1.0};
    r.air_defense *= recon * network * if has("ground_radar_tracking") {1.25} else {1.0} * if has("ground_aa_missiles") {1.25} else {1.0};
    r.fire_support = r.fire_support.clamp(0.0,2.0); r.protected_mobility = r.protected_mobility.clamp(0.0,2.0);
    r.reconnaissance = r.reconnaissance.clamp(0.0,2.0); r.air_defense = r.air_defense.clamp(0.0,2.0);
    Some(r)
}

pub fn research_branch(id: &str) -> &'static str {
    match id {
        "tank_running_gear"|"ground_wheeled_chassis" => "chassis",
        "tank_powerpack"|"ground_engine_management" => "engines",
        "tank_autoloader"|"ground_medium_weapons"|"ground_artillery_automation"|"ground_guided_weapons" => "weapons",
        "ground_modular_armor" => "armor", "tank_fire_control_1990"|"ground_sensor_fusion" => "optics",
        "ground_secure_radios"|"ground_battlefield_network" => "communications",
        "air_propulsion_integration" => "engines", "air_mission_systems" => "optics", "air_guided_strike" => "weapons", _ => "unknown",
    }
}
pub fn research_prerequisites(id: &str) -> &'static [&'static str] {
    match id {
        "ground_wheeled_chassis" => &["tank_running_gear"], "ground_engine_management" => &["tank_powerpack"],
        "ground_artillery_automation" => &["tank_autoloader","ground_medium_weapons"],
        "ground_guided_weapons" => &["ground_medium_weapons","tank_fire_control_1990"],
        "ground_sensor_fusion" => &["tank_fire_control_1990"],
        "ground_battlefield_network" => &["ground_secure_radios","ground_sensor_fusion"],
        "air_guided_strike" => &["air_mission_systems"], _ => &[],
    }
}
fn research_requirements(n: &Nation, id: &str) -> Result<(),String> {
    let r = research(id).ok_or_else(|| "Choose a supported equipment research project.".to_string())?;
    if !n.tech.knows(r.prerequisite) { return Err(format!("Research {} before this integration programme.",r.prerequisite)); }
    let missing:Vec<_> = research_prerequisites(id).iter().filter(|key| !n.equipment.as_ref().is_some_and(|s| s.learned.contains(**key))).filter_map(|key|research(key).map(|r|r.name)).collect();
    if !missing.is_empty() { return Err(format!("Complete {} before this integration programme.",missing.join(" and "))); }
    Ok(())
}
pub fn research_available(n: &Nation, id: &str) -> Result<(),String> {
    research_requirements(n,id)?;
    if n.equipment.as_ref().is_some_and(|s| s.learned.contains(id)) { return Err("This integration is already known.".into()); }
    Ok(())
}
