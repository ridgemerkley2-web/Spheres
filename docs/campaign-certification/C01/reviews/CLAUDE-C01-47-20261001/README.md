# Independent France C01-47 review — 1 October 2026

**Decision: accepted as a bounded research intake after three corrections.** Independently retrieved and materially read all **24 originals / 47 claims / 13 holder observations (10 people)**. No held original remains. This is research acceptance, not country-cast, runtime mapping, art approval, C01/C06/S23/WC1/CP1 qualification or a continuous historical roster.

Reviewed exact Claude tip `8226c419fb4c3c1aa0073800f8f3bf80d0663f71`, substantive `0f5192424c29ff2da74dd20656849ad086b6c816`, claim `6f657179bd5f42f6355fb539d4ab6ab562f6b09f`, against base `5ea4f8fcd05e1858b1dfa83ae34c6d3703d6b104`.

- Exact authored import: `b94146ebd8d540bcc86e416ce0277291441afaf7` (35 files; incoming generated research-index excluded).
- Independent corrections and regressions: `578e6edfd3f24e6ffad008c046c6478ebb2eb331` (8 files).
- This receipt records the interpretation separately from response identity and test success; it does not self-reference its eventual commit.

## Material review and corrections

Sixteen party-authored printed bulletins were retrieved from the Fondation Jean-Jaurès archive; eight primary party web pages were retrieved as exact raw Internet Archive captures. All returned HTTP 200 with exact submitted response bytes and SHA-256. Two raw capture bodies were gzip-compressed; decoded material was read without substituting the decoded digest for the original response digest. The Archive pass was coordinated exclusively, at least 15 seconds between request starts; no retries, alternate-host bypass, authentication or 429/503 occurred. An auxiliary web-tool attempt could not open the first PDF; the direct exact HTTP 200 response was available and is the retained evidence.

The original PDF text at every cited locator (including the additional page 7 cited in prose for the 1994 issue) was read. **20 rendered pages** were visually inspected for titles, numeric tables, relative dates and multi-column attribution; the exact per-source page list is in `source-review.json`. HTML main text and relevant publication/capture metadata were read. The Foundation hosts digitizations, not contemporaneous website captures. Public response availability grants no portrait or reproduction license; originals and rendered images remain outside Git.

1. The undated Cambadélis relinquishment report borrowed **2017-06-17** from the start of a printed **17–30 June** issue range. Page 2 already reports a June 20 meeting. Its exact publication day and the announcement attestation are now **null**; the printed issue range and actual June 20 claim remain.
2. The 2025 congress ratification borrowed **June 5** from an earlier vote. Its exact attestation is now **null**. Votes on May 27 and June 5 and the result remain in the claim; the page publication stamp is not an effective ratification date.
3. Jospin's FLNKS communiqué is the lower-left item dated **18 October 1995**, not an adjacent October 17 item. Only its locator changed; its observation date remains October 18.

The exact 13-holder JSON block remains byte-identical to the authored import (SHA-256 `2c486c370641824cf3a1af8f8c71242358625bd14ffefb5eaa7992b0e456feb1`). All `from`/`until` values remain null. Earlier 110 France sources, both state institutions and all non-PS organization objects remain unchanged. The PS organization remains an unreconciled CNCCFP identity with no game-party binding or inferred lifecycle.

Rocard's provisional presidency, Hollande's delegated responsibility, Temal's coordination and collective interim arrangements remain claims only. Emmanuelli's dated first-secretary styling does not establish a permanent mandate: the same issue discusses later congress confirmation and contains provisional commentary. Faure's retrospective March 15 claim remains separate from the contemporary orientation vote and March 29 election. Conservative selection is not a ruling that other explicit office styling is false.

## Code and validation

The importer extension appends roles to a known reporting ID with source/claim collision and ownership checks. Older tests retain exact source/institution separation and restrict the additional organization role to PS only; no broad exclusions or lower acceptance thresholds were adopted. The new date and locator regressions first failed on the submission (three failures); after correction the **48 France/importer tests pass**. The separate **16 campaign research/census tests pass**. These counts are separate suites; no full avatar/native/performance run was made.

Importer freshness and a locally regenerated research-index check pass. The incoming index was never imported, and the local generated index was restored before commit. The first research/census attempt had a missing sparse `nations.rs` fixture; its failure is retained. Exact HEAD `nations.rs` and `government.rs` were materialized, after which all 16 pass. No test was skipped to obtain that result. Root owns combined metadata regeneration, central queue and publication.

Raw commands, exit codes, expected red failures and final logs are under `validation/`. `source-attempts.json`, `source-review.json`, `claim-review.json`, `holder-review.json`, `code-review.json`, `scope.json` and `manifest.json` distinguish identity, interpretation and scope.

## Reproduction

External originals and working reading aids: `D:/spheres-offload/codex-next-20260928/claude-review-20261001-04/france-evidence`. Full copyrighted originals are not committed.

```powershell
python docs/campaign-certification/C01/reviews/CLAUDE-C01-47-20261001/verify.py --require-validated --repo . --external-root D:/spheres-offload/codex-next-20260928/claude-review-20261001-04/france-evidence
```

Without optional paths, the verifier checks public receipt integrity/counts. `--repo` checks pinned committed source blobs; `--external-root` checks original response bytes. It does not independently recreate the historical reading or convert hash/test success into acceptance.
