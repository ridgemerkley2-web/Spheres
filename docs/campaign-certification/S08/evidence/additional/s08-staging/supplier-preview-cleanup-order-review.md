# Supplier stale-review cleanup correction

The retained supplier browser attempt (`evidence/S08-final-supplier-browser-route-pool-1.json/log`, run `supplier-XZ7jkY`) failed at the real-date-change stale-review step with `route.fulfill: Route is already handled!`. Its earlier exact preparation and preview archive comparisons passed; this attempt did not complete the supplier journey and remains failed.

The installed Playwright 1.58.2 source explains a concrete race in the harness cleanup:

1. `lib/client/page.js:435–456`: `page.unroute(pattern, handler)` uses default behavior, removes the handler and immediately updates interception patterns. It does not wait for the active callback.
2. `lib/server/dispatchers/pageDispatcher.js:159–168`: removing the last page URL pattern removes its server interceptor.
3. `lib/server/page.js:480–486`: interceptor removal notifies in-flight routes.
4. `lib/server/network.js:273–277`: a route currently owned by that interceptor falls back. With no later handler it continues the request, marking the route handled at lines 360–366.
5. The harness released the held response and immediately unregistered its handler before awaiting completion. That fallback can beat the original `fulfill`, whose own handling guard then rejects the already-handled route. Its earlier comment claiming that unroute simply let the callback finish was incorrect for this case.

`supplier-preview-cleanup-order.patch` changes exactly that one cleanup block: release the hold, await the existing bounded `completed` outcome (including fetch, fulfillment and response disposal), then remove only the exact registered handler. Fetch and manual waits keep their existing 30-second bounds; completion timeout still produces an explicit failure and attempts cleanup. No error is hidden, including an already-handled error. The original stale-response/date-change assertions, real responses, commands, archive comparisons, and all runtime code remain unchanged.

Prepared files and SHA-256:

- Frozen source: `integration/tools/ui/ci-supplier-imports.cjs`, `de8a13611ce1617133710b252253cd82e41d6a4b74d39799f7d29eb2b97ce7f1`.
- Full outside candidate: `s08-staging/ci-supplier-imports.cleanup-order.cjs`, `c74f8a8d1f1ee090113179a5a34acfada0ffa28e8689de4e3aba8fa766e21b9c`.
- Unified patch: `s08-staging/supplier-preview-cleanup-order.patch`, `1110994d8f2252345a73036940642ad94a22852b408e67128faa392796e6e53f`.
- Verifier: `s08-staging/verify-supplier-preview-cleanup-order.cjs`, `593ab3d7613a4709da45a2a202c4c68969ffcadd3173276d7937b7d0ad9b7a79`.

The full candidate preserves the original CRLF bytes outside the exact replacement; the unified patch uses LF for review. `prepare-supplier-preview-cleanup-order.py` writes only outside the repository and asserts an exact unique original block. Its first preparation attempt refused the CRLF source because it initially only recognized LF; the preparer was corrected to recognize either exact line ending. No repository file was changed by either attempt.

Verification completed at 2026-09-13 02:11:07 UTC: all 13 VM checks passed, with source, helper, patch, verifier and relevant Playwright source hashes retained in `supplier-preview-cleanup-order-vm-evidence.json` and its `.log`. The required negative control runs the actual frozen helper and deterministically reproduces the already-handled failure under a model of last-interceptor removal. The candidate successfully fulfills/disposes first. Other cases cover separately pending fulfillment and disposal; unrelated page/context handler identity; first versus later matching requests; fetch, parse, fulfillment, disposal and unroute errors; failed day callbacks; no interception; bounded completion timeout; and nonmatching continuation errors. Both the verifier and complete candidate passed `node --check`. These are focused harness tests, not an authentic supplier journey.

The proposed recorded overlay is appropriate only with explicit provenance: verify the frozen original SHA, exact one-block transformation and executed candidate SHA; compile at the original filename so relative helpers retain their identity; record the overlay runner/hash separately; bind the clean `d770592aeead51f1313d507edd26b02d75a69bba` runtime binary, served assets and genuine export inputs independently; and require the completed inner supplier result. A later test/docs commit must apply the exact executed harness patch. This note makes no supplier browser acceptance claim for the unexecuted corrected overlay.

No repository edits, Rust builds, browser launches, full archives, or simulations were performed for this correction. Only small outside preparation, syntax checks, and the bounded VM verification ran.
