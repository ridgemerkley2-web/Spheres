# Companies & Procurement

7 September 2026 · the accepted domestic route covers all nine implemented ground
platforms and both tactical aircraft. Company-made ammunition now uses the same
contractor and reviewed stock purchases. That extension is accepted through final
Rust, UI and browser checks. This guide describes the implemented domestic mechanics,
not a claim that the full [company roadmap](COMPANIES_AND_PROCUREMENT_PLAN.md)
has shipped.

The country designs a ground vehicle or aircraft and funds development. A manufacturer pays to
produce its own finished stock. The government chooses how many completed
vehicles to buy, and delivery makes those vehicles available for service.

## Play the first supplier loop

1. Open **Research → Companies & Procurement**, or use the link in the
   manufacturing room. Establish a state contractor with a name, a domestic
   province containing a free completed Arms Plant slot, and an explicit initial
   investment. Establishment costs **8 political capital**; the investment uses
   available **Defense procurement** funding. The suggested investment is editable.
   Establishment stops the country's background automatic catalogue purchases so
   unassigned procurement funding can accrue for reviewed purchases. Explicit
   public lines, projects, ammunition work and already paid deliveries continue.
2. Open **Designer** or choose a saved model. Configure a tank, ground specialist
   or tactical aircraft and review its components, ratings and costs. Choose the manufacturer
   and review a development ceiling and initial company stock target. The suggested
   initial buffer is four ground vehicles or two aircraft, editable from 1–12.
   This means complete company-owned units, not a government order. An unfinished
   local draft survives the trip to the company page; saving also retains it in
   the equipment library.
3. Confirm the development contract. **Defense R&D** pays for actual engineering,
   prototype and trials work over time. The contract freezes this exact model.
   Completion certifies the design; prototypes do not become usable or saleable
   vehicles. A later specification change needs a separate development review.
4. Follow **Preparing production** and **Building company stock**. The company
   pays tooling, material and fabrication costs from its own settled working
   capital. Its available stock belongs to the company and adds no military
   capability or government maintenance bill.
5. When stock is available, choose **Review stock purchase** and a whole number
   of vehicles or aircraft. Review the exact model, supplier, price, remaining stock, delivery,
   maintenance and fleet need before confirming. Buying part of a lot leaves the
   rest with the manufacturer. An unmet fleet target does not place another order.
6. Track **Purchases & deliveries**. Shipping begins after the public payment
   settles and takes **seven accessible days**. On arrival, the exact revision
   enters the Arsenal and uses existing maintenance, ammunition, operations,
   compatible refit and retirement rules.

The company page also offers additional paid investment, development funding
changes, cancellation of unfinished development and a finite stock target.
**Inspect 3D model** shows a product's frozen configuration without replacing an
unfinished design. Viewpoint, finish and model exports create no orders.

Use **All equipment**, **Ground vehicles** or **Aircraft** to filter model offers
and the 3D inspection. The filter uses the model's served family and platform
catalogue, preserves original purchase identities and leaves supplier accounts
and paid deliveries visible. Search can further narrow the displayed records.

With ammunition products present, the market shows **All stock** and adds an
**Ammunition** filter. It shows physical supply offers without inventing a vehicle
model. Exact vehicle/aircraft previews and unfinished drafts remain separate.

## Supported equipment

The same contractor can license all **eleven existing designer platforms**:
four tank classes; infantry fighting vehicles, armored personnel carriers,
reconnaissance vehicles, self-propelled artillery and mobile air defense; and
light attack/tactical strike aircraft. This extends the supplier route, not the
component catalogue or combat missions. All products share the contractor's one
existing Arms Plant slot; an aircraft contract grants no specialized factory,
airfield, basing rights or additional simultaneous capacity.

Aircraft retain their exact eight-component specification and enter the Arsenal's
Air class only after delivery. They require compatible physical bombs and theatre
access for tactical raids. Purchasing an aircraft grants no bombs, ground strength
or new air-superiority/transport mission. The purchase review shows the exact air
profile, required compatible store family and zero included stores, with a link
to inspect mission stores before purchase. Ground specialist profiles retain their
existing roles. See [AVIATION.md](AVIATION.md) and
[EQUIPMENT_DESIGNER.md](EQUIPMENT_DESIGNER.md) for those service rules.

## Company-made ammunition

