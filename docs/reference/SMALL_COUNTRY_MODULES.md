# Scalable starter workshops

Approved by Ridge on September 4, 2026, after the small-country diagnosis.
Local review build; no GitHub publication is authorized by this change.

## What a module buys

A starter workshop is a purchased fraction of four existing game-modeled
facilities: industrial estate, generation, local power grid and materials
processing. A standard package costs $580m in installation/labor, plus the
sum of the existing four raw recipes. This is a model price, not historical
factory data. One million capacity units equals one standard package.

An order specifies 1–1,000,000 integer capacity units. Its size never changes
because GDP grows, the budget changes, or another government captures its
province. Expansion buys additional units at the same proportional bill and
inputs. Existing integer-level facilities and unfinished full-size projects
keep their original costs, capacity and paid work. Cancelling old work still
forfeits its sunk money and inputs; it is not a free conversion into modules.

At scale `s`, the finished package provides:

- `s` intermediate packs/day before inputs, operating finance and storage limits;
- `10s` available generation and `5s` local grid capacity;
- `0.15s` additional national construction capacity, within the existing cap.

Its materials and installation cost also scale by `s`. Installed capacity alone
grants no goods, cash, research or GDP. Actual output consumes the existing
iron/bauxite/coal recipe, generating fuel, power and departmental operating
funds. Output and used internal power enter provincial value added once;
selling that inventory never grants GDP again.

## Construction and recommended sizing

The combined work requirement is 1,800 standard project-work units, scaled by
the purchased amount. Progress is internally normalized to 0–1,800 to retain
precision for tiny orders; the UI shows completion percentage and physical
work, not 1,800 calendar days for every workshop. Minimum commissioning time
is 90 actual dates. Budget, queue sharing, inputs and conflict can make it longer.

The recommended amount fits approximately one year of the current Factories
department's allocation AND one year of unshared construction capacity. Daily
authority follows `1 / (12 × days_in_month)`, so sizing uses the conservative
31-day-month release rate rather than assuming every day releases `annual/365`.
The displayed earliest date is an optimistic estimate, not a delivery guarantee.
The browser renders Rust quotes and sends the exact integer size chosen.

All projects and enrolled mines share national work capacity. Capital authority
is not treasury cash. Existing atomic preflight, daily accrual, annual renewal,
payment and material ledgers remain the only implementation paths.

## AI behavior

When a normal starter package exceeds a year's factory allocation, an eligible
government can choose an affordable module instead. There is no country-name
whitelist. Political capital, active budgets, ownership and material access
remain requirements. The AI uses actually enacted departmental shares at order
time, never an unapproved budget proposal.

Completed module countries can specialize in intermediates. They do not
immediately queue a giant machinery plant. Additional module capacity requires
real domestic consumption/construction demand or recent delivered exports,
and room for the goods. A fully paid standard estate-equivalent can eventually
meet full-facility prerequisites; a tiny module does not count as a whole estate.
An established full-size industrial chain keeps its ordinary next-project
planning even if that country acquires a small module. Owning one fractional
workshop does not force an otherwise large industrial economy to specialize.

There is a combined five-standard-capacity limit per relevant province
capability, counting both modules and ordinary installed/queued facilities.
Splitting a package cannot multiply enabling bonuses or avoid this ceiling.
Each new order pays the shared catalog political-capital price.

## Review surface and safety

Exchange → Industry → Choose a workshop size offers **Pocket workshop**, **Budget
fit**, and **Room to grow**, with a province selector, installation price,
production capacity, input list, minimum/estimated timing and construction
progress. Full-site counts and fractional workshop capacity remain distinct.

Module purchases use the existing campaign-bound, exactly-once command receipt.
A lost response can be retried without ordering another workshop. A changed
campaign/day/province invalidates older quotes; the server validates again.

### How to review in the game

1. Open Exchange and enable Economic Competition in the review campaign.
2. Enact the ministry/department budget if its books are not open yet. Factories
   pays installation; generating and operating departments still fund running
   production. A project quote is not new money added to the treasury.
3. In **Your industry**, open **Choose a workshop size** and select a province.
4. Compare Pocket workshop, Budget fit and Room to grow. Their names are choices,
   not timing guarantees: Room to grow is larger and can take about two years.
   Open **What this size needs** for the actual material units and current stock.
5. Build once, then advance ordinary days. Funding and raw shortages pause work;
   missing operating inputs can idle a completed workshop. Completed capacity
   remains visible on Production's province list/map and the Exchange operating
   cards. Provincial GDP records realized value added, not the build button.

In the initial Tonga browser demonstration, the default 20% Factories split
quotes $385.7k / $771.4k / $1.5428m of installation for the three options.
Budget fit buys 0.133% of a standard package, with 0.00133 intermediate packs/day
before operating constraints. These are observed game quotes for that budget,
not historical project prices or promises for a different campaign.

## Known boundary: six missing maps

Bahrain, Mauritius, Seychelles, Comoros, Cape Verde and Maldives have empty
territory/district lists in the existing source artifact. They currently cannot
host physical construction or manufactured freight routes. The builder names
that limitation. No province, resource deposit or gateway was fabricated to
turn the coverage test green. Sourced map expansion is separate work.

## Verification contract

- All 131 mapped starting countries: normal sourced GDP and ministry envelope,
  finite explicit political/material fixtures, paid completion and positive
  operating output within the tested recommendation horizon. This isolates
  physical affordability; it is not an organic macroeconomic success-rate claim.
