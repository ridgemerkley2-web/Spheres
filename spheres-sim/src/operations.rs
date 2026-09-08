//! National-command allocations and a conserved opening battlefield snapshot.
//! Opt-in: legacy calibration continues to use its original equations.
use crate::{arsenal::{self, Class}, theatre, war, world::*};
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, Default, Serialize)]
pub struct Capabilities {
    pub land: f64,
    pub strike: f64,
    pub lift: f64,
    /// Realized specialist bonuses after inventory coverage and support. These
    /// are bounded effects, not the per-vehicle ratings in a frozen profile.
    pub ground_roles: crate::equipment::GroundRoles,
}

/// Composition alone, never the uncalibrated equipment quality column. Existing
/// adequacy already prices total equipment: these bounded 0.65..1 penalties only
/// prevent an arsenal consisting entirely of one class replacing every role.
pub fn capabilities(n: &Nation) -> Capabilities {
    let mut value = [0.0; 6];
    let mut custom_land = 0.0;
    let mut custom_land_effect = 0.0;
    let mut custom_strike_aircraft = 0.0;
    let mut ground = crate::equipment::GroundRoles::default();
    for h in &n.arsenal.held {
        if let Some(d) = arsenal::DECK.get(h.kit as usize) {
            let weight = arsenal::combat_value(n, h);
            value[class_index(d.class)] += weight;
            if let Some(profile) = h.design_id.as_deref().and_then(|id| crate::equipment::profile(n, id)) {
                if profile.aviation.is_some() {
                    // Tactical strike aircraft provide no land hardware or
                    // transport lift. Their installed strike factor is applied
                    // once, with compatible stores, by ammunition_effects.
                    custom_strike_aircraft += weight;
                    continue;
                }
                custom_land += weight;
                custom_land_effect += weight * profile.land_factor.clamp(0.75, 1.25);
                if let Some(r) = profile.ground_roles {
                    ground.fire_support += weight * r.fire_support;
                    ground.protected_mobility += weight * r.protected_mobility;
                    ground.reconnaissance += weight * r.reconnaissance;
                    ground.air_defense += weight * r.air_defense;
                }
            }
        }
    }
    let total: f64 = value.iter().sum();
    let role = |v: f64, share: f64| 0.65 + 0.35 * (v / (total * share).max(1e-12)).clamp(0.0, 1.0);
    // Only the land role receives a tank's frozen component contribution. The
    // reference weight stays unchanged, so a higher price is never more force.
    let land_weight = value[0] + value[3];
    let land_factor = if custom_land > 0.0 && land_weight > 0.0 {
        ((land_weight - custom_land + custom_land_effect) / land_weight).clamp(0.75, 1.25)
    } else { 1.0 };
    // GAME assumptions: a fully covered ground force receives at most +25%
    // support, maneuver or observation, and 35% local air-strike protection.
    // Serviceable physical value supplies coverage, never price or ordered
    // stock. All zeroes are preserved exactly for historical and tank arsenals.
    let coverage = land_weight.max(1e-12);
    ground.fire_support = (ground.fire_support / coverage * 0.20).clamp(0.0,0.25);
    ground.protected_mobility = (ground.protected_mobility / coverage * 0.20).clamp(0.0,0.25);
    ground.reconnaissance = (ground.reconnaissance / coverage * 0.20).clamp(0.0,0.25);
    ground.air_defense = (ground.air_defense / coverage * 0.25).clamp(0.0,0.35);
    Capabilities { land: role(land_weight, 0.40) * land_factor,
        strike: role(value[1] + value[4], 0.25), lift: role((value[1] - custom_strike_aircraft).max(0.0) + value[2], 0.20), ground_roles: ground }
}
fn class_index(c: Class) -> usize {
    match c { Class::Armour => 0, Class::Air => 1, Class::Naval => 2,
        Class::Infantry => 3, Class::Missile => 4, Class::Space => 5 }
}

