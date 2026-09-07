# Tactical aviation designer

The first aviation slice adds **two airframes, eighteen component choices and
three component-research projects** to the existing Equipment bureau. It covers
light attack and tactical strike aircraft used in rung-6 air raids. The broader
[military equipment plan](MILITARY_EQUIPMENT_DESIGNER_PLAN.md) remains staged;
this is the first part of its aviation milestone.

## Build and support an aircraft

Choose **Light attack aircraft** or **Tactical strike aircraft** in the Designer.
Name the model, select its components and review the aircraft ratings,
installation allowance, fabrication price, development time and upkeep. Drafts,
comparisons and previews create no aircraft, stores or national capability.

| Specification | Available choices and current effect |
| --- | --- |
| Engines | Economical single, twin or researched managed engine; cost, upkeep and sustained sortie output |
| Wing and flight controls | Straight, swept or researched stabilized wing; installation and sustained sortie output |
| Attack radar | Basic navigation or researched ground-mapping radar; bounded strike effectiveness |
| Attack avionics | Analog or researched digital controls; strike effectiveness and guided-store compatibility |
| Mission protection | Basic warning/decoys or researched electronic countermeasures; bounded strike effectiveness |
| Store installations | Two or four stores per sortie; strike weight, installation and physical consumption |
| Certified strike loadout | Unguided or researched guided-bomb interface; exact required store family |
| Endurance installation | Standard or extended; sustained sortie output, installation and upkeep |

The light airframe cannot fit twin engines, swept wings or the four-store
installation. Guided bombs require digital avionics. Every configuration must
fit its installation allowance; choosing every advanced part is not always valid.
Propulsion integration unlocks managed engines and stabilized wings. Mission
systems unlock mapping radar, digital avionics and electronic countermeasures.
Guided-strike integration requires mission systems first. These projects use
the existing Aerospace research effort and acquisition limit; learning them
installs nothing on a fielded aircraft.

Fund development, certify the frozen revision, then order a finite production
batch at a province with a free completed arms plant. Development uses Defense
research; tooling, aircraft fabrication and refits use Defense procurement.
Aircraft share factory slots, funding pools and raw-input stocks with other
equipment. Fabrication prices exclude separately acquired materials. Completed
whole aircraft enter the existing delivery queue before service.

Delivered aircraft occupy the Arsenal's Air class and retain their exact design
identity. Upkeep uses the frozen model requirement; condition and refit
withdrawals limit supported aircraft. A same-airframe component change can be
developed and installed through a paid refit. Converting between the two
airframes requires new manufacture. Refits retain age, reserve real source units
and never duplicate an aircraft. Pause, cancellation and retirement follow the
ordinary [equipment lifecycle](EQUIPMENT_DESIGNER.md).

## Physical mission stores

Every custom aircraft requires its selected physical bomb family from its first
deployment. This does **not** activate physical ground ammunition: ground keeps
its separate prospective opt-in. Neither a certified loadout nor aircraft
delivery grants bombs, and a dry aircraft does not fall back to the inherited
national magazine.

The two families are `air_bomb_unguided` and `air_bomb_guided`. Manufacture them
in **Ammunition** after setting an actual fleet-maintenance plan. Fleet upkeep
uses Defense maintenance authority first; finite bomb batches use the remainder
and real arms-plant slots. Wrong-family bombs, unfinished batches and cargo still
in transit cannot supply a raid. Available stores are shared across all air
deployments and consumption is recorded once by the opening operations snapshot.

Existing manual reserve targets, explicitly authorized automatic batches and
optional raw-material purchasing also cover these families. These remain
separate permissions: a reserve target grants nothing, a fabrication order needs
its own funding and inputs, and purchasing materials does not manufacture bombs.
See [AMMUNITION.md](AMMUNITION.md) for caps, receipts and financial ownership.

## Supported strikes and scope

The baseline light aircraft plans twelve sorties per month with two stores per
sortie; the baseline tactical-strike model plans ten with four. Components change
those explicit game assumptions. Actual requirements scale with supported
aircraft, the conserved deployment share, operational intensity, date fraction
and available theatre basing. The displayed full-deployment reserve is a planning
reference, not guaranteed sorties or a forecast of war duration.

Engine, wing and endurance choices affect sortie output. Its ratio to the
airframe baseline is folded **once** into the frozen supported-strike factor,
alongside mission systems and loadout, with a final 0.75–1.25 bound. Operations
then applies that factor to supported, supplied aircraft once. Buying a more
expensive component does not increase the airframe's fixed physical reference
weight. More bombs cannot raise performance above the fully supplied level.

These aircraft contribute to accessible rung-6 raids. They provide no ground
fire, ground maneuver, air-superiority mission or transport lift. Existing
theatre home/access rules provide coarse basing and reach; this slice introduces
no physical runway inventory, runway throughput, flight-distance calculation,
carrier deck or separate aviation-fuel stock. Endurance does not grant overflight
rights. Operating support is represented by upkeep and finite mission stores.

Defender anti-aircraft demand follows actual launched custom raids. Aircraft add
no direct territory-seizing mass to a coalition; the existing coalition-wide
quality calculation remains shared. Custom aircraft casualties follow each
revision's supported, supplied raid exposure. This slice does not model aircraft
destroyed on airfields during land battles.

Fighters and interception, multirole mission assignment, strategic bombers,
carrier aviation, airlift, tankers, airborne warning/reconnaissance, helicopters,
drones, naval weapons and aircraft-specific industrial sites remain future work.
Each needs a real mission or capacity consumer before it is advertised as useful.
Prices, recipes, work rates, installation points, sortie rates and effectiveness
are game-balance assumptions, not sourced aircraft engineering specifications.
Historical starting aircraft/store presets and campaign balance remain separate
evidence and calibration tasks.

## Save and verification boundary

Equipment-state **version 8** accepts versions 1–8. New aircraft revisions use
specification version 4 and an optional frozen aviation profile. Earlier ground
profiles remain sparse and retain their identities, contracts and ratings.
Validation rejects mismatched profiles, store families and non-Air aircraft
holdings or deliveries; it does not repair them by creating stock.

Focused simulation coverage checks pure previews, research and installation
refusals, fully supplied component effects, both paid aircraft lifecycles,
raw-input conservation, delivery, same-airframe refits, forged state and exact
save/load continuation. Full-workspace, browser acceptance and release-artifact
checks are integration gates; this document does not certify their outcome.
