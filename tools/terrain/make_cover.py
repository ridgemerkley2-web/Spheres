#!/usr/bin/env python
# make_cover.py -- bake a single vegetation scalar out of Natural Earth 1 into the game canvas.
#
# Inputs (anchored on the repo root this file sits in):
#   tools/terrain/raster/NE1_50M_SR_W/NE1_50M_SR_W.tif
#     (Natural Earth NE1_50M_SR_W natural-colour + shaded relief, 10800x5400 8-bit RGB,
#      uncompressed strips, WGS84 geographic, exact 1/30-degree pixels; pixel-EDGE extent
#      lon [-180,180], lat [90,-90]; lon = -180 + (col+0.5)/30 ; lat = 90 - (row+0.5)/30 --
#      the SAME grid SR_50M.tif uses, so make_underlay.py's sampler is reused verbatim with
#      the channel count changed from 1 to 3. Public domain.
#      NOTE the extraction gotcha: unlike SR_50M.zip, which extracts flat, this archive
#      carries a SUBDIRECTORY, so extractall(raster/) lands the tif one level down. The tif
#      also carries a stray Photoshop 8BIM blob in tag 34377 that makes Pillow warn on
#      tag 33723; harmless, and silenced below so the run's output stays readable.
#      If the tif is missing it is re-extracted from spheres-web/data/NE1_50M_SR_W.zip --
#      raster/ is a scratch cache, not a committed artifact.)
#   spheres-web/ui/coast.png
#     (this run's coastline field -- make_coast.py must run FIRST. Used only to know where
#      land is, so the seaward fill below uses the SAME coastline the renderer samples.)
#   Projection constants replicated EXACTLY from spheres-web/src/bin/mapgen.rs.
#
# Output:
#   spheres-web/ui/cover.png   1200 x 509 L8 -- the committed vegetation index, baked into
#                              the server binary by main.rs and served from /cover.png.
#
# ONE CHANNEL, AND IT IS NOT NE1's COLOUR.
#   NE1's saturated greens and tans never reach the screen; only this scalar is extracted,
#   and the renderer's palette stays hand-authored against the Terrain legend's own swatches.
#   Extracting the colour instead would cost 628,049 B against this file's 260,856 B, import
#   a second art direction into a steel-and-brass UI, and -- measured -- leave 5.98/1.91/5.99
#   of high-frequency chromaticity residual, i.e. some of NE1's own baked hillshade surviving
#   to double-shade against the renderer's.
#
#   V = clamp((greenExcess - 0.005) / 0.100, 0, 1),  greenExcess = (G - (R+B)/2) / 255
#
#   The obvious aridity discriminator (R-B)/255 INVERTS on this raster and was rejected on
#   measurement: the Gangetic plain (+0.171) and the Nile delta (+0.175) score MORE arid than
#   the Great Victoria Desert (+0.162), while the Atacama (+0.064) and Tibet (-0.009) score
#   humid. Green-excess orders correctly across every biome box below, and the run asserts
#   that ordering rather than asserting the formula.
#
# HALF RESOLUTION IS THE POINT, NOT A COMPROMISE. V is a biome-scale field with no
# high-frequency content of its own: roughly 1.27 MB at 2400 wide against 260,856 B at 1200,
# for a scalar whose finest real feature is hundreds of kilometres across. Every
# high-frequency cue in the finished image comes from the renderer's own heightmap relief,
# which is baked at full width. The 2x supersample this file renders on IS the full-width
# grid, so nothing is lost to the choice except the bytes.
#
# SEAWARD FILL: before the downsample, every water texel within 12 canvas units of land takes
# the V of its nearest land texel. Without it, bilinear and mip sampling near a shore pull an
# ocean zero into a coastal biome and rim every continent with a band of false desert. The
# land mask is read from THIS run's coast.png, so the fill boundary is the same curve the
# renderer's land/water test uses -- not a second opinion about where the coast is.
#
# Off-globe texels (|lon| > 180) carry the edge-clamped on-globe value, implemented by
# clamping the sampled longitude to +/-180: the globe edge IS lon = +/-180, so the clamp
# reads the nearest on-globe value by construction.
#
# ROW EXTENT: texel (i, j) centre is canvas ((i+0.5)*2400/1200, (j+0.5)*H_EXT/509) with
#   H_EXT = Y_TOP - robinson_y(LAT_BOT) = 1018.1941195106424
# the exact projection extent -- NOT make_underlay.py's [0, 1018] rows and NOT WORLD.h's
# 1-dp 1018.2. The 2x supersample grid this bake renders on is therefore EXACTLY relief.png's
# and coast.png's 2400 x 1018 grid, which is why the coast mask can be consumed texel for
# texel with no resampling at all.
#
# Deterministic: no RNG, no wall clock, fixed float64 math, fixed iteration order, fixed PNG
# compression (compress_level=9, optimize=True). Running twice yields a byte-identical PNG,
# and the written file is scanned for gAMA/sRGB/iCCP and rejected if any is present.
#
# Invocation:  python tools/terrain/make_cover.py
#   (prints H_EXT, the inverse round-trip error, the output byte count, chunk hygiene, the
#    3-landmark alignment proof over Gibraltar / Cape Horn / Tokyo Bay, and the 12-biome
#    ordering table with its assertion.)

