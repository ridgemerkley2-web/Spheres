# S01 historical campaign performance evidence

This document inventories existing historical performance evidence and the shortest comparable headless measurement method. It does **not** report measurements or qualification of the active S01 candidate `5f7f355502f17bd6bd8f0383a2d14f0024fa7884`, master candidate `485c223f60d5ff6e46f6ae17164bf1ee3a8764d9`, or their eventual integration. Current-run results and release targets belong to the S01 owner and are intentionally not set here.

The historical artifacts were located and inspected read-only on 2026-09-10. No simulation, test, benchmark, game or server was run for this inventory. The USA fixture is a reusable **technical benchmark only**; it creates no requirement to develop or certify a USA campaign.

## Historical source and machine

The older workspace is `C:\Users\ridge\Documents\Codex\2026-09-04\ple`. Its location was recovered from `work/calibration-resumed/source_snapshot.json` in the active workspace. The following artifacts already exist under the older workspace:

| Relative path in older workspace | Purpose |
| --- | --- |
| `work/performance-release-final.json` | Original six-case server profile |
| `outputs/evidence/server-six-profile-results.json` | Byte-identical curated copy of the profile |
| `outputs/evidence/server-six-profile.stdout.txt` | Completed targeted-test transcript |
| `outputs/evidence/server-six-profile.stderr.txt` | Completion record for all six cases |
| `outputs/evidence/artifact-provenance.json` | Curation and revision provenance |
| `outputs/SPHERES-test-machine.json` | Reference machine and timing policy |
| `outputs/SPHERES-performance-fixtures.manifest.json` | Exact save paths, bytes, hashes, dates, rules and stress commands |
| `outputs/SPHERES-performance-fixtures.zip` | Six complete archived campaign inputs plus profile and manifest |
| `outputs/SPHERES-performance-fixtures-verification.json` | Original archive verification record |
| `work/profile-campaigns/saves/` | Existing directly usable idle inputs |
| `work/profile-stress-campaigns/saves/` | Existing prepared stress inputs |

The profile identifies version `0.6.0`, abbreviated revision `7e32a71f98db`, and mode `measure_input`. Its provenance manifest identifies the full measured revision as `7e32a71f98dbbadb618c6a91520b33c7b235881d`. Later package revision `2831c59daf33b9fd1609dbca08d92d12628250a2` is explicitly documented as a ZIP timestamp-handling change, not the measured gameplay revision.

Machine metadata was captured at `2026-09-05T06:04:16.7752217Z`: AMD Ryzen 7 9800X3D, 8 physical cores, 16 logical processors, 31.11 GiB RAM, Windows 11 Home `10.0.26200`, Rust `1.98.0 (88d9e12ae 2026-08-18)`. The recorded policy states that final request measurements ran without builds, calibration panels or world preparation. It separately labels contended phase diagnostics. The historical profile does not supply an exact measurement timestamp; its source file is dated September 4, 2026, and the archive was created at `2026-09-04T23:50:15.419938-07:00` and verified at `2026-09-05T06:50:21.435320+00:00`.

The full historical test-executable SHA256 is not identified by these server-profile artifacts. Do not substitute the daily-calibration executable hash: that is a different instrument and workload. New candidate runs should record their own exact executable hash and toolchain metadata.

## Exact idle inputs

These are the three source paths to copy into a disposable S01 fixture root, retaining its `saves` subdirectory:

```text
C:\Users\ridge\Documents\Codex\2026-09-04\ple\work\profile-campaigns\saves\profile-idle_human-0.json
C:\Users\ridge\Documents\Codex\2026-09-04\ple\work\profile-campaigns\saves\profile-idle_human-10.json
C:\Users\ridge\Documents\Codex\2026-09-04\ple\work\profile-campaigns\saves\profile-idle_human-30.json
```

All use the `spheres-campaign` envelope, schema 1, player USA and seed 1990. The filenames identify approximate campaign age, not January start dates.

| Age | Actual saved date | Bytes | Starting retained history points | Starting dispatches | Recorded source world FNV64 |
| --- | --- | ---: | ---: | ---: | --- |
| 0 | 1 Feb 1990 | 3,715,646 | 32 | 6 | `86f0352e6cb48e75` |
| 10 | 1 Feb 2000 | 39,688,734 | 1,186 | 4,618 | `bc0580f4344a038f` |
| 30 | 1 Jan 2020 | 51,282,713 | 1,315 | 33,844 | `d48d6a8f26b13e06` |

