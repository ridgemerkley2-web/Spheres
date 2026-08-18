//! Procurement: what a nation has actually bought, and when it turns up.
//!
//! # Why this is not HOI4's factory queue
//!
//! BIBLE §5 refused a production minigame, and was amended on 2026-08-18 to
//! allow one. The argument it was refused on is still correct and this module
//! has to answer it rather than ignore it: modern platforms take ten to
//! twenty-five years from requirement to fielding and then serve for forty, so
//! **you cannot out-produce a war**. In 1936 you can, which is why HOI4's loop
//! works there and would be a lie here.
//!
//! The answer is **lead time**. Nothing ordered this month helps this decade.
//! An order placed in 1990 for a stealth fighter lands in the 2000s, and the
//! arsenal a government fights with is mostly an inheritance from governments
//! that are gone. That inverts the HOI4 loop: the interesting decision is not
//! "what do I build now that war has come", it is "what will I be able to field
//! in fifteen years, and am I willing to pay for it while nothing is happening".
//! A player who lets procurement lapse for a decade should find out in the one
//! month it matters and be unable to fix it.
//!
//! Formations are what a theatre is given. Nothing here is moved between
//! districts — the province-level tactical map is still refused (§5).

use crate::nations::NationId;
use crate::world::{Nation, WorldState};
use serde::{Deserialize, Serialize};

/// What kind of thing it is. Classes exist so an arsenal can be lopsided in a
/// way that matters: an air force is not a substitute for armour.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Class {
    Armour,
    Air,
    Naval,
    Infantry,
    Missile,
    Space,
}

impl Class {
    pub fn name(self) -> &'static str {
        match self {
            Class::Armour => "Armour",
            Class::Air => "Air",
            Class::Naval => "Naval",
            Class::Infantry => "Infantry",
            Class::Missile => "Missile",
            Class::Space => "Space",
        }
    }
    pub fn parse(s: &str) -> Option<Class> {
        [Class::Armour, Class::Air, Class::Naval, Class::Infantry, Class::Missile, Class::Space]
            .into_iter()
            .find(|c| c.name().eq_ignore_ascii_case(s))
    }
}

/// A named piece of kit, permitted by a technology.
///
/// Real designations, per BIBLE §3.1: "Every derived quantity still needs a real
/// name on it." A player should recognise what their money bought.
#[derive(Clone, Debug)]
pub struct EquipmentDef {
    pub id: &'static str,
    /// The designation as it is actually written.
    pub name: &'static str,
    pub class: Class,
    /// The technology that permits it. Nothing can be ordered before it.
    pub tech: &'static str,
    /// Strength per unit held, before age is taken off it.
    pub quality: f64,
    /// $bn per unit, which is what makes an arsenal a budget question.
    pub unit_cost: f64,
    /// Requirement to delivery. The whole point of the module.
    pub lead_months: u32,
    /// Months in service before it starts losing value.
    pub service_months: u32,
}

// Eight, because a piece of kit is eight things. The same call already made for
// the roster row constructor in nations.rs.
#[allow(clippy::too_many_arguments)]
const fn kit(
    id: &'static str,
    name: &'static str,
    class: Class,
    tech: &'static str,
    quality: f64,
    unit_cost: f64,
    lead_months: u32,
    service_months: u32,
) -> EquipmentDef {
    EquipmentDef { id, name, class, tech, quality, unit_cost, lead_months, service_months }
}

