#!/usr/bin/env python
"""classify_districts.py — per-district terrain classification for Spheres.

INPUTS (read-only, paths anchored on the repo root this file sits in):
  spheres-web/data/ne_10m_admin_1.geojson
      admin-1 source polygons (lowercase property keys in this export)
  spheres-web/data/ne_10m_geography_regions_polys.geojson
      Natural Earth physiographic label polygons (UPPERCASE property keys)
  spheres-web/ui/index.html
      `const TERRITORY = {...}` — nation -> ISO3 roster, parsed with the same
      algorithm as mapgen.rs::territory_map (never a copy)
  spheres-sim/data/districts.json
      authoritative game district roster (2,610 unique ids) — used to validate
      that the id derivation below reproduces mapgen exactly

OUTPUT:
  spheres-web/data/district_terrain.json
      { districtId: {"t": class, "f": name|null} }, sorted keys, UTF-8.
      Class histogram printed to stdout. Committed; mapgen merges it into
      spheres-sim/data/districts.json (the sim never reads this file itself).
  spheres-web/ui/terrain.js
      The same classification baked for the browser, rivers.js-style:
      `window.TERRAIN={byId:{districtId:{"t":class[,"f":name]}}}`, sorted
      keys, null features omitted. Served by main.rs (/terrain.js) and read
      by ui/index.html for Terrain-mode fills and hover text — scenery only;
      the sim reads its own merged copy in districts.json.

INVOCATION:
  python tools/terrain/classify_districts.py
      (no arguments, no RNG — output is byte-stable)

DISTRICT ID DERIVATION — replicated exactly from
C:/Users/ridge/Spheres/spheres-web/src/bin/mapgen.rs (district_pass):
  * features bucketed by adm0_a3 (skip "", "-99", "ATA", and countries not in
    the TERRITORY roster), sorted by adm1_code
  * AGGREGATE countries (AZE ESP FRA GBR HUN IRL ITA LKA LVA MKD MLT PHL SVN
    THA UGA VNM): group by NE `region`; null-region features go to the nearest
    region mean centroid in PROJECTED px (Robinson, W=2400, LAT 83..-58,
    replicated below) with ties broken by region name ascending; id =
    ADM0_slug(region) with -2,-3.. suffixes in region-name order
  * everything else: id = iso_3166_2 when clean (^[A-Z]{2}-[A-Z0-9]{1,3}$) AND
    unique within the country, else ADM0_slug(name), suffixed on collision in
    adm1_code order

TERRAIN TAXONOMY (from the regions probe):
  mountain <- Range/mtn, Gorge          highland <- Plateau, Foothills
  desert   <- Desert, Depression        tundra   <- Tundra (+ lat>=66N fallback
  wetland  <- Wetlands, Delta                       for untagged points)
  lowland  <- Plain, Lowland, Valley, Basin, Coast, + default
  Locational classes (Island, Island group, Geoarea, Pen/cape, Continent,
  Peninsula, Isthmus, Lake, Dragons-be-here) are excluded from voting — the
  probe's caveats list Isthmus as locational (its only large member, CENTRAL
  AMERICA, would otherwise name 94 districts).
  Data-quirk blocklist (documented in the probe): SELVAS (Amazon rainforest
  mislabeled Foothills) is fully excluded; PENÍNSULA IBÉRICA (whole-peninsula
  blob tagged Plateau) keeps its class votes but is suppressed as a name.

SAMPLING (deterministic):
  For each district, an axis-aligned lattice of cell-center points over the
  lon/lat bbox of its (member) polygons, aspect-balanced (nx*ny ~= n^2, nx/ny
  ~ bbox aspect). Budget ladder n = 7,10,14,20,28,40,56,80,112: stop at the
  first rung yielding >= 25 interior points, or >= 5 interior points once
  n >= 40 (small districts); if the whole ladder yields none, the shapely
  representative_point() is the single sample. Interior test: any member
  polygon covers the point.

CLASSIFICATION:
  Per sample point: the highest-precedence class among covering physiographic
  polygons, precedence mountain > highland > desert > wetland > tundra >
  lowland; untagged points default to tundra when lat >= 66N else lowland.
  District class = majority of point classes; TIE RULE: the tied class that
  comes first in the precedence order wins.
  LATITUDE-BAND OVERRIDE (documented data-quirk fix): after the majority vote,
  any district the majority of whose land sits at or above 66N and whose
  class is not mountain or desert becomes tundra. Rationale: Natural Earth's
  tagged plain/plateau polygons (e.g. the Western Siberian Plain over
  RU-YAN Yamal-Nenets) outvote the >=66N tundra default on Arctic districts,
  classifying polar-circle ground as ordinary lowland. The override restores
  the band the default already encodes for untagged points; mountain and
  desert survive it because those landforms are accurate whatever the
  latitude. The band test is pure latitude geometry, so it is measured on the
  ladder's densest rung (n=112) rather than the class vote's adaptive budget:
  the adaptive rung under-samples a district whose polar half is peninsulas
  (RU-YAN reads 13/27 arctic at its class rung but 0.589 at n=112, and its
  land majority genuinely is polar). Every flipped district is printed.
  Feature name (two stages, both requiring 10*count >= 3*n_samples, i.e.
  >= 30% of samples covered): first the best-covering physio feature whose
  class equals the district class; if none qualifies, the best-covering physio
  feature of ANY class — so a Zagros district whose majority samples are
  untagged still carries "Zagros Mountains". Ties: higher count, then NAME
  ascending, then feature index ascending. Names ALL-CAPS in the source are
  title-cased ("ZAGROS MOUNTAINS" -> "Zagros Mountains"); mixed-case names
  pass through unchanged.
"""

