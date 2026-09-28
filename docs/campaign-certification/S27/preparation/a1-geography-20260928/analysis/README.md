# A1 geography: recorded firing guards across all electoral countries

Codex `/root/review_gap_submission` independently analyzed the already-retained **seed 0, 252-month** observer output at source `6818e4f0d94b01c86d7a9acc4252260947d13504`. This is a new read-only analysis, not a simulation, coefficient trial, holdout, A1 pass, or qualification. No repo, native source, or original evidence was changed.

**Conclusion:** the narrow geography is not simply an absence of crisis elsewhere, and this run does not support government AI buying away political penalties across otherwise coup-prone non-firing countries. Crisis plus sufficiently low *actual* loyalty seldom coincide outside the six firing countries. Most surviving crises retain a resource-supported target above the line without any recorded funding response. This better localizes the model question, but establishes no new functional contract defect or justified repair. A1 remains unresolved.

## Which guards operated

The 146,525 snapshots include **25,769 electoral trigger checks in 118 country IDs**; 40 other observed country IDs never reached this electoral route. Successors can exist during the run, so these counts are not the 137-country opening roster. Authoritarian/non-electoral coup mechanisms are outside this route and this count.

| First actual branch | All electoral checks | Countries with no elected coup |
|---|---:|---:|
| Army absent, unsettled, or interim | 6,786 | 6,417 |
| Live loyalty/discontent conditions inactive | 18,870 | 18,476 |
| Both live conditions, pressure below 1 | 103 | 1 |
| Firing | 10 | 0 |

For all electoral checks, nonexclusive blockers were Army absent **3,972**, unsettled **3,421**, interim **449**, loyalty >= .35 **25,495**, discontent < .25 **18,677**, and pressure < 1 **25,754**. These overlap and must not be added. The precise 18 joint combinations are retained. After Army/tenure/interim eligibility, the joint live conditions were **14,843 loyal/low-discontent**, **3,945 loyal/crisis**, **82 hostile/low-discontent**, and **113 hostile/crisis**. Ten of the latter fired.

The **112 non-firing electoral countries** divide descriptively as follows, without selecting or changing any outcome test:

- **17** never carried an Army at these checks.
- **4** carried an Army but never passed unsettled/interim eligibility: Egypt, Kazakhstan, Kenya and Yugoslavia. Their electoral windows contain only 3, 6, 14 and 5 checks respectively.
- **62** reached eligibility but no eligible crisis. Of these, **55** never had discontent >= .25 at any electoral check (including Pakistan and Thailand); the other **7** had crisis only outside eligibility: Algeria, Honduras, Israel, Mexico, Mongolia, Paraguay and Poland.
- **28** reached eligible crises but never had hostile loyalty during those crises.
- **1**, El Salvador, reached both conditions once, without enough pressure.

`reports/all-country-table.md` lists all 118 countries, their actual and overlapping guards, extrema and funding counts. `countries.json` retains exact joint counts and dated original-line witnesses per country. The 40 IDs without electoral checks are explicitly listed in summary.json; lack of route exposure is not relabeled as a peaceful or historically verified government.

## What explains survival during crises

The 29 non-firing countries with eligible crises contribute **3,552 eligible crisis checks**. Their army target was below .35 in only **2** checks, both El Salvador; in **3,550** it was at or above the line. **2,390** checks already had a fully provisioned model resource basket; 16 countries were saturated during every eligible crisis check. **629** checks had zero confidence penalty, including **433** with zero executive leverage. These are simultaneous observed states, not treatment-effect estimates.

Crucially, **none of those 29 countries received any recorded `ai_government` paid military funding increase during the entire run**, including their non-electoral periods. All 151 observed increases belonged to 13 other country IDs. Thus no new appropriation responding to confidence loss is needed to explain the loyalty floor in these non-firing trajectories. This does not claim that their original budgets, output changes, fiscal consolidation or other systems were irrelevant; existing resources and the implemented confidence channel are exactly what set their targets.

A same-snapshot algebraic decomposition identifies **12 paid increments** containing spending above both the existing military share and the material/war-only .40 inverse: **9 Myanmar, 1 Comoros, 1 Guatemala, 1 Sao Tome**. All four countries are already among the six with coups. This decomposition changes no world and is **not** a rerun of the rejected policy. It helps explain why removing compensation can increase repeat events without spreading first events. It does not prove a cross-seed causal effect.

Examples from the retained trace:

