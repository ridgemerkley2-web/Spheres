# Armored support vehicle rebuild — 8 September 2026

This pass extends the approved tank art direction to the five configurable specialist families: infantry fighting vehicle, armored personnel carrier, reconnaissance vehicle, self-propelled artillery and mobile air defense. The separate mechanized-infantry formation in the arsenal also receives a matching form pass.

## Inspect the vehicles

Open the [armored vehicle workshop](http://127.0.0.1:7841/tools/arsenal/armored-inspection.html). It uses the actual equipment mesh generator, material renderer, part picking and GLB exporter. The existing [tank workshop](http://127.0.0.1:7841/tools/arsenal/tank-inspection.html) remains available alongside it.

The workshop offers role-specific component lists, remembers each vehicle's draft while switching between families, preserves an inspected part across component/detail changes, and pairs weapons with their required mounts, ammunition and sensors. Choices and pairings are transcribed from `equipment_ground.rs` and `equipment_specs.rs`; research, installation capacity, performance, cost and campaign eligibility remain native game checks. This art preview does not issue campaign commands.

The [saved-draft follow-up](ARMORED_WORKSHOP_DRAFTS.md) adds automatic browser
storage for each family's components, finish, condition and detail level, with
validated restoration after a reload.

## Art direction

- IFV: a longer troop hull, continuous roof and glacis, lower integrated autocannon turret and rear egress.
- APC: a taller troop compartment, raked bow, compact protected station and visibly distinct six/eight-wheel running gear.
- Recon: a compact scout body, selectable machine-gun/autocannon station and observation equipment.
- Artillery: a longer chassis, rear-biased howitzer turret, recessed weapon mounting and loading installation.
- Air defense: a lower carrier with dedicated cannon cradles or missile equipment and a distinct radar installation.

The material treatment now supports all nine configurable armored platforms. It uses explicit paint, steel, track, rubber, glass, canvas, cable and lamp classes, with woodland camouflage, factory/service/field conditions and bounded contact shading. The tank shader itself remains unchanged. Wheeled tire tread blocks use rubber material; tracked shoes retain steel. See [surface direction](TANK_SURFACE_DIRECTION.md) for performance bounds and rendering details.

Canonical shipping GLBs keep raw generator vertex colors and exact specification rebuilds. Live custom exports sample the finish and contact shading at vertices. Fine per-fragment camouflage, wear and lighting are not reproduced exactly by sparse vertex colors in external viewers.

The models are original fictional game art. Manufacturer references inform their large forms and visible fittings; they are not measured historical reproductions or engineering plans. No newly downloaded third-party model or texture is required for this pass. The [specialist geometry report](SPECIALIST_GROUND_REDESIGN.md) records the CV90, Patria, Jaguar, PzH 2000 and Skyranger primary references, measurements and before-state audits. The [mechanized formation report](ARSENAL_INF_MECH_PASS.md) records the independent arsenal changes.

## Scope

The four approved tank designs and two aircraft designs are preserved. The separate `inf_mech` arsenal formation is refined within its existing budget. Other arsenal scenes, including missile trucks and command batteries, retain their distinct models. This pass does not add military-unit mesh placement to the active world map; the specialist meshes retain their existing inspection, catalogue and map detail-level API.

## Validation

- **1,296 UI checks passed**, zero failures/skips, in 82.57 seconds.
- **274 native tests passed**, zero failures and three existing ignored, in 79.76 seconds.
- Release build passed in 20.02 seconds. Executable SHA-256: `07CF09B625209A3AD1ADFFA731B2B8C12F6B649A44E04F3263421B4418A6A63A`.
- All **186 component comparisons** remain distinct, with zero weak or absent comparisons. Canonical GLBs regenerate exactly and the whitespace diff check passes.
- **642 tank/air configuration-and-LOD cases** preserve positions, normals, colors, material classes and picking identities. **294 specialist cases** pass unchanged geometry, identity, material and cross-LOD bounds contracts. Six new geometric regressions fail against the archived old source as intended.
- The mechanized formation uses **15,196 near triangles**, down from 15,760. Its 632-triangle far model, exact bounds, all other 45 near meshes and every far mesh remain unchanged.
- Actual browser review covered all five specialist families, open APC hubs, tracked-wheel clearance, an eight-wheel scout with mast/autocannon, automatic missile/radar/fire-control pairing, and the mechanized formation. No renderer warning/error logs were observed.

Final canonical specialist GLBs are 4,781,488 bytes (IFV), 2,893,352 (APC), 2,567,168 (recon), 4,901,380 (artillery) and 4,452,684 (air defense), all below the unchanged 5 MB cap. Geometry remains within the existing 48,000/12,000/1,500 triangle bands. Tank and aircraft GLB bytes remain unchanged from the approved tank pass.

The existing port 7841 static Python preview had stopped returning responses. It was restored on the same loopback port with file-backed logs. The native game executable was built and tested, but not launched; no campaign commands or saves were changed. The subsequent publication checkpoint is described in the [playset handoff](../PLAYSET_HANDOFF_2026_09_08.md).
