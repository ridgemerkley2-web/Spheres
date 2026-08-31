#!/usr/bin/env python3
"""
tools/resources/geo.py — point/polygon -> district id, and the nation crosswalk.

Shared plumbing for the resource transcription tools. Two jobs:

  1. DISTRICT LOOKUP. `DistrictIndex` answers "which game district contains this
     lat/lon?".  The district geometry is NOT re-derived here — it is imported
     wholesale from `tools/terrain/classify_districts.py::derive_districts`,
     which replicates mapgen.rs's identity rules exactly (the AGGREGATE set, the
     ISO-3166-2 uniqueness test, the slug fallback and the `-2` suffixing).  That
     is the same function that produced the committed terrain classification, so
     a resource and a terrain class attached to the same id refer to the same
     ground by construction.  Reusing it is deliberate: there is exactly one
     definition of "district AF-BAL" in this repo and it lives over there.

     Matching happens in WGS84 lon/lat, not in projected canvas space.  The
     Robinson projection is imported and re-exported for callers that need it,
     but point-in-polygon is done on the source geometry because it is
     full-precision, whereas `spheres-web/ui/districts.js` is rounded to 0.1px
     (~1.5 km) and simplified — a coastal mine could fall outside its own
     district's drawn outline.  The projection is a display transform; identity
     is geographic.

  2. NATION CROSSWALK. Every source names countries differently and none of them
     use the game's 1990-era CamelCase roster (`USSR`, `Zaire`, `UAE`,
     `Czechoslovakia`).  `NationCrosswalk` is hand-authored, one entry per source
     vocabulary, and it FAILS LOUDLY on an unmapped name that carries data rather
     than silently dropping it.  A silent drop here reads as "this country has no
     resources", which is a fabrication; the probe's naive matcher lost 49
     nations including the USA and the USSR that way.

Determinism: no RNG, no wall clock, no set iteration that reaches output.  The
index is built by scanning districts in sorted id order and every lookup returns
a single id chosen by a documented, total rule (see `locate`).
"""

import json
import math
import os
import sys

import shapely
from shapely.geometry import Polygon, shape
from shapely.ops import unary_union
from shapely.strtree import STRtree

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
TERRAIN = os.path.join(ROOT, "tools", "terrain")
if TERRAIN not in sys.path:
    sys.path.insert(0, TERRAIN)

# The one definition of district identity and of the map projection.  Imported,
# never reimplemented — see tools/terrain/classify_districts.py.
from classify_districts import (  # noqa: E402
    derive_districts,
    project,
    robinson_y,
    shoelace,
)

DISTRICTS_JSON = os.path.join(ROOT, "spheres-sim", "data", "districts.json")
DATA = os.path.join(ROOT, "spheres-web", "data")


# ---------------------------------------------------------------------------
# District spatial index
# ---------------------------------------------------------------------------

