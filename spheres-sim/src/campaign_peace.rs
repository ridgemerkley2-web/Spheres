//! Limited war, coalition consent and occupation. All constants are game rules.
//! Stock, cash and sovereignty remain owned by their existing settlement APIs.
use crate::{campaign, clock, control, districts, operations, world::*};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum WarAim {
    #[default]
    Expel,
    Recover {
        districts: Vec<String>,
    },
    Concession,
    GovernmentChange,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Terms {
    Ceasefire,
    Cede { districts: Vec<String> },
    Reparations { share_bp: u16 },
    Transition,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OccupationPolicy {
    #[default]
    Restraint,
    Security,
    Reconstruction,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum PeaceOrder {
    SetAim {
        conflict: u32,
        aim: WarAim,
    },
    Propose {
        conflict: u32,
        terms: Terms,
    },
    Respond {
        offer: u64,
        accept: bool,
    },
    Garrison {
        conflict: u32,
        share_bp: u16,
        policy: OccupationPolicy,
    },
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GarrisonPolicy {
    pub share_bp: u16,
    pub policy: OccupationPolicy,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Offer {
    pub id: u64,
    pub conflict: u32,
    pub from: NationId,
    pub loser: NationId,
    pub terms: Terms,
    pub participants: Vec<NationId>,
    pub approved: Vec<NationId>,
    #[serde(default)]
    pub side_a: Vec<NationId>,
    pub issued_day: i32,
    pub expires_day: i32,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PeaceRecord {
    pub offer: Offer,
    pub day: i32,
    pub outcome: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Occupation {
    pub district: String,
    pub conflict: u32,
    pub occupier: NationId,
    pub owner: NationId,
    pub days: u32,
    pub resistance: f64,
    pub coverage: f64,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PeaceState {
    pub aims: BTreeMap<String, WarAim>,
    pub garrisons: BTreeMap<String, GarrisonPolicy>,
    pub offers: Vec<Offer>,
    pub history: Vec<PeaceRecord>,
    pub occupation: BTreeMap<String, Occupation>,
    pub next_id: u64,
    pub last_day: Option<i32>,
    pub last_proposal: BTreeMap<String, i32>,
}
impl PeaceState {
    pub fn is_empty(&self) -> bool {
        self.aims.is_empty()
            && self.garrisons.is_empty()
            && self.offers.is_empty()
            && self.history.is_empty()
            && self.occupation.is_empty()
            && self.last_day.is_none()
    }
}
fn key(cid: u32, n: NationId) -> String {
    format!("{cid}:{n:?}")
}
pub fn price(order: &PeaceOrder) -> f64 {
    if matches!(order, PeaceOrder::SetAim { .. }) {
        3.0
    } else {
        0.0
    }
}
pub fn garrison_share(w: &WorldState, cid: u32, n: NationId) -> f64 {
    w.campaign_peace
        .garrisons
        .get(&key(cid, n))
        .map_or(0.0, |g| g.share_bp.min(5000) as f64 / 10000.0)
}
fn garrison_key(cid: u32, n: NationId, d: &str) -> String {
    format!("garrison:{cid}:{n:?}:{d}")
}
pub(crate) fn garrison_requests(
    w: &WorldState,
    snapshot: &operations::Snapshot,
) -> Vec<crate::campaign_supply::SupplyRequest> {
    let mut requests = vec![];
    for ((cid, n), row) in &snapshot.rows {
        let mass = row.deployed * garrison_share(w, *cid, *n);
        if mass <= 0.0 {
            continue;
        }
        let (sea_escort, sea_denial) = campaign::supply_missions(w, *cid, *n, snapshot);
        let Some(c) = w.conflict(*cid) else { continue };
        let held: Vec<_> = w
            .districts
            .iter()
            .filter(|(d, owner)| {
                occupies(c, d, **owner, *n) && control::controller(w, d) == Some(*n)
            })
            .collect();
        let population: f64 = held
            .iter()
            .map(|(d, _)| districts::population_of(w, d).unwrap_or(0.1).max(0.02))
            .sum();
        for (d, _) in held {
            let deployed = mass * districts::population_of(w, d).unwrap_or(0.1).max(0.02)
                / population.max(0.02);
            requests.push(crate::campaign_supply::SupplyRequest {
                key: garrison_key(*cid, *n, d),
                nation: *n,
                conflict: *cid,
                district: d.clone(),
                deployed,
                burn_monthly: 0.0,
                sea_escort,
                sea_denial,
            });
        }
    }
    requests
}
fn principal(c: &Conflict, n: NationId) -> bool {
    n == c.attacker() || n == c.defender()
}
fn opponent(c: &Conflict, n: NationId) -> NationId {
    if c.side_of(n) == Some(true) {
        c.defender()
    } else {
        c.attacker()
    }
}
fn participants(c: &Conflict) -> Vec<NationId> {
    let mut v = c.side_a.clone();
    v.extend(&c.side_b);
    v.sort();
    v.dedup();
    v
}
fn side_a(c: &Conflict) -> Vec<NationId> {
    let mut v = c.side_a.clone();
    v.sort();
    v
}
fn coalition_matches(c: &Conflict, o: &Offer) -> bool {
    participants(c) == o.participants
        && side_a(c) == o.side_a
        && principal(c, o.from)
        && opponent(c, o.from) == o.loser
}
fn occupies(c: &Conflict, d: &str, owner: NationId, n: NationId) -> bool {
    c.side_of(owner).is_some()
        && c.side_of(n).is_some()
        && c.side_of(owner) != c.side_of(n)
        && c.front.get(d).is_some_and(|h| {
            h.is_finite()
                && h.abs() as f64 > crate::front::HELD_BAND
                && (*h > 0.0) == c.side_of(n).unwrap()
        })
}
fn terms_refusal(w: &WorldState, c: &Conflict, from: NationId, terms: &Terms) -> Option<String> {
    let loser = opponent(c, from);
    if from == loser
        || !w.nation_opt(loser).is_some_and(|n| n.alive)
        || crate::sovereignty::hostility_blocked(w, from, loser)
    {
        return Some("The opposing government can no longer sign these terms.".into());
    }
    match terms {
        Terms::Cede { districts: ds } => {
            if w.nation(loser).nuclear {
                return Some("Nuclear deterrence prevents imposed territorial cession.".into());
            }
            let unique: BTreeSet<_> = ds.iter().collect();
            if ds.is_empty() || ds.len() > 6 || unique.len() != ds.len() {
                return Some("Select one to six distinct occupied districts.".into());
            }
            if w.districts.values().filter(|&&n| n == loser).count() <= ds.len() {
                return Some("Limited peace cannot annex a country's last district.".into());
            }
            for d in ds {
                if w.districts.get(d) != Some(&loser)
                    || control::controller(w, d) != Some(from)
                    || !occupies(c, d, loser, from)
                {
                    return Some("Only opposing principal territory actually held by the proposer can be ceded.".into());
                }
            }
        }
        Terms::Reparations { share_bp } if *share_bp == 0 || *share_bp > 200 => {
            return Some("Reparations must be 1–200 basis points of the payer's annual GDP.".into())
        }
        Terms::Transition if w.nation(loser).nuclear => {
            return Some("Nuclear deterrence prevents an imposed political transition.".into())
        }
        _ => {}
    }
    None
}
pub fn refusal(w: &WorldState, n: NationId, order: &PeaceOrder) -> Option<String> {
    if !campaign::enabled(w) {
        return Some("Operational warfare is not enabled.".into());
    }
    if !w.nation_opt(n).is_some_and(|x| x.alive) {
        return Some("The government no longer exists.".into());
    }
    if let PeaceOrder::Respond { offer, .. } = order {
        let Some(o) = w.campaign_peace.offers.iter().find(|o| o.id == *offer) else {
            return Some("This peace offer is no longer pending.".into());
        };
        if !o.participants.contains(&n) || o.approved.contains(&n) {
            return Some("This government has no pending response.".into());
        }
        if clock::absolute_day(w) > o.expires_day {
            return Some("This peace offer has expired.".into());
        }
        let Some(c) = w.conflict(o.conflict) else {
            return Some("The conflict has ended.".into());
        };
        if !coalition_matches(c, o) {
            return Some("The coalition changed; request fresh terms.".into());
        }
        return terms_refusal(w, c, o.from, &o.terms);
    }
    let cid = match order {
        PeaceOrder::SetAim { conflict, .. }
        | PeaceOrder::Propose { conflict, .. }
        | PeaceOrder::Garrison { conflict, .. } => *conflict,
        _ => unreachable!(),
    };
    let Some(c) = w.conflict(cid).filter(|c| c.posture_of(n).is_some()) else {
        return Some("This government is not a participant.".into());
    };
    match order {
        PeaceOrder::SetAim {
            aim: WarAim::Recover { districts: ds },
            ..
        } => {
            if ds.is_empty()
                || ds.len() > 6
                || ds.iter().collect::<BTreeSet<_>>().len() != ds.len()
                || ds.iter().any(|d| {
                    !w.districts.get(d).is_some_and(|owner| {
                        c.side_of(*owner) != c.side_of(n) && c.side_of(*owner).is_some()
                    })
                })
            {
                return Some("Choose one to six opposing districts as the recovery aim.".into());
            }
        }
        PeaceOrder::Propose { terms, .. } => {
            if !principal(c, n) {
                return Some(
                    "Only a principal may table coalition peace; every member must consent.".into(),
                );
            }
            if w.campaign_peace
                .offers
                .iter()
                .any(|o| o.conflict == cid && o.from == n)
            {
                return Some("Respond to or await the existing proposal first.".into());
            }
            return terms_refusal(w, c, n, terms);
        }
        PeaceOrder::Garrison { share_bp, .. } if *share_bp > 5000 => {
            return Some("Garrisons may use at most 50% of the deployed force.".into())
        }
        _ => {}
    }
    None
}
pub fn apply(w: &mut WorldState, n: NationId, order: &PeaceOrder) -> Result<(), String> {
    if let Some(r) = refusal(w, n, order) {
        return Err(r);
    }
    match order {
        PeaceOrder::SetAim { conflict, aim } => {
            w.campaign_peace.aims.insert(key(*conflict, n), aim.clone());
        }
        PeaceOrder::Garrison {
            conflict,
            share_bp,
            policy,
        } => {
            w.campaign_peace.garrisons.insert(
                key(*conflict, n),
                GarrisonPolicy {
                    share_bp: *share_bp,
                    policy: *policy,
                },
            );
        }
        PeaceOrder::Propose { conflict, terms } => {
            let c = w.conflict(*conflict).unwrap();
            let loser = opponent(c, n);
            let ps = participants(c);
            let sides = side_a(c);
            let responders = ps
                .iter()
                .filter(|id| **id != n)
                .map(|id| id.name())
                .collect::<Vec<_>>()
                .join(", ");
            let day = clock::absolute_day(w);
            w.campaign_peace.next_id += 1;
            w.campaign_peace.offers.push(Offer {
                id: w.campaign_peace.next_id,
                conflict: *conflict,
                from: n,
                loser,
                terms: terms.clone(),
                participants: ps,
                approved: vec![n],
                side_a: sides,
                issued_day: day,
                expires_day: day + 21,
            });
            w.campaign_peace
                .last_proposal
                .insert(key(*conflict, n), day);
            w.headline(format!(
                "{} proposes peace in conflict #{}: {}. Responses required from {}.",
                n.name(),
                conflict,
                summary(terms),
                responders
            ));
        }
        PeaceOrder::Respond { offer, accept } => {
            let index = w
                .campaign_peace
                .offers
                .iter()
                .position(|o| o.id == *offer)
                .unwrap();
            if !accept {
                close(w, index, "rejected");
                return Ok(());
            }
            w.campaign_peace.offers[index].approved.push(n);
            let o = &w.campaign_peace.offers[index];
            if o.participants.iter().all(|id| o.approved.contains(id)) {
                settle(w, index)?;
            }
        }
    }
    Ok(())
}
fn close(w: &mut WorldState, index: usize, outcome: &str) {
    let o = w.campaign_peace.offers.remove(index);
    w.campaign_peace.history.push(PeaceRecord {
        offer: o,
        day: clock::absolute_day(w),
        outcome: outcome.into(),
    });
    if w.campaign_peace.history.len() > 32 {
        w.campaign_peace.history.remove(0);
    }
}
fn settle(w: &mut WorldState, index: usize) -> Result<(), String> {
    let o = w.campaign_peace.offers[index].clone();
    let c = w.conflict(o.conflict).ok_or("The conflict has ended.")?;
    if !coalition_matches(c, &o) {
        return Err("The coalition changed; request fresh terms.".into());
    }
    if let Some(r) = terms_refusal(w, c, o.from, &o.terms) {
        return Err(r);
    }
    // All validation precedes mutation. Territory uses the established atomic
    // population/GDP/oil transfer, and cash uses the existing treasury arm.
    match &o.terms {
        Terms::Cede { districts: ds } => {
            for d in ds {
                districts::transfer_district(w, o.loser, o.from, d).expect("preflighted cession");
            }
        }
        Terms::Reparations { share_bp } => {
            let amount = w.nation(o.loser).gdp * *share_bp as f64 / 10000.0;
            crate::resources::settle_cash_transfer(w, o.loser, o.from, amount);
        }
        Terms::Transition => {
            // A consented political opening, not an invented named leader.
            w.nation_mut(o.loser).authoritarianism =
                (w.nation(o.loser).authoritarianism - 0.25).max(0.0);
            crate::government::ensure(w, o.loser);
            if crate::government::is_electoral(w, o.loser) {
                crate::government::hold_election(w, o.loser);
            } else if let Some(g) = w
                .governments
                .states
                .iter_mut()
                .find(|g| g.nation == o.loser)
            {
                g.coup_pressure = (g.coup_pressure + 0.20).min(1.0);
            }
        }
        Terms::Ceasefire => {}
    }
    w.conflicts.retain(|c| c.id != o.conflict);
    let prefix = format!("war:{}:", o.conflict);
    w.daily.counters.retain(|k, _| !k.starts_with(&prefix));
    w.campaign_peace
        .occupation
        .retain(|_, x| x.conflict != o.conflict);
    w.set_flag(&crate::dyads::settled_flag(o.from, o.loser));
    w.set_flag(&crate::dyads::settled_flag(o.loser, o.from));
    w.set_relation(o.from, o.loser, -35.0);
    w.resource_have.built = false;
    close(w, index, "accepted");
    w.campaign_peace.offers.retain(|x| x.conflict != o.conflict);
    w.headline(format!(
        "Peace agreed by every participant in conflict #{}: {}.",
        o.conflict,
        summary(&o.terms)
    ));
    Ok(())
}
pub fn summary(t: &Terms) -> String {
    match t {
        Terms::Ceasefire => "Ceasefire and withdrawal to legal borders".into(),
        Terms::Cede { districts: ds } => {
            format!("Cede {} held district(s) to the proposer", ds.len())
        }
        Terms::Reparations { share_bp } => format!(
            "Pay {:.2}% of annual GDP as one cash transfer",
            *share_bp as f64 / 100.0
        ),
        Terms::Transition => "Political opening; local institutions determine the successor".into(),
    }
}
fn acceptable(w: &WorldState, o: &Offer, n: NationId) -> bool {
    let Some(c) = w.conflict(o.conflict) else {
        return false;
    };
    if c.side_of(n) == c.side_of(o.from) {
        return true;
    }
    let b = c.posture_of(n).unwrap();
    let mine = if c.side_of(n) == Some(true) {
        c.control
    } else {
        -c.control
    };
    let fatigue = w.nation(n).war_exhaustion;
    match o.terms {
        Terms::Ceasefire => {
            // This offer restores legal control without extracting concessions.
            // Recovering occupied ground is a concrete benefit even while the
            // defender still has political resolve and surviving forces.
            mine < -0.10
                || b.resolve < 0.65
                || fatigue > 0.15
                || c.quiet_months > 0
                || (c.months >= 6 && mine < 0.05)
        }
        Terms::Cede { .. } => mine < -0.25 && b.resolve < 0.55,
        Terms::Reparations { share_bp } => b.resolve < 0.40 && mine < -0.10 && share_bp <= 100,
        Terms::Transition => mine < -0.65 && b.resolve < 0.20,
    }
}
/// Called after war settlement against the real, complete world.
pub fn tick(w: &mut WorldState) {
    if !campaign::enabled(w) {
        return;
    }
    let day = clock::absolute_day(w);
    if w.campaign_peace.last_day == Some(day) {
        return;
    }
    w.campaign_peace.last_day = Some(day);
    let ids: Vec<_> = w.campaign_peace.offers.iter().map(|o| o.id).collect();
    for id in ids {
        let Some(index) = w.campaign_peace.offers.iter().position(|o| o.id == id) else {
            continue;
        };
        let o = w.campaign_peace.offers[index].clone();
        let valid = w.conflict(o.conflict).is_some_and(|c| {
            coalition_matches(c, &o) && terms_refusal(w, c, o.from, &o.terms).is_none()
        });
        if day > o.expires_day || !valid {
            close(w, index, if valid { "expired" } else { "superseded" });
            continue;
        }
        for n in o
            .participants
            .iter()
            .copied()
            .filter(|n| Some(*n) != w.player && !o.approved.contains(n))
            .collect::<Vec<_>>()
        {
            if day <= o.issued_day {
                continue;
            }
            let accept = acceptable(w, &o, n);
            let _ = crate::apply_command(
                w,
                &crate::Command::WarDiplomacy {
                    nation: n,
                    order: PeaceOrder::Respond { offer: id, accept },
                },
            );
            if !w.campaign_peace.offers.iter().any(|x| x.id == id) {
                break;
            }
        }
    }
    // Staff periodically tables limited terms; the human always receives a
    // pending offer, never an automatically signed agreement.
    let conflicts = w.conflicts.clone();
    for c in &conflicts {
        for n in [c.attacker(), c.defender()] {
            if Some(n) == w.player || !w.nation_opt(n).is_some_and(|n| n.alive) {
                continue;
            }
            let last = w
                .campaign_peace
                .last_proposal
                .get(&key(c.id, n))
                .copied()
                .unwrap_or(i32::MIN / 2);
            if day - last < 30 || c.months < 3 {
                continue;
            }
            let b = c.posture_of(n).unwrap();
            let mine = if c.side_of(n) == Some(true) {
                c.control
            } else {
                -c.control
            };
            if b.resolve < 0.60 || mine > 0.35 || c.quiet_months > 0 {
                let mut terms = Terms::Ceasefire;
                if mine > 0.35 {
                    match w
                        .campaign_peace
                        .aims
                        .get(&key(c.id, n))
                        .cloned()
                        .unwrap_or_default()
                    {
                        WarAim::Recover { districts } => {
                            let ds: Vec<_> = districts
                                .into_iter()
                                .filter(|d| control::controller(w, d) == Some(n))
                                .collect();
                            if !ds.is_empty() {
                                terms = Terms::Cede { districts: ds };
                            }
                        }
                        WarAim::Concession => terms = Terms::Reparations { share_bp: 100 },
                        WarAim::GovernmentChange => terms = Terms::Transition,
                        _ => {}
                    }
                }
                let _ = crate::apply_command(
                    w,
                    &crate::Command::WarDiplomacy {
                        nation: n,
                        order: PeaceOrder::Propose {
                            conflict: c.id,
                            terms,
                        },
                    },
                );
            }
        }
    }
    update_occupation(w);
    let garrisons: BTreeSet<_> = w
        .campaign_peace
        .occupation
        .values()
        .map(|o| (o.conflict, o.occupier))
        .collect();
    for (conflict, nation) in garrisons {
        if w.player == Some(nation)
            || w.campaign_peace
                .garrisons
                .contains_key(&key(conflict, nation))
        {
            continue;
        }
        let _ = crate::apply_command(
            w,
            &crate::Command::WarDiplomacy {
                nation,
                order: PeaceOrder::Garrison {
                    conflict,
                    share_bp: 2000,
                    policy: OccupationPolicy::Restraint,
                },
            },
        );
    }
}
fn update_occupation(w: &mut WorldState) {
    let previous = std::mem::take(&mut w.campaign_peace.occupation);
    let mut rows = Vec::new();
    for (d, &owner) in &w.districts {
        let Some(occupier) = control::controller(w, d).filter(|n| *n != owner) else {
            continue;
        };
        let Some(c) = w
            .conflicts
            .iter()
            .filter(|c| occupies(c, d, owner, occupier))
            .min_by_key(|c| c.id)
        else {
            continue;
        };
        rows.push((d.clone(), owner, occupier, c.id));
    }
    let mut totals: BTreeMap<(u32, NationId), f64> = BTreeMap::new();
    for (d, _, n, cid) in &rows {
        *totals.entry((*cid, *n)).or_default() +=
            districts::population_of(w, d).unwrap_or(0.1).max(0.02);
    }
    let mut strain: BTreeMap<NationId, f64> = BTreeMap::new();
    for (d, owner, n, cid) in rows {
        let policy = w
            .campaign_peace
            .garrisons
            .get(&key(cid, n))
            .cloned()
            .unwrap_or_default();
        let force = operations::view(w, n)
            .deployments
            .iter()
            .find(|r| r.conflict == cid)
            .map_or(0.0, |r| r.deployed)
            * policy.share_bp as f64
            / 10000.0;
        let supplied = w
            .campaign_supply
            .deliveries
            .get(&garrison_key(cid, n, &d))
            .map_or(0.0, |x| x.coverage);
        let coverage = (force / (totals[&(cid, n)] * 2.0).max(0.01)).clamp(0.0, 1.0) * supplied;
        let old = previous
            .get(&d)
            .filter(|x| x.occupier == n && x.conflict == cid);
        let days = old.map_or(1, |x| x.days.saturating_add(1));
        let resistance = old.map_or(0.15, |x| x.resistance);
        let policy_effect = match policy.policy {
            OccupationPolicy::Restraint => 0.0005,
            OccupationPolicy::Security => -0.0003,
            OccupationPolicy::Reconstruction => 0.0008,
        };
        let unrestricted = w
            .conflict(cid)
            .and_then(|c| c.posture_of(n))
            .is_some_and(|b| b.roe == Roe::Unrestricted);
        let next =
            (resistance + 0.002 * (1.0 - coverage) + if unrestricted { 0.0015 } else { 0.0 }
                - 0.003 * coverage
                - policy_effect * coverage)
                .clamp(0.0, 1.0);
        *strain.entry(n).or_default() += next * (1.0 - coverage) * 0.0002;
        w.campaign_peace.occupation.insert(
            d.clone(),
            Occupation {
                district: d,
                owner,
                occupier: n,
                conflict: cid,
                days,
                resistance: next,
                coverage,
            },
        );
    }
    for (n, pain) in strain {
        let nation = w.nation_mut(n);
        nation.war_exhaustion = (nation.war_exhaustion + pain.min(0.003)).min(1.0);
    }
}

pub fn view(w: &WorldState, c: &Conflict, n: NationId) -> serde_json::Value {
    if !campaign::enabled(w) || c.posture_of(n).is_none() {
        return serde_json::Value::Null;
    }
    let aim = w
        .campaign_peace
        .aims
        .get(&key(c.id, n))
        .cloned()
        .unwrap_or_default();
    let targets: Vec<_> = w
        .districts
        .iter()
        .filter(|(_, owner)| c.side_of(**owner).is_some() && c.side_of(**owner) != c.side_of(n))
        .map(|(id, _)| serde_json::json!({"id":id,"name":districts::name_of(id).unwrap_or(id)}))
        .collect();
    let proposals:Vec<_>=w.campaign_peace.offers.iter().filter(|o|o.conflict==c.id && o.participants.contains(&n)).map(|o|
        serde_json::json!({"id":o.id,"from_name":o.from.name(),"incoming":o.from!=n,"summary":summary(&o.terms),"expires_day":o.expires_day,
            "days_left":o.expires_day-clock::absolute_day(w),"can_respond":!o.approved.contains(&n),"terms":o.terms})).collect();
    let occupation:Vec<_>=w.campaign_peace.occupation.values().filter(|o|o.conflict==c.id && o.occupier==n).map(|o|
        serde_json::json!({"district":o.district,"name":districts::name_of(&o.district).unwrap_or(&o.district),"days":o.days,"resistance":o.resistance,"coverage":o.coverage})).collect();
    serde_json::json!({"aim":aim,"aim_options":[{"kind":"expel","label":"Expel the invader","description":"Restore legal borders."},
        {"kind":"recover","label":"Recover territory","description":"Hold named opposing districts and negotiate cession."},
        {"kind":"concession","label":"Secure a concession","description":"Negotiate limited reparations."},
        {"kind":"government_change","label":"Political transition","description":"Seek political opening; local institutions determine leadership."}],
        "targets":targets,"proposals":proposals,"can_propose":principal(c,n),"reason":if principal(c,n){""}else{"Only a principal can table terms; every ally must consent."},
        "prices":{"aim":3,"proposal":0,"response":0},"limits":{"reparations_min_bp":1,"reparations_max_bp":200,"garrison_max_bp":5000},
        "garrison":w.campaign_peace.garrisons.get(&key(c.id,n)).cloned().unwrap_or_default(),"occupation":occupation})
}