The exact idle save SHA256s below were recomputed on 2026-09-10 and match the original manifest:

| File | SHA256 |
| --- | --- |
| `profile-idle_human-0.json` | `0a900f8c7c943267bb35cd5c433c64a2afeab38e5902d1ca4383eb155621cbbf` |
| `profile-idle_human-10.json` | `5d6b022e8273c5bbd569d96cc9ac4de1f4086fa69de110f835c63718d7469b53` |
| `profile-idle_human-30.json` | `c5a61334fa8ce8d23983103a26b0a12e9d70deef60c7648077e9857b9cd31a5a` |

The source and curated profile were also independently hashed and both match `264f21415c90027d87b3e5e875ff2646af7209a7680c544769bacfe3f68ce4f5`. The fixture manifest currently hashes to `db1be68c762362fded0c4edbda98b5e2595afd90bae3bb446d6b9373752034c3`, matching its original verification record.

The original verification record reports archive SHA256 `bd38906b505bb7487f93a4fd201b08b6c1d438780a1d3408867d54ee0a09fa1e`, size 52,214,550 bytes, six fixtures and eight total members. The archive itself was not re-extracted or revalidated during this inventory; those archive-level statements remain attributed to the original verification record.

World FNV hashes fingerprint the canonical serialized simulation world and exclude presentation history. File SHA256 hashes fingerprint the exact complete campaign-file bytes, including the envelope and archive. Do not interchange them. Current loaders may migrate old data and therefore change the post-load world hash even when input bytes are identical; such changes need explanation in the new run's provenance.

## Rules and controlled stress

The recorded source rules enable daily simulation, physical logistics, logistics routes, manufacturing, production, resource market and military operations. Seed is 1990; AI aggression and crisis intensity are 1.0. Economic Competition is disabled in idle inputs and enabled in stress copies. The JSON omits some false/default flags, so this is an inventory of the recorded relevant flags, not a claim that every possible current rule existed in that historical version.

Each stress case independently loads the same aged idle save, then performs setup outside the timed loop. Setup sets USA political capital and the selected attacker's political capital to 100, enables economic AI, enacts normal priced budgets, queues a real USA Starter Industry project, and declares a legal shooting war. It grants no treasury cash, GDP, materials, completed capacity or free work progress. Exact commands, political-capital changes, prices, project state and selected attackers are preserved in the profile and fixture manifest.

The early stress war is USSR versus Iran; the middle and late cases use Iraq versus Kuwait. Every stress sample retained at least one player project and one war throughout its 31 measured days. Late stress retained two wars. Late idle had one to two wars: "idle" describes no new player orders, not a peaceful world.

This is **new industrial/war stress on an aged idle archive**, not an uninterrupted 30-year economic-AI campaign. The distinct daily-calibration panel is evidence about long-run behavior, not interchangeable request-latency evidence. In particular, its recorded concurrent workers invalidate using its wall time as an uncontended responsiveness benchmark.

Prepared stress save provenance, as recorded by the original manifest (not rehashed during this inventory):

| File under `work/profile-stress-campaigns/saves` | Bytes | Recorded measured world FNV64 | Recorded file SHA256 |
| --- | ---: | --- | --- |
| `profile-industry_and_war-0.json` | 3,720,328 | `fcc676e24e37df82` | `e84c1bb70ddc7593a2bd41ba4c5c6d844e9ddadefc4f8bebc2cdf5b9046e2289` |
| `profile-industry_and_war-10.json` | 39,693,352 | `d54a2cf34377ef3c` | `f7fccaa40f5696e275f125ad50eac1e3a9018c9e29874faa080b10af12249089` |
| `profile-industry_and_war-30.json` | 51,287,339 | `de2815af7dd958b3` | `c4be78abb64387c86782e9577a51380da44c4b9e2ff66f574ecb1c032cd98682` |

## Historical measured results

Each case contains 31 consecutive one-day requests. The harness separates simulation plus history recording, state read-model generation, state JSON serialization, and selected-country history delta query plus serialization. Whole server turn time is measured around those operations per sample; its percentiles are not sums of stage percentiles. The p95 uses nearest rank, the 30th sorted sample of 31.

