# Equipment form and assembly realism

The tank-only redesign in [TANK_GEOMETRY_DIRECTION.md](TANK_GEOMETRY_DIRECTION.md) supersedes the tank dimensions, counts and coarse-hash preservation statements below. This document records the preceding pass; its aircraft and specialist geometry remains unchanged by the tank redesign.

This pass refines the original configurable game models in `spheres-web/ui/equipment-mesh.js`. They remain visual concepts, not representations of a named historical vehicle or engineering specifications.

## Visible changes

- Tank and specialist bows use folded armor planes instead of the broad cylindrical nose casting. The new plates stay below the upper deck and meet the existing hull; seams follow their upper edge.
- Rotating tank turrets retain a wide crew compartment but narrow their front roof around the gun opening. Casemates gain an intermediate armor shoulder. Existing roof heights, gun mounts, fittings and component choices remain in place.
- Tank headlamps are smaller forward-facing housings carried by brackets attached to the upper deck. Their faces use a neutral tone; sight optics retain their existing colors.
- Aircraft fuselages use monotonic curved interpolation between their authored cross-sections. Interpolation is bounded by adjacent sections so it cannot introduce an accidental bulge. Wing roots now have upper and lower fairing volume joining the wing and body.
- Each aircraft has two exposed side inlet mouths, rolled lips, recessed duct walls, and fan assemblies. Single-engine choices retain one engine part; twin-engine choices retain two. The engine casing ends behind the inlet throat so an opaque cap cannot obstruct it.

The previous track, suspension, panel, hinge, airfoil, canopy and service-detail work remains. Geometry is deterministic and retains the existing part names, order, slots and picking behavior. No simulation or component-performance values change.

## Cost and compatibility

All 18 existing ground catalogue/map meshes retain their exact position, normal and color hashes at LOD1 and LOD2. Aircraft continue to use their inspection mesh at every requested LOD, as before this pass.

Triangle counts for the direct `build({platform})` defaults are:

| Platform | Triangles |
| --- | ---: |
| Standard tank | 69,584 |
| Heavy tank | 74,868 |
| Light tank | 69,584 |
| Tank destroyer | 69,584 |
| Infantry fighting vehicle | 43,398 |
| Armored personnel carrier | 27,174 |
| Reconnaissance vehicle | 24,118 |
| Artillery | 45,142 |
| Air defense | 40,502 |
| Light attack aircraft | 22,272 |
| Tactical strike aircraft | 25,248 |

These direct defaults are not a substitute for simulation catalogue specifications: selected components can change the counts. The shipping budgets remain 12 MB for tanks and 5 MB for default specialist ground GLBs. Focused tests check raw geometry plus a metadata allowance; the separate export pipeline remains responsible for measuring actual serialized GLB bytes and round-trip fidelity. No budget, coverage threshold, or normal-quality tolerance was increased for this pass.

## Verification

`node --test tools/ui/check_equipment_realism.cjs tools/ui/check_equipment_mesh_detail.cjs` passes all 20 tests.

The new checks inspect armor planarity, turret roof narrowing, casemate shoulders, physically attached neutral headlamps, curved forebody sections, and volumetric wing-root fairings. Rays fired into both inlet mouths on every legal engine configuration must reach the recessed fan without hitting a blocked casing cap. Affected turret and engine configurations also pass the existing full finite-coordinate, nondegenerate-triangle, winding, unit-normal and shared-curved-normal contract.

The retained detail tests verify all 18 coarse hashes and part identities, guide-channel and horn geometry, unchanged meter scale, deterministic immutable inputs, and hydropneumatic component distinction using the unchanged production coverage metric. Full component coverage and regenerated GLB verification run through the normal integration pipeline after this geometry is frozen.
