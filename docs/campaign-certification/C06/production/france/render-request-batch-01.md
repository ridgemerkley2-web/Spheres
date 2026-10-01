# France cast batch 1: render request for Codex (FR-CAST-B01)

From Claude, task `CLAUDE-C06-FRANCE-01`, branch `claude/c06-fr-01`, 1 October 2026. State: **awaiting Codex render**. Nothing here is rendered, reviewed or approved yet.

Identity review: [`identity-review-batch-01.json`](identity-review-batch-01.json) (sha256 `7d5fb0fa3bf98df913943fe588ff18c87d86ed24d268d1d9fbd3f14538bbb5f7`). It holds the accepted C01 observations, the verified references, the rejected candidates and the verifier dispositions.

## What Codex does

For each of the seven primary jobs below:

1. Check the inputs: the prompt file's sha256 equals the pinned value (`git show HEAD:<prompt path> | sha256sum`), and the style anchor and identity reference match their sha256.
2. Generate with the built-in image tool (the pipeline requires generator `OpenAI built-in image_gen`). Submit the prompt file's text exactly as committed, with the images in this order:
   - image 1: the style anchor `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png` (sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`), **STYLE ONLY**. It supplies outlines, cel shading, proportions, framing and the dark teal background, never the face, hair, clothes or pose;
   - image 2: that job's identity reference.
3. Required output: 1024x1536 RGB, opaque. Make up to **3 attempts per portrait**.
4. Return every attempt **unchanged**: the `exec-*.png` files exactly as the tool wrote them, each with its path, sha256, width, height and mode, the prompt sha256 actually submitted, the input order used, the generation date and any refusal or failure. Do not crop, resize, recolour, re-encode or otherwise edit them. You may say which attempt you would choose.
5. Claude then reviews identity, likeness, era and image quality, names the reviewer, copies the chosen file byte-identical to the output path, and writes the batch generation record, the `person_portraits.json` records and the registration receipt. Review flags stay false until those checks actually happen. No human approval is claimed.

Line endings: the eight prompt files are LF and no `.gitattributes` rule covers them yet (core.autocrlf=true on this machine). This task may not edit `.gitattributes`, so please add a rule at integration, as for `tonga-*.txt`, for example `tools/avatars/person-prompts/<each France prompt>.txt text eol=lf`. Until then a Windows checkout writes CRLF; its hash is listed for each job.

## Decisions needed from Codex

- `francois_mitterrand`: Agree or amend the requested window 1991-01-01 to 1995-05-17 (the handoff marks it as needing Codex agreement).
- `francois_mitterrand`: Rule whether the reference keeps the Commons-stated year 1994 in its reference_id and filename or is renamed (for example to an undated or 'c1992' name) before registration; the bytes and sha256 stay identical either way.
- `francois_mitterrand`: If the uncertain date is unacceptable, switch to the exactly dated Godefroy alternate (28 November 1991, CC BY 3.0, black-and-white side profile); that would need a new prompt.
- `laurent_fabius`: Optional before rendering: search the EC Audiovisual Service (and the EP Multimedia Centre, if reachable without bypassing a challenge) for a dated 1989-1992 photograph of Fabius as the main subject with an exact CC BY 4.0 licence. If one qualifies it would replace this reference as v2; otherwise render with 53Fi3647 and record the search outcome.
- `alain_juppe`: Agree or amend the window start 1994-11-30, which rests on a registry RPR acting-president term rather than accepted C01 research.
- `alain_juppe`: Accept the 17 MB reference file size or substitute the smaller Archives nationales 1995 alternate.
- `francois_hollande`: Decide whether to split the window at 2005-01-01 (see requested_window.possible_split).
- `martine_aubry`: Render only if one of the seven primary portraits is excluded at Codex review, or if Codex chooses to render the alternate.

## Primary jobs

### 1. François Mitterrand (`francois_mitterrand`)

- Appearance window: 1991-01-01 to 1995-05-17 (exclusive end); status `needs_codex_agreement`.
- Output path (after review): `spheres-web/ui/person-portraits/francois-mitterrand-cartoon-1991-v1.png`
- Prompt file: `tools/avatars/person-prompts/francois-mitterrand-cartoon-1991-v1.txt`
  - sha256 (LF, as committed): `9384edce0aa124138909a5685c958abceb94fa7590c1c2546f79d2b98a756aa5`
  - sha256 if your checkout wrote CRLF: `27e751398366f635ff83acfa555222669ff6887b96e7feb19044e01da4e9bc29`; git blob `f2c1bc171472a16af84c181d18925f7be3c7dbf2`
- Input 1, STYLE ONLY: `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png`, sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`
- Input 2, identity reference: `spheres-web/ui/person-portraits/references/francois-mitterrand-1994-reference-v1.jpg`, sha256 `bb949d1cce4a2dcef0f7e1685676ac43721201257a7d80e3a3a271fb13e7a93b`, 927x1302 RGB JPEG
  - Photograph date: 1994 (Commons author statement, year only; uncertain). Licence: CC BY-SA 4.0 (https://creativecommons.org/licenses/by-sa/4.0).
  - Credit: Gorup de Besanez, 'François Mitterrand (cropped).jpg', dated 1994 on Commons (year uncertain), own work, CC BY-SA 4.0, via Wikimedia Commons; crop by Commons user Cheep (CropTool, 26 April 2021) of 'Roland Dumas, François Mitterrand and Gianni De Michelis in 1994.jpg'. Adapted for Spheres; no endorsement implied.
  - In frame: Mitterrand alone (head and shoulders); the crop removes Dumas and De Michelis.
- Required output: 1024x1536 RGB PNG, opaque flat dark teal #192D34 background, full body with both hands and shoes visible.
- Pipeline job, window-scoped: `francois_mitterrand-0ec871bb367f`. Default-inventory job partly closed: `francois_mitterrand-97412cfcaffc`.
- Leadership-production cartoon jobs: none exist for this person.
- Settle first: Agree or amend the requested window 1991-01-01 to 1995-05-17 (the handoff marks it as needing Codex agreement). Rule whether the reference keeps the Commons-stated year 1994 in its reference_id and filename or is renamed (for example to an undated or 'c1992' name) before registration; the bytes and sha256 stay identical either way. If the uncertain date is unacceptable, switch to the exactly dated Godefroy alternate (28 November 1991, CC BY 3.0, black-and-white side profile); that would need a new prompt.

### 2. Michel Rocard (`michel_rocard`)

- Appearance window: 1990-01-01 to 1994-06-19 (exclusive end); status `appearance_interval_from_handoff`.
- Output path (after review): `spheres-web/ui/person-portraits/michel-rocard-cartoon-1990-v1.png`
- Prompt file: `tools/avatars/person-prompts/michel-rocard-cartoon-1990-v1.txt`
  - sha256 (LF, as committed): `d6e36504fcb3980b9ec5ab7a32715dfbdccece26f092bb35306f7728d6271d29`
  - sha256 if your checkout wrote CRLF: `55291167b13b48376c88ec5d052fad9a518ceb26872e9579f0c034f39ad14e81`; git blob `1eaa8e278b46455c2c8c2e37b98e77aa9ff4385a`
- Input 1, STYLE ONLY: `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png`, sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`
- Input 2, identity reference: `spheres-web/ui/person-portraits/references/michel-rocard-1991-reference-v1.jpg`, sha256 `16842eea37ebc930b4c18851b2ed617d8f3d6fa650a1788360337b01464090c9`, 806x1075 RGB JPEG
  - Photograph date: 1991. Licence: CC BY 2.0 (https://creativecommons.org/licenses/by/2.0).
  - Credit: Jean Weber / INRA, DIST; 1991; CC BY 2.0. Commons crop 'Rocard 1991 cropped-1.jpg' of 'Salon du livre 1991-22-cliche Jean Weber.jpg' (Flickr 31492990916, INRA DIST stream). Adapted for Spheres; no endorsement implied.
  - In frame: Rocard is the clear main subject; a uniformed officer at the left edge and part of another person at the right are excluded by the prompt.
- Required output: 1024x1536 RGB PNG, opaque flat dark teal #192D34 background, full body with both hands and shoes visible.
- Pipeline job, window-scoped: `michel_rocard-3e73d0165afa`. Default-inventory job partly closed: `michel_rocard-435bcbe4a1a4`.
- Leadership-production cartoon jobs this window closes: `cartoon:michel_rocard:1993-04-03:1993-10-01:v1`, `cartoon:michel_rocard:1993-10-31:1994-06-19:v1`.

### 3. Laurent Fabius (`laurent_fabius`)

- Appearance window: 1992-01-09 to 1993-04-03 (exclusive end); status `appearance_interval_from_handoff`.
- Output path (after review): `spheres-web/ui/person-portraits/laurent-fabius-cartoon-1992-v1.png`
- Prompt file: `tools/avatars/person-prompts/laurent-fabius-cartoon-1992-v1.txt`
  - sha256 (LF, as committed): `9f2f006009031ccccd89cb50d8725603d1450e71299d855e160567ede34bfcab`
  - sha256 if your checkout wrote CRLF: `0c619be94d2a467fe4fed7bb13e444f2e98e6735c2fa669c508af194c40438c1`; git blob `b92fa3a6d12c09b29b596024fb83db1f815fccee`
- Input 1, STYLE ONLY: `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png`, sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`
- Input 2, identity reference: `spheres-web/ui/person-portraits/references/laurent-fabius-1984-reference-v1.jpg`, sha256 `b6779f1fe737d239cbd674fab3987c6658d314b265af7c078a93b90732424664`, 2268x1524 RGB JPEG
  - Photograph date: 1984-08-28. Licence: CC BY-SA 4.0 (https://creativecommons.org/licenses/by-sa/4.0).
  - Credit: André Cros / Fonds André Cros, Archives municipales de Toulouse, 53Fi3647, 28 August 1984; CC BY-SA 4.0 (deliberation n°27.3 of 23 June 2017 of the Town Council of the City of Toulouse); via Wikimedia Commons. Adapted for Spheres; no endorsement implied.
  - In frame: Fabius is the central man facing the camera in a medium-gray suit. Dominique Baudis is in the right background (light suit, lapel badge), as the independent verifier established by comparison with frames 53Fi3643 and 53Fi3646 of the same series. The smiling gray-haired man shaking Fabius's hand at left is unidentified. Other officials, a photographer and an aircraft are also in frame. The preparer's record had wrongly named the man at left as Baudis; this record and the prompt are corrected.
- Required output: 1024x1536 RGB PNG, opaque flat dark teal #192D34 background, full body with both hands and shoes visible.
- Pipeline job, window-scoped: `laurent_fabius-eab265350133`. Default-inventory job partly closed: `laurent_fabius-83f5d63841a0`.
- Leadership-production cartoon jobs this window closes: `cartoon:laurent_fabius:1992-01-09:1993-04-03:v1`.
- Settle first: Optional before rendering: search the EC Audiovisual Service (and the EP Multimedia Centre, if reachable without bypassing a challenge) for a dated 1989-1992 photograph of Fabius as the main subject with an exact CC BY 4.0 licence. If one qualifies it would replace this reference as v2; otherwise render with 53Fi3647 and record the search outcome.

### 4. Henri Emmanuelli (`henri_emmanuelli`)

- Appearance window: 1994-06-19 to 1995-10-14 (exclusive end); status `appearance_interval_from_handoff`.
- Output path (after review): `spheres-web/ui/person-portraits/henri-emmanuelli-cartoon-1994-v1.png`
- Prompt file: `tools/avatars/person-prompts/henri-emmanuelli-cartoon-1994-v1.txt`
  - sha256 (LF, as committed): `c172cbdf450f29518f86cc8c73adcf4fd404d87de0da44876930b0288fc4912d`
  - sha256 if your checkout wrote CRLF: `3ef3a2e15fc59901db94af55b01a2a6a3199d30e7b55e8e32a4a4c8b197619f2`; git blob `1b6904c7922740f9f115226778adda665966b0c6`
- Input 1, STYLE ONLY: `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png`, sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`
- Input 2, identity reference: `spheres-web/ui/person-portraits/references/henri-emmanuelli-2005-reference-v1.jpg`, sha256 `c902f7767f098e234478f786d0b7067244f252a563f2fe140c4fa74476e2b65b`, 1551x798 RGB JPEG
  - Photograph date: 2005-05. Licence: CC BY 3.0 (https://creativecommons.org/licenses/by/3.0).
  - Credit: Kenji-Baptiste OIKAWA, 'Henri Emmanuelli 2005' (May 2005), CC BY 3.0 via Wikimedia Commons. Adapted for Spheres; no endorsement implied.
  - In frame: Emmanuelli alone at a 2005 'Non' referendum rally lectern, in shirt sleeves, seen from slightly below; red backdrop, slogan banner, microphones and a campaign sticker are excluded by the prompt.
- Required output: 1024x1536 RGB PNG, opaque flat dark teal #192D34 background, full body with both hands and shoes visible.
- Pipeline job, window-scoped: `henri_emmanuelli-4b913639a6cc`. Default-inventory job partly closed: `henri_emmanuelli-39ed314c9b95`.
- Leadership-production cartoon jobs this window closes: `cartoon:henri_emmanuelli:1994-06-19:1995-01-01:v1`, `cartoon:henri_emmanuelli:1995-01-01:1995-10-14:v1`.

### 5. Alain Juppé (`alain_juppe`)

- Appearance window: 1994-11-30 to 1997-06-01 (exclusive end); status `orchestrator_choice_needs_codex_agreement`.
- Output path (after review): `spheres-web/ui/person-portraits/alain-juppe-cartoon-1994-v1.png`
- Prompt file: `tools/avatars/person-prompts/alain-juppe-cartoon-1994-v1.txt`
  - sha256 (LF, as committed): `cd52698af05019d711a3420b3f3fd7c1c05c1b59a5a342dae3bfbc6e76671d58`
  - sha256 if your checkout wrote CRLF: `98597596f2560b19996a5ebeca7488eec7241db60b3af1d79fe7a4ec1e2efc7b`; git blob `643b265f728e27b5dba9573334fa451128f19329`
- Input 1, STYLE ONLY: `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png`, sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`
- Input 2, identity reference: `spheres-web/ui/person-portraits/references/alain-juppe-1996-reference-v1.jpg`, sha256 `c663649d218f7271c8280190ee13e057c61c8d7fed6c33bdefd896e87b9abff2`, 5088x3562 RGB JPEG
  - Photograph date: 1996-02-12. Licence: CC BY 4.0 (https://creativecommons.org/licenses/by/4.0).
  - Credit: Tatarstan.ru (photo report https://shaimiev.tatarstan.ru/pressa/photoreports/photoreport/4787153.htm; hi-res scan http://www.speaker.tatarstan.ru/fs/a_photo_photos/122_pic.jpg), 12 February 1996, CC BY 4.0, via Wikimedia Commons. Adapted for Spheres; no endorsement implied.
  - In frame: Juppé is the man standing second from the left (balding, gray suit, light-blue shirt); Mintimer Shaimiev holds a souvenir folder in the centre; Farid Mukhametshin and others also appear; table flags, a microphone and a Cyrillic banner are in frame. Both of Juppé's hands are visible (his right hand holds a small white card at the table edge, his left hangs beside the table flags) and the frame runs to about mid-thigh.
- Required output: 1024x1536 RGB PNG, opaque flat dark teal #192D34 background, full body with both hands and shoes visible.
- Pipeline job, window-scoped: `alain_juppe-d0ebd721cf60`. Default-inventory job partly closed: `alain_juppe-a03c353185c4`.
- Leadership-production cartoon jobs this window closes: `cartoon:alain_juppe:1994-11-30:1995-01-01:v1`, `cartoon:alain_juppe:1995-01-01:1995-10-01:v1`, `cartoon:alain_juppe:1995-10-31:1997-06-01:v1`.
- Settle first: Agree or amend the window start 1994-11-30, which rests on a registry RPR acting-president term rather than accepted C01 research. Accept the 17 MB reference file size or substitute the smaller Archives nationales 1995 alternate.

### 6. Lionel Jospin (`lionel_jospin`)

- Appearance window: 1995-10-14 to 1997-06-07 (exclusive end); status `appearance_interval_from_handoff`.
- Output path (after review): `spheres-web/ui/person-portraits/lionel-jospin-cartoon-1995-v1.png`
- Prompt file: `tools/avatars/person-prompts/lionel-jospin-cartoon-1995-v1.txt`
  - sha256 (LF, as committed): `6eb81e10d03c043a8d9b1021b4ae3dbdfdac9777a69723d5345572ce0fb33ca5`
  - sha256 if your checkout wrote CRLF: `97acd7be9f9dc1fd19871b6d81aa76dba404b1419222f30e09191729a22a2e51`; git blob `5096aa4f7db977fbc1da3acb6499966dbfa0083b`
- Input 1, STYLE ONLY: `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png`, sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`
- Input 2, identity reference: `spheres-web/ui/person-portraits/references/lionel-jospin-1998-reference-v1.jpg`, sha256 `c5166b264aaa5f901b25c44d45fe19fac0ed937038cdaab46cc28e2cd167f612`, 723x1121 RGB JPEG
  - Photograph date: 1998-10-12 or 1998-10-13 (sources disagree). Licence: CC BY 4.0 (https://creativecommons.org/licenses/by/4.0).
  - Credit: Benoît Bourgeois / European Communities, 1998 / EC - Audiovisual Service, P-002498/01-8 (12 or 13 October 1998); © European Union; CC BY 4.0; Commons crop 'Jospin 1998 (cropped).jpg' (CropTool, 11 June 2024) of 'Lionel Jospin & Jacques Santer - 1998.jpg'. Adapted for Spheres; no endorsement implied.
  - In frame: Jospin alone in the crop (head and shoulders); Jacques Santer is removed by the Commons crop; a blue-and-yellow European flag star is behind him and excluded by the prompt.
- Required output: 1024x1536 RGB PNG, opaque flat dark teal #192D34 background, full body with both hands and shoes visible.
- Pipeline job, window-scoped: `lionel_jospin-75f8fcab4088`. Default-inventory job partly closed: `lionel_jospin-af1838a40839`.
- Leadership-production cartoon jobs this window closes: `cartoon:lionel_jospin:1995-10-14:1997-06-01:v1`.

### 7. François Hollande (`francois_hollande`)

- Appearance window: 1997-06-30 to 2008-11-01 (exclusive end); status `appearance_interval_from_handoff`.
- Output path (after review): `spheres-web/ui/person-portraits/francois-hollande-cartoon-1997-v1.png`
- Prompt file: `tools/avatars/person-prompts/francois-hollande-cartoon-1997-v1.txt`
  - sha256 (LF, as committed): `1453824cbd45beee1e84333ee0baa3f0c963d4605de7f31270ca999a8f6ed2b0`
  - sha256 if your checkout wrote CRLF: `769259f5fc414992f121a06752c61772c95fc128c56ce2104f57a1eff1d6ce8d`; git blob `a7622d1dcb7c40e20fd5ac40788a61133ef2265b`
- Input 1, STYLE ONLY: `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png`, sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`
- Input 2, identity reference: `spheres-web/ui/person-portraits/references/francois-hollande-2007-reference-v1.jpg`, sha256 `fbb52a4ff4ec0fe45b644386aaa720f298e824f64f06f625acf127f1a0af1fee`, 1600x2424 RGB JPEG
  - Photograph date: 2007-05-29. Licence: CC BY 2.5 (https://creativecommons.org/licenses/by/2.5).
  - Credit: © Marie-Lan Nguyen / Wikimedia Commons, CC BY 2.5; François Hollande at the Socialist Party rally at the Zénith, Paris, 29 May 2007; supported by Wikimédia France. Adapted for Spheres; no endorsement implied.
  - In frame: Hollande seated, waist-up, arms crossed, holding handwritten papers; other people behind and beside him are excluded by the prompt.
- Required output: 1024x1536 RGB PNG, opaque flat dark teal #192D34 background, full body with both hands and shoes visible.
- Pipeline job, window-scoped: `francois_hollande-cb70c49c38aa`. Default-inventory job partly closed: `francois_hollande-4cadb4cb57df`.
- Leadership-production cartoon jobs this window closes: `cartoon:francois_hollande:1997-06-30:1997-11-01:v1`, `cartoon:francois_hollande:1997-11-30:2000-01-01:v1`, `cartoon:francois_hollande:2000-01-01:2005-01-01:v1`, `cartoon:francois_hollande:2005-01-01:2008-11-01:v1`.
- Settle first: Decide whether to split the window at 2005-01-01 (see requested_window.possible_split).

## Reserve (do not render unless asked)

Martine Aubry is the prepared alternate. All seven primary people passed after mechanical fixes, so she is not in this render. Render her only if one of the seven is excluded at review or Codex chooses to.

### 8. Martine Aubry (`martine_aubry`)

- Appearance window: 2008-11-29 to 2012-09-01 (exclusive end); status `appearance_interval_from_handoff`.
- Output path (after review): `spheres-web/ui/person-portraits/martine-aubry-cartoon-2008-v1.png`
- Prompt file: `tools/avatars/person-prompts/martine-aubry-cartoon-2008-v1.txt`
  - sha256 (LF, as committed): `dc36c41ffb6300d273c37a4d0172bd6f2722338e311213889255b72eca3d6842`
  - sha256 if your checkout wrote CRLF: `15f0e6312c96b0faa76afee8d79d9b7b334b04aebb1239a62f74751a5a062d1f`; git blob `30756124eb342239b7930aeef6bfb07f491a044d`
- Input 1, STYLE ONLY: `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png`, sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`
- Input 2, identity reference: `spheres-web/ui/person-portraits/references/martine-aubry-2010-reference-v1.jpg`, sha256 `5cb06f5a61a081ed1d9fee9a94adcabe1b0b9fd09427212ae037b393cbb7d0f0`, 1000x1500 RGB JPEG
  - Photograph date: 2010-03-11. Licence: CC BY 3.0 (https://creativecommons.org/licenses/by/3.0).
  - Credit: Marie-Lan Nguyen / Wikimedia Commons, 11 March 2010, CC BY 3.0. Adapted for Spheres; no endorsement implied.
  - In frame: Aubry speaking at the Cirque d'Hiver, Paris, mouth open mid-speech; a red podium sign with 'HUCHON 2010' lettering, two microphones and blurred bystanders are excluded by the prompt, which draws her with lips closed.
- Required output: 1024x1536 RGB PNG, opaque flat dark teal #192D34 background, full body with both hands and shoes visible.
- Pipeline job, window-scoped: `martine_aubry-4f7ab403d012`. Default-inventory job partly closed: `martine_aubry-b1bb5b714480`.
- Leadership-production cartoon jobs this window closes: `cartoon:martine_aubry:2008-11-30:2010-01-01:v1`, `cartoon:martine_aubry:2010-01-01:2011-06-30:v1`, `cartoon:martine_aubry:2011-10-31:2012-09-01:v1`.
- Settle first: Render only if one of the seven primary portraits is excluded at Codex review, or if Codex chooses to render the alternate.

## Return format

One entry per attempt, for example:

```json
{
  "person_id": "michel_rocard",
  "attempt": 1,
  "generator": "OpenAI built-in image_gen",
  "generated_at": "YYYY-MM-DD",
  "prompt_path": "tools/avatars/person-prompts/michel-rocard-cartoon-1990-v1.txt",
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
