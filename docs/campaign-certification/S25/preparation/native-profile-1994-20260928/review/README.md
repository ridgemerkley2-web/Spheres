# Independent review: actual France, January 1994

Reviewer: Codex `/root/s20_preflight`. Reviewed on 2026-09-28. This packet is an offline review of the existing diagnostic, not a new native execution, performance qualification, full stability matrix result, or campaign-policy change.

**Finding:** the retained 31-day diagnostic passes its stated equality checks. The measured native ticks are relatively small at this checkpoint; most diagnostic wall time is verification and other work outside simulation clocks. No specific production algorithm defect or outcome-preserving production patch is established by this trace alone.

## Provenance and equality

The source revision is `ae8084e853a8eb01ef4d834017af3e94c8e2cc82`; the profile embeds its `ae8084e853a8` prefix. Independently streamed SHA-256 hashes match the execution receipt's before/after pins:

| Retained input | Bytes | SHA-256 |
| --- | ---: | --- |
| Frozen `spheres-web-test.exe` | 353,133,206 | `e90b8f0cb34603f5290c8a19b106a0d6dc31e3ea7ad293eb0135199fe1fd59e2` |
| Original interrupted France monthly checkpoint and diagnostic `input.json` copy | 135,739,703 each | `5480ccc296c2fe67ef2234871d1ef5fc9f098184ca1c83d885b07010a1368b20` |

The raw process receipt and profiler source bind the declared build identity; this review does not rebuild or independently reproduce the executable. Exact Git blobs from that revision are retained under `pinned-source/`. The original checkpoint and executable remain external at the absolute paths in `analysis.json`; they are not duplicated here.

All 31 rows independently checked: sequential indices, real dates from 1 January through 1 February 1994, exact native-world serialization equality, equal serialized lengths, exact returned-headline equality, null first-difference positions and validation errors, and continuous history/log row bookkeeping. All 31 stderr lines report equality; stdout reports one passed test and no failures. Profile summaries recompute from their raw values using the native nearest-rank p95 and upper-middle median definitions.

France remains alive with seed 1990 and required active capabilities. The first day applies the recorded ordinary zero-PC annual budget renewal for 1994; later days contain no player commands. No competition adoption, authored grants or redating occurs. Month-end history compaction is preserved: 1,140 to 1,111 history rows and epoch 12 to 13. An initial reviewer assumption of monotonic history row counts was corrected; that offline script failure is retained in `review-attempt-01.txt` and was not a native failure.

The equality scope is precise: the observed world is compared with the ordinary `Game` world by full `spheres_sim::save` bytes, and observed headlines with the new native log entries. The observer is a `WorldState`, not a second complete `Game`, so this is **not** independent equality of two complete campaign history/log/journey envelopes. That broader contract belongs to the separate S25 paired campaign harness.

## Simulation timings

The observed schedule executes before the ordinary native tick each day, using independent persistent route pools. These are sequential diagnostic measurements, not independent cold/uncontended qualification cells. Native tick timing includes actual tick, log and history work; subtracting the two timings does not isolate any of those components.

| Native day group | Days | Mean ms | Median ms | p95 ms | Max ms |
| --- | ---: | ---: | ---: | ---: | ---: |
| Ordinary 2–30 January | 29 | 87.340 | 87.580 | 93.560 | 96.379 |
| Annual 1 January | 1 | 166.406 | — | — | 166.406 |
| Month-end 31 January | 1 | 180.826 | — | — | 180.826 |

The observed schedule's corresponding means are 89.154, 177.190 and 199.889 ms. Annual command preflight adds 42.257 ms outside both tick clocks. The economic-AI stage is 61.895 ms on 1 January and 82.219 ms on 31 January; its ordinary-day median is about 0.012 ms. Month-end politics is 21.316 ms. Thus the two larger days are normal periodic work, not a recurring population-course merge spike.

Top-level timings below are **exclusive of one another** across the actual ordered schedule. `prelude.arsenal_spot_market` is a separate timed operation before the normal arsenal system. Values average all 31 days; `analysis.json` contains every stage and raw-day grouping.

| Exclusive stage | Mean ms/day | p95 ms | Total ms |
| --- | ---: | ---: | ---: |
| Arsenal spot market | 37.715 | 49.451 | 1,169.165 |
| Population | 15.231 | 16.785 | 472.163 |
| Resources | 12.836 | 14.159 | 397.921 |
| Politics | 5.013 | 5.543 | 155.391 |
| Economic AI | 4.660 | 61.895 | 144.469 |
| Industry | 4.086 | 5.162 | 126.669 |
| Province finish | 2.877 | 3.098 | 89.173 |
| Province begin | 2.418 | 2.768 | 74.961 |

All exclusive stages sum to 2,962.339 ms versus 2,962.536 ms for the enclosing instrumented clock; approximately 0.198 ms over the whole run is outside the stage boundaries.

