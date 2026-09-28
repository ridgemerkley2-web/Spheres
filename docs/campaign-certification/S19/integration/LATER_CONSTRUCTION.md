# S19 later construction qualification

The existing first-hour driver now supports an additional, opt-in construction
checkpoint. Set `SPHERES_GUIDANCE_LATER_CONSTRUCTION=1` when running
`node tools/ui/ci-guidance-route.cjs`, alongside the existing `SPHERES_BINARY`,
`SPHERES_EXPECTED_REVISION` and optional `SPHERES_GUIDANCE_OUTPUT` variables.
Build the executable from a clean committed runtime first. The driver starts
its own server with a disposable save directory and chooses a fresh France
campaign through the menu. Existing campaigns and servers are not reused.

The driver retains the first-hour budget, funded workshop, draft and airbase
actions. After that route, the extra checkpoint:

1. Tracks the exact workshop ID queued by the visible construction confirmation.
2. Advances only through the visible **+1 DAY** control, checking native records
   after every day, with a maximum of 540 additional days.
3. Requires a completion and positive output at the same district and site kind.
4. Opens live guidance and checks dated `project_completed` and `site_producing`
   milestones against its native reading and rendered milestone text.
5. Continues through the existing stale-response, named-save, confirmed-load,
   reload/Continue, narrow-layout and new-campaign isolation assertions.

The existing command audit remains unchanged. No funds, research, delivery or
completion are injected; game time and normal funding determine the outcome.
The progress journal captures the project every 30 days. A timeout is a failed
qualification, with its actual campaign state retained for diagnosis.

Without the flag, the original first-hour behavior and scope remain unchanged.
This mode does **not** qualify procurement or flown results, or close S19/G4.
Those still need ordinary purchases, actual deliveries, squadron readiness and
mission receipts with their own save/resume checks.

## Verified checkpoint — 27 September 2026

Runtime and driver: `72d3414d6b79e8d35ba54440c13d21b6ef04db31`.
The new route first exposed a raw district ID in the completion milestone. The
advisor now uses the mapped site and province, or a neutral completion label
when the location cannot be mapped. Its regression test covers both cases.

The repeated fresh France run passed all stages. After 169 extra visible daily
advances, it observed the workshop's **3 July 1990** completion and positive
output of **0.101388/day**, checked on 4 July. Guidance displayed Île-de-France.
Named-save, confirmed-load and reload/Continue checks passed with completion/output
milestones intact. Fresh-campaign isolation and 390-pixel layout checks passed.
Desktop and narrow screenshots were visually inspected.

All **1,702 UI tests**, including **80 targeted guidance tests**, passed. The
locked release build passed (existing vendored tiny_http warnings only). Eight
planning tests and the 44-marker/10-task workboard check also passed.

[Evidence manifest](later-construction-evidence/manifest.json) pins source,
driver bytes, artifact hashes and the separately retained original save files.
[Run result](later-construction-evidence/result.json) records commands and native
observations. The initial failing run is retained alongside the successful result.

### Remaining limitation found during this run

**Resolved prospectively at `24d05428`:** [payment retention qualification](PAYMENT_RETENTION.md).
The following describes the earlier `72d3414d` run, not the current behavior.

Once the completed project leaves the active queue, its earlier `work_paid`
milestone reads **Not yet**, although completion and output remain achieved.
The current native reader enumerates active projects only. Preserve or recover
dated payment receipts before qualifying payment-history behavior; do not infer
a payment from a completion headline, since legacy/prepaid projects exist.
This follow-up, company procurement/delivery and actual flown results keep S19 open.
