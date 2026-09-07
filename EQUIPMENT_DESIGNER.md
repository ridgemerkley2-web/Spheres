# Equipment designer — ground vehicles and component research

Open **Research → Equipment designer** or the designer link in the manufacturing catalogue. The current release implements nine ground platforms, thirteen component-integration research projects and the paid design-to-service loop from the [military equipment plan](MILITARY_EQUIPMENT_DESIGNER_PLAN.md). Aircraft and naval design, separate ammunition stocks, licences, exports and designer AI remain later milestones.

## Play the loop

1. Choose a vehicle family and a starting configuration. All nine platforms have valid starting designs using established components. Name the model and open its specification groups: tanks have twelve independent slots; specialists have thirteen. Every choice has its own installation load, fabrication price, maintenance requirement and relative rating effects.
2. Select a visible part on the 3D model, use **Inspect a visible part**, or select a specification directly. Review the ratings, installation space, development bill, fabrication price, maintenance need and minimum work times. Compare against a starting configuration, saved draft or published revision. Save an unfinished draft freely, including components you have yet to research.
3. Open the designer's **Research** tab to follow the chassis, engines, weapons, armor, optics and communications branches. Cards show prerequisites, unlocked parts and known/available/researching/locked states. Follow prerequisites across branches or open their ordinary technology research pages. Research uses existing Aerospace effort and unlocks components; it creates no vehicles and installs nothing on existing models.
4. Review development funding and confirm a daily limit. Defense's research department funds work over time. A published revision keeps its original configuration, prices and ratings. Changing the draft creates a different model that needs its own development.
5. Once certified, review a production batch. Choose a province with a free completed arms plant, a whole number of vehicles and a daily procurement limit. The quote shows tooling, work time and raw inputs. Tooling and fabrication use Defense procurement; physical inputs come from the existing resource market.
6. Completed vehicles travel through the Arsenal's delivery queue and then contribute to supported operations. The service page shows held and reserved vehicles, age, maintenance coverage and suggested modernization.
7. Develop a compatible revision and review a refit. A refit withdraws real source vehicles, consumes funding and replacement inputs, and returns the converted vehicles at their existing age. Chassis and main-weapon changes require new manufacture in this release.

Pausing keeps completed work. Cancellation refunds no sunk cost and retains completed products; unconverted reserved vehicles return to service. Project limits share the existing department pool. Increasing a limit cannot bypass engineering time, materials, annual budget renewal or factory space. Existing manufacturing lines retain their occupied slots if provincial capacity falls.

## Interactive 3D models

The Designer displays actual WebGL ground-vehicle geometry with perspective, lighting, depth and a ground shadow. Drag to orbit, scroll or pinch to zoom, or use the labelled view controls. With the canvas focused, arrow keys rotate, plus/minus zoom and Home resets the view. Auto rotation is optional and stops rendering while the preview is hidden. Clicking a visible part selects its actual geometry and opens the associated specification; the part selector provides a keyboard alternative.

The nine platforms have distinct hulls, running gear and mission fittings. Exterior choices change tracks or wheels, turret or weapon station, engine fixtures, armor, weapons, optics, troop access, scout masts, loading equipment and radar. The original models include individual track links, suspension and wheels, bevelled armor, hatches, grilles, stowage and hollow muzzles. Internal ammunition choices affect game ratings and exported metadata; visible ammunition lockers identify the associated specification. These are fictional representations of game components, not engineering models or reproductions of named historical vehicles.

Olive, sand and winter finishes are cosmetic. Camera and finish survive component and name edits. **Download 3D model** exports the current configuration, its twelve or thirteen specification IDs and finish as a self-contained GLB. Selection highlights are not exported. Ten example assets and regeneration instructions are in [equipment-models](spheres-web/ui/equipment-models/README.md).

The renderer requires WebGL; if unavailable, design reviews and geometry downloads still work. The model uses local code and embedded vertex colors, without a CDN or external textures. Viewing, rotating, repainting and downloading create no simulation orders and change no country statistics.

The separate manufacturing catalogue displays 46 static equipment models from the existing Claude-assisted Arsenal work. Source and integration attribution are recorded in [tools/arsenal](tools/arsenal/README.md). Catalogue aircraft and ship previews do not imply their component designers are implemented.

## Comparison and suggested modernization

Comparison uses server-compiled ratings and the original frozen prices when the baseline is a published revision. Fabrication and maintenance are **per vehicle**; development is **per model programme**; tooling is **per production batch**. Minimum work times and installation load are shown separately. A comparison delta is a prospective design difference, not a national-budget forecast or a refit invoice. Review the actual project quote before ordering. Cross-role comparisons explain that one vehicle is not automatically a replacement for another.

Suggested modernization first flags maintenance coverage below 95% when delivered custom vehicles are available. It then checks known, compatible, single-component changes for up to eight available custom holdings. Candidates must improve the primary role rating by at least 0.025; the score discounts improvement by proportional increases in fabrication and maintenance cost. Tanks use land contribution, specialists use their mission rating, and IFVs use the mean of fire support and protected mobility. The board shows at most five suggestions, including any maintenance priority.

