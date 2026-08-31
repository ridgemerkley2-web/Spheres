#!/usr/bin/env python
# make_relief.py -- bake the ETOPO 2022 heightmap into the Robinson game canvas.
#
# Inputs (anchored on the repo root this file sits in):
#   spheres-web/data/etopo_60s.nc
#     (NOAA NCEI ETOPO 2022 60-arc-second "Topography-Bathymetry; EGM2008 height", public
#      domain. netCDF-4, i.e. HDF5 -- read with h5py; netCDF4/xarray are absent and
#      scipy.io.netcdf_file is a netCDF-3 reader that cannot open it. Datasets: z
#      (10800, 21600) float32, lat, lon, crs. Cell CENTRES, node_offset=1:
#          lat = -89.99166666666666 + row/60      lon = -179.99166666666667 + col/60
#      so ROWS RUN SOUTH -> NORTH, the OPPOSITE of SR_50M.tif -- make_underlay.py's row
#      indexing must NOT be copied. Verified on this file: z[0] mean +2832.05 m
#      (Antarctic ice surface), z[10799] mean -4189.94 m (Arctic Ocean). The
#      crs.GeoTransform attribute claims origin at +90 with negative dy; it describes the
#      GDAL view, not the array order, and is wrong for this array. The grid is gap-free:
#      zero _FillValue (-99999) cells and zero NaNs over all 233,280,000 samples, so no
#      mask handling is needed anywhere below.)
#   Projection constants replicated EXACTLY from spheres-web/src/bin/mapgen.rs
#     (W=2400, LAT_TOP=83, LAT_BOT=-58, 19-entry RX/RY Robinson tables, radius(),
#      robinson_y(), project() -- mapgen.rs lines 19-81, the same block make_underlay.py
#      and check.py replicate).
#
# Output:
#   spheres-web/ui/relief.png   2400 x 1018 RGB8 -- the committed heightmap, baked into
#                               the server binary by main.rs (include_bytes) and served
#                               from /relief.png.
#   tools/terrain/relief_2x.png 4800 x 2036 RGB8 -- only with --2x, for eyeballing; the
#                               1x ships.
#
# Encoding (NO alpha channel, NO off-globe sentinel):
#   R,G   uint16 big-endian elevation, LINEAR over [ELEV_LO, ELEV_HI]
#           code = round((clip(h, LO, HI) - LO) * 65535/(HI-LO)) ; R = code >> 8, G = code & 255
#           decode: h = (R*256.0 + G) * ((HI-LO)/65535.0) + LO
#   B     uint8 sqrt-companded ocean depth over [0, DEPTH_MAX] m; 0 on land
#           code = 0 where h >= 0, else max(1, round(255 * sqrt(depth/DEPTH_MAX)))
#           decode: depth = DEPTH_MAX * (B/255)^2
#   Off-globe texels (|lon| > 180) carry the EDGE-CLAMPED on-globe value -- never 0, never
#   a sentinel. The renderer decides off-globe analytically from its own longitude, so no
#   mask is stored; edge-clamping only exists to stop mip reduction from bleeding a dark
#   fringe inward at the globe edge. It is implemented by clamping the sampled longitude to
#   +/-180 rather than by a post-pass: the globe edge IS lon = +/-180, so the clamp reads the
#   nearest on-globe value by construction.
#
#   Two bytes of elevation is not a luxury. An 8-bit encode of the full range uses 84
#   distinct codes over the whole rendered land surface and takes 78-97% of adjacent texel
#   pairs in the Amazon and West Siberia to an identical value -- the hillshade x-gradient
#   goes exactly zero and the relief reads as terraced flats. RG16 leaves 1.9% of such pairs
#   dead. A 16-bit greyscale PNG would be the obvious alternative and is a trap: <img>,
#   ImageBitmap, canvas and texImage2D all deliver 8 bits per channel, so the high bits are
#   unreachable from an <img>-sourced texture. Two 8-bit channels is the only route.
#
# ELEV_HI is DERIVED from the warped array, not assumed: peak amplitude is a function of the
# bake width, not of Everest (source 8157.36 m -> 7060.63 m at 2x -> 6230.66 m at 1x), so a
# 2x bake reusing the 1x ceiling silently truncates the summits. The smallest ceiling on a
# fixed ladder that clears the observed peak is chosen, printed, and asserted; the renderer's
# decode constant must match the printed value.
#
# Method: for every output texel, inverse-project canvas (x, y) -> (lon, lat) by inverting
# mapgen's project(): y -> lat via the EXACT piecewise-linear inverse of the RY interpolation
# (strictly monotonic in |lat|, so each 5-degree segment inverts in closed form; a forward
# round-trip check is printed to prove it), then x -> lon by dividing out the RX interpolation
# at that lat. ETOPO is sampled bilinearly (longitude wraps at the +/-180 seam). The shipped
# image is the 2x2 box downsample of a 2x supersampled render, and the downsample averages
# ELEVATION, before quantisation -- averaging codes would bake the quantiser into the terrain.
#
# ROW EXTENT -- the single most alignment-critical constant here:
#   H_EXT = Y_TOP - robinson_y(LAT_BOT) = 1018.1941195106424
#   texel (i, j) centre is canvas (x, y) = ((i+0.5)*2400/WT, (j+0.5)*H_EXT/HT)
# make_underlay.py bakes its rows over [0, 1018] and the page then stretches that over
# WORLD.h = 1018.2 -- a real 0.2-unit error at the bottom edge that today's terrain.png
# carries. This bake does NOT copy that row convention and does NOT substitute 1018.2: the
# renderer recovers latitude analytically from Y_TOP and the Robinson radius, so the texture
# rows and the shader's latitude agree only if both use the exact projection extent.
#
# Deterministic: no RNG, no wall clock, fixed float64 math, fixed iteration order, fixed PNG
# compression (compress_level=9, optimize=True, no time chunk). Running twice yields
# byte-identical PNGs. The written file is scanned for gAMA/sRGB/iCCP chunks and rejected if
# any is present -- a decoder that gamma-corrects the R,G pair destroys the packed uint16.
#
# Invocation:  python tools/terrain/make_relief.py [--2x]
#   (prints H_EXT, the inverse round-trip error, the derived ELEV_HI and its assertions, the
#    output byte count, chunk hygiene, the 3-landmark alignment proof over Gibraltar 36.14N
#    5.35W / Cape Horn 55.98S 67.27W / Tokyo Bay 35.5N 139.9E, and an elevation spot-check
#    over Everest, the Dead Sea, Death Valley and the Mariana Trench.)

