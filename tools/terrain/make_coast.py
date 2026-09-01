#!/usr/bin/env python
# make_coast.py -- bake a signed distance field to the game's OWN coastline.
#
# Input (anchored on the repo root this file sits in):
#   spheres-web/ui/world.js
#     `window.WORLD={w,h,countries:{ISO3: "M x y L x y ... Z M ... Z"},centroids,graticule}`,
#     written by `cargo run -p spheres-web --bin mapgen --features mapgen` from Natural Earth
#     10m admin-0. 257 countries, 866 subpath rings, 41,151 vertices, absolute M/L/Z only,
#     ALREADY PROJECTED into the Robinson game canvas -- so this generator needs no
#     re-projection at all and cannot disagree with mapgen by construction.
#
# Output:
#   spheres-web/ui/coast.png   2400 x 1018 L8 -- the committed coastline field, baked into
#                              the server binary by main.rs and served from /coast.png.
#
# WHY world.js AND NOT THE ELEVATION SIGN, AND NOT THE GEOJSON:
#   1. Registration by construction. The renderer's land/water boundary becomes the SAME
#      curve the SVG strokes as country outlines, Natural Earth's generalisation included.
#      Two independently derived coastlines cannot be made to agree at k=32; one curve
#      trivially agrees with itself.
#   2. `sign(elevation)` is simply wrong. NL-ZH (Zuid-Holland) has a mean of -1.2 m with
#      72.8% of the district below sea level; a sign rule paints the Randstad blue.
#   3. No shapely, no geojson re-projection, no second projection implementation.
#
# INLAND WATER, measured rather than assumed: NE 10m admin-0 EXCLUDES the Caspian (this bake
# reads d = -7.75 in the middle of it) but INCLUDES the Great Lakes, Baikal, Victoria, Great
# Bear and the Aral inside the country polygons. That is not a defect to correct here. The
# SVG country fill covers those lakes today, so a field that cut blue holes in them would
# put the GL underlay in disagreement with the district fills drawn over it -- the one thing
# this design cannot afford. They are painted as water by ui/rivers.js's 29 lake paths on
# top (Superior at x 656-703, Michigan/Huron at 695-724, Erie at 719-746, Baikal at
# 1793-1815, Victoria at 1411-1432), exactly as they are today. Registration beats
# hydrography: the value of deriving from world.js is that there is only ever one curve.
#
# Rings are accumulated by the EVEN-ODD rule, which is what `renderMap` uses: the scenery and
# per-nation paths in ui/index.html both carry fill-rule="evenodd" (index.html:4873, :4888).
# Holes (enclaves, inland seas) therefore fall out of the ring nesting with no winding logic.
#
# Encoding -- signed distance in CANVAS UNITS, clipped +/-8, sqrt-companded, land positive:
#   code   = round(128 + 127 * sign(d) * sqrt(min(|d|, 8) / 8))
#   decode = s = code/255*2 - 1 ;  d = sign(s) * 8 * s * s
# The companding is ODD-SYMMETRIC about the coastline, so the zero crossing survives bilinear
# filtering and mip reduction EXACTLY -- the midpoint of two texels straddling the shore still
# decodes to the shore. That is what keeps the coastline sub-pixel crisp at k=32 out of a
# 2400-wide bake, and it is why this channel is worth its 174 KB where a raw mask would not be.
#
# Method: the rings are rasterised by exact even-odd scanline crossing at 4x supersample
# (9600 x 4072) -- analytic in x, point-sampled in y -- then scipy's exact Euclidean distance
# transform is run on that 4x mask and its complement with per-axis `sampling` in canvas
# units, and the SIGNED distance is 4x4 box-downsampled to the shipped 2400 x 1018 grid.
#
# Running the transform at 4x and averaging the FIELD costs 128 KB over thresholding the 4x
# coverage to a 1x mask and transforming there -- measured, both ways, same rasteriser:
#     4x transform, field downsampled   292,852 B   253 distinct codes   (shipped)
#     1x mask, transform at 1x          165,143 B    62 distinct codes
# The 1x transform can only reproduce the staircase of a 1x mask, so its zero crossing is
# quantised to half a canvas unit; at k=32 that is a 16-device-pixel jag ruled along the one
# curve this whole file exists to keep crisp, and the 62 codes are the tell -- it is
# reproducing sqrt(integer) distances, not a coastline. Box-averaging a locally linear signed
# distance is exact, so the extra accuracy costs only the entropy it adds. 128 KB for the
# stated purpose of the channel is the right side of that trade.
#
# THE CAP MOVED, 2026-08-31, and this line used to name the old one. It read
# "3,400,000-byte cap on the three baked textures" back when there were three. The depth
# pass added a fourth (lake.png) and repacked relief.png's B plane for sky occlusion, so
# the budget is now the SUM OF EACH GENERATOR'S OWN STATED FAIL THRESHOLD rather than a
# round number moved to fit an overrun: 3,400,000 + 140,000 (occlusion repack) + 21,000
# (lake) = 3,561,000. check.py:527-533 is where that sum is enforced. The shipped set is
# 3,430,693 B, leaving 130,307 B of headroom, all of it budgeted against a named ceiling.
#
# ROW EXTENT: texel (i, j) centre is canvas ((i+0.5)*2400/WT, (j+0.5)*H_EXT/HT) with
#   H_EXT = Y_TOP - robinson_y(LAT_BOT) = 1018.1941195106424
# the exact projection extent -- NOT make_underlay.py's [0, 1018] rows and NOT WORLD.h's
# 1-dp 1018.2. The renderer recovers latitude analytically, so every baked layer must sit on
# this one row grid or they shear against each other at the bottom of the map.
#
# Deterministic: no RNG, no wall clock, fixed float64 math, countries iterated in sorted ISO3
# order, fixed PNG compression (compress_level=9, optimize=True). Running twice yields a
# byte-identical PNG, and the written file is scanned for gAMA/sRGB/iCCP and rejected if any
# is present (the renderer samples this field numerically; a gamma-correcting decoder would
# move the coastline).
#
# Invocation:  python tools/terrain/make_coast.py
#   (prints H_EXT, the inverse round-trip error, the ring/vertex counts, the land fraction,
#    the output byte count, chunk hygiene, the 3-landmark alignment proof over Gibraltar /
#    Cape Horn / Tokyo Bay, a sign spot-check over land, ocean, the Great Lakes, the Caspian
#    and Zuid-Holland, and the zero-level-set registration measured against world.js's own
#    ring vertices -- which is the check that would actually catch a row-grid mistake.)
#
# On the Cape Horn window specifically: it reads open water for 5x5, and that is correct
# rather than a miss. mapgen's generalisation of NE 10m puts the southernmost Tierra del
# Fuego vertex at canvas y = 1001.9, some 2.4 units north of the projected Cape (828.7,
# 1004.0); the island is below the source's resolution at this scale. The page draws no land
# there either, so the field agrees with the polygons -- which is the entire proposition of
# deriving it from world.js rather than from elevation.

