# Materials operating pilot

Local review build, 2026-09-04. Ridge approved the Materials-first operating
pilot and its arcade dashboard after the inherited industrial estimates. This
does not authorize publication, silently migrate old saves, or build the later
jobs, household-income and Welfare feedback loop.

## The playable loop

Open the Exchange, enable Economic Competition, and enact a ministry program
budget. A fresh campaign with inherited industry has a Materials order desk:
choose an owned province, a finite quantity and a 7–365-day delivery window.
Signing costs 2 political capital, reserves capacity, and creates no goods.

This pilot is a **government toll-manufacturing contract**. The government
supplies raw inputs from its actual stockpile and pays conversion and power
services as work occurs. It is not a free requisition of the country's whole
economy, a private-firm simulation, or a promise that every factory is producing
government inventory. The amount that cannot be supplied simply waits.

Each day, existing funded plants operate first. Contracts then share the exact
remaining national generation, provincial grid, raw inputs, storage and daily
ministry authority. A complete, paid input bundle creates Materials packs that
can feed machinery, construction or the existing manufactured-goods trade market.
No automatic resource purchase or free power is attached to an order.
The order's requirements remain visible in the resource forecast, but cannot
authorize automatic spot purchases. Existing funded-plant and military
restocking policies remain in force for their own needs; this pilot does not
disable those older systems. Already-owned ingredients needed by a finite
Materials order are protected from automatic surplus sales.

- Capacity is inherited, estimated annual value-added capacity converted into
  packs using the same constant-price output-minus-input recipe as GDP.
- One pack uses iron, bauxite, process coal and power. Generation consumes coal
  once. Efficiency upgrades use the existing power-efficiency effect.
- Conversion is $0.00001bn per delivered pack from Industry's processing
  department; energy is $0.000002bn per power unit from its energy department.
  These are explicit **game assumptions**, not historical prices or profit.
- The finite total is represented to nine decimal places, allowing small-country
  orders without rounding them into a whole industrial complex.
- At most one active contract per province and 32 per nation. Orders are capped
  at one million packs and use the actual calendar for their deadlines.
- Cancellation and expiry stop future work. Delivered goods and paid work remain;
  no sunk-cost refund or reversal is invented. Legal ownership loss closes the
  old government's contract; contested control can pause it.
- Closed receipts are kept for a year, bounded to the latest 256 per nation.
  Cumulative delivered packs and conversion/energy payments remain in the save.

## GDP: make existing production visible, do not pay it twice

Starting manufacturing output is already in national and provincial GDP.
An operation records actual packs at fixed model prices, less raw materials and
internal power. Its inherited share is identified from the reserved slice of
capacity and the current inherited utilization of that province.

The Materials account explicitly reconciles:

    observed output = already included in GDP + additional output
    total Materials GDP = background Materials GDP + additional output
    total Materials GDP = unobserved background + observed output

These are annualized value-added rates, not cumulative cash. A reserved slice
that was already 80% utilized contributes new GDP only for actual throughput
above that allowance. Below the allowance it only makes existing production
visible. Background production is not erased when a government order pauses,
expires or is cancelled. Newly funded generation records its separate utilities
value added through the existing dispatch ledger; coal and power are not counted
twice. Payments, sales and resales never create another output award.

The opening 80% utilization and factory-equivalent conversion remain model
assumptions. Current GDP and inherited utilization can change during play; this
pilot does not replace the macroeconomy with twelve ambient shortage penalties.

## Capacity planning and opponents

The capacity plan separates installed factories, queued factories, active
domestic contracts, inventory and inbound imports. Finite contracts cover part
of the near-term gap without becoming permanent installed capacity or packs
already in a warehouse. Their reserved power is included in grid/generation
planning. Expired, cancelled and lost-province orders stop covering demand.

Economic Competition opponents can commission a feasible 30-day Materials order
against real uncovered demand before seeking more imported packs. They use the
same political price, input checks, ministry funds and commands as the player.
After a successful reservation the AI re-reads its investment plan to avoid
ordering a duplicate processor. It does not build capacity just because a
historical estimate exists. Blocked reservations remain commitments until the
deadline or cancellation; they do not become actual supply or achieved GDP.

Subsequent AI integration adds a separately bounded first-machine startup lot.
A supplied, powered inherited contract can replace the bootstrap processor;
current funding and both command prices are checked before pairing the order
and machine shop. Prospective inventory is not public recurring consumption.
See [MATERIALS_AI_INTEGRATION.md](MATERIALS_AI_INTEGRATION.md) for the decision,
warehouse/import handoff and current verification. The original pilot's
three-year census below remains a record of that earlier build.

## Screen and integration

Three large Capacity / Actual output / Demand cards lead the Materials panel.
Detailed inputs, money and GDP composition are behind expandable sections.
Expand, Upgrade, Import and Sell reuse the existing production and trade screens.
The Rust quote is authoritative; JavaScript does not calculate economic effects.
Quotes are pure reads, actor-bound and campaign-checked. Commands use the existing
once-only receipt mechanism, including lost-response retries and cancellation.

Optional `WorldState.materials` preserves default-world serialization when no
orders exist. Existing saves without inherited estimates are not backfilled.
Six initially unmapped countries retain national capacity but cannot place a
province order until real geography exists; no location is fabricated.

## Verification

Focused tests cover proportional throughput plus input and payment conservation, shared funded-plant/contract
power and grids, finite quantities and deadlines, small fractional orders,
capacity/import/stock separation, actor binding, once-only receipts, cancellation,
ownership changes, exact GDP overlap, and deterministic save continuity.

`materials_census` runs the actual daily AI economy without gifted assets, raw
inputs, treasury, political capital or an overridden player. It observes all
137 starting countries and reports real adoption and output by size. Multi-seed
results are invariant/coverage checks, **not a statistically calibrated growth
or adoption-rate promise**. The original pilot's three 1,096-day seeds passed
those checks but produced no natural AI Materials orders. See
[the original verification results](MATERIALS_OPERATIONS_RESULTS.md) and the
subsequent AI integration report linked above; do not confuse evidence from
different builds or treat three-year inactivity as permanent inability to trade.

Run from the repository with an existing output directory:

```sh
cargo run -p spheres-sim --release --example materials_census -- 1096 42 artifacts/materials-census-42.json
cargo run -p spheres-sim --release --example materials_profile -- 40 42 artifacts/materials-profile-42.json
```

The profile measures the actual daily system order and then checks its entire
result against ordinary daily ticks. It exposed an existing route-search cost
once many ministries enrolled. Spot clearing now shares at most 128 shortest-path
trees for identical source, policy and transit permissions during that one
clearing. Search order, route choice, and live capacity usage are unchanged;
there is no persistent/cross-day cache. The original pair search remains the
fallback and an exact route/complete-ledger regression oracle.

## Deliberately later

Other inherited manufacturing groups, private firm finances, wages/jobs,
household demand, economic taxes on firm profits, resource extraction beyond
the existing map, and expanded Welfare service effects remain separate slices.
This foundation supplies paid physical output and honest accounting for those
later systems; it does not pretend they already exist.
