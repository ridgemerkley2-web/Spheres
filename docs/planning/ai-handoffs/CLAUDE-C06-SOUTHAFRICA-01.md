# CLAUDE-C06-SOUTHAFRICA-01: South Africa cast, batch 1 (identities, references, prompts; Codex renders)

Owner: Claude. State: **claimed** (1 October 2026; in progress, not complete). Parent: C06 (South Africa country cast),
with C03 cartoon production for these windows. Pending Codex registration and acceptance.

Origin: on 1 October 2026, after France batch 1 (`CLAUDE-C06-FRANCE-01`, branch `claude/c06-fr-01`) was prepared, the
user asked to continue the cast work in parallel. This batch runs alongside France's pending render and the Brazil batch
(`CLAUDE-C06-BRAZIL-01`). The CP1 plan (`CP1-ACCELERATION.md`) allows parallel artwork for another already-reviewed
batch while Codex owns Tonga (`CLAUDE-C06-TONGA-01`). Generator route, as for France: **Codex renders** with its
built-in image tool. Claude prepares the identity review, dated likeness references, exact prompts and input order, then
reviews and registers the outputs Codex returns. `person_art_pipeline.py` requires the generator "OpenAI built-in
image_gen", and Claude has no such tool, so Claude never generates, edits or labels artwork itself.

Branch: `claude/c06-za-01`. Base: `2fd186d6` (current `codex/campaign-certification`); not stacked on the France or
Brazil branch. Claim commit: this record's first commit on the branch.

## Bounded deliverable

Eight portraits of six people who already have registry IDs in `spheres-sim/data/party_leaders.json`, accepted C01
research and no art for these windows, plus two reserves. The accepted research is C01-30 (ACDP, Freedom Front and IFP
leaders; reserves also C01-32, PAC presidents) in `docs/campaign-certification/C01/research/south-africa.json`. Every
accepted holder observation of each person falls inside one of that person's windows below. The `to` dates are
exclusive. A window is an appearance interval, not a tenure, and grouping
observations under a person ID proves no term.

| person_id | requested window | accepted observations (attested_on) | window basis |
|---|---|---|---|
| mangosuthu_buthelezi | 1995-01-01 to 2005-01-01 | C01-30 IFP: 1995-10-24 | continues his existing 1990-1995 art; registry term `za_ifp_mangosuthu_buthelezi_19750321`; split at 2005-01-01 for age (a leadership-production job boundary) |
| mangosuthu_buthelezi | 2005-01-01 to 2019-08-25 | C01-30 IFP: 2012-12-16, 2019-01-20, 2019-08-24 | the same registry term (until 2019-08-24); ends the day after his last accepted observation (his final address as President), overlapping Hlabisa's first day as appearance only |
| velenkosini_hlabisa | 2019-08-24 to 2026-09-08 | C01-30 IFP: 2019-09-12, 2023-09-09, 2024-08-10, 2026-02-01 | registry term `za_ifp_velenkosini_hlabisa_20190824` to the 7 September 2026 cutoff |
| constand_viljoen | 1994-03-31 to 2001-01-01 | C01-30 FF: 1997-08-26 | registry term `za_ff_constand_viljoen_199403` (month and year precision; bounds as in leadership production) |
| pieter_mulder | 2001-06-21 to 2016-11-13 | C01-30 FF: 2001-06-21, 2003-09-28, 2016-11-12 | first accepted observation to the day after the last; contains registry term `za_ff_pieter_mulder_2001` as leadership production dates it |
| corne_mulder | 2025-07-16 to 2026-09-08 | C01-30 FF: 2025-07-16, 2026-03-25 | registry term `za_ff_corne_mulder_20250716` to the cutoff |
| kenneth_meshoe | 1993-12-31 to 2010-01-01 | C01-30 ACDP: 1999-05-01, 2001-10-31 | registry term `za_acdp_kenneth_meshoe_199312` (month precision; start as in leadership production); split at 2010-01-01 for age |
| kenneth_meshoe | 2010-01-01 to 2026-09-08 | C01-30 ACDP: 2014-02-18, 2018-02-14, 2026-03-04 | the same registry term to the cutoff |
| pieter_groenewald (reserve) | 2016-12-27 to 2025-07-16 | C01-30 FF: 2016-12-27, 2020-04-02, 2024-08-22 | needs Codex agreement: no registry term (the registry declares the FF gap 2016-11-12 to 2025-07-16); first accepted observation to Corne Mulder's first, used only as an appearance boundary |
| mzwanele_nyhontso (reserve) | 2020-02-15 to 2026-09-08 | C01-32 PAC: 2020-02-15, 2021-07-19, 2021-12-31, 2026-08-29 (leadership disputed in the research) | needs Codex agreement: no registry term (registry PAC gap from 1996); first accepted observation to the cutoff |