import math
import os
import sys

import h5py
import numpy as np
from PIL import Image

Image.MAX_IMAGE_PIXELS = None

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
ETOPO = os.path.join(ROOT, "spheres-web/data/etopo_60s.nc")
OUT_1X = os.path.join(ROOT, "spheres-web/ui/relief.png")
OUT_2X = os.path.join(ROOT, "tools/terrain/relief_2x.png")
EMIT_2X = "--2x" in sys.argv[1:]

# ---- mapgen.rs constants, replicated exactly (lines 19-81 of mapgen.rs) ----
W = 2400.0
LAT_TOP = 83.0
LAT_BOT = -58.0
RX = [1.0000, 0.9986, 0.9954, 0.9900, 0.9822, 0.9730, 0.9600, 0.9427, 0.9216, 0.8962,
      0.8679, 0.8350, 0.7986, 0.7597, 0.7186, 0.6732, 0.6213, 0.5722, 0.5322]
RY = [0.0000, 0.0620, 0.1240, 0.1860, 0.2480, 0.3100, 0.3720, 0.4340, 0.4958, 0.5571,
      0.6176, 0.6769, 0.7346, 0.7903, 0.8435, 0.8936, 0.9394, 0.9761, 1.0000]

# ---- ETOPO grid constants (probed off the file itself; asserted in main) ----
ET_H, ET_W = 10800, 21600
ET_LAT0 = -89.99166666666666      # centre of row 0 -- the SOUTHERNMOST row
ET_LON0 = -179.99166666666667     # centre of col 0
ET_STEP = 60.0                    # cells per degree

# ---- encoding ----
ELEV_LO = -1500.0
# Smallest ceiling that clears the observed peak of the warped array. 6400 is the 1x
# bake's; 7200 the 2x bake's; 8300 covers a source-resolution bake. Chosen, not assumed.
ELEV_HI_LADDER = (6400.0, 7200.0, 8300.0, 9000.0)
DEPTH_MAX = 11000.0


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

# ---- exact inverse of robinson_y: canvas y -> lat ----
RY_ARR = np.asarray(RY, dtype=np.float64)
RY_DIFF = np.diff(RY_ARR)


