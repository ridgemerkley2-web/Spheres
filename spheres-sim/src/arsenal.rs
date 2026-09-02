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
    /// The technology that permits it, or `None` for the legacy tier — the
    /// generic equipment every state already had in 1990 and can always replace.
    /// Without this nothing in the deck is orderable by anybody at t=0, because
    /// every nation starts knowing no technologies at all, and procurement is
    /// inert in both directions.
    pub tech: Option<&'static str>,
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
    tech: Option<&'static str>,
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
    // ---- The legacy tier: what a state already had in January 1990 ----
    //
    // Generic on purpose. These are not designations, they are what the
    // transcribed strength figure is MADE of, and every nation can always buy
    // more of its own tier because replacing what you have needs no discovery.
    // Costs are per notional formation-equivalent rather than per vehicle, which
    // is the altitude the rest of the sim runs at.
    kit("inf_light", "Light Infantry Formation", Class::Infantry, None, 0.8, 0.010, 24, 300),
    kit("inf_mech", "Mechanised Infantry Formation", Class::Infantry, None, 1.6, 0.035, 36, 300),
    kit("arm_gen2", "Second-Generation Armour", Class::Armour, None, 1.4, 0.030, 36, 360),
    kit("arm_gen3", "Third-Generation Armour", Class::Armour, None, 2.6, 0.075, 48, 420),
    kit("air_gen2", "Second-Generation Combat Aircraft", Class::Air, None, 1.0, 0.020, 48, 360),
    kit("air_gen3", "Third-Generation Combat Aircraft", Class::Air, None, 1.9, 0.045, 60, 420),
    kit("air_gen4", "Fourth-Generation Combat Aircraft", Class::Air, None, 3.4, 0.085, 72, 480),
    kit("nav_patrol", "Patrol and Coastal Craft", Class::Naval, None, 0.6, 0.020, 36, 360),
    kit("nav_escort", "Escort Frigate or Destroyer", Class::Naval, None, 2.2, 0.180, 72, 480),
    kit("nav_blue", "Blue-Water Task Group", Class::Naval, None, 5.0, 0.850, 108, 600),
    kit("msl_sam", "Area Air-Defence System", Class::Missile, None, 1.5, 0.040, 48, 420),
    kit("msl_brm", "Theatre Ballistic Missile", Class::Missile, None, 1.1, 0.025, 48, 360),
    kit("msl_deterrent", "Strategic Deterrent Force", Class::Missile, None, 6.0, 1.400, 120, 600),
    kit("spc_recon", "Reconnaissance Satellite Constellation", Class::Space, None, 3.0, 0.400, 84, 300),
    // ---- Air ----
    kit("f15e", "F-15E Strike Eagle", Class::Air, Some("aero_pulse_doppler_radar"), 2.4, 0.09, 60, 480),
    kit("f117", "F-117 Nighthawk", Class::Air, Some("aero_stealth_shaping"), 2.8, 0.11, 96, 360),
    kit("e3", "E-3 Sentry AWACS", Class::Air, Some("aero_airborne_battle_management"), 3.6, 0.28, 84, 540),
    kit("b2", "B-2 Spirit", Class::Air, Some("aero_flying_wing_stealth"), 6.0, 1.10, 144, 540),
    kit("predator", "RQ-1 Predator", Class::Air, Some("aero_unmanned_aircraft"), 0.7, 0.006, 36, 240),
    kit("mq1b", "MQ-1B Predator", Class::Air, Some("aero_armed_uav"), 1.1, 0.009, 36, 240),
    kit("f22", "F-22 Raptor", Class::Air, Some("aero_stealth_air_superiority"), 5.2, 0.20, 180, 480),
    kit("ea18g", "EA-18G Growler", Class::Air, Some("aero_electronic_attack"), 3.1, 0.10, 84, 420),
    kit("rq170", "RQ-170 Sentinel", Class::Air, Some("aero_stealth_uav"), 1.9, 0.03, 72, 240),
    kit("f35a", "F-35A Lightning II", Class::Air, Some("aero_stealth_multirole"), 4.6, 0.09, 168, 480),
    kit("cca", "Collaborative Combat Aircraft", Class::Air, Some("aero_collaborative_combat_aircraft"), 2.2, 0.03, 96, 300),
    kit("sixthgen", "Sixth-Generation Fighter", Class::Air, Some("aero_sixth_gen_air_dominance"), 7.5, 0.30, 216, 540),
    kit("aesa", "AESA Radar Refit", Class::Air, Some("aero_aesa_radar"), 1.3, 0.02, 48, 300),
    // ---- Naval ----
    kit("la_ssn", "Los Angeles-class SSN", Class::Naval, Some("aero_quiet_submarine"), 5.4, 0.90, 108, 600),
    kit("aip_ssk", "Air-Independent Propulsion Submarine", Class::Naval, Some("aero_air_independent_submarine"), 3.9, 0.50, 96, 480),
    kit("laws", "Shipboard Directed-Energy Mount", Class::Naval, Some("aero_directed_energy_laser"), 2.0, 0.06, 72, 300),
    // ---- Missile ----
    kit("paveway", "GBU-24 Paveway III", Class::Missile, Some("aero_precision_munitions"), 0.35, 0.0004, 18, 300),
    kit("tomahawk", "BGM-109 Tomahawk", Class::Missile, Some("aero_cruise_missile"), 1.2, 0.0016, 36, 360),
    kit("patriot", "MIM-104 Patriot", Class::Missile, Some("aero_theater_missile_defense"), 2.1, 0.02, 60, 420),
    kit("jdam", "JDAM Guidance Kit", Class::Missile, Some("aero_gps_guided_munition"), 0.5, 0.00003, 24, 300),
    kit("gbi", "Ground-Based Interceptor", Class::Missile, Some("aero_midcourse_defense"), 3.4, 0.07, 120, 480),
    kit("x51", "Scramjet Test Vehicle", Class::Missile, Some("aero_scramjet_propulsion"), 1.6, 0.05, 132, 240),
    kit("hgv", "Hypersonic Glide Vehicle", Class::Missile, Some("aero_hypersonic_glide_vehicle"), 4.1, 0.09, 144, 300),
    kit("owa", "One-Way Attack Drone", Class::Missile, Some("aero_attritable_strike_drone"), 0.3, 0.00005, 12, 120),
    // ---- Armour & Infantry ----
    kit("trophy", "Trophy Active Protection", Class::Armour, Some("aero_active_protection_system"), 1.4, 0.004, 48, 300),
    kit("raven", "RQ-11 Raven", Class::Infantry, Some("aero_small_uas"), 0.2, 0.0001, 18, 120),
    kit("switchblade", "Switchblade Loitering Munition", Class::Infantry, Some("aero_loitering_munition"), 0.4, 0.00008, 24, 144),
    kit("cuas", "Counter-UAS Battery", Class::Infantry, Some("aero_counter_uas_layered"), 1.0, 0.005, 36, 240),
    kit("link16", "Tactical Data Link Fit", Class::Infantry, Some("aero_tactical_datalink"), 1.5, 0.008, 48, 360),
    kit("c4isr", "Networked C4ISR", Class::Infantry, Some("aero_network_centric_warfare"), 2.6, 0.03, 72, 360),
    kit("atr", "Autonomous Target Recognition", Class::Infantry, Some("aero_autonomous_targeting"), 2.3, 0.02, 84, 300),
    // ---- Space ----
    kit("kh11", "Electro-Optical Reconnaissance Satellite", Class::Space, Some("aero_recon_satellite"), 3.2, 0.35, 96, 300),
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

