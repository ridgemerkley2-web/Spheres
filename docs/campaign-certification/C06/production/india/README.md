# India cast production (C06)

Task `CLAUDE-C06-INDIA-01` (owner Claude), branch `claude/c06-in-01`. State: **awaiting Codex render**; not ready for review. Claude prepares identities, dated likeness references and exact prompts; Codex renders with its built-in image tool; Claude then reviews and registers the returned outputs. Claude generates, edits and labels no image.

## Batch 1 (IN-CAST-B01), prepared 2 October 2026

- [Identity review](identity-review-batch-01.json): accepted C01-27 (BJP) and C01-40 (CPI(M), with its reviewed amendment) holder observations copied exactly, requested appearance windows and their basis, pipeline and leadership-production job IDs, verified likeness references, rejected candidates, exclusions, verifier dispositions and an inline source ledger. Authority flags are all false.
- [Render request](render-request-batch-01.md): per job, the prompt file and sha256, the input order (style anchor first, STYLE ONLY; then the identity reference), 1024x1536 RGB, up to 3 attempts, and unchanged `exec-*.png` outputs to return.

Every person here is a party leader. Every India Prime Minister, President and Congress-president observation is still `c01_integrated_pending`, so the heads of state and government are not in this batch; the [handoff](../../../../planning/ai-handoffs/CLAUDE-C06-INDIA-01.md) lists them and the deferred party leaders with reasons.

| person_id | role | window (exclusive end) | reference | photograph date | licence | prompt |
|---|---|---|---|---|---|---|
| `harkishan_singh_surjeet` | primary | 1992-01-31 to 2005-04-01 | `harkishan-singh-surjeet-2005-reference-v1.jpg` | 2005-03-28 or 2005-03-29 | CC BY 2.5 | `harkishan-singh-surjeet-cartoon-1992-v1.txt` |
| `lal_krishna_advani` | primary | 1995-01-01 to 2005-12-31 | `lal-krishna-advani-2001-reference-v1.jpg` | 2001-01 on Commons (about 29 Jan to 9 Feb 2001) | CC BY-SA 3.0 (VRTS) | `lal-krishna-advani-cartoon-1995-v1.txt` |
| `prakash_karat` | primary | 2005-04-30 to 2015-04-01 | `prakash-karat-2013-reference-v1.jpg` | undated, 2013-03-21 to 2017-09-17 (camera stamp impossible) | CC BY-SA 4.0 | `prakash-karat-cartoon-2005-v1.txt` |
| `rajnath_singh` | primary | 2005-12-31 to 2014-07-09 | `rajnath-singh-2013-reference-v1.jpg` | 2013-10-27 or 2013-10-28 | CC BY-SA 2.0 | `rajnath-singh-cartoon-2005-v1.txt` |
| `nitin_gadkari` | primary | 2009-12-19 to 2013-01-23 | `nitin-gadkari-2011-reference-v1.jpg` | 2011-07-19 | Open Government Licence v1.0 | `nitin-gadkari-cartoon-2009-v1.txt` |
| `sitaram_yechury` | primary | 2015-04-30 to 2024-09-12 | `sitaram-yechury-2019-reference-v1.jpg` | 2019-12-27 or 2019-12-28 | CC0 1.0 | `sitaram-yechury-cartoon-2015-v1.txt` |
| `jagat_prakash_nadda` | primary | 2020-01-20 to 2026-01-20 | `jagat-prakash-nadda-2018-reference-v1.jpg` | 2018-09-26 | CC BY-SA 4.0 | `jagat-prakash-nadda-cartoon-2020-v1.txt` |
| `m_a_baby` | reserve, promoted to the render | 2025-04-06 to 2026-09-08 | `m-a-baby-2026-reference-v1.jpg` | 2026-07-06 | CC BY-SA 4.0 | `m-a-baby-cartoon-2025-v1.txt` |

