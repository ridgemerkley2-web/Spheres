#!/usr/bin/env python
"""Estimate how much of the world's land the NE geography-region polys cover.

Inputs (read-only):
  C:/Users/ridge/Spheres/spheres-web/data/ne_10m_geography_regions_polys.geojson
  C:/Users/ridge/Spheres/spheres-web/data/ne_10m_admin_1.geojson  (land mask)

Method: deterministic 0.5-degree lon/lat grid (no RNG), each point weighted by
cos(lat) to approximate true area. A point is "land" if inside any admin-1
polygon. For land points we record which featurecla polys contain them.
Coverage is reported for all land and for land within the game's latitude band
[-58, 83]. "Terrain-relevant" excludes Continent, Island, Island group,
Dragons-be-here (i.e. classes that are not usable as terrain tags).

Invocation: python coverage_estimate.py
Output: JSON to stdout.
"""
import json
import sys
from collections import defaultdict

import numpy as np
import shapely
from shapely.geometry import shape
from shapely.strtree import STRtree

REG = "C:/Users/ridge/Spheres/spheres-web/data/ne_10m_geography_regions_polys.geojson"
ADM = "C:/Users/ridge/Spheres/spheres-web/data/ne_10m_admin_1.geojson"
STEP = 0.5
LAT_TOP, LAT_BOT = 83.0, -58.0
NON_TERRAIN = {"Continent", "Island", "Island group", "Dragons-be-here"}


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
    land_idx = np.unique(pairs[0])
    land_mask = np.zeros(len(pts), dtype=bool)
    land_mask[land_idx] = True

    land_pts = pts[land_mask]
    land_w = w[land_mask]
    land_lat = LAT.ravel()[land_mask]
    in_band = (land_lat >= LAT_BOT) & (land_lat <= LAT_TOP)

    reg_tree = STRtree(reg_geoms)
    rp = reg_tree.query(land_pts, predicate="intersects")

    per_fc = defaultdict(lambda: np.zeros(len(land_pts), dtype=bool))
    any_cov = np.zeros(len(land_pts), dtype=bool)
    terr_cov = np.zeros(len(land_pts), dtype=bool)
    for pi, gi in zip(rp[0], rp[1]):
        fc = reg_props[gi].get("featurecla")
        per_fc[fc][pi] = True
        any_cov[pi] = True
        if fc not in NON_TERRAIN:
            terr_cov[pi] = True

    def frac(mask, sel):
        tw = land_w[sel].sum()
        return round(float(land_w[mask & sel].sum() / tw), 4) if tw else None

    all_sel = np.ones(len(land_pts), dtype=bool)
    out = {
        "grid_step_deg": STEP,
        "land_sample_points": int(len(land_pts)),
        "coverage_all_polys": {"all_land": frac(any_cov, all_sel), "band_-58_83": frac(any_cov, in_band)},
        "coverage_terrain_relevant": {"all_land": frac(terr_cov, all_sel), "band_-58_83": frac(terr_cov, in_band)},
        "per_featurecla_land_fraction_band": {
            fc: frac(m, in_band) for fc, m in sorted(per_fc.items(), key=lambda kv: str(kv[0]))
        },
    }
    json.dump(out, sys.stdout, indent=1)


if __name__ == "__main__":
    main()
