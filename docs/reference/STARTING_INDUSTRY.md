# The 1990 industrial starting layer

Local implementation, 2026-09-04. Not pushed. Ridge approved game-capacity
estimates after the historical-inventory audit, and approved replacing Pensions
with Welfare. These are separate changes; neither rebases national GDP.

## What the new campaign contains

All 137 canonical starting countries receive five inherited manufacturing groups:
food and textiles, materials, chemicals, machinery and electronics, and other
manufacturing. The source artifact is `spheres-sim/data/industry_1990.json`.
Historical manufacturing shares size the existing game GDP where available;
missing coverage uses an explicitly labeled model fallback, never a claimed
historical zero. Sector classification and source footnotes remain important:
combined or missing observations are not five independently measured industries.

The shipped source coverage is **133 sourced 1990 manufacturing/GDP shares**;
USSR, Yugoslavia, Czechoslovakia and Taiwan use the explicit 20% model fallback.
Industry mixes have **3 complete observed profiles, 59 partial model profiles,
and 75 generic fallback profiles**. A partial profile may estimate at most five
uncovered divisions using the median of its positive, uncombined observations,
after resolving unambiguous same-group aggregates. Original missing values stay
null. Generic mixes use five equal weights. None of these observations supplies
literal factory counts or historically measured province locations.

Source requests and raw observations are retained in the artifact. The collector
and source-quality tests are documented in `tools/industry/README.md`; the final
artifact was reproduced byte-for-byte from its cached raw responses.

The game conversion is explicit and adjustable in one place:

- One factory equivalent represents **$100 million of annual value-added
  capacity**, not one literal factory building.
- Opening utilization is an **80% game assumption**, not a historical statistic.
- A group's opening value added is national opening GDP × manufacturing share
  × group weight. Its equivalents are that output divided by $80 million.
- Fractional equivalents are preserved, including small economies. No minimum
  whole factory is granted and no small positive value is rounded to zero.

Mapped provinces receive population-weighted allocations. These are **modeled
locations**, not an industrial census. Bahrain, Mauritius, Seychelles, Comoros,
Cape Verde and Maldives retain unallocated national capacity until geography is
available. The records retain their origin when land changes owners. Starting
quantities are frozen: growing GDP/population never silently builds more assets.

## What these factories do—and do not do

They describe the manufacturing economy already inside national/provincial GDP.
Current output follows that inherited GDP account; estimated utilization can
exceed 100% if output outgrows the fixed starting estimate. Other GDP sectors
retain their relative model shares, rescaled into the non-manufacturing remainder.
They are not newly verified historical agriculture/service accounts.

**Inherited equivalents are not free stockpile-pack factories.** They do not grant
cash, raw resources, finished goods, power or construction authority. They do not
consume the resource stockpile a second time or award another GDP increment.
Funded Materials Processing and Machinery Works remain recipe-driven facilities
for construction/trade packs. The subsequently approved Materials operating pilot
can commission a finite, paid conversion order against inherited Materials
capacity; see `MATERIALS_OPERATIONS.md`. This requires actual government inputs,
power and ministry funding and replaces the represented inherited GDP share.
Food, chemicals and other inherited
groups do not yet have separately buildable physical product chains.

This distinction avoids counting the whole inherited economy as freely available
government supplies, or stalling it because a newly introduced stockpile is empty.
Converting the remaining sectors into detailed operating chains is later work and must
replace their inherited accounting, not add on top of it.

## Investment association

The national capacity plan now includes a separate **annual value-added** sector
comparison. It counts inherited capacity, funded pack-plant potential and queued
extra potential once. Pack-plant potential uses the existing constant-price
output-minus-input recipe; it is not achieved GDP, cash or an export order.
Actual recorded output is distinct from this capacity ceiling.

The 25% planning buffer matches the 80% opening utilization assumption, so merely
enabling the baseline does not manufacture an expansion gap. Structural pressure
ranks Materials versus Machinery investments only when real pack demand already
justifies them. It never supplies imaginary packs or blocks the first usable
pack supply chain. Existing inventory, deliveries, input/funding blockers,
queued work and physical project limits still govern whether anything is built.

## Saves and visibility

Only fresh daily browser campaigns seed estimates, before provincial accounts
open. Loading an older campaign does not backfill 1990 assets into the present.
New saves persist frozen quantities and source profiles, so a later data update
cannot rewrite an ongoing campaign. Default headless worlds remain unseeded.

The Exchange shows the five-group overview and a closed capacity comparison.
Country/province economy dossiers expose the same Rust-owned estimates in closed
details panels, including source quality, location assumptions, fractional
equivalents, annual output, and utilization. Reads cannot alter a campaign.

## Ministry of Welfare

The Pensions ministry is now displayed as **Welfare** throughout the cabinet and
department budget views. Slot 3, the internal/wire `pensions` key, the 20%-of-GDP
cap, allocations and existing save arrays are unchanged. Its five managed
departments remain retirement benefits, disability benefits, survivor benefits,
minimum-income supplements, and benefits administration.

This does not invent five new calibrated mechanisms: current total-budget
effects remain standing, stability and pension-related labor-force withdrawal.
The unemployment explanation now explicitly says this is **not job creation**.

## Verification contract

Tests cover the canonical roster and explicit source gaps; fractional/mapped and
unallocated records; preserved opening GDP, cash and physical stock; no reseeding
on reads/load/growth; transfer and save continuity; potential versus queued/actual
capacity; browser new-campaign activation; legacy Welfare payloads; and escaped,
responsive UI renderers. The historical audit remains a before-change record,
not a claim that its then-empty browser baseline is still the current feature.

### Final local verification — 2026-09-04

- `cargo test --workspace --release --no-fail-fast`: **666 passed, 3 known
  failures, 55 ignored**. No new failures. The existing Belgium endowment
  comparison remains `0.001851` versus `0.001749`; the existing start/run hash
  actuals remain `0xe26e4bf8d6c60066` / `0xbe94d6125631829c`. Pins and tolerances
  were not changed.
- All **170** serverless UI checks and **11** collector/data checks passed.
- New model/planning/real-AI-priority integration tests: **16 passed**. Browser
  new-campaign and Welfare compatibility tests also passed in the full suite.
- Live Edge checks passed at 1440, 820 and 390 pixels: five-group overview,
  USA/Japan/Tonga/Bahrain GDP reconciliation, Welfare, all Exchange tabs,
  137-country filtering, exact-size Tonga workshop orders, and lost-response
  retry. No page errors or tested horizontal overflow. Desktop/mobile screenshots
  were visually inspected.
- Regressions were observed failing with the old uniform source mix, old fixed
  AI ordering, old Pensions label, or an intentionally removed construction/
  manufacturing distinction, then passed after restoring the implementation.
- Release preview: run `cargo run -p spheres-web` and use the printed local address.
  New estimates require a fresh campaign; older saves are intentionally not
  retroactively seeded. Local changes only; no Git commit or push.

The ordinary regression suite is not a new multi-seed long-horizon calibration
of the opt-in inherited-capacity layer. That broader balance exercise remains
separate from these accounting, integration, replay and browser checks.
