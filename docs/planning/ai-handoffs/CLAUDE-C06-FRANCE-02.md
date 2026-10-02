# CLAUDE-C06-FRANCE-02: France cast, batch 2 (identities, references, prompts; Codex renders)

Owner: Claude. State: **claimed** (2 October 2026; in progress, not complete). Parent: C06 (France country cast), with
C03 cartoon production for these windows. Pending Codex rendering, Claude review and Codex-coordinated registration and
acceptance.

Origin: Claude opened batch 2 on 2 October 2026 to continue cast preparation while Codex reviews batch 1
(`CLAUDE-C06-FRANCE-01`, branch `claude/c06-fr-01`). Batch 1 is the model for this batch and is not edited here.
Generator route, as for batch 1: **Codex renders** with its built-in image tool. Claude prepares the identity review,
dated likeness references, exact prompts and input order. `person_art_pipeline.py` requires the generator "OpenAI
built-in image_gen", and Claude has no such tool, so Claude never generates, edits or labels artwork itself.

Branch: `claude/c06-fr-02`. Base: `1cbf6200` (current `codex/campaign-certification`, checked against the remote on
2 October 2026); not stacked on `claude/c06-fr-01` or any other C06 branch. Claim commit: this record's first commit on
the branch.

## Bounded deliverable

Batch `FR-CAST-B02`: ten appearance windows for seven people who already have registry IDs in
`spheres-sim/data/party_leaders.json` and pending leadership-production cartoon jobs in
`spheres-web/data/leadership_production_2035.json`, plus two reserve people. The `to` dates are exclusive. Each window
starts and ends on leadership-production job boundaries and together a person's windows cover all of that person's
pending leadership-production cartoon jobs; any days between jobs inside a window are appearance only.

Coverage is **prospective**. A prepared window closes no job. A job can be closed only after Codex renders the portrait,
Claude reviews it and the portrait is registered. An appearance window is an art interval. It never establishes office
tenure, a continuous term, eligibility or an affiliation, and grouping observations under a person ID proves no term.

The batch-1 people are excluded entirely: `francois_mitterrand`, `michel_rocard`, `laurent_fabius`, `henri_emmanuelli`,
`alain_juppe`, `lionel_jospin`, `francois_hollande` and `martine_aubry`.

| # | person_id | window (exclusive end) | span | window basis |
|---|---|---|---|---|
| 1 | nicolas_sarkozy | 1999-04-30 to 1999-12-04 | 7 months | registry-derived: term `fr_rpr_nicolas_sarkozy_199904_acting` (RPR acting president; from month 1999-04, until 1999-12-04) |
| 2 | francois_bayrou | 1994-12-10 to 2000-01-01 | 5.1 years | registry-derived: terms `fr_udf_fr_udf_cds_francois_bayrou_19941210_leader` (CDS), `fr_udf_fr_udf_fd_francois_bayrou_19951125_leader` (Force démocrate) and the first part of `fr_udf_fr_udf_federation_francois_bayrou_19980917_leader` (UDF) |
| 3 | francois_bayrou | 2000-01-01 to 2007-12-01 | 7.9 years | registry-derived: rest of term `fr_udf_fr_udf_federation_francois_bayrou_19980917_leader` (until 2007-12-01) |
| 4 | harlem_desir | 2011-06-30 to 2014-04-01 | 2.8 years | accepted C01-47 observation 2013-01-08; registry-derived terms `fr_ps_harlem_desir_20110630_acting`, `fr_ps_harlem_desir_201209_acting`, `fr_ps_harlem_desir_201210_leader` |
| 5 | jean_christophe_cambadelis | 2014-04-15 to 2017-06-01 | 3.1 years | accepted C01-47 observation 2014-04-19; registry-derived term `fr_ps_jean_christophe_cambadelis_20140415_leader` (until month 2017-06) |
| 6 | olivier_faure | 2018-04-07 to 2026-09-08 | 8.4 years | accepted C01-47 observations 2018-07-09 and 2023-03-11; registry-derived term `fr_ps_olivier_faure_20180407_leader` (open), ending at the 7 September 2026 research cutoff |
| 7 | marine_le_pen | 2011-01-16 to 2017-04-01 | 6.2 years | registry-derived: term `fr_fn_federal_marine_le_pen_20110116_leader` (until month 2017-04) |
| 8 | marine_le_pen | 2017-05-15 to 2021-09-01 | 4.3 years | registry-derived: term `fr_fn_federal_marine_le_pen_20170515_leader` (FN/RN; until month 2021-09) |
| 9 | jean_marie_le_pen | 1995-01-01 to 2005-01-01 | 10.0 years | registry-derived: term `fr_fn_federal_jean_marie_le_pen_1972_leader`; starts where his registered 1990-1995 cartoon ends |
| 10 | jean_marie_le_pen | 2005-01-01 to 2011-01-16 | 6.0 years | registry-derived: same term, until 2011-01-16 |
| R1 | robert_hue (reserve) | 1994-01-29 to 2003-01-01 | 8.9 years | registry-derived: terms `fr_pcf_robert_hue_19940129_leader` and `fr_pcf_robert_hue_200110_co_leader` |
| R2 | marie_george_buffet (reserve) | 2001-10-31 to 2010-06-20 | 8.6 years | registry-derived: term `fr_pcf_marie_george_buffet_200110_leader` |

