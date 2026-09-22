# CLAUDE-C01-02 — Tonga's 2021 appointment transition

Owner: Claude. State: **accepted and integrated** as bounded research, 21 September 2026. Parent: C01 (incomplete).
Submission `b6767837b8080cae133e073ffb9ecb6f9371ee5f`; integration `2d5ffffe`.
[Codex review and validation](../../campaign-certification/C01/integrations/CLAUDE-C01-02/README.md).
Predecessor: [CLAUDE-C01-01](CLAUDE-C01-01.md), integrated and qualified at `fdb6d2c`.
Branch `claude/c01-tonga-02`; base `5b46e40a` (current integration); claim `852510d3`.
Result commit: `b6767837b8080cae133e073ffb9ecb6f9371ee5f` (separate regenerated index).
Reviewer/integrator: Codex. Separate from the S19 gameplay packet.
[Report](../../campaign-certification/C01/research/tonga-transition-2021-02.md): TO-TR21-01, 02, 03 and 06
accepted; TO-TR21-04 (warrant signature and presentation) and TO-TR21-05 (end of
Tu'i'onetoa's premiership) unresolved with sources attempted.
Touched paths: this record; `research/tonga.json` (seven sources, nine claims, holders on
`to_pm` from 2021-12-27 and `to_deputy_pm` from 2021-12-28); seven new
`research/sources/tonga-{assembly,pmo}-*-facts.json` extracts; new
`research/tonga-transition-2021-02.md`; `test_tonga_research_s10g.py` (totals updated,
none loosened); new `test_tonga_transition_c01_02.py`. Separate commit: `research-index.json` only.

## Bounded deliverable

Review at most six observations concerning the 2021 transition from Pohiva
Tu'i'onetoa to Siaosi Sovaleni. Existing IPU evidence records an Assembly selection;
the appointment instrument and effective date remain unresolved. Seek the original
Royal Warrant, Gazette, PMO release or Assembly record. Separate selection,
signature, presentation, appointment effectiveness and any explicitly recorded
caretaker boundary. Different dates are not automatically inconsistent.

The previous report mentions an unimported secondary account of a warrant. Treat
it as a search lead, not an accepted primary claim. If the primary record cannot
be retrieved, record the exact source attempted and uncertainty; do not infer a
term, end date or current office from the last observation. Preserve the frozen
7 September 2026 cutoff. Do not expand this packet to the 2019/2024/2025 transitions.

## Allowed files and handoff

- This record and a new `research/tonga-transition-2021-02.md` under
  `docs/campaign-certification/C01/`.
- The existing `research/tonga.json` and specifically related new factual extracts
  in `research/sources/`, with source IDs, locators, access dates and uncertainty.
- Focused Tonga validation under `tools/avatars/`.

Generated `research-index.json` changes must remain a separate integration patch.
Do not edit other country packets, shared atlas/UI code, the central roadmap,
installed character data, portraits, game schemas or simulation rules. Reuse IDs;
do not create a second prime-minister institution to work around unknown dates.

Run research-index validation, all Tonga and campaign research tests, atlas Node
checks and `git diff --check`, as listed in the preceding handoff. Return a separate
branch with exact base/result commits, source evidence, resolved/open observation
IDs and test results. Mark `ready_for_review`; C01 and all parent gates stay open.
