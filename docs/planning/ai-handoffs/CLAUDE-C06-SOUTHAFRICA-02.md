# CLAUDE-C06-SOUTHAFRICA-02: South Africa cast, batch 2 (identities, references, prompts; Codex renders)

Owner: Claude. State: **claimed** (2 October 2026; in progress, not complete). Parent: C06 (South Africa country cast),
with C03 cartoon production for these windows. Pending Codex registration and acceptance.

Origin: Claude opened batch 2 on 2 October 2026 to continue cast preparation while Codex reviews batch 1
(`CLAUDE-C06-SOUTHAFRICA-01`, branch `claude/c06-za-01`, reviewed tip `c0382bbcb92852ff6e885292bb3ecfe9a7793a75`; its
render is next). Batch 1 is not edited by this task. Generator route, as for France and batch 1: **Codex renders** with
its built-in image tool. Claude prepares the identity review, dated likeness references, exact prompts and input order;
Codex renders and returns the unchanged outputs. `person_art_pipeline.py` requires the generator "OpenAI built-in
image_gen", and Claude has no such tool, so Claude never generates, edits or labels artwork itself.

Branch: `claude/c06-za-02`. Base: `1cbf6200` (current `codex/campaign-certification`, checked against the remote on
2 October 2026; no `claude/c06-za-02` branch existed); not stacked on `claude/c06-za-01` or any other C06 branch. Claim
commit: this record's first commit on the branch.

## Bounded deliverable

Batch `ZA-CAST-B02`: seven portraits of seven people who already have registry IDs in
`spheres-sim/data/party_leaders.json`, pending leadership-production cartoon jobs and no art for these windows, plus one
reserve. Together with batch 1's eight windows, these seven cover every South Africa leadership-production cartoon job
(24 in batch 1, 17 here). The `to` dates are exclusive. A window is an appearance interval only: it establishes no
office tenure, no continuous term and no eligibility, and grouping observations under a person ID proves no term.

**Window basis.** None of the seven has an accepted C01 holder observation. The gap ledger at base `1cbf6200`
(`docs/campaign-certification/C01/gap-ledger/ledger.json`) classes South Africa's 85 holder observations as 48
`c01_accepted`, 32 `c01_integrated_pending` and 5 `s10_discovery_intake`; every state-office (C01-09), ANC-presidency
(C01-16) and deputy-state-office (C01-21) observation of these people is `c01_integrated_pending`, and van Schalkwyk and
Makwetu have no research observation at all. Each primary window is therefore **registry-derived**: it equals that
person's pending leadership-production job windows exactly, joined at job boundaries, which leadership production
derives from one `party_leaders.json` term (its `safe_art_window`). Registry terms are production registry rows, not
accepted research. The integrated-pending observations inside each window are listed as context only; they are not
copied as accepted holder objects, are not a window basis, and this claim changes no acceptance status.

| # | person_id | requested window | registry term (registry-derived basis) | C01 observations inside the window (context only, `c01_integrated_pending`) |
|---|---|---|---|---|
| 1 | f_w_de_klerk | 1995-01-01 to 1997-09-01 | `za_np_f_w_de_klerk_19890202` (NP national leader; from day 1989-02-02, until month 1997-09; safe end 1997-09-01); continues his existing 1990-1995 art | none (his C01-09 1990-02-02 and C01-21 1994-05-10 observations lie inside his existing art) |
| 2 | nelson_mandela | 1991-07-05 to 1997-12-01 | `za_anc_nelson_mandela_19910705` (ANC president; from day 1991-07-05, until month 1997-12; safe end 1997-12-01) | C01-16 ANC 1991-07-18, 1994-12-22; C01-09 President 1994-05-10 (its stated `until` 1999-06-16 lies outside the window) |
| 3 | thabo_mbeki | 1997-12-31 to 2007-12-18 | `za_anc_thabo_mbeki_199712` (from month 1997-12, safe start 1997-12-31; until day 2007-12-18) | C01-16 ANC 2002-12-20; C01-09 President 1999-06-16, 2004-04-27. His C01-16 1997-12-20 observation falls in the registry's declared December 1997 ANC gap, before the window |
| 4 | jacob_zuma | 2007-12-18 to 2017-12-18 | `za_anc_jacob_zuma_20071218` (days at both ends) | C01-16 ANC 2007-12-20, 2012-12-20; C01-09 President 2009-05-09 and from 2014-05-24 (its `until` 2018-02-14 lies outside the window) |
| 5 | cyril_ramaphosa | 2017-12-18 to 2026-09-08 | `za_anc_cyril_ramaphosa_20171218` (from day 2017-12-18, open; to the 7 September 2026 cutoff) | C01-16 ANC 2017-12-20, 2023-01-08, 2026-05-15; C01-09 President 2018-02-15, 2019-05-25, 2024-06-19 (and 2024-06-14, S10 discovery intake) |
| 6 | marthinus_van_schalkwyk | 1997-09-30 to 2005-04-01 | `za_np_marthinus_van_schalkwyk_199709` (NP / NNP national leader; month precision at both ends; the registry notes the end marks the dissolution decision, not a verified resignation or legal dissolution day) | none (no research observation) |
| 7 | clarence_makwetu | 1990-12-31 to 1996-01-01 | `za_pac_clarence_makwetu_1990` (PAC president; year precision at both ends; the registry warns not to place him before 1990-10-23) | none (no research observation; C01-32 PAC observations begin in 1998) |
| reserve | narius_moloto | 2018-05-25 to 2019-07-13 | **no registry term** (the registry declares a PAC gap from 1996 to the cutoff); basis is accepted research: C01-32 PAC president 2018-05-25 (`/organizations/38/roles/0/holder_claims/8`) and 2019-07-12 (`/organizations/38/roles/0/holder_claims/9`) in `docs/campaign-certification/C01/research/south-africa.json`, both `c01_accepted`; the research records the leadership as disputed. First accepted observation to the day after the last, appearance only; needs Codex agreement before preparation | (accepted, see basis) |