def lat_from_canvas_y(y):
    """Vectorized exact inverse. y: float64 array of canvas y in [0, H_EXT]."""
    g = (Y_TOP - np.asarray(y, dtype=np.float64)) / (1.3523 * R)   # signed interp(RY,|lat|)
    sign = np.where(g < 0.0, -1.0, 1.0)
    gg = np.abs(g)
    i = np.clip(np.searchsorted(RY_ARR, gg, side="right") - 1, 0, 17)
    t = i + (gg - RY_ARR[i]) / RY_DIFF[i]
    return sign * 5.0 * t


def png_chunk_hygiene(path):
    """Reject a colour-management chunk: a decoder that gamma-corrects R,G destroys the
    packed uint16 elevation. Pillow writes none of these by default -- this proves it."""
    blob = open(path, "rb").read()
    found = [c for c in (b"gAMA", b"sRGB", b"iCCP") if c in blob]
    assert not found, f"{os.path.basename(path)} carries colour chunks {found}"
    return "none (gAMA/sRGB/iCCP all absent)"


def warp(z, wt, ht, chunk_rows=256):
    """Bilinear-sample ETOPO onto a wt x ht Robinson canvas grid, float64 metres.

    Texel (i, j) centre is canvas ((i+0.5)*W/wt, (j+0.5)*H_EXT/ht). Longitude is clamped
    to +/-180 before sampling, which edge-clamps the off-globe wings to the nearest
    on-globe value. Output rows are processed in bands so only the ETOPO rows a band
    needs are decompressed; the mapping is monotonic, so the bands are contiguous.
    """
    yc = (np.arange(ht, dtype=np.float64) + 0.5) * (H_EXT / ht)
    lats = lat_from_canvas_y(yc)
    rt_err = float(np.max(np.abs(
        np.array([Y_TOP - robinson_y(la) for la in lats], dtype=np.float64) - yc)))
    rx = np.array([interp(RX, abs(la)) for la in lats], dtype=np.float64)

    xc = (np.arange(wt, dtype=np.float64) + 0.5) * (W / wt)
    k = 0.8487 * R

    sy_all = (lats - ET_LAT0) * ET_STEP                      # source row, decreasing
    out = np.empty((ht, wt), dtype=np.float64)
    off_globe = 0

    for r0 in range(0, ht, chunk_rows):
        r1 = min(r0 + chunk_rows, ht)
        sy = sy_all[r0:r1]
        b0 = int(math.floor(sy.min()))
        b1 = int(math.floor(sy.max())) + 1
        b0 = max(0, min(b0, ET_H - 2))
        b1 = max(b0 + 1, min(b1, ET_H - 1))
        band = np.asarray(z[b0:b1 + 1, :], dtype=np.float64)   # (b1-b0+1, ET_W)

        rxr = rx[r0:r1][:, None]
        lon_deg = np.degrees((xc[None, :] - W / 2.0) / (k * rxr))
        off_globe += int(np.count_nonzero(np.abs(lon_deg) > 180.0))
        np.clip(lon_deg, -180.0, 180.0, out=lon_deg)

        sx = (lon_deg - ET_LON0) * ET_STEP
        x0 = np.floor(sx).astype(np.int64)
        fx = sx - x0
        x0i = np.mod(x0, ET_W)
        x1i = np.mod(x0 + 1, ET_W)

        y0 = np.clip(np.floor(sy).astype(np.int64), 0, ET_H - 2)
        fy = np.clip(sy - y0, 0.0, 1.0)[:, None]
        y0b = (y0 - b0)[:, None]
        y0b = np.broadcast_to(y0b, sx.shape)
        y1b = y0b + 1

        out[r0:r1] = ((1.0 - fy) * ((1.0 - fx) * band[y0b, x0i] + fx * band[y0b, x1i])
                      + fy * ((1.0 - fx) * band[y1b, x0i] + fx * band[y1b, x1i]))
        del band

    return out, rt_err, off_globe