Nested attribution is reported separately and **must not be added to those parent values or indiscriminately to other nested values**:

- Within resources (12.836 ms/day), `post_market_flows` is 12.300 ms, including `begin_raw_freight` at 9.974 ms.
- Within the market (37.715 ms/day), `orders_and_dispatch` averages 9.298 ms for iron, 9.293 for bauxite and 8.985 for coal.
- Within each corresponding orders/dispatch scope, `dispatch_plan` averages 6.102/5.733/5.595 ms. Source search, assembly and cache-validity timers overlap plan subscopes as described by the retained `timing_hierarchy`.
- `draws_and_route_context.all` averages 5.572 ms inside the market; `opening_and_enrolled_draws` is another market child, not a second top-level system.
- Zero-duration count/buffer labels are observations, not elapsed work.

These measurements locate market dispatch/raw freight as better production investigation targets than industry or the already-indexed course merge. They do not establish a particular redundant lookup, safe cache lifetime or removable traversal.

## Verification and unmeasured overhead

| Wall-time component | Seconds | Scope |
| --- | ---: | --- |
| Both sets of tick clocks | 5.843 | 2.963 observed plus 2.880 ordinary native |
| Full-world comparison clock | 16.193 | Two native pretty serializations, byte equality, headline comparison and row assembly |
| Command preflight | 0.042 | Almost entirely first-day renewal trial |
| Outside these clocks | 23.841 | Unattributed; not presumed I/O or simulation |
| Outer process wall | 45.919 | Receipt wall time; Rust test reports 45.80 seconds |

Comparison averages 522.359 ms over 31 days (ordinary-day mean 522.290 ms). Each serialized world is approximately 220.2–221.2 MB; both worlds together generate **13,706,977,464 bytes** across the run. This clock accounts for 35.26% of outer wall time. The 51.92% residual includes loading, cloning, FNV input passes, validation, initial/final facts and state hashes, report construction/serialization/writes, buffer destruction and test/process overhead. Their individual costs are not measured here.

One bounded source-level hypothesis is worth measuring before changing gameplay: `s08_diagnostic_write` rewinds/truncates the report and passes an unbuffered `File` directly to `serde_json::to_writer_pretty` after every day. Adding clocks around report construction/write and load/facts would attribute the large residual. If report writes prove dominant, a buffered writer or same-byte `to_vec_pretty` plus one write could preserve all observations, chronology, exclusive creation and final bytes. That is a diagnostic-only hypothesis, not an evidenced speedup and not an explanation for the full stability matrix.

The S25 matrix already writes its report via `fs::write(to_vec_pretty(...))`; it compares complete canonical campaign envelopes at monthly reload, the following day, mandatory checkpoints and terminal boundaries, rather than serializing both complete worlds on every ordinary day as this diagnostic does. Therefore, extrapolating 45.9 seconds per 31 days to its 1990–2035 cells would be invalid. The matrix's monthly storage/reload/canonical-archive costs require their own attribution. No equality checks, retention, histories, course progress or gameplay policy should be removed to improve timing.

## Process observations and limits

The receipt records 172 process samples, normally 266.9 ms apart (maximum gap 270.5 ms). The last is 269.9 ms before outer process completion.

| Counter | Maximum retained value |
| --- | ---: |
| Sampled working set | 822,554,624 bytes |
| Sampled private bytes | 1,048,752,128 bytes |
| Sampled OS peak working-set counter | 834,560,000 bytes |
| Sampled OS peak paged-memory counter | 1,059,246,080 bytes |

Last sampled cumulative CPU is 45.265625 seconds: 98.58% of outer wall expressed as one-core capacity. This supports describing the diagnostic process as predominantly CPU-active overall; it does not attribute CPU to a subsystem, measure total machine utilization, or establish the process's final CPU counter. Private/working samples may miss short peaks. The Windows peak counters remain distinct from private bytes, each other and GPU memory.

This process holds an ordinary `Game`, an observed world, route pools and temporary serialized comparison buffers; it is not a one-world production server or an S22 memory gate. No memory pass/fail or hardware-wide bottleneck is inferred.

## Recommended next decision

No production change is justified solely to remove the previously alleged quadratic course scan: that optimization already exists in the frozen build. Preserve this successful diagnostic and first attribute the profiler's outer overhead if faster repeat diagnosis is needed. For the actual long-run matrix, instrument its monthly storage/validation/compare phases separately; then inspect market dispatch/raw freight only if their measured cumulative cost still warrants a narrowly scoped, byte-exact improvement. This review makes no coefficient, historical-policy or acceptance-threshold proposal.

`analysis.json` contains recomputed values, all daily proof flags, complete source/identity pins and explicit limitations. `retained/` contains byte-exact copies of the six compact original artifacts. `manifest.json` pins all local packet files except itself. No native execution, source edits, Cargo invocation or campaign mutation was performed for this review.
