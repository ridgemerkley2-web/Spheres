# Outstanding repairs — 22 September 2026

Baseline: `5d86fb35507acbd8fa4662ea9dc3f5f566ef93da`. This pass repairs
government behavior, saves funded political organizations, removes invisible
geometry, and reconciles the art audit with the detailed models already
accepted for the game. It does not claim that the optional historical
calibration or every full-detail town scene now passes.

## Government behavior and save compatibility

Disorder now enters the incumbent's performance record for both elected parties
and regime movements. Previously disorder increased opposition appeal without
charging the incumbent any support; a government losing control of its streets
could still gain support. The new term uses the existing price/growth weight of
0.90 and caps the order pain consistently with the discontent gauge. It is a
modeled gameplay coefficient, not a historically estimated causal effect.

Coup pressure is still accumulated and cooled by the existing rules, but firing
now also requires the underlying trigger to remain active. A recovered armed
institution cannot stage a regime coup solely from stored pressure. An electoral
coup likewise requires both a currently hostile army and a current crisis.
The existing thresholds, active-crisis path and save/load behavior are tested.

A successful covert funding operation that crosses the existing organization
threshold now records the resulting movement separately from its funding.
Withdrawal and exposure still reduce financial influence; they no longer erase
the organization. The optional, default-empty saved field preserves older saves
and is omitted until needed. Currently qualifying backing can initialize the
record when an older save resumes. The change does not reconstruct funding that
had already disappeared before the save, add votes or grant a takeover.
Organization presence and winning eligibility use the same saved state.

The movement feature remains inert with the political layer off. Disorder and
live coup-trigger repairs intentionally affect the base government simulation.
The startup fingerprint stays `6fc47dff64344b17`; the default 240-month timeline
changes from `4d2b187b81788c76` to `6866c0bc2e38de35`. Active timeline pins are
updated from measured runs; resource, market, treasury and bloc feature
invariants remain enforced. Historical calibration targets are unchanged.

[Final measured fingerprints](evidence/2026-09-22-outstanding/final-political-hashes.jsonl)
and [probe provenance](evidence/2026-09-22-outstanding/final-political-probe-provenance.json)
record the default and seeds 0–5. All nine focused government regressions pass,
including old-save migration before decay, exposure and crackdown at the exact
funding threshold, and a save with no government record yet.

## Libya's governing ideology

