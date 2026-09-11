# S06 — Money and recovery

Status: complete. Authorized by “Next” on 11 September 2026. Stop after S06; S07 remains planned.

The Economy overview now leads with a dated explanation of public cash and debt. The journal observes existing payments; it does not change their arithmetic or add a second financial settlement. A day can include the government budget, resource market, goods deposits and refunds, export dispatch receipts, public transfers, supplier input receipts and refit refunds. Debt restructuring is identified as noncash relief.

Annual authority, fresh spending awaiting fiscal close, supplier transfers already within that ledger, paid property in transit, held deposits and conditional future work have separate descriptions. There is no combined total across those categories. Unknown cargo prices stay unknown; scheduled arrivals remain conditional on transport.

Approved funding decisions retain their actual before/after settings, subsequent cash and debt balances, and later closed monthly observations. Monthly totals may include days before a mid-month decision. These are observed results of all intervening events, not a claim that one policy caused every change. Fiscal consolidation records one decision for its existing package.

The optional saved journal begins prospectively with a real payment or approved funding decision. Opening this page or loading an older save does not manufacture past transactions or enroll accounts. Up to 45 dated days and 12 decisions are retained. A balance difference outside a tagged payment is exposed explicitly.

## Qualification

Runtime revision: `e2f66bca0614cce254f3bb15122cff0dadff128b`. [Machine-readable manifest](manifest.json) records the executable, test binaries and evidence hashes. The following documentation and test-harness correction do not alter the qualified game code.

| Check | Result |
|---|---|
| Full native suite, Windows and Linux | 1,501 passed per platform; 0 failed, 75 explicitly ignored |
| Full UI suite, Windows and Linux | 1,483 passed per platform; 0 failed, 1 skipped |
| Original master, active supplier-stage saves and older equipment versions | Three external checks passed per platform |
| Existing browser regression | Passed, including command response-loss retry and named save/load |
| France money journey | Passed: real budget, tax 42.00% → 42.50%, three daily enacts, recorded movements, stale-response guard and exact reload |
| Six frozen performance workloads | All passed; worst simulation p95 250.22 ms, whole-turn p95 341.93 ms, maximum 471.76 ms |
| Sampled private memory | 401,223,680 bytes, below the 1 GiB limit; 100 ms sampling can miss short peaks |
| Protected campaign files | All eight original hashes unchanged |

The native recovery fixture uses an actual distressed closed month, one correctly priced Fiscal Consolidation, and the next monthly close. Exact reload continuation and the full world/RNG with only journal fields removed prove that reporting does not change the economic trajectory. Tests also cover ordinary negative interest, public supplier input receipts, fresh/prepaid company invoices, dispatch/refund timing, refit escrow/refunds, refused commands, bounded retention, strict journal fields and large-stock/small-payment precision.

[Desktop overview](evidence/money-browser/money-desktop.png) · [Narrow decision history](evidence/money-browser/money-decision-mobile.png) · [France journey evidence](evidence/money-browser/result.json) · [Performance acceptance](evidence/S06-performance/acceptance.json).

The first browser attempt exposed signed-zero normalization in the test observation; the test now serializes both readings consistently. A separate attempt timed out before country selection during overlapping checks; an unchanged bounded retry passed. Both failed attempt records are retained. The executed candidate differs from the committed harness only in the relative path needed to import the same helper from its temporary directory. No game data or timeout assertion was substituted. Native development failures and fixes are retained in [the development record](evidence/S06-development/findings.json).

Evidence retains original absolute paths as provenance. The original input saves are the S05 fixtures; raw disposable browser save directories and binaries remain local rather than duplicated into this report. Existing S05 timing variability remains documented, and S22 still owns broader supplier-heavy and 2035 performance. This session does not award G2, CP1 or a full 1990–2035 campaign certificate.

## Continue from here

Execution stopped after S06. **S07 — Finish construction, jobs and operating outcomes** is the next planned session. Its implementation has not begun.