/// Everything that can be fielded, in a fixed order.
///
/// Lead times are the real ones as far as they are knowable: a guided bomb is
/// months, a fighter programme is a decade and a half, a nuclear submarine is
/// most of a decade. Service lives are long on purpose — a B-52 airframe has
/// outlived the country it was built to bomb.
pub const DECK: &[EquipmentDef] = &[
    // ---- Air ----
    kit("f15e", "F-15E Strike Eagle", Class::Air, "aero_pulse_doppler_radar", 2.4, 0.09, 60, 480),
    kit("f117", "F-117 Nighthawk", Class::Air, "aero_stealth_shaping", 2.8, 0.11, 96, 360),
    kit("e3", "E-3 Sentry AWACS", Class::Air, "aero_airborne_battle_management", 3.6, 0.28, 84, 540),
    kit("b2", "B-2 Spirit", Class::Air, "aero_flying_wing_stealth", 6.0, 1.10, 144, 540),
    kit("predator", "RQ-1 Predator", Class::Air, "aero_unmanned_aircraft", 0.7, 0.006, 36, 240),
    kit("mq1b", "MQ-1B Predator", Class::Air, "aero_armed_uav", 1.1, 0.009, 36, 240),
    kit("f22", "F-22 Raptor", Class::Air, "aero_stealth_air_superiority", 5.2, 0.20, 180, 480),
    kit("ea18g", "EA-18G Growler", Class::Air, "aero_electronic_attack", 3.1, 0.10, 84, 420),
    kit("rq170", "RQ-170 Sentinel", Class::Air, "aero_stealth_uav", 1.9, 0.03, 72, 240),
    kit("f35a", "F-35A Lightning II", Class::Air, "aero_stealth_multirole", 4.6, 0.09, 168, 480),
    kit("cca", "Collaborative Combat Aircraft", Class::Air, "aero_collaborative_combat_aircraft", 2.2, 0.03, 96, 300),
    kit("sixthgen", "Sixth-Generation Fighter", Class::Air, "aero_sixth_gen_air_dominance", 7.5, 0.30, 216, 540),
    kit("aesa", "AESA Radar Refit", Class::Air, "aero_aesa_radar", 1.3, 0.02, 48, 300),
    // ---- Naval ----
    kit("la_ssn", "Los Angeles-class SSN", Class::Naval, "aero_quiet_submarine", 5.4, 0.90, 108, 600),
    kit("aip_ssk", "Air-Independent Propulsion Submarine", Class::Naval, "aero_air_independent_submarine", 3.9, 0.50, 96, 480),
    kit("laws", "Shipboard Directed-Energy Mount", Class::Naval, "aero_directed_energy_laser", 2.0, 0.06, 72, 300),
    // ---- Missile ----
    kit("paveway", "GBU-24 Paveway III", Class::Missile, "aero_precision_munitions", 0.35, 0.0004, 18, 300),
    kit("tomahawk", "BGM-109 Tomahawk", Class::Missile, "aero_cruise_missile", 1.2, 0.0016, 36, 360),
    kit("patriot", "MIM-104 Patriot", Class::Missile, "aero_theater_missile_defense", 2.1, 0.02, 60, 420),
    kit("jdam", "JDAM Guidance Kit", Class::Missile, "aero_gps_guided_munition", 0.5, 0.00003, 24, 300),
    kit("gbi", "Ground-Based Interceptor", Class::Missile, "aero_midcourse_defense", 3.4, 0.07, 120, 480),
    kit("x51", "Scramjet Test Vehicle", Class::Missile, "aero_scramjet_propulsion", 1.6, 0.05, 132, 240),
    kit("hgv", "Hypersonic Glide Vehicle", Class::Missile, "aero_hypersonic_glide_vehicle", 4.1, 0.09, 144, 300),
    kit("owa", "One-Way Attack Drone", Class::Missile, "aero_attritable_strike_drone", 0.3, 0.00005, 12, 120),
    // ---- Armour & Infantry ----
    kit("trophy", "Trophy Active Protection", Class::Armour, "aero_active_protection_system", 1.4, 0.004, 48, 300),
    kit("raven", "RQ-11 Raven", Class::Infantry, "aero_small_uas", 0.2, 0.0001, 18, 120),
    kit("switchblade", "Switchblade Loitering Munition", Class::Infantry, "aero_loitering_munition", 0.4, 0.00008, 24, 144),
    kit("cuas", "Counter-UAS Battery", Class::Infantry, "aero_counter_uas_layered", 1.0, 0.005, 36, 240),
    kit("link16", "Tactical Data Link Fit", Class::Infantry, "aero_tactical_datalink", 1.5, 0.008, 48, 360),
    kit("c4isr", "Networked C4ISR", Class::Infantry, "aero_network_centric_warfare", 2.6, 0.03, 72, 360),
    kit("atr", "Autonomous Target Recognition", Class::Infantry, "aero_autonomous_targeting", 2.3, 0.02, 84, 300),
    // ---- Space ----
    kit("kh11", "Electro-Optical Reconnaissance Satellite", Class::Space, "aero_recon_satellite", 3.2, 0.35, 96, 300),
];

