// Tactical strike aircraft. Prices, sortie rates and effectiveness are game
// assumptions. Basing and finite stores are resolved by military operations.
pub const AVIATION_SLOTS: [&str; 8] = [
    "air_engine",
    "air_wing",
    "air_radar",
    "air_avionics",
    "air_countermeasures",
    "air_hardpoints",
    "air_payload",
    "air_fuel",
];

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AviationProfile {
    pub strike_factor: f64,
    pub sorties_per_aircraft_month: f64,
    pub stores_per_sortie: f64,
    pub store_family: String,
}
impl AviationProfile {
    pub fn valid(&self) -> bool {
        self.strike_factor.is_finite()
            && (0.75..=1.25).contains(&self.strike_factor)
            && self.sorties_per_aircraft_month.is_finite()
            && (6.0..=20.0).contains(&self.sorties_per_aircraft_month)
            && matches!(self.stores_per_sortie, 2.0 | 4.0)
            && matches!(
                self.store_family.as_str(),
                "air_bomb_unguided" | "air_bomb_guided"
            )
    }
}

pub fn is_aviation_platform(platform: &str) -> bool {
    matches!(platform, "air_light_attack" | "air_tactical_strike")
}

pub fn design_base_kit(spec: &DesignSpec) -> &'static str {
    if is_aviation_platform(&spec.platform) {
        "air_gen3"
    } else {
        "arm_gen3"
    }
}

pub const AVIATION_COMPONENTS: &[ComponentDef] = &[
    component!("air_engine_economical","Economical single engine","air_engine",2,0.0014,0.00000065,0.0,0.0,0.0,0.0,None,None,"Lower fabrication and support cost. A tactical-strike airframe loses sustained sortie output with this smaller installation."),
    component!("air_engine_twin","Twin-engine installation","air_engine",4,0.0042,0.0000016,0.0,0.0,0.0,0.0,None,None,"Standard sustained sortie output on the larger tactical-strike airframe; cannot fit the light-attack airframe."),
    component!("air_engine_efficient","Managed efficient engine","air_engine",3,0.0032,0.0000010,0.0,0.0,0.0,0.0,Some("air_propulsion_integration"),None,"Raises sustained sortie output after installation. Research alone changes no aircraft."),
    component!("air_wing_straight","Straight attack wing","air_wing",2,0.00065,0.00000018,0.0,0.0,0.0,0.0,None,None,"Economical attack wing. Reduces sortie output when installed on the larger strike airframe."),
    component!("air_wing_swept","Swept strike wing","air_wing",3,0.0014,0.0000003,0.0,0.0,0.0,0.0,None,None,"Standard tactical-strike wing, available only on the larger airframe."),
    component!("air_wing_stable","Stabilized attack wing","air_wing",3,0.0016,0.00000032,0.0,0.0,0.0,0.0,Some("air_propulsion_integration"),None,"Improves sustained sortie output through integrated flight controls, with extra installation and support demand."),
    component!("air_radar_basic","Basic navigation radar","air_radar",1,0.0005,0.00000015,0.0,0.0,0.0,0.0,None,None,"Baseline navigation and attack observation. No interception or air-superiority capability is created."),
    component!("air_radar_mapping","Ground-mapping radar","air_radar",3,0.0020,0.0000004,0.0,0.0,0.0,0.0,Some("air_mission_systems"),None,"Improves the bounded tactical-strike contribution of a supported aircraft."),
    component!("air_avionics_analog","Analog attack avionics","air_avionics",1,0.00045,0.00000012,0.0,0.0,0.0,0.0,None,None,"Baseline attack controls for unguided stores."),
    component!("air_avionics_digital","Digital attack avionics","air_avionics",2,0.0015,0.00000025,0.0,0.0,0.0,0.0,Some("air_mission_systems"),None,"Improves strike integration and permits the separately selected guided-bomb interface."),
    component!("air_countermeasures_basic","Basic warning and decoys","air_countermeasures",1,0.00035,0.00000012,0.0,0.0,0.0,0.0,None,None,"Baseline mission protection included in supported strike effectiveness."),
    component!("air_countermeasures_ecm","Integrated electronic countermeasures","air_countermeasures",2,0.0013,0.00000035,0.0,0.0,0.0,0.0,Some("air_mission_systems"),None,"Raises bounded strike effectiveness under opposition. Does not grant a separate national protection bonus."),
    component!("air_hardpoints_light","Two-store attack installation","air_hardpoints",2,0.00045,0.00000012,0.0,0.0,0.0,0.0,None,None,"Carries and consumes two compatible physical stores per modeled sortie."),
    component!("air_hardpoints_heavy","Four-store strike installation","air_hardpoints",4,0.0010,0.00000025,0.0,0.0,0.0,0.0,None,None,"Tactical-strike airframes only. Four stores improve attack weight but must all be manufactured and supplied."),
    component!("air_payload_unguided","Unguided-bomb interface","air_payload",1,0.0002,0.00000007,0.0,0.0,0.0,0.0,None,None,"Certifies the model for the unguided-bomb family. The interface includes no bombs; manufacture finite stocks under Ammunition."),
    component!("air_payload_guided","Guided-bomb interface","air_payload",2,0.00085,0.00000018,0.0,0.0,0.0,0.0,Some("air_guided_strike"),None,"Requires digital avionics. Improves supported precision strike while consuming separately manufactured guided bombs."),
    component!("air_fuel_standard","Standard endurance installation","air_fuel",1,0.0003,0.00000013,0.0,0.0,0.0,0.0,None,None,"Baseline sustained sortie output within a theatre with available basing. Operating costs are paid through actual maintenance."),
    component!("air_fuel_extended","Extended endurance installation","air_fuel",2,0.0007,0.0000003,0.0,0.0,0.0,0.0,None,None,"Improves sustained sortie output at extra installation and operating cost. Does not bypass basing consent or create strategic lift."),
];
pub const AIR_COMPONENTS: &[ComponentDef] = AVIATION_COMPONENTS;