Production job IDs these windows would close. Window-scoped IDs were emitted by `python -B -X utf8
tools/avatars/person_art_pipeline.py jobs --from <from> --to <to> --out D:/spheres-scratch/za-cast2/windows/...` (de
Klerk's reproduces batch 1's deferred `f_w_de_klerk-0cced3f8af7c`); default-inventory IDs come from `jobs --out
D:/spheres-scratch/za-cast2/jobs.json` over 1990-01-01 to 2027-01-01 (sha256
`5f83d33def208c44f4a7a31e2b7f4ff5a8b3f15cf283abefba3455b7a87041ec`, identical to the India and France inventories) and
would be closed only for the requested windows. Both stay in `D:/spheres-scratch/za-cast2/`, never in the repository.
Coverage is prospective: preparation closes zero jobs; a job closes only after Codex renders and the portrait is
reviewed and registered. The generated `CA-SouthAfrica-B00N` work-order IDs are not reused.

| person_id | window | pipeline job, window-scoped | default-inventory job (projected partial coverage) | leadership-production cartoon jobs projected after rendering, review and registration |
|---|---|---|---|---|
| f_w_de_klerk | 1995-01-01 to 1997-09-01 | `f_w_de_klerk-0cced3f8af7c` | `f_w_de_klerk-eaa1fd0a4d5c` (from 1995-01-01) | `cartoon:f_w_de_klerk:1995-01-01:1997-09-01:v1` |
| nelson_mandela | 1991-07-05 to 1997-12-01 | `nelson_mandela-f3a806f5e7e5` | `nelson_mandela-cfb03d907dad` | `cartoon:nelson_mandela:1991-07-05:1995-01-01:v1`, `cartoon:nelson_mandela:1995-01-01:1997-12-01:v1` |
| thabo_mbeki | 1997-12-31 to 2007-12-18 | `thabo_mbeki-3645b8f20e55` | `thabo_mbeki-20e311a29dd4` | `cartoon:thabo_mbeki:1997-12-31:2000-01-01:v1`, `cartoon:thabo_mbeki:2000-01-01:2005-01-01:v1`, `cartoon:thabo_mbeki:2005-01-01:2007-12-18:v1` |
| jacob_zuma | 2007-12-18 to 2017-12-18 | `jacob_zuma-a27f4c8ffa89` | `jacob_zuma-cd2e63052775` | `cartoon:jacob_zuma:2007-12-18:2010-01-01:v1`, `cartoon:jacob_zuma:2010-01-01:2015-01-01:v1`, `cartoon:jacob_zuma:2015-01-01:2017-12-18:v1` |
| cyril_ramaphosa | 2017-12-18 to 2026-09-08 | `cyril_ramaphosa-1f52f30dea5a` | `cyril_ramaphosa-e49cbab19ba2` | `cartoon:cyril_ramaphosa:2017-12-18:2020-01-01:v1`, `cartoon:cyril_ramaphosa:2020-01-01:2025-01-01:v1`, `cartoon:cyril_ramaphosa:2025-01-01:2026-09-08:v1` |
| marthinus_van_schalkwyk | 1997-09-30 to 2005-04-01 | `marthinus_van_schalkwyk-749a6f217639` | `marthinus_van_schalkwyk-047e55361571` | `cartoon:marthinus_van_schalkwyk:1997-09-30:2000-01-01:v1`, `cartoon:marthinus_van_schalkwyk:2000-01-01:2005-01-01:v1`, `cartoon:marthinus_van_schalkwyk:2005-01-01:2005-04-01:v1` |
| clarence_makwetu | 1990-12-31 to 1996-01-01 | `clarence_makwetu-3ec4bf870451` | `clarence_makwetu-a358ceeb895c` | `cartoon:clarence_makwetu:1990-12-31:1995-01-01:v1`, `cartoon:clarence_makwetu:1995-01-01:1996-01-01:v1` |
| narius_moloto (reserve) | 2018-05-25 to 2019-07-13 | `narius_moloto-1f1273e7de8a` | `narius_moloto-b1f0c486e08b` | none exist (no registry term; leadership production lists him as eligibility research required) |

