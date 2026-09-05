# Build what the country needs

Local implementation, 2026-09-04. Not pushed to GitHub.

The subsequent inherited-sector association is documented in
[STARTING_INDUSTRY.md](STARTING_INDUSTRY.md). Its annual value-added estimates stay
separate from the usable physical packs described here.
The Materials operating pilot in [MATERIALS_OPERATIONS.md](MATERIALS_OPERATIONS.md)
adds finite, paid domestic commitments without converting them into free stock
or permanent factory capacity.
The follow-up in [MATERIALS_AI_INTEGRATION.md](MATERIALS_AI_INTEGRATION.md) adds
a bounded, physically backed first-machine startup intention. Its 15-pack
inventory target is AI-only: it is not new public recurring demand or an
automatic raw purchase. Both proposed power loads are checked together.

The civilian investment planner and Exchange now read the same pure national
capacity plan in `spheres-sim/src/industry_planning.rs`. The purpose is to stop
repeating a factory purchase when its output is already covered elsewhere.

## What is connected

- All legally owned provinces, including acquired factories and fractional
  workshops, contribute their actual installed capacity. Occupied capacity is
  still counted as owned, not mistaken for a factory that never existed.
- Owned, sponsored construction reserves its future capacity immediately.
  Pending automation is included once. Pending output is not usable inventory.
- Processing supplies intermediates; machinery consumes them and produces
  capital goods. Full downstream stock suppresses upstream expansion pressure.
- Current funded construction requests, useful research services, and actual
  delivered exports establish demand. Military raw-input procurement is not
  mislabelled as demand for manufactured packs.
- Existing inventory and paid inbound shipments count before new factory orders.
  Inbound packs remain unusable until their normal physical arrival.
- Active inherited Materials contracts cover near-term demand separately from
  installed/queued factories and imports. Their achievable remaining quantity
  respects current capacity and the finite deadline; expiry, cancellation and
  legal ownership loss release that commitment. Their power demand also enters
  national generation and local-grid planning.
- National generation and local grids are checked against the chosen factory,
  including already committed utility projects. Empty industrial estates do
  not automatically receive a full grid.

## Explicit game-planning policy

The target is current daily demand plus 25% spare capacity. Installed and queued
output are subtracted from that target. More than 90 days of stock plus inbound
supply pauses expansion. A full-size extra line must fit the remaining gap;
small processing gaps can use a paid, proportionally sized workshop package.
These are forecast rules, not extra resource consumption or fiscal multipliers.

Bare economies retain their first paid industrial bootstrap. One first machinery
shop may establish a capital-goods source, preserving the raw-only construction
path; it is not repeated after machinery exists or is queued, or when capital
inventory already covers use. Existing paid projects are retained. Their costs,
size, material obligations and progress are not rewritten by a new forecast.

Storage expansion requires evidenced turnover demand, not merely unsold stock.
Automation must fit the demand gap for every output it expands. Operating input
and funding problems must not be interpreted as a missing factory. All new
projects still pass ordinary political, budget, technology and resource checks.

## Export evidence and old saves

The manufactured trade ledger records good, quantity, contract and actual
delivery date, only when the buyer's warehouse receives cargo. Recent evidence
uses a trailing 90-day window; records are retained for 365 days. Asking to sell,
signing a contract, dispatching cargo and inflating a price do not create demand.

Old saves need no invented history. For an old contract accepted within the last
90 days, its delivered quantity is conservative evidence: its deliveries must
have happened since acceptance. Dated quantities are subtracted from that
fallback so nothing is counted twice. Older undated deliveries cannot be assigned
a trustworthy date and are not used. The optional new ledger is absent when
empty, preserving untouched default-world serialization.

## Player experience and limits

Exchange → Your industry → **Build what you need** shows Materials and Machinery:
installed output/day, queued extra output/day, current demand/day, and plain
advice. Stock, imports and supporting capacity sit behind expandable details.
This is advisory for players; manual construction remains available through the
same commands. AI governments use the plan for their existing 30-day reviews;
physical work, trade and production still run daily.

This counts tracked physical assets, not a historical census of every factory.
The inherited eight-sector GDP estimate is not silently turned into factories,
free stock or new demand. No starting data, GDP formula, production recipe,
appropriation rate, political price or calibrated legacy threshold was changed.

## Verification

Three regression cases were observed failing on the original implementation:
buying an already supplied fractional grid, overlooking a factory in another
owned province, and buying a warehouse for full unsold inventories. The shared
plan makes those cases pass. A fourth red test during integration caught an
empty alphabetical site being preferred over an already powered site; selection
now searches for ready sites before commissioning utility support. Additional
invariants cover pending workshop and
automated capacity, real expansion demand, stock and inbound supply, ownership
changes, dated exports, old-ledger fallback, read-only save continuity, and
all 137 starting countries without invented factories. Player override, fresh
versus stale input blockers, and twelve repeated reviews with unsold inventories
are also checked. All 13 dedicated planning regressions pass, alongside all 18
existing economic-AI tests and all nine small-country integration tests. The
serverless UI suite passes 163 tests.

The assembled release workspace suite finished with **648 passed, 3 known
baseline failures, 55 ignored**. The two golden actuals remain
`0xe26e4bf8d6c60066` and `0xbe94d6125631829c`; the existing Belgium endowment
growth failure also reproduces unchanged. No threshold or golden was repinned.
Evidence: `artifacts/capacity-planning-workspace-tests-v1.log` in the parent
workspace. Both disposable browser suites passed at desktop/tablet/mobile
widths: readable cards, no horizontal overflow, correct queued fractional
capacity, pure reads, exactly-once order retry, and no page errors.

Review preview: run `cargo run -p spheres-web`, then open Exchange → Your industry. It uses the
isolated `target/capacity-preview-7795` campaign; earlier user servers and saves
were not touched. The preview currently contains the disposable Tonga workshop
order used to verify the capacity association.

This is structural correctness evidence, not a new long-run balance census.
The previous 450-day results in `SMALL_COUNTRY_MODULES.md` describe that earlier
build and are not relabelled as a measurement of this planner.
