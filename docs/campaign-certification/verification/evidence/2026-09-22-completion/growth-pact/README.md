# Mature growth and pact integration diagnostics

Date: 2026-09-22. Both iteration-2 failures reproduced unchanged against the
iteration-3 compiled test executable (`iteration3-mature.log`,
`iteration3-pact.log`). This is a diagnosis and test-contract
repair, not a change to simulation growth or alliance probabilities.

The first requested iteration-2 rlib had been replaced by iteration 3 before
the diagnostic probe linked. `rlib-provenance.txt` identifies the binary actually
used. Diagnostic probes were compiled directly with rustc against that rlib,
without Cargo or source changes. Their compact derived measurements are in
`diagnostic-measurements.json`. Full probe source and repeated trajectories are
retained in the local session evidence folder
`work/campaign-certification/evidence/politics-growth-pact-diagnosis`, omitted
from this checked-in packet to avoid duplicating large trajectory logs.

## Mature growth

The old test read `growth_last` after 360 months for seeds 1990, 7 and 42 and the
USA, Germany, France and Italy, requiring 0.8–2.6% annual growth. Its stated
purpose was to catch structural technology productivity being counted twice.

In seed 1990 Germany experienced 17 months at war and was still recovering at
the endpoint. It had 1.1181404% potential growth, minus 0.1983685 percentage points
of weak demand, 0.0202332 points of oil exposure and 0.1504893 points of sanctions:
0.7490494% before noise. Its lagging smoothed reported growth was only 0.1795659%.
France had 13 war months and ended at 0.5278489% reported growth. Italy ended at
0.7941203% due to cyclical conditions. All 12 structural potentials were between
0.9240896% and 1.4896843%, within the unchanged original band.

The repaired test retains the same seeds, countries, horizon and numerical band,
and applies the band to structural potential. This deliberately changes the
quantity under test; it does not claim the old realised-growth test now passes.

Changing only that reading would be inadequate. An alternate calculation that
removed the technology benchmark subtraction at the endpoint added 0.763–0.774
percentage points, yet remained inside the broad band. Later frontier convergence
can mask that defect. The test therefore also checks a causal invariant through
the real `tech::tick`: isolate each actual mature country's existing 128–129
held technologies as the sole-country benchmark, clear unfinished research and
absorption, and price that unchanged common stock into the current trend. Without
new knowledge or a changed relative position, revaluing it must not create a TFP
windfall. The test checks that no technology was acquired, the benchmark is
nonzero, and the trend remains unchanged to 1e-12. It does not reproduce the TFP
formula. All 12 counterfactuals passed the original compiled implementation.

The actual missing-subtraction source mutation and exact-byte restoration are
recorded separately in `missing-reference-mutation.log` and `mutation-result.json`.
The faulty source compiled successfully; the test failed on its causal guard:
seed-1990 U.S. TFP changed from 0.012576600972939528 to 0.02129104441075399 without
new knowledge. The expected test exit was 101. The restored source SHA-256 is
`d9c74f5d85b4752e4dc2f586f8164b8d588d91872d6399388c152602c30eb8b9`, identical
to the before-mutation bytes. The final ordinary build is the parent task's
responsibility after restoration; the test executable immediately after this
experiment intentionally still contains the mutant until rebuilt.

Mutation command (from the integration checkout, `CARGO_INCREMENTAL=0`):
`cargo test -p spheres-sim --release --lib --target-dir ../integration-target tests::mature_economies_do_not_run_hot -- --exact --nocapture`.
Only `tech_tfp - reference` was changed to `tech_tfp` inside the real technology
trend assignment. Exact original bytes were restored in a `finally` block and
verified by hash, including if build/test execution failed.

## Great-power defensive pacts

The original 12 seeds and 360-month horizon produced 25 honours and 22 refusals
across all countries. Great powers received only five calls, in four campaigns:

| Seed | Guarantor / defended country | Outcome | Pre-month reputation | Fatigue | Approximate honour probability |
|---|---|---|---:|---:|---:|
| 1 | UK / Kuwait | Honour | 70.00 | 0 | 0.8800 |
| 2 | USSR / Mozambique | Honour | 70.00 | 0 | 0.8800 |
| 2 | China / North Korea | Refusal | 2.25 | 0 | 0.4840 |
| 7 | UK / Kuwait | Refusal | 63.80 | 0.06794 | 0.7748 |
| 11 | UK / Kuwait | Refusal | 59.20 | 0 | 0.7442 |

