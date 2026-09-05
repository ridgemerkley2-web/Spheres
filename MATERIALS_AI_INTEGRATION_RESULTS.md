# Materials AI integration: local verification

2026-09-04. Local review on `codex/economic-competition`, above unchanged HEAD
`ca767fe4c83c73c02c448b5d2aeab888d023c11a` and the existing economic worktree.
No commit or push. Scope and design: [MATERIALS_AI_INTEGRATION.md](MATERIALS_AI_INTEGRATION.md).

## What changed

Economic AI can back its first machinery investment with a finite inherited
Materials order. The whole raw-input lot, current conversion/energy authority,
both power loads, ordinary command prices and political reserve must be covered.
Owned stock, imports and existing contracts count once. Temporary funding waits
retain the physical machinery target; queued startup stock is protected from
surplus sales. Warehouse rescue covers both consumers and prefers a usable
capital-goods import over an unnecessary new machine.

Post-settlement orders now receive their full stated service window. This fixes
the measured 29-of-30-day defect without changing existing saved deadlines or
pre-settlement player orders. There is no new household demand, free stock,
automatic startup raw procurement, factory grant or GDP growth multiplier.

## Regression evidence

Nineteen new integration tests cover twelve bootstrap cases, two warehouse
cases and five calendar cases. Regressions were observed failing before their
corresponding fixes, including the fully supplied 15-pack order expiring at
14.5, stale export reserves, insufficient shared warehouse coverage and an
unnecessary startup lot when real capital imports could finish the warehouse.
All forty focused operating/bootstrap/planning/calendar tests passed after
the final calendar change.

Two census-observer tests check bounded reason categories and dated receipts;
the stale-receipt regression was also observed failing before correction.
The observer does not change the world or use wall time for state decisions.

| Final frozen-build check | Result |
| --- | --- |
| Full release workspace suite | 719 passed, 3 pre-existing failures, 57 ignored |
| New bootstrap / warehouse / timing tests | 12 / 2 / 5 passed; included above |
| Census observer tests | 2 passed separately |
| Serverless UI checks | 180 passed, 0 failed |
| Materials browser regression | Desktop, tablet and 390px phone passed; zero page errors |
| Once-only HTTP receipts | Lost-response retry, signing and cancellation passed |
| Patch whitespace check | Passed |

The suite is **not all green**. The same prior failures remain:

- `tech::tests::the_1990_endowment_does_not_move_year_one_growth`: Belgium
  `0.001851` granted versus `0.001749` ungranted.
- `tests::the_1990_start_is_pinned`: unchanged actual
  `0xe26e4bf8d6c60066`.
- `tests::golden_hash_of_a_known_run`: unchanged actual
  `0xbe94d6125631829c`.

No tolerance, sample-size bar, expected result or golden pin was loosened.

## Six-year same-seed comparison

Both builds ran seed 42 for 2,192 days from 1 January 1990 to 2 January 1996.
All accounting, frozen-capacity and exact continuation checks passed in both.

| Metric | Prior Materials pilot | Final AI integration |
| --- | ---: | ---: |
| Materials-producing countries, original cohort | 22 / 137 | 31 / 137 |
| Orders signed | 44 | 73 |
| Completed / expired / cancelled / still active | 0 / 44 / 0 / 0 | 69 / 2 / 0 / 2 |
| Actual inherited Materials packs | 581.595 | 537.809 |
| First order signed | 2 March 1994 | 1 May 1993 |
| First actual Materials output | 3 March 1994 | 2 May 1993 |
| Final installed machinery levels | 31 | 34 |
| Final installed full processor levels, excluding modules | 14 | 4 |

The first order arrived 305 days earlier in this particular world. More
countries used inherited Materials and the AI built fewer full processors before
machinery. Total inherited Materials packs were lower, not higher; this is not
a general GDP, welfare or throughput improvement claim. Contract quantities,
trade and downstream demand differ between the two worlds.

| Opening GDP group | Materials producers: prior → final | Packs: prior → final | Final machinery levels: prior → final |
| --- | ---: | ---: | ---: |
| Under $1bn (16 countries) | 0 → 0 | 0 → 0 | 0 → 0 |
| $1–10bn (49) | 0 → 0 | 0 → 0 | 0 → 0 |
| $10–100bn (42) | 9 → 14 | 242.000 → 246.210 | 12 → 14 |
| $100–1,000bn (23) | 9 → 13 | 228.547 → 226.533 | 15 → 15 |
| Over $1,000bn (7) | 4 → 4 | 111.048 → 65.067 | 4 → 5 |

The final observer additionally measured 86 countries with actual paid
processor/module output (13,765.149 packs), 34 countries producing machinery
goods (4,685.524 packs), and 22 countries receiving intermediate imports
(2,896.326 delivered packs). Machinery consumed 4,685.524 intermediate packs
under its existing one-to-one recipe. These cumulative flows were not recorded
in the old baseline, so no numerical before/after claim is made for them.

The remaining two Materials expiries belonged to France and Norway. Each
country recorded one blocked Materials operating-day with insufficient
conversion and energy department authority. Full service windows do not promise
funding on every date. Two other orders were still active at the horizon.

