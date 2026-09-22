# CLAUDE-C01-02 — Tonga's 2021 appointment transition

Owner: Claude. State: **claimed 21 September 2026; in progress** (not ready for review). Parent: C01 (incomplete).
Predecessor: [CLAUDE-C01-01](CLAUDE-C01-01.md), integrated and qualified at `fdb6d2c`.
Branch `claude/c01-tonga-02`; base `5b46e40a` (current integration); result commit: recorded at submission.
Reviewer/integrator: Codex. Separate from the ready S19 gameplay packet.

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
