# SPHERES Manufacturing — MVP Contract and Economy Integration

Status: implemented MVP contract. This document defines the boundaries that
keep manufacturing connected to the rest of SPHERES without creating a second
economy, a second resource market, or a second military inventory.

The central rule is:

> Manufacturing directs the equipment share of the defense budget through
> completed arms plants. It consumes the existing physical resource stockpile
> and places orders into the existing arsenal. It does not create money,
> materials, equipment value, military strength, or combat readiness by another
> route.

The implementation is opt-in through `GameRules.manufacturing_system`. The flag
is false and omitted from default saves; empty `WorldState.manufacturing` state
is also omitted. The calibrated/headless world therefore remains on its legacy
path. Browser play enables the system with the resource market, logistics, and
province production systems.

### Daily-clock amendment — 2026-09-03

Ridge's request to put everything on the daily ticker supersedes the monthly
settlement language below for worlds with `daily_simulation` enabled. All
monthly rates remain monthly **reference quantities**, not work deferred to
month-end. Each day places `monthly_budget_bn / days_in_current_month` plus
the existing banked dollar stock. The bank itself is never divided by the day
count. Tiny slices below the posting threshold remain banked, not discarded.

`planned_allocations`, `resource_draw` and `resources::draw` remain monthly
forecasts for reserves and negotiations. `tick_allocations` and
`resources::tick_draw` are the exact daily action bundles, including banked
money once. A daily shortage means today's bundle cannot be supplied; it does
not mean twelve months of stock must already exist. Priority and atomic
multi-input consumption are unchanged.

Orders carry optional `due_days`, derived from the real calendar at placement.
Old orders acquire a remaining-day clock from their remaining monthly term on
first daily processing. Equipment ages fractionally each day; retirement uses
the daily retention factor of the existing monthly rate. Repeating the same
day cannot place orders or age equipment twice. A month-batch repeatedly runs
the same daily path, including across leap days and save/load.

Physical freight is now implemented, as described in `LOGISTICS.md`: loaded
imports wait for their route's actual due day before this board can use them.
The older monthly-only logistics descriptions below are historical MVP scope.
The ministry matrix in section 7 likewise records the earlier proposal;
`ministries.rs`, the BIBLE amendment and SPEC define the current named arms.

## 1. Player loop

1. Build an **Arms Plant** in a province through the Production board.
2. Open the **Manufacture** tab in that same board.
3. Choose a real equipment designation the nation knows how to build.
4. Assign the line to an owned province with a free arms-plant level.
5. Set the line to High, Normal, or Low priority.
6. Each daily settlement (monthly in legacy audits) divides the existing procurement envelope among the
   active lines, buys any obtainable inputs through the existing market, and
   attempts each material bundle atomically.
7. A successful slice becomes a normal arsenal order with the equipment's real
   lead time. A blocked slice consumes nothing and its money remains banked.
8. Years later, the existing arsenal delivery path moves the order into held
   equipment. Age, condition, book value, adequacy, and the war model then read
   it without a manufacturing-specific combat bonus.

This is an arcade loop — a few lines, three priorities, clear blockers — but its
consequences use the simulation's existing long time constants. Manufacturing
cannot out-produce a war already under way.

## 2. Authoritative owners

