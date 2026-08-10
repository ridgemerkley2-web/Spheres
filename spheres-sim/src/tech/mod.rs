//! The technology tree: what a nation knows, what it is paying to learn, and
//! what knowledge does to it once it has it.
//!
//! The tree is static data — a `TechDef` per technology, concatenated from the
//! eight domain modules below plus a small foundation set defined here. Nothing
//! in the data says who gets what or when. A nation accumulates research out of
//! its own economy, spends it across the eight domains in proportions its own
//! situation dictates, and unlocks whatever its prerequisites allow. That the
//! rich and open economies run ahead is a consequence of the arithmetic, not an
//! instruction in the table.
//!
//! `earliest_year` is a floor and only a floor. It stops a 1990 command economy
//! buying the smartphone with a big enough budget; it never promises the
//! technology to anyone in that year, or ever.

use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

use crate::world::{EconomySystem, Nation, NationId, WorldState};

pub mod aerospace;
pub mod agriculture;
pub mod biotech;
pub mod communications;
pub mod computing;
pub mod energy;
pub mod materials;
pub mod transport;

// ---------------------------------------------------------------------------
// Static definition types
// ---------------------------------------------------------------------------

/// The eight branches of the tree. The order here is the order research budget
/// is walked in — fixed, so no iteration order can ever be in question.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Domain {
    Computing,
    Communications,
    Energy,
    Materials,
    Aerospace,
    Biotech,
    Transport,
    Agriculture,
}

pub const DOMAIN_COUNT: usize = 8;
pub const DOMAINS: [Domain; DOMAIN_COUNT] = [
    Domain::Computing,
    Domain::Communications,
    Domain::Energy,
    Domain::Materials,
    Domain::Aerospace,
    Domain::Biotech,
    Domain::Transport,
    Domain::Agriculture,
];

impl Domain {
    pub fn index(self) -> usize {
        match self {
            Domain::Computing => 0,
            Domain::Communications => 1,
            Domain::Energy => 2,
            Domain::Materials => 3,
            Domain::Aerospace => 4,
            Domain::Biotech => 5,
            Domain::Transport => 6,
            Domain::Agriculture => 7,
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Domain::Computing => "Computing & Software",
            Domain::Communications => "Communications & Space",
            Domain::Energy => "Energy",
            Domain::Materials => "Materials & Manufacturing",
            Domain::Aerospace => "Aerospace & Military",
            Domain::Biotech => "Biotech & Medicine",
            Domain::Transport => "Transport",
            Domain::Agriculture => "Agriculture & Environment",
        }
    }
}

/// Eras are calibration brackets, not phases anyone passes through. They exist
/// so costs and year floors stay in proportion across eight authors who cannot
/// see each other's files.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Era {
    /// 1990-1999
    Information,
    /// 2000-2009
    Networked,
    /// 2010-2019
    Platform,
    /// 2020-2029
    Intelligent,
    /// 2030+ — speculative, and must be commented as such
    Frontier,
}

impl Era {
    pub fn rank(self) -> u8 {
        match self {
            Era::Information => 0,
            Era::Networked => 1,
            Era::Platform => 2,
            Era::Intelligent => 3,
            Era::Frontier => 4,
        }
    }
    /// The years an entry in this era may declare as its floor.
    pub fn window(self) -> (i32, i32) {
        match self {
            Era::Information => (1990, 1999),
            Era::Networked => (2000, 2009),
            Era::Platform => (2010, 2019),
            Era::Intelligent => (2020, 2029),
            Era::Frontier => (2030, 2100),
        }
    }
    /// Research points an entry in this era may cost. The bands rise faster than
    /// GDP does; what closes the gap is that research itself gets cheaper as the
    /// tools improve, which is what `ResearchRate` and `CostReduction` are for.
    pub fn cost_band(self) -> (f64, f64) {
        match self {
            Era::Information => (40.0, 110.0),
            Era::Networked => (90.0, 200.0),
            Era::Platform => (170.0, 360.0),
            Era::Intelligent => (300.0, 600.0),
            Era::Frontier => (550.0, 1100.0),
        }
    }
}

/// Everything a technology can do to the simulation. One vocabulary for all
/// eight domains: an author who needs a verb that is not here has misread the
/// scope of their domain, not found a gap.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum Effect {
    /// Permanent addition to `tfp_trend`, an annual rate. 0.00005 is a useful
    /// tool; 0.00040 is a general-purpose technology that reorganises an
    /// economy. A domain's Productivity values should sum to about 0.0020.
    Productivity(f64),
    /// Multiplier on research output. 0.03 = this nation now researches 3% faster.
    ResearchRate(f64),
    /// Fractional discount on every future cost in one domain, 0..1.
    CostReduction { domain: Domain, frac: f64 },
    /// Additive points onto the military strength a given budget sustains.
    MilitaryStrength(f64),
    /// Multiplier on how much strength military spending buys. 0.04 = +4%.
    MilitaryEfficiency(f64),
    /// Fractional lift to oil output — recovery, not new geology. 0.03 = +3%.
    OilYield(f64),
    /// Fractional cut in what a barrel has to do. Softens the oil price for
    /// importers and its inflation pass-through. 0.03 = 3% less exposed.
    EnergyEfficiency(f64),
    /// Non-oil resources: ore grades, fisheries, water, arable yield, recycling.
    /// Reads through to productivity. 0.02 is a solid improvement.
    ResourceYield(f64),
    /// Mortality and morbidity. Raises population growth and, mildly, output.
    /// 0.05 is a vaccine; 0.20 is a whole class of disease retreating.
    Health(f64),
    /// Births. Negative for anything that lets people have fewer children on
    /// purpose. Same scale as Health, opposite sign convention.
    Fertility(f64),
    /// Pull on the regime's stability equilibrium. Positive for anything that
    /// makes a state easier to hold together, negative for anything corrosive.
    Stability(f64),
    /// How fast this nation absorbs what the rest of the world already knows.
    DiffusionSpeed(f64),
    /// How readily this nation's own knowledge leaks outward — publication,
    /// open standards, exported capital equipment, students who go home.
    DiffusionEmission(f64),
    /// Environmental load, positive for cleaner. Feeds health and stability.
    Environment(f64),
    /// Multiplier on what capital investment buys. Logistics, machine tools,
    /// project management, anything that makes a dollar of plant go further.
    InvestmentEfficiency(f64),
}

/// One technology. Data only — no behaviour, no nation, no date of arrival.
#[derive(Clone, Debug)]
pub struct TechDef {
    /// Stable, unique, snake_case, prefixed with the domain's short code.
    pub id: &'static str,
    pub name: &'static str,
    pub domain: Domain,
    pub era: Era,
    /// Ids that must already be known. May cross domains, but only to ids on
    /// the published cross-domain list.
    pub prereqs: Vec<&'static str>,
    /// Research points. Must sit inside `era.cost_band()`.
    pub cost: f64,
    /// The first year this may be unlocked by anyone. A floor, never a schedule.
    /// Must sit inside `era.window()`.
    pub earliest_year: i32,
    pub effects: Vec<Effect>,
}

