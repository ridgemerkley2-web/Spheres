# A1 scope at 39369f0: exact reproduction, diagnosis and decisions needed

Task: `CODEX-S27-A1-01` (Codex-owned). Prepared 4 October 2026 by **Claude, automated agent - not
human or Codex review**, at the user's request. Source: integration tip
`39369f0ee6451466d03778d0273a7f72aafcf8c0` (`codex/campaign-certification`, checked out as
`claude/nifty-bohr-0j27qi`).

This is a scoping packet. **Nothing was implemented.** No source, data, coefficient, threshold,
cohort or test changed, and nothing was committed or pushed. **A1, `CODEX-S27-A1-01`, S27, G5 and
CP1 remain open.** This packet gives no qualification, closes no task and records no acceptance.

## 1. Bottom line

**A1 still fails, and the failure reproduces exactly.** The unchanged gate command ran on local
Linux with rustc/cargo 1.97.0 on 4 CPUs:

    cargo test --locked --release -p spheres-sim --test bloc_census -- --include-ignored --skip bloc_census

It gave 10 passed, 1 failed (A1), exit 101, in 104.73 s ([log](reproduction/a1-repro.log)); a
`--nocapture` rerun printed every gate's value ([log](reproduction/a1-repro-nocapture.log)). The 12
seed rows are **byte-identical** to both CI jobs of run 37056033496: ubuntu-latest (job
111000855599, rustc 1.99.0, 97.47 s) and windows-latest (job 111000855667, rustc 1.99.0 msvc,
92.75 s). All three tables hash to SHA-256 `87b3ea9b…3deb` ([local](reproduction/a1-local-table.txt),
[ubuntu](reproduction/a1-ci-ubuntu-table.txt), [windows](reproduction/a1-ci-windows-table.txt)),
so these three runs reproduced identically across Linux/Windows and rustc 1.97/1.99.

| Gate | Value at 39369f0 | Bar | Result |
|---|---|---|---|
| A1 elected-government coups | median 7.5 coups; median top-3 share **4/7 = 0.571429** | 4..=14 and strictly < 0.50 | **fail** |
| A2 Islamist takeover by 2000 | 8/12; Algeria Islamist 0/12, annulled 12/12 | > 6 and <= 10; < 6; >= 2 | pass |
| A3 Communist takeover rare | 0/20 | <= 2 of 20 | pass |
| A4 democratisation by end-1996 | median 16 | 15..=40 | pass (margin 1) |
| A5 ex-communist returns | 12/12 | > 6 of 12 | pass |
| A6 no route event in a 1990 democracy | 0 events in 12/12 seeds, 420 months | 0 in every seed | pass |
| A7 ballot flips vs takeovers | 8.462 | >= 3.0 | pass |
| A8 big eight keep their bloc | 40/40 | >= 24 of 40 | pass |
| A9 coups under $8,000 a head | 168/168 = 1.000 | >= 0.80 | pass |
| A10 discontent lead | 0.335 (seeds .320-.358) | >= 0.25, a takeover per seed | pass |

The attribution control also passes. A1 per seed, exact:

| Seed | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Elected coups | 7 | 6 | 5 | 8 | 8 | 8 | 7 | 8 | 8 | 7 | 8 | 7 |
| Countries | 7 | 5 | 5 | 6 | 7 | 7 | 6 | 6 | 7 | 6 | 6 | 6 |
| Top-3 share | 3/7 | 4/6 | 3/5 | 5/8 | 4/8 | 4/8 | 4/7 | 5/8 | 4/8 | 4/7 | 5/8 | 4/7 |

Only seed 0 is below one half; three seeds equal it. The gate is
`spheres-sim/tests/bloc_census.rs:983-993` (assertions 991-992); CI runs it in
`.github/workflows/verify.yml:216-231`.

## 2. Who takes the coups, and why the strict rule fails

