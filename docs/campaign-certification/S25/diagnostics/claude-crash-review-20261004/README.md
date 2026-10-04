# S25 native-crash review: 30 September wave 2

**Status:** proposed for Codex review under `CODEX-S25-MATRIX-01`. **Reviewer label:** Claude, automated agent - not human or Codex review. This packet does not qualify anything, close any task or accept anything. **S25, S27, G5 and CP1 remain open.** No existing repository file was edited, and nothing was committed or pushed.

- **Integration tip read:** `39369f0ee6451466d03778d0273a7f72aafcf8c0`.
- **Crashed candidate:** `68ba0622ec709b78617aadd1f9198d18f532bb32`.
- **Frozen Windows test binary:** sha256 `7c0a07733fa7079c1ee2d21691be6a89b0c26e2a977e44b2356b0ff491562bf8`, 353,201,472 B.

On 4 October 2026, in a scratch Linux container, Claude did three things:
1. Analysed the timeline of the retained [30 September evidence](../../preparation/local-matrix-20260930/interruption-20260930/README.md), read-only.
2. Reviewed the 68ba0622 source and ran short Linux probes.
3. Reviewed each leading hypothesis adversarially twice: once through an evidence lens and once through a Windows-mechanics lens.

Both analyses and all 20 verdicts are copied verbatim into `adversarial-review/`; the only addition is a `lens` label on each verdict.

## 1. Bottom line

Japan / 7 and India / 7 died from access violations (`0xc0000005`) inside `ntdll.dll` 10.0.26100.7920, 16.76 s apart, in processes at unrelated simulated dates (2031-01 and 2024-12) and ages (6 h 57 m and 5 h 46 m). **Supported with medium confidence (an inference from code and retained artifacts, because the journal entry itself was lost; the accepted [journal diagnosis](../../preparation/journal-diagnosis-20261001/README.md) does not attribute the exit-1s to the kill path):** the other failures follow from long-lived file handles on D: (recorded in 28 September notes as the external USB drive) failing while newly opened D: handles still worked. Those failures are the two exit-1 workers, the runner's exit 120 and the verifier's `PermissionError`. The frozen runner turns the failed journal write into `halt_children()` → `Popen.kill()` → `TerminateProcess(handle, 1)`. **Unsupported, or contradicted by the exit signature:** every in-process cause. A deterministic simulation defect, a stack overflow and a Rust out-of-memory abort would each leave a different exit signature. Heap corruption has no supporting evidence and does not by itself explain two separate processes failing 17 s apart. **Plausible but not established:** a shared host-level event at about 23:28 UTC. If the two fault times were independent and uniformly distributed over the 20,755 s four-worker window, a gap this small would occur only about 0.16% of the time. Its mechanism is unknown: neither commit exhaustion nor the USB D: failure is shown to produce `0xc0000005` in ntdll. Linux probes of the same revision matched every retained Windows Japan / 7 fingerprint through 1995-02-01 without crashing; valgrind, run on January 1990 only, found no memory errors. They can neither reproduce nor exclude a Windows host event. **The native root cause is unknown**, and the decisive checks are on Windows (section 5).

## 2. Timeline of the 30 September wave-2 failure (UTC)

All four wave-2 workers ran together from 17:43:07.69 to 23:29:02.85 (20,755 s). In wave 1, four concurrent workers passed in 26,854–31,165 s. "Expected duration" is the wave-1 mean, 29,835 s. The horizon is 16,801 simulated days.

