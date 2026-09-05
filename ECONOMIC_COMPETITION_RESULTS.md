# Economic Competition — measured review results

Latest follow-on: Ridge approved scalable starter workshops after this overnight
diagnosis. They are now built and verified; see [SMALL_COUNTRY_MODULES.md](SMALL_COUNTRY_MODULES.md)
for the current preview, 131-country affordability test and final 450-day run
(54 producers, including seven micro-economies). The v5 results below are
preserved historical evidence, not the post-module results.

Status: implementation and verification complete, September 4, 2026, and included
in the integrated release. This is not a claim that the economy is fully balanced.

## Morning summary

The corrected ten-year run covers **all 137 starting countries across five GDP
tiers**: 123 paid for construction, 101 completed capability, 70 produced goods,
and 10 received manufactured deliveries. It has 490 installed capability levels.
The connected economic loop runs without free fixture subsidies; accounting and
retry tests pass. **Micro-economy pacing is not satisfactory:** just one of 16
finished a site in ten years, and none produced manufactured goods.

Run `cargo run -p spheres-web`, open the printed local address, then open
**Exchange** and **Enable Economic Competition** if the campaign has not enabled
it. The four views are Industry, Goods Market, World Economies and Your Sphere.

## What is implemented

The daily competition mode connects AI civilian investment, ministry funding,
raw inputs, manufactured goods, physical freight, treasury receipts, provincial
value added, funded research prototypes and voluntary economic spheres. The
Exchange exposes these through four full-page views. Activation is explicit;
old headless monthly campaigns retain the legacy model.

## What the tests establish

The assembled v5 Rust workspace run recorded **610 passes, three unchanged
pre-existing failures, and 55 ignored diagnostics**. The two legacy actual
hashes remained `0xe26e4bf8d6c60066` and `0xbe94d6125631829c`. E-3 and the two
pending golden-pin decisions were not suppressed or widened.

The browser target passed 127 tests, with two existing diagnostics
ignored. All 156 serverless interface tests passed. Real desktop/tablet/mobile
checks covered all four Exchange views, 137-country filtering, keyboard focus,
save/load, stale campaign protection, delayed responses, denied browser storage
and exactly-once order/turn retries. The delivery census observer passed three
additional tests separating arrivals, escrow and held cargo.

These counts include the v5 warehouse repair: three new regressions and an
18/18 AI integration suite. The observer separately passed 3/3. Both browser
suites and all 156 serverless checks were rerun on v5. The completed corrected
v5 five- and ten-year runs are distinguished below from the older v3/v4 evidence.

## Unmodified-world runs

The census starts from the actual 1990 game state, with no fixture cash,
political capital, materials or factories. Tiers use each country's starting
GDP in billions of 1990 dollars. Counts below are countries, not factory counts.

| Starting size | Countries | Paid construction after 1 year, both final seeds | Paid construction after 5 years | Completed capability after 5 years | Produced manufactured goods after 5 years |
| --- | ---: | ---: | ---: | ---: | ---: |
| Micro, below $1bn | 16 | 7 | 9 | 0 | 0 |
| Small, $1–10bn | 49 | 30 | 36 | 24 | 3 |
| Medium, $10–100bn | 42 | 35 | 40 | 36 | 19 |
| Large, $100–1,000bn | 23 | 21 | 21 | 20 | 14 |
| Major, $1,000bn+ | 7 | 7 | 7 | 6 | 1 |
| Total | 137 | 100 | 113 | 86 | 37 |

The final one-year runs use seeds 7 and 1990. Every country was evaluated 13
times. Budgets opened for 136 and 137 countries respectively; Peru accounts
for the difference but had no paid construction in either run. Neither run
completed a facility or produced/traded manufactured goods in its first year.

The five-year columns above apply to both v3 and corrected v5 seed 42: their
country-level milestone counts are identical, not their complete simulation
states. V5 completed 1,826 actual calendar days in **4,149.13 seconds**, exiting
zero on September 4 at 00:16 Pacific; v3 took 4,221.41 seconds. There are 250
installed facility/upgrade capability levels
at the end, spread across the 86 countries above. The Soviet Union and
Yugoslavia no longer exist as governments; their -100% national-series endpoint
is dissolution, not a claim that their successor economies vanished. There
are 135 surviving governments from the starting roster.

