#!/usr/bin/env python
# make_underlay.py -- build the Robinson-warped hillshade underlay for the Spheres game map.
#
# Inputs (anchored on the repo root this file sits in):
#   tools/terrain/raster/SR_50M.tif
#     (Natural Earth SR_50M v2.0.0 shaded relief, 10800x5400 8-bit grayscale, WGS84 geographic,
#      exact 1/30-degree pixels; pixel-EDGE extent lon [-180,180], lat [90,-90];
#      lon = -180 + (col+0.5)/30 ; lat = 90 - (row+0.5)/30.
#      If missing, it is re-extracted from spheres-web/data/SR_50M.zip — the raster/ dir is a
#      scratch cache, not a committed artifact.)
#   Projection constants replicated EXACTLY from spheres-web/src/bin/mapgen.rs
#     (W=2400, LAT_TOP=83, LAT_BOT=-58, 19-entry RX/RY Robinson tables, radius(), robinson_y(), project()).
#
# Outputs:
#   spheres-web/ui/terrain.png   2400 x 1018 grayscale+alpha (LA) — the committed underlay,
#                                baked into the server binary by main.rs (include_bytes).
#   tools/terrain/terrain_underlay_2x.png  4800 x 2036 LA — only with --2x, for eyeballing;
#                                the 1x ships (the 2x quadruples the binary for marginal gain
#                                under a <=0.4-opacity multiply blend).
#
# Method: for every output pixel, inverse-project canvas (x, y) -> (lon, lat) by inverting mapgen's
# project(): y -> lat via the EXACT piecewise-linear inverse of the RY interpolation (the interpolation
# is piecewise linear and strictly monotonic in |lat|, so each segment inverts in closed form -- this is
# the same solution bisection would converge to, without iteration error; a forward round-trip check is
# printed to prove it), then x -> lon by dividing out the RX interpolation at that lat. The source raster
# is sampled bilinearly (longitude wraps at the +/-180 seam). The 1x image is the premultiplied-alpha
# 2x2 box downsample of the 2x render (antialiased globe edge). Pixels off-globe (|lon| > 180 deg) are
# fully transparent; every row of the canvas lies inside [LAT_BOT, LAT_TOP] by construction.
#
# Deterministic: no RNG, fixed float64 math, fixed PNG compression (compress_level=9, optimize=True,
# no time chunk). Running twice yields byte-identical PNGs.
#
# Invocation:  python tools/terrain/make_underlay.py [--2x]
#   (also prints an alignment proof: 5x5 gray/alpha neighbourhoods of the 1x underlay at the forward
#    projections of Gibraltar 36.14N 5.35W, Cape Horn 55.98S 67.27W, Tokyo Bay 35.5N 139.9E)

import math
import os
import sys
import zipfile

import numpy as np
from PIL import Image

Image.MAX_IMAGE_PIXELS = None

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
RASTER_DIR = os.path.join(ROOT, "tools/terrain/raster")
TIF = os.path.join(RASTER_DIR, "SR_50M.tif")
SRC_ZIP = os.path.join(ROOT, "spheres-web/data/SR_50M.zip")
OUT_1X = os.path.join(ROOT, "spheres-web/ui/terrain.png")
OUT_2X = os.path.join(ROOT, "tools/terrain/terrain_underlay_2x.png")
EMIT_2X = "--2x" in sys.argv[1:]

# ---- mapgen.rs constants, replicated exactly (lines 19-81 of mapgen.rs) ----
W = 2400.0
LAT_TOP = 83.0
LAT_BOT = -58.0
RX = [1.0000, 0.9986, 0.9954, 0.9900, 0.9822, 0.9730, 0.9600, 0.9427, 0.9216, 0.8962,
      0.8679, 0.8350, 0.7986, 0.7597, 0.7186, 0.6732, 0.6213, 0.5722, 0.5322]
RY = [0.0000, 0.0620, 0.1240, 0.1860, 0.2480, 0.3100, 0.3720, 0.4340, 0.4958, 0.5571,
      0.6176, 0.6769, 0.7346, 0.7903, 0.8435, 0.8936, 0.9394, 0.9761, 1.0000]


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
HEIGHT = Y_TOP - robinson_y(LAT_BOT)

# ---- exact inverse of robinson_y: canvas y -> lat ----
RY_ARR = np.asarray(RY, dtype=np.float64)
RY_DIFF = np.diff(RY_ARR)


def lat_from_canvas_y(y):
    """Vectorized exact inverse. y: float64 array of canvas y in [0, HEIGHT]."""
    g = (Y_TOP - np.asarray(y, dtype=np.float64)) / (1.3523 * R)   # signed interp(RY,|lat|)
    sign = np.where(g < 0.0, -1.0, 1.0)
    gg = np.abs(g)
    i = np.clip(np.searchsorted(RY_ARR, gg, side="right") - 1, 0, 17)
    t = i + (gg - RY_ARR[i]) / RY_DIFF[i]
    return sign * 5.0 * t


