# France cast production (C06)

Task `CLAUDE-C06-FRANCE-01` (owner Claude), branch `claude/c06-fr-01`. State: **complete for the seven-portrait primary batch** (2 October 2026). Codex accepted registration `09cbb583` after 776 independent checks and regenerated the five derived export sets. See the [acceptance package](../../render-return-20261002/README.md) and original [registration proposal](registration-batch-01.md). The Aubry reserve remains unrendered; France country signoff, C06 and CP1 remain open. The preparation history below is retained. Claude generates, edits and labels no image.

## Batch 1 (FR-CAST-B01), prepared 1 October 2026

- [Identity review](identity-review-batch-01.json): accepted C01-23, C01-37 and C01-47 holder observations copied exactly, requested appearance windows and their basis, pipeline and leadership-production job IDs, verified likeness references, rejected candidates, verifier dispositions and an inline source ledger. Authority flags are all false.
- [Render request](render-request-batch-01.md): per job, the prompt file and sha256, the input order (style anchor first, STYLE ONLY; then the identity reference), 1024x1536 RGB, up to 3 attempts, and unchanged `exec-*.png` outputs to return.

| person_id | role | window (exclusive end) | reference | photograph date | licence | prompt |
|---|---|---|---|---|---|---|
| `francois_mitterrand` | primary | 1991-01-01 to 1995-05-17 | `francois-mitterrand-1994-reference-v1.jpg` | 1994 (Commons author statement, year only; uncertain) | CC BY-SA 4.0 | `francois-mitterrand-cartoon-1991-v1.txt` |
| `michel_rocard` | primary | 1990-01-01 to 1994-06-19 | `michel-rocard-1991-reference-v1.jpg` | 1991 | CC BY 2.0 | `michel-rocard-cartoon-1990-v1.txt` |
| `laurent_fabius` | primary | 1992-01-09 to 1993-04-03 | `laurent-fabius-1984-reference-v1.jpg` | 1984-08-28 | CC BY-SA 4.0 | `laurent-fabius-cartoon-1992-v1.txt` |
| `henri_emmanuelli` | primary | 1994-06-19 to 1995-10-14 | `henri-emmanuelli-2005-reference-v1.jpg` | 2005-05 | CC BY 3.0 | `henri-emmanuelli-cartoon-1994-v1.txt` |
| `alain_juppe` | primary | 1994-11-30 to 1997-06-01 | `alain-juppe-1996-reference-v1.jpg` | 1996-02-12 | CC BY 4.0 | `alain-juppe-cartoon-1994-v1.txt` |
| `lionel_jospin` | primary | 1995-10-14 to 1997-06-07 | `lionel-jospin-1998-reference-v1.jpg` | 1998-10-12 or 1998-10-13 (sources disagree) | CC BY 4.0 | `lionel-jospin-cartoon-1995-v1.txt` |
| `francois_hollande` | primary | 1997-06-30 to 2008-11-01 | `francois-hollande-2007-reference-v1.jpg` | 2007-05-29 | CC BY 2.5 | `francois-hollande-cartoon-1997-v1.txt` |
| `martine_aubry` | reserve | 2008-11-29 to 2012-09-01 | `martine-aubry-2010-reference-v1.jpg` | 2010-03-11 | CC BY 3.0 | `martine-aubry-cartoon-2008-v1.txt` |

Nobody was excluded. Every preparation was checked by an independent verification agent. Rocard and Hollande passed outright. The other findings were mechanical and are fixed in the records and prompts. They were: uncertain or conflicting photograph dates (Mitterrand, Jospin), a misidentified bystander and a suit colour (Fabius), an age-gap figure (Emmanuelli), and provenance, hands wording and window basis (Juppé). Two findings are recorded but not fixed here: the missing LF rule for the prompt files (needs `.gitattributes`, outside this task's files) and an optional EC Audiovisual search for a closer Fabius photograph. Martine Aubry stays prepared as the reserve.

Age and date limits declared in the prompts: the Fabius photograph is from 1984 (7 to 8.5 years before his window), Emmanuelli's from May 2005 (about 9.5 to 11 years after), Jospin's from October 1998 (about 16 months after), Hollande's from May 2007 (late in an eleven-year window), and Mitterrand's is dated 1994 on Commons but probably from about 1990 to 1993.

Original preparation decisions (resolved in the render return): the Mitterrand window and reference-year naming, the Juppé window start and 17 MB reference, a possible Hollande split at 2005-01-01, and the optional Fabius search. See the render request.

## Evidence and privacy

Original Commons metadata and image bytes stay outside Git under `D:/spheres-scratch/france-cast/refs/` and are pinned by sha256 in the identity review. The committed references are byte-identical copies. HTTP header captures that contain the client IP address or GeoIP cookies are not copied, cited or hashed.

## Checks at preparation

- `python -B -X utf8 tools/avatars/person_art_pipeline.py self-test`: exit 0; 21 tests OK.
- `python -B -X utf8 tools/avatars/person_art_pipeline.py validate`: exit 0; valid true, 0 errors, 530 warnings. person_portraits.json is unchanged, so no new portrait is registered.
- `python -X utf8 tools/planning/workboard.py --check`: PASS: 44 canonical markers, 59 bounded tasks.
- `git diff --cached --check (staged batch files)`: clean.

These checks show the existing manifest and tools still pass with the new files present. They are not a visual review, a registration or country sign-off (CS-France stays with the user and Codex).

## Codex return, 2 October 2026

Seven primary portraits are now generated and copied byte-identically to their planned PNG paths. The [render return](render-return-batch-01.json) pins the exact prompts, ordered inputs, original output paths, dimensions, hashes, source attribution and derivative licences. All seven were visually inspected by Codex root. Martine Aubry remains an unrendered reserve. Claude should perform the independent review and coordinate registration next; no runtime portrait or production job has been registered or closed. No human approval or France country sign-off is claimed.

The return resolves the appearance-window decisions without changing effective tenures. Juppé uses a pinned, uncropped, proportional transport-size reference after the original failed the image tool file reader. Fabius retains the 1984 source with declared ageing; no closer-photo search is claimed. Hollande is one mid/late-period interpretation, with an earlier variant still available as a registration decision. Specific LF checkout rules now protect all new prompts.

## Claude review and registration, 2 October 2026

All seven returned PNGs pass Claude's review ([claude-visual-review-batch-01.json](claude-visual-review-batch-01.json)). Seven additive records are proposed in `spheres-web/data/person_portraits.json`, with generation provenance in `tools/avatars/person-prompts/france-cast-batch-01.json`. The [registration receipt](registration-batch-01.md) gives the coverage effect, the checks and the derived outputs Codex regenerates at integration. No human approval or country sign-off is claimed.
