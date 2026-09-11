# W6 successor case: original checkpoint available

Read-only source and original-file inventory; no simulator, loader, browser, command, build or server was executed for this note. Integration source and fixture inputs were not changed. Exact extracted metadata and before/after file hashes are in `w6-successor-checkpoints.json` beside this note.

## Bounded review route

Use a disposable copy of `fixtures/lifetime-legacy/saves/profile-idle_human-10.json` through the existing visible Saves/load confirmation. This original `spheres-campaign` v1 checkpoint is dated **1 February 2000**, player **USA**, seed **1990**, size **39,688,734 bytes**, SHA-256 **`5d6b022e8273c5bbd569d96cc9ac4de1f4086fa69de110f835c63718d7469b53`**. Both `ussr_dissolved` and `yugoslavia_dissolved` are already recorded; their predecessors are dead.

Find **Slovenia** for the nation view, then **Ljubljana** for near zoom and actual surface selection. Slovenia is already alive and owns 12 districts in this untouched checkpoint. The shipped `cities.js` entry places Ljubljana at longitude 14.515, latitude 46.0553. Russia (86 owned districts), Ukraine (25), Serbia (28) and Croatia (18) are also living alternatives. No advance to a guessed activation date or hand-authored successor fixture is needed.

The original 30-year checkpoint is also usable: **1 January 2020**, same player/seed and living successor/district counts, **51,282,713 bytes**, SHA-256 **`c5a61334fa8ce8d23983103a26b0a12e9d70deef60c7648077e9857b9cd31a5a`**. The 10-year file is the smaller bounded review input.

Recorded rules in both: daily simulation, resource market, production, manufacturing, military operations, logistics routes and physical logistics are true; AI aggression/crisis intensity are 1.0. New connected-economy/company-network/operational-warfare flags do not exist in these historical files. Preserve that on load; label the observation **a legacy technical USA checkpoint displaying a saved successor**, not an integrated long-campaign playtest. A map-only read/reload does not need a new rules adoption.

S01's `docs/campaign-certification/S01/PERFORMANCE_BASELINE.md:30–65` records their original source under `C:/Users/ridge/Documents/Codex/2026-09-04/ple/work/profile-campaigns/saves`, dates and matching hashes. This inventory recomputed hashes before/after reading; both matched. It does not reconstruct the original producing executable or claim a newly observed dissolution. Root must still verify actual visible load, selection and reload on the candidate binary.

## Why Namibia did not appear after 21 March 1990

- `spheres-sim/src/nations.rs:2593–2617` records Namibia's historical independence date as roster documentation and sets `start_1990` false. It supplies no timed activation command or event.
- `spheres-sim/src/districts.rs:50–56` explicitly states that Namibia and East Timor have map entries but **no birth site** or federation parent; their districts remain unowned until a mechanism exists. `world.rs:1156` repeats this limitation.
- The actual state-birth gates are USSR and Yugoslavia dissolution when stability is below 25 or separatism exceeds 0.9, once per dissolution flag (`politics.rs:202–215`). They are state-driven, not calendar guarantees.
- `/api/state` publishes only living nations (`spheres-web/src/main.rs:5504–5511`). Finder searches these names plus province names/IDs/current owners (`spheres-web/ui/decision-tools.js:4–9,62–74`). Namibia has no living row or province-owner match. A **Windhoek city** match exists independently in the static city catalogue, but viewing it would not prove a living Namibian successor or its ownership.

This is a pre-existing missing statehood mechanism, not a failed rendering repair. Retain it explicitly rather than labeling a city visit as successor coverage.

## Qualification ownership

W6 requires a successor's terrain/selection/reload evidence alongside high terrain, lake/coast, dense city and microstate cases (`S01/RUNTIME_AND_VALIDATION.md:144`). The original Slovenia checkpoint offers a genuine bounded saved successor for that observation; the fresh France March 1990 run does not supply Namibia.

The current published pathway assigns **government/succession qualification to S10** (`CERTIFIED_CAMPAIGN_PATHWAY.md:467`), map/province navigation to **S20**, and explicit fixtures/results for all 23 successors to **S24** (`:644–652`). S07 is currently construction/jobs/operating outcomes, so moving Namibia activation there would be a deliberate owner change rather than an existing roadmap assignment. No S06+ work is started by this note.
