# S22 pair02 candidate validation — complete, qualification pending

**This packet does not qualify S22.** The completed [first pair](../qualification-pair-01/README.md)
remains failed at 17/18 cells. This new packet preserves the follow-up diagnoses,
reviewed repairs and all initial validation failures. Final validation candidate
`5d11dd6dae436eeca105dbaf0d02732cd768b35b` contains two test-only corrections
after runtime candidate `070141be78be39db0a66db09026c2887831acb8b`: the annual-plan
fixture at `a7356a98` and actual-input reuse applicability at `5d11dd6d`.

## Completed validation

| Scope | Candidate | Result |
| --- | --- | --- |
| Post-pair 2035 subsystem diagnosis | d50f7ee1 | 31 exact native worlds/headlines; source unchanged; diagnostic only |
| Post-pair 2035 room read diagnosis | d50f7ee1 | Single read-only sample; state unchanged; diagnostic only |
| Initial release/server compilation | 070141be | Completed; immutable binary metadata retained |
| Initial focused tests | 070141be | 8 passed, **1 failed**, 4 ignored |
| Initial workspace execution | 070141be | **Halted after library failure**: 1,067 passed, 1 failed, 43 ignored |
| Corrected release/server compilation | a7356a98 | Passed |
| Corrected focused tests | a7356a98 | 9 passed, 0 failed, 4 ignored |
| Actual-input oracle attempts | a7356a98 | 5 passed, **3 failed terminal coverage assertions** after all 31 world/headline comparisons matched |
| Corrected full workspace | a7356a98 | 1,963 passed, 0 failed, 112 ignored across 66 suites |
| Final release/server compilation and focused tests | 5d11dd6d | Build passed; 9 passed, 0 failed, 4 ignored |
| Final actual-input oracles | 5d11dd6d | All 8 passed; each compares 31 complete native days |
| Final full workspace | 5d11dd6d | 1,963 passed, 0 failed, 112 ignored across 66 suites |
| Fresh offline art audit | 5d11dd6d | All 245 configurations, 33-asset manifest and 13 GLB exports pass; 35 Node entries pass |
| Isolated 2015 and end-2035 native preflights | 5d11dd6d | Both pass every unchanged latency/memory limit |
| New complete 18-cell qualification pair | 5d11dd6d | Still required; not launched in this packet |

The focused and initial workspace failures are the same synthetic industry test:
`scoped_factory_pass_matches_original_bills_gdp_shortages_and_xp` panicked with
`program budget owns an annual plan`. Its helper installed department allocations
without the annual plan required by a later complete native day. The test-only
correction enacts those unchanged allocations through ordinary `SetProgramBudget`
and opens the department day. No runtime algorithm or assertion was weakened.
The failed logs remain exact and separate from corrected results.

The actual 2015 deployment oracle and both monthly-purchase oracles later failed
their terminal assertions requiring nonzero reuse. All 31 preceding complete
world/headline comparisons matched; this is inferred from the assertion order
and the recorded terminal panic, not a successful test verdict. Those three runs
remain **failed**. The 2035 deployment oracle passed with 1,120,154 repeated edge
reads, and both dated industry and arrival oracles passed. Input hashes before
and after all eight attempts matched. The follow-up below resolves the workload
assumption; no successful test or speed improvement is inferred from those failed runs.

Test-only follow-up `5d11dd6dae436eeca105dbaf0d02732cd768b35b` preserves every
world/headline/source comparison. Deployment requires positive reuse on the
reviewed 2035 case and reports the inactive 2015 case. The monthly test retains
positive synthetic coverage, requires real-save route-plan counts never to
increase, and requires `old_plans - new_plans == reused`. All eight final actual
oracles subsequently passed. Production is unchanged since `070141be`.

Source-only follow-up from Codex `/root/review_source05` found all nine actual
cover buyers selecting one-input Missile recipes (2015 Paveway; 2035 JDAM or
Paveway). That does not repeat seller/buyer pairs across military material lines,
unlike the synthetic two-input Trophy case. The civilian phase permits one
material intent, leaving only potential cross-phase reuse. No actual-checkpoint
saving is credited to this memo without positive executed counters. This test
scope clarification changes none of the qualification thresholds.

| Final oracle counters, 31 days each | 2015 | 2035 |
| --- | ---: | ---: |
| Repeated deployment edge reads avoided | 0 | 1,120,154 |
| Energy geometry/headroom reads reused | 15,248 | 19,458 |
| Owned arrival route-key vectors avoided | 113,506 | 96,914 |
| Arrival node-string copies avoided | 3,367,464 | 2,856,731 |
| Arrival string content bytes avoided | 28,604,415 | 24,365,628 |
| Repeated arrival-key hits | 105,868 | 90,247 |
| Purchase route searches, optimized / original | 200 / 200 | 234 / 234 |
| Purchase route results reused | 0 | 0 |

These are exercised work counters, not allocator measurements or measured speed
gains. The purchasing memo provides **zero demonstrated saving** on either
actual input. The eight comparisons ran concurrently with correctness regression;
their durations are not isolated qualification timing. Every comparison retained
all native-world/headline equality assertions and unchanged source hashes.

