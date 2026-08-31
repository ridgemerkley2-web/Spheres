#!/usr/bin/env python
"""Strict physiographic coverage + overlap stats + scalerank distribution.

Inputs (read-only):
  C:/Users/ridge/Spheres/spheres-web/data/ne_10m_geography_regions_polys.geojson
  C:/Users/ridge/Spheres/spheres-web/data/ne_10m_admin_1.geojson  (land mask)

Same deterministic 0.5-degree cos(lat)-weighted grid as coverage_estimate.py.
"Physio" classes = actual terrain: Range/mtn, Plateau, Desert, Basin, Plain,
Lowland, Tundra, Delta, Valley, Foothills, Gorge, Wetlands, Depression, Coast.
Reports: land fraction covered by >=1 physio poly (band -58..83), fraction
covered by >=2 (overlaps), top overlapping class pairs, scalerank histogram
per featurecla.

Invocation: python strict_coverage.py
"""
import json
import sys
from collections import Counter, defaultdict

import numpy as np
import shapely
from shapely.geometry import shape
from shapely.strtree import STRtree

REG = "C:/Users/ridge/Spheres/spheres-web/data/ne_10m_geography_regions_polys.geojson"
ADM = "C:/Users/ridge/Spheres/spheres-web/data/ne_10m_admin_1.geojson"
STEP = 0.5
LAT_TOP, LAT_BOT = 83.0, -58.0
PHYSIO = {"Range/mtn", "Plateau", "Desert", "Basin", "Plain", "Lowland",
          "Tundra", "Delta", "Valley", "Foothills", "Gorge", "Wetlands",
          "Depression", "Coast"}


def load_geoms(path):
    with open(path, encoding="utf-8") as f:
        gj = json.load(f)
    geoms, props = [], []
    for ft in gj["features"]:
        g = ft.get("geometry")
        if g is None:
            continue
        geoms.append(shape(g))
        props.append({k.lower(): v for k, v in ft.get("properties", {}).items()})
    return geoms, props


def main():
    adm_geoms, _ = load_geoms(ADM)
    reg_geoms, reg_props = load_geoms(REG)

    lons = np.arange(-180 + STEP / 2, 180, STEP)
    lats = np.arange(-90 + STEP / 2, 90, STEP)
    LON, LAT = np.meshgrid(lons, lats)
    pts = shapely.points(LON.ravel(), LAT.ravel())
    w = np.cos(np.deg2rad(LAT.ravel()))

    adm_tree = STRtree(adm_geoms)
    pairs = adm_tree.query(pts, predicate="intersects")
    land_mask = np.zeros(len(pts), dtype=bool)
    land_mask[np.unique(pairs[0])] = True

    land_pts = pts[land_mask]
    land_w = w[land_mask]
    land_lat = LAT.ravel()[land_mask]
    in_band = (land_lat >= LAT_BOT) & (land_lat <= LAT_TOP)
    band_w = land_w[in_band].sum()

    reg_tree = STRtree(reg_geoms)
    rp = reg_tree.query(land_pts, predicate="intersects")

    physio_sets = defaultdict(set)  # point index -> set of physio featureclas
    for pi, gi in zip(rp[0], rp[1]):
        fc = reg_props[gi].get("featurecla")
        if fc in PHYSIO:
            physio_sets[pi].add(fc)

    cov1 = np.zeros(len(land_pts), dtype=bool)
    cov2 = np.zeros(len(land_pts), dtype=bool)
    pair_w = Counter()
    for pi, fcs in physio_sets.items():
        cov1[pi] = True
        if len(fcs) >= 2:
            cov2[pi] = True
            if in_band[pi]:
                for a in sorted(fcs):
                    for b in sorted(fcs):
                        if a < b:
                            pair_w[(a, b)] += land_w[pi]

    def frac(mask):
        return round(float(land_w[mask & in_band].sum() / band_w), 4)

    sr = defaultdict(Counter)
    for p in reg_props:
        sr[p.get("featurecla")][p.get("scalerank")] += 1

    out = {
        "physio_coverage_band": frac(cov1),
        "physio_overlap_ge2_band": frac(cov2),
        "top_overlap_pairs_band": [
            {"pair": list(k), "land_fraction": round(float(v / band_w), 4)}
            for k, v in sorted(pair_w.items(), key=lambda kv: (-kv[1], kv[0]))[:10]
        ],
        "scalerank_by_featurecla": {
            fc: dict(sorted(c.items(), key=lambda kv: (kv[0] is None, kv[0])))
            for fc, c in sorted(sr.items(), key=lambda kv: str(kv[0]))
        },
    }
    json.dump(out, sys.stdout, indent=1)


if __name__ == "__main__":
    main()
