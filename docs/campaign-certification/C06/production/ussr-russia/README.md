# USSR / Russia cast production (C06)

Task `CLAUDE-C06-RUSSIA-01` (owner Claude), branch `claude/c06-ru-01`. State: **awaiting Codex render**; not ready for review. Claude prepares identities, dated likeness references and exact prompts; Codex renders with its built-in image tool; Claude then reviews and registers the returned outputs. Claude generates, edits and labels no image.

This covers the certified `USSR -> Russia` case, with people from both `ussr.json` and `russia.json`.

## Batch 1 (SURU-CAST-B01), prepared 2 October 2026

- [Identity review](identity-review-batch-01.json): the accepted C01-41 and C01-49 holder observations copied exactly, the requested appearance windows and their basis, pipeline job IDs, the verified likeness reference, rejected candidates, verifier dispositions, the 22 people without a registry person ID and an inline source ledger. Authority flags are all false.
- [Render request](render-request-batch-01.md): the prompt file and sha256, the input order (style anchor first, STYLE ONLY; then the identity reference), 1024x1536 RGB, up to 3 attempts, and the unchanged `exec-*.png` outputs to return.

| person_id | role | window (exclusive end) | reference | photograph date | licence | prompt |
|---|---|---|---|---|---|---|
| `mikhail_gorbachev` | primary | 1991-01-01 to 1991-12-26 | `mikhail-gorbachev-1991-reference-v1.jpg` (greyscale) | 1991-10 (uploader's statement at upload; month only) | CC BY-SA 4.0 (VRT ticket #2022050610007575) | `mikhail-gorbachev-cartoon-1991-v1.txt` |
| `nikolai_ryzhkov` | conditional reserve, **not prepared** | 1990-01-01 to 1990-11-25 | none | - | - | none (planned `nikolai-ryzhkov-cartoon-1990-v1.txt`) |

The batch has one portrait, below the six-to-eight target of the other cast batches. In the registry (`party_leaders.json`), only two people belong to this case. Of the case's 68 accepted holder observations, only Gorbachev's seven belong to a registered person. The other 61 belong to 22 people with no registry person ID, including Yeltsin, Rutskoy, Ivashko, Zyuganov, Zhirinovsky and Yavlinsky. Their art cannot be registered, and this task may not edit `party_leaders.json`. They are listed under `people_without_registry_id` for a batch 2, Yeltsin first. Ryzhkov's only research (C01-26) is integrated, but its acceptance is pending. He is therefore a conditional reserve and was not prepared.

Nobody was excluded. Gorbachev's preparation was checked by an independent verification agent, which found two mechanical problems. Both are fixed in the record and the prompt:

- **Photograph date.** The day 24 October 1991 shown on Commons was added in 2025 by a third-party editor with no source. The uploader's original statement is October 1991. The record now uses month precision, and the Commons revision history is pinned.
- **Birthmark side.** The prompt put the birthmark on the wrong side. It now places the mark on his right side of the crown and forehead (the viewer's left in the photograph) and forbids mirroring.

The assembler re-fetched the revision history and checked the birthmark side on a downscaled copy.

Age and colour notes declared in the prompt: the photograph is inside the window and shows him at 60, so no age adjustment is needed. It is black and white, so every colour is an artistic choice. The standing body, hands, trousers and shoes are extensions of a head-and-shoulders photograph. He wears no glasses in this photograph, unlike the 1990 reference.

Codex decisions requested:

- the Gorbachev window, whose end is the ledger's audit boundary;
- the licence basis (a VRT-confirmed CC BY-SA 4.0 grant from the photographer's archive);
- the month-precision date;
- spectacles continuity;
- whether Ryzhkov enters;
- registry IDs for batch 2.

See the render request.

## Evidence and privacy

Original Commons metadata (API JSON, revision history, file-page HTML) and image bytes stay outside Git under `D:/spheres-scratch/ru-cast/refs/`, pinned by sha256 in the identity review. The committed reference is a byte-identical copy (Commons SHA-1 matches). No HTTP response headers were saved, cited or hashed.

## Checks at preparation

- `python -B -X utf8 tools/avatars/person_art_pipeline.py self-test`: exit 0; 21 tests OK. This needed `git sparse-checkout add spheres-web/ui/portraits spheres-web/ui/leader-art` in this sparse worktree; without them, the test fixtures are missing and 9 tests fail.
- `python -B -X utf8 tools/avatars/person_art_pipeline.py validate`: exit 0; valid true, 0 errors, 530 warnings, 90 ready. `person_portraits.json` is unchanged, so no new portrait is registered.
- `python -X utf8 tools/planning/workboard.py --check`: PASS: 44 canonical markers, 59 bounded tasks.
- `git diff --cached --check` (staged batch files): clean.
- Also run, as the handoff lists them: `leadership_production.py check` exit 0; `campaign_census.py --check` exit 0; `cartoon_review.py --check` exit 0. The cartoon review check needed `spheres-web/ui/display-art` and `docs/campaign-certification/S10/c` in the sparse checkout.
- `python -m unittest discover -s tools/avatars`: 827 tests, 1 failure. This needed `docs/campaign-certification/C04`, `docs/campaign-certification/verification` and `docs/campaign-certification/S23/preparation/boundary-matrix` in the sparse checkout. The failure is `test_certified_gap_ledger.CheckedInLedger.test_committed_output_is_current`, and it already exists at base `2fd186d6`, before this batch. The committed `ledger.json` pins `docs/planning/ai-task-queue.json` at sha256 `50273940...` (55,081 bytes), but the base commit's queue file is `eca5a364...` (55,208 bytes). This batch changes neither file. A ledger refresh is left to Codex.

These checks show that the existing manifest and tools still pass with the new files present. They are not a visual review, a registration or a country sign-off. Sign-off for the USSR / Russia case stays with the user and Codex.
