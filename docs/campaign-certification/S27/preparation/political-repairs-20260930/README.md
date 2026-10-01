# Political correctness repairs — A1 remains blocked

Owner: Codex. Requested 30 September 2026. Integration base `02d2c5a2`;
tested repair `7c6f112c`. The user's request was to fix the political issue.
Two demonstrated defects are corrected, but **the A1 concentration gate still
fails**. This packet does not complete `CODEX-S27-A1-01`, S27 or CP1.

## What changed and why

1. **A hostile armed institution can no longer disappear inside an average.**
   Regime coup pressure reads the weakest armed institution, while the AI's
   negotiation preference previously read their mean. The extended, test-only
   seed-0 observer found 750 country-months across 11 countries with positive
   pressure, a currently hostile armed institution, a legal affordable round
   table and no selected lever. Other loyal institutions kept the mean above
   0.50. The AI now considers the same live threat as the coup mechanism.
   Original action priorities, legality, political-capital price, first-ballot
   delay and ordinary action draw remain in force. This is a justified AI
   preference correction, not a new historical fact. Haiti has no such hostile
   institution in this trace and is not repaired by this change.
2. **Every recorded armed institution receives one loyalty update.** Sudan has
   two legitimate Army entries: the Sudanese Armed Forces and Popular Defence
   Forces. The old target loop found the first entry twice, updating it twice
   and freezing the second. With the political lens enabled, each stored
   institution is now updated once. Neither sourced institution is deleted and
   loading preserves the recorded save. The pre-lens loop remains unchanged to
   preserve its recorded legacy replays.

The four opening regressions were built and run on the old runtime first:
two failed, two passed. After the correction, all four and the existing
government/stratagem tests pass. The separate institution regression also
failed first (`0.667595` versus the single-institution control's `0.659`). It now
passes daily and monthly checks with both equal and unequal starting loyalties,
using the public government tick and a save/load round trip.

The observer itself passes 252 exact complete-save/RNG/headline comparisons,
with 185,260 snapshots and ten actual elected-government coups. Its 252 control
digests also agree with the earlier seed-0 observer. It adds no release API or
simulation behavior. [Observer result](observer-result.json),
[opening witnesses](opening-witnesses.json), [full compact analysis](opening-analysis.json).

## Unchanged political outcomes

The original A1 cohort is seeds 0–11, 252 legacy monthly ticks. It requires
median elected-government coups in 4–14 and a **strict median per-seed top-three
share below 0.50**. Starting datasets, `bloc_census.rs`, thresholds and all other
original cohorts are unchanged. A6 retains its 420-month horizon. No reserved
1000-series cohort or new wider scan was used.

| Source | Median elected coups | Median top-three share | Original outcome gates |
|---|---:|---:|---|
| Prior baseline | 7 | 0.585714 | A1 fails; A2–A10 pass |
| Opening correction `434abd50` | 7.5 | 0.571429 | A1 fails; A2–A10 pass |
| Both corrections `7c6f112c` | 7.5 | 0.571429 | A1 fails; A2–A10 pass |
| Rejected urgent response `75af467f` | 12 | 0.54 displayed | A1 and A2 fail |

Attribution passes in each new candidate. The accepted repair has A2 at 8/12,
A8 at 40/40 and A9 at 168/168; the [unchanged gate log](institutions-gates.log)
retains every result. The [original 12-seed diagnostic](institutions-distribution.log)
counts 87 elected-government coups in eight countries: Guatemala 21; Myanmar
and Sao Tome and Principe 14 each; Ecuador and Guyana 12 each; Comoros 10; Chad
and Mozambique two each. These pooled counts are diagnostic, not the gate's
per-seed statistic.

The additional urgent-response policy executed an already selected threatened
negotiation immediately. It passed its two focused tests, but A1 still failed
and A2 fell to 6/12, below its strict-majority bar. It was rejected under its
[predeclared plan](urgent-plan.json). Its [exact patch](rejected-urgent-response.patch),
before/after tests, build and gate logs are preserved; it is not in the accepted
runtime. No additional coefficient, timing or country-filter trial follows.

## Verification and limits

- Full release workspace: **1,990 passed, zero failed, 115 ignored**, with the
  one absolute resource timing test deliberately filtered out. This includes
  deterministic replay, save/load, daily simulation and existing golden checks.
  Ignored political outcome tests were exercised separately as recorded above.
- UI full run: 1,796 passed, two failed, one skipped. Both failures came from
  a missing unchanged art-roadmap fixture in this sparse checkout. Restoring
  `docs/art` and rerunning both affected modules gave six and three passes.
  All non-skipped coverage is green across those runs; the original failed
  full-run log remains preserved. No UI or test implementation was changed.
- Release `spheres-web` build passed; existing vendored warnings remain.
- **Absolute resource timing remains pending.** Another development task was
  compiling/running native tests, so no uncontended 0.15 ms/month measurement
  is claimed. Run the exact standalone check from CONTRIBUTING when the machine
  is free of other CPU-heavy work, before calling full verification complete.
- No browser journey, new long campaign or release qualification is claimed.
  The frozen S25 matrix, its source, launcher and saves were not changed.

[Validation receipt](validation.json) binds exact commits, source trees,
working/Git hashes, frozen executable hashes, commands and outcomes. All builds
used private target directories; source remained stable through final workspace
verification. A failed D-drive build ran no tests and was superseded by fresh
builds on C; [storage preflight](storage-preflight.json) retains that boundary.
The institution test's initial import-visibility compile error is also retained
separately from its actual failing regression.

To reproduce on the recorded runtime, use a private `CARGO_TARGET_DIR`, then
run the verification commands in CONTRIBUTING and the ignored assertion suite:

```powershell
cargo test --locked --release -p spheres-sim --test bloc_census -- --ignored --nocapture --test-threads=2 --skip bloc_census
$env:SPHERES_CENSUS_SEEDS='12'
$env:SPHERES_CENSUS_MONTHS='252'
$env:SPHERES_CENSUS_SEED_START='0'
cargo test --locked --release -p spheres-sim --test bloc_census -- --ignored --exact bloc_census --nocapture
```

The 33 MB compressed full observer trace and frozen executables remain at
`C:/Users/ridge/spheres-political-repair-backup-20260930` (trace under
`retained-observation/opening-baseline`). [Baseline manifest](baseline-manifest.json)
pins compressed and decoded bytes; every retained file hash was rechecked.
The full trace is local evidence, not a remote Git artifact. Compact witnesses,
native results and failure logs are in this packet. [Evidence manifest](manifest.json)
pins packet files.

## Next checkpoint

Complete the uncontended resource timing check, then review the residual
concentration against the existing causal and historical contracts. Eight
countries still dominate the elected-coup outcomes; a weak armed institution,
an eligible crisis and access to elections are distinct requirements. The
observed negotiation defects do not establish a missing trigger in other
countries. Any further mechanism needs its own justified contract, a failing
regression and the same original gates. Preserve the rejected urgent policy;
do not rescue it with post hoc country exceptions or altered acceptance limits.
