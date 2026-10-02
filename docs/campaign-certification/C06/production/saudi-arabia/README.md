# Saudi Arabia cast production (C06)

Task `CLAUDE-C06-SAUDIARABIA-01` (owner Claude), branch `claude/c06-sa-01`. State: **awaiting Codex render**; not ready for review. Claude prepares identities, dated likeness references and exact prompts; Codex renders with its built-in image tool; Claude then reviews and registers the returned outputs. Claude generates, edits and labels no image.

## Batch 1 (SA-CAST-B01), prepared 2 October 2026

- [Identity review](identity-review-batch-01.json): accepted C01-45 and C01-50 holder observations copied exactly, requested appearance windows and their basis, pipeline job IDs (window-scoped, default inventory, alternatives and residuals), verified likeness references, rejected candidates, verifier dispositions and an inline source ledger. Authority flags are all false.
- [Render request](render-request-batch-01.md): per job, the prompt file and sha256, the input order (style anchor first, STYLE ONLY; then the identity reference), 1024x1536 RGB, up to 3 attempts, and unchanged `exec-*.png` outputs to return.

The batch has two people, not six to eight: `party_leaders.json` holds only two Saudi person IDs. Each has two appearance windows, so there are four portrait jobs. No reserve exists.

| person_id | window (exclusive end) | reference | photograph date | licence | prompt |
|---|---|---|---|---|---|
| `fahd_bin_abdulaziz_al_saud` | 1991-01-01 to 1996-01-01 | `fahd-bin-abdulaziz-1992-reference-v1.jpg` | 1992-12-31 | Public domain (PD-USGov; White House photograph P38849-17) | `fahd-bin-abdulaziz-cartoon-1991-v1.txt` |
| `fahd_bin_abdulaziz_al_saud` | 1996-01-01 to 2005-08-01 | `fahd-bin-abdulaziz-1998-reference-v1.jpg` | 1998-10-13 | Public domain (PD-USGov-Military; DoD 981013-D-2987S-196) | `fahd-bin-abdulaziz-cartoon-1996-v1.txt` |
| `abdullah_bin_abdulaziz_al_saud` | 1996-01-01 to 2005-08-01 | `abdullah-bin-abdulaziz-1998-reference-v1.jpg` | 1998-11-03 | Public domain (PD-USGov-Military; DoD 981103-D-9880W-172) | `abdullah-bin-abdulaziz-cartoon-1996-v1.txt` |
| `abdullah_bin_abdulaziz_al_saud` | 2005-08-01 to 2015-01-23 | `abdullah-bin-abdulaziz-2007-reference-v1.jpg` | 2007-01-17 | Public domain (PD-USGov-Military; DoD 070117-D-7203T-016) | `abdullah-bin-abdulaziz-cartoon-2005-v1.txt` |

Every photograph is exactly dated and lies inside its window. Nobody was excluded. Every window was checked by an independent verification agent. The Abdullah 1996-2005 preparation passed with two record nits. The other findings were mechanical and are fixed in the records and prompts:

- Fahd 1991: the Bush Library photograph number P38849-17 and its archived file, byte-identical to the reference, now cite the provenance. The creator field is clean, the revision count of the wikitext record is corrected, and the job IDs are recorded in the France shape.
- Fahd 1996: the prompt now says "medium" rather than "light" olive-tan complexion, and the reference record has the rights fields France carries.
- Abdullah 2005: the record now describes the file's embedded IPTC and XMP date, instead of saying the file has no embedded date.

The assembler also changed "light" to "medium" in the Abdullah 2005 prompt, to match the photograph and the 1996 prompt. That was the assembler's own finding, not a verifier's. One finding is recorded but not fixed: the missing LF rule for the prompt files, which needs `.gitattributes` and is outside this task's files.

Age limits declared in the prompts: birth years are general biographical knowledge, not C01 research. Fahd is about 70 in the 1992 photograph and about 77 in the 1998 one. Abdullah is about 74 in 1998 and about 82 in 2007, and the 2007 appearance is kept to the end of his window in January 2015 (about 90). Standing poses, robe hems and shoes are declared artistic extensions; Fahd's walking cane is omitted.

Codex decisions requested: the Fahd 1991-1996 window (no accepted observation inside it), the Fahd split or a single window, the Abdullah start (1996 or 1990), the unaged Abdullah 2005-2015 appearance, and registry identities for batch 2 (Salman and Mohammed bin Salman first). See the render request.

## Evidence and privacy

Original Commons metadata, Wayback captures and image bytes stay outside Git under `D:/spheres-scratch/sa-cast/refs/` and are pinned by sha256 in the identity review. The committed references are byte-identical copies. No HTTP response header captures were saved; HTTP 429 response bodies were deleted.

## Checks at preparation

- `python -B -X utf8 tools/avatars/person_art_pipeline.py self-test`: exit 0; 21 tests OK. It first failed with 8 failures and 1 error because the sparse worktree lacked `spheres-web/ui/leader-art` and `spheres-web/ui/portraits`. Both were added with `git sparse-checkout add`, which changes no tracked file.
- `python -B -X utf8 tools/avatars/person_art_pipeline.py validate`: exit 0; valid true, 0 errors, 530 warnings. person_portraits.json is unchanged, so no new portrait is registered.
- `python -X utf8 tools/planning/workboard.py --check`: PASS: 44 canonical markers, 59 bounded tasks.
- `git diff --check` and `git diff --cached --check` (staged batch files): clean.

These checks show the existing manifest and tools still pass with the new files present. They are not a visual review, a registration or country sign-off (CS-SaudiArabia stays with the user and Codex).
