# Town/site budget work and remaining runtime scope

Base: 5d86fb35507acbd8fa4662ea9dc3f5f566ef93da. Source frozen after changes to town-mesh.js, site-mesh.js, check_site_mesh.cjs, and new check_construction_budget_units.cjs. No commits, builds, downloads, or source threshold edits by this agent.

## Completed

- Metadata-only proof before geometry optimization: 320 site configurations, 204 town variants, 40 town blocks retain identical positions, normals, colors, bounds, and parts. See metadata-geometry-preserved.json.
- budgetUnits gives exact disjoint vertex ranges; budgetBuildings declares physical buildings. Shared envelope ranges are charged entirely to each owning building, once to the assembly. No division by dwelling count.
- Row terrace maximum: 16,382 actual assembly triangles; six dwelling assessments 4,828–4,878 including full common envelope and adjacent party stacks. University maximum: 16,656 actual assembly triangles; main range 9,472, two wings 2,298 each, remaining 2,588 terrain/props.
- Site part ownership is explicit at authoring calls. Attached roof plant, rooflights, wall vents, canopies and loading docks remain assigned to their buildings. Physical secondary halls and delivered wings are independent building units. Compound totals retain all yard and construction geometry.
- Complete near main halls omit only inner flange, inward anchor bolts, inner haunch, rafters and apex plate from five fully enclosed interior portals. End portals, external steel, every incomplete stage, secondary halls and all map geometry are preserved. Reduction: 980 actual triangles per complete near site. Arms plant lead hall 12,670 -> 11,690; whole compound 35,300 -> 34,320.
- Independent containment tests prove every removed vertex lies above the existing slab, behind the walls/gables and below the opaque roof. Other parts' attribute bytes are identical. Front/rear 512px WebGL comparisons each changed 0 of 262,144 pixels; viewed front pair manually. See site-enclosed-portal-render-result.json and image pairs.
- Final focused command: node --test tools/ui/check_construction_budget_units.cjs tools/ui/check_site_surface_optimization.cjs tools/ui/check_site_mesh.cjs tools/ui/check_town_surface_optimization.cjs. 29 passed, 0 failed/skipped/cancelled, 67.892s. Final log: construction-budget-units-final-tests.log. Initial stale reviewed near-geometry hash was updated only after containment/unchanged-part/pixel proofs; original map pin was retained. git diff --check passed.

## Two raw town scenes remain above the unchanged 150,000 ceiling

- mixed, numeric id1997: 163,671, including 145,583 building triangles and 18,088 streets/land.
- residential, numeric id1994: 174,540, including 155,984 building triangles and 18,556 streets/land.
- Conservative convex closed-solid profiling would remove only 962/max terrace, 551/max house, 391/campus, 726/park. Estimated ~10k mixed/~14k residential, insufficient to clear both; no global culler or geometry approximation was added.
- Existing mid maxima across ids1990–1997: mixed52,332, residential55,256. Map maxima3,878/4,098. These are distinct authored tiers, not substitutes relabelled as close.

## Actual consumers (read-only inventory)

- Main playset index.html:17506–17510 registers a town provider. No main-playset code produces or mounts a town: id. The provider is dormant.
- Actual globe city geometry uses CityMesh.build at index.html:11218, and city popup uses CityMesh at14025 then mounts city: at14043. Detailed TownMesh blocks are not the present campaign city view.
- tools/arsenal/art-gallery.html:71,155–158 is the only non-test concrete TownMesh.block consumer. It is an unshipped review bench; its header explicitly says the game does not load it. It renders id as the STRING "1990", not the numeric benchmark seed; do not confuse those layouts.
- Other uses are art manifests, benchmark, tests/cache/framing harnesses.
- arsenal3d.js:279–340 uploads one merged mesh. renderTo:506–555 has one gl.drawArrays call for the entire entry. No per-lot frustum culling, camera-dependent LOD or per-building draw list exists. Instancing/indexing would lower storage/processing overhead, not the submitted triangle budget.

## Minimum legitimate adaptive follow-up (not implemented)

1. Expose a stable town scene descriptor from the same layout/variant cache: full bounds, streetscape, per-lot transform/scheme, physical bounds, and close/mid/map meshes. Preserve identical lot IDs, placement and seeds across all tiers.
2. Add a scene rendering path to Arsenal3D that computes projected screen bounds from its actual MVP/viewport, frustum-culls each lot and selects a tier using projected feature size with hysteresis. Reuse mesh buffers per variant/scheme; do not rebake/upload a full block every camera frame.
3. Keep selected or sufficiently large buildings at full close detail; use existing cheaper geometry only where details are subpixel. Provide actual building focus/zoom on the review surface (and any future game consumer), keeping the full close assets available.
4. Count real GL draw submissions at initial overview and multiple viewport/DPR/rotation/zoom/selection cases; assert <=150k visible initial triangles. Assert that focus promotes to close, no building disappears on a tier switch, all layouts are coherent, and caches are bounded. Preserve current raw close scene overrun records until this renderer path exists and its scope is explicitly documented.

This is renderer/provider/view work, not a safe one-line TownMesh recipe change. Raw scene overages were not reported as fixed.