The natural run still recorded zero delivered capital-goods imports and zero
goods consumed by research. The 65 micro/small countries were not forced into
full-size machinery. This demonstrates the Materials-to-machinery integration,
not universal adoption, an optimized national economy, or a completed research
chain. Raw-input procurement for prospective orders and build-speed calibration
remain separate work.

The next supply-planning pressure is visible in the recorded reasons: machinery
had 6,390 blocked operation-days for missing intermediate packs; starter modules
had 14,395 storage-full operation-days. These are counts across sites and dates,
not distinct countries or evidence that every affected country stayed blocked.
The two active contracts were awaiting department authority on the final settled
date, 1 January 1996; that annual boundary is not evidence of permanent failure.

Final census hashes: prior `4fd52a8793ca766c`; new `b398fbbce9ac80b5`.

## Three-year cross-seed checks

Both 1,096-day runs ended on 1 January 1993 with all 137 original countries
enrolled and every invariant passing, including the extra 30-day exact replay.

| Opening GDP group | Paid processor producers: seed 1990 / 7 | Actual packs: seed 1990 / 7 |
| --- | ---: | ---: |
| Under $1bn (16 countries) | 8 / 8 | 67.867 / 68.504 |
| $1–10bn (49) | 33 / 32 | 2,165.456 / 2,150.558 |
| $10–100bn (42) | 19 / 19 | 3,012.015 / 2,589.852 |
| $100–1,000bn (23) | 1 / 1 | 171.094 / 171.094 |
| Over $1,000bn (7) | 0 / 0 | 0 / 0 |
| Total | 61 / 60 | 5,416.431 / 4,980.008 |

These are funded processor/module receipts, **not inherited Materials orders**.
Both shorter runs had zero Materials orders, machinery starts, delivered
manufactured imports or research-goods consumption. Their dominant recorded AI
decisions were continuing paid construction, waiting for real specialist demand,
and saving political capital. They establish paid specialist production and
accounting integrity, not universal or rapid completion of the industrial chain.
No new build-speed or adoption-rate calibration is claimed.

Final hashes: seed 1990 `5bc1b78e1e9bce8f`; seed 7 `f7a6511a30cc08fe`.

## Measurement boundaries

- Every census starts from the unmodified 137-country 1990 roster, with daily
  economic competition and inherited estimates enabled. No player, forced
  orders, gifted inputs, extra cash, reduced wars or imposed adoption target.
- Size groups use opening GDP, not whichever group a country reaches later.
  New successor states are checked by world invariants but not added to this
  original-cohort denominator.
- The six-year comparison uses seed 42 and 2,192 days in both builds, ending
  2 January 1996. The additional seeds use 1,096 days each, ending 1 January
  1993; they are not pooled with the longer run as an adoption statistic.
- A signed order is not output. A machinery construction start is not an
  installed machine. Ever-observed installation differs from end-state stock.
  Final factory counts must come from the saved world.
- Production sources and delivered imports are counted separately. Fungible
  packs have no lot provenance, so actual machinery/research use is not falsely
  attributed to a specific domestic contract or import. New cumulative
  observations were not recorded in the old baseline; missing is not zero.
- Running/limited/blocked counters omit a completion day's final `completed`
  status. They are status-day counts, not a total of all service opportunities.
- Each run checks finite balances daily, provincial/national GDP and Materials
  reconciliation periodically, unchanged inherited capacity, exact save
  roundtrip, and a further 30-day byte-identical save/resume and RNG branch.
  Continuation days do not enter the main census totals.
- Bahrain, Mauritius, Seychelles, Comoros, Cape Verde and Maldives remain
  explicitly unmapped. Their estimates are not turned into fabricated sites.

## Review preview and reproduction

The review server can be started with `cargo run -p spheres-web`. It starts with a clean France
campaign, Economic Competition enabled, and the ordinary unchanged ministry
program budget enacted. There are no test orders, gifted raw stocks or produced
packs. Previous campaign servers, including `7797`, were left untouched.

Build in a dedicated target directory before trusting test binaries:

```powershell
$env:CARGO_TARGET_DIR='target/materials-ai'
cargo build -p spheres-sim --release --example materials_census
cargo build -p spheres-web --release
cargo test --workspace --release --no-fail-fast
cargo test -p spheres-sim --release --example materials_census
& '.\target\materials-ai\release\examples\materials_census.exe' 2192 42 report.json final-world.json
& '.\target\materials-ai\release\examples\materials_census.exe' 1096 7 report-7.json
& '.\target\materials-ai\release\examples\materials_census.exe' 1096 1990 report-1990.json
```

The local evidence lives in the workspace's `artifacts` directory, not in the
game's save folder. Files named `materials-ai-final-*` and
`materials-ai-workspace-tests-final.log` are the final-build evidence; earlier
unsuffixed/pre-calendar partial runs are not final results. The preserved
`materials-ai-baseline-2192-42.json` and `materials-ai-baseline-world-42.json`
belong to the prior pilot binary, not the new implementation.