/// Months of the reference procurement line a fully-equipped force is worth.
///
/// Derived, not chosen. A steady stream of L $bn a month into kit with service
/// life S settles at a book value of L·S·(1+RESIDUAL)/2 = 0.675·L·S, and at the
/// middling service life in `DECK` that is 200 months of the line.
///
/// It is also the one number here that can be checked against the world, and it
/// checks out: at the transcribed 1990 figures it makes a fully-equipped United
/// States worth about $1.1tn of equipment, against a BEA gross stock of
/// national-defence equipment near $1.1tn. Nobody typed that — it falls out of
/// budget × share × horizon.
pub const EQUIP_HORIZON: f64 = 200.0;

/// What a force is worth with no procurement behind it at all: conscripts,
/// rifles, bases, and whatever the last government left. Not zero — an unequipped
/// army is the Iraqi army of 2003, not an empty field.
pub const BARE_FORCE: f64 = 0.55;

/// The most a nation can be over-equipped for its current budget. A government
/// that has just halved defence spending coasts on what it inherited; it does
/// not thereby become twice as strong.
pub const ADEQUACY_CAP: f64 = 1.30;

/// States with no coast in 1990, so the seeder does not give Chad a navy.
///
/// Geography, not invention — the same kind of transcribed fact
/// `NationRow::neighbours` already carries. Ethiopia is deliberately absent: it
/// held Assab and Massawa in 1990 and had a navy until 1991.
const LANDLOCKED: &[&str] = &[
    "Afghanistan", "Austria", "Bhutan", "Bolivia", "Botswana",
    "CentralAfricanRepublic", "Chad", "Czechoslovakia", "Hungary", "Laos",
    "Lesotho", "Luxembourg", "Malawi", "Mongolia", "Nepal", "Paraguay",
    "Swaziland", "Switzerland", "Uganda", "Zambia", "Zimbabwe",
];