| Cell | Process start | End | Exit | Age at fault/kill | Simulated position | % of expected duration (wave-1 mean) | % of horizon |
|---|---|---|---|---|---|---|---|
| Japan / 7 | 16:31:15.41 | fault 23:28:41.13; reaped 23:29:02.850 | `0xc0000005`, ntdll+`0x166167` | 25,045.7 s (6 h 57 m) | after 2031-01-01 checkpoint, before 2031-02-01 save's staging file | 83.9% | 89.1% |
| India / 7 | 17:43:07.69 | fault 23:28:57.88; reaped 23:29:02.850 | `0xc0000005`, ntdll+`0x10579` | 20,750.2 s (5 h 46 m) | after 2024-12-01, before 2025-01-01 staging file | 69.5% | 75.9% |
| Japan / 42 | 17:29:07.69 | 23:29:49.391 | 1 | 21,641.1 s | last checkpoint 2027-10-01 | 72.5% | 82.1% |
| India / 1990 | 17:40:25.95 | 23:29:49.124 | 1 | 20,962.6 s | last checkpoint 2023-10-01 | 70.3% | 73.4% |

- **Gap between the faults:** 16.76 s by Event 1000 time, or 17.09 s by WER EventTime.
- **Exits:** the two crashed processes were reaped 0.37 ms apart. The exit-1 exits came 0.27 s apart, 46 s later.
- **Runner:** exited 120 at 23:30:51.085, with empty stdout and stderr on D:.
- **Verifier:** recorded `[Errno 13]` at 23:30:51.323.
- **Pace:** projecting each worker's pace to the full horizon gives 26.4k–28.6k s, no slower than the completed 26.9k–31.2k s, so there is no visible slowdown.
- **State size:** assuming uniform pace, the summed archive state was about 541 MB at the faults, against about 709 MB at the wave-1 peak. Summing each wave-2 cell's last checkpoint instead gives 526.7 MB, and aligning the wave-1 cells by simulated date gives a 725.8 MB peak. Either way, the state at the faults was smaller than the state wave 1 ran through without a fault.
- **Concurrent build:** a release compile ran from 23:19:04 to about 23:31:21 and succeeded.

Sources: `interruption-20260930/evidence/{windows-crashes.json,cells/*}` and `journal-diagnosis-20261001/failure-evidence.json`, via `scripts/timeline.py` and `state.py`. Outputs are in `analysis-outputs/`. The 526.7 MB and 725.8 MB alternatives come from the E-H1 evidence-lens corrections and were recomputed from the same `result.json` files during the fact-check.

## 3. Hypotheses after adversarial review

The reviewers set `refuted: true` both for contradicted hypotheses and, by their default rule, for unsupported ones. This table separates the two:
- **Refuted:** contradicted by the recorded exit signature or by data.
- **Unsupported:** no checkable evidence, but not disproven.
- **Supported:** a demonstrated mechanism that fits the evidence.

IDs starting E-H come from the timeline analysis; IDs starting S-H come from the source review. "Not reviewed" means the hypothesis was outside the top five that went to adversarial review.

