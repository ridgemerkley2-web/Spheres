# Verify SPHERES: bounded remote CI review

Reviewed by Codex subagent `/root/s20_preflight` on 2026-09-28. Final job snapshot: **2026-09-28T20:03:09Z**. This is a point-in-time review of actual GitHub Actions metadata and decoded failed-job logs, not a new test run or a campaign qualification.

| Exact revision | Ordinary workflow run | Observed jobs |
| --- | --- | --- |
| `4a3d0572361f0fbaf16ffd296b106dece4f40dcf` | [36475526246](https://github.com/ridgemerkley2-web/Spheres/actions/runs/36475526246), integration branch | 1 failed, 2 running, 7 queued; no overall passing claim |
| `5d970f6d7370baf16760585c641d81253d1c2175` | [36474010935](https://github.com/ridgemerkley2-web/Spheres/actions/runs/36474010935), S25 run branch | 6 passed, 3 failed, 1 running |
| `5d970f6d7370baf16760585c641d81253d1c2175` | [36474011627](https://github.com/ridgemerkley2-web/Spheres/actions/runs/36474011627), integration branch | 5 passed, 3 failed, 2 running |

## Actionable findings

1. **Known A1 implementation/calibration failure remains open.** All four completed political-calibration jobs on `5d970f6d`, and the completed Linux political job on `4a3d0572`, report the same 10 passed / 1 failed result: median elected-government coups 7, median top-three-country share **0.59**, failing the unchanged `< 0.5` concentration assertion. This is an actual political outcome failure, not a runner resource failure. The other nine outcome gates and the additional attribution control passed in those jobs. No thresholds, acceptance tests or cohorts were changed here. Failed job IDs: `109103066103`, `109103066303`, `109103068320`, `109103068360`, `109108145092`.

2. **Windows-only stability test path-alias assertion needs a bounded test correction.** Both Windows JavaScript jobs on `5d970f6d` pass the UI unit suite and earlier checks, then fail `RetainedVerificationTests.test_relocated_bundle_keeps_original_provenance_and_validates_locally` at `tools/campaign/test_stability_matrix.py:499`. The verifier returns the canonical original path `C:/Users/runneradmin/AppData/Local/Temp/.../run`; the test expects the temporary-directory alias `C:/Users/RUNNER~1/AppData/Local/Temp/.../run`. Each Python run reports 60 tests / 1 failure. `run_matrix` deliberately resolves the output path through `plain_path`, and the frozen-plan file pin records that resolved path. `logical_path` only replaces separators. The expected assertion should therefore use the resolved original output path, or its frozen provenance, while retaining the separate relocated-source and passing-integrity assertions. Do not weaken production path confinement or normalize stored provenance against the receiving machine. Failed jobs: [109103066211](https://github.com/ridgemerkley2-web/Spheres/actions/runs/36474010935/job/109103066211), [109103068438](https://github.com/ridgemerkley2-web/Spheres/actions/runs/36474011627/job/109103068438). This is a test-harness portability defect exposed by Windows 8.3 aliases; no archive corruption or native gameplay mismatch appears in these failures.

The workflow, stability runner/test and political acceptance source files are byte-identical between the two requested revisions (retained Git blob and SHA-256 pins in `source-pins.json`). Thus these observed failure mechanisms predate the documentation/research changes in `4a3d0572`; Windows HEAD jobs were still queued at the snapshot, so their outcome is not inferred.

## Passing and unfinished scope

For both preceding runs, the Linux JavaScript job, both static town-browser jobs and both extracted-package jobs passed. In run `36474010935`, Linux native also completed successfully, including the ordinary workspace suite, isolated resource timing check and native browser step. Other native jobs remained running. No completed failure log reviewed here reports an out-of-memory, disk, timing-budget, compiler, map-selection, archive-byte-equality or packaging error. This does not establish the outcome of unfinished jobs or claim the complete workflow passed.

The separate full-campaign stability run `36474011141` appeared in commit discovery and is deliberately outside this ordinary Verify SPHERES review.

## Evidence and limitations

- `*-runs.json`, `run-*.json` and `jobs-*.json`: original public GitHub API response bytes. Corresponding request files retain exact URL, UTC retrieval time, response digest and byte count. `jobs-*-final.json` is the final snapshot; earlier snapshots remain intact.
- `job-*.connector.json`: complete GitHub connector responses for all seven failed jobs observed at the final snapshot. The connector provides decoded log text, not the original GitHub downloadable log archive. `.connector.decoded.log` preserves that returned text encoded as UTF-8; `.connector.failure-excerpt.txt` contains labeled excerpts for convenience.
- `source-*`: exact requested-commit Git object bytes used for the diagnosis; `source-pins.json` binds revision, path, Git blob and SHA-256.
- `review.json` and `artifact-ledger.json`: machine-readable scope, findings and SHA-256 inventory. No local code changes, Cargo execution, Git mutation, workflow dispatch/cancellation or campaign execution occurred during this review.

Queued/running work may change after the recorded snapshot. This packet makes no automatic completion or campaign certification claim.
