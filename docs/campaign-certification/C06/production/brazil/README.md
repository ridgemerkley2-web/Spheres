# Brazil cast production (C06)

Task `CLAUDE-C06-BRAZIL-01` (owner Claude), branch `claude/c06-br-01`. State: **bounded preparation reviewed; five render-ready jobs; the held Lupi 2015–2026 render is superseded by a verified era split into two jobs (6a and 6b), which Codex renders after reviewing the split**. No generated output is accepted. Claude prepares identities, dated likeness references and exact prompts; Codex renders with its built-in image tool; Claude then reviews and registers the returned outputs. Claude generates, edits and labels no image.

## Batch 1 (BR-CAST-B01), prepared 1 October 2026

- [Identity review](identity-review-batch-01.json): accepted C01-34 and C01-43 holder observations copied exactly, requested appearance windows and their basis, pipeline and leadership-production job IDs (window-scoped, default-inventory and residual), verified likeness references, rejected candidates, verifier dispositions, the excluded person and an inline source ledger. Authority flags are all false.
- [Render request](render-request-batch-01.md): per job, the prompt file and sha256, the input order (style anchor first, STYLE ONLY; then the identity reference), 1024x1536 RGB, up to 3 attempts, and unchanged `exec-*.png` outputs to return.

Five people hold accepted Brazil observations (C01-34 MDB and PDT, C01-43 Agir), so the batch takes all of them and has no reserve. Four are prepared in six windows; five renders may proceed as art proposals and Lupi's 2015–2026 render is held pending era correction. Tourinho remains unprepared.

