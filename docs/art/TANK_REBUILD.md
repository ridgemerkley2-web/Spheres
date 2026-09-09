# Tank workshop rebuild — 8 September 2026

The four configurable tank families now use longer hulls, integrated turrets, recessed mantlets, coherent armour banks and revised running gear. The change is in the actual designer meshes at all three detail levels. Component IDs, selections, semantic parts and simulation values retain their existing contracts.

Open the [tank workshop](http://127.0.0.1:7841/tools/arsenal/tank-inspection.html) on the existing static preview. It provides four presets, twelve independent component controls, four paint finishes, three cosmetic wear conditions, part inspection, detail levels and GLB downloads. It uses the game's real mesh generator and equipment renderer. The native designer also receives the tank finish and condition controls.

## Shape and materials

- [Geometry direction](TANK_GEOMETRY_DIRECTION.md): corrected hull and turret proportions, larger road wheels, aligned sprockets, finer guns and attached fittings. All tank detail levels were intentionally redesigned; older tank silhouette hash pins were updated. Aircraft and other ground vehicle geometry remain byte-identical to the preceding pass across 864 option/detail cases.
- [Surface direction](TANK_SURFACE_DIRECTION.md): explicit paint, steel, track, rubber, glass, canvas, cable and lamp classes replace colour guessing. Woodland camouflage is sampled per fragment; factory/service/field conditions control restrained wear. Bounded local contact shading and neutral studio lighting preserve readable dark materials.
- Inspection preset counts are 69,736 triangles for standard, 75,068 heavy, 69,724 light and 68,776 destroyer. Detail levels reduce the same forms for catalogue and map use. The existing triangle, export-size, geometry correctness and component-distinction limits were preserved.

The five canonical tank GLBs retain raw generator vertex colours and rebuild exactly from their stored specifications. Live designer downloads sample the selected finish and contact shading into vertex colours. Broad faces cannot reproduce every fragment-level camouflage boundary in an external viewer. The live shader and lighting are not exported as textures.

## Licensed textured reference

The workshop can separately load the author's [Stridsvagn 103](https://opengameart.org/content/stridsvagn-103), by canisferus / Lukasz Wesiora, under [CC BY 3.0](https://creativecommons.org/licenses/by/3.0/). This provides a detailed historical mesh to inspect alongside the original configurable designs. It has fixed equipment and is not mapped to campaign models.

The [converted asset package](../../spheres-web/ui/tank-assets/strv103/README.md) contains 25,675 triangles, 16 meshes and six embedded 2K textures in a 14,517,724-byte GLB. Four degenerate source faces were pruned. The original license, source hashes, modification notice, safe conversion code and validation receipts are included. The older source gloss/specular materials are approximated with core glTF materials; original scene units were not independently calibrated as metres.

An optional, locally bundled [model-viewer runtime](../../tools/arsenal/vendor/model-viewer/README.md) loads only when the reference is opened. This creates one additional renderer for the art reference. Six decoded 2K RGBA images with mipmaps can use approximately 128 MiB before renderer overhead; this cost does not apply to game equipment cards. The source GLB needs no external textures or compression decoders.

Primary design references were [General Dynamics Abrams](https://www.gdls.com/abrams/) and [KNDS Leopard](https://knds.com/en/products/leopard). [Asset research](TANK_ASSET_RESEARCH.md) records the other free candidates and why they were not selected.

## Validation and runtime boundary

The actual browser renders the configurable tanks and the converted reference without warning/error logs. Visual review includes the redesigned heavy tank in woodland, sand material separation, upgraded casemate configuration and the textured Strv 103. The workshop does not connect to campaign commands or saves.

Final main UI suite: **1,264 passed, zero failed or skipped** (71.21 seconds). The three focused workshop interaction tests also pass (0.39 seconds): selection retention, actual part events, and download failure/cleanup/retry. Native server tests: **274 passed, zero failed, three existing ignored** (64.32 seconds). The release build passed in 20.22 seconds. Exact canonical GLB regeneration and the licensed reference validator pass; all 186 component comparisons remain distinct.

The expanded hull exposed one legacy wear weakness: dust more than doubled an upward-facing black grille lip's luminance. Dark-material dust coverage was reduced from 45% to 40%, with the original brightness thresholds preserved and the failing coordinate retained as a regression. All 32 legacy wear checks pass. This does not change the new tank material shader.

The existing static preview is reused; the native build is not launched by this art pass. Final executable SHA-256: `27540DBC2F85A684C79F1DB45810A31AC4670C0FD22937D1708A95767040AB27`. Changes remain local and uncommitted.