Open **Ammunition → Manufacturer stock** to authorize supply for a certified
compatible family, review finished offers and track purchases into national
stores. All 23 existing families use their current recipes and production rates.
The same state contractor pays input and fabrication costs using its existing
capital and one shared plant slot. No new public fee, development contract,
tooling stage or physical ammunition is granted by authorizing supply.

Set a company buffer of 1–1,000,000 whole rounds or mission stores initially;
later 0–1,000,000 is allowed. Zero stops restocking and retains unsold stock.
Equipment development gets first use of the daily plant packet; otherwise the
contractor serves equipment and ammunition in product order. There is no second
ammunition factory or simultaneous free output.

Offers explain compatible models and the reserve gap after national stores,
unfinished public batches and already paid company deliveries. Prices are the
finished stock's average paid inputs and fabrication plus 15%. Purchases use
**Maintenance & supply**, protecting today's unpaid actual upkeep under the
maintenance plan first. The plan must be active before buying. A paid company
receivable settles once, then seven accessible delivery days put the exact family
into the existing national magazine with a matching receipt.

Company supply stops new automatic public reserve batches only for that family.
Saved target/preferences, existing public work and paid deliveries are retained.
Neither the reserve nor company buffer authorizes automatic government buying.
Purchased ammunition grants no vehicles or ground activation and follows existing
combat consumption rules. [AMMUNITION.md](AMMUNITION.md) gives the complete bounds,
funding and migration rules.

## What each payment buys

| Payment | Account | Result |
| --- | --- | --- |
| Establishment or additional investment | Government Defense procurement → company | Separate working capital; no free factory or equipment |
| Development and trials | Government Defense R&D → company engineering expense | Certified frozen model; no saleable prototypes |
| Tooling, materials and fabrication | Company working capital | Production readiness and company-owned work/stock |
| Purchase of finished stock | Government Defense procurement → company | One government-owned delivery |
| Purchase of finished ammunition | Government Maintenance & supply → company, after protecting current upkeep | One paid delivery into the existing ammunition store |
| Service after arrival | Existing government support and ammunition systems | Actual fleet maintenance, readiness and usable stores |

Public payments consume departmental authority and use the existing daily
fiscal settlement once. Until settlement, the company shows a receivable rather
than spendable cash. Development receipts are matched by engineering expense;
they do not become free capital for producing equipment.

For manufacturing, the company purchases a complete physical recipe from the
domestic national warehouse at current modeled resource prices. The warehouse
loses those inputs, the company owns them as work in progress, and the national
seller receives the payment. The company also needs enough cash for a complete
vehicle's fabrication before starting its inputs. It cannot create missing
materials or borrow automatically. Review Resources and the company's blocker
if warehouse stock is unavailable; ordinary construction remains financially
funded under its existing rules.

The purchase price is the finished lot's average paid material and fabrication
cost plus a **15% modeled margin**. Development and tooling are not charged again
on each unit. Seller prices never change the frozen physical/combat profile.
The current company layer grants no additional GDP, employment or productivity
bonus. Prices, margins, times and inventory limits are game assumptions, not
sourced historical-company balances.

## Limits and delays

- One player-established state contractor per country, with exclusive use of
  one existing completed Arms Plant slot. It shares physical capacity with
  existing public manufacturing, custom equipment and ammunition work; it does
  not create a second factory.
- All eleven implemented ground/air platforms use this route. The company retains up to 32 product
  records and supports one unfinished development contract at a time.
- Vehicle/aircraft stock targets are initially 1–12 complete units per model. Later targets may be 0–12.
  A zero target stops new replenishment; already started fabrication can finish.
  Targets authorize company inventory, never government purchases.
- One leased slot performs one daily work packet. An unfinished development
  contract takes priority over restocking existing models. A zero development
  ceiling pauses that contract while preserving its work; finish or cancel it
  to release that development priority. Otherwise equipment and ammunition
  products share that packet in product order.
- Tooling and manufacture require facility access, company cash and real inputs.
  Existing paid public work retains priority if capacity becomes unavailable.
  Estimates depend on those conditions and are distinct from certification.
- Delivery pauses while its source province is inaccessible or outside the
  buyer's control. Paid vehicles retain their owner and resume on restored
  access. They add no usable strength or upkeep while in transit.
- Cancelling unfinished development ends future work without refunding completed
  engineering or trials. It does not close the company or return its leased slot.

