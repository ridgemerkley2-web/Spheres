# Supplier delayed-preview timeout patch

Prepared outside the repository, then applied to `tools/ui/ci-supplier-imports.cjs` after the root opened the source window. It has not been browser-tested.

The patch adds one harness-only helper and replaces only the delayed preview interception block. The helper retains a real `route.fetch()` response until the existing visible-day callback finishes. Its manual waits and fetch use the harness's existing 30-second timeout convention. Handler errors are returned as small outcomes and then thrown by the awaiting test, so early rejection cannot become an unhandled promise rejection. Cleanup always releases the response hold, removes only the handler it registered, and checks fetch/fulfill/dispose completion when a target request began. Other request handlers and all later stale-date, command-count, archive, and maintenance assertions remain unchanged.

The patch passed `git apply --check` before application. After application, the actual helper passed all eight fake-route VM checks in `verify-supplier-preview-timeout.cjs`, plus `node --check` and `git diff --check`. Results, source/helper/patch hashes, and a plain log are retained in `supplier-preview-timeout-vm-evidence.json` and `.log`. No build, browser, or large archive was used. Only the first matching request is held; later requests continue, and request parsing failures are forwarded into the awaited result.

## Small verification

`verify-supplier-preview-timeout.cjs` extracts the applied `withHeldSupplierPreview` into a Node `vm` context with a fake page/route, ordinary Promises, and controllable `setTimeout`/`clearTimeout` stubs. This tests the actual helper text without importing or running `main`, launching Chrome, or creating a campaign. The script has a separate 10-second real watchdog to report an unexpected hang.

- **Success:** `trigger` invokes the registered target handler; `fetch` returns a fake response. In `whileHeld`, assert that fetching finished but `fulfill` has not run, mark the visible day callback complete, and return the original date. Assert the helper returns that date; fulfillment observes the completion marker; disposal runs once; `unroute` receives the exact same pattern and handler; an unrelated fake handler remains registered; all timers clear.
- **Fetch failure:** reject `fetch` with a sentinel Error. Assert `whileHeld` is never called, the helper rejects with that same Error without waiting for a timeout, and the exact handler is removed. No response exists to dispose.
- **No interception:** let `trigger` finish without invoking a handler, then fire the registered 30-second timeout callback. Assert the helper rejects with the stage-specific timeout, never runs `whileHeld`, and removes the exact handler. Do not sleep for 30 real seconds.
- **Cleanup failures:** independently reject `fulfill` and `dispose`; assert each reaches the helper's returned rejection. Fail `whileHeld` after a real response is held and assert release still permits fulfillment and disposal. If both the visible-day callback and disposal fail, verify the aggregate message contains both causes for `result.json` diagnostics.

After these bounded tests, the normal authentic supplier journey remains the final validation: the native stale response must be discarded after actual one-day advancement, and the complete journey must still issue exactly the reviewed purchase and separate maintenance command.
