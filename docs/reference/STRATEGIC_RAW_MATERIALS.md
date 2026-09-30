# Strategic Raw Materials

Status: implemented design and verification contract for the twelve-resource
strategic forecast, the recurring civilian trade intent, and their Supply
Command presentation. The dedicated strategic simulation, web, and browser
acceptance suites pass on the current shared tree. The 137-country and
multi-seed war censuses remain completion gates rather than claims in this
document.

## 1. Player promise and scope

Every country should understand the raw materials required by work it has
actually committed to, see danger before a line stalls, and react without
receiving free goods or an invisible national modifier. A shortage affects the
factory, project, Materials order, research program, energy action, or military
line that consumes the missing input. It does not directly reduce national GDP,
growth, stability, or welfare.

The feature uses the existing canonical `resources::ALL` order:

1. bauxite;
2. coal;
3. cobalt;
4. copper;
5. gas;
6. gold;
7. iron;
8. oil;
9. phosphate;
10. platinum group;
11. rare earths;
12. uranium.

No new commodity, district deposit, production figure, price, recipe, or
consumer is invented by this layer. A line with no current consumer is idle
with zero demand.

## 2. Authoritative ledgers

The forecast is a pure read over one economy. It is not a second inventory or
settlement system.

### 2.1 Physical non-oil goods

For the eleven non-oil lines, `resources::MarketState::stocks` is the
authoritative material already in each national warehouse after the market is
materialized. Before first settlement, `resources::stockpile` exposes the
legacy opening cover that the first physical ledger will materialize. A
`reserve_target` is sales policy, not inventory and not a storage cap.

The derived `resources::Have.flow` table is annual domestic production
capacity, including completed mines. It is not inventory. It becomes stock
only when `post_market_flows` posts the appropriate dated fraction.

`logistics::cargo` is paid, dispatched material in transit. It is not stock.
It becomes stock exactly once when the freight clock releases it on an open
route. Held or closed-route cargo is not secured coverage.

An active resource contract is a promise. `contract_fills` and shipment audits
describe dispatch from the latest settlement; they do not also grant an
arrival. Future service is secured only to the extent the projection can fund,
route, dispatch, and deliver it before the selected horizon ends.

Quotes, offers, unfilled orders, future mine output, and forecast production
that has not settled are not current inventory.

### 2.2 Consumption and blockers

The subsystem executing an action owns its consumption. A raw input bundle is
atomic: every required row is preflighted against one opening state; success
draws the complete bundle once; failure draws nothing and records the first
canonical missing commodity.

The red **STALLED** state requires an authoritative current/last-settled
consumer receipt. It may come from a manufacturing line, construction project,
mine installation, Materials order, commissioned civilian operation, or the
arsenal's structured `last_resource_stall`. A zero post-settlement warehouse is
not evidence of failure because a successful last-unit draw also ends at zero.
Monthly headline history is never used as a blocker ledger.

Opening Supply Command, computing a forecast, or refusing an action consumes
nothing and mutates no cash, stock, route, project, RNG, or cache persisted in
the save.

### 2.3 Money

Spot and negotiated trade use the existing resource-market price, contract,
treasury, debt, and commerce channels. Strategic AI must use the same command
price and affordability checks as the player. It may not grant cash, bypass a
spending cap, charge a buyer twice, or discard the seller's receipt.

### 2.4 Oil exception

Oil remains the existing priced national flow. The generic raw warehouse,
eleven-line physical spot matcher, and atomic raw bundle deliberately exclude
it; an oil contract settles through the existing oil-value path.

Supply Command therefore shows oil as informational market context. Its
physical demand, covered, gap, stock, outbound-claim, domestic-output,
contracted-inbound, and paid-inbound fields are `null` at the web boundary. The
layer must not create an oil `Stock`, clear oil through the generic raw market,
consume it through a non-oil recipe, or add a second price/GDP/blockade effect.

## 3. The 30/90/365 forecast contract

The required horizons are exactly 30-day **RUN**, 90-day **PLAN**, and 365-day
**WATCH**. Values remain in each commodity table's native physical unit inside
the simulation; the web adapter converts every value on one row by the same
declared stock-unit factor.

### 3.1 Phase-aware dates

