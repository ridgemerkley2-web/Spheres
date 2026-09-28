# S22 stock-target repair and 2015 preparation

**Progress only; qualification is false. S22, G5 and CP1 are not completed by this packet.** Evidence is pinned separately to preparation/regressions at `594c7ce1356a9c5a393e1079e4b7abff48cf093d`, focused stock-target work at `a96bb097b78ff8e65f06f347f26cde46a190467d`, and subsequent ministry-read work at `a4e246e4399e0cc3148bdbb60b39dde3c9932b83`. The cutoff is the completed 2015 preflight at 06:06:36.952 UTC; the running 2015 diagnosis and later candidates require separate records. No earlier artifact or failed attempt is replaced.

## Tests and the remaining timing failure

The a96 candidate passed **14 company tests, one pooled-clearing equivalence test and 425 web tests**; 24 web tests were ignored. Exact commands, binary hashes and the 2026-09-28 05:49:29-05:50:15 UTC execution window are retained in [test provenance](validation/s22-stock-target-tests-provenance.json). These focused checks are not a full workspace rerun.

The support-validation repair at 594c7ce1 already passed **1,902 workspace tests across 66 suites**, with zero failures and 92 explicitly ignored tests. That separate [support-fix packet](../support-fix-validation/README.md) includes the now-passing schedule fixture; its original failure remains in [runtime-preflight](../runtime-preflight/README.md).

The [isolated 2006 stock-fix preflight](s22-native-preflight-2006-stockfix-isolated-01/runner-result.json), 05:52:43-05:53:41 UTC, **still fails**:

| Metric | Observed | Unchanged limit | Result |
|---|---:|---:|---|
| Simulation/history p95 | 291.0196 ms | 300 ms | Pass |
| Whole-turn p95 | 395.7494 ms | 400 ms | Pass |
| Whole-turn maximum | **792.7021 ms** | **750 ms** | **Fail** |
| Maximum sampled private bytes | 935,395,328 | 1,073,741,824 | Pass |
| Observed OS peak working-set bytes | 934,604,800 | 1,073,741,824 | Pass |

All 31 samples, process observations, original `passed: false`, invocation and logs are retained. The owned preparation process was suspended before measurement and resumed afterward; both [pause](s22-native-preflight-2006-stockfix-isolated-01-pause.json) and [resume](s22-native-preflight-2006-stockfix-isolated-01-resume.json) records are included. No task builds, tests, other benchmarks or fixture advancement ran during that isolated preflight. Existing unrelated user applications were not controlled. Isolation does not turn a failed preflight into qualification.

The subsequent [31-day subsystem diagnosis](s22-subsystem-2006-stockfix-isolated-01/profile.json), 05:59:59-06:00:49 UTC, matched complete instrumented and ordinary native world bytes and headlines each day. Military AI mean was approximately 16.074 ms/day. These are diagnostic stage observations; nested detail timers overlap parent timers and must not be added to them. Its equivalence pass is neither latency nor memory qualification.

## Subsequent ministry-read validation

Candidate a4e246e4 passed **426 web tests** with 25 ignored, plus one explicit actual-2015 check proving complete ministry JSON equality and unchanged world/source bytes. The [separate provenance](validation/s22-ministry-tests-provenance.json) pins this candidate, binary and actual input. This is not another full workspace run.

Its [2006 preflight](s22-native-preflight-2006-ministry-isolated-01/runner-result.json), 06:04:02-06:04:57 UTC, passes the numerical checks: simulation/history p95 **290.692 ms**, whole-turn p95 **362.8336 ms**, maximum **511.0135 ms**, private peak **916,140,032 bytes**, OS working-set peak **915,787,776 bytes**.

Its [actual 2015 preflight](s22-native-preflight-2015-ministry-isolated-01/runner-result.json), 06:05:19-06:06:36 UTC, **fails all three latency limits**: simulation/history p95 **433.455 ms** (maximum 966.6955), whole-turn p95 **545.8514 ms**, maximum **1,084.5767 ms**. Private/OS working-set peaks **1,004,343,296 / 994,193,408 bytes** pass the 1 GiB ceilings. The source inputs remained unchanged in both runs. Full raw samples and failed status are retained. The passing 2006 result cannot substitute for 2015 or either required qualification round.

## Actual preparation to 2015

[Preparation active-02](s22-preparation-active-02/runner-result.json) used the preserved 2006 input at 594c7ce1 and completed **3,287 ordinary simulation days**, reaching **1 January 2015** at 05:56:16 UTC. It used normal existing-budget renewal commands, retained history and no second competition adoption, grant, date rewrite or authored outcome. The run overlapped other work and includes the recorded pause; preparation duration is not sustained game throughput.

The current facts retain economic/military activity (31,087 economic reviews and 29,150 military reviews), six supplier plans, 27 airbases and 32 world squadrons. France remains alive; 21,975 dispatches and 1,311 history points are retained. Full starting/ending rules, workload and paid-work facts plus the preparation journal remain in [profile.json](s22-preparation-active-02/profile.json). This proves the observed lineage, not that every game feature is certified.

- Original 2006 input, unchanged: SHA-256 `7de5c6d0cb30506f9fa6024e6025d31649246e1d53c138d82d0482f447726325`, already stored in [runtime-preflight/inputs](../runtime-preflight/inputs/).
- The preparation's separately serialized resume artifact has SHA-256 `90cd908d2f67de062a7ec907c4a5d45e74d1f1651bad705799ceb495bc38084f`; it is not relabeled as the original byte identity. The profile's immutable-source and starting world fingerprints both read `f35306a59bd9541a`.
- [Actual 2015 checkpoint](inputs/s22-2015-01-01.json.gz): **130,222,370 uncompressed bytes**, SHA-256 `e786ffb67a26c6ae28bd917d5d8dc26f857e441dd6a58d60adc1a1107937ca11`. The gzip was round-trip verified; the original file was left unchanged.

Preparation `passed: true` means its requested target was reached. Its observed private/OS working-set peaks were **1,413,386,240 / 1,290,878,976 bytes**, above 1 GiB. This was a preparation process rather than a dedicated performance case, so it supplies no 2015 latency or memory pass. End-2035 preparation and both complete final-candidate qualification rounds remain open.

## Retained renderer result and integrity

Texture validation is already functionally passing in [browser-ebQ8t2](../renderer-preflight/README.md): `memory_complete: true`, context recovery and six fighter closures with zero remaining viewer buffer/texture payload. Its 99.915 FPS diagnostic overlapped native tests and does not qualify S22. Queried GL buffer sizes and API-declared texture texel payloads are separate counters; unknown layouts stay unknown/null, and neither claims physical VRAM. No limit changes accompany this clarification.

[manifest.json](manifest.json) records exact captured-file hashes, source paths, candidate/binary pins and compressed/uncompressed checkpoint hashes. Git conversion is disabled by `.gitattributes`. The package excludes binaries, duplicate input copies and intermediate annual saves; their recorded identities remain in the original preparation result. Earlier failed artifacts are retained unchanged in their original packets. No new tests or benchmarks were run to create this package.