| Quantity | Sole authoritative owner | Manufacturing's relationship |
|---|---|---|
| GDP, growth, inflation, debt and taxes | `economy.rs` and `Nation` | Read GDP only to size the existing defense envelope. Never write macro outcomes directly. |
| Annual spending plan | `AnnualBudget` in `world.rs` | Read the Defense allocation through `mil_spend_gdp`; do not create a manufacturing appropriation beside it. |
| Political capital | `politics.rs` and the command price/charge path in `lib.rs` | Pay only for discrete line decisions. Manufacturing never earns or regenerates PC. |
| Physical resources | `resources.rs` `MarketState.stocks` | Submit demand and consume one atomic bundle. Never keep a parallel material stockpile. |
| Prices and import finance | `resources.rs` market settlement | Accept existing spot prices, trade cash, contract headroom, and debt writes. Never price inputs locally. |
| Mines and national resource flow | `resources.rs` HAVE ledger | Receive their output indirectly when it is posted to the common warehouse. |
| Industrial sites | `production.rs` `ProvinceCapabilities` | A line occupies one completed `arms_plants` level in one exact province. |
| Technology eligibility | `tech.rs` and `EquipmentDef.tech` | Check whether the nation knows the required technology. Do not add a second unlock tree. |
| Equipment definitions | `arsenal.rs` `DECK` | Reuse stable kit id, name, unit cost, lead time, service life, class, and tech gate. |
| Orders and holdings | `arsenal.rs` `Arsenal` | Successful work creates the existing `Order`; the Arsenal remains the only inventory. |
| Force structure and combat | `war.rs` | Receive manufacturing only through delivered arsenal book value and adequacy. No direct writes. |
| Munitions | `Nation.munitions` and `war.rs` | Deferred. Manufacturing does not refill magazines in the MVP. |
| Foreign arms aid | `statecraft.rs` | Deferred. Existing aid continues its legacy strength path and is not translated into factory output yet. |

## 3. Persisted state and commands

The sparse manufacturing ledger contains:

```text
Manufacturing
  lines: Vec<ManufacturingLine>
  next_id: u32

ManufacturingLine
  id: u32
  nation: NationId
  district: String
  kit: String                 # stable EquipmentDef id, never a DECK index
  priority: High|Normal|Low
  status: Producing|Blocked
  reason: Option<String>
  ordered_bn: f64             # cumulative procurement value successfully ordered
  resources_used: [f64; 12]   # cumulative conserved physical inputs
```

Player actions are server-authoritative commands:

| Command | Political cost | Availability |
|---|---:|---|
| `StartManufacturingLine` | 8 PC | Refusable; requires an eligible kit and a free owned arms-plant level at the named province. |
| `SetManufacturingPriority` | 0 PC | Owner only; changes allocation and scarce-input order. |
| `StopManufacturingLine` | 0 PC | Always available to the owner; already placed arsenal orders remain real. |

A refused command is atomic: no PC, line, order, stock, or headline side effect
may survive it. Stopping a line does not cancel orders already accepted by the
arsenal and does not recover materials already consumed.

One completed arms-plant level supports one directed line. Capacity is counted
only in provinces currently owned by the sponsoring nation. A captured site
therefore stops supporting its line, but capture does not teleport or delete
orders and holdings already on the national books.

The production project that creates the site remains governed by the Production
system: an Arms Plant is a 720-day province project, costs 16 PC to start, reads
the Defense ministry for construction throughput, and consumes its existing
project recipe over time. That construction decision and the 8-PC decision to
open an equipment line are separate decisions; neither creates another fiscal
spending stream.

## 4. One defense envelope

The equipment envelope already exists in `arsenal.rs`:

```text
monthly_budget_bn = clamp(GDP * mil_spend_gdp * PROCUREMENT_SHARE / 12, 0, 1_000_000)
PROCUREMENT_SHARE = 0.20
available_line_bn = monthly_budget_bn + Arsenal.banked
daily_available_line_bn = monthly_budget_bn / days_in_current_month + Arsenal.banked
```

`mil_spend_gdp` is the Defense ministry allocation after a detailed annual
budget is enacted. The whole defense share is already included in the economy's
fiscal identity:

```text
revenue/GDP = tax_rate + budget_oil_revenue
spending/GDP = social_spend + mil_spend_gdp + state_invest_gdp
deficit/GDP = spending/GDP - revenue/GDP
```

Manufacturing must not add its domestic order value to debt. The economy has
already charged the defense budget. Foreign material purchases continue to use
the existing market settlement, which may spend retained trade cash or add
debt; manufacturing adds no third charge.

When there are no directed manufacturing lines, `arsenal::tick` performs its
legacy automatic procurement exactly as before. This preserves old saves, AI
behavior, and calibrated timelines.

When at least one directed line exists for a nation, those lines **replace** the
legacy automatic placement for that nation. They do not run beside it.

Line weights are:

```text
High   = 3
Normal = 2
Low    = 1

line_slice_bn = available_line_bn * line_weight / sum(active_line_weights)
```