The [read-only harness](harness/a1-diag/src/main.rs) replays the gate's world for seeds 0-11 and
reproduces every seed's count and share exactly ([log](diagnostics/a1-diag-seeds0-11.log)). It finds
87 elected coups in 8 countries:

| Country | Coups | Seeds | Repeats | First coup | Leverage at first coup |
|---|---|---|---|---|---|
| Guatemala | 21 | 12/12 | 9 | 1991-09 in every seed | .80 |
| Sao Tome and Principe | 14 | 12/12 | 2 | 1993-03 (9 seeds), 1995-07..09 (3) | .25 |
| Myanmar | 14 | 12/12 | 2 | 1996-07..1997-05 | .667 |
| Guyana | 12 | 12/12 | 0 | 1991-08/09 | .50 |
| Ecuador | 12 | 12/12 | 0 | 1993-04/05 | 1.0 |
| Comoros | 10 | 10/12 | 0 | 1995-10..1996-11 | .571 |
| Chad | 2 | 2/12 | 0 | 1993-10, 1996-01 | .40 |
| Mozambique | 2 | 2/12 | 0 | 1999-05, 1999-08 | .25 |

The cohort is close to seed-invariant: the seed mainly changes the repeats, some first-coup dates
(Sao Tome, Comoros, Myanmar), whether Comoros fires (10/12) and the marginal Chad and Mozambique
events. All 13 repeats read live leverage 1.0, the value a first seizure sets
(`army_authority::army_seizure`, `army_authority.rs:115`), after a reopening.

**Why the median per-seed top-3 rule fails** (explanation only - **not a tuning target**, not a
prediction). With a fixed core of 5-7 single-event countries the share cannot fall below 3/7, and
any repeat, or fewer than seven countries, puts a seed at 0.5 or above. The
[exact arithmetic](diagnostics/a1-breadth-arithmetic.txt) ([script](harness/breadth_arithmetic.py))
applies hypothetical changes to the measured counts:

| Hypothetical change to every seed | Median coups | Median top-3 | A1 |
|---|---|---|---|
| none (current) | 7.5 | 4/7 | fail |
| remove all repeats | 6.0 | 1/2 | fail |
| add 1 new single-event country | 8.5 | 1/2 | fail |
| add 2 new single-event countries | 9.5 | 4/9 | pass |
| add 2 new countries and 1 repeat in the top country | 10.5 | 1/2 | fail |
| add 1 new country with 2 coups | 9.5 | 5/9 | fail |

The count band caps additions at about six per seed. A1 is a **breadth** problem: too few polities
can experience the modelled cause of an elected coup. Suppressing repeats is neither justified (they
follow live conditions) nor sufficient. The table describes what an honest mechanism would have to
produce on its own merits; it must not be used to choose coefficients.

## 3. Why the eligible set is narrow

Route 2 in `spheres-sim/src/government.rs`:
- `electoral_army_tick` (8881) accrues pressure only while effective Army loyalty is below .35
  (`ELECTORAL_COUP_ARMY`, 8843) **and** discontent is at least .25 (8844), at
  `0.30·(2·(0.35−eff)+(D−0.25))` per month; otherwise it cools.
- `maybe_electoral_coup` (8951) also needs an Army, 12 settled months under the same party (8848),
  no pending first election, both conditions still live, and pressure of at least 1.0.
- The Army target (`pillar_targets`, 8690) is `0.20 + 0.65·funded − 0.45·exhaustion` (8495, 8672)
  minus `army_civilian_confidence_penalty` (8573), zero below D .25 or without a 6-month record (8516).

So below discontent .25 the Army's target is purely material, and effective loyalty can then fall
below .35 only through underfunding, war exhaustion or foreign Nationalist backing.
Firing countries combine chronic crisis, little fiscal headroom (9 of 10 firings in the earlier,
pre-repair [seed-0 observer](../a1-firing-observer-20260928/README.md) lacked it) and either
high sourced leverage (Guatemala .8, Ecuador 1.0, Myanmar .667) or lower leverage (Guyana .5,
Chad .4, Sao Tome .25). At the first coup the confidence penalty is .24-.26 in Guatemala and
Ecuador and .04-.11 elsewhere. Expected countries, pooled over 12 seeds × 252 months:

