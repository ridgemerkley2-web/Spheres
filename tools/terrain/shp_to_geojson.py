#!/usr/bin/env python
# shp_to_geojson.py — convert a Natural Earth shapefile to the GeoJSON the
# existing bakers read, so a fresh download can be fed to tools written against
# the GeoJSON distribution without either being rewritten.
#
# Natural Earth publishes the same data both ways. The shapefile is about a
# tenth the size, which is why the sources fetched into spheres-web/data are
# shapefiles; mapgen.rs, classify_districts.py and make_rivers.py all read
# GeoJSON. This bridges the two, and does nothing else.
#
# RING GROUPING IS THE ONLY REAL WORK. A shapefile polygon is a flat list of
# rings that distinguishes holes by WINDING: an outer ring is clockwise, a hole
# counter-clockwise, and a hole belongs to the outer ring before it. GeoJSON
# instead nests them, [[outer, hole, hole], [outer], ...]. Getting this wrong
# does not fail loudly — it turns every lake in a country into land, or every
# island into a hole.
#
# Invocation:  python tools/terrain/shp_to_geojson.py <in-without-extension> <out.geojson>

import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import shapefile                                            # noqa: E402


def signed_area(ring):
    s = 0.0
    for i in range(len(ring)):
        x1, y1 = ring[i]
        x2, y2 = ring[(i + 1) % len(ring)]
        s += x1 * y2 - x2 * y1
    return 0.5 * s


def group_rings(parts):
    """Flat shapefile rings -> GeoJSON MultiPolygon coordinates."""
    polys = []
    for ring in parts:
        if len(ring) < 4:
            continue
        pts = [[float(x), float(y)] for x, y in ring]
        if pts[0] != pts[-1]:
            pts.append(list(pts[0]))
        if signed_area(ring) < 0 or not polys:      # clockwise: a new outer ring
            polys.append([pts])
        else:                                        # counter-clockwise: a hole
            polys[-1].append(pts)
    return polys


def convert(src, dst):
    recs = shapefile.read(src)
    features = []
    for rec in recs:
        parts = rec.pop("parts")
        rec.pop("shape_type", None)
        polys = group_rings(parts)
        if not polys:
            geom = None
        elif len(polys) == 1:
            geom = {"type": "Polygon", "coordinates": polys[0]}
        else:
            geom = {"type": "MultiPolygon", "coordinates": polys}
        features.append({"type": "Feature", "properties": rec, "geometry": geom})
    with open(dst, "w", encoding="utf-8") as fh:
        json.dump({"type": "FeatureCollection", "features": features}, fh)
    print("%s -> %s: %d features, %d bytes"
          % (os.path.basename(src), os.path.basename(dst), len(features), os.path.getsize(dst)))


if __name__ == "__main__":
    if len(sys.argv) != 3:
        raise SystemExit(__doc__)
    convert(sys.argv[1], sys.argv[2])