/// The arsenal a nation opens 1990 already holding.
///
/// Nobody starts from nothing: an army in January 1990 is the accumulated
/// purchases of the thirty years before it, and the module's whole thesis is
/// that this inheritance is most of what a government fights with. Without this
/// every nation opens with an empty order book and the United States would have
/// fought Desert Storm with no equipment at all.
///
/// **Solved from money, which is what makes it exact.** Units are back-computed
/// so that book value equals the target for every nation by construction —
/// there is no per-nation tuning and no global deck that has to fit a 630x
/// spread in strength-per-dollar. Seeding to a strength target cannot be made to
/// work; seeding to a budget can, because a budget is transcribed fact.
///
/// Consumes NO randomness, by construction rather than by discipline: it takes a
/// record and not a `&mut WorldState`, so there is no generator to draw from.
/// Drawing here would shift every downstream draw and re-roll fifteen
/// emergent-history tests.
pub fn inheritance(r: &crate::data::NationRecord) -> Arsenal {
    let budget = (r.economy.gdp_bn * r.economy.mil_spend_gdp).max(0.0);
    let line = budget * PROCUREMENT_SHARE / 12.0;
    if line <= 0.0 || r.military.strength <= 0.0 {
        return Arsenal::default();
    }
    let want = line * EQUIP_HORIZON;

    // Capital intensity: money per point of strength. A conscript army with
    // rifles is cheap per point; an air force is not.
    let k = (budget / r.military.strength.max(1.0) / 3.0).clamp(0.0, 1.2);
    let sat = |x: f64| 1.0 - crate::exact::exp(-x);
    let cmd = if r.system == crate::world::EconomySystem::Command { 1.0 } else { 0.0 };
    let coast = if LANDLOCKED.contains(&r.id.code()) { 0.0 } else { 1.0 };
    let nuke = if r.military.nuclear { 1.0 } else { 0.0 };

    // One doctrine vector, not a per-nation table. Space keys on ABSOLUTE
    // budget rather than intensity, because a reconnaissance constellation is an
    // absolute-scale capability: at the transcribed 1990 figures the threshold
    // selects the United States and the Soviet Union and nobody else, which is
    // the truth about military space in 1990 and is emergent rather than typed.
    let shares = [
        (Class::Infantry, 1.10),
        (Class::Armour, 1.30 * sat(k / 0.12) * (1.0 + 0.45 * cmd)),
        (Class::Air, 1.75 * sat(k / 0.32)),
        (Class::Naval, 1.30 * sat(k / 0.50) * coast),
        (Class::Missile, 0.35 * sat(k / 0.40) * (1.0 + nuke)),
        (Class::Space, 0.35 * ((budget - 100.0) / 250.0).clamp(0.0, 1.0)),
    ];
    let total: f64 = shares.iter().map(|(_, w)| w).sum();
    if total <= 0.0 {
        return Arsenal::default();
    }

    // Vintage. A rich force renews; a poor one flies what it was given. This is
    // the mechanism rather than decoration: age is what `condition` reads.
    let mean_age = 132.0 + 240.0 * crate::exact::exp(-k / 0.40);
    let modernity = sat(k / 0.35);
    const TRANCHES: [(f64, f64); 3] = [(0.30, -84.0), (0.45, 0.0), (0.25, 108.0)];

    let mut held: Vec<Holding> = vec![];
    for (class, w) in shares {
        let share = w / total;
        if share <= 0.0 {
            continue;
        }
        for (i, (weight, offset)) in TRANCHES.iter().enumerate() {
            let g = modernity - 0.25 * i as f64;
            let id = tier_for(class, g, r.military.nuclear && i == 0);
            let Some(kit) = index_of(id) else { continue };
            let def = &DECK[kit as usize];
            let age = (mean_age + offset).clamp(0.0, 2.0 * def.service_months as f64);
            let cond = condition(def, age);
            if cond <= 0.0 || def.unit_cost <= 0.0 {
                continue;
            }
            let units = want * share * weight / (def.unit_cost * cond);
            match held.iter_mut().find(|h| h.kit == kit) {
                Some(h) => {
                    let t = h.units + units;
                    h.age = (h.age * h.units + age * units) / t.max(1e-12);
                    h.units = t;
                }
                None => held.push(Holding { kit, units, age }),
            }
        }
    }
    // Orders are deliberately NOT seeded: what a nation had on order in January
    // 1990 is not in the data, and the pipeline refills itself within one lead
    // time anyway.
    Arsenal { held, orders: vec![], preference: None, banked: 0.0 }
}

