# Japan cast batch 1: render request for Codex (JP-CAST-B01)

From Claude, task `CLAUDE-C06-JAPAN-01`, branch `claude/c06-jp-01`, 1 October 2026. State: **awaiting Codex render**. Nothing here is rendered, reviewed or approved yet.

Identity review: [`identity-review-batch-01.json`](identity-review-batch-01.json) (sha256 `cd9181750ef311ec0595346387b3276882ee3d3dea31e919de4c8ae04ab8129d`). It holds the accepted C01 observations, the verified references, the rejected candidates, the verifier dispositions and the excluded person.

## What Codex does

For each of the six primary jobs below, once its "Settle first" items are ruled on:

1. Check the inputs: the prompt file's sha256 equals the pinned value (`git show HEAD:<prompt path> | sha256sum`), and the style anchor and identity reference match their sha256.
2. Generate with the built-in image tool (the pipeline requires generator `OpenAI built-in image_gen`). Submit the prompt file's text exactly as committed, with the images in this order:
   - image 1: the style anchor `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png` (sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`), **STYLE ONLY**. It supplies outlines, cel shading, proportions, framing and the dark teal background, never the face, hair, clothes or pose;
   - image 2: that job's identity reference.
3. Required output: 1024x1536 RGB, opaque. Make up to **3 attempts per portrait**.
4. Return every attempt **unchanged**: the `exec-*.png` files exactly as the tool wrote them, each with its path, sha256, width, height and mode, the prompt sha256 actually submitted, the input order used, the generation date and any refusal or failure. Do not crop, resize, recolour, re-encode or otherwise edit them. You may say which attempt you would choose.
5. Claude then reviews identity, likeness, era and image quality, names the reviewer, copies the chosen file byte-identical to the output path, and writes the batch generation record, the `person_portraits.json` records and the registration receipt. Review flags stay false until those checks actually happen. No human approval is claimed.

Line endings: the six prompt files are LF and no `.gitattributes` rule covers them yet (core.autocrlf=true on this machine). This task may not edit `.gitattributes`, so please add a rule at integration, as for `tonga-*.txt` and the France prompts, for example `tools/avatars/person-prompts/<each Japan prompt>.txt text eol=lf`. Until then a Windows checkout writes CRLF; its hash is listed for each job.

## Decisions needed from Codex

Before rendering:

- `tomiichi_murayama`: Accept a likeness reference dated by terminus ante quem (published by MOFA no later than 1995; capture date not stated) rather than by capture date. Precedent: the registered toshiki_kaifu identity_source (Kantei official portrait, 'exact capture date not established'). If refused, switch to the exactly dated in-window alternate File:Vo Van Kiet and Tomiichi Murayama.jpg (1994-08-25, CC BY 4.0, pinned in scratch), which needs a new prompt and gives a weaker likeness (face about 40 px wide).
- `tomiichi_murayama`: Rule whether the reference keeps '1994' in its reference_id and filename or is renamed (for example tomiichi-murayama-official-reference-v1.jpg); the bytes and sha256 stay identical either way.
- `tomiichi_murayama`: Agree or amend the window's bridge over the January 1996 rename month (1996-01-01 to 1996-01-31), which no leadership-production term and no accepted observation covers.
- `sadao_yamahana`: Rule that a still frame from a government-produced, producer-released CC BY film qualifies as a likeness reference here, citing the registered koshiro_ishida identity_source (references/koshiro-ishida-august-1993.jpg; same film, same day, same CC BY 3.0 licence). If refused, no qualifying reference exists and the person should be excluded.
- `akihiro_ota`: Confirm the GJSTU 2.0 to CC BY 4.0 licence route for an MLIT portrait (the same route the brief allows for Kantei images; MLIT now uses PDL1.0) and the use of an archived first Commons revision as the reference.
- `akihiro_ota`: Accept a reference about 3.3 years after the window with a latest-likely date (taken no later than 2012-12-27); no usable in-window photograph exists.
- `natsuo_yamaguchi`: Accept the 27,276,808-byte reference in Git (the untouched 8256x5504 original; larger than any existing reference and than France's 17 MB Juppé reference; Commons has no extracted crop).
- `natsuo_yamaguchi`: Decide whether to split the window at 2015-01-01 or 2020-01-01 (see requested_window.possible_split). For a 2009-2015 portrait, qualifying unviewed candidates are File:Natsuo Yamaguchi IMG 5607 20130707.JPG (CC BY 3.0) and the 2013-12-03 State Department public-domain Biden-meeting files.
- `takenori_kanzaki` (excluded, not in this render): rule whether a government-owned CC BY 4.0 video frame qualifies as a likeness reference. If yes, his prepared reference and prompt can be restored unchanged from scratch for a later batch (see the identity review's `excluded_people`).

Not blocking the render:

- `makoto_tanabe`: Optional: confirm the GJSTU 2.0 to CC BY 4.0 licence route (the Commons licence string is exactly 'CC BY 4.0'; MOFA's terms page returned HTTP 403, so any third-party exception there was not read).
- `makoto_tanabe`: Optional: the day 1992-04-07 comes from the Commons description; the structured date is 1992-04 (either is inside the window).
- `sadao_yamahana`: Optional: extend the window end from 1993-09-01 to 1993-10-01 on the strength of the 24 September 1993 continuation claim and the month-only registry end; the reference fits either.
- `takako_doi`: At visual review: keep or drop the thin metal-framed glasses, which are confirmed for 2005 but not for 1996-2003.
- `takako_doi`: At visual review: the prompt takes the jacket and skirt colours from the small in-window 2000 photograph; a reviewer may prefer the 2005 outfit (ivory jacket, black-and-white striped top, gray trousers).
- All: add an `eol=lf` rule for the six prompt files at integration.

## Primary jobs

### 1. Tomiichi Murayama (`tomiichi_murayama`)

- Appearance window: 1993-09-30 to 1996-09-01 (exclusive end); status `appearance_interval_from_handoff`.
- Output path (after review): `spheres-web/ui/person-portraits/tomiichi-murayama-cartoon-1993-v1.png`
- Prompt file: `tools/avatars/person-prompts/tomiichi-murayama-cartoon-1993-v1.txt`
  - sha256 (LF, as committed): `2faa578f7bc920a711c8fe5e193217a29f269404dc2283ff11f60b8455a4f436`
  - sha256 if your checkout wrote CRLF: `699b2d925ad299a4c83cc5b0a3f7f88ddc5b82766889ed7544fb9d18f76f8805`; git blob `f855aabec081704188db0ad4c33f873af04f8690`
- Input 1, STYLE ONLY: `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png`, sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`
- Input 2, identity reference: `spheres-web/ui/person-portraits/references/tomiichi-murayama-1994-reference-v1.jpg`, sha256 `0412c4e05e6a88db36ef8853b25d4abcf296dc549600057676d7cda3251b400b`, 318x410 RGB JPEG, 49,717 bytes
  - Image date: Capture date not stated; taken no later than 1995 (the same sitting is on MOFA's APEC 1995 Osaka profile page, copyright 1995); probably about 1994, unattested. Licence: CC BY 4.0 (https://creativecommons.org/licenses/by/4.0).
  - Credit: Cabinet Public Relations Office (内閣官房内閣広報室), Government of Japan: official portrait of Tomiichi Murayama, 81st Prime Minister, from 「歴代内閣ホームページ情報：村山富市 内閣総理大臣（第81代）」（首相官邸ホームページ）(https://www.kantei.go.jp/jp/rekidainaikaku/081.html); capture date not stated (published by MOFA no later than 1995); Government of Japan Standard Terms of Use 2.0 (compatible with CC BY 4.0); via Wikimedia Commons 'Tomiichi Murayama 19940630.jpg' (licence review by Leoboudv, 9 March 2019). Adapted (redrawn as a cartoon) for Spheres; no endorsement implied.
  - In frame: Murayama alone; head and shoulders, studio portrait on a plain light gray backdrop. He wears a Diet member's lapel badge, which the prompt omits.
- Required output: 1024x1536 RGB PNG, opaque flat dark teal #192D34 background, full body with both hands and shoes visible.
- Pipeline job, window-scoped: `tomiichi_murayama-98fcf367bb0e`. Default-inventory job partly closed: `tomiichi_murayama-79b359476464`.
- Leadership-production cartoon jobs this window closes: `cartoon:tomiichi_murayama:1993-09-30:1995-01-01:v1`, `cartoon:tomiichi_murayama:1995-01-01:1996-01-01:v1`, `cartoon:tomiichi_murayama:1996-01-31:1996-09-01:v1`.
- Settle first: Accept a likeness reference dated by terminus ante quem (published by MOFA no later than 1995; capture date not stated) rather than by capture date. Precedent: the registered toshiki_kaifu identity_source (Kantei official portrait, 'exact capture date not established'). If refused, switch to the exactly dated in-window alternate File:Vo Van Kiet and Tomiichi Murayama.jpg (1994-08-25, CC BY 4.0, pinned in scratch), which needs a new prompt and gives a weaker likeness (face about 40 px wide). Rule whether the reference keeps '1994' in its reference_id and filename or is renamed (for example tomiichi-murayama-official-reference-v1.jpg); the bytes and sha256 stay identical either way. Agree or amend the window's bridge over the January 1996 rename month (1996-01-01 to 1996-01-31), which no leadership-production term and no accepted observation covers.

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
- Pipeline job, window-scoped: `makoto_tanabe-24b6b1ce9158`. Default-inventory job partly closed: `makoto_tanabe-7ccebadc09cf`.
- Leadership-production cartoon jobs this window closes: `cartoon:makoto_tanabe:1991-07-31:1993-01-19:v1`.
- Also open (not blocking the render): Optional: confirm the GJSTU 2.0 to CC BY 4.0 licence route (the Commons licence string is exactly 'CC BY 4.0'; MOFA's terms page returned HTTP 403, so any third-party exception there was not read). Optional: the day 1992-04-07 comes from the Commons description; the structured date is 1992-04 (either is inside the window).

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
- Pipeline job, window-scoped: `sadao_yamahana-d987a53f065e`. Default-inventory job partly closed: `sadao_yamahana-1037f3c061ac`.
- Leadership-production cartoon jobs this window closes: `cartoon:sadao_yamahana:1993-01-19:1993-09-01:v1`.
- Settle first: Rule that a still frame from a government-produced, producer-released CC BY film qualifies as a likeness reference here, citing the registered koshiro_ishida identity_source (references/koshiro-ishida-august-1993.jpg; same film, same day, same CC BY 3.0 licence). If refused, no qualifying reference exists and the person should be excluded.
- Also open (not blocking the render): Optional: extend the window end from 1993-09-01 to 1993-10-01 on the strength of the 24 September 1993 continuation claim and the month-only registry end; the reference fits either.

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
- Pipeline job, window-scoped: `takako_doi-517fc7ec0833`. Default-inventory job partly closed: `takako_doi-79dea7fb7fdb`.
- Leadership-production cartoon jobs this window closes: `cartoon:takako_doi:1996-09-30:2000-01-01:v1`, `cartoon:takako_doi:2000-01-01:2003-11-01:v1`.
- Also open (not blocking the render): At visual review: keep or drop the thin metal-framed glasses, which are confirmed for 2005 but not for 1996-2003. At visual review: the prompt takes the jacket and skirt colours from the small in-window 2000 photograph; a reviewer may prefer the 2005 outfit (ivory jacket, black-and-white striped top, gray trousers).

### 5. Akihiro Ōta (`akihiro_ota`)

- Appearance window: 2006-09-30 to 2009-09-08 (exclusive end); status `appearance_interval_from_handoff`.
- Output path (after review): `spheres-web/ui/person-portraits/akihiro-ota-cartoon-2006-v1.png`
- Prompt file: `tools/avatars/person-prompts/akihiro-ota-cartoon-2006-v1.txt`
  - sha256 (LF, as committed): `6b423954a9667778cde3472b3ee286ab96583c280fec19ad92d4523232356185`
  - sha256 if your checkout wrote CRLF: `fb1f8ad6c177a100fcbb2fffb57d53cbb7e0b8861cfc74092ea5d431caad2aad`; git blob `ef2dbe555a2d96618bb265a809a196b41be4ab82`
- Input 1, STYLE ONLY: `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png`, sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`
- Input 2, identity reference: `spheres-web/ui/person-portraits/references/akihiro-ota-2012-reference-v1.jpg`, sha256 `dc66c2da51532403c4eb214191a34b8c28913a06d3fc6cc38a9982ae6997d4c6`, 339x507 RGB JPEG, 27,547 bytes
  - Image date: No later than 2012-12-27 (Commons date, from an editing timestamp; see date_precision); capture date unknown. Licence: CC BY 4.0 (https://creativecommons.org/licenses/by/4.0).
  - Credit: 国土交通省 (Ministry of Land, Infrastructure, Transport and Tourism, Japan): portrait of Akihiro Ōta from the MLIT flyer https://www.mlit.go.jp/common/001001960.pdf (2013); dated 27 December 2012 on Commons from an editing timestamp; CC BY 4.0 (Government of Japan Standard Terms of Use Ver. 2.0, stated compatible with CC BY 4.0), attribution 国土交通省; via Wikimedia Commons, first revision (23 July 2023, TKsdik8900) of 'Akihiro Ōta 20121227.jpg'. Adapted for Spheres; no endorsement implied.
  - In frame: Ota alone, head and shoulders to the upper chest, in a studio portrait against a plain pale gray backdrop: dark navy suit, white shirt, blue tie with a small white geometric pattern, no visible lapel pin, no glasses. Nothing else is in frame.
- Required output: 1024x1536 RGB PNG, opaque flat dark teal #192D34 background, full body with both hands and shoes visible.
- Pipeline job, window-scoped: `akihiro_ota-943abda7c780`. Default-inventory job partly closed: `akihiro_ota-a22bdc6a183b`.
- Leadership-production cartoon jobs this window closes: `cartoon:akihiro_ota:2006-09-30:2009-09-08:v1`.
- Settle first: Confirm the GJSTU 2.0 to CC BY 4.0 licence route for an MLIT portrait (the same route the brief allows for Kantei images; MLIT now uses PDL1.0) and the use of an archived first Commons revision as the reference. Accept a reference about 3.3 years after the window with a latest-likely date (taken no later than 2012-12-27); no usable in-window photograph exists.

### 6. Natsuo Yamaguchi (`natsuo_yamaguchi`)

- Appearance window: 2009-09-08 to 2024-09-28 (exclusive end); status `appearance_interval_from_handoff`.
- Output path (after review): `spheres-web/ui/person-portraits/natsuo-yamaguchi-cartoon-2009-v1.png`
- Prompt file: `tools/avatars/person-prompts/natsuo-yamaguchi-cartoon-2009-v1.txt`
  - sha256 (LF, as committed): `7ac461c81ecd482a38acf42bb1a7f95a888b54cd594bcbe3c3c2e63b5ac3d3ed`
  - sha256 if your checkout wrote CRLF: `a1085cbf1defd6525dbfef8386502bc84edfe2e19a6730ff69eafeb8e5eee685`; git blob `5b27d2f84526a461e8450081edf9cf9bf35e01e2`
- Input 1, STYLE ONLY: `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png`, sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`
- Input 2, identity reference: `spheres-web/ui/person-portraits/references/natsuo-yamaguchi-2019-reference-v1.jpg`, sha256 `7a027371b0e26341491ab8ea9393ddd13ed7db6c36d5843721a0a880d5724354`, 8256x5504 RGB JPEG, 27,276,808 bytes
  - Image date: 2019-08-29. Licence: CC0 (http://creativecommons.org/publicdomain/zero/1.0/deed.en).
  - Credit: TICAD7 Photographs / MOFA TICAD, 'Plenary Session 4' (Flickr 48640890007), 29 August 2019; CC0 1.0 Universal Public Domain Dedication; via Wikimedia Commons. Adapted for Spheres; no endorsement implied.
  - In frame: Yamaguchi is the central, sharply focused speaker. Also in frame: a bespectacled delegate with an earpiece on his right (viewer's left), in front of whom his name card stands; probably Ichiro Aisawa at the right edge (a name card reading 'Mr. Ichiro Aisawa' is nearby, but cards in this frame are offset from the people they belong to); the back of a head in the right foreground; seated audience, staff and two video cameras on tripods behind. Name cards, water bottles, glasses and microphones are on the table. His right hand is partly visible resting on the table edge between his name card and the water bottle, and a shirt cuff shows by the glass. The prompt excludes everything except Yamaguchi.
- Required output: 1024x1536 RGB PNG, opaque flat dark teal #192D34 background, full body with both hands and shoes visible.
- Pipeline job, window-scoped: `natsuo_yamaguchi-93769cc0a66c`. Default-inventory job partly closed: `natsuo_yamaguchi-a71007185b71`.
- Leadership-production cartoon jobs this window closes: `cartoon:natsuo_yamaguchi:2009-09-08:2010-01-01:v1`, `cartoon:natsuo_yamaguchi:2010-01-01:2015-01-01:v1`, `cartoon:natsuo_yamaguchi:2015-01-01:2020-01-01:v1`, `cartoon:natsuo_yamaguchi:2020-01-01:2024-09-28:v1`.
- Settle first: Accept the 27,276,808-byte reference in Git (the untouched 8256x5504 original; larger than any existing reference and than France's 17 MB Juppé reference; Commons has no extracted crop). Decide whether to split the window at 2015-01-01 or 2020-01-01 (see requested_window.possible_split). For a 2009-2015 portrait, qualifying unviewed candidates are File:Natsuo Yamaguchi IMG 5607 20130707.JPG (CC BY 3.0) and the 2013-12-03 State Department public-domain Biden-meeting files.

## Excluded (do not render)

### Takenori Kanzaki (`takenori_kanzaki`)

- Window that stays uncovered: 1998-11-07 to 2006-09-30 (exclusive end).
- Reason: No qualifying likeness reference. The only usable image, File:Takenori Kanzaki 20060926.jpg (CC BY 4.0 via GJSTU 2.0, 内閣広報室, 2006-09-26, 400x360), is a frame from the Government Internet TV programme '安倍内閣の発足-平成18年9月26日' (gov-online prg753; the archived source page has only a video player, data-movie-id 16832), not a photograph. The batch rule asks for a dated photograph and rejects TV stills, and the handoff already defers tetsuzo_fuwa partly because his only near-modern image is a video still. Unlike Yamahana's frame, no registered precedent covers this source. The only other free Commons image is the 1993-08-09 Hosokawa cabinet group photograph, about five years before the window, in which he is a tiny figure. The verifier recommended holding him; the assembler excluded him and removed his prepared files from the repository.
- Prepared files kept outside Git: reference original `D:/spheres-scratch/jp-cast/refs/takenori_kanzaki/Takenori_Kanzaki_20060926.jpg` (sha256 `560c9667532483ce67ab9efffde0d475eba6d91e0a1d81b1db3dfbf431e515bc`), prompt `D:/spheres-scratch/jp-cast/excluded/takenori_kanzaki/takenori-kanzaki-cartoon-1998-v1.txt` (sha256 `8f945ce2ba28f968fb8cc4d2e31e644313dd35980099e0fc4f000ae85acfce2a`).
- Restore condition: If Codex or the user rules that a government-owned video frame licensed CC BY 4.0 (GJSTU 2.0) qualifies, copy the reference original and the prompt back unchanged (sha256 values above), record the ruling here and in the README, and render him in a later batch. Otherwise leave him uncovered.

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
