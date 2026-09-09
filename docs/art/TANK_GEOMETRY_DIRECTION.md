# Tank geometry direction and completed redesign

The tank-only redesign responds to the request for a much stronger visual improvement using online inspiration. It replaces the main proportions and assembled shapes, rather than increasing detail count. These remain original configurable game vehicles. Manufacturer references inform the art direction; no model is presented as an exact Abrams, Leopard or Leclerc replica, and no simulation performance values change.

## Primary reference sources

- [General Dynamics Land Systems: Abrams](https://www.gdls.com/abrams/). The manufacturer's vehicle photographs and variant overview are references for a broad armored turret, long chassis and a gun assembly subordinate to those large forms. The text identifies the 120 mm main armament and distinguishes vehicle variants; it is not used to infer hidden construction or dimensions.
- [KNDS: Leopard 2 A8 brochure](https://media.knds.com/uploads/KNDS_B_Ansicht_LEOPARD_2_A8_EN_056fd1a8ad.pdf). The manufacturer's photographs provide a direction for coherent angular armor packages and the relationship of turret, mantlet and hull. The redesign's wedge modules are original geometry, not copied plates or specifications.
- [KNDS: Leclerc XLR](https://knds.com/en/products/systems/leclerc-xlr) and [manufacturer brochure](https://knds.com/media/KRV_041_LECLERC_XLR_EN_BAT_BDEF_6a502524d1.pdf). KNDS explicitly describes an autoloader and modular protection. This supports keeping an autoloader turret as a distinct design choice, with its own bustle geometry, rather than treating every turret as the same larger box.

Sources were consulted on 8 September 2026. These pages and photographs are reference material, not redistributable game assets. The dimensional choices below are authored art decisions measured from our generator, not claimed measurements of photographed vehicles.

## Why the preceding model looked assembled from blocks

The source audit found several large shape problems that survived the earlier detail pass:

1. The standard authored hull was only 5.67 m long while the complete vehicle was 4.146 m wide. Its short, wide stance overwhelmed the useful track and panel detail.
2. The turret used a narrow five-band loft, an exposed cylindrical mounting ring, separate thin cheeks and a small rear block. Those disconnected masses weakened the sense of one armored body.
3. The main gun started with a box, an exposed transverse drum, several collars and a canvas cone. Large sleeves and repeated bright bands made the barrel look thick and segmented.
4. Small road wheels occupied a much taller track envelope. The end wheels were offset upward from the curved belt centers; the depicted front drive sprocket also disagreed with the rear powerpack.
5. Heavy protection added three stair-like ledges to each cheek and two horizontal rows to the glacis. Tall side bins and engine-deck drums contributed another layer of boxes and cylinders.
6. The ammunition bay's braces connected a turret-carried assembly to the fixed engine deck. That attachment would visually contradict turret rotation.

## Implemented form changes

- **Chassis and running gear:** standard authored hull length is now 6.68 m, with 3.826 m overall default width. The heavy authored hull is 7.30 m long. Tank geometry shares these revised proportions across inspection, catalogue and map LODs. Road wheels are larger circular assemblies; the idler and drive sprocket are concentric with their track returns. The drive sprocket is now at the rear.
- **Turret:** a broad, low body carries continuous cheek and rear bustle forms. The mounting ring sits under the overhang. A narrow roof bevel leaves large readable armor planes. Compact, heavy, autoloader and fixed-casemate choices retain distinct shapes and their original selection semantics.
- **Gun:** a short recessed mount, restrained fabric seal and continuous slender barrel replace the oversized stack. The bore evacuator and thermal-sleeve joints remain visible without turning the gun into a row of beads. Selected armaments retain distinct dimensions and muzzle reach.
- **Protection:** each reinforced cheek is a deep swept wedge with a thin service seam. Flank modules are a single aligned row. A continuous sloped glacis bank replaces the horizontal staircase; it replaces the ordinary small applique plates when selected.
- **Attached fittings:** shallower side bins and low engine intake caps stay subordinate to the main silhouette. The ammunition support braces terminate on the rotating rear assembly. Existing track links, guide horns, hinges, periscopes and tools remain.

The implementation adds an explicit per-vertex `materialClasses` sidecar for the tank surface renderer. Palette identities survive `shade()` and `weathered()` through authored tokens; classes are not inferred from final RGB. This metadata does not change any existing position, normal or color array on other vehicle families.

## Compatibility and costs

All part names, order, slots and labels match the pre-redesign generator for the tested tank option space. The normal-quality tolerances, triangle budgets and 0.35 m bounds agreement between LODs remain unchanged. Tank LOD hash pins were deliberately recaptured because the user requested new primary proportions at every level. Specialist hashes retain their previous values.

Current simulation-catalogue defaults, including their selected components:

| Platform | Inspection triangles | Catalogue triangles | Map triangles |
| --- | ---: | ---: | ---: |
| Standard tank | 69,736 | 8,426 | 1,180 |
| Heavy tank | 75,068 | 6,238 | 1,252 |
| Light tank | 69,724 | 8,438 | 1,164 |
| Tank destroyer | 68,776 | 8,242 | 1,108 |

The fully loaded heavy fixture has 84,244 / 7,174 / 1,340 triangles. Direct `build({platform})` defaults differ from catalogue specifications: standard/light/destroyer use 68,872 / 8,050 / 1,168 and heavy uses 74,144 / 5,834 / 1,204. The 12 MB tank GLB shipping cap is unchanged; the export pipeline checks actual serialized bytes.

## Verification and before-state evidence

`node --test tools/ui/check_tank_redesign.cjs tools/ui/check_equipment_mesh_detail.cjs tools/ui/check_equipment_realism.cjs` passes **27 tests**. The new shape checks inspect chassis proportions, turret mass, barrel slenderness, continuous wedge faces and wheel/track relationships. Each of those five checks was run against the archived previous generator and failed on the behavior it is intended to reject. The material-sidecar check also rejects that previous generator.

Additional read-only sweeps recorded:

- **348** tank option/LOD cases passing the existing normal, winding, triangle, bounds and exact part-identity checks.
- **76** non-default tank component comparisons, all distinct under the unchanged production silhouette metric: **zero weak or absent**.
- **864** aircraft/specialist option/LOD cases with byte-identical position, normal and color buffers and unchanged part identities.
- **60** heavy-protection/turret/platform/LOD combinations passing the full existing normal contract and triangle ceilings.

The sibling workspace directory `work/tank-reference-research` preserves the pre-change generator, twelve before-state tank GLBs, hash manifests, coverage/identity audit receipts and negative-control results. These review artifacts are not runtime assets. Full UI integration and regenerated shipping GLBs remain the responsibility of the normal export pipeline.

This document supersedes the tank dimensions and tank coarse-hash preservation statements recorded for the preceding pass in `EQUIPMENT_REALISM_PASS.md` and `EQUIPMENT_MESH_DETAIL.md`. Aircraft and specialist improvements from those passes remain intact.
