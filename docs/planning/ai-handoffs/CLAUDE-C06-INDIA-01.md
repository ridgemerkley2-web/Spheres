# CLAUDE-C06-INDIA-01: India cast, batch 1 (identities, references, prompts; Codex renders)

Owner: Claude. State: **claimed** (2 October 2026; in progress, not complete). Parent: C06 (India country cast), with
C03 cartoon production for these windows. Pending Codex registration and acceptance.

Origin: the user asked on 1 October 2026 to continue the cast work in parallel after the France, Japan, Brazil and South
Africa batches were prepared (`CLAUDE-C06-FRANCE-01`, `CLAUDE-C06-JAPAN-01`, `CLAUDE-C06-BRAZIL-01`,
`CLAUDE-C06-SOUTHAFRICA-01`). The CP1 plan (`CP1-ACCELERATION.md`) allows parallel artwork for another already-reviewed
batch while Codex owns Tonga (`CLAUDE-C06-TONGA-01`), and India is in the CP1 country scope. Generator route, as for
France: **Codex renders** with its built-in image tool. Claude prepares the identity review, dated likeness references,
exact prompts and input order, then reviews and registers the outputs Codex returns. `person_art_pipeline.py` requires
the generator "OpenAI built-in image_gen", and Claude has no such tool, so Claude never generates, edits or labels
artwork itself.

Branch: `claude/c06-in-01`. Base: `2fd186d6` (current `codex/campaign-certification`, checked against the remote on
2 October 2026); not stacked on any other C06 branch. Claim commit: this record's first commit on the branch.

## Bounded deliverable

Batch `IN-CAST-B01`: eight portraits of eight people who already have registry IDs in
`spheres-sim/data/party_leaders.json`, accepted C01 holder observations and no art for these windows, plus two reserves.
The accepted research is C01-27 (BJP national presidents) and C01-40 (CPI(M) general secretaries, with its reviewed
amendment) in `docs/campaign-certification/C01/research/india.json`. Every accepted holder observation of each person
(its `attested_on`, or its stated `from` where there is no `attested_on`) falls inside one of that person's windows below
or inside their existing art. The `to` dates are exclusive. A window is an appearance interval, not a tenure, and
grouping observations under a person ID proves no term.

| person_id | requested window | accepted observations (C01 packet) | window basis |
|---|---|---|---|
| harkishan_singh_surjeet | 1992-01-31 to 2005-04-01 | C01-40 CPI(M): 2004-03-02 | registry term `in_cpm_harkishan_singh_surjeet_199201` (month precision; bounds as in leadership production); 13 years, so Codex may split at 2000-01-01 (a leadership-production job boundary) |
| lal_krishna_advani | 1995-01-01 to 2005-12-31 | C01-27 BJP: 1997-07-16, 2004-10-20 (his 1991-01-08 observation is inside his existing 1990-1995 art) | continues his existing art; covers registry terms `in_bjp_lal_krishna_advani_1993` and `in_bjp_lal_krishna_advani_20041027` and the accepted 2004-10-20 observation, which precedes that term's registry start of 2004-10-27; one portrait across 1998-2004, when he was not party president (appearance only); Codex may split it into 1995-01-01 to 1998-01-01 and 2004-10-20 to 2005-12-31 |
| prakash_karat | 2005-04-30 to 2015-04-01 | C01-40 CPI(M): 2011-09-10 | registry term `in_cpm_prakash_karat_200504`; his registry coordinator term `in_cpm_prakash_karat_20240929` (2024-09-29 to 2025-04-06) is not claimed, because C01-40 keeps that arrangement as claims only |
| rajnath_singh | 2005-12-31 to 2014-07-09 | C01-27 BJP: 2006-01-02, 2013-03-02 | registry terms `in_bjp_rajnath_singh_20051231` and `in_bjp_rajnath_singh_201301`; one portrait across Gadkari's presidency (appearance only); Codex may split at 2009-12-19 and 2013-01-31 |
| nitin_gadkari | 2009-12-19 to 2013-01-23 | C01-27 BJP: 2009-12-24, 2013-01-22 | registry term `in_bjp_nitin_gadkari_20091219` (ends in month 2013-01); end widened from leadership production's 2013-01-01 to the day after his last accepted observation, overlapping Rajnath Singh's window as appearance only |
| amit_shah | 2014-07-09 to 2020-01-20 | C01-27 BJP: 2014-07-09, 2016-02-02, 2019-06-17 | registry term `in_bjp_amit_shah_20140709` |
| sitaram_yechury | 2015-04-30 to 2024-09-12 | C01-40 CPI(M): 2015-05-06 (stated until 2024-09-12, his death) | registry term `in_cpm_sitaram_yechury_201504` |
| jagat_prakash_nadda | 2020-01-20 to 2026-01-20 | C01-27 BJP: from 2020-01-20; 2023-01-17, 2025-12-15 | registry term `in_bjp_jagat_prakash_nadda_20200120` |
| m_a_baby (reserve) | 2025-04-06 to 2026-09-08 | C01-40 CPI(M): 2025-05-12 | registry term `in_cpm_m_a_baby_20250406` to the 7 September 2026 cutoff |
| nitin_nabin (reserve) | 2026-01-20 to 2026-09-08 | C01-27 BJP: from 2026-01-20; 2026-08-17 | registry term `in_bjp_nitin_nabin_20260120` to the cutoff |

