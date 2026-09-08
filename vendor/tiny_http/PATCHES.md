# SPHERES transport patch

This directory contains the unmodified crates.io `tiny_http` 0.12.0 archive,
except for the worker-capacity fix and its regression test in
`src/util/task_pool.rs`, plus this note. The original MIT and Apache-2.0
licenses, attribution, manifest and upstream sources remain intact.
The packaged `examples/ssl-cert.pem` and `examples/ssl-key.pem` are upstream
public demonstration fixtures, retained byte-for-byte from that archive.

- Upstream: <https://github.com/tiny-http/tiny-http>
- Archive SHA-256: `389915df6413a2e74fb181895f933386023c71110878cd0825588928e64cdc82`
- Packaged upstream revision: `212b1c45852fef2093dc1374875a9393c55eb4b9`
- Local patch date: 2026-09-08.

`TaskPool::spawn` must reserve idle workers for already queued jobs. The
upstream check only asks whether any idle worker exists. A burst can therefore
queue more connections than idle workers; the first connections then occupy
those workers waiting for keep-alive input and the remaining connections never
reach the request handler. The fix creates a worker when the idle count is no
greater than the existing queue length. The queue mutex protects that decision.
The production predicate changes from `waiting_tasks == 0` to
`waiting_tasks <= queue.len()`; no other production statement changes.

SPHERES reproduced this with its original executable: stylesheet requests
remained pending while a new `/api/build` connection answered immediately.
Closing answered keep-alive connections released the original pending requests.
An instrumented, otherwise unchanged browser smoke likewise stalled on two
local stylesheets until fresh short-lived probe connections released workers.
These are transport scheduling symptoms, independent of simulation state.

The regression controls a real worker's wakeup, queues one blocking connection
into its reserved slot, then requires a second connection to answer before the
first is closed. It does not depend on winning a scheduler race. Reverting only
the capacity comparison makes this test fail.

The production change is one comparison. HTTP encoding, keep-alive behavior,
request ordering, and the game's serial mutation loop remain unchanged. No
browser wait, timeout or assertion is weakened. The workspace patch points here
so builds do not rely on a modified Cargo registry cache.

The focused standard-library-only regression can also be run without upstream
example/dev dependencies:

```text
rustc --edition 2018 --test vendor/tiny_http/src/util/task_pool.rs -o task-pool-tests
./task-pool-tests --test-threads=1
```
