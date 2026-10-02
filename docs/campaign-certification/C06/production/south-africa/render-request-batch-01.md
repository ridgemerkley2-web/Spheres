# South Africa cast batch 1: render request for Codex (ZA-CAST-B01)

From Claude, task `CLAUDE-C06-SOUTHAFRICA-01`, branch `claude/c06-za-01`, 1 October 2026. State: **awaiting Codex render**. Nothing here is rendered, reviewed or approved yet.

Identity review: [`identity-review-batch-01.json`](identity-review-batch-01.json) (sha256 over LF bytes `7c4cd24028f05249fc3e834130abe5cabd0d6c0bda7392091dee45d8f8a821a9`). It holds the accepted C01-30 observations, the verified references, the rejected candidates, the verifier dispositions and the four windows that are not ready, plus Codex's 2 October preparation review of submitted tip `c0382bbcb92852ff6e885292bb3ecfe9a7793a75`. Generated-art approval remains pending.

## What Codex does

For each of the 4 jobs below:

1. Check the inputs: the prompt file's sha256 equals the pinned value (`git show HEAD:<prompt path> | sha256sum`), and the style anchor and identity reference match their sha256.
2. Generate with the built-in image tool (the pipeline requires generator `OpenAI built-in image_gen`). Submit the prompt file's text exactly as committed, with the images in this order:
   - image 1: the style anchor `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png` (sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`), **STYLE ONLY**. It supplies outlines, cel shading, proportions, framing and the dark teal background, never the face, hair, clothes or pose;
   - image 2: that job's identity reference.
3. Required output: 1024x1536 RGB, opaque. Make up to **3 attempts per portrait**.
4. Return every attempt **unchanged**: the `exec-*.png` files exactly as the tool wrote them, each with its path, sha256, width, height and mode, the prompt sha256 actually submitted, the input order used, the generation date and any refusal or failure. Do not crop, resize, recolour, re-encode or otherwise edit them. You may say which attempt you would choose.
5. Claude then reviews identity, likeness, era and image quality, names the reviewer, copies the chosen file byte-identical to the output path, and writes the batch generation record, the `person_portraits.json` records and the registration receipt. Review flags stay false until those checks actually happen. No human approval is claimed.

The job lists below describe projected coverage after those steps; all 24 leadership-production cartoon jobs remain open during preparation. For Viljoen, registration must explicitly set `derivative_license` to `CC BY-SA 2.0` and `derivative_license_url` to `https://creativecommons.org/licenses/by-sa/2.0/`, retain Ian Barbour attribution and identify the cartoon adaptation. A `generated` label plus the source licence alone is insufficient.

Line endings: the four prompt files are LF and no `.gitattributes` rule covers them yet (core.autocrlf=true on this machine). This task may not edit `.gitattributes`, so please add a rule at integration, as for `tonga-*.txt`, for example `tools/avatars/person-prompts/<each South Africa prompt>.txt text eol=lf`. Until then a Windows checkout writes CRLF; its hash is listed for each job.

## Decisions needed from Codex

- `mangosuthu_buthelezi`: Render with the 1983 Anefo reference and its declared 11.6 to 21.6 year gap, or first request a single-frame scan of a 20 June 1991 frame (rolls WHPO-P22804 to P22807, P22809 or P22810) or a 28 February 1990 frame (P10615, P10616) from the George H. W. Bush Library (US federal work, public domain, about 3.5 years before the window), which would replace this reference as v2 with a new prompt. Confirm the assembly swap of the reference bytes to the Commons original (same frame, licence and prompt).
- `constand_viljoen`: Accept the small (383x388), month-dated 1984 crop in uniform with its declared 9.5 to 16 year gap and civilian re-dressing.
- `pieter_mulder`: Agree the window start 2001-06-21 (first accepted observation), earlier than leadership production's 2001-12-31. Decide whether to split the 15.4-year window (for example at 2010-01-01) if a qualifying 2001-2009 photograph turns up.
- `corne_mulder`: Accept the CropTool derivative crop (no FlickrReview of its own; parent reviewed CC BY 2.0).
- Not-ready windows (Buthelezi 2005-2019, Hlabisa 2019-2026, Meshoe 1993-2010 and 2010-2026): choose per window between accepting a declared fallback, leaving it open until a qualifying photograph is found, or a reserve (`pieter_groenewald` or `mzwanele_nyhontso`, each needing agreement on its window first). See the not-ready section and `excluded_people` in the identity review.
- `kenneth_meshoe`: rule whether a CC BY video frame may ever serve as an identity reference (this batch's rules say no).

## Jobs

### 1. Mangosuthu Buthelezi (`mangosuthu_buthelezi`)

- Appearance window: 1995-01-01 to 2005-01-01 (exclusive end); status `appearance_interval_from_handoff`.
- Output path (after review): `spheres-web/ui/person-portraits/mangosuthu-buthelezi-cartoon-1995-v1.png`
- Prompt file: `tools/avatars/person-prompts/mangosuthu-buthelezi-cartoon-1995-v1.txt`
  - sha256 (LF, as committed): `d066930ed4fac85b0a1089e92c2ae6e471341e3407b1d421d9e9ed57964479c6`
  - sha256 if your checkout wrote CRLF: `30caea89743a656bf4cbf2924137ab245f9840e0759c55bc5cc3e87c28df9759`; git blob `edb5856cfe81222fda67fff7fac3fe393e8fce60`
- Input 1, STYLE ONLY: `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png`, sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`
- Input 2, identity reference: `spheres-web/ui/person-portraits/references/mangosuthu-buthelezi-1983-reference-v1.jpg`, sha256 `91523be5d17fca2244286cc138ad23fe125b338d477d04a3535b47ffa6389fe0`, 2467x3685 RGB JPEG
  - Photograph date: 1983-06-10. Licence: CC0 (http://creativecommons.org/publicdomain/zero/1.0/deed.en).
  - Credit: Rob Bogaerts / Anefo; Nationaal Archief, Fotocollectie Anefo (archive 2.24.01.05), item 932-6173, 10 June 1983 (Chief Mangosuthu Gatsha Buthelezi arriving at Schiphol); CC0; via Wikimedia Commons. Adapted for Spheres; no endorsement implied.
  - In frame: Buthelezi alone: head-and-chest portrait, smiling at the camera, in a dark suit, white shirt, diagonal-striped tie and patterned pocket square. Behind him is a dark, blurred airport hall with bright windows, which the prompt excludes.
- Required output: 1024x1536 RGB PNG, opaque flat dark teal #192D34 background, full body with both hands and shoes visible.
- Pipeline job, window-scoped: `mangosuthu_buthelezi-355a6013bb99`. Default-inventory job with projected partial coverage after rendering, review and registration: `mangosuthu_buthelezi-571ede3793fd`.
- Projected leadership-production cartoon job coverage after rendering, review and registration: `cartoon:mangosuthu_buthelezi:1995-01-01:2000-01-01:v1`, `cartoon:mangosuthu_buthelezi:2000-01-01:2005-01-01:v1`.
- Settle first: Render with the 1983 Anefo reference and its declared 11.6 to 21.6 year gap, or first request a single-frame scan of a 20 June 1991 frame (rolls WHPO-P22804 to P22807, P22809 or P22810) or a 28 February 1990 frame (P10615, P10616) from the George H. W. Bush Library (US federal work, public domain, about 3.5 years before the window), which would replace this reference as v2 with a new prompt. Confirm the assembly swap of the reference bytes to the Commons original (same frame, licence and prompt).

### 2. Constand Viljoen (`constand_viljoen`)

- Appearance window: 1994-03-31 to 2001-01-01 (exclusive end); status `appearance_interval_from_handoff`.
- Output path (after review): `spheres-web/ui/person-portraits/constand-viljoen-cartoon-1994-v1.png`
- Prompt file: `tools/avatars/person-prompts/constand-viljoen-cartoon-1994-v1.txt`
  - sha256 (LF, as committed): `79f58149429784f37c9ccef47b98b6c8da4bcd3ddcdfa16cd8a9ecbdb1c5bb24`
  - sha256 if your checkout wrote CRLF: `030a41b15471bbafa7f2eeab29a2e26705b1c6c43619bb5af6c1f20e41b7db78`; git blob `aa1d065b7003aebef5e9da5a0bd2b8593e927c8e`
- Input 1, STYLE ONLY: `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png`, sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`
- Input 2, identity reference: `spheres-web/ui/person-portraits/references/constand-viljoen-1984-reference-v1.jpg`, sha256 `36245e927a9193ba27f980875dd4718ef72ab7ebd20d6abb3c175b0a7775d3e4`, 383x388 RGB JPEG
  - Photograph date: 1984-09. Licence: CC BY-SA 2.0 (https://creativecommons.org/licenses/by-sa/2.0).
  - Credit: Ian Barbour (Flickr: barbourians), September 1984, CC BY-SA 2.0, via Wikimedia Commons: 'Constand Viljoen 1984.jpg', a crop by Commons user TDKR Chicago 101 (CropTool, 13 September 2020) of '1984 PW Botha inspects the guard of honour.jpg' (Flickr 5971221747, uploaded by Flickr upload bot for Gbawden on 19 September 2013 and confirmed CC BY-SA 2.0 on that date). Adapted for Spheres; no endorsement implied.
  - In frame: A tight head-and-collar crop in which Viljoen's is the only complete face, seen in three-quarter view with eyes cast down, wearing a general's peaked cap with red band and cap badge. At the top-left and left edge are the out-of-focus partial face (cheek and ear) and gold-corded shoulder of a guardsman; at the right edge a partly visible figure, mostly pale with one dark green band; foliage behind. All of these are excluded by the prompt. The verifier found the partial face, which the preparer's record omitted; the assembler confirmed it on the reference. Identity was cross-checked by the preparer against the uncropped parent and the sibling frontal frame 5971221735.
- Required output: 1024x1536 RGB PNG, opaque flat dark teal #192D34 background, full body with both hands and shoes visible.
- Pipeline job, window-scoped: `constand_viljoen-3725bb6de879`. Default-inventory job with projected partial coverage after rendering, review and registration: `constand_viljoen-f4174a5400ed`.
- Projected leadership-production cartoon job coverage after rendering, review and registration: `cartoon:constand_viljoen:1994-03-31:1995-01-01:v1`, `cartoon:constand_viljoen:1995-01-01:2000-01-01:v1`, `cartoon:constand_viljoen:2000-01-01:2001-01-01:v1`.
- Settle first: Accept the small (383x388), month-dated 1984 crop in uniform with its declared 9.5 to 16 year gap and civilian re-dressing.

### 3. Pieter Mulder (`pieter_mulder`)

- Appearance window: 2001-06-21 to 2016-11-13 (exclusive end); status `appearance_interval_from_handoff`.
- Output path (after review): `spheres-web/ui/person-portraits/pieter-mulder-cartoon-2001-v1.png`
- Prompt file: `tools/avatars/person-prompts/pieter-mulder-cartoon-2001-v1.txt`
  - sha256 (LF, as committed): `3bf5004a8f37214eb77cb5f1d192c75c2ba0653f5f994474b8f230a434b32b1d`
  - sha256 if your checkout wrote CRLF: `25c183ba88162c3261e8dee004618a3a1e5b8e842138a1641fb6a7c21d751a12`; git blob `670c949f1ac5f23c322bfd73c4b59e1f551f67f7`
- Input 1, STYLE ONLY: `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png`, sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`
- Input 2, identity reference: `spheres-web/ui/person-portraits/references/pieter-mulder-2013-reference-v1.jpg`, sha256 `697749a02f43cb1cef5928479738640a12aa4cf4741d8f6ef6bb92139231d59f`, 3000x2000 RGB JPEG
  - Photograph date: 2013-09-16. Licence: CC BY 2.0 (https://creativecommons.org/licenses/by/2.0).
  - Credit: U.S. Department of Agriculture, photo by Blake Woodhams, '20130916-OSEC-BW-0002' (Flickr 9785284212), Johannesburg, 16 September 2013; CC BY 2.0; via Wikimedia Commons. Adapted for Spheres; no endorsement implied.
  - In frame: Mulder alone, waist-up, standing behind a wooden lectern and speaking into a microphone, holding papers; two water bottles and a glass on the lectern; gold damask wallpaper and a striped curtain behind; warm orange stage lighting. All of these are excluded by the prompt.
- Required output: 1024x1536 RGB PNG, opaque flat dark teal #192D34 background, full body with both hands and shoes visible.
- Pipeline job, window-scoped: `pieter_mulder-a245f929c659`. Default-inventory job with projected partial coverage after rendering, review and registration: `pieter_mulder-dde556c97c84`.
- Projected leadership-production cartoon job coverage after rendering, review and registration: `cartoon:pieter_mulder:2001-12-31:2005-01-01:v1`, `cartoon:pieter_mulder:2005-01-01:2010-01-01:v1`, `cartoon:pieter_mulder:2010-01-01:2015-01-01:v1`, `cartoon:pieter_mulder:2015-01-01:2016-11-12:v1`.
- Settle first: Agree the window start 2001-06-21 (first accepted observation), earlier than leadership production's 2001-12-31. Decide whether to split the 15.4-year window (for example at 2010-01-01) if a qualifying 2001-2009 photograph turns up.

### 4. Corné Mulder (`corne_mulder`)

- Appearance window: 2025-07-16 to 2026-09-08 (exclusive end); status `appearance_interval_from_handoff`.
- Output path (after review): `spheres-web/ui/person-portraits/corne-mulder-cartoon-2025-v1.png`
- Prompt file: `tools/avatars/person-prompts/corne-mulder-cartoon-2025-v1.txt`
  - sha256 (LF, as committed): `d0d1db4577c23590c2965a2c580f5e43eb8283a4141ad85e2c1d90bfddf8c018`
  - sha256 if your checkout wrote CRLF: `ed22a865652872374cdfb4ff1002a21c740638394e2c82068504fc6b44284ee9`; git blob `d225aa99e9b03f7163e8ede18ef2afb891b104a0`
- Input 1, STYLE ONLY: `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png`, sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`
- Input 2, identity reference: `spheres-web/ui/person-portraits/references/corne-mulder-2023-reference-v1.jpg`, sha256 `5c2e7e0caa54ba0e500b4127528663d3b53b6e55c4f4013a0df5682f90fb57e8`, 301x425 RGB JPEG
  - Photograph date: 2023-06-14. Licence: CC BY 2.0 (https://creativecommons.org/licenses/by/2.0).
  - Credit: Houses of the Oireachtas, 'Corne Mulder.jpg', taken 14 June 2023 at Leinster House, Dublin; CC BY 2.0, via Wikimedia Commons. Commons crop (CropTool, user Lefcentreright, 17 November 2024, two steps) of 'Pemmy Majodina visits Ireland.jpg' (Flickr 52974699940, Houses of the Oireachtas stream). Adapted for Spheres; no endorsement implied.
  - In frame: Mulder alone, head and shoulders, outdoors in strong sun against the grey stone wall and a column of Leinster House. The parent is a group photograph of the South African whips' delegation and Irish hosts. The lighter grey area at the lower right is the sunlit side of his own dark navy jacket.
- Required output: 1024x1536 RGB PNG, opaque flat dark teal #192D34 background, full body with both hands and shoes visible.
- Pipeline job, window-scoped: `corne_mulder-e523c7f55fc0`. Default-inventory job with projected partial coverage after rendering, review and registration: `corne_mulder-4a324fa308a8`.
- Projected leadership-production cartoon job coverage after rendering, review and registration: `cartoon:corne_mulder:2025-07-16:2026-09-08:v1`.
- Settle first: Accept the CropTool derivative crop (no FlickrReview of its own; parent reviewed CC BY 2.0).

## Not ready (do not render)

No dated photograph of the exact person with a licence string from `FREE_LICENSES` was found for these windows. No reference was copied and no prompt was written; every job below stays open.

| person_id | window (exclusive end) | why not ready | window-scoped job | leadership-production jobs left open |
|---|---|---|---|---|
| `mangosuthu_buthelezi` | 2005-01-01 to 2019-08-25 | closest qualifying photographs are from 1983 (age 54 against 76 to 90); in-window images are a TV still, 'Attribution' (GWOIA) and ND files | `mangosuthu_buthelezi-375dbc8d11f3` | `cartoon:mangosuthu_buthelezi:2005-01-01:2010-01-01:v1`, `cartoon:mangosuthu_buthelezi:2010-01-01:2015-01-01:v1`, `cartoon:mangosuthu_buthelezi:2015-01-01:2019-08-24:v1` |
| `velenkosini_hlabisa` | 2019-08-24 to 2026-09-08 | Commons has only an eNCA video still; all Flickr photographs are ND, NC-ND or all rights reserved | `velenkosini_hlabisa-5f182018a18c` | `cartoon:velenkosini_hlabisa:2019-08-24:2020-01-01:v1`, `cartoon:velenkosini_hlabisa:2020-01-01:2025-01-01:v1`, `cartoon:velenkosini_hlabisa:2025-01-01:2026-09-08:v1` |
| `kenneth_meshoe` | 1993-12-31 to 2010-01-01 | Commons has only video stills (2013/2014, 2019); Flickr photographs are ND or all rights reserved; a Clinton Library March 1998 lead is unexplored | `kenneth_meshoe-de5779eb28ae` | `cartoon:kenneth_meshoe:1993-12-31:1995-01-01:v1`, `cartoon:kenneth_meshoe:1995-01-01:2000-01-01:v1`, `cartoon:kenneth_meshoe:2000-01-01:2005-01-01:v1`, `cartoon:kenneth_meshoe:2005-01-01:2010-01-01:v1` |
| `kenneth_meshoe` | 2010-01-01 to 2026-09-08 | only CC BY video frames (2013/2014, 2019) and a CC BY-ND 2.0 GovernmentZA photograph (2025) | `kenneth_meshoe-a800a182982b` | `cartoon:kenneth_meshoe:2010-01-01:2015-01-01:v1`, `cartoon:kenneth_meshoe:2015-01-01:2020-01-01:v1`, `cartoon:kenneth_meshoe:2020-01-01:2025-01-01:v1`, `cartoon:kenneth_meshoe:2025-01-01:2026-09-08:v1` |

## Reserves (not prepared)

Neither reserve was prepared in this task. Each needs Codex agreement on its window first, because neither has a registry term.

- `pieter_groenewald`: 2016-12-27 to 2025-07-16; C01-30 FF Plus Leader attested 2016-12-27, 2020-04-02, 2024-08-22 (/organizations/50/roles/0/holder_claims/4-6). No registry term: the registry declares an FF gap 2016-11-12 to 2025-07-16. First accepted observation to Corné Mulder's first, used only as an appearance boundary. Jobs: `pieter_groenewald-d4a731ce349a` (window), `pieter_groenewald-04ce91397002` (default); no leadership-production jobs. Photo lead: 'Pieter Groenewald.jpg', Commons upload dated 2018-11-14, CC BY-SA 4.0 (planner's Commons API lead; unverified).
- `mzwanele_nyhontso`: 2020-02-15 to 2026-09-08; C01-32 PAC President attested 2020-02-15, 2021-07-19, 2021-12-31, 2026-08-29; the research records the leadership as disputed. No registry term (registry PAC gap from 1996). First accepted observation to the cutoff. Jobs: `mzwanele_nyhontso-d8537325291e` (window), `mzwanele_nyhontso-c2c1d5472e9d` (default); no leadership-production jobs. Photo lead: none; the planner found no Commons photograph.

## Return format

One entry per attempt, for example:

```json
{
  "person_id": "mangosuthu_buthelezi",
  "attempt": 1,
  "generator": "OpenAI built-in image_gen",
  "generated_at": "YYYY-MM-DD",
  "prompt_path": "tools/avatars/person-prompts/mangosuthu-buthelezi-cartoon-1995-v1.txt",
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
