# Military research and equipment designer

Spheres product and implementation plan · 6 September 2026

Status: proposed design, based on the current working tree. This document does
not describe a feature already implemented. It carries forward the request for
a full component-based military research system and equipment designer.

## 1. The experience we are building

You decide what your country needs, research the necessary components, combine
them into a named equipment model, fund its development, manufacture it, and
watch it enter service. Years later, you can improve that model, refit suitable
equipment, export it, or replace it. Every step explains its cost, time,
limitations, and effect on your forces before you commit.

The core sequence is:

**Need → component research → equipment design → development and trials →
production → fielding → maintenance, refit or retirement.**

A new radar does not instantly improve every aircraft. Research makes the
component available; you still need a compatible design and a funded route to
put it into service. An expensive design can lose to an affordable, well-supported
fleet. Keeping a useful older model in production is a legitimate decision.

The full scope includes land vehicles, aircraft, helicopters, ships,
submarines, missiles, infantry equipment, and military support systems. Guided
presets and advanced editing use the same rules. The player should never need
to design every truck or learn an aircraft engineering spreadsheet to govern.

The first playable delivery will take one land-vehicle family through the
entire loop. It is a foundation for the full designer, not the definition of
the final scope.

## 2. What we take from Millennium Dawn

The useful pattern is a researched platform with compatible, researched
modules that form a persistent named variant. The project's own variant
reference documents separate platform technology, enabling module technology,
and slot compatibility. Spheres should retain those distinctions, while using
its own interface, data and simulation. [Millennium Dawn variant reference](https://github.com/MillenniumDawn/Millennium-Dawn/blob/main/.claude/docs/oob-variants-reference.md)

Presets must also serve the AI. Millennium Dawn's development documentation
connects equipment role templates, available modules, and production choices;
Spheres needs equivalent coverage so small countries can participate without
hand-authored designs for every possible combination. [Millennium Dawn AI equipment reference](https://github.com/MillenniumDawn/Millennium-Dawn/blob/main/.claude/docs/ai-equipment-reference.md)

These are references to development documentation inspected on 6 September
2026, not a claim that every development feature is in a particular released
version. The following mechanics are proposals for Spheres. We will author
Spheres content and visuals rather than import the mod's scripts or artwork.

## 3. The existing game and the missing connections

The source code takes precedence over older design documents and stale comments.

| Existing system | What exists today | What the designer must add |
|---|---|---|
| Research | Eight domains, stable technology IDs, prerequisites, year floors, focus, allocation, diffusion and research points | Military browsing categories, component unlocks, additional component research, accurate switching previews |
| Equipment | 46 static definitions in `arsenal::DECK`, six broad classes, prices, service life, orders and holdings | Platforms, modules, immutable named designs and revisions |
| Manufacturing | Located arms-plant lines, priority, Defense procurement funding, resource constraints | A design revision as the line's product; tooling, repeat deliveries and conversion rules |
| Government finances | Daily actual-expenditure accounting and five Defense departments | Explicit development and lifecycle payments using those same departments |
| War | Conserved national deployment and losses; land, strike and lift composition factors | Bounded, role-specific capability from fielded equipment |
| Stock | Quantities, average age and pending deliveries | Design identity, service/refit/reserve status, conserved batches and consumables |
| Formation control | National forces and theatre commitments | Equipment requirements and allocation packages; individual division entities are not currently implemented |

Three connections are mandatory before this becomes more than a visual designer:

1. The design produced by a factory must be the exact revision that enters stock.
2. Its supported equipment roles must affect the simulation that resolves combat.
3. Its development, production and upkeep must appear once in government spending.

## 4. Decisions this plan makes

- Keep the January 1990 start and daily simulation. Inherited equipment remains
  useful. Later eras introduce possibilities without scripted national outcomes.
- Build one equipment system shared by land, air and naval design. Aircraft
  and ships receive their own slots and rules within it.
- Use research effort, existing departmental funding, industrial sites and time.
  Do not add separate Army/Air/Navy experience currencies or design tokens.
- Editing, comparing, renaming and saving a draft are free. Development,
  production, conversion and refitting spend real allocated funding.
- Keep construction financially funded as already requested. Buildings do not
  regain material start requirements. Equipment manufacturing retains its
  existing supply dependencies, summarized in the designer's cost and availability
  panel and expandable for detail.
- Separate platform research, reusable components, model development and factory
  tooling. A repeat production unit does not repeat the model's entire R&D history.
- Research unlocks hardware; fielded hardware supplies its capability. Doctrine
  and training affect how it is used. They must not award the same bonus twice.
- Preserve manual choice. Suggestions explain the need and tradeoff; automatic
  design and procurement are explicit staff policies with spending ceilings.
- Design ship classes and aircraft models, without requiring individual-ship
  tactical control or a new combat clock.

When implementation begins, update BIBLE/SPEC to record the requested equipment
and ship-class design scope alongside the older restrictions. The user's new
direction authorizes this planning change; it does not require another
confirmation merely because an older document discussed refusing that detail.
Keep the existing national/theatre command model explicit in the amendment.

## 5. Research navigation and progression

Research gains a **Military** view alongside the existing scientific-domain
view. Both show the same authoritative discoveries and progress. Military
categories are browsing filters, not eight additional research budgets.

| Military section | Main research branches | What it enables |
|---|---|---|
| Land vehicles | Chassis, mobility, protection, vehicle weapons, optics and fire control | Tanks, IFVs, APCs, reconnaissance and specialist vehicles |
| Infantry and support | Individual equipment, crew weapons, communications, medical and engineering systems | Standardized infantry and support equipment packages |
| Artillery and air defence | Gun systems, rocket launchers, detection, coordination and defensive systems | Towed/self-propelled artillery, rocket artillery and air-defence batteries |
| Combat aviation | Airframes, propulsion, avionics, sensors, protection and mission integration | Fighters, attack aircraft, bombers and carrier variants |
| Helicopters and uncrewed systems | Rotorcraft, control systems, sensors and mission packages | Attack/transport helicopters, reconnaissance and support drones |
| Naval systems | Surface hulls, propulsion, sensors, combat systems and support installations | Patrol vessels, escorts, major surface combatants and support ships |
| Submarines | Hull families, propulsion, quieting, sensors and compatible mission systems | Coastal/oceanic submarines and their upgrades |
| Missiles and shared electronics | Conventional munition families, guidance categories, communications, electronic warfare and defensive countermeasures | Compatible weapon stocks and cross-platform component families |

Doctrine is a separate adjacent view. It may unlock force policies and
organizational choices; it is not another interchangeable vehicle module.

### Technology nodes

Each node shows:

- What component family or platform it unlocks, and which roles can use it.
- The prerequisite chain, earliest availability, current research and remaining work.
- Whether the country knows it, can research it, has licensed access, or only
  possesses imported equipment using it.
- The difference it could make to saved designs, with **Open compatible designs**.
- A server quote for changing focus, including any research progress lost on
  switching. The current generic ETA cannot be reused blindly: focus switching
  retains only part of the existing bank.

Filters include **Available now**, **Needed by my designs**, **In progress**,
**Known**, and **Future**. A dependency button reveals the missing prerequisite
in the same research screen. A planned sequence pauses at a blocked prerequisite
and explains why; it does not silently change spending allocations.

### Shared discoveries and new component research

The first slice reuses existing discoveries wherever they are a valid match:
`aero_pulse_doppler_radar`, `aero_aesa_radar`, `aero_tactical_datalink`,
`aero_network_centric_warfare`, `aero_stealth_shaping`,
`aero_quiet_submarine`, `aero_air_independent_submarine`, and
`aero_active_protection_system`, plus relevant materials and computing foundations.
Existing dates are game data to audit, not assertions that all countries had the
capability at that date. A civilian engine discovery is not automatically a
qualified combat-aircraft engine.

Fine-grained military research is required for the finished system. Add missing
chassis, powertrain, weapon, optics, naval and support families after introducing
an explicit discovery classification:

- **Foundation:** existing scientific knowledge; retains its documented economic effects.
- **Component:** reusable hardware knowledge; unlocks modules and their compatibility.
- **Integration:** platform-family knowledge; enables a new combination or role.

Component/integration discoveries use the existing research allocation and
prerequisite machinery. They do not automatically count as a new macroeconomic
frontier revelation or add generic national military strength. Every node has
one owning domain; cross-domain prerequisites do not charge the same discovery
twice. Retain the existing domain banks and acquisition pacing until deliberately
rebalanced; cheaper component nodes must not evade acquisition limits.

This requires separating economic frontier/adoption accounting from raw known-node
counts. The current 328-node registry and nonempty-effect assumptions must become
explicit validated contracts for discovery kinds, not be removed to make content
load. Do not append a large military tree before this separation is tested.

### Time periods

Use an inherited pre-1990 foundation, then 1990s, 2000s, 2010s and contemporary
families. Later branches are labelled prospective game content, with conservative
availability assumptions. An era is a browsing aid; it is not a free worldwide
unlock. Owning imported hardware does not grant its manufacturing knowledge.

## 6. The equipment model

Every design combines these layers:

1. **Role:** what the equipment is meant to do.
2. **Platform:** chassis, airframe, hull, or standardized equipment package.
3. **Required systems:** the minimum functioning configuration for that family.
4. **Optional systems:** specialization and useful tradeoffs.
5. **Compatible loadouts:** mission stores or consumables the finished platform
   can use; these are not granted as free stock by saving a design.

The player can start from an existing model, a staff preset, or a blank platform.
Slots have readable names and accept compatible categories only. Platform limits
use clearly labelled game ratings for space, load, power and complexity; they
are not engineering blueprints or claims about real vehicle dimensions.

Changing a role should reveal the requirements it introduces. A transport cannot
retain full cargo capacity after filling its available space with other systems.
A ship cannot have unlimited weapon installations. An unsupported combination
is refused with an explanation, not silently ignored.

### Full platform coverage

| Family | Models the designer must support | Main configurable groups | Core tradeoff |
|---|---|---|---|
| Tanks | Main battle and light/reconnaissance tanks | Chassis, main system, protection, mobility, optics, communications, optional protection | Protection/firepower versus cost, mobility and support burden |
| Infantry vehicles | Tracked/wheeled IFVs and APCs | Mobility, troop space, protection, mission system, sensors, communications | Carrying troops versus protection and combat specialization |
| Reconnaissance vehicles | Scout cars, tracked scouts and reconnaissance variants | Mobility, observation, communications, light mission system | Information and reach versus survivability |
| Artillery | Towed and self-propelled guns; rocket artillery | Platform, weapon family, mobility, fire coordination, support package | Sustained support versus responsiveness, expense and ammunition burden |
| Ground air defence | Mobile guns, short/medium/long-range defensive batteries | Carrier, detection, control, interceptor family, protection | Coverage and specialization versus mobility and operating cost |
| Utility vehicles | Trucks, logistics, recovery, engineering and medical vehicles | Chassis, cargo/service module, mobility, protection | Support throughput versus cost and combat survivability |
| Infantry packages | Light, mechanized, airborne and specialist equipment sets | Individual equipment, optics, communications, crew weapons, protection, support tools | Carry burden and affordability versus protection and support |
| Fighters | Air-superiority, multirole and carrier-capable models | Airframe, propulsion, radar, avionics, mission integration, protection, stores compatibility | Mission effectiveness versus range, payload and sustainment |
| Strike aircraft | Close-support, tactical strike and bomber models | Airframe, propulsion, navigation, sensors, mission package, protection | Payload and endurance versus survivability and expense |
| Support aircraft | Transport, tanker, early warning, maritime patrol and electronic support | Airframe, propulsion, cargo/service capacity, sensors, mission systems | Support capacity versus specialized equipment and operating burden |
| Helicopters | Attack, utility, transport, maritime and reconnaissance | Rotorcraft family, propulsion, cabin/mission package, sensors and protection | Lift/endurance versus protection and specialization |
| Uncrewed aircraft | Reconnaissance, persistent observation, support and strike roles | Platform family, control architecture, sensors, mission package | Cost/persistence versus resilience and support needs |
| Small surface ships | Patrol, coastal combat and corvettes | Hull, propulsion, sensors, mission installations, defensive systems | Affordable local presence versus endurance and versatility |
| Escorts and major combatants | Frigates, destroyers and cruiser-sized classes | Hull, propulsion, combat system, sensors, mission installations, aviation facilities | Escort/defence/strike specialization versus cost and crew burden |
| Aviation and amphibious ships | Carrier and amphibious support classes | Hull, propulsion, aviation or landing capacity, command and defensive systems | Projection capacity versus escort and maintenance requirements |
| Naval support | Replenishment, transport, mine countermeasure and specialist support classes | Hull, propulsion, cargo/service installation, defensive systems | Fleet support versus direct combat capability |
| Submarines | Conventional coastal/oceanic and nuclear-powered families where eligible | Hull, propulsion category, quieting, sensors, mission systems | Endurance and survivability versus cost and national support capability |
| Conventional munitions | Air/surface/ground-launched mission families and defensive interceptors | Role, compatible launcher family, guidance category and support class | Mission suitability, stock depth and cost |
| Shared support systems | Communications, electronic support, command, surveillance and counter-drone packages | Platform-specific installation, interoperability and support | Coordination and resilience versus integration and maintenance burden |

Strategic deterrence remains governed by the existing national rules. A vehicle
or missile design must not silently grant nuclear status, alter the nuclear
taboo, or replace the existing proliferation system. Military space support can
reuse the shared systems model in the later roadmap; it is not required for the
first conventional-equipment release.

### Component catalogue rules

Each component record needs a family, generation, compatible roles/slots,
prerequisites, origin/access rules, required installation ratings, contribution
to supported capability, cost contribution, production demands, upkeep,
integration difficulty, and refit compatibility. Give each a plain explanation
of what improves and what becomes more expensive or difficult.

The catalogue must cover mobility/propulsion, protection, main mission systems,
sensors/optics, communications, control/avionics, electronic protection,
cargo/troop/service capacity, and family-specific support installations.

Research a shared technology once, but use platform-specific implementations.
The knowledge behind a sensor may serve several families; a ship installation
cannot be dropped unchanged into a helicopter. Higher tiers generally offer new
options, not universally better numbers at the same cost.

## 7. What the designer screen looks like

Entry points: **Research → Military → Design equipment**, the equipment library,
an existing production line, a fielded model, or an advisor's modernization need.

| Area | What it contains |
|---|---|
| Header | Model name, family, role, revision, origin, draft/development/service status |
| Left panel | Platform and named component slots; missing/locked choices stay explained |
| Centre | Clear vehicle silhouette with selected systems highlighted; visual changes where assets exist |
| Right panel | Role fit, cost, time, sustainment, compatibility and comparison with the selected baseline |
| Bottom action bar | Save draft, compare, missing research, fund development, or create production plan |

Use original silhouettes or authored assets, with the same readable style as
the rest of Spheres. Initial artwork may be two-dimensional. A full 3D assembly
viewer is optional polish and must not delay functioning research and production.

**Quick design** asks for a role and preference: affordable, balanced, advanced,
or maximum compatibility with current stock. It shows the proposed components
and reasons before anything is funded. **Advanced design** exposes every valid
slot, with search and comparisons. These are two views of the same model.

The standard summary should answer six questions:

1. Can we develop and build it with our present knowledge and access?
2. What role will it fill, and what existing equipment would it replace?
3. What does development cost, and what will each produced unit cost?
4. How soon can the first unit and the requested batch reach service?
5. Can our current force-support and procurement plans sustain it?
6. What do we give up compared with the current model?

Exact numerical detail stays expandable. Show a small set of role-relevant
ratings first, rather than a wall of every possible statistic. Avoid an opaque
overall "best design" score. The right panel updates from a pure server preview.

On narrow screens, keep the role, validity and action bar accessible; switch
between Components, Comparison and Funding. Never hide a required explanation
inside a hover-only tooltip. Keyboard users can traverse slots and select modules.

## 8. Pre-commit effects and useful comparisons

Every meaningful change shows **current model → proposed model** and a reason.
For instance, an optional protection system might improve survivability while
raising unit cost, development work and maintenance demand. The actual values
must come from the same rule resolver used during play.

The comparison has three distinct views:

- **Per unit:** capability, compatibility, cost, production time and upkeep.
- **For the procurement plan:** development/tooling cost, planned spending,
  first delivery, batch completion, supply bottlenecks and fleet operating burden.
- **For the force:** the portion that would actually be replaced, supported
  roles, units temporarily withdrawn for refit, and readiness once delivered.

Before a production province is selected, national figures are estimates under
stated assumptions. After selection, show actual plant eligibility, occupied
slots, control/occupation constraints and local production effects. Do not
invent a provincial GDP or employment gain. Time forecasts are conditional on
funding and supply, and must identify the limiting factor.

Budget figures use the existing game currency basis consistently. Show the
extra daily and annual operating commitment separately from one-time work.
Speculative battlefield success is never presented as a guaranteed percentage.

### Three example decisions

The names below are fictional player-created models, not historical claims or
pre-calibrated equipment specifications.

| Example | Player decision | What the game must explain |
|---|---|---|
| Sentinel 90 tank → Sentinel 90B | Keep the chassis and main system; choose improved observation and compatible protection | Development/refit work, changed survivability/support, number of existing vehicles eligible, and downtime |
| Kestrel multirole aircraft | Compare a common affordable fit with a newly researched sensor and mission package | Compatible airframe/propulsion, unit and fleet cost, support needs, operational roles, and whether existing aircraft can be upgraded |
| Harbor-class escort | Choose between a coastal escort configuration and a more specialized fleet-supporting fit | Mission coverage, endurance/support tradeoffs, eligible yard, construction/refit duration, and equipment or stores still to procure separately |

The same new component can suggest several compatible revisions. Researching it
does not auto-select the expensive option, retrofit the fleet, or cancel the old
production line. The player sees **keep building**, **upgrade new production**,
and **refit eligible stock** as distinct choices.

## 9. Design states, revisions and development

**Draft → valid concept → funded development → testing → certified design →
production/service → superseded or retired.**

- Drafts may include locked components so the player can plan a future model.
  Missing requirements prevent development, not saving or comparison.
- Funding development freezes a particular revision and opens a visible project.
  A later edit creates a new revision; it does not rewrite the one being tested.
- Development work reflects platform novelty, unfamiliar components and
  integration changes. A minor compatible upgrade takes less work than a new family.
- Development has both a funding rate and a minimum elapsed-work schedule.
  Unlimited cash cannot complete a novel aircraft overnight.
- Research centres can support testing when actually eligible. The system must
  work independently of the Economic Competition feature flag. A basic national
  development route remains available; local laboratories improve a specified
  capability rather than become an unexplained mandatory building.
- Testing uses deterministic completion and recorded constraints. Reliability
  and maturity are visible modeled properties, not hidden random design failures.
- Pausing preserves progress and spends nothing further. Cancellation ends the
  commitment; already paid work remains an expense. Any reusable engineering
  credit is explicitly recorded and capped, not a cash refund.
- Certification enables production. Staff approval or a UI checkmark alone does
  not create equipment, field strength or free technology.

Published revisions are immutable. Renaming changes a display label only.
Saving, duplicating, reloading or changing the name cannot reset development
cost, certification, production learning or service age.

## 10. Funding and the economy

One government budget owns this whole loop:

| Activity | Funding owner | What it buys |
|---|---|---|
| Shared scientific/component research | Existing funded research allocation | Knowledge and unlocks |
| Model development and certification | Defense → Military research | Actual development work and testing |
| Tooling, production, equipment purchases and refits | Defense → Equipment procurement | Manufacturing work and delivered equipment |
| Maintenance and supply | Defense → Maintenance & supply | Supported serviceability and replenishment |
| Crew preparation and transition training | Defense → Personnel & training | Available trained personnel and conversion effort |
| Deployment and use | Defense → Operations | Sustained operational activity |
| New arms plants and other buildings | Existing construction budget and appropriation rules | Physical facilities |

The Military research department currently contributes to generic force support.
Its new role therefore **replaces** that part of the existing effect in designer
mode; it cannot fund projects while continuing to provide the same support bonus.
Introduce a versioned force-support mapping and migrate its reference allocation
so merely loading a campaign does not remove part of the inherited army. The
reference normalization is a one-time compatibility adjustment, not free future
funding. Subsequent transfers have their actual opportunity cost.

There is also a cash-settlement change: the current program opening immediately
expenses Defense departments 0/1/2/4, and ordinary `available_bn`/`spend` helpers
do not make those departments spendable work pools. In designer mode, department
4 becomes explicitly accrued development authority; remove its automatic service
expense and charge only completed funded work. Classify that payment as R&D
expense, not a building investment that automatically creates provincial capital.
Use a versioned spending-category rule, rather than pretending it is an existing
capital department.

Likewise, quoted maintenance costs cannot be extra treasury debits on top of
department 2's automatic service payment. Replace that posting in the new mode
with a single support settlement: legacy force support, custom-lot maintenance
and magazine replenishment have explicit shares of the same maintenance/supply
authority. Unmet support reduces the appropriate readiness or replenishment
result, not inventory ownership. Migrate the old magazine-refill contribution
at the same boundary; the new stores system cannot keep free scalar ammunition
replenishment for the same equipment. Personnel and operations retain their
declared owners. Enrolled and non-enrolled legacy paths require separate tests.

Development and procurement each allow a daily ceiling plus project priority,
within their current appropriation. These are routing controls, not extra
treasuries. Show the available authority, planned next payment, actual last
payment, spent-to-date and remaining commitment. Year-end expiry does not erase
completed work; renewed appropriations are needed for further work.

For new design production, implement one complete procurement quote and cash
ledger. Material purchases, internal inputs and fabrication charges have named
owners. Imported inputs paid through trade must not also be billed as a second
embedded raw-material purchase in the fabrication price. Quotes distinguish
already-owned stock from new purchases and keep trade receipts linked to the
work they support. Existing prepaid procurement funds remain previously paid
funds when consumed. This accounting change is a required integration task,
not a claim that the current manufacturing screen already provides an all-in cap.

Civilian projects, military development and procurement compete through the
annual plan. They do not all secretly draw from the civilian construction slider.
Budget reopening retains existing political costs; funding the same order does
not charge a second unrelated design-currency toll.

The first component recipes resolve into the existing twelve raw commodities
and already supported industrial goods. The current class recipe is too coarse
to describe every custom design, so a deterministic recipe compiler replaces
that lookup for new designs. Adding new commodities would require a separate
schema migration for fixed-size inventories and histories; component variety
does not itself justify dozens of new resource types. Show detailed input
requirements only when they help explain cost or a real supply constraint.

Expose development spending in the economy overview as a named part of the
appropriate ministry total, with its dated receipt. It is never an additional
charge on top of that total. Apply the same distinction to tooling and refits.

## 11. Production and fielding

A production order names a **certified design revision**, a quantity, an eligible
province/line, priority, and a funding limit. Existing staff-managed procurement
remains available. A country's first manual line must not silently end replacement
procurement for every unrelated role; manual/staff ownership needs to become
explicit per role or assigned budget share.

Split the current long lead time into:

- Model development, paid once for that revision.
- Factory tooling or line conversion, paid at the relevant site.
- Repeat manufacture and acceptance, paid per unit or batch.
- Delivery and fielding, subject to actual control, transport and training rules.

Do not stack a new ten-year development phase on top of an unchanged legacy
requirement-to-delivery timer. Existing orders retain their promised remaining
time; the split governs new design orders after migration.

New physical platforms enter inventory as complete units. Work and spending may
be fractional; half a finished tank is work in progress, not a fielded tank.
Small consumables and standardized infantry packages can use documented batch
units. Legacy fractional holdings remain explicitly legacy equivalents rather
than being rounded into invented real vehicles.

Whole-unit rules apply to losses and retirement as well as deliveries. For new
platform lots, accumulate fractional expected attrition in saved, bounded
residuals within the appropriate stock group and write off complete units in a
stable order when a whole loss is due. Condition may change continuously without
creating half a vehicle. Scheduled retirement and scrapping remove complete
platforms; transfer, refit and lot merging preserve the associated residuals
without cloning them. Keep the current fractional treatment for legacy
equivalents. Test low-volume fleets, simultaneous conflicts and save/reload at a
residual boundary.

Production receipts distinguish **work funded**, **units completed**, **awaiting
delivery**, and **entered service**. Materials already consumed are not consumed
again at delivery. Occupied or contested plants stop producing for their legal
owner without transferring their stock to the occupier.

The fielding policy chooses reserve, replacement, or modernization of a named
equipment role. Automatic allocation is the default, with an inspectable priority
list and the option to direct new stock to theatre packages. Equipment cannot
appear in multiple packages or conflicts simultaneously. A later formation
designer can consume this same inventory; it is a separate feature, not a hidden
prerequisite for making the equipment designer useful.

## 12. Upgrades, refits and service life

The library groups revisions into a model family: original, upgraded, export,
specialist, and replacement variants. **Duplicate as new revision** opens a
comparison with the original. Classification comes from changed systems and
compatibility rules, not just the new name.

- A compatible retrofit uses existing units and procurement work. Selected units
  are withdrawn from deployable stock until the refit completes.
- A family conversion requires suitable facilities and additional work.
- A platform change outside the allowed conversion rules is new manufacture.
- New electronics do not reset the age of the underlying vehicle or ship.
  Track platform age separately from replaced module age where it affects cost
  or serviceability.
- Cancelled work conserves the original equipment and any completed modifications;
  it never clones a model or returns already spent money.
- Reserve, mothball, reactivate, retire and scrap are explicit inventory states.
  Lower upkeep has a readiness/recommissioning consequence. Salvage, if supported,
  returns a bounded recorded resource quantity, never both full cost and materials.
- Old production can continue after a newer model exists. **Superseded** means
  not preferred for new orders, not automatically deleted or combat-disabled.

Common components can reduce maintenance variety and conversion difficulty.
Use a small, explainable standardization effect based on actual fielded families,
not a stacking national modifier for every duplicate model saved.

## 13. Make component choices matter in the military model

The current resolver uses national technology strength and equipment book-value
adequacy, with broad land/strike/lift composition penalties. Feeding an authored
"quality" sum into it would recreate documented balance problems. The designer
needs a new, bounded capability adapter.

Start with role-specific game ratings: land combat contribution and protection,
reconnaissance, fire support, air defence, air superiority, strike, naval escort,
undersea operations, lift and support. Add a rating only alongside its actual
consumer. A sonar stat must not be presented as useful before naval detection
uses it. Disabled future systems stay labelled as such and cannot be mandatory
spending in a released slice.

The adapter evaluates **fielded quantity × condition × supported readiness ×
role contribution**, then applies the role's existing or replacement force cap.
This is a design shape, not a calibrated equation or a promise that all factors
can simply be multiplied. Coefficients, normalization and caps require measured
fixtures and multi-seed tests before activation.

Use one ownership map for every effect:

| Property | Simulation owner | What must not also award it |
|---|---|---|
| Hardware mission capability | Compiled fielded design and role resolver | Global bonus from researching the same hardware |
| Training and organization | Personnel/training and doctrine | Extra copies of equipment modules |
| Serviceability | Maintenance, condition and available support | Book value used as a second readiness multiplier |
| Consumption and stocks | Inventory/supply settlement | A second deduction from the old national magazine |
| Strategic access and reach | Basing, transit, theatre and lift rules | Owning a long-range model granting diplomatic access |
| Force losses | One national post-theatre settlement | Independent casualty writes per design or allied conflict |

Replace hardware-related national tech bonuses deliberately. Legacy equipment
receives a versioned compatibility capability profile preserving its initial
contribution; new revisions use compiled components. Genuine doctrine effects
remain separate. Economic effects of dual-use discoveries remain intact.
Do not both bake a radar improvement into a model and retain its old global
hardware multiplier on that model. Test mixed legacy/custom fleets explicitly.

Keep financial book value for accounts. Changing a model's price must not create
combat power when its physical configuration is unchanged. New and legacy stock
must not receive both the old financial-adequacy bonus and a new equivalent
coverage bonus for the same equipment contribution.

Mission capabilities need real coarse operational consumers:

- Ground roles feed the existing commitment/terrain/front outcomes.
- Air roles require available basing, mission reach, serviceability and compatible
  stores; sortie support must be bounded by supported aircraft.
- Naval roles require an explicit escort, blockade, transport or undersea mission
  model before those designs are considered mechanically complete.
- Sensors and electronic systems interact through bounded role effectiveness,
  rather than a universal flat attack bonus.
- Munitions link to compatible platforms and finite stores. Launchers are assets;
  rounds are consumables. A stored round cannot also increase the standing army
  as if it were a permanent vehicle.

The resolver still snapshots all participating forces together and settles
losses once. No hourly combat, individual projectile simulation or independent
random-number stream is introduced.

## 14. Foreign equipment, licences and industrial access

Distinguish **know**, **may manufacture**, **may integrate**, and **own in service**.
Importing a finished model does not disclose every component design. A licence
can cover production, a component, a limited quantity, an expiry date, or export
permission, with its terms visible before acceptance.

Owned equipment remains owned if a licence expires. New production can be
blocked, while maintenance and spare-parts access are evaluated under their own
terms. Sanctions can disrupt deliveries or replacement inputs without deleting
existing vehicles. A factory in occupied territory does not grant the occupier
the original designer's knowledge or permissions.

Use the current trade, freight and diplomacy systems for actual purchases and
deliveries. Add equipment-specific contract terms where missing. Start domestic
designing first; licensing and export are later full-scope milestones. Country
flavour may suggest names and presets, but must not force a country down a
scripted national technology tree.

Equipment aid must transfer or procure actual identified stock and deliver it
once. The current aid path can affect force directly; a donated custom model
must not receive that same aggregate force award as well as its equipment
contribution. Retain non-equipment aid effects under their separate owners.
In dissolution, succession or annexation, explicitly allocate each lot, pending
order, licensed right and design record once. Inherited equipment does not
automatically grant manufacturing rights. Several current successor constructors
start with empty arsenals, so this is a required conservation migration rather
than an already-working inheritance guarantee.

## 15. Starting equipment and credible content

Use sourced real model names and dated equipment histories where available.
Separate a historical name from a custom design: editing a historical preset
creates a visibly custom revision, not a claim that the historical model had
the player's configuration.

The current inherited arsenal is largely budget-derived; it is not a verified
count of every country's tanks and aircraft. Migration therefore preserves
legacy quantities, value, age and deliveries. It must not relabel a synthetic
holding as a precise historical order of battle. A sourced 1990 inventory
transcription is a separate content task with provenance per record.

Every component/platform record carries its provenance and data category:
historical fact, inherited compatibility data, or explicit game-balance assumption.
Unknown values remain unknown or clearly modeled. Nationally unknown technology
is not granted merely to make a preset pass validation. Imported holdings can
legitimately contain components the owner cannot manufacture.

Coverage is complete when every supported equipment role has an inherited
option, an affordable attainable improvement path, valid staff presets and an
operational consumer. A large node count alone is not completion.

## 16. Staff assistance and AI

Offer recommendations from actual gaps: missing role coverage, replacement
needs, poor serviceability, a compatible researched improvement, an affordable
domestic alternative, or excessive diversity of equipment.

Each recommendation includes **why now**, what it would replace, relevant
funding/supply constraints, and a downside. Suppress duplicates of funded
development and production. A country whose budget cannot sustain an advanced
model should see an affordable alternative, refit, import or maintenance option.

AI uses the same validation, funding and production rules. Generate a bounded
set of role presets, score them against needs and affordability, and choose in
stable order. Do not search every module permutation every day. Reconsider on
meaningful events or a fixed review cadence; use cooldowns to avoid redesigning
and cancelling production whenever one small component unlocks.

Staff policy can manage a chosen role or budget share, with clear limits on
development, procurement and recurring cost. Manual and automatic work coexist.
AI does not get free research, instant designs or off-ledger stock to compensate
for missing designer logic. All commands remain deterministic and auditable.

## 17. Technical contracts

The following are proposed types and interfaces, not existing APIs.

| Record | Required responsibilities |
|---|---|
| `PlatformDef` | Stable ID, family, roles, slot rules, game capacity ratings, prerequisites, source metadata |
| `ComponentDef` | Stable ID/version, compatibility, unlocks, cost/recipe/support contributions and real effect consumers |
| `EquipmentDesign` | Owner, stable ID, name, family and revision lineage |
| `DesignRevision` | Immutable platform/modules/loadout compatibility, rules version, content hash and compiled profile |
| `DesignDraft` | Editable concept; may contain missing requirements but cannot spend or field equipment |
| `DevelopmentProject` | Revision ID, work/calendar progress, applied limit, priority, expenditures and receipt |
| `EquipmentRef` | Tagged legacy kit ID or design-revision ID; preserve existing legacy serialization |
| `ProductionBatch` | Product revision, site, tooling/manufacture progress, paid inputs, completed units and delivery state |
| `EquipmentLot` | Product revision, origin, quantity, platform age, condition, service/reserve/refit state |
| `RefitOrder` | Reserved source lot/quantity, target revision, compatibility and paid conversion work |
| `EquipmentLicence` | Rights, counterparties, product scope, limits and expiry |
| `ForceEquipmentPlan` | Role requirements and conserved assignment of supported holdings |

Published designs store their specification and compiled rules version. A content
update must not silently rewrite old units, prices or contracted work. Planned
balance migrations are explicit; cached UI previews are never save truth.
Use stable IDs, deterministic iteration, bounded collections and validated content.

### Commands

Draft persistence can use a no-cost command. Spending and state transitions use
the existing authenticated command channel with session identity and idempotent
retry. Proposed commands include save/duplicate draft, start/pause/cancel
development, set development funding/priority, start design production,
convert a line, order/refuse a refit, set equipment allocation and retire a lot.
Licence and export commands follow when that milestone exists.

A duplicate retry must not start a second project, withdraw the same units
twice, spend again, or create a second certified revision. A late preview from
another campaign cannot authorize current work.

### Pure reads

Proposed endpoints: `/api/military-research`, `/api/equipment-library`,
`/api/equipment-design-preview`, `/api/equipment-development`, and
`/api/equipment-refit-preview`. Extend manufacturing and arsenal payloads with
design references rather than create a second inventory API with competing totals.

Preview responses include identity/revision, validity, exact blockers, supported
effects and their scope, costs by owner/period, planned and already paid work,
conditional timing, and legal next actions. Rust supplies the calculation;
JavaScript renders it. Previewing never discovers technology or advances work.
Bring the existing `/api/tech` read under the session guard as part of these
routes, and replace both its full-bank candidate ETA and the inspector's local
alternate ETA with the authoritative focus-switch quote.

### Code ownership and migration anchors

| Current file | Planned integration |
|---|---|
| `spheres-sim/src/tech/mod.rs` and domain files | Discovery classification, unlock index, research focus quotes and frontier accounting |
| `spheres-sim/src/arsenal.rs` | `EquipmentRef` compatibility, delivery/holding resolution, ageing and accounting |
| `spheres-sim/src/manufacturing.rs` | Design products, role-scoped staff/manual funding, tooling and production batches |
| `spheres-sim/src/programs.rs` | Development authority, single settlement, Defense support ownership change |
| `spheres-sim/src/industry.rs` | Research-centre support independent of Economic Competition when applicable |
| `spheres-sim/src/operations.rs`, `war.rs` | Bounded capabilities, snapshot/assignment and one loss settlement |
| `spheres-sim/src/resources.rs`, logistics and commerce modules | Inputs, inventories and paid physical deliveries |
| `spheres-sim/src/world.rs`, `lib.rs`, `clock.rs` | Versioned state, commands, daily ordering and exact save continuity |
| `spheres-web/src/main.rs`, `transport.rs` | Session-bound reads, command validation, assets and authoritative previews |
| `spheres-web/ui/index.html` and new equipment/research UI modules | Research entry points, designer, library, development and production routes |

The current kit deserializer only accepts static `DECK` IDs. Add tagged custom
references without replacing that old format. Holding and order merge keys must
include design revision and relevant lifecycle state; equal display names or
equal delivery months do not make two products interchangeable.

## 18. Save compatibility and activation

Develop behind a versioned equipment-design rule. Legacy headless mode remains
unchanged, with the absent/default-off state omitted from old saves. Do not turn
it on for the user's daily campaign just because a new screen exists.

Migration must preserve treasury/debt, departmental authority and prepaid funds,
technology IDs and progress, held stock and age, pending order quantities and
remaining delivery times, production lines, current conflicts, force allocations
and construction projects. Preserve unknown imported designs as equipment
records with limited rights rather than granting their component technology.

Do not delete old records after converting them until conservation checks pass.
Once a save uses custom designs, older builds must refuse it clearly instead of
silently discarding them. Keep a pre-migration backup. Same saved state and dated
commands must reproduce the same next-day results after reload.

Make that refusal structural: an activated designer save uses a versioned outer
save envelope containing the world, instead of a legacy-shaped world with extra
optional fields. Current loaders can ignore unknown fields, so a flag alone is
insufficient. Verify the prior executable rejects the new envelope because its
required root world fields are absent. The new loader accepts both formats and
validates the declared version before any migration. Apply this boundary as soon
as designer state is enabled, including drafts/development with no delivered
custom equipment yet. Legacy-mode saves retain their original format.

Enable the new rules only after stock, fiscal and combat migration checks pass.
Browser and headless simulation must read the same versioned rules. Developer
flags are development controls, not confusing player setup requirements.

## 19. Delivery sequence and completion gates

Each milestone delivers a playable, integrated increment. These are dependency
boundaries, not calendar promises. Prototype throughput and content-authoring
effort should be measured before estimating dates.

| Milestone | Deliverable | Completion gate |
|---|---|---|
| 0. Contracts and migration | Type/schema decisions, discovery classification, effect/spending ownership map, baseline fixtures and versioned rule/save envelope | Old saves/default simulations unchanged; no automatic economic frontier or force gain |
| 1. Component research foundation | Military research filters, unlock index, known/locked reasons, missing-prerequisite routes and one attainable early tank-component path | Exact focus-cost/ETA behavior; pure previews; useful research available in an eligible 1990 campaign |
| 2. First complete vehicle designer | Tank-family platform choices, useful components, presets, draft/revision comparison and validation | Several valid configurations with meaningful tradeoffs; no illegal or free upgraded model |
| 3. Develop, build, field and refit | Development funding, certification, tooling, production, delivery, maintenance, first role-capability consumer and one compatible tank refit | Exact revision enters service; costs settle once; combat reflects a tradeoff without duplicate tech/adequacy effects in mixed fleets; refit stock is conserved |
| 4. Ground forces and modernization | IFV/APC/recon/artillery/air-defence/support families, component-only research expansion, broader refits/replacement and minimum launcher/store compatibility | Ground roles have affordable alternatives; basic consumable conservation and replacement of overlapping scalar refill exist before aviation loadouts |
| 5. Aviation and helicopters | Airframes, sensors/engines/mission systems, support aircraft, rotary-wing and drone families | Basing, supported missions, loadouts and operating costs constrain actual use |
| 6. Naval designer | Surface/submarine families, ship-class revisions, yard eligibility and long refits | Escort/blockade/lift/undersea consumers exist; air wings and stores are separately procured where required |
| 7. Conventional munitions and shared support | Expand compatible ammunition/interceptors, expendable drones, communications and electronic support on the already working stock foundation | Full released weapon-family coverage; no standing-force bonus from unspent rounds; consumption and replenishment settle once |
| 8. Licences, exports and full AI | Rights, equipment contracts, role presets, staff procurement and small-country paths | AI completes the full loop under real budgets; imported equipment survives without granting knowledge |
| 9. Content and balance release | Sourced starting presets, later-era coverage, UX/accessibility, long-run calibration and performance | Every released role has useful choices and consumers; campaign/save/UI acceptance all pass |

Milestones 2 and 3 form the first release worth playing. Releasing only a slot
editor with no production or combat connection would not satisfy this plan.
Ground/air/naval content authoring can run in parallel once schemas stabilize;
stock/funding contracts and the capability resolver must remain coordinated.

## 20. First playable slice: exact scope

Build a ground-design library and one tank family with affordable and more
capable platform options. Provide component choices for mobility, main mission
system, protection, observation/fire control and communications, plus optional
equipment where its effect is supported. Use existing knowledge and clearly
declared legacy component mappings for the initial fixture.

Include a small, explicit early research path for vehicle observation and
fire-control integration, rather than depending on the much later active-protection
node. Audit late-Cold-War component sources and author its prerequisites and
earliest availability before choosing numerical gates. At least one valid 1990
starting nation must have an attainable useful component upgrade through normal
research. If a nation already knows that component, offer the corresponding
design/refit opportunity instead of making it research its own knowledge again.
The first component-only node follows milestone 0's accounting classification;
the broad component tree still expands in milestone 4.

The acceptance scenario is a complete player journey:

1. Open a military research branch and inspect a component that helps an existing role.
2. Follow its prerequisite/focus route and acquire it through normal funded research.
3. Duplicate a baseline vehicle, choose the new component and compare the tradeoffs.
4. Save a named revision and see exact missing requirements or development costs.
5. Fund development from Military research; pause/resume it without losing work.
6. Certify it, assign a production site and quantity, and inspect daily spending.
7. Observe a complete unit move through delivery into service as that exact revision.
8. See a bounded role-capability change and its new maintenance commitment.
9. Save and reload halfway through each stage with identical continuation.
10. Develop a compatible revision and refit existing stock without creating units
    or resetting the underlying platform age.

Do this in an isolated development campaign. Integrate the migration into the
user's campaign only after the sequence and conservation checks pass.

## 21. Tests and quality bars

| Area | Required evidence |
|---|---|
| Content | Unique IDs, valid prerequisites and slots, no cycles, provenance, affordable presets and correct required modules |
| Research | No macro frontier gain from component-only discoveries; focus switching, year floors, acquisition pacing, access and diffusion behave as specified |
| Design | Pure preview; missing knowledge and invalid combinations rejected; revisions immutable; preview and execution agree |
| Money | No duplicate development/material/procurement charge; limits and annual authority respected; prepaid funds consumed once; no cancellation refunds of sunk cost |
| Stock | Input/work/output conservation; no fractional new platform in service; refits reserve real stock; no cloning through merge/rename/export/retry |
| Calendar | Funding cannot bypass minimum development time; daily batching and save/load produce identical work and deliveries |
| Combat | Only fielded supported equipment counts; unchanged configuration cannot gain strength from a price change; no duplicate tech/adequacy effects or multi-theatre losses |
| Migration | Existing stock, orders, funds, research, force and construction preserved; explicit behavior for discontinued/unknown IDs and older clients |
| AI | Valid, affordable role coverage; no design churn, unlimited search or free products; small-country import/refit paths |
| Interface | Clear blockers and before/after effects; preserved drafts; stale-session/pending-order safety; keyboard and narrow-screen behavior |
| Balance/performance | Matched-seed conflict and economy measurements, supported equipment diversity, later-year tick cost and bounded design growth |

Run the applicable Rust/UI suites for each slice and the complete workspace
suite for integration. Calibration tests retain existing standards: enough
measured seeds, stated regression size, and negative controls demonstrating
that the intended defect is detectable. Do not widen tolerances or re-pin golden
hashes merely because a new designer changes outcomes.

A role's balanced/cheap/advanced choices need to succeed in different supported
conditions. If one module combination dominates every budget and mission,
additional content is not a substitute for fixing the tradeoff.

## 22. Decisions deliberately left to implementation evidence

The overall direction and scope above are settled recommendations. The values
below should be chosen after prototypes, without interrupting routine work for
repeated design confirmations:

- Exact component prices, work requirements, capacity ratings and research costs.
- Supported capability formulas, reference profiles and caps.
- Degree of commonality that meaningfully reduces maintenance burden.
- Minimum useful fleet/munition aggregation and design-library limits.
- Naval-yard and aircraft-site capability extensions to the existing arms-plant model.
- Sourced historical starting presets and the useful stopping point for speculative eras.

Document measured assumptions alongside their tests. If a rating lacks an
operational consumer, or a historical preset lacks evidence, record the gap
instead of presenting it as a completed feature.

## 23. Definition of a full equipment designer

The feature is complete when a player can research components; create, compare
and name valid land, air and naval models; fund development; produce, field and
maintain them; refit or replace old equipment; and use imported/licensed options,
with staff assistance and AI following the same rules. Every advertised choice
must have a real effect, cost or compatibility consequence. The existing
construction budget, economic accounts, military stocks and saved campaigns
must remain coherent throughout that loop.

## Implementation status — ground expansion, 2026-09-06

The playable scope now covers **nine platforms**: main battle, heavy and light
tanks; tank destroyers; IFVs; APCs; reconnaissance vehicles; self-propelled
artillery; and mobile air defense. Tanks retain twelve independent specification
slots. Each specialist has thirteen, including its troop compartment, scout
package, artillery loader or radar; wheeled carriers and scouts use wheels in
place of tracks. Every platform has an attainable starting configuration.

There are **thirteen component-integration research projects** across the six
chassis, engines, weapons, armor, optics and communications branches. This is
the four tank integrations plus nine ground integrations. Branching component
prerequisites are enforced by the simulation and shown in the illustrated
research view. Established-system cards and links to ordinary active-protection
and tactical-data research do not increase the component-project count.

The designer now includes family selection, selection of visible 3D parts with
matching specification controls, and a comparison table against starting
configurations, drafts or frozen revisions. Comparison costs distinguish
per-vehicle fabrication/maintenance, programme development and batch tooling;
they are prospective model differences, not a refit bill or budget forecast.
Ten GLB examples cover the nine platforms plus a mobile tank configuration.

The service view proposes maintenance recovery and known single-component
modernizations, with reasons and fabrication/maintenance tradeoffs. It checks
up to eight available custom holdings, requires at least a 0.025 primary-role
rating improvement and discounts gains by proportional increases in those two
costs. It shows at most five suggestions. It neither orders work automatically
nor searches complete multi-component rebuilds. Compatible refits still require
certification; a main-weapon or chassis change requires new manufacture.

All released families use the existing funded development, certification,
factory-slot, raw-input, production, delivery and refit lifecycle. Frozen
specialist ratings have operational consumers: fire support improves ground
attack, protected mobility improves seizure/control, reconnaissance improves
ground target exposure, and mobile air defense reduces rung-6 air-raid damage.
Only supported, available Arsenal stock contributes, and the same conserved
deployment remains shared across theatres. No designer vehicle creates
strategic air/naval lift or a second force stock.

Equipment-state version 3 adds specialist profiles while accepting old state
versions 1 and 2. Original package and detailed tank revisions retain their
versioned identity, frozen prices and contracts. Ground revisions carry new
version-3 mission ratings; old equipment loaders reject the newer state.

This is an integrated extension of milestone 4, **not completion of the full
milestone or roadmap**. Shared national munitions still supply all weapons;
separate shell/missile inventories, replacement of the scalar support system,
full fleet maintenance invoices, designer-specific automatic replenishment and
additional support families remain open. Aircraft, helicopters, naval design,
licences, exports, designer AI, sourced historical presets and long-run balance
work in milestones 5–9 remain planned. The static aircraft/ship models in the
46-model Arsenal catalogue are preview art, not implemented component designers.

The current behavior and economic boundaries are documented in
[EQUIPMENT_DESIGNER.md](EQUIPMENT_DESIGNER.md). Targeted verification covers
complete funded development/production/delivery for each specialist, physical
inputs and shared factory slots, research dependencies, unchanged frozen tank
profiles, real role consumers, maintenance/refit exclusion, conserved deployment
and deterministic reload. These checks do not claim historical calibration of
the new game-assumption ratings.
