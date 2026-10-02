# CLAUDE-C06-JAPAN-01: Japan cast, batch 1 (identities, references, prompts; Codex renders)

Owner: Claude. State: **ready for Codex render** (2 October 2026: batch 1 prepared and corrected after Codex's independent preparation review; Yamaguchi era fix added, its two new jobs to be checked by Codex before rendering; not ready_for_review, not complete). Parent: C06 (Japan country cast), with
C03 cartoon production for these windows. Pending Codex registration and acceptance.

Origin: the user asked on 1 October 2026 to continue the cast work in parallel after France batch 1
(`CLAUDE-C06-FRANCE-01`, branch `claude/c06-fr-01`) was prepared. The CP1 plan (`CP1-ACCELERATION.md`) allows parallel
artwork for another already-reviewed batch while Codex owns Tonga (`CLAUDE-C06-TONGA-01`). Generator route: **Codex
renders** with its built-in image tool, as for France. Claude prepares the identity review, dated likeness references,
exact prompts and input order, then reviews and registers the outputs Codex returns. `person_art_pipeline.py` requires
the generator "OpenAI built-in image_gen", and Claude has no such tool, so Claude never generates or labels artwork itself.

Branch: `claude/c06-jp-01`. Base: `2fd186d6` (current `codex/campaign-certification`); not stacked on France. Claim
commit: this record's first commit on the branch.

## Bounded deliverable

Batch `JP-CAST-B01`: seven portraits, plus two reserves, for people who already have registry IDs in
`spheres-sim/data/party_leaders.json`, accepted C01 holder observations (C01-29, C01-31) and no art for these windows.
The `to` dates are exclusive. Windows are appearance intervals, not tenures.

| person_id | requested window | research basis | game role shown |
|---|---|---|---|
| tomiichi_murayama | 1993-09-30 to 1996-09-01 | C01-29 chair 1994-10-13; C01-12 PM 1994-07-18 (integrated, acceptance pending) | JSP/SDP chair; Prime Minister 1994-1996 |
| makoto_tanabe | 1991-07-31 to 1993-01-19 | C01-29 chair 1991-08-20 (research spells him 田邊誠, registry 田辺誠) | JSP chair |
| sadao_yamahana | 1993-01-19 to 1993-09-01 | C01-29 chair 1993-01-25 | JSP chair |
| takako_doi | 1996-09-30 to 2003-11-01 | C01-29 chair 1996-11-30, 1998-01-21, 2000-01-21 (her 1990-04-06 observation is inside her existing 1990 art) | SDP leader (second portrait) |
| takenori_kanzaki | 1998-11-07 to 2006-09-30 | C01-31 representative 1998-11-08, 2002-11-03, 2004-10-31 | Komeito representative |
| akihiro_ota | 2006-09-30 to 2009-09-08 | C01-31 representative from 2006-09-30 | Komeito representative |
| natsuo_yamaguchi | 2009-09-08 to 2024-09-28 | C01-31 representative 2009-09-08 to 2022-09-25 (seven observations) | Komeito representative |
| tadatomo_yoshida (reserve) | 2013-10-24 to 2018-02-01 | C01-29 chair 2013-10-24 | SDP leader |
| seiji_mataichi (reserve) | 2018-02-25 to 2020-02-01 | C01-29 chair 2018-02-25 | SDP leader |

Selection. The C01 gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.json`, refreshed at `1fffb983`) classes
Japan's holder observations from C01-29 (JSP/SDP chairs), C01-31 (Komeito) and C01-42 (JCP, DPFP) as `c01_accepted`,
and those from C01-12 and C01-13 (prime ministers) and C01-18 (LDP presidents) as `c01_integrated_pending` (merged on
27 September; historical acceptance pending). Only accepted observations qualify a person for this batch. The country
card's opening leader, `toshiki_kaifu` (Japan's only `office_links` row), already has art for 1990-01-01 to 1995-01-01,
which covers all his observations. Murayama is the only head of government with an accepted observation. The others
lead game party rows and are taken in the order the game first shows them without art. Each window is the union of
that person's leadership-production required windows, widened only where an accepted observation falls outside it
(Yoshida, Mataichi). Every accepted `attested_on` or `from` of each person falls inside one of their windows.

Deferred, not claimed: `tetsuzo_fuwa` (JCP; uncovered from 1995-01-01; a first Commons check found only 1956 and 1973
public-domain photographs and a 2020s video still, so no qualifying reference near the window is established yet);
`kazuo_shii` and `mizuho_fukushima` (their accepted observations run 2001-2024 and 2003-2026 and need two portraits
each); later Komeito and JCP leaders; the LDP presidents and other prime ministers with registry IDs, until C01-12, C01-13
and C01-18 are historically accepted or Codex rules otherwise; and people with no registry ID or no accepted observation.

Job IDs (from `python -B -X utf8 tools/avatars/person_art_pipeline.py jobs --out D:/spheres-scratch/jp-cast/jobs.json`,
sha256 `5f83d33def208c44f4a7a31e2b7f4ff5a8b3f15cf283abefba3455b7a87041ec`, identical to the France inventory; the
window-scoped IDs use the pipeline's own job content and digest for the exact window):

| person_id | default-inventory job (prospective partial coverage) | window-scoped job | leadership-production cartoon jobs targeted after output review |
|---|---|---|---|
| tomiichi_murayama | `tomiichi_murayama-79b359476464` | `tomiichi_murayama-98fcf367bb0e` | `cartoon:tomiichi_murayama:1993-09-30:1995-01-01:v1`, `cartoon:tomiichi_murayama:1995-01-01:1996-01-01:v1`, `cartoon:tomiichi_murayama:1996-01-31:1996-09-01:v1` |
| makoto_tanabe | `makoto_tanabe-7ccebadc09cf` | `makoto_tanabe-24b6b1ce9158` | `cartoon:makoto_tanabe:1991-07-31:1993-01-19:v1` |
| sadao_yamahana | `sadao_yamahana-1037f3c061ac` | `sadao_yamahana-d987a53f065e` | `cartoon:sadao_yamahana:1993-01-19:1993-09-01:v1` |
| takako_doi | `takako_doi-79dea7fb7fdb` (from 1995-01-01) | `takako_doi-517fc7ec0833` | `cartoon:takako_doi:1996-09-30:2000-01-01:v1`, `cartoon:takako_doi:2000-01-01:2003-11-01:v1` |
| takenori_kanzaki | `takenori_kanzaki-82f870c30a67` | `takenori_kanzaki-3f485761c684` | `cartoon:takenori_kanzaki:1998-11-07:2000-01-01:v1`, `cartoon:takenori_kanzaki:2000-01-01:2005-01-01:v1`, `cartoon:takenori_kanzaki:2005-01-01:2006-09-30:v1` |
| akihiro_ota | `akihiro_ota-a22bdc6a183b` | `akihiro_ota-943abda7c780` | `cartoon:akihiro_ota:2006-09-30:2009-09-08:v1` |
| natsuo_yamaguchi | `natsuo_yamaguchi-a71007185b71` | `natsuo_yamaguchi-93769cc0a66c` | `cartoon:natsuo_yamaguchi:2009-09-08:2010-01-01:v1`, `cartoon:natsuo_yamaguchi:2010-01-01:2015-01-01:v1`, `cartoon:natsuo_yamaguchi:2015-01-01:2020-01-01:v1`, `cartoon:natsuo_yamaguchi:2020-01-01:2024-09-28:v1` |
| tadatomo_yoshida (reserve) | `tadatomo_yoshida-7955526f3eda` | `tadatomo_yoshida-6f0765f8ff8c` | `cartoon:tadatomo_yoshida:2013-10-31:2015-01-01:v1`, `cartoon:tadatomo_yoshida:2015-01-01:2018-02-01:v1` |
| seiji_mataichi (reserve) | `seiji_mataichi-740bbf421e31` | `seiji_mataichi-64650692526d` | `cartoon:seiji_mataichi:2018-02-28:2020-01-01:v1`, `cartoon:seiji_mataichi:2020-01-01:2020-02-01:v1` |

The residual pipeline jobs left after registration are listed in the identity review. The generated `CA-Japan-B00N`
work-order IDs are not reused.

Open at claim, for Codex: Yamaguchi's fifteen-year window may be split at a leadership-production boundary (2015-01-01
or 2020-01-01); Yamahana's window ends at the production boundary 1993-09-01, although C01-29 records a continuation
statement of 24 September 1993 (a claim) and the registry's end is month-only; Murayama's prime-ministerial observation
is integrated but not yet historically accepted, so his window rests on C01-29 and the registry terms.

Claude delivers:
- `docs/campaign-certification/C06/production/japan/identity-review-batch-01.json`, a proposal in the shape of
  France's identity review;
- dated likeness references, copied byte-identical into `spheres-web/ui/person-portraits/references/`, with free
  licences from `FREE_LICENSES` only;
- exact prompts in `tools/avatars/person-prompts/`;
- a render request for Codex: per job, the prompt file, the input order (style anchor first, marked style-only; then
  the identity reference) and the output size (1024x1536 RGB);
- after Codex returns the unchanged outputs: the visual review, the batch generation record, the `person_portraits.json`
  records and the registration receipt.

Any reviewer string names the actual reviewer. No human approval is claimed. Japan's country sign-off stays with the
user and Codex.

## Allowed files and checks

Allowed files:
- this record;
- `docs/campaign-certification/C06/production/japan/`;
- `spheres-web/ui/person-portraits/references/` (new Japan reference files only);
- `tools/avatars/person-prompts/` (new Japan prompt and batch files only);
- after rendering, additive `people[pid].portraits[]` records in `spheres-web/data/person_portraits.json` and the new PNG
  files, coordinated with Codex.

Not touched: `party_leaders.json`, existing portrait records (including Kaifu's and Doi's 1990 art), the pipeline code,
shared runtime, the task queue and the workboard. Regenerated shared outputs are left to Codex's integration.

Checks: `person_art_pipeline.py self-test` and `validate`; `cartoon_review.py --check`; `leadership_production.py check`;
`campaign_census.py --check`; `python -m unittest discover -s tools/avatars`; `workboard.py --check`; `git diff --check`.

## Preparation

Batch 1 (`JP-CAST-B01`) was prepared on 1 October 2026. Independent Codex review on 2 October inspected exact source tip `7f8c5ed1e8d5b8d4f5ee61e72376741d34ff1725`, all six likeness references/prompts, all fourteen accepted holder copies and the MLIT source flyer. The 114 input checks passed. This is preparation acceptance with corrected metadata, not generated-art or country acceptance.

The [independent disposition](../../campaign-certification/C06/production/japan/independent-review-20261002.md), [render request](../../campaign-certification/C06/production/japan/render-request-batch-01.md) and [identity review](../../campaign-certification/C06/production/japan/identity-review-batch-01.json) carry the current exact inputs.

- Murayama: retain the official Kantei portrait with unknown capture date. A 1999 capture of an APEC 1995 page and a separately captured 2001 image do not establish a hard 1995 photograph deadline. The corrected prompt uses illustrative age. The January 1996 bridge is appearance only.
- Ota: retain the verified first Commons revision from the MLIT flyer. The 2012 Photoshop timestamp and 2013 PDF creation/modification date are metadata, not capture deadlines. The corrected prompt avoids a precise age gap.
- Tanabe, Yamahana and Doi: usable within the documented source and artistic-age limitations. A licensed official film frame is not intrinsically excluded and creates no new user-permission requirement. Tanabe's primary album and a fresh full Yamahana video were not independently read.
- Yamaguchi: the 2019 reference is usable, but separate early and later variants are recommended for the fifteen-year appearance window. The current broad request is still an editorial proposal; one age-64 likeness does not prove four era jobs complete.
- All twelve mapped production jobs are targeted coverage only, requiring generation, registration and output review. Zero portraits have been rendered or approved in this preparation.

The six original reference files remain byte-identical, and their names are retained identifiers. Only Murayama and Ota prompts changed and were re-pinned as LF bytes. Root will enforce LF for all six at integration. No historical holder object, person registry, runtime file or portrait manifest changed.

Kanzaki remains outside this six-person review pending his own source review; his earlier TV-frame exclusion is not a blanket rule. Yoshida and Mataichi were not prepared. See the country README for the original preparation checks and preserved source limitations.

## Era fix, 2 October 2026

The independent review recommended a separately reviewed early reference and a later variant for Natsuo Yamaguchi's fifteen-year window. Claude workflow agents (a preparer, an independent verifier and this assembler) produced the fix below. It is a proposal for Codex to check before rendering. It is not human, Codex or user approval.

- Split at 2015-01-01, a leadership-production job edge; record [`yamaguchi-era-split-20261002.json`](../../campaign-certification/C06/production/japan/yamaguchi-era-split-20261002.json) (sha256 `3e43bf7c81e8caeda044b267fe77bb3237c8306486d5da0376bf81f58c4757a5`). The split date is an art choice, not an office, tenure or physical-change date. A 2020-01-01 split would leave the 2013 photograph covering ages 57 to 67.
- Early part, 2009-09-08 to 2015-01-01 (accepted observations 2009-09-08, 2012-09-22, 2014-09-21):
  - new reference `spheres-web/ui/person-portraits/references/natsuo-yamaguchi-2013-reference-v1.jpg` (sha256 `733c566ef07a1e0b91623ff7549e301a5c8e73f09246ae9a2ed234c7f411dd17`), a byte-identical copy of the single Commons version of File:Natsuo Yamaguchi IMG 5607 20130707.JPG. It is Ogiyoshisan's own work, photographed 7 July 2013 (the day as the source states it). The licence is exactly 'CC BY 3.0', unported;
  - he is 60 in it and 57 to 62 across the part;
  - new prompt `tools/avatars/person-prompts/natsuo-yamaguchi-cartoon-2009-v2.txt` (LF sha256 `bdd8f7a4b7410cde0312cbc839b2f1d711260403b71345680a41faf4ab278ee6`), job `natsuo_yamaguchi-0468451c3d0f`, output `natsuo-yamaguchi-cartoon-2009-v2.png`, cartoon jobs 2009-09-08:2010-01-01 and 2010-01-01:2015-01-01;
  - the suit, shirt and tie colours come as text only from a 2 January 2013 STB-1 photograph (CC BY-SA 3.0). It is not an input and is not committed.
- Later part, 2015-01-01 to 2024-09-28 (accepted observations 2016-09-17, 2018-09-30, 2020-09-27, 2022-09-25):
  - the 29 August 2019 CC0 reference is reused unchanged; he is 67 in it and 62 to 72 across the part;
  - new prompt `tools/avatars/person-prompts/natsuo-yamaguchi-cartoon-2015-v1.txt` (LF sha256 `167d41df240fabe1a5c92cdaf6b8551363242bfcb417dab060f983af8c83d6e2`), job `natsuo_yamaguchi-af89e21deee1`, output `natsuo-yamaguchi-cartoon-2015-v1.png`, cartoon jobs 2015-01-01:2020-01-01 and 2020-01-01:2024-09-28.
- Superseded and not rendered: prompt `natsuo-yamaguchi-cartoon-2009-v1.txt` (file unchanged), job `natsuo_yamaguchi-93769cc0a66c`, output `natsuo-yamaguchi-cartoon-2009-v1.png`. The window-scoped job in the table above is superseded the same way. The residual jobs `natsuo_yamaguchi-fb0a754eff9e` and `natsuo_yamaguchi-5aac700002a9` are unchanged and stay open.
- Correction: the 2019 photograph shows him at 67, not "about age 64". 64 was the old window's midpoint age and the v1 prompt's target.
- Verifier: no blocking problem. Its one finding was fixed: the early prompt's "plain red tie" now notes the small light dot pattern, and the prompt was re-pinned.
- Search: no 2009-2012 photograph usable as a single-person likeness with a qualifying licence string was found on Commons. A sharper 2010-10-26 India PMO photograph is licensed 'GODL-India', which is not in FREE_LICENSES. The rejected candidates are listed in the record.
- Repointed in this commit: the identity review (now sha256 `cb3ac83c278c34b5a146e9e41bfc41d039a97f675ffa321f9a2822eb7d30daaf`; era parts, both references, prompts, prospective job coverage, supersession and rejected candidates), the render request (sections 6 and 7 replace the single job; the v1 job is marked superseded) and the country README.
- Outside this task: `.gitattributes` needs `tools/avatars/person-prompts/natsuo-yamaguchi-cartoon-2009-v2.txt text eol=lf` and `tools/avatars/person-prompts/natsuo-yamaguchi-cartoon-2015-v1.txt text eol=lf`. Until then a core.autocrlf=true checkout writes CRLF; the CRLF hashes are in the render request.
- Checks at assembly: `person_art_pipeline.py self-test` passed (21 tests). `validate` exited 0 (valid, 0 errors, the existing 530 warnings). `workboard.py --check`, `cartoon_review.py --check`, `leadership_production.py check`, `campaign_census.py --check` and `git diff --check` all exited 0. The verifier ran `unittest discover -s tools/avatars` and got one failure: `test_committed_matrix_is_current`, because this sparse checkout lacks the S23 matrix outputs. The other 827 tests passed.

All coverage stays prospective. No job closes until generation, registration and output review, and appearance windows never establish office tenure. No image has been generated, edited or labelled. `person_portraits.json`, `party_leaders.json` and `.gitattributes` are untouched.
