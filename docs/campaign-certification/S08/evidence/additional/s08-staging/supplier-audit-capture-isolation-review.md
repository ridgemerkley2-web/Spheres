# Supplier audit capture placement correction

The first corrected c74 browser run genuinely delivered its purchase and paid maintenance but failed its final visible Save/Load wait. The retained direct GET diagnostic, `evidence/S08-save-menu-list-diagnostic-1/result.json`, measured one `/api/saves` call at **25.1878707 seconds** against that same disposable folder. It returned all 21 readable slots, including `s08-import-complete` dated 29 November 1995, and preserved every original file hash. The save was neither lost nor unreadable. Concurrent outstanding refreshes on the serial server can compound this cost, but their complete queue timing was not measured.

The folder held 2,560,752,214 bytes of JSON, mostly extra test snapshots. The unchanged game scans and fully parses every visible save for metadata. This is a known performance limitation for users who keep many large saves. The proposed harness correction does not fix or conceal that runtime limitation; it stops introducing every read-only evidence capture as an extra user save.

`ci-supplier-imports.audit-capture-isolation.cjs` changes only `archive()` on top of the previously executed cleanup-order c74 candidate. The function admits exactly 23 audit-only names, derived from all current callsites: sixteen fixed snapshots plus maintenance days 1–7. The two native Save/Load helpers generate only the enumerated `-before`/`-after` snapshots. Actual input slots, named midpoint/final saves, autosaves, unknown names and path traversal strings are refused before an API save or filesystem move. An existing file with an audit name is also refused before POST, protecting preexisting data.

After the ordinary successful `/api/save`, the function verifies the generated file is a regular non-symlink file inside this real disposable `saves` directory. It verifies `run/audit-captures`, creates an exclusive new subdirectory with `mkdtempSync`, checks the final target is absent and contained, then renames that one generated audit file before calling the unchanged worker. Renaming preserves its exact native bytes; there is no parse/re-encode. The original archive worker still computes/records the input hash, writes its canonical output, and performs the same exact comparisons. Captures remain available even when a later response disposal, buyer read or worker inspection fails. Real input/named save files remain in `saves`; their original 30-second visible Save/Load/Continue checks and all game commands are unchanged.

Prepared identities:

- Frozen committed harness: `de8a13611ce1617133710b252253cd82e41d6a4b74d39799f7d29eb2b97ce7f1`.
- Previously executed cleanup-order-only harness: `c74f8a8d1f1ee090113179a5a34acfada0ffa28e8689de4e3aba8fa766e21b9c`.
- New complete candidate: `cd9c5ab5783d6ca6bf71aa913842436e851311fa00ce4b6a33881a25f3bdc33a`.
- Complete two-block patch `supplier-audit-capture-isolation.patch`: `a3cfcc7a43185fe507aae8ff6f2bb0ec9f151d197a7762a8a9e1401f53769701`.
- Incremental capture-only patch: `ea9323659787c1f2b2d8d42650b4e3fdaa43e3039806fb5eb5874e961bd0218d`.

Twelve bounded VM/file checks passed, using tiny real files outside the repository and a simulated native save boundary. They verify exact source-block isolation and the closed callsite inventory; every permitted name preserves identical bytes and hashes outside the visible slots; real named/input files remain unchanged; existing source and target collision are preserved; escaped save/capture directories are refused; and worker, buyer, dispose, rename and native-save failures preserve the appropriate evidence and fail explicitly. The complete candidate also passed `node --check`. `supplier-audit-capture-isolation-vm-evidence.json` records the exact verifier/source/patch identities and retained tiny fixture directory. These tests do not establish full browser acceptance.

No repository file, runtime binary, game asset, real campaign, browser or build was modified or executed. A separately bound full browser run with the new candidate is still required. The c74 failed attempt and its actual paid-maintenance evidence must remain retained alongside the next attempt.
