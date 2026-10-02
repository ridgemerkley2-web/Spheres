# CLAUDE-C06-FRANCE-01: France cast, batch 1 (identities, references, prompts; Codex renders)

Owner: Claude. State: **ready_for_review** (2 October 2026; seven returned renders reviewed by Claude and proposed for registration; not complete). Parent: C06 (France country cast), with
C03 cartoon production for these windows. Pending Codex registration and acceptance.

Origin: the user chose on 1 October 2026 to start the France cast after Codex closed all C01 research tasks. The CP1
plan (`CP1-ACCELERATION.md`) allows parallel artwork for another already-reviewed batch while Codex owns Tonga
(`CLAUDE-C06-TONGA-01`). Generator route chosen by the user: **Codex renders** with its built-in image tool. Claude
prepares the identity review, dated likeness references, exact prompts and input order, then reviews and registers
the outputs Codex returns. `person_art_pipeline.py` requires the generator "OpenAI built-in image_gen", and Claude has
no such tool, so Claude never generates or labels artwork itself.

Branch: `claude/c06-fr-01`. Base: `2fd186d6` (current `codex/campaign-certification`); not stacked. Claim commit: this
record's first commit on the branch.

## Bounded deliverable

Seven portraits, plus one alternate, for people who already have registry IDs in `spheres-sim/data/party_leaders.json`,
accepted C01 research (C01-23, C01-37, C01-38, C01-47) and no art for these windows. The `to` dates are exclusive.

| person_id | requested window | research basis |
|---|---|---|
| francois_mitterrand | 1991-01-01 to 1995-05-17 | C01-23 (country card; current art ends 1991-01-01; window needs Codex agreement) |
| michel_rocard | 1990-01-01 to 1994-06-19 | C01-37 PM; C01-47 PS |
| laurent_fabius | 1992-01-09 to 1993-04-03 | C01-47 |
| henri_emmanuelli | 1994-06-19 to 1995-10-14 | C01-47 |
| alain_juppe | 1994-11-30 to 1997-06-01 | C01-37 PM |
| lionel_jospin | 1995-10-14 to 1997-06-07 | C01-47; C01-37 |
| francois_hollande | 1997-06-30 to 2008-11-01 | C01-47 |
| martine_aubry (alternate) | 2008-11-29 to 2012-09-01 | C01-47 |

The exact production job IDs these windows could cover after approved registration (from `person_art_pipeline.py jobs`) are listed in the identity review.
The generated `CA-France-B00N` work-order IDs are not reused.

Claude delivers:
- `docs/campaign-certification/C06/production/france/identity-review-batch-01.json`, a proposal in the shape of
  Tonga's identity review;
- dated likeness references, copied byte-identical into `spheres-web/ui/person-portraits/references/`, with free
  licences from `FREE_LICENSES` only;
- exact prompts in `tools/avatars/person-prompts/`;
- a render request for Codex: per job, the prompt file, the input order (style anchor first, marked style-only; then
  the identity reference) and the output size (1024x1536 RGB);
- after Codex returns the unchanged outputs: the visual review, the batch generation record, the `person_portraits.json`
  records and the registration receipt.

Any reviewer string names the actual reviewer. No human approval is claimed. Country sign-off (CS-France) stays with the
user and Codex.

## Allowed files and checks

Allowed files:
- this record;
- `docs/campaign-certification/C06/production/france/`;
- `spheres-web/ui/person-portraits/references/` (new France reference files only);
- `tools/avatars/person-prompts/` (new France prompt and batch files only);
- after rendering, additive `people[pid].portraits[]` records in `spheres-web/data/person_portraits.json` and the new PNG
  files, coordinated with Codex.