import math
import os
import warnings
import zipfile

import numpy as np
from PIL import Image
from scipy import ndimage

Image.MAX_IMAGE_PIXELS = None
warnings.filterwarnings("ignore", message=".*Corrupt EXIF data.*")
warnings.filterwarnings("ignore", category=UserWarning, module="PIL.TiffImagePlugin")

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
RASTER_DIR = os.path.join(ROOT, "tools/terrain/raster")
# The zip carries a subdirectory; SR_50M.zip does not. Do not "copy make_underlay.py".
TIF = os.path.join(RASTER_DIR, "NE1_50M_SR_W", "NE1_50M_SR_W.tif")
SRC_ZIP = os.path.join(ROOT, "spheres-web/data/NE1_50M_SR_W.zip")
COAST_PNG = os.path.join(ROOT, "spheres-web/ui/coast.png")
OUT = os.path.join(ROOT, "spheres-web/ui/cover.png")

# ---- mapgen.rs constants, replicated exactly (lines 19-81 of mapgen.rs) ----
W = 2400.0
LAT_TOP = 83.0
LAT_BOT = -58.0
RX = [1.0000, 0.9986, 0.9954, 0.9900, 0.9822, 0.9730, 0.9600, 0.9427, 0.9216, 0.8962,
      0.8679, 0.8350, 0.7986, 0.7597, 0.7186, 0.6732, 0.6213, 0.5722, 0.5322]
RY = [0.0000, 0.0620, 0.1240, 0.1860, 0.2480, 0.3100, 0.3720, 0.4340, 0.4958, 0.5571,
      0.6176, 0.6769, 0.7346, 0.7903, 0.8435, 0.8936, 0.9394, 0.9761, 1.0000]

GE_LO, GE_SPAN = 0.005, 0.100      # greenExcess -> V window
FILL_RADIUS = 12.0                 # canvas units of seaward fill = 6 cover texels
LOWPASS_SIGMA = 1.5                # supersample texels (= 0.75 cover texels); see header
SDF_CLIP = 8.0                     # coast.png's companding constant


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
    """Vectorized exact inverse of robinson_y (make_underlay.py's, verbatim)."""
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


