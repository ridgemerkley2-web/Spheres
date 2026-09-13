# S10 — Government, succession and diplomacy

Status: **in progress**. Parent source: `872442d7411d9986321248b41ccadf79e2851d1c`.
User instruction: Continue. Work is limited to S10; S11 is not started.

## S10.a — Reviewed political decisions

Implement a shared server precondition for government and diplomatic reviews:
the exact campaign, serialized world and command must still match at confirmation.
Receipts must replay an already processed order before checking its now-stale
review, without charging twice. Preserve the legacy unreviewed API.

The Decisions room gains review/cancel/confirm for pending diplomatic requests,
standing policy, monetary commitments and campaign aims. Sanction, lift and
improve-relations controls route to the same review. Native cloned command results
supply immediate changes; standing obligations and offer deadlines stay explicit.
Saved request history receives readable dates and country names in presentation.
This does not add a parallel simulation or change historical records.

## Qualification planned for this increment

- Native transaction tests: exact outcome, repeat receipt, altered command/batch,
  second-tab changes, date/session replacement, foreign and unaffordable orders,
  pending offers, deadline boundaries and save/load.
- Shipped leadership tests: parliamentary, presidential, authoritarian and
  institutional cases; saved incumbents, executive eligibility, historical
  browsing and a recorded succession. Deliberate boundary fixtures are labeled.
- Browser interaction tests and a disposable actual campaign walkthrough:
  cancel without effects, confirm once, stale second-tab refusal, save/load and
  readable desktop/narrow controls.
- Existing web and Node regression suites after the changes settle.

## Open dependency and certification limits

C01 is not complete. Its existing production board marks worldwide party research
as incomplete, and represented simulation rows are not an exhaustive historical
party census. A reproducible current-source census and numbered research backlog
are being prepared separately. Art jobs and future templates must not be counted
as verified people, institutions or completed portraits.

Passing S10.a checks does not close S10, earn G2, establish worldwide political
coverage, or qualify a campaign through the end of 2035. No such result is claimed.
