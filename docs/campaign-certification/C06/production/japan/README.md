# Japan cast production (C06)

Task `CLAUDE-C06-JAPAN-01`, branch `claude/c06-jp-01`. State: **awaiting Codex render after independent preparation corrections**. No output is rendered, registered or visually approved; no country or qualification gate is complete.

## Batch 1 (JP-CAST-B01)

[Identity review](identity-review-batch-01.json) preserves all 14 accepted C01-29/C01-31 holder records exactly. [Render request](render-request-batch-01.md) pins the current prompts and references. [Independent review](independent-review-20261002.md) records the 2 October review of exact submitted tip `7f8c5ed1e8d5b8d4f5ee61e72376741d34ff1725` and the bounded corrections.

| Person | Requested appearance (exclusive end) | Reference date and disposition |
|---|---|---|
| Tomiichi Murayama | 1993-09-30 to 1996-09-01 | Undated official Kantei portrait, CC BY 4.0. Later APEC 1995 page/image captures do not establish a hard 1995 deadline. Corrected prompt uses illustrative age. |
| Makoto Tanabe | 1991-07-31 to 1993-01-19 | April 1992 MOFA image, CC BY 4.0; Commons caption says 7 April. Primary album not independently read. |
| Sadao Yamahana | 1993-01-19 to 1993-09-01 | 9 August 1993 official film frame, CC BY 3.0. Usable with careful review of the soft likeness. |
| Takako Doi | 1996-09-30 to 2003-11-01 | 2 July 2005 Akira Kamikura photograph, CC BY 2.0. Earlier age/glasses treatment is explicitly artistic. |
| Akihiro Ota | 2006-09-30 to 2009-09-08 | Undated MLIT official portrait, verified first Commons revision, CC BY 4.0. Editing and PDF creation timestamps are metadata only. Corrected prompt discloses illustrative ageing. |
| Natsuo Yamaguchi, early part | 2009-09-08 to 2015-01-01 | 7 July 2013 Ogiyoshisan campaign photograph, CC BY 3.0 (new, era fix of 2 October 2026). Age 60 in the photo, 57-62 across the part. Suit and tie are artistic extensions; awaiting Codex review. |
| Natsuo Yamaguchi, later part | 2015-01-01 to 2024-09-28 | 29 August 2019 MOFA TICAD photograph, CC0 (reused unchanged). Age 67 in the photo, 62-72 across the part. |

The proposed twelve production jobs are **targeted coverage only**: generation, registration and output review must happen before any can close. Default-inventory residuals remain open. Appearance windows never establish continuous office tenure. The January 1996 Murayama bridge changes no historical claim. Yamaguchi's single 2019 reference did not prove his appearance throughout all four era jobs, so his window is now split at 2015-01-01 (see the era fix below); his superseded single-window prompt and job are not rendered.

The six reference files remain byte-identical. Murayama/Ota year-containing filenames are retained identifiers, not capture-date evidence. Only their two prompts changed; LF hashes are re-pinned. The other four prompts are unchanged.

Takenori Kanzaki remains outside this batch, and Yoshida/Mataichi were not prepared. His earlier exclusion was the preparer's scope choice, not a user rule against licensed official film frames. Reviewing his separate source is future work.

## Era fix, 2 October 2026

[Yamaguchi era split](yamaguchi-era-split-20261002.json) (sha256 `3e43bf7c81e8caeda044b267fe77bb3237c8306486d5da0376bf81f58c4757a5`) answers the independent review's recommendation. It was prepared, independently verified and assembled by Claude workflow agents and is a proposal awaiting Codex review; it is not human, Codex or user approval.

