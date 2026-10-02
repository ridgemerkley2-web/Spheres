# Saudi Arabia cast batch 1: render request for Codex (SA-CAST-B01)

From Claude, task `CLAUDE-C06-SAUDIARABIA-01`, branch `claude/c06-sa-01`, 2 October 2026. State: **awaiting Codex render**. Nothing here is rendered, reviewed or approved yet.

Identity review: [`identity-review-batch-01.json`](identity-review-batch-01.json) (sha256 `64533730456f558e232c9b3db16dfc2ce5322e0d4028ba5808220212f9901e38`). It holds the accepted C01-45 and C01-50 observations, the verified references, the rejected candidates and the verifier dispositions.

## What Codex does

For each of the four portrait jobs below (two people, two appearance windows each):

1. Check the inputs: the prompt file's sha256 equals the pinned value (`git show HEAD:<prompt path> | sha256sum`), and the style anchor and identity reference match their sha256.
2. Generate with the built-in image tool (the pipeline requires generator `OpenAI built-in image_gen`). Submit the prompt file's text exactly as committed, with the images in this order:
   - image 1: the style anchor `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png` (sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`), **STYLE ONLY**. It supplies outlines, cel shading, proportions, framing and the dark teal background, never the face, headdress, clothes or pose;
   - image 2: that job's identity reference.
3. Required output: 1024x1536 RGB, opaque. Make up to **3 attempts per portrait**.
4. Return every attempt **unchanged**: the `exec-*.png` files exactly as the tool wrote them, each with its path, sha256, width, height and mode, the prompt sha256 actually submitted, the input order used, the generation date and any refusal or failure. Do not crop, resize, recolour, re-encode or otherwise edit them. You may say which attempt you would choose.
5. Claude then reviews identity, likeness, era and image quality, names the reviewer, copies the chosen file byte-identical to the output path, and writes the batch generation record, the `person_portraits.json` records and the registration receipt. Review flags stay false until those checks actually happen. No human approval is claimed.

All four identity references are multi-person or cropped US federal photographs. Each prompt names the one man to draw and excludes everyone else; please check at review that only that man appears.

Line endings: the four prompt files are LF and no `.gitattributes` rule covers them yet (core.autocrlf=true on this machine). This task may not edit `.gitattributes`, so please add a rule at integration, as for `tonga-*.txt`, for example `tools/avatars/person-prompts/fahd-bin-abdulaziz-cartoon-*.txt text eol=lf` and `tools/avatars/person-prompts/abdullah-bin-abdulaziz-cartoon-*.txt text eol=lf`. Until then a Windows checkout writes CRLF; its hash is listed for each job.

## Decisions needed from Codex

- `fahd_bin_abdulaziz_al_saud` 1991: Agree or amend the window 1991-01-01 to 1996-01-01. No accepted observation lies inside it; it continues the existing 1990 art and ends at the accepted 1 January 1996 delegation decree, used only as an appearance boundary.
- `fahd_bin_abdulaziz_al_saud`: Keep the split at 1996-01-01, or rule one window 1991-01-01 to 2005-08-01 (`fahd_bin_abdulaziz_al_saud-f17eedfe6634`). One window would need a v2 of whichever prompt is kept, so please rule before rendering Fahd.
- `fahd_bin_abdulaziz_al_saud` 1991: Optionally prefer the PDM 1.0 URL over `Template:PD-USGov` as `license_url` at registration; the bytes are unaffected.
- `abdullah_bin_abdulaziz_al_saud` 1996: Keep the start 1996-01-01, or move it to 1990-01-01 on the `leaders_1990.json` heir record (`abdullah_bin_abdulaziz_al_saud-0ae5f49261bc`). A 1990 start needs a v2 prompt (window sentence and age range), so please rule before rendering this job.
- `abdullah_bin_abdulaziz_al_saud` 2005: Accept or amend keeping the January 2007 appearance (about 82) for the whole window to January 2015 (about 90), with no further ageing.
- Batch 2: registry identities for the accepted holders without a person ID, Salman bin Abdulaziz and Mohammed bin Salman first.

## Portrait jobs

### 1. Fahd bin Abdulaziz Al Saud, 1991 window (`fahd_bin_abdulaziz_al_saud`)

- Appearance window: 1991-01-01 to 1996-01-01 (exclusive end); status `needs_codex_agreement`.
- Output path (after review): `spheres-web/ui/person-portraits/fahd-bin-abdulaziz-cartoon-1991-v1.png`
- Prompt file: `tools/avatars/person-prompts/fahd-bin-abdulaziz-cartoon-1991-v1.txt`
  - sha256 (LF, as committed): `b6c2539c63ac0d31bdcb9b46e8c19a828e72e350117d140b49e1b865709828eb`
  - sha256 if your checkout wrote CRLF: `1c7bb088db42d2f75cf6e15a0c3b5a11bdf5fe481ec43b6c6aa41ca1481005bb`; git blob `4cdf248ab645b9a84936c640dcf52131544fbe92`
- Input 1, STYLE ONLY: `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png`, sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`
- Input 2, identity reference: `spheres-web/ui/person-portraits/references/fahd-bin-abdulaziz-1992-reference-v1.jpg`, sha256 `5980e5254377d826244ab965e1b650c5070a20353fb8ea6f1af8a3ea67e77fa4`, 600x410 RGB JPEG
  - Photograph date: 1992-12-31. Licence: Public domain (https://commons.wikimedia.org/wiki/Template:PD-USGov).
  - Credit: George H. W. Bush Presidential Library and Museum, official White House photograph P38849-17, 'President George H. W. Bush meets with Saudi Arabian King Fahd in Riyadh, Saudi Arabia', 31 December 1992; public domain (PD-USGov), via Wikimedia Commons. Adapted for Spheres; no endorsement implied.
  - In frame: Three seated men in a reception room with carved wooden doors, a Saudi flag behind the left chair and a flower arrangement on a small table. LEFT: President George H. W. Bush in a dark suit. MIDDLE: an unidentified man in white ghutra, black agal and light bisht leaning toward Bush. RIGHT: King Fahd, facing the camera and smiling, hands clasped on his lap; the frame cuts off below his knees, so his feet are not visible. The prompt selects Fahd alone and excludes the others, the flag, furniture and setting.
- Required output: 1024x1536 RGB PNG, opaque flat dark teal #192D34 background, full body with both hands and shoes visible.
- Pipeline job, window-scoped: `fahd_bin_abdulaziz_al_saud-912f7750444a`. Default-inventory job partly closed: `fahd_bin_abdulaziz_al_saud-b847f208ede0`.
- Leadership-production cartoon jobs: none exist for this person; leadership production marks Fahd `known_windows_covered` (his only required window, 1990-01-01 to 1990-01-02, is covered by the 1990 art).
- Settle first: Agree or amend the window 1991-01-01 to 1996-01-01 (no accepted observation inside; it continues the 1990 art and ends at the accepted delegation decree, used as an appearance boundary). Keep the split at 1996-01-01 or rule one window 1991-01-01 to 2005-08-01 (fahd_bin_abdulaziz_al_saud-f17eedfe6634); one window would need a v2 of whichever prompt is kept. Optionally prefer the PDM 1.0 URL over Template:PD-USGov as license_url at registration; the bytes are unaffected.

### 2. Fahd bin Abdulaziz Al Saud, 1996 window (`fahd_bin_abdulaziz_al_saud`)

- Appearance window: 1996-01-01 to 2005-08-01 (exclusive end); status `appearance_interval_from_handoff`.
- Output path (after review): `spheres-web/ui/person-portraits/fahd-bin-abdulaziz-cartoon-1996-v1.png`
- Prompt file: `tools/avatars/person-prompts/fahd-bin-abdulaziz-cartoon-1996-v1.txt`
  - sha256 (LF, as committed): `bedda1e071b620c748b17f2e9e23868abdf4d709ab61bede163800a91fb8be86`
  - sha256 if your checkout wrote CRLF: `46e06c2dc04234b8e3776a1cc90b1701697e5dc8e50dbd1d3ed1adb68d9702a1`; git blob `519c72e9bb85030dee60e50a7aa3f41b7d631906`
- Input 1, STYLE ONLY: `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png`, sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`
- Input 2, identity reference: `spheres-web/ui/person-portraits/references/fahd-bin-abdulaziz-1998-reference-v1.jpg`, sha256 `5d55840a9e9f72bd0c9e85a86c2ce6b42070662fbb369d5bf057544b9bc1bf6a`, 2496x1821 RGB JPEG
  - Photograph date: 1998-10-13. Licence: Public domain (https://commons.wikimedia.org/wiki/Template:PD-USGov-Military).
  - Credit: Helene C. Stikkel, U.S. Department of Defense photo 981013-D-2987S-196 (Released), 13 October 1998, Al-Yamamah Palace, Riyadh; public domain (PD-USGov-Military), via Wikimedia Commons. Adapted for Spheres; no endorsement implied.
  - In frame: Fahd is the elderly man seated in the gilded armchair at RIGHT: white ghutra with black agal, white thobe, sheer black bisht with broad gold bands, gold wristwatch with dark strap on left wrist, ring on right hand, walking cane in his right hand, black shoes; the frame shows him head to shoes. William S. Cohen (dark suit) sits on the sofa at left; an unidentified Saudi official in a black gold-edged bisht sits in the middle behind a small table. A Saudi flag, gilded panels, flowers and carpet are in frame. Identity rests on the Commons description and the embedded IPTC caption 'Cohen (left) meets with King Fahd ... (right)'.
- Required output: 1024x1536 RGB PNG, opaque flat dark teal #192D34 background, full body with both hands and shoes visible.
- Pipeline job, window-scoped: `fahd_bin_abdulaziz_al_saud-23affa4e0bcb`. Default-inventory job partly closed: `fahd_bin_abdulaziz_al_saud-b847f208ede0`.
- Leadership-production cartoon jobs: none exist for this person; leadership production marks Fahd `known_windows_covered` (his only required window, 1990-01-01 to 1990-01-02, is covered by the 1990 art).
- Settle first: Keep the split at 1996-01-01 or rule one Fahd window (see the 1991 entry).

### 3. Abdullah bin Abdulaziz Al Saud, 1996 window (`abdullah_bin_abdulaziz_al_saud`)

- Appearance window: 1996-01-01 to 2005-08-01 (exclusive end); status `appearance_interval_from_handoff`.
- Output path (after review): `spheres-web/ui/person-portraits/abdullah-bin-abdulaziz-cartoon-1996-v1.png`
- Prompt file: `tools/avatars/person-prompts/abdullah-bin-abdulaziz-cartoon-1996-v1.txt`
  - sha256 (LF, as committed): `c58709f80c63f395148cca942143fa8bdb98affe15df3a263d224d007b2f6084`
  - sha256 if your checkout wrote CRLF: `f69770b99422234f19ad872f4385af9dc265278e361701f8b5971926cc75f1c0`; git blob `4ee89a8473bbe3be2fdf7458fffdc450a6b0d755`
- Input 1, STYLE ONLY: `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png`, sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`
- Input 2, identity reference: `spheres-web/ui/person-portraits/references/abdullah-bin-abdulaziz-1998-reference-v1.jpg`, sha256 `243d6325a0c36e6e949a64985fadb9c89de87e2686a64fdd9d916b2a624cac64`, 1850x2990 RGB JPEG
  - Photograph date: 1998-11-03. Licence: Public domain (https://commons.wikimedia.org/wiki/Template:PD-USGov-Military).
  - Credit: R. D. Ward / U.S. Department of Defense, Defense.gov News Photo 981103-D-9880W-172, Riyadh, 3 November 1998; public domain (work of a U.S. military or Department of Defense employee, PD-USGov-Military) via Wikimedia Commons. Adapted for Spheres; no endorsement implied.
  - In frame: Full-length colour photograph. Abdullah is the tall man at centre right in a plain white ghutra with a double black agal, thin gold-toned metal-rimmed glasses, a black moustache and a separate black chin beard, a white thobe and a very dark brown bisht with gold edging. His left hand gathers the bisht at his waist and his right hangs at his side. William Cohen is at left (dark suit, red tie). Also in frame: a US official in glasses and a suit behind Cohen, a Saudi officer in a green beret, a uniformed officer at the left edge, and several Saudi men in red-and-white checked headcloths, including a bespectacled, goateed man at the far right. Palace hall with gilded walls, columns, chandeliers and a marble floor. The prompt excludes all of these.
- Required output: 1024x1536 RGB PNG, opaque flat dark teal #192D34 background, full body with both hands and shoes visible.
- Pipeline job, window-scoped: `abdullah_bin_abdulaziz_al_saud-236612642866`. Default-inventory job partly closed: `abdullah_bin_abdulaziz_al_saud-c6fae3497c05`.
- Leadership-production cartoon jobs: none exist for this person; leadership production marks Abdullah `eligibility_research_required` with no required window.
- Settle first: Keep the start 1996-01-01 or move it to 1990-01-01 (abdullah_bin_abdulaziz_al_saud-0ae5f49261bc) on the seed heir record; a 1990 start needs a v2 prompt.

### 4. Abdullah bin Abdulaziz Al Saud, 2005 window (`abdullah_bin_abdulaziz_al_saud`)

- Appearance window: 2005-08-01 to 2015-01-23 (exclusive end); status `appearance_interval_from_handoff`.
- Output path (after review): `spheres-web/ui/person-portraits/abdullah-bin-abdulaziz-cartoon-2005-v1.png`
- Prompt file: `tools/avatars/person-prompts/abdullah-bin-abdulaziz-cartoon-2005-v1.txt`
  - sha256 (LF, as committed): `9a2c0e346a3291f7a34b871bb194b687fe13f1229a4168f3c3dad3d79ddc6614`
  - sha256 if your checkout wrote CRLF: `4fb95dbc158f53d18069c9467980d7b9133264b7619b4abac8f3b89473b9a46a`; git blob `1c20f1494d207b82ca60e58a840f3894356654b0`
- Input 1, STYLE ONLY: `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png`, sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`
- Input 2, identity reference: `spheres-web/ui/person-portraits/references/abdullah-bin-abdulaziz-2007-reference-v1.jpg`, sha256 `90fb0896a31cb8b44e38af7298f20637125927ce6161de79545706806421ae82`, 648x848 RGB JPEG
  - Photograph date: 2007-01-17. Licence: Public domain (https://commons.wikimedia.org/wiki/Template:PD-USGov-Military).
  - Credit: Cherie A. Thurlby / U.S. Department of Defense, DoD photo 070117-D-7203T-016, 17 January 2007; public domain U.S. military photograph; Commons crop 'King Abdullah bin Abdul al-Saud January 2007.jpg' via Wikimedia Commons. Adapted for Spheres; no endorsement implied.
  - In frame: Abdullah alone, head and shoulders, glancing slightly to his left, in front of pale cream curtains. Gates, named in the caption as being at left, is outside the crop.
- Required output: 1024x1536 RGB PNG, opaque flat dark teal #192D34 background, full body with both hands and shoes visible.
- Pipeline job, window-scoped: `abdullah_bin_abdulaziz_al_saud-a0b3ce91e5c0`. Default-inventory job partly closed: `abdullah_bin_abdulaziz_al_saud-c6fae3497c05`.
- Leadership-production cartoon jobs: none exist for this person; leadership production marks Abdullah `eligibility_research_required` with no required window.
- Settle first: Accept or amend keeping the 2007 appearance (about 82) for the whole window to January 2015 (about 90), with no further ageing.

## Return format

One entry per attempt, for example:

```json
{
  "person_id": "abdullah_bin_abdulaziz_al_saud",
  "attempt": 1,
  "generator": "OpenAI built-in image_gen",
  "generated_at": "YYYY-MM-DD",
  "prompt_path": "tools/avatars/person-prompts/abdullah-bin-abdulaziz-cartoon-2005-v1.txt",
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