Libya's 1990 leader now has an explicit Nationalist game-bloc override. This is
a mapping of governing ideology, separate from diplomatic nonalignment or the
generic Party institutional pillar. The source is the US government-authored
1987 *Libya: A Country Study*, edited by Helen Chapin Metz. Its discussions of
[political ideology](https://countrystudies.us/libya/77.htm),
[Third Universal Theory](https://countrystudies.us/libya/78.htm) and
[revolutionary committees](https://countrystudies.us/libya/74.htm) support that
interpretation. The JSON row records the sources and the scope of the inference.

The opening bloc census becomes `[67,17,9,2,42]`. P-6's Algeria and Libya
disagreements are fixed. Its Communist quota still conflicts with the current
dated party roster: 17 versus an old design target of 11–13. Countries have not
been relabeled merely to meet that quota.

The explicit final optional runs still fail all seven gates:

| Gate | Before this pass | After | Unchanged target |
| --- | --- | --- | --- |
| P-6 Communist census | 17 | 17 | 11–13 |
| A1 coup distribution | Median 6; top-three share 1.00 | Median 5; share 1.00 | Median 4–14; share below 0.50 |
| A2 Islamist non-ballot takeover | 0/12 | 0/12; Algeria annulments 5/12 | 7–10/12 takeovers |
| A3 Communist non-ballot takeover | 14/20 | 12/20 | At most 2/20 |
| A4 democratization | Median 1 | Median 1 | 15–40 |
| A5 successor-party ballot return | 0/12 | 0/12 | More than 6/12 |
| A7 ballot/takeover ratio | 0.233 | 0.449519 | At least 3 |

These are monthly, ideology-takeover-enabled calibration fixtures. Normal
browser campaigns continue using the daily clock with ideological takeovers
disabled. Fixing organization lifetime did not itself change the aggregate
gate rates after the disorder/coup repairs. The remaining model work involves
term-long political performance, opposition organization and army funding;
changing numerical targets would not implement those missing relationships.
[Exact commands and results](evidence/2026-09-22-outstanding/persistence-final-checks-result.json)
retain the failing outcomes explicitly.

## Geometry and audit repairs

Buried track-pin caps are removed only when the solid connector fully encloses
them. This saves 4,032 triangles on standard/light/destroyer tank baselines and
4,416 on the heavy baseline. The reviewed loaded heavy falls from 84,244 to
79,828, retaining the same catalogue/map tiers. Six actual WebGL before/after
views are byte-identical. All aircraft geometry and exports, plus APC and
reconnaissance exports, remain unchanged. Eight affected ground GLBs are rebuilt
and reproduce exactly.

The broad JavaScript run also caught two stale material-buffer snapshots after
the cap removal. Their replacements are backed by a new permanent regression:
it reconstructs the old caps, verifies the historical surface hashes, then checks
all surviving triangles' material parameters and three finishes vertex by vertex.
Shader and coarse-detail pins remain unchanged.
[Derived-surface proof](evidence/2026-09-22-outstanding/derived-surface-preservation.json).

Completed factory halls omit 980 triangles of steel inside opaque walls, roof
and slab. Partial stages, external steel and all map geometry remain intact.
The arms-plant lead hall falls from 12,670 to 11,690 triangles; its whole
level-five compound falls from 35,300 to 34,320. Front/rear WebGL comparisons
are pixel-identical. Permanent tests independently check containment and the
unchanged bytes of every other part.

The original P0 proposals of 45k ground / 60k aircraft conflicted with the later
approved detailed tanks and the explicit 100k+ aircraft requirement. Contract
revision 2 now uses the already-enforced inspection bands and strict export
limits: tanks 20–150k / below 12 MB, specialists 8–48k / below 5 MB, aircraft
100–250k / below 28 MB. Aircraft falling below 100k fail. This is an explicit
requirements reconciliation, not a claim that those triangles were removed.
The old comparison remains in the generated report.

Whole compounds, terraces and campuses now pay the unchanged 150k scene limit
and each real building pays the unchanged 12k building limit. Authored ownership
ranges account for every triangle; shared walls and roofs are charged in full
to each owner and once to the assembly. Terrain and props stay in scene totals.
Coverage, overlaps, missing structures and unknown owners are tested. A six-home
terrace is no longer incorrectly graded as one building. Every measured physical
building fits its own limit.

The worst-configuration search now includes all native component files and all
accepted slots, including the optional seven slots missing from default tanks.
It also searches from the reviewed loaded-heavy fixture. The measured heavy
sample is 80,476 triangles. The search remains a deterministic lower bound on
the true maximum; generator acceptance is not proof of native design legality.
The gallery uses the same class budgets and still measures its actual layouts.

The revised audit grades **245 configurations: 164 PASS, 79 below advisory
density ranges, two OVER**. No required aircraft floor or serialized export
limit fails for the thirteen canonical reference exports. The export gate does
not certify every sampled component combination: the greedy geometry search can
include native-incompatible configurations, such as a howitzer turret on an IFV.
The old proposal comparison still has 33 over-limit configurations
on the current meshes, visibly distinguished from current-contract results.

The two real remaining scene overruns are mixed (163,671) and residential
(174,540), against 150,000. These full-detail TownMesh blocks currently appear
only in the art-review gallery; gameplay cities use CityMesh. Proper per-building
camera LOD needs a renderer/provider change and runtime draw-count verification.
No detail tier was renamed and no building was removed to hide these failures.

The equipment browser probe checks twelve platforms at LOD0/1/2/0, buffer reuse,
repainting, context recovery and disposal. It confirms allocation behavior on
the tested RTX 5070, not a frame-rate promise or driver VRAM usage. The roadmap's
60fps/30fps targets remain performance work.

## Evidence

- [Equipment geometry preservation](evidence/2026-09-22-outstanding/preservation-result.json)
  and [render comparisons](evidence/2026-09-22-outstanding/render-results.json).
- [Actual export sizes and hashes](evidence/2026-09-22-outstanding/exports-final.json)
  and [browser buffer results](evidence/2026-09-22-outstanding/equipment-browser-buffers.json).
- [Site/town metadata equality](evidence/2026-09-22-outstanding/metadata-geometry-preserved.json),
  [factory renders](evidence/2026-09-22-outstanding/site-enclosed-portal-render-result.json),
  and [remaining town-renderer scope](evidence/2026-09-22-outstanding/town-site-budget-work.md).
- [Political trajectory probe](evidence/2026-09-22-outstanding/political-bottleneck-result.log)
  records the remaining term-memory, organization-weighting and army-funding
  bottlenecks. It is diagnostic evidence, not calibration certification.
- [Protected originals](evidence/2026-09-22-outstanding/protected-originals.json):
  all eight protected files/saves and both original worktree heads remain intact.

## Final local validation

- Full native workspace: **1,774 passed, zero failed, 90 existing ignored**
  across 66 targets. This includes the simulation, CLI, web and HTTP tests.
  The isolated resource timing check adds one pass: **1,775 passed** total.
- Resource timing: 0.0607 ms/month, within the unchanged 0.15 assertion.
  The aspirational 0.05 target was not met on this reading.
- Full JavaScript suite: **1,694 passed, zero failed or skipped**.
- Leadership production/census: **15 self-tests passed**; production, census
  and research freshness checks pass. Generated changes contain only government
  source provenance and the dependent inventory hash.
- Equipment exports reproduce; the 33-asset art manifest is current; component
  coverage remains **191 distinct pairs**, zero weak or absent. Strict P0
  regeneration checks find current records and the **two genuine scene overruns**.
- The seven explicitly run optional political gates remain failing as listed
  above. They are not included among the passing checks.
- All original campaign saves/previews are preserved. Only obsolete, nonrunning
  derived test executables were removed to make room for the native rebuild.
- A final remote fetch found no additional upstream changes to integrate.

[Validation receipt](evidence/2026-09-22-outstanding/validation.json) records
commands, counts, log hashes and the distinction between required suites and
optional gates. The parent commit's six hosted Windows/Ubuntu jobs are green;
this repair commit receives its own hosted verification after publication.