Every active directed line participates in the same allocation pool. Settlement
order is High, then Normal, then Low, with stable line id as the tie-break. The
order governs which line receives scarce stock; it does not change the total
money available.

Successful slices become existing arsenal orders:

```text
units = line_slice_bn / EquipmentDef.unit_cost
due   = EquipmentDef.lead_months
```

Failed slices are banked, with total banked procurement capped by the existing
rule:

```text
Arsenal.banked <= 24 * current_monthly_budget_bn
```

The system does not mint value when a line is blocked, stopped, reprioritized,
captured, or unavailable for technical reasons.

## 5. One physical resource ledger

Manufacturing uses the twelve existing commodity rows:

`bauxite`, `coal`, `cobalt`, `copper`, `gas`, `gold`, `iron`, `oil`,
`phosphate`, `platinum_group`, `rare_earths`, and `uranium`.

The initial manufacturing recipes reuse the grounded class coefficients already
present in `resources.rs`. They are scaled by the line's monthly $bn slice and
converted into each commodity's table unit by the same code used for market
demand and settlement:

| Equipment class | Input per $1bn ordered |
|---|---|
| Naval | 12 kt iron ore and 6.5 kt coking coal |
| Armour | 20 kt iron ore and 11 kt coking coal |
| Air | 1.4 kt bauxite |
| Missile or Space | 0.1 kt copper |
| Infantry | No separately tracked bulk input in the MVP |

Directed manufacturing applies these class recipes to both modern and legacy
equipment lines. The legacy auto-procurement path retains its historical
exemption when no directed lines exist, which is part of the zero-line
compatibility guarantee.

The complete set of operational slices is aggregated before spot clearing. One
shared demand function must feed all of these consumers:

- warehouse reserve targets;
- negotiated contract and spot-market demand;
- the arcade board's demand, stock, and shortfall figures; and
- the atomic bundle actually consumed at settlement.

The market's reserve policy remains twelve months of current need
(`BUFFER_MONTHS = 12`). A reserve target is not a storage cap and changing the
line plan never deletes stock already held.

Before any line moves, every positive commodity in its bundle is checked against
one opening warehouse snapshot. If one input is short, the whole slice fails:

```text
BLOCKED: needs <quantity> <commodity> this day, have <quantity>.
```

No line may consume its iron and then discover that its copper is missing.
`resources_used` must close exactly to the fall in the national warehouse for
successful bundles, within the market's nanounit quantization.

The existing `resource_gates = false` calibration override is genuinely
ungated: a directed line may order without warehouse inputs and records no
physical `resources_used`. Browser play enables the gates.

### Existing market effects

- Domestic mine/resource production posts monthly to the national warehouse.
- Contract deliveries post before spot clearing and are curtailed pro rata when
  a seller cannot fill all promises.
- Spot matching fills the lowest-cover buyers first and is deterministic.
- The whole foreign-buy ceiling is `1% of GDP / 12` for the month, less contract
  spend already due. It is one ceiling across commodities, not one per row.
- A positive net import bill spends retained market cash first and then adds
  `unfunded_cost_bn / GDP` to debt/GDP.
- Export receipts retire debt first; any excess becomes retained market cash.
- Non-oil prices use an 80% memory of the old quote and 20% of a
  supply/demand-indexed reference quote, bounded to 0.4–3.5 times the sourced
  1990 reference price.
- Oil remains the separate calibrated world-price ledger and is not a physical
  manufacturing input in the MVP.

### Trade and logistics

Manufacturing does not create a second logistics model. Existing physical lanes
close when either party sanctions the other or they become direct belligerents.
A physical contract's financial counter-leg suspends with the blocked shipment.
Map-derived distance, modeled coastal gateways, shared land/sea throughput,
and chokepoints are now owned by `logistics.rs`, not by manufacturing. Actual
historical port/rail infrastructure is still not claimed. Paid cargo arrives
in the common warehouse before it can feed an equipment line.

Negotiated supply contracts can create `trade_dependency` and therefore
diplomatic leverage. Anonymous spot fills do not create that bilateral leverage.
Manufacturing may increase the need that motivates trade, but it does not award
relations or dependency merely for buying on spot.

### Mines

