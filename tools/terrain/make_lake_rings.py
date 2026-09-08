#!/usr/bin/env python
# make_lake_rings.py — re-simplify the lake shorelines in ui/rivers.js, and
# nothing else in it.
#
# WHY THIS EXISTS RATHER THAN A FLAG ON make_rivers.py. The shoreline problem is
# a simplification problem: make_rivers.py runs Douglas-Peucker at 0.4 canvas
# units, which is 6,679 m of tolerance, chosen when this layer was drawn on a
# flat world map where that is sub-pixel. The camera now reaches about 9 m per
# pixel. Lake Michigan is the lake in the owner's own view and it arrived as 42
# vertices with a median segment of 37.8 km and a longest of 233.8 km — a
# ruler-straight shore several screen widths long, beside a 300,982-triangle
# 3D Chicago.
#
# The fix is a finer epsilon, and it CANNOT be had by re-running make_rivers.py,
# for a reason worth writing down. That script also emits
# spheres-web/data/river_segments.json, the raw geometry crossing_edges.py turns
# into river-crossed district adjacency — a SIM input. And the Natural Earth
# export now on disk is a later one than the file was baked from: filtered the
# same way it yields 281 drawable rivers against the committed 263. Re-running
# would therefore quietly change which rivers exist and what the simulation
# believes about which districts a river separates. That is a decision for the
# owner, not a side effect of sharpening a lakeshore.
#
# So this reads the committed rivers.js, keeps its `rivers:[...]` array BYTE FOR
# BYTE, and rewrites only `lakes:[...]`. It asserts that the lake set it derives
# is the same 29 lakes in the same order as the file it is replacing, because
# spheres-web/ui/terrain-surface.js binds six water-surface heights to lakes BY
# INDEX into that array, and tools/ui/check_terrain_surface.cjs pins each of
# them by the sha256 of its path string.
#
# INPUT   tools/terrain/raster/ne/ne_10m_lakes.{shp,dbf}   (untracked; see
#         .gitignore — Natural Earth source data stays out of the repo)
# OUTPUT  spheres-web/ui/rivers.js, rewritten in place
#
# Deterministic: no RNG, no clock, stable sort by (name, raw ring length, input
# index) exactly as make_rivers.py sorts, and the ring length is computed on the
# RAW lon/lat ring so the ordering cannot move when the epsilon does.
#
# Invocation:  python tools/terrain/make_lake_rings.py [--eps 0.012] [--check]

import argparse
import math
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(HERE))
sys.path.insert(0, HERE)
import shapefile                                            # noqa: E402

SRC = os.path.join(HERE, "raster", "ne", "ne_10m_lakes")
OUT_JS = os.path.join(ROOT, "spheres-web/ui/rivers.js")

# --- projection: the same mapgen.rs replica make_rivers.py carries ----------
W = 2400.0
LAT_TOP, LAT_BOT = 83.0, -58.0
RX = [1, .9986, .9954, .99, .9822, .973, .96, .9427, .9216, .8962, .8679, .835,
      .7986, .7597, .7186, .6732, .6213, .5722, .5322]
RY = [0, .062, .124, .186, .248, .31, .372, .434, .4958, .5571, .6176, .6769,
      .7346, .7903, .8435, .8936, .9394, .9761, 1.0]
M_PER_UNIT = 40075017.0 / W          # metres per canvas unit at the equator


def interp(table, a):
    a = min(90.0, max(0.0, a))
    i = min(int(a / 5.0), 17)
    t = (a - i * 5.0) / 5.0
    return table[i] * (1 - t) + table[i + 1] * t


def radius():
    return W / (2 * math.pi * 0.8487)


def robinson_y(lat):
    return 1.3523 * radius() * interp(RY, abs(lat)) * (-1 if lat < 0 else 1)


def project(lon, lat):
    lat = max(LAT_BOT, min(LAT_TOP, lat))
    x = W / 2.0 + 0.8487 * radius() * interp(RX, abs(lat)) * math.radians(lon)
    return (x, robinson_y(LAT_TOP) - robinson_y(lat))


def dp_simplify(pts, eps):
    """Douglas-Peucker, iterative, eps in canvas units — make_rivers.py's."""
    n = len(pts)
    if n < 3:
        return list(pts)
    keep = [False] * n
    keep[0] = keep[n - 1] = True
    stack = [(0, n - 1)]
    while stack:
        a, b = stack.pop()
        ax, ay = pts[a]
        bx, by = pts[b]
        dx, dy = bx - ax, by - ay
        den = dx * dx + dy * dy
        best, bi = -1.0, -1
        for i in range(a + 1, b):
            px, py = pts[i]
            if den == 0.0:
                d = math.hypot(px - ax, py - ay)
            else:
                t = max(0.0, min(1.0, ((px - ax) * dx + (py - ay) * dy) / den))
                d = math.hypot(px - (ax + t * dx), py - (ay + t * dy))
            if d > best:
                best, bi = d, i
        if best > eps and bi > 0:
            keep[bi] = True
            stack.append((a, bi))
            stack.append((bi, b))
    return [pts[i] for i in range(n) if keep[i]]


def fmt2(v):
    s = "%.2f" % v
    if "." in s:
        s = s.rstrip("0").rstrip(".")
    return "0" if s == "-0" else s


def project_ring(ring, eps):
    proj = [project(lon, lat) for lon, lat in ring]
    out = []
    for x, y in dp_simplify(proj, eps):
        key = (fmt2(x), fmt2(y))
        if not out or key != out[-1]:
            out.append(key)
    return out