pub fn aviation_default_spec(platform: &str) -> DesignSpec {
    let strike = platform == "air_tactical_strike";
    DesignSpec {
        platform: platform.into(),
        components: [
            (
                "air_engine",
                if strike {
                    "air_engine_twin"
                } else {
                    "air_engine_economical"
                },
            ),
            (
                "air_wing",
                if strike {
                    "air_wing_swept"
                } else {
                    "air_wing_straight"
                },
            ),
            ("air_radar", "air_radar_basic"),
            ("air_avionics", "air_avionics_analog"),
            ("air_countermeasures", "air_countermeasures_basic"),
            (
                "air_hardpoints",
                if strike {
                    "air_hardpoints_heavy"
                } else {
                    "air_hardpoints_light"
                },
            ),
            ("air_payload", "air_payload_unguided"),
            ("air_fuel", "air_fuel_standard"),
        ]
        .into_iter()
        .map(|(k, v)| (k.into(), v.into()))
        .collect(),
    }
}

pub fn aviation_component_compatible(platform: &str, c: &ComponentDef) -> bool {
    is_aviation_platform(platform)
        && AVIATION_COMPONENTS.iter().any(|x| x.id == c.id)
        && (platform != "air_light_attack"
            || !matches!(
                c.id,
                "air_engine_twin" | "air_wing_swept" | "air_hardpoints_heavy"
            ))
}

fn aviation_configuration_refusals(spec: &DesignSpec) -> Vec<String> {
    let mut reasons = vec![];
    for c in spec.components.values().filter_map(|id| component(id)) {
        if !aviation_component_compatible(&spec.platform, c) {
            reasons.push(format!("{} does not fit this aircraft family.", c.name));
        }
    }
    if spec
        .components
        .get("air_payload")
        .is_some_and(|v| v == "air_payload_guided")
        && spec
            .components
            .get("air_avionics")
            .is_none_or(|v| v != "air_avionics_digital")
    {
        reasons.push("The guided-bomb interface requires digital attack avionics.".into());
    }
    reasons
}