"Registry-derived" means the window rests on registry terms in `party_leaders.json` as bounded by leadership
production. It is not accepted C01 research. The accepted observations are copied from
`docs/campaign-certification/C01/research/france.json` (all 48 France holder observations are `c01_accepted` in the
C01 gap ledger):

| person_id | packet | attested_on | research location in `france.json` |
|---|---|---|---|
| harlem_desir | C01-47 | 2013-01-08 | `/organizations/15/roles/0/holder_claims/9` (`fr_cnccfp_76` / `fr_ps_first_secretary`) |
| jean_christophe_cambadelis | C01-47 | 2014-04-19 | `/organizations/15/roles/0/holder_claims/10` |
| olivier_faure | C01-47 | 2018-07-09 | `/organizations/15/roles/0/holder_claims/11` |
| olivier_faure | C01-47 | 2023-03-11 | `/organizations/15/roles/0/holder_claims/12` |
| nicolas_sarkozy | C01-23 | from 2007-05-16 | `/institutions/0/roles/0/holder_claims/3` (`fr_presidency` / `fr_president`); also in `supplements/france.json` at the same pointer. **Outside his window**, see below |
| francois_bayrou | C01-38 | 2024-12-13 | `/institutions/1/roles/0/holder_claims/25` (`fr_prime_minister` / `fr_pm`); also in `supplements/france.json` at the same pointer. **Outside his windows**, see below |

Every holder object a preparation cites is copied exactly into the identity review. As C01-47 records, no reviewed
party record establishes a start or end day for any of these first secretaries. `attested_on` dates the cited record
only. Désir's 2011 and 2012 acting terms are registry terms, not accepted observations.

### Job IDs

Leadership-production job IDs come from `leadership_production_2035.json` (`people[].pending_art_job_ids`, reasons with
`nation` France). Pipeline IDs come from `python -B -X utf8 tools/avatars/person_art_pipeline.py jobs`. The default
inventory over 1990-01-01 to 2027-01-01 was written to `D:/spheres-scratch/fr-cast2/jobs.json` (sha256
`5f83d33def208c44f4a7a31e2b7f4ff5a8b3f15cf283abefba3455b7a87041ec`, identical to the batch-1 and India inventories).
Window-scoped IDs come from the same command with `--from <from> --to <to>`, written to
`D:/spheres-scratch/fr-cast2/winjobs/`. Nothing was written into the repository and `--prompts-dir` was not used.
Registration could later cover these jobs, but it is prospective only. The generated `CA-France-B00N` work-order IDs
are not reused.

