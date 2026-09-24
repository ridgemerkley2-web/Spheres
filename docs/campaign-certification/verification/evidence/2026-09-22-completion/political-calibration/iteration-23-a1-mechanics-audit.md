# Iteration 23: remaining A1 mechanics audit

Read-only review, 2026-09-22. No new simulation, coefficient sweep, source edit,
threshold change or independent-cohort inspection was performed for this audit.
The current source includes the isolated annual-plan ownership follow-up; its
closed-book planless cohort behavior is unchanged by that guard. Outcome
observations below remain bound to frozen iteration 23.

## Conclusion

No additional concrete defect explaining the remaining coup concentration was
established. The observed nine-country pool is a measured result, not proof that
all other countries are unreachable. Current evidence supports an unresolved
campaign-distribution/design limitation, not another justified parameter-only
repair. Do not label A1 passed or broaden its cohort until it passes.

The separate annual-plan ownership defect is real: the original-23 compiled
library deletes an existing explicit plan through public government::tick. Its
one-condition correction is recorded separately. These calibration worlds start
without that partial-save shape, so it is not an explanation of A1.

## Evidence checked

- Original A1 remains 7 coups / .59 median top-three share. Fixed development
  N200 remains 7 / .57. The pooled 1,498 elected coups occur in Guatemala 327,
  Sao Tome 276, Myanmar 232, Guyana 222, Ecuador 200, Comoros 199, Mozambique 26,
  Chad 9 and Cambodia 7. Pooled .5574 is not the per-seed median assertion.
- bloc_census.rs:377-412 increments each elected-coup count and its country
  numerator together, separate from annulments and internal regime coups.
  Lines 652-659 sort the same counts and divide by the same elected-coup total.
  Lines 985-992 apply the original 12-seed median and strict below-.5 share.
  No route-classification or denominator mismatch was found.
- government.rs:8862-8887 walks actual Army/Security targets and accumulates
  pressure only during current low effective Army loyalty and discontent.
  Lines 8928-8959 require both live conditions again, adequate observed tenure,
  no pending first election and actual accumulated pressure before the break.
  Old pressure alone cannot remove a recovered government.
- government.rs:9294-9338 clears coup pressure, election authority and pending
  deadlines, resets office age, restores the mover's loyalty to .90 and other
  pillars to .72, and seats the actual institutional successor. The regime
  branch then requires its own 36-month tenure and current weak institution
  (9167-9193). No stale-pressure immediate repeat was found.
- government.rs:8234-8238 and 8319 onward now distinguish an actual party-led
  executive from a dormant cabinet when choosing a voluntary round table.
  A newly installed strong Army does not inherit a civilian party's willingness
  to surrender office. The separate weak-armed route and explicit player
  command are intentionally preserved. This correction was already measured
  in iteration 23; residual repeats do not prove it failed to execute.

## Funding, confidence and eligibility links

- Personnel and annual-dollar resource units feed the same Army target used by
  the funding inverse. Invalid/missing inputs have explicit guarded fallbacks;
  absence is not silently interpreted as zero resources. Population-reference
  and sourced/estimated distinctions were covered by prior focused checks.
- government.rs:9353-9380 solves for raw Army loyalty .40 using current war and
  civilian-confidence costs, bounded by useful full resources, the existing
  nondeficit budget and share cap. AI then uses the actual paid SetMilSpend
  command at month end (9386-9409). It does not buy instant loyalty.
- Loyalty losses blend at .10 and gains at .045 (8773-8785). Thus a previous
  raw loyalty at or above .40 cannot fall below .35 in one ordinary monthly
  walk toward a nonnegative target (.40 * .90 = .36). The existing month-end
  emergency review is therefore not intrinsically one tick too late for that
  material/confidence path. This does not prove every nation can afford repair,
  nor cover a separate abrupt foreign-backing penalty.
- The older iteration-20 trace found 103/103 actual coups had equal raw and
  effective Army loyalty; all eleven apparently available upward allocations
  had already been raised in the preceding month. Those observations rebut
  that older alleged missed-funding defect, but are not presented as fresh
  iteration-23 trajectories.
- Known executive-removal leverage enters only the sustained confidence and
  prospective programme-veto channels; it is not an unconditional coup chance.
  Confidence additionally requires a matching six-month party record and
  discontent >= .25 (8507-8579). Quiet high-leverage countries are deliberately
  protected. Their lack of coups alone cannot demonstrate a broken hook.
- At full resources, no war, leverage .8, sustained crisis .75 and mandate .25,
  selected weight .65 leaves target .5575. Political disloyalty is therefore
  possible only under stronger applicable inputs or less funding in that
  example. This is an explicit model design limitation, not arithmetic or
  missing wiring. The separate weight-1.20 candidate was tested and rejected
  under joint criteria; its failures remain authoritative.
- A real Army seizure sets saved current leverage to 1 while keeping its
  historical assessment immutable. That is an explicit installed-regime
  transition assumption. V-Dem's lagged non-force-removal assessment alone
  cannot prove either this exact maximum or an automatic later erosion rate.
  No independent factual defect justifies changing it to manipulate repeats.

## Smallest optional next diagnostic

If another diagnostic is authorized, use one already-developed seed and the
frozen current binary, recording only first/repeat elected-coup transitions
and the nearest rejected trigger for a predeclared set of high-leverage
countries. Include actual programme/office, pending/settled record, D, raw and
effective loyalty, each target term, pressure, current leverage/mandate,
personnel source, fiscal affordable/useful floor, PC and last paid allocation.
This would distinguish unaffordable recurrent material crises from quiet or
politically protected near misses without altering the simulation. Existing
iteration-23 logs truncate each seed's country names at five, so they cannot
supply exact first/repeat counts. No such extra run has been started.