A completed mine feeds the same HAVE flow and warehouse as inherited production,
so manufacturing receives its output without a direct mine hook. Existing mine
economics remain authoritative:

```text
build time = 12 months
political cost = 6 PC
investment = clamp(2 * annual mine output value, $0.25bn, $25bn)
```

Mine investment is a one-time debt write at project start. Manufacturing must
not charge it again or apply a separate domestic-resource discount.

## 6. Province capabilities

Manufacturing's immediate province tie is deliberately narrow:

- `arms_plants`: one level equals one line slot in that exact owned province.

The other completed province capabilities keep their current owners:

- `infrastructure`: +10% construction speed per level at that site;
- `civilian_industry`: +0.15 national construction capacity per owned level;
- `power_grid`: persisted capability only;
- `research_centers`: persisted capability only.

Power-grid and civilian-industry manufacturing multipliers are deferred. Every
province begins with zero authored capability levels; treating zero power as
zero factory output would disable the world for a missing-data reason, while a
free multiplier would manufacture equipment value outside the defense envelope.
Any future connection must be a bounded utilization or allocation rule, not an
unfunded equipment bonus, and must be tested across seeded worlds first.

## 7. Ten-ministry economy matrix

Every ministry is a share of GDP with a stable cap. The full budget cannot
exceed 70% of GDP. `budget_gap` means enacted allocation minus the inherited
reference, so an untouched budget keeps all detailed channels inert.

The table records effects that already exist. Manufacturing adds only the
Defense-envelope read named in the last column.

| Ministry (cap) | Existing effects of its gap from reference | Manufacturing MVP |
|---|---|---|
| Health (15%) | Potential growth `+0.015*gap`; demand `+0.06*gap`; jobs term `+0.12*gap`; population growth `+0.030*gap`; stability `+8*gap`. | No direct tie. |
| Education (12%) | Potential `+0.050*gap`; jobs `+0.16*gap`; stability `+5*gap`; research-output multiplier includes `+20*gap`. | No direct tie. |
| Families (15%) | Demand `+0.28*gap`; population `+0.015*gap`; stability `+14*gap`; positive Families and Security gaps reduce separatism. | No direct tie. |
| Pensions (20%) | Demand `+0.18*gap`; stability `+12*gap`. | No direct tie. |
| Infrastructure (15%) | Potential `+0.025*gap`; jobs `+0.28*gap`; private-investment pressure `+0.02*gap`; funds Infrastructure province projects. | No direct tie beyond the existing construction path. |
| Industry & Energy (12%) | Potential `+0.035*gap`; jobs `+0.24*gap`; private-investment pressure `+0.04*gap`; funds Civilian Industry and Power Grid projects. | Deferred; no free throughput multiplier. |
| Science (8%) | Potential `+0.025*gap`; jobs `+0.08*gap`; private-investment pressure `+0.02*gap`; research-output multiplier includes `+35*gap`; funds Research Centers. | No direct tie. Equipment eligibility arrives through Tech. |
| Defense (35%) | Sets `mil_spend_gdp`; is already fiscal spending; sustains force structure and capital intensity; funds the existing 20% procurement line; supports army loyalty; funds Arms Plant construction throughput. | Sole monetary envelope for directed equipment orders. |
| Security (12%) | Stability `+16*gap`; positive Security and Families gaps reduce separatism. | No direct tie. |
| Diplomacy (8%) | Private-investment pressure `+0.01*gap`; stability `+3*gap`; diplomatic shield is `clamp(8*gap,-0.20,0.40)` against sanction growth drag. | Indirectly affects market access through ordinary statecraft only. |

These detailed ministry channels are explicitly uncalibrated and inert on the
default path. Manufacturing must not use them as justification for new GDP,
jobs, stability, research, or output bonuses.

## 8. GDP, political capital, and technology

### GDP and output

GDP increases the absolute size of a fixed defense share and therefore the
monthly procurement envelope. Growth and recession consequently change how
many units a fixed line plan can order without a manufacturing-specific GDP
formula.

Manufacturing never writes GDP, growth, employment, inflation, taxes,
investment, stability, or population. Industry spending, private investment,
capital deepening, TFP, and technology already own those outcomes. A permanent
factory growth-rate bonus would count the same capital stock as a new flow and
violate the economy's level-versus-rate rule.

