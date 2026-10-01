# Campaign matrix — attempt stopped, full preflight blocked

**30 September 2026 · `CODEX-S25-MATRIX-01` · candidate `68ba0622ec709b78617aadd1f9198d18f532bb32`.**

The attempt stopped at **30 September, 23:30:51 UTC** with **4 independently
revalidated passes, 4 abnormal exits and 16 cases not started**. No worker from
this attempt remains active. The launcher exited 120 without a final aggregate;
the dependent verifier failed with a PermissionError before retained verification
began. [Terminal reconciliation and failure evidence](interruption-20260930/README.md).
The earlier [22:11 UTC snapshot](snapshot.json) remains unchanged. The full
24-case preflight failed to complete and does not close S25, G5 or CP1.

| Country | Seed 1990 | Seed 7 | Seed 42 |
|---|---|---|---|
| France | Pass revalidated | Pass revalidated | Pass revalidated |
| Japan | Pass revalidated | Abnormal exit | Abnormal exit |
| India | Abnormal exit | Abnormal exit | Not started |
| Brazil | Not started | Not started | Not started |
| South Africa | Not started | Not started | Not started |
| Tonga | Not started | Not started | Not started |
| Saudi Arabia | Not started | Not started | Not started |
| USSR | Not started | Not started | Not started |

## Next action

Diagnose the native crashes and lost journal writes before declaring a fresh
complete attempt. Preserve these outcomes, the frozen candidate, full plan and
unchanged limits. No additional matrix or replacement verifier was launched.
[execution-status.json](execution-status.json) retains the earlier live-process
snapshot; the terminal packet records that those processes are now absent.

- Retained run: `D:/spheres-offload/codex-next-20260928/full-matrix-local-20260930-01`.
- Full verifier output: `D:/spheres-offload/codex-next-20260928/full-matrix-dependent-verification-20260930-01`.
- This audit's output: `D:/spheres-offload/codex-next-20260928/matrix-cleanup-audit-20260930-01`.

All 24 cases must finish on a declared candidate and pass independent retained
verification before the engineering preflight can close. This failure remains
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

This was a completed-cell audit, **not the whole-run verifier**: the run never
wrote its final aggregate or closing journal event. The audit launches no
native game and changes no simulation code or test limits.

The compact Git packet retains the audit, its receipt and byte-identical metadata
from the four completed cases. Large gzip saves stay at their original pinned
paths in the retained run. [snapshot.json](snapshot.json) records the source pins,
copied-file hashes and validation details. The Git packet alone cannot replay the
archive verification without those saves.

The [earlier failed attempt](../full-matrix-20260930/README.md) remains preserved.
Its results were not reused in this attempt.