/// Serialize a `DECK` index as the kit's stable id.
///
/// A straight copy of `tech::known_serde`, and for the identical reason it
/// exists: an index is a fact about the order of a table in one build, not about
/// the world. Insert one entry into `DECK` and every save on disk would
/// reinterpret its JDAMs as interceptors, with no error and no version bump.
/// `Command::EnactStratagem` already carries "the stratagem's stable id, never
/// an index into the deck" — same rule, same reason.
mod kit_serde {
    use super::DECK;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(v: &u16, s: S) -> Result<S::Ok, S::Error> {
        match DECK.get(*v as usize) {
            Some(d) => d.id.serialize(s),
            None => "".serialize(s),
        }
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<u16, D::Error> {
        let id = String::deserialize(d)?;
        super::index_of(&id).ok_or_else(|| serde::de::Error::custom(format!("unknown kit {id}")))
    }
}

/// What a nation holds of one thing, and how long it has held it.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Holding {
    /// Index into `DECK`, written to disk as a stable id.
    #[serde(with = "kit_serde")]
    pub kit: u16,
    pub units: f64,
    /// Months since delivery, units-weighted across everything merged into this
    /// row. Fractional because a fleet bought over twenty years has a mean age,
    /// and `condition` is linear in it, so the mean gives exactly the right
    /// answer rather than an approximation.
    pub age: f64,
}

/// An order placed and not yet delivered.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Order {
    #[serde(with = "kit_serde")]
    pub kit: u16,
    pub units: f64,
    /// Months still to run. This is the whole mechanism.
    pub due: u32,
}

/// Everything a nation has bought, has on order, and is still flying.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Arsenal {
    pub held: Vec<Holding>,
    pub orders: Vec<Order>,
    /// What the player has told procurement to buy, by `DECK` index. Empty means
    /// the staff choose, which is what they do for every AI nation.
    #[serde(default)]
    pub preference: Option<String>,
    /// Money charged to the economy that had nothing to buy yet. Capped in
    /// `tick` so it cannot be hoarded for a century.
    #[serde(default)]
    pub banked: f64,
}

/// The share of a military budget that buys equipment rather than paying people
/// and running what is already owned.
///
/// NATO's rule of thumb is that a serious force spends about a fifth of its
/// budget on new kit; below roughly 15% an arsenal is ageing faster than it is
/// being replaced, which is the state most of the world is in.
pub const PROCUREMENT_SHARE: f64 = 0.20;

/// Value left in a platform once it is past its service life. Nothing goes to
/// zero — a forty-year-old airframe still flies, it just is not what it was.
const RESIDUAL: f64 = 0.35;

pub fn registry() -> &'static [EquipmentDef] {
    DECK
}

pub fn index_of(id: &str) -> Option<u16> {
    DECK.iter().position(|k| k.id == id).map(|i| i as u16)
}

/// How much of its original value a platform still carries at this age.
///
/// Straight-line from new to `RESIDUAL` across the service life, rather than
/// flat-then-declining. The flat version defeated the entire module: service
/// lives run 240 to 600 months, so nothing bought in 1990 lost a penny of value
/// before 2010 and nothing reached residual before 2060. A government could stop
/// buying in 1995, spend nothing for thirty years, and still hold 100% of what
/// it had — which is the exact failure this module was amended into the bible to
/// model, left unimplemented.
///
/// Linear decay means a fleet is worth about half its purchase value halfway
/// through its life, which is both closer to the truth and, more importantly,
/// something a player feels within a decade of neglecting it.
pub fn condition(def: &EquipmentDef, age: f64) -> f64 {
    let life = def.service_months.max(1) as f64;
    (1.0 - (age / life) * (1.0 - RESIDUAL)).clamp(RESIDUAL, 1.0)
}

/// The strength a nation's arsenal is worth, held equipment only.
///
/// NOT YET READ BY `war.rs`, and the audit was right to call the previous
/// version of this sentence a lie. Wiring it in is blocked on seeding: every
/// nation currently starts 1990 with an empty arsenal, so this returns 0 for all
/// 137 and any integration would delete every army in the world in January 1990.
///
/// It is also probably the wrong quantity. `quality` is authored judgement and
/// five independent reviews found it wrong by between 5x and 10,000x in opposite
/// directions; `unit_cost` is transcribed money under iron rule 4. See
/// PROCUREMENT.md, which argues the war model should read book value against
/// what a properly-funded force would hold, and never this.
pub fn strength_of(n: &Nation) -> f64 {
    n.arsenal
        .held
        .iter()
        .map(|h| {
            match DECK.get(h.kit as usize) {
                Some(def) => h.units * def.quality * condition(def, h.age),
                None => 0.0,
            }
        })
        .sum()
}

