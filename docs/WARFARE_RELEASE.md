# Operational warfare release record

Prepared 2026-09-07. The delivered model and its assumptions are described in
[WARFARE.md](../WARFARE.md); the original execution plan is preserved in
[WAR_OVERHAUL_EXECUTION.md](../WAR_OVERHAUL_EXECUTION.md).

## Integration base

Repository: `ridgemerkley2-web/Spheres`. GitHub's actual default branch is
`claude/admiring-euclid-a867c9`; branch names alone were not used to select a base.
The isolated implementation branch is `codex/war-overhaul`.

The release incorporates these landed changes:

| Commit | Integration |
| --- | --- |
| `f2709d120673bf24c02460fb68babe4be5a5ba80` | Industry, Companies, equipment lifecycle, physical ammunition and aviation |
| `3b755687710bca6dbf8963bbe4f9acd878423f63` | Active default including shoreline work equivalent to `feat/art-p0` at `32333fe` |
| `825f7964ab3cceba9594e353634bb53f0ea86030` | Population, qualifications, household classes, staffing and education |

Population staffing is evaluated before war allocation. Combat debits the actual
national force loss once, then records its population casualty consequence once.
Population assigns workers before construction and production; military routing
uses the remaining freight capacity after commercial reservations. Negotiated
district transfers retain their population ledger through the existing ownership
transfer function. No other task's uncommitted work was copied.

Operational conflict IDs are reserved at creation and retained across saves.
This prevents a later war from inheriting a closed war's orders, forces or peace
policy. Existing operational saves adopt their retained conflict IDs before
cleanup; the legacy monthly allocator keeps its previous behavior.

## Verification

The locally verified simulation, web, UI and test source revision is
`c9c267cf30d33daa4efc3538972a3cb9732ed49a`. The final documentation commit does not
change those sources. GitHub publication verifies the changed blobs and the
complete final Git tree. The browser identifies itself as
`c9c267cf30d3-modified`: the only uncommitted files at build time were release
documentation (`ROADMAP.md`, `WARFARE.md`, `WAR_OVERHAUL_EXECUTION.md` and the new
release record). Source comparisons against the committed revision were clean.

| Final check | Result |
| --- | --- |
| Release Rust workspace | 1,291 passed, 0 failed, 70 existing ignored tests across 50 targets |
| Campaign observer example | 4 passed, including identity reuse, history retention and world/RNG purity |
| UI suite | 1,064 passed, 0 failed |
| Browser suites | All five passed; desktop/mobile inspection completed |
| Release web and diagnostic builds | Passed |

The [machine-readable verification record](warfare-verification.json) includes
the source tree, per-scenario results and SHA-256 hashes of the local raw evidence.
Local logs are in `artifacts/warfare/`; screenshots and browser results are in
`artifacts/browser-ci/final-c9c267c/`.

The Rust gate includes the existing monthly replay, front calibration, equipment,
logistics, resource accounting, population, industry and web suites. Existing
golden values and tolerances were not relaxed. Ignored tests remain explicitly
ignored; they are not counted as passes. The observer has its own purity test
because Cargo's normal workspace gate does not execute example tests.

Browser verification uses disposable campaigns and the built server. It covers
the operation board, real commands and refusals, authenticated actor binding,
enemy-data redaction, dated reports, human peace consent, save/load, uncertain
command recovery without a duplicate charge, and the combined Industry,
Companies and People interfaces. Desktop and narrow mobile layouts are included.

## Campaign diagnostics

The committed `campaign_report` example runs the full daily world with normal
command prices, actual starting forces, population, industry, Companies and
logistics. It declares war after an opening peaceful day, so force must travel.
The script changes approach and reserves and offers reparations followed by a
ceasefire. It leaves later player decisions unanswered; a continuing war does not
mean the AI never offered peace.

All nine final `campaign_report_v2` runs passed their invariants:

| Panel | Observed result |
| --- | --- |
| 90 days, two scenarios, seeds 1990/7/42 | All six passed. Iraq/Kuwait reached coalition-approved peace after 75 active days on 1990-03-17; Iraq/Iran remained active for 89 days. |
| 365 days, both scenarios, seed 1990 | Both passed. The original Kuwait war stayed closed; Iran's later ceasefire proposals expired because the scripted player did not respond. |
| 3,650 days, Iraq/Kuwait opening, seed 1990 | Passed whole-world invariants through later independent conflicts. The original war retained its 75 active days and original peace records. |

Across the 90-day seeds the Kuwait panel reached approximately 0.760 control with
seven direction changes; the Iran panel ended around -0.29834 with eleven. These
runs are similar across the sampled seeds, so they do not demonstrate a broad
distribution of outcomes. The year-long simulations took 84.7 and 157.7 seconds;
the 3,650-day run took 500.5 seconds of simulation time. Its largest recorded
national force excess was `1.42e-14`, within the declared `1e-7` roundoff allowance.
The instrument tracks the original war separately from later conflicts and
preserves its offer outcomes when the world's bounded history rolls forward.

These are descriptive gameplay and endurance checks. They do not establish
historical casualty rates, a calibrated win probability or human enjoyment.
Control reversals count changes in the direction of observed aggregate control,
not encirclements or won battles. Losses are saved actual battle receipts in
abstract force points, not personnel estimates. A closed war has no invented
terminal control value. The diagnostic checks finite state, nonnegative physical
force/service, bounded cohesion/control, and nationally conserved campaign pools.

Timing was collected on Windows, AMD Ryzen 7 9800X3D, Rust 1.98.0 and Node 24.12.0.
Other verification work ran concurrently; these timings are observations, not an
uncontended performance benchmark. Wall-clock time never feeds simulation rules.

## Reproduction

Use an isolated `CARGO_TARGET_DIR`, then run:

```sh
cargo test --locked --release --workspace --no-fail-fast
cargo test --locked --release -p spheres-sim --example campaign_report
node tools/ui/run-unit.cjs
cargo build --locked --release -p spheres-web
npm ci --prefix tools/ui
npx --prefix tools/ui playwright install --with-deps chromium
node tools/ui/ci-browser.cjs
node tools/ui/check_campaign_browser.cjs
node tools/ui/check_campaign_integration_browser.cjs
cargo run --locked --release -p spheres-sim --example campaign_report -- --days 90 --seeds 1990,7,42 --output artifacts/warfare/campaign-report.json
cargo run --locked --release -p spheres-sim --example campaign_report -- --days 365 --seeds 1990 --output artifacts/warfare/campaign-report-year.json
cargo run --locked --release -p spheres-sim --example campaign_report -- --days 3650 --seeds 1990 --scenarios unequal_neighbors --output artifacts/warfare/campaign-report-decade.json
```

`check_industry_rebuild_browser.cjs` and `check_population_browser.cjs` accept a running
disposable server URL; see [the UI tools guide](../tools/ui/README.md). Do not point
their mutation scenarios at a campaign that should be preserved. GitHub Verify
runs the workspace, observer, UI and campaign browser gates on Windows and Ubuntu.
Remote CI status belongs to the published commit's checks, not this local record.

## Declared model limits

Routed sustainment is a modeled service with a disclosed freight-capacity rule,
not a historical ammunition-tonnage estimate. Actual ammunition, equipment and
treasury transactions retain their existing authorities. Custom tactical
aircraft use supported strike missions and compatible stores; unavailable mission
capabilities are shown instead of invented. Version 1 uses consented limited
settlements and does not offer automatic annexation. Legacy monthly play retains
its previous conquest solver. Exported saves contain full simulation state for
replay even though normal opponent views redact exact tactical information.
