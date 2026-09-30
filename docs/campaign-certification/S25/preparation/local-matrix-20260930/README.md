# Current campaign matrix — still running

**30 September 2026 · `CODEX-S25-MATRIX-01` · candidate `68ba0622ec709b78617aadd1f9198d18f532bb32`.**

The current attempt has **4 completed passes, 4 running cases and 16 queued
cases** as of **30 September, 22:11 UTC** in [snapshot.json](snapshot.json). The completed cases
have been independently revalidated from their retained files. The full 24-case
matrix remains unfinished; this snapshot does not close S25, G5 or CP1.

| Country | Seed 1990 | Seed 7 | Seed 42 |
|---|---|---|---|
| France | Pass revalidated | Pass revalidated | Pass revalidated |
| Japan | Pass revalidated | Running | Running |
| India | Running | Running | Queued |
| Brazil | Queued | Queued | Queued |
| South Africa | Queued | Queued | Queued |
| Tonga | Queued | Queued | Queued |
| Saudi Arabia | Queued | Queued | Queued |
| USSR | Queued | Queued | Queued |

## Next action

Let the existing four-worker run finish, then collect its aggregate result and
the existing dependent verifier's final result. That verifier is already waiting
for the exact original launcher to exit. No additional matrix or full-run verifier was
launched during this cleanup.
[execution-status.json](execution-status.json) records the observed existing processes.

- Retained run: `D:/spheres-offload/codex-next-20260928/full-matrix-local-20260930-01`.
- Full verifier output: `D:/spheres-offload/codex-next-20260928/full-matrix-dependent-verification-20260930-01`.
- This audit's output: `D:/spheres-offload/codex-next-20260928/matrix-cleanup-audit-20260930-01`.

All 24 cases must finish on the declared candidate and pass independent retained
verification before the engineering preflight can close. Any failure remains
visible and needs a diagnosis and a newly declared complete attempt. Keep the
original countries, seeds, 1990–2035 horizon, reloads, invariants and archive checks.
Canonical S25 still has its recorded qualification dependencies.

## What was checked

[audit_completed.py](audit_completed.py) reads the running attempt without changing
it. It checks the frozen binary, plan and helper hashes; reconciles the completed
cases against the captured journal, file ledgers and transfer receipts; rehashes
their retained files; and decompresses every retained archive to its original
byte count and SHA-256.

The audit finished at 22:13 UTC: **56 retained files, 24 compressed save archives,
4,624 comparison records and 16 final/sandbox archives passed their checks**.
Every completed case covers 16,801 calendar days per leg plus the sandbox day.

For each completed case, the unchanged frozen helper rechecks native report
identity, ordinary campaign rules, daily invariants, command records, scheduled
reloads, comparison dates, full-horizon pause and sandbox continuation. It also
recomputes raw and canonical fingerprints from the actual final and sandbox
archives, and compares the uninterrupted and resumed saves. Only the permitted
terminal `saved_unix` timestamp is excluded from canonical comparison.

This is a completed-cell audit, **not the whole-run verifier**: the run has not
yet written its final aggregate or closing journal event. The audit launches no
native game and changes no simulation code or test limits.

The compact Git packet retains the audit, its receipt and byte-identical metadata
from the four completed cases. Large gzip saves stay at their original pinned
paths in the retained run. [snapshot.json](snapshot.json) records the source pins,
copied-file hashes and validation details. The Git packet alone cannot replay the
archive verification without those saves.

The [earlier failed attempt](../full-matrix-20260930/README.md) remains preserved.
Its results were not reused in this attempt.
