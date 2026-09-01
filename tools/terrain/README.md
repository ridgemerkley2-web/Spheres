# tools/terrain — one-shot terrain data generators

The mapgen pattern, verbatim: **run only when the data needs regenerating.**
The outputs are committed, so the game itself never needs these tools or the
Natural Earth source data. Almost everything here is transcription — terrain
classes, feature names, rivers, lakes and river-crossed borders all come from
the staged Natural Earth artifacts in `spheres-web/data/`; nothing is invented.

The one exception is named as such: `make_occlusion.py` **derives** a sky view
factor from the baked heightmap. It is not a measurement of the world, it is a
lighting term computed from one, at a vertical exaggeration (`Z = 4.20`) chosen
for amplitude rather than for geometry. It carries that disclosure at its own
definition, and so does this file. Everything else in `tools/terrain/` says
something the source data said first.

## Invocation order (from the repo root)

```
python tools/terrain/make_rivers.py
python tools/terrain/crossing_edges.py
python tools/terrain/classify_districts.py
python tools/terrain/make_underlay.py
python tools/terrain/make_relief.py
python tools/terrain/make_coast.py
python tools/terrain/make_cover.py
python tools/terrain/make_occlusion.py     # rewrites relief.png IN PLACE — must follow both
python tools/terrain/make_lakes.py
cargo run -p spheres-web --bin mapgen --features mapgen -- \
    spheres-web/data/ne_10m_admin_0.geojson spheres-web/data/ne_10m_admin_1.geojson
python tools/terrain/check.py
```

`crossing_edges.py` reads `make_rivers.py`'s segment output; `make_cover.py`
reads `make_coast.py`'s output, so those two are ordered; mapgen merges the
two python data outputs into `spheres-sim/data/districts.json` (mapgen stays
the sole writer of that file); `check.py` runs LAST and verifies every
committed artifact against ground truth — id coverage 2610/2610, famous
geography (Himalaya, Alps, Sahara, RU-YAN tundra), river bounds, the pinned
class histogram, crossing pairs ⊂ adjacency, the mapgen merge being verbatim,
and the five baked textures (see *The baked textures* below).

`make_coast.py` reads `spheres-web/ui/world.js`, which mapgen writes — so a
regeneration that changes the country outlines must re-run mapgen FIRST and
then `make_coast.py` and `make_cover.py` again. That dependency is the point of
the design rather than an inconvenience: the coastline the GL layer samples is
the same curve the SVG strokes, by construction, so the two cannot drift.

`make_lakes.py` is the same argument applied to the second water body: it reads
`ui/rivers.js`'s 29 already-projected lake rings — never `ne_10m_lakes.geojson`
— so the GL water edge and the `.lake` SVG stroke are the same curve at every
zoom. Re-running `make_rivers.py` therefore obliges a re-run of `make_lakes.py`.

**`make_occlusion.py` REWRITES `relief.png` IN PLACE**, and that is the one
ordering constraint here that bites silently. It reads the two *shipped* PNGs
— `relief.png` for elevation and `coast.png` for the land test — never the
source rasters, which is what makes it re-runnable on a checkout that has no
`etopo_60s.nc`. But `make_relief.py` owns `relief.png` and writes all three
planes, so **any re-bake of the heightmap wipes the occlusion channel and
`make_occlusion.py` must be re-run behind it.** Running it twice is harmless:
its output is a pure function of `(R, G, B-on-water, coast.png)`, none of which
it writes, so it reproduces its own output byte for byte, and it says so on the
second run instead of asserting a fresh-file count it cannot see.

## Inputs (all committed, under `spheres-web/data/` or `spheres-web/ui/`)

Four stages read a *committed artifact* rather than a source raster —
`make_coast` reads `world.js`, `make_lakes` reads `rivers.js`, and
`make_cover` and `make_occlusion` read `coast.png` — and that is deliberate in
every case: it is what makes the layers agree with the map by construction, and
what lets them re-run on a clone that has none of the staging data.

