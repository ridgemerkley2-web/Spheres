# Japan / seed 7 crash-capture preparation

**1 October 2026 — diagnostic preparation only.** The exact frozen executable and
full-horizon request are verified. Official Microsoft ProcDump captured an owned
test process successfully. A guarded single-case launcher is prepared for review.
**No Japan campaign was launched, no native cause was identified, and S25/G5/CP1
remain open.** [Machine receipt](evidence/preparation.json).

The original [failed matrix](../local-matrix-20260930/README.md) and
[journal diagnosis](../journal-diagnosis-20261001/README.md) remain unchanged.
This branch starts at `d0c6676b347c16226d7c7db9371a36cd0a51ce92`; it changes no
simulation, campaign data, acceptance limit, workboard, saved game or old evidence.

## Frozen inputs and symbol limitation

| Input | Verified identity |
|---|---|
| Candidate | `68ba0622ec709b78617aadd1f9198d18f532bb32` |
| Original executable | `D:\spheres-offload\codex-next-20260928\lazy-digest-validation-20260930\spheres-web-test.exe` |
| Executable SHA-256 | `7c0a07733fa7079c1ee2d21691be6a89b0c26e2a977e44b2356b0ff491562bf8` (353,201,472 bytes) |
| Original request | `full-matrix-local-20260930-01\cells\japan-7\request.json` under the same D: root |
| Request SHA-256 | `283ffcbf56774b69abc6af7bdc1d561d7cbd9af0dbc868c331e7d85d6f3b746e` |
| Test | `s25_stability_tests::s25_stability_cell`, one test thread |
| Campaign | Japan, seed 7, through **2035-12-31**, unchanged ordinary rules |

[PE inspection](evidence/pe-inspection.json) finds **85,715 COFF symbol-table
entries**, including the exact test function, and `.pdata`/`.xdata` unwind data.
These embedded function addresses belong to the exact frozen image. However,
the [original build artifact](evidence/frozen-build-artifact.json) says
`debuginfo: 0`; there are **no CodeView/PDB references or DWARF sections**. No
matching source-line, local-variable or inlined-call-site debug symbols are
available. A newly compiled debug companion would be a separately declared
executable; it cannot supply matching addresses for this frozen image. None was
built. A future dump can still preserve the fault context, modules and memory,
with this explicit limit on symbolic source diagnosis.

Current `ntdll.dll` is version `10.0.26100.7920`, matching the earlier WER version;
its current hash is [retained](evidence/current-ntdll.json). This is not evidence
that the same access violation will recur or that `ntdll` caused it.

## Capture proof

