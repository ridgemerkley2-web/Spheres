# S02 — Connected economy integration

Status: implementation and verification in progress. This is an integration session, not the CP1 campaign certificate. The playable integration branch is `codex/campaign-certification`; the original active playset is retained until the unified-playset gate.

## Ownership contract

Construction still uses an adjustable daily financial ceiling and departmental authority. Building projects and mines require no material packs, raw stock, construction capacity or assigned crews. Priority, saved contract value, payments, site lead times, ownership and placement prerequisites survive adoption. Completion creates capacity; only paid work receives construction value added.

Integrated civilian production and the new dock operations use staff, power, inputs and actual operating authority. Existing equipment suppliers and ordinary Arms Plant manufacturing lines retain their existing operating rules; their workforce and component integration belongs to S03. The original thirteen project IDs and seven extended-site slots keep their meanings. Office District, Shipyard and Advanced Industry append three IDs and use a separate three-slot site ledger. Existing civilian estates retain their enabling role. Offices pay for actual service output subject to a modeled demand ceiling. Advanced Industry produces a distinct component stock that can be stored and traded; its supplier-equipment consumer is reserved for S03. Hiring contractors is also S03 work.

Legacy naval lines retain their Arms Plant entitlement and company leases remain reserved. A new naval line may use an explicitly recorded optional shipyard slot; loading never reassigns old lines. Project previews distinguish current readiness from settled output. Inherited manufacturing and utility estimates remain labeled estimates and already belong to opening GDP. New output cannot pay their inherited share again.

The optional population book owns resident counts, cohorts, qualifications, training, employment matching and their economic outcomes. Enrollment uses current residents and labeled estimates derived from the sourced 1990 population dataset. It never restores a country's 1990 GDP, population, cash or industries. Mapped and unallocated residents, qualifications and unfinished courses follow ownership transfers. Existing population growth paths stand down only for a valid enabled population book. Training and demographic settlement are dated; construction contracts do not create fictitious domestic construction jobs.

The existing ordinary fiscal settlement or departmental program settlement owns each public bill. Supplier receivables retain their pre-open and post-close settlement positions. Fiscal recovery observes the exact posted receipt and final daily cash/debt; it cannot charge tax, construction, capitalization, procurement or refits again. Unopened treasuries begin at zero cash and their already-owed ratio-derived debt. Already-open books keep actual stocks. Observation uses actual calendar exposure, so partial months and leap days are not whole months. Confidence effects are bounded and gradual. AI fiscal policy uses the same priced commands, with one adjustment owner.

## Project identities and saved entitlements

The persisted kind IDs are the snake-case strings below, not catalogue positions. The original thirteen IDs and their order remain unchanged; the final three IDs are appended. Existing project instance IDs and the next-ID counter are a separate namespace and are never renumbered during adoption. Sources: [project catalogue and level lookup](../../../spheres-sim/src/production.rs), [seven-slot industry ledger](../../../spheres-sim/src/industry.rs), [fractional Starter Industry modules](../../../spheres-sim/src/industrial_modules.rs).

In this table, `d` is the stable district ID; a `provinces[]` row is identified by its `district` field. Array indices are zero based.

| ProjectKind | Persisted kind ID | Before S02 | S02 completed-capacity storage |
| --- | --- | --- | --- |
| Infrastructure | `infrastructure` | Existing | Unchanged: `production.provinces[].infrastructure` |
| CivilianIndustry | `civilian_industry` | Existing | Unchanged: `production.provinces[].civilian_industry` |
| PowerGrid | `power_grid` | Existing | Unchanged: `production.provinces[].power_grid` |
| ResearchCenter | `research_center` | Existing | Unchanged: `production.provinces[].research_centers` |
| ArmsPlant | `arms_plant` | Existing | Unchanged: `production.provinces[].arms_plants` |
| MachineryWorks | `machinery_works` | Existing | Unchanged: `production.industry.sites[d][0]` |
| Generation | `generation` | Existing | Unchanged: `production.industry.sites[d][1]` |
| ProcessingPlant | `processing_plant` | Existing | Unchanged: `production.industry.sites[d][2]` |
| FreightTerminal | `freight_terminal` | Existing | Unchanged: `production.industry.sites[d][3]` |
| Warehouse | `warehouse` | Existing | Unchanged: `production.industry.sites[d][4]` |
| Automation | `automation` | Existing | Unchanged: `production.industry.sites[d][5]` |
| Efficiency | `efficiency` | Existing | Unchanged: `production.industry.sites[d][6]` |
| StarterIndustry | `starter_industry` | Existing | Unchanged: `production.industry.modules[d]`, integer millionths of a standard module |
| OfficeDistrict | `office_district` | Absent | New: `production.rebuild_sites[d][0]` |
| Shipyard | `shipyard` | Absent | New: `production.rebuild_sites[d][1]` |
| AdvancedIndustry | `advanced_industry` | Absent | New: `production.rebuild_sites[d][2]` |

