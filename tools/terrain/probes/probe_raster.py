#!/usr/bin/env python
"""Probe SR_50M shaded-relief raster and replicate mapgen.rs projection constants.

Inputs:
  C:/Users/ridge/Spheres/spheres-web/data/SR_50M.zip  (Natural Earth 50M shaded relief, WGS84 geographic)
  C:/Users/ridge/Spheres/spheres-web/src/bin/mapgen.rs (projection constants, read manually; replicated below)

Invocation:
  python probe_raster.py

Outputs:
  Extracts the zip into ../raster/ (sibling of scripts/), prints a deterministic
  report: zip contents, world-file geotransform, TIF dimensions/mode/value range
  (PIL), and the exact mapgen canvas size a warped underlay should target.
Deterministic: no RNG; value-range sample uses a fixed stride grid.
"""
import os, sys, zipfile, math

STAGE = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
RASTER_DIR = os.path.join(STAGE, "raster")
ZIP_PATH = r"C:/Users/ridge/Spheres/spheres-web/data/SR_50M.zip"

# ---- mapgen.rs constants, replicated EXACTLY (mapgen.rs lines 19-81) ----
W = 2400.0
LAT_TOP = 83.0
LAT_BOT = -58.0
RX = [1.0000, 0.9986, 0.9954, 0.9900, 0.9822, 0.9730, 0.9600, 0.9427, 0.9216,
      0.8962, 0.8679, 0.8350, 0.7986, 0.7597, 0.7186, 0.6732, 0.6213, 0.5722, 0.5322]
RY = [0.0000, 0.0620, 0.1240, 0.1860, 0.2480, 0.3100, 0.3720, 0.4340, 0.4958,
      0.5571, 0.6176, 0.6769, 0.7346, 0.7903, 0.8435, 0.8936, 0.9394, 0.9761, 1.0000]

def interp(table, lat_abs):
    t = min(lat_abs / 5.0, 18.0)
    i = int(math.floor(t))
    if i >= 18:
        return table[18]
    return table[i] + (t - i) * (table[i + 1] - table[i])

def radius():
    return W / (2.0 * 0.8487 * math.pi)

def robinson_y(lat):
    return 1.3523 * radius() * interp(RY, abs(lat)) * (-1.0 if lat < 0.0 else 1.0)

def height():
    return robinson_y(LAT_TOP) - robinson_y(LAT_BOT)

def project(lon, lat):
    lat = max(LAT_BOT, min(LAT_TOP, lat))
    x = W / 2.0 + 0.8487 * radius() * interp(RX, abs(lat)) * math.radians(lon)
    y = robinson_y(LAT_TOP) - robinson_y(lat)
    return (x, y)

def main():
    os.makedirs(RASTER_DIR, exist_ok=True)
    print("=== ZIP CONTENTS ===")
    with zipfile.ZipFile(ZIP_PATH) as z:
        for info in sorted(z.infolist(), key=lambda i: i.filename):
            print(f"  {info.filename}  {info.file_size} bytes")
        z.extractall(RASTER_DIR)
    print(f"extracted to: {RASTER_DIR}")

    # world file(s)
    print("=== WORLD FILE(S) ===")
    tif_path = None
    for root, _dirs, files in os.walk(RASTER_DIR):
        for f in sorted(files):
            p = os.path.join(root, f)
            low = f.lower()
            if low.endswith((".tfw", ".wld", ".tifw")):
                with open(p) as fh:
                    vals = [line.strip() for line in fh if line.strip()]
                print(f"  {f}: {vals}")
                if len(vals) == 6:
                    a, d, b, e, c, fv = (float(v) for v in vals)
                    print(f"    pixel size x (A) = {a}")
                    print(f"    rotation (D,B)   = {d}, {b}")
                    print(f"    pixel size y (E) = {e}")
                    print(f"    upper-left center x (C) = {c}")
                    print(f"    upper-left center y (F) = {fv}")
            if low.endswith((".tif", ".tiff")):
                tif_path = p

    print("=== TIF PROBE (PIL) ===")
    try:
        from PIL import Image
    except ImportError:
        print("  PIL NOT AVAILABLE")
        return
    Image.MAX_IMAGE_PIXELS = None
    try:
        im = Image.open(tif_path)
        print(f"  path: {tif_path}")
        print(f"  size: {im.size[0]} x {im.size[1]}")
        print(f"  mode: {im.mode}")
        print(f"  format: {im.format}, compression: {im.info.get('compression')}")
        # fixed-stride sample grid for min/max (deterministic)
        wpx, hpx = im.size
        px = im.load()
        mn, mx = 255, 0
        for yy in range(0, hpx, 97):
            for xx in range(0, wpx, 97):
                v = px[xx, yy]
                if isinstance(v, tuple):
                    v = v[0]
                mn = min(mn, v); mx = max(mx, v)
        print(f"  sampled value range (stride 97): min={mn} max={mx}")
        # corner + center pixels
        for name, (xx, yy) in [("UL", (0, 0)), ("UR", (wpx - 1, 0)),
                               ("center", (wpx // 2, hpx // 2)),
                               ("LL", (0, hpx - 1)), ("LR", (wpx - 1, hpx - 1))]:
            print(f"  pixel {name} = {px[xx, yy]}")
        print("  PIL read OK (no external tiff plugin needed)")
    except Exception as ex:
        print(f"  PIL FAILED: {type(ex).__name__}: {ex}")

    print("=== MAPGEN CANVAS (python replica) ===")
    print(f"  W = {W}")
    print(f"  LAT_TOP = {LAT_TOP}, LAT_BOT = {LAT_BOT}")
    print(f"  radius() = {radius()!r}")
    print(f"  robinson_y(LAT_TOP) = {robinson_y(LAT_TOP)!r}")
    print(f"  robinson_y(LAT_BOT) = {robinson_y(LAT_BOT)!r}")
    h = height()
    print(f"  height() = {h!r}")
    print(f"  height rounded 1dp (as written to world.js '{{h:{{:.1f}}}}') = {h:.1f}")
    print(f"  target canvas: {int(W)} x {h:.1f} (px)")
    print(f"  project(-180, LAT_TOP) = {project(-180.0, LAT_TOP)}")
    print(f"  project(180, LAT_TOP)  = {project(180.0, LAT_TOP)}")
    print(f"  project(0, 0)          = {project(0.0, 0.0)}")
    print(f"  project(-180, LAT_BOT) = {project(-180.0, LAT_BOT)}")
    print(f"  project(180, LAT_BOT)  = {project(180.0, LAT_BOT)}")

if __name__ == "__main__":
    main()
