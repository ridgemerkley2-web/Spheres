# France batch 1 (FR-CAST-B01): Claude review and registration proposal, 2 October 2026

Task `CLAUDE-C06-FRANCE-01`, branch `claude/c06-fr-01`. Base: integration tip `1cbf6200` (merged into the branch as
`a300d0bd`, tree identical to the integration tip). Reviewer: Claude (Opus 5.5). No human approval, art approval or
France country sign-off (CS-France) is claimed.

## What was reviewed

The seven PNGs Codex returned in [render-return-batch-01.json](render-return-batch-01.json):

- **Script checks:**
  - each output's bytes match the return and are byte-identical to the retained generated original;
  - each file is a 1024x1536 RGB PNG and carries a C2PA manifest;
  - the prompt and both ordered inputs match their pinned sha256;
  - the border mean is within 11 levels of `#192D34` on every channel.
- **Visual review:** Claude viewed each output beside its identity reference at full size and at 106x152 card size.
  All seven pass, with no holds. The findings are in
  [claude-visual-review-batch-01.json](claude-visual-review-batch-01.json). This review is in addition to Codex's
  [independent-render-review-20261002.json](independent-render-review-20261002.json).

## Registered (proposal for Codex integration)

Seven additive `people[pid].portraits[]` records were added to `spheres-web/data/person_portraits.json`. The records
are as follows:
- the method is `generated`, with the generator `OpenAI built-in image_gen`, and `license` is `generated`;
- each `identity_source` is `observed_portrait` and carries the exact source licence, credit and rights statement;
- each record states `derivative_license` explicitly;
- every review flag is true and names the reviewer, and each record carries an era note that infers no tenure.

The page of each reference photograph is also added to that person's `identity_sources`. No existing record is
changed; Mitterrand's 1990 portrait is untouched. Generation provenance is in
[`tools/avatars/person-prompts/france-cast-batch-01.json`](../../../../../tools/avatars/person-prompts/france-cast-batch-01.json),
in the shape of the Tonga batch records.

| person_id | window (exclusive end) | asset | derivative licence |
|---|---|---|---|
| francois_mitterrand | 1991-01-01 to 1995-05-17 | `francois-mitterrand-cartoon-1991-v1.png` | CC BY-SA 4.0 |
| michel_rocard | 1990-01-01 to 1994-06-19 | `michel-rocard-cartoon-1990-v1.png` | CC BY 2.0 |
| laurent_fabius | 1992-01-09 to 1993-04-03 | `laurent-fabius-cartoon-1992-v1.png` | CC BY-SA 4.0 |
| henri_emmanuelli | 1994-06-19 to 1995-10-14 | `henri-emmanuelli-cartoon-1994-v1.png` | CC BY 3.0 |
| alain_juppe | 1994-11-30 to 1997-06-01 | `alain-juppe-cartoon-1994-v1.png` | CC BY 4.0 |
| lionel_jospin | 1995-10-14 to 1997-06-07 | `lionel-jospin-cartoon-1995-v1.png` | CC BY 4.0 |
| francois_hollande | 1997-06-30 to 2008-11-01 | `francois-hollande-cartoon-1997-v1.png` | CC BY 2.5 |

**Juppé's references:** his `identity_source` stays the committed 17 MB original, which is the rights record. A
`generation_input` sub-record pins the 1500 px transport JPEG that was actually submitted, with its derivation.

## Effect

- **Validation:** `person_art_pipeline.py validate` returns valid, 0 errors and 524 warnings (530 before). The six
  warnings removed are the six people who had no portrait.
- **Coverage:**
  - Validated cartoon people rise from 90 to 96, and assets from 100 to 107.
  - All seven people are partial, with these missing eras outside their windows.
  - Four of nine France registry party terms for these people are now fully covered. The other five miss only
    month-precision edges of 23 to 30 days:
    - Jospin to 1997-06-30;
    - Hollande from 1997-06-01 and to 2008-11-30;
    - Juppé from 1994-11-01 and to 1997-06-30.
- **Leadership production:** pending historical art jobs fall from 699 to 686. The 13 closed cartoon jobs are:
  - Juppé: `1994-11-30:1995-01-01`, `1995-01-01:1995-10-01` and `1995-10-31:1997-06-01`.
  - Hollande: `1997-06-30:1997-11-01`, `1997-11-30:2000-01-01`, `2000-01-01:2005-01-01` and `2005-01-01:2008-11-01`.
  - Emmanuelli: `1994-06-19:1995-01-01` and `1995-01-01:1995-10-14`.
  - Fabius: `1992-01-09:1993-04-03`.
  - Jospin: `1995-10-14:1997-06-01`.
  - Rocard: `1993-04-03:1993-10-01` and `1993-10-31:1994-06-19`.

## Derived outputs (left to Codex's integration, as the handoff says)

The manifest change makes ten generated files stale. They were regenerated locally in this order to prove the chain is
clean, then restored and not committed:

1. `leadership_production.py build`
2. `cartoon_review.py`
3. `campaign_census.py`
4. `certified_gap_ledger.py`
5. `certified_boundary_matrix.py`

| file | sha256 after regeneration (prefix) |
|---|---|
| `docs/campaign-certification/C01/census.json` | `4a98469459ed6860` |
| `docs/campaign-certification/C01/gap-ledger/ledger.json` | `4392c82af56d23ef` |
| `docs/campaign-certification/C01/gap-ledger/ledger.md` | `466efa6b2c760e1e` |
| `docs/campaign-certification/C01/work-orders.json` | `a6cca2ec90caca99` |
| `docs/campaign-certification/C03/preparation/cartoon-review/cartoon-review.json` | `2019db45f5be0de1` |
| `docs/campaign-certification/C03/preparation/cartoon-review/cartoon-review.md` | `64a7ee7b963ce4f7` |
| `docs/campaign-certification/S23/preparation/boundary-matrix/README.md` | `3ed210756c89b55b` |
| `docs/campaign-certification/S23/preparation/boundary-matrix/cases-france.json` | `e62cdcc2c97f73f3` |
| `docs/campaign-certification/S23/preparation/boundary-matrix/summary.json` | `9a4b00fb20ab17d8` |
| `spheres-web/data/leadership_production_2035.json` | `b42bb31247a4ed84` |

## Checks

Results at this commit, without the derived outputs:
- **Passing:**
  - `person_art_pipeline.py self-test`: 21 tests OK.
  - `validate`: valid, 0 errors.
  - `workboard.py --check`: PASS.
  - `git diff --check`: clean.
- **Failing, by design** (stale exports):
  - `cartoon_review.py --check`
  - `leadership_production.py check`
  - `campaign_census.py --check`
  - unittest: 828 tests, with two failures, `test_committed_matrix_is_current` and
    `test_committed_output_is_current`.

With the five regenerations applied:
- all three checks above exit 0;
- `python -X utf8 -m unittest discover -s tools/avatars` passes all 828 tests.

## Open (proposals, not holds)

- **Month edges:** widen the appearance windows by at most 30 days to close the five month-precision registry edges
  listed above. This is the appearance window only; no tenure is implied.
- **Hollande:** a younger 1997-2002 variant, drawn from a closer photograph, is a possible batch 2. The registered
  portrait is one mid-window appearance.
- **Reserve:** Martine Aubry remains a prepared, unrendered reserve (2008-11-29 to 2012-09-01).
