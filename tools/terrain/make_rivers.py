#!/usr/bin/env python
# make_rivers.py — river + lake map layer for the Spheres Robinson canvas.
#
# Inputs (read-only, anchored on the repo root this file sits in):
#   spheres-web/data/ne_10m_rivers_lake_centerlines.geojson
#   spheres-web/data/ne_10m_lakes.geojson
#
# Projection: exact replica of spheres-web/src/bin/mapgen.rs
#   W=2400.0, LAT_TOP=83.0, LAT_BOT=-58.0, Robinson RX/RY 19-entry tables,
#   radius() = W / (2 * 0.8487 * pi), linear interp at 5-degree steps,
#   project(lon, lat) -> (W/2 + 0.8487*R*interp(RX,|lat|)*radians(lon),
#                         robinson_y(83) - robinson_y(lat)), lat clamped to [-58, 83].
#
# Filters (from the hydro probe):
#   rivers: scalerank <= 5.0, BOTH featurecla values ("River" + "Lake Centerline")
#           -> 264 match, 263 emitted: the Loire (sr 5.0) has an EMPTY
#           MultiLineString in this export and is skipped as undrawable.
#           Keeps all marquee rivers (Rhine, Tigris rank 4.0).
#   lakes:  scalerank <= 1 -> 29 majors (Caspian is absent from ne_10m_lakes by
#           Natural Earth convention; it is not this layer's job)
#
# Simplification: Douglas-Peucker, eps = 0.4 canvas px, applied AFTER projection.
# Lake paths use exterior rings only (island holes dropped at this scale).
#
# Outputs (deterministic — no RNG, stable sort by (name, projected length, input index)):
#   spheres-web/ui/rivers.js          the committed, baked UI layer:
#       window.RIVERS={meta:{w,h},rivers:[{n:name|null,d:"M..L.."}],lakes:["M..L..Z"]}
#       SVG path strings, 2-decimal canvas coords, served by main.rs::/rivers.js
#   spheres-web/data/river_segments.json
#       same filtered rivers, RAW lon/lat parts (unsimplified), 3-decimal —
#       the input crossing_edges.py computes river-crossed adjacency from
#
# Invocation:  python tools/terrain/make_rivers.py   (no arguments)

import json
import math
import os

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
DATA = os.path.join(ROOT, "spheres-web/data")
OUT_JS = os.path.join(ROOT, "spheres-web/ui/rivers.js")
OUT_SEGMENTS = os.path.join(DATA, "river_segments.json")

# --- projection: exact mapgen.rs replica -----------------------------------
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


def project(lon, lat):
    lat = max(LAT_BOT, min(LAT_TOP, lat))
    x = W / 2.0 + 0.8487 * radius() * interp(RX, abs(lat)) * math.radians(lon)
    y = robinson_y(LAT_TOP) - robinson_y(lat)
    return (x, y)


# --- Douglas-Peucker (iterative, eps in canvas px) -------------------------
DP_EPS = 0.4


def dp_simplify(pts, eps):
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
        seg2 = dx * dx + dy * dy
        best, best_i = -1.0, -1
        for i in range(a + 1, b):
            px, py = pts[i]
            if seg2 <= 0.0:
                d2 = (px - ax) ** 2 + (py - ay) ** 2
            else:
                t = ((px - ax) * dx + (py - ay) * dy) / seg2
                t = max(0.0, min(1.0, t))
                qx, qy = ax + t * dx, ay + t * dy
                d2 = (px - qx) ** 2 + (py - qy) ** 2
            if d2 > best:
                best, best_i = d2, i
        if best > eps * eps:
            keep[best_i] = True
            stack.append((a, best_i))
            stack.append((best_i, b))
    return [pts[i] for i in range(n) if keep[i]]


# --- helpers ---------------------------------------------------------------
def fmt2(v):
    s = f"{v:.2f}"
    if "." in s:
        s = s.rstrip("0").rstrip(".")
    return "0" if s == "-0" else s


def split_antimeridian(part):
    """Split a lon/lat polyline where consecutive lons jump > 180 degrees."""
    out, cur = [], [part[0]]
    for prev, pt in zip(part, part[1:]):
        if abs(pt[0] - prev[0]) > 180.0:
            if len(cur) >= 2:
                out.append(cur)
            cur = [pt]
        else:
            cur.append(pt)
    if len(cur) >= 2:
        out.append(cur)
    return out


def project_part(part):
    """lon/lat part -> projected, deduped (post-rounding), DP-simplified pts."""
    proj = [project(lon, lat) for lon, lat in part]
    simp = dp_simplify(proj, DP_EPS)
    out = []
    for x, y in simp:
        key = (fmt2(x), fmt2(y))
        if not out or key != out[-1]:
            out.append(key)
    return out


def polyline_length(part):
    """Projected length of one lon/lat part, canvas px (pre-simplification)."""
    proj = [project(lon, lat) for lon, lat in part]
    return sum(math.hypot(b[0] - a[0], b[1] - a[1]) for a, b in zip(proj, proj[1:]))


def path_open(parts):
    """SVG path for projected open polylines: M x y L x y ... per part."""
    chunks = []
    for pts in parts:
        if len(pts) < 2:
            continue
        chunks.append("M" + pts[0][0] + " " + pts[0][1]
                      + "L" + " ".join(x + " " + y for x, y in pts[1:]))
    return "".join(chunks)


