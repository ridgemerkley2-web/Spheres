# S22 routing and stock validation records

**Completed correctness checks; qualification: false.** Original logs and
provenance are preserved byte-for-byte. These runs overlapped other task work;
their durations are not performance acceptance measurements. No executable or
duplicate campaign input is included.

At `8de6f2a0cc1f64932c28755539054fd7c852f3ca`, release workspace compilation
(`cargo test --no-run`) and the web server build completed. Four focused filters
then produced 10 passing entries, representing **9 unique ordinary tests**, with
zero failures. The standalone supply-graph handoff test also appears in the
broader graph filter and is counted once. Three ignored oracle entries in those
filters are not counted as executed tests.

All **six** explicitly run actual-checkpoint oracles passed at that revision:
sovereignty candidate filtering, contract nominal routing, and campaign supply
graph handoff each matched the original path over 31 complete native days from
both 2015 and end-2035. Full daily world/headline comparisons retained unchanged
input hashes. Contract cases exercised **1,209 / 1,395 nonzero dispatches**.

At `992bc99ab5529332cb7ba860f53ff0801a3283b3`, both actual-checkpoint
empty-import oracles passed: 31 complete daily world/headline comparisons each,
with **155 / 159 empty equipment catalogues skipped**, respectively. Input bytes
remained unchanged. The independent review in `reviews/` applies to the exact
implementation commit it names and is explicitly source-only; it makes no
execution or timing claim.

The now-completed `992bc99a` release workspace regression passed **1,933 tests,
0 failures, 100 ignored across 66 suites**, exit zero at
`2026-09-28T07:59:51.6768897Z`. Counts were independently summed from the retained
raw log. This is the command in its execution provenance:

```text
cargo test --locked --release --workspace -- --test-threads=2
```

The immutable runtime provenance snapshots predate that regression's completion;
their original wording is retained. The separate completed execution provenance
and log establish the later result. Runtime binary hashes are recorded, not
recomputed by this packet. Oracle execution provenance matches the corresponding
recorded simulation binary hashes.

Both actual input hashes match the previously archived
[late-input packet](../late-input-progress/README.md) and its referenced 2015
lineage: `e786ffb67a26c6ae28bd917d5d8dc26f857e441dd6a58d60adc1a1107937ca11`
and `67aadca2f55280abc0e4ad97944a654ec7d173ffb65cb7d71dd59090e10503cb`.
The manifest identifies every original artifact path and capture hash. The
earlier isolated latency failures remain valid failed attempts. Fresh isolated
measurements and two complete qualification rounds are still required; S22,
G5 and CP1 remain unearned.
