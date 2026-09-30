# One-off dependent verification of the current 24-cell matrix

This external job belongs to the execution launched on 2026-09-30 at 09:03:11 UTC. It is not a scheduler, recurrence, new campaign, or qualification award. It never modifies the matrix output, runner, source, budget, process, or qualification flags, and never publishes or commits.

`config.json` pins the current launcher PID 9040, its precise Windows creation FILETIME and command, candidate `68ba0622ec709b78617aadd1f9198d18f532bb32`, the frozen manifest/plan/helper/binary, the actual interpreter image, and the existing full-matrix freeze. `prepared.json` pins this configuration. Original small metadata is copied unchanged into `captured/`.

`dependent_verify.py` opens a read-only Windows handle to that exact process. The handle remains tied to it if Windows later reuses the PID. It waits without sampling simulation state. Once the process exits, the job requires its coherent `full-exit.json`; absence or disagreement records failure without starting a verifier. A normally completed failed matrix can be checked for integrity but can never receive a passing label.

After rechecking the pins, it invokes only the exact frozen helper with `--verify D:/spheres-offload/codex-next-20260928/full-matrix-local-20260930-01`. It captures the verifier's exact stdout, stderr, exit code and elapsed time, then rechecks immutable dependencies and the matrix result. No native test is executed by this command. The helper revalidates the retained full native archives and their restoration/hash maps.

The separate final gate requires a successful launcher and verifier, candidate identity, unchanged binary, a complete full plan, all 24 distinct declared/attempted/passed cells, no missing or failed cells, no interruption/resource halt, and complete final/sandbox archive coverage. Integrity alone, a pilot, or a one-cell shard cannot pass. `qualification` and `s25_complete` remain false even if the full preflight passes.

`launch.ps1` uses `Start-Process -WindowStyle Hidden`, writes into a newly created launch directory, and refuses repeat launch/overwrite. `pending.json` is an immutable armed handshake, not a success report; `events.jsonl` records subsequent transitions, and `result.json` records the terminal outcome. Read the terminal result if it exists. A machine shutdown or externally terminated worker is not automatically retried; absence of a terminal result remains pending/unverified.

Ten focused synthetic tests cover full-plan identity, false pass cases, stale pins, create-only output, duplicate/nonfinite JSON, and real Windows handle waiting on a short owned Python sleeper with wrong-creation refusal. They do not constitute simulation evidence. The first preparation failed while resolving a Microsoft Store execution alias; `preparation-failed-01.json` retains that failure, fixed by pinning the actual interpreter image before any launch.

No full retained verification or matrix pass is claimed by preparing or arming this job. The independent source review, launch receipt, and eventual verification records describe their own separate scopes.
