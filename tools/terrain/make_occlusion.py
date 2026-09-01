#!/usr/bin/env python
# make_occlusion.py -- horizon-based SKY OCCLUSION, repacked into relief.png's B plane on land.
#
# Inputs -- the two SHIPPED PNGs, never the source rasters:
#   spheres-web/ui/relief.png   R,G -> elevation (this stage never writes them)
#                               B   -> ocean depth (preserved verbatim on water)
#   spheres-web/ui/coast.png    the land test, and nothing else
# Reading the shipped artifacts rather than etopo_60s.nc is what makes this stage
# re-runnable and provably reproducible on a checkout with no source rasters: it needs
# 3.0 MB of committed PNG, not 478 MB of netCDF.
#
# Output:
#   spheres-web/ui/relief.png   REWRITTEN IN PLACE. 2400 x 1018 RGB8, same R and G bytes,
#                               same B bytes on water, occlusion codes in B on land.
#
# ORDER: this runs AFTER make_relief.py and make_coast.py. make_relief.py OWNS relief.png
# and rewrites all three planes, so any re-bake of the heightmap WIPES this channel and this
# stage must be re-run behind it. That is the whole of the coupling; there is no other.
#
# ================================================================ WHAT IS BAKED
# Horizon-based sky view factor (Dozier/Frew). For each land texel the horizon angle is
# marched along 16 azimuths UNIFORM IN GROUND SPACE at 12 log-spaced ground distances from
# 8 km to 220 km, and
#     SVF = mean over azimuths of (1 - sin(horizon)),   occlusion = 1 - SVF
# is the fraction of the sky hemisphere the surrounding terrain blocks. It ships because a
# referee regression proved it is NOT reconstructible from the terms the shader already has
# for free: against a THREE-octave openness stack off the existing uPhys mip chain plus slope,
# slope^2, h and a cross term, R^2 = 0.6379 over all land -- and only 0.2105 restricted to
# the 13.67% of land where occ > 0.05, which is precisely the mountain terrain the channel
# exists to serve. The free stack is no better there than a single octave.
#
# THE AZIMUTHS ARE UNIFORM ON THE GROUND, NOT IN TEXEL SPACE, and the march is banded in
# latitude for exactly that reason. Robinson is strongly anisotropic: mx/my is 1.120 at the
# equator and 0.140 at 83N, so a texel-space 45-degree ray at 83N points 8 degrees off due
# north on the ground. Per band, the texel offset for ground azimuth theta at distance D is
#     dx = D * cos(theta) / mx,   dy = D * sin(theta) / my
# with mx, my from the shader's OWN metric() (index.html:5226-5230) evaluated on the
# H_EXT = 1018.1941195106424 row grid -- the same metric the hillshade differences against, so
# the bake and the runtime normal cannot disagree about what a metre is. 32 bands, each with a
# row halo of the full 220 km reach, edge-replicated at the poles so a wrapped row can never
# be marched against.
#
# Z = 4.20. Z here is an AMPLITUDE choice, not a geometric claim -- the same disclosure the
# PROCEDURAL micro-relief comment carries. At Z = 1.35 the same march yields p90 = 0.010 and
# is invisible; the bake is fixed at the far-out exaggeration so that occlusion is the one
# depth term that does NOT fall off as the camera dives.
#
# NOT A CAST SHADOW, on measurement rather than taste. The sun is fixed NW at 40 degrees
# (SUN = vec3(-0.541675, -0.541675, 0.642788), index.html:5148); the horizon TOWARD that
# azimuth is reported below and its maximum over all land is 0.140 against tan(40 deg) =
# 0.839, so at the shipped sun exactly zero texels on Earth shadow. Manufacturing coverage
# needs Z ~ 20, at which the shadow would claim darkness on slopes the Z = 4.2 normal in the
# same shader calls lit. The directional horizon is printed anyway, because it is the
# measurement that closes the question and because it proves the march is oriented correctly.
#
# ================================================================ THE ENCODE
#   occ_code = round(clamp(occ / 0.55, 0, 1) * 64)      written into B ON LAND ONLY
#   SHADER DECODE (land, sd > 0):  occ   = B * (0.55 / 64.0)
#   SHADER DECODE (water, sd <= 0): depth = 11000.0 * pow(B / 255.0, 2.0)   [unchanged]
# The two meanings are disambiguated by SIGN in the RGBA16F target, not by a threshold on the
# byte, so a shore-straddling bilinear tap reduces dep by at most 0.55 m and always downward
# (no false shallow->deep transition can be created), while occ clamps to exactly 0 at the
# shore. Occlusion self-fades at the coast, which is the same behaviour the existing openness
# term buys deliberately.
#
# 0.55 / 64 was chosen against 0.40 / 64 on measurement: it is ~20,700 B cheaper on this field
# and clips nothing (max occ over land is 0.377, so neither ceiling clips -- the byte argument
# is what survives). 64 codes puts the luminance step at (0.55/64) * 0.4291 = 0.369%, under the
# 1/255 = 0.392% display quantum, which HARD-CAPS uAO at 1.0: a later pass wanting uAO > 1.0
# must move to 88 codes and pay the measured delta printed below. That is a budgeted
# contingency, not a surprise.
#
# THE LAND TEST IS THE COAST BYTE, c >= 128, and nothing else. It is bit-exactly the shader's
# own sd > 0.0 (code 126 decodes to -0.0118, code 128 to +0.0039; 127 and 129 do not occur).
# Measured on the shipped pair: coast-land = 654,935 texels, B == 0 = 675,927, XOR = 29,936
# = 1.225% of the map. Keying off h >= 0 instead would corrupt those 29,936.
#
# THIS STAGE OVERWRITES 4,472 LAND TEXELS THAT CARRIED SUB-SEA-LEVEL DEPTH CODES -- the Dead
# Sea, the Caspian depression, the bed lakes -- up to code 83. That is safe (the land path
# never reads dep, and the sign trick bounds any mip bleed) but it is counted and printed
# rather than asserted away, because a design that ships on "this plane is empty" must be
# right about the plane being empty. It was not: it is empty on 650,463 of the 654,935.
#
# RE-RUNNABLE. The output is a pure function of (R, G, B-on-water, coast.png), none of which
# this stage writes, so running it on its own output reproduces that output byte for byte.
# The fresh-file overwrite count can only be taken from a fresh file, so the assertion is
# branched: on a fresh relief.png the 4,472 are asserted; on an already-repacked one the
# recomputed codes are asserted EQUAL to the input's land bytes, which is the stronger claim.
#
# Deterministic: float64 throughout, no RNG, no wall clock in the data path, fixed band and
# azimuth iteration order, fixed PNG compression (optimize=True, compress_level=9). The
# written file is scanned for gAMA/sRGB/iCCP and rejected if any is present -- a decoder that
# gamma-corrected R,G would destroy the packed uint16 elevation outright.
#
# Invocation:  python tools/terrain/make_occlusion.py
#   (prints the two-encode sha256 equality, the byte delta against a 140,000 B ceiling, the
#    land and overwrite counts, the occlusion distribution over land, the sub-quantum
#    fraction, the decode contract for GLBAKE, the directional sun-horizon proof over the
#    Himalaya / Andes / Alps, an occlusion spot-check over gorges, plateaus and plains, and
#    the wall clock.)