**Manufactured commerce has not yet emerged in either five-year run:** net
import reservations and seller dispatch receipts are both zero. V5 also directly
records zero usable arrivals, escrow and moving/held cargo. Production is
therefore verified at five years, but naturally operating international goods
trade appears only in the later ten-year measurement below.
A signed/escrowed order would not prove usable delivery in any case.

### What the corrected final save explains

The v5 snapshot contains 48 standing sale policies but no manufactured offers,
contracts or shipments. There is no hidden payment or freight failure:
`commerce.next_id=0`, and there is no outstanding manufactured shortage at this
endpoint. Of 37 countries with processing plants, 11 have completed machinery,
21 have machinery queued, and five are waiting on political or fiscal decisions
(United States, Switzerland, Norway, Cameroon and Bangladesh). No warehouse
project is blocking the chain.

The first 11 machinery plants completed **December 27, 1994**. This date is
reconstructed from the saved December headline order: all 11 completions sit
after 26 and before 27 of the 31 once-daily US copper delivery notices, with
production preceding that day's notice. They operated through December 31,
producing 2.5 capital packs each. Their identical 254.5 combined production is
therefore not a permanent ceiling or evidence of years of idle machinery.
Their 249.5 intermediate stock exceeds the next planning period's 15-pack need.

France, Egypt, Luxembourg, Albania and Venezuela are building machinery without
domestic processors, showing the import-specialization path; none of these new
buyers is operational yet, and Venezuela's project is contested. The repair
also adds 124 paid machinery-work days in Czechoslovakia and Thailand, and 64
in Romania, relative to v3. Their facilities have not finished by the endpoint.
Five-year ramp-up is a measured pacing concern for an arcade experience, not
permission to invent buyers, stock or instant construction.

### Second corrected five-year seed

V5 seed 7 completed 1,826 days in **3,898.26 seconds**, exiting zero on September
4 at 01:21 Pacific. All 137 starting countries enrolled; 135 survive. It records
113 paid-work countries, 82 with completed capability, 38 producers and 243
installed levels. Fifty-two countries have positive cash. As in seed 42,
manufactured orders, dispatch cash, usable arrivals, escrow and cargo are zero.

| Starting size | Paid work, seeds 42 / 7 | Completed capability, seeds 42 / 7 | Producers, seeds 42 / 7 |
| --- | ---: | ---: | ---: |
| Micro | 9 / 9 | 0 / 0 | 0 / 0 |
| Small | 36 / 37 | 24 / 24 | 3 / 3 |
| Medium | 40 / 39 | 36 / 32 | 19 / 20 |
| Large | 21 / 21 | 20 / 20 | 14 / 13 |
| Major | 7 / 7 | 6 / 6 | 1 / 2 |

Both corrected reports have 137 unique rows and no invalid values in the
checked cash, debt, GDP, work, production and commerce fields. These two
realizations measure variation; they do not establish a calibrated success rate.

### Final corrected ten-year run

V5 seed 42 completed **3,652 calendar days in 9,762.11 seconds**, exiting zero on
September 4 at **04:29:38 Pacific**. The report and 51,698,152-byte actual final
save were verified. All 61 frozen source/Cargo hashes and the executable hash
still match the tested build. All 137 starting countries enrolled, and 135
survive as governments; successor states are simulated but are outside this
fixed starting-country table.

| Starting size | Countries | Paid construction | Completed capability | Produced goods | Received usable deliveries | Supplied deliveries |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Micro, below $1bn | 16 | 11 | 1 | 0 | 0 | 0 |
| Small, $1–10bn | 49 | 41 | 35 | 18 | 3 | 0 |
| Medium, $10–100bn | 42 | 41 | 37 | 30 | 4 | 0 |
| Large, $100–1,000bn | 23 | 23 | 22 | 17 | 0 | 2 |
| Major, $1,000bn+ | 7 | 7 | 6 | 5 | 3 | 2 |
| Total | 137 | 123 | 101 | 70 | 10 | 4 |

The four exporters are **United States, China, Japan and India**. Starting-roster
recipients are Germany, United Kingdom, France, Egypt, Venezuela, Cuba,
Luxembourg, Albania, Trinidad and Tobago, and Brunei. They received **$224.23m
of reference-valued goods**, not an equivalent amount of profit or new GDP.
Another **$6.64m** of reference-valued cargo is moving to starting-roster buyers.
No held cargo or undispatched escrow remains. Organic contracts observed so far
trade intermediates, not capital goods; the latter is supported and tested but
not demonstrated as organic commerce in this run.

