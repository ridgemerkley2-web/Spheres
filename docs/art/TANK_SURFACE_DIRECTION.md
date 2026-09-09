# Armoured-vehicle materials and finish — 8 September 2026

This pass gives the four configurable tank platforms and five specialist armoured platforms an authored surface response. It does not change aircraft, ships, infantry or civilian art. These are original game concepts and art-directed materials, not measured historical vehicle paint specifications. The public `TankSurface` module name and its existing viewer hooks remain compatible.

The specialist extension enables exactly `ground_ifv`, `ground_apc`, `ground_recon`, `ground_artillery` and `ground_air_defense`: infantry fighting vehicle, armoured personnel carrier, reconnaissance vehicle, self-propelled artillery and air-defence vehicle. It reuses the approved tank shader, palettes, roughness, noise, wear and contact bake without changing any of those functions. The geometry pass separately owns their body and mission-equipment shapes.

## Why the previous tanks looked like bright plastic

The former sand/winter conversion classified any sufficiently green-biased RGB as paint. The palette's steel, bright fittings, track shoes and cable all passed that condition. A sand repaint therefore turned track metal tan. The division by one generic luminance value also amplified upper armour by about 1.25 and edge caps by about 1.39, producing pale cream lids and very conspicuous block outlines. The shared renderer then inferred material properties from those recoloured RGB values, so useful metal/rubber separation disappeared a second time.

Large repeated bright bevels and bins also exaggerated the old geometry's box construction. Material changes cannot correct silhouette or proportions; the concurrent tank geometry pass owns that work.

## Explicit authored classes

`equipment-mesh.js` propagates palette identity through `shade()` and its callers into one `Uint8Array materialClasses` entry per vertex. The numeric colour buffers are not used to guess the material. The same semantic part can contain multiple materials; part names, first/count ranges, slots and triangle picking therefore remain independent of this sidecar.

| Class | ID | Response |
| --- | ---: | --- |
| Armour paint | 0 | Dielectric, roughness 0.84; the only class receiving camouflage/repaint |
| Steel fittings | 1 | Metallic 0.90, roughness 0.58 |
| Steel track shoes | 2 | Metallic 0.86, roughness 0.67; dry-earth deposits |
| Rubber and dark cavities | 3 | Dielectric, roughness 0.93 |
| Optics | 4 | Dielectric, roughness 0.17; restrained glazing reflection |
| Canvas | 5 | Dielectric, roughness 0.96 |
| Cable | 6 | Metallic 0.70, roughness 0.73 |
| Lamps | 7 | Dielectric, roughness 0.28 |
| Unclassified | 255 | Retains its source colour; never treated as paint |

`TankSurface.supports(mesh, platform)` activates only for `tank_standard`, `tank_heavy`, `tank_light`, `tank_destroyer` and the five specialist IDs above, with a correctly sized typed sidecar. Current tank and specialist geometry contracts require zero unclassified vertices. A material's base class remains constant across a primitive, allowing its properties to pass through the existing triangle pipeline without changing indices or draw order. A new or unrelated platform does not inherit this treatment merely because its ID begins with `ground_`.

## Finish and presentation

Olive, sand and winter use restrained paint palettes with no more than a few percent of the old baked plane variation. Woodland adds connected, metre-scale green/brown/charcoal shapes, sampled from two continuous model-space noise fields. Narrow transition bands produce irregular paint boundaries instead of broad airbrushed gradients. No pattern depends on triangle or part index.

Factory, service and field are explicit visual conditions, not claims about campaign damage, equipment age or readiness. Service is the default. Dry-earth deposits favour the lower hull, running gear and upward-facing surfaces. Optics and lamps retain their clear material. Canvas has its own subdued variation. This is a stylised wear treatment, not a full weathering simulation or a UV-authored historical camouflage pattern. No random rust, bright edge outlining, or supposed anti-slip coating is applied indiscriminately.

The dedicated `TankSurface.glsl` uses authored roughness and metallic response. It does not project the shared blue sheet-metal texture onto cast/welded armour. Neutral studio fill and a broad environment response retain readable steel and rubber in shadow while the paint stays matte. Existing actual directional self-shadows remain. The tank highlight is a restrained rim-weighted cool tint rather than an opaque yellow wash. Camera and bench background are owned by the viewer integration.