#[derive(Clone, Debug, Serialize)]
pub struct Deployment {
    pub conflict: u32,
    pub nation: NationId,
    pub overseas: bool,
    pub allocation_bp: Option<u16>,
    pub requested: f64,
    pub deployed: f64,
    pub effective_force: f64,
    pub quality: f64,
    pub capabilities: Capabilities,
    pub rung: u8,
    pub burn_monthly: f64,
    /// Present only after physical custom ammunition has been activated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ammunition: Option<crate::equipment::GroundAmmoEffects>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ForceView {
    pub enabled: bool,
    pub structure: f64,
    pub deployed: f64,
    pub reserve: f64,
    pub overseas_limit: f64,
    pub overseas_deployed: f64,
    pub capabilities: Capabilities,
    pub deployments: Vec<Deployment>,
}

/// Desired deployments are capped first by the shared overseas ceiling, then
/// by total structure. Scaling is proportional and sorted by conflict id.
fn allocate_base(w: &WorldState, id: NationId, extra: Option<&Conflict>) -> Vec<Deployment> {
    let Some(n) = w.nation_opt(id).filter(|n| n.alive) else { return vec![] };
    let cap = capabilities(n);
    let overseas_limit = n.mil_strength.max(0.0) * war::deployable_fraction(w, id) * cap.lift;
    let mut conflicts: Vec<&Conflict> = w.conflicts.iter().collect();
    if let Some(c) = extra { if !conflicts.iter().any(|x| x.id == c.id) { conflicts.push(c); } }
    conflicts.sort_by_key(|c| c.id);
    let mut rows: Vec<_> = conflicts.into_iter().filter_map(|c| {
        if c.side_a.is_empty() || c.side_b.is_empty() { return None; }
        let b = c.posture_of(id)?;
        if c.side_of(id).is_none() { return None; }
        let overseas = !theatre::is_home(w, id, c.theatre);
        let pool = if overseas { overseas_limit } else { n.mil_strength.max(0.0) };
        let access = if theatre::has_access(w, id, c.theatre) { 1.0 } else { 0.25 };
        let requested = (pool * war::RUNG_COMMIT[b.rung.min(9) as usize] * access)
            .min(b.force_share_bp.map_or(f64::INFINITY, |bp| n.mil_strength.max(0.0) * bp.min(10_000) as f64 / 10_000.0));
        Some(Deployment { conflict: c.id, nation: id, overseas, allocation_bp: b.force_share_bp,
            requested, deployed: requested, effective_force: 0.0, quality: war::quality(w, id),
            capabilities: cap, rung: b.rung, burn_monthly: 0.0, ammunition: None })
    }).collect();
    let abroad: f64 = rows.iter().filter(|r| r.overseas).map(|r| r.deployed).sum();
    if abroad > overseas_limit {
        for r in rows.iter_mut().filter(|r| r.overseas) { r.deployed *= overseas_limit / abroad; }
    }
    let total: f64 = rows.iter().map(|r| r.deployed).sum();
    if total > n.mil_strength.max(0.0) {
        for r in &mut rows { r.deployed *= n.mil_strength.max(0.0) / total; }
    }
    for r in &mut rows {
        r.effective_force = r.deployed * war::magazine_multiplier(w, id);
        let b = w.conflict(r.conflict).or_else(|| extra.filter(|c| c.id == r.conflict))
            .and_then(|c| c.posture_of(id));
        let roe = b.map_or(1.0, |b| war::roe_burn(b.roe));
        // Historical burn is a full national rung deployment. Scale by the
        // actual national share, so repeated conflicts cannot duplicate supply.
        let reference = n.mil_strength.max(0.0) * war::RUNG_COMMIT[r.rung.min(9) as usize];
        r.burn_monthly = if reference > 0.0 { war::BURN_BY_RUNG[r.rung.min(9) as usize] * roe * r.deployed / reference } else { 0.0 };
    }
    rows
}

