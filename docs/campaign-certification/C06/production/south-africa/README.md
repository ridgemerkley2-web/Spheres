# South Africa cast production (C06)

Task `CLAUDE-C06-SOUTHAFRICA-01` (owner Claude), branch `claude/c06-za-01`. State: **awaiting Codex render**; generated-art review remains pending. Claude prepares identities, dated likeness references and exact prompts; Codex renders with its built-in image tool; Claude then reviews and registers the returned outputs. Claude generates, edits and labels no image.

## Batch 1 (ZA-CAST-B01), prepared 1 October 2026

- [Identity review](identity-review-batch-01.json): accepted C01-30 holder observations copied exactly, requested appearance windows and their basis, pipeline and leadership-production job IDs, verified likeness references, rejected candidates, verifier dispositions, the four windows that are not ready, the unprepared reserves, the deferred heads of state and an inline source ledger. Authority flags are all false.
- [Render request](render-request-batch-01.md): per job, the prompt file and sha256, the input order (style anchor first, STYLE ONLY; then the identity reference), 1024x1536 RGB, up to 3 attempts, and unchanged `exec-*.png` outputs to return.

The batch was planned as eight windows for six people (the IFP, Freedom Front and ACDP leaders with accepted C01-30 observations). Four windows have a qualifying photograph and go to Codex. Four have none and are recorded as not ready.

### In the render (four windows)

| person_id | window (exclusive end) | reference | photograph date | licence | prompt |
|---|---|---|---|---|---|
| `mangosuthu_buthelezi` | 1995-01-01 to 2005-01-01 | `mangosuthu-buthelezi-1983-reference-v1.jpg` (Rob Bogaerts / Anefo, Nationaal Archief 932-6173) | 1983-06-10 | CC0 | `mangosuthu-buthelezi-cartoon-1995-v1.txt` |
| `constand_viljoen` | 1994-03-31 to 2001-01-01 | `constand-viljoen-1984-reference-v1.jpg` (Ian Barbour, Commons crop) | 1984-09 (month only) | CC BY-SA 2.0 | `constand-viljoen-cartoon-1994-v1.txt` |
| `pieter_mulder` | 2001-06-21 to 2016-11-13 | `pieter-mulder-2013-reference-v1.jpg` (U.S. Department of Agriculture, photo by Blake Woodhams) | 2013-09-16 | CC BY 2.0 | `pieter-mulder-cartoon-2001-v1.txt` |
| `corne_mulder` | 2025-07-16 to 2026-09-08 | `corne-mulder-2023-reference-v1.jpg` (Houses of the Oireachtas, Commons crop) | 2023-06-14 | CC BY 2.0 | `corne-mulder-cartoon-2025-v1.txt` |

After successful rendering, visual review and registration, these four windows are projected to cover 10 of the 24 leadership-production cartoon jobs for the six people. Preparation closes zero jobs; all 24 remain open.

### Not ready (four windows; nothing copied, no prompt written)

| person_id | window (exclusive end) | why |
|---|---|---|
| `mangosuthu_buthelezi` | 2005-01-01 to 2019-08-25 | The closest qualifying photographs are the 1983 Anefo frames (he is 54 in them, 76 to 90 in the window). In-window images are an eNCA TV still, two 2018 Taiwan files under the non-listed 'Attribution' (GWOIA) licence, and CC BY-ND GovernmentZA and GroundUp photographs. |
| `velenkosini_hlabisa` | 2019-08-24 to 2026-09-08 | Commons has only an eNCA video still. Every Flickr photograph is CC BY-ND, CC BY-NC-ND or all rights reserved. |
| `kenneth_meshoe` | 1993-12-31 to 2010-01-01 | Commons has only video stills (a Vimeo screenshot dated 2014 on Commons, titled 2013, and a 2019 YouTube still). Flickr photographs are CC BY-ND or all rights reserved. A Clinton Library lead from his March 1998 visit is unexplored. |
| `kenneth_meshoe` | 2010-01-01 to 2026-09-08 | Only CC BY video frames, plus a CC BY-ND 2.0 GovernmentZA photograph from 2025. |

Their 14 leadership-production jobs, their window-scoped pipeline jobs and the default inventory jobs all stay open. The reserves `pieter_groenewald` and `mzwanele_nyhontso` were not prepared, because neither has a registry term and each window needs Codex agreement first.

### Verification and assembly fixes

Each window had one preparation agent and one independent verification agent. The verifiers confirmed all four not-ready verdicts. Their findings on the four render windows were fixed in the records and prompts:

