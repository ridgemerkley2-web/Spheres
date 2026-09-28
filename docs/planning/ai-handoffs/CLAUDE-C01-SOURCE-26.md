# CLAUDE-C01-SOURCE-26: Review Vedomosti facsimile provenance

Owner: Claude. State: **ready_for_review** (submitted 28 September 2026 UTC; not complete). Parent: C01 (incomplete).

Assigned by `docs/planning/ai-handoffs/CLAUDE-C01-NEXT.md` and `docs/planning/ai-task-queue.json` (task
`CLAUDE-C01-SOURCE-26`, queued 27 September 2026) as a source-review follow-up to the integrated CLAUDE-C01-26.
It repairs integrated research; it does not claim historical acceptance, which Codex decides.

Branch: `claude/c01-source-26`. Base: `a33a8987` (current `codex/campaign-certification`). Claim commit: this
record's first commit on the branch.

## Bounded repair

Document provenance and legibility for the Vedomosti issue scans (vedomosti.sssr.su /1991/<n>.pdf and the Russian Historical Society copies) permitted by CLAUDE-C01-26, and explain why the PDFs can qualify as primary facsimiles while the same host's HTML transcriptions remain excluded; Codex decides acceptance, and a test allowlist is not historical proof. Also apply the six structural defects the C01-26 verifier found after its submission (handoff defect count, a transliterated name, a row without attested_on, unpinned holder names, the undisclosed loosening of the C01-05 host guard, and the Izvestia response obtained only with a browser User-Agent past an anti-robot block, which the C01 rules do not allow), and complete the source verification that did not run.

Keep historical dates unchanged unless the cited evidence supports a correction. Do not convert missing
response identities into fabricated checksums; retain disclosed limitations and propose further bounded review
separately. Never bypass an access control, CAPTCHA or bot block; record a blocked source instead.

## Allowed files and checks

Allowed files:

- this record;
- `docs/campaign-certification/C01/research/ussr.json` (CLAUDE-C01-26 source records, claims and holders only);
- CLAUDE-C01-26 extracts `docs/campaign-certification/C01/research/sources/ussr-*-facts.json`;
- `docs/campaign-certification/C01/research/ussr-government-and-supreme-soviet-1990-1991-26.md`;
- `tools/avatars/test_ussr_government_supreme_soviet_c01_26.py`, `tools/avatars/test_ussr_russia_transition_c01_05.py` and `tools/avatars/test_ussr_research_s10h.py` where they pin changed values;
- `docs/campaign-certification/C01/research-index.json`, regenerated in a separate commit.

Checks: research-index `--check`; the USSR, research and campaign Python tests with game data present
(campaign census included); `campaign_census.py --check`; the atlas Node check; `workboard.py --check`;
`git diff --check`.

Return `ready_for_review` with exact commits, original-response identities where available, content locators and
remaining gaps.

## Submission (ready_for_review)

Result commits on `claude/c01-source-26`, after the claim commit `529a3ebb`: "Review source for CLAUDE-C01-SOURCE-26"
(`4bb1b3a7`; every file below except the index) and "Regenerate the C01 research index for CLAUDE-C01-SOURCE-26" (`c1f537f2`;
`research-index.json` only), the two commits at the branch head at submission. The verifier's fixes follow in two more commits:
"Apply verifier fixes to CLAUDE-C01-SOURCE-26" (the Vedomosti page images described as 300 dpi bitonal, the No. 36 imprint date
quoted as printed, the Russian Historical Society copies' undocumented issue date and imprint, and the report's per-request
exception marked withdrawn; six extracts, `ussr.json`, the report and this record) and "Regenerate the C01 research index after
CLAUDE-C01-SOURCE-26 fixes" (`research-index.json` only). When the fixes were made, `codex/campaign-certification` was at
`e41aa18d`; its commits since `a33a8987` touch none of this repair's files, so no merge was needed. Report: the new section
"Source review (CLAUDE-C01-SOURCE-26)" at the end of
`docs/campaign-certification/C01/research/ussr-government-and-supreme-soviet-1990-1991-26.md`. Codex decides acceptance; the
tests that admit the scans are not historical proof.

Method: every recorded and attached response of CLAUDE-C01-26 downloaded again twice with plain curl 8.16.0 (its own
User-Agent, `Accept-Encoding: identity`, no cookies, no cache-busting), 2026-09-27T23:40:28Z-23:42:16Z and
2026-09-28T00:15:03Z-00:18:32Z (32-36 minutes apart per response); the cited pages of every scanned PDF rendered from the
recorded bytes with pypdfium2 and read from the page images; the facsimile images viewed; the HTML quotations matched against
the response texts. No browser User-Agent was sent and no block was worked around. (The post-submission structural check
disclosed that its own spot re-download of the Izvestia page had used the packet's recorded browser User-Agent once, before the
concern was recognised.)