### Political capital

Political capital is earned from stability, controlled prices, growth,
government composition, and low war exhaustion. Manufacturing only spends it
on opening a directed line. Reprioritizing and stopping are operational acts and
remain free. This prevents a daily click tax and keeps PC as the price of making
a national commitment rather than the price of receiving every unit.

### Technology

`EquipmentDef.tech` is the only manufacturing unlock. Legacy kits remain
available without a named technology; modern kits require the exact existing
technology. Research retains all of its current economic inputs:

```text
development = clamp((GDP per capita / $24,000), 0, 1)
research intensity = (0.008 + 0.017*development)
                     * (0.55 + 1.5*(state investment + private investment))
monthly research = GDP * intensity / 12
```

Education and Science gaps, known Research Rate bonuses, regime type,
stability, war, sanctions, diffusion, prerequisites, and year floors continue
to modify that system. Research Centers do not add another research multiplier
in this MVP.

## 9. Arsenal, readiness, and war

The Arsenal remains the only bridge from manufacturing to military power:

```text
Order --after EquipmentDef.lead_months--> Holding
Holding value = units * unit_cost * condition(age)
book_value = sum(Holding value)

wanted book value = monthly_budget_bn * EQUIP_HORIZON
EQUIP_HORIZON = 200 months
adequacy = BARE_FORCE + (1 - BARE_FORCE)
           * clamp(book_value / wanted, 0, ADEQUACY_CAP)
BARE_FORCE = 0.55
ADEQUACY_CAP = 1.30
```

The war model already consumes that adequacy exactly once:

```text
sustained_force = sqrt(GDP * mil_spend_gdp * 0.30) * 8
                  * tech_military_multiplier
                  * arsenal_adequacy
                  + tech_military_floor
```

Force structure converges toward that level at 2% per month. Manufacturing
must not add `ordered_bn`, unit count, or authored `EquipmentDef.quality`
directly to `mil_strength`. The arsenal deliberately uses money against money;
cross-class quality values are not a safe scale.

### Deferred readiness effects

Munitions currently rebuild by:

```text
monthly refill = 0.030 * capital_intensity
```

and burn by commitment rung. Manufacturing missiles, infantry equipment, or
aircraft does not also refill `Nation.munitions` in the MVP. Connecting an
equipment or ammunition stock to magazines must **replace** an appropriate part
of the current rebuild rule; adding it on top would buy the same defense
industry twice.

The same rule applies to deployable fraction and combat quality, both already
derived from defense budget, force structure, and technology. They receive no
factory addend.

Foreign Arms Aid currently bypasses the Arsenal and directly nudges a client's
force structure toward a patron-funded level. Converting it into transferred
orders or holdings is a separate migration. Manufacturing must not mirror that
flow into both the Arsenal and `mil_strength`.

## 10. Statecraft and resource pressure

Manufacturing can affect statecraft through real dependencies, never through a
free diplomatic modifier:

- A line's aggregate resource demand enters the market.
- A shortage may motivate a negotiated supply contract.
- Contract dependency can improve access and commitment receptivity through the
  existing `trade_dependency` path.
- Sanctions, bilateral war, hostile relations, or universal seller refusal can
  close supply.
- Only after the existing AI has tried to buy and every producer has genuinely
  refused may the existing last-resort resource-war appetite consider a
  resource-bearing district.

The same aggregate manufacturing demand must therefore replace the old
single-auto-pick `resources::draw` wherever reserve targets, seller surplus,
market buys, refusal memory, and last-resort appetite ask what a nation needs.
If those readers see one plan while settlement consumes another, the AI's causal
story is false.

Manufacturing does not declare war, alter relations, fabricate a refusal, or
turn a market price into a war aim itself.

## 11. Deferred integrations

These are deliberate boundaries, not forgotten ties:

1. **Power-grid manufacturing efficiency.** Deferred until province capability
   levels have an honest opening seed and the effect can be a bounded
   utilization rule.
2. **Civilian-industry spillovers into military output.** Deferred. Civilian
   Industry already accelerates construction; another output multiplier needs
   balance measurement and still cannot exceed the defense envelope.
