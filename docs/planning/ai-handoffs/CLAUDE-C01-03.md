# CLAUDE-C01-03 — Tonga's next succession transition

Owner: Claude. State: **ready_for_review** (submitted 21 September 2026; not complete). Parent: C01 (incomplete).
Meshed on 22 September 2026: merged integration `9d352f03` (with the accepted CLAUDE-C01-04 and C01-07). The
shared `to_ipu_2025` source now carries both packets' claims (its C01-04 fetch figure is kept; this packet's
165,854-byte fetch is noted in the extract), and the exact pins in the C01-03/C01-04/s10g tests now include each
other's additions. None was loosened.
Branch `claude/c01-tonga-03`; base `04bc99a6` (current integration); claim `bc48be49`.
Result commit: the head of `claude/c01-tonga-03` at submission (the separate index commit); to be recorded by the integrator.
[Report](../../campaign-certification/C01/research/tonga-transition-2024-03.md): TO-TR24-01 (resignation and
acceptance, 9 Dec 2024), 03 (Assembly selection, 24 Dec 2024), 04 (royal appointment, 22 Jan 2025) and 06
(Cabinet effective 28 Jan 2025) accepted; TO-TR24-02 accepted as attestation (Samiu Kuita Vaipulu acting on
24 Dec 2024 and 6 Jan 2025) with start, instrument and end unresolved; TO-TR24-05 partly resolved (posting date
and 31 Jan oath recorded; warrant and any separate effective date unresolved). TO-TR21-04 and TO-TR21-05 unchanged.
Touched paths: this record; `research/tonga.json` (17 sources, 27 claims; Sovaleni `to_pm` holder `until`
2024-12-09 from the stated resignation; new holders on `to_pm` (Eke, event 2025-01-22) and `to_deputy_pm`
(Vaipulu, event 2024-12-09; Fusimalohi from 2025-01-28)); 17 new
`research/sources/tonga-{assembly,gazette,pmo,ipu}-*-facts.json` extracts; new
`research/tonga-transition-2024-03.md`; `test_tonga_research_s10g.py` and `test_tonga_transition_c01_02.py`
(pins updated to the new totals, holders and stated end; none loosened); new `test_tonga_transition_c01_03.py`.
Separate commit: `research-index.json` only. Outside the boundary and left for the integrator:
`tools/ui/ci-leadership-research-browser.cjs` expects the Sovaleni card to end "→ Not established".
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
