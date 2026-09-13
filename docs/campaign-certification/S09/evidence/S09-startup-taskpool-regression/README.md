# Deterministic connection-pool regression

The completed fixture is `pool-6v_p3vas/result.json` with the exact upstream source copy, its SHA-256, both source variants, the fixture-only seam diff, the one-condition candidate diff, and Rust compiler/test logs.

The original tiny_http 0.12.0 `TaskPool::spawn` checks only whether `waiting_tasks == 0`. While notified workers are waiting to reacquire `todo`, subsequent admissions can still see them counted as waiting. An allowed burst can therefore queue more connection tasks than those workers can service. Persistent connection tasks remain occupied reading their next keep-alive request, so the excess queued connection is not reached.

To reproduce that interleaving deterministically, the fixture extracts the unchanged queue-locked admission body into `spawn_locked`, then holds that same lock across eight admissions with exactly four sleeping workers. Every connection task reports that it started and then blocks on an explicit release condition. Both variants use identical fixture code. All tasks are released and checked for completion before the expected assertion is evaluated.

| Variant | Started before release | Completed after release | Native Rust test result |
| --- | --- | --- | --- |
| Original `waiting_tasks == 0` | 4 of 8 | 8 of 8 | Expected failure, exit 101 |
| Candidate `waiting_tasks <= queue.len()` | 8 of 8 | 8 of 8 | Pass, exit 0 |

This proves the admission-rule defect and that the proposed condition fixes the constructed scheduling case. It is not a substitute for testing the actual patched dependency and the complete game browser journey. No repository or dependency files were changed by this fixture.

The real failed-browser wire record is `../S09-browser-netlog-1/france-7wn5I1/chrome-netlog.json`. Its stalled asset is `/chronicle.css` (CDP request `26376.5`, URL-request source 158), which binds stream job 161 and socket 163. The new TCP connection from port 64115 to the own game server port 58941 completes; Chrome sends 556 bytes at net-log time 83068401, then receives no socket bytes or response headers before cancellation near 83128399. Other resources, including guidance-ui.css, complete. This is consistent with an unserviced new connection and excludes a purely missing renderer event or a request that never left Chrome's socket queue. It does not alone prove which server-side scheduling state occurred; the standalone regression provides the independent source-level evidence.
