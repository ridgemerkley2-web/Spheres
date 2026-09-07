// Detailed vehicle specifications. Numeric values are game balance assumptions.
// Kept separate from the original catalogue so old frozen revisions retain their identity.
pub const DESIGN_SLOTS: [&str; 12] = ["mobility", "transmission", "tracks", "suspension", "turret", "armament", "ammunition", "protection", "active_protection", "sensors", "fire_control", "communications"];
pub fn slot_name(slot: &str) -> &str {
    match slot {
        "mobility" => "Engine", "transmission" => "Transmission", "tracks" => "Tracks",
        "suspension" => "Suspension", "turret" => "Turret", "armament" => "Main gun",
        "ammunition" => "Ammunition load", "protection" => "Armor", "active_protection" => "Active protection",
        "sensors" => "Observation optics", "fire_control" => "Fire control", "communications" => "Communications",
        "wheels" => "Wheeled running gear", "troop_compartment" => "Troop compartment",
        "recon_package" => "Scout equipment", "artillery_loader" => "Artillery loading", "radar" => "Air-defense radar",
        "air_engine" => "Aircraft engines", "air_wing" => "Wing and flight controls", "air_radar" => "Attack radar",
        "air_avionics" => "Attack avionics", "air_countermeasures" => "Mission protection", "air_hardpoints" => "Store installations",
        "air_payload" => "Certified strike loadout", "air_fuel" => "Endurance installation", _ => slot,
    }
}
pub const SPEC_COMPONENTS: &[ComponentDef] = &[
    component!("engine_diesel_600","600 hp diesel","mobility",1,0.00022,0.00000006,0.0,0.0,-0.10,0.0,None,None,"Compact, economical engine. Nominal horsepower identifies this game specification; mobility is a relative rating."),
    component!("engine_diesel_900","900 hp diesel","mobility",2,0.00035,0.00000008,0.0,0.0,0.0,0.0,None,None,"General-purpose engine. Transmission, tracks and suspension are chosen separately."),
    component!("engine_diesel_1200","1,200 hp diesel","mobility",3,0.00065,0.00000015,0.0,0.0,0.20,0.0,None,None,"More engine output, installation space and maintenance demand."),
    component!("engine_turbine_1500","1,500 hp gas turbine","mobility",4,0.00095,0.00000025,0.0,0.0,0.30,0.0,Some("tank_powerpack"),None,"Highest mobility rating and support cost. Requires powerpack integration research; unavailable on the light chassis."),
    component!("transmission_manual","Manual gearbox","transmission",1,0.00010,0.00000002,0.0,0.0,-0.03,0.0,None,None,"Lower purchase cost; modest mobility penalty."),
    component!("transmission_auto","Automatic cross-drive","transmission",2,0.00023,0.00000004,0.0,0.0,0.06,0.0,None,None,"Improves mobility but costs more and occupies another installation point."),
    component!("tracks_standard","Standard steel tracks","tracks",1,0.00008,0.00000002,0.0,0.0,0.0,0.0,None,None,"Standard running surface and support cost."),
    component!("tracks_wide","Wide ground-contact tracks","tracks",2,0.00015,0.00000004,0.0,0.0,0.07,0.0,None,None,"Wider running surface improves the game's mobility rating and increases support needs."),
    component!("tracks_padded","Rubber-padded tracks","tracks",1,0.00013,0.000000025,0.0,0.0,0.03,0.0,None,None,"Replaceable pads provide a small mobility benefit at higher cost."),
    component!("suspension_torsion","Torsion-bar suspension","suspension",1,0.00012,0.00000002,0.0,0.0,0.0,0.0,None,None,"Conventional suspension with lower purchase and support costs."),
    component!("suspension_hydro","Hydropneumatic suspension","suspension",2,0.00029,0.00000006,0.02,0.0,0.10,0.0,Some("tank_running_gear"),None,"Improves mobility and firing stability after running-gear integration research."),
    component!("turret_compact","Compact two-person turret","turret",1,0.00018,0.00000002,-0.03,-0.04,0.04,0.0,None,None,"Small and affordable. Supports the 90 mm and 105 mm guns."),
    component!("turret_standard","Three-person turret","turret",2,0.00032,0.000000035,0.0,0.0,0.0,0.0,None,None,"General-purpose crew layout for the 90 mm and 105 mm guns."),
    component!("turret_heavy","Large reinforced turret","turret",3,0.00055,0.000000065,0.03,0.06,-0.05,0.0,None,None,"Supports every gun option, including the larger smoothbores. Too large for the light chassis."),
    component!("turret_autoload","Autoloading turret","turret",3,0.00072,0.00000008,0.08,0.0,0.0,0.0,Some("tank_autoloader"),None,"Supports every gun; higher firepower and maintenance after turret integration research."),
    component!("turret_casemate","Fixed casemate","turret",2,0.00025,0.000000025,0.07,0.05,-0.08,-0.03,None,None,"Fixed gun mounting reserved for the tank destroyer; stronger frontal layout, lower flexibility."),
    component!("gun_90","90 mm rifled gun","armament",2,0.00032,0.00000004,-0.12,0.0,0.05,0.0,None,None,"Light weapon with lower firepower, cost and installation load."),
    component!("gun_105","105 mm rifled gun","armament",3,0.00055,0.00000006,0.0,0.0,0.0,0.0,None,None,"General-purpose gun. Ammunition mix and fire control are separate choices."),
    component!("gun_120","120 mm smoothbore","armament",4,0.00090,0.00000010,0.20,0.0,-0.06,0.0,None,None,"Higher firepower. Requires a large, autoloading or fixed mounting; not compatible with the light chassis."),
    component!("gun_125","125 mm smoothbore","armament",4,0.00102,0.00000012,0.23,0.0,-0.08,0.0,None,None,"Highest gun rating and support burden. Requires an autoloading turret or fixed casemate."),
    component!("ammo_mixed","Mixed-purpose ammunition","ammunition",1,0.00005,0.00000001,0.0,0.0,0.0,0.0,None,None,"Balanced load specification. Sets the compatible store family; physical rounds are manufactured separately under Ammunition."),
    component!("ammo_penetrator","Penetrator-focused ammunition","ammunition",2,0.00013,0.000000035,0.10,0.0,0.0,0.0,None,None,"Higher composite firepower rating, procurement cost and support demand. The current combat system does not resolve individual projectile impacts."),
    component!("ammo_support","Fire-support ammunition","ammunition",1,0.00003,0.000000008,-0.04,0.0,0.0,0.0,None,None,"Cheaper support load with a lower composite firepower rating; no separate anti-infantry damage model is implied."),
    component!("aps_none","No active protection","active_protection",0,0.000001,0.000000001,0.0,0.0,0.0,0.0,None,None,"Basic wiring provision only. Armor is selected independently."),
    component!("aps_soft","Soft-kill countermeasures","active_protection",1,0.00018,0.000000035,0.0,0.08,0.0,0.0,None,None,"Separate warning and obscurant package; a modest protection benefit."),
    component!("aps_hard","Hard-kill active protection","active_protection",2,0.00075,0.00000010,0.0,0.28,-0.03,0.0,None,Some("aero_active_protection_system"),"Active-protection research unlocks this separate installation. Can accompany reinforced armor."),
    component!("optics_day","Daylight observation optics","sensors",1,0.00015,0.00000002,0.0,0.0,0.0,0.0,None,None,"Basic observation. Gun control is specified independently."),
    component!("optics_night","Night-vision observation optics","sensors",2,0.00028,0.00000004,0.0,0.0,0.0,0.12,None,None,"Improved observation with additional installation and upkeep cost."),
    component!("optics_thermal","Thermal panoramic sight","sensors",2,0.00044,0.000000055,0.02,0.0,0.0,0.25,Some("tank_fire_control_1990"),None,"Thermal observation requires vehicle electronics integration; fire-control selection remains independent."),
    component!("fcs_basic","Basic gun control","fire_control",1,0.00008,0.00000001,0.0,0.0,0.0,0.0,None,None,"Basic laying and range input at lower cost."),
    component!("fcs_stabilized","Stabilized gun control","fire_control",2,0.00020,0.00000003,0.06,0.0,0.0,0.0,None,None,"Improves composite firepower through stability; observation optics are a separate choice."),
    component!("fcs_digital","Digital ballistic fire control","fire_control",2,0.00033,0.00000005,0.12,0.0,0.0,0.04,Some("tank_fire_control_1990"),None,"Vehicle electronics integration unlocks digital gun control."),
];

