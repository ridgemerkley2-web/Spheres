# Full-preflight startup observations

Both disposable runs called the unchanged `tools/ui/ci-integrated.cjs` `verifyBuild` helper before navigating. All 22 asset byte checks and native revision/binary provenance passed against `94d2c094b2c69a70613af2258acbf49e10d9e871`. Both then used the unchanged `page.goto(url, { waitUntil: 'domcontentloaded', timeout: 30000 })` call.

- `chrome-YlC1it`: CDP network ExtraInfo, Chrome net-log and CPU profiling enabled. DOMContentLoaded completed normally.
- `chrome-fNBq97`: identical preflight/navigation with network observation only; no CPU profile was started. DOMContentLoaded also completed normally.

The latter successful Chrome net-log records `/guidance-ui.css` as URL-request source 174: its socket stream was allocated, request headers sent, a native `200 OK` response with `Content-Length: 7496` received, and the complete body read within approximately one millisecond. This is a healthy comparison record; it does not explain the actual failing full-harness request which had only `requestWillBeSent`.

These samples show that preflight alone is not sufficient to reproduce the intermittent startup failure. They do not establish whether that failing request remained queued in Chrome or reached the server. Attach the same net-log and ExtraInfo observers to the actual full harness to capture a failing instance; no timeout increase or response substitution is justified by these observations.

Each run retains `chrome-netlog.json`, `events.jsonl`, complete per-request metadata, initial menu screenshot and server log. No campaign creation, game orders, response interception, or production edits occurred. Own Chrome instances and servers (PIDs 29748 and 33380) were closed. The helper and bounded per-request net-log summarizer are outside the source checkout.