3. **Factory GDP, growth, export, or jobs rewards.** Deferred. The macroeconomy
   already prices investment, industry spending, private investment, TFP, and
   trade. Any connection must be a bounded level effect with a paid-state
   tracker, never a permanent growth-rate addend.
4. **Research Center bonuses.** Deferred. Education, Science, capital investment,
   and technology bonuses already fund research output.
5. **Manufactured ammunition to `munitions`.** Deferred until it replaces, not
   supplements, part of `MAGAZINE_REBUILD`.
6. **Direct equipment-class combat effects.** Deferred. The MVP changes
   adequacy through Arsenal book value only.
7. **Arms-aid inventory transfers.** Deferred until Statecraft can transfer
   orders/holdings without retaining its direct strength payment.
8. **Maintenance and operating costs.** Deferred. Defense spending already
   pays the entire force curve; a maintenance debit would require an explicit
   subdivision of that budget.
9. **Plant damage, conversion, and repair.** Deferred. Ownership gates capacity;
   physical damage needs a separate province consequence and recovery path.
10. **Ports, rail throughput, distance, and chokepoints.** Deferred to a real
    logistics-capacity model; current logistics provides hard route closures
    and shipment audits only.

## 12. Double-count red lines

The following are implementation errors:

1. Directed manufacturing places orders **and** legacy `arsenal::tick` places a
   full automatic order for the same nation and month.
2. Manufacturing consumes a resource bundle and then calls a legacy gate that
   consumes the same bundle again.
3. A domestic equipment order adds debt after the Defense share has already
   entered the fiscal deficit.
4. `ordered_bn`, equipment units, plant levels, or project progress directly
   adds GDP, growth, military strength, munitions, combat quality, or technology
   progress.
5. An Arms Plant creates equipment without receiving a share of the fixed
   procurement envelope.
6. Power Grid or Civilian Industry levels multiply equipment value above the
   money actually allocated.
7. A UI computes eligibility, affordability, inputs, ETA, or shortfalls from
   copied constants instead of rendering the server's answer.
8. A failed multi-input bundle leaves some inputs consumed.
9. A stopped or captured line deletes existing national orders or holdings.
10. A second equipment stockpile is introduced beside `Arsenal.held` and
    `Arsenal.orders`.

One existing modeling tension is recorded rather than hidden: imported raw
materials can add market debt while the purchased equipment's `unit_cost` is
also what sizes units from an already-funded procurement line. That is the
resource market's current fiscal design. Manufacturing reuses it and must not
add another cash charge. Repricing the relationship between kit cost and input
imports belongs to a separate economic-calibration pass.

A second tension is also explicit: the Defense allocation controls both Arms
Plant construction throughput and the Arsenal's procurement envelope without a
zero-sum subdivision between them. The MVP treats Defense as a common readiness
signal and does not claim those subprogram dollars are separately conserved. A
future defense-budget breakdown may split construction, personnel, operations,
procurement, and munitions; this MVP must not invent that split implicitly.

## 13. API and UI contract

`GET /api/manufacturing` supplies the complete server-authored board:

- summary and capacity;
- active lines and their exact province, priority, status, reason, allocation,
  ordered value, input demand, held stock, and present shortfall;
- the complete `DECK` catalogue with stable kit ids, real names, classes, unit
  costs, lead times, service lives, and technology lock reasons;
- eligible arms-plant provinces and occupied/free levels;
- existing Arsenal holdings and outstanding orders; and
- only actions the current player may actually issue.

The command JSON shapes are:

```json
{"kind":"start_manufacturing_line","district":"<district-id>","kit":"<stable-kit-id>"}
{"kind":"set_manufacturing_priority","line":1,"priority":"high|normal|low"}
{"kind":"stop_manufacturing_line","line":1}
```

`/api/state` carries only a compact `manufacturing_summary` for the dock/tab
badge. The detailed board is fetched on demand.

Manufacturing lives as a **Manufacture** tab inside the existing Production
panel. It does not add a fifth permanent dock button or cover the map with the
entire equipment catalogue. Province markers appear only while the relevant
board is open. The base screen remains the globe plus a compact alert/count.

