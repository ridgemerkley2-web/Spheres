# Specialist ground vehicle redesign

This pass rebuilds `ground_ifv`, `ground_apc`, `ground_recon`,
`ground_artillery` and `ground_air_defense` inside `buildGround()` in
`spheres-web/ui/equipment-mesh.js`. The approved tanks and both aircraft retain
their exact geometry, normals, vertex colors, material classes and picking
identities. Component IDs, slots, part order, six/eight-wheel choices and
six/seven-road-wheel choices remain unchanged. Simulation specifications are
unchanged.

## Shape direction

The previous five hulls used almost the same short, vertically lofted octagon,
tall separate turret and long side locker. Their new primary sections run along
the vehicle: a flat working roof joins a raked glacis, chamfered shoulders and
a folded chin. All three LODs use the new proportions.

| Family | Visible changes |
| --- | --- |
| IFV | Longer troop hull, low broad turret with converging cheeks and an integrated rear enclosure, recessed autocannon joint, shallow flank feed boxes, continuous rear roof and access ramp. |
| APC | Longer and taller troop body, wider roof, modest protected weapon station, larger road tires, open rim centers, shallow rear tool lockers and clear rear egress. |
| Reconnaissance | Shorter and lower body than the APC, strongly tapered bow, compact weapon station, rear engine service deck and separately mounted observation package. |
| Artillery | Long seven-road-wheel carrier, rear-biased high fighting compartment, tapered front cheeks, slender gun and a rear loading door/enclosure. |
| Air defense | Low tracked hull and faceted central mount, two outboard cannon cradles or the selected missile canisters, an elevated chamfered radar panel and a curved tracking radome. |

Engine access, the driver's hatch and the forward sensor fittings now sit behind
the roof/glacis junction. Roof seams, modular armor, lights and tool stowage
follow their supporting surfaces. The change allocates detail to recognizable
forms rather than increasing triangle count throughout the model.

Tracked vehicles have independent front/rear end discs, with their road wheels
between them. Tire radius and spacing leave positive clearance between adjacent
road tires and the end disc. The articulated steel belt retains connectors,
guide horns and separate rubber-padded shoes when selected. Wheeled vehicles
use open bead annuli: the previous filled black sidewall caps covered the rims
and nuts. Side raycasts now verify the actual painted wheel center is visible
on both sides of every six/eight-wheel configuration.

All new colors inherit explicit material tokens. Road tire tread uses rubber;
tracked shoes remain steel. Radar panels and radomes use structural surfaces,
with glass reserved for optical fittings. No material is inferred from final
RGB values.

## Manufacturer references

These primary pages inform the visual layout and silhouette. The meshes remain
original configurable game art; dimensions and part arrangements are authored
for the existing fictional equipment catalogue.

- [BAE Systems CV90 MkIV](https://www.baesystems.com/en/product/cv90-mkiv): the relationship between a tracked troop body, compact turret and rear compartment.
- [Patria AMV XP](https://www.patriagroup.com/products-and-services/protected-mobility/wheeled-mobility/patria-amv-xp-number-1-in-the-battlefield): continuous carrier body, inclined bow and visible wheeled suspension. The game retains its existing six/eight-wheel component choices.
- [KNDS Jaguar](https://knds.com/en/products/systems/jaguar): distinct reconnaissance vehicle proportions and concentrated observation/weapon fittings.
- [KNDS PzH 2000](https://knds.com/en/products/systems/pzh-2000): long tracked carrier, rear fighting compartment and slender projecting gun.
- [Rheinmetall Skyranger](https://www.rheinmetall.com/en/products/air-defence-systems/mobile-air-defence-skyranger): clearly separated gun/sensor roles around a compact mobile mount. The game keeps its selected twin-cannon or missile installation.

## Measured budgets and verification

Direct `build({platform})` triangle counts after the pass:

| Platform | Inspection | Catalogue | Map pin | Inspection GLB bytes, plain exporter |
| --- | ---: | ---: | ---: | ---: |
| IFV | 44,222 | 5,862 | 1,144 | 4,781,416 |
| APC | 26,746 | 4,197 | 982 | 2,893,284 |
| Reconnaissance | 23,726 | 4,476 | 944 | 2,567,116 |
| Artillery | 45,330 | 5,182 | 1,144 | 4,901,332 |
| Air defense | 41,178 | 5,632 | 1,112 | 4,452,660 |

The existing specialist limits remain 48,000/12,000/1,500 triangles and a
5,000,000-byte default shipping asset. The normal, winding, nondegeneracy,
cross-LOD bounds and component-distinction requirements remain unchanged.
Shipping GLBs include small catalogue/material metadata differences from the
plain exporter sizes above; the integration build checks their actual bytes.

The pre-change source, five inspection GLBs and full hash manifests were saved
outside the repository in `../specialist-reference-research/` before edits.
Its audit checks 642 tank/air option-and-LOD cases, including material sidecars.
The final audit also passes 294 fitted specialist option-and-LOD cases with
the original part identities/order, complete material classes, full normal
contract, unchanged triangle bands and unchanged cross-LOD bounds tolerance.
Every compared fitted component option remains distinct under the existing
coverage metric. The largest inspected single-option specialist configuration
is the artillery vehicle at 46,114 triangles, below the 48,000-triangle band.
Only the 15 specialist shape pins in `check_tank_redesign.cjs` and the 10
specialist coarse pins in `check_equipment_mesh_detail.cjs` were deliberately
updated. Every approved tank and aircraft pin remains unchanged.

`check_specialist_redesign.cjs` adds seven tests for preservation, hull
proportions, low IFV cheeks, running-gear clearance, visible wheeled rim centers,
flat bow plates across LODs and non-optical radar surfaces. Running it against
the archived previous source gives six expected failures and one preservation
pass. Against the new source all seven pass. The existing detail, realism and
tank-redesign suites bring the focused geometry total to 34 passing tests;
all 23 shared material tests also pass against these meshes.
