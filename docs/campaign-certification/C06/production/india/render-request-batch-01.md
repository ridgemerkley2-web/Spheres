# India cast batch 1: render request for Codex (IN-CAST-B01)

From Claude, task `CLAUDE-C06-INDIA-01`, branch `claude/c06-in-01`, 2 October 2026. State: **awaiting Codex render**. Nothing here is rendered, reviewed or approved yet.

Identity review: [`identity-review-batch-01.json`](identity-review-batch-01.json) (sha256 `57d1d9ea673312f9937a4a3355c56a180a0c6edf654ea8de22f8904bfd2e8c9c`). It holds the accepted C01-27 and C01-40 observations, the verified references, the rejected candidates, the exclusions and the verifier dispositions.

## What Codex does

For each of the 8 jobs below (seven primary people and the promoted reserve M. A. Baby):

1. Check the inputs: the prompt file's sha256 equals the pinned value (`git show HEAD:<prompt path> | sha256sum`), and the style anchor and identity reference match their sha256.
2. Generate with the built-in image tool (the pipeline requires generator `OpenAI built-in image_gen`). Submit the prompt file's text exactly as committed, with the images in this order:
   - image 1: the style anchor `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png` (sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`), **STYLE ONLY**. It supplies outlines, cel shading, proportions, framing and the dark teal background, never the face, hair, clothes or pose;
   - image 2: that job's identity reference, passed as its unchanged bytes (no conversion, crop or re-encode; the Nadda reference embeds an Adobe RGB (1998) ICC profile).
3. Required output: 1024x1536 RGB, opaque. Make up to **3 attempts per portrait**.
4. Return every attempt **unchanged**: the `exec-*.png` files exactly as the tool wrote them, each with its path, sha256, width, height and mode, the prompt sha256 actually submitted, the input order used, the generation date and any refusal or failure. Do not crop, resize, recolour, re-encode or otherwise edit them. You may say which attempt you would choose.
5. Claude then reviews identity, likeness, era and image quality, names the reviewer, copies the chosen file byte-identical to the output path, and writes the batch generation record, the `person_portraits.json` records and the registration receipt. Review flags stay false until those checks actually happen. No human approval is claimed.

Line endings: the 8 prompt files are LF and no `.gitattributes` rule covers them yet (core.autocrlf=true on this machine). This task may not edit `.gitattributes`, so please add a rule at integration, as for `tonga-*.txt`, for example `tools/avatars/person-prompts/<each India prompt>.txt text eol=lf`. Until then a Windows checkout writes CRLF; its hash is listed for each job.

## Decisions needed from Codex

- `harkishan_singh_surjeet`: Agree the window bounds 1992-01-31 to 2005-04-01 (registry-term bounds, not accepted research).
- `harkishan_singh_surjeet`: Decide whether to split at 2000-01-01; the same reference can serve both halves with an age-adjusted v2 prompt.
- `lal_krishna_advani`: Decide whether to split into 1995-01-01 to 1998-01-01 and 2004-10-20 to 2005-12-31 (one portrait spans 1998-2004, appearance only).
- `lal_krishna_advani`: Costume continuity with his existing 1990-1995 art (cream kurta, white dhoti, sandals) versus the reference's slate-blue bandhgala: accept, or request a v2 prompt.
- `prakash_karat`: Rule on the undated reference (camera stamp impossible; taken between 2013-03-21 and 2017-09-17) and the '2013' in its reference_id and filename; bytes and sha256 stay identical either way. If unacceptable, switch to the in-window alternate File:Prakash Karat CPIM.jpg (Suthir, EXIF 29 May 2014, CC BY-SA 3.0; soft and mid-speech), which would need a new prompt.
- `rajnath_singh`: Decide whether to split at 2009-12-19 / 2013-01-31 (the window spans Gadkari's presidency, appearance only). The reference is small (346x638).
- `nitin_gadkari`: Agree the end widened from 2013-01-01 to 2013-01-23 (the day after the accepted 2013-01-22 observation).
- `sitaram_yechury`: Decide whether to split at 2020-01-01; the same reference fits both halves.
- `jagat_prakash_nadda`: Agree the exclusive end 2026-01-20 (registry and leadership production, not accepted research).
- `jagat_prakash_nadda`: Era dress: the 2018 bandhgala is kept for the whole window (his in-window public dress was often a white kurta-pyjama with a saffron stole); accept or request a v2 prompt.
- `jagat_prakash_nadda`: The reference embeds an Adobe RGB (1998) ICC profile: pass the bytes unchanged and judge colour in a colour-managed view.
- `m_a_baby`: No decision needed: rendered in place of amit_shah, who was excluded at assembly.
- `amit_shah` (excluded) and `nitin_nabin` (reserve, not ready): no render. See "Not rendered" below.

## Jobs

### 1. Harkishan Singh Surjeet (`harkishan_singh_surjeet`)

- Appearance window: 1992-01-31 to 2005-04-01 (exclusive end); status `appearance_interval_from_handoff`.
- Output path (after review): `spheres-web/ui/person-portraits/harkishan-singh-surjeet-cartoon-1992-v1.png`
- Prompt file: `tools/avatars/person-prompts/harkishan-singh-surjeet-cartoon-1992-v1.txt`
  - sha256 (LF, as committed): `418df96663ae4260cdcbb850649c2a10864fb315972ac01309e5a62d9ab2b89e`
  - sha256 if your checkout wrote CRLF: `1b6e96a06084c87100c37694c8fa0d9c3c315b55de59a5eeefd2d6d592566c5d`; git blob `f1c3c31219087d0431983dc64450e0eb5b6648ce`
- Input 1, STYLE ONLY: `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png`, sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`
- Input 2, identity reference: `spheres-web/ui/person-portraits/references/harkishan-singh-surjeet-2005-reference-v1.jpg`, sha256 `f6b457cba512e64003f9780cfb04b6ef92dcacbe4b926056eb6f8090383a87b8`, 1280x960 RGB JPEG, 123,462 bytes
  - Photograph date: 2005-03-28 or 2005-03-29 (camera clock 28 March; the congress banner behind him gives 29 March). Licence: CC BY 2.5 (https://creativecommons.org/licenses/by/2.5).
  - Credit: Soman, 'Bardhanhss.jpg' (Harkishan Singh Surjeet, A. B. Bardhan and D. Raja on the dais of the Communist Party of India's 19th Party Congress, Chandigarh, 28 or 29 March 2005), own work, CC BY 2.5, via Wikimedia Commons. Adapted for Spheres; no endorsement implied.
  - In frame: Surjeet seated at the left, turned three-quarters to his left and speaking with lips parted, framed from the turban to his lap; his left hand rests at his lap. D. Raja (gray hair, glasses, white shirt) leans in behind him with a hand on Surjeet's left shoulder; A. B. Bardhan (white hair, glasses, gray shirt, red CPI delegate badge) sits at the right; a man in a striped shirt and belt stands at the left edge and three men stand behind; a red banner with white lettering ('19th PARTY CONGRESS', '...NIST PARTY ... NDIA', '...H 29th...') and a white star fills the background; a chair edge is at the lower left. The prompt excludes all of them.
- Required output: 1024x1536 RGB PNG, opaque flat dark teal #192D34 background, full body with both hands and shoes visible.
- Pipeline job, window-scoped: `harkishan_singh_surjeet-85010e8ec9dc`. Default-inventory job partly closed: `harkishan_singh_surjeet-69d8afb34e72`.
- Leadership-production cartoon jobs this window closes: `cartoon:harkishan_singh_surjeet:1992-01-31:1995-01-01:v1`, `cartoon:harkishan_singh_surjeet:1995-01-01:2000-01-01:v1`, `cartoon:harkishan_singh_surjeet:2000-01-01:2005-01-01:v1`, `cartoon:harkishan_singh_surjeet:2005-01-01:2005-04-01:v1`.
- Settle first: Agree the window bounds 1992-01-31 to 2005-04-01 (registry-term bounds, not accepted research). Decide whether to split at 2000-01-01; the same reference can serve both halves with an age-adjusted v2 prompt.

### 2. Lal Krishna Advani (`lal_krishna_advani`)

- Appearance window: 1995-01-01 to 2005-12-31 (exclusive end); status `appearance_interval_from_handoff`.
- Output path (after review): `spheres-web/ui/person-portraits/lal-krishna-advani-cartoon-1995-v1.png`
- Prompt file: `tools/avatars/person-prompts/lal-krishna-advani-cartoon-1995-v1.txt`
  - sha256 (LF, as committed): `8faaa5d13c8681739406ca7b3c34dadb83ee84f557071c4b0f6cc23f3f3b1bc0`
  - sha256 if your checkout wrote CRLF: `b4dfbe9979757cb6dd3b5e228407603e4b142b6b228bb428a63a821ebf0b0761`; git blob `3b210eb23d1e922f16c7f2bc75dc6e0e58a6818a`
- Input 1, STYLE ONLY: `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png`, sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`
- Input 2, identity reference: `spheres-web/ui/person-portraits/references/lal-krishna-advani-2001-reference-v1.jpg`, sha256 `dd7ece8740ef8ae489466b2e3440bf5c3514e785a3bce33d55dd28ca8b9163eb`, 1024x765 RGB JPEG, 160,687 bytes
  - Photograph date: 2001-01 (Commons; the photograph itself is from about 29 January to 9 February 2001). Licence: CC BY-SA 3.0 (https://creativecommons.org/licenses/by-sa/3.0).
  - Credit: IDF Spokesperson's Unit, 'IDF Aid Mission to India, January 2001', late January or early February 2001; source: Flickr idfonline 11047435913; licence CC BY-SA 3.0 by VRTS permission ticket 2021051710001034; via Wikimedia Commons. Adapted for Spheres; no endorsement implied.
  - In frame: Advani is the central elderly man in a slate-blue closed-collar bandhgala coat, gesturing with an open right palm; the frame ends just below his hands, before the coat hem. Also in frame and excluded by the prompt: two IDF medical officers at right (one with a stethoscope, one with red-tinted glasses), Indian army officers and a turbaned officer behind him, a woman in a red head-covering at left, a transparent infant incubator and the tent interior.
- Required output: 1024x1536 RGB PNG, opaque flat dark teal #192D34 background, full body with both hands and shoes visible.
- Pipeline job, window-scoped: `lal_krishna_advani-a86e3e748579`. Default-inventory job partly closed: `lal_krishna_advani-49f5deb5285b`.
- Leadership-production cartoon jobs this window closes: `cartoon:lal_krishna_advani:1995-01-01:1998-01-01:v1`, `cartoon:lal_krishna_advani:2004-10-27:2005-01-01:v1`, `cartoon:lal_krishna_advani:2005-01-01:2005-12-31:v1`.
- Settle first: Decide whether to split into 1995-01-01 to 1998-01-01 and 2004-10-20 to 2005-12-31 (one portrait spans 1998-2004, appearance only). Costume continuity with his existing 1990-1995 art (cream kurta, white dhoti, sandals) versus the reference's slate-blue bandhgala: accept, or request a v2 prompt.

### 3. Prakash Karat (`prakash_karat`)

- Appearance window: 2005-04-30 to 2015-04-01 (exclusive end); status `appearance_interval_from_handoff`.
- Output path (after review): `spheres-web/ui/person-portraits/prakash-karat-cartoon-2005-v1.png`
- Prompt file: `tools/avatars/person-prompts/prakash-karat-cartoon-2005-v1.txt`
  - sha256 (LF, as committed): `6929a21fa194c518b3c33da71108ad88e220d76910d9185166315259b900d6e8`
  - sha256 if your checkout wrote CRLF: `459ac42ade0a796ce37ba017b2d5b8080ddebef0d728c4726e7286a7843333bc`; git blob `135aea4ec5baa78107aa58979af6956e453ae660`
- Input 1, STYLE ONLY: `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png`, sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`
- Input 2, identity reference: `spheres-web/ui/person-portraits/references/prakash-karat-2013-reference-v1.jpg`, sha256 `9f3d51f665cac02a360c9c851fc2e2db48034b24959ffd154d5b584b74861e87`, 5184x3456 RGB JPEG, 5,069,190 bytes
  - Photograph date: Undated: between 2013-03-21 and 2017-09-17 (the camera stamp 2013-03-16 19:41:05 is impossible for the camera model). Licence: CC BY-SA 4.0 (https://creativecommons.org/licenses/by-sa/4.0).
  - Credit: Erfanebrahimsait, 'PrakashKarat.jpg' (undated; camera stamp 16 March 2013 unreliable; taken between 2013 and 2017), own work, CC BY-SA 4.0, via Wikimedia Commons. Adapted for Spheres; no endorsement implied.
  - In frame: Karat alone as the subject, waist-up, standing smiling at the open door of a silver car, in a white short-sleeved shirt with a pen in the chest pocket; a uniformed guard is partly visible in a booth in the background (excluded by the prompt, as are the car, booth, yellow barrier, building and trees).
- Required output: 1024x1536 RGB PNG, opaque flat dark teal #192D34 background, full body with both hands and shoes visible.
- Pipeline job, window-scoped: `prakash_karat-a6ef64a014c1`. Default-inventory job partly closed: `prakash_karat-eda827228641`.
- Leadership-production cartoon jobs this window closes: `cartoon:prakash_karat:2005-04-30:2010-01-01:v1`, `cartoon:prakash_karat:2010-01-01:2015-01-01:v1`, `cartoon:prakash_karat:2015-01-01:2015-04-01:v1`.
- Settle first: Rule on the undated reference (camera stamp impossible; taken between 2013-03-21 and 2017-09-17) and the '2013' in its reference_id and filename; bytes and sha256 stay identical either way. If unacceptable, switch to the in-window alternate File:Prakash Karat CPIM.jpg (Suthir, EXIF 29 May 2014, CC BY-SA 3.0; soft and mid-speech), which would need a new prompt.

### 4. Rajnath Singh (`rajnath_singh`)

- Appearance window: 2005-12-31 to 2014-07-09 (exclusive end); status `appearance_interval_from_handoff`.
- Output path (after review): `spheres-web/ui/person-portraits/rajnath-singh-cartoon-2005-v1.png`
- Prompt file: `tools/avatars/person-prompts/rajnath-singh-cartoon-2005-v1.txt`
  - sha256 (LF, as committed): `951e50844f61f5d49ce37a8bcd2002d54625a6bf8cdc138da1855047d03bb6b5`
  - sha256 if your checkout wrote CRLF: `ca286ba09fdd22166916e75c8e6c8eeb1664b11166b25714da2be9ef2d0cd739`; git blob `d50c92b1e971210ebf22de9701c9be5240337be6`
- Input 1, STYLE ONLY: `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png`, sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`
- Input 2, identity reference: `spheres-web/ui/person-portraits/references/rajnath-singh-2013-reference-v1.jpg`, sha256 `728d8f40f91b9f7c23bb67175c56066baf00391c43850bef3acab4b2c70e4348`, 346x638 RGB JPEG, 142,836 bytes
  - Photograph date: 2013-10-27 or 2013-10-28 (Commons states 28 October 2013 08:14:38, the Flickr date; the file has no camera EXIF; the Hunkar Rally in Patna was held on 27 October 2013, general knowledge). Licence: CC BY-SA 2.0 (https://creativecommons.org/licenses/by-sa/2.0).
  - Credit: Narendra Modi (official Flickr account), 'Shri Modi at Hunkar Rally in Patna', Patna, October 2013; CC BY-SA 2.0; Commons crop 'Rajnath Singh at Hunkar Rally.jpg' of 'Modi at Hunkar Rally in Patna.jpg', via Wikimedia Commons. Adapted for Spheres; no endorsement implied.
  - In frame: Rajnath Singh alone, waist up, on stage, right hand raised in a victory sign with a gold ring on the ring finger. Also in frame: a red stage backdrop, partial Devanagari lettering (top left), part of a party emblem (bottom left), a dark screen edge (bottom right) and a sliver of another person at the right edge. The prompt excludes all of these and the raised-arm pose.
- Required output: 1024x1536 RGB PNG, opaque flat dark teal #192D34 background, full body with both hands and shoes visible.
- Pipeline job, window-scoped: `rajnath_singh-a4531051316c`. Default-inventory job partly closed: `rajnath_singh-909e17fd4488`.
- Leadership-production cartoon jobs this window closes: `cartoon:rajnath_singh:2005-12-31:2009-12-19:v1`, `cartoon:rajnath_singh:2013-01-31:2014-07-09:v1`.
- Settle first: Decide whether to split at 2009-12-19 / 2013-01-31 (the window spans Gadkari's presidency, appearance only). The reference is small (346x638).

### 5. Nitin Gadkari (`nitin_gadkari`)

- Appearance window: 2009-12-19 to 2013-01-23 (exclusive end); status `appearance_interval_from_handoff`.
- Output path (after review): `spheres-web/ui/person-portraits/nitin-gadkari-cartoon-2009-v1.png`
- Prompt file: `tools/avatars/person-prompts/nitin-gadkari-cartoon-2009-v1.txt`
  - sha256 (LF, as committed): `faebfd01a2aedb716f15380c69a31ae289fbd28a1ea6a7fa6d2bba68fc8b39bd`
  - sha256 if your checkout wrote CRLF: `69870282bdd42a44d3e07e311863283787161227f73b085dcae2b416c86cf44b`; git blob `a5bfff6c82bcfd7faa3728d5a7f81552521ab6f0`
- Input 1, STYLE ONLY: `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png`, sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`
- Input 2, identity reference: `spheres-web/ui/person-portraits/references/nitin-gadkari-2011-reference-v1.jpg`, sha256 `a62ff91b6c7038628b3fd26207b4d5dbfae3cd46a80022747509a2c8ef740138`, 2136x1424 RGB JPEG, 790,706 bytes
  - Photograph date: 2011-07-19. Licence: Open Government Licence v1.0 (https://www.nationalarchives.gov.uk/doc/open-government-licence/version/1/).
  - Credit: UK Foreign and Commonwealth Office, Foreign Secretary William Hague meeting BJP National President Nitin Gadkari in London, 19 July 2011 (Flickr 5954466771), Open Government Licence v1.0; via Wikimedia Commons. Adapted for Spheres; no endorsement implied.
  - In frame: Two men standing side by side, about three-quarter length (cut at the upper thigh): William Hague on the left (dark suit, yellow tie) and Nitin Gadkari on the right (light beige-silver bandhgala), both facing the camera, in an FCO interior with a gilt-framed oil painting, patterned wallpaper, a framed notice, a striped armchair and a sofa. The prompt excludes Hague and the whole setting.
- Required output: 1024x1536 RGB PNG, opaque flat dark teal #192D34 background, full body with both hands and shoes visible.
- Pipeline job, window-scoped: `nitin_gadkari-8a3390e6cd1e`. Default-inventory job partly closed: `nitin_gadkari-1c5c2e857ab6`.
- Leadership-production cartoon jobs this window closes: `cartoon:nitin_gadkari:2009-12-19:2010-01-01:v1`, `cartoon:nitin_gadkari:2010-01-01:2013-01-01:v1`.
- Settle first: Agree the end widened from 2013-01-01 to 2013-01-23 (the day after the accepted 2013-01-22 observation).

### 6. Sitaram Yechury (`sitaram_yechury`)

- Appearance window: 2015-04-30 to 2024-09-12 (exclusive end); status `appearance_interval_from_handoff`.
- Output path (after review): `spheres-web/ui/person-portraits/sitaram-yechury-cartoon-2015-v1.png`
- Prompt file: `tools/avatars/person-prompts/sitaram-yechury-cartoon-2015-v1.txt`
  - sha256 (LF, as committed): `1e32883811ee3ce51a3ff91a3aa802926a1da5163eef4bd475dbd6890397899d`
  - sha256 if your checkout wrote CRLF: `7eb663c63a946f3c86c5fe70fc0b28263da6ff227b2cf0cd575a6ad3bf0c5605`; git blob `6b463447c740d9f7e9d47c609781bf38ee8fa948`
- Input 1, STYLE ONLY: `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png`, sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`
- Input 2, identity reference: `spheres-web/ui/person-portraits/references/sitaram-yechury-2019-reference-v1.jpg`, sha256 `2e10cebb6c0f03f9f3aafb6f1c8c3ea8e29ba3496dffd9be287aaf8efdc9041c`, 1828x2345 RGB JPEG, 1,422,511 bytes
  - Photograph date: 2019-12-27 or 2019-12-28 (sources disagree). Licence: CC0 1.0 (https://creativecommons.org/publicdomain/zero/1.0/).
  - Credit: Batthini Vinay Kumar Goud, 'Sitaram Yechury at 2019 Hyderabad Book fair1 (cropped).jpg', 27 or 28 December 2019, own work, CC0 1.0, via Wikimedia Commons; crop by the photographer (CropTool, 2 January 2025) of 'Sitaram Yechury at 2019 Hyderabad Book fair1.jpg'. Adapted for Spheres; no endorsement implied.
  - In frame: Yechury alone, head and chest, speaking. A microphone is at the right and a pink/purple stage banner with Telugu lettering and '33' is behind him; the prompt excludes both and the pink-violet stage-light cast. The parent frame also shows an acrylic podium and part of a seated man, which the crop removes.
- Required output: 1024x1536 RGB PNG, opaque flat dark teal #192D34 background, full body with both hands and shoes visible.
- Pipeline job, window-scoped: `sitaram_yechury-e1e306607c4e`. Default-inventory job partly closed: `sitaram_yechury-cf3ef5171ade`.
- Leadership-production cartoon jobs this window closes: `cartoon:sitaram_yechury:2015-04-30:2020-01-01:v1`, `cartoon:sitaram_yechury:2020-01-01:2024-09-12:v1`.
- Settle first: Decide whether to split at 2020-01-01; the same reference fits both halves.

### 7. Jagat Prakash Nadda (`jagat_prakash_nadda`)

- Appearance window: 2020-01-20 to 2026-01-20 (exclusive end); status `appearance_interval_from_handoff`.
- Output path (after review): `spheres-web/ui/person-portraits/jagat-prakash-nadda-cartoon-2020-v1.png`
- Prompt file: `tools/avatars/person-prompts/jagat-prakash-nadda-cartoon-2020-v1.txt`
  - sha256 (LF, as committed): `1930455b813b99bd727ede047db90a290f652dfa41c1332e5985163421468088`
  - sha256 if your checkout wrote CRLF: `99faaeae883c631262e5f44d8a0bc03d59b86241b4ad17d49ea8524e5011b644`; git blob `2535c0b1220d97b8ca615a47fef672ef4748c581`
- Input 1, STYLE ONLY: `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png`, sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`
- Input 2, identity reference: `spheres-web/ui/person-portraits/references/jagat-prakash-nadda-2018-reference-v1.jpg`, sha256 `af91b1f05401ccafaa78e2c22226c16a0e6e70dcc11cb0518c53f8299a10825c`, 4912x7360 RGB JPEG, 2,657,486 bytes
  - Photograph date: 2018-09-26. Licence: CC BY-SA 4.0 (https://creativecommons.org/licenses/by-sa/4.0).
  - Credit: AbhiSuryawanshi, 'JP Nadda (Minister of Health and Family Welfare, India) at End TB Campaign.jpg', 26 September 2018, own work, CC BY-SA 4.0, via Wikimedia Commons. Adapted for Spheres; no endorsement implied.
  - In frame: Nadda alone, standing full length and frontal beside a large circular WHO/Stop TB 'End TB' Sustainable Development Goals wheel display in a lobby, in a dark aubergine-brown bandhgala suit with a magenta pocket square, pale pink shirt cuffs and brown shoes, a UN identity badge clipped to his jacket and his hands clasped at the waist. His right elbow touches the left frame edge. Nobody else is in frame.
  - Colour profile: Adobe RGB (1998), embedded (EXIF ColorSpace 65535, Photoshop CS4). Viewed without colour management the photograph looks desaturated; pass the bytes unchanged and do colour checks in a colour-managed view.
- Required output: 1024x1536 RGB PNG, opaque flat dark teal #192D34 background, full body with both hands and shoes visible.
- Pipeline job, window-scoped: `jagat_prakash_nadda-feaabd468258`. Default-inventory job partly closed: `jagat_prakash_nadda-15e969dcbefe`.
- Leadership-production cartoon jobs this window closes: `cartoon:jagat_prakash_nadda:2020-01-20:2025-01-01:v1`, `cartoon:jagat_prakash_nadda:2025-01-01:2026-01-20:v1`.
- Settle first: Agree the exclusive end 2026-01-20 (registry and leadership production, not accepted research). Era dress: the 2018 bandhgala is kept for the whole window (his in-window public dress was often a white kurta-pyjama with a saffron stole); accept or request a v2 prompt. The reference embeds an Adobe RGB (1998) ICC profile: pass the bytes unchanged and judge colour in a colour-managed view.

### 8. M. A. Baby (`m_a_baby`) (reserve, promoted to this render)

- Appearance window: 2025-04-06 to 2026-09-08 (exclusive end); status `appearance_interval_from_handoff`.
- Output path (after review): `spheres-web/ui/person-portraits/m-a-baby-cartoon-2025-v1.png`
- Prompt file: `tools/avatars/person-prompts/m-a-baby-cartoon-2025-v1.txt`
  - sha256 (LF, as committed): `2d1b6853ba359653247a0b3aaf2bd25144b04df7e594501c48b2c1e1abe5eaf9`
  - sha256 if your checkout wrote CRLF: `66e5daf2c8eed3a81a5e954d9fa9a136677985f99987e1b23b0c8fd495237da8`; git blob `4b00b123fb8c3170ace5817497f07293d964a6a3`
- Input 1, STYLE ONLY: `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png`, sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`
- Input 2, identity reference: `spheres-web/ui/person-portraits/references/m-a-baby-2026-reference-v1.jpg`, sha256 `4ac001ff5468ccf511b7b0a6cb0952f89b0f053b8bd8dcf9278084f7ae52a637`, 6000x4000 RGB JPEG, 8,126,087 bytes
  - Photograph date: 2026-07-06. Licence: CC BY-SA 4.0 (https://creativecommons.org/licenses/by-sa/4.0).
  - Credit: Fotokannan, 'MA Baby at Kollam 2026 18.jpg' (6 July 2026, Kadammanitta kavitha puraskaram event, Kollam), own work, CC BY-SA 4.0, via Wikimedia Commons. Adapted for Spheres; no endorsement implied.
  - In frame: M. A. Baby alone, head and shoulders, three-quarter view facing right, speaking with lips parted at a microphone; a blurred poster portrait of another man and a gray backdrop behind him. The prompt excludes the microphone, poster and backdrop and draws his lips closed.
- Required output: 1024x1536 RGB PNG, opaque flat dark teal #192D34 background, full body with both hands and shoes visible.
- Pipeline job, window-scoped: `m_a_baby-a1a2a42a89bf`. Default-inventory job partly closed: `m_a_baby-31e12a5aba87`.
- Leadership-production cartoon jobs this window closes: `cartoon:m_a_baby:2025-04-06:2026-09-08:v1`.
- Settle first: No decision needed: rendered in place of amit_shah, who was excluded at assembly.

## Not rendered

- `amit_shah` (primary, window 2014-07-09 to 2020-01-20): **excluded at assembly**. The only usable in-window reference, File:Pic of Amit Shah IMG 9161.jpg (CC BY-SA 4.0, own work, no VRT ticket), is very probably from a commissioned BJP portrait session whose sibling frames the party distributed for download in 2016, so the own-work licence cannot be relied on. Its copied reference and prompt were removed before any commit; the jobs stay open. Future routes (VRT-confirmed photographs with small faces, or a U.S. federal public-domain photograph) are listed in the identity review. M. A. Baby takes the render slot.
- `nitin_nabin` (reserve, window 2026-01-20 to 2026-09-08): **not ready**. No qualifying photograph exists: both freely labelled Commons candidates fail on provenance, and the rest are GODL-India or "Attribution". No reference or prompt was written; the jobs stay open.

## Return format

One entry per attempt, for example:

```json
{
  "person_id": "rajnath_singh",
  "attempt": 1,
  "generator": "OpenAI built-in image_gen",
  "generated_at": "YYYY-MM-DD",
  "prompt_path": "tools/avatars/person-prompts/rajnath-singh-cartoon-2005-v1.txt",
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