- Split at 2015-01-01, a leadership-production job edge. It is an art choice, not an office, tenure or physical-change date. The four cartoon jobs divide two and two, and the seven accepted C01-31 observations divide three and four.
- Early part: new reference `natsuo-yamaguchi-2013-reference-v1.jpg` (File:Natsuo Yamaguchi IMG 5607 20130707.JPG, Ogiyoshisan own work, 7 July 2013, licence exactly 'CC BY 3.0', unported; 1200x1600; byte-identical to the single Commons file version). New prompt `natsuo-yamaguchi-cartoon-2009-v2.txt`, job `natsuo_yamaguchi-0468451c3d0f`, output `natsuo-yamaguchi-cartoon-2009-v2.png`. Suit, shirt and tie colours come as text only from a 2 January 2013 STB-1 photograph (CC BY-SA 3.0), which is not an input and is not committed.
- Later part: the 2019 CC0 reference is reused unchanged. New prompt `natsuo-yamaguchi-cartoon-2015-v1.txt` (only its window and age sentences differ from v1), job `natsuo_yamaguchi-af89e21deee1`, output `natsuo-yamaguchi-cartoon-2015-v1.png`.
- Superseded, not rendered: prompt `natsuo-yamaguchi-cartoon-2009-v1.txt` (file unchanged), job `natsuo_yamaguchi-93769cc0a66c`, output `natsuo-yamaguchi-cartoon-2009-v1.png`.
- Correction: the 2019 photograph shows him at 67, not about 64. 64 was the old window's midpoint age and the v1 prompt's target.
- No 2009-2012 photograph usable as a single-person likeness with a qualifying licence string was found on Commons. A sharper 2010-10-26 India PMO photograph is licensed 'GODL-India', which is not in FREE_LICENSES, so it is rejected unless Codex rules otherwise.
- The verifier's one finding was fixed: the early prompt's tie now carries the small light dot pattern visible in the January photograph, and the prompt is re-pinned (LF sha256 `bdd8f7a4b7410cde0312cbc839b2f1d711260403b71345680a41faf4ab278ee6`).
- At integration, `.gitattributes` needs `tools/avatars/person-prompts/natsuo-yamaguchi-cartoon-2009-v2.txt text eol=lf` and `tools/avatars/person-prompts/natsuo-yamaguchi-cartoon-2015-v1.txt text eol=lf`; this task may not edit that file.

The [identity review](identity-review-batch-01.json) (now sha256 `cb3ac83c278c34b5a146e9e41bfc41d039a97f675ffa321f9a2822eb7d30daaf`) and the [render request](render-request-batch-01.md) point at the two parts. All coverage remains prospective; no output is rendered, registered or approved.

## Evidence and privacy

Original Commons metadata and image bytes stay outside Git under `D:/spheres-scratch/jp-cast/refs/` and are pinned by sha256 in the identity review. The committed references are byte-identical copies. HTTP header captures, response bodies containing the client IP address and YouTube watch-page HTML are not copied, cited or hashed.

## Historical checks reported by Claude at preparation (1 October 2026)

- `python -B -X utf8 tools/avatars/person_art_pipeline.py self-test`: exit 0; 21 tests OK (after adding spheres-web/ui/portraits, leader-art and display-art to this sparse worktree, which the self-test reads).
- `python -B -X utf8 tools/avatars/person_art_pipeline.py validate`: exit 0; valid true, 0 errors, 530 warnings. person_portraits.json is unchanged, so no new portrait is registered.
- `python -X utf8 tools/planning/workboard.py --check`: PASS: 44 canonical markers, 59 bounded tasks.
- `python -B -X utf8 tools/avatars/leadership_production.py check`: exit 0.
- `python -B -X utf8 tools/avatars/campaign_census.py --check`: exit 0.
- `python -B -X utf8 tools/avatars/cartoon_review.py --check`: exit 0 (after adding docs/campaign-certification/S10 to the sparse worktree).
- `python -B -X utf8 -m unittest discover -s tools/avatars`: 827 tests, 2 failures, both outside this batch: the S23 boundary-matrix outputs are not in this sparse worktree, and the C01 gap ledger reports stale inputs on this checkout (Markdown inputs written as CRLF by core.autocrlf, and docs/planning/ai-task-queue.json changed at base 2fd186d6). No ledger input is a file of this batch.
- `git diff --cached --check (staged batch files)`: clean.

These checks show the existing manifest and tools still pass with the new files present. They are not a visual review, a registration or country sign-off (Japan's country sign-off stays with the user and Codex).

Integration note, 2 October 2026: all new prompts in this packet now have specific LF checkout rules in `.gitattributes`. The historical Windows CRLF values remain diagnostic; use the corrected exact LF hashes for generation.