- The six unmapped governments: exact, atomic refusal, explicitly enumerated.
- Exact proportional bills/input closure, no free integer levels, bounded
  expansions, minimum lead time, shared funding/work, save/resume and capture.
- Provincial GDP: actual intermediate and power receipts reconcile, migration
  recognizes existing scale, expansions do not grant a completion bonus.
- UI/server: read-only quotes, bound actor, invalid-size rejection, stale quote
  protection, exact order size, retry safety and responsive browser layout.
- Separate unmodified-world census: ordinary initial stocks, budgets, PC and
  conflicts, reporting completed module capacity and first actual production.

## Verification results — September 4, 2026

- Final simulation/CLI and web release suites combined: **634 passed, 3 known
  baseline failures, 55 ignored**. No test tolerance or expected legacy hash was
  changed. The 18-test AI integration suite includes deterministic save/resume.
- **9/9** dedicated small-country integration tests; the 131-country affordability
  check completes every mapped country within 90–365 days under the explicit
  fixture described above. **11/11** core module tests also pass.
- **161/161** final serverless UI tests. The real-browser module purchase test
  passed at 1440, 820 and 390 pixels; Exchange navigation and save/load/retry
  suites also passed against the disposable review server, with no page errors.
- **3/3** census observer tests pass: retained receipts are counted once and
  paid escrow/held cargo are never misreported as usable delivered goods.
- The mixed-assets planner and disappearing-completed-workshop regressions were
  each observed failing before their repairs, then passing afterward.

The unchanged failures are the existing Belgium E-3 technology growth check and
the two legacy golden expectations. Actual start hash is `0xe26e4bf8d6c60066`;
actual known-run hash is `0xbe94d6125631829c`, both unchanged from overnight v5.
This is not an all-green baseline and does not claim to resolve those decisions.

Evidence is kept in the sibling `../../artifacts/` directory:

- `small-country-modules-sim-cli-v7.log`
- `small-country-modules-web-v7.log`
- `small-country-modules-ui-v7-final.log`
- `small-country-modules-browser-v7-final.log`
- `small-country-modules-exchange-browser-v7.log`
- `small-country-modules-session-browser-v7.log`
- `small-country-modules-observer-v7.log`
- `small-country-modules-v7-final-screenshots/`

### Unmodified-world outcome

The final `modules-v7` run completed **450 daily ticks, seed 42**, ending March 27,
1991. It changed no starting GDP, stocks, political capital, budgets or conflicts
to manufacture success. All 137 starting governments were evaluated and remained
alive. **101 countries paid for construction; 54 completed modules and produced
actual intermediate goods.** This is one measured world, not a calibrated
success-rate claim or a promise that every government automatically develops.

| Starting GDP tier | Countries | Paid construction | Completed modules / producers |
| --- | ---: | ---: | ---: |
| Under $1bn | 16 | 7 | 7 |
| $1–10bn | 49 | 31 | 30 |
| $10–100bn | 42 | 35 | 16 |
| $100–1,000bn | 23 | 21 | 1 |
| Over $1,000bn | 7 | 7 | 0 |

Tonga and Solomon Islands both reached actual production in their first year
(settled-day indices 349 and 353). Their final annual project value-added rates
were about **$136.6k** and **$112.0k**, respectively. These are components of GDP,
not treasury deposits or the total growth caused by every economic system.
The seven micro producers were Belize, Solomon Islands, Vanuatu, Tonga, Bhutan,
Lesotho and Suriname.

The remaining limitations are visible, not counted as successes:

- Equatorial Guinea, Sao Tome and Principe, and Western Samoa were still making
  fiscal adjustments; Laos and Guyana were saving political capital. The new
  construction size does not bypass those decisions or their costs.
- The six map gaps remain. Four fall in this run's micro tier and two in its
  small tier.
- No manufactured-goods deliveries occurred by this horizon. Producing inventory
  is not evidence of finding a buyer. Large legacy industrial chains remain
  slower and their unfinished paid work was not converted for free.

The independent final-world audit found **zero violations**: 54 module provinces
(7.312231 standard capacities total), three active modules, 47 unique active
projects, valid component caps and finite nonnegative physical inventories.
Paid progress reconciled within `1.39e-17` bn and raw recipes within `1.28e-13`
units. All 137 national/provincial GDP allocations reconciled within `2.73e-12`
bn; open-books debt ratios reconciled exactly. An endpoint check cannot prove
full historical conservation or frozen sizing; the regression tests cover those.

Preserved evidence:

- `economic-competition-census-450-seed42-modules-v7.json`
- `economic-competition-census-450-seed42-modules-v7.world.json`
- `economic-competition-census-450-seed42-modules-v7.log`
- `economic-competition-census-450-seed42-modules-v7.run.json`
- `economic-competition-census-450-seed42-modules-v7.audit.json`
- `economic-competition-census-modules-v7-source-manifest.json`
- `audit-industrial-modules.cjs` (read-only endpoint verifier)

The watched census executable SHA256 is
`9A318C75B878624CAE457D4ABB0119E253715568401002E1A62F7E9087F87C79`.
All 204 recorded simulation source/data files still matched the frozen manifest
at final verification. The interrupted v6 diagnostic is explicitly marked
partial, not relabeled as final-source evidence. Old overnight v5 measurements
remain preserved. No commit or GitHub push was made.
