# C01-34–37 integration audit

Read-only audit by Codex `/root/s20_preflight`, 30 September 2026. **No actionable discrepancy found** in the audited imports, receipt preservation, regenerated acceptance metadata or CI setup changes.

## Verified integration

- Brazil, USSR, Tonga and France country Git blobs equal their reviewed repaired revisions (`255a66c0`, `6582e01c`, `7caba838`, `73d748ef`). The complete country JSON also matches in the working tree. All 584 referenced factual-extract files match the corresponding reviewed Git bytes and declared pins.
- All 219 wrapper payload pins and all 211 original review payload pins match. Each original README and manifest remains byte-identical to its review commit. Earlier records remain intact, including SOURCE-17 extracts, the SOURCE-26-containing USSR baseline, Tonga C01-24/C04-era data, and France C01-23/635 organizations. Tonga's four original deputy-PM rows are unchanged at positions 0, 11, 12 and 13 in the extended list.
- The 162 new sources have the correct first-import attribution (76 Brazil, 16 USSR, 39 Tonga, 31 France), and old attribution rows are unchanged. All 141 gap-ledger and 105 boundary input pins match.
- The generated matrix contains 8,039 cases. Its 17 accepted packet IDs and the gap ledger's 11 completed recent intakes match the actual acceptance wrappers/queue; 34–37 close only bounded research intake. C01 remains in progress, C06/S23 remain planned, and runtime/full-history/qualification flags remain false. Canonical parent roadmap files are unchanged.

## CI evidence and scope

The retained Linux run at `3947d1f1` has 539 avatar tests with four failure entries and two skips. Its explicit `No module named PIL` failure and three portrait-selection failures follow the same image-validation dependency path. Installing the pinned Pillow 11.3.0 wheel is a setup repair. The retained clean Windows environment log installs the CPython 3.13 Windows wheel and all six targeted portrait tests pass; this is not a new Linux CI pass.

The Windows JavaScript job was cancelled at its old 20-minute limit: full-history fetch/checkout consumed approximately 13 minutes and tests started at 09:38:39, before cancellation at 09:44:46. The 20-to-40-minute job timeout change preserves the source-history requirements and all test commands. No passing result is inferred from that cancelled run. The separate A1 political failure remains open.

Two stale/incorrect metadata-test expectations failed before the corrected 65-test run passed. Those logs remain retained. This audit read those logs; it did not execute tests. The recorded workboard check reports 44 canonical markers and 39 bounded tasks.

## Limits and artifacts

This is a source/integration audit, not an independent repetition of original research. In particular, this same agent authored the C01-37 source review; only its byte preservation and metadata were checked again. No network retrieval, Cargo, gameplay run or test rerun was performed. Japan C01-29 and newly retained held Russia C01-28 are outside scope and are not accepted by this audit.

`audit.json` records exact country/review revisions, source/receipt checks, working-file and log pins. `audit.py` is the read-only hash/comparison recipe; the final report also includes manually reviewed CI/status findings. `initial-positional-audit.json` preserves a preliminary positional-prefix exploration: its Tonga offsets reflect insertion of new rows, not changed old holders. The completed check uses the exact ordered subsequence. The root working metadata was uncommitted; its observed bytes are pinned. These artifacts grant no parent-session qualification.