| Actual starting date | Case | Simulation/history median / p95 ms | Whole server turn median / p95 / maximum ms |
| --- | --- | ---: | ---: |
| 1 Feb 1990 | Idle | 16.3 / 19.5 | 46.8 / 61.0 / 66.2 |
| 1 Feb 2000 | Idle | 89.1 / 109.4 | 149.3 / 170.5 / 177.5 |
| 1 Jan 2020 | Idle | 120.3 / 155.0 | 180.3 / 212.2 / 240.3 |
| 1 Feb 1990 | Industry/war stress | 15.3 / 79.6 | 39.8 / 111.3 / 313.8 |
| 1 Feb 2000 | Industry/war stress | 93.0 / 175.7 | 158.0 / 231.0 / 509.1 |
| 1 Jan 2020 | Industry/war stress | 158.2 / 219.6 | 217.6 / 276.3 / 574.8 |

Network transfer, browser painting and disk autosave are excluded. Checkpoint loading, controlled setup and separately queried full-history/production-room payloads are outside the per-day timed loop. These machine-specific exploratory measurements are not cross-device guarantees or an old-versus-new controlled performance comparison.

| Age | Idle largest state reply, bytes | Stress largest state reply, bytes | Idle all-country history at sample end, bytes | Stress all-country history at sample end, bytes |
| --- | ---: | ---: | ---: | ---: |
| 0 | 1,982,661 | 2,007,743 | 405,633 | 405,580 |
| 10 | 3,148,678 | 3,161,503 | 8,169,020 | 8,169,158 |
| 30 | 3,201,773 | 3,214,648 | 8,863,079 | 8,863,095 |

The targeted-test transcript reports completion in **28.99 seconds** for all six cases. That duration includes setup, loading and other work outside daily samples. It supports a short input-based rerun instead of decades of warming, but is not per-case sustained throughput. The profile only stores median/p95/maximum timing summaries, not timed-loop totals or every timing sample. Do not label `1000 / median_ms` as measured sustained days per second.

No process-memory evidence was found in this six-case evidence set. Save-file sizes, reply bytes, available machine RAM and retained history counts are **not** working-set, private-memory or peak-memory measurements. No current or historical process-RAM budget can be claimed as verified from these data.

## Harness identity across candidate revisions

The following Git blobs are identical at historical measured revision `7e32a71f98dbbadb618c6a91520b33c7b235881d`, active S01 candidate `5f7f355502f17bd6bd8f0383a2d14f0024fa7884`, and master S01 candidate `485c223f60d5ff6e46f6ae17164bf1ee3a8764d9`:

| Path | Git blob |
| --- | --- |
| `spheres-web/src/performance.rs` | `bbc0753317b9b4bf501fee2f0c43fa0393e677c5` |
| `PERFORMANCE.md` | `155aee35948df9a78b61a11dcf5055ed6c985242` |

This establishes instrument-source continuity, not gameplay equivalence, fixture-load compatibility, or performance parity. Relevant current source anchors: `performance.rs:328` configuration; `:435` age selection; `:442` archive loading; `:531` 31-sample daily loop; `:571` result fields; `:583` provenance; `:592` input-save preservation. `PERFORMANCE.md` documents the same workflow.

## Proposed candidate measurement method

1. Copy the three verified idle inputs into a dedicated disposable fixture root with a `saves` subdirectory. Verify copied file SHA256s. Leave historical originals and the user's campaign untouched.
2. Build the exact pinned candidate's release test executable before measuring. Record commit, dirty state, executable SHA256, Rust version, OS, CPU and RAM. Separate compilation from the measurement interval so compiler memory is excluded.
3. Stop competing compilation, calibration and world-preparation work. Set `SPHERES_PROFILE_INPUT` to the **absolute copied fixture root** and `SPHERES_PROFILE_OUT` to a new disposable JSON output. Unset `SPHERES_PROFILE_PREPARE_ONLY`, `SPHERES_PROFILE_PREPARE_RESUME_SLOT`, `SPHERES_PROFILE_SCENARIO` and `SPHERES_PROFILE_EXPORT_STRESS`; use the default `SPHERES_PROFILE_YEARS=30` or explicitly set 30.
4. Execute only the existing `performance::campaign_lifetime_profile` test with `--ignored --exact --nocapture --test-threads=1`. The documented Cargo invocation is below; when measuring process memory, prefer the already built test executable directly and observe that process rather than Cargo plus its children.
5. Validate successful archive loading, real saved dates, player, rules, sample counts, source and post-setup world hashes, and active workload before interpreting timing. Record migration-induced world changes. Do not treat simply reaching a runnable executable as proof of compatible fixtures.
6. Preserve new report and transcript hashes, input hashes, build identity and machine metadata together. Verify originals/copies are unchanged after the run. Interpret current-run results separately from the historical table above.

