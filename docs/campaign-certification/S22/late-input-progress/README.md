# S22 late input and functional progress

This append-only packet preserves the actual **30 November 2035 France
checkpoint**, its reviewed lineage, focused regression results, and two map
preflights. **Qualification is false. S22, G5 and CP1 remain open.** The
preparation and browser timings below are diagnostic observations collected with
other work active; they do not establish a performance pass or comparison.

## Actual campaign preparation

`preparation/` contains the complete active03 invocation, native profile,
runner result, journal, console, stdout/stderr and compressed memory CSV.
Candidate `a4e246e4399e0cc3148bdbb60b39dde3c9932b83` advanced the unchanged
2015 input through **7,638 ordinary days**, ending on 2035-11-30 with France
alive, 1,360 history points and 37,893 dispatches. Native preparation and its
wrapper passed. The final world fingerprint is `d80100ad9735c60b`.

The final input in `inputs/s22-2035-11-30.json.gz` expands to **145,134,799
bytes**, SHA256
`67aadca2f55280abc0e4ad97944a654ec7d173ffb65cb7d71dd59090e10503cb`.
Its 24,310,728 compressed bytes have SHA256
`42064dc3a9e3f0404a967979bcea1fee809afbfeb122fa241a0ddffe50a00c8b`.
The retained input manifest records the verified lossless round trip. Original
inputs and annual checkpoints remain at their recorded evidence paths.

`lineage/` preserves the review and configuration fragment exactly as written.
The four reviewed links are original S19 → ordinary same-date competition
adoption → retained 2006 → completed 2015 → completed 2035. The reviewer checked
actual SHA256 values, nested saved dates, commands, checkpoint receipts and
monotonic/alive journal records. **This was a record review, not an independent
replay of the intervening years.**

The active01 attempt remains explicitly interrupted/failed (exit -1), with its
valid 2006 checkpoint retained. Active02 actually resumed the original `7de5…`
input, not its `90cd…` native resume-save output; active03 resumed `e786…`, not
its `e3f977…` resume-save output. No serialization output becomes an invented
ancestor. The manifest byte-verifies 13 earlier reports in
[runtime-preflight](../runtime-preflight/README.md) and
[stock-fix-progress](../stock-fix-progress/README.md); they are not duplicated.
Captured absolute provenance paths remain original machine paths.

Preparation success only means that the requested date was reached. Its
recorded 1,472,331,776 sampled private bytes and 1,428,496,384 OS working-set
peak are retained, not presented as satisfying the separate native memory gate.

## Focused verification and retained failures

| Evidence | Revision/scope | Recorded result |
| --- | --- | --- |
| Decoder regression group | `568e5a0b` | 11 passed, 0 failed, 1 ignored |
| Actual 2015 decoder versus original Value decoder | `568e5a0b` | 1 passed; full archive parity |
| Actual 2035 decoder versus original Value decoder | `9c7cf934` | 1 passed; full archive parity |
| Procurement, campaign preparation and observed schedule parity | `9c7cf934` | Three corrected runs, one test passed each |
| Camera/control contract suite, first attempt | `97ad0527` | 25 passed, 1 failed |
| Camera/control contract suite, corrected assertion | `7762aca3` | 26 passed, 0 failed |

The first storage build selected a nonexistent library target; that failure is
retained beside the corrected build logs. Two initial parity filters matched
**zero tests**; those logs remain and are not counted as coverage. The first
camera test expected `.count` from a validator returning `.samples`; its
assertion was corrected. The independent source/evidence review is included.
No frame-rate, latency, memory or workload limit was relaxed.

The 568e provenance is contemporaneous. The 9c provenance explicitly identifies
itself as a retrospective command record, with the actual retained binary hash.
Sequential decoder load timings in the logs are neither memory measurements nor
qualification. These are focused tests, not a new complete workspace regression
run.

`diagnosis/` preserves the 9c actual-2015 leaf-timer run: **31 consecutive days**,
January 1 to February 1, exact native world and returned-headline equivalence
each day, unchanged input, ending fingerprint `5c486d0c186f921e`. Its nested
timers overlap parent timers; do not sum them as independent costs.

## Map attempts

`browser-BaJ828` failed before loading the campaign: the served build reported
`9c7cf934308e-modified` instead of the expected clean revision. Its original
result, failure screenshot, sources, console and trace remain intact.

`browser-3VKqAe`, at clean `5c6509918a65cffeffd22c00f8645bc2c4ac4ede`,
passed all eight map-view checks, both ordered sets of 31 trusted controls,
390/3440 layout and focus routes, memory-observation completeness, and saved
state purity. Recorded control p95 values are 48.20000000298023 ms (standard)
and 44.5 ms (low). This is **functional preflight only**, not qualification:
archive compression, decoder checks and other work overlapped its run.

Both exact compressed traces are retained without recompression. The successful
trace is 103,788,652 bytes (below 100 MiB); the result records 1,001,239,138 raw
bytes. Its compressed bytes and hash were checked during packaging. Browser
process/heap observations and declared GL buffer/texture payloads stay separate;
they are not physical driver VRAM measurements.

`manifest.json` records every captured file's source, byte count, SHA256 and
compression mapping. Large JSON and the preparation memory CSV are losslessly
compressed. `.gitattributes` prevents newline conversion. Executables, server
directories, duplicate raw inputs and duplicate browser save archives are
excluded; original purity hashes and read-only observations remain in results.
