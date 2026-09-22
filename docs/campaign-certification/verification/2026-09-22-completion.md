# Completion follow-up — 22 September 2026

Baseline: `f40f9216452f446db460ee3ca0622163d8123e7a`. This follow-up addresses
the seven political checks and two town-scene overruns left by the preceding
[outstanding-repairs report](2026-09-22-outstanding-repairs.md). It does not
equate a green regression suite with completion of the entire campaign roadmap.

**Validation in progress.** The final political readings and complete native
workspace verification must be recorded before this repair pass is closed.

## Town scenes

The gallery now renders a camera-dependent list of individual buildings. Each
lot retains its stable placement and its original close, middle and map meshes.
The visible list accounts for the background, frustum culling, projected size,
the selected building and the unchanged 150,000-triangle submission ceiling.
Selecting a building keeps that building at close detail; surrounding buildings
can move to a cheaper existing tier. A pre-upload eviction fixes the temporary
GPU payload overrun found during verification.

All 135 compared raw block configurations retain byte-identical geometry,
materials, ownership and bounds. The raw mixed/residential blocks still contain
163,671/174,540 close-detail triangles; these counts remain in the budget report.
The repaired budget measures what the renderer actually submits. No detail tier
has been renamed and no stored building has been removed.

The current audit grades 245 configurations: **166 pass, 79 below advisory
density ranges, zero over**. Its 229,760 camera/selection plans include the
gallery's string-valued default block ID. Real Chrome WebGL verification covers
360 overview draws and 104 selected buildings, with measured submissions equal
to their plans. The tested maximum is 149,934 triangles. Resize, controls,
context recovery, cache limits and disposal pass. These are geometry/payload
measurements, not a frame-rate or driver-VRAM guarantee.

