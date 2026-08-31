#!/usr/bin/env python
"""check.py — verifies spheres-web/data/district_population.json against ground truth.

Runs LAST, the way tools/terrain/check.py does. Every check below either passes
or fails the process; none of them can be satisfied by widening a tolerance,
because each tolerance is pinned to an independently sourced number.

  1  COVERAGE          every district in districts.json has a share, and no
                       share exists for a district that is not in the roster.
                       2,610 unique ids, 2,985 (nation, district) pairs.
  2  SHARE SUMS        every nation's shares sum to 1 within 1e-12.
  3  HEAD COUNT SUMS   every nation's district head counts sum EXACTLY to
                       round(economy.population_m * 1e6). Largest-remainder
                       apportionment makes this an equality, not a tolerance.
  4  DOUBLE MEMBERSHIP the 375 ids held by two nations carry a DIFFERENT share
                       under each, and each nation's own set still sums to 1.
                       This is the check that catches a flat id->share map.
  5  RASTERIZER        the coverage masks are re-used to reconstruct district
                       AREA with cos-latitude cell areas, and compared against
                       mapgen.rs's separately computed `area_sqkm`. The two
                       paths share only the source polygons, so agreement is
                       evidence the zonal pass is not silently mis-registered.
                       Checked across five orders of magnitude, 1% tolerance.
  6  CENSUS            pinned against real 1990 censuses, which are external to
                       every input this pipeline reads.
  7  ORDERING          the questions a designer would actually ask: does Moscow
                       outweigh Chukotka, Tokyo outweigh Tottori, Uttar Pradesh
                       outweigh Sikkim, Java outweigh Papua, Cairo outweigh the
                       New Valley. Pinned as inequalities with margin.
  8  BYTE-IDENTITY     re-runs the generator into a temp sibling (never over the
                       committed artifact) and compares sha256.

Usage (from the repo root):
    python tools/population/check.py            # all checks
    python tools/population/check.py --fast     # skip 5 and 8 (the slow ones)
"""
import hashlib
import json
import math
import os
import subprocess
import sys

import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(HERE))
sys.path.insert(0, HERE)
import make_population as mp  # noqa: E402

ART = os.path.join(ROOT, "spheres-web/data/district_population.json")
DISTRICTS = os.path.join(ROOT, "spheres-sim/data/districts.json")
PREFIX = os.path.join(HERE, "raster/ghspop1990")

R_KM = 6371.0088          # IUGG mean Earth radius
AREA_TOL = 0.01           # 1% — the two area paths share only the polygons

FAIL = []


def ok(cond, label, detail=""):
    print(("  PASS  " if cond else "  FAIL  ") + label + (("  " + detail) if detail else ""))
    if not cond:
        FAIL.append(label)


# --- 6. real 1990 censuses, external to every input here --------------------
#     ([districts summed], census figure in millions, source, tolerance)
#
# Districts are summed where a modern Natural Earth boundary splits a 1990 unit,
# so the check tests the RASTER rather than the boundary vintage: undivided Uttar
# Pradesh is today's IN-UP plus IN-UT, and comparing IN-UP alone against the 1991
# undivided figure would fail for a reason that has nothing to do with this file.
CENSUS_M = [
    (["US-CA"], 29.760021, "1990 US Census, California", 0.03),
    (["US-TX"], 16.986510, "1990 US Census, Texas", 0.05),
    (["US-WY"], 0.453588, "1990 US Census, Wyoming", 0.05),
    (["RU-MOS"], 8.967332, "1989 Soviet Census, Moscow city", 0.05),
    (["IN-UP", "IN-UT"], 139.112287,
     "1991 Census of India, undivided Uttar Pradesh", 0.08),
    (["JP-13"], 11.855563, "1990 Japan Census, Tokyo-to", 0.12),
    (["CA-ON"], 10.084885, "1991 Census of Canada, Ontario", 0.08),
    (["CN-HA"], 85.509535, "1990 China Census, Henan", 0.10),
]

