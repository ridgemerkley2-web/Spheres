# CLAUDE-C06-FRANCE-01: France cast, batch 1 (identities, references, prompts; Codex renders)

Owner: Claude. State: **claimed** (1 October 2026; in progress, not complete). Parent: C06 (France country cast), with
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

The exact production job IDs these windows close (from `person_art_pipeline.py jobs`) are listed in the identity review.
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