Production job IDs these windows close. Window-scoped IDs are computed with the pipeline's own job content and digest
(the same IDs `person_art_pipeline.py jobs --from <from> --to <to>` emits; cross-checked for the Gadkari window);
default-inventory IDs come from `python -B -X utf8 tools/avatars/person_art_pipeline.py jobs --out
D:/spheres-scratch/in-cast/jobs.json` over 1990-01-01 to 2027-01-01 (sha256
`5f83d33def208c44f4a7a31e2b7f4ff5a8b3f15cf283abefba3455b7a87041ec`, identical to the France and Japan inventories) and are
closed only for the requested windows. Both were written to `D:/spheres-scratch/in-cast/`, never into the repository.
The eight primary windows close 21 of these eight people's 23 leadership-production cartoon jobs; Karat's two
coordinator-term jobs stay open. The generated `CA-India-B00N` work-order IDs are not reused.

| person_id | window | pipeline job, window-scoped | default-inventory job (partly closed) | leadership-production cartoon jobs closed |
|---|---|---|---|---|
| harkishan_singh_surjeet | 1992-01-31 to 2005-04-01 | `harkishan_singh_surjeet-85010e8ec9dc` | `harkishan_singh_surjeet-69d8afb34e72` | `cartoon:harkishan_singh_surjeet:1992-01-31:1995-01-01:v1`, `cartoon:harkishan_singh_surjeet:1995-01-01:2000-01-01:v1`, `cartoon:harkishan_singh_surjeet:2000-01-01:2005-01-01:v1`, `cartoon:harkishan_singh_surjeet:2005-01-01:2005-04-01:v1` |
| lal_krishna_advani | 1995-01-01 to 2005-12-31 | `lal_krishna_advani-a86e3e748579` | `lal_krishna_advani-49f5deb5285b` (from 1995-01-01) | `cartoon:lal_krishna_advani:1995-01-01:1998-01-01:v1`, `cartoon:lal_krishna_advani:2004-10-27:2005-01-01:v1`, `cartoon:lal_krishna_advani:2005-01-01:2005-12-31:v1` |
| prakash_karat | 2005-04-30 to 2015-04-01 | `prakash_karat-a6ef64a014c1` | `prakash_karat-eda827228641` | `cartoon:prakash_karat:2005-04-30:2010-01-01:v1`, `cartoon:prakash_karat:2010-01-01:2015-01-01:v1`, `cartoon:prakash_karat:2015-01-01:2015-04-01:v1` (not `cartoon:prakash_karat:2024-09-29:2025-01-01:v1` or `cartoon:prakash_karat:2025-01-01:2025-04-06:v1`) |
| rajnath_singh | 2005-12-31 to 2014-07-09 | `rajnath_singh-a4531051316c` | `rajnath_singh-909e17fd4488` | `cartoon:rajnath_singh:2005-12-31:2009-12-19:v1`, `cartoon:rajnath_singh:2013-01-31:2014-07-09:v1` |
| nitin_gadkari | 2009-12-19 to 2013-01-23 | `nitin_gadkari-8a3390e6cd1e` | `nitin_gadkari-1c5c2e857ab6` | `cartoon:nitin_gadkari:2009-12-19:2010-01-01:v1`, `cartoon:nitin_gadkari:2010-01-01:2013-01-01:v1` |
| amit_shah | 2014-07-09 to 2020-01-20 | `amit_shah-cf9548aa05ad` | `amit_shah-4ed982018cb1` | `cartoon:amit_shah:2014-07-09:2015-01-01:v1`, `cartoon:amit_shah:2015-01-01:2020-01-01:v1`, `cartoon:amit_shah:2020-01-01:2020-01-20:v1` |
| sitaram_yechury | 2015-04-30 to 2024-09-12 | `sitaram_yechury-e1e306607c4e` | `sitaram_yechury-cf3ef5171ade` | `cartoon:sitaram_yechury:2015-04-30:2020-01-01:v1`, `cartoon:sitaram_yechury:2020-01-01:2024-09-12:v1` |
| jagat_prakash_nadda | 2020-01-20 to 2026-01-20 | `jagat_prakash_nadda-feaabd468258` | `jagat_prakash_nadda-15e969dcbefe` | `cartoon:jagat_prakash_nadda:2020-01-20:2025-01-01:v1`, `cartoon:jagat_prakash_nadda:2025-01-01:2026-01-20:v1` |
| m_a_baby (reserve) | 2025-04-06 to 2026-09-08 | `m_a_baby-a1a2a42a89bf` | `m_a_baby-31e12a5aba87` | `cartoon:m_a_baby:2025-04-06:2026-09-08:v1` |
| nitin_nabin (reserve) | 2026-01-20 to 2026-09-08 | `nitin_nabin-364ea9efb3bd` | `nitin_nabin-fe8bcee5af02` | `cartoon:nitin_nabin:2026-01-20:2026-09-08:v1` |