This path serves the art-review gallery. The main map's separate CityMesh
renderer is unchanged. Local bundled Chromium failed before process launch;
the successful local browser evidence uses Chrome. Hosted Chromium passed on
Ubuntu at checkpoint `9d352f03`. That checkpoint's Windows native job exhausted
its 45-minute allocation during the town harness after all preceding checks had
passed; its interrupted town lane is not a pass. [The timeout record](evidence/2026-09-22-completion/checkpoint-ci-timeout.json)
preserves the original result. Commit `78cd39bb` gives the unchanged town harness
independent Ubuntu/Windows jobs and retains the aggregate success requirement.
In [run 35761708980](https://github.com/ridgemerkley2-web/Spheres/actions/runs/35761708980),
both independent town jobs passed with installed Chromium 145: Ubuntu in 132.14
seconds and Windows in 152.48 seconds. The [job receipts](evidence/2026-09-22-completion/town-ci-independent.json)
pin the checked-out revision and uploaded artifact identities. All eight jobs
have now completed successfully: native, JavaScript, town browser and required
aggregate checks on both operating systems. The [complete checkpoint receipt](evidence/2026-09-22-completion/checkpoint-ci-complete.json)
records their API conclusions and exact head `78cd39bb3b2ee2a7d4fe1d43b18702a51fbfbdbc`.
This certifies that checkpoint, not the later uncommitted political repairs.

- [Renderer architecture and limits](../../art/TOWN_SCENE_RENDERING.md)
- [Actual browser submissions](evidence/2026-09-22-completion/town-scene-browser.json)
- [Raw geometry preservation](evidence/2026-09-22-completion/town-raw-preservation.json)

## Opening historical census

P-6's original 11–13 Communist-government estimate conflicts with the dated,
named roster. It is explicitly replaced by a strict named-country assertion,
not reported as having met the old quota. Angola and São Tomé receive sourced
January 1990 leader overrides, distinct from their later election-party rows.
The opening census is `[65,19,9,2,42]` across 137 countries.

The [reconciliation record](../../political-arm/1990-census-reconciliation.md)
preserves the old failed requirement, sources, date distinctions and the scope
of the correction. No A1–A10 statistical acceptance threshold is changed by it.

## Claude research integration

CLAUDE-C01-01/02/03/04/07/08 are merged and accepted as bounded research intake.
The [C01-04/07 review](../C01/integrations/CLAUDE-C01-04-07/README.md) reproduced
twelve source response identities and checked two IPU pages by content.
The [C01-08 review](../C01/integrations/CLAUDE-C01-08/README.md) reproduced another
34 response identities and explicitly retains four independently unverified
source contents. Its reviewed revision `14f01c5a` was merged at `bd24d577`;
all 58 merged Tonga tests and the exact research-index check pass.
The [C01-03 review](../C01/integrations/CLAUDE-C01-03/README.md) accepts the
additional Tonga resignation, caretaker and appointment evidence merged at
`5a63d7b6`. Seven critical source response identities were independently
reproduced; 85 Python checks and 11 atlas unit tests pass. The [merged browser check](../C01/integrations/CLAUDE-C01-03/browser-validation.json)
passes all nine packets at 1,440, 390 and 320 pixels, including the corrected
holder dates and uncertain term endings. Its research-index checks pass; the
shared production census fingerprint remains pending the political source
freeze. C01-05/06 remain submitted and pending independent acceptance; the older
Saudi C01-03 branch is the renamed C01-06 packet, not separate work.
Missing instruments, uncertain terms and incomplete country coverage remain
open. No portraits, complete historical chains or C01 closure are implied.

## Correcting two broader test contracts

The mature-economy test's stated purpose is preventing technology from being
counted twice in productivity. Its old reading used the final smoothed growth
rate, which includes wars and recovery. In the diagnostic seed, Germany's
0.18% reported growth follows seventeen war months despite 1.12% structural
potential; France is also recovering. The same three seeds, thirty-year horizon,
four countries and 0.8–2.6% band now test structural potential. A separate
invariant inside the test runs the real technology tick against each unchanged
mature stock as a common world benchmark: technology already priced into the
trend cannot create a second productivity gain. The band alone would be too
weak, so deliberately removing the reference subtraction must fail this guard.

The pact integration test formerly required honoured great-power guarantees in
at least three of twelve campaign runs. The unchanged cohort produced only five
actual calls: two honours and three refusals, across four campaigns. Traced
refusal probabilities correctly reflect reputation and fatigue; no production
defect was found. This incidental honour quota is explicitly superseded by an
opportunity-and-outcome integration contract: three to eleven campaigns must
produce answered great-power obligations, both outcomes must occur, honours
must actually join the defender's side, and refusals must actually break the
pact. The separate forty-invasion probability check remains unchanged: all
forty answered, more than two honours per refusal, and at least one refusal.
The old two-of-twelve reading is retained; it is not claimed to meet the old
three-honour quota. No war, reputation or pact probability coefficient changes.

These test-contract corrections are distinct from the A1–A10 political
calibration thresholds, which remain unchanged.

## Validation recorded so far

- Political verification is still incomplete. The [iteration-9 workspace receipt](evidence/2026-09-22-completion/political-calibration/iteration-9-workspace-receipt.json)
  retains **1,839 passes, one fixture failure, 89 ignored and one resource check
  filtered**. Its corrected fixture passes in later focused runs; those runs
  do not retroactively turn that whole-workspace result green.
- Iterations 11/12/13 measure the predeclared `.024`/`.024`/`.030` accountability
  responses. Their focused results are **85/86, 87/88 and 88/88**. The two earlier
  failures are preserved: a coefficient-dependent equilibrium expectation and
  a full-Polity pointer assertion invalidated by France's sourced Army overlay.
  Corrected conservation/continuity requirements pass in iteration 13.
  Each build passes **nine of the ten original political outcome gates** plus
  the attribution guard; **A1 coup concentration still fails**. The
  [iteration history](evidence/2026-09-22-completion/political-calibration/iteration-history.json)
  and neighbouring source manifests retain exact measured-build identities.
- Same-base 200-seed development results favour the smaller `.024` response:
  [iteration 12](evidence/2026-09-22-completion/political-calibration/iteration-12-development-n200-summary.log)
  reads A2/A3/A5 at **125/2/104 of 200**; the `.030`
  [iteration 13](evidence/2026-09-22-completion/political-calibration/iteration-13-development-n200-summary.log)
  reads **162/14/105 of 200**. A5's narrow `.520`/`.525` majority remains uncertain
  and depends on the Russian return in addition to Poland and Bulgaria. The
  smaller value is selected for the next combined base, not certified as final.
  A1 concentration remains `.60`/`.57` in those cohorts. All targets are unchanged.
- The [mechanics record](../../political-arm/2026-09-22-calibration-repairs.md)
  explains these results and the predeclared confidence comparison. Diagnostic
  exit zero is completion, not acceptance; ignored checks are not passes.
  The independent 1000-series block remains uninspected, and no final whole-workspace
  or hosted verification of these later political changes is claimed.
- The [completed confidence grid](evidence/2026-09-22-completion/political-calibration/confidence-grid-results.json)
  retains iterations 14–16: focused **91/91, 90/91 and 89/91**, with
  **9/10, 8/10 and 7/10 original outcome gates** passing respectively.
  Development median coup concentration remains **.62, .56 and .50**, all
  failing the strict `<.50` requirement. The grid is rejected as a remedy;
  confidence is restored to `.65`, with `.024` accountability. The later
  correction of the negative funding fixture is not counted as tested by
  these earlier binaries. No sample sizes, outcome thresholds or winner
  definitions were changed; independent validation remains pending.
- [Iteration 18](evidence/2026-09-22-completion/political-calibration/iteration-18-calibration-summary.json)
  verifies the sourced opening-mandate and model-clock corrections at the restored
  `.65` confidence/`.024` accountability settings: **99/99 focused checks** and
  **nine of ten original outcome gates** pass, alongside the attribution guard.
  A1 still fails at median **7 coups, .69 top-three concentration**. Its fixed
  200-seed development cohort reads A1 **7/.67**, A2 **125/200**, A3 **5/200**,
  A5 **112/200**; the other measured bars pass within their recorded scopes.
  The new whole-world political clock equivalence guard passes. This remains
  incomplete calibration, not a final certification or independent validation.
- The subsequent [iteration-18 full workspace receipt](evidence/2026-09-22-completion/political-calibration/iteration-18-workspace-result.json)
  retains **1,853 passes, one basing scenario fixture failure, 89 ignored and
  one resource timing check filtered** across 66 targets. The failed scenario
  saw no ambient AI grants in twenty years. A later test-only correction
  explicitly verifies real grant/revoke commands and save persistence before
  retaining the long-run structural access invariant; this receipt predates
  that correction and remains a failed whole-workspace result. The exact
  corrected [iteration-19 basing check](evidence/2026-09-22-completion/political-calibration/iteration-19-basing-result.json)
  subsequently passes on its recorded new web binary; this targeted pass does
  not certify the entire workspace.
- The [iteration-19 snap-election grace repair](evidence/2026-09-22-completion/political-calibration/iteration-19-calibration-summary.json)
  passes **102/102 focused checks** and nine of ten original political gates.
  A1 remains **7.5/.71**. Its fixed development cohort measures A1 **7/.67**
  and A5 **99/200**, so both fail their development bars. The original A5
  result remains7/12; neither reading is substituted for the other. The
  correction prevents a continuing governing party buying repeated coup
  immunity through snap elections; election dates and thresholds stay intact.
- The [corrected iteration-20 workspace](evidence/2026-09-22-completion/political-calibration/iteration-20-workspace-result.json)
  passes **1,862 tests, zero failed, 89 ignored and one resource timing check
  filtered**, across 66 targets. All **107 focused checks** pass. This source
  has the sourced military leverage lifecycle; an earlier encoding-damaged
  preflight build was repaired and never measured. Exact repair receipts and
  verified raw native-source snapshots distinguish the corrected build.
  [Political outcome verification](evidence/2026-09-22-completion/political-calibration/iteration-20-calibration-summary.json)
  remains incomplete: original A1 is **8.5/.62** and development A1 **8/.62**.
  The other original and development bars pass, including development A5
  **124/200**. Ignored checks, filtered timing and independent validation are
  not included in the ordinary workspace pass count.
- [Iteration 21](evidence/2026-09-22-completion/political-calibration/iteration-21-calibration-summary.json)
  passes **111/111 focused checks**. Preserving real accountability while
  scheduling successor elections repairs A5 in both unchanged cohorts:
  **12/12 original and 200/200 development**. A1 is still the sole outcome
  failure, at **8.5/.62 original and 8/.62 development**. A later exact
  [restored-21 workspace run](evidence/2026-09-22-completion/political-calibration/iteration-21-restored-workspace-result.json)
  passes **1,866 tests, zero failed, 89 ignored and one resource timing check
  filtered**. It does not certify subsequent repairs or independent outcomes.
- [Iteration 22 was rejected](evidence/2026-09-22-completion/political-calibration/iteration-22-calibration-summary.json):
  **113/114 focused checks** and eight of ten original outcome gates pass.
  Original A1 is **12.5/.52**, original A2 **6/12**, and development A1
  **13/.50**, still outside the strict concentration bar. The three new
  severe-crisis controls pass, but do not override those failures. Exact
  iteration 21 government source was restored; all candidate evidence and
  its failing Algeria scenario are preserved. Its target executables were
  subsequently replaced by the explicitly recorded restored-21 build.
- [Iteration 23](evidence/2026-09-22-completion/political-calibration/iteration-23-calibration-summary.json)
  passes **128/128 focused checks**, including actual fiscal ownership/bounds,
  paid-card effects, coup debt accounting and live-executive AI choice. Nine
  of ten original outcome gates and the attribution guard pass. A1 remains
  **7/.59 original and 7/.57 development**. The original A4 median is 15 and
  the fixed development median 16; A5 stays 12/12 and 200/200. All other measured
  development bars pass within their stated horizons. The complete pooled
  coup distribution still concentrates 835 of 1,498 events in Guatemala, Sao Tome
  and Myanmar. No exact first/repeat decomposition is printed by this diagnostic.
  Fresh source capture records 1,963 inputs and 468 native source bodies; the
  copied binaries are tied to those hashes. Full-workspace verification of
  these later changes and independent validation remain separate work.
- Earlier local JavaScript suite: **1,701 passed, zero failed or skipped**.
  The hosted `9d352f03` checkpoint measured **1,700 passed, zero failed, one
  skipped** on both operating systems because the archived advisor API file
  was outside the checkout. The current repair checks in a 32,618-byte
  projection of that original reading with response provenance and identical
  complete advisor output; the four original assertions are mandatory.
  Advisor/guidance checks pass **50/50 with no skips**, and the advisor's 27
  tests also pass in an isolated file set without the external archive.
  A deliberately corrupted fixture fails. The subsequent [full local Node run](evidence/2026-09-22-completion/advisor-full-js.json)
  also passes **1,701 tests, zero failed or skipped** in 107.39 seconds, with
  the repaired fixture. That receipt identifies its working-tree inputs; it
  predates the later C01-03 browser assertion update and does not claim a
  hosted zero-skip result. [Fixture provenance](../../../tools/ui/fixtures/ADVISOR_ARCHIVE.md).
- Art manifest: 33 assets current. Component coverage: 191 distinct pairs,
  zero weak or absent. All thirteen canonical equipment exports reproduce.
- Strict current art-budget regeneration check passes.
- All 44 canonical roadmap markers remain assigned exactly once.
- Hosted results must identify their own revision: the `9d352f03` checkpoint
  retained an incomplete Windows town lane after a job timeout; the separate
  town-job repair at `78cd39bb` has passed all eight hosted jobs, including
  both native and independent town lanes. Neither checkpoint
  certifies the subsequent uncommitted political calibration changes.

The next narrow correction protects an existing annual budget in legacy saves
that lack dollar stocks. An [original-23 compiled-library witness](evidence/2026-09-22-completion/political-calibration/army-plan-owner-original23-witness-result.json)
reproduces its erasure through the public government tick. The independent plan
ownership guard and positive/save-load regression pass checkpoint 24's
1,005-test simulation library. This is not evidence of an A1 repair; its
original and development concentration failures remain open.

The [full checkpoint-24 workspace run](evidence/2026-09-22-completion/political-calibration/iteration-24-workspace-result.json)
records **1,881 passed, three failed, 89 ignored and one timing check filtered**.
Failures are the treasury source audit encountering a test-fixture debt write,
the web clock scenario after country disappearance, and the dead-belligerent
scenario failing to create its required state. Their follow-up corrections
await checkpoint 25; the original failing log is retained.

A [fixed seed-0 trace](evidence/2026-09-22-completion/political-calibration/iteration-24-seed0-trace-analysis.md)
finds six first and four repeated elected coups. No additional funding-threshold
or authority-reset defect is supported: the four affordable sub-threshold
increases already target loyalty above .397. The prospective plan, compact
case evidence and exact binary/library/log receipts are archived. This
diagnostic does not certify A1 or replace its failed ensemble result.
