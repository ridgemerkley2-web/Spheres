# Rejected confidence reassessment 27

Candidate 27 changed only `ARMY_CRISIS_CONFIDENCE_WEIGHT` from 0.65 to 1.20 on runtime checkpoint `9f7faed5f541e14545e01caf1616ba08c6ad8ff9`; its evidence-only parent revision was `12278ea4c66863200fd10948350ae69391954cfa`. The single change is preserved in `iteration-27-candidate.patch`. The candidate was rejected and the coefficient restored to 0.65. It is not an accepted model or a certification result.

## Unchanged tests and result

- Sim library: **1,004 passed, 1 failed, 25 ignored, 1 filtered**. The existing Algeria annulment chronology failed after an earlier elected-government coup. No fixture or assertion was changed to admit the candidate.
- Ordinary bloc integration target: **5 passed, 0 failed, 7 ignored**.
- Explicit original A1–A10 and attribution suite: **9 passed, 2 failed, 0 ignored, 1 filtered**. A1 recorded median 12 electoral coups but median top-three concentration 0.50, which fails the strict `< 0.5` requirement. A2 recorded Islamist takeovers by 2000 in 6/12 seeds, which fails the unchanged `> 6 && <= 10` requirement. The eight other outcome gates and the attribution regression passed.
- The result records no N200 development cohort and no independent holdout run. The candidate already violated the predeclared correctness stop rule; the original outcome suite was completed for diagnosis. There was no further parameter search in this trial.

The raw logs, prospective plan, precision review, application receipt, build receipt and final result retain their original bytes. The independent review clarifies two statements in the plan without changing implementation: a target exactly at 0.35 is not below the strict loyalty line, and pending-first-ballot protection belongs to coup eligibility rather than an unconditional zero raw confidence penalty.

## Provenance and restoration boundary

The parent result records all **1,932** baseline native inputs restored and a clean worktree at `2026-09-22T23:14:05.947771+00:00`. The independent audit verified the restored government bytes against both the preserved external baseline and the parent revision's Git blob. Replacing the one coefficient declaration reconstructs the candidate SHA exactly. The large duplicate baseline government body and executable files are not copied here; the Git revision, one-line patch, input manifest and binary identities identify them.

The independent audit happened after a separately authorized three-line empty-stall optimization in `spheres-sim/src/dyads.rs`. At that later audit, **1,931/1,932** current inputs still matched the baseline raw hashes, with only that recorded later edit differing. The original dyads Git blob, reconstructed using its existing CRLF checkout convention, matches the baseline manifest. This distinction preserves the recorded clean restoration without claiming the later working tree was clean. The audit did not write runtime source or execute candidate binaries.

Candidate binary hashes are the contemporaneous build receipt's recorded identities. Those mutable paths were subsequently used for a restored-baseline rebuild; this archive does not claim to preserve or independently rehash the rejected executables after replacement. Baseline correctness and remaining A1 work are separate from this rejected trial.

`manifest.json` lists SHA-256 and byte sizes for every other archived file. `iteration-27-independent-verification.json` records the independent receipt/log/source checks and their limitations. The verification script is an external audit snapshot with machine-specific paths, not a runtime or automatic CI entrypoint.
