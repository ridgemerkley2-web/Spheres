# S01 — company property and save compatibility

Source review only. This document defines the preservation requirements for S03
and S05; it does not claim that migration has been implemented or qualified.
`A` means active `5f7f355502f17bd6bd8f0383a2d14f0024fa7884` and `M` means
master/Claude `485c223f60d5ff6e46f6ae17164bf1ee3a8764d9`. All source locations
below are paths at those pinned revisions, relative to `spheres-sim/src/`
unless a different root is supplied.

## The ownership decision

**Retain active company procurement as the owner of supplier property.** The
approved game loop is design → paid company development → company stock →
purchase → delivery → service. Master adds companies that modify public work;
those contractor assignments can complement the supplier system, but cannot
replace it. Similar company names and numeric IDs are not identity mappings.

| Saved state | Active | Master |
| --- | --- | --- |
| `EquipmentState.version` | 8 | 8, with incompatible fields |
| `Companies` | Version, next ID, firms, equipment/ammunition deliveries, last tick | Enabled flag, roster, assignments, growth, news, next ID, last day/month |
| Company book versions | 1: tanks; 2: mixed equipment; 3: ammunition; 4: refit services | No supplier-book version |
| `spheres-equipment-save` | Versions 1–5; company books 1–4 map to envelopes 2–5 | Version 1 only |
| `spheres-party-leadership-save` | Version 1; equipment sub-version 0–5 | Unsupported |
| Legacy empty world | Raw JSON | Raw JSON |

Evidence: A `companies.rs:11,126,146`, `equipment.rs:13,267,305`,
`lib.rs:1496–1543`; M `companies.rs:117,201,218,254`,
`equipment.rs:14,231–267`, `lib.rs:1613–1633`.

Accepting a version number does not make these shapes compatible. Master rejects
the active supplier and party envelopes, and its defaulted company structure
could discard unknown supplier properties if the outer guards were relaxed.

## Property that must survive

| Property | Authoritative active location and requirement |
| --- | --- |
| Corporate funds | `Company.cash_bn`, capital received, dated expenses/revenue, receivables and transactions. Government cash and company cash remain separate. |
| Development and fabrication | Product identity/revision, frozen profiles, research eligibility, development funding, WIP work, paid costs and `unit_inputs[12]`. |
| Unsold vehicles | `Company.products[].stock` and `stock_cost_bn`. Migration must not grant these to the country. |
| Purchased vehicles | `Companies.deliveries[]`: buyer, revision, cost basis, settlement/transit/arrival dates. No repeated debit or receipt. |
| Arsenal holdings | Exact design ID, fractional legacy quantities, age, condition and reservations. |
| Supplier ammunition | Ammunition products, deliveries and national `AmmunitionState.supplier_receipts[]`. Preserve provenance after ammunition is consumed. |
| Manufacturer refits | Corporate refit contracts plus `EquipmentState.company_refits` source claims; fixed price/recipe, public escrow, earned revenue, refunds, locked labor funds, current inputs/work, completed/cancelled quantities and settlement dates. |
| Master public work | Preserve its additional `EquipmentProject.company_inputs_saved[12]` and `company_fees_bn` as historical receipts. They cannot reprice active supplier work. |

Evidence: A `companies.rs:146,179,206,225`, `companies_ammunition.rs:8,28`,
`companies_refits.rs:12`, `equipment_ammunition_production.rs:329–373`;
M `equipment.rs:231–267`.

## Commands and daily settlement

Active has `Command::Company { nation, order }` with establishment,
capitalization, development, purchase, funding, stock target, development
cancellation, ammunition supply/target/purchase, refit and refit cancellation.
Master removes it and supplies `EnableCompanies`, `AssignCompany` and
`UnassignCompany`. These are different commands, not aliases. Evidence:
A `companies.rs:54`, `lib.rs:82`, `spheres-web/src/main.rs:6565`;
M `lib.rs:95`, `spheres-web/src/main.rs:6717`.

Active settles manufacturer receivables before the day opens and again after
fiscal close, with company work in `SYSTEMS`. Master removes both settlement
hooks and ticks its contractors after fiscal close. Preserve exactly-once
settlement across ordinary days, year boundaries, reloads and retries.
Evidence: A `lib.rs:1335,1443,1451`; M `lib.rs:1559–1574`.

