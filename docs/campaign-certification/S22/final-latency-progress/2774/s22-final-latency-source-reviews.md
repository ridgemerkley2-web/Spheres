# S22 final latency changes: independent source reviews

Recorded 2026-09-28. These are bounded source inspections, not executed test
results or performance qualification. Neither reviewer built the game, ran tests,
or measured speed for these reviews. S22 timing limits remain unchanged.

## Terminal adjacency lookup

- Author: Codex agent `/root/review_source05`.
- Reviewed commit: `6ad7908c8173823b687c57f2f745584b96e090b3`.
- Integration commit reported by root: `fbbc4e1e`.
- Independent reviewer: Codex agent `/root/s20_preflight`.
- Scope: `spheres-sim/src/logistics.rs`, including the exhaustive old-scan parity
  regression. No other production behavior was reviewed or certified here.
- Finding: no actionable source-level issue.

The immutable network initializer asserts unique node IDs, resolves both endpoints
of every edge, and adds each edge to both endpoint adjacency lists. Looking up a
node's adjacent terminal edges therefore preserves the old full-edge scan's
boolean result for districts, gateways, unknown or non-normalized IDs, and
self-edges. The change introduces no world reads or writes, route decisions,
floating-point operations, or mutable cache. The regression inspects index and
adjacency completeness and compares all authored nodes plus unknown and malformed
IDs with the literal previous scan. This review does not claim the test ran or
that the change achieved a particular speed improvement.

## Same-review mine forecast reuse

- Author: Codex agent `/root/s20_preflight`.
- Independent reviewer: Codex agent `/root/review_source05`.
- Reviewed source: pending worktree changes on base
  `81e408e03cb82e7233a2f866e7a16dd37c32eb93`, branch
  `codex/s22-mine-review-forecast`, in `s22-browser-qualification-guards`.
- Final commit: `28f2e455e827be178a1745d4bed4c54d10e9c1ef`. The author confirmed
  that only indentation of the two `Candidate` match-arm bodies changed after
  the reviewed snapshot; no source tokens or behavior changed.
- Scope: `spheres-sim/src/economic_ai.rs` and new
  `spheres-sim/src/economic_ai_mine_tests.rs`.
- Finding: no actionable source-level issue.

An owned forecast is retained only when the mine review proposes no command. It
passes directly through immutable reads into the ordinary record operation.
Proposing a mine discards that forecast before either successful or refused
command execution, so those paths keep their fresh post-attempt read. The
original `demand <= stock` and `run_gap <= 1e-9` comparisons, including their NaN
behavior, remain unchanged; the public forecast implementation is unchanged.
`RawSupplyContext` contains no interior mutable cache. Test source retains the
original eager selection and full-review read paths and covers accepted/refused
commands, budget-renewal boundaries, exact world/headline comparisons, and an
ignored actual-checkpoint 31-day oracle requiring nonzero reuse. These are
observations about test coverage in the source, not test execution claims.

### Test-only coverage correction

After root compiled the tests, the author reported that selection/renewal checks
passed and full-review bytes matched, but the accepted/refused coverage assertion
failed: daily mine construction deliberately costs zero political capital, so
zero standing did not create a refusal. Follow-up
`080a3b2d45a5558dd97d7d29deaeb941190cd29f` adds two explicitly legacy-priced
private-review cases while preserving every daily case. `/root/review_source05`
inspected this test-only diff and found no actionable issue: it asserts the native
command price, exact standing refusal/no mutation, actual full-review attempt,
and expected accepted/refused outcome. It explicitly does not claim the public
legacy scheduler enables economic AI. Production code is unchanged. This note
records the reported earlier failure honestly; it does not claim execution of
the corrected tests by either source reviewer.

The author subsequently reported exact 31-day world/headline parity for actual
2015 with one retained-forecast reuse, and exact 31-day parity for actual 2035
with zero retained reuse, which failed the original nonzero-reuse coverage gate.
Follow-up `c6112d10cfdf298b56181cabb944790f221e5783` separately counts completed
stock-sufficient scans that defer the formerly eager forecast entirely. The
counter excludes existing active-mine/uncleared-market early returns and exists
only under `cfg(test)`. The actual oracle still compares every complete day and
now requires nonzero retained reuse or nonzero deferred eager work, logging both;
the synthetic true-reuse assertion remains. `/root/review_source05` reviewed this
small diff without execution and found no actionable issue. This is an explicit
coverage correction for the two optimization paths, not a relaxed performance
limit or evidence of timing qualification.

Subsequent source changes require their own review or an explicit confirmation
that the reviewed source was preserved; an integration commit alone is not a
performance verdict.
