# S22 — art accounting and performance qualification

**Status: in progress. Owner: Codex.**

This progress snapshot retains completed evidence through 28 September 2026,
06:06:37 UTC: preparation/full regressions at `594c7ce1`, stock-target work at
`a96bb097`, and ministry-read validation/preflights at `a4e246e4`. Later candidate
changes and the running 2015 subsystem diagnosis are outside these claims.

The [plan](PLAN.md) and [frozen measurement protocol](measurement-protocol.json)
start the next canonical session after S20/G4. They preserve the engineering
targets from [S01](../S01/PERFORMANCE_BASELINE.md) and the existing
[art contract](../../art/3D_MODEL_MASTER_ROADMAP.md). No current runtime timing
qualification is claimed by this packet.

A pre-qualification clarification fixes the 31-click local camera sequence and
labels its measured endpoint as paint opportunity plus GPU completion, not proven
screen presentation. The inspected released `air_fighter` asset may differ from
S19's delivered aircraft; rendering it does not claim stock ownership. All original
limits and the two complete confirmation rounds remain unchanged. Chrome trace
capture now works in the disposable native-browser preflights. The newest harness
also records declared texture payload separately from queried buffer payload;
its functional browser validation passed in the retained
[renderer preflight](renderer-preflight/README.md). That run overlapped native
regressions, so its timings remain diagnostic. Neither counter is driver VRAM.

| Work | State |
| --- | --- |
| Current offline art, accounting and reproduction audit | **Complete and passing**, bounded to the scope below |
| Actual France 1999 / 2015 / end-2035 inputs | Adopted 1999 input and actual 1 January 2015 descendant retained; preparation to end-2035 remains open |
| Native 31-day timing and headless memory | a4e246e4 2006 preflight passes numerical checks; actual 2015 still fails all latency limits while passing memory |
| Actual rendered map, aircraft and UI performance | Early functional preflights pass; concurrent-run timings do not qualify, and both complete qualification rounds remain required |
| City/inspection caches, context recovery, loading and layouts | Functional renderer/texture validation retained at 594c7ce1; final-candidate qualification required in both rounds |
| S22 closure | **Not earned** |

The original S19 save has economic competition disabled, which also gates
supplier/military AI ticks despite their enabled flags. Qualification first
records an ordinary zero-cost enable command on a disposable copy, then ages
that adopted lineage with actual activity evidence. The original passive
`prepare01` remains a diagnostic; its unchanged source is preserved.

## Current runtime findings

The [original preflight packet](runtime-preflight/README.md) preserves failures,
raw samples, memory observations and immutable campaign inputs. These runs were
declared diagnostic before execution and cannot become qualification afterward.
The [stock-fix progress packet](stock-fix-progress/README.md) adds subsequent
candidate-specific tests, the still-failing isolated preflight, and the completed
2015 preparation without replacing any earlier attempt.

The actual **1 January 2006** checkpoint failed the original limits: simulation
p95 **647.0195 ms**, whole-turn p95 **742.3764 ms**, maximum **1087.362 ms**,
and peak sampled private memory **1,736,220,672 bytes**. No threshold changed.

Consuming parsed save trees through the existing validation/migration path
removes redundant world copies. Repeating the same checkpoint then used
**850,567,168 bytes** peak private memory and **844,275,712 bytes** observed OS
peak working set, both below 1 GiB. Latency still fails: **609.6552 ms** simulation
p95, **712.4279 ms** whole-turn p95 and **942.5057 ms** maximum. The small timing
change is not attributed to loading, which is outside the timed turn.

A separate 31-day subsystem diagnosis matched the complete native world and
returned headlines each day. Military AI was the largest measured component,
averaging about **278 ms/day**. Its air-support command preflight copied the
entire world to check a small budget edit. The narrow validation repair is now
verified at `594c7ce1`: [1,902 release workspace tests passed](support-fix-validation/README.md)
across 66 suites, with no failures and 92 explicitly ignored tests. The earlier
schedule-fixture failure also passes there. This full regression result belongs
to that candidate, not to every later change.

At `a96bb097`, stock-target-only company commands also avoid whole-world trial
copies. Its focused validation passed **14 company tests, one pooled-clearing
equivalence test and 425 web tests**, with 24 web tests ignored. This is not a
second full workspace run. The isolated 2006 preflight then measured simulation
p95 **291.0196 ms**, whole-turn p95 **395.7494 ms**, and maximum **792.7021 ms**.
Peak sampled private bytes were **935,395,328**, with observed OS peak working
set **934,604,800**. Both p95 and memory checks pass, but the maximum still fails
the unchanged 750 ms limit; the attempt remains failed. A separate 31-day
subsystem run matched native world bytes and headlines each day. Its nested
diagnostic timers overlap their parents and are not qualification measurements.