- **Peru:** 211 eligible crisis checks, no paid increase, minimum target .369766869, minimum actual loyalty .380764724, maximum confidence penalty .179740804, pressure always zero.
- **Philippines:** 169 eligible crisis checks, all resource-saturated; minimum eligible-crisis target .643270892, minimum actual loyalty across all checks .657434411, maximum penalty .229318684, pressure zero.
- **Azerbaijan:** 160 eligible crisis checks, all resource-saturated; minimum eligible-crisis target .366562500, minimum actual loyalty .521612751, maximum penalty across all checks .492879171, pressure zero. Distinct extrema are not presented as if they occurred together; exact rows are retained.
- **Pakistan and Thailand:** zero crisis checks across 252 electoral checks each; no funding response and no pressure. Changing political coefficients cannot legitimately invent the missing crisis exposure.

## El Salvador: the one additional live near miss

At 1991-12-01, PDC had 12 settled months, loyalty **.33880055075691395**, discontent **.40406468087279657**, and pressure **.16178330466356525** (line 11740): pressure prevented firing. By January, the ordinary party transition had seated ARENA, its observed record age was 1, the confidence penalty was zero, and new-party grace applied. Pressure reached its overall maximum **.24942763380274532** in February while the new government was still protected; loyalty recovered above .35 by March and pressure cooled. Military share stayed .034 and fiscal headroom remained zero. This demonstrates an existing accountability/tenure and recovery mechanism, not hidden cash rescue or a same-party reset bug. The sequence is preserved verbatim in `el-salvador-sequence.json`.

## Comparison with already rejected trials

The retained 28 September inverse-removal trial explicitly removed confidence from the funding inverse while preserving coefficients and gates. Its original12 result worsened median coups **7 -> 12** and displayed top-three share **.59 -> .67**, while A1 remained failed; it was rejected and source restored. The present observation that extra compensation spending is confined to already-firing countries is compatible with that result. It is not a reason to repeat the experiment or infer its exact per-country changes, which this observer did not record under the rejected policy.

The retained confidence-weight experiments already tested stronger political penalties. The earlier `.90` and `1.20` comparisons failed additional focused/outcome gates; the later corrected-base confidence reassessment27 again changed only `.65 -> 1.20`, failed Algeria's unchanged chronology, A1 at **12/.50** (strictly below .50 required), and A2 at **6/12** (more than six required). The former coefficient was restored. Raising it again based only on the resource ceiling arithmetic would repeat a rejected design intervention, not repair a demonstrated omission.

The compact original trial plans/results/patches and calibration history are copied unchanged under `prior-trials/`. I did not execute those candidates, inspect reserved cohorts, change their criteria, or reinterpret their failures as passes.

## Limits, provenance and reproduction

Counts are repeated **country-month exposure**, not independent samples. They cover one already-used development seed in legacy monthly mode, not all development seeds or daily play. The world/return-headline/RNG parity is the native observer's retained claim; the prior packet independently checks its final pair and recorded arithmetic. This extension independently rehashed the gzip and exact decoded 240,585,752 bytes, checked all raw stage counts and firing rows against result.json, and derived the per-country counts. It does not independently replay 252 worlds.

Exact decoded observations SHA-256: `13676236845d5c814e640520d322227769d4a7e16c685a237a6ce99a2ca46b49`; portable gzip SHA-256: `b39a00d7c6eb7fef5f5f20ce105b64b4f33ec7440a45560d7177bd78d8a49186`; original result SHA-256: `28b9e9177d800e78566c2bec8a878245515932e342c5304ea2f1bcaf6bfca6f6`. The original execution.json source/binary binding and source raw/Git-byte pins are retained under inputs; no compiler or binary execution was performed here.

To regenerate the main portable analysis from the existing observer packet (raw JSONL also accepted), choose a **new** output directory:

```text
python -B -X utf8 analyze_geography.py --observations <observer-packet>/original-run/observations.jsonl.gz --result <observer-packet>/original-run/result.json --out <NEW-report-directory>
```

The supplementary resource_context.py records the local input paths used for its read-only resource/witness pass. Its outputs and exact script are pinned. No raw observation duplicate is stored here; it remains complete in the existing portable observer packet. Source references remain government.rs:8488 (resource target), :8566 (confidence), :8832 (settled tenure), :8856 (walk/pressure), :8926 (live trigger), and :9365/:9398 (funding inverse/AI) at the pinned observer revision.