/// The constructor every domain file uses. Slices in, owned data out, so an
/// author never has to think about lifetimes or const promotion.
#[allow(clippy::too_many_arguments)]
pub fn tech(
    id: &'static str,
    name: &'static str,
    domain: Domain,
    era: Era,
    prereqs: &[&'static str],
    cost: f64,
    earliest_year: i32,
    effects: &[Effect],
) -> TechDef {
    TechDef {
        id,
        name,
        domain,
        era,
        prereqs: prereqs.to_vec(),
        cost,
        earliest_year,
        effects: effects.to_vec(),
    }
}

// ---------------------------------------------------------------------------
// The registry
// ---------------------------------------------------------------------------

/// Shared infrastructure every domain is allowed to hang prerequisites off.
/// These live here rather than in a domain file so that no author's work can
/// break another's: the cross-domain anchors are guaranteed to exist because
/// this module defines them.
///
/// Sources are the first real deployment, not the first paper.
fn foundation() -> Vec<TechDef> {
    use Domain::*;
    use Effect::*;
    use Era::*;
    vec![
        // Sub-micron CMOS in volume production: Intel's 80486 shipped on a
        // 1.0-micron process in 1989 and the industry was below it by 1991.
        tech(
            "core_cmos_submicron", "Sub-Micron CMOS Fabrication", Computing, Information,
            &[], 45.0, 1990,
            &[Productivity(0.00025), ResearchRate(0.04)],
        ),
        // TCP/IP internetworking: the ARPANET was decommissioned in 1990 and the
        // NSFNET backbone carried the traffic it had proved could be carried.
        tech(
            "core_packet_internetworking", "Packet Internetworking", Communications, Information,
            &[], 40.0, 1990,
            &[Productivity(0.00020), DiffusionSpeed(0.05), DiffusionEmission(0.05)],
        ),
        // Single-mode long-haul fibre: TAT-8, the first transatlantic fibre
        // cable, entered service in 1988 and carried more than every copper
        // cable before it combined.
        tech(
            "core_single_mode_fiber", "Single-Mode Optical Trunk", Communications, Information,
            &[], 45.0, 1990,
            &[Productivity(0.00018), DiffusionEmission(0.04)],
        ),
        // Carbon-fibre primary structure: in service through the 1980s on the
        // AV-8B's wing and the Boeing 767's control surfaces.
        tech(
            "core_carbon_composites", "Carbon-Fibre Composite Structures", Materials, Information,
            &[], 50.0, 1990,
            &[Productivity(0.00012), MilitaryEfficiency(0.03)],
        ),
        // Combined-cycle gas turbine: GE's Frame 7F entered commercial service
        // in 1990 and took thermal efficiency past 50% for the first time.
        tech(
            "core_combined_cycle_turbine", "Combined-Cycle Gas Turbine", Energy, Information,
            &[], 55.0, 1990,
            &[EnergyEfficiency(0.05), Productivity(0.00012)],
        ),
        // PCR: Mullis conceived it in 1983, and Cetus shipped Taq polymerase and
        // the first thermal cycler in 1987, which is when it became a method
        // rather than a result.
        tech(
            "core_pcr", "Polymerase Chain Reaction", Biotech, Information,
            &[], 40.0, 1990,
            &[Health(0.06), ResearchRate(0.02)],
        ),
        // GPS reached initial operational capability in December 1993 and full
        // capability in 1995.
        tech(
            "core_gnss", "Satellite Navigation Constellation", Communications, Information,
            &["core_packet_internetworking"], 90.0, 1993,
            &[Productivity(0.00015), MilitaryEfficiency(0.05), InvestmentEfficiency(0.02)],
        ),
        // The first GSM call was placed on Radiolinja's network in Finland on
        // 1 July 1991.
        tech(
            "core_digital_cellular", "Digital Cellular Network", Communications, Information,
            &["core_cmos_submicron"], 70.0, 1991,
            &[Productivity(0.00022), DiffusionSpeed(0.04)],
        ),
        // Sony commercialised the lithium-ion cell in 1991.
        tech(
            "core_lithium_ion_cell", "Lithium-Ion Cell", Materials, Information,
            &[], 60.0, 1991,
            &[Productivity(0.00010), EnergyEfficiency(0.03)],
        ),
        // 248nm KrF deep-ultraviolet steppers went into volume production in the
        // mid-1990s and carried the industry down to 250nm and below.
        tech(
            "core_duv_lithography", "Deep-Ultraviolet Lithography", Computing, Information,
            &["core_cmos_submicron"], 105.0, 1997,
            &[Productivity(0.00030), ResearchRate(0.05), CostReduction { domain: Computing, frac: 0.05 }],
        ),
        // The Human Genome Project and Celera published draft sequences in
        // February 2001.
        tech(
            "core_genome_sequencing", "Whole-Genome Sequencing", Biotech, Networked,
            &["core_pcr"], 150.0, 2001,
            &[Health(0.10), ResearchRate(0.03)],
        ),
        // AlexNet won ImageNet in 2012 on two consumer GPUs, which is the moment
        // the hardware and the method found each other.
        tech(
            "core_gpu_deep_learning", "GPU Deep Learning", Computing, Platform,
            &["core_duv_lithography"], 260.0, 2012,
            &[Productivity(0.00035), ResearchRate(0.10), CostReduction { domain: Computing, frac: 0.06 }],
        ),
        // Jinek et al. showed programmable Cas9 cutting in 2012; Zhang and Church
        // showed it working in mammalian cells in January 2013.
        tech(
            "core_crispr_editing", "CRISPR-Cas9 Editing", Biotech, Platform,
            &["core_genome_sequencing"], 280.0, 2013,
            &[Health(0.12), ResourceYield(0.02)],
        ),
        // Falcon 9 landed a first stage in December 2015 and flew one again in
        // March 2017.
        tech(
            "core_reusable_booster", "Reusable Orbital Booster", Aerospace, Platform,
            &["core_carbon_composites"], 300.0, 2016,
            &[
                MilitaryEfficiency(0.06),
                Productivity(0.00010),
                CostReduction { domain: Aerospace, frac: 0.08 },
            ],
        ),
    ]
}

fn build_registry() -> Vec<TechDef> {
    let mut v = foundation();
    v.extend(computing::techs());
    v.extend(communications::techs());
    v.extend(energy::techs());
    v.extend(materials::techs());
    v.extend(aerospace::techs());
    v.extend(biotech::techs());
    v.extend(transport::techs());
    v.extend(agriculture::techs());
    v
}

/// The whole tree, built once. Pure data, so caching it changes nothing about
/// what the simulation computes.
pub fn registry() -> &'static [TechDef] {
    static REG: OnceLock<Vec<TechDef>> = OnceLock::new();
    REG.get_or_init(build_registry)
}