## Final isolated preflights and offline art

After correctness work finished, the two native preflights ran in an idle window
against the immutable final binary. Both passed the unchanged 300 ms simulation/
history p95, 400 ms whole-turn p95, 750 ms whole-turn maximum and both 1 GiB
observed memory limits. Values below are displayed to four decimal places;
original reports retain their full numeric values.

| Actual input | Simulation/history p95 ms | Whole-turn p95 ms | Whole-turn maximum ms | Sampled private / OS peak bytes |
| --- | ---: | ---: | ---: | ---: |
| 2015-01-01 | 258.6063 | 318.5443 | 683.2764 | 867,627,008 / 864,006,144 |
| 2035-11-30 | 282.2180 | 382.0407 | 392.1018 | 902,393,856 / 886,210,560 |

Ordinary and batch final facts match. Their final world fingerprints remain
`5c486d0c186f921e` and `3c3f21d9d67bd7b8`, respectively, matching the earlier
candidate. Original and copied input hashes match before and after. The memory
envelope covers the entire native process, including loading and report work;
sampled private bytes and the OS working-set high-water mark are distinct. These
are **preflights**, not cells from the new declared qualification pair.

The fresh art audit ran at the exact final candidate from 11:20:16.296 to
11:23:24.775 UTC, with a clean checkout before/after and all 39 pinned source
hashes unchanged within the run. All four commands passed: 245 graded
configurations (166 passes and 79 advisory density notes; zero current ceiling,
required-floor or export failures), the 33-asset manifest, all 13 byte-identical
canonical GLBs, and 35 focused Node test entries. The retained 33 historical
proposal overages remain diagnostic. This offline audit overlaps correctness
work and measures no live frame rate or driver VRAM. Its exact scripts, command
arguments, inventory, logs and results are retained under `art-5d11dd6d/`.

The new candidate is ready for a fresh frozen initial and confirmation pair.
Earlier passing browser cells or these preflights cannot be selected as its
replacement cells. Pair01 remains failed; S22, G5 and CP1 remain open.

The earlier standalone deployment Cargo log is retained separately. It is a
filtered run with many zero-match suites, not an additional full-workspace pass;
there is no separately captured binary provenance for that earlier command.

## Diagnostic and source-review scope

The d50 subsystem diagnosis starts from the unchanged actual 30 November 2035
input and compares 31 complete ordinary days with the instrumented schedule.
All worlds and returned/retained headlines match, ending at world fingerprint
`3c3f21d9d67bd7b8`. Nested detail timers overlap their parent timers and must not
be added together. These post-failure diagnostics are not isolated qualification
measurements and do not replace the failed confirmation cell.

The separate read-model record is one pure sample including serialization:
whole-state 49.8013 ms and equipment 70.3838 ms. It is not an acceptance result or
a multi-sample latency distribution.

Independent source review by Codex `/root/review_gap_submission` inspected:

- `fe15010be4e5379a34d0162cbbcd6a470ead9a41`: exact sea-factor-bit capacity reuse
  stays within an immutable deployment world/graph borrow and is dropped before
  mutable freight reservations; original ordering and arithmetic remain intact.
- `09c9c8ccf01cb0c78363fde3e0255f5ba4ace9d3`: only fixed energy geometry and
  inherited headroom are cached within the factory pass. XP-sensitive modifiers
  remain live, the two original capacity expressions stay distinct, and later
  Materials/public paths receive no retained scope.
- `2fb9227320b4985c28063955afe3def46a1d7833`: borrowed arrival keys preserve
  ID-only versus ID/name equivalence. Only disjoint due/hold fields change before
  dropping the keys; encounter order and stable cargo-ID sorting remain intact.
- `070141be78be39db0a66db09026c2887831acb8b`: monthly buyer waves reuse only
  ordered seller/buyer route booleans. Money/Commodity proposals cannot change
  route topology, sanctions, access, policy or living governments; supply, caps,
  pricing, refusal checks and decision evaluation are still read live.

No actionable source finding was identified. This independent reviewer ran no
build, test or benchmark; executed evidence is attributed to its captured
commands/provenance. The separate consolidated review and industry proposal are
retained unchanged under `reviews/`.

## Preservation

[manifest.json](manifest.json) gives the original absolute source path, byte
count and SHA-256 for all **93 retained original files (4,129,867 bytes)**.
Copies are byte-identical, with text
conversion disabled. Runtime bodies and large save copies are excluded; their
exact external pins remain in original provenance. Canonical 2015/end-2035 inputs
remain in the previously verified [2015](../stock-fix-progress/README.md) and
[2035](../late-input-progress/README.md) archives and were not modified here.

The root's independent pair01 archive-verification output is retained under
`archive/`. It verifies byte integrity/recoverability, not a successful replay
or qualification. No previous packet, frozen artifact or source save was replaced.
The initial and amended consolidated source-review notes are retained separately;
the later source file does not overwrite the captured earlier version.