import json
import math
import os

import numpy as np
from PIL import Image
from scipy import ndimage

Image.MAX_IMAGE_PIXELS = None

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
WORLD_JS = os.path.join(ROOT, "spheres-web/ui/world.js")
OUT = os.path.join(ROOT, "spheres-web/ui/coast.png")

# ---- mapgen.rs constants, replicated exactly (lines 19-81 of mapgen.rs) ----
W = 2400.0
LAT_TOP = 83.0
LAT_BOT = -58.0
RX = [1.0000, 0.9986, 0.9954, 0.9900, 0.9822, 0.9730, 0.9600, 0.9427, 0.9216, 0.8962,
      0.8679, 0.8350, 0.7986, 0.7597, 0.7186, 0.6732, 0.6213, 0.5722, 0.5322]
RY = [0.0000, 0.0620, 0.1240, 0.1860, 0.2480, 0.3100, 0.3720, 0.4340, 0.4958, 0.5571,
      0.6176, 0.6769, 0.7346, 0.7903, 0.8435, 0.8936, 0.9394, 0.9761, 1.0000]

SDF_CLIP = 8.0          # canvas units; beyond this the field is saturated and unused
SS = 4                  # supersample factor for the rasterise + transform


def radius():
    return W / (2.0 * 0.8487 * math.pi)


def interp(table, lat_abs):
    t = min(lat_abs / 5.0, 18.0)
    i = int(t)
    if i >= 18:
        return table[18]
    return table[i] + (t - i) * (table[i + 1] - table[i])


def robinson_y(lat):
    return 1.3523 * radius() * interp(RY, abs(lat)) * math.copysign(1.0, lat) if lat != 0.0 \
        else 0.0


def project(lon, lat):
    lat = max(LAT_BOT, min(LAT_TOP, lat))
    x = W / 2.0 + 0.8487 * radius() * interp(RX, abs(lat)) * math.radians(lon)
    y = robinson_y(LAT_TOP) - robinson_y(lat)
    return x, y


R = radius()
Y_TOP = robinson_y(LAT_TOP)
H_EXT = Y_TOP - robinson_y(LAT_BOT)     # 1018.1941195106424 -- computed, never a literal

