# Materials pilot: local verification results

This records the initial pilot, before the subsequent first-machinery AI
integration. See [MATERIALS_AI_INTEGRATION.md](MATERIALS_AI_INTEGRATION.md) for
the follow-up; these earlier measurements are retained, not relabeled.

2026-09-04. Local review only; no commit or push. This records the Materials
operating pilot on top of the existing, already-dirty economic-competition work.
The full workspace is **not all green**: the same three pre-existing simulation
failures remain. No golden hashes or test expectations were repinned.

## Playable scope

The Exchange has a Materials dashboard and a paid, finite province-order desk.
Capacity, actual output, demand, inventory, domestic reservations and imports
remain separate. Government orders use real raw stocks, ministry conversion and
energy authority, generation, provincial grids and storage. Funded plants have
first call on shared power. Signing supplies neither free packs nor free GDP.

Observed inherited Materials production replaces its already-counted GDP share;
only production above that allowance adds GDP. The province and national
accounting screens expose the reconciliation. Cancellation, expiry, ownership
loss, once-only command receipts and save continuation are implemented.

The review preview can be started with `cargo run -p spheres-web`. It starts with a fresh France
campaign, Economic Competition enabled and the ordinary unchanged ministry
program budget enacted. It has no test order, gifted input stocks or fabricated
output. The earlier `7796` campaign was not modified.

## Automated and browser checks

| Check | Final result |
| --- | --- |
| Full release workspace suite | 700 passed, 3 pre-existing failures, 57 ignored |
| Serverless UI checks | 180 passed, 0 failed |
| Materials operating integration | 17 passed |
| Materials GDP accounting | 8 passed |
| Materials capacity/AI planning | 4 passed |
| Web quote and command-receipt tests | 2 passed, included in 134 passing web tests |
| Materials browser regression | Passed at desktop, tablet and 390px phone widths |
| Existing Exchange, starting-industry/Welfare and small-module browser regressions | Passed |

Focused checks are included in the workspace total, not additional to it.
Ignored tests were not silently counted as passes. The original baseline had
666 passes, the same 3 failures and 55 ignored tests.

Browser evidence covers authoritative editable quotes, capacity/output/demand
cards, paid signing with no instantaneous goods, intentionally lost HTTP replies
retried against the same receipt exactly once, cancellation, stale drafts and
campaign changes, and Expand / Upgrade / Import / Sell navigation. Phone-height
checks also verified that the fixed header does not cover inputs or the action.

The three unchanged failures are:

- `tech::tests::the_1990_endowment_does_not_move_year_one_growth`: Belgium growth
  `0.001851` granted versus `0.001749` ungranted.
- `tests::the_1990_start_is_pinned`: actual `0xe26e4bf8d6c60066`.
- `tests::golden_hash_of_a_known_run`: actual `0xbe94d6125631829c`.

## Three-year natural-world census

Each seed ran 1,096 ordinary daily ticks, 1 January 1990 to 1 January 1993,
with Economic Competition, resources, production, manufacturing and physical
logistics enabled. All 137 initial living countries were observed; there were
no gifts, forced orders, player overrides or suppressed wars. Inherited
estimates were enabled before province accounts, without granting usable assets.

| Seed | Initial countries | Materials ordering / producing countries | Final census hash | Extra exact save/replay days |
| --- | --- | --- | --- | --- |
| 42 | 137 | 0 / 0 | `f04039da53848c78` | 30 |
| 7 | 137 | 0 / 0 | `f7a6511a30cc08fe` | 30 |
| 1990 | 137 | 0 / 0 | `5bc1b78e1e9bce8f` | 30 |

**This is inactive-safety and world-coverage evidence, not proof of natural AI
Materials adoption.** There were zero Materials orders, delivered packs and
service payments in these runs. The targeted supplied/powered AI fixture does
place a real paid command and avoids duplicate reservations, but that fixture
is not a historical initial state or a natural adoption result.

