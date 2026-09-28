# S22 latency follow-up and repair validation

**Progress evidence only; qualification: false.** No authoritative pair has
started. S22, G5 and CP1 remain unearned. All copied artifacts retain original
bytes, source paths and SHA-256 hashes in `manifest.json`; no raw campaign input
or executable is duplicated here.

## Latest isolated preflights at 992bc99a

The full release regression at this candidate passed 1,933 tests, as retained in
the [routing validation packet](../routing-validation-progress/README.md).
The subsequent isolated native preflights both completed their functional
workload but failed unchanged latency limits:

| Actual input | Simulation/history p95 | Whole-turn p95 | Whole-turn maximum | Sampled private / observed OS peak bytes |
| --- | ---: | ---: | ---: | ---: |
| 2015-01-01 | 276.1008 ms | 387.2577 ms | **790.7846 ms** | 850,944,000 / 843,931,648 |
| 2035-11-30 | **383.565 ms** | **507.0357 ms** | 641.5211 ms | 902,459,392 / 889,171,968 |

Both observed memory checks pass 1 GiB. The 2015 maximum exceeds 750 ms; the
2035 simulation/history and whole-turn p95 exceed 300/400 ms. Both original
runner results remain `passed:false` and `numerical_acceptance:false`. Raw
31-day profiles, memory CSVs, invocation and console/stdout/stderr are included.

Two concurrent diagnostic runs subsequently matched all 31 complete native
worlds and returned headlines against the ordinary engine, with unchanged
actual inputs. Their ending world fingerprints are `5c486d0c186f921e` (2015)
and `3c3f21d9d67bd7b8` (2035). Diagnostic timers overlap and their nested detail
stages must not be added to their parents. These exactness checks and contended
stage timings are not latency qualification or memory measurements.

## Original combined repair attempt at 2774b0c5

Release compilation succeeded. The immutable runtime provenance pins simulation
test SHA-256 `04e657399575687379a89586a068c23f6048bc4819c1419c79fd4bccd189a231`.
Terminal adjacency and dependency-guard focused checks passed. The mine group
had two passes, one failure and one ignored actual oracle: full-review equality
held, but the synthetic fixture failed to exercise both successful and refused
commands. Zero standing cannot refuse a daily mine command with zero price.

The workspace invocation stopped with exit 101: the CLI suite passed one test,
then the simulation library recorded 1,042 passes, one failure and 33 ignored.
Thus the captured aggregate is **1,043 passes, one failure, 33 ignored across
two suites**, not a completed full-workspace regression. Its failure is the same
synthetic mine-coverage assertion; the original log is preserved.

Both explicitly executed dependency oracles passed 31 complete native days,
skipping 62/74 unrelated pairs for actual 2015/2035 and retaining unchanged
source hashes. The actual-2015 mine oracle passed. Actual 2035 matched every
daily world/headline but then failed the required nonzero retained-forecast
reuse assertion. A failed coverage assertion remains a failed test.

Test-only follow-ups distinguish the two implemented optimization paths:
retaining an already-read forecast, and deferring an eager forecast entirely
after a completed stock-sufficient scan. They also add a genuinely priced
legacy refusal fixture. The source reviews disclose both corrections and their
scope. Original failed logs remain unchanged.

## Corrected checks at 6ff13350

Candidate `6ff13350e8096d67c955b9710e983281aebc3eae` compiled successfully.
The corrected mine group reports **three passes, zero failures, one ignored**.
Its log and retrospective execution sidecar preserve the actual command, exit
status and hashes; the sidecar does not invent start/finish timestamps.

All **four** explicitly executed actual-2015/2035 oracles pass with source and
binary hashes retained. Mine cases compare 31 complete daily worlds/headlines:
2015 reuses one forecast and defers 114 eager forecasts; 2035 reuses zero and
defers 129. Dependency cases compare the same full-day outcomes and skip 62/74
unrelated pairs. The simulation executable recorded by these runs has SHA-256
`917e77da41abea7b46f7da77cad58cb47c8910bf2a5e2be330f37f35c2bec1c5`.
The full corrected release workspace regression completed at
`2026-09-28T08:50:40.9423135Z`, exit zero: **1,938 passed, zero failures,
102 ignored across 66 suites**. Counts are independently summed from the
retained raw log. These results do not retroactively pass the original coverage
failures.

Subsequent isolated preflights use the unchanged immutable candidate:

| Actual input | Simulation/history p95 | Whole-turn p95 | Whole-turn maximum | Sampled private / observed OS peak bytes | Result |
| --- | ---: | ---: | ---: | ---: | --- |
| 2015-01-01 | 274.4605 ms | 368.194 ms | 522.5872 ms | 854,024,192 / 825,348,096 | Pass |
| 2035-11-30 | **334.0103 ms** | **463.9538 ms** | 494.237 ms | 915,517,440 / 910,614,528 | Fail: both p95 limits |

The 2015 preflight clears all native limits. The 2035 maximum and both memory
limits pass, but its simulation/history and whole-turn p95 exceed 300/400 ms.
Final world fingerprints equal the earlier exact diagnostic results. Raw
profiles, runner decisions, memory CSVs and invocation/console records are
retained for both outcomes. These are declared preflights, not retroactively
qualified cells; the complete two-round qualification remains outstanding.

## Recoverable old preflight copies

`dedup/` preserves both maintenance attempts. The first helper verified 37
eligible copies but a tally error caused **zero removals**. The corrected
attempt removed exactly 37 redundant raw copies from explicitly named,
completed, nonqualifying preflight directories only after checking their
recovery sources. It recovered 2,753,814,871 bytes; free space at that operation's
end was 10,830,450,688 bytes. This historical value is not current free space.

The restore manifest records each original path, byte count, hash, timestamps,
completed receipt, and retained canonical/gzip recovery source. To recover an
entry, first verify its recovery source hash and size; require the original
target to be absent; copy or decompress according to `restore.method` using
create-new semantics; then verify the result against the recorded original size
and SHA-256 before restoring timestamps. Keep the recovery source intact.
This packet did not execute restoration or repeat decompression. The author’s
verification, per-file completion journal and both helper versions are retained.

Canonical 1999/2006/2015/end-2035 inputs and resumes, binaries, original reports,
traces and existing gzip files remain intact. No frozen output or future
qualification input was removed. These records are separate from the earlier
[annual checkpoint compression](../checkpoint-storage-progress/README.md).
