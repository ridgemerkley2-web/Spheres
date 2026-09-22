# CLAUDE-S19-01 — outcome-aware first-hour guidance

Owner: Claude. State: **ready_for_review** (submitted 21 September 2026; not complete). Parent: S19.
Canonical dependencies: S06, S07, S10, S18. Reviewer/integrator: Codex.
Branch `claude/s19-guidance-01`; base `939e8f9`, merged with integration `5b46e40a`;
result `f3d4f82ae673859052404529c32853f398019464` plus the evidence commit at the branch head.
Evidence: [S19 README](../../campaign-certification/S19/README.md) and [manifest](../../campaign-certification/S19/manifest.json).
Coordination patch for Codex: `main.rs` (`mod guidance_outcomes;`, one `outcomes` key, tests) and
`index.html` (guidance routes cash_flow/air/companies/equipment tab/industry; `GUIDANCE_RECEIPT` hook in `api()`).
Next checkpoint: Codex review and integration; S20 navigation can build on these routes.

Read S19's exact acceptance criteria through
`python tools/planning/workboard.py --session S19`, then inspect the current code.
`docs/GUIDANCE.md` describes the original implementation and includes old branch
and launch notes; the current integration code and campaign evidence take precedence.

## Deliverable

Create an optional first-hour route covering country finances, a useful financially
funded construction project, research/design, a company purchase/delivery, air-force
preparation and save/resume. Separate "read this lesson" from "the campaign achieved
this result." Successful outcomes must come from current native receipts/state;
opening a screen, clicking a button or receiving a stale response cannot complete them.

Advice should identify the current obstacle, explain why it matters and open an
existing review screen. Keep it arcade-friendly and concise. No automatic orders,
hidden treasury changes, free inventory or replacement simulation in JavaScript.
Missing data is unknown; different campaign/session/date identities invalidate it.
Skipping, resetting and closing must work with keyboard and narrow layouts.

## File ownership

Own `spheres-web/ui/tutorial-model.js`, `advisor-model.js`, `guidance-ui.js`,
`guidance-ui.css`, focused guidance tests and new S19 documentation/evidence.
Inspect the actual `/api/guidance` read model before proposing extensions.
For `spheres-web/src/main.rs`, `spheres-web/ui/index.html`, saved schemas or shared
navigation, coordinate a small explicit patch with Codex before concurrent edits.
Leave equipment art/flight modules and central roadmap status to the integrator.

## Completion checks

Run the focused tutorial/advisor/guidance/host Node checks; add meaningful regressions
for real outcomes versus reading, stale responses, loaded saves, skipped steps and
unknown data. Check native read-only behavior if the endpoint changes. Complete an
ordinary desktop/narrow browser route against a real disposable campaign with dated
receipts and save/resume. Demonstration fixtures may exercise rendering but do not
prove actual campaign success. Retain exact source/build IDs and evidence paths.

Return a reviewable branch/commit, file list, validation, captures and remaining
limitations. Mark the packet `ready_for_review`. Codex integrates it and updates the
canonical session status. S20 shared-shell implementation is scheduled after this
handoff to avoid overlapping navigation rewrites; G4 still requires later sessions.
