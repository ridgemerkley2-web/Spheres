# Failure repair follow-up — 22 September 2026

Baseline: `ca00e02b59fa87b408d482d8f105dce5b047c0c0`, whose required
[Windows and Ubuntu CI](https://github.com/ridgemerkley2-web/Spheres/actions/runs/35726228948)
passed. Additional audits still exposed terrain regeneration failures, 42 art
budget overruns and seven ignored political calibration failures. This follow-up
addresses reproducibility and avoidable geometry; it does not change game balance
or raise the art ceilings.

## Terrain reproduction

The detailed lake shores integrated in S04 originated in `541a6b11`, which used
Douglas-Peucker tolerance 0.012 canvas units. The water-surface generator still
used the original river tolerance, 0.4, so none of its six derived paths matched.
It now explicitly requests 0.012 from the same projection helper. The ordinary
river default stays 0.4. Exact identity matching remains mandatory, including
rejection of missing, duplicate, empty and ambiguous shores.

All six paths, indices, source identities and water elevations reproduce without
changes. Regeneration changes only an outdated whole-file rivers digest in the
provenance table. Rivers, the lake mask and the terrain-surface runtime are
unchanged.

The terrain tile embed file now has an LF checkout rule. With the recorded
Python/NumPy/Pillow/h5py runtime, the full source check reproduces all 608 PNGs
(133,418,791 bytes), 40 constant tiles, manifest and Rust embed. Generator hashes,
encoder versions and manifest serialization also match. The PNG comparisons
remain exact; no tolerance or pixel-only substitute was introduced.

The ten standard-library lake tests run in CI on both operating systems.
Fifteen existing terrain tile unit tests also pass locally. Full source
regeneration still needs the original NOAA and HydroLAKES downloads.

## Town geometry

Remove frame faces enclosed by opaque glazing/reveals, one concealed duplicate
glazing bar, and covered interior railing caps. Preserve projecting end caps,
openings, panes, bevels, materials and all existing detail levels.

| Formerly failing configuration | Before | After | Ceiling | Result |
| --- | ---: | ---: | ---: | --- |
| Mid-rise apartment, maximum size | 14,540 | 11,768 | 12,000 | Pass |
| High-rise apartment, maximum size | 12,028 | 9,500 | 12,000 | Pass |
| Civic building, maximum size | 13,637 | 10,445 | 12,000 | Pass |
| Commercial block | 151,008 | 129,734 | 150,000 | Pass |
| Civic block | 161,077 | 135,157 | 150,000 | Pass |
| Row house, maximum size | 21,222 | 17,904 | 12,000 | Still over |
| University, maximum size | 24,252 | 19,046 | 12,000 | Still over |
| Mixed block | 199,331 | 175,277 | 150,000 | Still over |
| Residential block | 214,044 | 184,684 | 150,000 | Still over |

The 23 existing town checks pass. The new regression compares 90,383 rays
against closed-solid references, including grazing views and rail ends. Two
512×512 WebGL comparisons change only 40 pixels for the row house and 20 for
the residential block, where overlapping surfaces were removed. These are
representative visual checks, not exhaustive images of every possible town.

[Town measurements](evidence/2026-09-22/town-optimization-result.json) ·
[Row-house comparison](evidence/2026-09-22/town-row-house-before-after.png) ·
[Residential comparison](evidence/2026-09-22/town-residential-block-before-after.png)

## Tactical aircraft

Catalogue and map meshes now sample small stores, fairings and ducts more
economically. Solid fans, connected intake lips/liners, canopy, wing and
fuselage sampling remain. An interior-geometry check removes only faces whose
whole convex hull lies inside an inscribed fuselage cylinder, accounting for
the actual cockpit opening. Exposed taper tips are retained.

| Configuration | Catalogue before → after | Map before → after |
| --- | ---: | ---: |
| Default | 14,904 → 10,936 | 1,696 → 1,392 |
| Fully loaded, efficient engine | 15,704 → 10,896 | 1,908 → 1,420 |
| Fully loaded, twin engine | 16,828 → 11,900 | 1,984 → 1,488 |
| Unchanged ceiling | 12,000 | 1,500 |

All 576 valid tactical configurations at both coarse levels pass: 1,152 builds,
with maxima 11,932 and 1,488. Inspection meshes remain byte-identical for all
three reviewed presets, including the 228,640-triangle default. All fighter
and light-attack default/loaded meshes remain byte-identical at every LOD.
The 62 focused mesh/export checks pass. Sixteen before/after Chrome views
render without page errors; coarse geometry remains intentionally faceted
when enlarged in the workshop.

[Aircraft matrix and preservation results](evidence/2026-09-22/measure-final.json) ·
[Catalogue before](evidence/2026-09-22/before-default-lod1-hero.png) ·
[Catalogue after](evidence/2026-09-22/after-default-lod1-hero.png) ·
[Coarse intake before](evidence/2026-09-22/before-default-lod1-intake.png) ·
[Coarse intake after](evidence/2026-09-22/after-default-lod1-intake.png)

Exactly four tactical coarse hashes were refreshed in the S18 snapshot,
plus the two tactical coarse entries in each of the older tank/specialist
regression tables. The first combined run caught those two stale tables
(1,673 pass / two fail); all unaffected inspection, fighter, light-attack and
ground pins remain. The shipping model README now derives its triangle counts
from the generator.

## Remaining art gate

The reproducible budget record improves from **70 pass / 42 over** to
**77 pass / 35 over**, with no under-detail results and no changed ceilings.
The full art budget gate therefore still fails: 12 ground inspection
configurations, three aircraft inspection configurations, 16 near facility
configurations and four town configurations remain over. Inspection fidelity
was not reduced to force those older ceilings.

Current counts: [P0 budgets](../../art/P0_BUDGETS.md) and
[machine-readable measurements](../../art/P0_MEASUREMENTS.json).
`bench_art.cjs --check-records` only verifies reproducibility; it is not a
passing result for `bench_art.cjs --check`.

## Political failures remain visible

The six dynamic gates A1–A5/A7 simulate 252 monthly ticks with the default
economy and ideological takeovers enabled. Browser campaigns explicitly use
daily simulation with ideological takeovers disabled. These legacy calibration
failures remain useful, but do not measure event frequencies in current
campaign mode. The character system assigns identities after succession; it
does not choose election winners or repair those calibration rates.

P6 also exposes a current startup issue: its static government classification
uses the same tables as the browser. Algeria's table explicitly describes June
1990 local-result shares while assigning initial national seats. Correcting
that needs reviewed historical national-government data and focused startup
validation. The old targets also require independent historical review.

No political assertions, targets, initial shares, coup probabilities or election
behavior were changed. The seven earlier failures are not reclassified as
passes. The useful next political repair is accurate January-start national
seating, followed by a separately defined current-campaign calibration study.

Source references:
[legacy census](../../../spheres-sim/tests/bloc_census.rs),
[campaign startup](../../../spheres-web/src/main.rs),
[government tables](../../../spheres-sim/src/government.rs),
[identity succession](../../../spheres-sim/src/party_leadership.rs).

## Combined validation

- Complete JavaScript suite: **1,675 passed, zero failed, zero skipped**,
  including the available real archived advisor fixture.
- Lake tests: **10 passed**; terrain tile tests: **15 passed**.
- Strict lake/source and complete terrain tile regeneration: **passed**.
- Art measurement reproducibility and the 33-asset manifest: **current**.
- Component coverage: **191 distinct pairs**, zero weak or absent; record current.
- Thirteen committed model exports reproduce byte for byte; only their README
  changed.
- The eight protected original files/saves and both protected worktree heads
  remain unchanged.

[Machine-readable validation](evidence/2026-09-22/validation.json) records the
edited generator hashes and bounded results. The full art budget gate remains
**failed**, as documented above. Political simulation source is unchanged and
those seven failures were not rerun for this art/terrain-only repair.
This local record does not claim a new native build or hosted-CI result.
