# Brazil cast production (C06)

Task `CLAUDE-C06-BRAZIL-01` (owner Claude), branch `claude/c06-br-01`. State: **awaiting Codex render**; not ready for review. Claude prepares identities, dated likeness references and exact prompts; Codex renders with its built-in image tool; Claude then reviews and registers the returned outputs. Claude generates, edits and labels no image.

## Batch 1 (BR-CAST-B01), prepared 1 October 2026

- [Identity review](identity-review-batch-01.json): accepted C01-34 and C01-43 holder observations copied exactly, requested appearance windows and their basis, pipeline and leadership-production job IDs (window-scoped, default-inventory and residual), verified likeness references, rejected candidates, verifier dispositions, the excluded person and an inline source ledger. Authority flags are all false.
- [Render request](render-request-batch-01.md): per job, the prompt file and sha256, the input order (style anchor first, STYLE ONLY; then the identity reference), 1024x1536 RGB, up to 3 attempts, and unchanged `exec-*.png` outputs to return.

Five people hold accepted Brazil observations (C01-34 MDB and PDT, C01-43 Agir), so the batch takes all of them and has no reserve. Four are prepared in six windows; one is excluded.

| person_id | window | appearance window (exclusive end) | reference | photograph date | licence | prompt |
|---|---|---|---|---|---|---|
| `michel_temer` | 1 of 2 | 2001-09-09 to 2010-06-15 | `michel-temer-2009-reference-v1.jpg` | 2009-11-11 | CC BY 3.0 | `michel-temer-cartoon-2001-v1.txt` |
| `michel_temer` | 2 of 2 | 2010-06-15 to 2019-01-01 | `michel-temer-2017-reference-v1.jpg` | May 2017 (Commons gives 17 May, from file metadata only) | CC BY 2.0 | `michel-temer-cartoon-2010-v1.txt` |
| `baleia_rossi` | 1 of 1 | 2019-10-06 to 2026-09-08 | `baleia-rossi-2021-reference-v1.jpg` | 2021-12-08 | CC BY 2.0 | `baleia-rossi-cartoon-2019-v1.txt` |
| `leonel_brizola` | 1 of 1 | 1995-01-01 to 2004-06-21 | `leonel-brizola-1998-reference-v1.jpg` | 1998-09-19 (photographer's own statement; no embedded capture date) | CC BY-SA 4.0 | `leonel-brizola-cartoon-1995-v1.txt` |
| `carlos_lupi` | 1 of 2 | 2004-06-30 to 2015-01-01 | `carlos-lupi-2009-reference-v1.jpg` | 2009-11-11 | CC BY 3.0 | `carlos-lupi-cartoon-2004-v1.txt` |
| `carlos_lupi` | 2 of 2 | 2015-01-01 to 2026-09-08 | `carlos-lupi-2023-reference-v1.jpg` | 2023-10-24 | CC BY 2.0 | `carlos-lupi-cartoon-2015-v1.txt` |
| `daniel_sampaio_tourinho` | excluded | 2014-05-16 to 2026-09-08 | none | none qualifying | none | none |

Excluded: Daniel Tourinho. No dated photograph of him with a licence string from `FREE_LICENSES` exists on Commons, Wikidata or pt.wikipedia. His TSE 2010 candidate photograph exists, but it is not on Commons, the TSE portal's licence is an unversioned 'Creative Commons Attribution', and it is a 161x225 greyscale thumbnail from 2010 or earlier. No reference or prompt for him is in the repository; his three jobs stay open, and Codex is asked to rule on the TSE licence.

Every window had one preparation agent and one independent verification agent. Baleia Rossi and Lupi's 2015 window passed outright. The other findings were fixed in the records and prompts:
- Temer 2001 window: the preparer's 11 March 2009 photograph is a strict right profile, so the reference was switched to the verifier's preferred near-frontal J. Batista / Câmara dos Deputados photograph of 11 November 2009 (CC BY 3.0), the same bytes as Lupi's 2009 reference. The prompt was rewritten and re-pinned, and after-window candidates were added to the ledger.
- Temer 2010 window: the date evidence is restated (an XMP Photoshop creation time, month corroborated by Gazeta do Povo).
- Brizola: the ears wording in the prompt (re-pinned) and the job labels.
- Lupi 2004 window: the accepted observation, window basis and registry term are recorded, the face width corrected and the jobs labelled.

Together the six windows close 14 of the 15 leadership-production cartoon jobs for these five people; Tourinho's `cartoon:daniel_sampaio_tourinho:2026-07-05:2026-09-08:v1` stays open. Temer's 2010-2019 window has no leadership-production job.

Age and date limits declared in the prompts: every photograph falls inside its window. Temer's 2001 portrait is drawn about four years younger than his 2009 photograph (age 69; window 60 to 69). His 2010 portrait follows a May 2017 photograph across ages 69 to 78. Lupi's 2015 portrait follows a 2023 photograph across ages 57 to 69 and does not show his darker hair of 2015. Brizola's hat hides his crown, which is an artistic completion.

Codex decisions requested: the Temer 2010-2019 window (and an optional early-window split), the Brizola registry birth date conflict, the photographer's embedded IPTC contact fields and 7.66 MB size of the Brizola reference, the Lupi 2004 window start, an optional Lupi split at 2020-01-01, the TSE licence ruling and optional municipal-year search for Tourinho, and an LF `.gitattributes` rule for the six prompts. See the render request.

## Evidence and privacy

Original Commons metadata and image bytes stay outside Git under `D:/spheres-scratch/br-cast/refs/` and are pinned by sha256 in the identity review. The committed references are byte-identical copies. No preparer kept an HTTP header capture; a Flickr page body that embeds a viewer-geolocation country code was discarded, and only its extracted date and licence fields are kept. The Brizola reference carries the photographer's own IPTC contact fields, as Commons publishes them; they are not quoted anywhere in these records.

## Checks at preparation

- `python -B -X utf8 tools/avatars/person_art_pipeline.py self-test`: exit 0; 21 tests OK. This worktree is a sparse checkout; the self-test reads source art in `spheres-web/ui/portraits` and `spheres-web/ui/leader-art`, so both directories were added to it (before that, 8 failures and 1 error from the missing files). No file there was changed.
- `python -B -X utf8 tools/avatars/person_art_pipeline.py validate`: exit 0; valid true, 0 errors, 530 warnings. person_portraits.json is unchanged, so no new portrait is registered.
- `python -X utf8 tools/planning/workboard.py --check`: PASS: 44 canonical markers, 59 bounded tasks.
- `git diff --cached --check` (staged batch files): clean.

These checks show the existing manifest and tools still pass with the new files present. They are not a visual review, a registration or country sign-off (CS-Brazil stays with the user and Codex).