| file | used by |
|---|---|
| `ne_10m_admin_1.geojson` | classify_districts, crossing_edges (district geometry, mapgen-exact ids) |
| `ne_10m_geography_regions_polys.geojson` | classify_districts (physiographic label polygons) |
| `ne_10m_rivers_lake_centerlines.geojson` | make_rivers (scalerank ≤ 5 majors) |
| `ne_10m_lakes.geojson` | make_rivers (scalerank ≤ 1 majors), check |
| `SR_50M.zip` | make_underlay (v2.0.0 shaded relief; extracted to `tools/terrain/raster/`, a scratch cache) |
| `spheres-web/ui/index.html` | classify_districts (the `TERRITORY` roster, parsed with mapgen's own algorithm) |
| `spheres-sim/data/districts.json` | classify_districts, crossing_edges (authoritative ids + adjacency) |
| `spheres-web/ui/world.js` | make_coast (country outlines, already projected — the coastline source) |
| `spheres-web/ui/rivers.js` | make_lakes (29 lake rings, 982 vertices, already projected — the lake shoreline source) |
| `spheres-web/ui/relief.png` | make_occlusion (elevation from R,G; the water B plane is passed through untouched) |
| `spheres-web/ui/coast.png` | make_cover, make_occlusion (the land test, `c >= 128`, which is bit-exactly the shader's own `sd > 0.0`) |

Two source rasters are **staging data, not committed inputs**: they are large,
public-domain, and re-downloadable, so `spheres-web/data/` holds them untracked
and only the baked outputs ship.

| file (untracked) | bytes | used by |
|---|---|---|
| `etopo_60s.nc` | 478,290,125 | make_relief (NOAA NCEI ETOPO 2022, 60″ topography-bathymetry, EGM2008, netCDF-4/HDF5 — read with `h5py`, **not** netCDF4 or `scipy.io.netcdf_file`) |
| `NE1_50M_SR_W.zip` | 88,413,091 | make_cover (Natural Earth 1 natural-colour + shaded relief; extracts to a **subdirectory**, unlike `SR_50M.zip`) |

`check.py` skips the byte-identity check for a generator whose staging raster is
absent, with a warning, rather than failing — a fresh clone can still verify
everything that is committed, including `make_coast.py`, whose only input is
`world.js`.

## Outputs (committed)

| file | contents |
|---|---|
| `spheres-web/data/district_terrain.json` | `{districtId: {"t": class, "f": feature\|null}}` for all 2,610 districts |
| `spheres-web/data/river_segments.json` | filtered rivers as raw lon/lat polylines (crossing_edges input) |
| `spheres-web/data/crossing_edges.json` | river-crossed adjacency edges, `{"rule","eps_deg","count","edges"}` |
| `spheres-web/ui/rivers.js` | `window.RIVERS=` baked river/lake layer, same Robinson canvas as world.js |
| `spheres-web/ui/terrain.js` | `window.TERRAIN=` baked per-district class/feature layer (classify_districts.py, same ids as districts.js, null features omitted) |
| `spheres-web/ui/terrain.png` | 2400×1018 LA hillshade underlay, same canvas, baked by main.rs — now also the fallback the map drops to when WebGL2 is unavailable |
| `spheres-web/ui/relief.png` | 2400×1018 RGB8 heightmap: R,G packed uint16 elevation (make_relief.py) + B, which is sqrt ocean depth on water and baked sky occlusion on land (make_occlusion.py) |
| `spheres-web/ui/coast.png` | 2400×1018 L8 signed distance to the coastline, sqrt-companded (make_coast.py) |
| `spheres-web/ui/cover.png` | 1200×509 L8 vegetation index (make_cover.py) |
| `spheres-web/ui/lake.png` | 2400×1018 L8 signed distance to the 29 lake shorelines, same encode, clip and polarity as coast.png (make_lakes.py) |

## The baked textures

Five PNGs ship inside the server binary (`include_bytes!` in
`spheres-web/src/main.rs`, one route each, `Cache-Control: public, max-age=86400`).
The four new ones are read by the GL underlay as **numbers, not pictures**:

| file | dims | encoding | bytes |
|---|---|---|---|
| `relief.png` | 2400×1018 | RGB8 — `h = (R*256+G) * (7900/65535) - 1500`; B is **signed by the coast field**: `depth = 11000*(B/255)²` on water, `occ = B * 0.55/64` on land | 2,856,812 |
| `coast.png` | 2400×1018 | L8 — `s = t*2-1`, `d = sign(s)*8*s*s` canvas units, land positive | 292,852 |
| `cover.png` | 1200×509 | L8 — `V` directly, 0..1 | 260,856 |
| `lake.png` | 2400×1018 | L8 — identical to coast.png's encode and clip; positive means "not this lake" | 20,173 |
| **added** | | | **3,430,693 = 3.27 MiB** |
| `terrain.png` (kept) | 2400×1018 | LA8 hillshade, the fallback | 613,675 |
| **baked total** | | | **4,044,368 = 3.86 MiB** |

The added payload moved from 3,307,484 B to 3,430,693 B, **+123,209 B = +120.3 KiB**:
+103,036 B for the occlusion repack of `relief.png`'s B plane (the entropy the
occlusion field adds where the depth plane was flat) and +20,173 B for
`lake.png`. `check.py`'s payload cap is the sum of stated ceilings —
3,400,000 + the 140,000 B `make_occlusion.py` fails above + the 21,000 B
`make_lakes.py` fails above — rather than a round number moved to fit. Note the
repack delta cannot be predicted, only measured: it depends on the entropy of
the field actually shipped, so the generator prints it and fails on the ceiling.

Five consequences worth knowing before touching any of them:

- **No `gAMA`, `sRGB` or `iCCP` chunk, ever.** A decoder that gamma-corrected
  the R,G pair would destroy the packed uint16 outright and shift the
  coastline's zero crossing. Every generator scans its own output for those
  chunks and refuses to finish if one appears; `check.py` and
  `the_map_ships_terrain_and_rivers` in `main.rs` both check it again.
- **`H_EXT = 1018.1941195106424`, computed, never a literal.** Texel `(i,j)`
  centres sit at canvas `((i+0.5)*2400/WT, (j+0.5)*H_EXT/HT)`. This is *not*
  `make_underlay.py`'s `[0, 1018]` row convention (which `terrain.png` carries,
  and which the page then stretches over `WORLD.h = 1018.2` — a real 0.2-unit
  error at the bottom edge), and *not* `WORLD.h` itself. The renderer recovers
  latitude analytically from `Y_TOP` and the Robinson radius, so the rows and
  the shader's latitude agree only on the exact projection extent.