pub fn compile_aviation_profile(spec: &DesignSpec) -> Option<AviationProfile> {
    if !is_aviation_platform(&spec.platform) {
        return None;
    }
    let get = |slot: &str| spec.components.get(slot).map(String::as_str).unwrap_or("");
    let strike = spec.platform == "air_tactical_strike";
    let mut sorties: f64 = if strike { 10.0 } else { 12.0 };
    sorties *= match get("air_engine") {
        "air_engine_efficient" => 1.20,
        "air_engine_economical" if strike => 0.80,
        _ => 1.0,
    };
    sorties *= match get("air_wing") {
        "air_wing_stable" => 1.10,
        "air_wing_straight" if strike => 0.90,
        _ => 1.0,
    };
    if get("air_fuel") == "air_fuel_extended" {
        sorties *= 1.10;
    }
    let mut factor: f64 = if strike { 1.0 } else { 0.92 };
    if get("air_radar") == "air_radar_mapping" {
        factor += 0.06;
    }
    if get("air_avionics") == "air_avionics_digital" {
        factor += 0.04;
    }
    if get("air_countermeasures") == "air_countermeasures_ecm" {
        factor += 0.04;
    }
    if get("air_hardpoints") == "air_hardpoints_heavy" {
        factor += 0.06;
    }
    let guided = get("air_payload") == "air_payload_guided";
    if guided {
        factor += 0.06;
    }
    // Installed sortie output has one bounded combat consumer. Without this
    // ratio, a better engine would merely consume extra stores at the same
    // attack strength. Operations applies the resulting factor only once.
    factor *= sorties / if strike { 10.0 } else { 12.0 };
    Some(AviationProfile {
        strike_factor: factor.clamp(0.75, 1.25),
        sorties_per_aircraft_month: sorties.clamp(6.0, 20.0),
        stores_per_sortie: if get("air_hardpoints") == "air_hardpoints_heavy" {
            4.0
        } else {
            2.0
        },
        store_family: if guided {
            "air_bomb_guided"
        } else {
            "air_bomb_unguided"
        }
        .into(),
    })
}

fn compile_aviation_model(spec: &DesignSpec) -> Option<CompiledProfile> {
    let p = PLATFORMS
        .iter()
        .find(|p| p.id == spec.platform && is_aviation_platform(p.id))?;
    let selected: Vec<_> = AVIATION_SLOTS
        .iter()
        .map(|slot| {
            spec.components
                .get(*slot)
                .and_then(|id| component(id))
                .filter(|c| c.slot == *slot)
        })
        .collect::<Option<_>>()?;
    let strike = spec.platform == "air_tactical_strike";
    let used = selected.iter().map(|c| c.load).sum::<u32>();
    let cost = p.cost_bn + selected.iter().map(|c| c.cost_bn).sum::<f64>();
    let mut recipe = [0.0; 12];
    recipe[crate::resources::Commodity::Iron.idx()] = if strike { 12.0 } else { 6.0 };
    recipe[crate::resources::Commodity::Coal.idx()] = if strike { 0.020 } else { 0.010 };
    recipe[crate::resources::Commodity::Copper.idx()] =
        0.60 + selected.iter().filter(|c| c.research.is_some()).count() as f64 * 0.20;
    Some(CompiledProfile {
        rules_version: 4,
        unit_cost_bn: cost,
        fabrication_cost_bn: cost,
        development_cost_bn: cost * 24.0,
        development_days: 240 + used * 5,
        tooling_cost_bn: cost * 4.0,
        tooling_days: 45,
        production_days: 40 + used * 2,
        service_months: 360,
        maintenance_bn_day: 0.0000010 + selected.iter().map(|c| c.upkeep_bn_day).sum::<f64>(),
        land: 1.0,
        protection: 1.0,
        mobility: 1.0,
        recon: 1.0,
        land_factor: 1.0,
        reference_weight_bn: if strike { 0.025 } else { 0.012 },
        recipe,
        component_costs: selected
            .iter()
            .map(|c| (c.slot.into(), c.cost_bn))
            .collect(),
        installation_used: used,
        installation_capacity: p.capacity,
        ground_roles: None,
        aviation: compile_aviation_profile(spec),
    })
}