This is a bounded suggestion search, not a complete fleet optimizer or affordability guarantee. It does not search multi-component rebuilds or forecast battlefield demand. Compatible upgrades still need development and a certified refit target; a main-weapon change is labelled a replacement requiring new manufacture. **Explore this design** opens a draft for comparison and places no order.

## Scope and economic ownership

This release has **nine platforms**. The four tank types retain their 36 choices across twelve independent specification slots:

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

The **thirteen component research projects** comprise the four existing tank integrations and nine ground integrations. The six illustrated branches also show established-system foundations and links to existing active-protection and tactical-data technologies; these cards are not additional component projects. Integration programmes require the existing submicron-electronics foundation. Advanced nodes also require completed component research: guided weapons combine medium weapons and fire-control knowledge, while networked command combines secure radios and sensor fusion. Armor and active protection can be fitted together. Observation and gun control remain independently selected and priced.

Prices, nominal horsepower/caliber labels, installation points, relative ratings, work times and the seven-day delivery period are explicit game assumptions, not historical vehicle specifications. Firepower, protection, mobility and observation combine into a bounded land contribution; there is no new projectile, anti-infantry or tactical terrain damage model. Ammunition loads change composite firepower, fabrication cost and maintenance needs, without introducing a separate consumable ammunition stockpile. Custom vehicles do not boost unrelated air or naval roles.

Specialist profiles also freeze four mission ratings, weighted in operations by supported, available physical Arsenal inventory:

| Specialist rating | Current operational consumer |
| --- | --- |
| Fire support | Improves attack at ground-operation rungs |
| Protected mobility | Improves seizing/control contribution |
| Reconnaissance | Improves the ground force's target-observation exposure calculation |
| Air defense | Reduces incoming rung-6 air-raid damage; it does not intercept ground-rung attacks |

Realized support, maneuver and observation bonuses cap at 25%; air-raid protection caps at 35%. A per-vehicle rating is not itself a national percentage bonus. Composition, committed force and maintenance determine its contribution. Orders and refit-reserved vehicles provide no coverage. Every new custom unit is one complete vehicle; the existing Armour Arsenal class stores these ground families while their frozen designs supply distinct roles. They create no aircraft, ships or additional overseas lift.

Shared national munitions still supply all weapons. There are no separate shell or missile inventories or projectile-level penetration calculations. Separate consumable accounting and additional support-vehicle families remain later work.

Component research shares Aerospace's effort and monthly acquisition limit. Switching between normal research and component research retains half the previous effort once. Component-only discoveries are outside the 328-node economic frontier and grant no national productivity bonus. Existing technology effects mapped to replaceable tank hardware are neutralized for the custom-equipped force share; the fixed installed revision supplies its contribution.

Defense research becomes a work-funded department on the next funding day after the first model-development order. It stops auto-expensing that allocation. Development is an operating expense, not civilian investment. A fixed transition scale preserves the previously funded force-support baseline; moving money into research thereafter cannot also fund personnel and operations.

Maintenance in this first release **earmarks part of the existing, already-paid Defense maintenance service allocation**. The same amount is removed from legacy magazine support. It is not a second cash invoice. Insufficient allocation reduces custom equipment's readiness contribution. Replacing the whole maintenance service allocation with actual fleet invoices remains a later accounting milestone.

Fabrication excludes raw input purchase costs. Inputs are consumed as work progresses; no cash-only construction rule is changed. Procurement funding, including eligible prepaid funds, is consumed once. New vehicles use whole quantities; fractional loss expectations accumulate in a saved residual. Refit vehicles cannot deploy while reserved. Fixed physical coverage, rather than the chosen purchase price, controls capability and military-industrial ranking.

The new batch recipe is not yet included in the automatic national resource-demand forecast. Use **Review raw inputs** to arrange additional supply through Resources; a shortage pauses the programme without charging unfunded work. Automatic designer-specific replenishment is later integration work.

## Saves and compatibility

Campaigns that never use designer state keep the legacy world format and behavior. Saving a draft or starting component research introduces a versioned equipment envelope. Current builds preserve it in both raw simulation saves and campaign archives. Older loaders reject that structural envelope instead of silently discarding custom designs and inventories. Keep a pre-designer save if you intend to return to an older executable.

The loader validates frozen revisions, project references, bounded values and Arsenal reservations. It refuses unknown versions or contradictory equipment stock rather than inventing missing vehicles. The UI's reads and quotes do not mutate the campaign; confirmed orders use the normal session-bound command channel.

The current loader accepts equipment-state versions **1, 2 and 3**. Original five-package tank revisions remain specification version 1; detailed tank revisions remain version 2. Their canonical identities, frozen prices, ratings and contracts are preserved. Editing a legacy model expands a new draft for fresh review and does not rewrite its source revision.

New ground-specialist revisions use specification version 3 with frozen mission ratings. Current mutations upgrade the library's state version to 3 while preserving old revisions. Validation rejects missing research prerequisites, mission ratings attached to tank revisions and missing or invalid ground ratings. Older equipment loaders reject the unsupported state version. Keep a backup before returning to an older executable.
