# Conserved military operations — 2026-09-04

Ridge authorized implementing the game-audit recommendations. This extension
answers three reproduced defects: zero-deployment allies losing national armies,
multiple conflicts fielding 170% of one army, and an enemy-held factory producing
for its legal owner.

`GameRules.military_operations` enables these changes. It defaults false and is
omitted when false; old headless calibration retains its exact equations.
Browser play enables it on boot, new and load. Missing force allocations migrate
to automatic staff allocation without inventing equipment or changing sovereignty.

## National command and losses

Each conflict requests its rung's normal force, bounded by an optional national
allocation ceiling. `SetForceAllocation` accepts `None` (automatic) or 0–10000
basis points. It is free; raising the rung remains politically priced. Requests
share national structure and one overseas ceiling proportionally, in conflict-id
order. An allocation is a ceiling, not a promise that access or simultaneous
commitments can supply it.

War snapshots forces after regeneration and AI posture decisions. All theatres
read the same forces, quality, equipment composition and magazines. Losses debit
deployed force, scaled by each belligerent's exposure rung. National structure
and equipment settle once after all theatres. Zero deployment incurs zero direct
force or equipment losses. Magazine use scales to deployment and shares one
national magazine. Pending equipment orders cannot suffer battlefield losses.

## Bounded equipment roles

The uncalibrated quality column is not used. Existing adequacy prices total
equipment. Three composition factors use depreciated book-value shares: land
(armour + infantry, reference 40%), strike (air + missiles, 25%) and lift
(air + naval, 20%). Each ranges from 0.65 to 1.0: a composition penalty, never
a second technology/equipment-value bonus. Land affects taking ground, strike
affects standoff combat, their mean affects other combat and lift caps overseas
deployments. These are modeled game roles, not transcribed historical capacities.

Half of national force casualty share becomes irrecoverable material loss in
participating classes. Exposure weights by rung are explicit game assumptions:

| Rung | Armour | Air | Naval | Infantry | Missile | Space |
|---|---:|---:|---:|---:|---:|---:|
| 6, standoff | 0 | 1 | 0 | 0 | 1 | 0 |
| 7, limited | .6 | .5 | .5 | 1 | .3 | 0 |
| 8, campaign | 1 | .5 | .2 | 1 | .3 | 0 |
| Other | .7 | .25 | 0 | 1 | .1 | 0 |

Personnel and recoverable damage remain in the existing regeneration model.
There is no individual-platform model. Wider daily calibration must measure balance.

## Physical control

`control::controller` returns the legal owner on friendly ground, the living
opposing principal on occupied ground, and no controller on contested or
contradictory fronts. `can_operate` also requires legal ownership: occupation
grants no factories, inventory, cash or territory to an invader. Manufacturing
and construction pause without inputs. Industry, materials and project guards
use the same predicate through `district_contested`. Located non-oil extraction
and completed mines pause under occupation. Oil's existing national system
retains its boundary. Freight reads the same controller. Recapture resumes work;
formal cession retains existing ownership rules. Unfought provinces stay usable
while a country has a rhetorical quarrel.

## Evidence

The first three integration regressions were watched fail before repair. The
nine-test suite in `spheres-sim/tests/military_operations.rs` adds overseas caps,
player ceilings/rejection, composition tradeoffs, inventory losses, simultaneous
snapshots, control conflicts and daily batching/save continuity. All nine pass.
Existing affected suites pass: war 13, arsenal 4, manufacturing 15, production 12,
industry 13, logistics 17 (one existing ignored profile). Workspace compilation
passes. No existing assertions, historical inputs, calibration thresholds or
golden pins changed. Fixtures establish correctness, not statistical balance.

## Player controls

The Wars card and each participating conflict sheet show national force, actual
fielded force, reserves and the shared overseas limit. Players can select an
automatic ceiling, zero (reserve), preset percentages or preserve any saved
basis-point value. The command is free and binds the authenticated campaign
player. Negative, fractional, oversized and missing inputs are refused.
The UI adopts the returned authoritative world after a successful command and
shows command failures inline. Browser boot/new/load enables these rules.

Validation adds four real-module Node renderer/submission tests, two Rust web
parser/state/load tests and the existing browser rule migration test. The 14
existing operations UI tests, all nine military integration tests and workspace
compilation pass. Full browser viewport coverage is an integration check.

## Congestion-aware freight

With military operations enabled, a dispatch first tries the normal route. If
that route cannot carry the requested lot, two deterministic searches may find
(1) a route that fits the whole lot or (2) one with more available capacity.
Each search settles at most 1,024 graph nodes, using the existing integer travel
costs and stable tie order. Failure or the bound leaves the original route and
its available fraction. This is a bounded local improvement, not a global
maximum-flow solver or a guarantee that every distant alternative is found.
The same policy, diplomatic and physical-control restrictions apply throughout.

Raw and manufactured goods reserve the existing shared edge ledger. A diversion
creates one cargo on its actual route, with that route's travel time and an
explanation retained through saves and displayed in the freight room. Unshipped
goods remain at origin. Contract routes are frozen together before allocation;
every barter leg and its payment retain one service fraction. Fresh contract
forecasts use the same routing and capacity rules. Nothing is split in transit.

Nominal paths remain cached independently of diversions, and capacity values are
reused only within one spot clearing, whose infrastructure/calendar cannot
change. Cargo permission checks may reuse an identical booked path during one
arrival pass. Both are ephemeral, never save truth. Contested endpoints are
rejected by original, memoized and shared-tree searches. Cession invalidates
cached departures. Loaded cargo keeps its booked route and quantity; closures
are visible before the due date, and arrival is held until that route reopens.
No unknown mid-journey position is invented to teleport paid cargo elsewhere.

Seven routing invariants cover alternates, capacity, all-land/sanctions refusal,
save/cache parity, exact arrival dates, frozen bundle/forecast parity, early
holds and cession. A resource-ledger regression checks two diverted barter
contracts debit only their common service fractions. The first two routing
regressions were watched fail before repair. An optional 1–45 day observer uses
`SPHERES_MILITARY_PROFILE_DAYS` to measure actual daily worlds, without a timing
threshold or any wall-clock input to simulation.

Spot clearing has one additional deterministic budget of 32,768 settled nodes
for alternate searches, consumed in the existing commodity/buyer/seller order.
Once exhausted, later orders retain their nominal routes. This is a declared
routing rule, not a time limit or an unsaved cache hit deciding gameplay. Both
cached and uncached clearing paths use it; clearing is atomic and its daily
completion is saved, so reload cannot obtain another budget for the same day.
The exhaustion regression verifies unchanged capacity and no phantom cargo.
The day-32 full-ledger regression compares cached clearing against a reloaded
world with pure-read caches disabled and checks same-day idempotence.

The 40-day debug observer measured 14.382 seconds with legacy military rules and
15.755 seconds with the extension (6,109 versus 6,188 outstanding consignments).
This replaced a 133.477-second intermediate result before the shared search
budget. These are one-run development measurements, not statistically calibrated
performance bars; later campaigns and release builds need the integration
performance harness. The extension changes physical control and combat as well
as routes, so the two final worlds are not expected to be identical.

Final targeted validation: logistics 25 passed (two profile observers ignored),
resource freight 7 passed, modern day-32 full-ledger parity 1 passed, commerce
16 passed, military integration 9 passed, web logistics 4 passed, and Node
operations/military UI 19 passed. `cargo check --workspace --all-targets` passes.
No legacy regression assertions or calibration pins were modified. Integration
owns the complete suite, actual browser viewport QA and later-year release
performance sampling.
