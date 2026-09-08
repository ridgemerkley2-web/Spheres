# Companies, equipment development and procurement

7 September 2026 · planned direction; no company simulation is implemented yet.

Ridge wants the country to design equipment, put it through development with a
manufacturer, and buy the manufacturer's finished stock. Important national
companies should become part of the wider economy. This replaces government
production management as the intended default for new equipment acquisition.
It takes priority over expanding the aircraft/naval catalogue on the old path.

## The player experience

**Research → design → choose a manufacturer → fund development and trials →
company manufactures stock → purchase → delivery → service.**

1. **Design:** choose a tank class and its separate specifications. Keep the
   existing component research, comparison, 3D view and immutable revisions.
2. **Choose a manufacturer:** compare eligible firms by development quote,
   completion estimate, production readiness, indicative unit price and support.
   Eligibility requires the actual facilities, knowledge and manufacturing rights.
3. **Develop:** approve a finite development contract and spending ceiling.
   Engineering, prototype work and trials take funded time. Show milestones,
   remaining cost and the reason for any delay. A specification change makes a
   new revision; it does not rewrite a contract already under development.
4. **Wait for stock:** certification makes the model eligible for manufacture.
   The company pays for tooling and builds a bounded initial batch with its own
   working capital. Show "In development", "Preparing production", "Building
   first batch" and "In stock" separately, including the first-stock estimate.
5. **Purchase:** select available quantity, review the total price, delivery,
   upkeep and effect on fleet needs, then buy. The company loses that stock and
   the country receives an owned delivery. Only arrival adds usable equipment.
6. **Support:** existing service, ammunition compatibility, upgrades and retirement
   continue. Later, manufacturers can also sell ammunition and refit services.

Illustrative interaction, not starting data: a company has eight of your tank
model ready and the fleet needs twenty. Buy eight now or wait for restocking.
The screen explains the remaining shortage. It never silently turns the other
twelve into a government production order. Optional future standing purchases
may buy later stock within an expressly selected budget and price limit.

Capacity remains physical: plants, supply, capital and time determine output.
The company manages those decisions. The country's normal equipment controls
are design requirements, development contracts, purchases and fleet targets.
Ordinary construction remains financially funded as previously requested.

## Companies are actual suppliers

The first company implementation needs legal identity and ownership, real rights
to use facilities, cash, inputs, work in progress, finished inventory and dated
transactions. A company name attached to the current public production queue
would not deliver this loop.

- **Identity:** stable company ID, name, home country, private/state/mixed
  ownership and supported sectors. Legal home, ownership and factory location
  are separate facts.
- **Industrial assets:** owned or leased rights to existing province facilities.
  Each physical slot has one reservation owner across legacy public work, custom
  work, ammunition and company work. No free second factory under a new name.
- **Design rights:** the commissioning country keeps the design; the selected
  company receives explicit manufacturing rights to a frozen revision. Product
  identity includes the design's nation and revision, since revision numbers
  are currently unique only within each country. A product remains resolvable
  after a company closes or loses a license.
- **Finances:** working capital, receipts and expenses, finished-stock cost and
  operating results. State ownership does not make the firm's cash the treasury.
  Dividend, capitalization or subsidy transfers must be separately recorded.
- **Operations:** finite tooling and manufacturing batches, material purchases,
  inventory targets and a dated restocking forecast. Production stops with a
  readable reason when cash, supplies, capacity or access are unavailable.

Begin with domestic development and sales. Foreign licensing, imports and
exports follow after ownership and delivery work domestically. A country without
an eligible supplier sees the missing requirement; the game does not fabricate
a manufacturer or grant another country's research. Imports are a necessary
later route for countries without their own complete defense industry.

## Development, production and purchase are different payments

| Payment | Payer and purpose | Result |
|---|---|---|
| Component research | Existing national research allocation | Knowledge used by compatible designs |
| Development contract | Government Defense R&D; paid as actual contracted work progresses | Certified revision and agreed design/manufacturing rights |
| Tooling and inventory manufacture | Company working capital | Company-owned production readiness and finished stock |
| Equipment purchase | Government Defense procurement | Transfer of existing finished units into a delivery |
| Maintenance, ammunition and refit services | Existing appropriate service/procurement departments | Actual delivered support, supplies or conversion work |

