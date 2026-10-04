# Civilian-authority dispute (D1): predeclared measurement plan

**Status: predeclared on 4 October 2026, before any D1 runtime edit or build. Prepared by Claude, an automated agent. This is not human review, not Codex review and not approval.**

[`plan.json`](plan.json) is the binding record; this page summarises it. Where the two differ, `plan.json` governs.

| | |
| --- | --- |
| Task | `CODEX-S27-A1-01`, direction D1 |
| Design | [D1 contract](../../../../political-arm/2026-10-04-civilian-authority-dispute-contract.md), as decided by the user in its section 19 (Q1-Q24; Q25 and Q26 undecided) |
| Base | `f6597cf5`, which is `39369f0` plus the user-decided D2 guard. Its runtime diff is byte-identical to CI-tested `e857e8ac`. |
| Where | Sparse worktree `scratchpad/wt-d2`, new local branch `claude/d1-dispute-wip` (not pushed), `CARGO_TARGET_DIR=scratchpad/target-d2`, `CARGO_BUILD_JOBS=3` |
| Toolchain | rustc and cargo 1.97.0, Linux x86_64. The container is shared with the S25 diagnostic. |

## Hypothesis and prediction

The model does not represent an Army institution refusing a lawful civilian order. The decided mechanism adds that cause.

**No prediction is made.** The effect on A1 and on every other gate is unknown. No value or direction is recorded, and nothing here was chosen by its effect on seeds 0-11.

One structural fact, not a prediction, affects how the result should be read. Under the decided Prudent AI (Q1a):

