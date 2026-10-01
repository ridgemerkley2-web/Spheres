# Resource assertion passes; quiet timing and A1 remain open

The pending **0.15 ms/month resource test passes** on a fresh isolated release
library build. The printed total is **0.0623 ms/month**: resources 0.0360,
buy pass 0.0093 and appetite term 0.0170. The test's aspirational 0.05 budget
was **not** met; its warning remains in the [unaltered log](resource-timing.log).
This verifies the existing assertion from the
[accepted political-repair checkpoint](../political-repairs-20260930/README.md).
**An uncontended timing measurement remains pending:** app Git scans persisted
through the bounded confirmation window, so the confirmation was not run.
This does not complete A1, S27 or CP1, or replace any campaign qualification.

## Exact build and test

Source is integration `a81d2486`; its full `spheres-sim` tree, Cargo manifest
and lockfile are unchanged from accepted repair `7c6f112c`. The previous final
library executable was not included in the original frozen-binary manifest,
so its filename was not treated as sufficient build provenance. A new private
target directory was used with Rust/Cargo 1.98.0, `x86_64-pc-windows-gnu`.

- First build stopped before compilation because the sparse checkout omitted
  `spheres-web/src/bin/mapgen.rs`, which Cargo needs to parse its workspace.
  [Failed build](build.log) and [receipt](build-result.json) remain intact.
- Restoring the exact unchanged `spheres-web/src` directory allowed the
  [second build](build-02/build.log) to pass in 201.86 seconds. No source fix
  was made. [Build receipt](build-02/build-result.json) binds its executable.
- The existing exact timing command ran once, with one test passed, zero
  failed and 1,118 filtered. It ran for 9.02 seconds including Cargo startup;
  Cargo reported a fresh target without compiling. The test's own three
  1,200-month passes are unchanged. [Timing receipt](resource-timing-result.json).

The executable SHA-256 was unchanged before and after the measurement:
`988e8062ef5b680fd489f50c77797ee729e90b9e8b6f462ab525e16b9e3b95bf`.
The private target is recorded in both receipts. Other agents and the root
paused heavy work; no compiler or competing native test was present in the
[CPU preflight](timing-environment.json). It was not a perfectly idle desktop:
Git used 3 CPU-seconds and ChatGPT 1 CPU-second over the 3.13-second preflight,
on 16 logical processors. No user services were stopped.

The root requested one additional confirmation to resolve this measurement
condition, not to select a better reading. Read-only process inspection found
ongoing Git untracked-file/status scans launched by the app (parent PID 11608);
the original measured Git PID had already exited. After a preliminary
35-second wait, a single [bounded confirmation wrapper](confirm_quiet.ps1)
waited up to 45 seconds without tool-side polling. The scans persisted:
[confirmation-window.json](confirmation-window.json) records that **no second
timing test ran**. Other agents then resumed. The standing test is green under
the disclosed background activity; perfectly idle or uncontended qualification
is not claimed. Complete that condition when the app scan has finished,
preserving both this result and the new measurement.

```powershell
$env:CARGO_TARGET_DIR='D:/spheres-offload/codex-next-20260928/cp1-a1-followup-20261001-target'
cargo test --locked --release -p spheres-sim --lib --no-run -j 2
cargo test --locked --release -p spheres-sim --lib tests::the_resource_pass_stays_under_budget -- --exact --nocapture --test-threads=1
```

## Bounded residual diagnosis

[Offline analysis](existing-analysis.json) reproduces the existing accepted
repair's twelve-seed result: 7.5 median coups and **4/7 median top-three share**,
which still fails the unchanged strict `< 0.50` gate. Only one seed is below
one half; three equal it. Two seeds have five or six coups, where three leading
countries necessarily account for at least one half. This arithmetic is a
diagnosis of the current result, not a proposal to change the gate.

The script separately rechecks both compressed and decoded hashes of the
retained **pre-repair** seed-0 trace, all 185,260 stage observations and ten
actual firings. That observer is source `c41376f5`, not the repaired runtime.
Pakistan and Thailand reach 252 electoral Army ticks but neither hostile-Army
nor crisis conditions; Haiti and Nigeria never enter that branch. Those
observations identify different missing conditions, not four prescribed
historical outcomes. They do not establish a new implementation defect.

No coefficients, cohorts, thresholds, country data or runtime are changed.
No new campaign, reserved seed or native outcome sweep was run. The rejected
urgent-response policy and previous confidence/funding trials remain rejected.
The [mechanism proposal](mechanism-proposal.md) specifies an observable refused
civilian order, its command receipt and institution, resolution options and a
red-regression plan. Its missing compliance/escalation design is explicit;
it is not an installed trigger or a claim that A1 would pass.

Run `python verify.py` from this packet to check its manifest, local links,
the timing result, source identity and bounded-analysis counts. The
[analysis script](analyze_existing.py) can reproduce the derived output when
the separately retained local trace is available. The full trace and test
executable remain local, hash-pinned evidence; compact results are in Git.
