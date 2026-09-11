# Military flight layer — arcade-first plan for review

8 September 2026 · Approved for implementation. Arcade command-room prototype and
the first two detailed aircraft meshes are built; campaign fleet/mission integration
and the remaining aircraft families are still outstanding.

Model requirement: every assembled aircraft receives a 100,000+ triangle inspection
mesh. Geometry should describe curved surfaces, fittings and recognizable equipment;
catalogue and map views use cheaper levels of detail. The first light-attack and
tactical-strike inspection presets contain 197,632 and 210,336 triangles respectively.

## 1. The direction

Build a beautiful, approachable air-force game inside Spheres. The player makes a
few important decisions, sees aircraft doing recognizable jobs, and understands
why an operation succeeds or struggles. The component designer and company
lifecycle provide depth; everyday operation stays quick and visual.

**Choose a role → design an aircraft → fund company development → buy stock →
add aircraft to a squadron → pick a mission on the map.**

Arcade style means simpler interaction and fewer management systems, not just
bright artwork over a simulation spreadsheet. No cockpit controls, individual
pilot schedules, manually planned fuel routes, daily loadout shopping or separate
engineering calculators. The game handles routine support within the budget the
player authorizes. Aircraft, money and supplies still have real ownership and
cannot be duplicated.

The target interaction is: select a squadron, press **Defend skies**, choose an
area, inspect a short cost/risk card and confirm. The player should be able to do
this in a few clear actions. Optional details explain the result without becoming
required reading.

## 2. The three things the player manages

1. **Aircraft:** what to develop, buy, upgrade or retire.
2. **Squadrons:** where they are based and which job they should do.
3. **Air Force support:** one understandable funding priority using the existing
   defense budget, with automatic routine maintenance, training and supply support.

The main screen shows aircraft owned/ready, current missions, support funding and
at most three actionable problems. A shortage becomes a readable reason and a
useful button: **Fund support**, **Choose a closer base**, or **Replace losses**.
It does not become another disconnected minigame.

New aircraft purchases and development contracts still need player approval.
An optional replacement policy can later buy available company stock within an
explicit cap; setting a desired squadron size alone never spends money.

## 3. Aircraft families

Present aircraft as illustrated role cards with a short description. Specialists
can share airframes and researched parts. A multirole aircraft changes its job
through a mission preset; the same aircraft cannot perform two jobs at once.

| Player-facing family | Included types | Simple purpose |
| --- | --- | --- |
| Fighters | Interceptors, air-superiority and multirole fighters | Protect your skies and help friendly aircraft get through |
| Attack aircraft | Light attack, tactical strike and strike-configured multirole aircraft | Support armies and attack eligible targets |
| Bombers | Conventional long-range bombers | Reach distant targets at a high purchase and support cost |
| Support aircraft | Tankers, airborne warning/control and electronic warfare | Improve the reach or effectiveness of an assigned air group |
| Recon aircraft | Reconnaissance, intelligence and maritime patrol variants | Reveal better, dated information about an area |
| Transports | Tactical/strategic airlift and utility helicopters | Move eligible troops, supplies and evacuation loads |
| Attack helicopters | Armed rotary-wing variants | Provide local army support with limited reach |
| Drones | Reusable reconnaissance/strike aircraft and later collaborative variants | Provide affordable or persistent specialized missions |

Trainer/light-combat trainers support automatic crew development; managing each
training flight is unnecessary. Carrier-capable fighters, helicopters and support
aircraft appear as compatible variants, not a second copy of the entire air-force
interface. Tiltrotor and STOVL options can arrive as later compatible variants.

Expendable loitering weapons remain ammunition: they disappear when used and do
not return to the reusable aircraft inventory.

## 4. A fun component designer

The aircraft sits in a large rotatable 3D hangar view. Clicking a part or its card
opens a short list of researched choices. Changing a visible component updates the
model, highlights the change and shows the most important benefits and costs.

Use these separate specification cards where relevant:

- **Airframe:** fighter, attack, transport or another role-compatible chassis.
- **Engines:** economical, performance-focused or other researched installations.
- **Wings and controls:** compatible configurations for that airframe.
- **Weapons:** a small set of certified role loadouts.
- **Radar and sensors:** the aircraft's detection and targeting equipment.
- **Avionics:** navigation, mission computer and communications integration.
- **Protection:** warning systems, countermeasures and signature treatment.
- **Mission equipment:** extra fuel, cargo, refueling, warning/control or another
  family-specific package.

Offer **Recommended build**, **Affordable**, and **Advanced** presets assembled
from the same legal parts. The player can customize any preset. Explain an invalid
choice directly and suggest a compatible alternative; do not expose electrical,
cooling and weight spreadsheets as mandatory controls.