| # | Hypothesis | IDs (lens verdicts) | Standing | Reason |
|---|---|---|---|---|
| 1 | Deterministic simulation or logic defect at a specific simulated state | E-H2 (refuted ×2) | **Refuted** | A safe-Rust defect in this release build ends as an Err or panic (stderr text, result.json failure, exit 101), `0xc00000fd` or `0xc0000409`. It never ends as a silent `0xc0000005` in ntdll. The faults were also 17 s apart at unrelated simulated dates. Its residual variants (deep recursion, or undefined behaviour in std or a dependency) are rows 2 and 4; on that basis the evidence lens rated it unsupported rather than disproven. |
| 2 | Stack exhaustion | E-H3, S-H4 (refuted ×4) | **Refuted** | It would give `0xc00000fd` and "has overflowed its stack". Observed: `0xc0000005` and 0-byte stderr. JSON depth is 13 in Linux snapshots from 1991 to 1995; serde_json's limit is 128. |
| 3 | Rust allocation failure (OOM abort) | S-H5 (refuted ×2) | **Refuted** | It would print "memory allocation of N bytes failed" and give `0xc0000409` in the exe. Observed: `0xc0000005` at two different ntdll offsets. |
| 4 | Heap corruption from unsafe code in std or dependencies (serde_json, itoa, memchr, zmij), or a miscompile | E-H6, S-H3 (refuted ×4) | **Unsupported** | The exit code does not exclude it: undetected NT-heap damage can fault inside ntdll. But there is no dump, the offsets are unresolved and valgrind was clean for one simulated month on Linux. It also does not explain faults 17 s apart in separate processes, or the Python-side failures. |
| 5 | Worker memory as a contributor | S-H2 (refuted ×2) | **Unsupported** | Workers are large (Linux VmHWM 1.50 GiB by 1994). But wave 1 ran four workers through the larger 2008–2014 state without a fault, and no memory data exists from crash time. |
| 6 | Host commit or memory exhaustion at about 23:28 | E-H1 (refuted ×2); S-H1 mechanism (refuted by the mechanics lens) | **Unsupported** | No commit sample exists for 23:28; the 7.94–8.15 GiB headroom was measured 3.5–4 h later. Known exhaustion paths give `0xc0000409`, `0xc00000fd` or `0xc0000006`. A rustc release build running at the same time survived. The Python errors were EACCES, not MemoryError. |
| 7 | Unbounded growth of simulated state | S-H6 (not reviewed) | **Unsupported**; contradicted by the size data | Archives stay between 116.6 and 186.7 MB after 1994 and peak around 2010. At the crash they were 126.8–133.5 MB, 0.71–0.73 of each process's own peak, which those processes had already survived. |
| 8 | windows-gnu unwinding or TLS path faulting in ntdll | S-H7 (not reviewed) | **Unsupported** | No panic message was printed, and catch_unwind recorded nothing. Step 5.1 would test this. |
| 9 | A shared host-level event at about 23:28 | S-H1 core (evidence lens: not refuted, overstated) | **Plausible; mechanism not established** | Two independent processes faulted 16.76 s apart, about a 0.16% chance if fault times were independent and uniform over the four-worker window. Every candidate (commit, storage, driver, injected software, hardware) lacks direct evidence. |
| 10 | D: handle failure causing the evidence loss and the kills | E-H4 narrowed (not refuted ×2, overstated) | **Supported (medium; inference)** | Handles held open on D: failed: the runner journal, the launcher's stdout and stderr, and the verifier's events.jsonl. New D: handles worked. The frozen runner and verifier code then explain both exit-1s, exit 120 and Errno 13. The Win32 error is unknown, and "device not ready" is plausible but not shown. The failure's start is only bounded to after 17:43. The evidence lens adds that a release build on D: ran successfully from 23:19:04 to 23:36:50, so any volume-wide event more likely came before 23:19. |
| 11 | The same D: event caused the native faults | E-H4 native link (contradicted by the mechanics lens) | **Unsupported** (weakly contradicted) | A failed page-in of the executable mapped from D: gives `0xc0000006`, even inside ntdll. The two workers running the same image kept going until they were killed. |
| 12 | Hardware fault or an injected third-party module | E-H5 (not reviewed) | **Open**; no evidence either way | No WHEA, security-product or loaded-module evidence was collected. |

Figure corrections from each verdict's `corrections` list (for example the headroom, the 17.09 s WER gap and the alternative state sums) are applied where those figures appear above.

## 4. Linux measurements

**Setup.** The binary is `spheres_web-7f381178d9133e63`, built from a clean scratch worktree at `68ba0622ec709b78617aadd1f9198d18f532bb32` with rustc 1.97.0 for `x86_64-unknown-linux-gnu`. Its sha256 is `a6e0dc501d4b23c2917b89f68cd4fff567927d33faeb03cbbc00300263f77f45`, recorded at 01:13Z; no rebuild is recorded before 03:08Z. Every probe reports `compiled_revision` `68ba0622ec70`. The probes ran 01:27–02:53Z on 4 CPUs with about 16 GiB of memory and no swap. Linux RSS is not Windows commit. Commands are in `linux-probes/launch-commands.txt`, and `scripts/linux_measurements.py` produces `analysis-outputs/linux-measurements.txt`.

