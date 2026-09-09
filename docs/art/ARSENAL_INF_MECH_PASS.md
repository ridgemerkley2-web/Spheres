# Mechanised formation IFV — 8 September 2026

The independent `ArsenalModels` entry `inf_mech` is a mechanised infantry formation represented by a tracked infantry fighting vehicle and a dismounted soldier. Its name, ID, Infantry class, 6.7 m published span, scene bounds and catalogue role are unchanged. It is not replaced by or remapped to the configurable `ground_ifv` equipment family. No simulation or equipment availability changes belong to this pass.

The close mesh now has a coherent front engine/driver station, a lower welded cannon housing ahead of the rear troop bay, two physical roof exits and an external rear ramp. Its right-side engine grille follows the sloping deck beside the driver's hatch; periscopes sit on that deck rather than floating above the glacis. The troop bay keeps a continuous roof behind the turret. The ramp is a thin plate outside the rear bulkhead, with a lower hinge, separate personnel-door face and handles.

Six road-wheel stations on each side replace the former five. Their suspension arms reach from hull mounts to the wheel hubs, complementing the existing linked belt, return rollers and end wheels. The shoe loop is retained so the ground contact and exact transverse envelope stay stable. Broad, thin flank armour and four serviceable guards use restrained consistent colour and narrower chamfers. Reducing repetitive fasteners funds the structural changes instead of adding triangles to an already full card budget.

The low autocannon, side missile tubes, sights, smoke dischargers and turret hatch remain recognisable mission equipment. Existing antennae, rear stowage rail, front recovery fittings and dismount retain the original scene bounds. The art remains an original representative IFV concept; it does not claim to reproduce a particular historical vehicle.

## Preserved contracts

- Only `inf_mech` **near** position, normal and colour buffers change. All other 45 near meshes and all 46 far meshes match their archived SHA-256 hashes exactly, including `arm_gen2`, `arm_gen3` and `trophy`.
- All IDs, metadata, ordering and exact near/far bounds remain unchanged. There is no new public part or model mapping.
- The far `inf_mech` remains the existing 632-triangle representation. It retains the old simpler layout, so it is an intentionally coarser recognition model rather than an exact rendering of every revised near feature.
- The independent catalogue renderer keeps its existing shared military materials. The new authored `TankSurface` treatment applies to configurable armoured equipment and is not silently injected into this formation scene.
- No shared geometry helper, renderer, texture, simulation or server behaviour changes are required. The existing API comment is updated with the resulting catalogue counts.

## Measurements

| Buffer set | Before | After |
| --- | ---: | ---: |
| `inf_mech` near triangles | 15,760 | 15,196 |
| `inf_mech` far triangles | 632 | 632 |
| Complete 46-model near catalogue | 343,519 | 342,955 |
| Complete 46-model far catalogue | 39,508 | 39,508 |
| All 92 position/normal/colour buffer sets | 39.45 MiB | 39.39 MiB |

The near reduction is 564 triangles. The existing 16,000 near, 1,500 far and 40 MiB combined attribute-buffer ceilings remain unchanged. These counts cover CPU vertex attributes, not GPU copies, renderer framebuffers or transient authoring allocations.

The exact near minimum remains `[-1.6887860298156738, -0.05352136120200157, -0.10000000149011612]`; maximum remains `[2.3899717330932617, 2.824749708175659, 6.800000190734863]`. The small negative Y extent belongs to the retained shoe geometry; no bounds-padding geometry was added. The new near hash is `5ece92ffe4420f168940c6c39b30683e6caed67cd5affba378bd7a1e05b47a88`; far remains `a90be9635e44b7ced548a6d78ceeebdd5e83b27de3a96651cb772165dd08b681`.

## Verification

The pre-pass 46-model hashes, counts, metadata and bounds are recorded in `tools/ui/fixtures/inf-mech-preservation.json`. `check_arsenal_inf_mech.cjs` exercises the actual recipe to verify the preserved buffers, six linked suspension stations, front-engine/rear-bay layout, exposed ramp and roof-hatch faces, smooth normals, winding, finite nonzero face area and existing budgets. The preceding air/naval pass test now explicitly pins this later authorised `inf_mech` revision rather than claiming that its original near buffer is still current. Its other 45 assertions remain unchanged.

```text
node --test tools/ui/check_arsenal_inf_mech.cjs tools/ui/check_arsenal_models.cjs tools/ui/check_arsenal_model_detail.cjs tools/ui/check_arsenal_realism.cjs
```

All 29 checks pass, including the existing OBJ, determinism, exact-bound and far-buffer tests. Actual visual review is performed in the parent integration task.