def path_closed(rings):
    """SVG path for projected rings: M x y L ... Z per ring."""
    chunks = []
    for pts in rings:
        if len(pts) >= 2 and pts[0] == pts[-1]:
            pts = pts[:-1]
        if len(pts) < 3:
            continue
        chunks.append("M" + pts[0][0] + " " + pts[0][1]
                      + "L" + " ".join(x + " " + y for x, y in pts[1:]) + "Z")
    return "".join(chunks)


def feat_name(props):
    return props.get("name_en") or props.get("name") or ""


def load(fname):
    with open(os.path.join(DATA, fname), encoding="utf-8") as f:
        return json.load(f)["features"]


# --- rivers ----------------------------------------------------------------
RIVER_SR_MAX = 5.0
LAKE_SR_MAX = 1

rivers = []
for idx, feat in enumerate(load("ne_10m_rivers_lake_centerlines.geojson")):
    props = feat["properties"]
    sr = props.get("scalerank")
    if sr is None or sr > RIVER_SR_MAX:
        continue
    geom = feat["geometry"]
    raw_parts = geom["coordinates"] if geom["type"] == "MultiLineString" else [geom["coordinates"]]
    parts = []
    for part in raw_parts:
        ll = [(p[0], p[1]) for p in part]
        if len(ll) >= 2:
            parts.extend(split_antimeridian(ll))
    if not parts:
        continue
    total_len = sum(polyline_length(p) for p in parts)
    rivers.append({
        "name": feat_name(props),
        "sr": sr,
        "featurecla": props["featurecla"],
        "parts": parts,
        "len": total_len,
        "idx": idx,
    })

rivers.sort(key=lambda r: (r["name"], r["len"], r["idx"]))

rivers_out = []
segments_out = []
for r in rivers:
    proj_parts = [project_part(p) for p in r["parts"]]
    rivers_out.append({"name": r["name"], "sr": r["sr"], "path": path_open(proj_parts)})
    segments_out.append({
        "name": r["name"],
        "sr": r["sr"],
        "featurecla": r["featurecla"],
        "parts": [[[round(lon, 3), round(lat, 3)] for lon, lat in p] for p in r["parts"]],
    })

# --- lakes -----------------------------------------------------------------
lakes = []
for idx, feat in enumerate(load("ne_10m_lakes.geojson")):
    props = feat["properties"]
    sr = props.get("scalerank")
    if sr is None or sr > LAKE_SR_MAX:
        continue
    geom = feat["geometry"]
    polys = geom["coordinates"] if geom["type"] == "MultiPolygon" else [geom["coordinates"]]
    # exterior rings only — island holes are sub-pixel noise at this scale
    rings = [[(p[0], p[1]) for p in poly[0]] for poly in polys if poly and len(poly[0]) >= 4]
    if not rings:
        continue
    total_len = sum(polyline_length(rg) for rg in rings)
    lakes.append({"name": feat_name(props), "rings": rings, "len": total_len, "idx": idx})

lakes.sort(key=lambda l: (l["name"], l["len"], l["idx"]))

lakes_out = []
for l in lakes:
    proj_rings = [project_part(rg) for rg in l["rings"]]
    lakes_out.append({"name": l["name"], "path": path_closed(proj_rings)})

# --- write -----------------------------------------------------------------
def canvas_height():
    return robinson_y(LAT_TOP) - robinson_y(LAT_BOT)


def js_str(s):
    return json.dumps(s, ensure_ascii=False)


river_chunks = []
for r in rivers_out:
    if not r["path"]:
        continue  # undrawable (empty geometry in the source export)
    name = r["name"] or None
    if name is not None:
        # keeps the emit trivially parseable for check.py's key-quoting pass
        assert ":" not in name and '"' not in name, name
    river_chunks.append(
        "{n:%s,d:%s}" % ("null" if name is None else js_str(name), js_str(r["path"]))
    )
lake_chunks = [js_str(l["path"]) for l in lakes_out if l["path"]]

js = (
    "// Generated by tools/terrain/make_rivers.py. "
    "Source: Natural Earth 10m rivers/lakes (public domain).\n"
    "// Robinson, clipped to 83N..58S, same canvas as world.js. Do not hand-edit.\n"
    "window.RIVERS={meta:{w:%d,h:%.1f},rivers:[%s],lakes:[%s]};\n"
    % (int(W), canvas_height(), ",".join(river_chunks), ",".join(lake_chunks))
)
with open(OUT_JS, "w", encoding="utf-8", newline="\n") as f:
    f.write(js)

with open(OUT_SEGMENTS, "w", encoding="utf-8", newline="\n") as f:
    json.dump({"rivers": segments_out}, f, ensure_ascii=False, separators=(",", ":"))

print(f"rivers kept: {len(river_chunks)} (scalerank <= {RIVER_SR_MAX}, both featurecla)")
print(f"lakes kept:  {len(lake_chunks)} (scalerank <= {LAKE_SR_MAX})")
for path in (OUT_JS, OUT_SEGMENTS):
    print(f"{os.path.relpath(path, ROOT)}: {os.path.getsize(path)} bytes")
for want in ("Rhine", "Tigris", "Euphrates", "Yangtze", "Mississippi", "Volga",
             "Nile", "Amazon", "Danube", "Mekong"):
    hits = [r for r in rivers_out if want.lower() in r["name"].lower()]
    pts = sum(sum(seg.count("M") + seg.count(" ") // 2 for seg in [r["path"]]) for r in hits)
    print(f"spotcheck {want}: {len(hits)} feature(s) "
          + (f"e.g. name={hits[0]['name']!r} sr={hits[0]['sr']}" if hits else "MISSING"))
