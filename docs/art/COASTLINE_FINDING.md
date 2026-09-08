# The coastline: what it costs, what it needs, and why it did not ship

Written 2026-09-07, after Ridge approved a download to fix the coastline. The
download happened, the lakes were fixed with it, and the coastline was taken as
far as it can go without one more source file. This records exactly where it
stopped and what would restart it.

## What is wrong with the coastline today

| | resolution |
| --- | --- |
| `ui/coast.png`, the ground's land/water field | 16,698 m per texel |
| `ui/world.js`, the outline drawn over it | 5,844 m simplification tolerance, median segment **24,313 m** |
| `ui/terrain-tiles/`, the elevation underneath | **1,855 m** per sample |

The camera now reaches about 9 m per pixel. A 24 km segment is 2,700 screen
pixels of dead-straight shore. On top of that the land test is pushed 2.0 km out
to sea by a constant, and the shoreline the map draws sits a **median 3.02 km**
from the true one (p90 6.80 km, 6.47% of area on the wrong side), measured
against Natural Earth 10m admin-0 rasterised at 334 m over ten coastal regions.

## What was done

`ne_10m_coastline`, `ne_10m_lakes`, `ne_10m_admin_0_countries` and
`ne_10m_admin_1_states_provinces` were fetched into `spheres-web/data` (25 MB of
shapefiles, untracked, as `.gitignore` requires). `tools/terrain/shapefile.py`
reads them without a dependency; `tools/terrain/shp_to_geojson.py` converts for
the tools that want GeoJSON.

**The lakes shipped.** See the commit "Lake shores at 200 m instead of 6.7 km".

**The source is compatible with the simulation, and this was measured rather
than assumed.** Running `mapgen` against the fresh download reproduces the
district roster exactly: the same 2,610 ids, **zero** districts with changed
adjacency, **zero** with a changed river-crossed set, **zero** with a changed
terrain class, and 44 of 2,610 with an area moved by a median 0.0003%. The
newer Natural Earth export is not a different world.

**The finer outline was built and measured.** At `EPS_WORLD` 0.05 (835 m)
`world.js` goes 513 KB to 2,156 KB, its segment count 40,285 to 179,908, and its
median segment **24,313 m to 6,021 m**. `coast.png` re-bakes from it cleanly --
`make_coast.py` reads `world.js` by design, so the two register by construction
-- and the registration measurably improves: RMS 0.303 to **0.187** canvas
units, with the unclassifiable tail falling from 20 vertices to 4.

## Why it did not ship

`relief.png`'s blue plane carries baked sky occlusion **on exactly the set of
texels `coast.png` calls land**. A finer coastline encloses 431 more of them
(654,935 to 655,366, +0.066%), so that plane has to be rewritten with it.

`make_occlusion.py` can do that, and only from a **freshly baked** `relief.png`:
it refuses to run on its own output, correctly, because the operation is not
idempotent. Baking a fresh `relief.png` needs `etopo_60s.nc` -- **478 MB**,
deliberately not committed, and not on disk.

So the chain is: finer `world.js` -> re-bake `coast.png` -> re-bake
`relief.png` -> re-bake occlusion. The third link needs a source this repo does
not carry, and the change is not shippable in pieces: shipping the finer outline
without the matching land set leaves the occlusion plane written on the wrong
texels, and shipping it without re-baking `coast.png` at all leaves the drawn
coast and the ground's water edge disagreeing by kilometres. That was tried on
screen: national borders stand out over open water, which reads as a bug because
it is one.

**To finish it, fetch `etopo_60s.nc` (478 MB) and run, in order:**
`mapgen` with `EPS_WORLD` lowered, `make_coast.py`, `make_relief.py`,
`make_occlusion.py`, then `check.py`. Everything else is already in place and
measured. The cost is 1.6 MB of extra committed `world.js` and one large
download that need not be kept.

## A defect found on the way, and fixed

`make_coast.py`'s own registration check -- the one that catches a half-texel
row offset -- sampled `coast_v[::200]`, about 200 of 29,000 vertices, and
asserted a maximum over them. **It was passing on which vertices the stride
happened to land on.** Checking every vertex on the committed, unmodified bake
gives max |d| = 7.92 against a bar of 2.0.

Two causes, both now addressed. The stride is gone: all 27,576 classified
coastline vertices are checked. And the classifier was wrong -- "a vertex
appearing in exactly one ring is coastline" calls every inland border vertex
whose two sides did not survive simplification alike a shoreline, and calls a
disputed boundary one too. It is now a segment-proximity test against other
countries' edges, which also catches the long straight political borders a
vertex-to-vertex test misses, plus explicit exclusions for rings smaller than a
texel and for the 83N/58S clip edge.

A tail of about 20 vertices survives and cannot be classified away: Egypt's
boundary at the Hala'ib triangle and Bir Tawil, where the neighbouring polygon
deliberately does not meet it. Those are counted and bounded rather than hidden.
`check.py`'s independent second opinion keeps the simpler classifier, so it
asserts the mean -- which is what detects a row offset and is robust to a
misclassified tail -- and reports the rest.
