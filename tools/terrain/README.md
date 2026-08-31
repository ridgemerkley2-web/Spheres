# tools/terrain — one-shot terrain data generators

The mapgen pattern, verbatim: **run only when the data needs regenerating.**
The outputs are committed, so the game itself never needs these tools or the
Natural Earth source data. Everything here is transcription — terrain classes,
feature names, rivers, lakes and river-crossed borders all come from the
staged Natural Earth artifacts in `spheres-web/data/`; nothing is invented.

## Invocation order (from the repo root)

```
python tools/terrain/make_rivers.py
python tools/terrain/crossing_edges.py
python tools/terrain/classify_districts.py
python tools/terrain/make_underlay.py
python tools/terrain/make_relief.py
python tools/terrain/make_coast.py
python tools/terrain/make_cover.py
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
and the four baked textures (see *The baked textures* below).

`make_coast.py` reads `spheres-web/ui/world.js`, which mapgen writes — so a
regeneration that changes the country outlines must re-run mapgen FIRST and
then `make_coast.py` and `make_cover.py` again. That dependency is the point of
the design rather than an inconvenience: the coastline the GL layer samples is
the same curve the SVG strokes, by construction, so the two cannot drift.

## Inputs (all committed under `spheres-web/data/`)

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
| `spheres-web/ui/relief.png` | 2400×1018 RGB8 heightmap: R,G packed uint16 elevation + B sqrt ocean depth (make_relief.py) |
| `spheres-web/ui/coast.png` | 2400×1018 L8 signed distance to the coastline, sqrt-companded (make_coast.py) |
| `spheres-web/ui/cover.png` | 1200×509 L8 vegetation index (make_cover.py) |

## The baked textures

Four PNGs ship inside the server binary (`include_bytes!` in
`spheres-web/src/main.rs`, one route each, `Cache-Control: public, max-age=86400`).
The three new ones are read by the GL underlay as **numbers, not pictures**:

| file | dims | encoding | bytes |
|---|---|---|---|
| `relief.png` | 2400×1018 | RGB8 — `h = (R*256+G) * (7900/65535) - 1500`, `depth = 11000*(B/255)²` | 2,753,776 |
| `coast.png` | 2400×1018 | L8 — `s = t*2-1`, `d = sign(s)*8*s*s` canvas units, land positive | 292,852 |
| `cover.png` | 1200×509 | L8 — `V` directly, 0..1 | 260,856 |
| **added** | | | **3,307,484 = 3.15 MiB** |
| `terrain.png` (kept) | 2400×1018 | LA8 hillshade, the fallback | 613,675 |
| **baked total** | | | **3,921,159 = 3.74 MiB** |

Three consequences worth knowing before touching any of them:

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

mapgen then merges `district_terrain.json` + `crossing_edges.json` into each
district record of `spheres-sim/data/districts.json` as `"t"`, `"f"` (omitted
when null) and `"riv"` (sorted river-crossed neighbour subset, omitted when
empty). The regeneration must diff purely additively — `id`/`name`/
`area_sqkm`/`adj` never move.

## Determinism

No RNG anywhere, no wall clock, fixed float64 math, sorted iteration and
sorted output, fixed PNG compression. Running any generator twice yields
byte-identical files; that is checked on every regeneration. `check.py` now
proves it mechanically for the three GL textures: it re-runs each generator
into a temp path (never over the committed artifact) and compares sha256.

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