- Buthelezi: the verifier named Bush White House contact sheets from 1990 and 1991. The assembler fetched and inspected roll WHPO-P22804 (NARA NAID 543950122, 20 June 1991, Susan Biddle). It is a scanned contact sheet: six small frames of a crowded luncheon table, with printed labels and film-edge text. It cannot be used unchanged, so it and the seven other sheets are rejected, and a single-frame scan is left as an open item for Codex. The birth date now has a cited source (South African History Online; Wikidata Q554131). The reference bytes were also switched to the Commons original: same frame, sha1 verified against Commons. The preparer had used the Nationaal Archief's on-the-fly grayscale rendition while Commons returned HTTP 429.
- Viljoen: the prompt now notes the partial face of a guardsman at the top-left of the crop. The birth date is cited (Wikidata Q2568741). The default-inventory job has projected partial coverage after rendering, review and registration.
- Pieter Mulder: the preparer had his birth date wrong. It is 26 July 1951 (Wikidata Q770995), not 1954, so the prompt ages are corrected: 49 to 65 across the window, 62 in the photograph, drawn at about 57. A sixth Commons file is added to the rejected candidates, and the FlickrReview date is corrected.
- Corné Mulder: the prompt now describes his glasses as browline frames (black top bars, silver lower rims), not plain dark frames.
- Privacy: ten Flickr pages with GeoIP fields left in the session scratchpad by a preparer were deleted. None was cited or hashed.

Age and date limits declared in the prompts:
- Buthelezi's photograph is from 1983, 11.6 to 21.6 years before his window.
- Viljoen's is a small crop from September 1984, 9.5 to 16 years before his window, showing him in uniform, which is not copied.
- Pieter Mulder's is from 2013, inside his window but late in its 15.4 years.
- Corné Mulder's is from June 2023, two to three years before his window.

Codex decisions requested (details in the render request):
- whether to render Buthelezi from the 1983 frame or first obtain a 1991 single-frame scan;
- the Viljoen reference;
- the Pieter Mulder window start, and a possible split of his window;
- the Corné Mulder derivative crop;
- the not-ready windows and reserves;
- whether a video frame may ever serve as a reference;
- an `eol=lf` rule for the four prompts.

The pinned gap-ledger snapshot refreshed at `1fffb983` records 32 `c01_integrated_pending` observations across three separate offices: C01-09 heads of state, C01-16 ANC presidents and C01-21 deputy presidents. These office observations are outside this batch; this preparation review changes no historical acceptance status. Proposed batch-2 windows and job IDs are under `deferred_batch_2` and require the applicable office's current acceptance record, rather than a general acceptance inferred from the person's name.

## Independent preparation review, 2 October 2026

Codex reviewed exact submitted tip `c0382bbcb92852ff6e885292bb3ecfe9a7793a75`: **adopt the four references and prompts as preparation with these documentation corrections; hold generated-art approval and registration**. All four photographs were visually inspected and the prompts read. The four reference copies match retained originals and SHA256/Commons SHA1; fifteen supporting metadata pins and four prompt Git-byte hashes match. All nineteen copied holder objects match accepted research exactly. Age interpretations remain subject to visual review of the actual outputs.

Viljoen's eventual cartoon adaptation must explicitly retain **CC BY-SA 2.0** as its derivative licence, with `derivative_license`, [licence URL](https://creativecommons.org/licenses/by-sa/2.0/), Ian Barbour attribution and an adaptation notice. The `generated` label and source rights on `identity_source` alone do not record that obligation. No rendered portrait, historical acceptance, human approval or country signoff is granted here.

## Evidence and privacy

Original Commons and archive metadata and image bytes stay outside Git under `D:/spheres-scratch/za-cast/refs/`, pinned by sha256 in the identity review. The committed references are byte-identical copies, and each matches its Commons sha1. HTTP header captures and pages that contain the client IP address or GeoIP fields are not kept, copied, cited or hashed.

## Checks at preparation

The local sparse checkout was widened for the checks with `git sparse-checkout add` for `spheres-web/ui/{leader-art,portraits,government-art,display-art}`, `docs/campaign-certification/S10/c`, `docs/campaign-certification/C04`, `docs/campaign-certification/verification` and `docs/campaign-certification/S23/preparation/boundary-matrix`. This is worktree configuration only and is not committed.

- `python -B -X utf8 tools/avatars/person_art_pipeline.py self-test`: exit 0; 21 tests OK.
- `python -B -X utf8 tools/avatars/person_art_pipeline.py validate`: exit 0; valid true, 0 errors, 530 warnings. person_portraits.json is unchanged, so no new portrait is registered.
- `python -X utf8 tools/planning/workboard.py --check`: PASS: 44 canonical markers, 59 bounded tasks.
- `python -B -X utf8 tools/avatars/cartoon_review.py --check`, `leadership_production.py check`, `campaign_census.py --check`: exit 0.
- `python -B -X utf8 -m unittest discover -s tools/avatars`: 827 tests, 1 failure, which was already present at the base. `test_certified_gap_ledger.CheckedInLedger.test_committed_output_is_current` fails because `C01/gap-ledger/ledger.json` (refreshed at `1fffb983`) pins `docs/planning/ai-task-queue.json` at 55,081 bytes, and base commit `2fd186d6` later changed the queue (now 55,208 bytes). This batch touches neither file, and regenerating the ledger is left to Codex's integration.
- `git diff --cached --check` (staged batch files): clean.

These checks show the existing manifest and tools still pass with the new files present. They are not a visual review, a registration or country sign-off (CS-SouthAfrica stays with the user and Codex).

Integration note, 2 October 2026: all new prompts in this packet now have specific LF checkout rules in `.gitattributes`. The historical Windows CRLF values remain diagnostic; use the corrected exact LF hashes for generation.