## Bounded contact shading

`prepare(mesh)` rasterises triangle surfaces into a temporary occupancy grid. Five outward directions, each with at most six short probes, estimate nearby surface occlusion. Quantised position **and normal** deduplicate split vertices without merging opposite sides of a plate. This is an approximate local contact bake; it is not triangle ray tracing, global illumination or a guarantee that every tiny recess is resolved. It detects nearby plates and fittings instead of darkening everything based solely on height.

The bake uses cells at least 85 mm wide and at most 112 cells across the longest model axis before padding. Triangle raster work is limited to 2.4 million samples; unusually costly geometry lowers raster density uniformly. Supported input is at most 900,000 vertices and 1,000 metres per axis. Contact factors stay between 0.715 and 1. The temporary grid, deduplication map and raster-step buffer are released after preparation. A WeakMap retains only per-mesh contact values, packed parameters and small metadata while the mesh remains reachable.

A representative pre-final-geometry snapshot of the four default inspection tanks measured 127–158 ms for the initial contact/material preparation, 33–58 ms for preview base-colour baking and 36–55 ms for the sampled export finish on the development machine. The 206,616–222,432-vertex meshes used about 4.13–4.45 MB of cached typed arrays; the extra uploaded `vec4` material buffer accounts for about 3.31–3.56 MB. The transient occupancy grid was 192–280 KB, plus the deduplication map and raster steps. These are measurements, not device latency guarantees or fixed geometry budgets. Camera movement and per-frame rendering do not rerun this CPU bake.

## API and export boundary

The offline UMD module exports `classes`, `finishes`, `wearLevels`, `supports`, `prepare`, `bake`, and `glsl`.

- `prepare(mesh)` returns cached `{ ao, parameters, min, max, stats }`. Parameters are a `Float32Array` of roughness, metallic factor, material class and contact factor per vertex.
- `bake(mesh, { finish, wear, baseOnly: true })` supplies unpatterned authored colours for the live shader.
- `bake(mesh, { finish, wear })` samples the finish, deposits and contact shading into ordinary RGB vertex colours for export.
- Viewer controls call `setFinish('olive'|'sand'|'winter'|'woodland')` and `setWear('factory'|'service'|'field')`. Native controls use `data-model-finish` and `data-model-wear`. The existing `data-model-tank-only` wrapper attribute is kept for compatibility; visibility is governed by supported authored armour metadata, so the five specialists now expose it too. Unsupported vehicles or missing material metadata hide these controls and retain the ordinary viewer.

The live shader samples camouflage and deposits **per fragment**, so a broad two-triangle armour plate still displays the pattern. The current GLB format has one standard material and `COLOR_0`; it receives sampled vertex colours and unchanged geometry/semantic ranges. Large faces therefore export an **approximation** of the live camouflage and wear. Floating-point precision also differs between CPU and GPU noise. The live shader and authored per-class response are not embedded into this GLB. Matching every camouflage boundary in external viewers requires a future texture/UV export or controlled geometry subdivision and a separately reviewed loader/export contract.

## Verification

`tools/ui/check_tank_surface.cjs` covers all four tank and five specialist platform/LOD sidecars, paint-only recolouring, bounded bevel brightness, wear/material separation, explicit roughness and metalness, geometry-dependent contact shading, vertical-translation invariance, determinism, source-buffer immutability, cache/work bounds, malformed metadata, GLB colour/part preservation, live attribute/uniform integration, aircraft isolation and context restoration. The specialist checks exercise real ground meshes, native control visibility, selected-condition exports, missing-module/metadata fallback and transitions to aircraft. A shader hash and representative four-tank material/finish buffer hashes preserve the approved tank appearance exactly during the extension. The existing viewer and shadow suites remain applicable. Actual browser shader compilation and visual inspection are recorded by the parent integration task.

The separately acquired Strv 103 author render was inspected as a visual reference for continuous plates, restrained green paint, steel/rubber contrast and selective wear. Its photo/mesh/textures are not copied into this procedural material module. Any independently shipped historical reference asset retains its own provenance and licence record.
