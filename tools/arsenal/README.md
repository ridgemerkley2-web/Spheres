# Arsenal models

Forty-six procedural meshes, one for every id in `spheres-sim/src/arsenal.rs`'s
`DECK`, plus the two things that keep them honest.

## Source and integration attribution

This static catalogue deck comes from the existing Claude-assisted Arsenal
work integrated in commit `092569227023ff4278a5d699018af46bd39c7c94`
(`feat(arsenal): a 3D model for every kit in the deck, drawn from one WebGL2 context`).
That commit records Claude Opus 5 as co-author and the source branch work as
`8ec69ee` on `feat/hoi4-map-and-tech`. The integration retains its 46 deterministic
catalogue meshes, shared WebGL2 renderer and OBJ export tools.

The 7 September integration combines `feat/hoi4-map-and-tech` through `fc0f0c2`
with `feat/art-p0` through `c2e49c6`. The art branch raises the deck to card-level
detail, adds a coarse level of detail and selective smooth shading, and improves
lighting and surface treatment. Its separate globe scatter experiment remains
withdrawn; see [the scale finding](../../docs/art/SCATTER_SCALE_FINDING.md).

These static catalogue previews are separate from the configurable ground and
tactical-aircraft designer and its GLB exports. The first aircraft design slice
is documented in [AVIATION.md](../../AVIATION.md); displaying other aircraft and
ships here does not add their mission or naval component-design systems.
This integration adds no new claim of independently verified historical
dimensions or engineering fidelity; the
existing names and dimension annotations are inherited from that source work.

| file | what it is |
| --- | --- |
| `spheres-web/ui/arsenal-models.js` | the meshes. No DOM, no fetches, runs under node. |
| `spheres-web/ui/arsenal3d.js` | one WebGL2 context, drawing them into the equipment cards. |
| `spheres-web/ui/arsenal3d.css` | where a model sits on a card, and what hides when one appears. |
| `tools/arsenal/gallery.html` | the bench: the whole deck at a size a card never uses. |
| `tools/arsenal/export_obj.js` | the same meshes as Wavefront OBJ, for anything that is not this page. |

## Why the models are code

This static catalogue renderer has **no build step and no CDN** (CLAUDE.md;
`main.rs` asserts it). It generates its meshes without a glTF loader or
forty-six binary payloads, the way `mapgen.rs` builds the map: author the recipe,
ship the recipe, let the client bake it. The current near-detail deck contains
**272,491 triangles** across 46 models; cards can use the coarse geometry.
The browser downloads the source generator rather than separate binary models.

It also means the models are **diffable**. A tank here is forty lines of
readable code; a change to one shows up in review as a change to one.

## Conventions

- **Model space**: `+X` right, `+Y` up, `+Z` the direction the thing points —
  nose, bow, muzzle. Right-handed.
- **Units are metres**, and they are the real ones where the real thing exists:
  an F-15E is 19.4 m long beside an M1 that is 7.9 m, because a future map layer
  will place these against terrain and guessing now means re-measuring later.
  `MODELS[id].span` carries the largest real dimension for that use.
- **Shading is selected per part.** Face normals are derived from transformed
  geometry; curved parts may use averaged vertex normals while faceted designs
  retain their silhouette. OBJ export preserves per-vertex normals.
- **No randomness anywhere.** Same id, same buffers, every run. This project's
  first iron rule is determinism and there is no reason for art to be the
  exception.

## Working on a model

```bash
# The bench. Opens straight from the filesystem — it fetches nothing.
start tools/arsenal/gallery.html          # Windows
```

Edit `arsenal-models.js`, reload the page. The bench loads the same two files
the game does, so a model that looks right there looks right on a card.

```bash
# Every mesh to OBJ (metres, +Z forward, vertex colours on the `v` lines).
node tools/arsenal/export_obj.js          # -> tools/arsenal/obj/, gitignored
node tools/arsenal/export_obj.js --id f22 # -> stdout
```

The OBJ output is **derived and not checked in**. Regenerating is a second. If
one of these is ever replaced by something hand-modelled, the exported OBJ is
the thing for the new mesh to match — same axes, same metres, same origin.

## Adding a kit

Add it to `DECK` in `spheres-sim/src/arsenal.rs`, then add a model under the
same id. `cargo test -p spheres-web every_kit_in_the_deck_has_a_model` fails
until you do, in both directions — a kit with no model, and a model whose kit
has been deleted. The model's `name` must match the deck's name exactly; that
is asserted too, because the name is what the player reads on the card.

Most models are a call into one of the families rather than new geometry:
`jet`, `armour`, `truck`, `ship`, `submarine`, `missile`, `satellite`,
`soldier`, `arrayFace`, `dish`. Six of the eleven ordnance entries are one
`missile` call with a different fin table. Reach for a family first; author new
primitives only when the shape is the point — the F-117, the B-2 and the RQ-170
are the three that earned it.

## What happens without WebGL2

Nothing breaks. `Arsenal3D.scan` removes the canvas it cannot draw into, the
class glyph from `MANU_CLASS_MARK` that was under it all along stays visible,
and the panel is what it was before these files existed. That path is asserted
in `the_equipment_models_are_self_contained_and_degrade_to_the_glyph`.