**Whole-world accounting is a separate check.** Including successor countries,
15 buyers reserved **$291.63m**, four sellers received that amount at dispatch,
and remaining escrow/refunds are zero. The difference is only floating-point
rounding (5.55e-17 bn). Fourteen buyers have actual arrivals in retained
contracts; the fifteenth has only paid cargo still moving. All **264 retained
contracts** reconcile ordered quantities with remaining, delivered, cancelled
and in-transit quantities, and paid cash with dispatched goods at contract
prices. Of these, 256 are delivered; eight shipments are legitimately not due.
There are no duplicate contract/cargo IDs, orphan cargo, invalid checked
finances, or manufactured stocks above warehouse capacity. This endpoint audit
does not reconstruct full-history stock conservation from pruned records.

Progress continues after year five: **113 starting countries gained more paid
construction days**, 15 gained their first completed capability, and 70
increased manufactured output. No living country's AI review is over 30 days
stale. The remaining 82 projects include 48 slowed by funding/shared capacity,
four contested, and 30 without a reported blocker. No current project reports
the repaired manufactured-goods construction deadlock.

The world has **52 warehouses and 35 research centers**, with all 35 centers
performing funded prototype work on the final day. Existing voluntary compacts
are **China–Tanzania and United States–Chad**. Neither partner has purchased
manufactured goods in the accounts, so these are not evidence of manufactured
trade producing peaceful domination. The largest current manufactured-only
dependency is approximately **0.1342%**, far below the 12% compact threshold.

### Older ten-year diagnostic: actual trade emerged

The **pre-repair v4** ten-year seed-42 run completed 3,652 days in **12,197.25
seconds**, exiting zero on September 4 at 01:05 Pacific. It is not a validation
of v5. Among the 137 starting countries: 123 paid for work, 101 own completed
capability, 69 produced goods, **15 received usable manufactured deliveries and
four supplied them**. There are 476 installed levels. Lesotho is the sole
micro-country with a completed site; no micro-country produced manufactured
packs. The small-country pacing limitation remains.

The starting-roster importers received $236.84m of goods at modeled reference
prices across the run. This is not negotiated cash, profit or new GDP. Delivered
exports attributed to starting-roster sellers total $278.82m; those two sums
must not be balanced against each other as a global conservation check because
successor counterparties are outside the starting-country reporting cohort.
There is $2.81m of reference-valued cargo still moving to starting-roster buyers,
zero held cargo and zero undispatched escrow at the endpoint. Sellers include
the United States, China, Japan and France; buyers span small, medium, large
and major economies. These older results are retained as provenance, not
substituted for the corrected v5 results above.

## What needs attention

1. **Warehouse bootstrapping deadlock fixed; mature trade verified.** The old
   planner could choose storage expansion when intermediates approach capacity,
   before it owns machinery. Warehouses require capital-goods packs; machinery
   is their producer. A focused test failed before the correction, and an
   independent two-country control verified that an actually stocked seller
   receives a valid quote. The verified repair corrects dependency ordering and
   recovers a previously queued warehouse using normal costs and slots, while
   preserving its paid work and funding the real machinery prerequisite.
   This defect must not be treated as the proven explanation for every
   zero-trade country in the census.
2. **Small-country pacing is demonstrably slow.** Every mapped micro-economy's
   reported full-size site funding/work lower bound exceeds five years. Examples
   are roughly 73 years for Tonga, 90 for Solomon Islands and 29 for Vanuatu.
   These are optimistic bounds, not completion promises. An appropriate small-
   site design would need proportional cost, work, inputs, power and output;
   no AI-only discount was introduced to improve a test result. At ten years,
   Lesotho is the sole micro-country with completed capability. Remaining
   optimistic first-site estimates still reach decades; uniform full-size
   projects need a player-reviewed scale/pacing design.
3. **Political and fiscal constraints matter.** Many countries wait for normal
   command costs plus the AI reserve. Seventy-five countries ended each
   one-year run with zero cash; 86 did in the five-year run. Yet 51 five-year
   countries had positive cash, so lack of money alone cannot explain all zero
   trade. A ministry appropriation is not cash available for imports.
4. **Six countries have no mapped province in the current artifact:** Bahrain,
   Mauritius, Seychelles, Comoros, Cape Verde and Maldives. Afghanistan's wait
   was conflict-related, not missing geography. No province was invented.
5. **Goods-only peaceful domination is not demonstrated.** The AI's normal
   import envelope is far below the 12% dependence required for a compact.
   Pacts/raw sourcing may provide the larger dependency; lowering consent
   thresholds just to force a successful outcome is not a valid correction.

