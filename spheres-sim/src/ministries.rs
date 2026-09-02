//! What each of the ten ministries BUYS, held in one place.
//!
//! Stage 4 of the ministry economy is the CARDS, and a card is only worth
//! reading if it prints the number the sim is charging. The way this codebase
//! has repeatedly got that wrong is by writing the rule twice: once where the
//! sim charges it and once where the browser draws it. The growth model's
//! browser copy was missing the whole net-of-replacement capital shape, the
//! labour term, all three demand gates and the collapse floor by the time
//! anybody checked; the sanction drag was a count against a share and the two
//! answers ran 313x apart. `GrowthTerms` and `ResearchTerms` are the repairs.
//! This module is the same repair for the budget.
//!
//! So every named arm in the approved design is defined HERE, exactly once, as
//! a pure function of the ministry gap. `economy::tick`, `war`, `tech`,
//! `politics`, `resources` and `statecraft` all call these functions instead of
//! spelling the arithmetic out, and the browser reads the SAME functions
//! sampled over the dial's range. There is no second copy to drift.
//!
//! NOTHING HERE CHANGES STATE. This module is not in the `SYSTEMS` table and
//! never will be: it is the arithmetic the tick systems apply, factored out so
//! it can also be reported. Every function is `f(gap) -> value` and the
//! coefficients are the ones the design fixed, carried across character for
//! character from the sites they were factored out of.
//!
//! THE STABILITY ARMS ARE QUOTED AS DESTINATIONS, which is the design's ruling
//! and is not a presentation choice. See [`stability_destination`].

use crate::world::{Nation, NationId, WorldState, BUDGET_MINISTRIES};

// ---------------------------------------------------------------------------
// 0 HEALTH
// ---------------------------------------------------------------------------

/// HEALTH's first arm, INCUMBENT: a funded health service is a growing one.
/// Added to `population_growth` as an annual rate.
pub fn health_population(gap: f64) -> f64 {
    gap * 0.030
}

/// HEALTH's second arm: the wartime replacement multiplier, on the approach to
/// sustained force and never on `REPLACEMENT_RATE` itself. INVENTED: the x20.
/// The caller gates it on being a belligerent; this is the shape only.
pub fn health_replacement(gap: f64) -> f64 {
    (1.0 + gap * 20.0).clamp(0.60, 1.60)
}

// ---------------------------------------------------------------------------
// 1 EDUCATION
// ---------------------------------------------------------------------------

/// EDUCATION's only arm, and EDUCATION ALONE OWNS IT: how much research a
/// given amount of money buys. The 15.0 is DERIVED, not invented -- it is the
/// slope at which the top of education's own dial meets the 2.25 ceiling, so
/// no step of the dial buys nothing.
pub fn education_research(gap: f64) -> f64 {
    (1.0 + gap * 15.0).clamp(0.35, 2.25)
}

// ---------------------------------------------------------------------------
// 2 HOUSING
// ---------------------------------------------------------------------------

/// HOUSING's first arm, INCUMBENT: households that can afford a home have
/// children. An annual rate, added to `population_growth`.
pub fn housing_population(gap: f64) -> f64 {
    gap * 0.015
}

/// HOUSING's second arm, INCUMBENT: the contribution to `ds`.
pub fn housing_stability(gap: f64) -> f64 {
    gap * 14.0
}

// ---------------------------------------------------------------------------
// 3 PENSIONS
// ---------------------------------------------------------------------------

/// PENSIONS' first arm: the standing a government holds while a cut stands, as
/// a displacement of the political-capital CEILING. INVENTED: 1000.0, which
/// reads as ten points of ceiling per point of GDP.
pub fn pensions_standing(gap: f64) -> f64 {
    gap * 1000.0
}

/// PENSIONS' second arm, and PENSIONS ALONE OWNS IT: labour-force withdrawal.
/// SUBTRACTED from the unemployment rate by the caller, so a positive answer
/// here is a fall in unemployment. INVENTED: the 0.20 slope.
pub fn pensions_jobs(gap: f64) -> f64 {
    gap * 0.20
}