| person_id | window | appearance window (exclusive end) | reference | photograph date | licence | prompt |
|---|---|---|---|---|---|---|
| `michel_temer` | 1 of 2 | 2001-09-09 to 2010-06-15 | `michel-temer-2009-reference-v1.jpg` | 2009-11-11 | CC BY 3.0 | `michel-temer-cartoon-2001-v1.txt` |
| `michel_temer` | 2 of 2 | 2010-06-15 to 2019-01-01 | `michel-temer-2017-reference-v1.jpg` | May 2017 (Commons gives 17 May, from file metadata only) | CC BY 2.0 | `michel-temer-cartoon-2010-v1.txt` |
| `baleia_rossi` | 1 of 1 | 2019-10-06 to 2026-09-08 | `baleia-rossi-2021-reference-v1.jpg` | 2021-12-08 | CC BY 2.0 | `baleia-rossi-cartoon-2019-v1.txt` |
| `leonel_brizola` | 1 of 1 | 1995-01-01 to 2004-06-21 | `leonel-brizola-1998-reference-v1.jpg` | 1998-09-19 (photographer's own statement; no embedded capture date) | CC BY-SA 4.0 | `leonel-brizola-cartoon-1995-v1.txt` |
| `carlos_lupi` | 1 of 3 | 2004-06-30 to 2015-01-01 | `carlos-lupi-2009-reference-v1.jpg` | 2009-11-11 | CC BY 3.0 | `carlos-lupi-cartoon-2004-v1.txt` |
| `carlos_lupi` | was 2 of 2 — **superseded, never rendered** | 2015-01-01 to 2026-09-08 | `carlos-lupi-2023-reference-v1.jpg` | 2023-10-24 | CC BY 2.0 | `carlos-lupi-cartoon-2015-v1.txt` |
| `carlos_lupi` | 2 of 3 — proposed era split, pending Codex review | 2015-01-01 to 2020-01-01 | `carlos-lupi-2015-reference-v1.jpg` | 2015-08-18 | CC BY 2.0 | `carlos-lupi-cartoon-2015-v2.txt` |
| `carlos_lupi` | 3 of 3 — proposed era split, pending Codex review | 2020-01-01 to 2026-09-08 | `carlos-lupi-2023-reference-v1.jpg` | 2023-10-24 | CC BY 2.0 | `carlos-lupi-cartoon-2020-v1.txt` |
| `daniel_sampaio_tourinho` | excluded | 2014-05-16 to 2026-09-08 | none | none qualifying | none | none |

Excluded: Daniel Tourinho. No dated photograph of him with a licence string from `FREE_LICENSES` was found under this preparation's search and licence filter on Commons, Wikidata or pt.wikipedia; this does not establish that no useful identity photograph exists. His TSE 2010 candidate photograph exists, but it is not on Commons, the TSE portal's licence is an unversioned 'Creative Commons Attribution', and it is a 161x225 greyscale thumbnail from 2010 or earlier. No reference or prompt for him is in the repository; his three jobs stay open. The TSE lead needs an independently inspected exact-person original; do not silently relabel its unversioned grant as CC BY 4.0. A generated authored-identity route based on verified visual research can be assessed separately from shipping or adapting that photo.

Every window had one preparation agent and one independent verification agent. The preparation verifiers passed Baleia Rossi and Lupi's 2015 window; the independent Codex review now holds the latter render because its prompt applies the 2023 appearance to the whole window. The other findings were fixed in the records and prompts:
- Temer 2001 window: the preparer's 11 March 2009 photograph is a strict right profile, so the reference was switched to the verifier's preferred near-frontal J. Batista / Câmara dos Deputados photograph of 11 November 2009 (CC BY 3.0), the same bytes as Lupi's 2009 reference. The prompt was rewritten and re-pinned, and after-window candidates were added to the ledger.
- Temer 2010 window: the date evidence is restated (an XMP Photoshop creation time, month corroborated by Gazeta do Povo).
- Brizola: the ears wording in the prompt (re-pinned) and the job labels.
- Lupi 2004 window: the accepted observation, window basis and registry term are recorded, the face width corrected and the jobs labelled.

The six proposed windows could cover 14 of the 15 leadership-production cartoon jobs for these five people after generation, registration and review, including resolution of the held Lupi era window; none is closed by this preparation. Tourinho's `cartoon:daniel_sampaio_tourinho:2026-07-05:2026-09-08:v1` stays open. Temer's 2010-2019 window has no leadership-production job.

Age and date limits declared in the prompts: every photograph falls inside its window. Temer's 2001 portrait is drawn about four years younger than his 2009 photograph (age 69; window 60 to 69). His 2010 portrait follows a May 2017 photograph across ages 69 to 78. Lupi's 2015 portrait follows a 2023 photograph across ages 57 to 69 and does not show his darker hair of 2015; that job is superseded by the era correction below. Brizola's hat hides his crown, which is an artistic completion.

Codex decisions requested: the Temer 2010-2019 window (and an optional early-window split), the Brizola registry birth date conflict, the photographer's embedded IPTC contact fields and 7.66 MB size of the Brizola reference, the Lupi 2004 window start, a required Lupi era correction before rendering (a narrower later-era window or a reviewed earlier variant), the TSE licence ruling and optional municipal-year search for Tourinho, and an LF `.gitattributes` rule for the six prompts. See the render request.

## Independent disposition, 2 October 2026

Codex independently reviewed exact source tip `383086651685ff2f8ba6aaff3a575cc4ae67df70`: all 17 holder dictionaries match accepted research, all six reference copies match the retained originals, all five distinct images were visually inspected, and all six prompt hashes match their committed LF bytes. The intentional Lupi/Temer group-photo reuse has distinct subject cues. One receipt pin is corrected from its CRLF rendition to the committed LF hash. No holder, prompt, registry, portrait or runtime record changes.

Five jobs are render-ready as art proposals. **Do not render `carlos-lupi-cartoon-2015-v1.txt` until its era application is corrected and the revised prompt/window is reviewed.** The current prompt is preserved as the held proposal. Brizola's birthday conflict remains disclosed; its approximate-age wording is usable. His resulting adaptation must retain Sérgio Neglia attribution, change disclosure and CC BY-SA 4.0 derivative requirements, in addition to its generated-method label. No output review, job completion or country signoff is claimed.

## Lupi era correction, 2 October 2026 (verified and fixed; pending Codex review)

[`lupi-era-correction-batch-01.json`](lupi-era-correction-batch-01.json) answers the hold on Lupi's 2015–2026 render by splitting it at 2020-01-01, an existing leadership-production job boundary chosen as an art boundary (not a sourced office or physical-change date). One preparation agent and one independent verification agent worked on it; Claude assembled it and fixed the verifier's findings.

- Window 2 of 3, 2015-01-01 to 2020-01-01 (job 6a): new reference `carlos-lupi-2015-reference-v1.jpg` (Jane de Araújo/Agência Senado, 18 August 2015, Commons `File:Presidência do Senado (20659268666).jpg`, CC BY 2.0; EXIF capture 2015-08-18 12:22, not corroborated by a news report; byte-identical to the retained original, sha256 `fbaff7ed9f7313fef48c081d62689e6ce5490a08512b9b833ea4c65c3b3b23a6`) and new prompt `carlos-lupi-cartoon-2015-v2.txt` (LF sha256 `777d1e145a0ffe2ed9a2e02d4e83bf948d27f445083a160ecf6065c5704cc530`). Planned coverage: `carlos_lupi-400fe963352c` and `cartoon:carlos_lupi:2015-01-01:2020-01-01:v1`. No accepted C01 observation falls in this window; it rests on the registry term and the leadership-production job.
- Window 3 of 3, 2020-01-01 to 2026-09-08 (job 6b): the reviewed 2023 reference, unchanged, and new prompt `carlos-lupi-cartoon-2020-v1.txt` (LF sha256 `cc0a4c92eeea27ed6b37769a41ea8c3de6ff9edd75e10f47cbb1adea1070a218`). Planned coverage: `carlos_lupi-84f4126df75f`, `cartoon:carlos_lupi:2020-01-01:2025-01-01:v1` and `cartoon:carlos_lupi:2025-01-01:2026-09-08:v1`. The three accepted observations (2021-12-21, 2025-05-21, 2026-09-04) fall here.

Verifier findings and fixes:
- `File:Lupi.png` (CC BY-SA 4.0, 'own work', uploader-dated 1 May 2018) had been dismissed without inspection. It shows Lupi grayer than in August 2015 (salt-and-pepper hair, gray-brown beard). It is now recorded with its inspection result as a comparison photograph, not a reference: its date and authorship rest only on the uploader's statement (no capture metadata, a square PNG carrying an IPTC transmission token, uploaded almost four months after the stated date). The records now say that no verified dated photograph from 2016 to 2022 was found and that this uploader-dated 2018 photograph exists.
- The 2015 prompt no longer asks for the same dark look through 2019. It follows the 2015 photograph (dark charcoal hair with gray at the temples and sides; dark brown-black beard with gray at the chin and sideburns), allows a little more gray, and discloses the likely graying of 2018–2019. 'White-haired' became 'gray hair and gray-and-white beard'.
- The 2020 prompt no longer mandates the October 2023 gray for the whole window. It asks for salt-and-pepper to gray hair and a salt-and-pepper beard, never dark, and says October 2023 may be near the grayest point of the window.
- The identity review is updated additively: the held entry (`/batches/0/people/5`) is marked superseded in place; the two new windows (`/batches/0/people/6` and `/7`), the new reference and its source-ledger entry are added. All 17 holder dictionaries are unchanged; the three in window 3 of 3 are exact copies.

`carlos-lupi-cartoon-2015-v1.txt` is preserved unchanged and superseded; it must not be rendered. Coverage stays prospective: no job is closed until the output is generated, reviewed and registered. Open for Codex: the split and window 2 of 3; a ruling on `File:Lupi.png` (accept the softened wording, or verify it and split window 2 again, for example at 2018-01-01); the reference's embedded Senate-archive IPTC contact fields and camera serial number, kept for byte identity and not quoted here; and LF rules in `.gitattributes` for the two new prompts (outside this task). The search covered Commons only, not Flickr-only files.

## Evidence and privacy

Original Commons metadata and image bytes stay outside Git under `D:/spheres-scratch/br-cast/refs/` and are pinned by sha256 in the identity review. The committed references are byte-identical copies. No preparer kept an HTTP header capture; a Flickr page body that embeds a viewer-geolocation country code was discarded, and only its extracted date and licence fields are kept. The Brizola reference carries the photographer's own IPTC contact fields, as Commons publishes them; they are not quoted anywhere in these records.

## Checks at preparation

- `python -B -X utf8 tools/avatars/person_art_pipeline.py self-test`: exit 0; 21 tests OK. This worktree is a sparse checkout; the self-test reads source art in `spheres-web/ui/portraits` and `spheres-web/ui/leader-art`, so both directories were added to it (before that, 8 failures and 1 error from the missing files). No file there was changed.
- `python -B -X utf8 tools/avatars/person_art_pipeline.py validate`: exit 0; valid true, 0 errors, 530 warnings. person_portraits.json is unchanged, so no new portrait is registered.
- `python -X utf8 tools/planning/workboard.py --check`: PASS: 44 canonical markers, 59 bounded tasks.
- `git diff --cached --check` (staged batch files): clean.

These checks show the existing manifest and tools still pass with the new files present. They are not a visual review, a registration or country sign-off (CS-Brazil stays with the user and Codex).

Integration note, 2 October 2026: all new prompts in this packet now have specific LF checkout rules in `.gitattributes`. The two era-fix prompts added afterwards (`carlos-lupi-cartoon-2015-v2.txt`, `carlos-lupi-cartoon-2020-v1.txt`) have no rule yet; see the era correction. The historical Windows CRLF values remain diagnostic; use the corrected exact LF hashes for generation.