Also retain the supplier enrollment guard suppressing automatic catalogue
purchases, inbound quantities in fleet/ammunition targets, manufacturer-refit
reservations, and the guard against automatic public ammunition fabrication for
converted families. Evidence: A `arsenal.rs:898`, `equipment_targets.rs:166`,
`equipment_ammunition_reserves.rs:138`; corresponding deletions in M.

## S03 migration contract

1. Preserve active suppliers and import contractor assignments into a distinct
   typed namespace. A proposed name is `sector_contractors`; S03 chooses the
   final schema before writing combined saves.
2. Classify active-book, master-roster and absent legacy shapes before decoding.
   Reject mixed or ambiguous payloads. Never silently ignore property fields.
3. Introduce an explicit combined save capability/version. Retain the party
   wrapper and equipment sub-version; equipment state version 8 is insufficient.
4. Decoding performs no economic event: no cash grant, repricing, stock delivery,
   settlement, contractor enable, opening company creation or date advance.
5. Preserve exact IDs and revisions, frozen costs, inventories, departmental
   authority, company capital, escrow/refund state, dates and reservations.
6. Reconcile ammunition from both public batches and supplier receipts. Keep
   inbound orders in reserve gaps and retain provenance after consumption.
7. Apply new contractor modifiers only to eligible prospective work. Preserve
   past savings/fees without stacking a second fee onto a fixed supplier price.
8. Allocate each real facility once across public work and company leases.
   Retain the pre-open/post-close corporate settlement and fiscal observation
   order; prior-year receipts must settle before renewed authority is spent.
9. Validate equipment, ammunition, suppliers/refits, assignments and party
   leadership before returning a loaded world. Save/load twice must be
   idempotent; unsupported histories fail with an actionable error.

## Migration fixture inventory

These are existing generators/tests, not newly created campaign fixtures.
Paths below are relative to `spheres-sim/`.

| Source | Existing scenario stages/checks |
| --- | --- |
| A `tests/companies_integration.rs:1746` | Eligible, development, company stock, settled transit, arrived |
| A `tests/companies_integration.rs:1639` | Ground stock, aircraft development, mixed stock, mixed transit, mixed arrival |
| A `tests/company_ammunition.rs:1285` | Supplier stock, awaiting settlement, transit, arrived |
| A `tests/company_refits.rs:1245` | Seven stages for ground and aircraft: ready, unsettled booking, settled escrow, active work, partial cancellation, returned remainder, complete |
| A `tests/companies_integration.rs:897,927,1129` | Downgrade refusal, matched invoices, fiscal/year boundary settlement |
| A `tests/company_ammunition.rs:791,1091,1204` | Inbound conservation, forged receipt refusal, aircraft stores consumed once across reload |
| A `tests/company_refits.rs:824,973,1015,1216` | Shared reservations/work, old-book preservation, prior-year refund authority |
| M `src/companies.rs:1425,1443,1503,1584,1629` | Inert defaults/idempotent enable, assignments, work/capture receipts, growth continuity, contractor AI |
| M `src/equipment.rs:1899` | Ingredient savings/penalties and fees across reassignment and save/load |

The three active supplier integration files are absent from master. Retaining
only master's test suites would discard essential procurement coverage.

S03/S05 must generate fixtures for every active envelope generation, party
wrapper combinations, master contractor work already paid, both company systems
together, pending year-end settlement, consumed supplier ammunition and
partially cancelled refits. Compare a preservation manifest before migration,
after migration, after a second load and after deterministic continuation.
Use disposable roots; never exercise migrations on the user's only save.

## Stable risks and ownership

| ID | Risk | Resolution session |
| --- | --- | --- |
| S01-R-COMP-01 | Same `companies` field loses supplier property under permissive defaults | S03, validated S05 |
| S01-R-COMP-02 | Dropped settlement or new fees duplicate/omit expenditure | S03/S02, validated S05 |
| S01-R-COMP-03 | Lost inbound/refit claims create phantom available units or duplicate replenishment | S03, validated S05 |
| S01-R-COMP-04 | Missing supplier ammunition receipts erase provenance | S03, validated S05 |
| S01-R-COMP-05 | Company leases and industry support allocate a facility twice | S02/S03 |
| S01-R-COMP-06 | Shared equipment version falsely implies save compatibility | S03/S05 |
| S01-R-COMP-07 | Deleted supplier suites leave ownership regressions untested | S03/S05 |
