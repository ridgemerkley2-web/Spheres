# Current architecture — SPHERES 0.6

## Daily operational warfare (version 1)

`campaign.rs` owns staff sector assignments, dated transfers, cohesion, operations,
observations and local contacts. It divides the conserved `operations::Snapshot`
and returns aggregated deployed-force loss fractions. Physical control is captured
before `war.rs` removes conflicts for resolution. Local outcomes replace scalar
front-budget projection only in the versioned daily mode.

`campaign_supply.rs` routes finite modeled support services through shared freight
capacity using military access and physical control. It does not own ammunition,
oil, equipment or money. `campaign_peace.rs` owns war aims, coalition consent and
occupation; actual cash and territory settle through existing resource/treasury
and district APIs. `campaign-operations-ui.js` displays Rust assessments and sends
normal authenticated commands. See `WARFARE.md` for player behavior and limitations.

## Authority and cadence

`spheres-sim` owns state, commands, costs and simulation results. A single seeded
SplitMix64 stream and deterministic ordering govern random decisions. Only
commands and scheduled ticks mutate the world; quotes and read models are pure.
The local `spheres-web` server owns sessions, receipts, persistence and presentation
DTOs. Browser rooms display those DTOs and submit intent. They do not compute a
second fiscal, production or combat model. `spheres-cli` provides headless and
interactive simulation access.

Browser boot/load enables daily integration, resource markets, routed physical
freight, production, manufacturing, province accounts and conserved military
operations. New worlds also seed the paid/modelled 1990 industrial inheritance
and frozen sourced broad sectors. Loading does not retroactively grant these
assets. Department enrollment and economic competition remain deliberate player
choices. The legacy monthly headless configuration keeps its own compatibility
contract; it is not evidence for daily campaign balance.

## Economy and physical work

The macroeconomy prices growth, demand, inflation, debt, war and instability.
Annual budgets authorize ministry shares; department programs consume their real
daily authority. The first-plan preview includes opening debt interest and queued
tax/rate decisions on one stated GDP basis. Full-use spending is a ceiling, not a
promise that blocked departments will spend it.

Production spends funded work and materials to create durable province assets.
Materials and manufactured packs share physical freight capacity; escrow settles
on actual delivery. Industrial planning exposes input coverage and actionable
capacity plans. Research Centers feed bounded, funded prototype work with real
inputs and prerequisite gates. They do not mint free technologies. The optional
AI economy enrolls budgets, plans purchases and builds under the same constraints.

Province GDP reconciles exactly to national output. Country agriculture,
industry and service weights use frozen WDI observations and disclosed fallbacks;
manufacturing keeps the existing sourced profile. The narrower eight-sector split
and density-weighted provincial allocation remain explicitly modeled estimates.
Historical data, inherited capacity and actual new production are separate views.

## Diplomacy, forces and sovereignty

Offers and defense requests enter the human's consent inbox. Standing policies
are explicit, deadlines are dated, sanctions persist until lifted, and pegs bind
rates until the player pays the disclosed exit cost. These rules consume no
new random acceptance roll when the player replies.

All theatres share one national army and overseas ceiling. A conflict requests
force under an optional ceiling; access, posture and competing deployments bound
actual use. One force snapshot settles losses and munitions across all conflicts.
Zero deployment creates zero battlefield loss. Bounded land, strike and lift
composition penalties and inventory losses connect procurement to combat without
activating the old unused quality score as a second strength bonus.

Legal ownership and physical control are distinct. Occupied or contested
facilities cannot produce for their legal owner; occupiers receive no free
ownership or inventories. Freight and industrial work use the same control rule.
Congestion-aware routing preserves shipment, shared-capacity and payment ledgers.
Freight remains a commercial network; it is not a complete theatre supply model.
Conquest, formal cession and voluntary economic compacts retain their own
sovereignty and consent rules.

## Campaign, transport and UI boundaries

Campaign aims observe real outcomes, freeze their targets, show blockers and
record completion. Peaceful aims require peace during their qualifying streaks.
There are no arbitrary completion rewards and the sandbox can continue.

The web layer separates campaign persistence, retained history and mutation
receipts into modules. The versioned envelope retains world, history and events.
History keeps recent daily observations and coarser older observations; selected
country and delta reads avoid repeatedly shipping every series. Loading creates
a fresh transport session while preserving the campaign record.

An importable command channel owns immediate command identity and recovery.
Pending intent survives reload in local browser storage. Receipt lookup returns
authoritative state rather than silently replaying a paid action. UI errors are
visible and in-flight actions are serialized.

Decision helpers, diplomacy, operations, campaign controls and existing room
modules have their own JavaScript/CSS. New pure functions are importable in Node
tests. Some older rooms still live in `index.html`, and some read models/routes
remain in `main.rs`; this release establishes boundaries without a framework
rewrite. Future extraction should preserve the tested contracts.

## Evidence and limits

Correctness comes from conservation, fixed-date comparisons, red/green defect
fixtures, save/replay checks, UI interaction and explicit long-run observations.
Campaign difficulty, historical fit and human comprehension need different
evidence. Do not replace a failed model test with a wider tolerance or use three
seeds to claim reliable difficulty. Per-machine timing and source-data fallbacks
must remain visible in release evidence. See PLAYTEST.md for human validation.