class DistrictIndex:
    """Containment lookup over the game's districts, in WGS84 lon/lat.

    A 1-degree bucket grid narrows candidates; each candidate is then tested
    ring-exactly (exterior minus holes).  Points that land in no district at all
    — an offshore platform, a coastline generalisation gap — fall back to the
    nearest district centroid, but ONLY within `snap_deg`, and the result is
    tagged so the caller can record that the placement was snapped rather than
    contained.  Beyond that radius the answer is None and the record is dropped.
    """

    CELL = 1.0  # degrees

    def __init__(self, roster=None):
        self.polys = {}       # district id -> [(rings, minx, miny, maxx, maxy), ...]
        self.centroid = {}    # district id -> (lon, lat) of largest ring
        self.grid = {}        # (ix, iy) -> [(did, poly_idx), ...]
        self._build(roster)

    def _build(self, roster):
        derived = derive_districts()
        ids = sorted(derived)
        if roster is not None:
            ids = [d for d in ids if d in roster]
        for did in ids:
            entries = []
            best = None
            for poly in derived[did]:
                rings = [r for r in poly if len(r) >= 3]
                if not rings:
                    continue
                xs = [p[0] for p in rings[0]]
                ys = [p[1] for p in rings[0]]
                bbox = (min(xs), min(ys), max(xs), max(ys))
                entries.append((rings, bbox))
                cx, cy, a = _ring_centroid(rings[0])
                if best is None or abs(a) > best[1]:
                    best = ((cx, cy), abs(a))
            if not entries:
                continue
            self.polys[did] = entries
            self.centroid[did] = best[0]
            for pi, (_rings, bbox) in enumerate(entries):
                minx, miny, maxx, maxy = bbox
                for ix in range(int(math.floor(minx / self.CELL)),
                                int(math.floor(maxx / self.CELL)) + 1):
                    for iy in range(int(math.floor(miny / self.CELL)),
                                    int(math.floor(maxy / self.CELL)) + 1):
                        self.grid.setdefault((ix, iy), []).append((did, pi))

    def contains(self, lon, lat):
        """Every district whose polygon contains the point, in sorted id order."""
        key = (int(math.floor(lon / self.CELL)), int(math.floor(lat / self.CELL)))
        hits = []
        for did, pi in self.grid.get(key, ()):
            rings, bbox = self.polys[did][pi]
            minx, miny, maxx, maxy = bbox
            if not (minx <= lon <= maxx and miny <= lat <= maxy):
                continue
            if _in_rings(lon, lat, rings):
                hits.append(did)
        return sorted(set(hits))

    def locate(self, lon, lat, snap_deg=0.0):
        """(district_id, how) or (None, 'unplaced').

        `how` is 'contained' when the point is inside the district's own polygon
        and 'snapped:<deg>' when it fell outside every district and was attached
        to the nearest centroid within `snap_deg`.  Overlapping districts (rare;
        Natural Earth has a few slivers) resolve to the lowest id, so the result
        does not depend on scan order.
        """
        hits = self.contains(lon, lat)
        if hits:
            return hits[0], "contained"
        if snap_deg <= 0.0:
            return None, "unplaced"
        best = None
        cell = int(math.ceil(snap_deg / self.CELL))
        cx0 = int(math.floor(lon / self.CELL))
        cy0 = int(math.floor(lat / self.CELL))
        seen = set()
        for ix in range(cx0 - cell, cx0 + cell + 1):
            for iy in range(cy0 - cell, cy0 + cell + 1):
                for did, _pi in self.grid.get((ix, iy), ()):
                    if did in seen:
                        continue
                    seen.add(did)
                    clon, clat = self.centroid[did]
                    d = math.hypot(clon - lon, (clat - lat))
                    if best is None or d < best[1] or (d == best[1] and did < best[0]):
                        best = (did, d)
        if best is not None and best[1] <= snap_deg:
            return best[0], "snapped"
        return None, "unplaced"


def _ring_centroid(ring):
    return shoelace(ring)


def _in_rings(x, y, rings):
    """GeoJSON semantics: inside the exterior ring and outside every hole."""
    if not _in_ring(x, y, rings[0]):
        return False
    for hole in rings[1:]:
        if _in_ring(x, y, hole):
            return False
    return True


def _in_ring(x, y, ring):
    """Crossing-number test. Deterministic; boundary cases are consistent."""
    inside = False
    n = len(ring)
    j = n - 1
    for i in range(n):
        xi, yi = ring[i]
        xj, yj = ring[j]
        if (yi > y) != (yj > y):
            xint = (xj - xi) * (y - yi) / (yj - yi) + xi
            if x < xint:
                inside = not inside
        j = i
    return inside


# ---------------------------------------------------------------------------
# Polygon helpers (WEP petroleum provinces)
# ---------------------------------------------------------------------------

def polygon_district_overlap(index, rings, step=0.25):
    """District ids a province polygon covers, by sampling a lon/lat lattice.

    Kept for callers that only need "does this polygon touch that district".
    `measure_overlap` below supersedes it for the petroleum provinces: a lattice
    answers the containment question but cannot say whether the province covers
    all of a district or clips one corner of it, and that difference is what put
    Iraq's oil in Al-Anbar rather than Al-Basrah.
    """
    xs = [p[0] for p in rings[0]]
    ys = [p[1] for p in rings[0]]
    minx, maxx = min(xs), max(xs)
    miny, maxy = min(ys), max(ys)
    out = set()
    nx = int((maxx - minx) / step) + 1
    ny = int((maxy - miny) / step) + 1
    for i in range(nx + 1):
        x = minx + i * step
        for j in range(ny + 1):
            y = miny + j * step
            if not _in_rings(x, y, rings):
                continue
            for did in index.contains(x, y):
                out.add(did)
    return sorted(out)


