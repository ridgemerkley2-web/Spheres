# Military mesh and lighting upgrade — 8 September 2026

This document records the first inspection pass. The subsequent [form and surface refinement](MILITARY_REALISM_UPDATE.md) adds revised armor, aircraft and ship forms plus bundled CC0 normal/roughness maps, and records the latest validation and measurements.

The equipment designer now has physically modeled close-up fittings and cached mesh self-shadows. The shared arsenal renderer uses a local material-lighting shader for military models, preserving the separate character and civil-art lighting paths. This is original configurable game art, with researched third-party candidates documented separately.

## What is available

- Eleven configurable platform families: four tanks, five specialist ground vehicles and two tactical aircraft. Paired road tyres, tapered guide horns, track connectors, suspension pressure reservoirs, panel hardware, curved aircraft surfaces and gear fittings are actual triangles. Specifications, picking ranges, dimensions and simulation values remain separate from presentation.
- All 46 catalogue recipes gain near-view hardware. Their total near geometry is 338,727 triangles, up from 272,491. IDs, published spans and exact bounds are preserved. All 46 far meshes and all 18 configurable ground LOD1/LOD2 meshes remain byte-identical.
- The designer's directional shadow map uses one cached 1024-square RGBA8 texture and DEPTH_COMPONENT16 renderbuffer (approximately 6 MiB together). Devices with smaller limits use 512-square targets. Unsupported or failed shadow passes keep the projected-shadow fallback. Camera, paint, name and selection changes reuse depth; mesh changes invalidate it. No additional WebGL context or duplicate geometry buffers are allocated.
- `military-surface.js` supplies GGX/Smith lighting, Fresnel response, roughness variation, subdued dust, glass reflections and tone mapping. Material classes are inferred from the existing vertex palette; these are artistic approximations, not authored metalness, normal or roughness textures. Catalogue models share this material response but do not gain the designer's shadow map.
- Twelve regenerated GLBs contain the actual updated meshes. Their embedded vertex colours, semantic parts and configuration metadata still round-trip through the existing importer. Renderer self-shadows and material lighting are not baked into exports; other GLB viewers use their own lighting.

## Review

[Open the military model workshop](http://127.0.0.1:7841/tools/arsenal/military-inspection.html). It uses the actual game mesh and renderer modules, selectable platform presets, finishes, part selection, camera controls, GLB download and a balanced catalogue of armour, air, naval, missile and space models. It has no campaign connection or API commands. Aircraft detail selection is disabled because those two procedural aircraft still have inspection geometry only.

Desktop browser review covered the standard and upgraded heavy tanks, sand and olive materials, side and perspective views, zoom, selected running gear, tactical aircraft, all sixteen displayed arsenal cards and real switching between inspection and the 1,164-triangle map form. The reviewed upgraded heavy preset has 85,412 triangles and 24 selectable parts. No new console warning or error remained after fixing the shader uniform precision mismatch; the browser log retains that earlier failure at 22:19 UTC.

The new native game binary was built but not launched. A previous automatic approval review rejected the preview launch with “blocked by policy”; no new launch, port workaround, process replacement or game mutation was attempted for this task. The existing static server supplies this art review.

## Validation

- Complete serverless UI suite: **1,203 passed**, zero failures/skips, 69.7 seconds. Workspace log `work/military-ui-tests-final.log`.
- Native release web suite: **274 passed**, zero failures, three existing ignored tests. Workspace log `work/military-native-tests.log`.
- Release build passed. Workspace log `work/military-release-build.log`.
- Release executable `company-sim-target/release/spheres-web.exe`, SHA256 `CFAD8F67AC36C9E8A9FD22608E553BBADE07F450D1760C8151E66CBB39A384EE`.
- All twelve generated GLBs pass byte-for-byte regeneration checks and export/import validation. Reference tank files range 7.54–8.24 MB, specialists 2.61–4.87 MB and aircraft 1.90–2.31 MB.
- Component coverage: **186 distinct / 0 weak / 0 absent**, using the unchanged visual-distinction criteria. Four hydropneumatic tank upgrades gained connected reservoirs/lines after the extra base detail initially weakened their distinction.
- Shadow allocation failures, draw failures, caching, context loss/restoration and cleanup have explicit tests. Detail-level changes now invalidate the viewer mesh identity; otherwise the preview could report a lower triangle count while continuing to display and export the previous mesh.
- Existing normals, winding, no-degenerate-triangle, part continuity and deterministic generation checks remain intact. New geometric checks were verified to reject the preceding meshes. `git diff --check` passes.

The near-art budgets were deliberately revised for the requested detail: catalogue cards 16,000 triangles, specialist inspection designs 48,000, shipping tank GLBs 12 MB and specialist GLBs 5 MB. Coarse budgets and the 1.2-million-triangle shared renderer cache are unchanged. The complete near catalogue is 34.89 MiB of CPU attributes; far attributes add 4.07 MiB, with GPU uploads and other renderer resources additional.

## External assets and next quality step

[Free military asset research](FREE_MILITARY_ASSET_RESEARCH.md) records eight primary-source candidates, author/license evidence, formats, download status and exclusions. Detailed T-90 and F-22 candidates require authenticated official downloads. Two lower-detail CC0 Blender sources were downloaded outside the repository for inspection; neither is shipped or represented as an approved high-resolution replacement.

The current Spheres GLB importer only supports its own unindexed position/normal/colour format. A future third-party textured model path should support indexed geometry, authored PBR textures and node transforms, with prepared near/far LODs and explicit component-to-part mapping. Those models cannot be dropped into this strict round-trip importer while retaining their textures and articulation. No external art, texture or CDN dependency was added by this pass.

Geometry detail and per-platform measurements: [configurable equipment](EQUIPMENT_MESH_DETAIL.md) and [46-model arsenal](ARSENAL_MESH_DETAIL.md).