Era. No window exceeds ten years: Zuma's is exactly 10.0 years, Mbeki's 9.96, Ramaphosa's 8.7, van Schalkwyk's 7.5,
Mandela's 6.4, Makwetu's 5.0 and de Klerk's 2.7. Codex may split Mbeki at 2005-01-01 or Zuma at 2015-01-01 (both
leadership-production job boundaries); each part would then need its own reference and prompt. Each likeness
photograph must fall inside its window or within about five years of it, and each prompt names the age it draws and
discloses the photograph's distance from the window; ageing is illustrative. The registry gives a birth date only for
`f_w_de_klerk` (1936-03-18: 58 to 61 in his window). The approximate window ages of the others (Mandela about 72 to 79,
Mbeki 55 to 65, Zuma 65 to 75, Ramaphosa 65 to 73, van Schalkwyk 38 to 45, Makwetu 62 to 67) are general biographical
knowledge, to be cited from a named source at preparation and labelled as not C01 research; the registry is not edited.

### Why these people

Selection order: heads of state first, then party leaders by pending jobs. `f_w_de_klerk` is the country card's leader by
`office_links` (since 1989-08-15) and has the only open continuation of a 1990 portrait; then `nelson_mandela`,
`thabo_mbeki`, `jacob_zuma` and `cyril_ramaphosa`, each a later head of state whose registry ANC term carries pending
jobs. No remaining candidate has accepted C01 observations, so the two party leaders follow by pending jobs:
`marthinus_van_schalkwyk` (three) and `clarence_makwetu` (two). These seven are every South Africa person with pending
leadership-production jobs outside batch 1.

Excluded entirely, because batch 1 lists them as primary, reserve or not ready: `mangosuthu_buthelezi`,
`constand_viljoen`, `pieter_mulder`, `corne_mulder`, `velenkosini_hlabisa`, `kenneth_meshoe` (primary; the Buthelezi
2005-2019, Hlabisa and both Meshoe windows lack a usable photograph) and `pieter_groenewald`, `mzwanele_nyhontso`
(reserves). Their 24 jobs stay with batch 1.

Not used from batch 1's `deferred_batch_2`: its Mandela (to 1999-06-16), Mbeki (1994-05-25 to 2008-09-25), Zuma
(1999-06-17 to 2018-02-14) and Ramaphosa (from 2014-05-30) proposals spanned integrated-pending state and deputy-state
observations. Those extra spans have no leadership-production job and no accepted observation, so they are not claimed
here; the presidency years outside the ANC terms (for example Mandela 1997-12-01 to 1999-06-16) stay without art until
the applicable office's acceptance supports a separate claim. De Klerk's proposal equals window 1.

Reserve: `narius_moloto` was named in batch 1's handoff as third in line after its two reserves and is not in batch 1's
identity review. No second reserve qualifies: `geordin_hill_lewis` has only an S10 discovery-intake observation and no
registry term; `zach_de_beer`, `denis_worrall` and `wynand_malan` have no pending leadership-production job; Kgalema
Motlanthe and the DA federal leaders (accepted C01-39) have no registry person ID.

### Planner's photograph leads

Commons API search metadata only, saved under `D:/spheres-scratch/za-cast2/leads/`; nothing was downloaded or verified,
and preparation must confirm each lead (subject identity, licence string exactly as Commons states it, creator,
photograph date and its basis, provenance) or reject it. Commons answered several lead queries with HTTP 429; those
responses were discarded and the queries retried singly.
- de Klerk: Walter Rutishauser, 22 May 1990, CC BY-SA 4.0 (Bibliothek am Guisanplatz; `Frederik Willem de Klerk
  1990.jpg`, 1006x745, and a 603x750 version), 4.6 years before the window. A 2012 U.S. Department of State derivative
  (public domain) is 15 years after it. A closer U.S. federal or Nationaal Archief photograph from 1993-1997 should be
  sought first.