Starter modules continue to contribute their existing fractional estate, processing, generation and grid capacity through the shared effective-capacity calculation. They are not converted into whole levels or moved into either site array. Advanced components have their own national quantity ledger, `production.operations.advanced_components[nation]`; they do not replace intermediate packs or capital goods. New facilities and component state require the enabled daily industry rule and its save envelope.

Adoption preserves each unfinished project's sponsor, district, kind, priority, progress, total lead time, historical raw-input receipts and any fractional module size/start date. Its financial record remains keyed by the same instance ID in `production.industry.projects`: frozen total contract cost, cumulative spending, historical goods receipts and settled date are retained. Already paid work is neither billed again nor converted into a new raw-material obligation. Ordinary ownership, siting and commissioning restrictions continue to apply; adoption grants no completed capacity.

Factory and naval entitlements follow [manufacturing's reservation rules](../../../spheres-sim/src/manufacturing.rs):

- A pre-existing naval line keeps its Arms Plant reservation. A missing `manufacturing.shipyard_lines` entry means the original Arms Plant path; loading never infers a dock reservation from its naval equipment type.
- Arms Plant occupancy still includes ordinary lines, custom equipment reservations and `companies::reserved_slots`. Supplier factory leases are neither freed nor consumed a second time by the new shipyard ledger.
- A newly ordered naval line uses an available Arms Plant slot first. If those slots are full, an available optional Shipyard slot may be reserved by adding that line's unchanged instance ID to `manufacturing.shipyard_lines`. Dock occupancy is counted separately from Arms Plant occupancy.
- New dock lines use actual dock operating readiness plus the existing equipment budget, recipe and delivery lead time. They do not impose a new dock requirement on old naval lines. Losing access blocks the affected work without erasing paid orders; stopping a line releases its reservation and leaves already ordered deliveries owned.

The [S02 industry fixtures](../../../spheres-sim/tests/s02_industry.rs) exercise the thirteen retained keys, all sixteen financial construction contracts, frozen paid work, old naval and company reservations, optional new dock reservations, and separate component stock. Final execution evidence remains pending below.

## Adoption and saves

Fresh integration-browser campaigns explicitly enable the connected economy. An older campaign must use the visible upgrade action; loading alone does not enable population, rebuilt industry or fiscal recovery. Adoption is staged atomically. The same command is available to a living daily player government, costs no political capital and cannot reset existing training or fiscal clocks.

Legacy peace terms changed national residents by a percentage while transferring whole provinces. That can leave the old provincial estimates above the current national total. Initial adoption keeps that national total and proportionally reduces only the excess mapped estimates; an under-mapped country's remainder stays explicitly unallocated. The affected country's source note records the original estimate, retained total and scale. Ownership, assets, GDP, cash and debt are unchanged. Loading alone does not make this adjustment, and already-enabled population books still refuse inconsistencies rather than repairing them. The year-10 Kuwait benchmark exposed this case; the regression reproduces its exact resident totals through the existing province-transfer API.

The year-30 checkpoint also exposed a positive rounding remainder in China's provincial totals. Workforce allocation now divides positive demand proportionally even for such tiny resident accounts; it cannot send the entire remainder into the last sector and overfill its jobs. The remainder is retained, and the same strict staffing validation applies. Regression checks cover zero, tiny and ordinary populations; a separately invoked archive check adopts the unchanged checkpoint and compares two daily save/resume steps.

| Saved form | S02 behavior |
| --- | --- |
| Legacy raw world | Retains disabled connected-economy defaults until explicit adoption. Existing compatibility repairs can still normalize older supported saves; loading does not enroll the new economic books. |
| Equipment envelope versions 1–5 | Keeps the active equipment/supplier version and all property. |
| Party-leadership envelope version 1 | Keeps real person/party bindings and its exact equipment subversion. |
| Connected economy envelope version 1 | Declares equipment and party subversions and retains the three economic books. Older loaders fail clearly rather than dropping them. |
| Browser campaign archive | Keeps recorded history around its supported inner world envelope. Standalone party/economy imports also use the simulator loader. |
| Separate master contractor/operational-war dialect | Explicit refusal before deserialization can discard property; full migration belongs to S03–S05. |

The new economic validators are read-only. The surrounding loader retains its documented compatibility repairs, including deriving a stale debt-to-GDP ratio from existing debt and GDP without changing debt, cash or GDP ([loader](../../../spheres-sim/src/lib.rs), [small-country regression](../../../spheres-sim/src/economy.rs)). Orphaned population outcomes, disabled but populated books, impossible dates, inconsistent residents, invalid operating stocks and mismatched envelopes are errors rather than silent reseeding. The original user campaign files are protected by the S01 hash manifest; tests use controlled fixtures and disposable copies.

## Evidence and remaining gates

The acceptance tests cover financial-only construction, existing project and company property, real daily fiscal settlement, population training and succession, explicit adoption, and deterministic save/resume. Before/after accounting fixtures and final test results will be linked here after verification.

S03 integrates contractor identities and supplier consumption. S04 integrates operational warfare. S05 qualifies their combined saves and earns G1. S22 and S25 still own performance qualification and full 1990–2035 country campaigns; passing this session does not earn those markers.
