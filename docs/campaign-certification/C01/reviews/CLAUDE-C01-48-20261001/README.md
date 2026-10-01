# Independent review — CLAUDE-C01-48 India

**Decision: accept the bounded research intake with the five-file review repair.** This does not establish complete party histories, create runtime identities, authorize art, or close C01, C06, S23, WC1 or CP1. No integration or Tonga checkout was modified, and no commit was made.

Reviewer: Codex `/root/tonga_civilian_roster`, 1 October 2026 UTC. Exact submitted branch `origin/claude/c01-in-48` at `bb0f86cded9fb0d196f32e385cbe7f30d3cecded`, compared against `5ea4f8fcd05e1858b1dfa83ae34c6d3703d6b104`. The isolated detached checkout is the sibling `india` directory. The authored submission is `8ce5058b1404733123aafde094fa07c85a4d6f07`; the tip commit changes only the generated research index.

## Source and historical assessment

All **19 primary originals** were independently retrieved: **16 sequential Internet Archive responses and three current AAP REST records**. All returned HTTP 200 with exactly the submitted byte count and SHA-256. There were no retries or rate-limit responses. Commands, request times, response metadata, headers and raw bodies are retained in [source-retrieval.json](source-retrieval.json) and `originals/`. The Archive lane was released after retrieval. No source identity was silently replaced.

Every material passage supporting the **24 new claims and four holder observations** was read. The six PDFs were checked through their material passages, including **11 visually inspected pages**, not merely their text layers. The remaining pages of the 86-page ECI compilation were not certified. [source-review.json](source-review.json) pins each reading aid and inspected render; [claim-review.json](claim-review.json) records a separate disposition for every claim. Matching hashes and passing tests are not the basis for material acceptance.

The four accepted observations remain exactly as submitted:

| Role/person | Observed on | Effective start | End |
|---|---|---|---|
| AAP — Arvind Kejriwal | 2013-10-07 | Unknown | Unknown |
| AAP — Arvind Kejriwal, separate entry | — | 2016-04-27 | Unknown |
| BSP — Mayawati | 2012-07-16 | Unknown | Unknown |
| NPP — Conrad K. Sangma | 2020-07-11 | Unknown | Unknown |

The certified AAP minutes visibly state an effective date for the named office bearers; that supports the 2016 start rather than an election-only inference. The 2015 letter's handwritten correction and printed annexure support the recalled 25 November 2012 election, retained only as a claim. Mayawati's signed 2012 letter and attached office-bearer list establish the dated observation. The NPP release connects its publication date, its same-day conclave and the explicitly named National President. [holder-review.json](holder-review.json) preserves the actual holder objects and their limits.

Kanshi Ram and Purno Agitok Sangma remain claim-only founders. A founder's death and a printed lifespan do not establish death in office or a dated term. Mayawati's 2003 retrospective assumption statement is real and preserved, while its use as a structured boundary remains unadopted. Conrad's recalled March 2016 election remains month-only. ECI catalogue wording is accepted only as catalogue evidence: the underlying orders were not read. The NPP committee's 2025–2028 tenure does not extend factual presidency coverage beyond the cutoff.

The three AAP REST responses are current representations with publisher-supplied historical dates. They are not independently timestamped contemporary captures. Post 883198 reports modification on 21 November 2024, three days after publication; the cause is unknown. Reproducing its bytes does not establish an unchanged original body. Acceptance is confined to the attributed office statements now served; unrelated political claims and claimed national recognition are outside this review.

## Defects and exact repairs

