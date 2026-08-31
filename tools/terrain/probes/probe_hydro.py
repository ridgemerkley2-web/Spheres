# probe_hydro.py
# Inputs:
#   C:/Users/ridge/Spheres/spheres-web/data/ne_10m_rivers_lake_centerlines.geojson
#   C:/Users/ridge/Spheres/spheres-web/data/ne_10m_lakes.geojson
# Invocation: python probe_hydro.py
# Purpose: report feature counts, geometry types, property names, scalerank/strokeweig
#          distributions, name coverage, and counts at candidate thresholds for
#          selecting militarily-significant rivers and major lakes.
# Deterministic: read-only probe, stable sorted output.

import json
from collections import Counter

DATA = "C:/Users/ridge/Spheres/spheres-web/data"

def load(name):
    with open(f"{DATA}/{name}", encoding="utf-8") as f:
        return json.load(f)

def probe_rivers():
    gj = load("ne_10m_rivers_lake_centerlines.geojson")
    feats = gj["features"]
    print("=== RIVERS: ne_10m_rivers_lake_centerlines.geojson ===")
    print("feature_count:", len(feats))
    geom_types = Counter(f["geometry"]["type"] if f["geometry"] else "null" for f in feats)
    print("geometry_types:", dict(sorted(geom_types.items())))
    # property names (union across all features)
    prop_names = Counter()
    for f in feats:
        for k in f["properties"]:
            prop_names[k] += 1
    print("property_names (name: count_present):")
    for k in sorted(prop_names):
        print(f"  {k}: {prop_names[k]}")
    # sample one feature's properties
    print("sample_properties_feature0:", json.dumps(feats[0]["properties"], sort_keys=True)[:800])

    # scalerank distribution
    sr = Counter(f["properties"].get("scalerank") for f in feats)
    print("scalerank_distribution:", dict(sorted(sr.items(), key=lambda x: (x[0] is None, x[0]))))
    # strokeweig distribution
    sw = Counter(f["properties"].get("strokeweig") for f in feats)
    print("strokeweig_distribution:", dict(sorted(sw.items(), key=lambda x: (x[0] is None, x[0]))))
    # featurecla distribution
    fc = Counter(f["properties"].get("featurecla") for f in feats)
    print("featurecla_distribution:", dict(sorted(fc.items(), key=lambda x: (x[0] is None, str(x[0])))))

    # name coverage
    named = sum(1 for f in feats if f["properties"].get("name"))
    print("name_coverage:", named, "/", len(feats))
    # other name fields
    for nf in ("name_en", "name_alt", "label"):
        if prop_names.get(nf):
            n = sum(1 for f in feats if f["properties"].get(nf))
            print(f"{nf}_coverage:", n, "/", len(feats))

    # counts at each scalerank threshold
    print("cumulative_counts_by_scalerank_threshold (scalerank <= t):")
    ranks = sorted(k for k in sr if k is not None)
    for t in ranks:
        c = sum(v for k, v in sr.items() if k is not None and k <= t)
        print(f"  <= {t}: {c}")
    # counts at strokeweig thresholds
    sws = sorted(k for k in sw if k is not None)
    print("strokeweig_values_sorted:", sws[:5], "...", sws[-5:] if len(sws) > 5 else "")
    print("cumulative_counts_by_strokeweig_threshold (strokeweig >= t):")
    for t in sorted(set(sws)):
        c = sum(v for k, v in sw.items() if k is not None and k >= t)
        print(f"  >= {t}: {c}")

    # check the marquee rivers: what scalerank/strokeweig do they carry?
    targets = ["Rhine", "Danube", "Euphrates", "Tigris", "Volga", "Mississippi",
               "Yangtze", "Mekong", "Nile", "Amazon", "Amazonas", "Chang Jiang",
               "Rhein", "Donau"]
    print("marquee_rivers (name match, exact or substring):")
    seen = []
    for f in feats:
        p = f["properties"]
        nm = p.get("name") or ""
        nme = p.get("name_en") or ""
        for t in targets:
            if t.lower() in nm.lower() or t.lower() in nme.lower():
                seen.append((nm, nme, p.get("scalerank"), p.get("strokeweig"),
                             p.get("featurecla"), f["geometry"]["type"] if f["geometry"] else None))
                break
    for row in sorted(set(seen)):
        print("  name=%r name_en=%r scalerank=%s strokeweig=%s featurecla=%r geom=%s" % row)

    # per-threshold: are all marquee rivers retained?
    marquee_exact = {"Rhine", "Danube", "Euphrates", "Tigris", "Volga", "Mississippi",
                     "Yangtze", "Mekong", "Nile", "Amazon"}
    print("marquee_retention_by_scalerank_threshold:")
    for t in ranks:
        kept = set()
        for f in feats:
            p = f["properties"]
            if p.get("scalerank") is not None and p["scalerank"] <= t:
                for nm in (p.get("name") or "", p.get("name_en") or ""):
                    for m in marquee_exact:
                        if m.lower() in nm.lower():
                            kept.add(m)
        missing = sorted(marquee_exact - kept)
        cnt = sum(v for k, v in sr.items() if k is not None and k <= t)
        print(f"  <= {t}: total={cnt} marquee_kept={len(kept)}/10 missing={missing}")

def probe_lakes():
    gj = load("ne_10m_lakes.geojson")
    feats = gj["features"]
    print()
    print("=== LAKES: ne_10m_lakes.geojson ===")
    print("feature_count:", len(feats))
    geom_types = Counter(f["geometry"]["type"] if f["geometry"] else "null" for f in feats)
    print("geometry_types:", dict(sorted(geom_types.items())))
    prop_names = Counter()
    for f in feats:
        for k in f["properties"]:
            prop_names[k] += 1
    print("property_names (name: count_present):")
    for k in sorted(prop_names):
        print(f"  {k}: {prop_names[k]}")
    print("sample_properties_feature0:", json.dumps(feats[0]["properties"], sort_keys=True)[:800])

    sr = Counter(f["properties"].get("scalerank") for f in feats)
    print("scalerank_distribution:", dict(sorted(sr.items(), key=lambda x: (x[0] is None, x[0]))))
    fc = Counter(f["properties"].get("featurecla") for f in feats)
    print("featurecla_distribution:", dict(sorted(fc.items(), key=lambda x: (x[0] is None, str(x[0])))))

    named = sum(1 for f in feats if f["properties"].get("name"))
    print("name_coverage:", named, "/", len(feats))

    ranks = sorted(k for k in sr if k is not None)
    print("cumulative_counts_by_scalerank_threshold (scalerank <= t):")
    for t in ranks:
        c = sum(v for k, v in sr.items() if k is not None and k <= t)
        print(f"  <= {t}: {c}")

    # marquee lakes
    targets = ["Caspian", "Superior", "Michigan", "Huron", "Erie", "Ontario",
               "Baikal", "Baykal", "Victoria", "Tanganyika", "Ladoga", "Balkhash",
               "Great Bear", "Great Slave", "Winnipeg", "Titicaca", "Aral", "Chad",
               "Malawi", "Nyasa"]
    print("marquee_lakes:")
    seen = []
    for f in feats:
        p = f["properties"]
        nm = p.get("name") or ""
        for t in targets:
            if t.lower() in nm.lower():
                seen.append((nm, p.get("scalerank"), p.get("featurecla"),
                             f["geometry"]["type"] if f["geometry"] else None))
                break
    for row in sorted(set(seen)):
        print("  name=%r scalerank=%s featurecla=%r geom=%s" % row)

if __name__ == "__main__":
    probe_rivers()
    probe_lakes()