RY_ARR = np.asarray(RY, dtype=np.float64)
RY_DIFF = np.diff(RY_ARR)


def lat_from_canvas_y(y):
    """Vectorized exact inverse of robinson_y. Used here only to prove that this bake's row
    grid is the same one make_relief.py inverts -- the raster itself needs no inverse."""
    g = (Y_TOP - np.asarray(y, dtype=np.float64)) / (1.3523 * R)
    sign = np.where(g < 0.0, -1.0, 1.0)
    gg = np.abs(g)
    i = np.clip(np.searchsorted(RY_ARR, gg, side="right") - 1, 0, 17)
    t = i + (gg - RY_ARR[i]) / RY_DIFF[i]
    return sign * 5.0 * t


def png_chunk_hygiene(path):
    blob = open(path, "rb").read()
    found = [c for c in (b"gAMA", b"sRGB", b"iCCP") if c in blob]
    assert not found, f"{os.path.basename(path)} carries colour chunks {found}"
    return "none (gAMA/sRGB/iCCP all absent)"


def read_countries():
    """WORLD.countries as {ISO3: pathdata}. Brace-balanced extraction: the generated file
    holds no braces inside its path strings, so a counter is exact here."""
    src = open(WORLD_JS, encoding="utf-8").read()
    i = src.index("countries:") + len("countries:")
    assert src[i] == "{", src[i:i + 20]
    depth, j = 0, i
    while True:
        if src[j] == "{":
            depth += 1
        elif src[j] == "}":
            depth -= 1
            if depth == 0:
                break
        j += 1
    return json.loads(src[i:j + 1])


def rings_of(pathdata):
    """Absolute M/L/Z path data -> list of closed rings, each an (n, 2) float64 array."""
    out = []
    for chunk in pathdata.split("M"):
        chunk = chunk.strip()
        if not chunk:
            continue
        assert chunk.endswith("Z"), chunk[-30:]
        pts = []
        for pair in chunk[:-1].split("L"):
            xs, ys = pair.split()
            pts.append((float(xs), float(ys)))
        if len(pts) >= 3:
            out.append(np.asarray(pts, dtype=np.float64))
    return out