fn aviation_frozen_profile_valid(spec: &DesignSpec, p: &CompiledProfile) -> bool {
    if !is_aviation_platform(&spec.platform) {
        return p.aviation.is_none();
    }
    p.aviation.as_ref().is_some_and(AviationProfile::valid)
        && compile_aviation_model(spec).as_ref() == Some(p)
}

#[cfg(test)]
mod aviation_tests {
    use super::*;
    use crate::{
        resources::{self, Commodity},
        world::NationId,
    };
    const USA: NationId = NationId::USA;

    fn fixture() -> (WorldState, String) {
        super::tests::fixture()
    }
    fn work_day(w: &mut WorldState) {
        clock::advance_date(w);
        if w.nation(USA).program_budget.as_ref().unwrap().fiscal_year != w.year {
            crate::programs::set_construction_budget(w, USA, 0.001).unwrap();
        }
        crate::programs::begin_day(w);
        tick_day(w);
        crate::programs::finish_day(w);
    }
    fn complete_project(w: &mut WorldState, job: u32) {
        for _ in 0..1500 {
            let p = w
                .nation(USA)
                .equipment
                .as_ref()
                .unwrap()
                .projects
                .iter()
                .find(|p| p.id == job)
                .unwrap();
            if p.status == ProjectStatus::Complete {
                return;
            }
            work_day(w);
        }
        panic!(
            "paid aircraft project did not complete: {:?}",
            w.nation(USA)
                .equipment
                .as_ref()
                .unwrap()
                .projects
                .iter()
                .find(|p| p.id == job)
        );
    }
    fn develop(w: &mut WorldState, name: &str, spec: DesignSpec) -> String {
        let revision = start_development(w, USA, name, spec, 1.0).unwrap();
        let job = w
            .nation(USA)
            .equipment
            .as_ref()
            .unwrap()
            .projects
            .last()
            .unwrap()
            .id;
        complete_project(w, job);
        assert!(certified(w.nation(USA), &revision).is_ok());
        revision
    }

    #[test]
    fn aviation_reviews_are_pure_complete_and_do_not_grant_stores() {
        let (mut w, _) = fixture();
        let before = crate::save(&w);
        for platform in ["air_light_attack", "air_tactical_strike"] {
            let spec = default_spec(platform);
            let q = design_preview(&w, USA, &spec);
            assert!(q.valid, "{:?}", q.blockers);
            assert_eq!(spec.components.len(), 8);
            let p = q.profile.unwrap();
            assert_eq!(p.rules_version, 4);
            assert_eq!(p.land_factor, 1.0);
            assert!(p.ground_roles.is_none());
            assert_eq!(
                p.aviation.as_ref().unwrap().store_family,
                "air_bomb_unguided"
            );
            assert!(p.aviation.as_ref().unwrap().valid());
            assert_eq!(design_base_kit(&spec), "air_gen3");
            for slot in AVIATION_SLOTS {
                let mut missing = spec.clone();
                missing.components.remove(slot);
                assert!(!design_preview(&w, USA, &missing).valid, "{slot}");
            }
            assert_eq!(crate::save(&w), before);
        }
        let stocks = resources::stockpile(&w, USA, Commodity::Iron);
        let strength = w.nation(USA).mil_strength;
        save_draft(
            &mut w,
            USA,
            "Future attack aircraft",
            default_spec("air_light_attack"),
        )
        .unwrap();
        assert!(w
            .nation(USA)
            .equipment
            .as_ref()
            .unwrap()
            .ammunition
            .is_none());
        assert!(w
            .nation(USA)
            .arsenal
            .held
            .iter()
            .all(|h| h.design_id.is_none()));
        assert_eq!(resources::stockpile(&w, USA, Commodity::Iron), stocks);
        assert_eq!(w.nation(USA).mil_strength, strength);
    }

