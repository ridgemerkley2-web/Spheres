# CLAUDE-C01-01 — one Tonga census reconciliation packet

Owner: Claude. State: **ready_for_review** (claimed and submitted 21 September 2026; not complete).
Parent: C01 (in progress, incomplete). Dependency: S01 (complete).
Branch `claude/c01-tonga-01`; base `3d422da`; result `8e94a3b` (packet) and
`8b9b7e6` (regenerated research index, a separate integration patch).
Touched paths: `tonga.json`, `tonga-reconciliation-01.md`, three new
`research/sources/tonga-*-facts.json` extracts, `test_tonga_research_s10g.py`
(pins updated, none loosened) and new `test_tonga_reconciliation_c01.py`. Next checkpoint:
Codex review, then `C01-Tonga-REC-002`..`005` as proposed in the report.
Integrator and reviewer: Codex. Runs independently of S18.

Read `docs/campaign-certification/C01/README.md`, `research/README.md`,
`research/tonga.json`, `research-index.json`, `work-orders.json`, and
`docs/planning/campaign-pathway.json`. Reuse existing IDs and source records.

## Bounded deliverable

Reconcile at most ten Tonga organization/office observations, beginning with the
PATOA/PTOA spelling ambiguity, the People's Party observation and the separation
between parliamentary selection and royal appointment. Use current primary-source
research where available. Link exact claims to sources and event dates; keep unknown
lifespans and leadership terms unknown. A negative result with cited evidence and
explicit follow-up is useful. Do not manufacture a closed census to fill a table.

Produce a short reconciliation report with accepted matches, rejected matches,
unresolved cases and suggested next work-order IDs. If a fact changes, update the
relevant source-backed packet and its focused validation. Distinguish checked-in
factual extracts from original response hashes; do not claim downloads you did not retain.

## Allowed paths

- This handoff record and a new `docs/campaign-certification/C01/research/tonga-reconciliation-01.md`.
- `docs/campaign-certification/C01/research/tonga.json` and specifically related
  new source/factual-extract files under its `research/sources/` directory.
- Relevant Tonga-focused tests under `tools/avatars/`; no weakening shared validation.

The generated central census/index/work-order changes should be supplied as a
focused integration patch or documented regeneration instructions. Avoid changing
other countries' packets, the central roadmap, game/save schemas, installed leader
data, portrait manifests, UI or flight/model files in this packet. Installed content
is handled by C02–C05 after the evidence is adequate.

## Checks and return record

```text
python -X utf8 tools/avatars/campaign_research.py --check
python tools/planning/workboard.py --session C01
git diff --check
```

Run applicable existing Tonga validation discovered in `tools/avatars/` and any
meaningful new check for changed records. If regeneration is required, report the
exact failing output and resulting patch; do not silently relax the check.

Return branch, base/result commit, changed paths, cited sources, commands and results,
resolved/open observation IDs and next small packet. Mark this packet
`ready_for_review`; **do not mark C01, C06, S23, WC1 or CP1 complete**. All 160 country
identities and historical/future appearance obligations remain the parent scope.
