# CLAUDE-C01-GAPS-01 — independent bounded audit review

**Decision: ready for integration after repairs.** Codex reviewed submission
`b4d953964d7f1ab720c3fe72a62f74437bc81edf` and qualified its repaired implementation
at `12ee48f1ac1e73c589ed37524bfc49dbf37202e1` in a separate worktree. The parent
integration task must merge the submission and repair, update the task queue,
regenerate the ledger, and rerun `--check` before recording the task as complete.
This accepts a planning audit of known inputs; it does not accept pending research,
prove an exhaustive country census, or close C01, G4, CP1, or a campaign session.

## Review and repairs

The submitted changes are restricted to the seven assigned handoff, gap-ledger,
and tool paths. No runtime, campaign save, leader roster, research packet, source
extract, census generator, or research-index generator is changed.

The ledger keeps all eight certified cases, party versus executive roles,
coalition components, uncertain identity matches, acting terms, isolated
attestations, and in-flight work separate. Its limits are stated alongside the
results. The 2026-09-07 historical cutoff remains frozen. The 1,329 pinned source
attributions were independently regenerated from the first-add/source-id Git
history and match exactly; this verifies attribution, not historical truth.

Two defects were repaired:

1. Matching directory names previously promoted packets to accepted status without
   reading an acceptance decision. The generator now requires an explicit accepted
   decision in the integration review and includes each review in its pinned inputs.
   Missing or pending decisions fail closed; a changed scope invalidates the ledger.
2. The integration-record Markdown hash previously varied with Windows checkout
   line endings and differed from the LF Git blob. Markdown input hashes and byte
   counts now explicitly use UTF-8 with LF endings. JSON/source hashes remain over
   exact bytes. A CRLF/LF regression verifies equivalent checkouts regenerate alike.

The repairs leave the audit results unchanged: 35 represented party rows, 50 party
chains with gaps, 58 research roles (49 with unresolved days), and 12 proposed
batches containing 91 items. These are work items, not completed historical coverage.

## Validation

The [manifest](manifest.json) records exact log hashes and commands. Passing checks:

- Gap-ledger regeneration and 22 unit tests. All three added regressions fail on
  the original submission and pass after the repairs; the negative-control log's
  `FAILED (failures=3)` is the expected demonstration, not a remaining defect.
- Research-index regeneration (1,329 sources and 3,731 claims) and 79 research tests.
- 207 C01 research tests; census regeneration and 16 campaign tests.
- 11 atlas Node tests; workboard validation and `git diff --check`.

The first census run lacked game data in the sparse checkout. Its setup failures
are retained; after adding the required checked-in data/source paths, both checks
passed. An initial new line-ending fixture doubled an existing CRLF sequence;
its failed logs are retained. The corrected fixture explicitly writes LF before
testing a CRLF checkout. These were test setup defects, not modified campaign data.

No new primary historical source review or browser/runtime qualification is claimed.
The planning audit's own executable behavior and provenance were the review scope.

## Integration note

The current integration queue has moved since this submission's pinned base.
After merging and updating that queue, run:

```text
python -B -X utf8 tools/avatars/certified_gap_ledger.py
python -B -X utf8 tools/avatars/certified_gap_ledger.py --check
python -B -X utf8 -m unittest discover -s tools/avatars -p test_certified_gap_ledger.py
python -B -X utf8 tools/planning/workboard.py --check
```

The eight explicitly tracked in-flight research/source-repair targets must remain
excluded from new batches. If one is integrated or newly accepted simultaneously,
review the generator's `IN_FLIGHT`/attribution classification before regeneration;
do not bypass its drift failure or infer acceptance from a branch name.

## Accepted integration — 28 September UTC

Codex merged reviewed tip `157aac55` with the repaired implementation, resolved
the handoff in favor of this reviewed decision, marked the bounded queue task
complete, regenerated the ledger against the current queue, and reran all 22 gap
tests, exact generated-output check and workboard validation successfully.
The accepted deliverable is the gap-audit tool and planning ledger. C01 and all
pending source/content acceptance remain open.