The browser does not infer rules. It renders server values and server-authorized
actions, uses the ordinary `/api/command` path, and refreshes from returned game
state after every command.

## 14. Invariants and acceptance tests

The MVP is complete only while all of these remain true:

### Compatibility and determinism

- With `manufacturing_system == false`, the save omits the flag and ledger and
  the world is byte-identical to the pre-manufacturing path.
- With the feature enabled but no directed lines, automatic Arsenal procurement
  is byte-identical to the legacy path.
- Old saves load with no manufacturing state and retain automatic procurement.
- Manufacturing state, line ids, stable kit ids, priority, cumulative ordered
  value, and cumulative resources survive save/load exactly.
- `resources_used` deliberately matches the resource layer's fixed twelve-row
  save array. Adding a thirteenth commodity requires an explicit save migration
  (or keyed custom deserialization); it must not silently resize old histories.
- Line iteration, priority dispatch, market demand, and order placement use
  stable ordering and no RNG.
- A month stepped day by day equals the same month stepped at once at its
  settlement boundary.

### Ownership and commands

- A line cannot start without a completed free arms-plant level in the exact
  named province.
- Foreign, nonexistent, captured, or fully occupied sites are refused without a
  partial mutation or PC charge.
- A locked or unknown kit is refused with a specific reason.
- One arms-plant level cannot support two active directed lines.
- Capture blocks the site's active line while preserving earlier orders and
  holdings.
- Only the sponsoring nation may reprioritize or stop a line.

### Money conservation

- In a directed month, successful slices plus newly banked failed slices cannot
  exceed opening `arsenal::line_of` except for market nanounit tolerance.
- A nation with directed lines receives no legacy automatic order that month.
- The sum of line shares is the one procurement envelope, not one envelope per
  line.
- Banked money never exceeds 24 current monthly budgets.
- Manufacturing creates no direct domestic debt write.

### Material conservation

- Market demand is the sum of the same operational line slices settlement will
  attempt.
- A fully supplied slice reduces each warehouse row by exactly its recorded
  `resources_used` increment.
- A short bundle consumes no commodity and creates no Arsenal order.
- High priority receives a scarce atomic bundle before Normal, Normal before
  Low, and line id breaks equal-priority ties.
- Domestic production, mine output, contracts, spot imports, and route closures
  all affect the same stock rows the line reads.

### Arsenal and war

- A successful slice creates an existing Arsenal order with
  `units = slice/unit_cost` and `due = lead_months`.
- No held equipment appears before the existing order lead time expires.
- Delivery, ageing, condition loss, retirement, book value, and adequacy remain
  owned by `arsenal.rs`.
- Manufacturing cannot directly change `mil_strength`, `munitions`,
  `war_exhaustion`, force deployment, or combat outcome in its settlement
  function.
- The eventual military effect appears through delivered book value and the
  existing adequacy multiplier exactly once.

### API and presentation

- The API reports exact present-tense input need and shortfall, not the total
  lifetime wish for a line.
- Unknown enum values, kit ids, line ids, and foreign player identities are
  rejected by the server.
- Buttons are generated from server-authorized actions; disabled reasons are
  legible.
- The Production panel remains nonmodal and the map remains usable and dominant.
- Desktop and mobile layouts expose the same commands without displaying the
  full catalogue until the player asks for it.

## 15. Next integration order

After this groundwork is measured in play, connect later systems in this order:

1. Seed or derive honest opening province power and industrial capability.
2. Add a bounded plant-utilization rule that never exceeds the fixed defense
   procurement envelope.
3. Split equipment roles if the Arsenal needs explicit line, lift, deterrent,
   and magazine stocks.
4. Replace part of munitions rebuild with manufactured magazine output.
5. Convert Arms Aid from a direct strength nudge into conserved equipment
   transfers.
6. Add plant damage/repair and only then province logistics capacity.
7. Recalibrate the economy and emergent-history suite before considering any
   GDP, employment, export, or productivity feedback.

That order preserves one causal chain throughout:

```text
annual budget -> procurement envelope -> province arms-plant slots
             -> resource demand -> mine/contract/spot/logistics fulfillment
             -> long-lead Arsenal orders -> held book value -> adequacy
             -> sustained force -> war
```
