//! Geographic airfields and paid improvements. Locations reuse the campaign's
//! district centroids; capacities, prices, lead times and aircraft radii are
//! explicit game assumptions, not a historical airfield census.
use crate::{
    aviation::{AirTransit, Squadron},
    clock,
    equipment::DesignSpec,
    programs,
    world::{NationId, WorldState},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::OnceLock,
};

pub const AIRCRAFT_PER_CAPACITY_LEVEL: u32 = 12;
pub const MAX_LEVEL: u8 = 5;
pub const FOREIGN_SPONSORSHIP_NOTE: &str = "Model assumption: an existing host basing agreement permits financially sponsored airfield construction and improvements while that exact host controls the province and access remains open. The sponsoring government pays within its existing construction budget. Land ownership, the host's money and diplomatic authority do not change.";
const EPS: f64 = 1e-10;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum UpgradeTrack {
    Capacity,
    Support,
    Protection,
}
impl UpgradeTrack {
    pub fn max_level(self) -> u8 {
        match self {
            Self::Support => 1,
            _ => MAX_LEVEL,
        }
    }
    pub fn key(self) -> &'static str {
        match self {
            Self::Capacity => "capacity",
            Self::Support => "support",
            Self::Protection => "protection",
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Self::Capacity => "Capacity",
            Self::Support => "Support",
            Self::Protection => "Protection",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct BaseProject {
    pub id: u32,
    pub sponsor: NationId,
    pub track: UpgradeTrack,
    pub target_level: u8,
    pub total_cost_bn: f64,
    pub paid_bn: f64,
    pub total_days: u32,
    pub progress_days: f64,
    pub daily_budget_mn: f64,
    pub started_day: i32,
    pub last_paid_day: Option<i32>,
    pub paused: bool,
    pub completed_day: Option<i32>,
    pub cancelled_day: Option<i32>,
    pub reason: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Airbase {
    /// Stable id is the district id; one geographic airfield per province.
    pub id: String,
    pub district: String,
    pub name: String,
    pub sponsor: NationId,
    pub capacity_level: u8,
    pub support_level: u8,
    pub protection_level: u8,
    pub project: Option<BaseProject>,
    #[serde(default)]
    pub history: Vec<BaseProject>,
}
impl Airbase {
    pub fn capacity(&self) -> u32 {
        self.capacity_level as u32 * AIRCRAFT_PER_CAPACITY_LEVEL
    }
    pub fn level(&self, track: UpgradeTrack) -> u8 {
        match track {
            UpgradeTrack::Capacity => self.capacity_level,
            UpgradeTrack::Support => self.support_level,
            UpgradeTrack::Protection => self.protection_level,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AirbaseState {
    #[serde(default)]
    pub bases: Vec<Airbase>,
    #[serde(default)]
    pub next_project_id: u32,
    #[serde(default)]
    pub last_tick_day: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "action", rename_all = "snake_case", deny_unknown_fields)]
pub enum AirbaseCommand {
    Establish {
        district: String,
        name: String,
        daily_budget_mn: f64,
    },
    Upgrade {
        base: String,
        track: UpgradeTrack,
        daily_budget_mn: f64,
    },
    Rebase {
        squadron: u32,
        base: String,
    },
    PauseUpgrade {
        base: String,
        paused: bool,
    },
    CancelUpgrade {
        base: String,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct BaseLocation {
    pub district: String,
    pub name: String,
    pub lon: f64,
    pub lat: f64,
}

fn locations() -> &'static BTreeMap<String, BaseLocation> {
    static LOCATIONS: OnceLock<BTreeMap<String, BaseLocation>> = OnceLock::new();
    LOCATIONS.get_or_init(|| {
        #[derive(Deserialize)]
        struct Network {
            nodes: Vec<Node>,
        }
        #[derive(Deserialize)]
        struct Node {
            id: String,
            name: String,
            kind: String,
            lon: f64,
            lat: f64,
            district: Option<String>,
        }
        let network: Network = serde_json::from_str(crate::logistics::EMBEDDED_NETWORK)
            .expect("campaign logistics geography");
        network
            .nodes
            .into_iter()
            .filter(|n| n.kind == "district")
            .map(|n| {
                let district = n.district.unwrap_or(n.id);
                (
                    district.clone(),
                    BaseLocation {
                        district,
                        name: n.name,
                        lon: n.lon,
                        lat: n.lat,
                    },
                )
            })
            .collect()
    })
}

pub fn district_location(district: &str) -> Option<&'static BaseLocation> {
    locations().get(district)
}
pub fn base<'w>(w: &'w WorldState, id: &str) -> Option<&'w Airbase> {
    w.airbases.as_ref()?.bases.iter().find(|b| b.id == id)
}
pub fn base_location(w: &WorldState, id: &str) -> Option<&'static BaseLocation> {
    district_location(&base(w, id)?.district)
}

pub fn distance_km(a: &BaseLocation, b: &BaseLocation) -> f64 {
    let lat = (b.lat - a.lat).to_radians();
    let lon = (b.lon - a.lon).to_radians();
    let slat = geographic_sine(lat / 2.0);
    let slon = geographic_sine(lon / 2.0);
    let ca = geographic_sine(std::f64::consts::FRAC_PI_2 - a.lat.to_radians());
    let cb = geographic_sine(std::f64::consts::FRAC_PI_2 - b.lat.to_radians());
    let h = (slat * slat + ca * cb * slon * slon).clamp(0.0, 1.0);
    // asin(sqrt(h)) = 2 atan(sqrt(h)/(1+sqrt(1-h))). All arithmetic
    // follows IEEE operations, so border/range and transit decisions agree
    // between Windows and Linux without platform trigonometric libraries.
    let t = h.sqrt() / (1.0 + (1.0 - h).sqrt());
    let (offset, x) = if t > 0.41421356237309503 {
        (std::f64::consts::FRAC_PI_4, (t - 1.0) / (t + 1.0))
    } else {
        (0.0, t)
    };
    let mut term = x;
    let mut arc = x;
    for k in 1..32 {
        term *= -x * x;
        arc += term / (2 * k + 1) as f64;
    }
    6371.0 * 4.0 * (offset + arc)
}

// Map coordinates bound every input to [-pi, pi]. 24 terms comfortably
// resolve this interval to floating precision without range-reduction state.
fn geographic_sine(x: f64) -> f64 {
    let mut term = x;
    let mut sum = x;
    for k in 1..24 {
        term *= -x * x / ((2 * k) * (2 * k + 1)) as f64;
        sum += term;
    }
    sum
}

/// Combat radius, with return flight included in this modeled rating. Frozen
/// installed components, rather than newly researched components, determine it.
pub fn range_km(spec: &DesignSpec) -> f64 {
    let radius = match spec.platform.as_str() {
        "air_light_attack" => 700.0,
        "air_tactical_strike" => 1400.0,
        "air_fighter" => 900.0,
        _ => return 0.0,
    };
    radius
        * if spec
            .components
            .get("air_fuel")
            .is_some_and(|v| v == "air_fuel_extended")
        {
            1.35
        } else {
            1.0
        }
}

/// The same permission governs geographic basing and sponsored construction.
/// A host grant is never substituted with membership in the same theatre or
/// another government's consent; the province remains the host's property.
pub fn site_access_refusal(w: &WorldState, nation: NationId, district: &str) -> Option<String> {
    if !w.nation_opt(nation).is_some_and(|n| n.alive) {
        return Some("The sponsoring government no longer exists.".into());
    }
    let Some(&owner) = w.districts.get(district) else {
        return Some("This airbase has no current province owner. Choose another base.".into());
    };
    if let Some(reason) = crate::control::blocker(w, owner, district) {
        return Some(format!("{reason} Choose an accessible base."));
    }
    if !w.nation_opt(owner).is_some_and(|n| n.alive) {
        return Some("The airbase host no longer exists. Choose another base.".into());
    }
    if owner != nation {
        if w.is_sanctioning(owner, nation) || w.is_sanctioning(nation, owner) {
            return Some(format!(
                "Sanctions close {}'s airbase access. Restore access or choose a domestic base.",
                owner.name()
            ));
        }
        if !w
            .access
            .iter()
            .any(|a| a.host == owner && a.seeker == nation)
        {
            return Some(format!("{} has not granted this country basing access. Request access from this host or choose a domestic base.", owner.name()));
        }
    }
    None
}

fn access_refusal(w: &WorldState, nation: NationId, b: &Airbase) -> Option<String> {
    site_access_refusal(w, nation, &b.district)
}

pub fn base_blocker(w: &WorldState, nation: NationId, id: &str) -> Option<String> {
    let Some(b) = base(w, id) else {
        return Some("This airbase no longer exists. Choose a completed base.".into());
    };
    if b.capacity() == 0 {
        return Some(
            "This airbase is under construction. Fund completion or choose a completed base."
                .into(),
        );
    }
    access_refusal(w, nation, b)
}

/// Count every country's stationary and inbound aircraft once. A departing
/// squadron releases its origin and reserves only its selected destination.
pub fn occupied_capacity(w: &WorldState, id: &str) -> u32 {
    w.nations
        .iter()
        .filter_map(|n| n.aviation.as_ref())
        .flat_map(|a| &a.squadrons)
        .filter(|sq| {
            sq.transit
                .as_ref()
                .map(|t| t.to.as_str())
                .or(sq.base.as_deref())
                == Some(id)
        })
        .fold(0u32, |total, sq| total.saturating_add(sq.assigned))
}
pub fn free_capacity(w: &WorldState, id: &str) -> u32 {
    base(w, id).map_or(0, |b| b.capacity().saturating_sub(occupied_capacity(w, id)))
}

/// Call before changing a squadron's establishment. Its existing reservation is
/// removed from the comparison, so a size-preserving edit needs no extra slots.
pub fn assignment_refusal(
    w: &WorldState,
    nation: NationId,
    squadron: Option<u32>,
    id: &str,
    assigned: u32,
) -> Option<String> {
    if let Some(reason) = base_blocker(w, nation, id) {
        return Some(reason);
    }
    let own = squadron
        .and_then(|key| {
            w.nation(nation)
                .aviation
                .as_ref()?
                .squadrons
                .iter()
                .find(|s| s.id == key)
        })
        .filter(|s| {
            s.transit
                .as_ref()
                .map(|t| t.to.as_str())
                .or(s.base.as_deref())
                == Some(id)
        })
        .map_or(0, |s| s.assigned);
    let available = free_capacity(w, id).saturating_add(own);
    (assigned > available).then(|| format!("This base has {available} aircraft spaces available, including this squadron's current reservation. Choose a smaller squadron, another base, or complete a Capacity improvement."))
}

pub fn squadron_blocker(w: &WorldState, nation: NationId, sq: &Squadron) -> Option<String> {
    if let Some(t) = &sq.transit {
        let why = base_blocker(w, nation, &t.to)
            .map(|s| format!(" Destination unavailable: {s}"))
            .unwrap_or_default();
        return Some(format!("Aircraft are in transit until day {} and cannot fly missions.{why} Use Move to base to redirect if needed.", t.arrival_day));
    }
    let Some(id) = &sq.base else {
        return Some(
            "Choose Move to base and complete the transfer before launching a mission.".into(),
        );
    };
    if let Some(reason) = base_blocker(w, nation, id) {
        return Some(reason);
    }
    if occupied_capacity(w, id) > base(w, id).unwrap().capacity() {
        return Some(
            "This base is over capacity. Move a squadron or complete a Capacity improvement."
                .into(),
        );
    }
    if sq.service_days_left > 0 {
        return Some(format!(
            "Aircraft need {} day(s) of funded routine support before missions.",
            sq.service_days_left
        ));
    }
    None
}
pub fn support_level(w: &WorldState, sq: &Squadron) -> u8 {
    sq.base
        .as_deref()
        .and_then(|id| base(w, id))
        .map_or(0, |b| b.support_level)
}
pub fn protection_level(w: &WorldState, sq: &Squadron) -> u8 {
    sq.base
        .as_deref()
        .and_then(|id| base(w, id))
        .map_or(0, |b| b.protection_level)
}

pub fn target_in_range(
    w: &WorldState,
    nation: NationId,
    sq: &Squadron,
    district: &str,
) -> Result<f64, String> {
    if let Some(reason) = squadron_blocker(w, nation, sq) {
        return Err(reason);
    }
    let b = base(w, sq.base.as_deref().unwrap()).ok_or("Choose a completed airbase.")?;
    let target = district_location(district)
        .ok_or("This target has no mapped mission location. Choose a mapped province.")?;
    let owner = *w
        .districts
        .get(district)
        .ok_or("This target has no current owner. Choose an eligible target.")?;
    let theatre = crate::theatre::home_theatre(w, owner)
        .ok_or("This target has no supported operational theatre.")?;
    if !crate::theatre::has_access(w, nation, theatre) {
        return Err(format!("No operational access to {}. Request basing access in Diplomacy or choose an accessible target.", theatre.name()));
    }
    let host = *w
        .districts
        .get(&b.district)
        .ok_or("The base no longer has a host.")?;
    if host != nation
        && !w
            .access
            .iter()
            .any(|a| a.host == host && a.seeker == nation && a.theatre == theatre)
    {
        return Err(format!("{} has not granted this base permission for operations in {}. Request access for this theatre or choose another base.", host.name(), theatre.name()));
    }
    let revision = w
        .nation(nation)
        .equipment
        .as_ref()
        .and_then(|s| s.revisions.get(&sq.revision))
        .ok_or("The squadron's certified aircraft revision is unavailable.")?;
    let radius = range_km(&revision.spec);
    let distance = distance_km(
        base_location(w, &b.id).ok_or("The base has no mapped location.")?,
        target,
    );
    if distance > radius + EPS {
        return Err(format!("Target is {:.0} km away; these aircraft have a {:.0} km mission radius. Choose a closer base or a target within range.", distance, radius));
    }
    Ok(distance)
}

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct ImprovementQuote {
    pub track: UpgradeTrack,
    pub target_level: u8,
    pub total_cost_bn: f64,
    pub total_days: u32,
    pub daily_budget_mn: f64,
    pub earliest_funded_days: u32,
    pub capacity_after: u32,
    pub effect: String,
}
pub fn improvement_quote(
    existing: Option<&Airbase>,
    track: UpgradeTrack,
    daily_budget_mn: f64,
) -> ImprovementQuote {
    let current = existing.map_or(0, |b| b.level(track));
    let target_level = current.saturating_add(1);
    let foundation =
        existing.is_none_or(|b| b.capacity_level == 0) && track == UpgradeTrack::Capacity;
    let (price, days) = if foundation {
        (0.024, 12)
    } else {
        match track {
            UpgradeTrack::Capacity => (0.018, 9),
            UpgradeTrack::Support => (0.012, 6),
            UpgradeTrack::Protection => (0.015, 8),
        }
    };
    let total_cost_bn = price * target_level as f64;
    let total_days = days * target_level as u32;
    let daily_cost = total_cost_bn / total_days as f64;
    let cap = (daily_budget_mn / 1000.0).max(0.0).min(daily_cost);
    let earliest_funded_days = if cap > 0.0 {
        (total_cost_bn / cap).ceil().min(u32::MAX as f64) as u32
    } else {
        u32::MAX
    };
    let capacity_after = if track == UpgradeTrack::Capacity {
        target_level as u32 * AIRCRAFT_PER_CAPACITY_LEVEL
    } else {
        existing.map_or(0, |b| b.capacity())
    };
    ImprovementQuote { track, target_level, total_cost_bn, total_days, daily_budget_mn, earliest_funded_days, capacity_after,
        effect: match track { UpgradeTrack::Capacity => format!("{capacity_after} aircraft spaces after completion."), UpgradeTrack::Support => "Support level 1 after completion: each flown mission needs one funded post-flight service day instead of two. This track has one useful level in the current campaign.".into(), UpgradeTrack::Protection => format!("Protection level {target_level} after completion: modeled sortie aircraft losses reduced by {}% (8% per completed level, maximum 40%).", 8 * target_level.min(MAX_LEVEL)) } }
}

fn project_from_quote(id: u32, sponsor: NationId, day: i32, q: ImprovementQuote) -> BaseProject {
    BaseProject {
        id,
        sponsor,
        track: q.track,
        target_level: q.target_level,
        total_cost_bn: q.total_cost_bn,
        paid_bn: 0.0,
        total_days: q.total_days,
        progress_days: 0.0,
        daily_budget_mn: q.daily_budget_mn,
        started_day: day,
        last_paid_day: None,
        paused: false,
        completed_day: None,
        cancelled_day: None,
        reason: Some("Work begins on an eligible future construction funding day.".into()),
    }
}
fn budget_refusal(value: f64) -> Option<String> {
    (!value.is_finite() || value <= 0.0 || value > 1000.0).then(|| "Choose a positive daily construction cap up to $1,000m. The existing national construction budget remains the spending limit.".into())
}
fn directed_refusal(w: &WorldState, nation: NationId) -> Option<String> {
    if !clock::is_daily(w) {
        return Some("Airbase work and rebasing require the daily campaign.".into());
    }
    if !w.nation_opt(nation).is_some_and(|n| n.alive) {
        return Some("This government no longer exists.".into());
    }
    if !crate::economic_ai::may_direct(w, nation) {
        return Some("Only the directing government may issue these airbase orders.".into());
    }
    None
}

pub fn refusal(w: &WorldState, nation: NationId, command: &AirbaseCommand) -> Option<String> {
    if let Some(reason) = directed_refusal(w, nation) {
        return Some(reason);
    }
    match command {
        AirbaseCommand::Establish {
            district,
            name,
            daily_budget_mn,
        } => {
            if let Some(reason) = budget_refusal(*daily_budget_mn) {
                return Some(reason);
            }
            if name.trim().is_empty()
                || name.chars().count() > 80
                || name.chars().any(char::is_control)
            {
                return Some("Choose an airbase name between 1 and 80 characters without control characters.".into());
            }
            if district_location(district).is_none() {
                return Some("Choose a province with a mapped location.".into());
            }
            if let Some(reason) = site_access_refusal(w, nation, district) {
                return Some(reason);
            }
            if let Some(existing) = base(w, district) {
                if existing.capacity_level > 0 || existing.project.is_some() {
                    return Some(
                        "An airbase already occupies this province. Open its improvement controls."
                            .into(),
                    );
                }
            }
        }
        AirbaseCommand::Upgrade {
            base: id,
            track,
            daily_budget_mn,
        } => {
            if let Some(reason) = budget_refusal(*daily_budget_mn) {
                return Some(reason);
            }
            let Some(b) = base(w, id) else {
                return Some("This airbase no longer exists.".into());
            };
            if let Some(reason) = site_access_refusal(w, nation, &b.district) {
                return Some(reason);
            }
            if b.capacity_level == 0 {
                return Some("Complete the airbase foundation before adding improvements.".into());
            }
            if b.project.is_some() {
                return Some(
                    "This base already has paid work in progress. Finish or cancel it first."
                        .into(),
                );
            }
            if b.level(*track) >= track.max_level() {
                return Some("This improvement has reached its maximum level.".into());
            }
        }
        AirbaseCommand::PauseUpgrade { base: id, .. }
        | AirbaseCommand::CancelUpgrade { base: id } => {
            let Some(b) = base(w, id) else {
                return Some("This airbase no longer exists.".into());
            };
            let Some(project) = &b.project else {
                return Some("This airbase has no improvement in progress.".into());
            };
            if project.sponsor != nation {
                return Some("Only the government funding this project may change it.".into());
            }
        }
        AirbaseCommand::Rebase { squadron, base: id } => {
            let Some(sq) = w
                .nation(nation)
                .aviation
                .as_ref()
                .and_then(|a| a.squadrons.iter().find(|s| s.id == *squadron))
            else {
                return Some("Choose an existing squadron.".into());
            };
            if sq.assigned == 0 {
                return Some("Assign owned aircraft before moving this squadron.".into());
            }
            if crate::airmissions::busy(w, nation, *squadron) {
                return Some("This squadron has a queued mission. Cancel the mission before moving to another base.".into());
            }
            if sq.service_days_left > 0 {
                return Some("Complete this squadron's funded service before moving it.".into());
            }
            if sq.transit.is_none() && sq.base.as_deref() == Some(id) {
                return Some("This squadron is already at that base.".into());
            }
            if sq.transit.as_ref().is_some_and(|t| t.to == *id) {
                return Some("This squadron is already moving to that base.".into());
            }
            if let Some(reason) = assignment_refusal(w, nation, Some(*squadron), id, sq.assigned) {
                return Some(reason);
            }
            if let Err(reason) = travel_days(w, nation, sq, id) {
                return Some(reason);
            }
        }
    }
    None
}

fn travel_origin<'a>(
    w: &WorldState,
    nation: NationId,
    sq: &'a Squadron,
) -> Option<&'static BaseLocation> {
    sq.transit
        .as_ref()
        .and_then(|t| t.from.as_deref())
        .or(sq.base.as_deref())
        .and_then(district_location)
        .or_else(|| {
            w.districts
                .iter()
                .filter(|(_, owner)| **owner == nation)
                .find_map(|(id, _)| district_location(id))
        })
}
pub fn travel_days(
    w: &WorldState,
    nation: NationId,
    sq: &Squadron,
    destination: &str,
) -> Result<u32, String> {
    // Procurement delivers to the national pool without an invented airfield
    // location. Its first placement at an owned base takes one conversion day;
    // a foreign move must start from an established geographic home base.
    if sq.base.is_none() && sq.transit.is_none() {
        if base(w, destination).is_some_and(|b| w.districts.get(&b.district) == Some(&nation)) {
            return Ok(1);
        }
        return Err("Move newly assigned aircraft to a domestic base first, then review a geographic transfer abroad.".into());
    }
    let from = travel_origin(w, nation, sq)
        .ok_or("This country has no mapped aircraft staging location.")?;
    let to = base_location(w, destination).ok_or("Choose a mapped destination airbase.")?;
    let revision = w
        .nation(nation)
        .equipment
        .as_ref()
        .and_then(|s| s.revisions.get(&sq.revision))
        .ok_or("The squadron's aircraft revision is unavailable.")?;
    let ferry_range = range_km(&revision.spec) * 2.0;
    let distance = distance_km(from, to);
    if ferry_range <= 0.0 || distance > ferry_range + EPS {
        return Err(format!("Transfer is {:.0} km; these aircraft have {:.0} km ferry reach. Choose an intermediate accessible base.", distance, ferry_range));
    }
    Ok((distance / 1200.0).ceil().max(1.0) as u32)
}

