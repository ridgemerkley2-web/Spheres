# CLAUDE-C01-03 — Tonga's next succession transition

Owner: Claude. State: **available, unclaimed**. Parent: C01 (incomplete).
Predecessor: [CLAUDE-C01-02](CLAUDE-C01-02.md), accepted as bounded research.
Start from the current `codex/campaign-certification`; record the actual base and
claim commit before work. Suggested branch: `claude/c01-tonga-03`.
Reviewer/integrator: Codex.

## Bounded deliverable

Review at most six observations about Tonga's 2024–2025 prime-minister transition
and the first resulting Cabinet. Locate original Assembly, Royal, Gazette or PMO
records. Separate resignation/office end, any caretaker arrangement, Assembly
selection, royal appointment, effectiveness, publication and Cabinet appointments.
Keep unidentified or inaccessible instruments explicit; a report date does not
establish when an office started or ended. Existing source records are search
leads, not automatically verified full terms.

Reuse the existing prime-minister and Cabinet institutions and role IDs. Preserve
the unresolved 2021 questions; do not expand this packet to resolve every Tonga
leadership chain. Do not infer party leadership from national office. The cutoff
stays 7 September 2026, with no source-backed history added beyond it.

## File boundary and handoff

Allowed: this handoff; a new `docs/campaign-certification/C01/research/` report;
the existing `tonga.json`; specifically related factual extracts in `sources/`;
focused tests under `tools/avatars/`. Put any generated research-index change in
a separate commit. No shared UI, central roadmap, game schemas, installed leaders,
avatars, other country packets or simulation changes.

Run exact research-index validation, Tonga and campaign/research Python tests,
atlas Node checks and `git diff --check`. Return source locators, access dates,
response/extract hashes when available, explicit resolved/open observation IDs,
exact base/result commits and test results. Mark `ready_for_review`; the integrator
decides acceptance. C01, C06, S23, WC1 and CP1 remain open.