Each horizon contains the next `H` unsettled resource dates as a half-open
interval `[S, S + H)`:

```text
S = today + 1  when resources.last_tick_day == today
    today      otherwise
```

This makes an AI review after today's systems and the player read immediately
after `clock::advance_date` describe the same absolute dates. A due date equal
to `S + H - 1` is inside; `S + H` is outside. Materials start/deadline,
contract-expiry, legacy-cargo, month-length, leap-day, project-workforce, and
fiscal-year calculations must use that same interval.

### 3.2 Demand

For non-oil commodity `c` and horizon `H`:

```text
demand(c,H)
  = civilian_operating_daily(c) * authorized_days(H)
  + military_policy_monthly(c) * authorized_days(H) * 12 / 365
  + sum(project paced draw(c,H))
  + sum(mine-installation paced draw(c,H))
  + Materials scheduled draw(c,[S,S+H))

authorized_days(H)
  = funded_days(H) for a department-budget-enrolled government
    H              for legacy non-enrolled military procurement

project paced draw(c,H)
  = min(project remaining input(c), next standing daily draw(c) * funded_days(H))

mine paced draw(c,H)
  = min(mine remaining input(c), next standing daily draw(c) * funded_days(H))

funded_days(H)
  = days in intersection([S,S+H), authorized fiscal year)
```

The finite rows expose their full remaining bill separately, but a 30-day card
does not pretend that the whole remaining project is due in 30 days. Each
project is capped individually before projects are summed. Standing department
authority is the applicable annual allocation times current GDP and department
share, divided by 365 and keyed to the fiscal year containing `S`. Each
horizon multiplies by only the dates where that authorization is still live;
it never carries an unrenewed December plan into January. This normalized
pacing deliberately does not dump a banked cash balance into one forecast day.
Projects share that authority in priority then stable-id order; mines see only
the Industry & Energy minerals authority left by earlier projects. The same
national construction-work pool is shared by projects and mines. Ownership
loss, contest, expiry, completion, unavailable authority, and the Starter
Industry calendar cap can reduce the next standing draw to zero.

Materials demand intersects `[S,S+H)` with each active order's
`[start_day, deadline_day)` window, then caps output by remaining quantity,
reserved daily throughput, and located inherited capacity before applying the
processing recipe once. This is separate from the ordinary modeled processing
plants' recurring draw and is not counted twice.

Military demand is a standing policy-month recipe without an old blocked cash
bank; the arsenal remains authoritative for actual daily execution. Enrolled
civilian plants and enrolled automatic or directed military production stop at
the enacted program's fiscal boundary. Legacy non-enrolled military policy
continues across the full horizon. Directed manufacturing replaces the legacy
automatic military pick rather than adding another copy of it.

### 3.3 Supply and prior outbound claims

Let:

- `W(c)` be current physical warehouse stock;
- `P(c,H)` be domestic production posted over each actual date in the window;
- `F(c,H)` be already-paid, route-executable cargo arriving in the window;
- `C(c,H)` be not-yet-dispatched contract service that can arrive in the
  window; and
- `E(c,H)` be executable outbound contract dispatches in the window.

The authoritative total is:

```text
gross_pool(c,H) = W(c) + P(c,H) + F(c,H) + C(c,H)
prior_claims(c,H) = min(E(c,H), gross_pool(c,H))
coverage(c,H) = max(0, gross_pool(c,H) - E(c,H))
gap(c,H) = max(0, demand(c,H) - coverage(c,H))
```

The contract projector must model one combined timed reservoir. On every
unsettled date it:

1. credits executable freight arrivals due that date;
2. posts that date's domestic production fraction;
3. evaluates all still-live contract promises against one seller/commodity
   pool;
4. for physical logistics, applies one atomic service fraction to every
   non-oil physical leg and shares route capacity in stable contract-id order;
5. deducts each dispatch once; and
6. schedules physical receipt after route travel time.

Without physical logistics, actual settlement rations each seller/commodity
pool against its open promises from one opening snapshot; it does not invent a
physical bundle-capacity rule. Those abstract receipts become re-exportable on
the following date because all transfers are applied only after the opening
claims are computed. Physical goods become re-exportable on their due date
because freight arrivals settle before that date's contract dispatch. Existing
cargo can fund a later re-export but remains reported only as paid inbound; it
must not also appear as new contract service.