The comparison strip has six readable game ratings: **Air combat, Ground attack,
Range, Speed, Survivability, Upkeep**. Role cards replace irrelevant ratings with
useful measures such as cargo or support. Price and development time sit beside
the commissioning button. These are labeled game ratings or estimates, not claims
of measured real-world engineering performance. Range also has a useful map view.

A certified mission loadout can be changed without inventing a new aircraft model.
Changing major hardware requires a compatible paid refit or a new development
revision. Research unlocks choices; it does not magically upgrade the fleet.

Research stays visual: clear branches for airframes, engines/controls, sensors,
weapons integration, protection and specialist equipment. Cards say exactly which
aircraft choice becomes available. Avoid dozens of tiny percentage-only unlocks.

## 5. Companies and development

Keep the approved government/company relationship:

**In development → Company stock → Purchased → In service.**

You design the aircraft, choose a manufacturer and approve development funding.
A clear progress bar covers engineering, prototype and testing. Important milestone
art and short updates provide feedback. Delays explain their actual cause, such
as interrupted funding, rather than hiding an arbitrary dice roll.

After certification, the company finances and builds its own stock. You buy the
finished aircraft it has available. The government never receives prototypes or
unsold company aircraft as free military strength. Delivery and squadron conversion
are represented by short, understandable progress states using the game calendar.

Manufacturer cards show name, nationality, specialty, available models, stock and
support quality where that quality has a modeled effect. Aerospace capabilities
extend the existing company system; the player does not have to negotiate separately
with an engine, radar and electronics supplier for every aircraft.

Accessible foreign stock and licensed models help small countries participate.
Imports require the existing financial and diplomatic rules. Buying a foreign
plane does not automatically teach the country how to build its radar.

## 6. Squadrons and readiness

A squadron is a named card: emblem, aircraft picture, number owned/ready, experience,
base and current mission. **Create squadron** suggests a sensible size from available
aircraft; smaller forces can use smaller establishments. Wings are optional groups
for issuing one order to several squadrons.

Use one prominent **Ready to fly** indicator. Expand it only when the player wants
the explanation: aircraft under maintenance, replacement crews still training,
insufficient support, or a damaged base. Do not make the player manage a separate
bar and shopping list for every underlying resource.

Crew recruitment, conversion and routine training happen automatically over time
within approved staffing and support limits. Crews and maintenance personnel still
come from the existing national personnel/population model. Experience uses simple
labels and visible progress; named commanders or ace cards are optional flavor,
not a requirement to hire every pilot.

Aircraft under delivery, repair, refit, transit or another mission cannot be assigned
twice. Replacing a destroyed plane still requires a real purchased aircraft.

## 7. Simple, useful airbases

Put airbases on the map and give each an illustrated base panel. The main decisions
are where to operate and which of three upgrade tracks to fund:

| Upgrade | What the player buys |
| --- | --- |
| Capacity | Space and runway service for more aircraft |
| Support | Better maintenance, preparation and recovery capacity |
| Protection | Shelters and base defenses against modeled threats |

New bases and upgrades use the user's financial construction model: pay through
the construction budget and watch work progress. Show the effect on the province
and national air force before confirming. No extra raw-material shopping interface
is needed to place a base construction order.

Fuel, stores and maintenance flow through existing supply/economic systems under
the approved support budget. Only meaningful shortages surface. Avoid introducing
a separate refinery-management expansion as a prerequisite for flying an aircraft.
If a fuel product is required internally, its paid source, stock and consumption
must be explicit and must replace any overlapping legacy charge.

Rebasing is **Move to base**, with valid destinations highlighted, a travel estimate
and a clear readiness warning. Foreign access, distance and damaged runways matter,
but the game handles routine flight preparation and supply routing.

## 8. A small mission menu

Six primary buttons cover normal play. The engine selects compatible loadouts and
support from the assignment; advanced targeting can stay inside optional details.

| Button | What it does |
| --- | --- |
| **Defend skies** | Patrol and intercept threats in a selected area |
| **Support army** | Assist a selected army/theatre; staff coordinates the exact air contribution |
| **Strike target** | Attack an eligible military/infrastructure target through a reviewed operation |
| **Scout area** | Improve information and report confidence over an area |
| **Transport** | Move an eligible force, supply load or evacuation group between valid locations |
| **Support fleet** | Maritime patrol or carrier/naval support when those systems are available |

Support aircraft occupy a small number of support slots on the air-group card.
Tankers provide finite reach/endurance support; warning aircraft improve detection;
electronic warfare contests supported defenses. They require actual aircraft and
support, but the player need not plan a separate tanker itinerary for every raid.
Support slot stacking is bounded and benefits apply once.