import json
import math
import os
import sys

import numpy as np
import shapely
from shapely.geometry import Polygon
from shapely.strtree import STRtree
from shapely.validation import make_valid

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
ADMIN1 = os.path.join(ROOT, "spheres-web/data/ne_10m_admin_1.geojson")
REGIONS = os.path.join(ROOT, "spheres-web/data/ne_10m_geography_regions_polys.geojson")
INDEX_HTML = os.path.join(ROOT, "spheres-web/ui/index.html")
DISTRICTS_JSON = os.path.join(ROOT, "spheres-sim/data/districts.json")
OUT = os.path.join(ROOT, "spheres-web/data/district_terrain.json")
UI_OUT = os.path.join(ROOT, "spheres-web/ui/terrain.js")

# --------------------------------------------------------------------------
# mapgen.rs replication: projection (only used for AGGREGATE centroid ties)
# --------------------------------------------------------------------------
W = 2400.0
LAT_TOP, LAT_BOT = 83.0, -58.0
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


def shoelace(pts):
    """Centroid + signed area, identical to mapgen.rs::shoelace."""
    area = 0.0
    cx = cy = 0.0
    n = len(pts)
    for i in range(n):
        xa, ya = pts[i]
        xb, yb = pts[(i + 1) % n]
        cross = xa * yb - xb * ya
        area += cross
        cx += (xa + xb) * cross
        cy += (ya + yb) * cross
    area *= 0.5
    if abs(area) < 1e-9:
        x0 = min(p[0] for p in pts); x1 = max(p[0] for p in pts)
        y0 = min(p[1] for p in pts); y1 = max(p[1] for p in pts)
        return ((x0 + x1) / 2.0, (y0 + y1) / 2.0, 0.0)
    return (cx / (6.0 * area), cy / (6.0 * area), area)


# --------------------------------------------------------------------------
# mapgen.rs replication: identity helpers
# --------------------------------------------------------------------------
AGGREGATE = {"AZE", "ESP", "FRA", "GBR", "HUN", "IRL", "ITA", "LKA", "LVA",
             "MKD", "MLT", "PHL", "SVN", "THA", "UGA", "VNM"}


def slug(s):
    out = []
    for c in s.lower():
        if c.isascii() and c.isalnum():
            out.append(c)
        elif out and out[-1] != "-":
            out.append("-")
    return "".join(out).rstrip("-")


