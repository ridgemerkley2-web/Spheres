# Custom ammunition and aircraft mission stores

The Equipment bureau's **Ammunition** tab connects certified weapon configurations
to finite manufacture, physical stores and conserved operational consumption.
Ground weapons have 21 compatible families; tactical aircraft add unguided and
guided bombs. Inherited catalogue equipment continues to use its share of the
existing national magazine. Naval loadouts and other air missions remain future
work; the current aircraft slice is documented in [AVIATION.md](AVIATION.md).

## Player workflow

1. Certify a vehicle. The Designer review and Library show its compatible
   ammunition. A changed gun or carried-load specification can require a different
   store family; research alone does not convert existing rounds.
2. Set an actual maintenance plan in **In service**. Fleet upkeep is paid first.
   Ammunition manufacture can use the remaining Defense maintenance authority.
3. In **Ammunition**, review a finite batch, an arms-plant province and its daily
   payment ceiling. The quote separates fabrication from physical raw inputs.
4. Source missing inputs through the existing Resources market and reviewed
   material-purchase assistance. Paid cargo must arrive before factories use it.
5. For ground weapons, prepare stores and review physical ammunition activation.
   It starts on the next eligible date and grants no rounds. This migration
   cannot be toggled off to recover the former shared supply. Custom aircraft
   always require their exact physical bombs from their first deployment;
   manufacturing aircraft stores does not activate ground ammunition.

Each family shows completed stock, compatible available vehicles, unfinished
batches, current operation requirements and supported firing. A planning reserve
uses one modeled firing month for all available vehicles. It subtracts stock and
all active batch commitments before suggesting a new quantity; paused batches
remain explicitly conditional. It is a planning reference, not a predicted war
duration or an automatic order.

## Reserve plans and optional replenishment

Use **Set ammunition reserve** on a compatible family to choose a fixed stock
target, preferred province and ceiling for new batches. A new plan defaults to
**Manual target**. Manual plans track the gap and offer a reviewed order using
the saved site and funding defaults; saving the plan creates no work or spending.
Targets may be zero and are bounded at ten million rounds per family. A fixed
target does not silently grow when the vehicle fleet expands.

Selecting **Automatic replenishment** explicitly authorizes repeating finite
production batches. The first review occurs after the authorization date. On
each eligible daily tick, existing maintenance and ammunition work settle first.
The plan then nets physical stocks and all unfinished matching commitments,
including paused and blocked orders. It waits for every existing batch of that
family to finish before scheduling another, and caps each new batch at one
million rounds. Different families share the real plant slots and departmental
funds. The scheduler only starts ordinary orders; fabrication begins on a later
eligible date under its existing cash, raw-input and factory rules.

Automatic plans require an actual maintenance plan. A closed funding date, zero
ceiling, unavailable authority or factory access delays scheduling and produces
a dated explanation. Missing raw materials can leave a scheduled batch waiting:
automatic replenishment never purchases inputs. Only actual batches enter the
Resources forecast; an unissued reserve target does not claim materials or
create an import request. Existing finite batches continue to use their own
spending ceilings.

Changing, disabling or clearing a reserve plan affects future scheduling only.
It never changes existing batch quantities, sites, funding, paid work or stock.
Use the batch's separate pause/cancel controls to stop existing work. Clearing
the target stops its automatic orders; it grants no refund or replacement stock.

## Compatibility

| Weapon configuration | Required family |
| --- | --- |
| 90, 105, 120 or 125 mm tank gun | Matching caliber and mixed, penetrator or support load: twelve families |
| 25 or 35 mm autocannon | Matching autocannon caliber: two families |
| 12.7 mm weapon station | Machine-gun rounds |
| 122 or 155 mm howitzer | Matching caliber and high-explosive or guided load: four families |
| Mobile air-defense cannon | Air-defense cannon rounds |
| Mobile air-defense missile launcher | Short-range air-defense missiles |
| Aircraft unguided-bomb interface | Unguided aircraft bombs |
| Aircraft guided-bomb interface with digital avionics | Guided aircraft bombs |

The old five-package tank designs use an explicit compatibility mapping:
general-purpose armament maps to 105 mm mixed stores and heavy armament maps to
120 mm mixed stores. Their frozen vehicle profiles and canonical identities are
unchanged. This mapping is a game rule, not a historical specification.

There are **23 store families**. The ammunition or aircraft-payload slot's price
represents the installed load arrangement and integration; it does not grant
physical rounds. Fabrication prices, simplified iron/copper/coal recipes,
production rates and firing rates are all explicit game assumptions in
`spheres-sim/src/equipment_ammunition_production.rs`. No historical starting
inventory is invented. This remains an aggregate operational model with no
projectile trajectories, penetration simulation or detailed damage to components.