    #[test]
    fn aircraft_research_installation_and_payload_pairing_reject_before_spending() {
        let (mut w, _) = fixture();
        let before = crate::save(&w);
        let mut spec = default_spec("air_light_attack");
        spec.components
            .insert("air_hardpoints".into(), "air_hardpoints_heavy".into());
        assert!(!design_preview(&w, USA, &spec).valid);
        assert!(start_development(&mut w, USA, "Overloaded", spec, 1.0).is_err());
        let mut spec = default_spec("air_tactical_strike");
        spec.components
            .insert("air_payload".into(), "air_payload_guided".into());
        let q = design_preview(&w, USA, &spec);
        assert!(q
            .blockers
            .iter()
            .any(|r| r.contains("digital attack avionics")));
        assert!(q.blockers.iter().any(|r| r.contains("Research")));
        assert!(start_development(&mut w, USA, "Unqualified", spec.clone(), 1.0).is_err());
        assert_eq!(crate::save(&w), before);
        let s = state_mut(w.nation_mut(USA));
        s.learned.extend([
            "air_propulsion_integration".into(),
            "air_mission_systems".into(),
            "air_guided_strike".into(),
        ]);
        spec.components
            .insert("air_avionics".into(), "air_avionics_digital".into());
        assert!(design_preview(&w, USA, &spec).valid);
        for (slot, id) in [
            ("air_radar", "air_radar_mapping"),
            ("air_countermeasures", "air_countermeasures_ecm"),
            ("air_fuel", "air_fuel_extended"),
        ] {
            spec.components.insert(slot.into(), id.into());
        }
        let q = design_preview(&w, USA, &spec);
        assert!(!q.valid);
        assert!(q.blockers.iter().any(|r| r.contains("Installation load")));
        assert!(research_prerequisites("air_guided_strike").contains(&"air_mission_systems"));
    }

    #[test]
    fn aircraft_components_change_frozen_air_effects_and_keep_ground_neutral() {
        let (w, _) = fixture();
        let base = default_spec("air_tactical_strike");
        let original = design_preview(&w, USA, &base).profile.unwrap();
        for c in AVIATION_COMPONENTS {
            if base.components.get(c.slot).is_some_and(|id| id == c.id) {
                continue;
            }
            let mut spec = base.clone();
            spec.components.insert(c.slot.into(), c.id.into());
            let p = design_preview(&w, USA, &spec).profile.unwrap();
            assert_ne!(p, original, "{} must have a visible tradeoff", c.id);
            assert_ne!(
                p.aviation, original.aviation,
                "{} must have an operational consumer",
                c.id
            );
            assert_eq!(p.land_factor, 1.0);
            assert!(p.ground_roles.is_none());
            assert_eq!(p.reference_weight_bn, original.reference_weight_bn);
        }
        for (slot, id) in [
            ("air_engine", "air_engine_efficient"),
            ("air_wing", "air_wing_stable"),
            ("air_fuel", "air_fuel_extended"),
        ] {
            let mut spec = base.clone();
            spec.components.insert(slot.into(), id.into());
            let p = compile_aviation_profile(&spec).unwrap();
            let initial = original.aviation.as_ref().unwrap();
            assert!(p.sorties_per_aircraft_month > initial.sorties_per_aircraft_month);
            assert!(
                p.strike_factor > initial.strike_factor,
                "{id} must improve fully supplied attack, not only increase consumption"
            );
        }
    }

