# S05 routing outcome comparison

This independent Node 24 runner starts a hidden server of the explicitly pinned executable in a new disposable output directory. It copies the unchanged genuine pinned-master archive into `saves/source-master.json`, loads it through `/api/load`, and makes 31 separate protected `/api/advance` requests for one day each. It issues no gameplay or rule commands.

The actual `/api/save` campaign archives are captured on days 0, 1, 15 and 31. Day 15 is saved both before and after a normal `/api/load`, and the runner requires a new session and identical persisted state. It terminates only its own spawned process. Existing live game processes and save directories are never selected or modified.

Every archived field is compared, including nested save envelopes, simulation RNG, history, dispatch log, dates, rules and all property books. Only top-level `saved_unix` is removed because it is save-time wall clock metadata. Session identities are outside the archive and are recorded separately. The lossless parser uses Node's numeric reviver `context.source`, retaining exact u64 and floating-point wire tokens; it does not round RNG values, apply tolerances, or recursively omit fields. Object key order and insignificant whitespace are canonicalized.

Example baseline command from the integration directory (the output directory must not exist):

```powershell
& 'C:/Program Files/nodejs/node.exe' '../s05-staging/deployment-outcome/run-deployment-outcome.cjs' --binary '../evidence/S05-live-map-93a49f5/spheres-web.exe' --binary-sha256 '183558cfc758128da47c04d08fc0eb4918dce681f0fc95c804364992c30f3997' --revision '93a49f58e999e65c2df1649c62d9f21ba8ea1d6a' --archive '../fixtures/s05-pinned-master/profile-campaigns/saves/profile-industry_and_war-0.json' --output '../evidence/S05-deployment-outcome-preopt'
```

Run the optimized executable with its own exact SHA256/revision, a fresh output directory, and `--baseline '../evidence/S05-deployment-outcome-preopt'`. `result.json` records pins, protocol, input preservation, process ownership, checkpoint hashes and exact A/B mismatch locations. Full canonical archives and original `/api/save` files remain available for inspection. `requests.jsonl` records every HTTP transaction's request and response summary, while server output is captured separately.

This is outcome evidence for one genuine technical USA workload, not a performance measurement, required USA campaign scope, or full campaign qualification. It starts from the original archive's actual date and existing operational state; no busy workload is fabricated. Captured counts disclose whether war, sectors, movement and service cargo persist. No Cargo, production test hook, or integration source edit is used.