| # | person_id | window | pipeline job, window-scoped | default-inventory job (partly covered) | leadership-production cartoon jobs |
|---|---|---|---|---|---|
| 1 | nicolas_sarkozy | 1999-04-30 to 1999-12-04 | `nicolas_sarkozy-a8b172db230f` | `nicolas_sarkozy-339f12bb57b9` | `cartoon:nicolas_sarkozy:1999-04-30:1999-12-04:v1` |
| 2 | francois_bayrou | 1994-12-10 to 2000-01-01 | `francois_bayrou-bf7d9a3af45a` | `francois_bayrou-9e6c66273337` | `cartoon:francois_bayrou:1994-12-10:1995-01-01:v1`, `cartoon:francois_bayrou:1995-01-01:2000-01-01:v1` |
| 3 | francois_bayrou | 2000-01-01 to 2007-12-01 | `francois_bayrou-a55dff018217` | `francois_bayrou-9e6c66273337` | `cartoon:francois_bayrou:2000-01-01:2005-01-01:v1`, `cartoon:francois_bayrou:2005-01-01:2007-12-01:v1` |
| 4 | harlem_desir | 2011-06-30 to 2014-04-01 | `harlem_desir-aca200b01281` | `harlem_desir-3d6f9a6ab957` | `cartoon:harlem_desir:2011-06-30:2011-10-01:v1`, `cartoon:harlem_desir:2012-09-30:2012-10-01:v1`, `cartoon:harlem_desir:2012-10-31:2014-04-01:v1` |
| 5 | jean_christophe_cambadelis | 2014-04-15 to 2017-06-01 | `jean_christophe_cambadelis-3460b20418fb` | `jean_christophe_cambadelis-d7bb02a4f02b` | `cartoon:jean_christophe_cambadelis:2014-04-15:2015-01-01:v1`, `cartoon:jean_christophe_cambadelis:2015-01-01:2017-06-01:v1` |
| 6 | olivier_faure | 2018-04-07 to 2026-09-08 | `olivier_faure-c9e62dbb5ac6` | `olivier_faure-155a28dab9cb` | `cartoon:olivier_faure:2018-04-07:2020-01-01:v1`, `cartoon:olivier_faure:2020-01-01:2025-01-01:v1`, `cartoon:olivier_faure:2025-01-01:2026-09-08:v1` |
| 7 | marine_le_pen | 2011-01-16 to 2017-04-01 | `marine_le_pen-7e22ea321854` | `marine_le_pen-f19ca228197a` | `cartoon:marine_le_pen:2011-01-16:2015-01-01:v1`, `cartoon:marine_le_pen:2015-01-01:2017-04-01:v1` |
| 8 | marine_le_pen | 2017-05-15 to 2021-09-01 | `marine_le_pen-f1ca2dfb17bf` | `marine_le_pen-f19ca228197a` | `cartoon:marine_le_pen:2017-05-15:2020-01-01:v1`, `cartoon:marine_le_pen:2020-01-01:2021-09-01:v1` |
| 9 | jean_marie_le_pen | 1995-01-01 to 2005-01-01 | `jean_marie_le_pen-1b822bb91223` | `jean_marie_le_pen-aa4b427d4088` (from 1995-01-01) | `cartoon:jean_marie_le_pen:1995-01-01:2000-01-01:v1`, `cartoon:jean_marie_le_pen:2000-01-01:2005-01-01:v1` |
| 10 | jean_marie_le_pen | 2005-01-01 to 2011-01-16 | `jean_marie_le_pen-a976a0805aa1` | `jean_marie_le_pen-aa4b427d4088` (from 1995-01-01) | `cartoon:jean_marie_le_pen:2005-01-01:2010-01-01:v1`, `cartoon:jean_marie_le_pen:2010-01-01:2011-01-16:v1` |
| R1 | robert_hue | 1994-01-29 to 2003-01-01 | `robert_hue-50cf07d7fea9` | `robert_hue-17e948436e47` | `cartoon:robert_hue:1994-01-29:1995-01-01:v1`, `cartoon:robert_hue:1995-01-01:2000-01-01:v1`, `cartoon:robert_hue:2000-01-01:2001-10-01:v1`, `cartoon:robert_hue:2001-10-31:2003-01-01:v1` |
| R2 | marie_george_buffet | 2001-10-31 to 2010-06-20 | `marie_george_buffet-b092693aa3ae` | `marie_george_buffet-f0e92709817c` | `cartoon:marie_george_buffet:2001-10-31:2005-01-01:v1`, `cartoon:marie_george_buffet:2005-01-01:2010-01-01:v1`, `cartoon:marie_george_buffet:2010-01-01:2010-06-20:v1` |