def clean_iso_3166_2(s):
    b = s.encode("utf-8", "surrogateescape")
    if len(b) < 4 or len(b) > 6:
        return False
    return (chr(b[0]).isupper() and chr(b[0]).isascii() and
            chr(b[1]).isupper() and chr(b[1]).isascii() and
            b[2] == ord("-") and
            all(chr(c).isascii() and (chr(c).isupper() or chr(c).isdigit()) for c in b[3:]))


def territory_map(html):
    """Replicates mapgen.rs::territory_map."""
    body = html.split("const TERRITORY = {", 1)[1].split("};", 1)[0]
    clean = "\n".join(line.split("//")[0] for line in body.splitlines())
    chars = clean
    out = {}
    i, n = 0, len(chars)
    while i < n:
        c = chars[i]
        if (c.isascii() and c.isalnum()) or c == "_":
            start = i
            while i < n and ((chars[i].isascii() and chars[i].isalnum()) or chars[i] == "_"):
                i += 1
            ident = chars[start:i]
            j = i
            while j < n and chars[j].isspace():
                j += 1
            if j < n and chars[j] == ":":
                k = j + 1
                while k < n and chars[k].isspace():
                    k += 1
                if k < n and chars[k] == "[":
                    end = k + 1
                    while end < n and chars[end] != "]":
                        end += 1
                    lst = chars[k + 1:end]
                    codes = lst.split('"')[1::2]
                    out[ident] = codes
                    i = end + 1
                    continue
        i += 1
    return out


def geometry_polys(geom):
    """polygons -> rings -> lon/lat points, replicating mapgen.rs."""
    t = geom.get("type")
    if t == "Polygon":
        polys = [geom["coordinates"]]
    elif t == "MultiPolygon":
        polys = list(geom.get("coordinates") or [])
    else:
        polys = []
    out = []
    for poly in polys:
        p = []
        for ring in poly:
            pts = [(float(c[0]), float(c[1])) for c in ring if len(c) >= 2]
            if len(pts) >= 3:
                p.append(pts)
        if p:
            out.append(p)
    return out


def centroid_px(polys):
    """Replicates Adm1Feature::centroid_px: projected shoelace centroid of the
    largest-|area| ring over ALL rings (holes included), first max wins."""
    best = None  # ((cx,cy), abs_area)
    for poly in polys:
        for ring in poly:
            proj = [project(lo, la) for (lo, la) in ring]
            cx, cy, a = shoelace(proj)
            if best is None or abs(a) > best[1]:
                best = ((cx, cy), abs(a))
    return best[0] if best else (0.0, 0.0)


