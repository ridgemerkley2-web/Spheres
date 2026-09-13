# S08 observed general browser run — independent evidence review

The completed general browser lane passed all original assertions on candidate `d770592aeead51f1313d507edd26b02d75a69bba`. `S08-final-browser-installed-route-pool-3.json` records clean source before/after, exit 0, preservation passed, and the unchanged server binary SHA-256 `c5852894069c797c1cbf6b15a302f73f85a0f0b4a4edfee63eb1ca6b367f8435`. The run lasted 2026-09-13 02:02:42.442–02:02:59.037 UTC. Its preserved detailed result and screenshots are in `S08-general-browser-d770592/`.

The outside-only observer preserved launch arguments, the original 30-second default/navigation timeout, all route handlers and all assertions. It added event logging; no failure diagnostic reads ran because all three navigations succeeded. This is the successful general browser acceptance run with additional observation, not a performance benchmark or a qualification of other lanes.

Timeline from `S08-general-navigation-observed-route-pool-3/events.jsonl` (Node event receipt times, relative to initial `page.goto`):

| Event | Elapsed |
| --- | ---: |
| Latest of 22 stylesheet requests finished | 74 ms |
| Latest of 50 script requests finished | 127 ms |
| Initial DOMContentLoaded | 150 ms |
| Initial goto resolved | 151 ms |
| Initial load | 156 ms |
| Initial `/api/saves` completed | 158 ms |
| Initial `/api/roster` completed | 159 ms |
| Initial nation figure metadata completed | 166 ms |
| Initial `/api/state` completed, including the original delayed-response gate | 294 ms |

The two later reloads reached DOMContentLoaded and resolved in 120 ms and 93 ms. At initial DOMContentLoaded the observer had one pending request, `/api/saves`; no script or stylesheet remained pending. Across the full run, all 313 observed requests used `127.0.0.1:59464`, 312 finished, and one failed. The failed `/api/command` at +8,975 ms matches the committed harness's deliberate abort after `route.fetch`; the preserved result confirms lost committed response recovery, recovery across reload, two identical retry requests, delayed-boot navigation preservation, named-load cancellation, and save/history round trip. The observer records no uncaught page errors, crash, failed navigation, listener error, truncation, or pending requests at browser close.

Console issues are retained rather than suppressed: three `WebGL: INVALID_OPERATION: useProgram: attempt to use a deleted object` warnings point at `height-detail.js` line 99; one console error accompanies the intentional command abort; and a later console error reports `/favicon.ico` 404. The browser did not expose a corresponding favicon request through this page event stream. These appeared after initial startup, and the original page-error and UI assertions still passed. They do not explain the earlier timeout.

Failure history remains separate and unchanged. Attempt 1 (`S08-final-browser-installed-route-pool-1.json/log`) timed out waiting for DOMContentLoaded at the original 30-second limit, before UI assertions; its cause remains unknown because it lacked the request/lifecycle observations now available. The coordinator reports that attempt overlapped Linux native tests and the supplier exporter, but this successful isolated observation does not prove contention caused that failure. Attempt 2 (`S08-final-browser-installed-route-pool-2.json/log`) failed during Node preload setup with `MODULE_NOT_FOUND` after backslashes were stripped from `NODE_OPTIONS`; it did not launch a browser. Attempt 3 corrected only that outside-runner path spelling and succeeded on the same candidate with the original assertions and timeouts.

This review parsed the small retained JSON/log files and read the existing harness. It ran no browser, tests, builds, or simulations and made no repository edits. No further unchanged general browser run is recommended from these findings.
