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
/// sustained force and never on `REPLACEMENT_RATE` itself. The caller gates it
/// on being a belligerent; this is the shape only.
///
/// THE SLOPE IS DERIVED, the way education's 15.0 is, and it is no longer the
/// design's first-draft x20. The rule the design states in education's own row
/// is that a slope is chosen so the top of the ministry's own dial meets the
/// clamp ceiling, "so no step of the dial buys nothing". MEASURED on this tree,
/// across all 137 living 1990 nations, health's reachable raise
/// (`BUDGET_CAPS[0] - reference`) runs min 0.09575, mean 0.10112, median
/// 0.10125, max 0.10725. At x20 the 1.60 ceiling was met at a gap of 0.030, so
/// between 68.7% and 72.0% of every nation's raise range (mean 70.3%) bought
/// nothing on this arm. At 6.0 the ceiling is met at 0.10000, which is the
/// measured mean reach to within 1.1%.
///
/// Inert where it matters: at gap 0.0 the multiplier is 1.0 whatever the slope,
/// so no closed-books nation and no unenacted board sees a different number.
pub fn health_replacement(gap: f64) -> f64 {
    (1.0 + gap * 6.0).clamp(0.60, 1.60)
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
/// the 0.70/1.40 clamp. Not gated on war -- an arsenal is built in peace.
///
/// THE SLOPE IS DERIVED, and this is the ministry where getting it wrong cost
/// the most, because INDUSTRY HAS EXACTLY ONE ARM: when it saturates, the whole
/// card is dead and the page prints "another point of GDP buys nothing here"
/// for every remaining press. MEASURED on this tree across all 137 living 1990
/// nations, industry's reachable raise runs min 0.03900, mean 0.09505, median
/// 0.10200, max 0.11790. At x20 the 1.40 ceiling was met at a gap of 0.020, so
/// between 48.7% and 83.0% of the raise range (mean 78.0%) bought nothing at
/// all. At 4.2 the ceiling is met at 0.09524, the measured mean reach to within
/// 0.2%, which is the same derivation education's 15.0 carries.
///
/// Inert where it matters: at gap 0.0 the multiplier is 1.0 whatever the slope.
pub fn industry_refill(gap: f64) -> f64 {
    (1.0 + gap * 4.2).clamp(0.70, 1.40)
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
///
/// THE x8 IS NOT RE-DERIVED and that is deliberate. It is an INCUMBENT
/// calibration of the sanction shield, carried across from before the
/// ministries existed and kept by the design ("the sanction shield, kept,
/// ceiling 0.40"); health's and industry's slopes were the design's own
/// first-draft inventions and could be sized, this one is somebody's
/// measurement of how much a foreign service can argue away. MEASURED on this
/// tree: the 0.40 ceiling is met at a gap of 0.05000 against a reachable raise
/// of min 0.07566 / mean 0.07609 / max 0.07658 across 137 nations, so the top
/// 33.9% to 34.7% of the dial buys nothing ON THIS ARM. What the rest of the
/// dial buys is stated on the card, which is the design's other option: the
/// COUNTER-INTELLIGENCE arm below keeps paying past this one's ceiling, so the
/// ministry is never dead even where the shield is.
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

/// The stability a nation SETTLES AT under a standing pressure, bounded exactly
/// the way `economy::tick` bounds it.
///
/// `economy::tick` integrates stability as
///
/// ```text
/// ds = stability_pressure(..) + (60.0 - stability) * 0.01
/// stability = (stability + ds * 0.25).clamp(0.0, 100.0)
/// ```
///
/// so the fixed point of a standing pressure `p` is `60 + p/0.01`: the mean
/// reversion is what the pressure has to beat, and it is beaten at a hundred
/// times the pressure. THE CLAMP IS PART OF THE ANSWER. A nation cannot settle
/// above 100 or below 0, and the integrator does not merely approach those
/// bounds, it stops at them.
pub fn stability_settles_at(pressure: f64) -> f64 {
    (60.0 + pressure / MEAN_REVERSION).clamp(0.0, 100.0)
}

/// Turn a contribution to `ds` into the displacement of where a nation settles,
/// FROM WHERE IT ALREADY SETTLES, inside the 0..100 bound.
///
/// THE DESIGN'S RULING, and it is arithmetic rather than taste: a card quoting
/// the raw `ds` contribution `x` -- or the first month's `0.25x`, or the first
/// year's -- understates where the nation is going by two orders of magnitude,
/// and a player reading it would conclude the security budget does nothing.
/// SECURITY's 16.0 is +16 points of destination per point of GDP, HOUSING's
/// 14.0 is +14, PENSIONS' 12.0 is +12 -- but only while there is room on the
/// scale, which is the repair this function's second argument exists for.
///
/// WHY IT TAKES THE BASE. Quoting `x / MEAN_REVERSION` alone is right about the
/// reciprocal and wrong about the bound, and the error is not small. MEASURED
/// on this tree by running the integrator to its fixed point (6000 months,
/// everything else frozen): the USA settles at 48.933 with no gap, so a security
/// gap of +0.030 moves it +48.000 as quoted, but +0.050 delivers +51.067 against
/// a quoted +80.0, and at the top of the dial (+0.10495) it delivers the same
/// +51.067 against a quoted +167.9 -- a displacement that is not merely wrong
/// but impossible on a 0..100 scale. India, base 29.380, quoted +169.0 at the
/// top of security and delivering +70.620. Roster-wide, computed from each
/// nation's own fixed point, the DEAD top of the dial ran security min 40.4% /
/// mean 56.6% / max 71.7%, housing 33.2 / 52.4 / 69.8, pensions 40.4 / 57.6 /
/// 73.2, n=137 -- and the served `per_point` never fell to zero, so the page's
/// "another point of GDP buys nothing here" branch could not fire.
///
/// Differencing two bounded destinations fixes both halves at once: the number
/// is reachable, and `per_point` goes to zero exactly where the sim stops
/// paying.
pub fn stability_destination(base_pressure: f64, ds_contribution: f64) -> f64 {
    stability_settles_at(base_pressure + ds_contribution) - stability_settles_at(base_pressure)
}

/// The `ds` contribution ONE ministry makes, by index. `None` for the seven
/// that own no stability arm.
///
/// One place, so that `stability_pressure`'s three addends and the card's base
/// cannot fall out of step: `arms_at` subtracts a ministry's own current
/// contribution from the nation's pressure to get the base the card's curve is
/// measured from, and it must subtract exactly what `economy` added.
pub fn stability_arm(ministry: usize, gap: f64) -> Option<f64> {
    use crate::world::{BUDGET_HOUSING, BUDGET_PENSIONS, BUDGET_SECURITY};
    match ministry {
        BUDGET_HOUSING => Some(housing_stability(gap)),
        BUDGET_PENSIONS => Some(pensions_stability(gap)),
        BUDGET_SECURITY => Some(security_stability(gap)),
        _ => None,
    }
}

/// The pressure a nation carries with ONE ministry's stability arm taken back
/// out -- the base every stability card's curve is differenced against.
fn stability_base(w: &WorldState, n: &Nation, ministry: usize) -> f64 {
    let carried = stability_arm(ministry, n.budget_gap(ministry)).unwrap_or(0.0);
    crate::economy::stability_pressure_of(w, n) - carried
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
    let gap = if n.program_budget.is_some() { crate::programs::service_gap(n, ministry, allocation) } else { allocation - reference };
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
                value: stability_destination(stability_base(w, n, ministry), housing_stability(gap)),
            },
        ],
        BUDGET_PENSIONS => vec![
            Arm {
                id: "standing",
                name: "Standing ceiling",
                note: "a cut bleeds every month it stands, on top of the vote it cost",
                kind: ArmKind::Points,
                // THE REALISED MOVE IN THE CEILING, not the slope. `politics`
                // clamps the target to 0..100 immediately after adding this arm,
                // and the arm saturates that clamp long before the dial runs
                // out. MEASURED on this tree, Brazil with the books open, one
                // `politics::tick`, seed 1990: no gap -> political_capital
                // 20.7002; +0.005 -> 20.9285; +0.020 -> 21.3485; +0.040 ->
                // 21.9085; +0.060 -> 22.4685; +0.100 -> 23.0954; +0.14120, the
                // top of the dial -> 23.0954, identical, the ceiling already
                // saturated. Bisected, the 100-point ceiling bound at gap
                // 0.08239, so the top 41.7% of the pensions dial bought ZERO
                // standing while the card kept printing +10.0 a point.
                // `politics.rs` already said so in a comment; now the card
                // agrees with it.
                value: (crate::politics::standing_target(w, n.id) + pensions_standing(gap))
                    .clamp(0.0, 100.0)
                    - crate::politics::standing_target(w, n.id).clamp(0.0, 100.0),
            },
            Arm {
                id: "jobs",
                name: "Unemployment",
                note: "retirement support reduces the measured labour force; this is not job creation",
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
                value: stability_destination(stability_base(w, n, ministry), pensions_stability(gap)),
            },
        ],
        BUDGET_INFRASTRUCTURE => vec![Arm {
            id: "extraction",
            name: "Resource output",
            note: "located non-oil production; a stock built at 2pp a month, not a switch",
            kind: ArmKind::Share,
            value: infrastructure_extraction(gap),
        }],
        BUDGET_INDUSTRY if n.program_budget.is_some() => vec![],
        BUDGET_INDUSTRY => vec![Arm {
            id: "refill",
            name: "Magazine refill",
            note: "the war-industrial base, and it is built in peace",
            kind: ArmKind::Mult,
            value: industry_refill(gap),
        }],
        BUDGET_SCIENCE => vec![{
            // THE REALISED CHANGE IN CAPACITY, not the raw `gap * 6.0`.
            // `tech::absorptive_capacity` clamps the sum to 0.05..1.20, and the
            // arm is largest exactly where a nation has no headroom left: the
            // 6.0 slope was sized so a maximal programme is "worth about as much
            // reach as being rich", and a nation that IS rich has already spent
            // that room. MEASURED on this tree, seed 1990, books open, bisected
            // to the first gap whose capacity equals the value at the cap: USA
            // base 0.9746, ceiling met at 0.03757 of a 0.07475 reachable range,
            // top 49.7% of the dial dead; Japan 0.9119 and 33.8%; Germany 0.8435
            // and 20.0%; Brazil 0.4715, never binding. Differencing the clamped
            // level makes the card saturate where the sim does.
            let base = crate::tech::absorptive_capacity_before_science(w, n, crate::tech::dev_of(n));
            Arm {
                id: "reach",
                name: "Absorptive capacity",
                note: "a laboratory system buys a cheaper technology, not a bigger budget",
                kind: ArmKind::Level,
                value: (base + science_absorption(gap)).clamp(0.05, 1.20)
                    - base.clamp(0.05, 1.20),
            }
        }],
        BUDGET_DEFENSE if n.program_budget.is_some() => vec![Arm {
            id: "force", name: "Sustained force", note: "standing defense envelope; procurement shares its department with arms-plant construction",
            kind: ArmKind::Force, value: crate::war::sustained_force(n, allocation),
        }, Arm {
            id: "refill", name: "Magazine refill", note: "Defense maintenance & supply replaces Industry's former refill responsibility",
            kind: ArmKind::Mult, value: crate::programs::refill_multiplier(n, Some(allocation)),
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
                value: stability_destination(stability_base(w, n, ministry), security_stability(gap)),
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
                note: "how much likelier a COLD foreign operation here is to be exposed;                        every operation after the first starts hotter and has less room",
                kind: ArmKind::Share,
                // THE REALISED CHANGE IN THE PROBABILITY, not the raw
                // `gap * 10.0`. `statecraft::exposure_probability` clamps to
                // 0.05..0.85, and the arm's slope can offer more than the
                // probability has room for: MEASURED on this tree, the largest
                // reachable diplomacy gap is 0.07580 (Brazil), 0.07570 (Belgium
                // and the USA) and 0.07590 (India), so the card printed
                // +75.8pp, +75.7pp and +75.9pp of exposure into a probability
                // whose ceiling leaves at most 75 points of room even from a
                // cold channel against a pure democracy, and the served
                // `per_point` never fell below +0.100000.
                //
                // QUOTED AT ZERO HEAT, which is the FIRST operation mounted
                // against this nation and the case where the arm buys the most.
                // Heat accrues 0.18 per operation and only ever eats headroom,
                // so the number on the card is an upper bound the note names
                // rather than a promise the sim will exceed.
                value: crate::statecraft::exposure_probability(
                    0.0,
                    n.authoritarianism,
                    diplomacy_counterintel(gap),
                ) - crate::statecraft::exposure_probability(0.0, n.authoritarianism, 0.0),
            },
        ],
        _ => Vec::new(),
    }
}

/// The reference settlement every gap is measured against, for one nation.
pub fn reference_of(w: &WorldState, id: NationId) -> [f64; BUDGET_MINISTRIES] {
    w.nation(id).budget_for(w.year).reference
}
