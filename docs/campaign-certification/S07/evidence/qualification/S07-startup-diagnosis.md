# S07 startup diagnosis — 2026-09-11

Scope: read-only source inspection plus one fresh disposable GET-only server probe. No country selection, command, advance, load, save, original campaign input, source edit, build or gameplay journey was performed.

## Confirmed mechanism

The construction readiness probe at `tools/ui/ci-construction.cjs:114` returns as soon as global Node `fetch('/api/state')` resolves its headers. It neither consumes nor cancels the body. The same pattern exists in `tools/ui/ci-browser.cjs:15` and `tools/ui/ci-money.cjs:139`.

The game handles requests in one incoming-request loop (`spheres-web/src/main.rs:7265`). State JSON is built under the game lock (`:7762`) and the lock leaves scope before response writing; this is not a held campaign mutex. The loop calls `request.respond(response)` synchronously (`:8132`). tiny_http 0.12.0 `src/request.rs:443–459` performs `response.raw_print(...)` followed by `writer.flush()` before returning. An unconsumed large response can therefore leave this sole handler waiting for socket output and prevent unrelated requests from completing.

## Observed probe

- Runtime source pin: `041007fbfda48cc027d0c24a1189705a1dcaa89b` (also verified from GET `/api/build`).
- Binary: `integration-target/release/spheres-web.exe`, SHA-256 `e845d3179f72e1ec49ae68197b48cad94dcfd0648ab5f358e0e9d28115d6d3c7`.
- Node: v24.12.0.
- Disposable server: PID 50784, port 57782; child exit observed with SIGTERM after the probe. A subsequent process lookup found no PID 50784.
- Strongly retain the Response from ordinary Node `fetch('/api/state')` without consuming its body: 200 headers received, bodyUsed false.
- Issue GET `/api/build` on an independent Node HTTP connection: timeout after 2,006.5 ms.
- Consume the original state body: 2,097,914 bytes drained in 7.9 ms.
- Immediately issue GET `/api/build` again: 200, 481 bytes, 1.9 ms.
- Repeat five readiness requests using `/api/build`, consume each body, set `Connection: close`, then independently request build again: all ten requests returned 200, with subsequent request durations 0.54–0.87 ms.

These timings diagnose ordering/blocking only. They are not campaign throughput or performance qualification measurements.

## Narrow correction

Use GET `/api/build` for readiness, fully consume its body before breaking, and set an explicit request timeout; `Connection: close` is also appropriate for this one-off startup probe. Preserve the subsequent independent build/source identity checks. For example:

```js
const response = await fetch(url + '/api/build', {
  signal: AbortSignal.timeout(2000),
  headers: { Connection: 'close' }
});
await response.arrayBuffer();
if (response.ok) break;
```

This is a harness-only correction. No runtime/gameplay change is needed to address the reproduced startup failure. Full construction/general-browser reruns remain the parent's qualification work.

## Prior failure attribution limits

The Tonga artifact `integration/artifacts/browser-construction-ci/tonga-NPEta5/result.json` failed before any country choice, command or advance, on `/api/build`, matching this reproduced mechanism. The successful France journey `france-xUfR8c` remains valid.

The earlier `france-eXE1kF` failure reached `page.goto('/')` after build and asset checks succeeded. This GET-only probe did not reproduce that separate navigation sequence; the unread readiness defect is present there too, but it is not proof of the exact cause of that earlier navigation timeout. Any navigation failure after the corrected readiness should be diagnosed separately rather than attributed automatically.

## Evidence

- Reproducible script: `S07-startup-diagnosis.cjs` (in this evidence directory).
- Invocation/output: `S07-startup-diagnosis.log`; command: `node ../evidence/S07-startup-diagnosis.cjs` from integration.
- Structured observation: `S07-startup-diagnosis-QMJ9Yu/result.json`.
- Child output: `S07-startup-diagnosis-QMJ9Yu/server.log`.

All server-created output stays in the fresh `S07-startup-diagnosis-QMJ9Yu` directory. Integration source was clean when inspected after the probe.
