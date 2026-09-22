# Adaptive town scenes

The art gallery now renders town blocks as individual buildings with camera-driven
detail. The scene ceiling remains **150,000 submitted triangles**. Full building
inspection is preserved; the original merged close models have not been reduced.

## Scope

The active consumer is `tools/arsenal/art-gallery.html`, in **Town** and **Town
budget cases**. The latter opens numeric seeds 1997/mixed and 1994/residential,
whose complete close models contain 163,671 and 174,540 triangles. Their raw
counts remain visible in the gallery, P0 measurements, and legacy diagnostics.

This change does **not** replace the campaign globe's separate CityMesh renderer.
The ordinary game's dormant `town:` provider still exposes the original merged
mesh. No campaign-map performance claim follows from the gallery measurements.

## Geometry and rendering contract

`TownMesh.scene(options)` shares the existing `TownMesh.block` layout function.
Each lot retains its stable ID, exact placement, cardinal rotation, seed, shape,
and finish. All three existing building tiers use those same parameters. Rotated
normals and finish colors are baked exactly as for the merged block. A single
scene ground offset preserves the original relative building heights.

The descriptor retains the original close streetscape once, light lot metadata,
triangle counts and bounds, and lazy mesh factories. Culling bounds encompass
all three tiers. The merged `block()` and standalone `building()` APIs remain
available for inventory, exports, and comparisons.

`Arsenal3D.scenePlan(scene, options, previousHistory)` computes the camera and
the draw list used by `drawScene`. Options include device-pixel width/height,
yaw, pitch, zoom, and selected lot ID. Homogeneous frustum tests conservatively
handle buildings crossing the near plane. Projected bounds select close, mid,
or map detail, with separate enter/exit thresholds to avoid flickering near a
transition. The renderer caps device-pixel ratio at 2 and each canvas axis at
2,048 pixels.

If the requested tiers exceed the unchanged scene ceiling, the smallest
unselected buildings move to their existing lower tiers first. Every building
intersecting the frustum remains submitted. The selected building always keeps
its complete close mesh. An impossible draw budget throws an error instead of
dropping a building or hiding an overage.

Accounting includes the entire streetscape and every submitted building once,
including occluded triangles and partially clipped meshes. There is no separate
studio floor or shadow pass in this renderer. The budget does not divide a
building's triangle count by its number of lots or equate stored geometry with
the current frame's cost.

## Inspection and memory

```js
const scene = TownMesh.scene({ id: "1990", district: "residential" });
const viewer = Arsenal3D.mountScene(canvas, scene, {
  onDraw(plan) { caption.textContent = `${plan.triangles} submitted triangles`; }
});
viewer.focus("lot-0"); // move the actual camera to the full-detail building
viewer.reset();
// When replacing the panel:
viewer.dispose();
```

The gallery supplies a labeled building selector and reset button. Dragging or
arrow keys rotates the camera; the wheel and +/− zoom; Escape resets. There is
no automatic scene animation, including when reduced motion is requested.
Resize observers refresh the actual camera and draw list. Context restoration
rebuilds buffers and restores the selected view; disposal removes input handlers
and the resize observer. WebGL unavailability has an explicit fallback.

Lot meshes share the existing page-wide WebGL context and GPU LRU. Variant,
rotation, and finish form the cache key; lot placement uses a world offset, so
identical buildings can share buffers. The 1.2-million-triangle cache evicts
before uploading, keeping even transient base-buffer payload within its cap.
The deterministic CPU variant cache is separately bounded at 400,000 triangles.
These are cache payload bounds, not measurements of total JavaScript heap or
driver VRAM.

## Validation recorded on 2026-09-22

Evidence is retained under
`docs/campaign-certification/verification/evidence/2026-09-22-completion/`:

- `town-scene-browser.json`: 360 real Chrome/WebGL2 draw cases spanning three
  blocks, four CSS viewports, DPR 1 and 2, five yaw angles, and three zoom levels;
  104 full-detail building selections; and two additional 2,048×1,440 focused
  views. Actual draw-call totals equal the plan exactly. The highest of these
  measured submissions is **149,934 triangles**.
- The same browser run verifies wheel, pointer and keyboard controls, reset,
  actual context loss/restoration at each DPR, buffer reuse, disposal, no-WebGL
  fallback, and repeated city browsing sufficient to evict cached geometry.
  Peak queried buffer payload stays below **129,600,000 bytes** (the existing
  1.2M-triangle cap × 108 bytes). Shader/GL checks reported no errors.
- `town-raw-preservation.json`: all 135 original merged blocks from eight numeric
  seeds plus string `"1990"`, five districts and three tiers match the prior
  source byte-for-byte for positions, normals, colors, bounds, parts, lot records,
  and variant records.
- `check_town_scene.cjs` proves separately drawn close lot geometry matches its
  original merged geometry, all tiers share layout and culling bounds, every
  frustum-visible lot survives the budget governor, every selection remains
  close, hysteresis works, and cache eviction preserves deterministic output.
- `check_town_scene_budget.cjs` independently sums actual mesh vertex counts for
  the benchmark's draw plans, checks default camera/selection coverage, and
  proves an oversized background cannot silently pass the scene gate.

The P0 benchmark additionally sweeps the renderer's planner over all five
districts, eight numeric seeds and the actual string seed, four device-pixel
viewports, five yaw angles, four pitches, four zooms, and every selectable lot.
Its maximums and raw-storage comparisons are recorded in `P0_BUDGETS.md`.

Browser evidence includes exact checkout hashes and canonical LF source hashes.
Its recorded Git revision is the base revision with an explicitly dirty working
tree; the hashes identify the tested implementation before it is committed.
Raw-preservation evidence identifies the previous committed baseline separately.

The residential comparison uses the same camera and complete unchanged layout:
[original full-close overview](../campaign-certification/verification/evidence/2026-09-22-completion/town-residential-raw-close.png),
[adaptive overview](../campaign-certification/verification/evidence/2026-09-22-completion/town-residential-adaptive.png),
and [selected building at full detail](../campaign-certification/verification/evidence/2026-09-22-completion/town-residential-focus.png).

This validates deterministic geometry, real submissions, interaction and buffer
accounting. It does **not** certify 60fps, upload latency, fill rate, shader cost,
occlusion efficiency, total process memory, or campaign-map performance. Lower
tiers intentionally omit subpixel facade/roof detail; selecting a building brings
its full unchanged close mesh back into view.

To repeat:

```text
node --test tools/ui/check_town_scene.cjs tools/ui/check_town_scene_budget.cjs
node --test tools/ui/check_art_gallery.cjs tools/ui/check_arsenal3d_cache.cjs
node tools/ui/town-scene-browser.cjs
node tools/ui/bench_art.cjs --check
```

The browser runner uses installed Playwright and Chrome, a temporary loopback
static server, and a fresh browser context. It does not read or modify campaign
saves. `SPHERES_ART_OUTPUT` selects the parent directory for its retained report
and compact comparison images.
