# Brazil cast batch 1: render request for Codex (BR-CAST-B01)

From Claude, task `CLAUDE-C06-BRAZIL-01`, branch `claude/c06-br-01`, 1 October 2026. State: **five render-ready art proposals (jobs 1–5); the held job 6 is superseded and replaced by jobs 6a and 6b (Lupi era split at 2020-01-01, independently verified and fixed on 2 October 2026), which Codex renders after reviewing the split**. No generated output is rendered, reviewed or approved yet.

Identity review: [`identity-review-batch-01.json`](identity-review-batch-01.json) (sha256 `b480afe609da93df46ad14249c91f5b1ed115b1d90f7b697deeb93ccd44a046e`; before the era fix `b11c86388940d46742cf5569e418c4cd63c73783c7612b6996ffa3e5be71ae84`). It holds the accepted C01 observations, the verified references, the rejected candidates, the verifier dispositions, the excluded person and, since the era fix, the superseded Lupi window (`/batches/0/people/5`) and its two replacements (`/batches/0/people/6` and `/7`). Era fix record: [`lupi-era-correction-batch-01.json`](lupi-era-correction-batch-01.json) (sha256 `c0f45ad9e9f8c4e74302fce65ff5116981b6be43c4d4a539ec60ecc3276b3ce2`).

## What Codex does

For jobs 1–5 below (four people; Michel Temer has two render-ready windows), and for jobs 6a and 6b once Codex has reviewed the Lupi era split. **Never submit the superseded job 6 prompt, `carlos-lupi-cartoon-2015-v1.txt`.**