Downloaded **ProcDump 12.01** from the download linked by
[Microsoft's documentation](https://learn.microsoft.com/en-us/sysinternals/downloads/procdump).
The selected `procdump64.exe` is 741,216 bytes, SHA-256
`d1fc99ae304bd1d2bf28abeb62531da959e2431916194981b88c958fd713a8e6`, with a valid
Microsoft Corporation Authenticode signature. [All three executable signatures](evidence/tool-signatures.json)
and the ZIP pin are retained. The portable files stay outside Git at:

`D:\spheres-offload\codex-next-20260928\s25-crash-prep-raw-20261001\procdump`

The sole deliberate registry setup was normal per-user
`HKCU\Software\Sysinternals\ProcDump\EulaAccepted=1`, previously absent.
Selected values at both AeDebug keys and the global LocalDumps key were unchanged
in [before](evidence/registry-before.json)/[after](evidence/registry-after.json)
snapshots. No `-i`, global debugger registration, WER policy change or attachment
to an existing game was used. This is a comparison of those selected keys, not a
claim to have audited all registry activity.

[owned_fault.py](owned_fault.py) starts a deliberately invalid thread inside its
own disposable interpreter. Command used:

```powershell
& 'D:\spheres-offload\codex-next-20260928\s25-crash-prep-raw-20261001\procdump\procdump64.exe' -accepteula -ma -e -n 1 -x 'D:\spheres-offload\codex-next-20260928\s25-crash-prep-raw-20261001\owned-proof-dumps' 'C:\Users\ridge\.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe' '<this packet>\owned_fault.py'
```

The [retained proof](evidence/owned-proof-readable.txt) and
[independent dump parsing](evidence/final-dump-checks/owned-dump-inspection.json)
agree on PID **42036**, faulting thread **39588**, exception **0xc0000005**, address
**1**, parameters **[8, 1]**, and full-memory flag. The **40,903,229-byte** dump has
a complete memory payload, module stream and 1,232-byte thread context. Its hash
is `988384b82553b85847745e10d9b69794419deb70a6775b7af9179654aca5e39c`.
The original mixed-encoding output is preserved separately; the readable file
removes NUL bytes and normalizes line endings and trailing whitespace. The dump remains outside Git, under the external raw root.

## Guarded single-case plan

[launch-japan-diagnostic.ps1](launch-japan-diagnostic.ps1) defaults to a **dry run**.
It pins the original executable/request and signed tool, rejects existing or
ambiguous output paths and junction ancestors, and freezes the dump inspector.
An explicit `-Run` creates a new D: directory, copies the original request bytes,
then runs ProcDump with `-ma -e -n 1 -x`. The native output directory stays absent
until the frozen test creates it, as required by the original test precondition.

The wrapper flushes its journal to disk and checks resources every two seconds.
It retains the original **3 GiB free-disk floor on C: and D:** and adds a **64 GiB
dump reserve**, crediting bytes already consumed by its own dump. It requires
3 GiB available commit before launch and stops its own run below 1 GiB. Protective
bounds are one dump, 64 MiB per monitor log, 128 GiB total output and 12 hours.
These are diagnostic safety ceilings: reaching one records an incomplete run,
never a shorter passing campaign. Prior Japan/7 lasted **25,066.82 seconds
(6h 57m)** before crashing; the **52,058.57-second** figure belongs to the entire
four-worker matrix launcher. The 12-hour child limit covers that prior failure
with margin; it does not guarantee reproduction.

Only the launched monitor and its exact-path native child are controlled. The
held child handle is checked against captured creation time, executable path and
monitor lifetime. Cleanup stops the owned monitor before a final child sweep,
then tries every owned handle. Auxiliary console hosts are observed, not targeted.
Native failure remains failure even if ProcDump exits zero. Journal, inventory and
cleanup errors remain separate; dump parsing checks complete memory payload and
owned PID. A dump inspector failure does not erase the earlier error. The parser
uses read-only mmap plus streamed hashing, not a whole-dump RAM allocation.

**Prepared, not executed:** after independent review and a fresh resource check,
the parent can run the following in a dedicated hidden PowerShell process:

```powershell
& '<this packet>\launch-japan-diagnostic.ps1' -OutputDirectory 'D:\spheres-offload\codex-next-20260928\japan-7-diagnostic-20261001-01' -Run
```

Retain that process's stdout/stderr outside its new output directory, too, so
even a setup or final receipt-write failure leaves an operator log. Do not reuse
the proposed directory if another attempt has created it. No dependent verifier
or full matrix is started by this script. All terminal receipts keep qualification,
campaign pass and S25 completion false; a non-reproducing diagnostic proves no gate.

## Resource conditions and validation

The [snapshot](evidence/environment-before.json) showed about 9.5 GiB free physical
memory, 8.1 GiB available virtual/commit headroom, 58.6 GiB free on C: and 704.7 GiB
free on D:. These are observations, not a reservation. The later
[three-second sample](evidence/resource-sample.json) showed two Git processes each
using roughly one core. No `cargo` or `rustc` process was observed; an idle,
protected `GCC.exe` could not be identified and is not assumed to be a compiler.
Claude and the user's game remained running and untouched. The existing
`C:\Users\ridge\spheres-companies\target` junction points to
`D:\spheres-offload\spheres-companies\target`; it was read only. No caches or
offloaded files were moved or removed. Coordinate a fresh slot before the long run.

- **12 launcher checks:** parse, exact plan/refusal paths, exclusive durable
  writes, changed-hash refusal, absent native directory and reserve formula.
- **7 dump checks:** owned identity, fault parameters, full payload/context and
  rejection of a deliberately truncated dump.
- **5 real disposable monitor cases:** the actual production monitor
  try/catch/finally and helper functions ran with owned Python children. Clean
  exit, captured AV, synthetic low-commit halt, late journal failure and an early journal failure before child registration all
  retained correct outcomes and left their owned handles exited.

[Final synthetic cases](evidence/synthetic-final/monitor-checks.log) retain the exact
launcher source hash. Two earlier failures remain visible: ProcDump creates a
console-host child, which the first version incorrectly rejected. The first
failed run's log and the second failed run's explicit result are retained, along
with a stopped inefficient PE-inspection attempt. No failed checkpoint was
rewritten as success. The process cases inject resource snapshots; the actual
multi-hour game, real low-disk conditions and failure of Windows termination APIs
have not been exercised.

Reproduce without launching a campaign using [check-launcher.ps1](check-launcher.ps1),
[check-monitor.ps1](check-monitor.ps1) (new disposable output root) and
[check_dump.py](check_dump.py) (retained proof plus new truncation-fixture root).
The readers follow Microsoft's
[PE format](https://learn.microsoft.com/en-us/windows/win32/debug/pe-format) and
[exception-stream structure](https://learn.microsoft.com/en-us/windows/win32/api/minidumpapiset/ns-minidumpapiset-minidump_exception_stream).
Hashes and payload paths are listed in [manifest.json](manifest.json).
