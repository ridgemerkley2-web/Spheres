# Remaining-failure follow-up — 22 September 2026

Baseline: `be67c9b8785ec8a6919a8e354e536e8237b6c566`. This follow-up
corrects Algeria's opening national government and removes more redundant
town/facility geometry. It does not resolve the remaining art-budget or political
calibration gates by changing their targets.

## Algeria: seats are distinct from modeled support

The campaign previously installed FIS in the national chamber by applying a
national electoral formula to support proxies drawn from June 1990 local
elections. Those elections occurred after the January campaign start and did
not determine the national Assembly.

The [IPU's 1987 report](https://data.ipu.org/election-summary/PDF/ALGERIA_1987_E.PDF)
records FLN winning all 295 seats on 26 February 1987, with a five-year term.
The government-authored US State Department reports for
[1989](https://www.ecoi.net/en/document/1280981.html) and
[1990](https://www.ecoi.net/en/document/1324300.html) confirm that FLN remained
the only represented national party, despite legal opposition and the June
local election. This is the last election's allocation and party representation,
not a claim to have individually verified every January vacancy.

Newly initialized governments on exactly **1 January 1990** therefore inherit
FLN's full chamber. The ordinary coalition code selects FLN before character
identities are enrolled. No synthetic election, succession headline, political
ban or random draw is added. This applies to both monthly/headless and daily
campaign starts.

The seven existing party IDs, support values, army/pillar state and election
schedule remain. Support is still a normalized proxy from a later local poll;
it is not presented as measured January national popularity. December 1991
remains the model's historical election date, not a date already announced at
campaign start. An actual later election recalculates seats from current support
and can elect FIS normally.

Existing saved government records are never overwritten. A missing record on
January 2, February 1 or a later year retains the previous initialization
fallback. Tests cover both clocks, separate support/seats, other nations,
repeated initialization, divergent saved governments, save/load and a subsequent
election. The existing paid/hostile-army tests still exercise an explicit
FIS-winning result; the 24-month annulment scenario also passes.

## Measured baseline change

This deliberately changes political timelines. The old library independently
reproduces the previous default startup and 240-month fingerprints. Applying
only the new opening seats/coalition to that library produces exactly the same
new fingerprints as the newly compiled source.

The full serialized startup comparison finds exactly three changed leaves:
Algeria's coalition leader `dz_fis → dz_fln`, FIS seats
`0.788152122878798 → 0`, and FLN seats `0.21184787712120196 → 1`.
No other saved field, economic figure, support value or RNG state changes at
startup. This is not a claim that later economic or political trajectories stay
unchanged.

| Fingerprint | Before | After |
| --- | --- | --- |
| Default startup | `e26e4bf8d6c60066` | `6fc47dff64344b17` |
| Default 240 months | `0cbd02497c30957c` | `4d2b187b81788c76` |
| Seed 0, 240 months | `8834ad709d4bf805` | `95e5fe35215c4fc9` |
| Seed 1, 240 months | `bd3f3e335fb6161c` | `7bbec62d113c034b` |
| Seed 2, 240 months | `9120a2ff805b184f` | `b3e5a927a2f9e509` |
| Seed 3, 240 months | `de17f4fdef2c0d7f` | `75c7ea1b2a4b46f8` |
| Seed 4, 240 months | `eb92ae6a6418b12e` | `9a08dffbeab9da85` |
| Seed 5, 240 months | `ef75c8dcbe4335b5` | `a9a05daae84bd1bb` |

Only active default-world fingerprints and the exact opening census expectations
were updated. Feature-on/off invariants, calibration limits, historical evidence
and election algorithms remain. The first broad unit run recorded 880 passes
and nine expected old-hash/census failures before these baselines were updated.

[Independent probe output](evidence/2026-09-22-followup/algeria-probe.jsonl) ·
[Probe source](evidence/2026-09-22-followup/algeria-probe.rs)

The opening bloc census is now `[67,17,8,2,43]`: Algeria moves from Islamist to
Nationalist, leaving only Iran and Sudan Islamist. **P-6 still fails** on the
Communist count (17 versus 11–13) and Libya's Non-Aligned classification. Its
assertions remain unchanged. The six dynamic A1–A5/A7 gates remain separate
calibration work; fixing national seating is not a substitute for that work.

The explicit post-fix runs retain all six dynamic failures:

| Gate | Before | After | Unchanged target |
| --- | --- | --- | --- |
| A1 coup distribution | Median 6; top-three share 1.00 | Same | Median 4–14; share below 0.50 |
| A2 Islamist non-ballot takeover | 0/12 seeds | 0/12 seeds | 7–10/12 seeds |
| A3 Communist non-ballot takeover | 10/20 seeds | 14/20 seeds | At most 2/20 seeds |
| A4 democratization | Median 1 | Median 1 | 15–40 |
| A5 successor-party ballot return | 0/12 seeds | 0/12 seeds | More than 6/12 seeds |
| A7 ballot/takeover ratio | 0.160 | 0.233 | At least 3 |

A3 worsens; A7 improves but stays well below its target. No rates are tuned in
this correction. These fixtures use monthly simulation, default economics and
ideological takeovers enabled; current browser campaigns use daily simulation
with that takeover switch disabled. They are useful unresolved calibration
evidence, not measurements of event frequency in the current campaign mode.

## Additional geometry reductions

Town meshes remove covered sash-jamb caps/rears, chimney geometry enclosed by
opaque rims/corbels, and university rear windows completely inside attached
wings. Visible facade edges, trim, roof silhouettes and detail tiers remain.

| Configuration | Before | After | Ceiling |
| --- | ---: | ---: | ---: |
| Row house | 17,904 | 16,382 | 12,000 |
| University | 19,046 | 16,656 | 12,000 |
| Mixed block | 175,277 | 163,671 | 150,000 |
| Residential block | 184,684 | 174,540 | 150,000 |

The 23 existing town checks, 180,766 exterior-ray comparisons and eight
university width/storey/detail cases pass. Row-house and university comparison
renders are pixel-identical at the two reviewed 512-pixel views.

Facilities merge coplanar rafter subdivisions and remove four duplicated
perimeter posts. This saves 6,016 triangles across sixteen complete level-five
facilities. Their new totals range from 29,828 to 38,646, still above 12,000.
All 800 far meshes remain byte-identical, as do 21,030 unaffected near parts in
the 1,600-configuration comparison. The 24 focused site checks pass. Twelve
before/after views differ only at rasterization edges; the worst comparison has
45 of 480,000 pixels differing by more than two channel levels.

Across the measured near inventory, 38,786 triangles were removed. This is an
offline geometry count, not a measured frame-rate improvement. Equipment
inspection assets and budgets are unchanged. The strict art gate remains
**77 pass / 35 over / zero under**: 15 equipment, 16 facilities and four towns.
The four generated P0 records reproduce and the 33-asset manifest is current.

[Town results](evidence/2026-09-22-followup/town-results.json) ·
[University before/after](evidence/2026-09-22-followup/university-before-after.png) ·
[Site matrix](evidence/2026-09-22-followup/site-matrix.json) ·
[Site render comparisons](evidence/2026-09-22-followup/site-renders.json)

## Verification

- Native simulation unit/integration suite: **1,351 passed, zero failed, 69
  existing ignored tests**. The isolated resource timing test adds one pass:
  **1,352 native simulation checks passed** in total. Its 0.0617 ms/month
  reading meets the unchanged 0.15 assertion; the aspirational 0.05 target was
  not met on this machine.
- Full JavaScript suite: **1,677 passed, zero failed or skipped**.
- Leadership production checks: **eight self-tests passed**; census: **seven
  tests passed**. Production, census and research freshness checks pass.
- Leadership generated diffs contain only the government source byte count/hash
  and dependent inventory hash. Country/party/art records and other generated
  C01 files remain byte-identical.
- All eight protected files/saves and both original worktree heads remain
  unchanged. No original preview process was stopped or replaced.
- Explicit optional political run: **seven still failing**, with the updated
  measurements above. Strict art budget run: **35 still over**.
- Full web/native Windows and Ubuntu CI must verify the published commit;
  the local native run above covers `spheres-sim`, not the complete workspace.

[Validation record](evidence/2026-09-22-followup/validation.json) contains the
exact check outcomes, including the still-failing additional gates. Required CI
and optional calibration/art gates must
be reported separately; passing the required suite does not certify those
additional gates.
