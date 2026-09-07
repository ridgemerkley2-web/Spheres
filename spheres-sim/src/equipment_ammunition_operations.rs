/// Pure opening-day requirements for conserved ground deployments. One family's
/// available rounds are apportioned across all theatres before any battle fires.
#[derive(Clone, Debug, Serialize)]
pub struct AmmoFamilyUse {
    pub family: String,
    pub vehicles: f64,
    pub stock: f64,
    pub required: f64,
    pub used: f64,
    pub coverage: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct AmmunitionOverview {
    pub active: bool,
    pub legacy_share: f64,
    pub families: Vec<AmmoFamilyUse>,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct AmmoDeployment {
    pub ground: bool,
    pub deployed_share: f64,
    /// Conserved aircraft allocation, zero outside an accessible rung-6 mission.
    pub aircraft_share: f64,
    pub intensity: f64,
    pub air_exposure: f64,
}

pub(crate) fn has_aviation_holdings(n: &Nation) -> bool {
    n.arsenal.held.iter().any(|h| {
        h.design_id
            .as_deref()
            .and_then(|id| profile(n, id))
            .is_some_and(|p| p.aviation.is_some())
    })
}

/// Ground stores retain their explicit activation. A delivered aircraft always
/// needs physical stores, including while all its airframes are reserved for refit.
pub fn physical_ammunition_required(n: &Nation, day: i32) -> bool {
    ammunition_active(n, day) || has_aviation_holdings(n)
}

#[derive(Default)]
struct AmmunitionReferences {
    legacy: f64,
    ground: f64,
    aviation: f64,
}
impl AmmunitionReferences {
    fn total(&self) -> f64 {
        self.legacy + self.ground + self.aviation
    }
    fn legacy_share(&self) -> f64 {
        if self.total() > 0.0 {
            (self.legacy / self.total()).clamp(0.0, 1.0)
        } else {
            0.0
        }
    }
}

// A depleted or unsupported physical airframe is never repriced into the legacy
// magazine. Ground stock remains on that magazine until its own activation.
fn ammunition_references(n: &Nation, ground_active: bool) -> AmmunitionReferences {
    let mut out = AmmunitionReferences::default();
    for h in &n.arsenal.held {
        if let Some(revision) = h
            .design_id
            .as_deref()
            .and_then(|id| n.equipment.as_ref().and_then(|s| s.revisions.get(id)))
        {
            let value = crate::arsenal::available_design_units(h) as f64
                * revision.profile.reference_weight_bn;
            if revision.profile.aviation.is_some() {
                out.aviation += value;
            } else if ground_active && ammunition_family(&revision.spec).is_some() {
                out.ground += value;
            } else {
                out.legacy += value;
            }
        } else if let Some(def) = crate::arsenal::DECK.get(h.kit as usize) {
            out.legacy += h.units.max(0.0) * def.unit_cost;
        }
    }
    out
}

pub fn ammunition_family_coverage(overview: &AmmunitionOverview, family: &str) -> f64 {
    overview
        .families
        .iter()
        .find(|r| r.family == family)
        .map_or(0.0, |r| r.coverage)
}

/// Composition uses physical reference before maintenance, age or ammunition.
/// A dry custom fleet must never turn into an implicitly supplied legacy fleet.
pub fn ammunition_legacy_reference_share(n: &Nation) -> f64 {
    let mut custom = 0.0;
    let mut legacy = 0.0;
    for h in &n.arsenal.held {
        if let Some(revision) = h
            .design_id
            .as_deref()
            .and_then(|key| n.equipment.as_ref().and_then(|s| s.revisions.get(key)))
        {
            if ammunition_family(&revision.spec).is_some() {
                custom += crate::arsenal::available_design_units(h) as f64
                    * revision.profile.reference_weight_bn;
                continue;
            }
        }
        if let Some(def) = crate::arsenal::DECK.get(h.kit as usize) {
            legacy += h.units.max(0.0) * def.unit_cost;
        }
    }
    if custom + legacy > 0.0 {
        (legacy / (custom + legacy)).clamp(0.0, 1.0)
    } else {
        1.0
    }
}

pub fn ammunition_legacy_refill_share(w: &WorldState, id: NationId) -> f64 {
    if has_aviation_holdings(w.nation(id)) {
        ammunition_references(
            w.nation(id),
            ammunition_active(w.nation(id), clock::absolute_day(w)),
        )
        .legacy_share()
    } else if ammunition_active(w.nation(id), clock::absolute_day(w)) {
        ammunition_legacy_reference_share(w.nation(id))
    } else {
        1.0
    }
}

pub fn ammunition_overview(w: &WorldState, id: NationId) -> AmmunitionOverview {
    crate::operations::ammunition_overview(w, id)
}

pub(crate) fn plan_ammunition(
    w: &WorldState,
    id: NationId,
    deployments: &[AmmoDeployment],
) -> AmmunitionOverview {
    let n = w.nation(id);
    let ground_active = ammunition_active(n, clock::absolute_day(w));
    let active = physical_ammunition_required(n, clock::absolute_day(w));
    let stores = n.equipment.as_ref().and_then(|s| s.ammunition.as_ref());
    let mut families: BTreeMap<String, AmmoFamilyUse> = BTreeMap::new();
    for h in &n.arsenal.held {
        let Some(revision) = h
            .design_id
            .as_deref()
            .and_then(|key| n.equipment.as_ref().and_then(|s| s.revisions.get(key)))
        else {
            continue;
        };
        let Some(family) = ammunition_family(&revision.spec) else {
            continue;
        };
        let Some(def) = ammo_def(family) else {
            continue;
        };
        let vehicles = crate::arsenal::available_design_units(h) as f64;
        if vehicles == 0.0 {
            continue;
        }
        let air_defense = revision.spec.platform == "ground_air_defense";
        let exposure: f64 = if let Some(aviation) = &revision.profile.aviation {
            let supported =
                crate::arsenal::combat_value(n, h) / revision.profile.reference_weight_bn;
            let share = deployments
                .iter()
                .filter(|d| !d.ground)
                .map(|d| d.aircraft_share * d.intensity)
                .sum::<f64>();
            supported / vehicles
                * aviation.sorties_per_aircraft_month
                * aviation.stores_per_sortie
                * share
        } else if ground_active {
            deployments
                .iter()
                .filter(|d| d.ground)
                .map(|d| {
                    d.deployed_share * d.intensity * if air_defense { d.air_exposure } else { 1.0 }
                })
                .sum()
        } else {
            0.0
        };
        let row = families.entry(family.into()).or_insert(AmmoFamilyUse {
            family: family.into(),
            vehicles: 0.0,
            stock: stores
                .and_then(|s| s.stocks.get(family))
                .copied()
                .unwrap_or(0.0),
            required: 0.0,
            used: 0.0,
            coverage: 1.0,
        });
        row.vehicles += vehicles;
        let rate = if revision.profile.aviation.is_some() {
            1.0
        } else {
            def.rounds_per_vehicle_month
        };
        row.required += vehicles * rate * exposure * clock::month_fraction(w);
    }
    if let Some(stores) = stores {
        for (family, stock) in &stores.stocks {
            if *stock > 0.0 {
                families.entry(family.clone()).or_insert(AmmoFamilyUse {
                    family: family.clone(),
                    vehicles: 0.0,
                    stock: *stock,
                    required: 0.0,
                    used: 0.0,
                    coverage: 1.0,
                });
            }
        }
    }
    for row in families.values_mut() {
        row.used = row.required.min(row.stock.max(0.0));
        row.coverage = if row.required > 0.0 {
            (row.used / row.required).clamp(0.0, 1.0)
        } else {
            1.0
        };
    }
    AmmunitionOverview {
        active,
        legacy_share: if has_aviation_holdings(n) {
            ammunition_references(n, ground_active).legacy_share()
        } else {
            ammunition_legacy_reference_share(n)
        },
        families: families.into_values().collect(),
    }
}

/// Final battle contributions, deliberately separate from standing strength and
/// maneuver/observation. Specialist fires are added once, never multiplied by
/// an ammunition-gated base and then gated a second time.
#[derive(Clone, Copy, Debug, Serialize)]
pub struct GroundAmmoEffects {
    pub legacy_share: f64,
    pub fire_fraction: f64,
    pub maneuver_fraction: f64,
    pub attack_coefficient: f64,
    pub fire_support: f64,
    pub air_defense: f64,
    pub physical_dry: bool,
}

pub(crate) fn ammunition_effects(
    w: &WorldState,
    id: NationId,
    deployment: &AmmoDeployment,
    caps: crate::operations::Capabilities,
    overview: &AmmunitionOverview,
) -> GroundAmmoEffects {
    let n = w.nation(id);
    if has_aviation_holdings(n) {
        return aviation_ammunition_effects(w, id, deployment, caps, overview);
    }
    let magazine = crate::war::magazine_multiplier(w, id);
    if !deployment.ground {
        return GroundAmmoEffects {
            legacy_share: 1.0,
            fire_fraction: magazine,
            maneuver_fraction: magazine,
            attack_coefficient: caps.strike * magazine,
            fire_support: 0.0,
            air_defense: 0.0,
            physical_dry: false,
        };
    }
    let mut custom_weight = 0.0;
    let mut firing_weight = 0.0;
    let mut support_full = 0.0;
    let mut support_ready = 0.0;
    let mut defense_full = 0.0;
    let mut defense_ready = 0.0;
    let mut offensive_used = 0.0;
    for h in &n.arsenal.held {
        let Some(revision) = h
            .design_id
            .as_deref()
            .and_then(|key| n.equipment.as_ref().and_then(|s| s.revisions.get(key)))
        else {
            continue;
        };
        let Some(family) = ammunition_family(&revision.spec) else {
            continue;
        };
        let physical =
            crate::arsenal::available_design_units(h) as f64 * revision.profile.reference_weight_bn;
        if physical <= 0.0 {
            continue;
        }
        let coverage = overview
            .families
            .iter()
            .find(|r| r.family == family)
            .map_or(0.0, |r| r.coverage);
        custom_weight += physical;
        // Air-defense weapons intercept only. Their bodies and observation are
        // retained through maneuver_fraction, never as a free offensive gun.
        let anti_air = revision.spec.platform == "ground_air_defense";
        firing_weight += physical * if anti_air { 0.0 } else { coverage };
        if !anti_air {
            offensive_used += physical * coverage;
        }
        if let Some(roles) = revision.profile.ground_roles {
            let service = crate::arsenal::combat_value(n, h);
            support_full += service * roles.fire_support;
            support_ready += service * roles.fire_support * coverage;
            defense_full += service * roles.air_defense;
            defense_ready += service * roles.air_defense * coverage;
        }
    }
    let custom_share = 1.0 - overview.legacy_share;
    let custom_fraction = if custom_weight > 0.0 {
        (firing_weight / custom_weight).clamp(0.0, 1.0)
    } else {
        0.0
    };
    let fire_fraction = overview.legacy_share * magazine + custom_share * custom_fraction;
    let maneuver_fraction = overview.legacy_share * magazine + custom_share;
    let support = caps.ground_roles.fire_support
        * if support_full > 0.0 {
            (support_ready / support_full).clamp(0.0, 1.0)
        } else {
            0.0
        };
    let defense = if deployment.air_exposure > 0.0 {
        caps.ground_roles.air_defense
            * if defense_full > 0.0 {
                (defense_ready / defense_full).clamp(0.0, 1.0)
            } else {
                0.0
            }
    } else {
        0.0
    };
    let base_attack = (caps.land + caps.strike) * 0.5;
    GroundAmmoEffects {
        legacy_share: overview.legacy_share,
        fire_fraction,
        maneuver_fraction,
        attack_coefficient: base_attack * (fire_fraction + support),
        fire_support: support,
        air_defense: defense,
        physical_dry: custom_weight > 0.0 && offensive_used <= 1e-12,
    }
}

// Custom tactical aircraft count toward air composition only while their own
// physical contribution fires. They cannot improve the remaining legacy attack
// or a ground attack merely by making Class::Air a larger inventory column.
fn legacy_strike_composition(n: &Nation) -> f64 {
    let mut total = 0.0;
    let mut legacy_strike = 0.0;
    for h in &n.arsenal.held {
        let weight = crate::arsenal::combat_value(n, h);
        total += weight;
        if h.design_id
            .as_deref()
            .and_then(|id| profile(n, id))
            .is_some_and(|p| p.aviation.is_some())
        {
            continue;
        }
        if crate::arsenal::DECK.get(h.kit as usize).is_some_and(|d| {
            matches!(
                d.class,
                crate::arsenal::Class::Air | crate::arsenal::Class::Missile
            )
        }) {
            legacy_strike += weight;
        }
    }
    0.65 + 0.35 * (legacy_strike / (total * 0.25).max(1e-12)).clamp(0.0, 1.0)
}

fn aviation_ammunition_effects(
    w: &WorldState,
    id: NationId,
    deployment: &AmmoDeployment,
    caps: crate::operations::Capabilities,
    overview: &AmmunitionOverview,
) -> GroundAmmoEffects {
    let n = w.nation(id);
    let ground_active = ammunition_active(n, clock::absolute_day(w));
    let reference = ammunition_references(n, ground_active);
    let total = reference.total().max(1e-12);
    let legacy_share = reference.legacy_share();
    let magazine = crate::war::magazine_multiplier(w, id);
    let legacy = legacy_share * magazine;
    let legacy_strike = legacy_strike_composition(n);
    if !deployment.ground {
        let mut covered = 0.0;
        let mut strike = 0.0;
        if deployment.aircraft_share > 0.0 && deployment.intensity > 0.0 {
            for h in &n.arsenal.held {
                let Some(p) = h.design_id.as_deref().and_then(|id| profile(n, id)) else {
                    continue;
                };
                let Some(aviation) = &p.aviation else {
                    continue;
                };
                let supported = crate::arsenal::combat_value(n, h);
                let available =
                    supported * ammunition_family_coverage(overview, &aviation.store_family);
                covered += available;
                strike += available * aviation.strike_factor;
            }
        }
        let launched = legacy + covered / total;
        return GroundAmmoEffects {
            legacy_share,
            fire_fraction: launched,
            maneuver_fraction: launched,
            attack_coefficient: legacy_strike * legacy + caps.strike * strike / total,
            fire_support: 0.0,
            air_defense: 0.0,
            physical_dry: covered <= 1e-12,
        };
    }
    let mut ground_firing = 0.0;
    let mut ground_reference = 0.0;
    let mut support_full = 0.0;
    let mut support_ready = 0.0;
    let mut defense_full = 0.0;
    let mut defense_ready = 0.0;
    for h in &n.arsenal.held {
        let Some(revision) = h
            .design_id
            .as_deref()
            .and_then(|id| n.equipment.as_ref().and_then(|s| s.revisions.get(id)))
        else {
            continue;
        };
        if revision.profile.aviation.is_some() {
            continue;
        }
        let Some(family) = ammunition_family(&revision.spec) else {
            continue;
        };
        let raw =
            crate::arsenal::available_design_units(h) as f64 * revision.profile.reference_weight_bn;
        ground_reference += raw;
        let coverage = if ground_active {
            ammunition_family_coverage(overview, family)
        } else {
            magazine
        };
        if ground_active && revision.spec.platform != "ground_air_defense" {
            ground_firing += raw * coverage;
        }
        if let Some(roles) = revision.profile.ground_roles {
            let service = crate::arsenal::combat_value(n, h);
            support_full += service * roles.fire_support;
            support_ready += service * roles.fire_support * coverage;
            defense_full += service * roles.air_defense;
            defense_ready += service * roles.air_defense * coverage;
        }
    }
    let fire_fraction = legacy + ground_firing / total;
    let maneuver_fraction = legacy + reference.ground / total;
    let ground_share = (ground_reference / total).clamp(0.0, 1.0);
    let support = caps.ground_roles.fire_support
        * ground_share
        * if support_full > 0.0 {
            (support_ready / support_full).clamp(0.0, 1.0)
        } else {
            0.0
        };
    let defense = if deployment.air_exposure > 0.0 {
        caps.ground_roles.air_defense
            * ground_share
            * if defense_full > 0.0 {
                (defense_ready / defense_full).clamp(0.0, 1.0)
            } else {
                0.0
            }
    } else {
        0.0
    };
    GroundAmmoEffects {
        legacy_share,
        fire_fraction,
        maneuver_fraction,
        attack_coefficient: (caps.land + legacy_strike) * 0.5 * (fire_fraction + support),
        fire_support: support,
        air_defense: defense,
        physical_dry: ground_firing <= 1e-12,
    }
}

pub(crate) fn settle_ammunition(w: &mut WorldState, id: NationId, overview: &AmmunitionOverview) {
    if !overview.active {
        return;
    }
    let day = clock::absolute_day(w);
    let Some(stores) = w
        .nation_mut(id)
        .equipment
        .as_mut()
        .and_then(|s| s.ammunition.as_mut())
    else {
        return;
    };
    if stores
        .last_consumption
        .as_ref()
        .is_some_and(|r| r.day >= day)
    {
        return;
    }
    if overview.families.iter().any(|r| {
        !r.used.is_finite()
            || r.used < 0.0
            || r.used > stores.stocks.get(&r.family).copied().unwrap_or(0.0)
    }) {
        return;
    }
    let mut required = BTreeMap::new();
    let mut used = BTreeMap::new();
    for row in &overview.families {
        if row.required > 0.0 {
            required.insert(row.family.clone(), row.required);
        }
        if row.used > 0.0 {
            *stores.stocks.entry(row.family.clone()).or_default() -= row.used;
            *stores.consumed.entry(row.family.clone()).or_default() += row.used;
            used.insert(row.family.clone(), row.used);
        }
    }
    stores.last_consumption = Some(AmmoConsumptionReceipt {
        day,
        required,
        used,
    });
}

#[cfg(test)]
pub(crate) mod ammunition_operation_tests {
    use super::*;
    use crate::{arsenal, operations, world::GameRules};
    const USA: NationId = NationId::USA;

    pub(crate) fn fixture(platform: &str) -> WorldState {
        let mut w = crate::init::world_1990(GameRules {
            daily_simulation: true,
            military_operations: true,
            manufacturing_system: true,
            production_system: true,
            resource_market: true,
            resource_gates: true,
            ..Default::default()
        });
        w.player = Some(USA);
        crate::programs::set_construction_budget(&mut w, USA, 0.0).unwrap();
        set_maintenance_plan(&mut w, USA, 1.0).unwrap();
        clock::advance_date(&mut w);
        crate::programs::begin_day(&mut w);
        let spec = default_spec(platform);
        let profile = design_preview(&w, USA, &spec).profile.unwrap();
        let day = clock::absolute_day(&w);
        let n = w.nation_mut(USA);
        n.arsenal.held.clear();
        n.arsenal.orders.clear();
        let s = state_mut(n);
        s.revisions.insert(
            "ammo-model".into(),
            DesignRevision {
                id: "ammo-model".into(),
                name: "Ammo model".into(),
                specification_key: specification_key(&spec),
                spec,
                profile,
                created_day: day,
                certified_day: Some(day),
            },
        );
        s.ammunition = Some(AmmunitionState {
            active_from_day: Some(day),
            ..Default::default()
        });
        arsenal::deliver_design(n, "ammo-model", 1000, 0.0).unwrap();
        w.conflicts.push(conflict(&w, 1, NationId::Canada));
        w
    }
    pub(crate) fn conflict(w: &WorldState, id: u32, opponent: NationId) -> crate::world::Conflict {
        use crate::world::{Belligerent, Conflict, Objective};
        Conflict {
            id,
            theatre: crate::war::theatre_between(w, USA, opponent),
            side_a: vec![USA],
            side_b: vec![opponent],
            posture: vec![
                Belligerent::new(USA, 8, Objective::Seize),
                Belligerent::new(opponent, 8, Objective::Hold),
            ],
            control: 0.0,
            months: 0,
            quiet_months: 0,
            frozen_since: None,
            start_year: 1990,
            start_month: 1,
            origin_attacker: USA,
            invasion_declared: true,
            front: Default::default(),
            pockets: vec![],
            aim: None,
        }
    }
    pub(crate) fn stock(w: &mut WorldState, rounds: u32) {
        let family = ammunition_family(
            &w.nation(USA).equipment.as_ref().unwrap().revisions["ammo-model"].spec,
        )
        .unwrap();
        seed_test_ammunition(w, USA, family, rounds);
    }
    fn near(a: f64, b: f64) {
        assert!(
            (a - b).abs() < 1e-9 * (1.0 + a.abs().max(b.abs())),
            "{a} != {b}"
        );
    }

    #[test]
    fn ammunition_compatibility_separates_every_weapon_caliber_and_load() {
        let mut keys = BTreeSet::new();
        for (gun, caliber) in [
            ("gun_90", "90"),
            ("gun_105", "105"),
            ("gun_120", "120"),
            ("gun_125", "125"),
        ] {
            for (load, suffix) in [
                ("ammo_mixed", "mixed"),
                ("ammo_penetrator", "penetrator"),
                ("ammo_support", "support"),
            ] {
                let mut spec = tank_spec("tank_heavy");
                spec.components.insert("armament".into(), gun.into());
                spec.components.insert("ammunition".into(), load.into());
                let expected = format!("tank_{caliber}_{suffix}");
                assert_eq!(ammunition_family(&spec), Some(expected.as_str()));
                keys.insert(expected);
            }
        }
        for (gun, load, expected) in [
            ("ground_gun_25", "ground_ammo_autocannon", "autocannon_25"),
            ("ground_gun_35", "ground_ammo_autocannon", "autocannon_35"),
            ("ground_mg_127", "ground_ammo_ball", "mg_127"),
            ("ground_howitzer_122", "ground_ammo_he", "howitzer_122_he"),
            (
                "ground_howitzer_122",
                "ground_ammo_guided",
                "howitzer_122_guided",
            ),
            ("ground_howitzer_155", "ground_ammo_he", "howitzer_155_he"),
            (
                "ground_howitzer_155",
                "ground_ammo_guided",
                "howitzer_155_guided",
            ),
            ("ground_aa_gun", "ground_ammo_aa", "aa_cannon"),
            ("ground_aa_missiles", "ground_ammo_missiles", "aa_missile"),
        ] {
            let mut spec = default_spec("ground_ifv");
            spec.components.insert("armament".into(), gun.into());
            spec.components.insert("ammunition".into(), load.into());
            assert_eq!(ammunition_family(&spec), Some(expected));
            keys.insert(expected.into());
            spec.components
                .insert("ammunition".into(), "ammo_mixed".into());
            assert_eq!(ammunition_family(&spec), None);
        }
        assert_eq!(keys.len(), 21);
        assert!(keys.iter().all(|key| ammo_def(key).is_some()));
        assert_eq!(ammunition_family(&baseline_spec()), Some("tank_105_mixed"));
    }

    #[test]
    fn ammunition_opening_plan_conserves_shared_stock_across_theatres_and_is_pure() {
        let mut w = fixture("tank_standard");
        w.conflicts.push(conflict(&w, 2, NationId::Mexico));
        stock(&mut w, 1000);
        let before = crate::save(&w);
        let plan = ammunition_overview(&w, USA);
        let family = &plan.families[0];
        assert!(family.required > family.stock);
        near(family.used, 1000.0);
        let view = operations::view(&w, USA);
        assert_eq!(view.deployments.len(), 2);
        assert!(view
            .deployments
            .iter()
            .all(|d| (d.ammunition.unwrap().fire_fraction - family.coverage).abs() < 1e-10));
        let deployed = view.deployments.iter().map(|d| d.deployed).sum::<f64>();
        assert!(
            deployed <= w.nation(USA).mil_strength && deployed > 0.0,
            "all theatres share one national force, including the overseas limit"
        );
        near(
            family.required,
            1000.0 * ammo_def("tank_105_mixed").unwrap().rounds_per_vehicle_month * deployed
                / w.nation(USA).mil_strength
                * clock::month_fraction(&w),
        );
        assert_eq!(crate::save(&w), before);
        let mut reordered = w.clone();
        reordered.conflicts.reverse();
        assert_eq!(
            serde_json::to_string(&ammunition_overview(&w, USA)).unwrap(),
            serde_json::to_string(&ammunition_overview(&reordered, USA)).unwrap()
        );
        operations::Snapshot::new(&w).settle(&mut w);
        let stores = w
            .nation(USA)
            .equipment
            .as_ref()
            .unwrap()
            .ammunition
            .as_ref()
            .unwrap();
        near(stores.stocks["tank_105_mixed"], 0.0);
        near(stores.consumed["tank_105_mixed"], 1000.0);
        let after = crate::save(&w);
        settle_ammunition(&mut w, USA, &plan);
        assert_eq!(
            crate::save(&w),
            after,
            "a repeated settlement cannot debit twice"
        );
        validate_state(w.nation(USA)).unwrap();
    }

    #[test]
    fn ammunition_wrong_family_transit_and_refit_reservations_cannot_supply_firing() {
        let mut w = fixture("tank_standard");
        let spec = tank_spec("tank_light");
        let compiled = design_preview(&w, USA, &spec);
        let day = clock::absolute_day(&w);
        state_mut(w.nation_mut(USA)).revisions.insert(
            "other".into(),
            DesignRevision {
                id: "other".into(),
                name: "Other".into(),
                specification_key: specification_key(&spec),
                spec,
                profile: compiled.profile.unwrap(),
                created_day: day,
                certified_day: Some(day),
            },
        );
        seed_test_ammunition(&mut w, USA, "tank_90_mixed", 1000);
        let row = operations::view(&w, USA).deployments[0].clone();
        near(row.effective_force, 0.0);
        assert_eq!(row.ammunition.unwrap().fire_fraction, 0.0);
        arsenal::queue_design_order(w.nation_mut(USA), "ammo-model", 1000, 7, 0.0).unwrap();
        let wanted = ammunition_overview(&w, USA)
            .families
            .iter()
            .find(|r| r.family == "tank_105_mixed")
            .unwrap()
            .required;
        // Reservation is tested at the physical helper boundary; lifecycle
        // reservation conservation is independently covered by refit tests.
        arsenal::reserve_refit(w.nation_mut(USA), "ammo-model", 250).unwrap();
        let reserved = ammunition_overview(&w, USA);
        let row = reserved
            .families
            .iter()
            .find(|r| r.family == "tank_105_mixed")
            .unwrap();
        near(row.vehicles, 750.0);
        near(row.required, wanted * 0.75);
    }

    #[test]
    fn ammunition_physical_stores_do_not_change_valuation_or_standing_force() {
        let mut w = fixture("ground_artillery");
        let n = w.nation(USA);
        let before = (
            arsenal::book_value(n),
            arsenal::adequacy(n),
            crate::war::sustained_force(n, n.mil_spend_gdp),
            operations::capabilities(n).land,
            n.mil_strength,
        );
        stock(&mut w, 100_000);
        let n = w.nation(USA);
        assert_eq!(
            before,
            (
                arsenal::book_value(n),
                arsenal::adequacy(n),
                crate::war::sustained_force(n, n.mil_spend_gdp),
                operations::capabilities(n).land,
                n.mil_strength
            )
        );
        w.nation_mut(USA)
            .equipment
            .as_mut()
            .unwrap()
            .maintenance_fraction = 0.0;
        assert_eq!(
            ammunition_legacy_reference_share(w.nation(USA)),
            0.0,
            "unsupported custom stock does not become legacy supply"
        );
        near(ammunition_legacy_refill_share(&w, USA), 0.0);
        let scalar = w.nation(USA).munitions;
        operations::Snapshot::new(&w).settle(&mut w);
        near(w.nation(USA).munitions, scalar);
    }

    #[test]
    fn ammunition_mixed_fleet_replaces_scalar_supply_only_for_its_physical_share() {
        let mut w = fixture("tank_standard");
        stock(&mut w, 100_000);
        let mut legacy = crate::init::world_1990(GameRules::default())
            .nation(USA)
            .arsenal
            .held[0]
            .clone();
        let custom_weight = 1000.0
            * profile(w.nation(USA), "ammo-model")
                .unwrap()
                .reference_weight_bn;
        legacy.units = custom_weight / arsenal::DECK[legacy.kit as usize].unit_cost;
        w.nation_mut(USA).arsenal.held.push(legacy);
        w.nation_mut(USA).munitions = 0.01;
        let mut old = w.clone();
        old.nation_mut(USA)
            .equipment
            .as_mut()
            .unwrap()
            .ammunition
            .as_mut()
            .unwrap()
            .active_from_day = None;
        let physical = operations::view(&w, USA).deployments[0].clone();
        let scalar = operations::view(&old, USA).deployments[0].clone();
        near(ammunition_legacy_reference_share(w.nation(USA)), 0.5);
        near(ammunition_legacy_refill_share(&w, USA), 0.5);
        near(physical.burn_monthly, scalar.burn_monthly * 0.5);
        near(
            physical.effective_force / physical.deployed,
            0.5 * crate::war::magazine_multiplier(&w, USA) + 0.5,
        );
        let magazine = w.nation(USA).munitions;
        let expected = (magazine - physical.burn_monthly * clock::month_fraction(&w)).max(0.0);
        operations::Snapshot::new(&w).settle(&mut w);
        near(w.nation(USA).munitions, expected);
    }

    #[test]
    fn ammunition_air_only_offense_uses_no_ground_rounds_and_aa_requires_real_air_exposure() {
        let mut w = fixture("ground_air_defense");
        stock(&mut w, 100_000);
        assert!(ammunition_overview(&w, USA)
            .families
            .iter()
            .all(|r| r.required == 0.0));
        w.conflicts[0].posture_mut(NationId::Canada).unwrap().rung = 6;
        assert!(ammunition_overview(&w, USA)
            .families
            .iter()
            .any(|r| r.required > 0.0));
        assert!(
            operations::view(&w, USA).deployments[0]
                .ammunition
                .unwrap()
                .air_defense
                > 0.0
        );
        w.conflicts[0]
            .posture_mut(NationId::Canada)
            .unwrap()
            .force_share_bp = Some(0);
        assert!(ammunition_overview(&w, USA)
            .families
            .iter()
            .all(|r| r.required == 0.0));
        let mut w = fixture("tank_standard");
        stock(&mut w, 100_000);
        w.conflicts[0].posture_mut(USA).unwrap().rung = 6;
        assert!(ammunition_overview(&w, USA)
            .families
            .iter()
            .all(|r| r.required == 0.0));
        assert_eq!(
            operations::view(&w, USA).deployments[0]
                .ammunition
                .unwrap()
                .legacy_share,
            1.0
        );
    }

    #[test]
    fn ammunition_inactive_mode_preserves_old_readings_and_saved_consumption_continues() {
        let mut w = fixture("tank_standard");
        stock(&mut w, 100_000);
        let day = clock::absolute_day(&w);
        w.nation_mut(USA)
            .equipment
            .as_mut()
            .unwrap()
            .ammunition
            .as_mut()
            .unwrap()
            .active_from_day = Some(day + 1);
        let mut old = w.clone();
        old.nation_mut(USA).equipment.as_mut().unwrap().ammunition = None;
        assert_eq!(
            serde_json::to_string(&operations::view(&w, USA)).unwrap(),
            serde_json::to_string(&operations::view(&old, USA)).unwrap()
        );
        assert_eq!(ammunition_legacy_refill_share(&w, USA), 1.0);
        w.nation_mut(USA)
            .equipment
            .as_mut()
            .unwrap()
            .ammunition
            .as_mut()
            .unwrap()
            .active_from_day = Some(day);
        operations::Snapshot::new(&w).settle(&mut w);
        validate_state(w.nation(USA)).unwrap();
        let mut loaded = crate::load(&crate::save(&w)).unwrap();
        assert_eq!(crate::save(&loaded), crate::save(&w));
        clock::advance_date(&mut w);
        clock::advance_date(&mut loaded);
        crate::programs::begin_day(&mut w);
        crate::programs::begin_day(&mut loaded);
        operations::Snapshot::new(&w).settle(&mut w);
        operations::Snapshot::new(&loaded).settle(&mut loaded);
        assert_eq!(crate::save(&loaded), crate::save(&w));
    }
}