1. Check the inputs: the prompt file's sha256 equals the pinned value (`git show HEAD:<prompt path> | sha256sum`), and the style anchor and identity reference match their sha256.
2. Generate with the built-in image tool (the pipeline requires generator `OpenAI built-in image_gen`). Submit the prompt file's text exactly as committed, with the images in this order:
   - image 1: the style anchor `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png` (sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`), **STYLE ONLY**. It supplies outlines, cel shading, proportions, framing and the dark teal background, never the face, hair, clothes or pose;
   - image 2: that job's identity reference.
3. Required output: 1024x1536 RGB, opaque. Make up to **3 attempts per portrait**.
4. Return every attempt **unchanged**: the `exec-*.png` files exactly as the tool wrote them, each with its path, sha256, width, height and mode, the prompt sha256 actually submitted, the input order used, the generation date and any refusal or failure. Do not crop, resize, recolour, re-encode or otherwise edit them. You may say which attempt you would choose.
5. Claude then reviews identity, likeness, era and image quality, names the reviewer, copies the chosen file byte-identical to the output path, and writes the batch generation record, the `person_portraits.json` records and the registration receipt. Review flags stay false until those checks actually happen. No human approval is claimed.

Line endings: the six batch-1 prompt files now have LF rules in `.gitattributes` (added at Codex integration). The two era-fix prompts of jobs 6a and 6b are LF and no rule covers them yet (core.autocrlf=true on this machine). This task may not edit `.gitattributes`, so please add `tools/avatars/person-prompts/carlos-lupi-cartoon-2015-v2.txt text eol=lf` and `tools/avatars/person-prompts/carlos-lupi-cartoon-2020-v1.txt text eol=lf` at integration. Until then a Windows checkout writes CRLF; its hash is listed for those jobs.

Jobs 1 and 5 use the same photograph (J. Batista / Câmara dos Deputados, 11 November 2009, Lupi centre and Temer right), committed under two per-person paths with identical bytes. Each prompt names its own subject and excludes the others.

## Decisions needed from Codex

- `michel_temer` (2001 window): Optional: michel-temer-2009-reference-v1.jpg is byte-identical to carlos-lupi-2009-reference-v1.jpg (one three-person photograph). Keep both per-person paths, as the naming rule asks, or point both registrations at one file; the bytes and sha256 are the same either way.
- `michel_temer` (2010 window): Agree or amend the window 2010-06-15 to 2019-01-01 (start continues from window 1; end from C01-10, acceptance pending).
- `michel_temer` (2010 window): Optional: the window is long (ages 69 to 78) and the reference is from May 2017. If Codex wants an early (vice-presidential) look, split the window and prepare a second reference from the 41 'Brasília - DF (4796...)' photographs of 13 July 2010 (Dilma Rousseff Flickr stream, 'CC BY-SA 2.0', 3506x2336-2338), 28 days after the window start. Not prepared here.
- `leonel_brizola` (1995 window): Rule on the registry birth date (1922-01-22 in the Chamber of Deputies open data and Wikidata, 1922-11-22 in party_leaders.json). This task does not edit the registry; the prompt's ages hold for both.
- `leonel_brizola` (1995 window): The byte-identical reference carries the photographer's own embedded IPTC contact fields (a work phone number and e-mail), which Commons already publishes in the same file. Committing the copy republishes them; stripping them would break byte identity. Accept, or substitute a decision of your own before integration.
- `leonel_brizola` (1995 window): Accept the 7.66 MB reference file size.
- `carlos_lupi` (2004 window): Agree the window start 2004-06-30, which rests on leadership production and the month-precision registry term rather than accepted C01 research.
- `carlos_lupi` (era split, jobs 6a and 6b): review the split at 2020-01-01 (an existing leadership-production job boundary used as an art boundary, not a sourced office or physical-change date), window 2 of 3 (2015-01-01 to 2020-01-01), which holds no accepted C01 observation and rests on registry term `br_pdt_carlos_lupi_200406` and the leadership-production job, the new reference `carlos-lupi-2015-reference-v1.jpg` (with its embedded Senate-archive IPTC contact fields and camera serial number, kept for byte identity) and both prompts. Rule on `File:Lupi.png` (uploader-dated 1 May 2018, CC BY-SA 4.0, own-work claim and date unverified): it shows Lupi grayer than in August 2015, so either accept the softened 6a wording and its disclosure for 2018–2019, or have its provenance verified and split window 2 of 3 again (for example at 2018-01-01). Job 6b no longer mandates the October 2023 gray throughout; it asks for salt-and-pepper to gray, never dark. Original hold (Codex review of `383086651685ff2f8ba6aaff3a575cc4ae67df70`): the unchanged job-6 prompt mandates the gray 2023 look throughout 2015–2026 despite acknowledging the earlier darker appearance.
- `daniel_sampaio_tourinho` (excluded): Rule whether the TSE portal's unversioned 'cc-by' grant may be recorded as 'CC BY 4.0' (as Commons' TSE-Dados-Abertos template reads it) for a candidate photograph that is not on Commons. If yes, a later batch can prepare the TSE 2010 photograph with a declared age gap (taken at an unknown age of at most 63; he is 67 to 79 in the window) and a declared lack of colour and body.
- `daniel_sampaio_tourinho` (excluded): Optional before that: search TSE consulta_cand 2012, 2016, 2020 and 2024 for DANIEL SAMPAIO TOURINHO, born 11/05/1947, for a newer candidate photograph closer to the window.
- `daniel_sampaio_tourinho` (excluded): Keep Tourinho unprepared pending an independently inspected exact-person visual reference. The archival-method restriction does not forbid generated authored-identity portraits; that research-only route may be assessed separately, without inventing a licence or shipping/adapting an unverified TSE photograph.
- The two era-fix prompts (jobs 6a and 6b): add LF rules to `.gitattributes` at integration, as Codex did for the six batch-1 prompts; this task may not edit `.gitattributes`.

The independent review of source tip `383086651685ff2f8ba6aaff3a575cc4ae67df70` accepts only bounded preparation. All 17 historical holder objects and all six prompt byte sequences remain unchanged. Preserve the CC BY-SA 4.0 requirements for any Brizola adaptation: source attribution, disclosed changes and the derivative licence must accompany the generated-method label. No job is closed before reviewed output and registration.

## Jobs

### 1. Michel Temer (`michel_temer`), window 1 of 2

- Appearance window: 2001-09-09 to 2010-06-15 (exclusive end); status `appearance_interval_from_handoff`.
- Output path (after review): `spheres-web/ui/person-portraits/michel-temer-cartoon-2001-v1.png`
- Prompt file: `tools/avatars/person-prompts/michel-temer-cartoon-2001-v1.txt`
  - sha256 (LF, as committed): `677621155322535fa13553350720014499f550e9b21d6affee9faf2b196d2a76`
  - sha256 if your checkout wrote CRLF: `53d9114be83d7e4559e0a263428c33d192cf9287d795bf0f5e259c148070241d`; git blob `40ccd899c95b16e989f2c4087adb3fdf4f5c0e56`
- Input 1, STYLE ONLY: `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png`, sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`
- Input 2, identity reference: `spheres-web/ui/person-portraits/references/michel-temer-2009-reference-v1.jpg`, sha256 `ef8613540884a0f14847c16e9788307211a55b69e4bdafc206b03c8fee1daee8`, 2000x1312 RGB JPEG, 1,315,244 bytes
  - Photograph date: 2009-11-11. Licence: CC BY 3.0 (https://creativecommons.org/licenses/by/3.0).
  - Credit: J. Batista/Câmara dos Deputados, 'Dagoberto Nogueira Filho, Carlos Lupi e Michel Temer.jpg', Medalha Mérito Legislativo ceremony, Salão Negro, Chamber of Deputies, Brasília, 11 November 2009; CC BY 3.0, via Wikimedia Commons (Banco de Imagens da Câmara dos Deputados). Adapted for Spheres; no endorsement implied.
  - In frame: Temer is the silver-haired man standing at the right (light gray suit, plain white shirt, burgundy tie), face turned slightly to his left, hands held together in a presenting gesture. Carlos Lupi (centre, green diploma folder), Dagoberto Nogueira Filho (left), a seated audience, flags, spotlights and the Salão Negro are excluded by the prompt. The same bytes are Lupi's 2009 reference (job 5).
- Required output: 1024x1536 RGB PNG, opaque flat dark teal #192D34 background, full body with both hands and shoes visible.
- Pipeline job, window-scoped: `michel_temer-0a0870c78f13`. Default-inventory job planned to be partly covered after registration: `michel_temer-4a7a936add8d` (1990-01-01 to 2027-01-01).
- Leadership-production cartoon jobs this window could cover after generation, registration and review: `cartoon:michel_temer:2001-09-09:2005-01-01:v1`, `cartoon:michel_temer:2005-01-01:2009-03-10:v1`, `cartoon:michel_temer:2010-01-27:2010-06-15:v1`.
- Settle first: Optional: michel-temer-2009-reference-v1.jpg is byte-identical to carlos-lupi-2009-reference-v1.jpg (one three-person photograph). Keep both per-person paths, as the naming rule asks, or point both registrations at one file; the bytes and sha256 are the same either way.

### 2. Michel Temer (`michel_temer`), window 2 of 2

- Appearance window: 2010-06-15 to 2019-01-01 (exclusive end); status `needs_codex_agreement`.
- Output path (after review): `spheres-web/ui/person-portraits/michel-temer-cartoon-2010-v1.png`
- Prompt file: `tools/avatars/person-prompts/michel-temer-cartoon-2010-v1.txt`
  - sha256 (LF, as committed): `d3bfc1cbdb8723577ae3da80e38e74e05c94c5c8f6fb79341d5a2aaacfa9c5c0`
  - sha256 if your checkout wrote CRLF: `2ff1dbd260c257c7d58c7fdbf28e20a20f8c5b1caa7c170c89c4e1582e729c58`; git blob `a5131e748407d14d6bed42057fc3f450e0873659`
- Input 1, STYLE ONLY: `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png`, sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`
- Input 2, identity reference: `spheres-web/ui/person-portraits/references/michel-temer-2017-reference-v1.jpg`, sha256 `3d1fc06be507c6b1694fb6fcc8b8275b06cf073cfa33051955ad71f707bf97bc`, 5906x7087 RGB JPEG, 2,242,245 bytes
  - Photograph date: May 2017 (Commons gives 17 May, from file metadata only). Licence: CC BY 2.0 (https://creativecommons.org/licenses/by/2.0).
  - Credit: Beto Barata/PR, 'Presidente Michel Temer - Foto Oficial Original.jpg' (official portrait 'Foto Oficial do Presidente da República', made in May 2017; dated 17 May 2017 on Commons), Michel Temer Flickr stream photo 24439184568, CC BY 2.0, via Wikimedia Commons. Adapted for Spheres; no endorsement implied.
  - In frame: Temer alone, standing, framed to mid-thigh; the Brazilian flag, bookshelves, desk and white print border are excluded by the prompt.
- Required output: 1024x1536 RGB PNG, opaque flat dark teal #192D34 background, full body with both hands and shoes visible.
- Pipeline job, window-scoped: `michel_temer-555b84955d80`. Default-inventory job planned to be partly covered after registration: `michel_temer-4a7a936add8d` (1990-01-01 to 2027-01-01).
- Leadership-production cartoon jobs: none exist for this window.
- Settle first: Agree or amend the window 2010-06-15 to 2019-01-01 (start continues from window 1; end from C01-10, acceptance pending). Optional: the window is long (ages 69 to 78) and the reference is from May 2017. If Codex wants an early (vice-presidential) look, split the window and prepare a second reference from the 41 'Brasília - DF (4796...)' photographs of 13 July 2010 (Dilma Rousseff Flickr stream, 'CC BY-SA 2.0', 3506x2336-2338), 28 days after the window start. Not prepared here.

### 3. Baleia Rossi (`baleia_rossi`), window 1 of 1

- Appearance window: 2019-10-06 to 2026-09-08 (exclusive end); status `appearance_interval_from_handoff`.
- Output path (after review): `spheres-web/ui/person-portraits/baleia-rossi-cartoon-2019-v1.png`
- Prompt file: `tools/avatars/person-prompts/baleia-rossi-cartoon-2019-v1.txt`
  - sha256 (LF, as committed): `d1a0008ac591682d64e3aa4dfbd378e2789446181cde9f32486c8a203f77a8e4`
  - sha256 if your checkout wrote CRLF: `70fae18d4a903f77c71eb6a82b6e44685307b04836bae18b31126c651e5091d5`; git blob `cd1962142229660c79b9ab3cfc0b7de7956d740d`
- Input 1, STYLE ONLY: `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png`, sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`
- Input 2, identity reference: `spheres-web/ui/person-portraits/references/baleia-rossi-2021-reference-v1.jpg`, sha256 `6e97ad854a7731eaf5751bf001e0e14aad7147a40ad6e0b065b6a8093909d398`, 1514x2017 RGB JPEG, 2,079,037 bytes
  - Photograph date: 2021-12-08. Licence: CC BY 2.0 (https://creativecommons.org/licenses/by/2.0).
  - Credit: MDB Nacional, '08-12-2021 Lançamento da pré-candidatura de Simone Tebet à Presidência da República' (Flickr 51735864101), 8 December 2021, CC BY 2.0, via Wikimedia Commons; Commons crop '... (519) (cropped).jpg' by Commons user Przelijpdahl (CropTool, 18 January 2023). Adapted for Spheres; no endorsement implied.
  - In frame: Baleia Rossi alone, head and shoulders, speaking at a microphone; the microphones, palm and wall are excluded and the prompt draws him with lips closed.
- Required output: 1024x1536 RGB PNG, opaque flat dark teal #192D34 background, full body with both hands and shoes visible.
- Pipeline job, window-scoped: `baleia_rossi-86cb96c29aa9`. Default-inventory job planned to be partly covered after registration: `baleia_rossi-438d8a547750` (1990-01-01 to 2027-01-01).
- Leadership-production cartoon jobs this window could cover after generation, registration and review: `cartoon:baleia_rossi:2019-10-06:2020-01-01:v1`, `cartoon:baleia_rossi:2020-01-01:2025-01-01:v1`, `cartoon:baleia_rossi:2025-01-01:2026-09-08:v1`.

### 4. Leonel Brizola (`leonel_brizola`), window 1 of 1

- Appearance window: 1995-01-01 to 2004-06-21 (exclusive end); status `appearance_interval_from_handoff`.
- Output path (after review): `spheres-web/ui/person-portraits/leonel-brizola-cartoon-1995-v1.png`
- Prompt file: `tools/avatars/person-prompts/leonel-brizola-cartoon-1995-v1.txt`
  - sha256 (LF, as committed): `3170bb3479c8e23145ab28da573898cc0342c3f5c065475d8f9216f5339dc90d`
  - sha256 if your checkout wrote CRLF: `49f2cbe4e1e8a84ff966a86cd77e1c249c8e0a25f81aaf57df7615b8608374db`; git blob `b2000a0ecc2943ce9ef5c335f40e10563000f601`
- Input 1, STYLE ONLY: `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png`, sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`
- Input 2, identity reference: `spheres-web/ui/person-portraits/references/leonel-brizola-1998-reference-v1.jpg`, sha256 `40a4a67c4e04cd4d0dccaca29645bffd48123bd41011d010e54d4ecc12109fd1`, 2959x1941 RGB JPEG, 7,660,996 bytes
  - Photograph date: 1998-09-19 (photographer's own statement; no embedded capture date). Licence: CC BY-SA 4.0 (https://creativecommons.org/licenses/by-sa/4.0).
  - Credit: Sérgio Neglia (embedded credit '© Serginho Neglia'), 'Leonel Brizola em Palmeira das Missões.jpg', 19 September 1998, own work, CC BY-SA 4.0, via Wikimedia Commons. Adapted for Spheres; no endorsement implied.
  - In frame: Brizola is the large figure at the right, in side profile under a wide-brimmed gaucho hat and red poncho, speaking into a microphone at a rally. The prompt draws him bareheaded in a suit, turned to a slight three-quarter view; the forehead and crown are a declared artistic completion. The crowd, flags, placards, balloons, street and signs are excluded.
- Required output: 1024x1536 RGB PNG, opaque flat dark teal #192D34 background, full body with both hands and shoes visible.
- Pipeline job, window-scoped: `leonel_brizola-fa7af2b7d699`. Default-inventory job planned to be partly covered after registration: `leonel_brizola-cbbd6332435f` (1995-01-01 to 2027-01-01).
- Leadership-production cartoon jobs this window could cover after generation, registration and review: `cartoon:leonel_brizola:1995-01-01:2000-01-01:v1`, `cartoon:leonel_brizola:2000-01-01:2004-06-21:v1`.
- Settle first: Rule on the registry birth date (1922-01-22 in the Chamber of Deputies open data and Wikidata, 1922-11-22 in party_leaders.json). This task does not edit the registry; the prompt's ages hold for both. The byte-identical reference carries the photographer's own embedded IPTC contact fields (a work phone number and e-mail), which Commons already publishes in the same file. Committing the copy republishes them; stripping them would break byte identity. Accept, or substitute a decision of your own before integration. Accept the 7.66 MB reference file size.

### 5. Carlos Lupi (`carlos_lupi`), window 1 of 3

- Appearance window: 2004-06-30 to 2015-01-01 (exclusive end); status `appearance_interval_from_handoff`.
- Output path (after review): `spheres-web/ui/person-portraits/carlos-lupi-cartoon-2004-v1.png`
- Prompt file: `tools/avatars/person-prompts/carlos-lupi-cartoon-2004-v1.txt`
  - sha256 (LF, as committed): `0c57b3987daa007d9bfb103b4269499a80f359b923b42e1eb6fd4a273c00cf56`
  - sha256 if your checkout wrote CRLF: `bc2368fa94260f83635ba057b3fc479e58e010d0d2cb51137639e359d1f99f87`; git blob `8739a723a6ff84281f1b657021013d17d2100a60`
- Input 1, STYLE ONLY: `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png`, sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`
- Input 2, identity reference: `spheres-web/ui/person-portraits/references/carlos-lupi-2009-reference-v1.jpg`, sha256 `ef8613540884a0f14847c16e9788307211a55b69e4bdafc206b03c8fee1daee8`, 2000x1312 RGB JPEG, 1,315,244 bytes
  - Photograph date: 2009-11-11. Licence: CC BY 3.0 (https://creativecommons.org/licenses/by/3.0).
  - Credit: J. Batista/Câmara dos Deputados, 'Dagoberto Nogueira Filho, Carlos Lupi e Michel Temer.jpg', Medalha Mérito Legislativo ceremony, Salão Negro, Chamber of Deputies, Brasília, 11 November 2009; CC BY 3.0, via Wikimedia Commons (Banco de Imagens da Câmara dos Deputados). Adapted for Spheres; no endorsement implied.
  - In frame: Lupi is the smiling central man holding an open green diploma folder; Dagoberto Nogueira Filho (left), Michel Temer (right), the audience, flags, spotlights and hall are excluded by the prompt, as are the folder and lapel pin. The same bytes are Temer's 2009 reference (job 1).
- Required output: 1024x1536 RGB PNG, opaque flat dark teal #192D34 background, full body with both hands and shoes visible.
- Pipeline job, window-scoped: `carlos_lupi-45b22e501a80`. Default-inventory job planned to be partly covered after registration: `carlos_lupi-1a8bd5270f54` (1990-01-01 to 2027-01-01).
- Leadership-production cartoon jobs this window could cover after generation, registration and review: `cartoon:carlos_lupi:2004-06-30:2005-01-01:v1`, `cartoon:carlos_lupi:2005-01-01:2010-01-01:v1`, `cartoon:carlos_lupi:2010-01-01:2015-01-01:v1`.
- Settle first: Agree the window start 2004-06-30, which rests on leadership production and the month-precision registry term rather than accepted C01 research.

### 6. Carlos Lupi (`carlos_lupi`), held 2015–2026 window — SUPERSEDED, DO NOT RENDER

Superseded on 2 October 2026 by jobs 6a and 6b (era split at 2020-01-01; see [`lupi-era-correction-batch-01.json`](lupi-era-correction-batch-01.json) and identity review `/batches/0/people/5`). Never submit `tools/avatars/person-prompts/carlos-lupi-cartoon-2015-v1.txt` (LF sha256 `04a38571bc903f152d9b43952956fca94c88451bcbe0378ab0ae71fef4b41aa8`). It stays in the repository unchanged as the reviewed held proposal, and its planned output `carlos-lupi-cartoon-2015-v1.png` is never created. The window 2015-01-01 to 2026-09-08 and its window-scoped job `carlos_lupi-7293a9e06d87` are no longer planned. Hold reason (Codex review of `383086651685ff2f8ba6aaff3a575cc4ae67df70`): the prompt mandates the gray 2023 look throughout 2015–2026 despite acknowledging the earlier darker appearance.

### 6a. Carlos Lupi (`carlos_lupi`), window 2 of 3 — replaces job 6; render after Codex reviews the era split

- Appearance window: 2015-01-01 to 2020-01-01 (exclusive end); status `proposed_era_split_pending_codex_review`. The end is the era split, an existing leadership-production job boundary used as an art boundary. Records: identity review `/batches/0/people/6`; [`lupi-era-correction-batch-01.json`](lupi-era-correction-batch-01.json) `/windows/0`.
- Output path (after review): `spheres-web/ui/person-portraits/carlos-lupi-cartoon-2015-v2.png`
- Prompt file: `tools/avatars/person-prompts/carlos-lupi-cartoon-2015-v2.txt`
  - sha256 (LF, as committed): `777d1e145a0ffe2ed9a2e02d4e83bf948d27f445083a160ecf6065c5704cc530`
  - sha256 if your checkout wrote CRLF (no `.gitattributes` rule yet): `dfe48637c274a3314db55f1ee0fc35af873d8d619080776aaf51456445b85ca1`; git blob `88bde183da1b7e96c05d039f4357346044dc8787`; 4,890 bytes, 9 lines
- Input order:
  1. STYLE ONLY: `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png`, sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`
  2. Identity reference: `spheres-web/ui/person-portraits/references/carlos-lupi-2015-reference-v1.jpg`, sha256 `fbaff7ed9f7313fef48c081d62689e6ce5490a08512b9b833ea4c65c3b3b23a6`, 3184x2120 RGB JPEG, 1,696,493 bytes
  - Photograph date: 2015-08-18 (EXIF DateTimeOriginal 2015-08-18 12:22:13 and the Commons/Flickr import; no news report corroborates the day; the Commons upload of 2015-08-31 is not the capture date). Licence: CC BY 2.0 (https://creativecommons.org/licenses/by/2.0).
  - Credit: Jane de Araújo/Agência Senado, 'Presidência do Senado (20659268666).jpg', visit of the PDT leadership to the President of the Senate, Brasília, 18 August 2015; CC BY 2.0, via Wikimedia Commons (Senado Federal Flickr 20659268666; licence confirmed by FlickreviewR on 31 August 2015). Adapted for Spheres; no endorsement implied.
  - In frame: four men in a row; Lupi is third from the left, the only bearded man, frontal and full length, smiling, in a royal-blue windowpane jacket, slate-navy trousers, a white shirt and a burgundy tie with white stripes, hands clasped; dark charcoal hair with gray at the temples and sides, dark brown-black beard with gray at the chin and sideburns. Renan Calheiros, Acir Gurgacz, André Figueiredo, the paneled wall, coat of arms, sofa, paintings, chairs, floor and table edge are excluded by the prompt.
  - Era limit, declared in the prompt: the photograph is 7.5 months into a window covering ages 57 to 62. An uploader-dated photograph of 1 May 2018 (`File:Lupi.png`, CC BY-SA 4.0, own-work claim and date unverified; a comparison, not a reference) shows him grayer, with salt-and-pepper hair and a gray-brown beard. The prompt allows a little more gray than in 2015 but does not show the 2018–2019 graying.
- Required output: 1024x1536 RGB PNG, opaque flat dark teal #192D34 background, full body with both hands and shoes visible; no text, flags, insignia or extra people. Up to **3 attempts**.
- Return every attempt **unchanged**: the `exec-*.png` file exactly as the tool wrote it, with its path, sha256, width, height and mode, the prompt sha256 actually submitted, the input order used, the generation date and any refusal or failure (format below). No crop, resize, recolour or re-encode.
- Pipeline job, window-scoped: `carlos_lupi-400fe963352c`. Default-inventory job planned to be partly covered after registration: `carlos_lupi-1a8bd5270f54` (1990-01-01 to 2027-01-01).
- Leadership-production cartoon job this window could cover after generation, registration and review: `cartoon:carlos_lupi:2015-01-01:2020-01-01:v1`. Coverage is prospective; nothing is closed before reviewed registration.
- Settle first: agree the split and this window (no accepted C01 observation); rule on `File:Lupi.png` (accept the softened wording, or verify it and split again near 2018); accept the reference's embedded Senate-archive IPTC contact fields and camera serial number; add the LF rule.

### 6b. Carlos Lupi (`carlos_lupi`), window 3 of 3 — replaces job 6; render after Codex reviews the era split

- Appearance window: 2020-01-01 to 2026-09-08 (exclusive end); status `proposed_era_split_pending_codex_review`. It contains the three accepted C01-34 PDT observations (2021-12-21, 2025-05-21, 2026-09-04). Records: identity review `/batches/0/people/7`; [`lupi-era-correction-batch-01.json`](lupi-era-correction-batch-01.json) `/windows/1`.
- Output path (after review): `spheres-web/ui/person-portraits/carlos-lupi-cartoon-2020-v1.png`
- Prompt file: `tools/avatars/person-prompts/carlos-lupi-cartoon-2020-v1.txt`
  - sha256 (LF, as committed): `cc0a4c92eeea27ed6b37769a41ea8c3de6ff9edd75e10f47cbb1adea1070a218`
  - sha256 if your checkout wrote CRLF (no `.gitattributes` rule yet): `0e2fc6040e67b7cbe2bea70378478b35263d8c27ac7826aad3ff19936532c7e3`; git blob `8e69112342bd3b5193be572d2ffcddb63188b14f`; 4,456 bytes, 9 lines
- Input order:
  1. STYLE ONLY: `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png`, sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`
  2. Identity reference: `spheres-web/ui/person-portraits/references/carlos-lupi-2023-reference-v1.jpg`, sha256 `44a6d135f5b77300821f15b38b93fe0ca8134d7ad6608520475d48ebe39efbfb`, 1545x2064 RGB JPEG, 687,212 bytes (the reviewed batch-1 reference, unchanged)
  - Photograph date: 2023-10-24. Licence: CC BY 2.0 (https://creativecommons.org/licenses/by/2.0).
  - Credit: Geraldo Magela/Agência Senado, 'Carlos Lupi. CAS - Comissão de Assuntos Sociais (cropped).jpg', 24 October 2023, CC BY 2.0, via Wikimedia Commons (Senado Federal Flickr 53282670483; licence confirmed by FlickreviewR 2 on 30 January 2024 on the parent file); crop uploaded by Commons user Rodolfo Matias on 30 January 2024. Adapted for Spheres; no endorsement implied.
  - In frame: Lupi alone, chest-up, speaking at a Senate committee with both index fingers raised; the microphone across his beard, the wall panels and the lapel pin are excluded, and the prompt asks for a closed-mouth smile and a neutral pose.
  - Era limit, declared in the prompt: the photograph is near the middle of a window covering ages 62 to 69. The prompt asks for salt-and-pepper to gray hair and a salt-and-pepper beard, following the photograph or very slightly darker, never dark. Comparison photographs (1 May 2018, uploader-dated and unverified; January 2023) show him a little darker than in October 2023, which may be near the grayest point of the window; no verified dated photograph from 2020 to 2022 was found.
- Required output: 1024x1536 RGB PNG, opaque flat dark teal #192D34 background, full body with both hands and shoes visible; no text, flags, insignia or extra people. Up to **3 attempts**.
- Return every attempt **unchanged**: the `exec-*.png` file exactly as the tool wrote it, with its path, sha256, width, height and mode, the prompt sha256 actually submitted, the input order used, the generation date and any refusal or failure (format below). No crop, resize, recolour or re-encode.
- Pipeline job, window-scoped: `carlos_lupi-84f4126df75f`. Default-inventory job planned to be partly covered after registration: `carlos_lupi-1a8bd5270f54` (1990-01-01 to 2027-01-01).
- Leadership-production cartoon jobs this window could cover after generation, registration and review: `cartoon:carlos_lupi:2020-01-01:2025-01-01:v1`, `cartoon:carlos_lupi:2025-01-01:2026-09-08:v1`. Coverage is prospective; nothing is closed before reviewed registration.
- Settle first: agree this window and the reworded prompt (if one portrait still cannot represent 2020–2022, the window needs a narrower start or a verified 2020–2022 reference, and none was found on Commons); add the LF rule.

## Excluded (not rendered)

Daniel Tourinho (`daniel_sampaio_tourinho`), window 2014-05-16 to 2026-09-08 (C01-43 Agir, five accepted observations), has no prompt and no reference in the repository. No qualifying likeness reference was found under this preparation's search and licence filter. No dated photograph of him with a licence string that matches FREE_LICENSES was found on Wikimedia Commons, Wikidata or pt.wikipedia, and no free institutional photograph naming him was found in that search. The one photograph identified, his TSE 2010 candidate photo, is not on Commons; the TSE portal grants it under an unversioned 'Creative Commons Attribution' licence, so no exact FREE_LICENSES string can be recorded for this file; and it is 161x225 greyscale, from 2010 or earlier (before the window). The batch rejects TSE thumbnails of that size for Temer and Baleia Rossi too. No reference was copied and no prompt was written; no face is invented and nobody substitutes for him.

His jobs stay open: `daniel_sampaio_tourinho-fde53fa22842`, `daniel_sampaio_tourinho-c02873ad7899`, `cartoon:daniel_sampaio_tourinho:2026-07-05:2026-09-08:v1`. There is no reserve: no other registry person has an accepted Brazil observation.

## Return format

One entry per attempt, for example:

```json
{
  "person_id": "baleia_rossi",
  "attempt": 1,
  "generator": "OpenAI built-in image_gen",
  "generated_at": "YYYY-MM-DD",
  "prompt_path": "tools/avatars/person-prompts/baleia-rossi-cartoon-2019-v1.txt",
  "prompt_sha256_submitted": "<sha256>",
  "inputs": [
    {
      "order": 1,
      "path": "spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png",
      "sha256": "8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef",
      "use": "STYLE ONLY"
    },
    {
      "order": 2,
      "path": "<identity reference>",
      "sha256": "<sha256>",
      "use": "identity reference"
    }
  ],
  "output": {
    "path": "<exec-*.png as written>",
    "sha256": "<sha256>",
    "width": 1024,
    "height": 1536,
    "mode": "RGB"
  },
  "edited": false,
  "notes": ""
}
```
