# S22 — qualification plan

**In progress · Codex · base `846df4797712935efb5a221e188d410b09f0d083`.**
S18 and S20 are complete. This session measures performance of the integrated
campaign using the existing S01 reference limits. The
[machine-readable protocol](measurement-protocol.json) is frozen before timing.
It sets requirements and repeat policy; it contains no performance result.

## 1. Preserve the contract and identify the candidate

Use `SPHERES-REF-WIN-01`, the existing Ryzen 7 9800X3D / RTX 5070 workstation,
and capture its current OS, driver, RAM, browser, GPU/WebGL identity and display
settings. Record existing background applications. Do not stop user applications
or unrelated processes to improve a number. Run authoritative lanes sequentially,
without task compilation, tests, fixture aging or another measurement lane.

Before either qualification round, freeze an immutable manifest containing the
full runtime revision, release binary and served-asset hashes, harness sources,
browser flags, all three input-save hashes, detail settings, camera path and
ordered control workload. Store the protocol hash with it. A missing or changed
identity leaves the affected qualification open.

## 2. Prepare actual dated campaign inputs

Start from the unchanged [S19 France save](../S19/integration/flight-closeout-evidence/recorded-flight-qualified.json.gz),
dated **13 May 1999**, raw SHA-256
`1af599ec1568a5b2148354de25cb800780d8ce69d352664f0a944242fac5ca8c`.
It already contains actual funded construction, company procurement, delivered
aircraft, a supported flown mission and readiness recovery.

The source nevertheless has **economic competition disabled**. Its supplier and
military AI enable flags do not prove active planning: their ticks are gated by
that rule. Apply the ordinary **EnableEconomicCompetition** command, costing
zero political capital, to a disposable copy on 13 May 1999. Record the command
result, before/after rules and whole-save hashes. The adopted early checkpoint
is a new input identity; keep the original source untouched. Check actual AI
planning/activity receipts during subsequent days, not just enabled flags.
The original passive `prepare01` lineage remains a diagnostic and cannot qualify.

Create disposable copies of that adopted lineage and age them through normal
native daily simulation. Keep history and the enabled qualification rules, and record every preparation command and
relevant government/company/force/ledger count. Do not replace dates, grant
resources, remove history or fabricate feature activity.

| Case | Actual input date | Consecutive one-day samples | Final measured date |
| --- | --- | ---: | --- |
| Early recorded France | 13 May 1999 | 31 | 13 June 1999 |
| Mid-2010s France | 1 January 2015 | 31 | 1 February 2015 |
| End-2035 France | 30 November 2035 | 31 | 31 December 2035 |

“Early” describes the selected feature-rich 1999 source. It does not establish
1990 startup performance. A save labeled “late” must contain its actual 2035
state; the old 2020 technical fixtures cannot substitute for it. Preparation
and loading are recorded outside the timed daily loop.

## 3. Qualify native daily work and memory

For each input, measure 31 consecutive one-day requests with raw simulation plus
history, read-model generation, serialization, selected-country history delta,
and whole-turn observations. Use nearest-rank p95: sample 30 in sorted order.
Whole-turn percentiles come from whole-turn samples, never sums of percentiles.

| Existing S01 limit | Required result |
| --- | --- |
| Simulation plus history recording | p95 ≤300 ms per case |
| Complete server turn | p95 ≤400 ms and maximum ≤750 ms per case |
| Dedicated headless process | Observed OS peak working set ≤1 GiB **and** maximum sampled private bytes ≤1 GiB |

Retain nominal 100 ms or finer memory samples, actual sampling gaps, failures
and accessible OS high-water readings. Sampled private bytes do not establish
an instantaneous private-memory peak. Report actual completed days divided by
the full measured loop duration for sustained throughput. Loading, startup,
preparation, disk autosave and browser/network work remain separately labeled;
`1000 / median` is not observed throughput.

## 4. Qualify the actual rendered browser workload

Load each of the three actual anchor inputs: 13 May 1999, 1 January 2015 and
30 November 2035. Browser late coverage is explicitly 30 November; the native
31-day window supplies actual 31 December endpoint timing and state facts.
Do not label the browser input as 31 December. At **1920×1080 CSS pixels, DPR 1**, measure
both ordinary and low map detail at world, national France, regional Paris
province and Paris city views. Each view records **3 seconds of settling/cold
work separately**, followed by **12 seconds of actual navigation**.

Count completed globe redraws over the full elapsed active window, including
blocked time. Record draw durations and intervals, actual GL submissions and
GPU completion. The frozen method calls real `gl.finish()` after real draws;
this conservative extra synchronization is disclosed. Idle animation callbacks
are not rendered frames. The existing map target is **at least 30 FPS** in
every required cell, with no rounding tolerance added after a result is known.

Inspect the released `air_fighter` asset in the actual native equipment designer
of the loaded campaign with at least **100,000 triangles**, recording its asset
revision, visible design/catalog specification and measured cost. This may differ
from S19's delivered light-attack aircraft; it makes no stock-ownership or delivery
claim for the inspected design. Retain
the existing art-roadmap target of **60 FPS** for focused single-model viewing.
Use the same settling/active-window and completed-draw principles; 59.x does not
pass 60. Preserve the raw data if the target is missed and repair the cause.

For each dated input and detail profile, start at national France zoom 8 and run
31 visible trusted camera-control clicks: repeat **west, east, zoom-in, zoom-out**
in that order, ending at action 31. Measure each trusted event timestamp through
confirmed camera-state change, two animation-frame paint opportunities and real
GL completion. This is local UI handling plus paint opportunity/GPU completion;
it does not prove compositor presentation or screen scanout. These view controls
make no read request, so no server-response latency is claimed. Require
**p95 ≤200 ms**. Retain every latency, control identity and before/after camera
state, and assert the requested state change. Freeze selectors and exact camera
parameters in the candidate manifest before either round.

Record uncached loading, JS heap method/availability, browser-process observations
and actual GL allocation/cache payloads separately. Neither GL payload nor
process memory is a driver-VRAM measurement. Verify lazy loading, repeated
inspection/room visits, CityMesh and inspection cache bounds, disposal and
context loss/recovery with actual counters and visible recovery evidence.
The separate gallery TownMesh planner is not a substitute for the campaign
CityMesh path.

Check keyboard/touch, focus, scrolling and readable content at **390×844** and
room/map framing at the observed **3440×1440** desktop. Keep these layout checks
separate from the 1080p performance cells. Retain screenshots and Chrome traces;
trace capture is still pending implementation/verification by the browser owner,
and is required before the evidence packet can qualify.

## 5. Require a complete confirmation pair

Declare qualification attempt IDs and round roles before launch. Run one full
initial qualification and one full confirmation on the **same runtime, binary,
assets, harness, inputs, settings and workload**. Every required native, map,
inspection, UI, memory, lifecycle and layout cell must pass in both rounds.
Do not pool cases, average away a failure or fill a failed cell with a later
faster sample.

Keep every failed, interrupted, timed-out and debugging attempt unchanged with
its identity and reason. Diagnose and fix failures. A runtime, harness, input or
configuration change starts a new full pair. Even an unchanged-candidate retry
starts a new full pair; it cannot cherry-pick cells from older attempts.
Objective contamination remains documented. Missing a limit alone is not a
reason to relabel an attempt invalid. Preflight work cannot be promoted into a
qualification round after observing its numbers.

Only accepted complete results can close S22. The offline art audit and this
plan leave **S22 in progress**, with G5, CP1 and human qualification open.