## Production and financial ownership

A batch reserves one real arms-plant slot, shared with ordinary manufacturing,
vehicle production and refits. A lost or occupied province, lost plant capacity,
zero funding, a funding-authority expiry or missing raw inputs can stop work.
There is no work on the issue date and at most one work settlement per date.

The daily payment is proportional to funded fabrication and bounded by the
batch's frozen remaining bill, daily ceiling, physical rate and remaining
departmental authority. Defense department 2 owns the payment through the
existing fiscal ledger. Raw inputs are bought separately through the existing
market and consumed once from shared stock; fabrication is not a second invoice
for them. Vehicle upkeep and ammunition cannot spend the same authority twice.
Finished whole rounds enter stores as work completes. The aggregate combat
model can record fractional expected consumption.

Pausing preserves work, stock and the plant reservation. Cancellation releases
the reservation and abandons unfinished work. It refunds neither paid work nor
consumed inputs, and removes no completed rounds. Orders retain frozen recipes,
costs and rates. Raw outstanding requirements and funded 30/90/365-day horizons
feed the same supply forecast and reviewed purchasing flow as other equipment.
The 128-batch queue limit counts unfinished work, including paused batches.
Completed and canceled receipts remain in history without blocking future orders.

## Operational ownership

The opening operations snapshot first allocates the same national force across
all conflicts. It plans compatible-family demand across those deployments,
apportions limited stock once, and settles consumption once after the conflicts
resolve. Refit reservations and pending deliveries do not supply available
vehicles. Wrong-family rounds do not satisfy demand.

After ground activation, custom ground weapons receive no scalar-magazine refill
or coverage. Delivered custom aircraft always use physical mission stores,
independently of ground activation. Legacy and custom shares are derived from
available physical reference weights before ammunition or maintenance degradation, so an unsupported custom
vehicle cannot become implicitly supplied inherited equipment. The inherited
magazine retains only its legacy share of burn and refill.

Firing, maneuver and observation have separate operational contributions.
Shortages gate a weapon's firing and fire support once, without removing a scout's
observation or a carrier's movement. Ground weapons consume no rounds during an
offensive air-only deployment. Mobile air defense uses its family when exposed
to opposing air raids; its stores do not create a general offensive gun.
Ammunition does not alter vehicle book value, standing force, physical condition
or the number of vehicles held. More stock extends supply; it grants no bonus
above fully supplied performance.

Aircraft stores are required only for assigned accessible rung-6 raids. Demand
uses supported aircraft equivalents, the model's frozen sorties per month and
stores per sortie, the conserved deployment share, operational intensity and
the simulation date fraction. Two- and four-store installations consume their
actual selected quantity. Wrong-family stores, insufficient maintenance,
withdrawn refits and missing theatre access cannot provide full strike output.
Aircraft never receive a free scalar fallback or contribute physical ground fire.
Their sortie-rate improvement is folded into the frozen strike factor once,
rather than awarding a second effect when the snapshot consumes bombs.

## Persistence and verification

Equipment-state version 6 added optional reserve plans and dated scheduling
receipts. Version 7 adds separate optional material-purchasing authority.
Version 8 adds frozen tactical-aircraft profiles and accepts versions 1–8;
ground activation remains separate from mandatory aircraft-store use.
Version 5 introduced physical ammunition.
Campaigns without reserve plans retain their earlier behavior. Completed
production must equal current stock plus cumulative consumption for every family.
The loader rejects contradictory quantities, payments, raw receipts, dates and
references rather than creating missing stock.

Verification covers compatibility, no starting grants, finite paid production,
shared slots and funding, fractional work and raw-stock conservation,
pause/cancel behavior, fiscal expiry, proportional multi-conflict use, dry and
partially supplied weapons, independent non-firing roles, unchanged vehicle
valuation, preview purity and deterministic save/resume. Tactical aircraft extend
the same ownership to finite bombs; full inherited-weapon conversion, other
air/naval loadouts and sourced historical presets remain later milestones.

## Optional purchases for ammunition production

Use **Review material purchasing plan** to open Production. The shared military
plan can buy missing raw materials for funded ammunition batches, vehicle orders
and refits. Choose automatic purchasing explicitly, set a per-review limit and
cash reserve, and choose a review interval. Manual purchasing remains available.
This is separate from the ammunition reserve plan: an unissued reserve goal has
no material bill, while a scheduled funded batch enters the finite forecast.
All paid incoming cargo counts before another purchase. Purchases use cash left
after daily bills, real supplier surplus and physical freight. Fabrication still
needs its own Defense maintenance funding and plant access. Disabling the
purchasing plan leaves paid shipments and existing batches intact.
