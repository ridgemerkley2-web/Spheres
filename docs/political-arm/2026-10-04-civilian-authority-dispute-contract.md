# Civilian-authority dispute: design contract (D1)

**Status: Proposed by Claude (automated agent); revised after two automated adversarial reviews; the user, as design authority, decided the blocking questions on 4 Oct 2026 (section 19). Not yet implemented or measured.**

| | |
| --- | --- |
| Task | `CODEX-S27-A1-01`, direction D1 (Codex-owned task; this document was prepared at the user's request) |
| Date | 4 October 2026 (first draft and revision) |
| Author | Claude, automated agent. Not human review, not Codex review, not approval. |
| Reviews | Two adversarial reviews by Claude, automated agent (design-authority lens and engineering-risk lens); not human or Codex review. Recorded verbatim, with the disposition of every finding, in [the review record](2026-10-04-civilian-authority-dispute-contract-review.md). Changes made in response are in section 18. |
| Base | integration tip `39369f0ee6451466d03778d0273a7f72aafcf8c0` (`codex/campaign-certification`); line numbers below are at that tip. The commits through `eac153f7` change documentation only. The D2 commits after it (section 3.1) change `government.rs` only: at the post-D2 tip, `government.rs` lines `8505-8506` sit 2 lines lower, `8507-8575` sit 7 lines lower, and non-test lines from `8576` sit 13 lines lower. Every other file's line numbers are unchanged. |
| Completes | the [1 October mechanism proposal](../campaign-certification/S27/preparation/a1-followup-20261001/mechanism-proposal.md), whose compliance and escalation design, AI objective, costs and refusal rule were explicitly missing |
| Depends on | the user's 4 October decision on D2 (below), implemented and measured as its own change and integrated before any D1 work (status in 3.1) |
| Changes | nothing. No code, data, coefficient, test, cohort or threshold changed in writing or revising this. |

**User decisions this contract implements (design authority, 4 October 2026).**
(a) D2, "Docs: restore the guard": [the 22 September record](2026-09-22-calibration-repairs.md)
is authoritative that civilian control at authoritarianism `<= .20` blocks the Army's
civilian-confidence channel, and the code must apply that guard even where sourced V-Dem
leverage exists. (b) D1: write this contract only, for review; no implementation.

A1, `CODEX-S27-A1-01`, S27, G5 and CP1 stay open. This contract is not evidence for any of them.

> **BLOCKING: no measurement may be predeclared until the design authority rules on
> questions Q1-Q14 (section 17.1).** They are design conflicts the reviews found between
> this contract and a binding rule, a hard constraint for A1 work or a source text. This
> revision takes no position on them, and where the first draft took one (for example the
> "Floor" AI default and funding settling a dispute automatically), that position is
> withdrawn and the drafted behaviour is kept only as one labelled option.
>
> 1. AI temperament (Floor and Assertive appear to recreate forbidden triggers by combination)
> 2. AI issuance: the 0.02 draw or a deterministic review
> 3. Order precedence over stratagem cards
> 4. Escalation gap and route 2's `D >= .25` guard
> 5. Funding and compliance while a dispute is open
> 6. Compliance effect: repeat orders, saved institutional powers, `army_authority`'s two paths
> 7. Rule 9: extending the user's D2 decision to orders, accrual and firing
> 8. Lawful issuer before the first modelled ballot
> 9. Prices, AI reserve and political-capital drain
> 10. Cooldown basis and carry-over to a new governing party
> 11. Pressure surviving a negotiation to a narrower scope
> 12. Enforcement at zero political capital
> 13. Escalation headline suffix and takeover-watch legibility
> 14. Sequencing if D2 moves a gate (on current evidence it moved none)
>
> Questions Q15-Q24 must also be answered before stage 1 is implemented; Q25-Q26 can wait.

---

## 1. Purpose, and the A1 breadth problem it addresses

### 1.1 What is missing from the model

The political arm has two ways for an Army to remove an elected government:

- **Route 2.** A materially or economically distressed Army: `electoral_army_tick`
  (`government.rs:8881`) and `maybe_electoral_coup` (`:8951`).
- **The annulment.** A prospective veto of a new programme at a completed election:
  `annulment_check` (`:7197`), reading `army_programme_veto_loyalty` (`:8591`).

The model has **no representation of a lawful civilian order that an Army refuses**. The
[geography model review](../campaign-certification/S27/preparation/a1-geography-20260928/model-review/README.md)
named the gap as "a separately observable conflict over civilian executive authority". It
required that "a player would need to see and influence that conflict", and that "military
funding would retain its material benefit while not automatically settling the specified
dispute".

This contract specifies that mechanism end to end:

- a government command;
- an institutional compliance rule;
- a receipt-backed dispute record;
- three resolutions;
- an escalation route that exists only while an enforced dispute is unresolved;
- the AI's use of the same command;
- the player-facing screen.

### 1.2 The A1 breadth problem

> **Derived from seeds 0-11; must not inform the decision.** This section diagnoses the A1
> failure on the gate's own cohort. It explains the failure and is not a target (1.3). Its
> country list and readings must not inform the temperament (Q1), any other 17.1 or 17.2
> answer, or any coefficient (11.3).

A1 (`spheres-sim/tests/bloc_census.rs:983-993`, seeds 0-11, 252 monthly ticks, both switches
on) fails at the tip.

- **Count band:** median elected-government coups is 7.5, inside the 4..=14 band.
- **Concentration:** the median per-seed top-three share is 4/7 = 0.571429, against a strict
  bar of `< 0.50`.
- **Per-seed shares:** s0 3/7, s1 4/6, s2 3/5, s3 5/8, s4 4/8, s5 4/8, s6 4/7, s7 5/8, s8 4/8,
  s9 4/7, s10 5/8, s11 4/7.

The scouting diagnosis
([scope packet](../campaign-certification/S27/preparation/a1-claude-scope-20261004/README.md))
found a **breadth** problem. There are 87 elected coups in 8 countries across the 12 seeds.

- Five countries take a first coup in 12 of 12 seeds at almost fixed dates.
- With a fixed core of 5 to 7 single-event countries, a seed's top-three share cannot fall
  below 3/7.
- Every repeat, and every seed with fewer than seven countries, puts that seed at or above 1/2.

Every earlier trial acted on the same crisis-driven population, and all of them stay rejected.

- **Confidence weights .90 and 1.20.**
- **The iteration-22 candidate.**
- **Reassessment 27.**
- **Urgent response `75af467f`.**
- **Trial 01's material-only funding inverse.**

A **quiet, funded, high-leverage** electoral population never fires. Each country below has at
least 1,000 months with discontent under .25 and effective loyalty of at least .35. The figure
is first-observed leverage:

| Country | Leverage | Country | Leverage |
| --- | --- | --- | --- |
| Ghana | 1.0 | El Salvador | .6 |
| Gabon | .833 | Honduras | .6 |
| Thailand | .833 | Fiji | .5 |
| Pakistan | .8 | South Korea | .5 |
| Paraguay | .8 | Turkey | .5 |
| Lesotho | .75 | | |
| Cameroon | .714 | | |
| CAR | .6 | | |

None of these countries has an elected coup. Pakistan and Thailand never meet either of route
2's live conditions: their maximum discontent is .211 and .182, and their minimum effective
loyalty .647 and .650. That is correct for route 2, and its guards (discontent `>= .25`,
effective loyalty `< .35`) are not to be deleted.

A refused civilian order is a different cause, so it is the only identified direction that
could give some of these polities a modelled reason to experience an elected-government coup.

### 1.3 What this contract does not promise

**The effect on A1 is unknown until measured.** It may be zero or negative under any AI
temperament. This contract records **no expected A1 direction** for any temperament, scope,
price or rule. The first draft did (for its proposed "Floor" default); those expectations were
derived from seeds 0-11 and were removed in this revision (section 18, R12), because the
temperament must be chosen on design grounds alone (Q1).

**Nothing in this contract may be tuned to seeds 0-11.** That covers coefficients, scopes,
prices, timing and the choice of AI temperament. Section 1.2's arithmetic explains the failure;
it is not a target. The breadth arithmetic in the scope packet's
`diagnostics/a1-breadth-arithmetic.txt` is the same: an explanatory check, never a goal. A
useful mechanic can still fail A1, and that failure must be reported as a failure.

---

## 2. Historical motivation (only what the repository already holds)

### 2.1 Sourced or recorded in the repository

The archived calibration note [BUGS S4-1](../archive/2026-09-30/BUGS.md) (line 3091) lists
four removals of elected governments as cases that "would calibrate" route 2's rate. Three are
relevant here (Turkey 1980 is discussed below). Its notes on two of them say the crisis-pressure
shape cannot reproduce them:

- **Pakistan, October 1999:** "a paid army, a government at 0.60 of the chamber, no pressure of
  this shape at all — the coup was the army's, not the street's".
- **Thailand, February 1991.**
- **Haiti, September 1991:** "eight months after the vote: faster than any setting of this rate
  reaches from 0.65".

These are a **motivation**, not calibration. S4-1 does not record what civilian order, if any,
preceded any of them. Nothing in this contract is fitted to them. No country or date is
special-cased, and no dated coup is forced. Haiti is never electoral in the model today: all
3,024 of its country-months over seeds 0-11 are regime months. A dispute cannot occur there unless D3 (the opening-route
review) changes that.

S4-1 also names Turkey 1980 ("two years of street war before the generals moved"). That case
is the crisis route's, not this one's.

Related repository text, cited for context only:

- **The 22 September record.** It says that "A financial appropriation is not an automatic
  cure for political opposition to civilian authority". It also cites the archived 1990 USDOS
  report on Honduras, which "describes the elected government and Armed Forces' legal and
  institutional autonomy". That establishes institutional presence and autonomy, not a dispute.
- **Ridge's R2 ruling, quoted at `annulment_check`.** It cites "Turkey's 1997 memorandum came
  from a hostile general staff". That is a hostile military acting on an elected government; it
  is the annulment's basis and is not evidence about civilian orders.
- **Powell (2012), pp. 1020-1021, already cited at `army_civilian_confidence_penalty`.** It
  distinguishes public legitimacy from military corporate grievances. It is directional
  grounding only and supplies no coefficient here.

### 2.2 Not sourced in the repository (flagged)

The following are general knowledge from Claude. **No repository source supports them.**

- **Pakistan 1999:** the dismissal of the army chief.
- **Thailand 1991:** a civilian government's dispute with the military leadership over
  appointments.
- **Haiti 1991:** military reforms by the elected president.
- **Other anchors:** Honduras 2009, Fiji 2006, and Paraguay 1996 (an order refused without a
  coup).

None of them may inform a coefficient, a scope, a price or a test until a research packet
sources and reviews it under the C01 research workflow. Until then, every number in this
contract is labelled an invented design assumption.

---

## 3. Boundaries and sequencing

### 3.1 Prerequisite: D2 lands first

D2 must land first, as its own change with its own red regression and measurement. Its
predeclared plan is
[`civilian-guard-20261004/plan.json`](../campaign-certification/S27/preparation/civilian-guard-20261004/plan.json),
committed in `eac153f7` before any runtime edit.

**D2 as implemented (facts, checked for this revision).**

- **Commits.** The guard is `215bd023` and a test follow-up is `2bce829a`, with evidence in
  `e857e8ac`, on `claude/nifty-bohr-0j27qi`. The same patches are `51a7ddfa` and `f6597cf5`
  on the local branch `claude/civilian-guard-wip`. None is integrated into
  `codex/campaign-certification`.
- **Where the guard lives.** It is a **private** constant,
  `const CIVILIAN_CONTROL_AUTHORITARIANISM: f64 = 0.20;` (post-D2 `government.rs:8513`), read
  in one place: inside `army_civilian_confidence_penalty` (`:8573` here, `:8580` post-D2),
  after its `leverage <= 0` return, as
  `if w.nation(id).authoritarianism <= CIVILIAN_CONTROL_AUTHORITARIANISM { return 0.0; }`.
  The penalty's consumers, the two Army lines of `pillar_targets` and `ai_army_funding_floor`,
  inherit it.
- **What D2 leaves unguarded.** Three things are unchanged:
  - `civilian_army_executive_leverage` (`:8508`), including its unknown-assessment proxy;
  - the programme veto, `army_programme_veto_loyalty`. It is runtime-inert at `<= .20` today,
    because its only consumer, `annulment_check`, returns `None` below `.35`. Whether it should
    carry the guard explicitly is left open by the D2 packet and review. That is a D2 question
    and is not decided here.
  - the live `army_authority` records.
- **There is no `civilian_control_holds(w, id)`.** The first draft of this contract referred to
  that helper as if D2 provided it. It does not exist.
- **Gate result.** The local Linux run of the unchanged gate printed every value identical to
  the `39369f0` baseline, including the per-seed A1 fractions; the 12 seed rows hash to
  `87b3ea9b…3deb`. The Windows and CI runs are still pending. *(Update, later on 4 Oct: CI run 37178241397 on `e857e8ac` reproduced the identical A1 seed table on ubuntu and windows, and every other CI result passed; see section 9 of the D2 packet README.)*

**D1 therefore cannot rely on D2 to keep disputes out of civilian-control states.** It carries
its own explicit guard: legality rule 9 and the validation in 8.3. Rule 9's reach beyond the
confidence channel is blocking question Q7.

**D1's measurement baseline is the post-D2 tip, not `39369f0`.** On the D2 evidence above the
post-D2 baseline **equals** the pre-D2 baseline (section 15, step 3). If D2's pending Windows or
CI runs, or its integration, show a different value, the post-D2 values become D1's baseline
and Q14 applies. D2's effect is recorded separately and never credited to D1.

**What D1 stage 1 would have to add.** To keep the boundary defined once, stage 1 must:

- promote `CIVILIAN_CONTROL_AUTHORITARIANISM` to `pub(crate)`;
- add the predicate `pub(crate) fn civilian_control_holds(w, id) -> bool`, true when current
  authoritarianism `<= CIVILIAN_CONTROL_AUTHORITARIANISM`;
- replace the penalty's inline comparison with a call to it, keeping it after the
  `leverage <= 0` return so the penalty stays bit-identical. D2's six guard tests must pass
  unmodified.

D1 never writes a second `0.20` literal. If the design authority prefers D1 not to touch D2's
code, the alternative is for D1 to read the promoted constant directly, leaving the penalty
untouched.

### 3.2 Two stages, each with its own red regression

- **Stage 1: the order and the record.** This stage adds:
  - the command, legality, price and compliance;
  - the `DisputeRecord`, the three resolutions and the decision window;
  - the AI issuance and response rules, and the UI.

  Nothing can remove a government in stage 1: an enforced dispute records its stance but
  accrues no pressure. This is the proposal's step 2: "a receipt-backed dispute record and a
  read-only assessment before any overthrow path".
- **Stage 2: escalation.** This stage adds authority-pressure accrual and firing. It needs its
  own failing regression on the stage-1 runtime (proposal step 4).
- **Gate measurement** happens once, on the complete stage-2 candidate (section 15).

### 3.3 Not changed by this contract

- `spheres-sim/tests/bloc_census.rs` stays byte-identical: every assertion, cohort, horizon and
  headline definition.
- Route 2:
  - `electoral_army_tick`;
  - `maybe_electoral_coup`;
  - the live `D >= .25` and `eff < .35` guards;
  - the 12-month settled rule;
  - the pressure rate and cap.
- `pillar_targets`, the confidence penalty, and `ARMY_CRISIS_CONFIDENCE_WEIGHT` (.65).
- The material loyalty model and the AI funding policy (`ai_army_funding_floor`).
- The annulment and the programme veto. The veto's arithmetic may be factored into a shared
  helper (section 7.2), but it must stay bit-identical.
- The single 0.02 draw in `stratagems::ai_stratagems`. Its code is unchanged; the new lever
  arrives through `government::ai_lever` (section 11.4). Whether it should take precedence over
  cards there is blocking Q3.
- All rejected policies stay rejected.

### 3.4 New files and touched files (for the implementer)

| Path | Role |
| --- | --- |
| `spheres-sim/src/civilian_authority.rs` (new) | Types, constants, plan, refusal, effects and arm functions, `tick`, AI helpers. Keeps `government.rs` (15,822 lines) from growing. |
| `spheres-sim/src/civilian_authority_tests.rs` (new, `#[cfg(test)]`) | Red regressions and controls (section 14) |
| `spheres-sim/src/government.rs` | Changes, by area. **`GovState`:** two fields; the single `GovState { .. }` literal at `:6114`. **Hooks:** the validation call in `tick`'s per-nation loop **before** the `if is_electoral(w, id)` split (`:9505`; section 8.3); the call in `tick`'s electoral branch after the route-2 block at `:9543-9548`; a `regime_break` hook; an `ai_lever` branch; the `ai_government` month-end response. **Visibility:** `pub(crate)` on `established_civilian_record` and `civilian_army_executive_leverage` (`electoral_coup_settled_months`, `:8857`, is already `pub`); promote D2's private `CIVILIAN_CONTROL_AUTHORITARIANISM` and add `civilian_control_holds` (3.1). **Readers:** one institution-name reader, `institution_spec_name` (4.2). **Veto helper:** the shared acceptance helper. |
| `spheres-sim/src/army_authority.rs` | Make `model_date` `pub(crate)`. Compliance reuses `consolidate_transfer` (`:127`) unchanged. |
| `spheres-sim/src/lib.rs` | Two `Command` variants (beside the levers at `:295-305`); `command_price` (`:386`); `world_refusal` (`:702`); `dispatch` (`:1026`, reached from `apply_command` at `:900`). |
| `spheres-sim/src/government_a1_observer.rs` | Optional test-only stages `dispute_before_tick` and `dispute_after_tick` |
| `spheres-web/src/main.rs` | Actions in `government_json` (`:1351`), `parse_command` (`:6516`), `effects_of` (`:1270`) |
| `spheres-web/src/government_view.rs` | Briefing attention item (`enrich`, `:92`); preview change rows and warnings (`:234`); `category` (`:64`). Its private `institution_name(w, id, Pillar)` (`:58`) is replaced by a call to the sim reader with ordinal 0, keeping its own `pillar.key()` fallback, so every existing payload stays byte-identical (4.2). |
| `spheres-web/ui/government-ui.js` | Render `action_indices` on an attention card (presentation only) |
| `docs/government-ui.md`, SPEC §4 | Record the surface and the lever, only after approval |

---

## 4. The command

### 4.1 Name and parameters

```rust
// spheres-sim/src/lib.rs, beside the five levers
/// A lawful order from an elected government bringing part of an Army
/// institution's executive authority under civilian control. The institution
/// may refuse; a refusal is recorded as a dispute.
AssertCivilianAuthority {
    nation: NationId,
    institution: civilian_authority::InstitutionRef,
    scope: civilian_authority::AuthorityScope,
},
/// Settle the nation's one unresolved dispute.
ResolveAuthorityDispute {
    nation: NationId,
    action: civilian_authority::DisputeAction, // Negotiate | Withdraw | Enforce
},
```

```rust
// spheres-sim/src/civilian_authority.rs
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DisputeAction { Negotiate, Withdraw, Enforce }
```

**Derives.** `Command` derives `Clone, Debug, Serialize, Deserialize, PartialEq` (`lib.rs:105`), so
every type a `Command` variant carries (`InstitutionRef`, `AuthorityScope`, `DisputeAction`) needs
at least those; all three derive them plus `Copy` and `Eq`. The saved records in 8.1 derive
`Clone, Debug, PartialEq, Serialize, Deserialize`, because `GovState` derives
`Clone, Debug, Serialize, Deserialize` (`government.rs:5907`) and the controls compare records.
The plan structs in 4.4 derive `Clone, Debug, PartialEq`, like the existing lever plans (for
example `LegalizePlan`). `f64` fields rule out `Eq` on records and plans.

### 4.2 Institution identity (same-type institutions are never collapsed)

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InstitutionRef {
    pub pillar: Pillar, // must be Pillar::Army in this contract
    pub ordinal: u8,    // 0 = the first stored Army entry, 1 = the second, ...
}
```

- **Ordinal.** `ordinal` indexes the entries of `GovState.pillars` whose kind is `pillar`, in
  stored order. This is the order that `walk_pillars` (`:8774`) walks per institution under the
  lens, so Sudan's "Sudanese Armed Forces" (ordinal 0) and "Popular Defence Forces" (ordinal 1)
  stay distinct.
- **Ordinal correspondence (asserted, not assumed).** The name comes from the polity table
  (`polity_in(w, id).pillars`), while the ordinal indexes the saved `GovState.pillars`. The two
  lists are not guaranteed to match: `seat_spec_pillars` (`:8932`) appends a spec kind only when
  no entry of that kind is stored, at the end of the list, and never adds a second same-kind
  entry. So stage 1 adds a test that, for every 1990 polity and after each reseat path
  (`seat_spec_pillars`, `regime_break`'s reseat, the regime branch's `needs_pillars` seed), the
  k-th stored entry of a kind corresponds to the k-th spec entry of that kind wherever both
  exist. A stored entry with no spec counterpart takes the caller's fallback. An ordinal at or
  beyond the stored count is refused by legality rule 7.
- **Name: one reader.** The single reader is a new sim function,
  `institution_spec_name(w, id, InstitutionRef) -> Option<&'static str>`. It returns the
  `ordinal`-th same-kind spec name, or `None`. Callers keep their own fallbacks:
  - the dispute uses the existing `pillar_name` fallback (`:9302`, "the security apparatus");
  - `spheres-web`'s private `institution_name(w, id, Pillar)` (`government_view.rs:58`) is
    replaced by a call with ordinal 0 and its existing `pillar.key()` fallback, so its output is
    byte-identical.

  The name is **copied into the receipt at issue time**, so a later table change cannot rename a
  recorded dispute. This leaves one name reader instead of two readers with the same name in
  different crates.
- **Leverage.** The sourced leverage is national. `ArmyAuthority` is one record per nation, and
  `army_seizure` sets it for any Army mover. So compliance by either of two Army institutions
  reduces the one national leverage. Per-institution leverage is open question Q20.

### 4.3 Scopes (an ordered ladder)

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityScope { BudgetOversight, SeniorAppointments, CivilianCommand }
```

| Scope | Stake `s` | Order phrase (served text) | Narrower |
| --- | --- | --- | --- |
| `BudgetOversight` | `1.0 / 3.0` | "civilian audit of its budget" | none |
| `SeniorAppointments` | `2.0 / 3.0` | "civilian control of senior appointments" | `BudgetOversight` |
| `CivilianCommand` | `1.0` | "the civilian chain of command" | `SeniorAppointments` |

The stakes are an **invented design assumption** (register entry C2). They are three evenly
spaced steps from part of the Army's executive authority to all of it. The full order carries
stake 1.0, the same maximum as a programme veto by a winner holding the whole chamber. The
categories are game abstractions; no source supplies them.

### 4.4 Plan structs (iron rule 8: refusal, then plan, then arm, then effects, off one plan)

Following the levers' contract at `government.rs:7514-7525`, each command has four functions
off one plan.

- `order_refusal(w, id, inst, scope) -> Option<String>` and
  `resolution_refusal(w, id, action) -> Option<String>`.
  - **Behaviour:** pure; they read the switches before any state.
- `order_plan(..) -> Result<OrderPlan, String>` and
  `resolution_plan(..) -> Result<ResolutionPlan, String>`.
  - **Behaviour:** every number the arm will write, computed once and clamped where the world
    clamps.
- `assert_civilian_authority(w, ..)` and `resolve_authority_dispute(w, ..)`.
  - **Behaviour:** the arms; they write the plan's numbers and nothing else.
- `order_effects(..)`, `resolution_effects(..)`, and `civilian_authority::effects(w, &Command)`.
  - **Behaviour:** the card's lines, rendered from the same plan.
  - **Served by:** `effects_of` in `spheres-web/src/main.rs`.

**Validation comes first in every one of these functions.** Each refusal, plan, arm and effects
function, and the AI's `ai_order` and `ai_response`, first evaluates the pure
`pending_lapse(w, id) -> Option<Resolution>` (8.3). A stored record that would lapse is treated
as **already closed today** with that resolution. That is the "post-lapse view": the issuer may
already have been replaced earlier in the same tick, for example by a ballot in `hold_election`
after the dispute tick and before the month-end AI review. The arms then write that closure,
through the same `close_lapsed` function the tick uses, before writing anything else. The plan
carries it as `closes_stale`, so card, preview and world agree.

```rust
#[derive(Clone, Debug, PartialEq)]
pub struct OrderPlan {
    pub closes_stale: Option<Resolution>,         // a lapsing record the arm closes first
    pub institution: InstitutionRef, pub institution_name: String,
    pub scope: AuthorityScope, pub stake: f64,
    pub loyalty: f64, pub nationalist_backing: f64, pub leverage: f64,
    pub acceptance: f64, pub line: f64,           // line = ELECTORAL_COUP_ARMY
    pub complies: bool,
    pub leverage_after: f64,                      // == leverage when refused
    pub price_pc: f64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ResolutionPlan {
    pub action: DisputeAction, pub scope_after: AuthorityScope,
    pub acceptance_after: f64, pub complies_after: bool,
    pub leverage_after: f64, pub price_pc: f64,
    pub pressure: f64, pub pressure_rate_per_month: f64, pub threshold: f64,
    pub enforcement_upkeep_per_month: f64,
    pub next_order_from: Option<(i32, u32)>,      // the cooldown date a withdrawal sets
}
```

A test compares the card's numbers with the world's bit for bit (section 14).

---

## 5. Legality

`order_refusal` answers in this order and returns the first sentence that applies. `world_refusal`
calls it, and the arm calls it again before writing.

| # | Condition refused | Served sentence (exact form) |
| --- | --- | --- |
| 1 | `!rules.ideology_blocs` | `NO_MOVEMENTS`: "This world does not model ideological movements." |
| 2 | `!rules.ideology_takeover` | `AUTHORITY_ORDERS_CLOSED`: "Civilian-authority orders are closed while the takeover routes await calibration." |
| 3 | Nation dead, or no `GovState` | "This government no longer exists." |
| 4 | `!is_electoral(w, id)` | "{N} holds no elections; there is no elected government to issue the order." |
| 5 | Not `g.elected && g.unrestricted_mandate && !g.awaiting_first_election` | "{N}'s government has not won an unrestricted completed ballot; an interim or restricted mandate cannot order the armed forces." |
| 6 | `established_civilian_record(w, id)` is `None` (`:8516`: under 6 months, or not the current party's record) | "{N}'s government has no established six-month record in office." |
| 7 | No entry of `institution.pillar` at `institution.ordinal` in `g.pillars` | "{N} has no such institution." |
| 8 | `institution.pillar != Pillar::Army` | "Only an Army institution can be ordered under civilian authority." |
| 9 | **Civilian-control guard.** `civilian_control_holds(w, id)`, i.e. current authoritarianism `<= .20` (D2's boundary; the predicate is added by D1 stage 1, see 3.1) | "{N} already holds civilian control: authoritarianism {a:.2}, at or under 0.20." |
| 10 | `army_authority::current_leverage(w, id)` is `None` (no sourced assessment: an unknown or old save) | "No sourced assessment of {institution}'s executive leverage exists; the order has nothing to transfer." |
| 11 | `civilian_army_executive_leverage(w, id) <= 0` | "{institution} holds no executive leverage to transfer." |
| 12 | An unresolved dispute is open in this nation, with any institution, **on the post-lapse view** (4.4): a record that `pending_lapse` would close does not count | "{N} is already in dispute with {institution'} over {phrase'}." |
| 13 | Cooldown: fewer than 12 months since the last closed receipt (C10), **on the post-lapse view**: a lapsing record counts as closed today. Whether a lapse or another party's receipt starts the cooldown for a new governing party is Q10. | "{N} settled its last order in {yyyy-mm}; another may be issued from {yyyy-mm}." |
| 14 | Political capital below the price | The standard `standing_refusal` from `refusal_of` (`lib.rs:857`) |

Notes:

- **Rule 9 is the dispute's own guard.** D2 guards only the confidence penalty, and
  `civilian_army_executive_leverage` still returns sourced leverage at `<= .20`. Without rule 9,
  Uruguay (.12, leverage .6) or Papua New Guinea (.15, .5) could legally receive an order. Rule
  9 would give D1 this rule: **no order, dispute, accrual or firing at current authoritarianism
  `<= .20`.**
- **Rule 9 extends the user's decision; it does not merely apply it (blocking Q7).** The 4
  October decision names the Army's civilian-confidence channel. Applying the same boundary to
  orders, accrual and firing (rule 9, I11, the `CivilianControl` lapse) is this contract's
  proposal. It protects A6, but it extends a design-authority ruling and needs the user's
  confirmation before measurement.
- **Rule 5 decides who is a lawful issuer (blocking Q8).** It requires `g.elected`, which
  means a ballot has been held under this module, plus the six-month record. Every 1990
  incumbent is therefore excluded until its first modelled ballot, including governments with a
  sourced opening mandate. The first draft made that choice silently. It has A4 timing
  consequences (before end-1996).
- **Rule 10.** Rule 10 keeps old saves and nations without a 1989 source row out. No hostility
  or leverage is inferred, and the `(auth - .20)/.40` proxy is never used to permit an order.
- **What an order never needs.** Legality never reads discontent, poverty, party, popularity or
  a historical date. An order needs a lawful issuer and a real institution with real sourced
  leverage, and nothing else.

---

## 6. Price and the command machinery

| Command | `command_price` (`lib.rs:386`) | Charged |
| --- | --- | --- |
| `AssertCivilianAuthority` | `(nation, AUTHORITY_ORDER_PC = 25.0, REFUSABLE)` | Once the arm returns `Ok`, whether the institution complies or refuses. A refusal is the institution's answer, not a world refusal. |
| `ResolveAuthorityDispute { Negotiate }` | `(nation, 0.0, REFUSABLE)` | Its cost is the scope given up |
| `ResolveAuthorityDispute { Withdraw }` | `(nation, AUTHORITY_WITHDRAW_PC = 12.0, ALWAYS)` | Always available and charged to the point of bankruptcy. This is the rule `ExpelFromGovernment` uses for breaking one's word (`lib.rs:603-604`). |
| `ResolveAuthorityDispute { Enforce }` | `(nation, 0.0, REFUSABLE)` | Nothing up front. While enforced, `AUTHORITY_ENFORCEMENT_UPKEEP = 0.20` PC a month is charged in the dispute tick, as `pc = (pc - 0.20*dt).max(0.0)`, the same form as `upkeep` at `government.rs:9614-9617`. |

**Enforcement at zero political capital (blocking Q12).** Because upkeep floors at zero, as the
existing `upkeep` does, an enforced order continues at no cost once political capital reaches
0. The first draft left this unstated. Whether enforcement should instead lapse, for example
into a withdrawal, is a design choice.

**Political-capital drain (blocking Q9).** An AI that issues orders spends 25 PC at most once
per cooldown, plus .20 PC a month while enforcing. That competes with the AI's other uses of
political capital, at the holdings its rules require: `SuspendConstitution` (55 held in
`ai_lever`; price 40), `BanParty` (60 held; price 18), cards (cost + 20) and `SecurePillar`
(above 55 held, in `ai_government`). It is a behavioural
path to A2, A4 and A7, separate from the RNG shift (11.4). The prices (C5-C7) and the AI
reserve (C12) are therefore decided together in Q9.

`world_refusal` (`lib.rs:702`) gains both commands beside the levers at `:750-754`, so the
switch sentence outranks the treasury's. `dispatch` (`:1026`), reached from `apply_command`
(`:900`), dispatches to the two arms beside the lever arms at `:1412-1418`.
`price_of` and `affordable` need no change.

---

## 7. Compliance rule

### 7.1 Formula

The rule is evaluated by `acceptance(w, id, inst, scope) -> Option<f64>`. It is pure, draws no
RNG, and returns `None` where legality rules 3-11 fail.

```text
loyalty_k  = stored loyalty of the inst.ordinal-th Army entry of GovState.pillars
F_N        = blocs::backing(w, id)[Nationalist]          (the term effective_army_loyalty subtracts)
leverage   = government::civilian_army_executive_leverage(w, id)   (unchanged by D2; rule 9 and 8.3 exclude auth <= .20)
s          = scope.stake()                                (1/3, 2/3, 1)

acceptance = (loyalty_k - F_N) - ARMY_PROGRAMME_VETO_WEIGHT * leverage * s
complies   = acceptance >= ELECTORAL_COUP_ARMY            (0.35)
refuses    = acceptance <  ELECTORAL_COUP_ARMY
```

For ordinal 0, `loyalty_k - F_N` is exactly `blocs::effective_army_loyalty` (`blocs.rs:378`),
which route 2 and the annulment read.

### 7.2 Reuse of the programme-veto structure

`army_programme_veto_loyalty` (`government.rs:8591-8601`) computes
`loyalty - ARMY_PROGRAMME_VETO_WEIGHT * leverage * mandate`. `annulment_check` then treats
`prospective < ELECTORAL_COUP_ARMY` as a hostile veto. The order uses the same structure:

- **Loyalty:** the same loyalty term.
- **Leverage:** the same leverage reader.
- **Weight:** the same weight.
- **Line:** the same .35 line.
- **Stake:** the order's scope stake replaces the winner's seat share.

The implementer should factor a shared pure helper,
`fn prospective_acceptance(loyalty: f64, leverage: f64, stake: f64) -> f64 { loyalty - ARMY_PROGRAMME_VETO_WEIGHT * leverage * stake }`.
`army_programme_veto_loyalty` then calls it **with the identical operation order**, so its
results stay bit-identical and the Algeria chronology tests are unaffected.

### 7.3 Effect of compliance

Compliance, whether at issue, after negotiation, or while enforced, calls the existing function
unchanged:

```rust
crate::army_authority::consolidate_transfer(w, id, AUTHORITY_CONSOLIDATION * s); // 0.05 * s
```

That lowers live leverage by `0.05*s/0.40 = 0.125*s`, floored at 0, and stamps
`updated_on = model_date(w)`. As drafted, authoritarianism, loyalty, equipment, funding and the
immutable 1989 source are **not** changed (Q6).

**This effect is a blocking design conflict (Q6).** As drafted, scopes are only stake
multipliers. No institutional power is saved, so the same scope can be ordered again after
every cooldown, and each compliance calls `consolidate_transfer` again. The reviews found
four conflicts:

- **The proposal.** Its step 1 requires specifying "which saved institutional powers an order
  would remove" before any runtime trial.
- **`consolidate_transfer` itself.** Its doc comment says to use the lawful-transfer amount
  "once". Its caller (`government.rs:7154-7163`) prorates explicitly to stop "repeated snap
  elections from manufacturing extra institutional gains".
- **A ratchet.** Under any temperament that issues orders, including Prudent, quiet funded armies
  could ratchet toward zero leverage: up to .125 per cooldown this way, against .125 per full
  term for lawful transfer. That shifts confidence penalties, programme vetoes and AI funding
  paths across the world.
- **`army_authority`'s module contract.** It allows live leverage to change only "through an
  actual military seizure or a qualifying civilian handover" (`army_authority.rs:3-4`).
  Compliance would be a third path.

The options are in Q6. This contract does not choose among them.

**Why the AI wants this.** The lower leverage is the AI's positive objective. It shrinks the two
existing channels that read leverage:

- the crisis-confidence penalty, `0.65 * leverage * crisis * (1 - mandate)`;
- the programme veto, `0.45 * leverage * seats`.

### 7.4 Funding versus settlement (blocking Q5)

Paying the Army raises `loyalty_k` and therefore acceptance. As drafted (9.4), a dispute closes
as `CompliedWhileOpen` with no government action whenever funding lifts acceptance to .35. It
then has its own receipt and headline.

**That default conflicts with two of the three source texts:**

- **The geography model review:** "military funding would retain its material benefit while
  not automatically settling the specified dispute"
  ([model review](../campaign-certification/S27/preparation/a1-geography-20260928/model-review/README.md), line 22).
- **The 22 September record:** "A financial appropriation is not an automatic cure for
  political opposition to civilian authority" ([record](2026-09-22-calibration-repairs.md),
  lines 116-117).

Only the mechanism proposal's weaker wording ("cannot *silently* settle the authority dispute")
supports it. The first draft claimed this was the material benefit "the geography review
requires to be kept". That misread the review, and the claim is withdrawn. Whether, and how,
material recovery may end a dispute is blocking question Q5.

### 7.5 Illustration (held inputs; not a prediction)

> **Derived from seeds 0-11; must not inform the decision.** The inputs are the scout's
> observed minimum effective loyalties on seeds 0-11: Pakistan .647 at leverage .8, and
> Thailand .650 at leverage .833. The table only checks the formula's arithmetic. The
> accept/refuse readings must not inform the temperament (Q1), the scopes (Q15), the weight
> (Q16) or any other choice.

| Scope | Pakistan acceptance | Thailand acceptance |
| --- | --- | --- |
| `CivilianCommand` | .647 - .45×.8×1 = **.287 refuses** | .650 - .375 = **.275 refuses** |
| `SeniorAppointments` | .647 - .240 = **.407 accepts** | .650 - .250 = **.400 accepts** |
| `BudgetOversight` | .647 - .120 = **.527 accepts** | .650 - .125 = **.525 accepts** |

An Army refuses even `BudgetOversight` only when `loyalty_k - F_N < .35 + .15*leverage`. That
is under .47 at leverage .8, and under .50 at leverage 1.0.

### 7.6 Coefficient register

Every coefficient below is an **invented design assumption**. None is calibrated, and none may
be fitted to seeds 0-11. SPEC §4's convention is that every invented coefficient is "filed in
BUGS with what would calibrate it". The last column is the draft of those BUGS entries, to be
filed only on approval. Where it says "none in the repository", no calibration source exists
yet, and the research packet (Q26) is the route to one.

| ID | Constant | Value | Status | Basis | What would calibrate it |
| --- | --- | --- | --- | --- | --- |
| C1 | `ARMY_PROGRAMME_VETO_WEIGHT` (reused) | .45 | Invented design assumption (22 Sep record) | Same structure as the approved programme veto: loyalty − W·leverage·stake. No new weight is introduced. (Q16) | A sourced set of civilian orders to armed forces, with the institution's assessed leverage and whether it complied. None in the repository. |
| C2 | Scope stakes | 1/3, 2/3, 1 | Invented design assumption | Three evenly spaced steps; the full order's maximum stake equals the veto's maximum (whole chamber). No source. (Q15) | Nothing empirical: a game abstraction of scope. Revisit only if a reviewed research packet classifies sourced orders by scope. |
| C3 | Compliance line, `ELECTORAL_COUP_ARMY` (reused) | .35 | Invented design assumption (design S4, route 2) | The hostile-Army line already shared by route 2 and, through R2, the annulment | As route 2's line (its BUGS S4 entry). |
| C4 | `AUTHORITY_CONSOLIDATION` | .05 × stake (leverage −.125 × stake) | Invented design assumption; **blocking Q6** | Reuses the lawful-transfer consolidation amount (`government.rs:7160-7163`, .05 per full term) through `consolidate_transfer`. A complied full order equals one full term of lawful transfer. | Sourced change in assessed military executive-removal leverage after a complied civilian order. None in the repository. |
| C5 | `AUTHORITY_ORDER_PC` | 25 PC | Invented design assumption; **blocking Q9** | Priced like `CallElection` (25): the other constitutional gamble open to an elected government | Play-testing of the political-capital economy; no historical calibration exists. |
| C6 | `AUTHORITY_WITHDRAW_PC` | 12 PC, `ALWAYS` | Invented design assumption; **blocking Q9** | `ExpelFromGovernment`'s 12 PC: always available and never free | As C5. |
| C7 | `AUTHORITY_ENFORCEMENT_UPKEEP` | .20 PC/month | Invented design assumption; **blocking Q9, Q12** | `upkeep = strain × 0.20` (`:7328`) at one strain point: the floor cost of one coalition partner | As C5. |
| C8 | Decision window | the refusal month plus the whole following calendar month | Invented design assumption (Q17) | One full month for a player on any clock; the AI decides at its own month-end review | Play-testing; no historical calibration exists. |
| C9 | Escalation slope and cap (reused) | .60 per unit of gap per month (= .30 × 2); cap 1.5 | Invented design assumption (design S4, route 2); **blocking Q4**, Q18 | Route 2's hostility slope `0.30·2·(0.35 − eff)`, with the refusal gap in place of `(0.35 − eff)` and no discontent term | Sourced durations from a recorded refusal to a removal. None in the repository. |
| C10 | `AUTHORITY_ORDER_COOLDOWN_MONTHS` | 12 | Invented design assumption; **blocking Q10** | `ELECTORAL_COUP_SETTLED` (12) reused as the minimum interval between orders | Play-testing; no historical calibration exists. |
| C11 | Firing threshold and settled months (reused) | `1/crisis_intensity.max(0.1)`; 12 | Invented design assumption (design S4) | Route 2's own threshold and honeymoon | As route 2 (its BUGS S4 entry). |
| C12 | AI reserve (reused) | price + 20 PC | Invented design assumption (the deck); **blocking Q9** | The card rule `s.cost <= held - 20` (`stratagems.rs:567`) | As the card rule. |
| C13 | AI majority line (reused) | `government_seats() >= .50` | Invented design assumption | The majority line of `suspend_refusal` (`:7621`) | As `suspend_refusal`. |
| C14 | AI discontent line (reused) | `D < .25` | Invented design assumption | `ELECTORAL_COUP_DISCONTENT` | As route 2. |
| C15 | Civilian-control guard | auth `<= .20` | Design authority (user, 4 Oct 2026; 22 Sep record); extension to D1 is **blocking Q7** | D2 | Not a coefficient: a design-authority boundary. |
| E1 | `AUTHORITY_HISTORY_KEPT` | 8 receipts per nation | Engineering bound, not a model coefficient | Save size | Not applicable. |

---

## 8. State: `DisputeRecord`

### 8.1 Fields

```rust
// spheres-sim/src/government.rs, GovState (the single literal at :6114 sets both empty)
/// The one unresolved civilian-authority dispute, if any. Old saves have none.
#[serde(default, skip_serializing_if = "Option::is_none")]
pub authority_dispute: Option<crate::civilian_authority::DisputeRecord>,
/// Closed orders and disputes, oldest first, at most AUTHORITY_HISTORY_KEPT.
#[serde(default, skip_serializing_if = "Vec::is_empty")]
pub authority_history: Vec<crate::civilian_authority::AuthorityReceipt>,
```

```rust
type Date = (i32, u32, u32); // army_authority::model_date: (y, m, 1) monthly, (y, m, d) daily

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderReceipt {
    pub issued_on: Date,
    pub nation: NationId,
    pub issuer: String,              // PoliticalRecord.government at issue, "party:<id>"
    pub institution: InstitutionRef,
    pub institution_name: String,    // resolved once, at issue
    pub scope: AuthorityScope,       // as ordered
    pub acceptance: f64,             // quoted at issue
    pub leverage: f64,               // live leverage read at issue
    pub price_pc: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DisputeRecord {
    pub order: OrderReceipt,
    pub refused_on: Date,
    pub scope: AuthorityScope,       // current scope; narrowed by negotiation
    pub status: DisputeStatus,
    pub authority_pressure: f64,     // 0..=1.5; accrues only while Enforced (stage 2)
    pub steps: Vec<DisputeStep>,     // each negotiation and enforcement, in order
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum DisputeStatus {
    Refused { decide_by_month: i32 },   // clock::month_index of the window's last month
    Enforced { since: Date },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DisputeStep { pub on: Date, pub kind: StepKind, pub scope: AuthorityScope, pub acceptance: f64 }
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StepKind { Negotiated, Enforced, EnforcedByDefault }

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorityReceipt {
    pub order: OrderReceipt,
    pub refused_on: Option<Date>,    // None: complied at issue
    pub final_scope: AuthorityScope,
    pub steps: Vec<DisputeStep>,
    pub closed_on: Date,
    pub resolution: Resolution,
    pub leverage_after: f64,         // live leverage when this receipt is written (see below)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Resolution {
    Complied,                  // accepted at issue
    NegotiatedCompliance,      // accepted after narrowing
    CompliedWhileOpen,         // acceptance reached the line while the dispute was open
    Withdrawn,
    Escalated,                 // stage 2: the institution removed the government
    GovernmentRemoved,         // another break (route 2, annulment, any coup) removed the issuer
    IssuerReplaced,            // a ballot or formation changed the governing party
    ElectoralRuleEnded,        // suspension, uprising, collapse: no longer electoral
    CivilianControl,           // authoritarianism fell to <= .20 (the guard)
    InstitutionGone,           // the referenced Army entry no longer exists
    InvalidRecord,             // semantic validation failed (8.3)
}
```

**`leverage_after`** is the live `army_authority::current_leverage` read when the receipt is
written, and 0 when that reads `None`. For compliance receipts it is read after
`consolidate_transfer`. For `Escalated` it is read before `break_electoral`, and for
`GovernmentRemoved` it is read by the `regime_break` hook before that function writes
anything. In both cases that is **before** `army_seizure` sets leverage to 1.0. The post-break
1.0 is the world's `army_authority` record, not the receipt's.

### 8.2 Persistence and old saves

- **Fields absent.** An old save, or any save from a lens-off or takeover-off world, has no
  `authority_dispute` and no `authority_history` key. It loads with **no dispute, no history
  and no cooldown**, and nothing is inferred from leverage, loyalty or the date.
- **Later orders.** After loading, an order is possible only if every legality rule in section
  5 holds. Rule 10 holds only where a sourced `army_authority` was saved.
- **Dates.** They are stamped with `army_authority::model_date` (made `pub(crate)`): day 1 on
  the monthly clock, so the legacy day wrapper cannot change a monthly receipt; the actual day on
  the daily clock.
- **New types.** They are strict (`deny_unknown_fields`, typed enums). `GovState` keeps its
  present tolerant deserialization. An older build loading a newer save silently drops the two
  fields; forward compatibility is not promised.
- **Capped history.** `authority_history` is capped at 8 by dropping the oldest receipt on push.
  The cap is deterministic, and the cooldown reads only the newest receipt.

### 8.3 Validation and lapses (no stale gauge can survive its dispute)

**Where validation runs (fixed in this revision; both reviews' HIGH-1).** The first draft
validated only inside the dispute tick, which runs only in `government::tick`'s electoral
branch. Its `ElectoralRuleEnded` row could therefore never run. A record could then survive a
suspension, an uprising, a collapse or a drift to authoritarianism `>= .60`, and resume
accruing if the same party governed again after a reopening. That broke I10 and the
proposal's "an old gauge cannot manufacture a coup after the dispute ends".

The validation therefore runs as its own step, `civilian_authority::validate(w, id)`:

- **Placement.** It is called in `government::tick`'s per-nation loop **for every nation**,
  immediately before the `if is_electoral(w, id)` split (`:9505`), so it runs in the electoral
  and regime branches alike.
- **Switches.** It returns before reading anything when `!rules.ideology_blocs` or
  `!rules.ideology_takeover`. The takeover-off case is the frozen dispute below (Q22).
- **No record, no writes.** It returns at once when `authority_dispute` is `None`. A world with
  no record therefore has unchanged bytes and draws nothing (I1, I2).
- **Same tick, again.** The electoral-branch dispute tick (10.1) calls the same validation
  first. It is idempotent, so a change between the two points in the same tick that a row
  below reads is caught. The relevant writer between them is the scheduling seam
  (`schedule_first_elections`, reached at `:9512-9535`). It calls `form_government`, which can
  change the leader (row 3 reads that), and it sets `awaiting_first_election` and clears
  `elected`. Those last two are legality rule 5, which no row below reads yet (row (6),
  **Q23**); the firing check reads `!awaiting_first_election` regardless (10.3).

**What it does.** The pure `pending_lapse(w, id) -> Option<Resolution>` evaluates the rows below
**in table order**; the first row that holds gives the resolution. `close_lapsed(w, id)` then
pushes an `AuthorityReceipt` with that resolution and `closed_on = model_date(w)` of the tick
that closes it, and clears the record, deleting its `authority_pressure`. It writes **no other
effect, no pressure and no headline**.

| Order | Condition | Resolution |
| --- | --- | --- |
| 1 | Record non-finite or out of range, nation mismatch, or `institution.pillar != Army` | `InvalidRecord` |
| 2 | `!is_electoral` | `ElectoralRuleEnded` |
| 3 | `format!("party:{}", leader)` differs from `order.issuer` (the key `established_civilian_record` compares), or there is no leader | `IssuerReplaced` |
| 4 | `civilian_control_holds(w, id)` (authoritarianism `<= .20`) | `CivilianControl` |
| 5 | No Army entry exists at `ordinal` | `InstitutionGone` |
| (6) | Legality rule 5 or 6 fails while the state is still electoral (a ban clears `unrestricted_mandate`; interim status) | **Open: Q23.** Either a further lapse row, or a dispute acceptance that ignores rules 5-6. |

**Which exit closes a dispute, and how.** Every numbered row above was already in the first
draft; only where it runs has changed. The exits the reviews listed map onto it as follows.

| Exit (code at `39369f0`) | Calls `regime_break`? | Closed when | Resolution |
| --- | --- | --- | --- |
| Route 2 (`maybe_electoral_coup` → `break_electoral`, `:9006`) | yes | at once, by the `regime_break` hook | `GovernmentRemoved` |
| The annulment (`annul_election` → `break_electoral`, `:7252`) | yes | at once, by the hook | `GovernmentRemoved` |
| Stage-2 escalation (10.3) | yes, after closing | by itself, before the break | `Escalated` |
| `suspend_constitution` (`:7679`), whether the AI lever or a player's command, the issuer's own included | no | the same tick: commands and `ai_stratagems` run before `government` | `ElectoralRuleEnded` (authoritarianism becomes `>= .65`). Whether the issuer's own suspension may end its dispute with no consequence is **Q24**. |
| Authoritarianism drift to `>= .60` (`is_electoral`'s ceiling), for example the crackdown card's `+.06` (`stratagems.rs:381`) | no | the same tick (cards run before `government`) | `ElectoralRuleEnded` |
| `uprising` → `settle_uprising` (`:9065`, `:9077`; called from `politics.rs:249`, which runs after `government`) | no | the next tick | `ElectoralRuleEnded` if no longer electoral, else `IssuerReplaced` (the cabinet is cleared). The other caller, `armed_opposition_victory` (`:9073`, from `armed_security::tick` at the end of `government::tick`), cannot meet an open dispute: `armed_security::tick` drops its record once a state is electoral and past its first ballot, which an issuer must be (rule 5). |
| `generic_regime_collapse` (`:9149`; `politics.rs:252`, `:259`) | no | the next tick | the first row that holds |
| A ballot or formation changes the governing party (`hold_election`, after the dispute tick; the peace transition at `campaign_peace.rs:438`) | no | at the next validation step (the next tick for `hold_election`), and earlier for any command or AI action through the post-lapse view (4.4) | `IssuerReplaced` |
| Authoritarianism falls to `<= .20` (for example the peace transition's `-.25`) | no | at the next validation step | `CivilianControl` |
| The regime's own coup (`maybe_coup`, `:9204`), reachable only by a record frozen in a takeover-off world | yes | by the hook, unless the hook is gated (**Q22**) | `GovernmentRemoved` |

Between an exit and the closing tick, nothing can act on the stale record. Every refusal, plan,
arm and AI function uses the post-lapse view (4.4), and `ai_response` runs only for electoral
nations whose record passes `pending_lapse`.

**Routine succession.** A new officeholder from the same party, for example by mortality or a
term limit, keeps the dispute, because the issuer key is the party.

**The `regime_break` hook.** `regime_break` (`:9331`) gains one line before it writes anything:
if an open dispute exists, close it as `GovernmentRemoved`. Stage 2's escalation closes it as
`Escalated` before calling the break, so the hook then finds nothing. Without a dispute the hook
writes nothing, which keeps the byte identity in section 13.

**Acceptance must be total on a validated record.** Tick steps 3-5 (compliance, gap, firing) read
`dispute_acceptance(w, id, &record)`. That function must return `Some` for every record that
passes validation. The first draft read the order-time `acceptance`, which returns `None`
whenever legality rules 3-11 fail, so steps 3-5 were undefined once rule 5 or 6 failed
mid-dispute. Whichever answer Q23 takes, every condition under which `dispute_acceptance` is
`None` must be a validation row. A unit test asserts this totality (control 34).

**A dispute that cannot advance (Q22).** If a save with an open dispute is loaded into a world
whose `ideology_takeover` is off, the validation step and the dispute tick return before reading
it. The record is frozen: no accrual, no firing. The resolution commands are refused by the
switch sentence. The record stays in the save until the switch is on again. When it is, the
validation step runs first, so a frozen record whose issuer or electoral status has gone closes
before it can accrue. Three consequences of the proposed freeze are open in Q22:

- **The hook.** `maybe_coup` calls `regime_break` with the takeover switch off. An ungated hook
  can therefore close a "frozen" record as `GovernmentRemoved`, which contradicts the freeze.
- **The browser.** `/api/load` forces `ideology_takeover` off (`spheres-web/src/main.rs:7166`).
  A loaded dispute is therefore frozen in the browser.
- **Visibility.** The screen serves disputes only with both switches on, so a loaded dispute is
  also invisible there.

---

## 9. Resolutions and the decision window

### 9.1 On refusal

The arm opens `DisputeRecord { status: Refused { decide_by_month: month_index + 1 }, authority_pressure: 0.0, .. }`
and prints a headline. The government may then decide during the refusal month and the whole
following calendar month. At the first dispute tick with `month_index(w) > decide_by_month`, an
undecided dispute becomes `Enforced` (`StepKind::EnforcedByDefault`): **the lawful order stands
until withdrawn** (Q17). No pressure accrues while `Refused`.

### 9.2 The three resolutions

| Action | Legal when | Price | Effect (from `ResolutionPlan`) |
| --- | --- | --- | --- |
| **Negotiate** a narrower scope | Dispute open and `scope.narrower()` exists | 0 PC | Step the scope down one rung and re-test `dispute_acceptance` now. **If it complies:** close as `NegotiatedCompliance` and apply `consolidate_transfer(0.05 × s_narrower)`. **If it still refuses:** append a `Negotiated` step and return to `Refused` with a new decision window. As drafted, accrued `authority_pressure` is kept and does not accrue while `Refused`; it is deleted only when the dispute closes. Whether pressure accrued against the wider order should survive into the narrower one is **blocking Q11**. |
| **Withdraw** the order | Dispute open | 12 PC, `ALWAYS` | Close as `Withdrawn`. Leverage, loyalty and authoritarianism are unchanged. The cooldown starts at closure. Any accrued pressure is deleted. |
| **Enforce** the order | Dispute open and status `Refused` | 0 PC up front; .20 PC/month while enforced | Status becomes `Enforced { since: today }` with an `Enforced` step. In stage 2 pressure accrues as in section 10. Refused when already enforced: "The order is already being enforced." |

Refusal sentences for resolutions, in order:

1. The switch sentences of section 5.
2. "{N} has no unresolved order to settle." This is also served when the stored record would
   lapse, because resolutions read the post-lapse view (4.4).
3. "There is no narrower order than {phrase} to offer." (for Negotiate)
4. "The order is already being enforced." (for Enforce)

### 9.3 Headlines

Headlines are written so that none matches a pattern the census parses:

- `COUP IN `;
- `Revolution in `;
- ` votes: `;
- `The government of `;
- ` sets a date for its first free elections`;
- ` opens up.`;
- ` convenes a round table`;
- ` suspends its constitution`;
- ` declares a `…`programme.`;
- `dies in office.`

A test asserts this (section 14).

| Event | Headline |
| --- | --- |
| Accepted at issue | "{N} orders {phrase}; {institution} accepts." |
| Refused | "{N} orders {phrase}; {institution} refuses." |
| Negotiated | "{N} narrows its order to {phrase}; {institution} accepts." or "...; {institution} still refuses." |
| Withdrawn | "{N} withdraws its order on {phrase}." |
| Enforced (explicit) | "{N} enforces its order on {phrase}; {institution} still refuses." |
| Enforced by default | "{N}'s order on {phrase} stands; {institution} still refuses." |
| Compliance while open | "{N}'s order on {phrase} is carried out; {institution} gives way." |
| Escalation (stage 2) | the existing route-2 template, see 10.3; a census-compatible suffix naming the refused order is an option in **blocking Q13** |
| Lapses (8.3) | none: the event that caused the lapse has its own headline |

### 9.4 Compliance while open (drafted behaviour; blocking Q5)

**As drafted:** at every dispute tick, after validation and before accrual, the institution
complies if `dispute_acceptance(current scope) >= .35`, in either status. The dispute then
closes as `CompliedWhileOpen` and `consolidate_transfer(0.05 × s)` applies.

The first draft called this "the recorded, non-silent way a material or political recovery
ends a standoff". Because funding alone can trigger it, it conflicts with the geography review
and the 22 September record (7.4). This revision keeps it only as option (a) of Q5.

**No hysteresis (part of Q5).** `electoral_army_tick` walks loyalty before the dispute tick in
the same branch. An order refused at issue (from `ai_stratagems` or a player command, both before
`government`) can therefore close as `CompliedWhileOpen` in the same tick, producing a
refuse/carry-out headline pair. Whether a margin or a minimum duration is wanted is a design
choice.

---

## 10. Escalation (stage 2)

### 10.1 Where it runs

There are two call sites. The validation step, `civilian_authority::validate(w, id)`, runs for
every nation before the electoral/regime split (8.3). `civilian_authority::tick(w, id) -> bool`
is called in `government::tick`'s electoral branch immediately after route 2:

```rust
electoral_army_tick(w, id);
if maybe_electoral_coup(w, id) { continue; }
if crate::civilian_authority::tick(w, id) { continue; }   // new; fired => regime branch next tick
// fragile-cabinet branch and hold_election unchanged below
```

It returns `false` before reading anything else when `!rules.ideology_blocs`, when
`!rules.ideology_takeover`, or when `authority_dispute` is `None`. Its order of work is:

1. validation and lapses, re-run, idempotent (8.3; the first run is the pre-split step);
2. the decision window (9.1);
3. compliance (9.4, subject to Q5), reading `dispute_acceptance`;
4. upkeep and accrual;
5. the firing check.

Stage 1 implements steps 1-3 and the enforcement upkeep; stage 2 adds the accrual and the firing
check.

### 10.2 Accrual (only while an enforced dispute is unresolved)

```text
dt   = clock::month_fraction(w)
gap  = ELECTORAL_COUP_ARMY - dispute_acceptance(current scope)   (> 0, or 9.4 would have closed it)
if status == Enforced:
    political_capital = max(0, political_capital - AUTHORITY_ENFORCEMENT_UPKEEP * dt)
    authority_pressure = min(1.5, authority_pressure + 0.60 * gap * dt)
```

The accrual rules are:

- **Discontent.** There is no discontent condition. The cause is the recorded refusal of a
  lawful order, not a crisis.
- **Refused status.** There is no accrual while `Refused`.
- **No cooling.** There is no cooling branch. The gauge exists only inside the record, so every
  closure deletes it: "Resolution stops further accumulation immediately; an old gauge cannot
  manufacture a coup after the dispute ends."
- **Separate gauges.** The gauge never reads or writes `GovState.coup_pressure`, the route-2
  gauge.

**The gap and route 2's guard (blocking Q4).** The gap decomposes as

```text
gap = 0.35 - acceptance = (0.35 - eff_k) + 0.45 * leverage * s        where eff_k = loyalty_k - F_N
```

When an Army's effective loyalty is below .35, the first term is route 2's own material-hostility
term. As drafted, it accrues here at route 2's slope (.60) with **no** `D >= .25` condition.

Worked example, with held inputs: a Sao Tome-shaped polity at leverage .25 and effective loyalty
.30 ordering `BudgetOversight`. The gap is .05 + .0375 = .0875, so pressure grows .0525 a
month and reaches 1.0 at the 20th monthly accrual (1/.0525 ≈ 19.0), with no discontent at all,
where route 2 could never fire.
That is a route to a coup that bypasses route 2's live `D >= .25` guard, which the A1 hard
constraints protect. The first draft called this a "correlated input, not a double charge"
(10.5). The reviews found it to be the same grievance feeding a second gauge without the
guard. The options are in Q4.

> **Derived from seeds 0-11; must not inform the decision.** The illustration below uses the
> scout's seeds 0-11 minimum effective loyalties (7.5). It checks the accrual arithmetic only.

Illustration, with held inputs, of an enforced full-scope dispute, whoever issued it; it says
nothing about any temperament. Pakistan enforcing `CivilianCommand` at acceptance .287 has gap .063, accrues .0378 a month, and reaches
1.0 at the 27th monthly accrual (1/.0378 ≈ 26.5 months). Thailand has gap .075, accrues .0449 a
month, and reaches it at the 23rd (1/.0449 ≈ 22.3 months).
Either case resolves sooner if the Army's acceptance reaches .35, through compliance, or if the
government negotiates or withdraws.

### 10.3 Firing

The dispute fires in the same tick only when **all** of these hold:

| Condition | Source of the rule |
| --- | --- |
| `rules.ideology_takeover` | route 2 |
| status `Enforced` | this contract |
| `dispute_acceptance(current scope) < .35` (the refusal is still live) | mirrors route 2's live-condition rule (`:8976-8982`) |
| `electoral_coup_settled_months(w, id) >= 12` and `!awaiting_first_election` | route 2 (`:8965-8972`) |
| validation passed, i.e. the issuer still governs, the state is electoral, authoritarianism `> .20`, and the institution exists | 8.3 |
| `authority_pressure >= 1.0 / rules.crisis_intensity.max(0.1)` | route 2 |

Then:

1. Push an `AuthorityReceipt { resolution: Escalated, .. }` (its `leverage_after` read now, before
   the break; 8.1) and clear `authority_dispute`.
2. Call the **existing** `break_electoral(w, id, format!("COUP IN {}: {} removes the elected government.", id.name().to_uppercase(), receipt.order.institution_name))`,
   unchanged. It in turn calls `regime_break`, which:
   - lowers stability by 16 (floor 5) and output ×0.97;
   - sets authoritarianism to `max(auth + .25, .65)`, capped at `.98` (`break_electoral`), and the Nationalist colour;
   - keeps the cabinet dormant, and reseats institutions (Army entries .90, the rest .72);
   - clears `coup_pressure` and reseats political capital;
   - calls `army_seizure`, which sets leverage to 1.0;
   - runs succession and the foreign payoff.
3. Return `true`.

**This is not misattribution.** The event is a military institution removing an elected
government. That is the event class the headline names and the class A1 is defined to count, so
the census classifies it as `coup_el` with no census change. The headline claims what happened,
not why. The cause stays observable in three places:

- the `Escalated` receipt, with its order receipt, refusal date and steps;
- the refusal and enforcement headlines that preceded it;
- the government screen.

Contrast D4: a constitutional dismissal given this headline *would* be misattribution, which is
why D4 is not pursued.

**One known simplification.** `break_electoral` and `seat_office` take a `Pillar`, not an
ordinal. In a polity with two Army entries, the break reseats both Army entries at .90, and the
office description uses the first Army name. This is existing `regime_break` behaviour. No
electoral polity has two Army entries at 1990; Sudan is a regime. See Q20.

### 10.4 No double charge with the annulment or the programme veto

- **The dispute writes no loyalty.** The programme veto and annulment read loyalty and leverage.
  A refusal changes neither, so one refusal cannot feed the veto. Compliance *lowers* leverage;
  that is the intended effect, not a charge.
- **The annulment is separate.** It fires only inside `hold_election`, under its own conditions.
  If it fires while a dispute is open, `regime_break`'s hook closes the dispute as
  `GovernmentRemoved`, and escalation can never follow. If the dispute escalated earlier, the
  issuing government no longer exists to hold that election.
- **Algeria is structurally untouched.** Its first completed model ballot is the one annulled,
  and legality rule 5 needs a completed unrestricted ballot first. So no order can precede the
  staged annulment. The two chronology tests must still pass:
  `the_algeria_shaped_annulment_fires_under_its_conditions_and_not_under_the_court` (`:12813`)
  and `a_paid_army_seats_the_islamists_and_a_hostile_one_annuls_the_algerian_way` (`:14716`).
- **At most one break per government per tick.** Route 2 is evaluated first and `continue`s.
  Escalation `continue`s. `hold_election` comes later in the branch and is not reached after
  either.

### 10.5 Exact interaction with route 2

| | Route 2 | Dispute escalation |
| --- | --- | --- |
| Gauge | `GovState.coup_pressure` | `DisputeRecord.authority_pressure` |
| Accrues while | `eff < .35` **and** `D >= .25` | status `Enforced` **and** the refusal is live (`acceptance < .35`); no discontent condition (blocking Q4) |
| Rate | `0.30·(2·(0.35 − eff) + (D − 0.25))` | `0.60·(0.35 − acceptance)` |
| Cools | `−0.03·dt` | never; deleted on closure |
| Fires | settled ≥ 12, live conditions, gauge ≥ threshold | settled ≥ 12, live refusal, gauge ≥ threshold, auth > .20 |
| Break and headline | `break_electoral`, "removes the elected government" | identical |

- **Separate gauges.** The two gauges never add or transfer. Route-2 code and its arithmetic are
  unchanged; a test asserts route 2's `coup_pressure` is bit-identical with and without an open
  dispute.
- **Shared inputs (blocking Q4).** Both read loyalty, so a crisis that lowers loyalty speeds
  both. The first draft called that "a correlated input, not a double charge, because only one
  break can occur". The reviews disagree. When effective loyalty is below .35, the dispute gap
  contains route 2's material term, `0.35 - eff`, at route 2's slope, and the dispute accrues it
  without route 2's `D >= .25` guard (10.2). The claim is withdrawn pending Q4.
- **When both are ready in the same tick.** Route 2 fires. The dispute closes as
  `GovernmentRemoved`. A1 sees one coup either way.
- **After either break.** Leverage is 1.0 and `coup_pressure` is 0. A later civilian government
  may issue a new order after reopening and a new six-month record. Repeat risk is listed in
  section 16.

---

## 11. AI objective and decision rule

### 11.1 Objective

The AI's objective is to **reduce the Army's live executive-removal leverage** through lawful
orders it can obtain. That reduction is what lowers its exposure to the two channels that read
leverage (7.3). The AI never pursues a dispute, a refusal or a coup for its own sake. It uses
the same commands, prices, refusals and plans as the player, and it never resolves a player's
dispute.

### 11.2 The temperaments (no default proposed; blocking Q1)

The first draft proposed "F: Floor" as the default. Both reviews found that F, and A, appear to
recreate forbidden triggers by combination (11.3). **That proposal is withdrawn.** The three
temperaments are specified below so that whichever one the design authority chooses can be
implemented exactly. A fourth option is for the AI to issue no orders at all, leaving disputes
to the player; under it, `ai_order` and `ai_response` are never added.

**Issuance (pure), common to P, F and A.** `civilian_authority::ai_order(w, id) -> Option<Command>`
returns `None` before any read when either switch is off, and `None` for the player's nation.
It reads legality on the post-lapse view (4.4). It returns a command only when **all** of these
hold:

- the nation is electoral and the order is legal for some Army institution (the first legal
  ordinal in stored order);
- `D < .25` (C14): a government does not pick a fight with its army during an economic crisis;
- `government_seats() >= .50` (C13);
- `political_capital >= AUTHORITY_ORDER_PC + 20` (C12; Q9).

**Response (pure, deterministic, no RNG), common machinery.** `civilian_authority::ai_response(w, id)`
runs from `ai_government` (`:9423`) at `clock::month_end(w)`. It runs for every non-player
electoral nation whose open record passes `pending_lapse`, before that function's existing
`if is_electoral(w, id) { continue; }`. The command is applied through `apply_command` when
`affordable`; the arm re-validates first (4.4).

| Temperament | Issue (scope asked for) | Respond: `Refused` | Respond: `Enforced` |
| --- | --- | --- | --- |
| **P: Prudent** | The widest scope whose acceptance is `>= .35`; nothing when none would be accepted. `ai_order` and the arm read the same state in the same tick, so a P order is accepted at issue and **P never opens an AI dispute**. | Withdraw (a fallback; unreachable for AI-issued orders) | Withdraw (fallback) |
| **F: Floor** | The widest accepted scope; otherwise `BudgetOversight` (an order its own plan says will be refused) | If a narrower scope exists and `acceptance(BudgetOversight) >= .35`: **Negotiate**. Else if `government_seats() >= .50`: **Enforce**. Else: **Withdraw**. | If a narrower scope exists and `acceptance(BudgetOversight) >= .35`: **Negotiate**. Else if `government_seats() < .50`: **Withdraw**. Else: no action. |
| **A: Assertive** | `CivilianCommand` whenever the common preconditions hold | Enforce while it holds a majority; withdraw without one; never negotiate | Withdraw without a majority; else no action |

The first draft listed an "expected direction" for each temperament, including its likely
effect on A1. Those statements were derived from seeds 0-11 observations and invited selection
by A1 effect, so they were **removed** in this revision (R12).

### 11.3 Consistency with 11.1 and the A1 hard constraints (blocking Q1)

The choice must be made on design grounds **before** measurement and recorded in the
predeclared plan. It is measured once.

- **Not by A1 effect.** Selecting among the temperaments by their A1 effect is forbidden.
- **Not on seeds 0-11.** The decision record must not cite seeds 0-11 material: the section
  1.2 tables, the 7.5 and 10.2 illustrations, or any per-country outcome.

The analysis below uses only the formulas and the binding texts. Issuance and response are
deterministic functions of live state, so each temperament, taken as a whole, acts as a trigger
rule. It must be judged as one.

- **P: Prudent.**
  - **Consistent.** P issues only orders its own plan says will be accepted, so every AI order
    obtains a leverage reduction. That is consistent with 11.1 ("through lawful orders it can
    obtain"; "never pursues a dispute, a refusal or a coup for its own sake") and with the
    proposal's "positive governance objective".
  - **No AI trigger.** No AI-originated dispute, refusal or escalation arises, so no AI-driven
    trigger exists.
  - **Still open under P:** card precedence (Q3), the draw (Q2), the compliance ratchet (Q6)
    and the political-capital drain (Q9).
- **F: Floor.**
  - **Conflicts with 11.1 as written.** Its second branch issues `BudgetOversight` when its own
    plan says `complies: false`. That order obtains nothing, which conflicts with 11.1 and with
    the proposal ("must not manufacture disputes").
  - **Recreates route 2 without its guard.** Taken as a whole, F's AI disputes arise exactly
    when `(loyalty_k - F_N) < .35 + .15 * leverage`, with `D < .25` and a majority, and
    escalation has no discontent term. For AI-governed states that is route 2 with its
    `D >= .25` guard removed and its `.35` line raised by `.15 * leverage`. That is the
    forbidden "delete the live guards" or "lower a ceiling", reached by combination even though
    route-2 code is untouched (engineering review).
  - **Close to a lottery.** Under the 0.02 draw, the draw becomes a randomly timed start of a
    deterministic escalation countdown, close to the forbidden background lottery (design
    review).
- **A: Assertive.**
  - **A leverage trigger.** The Army refuses the full order exactly when
    `eff_k < .35 + .45 * leverage`. For a given loyalty, whether the
    order → refusal → escalation chain starts is then set by sourced leverage, with no
    discontent term: "high sourced leverage plus calm" acts as the trigger.
  - **Excluded twice.** The A1 hard constraints exclude "using high V-Dem leverage alone … as a
    trigger". The proposal says "A refusal must not follow automatically from a nonzero
    historical leverage score".

**Conclusion, without choosing.** On this contract's own text, **P is the only temperament
consistent with 11.1 as written and with the A1 hard constraints**. F or A could be chosen only
if the design authority first rewrites and approves 11.1, and rules explicitly that the chosen
combination is neither a forbidden leverage-alone trigger nor a bypass of route 2's guard.
This revision proposes no default. The no-AI-orders option is also consistent with 11.1, and it
removes the AI's draw, card and political-capital effects (Q2, Q3, Q9) and the AI ratchet (Q6).

### 11.4 The draw site, and the RNG-stream shift risk to A2

**Issuance rides the existing single draw.** `ai_lever` (`government.rs:8254`) gains one
branch for electoral states, after the suspension and ban checks and before its
`if electoral { return None; }`:

```rust
if electoral {
    if let Some(order) = crate::civilian_authority::ai_order(w, id) { return Some(order); }
    return None;
}
```

`stratagems::ai_stratagems` (`stratagems.rs:529-583`) is unchanged. As drafted, a lever still
takes precedence over a card. The month still holds one decision, and the order executes only
when `w.rng.chance(clock::chance(w, 0.02))` passes (`:577`). The government and
`civilian_authority` modules draw no RNG. The response rule is deterministic and draws nothing.

**Card displacement (blocking Q3).** In `ai_stratagems` the lever wins over the card
(`stratagems.rs:567-575`) **before** the draw, and the draw only gates execution. So in every
tick `ai_order` returns `Some`, the order is the actor's only candidate: a passing draw
executes the order, never a card. That covers every eligible month (an order legal, calm,
majority, PC `>= 45`, outside the cooldown; under P, also some scope that would be accepted)
under any temperament that issues, P included. A qualifying electoral AI therefore loses card play for
most of each wait for the draw, about 50 months on average per order at 0.02 a month, and again
after each cooldown. The cards lost include liberalisation (which prints a census "opens up."
headline), `professional_army`, austerity and debt restructuring.

The code's own rationale for lever-first, "its conditions are the narrower crisis"
(`stratagems.rs:553-561`), does not fit an order issued in calm times. The first draft treated
this only as an RNG risk. It is an AI-behaviour decision, and the options are in Q3. Under the
"only when no card is chosen" option, a control asserts that a chosen card is unchanged
(control 35).

**The stream-shift risk.** With the lens on, an AI actor without a lever consumes **no** draw
today in any tick where it holds under 55 PC, has no available card, or cannot afford any card
while keeping the 20 PC reserve. When `ai_order` returns `Some` in such a tick, that actor now
consumes a draw. The single SplitMix64 stream then shifts for every later random event in the
world:

- uprising draws, covert operations, mortality, and so on;
- a card displaced by the lever in a month it would have been played.

A2 depends on many such draws: 8 of 12 against a bar of more than 6. The iteration-22
candidate, reassessment 27 and the urgent response each went to 6 of 12. **A stream shift alone
can move A2.** The stop rule rejects on that regardless of cause. It may not be rescued by
seed, country, coefficient or timing changes.

Any mechanism that changes state shifts later random outcomes anyway, so the risk cannot be
removed, only made smaller. One alternative, blocking question Q2, issues deterministically at the
January review in `ai_government` with no draw. That trades the immediate shift for a
seed-invariant political calendar. Iron rule 3 says events should "*usually* happen across
seeds, not always". It must be decided before measurement.

---

## 12. Player UI

The screen computes nothing in JavaScript. Every number and sentence comes from the
simulation's plan functions.

- **Availability.** Everything below is served only when **both** switches are on and the
  nation is electoral. Browser play today has `ideology_takeover` off, so the current government
  payload and every existing web test's action list stay byte-identical (Q19). A save carrying a
  dispute loaded through `/api/load` is frozen and not shown (Q22).
- **Actions** (`government_json`, `spheres-web/src/main.rs:1351`). For each Army institution
  (`ordinal`) and each scope there is one `action_json`:
  - kind `civilian_order`;
  - label "Order {phrase} over {institution}";
  - blurb "a lawful order the institution may refuse";
  - command `{"kind":"assert_civilian_authority","institution":{"pillar":"army","ordinal":0},"scope":"senior_appointments"}`;
  - price, `affordable`, `refusal` and `effects`, from `price_of`, `affordable`, `refusal_of`
    and `civilian_authority::effects`.

  `category()` maps `civilian_order` and `authority_dispute` to `"institutions"`. The effects
  lines render `OrderPlan`, for example:
  1. "{institution}: loyalty 0.65, Nationalist backing 0.00, executive leverage 0.80 (sourced
     1989 assessment, campaign value)."
  2. "Acceptance 0.65 − 0.45 × 0.80 × 0.67 = 0.41 against the 0.35 line: {institution} will
     accept."
  3. "On acceptance: executive leverage 0.80 → 0.72; the confidence and programme-veto channels
     read the lower value." Or, when refused: "On refusal: a dispute is recorded and you choose
     to negotiate, withdraw (12 PC) or enforce."
- **The alert** (`government_view::enrich`, `:92`). While a dispute is open, the briefing gains
  one attention item:
  - `id` `"authority_dispute"`, `tone` `"attention"`;
  - title "{institution} refuses your order";
  - `detail` from a new sim function, `civilian_authority::dispute_summary(w, id)`. It names the
    order (phrase, issue date), the institution and the unresolved issue. It also gives the
    status: "awaiting your decision until the end of {month}; if you do not decide, the order
    stands", or "enforced since {date}; authority pressure {p:.2} of {t:.2}, rising {r:.3} a
    month; .20 PC a month".
  - `action_indices`, the three resolution actions. The existing `action_index` field is kept
    for every other item.

  `government-ui.js` (`attention`, `:73`) renders one review button per index, as presentation
  only.
- **Three quoted actions from one server plan.** Kind `authority_dispute`:
  - "Negotiate: narrow to {phrase}";
  - "Withdraw the order";
  - "Enforce the order".

  The commands are `{"kind":"resolve_authority_dispute","action":"negotiate"|"withdraw"|"enforce"}`.
  Each comes from `resolution_plan` through `action_json`, with its refusal served where it does
  not apply: "There is no narrower order...", "The order is already being enforced.". Enforce's
  effects quote the accrual rate, the threshold, the upkeep, and that acceptance at .35 ends the
  dispute in compliance.
- **Review** (`government_view::preview`, `:234`). This is the existing clone-and-apply review.
  It adds change rows:
  - `authority_dispute`, the status label before and after, from
    `civilian_authority::status_label`;
  - `army_leverage`, "Army executive leverage", from `army_authority::current_leverage` before
    and after;
  - `authority_pressure`.

  It adds one warning for both kinds: "Acceptance is the simulation's own reading. Enforcement
  accrues authority pressure while the institution refuses; at the threshold it removes the
  government. This is not a probability forecast."
- **Parsing** (`parse_command`, `:6516`). The two kinds are accepted only for the player's own
  nation, with a valid ordinal and scope string.
- **Foreign governments.** AI disputes appear on that nation's page as a read-only attention
  item, and in headlines. The existing foreign-view disabling of actions applies.
- **The takeover watch (blocking Q13).** As drafted, no fifth road or gauge is added, because it
  changes the watch payload in every lens-on world. The first draft did not state the cost of
  that choice. Without a fifth road, the watch's coup road reads far from its trigger right
  before a dispute coup, because route 2's gauge is not the one rising. The map hatching and the
  header chip also stay quiet. A player could therefore be surprised by an escalation that the
  dispute alert, but not the watch, was tracking.
- **The escalation headline (blocking Q13).** The census reads the text after
  `COUP IN <NATION>:` with `contains("removes the elected government")`
  (`bloc_census.rs:377-398`). A suffix clause naming the refused order would therefore still be
  counted the same way by A1, A6 and A9, and it would make the cause legible in the headline
  itself. The first draft wrongly implied that the exact template was required. Whether to add
  the suffix is part of Q13; control 22 must cover whichever form is chosen.

---

## 13. Invariants

| ID | Invariant | How it holds |
| --- | --- | --- |
| I1 | Lens-off byte identity | Every new entry point returns before reading state when `!ideology_blocs`. The new fields are skipped when empty. The `Command` variants are never issued. `the_bloc_layer_is_inert_at_1990`, the golden digests and the determinism and save-roundtrip tests are unchanged. |
| I2 | Takeover-off byte identity (lens on) | Same as I1 for `!ideology_takeover`: `ai_order` is `None`, so `ai_lever` is unchanged and no draw is added. The pre-split validation step and the tick return at once, and the web payload is unchanged. `every_road_reads_closed_while_takeover_is_off` is unchanged. With both switches on and no record anywhere, the validation step writes nothing (control 31). |
| I3 | Daily equals monthly | Accrual and upkeep use `month_fraction`. Windows and cooldowns are in `clock::month_index`. Dates are stamped with `model_date`. The AI response runs at `month_end`. Under held inputs the same calendar month fires on both clocks, and a month's accrued pressure agrees to 1e-9. |
| I4 | No RNG in `government` or `civilian_authority` | `w.rng` is unchanged across `tick`, the arms, `ai_order` and `ai_response` (asserted) |
| I5 | At most one draw per AI actor per tick, at the existing site | No new draw site exists. The draw is consumed only where `ai_lever` returns `Some` (11.4). |
| I6 | Same-type institutions distinct | `InstitutionRef.ordinal`; acceptance reads that entry; there is at most one open dispute per nation |
| I7 | Old saves load with no dispute | serde defaults; rule 10; 8.2 |
| I8 | One plan: card equals world | Effects, refusal and arm all come from one plan (bit-equality test) |
| I9 | At most one break per government per tick | Route 2, then escalation, each `continue`; the `regime_break` hook closes a dispute |
| I10 | No gauge outlives its dispute | The pressure lives inside the record and is deleted on every closure. Validation runs for every nation holding a record, before the electoral/regime split, so every exit from electoral rule closes the record (8.3; controls 27-30). |
| I11 | Civilian-control guard (scope pending blocking Q7) | No order, accrual or firing at authoritarianism `<= .20`; an open dispute lapses `CivilianControl` |
| I12 | Census unchanged | `bloc_census.rs` byte-identical. The new headlines avoid every parsed pattern; escalation uses the route-2 template, with or without a census-compatible suffix (Q13). |
| I13 | No special cases | No country, date or seed is named in code or data. Legality and the AI read only live, sourced state. |
| I14 | Save, load and replay continuity mid-dispute | `save(load(save(w))) == save(w)`. A resumed run's hash equals the continuous run's. The same dated commands give the same world. |
| I15 | Nothing acts on a lapsed record | Every refusal, plan, arm, effects function, `ai_order` and `ai_response` reads the post-lapse view, and the arms close a lapsing record first (4.4; control 32) |
| I16 | Dispute acceptance is total | `dispute_acceptance` is `Some` for every record that passes validation; every `None` case is a validation row (8.3; control 34) |

---

## 14. Test plan

The focused suites live in `spheres-sim/src/civilian_authority_tests.rs`. Web tests go beside
the existing government-screen tests in `spheres-web`.

### 14.1 Red first (recorded before each stage's change, on the unmodified runtime)

**Stage 1 red.** The test is `a_lawful_civilian_order_and_its_refusal_are_represented`, a
`spheres-web` test. It uses only stringly typed entry points that exist today (`parse_command`,
`government_json`, `preview`), so it **compiles and fails on `39369f0`** (or on the post-D2
tip), not merely fails to build. The fixture is Pakistan-shaped, built with authored and
disclosed state edits:

- lens and takeover on, with Pakistan as the player (`preview` serves only the player's
  government);
- Pakistan electoral, elected, `unrestricted_mandate`, with a 7-month record;
- sourced leverage .8, Army loyalty .65, `D < .25`, PC 60.

It asserts, in order:

1. `parse_command` of the full-scope order is `Some`. This fails today: `None`.
2. The order is served with a null refusal and price 25.
3. Its review shows `authority_dispute`: "none" → "refused".
4. Applying it leaves `authority_dispute` present in the save JSON.

Record the failing output, the binary hash and the toolchain.

**Stage 2 red.** The test is `an_enforced_refusal_accrues_authority_pressure_and_removes_the_government`,
run on the stage-1 runtime. With held inputs and an enforced full-scope dispute, it asserts:

- after one month `authority_pressure == 0.60 × gap` (to 1e-12);
- the escalation headline appears in the month computed from the formula.

It fails on stage 1 because there is no accrual.

### 14.2 Controls

"Twin" means identical provisioning and macro conditions.

Where a control tests behaviour that is still an open question, it is written for the drafted
option and changes with the answer. This covers controls 5 (Q11), 6 (Q9, Q10, Q21), 7 (Q17),
8 (Q5), 10 (Q7), 21 (Q13), 24-25 (Q1), 27(b) (Q24), 32(c) (Q10), 34 (Q23) and 35 (Q3), and
the prices and amounts quoted in others (Q6, Q9). Writing a control for the drafted option does not choose it.

| # | Control | Expected |
| --- | --- | --- |
| 1 | No order (twin) | No dispute, no receipt, identical save bytes |
| 2 | Refused order | `DisputeRecord` with every `OrderReceipt` field; status `Refused`; pressure 0; the refusal headline |
| 3 | Accepted at issue | No dispute; `Complied` receipt; leverage −0.125·s exactly; loyalty and authoritarianism untouched |
| 4 | Negotiated and accepted | `NegotiatedCompliance`; leverage reduced by the narrower amount only |
| 5 | Negotiated, still refused | Back to `Refused` with a new window; pressure kept and not accruing |
| 6 | Withdrawn | Closed; 12 PC charged to the point of bankruptcy (PC 5 gives 0); leverage unchanged; cooldown blocks a new order for 12 months, then allows it |
| 7 | Window lapses | `EnforcedByDefault` at the first tick after `decide_by_month` |
| 8 | Compliance while enforced (funding raises loyalty) | `CompliedWhileOpen`; the gauge is deleted; it cannot fire on the next tick even with pressure set to 1.49 just before |
| 9 | Missing Army; regime; interim (`awaiting_first_election`); restricted mandate; record under 6 months | Each refused with its sentence |
| 10 | Civilian-control guard: Uruguay-shaped (.12) and PNG-shaped (.15) fixtures with sourced leverage; Brazil-shaped at exactly .20 | Refused. An open dispute whose authoritarianism is set to .20 lapses as `CivilianControl` with no accrual. |
| 11 | Unknown leverage: an old save with `army_authority` removed | Refused; no dispute is inferred; the save loads with none |
| 12 | Routine succession: same-party officeholder change | Dispute kept. A ballot changing party gives `IssuerReplaced`, with no accrual or firing afterwards. |
| 13 | Strong lawful compliance: loyalty .85, any leverage | Every scope accepted |
| 14 | Same-type institutions: an electoral Sudan-shaped fixture with two Army entries | Ordinal 1's loyalty is read and its name recorded. An order to ordinal 0 is refused while ordinal 1's dispute is open. Walks stay per institution. |
| 15 | Daily against monthly | The same firing calendar month; each month's accrual agrees to within 1e-9; receipts dated day 1 on the monthly clock |
| 16 | Save, load and replay mid-dispute (Refused and Enforced) | Continuity hashes equal; the same dated commands give the same world |
| 17 | No RNG | `w.rng` unchanged across `tick`, both arms, `ai_order` and `ai_response`. Inactive path: on a fixture where no order is legal anywhere (switches on, every `army_authority` removed), the RNG state after 12 monthly ticks equals the value recorded from the base build on the same fixture and pinned in the test. |
| 18 | Lens off; takeover off | Refusal sentences 1 and 2. No fields serialized. `ai_lever` unchanged. The two named inertness tests pass unmodified. |
| 19 | Annulment while a dispute is open | Exactly one `COUP IN` headline (the annulment's); the dispute closes `GovernmentRemoved`; no later escalation |
| 20 | Route 2 interaction | `coup_pressure` bit-identical with and without an open dispute. When both are ready, route 2 fires and the dispute closes `GovernmentRemoved`. |
| 21 | Escalation attribution | The headline equals the route-2 template, with the recorded institution name. An `Escalated` receipt exists. The leverage after the break is 1.0. |
| 22 | Headline non-collision | No new headline matches any census pattern listed in 9.3 |
| 23 | Plan parity | `order_effects` and `resolution_effects` numbers equal the plan fields. The preview's after-state equals `apply_command`'s, bit for bit. |
| 24 | AI issuance | Never when `D >= .25`, without a majority, below the reserve, inside the cooldown, or for the player. Scope choice per the temperament chosen in Q1 (11.2). |
| 25 | AI response | The chosen temperament's row of the 11.2 table, row by row. Never acts on the player's dispute. Always deterministic. |
| 26 | Programme veto unchanged | `army_programme_veto_loyalty` bit-identical through the shared helper; both Algeria chronology tests pass |
| 27 | Suspension mid-dispute | With an open `Enforced` dispute and pressure set to 1.49: suspend the constitution, (a) through the AI lever and (b) as the issuing government's own player command. Expect an `ElectoralRuleEnded` receipt in the same tick, the pressure deleted, and no escalation headline. Then reopen the polity with the same party leading, past its first ballot and 12 settled months: no dispute, no accrual, no firing. |
| 28 | Uprising or armed victory mid-dispute | `uprising` → `settle_uprising` with an open dispute: the record closes at the next tick as `ElectoralRuleEnded`, or as `IssuerReplaced` if the state stays electoral. Nothing accrues or fires afterwards. |
| 29 | Collapse mid-dispute | `generic_regime_collapse` with an open dispute: the record closes at the next tick with the first matching 8.3 row, and nothing accrues or fires afterwards |
| 30 | Drift out of electoral rule | Authoritarianism raised to `>= .60` by the crackdown card (`stratagems.rs:381`) with an open dispute: `ElectoralRuleEnded` the same tick. Authoritarianism lowered to `<= .20` with the same party still leading: `CivilianControl`. (The peace transition also holds an election; if that changes the governing party, row 3, `IssuerReplaced`, applies first.) |
| 31 | Validation step is inert without a record | Both switches on, a 24-month whole-world run with every `army_authority` removed (no order possible): save bytes and RNG state equal the base build's, pinned. With either switch off the step reads nothing (I1, I2). |
| 32 | Stale issuer within one tick | A ballot in `hold_election` changes the governing party after the dispute tick. (a) The month-end `ai_response` takes no action on the old dispute. (b) The new government's `Negotiate` is refused with "{N} has no unresolved order to settle." (c) Its own order is not refused by rule 12; the arm first closes the old record as `IssuerReplaced`, and rule 13 applies as Q10 decides. (d) `NegotiatedCompliance` is never credited to the new government for the old order. |
| 33 | Ordinal and name correspondence | For every 1990 polity, and after `seat_spec_pillars`, `regime_break`'s reseat and the `needs_pillars` seed: the k-th stored entry of a kind maps to the k-th spec entry of that kind wherever both exist (4.2). `spheres-web`'s government payload is byte-identical after the web reader is replaced. |
| 34 | Dispute acceptance totality | For records satisfying every validation row, including the cases Q23 decides (a ban clearing `unrestricted_mandate`; interim status), `dispute_acceptance` is `Some`, and steps 3-5 never read `None` |
| 35 | Card precedence (form depends on Q3) | Under the chosen precedence: if orders yield to cards, an actor with an affordable card plays the same card, with the same draw sequence, as on the base build; if orders keep precedence, the displacement is recorded in diagnostic 4 |
| 36 | Receipt leverage | `leverage_after` on `Escalated` and `GovernmentRemoved` receipts equals the pre-break leverage, while the world's leverage after the break is 1.0 (8.1) |

### 14.3 Existing suites to run unchanged

- `cargo test --locked --release -p spheres-sim --lib government`, plus `stratagems`, `blocs`
  and `army_authority`.
- The two Algeria chronology tests.
- `the_ai_takes_each_lever_only_under_its_thresholds`.
- D2's six `civilian_guard` tests (`government_civilian_guard_tests.rs`), unmodified, since stage 1
  factors the guard's comparison into `civilian_control_holds` (3.1).
- The determinism and save-roundtrip tests.
- **`the_daily_clock_preserves_the_political_arm_on_world`** (`lib.rs:2183`). This is the existing
  whole-world daily-equals-monthly test with both switches on. Under any temperament that
  issues, it will now see AI orders and, under F or A, AI disputes. It must pass unmodified.
  Any change to it is a predeclared edited test.
- The test-only A1 observer's `selected_lever` field (`government_a1_observer.rs:80`) reads
  `ai_lever`, so it will now show civilian orders. Any diagnostic that compares observer output
  with an earlier log must expect that difference, and the plan must say so.
- The `spheres-web` government-screen tests.
- `node tools/ui/run-unit.cjs`.

---

## 15. Predeclared measurement plan

**Before building** (step 0), add a new S27 preparation packet, for example
`docs/campaign-certification/S27/preparation/a1-dispute-<date>/`. Its `plan.json` follows
[`urgent-plan.json`](../campaign-certification/S27/preparation/political-repairs-20260930/urgent-plan.json)
and records:

- the base, which is the post-D2 commit;
- the kind;
- the hypothesis: a recorded refusal of a lawful order is a missing cause;
- the change: the design authority's answers to every question in 17.1 and 17.2, **including
  the AI temperament (Q1)**. The plan may not be written while any 17.1 question is unanswered.
  The temperament decision is recorded with its design reasons only, citing no seeds 0-11
  material (11.3);
- the counterexamples (14.2);
- the validation;
- the stop rule.

**No predicted A1 value is written.** Pin the toolchain and use a private `CARGO_TARGET_DIR`
(CONTRIBUTING:39). Do not create worktrees in the constrained container.

1. **Red-first evidence** for each stage, as in 14.1.
2. **Focused suites** (14.2, 14.3) green.
3. **The unchanged gate, on ubuntu-latest and windows-latest** (CI `political-calibration`,
   `.github/workflows/verify.yml:216-231`), plus one local run:

   ```sh
   cargo test --locked --release -p spheres-sim --test bloc_census -- --include-ignored --skip bloc_census --nocapture
   ```

   - **Pass:** exit 0, 11 of 11, and tables byte-identical across the two operating systems.
   - **Record:** A1 count median and every per-seed exact top-three fraction; A2 seeds and the
     Algeria counts; A3-A10; attribution.
   - **Diff:** against the post-D2 baseline. **D2's local Linux gate run printed every value
     identical to `39369f0`** (3.1), so the post-D2 baseline equals the pre-D2 baseline: A1 7.5
     and .571429, failing, with per-seed fractions as in 1.2; A2 8/12 (Algeria Islamist 0/12,
     annulled 12/12); A3 0/20; A4 16; A5 12/12; A6 0 in 12/12; A7 8.462; A8 40/40; A9 168/168;
     A10 .335; attribution passes. D2's Windows and CI runs are still pending. *(Update, later on 4 Oct: CI run 37178241397 on `e857e8ac` reproduced the identical A1 seed table on ubuntu and windows, and every other CI result passed; see section 9 of the D2 packet README.)* If they, or D2's
     integration, show any different value, that value is the baseline and Q14 applies.
   - **Time:** about 100 s locally and 93-97 s in CI. The build takes about 2.5 min locally
     (`-j3`) and 1.5 min in CI.
4. **Same-cohort diagnostics** (seeds 0-11, measurement only): the census scan
   (`SPHERES_CENSUS_SEEDS=12 SPHERES_CENSUS_MONTHS=252 SPHERES_CENSUS_SEED_START=0 ... --ignored --exact bloc_census --nocapture`),
   plus a read-only headline harness. It counts, per seed and country:
   - orders, compliances, refusals, negotiations, withdrawals and enforcements;
   - escalations, first dates, and repeats after escalation.

   Report these as observations, never as tuning input.
5. **Engineering** (CONTRIBUTING:43-46):
   - `cargo test --locked --release --workspace --no-fail-fast -- --skip tests::the_resource_pass_stays_under_budget`
     (about 20-25 min);
   - the isolated `tests::the_resource_pass_stays_under_budget` run, alone and with no other
     heavy CPU load;
   - `node tools/ui/run-unit.cjs`;
   - `cargo build --locked --release -p spheres-web`.
6. **Only if every original gate and attribution pass on both operating systems:** optionally,
   **one** predeclared run of the fixed development cohort 0-199
   (`SPHERES_CENSUS_SEEDS=200`, about 200 s). **Never the 1000-series. Never append seeds.**
7. **Stop rule.**
   - **Any regression in an original gate, or a failing Algeria chronology test:** reject,
     restore the base, and retain every artifact. There is no compensating change to a gate,
     country, coefficient, scope, price, timing, temperament or cohort.
   - **A1 still fails while every other gate passes:** report a failure, and A1 stays open.
     Whether the mechanic is kept without A1 credit is a design-authority decision (Q25).
   - **The post-D2 baseline itself fails a gate other than A1:** the "every other gate passes"
     branch and step 6's condition are then ill-defined. Q14 must be answered before the plan
     is written.
8. **Records.** Logs, binary hashes and every failure go into the new packet. Reviewers are
   labelled honestly. A passing A1 would not by itself close S27, G5 or CP1.

---

## 16. Regression risk register

> **Derived from seeds 0-11; must not inform the decision.** The margins, country lists and
> recomputed values in this register come from seeds 0-11 observations at `39369f0`. They say
> what to watch in the predeclared run. They must not inform the temperament (Q1), any other
> 17.1 or 17.2 answer, or any coefficient. The first draft's temperament-specific A1
> expectations were removed from this register (R12).

| Risk | Current margin | Mechanism | What shows it | Allowed response |
| --- | --- | --- | --- | --- |
| **A2**: Islamist takeover without a ballot by 2000 in more than 6 and at most 10 of 12; Algeria Islamist in under 6 and annulled in at least 2 | 8/12 (fragile); Algeria 0/12 and 12/12 | The single RNG stream shifts from the first new draw (11.4), and cards are displaced by the lever. Long-run leverage decline also changes confidence penalties and AI funding paths. Three rejected candidates reached 6 of 12. | Gate A2, and the Algeria counts | Stop rule. No seed, timing or country rescue. Q2 and Q3 must be decided before measurement. |
| **A4**: opened 1990 regimes by end-1996 | Median 16 against a floor of 15 (margin 1) | An escalation before end-1996 in an opened 1990 regime: Ghana 1.0, Gabon .833, Cameroon .714, CAR .6, Lesotho .75. It costs about one per affected seed; two such countries fail A4. | Gate A4; first dates in diagnostic 4 | Stop rule |
| **A6**: no route event in a 1990 democracy over 420 months | 0 in 12 of 12 | Tripwires: Uruguay (.12, leverage .6) and PNG (.15, .5), blocked only by the guard. The guard reads *current* authoritarianism while A6's set is defined at 1990. A 1990 democracy pushed above .20 later, for example by the pre-arm collapse's random shift, becomes eligible. There is one such collapse per seed today; SPEC records Argentina's in 59 of 60 seeds on an earlier tree. Route 2 already has this exposure. | Gate A6 at 420 months | Stop rule. A 1990-democracy exception would be a forbidden country special case. |
| **A10**: discontent lead | .335 (seed medians .320-.358) | Escalations in quiet countries carry leads near zero (Pakistan −.014, Thailand −.023). The scout recomputed .319, .313, .303 and .295 for +1 to +4 such takeovers per seed, with the weakest seed at .246 at +4. Takeovers count only when the ruling bloc changes. | Gate A10 | Stop rule |
| **A1 repeats** | Core repeats today (13 of 87) all follow a seizure that set leverage to 1.0 | After an escalation, leverage is 1.0. A reopened civilian government may order again and be refused again. Where effective loyalty is below .35, an enforced dispute accrues route 2's material term without its discontent guard (10.2, Q4). | Diagnostic 4, repeats | Report only |
| **A1 count** | 7.5 within 4..=14 | Compliance lowers leverage; escalations add removals. No direction is recorded (1.3). | Gate A1 | Report only |
| **AI behaviour: card displacement** | n/a | Under lever-first precedence, an eligible electoral AI plays no card while an order is pending its draw (11.4, Q3). Liberalisation's "opens up." headline is a census pattern. | Diagnostic 4; control 35 | Decided by Q3 before measurement |
| **AI behaviour: political-capital drain** | n/a | Order and enforcement costs compete with suspension, bans, cards and `SecurePillar` (6, Q9) | Gates A2, A4, A7 | Decided by Q9 before measurement |
| **World-wide leverage ratchet** | n/a | Repeated compliance lowers leverage after every cooldown, shifting confidence penalties, programme vetoes and AI funding paths world-wide (7.3, Q6) | Diagnostic 4: leverage paths | Decided by Q6 before measurement |
| **Engineering**: stale records | n/a | A record surviving an exit from electoral rule, or a resolution acting on a replaced issuer (fixed in this revision: 8.3, 4.4) | Controls 27-32 | Fix before measurement |
| **A7**: ballot flips against non-ballot takeovers | 8.462 against 3 | More takeovers in the denominator | Gate A7 | Stop rule |
| **A9**: coups under $8,000 a head | 168/168 | Electoral nations with sourced leverage and higher income per head (check South Korea, Turkey and any big-economy democracy above .20) | Gate A9 | Stop rule |
| **A3, A5, A8** | 0/20; 12/12; 40/40 | The break seats Nationalist; big-eight exposure is low | Gates | Stop rule |
| **Engineering**: save format | n/a | The single `GovState` literal; strict new types; old saves; forward-compatibility loss | Controls 11, 16 | Fix before measurement |
| **Engineering**: byte identity | n/a | An entry point reading state before its switch check | I1, I2, the golden digests | Fix before measurement |
| **Engineering**: UI and server parity | n/a | A number recomputed in JavaScript, or effects drifting from the arm | Control 23, the web tests | Fix before measurement |
| **Engineering**: performance | **Not measured for this path; no figure supports a claim either way.** The isolated resource-timing test (bar .15 ms/month; 0.0623 in the 1 October follow-up, 0.1114 in D2's run on a loaded container) times only resource work in a lens-off world (`lib.rs:5966`): the `resources` row of `SYSTEMS`, its buy pass and the appetite scan. So it cannot see this change. | Per nation each tick (each day on the daily clock): one `Option` check in the pre-split validation step, and one more in the electoral-branch tick. Per AI actor each tick, since `ai_stratagems` asks `ai_lever` every tick and not once a month: up to three acceptance reads in `ai_order`. | A government/stratagems timing on a both-switches world, predeclared in the plan. The isolated resource-timing test is still run (CONTRIBUTING) but cannot show this cost. | Report and fix |
| **Engineering**: census parse | n/a | A new headline colliding with a census pattern | Control 22 | Fix before measurement |
| **Engineering**: daily clock and decision windows in browser saves | n/a | Calendar edge cases (month 12 rollover, day 31) | Control 15 | Fix before measurement |

---

## 17. Open questions for the design authority (consolidated)

This list merges the first draft's Q1-Q17 with the design review's QA-QK and the engineering
review's Q18-Q27, without duplicates, and renumbers them. Each entry gives the options in one
line and its sources:

- **"Was Qn"** is the first draft's number.
- **DA** is the design-authority-lens review; **ER** is the engineering-risk-lens review. Both
  are in the [review record](2026-10-04-civilian-authority-dispute-contract-review.md).

"As drafted" marks the first draft's behaviour. It is kept as one option and is not
recommended.

### 17.1 Blocking before measurement

These are design conflicts with a binding rule, an A1 hard constraint or a source text. No
default is proposed. The predeclared plan (section 15, step 0) may not be written until each
one is answered.

1. **Q1. AI temperament.** (a) P Prudent; (b) F Floor; (c) A Assertive; (d) no AI orders (player-only disputes). On 11.1 as written only (a) is consistent, and (d) makes 11.1 moot; (b) or (c) need 11.1 rewritten and an explicit ruling that the combination is not a forbidden trigger (11.3). Decide on design grounds only, citing no seeds 0-11 material. *Was Q1; DA HIGH-2, QF; ER HIGH-2, MEDIUM-5, Q21, Q27.*
2. **Q2. AI issuance mechanism.** (a) the existing 0.02 draw through `ai_lever` (shifts the RNG stream; A2 risk); (b) deterministic issuance at the January review in `ai_government` (no draw; a seed-invariant calendar, against iron rule 3's "usually"). Interacts with Q3. *Was Q11.*
3. **Q3. Order precedence over stratagem cards.** (a) the order before cards, as drafted (displaces card play for most of each wait under any temperament that issues); (b) the order only in ticks where no card is chosen; (c) the order outside `ai_stratagems` altogether (needs Q2 (b)). *DA MEDIUM-3, QD; ER MEDIUM-1, Q20.*
4. **Q4. Escalation gap and route 2's `D >= .25` guard.** (a) the full gap with no discontent condition, as drafted (the reviews find it bypasses the guard where effective loyalty `< .35`); (b) the gap capped at the authority component, `min(gap, .45·leverage·s)`; (c) orders or enforcement refused while effective loyalty is below .35 (route 2's domain); (d) another rule. *DA HIGH-3, QB.*
5. **Q5. Funding and compliance while a dispute is open.** (a) any rise of acceptance to .35 closes the dispute as `CompliedWhileOpen`, as drafted (conflicts with the geography review's "not automatically settling" and the 22 September record's "not an automatic cure"; 7.4); (b) compliance while open only through an explicit government action; (c) recovery stops accrual but does not close the dispute. Also: is a hysteresis margin or minimum duration wanted? *Was Q7; DA MEDIUM-1, QG, LOW-2.*
6. **Q6. Compliance effect, repeat orders and saved powers.** (a) leverage −.125·s on every compliance, repeatable after each cooldown, with no powers saved, as drafted; (b) complied scopes saved as institutional powers that cannot be ordered again; (c) total civilian-order consolidation bounded or prorated like the lawful transfer. Also: does `army_authority`'s two-path module contract gain a third path, and should compliance also lower authoritarianism, as a lawful transfer does (drafted: no)? *Was Q4; DA MEDIUM-2, QC; proposal step 1.*
7. **Q7. Rule 9's reach.** (a) confirm that the user's 4 October D2 decision extends to orders, disputes, accrual and firing (rule 9, I11, the `CivilianControl` lapse); (b) treat the D1 guard as a separate rule needing its own approval; (c) no civilian-control guard in D1 (exposes the A6 tripwires; section 16). *DA MEDIUM-4, QE.*
8. **Q8. Lawful issuer before the first modelled ballot.** (a) a completed modelled ballot (`g.elected`) and a six-month record, as drafted (every 1990 incumbent waits for its first ballot; A4 timing); (b) a sourced opening mandate also counts; (c) any electoral government with an established six-month record. *DA MEDIUM-6, QI.*
9. **Q9. Prices, AI reserve and political-capital drain.** (a) order 25, withdraw 12 `ALWAYS`, negotiate 0, enforce 0 plus .20 a month, and an AI reserve of price + 20, as drafted; (b) other prices; (c) a larger AI reserve, so orders cannot starve `SuspendConstitution` (55 PC held), `BanParty` (60 held), cards (cost + 20) or `SecurePillar` (above 55 held). *Was Q13; ER MEDIUM-2, Q25.*
10. **Q10. Cooldown basis and carry-over.** Basis: (a) 12 months from closure, as drafted, or (b) from issue. Carry-over: (c) it binds whichever party governs, as drafted, or (d) it resets when the issuer is replaced. Also: do lapse receipts start it? *Was Q14; ER Q26.*
11. **Q11. Pressure across a negotiation.** (a) pressure accrued against the wider order is kept, as drafted; (b) it is rescaled to the narrower scope's gap; (c) it is reset to 0 on narrowing. *ER L2, Q22.*
12. **Q12. Enforcement at zero political capital.** (a) enforcement continues at no cost, as drafted (consistent with the existing upkeep); (b) it lapses into a withdrawal; (c) another consequence. *DA LOW-1, QK.*
13. **Q13. Escalation headline and takeover-watch legibility.** Headline: (a) the exact route-2 template, as drafted, or (b) the template plus a census-compatible suffix naming the refused order. Watch: (c) no fifth road, as drafted (the coup road reads far from its trigger before a dispute coup, and the hatching and chip stay quiet), or (d) a fifth road or gauge (changes the watch payload in every lens-on world). *Was Q10; DA MEDIUM-5, QH.*
14. **Q14. Sequencing if D2 moves a gate.** On current evidence D2 moved no gate (confirmed on ubuntu and windows CI, run 37178241397): its local Linux run is identical to `39369f0`, so the post-D2 baseline equals the pre-D2 baseline (3.1, 15). The question is conditional. If D2's pending Windows or CI runs, or its integration, show a regression other than A1: (a) D1 waits until it is resolved; or (b) D1 proceeds against that baseline, with a stated reading of the stop rule, Q25 and the 0-199 condition. *DA MEDIUM-7, QJ.*

### 17.2 Before implementation

Q15-Q22 have a proposed default that is consistent with the binding rules. Q23 and Q24, raised
by the reviews, have no proposed default. Confirm or answer each one before stage 1; the answers
go into the plan.

15. **Q15. Scopes and stakes.** (a) three rungs at 1/3, 2/3 and 1, Army only (proposed); (b) other rungs or stakes; (c) Security institutions orderable too. *Was Q2.*
16. **Q16. Compliance weight.** (a) reuse the programme veto's .45 (proposed); (b) a separately labelled constant. *Was Q3.*
17. **Q17. Decision-window default.** (a) one following calendar month, after which the order stands as enforced (proposed); (b) a default of withdrawal. *Was Q5.*
18. **Q18. Escalation rate and duration.** (a) route 2's .60 slope with no floor (proposed; depends on Q4); (b) slowed by the existing `civilian_public_mandate`; (c) a maximum standoff duration. *Was Q6.*
19. **Q19. Availability with the takeover switch off.** (a) closed entirely (proposed; keeps browser play byte-identical); (b) the order and compliance without escalation in browser play. *Was Q8.*
20. **Q20. Same-type institutions.** (a) one national leverage (proposed); (b) per-institution leverage (a data and research question). Also: in a coup, should only the disputing institution be reseated as the mover? *Was Q9.*
21. **Q21. Withdrawal consequence.** (a) political capital only (proposed); (b) a withdrawal also raises leverage or costs stability (no source supports either). *Was Q12.*
22. **Q22. Frozen disputes.** For a save carrying an open dispute into a takeover-off world, which includes every browser `/api/load`: (a) freeze it (proposed), which also requires gating the `regime_break` hook on the switches and leaves the dispute invisible in the browser; (b) close it as lapsed on load. *Was Q16; ER L1, L7, Q24.*
23. **Q23. Legality lost mid-dispute while still electoral.** When rule 5 or 6 fails during a dispute (a ban clears `unrestricted_mandate`; interim status; the record lapses): (a) the dispute lapses, with a new resolution variant; (b) `dispute_acceptance` ignores rules 5-6 for open records. Either way I16 must hold. *ER MEDIUM-3, Q19; DA HIGH-1 (last bullet), QA.*
24. **Q24. The issuer's own exit.** As drafted, an issuing government can end its own enforced dispute by suspending the constitution (40 PC; `ElectoralRuleEnded`, no further consequence), or, under Q23 (a), by banning a party. Options: (a) accept that; (b) record such an exit as `Withdrawn`, with its price; (c) another consequence. *DA HIGH-1, QA.*

### 17.3 Later

25. **Q25. If A1 still fails but every other gate passes.** (a) keep the mechanic, for example for the player, without A1 credit; (b) revert. It changes no gate claim, so it may be answered after measurement, though answering it in the plan is preferable. *Was Q15.*
26. **Q26. Historical anchors.** Commission a research packet under the C01 workflow for the anchors in 2.2 before any coefficient, scope, price or test may cite them? It is needed before any 7.6 calibration entry can be filled. *Was Q17.*

### 17.4 Raised by the reviews and resolved in this revision (no design choice)

- **ER Q18 (where lapses run):** validation runs for every nation holding a record, before the
  electoral/regime split (8.3).
- **ER Q23 (re-run validation first):** every refusal, plan, arm and AI function reads the
  post-lapse view (4.4).
- **DA QA (which exits close a dispute, and how):** the exits are mapped in 8.3. Its residual
  design parts are Q23 and Q24, and its control requirement is controls 27-30.

**First-draft numbers to this list:**

| Was | Now |
| --- | --- |
| Q1 | Q1 |
| Q2 | Q15 |
| Q3 | Q16 |
| Q4 | Q6 |
| Q5 | Q17 |
| Q6 | Q18 |
| Q7 | Q5 |
| Q8 | Q19 |
| Q9 | Q20 |
| Q10 | Q13 |
| Q11 | Q2 |
| Q12 | Q21 |
| Q13 | Q9 |
| Q14 | Q10 |
| Q15 | Q25 |
| Q16 | Q22 |
| Q17 | Q26 |

---

## 18. Revision log (4 October 2026)

The revision was made by Claude, automated agent, after the two automated reviews. No code,
data, test, threshold or cohort changed. The finding IDs are the reviews' own. The review
record maps every finding to its disposition.

| # | Change | Sections | Findings addressed |
| --- | --- | --- | --- |
| R1 | Status line replaced. Added the Reviews row, and the post-D2 line-shift note to the Base row. | header | (request) |
| R2 | Added the BLOCKING notice listing Q1-Q14. | after the decisions | DA, ER overall verdicts |
| R3 | Validation and lapses now run for every nation holding a record, before the electoral/regime split, behind both switches, and do nothing without a record. The electoral-branch tick re-runs them, idempotently. | 3.4, 8.3, 10.1, I2, I10, controls 27-31 | DA HIGH-1; ER HIGH-1, Q18 |
| R4 | Mapped every exit from electoral rule to the drafted resolution it implies (8.3 table). The residual design choices became Q23 and Q24. | 8.3, 17 | DA HIGH-1, QA; ER HIGH-1 |
| R5 | Post-lapse view: every refusal, plan, arm, effects function, `ai_order` and `ai_response` re-runs validation first. Rules 12-13 read the post-lapse view, and the order arm closes a stale record first (`closes_stale`). | 4.4, 5, 11.2, I15, control 32 | ER MEDIUM-4, Q23 |
| R6 | Acceptance made total on validated records (`dispute_acceptance`, I16). The rule 5/6 choice became Q23. | 8.3, 10.1-10.3, I16, control 34 | ER MEDIUM-3, Q19 |
| R7 | Defined `leverage_after` (read before `army_seizure` on `Escalated` and `GovernmentRemoved`). | 8.1, 10.3, control 36 | ER L3 |
| R8 | Specified derives for every new type and added `DisputeAction`'s definition. | 4.1, 4.4, 8.1 | ER L4 |
| R9 | Named `the_daily_clock_preserves_the_political_arm_on_world` (`lib.rs:2183`) and the observer's `selected_lever` (`government_a1_observer.rs:80`), and listed D2's guard tests. | 14.3 | ER L5 |
| R10 | Ordinal correspondence is now asserted. One sim name reader (`institution_spec_name`) replaces the web's private `institution_name`, with byte-identical output. | 3.4, 4.2, control 33 | DA LOW-3; ER L6 |
| R11 | D2 alignment facts: D2's guard is a private constant read only inside `army_civilian_confidence_penalty`; `civilian_control_holds` does not exist and D1 would add it; added commits, integration status and the gate result. | header, 3.1, 3.4, 5 | D2 review process note; DA, ER "D2 plan claims" |
| R12 | Removed every "expected direction" and A1-effect statement per temperament (1.3, 11.2, 11.3, 16). Labelled the 7.5 and 10.2 illustrations and the risk register "derived from seeds 0-11, must not inform the decision". | 1.3, 7.5, 10.2, 11, 16 | DA HIGH-2 (last bullet), QF; ER MEDIUM-5, Q27 |
| R13 | Withdrew the Floor default. The temperaments are now presented neutrally, with a formula-based consistency analysis. | 11.2, 11.3 | DA HIGH-2; ER HIGH-2, Q21 |
| R14 | Surfaced the escalation-gap bypass of route 2's `D >= .25` guard, and withdrew the "correlated input" claim. | 10.2, 10.5, 16 | DA HIGH-3, QB |
| R15 | Quoted both source texts against funding settling a dispute automatically, and withdrew the misreading of the geography review. Added the hysteresis note. | 7.4, 9.4 | DA MEDIUM-1, QG, LOW-2 |
| R16 | Surfaced the compliance ratchet, the missing saved institutional powers and `army_authority`'s two-path contract. | 7.3, 7.6, 16 | DA MEDIUM-2, QC |
| R17 | Surfaced card displacement as an AI-behaviour decision, not only an RNG risk. | 3.3, 11.4, 16, control 35 | DA MEDIUM-3, QD; ER MEDIUM-1, Q20 |
| R18 | Rule 9 is now labelled an extension of the user's decision that needs confirmation. | 5, I11, 7.6 | DA MEDIUM-4, QE |
| R19 | Stated rule 5's choice of lawful issuer. | 5 | DA MEDIUM-6, QI |
| R20 | Recorded that the post-D2 baseline equals the pre-D2 baseline on the local evidence, and added the conditional stop-rule branch. | 3.1, 15 | DA MEDIUM-7, QJ |
| R21 | Stated the political-capital drain and enforcement at zero political capital. | 6, 7.6, 16 | ER MEDIUM-2, Q25; DA LOW-1, QK |
| R22 | Stated the census-compatible headline suffix option and the takeover-watch legibility cost. | 9.3, 12, I12 | DA MEDIUM-5, QH |
| R23 | Stated the frozen-dispute consequences: the hook's switch gating and the browser `/api/load`. | 8.3, 12 | ER L1, L7, Q24 |
| R24 | Flagged pressure carried across a negotiation. | 9.2 | ER L2, Q22 |
| R25 | Added the "What would calibrate it" column (SPEC §4 convention). | 7.6 | DA LOW-4 |
| R26 | Performance row: no figure measures this path. Kept the reviewer's correction (the resource-timing test sees only the `resources` row; `ai_lever` runs every tick), and added the per-nation validation check and D2's 0.1114 reading. | 16 | ER correction 5 |
| R27 | Consolidated, deduplicated and renumbered every open question (17), and updated all references in the body. | 17, body | (request) |
| R28 | Retained the reviewers' in-file factual corrections (DA corrections 1-5, ER corrections 1-5). | various | DA, ER corrections |
| R29 | Verification pass (Claude, automated agent; not human or Codex review), factual and labelling fixes only. Labelled 1.2 "derived from seeds 0-11". Removed the 10.2 illustration's link to the Assertive temperament. Corrected 8.3's same-tick example: the scheduling seam re-forms the government (row 3) and sets `awaiting_first_election`, which no row reads until Q23. Added `armed_opposition_victory` to the exit table. Corrected the lever-first rationale's lines to `stratagems.rs:553-561`, and the card-displacement wording. Stated that 55/60 are the AI's holding thresholds, not prices (40/18). Noted which controls test a drafted option. Qualified control 30's peace-transition case. The timing test times resource work, not only the `resources` row. Stated that Q23 and Q24 have no proposed default. Clarified two Appendix A rows. Made 10.2's Sao Tome arithmetic exact (20th monthly accrual), as ER correction 1 did for Pakistan and Thailand. | 1.2, 6, 8.3, 10.2, 11.4, 14.2, 16, 17.1, 17.2, Appendix A | ER MEDIUM-5, Q27; DA HIGH-1; citation checks |

---

## 19. Design-authority decisions (4 October 2026)

The user, as design authority, adopted the following answers in one decision on 4 October 2026
("Adopt recommended set"), after Claude presented them as the set consistent with the binding
rules and source texts. They replace the open status of the questions in 17.1 and 17.2; the
question text above is kept as the record of what was decided.

| Q | Decision |
|---|---|
| Q1 | (a) Prudent: the AI issues only an order its own pure plan says the Army would accept. |
| Q2 | (a) the existing 0.02 draw through `ai_lever`. |
| Q3 | (b) the order is chosen only in ticks where no stratagem card is chosen. |
| Q4 | (b) the escalation gap is capped at the authority component, `min(gap, .45 * leverage * s)`, so route 2's live `D >= .25` guard is not bypassed. |
| Q5 | (b) compliance while a dispute is open happens only through an explicit government action; funding does not settle a dispute automatically. |
| Q6 | (b) complied scopes are saved as institutional powers and cannot be ordered again. |
| Q7 | (a) the civilian-control guard (authoritarianism `<= .20`) also blocks orders, disputes, accrual and firing. |
| Q8 | (b) a sourced opening mandate counts as lawful authority to issue an order. |
| Q9 | (a) prices as drafted, plus (c) a larger AI reserve, so orders cannot starve `SuspendConstitution`, `BanParty`, cards or `SecurePillar`. |
| Q10 | (a) 12 months from closure, and (d) the cooldown resets when the issuer is replaced. |
| Q11 | (b) pressure is rescaled to the narrower scope's gap on negotiation. |
| Q12 | (b) enforcement at zero political capital lapses into a withdrawal. |
| Q13 | (b) the route-2 headline template plus a census-compatible suffix naming the refused order; (c) no fifth takeover-watch road. |
| Q14 | Moot: D2 moved no gate on ubuntu or windows CI (run 37178241397). |
| Q15-Q22 | The proposed defaults: (a) in each case. |
| Q23 | (a) the dispute lapses, with a new resolution variant. |
| Q24 | (b) an issuer's own exit by suspension (or ban) is recorded as `Withdrawn`, with its price. |
| Q25, Q26 | Not decided (section 17.3). |

The measurement remains unknown until run, follows section 15 once, and must not be tuned to
seeds 0-11.

## Appendix A: code anchors at `39369f0`

| Item | Location |
| --- | --- |
| `GovState`, its constructor literal | `spheres-sim/src/government.rs:5908`, `:6114` |
| `annulment_check`, `annul_election` | `:7197`, `:7235` |
| Lawful-transfer consolidation | `:7160-7163` |
| `upkeep` | `:7328` |
| Lever block, prices, `suspend_refusal` | `:7514-7534`, `:7608` |
| `lever_effects`, `ai_lever` | `:8211`, `:8254` |
| `ARMY_PROGRAMME_VETO_WEIGHT`, `civilian_army_executive_leverage`, `established_civilian_record` | `:8506`, `:8508`, `:8516` |
| `civilian_public_mandate`, `army_civilian_confidence_penalty`, `army_programme_veto_loyalty` | `:8531`, `:8573`, `:8591` |
| `pillar_targets`, `walk_pillars` | `:8690`, `:8774` |
| `ELECTORAL_COUP_*`, `electoral_coup_settled_months` | `:8843-8848`, `:8857` |
| `electoral_army_tick`, `maybe_electoral_coup`, `break_electoral` | `:8881`, `:8951`, `:9006` |
| `regime_break` (`army_seizure` call) | `:9331` (`:9372`) |
| `ai_army_funding_floor`, `ai_government` | `:9390`, `:9423` |
| `tick` (route-2 call site) | `:9482` (`:9543-9548`) |
| `current_leverage`, `army_seizure`, `consolidate_transfer` | `spheres-sim/src/army_authority.rs:104`, `:115`, `:127` |
| `effective_army_loyalty`, `backing` | `spheres-sim/src/blocs.rs:378`, `:364` |
| `ai_stratagems`, `ai_lever` call, the 0.02 draw | `spheres-sim/src/stratagems.rs:529`, `:540`, `:577` |
| `Command`, lever variants | `spheres-sim/src/lib.rs:106`, `:295-305` |
| `price_of`, `affordable`, `command_price`, lever prices | `:368`, `:377`, `:386`, `:612-616` |
| `world_refusal`, lever refusals | `:702`, `:750-754` |
| `refusal_of`, `apply_command`, lever arms | `:857`, `:900`, `:1412-1418` |
| `SYSTEMS` (`ai_stratagems` before `government`) | `:1507` (`:1535`, `:1540`) |
| `action_json`, `effects_of`, `government_json`, `parse_command` | `spheres-web/src/main.rs:1242`, `:1270`, `:1351`, `:6516` |
| `category`, `enrich`, `preview` | `spheres-web/src/government_view.rs:64`, `:92`, `:234` |
| `attention`, `actionCard` | `spheres-web/ui/government-ui.js:73`, `:86` |
| A1 gate; A6 definition | `spheres-sim/tests/bloc_census.rs:983-993`; `:79-92`, `:835` |
| Census coup headline parse (`COUP IN `, `removes the elected government`) | `spheres-sim/tests/bloc_census.rs:377`, `:394` |
| `tick`'s electoral/regime split (pre-split validation site) | `government.rs:9505` |
| `suspend_constitution`, `ban_party` (clears `unrestricted_mandate` at `:7813`) | `:7679`, `:7800` |
| `seat_spec_pillars`, `pillar_name`, `maybe_coup`; `regime_break`'s two callers (`break_electoral` at `:9015`, `maybe_coup` at `:9249`) | `:8932`, `:9302`, `:9204` |
| `uprising`, `settle_uprising`, `generic_regime_collapse` | `:9065`, `:9077`, `:9149` |
| Their callers in `politics::tick` (after `government` in `SYSTEMS`) | `spheres-sim/src/politics.rs:249`, `:252`, `:259` |
| Crackdown card `+.06` authoritarianism; peace transition `-.25` | `spheres-sim/src/stratagems.rs:381`; `spheres-sim/src/campaign_peace.rs:438` |
| `army_authority` module contract (two paths) | `spheres-sim/src/army_authority.rs:3-4` |
| `the_daily_clock_preserves_the_political_arm_on_world`; resource-timing test | `spheres-sim/src/lib.rs:2183`; `:5966` |
| A1 observer `selected_lever` | `spheres-sim/src/government_a1_observer.rs:80` |
| Web `institution_name` (to be replaced); `/api/load` forcing takeover off | `spheres-web/src/government_view.rs:58`; `spheres-web/src/main.rs:7166` |
| D2 (post-D2 tip, not at `39369f0`): `CIVILIAN_CONTROL_AUTHORITARIANISM`, the guard line | `government.rs:8513`; `:8588`, inside `army_civilian_confidence_penalty` (`:8580`) |
