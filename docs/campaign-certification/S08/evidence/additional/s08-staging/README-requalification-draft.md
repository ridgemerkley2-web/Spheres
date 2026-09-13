# S08 — Supplier choice and reviewed imports

**Status: In progress — optimized candidate qualification pending.** Outside-repository closeout draft prepared 12 September 2026. S08 is authorized; execution stops before S09. No completion date or new campaign gate is awarded.

The initial candidate `e6187df8044556a8262154e7a6c8baa32b283799` passed native, Node, original-archive, general browser, Money browser and fresh supplier-stock checks. It **failed the declared genuine-supplier performance acceptance**, and its supplier browser journey **terminated before completion**. Those outcomes remain attached to that initial candidate. Source was reopened for performance corrections and a bounded browser audit harness. The replacement runtime pin, binary identities and acceptance results remain **PENDING**; initial passes do not qualify changed code automatically.

This draft supersedes the status assessment in `README-final-draft.md`, `manifest-draft.json` and `STATUS_UPDATES.md` without modifying those retained drafts. Paths below are relative to the external `campaign-certification` directory unless prefixed `integration/`. The final collector must replace them with its actual collected paths and verify original/stored hashes.

## Player outcome and retained scope

Players can compare finite domestic and foreign stock, inspect the exact model, review a quantity at its current native price, and track paid imports. Adoption grants no equipment, factory, cash or component research. The original Defense payment settles once into escrow; the seller receives it on delivery. A reviewed cancellation returns the original lot and cost basis when the seller can receive it and refunds settled money once without recreating appropriation. Access loss and sanctions retain ownership and explain the delay. Imported revisions grant ordinary use and maintenance rights without a domestic manufacturing licence.

Open **Government → Companies → Compare equipment suppliers**. Dated supplier work, stock filters, exact model inspection, reviewed quantities and delivery records share the existing company and Money systems. Money distinguishes an unsettled transfer from held import escrow. Maintenance uses the ordinary paid plan and current authority.

Seven bounded modeled supplier programmes use ordinary construction appropriations, company capital, paid development/tooling and operating inputs. They skip the player, require the existing economic-competition decision, and remain opt-in on legacy loads. They promise no first-stock date. They are modeled programmes, not historical companies or historical opening fleets; historical company content remains E05.

## Initial candidate: preserved results

All entries in this section belong to `e6187df8044556a8262154e7a6c8baa32b283799`, unless explicitly labeled developmental or post-termination diagnosis.

| Identity | SHA256 |
| --- | --- |
| Windows game | `d878f36a4dec2c83cfb7477a49ce678d4579a26388b0b45f2504c974bba2ee90` |
| Windows web test | `08efa87e668186bddca8a7265c9068578708ebb7ed379b36e51fca22354df2ed` |
| Linux web test | `5875894b7d04e7ccbf1b760f32b4c32b790e7ffa521c4da4470bef216fa149ef` |

| Check | Recorded disposition | Evidence |
| --- | --- | --- |
| Windows full native workspace | **Passed:** 1,537 passed, 0 failed, 77 ignored; 65 targets | `evidence/S08-final-native.json` and `.log` |
| Linux full native workspace | **Passed:** 1,537 passed, 0 failed, 77 ignored; 65 targets | `evidence/S08-linux-final/runner-result.json`, `native.log` |
| Windows and Linux full Node suites | **Passed on each:** 1,507 passed, 0 failed, 1 skipped; 1,508 tests | `evidence/S08-final-node.*`; `evidence/S08-linux-final/node.log` |
| Original pinned-master, 28 active supplier fixtures, party v1/equipment v2–5 | **Passed:** three explicitly invoked ignored tests on each platform, each 1/1 | `evidence/S08-external-final/result.json`; three Linux external result rows |
| Fresh supplier stock and Tonga import | **Passed:** 2,155 actual daily advances; four named phases decoded and canonical roundtripped | `evidence/S08-genuine-supplier-final-1/runner-result.json` |
| General installed-Chrome regression | **Passed:** actual result and wrapper, desktop 1440×1000 and mobile 390×844 | `evidence/S08-final-browser-installed.json`; `evidence/S08-general-browser-e6187df/result.json` and retained screenshots |
| Money installed-Chrome regression | **Passed:** actual result and wrapper, desktop/mobile, reviewed France tax change, stale response and Save/Load/Continue | `evidence/S08-final-money-browser.json`; `evidence/S08-money-browser-e6187df/result.json` and retained screenshots |
| Six legacy performance workloads | **Passed:** all six timings and sampled private-memory bar | `evidence/S08-performance/acceptance.json`, `runner-result.json` |
| Genuine supplier performance workload | **Failed:** four timing bars; purchase quote passed | `evidence/S08-feature-performance-final-1/runner-result.json`, `profile.json` |
| Supplier browser attempt 1 | **Failed:** initial-load confirmation/response harness contract | `evidence/S08-final-supplier-browser.json`; `integration/artifacts/browser-supplier-imports-ci/supplier-y0aPsr/` |
| Supplier browser attempt 2 with load correction | **Failed:** process terminated before final assertions/result | `evidence/S08-final-supplier-browser-load-fix-1.json`; `integration/artifacts/browser-supplier-imports-ci/supplier-g630Bz/termination-result.json` |
| Final protected saves and collected archive integrity | **Pending:** qualified-start and per-run checks are retained; no final closeout award | Final preservation and collector records remain pending |

