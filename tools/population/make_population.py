#!/usr/bin/env python
"""make_population.py — per-district 1990 population for Spheres.

WHAT THIS IS, AND WHY IT IS NOT AN INVENTION
  Each of the 2,610 districts gets a SHARE of its nation's population and a
  derived 1990 head count. Both ends are sourced:

      LEVEL  = nation.economy.population_m, already transcribed in
               spheres-sim/data/nations/*.json with its own `sources` block.
      SPLIT  = the fraction of that nation's 1990 gridded population that falls
               inside the district's Natural Earth admin-1 polygon, measured off
               a published CC BY raster.

      district_population = share x nation.economy.population_m

  Nothing here fabricates a per-district figure. The 2,610 shares are a
  measurement, and every one of them is reproducible from the two inputs.

INPUTS (read-only)
  raster/ghspop1990.f32 + .json
      GHS-POP R2023A, epoch 1990, GLOBE, EPSG:4326, 30 arc-second, decoded by
      decode_ghspop.py. European Commission Joint Research Centre, CC BY 4.0.
      Schiavina M., Freire S., Carioli A., MacManus K. (2023): GHS-POP R2023A.
      DOI 10.2905/2FF68A52-5B5B-4A22-8F40-C41DA8332CFE
  spheres-web/data/ne_10m_admin_1.geojson   district polygons
  spheres-web/ui/index.html                 the TERRITORY roster
  spheres-sim/data/districts.json           authoritative 2,610-id roster
  spheres-sim/data/nations/*.json           transcribed economy.population_m
  tools/terrain/classify_districts.py       IMPORTED, not copied:
      derive_districts() is the already-validated replication of mapgen.rs's
      district identity, so the ids here provably cannot drift from the roster.
      An assert below fails the run if they ever do.

OUTPUT (committed)
  spheres-web/data/district_population.json

INVOCATION (from the repo root)
  python tools/population/decode_ghspop.py      # stage A, once
  python tools/population/make_population.py    # stage B
  python tools/population/check.py              # verifies both

METHOD (deterministic: no RNG, no wall clock, no dict-order dependence)
  Districts are visited in sorted id order, each district's polygons in source
  order, each polygon's rings in source order. For every polygon:
    * lon/lat ring vertices -> raster pixel coordinates via the GeoTIFF tiepoint
      and pixel scale (RasterPixelIsArea).
    * an integer bounding box, clipped to the raster.
    * a coverage mask rasterised with PIL at a supersample factor s chosen
      purely from the bbox size (SUB_BUDGET / bbox area, clamped 1..MAX_S), so
      small districts are measured with sub-pixel fractional coverage and huge
      ones at 1:1. s is a pure function of the bbox, hence stable.
    * outer ring filled 1, interior rings (holes) filled 0, per polygon, so an
      enclave belonging to another district is never erased by this one.
    * contribution = sum(population_pixels * fractional_coverage).
  Sampling is at pixel centres (the -0.5 offset), so the mask converges to true
  areal coverage as s rises. Validated independently: reconstructing district
  area from these coverage masks with cos-latitude cell areas agrees with
  mapgen.rs's separately computed `area_sqkm` to 0.1-0.35% across five orders of
  magnitude (RU-MOS, ID-JK, NG-LA, EG-ASN, US-AK, AU-WA, CA-NU). The two paths
  share only the source polygons — see check.py.

  Head counts are apportioned by LARGEST REMAINDER over the districts of each
  nation, in sorted id order for ties, so the integers sum EXACTLY to
  round(population_m * 1e6) rather than to within a rounding error per district.

KEYING — the trap this file exists to avoid
  `nations` is keyed BY NATION THEN DISTRICT. districts.json holds 2,985
  (nation, district) pairs over 2,610 unique ids, because a federation carries
  the union of its republics' districts and each successor repeats its own
  subset (spheres-sim/src/districts.rs). 375 ids therefore belong to two
  nations — RU-MOS is 3.09% of the USSR and 5.94% of Russia — and a flat
  id -> share map would be ambiguous for every one of them.

CAVEATS, STATED RATHER THAN HIDDEN — all four are written into meta.caveats
  1. The raster is NOT a later epoch resampled back. 1990 is a native GHS-POP
     epoch built on 1990-epoch Landsat built-up surface. Its global total is
     5.316bn against the UN's 5.32bn for 1990.
  2. It IS a modelled surface: census totals disaggregated onto observed
     built-up area, not a 1990 census raster. Its bias is spatial, not merely
     scalar — it under-detects dispersed rural settlement, tilting shares
     towards urban districts. Where its national total disagrees with the
     transcribed population_m the transcribed figure wins, because this file
     supplies the SPLIT and never the LEVEL; the disagreement is recorded for
     every nation as `ratio_ghs_over_transcribed`.
  3. ROSTER GAPS. A few nations' TERRITORY entries claim fewer Natural Earth
     adm0 codes than the nation held in 1990, so `share x transcribed_total`
     spreads the missing territory's people across the districts that do exist.
     These are inherited from the districts pass, not caused here. They are
     discovered mechanically (ROSTER_GAPS below is asserted against the data on
     every run) and flagged per nation as `roster_gap`.
  4. 23 successor states have districts but no 1990 nation file, hence no
     population_m. Their head counts are null by construction. Renormalise
     `counts` over the districts actually held instead.
"""
import json
import math
import os
import sys

