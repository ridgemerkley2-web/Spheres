# Companies & Procurement

7 September 2026 · first domestic tank implementation accepted in Rust, UI and
browser checks. This guide describes the implemented mechanics,
not a claim that the full [company roadmap](COMPANIES_AND_PROCUREMENT_PLAN.md)
has shipped.

The country designs the tank and funds development. A manufacturer pays to
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
2. Open **Designer** or choose a saved model. Configure one of the four tank
   platforms and review its components, ratings and costs. Choose the manufacturer
   and review a development ceiling and initial company stock target. An unfinished
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
   of tanks. Review the exact model, supplier, price, remaining stock, delivery,
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

## What each payment buys

| Payment | Account | Result |
| --- | --- | --- |
| Establishment or additional investment | Government Defense procurement → company | Separate working capital; no free factory or tanks |
| Development and trials | Government Defense R&D → company engineering expense | Certified frozen model; no saleable prototypes |
| Tooling, materials and fabrication | Company working capital | Production readiness and company-owned work/stock |
| Purchase of finished stock | Government Defense procurement → company | One government-owned delivery |
| Service after arrival | Existing government support and ammunition systems | Actual fleet maintenance, readiness and usable stores |

Public payments consume departmental authority and use the existing daily
fiscal settlement once. Until settlement, the company shows a receivable rather
than spendable cash. Development receipts are matched by engineering expense;
they do not become free capital for producing tanks.

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
on each tank. Seller prices never change the frozen physical/combat profile.
The current company layer grants no additional GDP, employment or productivity
bonus. Prices, margins, times and inventory limits are game assumptions, not
sourced historical-company balances.

## Limits and delays

- One player-established state contractor per country, with exclusive use of
  one existing completed Arms Plant slot. It shares physical capacity with
  existing public manufacturing, custom equipment and ammunition work; it does
  not create a second factory.
- Tank platforms only in this first route. The company retains up to 32 product
  records and supports one unfinished development contract at a time.
- Initial stock targets are 1–12 tanks per model. Later targets may be 0–12.
  A zero target stops new replenishment; already started fabrication can finish.
  Targets authorize company inventory, never government purchases.
- One leased slot performs one daily work packet. An unfinished development
  contract takes priority over restocking existing models. A zero development
  ceiling pauses that contract while preserving its work; finish or cancel it
  to release that development priority.
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
The company ledger is version 1, carried in the **`spheres-equipment-save`
envelope version 2** whenever corporate property exists. This prevents an older
company-unaware loader from silently dropping company assets. The current loader
rejects corporate property disguised as a raw or version-1 save. An unused
company book remains absent, preserving legacy and equipment-only save shapes.

## Existing campaigns and next work

No historical company, opening company balance, new factory or starting stock is
invented. The unused company book is sparse. Existing public projects, inherited
equipment, paid shipments and refit reservations retain their owners and funding
history. New tank design actions lead to the supplier route; other vehicle
families retain their explicit public work paths. Establishment ends only the
background automatic catalogue buyer, not those explicit or already paid orders.

The wider conversion remains unfinished: other ground vehicles and aircraft,
company-made ammunition, supplier refit services, broader migration, AI buyers,
foreign purchases, exports and optional standing purchase plans come later.
Private/historical firms, competition, civilian companies, lending and corporate
failure rules need their own data and accounting work.

## Verification

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
