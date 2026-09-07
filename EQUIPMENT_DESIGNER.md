# Equipment designer — ground vehicles and tactical aviation

Open **Research → Equipment designer** or the designer link in the manufacturing catalogue. The current implementation has nine ground platforms and two tactical-strike airframes, sixteen component-integration research projects and the paid design-to-service loop from the [military equipment plan](MILITARY_EQUIPMENT_DESIGNER_PLAN.md). The Ammunition tab adds compatible physical stores and funded manufacture for ground weapons and aircraft. [AVIATION.md](AVIATION.md) covers the first two-airframe slice; other air missions, naval design, licences, exports and designer AI remain later milestones.

## Play the loop

1. Choose a vehicle family and a starting configuration. All eleven platforms have valid starting designs using established components. Name the model and open its specification groups: tanks have twelve independent slots, ground specialists thirteen and aircraft eight. Every choice has its own installation load, fabrication price, maintenance requirement and relative rating effects.
2. Select a visible part on the 3D model, use **Inspect a visible part**, or select a specification directly. Review the ratings, installation space, development bill, fabrication price, maintenance need and minimum work times. Compare against a starting configuration, saved draft or published revision. Save an unfinished draft freely, including components you have yet to research.
3. Open the designer's **Research** tab to follow the chassis, engines, weapons, armor, optics and communications branches. Cards show prerequisites, unlocked parts and known/available/researching/locked states. Follow prerequisites across branches or open their ordinary technology research pages. Research uses existing Aerospace effort and unlocks components; it creates no vehicles and installs nothing on existing models.
4. Review development funding and confirm a daily limit. Defense's research department funds work over time. A published revision keeps its original configuration, prices and ratings. Changing the draft creates a different model that needs its own development.
5. Once certified, review a production batch. Choose a province with a free completed arms plant, a whole number of vehicles and a daily procurement limit. The quote shows tooling, work time and raw inputs. Tooling and fabrication use Defense procurement; physical inputs come from the existing resource market.
6. Completed vehicles travel through the Arsenal's delivery queue and then contribute to supported operations. The service page shows held and reserved vehicles, age, maintenance coverage and suggested modernization.
7. Develop a compatible revision and review a refit. A refit withdraws real source vehicles, consumes funding and replacement inputs, and returns the converted vehicles at their existing age. Chassis, airframe and main ground-weapon changes require new manufacture in this release.

Pausing keeps completed work. Cancellation refunds no sunk cost and retains completed products; unconverted reserved vehicles return to service. Project limits share the existing department pool. Increasing a limit cannot bypass engineering time, materials, annual budget renewal or factory space. Existing manufacturing lines retain their occupied slots if provincial capacity falls.

## Interactive 3D models