Not touched: `party_leaders.json`, existing portrait records (including Mitterrand's 1990 art), the pipeline code,
shared runtime, the task queue and the workboard. Regenerated shared outputs are left to Codex's integration.

Checks: `person_art_pipeline.py self-test` and `validate`; `cartoon_review.py --check`; `leadership_production.py check`;
`campaign_census.py --check`; `python -m unittest discover -s tools/avatars`; `workboard.py --check`; `git diff --check`.

## Preparation

Batch 1 (`FR-CAST-B01`) was prepared on 1 October 2026. Each person had one preparation agent and one independent
verification agent; Claude assembled the results and fixed the verifiers' mechanical findings. No image was generated,
edited or labelled. The next step is Codex's render:
[render request](../../campaign-certification/C06/production/france/render-request-batch-01.md). The
[identity review](../../campaign-certification/C06/production/france/identity-review-batch-01.json) and
[README](../../campaign-certification/C06/production/france/README.md) carry the observations, references and job IDs.

In (seven primary renders):

| person_id | likeness reference (photograph date, licence) | prompt |
|---|---|---|
| francois_mitterrand | Gorup de Besanez, dated 1994 on Commons but probably about 1990-1993; CC BY-SA 4.0 | `francois-mitterrand-cartoon-1991-v1.txt` |
| michel_rocard | Jean Weber / INRA, 1991; CC BY 2.0 | `michel-rocard-cartoon-1990-v1.txt` |
| laurent_fabius | Andre Cros / Archives municipales de Toulouse, 28 Aug 1984; CC BY-SA 4.0 | `laurent-fabius-cartoon-1992-v1.txt` |
| henri_emmanuelli | Kenji-Baptiste OIKAWA, May 2005; CC BY 3.0 | `henri-emmanuelli-cartoon-1994-v1.txt` |
| alain_juppe | Tatarstan.ru, 12 Feb 1996; CC BY 4.0 | `alain-juppe-cartoon-1994-v1.txt` |
| lionel_jospin | Benoit Bourgeois / EC Audiovisual Service, 12 or 13 Oct 1998; CC BY 4.0 | `lionel-jospin-cartoon-1995-v1.txt` |
| francois_hollande | Marie-Lan Nguyen, 29 May 2007; CC BY 2.5 | `francois-hollande-cartoon-1997-v1.txt` |

Reserve: martine_aubry (Marie-Lan Nguyen, 11 Mar 2010; CC BY 3.0; `martine-aubry-cartoon-2008-v1.txt`) is fully prepared
but not part of the render, because none of the seven was excluded.

Excluded: nobody. The verifiers passed Rocard and Hollande outright. Their other findings were fixed in the records and
prompts: photograph dates (Mitterrand, Jospin), a misidentified bystander and a suit colour (Fabius), an age-gap figure
(Emmanuelli), and provenance, hands wording and window basis (Juppe).

Still open:
- The prompt files have no `eol=lf` rule, and `.gitattributes` is not on this task's file list. Each pin is the LF
  (committed) sha256 and also gives the CRLF-checkout hash.
- An optional EC Audiovisual search for a 1989-1992 Fabius photograph.
- Codex rulings: the Mitterrand window and reference-year naming, the Juppe window start and 17 MB reference, and a
  possible Hollande split at 2005-01-01.

## Render handback, 2 October 2026

Codex completed all seven primary outputs. Use [render-return-batch-01.json](../../campaign-certification/C06/production/france/render-return-batch-01.json) as the generation provenance; it records the exact submitted LF prompts and ordered references, including Juppé’s smaller transport input. Review the unchanged PNGs, preserve BY-SA for Mitterrand/Fabius, and coordinate the shared manifest before registration. Do not regenerate these seven unless a specific visual review fails. The earlier open-decision list is historical; decisions and remaining limits are resolved in the return. No country sign-off or production job closure is implied.

## Review and registration, 2 October 2026

Claude reviewed all seven returned PNGs, and all seven pass with no holds.
- Script checks confirmed bytes, generated-original identity, C2PA presence, size and mode, prompt and input hashes,
  and background.
- Claude viewed each output beside its reference at full size and at card size.

Claude then added seven additive records to `person_portraits.json`, with explicit source and derivative licences, and
a batch generation record, `tools/avatars/person-prompts/france-cast-batch-01.json`. The
[registration receipt](../../campaign-certification/C06/production/france/registration-batch-01.md) gives the
coverage effect (13 leadership-production cartoon jobs close), the checks and the ten derived outputs. Those outputs go
stale by design: Codex regenerates them at integration. A local regeneration proved the chain clean, with all 828
avatar tests passing. The
[visual review](../../campaign-certification/C06/production/france/claude-visual-review-batch-01.json) records the
per-portrait findings.

Open proposals, none of them holds:
- close five month-precision registry edges by widening appearance windows by at most 30 days;
- a younger Hollande variant for a later batch;
- the unrendered Aubry reserve.

No human approval or CS-France sign-off is claimed.