/// What this nation may order today: everything whose technology it holds.
pub fn available(n: &Nation) -> Vec<u16> {
    DECK.iter()
        .enumerate()
        .filter(|(_, k)| {
            crate::tech::index_of(k.tech).is_some_and(|t| n.tech.knows_index(t))
        })
        .map(|(i, _)| i as u16)
        .collect()
}

/// Procurement, once a month.
///
/// Money becomes orders, orders become holdings when their lead time runs out,
/// and everything already held gets a month older. Nothing here rolls dice: a
/// procurement programme is a budget line, not a gamble.
pub fn tick(w: &mut WorldState) {
    let ids: Vec<NationId> = w.nations.iter().filter(|n| n.alive).map(|n| n.id).collect();
    for id in ids {
        let budget = {
            let n = w.nation(id);
            // Clamped rather than floored: `.max(0.0)` handles a negative GDP
            // (which this tree has produced — see ff77690) and a NaN, because
            // f64::max returns the non-NaN operand. It does not handle +inf,
            // which would place an infinite order and field an infinite arsenal.
            (n.gdp * n.mil_spend_gdp * PROCUREMENT_SHARE / 12.0).clamp(0.0, 1e6)
        };
        let choice = pick(w.nation(id));
        let n = w.nation_mut(id);

        // Age what is already in service.
        for h in n.arsenal.held.iter_mut() {
            h.age += 1.0;
        }

        // Deliveries. An order that has run its lead time becomes a holding.
        let mut arrived: Vec<(u16, f64)> = vec![];
        for o in n.arsenal.orders.iter_mut() {
            o.due = o.due.saturating_sub(1);
            if o.due == 0 {
                arrived.push((o.kit, o.units));
            }
        }
        n.arsenal.orders.retain(|o| o.due > 0);
        for (kit, units) in arrived {
            // One row per kit, merged with a units-weighted mean age. The old
            // `age < 12` predicate opened a new row every year, so a seventy-year
            // run carried seventy rows per kit and `units` only ever rose.
            match n.arsenal.held.iter_mut().find(|h| h.kit == kit) {
                Some(h) => {
                    let total = h.units + units;
                    if total > 0.0 {
                        h.age *= h.units / total;
                    }
                    h.units = total;
                }
                None => n.arsenal.held.push(Holding { kit, units, age: 0.0 }),
            }
        }

        // ...and this month's money goes onto the order book.
        // Money with nothing to buy is banked rather than evaporating. The
        // economy has already been charged for it in economy.rs, so losing it
        // here was years of funding that produced nothing, with no record.
        // Capped at two years of the line so it cannot be hoarded for a century.
        let line = budget + n.arsenal.banked;
        n.arsenal.banked = 0.0;
        if choice.is_none() {
            n.arsenal.banked = line.min(budget * 24.0);
        }
        if let Some(kit) = choice {
            let def = &DECK[kit as usize];
            let units = line / def.unit_cost.max(1e-9);
            if units > 0.0 {
                match n.arsenal.orders.iter_mut().find(|o| o.kit == kit && o.due == def.lead_months) {
                    Some(o) => o.units += units,
                    None => n.arsenal.orders.push(Order { kit, units, due: def.lead_months }),
                }
            }
        }

        // Written off once it is past twice its service life, about 22% a year.
        // Without this `units` only ever rose and `retain` below could never
        // fire, so an arsenal was monotonically non-decreasing for the whole game.
        for h in n.arsenal.held.iter_mut() {
            if let Some(def) = DECK.get(h.kit as usize) {
                if h.age > 2.0 * def.service_months as f64 {
                    h.units *= 0.98;
                }
            }
        }
        n.arsenal.held.retain(|h| h.units > 1e-6);
    }
}

/// What procurement buys this month.
///
/// The player's standing preference if they have set one and can still build it;
/// otherwise the best quality per pound available, which is what a staff with no
/// political direction does. Deterministic — `DECK` order breaks every tie.
fn pick(n: &Nation) -> Option<u16> {
    let open = available(n);
    if open.is_empty() {
        return None;
    }
    if let Some(p) = n.arsenal.preference.as_deref().and_then(index_of) {
        if open.contains(&p) {
            return Some(p);
        }
    }
    open.into_iter().max_by(|a, b| {
        let va = DECK[*a as usize].quality / DECK[*a as usize].unit_cost.max(1e-9);
        let vb = DECK[*b as usize].quality / DECK[*b as usize].unit_cost.max(1e-9);
        va.partial_cmp(&vb).unwrap_or(std::cmp::Ordering::Equal).then(b.cmp(a))
    })
}