# --- 7. the ordering questions a designer would actually ask ----------------
#     (nation, heavy district, light district, minimum ratio, what it means)
#
# These are the checks that would catch an unweighted or area-weighted mean
# sneaking back in: every pair below is one where AREA points the other way.
ORDERING = [
    ("Russia", "RU-MOS", "RU-CHU", 50.0, "Moscow city vs Chukotka"),
    ("Russia", "RU-MOW", "RU-CHU", 40.0, "Moscow oblast vs Chukotka"),
    ("Japan", "JP-13", "JP-31", 10.0, "Tokyo vs Tottori"),
    ("India", "IN-UP", "IN-SK", 100.0, "Uttar Pradesh vs Sikkim"),
    ("Indonesia", "ID-JI", "ID-PA", 25.0, "East Java vs Papua"),
    ("Indonesia", "ID-JB", "ID-PB", 50.0, "West Java vs West Papua"),
    ("Egypt", "EG-C", "EG-WAD", 20.0, "Cairo vs the New Valley"),
    ("China", "CN-HA", "CN-XZ", 10.0, "Henan vs Tibet"),
    ("Canada", "CA-ON", "CA-NU", 100.0, "Ontario vs Nunavut"),
]


def main():
    fast = "--fast" in sys.argv
    with open(ART, encoding="utf-8") as f:
        art = json.load(f)
    with open(DISTRICTS, encoding="utf-8") as f:
        roster = json.load(f)["nations"]
    nations = art["nations"]
    counts = art["counts"]

    print("1  COVERAGE")
    pairs_roster = {(n, d["id"]) for n in roster for d in roster[n]}
    pairs_art = {(n, d) for n in nations for d in nations[n]["districts"]}
    ids_roster = {d for _, d in pairs_roster}
    ok(pairs_art == pairs_roster, "(nation, district) pairs match districts.json",
       "%d pairs" % len(pairs_art))
    ok(set(counts) == ids_roster, "counts covers exactly the roster ids",
       "%d ids" % len(counts))
    ok(len(ids_roster) == 2610, "2,610 unique districts", str(len(ids_roster)))
    ok(len(pairs_roster) == 2985, "2,985 (nation, district) pairs",
       str(len(pairs_roster)))
    missing = [d for d in counts if counts[d] is None]
    ok(not missing, "no district lacks a head count")

    print("2  SHARE SUMS")
    worst, worst_n = 0.0, None
    for n, rec in nations.items():
        s = sum(v["share"] for v in rec["districts"].values())
        if abs(s - 1.0) > worst:
            worst, worst_n = abs(s - 1.0), n
    ok(worst < 1e-12, "every nation's shares sum to 1",
       "max |sum-1| = %.3e (%s)" % (worst, worst_n))

    print("3  HEAD COUNT SUMS (exact, not a tolerance)")
    bad = []
    checked = 0
    for n, rec in nations.items():
        lvl = rec["transcribed_population_m"]
        if lvl is None:
            if any(v["pop_1990"] is not None for v in rec["districts"].values()):
                bad.append(n + " has head counts without a transcribed level")
            continue
        want = int(round(lvl * 1e6))
        got = sum(v["pop_1990"] for v in rec["districts"].values())
        checked += 1
        if got != want:
            bad.append("%s: %d != %d" % (n, got, want))
    ok(not bad, "head counts sum exactly to the transcribed total",
       "%d nations exact%s" % (checked, "" if not bad else "; " + "; ".join(bad[:3])))

    print("4  DOUBLE MEMBERSHIP (the flat-map trap)")
    holders = {}
    for n in nations:
        for d in nations[n]["districts"]:
            holders.setdefault(d, []).append(n)
    dual = {d: ns for d, ns in holders.items() if len(ns) > 1}
    ok(len(dual) == 375, "375 ids belong to two nations", str(len(dual)))
    # A dual-held id must weigh DIFFERENTLY under its two holders — that is the
    # whole reason `nations` is keyed by nation and not by district alone. The
    # one exception is an uninhabited district, whose share is 0 under both by
    # arithmetic rather than by collapse; assert that explicitly instead of
    # loosening the rule.
    same = [d for d, ns in dual.items()
            if len({nations[n]["districts"][d]["share"] for n in ns}) == 1]
    ok(all(counts[d] == 0.0 for d in same),
       "dual-held ids differ under each holder, except uninhabited ones",
       "%d/%d differ; identical only for %s (all zero-population)"
       % (len(dual) - len(same), len(dual), sorted(same) or "none"))
    ok(all(nations[n]["districts"][d]["share"] == 0.0 for d in same for n in dual[d]),
       "the identical ones are identically ZERO, not identically collapsed")
    if "RU-MOS" in dual:
        ru = nations["Russia"]["districts"]["RU-MOS"]["share"]
        su = nations["USSR"]["districts"]["RU-MOS"]["share"]
        ok(ru > su * 1.5, "RU-MOS weighs more in Russia than in the USSR",
           "%.4f vs %.4f" % (ru, su))

    print("5  RASTERIZER — area reconstructed from the same coverage masks")
    if fast:
        print("  SKIP  (--fast)")
    else:
        with open(PREFIX + ".json", encoding="utf-8") as f:
            geo = json.load(f)
        districts = mp.cd.derive_districts()
        area_by_id = {}
        for n in roster:
            for d in roster[n]:
                area_by_id[d["id"]] = d["area_sqkm"]
        sy, sx = geo["sy"], geo["sx"]
        y0 = geo["y0"]
        # cell area of raster row j, exact on a sphere:
        #   R^2 * dlon * (sin(lat_top) - sin(lat_bottom))
        dlon = math.radians(sx)
        probe = ["RU-MOS", "ID-JK", "NG-LA", "EG-ASN", "US-AK", "AU-WA", "CA-NU"]
        for did in probe:
            if did not in districts:
                ok(False, "probe district %s missing" % did)
                continue
            tot = 0.0
            for poly in districts[did]:
                got = mp.polygon_mask(poly, geo)
                if got is None:
                    continue
                by0, _bx0, cov = got
                bh = cov.shape[0]
                j = np.arange(by0, by0 + bh, dtype=np.float64)
                lat_t = np.radians(y0 - j * sy)
                lat_b = np.radians(y0 - (j + 1) * sy)
                cell = (R_KM ** 2) * dlon * (np.sin(lat_t) - np.sin(lat_b))
                tot += float((cov.sum(axis=1, dtype=np.float64) * cell).sum())
            ref = area_by_id[did]
            r = tot / ref
            ok(abs(r - 1.0) < AREA_TOL,
               "%-7s area %10.0f km2 vs mapgen %10.0f" % (did, tot, ref),
               "ratio %.4f" % r)

    print("6  CENSUS (external to every input)")
    for dids, census, src, tol in CENSUS_M:
        if any(d not in counts for d in dids):
            ok(False, "%s absent" % "+".join(dids))
            continue
        got = sum(counts[d] for d in dids) / 1e6
        r = got / census
        ok(abs(r - 1.0) <= tol, "%-14s GHS %8.3fm vs %8.3fm — %s"
           % ("+".join(dids), got, census, src),
           "ratio %.3f (tol %.0f%%)" % (r, tol * 100))

    print("7  ORDERING — does the weighting invert the country?")
    for nation, heavy, light, ratio, what in ORDERING:
        rec = nations.get(nation, {}).get("districts", {})
        if heavy not in rec or light not in rec:
            ok(False, "%s: %s or %s absent" % (nation, heavy, light))
            continue
        h, l = rec[heavy]["share"], rec[light]["share"]
        got = h / l if l > 0 else float("inf")
        ok(got >= ratio, "%-28s %s/%s" % (what, heavy, light),
           "%.1fx (need >= %.0fx)" % (got, ratio))

    print("8  BYTE-IDENTITY")
    if fast:
        print("  SKIP  (--fast)")
    else:
        tmp = ART + ".checktmp.json"
        sha0 = hashlib.sha256(open(ART, "rb").read()).hexdigest()
        subprocess.check_call([sys.executable,
                               os.path.join(HERE, "make_population.py"),
                               PREFIX, tmp], stdout=subprocess.DEVNULL)
        sha1 = hashlib.sha256(open(tmp, "rb").read()).hexdigest()
        os.remove(tmp)
        ok(sha0 == sha1, "regenerating reproduces the committed file byte for byte",
           sha0[:16] + "..." if sha0 == sha1 else "%s vs %s" % (sha0[:16], sha1[:16]))

    print()
    if FAIL:
        print("FAILED %d check(s):" % len(FAIL))
        for f in FAIL:
            print("  -", f)
        sys.exit(1)
    print("all checks passed")


if __name__ == "__main__":
    main()