# ---------------------------------------------------------------------------
# Measured polygon intersection (petroleum province x district)
# ---------------------------------------------------------------------------
#
# WHY THIS EXISTS.  A WEP province polygon covers many districts.  The first
# edition asked a 0.25-degree lattice "does this province touch this district"
# and recorded a yes as an entry indistinguishable from any other, so a consumer
# ranking Iraq's districts read Al-Anbar — which the Mesopotamian Foredeep only
# clips along its eastern margin — ahead of Al-Basrah, which the Foredeep covers
# whole.  The second measured area by latitude-row integration, which fixed the
# ranking but discretised latitude and could not resolve a sliver.
#
# This is the third and it is the real thing: GEOS polygon clipping through
# shapely, discovery through an STRtree, and an EXACT spherical area for the
# resulting geometry.  Nothing here is sampled or discretised.
#
# METHOD, in three parts.
#
#   1. GEOMETRY.  Province rings arrive from the shapefile in ESRI's convention
#      — clockwise exterior, counter-clockwise hole — with parts concatenated
#      and no part index.  `rings_to_geometry` classifies each ring by the sign
#      of its shoelace area and rebuilds the true Polygon/MultiPolygon, so a
#      multi-part province is a multipolygon and a hole is a hole.  (Treating
#      ring 0 as the exterior and every later ring as its hole, which the flat
#      list invites, is wrong the moment a province has two separate parts.)
#      Every geometry is run through `make_valid`, because a self-touching
#      digitised outline makes GEOS refuse the intersection outright.
#
#   2. INTERSECTION.  `district.intersection(province)` in WGS84 lon/lat.  This
#      is exact polygon clipping, not sampling: a province that covers 6% of
#      Al-Anbar and 90% of Al-Basrah measures as those two numbers and not as
#      two identical booleans.
#
#   3. AREA.  Green's theorem on the sphere.  For a region bounded by straight
#      edges in lon/lat, area = -contour_integral sin(phi) d(lambda), and each
#      edge integrates in closed form, so the area is EXACT for the polygon as
#      the source actually stores it.  No cos(lat) row weighting, no midpoint
#      rule, no discretisation constant to cancel.  Results are in km^2 on a
#      sphere of radius 6371.0088 km, so they can be sanity-checked against the
#      districts' own published `area_sqkm`.
#
# WHAT IT MEASURES.  Three numbers per attachment: the intersection area, the
# fraction of the DISTRICT that lies inside the province, and the fraction of
# the PROVINCE that lies inside the district.  The second says how much of this
# ground is oil province; the third is the apportionment weight — the share of
# the province that this district holds.
#
# EDGE CASES, all four of them real in this data.
#
#   * A province spanning many districts.  The Mesopotamian Foredeep reaches 22
#     districts across three nations; each gets its own measured pair.
#   * A district touched by several provinces.  Al-Anbar meets seven.  They
#     accumulate; they are never merged, because they carry different volumes.
#   * A province extending beyond every district.  Summing `area_frac_province`
#     over all districts gives how much of the province is on land in the
#     roster at all: the North Sea Graben is 0.26%, the Niger Delta 17.4%.  The
#     remainder is offshore and is left explicitly unapportioned rather than
#     redistributed onto the coast.
#   * Multipolygons and holes.  Both province multi-ring cases in WEP_PRVA are
#     exteriors with holes (Tian Shan Foldbelt has three, East Greenland one);
#     districts are routinely multipolygons — islands, exclaves — and
#     `unary_union` of a district's parts stops two touching parts from
#     double-counting the overlap.
#
# THE SLIVER FLOOR, and why the United States needs it.  Exact clipping finds
# overlaps that sampling could not, including ones that are not real.  WEP's
# North American polygons are cut at the 49th parallel and along the
# Yukon-Alaska line, and those cuts do not fall exactly on Natural Earth's
# border, so the Alberta Basin overlaps Montana by 0.14 km^2, the Williston
# Basin overlaps North Dakota by 0.08 km^2, and the Mackenzie Foldbelt overlaps
# Alaska by 13.9 km^2 — ribbons a few hundred metres wide along a shared line.
# Kept, they would have handed the world's number two oil producer a "located"
# oil district built out of a digitising mismatch, which is precisely the
# fabrication the unlocated-producer ruling exists to prevent.
#
# The floor drops an attachment only when it is negligible in BOTH polygons:
# under one part in ten thousand of the district AND under one part in ten
# thousand of the province.  Requiring both is what keeps the genuinely tiny
# districts — Port of Spain is 0.77 km^2 of the East Venezuela Basin but that
# is 5.3% of Port of Spain, and Ajman is 140 km^2 but 93% of Ajman.  It removes
# 15 attachments in total and every one is a boundary ribbon.  The value is a
# STATED CHOICE, not a discovered constant; the measured distribution ships in
# the artifact so it can be re-argued against the numbers.

