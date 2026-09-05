# Economic Competition

This increment connects civilian investment, manufactured goods, physical trade,
research prototypes and formal spheres. It is a game model, not a claim to have
reconstructed historical factories or district production. Read
`ECONOMIC_COMPETITION_PROGRESS.md` for verified results and unfinished checks.

## Entering the system

In a daily browser campaign, open **Exchange → Enable Economic Competition**.
Enabling is free but creates no cash, political capital, factories or stock. It
activates production, resource markets, physical logistics and province GDP
accounting. The human still chooses their own budget and investments; AI plans
never issue orders for the player. Old saves are not silently enrolled.

The new competition systems are default-off for legacy headless campaigns.
Two daily-clock corrections apply to daily play independently: additive private
investment pressure is prorated as a flow, and technology acquisitions retain a
calendar-month limit. Neither changes the legacy monthly branch.

## Government investment

Every living non-player country is eligible; there is no major-power whitelist.
The deterministic planner reviews every 30 days, renews annual appropriations and
uses ordinary commands. Political prices, department authority, site ownership,
construction capacity, prerequisites, actual cash and input shortages still apply.
Unchanged free annual renewal must work even at zero political capital.
Renewal precedes other review actions. Permanent loss of a construction province
cancels only that stranded project; temporary occupation does not discard it.
Routine import replenishment does not consume the strategic investment review.

The initial civilian chain is industrial estate → power/grid → processing →
machinery. A consenting intermediate supplier can support specialization. The
planner can seek raw inputs or mapped mines, protect starter inventories, offer
surplus and buy useful manufactured inputs. It keeps bounded civilian queues and
limits discretionary purchases; it does not conjure buyers or inventories to
make a growth target pass. Storage, upgrades and research produce real demand.

Country decisions disclose both their current action and a concrete blocking
reason. A funding horizon gives an optimistic lower bound using unchanged GDP,
renewed budgets and unshared construction capacity. It is NOT a promise: competing
projects, materials, transport and politics can delay it.

**Known balance boundary:** existing project recipes describe full-size facilities.
Very small economies can do paid work yet require decades to afford completion.
An AI-only discount would create disproportionate free capacity. Proportional
small sites would need matching cost, materials, construction, power, storage,
output and GDP scaling; they are not smuggled into this planner.

## Manufactured commerce

The twelve mapped raw commodities are unchanged. `commerce::Good` adds a separate
market for **intermediate packs** and **capital goods**, consuming the same actual
inventory that civilian projects and factories use. Packs are explicit modeled
game units, not newly sourced national production observations.

1. A seller authorizes exports, sets a domestic reserve and posts an ask.
2. A buyer requests a finite lot. The evaluator accepts, counters or explains
   refusal. Quotes reserve nothing and can become stale.
3. Acceptance reserves actual seller goods and buyer treasury cash. No implicit
   loans, future tax revenues or unappropriated project authority fund a purchase.
4. Dispatch uses current route access and the SAME capacity ledger as raw cargo.
   The seller is paid once for the dispatched quantity.
5. Arrival adds usable inventory, subject to warehouse space and open delivery
   conditions. Paid goods are never usable while merely in transit.
6. Cancellation/expiry refunds only the undispatched remainder. Already-paid
   cargo remains the buyer's property; route and warehouse holds preserve it.

The loading window is 1–365 days, not an arrival guarantee. Offers expire after
7 days; negotiation costs 2 PC. Quantities and prices are finite and bounded; active
contracts/counters are bounded and completed history is trimmed. Account totals
continue after detailed history trimming. Zero/negative/nonfinite or unaffordable
requests are refused without a financial side effect.

**Successor limitation:** a dead government's undispatched property returns to
its named ledger, while paid cargo stays held for the named buyer. This increment
does not invent a successor inheritance policy for commerce stock/cash.

## One set of economic books

- Construction pays its actual funded work and consumes actual materials.
- Factories need completed capacity, power, inputs and operating authority.
- Province GDP sees actual modeled value added; country GDP reconciles to its
  provincial components and inherited base.
- Selling previously produced inventory is a cash transfer, **not another GDP
  award**. Signing a compact likewise transfers no GDP, province or treasury.
- Debt, government cash, annual GDP and departmental spending authority are
  different quantities and are labeled separately in the Exchange.

## Research workshops and the daily clock

A completed, controlled research center can fund prototype/testing work for one
eligible active technology. It consumes Science → Basic research authority and
intermediate/capital-goods packs. Credit reduces that technology's acquisition
bill; it is neither extra research money nor direct GDP.

Credit is capped at 20% of the undiscounted bill. Daily useful work is limited by
ordinary domain effort and completed lab capacity; no funds are spent beyond the
useful remaining bill. Prerequisites, year gates and available resources are not
bypassed. Credit is owned by the nation and specific technology, does not move on
capture/focus changes, and saves use stable technology IDs.

Each domain can pay for at most 6 acquisitions per actual calendar month. Unused
slots expire; research effort keeps banking. The quota survives save/reload and
does not reset when focus changes. Old mid-month daily saves lack an acquisition
record, so they conservatively wait until the next month. Their view says so and
does not pretend six acquisitions occurred. New day-one campaigns start with 6 slots.

