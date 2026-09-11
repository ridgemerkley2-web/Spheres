# S03 — Companies and procurement integration

Status: complete for the S03 integration scope. Input: `cd9eb8f18579a9c37fd10b1d4ba86b0d50b2a03a` on `codex/campaign-certification`. This session follows the [S01 property contract](../S01/COMPANY_AND_SAVE_CONTRACT.md) and [completed S02 economy](../S02/README.md). The original active playset remains unchanged until S05.

## Ownership contract

The existing `companies` book remains the only owner of equipment suppliers, corporate cash, paid development, work in progress, unsold stock, purchases, ammunition provenance and manufacturer refits. Its numeric IDs and book versions keep their meanings. Government stock, held funds, corporate cash and earned revenue are separate property.

Master's fictional service-company roster and assignments are imported into `sector_contractors`, a separate typed namespace. A supplier and contractor with the same numeric ID are distinct identities. One country company directory presents both roles using stable qualified references; neither matching names nor matching numbers merge their property. Imported names, logos, assignment dates, experience and historical service receipts remain intact.

Contractor work may modify eligible prospective public work only after its normal owner pays and settles it. Financial construction retains frozen base contract values and requires no material pack, capacity allocation or crew assignment. Any separately accepted service fee must fit the same construction spending ceiling and must not reprice previously paid work. Contractor operating assignments do not claim a second physical factory slot or add a second population job pool. They do not modify supplier fixed prices, certified designs or refit reservations.

New supplier operating requirements are explicitly adopted. Existing owned stock, purchased deliveries, certification, paid work-in-progress recipes and fixed refit agreements keep their property and terms. New production must use actual available operating inputs, existing Arms Plant workforce and company funds; government material sales transfer money and stock once and do not award sale GDP. New rules must not silently impose a second input purchase on existing work.

## Saves and commands

The combined save format will declare the company integration capability and retain the exact equipment, party and economy subversions. Shape classification happens before deserialization: supplier books, old contractor rosters and absent legacy books are different inputs; ambiguous mixtures are errors. Loading performs no enable action, payment, delivery, date advance or company creation. Unintegrated operational-war property remains an explicit S04/S05 refusal.

The existing `Company` orders and pre-open/post-close receivable settlement remain authoritative. Explicit directory adoption and service assignment commands supplement that lifecycle. New construction and supplier prices must use reviewed current-state actions and clear blockers.

## Acceptance and evidence

- [x] Reconcile the two company roles and render one directory with persistent identities.
- [x] Preserve company cash, held money, earned revenue, inventories and physical entitlements.
- [x] Connect prospective operating inputs without repricing existing property.
- [x] Preserve supplier/party/economy envelopes and supported contractor work across two loads and deterministic continuation.
- [x] Retain certification, incoming equipment/ammunition purchases and partial refit reservations.
- [x] Verify the combined public and supplier flows, UI and bounded performance; publish exact evidence and limitations.

The integrated source is verified locally. Active-playset replacement, the S05 live-browser matrix, remote CI and campaign certification are separate gates.

## Current run boundary

The user extended this run to finish S03, S04 and S05 and then stop before S06.
S04 source is prepared separately until S03 is committed and checked. The
approved roadmap's later country, fleet, content and certification sessions
are not folded into this integration run.

## Version and property mapping

| Input | Handling |
| --- | --- |
| Absent legacy books | Remain absent and inert; no companies are seeded by load. |
| Active supplier books 1–4 / equipment envelopes 2–5 | Preserve IDs, profiles, cash, stock, dated invoices, ammunition provenance and refit reservations. |
| Original master roster in `world.companies` | Recognize its complete original shape before decode, move it into `sector_contractors`, validate all identities and receipts, and leave the supplier book empty. |
| Both company roles | Write `spheres-companies-save` v1 with equipment, party, economy and supplier-operations subversions. |
| Paid master public equipment | Preserve signed ingredient savings/penalties and separate historical fees; nominal inputs equal actual inputs plus recorded savings. |
| Mixed/unknown company fields or unsupported capability | Refuse before replacing campaign state. |
| Operational-war property | Held for the explicit S04 adapter; never silently discarded by S03. |

The operating book explicitly records inherited program, vehicle-WIP and fixed
refit identities. New work commissioned on the adoption date is distinguished
by identity, not guessed from a date comparison. Cost bases include actual
operating purchases; raw unit recipes remain separate from advanced components.

## Verified result

Runtime candidate `8b1ff717ae5c21d1b96d183fbd987164ed10b03b`: 1,377 native tests passed, 0 failed, 72 ignored. The full UI run passed 1,397 with one optional external advisor fixture skipped; 83 relevant checks passed again after the final responsive fix. See [manifest](manifest.json) and its hashed evidence files.

The first full native run caught circular blocking between public refits and company leases after a capacity reduction. Public dated dispatch now retains priority; supplier scheduling yields once and new starts still count both reservations. The other failed case lacked the political-institutions prerequisite in its new fixture. Both corrected cases pass in the final full run.

All six copied lifetime cases met the frozen latency and memory limits. This is 186 bounded legacy-workload days after explicit company/economy adoption, not a full supplier-heavy or 2035 campaign. The Companies fixture passed local search, role/sector filters, reviewed-fee display and 390px layout with no console errors. Campaign-changing controls were disabled in that static review; no live server was launched. All eight protected campaign/archive hashes remain unchanged.

The first timing run exceeded the late busy p95 bars (307.1ms simulation / 432.0ms whole turn). A same-binary economy-only control measured 374.5ms whole-turn p95; the subsequent company-enabled run measured 391.9ms. No code or targets changed between these runs. The substantial shared-component timing variation is recorded rather than attributed to a specific system cause. Final headroom is narrow; S04 and S22 must remeasure it. All three runs are retained in evidence.
