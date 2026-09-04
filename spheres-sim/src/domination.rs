//! The universal campaign director.
//!
//! SPHERES has one ending: no sovereign rival remains outside the player's
//! hierarchy.  The agendas here are not alternate victories and do not pay
//! bonuses.  They are state-derived signposts that turn the existing economy,
//! resources, industry, statecraft and war systems toward that one ending.
//!
//! Everything is deterministic.  Offers are ranked from the current world,
//! carry stable string ids, and consume no RNG.  Progress is observed once per
//! settlement — each monthly settlement in legacy replay, each day in daily
//! play, since `tick` sits in the SYSTEMS table and that table runs whole on
//! every day (lib.rs `tick_day`); milestones latch, so the cadence changes
//! when a mark is seen, never whether.  Completing an agenda records a legacy
//! entry and deals three more cards; it never writes GDP, resources, standing,
//! technology or military strength.

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

use crate::world::{NationId, WorldState};

pub const VICTORY_NAME: &str = "World Domination";
pub const VICTORY_RULE: &str =
    "Every surviving sovereign government is formally subordinate to the player.";
pub const CLIENT_LEVERAGE_THRESHOLD: f64 = 0.10;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum AgendaArchetype {
    ConsolidateEconomicBase,
    SecureStrategicResources,
    BuildArsenal,
    EstablishRegionalSphere,
    SettleRivalConflict,
    SubjugateRival,
}