The ten primary windows target 21 leadership-production cartoon jobs, all of the seven people's pending jobs. The two
reserves hold seven more.

### Era and splits

One portrait should span no more than about ten years. Its photograph should fall inside the window or within about
five years of it. Longer spans are split at leadership-production job boundaries, and each window gets its own
reference:
- Bayrou's 13-year span (1994-12-10 to 2007-12-01) is split at 2000-01-01. That gives 5.1 and 7.9 years; a split at
  2005-01-01 would leave a 10.1-year first window.
- Marine Le Pen's 10.6-year span is split at the registry gap between her two terms (2017-04-01 to 2017-05-15). The
  registry's acting terms of `jean_francois_jalkh` (from 2017-04-25) and `steeve_briois` (to 2017-05-15) fall inside
  that gap; neither is in this batch.
- Jean-Marie Le Pen's 16-year span is split at 2005-01-01 into exactly 10.0 and 6.0 years. A split at 2000-01-01 would
  leave an 11-year second window.
- Faure's single window runs 8.4 years, up to the research cutoff. The registry term is open and no successor is
  asserted.

Each prompt names the age the drawing targets and discloses the gap between the photograph and the window. Ageing is
illustrative. The registry gives birth dates only for `francois_bayrou` (1951-05-25), `olivier_faure` (1968-08-18) and
`jean_marie_le_pen` (1928-06-20; died 2025-01-07). Any other age in a prompt comes from a cited source and is labelled
general biographical knowledge, not C01 research. The registry is not edited.

### Why these people

Batch-1 people are excluded. The rest of the order comes from the claim rules:

1. **Heads of state and government.** The only France `office_links` entry is `francois_mitterrand`, who is in batch 1.
   `nicolas_sarkozy` (President, C01-23) and `francois_bayrou` (Prime Minister, C01-38) are the remaining heads with
   registry IDs and pending jobs, so they come first. `jacques_chirac` has no pending job; his 1990-1995 art covers his
   only registry term. `valery_giscard_d_estaing` was head of state only before 1990.
2. **Accepted C01 holder observations.** Next come `harlem_desir`, `jean_christophe_cambadelis` and `olivier_faure`,
   the three C01-47 PS first secretaries. Apart from Sarkozy and Bayrou, they are the only people outside batch 1 who
   have accepted France holder observations, registry IDs and pending jobs.
3. **Major party leaders with the most pending jobs.** Among the remaining people, four each have four pending jobs:
   `jean_marie_le_pen`, `marine_le_pen`, `robert_hue` and `herve_de_charette`. De Charette's registry terms are for
   Perspectives et Réalités and the PPDF inside the UDF, both minor components. Both Le Pens need two windows, so `robert_hue` becomes the first reserve. That keeps the batch at ten
   windows. `marie_george_buffet` (three jobs, PCF) is the second reserve.