/// Which tier of a class a force at this generation fields.
///
/// The deterrent tier is gated on the record's own nuclear flag and given only
/// to the newest tranche, so France seeds one and Japan does not.
fn tier_for(class: Class, g: f64, deterrent: bool) -> &'static str {
    match class {
        Class::Infantry => if g >= 0.55 { "inf_mech" } else { "inf_light" },
        Class::Armour => if g >= 0.50 { "arm_gen3" } else { "arm_gen2" },
        Class::Air => {
            if g >= 0.60 { "air_gen4" } else if g >= 0.30 { "air_gen3" } else { "air_gen2" }
        }
        Class::Naval => {
            if g >= 0.72 { "nav_blue" } else if g >= 0.30 { "nav_escort" } else { "nav_patrol" }
        }
        Class::Missile => {
            if deterrent { "msl_deterrent" }
            else if g >= 0.70 { "msl_brm" }
            else { "msl_sam" }
        }
        Class::Space => "spc_recon",
    }
}

/// What a nation's holdings are worth at cost, in $bn.
///
/// The quantity the war model should read, per PROCUREMENT.md: `unit_cost` is
/// transcribed money under iron rule 4, while `quality` is authored judgement
/// that five independent reviews found wrong by up to four orders of magnitude
/// in both directions. Money is the column that is fact.
pub fn book_value(n: &Nation) -> f64 {
    n.arsenal
        .held
        .iter()
        .filter_map(|h| DECK.get(h.kit as usize).map(|d| h.units * d.unit_cost * condition(d, h.age)))
        .sum()
}

/// What share of the force its budget describes a nation has actually equipped.
///
/// **The only quantity `war.rs` reads out of this module**, and it is money
/// against money: what stands in the books, against what a force funded at the
/// reference share for the reference horizon would hold. Because it never reads
/// the `quality` column it survives every balance error in the deck — and five
/// independent reviews found that column wrong by between 5x and 10,000x in
/// opposite directions, so that is not a hypothetical.
///
/// Normalised so that a nation equipped exactly to its budget returns 1.0. The
/// seeder solves units from money, so every one of the 137 nations returns
/// exactly 1.0 in January 1990 and the war model opens unchanged to within
/// floating point. Everything after that is the player's doing.
pub fn adequacy(n: &Nation) -> f64 {
    adequacy_at(n, n.mil_spend_gdp)
}