def path_closed(rings):
    """make_rivers.py's path_closed, verbatim in behaviour."""
    chunks = []
    for pts in rings:
        if len(pts) >= 2 and pts[0] == pts[-1]:
            pts = pts[:-1]
        if len(pts) < 3:
            continue
        chunks.append("M" + pts[0][0] + " " + pts[0][1]
                      + "L" + " ".join(x + " " + y for x, y in pts[1:]) + "Z")
    return "".join(chunks)


def polyline_length(pts):
    return sum(math.hypot(pts[i + 1][0] - pts[i][0], pts[i + 1][1] - pts[i][1])
               for i in range(len(pts) - 1))


def signed_area(ring):
    s = 0.0
    for i in range(len(ring)):
        x1, y1 = ring[i]
        x2, y2 = ring[(i + 1) % len(ring)]
        s += x1 * y2 - x2 * y1
    return 0.5 * s


def exterior_rings(parts):
    """Shapefile polygons mark holes by winding: an OUTER ring is clockwise,
    which is a negative shoelace area with the y-axis pointing north. GeoJSON
    instead nests holes inside their polygon, which is how make_rivers.py could
    write `poly[0]`. Same intent, different encoding — take the outer rings."""
    return [r for r in parts if len(r) >= 4 and signed_area(r) < 0]


LAKE_SR_MAX = 1


def build(eps):
    lakes = []
    for idx, rec in enumerate(shapefile.read(SRC)):
        sr = rec.get("scalerank")
        if sr is None or sr > LAKE_SR_MAX:
            continue
        rings = exterior_rings(rec["parts"])
        if not rings:
            continue
        name = rec.get("name_en") or rec.get("name") or ""
        lakes.append({"name": name, "rings": rings,
                      "len": sum(polyline_length(r) for r in rings), "idx": idx})
    # make_rivers.py's ordering, and it is load-bearing: terrain-surface.js
    # addresses lakes by their position here. The length is measured on the RAW
    # ring, so the order cannot move when the epsilon does.
    lakes.sort(key=lambda l: (l["name"], l["len"], l["idx"]))
    return [{"name": l["name"], "path": path_closed([project_ring(r, eps) for r in l["rings"]])}
            for l in lakes]


def js_str(s):
    return '"' + s.replace("\\", "\\\\").replace('"', '\\"') + '"'


def current_lakes(src):
    m = re.search(r",lakes:\[(.*)\]\};", src, re.S)
    if not m:
        raise SystemExit("rivers.js: could not find the lakes array")
    return m, re.findall(r'"((?:[^"\\]|\\.)*)"', m.group(1))


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--eps", type=float, default=0.012,
                    help="Douglas-Peucker tolerance in canvas units "
                         "(default 0.012 = 200 m; make_rivers.py uses 0.4 = 6,679 m)")
    ap.add_argument("--check", action="store_true",
                    help="report and change nothing")
    args = ap.parse_args()

    src = open(OUT_JS, encoding="utf-8").read()
    match, before = current_lakes(src)
    built = build(args.eps)

    print("lakes: %d before, %d after" % (len(before), len(built)))
    if len(built) != len(before):
        raise SystemExit("REFUSED: the lake set changed size; terrain-surface.js "
                         "binds water heights to these by index")
    v_before = sum(p.count(" ") for p in before)
    v_after = sum(p["path"].count(" ") for p in built)
    print("vertices: %d before, %d after (%.1fx) at eps %.4f canvas units = %.0f m"
          % (v_before, v_after, v_after / max(v_before, 1), args.eps, args.eps * M_PER_UNIT))

    # EVERY INDEX MUST STILL BE THE SAME LAKE. Counting them is not enough:
    # spheres-web/ui/terrain-surface.js maps six positions in this array to
    # water-surface heights, so a reorder would apply Lake Michigan's level to
    # Lake Baikal and nothing in the emit would notice. Compare where each one
    # sits on the canvas, before against after.
    def centre(path):
        nums = [float(v) for v in re.findall(r"-?\d+(?:\.\d+)?", path)]
        xs, ys = nums[0::2], nums[1::2]
        return (sum(xs) / len(xs), sum(ys) / len(ys),
                max(xs) - min(xs), max(ys) - min(ys))

    moved = []
    for i, b in enumerate(built):
        if not b["path"] or not before[i]:
            continue
        ax, ay, aw, ah = centre(before[i])
        bx, by, bw, bh = centre(b["path"])
        # A finer ring moves its own centroid a little; a DIFFERENT lake moves it
        # by its own size. Half the feature's extent is the line between those.
        tol = max(1.0, 0.5 * max(aw, ah))
        if math.hypot(bx - ax, by - ay) > tol:
            moved.append("%d (%s): centroid moved %.1f canvas units, tolerance %.1f"
                         % (i, b["name"], math.hypot(bx - ax, by - ay), tol))
        if b["path"] and len(b["path"]) < len(before[i]) * 0.9:
            moved.append("%d (%s): the path got SHORTER, which a finer epsilon cannot do"
                         % (i, b["name"]))
    if moved:
        raise SystemExit("REFUSED: the lake array has reordered or changed identity:\n  "
                         + "\n  ".join(moved))
    print("identity: all %d lakes still at their own index" % len(built))
    if args.check:
        return

    payload = ",".join(js_str(b["path"]) for b in built if b["path"])
    out = src[:match.start(1)] + payload + src[match.end(1):]
    open(OUT_JS, "w", encoding="utf-8", newline="\n").write(out)
    print("wrote %s (%d bytes, was %d)" % (OUT_JS, len(out.encode()), len(src.encode())))


if __name__ == "__main__":
    main()