Not chosen, all registry-derived only: `fabien_roussel` (three jobs), `jordan_bardella`, `pierre_laurent`,
`michele_alliot_marie`, `philippe_seguin`, `valery_giscard_d_estaing` (one job, 1995-01-01 to 1996-03-31) and the UDF
component leaders. Each is a batch-3 candidate.

Two accepted observations fall **outside** this batch's windows. Sarkozy's C01-23 presidency observation (from
2007-05-16) and Bayrou's C01-38 Prime Minister observation (2024-12-13) have no leadership-production cartoon job. The
production inventory builds France's historical jobs only from party terms and the 1990 executive seed. Neither period
is claimed here. Art for them would need its own window claim and Codex agreement, and this batch implies no coverage
of either office.

A reserve is prepared only in place of a primary window that is excluded, for example because no qualifying photograph
exists. The batch never exceeds ten prepared windows.

### Planner's photograph leads

These leads come from Commons API metadata only, saved under `D:/spheres-scratch/fr-cast2/leads/`. Nothing was
downloaded or verified. Preparation must confirm each lead, including that the person is the subject and the exact
licence string. Two requests were refused with a rate-limit message. Their responses were deleted, and the planner
backed off before the next request.
- Sarkozy (1): EC Audiovisual Service (Christian Lambiotte), 7 January 2003, CC BY 4.0. The file is `Nicolas Sarkozy,
  French Minister of the Interior - 2003.jpg` (2000x1312), with two group frames of the same visit. It falls about three
  years after the window. `Nicolas Sarkozy 1999.jpg` (EP Multimedia Centre, 1999) is inside the window, but Commons
  gives the licence string "European Parliament", which is not on the list.
- Bayrou (2): EC Audiovisual Service (Christian Lambiotte), 17 July 2001, CC BY 4.0. The file is `Visit by François
  Bayrou, President of the UDF to the EC - 2001.jpg` (2000x1312), with a Commons crop. It falls about 18 months after
  the window. Commons has no Bayrou year category before 2001.
- Bayrou (3): Enrique Dans (Flickr), 12 December 2006, CC BY 2.0, `Loïc avec François Bayrou (324736447).jpg`
  (2816x2112, with another person; check FlickreviewR). Also Solensean, July 2006, CC BY-SA 2.5, `François Bayrou.jpg`
  (1658x1110). `BayrouEM.jpg` (Antonin Borgeaud, 2006, "Public domain", noted as also published on bayrou.fr) needs a
  provenance check before use.
- Jean-Marie Le Pen (9): own-work files by Novosti yu, dated 24 January, 20 February and 1 May 2004, licensed CC0,
  CC BY 4.0 or CC BY-SA 3.0. One is `Jean Marie LE PEN 01.jpg` (1312x2000, CC0, 1 May 2004). Own-work provenance must be
  checked. There is also Semnoz, September 2001, CC BY-SA 3.0 ("Photography given by OP to Semnoz"; provenance to verify;
  539x728). EP portraits from 1999 ("Attribution") and 2004 ("European Parliament") carry strings that are not on the
  list.
- Jean-Marie Le Pen (10): Commons year categories exist for 2005, 2007, 2008 and 2010; not queried. Marine Le Pen (7, 8):
  Commons year categories exist for every year from 2011 to 2019; not queried.
- Faure (6): a Commons search lists candidates such as `Portrait d'Olivier Faure.jpg`, `Olivier Faure PSE-CARCA--1194.jpg`
  and `JDE2024 8136.jpg`-`8138.jpg`; licences and dates not checked. Désir (4) and Cambadélis (5): not queried.

If no qualifying photograph exists for a window, that window is recorded as not ready. No face is invented and no other
person substitutes.

### What Claude delivers

- `docs/campaign-certification/C06/production/france/identity-review-batch-02.json`, a proposal in the shape of the
  batch-1 identity review. It carries the accepted holder objects copied exactly, each window's basis and job IDs, the
  verified references, rejected candidates and an inline source ledger. Authority flags are all false.