/// [`adequacy`] at a military share the nation is not currently spending.
///
/// Split out so that "what would this budget sustain?" can be answered by the
/// sim rather than guessed at by a caller. It is not a question with a closed
/// form: adequacy FALLS as the share rises, because `want` is the equipment a
/// budget of that size implies and the books do not grow to meet it in the same
/// month. A caller that assumed the force curve was `k·sqrt(share)` — which is
/// what the shape of `war::sustained_force` looks like until you notice this
/// term is inside it — would be wrong in a direction that flatters raising
/// spending and punishes cutting it.
pub fn adequacy_at(n: &Nation, mil_spend_gdp: f64) -> f64 {
    let line = (n.gdp * mil_spend_gdp * PROCUREMENT_SHARE / 12.0).max(0.0);
    let want = line * EQUIP_HORIZON;
    if want <= 0.0 {
        return BARE_FORCE;
    }
    let f = (book_value(n) / want).clamp(0.0, ADEQUACY_CAP);
    BARE_FORCE + (1.0 - BARE_FORCE) * f
}

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

/// This month's procurement money before anything banked: the budget's
/// equipment share, one month of it.
///
/// Clamped rather than floored: `.max(0.0)` handles a negative GDP (which
/// this tree has produced — see ff77690) and a NaN, because f64::max returns
/// the non-NaN operand. It does not handle +inf, which would place an
/// infinite order and field an infinite arsenal.
pub(crate) fn budget_of(n: &Nation) -> f64 {
    (n.gdp * n.mil_spend_gdp * PROCUREMENT_SHARE / 12.0).clamp(0.0, 1e6)
}

/// What the line can spend this month: the budget plus whatever earlier
/// months banked with nothing to buy. The one expression `tick` orders
/// against and `resources::kit_need` sizes a draw against.
pub(crate) fn line_of(n: &Nation) -> f64 {
    budget_of(n) + n.arsenal.banked
}

/// Each kit's technology resolved to its index once. `available` and `pick`
/// run every nation-month, and resolving thirty-three names by search each
/// time was most of the arsenal's cost (SPEC section 8's "free win";
/// measured 2.2 µs a pick, six picks inside every resource evaluation). A
/// name the tree does not know resolves to `None` and stays unorderable,
/// exactly as before.
fn deck_tech() -> &'static [Option<u16>] {
    static T: std::sync::OnceLock<Vec<Option<u16>>> = std::sync::OnceLock::new();
    T.get_or_init(|| DECK.iter().map(|k| k.tech.and_then(crate::tech::index_of)).collect())
}

/// Whether `n` may order kit `i` today. The legacy tier is always available:
/// replacing what you already field is a budget decision, not a discovery.
fn orderable(n: &Nation, i: usize) -> bool {
    orderable_with(n, i, deck_tech())
}

/// `orderable` with the deck's technology column already in hand, so a scan
/// of the whole deck pays one `OnceLock` read rather than `DECK.len()` of
/// them. Same three cases, same answers.
fn orderable_with(n: &Nation, i: usize, tech: &[Option<u16>]) -> bool {
    match (DECK[i].tech, tech[i]) {
        (None, _) => true,
        (Some(_), None) => false,
        (Some(_), Some(t)) => n.tech.knows_index(t),
    }
}

/// The deck ranked exactly as `pick`'s fold ranks it — quality per pound
/// descending, the lower `DECK` index first on a tie — computed once.
///
/// `pick`'s fold takes the maximum of a STRICT TOTAL order (the value
/// comparison is broken by the index, so no two entries compare equal), and
/// the maximum of a strict total order over a subset is the first element of
/// the whole set's sorted sequence that the subset contains. So "the first
/// orderable entry in this order" IS the fold's answer, and
/// `the_ranked_pick_is_the_folded_pick` holds it to that over a real board.
/// It saves the fold's per-entry division and, for a nation that holds the
/// modern technology, stops within a few entries instead of scanning 46.
fn value_order() -> &'static [u16] {
    static ORDER: std::sync::OnceLock<Vec<u16>> = std::sync::OnceLock::new();
    ORDER.get_or_init(|| {
        let mut v: Vec<u16> = (0..DECK.len() as u16).collect();
        v.sort_by(|a, b| {
            deck_value(*b as usize)
                .partial_cmp(&deck_value(*a as usize))
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(a.cmp(b))
        });
        v
    })
}

