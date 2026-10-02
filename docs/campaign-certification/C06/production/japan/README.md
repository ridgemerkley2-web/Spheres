# Japan cast production (C06)

Task `CLAUDE-C06-JAPAN-01` (owner Claude), branch `claude/c06-jp-01`. State: **awaiting Codex render**; not ready for review. Claude prepares identities, dated likeness references and exact prompts; Codex renders with its built-in image tool; Claude then reviews and registers the returned outputs. Claude generates, edits and labels no image.

## Batch 1 (JP-CAST-B01), prepared 1 October 2026

- [Identity review](identity-review-batch-01.json): accepted C01-29 and C01-31 holder observations copied exactly, requested appearance windows and their basis, pipeline and leadership-production job IDs, verified likeness references, rejected candidates, verifier dispositions, the excluded person and an inline source ledger. Authority flags are all false.
- [Render request](render-request-batch-01.md): per job, the prompt file and sha256, the input order (style anchor first, STYLE ONLY; then the identity reference), 1024x1536 RGB, up to 3 attempts, and unchanged `exec-*.png` outputs to return.

| person_id | role | window (exclusive end) | reference | image date | licence | prompt |
|---|---|---|---|---|---|---|
| `tomiichi_murayama` | primary | 1993-09-30 to 1996-09-01 | `tomiichi-murayama-1994-reference-v1.jpg` | not stated; no later than 1995 (MOFA APEC 1995 page) | CC BY 4.0 | `tomiichi-murayama-cartoon-1993-v1.txt` |
| `makoto_tanabe` | primary | 1991-07-31 to 1993-01-19 | `makoto-tanabe-1992-reference-v1.jpg` | 1992-04-07 | CC BY 4.0 | `makoto-tanabe-cartoon-1991-v1.txt` |
| `sadao_yamahana` | primary | 1993-01-19 to 1993-09-01 | `sadao-yamahana-1993-reference-v1.jpg` | 1993-08-09 (film frame) | CC BY 3.0 | `sadao-yamahana-cartoon-1993-v1.txt` |
| `takako_doi` | primary | 1996-09-30 to 2003-11-01 | `takako-doi-2005-reference-v1.jpg` | 2005-07-02 | CC BY 2.0 | `takako-doi-cartoon-1996-v1.txt` |
| `akihiro_ota` | primary | 2006-09-30 to 2009-09-08 | `akihiro-ota-2012-reference-v1.jpg` | no later than 2012-12-27 (Commons, editing timestamp) | CC BY 4.0 | `akihiro-ota-cartoon-2006-v1.txt` |
| `natsuo_yamaguchi` | primary | 2009-09-08 to 2024-09-28 | `natsuo-yamaguchi-2019-reference-v1.jpg` | 2019-08-29 | CC0 | `natsuo-yamaguchi-cartoon-2009-v1.txt` |
| `takenori_kanzaki` | excluded | 1998-11-07 to 2006-09-30 | none (video frame; kept in scratch) | 2006-09-26 | CC BY 4.0 | none |

Every preparation was checked by an independent verification agent. Tanabe and Doi passed outright. The other findings were fixed in the records and prompts:

- **Murayama**: the Kantei portrait had no date. The assembler showed that MOFA used the same sitting on its APEC 1995 Osaka profile page (copyright 1995; raw Wayback capture of 1999; the archived image matches Commons' black-and-white copy pixel for pixel). So the photograph was taken no later than 1995. Also fixed: the provenance notes and the missing rights, limitation and date fields. Codex is asked to accept a reference dated this way (the Kaifu precedent) and to rule on the year in its name.
- **Yamahana**: suit colour corrected to dark charcoal-gray. His own jacket and lapel badge, misread as another man's shoulder, are corrected. The EXIF wording, a draft hash fragment and a missing byte count are fixed. The reference is a frame from the Defense Agency's 1993 record film, put to Codex on the koshiro_ishida precedent (same film and day).
- **Ota**: the preparer's bytes were the current Commons revision, uploaded from a commercial news site (response.jp). They are replaced by the first revision, whose source is MLIT's own flyer (339x507, sha1 verified). Credit, rights and date notes are rewritten from the file history, and the prompt now describes the tighter crop.
- **Yamaguchi**: the prompt now picks him out as the central speaker and tells the model to ignore everyone else; his name card stands in front of the delegate next to him. Hands wording and record wording are corrected.

Excluded: **Takenori Kanzaki**. His only usable image is a frame from the Government Internet TV programme of 26 September 2006. The batch rule rejects TV stills, and no registered precedent covers this source. His reference copy and prompt were removed from the repository before staging. Both are pinned in scratch and can be restored unchanged if Codex rules that such frames qualify. The handoff reserves (Yoshida, Mataichi) were not prepared.

Age and date limits declared in the prompts:

- Doi's photograph is from July 2005, about 20 months after her window.
- Ota's is dated no later than December 2012, about 3.3 years after his window.
- Yamaguchi's is from August 2019, inside a fifteen-year window, about 2.4 years after its midpoint.
- Murayama's is undated but no later than 1995.
- Tanabe's (April 1992) and Yamahana's (August 1993) fall inside their windows.

Codex decisions requested:

- Murayama: accept the reference dated by terminus ante quem, the reference-year naming, and the January 1996 bridge.
- Yamahana: the film-frame ruling.
- Ota: the licence route and the out-of-window reference.
- Yamaguchi: the 27 MB reference size and a possible window split.
- Kanzaki: the video-frame question.
- All: the LF rule for the prompt files.

See the render request.

## Evidence and privacy

Original Commons metadata and image bytes stay outside Git under `D:/spheres-scratch/jp-cast/refs/` and are pinned by sha256 in the identity review. The committed references are byte-identical copies. HTTP header captures, response bodies containing the client IP address and YouTube watch-page HTML are not copied, cited or hashed.

## Checks at preparation

- `python -B -X utf8 tools/avatars/person_art_pipeline.py self-test`: exit 0; 21 tests OK (after adding spheres-web/ui/portraits, leader-art and display-art to this sparse worktree, which the self-test reads).
- `python -B -X utf8 tools/avatars/person_art_pipeline.py validate`: exit 0; valid true, 0 errors, 530 warnings. person_portraits.json is unchanged, so no new portrait is registered.
- `python -X utf8 tools/planning/workboard.py --check`: PASS: 44 canonical markers, 59 bounded tasks.
- `python -B -X utf8 tools/avatars/leadership_production.py check`: exit 0.
- `python -B -X utf8 tools/avatars/campaign_census.py --check`: exit 0.
- `python -B -X utf8 tools/avatars/cartoon_review.py --check`: exit 0 (after adding docs/campaign-certification/S10 to the sparse worktree).
- `python -B -X utf8 -m unittest discover -s tools/avatars`: 827 tests, 2 failures, both outside this batch: the S23 boundary-matrix outputs are not in this sparse worktree, and the C01 gap ledger reports stale inputs on this checkout (Markdown inputs written as CRLF by core.autocrlf, and docs/planning/ai-task-queue.json changed at base 2fd186d6). No ledger input is a file of this batch.
- `git diff --cached --check (staged batch files)`: clean.

These checks show the existing manifest and tools still pass with the new files present. They are not a visual review, a registration or country sign-off (Japan's country sign-off stays with the user and Codex).