    #[test]
    fn aircraft_paid_lifecycle_conserves_fabrication_raw_inputs_and_refits() {
        for platform in ["air_light_attack", "air_tactical_strike"] {
            let (mut w, district) = fixture();
            let spec = default_spec(platform);
            let revision = develop(&mut w, platform, spec.clone());
            let frozen = profile(w.nation(USA), &revision).unwrap().clone();
            let stock: [f64; 12] =
                std::array::from_fn(|i| resources::stockpile(&w, USA, resources::ALL[i]));
            let job = start_production(&mut w, USA, &revision, &district, 2, 1.0).unwrap();
            complete_project(&mut w, job);
            let p = w
                .nation(USA)
                .equipment
                .as_ref()
                .unwrap()
                .projects
                .iter()
                .find(|p| p.id == job)
                .unwrap();
            assert_eq!(p.completed_units, 2);
            assert!((p.spent_bn - p.cost_bn).abs() < 1e-9);
            for i in 0..12 {
                assert!((p.resources_used[i] - 2.0 * frozen.recipe[i]).abs() < 1e-8);
                assert!(
                    (stock[i]
                        - resources::stockpile(&w, USA, resources::ALL[i])
                        - p.resources_used[i])
                        .abs()
                        < 1e-7
                );
            }
            assert_eq!(
                w.nation(USA)
                    .arsenal
                    .orders
                    .iter()
                    .filter(|o| o.design_id.as_deref() == Some(&revision))
                    .map(|o| o.units)
                    .sum::<f64>(),
                2.0
            );
            w.nation_mut(USA).mil_spend_gdp = 0.0;
            w.nation_mut(USA).arsenal.banked = 0.0;
            for _ in 0..=DELIVERY_DAYS {
                work_day(&mut w);
                crate::arsenal::tick(&mut w);
            }
            let held = w
                .nation(USA)
                .arsenal
                .held
                .iter()
                .find(|h| h.design_id.as_deref() == Some(&revision))
                .unwrap();
            assert_eq!(held.units, 2.0);
            assert_eq!(
                crate::arsenal::DECK[held.kit as usize].class,
                crate::arsenal::Class::Air
            );
            assert_eq!(
                fleet_maintenance_requirement(w.nation(USA)),
                2.0 * frozen.maintenance_bn_day
            );
            assert!(w
                .nation(USA)
                .equipment
                .as_ref()
                .unwrap()
                .ammunition
                .is_none());
            let mut upgrade = spec.clone();
            upgrade
                .components
                .insert("air_fuel".into(), "air_fuel_extended".into());
            let target = develop(&mut w, "Endurance refit", upgrade);
            let job = start_refit(&mut w, USA, &revision, &target, &district, 1, 1.0).unwrap();
            assert_eq!(
                w.nation(USA)
                    .arsenal
                    .held
                    .iter()
                    .find(|h| h.design_id.as_deref() == Some(&revision))
                    .unwrap()
                    .refit_reserved,
                1
            );
            complete_project(&mut w, job);
            assert_eq!(
                w.nation(USA)
                    .arsenal
                    .held
                    .iter()
                    .filter(|h| h.design_id.is_some())
                    .map(|h| h.units)
                    .sum::<f64>(),
                2.0
            );
            assert_eq!(profile(w.nation(USA), &revision).unwrap(), &frozen);
            validate_state(w.nation(USA)).unwrap();
            let raw = crate::save(&w);
            assert_eq!(crate::save(&crate::load(&raw).unwrap()), raw);
            let mut bad = w.clone();
            let p = &mut bad
                .nation_mut(USA)
                .equipment
                .as_mut()
                .unwrap()
                .revisions
                .get_mut(&target)
                .unwrap()
                .profile;
            p.aviation.as_mut().unwrap().sorties_per_aircraft_month *= 1.01;
            assert!(validate_state(bad.nation(USA)).is_err());
            let mut bad = w.clone();
            bad.nation_mut(USA)
                .arsenal
                .held
                .iter_mut()
                .find(|h| h.design_id.is_some())
                .unwrap()
                .kit = crate::arsenal::index_of("arm_gen3").unwrap();
            assert!(validate_state(bad.nation(USA)).is_err());
        }
    }

    #[test]
    fn version_seven_ground_library_remains_sparse_and_airframe_changes_are_new_builds() {
        let (mut w, _) = fixture();
        let id = start_development(&mut w, USA, "Old ground model", baseline_spec(), 0.0).unwrap();
        w.nation_mut(USA).equipment.as_mut().unwrap().version = 7;
        let raw = crate::save(&w);
        assert!(!raw.contains("\"aviation\""));
        assert_eq!(crate::save(&crate::load(&raw).unwrap()), raw);
        assert_eq!(profile(w.nation(USA), &id).unwrap().aviation, None);
        let day = clock::absolute_day(&w);
        let make = |id: &str, platform: &str| {
            let spec = default_spec(platform);
            DesignRevision {
                id: id.into(),
                name: id.into(),
                profile: compile_aviation_model(&spec).unwrap(),
                specification_key: specification_key(&spec),
                spec,
                created_day: day,
                certified_day: Some(day),
            }
        };
        assert!(refit_terms(
            &make("light", "air_light_attack"),
            &make("strike", "air_tactical_strike")
        )
        .is_err());
    }
}