The four UI source rows are retained supply, not four unrelated gross claims.
Outbound dispatch is allocated deterministically across warehouse stock,
domestic output, paid inbound, then projected contract inbound, and:

```text
retained_stock + retained_domestic + retained_paid_inbound
  + retained_contract_inbound == coverage
```

The full warehouse and total prior claims remain visible as context. Gross
source totals and outbound totals are cumulative, but net coverage itself need
not be monotonic when a later contract dispatch consumes more than the later
window adds. Demand is nondecreasing across 30/90/365 for nonnegative committed
components; the gap need not scale linearly.

### 3.4 Interpretation

- **RUN (30):** the only horizon that sizes an ordinary replenishment intent.
- **PLAN (90):** warning and preparation; it does not buy three RUN windows.
- **WATCH (365):** structural exposure; it does not authorize a year's hoard.

The forecast is an aggregate secured-supply view, not a promise that every
consumer completes. A named actual blocker takes precedence over prospective
color. Red means a recorded stall; an uncovered but not-yet-blocked RUN gap is
amber/action; later gaps are watch; no committed use is idle.

## 4. AI action order

The recurring raw purchase review remains monthly in daily play. It runs after
the ordinary resource market and current arsenal attempt have settled, and
before the later economic-AI mine fallback.

Within one resource purchase wave:

1. the existing military cover pass runs first in its existing stable order;
2. one immutable cold-cache-safe `Have` plus one world-wide 30/90/365 contract
   projection is built after that pass;
3. every living, enrolled, non-player government is reviewed in canonical
   nation order; and
4. each government may create at most one additional civilian recurring raw
   intent, selecting the first canonical commodity with an uncovered RUN rate.

Civilian sizing uses only the uncovered installed civilian recurring rate over
the funded dates inside RUN. It reserves earlier project and mine slices
because those execute before installed civilian plants; later Materials and
military consumers do not hide or inflate the civilian plant's own shortage.
The resulting 30-day gap is converted to one policy-month rate. A finite
project, mine, or Materials bill never becomes a 36-month recurring commodity
leg.

The seller list contains living producers with a currently executable route
when physical logistics is enabled. The existing negotiation/evaluation path
may accept, counter, price out, or hard-refuse. A signed contract or an offer to
the player ends that line's search. A pending same-buyer/same-commodity player
offer prevents a duplicate offer or a second AI contract for that remedy.

Economic AI considers a mapped domestic mine only later, and only when all of
these are true:

- today's ordinary raw market actually cleared;
- an automatic consumer has an immediate shortage and RUN remains uncovered;
- no currently executable new inbound contract or still-valid pending player
  answer is the current remedy;
- no active sponsored mine already exists;
- there is an eligible owned, uncontested mapped deposit; and
- no currently reachable peaceful supplier can cover the line, or the full
  existing universal-refusal predicate has been met.

A no-route world or a commodity with no foreign producer may justify domestic
mining; it does not synthesize refusal evidence and cannot by itself justify
war. If neither peaceful supply nor an eligible mine is executable, the exact
consumer slows to the fraction its tightest input can support, or pauses at zero
without spending. There is no ambient shortage tax and no penalty to unrelated
economic activity.

All iteration and tie-breaking is deterministic. A multi-country AI wave may
reuse one immutable global supply context, but it may not rerun the expensive
365-day world contract projection once per country.

## 5. Trade before force; refusal before war

The existing refusal memory is authoritative and must not be weakened.

These are not hard refusals:

- an unmet spot order;
- an unaffordable quote or `NotForThatPrice`;
- no physical route or exhausted freight capacity;
- no producer in the world;
- the buyer declining or lacking standing to ask;
- a buyer-created sanction; or
- any forecast shortage.

`NotForThatPrice` is only a re-ask clock and `Reason::is_refusal()` remains
false. A hard refusal is written only by the existing negotiated `ask` and
seller evaluation path. Signing supply forgets the answered refusal. Merely
making an offer does not erase it. Cooling stays on the existing once-per-month
twenty-four-step lattice.

`resources::refused_all` remains true only when every counted living foreign
producer has the required twice-asked hard-refusal heat and the buyer did not
close those sellers with its own sanctions.