/// Ids sorted for lookup. A sorted slice rather than a hash map, because
/// nothing in this file may ever depend on hash iteration order.
fn id_index() -> &'static [(&'static str, u16)] {
    static IDX: OnceLock<Vec<(&'static str, u16)>> = OnceLock::new();
    IDX.get_or_init(|| {
        let mut v: Vec<(&'static str, u16)> = registry()
            .iter()
            .enumerate()
            .map(|(i, t)| (t.id, i as u16))
            .collect();
        v.sort_by(|a, b| a.0.cmp(b.0));
        v
    })
}

pub fn index_of(id: &str) -> Option<u16> {
    let idx = id_index();
    idx.binary_search_by(|probe| probe.0.cmp(id)).ok().map(|i| idx[i].1)
}

/// Prerequisites resolved to indices once. Any id that does not resolve is
/// dropped here and caught loudly by `tree_is_well_formed`.
fn prereq_table() -> &'static [Vec<u16>] {
    static PRE: OnceLock<Vec<Vec<u16>>> = OnceLock::new();
    PRE.get_or_init(|| {
        registry()
            .iter()
            .map(|t| t.prereqs.iter().filter_map(|p| index_of(p)).collect())
            .collect()
    })
}

// ---------------------------------------------------------------------------
// Per-nation state
// ---------------------------------------------------------------------------

/// Everything a nation's technologies have done to it, kept as one running
/// total so effects can be reapplied every tick without being counted twice.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TechBonuses {
    pub productivity: f64,
    pub research_rate: f64,
    pub military_strength: f64,
    pub military_efficiency: f64,
    pub oil_yield: f64,
    pub energy_efficiency: f64,
    pub resource_yield: f64,
    pub health: f64,
    pub fertility: f64,
    pub stability: f64,
    pub diffusion_speed: f64,
    pub diffusion_emission: f64,
    pub environment: f64,
    pub investment_efficiency: f64,
    pub cost_reduction: Vec<f64>,
}

impl Default for TechBonuses {
    fn default() -> Self {
        TechBonuses {
            productivity: 0.0,
            research_rate: 0.0,
            military_strength: 0.0,
            military_efficiency: 0.0,
            oil_yield: 0.0,
            energy_efficiency: 0.0,
            resource_yield: 0.0,
            health: 0.0,
            fertility: 0.0,
            stability: 0.0,
            diffusion_speed: 0.0,
            diffusion_emission: 0.0,
            environment: 0.0,
            investment_efficiency: 0.0,
            cost_reduction: vec![0.0; DOMAIN_COUNT],
        }
    }
}

impl TechBonuses {
    fn absorb(&mut self, e: &Effect) {
        match e {
            Effect::Productivity(v) => self.productivity += v,
            Effect::ResearchRate(v) => self.research_rate += v,
            Effect::CostReduction { domain, frac } => {
                self.cost_reduction[domain.index()] += frac;
            }
            Effect::MilitaryStrength(v) => self.military_strength += v,
            Effect::MilitaryEfficiency(v) => self.military_efficiency += v,
            Effect::OilYield(v) => self.oil_yield += v,
            Effect::EnergyEfficiency(v) => self.energy_efficiency += v,
            Effect::ResourceYield(v) => self.resource_yield += v,
            Effect::Health(v) => self.health += v,
            Effect::Fertility(v) => self.fertility += v,
            Effect::Stability(v) => self.stability += v,
            Effect::DiffusionSpeed(v) => self.diffusion_speed += v,
            Effect::DiffusionEmission(v) => self.diffusion_emission += v,
            Effect::Environment(v) => self.environment += v,
            Effect::InvestmentEfficiency(v) => self.investment_efficiency += v,
        }
    }
    fn cost_reduction_for(&self, d: Domain) -> f64 {
        self.cost_reduction.get(d.index()).copied().unwrap_or(0.0)
    }

    // Every channel saturates. The tenth thing that makes a factory more
    // productive does less than the first did, because the first one already
    // took the slack out — and, less romantically, because eight authors who
    // cannot see each other's files will collectively overspend, and the
    // simulation must absorb that rather than fly apart. Raw sums are what gets
    // stored; these are what gets read.
    pub fn productivity_eff(&self) -> f64 {
        saturate(self.productivity, 0.012)
    }
    pub fn research_rate_eff(&self) -> f64 {
        saturate(self.research_rate, 2.50)
    }
    pub fn military_strength_eff(&self) -> f64 {
        saturate(self.military_strength, 60.0)
    }
    pub fn military_efficiency_eff(&self) -> f64 {
        saturate(self.military_efficiency, 1.50)
    }
    pub fn oil_yield_eff(&self) -> f64 {
        saturate(self.oil_yield, 0.50)
    }
    pub fn energy_efficiency_eff(&self) -> f64 {
        saturate(self.energy_efficiency, 0.75)
    }
    pub fn resource_yield_eff(&self) -> f64 {
        saturate(self.resource_yield, 0.60)
    }
    pub fn health_eff(&self) -> f64 {
        saturate(self.health, 2.00)
    }
    pub fn fertility_eff(&self) -> f64 {
        -saturate(-self.fertility, 1.50)
    }
    pub fn stability_eff(&self) -> f64 {
        saturate(self.stability, 5.0)
    }
    pub fn diffusion_speed_eff(&self) -> f64 {
        saturate(self.diffusion_speed, 0.50)
    }
    pub fn diffusion_emission_eff(&self) -> f64 {
        saturate(self.diffusion_emission, 1.00)
    }
    pub fn environment_eff(&self) -> f64 {
        saturate(self.environment, 3.00)
    }
    pub fn investment_efficiency_eff(&self) -> f64 {
        saturate(self.investment_efficiency, 0.80)
    }
}

/// Diminishing returns with a hard ceiling, smooth and sign-preserving for the
/// positive branch. `saturate(x, c)` is roughly `x` while `x` is small against
/// `c`, and approaches `c` from below however large `x` gets.
fn saturate(x: f64, cap: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    cap * (1.0 - (-x / cap).exp())
}