Two windows run longer than ten years (Surjeet 1992-2005, Advani 1995-2005), and Karat's and Yechury's run about ten
and nine years. Each prompt names the age the drawing targets and discloses the photograph's distance from the window;
Codex may rule a split.

### Why these people, and not the heads of state

The C01 gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.json`, last refreshed at `1fffb983`) classes
India's 60 holder observations as 29 `c01_accepted` (C01-27: 19; C01-40: 4; C01-33: 2; C01-48: 4) and 31
`c01_integrated_pending` (C01-11 prime ministers: 13; C01-15 presidents: 8; C01-20 Congress presidents: 10; merged,
historical acceptance pending). The CP1 plan allows parallel artwork only for an already-reviewed batch, so only accepted
observations qualify a person here, and the people the game shows first are **not in this batch**:

- `v_p_singh`, the country card's leader by `office_links` (since 1989-12-02): his 1990 art (1990-01-01 to 1991-01-01)
  covers his only in-period observation (Prime Minister, 1990-03-12, C01-11, acceptance pending). Leadership production
  requires no further window for him; the default-inventory job `v_p_singh-318fd416ef21` (from 1991-01-01) stays open.
- Heads of state and government: every Prime Minister and President observation is acceptance pending, and of those
  holders only `p_v_narasimha_rao` and `r_venkataraman` have registry person IDs (Chandra Shekhar, Vajpayee, Deve Gowda,
  Gujral, Manmohan Singh, Modi and the Presidents from Shankar Dayal Sharma on have none).
- Congress presidents (`rajiv_gandhi`, `p_v_narasimha_rao`, `sitaram_kesri`, `sonia_gandhi`, `rahul_gandhi`,
  `mallikarjun_kharge`): all ten observations are C01-20, acceptance pending. With Narasimha Rao as Prime Minister they
  are the first candidates for a batch 2 once Codex accepts C01-11, C01-15 and C01-20. Rajiv Gandhi's 1990-1995 art
  already covers his registry term.
- Accepted, but the planner found no qualifying photograph lead near the window (deferred, as Japan deferred Fuwa):
  `murli_manohar_joshi` (BJP 1991-12-10; his window would be 1991-12-10 to 1993-01-01, the first uncovered BJP window in
  game order; Commons holds GODL-India government photographs from 2003 on, which are not on the list, and one 2013 group
  photograph of 630x275, CC BY-SA 2.0); `kushabhau_thakre` (BJP 1999-02-27; three GODL-India files only);
  `bangaru_laxman` (BJP 2000-08-31, stated until 2001-03-14; no Commons category); `m_venkaiah_naidu` (BJP from
  2002-07-01; free Commons photographs begin in 2016, twelve or more years after his window, and most of his files are
  GODL-India). A qualifying photograph found later would make Joshi the first batch-2 candidate.
- `lalu_prasad_yadav`: an accepted C01-33 Janata Dal observation (1996-07-15) and a registry person ID, but no registry
  term (leadership production: eligibility research required), so any window needs Codex agreement; third in line after
  the two reserves.
- `s_r_bommai`: his accepted 1990-07-14 observation is inside his existing 1990-1995 art; the open continuation
  `cartoon:s_r_bommai:1995-01-01:1996-01-01:v1` holds no accepted observation.
- Arvind Kejriwal (AAP), Mayawati (BSP) and Conrad K. Sangma (NPP), all C01-48: no registry person ID.
- `jana_krishnamurthi` (his acting presidency is claims only in C01-27), `sharad_yadav` (C01-33 leads only) and
  `e_m_s_namboodiripad` (no accepted in-period observation; his 1990-1995 art covers his registry window): no accepted
  holder observation.

Selection order: the remaining people are taken in the order the game first shows their uncovered windows: Surjeet
(1992), Advani (1995), Karat (April 2005), Rajnath Singh (December 2005), Gadkari (2009), Shah (2014), Yechury (2015) and
Nadda (2020); the reserves are Baby (2025) and Nabin (2026).

Planner's photograph leads (Commons API metadata only, saved under `D:/spheres-scratch/in-cast/leads/`; nothing was
downloaded or verified, and preparation must confirm each one, including that the person is the subject):
- Surjeet: Fotokannan, 7 February 2003, CC BY-SA 3.0 (`Surjith-6.JPG`, 2136x2848); Soman, 2005, "Public domain"
  (`Hssurjeet.jpg`).
- Advani: U.S. Department of State, `Condoleezza Rice meets L.K. Advani, New Delhi, 2005.jpg` (public domain, 600x464);
  IDF Spokesperson's Unit, January 2001, CC BY-SA 3.0 (his presence unconfirmed). A search for U.S. federal photographs
  of him from 1995-2005 at higher resolution is open.
- Karat: Erfanebrahimsait, 16 March 2013, CC BY-SA 4.0 (`PrakashKarat.jpg`, 5184x3456, and its Commons crop).
- Rajnath Singh: U.S. Department of State, India 2+2 Ministerial Dialogue, 18 December 2019 (public domain, about
  7000x4000; five and a half years after his window); White House (Pete Souza), 26 January 2015 (public domain; his
  presence to be confirmed); a 28 October 2013 Narendra Modi Flickr crop (CC BY-SA 2.0, 346x638, inside the window but
  small).
- Gadkari: `Nitin Gadkari 1.JPG`, 11 December 2012, CC0, author given as "Nitin Gadkari" (provenance to verify);
  Gppande, 1 June 2014, CC BY-SA 3.0. Files credited to the Press Information Bureau but labelled CC BY 3.0, and an ETV
  video still, are rejected.
- Shah: Captgs, 16 January 2015, CC BY-SA 4.0; Duggempudi Ravinder Reddy, 30 September 2016, CC BY-SA 4.0.
- Yechury: Batthini Vinay Kumar Goud, 28 December 2019, CC0 (6000x4000, and a crop); Voiceofpunjab, 25 August 2016,
  CC BY-SA 4.0.
- Nadda: AbhiSuryawanshi, 25-27 September 2018 (New York, UN General Assembly week), CC BY-SA 4.0, high resolution;
  about sixteen months before his window.
- Baby (reserve): Fotokannan, 10 April 2025 (Kollam reception as General Secretary), CC BY-SA 4.0; Fotokannan, 6 July
  2026, CC BY-SA 4.0.
- Nabin (reserve): `Dayal Shugani with Nitin Nabin.jpg`, 23 December 2025, CC BY 4.0 (1000x667; provenance to verify).
  Bihar government files carry the Commons licence string "Attribution" and Rashtrapati Bhavan files GODL-India; neither
  is on the list.

If no qualifying photograph exists for a window, that window is recorded as not ready and a reserve may take its place.

Claude delivers:
- `docs/campaign-certification/C06/production/india/identity-review-batch-01.json`, a proposal in the shape of
  France's identity review, with the accepted holder dictionaries copied exactly;
- dated likeness photographs of the exact person, copied byte-identical into
  `spheres-web/ui/person-portraits/references/<slug>-<yyyy>-reference-v1.<ext>` (yyyy is the photograph's year), with
  licence strings from `FREE_LICENSES` only. GODL-India and other government open licences not on the list, ported
  licences such as `CC BY-SA 3.0 DE`, NC/ND terms, agency or press photos, TV or video stills, personal sites and non-free
  files are rejected. A window without a qualifying photograph is recorded as not ready; no face is invented and no other
  person substitutes;
- exact LF prompts `tools/avatars/person-prompts/<slug>-cartoon-<startyear>-v1.txt`, in the shape of the France prompts
  (style anchor `margaret-thatcher-cartoon-1990-v3.png` as image 1, STYLE ONLY; the identity reference as image 2),
  disclosing any age gap between the photograph and the window: `harkishan-singh-surjeet-cartoon-1992-v1.txt`,
  `lal-krishna-advani-cartoon-1995-v1.txt`, `prakash-karat-cartoon-2005-v1.txt`, `rajnath-singh-cartoon-2005-v1.txt`,
  `nitin-gadkari-cartoon-2009-v1.txt`, `amit-shah-cartoon-2014-v1.txt`, `sitaram-yechury-cartoon-2015-v1.txt`,
  `jagat-prakash-nadda-cartoon-2020-v1.txt`, and for the reserves `m-a-baby-cartoon-2025-v1.txt` and
  `nitin-nabin-cartoon-2026-v1.txt`;
- a render request for Codex: per job, the prompt file, the input order and the output size (1024x1536 RGB);
- after Codex returns the unchanged outputs: the visual review, the batch generation record, the `person_portraits.json`
  records and the registration receipt.

Among these ten people the registry gives a birth date only for `rajnath_singh` (1951-07-10) and a death date only for
`sitaram_yechury` (2024-09-12). Any other age in a prompt comes from a cited source and is labelled general biographical
knowledge, not C01 research; the registry is not edited.

Any reviewer string names the actual reviewer. No human approval is claimed. Country sign-off (CS-India) stays with the
user and Codex.

## Allowed files and checks

Allowed files:
- this record;
- `docs/campaign-certification/C06/production/india/`;
- `spheres-web/ui/person-portraits/references/` (new India reference files only);
- `tools/avatars/person-prompts/` (new India prompt and batch files only);
- after rendering, additive `people[pid].portraits[]` records in `spheres-web/data/person_portraits.json` and the new PNG
  files, coordinated with Codex.

Not touched: `party_leaders.json`, existing portrait records (including the 1990 art of V. P. Singh, Rajiv Gandhi,
L. K. Advani, S. R. Bommai and E. M. S. Namboodiripad), the pipeline code, `.gitattributes`, shared runtime, the task
queue, the workboard and the other C06 branches. Regenerated shared outputs are left to Codex's integration. Original
source metadata and image bytes stay outside Git under `D:/spheres-scratch/in-cast/refs/<person_id>/`, pinned by sha256;
response-header captures are not kept.

Inputs at claim (sha256 of the committed bytes at `2fd186d6`):

| path | sha256 |
|---|---|
| `docs/campaign-certification/C01/research/india.json` | `4341a287e2eafed413fc375a9bac506241bb3677f7eef7bc39e507bc50244a73` |
| `docs/campaign-certification/C01/gap-ledger/ledger.json` | `a7b3c1f1924394743cfebfc1fc7cac7ee24970f40451616273f91aafa374962b` |
| `docs/campaign-certification/C01/integrations/CLAUDE-C01-27/README.md` | `6eefa2bf2dfeb1e48c5d90b46861c10ecb22f66ff8ae8704180f69255b105bb9` |
| `docs/campaign-certification/C01/integrations/CLAUDE-C01-40/README.md` | `e418f69f8bbb79f9ef2b9b90412d2825c5a8cda9c9d89d7679418c0add9f7fa0` |
| `spheres-sim/data/party_leaders.json` | `b330e2c49fa14a615bcb50fe7e5c6b240bd6678076869699788fdd36f6fdf077` |
| `spheres-web/data/person_portraits.json` | `45fb29edd8b2b3499f47593376fe009db9a6da329d2d79e061128d2d0e333825` |
| `spheres-web/data/leadership_production_2035.json` | `42bdb24bf47cd31367b25069ca2a2381ebff44b9d5dcc55d884550777b8df7c4` |
| `tools/avatars/person_art_pipeline.py` | `b9715a5e841c7e8683bc3ec60a26754244b62d755150f06ebc0911be171487ab` |

Checks: `person_art_pipeline.py self-test` and `validate`; `cartoon_review.py --check`; `leadership_production.py check`;
`campaign_census.py --check`; `python -m unittest discover -s tools/avatars`; `workboard.py --check`; `git diff --check`.