Inspection of the seed-42 final state found no installed machinery works or
research centers, hence no downstream Materials consumption and zero current
Materials demand for every country. Its ongoing starter-industry, generation
and grid projects consume raw inputs instead. The AI deliberately does not
commission packs merely because inherited capacity exists. Longer-run demand
formation and opponent adoption remain an explicit next calibration question;
the census was not altered to force a favorable adoption rate.

There is also a specific AI bootstrap limitation: the first-machinery decision
still seeks an existing/queued processor or an imported/owned 15-pack supply
before prospective machinery demand qualifies for a Materials contract. The
pilot therefore cannot yet replace that first processor/import decision with an
inherited-capacity contract. This is not a missing command hook or a total
deadlock, but it is a real deferred behavior, not validated adoption.

In seed 42, the original-country cohort split into 43 without generation,
35 with generation but no completed grid, and 59 running starter modules without
downstream Materials consumers. The USA's grid was at 274.4/420 progress;
China and Japan were at 301/420. Germany's grid was waiting for rare earths.
Tonga and Bangladesh already held about 2.983 and 174.135 packs respectively,
with their AI explicitly waiting for actual domestic use or export demand.

Every seed covered 16 micro economies (under $1bn starting GDP), 49 small
($1–10bn), 42 medium ($10–100bn), 23 large ($100–1,000bn) and 7 major
(over $1,000bn). All mapped countries had a Materials estimate. Bahrain,
Mauritius, Seychelles, Comoros, Cape Verde and Maldives retained unallocated
national estimates because the map lacks their provinces; they cannot place
province orders on invented geography. Of the original 137 countries, 136 were
alive at the end of each run. New successor states were included in live balance
checks, not relabeled as original-country adoption observations.

All runs passed:

- Daily finite positive GDP and nonnegative treasury, debt, raw and manufactured
  inventories, cargo and escrow checks.
- Frozen inherited capacity/provenance, and periodic province plus unallocated
  GDP, sector totals and national totals reconciliation.
- Materials background/observed/already-included/additional accounting identities.
- Exact save roundtrip plus 30 further days of identical saved world bytes and RNG
  versus uninterrupted execution.

Because natural orders were absent, active-contract conservation, payment,
shortage, delivery and GDP-overlap behavior was verified separately in the
focused operating/accounting tests, including fractional small-country orders.

## Route-search performance and exactness

The natural census exposed a cold route-search cost in existing global spot
clearing. A bounded 40-day, seed-42 profile measured the actual system sequence
and checked its entire result against ordinary daily ticks before optimization.
One clearing can now reuse at most 128 shortest-path trees with identical
source, policy and transit permission keys; overflow uses the original search.
The cache does not survive a clearing or appear in a save. Live cargo capacity
is still subtracted at every dispatch.

In the recorded runs, measured 40-day system time fell from 25.3333s to 2.8953s
(about 8.75 times faster). This is a local bounded comparison, not a universal
game-speed guarantee. Both builds ended at exactly `e079686a12a8aede`, matching
ordinary daily world state, headlines and RNG. Exact oracles also cover 2,592
route comparisons, the cache limit/fallback, three seeded Materials-era
clearings and full clearing stock/cash/cargo ledgers.

## Evidence files

Generated logs, JSON reports and screenshots are under the workspace's
`../../artifacts/` directory, outside this repository:

- `materials-workspace-tests-fast.log`
- `materials-ui-tests-final.log`
- `materials-browser-fast.log` and `materials-browser-fast/`
- `materials-exchange-regression.log`, `materials-starting-regression.log`,
  `materials-modules-regression.log`
- `materials-census-1096-{42,7,1990}.json` and corresponding
  `materials-census-{42,7,1990}-fast.log`
- `materials-census-final-42.json`
- `materials-profile-40-42.json` and `materials-profile-40-42-fast.json`

Earlier partial census logs were superseded by the completed `-fast` runs.
Full system behavior, model assumptions and deferred layers are documented in
[MATERIALS_OPERATIONS.md](MATERIALS_OPERATIONS.md).