- `docs/campaign-certification/C06/production/france/render-request-batch-02.md`, in the shape of the batch-1 render
  request. Per job it gives the prompt file and its LF sha256, the input order (image 1 the style anchor
  `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png`, sha256
  `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`, STYLE ONLY; image 2 the identity reference),
  1024x1536 RGB, up to three attempts, and unchanged outputs to return.
- `docs/campaign-certification/C06/production/france/README-batch-02.md`.
- Dated likeness photographs of the exact person, copied byte-identical into
  `spheres-web/ui/person-portraits/references/<slug>-<yyyy>-reference-v1.<ext>`. Here yyyy is the photograph's year,
  and the name must be one no existing file uses. Licence strings must match `FREE_LICENSES` exactly as Commons states
  them: public domain, PDM-1.0, CC0 1.0, CC BY 1.0-4.0 or CC BY-SA 1.0-4.0. The following are rejected: NC/ND terms;
  ported licences (for example CC BY-SA 3.0 DE); government licences not on the list; Commons strings such as
  "Attribution" or "European Parliament"; agency or press photos; TV or video stills; personal sites; non-free files;
  and own-work claims with doubtful provenance. Each reference records the creator, credit, source page URL and the
  photograph's own date with its basis: a capture date is kept distinct from upload, EXIF, archive or PDF dates, and no
  hard day is given that the source does not support. For a BY-SA source, the derivative BY-SA obligation is stated
  explicitly.
- Exact LF prompts in `tools/avatars/person-prompts/`, in the shape of the batch-1 France prompts:
  `nicolas-sarkozy-cartoon-1999-v1.txt`, `francois-bayrou-cartoon-1994-v1.txt`, `francois-bayrou-cartoon-2000-v1.txt`,
  `harlem-desir-cartoon-2011-v1.txt`, `jean-christophe-cambadelis-cartoon-2014-v1.txt`, `olivier-faure-cartoon-2018-v1.txt`,
  `marine-le-pen-cartoon-2011-v1.txt`, `marine-le-pen-cartoon-2017-v1.txt`, `jean-marie-le-pen-cartoon-1995-v1.txt` and
  `jean-marie-le-pen-cartoon-2005-v1.txt`. A reserve, if used, gets `robert-hue-cartoon-1994-v1.txt` or
  `marie-george-buffet-cartoon-2001-v1.txt`. Each prompt describes appearance honestly, discloses every age gap and has
  its LF sha256 pinned.

The planned output paths after review are `spheres-web/ui/person-portraits/<slug>-cartoon-<startyear>-v1.png`. Codex
returns the PNGs. The visual review, the `person_portraits.json` records and the registration receipt are a later
step, coordinated with Codex, and are not files of this task.

Any reviewer string names the actual reviewer. This record claims no user, human or Codex approval. Country sign-off
(CS-France) stays with the user and Codex.

## Allowed files and checks

Allowed files (additions only):
- this record;
- `docs/campaign-certification/C06/production/france/identity-review-batch-02.json`,
  `docs/campaign-certification/C06/production/france/render-request-batch-02.md` and
  `docs/campaign-certification/C06/production/france/README-batch-02.md`;
- new reference files in `spheres-web/ui/person-portraits/references/`;
- new prompt files in `tools/avatars/person-prompts/`.

Not touched: `person_portraits.json`, `party_leaders.json`, `.gitattributes`, every batch-1 file (including the France
`README.md`, `identity-review-batch-01.json`, `render-request-batch-01.md`, `render-return-batch-01.json` and the batch-1
prompts and references), all existing references and prompts (including the 1990 art of Jean-Marie Le Pen), the
pipeline code, shared runtime, the task queue, the workboard and the other C06 branches. The new prompts have no
`text eol=lf` rule, because `.gitattributes` is outside this task. Each pin is therefore the LF (committed) sha256, and
Codex may add LF rules at integration as it did for batch 1.

