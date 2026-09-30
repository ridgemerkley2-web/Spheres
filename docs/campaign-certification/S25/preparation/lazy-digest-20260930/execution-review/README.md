# Old/new S25 pilot equivalence review

**PASS for the two recorded pilot cells; no full-matrix, performance or qualification claim.** Codex `/root/review_gap_submission` performed this offline review on 30 September 2026. The old native candidate is `ae8084e853a8eb01ef4d834017af3e94c8e2cc82`; the new candidate is `68ba0622ec709b78617aadd1f9198d18f532bb32`.

The reviewer freshly read and SHA-256 checked both native binaries, the frozen binary copy, frozen plan and Python tools, the recorded per-cell payloads, and all 16 gzip files. All gzip bodies were fully decoded: **993,665,130 bytes read**. Eight corresponding old/new archives were directly compared byte for byte, including both final legs, monthly scratch and its backup for each cell. The only removal is the exact terminal compact `,"saved_unix":N` member; all other bytes, key order and numeric spelling remain unchanged. No parsed-world normalization or sampled comparison was used. Every decoded size/SHA agrees with its retained original pin; every final canonical SHA agrees with the runner's recorded final pair. The native final legs also have equal complete canonical bytes within each candidate.

| Cell | Days per leg | Old/new comparison rows equal | All action rows equal | Native checks per candidate |
|---|---:|---:|---:|---|
| France / 1990 | 398 | 30 | 19 | 796 daily invariants, 60 native validations, 13 monthly reloads, 2 terminal reloads |
| Tonga / 7 | 398 | 30 | 13 | 796 daily invariants, 60 native validations, 13 monthly reloads, 2 terminal reloads |

Both runs start on 1990-01-01 and end on 1991-02-03 after the inclusive 1991-02-02 horizon. Their actual initial rules, all checks, actions, complete comparison rows, identities and dates agree. Each retained native execution exited zero and reports exactly one passed ignored test. Both matrix reports and both root-executed frozen-verifier receipts pass the requested two-cell pilot while explicitly leaving all 24 full-horizon cases missing. The receipts each verify eight retained archives; this review read and pinned those receipts but did **not** rerun the frozen verifier or separately recalculate diagnostic FNV. It independently performed the complete decoded-byte and SHA checks above.

## Source and build scope

Git comparison identifies five changed files in the native/UI source trees between these candidates: `government.rs`, the new `government_a1_observer.rs`, `s25_stability_tests.rs`, `globe3d.js` and `index.html`. The government additions are opt-in `cfg(test)` observation hooks. The retained Cargo trace confirms that the simulation dependency was built with `profile.test=false`, so these hooks are absent from the paired web-test binary's dependency. The native web change retains both encodes and exact canonical equality but computes mismatch-only diagnostic hashes lazily. Storage, other native web source, CLI, manifests, lockfile and simulation data are unchanged. The two embedded UI changes provide visible map-label selection for missing island geometry; they are outside this native paired execution. The entire candidate therefore must not be described as differing only in lazy diagnostics.

The retained build trace ends in `build-finished: success` and records a newly built optimized release web-test artifact. The recorded revision, actual native compiled-revision fields, frozen binary hashes and execution before/after pins agree. Both binary files and the new frozen binary copy were freshly rehashed by this review. Frozen Python tools and full plan match the candidate's Git blobs after explicitly identified CRLF-to-LF checkout normalization; their **raw frozen bytes** are pinned separately. No timestamp or source hash was inferred from a newer checkout.

Recorded focused checks show **9 passed, 0 failed, 2 manually invoked preflights ignored**. Recorded tooling checks show **71 run: 70 passed and one privilege-dependent skip**. These results were read from retained logs, not rerun here. The Cargo JSON trace establishes the artifact and success; it does not itself retain the original Cargo command line. This review performed no compilation or native execution.

The old pilot overlapped the new full-run host workload, and both pilots used concurrent workers. Their elapsed times are not an isolated performance benchmark or a measured speedup. Equivalence of these pilots does not establish full-2035 completion, the full-horizon pause/sandbox path, A1 acceptance, S25 or CP1. The original failed distributed matrix remains a separate retained result.

## Reproduction and reviewer limitation

`review.json` records all 91 freshly hashed input paths, exact source blobs, archive identities and scope. `verify_equivalence.py` reads existing evidence only and writes a new result path; it never launches the game or Cargo. With the original directory layout and Git objects available:

```powershell
python -B -X utf8 verify_equivalence.py --base D:/spheres-offload/codex-next-20260928 --repo PATH_TO_SPHERES_REPOSITORY --out NEW_REVIEW_JSON
```

The reviewer authored the bounded lazy-comparison patch. This is an independent check of root-built executions using a separate streaming comparison, not a claim of a second independent source review of that patch. Root owns the separate source review, compilation and actual native runs. `manifest.json` pins this compact review; it does not copy the large archives or binaries, which remain in the input locations recorded by `review.json` and the parent evidence packet.
