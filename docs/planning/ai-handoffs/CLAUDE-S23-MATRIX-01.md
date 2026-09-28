# CLAUDE-S23-MATRIX-01 — historical and cartoon boundary audit

Owner: Claude; reviewer/integrator: Codex. State: **complete — bounded preparation only** (28 September 2026).
Submitted `8c9f6ce1`, integrated with repairs at `36740f8b`. All 36 tests pass.
See [independent review](../../campaign-certification/S23/integrations/CLAUDE-S23-MATRIX-01/README.md)
and the regenerated [coverage report](../../campaign-certification/S23/preparation/boundary-matrix/README.md).
The audit preserves uncertain dates, missing history/art and campaign divergence; it is not native replay.
S23 remains planned pending C06. Parent: S23, **preparation only**.
Branch: `claude/s23-matrix-01`.
Follow the [expanded working contract](CLAUDE-EXPANDED-NEXT.md).

## Build

Build a standalone audit generating date/role/appearance cases for the eight CP1
country cases from current checked-in research and production bindings. Cover yearly
samples 1990–2035, day-before/day-of/day-after known handovers and deaths/dissolutions,
the frozen 2026-09-07 cutoff, future eligibility, 2030 and 2035-12-31. Unknown interval
boundaries must remain unknown, not extrapolated from an isolated attestation.

Report historical identity, role, source acceptance, actual image binding and asset
availability separately. Compare a historical lookup and a saved gameplay incumbent
without assuming history overwrites a divergent campaign. Keep fixture assertions
separate from actual production observations. Reuse existing readable sources;
this tool must not depend on CLAUDE-C01-GAPS-01 being merged first.

## Owned paths

New `tools/avatars/certified_boundary_matrix.py`, `test_certified_boundary_matrix.py`,
`docs/campaign-certification/S23/preparation/boundary-matrix/`; this handoff.
Read existing research, data and tests; no shared schema, runtime or acceptance edits.

## Acceptance

Deliver deterministic case generation, JSON/Markdown findings and `--check` for
stale output. Test off-by-one handovers, co-leaders, missing versus inapplicable roles,
pending evidence, death/dissolution and missing/wrong image bindings. Run against
current integration and keep actual coverage gaps visible. Passing checker tests
does not mean its coverage report passes; C06/S20 and final S23 review remain required.
