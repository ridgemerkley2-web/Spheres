//! Who wants to invade whom, derived rather than enumerated.
//!
//! This replaces a `match (attacker, target)` in `politics::ai_wars` that gave
//! fourteen ordered pairs a hand-set base appetite and everyone else zero. At
//! thirty nations that was already a table nobody could audit; at the ~190
//! BIBLE.md commits to it is ~36,000 ordered dyads and cannot be a match
//! statement at all. Worse, it was a special case named after a country, which
//! §7 of BIBLE.md calls the tell that the model is wrong.
//!
//! So appetite is a function of six things a data file or the world state can
//! state, and of nothing else:
//!
//! 1. **Reach** — do they share a border, or at least a region? (`nations.rs`)
//! 2. **Claim** — what share of the target does the aggressor say is its own?
//!    This is the term that does the most work, and it is the one that makes
//!    Kuwait different from Saudi Arabia and Bosnia different from Slovenia.
//! 3. **Grudge** — how bad relations already are.
//! 4. **Disposition** — who starts wars: armed, authoritarian governments.
//!    Both inputs are transcribed 1990 figures, and neither is a dyad.
//! 5. **Digestibility** — how much smaller the target is.
//! 6. **Worth** — how much the target would actually add.
//!
//! Then the same conditions the old code applied: fiscal desperation, the
//! target's own separatism, the strength ratio net of the coalition the
//! aggressor expects, the lesson of an invasion already repelled, and the
//! nuclear taboo.
//!
//! The behaviour the old table encoded is preserved and, where the two differ,
//! the derivation is closer to what happened: Iraq still wants Kuwait far more
//! than it wants Saudi Arabia; Belgrade still wants Bosnia more than Croatia
//! and Slovenia hardly at all — but now because Slovenia has no Serb minority
//! and no shared border, not because a line in a table says 0.010.

use crate::nations::{adjacent, claim_share};
use crate::war;
use crate::world::*;

/// Scales the whole table. Set so that Iraq's appetite for Kuwait in January
/// 1990 lands where the hand-written table had it (~12.6%/month all-in), which
/// is the one figure in the old table with a calibration test attached to it.
const APPETITE: f64 = 0.100;

/// A shared border is the ordinary way one state reaches another. A shared
/// region is a distant second: it is Israel and Iraq, five hundred miles and
/// two other countries apart, which is a raid and not an invasion.
///
/// This is deliberately small, and the reason is measurable. Raised to 0.30 —
/// with the two floors below raised in proportion — the model starts
/// manufacturing wars nobody has a quarrel over: Saudi Arabia invading Kuwait,
/// Italy invading Slovenia, Ukraine invading Poland, and twenty Chinese
/// invasions of Vietnam across twelve runs, which on their own lift China's
/// thirty-year growth out of its calibration band. Distance without a border
/// has to be a real barrier or every region becomes a brawl.
const REACH_BORDER: f64 = 1.0;
const REACH_REGION: f64 = 0.15;

/// What a state wants from a neighbour it claims nothing of. Not zero — border
/// wars happen over nothing — but a tenth of what a whole-country claim is.
const WANT_FLOOR: f64 = 0.10;

/// Even a state on cordial terms is not at absolute zero risk. Everything above
/// this floor is earned by hostility, squared, so relations do most of the
/// sorting: the difference between -20 and -50 is a factor of five and a half.
/// The floor is low because a border alone must not be a casus belli — at 0.04
/// the Soviet Union starts invading Poland, with which its relations are +10.
const GRUDGE_FLOOR: f64 = 0.02;
/// Relations at which hostility is complete. Chosen so the deepest enmities in
/// the 1990 data (India/Pakistan at -55, Israel/Iraq at -70) saturate.
const GRUDGE_SATURATION: f64 = 55.0;

/// Military strength per unit of economy at which a state counts as fully
/// militarised. Iraq in 1990 sits at 2.1 of this, Japan at 0.24.
const MILITARISATION_REF: f64 = 1.6;