Reviewed purchases are tied to current funding, facility and stock conditions.
A stale or invalid quote spends nothing; refresh and review again. Saves retain
cash, receivables, frozen licenses, partial work, company stock and paid deliveries.
Tank-only company ledgers retain **company version 1** and the
**`spheres-equipment-save` envelope version 2**. Starting the first specialist
ground or aircraft contract upgrades the company book to **version 2**, requiring
**save envelope version 3**. This does not rewrite existing tanks, paid work or
company balances. A loader refuses a mixed-family book disguised as an older
envelope rather than silently discarding its property or misclassifying aircraft.
Starting company ammunition supply upgrades to **company version 3/save envelope
4**, retaining the earlier corporate property and matched ammunition receipts.
Without that action, tank-only and mixed-equipment books retain their earlier versions.
These are separate from equipment-state and frozen-profile versions. An unused
company book remains absent, preserving legacy and equipment-only save shapes.

## Existing campaigns and next work

No historical company, opening company balance, new factory or starting stock is
invented. The unused company book is sparse. Existing public projects, inherited
equipment, paid shipments and refit reservations retain their owners and funding
history. New design actions for all eleven implemented platforms lead to the
supplier route; existing explicit public work retains its path. Establishment ends only the
background automatic catalogue buyer, not those explicit or already paid orders.

The wider conversion remains unfinished: supplier refit services,
broader inherited-equipment migration, AI buyers,
foreign purchases, exports and optional standing purchase plans come later.
Private/historical firms, competition, civilian companies, lending and corporate
failure rules need their own data and accounting work.

## Verification

**Current ammunition-supplier expansion: accepted.** The final release build,
simulator, web, UI and browser checks pass.

| Check | Result |
| --- | --- |
| Full release simulator and CLI integration suites | 941 passed; 67 ignored |
| Full Rust web suite | 251 passed; 3 ignored |
| Final affected equipment API checks | 50 passed; focused rerun |
| Full non-browser Node batch | 955 passed; includes the earlier 109 equipment checks |
| Final focused equipment UI checks | 110 passed; includes the suspended automatic-preference correction |
| Focused company-ammunition audit | 12 passed including the explicit QA exporter; 11 ordinary checks overlap the simulator suite |

Combined Rust integration totals are **1,192 passed, 70 ignored**. Focused reruns
overlap the full suites and are not added to their totals. The full Node batch
preceded the final localized confirmation-wording correction; the focused
110-check run verifies that correction without claiming a second full batch.

Browser acceptance bought 20,000 machine-gun rounds and 60 unguided mission stores
on 7 January in an isolated QA campaign. Saving before fiscal settlement and
restarting the final executable preserved the two paid deliveries. After
settlement and seven accessible delivery days, both arrived on 14 January,
observed in the browser on 15 January. National stores held exactly 20,000 rounds
and 60 mission stores with two native delivery records. Ground ammunition
activation remained off.

At 390×844, the page and dialog measured 390 pixels with no horizontal overflow.
The supplier shelf and reviews remained readable; filtered actions retained the
exact ammunition identity. The Companies Ammunition filter preserved accounts
and paid deliveries while showing no vehicle viewer. Final browser console
errors were zero.

Evidence logs in the parent workspace: `work/company-ammunition-sim-final.log`,
`work/company-ammo-web-all.log`, `work/company-ammo-web-final.log`,
`work/company-ammo-node-all.log`, `work/company-ammo-equipment-final.log` and
`work/company-ammo-build-final.log`. Verified final release SHA256:
`32B2690F481DEA8B88C09508B6DB9A31459615C19C7372B71927D6DF5F70F55F`.
Earlier accepted vehicle/aircraft evidence follows separately.

### Previously accepted ground/air expansion

The release build, simulator, web, UI
and mixed ground/air browser checks pass.

| Check | Result |
| --- | --- |
| Full Rust web suite | 247 passed; 3 ignored |
| Final affected equipment API checks after shared-queue estimate correction | 46 passed; focused rerun |
| Full non-browser Node batch | 947 passed; no failures or skipped checks |
| Focused equipment UI checks after final singular/plural wording fix | 101 passed; focused rerun |
| Final simulator and CLI integration suites | 929 passed; 66 ignored; 41 result groups; exit 0 |
| Focused company audit, including two explicitly run QA exporters | 24 passed; overlapping focused run |

