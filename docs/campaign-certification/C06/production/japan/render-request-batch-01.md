# Japan cast batch 1: render request for Codex (JP-CAST-B01)

From Claude, task `CLAUDE-C06-JAPAN-01`, branch `claude/c06-jp-01`, 1 October 2026; Yamaguchi era fix added 2 October 2026. State: **awaiting Codex render**. Nothing here is rendered, reviewed or approved yet.

Identity review: [`identity-review-batch-01.json`](identity-review-batch-01.json) (sha256 `cb3ac83c278c34b5a146e9e41bfc41d039a97f675ffa321f9a2822eb7d30daaf`, LF as committed). It holds the accepted C01 observations, the verified references, the rejected candidates, the verifier dispositions and the excluded person.

Yamaguchi era split: [`yamaguchi-era-split-20261002.json`](yamaguchi-era-split-20261002.json) (sha256 `3e43bf7c81e8caeda044b267fe77bb3237c8306486d5da0376bf81f58c4757a5`, LF as committed). It replaces his single 2009-2024 job with two era parts (sections 6 and 7) and supersedes the v1 prompt and job (see the end of the primary jobs).

## What Codex does

For each of the seven primary jobs below (six people; Natsuo Yamaguchi has two era parts), use the independent preparation disposition and retain its output-review limitations. Do not render the superseded Yamaguchi v1 job:

1. Check the inputs: the prompt file's sha256 equals the pinned value (`git show HEAD:<prompt path> | sha256sum`), and the style anchor and identity reference match their sha256.
2. Generate with the built-in image tool (the pipeline requires generator `OpenAI built-in image_gen`). Submit the prompt file's text exactly as committed, with the images in this order:
   - image 1: the style anchor `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png` (sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`), **STYLE ONLY**. It supplies outlines, cel shading, proportions, framing and the dark teal background, never the face, hair, clothes or pose;
   - image 2: that job's identity reference.
3. Required output: 1024x1536 RGB, opaque. Make up to **3 attempts per portrait**.
4. Return every attempt **unchanged**: the `exec-*.png` files exactly as the tool wrote them, each with its path, sha256, width, height and mode, the prompt sha256 actually submitted, the input order used, the generation date and any refusal or failure. Do not crop, resize, recolour, re-encode or otherwise edit them. You may say which attempt you would choose.
5. Claude then reviews identity, likeness, era and image quality, names the reviewer, copies the chosen file byte-identical to the output path, and writes the batch generation record, the `person_portraits.json` records and the registration receipt. Review flags stay false until those checks actually happen. No human approval is claimed.

Line endings: the six original prompt files now have LF checkout rules in `.gitattributes` (added at integration). The two new Yamaguchi era prompts (sections 6 and 7) do not yet, and core.autocrlf=true on this machine. This task may not edit `.gitattributes`, so please add at integration `tools/avatars/person-prompts/natsuo-yamaguchi-cartoon-2009-v2.txt text eol=lf` and `tools/avatars/person-prompts/natsuo-yamaguchi-cartoon-2015-v1.txt text eol=lf`. Until then a Windows checkout writes CRLF; its hash is listed for each job.

## Independent preparation disposition (2 October 2026)

[Review](independent-review-20261002.md) of exact submitted tip `7f8c5ed1e8d5b8d4f5ee61e72376741d34ff1725`: retain the six identified references. Murayama and Ota now state unknown capture dates; archive/editor/document metadata are not capture deadlines. Their corrected exact prompts are re-pinned below. No historical holder record changes.

The licensed official Yamahana film frame is usable, with careful output likeness review because it is soft. The absence of a still photograph creates no new approval rule. Tanabe's primary album was not independently read; its day-level date remains attributed to the Commons caption. Doi's younger treatment and glasses remain an expressly artistic interpretation.

Yamaguchi's 2019 source is usable, but a single age-64 image across 2009-2024 does not establish age-appropriate likeness for every year. A later 2015-2024 variant and a separately reviewed early reference are recommended. The current requested window and prompt are retained pending that editorial choice; no four era jobs are closed by this preparation.

Kanzaki stays outside this six-person batch pending his own source review; there is no blanket prohibition on licensed official video frames. Existing input references remain byte-identical, including the 27 MB Yamaguchi original. Year-containing Murayama/Ota reference filenames are identifiers, not capture-date claims. LF enforcement remains an integration concern for the exact hashes below.

All mapped job coverage is prospective and requires generation, registration and output review. No output is generated or approved.

## Yamaguchi era fix (2 October 2026)

Prepared, independently verified and assembled by Claude workflow agents after the disposition above; not yet reviewed by Codex. The single 2009-09-08 to 2024-09-28 window is split at the leadership-production boundary 2015-01-01:

- early part, 2009-09-08 to 2015-01-01: new reference, a 7 July 2013 photograph by Ogiyoshisan (CC BY 3.0, unported). He is 60 in it and 57 to 62 across the part;
- later part, 2015-01-01 to 2024-09-28: the existing 29 August 2019 CC0 reference, reused unchanged. He is 67 in it (the "age-64" above was the superseded single window's midpoint age and the v1 prompt's target, not his age in the photograph) and 62 to 72 across the part. The photograph is 78 days before the part's midpoint of 2019-11-15.

A 2020-01-01 split would leave the 2013 photograph covering ages 57 to 67. The split date is an art choice, not an office, tenure or physical-change date. The four leadership-production cartoon jobs are unchanged and divide two and two. No 2009-2012 photograph usable as a single-person likeness with a qualifying licence string was found on Commons; the rejected candidates and the search scope are in the era-split record. The v1 prompt file stays byte-identical as reviewed evidence and is superseded; do not render it.

All coverage stays prospective: no job closes before generation, registration and output review, and appearance windows never establish office tenure.

## Primary jobs

### 1. Tomiichi Murayama (`tomiichi_murayama`)

- Appearance window: 1993-09-30 to 1996-09-01 (exclusive end); status `appearance_interval_from_handoff`.
- Output path (after review): `spheres-web/ui/person-portraits/tomiichi-murayama-cartoon-1993-v1.png`
- Prompt file: `tools/avatars/person-prompts/tomiichi-murayama-cartoon-1993-v1.txt`
  - sha256 (LF, as committed): `8321abccdc8a9fe46af9fcfa966995c15b356e1bc03320fb07e279ca42a92af0`
  - sha256 if your checkout wrote CRLF: `d26f373f3d3dca052c632f181affc7cb45ef22e3fcc1c1e5441115e90aa0ffdb`; git blob `c85479b0eda0bcc7fbd608b1007cb6aa1221db7b`
- Input 1, STYLE ONLY: `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png`, sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`
- Input 2, identity reference: `spheres-web/ui/person-portraits/references/tomiichi-murayama-1994-reference-v1.jpg`, sha256 `0412c4e05e6a88db36ef8853b25d4abcf296dc549600057676d7cda3251b400b`, 318x410 RGB JPEG, 49,717 bytes
  - Image date: Capture date unknown; later archived APEC 1995-era material suggests context but no hard capture deadline. Licence: CC BY 4.0 (https://creativecommons.org/licenses/by/4.0).
  - Credit: Cabinet Public Relations Office (内閣官房内閣広報室), Government of Japan: official portrait of Tomiichi Murayama, 81st Prime Minister, from 「歴代内閣ホームページ情報：村山富市 内閣総理大臣（第81代）」（首相官邸ホームページ）(https://www.kantei.go.jp/jp/rekidainaikaku/081.html); capture date unknown; later archive captures do not establish a 1995 deadline; Government of Japan Standard Terms of Use 2.0 (compatible with CC BY 4.0); via Wikimedia Commons 'Tomiichi Murayama 19940630.jpg' (licence review by Leoboudv, 9 March 2019). Adapted (redrawn as a cartoon) for Spheres; no endorsement implied.
  - In frame: Murayama alone; head and shoulders, studio portrait on a plain light gray backdrop. He wears a Diet member's lapel badge, which the prompt omits.
- Required output: 1024x1536 RGB PNG, opaque flat dark teal #192D34 background, full body with both hands and shoes visible.
- Pipeline job, window-scoped: `tomiichi_murayama-98fcf367bb0e`. Default-inventory job targeted for partial coverage: `tomiichi_murayama-79b359476464`.
- Leadership-production cartoon jobs targeted after registration and visual review: `cartoon:tomiichi_murayama:1993-09-30:1995-01-01:v1`, `cartoon:tomiichi_murayama:1995-01-01:1996-01-01:v1`, `cartoon:tomiichi_murayama:1996-01-31:1996-09-01:v1`.
- Output review: Retain the independent preparation disposition and documented likeness/era limitations.

### 2. Makoto Tanabe (`makoto_tanabe`)

- Appearance window: 1991-07-31 to 1993-01-19 (exclusive end); status `appearance_interval_from_handoff`.
- Output path (after review): `spheres-web/ui/person-portraits/makoto-tanabe-cartoon-1991-v1.png`
- Prompt file: `tools/avatars/person-prompts/makoto-tanabe-cartoon-1991-v1.txt`
  - sha256 (LF, as committed): `0cd6d473e072f5a080cef12d241523d1ee43732c9784ece761790db065dc9ac9`
  - sha256 if your checkout wrote CRLF: `097f07cbf0c124139122eb102a372e5f327cdab90985cdeba27219fb213b0fd4`; git blob `dcf26f96bdd0a4c239724f76b4e03907ab36919d`
- Input 1, STYLE ONLY: `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png`, sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`
- Input 2, identity reference: `spheres-web/ui/person-portraits/references/makoto-tanabe-1992-reference-v1.jpg`, sha256 `ed698782f20e16860322b8a90d297b00b662f149a5a9f202b421c74b3f10f687`, 581x774 RGB JPEG, 86,215 bytes
  - Image date: 1992-04-07. Licence: CC BY 4.0 (https://creativecommons.org/licenses/by/4.0).
  - Credit: 外務省大臣官房儀典官室 (MOFA Protocol Office); Makoto Tanabe at the State Guest House, Tokyo, 7 April 1992, from MOFA's released photo album of Jiang Zemin's 1992 visit to Japan (https://www.mofa.go.jp/mofaj/annai/honsho/shiryo/shozo/pdfs/2023/2023-0620.pdf); attribution 外務省 (MOFA); CC BY 4.0 (Government of Japan Standard Terms of Use 2.0, compatible with CC BY 4.0), via Wikimedia Commons; Commons crop 'Makoto Tanabe 19920407.jpg' (CropTool, Taro2968332, 28 July 2025) of 'Makoto Tanabe and Jiang Zemin 19920407.jpg'. Adapted for Spheres; no endorsement implied.
  - In frame: Tanabe alone, head to upper chest, broad open smile; Jiang Zemin is removed by the crop. A gold-framed wall panel and dark green-and-gold damask wall are behind him, and a small round gold Diet-member lapel badge is on his left lapel; the prompt excludes all of these.
- Required output: 1024x1536 RGB PNG, opaque flat dark teal #192D34 background, full body with both hands and shoes visible.
- Pipeline job, window-scoped: `makoto_tanabe-24b6b1ce9158`. Default-inventory job targeted for partial coverage: `makoto_tanabe-7ccebadc09cf`.
- Leadership-production cartoon jobs targeted after registration and visual review: `cartoon:makoto_tanabe:1991-07-31:1993-01-19:v1`.
- Source limits: Current MOFA terms and third-party exceptions were read independently. The primary album was not independently read; the exact day remains attributed to the Commons caption and the structured month is April 1992.

### 3. Sadao Yamahana (`sadao_yamahana`)

- Appearance window: 1993-01-19 to 1993-09-01 (exclusive end); status `appearance_interval_from_handoff`.
- Output path (after review): `spheres-web/ui/person-portraits/sadao-yamahana-cartoon-1993-v1.png`
- Prompt file: `tools/avatars/person-prompts/sadao-yamahana-cartoon-1993-v1.txt`
  - sha256 (LF, as committed): `0f6270864427465f7d5aaa409ea87168e10a5e0c9e8184076726e972e62806aa`
  - sha256 if your checkout wrote CRLF: `c21ba5d3d1db85012e470acb4ba8de8c2ca6f2dae62ee8e5187a5c3af27522c6`; git blob `297ea9b4f4434e8cb8ecbe34d8d2bbdbdf06094a`
- Input 1, STYLE ONLY: `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png`, sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`
- Input 2, identity reference: `spheres-web/ui/person-portraits/references/sadao-yamahana-1993-reference-v1.jpg`, sha256 `a256049c2da071664809609e2021f910c22966d0633656299614d92e4489bb85`, 275x366 RGB JPEG, 85,556 bytes
  - Image date: 1993-08-09. Licence: CC BY 3.0 (https://creativecommons.org/licenses/by/3.0).
  - Credit: Japan Defense Agency (防衛庁), '平成5年防衛庁記録' (1993 record film), Ministry of Defense official YouTube channel, video KFhAKAU2Q_8 (published 27 December 2020), CC BY 3.0. Still frame of 9 August 1993 extracted and cropped on Wikimedia Commons by Taro2968332 ('Sadao Yamahana Morihiro Hosokawa Cabinet 19930809-2.jpg', from 'Morihiro Hosokawa Cabinet 19930809-2.jpg'). Adapted for Spheres; no endorsement implied.
  - In frame: Yamahana is the central man in large glasses, head and shoulders, turned toward the left in three-quarter view. The parent caption names, from the left, Yamahana, Prime Minister Morihiro Hosokawa and Tsutomu Hata. Other figures at the edges: part of another man's face at the upper right (probably Hosokawa, not identifiable in the crop), a white collar and dark cloth behind his ear at the right, and a man in a dark suit and white shirt behind him at the upper left. The dark area at the lower right is Yamahana's own charcoal jacket (left lapel and shoulder, continuous with his collar), with a small round silver lapel badge, probably the Diet member's badge, which the prompt omits. The prompt excludes the other figures and the outdoor garden.
- Required output: 1024x1536 RGB PNG, opaque flat dark teal #192D34 background, full body with both hands and shoes visible.
- Pipeline job, window-scoped: `sadao_yamahana-d987a53f065e`. Default-inventory job targeted for partial coverage: `sadao_yamahana-1037f3c061ac`.
- Leadership-production cartoon jobs targeted after registration and visual review: `cartoon:sadao_yamahana:1993-01-19:1993-09-01:v1`.
- Output review: Retain the independent preparation disposition and documented likeness/era limitations.
- Also open (not blocking the render): See the independent preparation disposition above; retain the documented output-review limitations.

### 4. Takako Doi (`takako_doi`)

- Appearance window: 1996-09-30 to 2003-11-01 (exclusive end); status `appearance_interval_from_handoff`.
- Output path (after review): `spheres-web/ui/person-portraits/takako-doi-cartoon-1996-v1.png`
- Prompt file: `tools/avatars/person-prompts/takako-doi-cartoon-1996-v1.txt`
  - sha256 (LF, as committed): `14bc6db8eea91726074ea2c0cb21db7ac1f08e1c972226a5b4849f35074e5c96`
  - sha256 if your checkout wrote CRLF: `f795625b0af36bae87486a3ab8f06d6d2d4f3b9ce0c4ff1ac9a0034fa81d8d87`; git blob `8f8e7cd9503eff7bcb37b3c067504e57aabcfea9`
- Input 1, STYLE ONLY: `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png`, sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`
- Input 2, identity reference: `spheres-web/ui/person-portraits/references/takako-doi-2005-reference-v1.jpg`, sha256 `d7b1ebe60fcc7d6abf4e2e29272a01bb31a5b2e582f32e87a26dc05e1c5ef6da`, 1280x960 RGB JPEG, 708,234 bytes
  - Image date: 2005-07-02. Licence: CC BY 2.0 (https://creativecommons.org/licenses/by/2.0).
  - Credit: Akira Kamikura, Flickr photo 23039536, 2 July 2005, CC BY 2.0, via Wikimedia Commons ('Takako Doi in Tokyo congressist election.jpg'; FlickreviewR passed cc-by-2.0 on 7 June 2007). Adapted for Spheres; no endorsement implied.
  - In frame: Doi is the woman in the centre in an ivory jacket, black-and-white striped top and gray trousers, stepping forward, looking down and holding out a microphone, seen from head to about the knees. Also in frame: a man in a dark jacket holding a microphone, a person wearing the candidate's sash, a woman in pink, a camera operator and passers-by, plus vertical banners and posters with Japanese lettering (one banner reads 社会民主党 土井たか子; posters show other people's printed faces), loudspeakers and a vehicle. Commons also categorises the file under Nobuto Hosaka and Mizuho Fukushima; neither identification is relied on, and Fukushima appears to be present only as a banner name. The prompt excludes everyone and everything except Doi.
- Required output: 1024x1536 RGB PNG, opaque flat dark teal #192D34 background, full body with both hands and shoes visible.
- Pipeline job, window-scoped: `takako_doi-517fc7ec0833`. Default-inventory job targeted for partial coverage: `takako_doi-79dea7fb7fdb`.
- Leadership-production cartoon jobs targeted after registration and visual review: `cartoon:takako_doi:1996-09-30:2000-01-01:v1`, `cartoon:takako_doi:2000-01-01:2003-11-01:v1`.
- Also open (not blocking the render): At visual review: keep or drop the thin metal-framed glasses, which are confirmed for 2005 but not for 1996-2003. At visual review: the prompt takes the jacket and skirt colours from the small in-window 2000 photograph; a reviewer may prefer the 2005 outfit (ivory jacket, black-and-white striped top, gray trousers).

### 5. Akihiro Ōta (`akihiro_ota`)

- Appearance window: 2006-09-30 to 2009-09-08 (exclusive end); status `appearance_interval_from_handoff`.
- Output path (after review): `spheres-web/ui/person-portraits/akihiro-ota-cartoon-2006-v1.png`
- Prompt file: `tools/avatars/person-prompts/akihiro-ota-cartoon-2006-v1.txt`
  - sha256 (LF, as committed): `bd0fdc42e07e9ade28fcf5df7c6a77b90457ba7ebfb9adef63136767cd31cbab`
  - sha256 if your checkout wrote CRLF: `8e7ae1d8de8ea883dbd2c722663f1da5d3a4714e85a9e0f6c5c5157fa4e39a82`; git blob `3714107162c89167c65ecfa0736d8fbe0c2f9348`
- Input 1, STYLE ONLY: `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png`, sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`
- Input 2, identity reference: `spheres-web/ui/person-portraits/references/akihiro-ota-2012-reference-v1.jpg`, sha256 `dc66c2da51532403c4eb214191a34b8c28913a06d3fc6cc38a9982ae6997d4c6`, 339x507 RGB JPEG, 27,547 bytes
  - Image date: Capture date unknown; 2012 editing metadata and 2013 PDF creation metadata are not capture dates. Licence: CC BY 4.0 (https://creativecommons.org/licenses/by/4.0).
  - Credit: 国土交通省 (Ministry of Land, Infrastructure, Transport and Tourism, Japan): official portrait of Akihiro Ōta in https://www.mlit.go.jp/common/001001960.pdf; capture date unknown (the PDF carries creation/modification metadata dated 2013-06-25). CC BY 4.0 via the stated Government of Japan terms, attribution 国土交通省; current MLIT PDL 1.0 terms preserve the compatible reuse route. Wikimedia Commons first revision (23 July 2023, TKsdik8900) of 'Akihiro Ōta 20121227.jpg'. Adapted for Spheres; no endorsement implied.
  - In frame: Ota alone, head and shoulders to the upper chest, in a studio portrait against a plain pale gray backdrop: dark navy suit, white shirt, blue tie with a small white geometric pattern, no visible lapel pin, no glasses. Nothing else is in frame.
- Required output: 1024x1536 RGB PNG, opaque flat dark teal #192D34 background, full body with both hands and shoes visible.
- Pipeline job, window-scoped: `akihiro_ota-943abda7c780`. Default-inventory job targeted for partial coverage: `akihiro_ota-a22bdc6a183b`.
- Leadership-production cartoon jobs targeted after registration and visual review: `cartoon:akihiro_ota:2006-09-30:2009-09-08:v1`.
- Output review: Retain the independent preparation disposition and documented likeness/era limitations.

### 6. Natsuo Yamaguchi, early part (`natsuo_yamaguchi`)

- Appearance window: 2009-09-08 to 2015-01-01 (exclusive end); status `appearance_interval_era_part` (era split of 2 October 2026, [record](yamaguchi-era-split-20261002.json)).
- Output path (after review): `spheres-web/ui/person-portraits/natsuo-yamaguchi-cartoon-2009-v2.png`
- Prompt file: `tools/avatars/person-prompts/natsuo-yamaguchi-cartoon-2009-v2.txt`
  - sha256 (LF, as committed): `bdd8f7a4b7410cde0312cbc839b2f1d711260403b71345680a41faf4ab278ee6`
  - sha256 if your checkout wrote CRLF: `ca9d98b4d5c6dfebb2fe816d98b781ee6a5406d7329665a79ae5f9c0de840299`; git blob `11d07e1a2fde4a03b58629b04a170ec0d66ce635`
- Input 1, STYLE ONLY: `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png`, sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`
- Input 2, identity reference: `spheres-web/ui/person-portraits/references/natsuo-yamaguchi-2013-reference-v1.jpg`, sha256 `733c566ef07a1e0b91623ff7549e301a5c8e73f09246ae9a2ed234c7f411dd17`, 1200x1600 RGB JPEG, 824,703 bytes (byte-identical to the single Commons file version; Commons SHA-1 `f6b1e05247d97514f3b63ebabd0ba94b4763cee1`)
  - Image date: 2013-07-07, as the source states it (Commons Date field and file name); EXIF DateTimeOriginal 2013:07:07 15:54:28 is the camera clock. Not capture dates: the EXIF DateTime 17:13:33 (a later Windows Photo Viewer save) and the upload of 22 July 2013. Licence: CC BY 3.0 (https://creativecommons.org/licenses/by/3.0), unported.
  - Credit: Ogiyoshisan, 'Natsuo Yamaguchi IMG 5607 20130707.JPG' (own work), photograph of 7 July 2013, CC BY 3.0 (https://creativecommons.org/licenses/by/3.0), via Wikimedia Commons (https://commons.wikimedia.org/wiki/File:Natsuo_Yamaguchi_IMG_5607_20130707.JPG). Adapted (redrawn as a cartoon) for Spheres; no endorsement implied.
  - In frame: Yamaguchi is the central, nearest man, waist-up, giving an outdoor street speech in the 2013 House of Councillors campaign in harsh sun (probably Umeda, Osaka, inferred from the same photographer's same-day IMG 5611, not stated for this file): white shirt with thin blue stripes and rolled-up sleeves, no jacket or tie, microphones in his right hand, left hand raised in a loose fist, wristwatch on his left wrist. Also in frame: a man in a white shirt and glasses at the left edge, a head behind his left shoulder, a dark figure and a blue panel at the right edge, microphones, cables, a railing, a black panel, a shrub and a building facade. The prompt excludes everything except Yamaguchi.
  - Clothing colours (text only, not an input and not committed): File:Yamaguchi natsuo.jpg, street speech at Shinjuku West Exit, 2 January 2013, STB-1 own work, CC BY-SA 3.0 and GFDL: dark navy suit, white shirt, red tie with a small light dot pattern, Diet member's badge (omitted).
- Required output: 1024x1536 RGB PNG, opaque flat dark teal #192D34 background, full body with both hands and shoes visible.
- Pipeline job, window-scoped: `natsuo_yamaguchi-0468451c3d0f`. Default-inventory job targeted for partial coverage: `natsuo_yamaguchi-a71007185b71`.
- Leadership-production cartoon jobs targeted after registration and visual review: `cartoon:natsuo_yamaguchi:2009-09-08:2010-01-01:v1`, `cartoon:natsuo_yamaguchi:2010-01-01:2015-01-01:v1`.
- Accepted C01-31 observations inside: 2009-09-08, 2012-09-22, 2014-09-21.
- Age: he is 60 in the photograph (five days before his 61st birthday) and 57 to 62 across this part; the prompt asks for about 60, with very dark hair and no gray, and says one dated photograph is not evidence of every year.
- Output review: the suit, tie, trousers, standing pose and shoes are artistic extensions of a shirtsleeved, slightly soft waist-up campaign photograph; check the likeness against it.
- Settle first: Codex review of this new reference, this prompt and the 2015-01-01 split; so far they are Claude-prepared and Claude-verified only.

### 7. Natsuo Yamaguchi, later part (`natsuo_yamaguchi`)

- Appearance window: 2015-01-01 to 2024-09-28 (exclusive end); status `appearance_interval_era_part` (era split of 2 October 2026, [record](yamaguchi-era-split-20261002.json)).
- Output path (after review): `spheres-web/ui/person-portraits/natsuo-yamaguchi-cartoon-2015-v1.png`
- Prompt file: `tools/avatars/person-prompts/natsuo-yamaguchi-cartoon-2015-v1.txt`
  - sha256 (LF, as committed): `167d41df240fabe1a5c92cdaf6b8551363242bfcb417dab060f983af8c83d6e2`
  - sha256 if your checkout wrote CRLF: `2c1be3052db6d9f155390f343a92877c64cd3a42c4b67f3afa69d31047269746`; git blob `0c1f43baa981ac9e679e1629ef362194703ebc53`
  - Differs from the superseded v1 prompt only in the window sentence and the age sentence.
- Input 1, STYLE ONLY: `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png`, sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`
- Input 2, identity reference (unchanged, reused): `spheres-web/ui/person-portraits/references/natsuo-yamaguchi-2019-reference-v1.jpg`, sha256 `7a027371b0e26341491ab8ea9393ddd13ed7db6c36d5843721a0a880d5724354`, 8256x5504 RGB JPEG, 27,276,808 bytes
  - Image date: 2019-08-29. Licence: CC0 (http://creativecommons.org/publicdomain/zero/1.0/deed.en).
  - Credit: TICAD7 Photographs / MOFA TICAD, 'Plenary Session 4' (Flickr 48640890007), 29 August 2019; CC0 1.0 Universal Public Domain Dedication; via Wikimedia Commons. Adapted for Spheres; no endorsement implied.
  - In frame: Yamaguchi is the central, sharply focused speaker. Also in frame: a bespectacled delegate with an earpiece on his right (viewer's left), in front of whom his name card stands; probably Ichiro Aisawa at the right edge (a name card reading 'Mr. Ichiro Aisawa' is nearby, but cards in this frame are offset from the people they belong to); the back of a head in the right foreground; seated audience, staff and two video cameras on tripods behind. Name cards, water bottles, glasses and microphones are on the table. His right hand is partly visible resting on the table edge between his name card and the water bottle, and a shirt cuff shows by the glass. The prompt excludes everything except Yamaguchi.
- Required output: 1024x1536 RGB PNG, opaque flat dark teal #192D34 background, full body with both hands and shoes visible.
- Pipeline job, window-scoped: `natsuo_yamaguchi-af89e21deee1`. Default-inventory job targeted for partial coverage: `natsuo_yamaguchi-a71007185b71`.
- Leadership-production cartoon jobs targeted after registration and visual review: `cartoon:natsuo_yamaguchi:2015-01-01:2020-01-01:v1`, `cartoon:natsuo_yamaguchi:2020-01-01:2024-09-28:v1`.
- Accepted C01-31 observations inside: 2016-09-17, 2018-09-30, 2020-09-27, 2022-09-25.
- Age: he is 67 in the photograph and 62 to 72 across this part; the photograph is 78 days before the part's midpoint of 2019-11-15. The prompt asks for about 67, as in the photograph, and says one dated photograph is not evidence of every year.
- Output review: the standing pose, trousers and shoes are artistic extensions of a seated, waist-up photograph.
- Settle first: Codex review of this prompt and the 2015-01-01 split.

### Superseded: Natsuo Yamaguchi single window (do not render)

Superseded on 2 October 2026 by sections 6 and 7. Kept for the record of what was reviewed:

- Appearance window: 2009-09-08 to 2024-09-28; pipeline job `natsuo_yamaguchi-93769cc0a66c`; planned output `spheres-web/ui/person-portraits/natsuo-yamaguchi-cartoon-2009-v1.png`.
- Prompt file `tools/avatars/person-prompts/natsuo-yamaguchi-cartoon-2009-v1.txt` (LF sha256 `7ac461c81ecd482a38acf42bb1a7f95a888b54cd594bcbe3c3c2e63b5ac3d3ed`, CRLF `a1085cbf1defd6525dbfef8386502bc84edfe2e19a6730ff69eafeb8e5eee685`, git blob `5b27d2f84526a461e8450081edf9cf9bf35e01e2`), unchanged as reviewed evidence. Do not submit it.
- Its open items: the 27,276,808-byte 2019 reference stays byte-identical in Git, as the independent review retained it, and is reused by section 7; the split question is answered by the era-split proposal at 2015-01-01.

## Excluded (do not render)

### Takenori Kanzaki (`takenori_kanzaki`)

- Window that stays uncovered: 1998-11-07 to 2006-09-30 (exclusive end).
- Historical preparer reason (1 October; not an adopted blanket rule): No qualifying likeness reference. The only usable image, File:Takenori Kanzaki 20060926.jpg (CC BY 4.0 via GJSTU 2.0, 内閣広報室, 2006-09-26, 400x360), is a frame from the Government Internet TV programme '安倍内閣の発足-平成18年9月26日' (gov-online prg753; the archived source page has only a video player, data-movie-id 16832), not a photograph. The batch rule asks for a dated photograph and rejects TV stills, and the handoff already defers tetsuzo_fuwa partly because his only near-modern image is a video still. Unlike Yamahana's frame, no registered precedent covers this source. The only other free Commons image is the 1993-08-09 Hosokawa cabinet group photograph, about five years before the window, in which he is a tiny figure. The verifier recommended holding him; the assembler excluded him and removed his prepared files from the repository.
- Prepared files kept outside Git: reference original `D:/spheres-scratch/jp-cast/refs/takenori_kanzaki/Takenori_Kanzaki_20060926.jpg` (sha256 `560c9667532483ce67ab9efffde0d475eba6d91e0a1d81b1db3dfbf431e515bc`), prompt `D:/spheres-scratch/jp-cast/excluded/takenori_kanzaki/takenori-kanzaki-cartoon-1998-v1.txt` (sha256 `8f945ce2ba28f968fb8cc4d2e31e644313dd35980099e0fc4f000ae85acfce2a`).
- Restore condition: Independently verify this excluded person's exact source, identity and licence before a later batch. A licensed official film frame is not intrinsically barred and creates no new user-permission requirement.

## Reserves

The handoff's two reserves, `tadatomo_yoshida` (2013-10-24 to 2018-02-01) and `seiji_mataichi` (2018-02-25 to 2020-02-01), were not prepared in this batch: there is no reference or prompt for them yet.

## Return format

One entry per attempt, for example:

```json
{
  "person_id": "makoto_tanabe",
  "attempt": 1,
  "generator": "OpenAI built-in image_gen",
  "generated_at": "YYYY-MM-DD",
  "prompt_path": "tools/avatars/person-prompts/makoto-tanabe-cartoon-1991-v1.txt",
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