pub fn apply(
    w: &mut WorldState,
    nation: NationId,
    command: &AirbaseCommand,
) -> Result<String, String> {
    if let Some(reason) = refusal(w, nation, command) {
        return Err(reason);
    }
    let day = clock::absolute_day(w);
    match command {
        AirbaseCommand::Establish {
            district,
            name,
            daily_budget_mn,
        } => {
            let q = improvement_quote(None, UpgradeTrack::Capacity, *daily_budget_mn);
            let state = w.airbases.get_or_insert_with(Default::default);
            let project_id = state
                .next_project_id
                .checked_add(1)
                .ok_or("Airbase project identifiers are exhausted.")?;
            state.next_project_id = project_id;
            let p = project_from_quote(project_id, nation, day, q);
            if let Some(b) = state.bases.iter_mut().find(|b| b.id == *district) {
                b.name = name.trim().into();
                b.sponsor = nation;
                b.project = Some(p);
            } else {
                state.bases.push(Airbase {
                    id: district.clone(),
                    district: district.clone(),
                    name: name.trim().into(),
                    sponsor: nation,
                    capacity_level: 0,
                    support_level: 0,
                    protection_level: 0,
                    project: Some(p),
                    history: vec![],
                });
            }
            Ok(format!("Airbase foundation ordered. Capacity remains zero until paid construction completes; existing national construction authority funds actual daily work. {FOREIGN_SPONSORSHIP_NOTE}"))
        }
        AirbaseCommand::Upgrade {
            base: id,
            track,
            daily_budget_mn,
        } => {
            let q = improvement_quote(base(w, id), *track, *daily_budget_mn);
            let state = w.airbases.as_mut().unwrap();
            let project_id = state
                .next_project_id
                .checked_add(1)
                .ok_or("Airbase project identifiers are exhausted.")?;
            state.next_project_id = project_id;
            state
                .bases
                .iter_mut()
                .find(|b| b.id == *id)
                .unwrap()
                .project = Some(project_from_quote(project_id, nation, day, q));
            Ok(format!("{} improvement ordered. Its effect begins only after funded completion. {FOREIGN_SPONSORSHIP_NOTE}", track.name()))
        }
        AirbaseCommand::PauseUpgrade { base: id, paused } => {
            let p = w
                .airbases
                .as_mut()
                .unwrap()
                .bases
                .iter_mut()
                .find(|b| b.id == *id)
                .unwrap()
                .project
                .as_mut()
                .unwrap();
            p.paused = *paused;
            p.reason = Some(
                if *paused {
                    "Paused by the funding government; completed work is preserved."
                } else {
                    "Resumed; work waits for an eligible construction funding day."
                }
                .into(),
            );
            Ok(p.reason.clone().unwrap())
        }
        AirbaseCommand::CancelUpgrade { base: id } => {
            let b = w
                .airbases
                .as_mut()
                .unwrap()
                .bases
                .iter_mut()
                .find(|b| b.id == *id)
                .unwrap();
            let mut p = b.project.take().unwrap();
            p.cancelled_day = Some(day);
            p.reason = Some("Cancelled; completed construction work is a sunk cost and grants no unfinished improvement.".into());
            b.history.push(p);
            Ok("Construction cancelled. Spent money remains recorded; no unfinished capacity, support or protection was granted.".into())
        }
        AirbaseCommand::Rebase { squadron, base: id } => {
            let sq = w
                .nation(nation)
                .aviation
                .as_ref()
                .unwrap()
                .squadrons
                .iter()
                .find(|s| s.id == *squadron)
                .unwrap();
            let days = travel_days(w, nation, sq, id)?;
            let from = sq
                .transit
                .as_ref()
                .and_then(|t| t.from.clone())
                .or_else(|| sq.base.clone());
            let returning_to_origin = sq.transit.is_some() && from.as_deref() == Some(id);
            let from = if returning_to_origin { None } else { from };
            let sq = w
                .nation_mut(nation)
                .aviation
                .as_mut()
                .unwrap()
                .squadrons
                .iter_mut()
                .find(|s| s.id == *squadron)
                .unwrap();
            if returning_to_origin {
                sq.base = None;
            }
            sq.transit = Some(AirTransit {
                from,
                to: id.clone(),
                departed_day: day,
                arrival_day: day + days as i32,
            });
            Ok(format!("Squadron moving to base; arrival in {days} day(s). Its aircraft cannot launch missions while in transit."))
        }
    }
}

