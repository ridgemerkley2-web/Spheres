# AI industrial supply manager

Local-review design and implementation note, 2026-09-04. This layer extends
the opt-in, daily Economic Competition model. It does not authorize a commit or
push, grant starting goods, add household consumption, or let the AI direct the
player's country.

## The policy

Industrial governments review strategy every 30 days while construction,
production, freight, markets and fiscal settlement continue daily. After any
required fiscal action, the supply manager asks whether real goods can cover the
next 90 days of evidenced industrial use. It manages the two manufactured-goods
ledgers already in the game:

- **Materials / intermediates** feed machinery, projects and prototype research.
- **Machinery / capital goods** feed projects and prototype research.

The normal order is: use owned stock; recognize already-paid incoming supply;
recognize finite domestic Materials contracts; credit only recent actual domestic
output; commission inherited domestic Materials capacity when it is physically
and fiscally executable; buy from a consenting reachable foreign seller; then
allow the existing industrial planner to commission a processor when the supply
route cannot cover the need. A successful foreign manufactured-goods purchase
ends that review, so a processor chosen from the pre-purchase state cannot also
start from a stale plan. Existing projects keep moving between reviews.

A domestic Materials order is finite and may accompany the backed first-machine
bootstrap. It still requires real raw inputs, shared power, department funds,
cash and political capital through the ordinary command path. Signing either a
domestic order or an import produces no goods; packs become usable only as daily
work or freight actually delivers them.

## The 90-day forecast

`supply_forecast` is a pure read. For each good it calculates:

```text
project remaining = existing 30-day demand - (recurring daily use × 30)
90-day target      = project remaining
                   + (recurring daily use × 90)
                   + startup reserve

coverage           = stock
                   + paid imports/cargo still incoming
                   + finite domestic Materials still contracted
                   + projected recent actual domestic output

shortage           = max(0, target - coverage)
```

Unfinished construction is therefore counted once, not multiplied into three
months of recurring demand. Recurring use includes installed machinery's
Materials rate and current prototype-research demand. The startup reserve adds
15 Materials packs for each queued Machinery Works and one review-period of
prototype inputs for each queued Research Center. An enrolled non-player
government with a controlled full-size estate can also show one prospective
15-pack first-machine intention before construction is queued. That intention
is AI planning only: it never enters public commerce demand, pulls raw goods
automatically, or applies to the player's country.

Domestic output counts only when the latest industry settlement is today or
yesterday and the operation reported positive output: Processing Plants and
starter workshops cover Materials, while Machinery Works covers capital goods.
Unfinished, blocked or estimated capacity is never called supply. The projected
run rate is capped at the target. Stock, incoming lots and contracts remain
separate fields so the UI never implies that undelivered goods are on hand.

Each line also reports storage capacity, current headroom and one of three plain
states: `idle`, `covered`, or `replenish`, with the measured reason attached.

## Bounded action size

The 90-day horizon is an early warning, not permission to buy a 90-day pile.
One replenishment action is limited to:

```text
min(
  90-day shortage,
  project remaining + startup reserve + recurring use for 30 days,
  storage capacity - stock - paid imports - domestic contracts
)
```

Foreign purchases are additionally limited to 0.1% of current annual GDP for
that review, the actual quoted quantity and price, and available treasury cash.
They use a 30-day delivery window and do not borrow automatically. The manager
checks Materials before Machinery and can place at most one foreign manufactured
lot in a review.

A finite domestic Materials order is also capped to the whole raw bundle the
government owns after subtracting every active contract's remaining claim. A
one-day operating quote is used only as a flow limit; it is never multiplied by
30 and mistaken for 30 days of ore. Later loss of territory, power or operating
authority can still interrupt the contract, but the AI does not knowingly sign
an order whose full inputs are absent on signing.

## Processing, mines, storage and exports

- **Processor:** the existing capacity planner remains responsible for selecting
  a Processing Plant or proportional starter workshop. The supply action runs
  before that candidate is executed, and an accepted import forces a later
  re-plan instead of an import-plus-stale-processor double action. Any positive
  executable first-machine quote counts, even when one seller cannot fill all
  15 packs; the AI pays for that partial lot and defers the processor until the
  reachable import route is exhausted.