Resource scarcity can reach the existing `dyads::last_resort` predicate only
when the relevant action is currently stalled, universal refusal evidence is
present, the target is a counted refuser that still produces the line, the pair
was not already settled by force, and the target holds reachable mapped ground
for that commodity. Statecraft still owns quarrels, costs, pacts, deterrence,
validated aims, and declarations. The resource manager never creates a
refusal, aim, or war directly.

## 6. Save, replay, and read compatibility

The player/API forecast is recomputed live on every read and never persists as
a second economy. `NationPlan.raw_supply_review` may persist the forecast used
to explain an AI government's latest strategic review, like the existing goods
`supply_review`; it is audit history, not input to future settlement or the
player's current screen. Passive/default/player-only reads write no snapshot.

Every new persisted optional receipt or snapshot uses `serde(default)` and an
empty/`None` skip where compatible. Sparse maps and rows retain deterministic
ordering. Old saves receive no stock, demand, action, cash, or output gift.

Acceptance requires:

- feature-off and player-controlled paths remain byte-inert;
- pure reads preserve serialized bytes and RNG state, including a cold cache;
- save/load continuation preserves exact bytes/RNG under the same command
  schedule;
- daily stepping and monthly batching agree at settlement boundaries;
- a read immediately after settlement and the same state immediately after
  date advance describe the same next unsettled dates; and
- ownership, recipe, contract, route, cargo, and completion changes cannot
  leave a serialized read cache authoritative.

## 7. UI/API contract

Supply Command uses two obvious full-page screens rather than one dense
dashboard. The overview leads with one server-owned mission: **Protect the next
90 days — keep every active production and construction line supplied.** Its
progress is a count of active storable materials, never a sum of unlike physical
units. Oil and idle materials remain available to inspect but do not pad the
score. Zero active lines is an idle/onboarding state, not a false victory.

The overview keeps twelve large selectable commodity buttons in canonical
order. Its single primary action opens the highest-ranked material brief
(`stalled`, then 30-day `action`, then 90-day `watch`); it never executes Mine,
Trade, or Take. Secondary risks are folded. Selecting any commodity opens a
separate full-page action screen with a clear route back to the overview, the
served need/coverage/gap, and three large function cards in peaceful-first
order: Trade, Mine/Develop, then the gated Take route. Their existing costs,
eligibility, refusal evidence, and commands remain
authoritative.

The player may change the exploratory horizon between 30/90/365 days. The
mission remains server-ranked at 90 days and exposes at most three ranked
risks; changing the selected card horizon does not silently rerank it.

The server owns arithmetic, ordering, status, units, and blocker truth.
JavaScript presents those values and does not recreate coefficients or infer a
red state from a legacy row. Red/stalled is emitted only when `blocked_now` is
true. A prospective RUN gap is action/amber. Oil's physical values are `null`,
not synthetic zeroes. Missing modeled data is explicit.

## 8. Focused acceptance gates

The dedicated suite must cover these deterministic cases:

1. All twelve rows appear once in canonical order; unused rows are idle.
2. Cold and warm forecasts are equal and byte-pure.
3. Recurring demand scales over 30/90/365; every finite project/mine bill is
   capped individually and shares authority/workforce in actual priority.
4. Materials start/deadline and cargo due dates obey `[S,S+H)` exactly before
   and after clock advance, including month, leap, and Dec-31 boundaries.
5. A completion or late project/mine start cannot make the same next-day
   forecast reuse today's frozen workforce or expired department authority.
6. Two contracts share one seller/commodity pile; physical multi-leg bundles
   are atomic; shared route capacity is reserved once in stable contract-id
   order. Abstract transfers retain opening-snapshot rationing and the one-day
   re-export lag.
7. In B -> A -> C fixtures, A can re-export physical supply no earlier than its
   due date and abstract supply no earlier than the next settlement; inbound,
   outbound, retained source rows, and stock conserve exactly.
8. Existing paid cargo and new contract service are counted once. Held,
   closed-route, out-of-window, expired, dead-party, quote, and offer quantities
   are zero secured supply.