SLIVER_FRAC = 1e-4
EARTH_R_KM = 6371.0088


def ring_area_sr(ring):
    """Signed area in steradians of a lon/lat ring with straight edges.

    Green's theorem with P = -sin(phi), Q = 0, so the integrand is cos(phi) —
    the spherical area element — and each straight edge integrates in closed
    form, which makes this exact for the polygon as the source stores it.
    Counter-clockwise is positive.

    The edge integral is -dlam * (cos p1 - cos p2) / dphi, and that form must
    NOT be used: GEOS emits edges cut along a latitude line whose two latitudes
    differ only in the last bit or two, and `cos p1 - cos p2` then cancels away
    every significant digit. It is not a rounding wobble. On the Pannonian
    Basin's clip of Tuzla one such edge -- dphi = -6.7e-16, dlam = -3.0e-3 --
    was wrong by 5% of a term the size of the whole polygon, and the district
    came out at 6,129 km^2 against a true 3,047.

    The identity cos A - cos B = 2 sin((A+B)/2) sin((B-A)/2) rewrites the same
    integral as sin(midpoint) * sinc(dphi/2), which contains no subtraction of
    near-equal quantities and tends to sin(midpoint) smoothly as the edge goes
    horizontal. That is the form below.
    """
    total = 0.0
    n = len(ring)
    for i in range(n):
        lon1, lat1 = ring[i - 1]
        lon2, lat2 = ring[i]
        dlam = math.radians(lon2 - lon1)
        if dlam == 0.0:
            continue
        p1 = math.radians(lat1)
        p2 = math.radians(lat2)
        half = (p2 - p1) / 2.0
        sinc = 1.0 if half == 0.0 else math.sin(half) / half
        total -= dlam * math.sin((p1 + p2) / 2.0) * sinc
    return total


def geom_area_km2(geom):
    """Exact spherical area of a shapely Polygon/MultiPolygon, in km^2."""
    if geom is None or geom.is_empty:
        return 0.0
    if geom.geom_type == "Polygon":
        polys = [geom]
    elif geom.geom_type == "MultiPolygon":
        polys = list(geom.geoms)
    else:
        polys = [g for g in getattr(geom, "geoms", ()) if g.geom_type == "Polygon"]
    sr = 0.0
    for p in polys:
        sr += abs(ring_area_sr(list(p.exterior.coords)[:-1]))
        for hole in p.interiors:
            sr -= abs(ring_area_sr(list(hole.coords)[:-1]))
    return sr * EARTH_R_KM * EARTH_R_KM


def validated(geom):
    """A polygonal geometry GEOS will accept for clipping.

    Digitised outlines self-touch. `make_valid` repairs without moving a vertex;
    anything it hands back that is not areal (a stray line where two rings meet
    at a point) is discarded rather than buffered into existence.
    """
    if geom is None or geom.is_empty:
        return geom
    if geom.is_valid:
        return geom
    fixed = shapely.make_valid(geom)
    if fixed.geom_type in ("Polygon", "MultiPolygon"):
        return fixed
    parts = [g for g in getattr(fixed, "geoms", ())
             if g.geom_type in ("Polygon", "MultiPolygon")]
    if not parts:
        return Polygon()
    return unary_union(parts)


