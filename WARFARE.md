# Operational warfare

Browser campaigns use version 1 of the daily operational model. National command
still sets escalation, strategic objectives, rules of engagement and force ceilings.
The operation board adds a geographic focus, approach, reserves, support missions,
peace terms and garrison policy.

## Playing an operation

Open a participating conflict from the Wars board. Choose a district objective,
an approach and a reserve share, then issue the operation. Editing the form pauses
the clock so an advancing day cannot overwrite an unfinished order.

| Approach | Purpose | Cost or constraint |
| --- | --- | --- |
| Probe | Test local resistance | Limited ground gain; still exposes forces |
| Advance | Sustain a supported offensive | Cohesion and delivery must keep pace |
| Breakthrough | Concentrate pressure at the selected focus | Higher ammunition use and fatigue |
| Hold | Prepare defenses and recover | Concedes the initiative |
| Fighting withdrawal | Preserve survivors and shorten the front | Gives up positions and takes travel time |

Forces travel between assignments. They keep their fatigue while in transit and
remain part of the same national army. An allocation ceiling requests force;
it is not an immediate promise of arrival. Reserves and garrisons reduce the force
available for offensive contacts. Staff handles individual district assignments.

Strength means existing national force allocated to a location. Readiness means
formation cohesion and fatigue. It does not replace equipment serviceability or
a government's political resolve. Supply reports delivered support, local cover,
route constraints and the next known arrival. Unknown enemy strength is shown as
an observed interval with confidence and age; forecasts are estimates, not promises.

## What determines a battle

Contacts use adjacent district positions, terrain, prepared defenses, frontage,
cohesion, delivered support and the active equipment capability model. Limited
frontage gives diminishing returns to stacking force. Rivers and mountains can
stop total progress; there is no final pass that spends unused movement elsewhere.
Aggregate conflict control is derived from what happened in the districts.

Losses are collected against one opening force and equipment snapshot, then
debited once nationally. Retreat moves surviving force on a saved travel schedule.
Prolonged isolation and loss of cohesion can force surrender when no friendly
fallback exists. A cut route first exhausts local reserves rather than instantly
destroying the force.

Ground equipment keeps distinct support, mobility, reconnaissance and air-defense
effects. Air intent selects reconnaissance, interception, ground support or
interdiction. Naval intent selects transport, escort or sea denial. Actual
serviceable equipment and access gate these missions. Custom tactical aircraft
use their installed strike ratings and compatible bombs on supported rung-6
strikes; they do not grant unmodeled reconnaissance, interception or lift.
The operation card explains unavailable missions. Air attacks cannot take ground
without a ground commitment; overseas movement needs supported lift.

## Military supply and the economy

Military routing uses the existing freight graph, physical control, infrastructure
and shared daily link capacity after commercial reservations. Foreign military
transit requires explicit access. A closed corridor retains its cargo on the
booked route; a newly created sector receives no free local stock.

The routed quantity is **sustainment service**, measured in modeled force-days:
transport, handling, food and maintenance support already funded by the standing
force. One service unit reserves 20 modeled freight tonnes. That is a declared
game capacity rule, not a conversion from a real brigade's consumption. The hub
is a fixed most-populous suitable home district until lost, not a claimed
historical depot. Central storage holds at most 14 modeled force-days per unit of
national strength; daily generation is 1.1 service units per strength. Actual
coverage depends on deployed demand. Local storage targets seven demand-days.
Actual travel times determine the required supply pipeline.

National magazines and equipment remain in their existing accounting systems.
Service dispatch does not create or subtract ammunition, oil, cash, GDP or
equipment. Ammunition burn uses the existing conserved deployment snapshot with
one approach/mission multiplier. Activated custom ammunition uses that same
intensity against compatible physical rounds. Maneuver, paid fire support and paid
air defense remain distinct; a dry gun does not erase its vehicle. Actual local
casualties use the national loss conversion, including launched custom aircraft.
Procurement and maintenance remain funded through the equipment/industry APIs.
When People is enabled, counted military staffing constrains replacements and
actual national combat losses feed the population casualty ledger once. Workers
are assigned before construction and industrial output, so workforce shortages
can affect the equipment pipeline through the existing production system.
Occupation resistance reduces uncovered corridor
capacity; supplied garrisons can restore service.

## Peace and occupation

War aims distinguish expelling an invader, recovering named opposing districts,
securing a concession and seeking political transition. Setting an aim costs
three political capital; tabling terms or responding does not.

Principals can propose a ceasefire, cession of held districts, a limited cash
reparation or a political opening. Every coalition member must consent. A human
receives a pending offer; AI does not sign on the player's behalf. Proposals expire
after 21 days. A changed coalition or invalidated territory requires fresh terms.
Declining and counterproposing are available through the same command channel.
These consented limited settlements replace automatic annexation in version 1.
The legacy monthly solver retains its previous conquest rules.

Ceasefire restores legal control without changing ownership and schedules surviving
forces' return on the next daily preparation step. Limited
cession cannot take a country's last province and respects nuclear deterrence.
Reparations transfer cash through the established treasury/legacy cash ledger;
they do not add GDP. A political opening changes institutions and, where eligible,
uses the existing election model; it does not invent a chosen leader.

Garrisons can receive up to 50% of a theatre's deployed allocation. They require
delivered support. Restraint, security and reconstruction posture change resistance
and the strain of occupation. Reconstruction posture grants no free buildings:
physical construction remains a separately funded activity. Occupation stops the
owner's production without awarding the occupier its inventory, treasury or legal
ownership. Formal settlement is still the transfer point.

## Compatibility, assumptions and validation

`GameRules.operational_warfare = 1` requires daily simulation and conserved
military operations. Zero preserves the old solver and its serialization. Browser
boot/new/load enables version 1; previously active wars migrate their existing
force/control once. Enrollment captures exactly the already-active conflicts, so
a war declared before the first browser tick still requires dated deployment.
Unsupported future versions are refused on load.
Operational conflict identities persist across saves and are never reused, so a
later war cannot inherit an earlier war's orders, deployment or garrison policy.

The simulation uses deterministic ordering and the existing clock. Same dated
commands replay identically when stepped, batched or saved and resumed. Viewer
assessments do not advance RNG or perform allocation. The nation list gives
coarse public military estimates; opponent tactical allocations, magazines and
political red lines are not sent as exact values. Exported saves remain complete
simulation records for replay and debugging.

Relevant checks are the campaign, supply, peace and web API suites, operation UI
tests, and the actual server/browser scenario. Full workspace and legacy replay
checks remain required. `daily_calibration` accepts `--operational-warfare true`
and records the flag; its multi-seed panel is descriptive balance evidence,
not proof of historical fit or a statistically calibrated difficulty rating.
`campaign_report` exercises actual commands, full world ticks and multiple seeds,
recording control reversals, delivered service, casualties, peace and performance:

```sh
cargo run --locked --release -p spheres-sim --example campaign_report -- --days 90 --seeds 1990,7,42 --output artifacts/campaign-report.json
```

The implementation plan and final GitHub fit procedure are retained in
`WAR_OVERHAUL_EXECUTION.md`. [The release record](docs/WARFARE_RELEASE.md) identifies
the integrated source, completed checks and limits of the diagnostic evidence.