The 7 September art integration combines `feat/hoi4-map-and-tech` through
`fc0f0c2` with `feat/art-p0` through `c2e49c6`. It adds ground-model detail,
component-visible geometry and levels of detail while retaining the two current
aircraft and the simulation-owned design, cost and ammunition rules. See the
[integration record](ROADMAP.md#art-integration--shared-catalogue-sites-and-city-previews-2026-09-07)
and [asset backlog](docs/art/3D_ASSET_BACKLOG.csv) for the wider art work and limits.

The Designer displays actual WebGL ground-vehicle and aircraft geometry with perspective, lighting, depth and a ground shadow. Drag to orbit, scroll or pinch to zoom, or use the labelled view controls. With the canvas focused, arrow keys rotate, plus/minus zoom and Home resets the view. Auto rotation is optional and stops rendering while the preview is hidden. Clicking a visible part selects its actual geometry and opens the associated specification; the part selector provides a keyboard alternative.

The nine ground platforms have distinct hulls, running gear and mission fittings. Exterior choices change tracks or wheels, turret or weapon station, engine fixtures, armor, weapons, optics, troop access, scout masts, loading equipment and radar. The original models include individual track links, suspension and wheels, bevelled armor, hatches, grilles, stowage and hollow muzzles. Internal ammunition choices affect game ratings and exported metadata; visible ammunition lockers identify the associated specification. These are fictional representations of game components, not engineering models or reproductions of named historical vehicles.

Olive, sand and winter finishes are cosmetic. Camera and finish survive component and name edits. **Download 3D model** exports the current configuration, its specification IDs and finish as a self-contained GLB. Selection highlights are not exported. Twelve example assets and regeneration instructions are in [equipment-models](spheres-web/ui/equipment-models/README.md).

The renderer requires WebGL; if unavailable, design reviews and geometry downloads still work. The model uses local code and embedded vertex colors, without a CDN or external textures. Viewing, rotating, repainting and downloading create no simulation orders and change no country statistics.

The separate manufacturing catalogue displays 46 static equipment models from the existing Claude-assisted Arsenal work. Source and integration attribution are recorded in [tools/arsenal](tools/arsenal/README.md). The tactical aviation designer now covers two fictional airframes. Other catalogue aircraft and ship previews do not imply their component designers or missions are implemented.

Open **Construction → Manufacture → Browse equipment catalogue** to inspect the deck before an arms plant is completed. Class filters and the technology-locked toggle expose the full collection. Production choices remain unavailable until the existing research and free-plant requirements are met.

The two aircraft have eight visible, selectable specification groups. Engine, wing, radar, avionics, countermeasure, hardpoint, payload and endurance choices appear in their fictional geometry and exported metadata. Their component-based visuals share the same simulation configuration; the renderer computes no performance ratings.

## Comparison and suggested modernization

Comparison uses server-compiled ratings and the original frozen prices when the baseline is a published revision. Fabrication and maintenance are **per vehicle**; development is **per model programme**; tooling is **per production batch**. Minimum work times and installation load are shown separately. A comparison delta is a prospective design difference, not a national-budget forecast or a refit invoice. Review the actual project quote before ordering. Cross-role comparisons explain that one vehicle is not automatically a replacement for another.

Suggested modernization first flags maintenance coverage below 95% when delivered custom vehicles are available. It then checks known, compatible, single-component changes for up to eight available custom holdings. Candidates must improve the primary role rating by at least 0.025; the score discounts improvement by proportional increases in fabrication and maintenance cost. Tanks use land contribution, ground specialists use their mission rating, IFVs use the mean of fire support and protected mobility, and aircraft use supported strike effectiveness. The board shows at most five suggestions, including any maintenance priority.

This is a bounded suggestion search, not a complete fleet optimizer or affordability guarantee. It does not search multi-component rebuilds or forecast battlefield demand. Compatible upgrades still need development and a certified refit target; a main-weapon change is labelled a replacement requiring new manufacture. **Explore this design** opens a draft for comparison and places no order.

## Scope and economic ownership

The designer has **eleven platforms**: nine ground vehicles and two tactical-strike airframes. The four tank types retain their 36 choices across twelve independent specification slots:

| Group | Independent specifications |
| --- | --- |
| Engine and running gear | Engine (600/900/1,200 hp diesel or 1,500 hp turbine), manual/automatic transmission, standard/wide/padded tracks, torsion-bar/hydropneumatic suspension |
| Turret and armament | Compact/standard/large/autoloading turret or fixed casemate, 90/105/120/125 mm gun, mixed/penetrator/fire-support ammunition load |
| Protection | Standard/reinforced armor, no/soft-kill/hard-kill active protection |
| Observation and control | Day/night/thermal optics, basic/stabilized/digital fire control, field radio/tactical data integration |

The five specialist platforms each have thirteen slots. They share the same specification categories with compatible vehicle-specific components and one mission installation:

| Platform | Running gear and mission installation | Supported role |
| --- | --- | --- |
| Infantry fighting vehicle | Tracks and troop compartment | Fire support and protected ground maneuver |
| Armored personnel carrier | Wheels replace tracks; troop compartment | Protected ground maneuver |
| Reconnaissance vehicle | Wheels replace tracks; scout equipment | Better battlefield observation |
| Self-propelled artillery | Tracks and artillery loading | Indirect fire support |
| Mobile air defense | Tracks and radar | Protection against air raids |

Ground specialists add autocannons, machine-gun stations, howitzers, air-defense guns and missiles, compact managed engines, modular protection, troop compartments, scout packages, loading equipment and radar. Autocannons and machine guns require their matching mount and ammunition. Mobile missiles require their missile load, tracking radar and digital fire control; guided artillery ammunition also requires digital fire control. Every combination must fit its installation allowance.

The light chassis cannot carry reinforced armor, a large/autoloading turret, a 120/125 mm gun or a turbine. The 120 mm gun requires a large/autoloading turret or casemate; the 125 mm requires autoloading or casemate. Only tank destroyers accept the fixed casemate. Every type must also stay within its installation allowance. The server explains incompatible combinations before development can be funded; unfinished drafts can still be saved.

The **sixteen component research projects** comprise four tank, nine ground-specialist and three tactical-aviation integrations. The six illustrated branches also show established-system foundations and links to existing active-protection and tactical-data technologies; these cards are not additional component projects. Integration programmes require the existing submicron-electronics foundation. Advanced nodes also require completed component research: guided weapons combine medium weapons and fire-control knowledge, while networked command combines secure radios and sensor fusion. Armor and active protection can be fitted together. Observation and gun control remain independently selected and priced.

Prices, nominal horsepower/caliber labels, installation points, relative ratings, work times and the seven-day delivery period are explicit game assumptions, not historical vehicle specifications. Firepower, protection, mobility and observation combine into a bounded land contribution; there is no new projectile, anti-infantry or tactical terrain damage model. Ammunition loads change composite firepower, fabrication cost and maintenance needs, and identify the compatible physical store family. Installed load specifications do not grant rounds. Custom vehicles do not boost unrelated air or naval roles.

Specialist profiles also freeze four mission ratings, weighted in operations by supported, available physical Arsenal inventory:

| Specialist rating | Current operational consumer |
| --- | --- |
| Fire support | Improves attack at ground-operation rungs |
| Protected mobility | Improves seizing/control contribution |
| Reconnaissance | Improves the ground force's target-observation exposure calculation |
| Air defense | Reduces incoming rung-6 air-raid damage; it does not intercept ground-rung attacks |

Realized support, maneuver and observation bonuses cap at 25%; air-raid protection caps at 35%. A per-vehicle rating is not itself a national percentage bonus. Composition, committed force and maintenance determine its contribution. Orders and refit-reserved vehicles provide no coverage. Every new custom unit is one complete vehicle; the existing Armour Arsenal class stores these ground families while their frozen designs supply distinct roles. They create no aircraft, ships or additional overseas lift.

Physical ammunition can be activated for custom ground weapons after preparing compatible stores. Aircraft require physical mission stores from their first deployment, independently of ground activation. The Ammunition tab covers 23 store families (21 ground and two aircraft), finite paid manufacture and conserved combat consumption; inherited equipment retains its share of the shared magazine. There are no projectile-level penetration calculations. See [AMMUNITION.md](AMMUNITION.md) for the production, funding, compatibility and migration rules. Additional support-vehicle families remain later work.

Component research shares Aerospace's effort and monthly acquisition limit. Switching between normal research and component research retains half the previous effort once. Component-only discoveries are outside the 328-node economic frontier and grant no national productivity bonus. Existing technology effects mapped to replaceable tank hardware are neutralized for the custom-equipped force share; the fixed installed revision supplies its contribution.

Defense research becomes a work-funded department on the next funding day after the first model-development order. It stops auto-expensing that allocation. Development is an operating expense, not civilian investment. A fixed transition scale preserves the previously funded force-support baseline; moving money into research thereafter cannot also fund personnel and operations.

Existing campaigns retain allocation-based maintenance until the player confirms **Review actual maintenance plan** in In service. The new plan starts on the next funding date and replaces automatic Defense maintenance expense with an actual invoice for delivered inherited and custom stock. Custom vehicles use frozen upkeep; inherited equipment uses the explicit game assumption of 4% of catalog purchase value annually, spread over 365 days. Refit reservations are included; incoming orders are charged after delivery. This assumption is not a historical cost estimate.

Invoices share the existing departmental authority, prepaid funds and fiscal posting. Payment is capped by the actual bill, available funds and the selected daily ceiling. Insufficient payment is allocated proportionally across custom and inherited equipment, reducing supported combat coverage and the inherited magazine-refill role. Age and refit withdrawal remain separate constraints. Unused authorization stays in the department until normal fiscal expiry; it is not automatically refunded or reassigned. The invoice is recorded once after deliveries, with no duplicate treasury charge. The readiness board distinguishes last required/paid/unfunded amounts, funding and ceiling blockers, condition, ammunition and refit withdrawals.

Fabrication excludes raw input purchase costs. Inputs are consumed as work progresses; no cash-only construction rule is changed. Procurement funding, including eligible prepaid funds, is consumed once. New vehicles use whole quantities; fractional loss expectations accumulate in a saved residual. Refit vehicles cannot deploy while reserved. Fixed physical coverage, rather than the chosen purchase price, controls capability and military-industrial ranking.

The **Production supply plan** shows each production or refit's unconsumed material bill, its next eligible work requirement, warehouse stock and the next-work gap. Production and refit quotes show this before confirmation; invalid orders still show a known batch recipe when available. Quantities retain the physical unit printed on each row. Tooling consumes funding before fabrication starts, so a zero next-work material requirement during tooling does not mean the whole batch is supplied.

Project reviews account for priority, plant access, the project limit and available department funding. Earlier custom projects reduce the stock available to later ones; when those projects cannot work, their unused money remains available. The national target instead allocates shared funding once in priority order, assuming the needed inputs can be secured. It counts warehouse stock once; that stock is shared with civilian industry and other military consumers, not reserved. Paused programmes retain their outstanding bills but schedule no work. A new order performs no work on its issue date. The planning date is the next unsettled simulation date, not a promise that supplies or funding will remain unchanged.

Eligible custom production and refit demand also enters the national resource forecast as finite commitments. Resource-specific review links open the relevant commodity to inspect domestic production, incoming deliveries, other claims and supply choices. Opening a plan or following a link places no purchase order. A material shortage limits actual progress; work is charged only when funded work can consume its required inputs.

**Review material purchase** in Production proposes a finite purchase for a selected 30-, 90- or 365-day horizon and one-time spending cap. The plan nets shared stock, domestic supply, existing contracts and already-paid cargo against the custom work bill, including delayed cargo to avoid ordering the same requirement again. It uses current supplier surplus and operating reserves, open trade routes, shared freight capacity and available cash. The review shows suppliers, quantities, actual payment, estimated arrivals and unfilled demand. Confirmation uses the existing spot-market and freight ledgers, creates at most twelve shipments, and never borrows or creates a recurring order. Materials stay shared with other consumers and become usable only on arrival. Current spot prices include the complete modeled charge; there is no additional freight invoice in this model. Optional repeat purchases use a separately reviewed purchasing plan, described below.

The 30-, 90- and 365-day equipment estimates keep current GDP, allocations, project caps and eligible sites fixed. They assume inputs can be sourced, so an empty warehouse does not erase the demand that needs attention. Tooling delays material use. Ordinary funding stops at its fiscal authority boundary; eligible prepaid balances can continue funding work. Custom procurement claims reduce the same recurring procurement envelope rather than adding a second copy of that budget. These are conditional plans, not guaranteed deliveries or automatic orders.

## Tactical aviation

Light attack and tactical strike aircraft add eighteen components across eight slots and three research projects. They use the same paid development, manufacture, delivery, maintenance, refit and retirement owners. Their finite unguided/guided bomb stocks use the existing Ammunition production, reserve and material-purchasing flows.

Supported sortie output is folded once into the frozen bounded strike factor. Only available, maintained, correctly supplied aircraft contribute to rung-6 raids with theatre access; they provide no ground fire or transport lift. Ground ammunition activation remains separate. Coarse basing uses existing theatre consent, with no new runways, kilometer-range model or aviation-fuel ledger. [AVIATION.md](AVIATION.md) gives the exact scope, tradeoffs, migration and remaining mission families.

## Fleet service and retirement

**In service** now summarizes delivered custom vehicles, units available for operations, vehicles withdrawn for refit, completed vehicles in transit and unfinished manufacture. Current physical maintenance requirements are separate from the last settled coverage and earmark. The national composition figures use the same capability calculation as operations, including the inherited arsenal and recorded support; they are not per-vehicle statistics or battle predictions. Shortcuts open maintenance funding, production and the equipment library without issuing orders.

Each delivered model offers **Review vehicle retirement**. Choose a whole quantity of available vehicles and review the remaining inventory, national composition and maintenance requirement before confirming. Refit reservations cannot be retired. The confirmed command immediately removes only the selected quantity; survivors keep their age, condition and loss carry. Existing refits, deliveries, manufacturing contracts, design revisions and financial receipts remain intact.

Retirement has no cash or material refund and cannot be undone within the game. It reduces the physical fleet's future maintenance need and can lower future actual invoices; existing allocations and recorded payments stay unchanged. The comparison holds recorded support fixed; normal settlement recalculates support later.

**Fleet quantity targets** save desired counts for exact certified revisions, set from the Library. They count delivered vehicles, completed deliveries and unfinished manufacture once. Refits transfer quantities out of the source and into the target; cancelled work is excluded and paused or blocked commitments are labelled conditional. Shortfalls produce separately reviewed production or compatible-refit suggestions, protecting source targets before offering conversions. Zero is a valid desired count; stopping tracking removes only the preference. Targets never order, cancel, refit or retire anything automatically.

## Optional automatic material purchasing

Open **Production → Military supply purchasing → Set a purchasing plan**.
The default is manual purchases only. To authorize future purchases, select
automatic mode and review these settings:

| Setting | Effect |
| --- | --- |
| Production horizon | Cover finite funded work over the next 30, 90 or 365 days |
| Maximum per review | Limit cash spent at each scheduled review; zero buys nothing |
| Cash to leave available | Keep this amount outside this plan's buying allowance |
| Review interval | Review every 1, 7 or 30 days, starting after authorization |

The confirmation shows an illustration at today's cash and prices. It does not
buy immediately or lock a future price. Every scheduled review checks actual
funded vehicle, refit and ammunition orders, shared stocks, expected supply and
all paid cargo. Paused work and reserve targets without issued batches do not
request raw purchases. Reviews run after the day's bills and can spend only
available cash above your reserve, up to the chosen maximum. Other bills and
manual purchases can still use the cash reserve; it is a limit on this plan.

There is no borrowing, catch-up purchasing or accumulation of unused limits.
A review with no shortage, no cash or no supplier records why and waits for its
next scheduled date. Partial purchases show quantities still unfilled. The
latest 64 reviews retain their original limits, actual payments, suppliers and
dispatch estimates even after settings change. Inspect current cargo in
Resources for its present status.

Disabling or removing the plan stops future automatic purchases. Paid shipments
continue, and existing fabrication keeps its own funding controls. Material
payments remain separate from fabrication costs; buying inputs cannot create
factory capacity or renew an expired Defense allocation.

## Saves and compatibility

Campaigns that never use designer state keep the legacy world format and behavior. Saving a draft or starting component research introduces a versioned equipment envelope. Current builds preserve it in both raw simulation saves and campaign archives. Older loaders reject that structural envelope instead of silently discarding custom designs and inventories. Keep a pre-designer save if you intend to return to an older executable.

The loader validates frozen revisions, project references, bounded values and Arsenal reservations. It refuses unknown versions or contradictory equipment stock rather than inventing missing vehicles. The UI's reads and quotes do not mutate the campaign; confirmed orders use the normal session-bound command channel.

The current loader accepts equipment-state versions **1–8**. Version 4 adds optional maintenance plans and fleet quantity targets; version 5 adds physical ammunition, batch receipts and consumption accounting; version 6 adds ammunition reserve plans and dated scheduling receipts; version 7 adds optional material-purchasing policies and immutable dated purchase reviews; version 8 adds frozen tactical-aviation profiles. Missing fields preserve the old behavior. Validation rejects future-dated or contradictory invoices, altered proportional payments/support caches and targets referencing missing or uncertified revisions. Older loaders reject the new version. Original five-package tank revisions remain specification version 1; detailed tank revisions remain version 2. Their canonical identities, frozen prices, ratings and contracts are preserved. Editing a legacy model expands a new draft for fresh review and does not rewrite its source revision.

New ground-specialist revisions use specification version 3 with frozen mission ratings. Aircraft use specification version 4 with frozen supported-strike effectiveness, sortie rates and exact store families. Current mutations upgrade the library's state version to 8 while preserving old revisions. Validation rejects missing research prerequisites, mission ratings attached to tank revisions and missing or invalid ground ratings. Keep a backup before returning to an older executable.

The Ammunition tab supports manual reserve targets and separately authorized automatic replenishment. Plans net physical stock and unfinished matching batches before proposing a shortfall. Automatic scheduling waits for existing matching work, uses the saved site and new-batch ceiling, and starts only on a future eligible date. Clearing or disabling a plan stops future automatic orders; existing batches retain their own controls. An ammunition reserve plan itself never purchases raw inputs or grants stock. See [AMMUNITION.md](AMMUNITION.md) for the exact lifecycle and limits.