def _shoelace_sign(ring):
    s = 0.0
    for i in range(len(ring)):
        x1, y1 = ring[i - 1]
        x2, y2 = ring[i]
        s += x1 * y2 - x2 * y1
    return s


def rings_to_geometry(rings):
    """A flat ESRI ring list -> Polygon or MultiPolygon.

    Clockwise (negative shoelace) opens a new part; counter-clockwise is a hole
    in the part it follows. This is the shapefile's own convention and it is the
    only correct reading of a flat list: a province with two separate parts is
    two exteriors, not an exterior with a hole shaped like its other half.
    """
    parts = []
    exterior = None
    holes = []
    for ring in rings:
        if len(ring) < 3:
            continue
        if _shoelace_sign(ring) < 0.0:
            if exterior is not None:
                parts.append((exterior, holes))
            exterior, holes = ring, []
        elif exterior is None:
            exterior, holes = ring, []      # a lone CCW ring is still the part
        else:
            holes.append(ring)
    if exterior is not None:
        parts.append((exterior, holes))
    polys = []
    for ext, hls in parts:
        g = validated(Polygon(ext, hls))
        if g is not None and not g.is_empty:
            polys.append(g)
    if not polys:
        return None
    if len(polys) == 1:
        return polys[0]
    return validated(unary_union(polys))


class ProvinceIntersector:
    """Measured province-x-district intersection, via GEOS and an STRtree.

    Districts are converted once from `DistrictIndex`'s rings — which came from
    `tools/terrain/classify_districts.derive_districts`, the one definition of
    district identity in this repo — into validated shapely geometries, and
    indexed in an STRtree. `measure` then queries the tree with a province and
    clips against only the districts whose envelopes actually meet it.

    Determinism: the tree is built over ids in sorted order, query results are
    sorted before use, and GEOS clipping is a deterministic function of its
    inputs. Two runs produce the same floats.
    """

    def __init__(self, index):
        self.ids = []
        self.geom = {}
        self.area = {}
        for did in sorted(index.polys):
            polys = []
            for rings, _bbox in index.polys[did]:
                g = validated(Polygon(rings[0], rings[1:]))
                if g is not None and not g.is_empty:
                    polys.append(g)
            if not polys:
                continue
            g = polys[0] if len(polys) == 1 else validated(unary_union(polys))
            if g is None or g.is_empty:
                continue
            self.ids.append(did)
            self.geom[did] = g
            self.area[did] = geom_area_km2(g)
        self.tree = STRtree([self.geom[d] for d in self.ids])

    def district_area(self, did):
        return self.area.get(did, 0.0)

    def measure(self, geom, sliver_frac=SLIVER_FRAC):
        """{district_id: (intersection_km2, frac_of_district, frac_of_province)}.

        Attachments negligible in both polygons are dropped as boundary
        ribbons; see SLIVER_FRAC above. Districts are returned in sorted id
        order so a caller that iterates gets a stable sequence.
        """
        parea = geom_area_km2(geom)
        if parea <= 0.0:
            return {}, []
        out = {}
        slivers = []
        for hit in sorted(self.tree.query(geom)):
            did = self.ids[hit]
            dgeom = self.geom[did]
            try:
                clipped = geom.intersection(dgeom)
            except shapely.errors.GEOSException:
                clipped = validated(geom).intersection(validated(dgeom))
            if clipped.is_empty:
                continue
            inter = geom_area_km2(clipped)
            if inter <= 0.0:
                continue
            darea = self.area[did]
            fd = min(inter / darea, 1.0) if darea > 0.0 else 0.0
            fp = min(inter / parea, 1.0)
            if fd < sliver_frac and fp < sliver_frac:
                slivers.append((did, inter, fd, fp))
                continue
            out[did] = (inter, fd, fp)
        return {k: out[k] for k in sorted(out)}, sorted(slivers)