**Memory high-water by simulated year.** From probe `japan7-to1995`, sampled every second, with 2 threads throughout. VmHWM is cumulative.

| Simulated year | RSS min–max | Cumulative VmHWM | Max canonical archive bytes |
|---|---|---|---|
| 1990 | 164–722 MiB | 733 MiB (750,828 kB) | 44,993,670 |
| 1991 | 621–903 MiB | 932 MiB (954,372 kB) | 78,062,994 |
| 1992 | 699–1,136 MiB | 1,183 MiB (1,211,144 kB) | 105,366,623 |
| 1993 | 874–1,451 MiB | 1,500 MiB (1,535,716 kB) | 134,408,540 |
| 1994 | 1,089–1,491 MiB | 1,539 MiB (1,576,344 kB), reached after 1994-11-01 | 142,031,038 |
| 1995 (to 02-01; then killed, exit 143) | 1,143–1,483 MiB | 1,539 MiB | 137,120,071 |

RssFile was about 32 MB. The 2005–2015 peak-state years and the 2031 crash window were **not** measured.

**Stack high-water.** libtest runs the cell on a spawned thread (`s25_stability_tests::s25_stability_cell`) with a 2,048 KiB stack; `stack/stackprobe` confirms both the default and the `RUST_MIN_STACK` override. Resident stack was 84 KiB at the opening, then 108 KiB. From 1990-02-01 to 1990-04-01 it held at 116 KiB in continuous 0.5 s samples (`stack/h4-stack-hwm-review/stack.csv`). Point readings near 1991-04, 1992-05, 1994-07/08 and 1995-02 were also 116 KiB (transcribed into `stack/smaps-point-readings.txt`). A deliberate overflow printed "has overflowed its stack" and exited 134. Linux frame sizes are only a proxy for Windows.

**Archive plateau.** JSON depth was 13 at all eight snapshots from 1991-07 to 1995-01, while the value count grew from 3.6M (1991-07) to 7.75M (1994-07), then 7.66M (1995-01). Linux archives grew from 63.6 MB (1991-07) to 139.3 MB (1994-07), then 137.0 MB (1995-01). On Windows, every cell peaks at 175.4–186.7 MB around 2010 and then falls back; Japan / 7 went from 185.2 MB at its peak to 133.5 MB at 2031-01-01.

**Cross-platform trajectory.** All 127 Linux comparison rows through 1995-02-01 are identical to the Windows Japan / 7 rows. The comparison covers kind, date, `canonical_bytes`, `canonical_fnv64`, `history_rows` and `log_rows`.

**Valgrind.** memcheck 3.22.0 ran the paired cell from 1990-01-01 to 01-31 (one monthly write and reload; 13–26 MB archives). The test passed in 409.45 s with `ERROR SUMMARY: 0 errors`, after 37,640,321 allocations and 10,523,808,523 bytes allocated in total. This is weak evidence for the NT heap or for late-campaign state.

**Out-of-memory signature.** Six-month runs passed with `ulimit -v` at 1,150,000 KB and at 900,000 KB. At 650,000 KB the binary printed "memory allocation of 632 bytes failed", then a second 192-byte failure, and exited 134 (SIGABRT, no SIGSEGV). So this binary's out-of-memory failure is loud. That says nothing about memory pressure on Windows.

## 5. Next steps for the Windows owner (in priority order)

Write all outputs to a new evidence directory. Do not touch the stopped attempt.

