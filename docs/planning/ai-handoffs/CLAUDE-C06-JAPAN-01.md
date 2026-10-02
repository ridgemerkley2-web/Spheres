# CLAUDE-C06-JAPAN-01: Japan cast, batch 1 (identities, references, prompts; Codex renders)

Owner: Claude. State: **awaiting Codex render** (1 October 2026; batch 1 prepared; not ready_for_review, not complete). Parent: C06 (Japan country cast), with
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

| person_id | default-inventory job (partly closed) | window-scoped job | leadership-production cartoon jobs closed |
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

Batch 1 (`JP-CAST-B01`) was prepared on 1 October 2026. Each person had one preparation agent and one independent
verification agent. Claude assembled the results and fixed the verifiers' mechanical findings. For Murayama and Ota,
Claude also fetched a few dated sources to settle the findings. No image was generated, edited or labelled. The next
step is Codex's render: [render request](../../campaign-certification/C06/production/japan/render-request-batch-01.md).
The [identity review](../../campaign-certification/C06/production/japan/identity-review-batch-01.json) and
[README](../../campaign-certification/C06/production/japan/README.md) carry the observations, references and job IDs.

In (six primary renders):

| person_id | likeness reference (image date, licence) | prompt |
|---|---|---|
| tomiichi_murayama | Kantei official portrait (内閣官房内閣広報室), no capture date; the same sitting is on MOFA's APEC 1995 Osaka page, so no later than 1995; CC BY 4.0 | `tomiichi-murayama-cartoon-1993-v1.txt` |
| makoto_tanabe | MOFA Protocol Office, 7 Apr 1992 (Commons crop of a handshake with Jiang Zemin); CC BY 4.0 | `makoto-tanabe-cartoon-1991-v1.txt` |
| sadao_yamahana | Defense Agency 1993 record film, frame of 9 Aug 1993 (Commons crop); CC BY 3.0 | `sadao-yamahana-cartoon-1993-v1.txt` |
| takako_doi | Akira Kamikura, 2 Jul 2005 (Flickr); CC BY 2.0 | `takako-doi-cartoon-1996-v1.txt` |
| akihiro_ota | MLIT portrait, first Commons revision (MLIT flyer source), no later than 27 Dec 2012; CC BY 4.0 | `akihiro-ota-cartoon-2006-v1.txt` |
| natsuo_yamaguchi | TICAD7 Photographs / MOFA TICAD, 29 Aug 2019; CC0 | `natsuo-yamaguchi-cartoon-2009-v1.txt` |

Excluded: `takenori_kanzaki`. His only usable image is a frame from the Government Internet TV programme of
26 September 2006. The batch rule rejects TV stills, and no registered precedent covers that source. The only other free
image is a 1993 cabinet group photograph in which he is tiny. His reference copy and prompt were removed from the
repository before staging. They are pinned in scratch (`D:/spheres-scratch/jp-cast/excluded/takenori_kanzaki/` and the
refs folder) and can be restored unchanged if Codex rules that such frames qualify. His window, 1998-11-07 to
2006-09-30, stays uncovered.

Reserves: `tadatomo_yoshida` and `seiji_mataichi` were not prepared in this batch.

Verifier findings fixed:
- Murayama: the undated reference now has a terminus ante quem of 1995, from MOFA's APEC 1995 page. Provenance notes and
  the rights, limitation and date fields are also fixed.
- Yamahana: the suit colour; his own jacket, misread as a bystander; the EXIF wording; a hash fragment; a byte count.
- Ota: the response.jp bytes are replaced by the MLIT-sourced first Commons revision. The date and credit are rewritten,
  and the prompt describes the tighter crop.
- Yamaguchi: an isolation clause, the name-card position and the hands wording.

Tanabe and Doi passed outright.

Open questions for Codex:
- Murayama: accept a reference dated by terminus ante quem (Kaifu precedent), or switch to the dated 25 August 1994
  alternate. Also the '1994' in the reference name and the January 1996 bridge.
- Yamahana: confirm that a government film frame qualifies (koshiro_ishida precedent). Optionally end the window at
  1993-10-01.
- Ota: the GJSTU 2.0 to CC BY 4.0 route for MLIT and the archived first revision; the out-of-window reference with a
  latest-likely date.
- Yamaguchi: the 27 MB reference; a window split at 2015-01-01 or 2020-01-01.
- Kanzaki: whether a government-owned CC BY 4.0 video frame qualifies.
- Doi, at visual review: the glasses and the clothing colours taken from the 2000 photograph.
- All: an `eol=lf` rule for the six prompt files (`.gitattributes` is outside this task's files).

Worktree note: the sparse checkout gained `spheres-web/ui/{portraits,leader-art,display-art}` and
`docs/campaign-certification/{S10,C04,verification}` so that the self-test and tool checks could read their inputs. No
file there was changed.