```text
cargo test --locked --release -p spheres-web performance::campaign_lifetime_profile -- --ignored --exact --nocapture --test-threads=1
```

In `INPUT` mode, both scenarios independently load `saves/profile-idle_human-{0,10,30}.json`. The harness reconstructs controlled stress using the candidate's ordinary commands; it ignores prebuilt busy slots. It performs 186 measured daily advances in total, with no interyear warming, game/server launch or input-save overwrite. `SPHERES_PROFILE_YEARS` selects ages from `[0, 10, 30]` at or below the requested last age; it is not an arbitrary single-age selector. `SPHERES_PROFILE_SCENARIO` can select one scenario if needed. A year-0 pilot alone cannot support mid/late conclusions.

For future memory evidence, sample the dedicated test process's private bytes and working set, recording the interval and sampling method. Distinguish loading peaks from steady stepping where measurement boundaries permit. A process-wide peak across all six cases must remain labeled a process-wide peak, not six separate per-age readings. The existing harness emits no phase memory markers; any finer attribution requires explicit instrumentation rather than guessed timestamps. Timed-loop totals or raw durations are similarly needed for honest per-case sustained throughput.

These old archives cover retained-history and legacy industry/war workload costs. They predate active procurement, company ammunition/refits and the master company/warfare rebuilds. Even if they load successfully, later certification needs feature-rich workload coverage in addition to this common baseline. The USA-only technical fixture must not silently expand the intended playable campaign scope.


## S01 current candidate measurements

Executed sequentially after both functional suites finished, on **SPHERES-REF-WIN-01**, 10 September 2026. These are one exploratory run per candidate, with 31 daily samples per case. The existing workstation apps were not controlled. The child executable identifies the exact clean pin; no game/server was started. All three copied input archives retained their SHA256 values.

| Candidate | Actual starting date | Case | Simulation/history p95 ms | Whole turn median / p95 / max ms |
| --- | --- | --- | ---: | ---: |
| active `5f7f355502f1` | 1 Feb 1990 | Idle archive | 27.1 | 65.9 / 84.6 / 85.6 |
| active `5f7f355502f1` | 1 Feb 2000 | Idle archive | 132.1 | 181.0 / 218.9 / 230.3 |
| active `5f7f355502f1` | 1 Jan 2020 | Idle archive | 178.7 | 218.3 / 255.2 / 258.2 |
| active `5f7f355502f1` | 1 Feb 1990 | New industry/war stress | 85.9 | 54.4 / 128.7 / 137.0 |
| active `5f7f355502f1` | 1 Feb 2000 | New industry/war stress | 194.0 | 182.7 / 265.3 / 298.2 |
| active `5f7f355502f1` | 1 Jan 2020 | New industry/war stress | 250.0 | 243.1 / 339.0 / 411.2 |
| master `485c223f60d5` | 1 Feb 1990 | Idle archive | 42.0 | 81.4 / 91.7 / 93.3 |
| master `485c223f60d5` | 1 Feb 2000 | Idle archive | 121.0 | 172.7 / 206.3 / 221.9 |
| master `485c223f60d5` | 1 Jan 2020 | Idle archive | 172.8 | 211.5 / 257.6 / 462.3 |
| master `485c223f60d5` | 1 Feb 1990 | New industry/war stress | 584.4 | 292.1 / 625.7 / 5453.9 |
| master `485c223f60d5` | 1 Feb 2000 | New industry/war stress | 3335.8 | 223.0 / 3412.6 / 9531.2 |
| master `485c223f60d5` | 1 Jan 2020 | New industry/war stress | 7626.7 | 260.2 / 7705.0 / 15527.2 |

