#!/usr/bin/env python
"""crossing_edges.py — river-crossed adjacency edges for Spheres.

INPUTS (read-only, anchored on the repo root this file sits in):
  spheres-web/data/river_segments.json
      filtered rivers (scalerank <= 5.0), RAW lon/lat parts as emitted
      by make_rivers.py (unsimplified, 3-decimal) — run make_rivers.py first
  spheres-web/data/ne_10m_admin_1.geojson
      admin-1 source polygons (via classify_districts.derive_districts, which
      replicates mapgen.rs district ids exactly — validated 2610/2610)
  spheres-sim/data/districts.json
      authoritative adjacency ("adj" lists) — 6,106 undirected edges

OUTPUT:
  spheres-web/data/crossing_edges.json
      {"rule", "eps_deg", "count", "edges": [[a, b], ...]}
      each pair sorted lexicographically, list of pairs sorted; UTF-8, stable.
      Committed; mapgen merges it into spheres-sim/data/districts.json as
      per-district "riv" lists (the sim never reads this file itself).

INVOCATION:
  python tools/terrain/crossing_edges.py     (no args, no RNG)

RULE (documented choice):
  An adjacency edge (A, B) is river-crossed when some filtered river PART
  (one polyline between junctions) passes within the shared boundary region
  of A and B. Cheap robust test, per candidate (part, A, B):

      crossed  iff  (part∩A nonempty and dist(part∩A, B) <= EPS)
                 or (part∩B nonempty and dist(part∩B, A) <= EPS)

  i.e. the piece of the river inside one district comes within EPS degrees of
  the other district. This catches both geometries that matter:
    * perpendicular crossings (river flows from A into B: the clipped piece
      touches the shared edge, distance 0), and
    * border-following rivers (Rio Grande, Rhine: the centerline wiggles a
      hair to one side of the admin boundary; the clipped piece stays within
      EPS of the far district).
  It rejects rivers that visit A and B far apart (via a third district):
  there the piece inside A stays far from B. EPS = 0.05 deg (~5.5 km at the
  equator, less at temperate latitudes) absorbs Natural Earth's river-vs-
  boundary digitisation mismatch; the eps sweep printed by this script shows
  counts are not knife-edge sensitive around that value.
  Known miss (accepted): a river lying wholly in a data gap strip between two
  sloppily-digitised national polygons intersects neither, so neither clip
  exists. NE admin-1 borders are built on the same centerlines, so in
  practice intersection occurs; the sweep confirms no cliff at small EPS.

  Candidate generation: STRtree over district geometries; for each river part
  take districts within EPS_MAX = 0.1 deg of the part, then test every
  adjacency edge with both endpoints in that set. Per-edge minimum distance
  is kept, so counts at every eps in the sweep come from one pass.

Deterministic: no RNG, fixed thresholds, sorted output.
"""

import json
import os
import sys
from collections import defaultdict

import shapely
from shapely.geometry import LineString
from shapely.strtree import STRtree

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from classify_districts import derive_districts, district_parts  # noqa: E402

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
DISTRICTS_JSON = os.path.join(ROOT, "spheres-sim/data/districts.json")
SEGMENTS = os.path.join(ROOT, "spheres-web/data/river_segments.json")
OUT = os.path.join(ROOT, "spheres-web/data/crossing_edges.json")

EPS = 0.05           # degrees — the chosen rule threshold
EPS_SWEEP = [0.01, 0.02, 0.03, 0.05, 0.08, 0.10]
EPS_MAX = 0.10       # candidate window