def derive_districts():
    """Returns {district_id: [feature polys lists]} exactly per mapgen ids."""
    with open(INDEX_HTML, encoding="utf-8") as f:
        territory = territory_map(f.read())
    assert len(territory) >= 150, "TERRITORY parse too small"
    roster = {code for codes in territory.values() for code in codes}

    with open(ADMIN1, encoding="utf-8") as f:
        gj = json.load(f)

    by_adm0 = {}
    for feat in gj["features"]:
        props = feat.get("properties") or {}
        adm0 = props.get("adm0_a3") or props.get("ADM0_A3") or ""
        if adm0 in ("", "-99", "ATA"):
            continue
        if adm0 not in roster:
            continue
        polys = geometry_polys(feat.get("geometry") or {})
        if not polys:
            continue
        adm1_code = props.get("adm1_code") or ""
        iso = props.get("iso_3166_2")
        name = props.get("name") or props.get("name_en") or adm1_code
        region = props.get("region") or None
        by_adm0.setdefault(adm0, []).append({
            "adm1_code": adm1_code, "iso": iso, "name": name,
            "region": region, "polys": polys,
        })
    for feats in by_adm0.values():
        feats.sort(key=lambda f: f["adm1_code"])

    districts = {}  # id -> list of polys (each: list of rings)
    for adm0 in sorted(by_adm0):
        feats = by_adm0[adm0]
        if adm0 in AGGREGATE:
            groups = {}
            for i, f in enumerate(feats):
                if f["region"] is not None:
                    groups.setdefault(f["region"], []).append(i)
            assert groups, f"{adm0} in AGGREGATE but no region fields"
            means = {}
            for r in sorted(groups):
                idxs = groups[r]
                sx = sy = 0.0
                for i in idxs:
                    x, y = centroid_px(feats[i]["polys"])
                    sx += x; sy += y
                means[r] = (sx / len(idxs), sy / len(idxs))
            for i, f in enumerate(feats):
                if f["region"] is not None:
                    continue
                c = centroid_px(f["polys"])
                best = None  # (region, d2) — strict <, regions in sorted order
                for r in sorted(means):
                    m = means[r]
                    d = (c[0] - m[0]) ** 2 + (c[1] - m[1]) ** 2
                    if best is None or d < best[1]:
                        best = (r, d)
                groups[best[0]].append(i)
            used = {}
            for region in sorted(groups):
                idxs = sorted(groups[region])
                base = f"{adm0}_{slug(region)}"
                n = used.get(base, 0) + 1
                used[base] = n
                did = base if n == 1 else f"{base}-{n}"
                member_polys = []
                for i in idxs:
                    member_polys.extend(feats[i]["polys"])
                districts[did] = member_polys
        else:
            iso_count = {}
            for f in feats:
                iso = f["iso"]
                if iso is not None and clean_iso_3166_2(iso):
                    iso_count[iso] = iso_count.get(iso, 0) + 1
            used = {}
            for f in feats:
                iso = f["iso"]
                if iso is not None and clean_iso_3166_2(iso) and iso_count[iso] == 1:
                    base = iso
                else:
                    s = slug(f["name"])
                    base = f"{adm0}_{slug(f['adm1_code'])}" if not s else f"{adm0}_{s}"
                n = used.get(base, 0) + 1
                used[base] = n
                did = base if n == 1 else f"{base}-{n}"
                districts[did] = list(f["polys"])
    return districts


# --------------------------------------------------------------------------
# Terrain taxonomy
# --------------------------------------------------------------------------
CLASS_OF = {
    "Range/mtn": "mountain", "Gorge": "mountain",
    "Plateau": "highland", "Foothills": "highland",
    "Desert": "desert", "Depression": "desert",
    "Tundra": "tundra",
    "Wetlands": "wetland", "Delta": "wetland",
    "Plain": "lowland", "Lowland": "lowland", "Valley": "lowland",
    "Basin": "lowland", "Coast": "lowland",
}
PRECEDENCE = {"mountain": 0, "highland": 1, "desert": 2, "wetland": 3,
              "tundra": 4, "lowland": 5}
SMALL_WORDS = {"of", "the", "and", "de"}
# NE data quirks (see module docstring): fully excluded / name-suppressed.
EXCLUDE_NAMES = {"SELVAS"}
NAME_SUPPRESS = {"PENÍNSULA IBÉRICA"}


def pretty_name(name):
    if not name.isupper():
        return name
    words = name.lower().split(" ")
    out = []
    for wi, w in enumerate(words):
        if wi > 0 and w in SMALL_WORDS:
            out.append(w)
            continue
        parts = w.split("-")
        out.append("-".join(p[:1].upper() + p[1:] if p else p for p in parts))
    return " ".join(out)


def load_regions():
    """Physio features only: parallel lists of geoms, classes, names."""
    with open(REGIONS, encoding="utf-8") as f:
        gj = json.load(f)
    geoms, classes, names = [], [], []
    for feat in gj["features"]:
        props = feat.get("properties") or {}
        cla = props.get("FEATURECLA") or props.get("featurecla") or ""
        cls = CLASS_OF.get(cla)
        if cls is None:
            continue
        if (props.get("NAME") or "") in EXCLUDE_NAMES:
            continue
        polys = geometry_polys(feat.get("geometry") or {})
        if not polys:
            continue
        parts = []
        for poly in polys:
            g = make_valid(Polygon(poly[0], poly[1:]))
            if not g.is_empty:
                parts.append(g)
        if not parts:
            continue
        geom = shapely.union_all(parts) if len(parts) > 1 else parts[0]
        name = props.get("NAME") or props.get("name") or None
        geoms.append(geom)
        classes.append(cls)
        names.append(name)
    return geoms, classes, names