## Next review decisions, not automatic changes

All scheduled measurements are complete; **no census process or later seed is
queued**. Existing evidence is preserved. The sequential runner's smoke checks
passed, including refusal of duplicate queues, existing evidence and a changed
binary hash. The next changes should follow review of these results:

1. Scale down starter projects with proportionally smaller costs, work, inputs,
   power and output so micro-economies can participate without free subsidies.
2. Choose a faster, readable construction/ramp-up pace for the arcade experience.
3. Review political-capital costs/reserves for routine civilian decisions. Cash
   and appropriated budget are distinct; neither should bypass a real shortage.
4. Decide whether manufactured trade should create stronger economic dependence,
   or whether peaceful compacts should continue to rely primarily on pacts/raw
   sourcing. Do not silently lower sovereignty consent requirements.
5. Resolve the six missing country geometries and the existing E-3/golden-pin
   decisions separately, without fabricated province data or repinning failures.

The census is descriptive game-model evidence, not historical calibration and
not a statistically established success-rate bar. A positive viable-fixture
test, a real paid construction day, a completed facility, factory output and a
usable delivery are different milestones. No universal GDP-growth requirement
is imposed through recession, war or dissolution.

## Repeatable verification

From the repository root in PowerShell, using an isolated build directory:

```powershell
$env:CARGO_TARGET_DIR = 'target/competition-root'
cargo test --workspace --release --no-fail-fast -- --nocapture
cargo test -p spheres-sim --example economic_competition_census --release -- --nocapture
$checks = rg --files tools/ui | Where-Object { $_ -match 'check_.*\.cjs$' -and $_ -notmatch '_browser\.cjs$' }
node --test $checks
```

The first command intentionally still exits nonzero for the **three documented
baseline failures**; this is not an all-green suite. Real-browser commands and
their disposable-server requirement are documented in `tools/ui/README.md`.
Do not run mutating browser checks against a user's saved campaign. Measured
overnight outputs are listed below; reruns must use new artifact names.

## Evidence and provenance

The following verification artifacts were generated outside the repository and
are intentionally not part of Git history:

- `economic-competition-rust-suite-v3.log`
- `economic-competition-web-final.log`
- `economic-competition-ui-checks-v3-final.log`
- `economic-competition-browser-v3-final.log`
- `economic-competition-session-browser-v3.log`
- `economic-competition-census-observer-tests.log`
- `economic-competition-census-365-seed7-v3.json`
- `economic-competition-census-365-seed1990-v3.json`
- `economic-competition-census-1826-seed42-v3.json`
- `economic-competition-census-v3-source-manifest.json`
- `economic-competition-census-v4-source-manifest.json`
- `economic-competition-rust-suite-v5.log`
- `economic-competition-ui-checks-v5.log`
- `economic-competition-browser-v5.log`
- `economic-competition-session-browser-v5.log`
- `economic-competition-census-observer-tests-v5.log`
- `economic-competition-census-v5-source-manifest.json`
- `economic-competition-census-1826-seed42-v5.json`
- `economic-competition-census-1826-seed42-v5.world.json`
- `economic-competition-census-1826-seed42-v5.log`
- `economic-competition-census-1826-seed42-v5.run.json`
- `economic-competition-census-1826-seed7-v5.json`
- `economic-competition-census-1826-seed7-v5.world.json`
- `economic-competition-census-1826-seed7-v5.log`
- `economic-competition-census-3652-seed42-v4.json`
- `economic-competition-census-3652-seed42-v5.json`
- `economic-competition-census-3652-seed42-v5.world.json`
- `economic-competition-census-3652-seed42-v5.log`
- `economic-competition-census-3652-seed42-v5.run.json`

The v3 census binary hash is
`6EE008FAF32183EEC1E4EC7EDFDF585DEED0150F8826DFCB36F520A8272BDB97`.
The v4 observer binary hash is
`35B78EA6E95D3AC3E8480FAB167321BD78BD6E093A7F5D389D6EEA2CAC3A7E95`.
The corrected v5 census binary hash is
`24718EEDAD1AB7C311F5BDC9E66957B06ACDBFF169E03CBF778BA71B0F0FD2FF`.
The earlier unversioned seed-42 one-year report is diagnostic and predates final
planner/conquest repairs; it is not substituted for a final-source run.

The durable implementation boundaries, measured results and known limitations are
captured in this report and the linked system specifications.