Production job IDs these windows close. Window-scoped IDs are computed with the pipeline's own job content and digest
(the same IDs `person_art_pipeline.py jobs --from <from> --to <to>` emits; cross-checked for one window); default-inventory
IDs come from `jobs` over 1990-01-01 to 2027-01-01 and are closed only for the requested windows. Both were written to
`D:/spheres-scratch/za-cast/`, never into the repository. The eight primary windows close all 24 leadership-production
cartoon jobs of these six people. The generated `CA-SouthAfrica-B00N` work-order IDs are not reused.

| person_id | window | pipeline job, window-scoped | default-inventory job (partly closed) | leadership-production cartoon jobs closed |
|---|---|---|---|---|
| mangosuthu_buthelezi | 1995-01-01 to 2005-01-01 | `mangosuthu_buthelezi-355a6013bb99` | `mangosuthu_buthelezi-571ede3793fd` | `cartoon:mangosuthu_buthelezi:1995-01-01:2000-01-01:v1`, `cartoon:mangosuthu_buthelezi:2000-01-01:2005-01-01:v1` |
| mangosuthu_buthelezi | 2005-01-01 to 2019-08-25 | `mangosuthu_buthelezi-375dbc8d11f3` | `mangosuthu_buthelezi-571ede3793fd` | `cartoon:mangosuthu_buthelezi:2005-01-01:2010-01-01:v1`, `cartoon:mangosuthu_buthelezi:2010-01-01:2015-01-01:v1`, `cartoon:mangosuthu_buthelezi:2015-01-01:2019-08-24:v1` |
| velenkosini_hlabisa | 2019-08-24 to 2026-09-08 | `velenkosini_hlabisa-5f182018a18c` | `velenkosini_hlabisa-92ddaa2ee32a` | `cartoon:velenkosini_hlabisa:2019-08-24:2020-01-01:v1`, `cartoon:velenkosini_hlabisa:2020-01-01:2025-01-01:v1`, `cartoon:velenkosini_hlabisa:2025-01-01:2026-09-08:v1` |
| constand_viljoen | 1994-03-31 to 2001-01-01 | `constand_viljoen-3725bb6de879` | `constand_viljoen-f4174a5400ed` | `cartoon:constand_viljoen:1994-03-31:1995-01-01:v1`, `cartoon:constand_viljoen:1995-01-01:2000-01-01:v1`, `cartoon:constand_viljoen:2000-01-01:2001-01-01:v1` |
| pieter_mulder | 2001-06-21 to 2016-11-13 | `pieter_mulder-a245f929c659` | `pieter_mulder-dde556c97c84` | `cartoon:pieter_mulder:2001-12-31:2005-01-01:v1`, `cartoon:pieter_mulder:2005-01-01:2010-01-01:v1`, `cartoon:pieter_mulder:2010-01-01:2015-01-01:v1`, `cartoon:pieter_mulder:2015-01-01:2016-11-12:v1` |
| corne_mulder | 2025-07-16 to 2026-09-08 | `corne_mulder-e523c7f55fc0` | `corne_mulder-4a324fa308a8` | `cartoon:corne_mulder:2025-07-16:2026-09-08:v1` |
| kenneth_meshoe | 1993-12-31 to 2010-01-01 | `kenneth_meshoe-de5779eb28ae` | `kenneth_meshoe-d1adc8679437` | `cartoon:kenneth_meshoe:1993-12-31:1995-01-01:v1`, `cartoon:kenneth_meshoe:1995-01-01:2000-01-01:v1`, `cartoon:kenneth_meshoe:2000-01-01:2005-01-01:v1`, `cartoon:kenneth_meshoe:2005-01-01:2010-01-01:v1` |
| kenneth_meshoe | 2010-01-01 to 2026-09-08 | `kenneth_meshoe-a800a182982b` | `kenneth_meshoe-d1adc8679437` | `cartoon:kenneth_meshoe:2010-01-01:2015-01-01:v1`, `cartoon:kenneth_meshoe:2015-01-01:2020-01-01:v1`, `cartoon:kenneth_meshoe:2020-01-01:2025-01-01:v1`, `cartoon:kenneth_meshoe:2025-01-01:2026-09-08:v1` |
| pieter_groenewald (reserve) | 2016-12-27 to 2025-07-16 | `pieter_groenewald-d4a731ce349a` | `pieter_groenewald-04ce91397002` | none exist (no registry term) |
| mzwanele_nyhontso (reserve) | 2020-02-15 to 2026-09-08 | `mzwanele_nyhontso-d8537325291e` | `mzwanele_nyhontso-c2c1d5472e9d` | none exist (no registry term) |

