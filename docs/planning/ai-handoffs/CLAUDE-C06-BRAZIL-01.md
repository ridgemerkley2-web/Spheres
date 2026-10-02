# CLAUDE-C06-BRAZIL-01: Brazil cast, batch 1 (identities, references, prompts; Codex renders)

Owner: Claude. State: **awaiting Codex render** (1 October 2026; batch 1 prepared; not ready_for_review, not complete). Parent: C06 (Brazil country cast), with
C03 cartoon production for these windows. Pending Codex registration and acceptance.

Origin: on 1 October 2026, after France batch 1 (`CLAUDE-C06-FRANCE-01`, branch `claude/c06-fr-01`) was prepared, the
user asked to continue the cast work. This batch runs in parallel with France's pending render. The CP1 plan
(`CP1-ACCELERATION.md`) allows parallel artwork for another already-reviewed batch while Codex owns Tonga
(`CLAUDE-C06-TONGA-01`). Generator route, as for France: **Codex renders** with its built-in image tool. Claude
prepares the identity review, dated likeness references, exact prompts and input order, then reviews and registers
the outputs Codex returns. `person_art_pipeline.py` requires the generator "OpenAI built-in image_gen", and Claude has
no such tool, so Claude never generates, edits or labels artwork itself.

Branch: `claude/c06-br-01`. Base: `2fd186d6` (current `codex/campaign-certification`); not stacked on the France
branch. Claim commit: this record's first commit on the branch.

## Bounded deliverable

Seven portraits of five people who already have registry IDs in `spheres-sim/data/party_leaders.json`, accepted C01
research and no art for these windows. The accepted research is C01-34 (PMDB/MDB and PDT national presidents) and
C01-43 (PRN/PTC/Agir national presidents) in `docs/campaign-certification/C01/research/brazil.json`. Every accepted
holder observation of each person falls inside one of that person's windows below. The `to` dates are exclusive. A
window is an appearance interval, not a tenure, and grouping observations under a person ID proves no term.

