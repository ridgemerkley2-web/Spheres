# CODEX-S19-LATER-01 — later outcome qualification

Owner: Codex. State: in progress (preflight complete). Parent S19 remains in progress; no closure is claimed.
Start from `codex/campaign-certification` at or after `474df63e`.

Use a separate native preview and campaign save. Extend the existing
`tools/ui/ci-guidance-route.cjs` approach with ordinary visible controls and real
dated native receipts. The retained first-hour route is already qualified; do not
replace its evidence or change its four-command assertion to imply later coverage.

Checkpoints:

1. Identify a feasible ordinary company purchase; pay for it, advance to delivery,
   and confirm guidance recognizes the actual receipt, not starting inventory.
2. Complete a funded construction project and observe its resulting output;
   verify guidance dates/status and navigation to the relevant controls.
3. Ready a squadron and fly a supported mission; confirm the actual result in
   guidance. A base order or starting aircraft is insufficient.
4. Save and resume; rederive all three outcomes from native state. Reload/Continue
   must preserve them, while a fresh campaign must not inherit achievements.
5. Record source revision, served asset identity, command log, dated receipts,
   save hashes and desktop/narrow screenshots. Inspect the screenshots. Run
   affected guidance tests and record any natural campaign blockers explicitly.

Bounded repairs may touch the driver and reproduced guidance defects. Shared
runtime or economy changes require a concrete defect and regression check; do not
endow a campaign, inject synthetic results or relax certification criteria to
make qualification pass. Original saves and preview servers remain untouched.
Only after the required evidence passes may S19 close and S20 shared-shell work begin.

## 27 September preflight

All 79 tests in `check_guidance_outcomes.cjs`, `check_guidance_host.cjs` and
`check_guidance_ui.cjs` passed against the unchanged runtime at `474df63e`.
[Raw log and source identity](../../campaign-certification/S19/integration/later-preflight/manifest.json).
These are contract checks, not later-campaign browser evidence.

The existing driver explicitly prohibits purchasing in its first-hour stage and
asserts only four visible commands. Its source notes at least 180 days of ground
or 240 days of aircraft development before certification, and roughly 182 days
for the selected workshop. The later driver must retain those natural delays,
record real delivery/output/mission outcomes and preserve the first-hour route's
narrow assertions. No bypass or synthetic fixture was introduced in preflight.

## Completed construction checkpoint

On 27 September, runtime `72d3414d` passed the extended native/browser route:
3 July workshop completion/output, named save/load, reload/Continue,
new-campaign isolation and narrow layout. Fixed the completion detail exposing
an internal district ID. All 1,702 UI tests pass. See
[checkpoint evidence and limitations](../../campaign-certification/S19/integration/LATER_CONSTRUCTION.md).

Next: recover dated completed-project payment history (the work-paid milestone
currently reverts to Not yet after the project leaves the active queue), then
qualify actual procurement/delivery and flown results. Completion is not proof
of payment for legacy projects; do not fabricate that missing receipt.