- Mandela: `Nelson Mandela 1994.jpg`, 4 October 1994, CC BY-SA 2.0, Flickr "Kingkongphoto & www.celebrity-photos.com"
  (1500x1940; Flickr provenance and review to verify), inside the window. Paul Weinberg's `Mandela voting in 1994.jpg`
  (April 1994, CC BY-SA 3.0, 538x800). The White House 1994 state-dinner files are television stills and are rejected;
  `Mandela 1991.jpg` is GFDL only and is rejected.
- Mbeki: `Thabo Mbeki - World Economic Forum Annual Meeting New York 2002.jpg`, 1 February 2002, CC BY-SA 2.0, World
  Economic Forum / swiss-image.ch, photo by Marcel Bieri (1782x2560), inside the window. Agência Brasil files with the
  Commons licence string "Attribution" are not on the list.
- Zuma: `Jacob Zuma September 2012.jpg`, 18 September 2012, CC BY 4.0, Etienne Ansotte, EC Audiovisual Service
  (4827x3218), inside the window; World Economic Forum on Africa, 10 June 2009, CC BY-SA 2.0 (portrait 2404x3150);
  Kremlin.ru, 5 August 2010, CC BY 4.0. GODL-India files are rejected.
- Ramaphosa: `Cyril Ramaphosa June 2022.jpg`, 13 June 2022, CC BY 2.0, US Embassy South Africa (2388x2824), inside the
  window; ITU Flickr, 10 September 2018, CC BY 2.0 (up to 3840x5760). RIA Novosti agency files, 'CC BY 3.0 cl' ported
  files and the 'Attribution' Moncloa file are rejected.
- van Schalkwyk: `Marthinus van Schalkwyk.jpg`, 28 January 2009, CC BY-SA 2.0, World Economic Forum / swiss-image.ch,
  photo by Remy Steinegger (5120x3413), 3.8 years after the window (age about 49 against 38 to 45).
- Makwetu: only `ClarenceMakwetu.jpg` and its crop, CC BY 4.0 on Commons but credited to "Sowetan Live" (a newspaper)
  with the date 2017-07-19 (after his death; likely an upload or publication date). A press photo with doubtful
  provenance is rejected unless preparation establishes otherwise, so this window is likely not ready.
- Moloto (reserve): no Commons file found by name.

For every BY-SA source, the eventual cartoon must carry the derivative BY-SA obligation explicitly
(`derivative_license`, `derivative_license_url`, creator attribution and an adaptation notice), as Codex required for
Viljoen. If no qualifying photograph exists for a window, that window is recorded as not ready, no face is invented, no
other person substitutes, and the reserve may take its place with Codex agreement.

Claude delivers:
- `docs/campaign-certification/C06/production/south-africa/identity-review-batch-02.json`, a proposal in the shape of
  batch 1's and France's identity reviews, recording each window's registry-derived basis, the integrated-pending
  observations as context by research location (not as accepted holders) and the reserve's two accepted C01-32 holder
  objects copied exactly;
- dated likeness photographs of the exact person, original bytes and API metadata saved under
  `D:/spheres-scratch/za-cast2/refs/<person_id>/` and pinned by sha256, copied unchanged to
  `spheres-web/ui/person-portraits/references/<slug>-<yyyy>-reference-v1.<ext>` (yyyy is the photograph's year; a name
  no existing file uses) with the copy's sha256 confirmed; licence strings from `FREE_LICENSES` only. NC/ND terms,
  ported licences, government licences not on the list (GODL-India and similar), agency or press photos, TV or video
  stills, personal sites, non-free files and doubtful 'own work' claims are rejected;