/// What a nation knows and what it is working on. Indices into `registry()`,
/// kept sorted, which keeps saves compact and comparisons total.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct TechState {
    /// Sorted ascending, no duplicates. Held as registry indices for speed, but
    /// written to disk as stable ids — see `known_serde`.
    #[serde(with = "known_serde")]
    pub known: Vec<u16>,
    /// One project per domain, or none if nothing is eligible. Also written by
    /// id, so a resumed save carries on researching the same thing.
    #[serde(with = "focus_serde")]
    pub focus: Vec<Option<u16>>,
    /// Research points banked against the current project in each domain.
    pub progress: Vec<f64>,
    /// Lifetime research points generated — the honest measure of effort.
    pub research_total: f64,
    /// The nation's productivity trend before any technology touched it.
    pub tfp_base: f64,
    /// How much of the accumulated oil-yield bonus has been worked into the
    /// wells so far. Recovery arrives gradually and cannot be undone.
    pub oil_yield_applied: f64,
    /// Rebuilt from `known` every tick, so the totals can never drift away from
    /// the technologies that justify them. Still written to disk because
    /// `economy::tick` reads it before `tech::tick` gets a chance to rebuild,
    /// so a freshly loaded save must arrive with it already correct.
    pub bonus: TechBonuses,
    /// Technologies actually put into service per year, smoothed. Catching up is
    /// something a nation *does*, not a position it occupies, and this is the
    /// measure of it doing so. Defaulted on older saves, where it rebuilds
    /// within a year or two of play.
    #[serde(default)]
    pub absorption_rate: f64,
    /// False on a save written before this module existed, or on a default.
    pub initialized: bool,
}

// The registry is a concatenation of eight independently-authored files, so a
// technology's index moves whenever any earlier domain gains an entry. Indices
// are therefore a runtime detail and never touch the disk: saves carry the
// stable ids from `TechDef::id`, and an id this build no longer knows is
// dropped rather than silently reinterpreted as its neighbour.

