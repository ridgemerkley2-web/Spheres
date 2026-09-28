# CI follow-up and exact full-matrix freeze

This checkpoint corrects the Windows stability test oracle and retains actual
failed CI logs. It is not an overall CI, political-calibration or campaign pass.

## Windows alias failure and repair

The Windows runner uses `RUNNER~1` as its TEMP path. The production runner resolves
the recorded evidence origin to `runneradmin`; the old relocation test compared
that correct canonical origin with its unresolved input. Only the test expectation
now calls `out.resolve()`. No production path, archive comparison or provenance
check is relaxed. `change.patch` contains the exact change and the ordinary CI
expansion to include the distributed verifier's tests.

The local reproduction uses a real Windows 8.3 directory alias. The unchanged test
fails with the same alias-versus-resolved mismatch in `windows-alias-fix/before.log`;
the corrected test passes in `after.log`. The full tooling suite then ran 71 tests:
70 passed and the existing privilege-dependent symlink test skipped. This is
synthetic harness verification; no long campaign is represented by those fixtures.

## Actual GitHub failures

`ci-review/` retains seven failed job logs from the two preceding 5d970f6d Verify
runs and the 4a3d0572 run's political job, with original API snapshots, source
objects and hashes. The two failure classes are the fixed Windows test expectation
and the separately unchanged A1 concentration gate (median coups 7, displayed
top-three share .59). The review's timestamped job statuses remain historical;
incomplete native jobs are not counted as passes. See its README for job IDs and
which stages actually finished.

## Frozen full-campaign execution

The separate [full campaign run](https://github.com/ridgemerkley2-web/Spheres/actions/runs/36474011141)
uses candidate `5d970f6d7370baf16760585c641d81253d1c2175` and batch manifest SHA256
`70a51be54a7e3100a5267ca310bf5c7f05a3c388a389b7c54904e6365c477cc8`.
The build/freeze succeeded. The downloaded frozen payload was independently
verified against GitHub's ZIP digest, extracted through a strict five-entry
allowlist, and validated by the exact frozen helper. The Linux executable was
not run locally. The 24 actual campaign results remain pending at this checkpoint.

`distributed-freeze/` contains compact exact metadata and logs, not the 252 MB
frozen ZIP or its 350 MB executable. Those remain at
`D:/spheres-offload/codex-next-20260928/distributed-run-01/`. Complete cell archives
must be downloaded and independently aggregated using the frozen helper and
manifest before any full-matrix pass. These test-only follow-ups do not replace
the frozen candidate or silently mix the running batch with another revision.

`manifest.json` hashes every other file in this checkpoint. Captured bytes are
preserved in Git. This checkpoint changes no work-board task to complete and
does not close S25, S27, G5 or CP1.
