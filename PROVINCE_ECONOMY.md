# Province economies — local review, 3 September 2026

Ridge approved a worldwide GDP foundation: every country has an economic
composition, its mapped provinces have local accounts, and the current project
systems appear in those accounts. This work remains local until review.

## What the numbers mean

GDP is an **annual output rate in billions of 1990 dollars**, not treasury cash,
company profit, cumulative investment, or the value of everything in storage.
The country's existing GDP is the starting anchor. Province allocations and
eight sector shares are explicitly **modeled game estimates**, not a new claim
to have sourced historical provincial accounts. They can be replaced with better
data later without changing the project accounting interface.

The eight starting sectors are agriculture, extraction, manufacturing, utilities,
construction, transport, services, and public services. Population-weighted
province allocations are reconciled to the national anchor. Countries without
mapped provinces retain an explicit unallocated national account; their output
does not vanish. Existing economic activity is not represented by invented
factory objects or free starting inventories.

The initial preset is deliberately shared, not a claim about each country's
historical specialization: agriculture 8%, extraction 6%, manufacturing 20%,
utilities 4%, construction 7%, transport 10%, market services 30%, and public
services 15%. `MODEL_SECTOR_SHARES` in `province_economy.rs` is its single source.
Project output changes the resulting mix. Country-specific sector profiles and
more accurate provincial weights are the next data refinement, not implied
by this first foundation.

## Project coverage

| System | GDP treatment |
| --- | --- |
| Construction of all twelve project kinds | Actual installation/labor work contributes while building. Materials are not counted again as construction value added. Completion alone pays nothing. |
| Mine development | Construction work while building; afterward, only the new non-oil extraction flow has an incremental valuation. Missing commodity prices are disclosed. |
| Materials processing | Realized output less raw and power inputs. Full storage or missing inputs limits output and its contribution. |
| Machinery works | Realized capital-goods output less intermediate packs, copper, and power. |
| Power generation | Only dispatched power earns a contribution, located among available generating sites. Factories deduct the same power value. |
| Industrial estates, roads, grids, warehouses, freight terminals | Enabling assets. Their effect and level are visible; empty capacity does not invent output. |
| Automation and efficiency | Their benefits appear through the plant's realized output/input use, never a second bonus. |
| Research centers | Construction contributes while built. Completed center levels are currently reserved capability data, not a connected research-output multiplier or fabricated laboratory sales. |
| Arms plants and directed military procurement | Capacity and order commitments remain visible. Placing a long-lead order is not equipment delivery or a new GDP award. |

A manufactured pack is valued using a documented **game price**, not a sourced
historical commodity quotation: intermediate pack $100,000, capital-goods pack
$250,000, modeled power unit $10,000. Existing raw-resource reference prices
remain authoritative for their input units. Wages are part of value added, not
deducted as if GDP meant profit. Inventory production can contribute once;
using or selling the same inventory does not recreate its original value.

## Connection to the existing economy

The daily ledger records actual successful work and production. Daily value
added is expressed as a fixed 365-day annual equivalent, so a constant factory
does not appear to grow merely because February follows January. This is a
run-rate, **not** a trailing-year statistical GDP series. The prior project
component is replaced, not added afresh forever. Changes in capacity utilization
can therefore raise or lower the reported rate.

Mines retain their source annual-production benchmark while daily quantities
follow the existing resource posting calendar. Their receipts follow that
physical ledger: current resource rules do not suspend completed-mine output
merely because a province is contested. This GDP work does not add a conflicting
second blockade rule. Oil stays inside the existing national oil economy.

For enrolled department budgets, explicitly represented investment no longer
also buys the old aggregate public-capital boost. The inherited public-investment
reference is retained for the background economy; private investment and the
existing macroeconomic conditions remain there. Legacy unenrolled/headless
worlds keep their previous accounting path.

GDP does not deposit its whole value into the treasury. Existing tax and fiscal
systems continue to read national GDP on their normal daily schedule. Ownership
and succession use the game's existing national transfers; local accounts are
reconciled afterward, without issuing a second transfer.

When an inherited settlement values a captured project's GDP below its prior
accounting value, the ledger preserves that settlement and records a persistent
valuation adjustment. Its gross and intermediate dollar valuations adjust
together; physical quantities and treasury payments do not change. Stopping,
restarting, or reloading the project cannot erase the adjustment and create a
windfall. Existing completed assets first seen in an upgraded save are split
out of the opening economy, not rewarded with a second opening GDP grant.

## Review surface and extension points

Open any province for its annual GDP, inherited-sector composition, and project
impact. A country's Economy dossier provides the national composition and all
its mapped provinces, including countries not controlled by the player. Figures
are computed in Rust and refreshed with the game date; viewing never advances
the simulation or enacts a budget.

Future work can replace the estimated sector mix, add demand and sales, introduce
employment and wages, or price freight and research services. Those systems must
post their own value-added flows and must not count intermediate output twice.

## Local verification

- 13 provincial-accounting tests and 10 project-adapter tests pass, including
  daily replacement, intermediate-value conservation, no-project inertness,
  captured small-country accounts, migration, and save/resume.
- 118 UI checks pass, including a complete daily map rebuild retaining province
  disclosures, asynchronous response guards, narrow layouts, and safe labels.
- The web suite passes 119 tests (2 intentionally ignored).
- The broad simulation run passes 378 tests with 52 ignored and only the three
  existing baseline failures; the final migration edge also passed its focused
  rerun afterward. The known failures are the 1990 technology-growth baseline
  and the two golden pins. Their actual hashes remain `0xe26e4bf8d6c60066`
  (start) and `0xbe94d6125631829c` (known run); no pins were changed.
- Browser/API checks covered all-country reconciliation, foreign provinces,
  funded construction receipts, 414px layout, and a clean browser error log.
  Testing used a separate preview and did not write `save.json`.
