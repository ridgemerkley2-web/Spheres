# S22 remaining native latency work

**Progress evidence only; qualification: false. S22, G5 and CP1 remain open.**
This append-only packet follows the [697 validation and isolated preflights](../pass-local-progress/README.md).
That candidate passed 2015 and missed only end-2035 simulation/history p95:
302.2232 ms against the unchanged 300 ms limit. That failed attempt remains
unchanged; no rounded or substituted pass is claimed.

## Completed 697 diagnosis

The actual end-2035 diagnosis at `697448c160a53cadd8b6132717a0df22ed430eb8`
completed all 31 ordinary native days. Every instrumented day matches the
ordinary engine's complete world and returned headlines; original and copied
input hashes remain `67aadca2f55280abc0e4ad97944a654ec7d173ffb65cb7d71dd59090e10503cb`.
The final world fingerprint is `3c3f21d9d67bd7b8`.

Its raw profile, native log and provenance are retained with exact byte hashes.
The source checkpoint is referenced by original path and SHA-256, not copied
again. Detail timers overlap parent timers and cannot be added together. This
diagnosis overlapped ordinary task work and is neither latency acceptance nor
a memory measurement.

## Source-reviewed candidate

The combined candidate is `d50f7ee1a3440b00c8968b530e8e8b45aa0899fd`.
It avoids quotes for ground ammunition rows the existing consuming loop never
uses, avoids copying complete arrival routes when crediting three scalar
fields, and indexes incoming population course identities within a single
merge. Original quote/refusal behavior, live reads, arrival guards, first-match
identity, append order and floating-point addition order are preserved by the
reviewed source changes.

The [independent source review](reviews/s22-margin-source-reviews.md) identifies
all author revisions and states its limitations. The population author's
[proposal and workload counts](reviews/s22-population-course-merge-proposal.md)
are a read-only snapshot inspection. Their candidate counts and upper bounds
are not exact next-day work or measured savings.

## Completed compilation and focused checks

The combined release workspace compilation and web build completed at
`2026-09-28T09:42:14.8750919Z`, exit zero. The command first used
`cargo test --locked --release --workspace --no-run`, then
`cargo build --locked --release -p spheres-web`. Immutable runtime provenance
retains exact binary hashes and byte counts; executable bodies are excluded.

Four focused groups then passed **seven unique tests, zero failures, three
ignored**: two catalogue comparisons, the existing paid ground-ammunition
purchase/retry test, two arrival-payload tests and two course-merge tests.
The explicit actual-campaign oracles are separate from these ignored entries.
All commands, final logs and provenance are retained.

## Completed actual-input comparisons

All six explicitly invoked actual-2015/end-2035 comparisons passed, each with
31 complete native worlds and returned/retained headlines equal to the original
path. Every source hash remains unchanged. The retained logs record nonzero
work removed by each path:

| Family | 2015 | End-2035 |
| --- | ---: | ---: |
| Ground ammunition reviews / omitted quotes | 16 / 215 | 40 / 897 |
| Avoided Cargo / route-node copies | 5,038 / 114,857 | 3,347 / 68,485 |
| Avoided route/hold string content bytes | 4,776,466 | 2,852,122 |
| Original linear row comparisons / destination index probes | 434,594,145 / 6,577,268 | 538,378,328 / 7,332,887 |

These counters show that each oracle exercises the changed path. String byte
counts measure content, not allocator or resident memory. The population counter
uses the original first-match position to count linear comparisons; index probes
count visited destination rows, not every map lookup or total CPU operation.
The comparisons overlapped regression work, so their durations are not latency
acceptance measurements.

## Completed release regression and isolated preflights

The full release workspace completed at `2026-09-28T09:50:51.7277079Z`, exit
zero: **1,954 passed, zero failed, 108 ignored across 66 suites**. The exact
command was `cargo test --locked --release --workspace -- --test-threads=4`.
The final log, provenance and successful candidate Git push are retained.

After regression and agent file work stopped, both isolated native preflights
passed all unchanged limits:

| Actual input | Simulation/history p95 | Whole-turn p95 | Whole-turn max | Private / OS peak bytes | Result |
| --- | ---: | ---: | ---: | ---: | --- |
| January 2015 | 260.5369 ms | 316.5608 ms | 478.3212 ms | 996,524,032 / 858,603,520 | Pass |
| November 2035 | 240.6596 ms | 312.8513 ms | 433.3893 ms | 905,084,928 / 897,794,048 | Pass |

Each case retains the complete 31-day profile, independent 31-day batch,
invocation, memory CSV, runner result, stdout/stderr and console log. Both
original/copy input hashes are unchanged. Final world fingerprints remain
`5c486d0c186f921e` (2015) and `3c3f21d9d67bd7b8` (2035), matching prior runs.
The source saves are referenced by absolute path and SHA-256; no save bodies
are duplicated. Sampled private and observed OS memory are separate conservative
whole-process observations, not steady-state server or GPU memory.

These successful preflights are not cells in a declared qualification pair.
All 18 cells across both native/browser rounds remain required on the reviewed
inputs and unchanged candidate. At this record's close, that pair has not
started. S22, G5 and CP1 remain open. No limit, schedule, feature workload or
original input was changed to claim success, and all earlier failed attempts
remain preserved in their original packets.
