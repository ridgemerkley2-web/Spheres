# Checkpoint26 validated repair

Validated source: `9f7faed5f541e14545e01caf1616ba08c6ad8ff9`. This is a technical verification checkpoint, not campaign certification: the original A1 political concentration rule still fails.

- Native workspace: 1,886 passed, zero failed, 89 ignored, one resource timing check filtered, across 66 targets. The new load/fresh-start regressions all pass.
- Isolated resource timing: 0.0612 ms/month passes the unchanged 0.15 hard limit; the 0.05 advisory target is not met.
- Full Node: 1,701 passed, zero failed or skipped. This ran after runtime/metadata freeze and before the commit; documentation and evidence attributes were finalized afterward, without changing runtime or test inputs. The receipt does not claim a clean tree at test start.
- Four unchanged original-archive checks: all pass, including the pinned-master archive that exposed the bug. All 32 fixture/provenance hashes match before and after.
- Mature native preview latency: decision p95/max 13.9524/14.4049 ms; government 11.6118/11.6136 ms; reciprocal-route 172.6067/179.594 ms. Both tests pass their unchanged 300/750 ms limits with 3 warmups and 21 samples; both original archives remain unchanged.
- Browser CI journey: passes on the clean exact build using installed Chrome 153.0.8010.53 and Playwright 1.62.1, via the supported launch override. The unchanged assertions verify all 22 served UI assets, panels, save/history continuity and recovery from a lost committed response. This is separate from hosted Chromium verification.
- Original files and preview servers: preservation verifier passes, including all eight protected files. Source and browser binary remain unchanged through the completed checks.
- Political suite: 10 passed, one failed (A1), one diagnostic filtered. Every printed metric and all 20 printed seed rows match iteration23. There is no new checkpoint26 N200 result and no held-out cohort was inspected.

The browser screenshots remain in the external checkpoint26 capture (their hashes are recorded); binaries and large campaign saves are not duplicated here. Full raw logs and receipts are retained, including the failed political result.

`preceding-hosted25/` closes the prior7b8a3dbb run: both native/core-browser jobs, both JavaScript/tooling jobs and both town-rendering jobs passed. Both political jobs failed only A1; the aggregate checks correctly failed closed. Current9f7faed5 hosted verification is a separate in-progress run35794328514 when this archive is written.