- **`ELEV_HI` is derived from the warped array, not assumed.** Peak amplitude is
  a function of bake width, not of Everest: 8157 m in the source, 7060.63 m at
  2×, 6230.66 m at 1×. `make_relief.py` picks the smallest ceiling on a fixed
  ladder that clears the measured peak, prints it, and asserts it — a `--2x`
  bake reusing the 1× ceiling would silently flatten every summit.
- **`relief.png`'s B byte means two different things, and the discriminator is
  the SIGN of `coast.png`, never a threshold on the byte.** Ocean depth on
  water, sky occlusion on land. `GLSL_DECODE` writes them into one RGBA16F
  channel with opposite signs — depth positive, occlusion negative — so a
  bilinear tap or a mip level straddling the shore degrades gracefully in both
  directions: the sea path's `clamp(dep/6500, 0, 1)` floors a leaked occlusion
  at exactly the 0 a land texel contributed before the channel existed, and the
  land path's `max(-g, 0)` floors a leaked depth the same way. A threshold on
  the byte would be destroyed by the first mip level. The land test is
  `coast >= 128`, which is bit-exactly the shader's own `sd > 0.0` (code 126
  decodes to −0.0118, 128 to +0.0039; 127 and 129 do not occur); the alternative
  `h >= 0` disagrees on 29,936 texels, 1.225% of the map. The repack overwrites
  4,472 land texels that carried sub-sea-level depth codes — the Dead Sea, the
  Caspian depression, the bed lakes, up to code 83. That is safe because the
  land path never reads depth, but the generator counts and prints it rather
  than asserting the plane was empty, because it was not.
