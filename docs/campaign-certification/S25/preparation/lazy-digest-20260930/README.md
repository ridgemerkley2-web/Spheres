# Lossless stability comparison optimization

Candidate `68ba0622ec709b78617aadd1f9198d18f532bb32` passes the focused native checks and both standard 398-day pilots. This is preparation, not a full-matrix or campaign-certification pass.

The test harness now computes the two diagnostic archive fingerprints only when complete canonical bytes differ. Both native encodes, every successful comparison fingerprint, native validators, monthly save/reloads, daily invariants and exact byte equality remain. The only excluded archive field remains the terminal wall-clock save timestamp. The new regression compares the previous implementation directly, including same-length valid mutations, exact failure text and deliberately colliding diagnostic hashes.

- Native focused checks: 9 passed, 2 explicitly invoked preflights ignored.
- Orchestration checks: 71 run, 70 passed and one existing privilege-dependent skip.
- New and old pilots: France/1990 and Tonga/7 each complete 398 days per leg, 13 monthly reloads, 796 daily checks, 60 native validations and two terminal reloads.
- Both frozen retained-evidence verifications pass, checking eight complete gzip archives apiece.
- The separate execution review fully reads 16 gzip streams (993,665,130 decoded bytes). All eight corresponding old/new archive pairs, 60 comparison rows, 32 action rows and every check agree. Only terminal `saved_unix` bytes are removed; no JSON round-trip or sampling replaces byte comparison.

The [source review](source-review/README.md) and [execution review](execution-review/README.md) distinguish reviewer roles and exact source/build scope. The old candidate is `ae8084e853a8eb01ef4d834017af3e94c8e2cc82`. Besides the harness change, the candidate range includes test-only simulation observers (absent from this ordinary simulation dependency) and two previously accepted map UI edits. The pilots overlap other workloads, so their elapsed times are not a speedup benchmark. No claim is made that lazy hashing alone fixes the hosted timeouts.

## Complete archive retention

Both pilot folders contain every original report, request, execution record, archive manifest and all sixteen complete compressed campaign archives. They are portable for evidence verification:

```powershell
python -B frozen/stability_matrix.py --verify new-pilot
python -B frozen/stability_matrix.py --verify old-pilot
```

The compiled binaries are excluded from Git. Their exact hashes and external locations remain in each freeze record and the execution review. This packet can verify and restore campaign evidence without those binaries; it cannot rerun native campaigns without obtaining the exact executable. The original source-review and execution-review manifests remain unchanged within their subfolders.

## Fresh full run

A new complete 24-cell attempt started at 2026-09-30T09:03:11Z on the new frozen candidate. It uses four workers, a 12-hour execution budget per cell, newly created NTFS-compressed SSD scratch, a 3 GiB free-space reserve and verified offload to D:. The 1990–2035 horizon, seeds, save/reload schedule, terminal pause and sandbox day are unchanged. No previous partial cell is reused.

The immutable launch and resource declarations are retained here. The live run is at `D:/spheres-offload/codex-next-20260928/full-matrix-local-20260930-01`. Completion and independent final verification remain pending. The [failed hosted run](../full-matrix-20260930/README.md) remains failed and retained separately. A1, S25, G5 and CP1 remain open.
