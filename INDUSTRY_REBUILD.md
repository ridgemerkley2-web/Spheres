# Construction and industry rebuild

Approved 7 September 2026: rebuild the system around visible project purposes
and integrate the result with the other Spheres GitHub changes.

## Decisions and effects

The catalogue groups projects by purpose. Every card explains the problem it
solves, its costs, its operating requirements and its province eligibility.
Effects reviews use simulation calculations rather than JavaScript estimates.

| Project | Direct result |
| --- | --- |
| Civilian Factory | More assignable construction capacity, subject to staffing and power |
| Office District | Service output and a taxable GDP base, limited by jobs, inputs and demand |
| Military Factory | Production slots for equipment and ammunition |
| Shipyard | Separate coastal naval production slots and more efficient funded naval maintenance |
| Power Plant | Electricity when generating fuel and operating funds are available |
| Materials Plant | Prepared materials for installation, machinery, offices and trade |
| Machinery Works | Machine/tool packs for installation, research and trade |
| Advanced Industry | Distinct advanced components for modern equipment and trade |
| Infrastructure | Faster local work and better freight throughput |
| Research Center | Prototype work for assigned research, with scientists and funded inputs |
| Mine/oil field | Additional output from a mapped deposit |
| Freight Terminal | More throughput at an existing coastal gateway |

Grid, warehouse, automation and efficiency investments appear as facility
upgrades. The standard starter preset atomically queues a Civilian Factory,
Power Plant, Power Grid and Materials Plant as four independently managed jobs.
Fractional workshops remain for smaller economies and old campaigns; quotes
name and price their components.

## Conserved construction and operation

MODEL scale: ten capacity performs one work day, each site accepts at most
twenty, and local infrastructure adds 10% work speed per level. Inherited
industrial estimates supply the starting construction base, with a ten-capacity
minimum for small countries. Working civilian factories add ten per level.

Auto allocation follows priorities and per-site caps. Manual requests reserve
capacity first; zero pauses work. A loss of national capacity scales existing
manual grants proportionally. Buildings and funded mines share the same opening
daily pool: completed work cannot spend its capacity again on another job later
that day. Changes after a settled date take effect on the next funding day.

Progress is limited by capacity, cash and installation inputs. Every input and
payment follows actual progress; incomplete bundles consume nothing. Prepared
materials can replace the iron input and machine/tool packs their copper
equivalent. Both share the existing warehouse stock and trade system. Shortages
remain visible in forecasts even when they stop today's work.

New facilities share the available hiring pool and qualified workers. Power
consumers share generation and local grid capacity. Civilian factories,
military factories, shipyards and research support settle their funded service
before dependent work uses it. Processing, machinery and inherited Materials
contracts retain their existing physical operating ledger; new office and
component production uses remaining power, inputs and funds.

Offices post actual service value added through the province GDP ledger once;
tax previews use the existing fiscal calculation. GDP is not a direct treasury
payment. Advanced components use a distinct stock and the existing contract,
escrow and freight system. Technology-gated catalog equipment and custom
fabrication require them. Naval lines consume shipyard slots separately from
land/air production. Operating shipyards reduce the naval portion of actual
maintenance invoices by a MODEL 5% per level, capped at 25%; this does not
restore age, create ships or alter custom equipment's frozen upkeep.

## Inherited economy and compatibility

The common industry view shows installed capacity, utilization, workers, power,
actual dated output and limiting reasons. It distinguishes new-facility demand
from inherited employment and utilities. Inherited 1990 manufacturing is
reconciled to its existing sourced/proxied GDP account. Its background supply
is not free government stock or another GDP award. Active inherited Materials
contracts use the same shared inputs and power as new activity. Factory
equivalents, qualification ratios and spare power are explicit model estimates,
not newly invented historical factory counts.

Browser new/load paths enable `GameRules.industry_rebuild`. An absent/false flag
preserves legacy monthly and daily replay. New buildings use separate slots;
the seven-element legacy industry array stays intact. Existing project IDs,
progress, payments, receipts, procurement and equipment revisions survive.
All commands use the normal command queue. Previews never advance time,
reserve inventory or spend money. AI uses the same project evidence and
supply constraints, including component purchases and export policies.

The integration worktree combines current map/art with the funded economy,
equipment, aviation and government branch. Concurrent character development
remains in its own working tree and is not overwritten.

The published specialist-company system is also integrated. Contractors modify
funded work and its real input bundle; only completed work pays their service
fee. Advanced components follow the contractor-adjusted physical equipment
output, while historical spare electricity carries no private operator fee.
Saved contractor assignments and signed input savings survive the rebuild.
The population rebuild has a separate counted-labor owner; its integration
will replace the fallback hiring estimate through the shared staffing APIs.

## Verification

Simulation checks cover allocation across buildings/mines, input atomicity,
preset atomicity, save/replay continuity, dated operations, power/workforce
limits, GDP accounting, component consumption, naval slots and maintenance
invoices. Web tests check command validation and read-only effects. Frontend
checks cover roles, assignments, presets, shortages and supplier selection.

`tools/ui/check_industry_rebuild_browser.cjs URL --disposable` exercises a new
campaign, four-project preset, pause/Auto and the industry view at desktop,
tablet and phone widths. Run only against an isolated disposable server.
Full-suite results are reported with the delivery; these targeted tests do not
claim new long-run economic calibration.

UI acceptance checks exercise the shipped renderers and command adapters:

- Project purposes and tradeoffs are grouped by decision; facility upgrades
  appear after selecting a province.
- Impact review shows local and national effects, named preset costs and
  operating needs before a construction command is sent.
- Manual assignments, zero to pause and Auto preserve the server's building
  or mine identity. Invalid numbers and stale readings cannot submit orders.
- Operations keep installed capacity, usable capacity and dated output
  separate, and distinguish economic output from tax revenue.
- Advanced components appear in supplier selection only when the served
  goods catalogue includes them.
- Browser checks use 1440, 820 and 390 pixel viewports, check horizontal
  overflow and JavaScript errors, and capture catalog, preview, queue and
  industry screens. Funding remains accessible from a compact disclosure
  while reviewing projects.