Choose **Cautious**, **Balanced**, or **Intense** activity. The review card shows
allocated aircraft, reach, likely opposition, support cost and a plain-language
risk estimate. Friendly coverage appears clearly; enemy capability is uncertain
and dated. No precise hidden enemy inventory leaks through the preview.

The game's daily engine resolves missions. Air control is local and changes as
forces act. Air defenses and fighters share one resolution system. Aircraft can
support ground gains but cannot occupy provinces by themselves. A patrol does
not independently declare a war or ignore overflight restrictions.

Battle feedback is immediate when the daily result arrives: illustrative flight
arcs, aircraft silhouettes, readable icons, sound and a short result card. Animation
can be accelerated or disabled and never controls the simulation outcome. No
hourly engine, cockpit controls or manual dogfight maneuvers are required.

## 9. The Air Force room and visual style

Build one bold, art-heavy **Military → Air Force** room. Give it a distinctive
command-center background, large aircraft renders, squadron emblems and clear
color-coded mission cards with text/icon alternatives for accessibility.

Keep four primary destinations:

1. **Command:** map, squadron cards and mission buttons.
2. **Aircraft:** designer, research shortcuts, manufacturer stock and upgrades.
3. **Bases:** map locations and three upgrade tracks.
4. **Reports:** concise mission results, losses, spending and useful trends.

The top strip answers: **How many can fly? What are they doing? Can I afford it?**
Avoid opening several separate departments just to fly a purchased aircraft.
Preserve the designer draft while visiting a manufacturer or budget review.

Art work spans detailed aircraft silhouettes, visible weapons/sensors/extra tanks,
hangar backgrounds, base scenes and readable map representations. Internal parts
use inspection overlays rather than fake exterior changes. Use near, card and map
LODs, bounded caches and lazy loading; visual clarity matters more than prescribing
the same triangle count for every screen. Retain licensed-asset attribution and
self-contained packaging.

Advisors suggest one useful action at a time: **Your fighters cannot reach this
area — move them to this base**, or **Your squadron needs support funding**.
A short guided tutorial takes the player from available company stock to a first
defensive assignment. It should teach by completing the normal workflow.

## 10. Scope and delivery

The first release must deliver a complete, easy loop rather than a large catalogue
of aircraft that have no useful missions. Each phase includes the corresponding
art, AI, controls, save support and verification.

| Phase | Deliverable | Review milestone |
| --- | --- | --- |
| 0 — Agree the foundation | Reconcile current warfare/company/population changes; prototype the arcade command room and define shared ownership | The screen and interaction feel right, and there is one agreed combat/accounting baseline |
| 1 — Own and field aircraft | Connect company stock, delivery, squadrons, simple bases and automatic readiness support | Buy aircraft, create a squadron and rebase through a short guided flow |
| 2 — First complete air campaign | Fighters, multirole aircraft and existing attack aircraft; Defend skies, Support army and Strike target | Two opposing AI/player forces fly, spend supplies, return or take losses, and affect real operations |
| 3 — Support and reach | Scout area, warning aircraft, tankers, transports and support slots | Support aircraft visibly help an assignment; transport moves actual cargo and people |
| 4 — Specialist combat | Conventional bombers, electronic warfare, advanced defenses and infrastructure damage/repair | New choices have understandable strengths, counters and costs |
| 5 — Helicopters and drones | Utility/attack helicopters, reusable drones and distinct expendable systems | They use the same simple command flow with role-specific results |
| 6 — Naval flight | Support fleet, maritime patrol and carrier-qualified aircraft | A carrier provides a real moving base with finite deck/support capacity |
| 7 — Worldwide polish | Historical companies/equipment, small-country affordability, tutorials, AI and optimization | Both a small country and a major power are fun without recurring logistics micromanagement |

**Recommended first playable release: phases 0–2.** Show the command-room prototype
before investing in the full specialist catalogue. Do not begin with a detailed
pilot-training or fuel-management screen.

Naval flight requires a minimum carrier-host/deck/location/support interface. Build
that bounded connection in phase 6 if the naval branch does not already provide
it; do not pretend a catalogue ship icon is a working carrier or demand an
unspecified full naval overhaul first.

These are milestones, not promises that each fits a single session. Estimate work
after phase 0 confirms integration risks. The full flight layer includes every
phase; the first release is deliberately a smaller complete experience.

## 11. Keep it connected and trustworthy

The game still needs strong internal rules, but those rules should simplify the
player's experience rather than become extra chores:

- Reuse equipment IDs, frozen design revisions, company stock and existing budgets.
- Squadrons reference owned aircraft; they do not create a second inventory.
- Plan missions from one opening snapshot and reserve aircraft, supplies and support
  before resolving them. Settle consumption and losses once across all theatres.
- Replace overlapping legacy aviation bonuses/loss paths when the new resolver
  takes ownership. A research unlock, support aircraft or mission cannot confer
  the same benefit twice through different systems.