/// Invoked after the existing daily construction queues and before the fiscal
/// closing entry. Each project consumes only the remaining shared cash envelope.
pub fn tick_day(w: &mut WorldState) -> Vec<String> {
    let day = clock::absolute_day(w);
    if !clock::is_daily(w)
        || w.airbases
            .as_ref()
            .is_none_or(|s| s.last_tick_day == Some(day))
    {
        return vec![];
    }
    let mut events = vec![];
    let ids: Vec<String> = w
        .airbases
        .as_ref()
        .unwrap()
        .bases
        .iter()
        .map(|b| b.id.clone())
        .collect();
    for id in ids {
        let Some(p) = base(w, &id).and_then(|b| b.project.clone()) else {
            continue;
        };
        if p.started_day >= day || p.last_paid_day == Some(day) {
            continue;
        }
        let district = base(w, &id).unwrap().district.clone();
        let mut reason = if p.paused {
            Some("Paused by the funding government.".into())
        } else {
            site_access_refusal(w, p.sponsor, &district)
        };
        let mut paid = 0.0;
        if reason.is_none() {
            let daily_price = p.total_cost_bn / p.total_days as f64;
            let available = programs::construction_available_bn(w, p.sponsor);
            let amount = daily_price
                .min(p.daily_budget_mn / 1000.0)
                .min(available)
                .min((p.total_cost_bn - p.paid_bn).max(0.0));
            if amount > 0.0 {
                match programs::spend_construction(w, p.sponsor, amount) {
                    Ok(()) => paid = amount,
                    Err(why) => reason = Some(why),
                }
            } else {
                reason = Some("Construction budget or capital funding is exhausted. Fund the existing national construction budget; paid work is preserved.".into());
            }
            if paid > 0.0 && paid + EPS < daily_price {
                reason = Some("Partial work within the project's daily cap and remaining national construction budget.".into());
            }
        }
        let b = w
            .airbases
            .as_mut()
            .unwrap()
            .bases
            .iter_mut()
            .find(|b| b.id == id)
            .unwrap();
        let work = b.project.as_mut().unwrap();
        work.reason = reason;
        if paid > 0.0 {
            work.paid_bn = (work.paid_bn + paid).min(work.total_cost_bn);
            work.progress_days = (work.paid_bn / work.total_cost_bn * work.total_days as f64)
                .min(work.total_days as f64);
            work.last_paid_day = Some(day);
        }
        if work.paid_bn + EPS >= work.total_cost_bn {
            let mut completed = b.project.take().unwrap();
            completed.completed_day = Some(day);
            completed.reason = None;
            match completed.track {
                UpgradeTrack::Capacity => b.capacity_level = completed.target_level,
                UpgradeTrack::Support => b.support_level = completed.target_level,
                UpgradeTrack::Protection => b.protection_level = completed.target_level,
            }
            events.push(format!(
                "[airbase] {} completed {} level {} on day {day}.",
                b.name,
                completed.track.name(),
                completed.target_level
            ));
            b.history.push(completed);
        }
    }
    let arrivals: Vec<_> = w
        .nations
        .iter()
        .flat_map(|n| {
            n.aviation.as_ref().into_iter().flat_map(move |a| {
                a.squadrons.iter().filter_map(move |s| {
                    s.transit
                        .as_ref()
                        .filter(|t| t.arrival_day <= day)
                        .map(|t| (n.id, s.id, t.to.clone()))
                })
            })
        })
        .collect();
    for (nation, squadron, destination) in arrivals {
        if base_blocker(w, nation, &destination).is_some()
            || occupied_capacity(w, &destination)
                > base(w, &destination).map_or(0, |b| b.capacity())
        {
            continue;
        }
        let sq = w
            .nation_mut(nation)
            .aviation
            .as_mut()
            .unwrap()
            .squadrons
            .iter_mut()
            .find(|s| s.id == squadron)
            .unwrap();
        sq.base = Some(destination);
        sq.transit = None;
        events.push(format!(
            "[airbase] {} arrived at its selected base on day {day}.",
            sq.name
        ));
    }
    w.airbases.as_mut().unwrap().last_tick_day = Some(day);
    events
}