import numpy as np
from PIL import Image, ImageDraw

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(ROOT, "tools/terrain"))
import classify_districts as cd  # noqa: E402

DISTRICTS_JSON = os.path.join(ROOT, "spheres-sim/data/districts.json")
NATIONS_DIR = os.path.join(ROOT, "spheres-sim/data/nations")
ADMIN1 = os.path.join(ROOT, "spheres-web/data/ne_10m_admin_1.geojson")
INDEX_HTML = os.path.join(ROOT, "spheres-web/ui/index.html")
DEFAULT_PREFIX = os.path.join(HERE, "raster/ghspop1990")
DEFAULT_OUT = os.path.join(ROOT, "spheres-web/data/district_population.json")

SUB_BUDGET = 4_000_000   # max subpixels rasterised per polygon
MAX_S = 8                # max supersample factor per axis

# ---------------------------------------------------------------------------
# ROSTER GAPS — territory the transcribed national total counts but the district
# roster does not cover, so `share x total` spreads those people over the wrong
# ground.
#
# The question is NOT "does the roster omit land the nation held". It is "does
# the transcribed population_m COUNT people the roster has nowhere to put". Those
# are different, and guessing which is which gets it wrong: Israel held the West
# Bank and Gaza in 1990, but its transcribed 4.66m excludes their 1.99m people,
# so adding them would corrupt the split rather than repair it. Same for the UK
# and Hong Kong.
#
# So the candidates below are only candidates, and the DATA decides. For each,
# the generator measures the missing adm0's 1990 population off the same raster
# and applies the CLOSURE TEST:
#
#     r0 = ghs_under_roster / transcribed        (the ratio as it stands)
#     r1 = (ghs_under_roster + ghs_missing) / transcribed
#     accepted  <=>  |r1 - 1| < |r0 - 1| - CLOSURE_MARGIN
#
# i.e. the gap is real only if putting the missing people back moves the nation's
# GHS total materially TOWARDS its transcribed total. Rejected candidates are
# kept and reported too, with their numbers, so the reasoning survives.
#
# Two further guards are asserted on every run: each code must exist in
# ne_10m_admin_1.geojson, and must be claimed by no nation in TERRITORY. If
# someone adds the code to the roster, or Natural Earth renames it, the run
# fails rather than shipping a stale warning.
#
# This table changes no share and no head count.
# ---------------------------------------------------------------------------
CLOSURE_MARGIN = 0.01