pub fn all_components() -> impl Iterator<Item = &'static ComponentDef> { COMPONENTS.iter().chain(SPEC_COMPONENTS.iter()).chain(GROUND_COMPONENTS.iter()).chain(AVIATION_COMPONENTS.iter()) }
pub fn detailed_spec(spec: &DesignSpec) -> bool {
    !matches!(spec.platform.as_str(), "tank_standard" | "tank_heavy") || spec.components.iter().any(|(slot,id)| !SLOTS.contains(&slot.as_str()) || SPEC_COMPONENTS.iter().chain(GROUND_COMPONENTS.iter()).any(|c|c.id == id))
}
pub fn spec_version(spec: &DesignSpec) -> u32 { if is_aviation_platform(&spec.platform) { 4 } else if is_ground_platform(&spec.platform) { 3 } else if detailed_spec(spec) { 2 } else { 1 } }
pub fn slots_for(spec: &DesignSpec) -> &'static [&'static str] { if is_aviation_platform(&spec.platform) || is_ground_platform(&spec.platform) { platform_slots(&spec.platform) } else if detailed_spec(spec) { &DESIGN_SLOTS } else { &SLOTS } }
pub fn design_component(c: &ComponentDef) -> bool {
    SPEC_COMPONENTS.iter().any(|s| s.id == c.id) || matches!(c.id,"protection_standard"|"protection_heavy"|"comms_radio"|"comms_data")
}
pub fn tank_spec(platform: &str) -> DesignSpec {
    let mut spec = DesignSpec { platform:platform.into(), components:[
        ("mobility","engine_diesel_900"),("transmission","transmission_manual"),("tracks","tracks_standard"),
        ("suspension","suspension_torsion"),("turret","turret_standard"),("armament","gun_105"),
        ("ammunition","ammo_mixed"),("protection","protection_standard"),("active_protection","aps_none"),
        ("sensors","optics_day"),("fire_control","fcs_basic"),("communications","comms_radio")
    ].into_iter().map(|(k,v)|(k.into(),v.into())).collect() };
    let changes:&[(&str,&str)] = match platform {
        "tank_light" => &[("mobility","engine_diesel_600"),("turret","turret_compact"),("armament","gun_90")],
        "tank_heavy" => &[("mobility","engine_diesel_1200"),("turret","turret_heavy"),("armament","gun_120"),("protection","protection_heavy")],
        "tank_destroyer" => &[("turret","turret_casemate"),("armament","gun_120"),("ammunition","ammo_penetrator")], _ => &[],
    };
    for (k,v) in changes {spec.components.insert((*k).into(),(*v).into());} spec
}
pub fn configuration_refusals(spec: &DesignSpec) -> Vec<String> {
    if is_aviation_platform(&spec.platform) { return aviation_configuration_refusals(spec); }
    if is_ground_platform(&spec.platform) { return ground_configuration_refusals(spec); }
    if !detailed_spec(spec) {return vec![];}
    let mut reasons=vec![]; let get=|slot:&str|spec.components.get(slot).map(String::as_str).unwrap_or("");
    for id in spec.components.values() {if component(id).is_some_and(|c|!design_component(c)) {reasons.push("Choose separate specifications instead of a legacy combined package.".into());}}
    let gun=get("armament");let turret=get("turret");
    if spec.platform=="tank_light" && (matches!(gun,"gun_120"|"gun_125") || matches!(turret,"turret_heavy"|"turret_autoload") || get("mobility")=="engine_turbine_1500" || get("protection")=="protection_heavy") {reasons.push("The light chassis cannot carry reinforced armor, a large turret, a 120/125 mm gun or the turbine engine.".into());}
    if (spec.platform=="tank_destroyer") != (turret=="turret_casemate") {reasons.push("Tank destroyers require a fixed casemate; other tank types require a rotating turret.".into());}
    if gun=="gun_120" && !matches!(turret,"turret_heavy"|"turret_autoload"|"turret_casemate") {reasons.push("The 120 mm gun requires a large turret, autoloading turret or fixed casemate.".into());}
    if gun=="gun_125" && !matches!(turret,"turret_autoload"|"turret_casemate") {reasons.push("The 125 mm gun requires an autoloading turret or fixed casemate.".into());}
    reasons
}

pub fn editable_spec(spec: &DesignSpec) -> DesignSpec {
    if detailed_spec(spec) {return spec.clone();}
    let mut next=tank_spec(&spec.platform);
    for (slot,id) in &spec.components {
        let replacement=match id.as_str() {
            "drive_standard"=>"engine_diesel_900", "drive_mobile"=>"engine_diesel_1200",
            "armament_standard"=>"gun_105", "armament_heavy"=>"gun_120",
            "sensors_optical"=>"optics_day", "sensors_integrated"=>"optics_thermal",
            "protection_active"=>"protection_standard", _=>id.as_str(),
        };
        next.components.insert(slot.clone(),replacement.into());
        if id=="armament_heavy" {next.components.insert("turret".into(),"turret_heavy".into());}
        if id=="sensors_integrated" {next.components.insert("fire_control".into(),"fcs_digital".into());}
        if id=="protection_active" {next.components.insert("active_protection".into(),"aps_hard".into());}
    }
    next
}