/// PENSIONS' third arm, INCUMBENT: the contribution to `ds`.
pub fn pensions_stability(gap: f64) -> f64 {
    gap * 12.0
}

// ---------------------------------------------------------------------------
// 4 INFRASTRUCTURE
// ---------------------------------------------------------------------------

/// INFRASTRUCTURE's only new arm: the uplift a standing road budget buys on
/// LOCATED NON-OIL production, as a target the stock walks toward at
/// `resources::INFRA_EXTRACTION_RATE` a month. Oil is excluded because oil is
/// already a complete national system.
pub fn infrastructure_extraction(gap: f64) -> f64 {
    (gap * crate::resources::INFRA_EXTRACTION_SLOPE).clamp(
        -crate::resources::INFRA_EXTRACTION_CEILING,
        crate::resources::INFRA_EXTRACTION_CEILING,
    )
}

// ---------------------------------------------------------------------------
// 5 INDUSTRY & ENERGY
// ---------------------------------------------------------------------------

/// INDUSTRY & ENERGY's only arm: how fast the magazines come back. INVENTED:
/// the x20 and the 0.70/1.40 clamp. Not gated on war -- an arsenal is built in
/// peace.
pub fn industry_refill(gap: f64) -> f64 {
    (1.0 + gap * 20.0).clamp(0.70, 1.40)
}

// ---------------------------------------------------------------------------
// 6 SCIENCE
// ---------------------------------------------------------------------------

/// SCIENCE's only arm, moved off the quantity side onto the price side:
/// absorptive capacity, which is how well a country can read somebody else's
/// paper and build the machine it describes. INVENTED: the 6.0 slope.
pub fn science_absorption(gap: f64) -> f64 {
    gap * 6.0
}

// ---------------------------------------------------------------------------
// 7 DEFENSE -- deliberately empty
// ---------------------------------------------------------------------------

// DEFENSE HAS NO GAP ARM AND THAT IS THE DESIGN. Its allocation IS
// `mil_spend_gdp`, the priced aggregate `sustained_force`, the arsenal and the
// force model already read, so a gap arm here would charge the same money
// twice. If a later session notices that nine ministries have a named arm in
// this file and defense does not, this comment is the answer.

// ---------------------------------------------------------------------------
// 8 SECURITY
// ---------------------------------------------------------------------------

/// SECURITY's first arm, INCUMBENT: the contribution to `ds`.
pub fn security_stability(gap: f64) -> f64 {
    gap * 16.0
}

/// SECURITY's second arm, and SECURITY ALONE OWNS IT: separatism suppressed,
/// per month. A POSITIVE GAP ONLY -- reading a cut here would conjure secession
/// that is not in the nation's `separatism` stock.
pub fn security_cohesion(gap: f64) -> f64 {
    gap.max(0.0) * 0.04
}

// ---------------------------------------------------------------------------
// 9 DIPLOMACY
// ---------------------------------------------------------------------------

/// DIPLOMACY's first arm, INCUMBENT: the share of the sanction drag a funded
/// foreign service argues away.
pub fn diplomacy_shield(gap: f64) -> f64 {
    (gap * 8.0).clamp(-0.20, 0.40)
}

/// DIPLOMACY's second arm: added to the probability that a foreign covert
/// operation against you is EXPOSED, which is what costs its sponsor relations
/// and reputation on the path that already exists. INVENTED: the 10.0 slope.
pub fn diplomacy_counterintel(gap: f64) -> f64 {
    gap * 10.0
}

// ---------------------------------------------------------------------------
// The stability integrator, and why a card may not quote a rate
// ---------------------------------------------------------------------------

/// The mean reversion in `economy::tick`'s stability integrator, named rather
/// than left as a magic 0.01 so that [`stability_destination`] is visibly its
/// reciprocal instead of a magic 100.
pub const MEAN_REVERSION: f64 = 0.01;

