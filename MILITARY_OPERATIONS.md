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