The native ignored count contains optional performance/export fixtures. The three external tests and genuine exporter were invoked separately. The Node skip is the optional archived real-API advisor reading. The general browser uses a fresh USA technical fixture only; it does not add the USA to certified campaign scope.

General browser execution was **2026-09-11 16:55:42.410126–16:56:06.836823 UTC**. Money execution was **16:56:24.729877–16:56:38.303325 UTC** that day. Money reviewed tax from 42% to 42.5%, reconciled actual values, rejected a stale response and retained Save/Load/Continue. Each result identifies the actual served assets and original game binary.

Linux launch 1 failed before Python or any tests with `HCS_E_CONNECTION_TIMEOUT`; its original log, exit and diagnosis remain retained. The bounded second launch passed without WSL shutdown, service restart or settings changes. The exact startup cause is unproven.

## Initial candidate: actual performance failure

The declaration predates measurements: **2026-09-11T15:57:21.546117+00:00**, `evidence/S08-performance-plan.json`, SHA256 `8d2cb23fe996ba9dc35da0aed5859072a83c2e61b55d4881cf047bc1ac195a28`. These limits remain unchanged for requalification.

| Genuine supplier metric | Initial measurement | Declared limit | Result |
| --- | ---: | ---: | --- |
| Simulation and history p95 | 3,600.9692 ms | 300 ms | Fail |
| Whole server turn p95 | 6,324.6655 ms | 400 ms | Fail |
| Whole server turn maximum | 7,856.5128 ms | 750 ms | Fail |
| Equipment board read plus serialization p95 | 1,996.7904 ms | 300 ms | Fail |
| Purchase quote p95 | 1.2574 ms | 250 ms | Pass |

The feature run executed **2026-09-11T17:25:31.506248+00:00–17:28:25.963605+00:00**, over 31 actual days from the genuinely purchased day-2127 archive. The native test exited 0 and source/input/executable integrity passed, but threshold acceptance was **false**. Native execution success must not be reported as a performance pass. Sampled private memory peaked at **2,218,061,824 bytes**; no feature-case memory acceptance bar was declared, so this remains an observation.

The separate six-workload legacy result passed on the same initial candidate: worst simulation p95 **273.282 ms**, whole-turn p95 **374.0882 ms**, whole-turn maximum **467.0264 ms**, and sampled private memory **426,790,912 bytes**, below the legacy **1,073,741,824-byte** bar. Legacy workloads are ages 0/10/30, each idle and industry/war, 31 days each. These passes do not erase the feature-case failure.

Retain every initial timing record. Corrections must address measured costs and preserve gameplay; no raised limits, different feature workload, suppressed history, injected funds/stock, or unchanged retries chosen for a favorable sample.

## Initial candidate: supplier browser failure and bounded correction

Attempt 1 ran **2026-09-11T17:11:01.913441+00:00–17:11:36.530007+00:00** and exited 1. It expected replacement confirmation when loading the first campaign; the host confirms only replacement of an already live player. The corrected harness registers the response before Load and confirms only actual replacement. The initial result and correction provenance are retained.

Attempt 2 ran **2026-09-11T17:13:38.797288+00:00–17:22:18.894107+00:00** and terminated with **3221226505 (`0xC0000409`)**, empty stdout/stderr, and no original `result.json`. Artifacts reach a purchased import, delivery and maintenance approval; this is partial progress, not a completed journey or paid-service receipt. The termination record is an explicitly labeled post-termination audit, not a recovered original success result. No matching Application Error/WER 1000/1001 event established a cause; memory pressure remains a hypothesis.

A later bounded, short-lived archive worker compared the retained delivered and maintenance-approved saves exactly over **121,248,846 typed canonical bytes**, excluding only Tonga's approved maintenance-plan field. The comparison passed, preserving signed zero and all other financial/property values (`evidence/S08-supplier-browser-memory-fix/diagnostic-2/result.json`, **17:34:33.358935–17:35:03.685399 UTC**, 11 September). It proves that narrow transition only; it does not complete the interrupted browser run.

The replacement harness disposes responses, streams file hashes, keeps small facts in Node and compares full saves through a short-lived worker. It must retain the original exact stock, appropriation, escrow, revision, research, approval-purity and save-continuation assertions. Its final tracked source, new candidate execution and final paid-service receipt remain pending. The collector must explicitly include `supplier-g630Bz/termination-result.json` because ordinary `*/result.json` discovery cannot find that failed attempt. No `postmortem-result.json` was present at this draft audit.

