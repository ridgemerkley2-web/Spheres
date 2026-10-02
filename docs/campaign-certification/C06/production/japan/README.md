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
| Natsuo Yamaguchi | 2009-09-08 to 2024-09-28 | 29 August 2019 MOFA TICAD photograph, CC0. Usable; separate early/later variants are recommended for the fifteen-year span. |

The proposed twelve production jobs are **targeted coverage only**: generation, registration and output review must happen before any can close. Default-inventory residuals remain open. Appearance windows never establish continuous office tenure. The January 1996 Murayama bridge changes no historical claim. Yamaguchi's single 2019 reference does not prove his appearance throughout all four era jobs; the unchanged broad requested window remains an editorial proposal.

The six reference files remain byte-identical. Murayama/Ota year-containing filenames are retained identifiers, not capture-date evidence. Only their two prompts changed; LF hashes are re-pinned. The other four prompts are unchanged.

Takenori Kanzaki remains outside this batch, and Yoshida/Mataichi were not prepared. His earlier exclusion was the preparer's scope choice, not a user rule against licensed official film frames. Reviewing his separate source is future work.

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
