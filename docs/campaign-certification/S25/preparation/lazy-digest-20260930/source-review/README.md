# S25 harness cost review — one lossless candidate

**Recommend one small test-harness optimization for root review: compute full-archive mismatch fingerprints only when bytes actually differ.** It preserves every campaign byte comparison, validator, reload and scheduled boundary. No source change or execution was performed here, and no speedup or completed matrix is claimed.

Frozen matrix source: `5d970f6d7370baf16760585c641d81253d1c2175`; current integration `44098c5a48f491f43fbcb74d121f290380af0b12`. The inspected harness and storage Git bytes are identical at both revisions. The external batch manifest binds the original native binary; this review did not execute or independently rebuild it.

## Concrete redundant work

`spheres-web/src/s25_stability_tests.rs:190–195` computes both native encodes and canonical archives, then calls:

```rust
require(a == b, format!("Complete archive mismatch ...",
    /* existing date and lengths */, fingerprint(&a), fingerprint(&b)))?;
```

The second function argument is an eagerly constructed String. Even an equal comparison requests two whole-buffer FNV scans solely for a failure message that `require` immediately discards. The successful `compare()` separately calculates the required recorded canonical FNV at line218. Those success-record hashes must stay.

The bounded replacement is an explicit `if a != b { return Err(format!(/* identical existing diagnostic */)); }` followed by the same `Ok(a)`. Keep both `storage::encode` calls, canonical timestamp exception and exact byte equality. On mismatch, preserve the identical diagnostic with both hashes. Do not replace equality with a hash, omit native validators, share a Game across legs or skip a monthly reload.

A calendar-only reading of the existing full-horizon schedule yields 1,156 report comparisons plus4 other `equivalent` calls, or 1,160 calls if the cell completes. The source currently requests 2,320 success-path diagnostic fingerprints across those calls. This count is **not** an executed full-cell result. Actual compiler behavior and elapsed saving remain unmeasured; the proposal is not claimed to solve a18000-second timeout by itself.

## Relation to the retained January1994 diagnostic

The existing31-day record has equal worlds/headlines on every day. Its comparison clocks total 16.193143s versus 2.880086s ordinary native tick clocks and 2.962536s observed tick clocks. Each pretty world is220.2–221.4MB. This supports investigating harness work, but those comparison clocks belong to a different diagnostic: two native pretty world serializations every day, not S25's complete campaign archives at its less frequent checkpoint schedule. They do not measure the two eager FNV passes identified here.

The earlier diagnostic report-writer hypothesis does not transfer: S25 already uses `to_vec_pretty` plus `fs::write` (lines223–229). Monthly `storage::write/read`, prior-backup validation, native validators and terminal pre/post-load checks have real evidence purposes. This review proposes no bypass of those production paths. Duplicate boundary labels are retained even when dates coincide.

## Equivalence test plan

1. A focused lazy-evaluation guard must demonstrate zero diagnostic hash evaluations for equal byte buffers and exactly two on mismatch, while the old/new mismatch text and returned canonical bytes match. Merely passing existing equality tests would not prove the redundant work is gone.
2. Reuse the existing archive tests plus old-function oracle to detect same-length world/finance/forces/ammunition/history/log/journey drift, preserve malformed timestamp rejection and allow only the terminal wall-clock difference. Keep exact byte equality even if any digest happens to match.
3. Ensure both native validations still run and recorded canonical length/FNV remain unchanged. Use a fresh paired pilot under immutable old/new builds for complete checkpoint/counter/action and canonical-final comparison, followed by the strict retained-evidence verifier. Root owns any execution.
4. Measure the identified phase before predicting full-matrix completion. Retain all23 failed cells and the separate missing India1990 artifact state; no old cell may substitute for a new candidate run.

`review.json` contains the structured proposal and static schedule counts. `source-pins.json` pins source and original evidence; `source-excerpts.md` retains the relevant code. `manifest.json` hashes all payload files. No production game mechanism, saved data, seed, duration, daily invariant, equality scope, validation frequency or matrix acceptance rule is changed by this review.