| Country | Observed | Missing condition |
|---|---|---|
| Pakistan | electoral with Army 3,024/3,024 months; max D .211; min eff .647 | neither ever met; penalty always 0 |
| Thailand | 3,024/3,024; max D .182; min eff .650 | neither ever met; penalty always 0 |
| Haiti | never electoral (3,024 regime months; authoritarianism .78) | route unreachable; [30 Sep witness](../political-repairs-20260930/opening-witnesses.json): round table unaffordable on 1 Jan 1990 (affordable by 1 Jan 1991); executive party cannot contest on both dates |
| Nigeria | never electoral (3,024; authoritarianism .85) | route unreachable |
| Peru | D >= .25 in 2,996 months; min eff .380; max penalty .180 | army never hostile |

A **quiet, funded, high-leverage population** never fires: each has at least 1,000 months with
D < .25 and effective loyalty of at least .35, and zero coups. Ghana 1.0, Gabon .833, Thailand .833,
Pakistan .8, Paraguay .8, Lesotho .75, Cameroon .714, CAR .6, El Salvador .6, Honduras .6, Fiji .5,
South Korea .5, Turkey .5 (first-observed leverage). Uruguay .6 and PNG .5 meet the same test but
are 1990 democracies protected by A6 (section 5).

[BUGS S4-1](../../../../archive/2026-09-30/BUGS.md) (line 3091) lists as calibration cases Pakistan
October 1999 ("a paid army ... the coup was the army's, not the street's"), Thailand February 1991
and Haiti September 1991 ("faster than any setting of this rate reaches"); none of the three
countries has an elected coup in this run. **The model has no representation of a lawful civilian
order refused by an army.** This packet reads that as a missing mechanism rather than a defect in
the existing guards; the [geography](../a1-geography-20260928/README.md) and
[follow-up](../a1-followup-20261001/README.md) packets likewise found no implementation defect.
Earlier trials acted on the same crisis population; those rejections stand.

## 4. Candidate directions (for design review; none implemented)