def main():
    # --- district geometries (mapgen-exact ids) ---
    raw = derive_districts()
    with open(DISTRICTS_JSON, encoding="utf-8") as f:
        roster = json.load(f)
    game_ids = {d["id"] for ds in roster["nations"].values() for d in ds}

    dist_ids = []
    dist_geoms = []
    for did in sorted(raw):
        if did not in game_ids:
            continue
        parts = district_parts(raw[did])
        if not parts:
            continue
        g = shapely.union_all(parts) if len(parts) > 1 else parts[0]
        dist_ids.append(did)
        dist_geoms.append(g)
    geom_of = dict(zip(dist_ids, dist_geoms))
    print(f"district geometries: {len(dist_ids)} / {len(game_ids)}")

    # --- adjacency edges ---
    adj_edges = set()
    for ds in roster["nations"].values():
        for d in ds:
            for n in d["adj"]:
                adj_edges.add(tuple(sorted((d["id"], n))))
    edges_of = defaultdict(set)   # district id -> incident edges
    for e in adj_edges:
        edges_of[e[0]].add(e)
        edges_of[e[1]].add(e)
    print(f"adjacency edges: {len(adj_edges)}")

    # --- river parts ---
    with open(SEGMENTS, encoding="utf-8") as f:
        rivers = json.load(f)["rivers"]
    part_lines, part_name = [], []
    for r in rivers:
        for part in r["parts"]:
            if len(part) >= 2:
                part_lines.append(LineString(part))
                part_name.append(r["name"])
    print(f"river parts: {len(part_lines)} from {len(rivers)} rivers")

    tree = STRtree(dist_geoms)

    # --- per-edge minimum "shared-boundary" distance ---
    edge_min = {}                 # edge -> min distance metric
    edge_rivers = defaultdict(set)  # edge -> river names at <= EPS
    for pi, line in enumerate(part_lines):
        cand = tree.query(line)   # bbox candidates
        near = []
        for gi in cand.tolist():
            if line.distance(dist_geoms[gi]) <= EPS_MAX:
                near.append(dist_ids[gi])
        if len(near) < 2:
            continue
        near_set = set(near)
        clips = {}
        for did in near:
            c = line.intersection(geom_of[did])
            clips[did] = None if c.is_empty else c
        seen = set()
        for did in near:
            for e in edges_of[did]:
                if e in seen:
                    continue
                seen.add(e)
                a, b = e
                if a not in near_set or b not in near_set:
                    continue
                d = None
                if clips[a] is not None:
                    d = clips[a].distance(geom_of[b])
                if clips[b] is not None:
                    d2 = clips[b].distance(geom_of[a])
                    d = d2 if d is None else min(d, d2)
                if d is None:
                    continue
                if e not in edge_min or d < edge_min[e]:
                    edge_min[e] = d
                if d <= EPS and part_name[pi]:
                    edge_rivers[e].add(part_name[pi])

    # --- eps sweep ---
    print("eps sweep (crossed-edge counts):")
    for eps in EPS_SWEEP:
        n = sum(1 for d in edge_min.values() if d <= eps)
        mark = "  <-- chosen" if eps == EPS else ""
        print(f"  eps={eps:.2f}  {n}{mark}")

    crossed = sorted(e for e, d in edge_min.items() if d <= EPS)

    # --- sanity: Rhine and Rio Grande ---
    for rname in ("Rhine", "Rio Grande"):
        hits = sorted(e for e in crossed if rname in edge_rivers[e])
        print(f"sanity {rname}: {len(hits)} edges")
        for e in hits:
            print(f"    {e[0]} -- {e[1]}")

    out = {
        "rule": ("adjacency edge (A,B) is river-crossed iff a filtered river "
                 "part's clip inside one district comes within eps degrees of "
                 "the other district (shared-boundary-region test; see "
                 "tools/terrain/crossing_edges.py)"),
        "eps_deg": EPS,
        "count": len(crossed),
        "edges": [list(e) for e in crossed],
    }
    with open(OUT, "w", encoding="utf-8", newline="\n") as f:
        json.dump(out, f, ensure_ascii=False, indent=1)
        f.write("\n")
    print(f"wrote {OUT}: {len(crossed)} crossed edges of {len(adj_edges)}")


if __name__ == "__main__":
    main()