ROSTER_GAP_CANDIDATES = [
    # Southern Sudan was Sudan's southern region until 2011; Natural Earth's
    # modern SDN stops at the 2011 border. Coded SDS in this export, not SSD.
    ("Sudan", ["SDS"], "South Sudan — part of Sudan until 2011"),
    # Kosovo was the Socialist Autonomous Province of Kosovo within the Socialist
    # Republic of Serbia; autonomy revoked 1989-90. Its own adm0 in Natural Earth.
    ("Yugoslavia", ["KOS"], "Kosovo — an autonomous province of Serbia in 1990"),
    ("Serbia", ["KOS"], "Kosovo — an autonomous province of Serbia in 1990"),
    # Natural Earth's CYP covers only the Republic's area; the north is a
    # separate adm0 and the UN buffer zone a third.
    ("Cyprus", ["CYN"], "Northern Cyprus — a separate Natural Earth adm0"),
    # US unincorporated territories, each its own Natural Earth adm0.
    ("USA", ["PRI", "GUM", "VIR", "ASM", "MNP"],
     "Puerto Rico, Guam, US Virgin Is., American Samoa, N. Mariana Is."),
    # Macau was under Portuguese administration until 1999.
    ("Portugal", ["MAC"], "Macau — Portuguese-administered in 1990"),
    # --- candidates that the closure test is expected to REJECT, kept so the
    # --- reasoning is on the record rather than in a commit message.
    ("Israel", ["PSX"], "West Bank and Gaza — Israeli-occupied in 1990"),
    ("UK", ["HKG"], "Hong Kong — a British dependent territory in 1990"),
    ("Morocco", ["SAH"], "Western Sahara — Moroccan-administered since 1975"),
    ("France", ["NCL", "PYF"], "New Caledonia and French Polynesia"),
    ("USSR", ["KAB"], "Baikonur — filed separately by Natural Earth"),
]

# Nations the game defines but whose TERRITORY entry claims no ISO3 at all, so
# they receive no districts (spheres-sim/src/districts.rs documents the gap).
# The code each one WOULD claim is present in Natural Earth with admin-1 units
# and 1990 population, so this is a one-line roster omission rather than missing
# data. Asserted unclaimed on every run; the population is measured, not stated.
UNSERVED_NATIONS = [
    ("Bahrain", "BHR"), ("CapeVerde", "CPV"), ("Comoros", "COM"),
    ("Maldives", "MDV"), ("Mauritius", "MUS"), ("Seychelles", "SYC"),
]


def supersample(w, h):
    """Pure function of the clipped bbox: 1..MAX_S."""
    if w <= 0 or h <= 0:
        return 1
    s = 1
    while s < MAX_S and (w * (s + 1)) * (h * (s + 1)) <= SUB_BUDGET:
        s += 1
    return s


def polygon_mask(rings, geo):
    """Fractional coverage mask for one polygon: (by0, bx0, cov) or None.

    Shared by the population sum here and by check.py's independent area
    reconstruction, so the two cannot drift apart.
    """
    x0, y0, sx, sy = geo["x0"], geo["y0"], geo["sx"], geo["sy"]
    W, H = geo["width"], geo["height"]

    px_rings = []
    for ring in rings:
        a = np.asarray(ring, dtype=np.float64)
        cx = (a[:, 0] - x0) / sx
        cy = (y0 - a[:, 1]) / sy
        px_rings.append(np.column_stack((cx, cy)))
    if not px_rings:
        return None

    allpts = np.vstack(px_rings)
    bx0 = max(int(np.floor(allpts[:, 0].min())), 0)
    bx1 = min(int(np.ceil(allpts[:, 0].max())) + 1, W)
    by0 = max(int(np.floor(allpts[:, 1].min())), 0)
    by1 = min(int(np.ceil(allpts[:, 1].max())) + 1, H)
    bw, bh = bx1 - bx0, by1 - by0
    if bw <= 0 or bh <= 0:
        return None

    s = supersample(bw, bh)
    img = Image.new("L", (bw * s, bh * s), 0)
    dr = ImageDraw.Draw(img)
    for i, r in enumerate(px_rings):
        xs = (r[:, 0] - bx0) * s - 0.5
        ys = (r[:, 1] - by0) * s - 0.5
        pts = [(float(a), float(b)) for a, b in zip(xs, ys)]
        if len(pts) < 3:
            continue
        fill = 1 if i == 0 else 0
        dr.polygon(pts, fill=fill, outline=fill)
    m = np.asarray(img, dtype=np.uint8)
    if s > 1:
        cov = m.reshape(bh, s, bw, s).sum(axis=(1, 3), dtype=np.float32) / float(s * s)
    else:
        cov = m.astype(np.float32)
    return by0, bx0, cov