def encode(h, elev_hi):
    """Elevation metres -> (R, G, B) uint8 planes."""
    code = np.rint((np.clip(h, ELEV_LO, elev_hi) - ELEV_LO)
                   * (65535.0 / (elev_hi - ELEV_LO))).astype(np.int64)
    np.clip(code, 0, 65535, out=code)
    r = (code >> 8).astype(np.uint8)
    g = (code & 255).astype(np.uint8)

    depth = np.where(h < 0.0, -h, 0.0)
    np.clip(depth, 0.0, DEPTH_MAX, out=depth)
    b = np.rint(255.0 * np.sqrt(depth / DEPTH_MAX)).astype(np.int64)
    b = np.where(h < 0.0, np.maximum(b, 1), 0)
    return r, g, b.astype(np.uint8)


def decode(r, g, elev_hi):
    return (r.astype(np.float64) * 256.0 + g.astype(np.float64)) \
        * ((elev_hi - ELEV_LO) / 65535.0) + ELEV_LO


def main():
    with h5py.File(ETOPO, "r") as f:
        z = f["z"]
        assert z.shape == (ET_H, ET_W), z.shape
        lat = f["lat"]
        lon = f["lon"]
        assert abs(float(lat[0]) - ET_LAT0) < 1e-9, float(lat[0])
        assert abs(float(lon[0]) - ET_LON0) < 1e-9, float(lon[0])
        assert float(lat[-1]) > float(lat[0]), "ETOPO rows must run SOUTH -> NORTH"

        W1, H1 = 2400, 1018
        W2, H2 = 2 * W1, 2 * H1

        warped2, rt_err, off_globe = warp(z, W2, H2)

    # Average ELEVATION, not codes: the quantiser must see the final value once.
    warped1 = warped2.reshape(H1, 2, W1, 2).mean(axis=(1, 3))

    peak2, floor2 = float(warped2.max()), float(warped2.min())
    peak1, floor1 = float(warped1.max()), float(warped1.min())

    # Each image gets the ceiling ITS OWN array needs. Deriving one ceiling from whichever
    # array happens to be the largest would make `--2x` silently re-encode the shipped 1x
    # file against a constant the renderer does not use -- a flag that changes the committed
    # artifact is exactly the kind of quiet breakage this whole ladder exists to prevent.
    def ceiling_for(peak, what):
        c = next((c for c in ELEV_HI_LADDER if c >= peak), None)
        assert c is not None, f"{what} peak {peak:.2f} m exceeds every ladder ceiling"
        return c

    elev_hi = ceiling_for(peak1, "1x")
    assert warped1.max() <= elev_hi, f"{warped1.max()} > ELEV_HI {elev_hi}"
    # ELEV_LO deliberately clips the deep ocean out of the 16-bit field -- that constant
    # region is what the PNG filter collapses, and it is why this encode costs 2.76 MB
    # where an unclipped RG16 over the full range costs 3.47 MB. The floor is therefore
    # asserted against the LAND risk instead: no continental depression may be flattened.
    # The deepest sub-sea-level surface in the source is the Caspian floor at -815.8 m
    # (Dead Sea shore -427 m, Qattara -129 m, Turpan -154 m), so the ceiling on ELEV_LO
    # is -900; -1500 clears it by 684 m and still buys the shelf its own headroom.
    assert ELEV_LO <= -900.0, f"ELEV_LO {ELEV_LO} would flatten a real land depression"
    clipped_lo = int(np.count_nonzero(warped1 < ELEV_LO))
    assert abs(H_EXT - 1018.2) < 0.01, H_EXT

    r1, g1, b1 = encode(warped1, elev_hi)
    im1 = Image.fromarray(np.dstack([r1, g1, b1]))          # 3 uint8 planes -> mode "RGB"
    im1.save(OUT_1X, optimize=True, compress_level=9)
    size1 = os.path.getsize(OUT_1X)
    hygiene1 = png_chunk_hygiene(OUT_1X)

    size2, elev_hi2 = None, None
    if EMIT_2X:
        elev_hi2 = ceiling_for(peak2, "2x")
        r2, g2, b2 = encode(warped2, elev_hi2)
        im2 = Image.fromarray(np.dstack([r2, g2, b2]))
        im2.save(OUT_2X, optimize=True, compress_level=9)
        size2 = os.path.getsize(OUT_2X)
        png_chunk_hygiene(OUT_2X)

    # ---- report ----
    print(f"H_EXT = {H_EXT!r}   Y_TOP = {Y_TOP!r}   radius = {R!r}")
    print(f"WORLD.h = 1018.2 is a 1-dp rounding of H_EXT; rows use H_EXT (delta "
          f"{abs(H_EXT - 1018.2):.4f} canvas units)")
    print(f"inverse round-trip max |robinson_y(lat(y)) - y| over all {H2} 2x rows: "
          f"{rt_err:.3e} canvas px")
    print(f"off-globe texels at 2x (|lon| > 180, edge-clamped): {off_globe} / {W2 * H2} "
          f"= {off_globe / (W2 * H2):.5f}")
    print()
    print(f"warped peak: 2x array {peak2:.2f} m   1x (2x2 box) {peak1:.2f} m")
    print(f"warped floor: 2x array {floor2:.2f} m   1x (2x2 box) {floor1:.2f} m")
    print(f"ELEV_LO = {ELEV_LO}   ELEV_HI = {elev_hi} (derived from the shipped 1x array; "
          f"ladder {ELEV_HI_LADDER})")
    print(f"  assert warped1.max() {peak1:.2f} <= ELEV_HI {elev_hi}  -> OK "
          f"(headroom {elev_hi - peak1:.2f} m)")
    if elev_hi2 is not None:
        print(f"  the 2x preview gets its OWN ceiling {elev_hi2} for peak {peak2:.2f} m; "
              f"the shipped 1x encoding is unchanged by --2x")
    print(f"  assert ELEV_LO {ELEV_LO} <= -900.0 (deepest land depression, Caspian floor "
          f"-815.8 m)  -> OK")
    print(f"  texels clipped at ELEV_LO: {clipped_lo} / {warped1.size} = "
          f"{clipped_lo / warped1.size:.5f}, all ocean -- the depth channel carries them")
    print(f"  elevation step = {(elev_hi - ELEV_LO) / 65535.0:.6f} m/code")
    print(f"  SHADER DECODE: h = (R*256.0 + G) * {(elev_hi - ELEV_LO) / 65535.0!r} + "
          f"{ELEV_LO!r}")
    print(f"  SHADER DECODE: depth = {DEPTH_MAX!r} * pow(B/255.0, 2.0)")
    print()
    print(f"spheres-web/ui/relief.png : {W1} x {H1}  RGB8  {size1} bytes  "
          f"({size1 / (W1 * H1):.3f} B/px)   colour chunks: {hygiene1}")
    if size2 is not None:
        print(f"tools/terrain/relief_2x.png: {W2} x {H2}  RGB8  {size2} bytes (not committed)")
    else:
        print("tools/terrain/relief_2x.png: skipped (pass --2x to emit)")

    # ---- round-trip accuracy of the encode over the shipped image ----
    back = decode(r1, g1, elev_hi)
    land = warped1 >= 0.0
    err = np.abs(back - np.clip(warped1, ELEV_LO, elev_hi))
    print(f"encode round-trip max |err| over land: {err[land].max():.4f} m "
          f"(mean {err[land].mean():.4f} m)")

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
        print("5x5 decoded elevation m (rows py-2..py+2, cols px-2..px+2), depth m in ():")
        for rr in range(py - 2, py + 3):
            cells = []
            for cc in range(px - 2, px + 3):
                d = DEPTH_MAX * (b1[rr, cc] / 255.0) ** 2
                cells.append(f"{back[rr, cc]:8.1f}({d:6.0f})")
            print("   " + " ".join(cells))

    # ---- elevation spot-check: the numbers a reader can verify against an atlas ----
    print("\nelevation spot-check (forward-projected, decoded from the shipped PNG):")
    spots = [
        ("Everest summit cell", 86.925, 27.9917),
        ("Mont Blanc", 6.865, 45.833),
        ("Dead Sea", 35.50, 31.50),
        ("Death Valley", -116.83, 36.25),
        ("Amazon floodplain", -62.0, -3.0),
        ("Mariana Trench", 142.20, 11.35),
        ("Mid-Atlantic abyssal", -40.0, 30.0),
    ]
    for name, lon_d, lat_d in spots:
        x, y = project(lon_d, lat_d)
        px, py = int(math.floor(x)), int(math.floor(y / H_EXT * H1))
        d = DEPTH_MAX * (b1[py, px] / 255.0) ** 2
        print(f"  {name:22s} texel ({px:4d},{py:4d})  h = {back[py, px]:9.2f} m  "
              f"depth = {d:8.1f} m  (source cell {warped1[py, px]:9.2f} m)")


if __name__ == "__main__":
    main()