/// Turn a contribution to `ds` into the displacement of the stability a nation
/// SETTLES AT.
///
/// THE DESIGN'S RULING, and it is arithmetic rather than taste. `economy::tick`
/// integrates stability as
///
/// ```text
/// ds += ... + (60.0 - stability) * 0.01
/// stability += ds * 0.25
/// ```
///
/// so the fixed point of a standing contribution `x` is `60 + 100x`: the mean
/// reversion is what the contribution has to beat, and it is beaten at a
/// hundred times the contribution. A card quoting `x` -- or quoting the first
/// month's `0.25x`, or the first year's -- understates where the nation is
/// actually going by two orders of magnitude, and a player reading it would
/// conclude the security budget does nothing. So every stability arm on every
/// card is quoted through this function: SECURITY's 16.0 is +16 points of
/// destination per point of GDP, HOUSING's 14.0 is +14, PENSIONS' 12.0 is +12.
pub fn stability_destination(ds_contribution: f64) -> f64 {
    ds_contribution / MEAN_REVERSION
}

// ---------------------------------------------------------------------------
// Reporting: what one dial buys, for a browser that must not do this itself
// ---------------------------------------------------------------------------

/// How a card should print an arm. The browser chooses words; it does not
/// choose numbers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArmKind {
    /// An annual rate, e.g. +0.03pp/yr of population growth.
    Rate,
    /// Points on a 0..100 scale -- stability, standing.
    Points,
    /// A multiplier, printed x1.40.
    Mult,
    /// A share of something, printed as a percentage.
    Share,
    /// A dimensionless level added to a bounded sum (absorptive capacity).
    Level,
    /// Absolute military force.
    Force,
}

impl ArmKind {
    pub fn id(self) -> &'static str {
        match self {
            ArmKind::Rate => "rate",
            ArmKind::Points => "points",
            ArmKind::Mult => "mult",
            ArmKind::Share => "share",
            ArmKind::Level => "level",
            ArmKind::Force => "force",
        }
    }
}

/// One named arm of one ministry, evaluated at one hypothetical allocation.
#[derive(Clone, Debug)]
pub struct Arm {
    pub id: &'static str,
    pub name: &'static str,
    /// The sentence the design asks each card to carry, minus its number.
    pub note: &'static str,
    pub kind: ArmKind,
    pub value: f64,
}

