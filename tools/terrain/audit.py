#!/usr/bin/env python
# audit.py -- hands-on audit of the committed terrain artifacts (read-only).
# A quick structural dump for eyeballing; check.py is the pass/fail gate.
# Invocation: python tools/terrain/audit.py   (deterministic, no RNG)
import json
import os
import re

from PIL import Image

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
SIM = os.path.join(ROOT, "spheres-sim/data/districts.json")

out = {}

# 1. district id validation
with open(os.path.join(ROOT, "spheres-web/data/district_terrain.json"), encoding="utf-8") as f:
    terr = json.load(f)
with open(SIM, encoding="utf-8") as f:
    dist = json.load(f)
roster = set()
for nation, dl in dist["nations"].items():
    for d in dl:
        roster.add(d["id"])
tkeys = set(terr.keys())
out["districts_json_ids"] = len(roster)
out["district_terrain_ids"] = len(tkeys)
out["matched"] = len(roster & tkeys)
out["missing_from_terrain"] = sorted(roster - tkeys)[:10]
out["extra_in_terrain"] = sorted(tkeys - roster)[:10]
from collections import Counter
out["class_histogram"] = dict(sorted(Counter(v["t"] for v in terr.values()).items()))
out["named_feature_count"] = sum(1 for v in terr.values() if v["f"])

# 2. rivers.js path sampling
with open(os.path.join(ROOT, "spheres-web/ui/rivers.js"), encoding="utf-8") as f:
    body = f.read().split("window.RIVERS=", 1)[1].rstrip().rstrip(";")
rr = json.loads(re.sub(r'([{,])(meta|rivers|lakes|[nwhd])\s*:', r'\1"\2":', body))
num = re.compile(r"-?\d+\.?\d*")
def path_coords(p):
    vals = [float(x) for x in num.findall(p)]
    return list(zip(vals[::2], vals[1::2]))
xs, ys, bad = [], [], 0
tok = re.compile(r"^(M-?[\d.]+ -?[\d.]+(L-?[\d.]+ -?[\d.]+| -?[\d.]+ -?[\d.]+)*)+$")
for r in rr["rivers"]:
    if not tok.match(r["d"]):
        bad += 1
    for x, y in path_coords(r["d"]):
        xs.append(x); ys.append(y)
out["rivers_count"] = len(rr["rivers"])
out["lakes_count"] = len(rr.get("lakes", []))
out["river_paths_bad_syntax"] = bad
out["river_coord_bbox"] = [min(xs), min(ys), max(xs), max(ys)]
out["river_named"] = sum(1 for r in rr["rivers"] if r["n"])
samp = [rr["rivers"][i] for i in (0, len(rr["rivers"]) // 2, len(rr["rivers"]) - 1)]
out["river_samples"] = [{"n": s["n"], "path_head": s["d"][:60]} for s in samp]
names = {r["n"] for r in rr["rivers"] if r["n"]}
out["marquee_present"] = {n: (n in names) for n in ("Rhine", "Rio Grande", "Danube", "Mekong", "Nile")}

# river_segments.json structural sample
with open(os.path.join(ROOT, "spheres-web/data/river_segments.json"), encoding="utf-8") as f:
    seg = json.load(f)
out["segments_count"] = len(seg["rivers"])
lons, lats, pts = [], [], 0
for r in seg["rivers"]:
    for part in r["parts"]:
        for lon, lat in part:
            pts += 1
            lons.append(lon); lats.append(lat)
out["segments_total_points"] = pts
out["segments_lonlat_bbox"] = [min(lons), min(lats), max(lons), max(lats)]
segnames = {r["name"] for r in seg["rivers"] if r["name"]}
out["segments_marquee"] = {n: (n in segnames) for n in ("Rhine", "Rio Grande")}

# 3. crossing edges
with open(os.path.join(ROOT, "spheres-web/data/crossing_edges.json"), encoding="utf-8") as f:
    ce = json.load(f)
out["crossing_edges"] = {"count": ce["count"], "eps_deg": ce["eps_deg"],
                         "head": ce["edges"][:3], "tail": ce["edges"][-3:]}

# 4. PNG
im = Image.open(os.path.join(ROOT, "spheres-web/ui/terrain.png"))
im.load()
out["terrain.png"] = {"size": list(im.size), "mode": im.mode}

print(json.dumps(out, indent=1))
