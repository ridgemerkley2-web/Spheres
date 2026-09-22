//! External security supplies and an already active armed opposition.
//!
//! Popularity is not military capacity. A government may retain loyal soldiers
//! yet lose the fuel, spares and supplies needed to hold its position. This
//! small model follows that material dependency separately from votes. Opening
//! organizations and support channels are sourced data; readiness, accumulation
//! rates and the risk of military collapse are explicit game design quantities.
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::sync::OnceLock;
use crate::government::{self, Bloc};
use crate::world::{AidFlow, AidKind, NationId, WorldState};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum SupportPurpose { IdeologicalSecurity }

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OpeningSecurity {
    pub nation: NationId,
    pub opposition: Bloc,
    pub organization: String,
    pub patron: NationId,
    pub programme: SupportPurpose,
    pub annual_support_bn: f64,
    pub sources: Vec<String>,
    pub note: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Roster { version: u32, as_of: String, rows: Vec<OpeningSecurity> }
pub const OPENING_JSON: &str = include_str!("../data/armed_security_1990.json");

pub fn parse_opening(json: &str) -> Result<Vec<OpeningSecurity>, String> {
    let rows: Roster = serde_json::from_str(json).map_err(|e| e.to_string())?;
    if rows.version != 1 || rows.as_of != "1990-01-01" {
        return Err("expected version 1, as_of 1990-01-01".into());
    }
    let mut seen = BTreeSet::new();
    for r in &rows.rows {
        if !seen.insert(r.nation) || r.nation == r.patron
            || !r.annual_support_bn.is_finite() || r.annual_support_bn <= 0.0
            || r.organization.trim().is_empty() || r.note.trim().is_empty()
            || r.sources.is_empty() || r.sources.iter().any(|s| !s.starts_with("https://"))
        {
            return Err(format!("invalid or duplicate armed security record for {:?}", r.nation));
        }
        if !crate::data::opening_movements_1990().iter()
            .any(|m| m.nation == r.nation && m.bloc == r.opposition)
        {
            return Err(format!("{:?} has no sourced opening opposition organization", r.nation));
        }
    }
    Ok(rows.rows)
}

pub fn opening() -> &'static [OpeningSecurity] {
    static ROWS: OnceLock<Vec<OpeningSecurity>> = OnceLock::new();
    ROWS.get_or_init(|| parse_opening(OPENING_JSON).expect("valid opening armed security data"))
}

/// No numbers here are voters, territory or historical predictions. The
/// sustainment reference is annual $bn of domestic defence and opening support.
/// Readiness is a unitless supply-continuity estimate; shortage accumulates
/// the gap below half the monthly requirement, not a calendar deadline.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SecurityContest {
    pub opposition: Bloc,
    pub incumbent: Bloc,
    pub sustainment_annual_bn: f64,
    pub readiness: f64,
    pub shortage_months: f64,
    /// Only explicitly tagged inherited programmes are reviewed on a change
    /// of the donor's governing bloc. Missing old saves add no such mandate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub support_mandate: Option<SupportMandate>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SupportMandate {
    pub patron: NationId,
    pub reviewed_bloc: Bloc,
}

fn enabled(w: &WorldState) -> bool { w.rules.ideology_blocs && w.rules.ideology_takeover }

/// Called only by the fresh campaign constructor. Loading a save, even one on
/// 1 January 1990, does not add support, create an insurgency or invent history.
pub(crate) fn prepare_opening(w: &mut WorldState) {
    if !enabled(w) || (w.year, w.month, w.day) != (1990, 1, 1) { return; }
    for row in opening() {
        let Some(patron) = w.nation_opt(row.patron).filter(|p| p.alive && p.gdp > 0.0) else { continue; };
        let share_gdp = row.annual_support_bn / patron.gdp;
        let authorizing_bloc = crate::blocs::ruling_bloc(w, row.patron);
        let Some(client) = w.nation_opt(row.nation).filter(|n| n.alive) else { continue; };
        let domestic = client.gdp.max(0.0) * client.mil_spend_gdp.max(0.0);
        if government::is_electoral(w, row.nation) { continue; }
        let Some(incumbent) = crate::blocs::ruling_bloc(w, row.nation) else { continue; };
        if incumbent == row.opposition { continue; }
        let Some(g) = w.governments.states.iter_mut().find(|g| g.nation == row.nation) else { continue; };
        if g.armed_security.is_some() || !g.established_movements.contains(&row.opposition) { continue; }
        g.armed_security = Some(SecurityContest {
            opposition: row.opposition, incumbent,
            sustainment_annual_bn: domestic + row.annual_support_bn,
            readiness: 1.0, shortage_months: 0.0,
            support_mandate: authorizing_bloc.map(|bloc| SupportMandate {
                patron: row.patron, reviewed_bloc: bloc,
            }),
        });
        if !w.statecraft.aid.iter().any(|f| f.patron == row.patron && f.client == row.nation && f.kind == AidKind::Arms) {
            w.statecraft.aid.push(AidFlow { patron: row.patron, client: row.nation,
                kind: AidKind::Arms, share_gdp, since_year: 1990 });
        }
    }
}