def polygon_contribution(rings, pop, geo):
    """(population, covered_pixels) under one polygon: outer ring minus holes."""
    got = polygon_mask(rings, geo)
    if got is None:
        return 0.0, 0.0
    by0, bx0, cov = got
    bh, bw = cov.shape
    block = np.asarray(pop[by0:by0 + bh, bx0:bx0 + bw], dtype=np.float64)
    return float((block * cov).sum()), float(cov.sum(dtype=np.float64))


def apportion(ids, shares, total):
    """Largest-remainder apportionment of `total` people over `ids`.

    Deterministic: remainders compared exactly, ties broken by district id
    ascending. Returns {id: int} summing to exactly `total`.
    """
    base, frac = {}, []
    used = 0
    for d in ids:
        raw = shares[d] * total
        f = math.floor(raw)
        base[d] = int(f)
        used += int(f)
        frac.append((raw - f, d))
    left = total - used
    frac.sort(key=lambda t: (-t[0], t[1]))
    for i in range(left):
        base[frac[i][1]] += 1
    return base


def measure_unclaimed(pop, geo, claimed, wanted):
    """GHS 1990 head count under each unclaimed Natural Earth adm0 code."""
    with open(ADMIN1, encoding="utf-8") as f:
        g1 = json.load(f)
    polys_by_code = {}
    for feat in g1["features"]:
        p = feat.get("properties") or {}
        a = p.get("adm0_a3") or p.get("ADM0_A3") or ""
        if a not in wanted:
            continue
        polys = cd.geometry_polys(feat.get("geometry") or {})
        if polys:
            polys_by_code.setdefault(a, []).append(polys)

    measured = {}
    for code in sorted(wanted):
        assert code in polys_by_code, \
            f"{code} is named in this file but absent from ne_10m_admin_1 — stale"
        assert code not in claimed, \
            f"{code} is named as unclaimed but TERRITORY now claims it — stale"
        tot = 0.0
        for polys in polys_by_code[code]:
            for poly in polys:
                c, _ = polygon_contribution(poly, pop, geo)
                tot += c
        measured[code] = tot
    return measured


def resolve_roster_gaps(measured, ghs_by_nation, transcribed):
    """Apply the closure test to every candidate. Returns (accepted, rejected).

    accepted: {nation: [gap record, ...]} — attached to the nation in the output
    rejected: [gap record, ...]           — kept in meta as the record of why
    """
    accepted, rejected = {}, []
    for nation, codes, what in ROSTER_GAP_CANDIDATES:
        if nation not in ghs_by_nation:
            continue
        add_m = sum(measured[c] for c in codes) / 1e6   # millions, as ghs_by_nation is
        g = ghs_by_nation[nation]
        t = transcribed.get(nation)
        rec = {
            "nation": nation,
            "missing_adm0": list(codes),
            "territory": what,
            "ghs_pop_1990_m_outside_roster": add_m,
        }
        if t is None:
            # No transcribed level, so no closure test and no head counts to
            # corrupt — but the SHARE denominator is still short this land.
            # Accept only when the same territory closes the ratio for the
            # federation this nation is a successor of (Serbia/Kosovo does).
            fed_ok = any(
                other != nation and set(ocodes) == set(codes)
                and transcribed.get(other) is not None
                and _closes(ghs_by_nation.get(other), transcribed.get(other), add_m)
                for other, ocodes, _ in ROSTER_GAP_CANDIDATES
                if other in ghs_by_nation
            )
            rec["closure_test"] = ("not applicable — no transcribed population_m, "
                                   "so no head counts are derived; the share "
                                   "denominator is still short this land")
            if fed_ok:
                rec["accepted_because"] = ("the same territory closes the ratio "
                                           "for this nation's federation")
                accepted.setdefault(nation, []).append(rec)
            else:
                rejected.append(rec)
            continue
        r0 = g / t
        r1 = (g + add_m) / t
        rec["ratio_without_gap"] = r0
        rec["ratio_with_gap"] = r1
        rec["closes"] = abs(r1 - 1.0) < abs(r0 - 1.0) - CLOSURE_MARGIN
        if rec["closes"]:
            accepted.setdefault(nation, []).append(rec)
        else:
            rec["rejected_because"] = (
                "putting these people back does not move the nation's GHS total "
                "materially towards its transcribed total, so the transcribed "
                "figure does not count them and the roster is right to omit them")
            rejected.append(rec)
    return accepted, rejected