9. Retained source rows sum to coverage for every non-oil row/horizon.
10. Failed atomic consumption changes no row; successful last-unit use ending
    at zero is not red. A day-one arsenal block cannot survive a successful
    day-two structured receipt merely because its headline remains in history.
11. Civilian contract selection and sizing use the same uncovered recurring
    RUN gap; finite demand, later-priority demand, and player offers cannot
    create duplicate or oversized recurring contracts.
12. A reachable supplier or executable inbound remedy prevents a mine. No
    route creates neither an offer nor a refusal; a legitimate domestic mine is
    allowed without weakening the war gate.
13. Deficit, price, route failure, one refusal, an unasked producer, or a
    buyer-created embargo never opens resource last resort.
14. Oil produces no physical strategic quantity or second economic effect.
15. One global AI wave builds one world contract projection, not one per
    country.
16. Feature-off, player, save/load, daily/batch, and RNG continuation remain
    exact.

Focused commands:

```text
cargo test -p spheres-sim --test strategic_raw_supply -- --nocapture
cargo test -p spheres-sim --test economic_ai_supply -- --nocapture
cargo test -p spheres-sim --lib resources::tests:: -- --nocapture
cargo test -p spheres-web strategic_supply -- --nocapture
node tools/ui/check_discovery_arcade.cjs
cargo check -p spheres-sim -p spheres-web
```

The existing refusal/war regressions remain mandatory, especially
`the_last_resort_predicate`, `a_deficit_alone_never_raises_appetite`,
`priced_out_is_not_a_refusal`,
`a_starved_ai_asks_before_it_wants_and_the_universal_refusal_is_headlined_once`,
`cooling_is_on_the_lattice`, and
`daily_play_cools_a_refusal_once_a_month_not_once_a_day`.

## 9. 137-country census gate

Use the observer discipline in `spheres-sim/examples/materials_census.rs`: start
the unmodified 137-country daily world, observe rather than steer, check finite
balances daily, preserve GDP/province/sector reconciliation, and prove an exact
save/resume plus RNG continuation. A strategic extension records bounded
country/commodity totals for:

- reviews and decisions by kind;
- 30/90/365 need, retained supply, prior claims, and gaps;
- production, consumption, dispatch, arrival, pending cargo, and closing stock;
- actual blocked consumer-days and normalized reasons;
- spot attempts/fills, negotiated asks/contracts, prices, and hard refusals;
- mine starts/completions and the evidence present at each start; and
- last-resort candidates, aims, and resource-war headlines with causal
  evidence.

Hard census invariants:

- exactly 137 original countries and twelve canonical rows per reviewed state;
- every financial/physical value is finite and every physical quantity is
  nonnegative;
- for every non-oil nation/commodity interval, closing stock reconciles opening
  stock plus posted production and arrivals minus dispatches and actual
  consumption within the declared rounding lattice;
- every source decomposition reconciles its coverage and no re-export creates a
  unit;
- no action occurs without its executable trigger, no duplicate action occurs
  in one review, and no civilian recurring contract exceeds its RUN-rate cap;
- no mine begins while a sufficient reachable remedy is active;
- no resource aim/war exists without all refusal/last-resort clauses;
- player/read paths consume no action or RNG; and
- existing GDP/province/sector and exact save/resume invariants remain green.

Development runs are 1,096 days on seeds 42, 7, and 1990. The integration run
is 2,192 days on seed 42. Freeze and report the executable hash, seed, switches,
elapsed time, and unique artifact path. Compare new counts with the prior
Materials/economic-competition census artifacts descriptively for churn,
hoarding, blocked-country concentration, small-country exclusion, and mine or
import cascades; do not invent a balance threshold after seeing the results.

Finish with the existing independent resource-war instrument: a 20-seed smoke,
then 200 seeds through 480 months. Report the clause funnel and frequency. The
historical zero-war sample is context, not a requirement that the new frequency
remain zero; the hard bar is the complete causal chain.

## 10. Definition of done

The focused implementation is ready for player review only when all focused
suites pass on the final shared tree, phase/source/atomic invariants have direct
regressions, feature-off and player paths remain inert, and Supply Command shows
all three horizons without returning the base screen to a dense spreadsheet.
The strategic layer is complete only after the stated 137-country and
multi-seed war census gates also pass and their immutable artifacts are kept.