// Build aircraft allocations independently of opposing air exposure. Their
// national compatible-store plan can then establish actual launched raids
// without recursively asking another nation's full ammunition allocation.
fn ammo_deployments_base(w: &WorldState, id: NationId, rows: &[Deployment], extra: Option<&Conflict>) -> Vec<crate::equipment::AmmoDeployment> {
    let strength = w.nation(id).mil_strength.max(0.0);
    rows.iter().map(|row| {
        let conflict = w.conflict(row.conflict).or_else(|| extra.filter(|c| c.id == row.conflict));
        let posture = conflict.and_then(|c| c.posture_of(id));
        let roe = posture.map_or(1.0, |b| war::roe_burn(b.roe));
        let rung = row.rung.min(9) as usize;
        let reference = war::RUNG_COMMIT[rung];
        let intensity = if reference > 0.0 {
            war::BURN_BY_RUNG[rung] / reference / (war::BURN_BY_RUNG[8] / war::RUNG_COMMIT[8]) * roe
        } else { 0.0 };
        crate::equipment::AmmoDeployment {
            ground: row.rung != 6,
            aircraft_share: if row.rung == 6 && strength > 0.0 && conflict.is_some_and(|c| theatre::has_access(w,id,c.theatre)) {
                (row.deployed / strength).clamp(0.0,1.0)
            } else { 0.0 },
            deployed_share: if strength > 0.0 { (row.deployed / strength).clamp(0.0, 1.0) } else { 0.0 },
            intensity, air_exposure: 0.0,
        }
    }).collect()
}

fn launched_enemy_air(w: &WorldState, id: NationId, conflict: u32, extra: Option<&Conflict>) -> f64 {
    let rows = allocate_base(w, id, extra);
    let Some(index) = rows.iter().position(|r| r.conflict == conflict && r.rung == 6) else { return 0.0; };
    let row = &rows[index];
    // Preserve the old legacy-air exposure rule. Only nations with actual
    // custom aircraft need the new supported, based and paid-store check.
    if !crate::equipment::has_aviation_holdings(w.nation(id)) { return row.deployed; }
    let deployments = ammo_deployments_base(w, id, &rows, extra);
    let plan = crate::equipment::plan_ammunition(w, id, &deployments);
    let effects = crate::equipment::ammunition_effects(w, id, &deployments[index], row.capabilities, &plan);
    row.deployed * effects.fire_fraction
}

fn ammo_deployments(w: &WorldState, id: NationId, rows: &[Deployment], extra: Option<&Conflict>) -> Vec<crate::equipment::AmmoDeployment> {
    let mut deployments = ammo_deployments_base(w, id, rows, extra);
    for (row, deployment) in rows.iter().zip(&mut deployments) {
        let conflict = w.conflict(row.conflict).or_else(|| extra.filter(|c| c.id == row.conflict));
        let enemy_air: f64 = conflict.map_or(0.0, |c| {
            let opponents = if c.side_a.contains(&id) { &c.side_b } else { &c.side_a };
            opponents.iter().map(|enemy| launched_enemy_air(w, *enemy, row.conflict, extra)).sum()
        });
        deployment.air_exposure = if row.deployed > 0.0 { (enemy_air / row.deployed).clamp(0.0, 1.0) } else { 0.0 };
    }
    deployments
}

fn allocate(w: &WorldState, id: NationId, extra: Option<&Conflict>) -> Vec<Deployment> {
    let mut rows = allocate_base(w, id, extra);
    if !crate::equipment::physical_ammunition_required(w.nation(id), crate::clock::absolute_day(w)) { return rows; }
    let deployments = ammo_deployments(w, id, &rows, extra);
    let plan = crate::equipment::plan_ammunition(w, id, &deployments);
    for (row, deployment) in rows.iter_mut().zip(&deployments) {
        let effects = crate::equipment::ammunition_effects(w, id, deployment, row.capabilities, &plan);
        row.effective_force = row.deployed * effects.fire_fraction;
        row.burn_monthly *= effects.legacy_share;
        row.ammunition = Some(effects);
    }
    rows
}

pub(crate) fn ammunition_overview(w: &WorldState, id: NationId) -> crate::equipment::AmmunitionOverview {
    let rows = allocate_base(w, id, None);
    crate::equipment::plan_ammunition(w, id, &ammo_deployments(w, id, &rows, None))
}

pub(crate) fn deployment(w: &WorldState, c: &Conflict, id: NationId) -> Option<Deployment> {
    allocate(w, id, Some(c)).into_iter().find(|r| r.conflict == c.id)
}