pub fn validate(w: &WorldState) -> Result<(), String> {
    let Some(state) = &w.airbases else {
        if w.nations
            .iter()
            .filter_map(|n| n.aviation.as_ref())
            .flat_map(|s| &s.squadrons)
            .any(|s| s.base.is_some() || s.transit.is_some())
        {
            return Err("A squadron claims a base or transit without an airbase ledger.".into());
        }
        return Ok(());
    };
    let day = clock::absolute_day(w);
    if state.last_tick_day.is_some_and(|d| d > day) {
        return Err("Airbase settlement date is in the future.".into());
    }
    let mut bases = BTreeSet::new();
    let mut projects = BTreeSet::new();
    for b in &state.bases {
        if b.id != b.district
            || !bases.insert(&b.id)
            || district_location(&b.district).is_none()
            || b.name.trim().is_empty()
            || b.name.chars().count() > 80
            || b.name.chars().any(char::is_control)
            || w.nation_opt(b.sponsor).is_none()
            || [b.capacity_level, b.protection_level]
                .iter()
                .any(|v| *v > MAX_LEVEL)
            || b.support_level > UpgradeTrack::Support.max_level()
            || (b.capacity_level == 0 && (b.support_level > 0 || b.protection_level > 0))
        {
            return Err("Invalid geographic airbase record.".into());
        }
        if occupied_capacity(w, &b.id) > b.capacity() {
            return Err(
                "Saved stationary and inbound aircraft exceed completed airbase capacity.".into(),
            );
        }
        for p in b.project.iter().chain(&b.history) {
            if p.id == 0
                || p.id > state.next_project_id
                || !projects.insert(p.id)
                || w.nation_opt(p.sponsor).is_none()
                || p.target_level == 0
                || p.target_level > p.track.max_level()
                || !p.total_cost_bn.is_finite()
                || p.total_cost_bn <= 0.0
                || !p.paid_bn.is_finite()
                || p.paid_bn < 0.0
                || p.paid_bn > p.total_cost_bn + EPS
                || p.total_days == 0
                || !p.progress_days.is_finite()
                || p.progress_days < 0.0
                || p.progress_days > p.total_days as f64 + EPS
                || (p.progress_days - p.paid_bn / p.total_cost_bn * p.total_days as f64).abs()
                    > 1e-7
                || budget_refusal(p.daily_budget_mn).is_some()
                || p.started_day > day
                || p.last_paid_day
                    .is_some_and(|d| d <= p.started_day || d > day)
                || p.completed_day.is_some_and(|d| {
                    d <= p.started_day || d > day || p.paid_bn + EPS < p.total_cost_bn
                })
                || p.cancelled_day
                    .is_some_and(|d| d < p.started_day || d > day)
                || (p.completed_day.is_some() && p.cancelled_day.is_some())
            {
                return Err("Invalid saved airbase construction contract.".into());
            }
            let mut opening = b.clone();
            match p.track {
                UpgradeTrack::Capacity => opening.capacity_level = p.target_level - 1,
                UpgradeTrack::Support => opening.support_level = p.target_level - 1,
                UpgradeTrack::Protection => opening.protection_level = p.target_level - 1,
            }
            let terms = improvement_quote(Some(&opening), p.track, p.daily_budget_mn);
            if p.total_days != terms.total_days
                || (p.total_cost_bn - terms.total_cost_bn).abs() > 1e-12
            {
                return Err("Saved airbase construction price or lead time differs from its reviewed contract terms.".into());
            }
        }
        if b.project.as_ref().is_some_and(|p| {
            p.completed_day.is_some()
                || p.cancelled_day.is_some()
                || p.target_level != b.level(p.track) + 1
        }) {
            return Err("Active airbase work has an invalid target or completion state.".into());
        }
        if b.history
            .iter()
            .any(|p| p.completed_day.is_none() && p.cancelled_day.is_none())
        {
            return Err("Airbase construction history lacks a dated outcome.".into());
        }
    }
    for n in &w.nations {
        if let Some(a) = &n.aviation {
            for sq in &a.squadrons {
                if sq.base.as_ref().is_some_and(|id| base(w, id).is_none())
                    || sq.transit.as_ref().is_some_and(|t| {
                        base(w, &t.to).is_none()
                            || t.from.as_ref().is_some_and(|id| base(w, id).is_none())
                    })
                {
                    return Err("A squadron refers to an unknown airbase.".into());
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "airbases_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "airbase_distance_tests.rs"]
mod distance_tests;