## Genuine initial campaign and coverage boundaries

The retained developmental default-budget run advanced **3,000 days to 20 March 1998** and failed to buy: France held four earned APCs at `0.0018396365958883887` billion each; Tonga could afford zero. That mutable-source development result is not qualification of either candidate, and no blocker was bypassed.

The initial pinned fresh journey explicitly transferred **0.025 of GDP** from Infrastructure to Defense while keeping total spending at `0.30900000000000005` of GDP. Defense departmental basis points were `[600, 600, 700, 8000, 100]`. The actual enrolled-path review cost **15.085 political capital** and opened **$3.0096 million annual procurement** at starting GDP. The same chosen policy was renewed through ordinary annual decisions. No immediate cash, debt, current authority, stock or research grant was made; the choice reduces infrastructure and military research while retaining positive personnel, operations and maintenance funding.

After **2,127 actual daily advances, 29 October 1995**, the exporter observed first finished stock/readiness and bought **one French APC** for `0.0018396366278316358` billion. Contract **3694** records purchase/settlement day 2127 and due/delivered day **2154**. The post-tick delivered snapshot is **day 2155, 26 November 1995**. Final escrow/refunds are zero; buyer revision `import-3-48` preserves the frozen source specification.

Exactly four named archive phases were decoded and compared with canonical native world serialization. The original proof records distinct raw archive hashes for first-stock and ready-before-purchase, despite equal lengths; do not describe those files as byte-identical.

| Decoded phase | Original SHA256 |
| --- | --- |
| `2127-first-finished-stock.campaign.json` | `3beea2a6c5936057c27cb99c7ed09de40d455b5e5ea1e3b6710d85ddd78f1a0d` |
| `2127-ready-before-purchase.campaign.json` | `d75256d72386842d79279b4fa26ea087d24578ab2a01221b6b4c3efb1a6acb37` |
| `2127-purchased.campaign.json` | `8b604a139dc824359fcc1206896bc5136538e9064eb55fc07af02ca09706f827` |
| `2155-delivered.campaign.json` | `58be4b7289213145b40d99eaec1d5eccf620ffe021bb35fb9d47ab218446f4ef` |

One full initial fresh journey ran; no second complete replay of that initial journey is claimed. Additional checkpoints are retained, but not all were run through the four-phase decoder assertion. The new optimized candidate needs its own provenance and fresh journey evidence; a reused initial archive must remain explicitly attributed to its original producer. The browser deliberately ages a quote, so its own dates, price and service receipt must be recorded independently.

The justified synthetic matrix remains eleven existing platforms, 23 domestic ammunition families and representative imported `mg_127` and `air_bomb_guided` rounds/stores. Synthetic funding/capacity/research endowments are disclosed. Eleven-platform import tests establish paid stock, exact holdings, rights and positive upkeep requirements; they do not establish a separate fully funded service receipt for every platform. The final imported APC browser service receipt remains pending. Existing ground/air operational-consumption checks remain compositional coverage, not new full-flight certification. Preserve the exact test inventory and detailed boundaries from the retained initial draft when publishing final evidence.

The sparse `preproduction_inputs_bn` classification accounts once for utilities already paid before stock existed. Legacy recovery is allowed only when the saved phase and receipts uniquely prove preproduction; explicit corruption or ambiguous production history refuses. No repricing, payment or work is created on load.

## Replacement optimized candidate: pending qualification

| Requirement | Status |
| --- | --- |
| Clean replacement runtime pin and reviewed source diff | **Pending** |
| Windows/Linux binary hashes and build provenance bound to that pin | **Pending** |
| Exact semantic tests and source-pinned full native/Node suites | **Pending final association** |
| Original external archive tests on both platforms | **Pending** |
| Genuine fresh supplier stock/purchase/delivery proof for replacement runtime | **Pending** |
| Complete supplier browser: review/cancel/stale quote/purchase/delivery/paid maintenance/Save/Load/Continue | **Pending** |
| General/Money browser checks appropriate to replacement runtime | **Pending** |
| Same six legacy workloads and genuine feature workload against unchanged limits | **Pending** |
| Final protected saves/source preservation and verified collected evidence | **Pending** |
| Isolated local review launch; final commit and ordinary push/bundle disposition | **Pending** |

Development diagnostics and parity tests may justify corrections, but single-sample timings and dirty-source runs cannot fill these source-pinned acceptance rows. Link every new result to its actual pin, executable, input producer and runner. Keep all initial failures visible in the final manifest even after the replacement passes.

**S08 remains in progress. S09 remains Planned.** Only after all required replacement evidence passes and is collected may S08's three acceptance boxes and both pathway status documents be updated. Execution must stop after S08, with no new G2–G5, CP1, worldwide content or release certificate.
