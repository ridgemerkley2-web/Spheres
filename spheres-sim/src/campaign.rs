//! Daily national-command campaigns. All figures below are modeled game rates,
//! not invented historical observations. National force and physical equipment
//! remain owned by operations/arsenal; the pools here account for their location
//! and cohesion. No random draws, hidden state in forecasts, or per-unit orders.
use crate::world::{Conflict, NationId, Objective, Roe, WorldState, INVASION_RUNG, SHOOTING_RUNG};
use crate::{arsenal, clock, control, districts, front, operations, theatre, war};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

const EPS: f64 = 1e-9;
const MAX_SECTORS: usize = 24;
const MAX_TRANSFER_DAYS: i32 = 90;

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Approach {
    Probe,
    #[default]
    Advance,
    Breakthrough,
    Hold,
    FightingWithdrawal,
}
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AirMission {
    #[default]
    None,
    Reconnaissance,
    Interception,
    GroundSupport,
    Interdiction,
}
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NavalMission {
    #[default]
    None,
    Transport,
    Escort,
    SeaDenial,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct OperationOrder {
    pub conflict: u32,
    pub nation: NationId,
    pub target: Option<String>,
    pub approach: Approach,
    pub reserve_bp: u16,
    pub air: AirMission,
    pub naval: NavalMission,
}
impl OperationOrder {
    pub fn automatic(c: &Conflict, nation: NationId) -> Self {
        let posture = c.posture_of(nation);
        let approach = match posture.map(|p| p.objective) {
            Some(Objective::Hold | Objective::Stabilise | Objective::Deny) => Approach::Hold,
            Some(Objective::Withdraw) => Approach::FightingWithdrawal,
            _ => Approach::Advance,
        };
        Self {
            conflict: c.id,
            nation,
            target: c.aim.as_ref().map(|a| a.district.clone()),
            approach,
            reserve_bp: 1500,
            air: AirMission::None,
            naval: NavalMission::None,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Reserve {
    pub strength: f64,
    pub cohesion: f64,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Sector {
    pub conflict: u32,
    pub nation: NationId,
    pub district: String,
    pub strength: f64,
    pub cohesion: f64,
    pub preparation: f64,
    pub isolated_days: u32,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Transfer {
    pub nation: NationId,
    pub conflict: Option<u32>,
    pub district: Option<String>,
    pub strength: f64,
    pub cohesion: f64,
    pub route: Vec<String>,
    pub arrives_day: i32,
    #[serde(default)]
    pub departed_day: i32,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct EnemyEstimate {
    pub low: f64,
    pub high: f64,
    pub confidence: f64,
    pub observed_day: i32,
    pub observed_label: String,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct BattleReport {
    pub day: i32,
    pub day_label: String,
    pub advanced: f64,
    pub retreated: f64,
    pub losses: f64,
    pub readiness: f64,
    pub summary: String,
    #[serde(default)]
    pub supply: f64,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct CampaignState {
    #[serde(default)]
    pub initialized: bool,
    /// Reserved at conflict creation, including wars closed before a tick.
    #[serde(default, skip_serializing_if = "is_zero_id")]
    pub conflict_id_high_water: u32,
    /// Explicit enrollment distinguishes saved ongoing wars from commands
    /// issued before the first operational tick. `Some(empty)` matters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub migration_conflicts: Option<BTreeSet<u32>>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub orders: BTreeMap<String, OperationOrder>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub sectors: BTreeMap<String, Sector>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub reserves: BTreeMap<NationId, Reserve>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transfers: Vec<Transfer>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub observations: BTreeMap<String, EnemyEstimate>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub reports: BTreeMap<String, BattleReport>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub ai_review: BTreeMap<String, i32>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub alerts: BTreeMap<String, i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_prepared: Option<i32>,
}
impl CampaignState {
    pub fn is_empty(&self) -> bool {
        !self.initialized
            && self.conflict_id_high_water == 0
            && self.migration_conflicts.is_none()
            && self.orders.is_empty()
            && self.sectors.is_empty()
            && self.transfers.is_empty()
    }
}
fn is_zero_id(id: &u32) -> bool {
    *id == 0
}

/// Upgrade already-enabled saves before transient fronts are cleaned up.
/// Every retained namespace contributes its old IDs; none may silently become
/// state for a later war involving the same nation and district.
pub(crate) fn conflict_id_high_water(w: &WorldState) -> u32 {
    let mut high = w.campaign.conflict_id_high_water;
    let mut observe = |id: u32| high = high.max(id);
    for c in &w.conflicts {
        observe(c.id);
    }
    if let Some(ids) = &w.campaign.migration_conflicts {
        for &id in ids {
            observe(id);
        }
    }
    for o in w.campaign.orders.values() {
        observe(o.conflict);
    }
    for s in w.campaign.sectors.values() {
        observe(s.conflict);
    }
    for t in &w.campaign.transfers {
        if let Some(id) = t.conflict {
            observe(id);
        }
    }
    for o in &w.campaign_peace.offers {
        observe(o.conflict);
    }
    for r in &w.campaign_peace.history {
        observe(r.offer.conflict);
    }
    for offer in w
        .agency
        .offers
        .iter()
        .chain(w.agency.history.iter().map(|r| &r.offer))
    {
        if let crate::agency::OfferKind::CallToArms { conflict, .. } = &offer.kind {
            observe(*conflict);
        }
    }
    for o in w.campaign_peace.occupation.values() {
        observe(o.conflict);
    }
    for b in w.campaign_supply.buffers.values() {
        observe(b.conflict);
    }
    for c in &w.campaign_supply.cargo {
        observe(c.conflict);
    }
    for key in w
        .campaign
        .observations
        .keys()
        .chain(w.campaign.reports.keys())
        .chain(w.campaign.ai_review.keys())
        .chain(w.campaign.alerts.keys())
        .chain(w.campaign_peace.aims.keys())
        .chain(w.campaign_peace.garrisons.keys())
        .chain(w.campaign_peace.last_proposal.keys())
        .chain(w.campaign_supply.deliveries.keys())
    {
        let suffix = key.strip_prefix("garrison:").unwrap_or(key);
        if let Some(id) = suffix.split(':').next().and_then(|s| s.parse::<u32>().ok()) {
            observe(id);
        }
    }
    for key in w.daily.counters.keys() {
        if let Some(id) = key
            .strip_prefix("war:")
            .and_then(|s| s.split(':').next())
            .and_then(|s| s.parse::<u32>().ok())
        {
            observe(id);
        }
    }
    high
}
pub fn enabled(w: &WorldState) -> bool {
    w.rules.daily_simulation && w.rules.military_operations && w.rules.operational_warfare == 1
}
/// Capture existing fronts when an application enables operational warfare.
/// Repeated enrollment and save/load cannot admit later declarations into the
/// one-time migration allowance. Headless callers may still migrate on tick.
pub fn enroll(w: &mut WorldState) {
    if enabled(w) {
        w.campaign.conflict_id_high_water = conflict_id_high_water(w);
    }
    if enabled(w) && !w.campaign.initialized && w.campaign.migration_conflicts.is_none() {
        w.campaign.migration_conflicts = Some(w.conflicts.iter().map(|c| c.id).collect());
    }
}
pub fn order_key(conflict: u32, nation: NationId) -> String {
    format!("{}:{}", conflict, nation.index())
}
pub fn sector_key(conflict: u32, nation: NationId, district: &str) -> String {
    format!("{}:{}:{}", conflict, nation.index(), district)
}
fn date_label(day: i32) -> String {
    let (y, m, d) = clock::date_from_day(day);
    format!("{y:04}-{m:02}-{d:02}")
}
fn order_for(w: &WorldState, c: &Conflict, nation: NationId) -> OperationOrder {
    let mut o = w
        .campaign
        .orders
        .get(&order_key(c.id, nation))
        .cloned()
        .unwrap_or_else(|| OperationOrder::automatic(c, nation));
    // A national withdrawal binds subordinate staff intent even when it was
    // selected automatically by the political red line after the last order.
    if c.posture_of(nation)
        .is_some_and(|b| b.objective == Objective::Withdraw)
    {
        o.approach = Approach::FightingWithdrawal;
    }
    o
}
pub fn order_refusal(w: &WorldState, o: &OperationOrder) -> Option<String> {
    if !enabled(w) {
        return Some("Daily operational warfare is not enabled.".into());
    }
    let Some(c) = w.conflict(o.conflict) else {
        return Some("This conflict has ended.".into());
    };
    if !w.nation_opt(o.nation).is_some_and(|n| n.alive)
        || c.posture_of(o.nation).is_none()
        || c.side_of(o.nation).is_none()
    {
        return Some("Only a living participant can issue this operation.".into());
    }
    if o.reserve_bp > 10_000 {
        return Some("Reserve allocation must be between 0 and 10000 basis points.".into());
    }
    if let Some(d) = &o.target {
        if !front::contested_set(w, c).k.contains_key(d) {
            return Some("Select a district in this conflict's operating area.".into());
        }
    }
    None
}
pub fn set_order(w: &mut WorldState, o: &OperationOrder) -> Result<(), String> {
    if let Some(e) = order_refusal(w, o) {
        return Err(e);
    }
    w.campaign
        .orders
        .insert(order_key(o.conflict, o.nation), o.clone());
    Ok(())
}

/// The one operation consumption quote used by national magazine settlement
/// and the player deployment view. Reserve orders do not fire a second army.
pub fn consumption_multiplier(w: &WorldState, conflict: u32, nation: NationId) -> f64 {
    if !enabled(w) {
        return 1.0;
    }
    let Some(c) = w.conflict(conflict) else {
        return 1.0;
    };
    let o = order_for(w, c, nation);
    let garrison = crate::campaign_peace::garrison_share(w, conflict, nation);
    let combat = (1.0 - o.reserve_bp as f64 / 10000.0 - garrison).clamp(0.0, 1.0);
    let intensity = match o.approach {
        Approach::Probe => 0.65,
        Approach::Advance => 1.0,
        Approach::Breakthrough => 1.55,
        Approach::Hold => 0.55,
        Approach::FightingWithdrawal => 0.4,
    };
    let air = if matches!(o.air, AirMission::GroundSupport | AirMission::Interdiction)
        && order_air(w, c, &o) > 0.0
    {
        0.10
    } else {
        0.0
    };
    combat * (intensity + air) + garrison * 0.25
}
/// Occupation service and ground consumables cannot launch parked strike
/// aircraft. Their sortie plan pays only the fighting operation's duty share.
pub(crate) fn aircraft_consumption_multiplier(
    w: &WorldState,
    conflict: u32,
    nation: NationId,
) -> f64 {
    if !enabled(w) {
        return 1.0;
    }
    (consumption_multiplier(w, conflict, nation)
        - crate::campaign_peace::garrison_share(w, conflict, nation) * 0.25)
        .max(0.0)
}

/// Real equipment presence gates missions. Composition factors alone have a
/// 0.65 floor and therefore must never be used as proof that aircraft exist.
fn mission_assets(w: &WorldState, nation: NationId) -> (f64, f64) {
    let Some(n) = w.nation_opt(nation) else {
        return (0.0, 0.0);
    };
    let (mut total, mut air, mut sea) = (0.0, 0.0, 0.0);
    for h in &n.arsenal.held {
        let Some(kit) = arsenal::DECK.get(h.kit as usize) else {
            continue;
        };
        let value = arsenal::combat_value(n, h).max(0.0);
        total += value;
        if h.design_id
            .as_deref()
            .and_then(|id| crate::equipment::profile(n, id))
            .is_some_and(|profile| profile.aviation.is_some())
        {
            // Current custom aircraft are tactical strike designs. They do
            // not acquire reconnaissance, interception or transport roles
            // merely by occupying the historical Air inventory category.
            continue;
        }
        match kit.class {
            arsenal::Class::Air => air += value,
            arsenal::Class::Naval => sea += value,
            _ => {}
        }
    }
    (
        (air / (total * 0.15).max(EPS)).clamp(0.0, 1.0),
        (sea / (total * 0.15).max(EPS)).clamp(0.0, 1.0),
    )
}
fn custom_strike_assets(w: &WorldState, nation: NationId) -> bool {
    w.nation_opt(nation).is_some_and(|n| {
        n.arsenal.held.iter().any(|h| {
            arsenal::combat_value(n, h) > EPS
                && h.design_id
                    .as_deref()
                    .and_then(|id| crate::equipment::profile(n, id))
                    .is_some_and(|profile| profile.aviation.is_some())
        })
    })
}
fn order_air(w: &WorldState, c: &Conflict, o: &OperationOrder) -> f64 {
    if c.posture_of(o.nation)
        .is_none_or(|b| b.rung < SHOOTING_RUNG)
        || !theatre::has_access(w, o.nation, c.theatre)
    {
        0.0
    } else {
        mission_assets(w, o.nation).0
    }
}
/// Tactical custom aircraft implement paid standoff strike, not generic air
/// roles. Operations uses this same intent gate when planning physical stores.
pub(crate) fn custom_strike_ordered(w: &WorldState, c: &Conflict, nation: NationId) -> bool {
    !enabled(w)
        || (c
            .posture_of(nation)
            .is_some_and(|b| b.rung == SHOOTING_RUNG)
            && matches!(
                order_for(w, c, nation).air,
                AirMission::GroundSupport | AirMission::Interdiction
            ))
}
pub(crate) fn air_exposure_share(w: &WorldState, c: &Conflict, nation: NationId) -> f64 {
    if !enabled(w) {
        return 1.0;
    }
    let order = order_for(w, c, nation);
    let Some(posture) = c.posture_of(nation) else {
        return 0.0;
    };
    let strike = matches!(
        order.air,
        AirMission::GroundSupport | AirMission::Interdiction
    );
    let mission = if posture.rung == SHOOTING_RUNG && strike {
        1.0
    } else if posture.rung >= INVASION_RUNG && order.air == AirMission::GroundSupport {
        0.35
    } else {
        0.0
    };
    let assets = order_air(w, c, &order).max(
        if custom_strike_ordered(w, c, nation)
            && custom_strike_assets(w, nation)
            && theatre::has_access(w, nation, c.theatre)
        {
            1.0
        } else {
            0.0
        },
    );
    mission
        * assets
        * (1.0
            - order.reserve_bp as f64 / 10000.0
            - crate::campaign_peace::garrison_share(w, c.id, nation))
        .clamp(0.0, 1.0)
}
fn order_naval(w: &WorldState, c: &Conflict, o: &OperationOrder) -> f64 {
    if c.posture_of(o.nation)
        .is_none_or(|b| b.rung < SHOOTING_RUNG)
        || !theatre::has_access(w, o.nation, c.theatre)
    {
        0.0
    } else {
        mission_assets(w, o.nation).1
    }
}
pub(crate) fn supply_missions(
    w: &WorldState,
    conflict: u32,
    nation: NationId,
    snapshot: &operations::Snapshot,
) -> (f64, f64) {
    let Some(c) = w.conflict(conflict) else {
        return (0.0, 0.0);
    };
    let o = order_for(w, c, nation);
    let escort = if o.naval == NavalMission::Escort {
        order_naval(w, c, &o)
    } else {
        0.0
    };
    let denial = c
        .participants()
        .iter()
        .filter(|n| c.side_of(**n) != c.side_of(nation))
        .map(|n| {
            let other = order_for(w, c, *n);
            if other.naval == NavalMission::SeaDenial {
                order_naval(w, c, &other) * snapshot.deployed(conflict, *n)
                    / snapshot.strength(*n).max(EPS)
            } else {
                0.0
            }
        })
        .sum::<f64>()
        .clamp(0.0, 1.0);
    (escort, denial)
}

/// Staff uses the same saved observation the human receives, plus its own
/// condition. Public objectives and physical geography are common knowledge.
pub fn ai_orders(w: &mut WorldState) {
    if !enabled(w) {
        return;
    }
    let day = clock::absolute_day(w);
    let mut orders = vec![];
    for c in &w.conflicts {
        for nation in c.participants() {
            if w.player == Some(nation) || !w.nation_opt(nation).is_some_and(|n| n.alive) {
                continue;
            }
            let key = order_key(c.id, nation);
            if w.campaign.ai_review.get(&key).is_some_and(|d| day - *d < 7) {
                continue;
            }
            let mut o = order_for(w, c, nation);
            if o.target
                .as_ref()
                .is_some_and(|d| !front::contested_set(w, c).k.contains_key(d))
            {
                o.target = None;
            }
            let own: Vec<_> = w
                .campaign
                .sectors
                .values()
                .filter(|s| s.conflict == c.id && s.nation == nation)
                .collect();
            let mass = own.iter().map(|s| s.strength).sum::<f64>();
            let cohesion = own.iter().map(|s| s.strength * s.cohesion).sum::<f64>() / mass.max(EPS);
            let isolated = own.iter().any(|s| s.isolated_days >= 5);
            if c.posture_of(nation)
                .is_some_and(|b| b.objective == Objective::Withdraw)
            {
                o.approach = Approach::FightingWithdrawal;
            } else if isolated && cohesion < 0.3 {
                o.approach = Approach::FightingWithdrawal;
            } else if mass > EPS && cohesion < 0.45 {
                o.approach = Approach::Hold;
            } else if c
                .posture_of(nation)
                .is_some_and(|b| matches!(b.objective, Objective::Seize | Objective::Degrade))
            {
                let estimate = w.campaign.observations.get(&key);
                o.approach = if cohesion > 0.75
                    && estimate.is_some_and(|e| e.confidence >= 0.5 && mass > e.high * 1.4)
                {
                    Approach::Breakthrough
                } else {
                    Approach::Advance
                };
            }
            let (air, sea) = mission_assets(w, nation);
            o.air = if air <= 0.0
                && custom_strike_assets(w, nation)
                && c.posture_of(nation)
                    .is_some_and(|b| b.rung == SHOOTING_RUNG)
                && attacking(o.approach)
            {
                AirMission::GroundSupport
            } else if air <= 0.0 {
                AirMission::None
            } else if o.approach == Approach::Hold {
                AirMission::Interception
            } else if w
                .campaign
                .observations
                .get(&key)
                .is_none_or(|e| e.confidence < 0.55)
            {
                AirMission::Reconnaissance
            } else {
                AirMission::GroundSupport
            };
            o.naval = if sea <= 0.0 {
                NavalMission::None
            } else if !theatre::is_home(w, nation, c.theatre) {
                NavalMission::Escort
            } else {
                NavalMission::SeaDenial
            };
            orders.push(o);
        }
    }
    for order in orders {
        let key = order_key(order.conflict, order.nation);
        if crate::apply_command(w, &crate::Command::SetOperation { order }).is_ok() {
            w.campaign.ai_review.insert(key, day);
        }
    }
}

pub(crate) struct Opening {
    pub sectors: BTreeMap<String, Sector>,
    pub control: BTreeMap<String, Option<NationId>>,
    pub supply: BTreeMap<String, crate::campaign_supply::SupplyDelivery>,
    orders: BTreeMap<String, OperationOrder>,
    assets: BTreeMap<String, (f64, f64)>,
}
pub struct Resolution {
    /// Daily fractions of Snapshot::deployed, aggregated once per belligerent.
    pub losses: BTreeMap<NationId, f64>,
    /// Monthly-equivalent (incoming,outgoing) rates for political resolve.
    pub kill_rates: BTreeMap<NationId, (f64, f64)>,
}

fn friendly(
    w: &WorldState,
    c: &Conflict,
    nation: NationId,
    d: &str,
    owners: &BTreeMap<String, Option<NationId>>,
) -> bool {
    let Some(owner) = owners.get(d).copied().flatten() else {
        return false;
    };
    c.side_of(owner) == c.side_of(nation) && c.side_of(owner).is_some()
        || owner == nation
        || w.access
            .iter()
            .any(|a| a.seeker == nation && a.host == owner && a.theatre == c.theatre)
}
fn route(
    w: &WorldState,
    c: &Conflict,
    nation: NationId,
    from: &str,
    to: &str,
    owners: &BTreeMap<String, Option<NationId>>,
) -> Option<Vec<String>> {
    if from == to {
        return Some(vec![from.to_owned()]);
    }
    if !friendly(w, c, nation, to, owners) {
        return None;
    }
    let mut prev: BTreeMap<String, String> = BTreeMap::new();
    let mut queue = VecDeque::from([from.to_owned()]);
    prev.insert(from.to_owned(), String::new());
    while let Some(d) = queue.pop_front() {
        for n in districts::adj_of(&d) {
            if prev.contains_key(n) || !friendly(w, c, nation, n, owners) {
                continue;
            }
            prev.insert(n.clone(), d.clone());
            if n == to {
                let mut path = vec![to.to_owned()];
                let mut cur = to;
                while cur != from {
                    cur = prev.get(cur)?.as_str();
                    path.push(cur.to_owned());
                }
                path.reverse();
                return Some(path);
            }
            queue.push_back(n.clone());
        }
    }
    None
}
fn distances_from(from: &str) -> BTreeMap<String, usize> {
    let mut seen = BTreeMap::from([(from.to_owned(), 0)]);
    let mut q = VecDeque::from([(from.to_owned(), 0usize)]);
    while let Some((d, depth)) = q.pop_front() {
        if depth >= 32 {
            continue;
        }
        for n in districts::adj_of(&d) {
            if !seen.contains_key(n) {
                seen.insert(n.clone(), depth + 1);
                q.push_back((n.clone(), depth + 1));
            }
        }
    }
    seen
}
fn positions(
    w: &WorldState,
    c: &Conflict,
    nation: NationId,
    o: &OperationOrder,
    owners: &BTreeMap<String, Option<NationId>>,
) -> Vec<(String, f64)> {
    let k = front::contested_set(w, c);
    let side = c.side_of(nation).unwrap_or(true);
    let mut border = vec![];
    for d in w.districts.keys() {
        if !friendly(w, c, nation, d, owners) {
            continue;
        }
        let on_front = districts::adj_of(d).iter().any(|x| {
            k.k.get(x).is_some_and(|&(base, _)| {
                let h = c
                    .front
                    .get(x)
                    .map_or(if base { 1.0 } else { -1.0 }, |h| *h as f64);
                if side {
                    h < front::HELD_BAND
                } else {
                    h > -front::HELD_BAND
                }
            })
        });
        if on_front
            || (k.k.contains_key(d)
                && c.front
                    .get(d)
                    .is_some_and(|h| h.abs() <= front::HELD_BAND as f32))
        {
            border.push(d.clone());
        }
    }
    if border.is_empty() {
        border =
            k.k.keys()
                .filter(|d| friendly(w, c, nation, d, owners))
                .cloned()
                .collect();
    }
    if border.is_empty() && theatre::has_access(w, nation, c.theatre) {
        // An expedition needs a real friendly sustaining district. This is a
        // staging base, never an invented enemy beachhead.
        border = w
            .districts
            .keys()
            .filter(|d| friendly(w, c, nation, d, owners) && crate::logistics::has_terminal(d))
            .cloned()
            .collect();
    }
    let distances = o.target.as_ref().map(|t| distances_from(t));
    border.sort_by_key(|d| {
        (
            distances
                .as_ref()
                .map_or(0, |m| m.get(d).copied().unwrap_or(64)),
            d.clone(),
        )
    });
    border.truncate(MAX_SECTORS);
    border
        .into_iter()
        .map(|d| {
            let weight = if o
                .target
                .as_ref()
                .is_some_and(|t| d == *t || districts::adj_of(&d).contains(t))
            {
                3.0
            } else {
                1.0
            };
            (d, weight)
        })
        .collect()
}
fn add_reserve(state: &mut CampaignState, n: NationId, m: f64, cohesion: f64) {
    if m <= EPS {
        return;
    }
    let r = state.reserves.entry(n).or_default();
    r.cohesion = (r.cohesion * r.strength + cohesion * m) / (r.strength + m);
    r.strength += m;
}
fn add_sector(state: &mut CampaignState, cid: u32, n: NationId, d: String, m: f64, cohesion: f64) {
    if m <= EPS {
        return;
    }
    let s = state
        .sectors
        .entry(sector_key(cid, n, &d))
        .or_insert(Sector {
            conflict: cid,
            nation: n,
            district: d,
            strength: 0.0,
            cohesion,
            preparation: 0.0,
            isolated_days: 0,
        });
    s.cohesion = (s.cohesion * s.strength + cohesion * m) / (s.strength + m);
    s.preparation *= s.strength / (s.strength + m);
    s.strength += m;
}
pub fn accounted_force(w: &WorldState, nation: NationId) -> f64 {
    w.campaign.reserves.get(&nation).map_or(0.0, |r| r.strength)
        + w.campaign
            .sectors
            .values()
            .filter(|s| s.nation == nation)
            .map(|s| s.strength)
            .sum::<f64>()
        + w.campaign
            .transfers
            .iter()
            .filter(|t| t.nation == nation)
            .map(|t| t.strength)
            .sum::<f64>()
}

pub(crate) fn prepare(w: &mut WorldState, snapshot: &operations::Snapshot) -> Opening {
    w.campaign.conflict_id_high_water = conflict_id_high_water(w);
    // First retain their identities above; closed-war intent is no longer an
    // active policy. Historic agreements and conserved return journeys live
    // in their own ledgers and are not removed here.
    let live: BTreeSet<_> = w.conflicts.iter().map(|c| c.id).collect();
    let live_key = |key: &String| {
        key.split(':')
            .next()
            .and_then(|s| s.parse::<u32>().ok())
            .is_some_and(|id| live.contains(&id))
    };
    w.campaign_peace.aims.retain(|key, _| live_key(key));
    w.campaign_peace.garrisons.retain(|key, _| live_key(key));
    w.campaign_peace
        .last_proposal
        .retain(|key, _| live_key(key));
    let day = clock::absolute_day(w);
    let owners: BTreeMap<_, _> = w
        .districts
        .keys()
        .map(|d| (d.clone(), control::controller(w, d)))
        .collect();
    let mut state = std::mem::take(&mut w.campaign);
    let migration = !state.initialized;
    let migration_conflicts = state
        .migration_conflicts
        .take()
        .unwrap_or_else(|| w.conflicts.iter().map(|c| c.id).collect());
    let mut orders = BTreeMap::new();
    let mut assets = BTreeMap::new();
    for c in &w.conflicts {
        for n in c.participants() {
            let key = order_key(c.id, n);
            let mut o = state
                .orders
                .get(&key)
                .cloned()
                .unwrap_or_else(|| OperationOrder::automatic(c, n));
            if c.posture_of(n)
                .is_some_and(|b| b.objective == Objective::Withdraw)
            {
                o.approach = Approach::FightingWithdrawal;
            }
            assets.insert(key.clone(), (order_air(w, c, &o), order_naval(w, c, &o)));
            orders.insert(key, o);
        }
    }
    if state.last_prepared != Some(day) {
        // Disbanded/conflict-ended troops return on a dated journey; their
        // fatigue and force cannot be laundered by closing/reopening a war.
        let old = std::mem::take(&mut state.sectors);
        for (key, s) in old {
            if !w.nation_opt(s.nation).is_some_and(|n| n.alive) {
                continue;
            }
            if w.conflict(s.conflict)
                .is_none_or(|c| c.side_of(s.nation).is_none())
            {
                state.transfers.push(Transfer {
                    nation: s.nation,
                    conflict: None,
                    district: None,
                    strength: s.strength,
                    cohesion: s.cohesion,
                    route: vec![],
                    arrives_day: day + 14,
                    departed_day: day,
                });
            } else {
                state.sectors.insert(key, s);
            }
        }
        state
            .reserves
            .retain(|n, _| w.nation_opt(*n).is_some_and(|n| n.alive));
        state
            .transfers
            .retain(|t| w.nation_opt(t.nation).is_some_and(|n| n.alive));
        for n in w.nations.iter().filter(|n| n.alive) {
            let total = state.reserves.get(&n.id).map_or(0.0, |r| r.strength)
                + state
                    .sectors
                    .values()
                    .filter(|s| s.nation == n.id)
                    .map(|s| s.strength)
                    .sum::<f64>()
                + state
                    .transfers
                    .iter()
                    .filter(|t| t.nation == n.id)
                    .map(|t| t.strength)
                    .sum::<f64>();
            let available = snapshot.strength(n.id);
            if total > available + EPS {
                let scale = available / total;
                if let Some(r) = state.reserves.get_mut(&n.id) {
                    r.strength *= scale;
                }
                for s in state.sectors.values_mut().filter(|s| s.nation == n.id) {
                    s.strength *= scale;
                }
                for t in state.transfers.iter_mut().filter(|t| t.nation == n.id) {
                    t.strength *= scale;
                }
            } else {
                add_reserve(
                    &mut state,
                    n.id,
                    (available - total).max(0.0),
                    if migration { 0.85 } else { 0.7 },
                );
            }
            if let Some(r) = state.reserves.get_mut(&n.id) {
                r.cohesion += (1.0 - r.cohesion) * 0.025;
            }
        }
        let transit = std::mem::take(&mut state.transfers);
        for mut t in transit {
            if t.conflict.is_some_and(|cid| {
                w.conflict(cid)
                    .is_none_or(|c| c.side_of(t.nation).is_none())
            }) {
                t.conflict = None;
                t.district = None;
                t.route.clear();
                t.arrives_day = t.arrives_day.max(day + 7);
            }
            if !t.route.is_empty() {
                // The source was departed when the order began. Check only
                // remaining legs, so its subsequent capture cannot strand
                // a retreat already on the friendly side of the crossing.
                let elapsed = (day - t.departed_day).max(0) as usize;
                let span = (t.arrives_day - t.departed_day).max(1) as usize;
                let completed =
                    (1 + elapsed * t.route.len().saturating_sub(1) / span).min(t.route.len());
                let remaining = completed.min(t.route.len() - 1);
                if !crate::campaign_supply::deployment_path_open(w, t.nation, &t.route[remaining..])
                {
                    t.departed_day += 1;
                    t.arrives_day += 1;
                    t.cohesion = (t.cohesion - 0.005).max(0.05);
                    state.transfers.push(t);
                    continue;
                }
            }
            if t.arrives_day > day {
                state.transfers.push(t);
                continue;
            }
            match (t.conflict, t.district.clone()) {
                (Some(cid), Some(d)) => {
                    let valid = w
                        .conflict(cid)
                        .is_some_and(|c| friendly(w, c, t.nation, &d, &owners));
                    if valid {
                        add_sector(&mut state, cid, t.nation, d, t.strength, t.cohesion);
                    } else {
                        t.cohesion = (t.cohesion - 0.005).max(0.05);
                        state.transfers.push(t);
                    }
                }
                _ => add_reserve(&mut state, t.nation, t.strength, t.cohesion),
            }
        }
        let mut desired: BTreeMap<String, (u32, NationId, String, f64)> = BTreeMap::new();
        for ((cid, n), row) in &snapshot.rows {
            let Some(c) = w.conflict(*cid) else { continue };
            let o = &orders[&order_key(*cid, *n)];
            let places = positions(w, c, *n, o, &owners);
            let sum = places.iter().map(|(_, v)| *v).sum::<f64>();
            let garrison = crate::campaign_peace::garrison_share(w, *cid, *n);
            let committed =
                row.deployed * (1.0 - o.reserve_bp as f64 / 10000.0 - garrison).clamp(0.0, 1.0);
            for (d, weight) in places {
                desired.insert(
                    sector_key(*cid, *n, &d),
                    (*cid, *n, d, committed * weight / sum.max(EPS)),
                );
            }
        }
        // Transfer excess directly to a deficit if a friendly route exists.
        // A changed objective changes the request, never the arrival date.
        let keys: Vec<_> = state.sectors.keys().cloned().collect();
        for key in keys {
            let mut s = state.sectors[&key].clone();
            let wanted = desired.get(&key).map_or(0.0, |x| x.3);
            let mut excess = (s.strength - wanted).max(0.0);
            if excess <= EPS {
                continue;
            }
            let Some(c) = w.conflict(s.conflict) else {
                continue;
            };
            for (dk, (cid, n, d, mass)) in &desired {
                if *cid != s.conflict || *n != s.nation || dk == &key {
                    continue;
                }
                let arriving = state
                    .transfers
                    .iter()
                    .filter(|t| {
                        t.nation == *n && t.conflict == Some(*cid) && t.district.as_ref() == Some(d)
                    })
                    .map(|t| t.strength)
                    .sum::<f64>();
                let deficit =
                    (*mass - state.sectors.get(dk).map_or(0.0, |s| s.strength) - arriving).max(0.0);
                if deficit <= EPS {
                    continue;
                }
                let Some(path) = route(w, c, *n, &s.district, d, &owners) else {
                    continue;
                };
                let moved = excess.min(deficit);
                if moved <= EPS {
                    break;
                }
                state.transfers.push(Transfer {
                    nation: *n,
                    conflict: Some(*cid),
                    district: Some(d.clone()),
                    strength: moved,
                    cohesion: s.cohesion,
                    arrives_day: day + ((path.len() as i32 - 1) * 2).clamp(1, MAX_TRANSFER_DAYS),
                    route: path,
                    departed_day: day,
                });
                excess -= moved;
                s.strength -= moved;
            }
            if excess > EPS && friendly(w, c, s.nation, &s.district, &owners) {
                if let Some((mut path, days)) =
                    crate::campaign_supply::deployment_route(w, s.nation, &s.district)
                {
                    path.reverse();
                    state.transfers.push(Transfer {
                        nation: s.nation,
                        conflict: None,
                        district: None,
                        strength: excess,
                        cohesion: s.cohesion,
                        route: path,
                        arrives_day: day + (days as i32).clamp(4, MAX_TRANSFER_DAYS),
                        departed_day: day,
                    });
                    s.strength -= excess;
                }
            }
            state.sectors.insert(key, s);
        }
        for (key, (cid, n, d, mass)) in desired {
            let arriving = state
                .transfers
                .iter()
                .filter(|t| {
                    t.nation == n && t.conflict == Some(cid) && t.district.as_ref() == Some(&d)
                })
                .map(|t| t.strength)
                .sum::<f64>();
            let deficit =
                (mass - state.sectors.get(&key).map_or(0.0, |s| s.strength) - arriving).max(0.0);
            let preposition = migration && migration_conflicts.contains(&cid);
            let deployment = if preposition {
                Some((vec![], 0))
            } else {
                crate::campaign_supply::deployment_route(w, n, &d)
            };
            let Some((path, days)) = deployment else {
                continue;
            };
            let Some(reserve) = state.reserves.get_mut(&n) else {
                continue;
            };
            let amount = deficit.min(reserve.strength);
            let cohesion = reserve.cohesion;
            if amount <= EPS {
                continue;
            }
            reserve.strength -= amount;
            if preposition {
                add_sector(&mut state, cid, n, d, amount, cohesion);
            } else {
                state.transfers.push(Transfer {
                    nation: n,
                    conflict: Some(cid),
                    district: Some(d),
                    strength: amount,
                    cohesion,
                    route: path,
                    arrives_day: day + (days as i32).clamp(4, MAX_TRANSFER_DAYS),
                    departed_day: day,
                });
            }
        }
        state.sectors.retain(|_, s| s.strength > EPS);
        state
            .orders
            .retain(|_, o| w.conflict(o.conflict).is_some_and(|c| c.involves(o.nation)));
        state.observations.retain(|key, _| orders.contains_key(key));
        state.reports.retain(|key, _| orders.contains_key(key));
        state.ai_review.retain(|key, _| orders.contains_key(key));
        state.alerts.retain(|key, _| orders.contains_key(key));
        state.initialized = true;
        state.last_prepared = Some(day);
    }
    let sectors = state.sectors.clone();
    w.campaign = state;
    let mut requests = vec![];
    for (key, s) in &sectors {
        let Some(c) = w.conflict(s.conflict) else {
            continue;
        };
        let Some(row) = snapshot.rows.get(&(s.conflict, s.nation)) else {
            continue;
        };
        let (escort, denial) = supply_missions(w, s.conflict, s.nation, snapshot);
        let pool = sectors
            .values()
            .filter(|x| x.conflict == s.conflict && x.nation == s.nation)
            .map(|x| x.strength)
            .sum::<f64>();
        let available = s.strength * (row.deployed / pool.max(EPS)).min(1.0);
        // A contested line receives service from an immediately adjacent
        // friendly staging district. Cargo never enters contested ground.
        // Preserve a valid ingress rather than resetting its paid buffer when
        // another equally valid adjacency happens to sort earlier.
        let held_by_enemy = owners
            .get(&s.district)
            .copied()
            .flatten()
            .is_some_and(|owner| {
                c.side_of(owner).is_some() && c.side_of(owner) != c.side_of(s.nation)
            });
        let previous = w
            .campaign_supply
            .buffers
            .get(key)
            .map(|b| b.district.clone());
        let ingress = if friendly(w, c, s.nation, &s.district, &owners) || held_by_enemy {
            s.district.clone()
        } else if let Some(d) = previous.filter(|d| {
            districts::adj_of(&s.district).contains(d) && friendly(w, c, s.nation, d, &owners)
        }) {
            d
        } else {
            districts::adj_of(&s.district)
                .iter()
                .filter(|d| friendly(w, c, s.nation, d, &owners))
                .min()
                .cloned()
                .unwrap_or_else(|| s.district.clone())
        };
        requests.push(crate::campaign_supply::SupplyRequest {
            key: key.clone(),
            nation: s.nation,
            conflict: s.conflict,
            district: ingress,
            deployed: available,
            burn_monthly: row.support_burn_monthly * available / row.deployed.max(EPS),
            sea_escort: escort,
            sea_denial: denial,
        });
    }
    requests.extend(crate::campaign_peace::garrison_requests(w, snapshot));
    let supply = crate::campaign_supply::prepare(w, &requests);
    Opening {
        sectors,
        control: owners,
        supply,
        orders,
        assets,
    }
}

#[derive(Clone)]
struct Contribution {
    key: String,
    nation: NationId,
    side: bool,
    mass: f64,
    maneuver_mass: f64,
    power: f64,
    /// Physical ammunition already prices firing roles and installed strike
    /// quality. `None` retains the historical role composition calculation.
    attack_power: Option<f64>,
    ground: f64,
    quality: f64,
    fire: f64,
    recon: f64,
    air: f64,
    air_defense: f64,
    interdict: f64,
    mobility: f64,
    land: f64,
    strike: f64,
    river: bool,
    preparation: f64,
    rung: u8,
    roe: Roe,
    approach: Approach,
    objective: Objective,
}
fn hold(c: &Conflict, k: &front::Contested, d: &str) -> f64 {
    c.front
        .get(d)
        .map_or(if k.k[d].0 { 1.0 } else { -1.0 }, |h| *h as f64)
}
fn attacking(o: Approach) -> bool {
    matches!(
        o,
        Approach::Probe | Approach::Advance | Approach::Breakthrough
    )
}
fn approach_push(o: Approach) -> f64 {
    match o {
        Approach::Probe => 0.45,
        Approach::Advance => 1.0,
        Approach::Breakthrough => 1.6,
        _ => 0.0,
    }
}
fn approach_fire(o: Approach) -> f64 {
    match o {
        Approach::Probe => 0.6,
        Approach::Breakthrough => 1.25,
        Approach::FightingWithdrawal => 0.35,
        _ => 1.0,
    }
}

/// Each sector's finite combat share is divided over its contacts once. The
/// defender and attacker never each receive a separate copy of the formation.
fn contacts(
    w: &WorldState,
    c: &Conflict,
    deployments: &BTreeMap<(u32, NationId), operations::Deployment>,
    opening: &Opening,
    k: &front::Contested,
) -> BTreeMap<String, Vec<Contribution>> {
    let mut out: BTreeMap<String, Vec<Contribution>> = BTreeMap::new();
    for (key, s) in opening.sectors.iter().filter(|(_, s)| s.conflict == c.id) {
        let Some(row) = deployments.get(&(c.id, s.nation)) else {
            continue;
        };
        let Some(posture) = c.posture_of(s.nation) else {
            continue;
        };
        let Some(side) = c.side_of(s.nation) else {
            continue;
        };
        let o = &opening.orders[&order_key(c.id, s.nation)];
        let assets = opening.assets[&order_key(c.id, s.nation)];
        let paid_strike = if posture.rung == SHOOTING_RUNG
            && matches!(o.air, AirMission::GroundSupport | AirMission::Interdiction)
        {
            row.ammunition
                .filter(|effects| effects.attack_coefficient > EPS)
                .map_or(0.0, |effects| effects.fire_fraction)
        } else {
            0.0
        };
        let mission_air = (assets.0 + paid_strike).clamp(0.0, 1.0);
        let total = opening
            .sectors
            .values()
            .filter(|x| x.conflict == c.id && x.nation == s.nation)
            .map(|x| x.strength)
            .sum::<f64>();
        let active = s.strength * (row.deployed / total.max(EPS)).min(1.0);
        if active <= EPS {
            continue;
        }
        let mut targets: BTreeMap<String, f64> = BTreeMap::new();
        if k.k.contains_key(&s.district) {
            targets.insert(
                s.district.clone(),
                if attacking(o.approach) { 1.0 } else { 3.0 },
            );
        }
        for d in districts::adj_of(&s.district) {
            if !k.k.contains_key(d) {
                continue;
            }
            let h = hold(c, k, d);
            let enemy = if side {
                h < front::HELD_BAND
            } else {
                h > -front::HELD_BAND
            };
            if enemy && attacking(o.approach) {
                targets.insert(
                    d.clone(),
                    if o.target.as_ref() == Some(d) {
                        4.0
                    } else {
                        1.0
                    },
                );
            }
        }
        let hostile_contact = targets.keys().any(|d| {
            let h = hold(c, k, d);
            if side {
                h < front::HELD_BAND
            } else {
                h > -front::HELD_BAND
            }
        });
        if !hostile_contact && theatre::has_access(w, s.nation, c.theatre) {
            let supported_landing = attacking(o.approach)
                && matches!(o.naval, NavalMission::Transport | NavalMission::Escort)
                && assets.1 > 0.0
                && row.capabilities.lift > 0.0;
            let standoff = posture.rung == SHOOTING_RUNG
                && mission_air > 0.0
                && matches!(o.air, AirMission::GroundSupport | AirMission::Interdiction);
            if supported_landing || standoff {
                // Only the named/nearest peripheral district receives an
                // expedition. This is a sustained contact from a real base,
                // and its ground advance is bounded by naval lift below.
                let distances = distances_from(&s.district);
                let target =
                    k.k.keys()
                        .filter(|d| {
                            let h = hold(c, k, d);
                            let enemy = if side {
                                h < front::HELD_BAND
                            } else {
                                h > -front::HELD_BAND
                            };
                            enemy
                                && (districts::adj_of(d).is_empty()
                                    || crate::logistics::has_terminal(d))
                        })
                        .min_by_key(|d| {
                            (
                                if o.target.as_ref() == Some(*d) { 0 } else { 1 },
                                distances.get(*d).copied().unwrap_or(64),
                                (*d).clone(),
                            )
                        });
                if let Some(d) = target {
                    targets.insert(d.clone(), 1.0);
                }
            }
        }
        let weights = targets.values().sum::<f64>();
        let coverage = opening
            .supply
            .get(key)
            .map_or(0.0, |d| d.coverage)
            .clamp(0.0, 1.0);
        let legacy_ammunition = if row.deployed > EPS {
            row.effective_force / row.deployed
        } else {
            0.0
        };
        let maneuver = row
            .ammunition
            .map_or(legacy_ammunition, |effects| effects.maneuver_fraction);
        for (d, weight) in targets {
            let mass = active * weight / weights.max(EPS);
            let conditioned =
                mass * row.quality * (0.15 + 0.85 * s.cohesion) * (0.12 + 0.88 * coverage);
            let strike_mission =
                matches!(o.air, AirMission::GroundSupport | AirMission::Interdiction);
            let air = if posture.rung == SHOOTING_RUNG && strike_mission {
                mission_air
            } else if o.air == AirMission::GroundSupport {
                assets.0 * 0.35
            } else {
                0.0
            };
            let remote = d != s.district && !districts::adj_of(&s.district).contains(&d);
            let ground = if posture.rung < INVASION_RUNG || coverage < 0.15 {
                0.0
            } else if remote {
                assets.1 * 0.35
            } else {
                1.0
            };
            out.entry(d.clone()).or_default().push(Contribution {
                key: key.clone(),
                nation: s.nation,
                side,
                mass,
                maneuver_mass: row.ammunition.map_or(mass, |_| mass * maneuver),
                power: conditioned * maneuver,
                attack_power: if posture.rung == SHOOTING_RUNG && !strike_mission {
                    Some(0.0)
                } else {
                    row.ammunition
                        .map(|effects| conditioned * effects.attack_coefficient)
                },
                ground,
                quality: row.quality,
                fire: row.capabilities.ground_roles.fire_support,
                recon: row.capabilities.ground_roles.reconnaissance
                    + if o.air == AirMission::Reconnaissance {
                        assets.0 * 0.25
                    } else {
                        0.0
                    },
                air,
                air_defense: row
                    .ammunition
                    .map_or(row.capabilities.ground_roles.air_defense, |effects| {
                        effects.air_defense
                    })
                    + if o.air == AirMission::Interception {
                        assets.0 * 0.35
                    } else {
                        0.0
                    },
                interdict: if o.air == AirMission::Interdiction {
                    mission_air * 0.25
                } else {
                    0.0
                },
                mobility: row.capabilities.ground_roles.protected_mobility,
                land: row.capabilities.land,
                strike: row.capabilities.strike,
                river: d != s.district && districts::crosses_river(&s.district, &d),
                preparation: s.preparation,
                rung: posture.rung,
                roe: posture.roe,
                approach: o.approach,
                objective: posture.objective,
            });
        }
    }
    out
}

#[derive(Default)]
struct ContactResult {
    movement: f64,
    losses: BTreeMap<String, f64>,
    pressure: BTreeMap<String, f64>,
}
fn contact_result(
    w: &WorldState,
    c: &Conflict,
    d: &str,
    h: f64,
    units: &[Contribution],
) -> ContactResult {
    let mut result = ContactResult::default();
    let tempo = front::tempo_of(districts::terrain_of(d));
    let th = theatre::theatre(w, c.theatre);
    let width = (districts::area_of(d).sqrt() / 8.0).clamp(2.0, 30.0);
    let mut power = [0.0; 2];
    let mut offensive = [0.0; 2];
    let mut mass = [0.0; 2];
    for u in units {
        let i = usize::from(!u.side);
        mass[i] += u.maneuver_mass;
    }
    for u in units {
        let i = usize::from(!u.side);
        let density = (width / mass[i].max(width)).sqrt();
        let held = if u.side {
            h > front::HELD_BAND
        } else {
            h < -front::HELD_BAND
        };
        let defence = if held {
            1.0 + u.preparation * 0.75 / tempo.max(0.3)
        } else {
            1.0
        };
        let interdiction = units
            .iter()
            .filter(|e| e.side != u.side)
            .map(|e| e.interdict * e.maneuver_mass / mass[1 - i].max(EPS))
            .sum::<f64>();
        power[i] += u.power * density * defence * (1.0 - interdiction);
        if u.rung >= INVASION_RUNG {
            let crossing = if u.river {
                front::RIVER_CROSS_TEMPO
            } else {
                1.0
            };
            offensive[i] += u.power
                * density
                * u.ground
                * u.land
                * approach_push(u.approach)
                * war::obj_seize(u.objective)
                * war::roe_seize(u.roe)
                * (0.4 + 0.6 * u.preparation)
                * (1.0 + u.mobility)
                * crossing
                * (1.0 - interdiction);
        }
    }
    // Defence can stop aggregate advance. No uncapped sweep redistributes a
    // mountain's unspent movement into another province after this result.
    let resistance_a = power[1] * 0.8;
    let resistance_b = power[0] * 0.8;
    let advance_a = (offensive[0] - resistance_a).max(0.0);
    let advance_b = (offensive[1] - resistance_b).max(0.0);
    let margin = (advance_a - advance_b) / (power[0] + power[1] + war::SEIZE_FLOOR);
    let district_scale = (districts::area_of(d) / 10000.0).sqrt().clamp(0.65, 2.0);
    result.movement = (0.30 * tempo * margin / district_scale).clamp(-0.45, 0.45);
    result.movement = result.movement.clamp(-1.0 - h, 1.0 - h);
    for victim in units {
        let i = usize::from(!victim.side);
        let mut daily_rate = 0.0;
        for hunter in units
            .iter()
            .filter(|u| u.side != victim.side && u.rung >= SHOOTING_RUNG)
        {
            let base = war::RUNG_EXPOSURE[victim.rung.min(9) as usize];
            let find =
                (hunter.quality * (1.0 + hunter.recon) / victim.quality.max(0.01)).clamp(0.3, 3.0);
            let gated = if find > 1.0 && base < 1.0 {
                crate::exact::powf(
                    find,
                    war::SENSOR_SATURATION + (1.0 - war::SENSOR_SATURATION) * base,
                )
            } else {
                find
            };
            let cover =
                tempo.min(1.0) * (1.0 - 0.55 * th.urbanisation) / (1.0 + victim.preparation * 0.35);
            let ground_fire = (1.0 - hunter.air) * (1.0 + hunter.fire) * hunter.land;
            let air_fire = hunter.air * (1.0 - victim.air_defense.clamp(0.0, 0.7)) * hunter.strike;
            let attack = war::obj_kill(hunter.objective)
                * war::roe_kill(hunter.roe)
                * approach_fire(hunter.approach);
            let firing = hunter
                .attack_power
                .map_or(hunter.power * (ground_fire + air_fire), |power| {
                    power * (1.0 - hunter.air * victim.air_defense.clamp(0.0, 0.7))
                });
            let rate = (war::KILL_RATE * firing * attack * base * gated * cover
                / (mass[i] + war::FLOOR_MASS))
                .clamp(0.0, 0.5);
            daily_rate += clock::blend(w, rate);
        }
        daily_rate = daily_rate.clamp(0.0, 0.05);
        result
            .losses
            .insert(victim.key.clone(), victim.mass * daily_rate);
        let enemy = power[1 - i] / (power[i] + power[1 - i] + EPS);
        let moving =
            if (result.movement > EPS && victim.side) || (result.movement < -EPS && !victim.side) {
                0.2
            } else {
                0.0
            };
        result
            .pressure
            .insert(victim.key.clone(), enemy.max(moving) * victim.mass);
    }
    result
}
pub(crate) fn resolve(
    w: &mut WorldState,
    c: &mut Conflict,
    snapshot: &operations::Snapshot,
    opening: &Opening,
) -> Resolution {
    let day = clock::absolute_day(w);
    let k = front::contested_set(w, c);
    let combat = contacts(w, c, &snapshot.rows, opening, &k);
    let mut amounts: BTreeMap<NationId, f64> = BTreeMap::new();
    let mut raw_amounts: BTreeMap<NationId, f64> = BTreeMap::new();
    let mut sector_loss: BTreeMap<String, f64> = BTreeMap::new();
    let mut pressure: BTreeMap<String, f64> = BTreeMap::new();
    let mut movement = [0.0; 2];
    let opening_conflict = c.clone();
    for (d, units) in &combat {
        let h = hold(&opening_conflict, &k, d);
        let result = contact_result(w, &opening_conflict, d, h, units);
        if result.movement.abs() > EPS {
            c.front
                .insert(d.clone(), (h + result.movement).clamp(-1.0, 1.0) as f32);
        }
        if result.movement > 0.0 {
            movement[0] += result.movement
        } else {
            movement[1] -= result.movement
        }
        for (key, m) in result.losses {
            *sector_loss.entry(key).or_default() += m;
        }
        for (key, p) in result.pressure {
            *pressure.entry(key).or_default() += p;
        }
    }
    let keys: Vec<_> = opening
        .sectors
        .iter()
        .filter(|(_, s)| s.conflict == c.id)
        .map(|(key, _)| key.clone())
        .collect();
    let mut pocket_districts: BTreeSet<String> = BTreeSet::new();
    let mut retreats_applied = BTreeSet::new();
    for key in keys {
        let Some(mut s) = w.campaign.sectors.get(&key).cloned() else {
            continue;
        };
        let o = &opening.orders[&order_key(c.id, s.nation)];
        let coverage = opening
            .supply
            .get(&key)
            .map_or(0.0, |d| d.coverage)
            .clamp(0.0, 1.0);
        let p = pressure.get(&key).copied().unwrap_or(0.0) / s.strength.max(EPS);
        let fighting = p > 0.0;
        let active = snapshot.deployed(c.id, s.nation) > EPS;
        let mut raw_loss = sector_loss
            .get(&key)
            .copied()
            .unwrap_or(0.0)
            .min(s.strength);
        let mut loss = snapshot.actual_loss(c.id, s.nation, raw_loss);
        let fatigue = if fighting && active {
            match o.approach {
                Approach::Probe => 0.006,
                Approach::Advance => 0.018,
                Approach::Breakthrough => 0.035,
                Approach::Hold => 0.004,
                Approach::FightingWithdrawal => 0.008,
            }
        } else {
            0.0
        };
        let recovery = if matches!(o.approach, Approach::Hold | Approach::Probe) || !fighting {
            0.025 * coverage * (1.0 - s.cohesion)
        } else {
            0.004 * coverage * (1.0 - s.cohesion)
        };
        s.cohesion = (s.cohesion + recovery
            - fatigue
            - p * 0.008
            - (1.0 - coverage) * 0.012
            - loss / s.strength.max(EPS) * 2.0)
            .clamp(0.0, 1.0);
        s.preparation = (s.preparation
            + if attacking(o.approach) && fighting {
                -0.02
            } else {
                0.035 * coverage * (1.0 - s.preparation)
            })
        .clamp(0.0, 1.0);
        s.isolated_days = if coverage < 0.05 {
            s.isolated_days.saturating_add(1)
        } else {
            0
        };
        if s.isolated_days >= 7 {
            pocket_districts.insert(s.district.clone());
        }
        let needs_retreat =
            active && fighting && (s.cohesion < 0.22 || o.approach == Approach::FightingWithdrawal);
        if needs_retreat {
            let fallback = districts::adj_of(&s.district)
                .iter()
                .filter(|d| friendly(w, c, s.nation, d, &opening.control))
                .max_by(|a, b| {
                    districts::area_of(a)
                        .total_cmp(&districts::area_of(b))
                        .then_with(|| b.cmp(a))
                })
                .cloned();
            if let Some(dest) = fallback {
                let survivors = (s.strength - loss).max(0.0);
                w.campaign.transfers.push(Transfer {
                    nation: s.nation,
                    conflict: Some(c.id),
                    district: Some(dest.clone()),
                    strength: survivors,
                    cohesion: s.cohesion,
                    route: vec![s.district.clone(), dest],
                    arrives_day: day + 2,
                    departed_day: day,
                });
                if k.k.contains_key(&s.district)
                    && retreats_applied.insert((s.district.clone(), c.side_of(s.nation)))
                {
                    let side = c.side_of(s.nation).unwrap_or(true);
                    let h = hold(c, &k, &s.district);
                    let step = if side { -0.2 } else { 0.2 };
                    c.front
                        .insert(s.district.clone(), (h + step).clamp(-1.0, 1.0) as f32);
                    movement[usize::from(side)] += 0.2;
                }
                s.strength = loss;
            } else if s.isolated_days >= 14 && s.cohesion < 0.12 {
                // Delayed surrender is a real debit, bounded by the still
                // deployed fraction if national command has already withdrawn.
                let total = opening
                    .sectors
                    .values()
                    .filter(|x| x.conflict == c.id && x.nation == s.nation)
                    .map(|x| x.strength)
                    .sum::<f64>();
                raw_loss =
                    s.strength * (snapshot.deployed(c.id, s.nation) / total.max(EPS)).min(1.0);
                loss = snapshot.actual_loss(c.id, s.nation, raw_loss);
            }
        }
        s.strength = (s.strength - loss).max(0.0);
        *amounts.entry(s.nation).or_default() += loss;
        *raw_amounts.entry(s.nation).or_default() += raw_loss;
        w.campaign.sectors.insert(key, s);
    }
    w.campaign.sectors.retain(|_, s| s.strength > EPS);
    c.pockets = connected_groups(&pocket_districts);
    front::finish(c, &k);
    let mut losses = BTreeMap::new();
    let mut kill_rates = BTreeMap::new();
    for nation in c.participants() {
        let deployed = snapshot.deployed(c.id, nation);
        let amount = amounts.get(&nation).copied().unwrap_or(0.0).min(deployed);
        let fraction = if deployed > EPS {
            raw_amounts
                .get(&nation)
                .copied()
                .unwrap_or(0.0)
                .min(deployed)
                / deployed
        } else {
            0.0
        };
        losses.insert(nation, fraction);
        let opposing: Vec<_> = c
            .participants()
            .into_iter()
            .filter(|n| c.side_of(*n) != c.side_of(nation))
            .collect();
        let enemy_deployed = opposing
            .iter()
            .map(|n| snapshot.deployed(c.id, *n))
            .sum::<f64>();
        let outgoing = opposing
            .iter()
            .map(|n| amounts.get(n).copied().unwrap_or(0.0))
            .sum::<f64>()
            / enemy_deployed.max(EPS);
        let monthly = |daily: f64| {
            1.0 - crate::exact::powf(
                (1.0 - daily.clamp(0.0, 1.0)).max(0.0),
                1.0 / clock::month_fraction(w),
            )
        };
        kill_rates.insert(
            nation,
            (monthly(amount / deployed.max(EPS)), monthly(outgoing)),
        );
        let key = order_key(c.id, nation);
        let own: Vec<_> = w
            .campaign
            .sectors
            .values()
            .filter(|s| s.conflict == c.id && s.nation == nation)
            .collect();
        let own_mass = own.iter().map(|s| s.strength).sum::<f64>();
        let readiness =
            own.iter().map(|s| s.strength * s.cohesion).sum::<f64>() / own_mass.max(EPS);
        let weighted_supply = opening
            .sectors
            .iter()
            .filter(|(_, s)| s.conflict == c.id && s.nation == nation)
            .map(|(key, s)| s.strength * opening.supply.get(key).map_or(0.0, |d| d.coverage))
            .sum::<f64>()
            / opening
                .sectors
                .values()
                .filter(|s| s.conflict == c.id && s.nation == nation)
                .map(|s| s.strength)
                .sum::<f64>()
                .max(EPS);
        let side = c.side_of(nation).unwrap_or(true);
        let i = usize::from(!side);
        let summary = if own_mass <= EPS {
            "Forces are in reserve or moving to their assigned sector.".into()
        } else if weighted_supply < 0.35 {
            "Advance constrained by supply; local service reserves are depleted.".into()
        } else if readiness < 0.35 {
            "Cohesion is low. A supplied pause or fighting withdrawal preserves the force.".into()
        } else if movement[i] > 0.001 {
            "Local superiority is opening ground; protect the supply corridor.".into()
        } else if movement[1 - i] > 0.001 {
            "The opposing force is gaining ground. Prepare a fallback or commit reserves.".into()
        } else {
            "Prepared defenses and local force balance are holding the front.".into()
        };
        let crossed = w.campaign.reports.get(&key).is_some_and(|old| {
            (old.supply >= 0.35 && weighted_supply < 0.35)
                || (old.readiness >= 0.35 && readiness < 0.35)
                || (old.advanced < 0.04 && movement[i] >= 0.04)
                || (old.retreated < 0.04 && movement[1 - i] >= 0.04)
        });
        if w.player == Some(nation)
            && crossed
            && w.campaign.alerts.get(&key).is_none_or(|d| day - *d >= 7)
        {
            w.headline(format!(
                "War: {} over {}. {}",
                nation.name(),
                c.theatre.name(),
                summary
            ));
            w.campaign.alerts.insert(key.clone(), day);
        }
        w.campaign.reports.insert(
            key.clone(),
            BattleReport {
                day,
                day_label: date_label(day),
                advanced: movement[i],
                retreated: movement[1 - i],
                losses: amount,
                readiness,
                summary,
                supply: weighted_supply,
            },
        );
        // Observations are updated only in a model step. Quantized brackets
        // prevent an exact opposing deployment leaking through its midpoint.
        let o = &opening.orders[&key];
        let recon = if o.air == AirMission::Reconnaissance {
            opening.assets[&key].0 * 0.35
        } else {
            0.0
        };
        let contact = if combat.values().any(|units| {
            units.iter().any(|u| u.nation == nation) && units.iter().any(|u| u.side != side)
        }) {
            0.25
        } else {
            0.0
        };
        let confidence = (0.2 + contact + recon).clamp(0.15, 0.85);
        let quantum = if confidence >= 0.7 {
            2.0
        } else if confidence >= 0.45 {
            5.0
        } else {
            10.0
        };
        let low = (enemy_deployed * (0.5 + confidence * 0.35) / quantum).floor() * quantum;
        let high = (enemy_deployed * (1.8 - confidence * 0.6) / quantum).ceil() * quantum + quantum;
        w.campaign.observations.insert(
            key,
            EnemyEstimate {
                low,
                high,
                confidence,
                observed_day: day,
                observed_label: date_label(day),
            },
        );
    }
    Resolution { losses, kill_rates }
}
fn connected_groups(districts_set: &BTreeSet<String>) -> Vec<Vec<String>> {
    let mut unseen = districts_set.clone();
    let mut groups = vec![];
    while let Some(start) = unseen.pop_first() {
        let mut group = vec![start.clone()];
        let mut queue = VecDeque::from([start]);
        while let Some(d) = queue.pop_front() {
            for n in districts::adj_of(&d) {
                if unseen.remove(n) {
                    group.push(n.clone());
                    queue.push_back(n.clone());
                }
            }
        }
        group.sort();
        groups.push(group);
    }
    groups.sort();
    groups
}

#[derive(Clone, Debug, Serialize)]
pub struct TargetView {
    pub id: String,
    pub name: String,
    pub held: bool,
}
#[derive(Clone, Debug, Serialize)]
pub struct SupplyView {
    pub coverage: f64,
    pub status: String,
    pub reason: String,
    pub eta_days: Option<u32>,
}
#[derive(Clone, Debug, Serialize)]
pub struct Forecast {
    pub advance_low: f64,
    pub advance_high: f64,
    pub loss_low: f64,
    pub loss_high: f64,
    pub reason: String,
    pub period: String,
}
#[derive(Clone, Debug, Serialize)]
pub struct OperationView {
    pub conflict: u32,
    pub enabled: bool,
    pub order: OperationOrder,
    pub targets: Vec<TargetView>,
    pub fielded: f64,
    pub reserve: f64,
    pub garrison: f64,
    pub in_transit: f64,
    pub readiness: f64,
    pub preparation: f64,
    pub supply: SupplyView,
    pub enemy: EnemyEstimate,
    pub forecast: Forecast,
    pub last_report: Option<BattleReport>,
    pub mission_constraints: Vec<String>,
}
/// View construction reads only the viewer's condition and saved observation.
/// It never creates a tactical snapshot or samples the opposing live military.
pub fn view(w: &WorldState, conflict: u32, viewer: NationId) -> Option<OperationView> {
    if !enabled(w) {
        return None;
    }
    let c = w.conflict(conflict)?;
    let side = c.side_of(viewer)?;
    let key = order_key(conflict, viewer);
    let order = order_for(w, c, viewer);
    let own: Vec<_> = w
        .campaign
        .sectors
        .values()
        .filter(|s| s.conflict == conflict && s.nation == viewer)
        .collect();
    let fielded = own.iter().map(|s| s.strength).sum::<f64>();
    let readiness = own.iter().map(|s| s.strength * s.cohesion).sum::<f64>() / fielded.max(EPS);
    let preparation =
        own.iter().map(|s| s.strength * s.preparation).sum::<f64>() / fielded.max(EPS);
    let in_transit = w
        .campaign
        .transfers
        .iter()
        .filter(|t| t.nation == viewer && t.conflict == Some(conflict))
        .map(|t| t.strength)
        .sum();
    // Supply receipts come from the supply authority; no web-side cargo math.
    let supply = crate::campaign_supply::view(w, conflict, viewer);
    let enemy = w
        .campaign
        .observations
        .get(&key)
        .cloned()
        .unwrap_or(EnemyEstimate {
            low: 0.0,
            high: 100.0,
            confidence: 0.0,
            observed_day: -1,
            observed_label: "Not yet observed".into(),
        });
    let k = front::contested_set(w, c);
    let targets =
        k.k.keys()
            .map(|d| {
                let h = hold(c, &k, d);
                TargetView {
                    id: d.clone(),
                    name: districts::name_of(d).unwrap_or(d).to_owned(),
                    held: if side {
                        h > front::HELD_BAND
                    } else {
                        h < -front::HELD_BAND
                    },
                }
            })
            .collect();
    let assessment = assessment(
        w,
        c,
        &order,
        fielded,
        readiness,
        preparation,
        supply.coverage,
        &enemy,
    );
    let mut mission_constraints = vec![];
    let paid_strike = custom_strike_ordered(w, c, viewer)
        && operations::view(w, viewer)
            .deployments
            .iter()
            .find(|row| row.conflict == conflict)
            .and_then(|row| row.ammunition)
            .is_some_and(|effects| effects.attack_coefficient > EPS);
    if order.air != AirMission::None && order_air(w, c, &order) <= EPS && !paid_strike {
        mission_constraints.push("Air mission is idle: compatible aircraft, stores, shooting commitment and theatre access are required. Custom strike aircraft support ground support or interdiction at standoff commitment only.".into());
    }
    if order.naval != NavalMission::None && order_naval(w, c, &order) <= EPS {
        mission_constraints.push("Naval mission is idle: serviceable ships, shooting commitment and theatre access are required.".into());
    }
    if (order.air != AirMission::None || order.naval != NavalMission::None)
        && supply.coverage < 0.35
    {
        mission_constraints.push("Support missions are constrained by delivered supply.".into());
    }
    let garrison = operations::view(w, viewer)
        .deployments
        .iter()
        .find(|r| r.conflict == conflict)
        .map_or(0.0, |r| r.deployed)
        * crate::campaign_peace::garrison_share(w, conflict, viewer);
    let assigned_garrisons = operations::view(w, viewer)
        .deployments
        .iter()
        .map(|r| r.deployed * crate::campaign_peace::garrison_share(w, r.conflict, viewer))
        .sum::<f64>();
    Some(OperationView {
        conflict,
        enabled: true,
        order,
        targets,
        fielded,
        reserve: (w.campaign.reserves.get(&viewer).map_or(0.0, |r| r.strength)
            - assigned_garrisons)
            .max(0.0),
        garrison,
        in_transit,
        readiness,
        preparation,
        supply: SupplyView {
            coverage: supply.coverage,
            status: supply.status,
            reason: supply.reason,
            eta_days: supply.eta_days,
        },
        enemy,
        forecast: assessment,
        last_report: w.campaign.reports.get(&key).cloned(),
        mission_constraints,
    })
}
fn assessment(
    w: &WorldState,
    c: &Conflict,
    o: &OperationOrder,
    force: f64,
    readiness: f64,
    _preparation: f64,
    supply: f64,
    enemy: &EnemyEstimate,
) -> Forecast {
    let k = front::contested_set(w, c);
    let side = c.side_of(o.nation).unwrap_or(true);
    let key = order_key(c.id, o.nation);
    // Construct ONLY the viewer's deployment. Contact selection, finite force
    // division, remote mission gates, crossings and equipment are the exact
    // same functions used by resolution; no live opposing military is read.
    let deployments: BTreeMap<_, _> = operations::view(w, o.nation)
        .deployments
        .into_iter()
        .map(|r| ((r.conflict, r.nation), r))
        .collect();
    let sectors: BTreeMap<_, _> = w
        .campaign
        .sectors
        .iter()
        .filter(|(_, s)| s.conflict == c.id && s.nation == o.nation)
        .map(|(key, s)| (key.clone(), s.clone()))
        .collect();
    let deliveries = w
        .campaign_supply
        .deliveries
        .iter()
        .filter(|(key, _)| sectors.contains_key(*key))
        .map(|(key, d)| (key.clone(), d.clone()))
        .collect();
    let opening = Opening {
        sectors,
        control: BTreeMap::new(),
        supply: deliveries,
        orders: BTreeMap::from([(key.clone(), o.clone())]),
        assets: BTreeMap::from([(key, (order_air(w, c, o), order_naval(w, c, o)))]),
    };
    let own_contacts = contacts(w, c, &deployments, &opening, &k);
    let hostile = |d: &str| {
        if side {
            hold(c, &k, d) < front::HELD_BAND
        } else {
            hold(c, &k, d) > -front::HELD_BAND
        }
    };
    let target = o
        .target
        .as_ref()
        .filter(|d| own_contacts.contains_key(*d) && hostile(d))
        .cloned()
        .or_else(|| own_contacts.keys().find(|d| hostile(d)).cloned());
    // Observed opposing force bounds are evaluated with explicit broad
    // assumptions about unobserved quality/preparation. These are assessments,
    // not invented exact enemy ratings or a guarantee of future control.
    let estimate = |enemy_mass: f64, enemy_quality: f64, enemy_prepared: f64| -> (f64, f64) {
        let Some(d) = target.as_ref() else {
            return (0.0, 0.0);
        };
        let mut units = own_contacts[d].clone();
        units.push(Contribution {
            key: "forecast-opponent".into(),
            nation: o.nation,
            side: !side,
            mass: enemy_mass,
            maneuver_mass: enemy_mass,
            power: enemy_mass * enemy_quality,
            attack_power: None,
            ground: 1.0,
            quality: enemy_quality,
            fire: 0.0,
            recon: 0.0,
            air: 0.0,
            air_defense: 0.0,
            interdict: 0.0,
            mobility: 0.0,
            land: 1.0,
            strike: 1.0,
            river: false,
            preparation: enemy_prepared,
            rung: c.top_rung(!side),
            roe: Roe::Standard,
            approach: Approach::Hold,
            objective: Objective::Hold,
        });
        let result = contact_result(w, c, d, hold(c, &k, d), &units);
        let losses = result
            .losses
            .iter()
            .filter(|(key, _)| key.as_str() != "forecast-opponent")
            .map(|(_, value)| {
                deployments
                    .get(&(c.id, o.nation))
                    .map_or(0.0, |row| row.actual_loss(*value))
            })
            .sum();
        (
            (result.movement * if side { 1.0 } else { -1.0 }).max(0.0),
            losses,
        )
    };
    let favorable = estimate(enemy.low, 0.5, 0.0);
    let adverse = estimate(enemy.high, 2.0, 1.0);
    let (low, high) = (adverse.0.min(favorable.0), adverse.0.max(favorable.0));
    let reason = if force <= EPS {
        "Forces must complete deployment before fighting."
    } else if supply < 0.35 {
        "Supply is limiting combat and cohesion recovery."
    } else if readiness < 0.35 {
        "Cohesion is limiting the force; consolidate while supplied."
    } else if !attacking(o.approach) {
        "The current order prepares or withdraws; it does not launch an advance."
    } else if target.is_none() {
        "No fielded approach can reach opposing ground yet. Complete deployment or provide supported transport."
    } else if enemy.confidence < 0.45 {
        "Enemy estimates are broad. Reconnaissance can improve this assessment."
    } else if low <= EPS {
        "Enemy strength and prepared ground may stop this operation."
    } else {
        "Local concentration and supply can support an advance; the range reflects observed opposition."
    };
    Forecast {
        advance_low: low,
        advance_high: high,
        loss_low: favorable.1.min(adverse.1),
        loss_high: favorable.1.max(adverse.1),
        reason: reason.into(),
        period: "per day".into(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::{Belligerent, GameRules};
    fn physical_fixture(platform: &str, rung: u8, rounds: u32) -> WorldState {
        let mut w = crate::equipment::ammunition_operation_tests::fixture(platform);
        w.rules.operational_warfare = 1;
        w.conflicts[0].posture_mut(NationId::USA).unwrap().rung = rung;
        let mut order = OperationOrder::automatic(&w.conflicts[0], NationId::USA);
        order.air = AirMission::GroundSupport;
        w.campaign.orders.insert(order_key(1, NationId::USA), order);
        crate::equipment::ammunition_operation_tests::stock(&mut w, rounds);
        w
    }
    fn supplied_contacts(w: &WorldState) -> (String, Vec<Contribution>) {
        let c = &w.conflicts[0];
        let k = front::contested_set(w, c);
        let target =
            k.k.keys()
                .find(|d| {
                    w.districts.get(*d) == Some(&NationId::Canada)
                        && districts::adj_of(d)
                            .iter()
                            .any(|x| w.districts.get(x) == Some(&NationId::USA))
                })
                .expect("mapped Canadian border contact")
                .clone();
        let source = districts::adj_of(&target)
            .iter()
            .find(|d| w.districts.get(*d) == Some(&NationId::USA))
            .unwrap()
            .clone();
        let snapshot = operations::Snapshot::new(w);
        let mut opening = Opening {
            sectors: BTreeMap::new(),
            control: BTreeMap::new(),
            supply: BTreeMap::new(),
            orders: BTreeMap::new(),
            assets: BTreeMap::new(),
        };
        for (nation, district) in [(NationId::USA, source), (NationId::Canada, target.clone())] {
            let key = sector_key(c.id, nation, &district);
            opening.sectors.insert(
                key.clone(),
                Sector {
                    conflict: c.id,
                    nation,
                    district,
                    strength: snapshot.deployed(c.id, nation),
                    cohesion: 0.9,
                    preparation: 0.8,
                    isolated_days: 0,
                },
            );
            opening.supply.insert(
                key.clone(),
                crate::campaign_supply::SupplyDelivery {
                    key,
                    coverage: 1.0,
                    days: 0,
                    route: vec![],
                    reason: "controlled delivered service".into(),
                },
            );
            let order = order_for(w, c, nation);
            opening.assets.insert(
                order_key(c.id, nation),
                (order_air(w, c, &order), order_naval(w, c, &order)),
            );
            opening.orders.insert(order_key(c.id, nation), order);
        }
        let contacts = contacts(w, c, &snapshot.rows, &opening, &k);
        (target.clone(), contacts[&target].clone())
    }
    fn casualties(w: &WorldState, district: &str, units: &[Contribution], nation: NationId) -> f64 {
        let result = contact_result(w, &w.conflicts[0], district, -1.0, units);
        units
            .iter()
            .filter(|u| u.nation == nation)
            .map(|u| result.losses[&u.key])
            .sum()
    }
    #[test]
    fn physical_dry_ground_keeps_maneuver_and_observation_but_cannot_fire() {
        let wet = physical_fixture("ground_recon", 8, 100_000);
        let mut dry = wet.clone();
        let stock = dry
            .nation_mut(NationId::USA)
            .equipment
            .as_mut()
            .unwrap()
            .ammunition
            .as_mut()
            .unwrap();
        stock.stocks.clear();
        let (d, wet_units) = supplied_contacts(&wet);
        let (_, dry_units) = supplied_contacts(&dry);
        let own = |units: &[Contribution]| {
            units
                .iter()
                .find(|u| u.nation == NationId::USA)
                .unwrap()
                .clone()
        };
        let (firing, silent) = (own(&wet_units), own(&dry_units));
        assert!(firing.recon > 0.0 && firing.power > 0.0);
        assert_eq!(firing.recon, silent.recon);
        assert_eq!(
            firing.power, silent.power,
            "dry grounded personnel retain maneuver and observation"
        );
        assert!(casualties(&wet, &d, &wet_units, NationId::Canada) > 0.0);
        assert_eq!(casualties(&dry, &d, &dry_units, NationId::Canada), 0.0);
        let mut half = wet.clone();
        let requirements = crate::equipment::ammunition_overview(&half, NationId::USA).families;
        let stock = half
            .nation_mut(NationId::USA)
            .equipment
            .as_mut()
            .unwrap()
            .ammunition
            .as_mut()
            .unwrap();
        for r in requirements {
            stock.stocks.insert(r.family, r.required * 0.5);
        }
        let (_, half_units) = supplied_contacts(&half);
        // The month-to-day survival transform is deliberately nonlinear.
        // Compare the recovered monthly hazard, not a ratio of daily deaths.
        let victim_mass = wet_units
            .iter()
            .filter(|u| u.nation == NationId::Canada)
            .map(|u| u.mass)
            .sum::<f64>();
        let hazard = |deaths: f64| {
            1.0 - crate::exact::powf(
                1.0 - deaths / victim_mass,
                1.0 / clock::month_fraction(&wet),
            )
        };
        let ratio = hazard(casualties(&half, &d, &half_units, NationId::Canada))
            / hazard(casualties(&wet, &d, &wet_units, NationId::Canada));
        assert!(
            (ratio - 0.5).abs() < 1e-9,
            "compatible half stock must not be squared: {ratio}"
        );
    }
    #[test]
    fn physical_air_defense_requires_paid_interceptors_and_cannot_attack_ground() {
        let mut wet = physical_fixture("ground_air_defense", 8, 100_000);
        wet.conflicts[0].posture_mut(NationId::Canada).unwrap().rung = 6;
        let mut order = OperationOrder::automatic(&wet.conflicts[0], NationId::Canada);
        order.air = AirMission::GroundSupport;
        wet.campaign
            .orders
            .insert(order_key(1, NationId::Canada), order);
        let mut dry = wet.clone();
        dry.nation_mut(NationId::USA)
            .equipment
            .as_mut()
            .unwrap()
            .ammunition
            .as_mut()
            .unwrap()
            .stocks
            .clear();
        let (d, wet_units) = supplied_contacts(&wet);
        let (_, dry_units) = supplied_contacts(&dry);
        assert!(
            wet_units
                .iter()
                .find(|u| u.nation == NationId::USA)
                .unwrap()
                .air_defense
                > 0.0
        );
        assert_eq!(
            dry_units
                .iter()
                .find(|u| u.nation == NationId::USA)
                .unwrap()
                .air_defense,
            0.0
        );
        assert_eq!(casualties(&wet, &d, &wet_units, NationId::Canada), 0.0);
        assert!(
            casualties(&wet, &d, &wet_units, NationId::USA)
                < casualties(&dry, &d, &dry_units, NationId::USA)
        );
    }
    #[test]
    fn custom_strike_uses_paid_stores_and_installed_quality_without_buying_other_air_roles() {
        let wet = physical_fixture("air_light_attack", 6, 100_000);
        let (d, units) = supplied_contacts(&wet);
        let baseline = casualties(&wet, &d, &units, NationId::Canada);
        assert!(baseline > 0.0);
        assert_eq!(mission_assets(&wet, NationId::USA).0, 0.0);
        let mut stronger = wet.clone();
        let mut spec = crate::equipment::default_spec("air_light_attack");
        spec.components
            .insert("air_payload".into(), "air_payload_guided".into());
        spec.components
            .insert("air_avionics".into(), "air_avionics_digital".into());
        let profile = crate::equipment::design_preview(&stronger, NationId::USA, &spec)
            .profile
            .unwrap();
        let revision = stronger
            .nation_mut(NationId::USA)
            .equipment
            .as_mut()
            .unwrap()
            .revisions
            .get_mut("ammo-model")
            .unwrap();
        // Controlled alternative installed design; no campaign grants a refit.
        revision.specification_key = crate::equipment::specification_key(&spec);
        revision.spec = spec;
        revision.profile = profile;
        crate::equipment::ammunition_operation_tests::stock(&mut stronger, 100_000);
        let (_, strong_units) = supplied_contacts(&stronger);
        assert!(casualties(&stronger, &d, &strong_units, NationId::Canada) > baseline);
        for mission in [
            AirMission::None,
            AirMission::Interception,
            AirMission::Reconnaissance,
        ] {
            let mut idle = wet.clone();
            idle.campaign
                .orders
                .get_mut(&order_key(1, NationId::USA))
                .unwrap()
                .air = mission;
            let (_, idle_units) = supplied_contacts(&idle);
            assert_eq!(
                casualties(&idle, &d, &idle_units, NationId::Canada),
                0.0,
                "unsupported mission {mission:?}"
            );
            let row = &operations::view(&idle, NationId::USA).deployments[0];
            assert_eq!(
                row.actual_loss(1.0),
                0.0,
                "grounded aircraft are not deployed casualties"
            );
        }
    }
    #[test]
    fn partial_aircraft_launch_losses_match_campaign_pools_receipts_and_national_settlement() {
        let mut w = physical_fixture("air_light_attack", 6, 100_000);
        let requirements = crate::equipment::ammunition_overview(&w, NationId::USA).families;
        let stores = w
            .nation_mut(NationId::USA)
            .equipment
            .as_mut()
            .unwrap()
            .ammunition
            .as_mut()
            .unwrap();
        for use_ in requirements {
            stores.stocks.insert(use_.family, use_.required * 0.5);
        }
        let before = w.nation(NationId::USA).mil_strength;
        let mut snapshot = operations::Snapshot::new(&w);
        let exposure = snapshot.actual_loss(1, NationId::USA, 1.0);
        assert!(
            exposure > 0.0 && exposure < 1.0,
            "controlled partially launched fleet"
        );
        let mut opening = prepare(&mut w, &snapshot);
        for (key, sector) in &opening.sectors {
            opening.supply.insert(
                key.clone(),
                crate::campaign_supply::SupplyDelivery {
                    key: key.clone(),
                    coverage: 1.0,
                    days: 0,
                    route: vec![sector.district.clone()],
                    reason: "controlled delivered service".into(),
                },
            );
        }
        let mut conflicts = std::mem::take(&mut w.conflicts);
        let resolution = resolve(&mut w, &mut conflicts[0], &snapshot, &opening);
        for (nation, fraction) in resolution.losses {
            snapshot.record_loss(1, nation, fraction);
        }
        snapshot.settle(&mut w);
        let receipt = w.campaign.reports[&order_key(1, NationId::USA)].losses;
        assert!(
            receipt > 0.0,
            "aircraft actually launch into an opposed battle"
        );
        assert!((before - w.nation(NationId::USA).mil_strength - receipt).abs() < 1e-9);
        assert!(
            (accounted_force(&w, NationId::USA) - w.nation(NationId::USA).mil_strength).abs()
                < 1e-9,
            "settlement must not restore phantom locally lost aircraft at next reconciliation"
        );
    }
    fn fixture() -> (WorldState, Conflict, Vec<Contribution>) {
        let w = crate::init::world_1990(GameRules {
            daily_simulation: true,
            military_operations: true,
            operational_warfare: 1,
            ..Default::default()
        });
        let c = Conflict {
            id: 1,
            theatre: theatre::TheatreId::Gulf,
            side_a: vec![NationId::Iraq],
            side_b: vec![NationId::Iran],
            posture: vec![
                Belligerent::new(NationId::Iraq, 8, Objective::Seize),
                Belligerent::new(NationId::Iran, 8, Objective::Hold),
            ],
            control: 0.0,
            months: 0,
            quiet_months: 0,
            frozen_since: None,
            start_year: 1990,
            start_month: 1,
            origin_attacker: NationId::Iraq,
            invasion_declared: true,
            front: Default::default(),
            pockets: vec![],
            aim: None,
        };
        let a = Contribution {
            key: "attacker".into(),
            nation: NationId::Iraq,
            side: true,
            mass: 50.0,
            maneuver_mass: 50.0,
            power: 65.0,
            attack_power: None,
            ground: 1.0,
            quality: 1.3,
            fire: 0.0,
            recon: 0.0,
            air: 0.0,
            air_defense: 0.0,
            interdict: 0.0,
            mobility: 0.0,
            land: 1.0,
            strike: 1.0,
            river: false,
            preparation: 0.9,
            rung: 8,
            roe: Roe::Standard,
            approach: Approach::Breakthrough,
            objective: Objective::Seize,
        };
        let b = Contribution {
            key: "defender".into(),
            nation: NationId::Iran,
            side: false,
            mass: 5.0,
            maneuver_mass: 5.0,
            power: 5.0,
            ground: 1.0,
            quality: 1.0,
            preparation: 0.3,
            approach: Approach::Hold,
            objective: Objective::Hold,
            ..a.clone()
        };
        (w, c, vec![a, b])
    }
    #[test]
    fn difficult_terrain_and_crossing_stop_real_local_throughput() {
        let (w, c, units) = fixture();
        let mut pair = None;
        for d in w
            .districts
            .keys()
            .filter(|d| districts::terrain_of(d) == districts::TerrainClass::Mountain)
        {
            if let Some(flat) = w
                .districts
                .keys()
                .filter(|x| districts::terrain_of(x) == districts::TerrainClass::Lowland)
                .find(|x| (districts::area_of(x) / districts::area_of(d) - 1.0).abs() < 0.15)
            {
                pair = Some((d, flat));
                break;
            }
        }
        let (mountain, lowland) =
            pair.expect("real map contains similar-size mountain and lowland districts");
        let difficult = contact_result(&w, &c, mountain, -1.0, &units);
        let open = contact_result(&w, &c, lowland, -1.0, &units);
        assert!(
            open.movement > difficult.movement * 1.5 && difficult.movement > 0.0,
            "terrain must constrain total contact progress"
        );
        let mut river = units.clone();
        river[0].river = true;
        assert!(contact_result(&w, &c, lowland, -1.0, &river).movement < open.movement);
    }
    #[test]
    fn specialist_roles_affect_their_actual_local_outcomes_once() {
        let (w, c, units) = fixture();
        let d = "IQ-BA";
        let reference = contact_result(&w, &c, d, -1.0, &units);
        let mut artillery = units.clone();
        artillery[0].fire = 0.25;
        let shelling = contact_result(&w, &c, d, -1.0, &artillery);
        assert!(shelling.losses["defender"] > reference.losses["defender"]);
        assert_eq!(
            shelling.movement, reference.movement,
            "artillery has a fire role, not a duplicated movement bonus"
        );
        let mut mobility = units.clone();
        mobility[0].mobility = 0.25;
        assert!(contact_result(&w, &c, d, -1.0, &mobility).movement > reference.movement);
        let mut protected = units.clone();
        protected[1].air_defense = 0.35;
        assert_eq!(
            contact_result(&w, &c, d, -1.0, &protected).losses["defender"],
            reference.losses["defender"],
            "air defense cannot intercept ground fire"
        );
        protected[0].air = 0.7;
        let mut exposed = protected.clone();
        exposed[1].air_defense = 0.0;
        assert!(
            contact_result(&w, &c, d, -1.0, &protected).losses["defender"]
                < contact_result(&w, &c, d, -1.0, &exposed).losses["defender"]
        );
        let mut suppressed = units.clone();
        suppressed[1].rung = 4;
        assert!(
            contact_result(&w, &c, d, -1.0, &suppressed).losses["defender"]
                < reference.losses["defender"],
            "dispersed commitments preserve the capability gate"
        );
    }
    #[test]
    fn supported_transport_opens_one_remote_contact_without_copying_home_defenders() {
        let (mut w, mut c, _) = fixture();
        c.side_a = vec![NationId::UK];
        c.side_b = vec![NationId::France];
        c.origin_attacker = NationId::UK;
        c.theatre = theatre::TheatreId::WesternEurope;
        c.posture = vec![
            Belligerent::new(NationId::UK, 8, Objective::Seize),
            Belligerent::new(NationId::France, 8, Objective::Hold),
        ];
        w.conflicts.push(c.clone());
        let snapshot = operations::Snapshot::new(&w);
        let source = districts::list_of(NationId::UK)
            .iter()
            .find(|d| crate::logistics::has_terminal(d))
            .expect("mapped British gateway")
            .clone();
        let key = sector_key(1, NationId::UK, &source);
        let mass = snapshot.deployed(1, NationId::UK);
        let sector = Sector {
            conflict: 1,
            nation: NationId::UK,
            district: source.clone(),
            strength: mass,
            cohesion: 0.9,
            preparation: 0.8,
            isolated_days: 0,
        };
        let mut o = OperationOrder::automatic(&c, NationId::UK);
        o.naval = NavalMission::Transport;
        let mut opening = Opening {
            sectors: BTreeMap::from([(key.clone(), sector)]),
            control: BTreeMap::new(),
            supply: BTreeMap::from([(
                key.clone(),
                crate::campaign_supply::SupplyDelivery {
                    key,
                    coverage: 1.0,
                    days: 0,
                    route: vec![],
                    reason: "fixture service".into(),
                },
            )]),
            orders: BTreeMap::from([(order_key(1, NationId::UK), o)]),
            assets: BTreeMap::from([(order_key(1, NationId::UK), (0.0, 1.0))]),
        };
        let k = front::contested_set(&w, &c);
        let resolved = contacts(&w, &c, &snapshot.rows, &opening, &k);
        assert_eq!(resolved.keys().filter(|d|w.districts.get(*d)==Some(&NationId::France)).count(),1,"a naval campaign needs one supported foreign contact even when its base belongs to the theatre");
        assert!(
            (resolved.values().flatten().map(|u| u.mass).sum::<f64>() - mass).abs() < 1e-9,
            "home defense and landing share the same force"
        );
        w.campaign.sectors = opening.sectors.clone();
        w.campaign.orders = opening.orders.clone();
        w.campaign_supply.deliveries = opening.supply.clone();
        let forecast = assessment(
            &w,
            &c,
            &opening.orders[&order_key(1, NationId::UK)],
            mass,
            0.9,
            0.8,
            1.0,
            &EnemyEstimate {
                confidence: 1.0,
                ..Default::default()
            },
        );
        assert!(
            forecast.advance_high > 0.0,
            "the player assessment must include the same supported remote contact"
        );
        opening
            .orders
            .get_mut(&order_key(1, NationId::UK))
            .unwrap()
            .naval = NavalMission::None;
        assert!(
            contacts(&w, &c, &snapshot.rows, &opening, &k)
                .keys()
                .all(|d| w.districts.get(d) != Some(&NationId::France)),
            "no unrequested remote invasion"
        );
    }
}