import hashlib
import io
import math
import os
import time

import numpy as np
from PIL import Image

Image.MAX_IMAGE_PIXELS = None

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
RELIEF = os.path.join(ROOT, "spheres-web/ui/relief.png")
COAST = os.path.join(ROOT, "spheres-web/ui/coast.png")

# ---- mapgen.rs constants, replicated exactly (lines 19-81 of mapgen.rs) ----
W = 2400.0
LAT_TOP = 83.0
LAT_BOT = -58.0
RX = [1.0000, 0.9986, 0.9954, 0.9900, 0.9822, 0.9730, 0.9600, 0.9427, 0.9216, 0.8962,
      0.8679, 0.8350, 0.7986, 0.7597, 0.7186, 0.6732, 0.6213, 0.5722, 0.5322]
RY = [0.0000, 0.0620, 0.1240, 0.1860, 0.2480, 0.3100, 0.3720, 0.4340, 0.4958, 0.5571,
      0.6176, 0.6769, 0.7346, 0.7903, 0.8435, 0.8936, 0.9394, 0.9761, 1.0000]

# ---- make_relief.py's encode contract, and GLBAKE's (index.html:4987-4993) ----
ELEV_LO = -1500.0
ELEV_HI = 6400.0
DEPTH_MAX = 11000.0