/// What is left of an appetite once the war has already been fought and won.
///
/// The old model had only half of this: `burned_A_B`, recorded when an invasion
/// is repelled, permanently raises what the aggressor expects to face. There
/// was no counterpart for the invasion that *succeeds*, so a state that took
/// what it claimed simply came back for it every two years — measured across
/// twelve thirty-year runs on master, 182 of 300 wars were somebody invading
/// Bosnia again. A claim pressed to a conclusion is a claim collected: the
/// grievance survives, the war aim does not.
const SETTLED: f64 = 0.50;

/// Below this a candidate is not worth a die roll. Purely a performance guard —
/// at 190 nations the appetite pass considers every contact of every state
/// every month, and a probability of one in a million is not a history.
const NEGLIGIBLE: f64 = 1e-6;

/// The flag `war.rs` writes when `a` has beaten `t` in a war `a` started, and
/// the flag this module reads to know the claim has been pressed already.
pub fn settled_flag(a: NationId, t: NationId) -> String {
    format!("pressed_{:?}_{:?}", a, t)
}

/// Everyone `a` could plausibly use force against: its neighbours, the states
/// it claims something from, and the rest of its region. Precomputed, so the
/// monthly pass is O(n·k) rather than O(n²).
pub fn contacts(a: NationId) -> &'static [NationId] {
    &a.def().contacts
}

/// How far one state can reach the other at all.
fn reach(a: NationId, t: NationId) -> f64 {
    if adjacent(a, t) {
        REACH_BORDER
    } else if a.region() == t.region() {
        REACH_REGION
    } else if claim_share(a, t) > 0.0 {
        // A claim implies a way of pressing it, or it would not be a claim.
        REACH_REGION
    } else {
        0.0
    }
}

/// Who starts wars. An armed and authoritarian government, and nothing here is
/// a dyad or a national constant: both terms are read off the nation's own
/// transcribed state and both move as the sim runs.
///
/// Authoritarianism is squared deliberately. The claim being made — that open
/// governments very rarely invade their neighbours, and that this is a property
/// of the government rather than of the pair — is the strongest regularity in
/// the period and it is what lets the aggressor list be thrown away.
fn disposition(w: &WorldState, a: NationId) -> f64 {
    let n = w.nation(a);
    let auth = n.authoritarianism.clamp(0.0, 1.0);
    // Strength per unit of economy: how much of itself a state has put under
    // arms, which is a better reading of intent than the budget share alone
    // because it counts the army a state actually has.
    let militarisation =
        (n.mil_strength / (n.gdp.max(1.0).sqrt() * MILITARISATION_REF)).clamp(0.0, 3.0);
    (0.10 + 0.90 * auth * auth) * (0.35 + 0.65 * militarisation)
}

/// How bad it already is between them.
fn grudge(w: &WorldState, a: NationId, t: NationId) -> f64 {
    let h = ((-w.relation(a, t)) / GRUDGE_SATURATION).clamp(0.0, 1.0);
    GRUDGE_FLOOR + (1.0 - GRUDGE_FLOOR) * h * h
}