Not rendered:
- `amit_shah` (primary, 2014-07-09 to 2020-01-20): **excluded at assembly**. The only usable in-window reference (File:Pic of Amit Shah IMG 9161.jpg, CC BY-SA 4.0, own work, no VRT ticket) is very probably from a commissioned BJP portrait session whose sibling frames the party distributed for download in 2016. A reference switch is not a mechanical fix, and the VRT-confirmed alternatives show only a small, lowered face. The copied reference and prompt were deleted before any commit, the jobs stay open, and the future routes are recorded in the identity review.
- `nitin_nabin` (reserve, 2026-01-20 to 2026-09-08): **not ready**. Both freely labelled Commons candidates fail on provenance; the rest are GODL-India or "Attribution". No reference or prompt was written.

Every preparation was checked by an independent verification agent. Rajnath Singh and Gadkari passed; the Nabin verifier agreed no qualifying photograph exists. The other findings are fixed in the records and prompts:
- Surjeet: the verifier showed that a rejected candidate, Soman's File:Bardhanhss.jpg, is dated inside the window (camera clock 28 March 2005; banner of the CPI's 19th Party Congress, 29 March). The assembler fetched it, made it the reference and rewrote the prompt. The undated Surjith-6.JPG (late July 2003 to August 2008) stays in scratch as the colour and detail alternate, with its frame description corrected.
- Advani: the identification history, the date range (field hospital in operation about 29 January to 9 February 2001), the licence provenance (the Flickr source was NC; the free licence rests on VRTS ticket 2021051710001034) and the coat-hem wording.
- Karat: the camera stamp (16 March 2013) is impossible for a Canon EOS 700D, which was announced on 21 March 2013. The date is now a range, and the prompt, age note and credit are corrected.
- Yechury: two candidate days, the job structure, and the semi-rimless glasses in the prompt.
- Nadda: the holder dictionaries are copied whole, the birth date is cited, two rejection reasons are corrected, the complexion wording is fixed and the ICC profile is noted.
- Baby: one wording fix.

The missing LF rule for the prompt files is recorded but not fixed here (it needs `.gitattributes`, outside this task's files).

Age and date limits declared in the prompts: Surjeet's photograph is from the window's last days (89, drawn about 80). Advani's is mid-window (73). Karat's is undated within 2013-2017 (65 to 69, drawn about 62). Rajnath Singh's is late in the window (62, drawn about 58 to 60). Gadkari's (54) and Baby's (72) are inside their windows. Yechury's is at the window's midpoint (67). Nadda's is 16 months before his window (57, drawn 59 to 65).

Codex decisions requested: the Surjeet window bounds and a possible split at 2000-01-01; Advani's split and costume continuity; the Karat reference date and naming, or the in-window alternate; Rajnath Singh's split; the widened Gadkari end; Yechury's split; the Nadda end and era dress. See the render request.

## Evidence and privacy

Original Commons metadata and image bytes stay outside Git under `D:/spheres-scratch/in-cast/refs/` and are pinned by sha256 in the identity review. The committed references are byte-identical copies. No HTTP header capture is kept, and 429 or 404 error bodies, which can contain the client IP address, were deleted unsaved. The assembler made five further single requests (Bardhanhss metadata, original and file page; the Advani revision history; Wikipedia lead extracts for birth dates); all returned HTTP 200.

## Checks at preparation

- `python -B -X utf8 tools/avatars/person_art_pipeline.py self-test`: exit 0; 21 tests OK, after `git sparse-checkout add spheres-web/ui/leader-art spheres-web/ui/portraits` (the self-test reads fixtures there).
- `python -B -X utf8 tools/avatars/person_art_pipeline.py validate`: exit 0; valid true, 0 errors, 530 warnings. person_portraits.json is unchanged, so no new portrait is registered.
- `pipeline approved_source_license`: true for all eight references (the Gadkari OGL v1.0 record carries the source-specific grant fields).
- `python -X utf8 tools/planning/workboard.py --check`: PASS: 44 canonical markers, 59 bounded tasks.
- `git diff --cached --check` (staged batch files): clean.

These checks show the existing manifest and tools still pass with the new files present. They are not a visual review, a registration or country sign-off (CS-India stays with the user and Codex).
