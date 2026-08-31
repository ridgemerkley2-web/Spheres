#!/usr/bin/env python
"""decode_ghspop.py — stage A of the district population pass.

Decodes the GHS-POP 1990 raster into a flat float32 memmap plus a small JSON
georeference sidecar, so stage B (`make_population.py`) can slice it by
bounding box without holding 3.7 GB of float64 in memory.

Why a hand-rolled decoder: the source is a **BigTIFF** (magic 43, not 42),
float64, 256x256 LZW-tiled, 43202x21384 — and this environment has no GDAL and
no rasterio. Pillow will not open a BigTIFF. The 120 lines below read the one
IFD, walk the tile offset table in index order, LZW-decode each tile with
`imagecodecs`, and write it to its fixed destination. Nothing about the result
depends on visitation order, so it is byte-stable by construction.

INPUT (untracked staging data, re-downloadable — tools/terrain/README's
convention for `etopo_60s.nc` and `NE1_50M_SR_W.zip`, followed here):
  spheres-web/data/GHS_POP_E1990_GLOBE_R2023A_4326_30ss_V1_0.tif
      GHS-POP R2023A, epoch 1990, GLOBE, EPSG:4326, 30 arc-second.
      European Commission Joint Research Centre. CC BY 4.0.
      https://jeodpp.jrc.ec.europa.eu/ftp/jrc-opendata/GHSL/
        GHS_POP_GLOBE_R2023A/GHS_POP_E1990_GLOBE_R2023A_4326_30ss/V1-0/
        GHS_POP_E1990_GLOBE_R2023A_4326_30ss_V1_0.zip   (443 MB zip, 364 MB tif)
      sha256 of the tif: 31002afc325652c7ea5b825069759689334dc282f2c136a3a038d94bc9061af2

OUTPUT (scratch cache, gitignored — `tools/population/raster/`, mirroring
`tools/terrain/raster/`):
  raster/ghspop1990.f32   height*width float32 little-endian, row-major (3.7 GB)
  raster/ghspop1990.json  {width,height,x0,y0,sx,sy,crs,grid_total_float64,
                           sha256_source}

INVOCATION (from the repo root):
  python tools/population/decode_ghspop.py
  python tools/population/decode_ghspop.py <in.tif> <out_prefix>   # explicit

Determinism: no RNG, no wall clock, no dict-order dependence. Tiles are visited
in index order; each writes to a fixed destination slice. The float64 -> float32
narrowing is round-to-nearest-even in numpy and is applied identically every run.
"""
import hashlib
import json
import os
import struct
import sys

import numpy as np
import imagecodecs

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
HERE = os.path.dirname(os.path.abspath(__file__))
DEFAULT_TIF = os.path.join(
    ROOT, "spheres-web/data/GHS_POP_E1990_GLOBE_R2023A_4326_30ss_V1_0.tif")
DEFAULT_PREFIX = os.path.join(HERE, "raster/ghspop1990")

# TIFF type code -> bytes per element.
TYPESZ = {1: 1, 2: 1, 3: 2, 4: 4, 5: 8, 6: 1, 7: 1, 8: 2, 9: 4,
          10: 8, 11: 4, 12: 8, 16: 8, 17: 8, 18: 8}


def read_ifd(path):
    """Open a little-endian BigTIFF and return (handle, {tag: (type, count, bytes)})."""
    f = open(path, "rb")
    hdr = f.read(16)
    assert hdr[:2] == b"II", "expected little-endian TIFF"
    magic = struct.unpack("<H", hdr[2:4])[0]
    assert magic == 43, f"expected BigTIFF (43), got magic {magic}"
    ifd = struct.unpack("<Q", hdr[8:16])[0]
    f.seek(ifd)
    n = struct.unpack("<Q", f.read(8))[0]
    raw = f.read(n * 20)
    tags = {}
    for i in range(n):
        e = raw[i * 20:(i + 1) * 20]
        tag, typ, cnt = struct.unpack("<HHQ", e[:12])
        nbytes = TYPESZ.get(typ, 1) * cnt
        if nbytes <= 8:
            data = e[12:12 + nbytes]
        else:
            off = struct.unpack("<Q", e[12:20])[0]
            keep = f.tell()
            f.seek(off)
            data = f.read(nbytes)
            f.seek(keep)
        tags[tag] = (typ, cnt, data)
    return f, tags


