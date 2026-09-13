# S08 candidate collection audit

Outside-repository read-only audit prepared 12 September 2026, America/Los_Angeles. No qualification, collector, compression, archive parsing or large-file hashing was run for this audit. The repository source is frozen while full Windows and Linux qualification runs. This document does not award completion.

## Candidate and evidence boundaries

- Replacement runtime: `d770592aeead51f1313d507edd26b02d75a69bba`, parent `e6187df8044556a8262154e7a6c8baa32b283799`. Git records the commit at `2026-09-12T18:40:13-07:00` (`2026-09-13T01:40:13Z`). These are commit timestamps, not completion timestamps. Linux `S08-linux-route-pool-1/source-before.json` records the replacement pin and a clean checkout; its runner was still running at this audit.
- The initial e6187df candidate retains its actual native/Node/external/general-browser/Money/legacy-performance passes, genuine supplier journey, feature-performance failure and incomplete supplier browser. Its source and executable identities must remain separate from d770592 qualification.
- Round 23 retains 40 passing tests and one fixture failure. The complete pooled/native world had matched before the disabled public stock view assertion failed. The fixture correction changed only the assertion to inspect retained market stock and explicitly preserved the public zero view.
- Round 24 (`S08-freight-checks-game-owned-pool-disabled-ledger-fixture-24/result.json`) records 143 passing test executions, zero failures and six ignored diagnostics across overlapping filters. These are developmental test executions, not a deduplicated full-suite count.
- Feature 10 (`S08-diagnostic-game-owned-nominal-pool-feature-10/development-timing-assessment.json`) meets all five original limits: simulation/history p95 290.69010000000003 ms, whole-turn p95 375.9804 ms, whole-turn maximum 497.1219 ms, equipment board p95 102.3519 ms, quote p95 2.0868 ms. The assessment explicitly identifies dirty source and the older genuine input. Its recorded profile SHA256 is `62e4fc9641833ec8c0086fbb7c80f96835726d5f404e4fdbae8ca28de0071fbe`; the assessment was created at `2026-09-13T01:37:21.649662+00:00`. This is not replacement qualification.
- Daily diagnostic 11 records 31 exact complete-world comparisons between the cold observed path and the Game-owned pool, using separate search state. Its runner passed from `2026-09-12T18:37:22.782921-07:00` to `2026-09-12T18:38:07.363901-07:00`. Its test binary SHA256 is recorded as `dcc3655cbbde7148bfa3ed301cc0afdeb4f60fd5fb0a7cc831f090e8d798f922`; this is the dirty diagnostic binary, not a newly qualified d770592 executable. The input producer remains the initial e6187df exporter, purchased archive SHA256 `8b604a139dc824359fcc1206896bc5136538e9064eb55fc07af02ca09706f827`.
- Preserve the rejected four-way queue source from feature 9/daily 10 source snapshots. Its comparison did not demonstrate a traversal benefit; d770592 restores `BinaryHeap`. Do not remove those failed timing records or imply a standalone attribution for the combined experiment.

## Collector findings and required additions

`collect-s08-evidence.py` accepts the candidate from its first argument and checks checkout HEAD before and after collection. Its constant `S08_BASE=68e863055ee6ddf6c9796547995dccf455557edd` is an ancestry boundary for browser discovery, not a stale replacement pin. It correctly keeps each browser record's revision and whether that revision equals the requested candidate.

Automatic `BASE/evidence/S08-*` discovery includes the original declaration/protocol, earlier failures and runner corrections, all diagnostic snapshots, rounds 23/24, and new qualification directories placed under that prefix. New genuine-export and feature-performance directories under their established prefixes retain their named campaign snapshots. The original genuine-export snapshots retain the older producer; diagnostic copies classified as ordinary records may be hash-only because their source snapshots are already retained through the genuine producer. Executables are deliberately hash-only provenance.

The default root runner patterns **do not include** these relevant files:

- `bind-s08-performance.py`
- `assess-s08-development-profile.py`
- `summarize-s08-diagnostic.py`
- `verify-s08-preservation.py`

They must be explicit `--include` inputs. `run-s08-economic-checks.py` is already covered by `run-s08-*.py`; retaining it still requires checking the actual final inventory.

Include the entire small `s08-staging` directory to retain the timeout patch, preparation script, VM verifier, evidence JSON/log, verification note, commerce preparation/provenance, and both superseded and current draft records. The collector already explicitly includes tracked `archive-worker.py` and `supplier-archive-audit.cjs`, along with the supplier/general/Money runners and the installed-Chrome fallback dependency.

Explicitly include `integration/artifacts/browser-supplier-imports-ci/supplier-g630Bz/termination-result.json` through `--failed-attempt`. The file exists; automatic `*/result.json` discovery cannot find this post-termination record. Its `passed=false` remains a failed attempt, never a recovered success. The collector includes that supplier folder's named saves and screenshots once the failure record is registered.

