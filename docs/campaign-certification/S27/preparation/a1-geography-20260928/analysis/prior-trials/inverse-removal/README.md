# A1 political calibration: failed baseline and rejected trial 01

`CODEX-S27-A1-01` remains **blocked and incomplete**. Source baseline:
`30410e55217c7461937ad42564678c59d34f53b1`. The 28 September 2026 attempt
preserves the existing A1-A10 outcome definitions, cohorts and thresholds.
No holdout was run; no political calibration, S27, G5 or CP1 closure is earned.

## Actual results

| Run | Outcome gates | Separate attribution control | A1 median elected coups | Displayed median top-three share |
|---|---|---|---|---|
| Fresh baseline | 9/10 pass; A1 fails | Pass | 7 | 0.59 |
| Trial 01 | 9/10 pass; A1 fails | Pass | 12 | 0.67 |

Both complete runs report **10 passed, 1 failed, 1 filtered**: ten historical
outcome gates plus one attribution regression ran; the measurement-only census
was filtered. A1 requires median coups in **4..14 inclusive** and median
per-seed top-three share **strictly below 0.50** over seeds 0..11 and 252 months.
Shares above are the two-decimal log displays, not higher-precision measurements.
The trial worsened the failed concentration reading while retaining the count
band. Logs retain all eleven outcomes and the original failing exit.

The [prospective plan](evidence/trial-01-plan.md) declared a single AI policy
revision: remove the civilian-confidence penalty from the Army funding inverse,
funding material and war needs only. This was a policy experiment, not correction
of an accidental arithmetic bug. The [exact patch](evidence/trial-01-candidate.patch)
changed no historical source, cohort or outcome assertion. The candidate failed
its first acceptance stage and was rejected; no candidate full-regression,
development N200 or reserved-seed validation is claimed.

## Restoration and evidence

The [result](evidence/trial-01-result.json) and [source receipt](evidence/trial-01-source.json)
pin the candidate, restored government source and unchanged acceptance test.
During packaging, the complete restored government bytes were compared with the
captured pre-trial body, and the test hash was checked against the result. The
redundant 848 KB pre-trial source body is omitted; the baseline Git revision and
raw source hash identify it. [manifest.json](manifest.json) hashes every retained
original artifact. Local attributes preserve captured bytes across checkouts.

**Executable boundary:** at rejection, restoring source did not rebuild the
candidate executable. Its recorded SHA identifies the rejected trial, not a
restored-source binary. Later tests or preflights must rebuild and record their
own source/executable identity; this packet does not claim that rebuild occurred.

## Next work without an unearned pass

The existing country-count and seed-0 diagnostics distinguish limited geographic
spread, repeat events, fiscal constraints and loyalty recovery. They demonstrate
no further concrete runtime defect requiring correction. In particular, removing
repeats from five or six distinct coup countries cannot by itself satisfy the
strict top-three share rule. Pooled shares and inverse-HHI are different measures
and are not substitutions for A1. The
[earlier calibration history](../../../../political-arm/2026-09-22-calibration-repairs.md)
retains rejected confidence trials and their Algeria/A2 regressions.

A1 therefore awaits a justified causal hypothesis or an explicitly reviewed game
design decision. No new coefficient search, manufactured event quota, weakened
assertion or use of reserved seeds is implied. The user's ordered work continues
with the independent startup engineering preflight after this recorded attempt;
A1 remains failed and excluded from any claim of aggregate CI or campaign
qualification. All canonical session dependencies remain unchanged.