1. **Resolve the two ntdll offsets.** No dump is needed.
   - First confirm that `C:\Windows\System32\ntdll.dll` still has sha256 `E758FA414086189B0DF0B569750D1E4E164EE1A43598D758536ACEC159A5F3E5` (10.0.26100.7920; WER module timestamp `0x5ffc11eb`). If not, obtain that exact build first.
   - Then run:
   ```
   cdb -z C:\Windows\System32\ntdll.dll -y "srv*C:\symbols*https://msdl.microsoft.com/download/symbols" -c ".reload /f; lmvm ntdll; ln ntdll+0x166167; ub ntdll+0x166167 L8; u ntdll+0x166167 L4; ln ntdll+0x10579; ub ntdll+0x10579 L8; u ntdll+0x10579 L4; q" > ntdll-offsets.txt
   ```
   - In WinDbg instead: open the DLL as a dump, then run `.symfix C:\symbols`, `.reload` and the same `ln` commands.
   - Heap routines (`RtlpAllocateHeap`, `RtlpFreeHeap`, `RtlpLowFragHeap*`) would raise row 4. `RtlDispatchException`, `RtlUnwindEx`, `RtlVirtualUnwind` or `RtlLookupFunctionEntry` would raise rows 2 and 8.
2. **Pull the Windows logs for 30 Sep 23:20–23:35 UTC** before they roll over. For storage and USB providers, also pull 17:43–23:31. First check how far back the logs reach with `Get-WinEvent -LogName System -MaxEvents 1 -Oldest`.
   ```powershell
   $from=[DateTime]::SpecifyKind([DateTime]'2026-09-30 23:20:00','Utc').ToLocalTime()
   $to=[DateTime]::SpecifyKind([DateTime]'2026-09-30 23:35:00','Utc').ToLocalTime()
   Get-WinEvent -FilterHashtable @{LogName='System','Application';StartTime=$from;EndTime=$to} |
     Select-Object @{n='utc';e={$_.TimeCreated.ToUniversalTime().ToString('o')}},LogName,ProviderName,Id,LevelDisplayName,Message |
     Export-Csv -NoTypeInformation -Encoding UTF8 .\events-20260930-2320-2335.csv
   ```
   Look for:
   - Resource-Exhaustion-Detector **2004**, which lists the top commit users, and Application Popup **26**.
   - `disk` **7, 11, 51, 153, 157**; Ntfs **50, 55, 98, 140**; volmgr, partmgr, storport, USBSTOR and UASPStor.
   - Kernel-PnP (System), plus the separate logs `Microsoft-Windows-Kernel-PnP/Configuration` (**400, 410, 420**) and `Microsoft-Windows-Partition/Diagnostic` (**1006**). Query each separate log with the same filter and its own `LogName`.
   - Any WHEA-Logger, Kernel-Power or Power-Troubleshooter event.
   - `Microsoft-Windows-Windows Defender/Operational` **1116, 1117, 1123, 5007**, and any third-party security product's log.
   - Application Error **1000**, WER **1001** and Application Hang **1002**, for any process.

   Also record D:'s `Get-PhysicalDisk` BusType and the pagefile settings.
