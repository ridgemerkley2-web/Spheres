# Measuring campaign performance

Use a release build and an otherwise idle machine. Long calibration panels, compilation and checkpoint preparation contend for CPU and memory; their timings are unsuitable as responsiveness benchmarks.

## Server measurements

`spheres-web/src/performance.rs` contains an ignored `performance::campaign_lifetime_profile` test. It uses the browser's actual daily rules and a human USA campaign. The preferred `INPUT` method compares the same aged idle campaign with a controlled busy copy: real construction is queued, constrained economic AI is enabled, and a new war is declared through ordinary priced commands immediately before timing.

Set `SPHERES_PROFILE_OUT` to a disposable JSON path and run:

```text
cargo test --locked --release -p spheres-web performance::campaign_lifetime_profile -- --ignored --exact --nocapture --test-threads=1
```

The legacy default run, without `INPUT`, still advances two uninterrupted worlds to ages 0, 10 and 30 and measures 31 consecutive one-day turns at each checkpoint. Its busy world enables AI and declares Iraq–Kuwait war in 1990; this older method cannot guarantee a busy world decades later. `SPHERES_PROFILE_YEARS` can restrict the last age. `SPHERES_PROFILE_SCENARIO` selects `idle_human` or `industry_and_war`; omit it for both measurements.

Separate long preparation from timing when running balance panels:

1. Set `SPHERES_PROFILE_PREPARE_ONLY=1`. The test advances only the idle world by default and saves complete campaign checkpoints under `profile-campaigns` beside the output JSON, without timing them. Explicit `SPHERES_PROFILE_SCENARIO=industry_and_war` retains the older busy-preparation option, but the preferred measurement does not need those files.
2. To resume preparation, set `SPHERES_PROFILE_INPUT` to that absolute directory and `SPHERES_PROFILE_PREPARE_RESUME_SLOT` to a saved slot such as `profile-idle_human-10`. Earlier checkpoint ages are skipped. The saved calendar date is preserved.
3. Stop other workloads. Unset preparation, resume and scenario variables, set `SPHERES_PROFILE_INPUT` to the complete checkpoint directory, and run again. Both scenarios load `saves/profile-idle_human-{0,10,30}.json` independently. Busy slots are ignored and source files are never overwritten.

For each busy measurement, setup retains the checkpoint's date, history and dispatch archive. It sets USA political capital to a controlled 100, enables economic competition, enacts an annual/department budget and starts a legal player project. Starter Industry with the normal recommended capacity is preferred; a legal conventional project is the fallback. The inherited allocation is retained except that an industry allocation below 2% receives that minimum authority within the total budget cap. A department budget already enrolled uses its normal renewal command. Existing queued work can satisfy the project requirement if no new construction slot is legal.

The setup prefers an ordinary Iraqi declaration against Kuwait. If that pair is unavailable, it selects a legal living AI pair deterministically, preferring neighbors and then nation-ID order. Only the selected attacker's political capital is set to 100; the declaration pays its ordinary price. Both participants must have positive existing military strength. Setup must establish a new shooting war and at least one actual player project before the first sample. No GDP, treasury cash, material stock, completed infrastructure or free construction progress is granted. Projects still face real inputs, operating authority and capacity constraints, and wars may end during the sample.

The report records every successful setup command, its price and payer's political capital before/after, both explicit political-capital overrides, source and measured-start world hashes, source and actual rules, starting archive sizes, starting counts and all 31 days' actual shooting-war/project counts. Hashes use the simulation's FNV-1a world fingerprint; they are not cryptographic hashes of the presentation archive. Setup runs outside the measured turns.

With `SPHERES_PROFILE_INPUT` set, the targeted test `performance::aged_checkpoint_stress_uses_priced_commands_and_preserves_the_archive` also validates all three saved ages and prints their setup records without advancing any measured days. Its built-in fixtures replay the recorded commands to check the full resulting world and ordinary side effects, including any priced access requests triggered by war.

Set `SPHERES_PROFILE_EXPORT_STRESS=1` during an `INPUT` measurement to save each newly established busy campaign before its first timed day. Full archives go to `profile-stress-campaigns/saves/profile-industry_and_war-{age}.json` beside the output JSON; the report records the destination. This disk work is outside the measured loop, and the idle input files remain untouched. Load these copies in a disposable browser server to measure the same starting workload through the UI.

This is **new stress on an aged idle world**, not an uninterrupted economic-AI campaign. It compares aged history/payload cost under an observable workload without decades of unnecessary AI warmup. The separate daily calibration panel still runs actual uninterrupted AI-on trajectories to assess long-run behavior.

The JSON separates simulation plus history recording, state read-model generation, state JSON serialization, selected-country history delta plus serialization, and their combined server time. It also records state/history/production payload sizes, median, nearest-rank p95 and maximum. The full all-country history request is reported for comparison; normal advances do not request that archive.

These server measurements exclude network transfer, browser painting and disk autosave. The report states each actual starting date: a checkpoint saved after an earlier 31-day sample can begin in February, rather than silently claiming January.

## Browser measurements

In a disposable campaign, open **About → Performance sample** and choose **Start sample**. Sampling is off by default, bounded to 256 observations, and sends no telemetry. Advance days and use ordinary controls. Return to the panel, choose **Stop sample**, and copy its displayed JSON.

The render metric measures synchronous work inside the state-adoption render call. Input timing measures a trusted click or key event until two animation-frame callbacks. It is a local estimate of the next rendering opportunity, not a standards-compliant INP measurement or proof that every pixel has painted. Dialog controls used to start and stop sampling are excluded. Pair these observations with the independent server timings; do not add unrelated percentile values together.

Repeat at ages 0, 10 and 30, including the industrial/war fixture. Record viewport, browser, hardware, build revision, campaign date, sample count and active workload. A short sample on one machine establishes a provisional target and catches large regressions; it does not guarantee performance on other hardware.

## Core phase diagnosis

`spheres-sim/examples/materials_profile.rs` times individual daily passes and compares its mirrored sequence to the ordinary tick. Its optional input accepts a saved campaign envelope or raw world. Retain flags and all source state when diagnosing an existing world. Exact saved-state, RNG and headline parity is required before using a profile to justify an optimization. Relative phase shares collected under contention can locate a hotspot, but absolute timing claims require a separate idle-machine run.