- exact LF prompts in the shape of the France prompts (style anchor `margaret-thatcher-cartoon-1990-v3.png` as image 1,
  STYLE ONLY; the identity reference as image 2; 1024x1536 RGB, full body, both hands and shoes visible, about 5.5
  heads tall, readable at 106x152, opaque flat dark teal #192D34), disclosing every age gap:
  `tools/avatars/person-prompts/f-w-de-klerk-cartoon-1995-v1.txt`, `nelson-mandela-cartoon-1991-v1.txt`,
  `thabo-mbeki-cartoon-1997-v1.txt`, `jacob-zuma-cartoon-2007-v1.txt`, `cyril-ramaphosa-cartoon-2017-v1.txt`,
  `marthinus-van-schalkwyk-cartoon-1997-v1.txt`, `clarence-makwetu-cartoon-1990-v1.txt`, and for the reserve only after
  Codex agreement `narius-moloto-cartoon-2018-v1.txt`;
- `docs/campaign-certification/C06/production/south-africa/render-request-batch-02.md` for Codex: per job, the prompt
  file and its LF sha256, the input order, 1024x1536 RGB output, up to three attempts and unchanged outputs returned;
- `docs/campaign-certification/C06/production/south-africa/README-batch-02.md` (batch 1's `README.md` is not edited).

After Codex returns outputs, the visual review, generation record, `person_portraits.json` records and registration
receipt are a later step coordinated with Codex, outside this claim's allowed files. Any reviewer string names the
actual reviewer. No user, human or Codex approval is claimed. Country sign-off (CS-SouthAfrica) stays with the user and
Codex.

Open questions for Codex:
- whether registry-derived windows suffice to render these heads of state, or whether rendering waits for C01-09 and
  C01-16 acceptance (batch 1 recorded that a batch 2 must consult each office's current acceptance record; at
  `1cbf6200` those observations are still integrated pending, so this batch does not rely on them);
- the Mbeki and Zuma splits named under Era;
- the reserve's window, which has no registry term;
- `eol=lf` rules in `.gitattributes` for the new prompts (this task may not edit `.gitattributes`).

## Allowed files and checks

Allowed files:
- this record;
- `docs/campaign-certification/C06/production/south-africa/identity-review-batch-02.json`,
  `render-request-batch-02.md` and `README-batch-02.md`;
- new South Africa batch-2 reference files in `spheres-web/ui/person-portraits/references/`;
- the new batch-2 prompt files named above in `tools/avatars/person-prompts/`.

Not touched: `person_portraits.json`, `party_leaders.json`, `.gitattributes`, every batch-1 file (its handoff,
`README.md`, `identity-review-batch-01.json`, `render-request-batch-01.md`, its references and prompts), every existing
reference and prompt (including de Klerk's 1990-1995 art), the pipeline code, shared runtime, the task queue, the
workboard and the other C06 branches. Regenerated shared outputs are left to Codex's integration. Original source
metadata and image bytes stay outside Git under `D:/spheres-scratch/za-cast2/refs/<person_id>/`; response-header
captures and pages that echo the client IP or GeoIP fields are not kept, cited or hashed.

Inputs at claim (sha256 of the committed bytes at `1cbf6200`):

| path | sha256 |
|---|---|
| `docs/campaign-certification/C01/research/south-africa.json` | `79664fc62be19ec17602f2524e595ac90f7a06cc7c8bea13047c285b7d09cb4b` |
| `docs/campaign-certification/C01/gap-ledger/ledger.json` | `39f52b0d1f26feaf2299a4b34f544aaffd31559b1b3b8d07fb5c647aac945a87` |
| `docs/campaign-certification/C01/integrations/CLAUDE-C01-32/README.md` | `e0d8901edcf739b52d2d915d0687d5d5385cf33edac80a9b122caee310aa2afd` |
| `docs/campaign-certification/C06/production/south-africa/identity-review-batch-01.json` | `7c4cd24028f05249fc3e834130abe5cabd0d6c0bda7392091dee45d8f8a821a9` |
| `docs/campaign-certification/C06/production/south-africa/render-request-batch-01.md` | `7b1532c66f42f4764951d9dab7668c5d374736676e32908a2342ab977c626d21` |
| `docs/campaign-certification/C06/reviews/claude-20261002/README.md` | `ff554e1db30c12d3d229a6b77077612d74fb8cc840f7a7496d17a81244b62e34` |
| `spheres-sim/data/party_leaders.json` | `b330e2c49fa14a615bcb50fe7e5c6b240bd6678076869699788fdd36f6fdf077` |
| `spheres-web/data/person_portraits.json` | `45fb29edd8b2b3499f47593376fe009db9a6da329d2d79e061128d2d0e333825` |
| `spheres-web/data/leadership_production_2035.json` | `42bdb24bf47cd31367b25069ca2a2381ebff44b9d5dcc55d884550777b8df7c4` |
| `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png` | `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef` |
| `tools/avatars/person_art_pipeline.py` | `b9715a5e841c7e8683bc3ec60a26754244b62d755150f06ebc0911be171487ab` |

Checks: `person_art_pipeline.py self-test` and `validate`; `cartoon_review.py --check`; `leadership_production.py check`;
`campaign_census.py --check`; `python -m unittest discover -s tools/avatars`; `workboard.py --check`; `git diff --check`.