Development fees pay engineering and trials, not free saleable tanks. Prototype
units are not fielded or listed for sale unless explicitly converted by later
rules. A recurring unit quote includes inputs, fabrication and the firm's margin;
the government does not also buy that company's raw materials. Do not charge
the full model development bill again on every unit. Display shipping and other
contract charges separately before purchase.

Companies need capital before their first sale. There are two explicit sources:
sourced opening private/state corporate balances with reconciled economic
ownership, or a paid investment to establish/capitalize a new company. The
initial implementation must not create cash from national GDP estimates, give
every firm unlimited credit, or require a hidden public payment per production
day. A player-created state company is a useful first supported path; it uses
an explicit one-time capitalization, then manages its own stock. Private firms
require the opening-data/accounting work below before being enabled in campaigns.

The first inventory policy builds a small finite buffer against licensed products
and evidenced demand, constrained by cash and a stock ceiling. Recent sales and
fleet needs can inform a forecast; fleet targets are not guaranteed orders or
permission to spend government money. No automatic borrowing in the first slice.
Advance purchases, guaranteed offtake, subsidized capacity and large bespoke ship
contracts can follow later as visible alternatives to buying ready stock.

## Screens and controls

- **Designer / Development:** retain specifications and comparisons; replace the
  final public production action with manufacturer selection and development
  contract review. Show engineering completion and first-stock availability as
  distinct dates. Estimates explain assumptions and blockers.
- **Equipment market:** browse exact models and manufacturers. Each offer shows
  ready stock, unit/total price, next restock estimate, delivery time, compatibility,
  upkeep and the fleet shortage it addresses. Show why an offer is unavailable.
- **Purchases and deliveries:** paid quantities, exact revision, supplier,
  shipment status and receipt. Failed or expired quotes spend nothing.
- **Companies:** start with defense firms. Company pages show facilities, products,
  stock, development work and meaningful financial results. Expand the same page
  structure to nationally important civilian firms as their simulation arrives.
- **Fleet and reserves:** targets remain advisory by default. Later optional
  buying plans specify spend cap, cash floor, price ceiling and review interval;
  count all owned stock and paid inbound deliveries before buying more.

The main presentation should answer "What can I get, when, for how much, and why
should I buy it?" Factory recipes and accounts belong in expandable explanations.

## Ownership and simulation contract

1. Every unit is in exactly one place: company work in progress, unsold company
   stock, reserved/purchased delivery, or government Arsenal. Unsold stock adds
   no military strength, fleet readiness, public upkeep or government inventory.
2. Sale preflight validates seller stock, buyer permission, price, delivery and
   funding together. Commit removes offered quantity once and creates one owned
   delivery with one receipt. Stale quotes and competing buyers cannot oversell.
3. Preserve the existing government fiscal owner. `programs::spend` consumes
   authority and `programs::finish_day` posts fresh expenditure once. Do not also
   directly debit treasury for the same purchase. Corporate receivables become
   spendable only through the corresponding settlement, including prepaid public
   funding rules. No temporary double-spend during the open day.
4. Company receipts go to company accounts. A domestic sale is not a second
   treasury credit; a foreign company sale is not automatically foreign-government
   revenue. Raw inputs have one owner too, transferred with recorded payment.
5. Existing nation-keyed resource and commerce APIs need explicit owner-aware
   settlement. Do not encode a company as a fake NationId or maintain duplicate
   national and corporate copies of the same physical stock.
6. Freeze the model profile, transaction price and cost basis separately. A higher
   seller markup never improves combat performance. Retain exact design identity
   and the existing fixed physical reference throughout delivery and losses.
7. Company output cannot add a second copy of inherited industry, employment or
   GDP. Attribute migrated production out of the existing inherited residual;
   account for value added once. Sales, revenue and profit are not three additions
   to GDP. Until that attribution is implemented, do not expose extra macro bonuses.
8. Use stable company/product ordering, command/tick mutations and the existing
   daily clock. Save/load must retain cash, licenses, reservations, partly completed
   work, unsold stock, invoices and inbound units exactly.
9. Factory occupation affects access to production, not automatic transfer of
   all corporate cash and IP. Bankruptcy, cancellation and transport disruption
   need explicit disposition of property and paid claims. The first release must
   block unavailable facilities and apply defined delivery/cancellation rules;
   more elaborate corporate succession can follow later.