def main():
    countries = read_countries()

    # ---- edge table, even-odd: every ring contributes its closing edge too ----
    ex0, ey0, ex1, ey1 = [], [], [], []
    n_rings = 0
    for code in sorted(countries):                     # sorted iteration, determinism
        for ring in rings_of(countries[code]):
            n_rings += 1
            a = ring
            b = np.roll(ring, -1, axis=0)              # closes last -> first
            ex0.append(a[:, 0]); ey0.append(a[:, 1])
            ex1.append(b[:, 0]); ey1.append(b[:, 1])
    ex0 = np.concatenate(ex0); ey0 = np.concatenate(ey0)
    ex1 = np.concatenate(ex1); ey1 = np.concatenate(ey1)
    n_edges = ex0.size

    # orient every edge downward so the half-open span test [ymin, ymax) is uniform
    swap = ey0 > ey1
    ax = np.where(swap, ex1, ex0); ay = np.where(swap, ey1, ey0)
    bx = np.where(swap, ex0, ex1); by = np.where(swap, ey0, ey1)
    dydx = np.divide(bx - ax, by - ay, out=np.zeros_like(ax), where=(by != ay))

    W1, H1 = 2400, 1018
    W4, H4 = SS * W1, SS * H1
    xc = (np.arange(W4, dtype=np.float64) + 0.5) * (W / W4)
    yc = (np.arange(H4, dtype=np.float64) + 0.5) * (H_EXT / H4)

    # projection sanity: this row grid must invert to the same latitudes make_relief.py uses
    lats = lat_from_canvas_y(yc)
    rt_err = float(np.max(np.abs(
        np.array([Y_TOP - robinson_y(la) for la in lats], dtype=np.float64) - yc)))

    # ---- exact even-odd scanline rasterisation at 4x ----
    mask = np.zeros((H4, W4), dtype=bool)
    order = np.argsort(ay, kind="stable")
    ay_s, by_s, ax_s, dydx_s = ay[order], by[order], ax[order], dydx[order]
    lo_all = np.searchsorted(ay_s, yc, side="right")     # edges whose ymin <= y
    for j in range(H4):
        y = yc[j]
        lo = lo_all[j]
        if lo == 0:
            continue
        sel = by_s[:lo] > y
        if not sel.any():
            continue
        xs = ax_s[:lo][sel] + (y - ay_s[:lo][sel]) * dydx_s[:lo][sel]
        xs.sort()
        mask[j] = (np.searchsorted(xs, xc, side="right") & 1).astype(bool)

    land_frac = float(mask.mean())

    # ---- exact Euclidean signed distance at 4x, in canvas units ----
    sampling = (H_EXT / H4, W / W4)
    half = 0.5 * (W / W4)          # centre the zero crossing between two texel centres
    dt = ndimage.distance_transform_edt(mask, sampling=sampling)
    sdf4 = np.where(mask, dt - half, 0.0)
    del dt
    dt = ndimage.distance_transform_edt(~mask, sampling=sampling)
    sdf4 -= np.where(mask, 0.0, dt - half)
    del dt

    sdf = sdf4.reshape(H1, SS, W1, SS).mean(axis=(1, 3))
    del sdf4

    # ---- sqrt companding, odd-symmetric about the shore ----
    d = np.clip(sdf, -SDF_CLIP, SDF_CLIP)
    q = np.sign(d) * np.sqrt(np.abs(d) / SDF_CLIP)
    code = np.clip(np.rint(128.0 + 127.0 * q), 0, 255).astype(np.uint8)

    Image.fromarray(code).save(OUT, optimize=True, compress_level=9)   # 2D uint8 -> mode "L"
    size = os.path.getsize(OUT)
    hygiene = png_chunk_hygiene(OUT)

    # decode exactly as the renderer will, for the proofs below
    s = code.astype(np.float64) / 255.0 * 2.0 - 1.0
    dec = np.sign(s) * SDF_CLIP * s * s

    # ---- report ----
    print(f"H_EXT = {H_EXT!r}   Y_TOP = {Y_TOP!r}   radius = {R!r}")
    print(f"inverse round-trip max |robinson_y(lat(y)) - y| over all {H4} 4x rows: "
          f"{rt_err:.3e} canvas px  (same row grid as make_relief.py)")
    print(f"world.js: {len(countries)} countries, {n_rings} rings, {n_edges} edges "
          f"(even-odd, fill-rule the page itself uses)")
    print(f"supersample {SS}x -> {W4} x {H4}; land fraction of the whole canvas = "
          f"{land_frac:.5f}")
    print(f"SDF clip +/-{SDF_CLIP} canvas units; sqrt-companded, land positive")
    print(f"  SHADER DECODE: s = texel*2.0 - 1.0 ; d = sign(s) * {SDF_CLIP!r} * s * s")
    near = np.abs(sdf) <= SDF_CLIP
    print(f"  round-trip max |err| within the clip band: "
          f"{np.abs(dec[near] - sdf[near]).max():.4f} canvas units")
    flip = np.sign(dec) != np.sign(sdf)
    print(f"  sign agreement between the field and its encode: "
          f"{1.0 - float(flip.mean()):.6f}  "
          f"(the {int(flip.sum())} disagreements all have |d| <= "
          f"{np.abs(sdf[flip]).max():.2e} canvas units -- code 128 is the quantiser's "
          f"nearest neighbour to zero on both sides)")
    print(f"  distinct codes in the shipped field: {len(np.unique(code))} / 256")
    print()
    print(f"spheres-web/ui/coast.png : {W1} x {H1}  L8  {size} bytes  "
          f"({size / (W1 * H1):.3f} B/px)   colour chunks: {hygiene}")

    # ---- alignment proof: 3 coastline landmarks through mapgen's forward math ----
    landmarks = [
        ("Gibraltar (36.14N, -5.35E)", -5.35, 36.14),
        ("Cape Horn (-55.98N, -67.27E)", -67.27, -55.98),
        ("Tokyo Bay (35.50N, 139.90E)", 139.90, 35.50),
    ]
    for name, lon_d, lat_d in landmarks:
        x, y = project(lon_d, lat_d)
        px, py = int(math.floor(x)), int(math.floor(y / H_EXT * H1))
        print(f"\n{name} -> canvas ({x:.3f}, {y:.3f}) -> texel ({px}, {py})")
        print("5x5 decoded signed distance, canvas units (land +, water -):")
        for rr in range(py - 2, py + 3):
            print("   " + " ".join(f"{dec[rr, cc]:+7.3f}" for cc in range(px - 2, px + 3)))
        print(f"   centre reads {'land' if dec[py, px] > 0 else 'water'}, "
              f"nearest shore {abs(dec[py, px]):.3f} canvas units away")

    # ---- the load-bearing registration proof: the zero level set sits ON world.js ----
    # A country ring vertex that appears in exactly ONE ring is a COASTLINE vertex: shared
    # land borders emit identical coordinates in both neighbours' paths (mapgen rounds to
    # 1 dp, so they coincide exactly), while a shoreline vertex belongs to one country only.
    # 29,041 of the 41,148 vertices survive that filter; every 200th is then sampled out of
    # the SHIPPED encode with the same bilinear filter the GPU will use. These points are by
    # definition exactly on the curve the page strokes, so the decoded field must read ~0.
    # This is the check that catches what the landmark windows cannot see -- a half-texel row
    # offset, an off-by-one in the scanline grid, or the [0,1018]-vs-H_EXT row convention. It
    # is reported stratified by canvas y because a row-convention error is a bias that GROWS
    # toward the bottom of the map (make_underlay.py's convention is exact at y=0 and 0.194
    # canvas units out at y=H_EXT), so a single global mean would dilute it away.
    counts = {}
    for codeiso in sorted(countries):
        for ring in rings_of(countries[codeiso]):
            for vx, vy in ring:
                key = (round(float(vx), 4), round(float(vy), 4))
                counts[key] = counts.get(key, 0) + 1
    coast_v = np.asarray([k for k, c in counts.items() if c == 1], dtype=np.float64)
    sel = coast_v[::200]
    u = np.clip(sel[:, 0] - 0.5, 0.0, W1 - 1.001)
    v = np.clip(sel[:, 1] / H_EXT * H1 - 0.5, 0.0, H1 - 1.001)
    i0 = u.astype(np.int64); j0 = v.astype(np.int64)
    fu = u - i0; fv = v - j0
    samp = ((1 - fv) * ((1 - fu) * dec[j0, i0] + fu * dec[j0, i0 + 1])
            + fv * ((1 - fu) * dec[j0 + 1, i0] + fu * dec[j0 + 1, i0 + 1]))
    print(f"\nzero-level-set registration: of {len(counts)} distinct ring vertices, "
          f"{len(coast_v)} appear once (coastline, not a shared land border);")
    print(f"  every 200th -> {samp.size} points sampled bilinearly out of the shipped encode")
    print(f"  ALL          mean d = {samp.mean():+.4f}   RMS = "
          f"{math.sqrt(float((samp ** 2).mean())):.4f}   max |d| = {np.abs(samp).max():.4f} "
          f"canvas units")
    for label, lo_y, hi_y in (("y   0- 340", 0.0, 340.0),
                              ("y 340- 680", 340.0, 680.0),
                              ("y 680-1018", 680.0, H_EXT)):
        band = (sel[:, 1] >= lo_y) & (sel[:, 1] < hi_y)
        if band.any():
            print(f"  {label}  mean d = {samp[band].mean():+.4f}   RMS = "
                  f"{math.sqrt(float((samp[band] ** 2).mean())):.4f}   n = {int(band.sum())}")
    assert abs(samp.mean()) < 0.25, f"coastline vertices sit off the zero level set: " \
                                    f"mean {samp.mean()}"
    assert np.abs(samp).max() < 2.0, f"a coastline vertex is {np.abs(samp).max()} canvas " \
                                     f"units from the zero level set"

    # ---- sign spot-check: the failure modes this encoding exists to avoid ----
    print("\nsign spot-check (forward-projected; land must be +, water -):")
    spots = [
        ("Sahara interior", 12.0, 24.0, "land"),
        ("Tibet interior", 88.0, 32.0, "land"),
        # The case sign(elevation) gets wrong: 72.8% of NL-ZH is below sea level.
        ("Zuid-Holland inland", 4.60, 52.05, "land"),
        ("Ganges delta", 90.4, 23.8, "land"),
        ("Amazon interior", -62.0, -3.0, "land"),
        ("mid-Pacific", -140.0, 0.0, "water"),
        ("North Atlantic", -30.0, 45.0, "water"),
        ("Caspian Sea", 51.0, 42.0, "water"),
        # Inside the admin-0 polygons, hence land here and water in the rivers layer
        # drawn on top -- see the INLAND WATER note in the header.
        ("Lake Superior (see header)", -87.5, 47.6, "land"),
        ("Lake Baikal (see header)", 108.0, 53.5, "land"),
    ]
    ok = True
    for name, lon_d, lat_d, want in spots:
        x, y = project(lon_d, lat_d)
        px, py = int(math.floor(x)), int(math.floor(y / H_EXT * H1))
        got = "land" if dec[py, px] > 0.0 else "water"
        ok &= got == want
        print(f"  {name:28s} texel ({px:4d},{py:4d})  d = {dec[py, px]:+7.3f}  "
              f"{got:5s} (want {want}) {'OK' if got == want else 'MISMATCH'}")
    assert ok, "coast mask disagrees with a known land/water point"


if __name__ == "__main__":
    main()
