# probe_hydro2.py
# Follow-up probe: min_zoom/min_label distributions for rivers, counts split by
# featurecla (River vs Lake Centerline), Caspian search in lakes, lakes min_zoom.
# Inputs: same as probe_hydro.py. Invocation: python probe_hydro2.py
import json
from collections import Counter

DATA = "C:/Users/ridge/Spheres/spheres-web/data"

with open(f"{DATA}/ne_10m_rivers_lake_centerlines.geojson", encoding="utf-8") as f:
    rivers = json.load(f)["features"]
with open(f"{DATA}/ne_10m_lakes.geojson", encoding="utf-8") as f:
    lakes = json.load(f)["features"]

print("=== RIVERS follow-up ===")
mz = Counter(f["properties"].get("min_zoom") for f in rivers)
print("min_zoom_distribution:", dict(sorted(mz.items())))
ml = Counter(f["properties"].get("min_label") for f in rivers)
print("min_label_distribution:", dict(sorted(ml.items(), key=lambda x: (x[0] is None, x[0]))))

riv_only = [f for f in rivers if f["properties"]["featurecla"] == "River"]
print("featurecla=='River' only count:", len(riv_only))
sr_r = Counter(f["properties"]["scalerank"] for f in riv_only)
print("cumulative scalerank thresholds, featurecla=='River' only:")
for t in sorted(set(sr_r)):
    print(f"  <= {t}: {sum(v for k, v in sr_r.items() if k <= t)}")

print("cumulative min_zoom thresholds (all 1455 features, min_zoom <= t):")
for t in sorted(set(mz)):
    print(f"  <= {t}: {sum(v for k, v in mz.items() if k <= t)}")

# distinct named river systems at candidate scalerank thresholds
for t in (4, 5, 6):
    names = sorted({(f["properties"].get("name_en") or f["properties"].get("name") or "?")
                    for f in rivers if f["properties"]["scalerank"] <= t})
    print(f"distinct name_en at scalerank<={t}: {len(names)}")

# unnamed features at candidate thresholds
for t in (4, 5, 6):
    unnamed = sum(1 for f in rivers
                  if f["properties"]["scalerank"] <= t and not f["properties"].get("name"))
    print(f"unnamed features at scalerank<={t}: {unnamed}")

# dissolve field sample values
dv = Counter()
for f in rivers[:50]:
    dv[f["properties"].get("dissolve")] += 1
print("dissolve_sample_values(first50):", dict(sorted(dv.items(), key=lambda x: str(x[0]))))

print()
print("=== LAKES follow-up ===")
# Caspian search across name fields
for f in lakes:
    p = f["properties"]
    hay = " ".join(str(p.get(k) or "") for k in ("name", "name_en", "name_alt", "label", "name_abb"))
    if "caspian" in hay.lower():
        print("CASPIAN found: name=%r name_en=%r label=%r scalerank=%s featurecla=%r geom=%s" %
              (p.get("name"), p.get("name_en"), p.get("label"), p.get("scalerank"),
               p.get("featurecla"), f["geometry"]["type"]))

mzl = Counter(f["properties"].get("min_zoom") for f in lakes)
print("lakes min_zoom_distribution:", dict(sorted(mzl.items(), key=lambda x: (x[0] is None, float(x[0]) if x[0] is not None else 0))))

# name_en coverage for lakes
ne_cov = sum(1 for f in lakes if f["properties"].get("name_en"))
print("lakes name_en_coverage:", ne_cov, "/", len(lakes))

# named coverage by scalerank
print("lakes named (name field) per scalerank:")
for t in sorted({f["properties"]["scalerank"] for f in lakes}):
    tot = sum(1 for f in lakes if f["properties"]["scalerank"] == t)
    nam = sum(1 for f in lakes if f["properties"]["scalerank"] == t and f["properties"].get("name"))
    print(f"  rank {t}: {nam}/{tot} named")

# lakes at scalerank<=1: list names to sanity check what a tight threshold keeps
print("lakes at scalerank<=1 (names):")
for f in sorted(lakes, key=lambda f: (f["properties"]["scalerank"], str(f["properties"].get("name")))):
    p = f["properties"]
    if p["scalerank"] <= 1:
        print("  rank=%s name=%r featurecla=%r" % (p["scalerank"], p.get("name"), p.get("featurecla")))