/// Current material sustainment, with any replacement patron treated equally.
/// A dead patron's stale save entry supplies nothing even before aid cleanup.
pub fn annual_resources(w: &WorldState, id: NationId) -> f64 {
    let Some(n) = w.nation_opt(id).filter(|n| n.alive) else { return 0.0; };
    let aid: f64 = w.statecraft.aid.iter()
        .filter(|f| f.client == id && f.kind == AidKind::Arms)
        .map(|f| w.nation_opt(f.patron).filter(|p| p.alive)
            .map_or(0.0, |p| p.gdp.max(0.0) * f.share_gdp.max(0.0))).sum();
    n.gdp.max(0.0) * n.mil_spend_gdp.max(0.0) + aid
}

/// Read-only material watch. Missing old-save state is explicitly unavailable.
#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct SecurityReadout {
    pub opposition: Bloc,
    pub annual_resources_bn: f64,
    pub annual_requirement_bn: f64,
    pub supplied_fraction: f64,
    pub readiness: f64,
    pub operational_capacity: f64,
    pub reserve_months_remaining: f64,
    pub shortage_months: f64,
    pub monthly_collapse_risk: f64,
}
pub fn readout(w: &WorldState, id: NationId) -> Option<SecurityReadout> {
    if !enabled(w) { return None; }
    let g = government::state(w, id)?;
    let c = g.armed_security.as_ref()?;
    let resources = annual_resources(w, id);
    let fraction = (resources / c.sustainment_annual_bn.max(1e-9)).clamp(0.0, 1.0);
    let risk = collapse_candidate(w, id).map_or(0.0, |(_, risk)| risk);
    Some(SecurityReadout { opposition: c.opposition, annual_resources_bn: resources,
        annual_requirement_bn: c.sustainment_annual_bn, supplied_fraction: fraction,
        readiness: c.readiness, operational_capacity: operational_capacity(c),
        reserve_months_remaining: (SUSTAINMENT_RESERVE_MONTHS - c.shortage_months).max(0.0),
        shortage_months: c.shortage_months, monthly_collapse_risk: risk })
}

/// A finite sustainment reserve, expressed in accumulated unmet half-supply
/// months. Twelve equivalent months is an explicit model stock size, not an
/// observed national inventory or a date for the war to end. An active force
/// cannot run forever on a small fraction of its recurring requirements:
/// continuing shortages consume equipment, reserves and field cohesion.
/// Existing saved shortage already records that unpaid material bill; no
/// historical reserve or campaign duration is invented when loading a save.
pub const SUSTAINMENT_RESERVE_MONTHS: f64 = 12.0;
fn operational_capacity(c: &SecurityContest) -> f64 {
    let reserve = (1.0 - c.shortage_months / SUSTAINMENT_RESERVE_MONTHS).clamp(0.0, 1.0);
    c.readiness.clamp(0.0, 1.0) * reserve
}

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct ProgrammeReview {
    pub patron: NationId,
    pub previous_bloc: Bloc,
    pub current_bloc: Bloc,
    pub end_inherited_arms_flow: bool,
}

/// An ideological security commitment is not an irrevocable order from a
/// predecessor. A changed AI donor mandate reviews this explicitly tagged
/// programme once, preserving a compatible recipient and all untagged aid.
/// Bilateral friendship alone otherwise keeps the inherited programme alive
/// forever, because aid itself reinforces that same friendship every month.
pub fn pending_review(w: &WorldState, id: NationId) -> Option<ProgrammeReview> {
    if !enabled(w) { return None; }
    let c = government::state(w, id)?.armed_security.as_ref()?;
    let mandate = c.support_mandate.as_ref()?;
    if w.player == Some(mandate.patron)
        || !w.nation_opt(mandate.patron).is_some_and(|n| n.alive) { return None; }
    let current = crate::blocs::ruling_bloc(w, mandate.patron)?;
    if current == mandate.reviewed_bloc { return None; }
    Some(ProgrammeReview { patron: mandate.patron, previous_bloc: mandate.reviewed_bloc,
        current_bloc: current,
        end_inherited_arms_flow: current != crate::blocs::ruling_bloc(w, id)?
            && w.aid_flow(mandate.patron, id, AidKind::Arms).is_some(),
    })
}

