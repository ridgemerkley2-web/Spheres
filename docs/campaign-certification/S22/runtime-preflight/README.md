# S22 runtime preflight and diagnostic record

**This packet is preflight, not S22 qualification. S22 remains open.** It preserves successful checks, failed measurements and an interrupted preparation run without replacing them with later results. The package base is `2a247ee7e6f311d01615650b94c761af198d1c9a`; individual runs retain their own source and executable pins.

## Observed results

The frozen native limits are simulation/history p95 <= 300 ms, whole-turn p95 <= 400 ms, whole-turn maximum <= 750 ms, and both sampled private bytes and observed OS peak working set <= 1,073,741,824 bytes. Each measurement contains 31 ordinary daily samples. Display values below are rounded for readability; the JSON retains raw values and pass/fail decisions.

| Preflight | Source revision | Simulation/history p95 ms | Whole-turn p95 / max ms | Private / OS peak bytes | Numerical result |
|---|---|---:|---:|---:|---|
| `early-01` | `bb83d3b81d5e` | 301.0566 | 369.2738 / 421.2500 | 722,505,728 / 723,673,088 | Failed simulation/history limit |
| `early-isolated-01` | `bb83d3b81d5e` | 276.4936 | 341.4175 / 345.4445 | 721,829,888 / 723,542,016 | Passed this preflight's numerical checks only |
| `2006-isolated-01` | `bb83d3b81d5e` | 647.0195 | 742.3764 / 1,087.3620 | 1,736,220,672 / 1,655,029,760 | Failed all latency checks and memory ceiling |
| `2006-loadfix-01` | `4ef20c6cf852` | 609.6552 | 712.4279 / 942.5057 | 850,567,168 / 844,275,712 | Memory passes; all three latency checks still fail |

These process-memory observations include loading, immutable input retention, serialization and report work. They are neither steady-state server memory nor browser/GPU memory. Sampled private bytes can miss brief peaks; the OS working-set high-water mark is a separate metric. Hardware is recorded in [s22-hardware.json](s22-hardware.json); unrelated workstation activity is not claimed to be controlled. No result here qualifies a final frozen candidate, the full campaign lifetime or browser rendering.

The separate [2006 subsystem diagnosis](s22-subsystem-2006-01/profile.json), source `4ef20c6cf852`, preserved equivalence between instrumented daily scheduling and native advancement. `system.military_ai` consumed **278.334 ms mean per completed day** (31 observations). This identifies a performance investigation target; nested detail timers must not be summed with parent timers. Its `passed: true` is the diagnostic equivalence result, not performance qualification.

## Input lineage and reproduction

The original S19 France save is already retained as [recorded-flight-qualified.json.gz](../../S19/integration/flight-closeout-evidence/recorded-flight-qualified.json.gz), uncompressed SHA-256 `1af599ec1568a5b2148354de25cb800780d8ce69d352664f0a944242fac5ca8c`. It is not duplicated here.

Preparation at `bb83d3b81d5e93a80925dfaeda1b6ec8764f7906` applied the ordinary native `EnableEconomicCompetition { nation: France }` command on 13 May 1999. [profile.json](s22-preparation-active-01/profile.json) retains the before/after facts, command, zero price and unchanged payer balance. The adopted save was then advanced through **2,425 ordinary days** to 1 January 2006 with existing budgets renewed through the recorded normal commands. There was no date rewrite, grant or synthetic workload patch.

- [s22-adopted-input.json.gz](inputs/s22-adopted-input.json.gz): 63,066,434 uncompressed bytes; SHA-256 `fc094d539a52273a7861f942bbd04fb6ddfa42effacbf21edf1632a5fab24f27`.
- [s22-2006-01-01.json.gz](inputs/s22-2006-01-01.json.gz): 114,060,874 uncompressed bytes; SHA-256 `7de5c6d0cb30506f9fa6024e6025d31649246e1d53c138d82d0482f447726325`.

The preparation invocation targeted 30 November 2035 but was deliberately interrupted after the completed 2006 checkpoint to change the preparation-only journal. Its original `exit_code: -1` and `passed: false` remain in [runner-result.json](s22-preparation-active-01/runner-result.json), with the [interruption record](s22-preparation-active-01-interruption.json), preparation journal and partial profile. This packet does not assert completed preparation to 2035.

To reproduce a specific preflight, decompress the corresponding input, verify its uncompressed hash, and use that run's pinned source, arguments and environment from `invocation.json`. Build a release test executable from that revision; `test_binary_sha256` identifies the original local binary, which is intentionally not redistributed. The retained preparation invocation, profile, journal and S19 input identify the original-to-2006 derivation. Local absolute paths in captured records are historical provenance and must be redirected to a fresh working location. Never overwrite the archived inputs or original reports.

## Supporting validation and remaining work

[Validation logs](validation/) retain 7/7 storage checks, 2/2 company-load checks and 14/14 texture-measurement checks. They also retain the **failed** diagnostic schedule unit test: 0 passed, 1 failed. That fixture failed certified-workload validation before comparing schedules. Fixture-only fix `3c4d29f2014feefa76794f0374e81b8eda66db03` adds one ordinary enrollment day before explicit competition adoption; its rerun is pending in this packet. The failed log is not relabeled as a pass.

The operator confirmed the storage and failed schedule tests used the same `4ef20c6c` release test binary (SHA-256 `33644cdd6fce5856cad5a54c0f4ba19f1c70146986d465c49e76780f858c3065`) as the load-fix preflight. The manifest records their arguments and distinguishes this operator-supplied provenance from fields embedded in the original logs. Company and texture checks have no separately captured exact source/binary provenance and remain standalone, unqualified validation. These logs do not establish full final-candidate regression coverage.

Remaining qualification includes the repaired fixture rerun, latency fixes and measurements, later campaign checkpoints, final frozen-source native regressions, and browser renderer/performance/memory evidence. No browser traces, screenshots, binaries, raw duplicate inputs or other annual save copies are included here.

## Integrity

[manifest.json](manifest.json) lists original source paths, file sizes and SHA-256 hashes for all captured files, plus compressed and uncompressed hashes for the two saves. Captured records were copied byte-for-byte and both gzip files were round-trip verified. `.gitattributes` disables text conversion within this packet. The README and manifest are explanatory packaging metadata; original evidence has not been rewritten.