- every AI order is accepted at issue;
- the census has no player, so it can contain no dispute and no escalation;
- any census change therefore comes only through compliance (lower leverage and saved powers) and through world-path effects (new draws, the order's candidate slot, political capital).

Diagnostic D3 checks this directly: a nonzero count of AI refusals is a bug.

## What is built (section 19, with labelled readings)

**The AI's orders**

- **Who and what (Q1a).** The AI issues only orders its own plan says the Army will accept.
- **When (Q2a, Q3b).** Orders ride the existing single 0.02 draw in `ai_stratagems`. An order is considered only when there is no lever and no chosen card.
- **Reserve (Q9a+c).** Prices are as drafted. The AI issues only at 93 PC or more: the 25 PC price plus a 68 PC reserve, which is the largest holding the AI's other political-capital uses need.

**Compliance and escalation**

- **Escalation gap (Q4b).** The gap is capped at the authority component, `min(0.35 - acceptance, 0.45 * leverage * s)`. Route 2 and its live guards are untouched.
- **Settling (Q5b).** Funding never settles a dispute by itself. Compliance happens only through an explicit Negotiate or Enforce.
- **Saved powers (Q6b).** Complied scopes are saved, per institution, and cannot be ordered again.
- **Civilian-control guard (Q7a).** At authoritarianism `<= .20` there is no order, dispute, accrual or firing.
- **Opening mandates (Q8b).** A sourced opening mandate counts as a lawful issuer.

**Cooldown, negotiation and enforcement**

- **Cooldown (Q10a+d).** 12 months from closure, binding the same issuer only.
- **Negotiation (Q11b).** Pressure is rescaled to the narrower scope's gap.
- **Unpaid upkeep (Q12b).** Upkeep that cannot be paid lapses into a withdrawal.

**Headline and lapses**

- **Headline (Q13b+c).** The route-2 headline plus a census-compatible suffix. There is no fifth watch road.
- **Lost mandate (Q23a).** A new `MandateLapsed` lapse.
- **The issuer's own exit (Q24b).** The issuer's own suspension or ban is recorded as `Withdrawn`, at its price.
- **Q15-Q22.** The proposed defaults.

Where section 19 leaves a mechanic open, `plan.json` records **readings R1-R13**. They are labelled as Claude's readings for review and are fixed now, before any result exists. A reading can be overruled only by a dated amendment made before measurement. The coefficients C1-C15 and E1 are each labelled an invented design assumption, with its basis.

## Stages and red-first tests

1. **S1-pre.** Create the branch from `f6597cf5`. On the unmodified runtime, run the existing focused suites and record the byte-identity pins:
   - P1, lens off;
   - P2, takeover off;
   - P3, both switches on with no sourced leverage.
2. **S1-red.** The spheres-web test `a_lawful_civilian_order_and_its_refusal_are_represented` must compile and fail on `f6597cf5`, at `parse_command`.
3. **S1a.** A behaviour-preserving refactor: `civilian_control_holds`, the shared veto helper, and the single institution-name reader.
4. **S1.** The order, the record, the resolutions, validation and lapses, the Prudent AI and the UI. There is no escalation yet.
5. **S2-red.** The sim test `an_enforced_refusal_accrues_authority_pressure_and_removes_the_government` must fail on the stage-1 runtime.
6. **S2.** Escalation.
7. **S-review.** An automated review (labelled), then the final commit SHA is recorded.

The controls are the contract's 1-36, updated by the decisions, plus 37-43 (saved powers, the zero-PC lapse, opening mandates, the AI reserve, the gap cap, frozen disputes and the pins).

The existing tests must pass unmodified:

- the government, army_authority, stratagem, blocs and opening_mandates tests;
- both Algeria chronology tests and D2's six guard tests;
- the inertness tests, `the_daily_clock_preserves_the_political_arm_on_world`, and the determinism and save tests;
- the web government tests and `run-unit.cjs`.

**Forbidden before measurement:** any `bloc_census` run, the census scan, the a1-diag and d2-diag harnesses, any multi-seed outcome sweep, and the workspace suite. The workspace suite is forbidden because A6, A8, A9, A10 and attribution are not `#[ignore]`d and would run.

## The single measurement

Run once, on the final reviewed commit, after a build-only `--no-run`:

```sh
cargo test --locked --release -p spheres-sim --test bloc_census -- --include-ignored --skip bloc_census --nocapture
```

Record:

- the commit, the diff against `f6597cf5`, the toolchain, the binary SHA-256, the exit code and the wall time;
- every gate line;
- the 12 A1 seed rows and their SHA-256.

A pass is exit 0 with 11 of 11. The cross-platform confirmation is the CI `political-calibration` job on ubuntu-latest and windows-latest for the same commit. It is pending until an authorized push and is not replaced by the local run.

**Baseline (post-D2, equal to `39369f0`)**

| Gate | Baseline | Result |
| --- | --- | --- |
| A1 | median 7.5 coups, top-three 4/7 = 0.571429; seed table `87b3ea9b…3deb` | fail |
| A1 per seed | 3/7, 4/6, 3/5, 5/8, 4/8, 4/8, 4/7, 5/8, 4/8, 4/7, 5/8, 4/7 | |
| A2 | 8/12; Algeria Islamist 0/12, annulled 12/12 | pass |
| A3 | 0/20 | pass |
| A4 | 16 | pass |
| A5 | 12/12 | pass |
| A6 | 0 in 12/12 | pass |
| A7 | 8.462 | pass |
| A8 | 40/40 | pass |
| A9 | 168/168 | pass |
| A10 | 0.335 | pass |
| Attribution | | pass |

**After the gate run, on the same commit**

- **Diagnostics** (measurement only):
  - the 12-seed census scan;
  - the unchanged scout harness, diffed against D2's log: which countries newly fire, their first-event dates and repeats;
  - a dispute harness counting orders, compliances, refusals, resolutions, escalations, AI refusals (expected 0), saved powers and leverage paths.
- **Engineering:**
  - the workspace suite (its census tests must equal the gate run);
  - the isolated resource-timing test, run alone;
  - `run-unit.cjs`, and the spheres-web release build;
  - a report-only cost test of the new entry points.
- **Development cohort, optional, once.** The fixed 0-199 cohort runs only if every original gate passes locally and on both CI platforms. **Never the 1000-series. Never append seeds.**

## Stop and report

- **Reporting.** Pass or fail is reported exactly as measured.
- **Any regression.** A baseline-passing gate failing, A1's count leaving 4..=14, or an Algeria test failing is reported, and the candidate is rejected for integration. No gate, country, coefficient, scope, price, timing, temperament, threshold, seed or cohort change is made to compensate.
- **A1 fails while the other gates pass.** This is reported as a failure; whether to keep the mechanic (Q25) is the user's decision.
- **Bugs found after measurement.** A demonstrated implementation bug may be fixed and measured once more. Both results are kept.
- **Design changes after measurement.** None without a new user decision.

A pass closes none of A1, `CODEX-S27-A1-01`, S27, G5 or CP1. SPEC, `docs/government-ui.md` and the BUGS coefficient entries change only after approval.