fn review_support(w: &mut WorldState, id: NationId) {
    let Some(review) = pending_review(w, id) else { return; };
    if review.end_inherited_arms_flow {
        let command = crate::Command::EndAid { patron: review.patron, client: id, kind: AidKind::Arms };
        // Manual repudiation is always available and can exhaust the last
        // political capital. This automatic policy review waits until it can
        // pay the complete existing price; it must not silently get a discount.
        if w.nation(review.patron).political_capital < crate::price_of(w, &command).unwrap_or(0.0) {
            return;
        }
        if crate::apply_command(w, &command).is_err() { return; }
    }
    // Consume even a review with no flow left to end. Deliberately restored
    // or subsequently chosen aid is not vetoed again under the same mandate.
    let c = w.governments.states.iter_mut().find(|g| g.nation == id).unwrap()
        .armed_security.as_mut().unwrap();
    c.support_mandate.as_mut().unwrap().reviewed_bloc = review.current_bloc;
}

/// Six-month supply memory, a half-supply shortage threshold, and six
/// accumulated shortage months are model choices. A restored channel or a
/// domestic replacement drains the shortage; time alone never triggers it.
fn advance(w: &mut WorldState, id: NationId) {
    let Some(c) = government::state(w, id).and_then(|g| g.armed_security.as_ref()) else { return; };
    let ratio = (annual_resources(w, id) / c.sustainment_annual_bn.max(1e-9)).clamp(0.0, 1.0);
    let dt = crate::clock::month_fraction(w);
    let blend = crate::clock::blend(w, 1.0 / 6.0);
    let g = w.governments.states.iter_mut().find(|g| g.nation == id).unwrap();
    let c = g.armed_security.as_mut().unwrap();
    c.readiness = (c.readiness + (ratio - c.readiness) * blend).clamp(0.0, 1.0);
    if ratio < 0.5 {
        c.shortage_months = (c.shortage_months + (0.5 - ratio) * dt).min(24.0);
    } else {
        c.shortage_months = (c.shortage_months - dt).max(0.0);
    }
}

/// Military collapse is distinct from a popular uprising. Only a saved active
/// armed organization can exploit sustained supply failure; a movement merely
/// present in a party table or funded this month cannot become an army here.
/// The 2% monthly battlefield hazard is an invented operational coefficient.
/// Exhausting the finite material reserve is a separate capacity failure:
/// with supplies still below half, a force with no remaining capacity cannot
/// indefinitely hold the capital against an already active armed opponent.
/// Neither the risk nor the winner is scheduled by year, country or seed.
pub fn collapse_candidate(w: &WorldState, id: NationId) -> Option<(Bloc, f64)> {
    if !enabled(w) || !w.nation_opt(id).is_some_and(|n| n.alive) { return None; }
    let g = government::state(w, id)?;
    // Scheduling a first ballot is not a settlement with an active armed
    // opponent. The same interim authority must still maintain its forces.
    // Legacy electoral states, without a pending transition, retain their
    // constitutional protection and never acquire an invented conflict.
    if government::is_electoral(w, id) && !g.awaiting_first_election { return None; }
    let c = g.armed_security.as_ref()?;
    let capacity = operational_capacity(c);
    if crate::blocs::ruling_bloc(w, id) != Some(c.incumbent)
        || c.opposition == c.incumbent || !g.established_movements.contains(&c.opposition)
        || capacity >= 0.35 || c.shortage_months < 6.0
        || annual_resources(w, id) / c.sustainment_annual_bn.max(1e-9) >= 0.5
    { return None; }
    let battlefield_risk = 0.02 * (1.0 - capacity / 0.35)
        * (c.shortage_months / 12.0).min(1.0);
    let risk = if c.shortage_months >= SUSTAINMENT_RESERVE_MONTHS { 1.0 } else { battlefield_risk }
        * w.rules.crisis_intensity.max(0.0);
    Some((c.opposition, risk.clamp(0.0, 1.0)))
}