3. **Inspect the LoadedModule lists in the two retained `Report.wer` files.** They are `japan-7.Report.wer` (sha256 `20000af0…5d7929993`) and `india-7.Report.wer` (`68516fd9…f750d691c`), in `D:\spheres-offload\codex-next-20260928\s25-diagnosis-raw-20261001T021229607793Z\`. Verify the hashes, then run `Select-String -Path <file> -Encoding Unicode -Pattern '^LoadedModule\[' | ForEach-Object Line`. Flag any module outside `C:\Windows` other than the test executable, and note whether both processes loaded it.
4. **Record the outcome of the 1 Oct Japan / 7 ProcDump diagnostic.** Its files are in `japan-7-diagnostic-20261001-01` and `japan-7-launch-20261001-01` under `D:\spheres-offload\codex-next-20260928\`. Record:
   - the terminal result;
   - the last `end_native_date`, and whether it passed 2031-01-01 → 2031-02-01;
   - any dump and exception;
   - peak private bytes and commit from the 2-second samples.

   The repository says only that the run stopped at its 12-hour bound. It ran as a single worker under `procdump -x` without `_NO_DEBUG_HEAP`, so it probably used the debug heap. A clean pass would not clear row 4.
5. **Keep long-lived journals and binaries off the external USB drive.** This covers the runner journal, the launcher's stdout and stderr, the verifier's journal and stderr, and `spheres-web-test.exe`. Keep them on an internal volume, and copy them to D: only after the terminal result. This removes the row-10 chain and the risk of image page-in failures, whatever the cause turns out to be.
6. **Add commit-charge and private-bytes sampling to the runner.** Every 5 s or less, record system Committed Bytes and Commit Limit. For each worker, record `PrivateMemorySize64`, `PeakPagedMemorySize64`, `WorkingSet64` and the last `end_native_date`. Write these to a separate append-only file on an internal volume. Today neither the runner nor the native result records memory, or wall time per checkpoint. `launch-japan-diagnostic.ps1` (lines 53–61 and 135–144) is a working model.
7. **Optional efficiency item; not a demonstrated cause.** At 68ba0622, `storage::write` (`storage.rs:257–277`) reads the previous 130–180 MB monthly file and fully decodes it into a third `Game`, including a whole-world serde_json `Value`. It does this only to confirm the file is sound before it becomes the `.bak`. `atomic_write` (line 219) then reads the file again. A validation that skips building a full `Game` could keep the guarantee with a smaller monthly spike. Any such change needs its own review and must not be presented as the crash fix.

## 6. Full-horizon Linux rerun (in progress)

On 4 October 2026, Claude started a Linux rerun of Japan / 7 at 68ba0622 through 2035-12-31 in a scratch container. It samples VmRSS, VmHWM, VmSize and thread count every 30 s. VmHWM is cumulative, so it captures the overall peak but not each month's peak. **Its result will be added later if it completes.** Nothing above depends on it.

- **Attempt 01** started at 01:13:31Z and failed after 0.08 s with exit 101 (recorded at 01:14:01Z by the 30 s sampler), at `s25_stability_tests.rs:634`: "Absolute immutable request and NEW output directory required". The launcher had run `mkdir -p $D/native` before starting the test, but the test requires a new output directory. **This was a launcher error, not a game result.** Logs are in `linux-rerun/attempt-01-failed/`.
- **Attempt 02** was rebuilt at 03:08:21Z and launched at 03:10:11Z as PID 1821, after the stale directory was removed. It uses the same path and revision (`68ba0622ec70`) as section 4, but it is a different build: sha256 `250624d4aedb7fbbfa5474d895fff17f62702c2445d6012db49b7a582513f074`, 353,075,664 B. Its inputs are in `linux-rerun/attempt-02-launch/`, and its outputs remain scratch-only. At 03:34Z, when this packet was written, it had passed 1994-07-01 and was still running; a read-only check at 03:46Z found it past 1995-12-01 and still running.

If it passes 2031-01 → 2031-02 and reaches 2035-12-31 with fingerprints identical to Windows, that would count further against a platform-independent state trigger for Japan / 7 (row 1). It would also give the Linux peak-state memory. It cannot test the Windows host, the NT heap, the USB drive or WER.

## Files and limits

- `review.json`: hypotheses, standings, key facts with sources, and next steps.
- `files.json`: bytes and sha256 for every packet file, plus hashes of large scratch-only files that were not copied. `.gitattributes` (`* -text`) keeps the recorded bytes stable on a Windows checkout.
- `adversarial-review/`: both analyses and all 20 verdicts.
- `scripts/`, `analysis-outputs/`, `linux-probes/` and `linux-rerun/`: scripts, outputs, inputs, logs, `memory.csv` and `result.json`.

This packet does **not** establish any of the following:
- a native root cause;
- that the D: event or memory pressure caused the access violations;
- any Windows behaviour, from the Linux results;
- matrix qualification;
- closure of `CODEX-S25-MATRIX-01`, S25, S27, G5 or CP1;
- acceptance by Codex, root or a human.

Claims about Windows and Rust-std internals rest on the reviewers' platform knowledge, as marked in `adversarial-review/`; nothing was run on Windows.

A second automated Claude pass fact-checked this packet's figures, timestamps, citations and hashes against the underlying files on 4 October and corrected several of them. That is still not human or Codex review.