## What we can reuse and what changes

| Current code | Use in the new direction |
|---|---|
| `equipment`, `equipment_specs`, ground/aviation compilers | Keep design, research, frozen specifications and validation; add contracted developer/license references |
| `manufacturing`, production province capabilities | Reuse location/control checks and shared physical capacity; move new manufacturing authority to firms |
| Equipment supply and resource markets | Reuse recipe and shortage calculations; add actual corporate input ownership and payment |
| `commerce` | Reuse the finite stock/transaction/delivery pattern; its nation-only settlement cannot serve firms unchanged |
| `arsenal`, operations, maintenance | Keep government delivered equipment and its real service/combat consequences |
| `programs`, `economy` | Keep departmental authority and the sole fiscal posting; add matching corporate receipts |
| `economic_ai` | Reuse bounded deterministic review ideas; it governs nations, so it cannot spend on behalf of a company unchanged |
| `starting_industry`, province economy | Preserve sourced aggregate baselines; add explicit corporate attribution without granting free assets |

## Delivery sequence and completion gates

### 1. Company ownership and funding foundation

Add the registry, explicit capitalization, facility rights and financial/physical
ledgers. Support a modeled player-established domestic contractor first. Prepare
a sourced 1990 company data schema; no invented historical balances or stock.
Pass money/asset conservation, shared capacity and save/load tests before sales.

### 2. One tank from design to the company's shelf

Implement manufacturer eligibility, a development contract, trials/certification,
manufacturing license, company-funded tooling, finite production and finished
inventory. Reuse existing tank components and 3D models. Demonstrate that a company
builds actual unsold units while the government's Arsenal and procurement spending
remain unchanged. Company costs must be paid and every completion reconciled.

### 3. Buy and field that tank

Deliver the market, reviewed purchase, supplier page and deliveries screen.
Buy part of a finite lot; verify the remaining listing and buyer delivery. Test
simultaneous/stale purchases, insufficient funds, unavailable facilities, date
boundaries, save/resume, cancellation and blocked transit. This is the first
complete playable milestone, covering steps 1–3 together.

### 4. Make the procurement loop the default

Carry the same company/purchase path across implemented ground and air equipment.
Add company-made ammunition, paid refit services and optional stock-buying plans.
Replace public raw-material automation for converted products with finished-goods
purchasing; do not keep both running. AI nations use the same offers, ownership,
budgets and delivery rules. Add imports/export permissions so small countries can
participate without building a domestic manufacturer.

### 5. Important national companies

Add verified 1990 firms and their ownership, capabilities and facilities, with
sources and explicit treatment of unknown financial/stock data. Do not backport
modern merged companies into 1990. Broaden from defense to energy, mining,
manufacturing, technology and transport as each sector gains actual inputs,
customers and output. Model employment, taxation, investment, trade exposure and
state stakes through those transactions. Banking, mergers, competition policy,
subsidies and nationalization follow when there are real accounts to act upon.

### 6. Further equipment and industrial depth

Resume additional aircraft, helicopters, drones and naval designs on this shared
supplier architecture. Add competitive development bids, multiple licensed
manufacturers, independent export products, component suppliers, service contracts
and explicit large-project purchase contracts where they improve player decisions.

## Migration and verification

Already funded public projects, paid deliveries, military holdings and refit
reservations keep their owners and payment history. Let existing contracts finish
under their old rules, clearly labelled; direct new acquisition through suppliers
once the replacement is complete. Never give the company public equipment and
charge the country to buy it back. Disable old automatic creation only for the
products whose new path is active, without deleting already paid cargo or work.

Version the new sparse company state and retain exact earlier frozen designs.
An unused company system must not perturb the old deterministic path. The live
USA campaign is disposable at Ridge's request if it blocks development; that does
not remove the game's obligation to validate saved ownership and paid contracts.

For implementation, require the relevant lifecycle, accounting and supply tests,
full Rust workspace checks, affected UI checks, and browser verification of
design → development → company stock → purchase → arrival → service. Include a
save/resume in the middle and a second buyer/insufficient-stock case. This planning
change has not modified runtime mechanics, opening data or the live campaign.
