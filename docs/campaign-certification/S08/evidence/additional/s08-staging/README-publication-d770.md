# S08 — Supplier choice and reviewed imports

**S08 acceptance evidence passed; publication remains pending. Execution stops before S09.** Runtime qualification uses clean candidate `d770592aeead51f1313d507edd26b02d75a69bba`. The full supplier journey, preservation audit and isolated review launch are complete; evidence collection and the final test/documentation commit remain pending. Paths below are relative to the external `campaign-certification` directory and must be remapped to the verified collection before publication. Proof hashes are recorded in `manifest-publication-d770.json`.

## Player-facing result

Use **Government → Companies → Compare equipment suppliers** to compare finite domestic and foreign stock, inspect an exact model, review quantity and price, and follow a purchase through payment, import and delivery. Money exposes unsettled payments and escrow. Reviewed cancellation refunds settled payment once and returns the original lot when the seller can receive it. Restricted access delays delivery while preserving ownership. Foreign equipment can be used and maintained without granting component research or a domestic manufacturing licence.

Seven modeled supplier programmes pay for facilities, development, tooling, inputs and completed stock through ordinary decisions. They skip the player, require economic competition and remain opt-in for legacy campaigns. They guarantee neither opening fleets nor a first-stock date. Exact read reuse and a bounded pool of 256 nominal route trees improve runtime cost; live prices, quantities, permissions, capacity, settlement and saved state retain their existing behavior. Load starts with an empty route pool.

## Completed qualification on d770592

| Lane | Result | Proof |
| --- | --- | --- |
| Windows and Linux native workspace | Each: 1,598 passed, 0 failed, 79 ignored; 65 completed targets | `evidence/S08-final-native-route-pool-1.json`; `evidence/S08-linux-route-pool-1/runner-result.json` |
| Windows and Linux Node | Each: 1,507 passed, 0 failed, 1 skipped | `evidence/S08-final-node-route-pool-1.json` and `.log`; Linux proof above |
| Original external archives | Three explicitly invoked ignored tests passed on each OS: pinned master, 28 active company fixtures, party v1/equipment v2–5 | `evidence/S08-external-route-pool-1/result.json`; Linux proof above |
| General and Money browser | Passed original assertions; served assets and runtime hash verified; desktop/mobile results retained | `evidence/S08-final-browser-installed-route-pool-3.json`; `evidence/S08-final-money-browser-route-pool-1.json` |
| Complete supplier browser | Passed purchase, delivery, paid maintenance and final Save/Load/Continue; two commands, no page errors | `evidence/S08-supplier-browser-corrected-route-pool-2/runner-result.json` and `supplier-TDUttg/result.json` |
| Fresh supplier stock, purchase and delivery | Passed 2,155 actual daily advances without synthetic endowments | `evidence/S08-genuine-supplier-route-pool-1/runner-result.json` |
| Six original legacy performance cases | All passed at ages 0/10/30, idle and industry/war; 31 days each | `evidence/S08-performance-route-pool-1/acceptance.json` |
| Fresh purchased supplier performance | Passed all five original limits over 31 actual days | `evidence/S08-feature-performance-route-pool-1/runner-result.json` |
| Visual review and preservation | Actual desktop/mobile captures inspected; eight protected saves and two source heads unchanged | `evidence/S08-visual-review-d770.json`; `evidence/S08-preservation-after.json` |

The unchanged performance declaration is SHA-256 `8d2cb23fe996ba9dc35da0aed5859072a83c2e61b55d4881cf047bc1ac195a28`.

| Metric | Original limit | Legacy worst case | Fresh supplier case |
| --- | ---: | ---: | ---: |
| Simulation/history p95 | 300 ms | 239.4053 ms | 299.4182 ms |
| Whole server turn p95 | 400 ms | 329.3678 ms | 381.4565 ms |
| Whole server turn maximum | 750 ms | 361.8156 ms | 496.2670 ms |
| Equipment board p95 | 300 ms, supplier case | — | 107.0694 ms |
| Purchase quote p95 | 250 ms, supplier case | — | 2.4301 ms |

Legacy sampled private memory peaked at 472,342,528 bytes against the original 1 GiB limit. Supplier sampled private memory was 2,218,074,112 bytes; no supplier-memory limit was declared. These measurements exclude network, disk autosave and browser rendering. Supplier simulation p95 is close to the declared boundary; the finite workload pass is not a universal performance guarantee.

## Genuine journeys and scope

The genuine exporter records Tonga buying **one French APC** from company 3/product 48 on day 2127, **29 October 1995**, for **0.0018396366278316358 billion**. Contract **3694** delivered exact revision **`import-3-48`** on recorded day 2154; its post-tick delivered snapshot is day 2155, **26 November 1995**. The eleven original archives and their recorded hashes remain attached to the exporter proof. The input used for supplier performance is its original purchased archive, SHA-256 `73a8ed5bd81502bf37d77f374c23f18ed96644a0032ba295317b25a0d01133f9`.