Four windows run longer than eleven years (Buthelezi 2005-2019, Mulder 2001-2016, both Meshoe windows). Each prompt
names the age the drawing targets and discloses the photograph's distance from the window; Codex may rule a further
split.

### Why these six people, and not the heads of state

The C01 gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.json`, last refreshed at `1fffb983`) classes South
Africa's 85 holder observations as 48 `c01_accepted` (C01-30: 22; C01-32: 14; C01-39: 12), 32 `c01_integrated_pending`
(C01-09 heads of state, C01-16 ANC presidents and C01-21 deputy presidents: merged, historical acceptance pending; see
also `CLAUDE-C01-NEXT.md`) and 5 `s10_discovery_intake`. The CP1 plan allows parallel artwork only for an
already-reviewed batch. So the people the game shows first are **not in this batch**:

- `f_w_de_klerk`, the country card's leader by `office_links` (since 1989-08-15): his 1990-1995 art covers both of his
  observations (1990-02-02, C01-09; 1994-05-10, C01-21), and both are acceptance pending. The open continuation is
  `cartoon:f_w_de_klerk:1995-01-01:1997-09-01:v1`.
- `nelson_mandela`, `thabo_mbeki`, `jacob_zuma` and `cyril_ramaphosa` (presidency C01-09, ANC C01-16, deputy presidency
  C01-21): every observation is acceptance pending. They are the first candidates for a batch 2 once Codex accepts
  C01-09/16/21. Proposed windows covering every observation, for that later claim only: de Klerk 1995-01-01 to
  1997-09-01 (`f_w_de_klerk-0cced3f8af7c`), Mandela 1991-07-05 to 1999-06-16 (`nelson_mandela-c06c52ed048c`), Mbeki
  1994-05-25 to 2008-09-25 (`thabo_mbeki-11b2907e7522`), Zuma 1999-06-17 to 2018-02-14 (`jacob_zuma-3c08a45bb4a1`),
  Ramaphosa 2014-05-30 to 2026-09-08 (`cyril_ramaphosa-7933d998e312`).
- `geordin_hill_lewis`: his only observation (2026-04-12) is S10 discovery intake, not accepted C01.
- `narius_moloto`: accepted C01-32 observations (2018-05-25, 2019-07-12) but a disputed leadership and no registry term;
  third in line after the two reserves.
- Kgalema Motlanthe, the DA leaders Tony Leon, Helen Zille, Mmusi Maimane and John Steenhuisen (C01-39), the other PAC
  presidents and the later deputy presidents: no registry person ID.
- `marthinus_van_schalkwyk`, `clarence_makwetu`, `zach_de_beer`, `denis_worrall` and `wynand_malan`: registry terms but
  no accepted C01 observation.
- Already drawn for 1990: `oliver_tambo` and `zephania_mothopeng` (their registry windows are covered),
  `mangosuthu_buthelezi` and `f_w_de_klerk` (1990-1995).

Planner's photograph leads (Commons API metadata only; nothing was downloaded or verified, and preparation must confirm
each one): Buthelezi, Anefo/Nationaal Archief 10 June 1983 (CC0) and a Luxembourgish Wikipedia upload of 8 November
2005 (CC BY-SA 3.0, photograph date unconfirmed); Viljoen, a September 1984 Flickr crop by Ian Barbour (CC BY-SA 2.0,
383x388); Pieter Mulder, U.S. Department of Agriculture, 16 September 2013 (CC BY 2.0); Corne Mulder, Houses of the
Oireachtas, 14 June 2023 (CC BY 2.0, 301x425); Groenewald, a Commons upload dated 14 November 2018 (CC BY-SA 4.0).
For Hlabisa and Meshoe only video stills turned up, which this batch rejects; for Nyhontso nothing. A 2018 Buthelezi
photograph carries the Commons licence string "Attribution", which is not on the list. If no qualifying photograph
exists, that window is recorded as not ready and a reserve may take its place.

Claude delivers:
- `docs/campaign-certification/C06/production/south-africa/identity-review-batch-01.json`, a proposal in the shape of
  France's identity review, with the accepted holder dictionaries copied exactly;
- dated likeness photographs of the exact person, copied byte-identical into
  `spheres-web/ui/person-portraits/references/<slug>-<yyyy>-reference-v1.<ext>` (yyyy is the photograph's year), with
  licence strings from `FREE_LICENSES` only. Ported licences such as `CC BY-SA 3.0 DE`, NC/ND terms, agency or press
  photos, TV or video stills and non-free files are rejected. A window without a qualifying photograph is recorded as not
  ready; no face is invented and no other person substitutes;
- exact LF prompts `tools/avatars/person-prompts/<slug>-cartoon-<startyear>-v1.txt`, in the shape of the France prompts
  (style anchor `margaret-thatcher-cartoon-1990-v3.png` as image 1, STYLE ONLY; the identity reference as image 2),
  disclosing any age gap between the photograph and the window. A person with two windows gets two prompts
  (`mangosuthu-buthelezi-cartoon-1995-v1.txt` and `-2005-v1.txt`; `kenneth-meshoe-cartoon-1993-v1.txt` and
  `-2010-v1.txt`);
- a render request for Codex: per job, the prompt file, the input order and the output size (1024x1536 RGB);
- after Codex returns the unchanged outputs: the visual review, the batch generation record, the `person_portraits.json`
  records and the registration receipt.

The registry gives no birth date for these eight people. Any age in a prompt comes from a cited source and is labelled
general biographical knowledge, not C01 research; the registry is not edited.

Any reviewer string names the actual reviewer. No human approval is claimed. Country sign-off (CS-SouthAfrica) stays
with the user and Codex.

## Allowed files and checks

Allowed files:
- this record;
- `docs/campaign-certification/C06/production/south-africa/`;
- `spheres-web/ui/person-portraits/references/` (new South Africa reference files only);
- `tools/avatars/person-prompts/` (new South Africa prompt and batch files only);
- after rendering, additive `people[pid].portraits[]` records in `spheres-web/data/person_portraits.json` and the new PNG
  files, coordinated with Codex.

Not touched: `party_leaders.json`, existing portrait records (including Buthelezi's and de Klerk's 1990-1995 art), the
pipeline code, `.gitattributes`, shared runtime, the task queue, the workboard and the France and Brazil branches.
Regenerated shared outputs are left to Codex's integration. Original source metadata and image bytes stay outside Git
under `D:/spheres-scratch/za-cast/refs/<person_id>/`, pinned by sha256; response-header captures are not kept.

Inputs at claim (sha256 of the committed bytes at `2fd186d6`):

| path | sha256 |
|---|---|
| `docs/campaign-certification/C01/research/south-africa.json` | `79664fc62be19ec17602f2524e595ac90f7a06cc7c8bea13047c285b7d09cb4b` |
| `docs/campaign-certification/C01/gap-ledger/ledger.json` | `a7b3c1f1924394743cfebfc1fc7cac7ee24970f40451616273f91aafa374962b` |
| `docs/campaign-certification/C01/integrations/CLAUDE-C01-30/README.md` | `95f531fa06a8f8d604673eb058fee2c76e222b25e64c63e773b80b3ada31e1d9` |
| `docs/campaign-certification/C01/integrations/CLAUDE-C01-32/README.md` | `e0d8901edcf739b52d2d915d0687d5d5385cf33edac80a9b122caee310aa2afd` |
| `spheres-sim/data/party_leaders.json` | `b330e2c49fa14a615bcb50fe7e5c6b240bd6678076869699788fdd36f6fdf077` |
| `spheres-web/data/person_portraits.json` | `45fb29edd8b2b3499f47593376fe009db9a6da329d2d79e061128d2d0e333825` |
| `spheres-web/data/leadership_production_2035.json` | `42bdb24bf47cd31367b25069ca2a2381ebff44b9d5dcc55d884550777b8df7c4` |
| `tools/avatars/person_art_pipeline.py` | `b9715a5e841c7e8683bc3ec60a26754244b62d755150f06ebc0911be171487ab` |

Checks: `person_art_pipeline.py self-test` and `validate`; `cartoon_review.py --check`; `leadership_production.py check`;
`campaign_census.py --check`; `python -m unittest discover -s tools/avatars`; `workboard.py --check`; `git diff --check`.