# ---- the shader's own constants (index.html:5148-5162) ----
EARTH = 6371008.8                 # IUGG mean radius, metres
DEG = math.pi / 180.0
SUN = (-0.541675, -0.541675, 0.642788)   # x=east, y=south, z=up

# ---- the march ----
Z = 4.20
N_AZ = 16
N_DIST = 12
D_MIN = 8000.0
D_MAX = 220000.0
N_BAND = 32

# ---- the encode ----
OCC_CLIP = 0.55
OCC_CODES = 64
DELTA_CEILING = 140_000           # bytes; FAIL above this
LAND_EXPECT = 654_935
OVERWRITE_EXPECT = 4_472
LUM_W = 0.4291                    # the sky share of the shipped lighting model


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
RX_ARR = np.asarray(RX, dtype=np.float64)
RY_DIFF = np.diff(RY_ARR)


def lat_from_canvas_y(y):
    """Vectorized exact inverse of robinson_y -- the shader's latFromY (index.html:5196)."""
    g = (Y_TOP - np.asarray(y, dtype=np.float64)) / (1.3523 * R)
    sign = np.where(g < 0.0, -1.0, 1.0)
    gg = np.minimum(np.abs(g), 1.0)
    i = np.clip(np.searchsorted(RY_ARR, gg, side="right") - 1, 0, 17)
    t = i + (gg - RY_ARR[i]) / RY_DIFF[i]
    return sign * 5.0 * t


def tab(lat_abs, table):
    """The shader's rxAt/ryAt (index.html:5204-5213), vectorized."""
    x = np.clip(np.abs(lat_abs) / 5.0, 0.0, 18.0)
    i = np.minimum(x.astype(np.int64), 17)
    return table[i] + (table[i + 1] - table[i]) * (x - i)


def metric(lat):
    """Metres per canvas unit east and south -- the shader's metric(), index.html:5226-5230.
    my is the CENTRAL DIFFERENCE of the RY table, because its analytic derivative is
    piecewise-constant and jumps 25% at 80 degrees."""
    a = np.abs(lat)
    mx = (EARTH * np.cos(lat * DEG)) / (0.8487 * R * tab(a, RX_ARR))
    a0 = np.maximum(a - 0.5, 0.0)
    a1 = np.minimum(a + 0.5, 90.0)
    my = (EARTH * DEG) / (1.3523 * R * (tab(a1, RY_ARR) - tab(a0, RY_ARR)) / (a1 - a0))
    return mx, my


def png_chunk_hygiene(path):
    blob = open(path, "rb").read()
    found = [c for c in (b"gAMA", b"sRGB", b"iCCP") if c in blob]
    assert not found, f"{os.path.basename(path)} carries colour chunks {found}"
    return "none (gAMA/sRGB/iCCP all absent)"


def encode_png(rgb):
    """Exactly make_relief.py's encoder settings, so a repack of the untouched planes is
    byte-identical to the file make_relief.py wrote."""
    buf = io.BytesIO()
    Image.fromarray(rgb).save(buf, "PNG", optimize=True, compress_level=9)
    return buf.getvalue()


