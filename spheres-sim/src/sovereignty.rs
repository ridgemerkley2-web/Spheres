//! Economic power can produce formal, consensual subordination. A compact
//! changes sovereignty, never GDP, ownership, inventory or the tax ledger.
//! All thresholds are visible game rules, not historical estimates. Dependence
//! alone is insufficient: trust, protection and a credible patron are required.
use crate::{
    clock, domination,
    world::{NationId, WorldState},
    Command,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const COMPACT_PC: f64 = 20.0;
pub const EXIT_PC: f64 = 12.0;
pub const MIN_RELATIONS: f64 = 55.0;
pub const MIN_DEPENDENCY: f64 = 0.12;
pub const MIN_SIZE_RATIO: f64 = 1.5;
pub const REVIEW_DAYS: i32 = 30;
pub const EXIT_AFTER_REVIEWS: u8 = 3;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Compact {
    pub patron: NationId,
    pub partner: NationId,
    pub formed_day: i32,
    pub reviewed_day: i32,
    pub strained_reviews: u8,
    pub reason: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CompactQuote {
    pub patron: NationId,
    pub partner: NationId,
    pub ready: bool,
    pub reason: String,
    pub dependency: f64,
    pub reciprocal_dependency: f64,
    pub relations: f64,
    pub size_ratio: f64,
    pub protected: bool,
    pub political_cost: f64,
    pub consequence: String,
}

pub fn enabled(w: &WorldState) -> bool {
    w.rules.economic_competition && clock::is_daily(w)
}

fn root(w: &WorldState, nation: NationId) -> NationId {
    let mut at = nation;
    let mut seen = BTreeSet::new();
    while seen.insert(at) {
        let Some(parent) = domination::direct_overlord(w, at) else {
            break;
        };
        if !w.nation_opt(parent).is_some_and(|n| n.alive) {
            break;
        }
        at = parent;
    }
    at
}

/// One gate used by player commands, AI appetite, escalation and intervention.
/// Both conquered and voluntary subjects must leave before attacking their bloc.
pub fn hostility_blocked(w: &WorldState, a: NationId, b: NationId) -> bool {
    enabled(w) && a != b && root(w, a) == root(w, b)
}

pub fn hostility_reason(w: &WorldState, a: NationId, b: NationId) -> Option<String> {
    hostility_blocked(w, a, b).then(||
        "These countries share a formal sphere. Leave it or release the subject before hostile action.".into())
}

/// A compact merges whole hierarchies, not just the two signing capitals.
/// Existing external wars are allowed, but opposing descendants cannot become
/// allies by signature while their war/sanctions continue underneath it.
fn hostile_merger(w: &WorldState, a: NationId, b: NationId) -> bool {
    let crosses = |x, y| {
        let (rx, ry) = (root(w, x), root(w, y));
        (rx == a && ry == b) || (rx == b && ry == a)
    };
    w.conflicts.iter().any(|c| {
        c.side_a
            .iter()
            .any(|x| c.side_b.iter().any(|y| crosses(*x, *y)))
    }) || w.sanctions.iter().any(|(x, y)| crosses(*x, *y))
}

pub fn quote(w: &WorldState, patron: NationId, partner: NationId) -> CompactQuote {
    let a = w.nation_opt(patron).filter(|n| n.alive);
    let b = w.nation_opt(partner).filter(|n| n.alive);
    let dependency = if a.is_some() && b.is_some() {
        w.trade_dependency(partner, patron)
    } else {
        0.0
    };
    let reciprocal = if a.is_some() && b.is_some() {
        w.trade_dependency(patron, partner)
    } else {
        0.0
    };
    let ratio = a.zip(b).map_or(0.0, |(a, b)| a.gdp / b.gdp.max(1e-9));
    let protected = w.pact_partners(partner).contains(&patron);
    let relations = w.relation(patron, partner);
    let reason = if !enabled(w) {
        Some("Enable Economic Competition in a daily campaign first.")
    } else if a.is_none() || b.is_none() {
        Some("Both governments must still exist.")
    } else if patron == partner {
        Some("A country cannot join itself.")
    } else if domination::direct_overlord(w, patron).is_some() {
        Some("A subordinate government cannot offer its own sovereign compact.")
    } else if domination::direct_overlord(w, partner).is_some() {
        Some("This government already belongs to a formal sphere.")
    } else if w.in_conflict(patron) || w.in_conflict(partner) {
        Some("Settle existing conflicts before negotiating sovereignty.")
    } else if hostile_merger(w, patron, partner) {
        Some("Subjects in these spheres are fighting or sanctioning one another. Resolve those hostilities before merging their hierarchies.")
    } else if w.is_sanctioning(patron, partner) || w.is_sanctioning(partner, patron) {
        Some("Lift bilateral sanctions first.")
    } else if !ratio.is_finite() || ratio < MIN_SIZE_RATIO {
        Some("The proposed patron needs at least 1.5 times the partner's economic output.")
    } else if w.reputation(patron) < 50.0 {
        Some("The patron needs at least 50 reputation to make a credible promise.")
    } else if relations < MIN_RELATIONS {
        Some("Build relations to at least 55 before asking for formal leadership.")
    } else if !protected {
        Some("Sign a mutual defense guarantee first; trade is not protection.")
    } else if dependency < MIN_DEPENDENCY || dependency - reciprocal < 0.04 {
        Some("Build sustained economic dependence of at least 12%, with a 4-point advantage over the reverse tie.")
    } else {
        None
    };
    CompactQuote {
        patron, partner, ready: reason.is_none(),
        reason: reason.unwrap_or("The government will consent to a protected economic compact.").into(),
        dependency, reciprocal_dependency: reciprocal, relations, size_ratio: ratio,
        protected, political_cost: COMPACT_PC,
        consequence: "Formal subordination counts toward world domination. Provinces, GDP, budgets and inventory stay with their current owner. The partner can leave; neglected compacts unravel.".into(),
    }
}

pub fn propose(
    w: &mut WorldState,
    patron: NationId,
    partner: NationId,
    partner_initiated: bool,
) -> Result<(), String> {
    let q = quote(w, patron, partner);
    if !q.ready {
        return Err(q.reason);
    }
    // Never silently surrender the human's sovereignty on an AI proposal.
    if w.player == Some(partner) && !partner_initiated {
        return Err("The player must explicitly choose to join this compact.".into());
    }
    if partner_initiated && w.player != Some(partner) {
        return Err("Only the player may use the voluntary-join command.".into());
    }
    let day = clock::absolute_day(w);
    domination::subjugate(w, patron, partner);
    w.domination.compacts.retain(|c| c.partner != partner);
    w.domination.compacts.push(Compact {
        patron,
        partner,
        formed_day: day,
        reviewed_day: day,
        strained_reviews: 0,
        reason: None,
    });
    w.domination.compacts.sort_by_key(|c| c.partner);
    w.headline(format!("SPHERE: {} voluntarily joins {}'s economic compact; sovereignty, not its GDP, changes hands.", partner.name(), patron.name()));
    Ok(())
}

pub fn leave(w: &mut WorldState, nation: NationId) -> Result<(), String> {
    if !enabled(w) {
        return Err("Economic Competition is not enabled.".into());
    }
    if !w.nation_opt(nation).is_some_and(|n| n.alive) {
        return Err("This government no longer exists.".into());
    }
    let patron =
        domination::direct_overlord(w, nation).ok_or("This government is already independent.")?;
    w.domination.subjects.retain(|s| s.subject != nation);
    w.domination.compacts.retain(|c| c.partner != nation);
    w.shift_relation(nation, patron, -25.0);
    w.shift_reputation(nation, -8.0);
    w.headline(format!(
        "SPHERE: {} reasserts independence from {}. Its own subjects remain with it.",
        nation.name(),
        patron.name()
    ));
    Ok(())
}

pub fn release(w: &mut WorldState, nation: NationId, subject: NationId) -> Result<(), String> {
    if !enabled(w) {
        return Err("Economic Competition is not enabled.".into());
    }
    if !w.nation_opt(nation).is_some_and(|n| n.alive)
        || !w.nation_opt(subject).is_some_and(|n| n.alive)
    {
        return Err("Both governments must still exist.".into());
    }
    if domination::direct_overlord(w, subject) != Some(nation) {
        return Err("Only a government's direct overlord can release it.".into());
    }
    w.domination.subjects.retain(|s| s.subject != subject);
    w.domination.compacts.retain(|c| c.partner != subject);
    w.shift_relation(nation, subject, 10.0);
    w.headline(format!(
        "SPHERE: {} releases {} as an independent government.",
        nation.name(),
        subject.name()
    ));
    Ok(())
}

fn maintenance_reason(w: &WorldState, c: &Compact) -> Option<String> {
    if w.relation(c.patron, c.partner) < 25.0 {
        return Some("Trust has fallen below 25.".into());
    }
    if !w.pact_partners(c.partner).contains(&c.patron) {
        return Some("The protection guarantee has ended.".into());
    }
    if w.trade_dependency(c.partner, c.patron) < MIN_DEPENDENCY / 2.0 {
        return Some(
            "The partner diversified its economy and no longer relies on this patron.".into(),
        );
    }
    if w.nation(c.patron).gdp < w.nation(c.partner).gdp * 1.1 {
        return Some("The partner's economy has caught up with its patron.".into());
    }
    None
}

pub fn tick(w: &mut WorldState) {
    if !enabled(w) {
        return;
    }
    let day = clock::absolute_day(w);
    if w.domination.sovereignty_day == Some(day) {
        return;
    }
    w.domination.sovereignty_day = Some(day);
    let mut retained = vec![];
    let mut leaving = vec![];
    for mut compact in w.domination.compacts.clone() {
        if !w.nation_opt(compact.patron).is_some_and(|n| n.alive)
            || !w.nation_opt(compact.partner).is_some_and(|n| n.alive)
            || domination::direct_overlord(w, compact.partner) != Some(compact.patron)
        {
            continue;
        }
        if day.saturating_sub(compact.reviewed_day) >= REVIEW_DAYS {
            compact.reviewed_day = day;
            compact.reason = maintenance_reason(w, &compact);
            compact.strained_reviews = if compact.reason.is_some() {
                compact.strained_reviews.saturating_add(1)
            } else {
                0
            };
            if compact.strained_reviews == 1 {
                w.headline(format!("SPHERE: {} questions its compact with {}: {} Three strained reviews allow the AI partner to leave.",
                    compact.partner.name(), compact.patron.name(), compact.reason.as_deref().unwrap_or("")));
            }
            if compact.strained_reviews >= EXIT_AFTER_REVIEWS && w.player != Some(compact.partner) {
                leaving.push(compact.partner);
            }
        }
        retained.push(compact);
    }
    w.domination.compacts = retained;
    for nation in leaving {
        let _ = crate::apply_command(w, &Command::LeaveEconomicUnion { nation });
    }

    // Review diplomacy monthly, after real economic changes; never every day.
    // A measured deterministic candidate is a policy, not a random country event.
    if w.day != 1 {
        return;
    }
    // Conquest is not permanent mind control. A hostile, recovered AI subject
    // may spend standing to assert independence before any hostile action.
    let recovered: Vec<NationId> = w
        .domination
        .subjects
        .iter()
        .filter_map(|s| {
            let subject = w.nation_opt(s.subject).filter(|n| n.alive)?;
            let patron = w.nation_opt(s.overlord).filter(|n| n.alive)?;
            (w.player != Some(subject.id)
                && !w
                    .domination
                    .compacts
                    .iter()
                    .any(|c| c.partner == subject.id)
                && w.relation(s.subject, s.overlord) < -25.0
                && subject.gdp >= patron.gdp * 0.75
                && subject.mil_strength >= patron.mil_strength * 0.60
                && subject.political_capital >= EXIT_PC)
                .then_some(subject.id)
        })
        .collect();
    for nation in recovered {
        let _ = crate::apply_command(w, &Command::LeaveEconomicUnion { nation });
    }
    let sovereigns: Vec<NationId> = w
        .nations
        .iter()
        .filter(|n| {
            n.alive && w.player != Some(n.id) && domination::direct_overlord(w, n.id).is_none()
        })
        .map(|n| n.id)
        .collect();
    for patron in sovereigns {
        if w.nation(patron).political_capital < COMPACT_PC {
            continue;
        }
        let candidate = w
            .nations
            .iter()
            .filter(|n| n.alive && w.player != Some(n.id))
            .filter_map(|n| {
                let q = quote(w, patron, n.id);
                q.ready.then_some((n.id, q.dependency))
            })
            .max_by(|a, b| a.1.total_cmp(&b.1).then_with(|| b.0.cmp(&a.0)))
            .map(|(id, _)| id);
        if let Some(partner) = candidate {
            let _ = crate::apply_command(w, &Command::ProposeEconomicUnion { patron, partner });
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SphereView {
    pub enabled: bool,
    pub overlord: Option<NationId>,
    pub partners: Vec<NationId>,
    pub compacts: Vec<Compact>,
    pub opportunities: Vec<CompactQuote>,
    pub join_opportunities: Vec<CompactQuote>,
    pub exit_cost: f64,
}

pub fn view(w: &WorldState, nation: NationId) -> SphereView {
    let mut opportunities: Vec<_> = w
        .nations
        .iter()
        .filter(|n| n.alive && n.id != nation && domination::direct_overlord(w, n.id).is_none())
        .map(|n| quote(w, nation, n.id))
        .collect();
    opportunities.sort_by(|a, b| {
        b.ready
            .cmp(&a.ready)
            .then_with(|| b.dependency.total_cmp(&a.dependency))
            .then_with(|| a.partner.cmp(&b.partner))
    });
    opportunities.truncate(8);
    let mut join_opportunities: Vec<_> = w
        .nations
        .iter()
        .filter(|n| n.alive && n.id != nation)
        .map(|n| quote(w, n.id, nation))
        .filter(|q| q.ready)
        .collect();
    join_opportunities.sort_by(|a, b| {
        b.dependency
            .total_cmp(&a.dependency)
            .then_with(|| a.patron.cmp(&b.patron))
    });
    join_opportunities.truncate(3);
    SphereView {
        enabled: enabled(w),
        overlord: domination::direct_overlord(w, nation),
        partners: w
            .domination
            .subjects
            .iter()
            .filter(|s| s.overlord == nation)
            .map(|s| s.subject)
            .collect(),
        compacts: w
            .domination
            .compacts
            .iter()
            .filter(|c| c.patron == nation || c.partner == nation)
            .cloned()
            .collect(),
        opportunities,
        join_opportunities,
        exit_cost: EXIT_PC,
    }
}