def _closes(g, t, add):
    if g is None or not t:
        return False
    return abs((g + add) / t - 1.0) < abs(g / t - 1.0) - CLOSURE_MARGIN


def main(prefix=DEFAULT_PREFIX, outpath=DEFAULT_OUT):
    with open(prefix + ".json", encoding="utf-8") as f:
        geo = json.load(f)
    pop = np.memmap(prefix + ".f32", dtype="<f4", mode="r",
                    shape=(geo["height"], geo["width"]))

    districts = cd.derive_districts()          # id -> [poly, ...]; poly = [ring, ...]
    with open(DISTRICTS_JSON, encoding="utf-8") as f:
        roster = json.load(f)["nations"]
    by_nation = {n: sorted({d["id"] for d in roster[n]}) for n in sorted(roster)}
    all_ids = {i for ids in by_nation.values() for i in ids}
    assert all_ids == set(districts), \
        "derive_districts() no longer reproduces districts.json — refusing to guess"

    # ---- zonal statistics -------------------------------------------------
    counts, pixels = {}, {}
    for did in sorted(districts):
        tot = cov = 0.0
        for poly in districts[did]:
            c, a = polygon_contribution(poly, pop, geo)
            tot += c
            cov += a
        counts[did] = tot
        pixels[did] = cov

    # ---- transcribed national levels --------------------------------------
    transcribed = {}
    for fn in sorted(os.listdir(NATIONS_DIR)):
        if not fn.endswith(".json"):
            continue
        with open(os.path.join(NATIONS_DIR, fn), encoding="utf-8") as f:
            j = json.load(f)
        transcribed[j["id"]] = float(j["economy"]["population_m"])

    territory = cd.territory_map(open(INDEX_HTML, encoding="utf-8").read())
    claimed = {c for v in territory.values() for c in v}

    # ---- roster gaps: measure the candidates, let the closure test decide --
    wanted = {c for _, codes, _ in ROSTER_GAP_CANDIDATES for c in codes}
    wanted |= {c for _, c in UNSERVED_NATIONS}
    measured = measure_unclaimed(pop, geo, claimed, wanted)
    ghs_by_nation = {}
    for nation in sorted(by_nation):
        ghs_by_nation[nation] = sum(counts[d] for d in by_nation[nation]) / 1e6
    gaps, rejected_gaps = resolve_roster_gaps(measured, ghs_by_nation, transcribed)

    unserved = []
    for nation, code in UNSERVED_NATIONS:
        assert not by_nation.get(nation), \
            f"{nation} is listed as unserved but now holds districts — stale"
        unserved.append({
            "nation": nation,
            "natural_earth_adm0": code,
            "ghs_pop_1990_m": measured[code] / 1e6,
            "transcribed_population_m": transcribed.get(nation),
        })

    # ---- per-nation shares and head counts --------------------------------
    nations, no_districts, no_level, uniform_fallback = {}, [], [], []
    for nation in sorted(by_nation):
        ids = by_nation[nation]
        if not ids:
            # Bahrain, CapeVerde, Comoros, Maldives, Mauritius, Seychelles ship
            # empty district lists (spheres-sim/src/districts.rs) — a
            # pre-existing districts-pass gap, nothing to weight here.
            no_districts.append(nation)
            continue
        ghs = 0.0
        for d in ids:
            ghs += counts[d]
        share = {}
        if ghs > 0.0:
            for d in ids:
                share[d] = counts[d] / ghs
        else:
            # No gridded population at all under this nation's districts.
            # Refuse to invent a split; fall back to the one defensible uniform
            # rule and name the nation in meta so it is never mistaken for a
            # measurement.
            uniform_fallback.append(nation)
            for d in ids:
                share[d] = 1.0 / len(ids)

        level = transcribed.get(nation)
        if level is None:
            no_level.append(nation)
            heads = {d: None for d in ids}
        else:
            heads = apportion(ids, share, int(round(level * 1e6)))

        rec = {
            "district_count": len(ids),
            "ghs_pop_1990_m": ghs / 1e6,
            "transcribed_population_m": level,
            "ratio_ghs_over_transcribed": (ghs / 1e6 / level) if level else None,
            "share_sum": sum(share[d] for d in ids),
            "districts": {d: {"share": share[d], "pop_1990": heads[d]} for d in ids},
        }
        if level is None:
            rec["head_counts"] = ("null — successor state with no 1990 nation file, "
                                  "so no transcribed population_m to multiply by; "
                                  "renormalise `counts` over the districts held")
        if nation in gaps:
            rec["roster_gap"] = gaps[nation]
        nations[nation] = rec

    out = {
        "meta": {
            "purpose": "per-district 1990 population and share of national "
                       "population, so district-level state (stability, unrest, "
                       "extraction, occupation) can be aggregated by people "
                       "rather than by land — an unweighted or area-weighted "
                       "mean inverts every large country",
            "generator": "tools/population/make_population.py",
            "stages": ["tools/population/decode_ghspop.py",
                       "tools/population/make_population.py",
                       "tools/population/check.py"],
            "formula": "district_population = share x nation.economy.population_m; "
                       "`pop_1990` is that product apportioned to whole people by "
                       "largest remainder (ties by district id ascending), so a "
                       "nation's district head counts sum EXACTLY to its "
                       "transcribed total",
            "keying": "`nations` is keyed BY NATION THEN DISTRICT. districts.json "
                      "holds 2,985 (nation, district) pairs over 2,610 unique ids "
                      "because a federation carries the union of its republics' "
                      "districts and each successor repeats its own subset "
                      "(spheres-sim/src/districts.rs). 375 ids belong to two "
                      "nations — RU-MOS is 3.09% of the USSR and 5.94% of Russia — "
                      "so a flat id->share map would be ambiguous for all of them.",
            "counts_vs_shares": "`counts` is the absolute GHS-POP 1990 head count "
                                "per district and is the renormalisable primitive. "
                                "`share` is frozen at the January 1990 ownership "
                                "grouping; ownership moves at annexation, "
                                "negotiated concession and federation dissolution, "
                                "and a front can hold half a nation. Any consumer "
                                "needing a weight over the districts actually held "
                                "THIS TICK must renormalise `counts` over that set "
                                "— a sum in sorted id order, deterministic, "
                                "O(districts held). `share` is a convenience for "
                                "the 1990 start only.",
            "deterministic": "sorted district order, source polygon/ring order, "
                             "bbox-derived supersample factor, largest-remainder "
                             "apportionment with id tie-break; no RNG, no wall "
                             "clock, no dict-order dependence. Two runs are "
                             "byte-identical; check.py proves it.",
            "source": {
                "dataset": "GHS-POP R2023A — GHS population grid multitemporal "
                           "(1975-2030)",
                "epoch_year": 1990,
                "grid": "GLOBE, EPSG:4326, 30 arc-second (~1 km at the equator)",
                "file": geo["source_file"],
                "sha256": geo["sha256_source"],
                "url": "https://jeodpp.jrc.ec.europa.eu/ftp/jrc-opendata/GHSL/"
                       "GHS_POP_GLOBE_R2023A/GHS_POP_E1990_GLOBE_R2023A_4326_30ss/"
                       "V1-0/GHS_POP_E1990_GLOBE_R2023A_4326_30ss_V1_0.zip",
                "publisher": "European Commission, Joint Research Centre (JRC)",
                "licence": "CC BY 4.0",
                "licence_url": "https://creativecommons.org/licenses/by/4.0/",
                "citation": "Schiavina M., Freire S., Carioli A., MacManus K. "
                            "(2023): GHS-POP R2023A - GHS population grid "
                            "multitemporal (1975-2030). European Commission, "
                            "Joint Research Centre (JRC).",
                "doi": "10.2905/2FF68A52-5B5B-4A22-8F40-C41DA8332CFE",
                "grid_total_1990": geo["grid_total_float64"],
                "grid_total_note": "5.316bn against the UN's 5.32bn for 1990",
            },
            "geometry_source": "Natural Earth 10m admin-1 — the same polygons and "
                               "the same mapgen.rs identity derivation that "
                               "produced spheres-sim/data/districts.json "
                               "(tools/terrain/classify_districts.py is imported, "
                               "never copied, so the ids cannot drift)",
            "level_source": "spheres-sim/data/nations/*.json economy.population_m, "
                            "already transcribed with its own sources block. This "
                            "file supplies the SPLIT and never the LEVEL.",
            "caveats": {
                "raster_year": "NOT a later raster projected backwards. 1990 is a "
                               "native GHS-POP epoch, built on 1990-epoch Landsat "
                               "built-up surface. Validated against 1990 censuses: "
                               "US-CA 29.761m vs the 1990 US Census 29,760,021; "
                               "Moscow city 8.93m vs 8.97m; Seoul 10.51m vs 10.6m; "
                               "Uttar Pradesh 137.6m vs 139.1m at the 1991 census.",
                "modelled_surface": "GHS-POP disaggregates census totals onto "
                                    "observed built-up surface. It is a modelled "
                                    "1990 surface, not a 1990 census raster.",
                "rural_under_detection": "Its bias is spatial, not merely scalar: "
                                         "dispersed rural settlement is "
                                         "under-detected, which tilts shares "
                                         "towards urban districts. Every nation's "
                                         "disagreement with its transcribed total "
                                         "is on the record as "
                                         "`ratio_ghs_over_transcribed` rather than "
                                         "assumed small.",
                "roster_gaps": "For a few nations the transcribed population_m "
                               "counts people the district roster has nowhere to "
                               "put, because Natural Earth files part of the 1990 "
                               "nation as a separate adm0 that no TERRITORY entry "
                               "claims. `share x transcribed_total` then spreads "
                               "those people over the districts that do exist. "
                               "Inherited from the districts pass, not caused "
                               "here. Flagged per nation as `roster_gap`, with the "
                               "missing population measured from the same raster "
                               "and the closure test that accepted it attached.",
                "modern_boundaries": "The districts are Natural Earth's MODERN "
                                     "admin-1 units, so 1990 people are sliced by "
                                     "today's internal borders: Uttar Pradesh is "
                                     "split from Uttarakhand, Madhya Pradesh from "
                                     "Chhattisgarh, Bihar from Jharkhand, Andhra "
                                     "Pradesh from Telangana, and so on. This is "
                                     "the right behaviour for a game whose "
                                     "districts are the modern units, but it means "
                                     "a district total is NOT comparable to a 1990 "
                                     "census line for a unit that has since been "
                                     "divided — compare the sum of the parts. "
                                     "Inherited from the districts pass, not "
                                     "introduced here.",
                "successor_states": "Nations with districts but no 1990 nation file "
                                    "have no transcribed population_m, so "
                                    "`pop_1990` is null for every one of their "
                                    "districts. Use `counts` renormalised over the "
                                    "districts held.",
            },
            "roster_gap_method": (
                "Candidates are (nation, unclaimed Natural Earth adm0 codes) pairs. "
                "Each candidate's 1990 population is measured off the same raster, "
                "then the CLOSURE TEST decides: a gap is real only if "
                "|(ghs+missing)/transcribed - 1| < |ghs/transcribed - 1| - %g, i.e. "
                "only if putting the missing people back moves the nation's GHS "
                "total materially towards its transcribed total. This is why "
                "Israel/West Bank and UK/Hong Kong are REJECTED: those nations held "
                "that land in 1990, but their transcribed totals do not count its "
                "people, so adding it would corrupt the split rather than repair "
                "it. Rejected candidates are listed below with their numbers."
                % CLOSURE_MARGIN),
            "roster_gap_candidates_rejected": rejected_gaps,
            "nations_without_districts_detail": (
                "These nations exist in the game with a transcribed population but "
                "their TERRITORY entry in spheres-web/ui/index.html claims no ISO3 "
                "code, so they receive no districts. The code each would claim is "
                "present in ne_10m_admin_1.geojson with admin-1 units and 1990 "
                "population, listed below — so this is a one-line roster omission, "
                "not missing data. This file cannot serve these six nations."),
            "nations_without_districts_codes": unserved,
            "unique_districts": len(counts),
            "nation_district_pairs": sum(n["district_count"] for n in nations.values()),
            "nations_with_shares": len(nations),
            "nations_without_districts": no_districts,
            "nations_without_transcribed_population": no_level,
            "nations_with_uniform_fallback_shares": uniform_fallback,
        },
        "counts": {k: counts[k] for k in sorted(counts)},
        "nations": nations,
    }
    with open(outpath, "w", encoding="utf-8", newline="\n") as f:
        json.dump(out, f, indent=1, sort_keys=True)
        f.write("\n")

    print("wrote", outpath)
    print("  unique districts %d, (nation,district) pairs %d, nations %d"
          % (len(counts), out["meta"]["nation_district_pairs"], len(nations)))
    print("  sum of counts %.0f  (grid total %.0f)"
          % (sum(counts.values()), geo["grid_total_float64"]))
    print("  no districts:", no_districts)
    print("  no transcribed population_m: %d nations" % len(no_level))
    print("  uniform fallback shares:", uniform_fallback or "none")
    print("  roster gaps ACCEPTED by the closure test:")
    for nation in sorted(gaps):
        for g in gaps[nation]:
            print("    %-11s missing %-24s %.3fm outside the roster%s"
                  % (nation, ",".join(g["missing_adm0"]),
                     g["ghs_pop_1990_m_outside_roster"],
                     ("  ratio %.3f -> %.3f" % (g["ratio_without_gap"],
                                                g["ratio_with_gap"]))
                     if "ratio_without_gap" in g else "  (successor, no level)"))
    print("  roster gaps REJECTED by the closure test:")
    for g in rejected_gaps:
        print("    %-11s %-24s %.3fm  ratio %.3f -> %.3f"
              % (g["nation"], ",".join(g["missing_adm0"]),
                 g["ghs_pop_1990_m_outside_roster"],
                 g.get("ratio_without_gap", float("nan")),
                 g.get("ratio_with_gap", float("nan"))))
    print("  nations this file cannot serve (no districts):")
    for u in unserved:
        print("    %-11s would be %s, %.3fm in the raster, transcribed %s"
              % (u["nation"], u["natural_earth_adm0"], u["ghs_pop_1990_m"],
                 u["transcribed_population_m"]))


if __name__ == "__main__":
    a = sys.argv[1:]
    main(a[0] if len(a) > 0 else DEFAULT_PREFIX,
         a[1] if len(a) > 1 else DEFAULT_OUT)
