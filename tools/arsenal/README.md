# Arsenal models

Forty-six low-poly meshes, one for every id in `spheres-sim/src/arsenal.rs`'s
`DECK`, plus the two things that keep them honest.

| file | what it is |
| --- | --- |
| `spheres-web/ui/arsenal-models.js` | the meshes. No DOM, no fetches, runs under node. |
| `spheres-web/ui/arsenal3d.js` | one WebGL2 context, drawing them into the equipment cards. |
| `spheres-web/ui/arsenal3d.css` | where a model sits on a card, and what hides when one appears. |
| `tools/arsenal/gallery.html` | the bench: the whole deck at a size a card never uses. |
| `tools/arsenal/export_obj.js` | the same meshes as Wavefront OBJ, for anything that is not this page. |

## Why the models are code

This page has **no build step and no CDN** (CLAUDE.md; `main.rs` asserts it).
That rules out a glTF loader and forty-six binary payloads, so the meshes are
built the way `mapgen.rs` builds the map: author the recipe, ship the recipe,
let the client bake it. The whole deck is about **20,800 triangles** and costs
the wire nothing — `arsenal-models.js` is source, and source gzips.

It also means the models are **diffable**. A tank here is forty lines of
readable code; a change to one shows up in review as a change to one.

## Conventions

- **Model space**: `+X` right, `+Y` up, `+Z` the direction the thing points —
  nose, bow, muzzle. Right-handed.
- **Units are metres**, and they are the real ones where the real thing exists:
  an F-15E is 19.4 m long beside an M1 that is 7.9 m, because a future map layer
  will place these against terrain and guessing now means re-measuring later.
  `MODELS[id].span` carries the largest real dimension for that use.
- **Flat shaded, no stored normals.** Faceting is the look, and normals are
  derived per triangle at `finish()` from the vertices *as transformed* — which
  is what makes mirroring safe (see `Mesh.both`).
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
