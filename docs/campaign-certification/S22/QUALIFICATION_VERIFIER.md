# S22 qualification freeze and verdict

`tools/campaign/s22-qualification.cjs` declares one complete, immutable pair of
qualification rounds **before** any measurement. It does not launch benchmarks,
award S22 completion, or promote a diagnostic/preflight. The existing
`measurement-protocol.json` remains the performance authority.

The helper requires exactly 18 cells: the three actual dated France inputs ×
native/map/renderer × initial qualification/second confirmation. Every cell has a
unique ID and an absolute output root reserved before launch. An unchanged-build
retry also needs a new complete pair with new roots. The shared append-only
registry must be reused across every pair; do not substitute an empty registry.

## Prepare the reviewed inputs

Create a configuration JSON with this shape. Paths are absolute existing files,
except the 18 output roots, which must not exist yet. Replace the illustrative
values with actual recorded values; this example is not evidence.

```json
{
  "schema": "spheres-s22-qualification-config/v1",
  "attempt_id": "s22-qualification-pair-01",
  "repository": "C:/absolute/integration",
  "candidate_revision": "FULL_40_CHARACTER_COMMITTED_RUNTIME_REVISION",
  "protocol_path": "C:/absolute/integration/docs/campaign-certification/S22/measurement-protocol.json",
  "hardware_path": "C:/absolute/evidence/current-hardware.json",
  "source_path": "C:/absolute/evidence/original-s19-source.json",
  "provenance_paths": ["C:/absolute/evidence/actual-adoption-and-preparation-report.json"],
  "lineage_path": "C:/absolute/evidence/reviewed-lineage.json",
  "binaries": {
    "native_test": "C:/absolute/release/deps/actual-native-test.exe",
    "server": "C:/absolute/release/spheres-web.exe"
  },
  "cases": [
    {"id":"early-france-1999","input_path":"C:/absolute/evidence/adopted-1999-05-13.json"},
    {"id":"mid-france-2015","input_path":"C:/absolute/evidence/actual-2015-01-01.json"},
    {"id":"late-france-2035","input_path":"C:/absolute/evidence/actual-2035-11-30.json"}
  ],
  "registry_path": "C:/absolute/evidence/s22-qualification-attempt-index.jsonl",
  "cells": []
}
```

Fill `cells` with every combination, in the intended execution order:

```js
const rounds = ['initial_full_qualification', 'second_full_confirmation'];
const cases = ['early-france-1999', 'mid-france-2015', 'late-france-2035'];
const roles = ['native', 'map', 'renderer'];
config.cells = rounds.flatMap((round, r) => cases.flatMap(c => roles.map(role => ({
  id: `r${r+1}-${c}-${role}`, round, case: c, role,
  output_path: `${absoluteNewPairDirectory}/r${r+1}-${c}-${role}`
}))));
```

The hardware JSON records `reference_id: "SPHERES-REF-WIN-01"`, actual `cpu`,
`gpu`, `physical_ram_bytes`, `os` including build, `gpu_driver`,
`power_background_observations`, and
`browser: {version, channel: "msedge", headless: true, extra_flags: []}`.
The source must hash to the protocol's preserved earned S19 source. Each dated
input must be an actual uncompressed campaign envelope at the specified date.
The helper reads the nested saved calendar rather than trusting the filename.

The reviewed lineage JSON has this schema:

```json
{
  "schema": "spheres-s22-reviewed-lineage/v1",
  "status": "reviewed",
  "reviewed_by": "ACTUAL reviewer identity",
  "reviewed_utc": "ACTUAL review UTC timestamp",
  "review_scope": "Describe the original command/checkpoint evidence reviewed and the limits of that review.",
  "original_source_sha256": "ACTUAL original SHA256",
  "adopted_early_sha256": "ACTUAL adopted early SHA256",
  "inputs": [
    {"id":"early-france-1999","actual_date":"1999-05-13","sha256":"ACTUAL SHA256"},
    {"id":"mid-france-2015","actual_date":"2015-01-01","sha256":"ACTUAL SHA256"},
    {"id":"late-france-2035","actual_date":"2035-11-30","sha256":"ACTUAL SHA256"}
  ],
  "links": [
    {
      "kind":"ordinary_competition_adoption",
      "from_sha256":"ACTUAL original SHA256",
      "to_sha256":"ACTUAL adopted early SHA256",
      "from_date":"1999-05-13", "to_date":"1999-05-13",
      "command":"EnableEconomicCompetition", "price_pc":0,
      "outcome":"completed",
      "evidence":[{"path":"C:/absolute/actual-adoption-report.json","sha256":"ACTUAL report SHA256"}]
    }
  ]
}
```

