# Local connection-pool fix

This is tiny_http 0.12.0, under its original MIT OR Apache-2.0 license.
Both license texts and the original package manifest are retained. UPSTREAM.json
records every copied upstream file and its SHA-256 before modification.

Only src/util/task_pool.rs changes upstream runtime behavior. Admission now
compares idle workers with already queued tasks. A notified worker is still
counted as idle until it reacquires the queue mutex; a burst can otherwise queue
more persistent connections than the available workers can start. Occupied
workers can then wait for further requests indefinitely while a new connection
has already sent a blocking stylesheet request and receives no response.

The locked admission body is extracted without changing its locking boundary,
so a deterministic test can force that scheduling interleaving. Its eight tasks
must all start before any simulated keep-alive connection is released. Cleanup
unblocks every task before the acceptance assertion, including on failure.
The game workspace runs this exact vendored source in tests/http_connections.rs.

The unchanged upstream condition fails that test (four of eight tasks start),
and the corrected condition passes (eight of eight). S09 retains the baseline,
candidate, diffs, logs and real Chrome network trace in its certification evidence.
HTTP parsing, response handling, campaign routing and gameplay are unchanged.

Upstream: https://github.com/tiny-http/tiny-http/tree/0.12.0

Cargo.toml uses a local crates.io patch, preserving all existing resolved
dependency versions and features. The vendored examples are excluded from the
game workspace. The local patch can be removed when an upstream release fixes
the admission race and passes the same regression.
