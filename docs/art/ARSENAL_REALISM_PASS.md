# Arsenal structural realism pass

8 September 2026. This pass refines the larger physical forms of eight fighter recipes and three surface-ship catalogue entries. It builds on the mechanical-detail pass described in `ARSENAL_MESH_DETAIL.md`; that document records the preceding measurements.

The later independent `inf_mech` formation revision is recorded in `ARSENAL_INF_MECH_PASS.md`. The measurements below describe this air/naval pass at completion; its other 45 resulting near meshes and every far buffer are preserved by that subsequent revision.

## Ships

**Bow flare is part of the hull section.** From 66% of hull length forward, the underwater body and the topside waterline narrow progressively beneath the unchanged deck edge. The narrowed wet and dry sections share the same waterline. Midship beam, depth, stem/transom positions, deck outlines and all catalogue bounding boxes remain exact. Rubbing strakes follow the changed topside surface rather than floating outside it.

**Decks have a crown.** The former two planar slopes become a parabolic section through centre, quarter-beam and deck edge. The original centre and edge heights remain fixed, and the sheer still follows the authored longitudinal stations.

**Bridge glazing replaces wall skin.** Previously a separate window strip could sit behind the opaque superstructure wall. The detailed house shell now has separate lower plating, a framed glazing course, shoulder and roof. Glass is recessed behind a connected reveal, with no solid wall or end cap covering the opening. Pane counts follow the size of each wall; the carrier's escort houses retain a lighter version. Existing roof overhangs, mast locations and deckhouse tiers stay in place.

**Boarding boats are open.** The full-width lid is removed at near detail, revealing inner sides, a raised cockpit sole, two separate bench seats and a lowered console. Tapered circular collars follow the old gunwale envelope. The console carries a small angled windscreen, and the original outboard remains in position. Far boats retain the old economical closed form.

## Aircraft

**Cockpits seat on a sill.** The shared fighter canopy is now a glazed hood with a flat base instead of a complete ellipsoid intersecting the airframe. Frame bows follow the hood, side sills follow its width, and the forward windscreen has a separate straight rake. Hidden lower glass and the former headrest intersecting opaque glass are removed. The established upper and overall model envelopes are retained.

**Conventional fuselages have fair transitions.** A shape-preserving cubic interpolation passes through the authored frames and adds a midpoint only where meaningful curvature differs from the straight chord. It does not subdivide straight barrels or linear tapers, and cannot exceed adjacent section extrema. Caps remain outside smooth normal groups. Faceted stealth airframes retain their hard cross-sections; their canopy construction still benefits from the hood change.

The previous compressor, aft exhaust, control-surface, track and fastener improvements remain intact. Infantry, armour, missile and spacecraft meshes are byte-identical to their pre-pass near buffers. No renderer, material, export, gameplay, part metadata or server changes belong to this pass.

## Measured geometry

| Model | Before near | Current near | Change |
| --- | ---: | ---: | ---: |
| air_gen2 | 5,580 | 5,608 | +28 |
| air_gen3 | 6,868 | 6,896 | +28 |
| air_gen4 | 7,692 | 7,720 | +28 |
| f15e | 8,200 | 8,192 | −8 |
| f22 | 5,308 | 5,160 | −148 |
| ea18g | 7,404 | 7,396 | −8 |
| f35a | 4,842 | 4,694 | −148 |
| sixthgen | 4,816 | 4,668 | −148 |
| nav_patrol | 7,780 | 9,018 | +1,238 |
| nav_escort | 10,488 | 12,686 | +2,198 |
| nav_blue | 11,486 | 13,218 | +1,732 |

All other 35 near meshes are unchanged. Aircraft use 376 fewer triangles overall; ships add 5,168. The complete near catalogue is **343,519 triangles / 35.38 MiB** of position, normal and colour arrays. The unchanged far catalogue is **39,508 triangles / 4.07 MiB**, leaving both complete buffer sets at **39.45 MiB**. The largest card remains mechanised infantry at **15,760**, within the existing 16,000 ceiling. These measurements cover CPU attribute arrays, not transient allocations, GPU copies or renderer framebuffers.

## Validation

```text
node tools/ui/check_arsenal_models.cjs
node tools/ui/check_arsenal_model_detail.cjs
node tools/ui/check_arsenal_realism.cjs
```

The first two suites retain all existing face-area, winding, normal, OBJ, determinism, exact-bound, exact-ID/span, far-buffer hash and memory-budget contracts. The new structural tests additionally verify:

- Only the eleven intended near models change.
- Bow waterlines narrow below fixed deck edges while midship dimensions remain fixed.
- Quarter-beam crown height differs from the former planar roof.
- Rays through side and front bridge windows reach recessed glass first; wall skin cannot obscure it.
- Rays through a boarding boat reach the cockpit sole, with benches and rounded collars above it.
- Fuselage interpolation preserves authored frames, local extrema, straight tapers and input data.
- Canopy glass stays above the sill and the forward screen remains a distinct panel run.

Private geometry probes are injected only into the test VM; the production API is unchanged. This is a procedural visual reconstruction, not a certified engineering model. Browser appearance is reviewed separately with the active renderer; this document does not claim measured frame-rate gains.