**D1 - complete the civilian-authority dispute contract.** Basis: the
[1 Oct proposal](../a1-followup-20261001/mechanism-proposal.md) (steps 1-5), the geography review's
"separately observable conflict over civilian executive authority", BUGS S4-1, the existing
programme veto (government.rs:8591, weight .45) and the 22 Sep record ("A financial appropriation is
not an automatic cure for political opposition to civilian authority"). Sketch, for review only:
- A refusable player/AI command, e.g. `AssertCivilianAuthority { nation, institution, scope }`,
  legal only with a completed unrestricted ballot, an established civilian record, a named Army with
  known live leverage above 0, authoritarianism above .20 (D2), no open dispute, and a paid
  political-capital price.
- Compliance from the server plan: acceptance = effective loyalty − W·leverage·scope (the
  programme-veto shape); refusal below the existing .35 line. Compliance lowers live leverage like
  `consolidate_transfer` (`army_authority.rs:127`); that is the AI's positive objective.
- A refusal stores a receipt-backed `DisputeRecord` (serde-default empty, so old saves load with no
  dispute). Resolutions from the same plan: negotiate a narrower scope, withdraw at a cost, or
  enforce.
- Only an unresolved, enforced dispute accrues a separate pressure, with no D >= .25 requirement.
  Firing reuses `break_electoral` and the existing "removes the elected government" headline,
  without double-charging an annulment or programme veto.
- The AI decides at the existing 0.02 draw (stratagems.rs:577) and never manufactures disputes; the
  government module draws no RNG. W, scope, price and rate are new invented assumptions to label.

*Expected effect.* The only identified direction that creates a new eligible population (the quiet
list). The section 2 rows are uniform illustrations, not necessary conditions: on their own merits
the measured outcome would have to broaden each seed's field by roughly two or more new countries
in most seeds, with any added repeats offset by further breadth, while staying within about six
extra coups per seed. Countries will tend to fire in almost every seed or none; neither sign nor
size can be predicted. Compliance in core countries could cut the count while the share stays or
worsens. A useful mechanic can still fail A1, and that must be reported.

*Regression risks.*
- **A2 (8/12, needs > 6):** a new AI draw in previously drawless months shifts the single world RNG
  stream; three earlier rejected candidates (iteration 22, reassessment 27, urgent response) had A2
  at 6/12. The two Algeria chronology tests must pass (named in [review.json](review.json)).
- **A4 (16 against 15, margin 1):** a dispute coup before end-1996 in an opened 1990 regime (Ghana,
  Gabon, Cameroon, CAR, Lesotho) costs about 1 per seed; two such countries fail A4.
- **A6 tripwires:** Uruguay (leverage .6, authoritarianism .12) and PNG (.5, .15); the D2 guard is
  required.
- **A10, margin quantified** ([log](diagnostics/a1-diag-a10-margin.log)): 1-4 added zero-lead
  takeovers per seed give medians .319/.313/.303/.295, weakest seed .246 at +4 (the same for any
  added lead below .187). Quiet-country leads are near zero, e.g. Pakistan −.014, Thailand −.023
  ([log](diagnostics/a1-diag-a10lead.log)).
- **Lower:** A7 (8.46) and A9 (168/168) low - check income per head of any richer new state, e.g.
  South Korea; A3 low (the break seats the Nationalist colour); A5 and A8 negligible.
- **Engineering:** old saves, daily == monthly, save/load/replay, lens-off and takeover-off byte
  identity, UI/server single-plan parity, same-type institutions; `bloc_census.rs` stays unchanged.

**D2 - civilian-control guard for sourced leverage.** A prerequisite safety decision (section 5). No
direct A1 effect, since no affected nation has an elected coup today; it protects A6.

**D3 - military-regime transitions (Haiti, Nigeria): opening-route review first.** Trace seed-0
opening decisions with the existing observer stages. Does a sourced 1 January 1990 transition
commitment justify a first-election record like `opening_mandates` (Nigeria's `leaders_1990.json`
row calls its SDP/NRC table "not yet elected")? Does the iteration-23 rule (the executive needs a
party in the promised ballot) block military executives contrary to design? No dated coup is
forced. At most one or two countries, possibly repeating; by the arithmetic it could pass A1 alone
only if both became single-event coup countries in most seeds.

**D4 - split-executive dismissals** ("Pakistan's dismissals" in the A1 docstring). A design question,
not an A1 repair: A1 counts only "removes the elected government" headlines, so counting a
constitutional dismissal would mean changing the census (forbidden) or misattributing it as a coup.

## 5. Docs-vs-code discrepancy: civilian control at authoritarianism <= .20 (D2)

[2026-09-22-calibration-repairs.md](../../../../political-arm/2026-09-22-calibration-repairs.md)
says at line 87 "Civilian control at authoritarianism `.20` or below blocks it" and at line 306
"That boundary suppresses the civilian-confidence penalty". The code does not apply that guard once
leverage is sourced: `civilian_army_executive_leverage` (government.rs:8508-8514) uses known live
leverage, and the `(authoritarianism − .20)/.40` proxy, zero at .20 or below, applies only to
unknown assessments. The comment at 8503-8504 records the iteration-20 choice: "Known historical
leverage replaces the old authoritarianism proxy only in this political channel". Maximum penalties
in 1990 democracies (authoritarianism in brackets): Uruguay (.12) .222, Brazil (.20) .185, Venezuela
(.18) .131, Argentina (.15) .102, PNG (.15) .088. Route 2 stays silent there today only because
its two conditions never coincide (Uruguay's minimum effective loyalty is .470). Options:
- **(a) The documented rule stands:** the function returns 0 at authoritarianism of .20 or below and
  D1 reuses it. This removes the five penalties, changing AI funding and fiscal paths (A2 risk),
  needs a check of focused tests that assume a penalty, and partly reverses iteration 20.
- **(b) The code stands:** a new record corrects the two doc statements (existing evidence is not
  edited), and D1 gets its own explicit guard so it can never act in an A6 nation.

## 6. Measurement protocol for any candidate

1. **Pin and predeclare:** base and candidate commits, diff, toolchain, a private `CARGO_TARGET_DIR`
   (CONTRIBUTING:39), and a plan like [urgent-plan.json](../political-repairs-20260930/urgent-plan.json)
   whose stop rule rejects on any original-gate regression, with no compensating change.
2. **Red first:** a focused regression fails on the unmodified runtime, plus the controls listed in
   [review.json](review.json).
3. **The unchanged gate on both ubuntu-latest and windows-latest** (section 1 command with
   `--nocapture`): pass means exit 0 and 11/11; record every value and a diff against review.json.
4. **Same-cohort diagnostics (measurement only):** the `bloc_census` scan with
   `SPHERES_CENSUS_SEEDS=12` and this harness; report newly firing countries, repeats, first dates.
5. **Engineering:** the workspace suite and the isolated resource-timing test (CONTRIBUTING:43-44).
6. **Only if every original gate passes,** at most one predeclared run of the fixed 0-199
   development cohort. **Never the 1000-series**; never append seeds. Retain every result.

## 7. Decisions requested from the user (design authority) before any implementation

1. **D1:** approve or reject completing the civilian-authority-dispute contract. If approved, decide
   its compliance and escalation rule: who may issue the order, the acceptance form and refusal
   line, the costs and three resolutions, what escalates and when, and the AI's objective.
2. **D2:** decide whether the documented .20 civilian-control guard or the current code is
   authoritative; either way, confirm a hard guard so no new authority route acts in an A6 nation.
3. **D3:** decide whether the Haiti/Nigeria opening-route review should proceed through a research
   packet before any data or policy change.

No decision is requested on D4, and no change to the A1 criterion is proposed: the docstring calls
the 0.50 literal "Ridge's to re-express", but it remains binding, and pooled, inverse-HHI or
effective-country substitutes stay unapproved.

## What this packet does not establish

- No implementation, candidate result or claim that any direction would pass A1. A1 stays blocked
  until a human-reviewed contract and passing evidence exist.
- No qualification, task closure or acceptance; canonical S25, S27, G5 and CP1 stay open.
- Only seeds 0-11 (the gate's own) were simulated; no development or 1000-series cohort was run.
  Local timings ran beside the S25 diagnostic and are not quiet.
- Historical anchors **not sourced in this repository** - Honduras 2009, Fiji 2006 and Paraguay 1996
  (a refusal without a coup) - are general knowledge from Claude and need research packets before
  informing anything. No historical identity or data was installed.

## Files and reproduction

`reproduction/` holds the local gate logs, the three identical seed tables (CI ones extracted from
the job logs, timestamps stripped), CI log tails, a Windows toolchain excerpt and the test-binary
hash. `diagnostics/` holds the harness outputs; `harness/breadth_arithmetic.py` reproduces the
arithmetic byte for byte. `harness/a1-diag/` is the read-only scratch crate (`main.rs`;
`bin/census_copy.rs`, a copy of `run_seed` plus the A10 margin; `bin/a10lead.rs`); its absolute
path dependency must point at a `39369f0` checkout (build 156 s, run 12 s). [files.json](files.json)
lists every file with bytes and SHA-256 and hashes the scratch-only binaries that were not copied.

A second automated Claude pass (not human or Codex review) re-checked the numbers, citations, CI
logs and hashes and corrected some wording; see `fact_check` in [review.json](review.json).