Original source metadata and image bytes stay outside Git under `D:/spheres-scratch/fr-cast2/refs/<person_id>/` and are
pinned by sha256. Downloads use curl with the User-Agent `SPHERES-research/1.0 (historical game research)` and no
personal contact data. Requests go one at a time and back off on HTTP 429. Response-header captures and pages that echo
the client IP or GeoIP are not kept.

Inputs at claim (sha256 of the committed bytes at `1cbf6200`):

| path | sha256 |
|---|---|
| `docs/campaign-certification/C01/research/france.json` | `207d4094a0594e6754f0a3ddc7fc9ab89f3611780e081dda2333979e9434288b` |
| `docs/campaign-certification/C01/research/supplements/france.json` | `9d64a41d80400befac9b8541b6c66b5f96da5bce5426c01100f4bfccef6ae27b` |
| `docs/campaign-certification/C01/reviews/CLAUDE-C01-47-20261001/README.md` | `6a371f7989e6f9c1e7cf7bb05019b534bd878fd9545f36e7005f5eba31d0b52b` |
| `docs/campaign-certification/C01/integrations/CLAUDE-C01-23/README.md` | `68bfa15292b62ec2a6326b10bc7253170d14514a917515e53884ea12d9bd48f7` |
| `docs/campaign-certification/C01/integrations/CLAUDE-C01-38/README.md` | `232c78225e45443303a77cebd7da8b044e1b157d28f2bcc640bca4d3784cad23` |
| `docs/campaign-certification/C01/gap-ledger/ledger.json` | `39f52b0d1f26feaf2299a4b34f544aaffd31559b1b3b8d07fb5c647aac945a87` |
| `spheres-sim/data/party_leaders.json` | `b330e2c49fa14a615bcb50fe7e5c6b240bd6678076869699788fdd36f6fdf077` |
| `spheres-web/data/person_portraits.json` | `45fb29edd8b2b3499f47593376fe009db9a6da329d2d79e061128d2d0e333825` |
| `spheres-web/data/leadership_production_2035.json` | `42bdb24bf47cd31367b25069ca2a2381ebff44b9d5dcc55d884550777b8df7c4` |
| `tools/avatars/person_art_pipeline.py` | `b9715a5e841c7e8683bc3ec60a26754244b62d755150f06ebc0911be171487ab` |
| `docs/campaign-certification/C06/production/france/identity-review-batch-01.json` | `d26449777123452d4fd1650600252d911c162e22b762e659e98e3392825f7a06` |
| `docs/campaign-certification/C06/production/france/render-request-batch-01.md` | `643c7b0888c0b8b4768725dc7969d49a22af7e4518aa8c057ffe818ce3ffdb6e` |
| `docs/campaign-certification/C06/production/france/render-return-batch-01.json` | `cd22f06e567f50238038b9acd6c8e30d834b21d4ffbf83ac4d4a8a6410e0e41f` |
| `docs/campaign-certification/C06/reviews/claude-20261002/README.md` | `ff554e1db30c12d3d229a6b77077612d74fb8cc840f7a7496d17a81244b62e34` |
| `tools/avatars/person-prompts/francois-hollande-cartoon-1997-v1.txt` | `1453824cbd45beee1e84333ee0baa3f0c963d4605de7f31270ca999a8f6ed2b0` |
| `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png` | `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef` |

Checks:
- `person_art_pipeline.py self-test` and `validate`;
- `cartoon_review.py --check`;
- `leadership_production.py check`;
- `campaign_census.py --check`;
- `python -m unittest discover -s tools/avatars`;
- `workboard.py --check`;
- `git diff --check`;
- a byte comparison of every reference copy against its pinned original;
- an LF sha256 of every new prompt.

These checks show that existing tools still pass with the new files present. They are not a visual review, a
registration or a country sign-off.