# --------------------------------------------------------------------------
# Deterministic interior sampling
# --------------------------------------------------------------------------
LADDER = [7, 10, 14, 20, 28, 40, 56, 80, 112]


def district_parts(polys):
    parts = []
    for poly in polys:
        try:
            g = make_valid(Polygon(poly[0], poly[1:]))
        except Exception:
            continue
        if not g.is_empty:
            parts.append(g)
    return parts


def lattice(minx, miny, maxx, maxy, n):
    w = maxx - minx
    h = maxy - miny
    if w <= 0.0 or h <= 0.0:
        return np.array([(minx + maxx) / 2.0]), np.array([(miny + maxy) / 2.0])
    aspect = math.sqrt(w / h)
    nx = max(1, int(round(n * aspect)))
    ny = max(1, int(round(n / aspect)))
    xs = minx + (np.arange(nx) + 0.5) * (w / nx)
    ys = miny + (np.arange(ny) + 0.5) * (h / ny)
    gx, gy = np.meshgrid(xs, ys)
    return gx.ravel(), gy.ravel()


def polar_majority(parts, fallback_lats):
    """True when the majority of the district's land sits at or above 66N.

    Measured on the ladder's densest rung (n=112) — see the module docstring's
    override notes for why the class vote's adaptive rung is not reused here.
    `fallback_lats` (the class vote's own sample latitudes) answers for the
    rare sliver the dense lattice misses entirely.
    """
    maxy = max(p.bounds[3] for p in parts)
    if maxy < 66.0:
        return False
    minx = min(p.bounds[0] for p in parts)
    miny = min(p.bounds[1] for p in parts)
    maxx = max(p.bounds[2] for p in parts)
    gx, gy = lattice(minx, miny, maxx, maxy, LADDER[-1])
    pts = shapely.points(gx, gy)
    inside = np.zeros(len(pts), dtype=bool)
    for p in parts:
        inside |= shapely.covers(p, pts)
    la = gy[inside] if inside.any() else np.asarray(fallback_lats)
    return 2 * int((la >= 66.0).sum()) > len(la)


def sample_points(parts):
    """Interior lon/lat sample points per the documented ladder rule."""
    minx = min(p.bounds[0] for p in parts)
    miny = min(p.bounds[1] for p in parts)
    maxx = max(p.bounds[2] for p in parts)
    maxy = max(p.bounds[3] for p in parts)
    best = None
    for n in LADDER:
        gx, gy = lattice(minx, miny, maxx, maxy, n)
        pts = shapely.points(gx, gy)
        inside = np.zeros(len(pts), dtype=bool)
        for part in parts:
            inside |= shapely.covers(part, pts)
        k = int(inside.sum())
        if k:
            best = (gx[inside], gy[inside])
        if k >= 25 or (n >= 40 and k >= 5):
            return best
    if best is not None:
        return best
    rp = shapely.union_all(parts).representative_point()
    return (np.array([rp.x]), np.array([rp.y]))


