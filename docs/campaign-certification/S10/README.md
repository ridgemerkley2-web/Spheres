# S10 — Government, succession and diplomacy

Status: **S10.a and S10.b complete; S10 and C01 remain in progress**.
The [S10.b report](b/README.md) records source-backed country discoveries,
Linux qualification and all eight ordinary government browser journeys.
Parent source: `872442d7411d9986321248b41ccadf79e2851d1c`.
Latest user instruction: Next. Work is limited to S10; S11 is not started.

## S10.a — Reviewed political decisions

Implemented a shared server precondition for government and diplomatic reviews:
the exact campaign, serialized world and command must still match at confirmation.
Receipts must replay an already processed order before checking its now-stale
review, without charging twice. Preserve the legacy unreviewed API.

The Decisions room gains review/cancel/confirm for pending diplomatic requests,
standing policy, monetary commitments and campaign aims. Sanction, lift and
improve-relations controls route to the same review. Native cloned command results
supply immediate changes; standing obligations and offer deadlines stay explicit.
Saved request history receives readable dates and country names in presentation.
This does not add a parallel simulation or change historical records.

## Verified in this increment

- 20 new native tests cover exact outcome, repeat receipt, altered command/batch,
  second-tab changes, date/session replacement, foreign and unaffordable orders,
  pending offers, deadline boundaries and save/load.
- Shipped leadership tests: parliamentary, presidential, authoritarian and
  institutional cases; saved incumbents, executive eligibility, historical
  browsing and a recorded succession. Deliberate boundary fixtures are labeled.
- 14 new interface interaction tests and a disposable actual France walkthrough:
  cancel without effects, confirm once, stale second-tab refusal, save/load and
  readable desktop/narrow controls.
- Windows web suite: **367 passed, 11 ignored**. Existing agency/succession
  integration suites: **12 passed**. Interface suite: **1,532 passed, 1 skipped**.
  The separate C01 census suite passed all **7** tests and reproduced its output.

The actual browser changed France's future trade-response policy in a second
tab. The first tab's old government confirmation was refused before any mutation.
A fresh review then invited the French Communist Party into the government for
exactly **17.1386273190299 PC**, with a dated result. Review/cancellation, stale
refusal, Save/Load cancellation, Load and Continue passed complete native campaign
comparisons without ignored fields. No time was advanced or resources granted.

Native preview timing includes the cloned command and complete-world review token.
Both endpoints passed predeclared p95 **300 ms** and maximum **750 ms** limits
with three warmups and 21 samples. The older 2000 technical save reached p95
**27.2 ms**; the 134 MB connected Tonga save reached **256.3 ms**. These measure
native review generation, not HTTP/browser latency or whole-campaign performance.

Runtime: `7aaa4517fbcc1c256481c2eefc5ef5e1c4153303`.
Browser driver: `5275f3f268a8b0529f7ae8fe03e8b0be195ed601`.
Executable SHA-256:
`c7eef9f281e7009aac73c722413fcab7587277e77124b9ba56d8ed121b077878`.
The [manifest](manifest.json) and [byte inventory](evidence/inventory.json)
bind the checks, screenshots and complete native archives to those sources.

[Review build](http://127.0.0.1:7848/) uses a separate copy of the tested France
save. All eight protected originals, two source worktrees and earlier review
servers were preserved.

## Open dependency and certification limits

C01 is not complete. Its existing production board marks worldwide party research
as incomplete, and represented simulation rows are not an exhaustive historical
party census. A reproducible current-source census and numbered research backlog
are being prepared separately. Art jobs and future templates must not be counted
as verified people, institutions or completed portraits.

Passing S10.a checks does not close S10, earn G2, establish worldwide political
coverage, or qualify a campaign through the end of 2035. No such result is claimed.

Linux and the remaining certified-country end-to-end qualification have not been
refreshed for this increment. Other government systems, accepted treaties, calls
to arms and capped peg exits are covered by explicitly labeled native fixtures.
The old peg dispatch still describes nominal penalties; the new review shows
the actual capped changes. Visual review found no blocking issues, while retaining
two polish notes: a transient old error toast and horizontally scrolling narrow
government tabs.

Development failures are retained separately: an initial test borrow error,
mixed host line endings (restored to CRLF), a relative-path benchmark invocation
and the browser driver's initial single-page context. The final checked game
passed after these repairs; failed attempts are not counted as qualification.