The collector is an evidence packager, not an acceptance validator. HEAD equality alone does not prove qualification, clean runtime sources, final binary identity, or final browser success. The closeout manifest must bind actual complete same-candidate proof records. If qualification fails, retain the failure and keep S08 in progress.

## Prepared invocation

`collect-s08-d770592.ps1` is an outside-repository wrapper containing the explicit additions above. It has not been run. Run it only after all builds, tests, browser journeys and performance windows have finished. Its default mode invokes the collector's dry-run, which still reads and hashes all discovered files; it is not suitable during measurement or compilation.

From this staging directory:

```powershell
.\collect-s08-d770592.ps1
```

After reviewing that inventory and confirming final evidence is complete, run:

```powershell
.\collect-s08-d770592.ps1 -Apply
```

The wrapper refuses an existing destination through the collector's existing contract. Do not delete or overwrite a partially collected directory automatically. The collector itself is unchanged by this audit. If a final workload directory is placed outside `BASE/evidence/S08-*`, add its exact path explicitly with `--performance-dir` or `--include` after checking its scope. Additional failed browser attempts without `result.json` also need their exact failure-record arguments. Do not redirect live collector output into a file inside its own input roots; retain console results after the process exits.

## Stale draft fields to update at closeout

- `S08-requalification-closeout-checklist.md:3` still says replacement identity awaits source freeze. Update that identity to d770592 and keep qualification pending until complete results exist.
- `README-requalification-draft.md:5` and its replacement requirements table still mark the pin pending. Fill the clean pin now when preparing the final draft; fill binary/result rows only from new completed proofs.
- `manifest-requalification-draft.json` has `replacement_optimized_candidate.status=pending_source_freeze_and_qualification` and `runtime_revision=null`. The correct current state is source frozen on d770592 with qualification pending. Keep `completed_date=null` until actual acceptance. Do not modify the nested initial e6187df candidate or its recorded dates/hashes.
- `S08-runtime-requalification-notes.md` was updated by root during this audit and now correctly records round 24, feature 10, daily 11 and d770592. Preserve its qualification-pending wording.
- `STATUS_UPDATES.md:7,12` assumes completion on 11 September and an unchanged initial runtime. It is superseded historical preparation and must not be applied mechanically. Likewise `README-final-draft.md` and `manifest-draft.json` describe the initial candidate only.
- UTC records cross into 13 September while local commit date is still 12 September. Preserve the recorded offsets/timestamps; do not use either commit date or development measurement date as S08's completion date.

## Final collection and publication checklist

- [ ] Finish d770592 full Windows/Linux native and Node suites; retain actual counts and ignored-test scope, source proofs, toolchains, binary hashes, logs and all failures. Invoke the three external original-archive tests separately on both platforms and retain unchanged fixture/producer proofs.
- [ ] Use `bind-s08-performance.py` only against the complete successful d770592 native proof and exact test executable. It checks the original plan SHA256 `8d2cb23fe996ba9dc35da0aed5859072a83c2e61b55d4881cf047bc1ac195a28` and writes a unique rebound protocol without replacing the old one. Preserve this new helper in collection.
- [ ] Run the unchanged six legacy workloads and original 31-day supplier case after other work is idle. Limits remain simulation p95 300 ms, whole p95 400 ms, whole maximum 750 ms; legacy private memory 1,073,741,824 bytes; supplier equipment board p95 300 ms and quote p95 250 ms. No feature-memory acceptance bar is introduced.
- [ ] Retain a new same-candidate genuine supplier export, four decoded named phases and its actual financial/date facts. Run the full supplier browser through positive paid maintenance and final save/continue, with its own dates/receipts; the initial delivered or maintenance-approved artifacts do not fill that row.
- [ ] Retain general/Money browser identity, assets, screenshots and results for the replacement runtime. Confirm every new outcome belongs to the intended executable, runner and archive producer.
- [ ] Run final protected-user-save/source and original fixture preservation checks only after measurement; retain their actual proof. Keep the initial game binary's preserved-copy manifest even though executable bytes remain outside the package.
- [ ] Inspect dry-run included/excluded roots, especially the explicit helpers, timeout and commerce provenance, first supplier failure, termination-result failure, rejected heap snapshots, round 23 failure, round 24 pass, feature 10/development labels, daily 11 and all new qualification directories.
- [ ] Collect once after evidence stabilizes; inspect `inventory.json` logical paths, categories and each original/stored hash. Verify reconstruction for named campaign phases into new outputs outside input roots. Distinct raw first-stock and ready-before-purchase hashes must not be merged merely because file lengths match.
- [ ] Use actual collection paths and completed proof values in final README/manifest. Do not carry forward old binary hashes, old 1,537/1,507 counts or initial exporter dates as replacement values. Keep initial failures and development experiments separate.
- [ ] After all acceptance rows pass, update both Markdown and JSON pathway representations consistently; S08 complete, S09 planned, execution stopped at S08. Record actual acceptance date and Git disposition. No later gate, full campaign/content/release certificate or S09 work is implied.