# --------------------------------------------------------------------------
# Main
# --------------------------------------------------------------------------
def main():
    districts = derive_districts()

    with open(DISTRICTS_JSON, encoding="utf-8") as f:
        roster = json.load(f)
    game_ids = {d["id"] for ds in roster["nations"].values() for d in ds}
    derived_ids = set(districts)
    missing = sorted(game_ids - derived_ids)
    extra = sorted(derived_ids - game_ids)
    if missing:
        print(f"WARNING: {len(missing)} game ids not derived: {missing[:20]}",
              file=sys.stderr)
    if extra:
        print(f"note: {len(extra)} derived ids not in game roster (skipped): "
              f"{extra[:10]}...", file=sys.stderr)

    geoms, classes, names = load_regions()
    tree = STRtree(geoms)

    out = {}
    hist = {}
    named = 0
    flips = []  # (district id, pre-override class) moved by the 66N band
    for did in sorted(districts):
        if did not in game_ids:
            continue
        parts = district_parts(districts[did])
        if not parts:
            out[did] = {"t": "lowland", "f": None}
            hist["lowland"] = hist.get("lowland", 0) + 1
            continue
        gx, gy = sample_points(parts)
        pts = shapely.points(gx, gy)
        n_samples = len(pts)
        qi, ti = tree.query(pts, predicate="covered_by")
        hits_per_point = [[] for _ in range(n_samples)]
        for a, b in zip(qi.tolist(), ti.tolist()):
            hits_per_point[a].append(b)
        cls_votes = {}
        feat_pts = {}  # region feature idx -> #sample points covered
        for i in range(n_samples):
            hits = hits_per_point[i]
            if hits:
                cls = min((classes[b] for b in hits), key=lambda c: PRECEDENCE[c])
            else:
                cls = "tundra" if gy[i] >= 66.0 else "lowland"
            cls_votes[cls] = cls_votes.get(cls, 0) + 1
            for b in hits:
                feat_pts[b] = feat_pts.get(b, 0) + 1
        # majority; ties broken by precedence order
        winner = min(cls_votes, key=lambda c: (-cls_votes[c], PRECEDENCE[c]))
        # latitude-band override (see module docstring): a district the
        # majority of whose land sits at or above 66N is tundra unless the
        # landform vote says mountain or desert. Runs before the feature-name
        # stages so a flipped district names its feature under the class it
        # ships with.
        if winner not in ("mountain", "desert", "tundra") and polar_majority(parts, gy):
            flips.append((did, winner))
            winner = "tundra"
        # dominant feature name, >= 30% of samples: same-class first, any-class
        # fallback (see module docstring)
        fname = None
        for stage in (lambda b: classes[b] == winner, lambda b: True):
            cands = sorted((-cnt, names[b], b)
                           for b, cnt in feat_pts.items()
                           if stage(b) and names[b]
                           and names[b] not in NAME_SUPPRESS)
            if cands and 10 * (-cands[0][0]) >= 3 * n_samples:
                fname = pretty_name(cands[0][1])
                break
        out[did] = {"t": winner, "f": fname}
        hist[winner] = hist.get(winner, 0) + 1
        if fname:
            named += 1

    with open(OUT, "w", encoding="utf-8", newline="\n") as f:
        json.dump(out, f, ensure_ascii=False, sort_keys=True, indent=1)
        f.write("\n")

    # The browser's copy of the same classification, baked like rivers.js.
    # Null features are omitted (the UI treats a missing "f" as unnamed), and
    # the payload is compact JSON — valid JS, deterministic via sorted keys.
    baked = {did: ({"t": e["t"], "f": e["f"]} if e["f"] else {"t": e["t"]})
             for did, e in out.items()}
    with open(UI_OUT, "w", encoding="utf-8", newline="\n") as f:
        f.write("// Generated by tools/terrain/classify_districts.py. "
                "Source: Natural Earth 10m\n")
        f.write("// physiographic regions voted over admin-1 districts "
                "(public domain). Same ids\n")
        f.write("// as districts.js; scenery and hover text only — the sim "
                "reads its own merged\n")
        f.write("// copy in districts.json. Do not hand-edit.\n")
        f.write("window.TERRAIN={byId:")
        json.dump(baked, f, ensure_ascii=False, sort_keys=True,
                  separators=(",", ":"))
        f.write("};\n")

    print(f"wrote {OUT}: {len(out)} districts")
    print(f"wrote {UI_OUT}: {len(baked)} districts baked for the UI")
    print(f"66N latitude-band override flipped {len(flips)} district(s) to tundra:")
    for did, was in flips:
        print(f"  {did}: {was} -> tundra")
    print("class histogram:")
    for cls in sorted(hist, key=lambda c: PRECEDENCE[c]):
        print(f"  {cls:9s} {hist[cls]:5d}")
    print(f"named features: {named} districts ({100.0 * named / len(out):.1f}%)")
    print(f"id check: {len(game_ids)} game ids, {len(missing)} missing, "
          f"{len(extra)} extra-derived")


if __name__ == "__main__":
    main()