mod known_serde {
    use super::*;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(v: &[u16], s: S) -> Result<S::Ok, S::Error> {
        let reg = registry();
        let ids: Vec<&str> = v.iter().filter_map(|i| reg.get(*i as usize).map(|d| d.id)).collect();
        ids.serialize(s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<u16>, D::Error> {
        let ids = Vec::<String>::deserialize(d)?;
        let mut out: Vec<u16> = ids.iter().filter_map(|id| index_of(id)).collect();
        out.sort_unstable();
        out.dedup();
        Ok(out)
    }
}

mod focus_serde {
    use super::*;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(v: &[Option<u16>], s: S) -> Result<S::Ok, S::Error> {
        let reg = registry();
        let ids: Vec<Option<&str>> = v
            .iter()
            .map(|o| o.and_then(|i| reg.get(i as usize).map(|d| d.id)))
            .collect();
        ids.serialize(s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<Option<u16>>, D::Error> {
        let ids = Vec::<Option<String>>::deserialize(d)?;
        Ok(ids.iter().map(|o| o.as_deref().and_then(index_of)).collect())
    }
}

impl TechState {
    pub fn new(tfp_base: f64) -> Self {
        TechState {
            known: vec![],
            focus: vec![None; DOMAIN_COUNT],
            progress: vec![0.0; DOMAIN_COUNT],
            research_total: 0.0,
            tfp_base,
            oil_yield_applied: 0.0,
            bonus: TechBonuses::default(),
            absorption_rate: 0.0,
            initialized: true,
        }
    }

    /// A successor state keeps the laboratories, the factories and the people
    /// who staffed them. It does not keep the research programme.
    pub fn inherit(parent: &TechState, tfp_base: f64) -> Self {
        TechState {
            known: parent.known.clone(),
            focus: vec![None; DOMAIN_COUNT],
            progress: vec![0.0; DOMAIN_COUNT],
            research_total: 0.0,
            tfp_base,
            oil_yield_applied: parent.oil_yield_applied,
            bonus: parent.bonus.clone(),
            // The programme stops; the plants that were already being fitted out
            // do not. A successor carries its parent's absorption into its first
            // years and then has to earn it.
            absorption_rate: parent.absorption_rate * 0.5,
            initialized: true,
        }
    }

    pub fn knows_index(&self, t: u16) -> bool {
        self.known.binary_search(&t).is_ok()
    }
    pub fn knows(&self, id: &str) -> bool {
        match index_of(id) {
            Some(i) => self.knows_index(i),
            None => false,
        }
    }
    pub fn count(&self) -> usize {
        self.known.len()
    }
    /// Display names of everything known, in registry order.
    pub fn known_names(&self) -> Vec<&'static str> {
        let reg = registry();
        self.known.iter().map(|i| reg[*i as usize].name).collect()
    }

    /// Recompute the running totals from scratch. Cheap, and it makes the
    /// bonuses a pure function of what the nation knows — a save that loses or
    /// drops a technology loses its effect too, rather than keeping a total
    /// nothing accounts for.
    fn rebuild_bonus(&mut self) {
        let reg = registry();
        let mut fresh = TechBonuses::default();
        fresh.cost_reduction.resize(DOMAIN_COUNT, 0.0);
        for i in &self.known {
            if let Some(def) = reg.get(*i as usize) {
                for e in &def.effects {
                    fresh.absorb(e);
                }
            }
        }
        self.bonus = fresh;
    }

    fn ensure_shape(&mut self, tfp_now: f64) {
        if !self.initialized {
            self.tfp_base = tfp_now;
            self.initialized = true;
        }
        self.rebuild_bonus();
        if self.focus.len() != DOMAIN_COUNT {
            self.focus.resize(DOMAIN_COUNT, None);
        }
        if self.progress.len() != DOMAIN_COUNT {
            self.progress.resize(DOMAIN_COUNT, 0.0);
        }
        if self.bonus.cost_reduction.len() != DOMAIN_COUNT {
            self.bonus.cost_reduction.resize(DOMAIN_COUNT, 0.0);
        }
    }

    fn learn(&mut self, t: u16) {
        if let Err(pos) = self.known.binary_search(&t) {
            self.known.insert(pos, t);
            for e in &registry()[t as usize].effects {
                self.bonus.absorb(e);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// The research engine
// ---------------------------------------------------------------------------

/// Development proxy, the same one the growth model uses, so a nation's
/// research capacity and its catch-up growth read from one idea of how rich
/// and how deep its economy is.
fn development(n: &Nation) -> f64 {
    let gdp_pc = n.gdp * 1000.0 / n.population.max(0.001);
    (gdp_pc / 24000.0).clamp(0.0, 1.0)
}

/// Research points generated this month. R&D intensity rises with development
/// and with how much of output is being ploughed back; the level is calibrated
/// so that the United States of 1990 spends a little over two percent of GDP on
/// research and China of 1990 spends well under one. The floor is not zero even
/// for the poorest, because the engineers who install imported plant and work
/// out why it keeps breaking are doing research whatever the budget calls it.
fn research_output(w: &WorldState, n: &Nation, dev: f64) -> f64 {
    let invest = n.state_invest_gdp + n.priv_invest_gdp;
    let intensity = (0.008 + 0.017 * dev) * (0.55 + 1.5 * invest);
    let mut out = n.gdp * intensity / 12.0;

    // Better tools make more research out of the same money.
    out *= 1.0 + n.tech.bonus.research_rate_eff();

    // A command economy can order the effort and does; what it cannot order is
    // anyone to want the result. The laboratories are full and the return is thin.
    if n.system == EconomySystem::Command {
        out *= 0.80;
    }
    // A state that is coming apart is not funding anything reliably.
    if n.stability < 40.0 {
        out *= 0.60 + n.stability / 100.0;
    }
    if w.at_war(n.id) {
        out *= 0.85;
    }
    out *= (1.0 - 0.03 * w.sanctioned_by_count(n.id) as f64).max(0.4);
    out.max(0.0)
}

/// Where the research money goes. Nothing here names a nation: an oil importer
/// with an expensive barrel funds energy, a state at war funds weapons, a poor
/// country funds the harvest, a rich one funds the laboratory. The weights are
/// read off the nation's own condition every month.
fn domain_weights(w: &WorldState, n: &Nation, dev: f64) -> [f64; DOMAIN_COUNT] {
    let at_war = w.at_war(n.id);
    let oil_stress = ((w.oil_price - 20.0) / 20.0).clamp(0.0, 2.0);
    let invest = n.state_invest_gdp + n.priv_invest_gdp;
    let mut wt = [1.0f64; DOMAIN_COUNT];
    wt[Domain::Computing.index()] = 0.80 + 1.00 * dev;
    wt[Domain::Communications.index()] = 0.80 + 0.70 * dev;
    wt[Domain::Energy.index()] = 0.90
        + if n.oil_mbd > 0.5 { 0.50 } else { 0.70 * oil_stress };
    wt[Domain::Materials.index()] = 0.90 + 2.00 * invest;
    wt[Domain::Aerospace.index()] =
        0.50 + 6.00 * n.mil_spend_gdp + if at_war { 1.00 } else { 0.0 };
    wt[Domain::Biotech.index()] = 0.60 + 1.00 * dev;
    wt[Domain::Transport.index()] = 0.90 + 0.40 * dev;
    wt[Domain::Agriculture.index()] = 0.70 + 1.20 * (1.0 - dev);
    let total: f64 = wt.iter().sum();
    for x in wt.iter_mut() {
        *x /= total;
    }
    wt
}

/// How much of what the world already knows this nation can actually take up.
/// Capacity, contact and openness — a closed, sanctioned, undercapitalised state
/// sits next to the frontier and cannot reach it.
fn absorptive_capacity(w: &WorldState, n: &Nation, dev: f64) -> f64 {
    let mut sum = 0.0;
    let mut count = 0.0;
    for other in w.nations.iter().filter(|o| o.alive && o.id != n.id) {
        sum += w.relation(n.id, other.id);
        count += 1.0;
    }
    let openness = if count > 0.0 {
        ((sum / count) + 40.0) / 90.0
    } else {
        0.5
    }
    .clamp(0.0, 1.0);

    let mut a = 0.30 + 0.40 * dev + 0.25 * openness + n.tech.bonus.diffusion_speed_eff();
    if n.system == EconomySystem::Command {
        a -= 0.15;
    }
    a -= 0.04 * w.sanctioned_by_count(n.id) as f64;
    a.clamp(0.05, 1.20)
}

/// What one technology costs this nation right now. The leader pays the whole
/// bill — the dead ends, the prototypes, the decade of not knowing whether it
/// works. Everyone after pays for a copy, and the copy collapses in price as the
/// thing becomes ordinary: the textbooks are written, the machine tools are for
/// sale, and the engineers who built it will take a consulting fee. That is why
/// the discount is convex in reach rather than proportional to it. Capacity
/// still gates the whole thing — a state nobody trades with, or that has nobody
/// trained to read the textbook, sits next to a cheap technology and cannot buy
/// it.
fn effective_cost(
    def: &TechDef,
    adopter_share: f64,
    absorb: f64,
    scale: f64,
    bonus: &TechBonuses,
) -> f64 {
    let capacity = (absorb / 1.20).clamp(0.0, 1.0);
    let share = adopter_share.clamp(0.0, 1.0);
    // Something the whole world already runs on has stopped being knowledge
    // anyone has to acquire and become a thing in a textbook, and the textbook
    // does not care how poor the reader is. Without this term absorptive
    // capacity gated everything, and a small closed economy could not pick up
    // even universal technology: Vietnam finished a thirty-year run knowing
    // nothing at all. The cube keeps it worth almost nothing until a technology
    // is genuinely everywhere, so it never cheapens the frontier.
    let textbook = 0.35 * share.powi(3);
    let reach = (share.powf(0.70) * (0.45 + 0.50 * capacity) + textbook).clamp(0.0, 0.98);
    let copy = (1.0 - reach).clamp(0.0, 1.0).powi(5);
    let own = (1.0 - bonus.cost_reduction_for(def.domain)).clamp(0.35, 1.0);
    // However ordinary a technology becomes, somebody still has to build it,
    // and the bill for building it is the size of the country that is building
    // it. That floor is what lets a small poor state pick up the ordinary
    // things without ever putting the frontier within reach.
    //
    // What it costs to build, though, depends on what is being built. Frontier
    // plant is bespoke and the floor should say so; something the whole world
    // already manufactures is bought off a shelf, and holding both to the same
    // floor is what shut the smallest economies out of even commodity
    // technology — the floor bound long before the copying discount could bite.
    let build = 0.30 * (1.0 - 0.70 * share.powi(2));
    (def.cost * copy * own).max(def.cost * build * scale)
}

/// Pick a project for one domain. Deterministic: cheapest first, ties broken by
/// registry position. Cheapest-first is not a shortcut — it is what makes
/// followers converge, because diffusion is exactly what makes a technology
/// cheap, so the cheapest thing available to a follower is whatever the world
/// has already built.
fn pick_focus(
    domain: Domain,
    n: &Nation,
    year: i32,
    weight: &[f64],
    world_weight: f64,
    absorb: f64,
    scale: f64,
) -> Option<u16> {
    let reg = registry();
    let pre = prereq_table();
    let mut best: Option<(f64, u16)> = None;
    let mut fallback: Option<(f64, u16)> = None;
    for (i, def) in reg.iter().enumerate() {
        if def.domain != domain {
            continue;
        }
        let idx = i as u16;
        if n.tech.knows_index(idx) {
            continue;
        }
        if !pre[i].iter().all(|p| n.tech.knows_index(*p)) {
            continue;
        }
        // The nation does not know this yet, so every point of weight on it
        // belongs to somebody else.
        let share = if world_weight > 0.0 {
            (weight[i] / world_weight).clamp(0.0, 1.0)
        } else {
            0.0
        };
        let cost = effective_cost(def, share, absorb, scale, &n.tech.bonus);
        // Work up to two years ahead of a floor; further ahead than that and a
        // nation is holding a whole branch hostage to one distant project.
        if def.earliest_year <= year + 2 {
            if best.map_or(true, |(c, _)| cost < c) {
                best = Some((cost, idx));
            }
        } else if fallback.map_or(true, |(c, _)| cost < c) {
            fallback = Some((cost, idx));
        }
    }
    best.or(fallback).map(|(_, i)| i)
}

/// One month of research, diffusion and effect for every living nation.
pub fn tick(w: &mut WorldState) {
    let reg = registry();

    // A snapshot of who knows what, weighted by the size of the economy that
    // knows it and by how freely that economy publishes. Taken once, so no
    // nation's turn order can matter.
    let mut weight = vec![0.0f64; reg.len()];
    let mut world_weight = 0.0;
    for n in w.nations.iter().filter(|n| n.alive) {
        let ww = n.gdp.max(0.0) * (1.0 + n.tech.bonus.diffusion_emission_eff());
        world_weight += ww;
        for t in &n.tech.known {
            if let Some(slot) = weight.get_mut(*t as usize) {
                *slot += ww;
            }
        }
    }
    let mut claimed = vec![false; reg.len()];
    let world_gdp: f64 = w.nations.iter().filter(|n| n.alive).map(|n| n.gdp.max(0.0)).sum();

    // The technology the world economy on average operates with, and the best
    // anyone has. Both are read off the state at the top of the tick, so what
    // one nation is scored against never depends on who was ticked first.
    let mut ref_weighted = 0.0;
    let mut frontier_known = 0.0f64;
    for n in w.nations.iter().filter(|n| n.alive) {
        ref_weighted += saturated_tech_tfp(n) * n.gdp.max(0.0);
        frontier_known = frontier_known.max(n.tech.count() as f64);
    }
    let reference = if world_gdp > 0.0 { ref_weighted / world_gdp } else { 0.0 };

    let ids: Vec<NationId> = w.nations.iter().filter(|n| n.alive).map(|n| n.id).collect();
    let year = w.year;
    let mut news: Vec<String> = vec![];

    for id in ids {
        let dev = development(w.nation(id));
        let absorb = absorptive_capacity(w, w.nation(id), dev);
        // Square root of the share of world output: what it costs this nation to
        // actually build a thing, rather than to work out that it can be built.
        // What it costs this nation to build a thing, rather than to work out
        // that it can be built, as a share of world output.
        //
        // Kept as the square root, so size still helps and helps less the bigger
        // you are. Holding it linear instead removes the size term entirely, and
        // with the budget linear in output too, affordability stops depending on
        // anything except development — China then absorbs at Japan's rate and
        // finishes a run level with it, which is not what happened. The tail is
        // dealt with in the cost floor instead.
        let scale = if world_gdp > 0.0 {
            (w.nation(id).gdp.max(0.0) / world_gdp).sqrt().clamp(0.005, 1.0)
        } else {
            1.0
        };
        let output = research_output(w, w.nation(id), dev);
        let weights = domain_weights(w, w.nation(id), dev);

        let mut firsts: Vec<u16> = vec![];
        {
            let n = w.nation_mut(id);
            n.tech.ensure_shape(n.tfp_trend);
            n.tech.research_total += output;
            let known_before = n.tech.count();

            for d in DOMAINS {
                let di = d.index();
                n.tech.progress[di] += output * weights[di];

                // Several cheap adoptions can land in one month — that is what
                // catching up looks like — but invention never comes in floods.
                for _ in 0..6 {
                    if n.tech.focus[di].is_none() {
                        let picked = pick_focus(d, n, year, &weight, world_weight, absorb, scale);
                        n.tech.focus[di] = picked;
                    }
                    let Some(t) = n.tech.focus[di] else { break };
                    let ti = t as usize;
                    if n.tech.knows_index(t) {
                        n.tech.focus[di] = None;
                        continue;
                    }
                    let def = &reg[ti];
                    let share = if world_weight > 0.0 {
                        (weight[ti] / world_weight).clamp(0.0, 1.0)
                    } else {
                        0.0
                    };
                    let cost = effective_cost(def, share, absorb, scale, &n.tech.bonus);
                    if n.tech.progress[di] < cost || year < def.earliest_year {
                        break;
                    }
                    n.tech.progress[di] -= cost;
                    n.tech.learn(t);
                    n.tech.focus[di] = None;
                    if weight[ti] <= 0.0 {
                        firsts.push(t);
                    }
                }
            }

            // Technologies fielded this month, annualised, folded into the
            // running rate. Done here rather than in `learn` so that a month
            // with no unlock pulls the rate down as surely as a month with one
            // pushes it up.
            let fielded = (n.tech.count() - known_before) as f64 * 12.0;
            n.tech.absorption_rate +=
                (fielded - n.tech.absorption_rate) * ABSORPTION_MEMORY;

            apply_bonuses(n, reference, frontier_known, absorb);
        }

        for t in firsts {
            let ti = t as usize;
            if !claimed[ti] {
                claimed[ti] = true;
                news.push(format!("{} is first to field {}.", id.name(), reg[ti].name));
            }
        }
    }

    for h in news {
        w.headline(h);
    }
}

/// Asymptotic annual productivity a saturated tree is worth. Approached, never
/// reached: a richer tree always buys something, but with sharply diminishing
/// returns, so two frontier economies with different trees do not land on the
/// same number the way a hard cap made them.
const TECH_CEILING: f64 = 0.022;
/// Annual growth one technology a year of sustained absorption is worth to a
/// nation far behind the frontier. This is the convergence engine: it replaced a
/// flat bonus for being poor, which double-counted the diffusion the tree
/// already models.
const ADOPTION_PER_TECH: f64 = 0.0085;
/// How fast the absorption rate follows what a nation is actually fielding.
/// Slow enough that one good year does not become a decade of growth, fast
/// enough that a country which stops absorbing stops being paid for it.
const ABSORPTION_MEMORY: f64 = 0.15;
/// The most annual growth adoption may ever be worth, whatever the tree says.
/// Korea and China sustained something close to this for a generation and no
/// one has ever done better for as long.
const ADOPTION_MAX: f64 = 0.045;
/// Adoption is worse than linear in the gap. The first nine tenths of a gap is
/// machinery and manuals that can be bought; the last tenth is tacit and is not
/// for sale. A nation that has copied everything copyable has to start
/// inventing, and most discover they cannot.
const TACIT: f64 = 1.35;

/// What a nation's known set is worth in annual productivity, before saturation.
/// Richer ore, better logistics and a healthier workforce all read through to it
/// at their own weights.
fn raw_tech_tfp(n: &Nation) -> f64 {
    let b = &n.tech.bonus;
    let invest = n.state_invest_gdp + n.priv_invest_gdp;
    b.productivity_eff()
        + b.resource_yield_eff() * 0.010
        + b.investment_efficiency_eff() * invest * 0.030
        + b.health_eff() * 0.0004
}

/// Saturating productivity value of a known set.
fn saturated_tech_tfp(n: &Nation) -> f64 {
    let raw = raw_tech_tfp(n).max(0.0);
    TECH_CEILING * (1.0 - (-raw / TECH_CEILING).exp())
}

/// Reapply the running totals to the nation every month. Productivity is a
/// stock and is simply recomputed; the rest are flows that arrive gradually,
/// because a technology that exists on paper still has to be built.
///
/// `reference` is the technology the world economy on average operates with, and
/// `frontier` the best anyone has. The tree is scored *against* the reference
/// rather than added on top of it, because `tfp_base` is a transcribed 1990
/// trend that already prices in the technology of 1990: adding to it counted the
/// same knowledge twice and inflated every developed economy by the same amount.
/// Scored this way the tree redistributes — a nation that out-researches the
/// world pulls ahead, one that falls behind loses ground — and in January 1990,
/// when nobody knows anything and the reference is zero, every nation sits
/// exactly on the trend `init.rs` transcribed for it.
fn apply_bonuses(n: &mut Nation, reference: f64, frontier_known: f64, _absorb: f64) {
    let b = &n.tech.bonus;
    let tech_tfp = saturated_tech_tfp(n);
    // How far behind there is still to be, counted in technologies rather than
    // in the productivity they are worth: saturation compresses the value of a
    // large known set into a narrow band, so measured that way a nation holding
    // a quarter of the frontier's technologies looked a third of the way behind.
    let gap = if frontier_known > 0.0 {
        ((frontier_known - n.tech.count() as f64) / frontier_known).clamp(0.0, 1.0)
    } else {
        0.0
    };
    // Catching up is something a nation does, not a position it occupies. The
    // first version of this paid out on the gap alone, which meant a country
    // that learned nothing for thirty years collected the convergence bonus for
    // all thirty of them — the flat bonus for being poor that this was supposed
    // to replace, wearing the tree's clothes. Vietnam finished a run knowing no
    // technology at all and growing 5.4% a year on the strength of it.
    //
    // So it is paid on absorption actually achieved. The gap still governs how
    // much each technology is worth when it lands: reorganising an economy
    // around something the rest of the world already runs on is worth more to
    // the nation twenty years behind than to the one at the frontier, which has
    // nothing left to copy and must invent instead.
    let adoption = ADOPTION_PER_TECH * n.tech.absorption_rate * gap.powf(TACIT);
    n.tfp_trend = n.tech.tfp_base + (tech_tfp - reference) + adoption.min(ADOPTION_MAX);

    // Recovery works its way into producing fields over years, and the barrels
    // it finds stay found even if the field changes hands in a war.
    if n.oil_mbd > 0.0 {
        let pending = b.oil_yield_eff() - n.tech.oil_yield_applied;
        if pending > 0.0 {
            let step = pending * 0.02;
            n.oil_mbd *= 1.0 + step;
            n.tech.oil_yield_applied += step;
        }
    }

    // Medicine keeps people alive; the pill and the city persuade them to have
    // fewer children. Both are annual rates spread over the month.
    let demographic =
        b.health_eff() * 0.002 + b.fertility_eff() * 0.002 + b.environment_eff() * 0.0006;
    n.population *= 1.0 + demographic / 12.0;

    // A steady nudge against the growth model's own mean reversion, so a well
    // served population settles a few points higher rather than running away.
    n.stability =
        (n.stability + (b.stability_eff() + b.environment_eff() * 0.30) * 0.005).clamp(0.0, 100.0);
}

/// How exposed a nation still is to the price of a barrel. Read by the growth
/// model for importers.
pub fn energy_exposure(n: &Nation) -> f64 {
    (1.0 - n.tech.bonus.energy_efficiency_eff()).clamp(0.20, 1.0)
}

/// Multiplier on the military strength a given budget sustains. Read by the
/// war model.
pub fn military_multiplier(n: &Nation) -> f64 {
    (1.0 + n.tech.bonus.military_efficiency_eff()).clamp(0.5, 4.0)
}

/// Flat strength a nation's arsenal carries regardless of budget. Read by the
/// war model alongside `military_multiplier`.
pub fn military_floor(n: &Nation) -> f64 {
    n.tech.bonus.military_strength_eff()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::init::world_1990;
    use crate::world::GameRules;

    /// A canary for a build that is not the build you think it is.
    ///
    /// `.cargo/config.toml` is tracked, so every worktree points at the same
    /// target directory as the main checkout, and OneDrive resets mtimes often
    /// enough that cargo will happily reuse a test binary compiled from another
    /// branch's source. That reads as a full green suite that never ran your
    /// code, and it is not hypothetical: it has already reported eight domain
    /// merges as verified when none of them had been compiled.
    ///
    /// Branches differ in how many technologies they define, so the count is a
    /// cheap thing to be wrong. If this fails and you did not add or remove a
    /// technology, you are looking at a stale binary — not a broken tree.
    #[test]
    fn the_registry_is_the_size_this_source_says_it_is() {
        assert_eq!(
            registry().len(),
            253,
            "registry has {} entries, not 253 — added a technology, or running a stale binary?",
            registry().len()
        );
    }

    #[test]
    fn tree_is_well_formed() {
        let reg = registry();
        assert!(!reg.is_empty(), "empty tech tree");

        // No duplicate ids.
        let mut ids: Vec<&str> = reg.iter().map(|t| t.id).collect();
        ids.sort_unstable();
        for pair in ids.windows(2) {
            assert_ne!(pair[0], pair[1], "duplicate tech id: {}", pair[0]);
        }

        for (i, t) in reg.iter().enumerate() {
            // Every prerequisite resolves.
            for p in &t.prereqs {
                let j = index_of(p)
                    .unwrap_or_else(|| panic!("{} lists unknown prerequisite {}", t.id, p));
                let q = &reg[j as usize];
                assert!(
                    q.era.rank() <= t.era.rank(),
                    "{} ({:?}) depends on later-era {} ({:?})",
                    t.id, t.era, q.id, q.era
                );
                assert!(
                    q.earliest_year <= t.earliest_year,
                    "{} ({}) depends on {} which is not possible until {}",
                    t.id, t.earliest_year, q.id, q.earliest_year
                );
                assert_ne!(p, &t.id, "{} is its own prerequisite", t.id);
            }
            // Era brackets are the shared calibration and are not optional.
            let (lo, hi) = t.era.window();
            assert!(
                t.earliest_year >= lo && t.earliest_year <= hi,
                "{}: year {} is outside {:?} ({}..{})",
                t.id, t.earliest_year, t.era, lo, hi
            );
            let (clo, chi) = t.era.cost_band();
            assert!(
                t.cost >= clo && t.cost <= chi,
                "{}: cost {} is outside the {:?} band ({}..{})",
                t.id, t.cost, t.era, clo, chi
            );
            assert!(!t.effects.is_empty(), "{} does nothing", t.id);
            assert!(t.id.contains('_'), "{} is not a prefixed snake_case id", t.id);
            let _ = i;
        }

        // No cycles: three-colour depth-first search over the whole registry,
        // walked in registry order so the result never depends on anything else.
        let pre = prereq_table();
        #[derive(Clone, Copy, PartialEq)]
        enum Mark {
            White,
            Grey,
            Black,
        }
        let mut mark = vec![Mark::White; reg.len()];
        for root in 0..reg.len() {
            if mark[root] != Mark::White {
                continue;
            }
            let mut stack: Vec<(usize, usize)> = vec![(root, 0)];
            mark[root] = Mark::Grey;
            while let Some((node, cursor)) = stack.pop() {
                if cursor < pre[node].len() {
                    stack.push((node, cursor + 1));
                    let next = pre[node][cursor] as usize;
                    match mark[next] {
                        Mark::Grey => panic!("prerequisite cycle through {}", reg[next].id),
                        Mark::White => {
                            mark[next] = Mark::Grey;
                            stack.push((next, 0));
                        }
                        Mark::Black => {}
                    }
                } else {
                    mark[node] = Mark::Black;
                }
            }
        }
    }

    #[test]
    fn diffusion_makes_the_follower_cheaper_than_the_leader() {
        let def = &registry()[0];
        let b = TechBonuses::default();
        let alone = effective_cost(def, 0.0, 0.7, 0.5, &b);
        let following = effective_cost(def, 0.6, 0.7, 0.02, &b);
        assert!(
            following < alone * 0.6,
            "copying is not meaningfully cheaper than inventing: {} vs {}",
            following, alone
        );
        // ...but capacity still gates it. A closed economy next to the frontier
        // pays far more for the same copy than an open one does.
        let closed = effective_cost(def, 0.6, 0.10, 0.02, &b);
        assert!(closed > following * 1.5, "absorptive capacity did nothing");
    }

    #[test]
    fn a_developed_economy_out_researches_a_poor_one() {
        // Nothing in the tree names the United States or Pakistan. What separates
        // them is that research is funded out of output, scaled by how much of
        // that output is already worth funding research with — so the gap opens
        // by itself, and closes by itself wherever diffusion can reach.
        let mut w = world_1990(GameRules::default());
        for _ in 0..360 {
            crate::tick_month(&mut w, &[]);
        }
        let rich = w.nation(NationId::USA);
        let poor = w.nation(NationId::Pakistan);
        assert!(
            rich.tech.count() > poor.tech.count(),
            "no technological gap opened: USA {} vs Pakistan {}",
            rich.tech.count(), poor.tech.count()
        );
        assert!(
            rich.tech.research_total > poor.tech.research_total * 5.0,
            "research effort did not track development: {:.0} vs {:.0}",
            rich.tech.research_total, poor.tech.research_total
        );
        // And the follower is not frozen out: what the world knows reaches it.
        assert!(
            poor.tech.count() > 0,
            "diffusion never reached a poor open economy"
        );
    }

    #[test]
    fn a_generous_tree_cannot_break_the_world() {
        // Eight authors who cannot see each other will collectively overspend
        // every effect budget. Saturation is what makes that a balance problem
        // rather than a correctness one, and this is the test that says so:
        // hand every nation a tree ten times richer than any author should
        // write, and the economy must still be an economy fifty years later.
        let mut w = world_1990(GameRules::default());
        for n in w.nations.iter_mut() {
            n.tech.bonus = TechBonuses {
                productivity: 1.00,
                research_rate: 40.0,
                military_strength: 900.0,
                military_efficiency: 20.0,
                oil_yield: 8.0,
                energy_efficiency: 9.0,
                resource_yield: 7.0,
                health: 30.0,
                fertility: -20.0,
                stability: 60.0,
                diffusion_speed: 8.0,
                diffusion_emission: 12.0,
                environment: 25.0,
                investment_efficiency: 9.0,
                cost_reduction: vec![0.9; DOMAIN_COUNT],
            };
        }
        for _ in 0..600 {
            crate::tick_month(&mut w, &[]);
            for n in w.nations.iter().filter(|n| n.alive) {
                assert!(n.gdp.is_finite() && n.gdp > 0.0, "{:?} gdp broke", n.id);
                assert!(n.population.is_finite() && n.population > 0.0);
                assert!(n.inflation.is_finite());
                assert!(n.debt_gdp.is_finite() && n.debt_gdp < 6.0);
                assert!((0.0..=100.0).contains(&n.stability));
                assert!(n.mil_strength.is_finite());
                assert!(n.tfp_trend < 0.10, "{:?} runaway tfp: {}", n.id, n.tfp_trend);
            }
            assert!(w.oil_price.is_finite() && w.oil_price > 0.0);
        }
    }
}




/// Calibration instrument, not a test. Prints what every nation's productivity
/// trend is actually made of after thirty years, so a change to the growth model
/// can be judged on the whole roster instead of on the four or five countries
/// whose numbers one happens to remember. Tuning this model on the G7 alone is
/// how it came to pay a convergence bonus to nations that had learned nothing.
///
///     cargo test -p spheres-sim --lib roster_decomposition -- --ignored --nocapture
#[cfg(test)]
mod diag {
    use super::*;
    use crate::init::world_1990;
    use crate::world::GameRules;

    #[test]
    #[ignore]
    fn roster_decomposition() {
        let mut w = world_1990(GameRules::default());
        for _ in 0..360 { crate::tick_month(&mut w, &[]); }
        let world_gdp: f64 = w.nations.iter().filter(|n| n.alive).map(|n| n.gdp.max(0.0)).sum();
        let mut refw = 0.0;
        let mut front = 0.0f64;
        for n in w.nations.iter().filter(|n| n.alive) {
            refw += saturated_tech_tfp(n) * n.gdp.max(0.0);
            front = front.max(n.tech.count() as f64);
        }
        let reference = refw / world_gdp;
        println!("\nDIAG reference={:.5} frontier_known={}", reference, front);
        // `cap` is absorptive capacity — what gates how fast anything is picked
        // up at all. `new/y` is technologies actually fielded per year, which is
        // what adoption is paid on.
        println!("{:<14}{:>6}{:>7}{:>7}{:>8}{:>9}{:>9}{:>9}{:>8}",
            "nation", "techs", "gap", "cap", "new/y", "adopt", "diff", "tfp", "grow%");
        let mut rows: Vec<_> = w.nations.iter().filter(|n| n.alive).collect();
        rows.sort_by(|a, b| b.gdp.partial_cmp(&a.gdp).unwrap());
        for n in rows {
            let sat = saturated_tech_tfp(n);
            let gap = ((front - n.tech.count() as f64) / front).clamp(0.0, 1.0);
            let dev = (n.gdp * 1000.0 / n.population / 24000.0).min(1.0);
            let absorb = absorptive_capacity(&w, n, dev);
            let adopt = (ADOPTION_PER_TECH * n.tech.absorption_rate * gap.powf(TACIT)).min(ADOPTION_MAX);
            println!("{:<14}{:>6}{:>7.3}{:>7.2}{:>8.2}{:>9.5}{:>9.5}{:>9.5}{:>8.2}",
                n.id.name(), n.tech.count(), gap, absorb, n.tech.absorption_rate, adopt,
                sat - reference, n.tfp_trend, n.growth_last * 100.0);
        }
    }
}