pub fn committed_force(w: &WorldState, c: &Conflict, id: NationId) -> f64 {
    allocate(w, id, Some(c)).iter().find(|r| r.conflict == c.id).map_or(0.0, |r| r.effective_force)
}
pub fn view(w: &WorldState, id: NationId) -> ForceView {
    let n = w.nation(id);
    let rows = allocate(w, id, None);
    let deployed = rows.iter().map(|r| r.deployed).sum::<f64>();
    let caps = capabilities(n);
    ForceView { enabled: w.rules.military_operations, structure: n.mil_strength, deployed,
        reserve: (n.mil_strength - deployed).max(0.0),
        overseas_limit: n.mil_strength.max(0.0) * war::deployable_fraction(w, id) * caps.lift,
        overseas_deployed: rows.iter().filter(|r| r.overseas).map(|r| r.deployed).sum(),
        capabilities: caps, deployments: rows }
}

pub fn allocation_refusal(w: &WorldState, conflict: u32, nation: NationId, share_bp: Option<u16>) -> Option<String> {
    if !w.rules.military_operations { return Some("Enable conserved military operations first.".into()); }
    if share_bp.is_some_and(|v| v > 10_000) { return Some("Force allocation must be between 0 and 10000 basis points.".into()); }
    if w.conflict(conflict).and_then(|c| c.posture_of(nation)).is_none() {
        return Some("The nation is not a participant in this conflict.".into());
    }
    None
}
pub fn set_allocation(w: &mut WorldState, conflict: u32, nation: NationId, share_bp: Option<u16>) -> Result<(), String> {
    if let Some(reason) = allocation_refusal(w, conflict, nation, share_bp) { return Err(reason); }
    w.conflict_mut(conflict).unwrap().posture_mut(nation).unwrap().force_share_bp = share_bp;
    Ok(())
}