Part 1, provenance and legibility (details in each extract's `source_review.provenance` and in the report):

- vedomosti.sssr.su (the SSSR.SU portal: self-described "прообраз официального сайта", non-commercial, registered as the media
  outlet "Информационное агентство «СССР»", ИА № ФС77-40345; not a state body or the publisher) serves page-image PDFs of 1991
  Nos. 35-38 and 41 only; its HTML pages are a retyped transcription. Each PDF is one 300 dpi bitonal image per printed page
  (covers in colour layers) with an ABBYY FineReader 11 OCR layer, not a re-typeset text.
- No. 35 (28 Aug 1991; imprint p. 1411 "28.08.91", "Зак 2981", the Izvestia printing house, Pushkinskaya pl. 5; printed pp.
  1409-1431 on PDF pp. 3-25): IA capture `https://web.archive.org/web/20211204065955id_/https://vedomosti.sssr.su/1991/35.pdf`,
  618,372 bytes, SHA-256 `5a8c0630da633ac68ef5c0514c05107497a9e84cda82541d561bc22013c55a63`, and the live file identical on both
  passes; cited PDF pp. 7, 8, 14, 15, 17, 18, 23, 25 legible.
- No. 36 (4 Sep 1991; imprint p. 1435 "04.09 91.", "Зак 3385"; pp. 1433-1470 on PDF pp. 3-40): IA capture 20250820135020,
  1,303,663 bytes, `87abb4c154470c0681cd0867ef63119675b249d323372fdcbfe87a34ab075b6f`, live file identical; cited PDF pp. 13, 15,
  17, 38, 39, 40 legible.
- No. 37 (11 Sep 1991; imprint p. 1474 "11.09.91", "Зак. 3418"; pp. 1473-1502 on PDF pp. 3-32): live file
  `https://vedomosti.sssr.su/1991/37.pdf`, 999,248 bytes, `3e77c34e4da247391f08d019c29bb24d9c94d28eeb6debda1efb95944eb603b0`, the
  same as raw capture 20240915135709; cited PDF pp. 15, 17, 18, 26, 31, 32 legible.
- No. 41 (9 Oct 1991, "Ведомости Верховного Совета СССР"; imprint p. 1566 "09.10.91", "Зак. 3978"; pp. 1565-1584 on PDF pp.
  3-22): live file `https://vedomosti.sssr.su/1991/41.pdf`, 556,781 bytes,
  `91cf65718b9dceaeb8cb4b4c6ce4eccf337af45afa3cdb897f197c97bdff902e`, the same as raw capture 20240906045835; PDF p. 20 legible.
- Russian Historical Society copies (docs.historyrussia.org, the Society's ЭБИД on ИнфоРост; a non-official host): photographs of
  the bound volume "Собрание постановлений правительства РСФСР за 1990 г. № 1-25. — М.: Юрид. лит., б. г. — 648 с.", No. 8,
  printed pp. 202 and 194 (art. 59; 30,424 bytes `d795d3dd…91a236`, 37,089 bytes `55bda52a…30a892`) and 214 and 203 (art. 60;
  31,779 bytes `20bd55b2…78d65f`, 38,198 bytes `3ab7bcd6…f404f9d2`), 328 px wide; headings, signatures, dates and numbers legible
  at 3x magnification; the photographed pages print neither the issue's date nor a printer's imprint.
- Why the PDFs and not the host's HTML: the PDFs are images of the official publication (its own pages, imprint and page
  numbers), so every quotation can be checked against the page; the HTML is the host's own text. The review found the OCR text
  wrong in five quotations (below), exactly what a transcription would carry unchecked.

Part 2, the six structural defects: (1) `CLAUDE-C01-26.md` now says "30 applied and C12 applied in part"; (2) the claim and row
now quote "М. С. Горбачев дополнил его сообщение" (stenogram vol. III, printed p. 163, PDF p. 165); (3) the row
`su_ved35_presidium_under_chamber_chairs_19910821` has `"attested_on": null`; (4) `ROW_HOLDERS` is pinned in
`test_ussr_government_supreme_soviet_c01_26.py` and both hand mutations fail; (5) the handoff and the report's Integration notes
call the C01-05 vedomosti.sssr.su guard change a loosening that needs the integrator's approval with the C12 ruling (the guard
itself is unchanged); (6) Izvestia: no permissible alternative was found, so the Izvestia source
`su_izvestia_19910115_no13_p1` (the Yandex archive page, previously 171,140 bytes `db9ec251…ea6847`, obtained with a browser
User-Agent), its extract, its three claims (`su_izv13_president_submission_recited_19910114`,
`su_izv13_vs_approves_pavlov_premier_19910114`, `su_izv13_lukyanov_signs_as_vs_chair_19910114`) and holder 3 (Валентин
Сергеевич Павлов, `attested_on` 1991-01-14) are withdrawn. Plain requests got HTTP 403 with `X-Yandex-Captcha: 403` for the
page and its preview on both passes (not bypassed). Searched without a workaround: no Internet Archive capture of the page;
`vedomosti.sssr.su/1991/4.pdf` 404 and no capture; `izvestija.sssr.su/1991/13mv.pdf` 404; the Rada database's `v1900400-91`
anti-DDoS 403 (not bypassed) and no capture; no Union act of 14.01.1991 on the legal portal; the RHS library's search HTTP 500
and no Vedomosti there; no archive.org scans. SU-GOV-05 has no permissible primary record, pending Codex's ruling; Павлов's one
observation is his signature of 19 August 1991.

