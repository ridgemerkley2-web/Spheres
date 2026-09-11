# S07 — Construction, staffing and operating outcomes

Status: complete on `041007fbfda48cc027d0c24a1189705a1dcaa89b`. Execution stopped after S07, before S08.

Construction now leads directly to the installed facility and its operating explanation. Current staffing and rated operating requirements are separate from the last dated output, workers used and charges. Expansion does not rewrite old receipts; new unmatched assignments stay unknown until the normal workforce match. Construction continues to require financial funding only.

The Industry view includes Office District, Advanced Industry and dock services alongside existing processing and workshop operations. Current requirements use the native recipes and contractor terms. Value added is the existing province/national contribution counted once, with inherited GDP distinguished from incremental output. The power summary counts displayed receipts on the same date only.

## Verification

[Machine-readable manifest](manifest.json) · [Raw evidence inventory](evidence/inventory.json).

| Check | Result |
|---|---|
| Full native suite, Windows and Linux | 1,514 passed per platform; 0 failed, 75 ignored, 63 completed targets |
| Full UI suite, Windows and Linux | 1,496 passed per platform; 0 failed, 1 skipped |
| Original master, active supplier stages and party/equipment versions | Three external checks passed per platform |
| General browser regression | Passed on the same runtime: saved command response recovery, delayed boot navigation, named load and history roundtrip |
| France and Tonga actual browser journeys | Financial previews, priorities, paid work, shared pause, reviewed cancellation without refund, paid save/load, visible annual-authority renewal in Tonga, completion, exact facility focus and saved outcomes |
| Six frozen performance workloads | All passed; worst simulation p95 251.44 ms, whole-turn p95 346.01 ms, maximum 487.88 ms |
| Sampled private memory | 449,568,768 bytes, below 1 GiB; sampling can miss short peaks |
| Protected campaign archives | All eight original hashes unchanged |

| Country | Completed construction date | Recorded workshop result | Evidence |
|---|---|---|---|
| France | 28 Dec 1990 | 0.101388 intermediate packs on 1990-12-27 | [Journey](evidence/browser/france/result.json) |
| Tonga | 28 Sep 1991 | 0 intermediate packs on 1991-09-27 | [Journey](evidence/browser/tonga/result.json) |

These browser journeys use fresh campaigns and actual controls without granting funds, inputs, workers or installed capacity. The server receipt supplies the displayed output or blocked reason; installed capacity is not presented as production. Native/server tests cover scaled and ordinary projects, contractor-aware estimates, actual charges and inputs, staffing attribution, dated ownership, expansion, pure reads, inherited GDP and bundled workshop components. Desktop and narrow screenshots were reviewed: [operating-desktop.png](evidence/browser/france/operating-desktop.png), [operating-mobile.png](evidence/browser/france/operating-mobile.png), [requirements-desktop.png](evidence/browser/france/requirements-desktop.png), [requirements-mobile.png](evidence/browser/france/requirements-mobile.png), [operating-desktop.png](evidence/browser/tonga/operating-desktop.png), [operating-mobile.png](evidence/browser/tonga/operating-mobile.png), [requirements-desktop.png](evidence/browser/tonga/requirements-desktop.png), [requirements-mobile.png](evidence/browser/tonga/requirements-mobile.png).

## Retained attempts and limits

- [France: country selection](evidence/failed-browser-attempts/france-eXE1kF/result.json): page.goto: Timeout 45000ms exceeded.
- [Tonga: launch](evidence/failed-browser-attempts/tonga-NPEta5/result.json): apiRequestContext.get: Timeout 30000ms exceeded.
- [Tonga: finish through visible daily controls](evidence/failed-browser-attempts/tonga-luNj8r/result.json): AssertionError [ERR_ASSERTION]: Construction did not finish within the declared 800-day bound

Development logs and all discovered S07 qualification/performance attempts are retained in the evidence inventory. The selected passed journeys do not remove earlier failed attempts; their result files retain the failure stage and diagnostic. No cause is inferred from a timeout alone.

The separately recorded startup diagnosis found an unconsumed roughly 2 MB state response holding the synchronous server response open. Consuming that body restored immediate small build responses. A later Tonga attempt omitted the existing annual capital renewal and reached the unchanged 800-day completion bound after 803 total advances. That failed run and its funding diagnosis are retained.

The selected Tonga journey reapplied the same daily ceiling through the visible funding control on 1 January 1991. It verified no immediate date, project-progress or paid-work change, then continued real daily advances and completed on 28 September 1991 after 635 total advances. These are two explicit harness corrections: startup polling/response handling and an additional annual-renewal player action. The accounting, save, outcome and completion-bound assertions remain; the harnesses are not claimed to have identical action sequences. The authoritative correction record is: France passed the committed gameplay harness unchanged. Tonga and general regression use bounded /api/build readiness probes whose bodies are fully consumed. The selected Tonga journey additionally checks annual funding once per observed year and reapplies the same daily budget through the visible Apply control when renewal is required. It asserts unchanged date, project progress and paid work, then continues actual daily advances. The original 800-day bound and accounting, save and outcome assertions remain. An earlier Tonga run omitted renewal and failed at that bound; its full evidence is retained. Candidates use Module._compile at original filenames; general keeps the supported installed Chrome channel override.

Neither correction changes the qualified game executable. The successful France journey retains its unchanged original harness association; corrected Tonga and general journeys retain theirs. Existing runtime limitation: The existing synchronous HTTP response loop can block behind an unread response. The probe establishes this mechanism for the Tonga-shaped startup failure. The earlier France navigation timeout remains of unproven cause; the runtime was not changed to add slow-reader isolation. See [harness provenance](evidence/qualification/S07-harness-correction.json) and [visual review](evidence/qualification/S07-visual-review.json). The slow-reader response behavior and the annual-renewal queue wording are not claimed to be fixed by this session.

- Construction remains funding-only. Operation uses its own workforce, qualifications, inputs, power, storage and department authority.
- The daily construction ceiling persists across years, but annual capital authority requires renewal through Apply funding. The queue can report budget exhausted without distinguishing expired authority; the detailed cash-flow alert explains it. This wording limitation remains.
- Current readiness, installed capacity and dated operation are separate; older receipts are not given invented workers, owners, completion dates or paid history.
- Research and equipment readiness depends on selected work; generic staffing does not certify a project or production line.
- Facility power totals include only displayed same-date receipts, not all national consumption.
- GDP valuation is the existing province/national contribution counted once; inherited value is excluded from incremental additions, and value added is not Treasury revenue.
- France and Tonga are bounded construction journeys, not the full eight-country 1990–2035 campaign matrix.
- The six frozen regression workloads and sampled memory do not certify all supplier-heavy or 2035 workloads; S22 retains broader performance ownership.
- No G2, CP1 or full campaign certificate is awarded. Stop after S07; S08 requires a new instruction.

## Continue from here

S07 is complete and execution is stopped. **S08 — Complete supplier choice and reviewed imports** is next and remains planned. The existing G1 award remains bound to its S05 evidence; this session awards no later campaign, content or release certificate.