Probabilities are diagnostic approximations from the state immediately before
the month; within-month recovery/order can change them slightly. They sum to
about 3.76 expected honours across the five observed calls. They are not an
independently sampled future confidence guarantee. Two actual honours are a
possible result of the configured contingent rolls.

The low reputations had real recorded causes. China repeatedly had separatist
arms operations exposed in Kyrgyzstan and paid for de-escalations. The UK in
seed 7 had defended Qatar, lost reputation on a denied basing request and stepped
back from a commitment; in seed 11 its Myanmar separatist-arms operation had been
exposed. No duplicate reputation charge, incorrect nuclear penalty, impossible
force ratio or invalid guaranteed-defender call was found. The UK–Kuwait initial
pact and the later USSR–Mozambique / China–North Korea pacts were present.

The old integration requirement of honours in at least 3 of 12 campaigns
**failed, with 2/12**, and is explicitly superseded. It conflated whether
endogenous play creates an obligation with whether a small number of loyalty
rolls succeed. The renamed integration test now requires actual answered calls
in 3..12 campaigns, requires both honour and refusal outcomes, checks that each
honouring great power actually appears on its ally's defending side in a war it
did not initiate, and checks that every refused pact was dissolved. It measures
four answered campaigns, two honours and three refusals on the diagnosed build.

The older `spheres-sim/tests/sample_size_audit.rs` had already identified the
original 3-of-12 honour quota as statistically underpowered for its false-red
goal (2.67% at the then-measured 48% campaign incidence). That historical audit
remains evidence about the superseded quota. The replacement integration count
is a coverage requirement, not a claim to inherit that probability calibration.

The adjacent independent 40-invasion test is unchanged: every call must be
answered, honours must outnumber refusals by more than two to one, and at least
one guarantee must fail. This retains the original controlled loyalty-rate
requirement; it is not replaced by the integration coverage count.

The local `current-test-probe.rs` was extracted from the three final test/helper bodies
and linked directly against the iteration-3 rlib. All three tests passed. The
normal compiled repository suite must still be rerun by the parent after the
faulty-build experiment is restored. No production economy, statecraft, AI or
random-seed behavior was changed by these two repairs.

## Restored ordinary build: focused verification

After the parent rebuilt iteration 4 from restored source, all three real
repository tests passed: mature growth (1.48s), actual defensive obligations
(5.69s), and the unchanged 40-invasion loyalty test (0.10s). Each was invoked
on `spheres_sim-de8a2e7790f15db6.exe` with its exact `tests::` name,
`--exact --nocapture`. `iteration4-focused-result.json` records executable and
source hashes and links the three green logs. This focused result is 3 passed,
0 failed; it does not replace the parent's full-suite validation.

## Full-suite follow-up: term-limit fixture

The iteration-4 full simulation unit run completed in 168.45s with 921 passed,
5 failed, 25 ignored diagnostics and the separately scheduled resource timing
test filtered. Four failures were old fingerprint assertions in the compiled
executable; the parent had independently measured and updated those source pins.
The one additional failure was the term-limit subsection of
`every_succession_rule_seats_the_right_description_and_never_a_name`.

The old subsection advanced seven years and interpreted the first generic U.S.
leadership headline as a constitutional expiry. Replaying its exact seed instead
shows George H. W. Bush dying in the first simulated month, reported on landing
at February 1990. The resulting Republican government description is a valid
death succession, not a failed term limit. It should not be changed to make the
historical incumbent survive a stochastic campaign.

The repaired subsection retains the source's exact `1997-01-20` deadline
assertion. It calls the real government tick in December 1996 and January 1997,
with the election scheduled outside that short window. It verifies no early
expiry; exactly one deadline-month seating; the current party and office;
removal of the old name and deadline; and no repeated expiry in the same month
or February. Other succession and 40-year coverage in the containing test are
unchanged. `term-limit-boundary-probe.log` records the extracted subsection
passing against the real iteration-4 simulation and the original death event.
The original failure is preserved separately. This standalone subsection pass
does not replace the subsequent recompiled repository test and full-suite run.

The subsequent real iteration-5 repository run confirms the entire succession
test now passes, including the repaired deadline subsection and its unchanged
40-year coverage. Mature growth, real pact obligations and the unchanged
40-invasion loyalty test also pass. The full run was 927 passed, 5 failed,
25 ignored and 1 resource-timing test filtered in 170.11s. Remaining failures
were four stale fingerprints and a separate armed-security preflight test;
those belong to the parent and the other simulation workstream.
`iteration5-followup-result.json` and the compact excerpt retain the proof and
full-log hash. Iteration 5 contains a subsequent political-memory production
change and its default timeline differs from iteration 4; the older receipts
are not presented as final-source certification.