/// The monthly probability that `a` starts a war on `t`, all in. Zero means it
/// is not a candidate at all and no die is rolled — which matters, because a
/// die rolled is a die that moves every other outcome in the world.
pub fn war_appetite(w: &WorldState, a: NationId, t: NationId) -> f64 {
    if a == t {
        return 0.0;
    }
    let reach = reach(a, t);
    if reach <= 0.0 {
        return 0.0;
    }
    let (an, tn) = match (w.nation_opt(a), w.nation_opt(t)) {
        (Some(x), Some(y)) if x.alive && y.alive => (x, y),
        _ => return 0.0,
    };

    // Deterrence, first, because it is absolute: nobody attacks a nuclear power
    // without an arsenal of their own.
    if tn.nuclear && !an.nuclear {
        return 0.0;
    }

    // Nor does anyone invade a state it has guaranteed. Without this the
    // region floors gave every pair of neighbours a standing appetite with
    // nothing institutional to damp it, and the result was France invading
    // Italy and Saudi Arabia invading Kuwait — eleven of seventy-five wars
    // across twelve runs had no basis in the period, and one was inside NATO.
    // The bible sells playing the present with real names; that is the credit
    // this spends. A guarantee is not a modifier, it is a bar.
    if w.statecraft.pacts.iter().any(|p| {
        (p.a == a && p.b == t) || (p.a == t && p.b == a)
    }) {
        return 0.0;
    }

    // --- What it wants. A claim already taken by force is no longer a war
    // aim, however sour the relationship stays. ---
    let settled = w.has_flag(&settled_flag(a, t));
    let want = if settled { WANT_FLOOR } else { WANT_FLOOR + claim_share(a, t) };

    // --- How easily it could be taken. A state ten times the size of its
    // neighbour is contemplating an occupation it can imagine finishing. ---
    let mass = an.population / (an.population + tn.population).max(1e-9);
    let digestible = 0.35 + 1.30 * mass * mass;

    // --- And what it would be worth. Kuwait was a third of Iraq's economy
    // sitting behind an army of two divisions; Vietnam in 1990 was worth 1.7%
    // of China's and cost a war to take. ---
    let worth = 0.25 + 0.75 * (tn.gdp / an.gdp.max(1e-9)).clamp(0.0, 1.0);

    let base = APPETITE
        * reach
        * want
        * digestible
        * worth
        * disposition(w, a)
        * grudge(w, a, t)
        * if settled { SETTLED } else { 1.0 };
    if base <= 0.0 {
        return 0.0;
    }

    // --- Circumstance: the conditions the old code applied on top of its
    // table, unchanged in substance. ---

    // Fiscal desperation raises appetite. Iraq entered 1990 owing more than its
    // annual output to states it had fought Iran on behalf of.
    let desperation =
        (an.debt_gdp - 0.6).max(0.0) * 1.5 + (0.4 - an.growth_last * 10.0).max(0.0) * 0.2;

    // A neighbour that cannot hold itself together is an invitation: its own
    // minorities are a lever, and its army is busy at home.
    let fragility = 1.0 + tn.separatism * 1.5;

    // Expected defence includes likely interveners — but a first-time gambler
    // discounts them, which is Saddam's 1990 misjudgment. After one repelled
    // invasion the lesson is learned permanently.
    let learned = w.has_flag(&format!("burned_{:?}_{:?}", a, t));
    let coalition_discount = if learned { 1.0 } else { 0.10 };
    let mut expected_def = tn.mil_strength;
    // Read the same list the coalition actually forms from, or the lesson of a
    // repelled invasion is never available to be learned.
    for m in majors() {
        if war::would_intervene(w, *m, t, a) {
            expected_def += w.nation(*m).mil_strength * coalition_discount;
        }
    }
    // A signed guarantee is the one commitment an aggressor cannot pretend not
    // to have seen. It is still discounted — pacts are broken — but far less
    // than a hoped-for coalition.
    for g in w.pact_partners(t) {
        if g == a || majors().contains(&g) && war::would_intervene(w, g, t, a) {
            continue;
        }
        if w.nation_opt(g).is_none_or(|n| !n.alive) {
            continue;
        }
        let honoured = if learned { 0.75 } else { 0.08 };
        expected_def += w.nation(g).mil_strength * honoured;
    }
    let strength_ratio = an.mil_strength / expected_def.max(1.0);

    // Deterrence is graded, not a wall.
    //
    // This was `if strength_ratio < 0.8 { return 0.0 }`, and combined with
    // summing every plausible intervener into expected_def it made a great
    // power's client literally un-invadable: the guarantee deterred the very
    // war that would have tested it, and pacts dragged a major into somebody
    // else's war in 1 run of 12 against a 3-12 band. Every other term in this
    // function is graded; this one alone treated a *hoped-for* intervention as
    // a certainty the aggressor could see and price.
    //
    // Aggressors misjudge, and that is most of what starts wars. Saddam in 1990
    // was not deterred by a coalition he did not believe would form. So the
    // curve falls away steeply below parity without ever reaching zero:
    // 0.9 -> 0.59, 0.8 -> 0.33, 0.6 -> 0.08, 0.4 -> 0.01. powi is exact
    // multiplication, so this costs nothing in cross-platform reproducibility.
    let odds = if strength_ratio >= 1.0 {
        strength_ratio.min(4.0)
    } else {
        strength_ratio.powi(8)
    };

    let p = base
        * w.rules.ai_aggression
        * (1.0 + desperation)
        * fragility
        * odds;
    if p < NEGLIGIBLE {
        0.0
    } else {
        p.min(0.25)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::init::world_1990;

    /// The shape the hand-written table encoded, asserted directly rather than
    /// only through the emergent-history tests downstream of it. These are the
    /// orderings that must survive any retuning of the constants above; a
    /// change that inverts one of them has broken the model, not the test.
    #[test]
    fn the_derived_table_keeps_the_orderings_the_old_one_encoded() {
        let w = world_1990(GameRules::default());
        let kuwait = war_appetite(&w, NationId::Iraq, NationId::Kuwait);
        let saudi = war_appetite(&w, NationId::Iraq, NationId::SaudiArabia);
        let iran = war_appetite(&w, NationId::Iraq, NationId::Iran);
        assert!(
            kuwait > saudi * 5.0,
            "Baghdad's claim on Kuwait no longer dominates: {:.5} vs {:.5} on Saudi Arabia",
            kuwait,
            saudi
        );
        // The old table put Kuwait fifteen times above Iran on the base rate
        // (0.030 against 0.002). The derived model delivers about 5.9x, so the
        // separation has genuinely eroded and Baghdad is keener on Tehran in
        // 1990 than it should be — a war it had just fought to a standstill
        // over eight years. That is recorded in ROADMAP as an open calibration
        // item rather than papered over. The threshold below locks what the
        // model actually holds today so it cannot erode further unnoticed; it
        // is deliberately NOT set at 3.0, which would have admitted a further
        // halving without failing.
        assert!(kuwait > iran * 5.0, "Kuwait {:.5} against Iran {:.5}", kuwait, iran);
        // ...and it is the claim doing it, not Iraq being generally warlike.
        assert!(saudi > 0.0, "an armed neighbour is never at absolute zero");

        // A country with no border, no claim and no quarrel is not a candidate.
        assert_eq!(war_appetite(&w, NationId::USA, NationId::Japan), 0.0);
        assert_eq!(war_appetite(&w, NationId::Brazil, NationId::Nigeria), 0.0);
        // Nobody invades a nuclear power without a bomb of their own.
        assert_eq!(war_appetite(&w, NationId::Iraq, NationId::Israel), 0.0);
    }

    #[test]
    fn the_breakup_is_asymmetric_because_the_census_was() {
        // The whole of `slovenia_escapes_the_wars_that_consume_bosnia`, stated
        // at the level of the model rather than measured across ten runs.
        let mut w = world_1990(GameRules::default());
        for _ in 0..120 {
            crate::tick_month(&mut w, &[]);
            if w.has_flag("yugoslavia_dissolved") {
                break;
            }
        }
        assert!(w.has_flag("yugoslavia_dissolved"), "the federation never came apart");
        let bosnia = war_appetite(&w, NationId::Serbia, NationId::Bosnia);
        let slovenia = war_appetite(&w, NationId::Serbia, NationId::Slovenia);
        assert!(
            bosnia > slovenia * 10.0,
            "Belgrade wants Bosnia {:.5} and Slovenia {:.5} — the asymmetry is gone",
            bosnia,
            slovenia
        );
    }

    #[test]
    fn a_claim_pressed_by_force_stops_being_a_war_aim() {
        let mut w = world_1990(GameRules::default());
        let before = war_appetite(&w, NationId::Iraq, NationId::Kuwait);
        w.set_flag(&settled_flag(NationId::Iraq, NationId::Kuwait));
        let after = war_appetite(&w, NationId::Iraq, NationId::Kuwait);
        assert!(
            after < before * 0.2,
            "a war already won left the appetite intact: {:.5} -> {:.5}",
            before,
            after
        );
        assert!(after > 0.0, "the grievance survives even when the claim does not");
    }
}

#[cfg(test)]
mod diag {
    use super::*;
    use crate::init::world_1990;

    /// Not a test — a readout. `cargo test --release -p spheres-sim -- --ignored
    /// --nocapture appetite_table` prints every dyad the model gives a non-zero
    /// appetite in January 1990, which is the audit the old match statement
    /// used to be.
    #[test]
    #[ignore]
    fn appetite_table() {
        let w = world_1990(GameRules::default());
        let mut rows: Vec<(f64, String)> = vec![];
        for a in all_nations().iter().copied() {
            for t in contacts(a).iter().copied() {
                let p = war_appetite(&w, a, t);
                if p > 0.0 {
                    rows.push((p, format!("{:>12} -> {:<12} {:.5}/mo", a.code(), t.code(), p)));
                }
            }
        }
        rows.sort_by(|x, y| y.0.partial_cmp(&x.0).unwrap());
        println!("\n{} live dyads in Jan 1990", rows.len());
        for (_, r) in &rows {
            println!("{}", r);
        }
    }

    /// Every war-shaped calibration test in the suite, measured at once, so a
    /// change to the appetite model can be judged against all of them in one
    /// run rather than one bisected failure at a time.
    #[test]
    #[ignore]
    fn calibration_dashboard() {
        let mut gulf = 0;
        for seed in 0..10u64 {
            let mut w = world_1990(GameRules { seed, ..GameRules::default() });
            let mut saw = false;
            for _ in 0..48 {
                if crate::tick_month(&mut w, &[])
                    .iter()
                    .any(|h| h.contains("Iraq invades Kuwait"))
                {
                    saw = true;
                }
            }
            if saw || !w.nation(NationId::Kuwait).alive {
                gulf += 1;
            }
        }

        let (mut slovenia, mut bosnia, mut broke) = (0, 0, 0);
        for seed in 0..10u64 {
            let mut w = world_1990(GameRules { seed, ..GameRules::default() });
            for _ in 0..120 {
                for h in crate::tick_month(&mut w, &[]) {
                    if h.starts_with("WAR:") {
                        if h.contains("Slovenia") {
                            slovenia += 1;
                        }
                        if h.contains("Bosnia") {
                            bosnia += 1;
                        }
                    }
                }
            }
            if w.has_flag("yugoslavia_dissolved") {
                broke += 1;
            }
        }

        let (mut dragged, mut settled, mut wars) = (0, 0, 0);
        for seed in 0..12u64 {
            let mut w = world_1990(GameRules { seed, ..GameRules::default() });
            let (mut d, mut s) = (false, false);
            for _ in 0..360 {
                for h in crate::tick_month(&mut w, &[]) {
                    if h.starts_with("WAR:") {
                        wars += 1;
                    }
                    if h.contains("honours its defence pact")
                        && patrons().iter().any(|p| h.starts_with(p.name()))
                    {
                        d = true;
                    }
                    if h.contains("sues for peace") || h.contains("agree peace terms") {
                        s = true;
                    }
                }
            }
            dragged += d as i32;
            settled += s as i32;
        }

        println!(
            "\ngulf {}/10 (>=5)   yugoslavia {}/10 (>=8)   bosnia {} vs slovenia {} (>3x)",
            gulf, broke, bosnia, slovenia
        );
        println!(
            "dragged {}/12 (3..12)   settled {}/12 (>=3)   wars in 12x30y: {}",
            dragged, settled, wars
        );

        let mut w = world_1990(GameRules::default());
        let start = w.nation(NationId::China).gdp;
        for _ in 0..360 {
            crate::tick_month(&mut w, &[]);
        }
        println!("china {:.1}x in 30y (6..14)", w.nation(NationId::China).gdp / start);
    }

    /// A census of what actually happens, seed by seed.
    #[test]
    #[ignore]
    fn war_census() {
        let mut total = 0;
        for seed in 0..12u64 {
            let mut w = world_1990(GameRules { seed, ..GameRules::default() });
            let mut wars = vec![];
            for _ in 0..360 {
                for h in crate::tick_month(&mut w, &[]) {
                    if h.starts_with("WAR:") || h.contains("defence pact") || h.contains("abandons its pact") {
                        wars.push(format!("{} {}", w.date_str(), h));
                    }
                }
            }
            total += wars.len();
            println!("--- seed {} ({} events)", seed, wars.len());
            for x in &wars {
                println!("   {}", x);
            }
        }
        println!("total {}", total);
    }
}
