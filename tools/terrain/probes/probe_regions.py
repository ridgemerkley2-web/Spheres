#!/usr/bin/env python
"""Probe ne_10m_geography_regions_polys.geojson.

Inputs (read-only):
  C:/Users/ridge/Spheres/spheres-web/data/ne_10m_geography_regions_polys.geojson

Invocation:
  python probe_regions.py

Output: JSON report to stdout. Deterministic (no RNG, sorted keys/orders).
"""
import json
import sys
from collections import Counter, defaultdict

SRC = "C:/Users/ridge/Spheres/spheres-web/data/ne_10m_geography_regions_polys.geojson"

FAMOUS = [
    "himalaya", "alps", "andes", "rocky", "zagros", "sahara", "gobi",
    "kalahari", "amazon", "siberia", "tibet", "plateau of tibet",
]


def main():
    with open(SRC, encoding="utf-8") as f:
        gj = json.load(f)
    feats = gj["features"]
    # Normalize property keys to lowercase (this NE export uses uppercase keys).
    for ft in feats:
        ft["properties"] = {k.lower(): v for k, v in ft.get("properties", {}).items()}
    report = {}
    report["feature_count"] = len(feats)

    # property keys union
    keys = Counter()
    for ft in feats:
        for k in ft.get("properties", {}):
            keys[k] += 1
    report["property_keys"] = dict(sorted(keys.items()))

    # featurecla vocabulary with counts + up to 8 example names each (sorted)
    fc_counts = Counter()
    fc_examples = defaultdict(set)
    region_counts = Counter()
    for ft in feats:
        p = ft["properties"]
        fc = p.get("featurecla")
        fc_counts[fc] += 1
        nm = p.get("name") or p.get("name_en") or "<unnamed>"
        fc_examples[fc].add(nm)
        region_counts[p.get("region")] += 1
    report["featurecla"] = {
        fc: {"count": n, "examples": sorted(fc_examples[fc])[:10]}
        for fc, n in sorted(fc_counts.items(), key=lambda kv: (-kv[1], str(kv[0])))
    }
    report["region_values"] = dict(sorted(region_counts.items(), key=lambda kv: (-kv[1], str(kv[0]))))

    # geometry sanity
    geom_types = Counter()
    bad = 0
    for ft in feats:
        g = ft.get("geometry")
        if g is None:
            bad += 1
            continue
        geom_types[g["type"]] += 1
    report["geometry_types"] = dict(sorted(geom_types.items()))
    report["null_geometries"] = bad

    # famous features by substring match on name/name_en (case-insensitive)
    famous_hits = {}
    for ft in feats:
        p = ft["properties"]
        nm = (p.get("name") or "")
        nml = nm.lower()
        for key in FAMOUS:
            if key in nml:
                famous_hits.setdefault(key, []).append(
                    {"name": nm, "featurecla": p.get("featurecla"), "region": p.get("region")}
                )
    for k in famous_hits:
        famous_hits[k] = sorted(famous_hits[k], key=lambda d: d["name"])
    report["famous"] = {k: famous_hits.get(k, []) for k in FAMOUS}

    json.dump(report, sys.stdout, indent=1)


if __name__ == "__main__":
    main()