/// Every named arm of one ministry, at an allocation the player has not
/// enacted yet.
///
/// `allocation` is a share of GDP; the GAP is `allocation - reference`, and the
/// reference is the settlement the books were opened on, so this answers "what
/// would this dial buy" and not "what is this dial worth in the abstract".
///
/// This is what the browser samples. It calls the same functions above that
/// `economy::tick` calls, so a card and the sim cannot disagree by
/// construction -- the only way to make them disagree is to edit one of the
/// functions above, which edits both.
///
/// SIGNS ARE THE READER'S, not the integrator's: `jobs` and `cohesion` are
/// negated here because the sim SUBTRACTS them, and a card that printed
/// "unemployment +0.2" for a pension rise would be exactly backwards.
pub fn arms_at(w: &WorldState, n: &Nation, ministry: usize, allocation: f64) -> Vec<Arm> {
    use crate::world::{
        BUDGET_DEFENSE, BUDGET_DIPLOMACY, BUDGET_EDUCATION, BUDGET_HEALTH, BUDGET_HOUSING,
        BUDGET_INDUSTRY, BUDGET_INFRASTRUCTURE, BUDGET_PENSIONS, BUDGET_SCIENCE, BUDGET_SECURITY,
    };
    const SETTLES: &str = "where order comes to rest, not where it is this month";
    let reference = n.budget_for(w.year).reference[ministry];
    let gap = allocation - reference;
    match ministry {
        BUDGET_HEALTH => vec![
            Arm {
                id: "population",
                name: "Population",
                note: "hospitals and public health, on the birth and survival rate",
                kind: ArmKind::Rate,
                value: health_population(gap),
            },
            Arm {
                id: "replacement",
                name: "Wartime replacement",
                note: "how fast a fighting army rebuilds toward the force it can sustain",
                kind: ArmKind::Mult,
                value: health_replacement(gap),
            },
        ],
        BUDGET_EDUCATION => vec![Arm {
            id: "research",
            name: "Research",
            note: "how much research a given research budget buys",
            kind: ArmKind::Mult,
            value: education_research(gap),
        }],
        BUDGET_HOUSING => vec![
            Arm {
                id: "population",
                name: "Population",
                note: "households that can afford a home have children",
                kind: ArmKind::Rate,
                value: housing_population(gap),
            },
            Arm {
                id: "stability",
                name: "Stability settles at",
                note: SETTLES,
                kind: ArmKind::Points,
                value: stability_destination(housing_stability(gap)),
            },
        ],
        BUDGET_PENSIONS => vec![
            Arm {
                id: "standing",
                name: "Standing ceiling",
                note: "a cut bleeds every month it stands, on top of the vote it cost",
                kind: ArmKind::Points,
                value: pensions_standing(gap),
            },
            Arm {
                id: "jobs",
                name: "Unemployment",
                note: "a pension worth taking is a pension people take",
                // A SHARE of the labour force, not a rate per year: the two
                // population arms move a growth rate and this moves a level,
                // and a card that gave both the same unit would be inviting the
                // reader to add them.
                kind: ArmKind::Share,
                value: -pensions_jobs(gap),
            },
            Arm {
                id: "stability",
                name: "Stability settles at",
                note: SETTLES,
                kind: ArmKind::Points,
                value: stability_destination(pensions_stability(gap)),
            },
        ],
        BUDGET_INFRASTRUCTURE => vec![Arm {
            id: "extraction",
            name: "Resource output",
            note: "located non-oil production; a stock built at 2pp a month, not a switch",
            kind: ArmKind::Share,
            value: infrastructure_extraction(gap),
        }],
        BUDGET_INDUSTRY => vec![Arm {
            id: "refill",
            name: "Magazine refill",
            note: "the war-industrial base, and it is built in peace",
            kind: ArmKind::Mult,
            value: industry_refill(gap),
        }],
        BUDGET_SCIENCE => vec![Arm {
            id: "reach",
            name: "Absorptive capacity",
            note: "a laboratory system buys a cheaper technology, not a bigger budget",
            kind: ArmKind::Level,
            value: science_absorption(gap),
        }],
        BUDGET_DEFENSE => vec![Arm {
            id: "force",
            name: "Sustained force",
            note: "the one ministry whose whole effect was modelled before the ministries were",
            kind: ArmKind::Force,
            value: crate::war::sustained_force(n, allocation),
        }],
        BUDGET_SECURITY => vec![
            Arm {
                id: "stability",
                name: "Stability settles at",
                note: SETTLES,
                kind: ArmKind::Points,
                value: stability_destination(security_stability(gap)),
            },
            Arm {
                id: "cohesion",
                name: "Separatism",
                note: "suppression bought each month; cutting police conjures no new secession",
                kind: ArmKind::Share,
                value: -security_cohesion(gap),
            },
        ],
        BUDGET_DIPLOMACY => vec![
            Arm {
                id: "shield",
                name: "Sanction shield",
                note: "the share of a sanction's bite a funded foreign service argues away",
                kind: ArmKind::Share,
                value: diplomacy_shield(gap),
            },
            Arm {
                id: "counterintel",
                name: "Counter-intelligence",
                note: "how much likelier a foreign operation here is to be exposed",
                kind: ArmKind::Share,
                value: diplomacy_counterintel(gap),
            },
        ],
        _ => Vec::new(),
    }
}

/// The reference settlement every gap is measured against, for one nation.
pub fn reference_of(w: &WorldState, id: NationId) -> [f64; BUDGET_MINISTRIES] {
    w.nation(id).budget_for(w.year).reference
}