def scalar(tags, tag, fmt="<H"):
    return struct.unpack(fmt, tags[tag][2][:struct.calcsize(fmt)])[0]


def main(tif=DEFAULT_TIF, prefix=DEFAULT_PREFIX):
    if not os.path.exists(tif):
        sys.exit(f"missing staging raster: {tif}\n"
                 "See the module docstring for the JRC download URL.")
    os.makedirs(os.path.dirname(prefix), exist_ok=True)

    f, tags = read_ifd(tif)
    width = scalar(tags, 256)
    height = scalar(tags, 257)
    bits = scalar(tags, 258)
    comp = scalar(tags, 259)
    tw = scalar(tags, 322)
    th = scalar(tags, 323)
    sample_fmt = scalar(tags, 339)
    assert bits == 64 and sample_fmt == 3, \
        f"expected float64 samples, got bits={bits} fmt={sample_fmt}"
    assert comp == 5, f"expected LZW compression (5), got {comp}"
    assert 317 not in tags or scalar(tags, 317) == 1, \
        "TIFF predictor must be none; a horizontal predictor would need undoing"

    scale = np.frombuffer(tags[33550][2], dtype="<f8")   # ModelPixelScale
    tie = np.frombuffer(tags[33922][2], dtype="<f8")     # ModelTiepoint
    sx, sy = float(scale[0]), float(scale[1])
    # tiepoint maps raster (i,j,k) -> model (x,y,z). RasterPixelIsArea, so the
    # model point is the OUTER corner of pixel (i,j), not its centre.
    x0 = float(tie[3]) - float(tie[0]) * sx
    y0 = float(tie[4]) + float(tie[1]) * sy

    offs = np.frombuffer(tags[324][2], dtype="<u8")      # TileOffsets
    cnts = np.frombuffer(tags[325][2], dtype="<u4")      # TileByteCounts
    tx = (width + tw - 1) // tw
    ty = (height + th - 1) // th
    assert tx * ty == len(offs), f"tile grid {tx}x{ty} != {len(offs)} offsets"

    out = np.memmap(prefix + ".f32", dtype="<f4", mode="w+", shape=(height, width))
    tile_bytes = tw * th * 8
    total = 0.0
    for k in range(len(offs)):
        f.seek(int(offs[k]))
        blob = f.read(int(cnts[k]))
        raw = imagecodecs.lzw_decode(blob)
        assert len(raw) == tile_bytes, f"tile {k}: {len(raw)} bytes != {tile_bytes}"
        tile = np.frombuffer(raw, dtype="<f8").reshape(th, tw)
        r, c = divmod(k, tx)
        y1, x1 = min((r + 1) * th, height), min((c + 1) * tw, width)
        sub = tile[: y1 - r * th, : x1 - c * tw]
        out[r * th:y1, c * tw:x1] = sub.astype("<f4")
        total += float(sub.sum())
    out.flush()
    del out
    f.close()

    meta = {
        "source_file": os.path.basename(tif),
        "sha256_source": hashlib.sha256(open(tif, "rb").read()).hexdigest(),
        "width": width, "height": height,
        "x0": x0, "y0": y0, "sx": sx, "sy": sy,
        "crs": "EPSG:4326", "pixel_is_area": True,
        "grid_total_float64": total,
    }
    with open(prefix + ".json", "w", encoding="utf-8", newline="\n") as g:
        json.dump(meta, g, indent=2, sort_keys=True)
        g.write("\n")
    print(json.dumps(meta, indent=2, sort_keys=True))


if __name__ == "__main__":
    a = sys.argv[1:]
    main(a[0] if len(a) > 0 else DEFAULT_TIF,
         a[1] if len(a) > 1 else DEFAULT_PREFIX)
