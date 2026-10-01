# Terminal failure of the 30 September campaign attempt

Candidate: `68ba0622ec709b78617aadd1f9198d18f532bb32`. Reconciled after the
existing launcher and dependent verifier exited; no new native run was launched.
**The complete 24-case engineering preflight remains blocked.**

The four earlier independently verified passes remain France/1990, France/7,
France/42 and Japan/1990. Four further cases exited abnormally, and the other
sixteen never started. All 24 dispositions appear in
[reconciliation.json](evidence/reconciliation.json).

| Interrupted case | Native exit | Last reported native date | Days per leg |
|---|---:|---|---:|
| Japan / 7 | 3221225477 (`0xc0000005`) | 2031-01-01 | 14,975 |
| Japan / 42 | 1 | 2027-10-01 | 13,787 |
| India / 1990 | 1 | 2023-10-01 | 12,326 |
| India / 7 | 3221225477 (`0xc0000005`) | 2024-12-01 | 12,753 |

All four partial native reports explicitly say `passed: false`; their null
failure strings are checkpoint state, not success. None reached the required
terminal comparison and sandbox day. Their nonzero exits occurred before the
unchanged 43,200-second per-cell limit; they are not relabeled as timeouts.

## Failure evidence and limits of the diagnosis

- The [launcher receipt](evidence/launcher/full-exit.json) records exit 120 at
  **2026-09-30 23:30:51 UTC**, after 52,058.57 seconds. There is no whole-matrix
  `result.json` or closing journal event, and no final verdict for the four
  interrupted cases. The retained journal ends with India/7's start at 17:43 UTC.
- [Windows Application Error events](evidence/windows-crashes.json) identify
  the exact frozen executable and original PIDs for Japan/7 and India/7, with
  `0xc0000005` in `ntdll.dll` at 23:28:41 and 23:28:57 UTC. These establish
  native crashes, not their underlying cause. The other two workers exited 1
  at 23:29:49 UTC; their empty stderr does not establish why.
- The [dependent result](evidence/dependent/result.json) records
  `PermissionError: [Errno 13] Permission denied` at 23:30:51 UTC. It never
  produced a verification invocation or result. Its journal contains only the
  original waiting event. No traceback identifies the exact rejected operation,
  so this packet does not assert a specific ACL, disk or gameplay cause.
- The frozen helper writes archive/transfer journal entries before cleanup.
  Here new transfer receipts exist but their journal entries and final verdicts
  do not. That is an incomplete evidence chain, not permission to synthesize
  missing success records. Original scratch and retained files were preserved.

The next investigation is the native crashes and durable journal failure.
Do not simply extend timeouts, weaken archive comparisons, restart the stopped
waiter in place or stitch successful cells from different attempts together.
A future complete run needs an explicit fresh candidate/attempt declaration
after the failure is addressed. The follow-up for this terminated attempt is
paused; the task stays blocked and S25 remains unqualified.

## Byte-integrity reconciliation

[reconcile.py](reconcile.py) performs read-only reconciliation against the
original frozen plan, binary and helper. It rechecks the earlier snapshot's 41
metadata pins without repeating its expensive unchanged archive audit. It also
rehashes the four new partial cells against their exact transfer receipts and
decodes their four retained gzip checkpoints to the recorded byte counts and
SHA-256 values. This checks recoverability and integrity, not campaign success.

The raw source data stays at
`D:/spheres-offload/codex-next-20260928/full-matrix-local-20260930-01`.
Byte-identical compact failure records are copied under `evidence/`; large
campaign saves remain at their original paths. [manifest.json](manifest.json)
pins the copied packet. The original snapshot and prior failures are unchanged.

Reproduce with a new output directory:

```powershell
python -B -X utf8 docs/campaign-certification/S25/preparation/local-matrix-20260930/interruption-20260930/reconcile.py --out C:/new-matrix-reconciliation
```

No model, plan, seed, horizon, acceptance limit, original artifact or active
user campaign was changed. This is documentation and evidence review only.