This journey uses a disclosed, ordinary fixed-total Tonga budget reallocation from Infrastructure to Defense procurement, with annual renewal. The personnel, operations and maintenance allocation amounts **exceed their positive opening amounts**; they are not unchanged absolute allocations. It grants no cash, facilities, technology, inputs or stock. A previous default-budget developmental run did not reach an affordable purchase within 3,000 days. Current authentic availability evidence covers this APC journey; it does not establish every platform or nation has affordable stock. The separate eleven-platform lifecycle matrix uses explicitly synthetic opening capital, plant, raw inputs and buyer appropriation. Domestic ammunition regressions cover 23 recipes; representative foreign ammunition branches do not constitute genuine fresh-world availability proof for all 23. Earlier raw proof prose remains unchanged.

The completed browser journey deliberately advanced a real day to prove stale-review rejection before purchasing **contract 3695** on day 2128 and delivering exact revision `import-3-48` on day 2155. Midpoint and final exact Save/Load, cancellation-review purity and final Continue all passed. The final campaign is **Tonga, 29 November 1995**. Two commands were issued: one purchase and one maintenance approval. Quantity editing was correctly skipped because only one unit was within actual stock/funding limits; no second unit or authority was fabricated.

Its day-2157 maintenance receipt paid `2.543259532221702e-7` billion toward `3.67e-7` billion of custom-fleet requirements: **69.3% common support**, with a disclosed shortfall. Total settled Defense maintenance was `8.998407463094934e-7` billion. This is an aggregate invoice, not a separate model payment or full coverage. The actual completed receipt, 23 screenshot names, command identities and original inner-result hash are retained in `s08-staging/supplier-browser-corrected-route-pool-2-complete-receipt.json`; visual inspection is bounded to the recorded frames.

The production runtime remains d770592, game SHA-256 `c5852894069c797c1cbf6b15a302f73f85a0f0b4a4edfee63eb1ca6b367f8435`. The passing browser executed explicitly bound test source SHA-256 **`cd9c5ab5783d6ca6bf71aa913842436e851311fa00ce4b6a33881a25f3bdc33a`**, distinct from committed harness `de8a13611ce1617133710b252253cd82e41d6a4b74d39799f7d29eb2b97ce7f1`. Two test-only blocks changed: finish held-response fulfillment/disposal before removing the interceptor; preserve audit captures outside visible saves. Thirteen cleanup checks and twelve capture-placement checks passed. Original timeouts, commands, assertions and served assets are unchanged. The final test/documentation commit must apply these exact executed bytes; its revision is pending.

## Retained failures and publication gates

All initial e6187df outcomes and intermediate diagnostics remain attached to their own source. They do not qualify this replacement. Retain its failed supplier performance, two incomplete supplier browser attempts (including termination `0xC0000409`), failed fixtures and diagnostic source snapshots.

For d770592, retain general browser attempt 1's 30-second startup timeout with **cause unknown**, and attempt 2's preload-path setup error before browser launch. The successful observed attempt 3 preserved all original assertions and reached DOMContentLoaded in 150 ms; three later WebGL warnings and a favicon 404 remain disclosed. Retain the first supplier browser attempt's `Route is already handled!` failure: Playwright could continue its held request when the harness removed the last interceptor before fulfillment completed. That failed journey is not overwritten by the corrected attempt.

Retain the c74 cleanup-only run's later final save-list timeout and its verified partial paid-maintenance receipt. Its audit captures accumulated 21 JSON files totaling 2,560,752,214 bytes in the visible saves folder. A retained GET took 25.1878707 seconds and returned all 21 readable slots, including the saved final campaign; the whole pending queue was not timed. The passing capture-placement correction preserves all audit bytes outside the menu's scan. It does not fix the game's known full-file scan cost for users with many large saves. No save/load timeout or assertion was relaxed.

Review the qualified build at **http://127.0.0.1:7846/** with `s08-tonga-in-service`, Tonga on 29 November 1995. `S08-review-launch.json` binds the runtime, actual process and five byte-identical review copies; the original campaigns remain preserved. The in-service copy SHA-256 is `651b3dd21f0331179b6b117b77d78ca05a6f5a91c16de933701364c12424a7a8`.

Remaining publication gates are collector inventory/reconstruction, exact harness plus final documentation commit, and verified Git push or bundle disposition. Update both pathway representations consistently only after publication acceptance. S09 remains Planned, and no later campaign, content or release certificate is granted by S08.
