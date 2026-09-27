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