Additive investment pressure uses `.06 × month_fraction` in daily mode. Exponential
smoothing remains for genuine smoothing processes; these are different operations.

## Economic influence and world domination

The shared dependency view takes the largest of existing pact/raw dependency and
manufactured dependency. Manufactured influence requires **delivered** reference
value over the trailing 365 days divided by the buyer's annual GDP, capped at 12%.
A signature, inflated asking price or escrowed shipment gives no leverage.

The initial AI buyer's discretionary envelope is 0.1% of GDP per 30-day review.
At stable GDP that is roughly 1.2% annually, below the 12% compact gate: this build
does not demonstrate manufactured-goods-only domination. Existing pacts/raw
sourcing can provide the larger dependency; manufactured delivery also has real
productive use. Do not increase purchases or lower consent thresholds merely to
force a desired census outcome.

A voluntary compact requires an independent credible patron and partner at
peace, no bilateral sanctions, relations 55+, patron reputation 50+, mutual
defense protection, patron GDP at least 1.5 times partner GDP, dependence 12%+
and a 4-percentage-point advantage over the reverse tie. These are visible gameplay
thresholds, not fitted historical facts. Negotiating costs 20 PC.

The partner stays economically separate but becomes formally subordinate, which
counts toward the existing domination condition. AI cannot automatically submit
the human; the player must explicitly choose to join another sphere. Countries
inside a formal sphere cannot attack, escalate against or intervene against their
own bloc without leaving/releasing first. Guards apply to player and AI paths.
Descendants are part of that check: a voluntary merger cannot conceal a war or
sanctions between the two subject trees. After conquest, only newly incompatible
participants withdraw; unrelated foreign coalitions remain. Empty conflicts close
without another conquest/claim/compensation reward, and stale war clocks retire.

An explicit exit costs 12 PC and reputation/relations, but remains available to a
politically exhausted subject and does not declare war. Patrons can release direct
subjects. AI voluntary partners review retained protection, trust, size and
dependency; three strained reviews can end the compact. Recovered conquered
subjects can also seek independence. It is not permanent free annexation.

## Browser safety and presentation

The full-page Exchange has four views: **Your industry**, **Goods market**,
**World economies**, **Your sphere**. Cards use large text and restrained soft
colors; secondary accounting sits behind disclosures. The map stays uncluttered.
Keyboard focus is contained, Escape returns to the opener, and hidden gameplay
shortcuts cannot advance time while the room is open.
Industrial output and spending are explicitly dated settled receipts, not a
promise that the current day's work has already happened.

Prices, quantities, country plans and effects come from Rust read models. Browser
mutation commands bind the actor to the seated player. Financial requests carry
campaign identity and an idempotency receipt. A lost response retains the same
receipt for retry; changed or stale requests are rejected, and retries return
current world state rather than rolling the UI back to an old snapshot.
An uncertain economic order blocks time advance, saving, campaign replacement
and unrelated immediate orders until its receipt is checked or explicitly
reviewed. Closing the room or unavailable browser storage does not discard it.

## Verification and remaining evidence

Run the Rust workspace suite with a task-specific `CARGO_TARGET_DIR` and preserve
known baseline failures. New integration tests live in `economic_competition`,
`economic_sovereignty`, `economic_sphere_intervention`, `research_centers` and
`daily_balance`; module commerce
tests cover settlement/conservation and raw logistics tests cover shared capacity.

The census executable accepts `days seed output.json [final-world.json]`:

```text
cargo run -p spheres-sim --release --example economic_competition_census -- 365 42 report.json
```

It evaluates all 137 starting countries, binned by starting GDP (<1, <10, <100,
<1000, and ≥1000 bn). It records enrollment, paid construction, completion, physical output,
goods cash, GDP change/project component, debt/cash and blockers. No initial
inventory, cash, political state or factories are overridden. Viable fixtures and
unmodified live-world runs are different evidence and must be reported separately.
The v4 observer distinguishes cash reserved for imports, cash paid at dispatch,
actual delivered reference value, final escrow, and moving/held cargo. Delivery
totals are accumulated daily so the rolling dependency window cannot erase an
older arrival from a multi-year report. Observer tests run with
`cargo test -p spheres-sim --release --example economic_competition_census`.

The optional fourth argument saves the actual final world for reproducing a
country's state, not a second simulation or synthetic fixture. For sequential
Windows runs, `tools/run_economic_census.ps1` accepts an already-built executable,
its SHA-256, an existing artifact directory, a unique label, horizon and seeds.
`-WorldSnapshots` requests those final saves. It preserves per-seed logs, rejects
overwriting prior evidence or a changed binary, and checks each completed report
before launching the next seed. Record a source manifest alongside every frozen
binary; an older running executable does not acquire subsequent source changes.

No universal positive-growth assertion is valid through wars and recessions.
Calibration bars must not be invented after observing a run. E-3 remains a known
productivity-reference decision, not permission to move legacy golden pins.

For browser QA, `tools/ui/check_competition_browser.cjs` and
`check_session_browser.cjs` require an explicitly disposable local server. Do not
run them against a user's campaign. All serverless UI checks exclude `*_browser.cjs`.

This document describes implementation, not a completion certificate. The
progress record contains measured results, current limitations and remaining work.