def main():
    if not os.path.exists(TIF):
        os.makedirs(RASTER_DIR, exist_ok=True)
        with zipfile.ZipFile(SRC_ZIP) as z:
            z.extractall(RASTER_DIR)
    assert os.path.exists(TIF), f"{TIF} missing after extraction (subdirectory layout?)"

    src = np.asarray(Image.open(TIF), dtype=np.uint8)          # (5400, 10800, 3)
    sh, sw = src.shape[:2]
    assert (sh, sw, src.shape[2]) == (5400, 10800, 3), src.shape
    srcf = src.astype(np.float64)

    W1, H1 = 1200, 509                     # shipped
    W2, H2 = 2 * W1, 2 * H1                # 2x supersample == relief/coast's own 1x grid
    assert (W2, H2) == (2400, 1018)

    yc = (np.arange(H2, dtype=np.float64) + 0.5) * (H_EXT / H2)
    lats = lat_from_canvas_y(yc)
    rt_err = float(np.max(np.abs(
        np.array([Y_TOP - robinson_y(la) for la in lats], dtype=np.float64) - yc)))
    rx = np.array([interp(RX, abs(la)) for la in lats], dtype=np.float64)

    xc = (np.arange(W2, dtype=np.float64) + 0.5) * (W / W2)
    k = 0.8487 * R

    ge = np.empty((H2, W2), dtype=np.float64)
    rb = np.empty((H2, W2), dtype=np.float64)
    off_globe = 0
    CHUNK = 128
    for r0 in range(0, H2, CHUNK):
        r1 = min(r0 + CHUNK, H2)
        rxr = rx[r0:r1][:, None]
        latr = lats[r0:r1][:, None]
        lon_deg = np.degrees((xc[None, :] - W / 2.0) / (k * rxr))
        off_globe += int(np.count_nonzero(np.abs(lon_deg) > 180.0))
        np.clip(lon_deg, -180.0, 180.0, out=lon_deg)

        sx = (lon_deg + 180.0) * 30.0 - 0.5
        sy = np.broadcast_to((90.0 - latr) * 30.0 - 0.5, sx.shape)

        x0 = np.floor(sx).astype(np.int64)
        fx = sx - x0
        x0i = np.mod(x0, sw)
        x1i = np.mod(x0 + 1, sw)
        y0 = np.clip(np.floor(sy).astype(np.int64), 0, sh - 2)
        fy = np.clip(sy - y0, 0.0, 1.0)

        # bilinear on all three channels, then the scalar -- the interpolation is of colour,
        # and the nonlinearity is applied once, at the output grid.
        rgb = ((1.0 - fy)[..., None] * ((1.0 - fx)[..., None] * srcf[y0, x0i]
                                        + fx[..., None] * srcf[y0, x1i])
               + fy[..., None] * ((1.0 - fx)[..., None] * srcf[y0 + 1, x0i]
                                  + fx[..., None] * srcf[y0 + 1, x1i]))
        ge[r0:r1] = (rgb[..., 1] - 0.5 * (rgb[..., 0] + rgb[..., 2])) / 255.0
        # The rejected discriminator, kept only so the biome table can show it inverting.
        rb[r0:r1] = (rgb[..., 0] - rgb[..., 2]) / 255.0

    del srcf, src
    v2 = np.clip((ge - GE_LO) / GE_SPAN, 0.0, 1.0)

    # ---- seaward fill from THIS run's coastline ----
    coast = np.asarray(Image.open(COAST_PNG), dtype=np.float64)
    assert coast.shape == (H2, W2), f"coast.png is {coast.shape}, expected {(H2, W2)}"
    s = coast / 255.0 * 2.0 - 1.0
    land = (np.sign(s) * SDF_CLIP * s * s) > 0.0
    dist, idx = ndimage.distance_transform_edt(
        ~land, sampling=(H_EXT / H2, W / W2), return_indices=True)
    fill = (~land) & (dist <= FILL_RADIUS)
    v2 = np.where(fill, v2[idx[0], idx[1]], v2)
    n_filled = int(fill.sum())
    del coast, s, dist, idx

    # ---- biome-scale low-pass, applied on the supersample grid before the downsample ----
    # NE1 is a land-COVER raster and carries per-pixel speckle -- field boundaries, cloud
    # residue, its own baked shading. None of that is signal for a palette blend at biome
    # scale, and all of it is entropy: measured on this bake, 359,442 B unfiltered against
    # 260,856 B at sigma = 1.5 supersample texels, for an RMS change of 0.030 in a scalar
    # whose narrowest useful contrast is the 0.073 gap between adjacent biome tiers. This is
    # a low-pass of a field that was never meant to carry detail, not compression after the
    # fact -- the 256 output levels and the smooth ramp survive, where quantising to 32
    # levels buys the same 94 KB by terracing the one gradient the channel exists to draw.
    v2 = ndimage.gaussian_filter(v2, LOWPASS_SIGMA, mode="nearest")

    v1 = v2.reshape(H1, 2, W1, 2).mean(axis=(1, 3))
    code = np.clip(np.rint(v1 * 255.0), 0, 255).astype(np.uint8)
    Image.fromarray(code).save(OUT, optimize=True, compress_level=9)   # 2D uint8 -> mode "L"
    size = os.path.getsize(OUT)
    hygiene = png_chunk_hygiene(OUT)
    dec = code.astype(np.float64) / 255.0

    # ---- report ----
    print(f"H_EXT = {H_EXT!r}   Y_TOP = {Y_TOP!r}   radius = {R!r}")
    print(f"inverse round-trip max |robinson_y(lat(y)) - y| over all {H2} 2x rows: "
          f"{rt_err:.3e} canvas px  (same row grid as make_relief.py / make_coast.py)")
    print(f"off-globe texels at 2x (|lon| > 180, edge-clamped): {off_globe} / {W2 * H2} "
          f"= {off_globe / (W2 * H2):.5f}")
    print(f"V = clamp((greenExcess - {GE_LO}) / {GE_SPAN}, 0, 1);  "
          f"greenExcess = (G - (R+B)/2)/255")
    print(f"  SHADER DECODE: V = texel   (L8, 0..1, no companding)")
    print(f"seaward fill: {n_filled} of {int((~land).sum())} water texels within "
          f"{FILL_RADIUS} canvas units of land take their nearest land V")
    print(f"land fraction of the 2x grid (from coast.png): {float(land.mean()):.5f}")
    print()
    print(f"spheres-web/ui/cover.png : {W1} x {H1}  L8  {size} bytes  "
          f"({size / (W1 * H1):.3f} B/px)   colour chunks: {hygiene}")
    print(f"  distinct codes: {len(np.unique(code))} / 256   "
          f"encode round-trip max |err| = {np.abs(dec - v1).max():.5f}")

    # ---- alignment proof: 3 coastline landmarks through mapgen's forward math ----
    landmarks = [
        ("Gibraltar (36.14N, -5.35E)", -5.35, 36.14),
        ("Cape Horn (-55.98N, -67.27E)", -67.27, -55.98),
        ("Tokyo Bay (35.50N, 139.90E)", 139.90, 35.50),
    ]
    for name, lon_d, lat_d in landmarks:
        x, y = project(lon_d, lat_d)
        px, py = int(math.floor(x / W * W1)), int(math.floor(y / H_EXT * H1))
        print(f"\n{name} -> canvas ({x:.3f}, {y:.3f}) -> texel ({px}, {py})")
        print("5x5 decoded V (rows py-2..py+2, cols px-2..px+2):")
        for rr in range(py - 2, py + 3):
            print("   " + " ".join(f"{dec[rr, cc]:.3f}" for cc in range(px - 2, px + 3)))

    # ---- biome ordering: the content proof, and the one this scalar was chosen on ----
    # 9x9 lattices of lon/lat cell centres inside each box, forward-projected one at a time
    # through mapgen's own project(). Ordering, not absolute values, is what the renderer
    # relies on and what the rejected (R-B) discriminator got wrong.
    # Ordering is asserted by TIER, not as a total order: "Scandinavian taiga is wetter than
    # West Siberian taiga" is not a claim this scalar makes or needs to make, and forcing a
    # total order would turn a real property into a coin flip between neighbours in the same
    # biome band. What the renderer relies on is that the bands separate, so that is what is
    # measured -- max(tier) < min(next tier), with the gap printed.
    tiers = [
        ("barren", [
            ("Atacama", -70.0, -68.0, -25.0, -23.0),
            ("Tibet plateau", 86.0, 92.0, 31.0, 34.0),
            ("Sahara (Ahaggar)", 4.0, 12.0, 22.0, 26.0),
            ("Great Victoria", 126.0, 132.0, -29.0, -26.0)]),
        ("semi-arid", [
            ("Sahel", 0.0, 10.0, 13.0, 16.0)]),
        ("temperate", [
            ("Alps", 6.0, 12.0, 46.0, 47.5),
            ("Great Plains", -102.0, -98.0, 38.0, 42.0)]),
        ("boreal / humid", [
            ("West Siberia", 70.0, 78.0, 58.0, 62.0),
            ("Gangetic plain", 80.0, 87.0, 25.0, 27.0),
            ("Scandinavia", 14.0, 20.0, 61.0, 64.0)]),
        ("rainforest", [
            ("Amazon basin", -64.0, -60.0, -4.0, -2.0),
            ("Congo basin", 18.0, 24.0, -2.0, 2.0)]),
    ]

    def box_mean(field, w, h, lo0, lo1, la0, la1):
        vals = []
        for a in range(9):
            for b in range(9):
                lon_d = lo0 + (lo1 - lo0) * (a + 0.5) / 9.0
                lat_d = la0 + (la1 - la0) * (b + 0.5) / 9.0
                x, y = project(lon_d, lat_d)
                px = min(max(int(x / W * w), 0), w - 1)
                py = min(max(int(y / H_EXT * h), 0), h - 1)
                vals.append(field[py, px])
        return float(np.mean(vals))

    print("\nbiome tiers (mean over a 9x9 lon/lat lattice per box; V from the shipped PNG,")
    print("(R-B)/255 from the warped source -- the discriminator this channel rejected):")
    tier_ranges = []
    for tier_name, tier_boxes in tiers:
        vs = []
        for name, lo0, lo1, la0, la1 in tier_boxes:
            v = box_mean(dec, W1, H1, lo0, lo1, la0, la1)
            a = box_mean(rb, W2, H2, lo0, lo1, la0, la1)
            vs.append(v)
            print(f"  [{tier_name:14s}] {name:18s} V = {v:.3f}  (R-B) = {a:+.3f}   "
                  f"{'#' * int(round(v * 40))}")
        tier_ranges.append((tier_name, min(vs), max(vs)))
    print()
    ok = True
    for (n0, _, hi0), (n1, lo1v, _) in zip(tier_ranges, tier_ranges[1:]):
        gap = lo1v - hi0
        ok &= gap > 0.0
        print(f"  {n0:14s} max {hi0:.3f}  <  {n1:14s} min {lo1v:.3f}   gap {gap:+.3f}  "
              f"{'OK' if gap > 0.0 else 'INVERTED'}")
    assert ok, "the vegetation index does not separate the biome tiers"

    # The rejected discriminator, on the same boxes, failing the same test.
    arid = [(name, box_mean(rb, W2, H2, lo0, lo1, la0, la1))
            for _, tb in tiers for (name, lo0, lo1, la0, la1) in tb]
    worst = [(a[0], b[0], a[1], b[1]) for a, b in zip(arid, arid[1:]) if a[1] < b[1]]
    print(f"\n  for the record, (R-B)/255 over the same boxes inverts on "
          f"{len(worst)} of {len(arid) - 1} adjacent pairs, e.g. "
          f"{worst[0][0]} {worst[0][2]:+.3f} vs {worst[0][1]} {worst[0][3]:+.3f}"
          if worst else "\n  (R-B)/255 happened not to invert on these boxes")


if __name__ == "__main__":
    main()