- **Mine:** mine development is a last fallback for an ordinary raw-input bundle,
  not for the manual inherited-Materials reservation. The raw market must have
  cleared on the current date, the country must still lack a required commodity,
  and no raw shipment may already be pending. The AI then chooses an eligible
  mapped, controlled deposit through the priced mine command. It keeps at most
  one mine project per country. While a proportional starter workshop is the
  active build target, its deficit cannot quietly commission a much larger mine
  behind it.
- **Warehouse:** storage expands only for evidenced turnover. Existing plus
  queued storage must be nearly full (at least 90% covered by stock and incoming
  goods), daily demand must be positive, and the 90-day turnover requirement
  must exceed that storage. A full pile with no consumer is a sell-or-idle signal,
  not a reason to build another warehouse.
- **Exports:** AI sale policies protect unfinished project goods,
  queued-project startup reserve and 30 days of recurring use. Once paid
  imports, a finite domestic contract, owned stock or recent actual production
  begins filling the first-machine plan, one bounded 15-pack starter lot is also
  protected. That latch prevents partial lots being resold between reviews;
  everything above it remains tradable, so the 90-day forecast is never
  hoarded. Surplus consent uses the normal sale-policy command with a 1.05 ask
  multiplier, and a stale cached reserve is repaired from the actual live
  policy. Player sale policies are untouched.

## Exchange visibility

The Industry view receives the player's live Rust forecast and renders two large
Materials/Machinery cards. Each leads with **Need**, **Covered** and **Gap**, then
offers the exact stock, incoming, contract, recent-output, project, startup and
storage accounting in a disclosure. JavaScript formats the supplied values; it
does not recreate the economic formulas.

The World view uses each AI government's saved `supply_review`, together with
its decision reason, next industrial target and funding outlook. The snapshot is
stored when the latest action is recorded, after that action has changed the
ledger, so a newly accepted inbound lot is visible without calculating 137 live
plans during an API read. Its date remains explicit. At narrow widths the world
table becomes country cards, and the supply totals stack on phone-sized screens.

## Determinism and safety boundaries

Forecasting does not mutate the world or RNG. Manufactured goods are evaluated
in fixed Materials-then-Machinery order; existing sorted nation and district
ordering supplies deterministic tie-breaks. Purchases, policies, projects,
domestic orders and mines all use ordinary game commands, so normal ownership,
funding, stock, political, logistics and refusal checks remain authoritative.

Dedicated regressions cover exact 90-day accounting, once-only netting of stock
and both kinds of incoming supply, domestic priority, import re-planning, raw
market-before-mine behavior, demand-gated storage, the 30-day export reserve,
save/replay across freight and review boundaries, and byte-inert player/default
paths. A legacy world does not acquire a serialized `supply_review`, and calling
the forecast alone never opts a country into the system.

The final seed-42, six-year, 137-country census and the two defects it exposed
and verified are recorded in `AI_INDUSTRIAL_SUPPLY_RESULTS.md`. All conservation,
save/resume and RNG-continuation invariants passed. This is descriptive evidence
for the reviewed seed, not a multi-seed calibration claim.

## Honest limits

- This is a supply policy for Materials and Machinery, not a general forecast
  for the twelve raw commodities, food, labor, consumer markets or inflation.
- Recent output is a simple observed run-rate, not a guaranteed reservation.
  Later funding, power, inputs, war, ownership or storage can stop it.
- The forecast counts the remaining quantity of paid contracts and cargo; it
  does not time-slice a long user-created delivery schedule inside the horizon.
- One review chooses a bounded lot rather than globally optimizing suppliers,
  future prices, lead times or inventories across countries. A failed domestic
  Materials attempt can remain the visible blocker until a later review.
- Mine fallback reacts to today's actually failed raw order. It does not predict
  a strategic 90-day raw stockpile or prove that a mine is the cheapest policy.
- The 15-pack machinery startup reserve, 30/90-day horizons, 0.1%-of-GDP import
  cap, 90% storage threshold and 1.05 export ask are explicit game policies, not
  sourced 1990 statistics or calibrated claims of optimal national behavior.
- UI snapshots explain the model's recorded accounting and choice. They do not
  claim that a signed order is delivered output, that capacity is production,
  or that this layer by itself improves GDP, welfare or adoption across seeds.