The Node batch includes five new mixed-family filtering, exact-model,
draft-preservation and purchase-identity regressions. It excludes browser scripts
and the standalone mesh check. Its log is `work/company-family-node-all.log` in
the parent workspace; the final focused UI log is `work/company-family-equipment.log`.
The final simulator log is `work/company-families-sim-final.log`. Combined Rust
integration totals are **1,176 passed, 69 ignored**. Focused reruns are not added
to unique-suite totals; ignored checks are not counted as passes.

In an isolated QA campaign, actual daily simulation produced one IFV and one light
attack aircraft by 27 December 1991. The browser reviewed and bought the aircraft
for $9.547m with $2,540/day future maintenance, and the IFV for $2.436m with
$413/day future maintenance. The aircraft review showed its exact frozen air
profile, required compatible stores and zero included bombs before confirmation.

Saving both purchases before fiscal settlement retained two deliveries, two
supplier receivables and zero remaining offered stock in company book version 2 /
save envelope 3. Restarting the final executable and loading restored that state.
Eight actual daily advances delivered both exact revisions on 4 January 1992,
with $2,953/day combined maintenance requirement and no free ammunition.

Ground/Aircraft filters switched the exact 3D model and preserved an unfinished
local designer draft. At 390×844, the page and equipment room had no horizontal
overflow, filters were visible and the model remained usable. The browser console
was empty and the viewport was restored after inspection.

A tactical-strike development review also retained its known 687-day development
estimate while explaining that first stock had no dated estimate because older
aircraft restocking shared the same company slot. That review created no order.

Verified release SHA256:
`64FE73F1A4A757C0D54FEAF9318EE3A22EE2FE19DD936D1C04DD2AF9B482961E`.
The matching build was launched from `work/economy-preview/spheres-web.exe` on
the existing `http://127.0.0.1:7836/` address. The 11 February 1990 USA campaign
was retained with no company or equipment orders created, and Companies &
Procurement was opened for review. QA purchases stayed in isolated saves.
The evidence below records the earlier accepted tank milestone separately.

### Previously accepted tank milestone

| Check | Result |
| --- | --- |
| Rust simulator and CLI integration suites | 922 passed; 65 ignored |
| Rust web suite | 246 passed; 3 ignored |
| Focused company checks, including the explicitly run QA exporter | 19 passed; a focused run, not an additional unique-suite total |
| Full non-browser Node batch | 942 passed; no failures or skipped checks |

The Node run includes 12 new company UI regressions covering exact-model
inspection, draft preservation, quoted payments, stale actions, typed forms and
unavailable facilities. It excludes standalone mesh checks and browser scripts.
The combined Rust suite total is **1,168 passed, 68 ignored**; ignored tests were
not converted into passing checks.

Browser acceptance passed on the final release build. In an isolated synthetic
France campaign, the interface established Atelier Defense with $250m of working
capital and commissioned a twelve-component Atelier MBT at a $500k daily ceiling.
Saving after one day of engineering and restarting preserved $390.12k of actual
development expenditure; the next day advanced to two work days and $780.23k.

A separate fixture reached certification, tooling and two finished tanks through
real daily ticks. The browser set a fleet target of five, reviewed one tank at
$3.892m and $466/day future maintenance, and verified supplier stock 2 → 1 and
fleet shortfall 5 → 4. Confirmation created one paid delivery and one supplier
receivable. Eight daily advances crossed the year end and delivered exactly one
France-design-1 vehicle on 8 January 1991, with the quoted maintenance requirement.
The remaining tank stayed with the supplier, which began funding replenishment.

Desktop and 390×844 layouts passed visual inspection, including the exact-model
3D viewer and purchase form. The narrow page and equipment dialog had no
horizontal overflow; the workflow tabs scroll independently. Browser warning and
error logs were empty. Invalid stock, stale quotes, duplicate receipts, occupied
facilities and malformed saves are covered by the automated company checks.
These synthetic QA saves are separate from the player's campaign.

Evidence logs: `work/company-web-final.log`, `work/company-sim-all.log`,
`work/company-integration-final.log` and `work/company-node-all.log` in the parent
workspace. Final release SHA256:
`24D319D4D02A13DA07235FA2071AEBD617A601CCCF818DECC295E4B73C10158D`.
