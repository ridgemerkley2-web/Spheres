# Physical logistics: first playable layer

Physical logistics extends the existing resource market. It does not introduce
a second warehouse, treasury, manufacturing queue, or combat supply model.
Browser games enable it; default/headless games keep the previous behavior.

## Player loop

1. Buy resources or negotiate a contract through the existing trade system.
2. Open **Logistics / World in Motion** to inspect the loaded cargo, booked
   route, capacity bottleneck, and arrival settlement.
3. Choose **Fastest open route**, **Keep it on land**, or **Go the long way**
   (avoid named sea chokepoints when an alternative exists). The setting is
   free and applies to future imports, not already-booked cargo.
4. Build province Infrastructure through the existing Production board to
   improve the applicable freight links. Existing construction costs, material
   requirements, ownership checks, and completion time still apply.
5. Cargo becomes usable only when it reaches the national warehouse. A closed
   route holds its cargo; it does not destroy it or create another bill.

The map stays the main screen. The logistics panel carries the details, with
short summaries and expandable consignments instead of a permanent ledger.

## What the network represents

The baked graph follows the game's district adjacency, coast polygons, and
modeled ocean waypoints. Distances are baked integers. The source, generator,
and limitations are documented under `tools/logistics`.

District centroids are national freight entry/exit points, not actual mines or
factory doors. Any currently owned and accessible district can serve its
nation's common warehouse. Coastal gateways are modeled connections, **not a
historical port census**. Ocean paths are strategic-scale, not navigation charts.
Road classes, railway gauges, actual port tonnage, pipelines, and individual
vehicles are not claimed or simulated.

The route graph uses current district ownership and wartime control. Civilian
transit through a third country requires an open relationship with both
endpoints: sanctions or belligerence close it. Military access treaties are not
silently repurposed as commercial transit rights.

All commodities share each link's capacity, measured in modeled tonnes. The
reference capacities are monthly; daily games reserve one actual calendar
day's share, so changing the ticker does not multiply transport throughput.
Native resource quantities retain their existing units. Conversion includes
kilotonnes, kilograms, and a stated game-scale gas freight factor. Oil remains
on its existing national oil/financial channel; this build does not add a
second oil stockpile.

## Settlement and accounting

**Daily-clock amendment, 2026-09-03:** on Ridge's request to put everything on
the daily ticker, new daily games settle freight and resource trade every day.
Legacy/headless worlds without `daily_simulation` retain the prior monthly
audit path. The same settlement owns both cadences:

1. Clear the previous tick's transport reservations and settle eligible
   arrivals exactly once.
2. Add domestic production to the existing resource warehouse.
3. Reserve contract freight before spot freight. Seller stock availability is
   still allocated pro rata across promises.
4. Scale every resource leg of a negotiated bundle to one common service
   fraction, accounting for shared transport links. Scale its recurring cash
   legs to that same fraction.
5. Remove dispatched goods from the seller, record the cargo in transit, and
   settle the payment through the existing resource-market finance path and
   `economy::charge`.
6. Spot clearing counts pending imports against its purchase target, preventing
   it from buying the same reserve again while that reserve is still traveling.
7. Manufacturing, arsenal procurement, and construction can consume only
   on-hand stock. Inbound cargo is never spendable inventory.

Daily cargo carries its actual dispatch day and due day. The due day is the
dispatch day plus the route's modeled travel days; arriving cargo is checked
on that simulation date. A closure holds paid goods until a later open day.
Retries on the same day cannot reset capacity, deliver twice, or charge twice.
Months still describe contract terms and annual budgets, not posting frequency.

Production uses annual output divided by twelve and by the current month's
actual number of days. Contract goods, payments and foreign-purchase ceilings
use that same daily fraction. Price inertia is converted to its daily retention
factor rather than applying a whole month's price movement each day. Reserve
targets remain months of need; only actual daily actions and payments are sliced.

Old saves omit the new day clocks. Unexpired mine, procurement and contract
terms acquire a remaining-day clock without replacing their stocks or cash.
Old freight preserves the known end-of-arrival-month boundary; its historical
dispatch day stays unknown rather than inventing one. All original monthly
fields remain readable for compatibility. Latest market records also store the
month length used for the posting, so a January 31 flow is not reinterpreted
using February's shorter month.

Conservation is seller stock + buyer stock + cargo in transit, apart from the
existing domestic production and legitimate resource consumption. Delivery is
not a second purchase. Expiring or canceling a contract affects future promises,
not cargo already loaded. Closed routes and dead endpoints retain cargo with a
reason, including across save/load. Cargo can resume when its booked route is
open; automatic in-flight rerouting is deferred.

## Integration ownership

| Owner | Responsibility |
|---|---|
| `logistics.rs` | Route search, shared link reservations, policies, persistent cargo, arrival/hold decisions |
| `resources.rs` | Warehouse quantities, actual trade quantities, contracts, market prices, purchase ceilings and payments |
| `production.rs` | Completed province infrastructure and the cost/time/materials required to build it |
| `manufacturing.rs` / `arsenal.rs` | Request and consume usable resources through their existing paths |
| `economy.rs` | The one cash/debt settlement channel; no logistics-specific GDP bonus or shortage tax |
| `spheres-web` | Display authoritative routes and cargo; send the routing-policy command |

`GameRules.physical_logistics` requires the resource gates, resource market,
and logistics-audit flags. It defaults to false and is omitted from default
saves. Empty `WorldState.logistics` is also omitted. The derived route cache is
not saved and cannot affect deterministic continuation. Existing abstract
shipment audits remain readable.

## Deliberate boundaries for later systems

- Military supply and reinforcement delivery are not routed through this graph
  yet. No new combat modifier is added.
- There are no fleet counters, submarine interception, cargo insurance,
  transport fees, player-built ports/railways, or custom maritime blockades.
- A named chokepoint is a route feature/avoidance choice, not a new conquerable
  political entity. Existing war control and sanctions govern closures.
- One preferred route per buyer/seller pair is used in a settlement; the model
  does not split an order across an optimized set of alternative corridors.
- Market quote formation keeps its existing bilateral-access and supply/demand
  rules. Freight capacity changes executable quantities, not a separate price
  or macroeconomic calibration.
- Throughput and gas-density factors are game tuning, not historical facts.
  Load balancing, freight-market pricing, and a broad calibration census can
  be layered on this foundation without duplicating the core ledgers.

## Verification

The focused `logistics::tests` and `resources::tests::freight_*` suites cover
route policies, shared capacity, whole-bundle fulfillment and payment,
arrival conservation, sanctions, expired contracts, pending-order deduplication,
and deterministic daily/monthly and save/load continuation. Web tests cover the
served manifest, policy command, and board wiring. Default-world golden pins
are not changed as part of this work.