The subsequent ministry-read repair at `a4e246e4` passed **426 web tests**
(25 ignored) and a separate actual-2015 check of exact ministry JSON, world and
source immutability. Its 2006 preflight passes numerical checks: simulation
p95 **290.692 ms**, whole-turn p95 **362.8336 ms**, maximum **511.0135 ms**.
The same candidate on the actual 2015 input **still fails latency**: simulation
p95 **433.455 ms**, whole-turn p95 **545.8514 ms**, maximum **1084.5767 ms**.
2015 private/OS working-set peaks **1,004,343,296 / 994,193,408 bytes** pass the
1 GiB limits. Both complete attempts remain archived; the passing 2006 case
does not qualify the failed 2015 case or the full campaign.

Preparation on `594c7ce1` advanced the preserved 2006 checkpoint through **3,287
ordinary days** to **1 January 2015**, without a second adoption or date rewrite.
The completed preparation and exact compressed checkpoint are retained in the
new packet. Preparation success does not establish 2015 performance: its observed
process-memory envelope exceeded 1 GiB, and it was not a dedicated timing run.
The original 2006 input remains unchanged. Later runtime repairs require their
own tests and measurements; none are inferred from this snapshot.

Early browser preflights exercised actual rendered city detail, the released
100k+ triangle fighter, cache eviction, context restoration, repeated visits,
390px/3440px layouts and read-only campaign preservation. The retained native
`browser-ebQ8t2` run specifically passes buffer/texture completeness, context
recovery and six fighter visit/closure checks, with zero viewer buffers/textures
after each close. Its 99.915 FPS orbit result is diagnostic only because the
workspace tests ran concurrently. Completed repairs
include eviction before city uploads, respecting Cities Off in the actual draw
path, releasing detached preview references and clearer Standard/Low controls.
These bounded results do not establish performance of the later dated inputs.

## Completed offline audit

[Audit findings](preflight-art/README.md), [command/source manifest](preflight-art/result.json)
and [derived verdicts](preflight-art/findings.json) are copied byte for byte from
the independent preflight at `846df4797712935efb5a221e188d410b09f0d083`.
Its clean checkout and all 39 pinned source, contract, record and export hashes
were unchanged. Execution ran 28 September 2026, **04:31:20–04:35:02 UTC**;
that window is not a game-performance result. No browser probe ran in this audit.

- `bench_art.cjs --check`: **245 configurations**, 166 passes and 79 advisory
  density notes; **zero current ceiling, required-quality-floor or export failures**.
  Both generated measurement records reproduce.
- `build_art_manifest.cjs --check`: the **33-asset** manifest is current.
- `build_equipment_models.cjs --check`: all **13 GLBs** reproduce byte for byte;
  a separate inventory check found no missing or extra canonical GLB.
- Nine focused source/accounting/quality/gallery files: **35 Node test entries
  pass**, zero failures/skips. Included files additionally report 32 equipment
  mesh and 5 fighter mesh internal checks; those are not 37 extra Node entries.

The earlier proposal still has **33 explicit diagnostic overages** on the
current meshes. Contract revision 2 predates this audit and reconciled later
approved inspection quality/export limits; this pass did not widen any bound.
Mixed and residential raw close town blocks remain 163,671 and 174,540
triangles. Their measured gallery draw plans submit 149,997 and 149,934,
within the unchanged 150,000 scene ceiling. The tiny 3/66-triangle headroom
remains guarded. Raw overages are retained rather than presented as reductions.

Accounting checks include CPU material tags, shared backing buffers, unknown
typed-array rejection, complete constituent ownership and shared-envelope
charges. No actual accounting failure was found within that declared scope.
The offline inventory excludes renderer-derived attributes, floors, shadows,
textures, driver overhead and application heap. It is neither live residency
nor a frame-rate result. Aircraft are measured separately from the hypothetical
ground/site/town inventory. The gallery **TownMesh** path is separate from the
campaign globe **CityMesh** path, which still needs final-candidate runtime
qualification despite its bounded functional preflight.

## Evidence preservation and next action

The original [S22 preparation](preparation/README.md), S01 timings, current art
records and S19 source save remain unchanged. The preflight artifact runner is
retained as captured: its original staging layout was
`work/campaign-certification/evidence/s22-preflight-art/`, next to `integration`.
The exact portable command arguments and source hashes are in `result.json`.

Next, finish the latency repair and end-2035 preparation, then freeze all dated
inputs, candidate/configuration manifest and attempt IDs before executing the
protocol's two complete qualification rounds. Preserve every
failed attempt and fix the cause without raising a threshold. S22 completion
does not follow from the offline audit, and no G5, CP1, worldwide-history or
human-playtest completion is awarded here.