/// Quality per pound — the fold's `value`, named so the ranking and the test
/// that checks it read the same expression.
fn deck_value(i: usize) -> f64 {
    DECK[i].quality / DECK[i].unit_cost.max(1e-9)
}

/// What this nation may order today: everything whose technology it holds.
pub fn available(n: &Nation) -> Vec<u16> {
    (0..DECK.len()).filter(|i| orderable(n, *i)).map(|i| i as u16).collect()
}

/// Procurement, once a month.
///
/// Money becomes orders, orders become holdings when their lead time runs out,
/// and everything already held gets a month older. Nothing here rolls dice: a
/// procurement programme is a budget line, not a gamble.
pub fn tick(w: &mut WorldState) {
    // Technology has already advanced this month, so the spot market now
    // sees the exact procurement recipes this tick will attempt to consume.
    crate::resources::clear_spot_market(w);
    let ids: Vec<NationId> = w.nations.iter().filter(|n| n.alive).map(|n| n.id).collect();
    for id in ids {
        let budget = budget_of(w.nation(id));
        let line = line_of(w.nation(id));
        let directed = w.rules.manufacturing_system
            && crate::manufacturing::lines_for(w, id).next().is_some();
        // The one gate the resource system has in cut one. It checks the kit
        // procurement would pick against what the nation can get this month
        // — its own flow, or any open holder — and delays the line only when
        // the pile it kept for exactly this is spent. A player's stated
        // preference stalls and says so; the staff buy the best kit whose
        // inputs are held. In an open world this returns the pick untouched.
        let (choice, stall) = if directed {
            (None, None)
        } else {
            match pick(w.nation(id)) {
                Some(kit) if w.rules.resource_gates => crate::resources::gate(w, id, kit, line),
                other => (other, None),
            }
        };
        if let Some(s) = stall {
            w.headline(s.headline());
        }
        {
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
            // `line` is `line_of(n)` as read before the gate: budget plus banked.
            if !directed {
                n.arsenal.banked = 0.0;
                if choice.is_none() {
                    n.arsenal.banked = line.min(budget * 24.0);
                }
                if let Some(kit) = choice {
                    let def = &DECK[kit as usize];
                    let units = line / def.unit_cost.max(1e-9);
                    if units > 0.0 {
                        match n
                            .arsenal
                            .orders
                            .iter_mut()
                            .find(|o| o.kit == kit && o.due == def.lead_months)
                        {
                            Some(o) => o.units += units,
                            None => n.arsenal.orders.push(Order {
                                kit,
                                units,
                                due: def.lead_months,
                            }),
                        }
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

        // A directed board owns this month's placement arm. It uses the same
        // opening `line` computed above through `line_of`, and writes the same
        // `Order` rows; nothing automatic is placed beside it.
        if directed {
            crate::manufacturing::settle_nation(w, id);
        }
    }
}

/// Every orderable kit, best-first — quality per pound descending, `DECK`
/// order breaking ties — with the player's preference first. `pick` is the
/// head of this list; the resource gate walks the rest only in the month a
/// line is delayed, which is why this sorts and `pick` does not.
pub(crate) fn ranked(n: &Nation) -> Vec<u16> {
    let mut open = available(n);
    open.sort_by(|a, b| {
        let va = DECK[*a as usize].quality / DECK[*a as usize].unit_cost.max(1e-9);
        let vb = DECK[*b as usize].quality / DECK[*b as usize].unit_cost.max(1e-9);
        vb.partial_cmp(&va).unwrap_or(std::cmp::Ordering::Equal).then(a.cmp(b))
    });
    if let Some(p) = n.arsenal.preference.as_deref().and_then(index_of) {
        if let Some(pos) = open.iter().position(|k| *k == p) {
            open.remove(pos);
            open.insert(0, p);
        }
    }
    open
}

/// What procurement buys this month.
///
/// The player's standing preference if they have set one and can still build it;
/// otherwise the best quality per pound available, which is what a staff with no
/// political direction does. Deterministic — `DECK` order breaks every tie.
pub(crate) fn pick(n: &Nation) -> Option<u16> {
    if let Some(p) = n.arsenal.preference.as_deref().and_then(index_of) {
        if orderable(n, p as usize) {
            return Some(p);
        }
    }
    // The same maximum `available(n)` sorted to — quality per pound, the
    // lower `DECK` index on a tie — read off the precomputed ranking instead
    // of refolded, and with one `deck_tech()` read for the whole scan.
    // `pick` is called for every nation every month by `resources::draw`;
    // the fold's 46 divisions and 46 `OnceLock` reads were 0.0284 of the
    // 0.0440 ms/month the appetite term cost (measured 2026-09-02).
    let tech = deck_tech();
    value_order().iter().copied().find(|i| orderable_with(n, *i as usize, tech))
}

#[cfg(test)]
mod ranking_tests {
    use super::*;

    /// `pick`'s original fold, kept verbatim as the thing the ranking is
    /// checked against. If this and `pick` ever disagree, `pick` is wrong.
    fn folded_pick(n: &Nation) -> Option<u16> {
        if let Some(p) = n.arsenal.preference.as_deref().and_then(index_of) {
            if orderable(n, p as usize) {
                return Some(p);
            }
        }
        let value = |i: usize| DECK[i].quality / DECK[i].unit_cost.max(1e-9);
        let mut best: Option<usize> = None;
        for i in 0..DECK.len() {
            if !orderable(n, i) {
                continue;
            }
            best = Some(match best {
                None => i,
                Some(b) => {
                    let ahead = value(i)
                        .partial_cmp(&value(b))
                        .unwrap_or(std::cmp::Ordering::Equal)
                        .then(b.cmp(&i));
                    if ahead == std::cmp::Ordering::Greater {
                        i
                    } else {
                        b
                    }
                }
            });
        }
        best.map(|i| i as u16)
    }

    /// The precomputed ranking answers what the fold answered, over a real
    /// board and a real month range: 137 nations every month for 25 years,
    /// with the market off and on, as the deck opens up under research.
    ///
    /// Iron rule 7: an INVARIANT — "the ranked pick is the folded pick" is a
    /// universal claim, so a small sample can only lose power, never produce
    /// a false red, and there is no n to derive. The power it has is recorded
    /// instead: measured on this tree 2026-09-02, each arm compares 46,088
    /// picks and sees 2 distinct kits chosen across them, so the answer is
    /// not one constant compared to itself (the distinct count is asserted
    /// above one). What this range does NOT exercise is a tie: reversing the
    /// ranking's tie break (`b.cmp(a)` for `a.cmp(b)` in `value_order`) was
    /// run 2026-09-02 and left the test GREEN, because no two `DECK` entries
    /// tie on quality per pound. That arm of the order is therefore carried
    /// by the argument in `value_order`'s comment and by `DECK` having no
    /// duplicate value, not by this test — recorded as decorative rather
    /// than believed.
    ///
    /// RED-CHECK, run 2026-09-02 on this tree against a perturbation the
    /// board does see: sorting `value_order` ASCENDING by value fails at the
    /// first nation compared — "market false, 1990-2 USA: ranked Some(12),
    /// folded Some(30)". Restored.
    #[test]
    fn the_ranked_pick_is_the_folded_pick() {
        use crate::init::world_1990;
        use crate::world::GameRules;
        use std::collections::BTreeSet;
        for market in [false, true] {
            let mut w = world_1990(GameRules { resource_market: market, ..GameRules::default() });
            let (mut compared, mut seen) = (0usize, BTreeSet::new());
            for _ in 0..(12 * 25) {
                crate::tick_month(&mut w, &[]);
                for n in w.nations.iter().filter(|n| n.alive) {
                    let ranked = pick(n);
                    assert_eq!(
                        ranked,
                        folded_pick(n),
                        "market {market}, {}-{} {}: ranked {ranked:?}, folded {:?}",
                        w.year,
                        w.month,
                        n.id.code(),
                        folded_pick(n)
                    );
                    compared += 1;
                    seen.insert(ranked);
                }
            }
            println!("    market {market}: {compared} picks compared, {} distinct kits chosen", seen.len());
            assert!(compared > 40_000, "market {market}: {compared} is not a real board");
            assert!(seen.len() > 1, "market {market}: one constant answer proves nothing");
        }
    }
}