def bbox_candidates(index, rings):
    """District ids whose bounding box meets this polygon's bounding box.

    Retained for callers that want a cheap pre-filter without building the
    STRtree. `ProvinceIntersector` does not use it — the tree is both faster and
    tighter — but a bounding-box test is still the right coarse filter, because
    a lattice at any step can step over a small district entirely.
    """
    xs = [p[0] for r in rings for p in r]
    ys = [p[1] for r in rings for p in r]
    minx, maxx, miny, maxy = min(xs), max(xs), min(ys), max(ys)
    out = set()
    for ix in range(int(math.floor(minx / index.CELL)),
                    int(math.floor(maxx / index.CELL)) + 1):
        for iy in range(int(math.floor(miny / index.CELL)),
                        int(math.floor(maxy / index.CELL)) + 1):
            for did, pi in index.grid.get((ix, iy), ()):
                bx0, by0, bx1, by1 = index.polys[did][pi][1]
                if bx1 >= minx and bx0 <= maxx and by1 >= miny and by0 <= maxy:
                    out.add(did)
    return sorted(out)


# ---------------------------------------------------------------------------
# Administrative centroids
# ---------------------------------------------------------------------------
# RULING 4, 2026-08-31. MRDS files a record it cannot place at the
# administrative centre of the unit it does know — the country, or the state.
# Those points are real published coordinates and they are transcribed
# unchanged, but they are not mine locations and must never be COUNTED as
# evidence of a site. This is how they are recognised without touching them:
# compute the centroid of every Natural Earth 10m admin-0 and admin-1 polygon
# and ask whether a published coordinate is sitting on one.
#
# Two centroids are computed per unit and the nearer wins, because neither alone
# finds them all. France's whole-geometry centroid is dragged to 42.18N by
# Guyane and Reunion, so only the largest part's centroid names the "centre of
# France" the data uses; Italy is the reverse, Sicily and Sardinia pushing the
# largest-part centroid 81 km north of the point that is actually in the data.
# Whichever matched is recorded, so the match can be checked rather than trusted.
#
# The radius is a STATED CHOICE. At 2 km the measured matches are sub-kilometre
# (Spain 0.44 km, Queensland 0.36 km, Northern Territory 0.62 km, Sweden 1.29
# km) and the exposure to coincidence is small but real: 5,656 centroids times a
# 12.6 km^2 disc is 0.05% of the world's land, so among 56,325 distinct
# coordinates roughly 27 matches are expected by chance alone. That estimate
# ships in the artifact beside the count, because a flag whose false-positive
# rate is unstated is a flag that cannot be argued with.

NE_ADMIN = (
    ("ne_10m_admin_0.geojson", ("ADMIN", "NAME"), "country"),
    ("ne_10m_admin_1.geojson", ("name", "name_en"), "province"),
)

CENTROID_KM = 2.0


def haversine_km(lon1, lat1, lon2, lat2):
    p1, p2 = math.radians(lat1), math.radians(lat2)
    dp = p2 - p1
    dl = math.radians(lon2 - lon1)
    h = (math.sin(dp / 2.0) ** 2
         + math.cos(p1) * math.cos(p2) * math.sin(dl / 2.0) ** 2)
    return 2.0 * EARTH_R_KM * math.asin(math.sqrt(min(1.0, h)))


class AdminCentroids:
    """Nearest administrative centroid to a published coordinate.

    Loads Natural Earth 10m admin-0 and admin-1 — the same two files the
    district roster is derived from, so no new dependency enters the pipeline —
    and holds two centroids per polygon: the whole geometry's, and the largest
    part's.  `nearest` returns (km, name, level, which) or None, searching a
    one-degree bucket grid with a one-cell margin, which is ample for a 2 km
    question.
    """

    CELL = 1.0

    def __init__(self):
        self.points = []            # (lat, lon, name, level, which)
        self.grid = {}
        for fname, name_fields, level in NE_ADMIN:
            path = os.path.join(DATA, fname)
            with open(path, encoding="utf-8") as f:
                gj = json.load(f)
            for feat in gj["features"]:
                props = feat.get("properties") or {}
                name = ""
                for field in name_fields:
                    name = (props.get(field) or "").strip()
                    if name:
                        break
                geom = feat.get("geometry")
                if not geom:
                    continue
                g = shape(geom)
                if g.is_empty or not g.area:
                    continue
                whole = g.centroid
                self._add(whole.y, whole.x, name, level, "whole")
                parts = list(g.geoms) if g.geom_type == "MultiPolygon" else [g]
                if len(parts) > 1:
                    big = max(parts, key=lambda q: q.area).centroid
                    self._add(big.y, big.x, name, level, "largest_part")
        self.points.sort()
        self.grid = {}
        for i, p in enumerate(self.points):
            self.grid.setdefault(self._cell(p[1], p[0]), []).append(i)

    def _cell(self, lon, lat):
        return (int(math.floor(lon / self.CELL)), int(math.floor(lat / self.CELL)))

    def _add(self, lat, lon, name, level, which):
        if not (-90.0 <= lat <= 90.0 and -180.0 <= lon <= 180.0):
            return
        self.points.append((lat, lon, name, level, which))

    def nearest(self, lon, lat):
        cx, cy = self._cell(lon, lat)
        best = None
        for ix in range(cx - 1, cx + 2):
            for iy in range(cy - 1, cy + 2):
                for i in self.grid.get((ix, iy), ()):
                    plat, plon, name, level, which = self.points[i]
                    d = haversine_km(lon, lat, plon, plat)
                    cand = (d, name, level, which)
                    if best is None or cand < best:
                        best = cand
        return best