def march(hL, mx, my):
    """Horizon-based occlusion, whole-raster shifted-max: N_AZ * N_DIST array ops, not
    H*W*N_AZ*N_DIST scalar ones. Returns (occ, sun_horizon_tangent)."""
    ht, wt = hL.shape
    az = np.arange(N_AZ, dtype=np.float64) * (2.0 * math.pi / N_AZ)
    dists = np.geomspace(D_MIN, D_MAX, N_DIST)

    # The sun's ground azimuth in the shader's (east, south) frame, snapped to the marched
    # set. atan2(south, east) of SUN's horizontal part: -135 deg == 225 deg == index 10.
    sun_az = math.atan2(SUN[1], SUN[0]) % (2.0 * math.pi)
    i_sun = int(round(sun_az / (2.0 * math.pi / N_AZ))) % N_AZ
    assert abs(az[i_sun] - sun_az) < 1e-12, (az[i_sun], sun_az)

    halo_max = int(math.ceil(D_MAX / my.min())) + 2
    hP = np.pad(hL, ((halo_max, halo_max), (0, 0)), mode="edge")

    bounds = np.linspace(0, ht, N_BAND + 1).astype(np.int64)
    occ = np.zeros((ht, wt), dtype=np.float64)
    sun_h = np.zeros((ht, wt), dtype=np.float64)
    for b in range(N_BAND):
        r0, r1 = int(bounds[b]), int(bounds[b + 1])
        mxb = float(mx[r0:r1].mean())
        myb = float(my[r0:r1].mean())
        halo = int(math.ceil(D_MAX / myb)) + 2
        sl = hP[r0 + halo_max - halo: r1 + halo_max + halo]
        tmax = np.zeros((N_AZ,) + sl.shape, dtype=np.float64)
        for D in dists:
            dx = D * np.cos(az) / mxb
            dy = D * np.sin(az) / myb
            for i in range(N_AZ):
                ix, iy = int(math.floor(dx[i])), int(math.floor(dy[i]))
                fx, fy = dx[i] - ix, dy[i] - iy
                s00 = np.roll(np.roll(sl, -iy, axis=0), -ix, axis=1)
                s10 = np.roll(s00, -1, axis=1)
                s01 = np.roll(s00, -1, axis=0)
                s11 = np.roll(s01, -1, axis=1)
                sh = ((1.0 - fy) * ((1.0 - fx) * s00 + fx * s10)
                      + fy * ((1.0 - fx) * s01 + fx * s11))
                np.maximum(tmax[i], Z * (sh - sl) / D, out=tmax[i])
        hor = np.arctan(np.maximum(tmax, 0.0))
        occ[r0:r1] = (1.0 - np.mean(1.0 - np.sin(hor), axis=0))[halo:halo + (r1 - r0)]
        sun_h[r0:r1] = np.maximum(tmax[i_sun], 0.0)[halo:halo + (r1 - r0)]
    return occ, sun_h, i_sun, az[i_sun]