- Reuse the dated command queue, daily clock and existing deterministic RNG.
  Previews are read-only; decorative animation has no simulation authority.
- Save squadron assignments, progress, mission orders and reservations. Preserve
  paid deliveries, refits, ownership and original model profiles during migration.
  Do not round legacy fractional holdings into free aircraft.
- AI uses the same budgets, bases and aircraft. It favors affordable supported
  forces, handles routine readiness and avoids changing plans every day.
- Starting companies, aircraft and exact inventories require sources. Unknown
  history stays explicit; future designs are labeled game projections. All
  countries can access the system without claiming complete historical data.
- Keep existing nuclear-deterrence rules. Conventional missions do not quietly
  introduce a nuclear-employment expansion. Infrastructure effects must feed the
  existing damage/economy rules once, without invented duplicate GDP penalties.

The inspected playset is `codex/resume-spheres` at
`b6f1e8f7c29d3abd12de557b19331dd492237047`. It already supports two tactical aircraft
platforms, 18 components, eight specification slots, company development/purchase,
manufacturer refits and finite bombs. It does not yet provide the full air-force
system described here. See [AVIATION.md](AVIATION.md) and [COMPANIES.md](COMPANIES.md).

The separately inspected `origin/master` reference is
`485c223f60d5ff6e46f6ae17164bf1ee3a8764d9`, with newer district warfare, population,
industry and other changes. Phase 0 must inspect current upstream again and
reconcile shared systems before implementation. Neither a merge nor a new gameplay
feature was performed while drafting this plan.

## 12. Acceptance tests, including the arcade requirement

Technical checks conserve aircraft, company/government ownership, funds, weapons,
fuel/support, cargo and personnel. A plane cannot fly two missions, a cancelled
order cannot refund expended stores, and a refit cannot duplicate inventory.
Unsupported or stale commands fail clearly. Identical dated commands must replay
identically across stepping, batching and save/reload; legacy saves retain their
own supported rules.

Gameplay checks matter equally:

- A new player with a ready squadron can issue a basic mission through a short
  select/button/area/review flow without visiting a supply or training screen.
- Buying stock, delivery and fielding form a guided sequence with useful next actions.
- Routine support works within the authorized budget without repeated shopping.
- Every grounding/shortage has a plain-language explanation and a relevant action.
- A small country can operate a modest useful force; quantity and upkeep remain
  meaningful even when a wealthy country researches advanced equipment.
- Six mission buttons, four main destinations and the three base upgrade tracks
  remain the default surface as specialist aircraft are added.
- Visuals remain legible at normal map zoom and narrow layouts. Keyboard access,
  reduced motion and renderer fallback work. Decorative flights have a density cap.

Prototype usability targets should be measured with actual players, not declared
passed from code inspection. Numerical combat balance remains explicitly authored
until calibrated; statistical tests need measured sampling and meaningful power,
with no widened tolerances to make a feature pass.

## 13. Recommended approval package

Approve the following direction together:

1. **Arcade-first strategy:** clear cards and mission buttons, no flight-simulator
   controls or daily logistics micromanagement.
2. **Deep aircraft, simple operations:** separate researched component choices,
   guided presets and a small set of readable ratings.
3. **Company development and stock purchases:** retain the approved equipment economy.
4. **Automatic routine support:** crews, upkeep and resupply work within authorized
   national budgets; expensive acquisitions and new offensive missions stay explicit.
5. **A complete small first release:** fighters and tactical support with real
   squadrons/bases before bombers, carriers and the specialist catalogue.
6. **A consistent interface for later families:** new roles expand the same system
   rather than adding separate control panels and resource economies.

The major review question is whether this level of arcade simplicity feels right.
The first implementation milestone should be the visual command-room prototype
and its short interaction flow, so that can be judged before the full buildout.

## 14. Reference basis

The breadth of the proposed aircraft roles is informed by the
[US Air Force's public mission framework](https://www.af.mil/About-Us/AF-Core-Values/),
which includes superiority, strike, mobility, intelligence and command/control.
[NATO's air and missile defense overview](https://www.nato.int/en/what-we-do/deterrence-and-defence/nato-integrated-air-and-missile-defence)
informs the connected defense and peacetime-policing categories.
[Airbus's A330 MRTT overview](https://www.airbus.com/en/products-services/defence/military-aircraft/a330-mrtt)
provides a concrete example of compatible tanker, transport and medical roles.

These public sources were checked 8 September 2026. They support category choices,
not Spheres balance coefficients or claims of engineering fidelity. The simplified
interface, ratings and mission rules above are original game-design proposals.
The user's Millennium Dawn preference informs component customization, not a claim
that all these features exist in a released version of that mod.