Append actual preparation links in reachable order, including retained 2006
checkpoint and later resumed runs. Preparation kinds are `native_preparation`
and `native_preparation_resume`; each has the same parent/child hash/date/evidence
fields. The allowed outcomes are `completed` and `checkpoint_retained`. The
latter preserves an interrupted preparation's valid earlier checkpoint without
pretending the interrupted full target passed. Every link's evidence file must
appear in `provenance_paths` with matching bytes. Native initial resume-save
outputs may differ from their input because of save-envelope serialization;
retain and explain them, but use the runner's actual source hash as the parent.

This verifies reviewed, immutable hash/date linkage and its completeness. It is
**not an independent replay of the intervening years** or automatic proof that a
manually asserted historical narrative is true. An actual reviewer must inspect
the retained native commands, input identities, checkpoint facts and preparation
reports before setting `status: "reviewed"`. An incomplete or unreviewed chain
cannot freeze.

## Freeze, run, verify

After the candidate, both executables, all three inputs, browser harness and
reviewed provenance are ready and committed:

```powershell
node tools/campaign/s22-qualification.cjs freeze --config C:/absolute/pair-config.json --manifest C:/absolute/pair-manifest.json
```

The helper hashes the protocol/config/hardware/provenance, original source,
three inputs, binaries, served model/map assets, and all measurement/probe/helper
dependencies. It pins runtime Git trees, the exact work distribution and complete
map presets. The reference viewport is 1920×1080 DPR1; its existing game layout
has a 1920×792 CSS/backing map rectangle. Both dimensions are frozen; a smaller
canvas cannot silently pass as the declared workload. Registry reservations and
manifest creation refuse overwrite. A stale `.lock` is evidence of an interrupted
reservation; investigate it, do not automatically remove or reuse its paths.

Run one declared cell at a time without compilation, preparation or other timing
lanes. Native cells use `run-s22-profile.ps1 -Mode measure -RenewBudget
-RequireCertified`, the declared native binary/revision/input SHA, and exact
`-EvidenceRoot`. Browser cells use the declared server binary/revision/input SHA,
`SPHERES_S22_QUALIFY=1`, and the exact declared parent `SPHERES_S22_OUTPUT`.
Set `SPHERES_S22_RENDERERS=1` only for renderer cells. A browser parent must contain
exactly one generated `browser-*` directory; multiple attempts invalidate it.
Complete all nine initial cells before beginning the nine confirmation cells.

```powershell
node tools/campaign/s22-qualification.cjs verify --manifest C:/absolute/pair-manifest.json --out C:/absolute/verdict-01.json
```

`--out` is optional, and also refuses overwrite. Exit status is nonzero for
`incomplete`, `failed`, or `invalid_identity`; only both complete passing rounds
return success. Missing outputs are reported as incomplete. Existing failed or
contradictory outputs are retained and reported as failed. Documentation-only
commits may advance HEAD when all runtime trees and frozen artifact bytes remain
identical. Changed runtime, binaries, harness, configuration, hardware records or
inputs invalidate the entire pair.

Raw checks include 31 consecutive certified native samples, nearest-rank p95,
whole-turn maximum, all timed stages, actual independently reloaded batch
equivalence/throughput, positive memory CSV readings, both 1 GiB limits, and
reported sample gaps. CSV timestamps use F3 precision; the consistency check
allows only that representation's rounding, never a performance-limit tolerance.
The OS peak may be higher than CSV rows when a failed final observation obtained
the peak but lacked valid private/working bytes; the higher observed peak remains
subject to the same limit and is retained in the verdict.

Browser checks independently validate eight map windows, all 62 ordered trusted
controls, actual draw counts/full elapsed intervals, complete presets and canvas
size, 100k–250k inspection mesh observations, the unchanged raw 60 FPS target,
layout/focus/touch/scroll evidence, trace content/hash, unchanged full saved state,
shader/accounting observations, cache bounds and disposal. Browser process, JS
heap and GL payload observations stay separate; neither a driver VRAM estimate
nor the native memory ceiling is applied to browser observations.

The unit suite uses explicitly synthetic tiny fixtures. It neither creates nor
accepts campaign qualification evidence:

```powershell
node --test tools/campaign/check_s22_qualification.cjs
```
