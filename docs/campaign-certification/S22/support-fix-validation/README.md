# S22 — aircraft support validation repair

**Passing regressions and behavior diagnostics; not performance qualification.**

Candidate `594c7ce1356a9c5a393e1079e4b7abff48cf093d` replaces the routine
air-support preflight world copy with the existing complete read-only validator.
The setters, prices, refusal order and AI decisions are unchanged.

The full release workspace run passed **1,902 tests across 66 suites**, with
**zero failures and 92 explicitly ignored tests**. Ignored tests are not counted
as passes. The three new command regressions compare exact errors and complete
world bytes with the former clone-and-apply path. The previously failing S22
schedule fixture now passes after normal initial enrollment.

The actual 1 January 2006 input was replayed for 31 days with the normal budget
renewal. Each instrumented day matched ordinary native world bytes and returned
headlines. Its ending world fingerprint `e5b4020488bb8e15` matches the previous
candidate's result; this cross-version observation is an FNV64 comparison, not
an archived whole-campaign SHA256 comparison.

Military review mean fell from about 278 ms/day to **23.9395 ms/day** in these
diagnostics. The new run overlapped legacy regression sweeps, so none of its
timings qualifies S22. It still exposed a 12 January company inventory-target
review spike and a month-end economic-policy spike. No limits changed, and the
required dated inputs and both full qualification rounds remain open.

The [manifest](manifest.json) identifies exact copied evidence bytes and source
paths. Large immutable inputs are stored once in
[runtime-preflight/inputs](../runtime-preflight/inputs/).