Part 3, source verification: 30 source responses and 11 attached responses, 41 of 41, byte- and SHA-256-identical to the
extracts on both passes (one connection failure retried at 23:42:16Z); raw pre-cutoff captures of Vedomosti Nos. 37 and 41 and
bulletins 1 and 2 match the recorded live files, correcting the packet's "no capture" statement. Reading the pages corrected six
quotations: `su_steno4_pavlov_reports_for_government_19901227` (defect 2),
`su_ss_res_2361i_presidium_removal_reference_19910822` ("сессии", No. 35 p. 1414),
`su_ved36_consent_to_pavlov_release_reported_19910828` (Cyrillic "Х", No. 36 p. 1469),
`su_cpd_res_2389i_lukyanov_released_19910904` ("Кремль. 4 сентября", No. 37 p. 1485),
`su_law_2392i_transition_and_entry_into_force_19910905` ("СССР. сессия", No. 37 p. 1488, a printing slip; the signed original
has a comma) and `su_sten2_presidium_removal_not_approved_no_quorum_19910826` (Council of the Union 163, not 153; bulletin No. 2,
PDF p. 43). No date, holder, event kind or locator changed.

Files: `docs/campaign-certification/C01/research/ussr.json` (40 sources, 95 claims; 199,067 bytes, SHA-256
`68b7d66a5ce6d5f4b88f5ea670c265e348f93ef689309b87967308d48e69c0b8`); the 30 remaining `ussr-*-facts.json` extracts of
CLAUDE-C01-26 (each with a `source_review`; 258,161 bytes in all) and the removed `ussr-izvestia-no13-19910115-facts.json`; the
report; `tools/avatars/test_ussr_government_supreme_soviet_c01_26.py` and `tools/avatars/test_ussr_research_s10h.py` (changed
values, the `ROW_HOLDERS` pin and stricter absence guards; nothing loosened); this record; and, outside this record's list but
named by its defect list, `docs/planning/ai-handoffs/CLAUDE-C01-26.md` (defects 1, 5 and 6). Separate commit:
`docs/campaign-certification/C01/research-index.json` (1,328 sources and 3,728 claims, from 1,329 and 3,731).
`test_ussr_russia_transition_c01_05.py` and `russia.json` are unchanged.

Checks (28 September 2026 UTC, sparse worktree with game data, not widened): `campaign_research.py` regeneration and `--check`
(1,328 sources, 3,728 claims); `campaign_census.py --check` exit 0; USSR tests 27, Russia 29, research 79, campaign 16 (census
included), all pass; atlas Node check 11 pass; `workboard.py --check` pass (44 markers); `git diff --check` clean on these
paths. Rerun after the verifier's fixes, also on 28 September 2026 UTC: the same results (the index regeneration changes only
`ussr.json`'s SHA-256).

Remaining gaps: SU-GOV-05 needs an official facsimile of Vedomosti 1991 No. 4 (art. 80); Codex's rulings on the withdrawn
Izvestia identity, the SSSR.SU and RHS facsimiles (C12) and the C01-05 guard loosening; an earlier permissible attestation of
Павлов (Cabinet resolution 274 of 22 May 1991 on the legal portal, or a Premier-signed act in the RHS's RSFSR 1991 gazette) is
proposed as C01-USSR-GOV-007; `docs/planning/ai-task-queue.json` still lists this task as queued (left to the integrator).
