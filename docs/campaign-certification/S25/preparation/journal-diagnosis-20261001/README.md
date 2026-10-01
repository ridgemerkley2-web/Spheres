# S25 journal failure diagnosis and bounded repair

The runner now retains a separate diagnostic when its journal cannot be written,
flushed or synchronized, and stops its owned workers when the background storage
guard encounters that failure. **The native crash cause remains unresolved.**
No new campaign attempt, native build or certification result is claimed.

This work starts from `c58ee025da50dfac164fc839b3fc021c39d483fe`. It preserves the
[30 September failed attempt](../local-matrix-20260930/interruption-20260930/README.md),
its original candidate `68ba0622ec709b78617aadd1f9198d18f532bb32`, four verified
passes, four abnormal exits, sixteen unstarted cases and all existing limits.

## What the failure evidence establishes

| Process | Retained observation | Supported conclusion |
|---|---|---|
| Japan / 7 | `0xc0000005`, `ntdll.dll` version `10.0.26100.7920`, offset `0x0000000000166167`, PID `0x9150`; 23:28:41 UTC | Native access violation. No source-level cause or read/write/execute subtype is available. |
| India / 7 | `0xc0000005`, same module version, offset `0x0000000000010579`, PID `0x4b98`; 23:28:57 UTC | A second native access violation at a different instruction. This does not prove a common cause. |
| Japan / 42 and India / 1990 | Exit `1` at 23:29:49 UTC; empty stderr | Abnormal termination. The runner has an owned-worker kill path after retention failure, but the missing journal cannot prove that it caused these particular exits. |
| Matrix Python process | Exit `120` at 23:30:51 UTC; empty redirected stderr and stdout | Python documents this code for errors during interpreter cleanup after `SystemExit`, including failed standard-stream flushes. It does not identify the rejected operation in this run. |
| Dependent verifier | `PermissionError: [Errno 13] Permission denied` at 23:30:51 UTC; no traceback, filename or WinError in its result | A failed operation, without enough retained context to identify an ACL, volume, stream or OS cause. It did not start retained verification. |

`0xc0000005` means an access violation; it is not the distinct in-page-error code
`0xc0000006`. These records do not establish storage corruption, exhausted memory,
faulty RAM, a Rust defect or an antivirus intervention. [Microsoft's access-violation
diagnostics](https://learn.microsoft.com/en-us/shows/inside/c0000005) explain the
exception parameters and context needed to distinguish access type and address.
[Python's exit-status documentation](https://docs.python.org/3.13/library/sys.html#sys.exit)
explains the separate cleanup status.

Windows Error Reporting still has both `Report.wer` files. Each names the frozen
executable and a temporary minidump, but neither listed minidump remains at that
path, and neither archived report directory contains a dump. The report's module
offset alone is insufficient to reconstruct a native stack. The compact
`failure-evidence.json` records exact file identities, available fields and dump
availability; the full WER files remain outside Git.

The frozen runner and verifier keep their journals open for the duration of the
job. The verifier's first event is present but its launcher-exit event is absent.
New transfer receipts exist for the failed cells while their associated journal
entries and final verdicts are missing. This bounds where evidence was lost; it
does not establish whether `write`, `flush`, `fsync` or another operation first
failed. No missing success event is reconstructed.

## Demonstrated runner defect and repair

A synthetic `PermissionError` during the runner's journal flush reproduces the
diagnostic loss: the original implementation raises an exception but creates no
durable record of its operation or traceback. The new `DurableJournal` records
the first failure through a fresh, exclusive file handle in
`journal-failure.json`. It captures the attempted event, operation, errno,
WinError when supplied, filename when supplied, and complete exception chain.

It rejects later appends to the failed handle. A separate close failure is kept
in `journal-close-failure.json` and cannot replace the primary exception or its
receipt. If diagnostic storage also fails, that failure is attached to the
original exception; existing evidence is never overwritten. The runner stops
only processes it owns, after releasing the journal lock, so a background guard
failure does not leave campaigns running with an unusable evidence log.
If the OS refuses a child termination, the runner still attempts every other
owned child, preserves a separate per-cell stop-failure diagnostic and retains
the journal error. It reports an attempted stop, never a confirmed termination
for the refused child.

Every diagnostic says `passed: false`, `qualification: false` and
`s25_complete: false`. It is neither a replacement journal entry nor a successful
matrix result. Normal-run output, limits, seeds, ordering and retained
verification rules are unchanged. The old frozen runner and one-off dependent
verifier are immutable; this patch does not change their past results. Only
`run_matrix` uses the new journal helper. The frozen one-off dependent waiter
has not been repaired, and the existing reusable `--verify` implementation is
unchanged. No verifier-journal repair is claimed.

## Checks and limits

- The injected flush regression failed against the original implementation and
  passed with the repair. Seven focused tests cover flush loss, sync loss, repeated
  appends, secondary close errors, exclusive diagnostic retention and a guard
  failure while an owned synthetic child is waiting. They also check a refused
  stop with another child still stoppable, and a close failure after passing cells
  that must prevent a successful aggregate.
- Complete campaign tooling discovery passed **105 tests, with one existing
  directory-symlink permission skip**. An earlier 76-test run covered only the
  materialized stability/distributed modules; its raw output is retained without
  presenting it as the complete suite. The final run includes all four modules. These
  use synthetic fixtures and do not run a native campaign. Original outputs are
  retained in `logs/`.
- [Fresh C: and D: probes](volume-probe.json) passed opening, two writes,
  flushing, `fsync`, reading while open, closing and reading afterward in newly
  created task directories. No retained campaign path was opened for writing.
  These immediate probes do not reproduce a fourteen-hour handle lifetime,
  memory load or the prior failure's environment. The original failure remains.

The reproducible probe is [probe_volumes.py](probe_volumes.py). Run a copied
recipe from a new evidence directory; it refuses to overwrite its result.
The focused and full tooling commands are retained in `validation.json`.

## Next native diagnostic, before another full matrix

1. Coordinate one native-worker slot and a fresh diagnostic directory with the
   integration owner. Do not rerun all twenty-four cells yet, reuse old completed
   cells for a new result, or mutate the stopped attempt.
2. Start with Japan / seed 7, the first confirmed access violation, using the
   exact declared binary, request and matching symbols in a new output directory.
   Preserve its full horizon and ordinary rules. Label the run diagnostic only;
   it cannot qualify the matrix.
3. Capture an unhandled-exception dump for that owned process. No `procdump`,
   `procdump64`, `cdb` or `windbg` command was found on PATH in this checkout's
   session. Microsoft's [ProcDump](https://learn.microsoft.com/en-us/sysinternals/downloads/procdump)
   supports starting a named executable and writing a full dump on an unhandled
   exception with `-ma -e -x <new-dump-directory> <image> <arguments>`. Pin the
   diagnostic tool and matching binary/symbols before the coordinated run;
   confirm capture with a disposable failing process first. Do not install a
   system-wide debugger or attach to the user's game.
4. Preserve the dump, exception parameters, faulting instruction, native stack,
   module versions and memory/commit observations. Determine the failing access
   before changing gameplay, storage or resource limits. A checkpoint replay may
   be useful afterward, but a new process may not reproduce a lifetime-dependent
   failure and cannot replace the required full run.
5. Use the repaired runner for any new matrix candidate. Run retained verification
   explicitly after its terminal result exists, with fresh output and complete
   traceback retention; do not restart the stopped dependent waiter in place.

S25, G5 and CP1 remain open. This closes the demonstrated loss of journal-error
context, not the native crash investigation or the campaign qualification.