impl AgendaArchetype {
    pub fn key(self) -> &'static str {
        match self {
            Self::ConsolidateEconomicBase => "consolidate_economic_base",
            Self::SecureStrategicResources => "secure_strategic_resources",
            Self::BuildArsenal => "build_arsenal",
            Self::EstablishRegionalSphere => "establish_regional_sphere",
            Self::SettleRivalConflict => "settle_rival_conflict",
            Self::SubjugateRival => "subjugate_rival",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::ConsolidateEconomicBase => "Consolidate the Economic Base",
            Self::SecureStrategicResources => "Secure Strategic Resources",
            Self::BuildArsenal => "Build the Arsenal",
            Self::EstablishRegionalSphere => "Establish a Regional Sphere",
            Self::SettleRivalConflict => "Settle the Rival Conflict",
            Self::SubjugateRival => "Subjugate a Rival",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AgendaOffer {
    /// Stable across save/load and independent of display order.
    pub id: String,
    pub archetype: AgendaArchetype,
    pub title: String,
    pub brief: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<NationId>,
    pub baseline: f64,
    pub goal: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ActiveAgenda {
    pub offer: AgendaOffer,
    pub value: f64,
    /// 0..=1.  This is the only progress scale a surface needs.
    pub progress: f64,
    /// The 25%, 50%, 75% and 100% seals.  Once lit, a seal stays lit.
    pub milestones: [bool; 4],
    pub chosen_year: i32,
    pub chosen_month: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct LegacyAgenda {
    pub id: String,
    pub title: String,
    pub archetype: AgendaArchetype,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<NationId>,
    pub completed_year: i32,
    pub completed_month: u32,
}

/// A formal conquest relationship.  Unlike aid, a guarantee or friendly
/// trade, this says who is sovereign over whom.  The hierarchy can be more
/// than one level deep; all global checks walk it transitively.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Subordination {
    pub overlord: NationId,
    pub subject: NationId,
    pub since_year: i32,
    pub since_month: u32,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct GlobalDomination {
    /// A presentation score only.  Victory is the discrete rule below, never
    /// a threshold on this number.
    pub progress: f64,
    pub victory: bool,
    /// Player plus fully absorbed start-state seats.
    pub directly_controlled: u32,
    /// Living governments transitively below the player.
    pub subordinate_clients: u32,
    /// Exact, stable roster order.  Victory requires this to be empty.
    pub independent_rivals: Vec<NationId>,
    /// Human-readable blockers; never used to decide victory.
    pub incomplete_conditions: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Campaign {
    pub nation: NationId,
    pub generation: u32,
    pub offers: Vec<AgendaOffer>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chosen: Option<ActiveAgenda>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub legacy: Vec<LegacyAgenda>,
    pub global: GlobalDomination,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Domination {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub campaigns: Vec<Campaign>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subjects: Vec<Subordination>,
}

impl Domination {
    pub fn is_empty(&self) -> bool {
        self.campaigns.is_empty() && self.subjects.is_empty()
    }
}

pub fn campaign(w: &WorldState, nation: NationId) -> Option<&Campaign> {
    w.domination.campaigns.iter().find(|c| c.nation == nation)
}

/// A complete server-ready read model.  Before the first command or month it
/// is derived purely, so selecting a country can immediately draw its cards
/// without mutating the timeline.
pub fn view(w: &WorldState, nation: NationId) -> Campaign {
    if let Some(stored) = campaign(w, nation) {
        let mut out = stored.clone();
        out.global = status(w, nation);
        if out.chosen.is_none() {
            out.offers = if out.global.victory {
                vec![]
            } else {
                build_offers(w, nation, out.generation, &out.legacy)
            };
        }
        out
    } else {
        fresh_campaign(w, nation)
    }
}

pub fn offers(w: &WorldState, nation: NationId) -> Vec<AgendaOffer> {
    view(w, nation).offers
}

fn direct_overlord_in(rows: &[Subordination], subject: NationId) -> Option<NationId> {
    rows.iter()
        .find(|r| r.subject == subject)
        .map(|r| r.overlord)
}

pub fn direct_overlord(w: &WorldState, subject: NationId) -> Option<NationId> {
    direct_overlord_in(&w.domination.subjects, subject)
}

/// Whether `subject` sits anywhere below `overlord` in the formal hierarchy.
pub fn is_subordinate_client(w: &WorldState, overlord: NationId, subject: NationId) -> bool {
    if overlord == subject {
        return false;
    }
    let mut at = subject;
    let mut seen = BTreeSet::new();
    while seen.insert(at) {
        let Some(parent) = direct_overlord(w, at) else {
            return false;
        };
        if parent == overlord {
            return true;
        }
        at = parent;
    }
    false
}

/// Persist the result of a conquest too large to annex.  If a subject defeats
/// an ancestor, its old direct link is cut first, preventing a cycle.  Existing
/// subjects of the defeated government stay beneath it and therefore pass,
/// transitively, into the victor's hierarchy.
pub fn subjugate(w: &mut WorldState, overlord: NationId, subject: NationId) {
    if overlord == subject {
        return;
    }
    if is_subordinate_client(w, subject, overlord) {
        w.domination.subjects.retain(|r| r.subject != overlord);
    }
    w.domination.subjects.retain(|r| r.subject != subject);
    w.domination.subjects.push(Subordination {
        overlord,
        subject,
        since_year: w.year,
        since_month: w.month,
    });
    sort_subjects(&mut w.domination.subjects);
}

/// A state that disappears cannot remain a node in the hierarchy.  Its direct
/// subjects pass to the annexing power; its own subject row disappears.
pub fn absorb_subjects(w: &mut WorldState, winner: NationId, loser: NationId) {
    let (year, month) = (w.year, w.month);
    let winner_was_below_loser = is_subordinate_client(w, loser, winner);
    let old = std::mem::take(&mut w.domination.subjects);
    let mut next = Vec::with_capacity(old.len());
    for mut row in old {
        if row.subject == loser || (winner_was_below_loser && row.subject == winner) {
            continue;
        }
        if row.overlord == loser {
            row.overlord = winner;
            row.since_year = year;
            row.since_month = month;
        }
        if row.overlord != row.subject {
            next.push(row);
        }
    }
    w.domination.subjects = next;
    sort_subjects(&mut w.domination.subjects);
}

fn sort_subjects(rows: &mut Vec<Subordination>) {
    rows.sort_by_key(|r| (r.subject, r.overlord));
    rows.dedup_by_key(|r| r.subject);
}

/// Remove dead subjects and walk past dead overlords to their nearest living
/// ancestor.  If there is none, the orphan becomes independent.  This catches
/// dissolution and event deaths that do not pass through `war::conquer`.
fn normalize_subjects(w: &mut WorldState) {
    let old = w.domination.subjects.clone();
    let mut next = vec![];
    for row in &old {
        if !w.nation_opt(row.subject).is_some_and(|n| n.alive) {
            continue;
        }
        let mut parent = row.overlord;
        let mut seen = BTreeSet::from([row.subject]);
        loop {
            if !seen.insert(parent) {
                break;
            }
            if w.nation_opt(parent).is_some_and(|n| n.alive) {
                next.push(Subordination {
                    overlord: parent,
                    subject: row.subject,
                    since_year: row.since_year,
                    since_month: row.since_month,
                });
                break;
            }
            let Some(up) = direct_overlord_in(&old, parent) else {
                break;
            };
            parent = up;
        }
    }
    sort_subjects(&mut next);
    w.domination.subjects = next;
}

pub fn status(w: &WorldState, nation: NationId) -> GlobalDomination {
    let player_alive = w.nation_opt(nation).is_some_and(|n| n.alive);
    let living: Vec<NationId> = w.nations.iter().filter(|n| n.alive).map(|n| n.id).collect();
    let subjects: BTreeSet<NationId> = living
        .iter()
        .copied()
        .filter(|id| is_subordinate_client(w, nation, *id))
        .collect();
    let independent_rivals: Vec<NationId> = living
        .iter()
        .copied()
        .filter(|id| *id != nation && !subjects.contains(id))
        .collect();
    let victory = player_alive && independent_rivals.is_empty();

    let dominated = |id: NationId| id == nation || subjects.contains(&id);
    let total_population: f64 = living
        .iter()
        .map(|id| w.nation(*id).population.max(0.0))
        .sum();
    let held_population: f64 = living
        .iter()
        .filter(|id| dominated(**id))
        .map(|id| w.nation(*id).population.max(0.0))
        .sum();
    let total_gdp: f64 = living.iter().map(|id| w.nation(*id).gdp.max(0.0)).sum();
    let held_gdp: f64 = living
        .iter()
        .filter(|id| dominated(**id))
        .map(|id| w.nation(*id).gdp.max(0.0))
        .sum();
    let total_ground: f64 = w.district_population.values().map(|p| p.max(0.0)).sum();
    let held_ground: f64 = w
        .district_population
        .iter()
        .filter(|(district, _)| w.districts.get(*district).is_some_and(|id| dominated(*id)))
        .map(|(_, p)| p.max(0.0))
        .sum();
    let sovereign_share = if living.is_empty() {
        0.0
    } else {
        (living.len() - independent_rivals.len()) as f64 / living.len() as f64
    };
    let share = |held: f64, total: f64| if total > 0.0 { held / total } else { 0.0 };
    let raw_progress = 0.35 * sovereign_share
        + 0.25 * share(held_population, total_population)
        + 0.25 * share(held_gdp, total_gdp)
        + 0.15 * share(held_ground, total_ground);
    let progress = if victory {
        1.0
    } else {
        raw_progress.clamp(0.0, 0.999)
    };

    let mut absorbed = 0u32;
    for former in crate::nations::start_nations().iter().copied() {
        if former == nation || w.nation_opt(former).is_some_and(|n| n.alive) {
            continue;
        }
        let mut had_ground = false;
        let mut all_held = true;
        for district in w.district_population.keys() {
            if crate::districts::start_owner_1990(district) == Some(former) {
                had_ground = true;
                all_held &= w.districts.get(district) == Some(&nation);
            }
        }
        if had_ground && all_held {
            absorbed += 1;
        }
    }

    let mut incomplete_conditions = vec![];
    if !player_alive {
        incomplete_conditions.push(format!(
            "{} no longer has a sovereign government.",
            nation.name()
        ));
    }
    if !independent_rivals.is_empty() {
        incomplete_conditions.push(format!(
            "{} sovereign rival{} remain outside the hierarchy.",
            independent_rivals.len(),
            if independent_rivals.len() == 1 {
                ""
            } else {
                "s"
            }
        ));
    }

    GlobalDomination {
        progress,
        victory,
        directly_controlled: if player_alive { 1 + absorbed } else { absorbed },
        subordinate_clients: subjects.len() as u32,
        independent_rivals,
        incomplete_conditions,
    }
}

fn active_conflict_target(w: &WorldState, nation: NationId) -> Option<NationId> {
    let mut targets = vec![];
    for conflict in &w.conflicts {
        let Some(side) = conflict.side_of(nation) else {
            continue;
        };
        let other = if side {
            &conflict.side_b
        } else {
            &conflict.side_a
        };
        targets.extend(other.iter().copied().filter(|id| {
            w.nation_opt(*id).is_some_and(|n| n.alive) && !is_subordinate_client(w, nation, *id)
        }));
    }
    targets.sort();
    targets.dedup();
    targets.into_iter().next()
}

fn target_rival(w: &WorldState, nation: NationId) -> Option<NationId> {
    if let Some(target) = active_conflict_target(w, nation) {
        return Some(target);
    }
    let own_power = military_industrial_power(w, nation).max(1.0);
    let mut candidates: Vec<(NationId, f64)> = status(w, nation)
        .independent_rivals
        .into_iter()
        .map(|id| {
            let relation_pressure = ((-w.relation(nation, id)).max(0.0)) * 0.4;
            let border = if crate::nations::adjacent(nation, id) {
                45.0
            } else {
                0.0
            };
            let threat = (military_industrial_power(w, id) / own_power).min(4.0) * 12.0;
            let output = (w.nation(id).gdp / w.nation(nation).gdp.max(0.1)).min(4.0) * 5.0;
            (id, border + relation_pressure + threat + output)
        })
        .collect();
    candidates.sort_by(|a, b| b.1.total_cmp(&a.1).then(a.0.cmp(&b.0)));
    candidates.first().map(|(id, _)| *id)
}

const STRATEGIC: [crate::resources::Commodity; 6] = [
    crate::resources::Commodity::Oil,
    crate::resources::Commodity::Gas,
    crate::resources::Commodity::Iron,
    crate::resources::Commodity::Copper,
    crate::resources::Commodity::RareEarths,
    crate::resources::Commodity::Uranium,
];

fn strategic_coverage(w: &WorldState, nation: NationId) -> f64 {
    let have = crate::resources::have(w);
    let row = have.flow.get(nation.index()).copied().unwrap_or([0.0; 12]);
    STRATEGIC
        .iter()
        .filter(|c| row[c.idx()] > 0.0 || crate::resources::stock_quantity(w, nation, **c) > 0.0)
        .count() as f64
}

fn military_industrial_power(w: &WorldState, nation: NationId) -> f64 {
    let Some(n) = w.nation_opt(nation) else {
        return 0.0;
    };
    let capabilities: f64 = w
        .production
        .provinces
        .iter()
        .filter(|p| w.districts.get(&p.district) == Some(&nation))
        .map(|p| p.arms_plants as f64 * 5.0 + p.civilian_industry as f64 * 2.0)
        .sum();
    n.mil_strength.max(0.0) + crate::arsenal::book_value(n).max(0.0) * 0.05 + capabilities
}

fn client_influence(w: &WorldState, nation: NationId) -> f64 {
    w.nations
        .iter()
        .filter(|n| n.alive && n.id != nation)
        .map(|target| {
            if is_subordinate_client(w, nation, target.id) {
                return 1.0;
            }
            let economic = w
                .aid_flow(nation, target.id, crate::world::AidKind::Economic)
                .is_some() as u8 as f64
                * 0.20;
            let arms = w
                .aid_flow(nation, target.id, crate::world::AidKind::Arms)
                .is_some() as u8 as f64
                * 0.20;
            let pact = w.allied(nation, target.id) as u8 as f64 * 0.15;
            let relations = (w.relation(nation, target.id).max(0.0) / 75.0).min(1.0) * 0.15;
            let dependency =
                (w.trade_dependency(target.id, nation) / CLIENT_LEVERAGE_THRESHOLD).min(1.0) * 0.30;
            (economic + arms + pact + relations + dependency).min(1.0)
        })
        .sum()
}

fn in_conflict(w: &WorldState, a: NationId, b: NationId) -> bool {
    w.conflicts.iter().any(|c| c.involves(a) && c.involves(b))
}

fn metric(w: &WorldState, nation: NationId, offer: &AgendaOffer) -> f64 {
    match offer.archetype {
        AgendaArchetype::ConsolidateEconomicBase => {
            w.nation_opt(nation).map_or(0.0, |n| n.gdp.max(0.0))
        }
        AgendaArchetype::SecureStrategicResources => strategic_coverage(w, nation),
        AgendaArchetype::BuildArsenal => military_industrial_power(w, nation),
        AgendaArchetype::EstablishRegionalSphere => client_influence(w, nation),
        AgendaArchetype::SettleRivalConflict => offer.target.map_or(0.0, |target| {
            (!w.nation_opt(target).is_some_and(|n| n.alive)
                || is_subordinate_client(w, nation, target)
                || !in_conflict(w, nation, target)) as u8 as f64
        }),
        AgendaArchetype::SubjugateRival => offer.target.map_or(0.0, |target| {
            (!w.nation_opt(target).is_some_and(|n| n.alive)
                || is_subordinate_client(w, nation, target)) as u8 as f64
        }),
    }
}

fn offer_id(
    nation: NationId,
    generation: u32,
    archetype: AgendaArchetype,
    target: Option<NationId>,
) -> String {
    format!(
        "domination:{}:{}:{}:{}",
        nation.code(),
        generation,
        archetype.key(),
        target.map_or("world", NationId::code)
    )
}

fn make_offer(
    w: &WorldState,
    nation: NationId,
    generation: u32,
    archetype: AgendaArchetype,
    target: Option<NationId>,
) -> AgendaOffer {
    let (title, brief, baseline, goal) = match archetype {
        AgendaArchetype::ConsolidateEconomicBase => {
            let base = w.nation(nation).gdp.max(0.0);
            (
                "Raise the Golden Engine".to_string(),
                "Expand national output by 10%. Every future campaign rides on this engine."
                    .to_string(),
                base,
                base + (base * 0.10).max(1.0),
            )
        }
        AgendaArchetype::SecureStrategicResources => {
            let base = strategic_coverage(w, nation);
            let goal = (base + 2.0).min(STRATEGIC.len() as f64);
            (
                "Gather the Six Keys".to_string(),
                "Control production or stocks of two more strategic lines: oil, gas, iron, copper, rare earths or uranium."
                    .to_string(),
                base,
                goal,
            )
        }
        AgendaArchetype::BuildArsenal => {
            let base = military_industrial_power(w, nation);
            (
                "Light the National Forge".to_string(),
                "Raise the combined strength of the armed forces, arsenal and war industry by 12%."
                    .to_string(),
                base,
                base + (base * 0.12).max(5.0),
            )
        }
        AgendaArchetype::EstablishRegionalSphere => {
            let base = client_influence(w, nation);
            (
                "Draw a New Constellation".to_string(),
                "Add one full measure of client influence through patronage, alliance, dependency or formal subordination."
                    .to_string(),
                base,
                base + 1.0,
            )
        }
        AgendaArchetype::SettleRivalConflict => {
            let name = target.map_or("the rival", NationId::name);
            (
                format!("Close the Iron Circle: {}", name),
                format!(
                    "End the active conflict with {}—by settlement, annexation or subordination—then turn outward again.",
                    name
                ),
                0.0,
                1.0,
            )
        }
        AgendaArchetype::SubjugateRival => {
            let name = target.map_or("a rival", NationId::name);
            (
                format!("Bring {} Into Orbit", name),
                format!(
                    "Remove {} as an independent rival by annexation or formal subordination.",
                    name
                ),
                0.0,
                1.0,
            )
        }
    };
    AgendaOffer {
        id: offer_id(nation, generation, archetype, target),
        archetype,
        title,
        brief,
        target,
        baseline,
        goal,
    }
}

fn repetition_penalty(legacy: &[LegacyAgenda], archetype: AgendaArchetype) -> f64 {
    legacy
        .iter()
        .rev()
        .take(3)
        .filter(|x| x.archetype == archetype)
        .count() as f64
        * 12.0
}

fn build_offers(
    w: &WorldState,
    nation: NationId,
    generation: u32,
    legacy: &[LegacyAgenda],
) -> Vec<AgendaOffer> {
    let global = status(w, nation);
    if global.victory || !w.nation_opt(nation).is_some_and(|n| n.alive) {
        return vec![];
    }
    let target = target_rival(w, nation);
    let active = active_conflict_target(w, nation);
    let direct_kind = if active.is_some() {
        AgendaArchetype::SettleRivalConflict
    } else {
        AgendaArchetype::SubjugateRival
    };
    let direct_target = active.or(target);

    let total_gdp: f64 = w
        .nations
        .iter()
        .filter(|n| n.alive)
        .map(|n| n.gdp.max(0.0))
        .sum();
    let own_gdp_share = w.nation(nation).gdp.max(0.0) / total_gdp.max(0.1);
    let missing_resources = STRATEGIC.len() as f64 - strategic_coverage(w, nation);
    let rival_power = target.map_or(0.0, |id| military_industrial_power(w, id));
    let own_power = military_industrial_power(w, nation).max(1.0);
    let mut enabling = vec![
        (
            AgendaArchetype::ConsolidateEconomicBase,
            39.0 + (1.0 - own_gdp_share).clamp(0.0, 1.0) * 18.0,
        ),
        (
            AgendaArchetype::BuildArsenal,
            43.0 + (rival_power / own_power).min(3.0) * 8.0,
        ),
        (
            AgendaArchetype::EstablishRegionalSphere,
            // Bounded linear pressure, not a platform logarithm: agenda order
            // is part of a deterministic save and must be bit-identical on
            // every supported machine.
            44.0 + (global.independent_rivals.len().min(50) as f64) / 10.0,
        ),
    ];
    if missing_resources > 0.0 {
        enabling.push((
            AgendaArchetype::SecureStrategicResources,
            46.0 + missing_resources * 3.0,
        ));
    }
    enabling.sort_by(|a, b| {
        let ascore = a.1 - repetition_penalty(legacy, a.0);
        let bscore = b.1 - repetition_penalty(legacy, b.0);
        bscore.total_cmp(&ascore).then(a.0.cmp(&b.0))
    });

    let mut out: Vec<AgendaOffer> = enabling
        .into_iter()
        .take(2)
        .map(|(kind, _)| make_offer(w, nation, generation, kind, None))
        .collect();
    if direct_target.is_some() {
        out.push(make_offer(
            w,
            nation,
            generation,
            direct_kind,
            direct_target,
        ));
    }
    // A living player with a rival always has a direct card.  This fallback is
    // defensive against a malformed world whose living roster vanished while
    // its victory status still read false.
    if out.len() < 3 {
        for kind in [
            AgendaArchetype::ConsolidateEconomicBase,
            AgendaArchetype::BuildArsenal,
            AgendaArchetype::EstablishRegionalSphere,
        ] {
            if !out.iter().any(|x| x.archetype == kind) {
                out.push(make_offer(w, nation, generation, kind, None));
                if out.len() == 3 {
                    break;
                }
            }
        }
    }
    out.truncate(3);
    out
}

fn fresh_campaign(w: &WorldState, nation: NationId) -> Campaign {
    let global = status(w, nation);
    Campaign {
        nation,
        generation: 0,
        offers: if global.victory {
            vec![]
        } else {
            build_offers(w, nation, 0, &[])
        },
        chosen: None,
        legacy: vec![],
        global,
    }
}

fn ensure_campaign(w: &mut WorldState, nation: NationId) {
    if campaign(w, nation).is_some() {
        return;
    }
    let c = fresh_campaign(w, nation);
    w.domination.campaigns.push(c);
    w.domination.campaigns.sort_by_key(|c| c.nation);
}

pub fn choose(w: &mut WorldState, nation: NationId, agenda: &str) -> Result<(), String> {
    if w.player != Some(nation) {
        return Err("Only the player may choose a domination agenda.".into());
    }
    if !w.nation_opt(nation).is_some_and(|n| n.alive) {
        return Err(format!(
            "{} no longer has a government to direct.",
            nation.name()
        ));
    }
    ensure_campaign(w, nation);
    let index = w
        .domination
        .campaigns
        .iter()
        .position(|c| c.nation == nation)
        .expect("ensured above");
    w.domination.campaigns[index].global = status(w, nation);
    // A same-day diplomatic command can change the state after the last
    // monthly refresh. Validate against what the world offers now, not a card
    // cached before that command.
    if w.domination.campaigns[index].chosen.is_none() {
        let current = build_offers(
            w,
            nation,
            w.domination.campaigns[index].generation,
            &w.domination.campaigns[index].legacy,
        );
        w.domination.campaigns[index].offers = current;
    }
    if w.domination.campaigns[index].global.victory {
        return Err("World domination is already complete.".into());
    }
    if w.domination.campaigns[index].chosen.is_some() {
        return Err("An agenda is already active.".into());
    }
    let offer = w.domination.campaigns[index]
        .offers
        .iter()
        .find(|o| o.id == agenda)
        .cloned()
        .ok_or_else(|| "That agenda is not among the three currently offered.".to_string())?;
    let value = metric(w, nation, &offer);
    let title = offer.title.clone();
    let (year, month) = (w.year, w.month);
    let c = &mut w.domination.campaigns[index];
    c.offers.clear();
    c.chosen = Some(ActiveAgenda {
        offer,
        value,
        progress: 0.0,
        milestones: [false; 4],
        chosen_year: year,
        chosen_month: month,
    });
    w.headline(format!(
        "{} sets its domination agenda: {}.",
        nation.name(),
        title
    ));
    Ok(())
}

/// Observe progress after every other monthly system has settled.
pub fn tick(w: &mut WorldState) {
    normalize_subjects(w);
    if let Some(player) = w
        .player
        .filter(|id| w.nation_opt(*id).is_some_and(|n| n.alive))
    {
        ensure_campaign(w, player);
    }
    let nations: Vec<NationId> = w.domination.campaigns.iter().map(|c| c.nation).collect();
    for nation in nations {
        let Some(index) = w
            .domination
            .campaigns
            .iter()
            .position(|c| c.nation == nation)
        else {
            continue;
        };
        let global = status(w, nation);
        let active = w.domination.campaigns[index].chosen.clone();
        w.domination.campaigns[index].global = global;
        let Some(active) = active else {
            w.domination.campaigns[index].offers = if w.domination.campaigns[index].global.victory {
                vec![]
            } else {
                build_offers(
                    w,
                    nation,
                    w.domination.campaigns[index].generation,
                    &w.domination.campaigns[index].legacy,
                )
            };
            continue;
        };
        let value = metric(w, nation, &active.offer);
        let span = active.offer.goal - active.offer.baseline;
        let progress = if span > 0.0 {
            ((value - active.offer.baseline) / span).clamp(0.0, 1.0)
        } else {
            1.0
        };
        let milestones = [0.25, 0.50, 0.75, 1.0].map(|mark| progress + 1e-12 >= mark);
        if progress < 1.0 {
            let current = w.domination.campaigns[index]
                .chosen
                .as_mut()
                .expect("cloned above");
            current.value = value;
            current.progress = progress;
            for (old, reached) in current.milestones.iter_mut().zip(milestones) {
                *old |= reached;
            }
            continue;
        }

        let legacy = LegacyAgenda {
            id: active.offer.id.clone(),
            title: active.offer.title.clone(),
            archetype: active.offer.archetype,
            target: active.offer.target,
            completed_year: w.year,
            completed_month: w.month,
        };
        let title = legacy.title.clone();
        {
            let c = &mut w.domination.campaigns[index];
            c.legacy.push(legacy);
            c.generation = c.generation.saturating_add(1);
            c.chosen = None;
        }
        let next = if w.domination.campaigns[index].global.victory {
            vec![]
        } else {
            build_offers(
                w,
                nation,
                w.domination.campaigns[index].generation,
                &w.domination.campaigns[index].legacy,
            )
        };
        w.domination.campaigns[index].offers = next;
        w.headline(format!(
            "{} completes its domination agenda: {}.",
            nation.name(),
            title
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{apply_command, init::world_1990, load, save, Command, GameRules};

    #[test]
    fn every_starting_nation_gets_three_deterministic_cards_and_one_direct_card() {
        let w = world_1990(GameRules::default());
        for nation in crate::nations::start_nations().iter().copied() {
            let a = view(&w, nation);
            let b = view(&w, nation);
            assert_eq!(a, b, "{} changed without the world changing", nation.name());
            assert_eq!(
                a.offers.len(),
                3,
                "{} did not receive three cards",
                nation.name()
            );
            assert!(a.offers.iter().any(|o| {
                matches!(
                    o.archetype,
                    AgendaArchetype::SubjugateRival | AgendaArchetype::SettleRivalConflict
                )
            }));
            let ids: BTreeSet<&str> = a.offers.iter().map(|o| o.id.as_str()).collect();
            assert_eq!(ids.len(), 3, "{} received duplicate ids", nation.name());
        }
    }

    #[test]
    fn choosing_is_free_and_observational() {
        let mut w = world_1990(GameRules::default());
        let nation = NationId::USA;
        w.player = Some(nation);
        let offer = view(&w, nation).offers[0].clone();
        let before = {
            let n = w.nation(nation);
            (
                n.gdp,
                n.population,
                n.mil_strength,
                n.political_capital,
                w.rng.state,
            )
        };
        apply_command(
            &mut w,
            &Command::ChooseDominationAgenda {
                nation,
                agenda: offer.id.clone(),
            },
        )
        .unwrap();
        let after = {
            let n = w.nation(nation);
            (
                n.gdp,
                n.population,
                n.mil_strength,
                n.political_capital,
                w.rng.state,
            )
        };
        assert_eq!(before, after);
        assert_eq!(
            campaign(&w, nation)
                .unwrap()
                .chosen
                .as_ref()
                .unwrap()
                .offer
                .id,
            offer.id
        );
    }

    #[test]
    fn milestones_and_legacy_observe_without_rewarding() {
        let mut w = world_1990(GameRules::default());
        let nation = NationId::USA;
        w.player = Some(nation);
        let offer = view(&w, nation)
            .offers
            .into_iter()
            .find(|o| o.archetype == AgendaArchetype::ConsolidateEconomicBase)
            .unwrap_or_else(|| {
                make_offer(
                    &w,
                    nation,
                    0,
                    AgendaArchetype::ConsolidateEconomicBase,
                    None,
                )
            });
        ensure_campaign(&mut w, nation);
        let ci = w
            .domination
            .campaigns
            .iter()
            .position(|c| c.nation == nation)
            .unwrap();
        w.domination.campaigns[ci].offers = vec![offer.clone()];
        choose(&mut w, nation, &offer.id).unwrap();
        w.nation_mut(nation).gdp = offer.goal;
        let before = {
            let n = w.nation(nation);
            (
                n.gdp,
                n.population,
                n.mil_strength,
                n.political_capital,
                w.rng.state,
            )
        };
        tick(&mut w);
        let after = {
            let n = w.nation(nation);
            (
                n.gdp,
                n.population,
                n.mil_strength,
                n.political_capital,
                w.rng.state,
            )
        };
        assert_eq!(before, after);
        let c = campaign(&w, nation).unwrap();
        assert!(c.chosen.is_none());
        assert_eq!(c.legacy.len(), 1);
        assert_eq!(c.generation, 1);
    }

    #[test]
    fn formal_hierarchy_is_transitive_and_is_the_only_victory() {
        let mut w = world_1990(GameRules::default());
        let player = NationId::USA;
        subjugate(&mut w, player, NationId::Canada);
        subjugate(&mut w, NationId::Canada, NationId::UK);
        assert!(is_subordinate_client(&w, player, NationId::UK));
        let roundtrip = load(&save(&w)).unwrap();
        assert!(is_subordinate_client(&roundtrip, player, NationId::UK));
        assert!(!status(&w, player).victory);
        let others: Vec<NationId> = w
            .nations
            .iter()
            .filter(|n| n.alive && n.id != player)
            .map(|n| n.id)
            .collect();
        for id in others {
            if !is_subordinate_client(&w, player, id) {
                subjugate(&mut w, player, id);
            }
        }
        let global = status(&w, player);
        assert!(global.victory);
        assert_eq!(global.progress, 1.0);
        assert!(global.independent_rivals.is_empty());
    }

    #[test]
    fn an_annexed_overlord_passes_its_subjects_to_the_winner() {
        let mut w = world_1990(GameRules::default());
        subjugate(&mut w, NationId::UK, NationId::Canada);
        absorb_subjects(&mut w, NationId::USA, NationId::UK);
        assert_eq!(direct_overlord(&w, NationId::Canada), Some(NationId::USA));
        assert_eq!(direct_overlord(&w, NationId::UK), None);
    }

    #[test]
    fn old_save_without_campaign_data_loads_and_replays_identically() {
        let mut w = world_1990(GameRules::default());
        w.player = Some(NationId::USA);
        let id = view(&w, NationId::USA).offers[0].id.clone();
        choose(&mut w, NationId::USA, &id).unwrap();
        let text = save(&w);
        let reloaded = load(&text).unwrap();
        assert_eq!(save(&reloaded), text);

        let mut value: serde_json::Value = serde_json::from_str(&text).unwrap();
        value.as_object_mut().unwrap().remove("domination");
        let old = load(&serde_json::to_string(&value).unwrap()).unwrap();
        assert!(old.domination.is_empty());
    }

    #[test]
    fn a_chosen_campaign_has_the_same_future_after_save_load() {
        let mut uninterrupted = world_1990(GameRules::default());
        uninterrupted.player = Some(NationId::USA);
        let id = view(&uninterrupted, NationId::USA).offers[0].id.clone();
        choose(&mut uninterrupted, NationId::USA, &id).unwrap();
        crate::tick_month(&mut uninterrupted, &[]);
        let mut reloaded = load(&save(&uninterrupted)).unwrap();
        for _ in 0..6 {
            crate::tick_month(&mut uninterrupted, &[]);
            crate::tick_month(&mut reloaded, &[]);
        }
        assert_eq!(save(&uninterrupted), save(&reloaded));
    }

    #[test]
    fn campaign_settlement_matches_the_daily_calendar() {
        let mut monthly = world_1990(GameRules::default());
        monthly.player = Some(NationId::USA);
        let id = view(&monthly, NationId::USA).offers[0].id.clone();
        choose(&mut monthly, NationId::USA, &id).unwrap();
        let mut daily = monthly.clone();
        crate::tick_month(&mut monthly, &[]);
        for _ in 0..31 {
            crate::tick_day(&mut daily, &[]);
        }
        assert_eq!(save(&monthly), save(&daily));
    }
}