- **`Z = 4.20` in the occlusion bake is an AMPLITUDE choice, not a geometric
  claim** — the same disclosure the procedural micro-relief comment carries. At
  `Z = 1.35` the same march yields p90 = 0.010 and is invisible. The bake is
  fixed at the far-out exaggeration so that occlusion is the one depth term that
  does *not* fall off as the camera dives, and it is why nothing downstream may
  raise `Z` to compensate for close zoom without double-counting.

mapgen then merges `district_terrain.json` + `crossing_edges.json` into each
district record of `spheres-sim/data/districts.json` as `"t"`, `"f"` (omitted
when null) and `"riv"` (sorted river-crossed neighbour subset, omitted when
empty). The regeneration must diff purely additively — `id`/`name`/
`area_sqkm`/`adj` never move.

## Wall clock

Offline cost of the two newest stages, measured on this machine (float64,
single process, no GPU):

| stage | wall clock | dominated by |
|---|---|---|
| `make_occlusion.py` | **6.8 s** | 4.0 s of horizon march — 16 ground-uniform azimuths × 12 log distances 8–220 km as 192 whole-raster shifted-max ops in 32 latitude bands, then the `compress_level=9` re-encode of a 2.86 MB PNG |
| `make_lakes.py` | **3.4 s** | 2.7 s of exact Euclidean distance transform at 9600×4072, run twice (mask and complement) |

The horizon march is the one with a trap in it: written as a per-pixel loop it
is 4.7×10⁸ scalar operations and takes about an hour. Vectorised as 192 whole-
raster shifted maxima it is four seconds. If you rewrite it, keep it vectorised.

## Determinism

No RNG anywhere, no wall clock in any data path, fixed float64 math, sorted
iteration and sorted output, fixed PNG compression. Running any generator twice
yields byte-identical files; that is checked on every regeneration. `check.py`
proves it mechanically for all four GL textures: it re-runs each generator into
a temp path (never over the committed artifact) and compares sha256.

`relief.png` is the two-stage one, so `check.py` runs the **pipeline** —
`make_relief.py` into the temp path, then `make_occlusion.py` pointed at that
same temp path — and hashes the result. Hashing `make_relief.py`'s output alone
against the committed file would fail by construction and would tempt someone to
delete the check rather than chain it.

Current hashes of the two artifacts this pass writes:

```
relief.png  fb4e7512c4b5527c3f99f30e08279a85aab13bfc027d023579ef760d60a95ed6
lake.png    32f8e00268ddc9ab721b6bcb380ae83862a37be61202c7464387f36586828879
```

## The 66°N latitude-band override

Natural Earth's tagged plain/plateau polygons outvote the classifier's ≥66°N
tundra default on Arctic districts — Yamal-Nenets (RU-YAN) read as ordinary
lowland via the Western Siberian Plain polygon. After the class vote,
`classify_districts.py` therefore flips any district the majority of whose
land sits at or above 66°N to tundra, unless the landform vote said mountain
or desert (those are accurate whatever the latitude). The band test is pure
latitude geometry, so it is measured on the sampling ladder's densest rung
(n=112) rather than the class vote's adaptive budget — the adaptive rung
under-samples a district whose polar half is peninsulas. The override
currently flips exactly RU-YAN (lowland→tundra) and RU-KYA
(highland→tundra), both printed on every run.

## probes/

Provenance only: the one-shot probes that established the taxonomy, the
hydro filters and the coverage estimates during staging. They are not part
of the regeneration path and still reference the staging directory they ran
from; kept as the record of how the documented choices were made.