| person_id | requested window | accepted observations (attested_on) | window basis |
|---|---|---|---|
| michel_temer | 2001-09-09 to 2010-06-15 | C01-34 MDB: 2003-07-02 | registry terms `br_pmdb_michel_temer_20010909` and `br_pmdb_michel_temer_20100127`; spans Iris de Araujo's acting term as appearance only |
| michel_temer | 2010-06-15 to 2019-01-01 | C01-34 MDB: 2016-03-29 | needs Codex agreement: starts where the first window ends; the end is the next presidency's stated start in C01-10 (integrated, acceptance pending), used only as an appearance boundary |
| baleia_rossi | 2019-10-06 to 2026-09-08 | C01-34 MDB: 2019-10-17, 2025-07-16, 2026-06-07 | registry term `br_pmdb_baleia_rossi_20191006` to the 7 September 2026 cutoff |
| leonel_brizola | 1995-01-01 to 2004-06-21 | C01-34 PDT: 1997-04-11, 1999-08-26, 2004-06-02 (until 2004-06-21) | continues his existing 1990-1995 art; ends at the accepted until (the party's same-day report of his death in office), as registry term `br_pdt_leonel_brizola_1980` |
| carlos_lupi | 2004-06-30 to 2015-01-01 | C01-34 PDT: 2007-02-09 | registry term `br_pdt_carlos_lupi_200406` (month precision; start as in leadership production); split at 2015-01-01 for age |
| carlos_lupi | 2015-01-01 to 2026-09-08 | C01-34 PDT: 2021-12-21, 2025-05-21, 2026-09-04 | the same registry term to the cutoff |
| daniel_sampaio_tourinho | 2014-05-16 to 2026-09-08 | C01-43 Agir: 2014-05-16, 2018-07-25, 2020-08-07, 2021-07-23, 2022-11-11 | first accepted observation to the cutoff; contains registry term `br_prn_daniel_sampaio_tourinho_20260705` |

Production job IDs these windows close. Window-scoped IDs come from `person_art_pipeline.py jobs --from <from> --to
<to>`; default-inventory IDs come from `jobs` over 1990-01-01 to 2027-01-01 and are closed only for the requested
windows. Both were written to `D:/spheres-scratch/br-cast/`, never into the repository. Together the windows close all
fifteen leadership-production cartoon jobs of these five people. The generated `CA-Brazil-B00N` work-order IDs are not
reused.

| person_id | window | pipeline job, window-scoped | default-inventory job (partly closed) | leadership-production cartoon jobs closed |
|---|---|---|---|---|
| michel_temer | 2001-09-09 to 2010-06-15 | `michel_temer-0a0870c78f13` | `michel_temer-4a7a936add8d` | `cartoon:michel_temer:2001-09-09:2005-01-01:v1`, `cartoon:michel_temer:2005-01-01:2009-03-10:v1`, `cartoon:michel_temer:2010-01-27:2010-06-15:v1` |
| michel_temer | 2010-06-15 to 2019-01-01 | `michel_temer-555b84955d80` | `michel_temer-4a7a936add8d` | none exist |
| baleia_rossi | 2019-10-06 to 2026-09-08 | `baleia_rossi-86cb96c29aa9` | `baleia_rossi-438d8a547750` | `cartoon:baleia_rossi:2019-10-06:2020-01-01:v1`, `cartoon:baleia_rossi:2020-01-01:2025-01-01:v1`, `cartoon:baleia_rossi:2025-01-01:2026-09-08:v1` |
| leonel_brizola | 1995-01-01 to 2004-06-21 | `leonel_brizola-fa7af2b7d699` | `leonel_brizola-cbbd6332435f` | `cartoon:leonel_brizola:1995-01-01:2000-01-01:v1`, `cartoon:leonel_brizola:2000-01-01:2004-06-21:v1` |
| carlos_lupi | 2004-06-30 to 2015-01-01 | `carlos_lupi-45b22e501a80` | `carlos_lupi-1a8bd5270f54` | `cartoon:carlos_lupi:2004-06-30:2005-01-01:v1`, `cartoon:carlos_lupi:2005-01-01:2010-01-01:v1`, `cartoon:carlos_lupi:2010-01-01:2015-01-01:v1` |
| carlos_lupi | 2015-01-01 to 2026-09-08 | `carlos_lupi-7293a9e06d87` | `carlos_lupi-1a8bd5270f54` | `cartoon:carlos_lupi:2015-01-01:2020-01-01:v1`, `cartoon:carlos_lupi:2020-01-01:2025-01-01:v1`, `cartoon:carlos_lupi:2025-01-01:2026-09-08:v1` |
| daniel_sampaio_tourinho | 2014-05-16 to 2026-09-08 | `daniel_sampaio_tourinho-fde53fa22842` | `daniel_sampaio_tourinho-c02873ad7899` | `cartoon:daniel_sampaio_tourinho:2026-07-05:2026-09-08:v1` |

### Why five people and no reserve

The C01 gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.json`, last refreshed at `1fffb983`) classes
Brazil's holder observations as 17 `c01_accepted` (C01-34: twelve; C01-43: five) and 39 `c01_integrated_pending`
(C01-10 presidents, C01-17 vice-presidents and C01-22 PT presidents: merged, historical acceptance pending). Only these
five registry people hold accepted observations, so the batch takes all of them and no reserve qualifies. Not in this
batch:
- `jose_sarney`, the country card's leader by `office_links` (since 1985-03-15): his existing 1990-01-01 to 1991-01-01
  art already covers his only observation (5 January 1990, C01-10).
- `fernando_collor_de_mello` and `luiz_inacio_lula_da_silva` (presidency, C01-10; Lula also PT, C01-22), and the PT
  presidents with registry IDs (`jose_dirceu`, `jose_genoino`, `tarso_genro`, `ricardo_berzoini`, `jose_eduardo_dutra`,
  `rui_falcao`, `gleisi_hoffmann`, `edinho_silva`): the first candidates for a batch 2 once Codex accepts C01-10 and
  C01-22.
- Itamar Franco, Fernando Henrique Cardoso, Dilma Rousseff and Jair Bolsonaro: no registry person ID.
- PMDB leaders of the 1990s (`ulysses_guimaraes`, `orestes_quercia` and others): registry terms but no accepted C01
  observation.
- Already drawn for 1990: `hugo_napoleao`, `luiz_gushiken` and `leonel_brizola` (1990-1995).

Claude delivers:
- `docs/campaign-certification/C06/production/brazil/identity-review-batch-01.json`, a proposal in the shape of
  France's identity review, with the accepted holder dictionaries copied exactly;
- dated likeness photographs of the exact person, copied byte-identical into
  `spheres-web/ui/person-portraits/references/<slug>-<yyyy>-reference-v1.<ext>` (yyyy is the photograph's year), with
  licence strings from `FREE_LICENSES` only. Ported licences such as Agencia Brasil's `CC BY 3.0 BR`, NC/ND terms,
  agency or press photos and non-free files are rejected. A window without a qualifying photograph is recorded as not
  ready; no face is invented and no other person substitutes;
- exact LF prompts `tools/avatars/person-prompts/<slug>-cartoon-<startyear>-v1.txt`, in the shape of the France prompts
  (style anchor `margaret-thatcher-cartoon-1990-v3.png` as image 1, STYLE ONLY; the identity reference as image 2),
  disclosing any age gap between the photograph and the window. A person with two windows gets two prompts (for
  example `michel-temer-cartoon-2001-v1.txt` and `michel-temer-cartoon-2010-v1.txt`);
- a render request for Codex: per job, the prompt file, the input order and the output size (1024x1536 RGB);
- after Codex returns the unchanged outputs: the visual review, the batch generation record, the `person_portraits.json`
  records and the registration receipt.

Ages in the prompts use the registry birth dates where present (Temer 1940-09-23, Baleia Rossi 1972-06-09, Lupi
1957-03-16). The registry gives Brizola 1922-11-22; preparation checks that against a source before stating his age
and does not edit the registry. Tourinho has no registry birth date.

Any reviewer string names the actual reviewer. No human approval is claimed. Country sign-off (CS-Brazil) stays with the
user and Codex.

## Allowed files and checks

Allowed files:
- this record;
- `docs/campaign-certification/C06/production/brazil/`;
- `spheres-web/ui/person-portraits/references/` (new Brazil reference files only);
- `tools/avatars/person-prompts/` (new Brazil prompt and batch files only);
- after rendering, additive `people[pid].portraits[]` records in `spheres-web/data/person_portraits.json` and the new PNG
  files, coordinated with Codex.

Not touched: `party_leaders.json`, existing portrait records (including Sarney's 1990 and Brizola's 1990-1995 art), the
pipeline code, `.gitattributes`, shared runtime, the task queue, the workboard and the France branch. Regenerated shared
outputs are left to Codex's integration. Original source metadata and image bytes stay outside Git under
`D:/spheres-scratch/br-cast/refs/<person_id>/`, pinned by sha256; response-header captures are not kept.

Inputs at claim (sha256 of the committed bytes at `2fd186d6`):

| path | sha256 |
|---|---|
| `docs/campaign-certification/C01/research/brazil.json` | `a5331b1826ffa3fd02833ac05f5055b9e1852c00800766f9efc27b603ef689a6` |
| `docs/campaign-certification/C01/gap-ledger/ledger.json` | `a7b3c1f1924394743cfebfc1fc7cac7ee24970f40451616273f91aafa374962b` |
| `spheres-sim/data/party_leaders.json` | `b330e2c49fa14a615bcb50fe7e5c6b240bd6678076869699788fdd36f6fdf077` |
| `spheres-web/data/person_portraits.json` | `45fb29edd8b2b3499f47593376fe009db9a6da329d2d79e061128d2d0e333825` |
| `spheres-web/data/leadership_production_2035.json` | `42bdb24bf47cd31367b25069ca2a2381ebff44b9d5dcc55d884550777b8df7c4` |
| `tools/avatars/person_art_pipeline.py` | `b9715a5e841c7e8683bc3ec60a26754244b62d755150f06ebc0911be171487ab` |

Checks: `person_art_pipeline.py self-test` and `validate`; `cartoon_review.py --check`; `leadership_production.py check`;
`campaign_census.py --check`; `python -m unittest discover -s tools/avatars`; `workboard.py --check`; `git diff --check`.

## Preparation

Batch 1 (`BR-CAST-B01`) was prepared on 1 October 2026. Each of the seven windows had one preparation agent and one
independent verification agent; Claude assembled the results and fixed the verifiers' mechanical findings. No image was
generated, edited or labelled. The next step is Codex's render:
[render request](../../campaign-certification/C06/production/brazil/render-request-batch-01.md). The
[identity review](../../campaign-certification/C06/production/brazil/identity-review-batch-01.json) and
[README](../../campaign-certification/C06/production/brazil/README.md) carry the observations, references and job IDs.

In (six renders, four people):

| person_id | window | likeness reference (photograph date, licence) | prompt |
|---|---|---|---|
| michel_temer | 2001-09-09 to 2010-06-15 | J. Batista / Camara dos Deputados, 11 Nov 2009; CC BY 3.0 | `michel-temer-cartoon-2001-v1.txt` |
| michel_temer | 2010-06-15 to 2019-01-01 | Beto Barata/PR official portrait, May 2017 (Commons: 17 May); CC BY 2.0 | `michel-temer-cartoon-2010-v1.txt` |
| baleia_rossi | 2019-10-06 to 2026-09-08 | MDB Nacional, 8 Dec 2021; CC BY 2.0 | `baleia-rossi-cartoon-2019-v1.txt` |
| leonel_brizola | 1995-01-01 to 2004-06-21 | Sergio Neglia, 19 Sep 1998; CC BY-SA 4.0 | `leonel-brizola-cartoon-1995-v1.txt` |
| carlos_lupi | 2004-06-30 to 2015-01-01 | J. Batista / Camara dos Deputados, 11 Nov 2009; CC BY 3.0 | `carlos-lupi-cartoon-2004-v1.txt` |
| carlos_lupi | 2015-01-01 to 2026-09-08 | Geraldo Magela / Agencia Senado, 24 Oct 2023; CC BY 2.0 | `carlos-lupi-cartoon-2015-v1.txt` |

Every photograph falls inside its window. The two 2009 references are the same photograph (Lupi centre, Temer right),
committed under two per-person paths with identical bytes. The six windows close 14 of the 15 leadership-production
cartoon jobs of the five people.

Excluded: `daniel_sampaio_tourinho` (C01-43 Agir, 2014-05-16 to 2026-09-08). No dated photograph of him with a licence
string from `FREE_LICENSES` exists on Commons, Wikidata or pt.wikipedia. The verifier found his TSE 2010 candidate
photograph, but it is not on Commons, the TSE portal's licence is an unversioned 'Creative Commons Attribution', and it
is a 161x225 greyscale thumbnail from 2010 or earlier. No reference or prompt for him is in the repository, and his
three jobs (`daniel_sampaio_tourinho-fde53fa22842`, `daniel_sampaio_tourinho-c02873ad7899`,
`cartoon:daniel_sampaio_tourinho:2026-07-05:2026-09-08:v1`) stay open. No reserve exists.

Verifier findings and fixes: Baleia Rossi and Lupi's 2015 window passed outright. The other findings were fixed:
- Temer 2001: the preparer's 11 March 2009 photograph is a strict right profile. The reference was switched to the
  verifier's preferred near-frontal J. Batista photograph, copied locally from the bytes already pinned for Lupi. The
  prompt was rewritten and re-pinned, and after-window candidates were added to the ledger.
- Temer 2010: the photograph's date evidence is restated (an XMP Photoshop creation time; the month is corroborated by
  Gazeta do Povo).
- Brizola: the prompt's ears wording (re-pinned) and the job labels.
- Lupi 2004: the accepted observation, window basis and registry term were added, the face width corrected and the jobs
  labelled.
- Tourinho: a wrong 'TSE portraits begin in 2004' claim was withdrawn, and the Flickr note corrected.

Still open (Codex):
- The Temer 2010-2019 window (end from C01-10, acceptance pending), and an optional early-window split using the CC BY-SA
  2.0 photographs of 13 July 2010.
- Brizola's registry birth date (1922-01-22 in the Chamber of Deputies open data and Wikidata, against 1922-11-22 in
  the registry), plus whether to accept his reference's embedded IPTC contact fields (the photographer's own, already
  public on Commons) and its 7.66 MB size.
- The Lupi 2004 window start, which rests on leadership production and a month-precision registry term, and an optional
  Lupi split at 2020-01-01.
- For Tourinho: a ruling on the TSE portal's unversioned cc-by grant, and an optional TSE municipal-year search.
- The six prompt files have no `eol=lf` rule, and `.gitattributes` is not on this task's file list. Each pin is the LF
  (committed) sha256 and also gives the CRLF-checkout hash.

The worktree's sparse checkout now also includes `spheres-web/ui/portraits` and `spheres-web/ui/leader-art`, which
`person_art_pipeline.py self-test` reads; no file there was changed.