# ---------------------------------------------------------------------------
# Nation crosswalk
# ---------------------------------------------------------------------------

def game_nations():
    with open(DISTRICTS_JSON, encoding="utf-8") as f:
        return sorted(json.load(f)["nations"])


class NationCrosswalk:
    """Source country name -> game nation, hand-authored and fail-loud.

    `IGNORE` holds names that are deliberately not game nations: aggregates
    ('World', 'OPEC'), microstates and dependencies outside the 160-nation
    roster, and Antarctica.  Anything not in a table and not ignored raises, so
    a new source vocabulary cannot quietly lose a country.
    """

    def __init__(self, table, ignore=()):
        self.table = dict(table)
        self.ignore = set(ignore)
        self.unmapped = {}

    def get(self, name, strict=True):
        n = (name or "").strip()
        if not n:
            return None
        if n in self.ignore:
            return None
        if n in self.table:
            return self.table[n]
        if n in GAME:
            return n
        self.unmapped[n] = self.unmapped.get(n, 0) + 1
        if strict:
            raise KeyError(f"unmapped country name: {n!r}")
        return None


GAME = set(game_nations())


if __name__ == "__main__":
    idx = DistrictIndex()
    print("districts indexed:", len(idx.polys))
    print("grid cells:", len(idx.grid))
    probes = [
        ("Kabwe Mine, Zambia", 27.800, -13.450),
        ("Chuquicamata, Chile", -68.900, -22.300),
        ("Kalgoorlie, Australia", 121.470, -30.750),
        ("Witwatersrand, South Africa", 27.000, -26.200),
        ("Ghawar, Saudi Arabia", 49.200, 25.400),
        ("Ruhr, Germany", 7.200, 51.500),
        ("Kolwezi, Zaire", 25.470, -10.720),
        ("Krivoy Rog, Ukraine", 33.400, 47.900),
        ("Houston, USA", -95.400, 29.750),
    ]
    for name, lon, lat in probes:
        did, how = idx.locate(lon, lat, snap_deg=1.0)
        print(f"  {name:32s} -> {did!s:22s} {how}")

    # The area integral, held against a number this file did not produce.
    # districts.json publishes `area_sqkm` from the terrain pass; if the
    # spherical integral is right they agree to a fraction of a percent, and if
    # a cancellation creeps back into `ring_area_sr` this prints the damage.
    inter = ProvinceIntersector(idx)
    with open(DISTRICTS_JSON, encoding="utf-8") as f:
        pub = {d["id"]: d.get("area_sqkm")
               for ds in json.load(f)["nations"].values() for d in ds}
    worst = None
    checked = 0
    for did in inter.ids:
        want = pub.get(did)
        if not want:
            continue
        checked += 1
        err = abs(inter.area[did] - want) / want
        if worst is None or err > worst[1]:
            worst = (did, err, inter.area[did], want)
    print(f"\narea integral vs districts.json area_sqkm ({checked} districts)")
    print("  worst: %s  measured %.1f vs published %.1f  (%.4f%%)"
          % (worst[0], worst[2], worst[3], 100 * worst[1]))