pub(crate) fn tick(w: &mut WorldState) {
    if !enabled(w) { return; }
    let ids: Vec<_> = w.governments.states.iter()
        .filter(|g| g.armed_security.is_some()).map(|g| g.nation).collect();
    for id in ids {
        if !w.nation_opt(id).is_some_and(|n| n.alive) { continue; }
        let g = government::state(w, id).unwrap();
        let incumbent = g.armed_security.as_ref().unwrap().incumbent;
        let completed_electoral_transition = government::is_electoral(w, id) && !g.awaiting_first_election;
        if completed_electoral_transition || crate::blocs::ruling_bloc(w, id) != Some(incumbent) {
            // The recorded contest belonged to that authority. A completed
            // election or another takeover does not preserve a delayed defeat
            // against an unrelated successor; an unvoted interim retains it.
            w.governments.states.iter_mut().find(|g| g.nation == id).unwrap().armed_security = None;
            continue;
        }
        review_support(w, id);
        advance(w, id);
        if let Some((winner, risk)) = collapse_candidate(w, id) {
            let probability = crate::clock::chance(w, risk);
            if probability > 0.0 && w.rng.chance(probability) {
                w.headline(format!("{}'s security supply failure lets the armed opposition take the capital.", id.name()));
                government::armed_opposition_victory(w, id, winner);
                w.governments.states.iter_mut().find(|g| g.nation == id).unwrap().armed_security = None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{init::world_1990, world::GameRules};
    const ID: NationId = NationId::Afghanistan;
    fn world() -> WorldState { world_1990(GameRules { ideology_blocs: true, ideology_takeover: true, ..GameRules::default() }) }
    fn age(w: &mut WorldState, months: usize) { for _ in 0..months { advance(w, ID); } }

    #[test]
    fn opening_security_is_sourced_priced_and_separate_from_votes() {
        let mut w = world();
        let row = &opening()[0];
        let g = government::state(&w, ID).unwrap();
        assert_eq!(g.support, vec![("af_pdpa".into(), 1.0)]);
        assert_eq!(g.armed_security.as_ref().unwrap().readiness, 1.0);
        let patron_gdp = w.nation(row.patron).gdp;
        let before = w.nation(row.patron).debt_gdp;
        crate::statecraft::tick(&mut w);
        let charged = (w.nation(row.patron).debt_gdp - before) * patron_gdp;
        assert!((charged - row.annual_support_bn * 0.5 / 12.0).abs() < 1e-9,
            "the actual Arms aid channel must charge its existing fiscal price: {charged}");
        let mut invalid: serde_json::Value = serde_json::from_str(OPENING_JSON).unwrap();
        invalid["rows"][0]["vote_share"] = serde_json::json!(0.8);
        assert!(parse_opening(&invalid.to_string()).is_err());
    }

    #[test]
    fn armed_security_sources_reject_invalid_parameters_and_unsupported_actors() {
        let original: serde_json::Value = serde_json::from_str(OPENING_JSON).unwrap();
        for case in 0..10 {
            let mut raw = original.clone();
            match case {
                0 => { let row = raw["rows"][0].clone(); raw["rows"].as_array_mut().unwrap().push(row); },
                1 => raw["rows"][0]["annual_support_bn"] = serde_json::json!(0.0),
                2 => raw["rows"][0]["annual_support_bn"] = serde_json::json!(-1.0),
                3 => raw["rows"][0]["sources"] = serde_json::json!([]),
                4 => raw["rows"][0]["patron"] = serde_json::json!("Afghanistan"),
                5 => raw["rows"][0]["opposition"] = serde_json::json!("Western"),
                6 => raw["rows"][0]["nation"] = serde_json::json!("InventedCountry"),
                7 => raw["rows"][0]["note"] = serde_json::json!(""),
                8 => raw["as_of"] = serde_json::json!("1992-04-01"),
                _ => raw["rows"][0]["programme"] = serde_json::json!("AllForeignAid"),
            }
            assert!(parse_opening(&raw.to_string()).is_err(), "invalid case {case} accepted");
        }
        assert!(parse_opening("{broken").is_err());
        assert!(parse_opening(&OPENING_JSON.replace("3.0", "1e999")).is_err());
    }

    fn reform_patron(w: &mut WorldState) {
        w.governments.states.iter_mut().find(|g| g.nation == NationId::USSR).unwrap()
            .regime_bloc = Some(Bloc::Western);
        assert_eq!(crate::blocs::ruling_bloc(w, NationId::USSR), Some(Bloc::Western));
    }

    #[test]
    fn an_inherited_security_review_uses_the_real_paid_decision_once() {
        let mut w = world();
        let patron = NationId::USSR;
        reform_patron(&mut w);
        w.nation_mut(patron).political_capital = 20.0;
        let rng = w.rng.clone();
        let before = crate::save(&w);
        assert_eq!(pending_review(&w, ID), Some(ProgrammeReview { patron,
            previous_bloc: Bloc::Communist, current_bloc: Bloc::Western,
            end_inherited_arms_flow: true }));
        assert_eq!(crate::save(&w), before, "the preview must be a pure read");
        let mut expected = w.clone();
        crate::apply_command(&mut expected, &crate::Command::EndAid {
            patron, client: ID, kind: AidKind::Arms }).unwrap();
        expected.governments.states.iter_mut().find(|g| g.nation == ID).unwrap()
            .armed_security.as_mut().unwrap().support_mandate.as_mut().unwrap()
            .reviewed_bloc = Bloc::Western;
        review_support(&mut w, ID);
        assert_eq!(crate::save(&w), crate::save(&expected), "review must have exactly the existing decision effects");
        assert_eq!(w.nation(patron).political_capital, 17.0);
        assert_eq!(w.rng, rng);
        assert!(w.aid_flow(patron, ID, AidKind::Arms).is_none());
        assert!(pending_review(&w, ID).is_none());
        let after = crate::save(&w);
        review_support(&mut w, ID);
        assert_eq!(crate::save(&w), after);
    }

    #[test]
    fn security_review_preserves_player_compatible_and_unreviewed_old_save_programmes() {
        let mut unchanged = world();
        let before = crate::save(&unchanged);
        review_support(&mut unchanged, ID);
        assert_eq!(crate::save(&unchanged), before);
        for (lens, takeover, player) in [(false, true, None), (true, false, None),
            (true, true, Some(NationId::USSR))] {
            let mut w = world();
            reform_patron(&mut w);
            w.rules.ideology_blocs = lens;
            w.rules.ideology_takeover = takeover;
            w.player = player;
            let before = crate::save(&w);
            assert!(pending_review(&w, ID).is_none());
            review_support(&mut w, ID);
            assert_eq!(crate::save(&w), before);
        }
        let mut compatible = world();
        reform_patron(&mut compatible);
        compatible.governments.states.iter_mut().find(|g| g.nation == ID).unwrap()
            .regime_bloc = Some(Bloc::Western);
        assert!(!pending_review(&compatible, ID).unwrap().end_inherited_arms_flow);
        let pc = compatible.nation(NationId::USSR).political_capital;
        review_support(&mut compatible, ID);
        assert!(compatible.aid_flow(NationId::USSR, ID, AidKind::Arms).is_some());
        assert_eq!(compatible.nation(NationId::USSR).political_capital, pc);
        assert!(pending_review(&compatible, ID).is_none());
        let mut raw: serde_json::Value = serde_json::from_str(&crate::save(&world())).unwrap();
        for g in raw["governments"]["states"].as_array_mut().unwrap() {
            if let Some(c) = g.get_mut("armed_security").and_then(|c| c.as_object_mut()) {
                c.remove("support_mandate");
            }
        }
        let mut old = crate::load(&raw.to_string()).unwrap();
        reform_patron(&mut old);
        let before = crate::save(&old);
        review_support(&mut old, ID);
        assert_eq!(crate::save(&old), before, "old saves cannot invent an inherited political mandate");
    }

    #[test]
    fn security_review_does_not_veto_replacement_or_deliberately_restored_supplies() {
        let mut w = world();
        let original = w.aid_flow(NationId::USSR, ID, AidKind::Arms).unwrap().clone();
        let replacement = AidFlow { patron: NationId::India, client: ID, kind: AidKind::Arms,
            share_gdp: 3.0 / w.nation(NationId::India).gdp, since_year: w.year };
        w.statecraft.aid.push(replacement);
        reform_patron(&mut w);
        review_support(&mut w, ID);
        assert!(w.aid_flow(NationId::India, ID, AidKind::Arms).is_some());
        age(&mut w, 120);
        assert!(collapse_candidate(&w, ID).is_none());
        w.statecraft.aid.push(original.clone());
        let before = crate::save(&w);
        review_support(&mut w, ID);
        assert_eq!(crate::save(&w), before, "a newly chosen policy is not repeatedly vetoed");

        let mut already_ended = world();
        already_ended.statecraft.aid.clear();
        reform_patron(&mut already_ended);
        review_support(&mut already_ended, ID);
        assert!(pending_review(&already_ended, ID).is_none(), "a review without a flow must still finish");
        already_ended.statecraft.aid.push(original);
        let saved = crate::save(&already_ended);
        let mut restored = crate::load(&saved).unwrap();
        review_support(&mut restored, ID);
        assert_eq!(crate::save(&restored), saved, "save/load must not resurrect a consumed review");
    }

    #[test]
    fn an_unaffordable_security_review_waits_without_free_effects_or_random_draws() {
        let mut w = world();
        reform_patron(&mut w);
        w.nation_mut(NationId::USSR).political_capital = 2.9;
        let before = crate::save(&w);
        review_support(&mut w, ID);
        assert!(crate::save(&w) == before, "an unaffordable automatic review changed state");
        assert!(pending_review(&w, ID).unwrap().end_inherited_arms_flow);
        w.nation_mut(NationId::USSR).political_capital = 3.0;
        review_support(&mut w, ID);
        assert!(w.aid_flow(NationId::USSR, ID, AidKind::Arms).is_none());
        assert_eq!(w.nation(NationId::USSR).political_capital, 0.0);
        assert!(pending_review(&w, ID).is_none());
    }

    #[test]
    fn an_unvoted_interim_keeps_its_real_supply_obligations_until_a_completed_ballot() {
        let mut funded = world();
        funded.nation_mut(ID).authoritarianism = 0.59;
        {
            let g = funded.governments.states.iter_mut().find(|g| g.nation == ID).unwrap();
            g.awaiting_first_election = true;
            g.elected = false;
            g.next_election = (1990, 7);
        }
        let rng = funded.rng.clone();
        for _ in 0..12 { tick(&mut funded); }
        assert!(government::state(&funded, ID).unwrap().armed_security.is_some());
        assert!(collapse_candidate(&funded, ID).is_none(), "a funded interim is not penalized");
        assert_eq!(funded.rng, rng);
        assert_eq!(readout(&funded, ID).unwrap().shortage_months, 0.0);

        let mut unfunded = funded.clone();
        unfunded.statecraft.aid.clear();
        age(&mut unfunded, 40);
        assert_eq!(collapse_candidate(&unfunded, ID), Some((Bloc::Islamist, 1.0)));
        let saved = crate::save(&unfunded);
        let mut restored = crate::load(&saved).unwrap();
        assert_eq!(collapse_candidate(&restored, ID), collapse_candidate(&unfunded, ID));
        tick(&mut restored);
        assert_eq!(crate::blocs::ruling_bloc(&restored, ID), Some(Bloc::Islamist));
        assert!(government::state(&restored, ID).unwrap().armed_security.is_none());

        let mut elected = unfunded.clone();
        for (_, loyalty) in &mut elected.governments.states.iter_mut().find(|g| g.nation == ID).unwrap().pillars {
            *loyalty = 0.90;
        }
        government::hold_election(&mut elected, ID);
        assert!(government::state(&elected, ID).unwrap().elected);
        assert!(!government::state(&elected, ID).unwrap().awaiting_first_election);
        assert!(collapse_candidate(&elected, ID).is_none());
        tick(&mut elected);
        assert!(government::state(&elected, ID).unwrap().armed_security.is_none(),
            "a genuinely completed constitutional transition retains its existing protection");

        for (lens, takeover) in [(false, true), (true, false)] {
            let mut off = unfunded.clone();
            off.rules.ideology_blocs = lens;
            off.rules.ideology_takeover = takeover;
            let before = crate::save(&off);
            assert!(collapse_candidate(&off, ID).is_none());
            tick(&mut off);
            assert!(crate::save(&off) == before, "disabled interim conflict logic must be inert");
        }
    }

    #[test]
    fn maintained_replaced_and_domestically_replaced_supplies_prevent_collapse() {
        let mut maintained = world();
        let before = government::state(&maintained, ID).unwrap().movements.clone();
        age(&mut maintained, 120);
        assert!(collapse_candidate(&maintained, ID).is_none());
        assert_eq!(readout(&maintained, ID).unwrap().readiness, 1.0);
        assert_eq!(government::state(&maintained, ID).unwrap().movements, before);
        let mut lost = maintained.clone();
        lost.statecraft.aid.clear();
        age(&mut lost, 36);
        assert_eq!(collapse_candidate(&lost, ID).map(|x| x.0), Some(Bloc::Islamist));
        let mut replacement = lost.clone();
        let donor = NationId::India;
        replacement.statecraft.aid.push(AidFlow { patron: donor, client: ID, kind: AidKind::Arms,
            share_gdp: 3.0 / replacement.nation(donor).gdp, since_year: replacement.year });
        assert!(collapse_candidate(&replacement, ID).is_none(), "a restored supply line acts immediately");
        age(&mut replacement, 36);
        assert!(readout(&replacement, ID).unwrap().readiness > 0.99);
        assert_eq!(readout(&replacement, ID).unwrap().shortage_months, 0.0);
        let mut domestic = lost;
        let needed = readout(&domestic, ID).unwrap().annual_requirement_bn;
        let n = domestic.nation_mut(ID);
        n.gdp = needed / n.mil_spend_gdp;
        assert!(collapse_candidate(&domestic, ID).is_none());
        age(&mut domestic, 36);
        assert_eq!(readout(&domestic, ID).unwrap().shortage_months, 0.0);
    }

    #[test]
    fn sustained_shortfalls_consume_operational_capacity_and_replacement_rebuilds_it() {
        let mut partial = world();
        let required = readout(&partial, ID).unwrap().annual_requirement_bn;
        let domestic = partial.nation(ID).gdp * partial.nation(ID).mil_spend_gdp;
        let donor_gdp = partial.nation(NationId::USSR).gdp;
        partial.statecraft.aid.iter_mut().find(|f| f.client == ID && f.kind == AidKind::Arms).unwrap()
            .share_gdp = (0.4 * required - domestic) / donor_gdp;
        age(&mut partial, 60);
        let early = readout(&partial, ID).unwrap();
        age(&mut partial, 30);
        let later = readout(&partial, ID).unwrap();
        assert!(later.operational_capacity < early.operational_capacity);
        assert!(later.monthly_collapse_risk > early.monthly_collapse_risk);
        age(&mut partial, 40);
        let exhausted = readout(&partial, ID).unwrap();
        assert_eq!(exhausted.operational_capacity, 0.0);
        assert_eq!(exhausted.monthly_collapse_risk, 1.0,
            "a continuously undersupplied active conflict cannot settle at permanent fractional capacity");
        let saved = crate::save(&partial);
        let mut restored = crate::load(&saved).unwrap();
        assert_eq!(readout(&restored, ID), Some(exhausted));
        restored.statecraft.aid.iter_mut().find(|f| f.client == ID && f.kind == AidKind::Arms).unwrap()
            .share_gdp = (required - domestic) / donor_gdp;
        assert!(collapse_candidate(&restored, ID).is_none(), "actual replenishment halts defeat immediately");
        age(&mut restored, 24);
        let recovered = readout(&restored, ID).unwrap();
        assert_eq!(recovered.shortage_months, 0.0);
        assert!(recovered.operational_capacity > 0.98);
        assert_eq!(recovered.monthly_collapse_risk, 0.0);
    }

    #[test]
    fn finite_reserves_follow_calendar_supply_in_both_clocks_without_harming_funded_forces() {
        let mut monthly = world();
        monthly.statecraft.aid.clear();
        age(&mut monthly, 24);
        let mut daily = monthly.clone();
        daily.rules.daily_simulation = true;
        advance(&mut monthly, ID);
        for _ in 0..31 { advance(&mut daily, ID); }
        let a = readout(&monthly, ID).unwrap();
        let b = readout(&daily, ID).unwrap();
        assert!((a.operational_capacity - b.operational_capacity).abs() < 1e-12);
        assert!((a.monthly_collapse_risk - b.monthly_collapse_risk).abs() < 1e-12);
        let mut funded = world();
        let before = crate::save(&funded);
        age(&mut funded, 480);
        assert!(crate::save(&funded) == before, "adequate unchanged supplies cannot consume a reserve");
        let full = readout(&funded, ID).unwrap();
        assert_eq!(full.operational_capacity, 1.0);
        assert_eq!(full.monthly_collapse_risk, 0.0);
    }

    #[test]
    fn exhausted_reserves_do_not_override_the_real_contest_or_disabled_crises() {
        let mut exhausted = world();
        exhausted.statecraft.aid.clear();
        age(&mut exhausted, 36);
        let before = crate::save(&exhausted);
        let report = readout(&exhausted, ID).unwrap();
        assert_eq!(report.reserve_months_remaining, 0.0);
        assert_eq!(report.monthly_collapse_risk, 1.0);
        assert!(crate::save(&exhausted) == before, "readout changed the exhausted state");
        for case in 0..7 {
            let mut w = exhausted.clone();
            match case {
                0 => w.rules.crisis_intensity = 0.0,
                1 => w.rules.ideology_blocs = false,
                2 => w.rules.ideology_takeover = false,
                3 => w.nation_mut(ID).alive = false,
                4 => w.nation_mut(ID).authoritarianism = 0.3,
                5 => w.governments.states.iter_mut().find(|g| g.nation == ID).unwrap().regime_bloc = Some(Bloc::NonAligned),
                _ => w.governments.states.iter_mut().find(|g| g.nation == ID).unwrap().established_movements.clear(),
            }
            assert_eq!(collapse_candidate(&w, ID).map_or(0.0, |(_, r)| r), 0.0,
                "exhaustion bypassed safety condition {case}");
        }
    }

    #[test]
    fn absent_armed_actors_and_old_saves_never_invent_a_military_claimant() {
        let mut w = world();
        w.statecraft.aid.clear();
        age(&mut w, 36);
        w.governments.states.iter_mut().find(|g| g.nation == ID).unwrap().established_movements.clear();
        assert!(collapse_candidate(&w, ID).is_none());
        let mut raw: serde_json::Value = serde_json::from_str(&crate::save(&w)).unwrap();
        for g in raw["governments"]["states"].as_array_mut().unwrap() {
            g.as_object_mut().unwrap().remove("armed_security");
        }
        let mut old = crate::load(&raw.to_string()).unwrap();
        let before = crate::save(&old);
        tick(&mut old);
        assert_eq!(crate::save(&old), before);
        assert!(readout(&old, ID).is_none());
    }

    #[test]
    fn security_state_round_trips_and_switches_are_inert() {
        let mut w = world();
        w.statecraft.aid.clear();
        age(&mut w, 36);
        let saved = crate::save(&w);
        w = crate::load(&saved).unwrap();
        assert_eq!(crate::save(&w), saved);
        let before = crate::save(&w);
        let _ = readout(&w, ID);
        let _ = collapse_candidate(&w, ID);
        assert_eq!(crate::save(&w), before);
        for (lens, takeover) in [(false, false), (true, false), (false, true)] {
            let mut off = w.clone();
            off.rules.ideology_blocs = lens;
            off.rules.ideology_takeover = takeover;
            let before = crate::save(&off);
            tick(&mut off);
            assert_eq!(crate::save(&off), before);
            assert!(readout(&off, ID).is_none());
        }
        let default = world_1990(GameRules::default());
        assert!(!crate::save(&default).contains("armed_security"));
    }

    #[test]
    fn a_military_collapse_does_not_mint_voters_and_ends_the_old_dependency() {
        let mut w = world();
        w.statecraft.aid.clear();
        age(&mut w, 36);
        w.rules.crisis_intensity = 1_000_000.0; // Exercise an eligible settlement deterministically.
        let old = government::state(&w, ID).unwrap().clone();
        assert_eq!(collapse_candidate(&w, ID).unwrap(), (Bloc::Islamist, 1.0));
        tick(&mut w);
        let g = government::state(&w, ID).unwrap();
        assert_eq!(crate::blocs::ruling_bloc(&w, ID), Some(Bloc::Islamist));
        assert_eq!(g.support, old.support);
        for (a, b) in g.movements.iter().zip(&old.movements) {
            assert_eq!(a.0, b.0);
            assert!((a.1 - b.1).abs() < 1e-12, "military victory fabricated a popular mandate");
        }
        assert!(g.armed_security.is_none());
        let after = crate::save(&w);
        tick(&mut w);
        assert_eq!(crate::save(&w), after, "a resolved armed contest fired twice");
        let mut opened = world();
        opened.nation_mut(ID).authoritarianism = 0.3;
        tick(&mut opened);
        assert!(readout(&opened, ID).is_none(), "negotiated representation left a delayed defeat");
    }

    #[test]
    fn dead_patrons_dates_and_daily_steps_follow_actual_supply() {
        let mut monthly = world();
        monthly.nation_mut(NationId::USSR).alive = false;
        assert!((annual_resources(&monthly, ID) - 0.45).abs() < 1e-9,
            "a stale aid entry cannot deliver after a patron dies");
        let mut daily = monthly.clone();
        daily.rules.daily_simulation = true;
        advance(&mut monthly, ID);
        for _ in 0..31 { advance(&mut daily, ID); }
        let m = readout(&monthly, ID).unwrap();
        let d = readout(&daily, ID).unwrap();
        assert!((m.readiness - d.readiness).abs() < 1e-12);
        assert!((m.shortage_months - d.shortage_months).abs() < 1e-12);
        monthly.year = 2035;
        let after_date = readout(&monthly, ID).unwrap();
        assert_eq!(m, after_date, "calendar years cannot cut off supply or select a victor");
        monthly.governments.states.iter_mut().find(|g| g.nation == ID).unwrap().armed_security = None;
        prepare_opening(&mut monthly);
        assert!(readout(&monthly, ID).is_none());
    }
}