1. The new `elif` in `test_india_research_s10e.py` accidentally moved the earlier CPI(M) publication and claim cutoff checks into the new packet branch. Both prior checks are restored, with independent negative controls for prior and new packet sources. The actual pre-repair run failed both CPI(M) mutation subtests because post-cutoff dates were wrongly accepted. The repaired 86-test run passes them. Other exact prior fixtures and negative cases remain intact; all seven modified older test diffs were inspected in [earlier-test-audit.json](earlier-test-audit.json).
2. The Conrad biography's submitted “fourth paragraph” locator points after the election passage. It now identifies the paragraph beginning with the election statement. The extract and packet locator agree, and the extract byte/hash pin is updated. The locator regression failed before repair and passes afterward. [locator-repair.json](locator-repair.json) records both forms. Claim wording, dates, event classification and all holder objects are unchanged.
3. The report now explicitly states the current REST representation limitation above.

The exact repair is [review-repairs.patch](review-repairs.patch): **five files, 41 insertions and five deletions**. [reviewed-files.json](reviewed-files.json) pins the 30 authored source files and reviewed workfiles. Import those authored files plus the repair; do not import the isolated generated index. The submitted report and handoff retain their original descriptions of the author's checks; this receipt supersedes their assertion that no guard was weakened and independently supplies the PDF visual review.

[scope-audit.json](scope-audit.json) proves that removing the packet's scoped additions reproduces the base data exactly: all 317 earlier sources, 625 earlier claims, both institutions and 80 other organizations are unchanged. The three recognition observations retain their identities, unresolved lifecycles and empty game mappings. The current repaired data differs from the submitted data only in the single new locator and its extract snapshot pin.

## Validation and retained failures

Final passing checks: **86 India tests**, **80 research tests**, **16 campaign tests**, **11 Node atlas tests**, census, workboard and scoped whitespace checks. The Python discovery patterns overlap; these are not a summed count of unique tests. Exact commands, times, logs and their hashes are in [validation/results.json](validation/results.json).

Both intentional red controls remain. An intermediate 86-test run was mistakenly labelled `india-focused-final`: a repair-script indentation assertion stopped after changing the extract but before updating the packet checksum. That run has 16 failures and 16 errors and remains intact. The corrected coherent tree passes `india-focused-after-repairs`. This transient mismatch is not a source acceptance defect or a waived test.

After the locator repair, the generated index correctly failed its old-pin check. Temporary regeneration and `--check` passed, changing only the India packet byte/hash fields. The original submitted index was then restored byte-for-byte. [index-review.json](index-review.json) records that controlled operation. **The integrator must regenerate the shared index after combining accepted packets.** Other attribution, gap-ledger or boundary metadata needs the integration's accepted revision; this review did not fabricate it or qualify those gates. No native, gameplay or production-browser check was run.

The checkout was narrowed to relevant paths to avoid copying the old binary-evidence tree. Five missing check dependencies were materialized from exact submitted blobs, including the S26 handoff that caused the author's sparse-checkout workboard failure; [supplemental-check-blobs.json](supplemental-check-blobs.json) pins them. They were not changed by the review. A slow full Git status was not used as evidence; the exact authored files, source blobs and five-file patch were checked directly.

## Remaining historical research

The intake does not close gaps for Kanshi Ram's 1990–2003 presidency, Purno Agitok Sangma's dated national presidency, exact assumption/end boundaries, intervening AAP/BSP organizational elections, acting arrangements, or uninterrupted continuity to 7 September 2026. The adopted final claims are AAP 25 August 2026, BSP 1 September 2021 and NPP 17 April 2025; none certifies a complete chain. Organization reconciliation and runtime mappings remain open. The 10 September 2026 AAP lead is beyond the fixed cutoff.

All complete source bodies and page renders remain outside Git here. No portrait, photograph, scanned source asset or open-license permission is granted. Run `python -B -X utf8 verify_receipt.py` to recheck retained bytes and reviewed-file identities; that verifier does not repeat the historical reading.

## Integration packaging

The independent review above is preserved verbatim. Its no-commit statement describes the agent review, before root imported it. Exact import `72d3d80d` and repair `0902fb61` are pinned in `integration.json` and `git-inputs.json`. Run this directory's `verify.py` to check retained external originals and accepted Git blobs. The shared generated index is excluded.
