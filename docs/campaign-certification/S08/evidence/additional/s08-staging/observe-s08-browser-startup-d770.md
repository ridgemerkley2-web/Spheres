# S08 browser startup observer (prepared, not executed)

Preload: `observe-s08-browser-startup-d770.cjs` beside this note. It loads the exact Playwright instance resolved by the committed browser CI and observes the installed-Chrome wrapper. It does not require or launch the CI itself.

Use it with the existing outer browser-installed lane so the unchanged source/binary binding and original failure assertions are still retained:

```powershell
$env:NODE_OPTIONS = '--require "C:\Users\ridge\Documents\Codex\2026-09-05\pick-up-the-spheres-game-on\work\campaign-certification\s08-staging\observe-s08-browser-startup-d770.cjs"'
```

Then run the existing `run-s08-final.py` browser-installed command with a fresh evidence suffix. Restore the previous `NODE_OPTIONS` afterward. Optionally set `SPHERES_BROWSER_OBSERVE_OUT` to an absolute directory that does not yet exist; otherwise the observer creates a unique `d770-navigation-observation-*` directory beside itself and prints its path. Do not run concurrently with other browser/simulation/build lanes. Preserve the previous shared `artifacts/browser-ci/result.json` and screenshots first: the unchanged CI writes those outside its unique campaign run directory.

Artifacts:

- `metadata.json`: observation schema, UTC time, Node arguments/version, expected candidate label, and SHA-256 of the observer, installed wrapper, committed CI and integrated helper. The outer qualification runner remains authoritative for actual source/binary revision.
- `events.jsonl`: timestamped request/response/finished/failed metadata; console/page errors; frame, DOMContentLoaded and load events; navigation start/success/failure; process lifecycle. Body contents are never read. Ordinary events cap at 12,000; failures and lifecycle summaries continue and truncation is explicit. Pending requests cap at 2,048, with an explicit untracked count.
- On a navigation exception, `page-N-goto-failure-N.json` (or reload) records the original error, pending requests, optional document/timing snapshot (2-second bound) and optional viewport PNG (2-second native timeout, 2.25-second outer bound). The exact original error is then rethrown. Navigation/default timeouts, route handlers, launch arguments, assertions and game commands are unchanged.

This preload adds event logging overhead and at most 4.25 seconds of diagnostic reads after a navigation has already failed. It is observational reproduction evidence and is not a timing benchmark. No CDP commands, routes, response substitutions, retries or altered navigation timeouts are introduced.

Interpretation: identify the final unfinished local script/stylesheet request, whether the inline boot reached `/api/state` and `/api/roster`, and whether the real state request finished. The boot route intentionally holds its fetched response, which by itself is not a parser wait. Completed blocking resources without DOMContentLoaded plus an unresponsive document snapshot would justify further renderer investigation, but does not by itself prove CPU contention. Preserve the original timeout evidence separately.

Preparation used file edits only outside the repository; this preload has not yet been syntax-checked or executed.