Master is not a behavior-identical workload: loading these archives activates its operational-warfare and rebuilt-industry rules. The reports retain exact source and measured rules, priced stress commands, state hashes and activity. These differences are part of the integration risk. They do not identify the function responsible for the slower days. S02/S04 must investigate the spikes before enabling those paths; S22 owns final performance qualification.

**S01-F-PERF-M:** master's late stress p95 is 7,705 ms and maximum is 15,527 ms; active's corresponding values are 339 ms and 411 ms. Preserve this failed performance budget even though the benchmark test itself exits successfully: its assertions verify workload execution, not latency thresholds.

### Observed process memory

Counters cover the entire six-case test process, including loading and read-model queries. They are not per-age memory readings, steady-state RSS, browser memory or GPU memory. Sampling requested every 100 ms; actual gaps and inaccessible post-exit readings are recorded. The OS working-set counter is the largest accessible high-water reading.

| Candidate | Sampled private maximum | Sampled working-set maximum | Observed OS peak working set | Samples / failed samples | Test-process wall seconds |
| --- | ---: | ---: | ---: | ---: | ---: |
| active | 402.3 MiB | 404.0 MiB | 408.2 MiB | 298 / 1 | 34.21 |
| master | 413.9 MiB | 414.1 MiB | 414.1 MiB | 738 / 1 | 86.08 |

Test-process wall time includes startup, checkpoint loading, stress preparation and extra queries. It is **not** sustained simulation throughput. No days/second claim is derived from median latency.

### S01 frozen engineering targets

These targets govern subsequent integration and S22 qualification on SPHERES-REF-WIN-01. Rounded latency budgets allow modest headroom above the active aged workload. A 1 GiB headless-process ceiling allows more than twice the observed roughly 414 MiB peak for added ledgers; it is a target, not a measurement of unbuilt features. Exceeding a target creates a repair item, not an automatic threshold increase.

| Metric | Frozen target | S01 observation |
| --- | --- | --- |
| Simulation plus history recording, 31 one-day samples per case | p95 ≤300 ms | Active meets; master stress fails |
| Whole server turn (simulation, read model, serialization, selected-history delta) | p95 ≤400 ms; maximum ≤750 ms per case | Active meets; master stress fails |
| Dedicated headless process across all checkpoints | Observed OS peak working set ≤1 GiB and max sampled private bytes ≤1 GiB | Both measured runs meet; later feature-rich and 2035 cases still required |
| Representative map navigation | ≥30 FPS | Not measured in S01 |
| Ordinary local UI control latency | p95 ≤200 ms | Not measured in S01 |

Map/UI qualification uses a 1920×1080 CSS-pixel viewport, DPR 1, with both ordinary and low map detail; detailed inspection uses one released 100,000+ triangle aircraft. Also check 390-pixel layout usability and the workstation's observed 3440×1440 desktop separately. Native benchmark timing is independent of those future browser configurations. Browser/GPU memory and cold loading must be recorded separately in S22; the headless ceiling must not be presented as a total desktop-app memory result.

The legacy checkpoints stop in 2020, so “late” here means the existing 30-year archive. It does **not** establish 2035 end-state performance. S22/S25 add actual mid-2010s and end-2035 feature-rich fixtures with companies, government history, deployed forces and the certified profile, retaining the targets above.

### Reproduction and evidence

Use [run-lifetime-baseline.ps1](../../../tools/campaign/run-lifetime-baseline.ps1) with an already-built release **test** executable, copied fixture root, new evidence folder and full candidate Git SHA. It clears inherited profiling options in the child, runs only the named ignored test, checks its embedded revision and all six 31-sample rows, records logs/memory, and hashes inputs again. Existing evidence folders are refused. Parent environment variables are unchanged.

Exact binary paths/hashes, command arguments, environment, memory sampling and input hashes: [active runner](evidence/active-profile/runner-result.json), [master runner](evidence/master-profile/runner-result.json). Exact timings, rules, setup actions and state hashes: [active profile](evidence/active-profile/profile.json), [master profile](evidence/master-profile/profile.json). [hardware.json](evidence/hardware.json) records OS, toolchain, CPU, RAM, display and GPU query.

Canonical world hashes differ from the historical 7e32 run after current loaders reconstruct state; the original serialized input files themselves remain byte-identical. This is a measured legacy-load compatibility check, not proof that newer supplier/party/master save dialects migrate or that cross-branch continuations match.