def main():
    if not os.path.exists(TIF):
        os.makedirs(RASTER_DIR, exist_ok=True)
        with zipfile.ZipFile(SRC_ZIP) as z:
            z.extractall(RASTER_DIR)

    src = np.asarray(Image.open(TIF), dtype=np.uint8)   # (5400, 10800) grayscale
    sh, sw = src.shape
    assert (sh, sw) == (5400, 10800), (sh, sw)
    srcf = src.astype(np.float64)

    W1, H1 = 2400, 1018            # full-res canvas (height() = 1018.1941195106424)
    W2, H2 = 2 * W1, 2 * H1        # 2x supersample

    # per-row latitude at 2x (row r center -> canvas y=(r+0.5)/2)
    yc = (np.arange(H2, dtype=np.float64) + 0.5) / 2.0
    lats = lat_from_canvas_y(yc)

    # round-trip proof of the inverse
    rt_err = np.max(np.abs(np.array([Y_TOP - robinson_y(la) for la in lats]) - yc))
    # rx interpolation per row
    rx = np.array([interp(RX, abs(la)) for la in lats])

    xc = (np.arange(W2, dtype=np.float64) + 0.5) / 2.0   # canvas x of 2x column centers
    k = 0.8487 * R

    gray2 = np.zeros((H2, W2), dtype=np.float64)
    mask2 = np.zeros((H2, W2), dtype=np.float64)

    CHUNK = 256
    for r0 in range(0, H2, CHUNK):
        r1 = min(r0 + CHUNK, H2)
        rxr = rx[r0:r1][:, None]                          # (n,1)
        latr = lats[r0:r1][:, None]
        lon_deg = np.degrees((xc[None, :] - W / 2.0) / (k * rxr))   # (n, W2)
        m = np.abs(lon_deg) <= 180.0

        sx = (lon_deg + 180.0) * 30.0 - 0.5               # source col (wraps)
        sy = (90.0 - latr) * 30.0 - 0.5                   # source row
        sy = np.broadcast_to(sy, sx.shape)

        x0 = np.floor(sx).astype(np.int64)
        fx = sx - x0
        x0i = np.mod(x0, sw)
        x1i = np.mod(x0 + 1, sw)
        y0 = np.clip(np.floor(sy).astype(np.int64), 0, sh - 2)
        fy = np.clip(sy - y0, 0.0, 1.0)

        val = ((1.0 - fy) * ((1.0 - fx) * srcf[y0, x0i] + fx * srcf[y0, x1i])
               + fy * ((1.0 - fx) * srcf[y0 + 1, x0i] + fx * srcf[y0 + 1, x1i]))

        gray2[r0:r1] = np.where(m, val, 0.0)
        mask2[r0:r1] = m.astype(np.float64)

    # ---- 2x output (quantized) ----
    g2u = np.rint(gray2).astype(np.uint8)
    a2u = (mask2 * 255.0).astype(np.uint8)

    # ---- 1x output: premultiplied 2x2 box downsample ----
    pm = (gray2 * mask2).reshape(H1, 2, W1, 2).sum(axis=(1, 3))
    msum = mask2.reshape(H1, 2, W1, 2).sum(axis=(1, 3))
    g1 = np.where(msum > 0.0, pm / np.maximum(msum, 1e-12), 0.0)
    a1 = msum / 4.0 * 255.0
    g1u = np.rint(g1).astype(np.uint8)
    a1u = np.rint(a1).astype(np.uint8)

    im1 = Image.fromarray(np.dstack([g1u, a1u]), mode="LA")
    im1.save(OUT_1X, optimize=True, compress_level=9)
    size1 = os.path.getsize(OUT_1X)

    size2 = None
    if EMIT_2X:
        im2 = Image.fromarray(np.dstack([g2u, a2u]), mode="LA")
        im2.save(OUT_2X, optimize=True, compress_level=9)
        size2 = os.path.getsize(OUT_2X)

    # ---- report ----
    print(f"height() = {HEIGHT!r}   Y_TOP = {Y_TOP!r}   radius = {R!r}")
    print(f"inverse round-trip max |robinson_y(lat(y)) - y| over all 2x rows: {rt_err:.3e} canvas px")
    print(f"spheres-web/ui/terrain.png : {W1} x {H1}  LA  {size1} bytes")
    if size2 is not None:
        print(f"terrain_underlay_2x.png    : {W2} x {H2}  LA  {size2} bytes (not committed)")
    else:
        print("terrain_underlay_2x.png    : skipped (pass --2x to emit)")

    # ---- alignment proof: 3 coastline landmarks through mapgen's forward math ----
    landmarks = [
        ("Gibraltar (36.14N, -5.35E)", -5.35, 36.14),
        ("Cape Horn (-55.98N, -67.27E)", -67.27, -55.98),
        ("Tokyo Bay (35.50N, 139.90E)", 139.90, 35.50),
    ]
    for name, lon, lat in landmarks:
        x, y = project(lon, lat)
        px, py = int(math.floor(x)), int(math.floor(y))
        print(f"\n{name} -> canvas ({x:.3f}, {y:.3f}) -> pixel ({px}, {py})")
        print("5x5 gray neighbourhood (rows py-2..py+2, cols px-2..px+2), alpha in ():")
        for rr in range(py - 2, py + 3):
            cells = []
            for cc in range(px - 2, px + 3):
                cells.append(f"{g1u[rr, cc]:3d}({a1u[rr, cc]:3d})")
            print("   " + "  ".join(cells))


if __name__ == "__main__":
    main()