def main():
    t_start = time.time()

    src_bytes = open(RELIEF, "rb").read()
    rgb = np.asarray(Image.open(RELIEF).convert("RGB")).copy()
    cst = np.asarray(Image.open(COAST).convert("L"))
    ht, wt = cst.shape
    assert rgb.shape == (ht, wt, 3), (rgb.shape, cst.shape)
    assert (ht, wt) == (1018, 2400), (ht, wt)

    # ---- decode, exactly as GLSL_MAP does ----
    h = (rgb[:, :, 0].astype(np.float64) * 256.0 + rgb[:, :, 1].astype(np.float64)) \
        * ((ELEV_HI - ELEV_LO) / 65535.0) + ELEV_LO
    land = cst >= 128
    hL = np.maximum(h, 0.0)          # the shader's own sea-level clamp, GLSL_MAP:5307-5308
    B_in = rgb[:, :, 2]

    n_land = int(land.sum())
    n_bzero = int((B_in == 0).sum())
    n_xor = int(np.count_nonzero(land ^ (B_in == 0)))
    n_overwrite = int(np.count_nonzero(land & (B_in > 0)))
    max_b_land = int(B_in[land].max())
    # The same count taken from the R,G pair this stage never writes. It reads 7 higher
    # because seven land texels whose true elevation was >= 0 quantise to a code that
    # decodes just below zero -- which is why the assertion below uses the raw B count on a
    # fresh file rather than this one.
    n_hneg = int(np.count_nonzero(land & (h < 0.0)))

    yc = (np.arange(ht, dtype=np.float64) + 0.5) * (H_EXT / ht)
    lat = lat_from_canvas_y(yc)
    rt_err = float(np.max(np.abs(
        np.array([Y_TOP - robinson_y(la) for la in lat], dtype=np.float64) - yc)))
    mx, my = metric(lat)

    t_march = time.time()
    occ, sun_h, i_sun, sun_az = march(hL, mx, my)
    t_march = time.time() - t_march

    # ---- encode: occlusion codes into B on land, depth preserved on water ----
    code = np.rint(np.clip(occ / OCC_CLIP, 0.0, 1.0) * OCC_CODES).astype(np.uint8)
    clip_frac = float((occ[land] > OCC_CLIP).mean())
    out = rgb.copy()
    out[:, :, 2] = np.where(land, code, B_in)

    blob_a = encode_png(out)
    blob_b = encode_png(out)          # a second independent encode of the same array
    sha_a = hashlib.sha256(blob_a).hexdigest()
    sha_b = hashlib.sha256(blob_b).hexdigest()
    assert sha_a == sha_b, "the PNG encoder is not deterministic"

    # Branch on WHAT THE INPUT IS, not on whether a count matched: an input whose land bytes
    # already equal the codes computed here can only be this stage's own output (4,472
    # non-zero land bytes against ~301,000 cannot collide), and it is the only input on which
    # the fresh-file overwrite count is unassertable.
    already = bool(np.array_equal(B_in[land], code[land]))
    fresh = not already
    assert n_land == LAND_EXPECT, f"land texel count moved: {n_land} (expect {LAND_EXPECT})"
    if fresh:
        assert n_overwrite == OVERWRITE_EXPECT, (
            f"overwrite count moved: {n_overwrite} land texels carry a non-zero B, expected "
            f"{OVERWRITE_EXPECT}. relief.png is neither freshly baked by make_relief.py nor "
            f"this stage's own output.")

    with open(RELIEF, "wb") as f:
        f.write(blob_a)
    size = os.path.getsize(RELIEF)
    delta = size - len(src_bytes)
    hygiene = png_chunk_hygiene(RELIEF)

    # ---- report ----
    print(f"H_EXT = {H_EXT!r}   Y_TOP = {Y_TOP!r}   radius = {R!r}")
    print(f"inverse round-trip max |robinson_y(lat(y)) - y| over all {ht} rows: "
          f"{rt_err:.3e} canvas px  (same row grid as make_relief.py and make_coast.py)")
    print(f"metric anisotropy mx/my: {mx[0] / my[0]:.3f} at row 0 ({lat[0]:.2f}N), "
          f"{mx[ht // 2] / my[ht // 2]:.3f} at the equator, "
          f"{mx[-1] / my[-1]:.3f} at row {ht - 1} ({lat[-1]:.2f}N) "
          f"-- why the march is banded in latitude")
    print()
    print(f"land test = coast.png >= 128 (bit-exactly the shader's sd > 0.0)")
    print(f"  land texels            {n_land}   (expect {LAND_EXPECT})")
    print(f"  B == 0                 {n_bzero}")
    print(f"  XOR(land, B == 0)      {n_xor}  = {100.0 * n_xor / (ht * wt):.3f}% of the map "
          f"-- what keying off h >= 0 would corrupt")
    if fresh:
        print(f"  OVERWRITTEN land texels carrying a sub-sea-level depth code: "
              f"{n_overwrite}  (expect {OVERWRITE_EXPECT}, max code {max_b_land})")
        print(f"    the land path never reads dep, so this is safe -- but the plane was NOT "
              f"empty; it was empty on {n_land - n_overwrite} of {n_land}")
        print(f"    the same count taken from R,G instead reads {n_hneg}: {n_hneg - n_overwrite}"
              f" land texels quantise from h >= 0 to a code that decodes below zero, and "
              f"{int(np.count_nonzero(land & (B_in > 0) & (h >= 0.0)))} go the other way")
    else:
        print(f"  input relief.png ALREADY carries this stage's repack: its {n_overwrite} "
              f"non-zero land bytes are bit-identical to the codes recomputed here, so the "
              f"output is byte-identical to the input. The {OVERWRITE_EXPECT} depth codes "
              f"were overwritten by the first run; that count is assertable only on a fresh "
              f"relief.png.")
    print()
    print(f"march: {N_AZ} azimuths uniform in GROUND space x {N_DIST} log distances "
          f"{D_MIN / 1000:.0f}..{D_MAX / 1000:.0f} km, bilinear, {N_BAND} latitude bands "
          f"with an edge-replicated {int(math.ceil(D_MAX / my.min())) + 2}-row halo")
    print(f"  {N_AZ * N_DIST} whole-raster shifted-max ops, not "
          f"{ht * wt * N_AZ * N_DIST:.3g} scalar ones   ({t_march:.2f} s) -- written as a "
          f"per-pixel loop this stage takes about an hour")
    print(f"  Z = {Z}. Z here is an AMPLITUDE choice, not a geometric claim -- the same "
          f"disclosure the PROCEDURAL micro-relief comment carries.")
    o = occ[land]
    print(f"  occlusion over land: p50 {np.percentile(o, 50):.4f}  p90 "
          f"{np.percentile(o, 90):.4f}  p99 {np.percentile(o, 99):.4f}  "
          f"p99.9 {np.percentile(o, 99.9):.4f}  max {o.max():.4f}")
    print(f"  as a luminance cut at uAO = 1.0: p90 {-100 * np.percentile(o, 90) * LUM_W:+.2f}%"
          f"  p99 {-100 * np.percentile(o, 99) * LUM_W:+.2f}%  "
          f"max {-100 * o.max() * LUM_W:+.2f}%")
    print(f"  land whose occ * {LUM_W} falls below the 1/255 = 0.392% display quantum: "
          f"{100 * float((o * LUM_W < 1 / 255).mean()):.2f}% -- literally unchanged, so flat "
          f"land still renders the authored hex and the Terrain legend holds")
    print()
    print(f"encode: occ_code = round(clamp(occ / {OCC_CLIP}, 0, 1) * {OCC_CODES}), B on land "
          f"only")
    print(f"  clipped: {100 * clip_frac:.4f}% of land (max occ {o.max():.4f} < {OCC_CLIP})")
    print(f"  distinct codes on land: {len(np.unique(code[land]))} of {OCC_CODES + 1}; "
          f"top code {int(code[land].max())}")
    print(f"  luminance step {100 * (OCC_CLIP / OCC_CODES) * LUM_W:.3f}% < the 0.392% display "
          f"quantum, which HARD-CAPS uAO at 1.0")
    print(f"  SHADER DECODE (land, sd > 0.0):  occ   = B * {OCC_CLIP / OCC_CODES!r}")
    print(f"  SHADER DECODE (water, sd <= 0.0): depth = {DEPTH_MAX!r} * pow(B/255.0, 2.0)"
          f"   [unchanged]")
    print()
    print(f"determinism: sha256 of two independent encodes of the same array")
    print(f"  {sha_a}")
    print(f"  {sha_b}   identical: {sha_a == sha_b}")
    if not fresh:
        same = blob_a == src_bytes
        print(f"  and byte-identical to the input file (the previous run's output): {same}")
        assert same, "re-running this stage on its own output changed the bytes"
    print()
    print(f"spheres-web/ui/relief.png : {wt} x {ht}  RGB8  {size} bytes  "
          f"({size / (wt * ht):.3f} B/px)   colour chunks: {hygiene}")
    print(f"  byte delta against the input: {delta:+d} B   ceiling {DELTA_CEILING:+d} B")
    assert delta <= DELTA_CEILING, (
        f"the repack costs {delta} B, over the {DELTA_CEILING} B ceiling. The delta depends "
        f"on the entropy of the field actually shipped, so it is measured, never promised.")

    # ---- the directional proof: the horizon toward the sun is oriented correctly ----
    # SUN's horizontal part is (west, north) = NW, elevation asin(0.642788) = 40 deg. A NW
    # sun throws shadow onto SE-FACING ground, so the sun-horizon must be high where the
    # terrain rises to the NW. Aspect from the same central differences the shader uses:
    # the normal is (-gx, -gy) in (east, south), so a slope faces the sun iff gx + gy > 0.
    gx = (np.roll(hL, -1, 1) - np.roll(hL, 1, 1)) / (2.0 * mx[:, None])
    gy = (np.roll(hL, -1, 0) - np.roll(hL, 1, 0)) / (2.0 * my[:, None])
    face_sun = (gx + gy) > 0.0
    tan_sun_elev = SUN[2] / math.hypot(SUN[0], SUN[1])
    print(f"\nsun-horizon proof. SUN = {SUN} -> ground azimuth "
          f"{math.degrees(sun_az):.1f} deg (NW), elevation "
          f"{math.degrees(math.asin(SUN[2])):.1f} deg, tan = {tan_sun_elev:.4f}")
    print(f"  marched azimuth index {i_sun} of {N_AZ} is exactly that bearing")
    print(f"  max horizon toward the sun over all {n_land} land texels: "
          f"{sun_h[land].max():.5f}  vs tan(elev) {tan_sun_elev:.4f}  -> "
          f"{int(np.count_nonzero(sun_h[land] > tan_sun_elev))} texels shadowed at the "
          f"shipped sun. THIS IS WHY NO CAST SHADOW SHIPS.")
    print(f"  the first single texel on Earth would shadow at Z = "
          f"{Z * tan_sun_elev / sun_h[land].max():.2f}")
    print("  directional check -- a NW sun must raise the horizon on SE-FACING ground:")
    ranges = [
        ("Himalaya", 78.0, 92.0, 27.0, 33.0),
        ("Andes (Peru/Bolivia)", -76.0, -66.0, -20.0, -8.0),
        ("Alps", 6.0, 14.0, 45.0, 48.0),
    ]
    for name, lo0, lo1, la0, la1 in ranges:
        x0, y1 = project(lo0, la1)
        x1, y0 = project(lo1, la0)
        c0, c1 = int(math.floor(x0)), int(math.ceil(x1))
        r0 = int(math.floor(y1 / H_EXT * ht))
        r1 = int(math.ceil(y0 / H_EXT * ht))
        win = np.zeros_like(land)
        win[r0:r1, c0:c1] = True
        win &= land
        se = win & ~face_sun          # faces away from the sun -> the shadow side
        nw = win & face_sun           # faces the sun -> the lit side
        m_se = float(sun_h[se].mean())
        m_nw = float(sun_h[nw].mean())
        print(f"    {name:22s} rows {r0:4d}-{r1:<4d} cols {c0:4d}-{c1:<4d}  n = {int(win.sum()):5d}"
              f"   SE-facing horizon {m_se:.4f}  NW-facing {m_nw:.4f}  ratio {m_se / m_nw:.2f}x")
        assert m_se > m_nw, f"{name}: the sun horizon is not higher on the shadow side"
    print("    every range shades to the SE of its crest, which is the correct side for a "
          "NW sun.")

    # ---- occlusion spot-check: deep ground dark, open ground clear ----
    # A canyon narrower than one texel cannot be read at its floor: 60 arc-second source
    # data warped to a 2400-wide canvas is 13-16 km per texel at these latitudes, and the
    # Grand Canyon is 16 km rim to rim. So each row also reports the LOWEST texel of the 5x5
    # window -- the gorge floor by construction, not by hand-picking -- beside the nominal
    # texel the forward projection lands on.
    print("\nocclusion spot-check (forward-projected; occ high = enclosed, occ ~0 = open).")
    print("  'floor' = the lowest-h texel of the 5x5 window, which is the canyon bottom "
          "wherever the feature is narrower than a texel.")
    print(f"  {'':24s} {'texel':13s} {'h m':>9s} {'occ':>7s} {'SVF':>7s} {'code':>5s}   "
          f"{'5x5 mean':>8s} {'5x5 max':>8s}   {'floor h':>8s} {'floor occ':>9s}")
    spots = [
        ("Grand Canyon", -112.10, 36.10),
        ("Yarlung Tsangpo gorge", 95.05, 29.60),
        ("Colca Canyon", -71.90, -15.62),
        ("Kali Gandaki gorge", 83.60, 28.75),
        ("Tibetan plateau", 88.00, 32.00),
        ("Altiplano", -67.50, -19.50),
        ("N European Plain", 18.00, 52.50),
        ("W Siberian Plain", 75.00, 60.00),
        ("Amazon floodplain", -62.00, -3.00),
        ("Sahara (Libya)", 24.00, 26.00),
        ("Everest massif", 86.93, 27.99),
        ("Mont Blanc", 6.87, 45.83),
    ]
    for name, lon_d, lat_d in spots:
        x, y = project(lon_d, lat_d)
        px, py = int(math.floor(x)), int(math.floor(y / H_EXT * ht))
        wo = occ[py - 2:py + 3, px - 2:px + 3]
        wh = h[py - 2:py + 3, px - 2:px + 3]
        fj, fi = np.unravel_index(int(np.argmin(wh)), wh.shape)
        print(f"  {name:24s} ({px:4d},{py:4d}) {h[py, px]:9.1f} {occ[py, px]:7.4f} "
              f"{1.0 - occ[py, px]:7.4f} {int(code[py, px]):5d}   {wo.mean():8.4f} "
              f"{wo.max():8.4f}   {wh[fj, fi]:8.1f} {wo[fj, fi]:9.4f}")
    print(f"\nwall clock: {time.time() - t_start:.2f} s")


if __name__ == "__main__":
    main()
