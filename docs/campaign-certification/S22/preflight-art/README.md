# S22 offline art preflight

**PASS** at `846df4797712935efb5a221e188d410b09f0d083`.
The repository was clean before and after the audit. The 39 source, contract,
generated-record and canonical-export hashes in `result.json` are unchanged.
No runtime, generated record, budget or contract was edited.

The audit ran from **2026-09-28 04:31:20.204 UTC to 04:35:02.593 UTC**.
These are execution windows for avoiding contention with the independent browser
performance run; their durations are not game-performance results. No browser
or WebGL performance tool was launched by this audit.

| Command | Result | Evidence |
| --- | --- | --- |
| `node tools/ui/bench_art.cjs --check` | Exit 0; 245 configurations, 0 current ceiling failures, 0 required quality-floor failures, 0 export overruns; generated records current | `budget-and-records.log` |
| `node tools/ui/build_art_manifest.cjs --check` | Exit 0; 33-asset manifest current | `manifest.log` |
| `node tools/ui/build_equipment_models.cjs --check` | Exit 0; 13 GLBs reproduce byte for byte, README matches after permitted checkout newline normalization | `canonical-exports.log` |
| Sequential Node test run of nine accounting, contract, quality and gallery files | Exit 0; 35 Node test entries pass, no failures or skips; two entries also report 32 equipment-mesh and 5 fighter-mesh internal checks | `focused-contract-accounting-quality.log` |
| `node ../evidence/s22-preflight-art/summarize.cjs` | Exact contract scalars and the actual 13-file GLB directory set independently match the expected inventory | `findings.log`, `findings.json` |

`result.json` records executable, Node version, exact command arguments,
timestamps, exits, source hashes and log hashes. `audit.cjs` reproduces those
four main jobs. `--check` already regenerates both measurement records in memory
and compares them; a separate `--check-records` run would not add a budget test.

## Current limits and retained older findings

The measured current contract is revision 2, established on 22 September in
`f40f9216452f446db460ee3ca0622163d8123e7a`; its original patch is retained as
`contract-reconciliation.patch`. It reconciles the earlier inspection proposals
with pre-existing detailed-model quality and export requirements. This audit did
not introduce that change or widen any limit.

The 245 graded configurations comprise **166 passes and 79 advisory density
notes**. The latter are ordinary lower-density guidance, not missed mandatory
floors. All three aircraft inspection baselines meet the required 100,000 minimum:
fighter 200,446, light attack 199,326 and tactical strike 228,640 triangles.
Each remains below the existing 250,000 ceiling. Individual buildings retain
12,000 close / 800 map ceilings; catalogue/map vehicles retain 12,000 / 1,500;
complete scenes retain the **150,000 submitted-triangle ceiling**.

**33 comparisons still exceed the superseded original proposal.** They remain
in the measured record and `findings.json`. No claim is made that geometry was
removed simply because its current-contract classification differs.

Two raw full-close town blocks still exceed 150,000:

| Block | Raw full-close triangles | Current measured camera submissions | Headroom |
| --- | ---: | ---: | ---: |
| Mixed | 163,671 | 149,997 | 3 |
| Residential | 174,540 | 149,934 | 66 |

The gallery's adaptive scene planner lowers/culls appropriate lots while keeping
the selected building at close detail. Its original scene ceiling is unchanged.
The tight headroom is real and should remain guarded; these are current passes,
not raw-mesh reductions. `town-scope-contract.patch` retains the historical scope
change. The source tests reject unrenderable backgrounds and incomplete triangle
accounting rather than removing their cost from the count.

## Accounting and remaining qualification scope

No current unknown-layout or constituent-accounting failure was found. The
focused checks prove that CPU material tags are counted separately, shared
backing allocations are not double counted, unknown typed-array layouts fail,
and every physical-building/shared-envelope range has valid ownership and
complete non-overlapping coverage. Canonical GLBs are also decoded and checked
for geometry, material data and semantic specification ranges.

The offline figures intentionally exclude derived renderer attributes,
floor/shadow/texture resources, driver overhead and general application heap.
The 30-asset ground/site/town inventory totals 185,348,952 base-attribute bytes
and 186,888,072 CPU backing bytes, but no frame draws that full inventory.
Aircraft appear in their own detail table and are not part of that total.
This is explicit scope, not a live memory estimate.

The adaptive **TownMesh gallery** is separate from the campaign globe's
**CityMesh** path. Whole-campaign city residency, actual submitted geometry,
context/cache recovery, frame rate, control latency, loading and day throughput
remain for S22's independent runtime work. Ground component sweeps are greedy
and aircraft detail rows are baselines; they are not exhaustive proofs of every
possible configured model.

**Required fixes from this offline audit: none.** Preserve the existing guards
and complete runtime qualification using actual browser observations.