pub(crate) struct Snapshot {
    pub rows: BTreeMap<(u32, NationId), Deployment>,
    strength: BTreeMap<NationId, f64>,
    magazines: BTreeMap<NationId, f64>,
    // Raw deployment loss retains legacy materiel arithmetic; launched force
    // loss may be smaller for nations operating physical tactical aircraft.
    losses: BTreeMap<(NationId, u32), (f64, u8, f64)>,
    ammunition: BTreeMap<NationId, crate::equipment::AmmunitionOverview>,
    aircraft_exposure: BTreeMap<(NationId, u32), BTreeMap<String, f64>>,
}
impl Snapshot {
    pub fn new(w: &WorldState) -> Self {
        let mut s = Self { rows: BTreeMap::new(), strength: BTreeMap::new(), magazines: BTreeMap::new(), losses: BTreeMap::new(), ammunition: BTreeMap::new(), aircraft_exposure: BTreeMap::new() };
        for n in w.nations.iter().filter(|n| n.alive) {
            s.strength.insert(n.id, n.mil_strength.max(0.0));
            let rows = allocate(w, n.id, None);
            if crate::equipment::physical_ammunition_required(n, crate::clock::absolute_day(w)) {
                let deployments = ammo_deployments(w, n.id, &rows, None);
                let plan = crate::equipment::plan_ammunition(w, n.id, &deployments);
                if crate::equipment::has_aviation_holdings(n) {
                    for (row, deployment) in rows.iter().zip(&deployments) {
                        let mut exposure = BTreeMap::new();
                        for h in &n.arsenal.held {
                            let Some(design) = h.design_id.as_deref() else { continue; };
                            let Some(profile) = crate::equipment::profile(n, design) else { continue; };
                            let Some(aviation) = &profile.aviation else { continue; };
                            let reference = arsenal::available_design_units(h) as f64 * profile.reference_weight_bn;
                            let launched = if deployment.aircraft_share > 0.0 && deployment.intensity > 0.0 && reference > 0.0 {
                                arsenal::combat_value(n, h) / reference
                                    * crate::equipment::ammunition_family_coverage(&plan, &aviation.store_family)
                            } else { 0.0 };
                            exposure.insert(design.to_string(), launched.clamp(0.0, 1.0));
                        }
                        s.aircraft_exposure.insert((n.id, row.conflict), exposure);
                    }
                }
                s.ammunition.insert(n.id, plan);
            }
            let burn: f64 = rows.iter().map(|r| r.burn_monthly).sum();
            s.magazines.insert(n.id, (n.munitions - burn * crate::clock::month_fraction(w)).clamp(0.0, 1.0));
            for row in rows { s.rows.insert((row.conflict, n.id), row); }
        }
        s
    }
    pub fn deployed(&self, cid: u32, id: NationId) -> f64 { self.rows.get(&(cid, id)).map_or(0.0, |r| r.deployed) }
    pub fn strength(&self, id: NationId) -> f64 { self.strength.get(&id).copied().unwrap_or(0.0) }
    pub fn dry(&self, cid: u32, id: NationId) -> bool {
        let scalar_dry = self.magazines.get(&id).is_some_and(|m| *m <= 0.0);
        if let Some(effects) = self.rows.get(&(cid,id)).and_then(|r| r.ammunition) {
            return (effects.legacy_share <= 1e-12 || scalar_dry)
                && (effects.legacy_share >= 1.0-1e-12 || effects.physical_dry);
        }
        scalar_dry
    }
    pub fn record_loss(&mut self, cid: u32, id: NationId, fraction: f64) {
        if let Some(r) = self.rows.get(&(cid,id)) {
            let raw = r.deployed * fraction.clamp(0.0,1.0);
            let force = if self.aircraft_exposure.contains_key(&(id,cid)) {
                raw * r.ammunition.map_or(1.0, |a| a.maneuver_fraction)
            } else { raw };
            self.losses.insert((id,cid), (raw, r.rung, force));
        }
    }
    /// Apply each nation's force and inventory debit once after every theatre
    /// has resolved against the same opening deployment and equipment snapshot.
    pub fn settle(self, w: &mut WorldState) {
        for (id, plan) in &self.ammunition { crate::equipment::settle_ammunition(w, *id, plan); }
        for (id, opening) in self.strength {
            let mut loss = 0.0;
            let mut material = [0.0; 6];
            let mut aircraft_material = BTreeMap::<String, f64>::new();
            for ((_, cid), (amount, rung, force)) in self.losses.range((id,0)..=(id,u32::MAX)) {
                loss += force;
                // Half of force casualties represent irrecoverable materiel;
                // the rest are personnel/damage already represented by force
                // regeneration. Space stocks are not destroyed in land combat.
                let weights = match rung {
                    6 => [0.0,1.0,0.0,0.0,1.0,0.0],
                    7 => [0.6,0.5,0.5,1.0,0.3,0.0],
                    8 => [1.0,0.5,0.2,1.0,0.3,0.0],
                    _ => [0.7,0.25,0.0,1.0,0.1,0.0],
                };
                for i in 0..6 { material[i] += amount / opening.max(1e-12) * 0.5 * weights[i]; }
                if *rung == 6 {
                    if let Some(exposure) = self.aircraft_exposure.get(&(id,*cid)) {
                        for (design, launched) in exposure {
                            *aircraft_material.entry(design.clone()).or_default() += amount / opening.max(1e-12) * 0.5 * launched;
                        }
                    }
                }
            }
            let n = w.nation_mut(id);
            n.mil_strength = (opening - loss).max(0.0);
            if let Some(m) = self.magazines.get(&id) { n.munitions = *m; }
            let equipment = n.equipment.as_ref();
            for h in &mut n.arsenal.held {
                if let Some(design) = h.design_id.as_deref().filter(|design| equipment
                    .and_then(|s| s.revisions.get(*design)).is_some_and(|r| r.profile.aviation.is_some())) {
                    // This first aircraft slice models launched strike sorties,
                    // not aircraft destroyed on airfields during a land battle.
                    arsenal::apply_holding_loss(h, aircraft_material.get(design).copied().unwrap_or(0.0));
                    continue;
                }
                if let Some(d) = arsenal::DECK.get(h.kit as usize) { arsenal::apply_holding_loss(h, material[class_index(d.class)]); }
            }
        }
    }
}

#[cfg(test)]
#[path = "operations_aviation_loss_tests.rs"]
mod aviation_loss_tests;
