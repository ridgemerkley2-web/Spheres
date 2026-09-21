//! Reviewed tactical orders. Campaign contacts own target effects; operations
//! owns ammunition; this ledger owns only launched aircraft losses.
use crate::{
    airbases, arsenal, aviation, clock, equipment, front, operations,
    world::{Conflict, NationId, WorldState},
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
const EPS: f64 = 1e-10;
pub const MAX_ORDERS: usize = 8192;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MissionKind {
    SupportArmy,
    StrikeTarget,
    DefendSkies,
}
impl MissionKind {
    pub fn name(self) -> &'static str {
        match self {
            Self::SupportArmy => "Support army",
            Self::StrikeTarget => "Strike target",
            Self::DefendSkies => "Defend skies",
        }
    }
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MissionStatus {
    Queued,
    Flown,
    Blocked,
    Cancelled,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct MissionReport {
    pub day: i32,
    pub summary: String,
    pub aircraft: u32,
    pub sorties: f64,
    pub family: String,
    pub stores_used: f64,
    pub aircraft_lost: u32,
    pub applied_power: f64,
    pub contacted: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub defense: Option<AirDefenseEffect>,
}
/// Air and ground defenses share one physical loss settlement. These are
/// expected losses of this flight, never additional whole-aircraft kill claims.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AirDefenseEffect {
    pub opposing_missions: u32,
    pub prevented_power: f64,
    pub air_combat_expected_loss: f64,
    pub ground_defense_expected_loss: f64,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct MissionOrder {
    pub id: u32,
    pub nation: NationId,
    pub squadron: u32,
    pub conflict: u32,
    pub kind: MissionKind,
    pub target: String,
    pub issued_day: i32,
    pub launch_day: i32,
    pub status: MissionStatus,
    pub report: Option<MissionReport>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct MissionPlan {
    pub order: u32,
    pub nation: NationId,
    pub squadron: u32,
    pub conflict: u32,
    pub kind: MissionKind,
    pub target: String,
    pub revision: String,
    pub base: String,
    pub aircraft: u32,
    pub family: String,
    pub required: f64,
    pub coverage: f64,
    pub sorties: f64,
    pub power: f64,
    pub expected_loss: f64,
    pub service_days: u8,
    pub contacted: bool,
    pub applied_power: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub defense: Option<AirDefenseEffect>,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AirMissionsState {
    pub next_id: u32,
    pub orders: Vec<MissionOrder>,
    pub prepared_day: Option<i32>,
    pub settled_day: Option<i32>,
    pub plans: Vec<MissionPlan>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "action", rename_all = "snake_case", deny_unknown_fields)]
pub enum MissionCommand {
    Queue {
        squadron: u32,
        conflict: u32,
        kind: MissionKind,
        target: String,
    },
    Cancel {
        mission: u32,
    },
}
#[derive(Clone, Debug, Serialize)]
pub struct MissionQuote {
    pub valid: bool,
    pub reason: Option<String>,
    pub aircraft: u32,
    pub distance_km: f64,
    pub radius_km: f64,
    pub family: String,
    pub stores_required: f64,
    pub stores_available: f64,
    pub sorties: f64,
    pub force_share: f64,
    pub launch_day: i32,
    pub detail: String,
}
pub fn busy(w: &WorldState, id: NationId, squadron: u32) -> bool {
    w.air_missions.as_ref().is_some_and(|s| {
        s.orders
            .iter()
            .any(|o| o.nation == id && o.squadron == squadron && o.status == MissionStatus::Queued)
    })
}
fn squadron(w: &WorldState, id: NationId, key: u32) -> Option<&aviation::Squadron> {
    w.nation_opt(id)?
        .aviation
        .as_ref()?
        .squadrons
        .iter()
        .find(|s| s.id == key)
}
pub(crate) fn target_reason(
    w: &WorldState,
    id: NationId,
    c: &Conflict,
    kind: MissionKind,
    target: &str,
) -> Option<String> {
    let Some(posture) = c.posture_of(id) else {
        return Some("Join this conflict before ordering a mission.".into());
    };
    if posture.rung < 6 {
        return Some(
            "Authorize at least Air raid in this conflict before flying a combat mission.".into(),
        );
    }
    if kind == MissionKind::SupportArmy && posture.rung < 7 {
        return Some("Support army needs an active ground operation. Choose Strike target at Air raid, or review your conflict commitment.".into());
    }
    let k = front::contested_set(w, c);
    let Some((base_a, _)) = k.k.get(target) else {
        return Some("Choose a province in this conflict's contested theatre.".into());
    };
    let h = c
        .front
        .get(target)
        .map_or(if *base_a { 1.0 } else { -1.0 }, |v| *v as f64);
    let side = c.side_of(id).unwrap_or(false);
    if kind == MissionKind::DefendSkies {
        if (side && h <= -front::HELD_BAND) || (!side && h >= front::HELD_BAND) {
            return Some(
                "Choose a friendly-held or contested province to protect in this conflict.".into(),
            );
        }
    } else if (side && h >= front::HELD_BAND) || (!side && h <= -front::HELD_BAND) {
        return Some("Choose an enemy-held or contested province; friendly territory is not a strike target.".into());
    }
    if kind == MissionKind::SupportArmy
        && !w.campaign.sectors.values().any(|s| {
            s.conflict == c.id
                && s.nation == id
                && s.strength > EPS
                && (s.district == target
                    || crate::districts::adj_of(&s.district)
                        .iter()
                        .any(|d| d == target))
        })
    {
        return Some(
            "Move an army into contact with this province before assigning Support army.".into(),
        );
    }
    None
}
fn service_fraction(w: &WorldState, id: NationId, revision: &str) -> f64 {
    let n = w.nation(id);
    let day = clock::absolute_day(w);
    let Some(s) = &n.equipment else {
        return 0.0;
    };
    let fresh = s
        .maintenance_plan
        .as_ref()
        .and_then(|p| p.receipt.as_ref())
        .is_some_and(|r| r.day >= day.saturating_sub(1) && r.day <= day && r.custom_paid_bn > EPS);
    if !fresh {
        return 0.0;
    }
    let Some(h) = n
        .arsenal
        .held
        .iter()
        .find(|h| h.design_id.as_deref() == Some(revision))
    else {
        return 0.0;
    };
    let Some(p) = equipment::profile(n, revision) else {
        return 0.0;
    };
    let reference = arsenal::available_design_units(h) as f64 * p.reference_weight_bn;
    if reference > 0.0 {
        (arsenal::combat_value(n, h) / reference).clamp(0.0, 1.0)
    } else {
        0.0
    }
}
pub fn quote(
    w: &WorldState,
    id: NationId,
    sq: u32,
    conflict: u32,
    kind: MissionKind,
    target: &str,
) -> MissionQuote {
    quote_inner(w, id, sq, conflict, kind, target, true)
}
fn quote_inner(
    w: &WorldState,
    id: NationId,
    sq: u32,
    conflict: u32,
    kind: MissionKind,
    target: &str,
    check_busy: bool,
) -> MissionQuote {
    let mut q=MissionQuote{valid:false,reason:None,aircraft:0,distance_km:0.0,radius_km:0.0,family:String::new(),stores_required:0.0,stores_available:0.0,sorties:0.0,force_share:0.0,launch_day:clock::absolute_day(w).saturating_add(1),detail:"One next-day mission. Aircraft remain national property; this order reserves the squadron until launch or cancellation. All fronts share finite force allocation and compatible stores. Partial store coverage reduces competing sorties proportionally. Return flight is included in the radius; survivors need funded service before flying again. Rates are modeled game values.".into()};
    let result = (|| -> Result<(), String> {
        let unavailable = if check_busy {
            equipment::actor_refusal(w, id)
        } else {
            // Issuing an order requires current command authority. An already
            // authorized saved order launches for its own country after a
            // player switch as long as its physical conditions still hold.
            equipment::operational_refusal(w, id)
        };
        if let Some(r) = unavailable {
            return Err(r);
        }
        if !crate::campaign::enabled(w) {
            return Err(
                "Enable daily operational warfare before ordering tactical missions.".into(),
            );
        }
        if check_busy
            && w.air_missions
                .as_ref()
                .is_some_and(|s| s.orders.len() >= MAX_ORDERS || s.next_id == u32::MAX)
        {
            return Err("This campaign's mission ledger is full.".into());
        }
        let s = squadron(w, id, sq).ok_or("Choose a saved squadron of delivered aircraft.")?;
        q.aircraft = s.assigned;
        if s.assigned == 0 {
            return Err("Assign delivered aircraft to this squadron first.".into());
        }
        if check_busy && busy(w, id, sq) {
            return Err(
                "This squadron already has a queued mission. Cancel it or wait for its result."
                    .into(),
            );
        }
        if let Some(r) = airbases::squadron_blocker(w, id, s) {
            return Err(r);
        }
        let c = w.conflict(conflict).ok_or("Choose an active conflict.")?;
        if let Some(r) = target_reason(w, id, c, kind, target) {
            return Err(r);
        }
        q.distance_km = airbases::target_in_range(w, id, s, target)?;
        let n = w.nation(id);
        let r = &n.equipment.as_ref().unwrap().revisions[&s.revision];
        let fighter = r.spec.platform == "air_fighter";
        if (kind == MissionKind::DefendSkies) != fighter {
            return Err(if fighter {
                "This fighter is equipped for Defend skies. Assign an attack aircraft for Support army or Strike target."
            } else {
                "Defend skies needs a delivered fighter model with air-to-air missiles. Attack aircraft cannot intercept."
            }.into());
        }
        q.radius_km = airbases::range_km(&r.spec);
        let a = r
            .profile
            .aviation
            .as_ref()
            .ok_or("This revision is not an aircraft.")?;
        q.family = a.store_family.clone();
        if kind == MissionKind::DefendSkies {
            q.detail = "One next-day patrol over this province. Paid fighters share finite missiles across all missions and reduce incoming Support army and Strike target missions here. Ground air defense remains separate. Each aircraft can be lost only once. A quiet patrol still consumes its reviewed stores and requires funded service. Return flight is included in the radius. Rates are modeled game values.".into();
        }
        q.force_share = operations::nominal_share(w, id, conflict);
        if q.force_share <= EPS {
            return Err("Allocate national force to this conflict before flying a mission.".into());
        }
        let service = service_fraction(w, id, &s.revision);
        if service <= EPS {
            return Err("No funded aircraft upkeep is available. Set fleet support, provide Defense authority and advance a day.".into());
        }
        q.sorties = s.assigned as f64
            * service
            * a.sorties_per_aircraft_month
            * q.force_share
            * clock::month_fraction(w);
        q.stores_required = q.sorties * a.stores_per_sortie;
        q.stores_available = n
            .equipment
            .as_ref()
            .and_then(|e| e.ammunition.as_ref())
            .and_then(|a| a.stocks.get(&q.family))
            .copied()
            .unwrap_or(0.0);
        if q.stores_available <= EPS {
            return Err(format!("No compatible {} stores are delivered. Fund support or buy a compatible company shipment before launch.",q.family));
        }
        Ok(())
    })();
    q.reason = result.err();
    q.valid = q.reason.is_none();
    q
}
pub fn refusal(w: &WorldState, id: NationId, order: &MissionCommand) -> Option<String> {
    if let Some(r) = equipment::actor_refusal(w, id) {
        return Some(r);
    }
    match order {
        MissionCommand::Queue {
            squadron,
            conflict,
            kind,
            target,
        } => quote(w, id, *squadron, *conflict, *kind, target).reason,
        MissionCommand::Cancel { mission } => w
            .air_missions
            .as_ref()
            .and_then(|s| s.orders.iter().find(|o| o.id == *mission && o.nation == id))
            .map_or(
                Some("This mission does not belong to this country.".into()),
                |o| {
                    if active_plans(w).any(|p| p.order == o.id) {
                        Some("This mission has entered today's launch settlement and cannot be cancelled.".into())
                    } else {
                        (o.status != MissionStatus::Queued)
                            .then(|| "Only a queued mission can be cancelled.".into())
                    }
                },
            ),
    }
}
pub fn apply(w: &mut WorldState, id: NationId, order: &MissionCommand) -> Result<(), String> {
    if let Some(r) = refusal(w, id, order) {
        return Err(r);
    }
    let day = clock::absolute_day(w);
    let s = w.air_missions.get_or_insert_with(|| AirMissionsState {
        next_id: 1,
        ..Default::default()
    });
    match order {
        MissionCommand::Queue {
            squadron,
            conflict,
            kind,
            target,
        } => {
            let key = s.next_id;
            s.next_id += 1;
            s.orders.push(MissionOrder {
                id: key,
                nation: id,
                squadron: *squadron,
                conflict: *conflict,
                kind: *kind,
                target: target.clone(),
                issued_day: day,
                launch_day: day.saturating_add(1),
                status: MissionStatus::Queued,
                report: None,
            });
        }
        MissionCommand::Cancel { mission } => {
            let o = s.orders.iter_mut().find(|o| o.id == *mission).unwrap();
            o.status = MissionStatus::Cancelled;
            o.report = Some(empty_report(
                day,
                "Cancelled before launch. No stores spent or aircraft lost.",
            ));
        }
    }
    Ok(())
}
fn empty_report(day: i32, summary: &str) -> MissionReport {
    MissionReport {
        day,
        summary: summary.into(),
        aircraft: 0,
        sorties: 0.0,
        family: String::new(),
        stores_used: 0.0,
        aircraft_lost: 0,
        applied_power: 0.0,
        contacted: false,
        defense: None,
    }
}
fn reference(n: &crate::world::Nation) -> f64 {
    n.arsenal
        .held
        .iter()
        .map(|h| {
            h.design_id
                .as_deref()
                .and_then(|id| equipment::profile(n, id))
                .map_or_else(
                    || {
                        arsenal::DECK
                            .get(h.kit as usize)
                            .map_or(0.0, |d| h.units.max(0.0) * d.unit_cost)
                    },
                    |p| arsenal::available_design_units(h) as f64 * p.reference_weight_bn,
                )
        })
        .sum::<f64>()
        .max(EPS)
}
fn mission_factor(a: &equipment::AviationProfile, kind: MissionKind) -> f64 {
    if kind == MissionKind::DefendSkies {
        a.intercept_factor
    } else {
        a.strike_factor
    }
}
/// Freeze eligibility before either side consumes ammunition or loses force.
pub(crate) fn prepare(w: &mut WorldState) {
    let day = clock::absolute_day(w);
    let Some(s) = &w.air_missions else {
        return;
    };
    if s.prepared_day.is_some_and(|d| d >= day) || !crate::campaign::enabled(w) {
        return;
    }
    let pending: Vec<_> = s
        .orders
        .iter()
        .filter(|o| o.status == MissionStatus::Queued && o.launch_day <= day)
        .cloned()
        .collect();
    let mut plans = Vec::new();
    let mut blocked = Vec::new();
    for o in pending {
        let q = quote_inner(
            w, o.nation, o.squadron, o.conflict, o.kind, &o.target, false,
        );
        if let Some(r) = q.reason {
            blocked.push((o.id, r));
            continue;
        }
        let n = w.nation(o.nation);
        if n.equipment
            .as_ref()
            .and_then(|e| e.ammunition.as_ref())
            .and_then(|a| a.last_consumption.as_ref())
            .is_some_and(|r| r.day >= day)
        {
            blocked.push((o.id,"Today's shared ammunition settlement is closed; queue another mission for tomorrow.".into()));
            continue;
        }
        let sq = squadron(w, o.nation, o.squadron).unwrap();
        let b = airbases::base(w, sq.base.as_deref().unwrap()).unwrap();
        let r = &n.equipment.as_ref().unwrap().revisions[&sq.revision];
        let a = r.profile.aviation.as_ref().unwrap();
        let ready = service_fraction(w, o.nation, &sq.revision);
        let power = n.mil_strength.max(0.0)
            * q.force_share
            * sq.assigned as f64
            * r.profile.reference_weight_bn
            / reference(n)
            * ready
            * mission_factor(a, o.kind);
        plans.push(MissionPlan {
            order: o.id,
            nation: o.nation,
            squadron: o.squadron,
            conflict: o.conflict,
            kind: o.kind,
            target: o.target,
            revision: sq.revision.clone(),
            base: b.id.clone(),
            aircraft: sq.assigned,
            family: q.family,
            required: q.stores_required,
            coverage: 1.0,
            sorties: q.sorties,
            power,
            expected_loss: 0.0,
            service_days: if b.support_level >= 1 { 1 } else { 2 },
            contacted: false,
            applied_power: 0.0,
            defense: None,
        });
    }
    plans.sort_by_key(|p| p.order);
    let mut totals = BTreeMap::<(NationId, String), f64>::new();
    for p in &plans {
        *totals.entry((p.nation, p.family.clone())).or_default() += p.required;
    }
    for p in &mut plans {
        let stock = w
            .nation(p.nation)
            .equipment
            .as_ref()
            .and_then(|e| e.ammunition.as_ref())
            .and_then(|a| a.stocks.get(&p.family))
            .copied()
            .unwrap_or(0.0);
        p.coverage = (stock / totals[&(p.nation, p.family.clone())].max(EPS)).clamp(0.0, 1.0);
    }
    let s = w.air_missions.as_mut().unwrap();
    s.prepared_day = Some(day);
    s.plans = plans;
    for (key, reason) in blocked {
        let o = s.orders.iter_mut().find(|o| o.id == key).unwrap();
        o.status = MissionStatus::Blocked;
        o.report = Some(empty_report(
            day,
            &format!("Mission held: {reason} No launch, stores or aircraft loss."),
        ));
    }
}
pub(crate) fn active_plans(w: &WorldState) -> impl Iterator<Item = &MissionPlan> {
    let day = clock::absolute_day(w);
    w.air_missions
        .iter()
        .filter(move |s| s.prepared_day == Some(day) && s.settled_day != Some(day))
        .flat_map(|s| &s.plans)
}
pub(crate) fn requirements(w: &WorldState, id: NationId) -> BTreeMap<String, f64> {
    let mut out = BTreeMap::new();
    for p in active_plans(w).filter(|p| p.nation == id) {
        *out.entry(p.family.clone()).or_default() += p.required;
    }
    out
}
pub(crate) fn exposure(w: &WorldState, id: NationId, conflict: u32) -> f64 {
    active_plans(w)
        .filter(|p| p.nation == id && p.conflict == conflict && p.kind != MissionKind::DefendSkies)
        .map(|p| p.power * p.coverage)
        .sum()
}
pub(crate) fn targets(w: &WorldState, conflict: u32) -> Vec<String> {
    active_plans(w)
        .filter(|p| p.conflict == conflict)
        .map(|p| p.target.clone())
        .collect()
}
/// Resolve each defended area from the immutable opening launch plans. All
/// defenders share all opposing strike flights proportionally; looping over
/// squadrons never grants another copy of a target or its available aircraft.
fn interceptions(w: &WorldState, c: &Conflict, target: &str) -> BTreeMap<u32, AirDefenseEffect> {
    let plans: Vec<_> = active_plans(w)
        .filter(|p| p.conflict == c.id && p.target == target)
        .collect();
    let mut effects = BTreeMap::new();
    for side in [true, false] {
        let defenders: Vec<_> = plans
            .iter()
            .copied()
            .filter(|p| p.kind == MissionKind::DefendSkies && c.side_of(p.nation) == Some(side))
            .collect();
        let attackers: Vec<_> = plans
            .iter()
            .copied()
            .filter(|p| p.kind != MissionKind::DefendSkies && c.side_of(p.nation) == Some(!side))
            .collect();
        let defense: f64 = defenders.iter().map(|p| p.power * p.coverage).sum();
        let attack: f64 = attackers.iter().map(|p| p.power * p.coverage).sum();
        let suppression = if defense > EPS && attack > EPS {
            (0.85 * defense / (defense + attack)).clamp(0.0, 0.85)
        } else {
            0.0
        };
        for p in defenders {
            let share = p.power * p.coverage / defense.max(EPS);
            let launched = (p.sorties * p.coverage).min(p.aircraft as f64);
            effects.insert(
                p.order,
                AirDefenseEffect {
                    opposing_missions: attackers.len() as u32,
                    prevented_power: attack * suppression * share,
                    air_combat_expected_loss: if attack > EPS {
                        launched * 0.08 * attack / (defense + attack).max(EPS)
                    } else {
                        0.0
                    },
                    ground_defense_expected_loss: 0.0,
                },
            );
        }
        if defense > EPS {
            for p in attackers {
                let launched = (p.sorties * p.coverage).min(p.aircraft as f64);
                effects.insert(
                    p.order,
                    AirDefenseEffect {
                        opposing_missions: plans
                            .iter()
                            .filter(|p| {
                                p.kind == MissionKind::DefendSkies
                                    && c.side_of(p.nation) == Some(side)
                            })
                            .count() as u32,
                        prevented_power: p.power * p.coverage * suppression,
                        air_combat_expected_loss: launched * 0.12 * suppression,
                        ground_defense_expected_loss: 0.0,
                    },
                );
            }
        }
    }
    effects
}
/// The distinct aircraft, shared allocation, age, upkeep and family coverage
/// are already included. No separate copy per ground sector or target.
pub(crate) fn fire(w: &WorldState, c: &Conflict, target: &str, side: bool) -> (f64, f64) {
    let mut strike = 0.0;
    let mut support = 0.0;
    let intercepted = interceptions(w, c, target);
    for p in active_plans(w).filter(|p| {
        p.conflict == c.id
            && p.target == target
            && c.side_of(p.nation) == Some(side)
            && p.kind != MissionKind::DefendSkies
    }) {
        let v = (p.power * p.coverage
            - intercepted.get(&p.order).map_or(0.0, |e| e.prevented_power))
        .max(0.0);
        strike += v;
        if p.kind == MissionKind::SupportArmy {
            support += v;
        }
    }
    (strike, support)
}
/// Record whether the actual campaign contact contained opponents. Defense
/// comes from the same opening snapshot of weapons with paid ammunition.
pub(crate) fn record_contact(
    w: &mut WorldState,
    c: &Conflict,
    target: &str,
    opposing: &[(NationId, f64)],
) {
    let day = clock::absolute_day(w);
    let intercepted = interceptions(w, c, target);
    let protection: BTreeMap<String, u8> = w
        .airbases
        .iter()
        .flat_map(|s| &s.bases)
        .map(|b| (b.id.clone(), b.protection_level))
        .collect();
    let Some(s) = &mut w.air_missions else {
        return;
    };
    if s.prepared_day != Some(day) || s.settled_day == Some(day) {
        return;
    }
    for p in s
        .plans
        .iter_mut()
        .filter(|p| p.conflict == c.id && p.target == target)
    {
        // Assignment, rather than addition, also makes repeated contact
        // inspection harmless before the single physical settlement.
        p.defense = intercepted.get(&p.order).cloned();
        p.contacted = false;
        p.applied_power = 0.0;
        p.expected_loss = 0.0;
        if p.kind == MissionKind::DefendSkies {
            if let Some(e) = &p.defense {
                p.contacted = e.opposing_missions > 0;
                p.applied_power = e.prevented_power;
                p.expected_loss = e.air_combat_expected_loss.min(p.aircraft as f64);
            }
            continue;
        }
        let enemies: Vec<_> = opposing
            .iter()
            .filter(|(id, _)| c.side_of(*id) != c.side_of(p.nation))
            .collect();
        let air_loss = p
            .defense
            .as_ref()
            .map_or(0.0, |e| e.air_combat_expected_loss);
        let prevented = p.defense.as_ref().map_or(0.0, |e| e.prevented_power);
        p.contacted =
            !enemies.is_empty() || p.defense.as_ref().is_some_and(|e| e.opposing_missions > 0);
        if !enemies.is_empty() {
            p.applied_power = (p.power * p.coverage - prevented).max(0.0);
        }
        let defense = enemies
            .iter()
            .map(|(_, v)| *v)
            .fold(0.0, f64::max)
            .clamp(0.0, 0.7);
        let launched = (p.sorties * p.coverage).min(p.aircraft as f64);
        let protect = 1.0 - 0.08 * protection.get(&p.base).copied().unwrap_or(0).min(5) as f64;
        let ground_loss = if enemies.is_empty() {
            0.0
        } else {
            (launched - air_loss).max(0.0) * (0.003 + 0.06 * defense) * protect
        };
        if let Some(e) = &mut p.defense {
            e.ground_defense_expected_loss = ground_loss;
        }
        p.expected_loss = (air_loss + ground_loss).min(p.aircraft as f64);
    }
}
/// After the shared ammunition and ground-equipment settlement.
pub(crate) fn settle(w: &mut WorldState) {
    let day = clock::absolute_day(w);
    let Some(s) = &w.air_missions else {
        return;
    };
    if s.prepared_day != Some(day) || s.settled_day == Some(day) {
        return;
    }
    let plans = s.plans.clone();
    let mut family_debits = BTreeMap::<(NationId, String), f64>::new();
    for p in &plans {
        *family_debits
            .entry((p.nation, p.family.clone()))
            .or_default() += p.required * p.coverage;
    }
    let mut reports = Vec::new();
    for p in plans {
        let paid = w
            .nation_opt(p.nation)
            .and_then(|n| n.equipment.as_ref())
            .and_then(|e| e.ammunition.as_ref())
            .and_then(|a| a.last_consumption.as_ref())
            .filter(|r| r.day == day)
            .is_some_and(|r| {
                r.used.get(&p.family).copied().unwrap_or(0.0) + EPS
                    >= family_debits[&(p.nation, p.family.clone())]
            });
        if !paid {
            reports.push((
                p.order,
                MissionStatus::Blocked,
                empty_report(
                    day,
                    "Shared ammunition settlement was unavailable. No aircraft were debited.",
                ),
            ));
            continue;
        }
        let n = w.nation_mut(p.nation);
        let mut lost = 0;
        if let Some(h) = n
            .arsenal
            .held
            .iter_mut()
            .find(|h| h.design_id.as_deref() == Some(&p.revision))
        {
            let loss = (p.expected_loss + h.loss_remainder)
                .min(p.aircraft as f64)
                .min(arsenal::available_design_units(h) as f64);
            if p.expected_loss > 0.0 {
                lost = loss.floor() as u32;
                h.units -= lost as f64;
                h.loss_remainder = (loss - lost as f64).clamp(0.0, 1.0 - f64::EPSILON);
            }
        }
        if let Some(sq) = n
            .aviation
            .as_mut()
            .and_then(|a| a.squadrons.iter_mut().find(|s| s.id == p.squadron))
        {
            sq.assigned = sq.assigned.saturating_sub(lost);
            if p.sorties * p.coverage > EPS {
                sq.service_days_left = p.service_days;
            }
        }
        aviation::reconcile(n);
        let outcome = if p.kind == MissionKind::DefendSkies {
            if p.contacted {
                "Intercepted hostile aircraft; the prevented strike power is recorded below."
            } else {
                "Patrol completed without hostile aircraft in this area. No ground attack was made."
            }
        } else if p.contacted {
            if p.defense.is_some() {
                "Resolved against the campaign contact and defending fighters; ground battle results remain in Operations."
            } else {
                "Applied to the campaign contact; ground battle results remain in Operations."
            }
        } else {
            "No opposing formation remained in contact; no target effect was applied."
        };
        reports.push((p.order, MissionStatus::Flown, MissionReport {
            day,
            summary: format!("{} at {}: {:.2} sortie equivalents, {:.2} {} stores, {} aircraft lost. {} Survivors need {} funded service day(s).",
                p.kind.name(), p.target, p.sorties * p.coverage, p.required * p.coverage,
                p.family, lost, outcome, p.service_days),
            aircraft: p.aircraft, sorties: p.sorties * p.coverage, family: p.family,
            stores_used: p.required * p.coverage, aircraft_lost: lost,
            applied_power: p.applied_power, contacted: p.contacted, defense: p.defense,
        }));
    }
    let s = w.air_missions.as_mut().unwrap();
    s.settled_day = Some(day);
    for (id, status, report) in reports {
        if let Some(o) = s.orders.iter_mut().find(|o| o.id == id) {
            o.status = status;
            o.report = Some(report);
        }
    }
}
pub fn validate(w: &WorldState) -> Result<(), String> {
    let Some(s) = &w.air_missions else {
        return Ok(());
    };
    let day = clock::absolute_day(w);
    let fail = || "Invalid saved tactical mission ledger.".to_string();
    if s.next_id == 0
        || s.orders.len() > MAX_ORDERS
        || s.plans.len() > s.orders.len()
        || (!s.plans.is_empty() && s.prepared_day.is_none())
        || s.prepared_day.is_some_and(|d| d > day)
        || s.settled_day.is_some_and(|d| Some(d) > s.prepared_day)
    {
        return Err(fail());
    }
    let mut ids = BTreeSet::new();
    let mut busy = BTreeSet::new();
    let mut reported = BTreeMap::<(NationId, String), f64>::new();
    for o in &s.orders {
        if o.id == 0
            || o.id >= s.next_id
            || !ids.insert(o.id)
            || w.nation_opt(o.nation).is_none()
            || !w.districts.contains_key(&o.target)
            || o.issued_day > day
            || o.launch_day != o.issued_day.saturating_add(1)
        {
            return Err(fail());
        }
        if o.status == MissionStatus::Queued
            && (o.report.is_some()
                || !busy.insert((o.nation, o.squadron))
                || squadron(w, o.nation, o.squadron).is_none())
        {
            return Err(fail());
        }
        if o.status != MissionStatus::Queued && o.report.is_none() {
            return Err(fail());
        }
        if let Some(r) = &o.report {
            if r.day < o.issued_day
                || r.day > day
                || r.aircraft_lost > r.aircraft
                || [r.sorties, r.stores_used, r.applied_power]
                    .iter()
                    .any(|v| !v.is_finite() || *v < 0.0)
                || (o.status != MissionStatus::Flown
                    && (r.aircraft != 0
                        || r.stores_used != 0.0
                        || r.aircraft_lost != 0
                        || r.applied_power != 0.0
                        || r.defense.is_some()))
                || r.defense
                    .as_ref()
                    .is_some_and(|e| !valid_defense_effect(e, r.aircraft, r.contacted))
            {
                return Err(fail());
            }
        }
        if let Some(r) = o
            .report
            .as_ref()
            .filter(|_| o.status == MissionStatus::Flown)
        {
            if r.day < o.launch_day
                || r.aircraft == 0
                || !matches!(
                    r.family.as_str(),
                    "air_bomb_unguided" | "air_bomb_guided" | "air_missile_short_range"
                )
                || ((o.kind == MissionKind::DefendSkies) != (r.family == "air_missile_short_range"))
                || (o.kind == MissionKind::DefendSkies && r.contacted && r.defense.is_none())
                || r.sorties <= 0.0
                || r.stores_used <= 0.0
                || (!r.contacted && r.applied_power != 0.0)
            {
                return Err(fail());
            }
            *reported.entry((o.nation, r.family.clone())).or_default() += r.stores_used;
        }
    }
    let mut plans = BTreeSet::new();
    if s.orders.iter().any(|o| {
        o.status == MissionStatus::Queued
            && s.prepared_day.is_some_and(|d| o.launch_day <= d)
            && !s.plans.iter().any(|p| p.order == o.id)
    }) {
        return Err(fail());
    }
    let mut family_requirements = BTreeMap::<(NationId, String), f64>::new();
    for p in &s.plans {
        *family_requirements
            .entry((p.nation, p.family.clone()))
            .or_default() += p.required;
    }
    for p in &s.plans {
        let Some(o) = s.orders.iter().find(|o| o.id == p.order) else {
            return Err(fail());
        };
        if !plans.insert(p.order)
            || p.nation != o.nation
            || p.squadron != o.squadron
            || p.conflict != o.conflict
            || p.kind != o.kind
            || p.target != o.target
            || p.aircraft == 0
            || !(0.0..=1.0).contains(&p.coverage)
            || p.service_days == 0
            || p.service_days > 2
            || [
                p.required,
                p.sorties,
                p.power,
                p.expected_loss,
                p.applied_power,
            ]
            .iter()
            .any(|v| !v.is_finite() || *v < 0.0)
            || p.expected_loss > p.aircraft as f64
            || p.applied_power > p.power * p.coverage + EPS
            || (!p.contacted && (p.applied_power != 0.0 || p.expected_loss != 0.0))
            || equipment::ammo_def(&p.family).is_none()
            || p.defense.as_ref().is_some_and(|e| {
                !valid_defense_effect(e, p.aircraft, p.contacted)
                    || (e.air_combat_expected_loss + e.ground_defense_expected_loss
                        - p.expected_loss)
                        .abs()
                        > EPS
                    || e.prevented_power > p.power * p.coverage + EPS
                    || (p.kind == MissionKind::DefendSkies
                        && (e.ground_defense_expected_loss != 0.0
                            || (p.applied_power - e.prevented_power).abs() > EPS))
            })
        {
            return Err(fail());
        }
        let Some(revision) = w
            .nation(p.nation)
            .equipment
            .as_ref()
            .and_then(|e| e.revisions.get(&p.revision))
        else {
            return Err(fail());
        };
        let Some(a) = &revision.profile.aviation else {
            return Err(fail());
        };
        if a.store_family != p.family
            || ((p.kind == MissionKind::DefendSkies) != (revision.spec.platform == "air_fighter"))
            || (p.required - p.sorties * a.stores_per_sortie).abs() > EPS
            || airbases::base(w, &p.base).is_none()
            || p.required <= 0.0
            || p.sorties <= 0.0
            || p.power <= 0.0
            || p.coverage <= 0.0
            || o.launch_day > s.prepared_day.unwrap_or(i32::MIN)
        {
            return Err(fail());
        }
        if s.settled_day != s.prepared_day {
            let stores = w
                .nation(p.nation)
                .equipment
                .as_ref()
                .and_then(|e| e.ammunition.as_ref())
                .ok_or_else(fail)?;
            let coverage = (stores.stocks.get(&p.family).copied().unwrap_or(0.0)
                / family_requirements[&(p.nation, p.family.clone())].max(EPS))
            .clamp(0.0, 1.0);
            if s.prepared_day != Some(day)
                || p.contacted
                || p.applied_power != 0.0
                || p.expected_loss != 0.0
                || p.defense.is_some()
                || (p.coverage - coverage).abs() > EPS
                || stores
                    .last_consumption
                    .as_ref()
                    .is_some_and(|r| r.day >= day)
            {
                return Err(fail());
            }
            let Some(sq) = squadron(w, p.nation, p.squadron) else {
                return Err(fail());
            };
            if o.status != MissionStatus::Queued
                || sq.assigned != p.aircraft
                || sq.revision != p.revision
                || sq.base.as_deref() != Some(p.base.as_str())
                || sq.transit.is_some()
                || sq.service_days_left > 0
            {
                return Err(fail());
            }
            let q = quote_inner(
                w, p.nation, p.squadron, p.conflict, p.kind, &p.target, false,
            );
            let expected_power = w.nation(p.nation).mil_strength.max(0.0)
                * q.force_share
                * p.aircraft as f64
                * revision.profile.reference_weight_bn
                / reference(w.nation(p.nation))
                * service_fraction(w, p.nation, &p.revision)
                * mission_factor(a, p.kind);
            if !q.valid
                || p.service_days
                    != if airbases::base(w, &p.base).unwrap().support_level >= 1 { 1 } else { 2 }
                || (p.required - q.stores_required).abs() > EPS
                || (p.sorties - q.sorties).abs() > EPS
                || (p.power - expected_power).abs() > EPS * expected_power.max(1.0)
            {
                return Err(fail());
            }
        } else if o.status == MissionStatus::Flown {
            let r = o.report.as_ref().ok_or_else(fail)?;
            if r.day != s.prepared_day.unwrap_or(i32::MIN)
                || r.aircraft != p.aircraft
                || r.family != p.family
                || (r.sorties - p.sorties * p.coverage).abs() > EPS
                || (r.stores_used - p.required * p.coverage).abs() > EPS
                || r.contacted != p.contacted
                || r.applied_power != p.applied_power
                || r.defense != p.defense
            {
                return Err(fail());
            }
        }
    }
    for ((id, family), amount) in reported {
        let consumed = w
            .nation(id)
            .equipment
            .as_ref()
            .and_then(|e| e.ammunition.as_ref())
            .and_then(|a| a.consumed.get(&family))
            .copied()
            .unwrap_or(0.0);
        if amount > consumed + EPS * consumed.max(1.0) {
            return Err(fail());
        }
    }
    Ok(())
}

fn valid_defense_effect(e: &AirDefenseEffect, aircraft: u32, contacted: bool) -> bool {
    let finite = [
        e.prevented_power,
        e.air_combat_expected_loss,
        e.ground_defense_expected_loss,
    ]
    .iter()
    .all(|v| v.is_finite() && *v >= 0.0);
    finite
        && e.opposing_missions as usize <= MAX_ORDERS
        && e.air_combat_expected_loss + e.ground_defense_expected_loss <= aircraft as f64 + EPS
        && (contacted
            || (e.opposing_missions == 0
                && e.prevented_power == 0.0
                && e.air_combat_expected_loss == 0.0
                && e.ground_defense_expected_loss == 0.0))
        && (e.opposing_missions > 0
            || (e.prevented_power == 0.0 && e.air_combat_expected_loss == 0.0))
}

#[cfg(test)]
#[path = "airmissions_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "airmissions_defense_tests.rs"]
mod defense_tests;
