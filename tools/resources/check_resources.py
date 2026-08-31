#!/usr/bin/env python3
"""
tools/resources/check_resources.py — verify the committed resource artifact.

Run LAST, from the repo root:

    python tools/resources/check_resources.py

Mirrors `tools/terrain/check.py`: it does not trust the generator, it re-reads
the committed file and holds it against ground truth. Four groups of checks —

  1. STRUCTURE. Ids exist in the district roster; every district entry cites a
     source that appears in the provenance block; no commodity carries a
     district-level tonnage; province figures are flagged `shared`.
  2. GROUND TRUTH. Named 1990 mining and petroleum regions must appear where
     history puts them: the Copperbelt in Katanga, the Witwatersrand in
     Gauteng, Krivoy Rog in Dnipropetrovsk, the Pilbara in Western Australia,
     Ghawar under Saudi Arabia's Eastern Province. These are pinned by district
     id, so a projection or identity regression breaks them loudly.
  3. THE FABRICATION GUARDS. The checks that matter most, because they fail in
     the direction the doctrine cares about: no district may carry a magnitude,
     no national figure may have been divided, and the artifact's own
     `districts_with_any_resource` must match the file it describes.
  4. DETERMINISM. Regenerates the artifact and requires byte-identical output.
     Skipped with --fast.

Every check prints PASS or FAIL and the script exits non-zero on any failure.
"""

import csv
import hashlib
import io
import json
import os
import subprocess
import sys
import zipfile

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, "..", ".."))
ART = os.path.join(ROOT, "spheres-web", "data", "district_resources.json")
DISTRICTS = os.path.join(ROOT, "spheres-sim", "data", "districts.json")

FAILURES = []
CHECKS = [0]


def ok(cond, label, detail=""):
    CHECKS[0] += 1
    if cond:
        print(f"  PASS  {label}")
    else:
        print(f"  FAIL  {label}   {detail}")
        FAILURES.append(label)


def main():
    fast = "--fast" in sys.argv
    with open(ART, encoding="utf-8") as f:
        art = json.load(f)
    with open(DISTRICTS, encoding="utf-8") as f:
        nations = json.load(f)["nations"]

    roster = set()
    d2n = {}
    for nation in sorted(nations):
        for d in nations[nation]:
            roster.add(d["id"])
            d2n.setdefault(d["id"], set()).add(nation)

    D = art["districts"]
    N = art["national"]
    SRC = art["sources"]

    def owned_by(nation, com):
        return sorted(d for d in D if com in D[d] and nation in d2n.get(d, ()))

    # --- 1. structure -----------------------------------------------------
    print("CHECK 1: structure and provenance")
    ok(set(D) <= roster, "every district id exists in the roster",
       f"stray={sorted(set(D) - roster)[:5]}")
    ok(len(roster) == 2610, "roster is 2610 districts", f"got {len(roster)}")

    # Coal is the one commodity placed from more than one dataset, so `src` may
    # be a '+'-joined list. Every part of it must still be a declared source.
    bad_src = sorted({p for d in D for v in D[d].values()
                      for p in v["src"].split("+")} - set(SRC))
    ok(not bad_src, "every district entry cites a declared source", f"{bad_src}")

    for key in SRC:
        s = SRC[key]
        ok(all(s.get(f) for f in ("title", "url", "licence", "role", "bytes")),
           f"source '{key}' carries title/url/licence/role/bytes")
        # A digest per staged file. OFR 01-104 publishes its coal deposit layer
        # as loose shapefile parts rather than an archive, so it declares
        # `files` with a digest each; everything else declares one `sha256`.
        parts = s.get("files") or [s]
        ok(bool(parts) and all(p.get("sha256") and p.get("file") and p.get("bytes")
                               for p in parts),
           f"source '{key}' digests every staged file ({len(parts)})")

    tonnage_keys = {"tons", "tonnes", "production", "output", "value", "reserve",
                    "reserves", "grade", "share"}
    offenders = []
    for d in sorted(D):
        for com, v in sorted(D[d].items()):
            if tonnage_keys & set(v):
                offenders.append((d, com, sorted(tonnage_keys & set(v))))
    ok(not offenders, "NO district carries a tonnage/grade/production field",
       f"{offenders[:4]}")

    unshared = []
    for d in sorted(D):
        for com, v in sorted(D[d].items()):
            for p in v.get("provinces", ()):
                if not p.get("shared"):
                    unshared.append((d, com, p["name"]))
    ok(not unshared, "every province figure is flagged shared", f"{unshared[:4]}")

    ok(art["meta"]["districts_with_any_resource"] == len(D),
       "meta district count matches the file",
       f'{art["meta"]["districts_with_any_resource"]} vs {len(D)}')

    # --- 1b. confidence, added by the 2026-08-31 correction pass -----------
    # Ruling 1 renders single-record districts weaker rather than deleting
    # them, which only works if every entry carries its grade.
    BANDS = {"single", "sparse", "moderate", "strong"}
    missing, badband = [], []
    for d in sorted(D):
        for com, v in sorted(D[d].items()):
            c = v.get("confidence")
            if not c or "band" not in c or "basis" not in c:
                missing.append((d, com))
            elif c["band"] not in BANDS:
                badband.append((d, com, c["band"]))
    ok(not missing, "EVERY district-commodity entry carries a confidence block",
       f"{missing[:4]}")
    ok(not badband, "every confidence band is one of the four declared values",
       f"{badband[:4]}")

    # A point-sourced entry is one that cites sites. A coal entry resting only
    # on a named coal-field POLYGON cites no site and has no record count to
    # state; it must instead state the area it was banded on.
    countless = [(d, com) for d in sorted(D) for com, v in sorted(D[d].items())
                 if v.get("sites") and "records" not in v["confidence"]]
    ok(not countless,
       "every point-sourced entry states its distinct supporting record count",
       f"{countless[:4]}")
    arealess = [(d, com) for d in sorted(D) for com, v in sorted(D[d].items())
                if v.get("fields") and ("coal_fields" not in v["confidence"]
                                        or "max_area_frac_district" not in v["confidence"])]
    ok(not arealess,
       "every coal-field entry states how many fields and how much of the district",
       f"{arealess[:4]}")
    naked = [(d, com) for d in sorted(D) for com, v in sorted(D[d].items())
             if not v.get("sites") and not v.get("fields") and not v.get("province_level")]
    ok(not naked, "no entry exists without sites, fields or a province", f"{naked[:4]}")

    badfrac = []
    for d in sorted(D):
        for com, v in sorted(D[d].items()):
            for p in v.get("provinces", ()):
                for f in ("area_frac_district", "area_frac_province"):
                    if not (0.0 <= p.get(f, -1.0) <= 1.0):
                        badfrac.append((d, com, p["code"], f, p.get(f)))
    ok(not badfrac, "every province attachment carries measured area fractions in [0,1]",
       f"{badfrac[:4]}")

    # --- 2. ground truth --------------------------------------------------
    print("\nCHECK 2: ground truth — named 1990 regions land where history puts them")
    pins = [
        ("CD-KA", "copper", "Katanga / the Copperbelt carries copper"),
        ("CD-KA", "cobalt", "Katanga carries cobalt"),
        ("ZM-08", "copper", "Zambian Copperbelt (Central) carries copper"),
        ("ZA-GT", "gold", "Witwatersrand / Gauteng carries gold"),
        ("ZA-NW", "platinum_group", "Bushveld / North West carries PGE"),
        ("ZA-LP", "platinum_group", "Bushveld / Limpopo carries PGE"),
        ("UA-12", "iron", "Krivoy Rog / Dnipropetrovsk carries iron"),
        ("CL-AN", "copper", "Antofagasta / Chuquicamata carries copper"),
        ("AU-WA", "iron", "Pilbara / Western Australia carries iron"),
        ("AU-QLD", "bauxite", "Weipa / Queensland carries bauxite"),
        ("AU-WA", "bauxite", "Darling Range / Western Australia carries bauxite"),
        ("SA-04", "oil", "Saudi Eastern Province carries oil"),
        ("IQ-BA", "oil", "Basra carries oil"),
        ("KW-AH", "oil", "Kuwait carries oil"),
        ("MA-09", "phosphate", "Moroccan phosphate belt carries phosphate"),
        ("BR-PA", "iron", "Carajas / Para carries iron"),
        ("CA-ON", "gold", "Ontario carries gold"),
    ]
    for did, com, label in pins:
        ok(did in D and com in D[did], label,
           f"{did} has {sorted(D.get(did, {}))}")

    print("\n  Persian Gulf oil, per nation:")
    for nation in ("SaudiArabia", "Iraq", "Kuwait", "Iran", "UAE", "Qatar"):
        got = owned_by(nation, "oil")
        ok(bool(got), f"{nation} has >=1 oil district", "none")

    # --- 2b. the contamination the correction pass removed -----------------
    # These are the specific defects two independent audits found in the
    # shipped artifact. Each one is now a test that fails loudly if it returns.
    print("\nCHECK 2b: the 2026-08-31 contamination stays gone")

    def has(did, com):
        return com in D.get(did, {})

    ok(not has("NO-12", "bauxite"),
       "NO-12 Hordaland carries no bauxite (Tyssedal is an aluminium SMELTER)",
       f'{sorted(D.get("NO-12", {}))}')
    ok(not has("NO-18", "bauxite"),
       "NO-18 Nordland carries no bauxite (Mosjoen is an aluminium SMELTER)",
       f'{sorted(D.get("NO-18", {}))}')
    for nation in ("Norway", "Japan"):
        got = owned_by(nation, "bauxite")
        ok(not got, f"{nation} carries no bauxite anywhere (it has no bauxite ore geology)",
           f"{got}")

    # Steep Rock Iron Mine: commod1 and commod2 empty, commod3 listing six
    # assayed elements. It filed as bauxite AND iron AND phosphate.
    steep = []
    for d in sorted(D):
        for com, v in sorted(D[d].items()):
            for s in v.get("sites", ()):
                if s["id"] == "10157857":
                    steep.append((d, com))
    ok(not steep, "Steep Rock Iron Mine (10157857) no longer files under any commodity",
       f"{steep}")
    ok(not has("CA-ON", "bauxite"), "Ontario carries no bauxite", "")
    ok(has("CA-ON", "iron"), "Ontario still carries iron from its real iron mines", "")

    # Nigeria's Delta Steel plant at Warri, oper_type Processing Plant.
    warri = [(d, com) for d in sorted(D) for com, v in sorted(D[d].items())
             for s in v.get("sites", ()) if s["id"] == "10304733"]
    ok(not warri, "Dsc Warri (10304733), the Delta Steel PLANT, is not an iron deposit",
       f"{warri}")

    # Ruling 2: the petroleum province rebuild. Al-Anbar carried the whole
    # Mesopotamian Foredeep entry, indistinguishable from Al-Basrah's.
    def cover(did):
        v = D.get(did, {}).get("oil")
        return v["confidence"]["max_area_frac_district"] if v else 0.0
    ok(cover("IQ-BA") > cover("IQ-AN"),
       "Al-Basrah's measured oil-province coverage exceeds Al-Anbar's",
       f'BA={cover("IQ-BA")} AN={cover("IQ-AN")}')
    ok(cover("NG-DE") > cover("NG-CR"),
       "Delta State's measured oil-province coverage exceeds Cross River's",
       f'DE={cover("NG-DE")} CR={cover("NG-CR")}')

    def app(did, com="oil"):
        v = D.get(did, {}).get(com)
        return v["apportionment"]["total"] if v else 0.0

    ok(app("IQ-BA") > app("IQ-AN"),
       "Al-Basrah out-ranks Al-Anbar on apportioned oil, not just on area",
       f'BA={app("IQ-BA")} AN={app("IQ-AN")}')
    ok(app("NG-DE") > app("NG-CR"),
       "Delta State out-ranks Cross River on apportioned oil",
       f'DE={app("NG-DE")} CR={app("NG-CR")}')
    ok(max(owned_by("Nigeria", "oil"), key=app) == "NG-DE",
       "Delta State is Nigeria's leading oil district",
       f'{sorted(owned_by("Nigeria", "oil"), key=app, reverse=True)[:3]}')
    # Al-Anbar's headline province must be the small one. Reading the whole
    # 292,442 MMBO Mesopotamian Foredeep off a 6% clip is the original defect.
    lead = {}
    for d in ("IQ-BA", "IQ-AN"):
        ps = D.get(d, {}).get("oil", {}).get("provinces", ())
        if ps:
            lead[d] = max(ps, key=lambda p: p["apportioned"])["code"]
    ok(lead.get("IQ-BA") == "2024" and lead.get("IQ-AN") == "2024",
       "both read the Foredeep as their lead province, but by measured share",
       f"{lead}")
    ok(app("IQ-BA") > 0.9 * 292442 * 0.05 and app("IQ-AN") < app("IQ-BA"),
       "Al-Basrah's Foredeep share is the larger one", f'{app("IQ-BA")}')
    foredeep = {}
    for d in ("IQ-BA", "IQ-AN"):
        for p in D.get(d, {}).get("oil", {}).get("provinces", ()):
            if p["code"] == "2024":
                foredeep[d] = p["area_frac_district"]
    ok(foredeep.get("IQ-BA", 0) > 0.8 and foredeep.get("IQ-AN", 1) < 0.2,
       "the Mesopotamian Foredeep covers Al-Basrah and only clips Al-Anbar",
       f"{foredeep}")

    # Ruling 2, second half: the apportionment itself. `apportioned` must be
    # exactly `known` x `area_frac_province` and nothing else, so nobody can
    # quietly fold a population weight or a production figure into it, and the
    # per-province total must never exceed the province's own volume.
    badapp, pertot = [], {}
    for d in sorted(D):
        for com in ("oil", "gas"):
            e = D[d].get(com)
            if not e:
                continue
            for p in e.get("provinces", ()):
                # `area_frac_province` ships rounded to 1e-6, so reconstructing
                # from the file admits known x 5e-7 of slack. On a 1,043,000
                # BCFG province that is half a unit, which is why the tolerance
                # scales with `known` instead of being a flat epsilon.
                want = p["known"] * p["area_frac_province"]
                if abs(p["apportioned"] - want) > p["known"] * 5e-7 + 1e-3:
                    badapp.append((d, com, p["code"], p["apportioned"], want))
                pertot.setdefault((com, p["code"], p["known"]), 0.0)
                pertot[(com, p["code"], p["known"])] += p["apportioned"]
            tot = round(sum(p["apportioned"] for p in e["provinces"]), 3)
            if abs(e["apportionment"]["total"] - tot) > 0.51:
                badapp.append((d, com, "TOTAL", e["apportionment"]["total"], tot))
    ok(not badapp,
       "every apportioned figure is exactly `known` x `area_frac_province`",
       f"{badapp[:3]}")
    over = [(k, v) for k, v in sorted(pertot.items()) if v > k[2] * 1.001 + 0.5]
    ok(not over,
       f"no province's apportioned volume exceeds its own assessed total "
       f"({len(pertot)} provinces summed)", f"{over[:3]}")

    # The area integral itself, held against a number this pipeline did not
    # produce: `districts.json` publishes `area_sqkm` per district from the
    # terrain pass. Recovering the district area from the artifact as
    # intersection_sqkm / area_frac_district must reproduce it.
    dareas = {}
    for nation in sorted(nations):
        for d in nations[nation]:
            if d.get("area_sqkm"):
                dareas[d["id"]] = d["area_sqkm"]
    offby, cmp_n = [], 0
    for d in sorted(D):
        for com in ("oil", "gas"):
            for p in D[d].get(com, {}).get("provinces", ()):
                if p["area_frac_district"] < 0.05 or d not in dareas:
                    continue
                got = p["intersection_sqkm"] / p["area_frac_district"]
                cmp_n += 1
                if abs(got - dareas[d]) > 0.02 * dareas[d] + 1.0:
                    offby.append((d, round(got, 1), dareas[d]))
                break
    ok(not offby,
       f"the measured district areas reproduce districts.json area_sqkm "
       f"({cmp_n} compared)", f"{sorted(set(offby))[:4]}")

    # Ruling 3: unlocated producers are marked, never faked.
    unl = art.get("unlocated_producers", {})
    oil_unl = {r["nation"] for r in unl.get("oil", ())}
    ok("USA" in oil_unl,
       "the USA is marked as an UNLOCATED oil producer, not as an empty one",
       f"{sorted(oil_unl)}")
    ok("Norway" in oil_unl,
       "Norway is marked as an UNLOCATED oil producer (its fields are offshore)",
       f"{sorted(oil_unl)}")
    ok(not owned_by("USA", "oil") and not owned_by("USA", "gas"),
       "no US district was invented to fill the oil or gas hole",
       f'{owned_by("USA", "oil") + owned_by("USA", "gas")}')

    # The boundary ribbons that would have filled it. Exact clipping finds three
    # US overlaps and all three are a digitising mismatch along a shared border;
    # if the floor is ever removed, this fails and the USA silently acquires
    # located oil.
    slivers = art["meta"].get("boundary_slivers", {}).get("dropped", ())
    sl_ids = {(s["code"], s["district"]) for s in slivers}
    for code, did, label in (("5243", "US-MT", "Alberta Basin x Montana"),
                             ("5244", "US-ND", "Williston Basin x North Dakota"),
                             ("5246", "US-AK", "Mackenzie Foldbelt x Alaska")):
        ok((code, did) in sl_ids,
           f"{label} is recorded as a dropped boundary ribbon", f"{sorted(sl_ids)}")
    ok(all(s["frac_of_district"] < 1e-3 and s["frac_of_province"] < 1e-3
           for s in slivers) and len(slivers) > 0,
       f"every dropped ribbon is negligible in BOTH polygons ({len(slivers)} dropped)")
    ok(all(r.get("basis") in ("offshore_only", "no_province_assessed",
                              "boundary_sliver_only")
           for c in ("oil", "gas") for r in unl.get(c, ())),
       "every unlocated petroleum producer states WHICH kind of hole it is")

    # --- 3. fabrication guards -------------------------------------------
    print("\nCHECK 3: fabrication guards")
    for com in sorted(N):
        for nation in sorted(N[com]):
            e = N[com][nation]
            ok_units = bool(e.get("units")) and bool(e.get("source"))
            if not ok_units:
                ok(False, f"national {com}/{nation} carries units and source")
                break
        else:
            continue
        break
    else:
        ok(True, "every national figure carries units and a source")

    # The real guard against a divided national figure: every province volume
    # on a district must be BYTE-FOR-BYTE the figure WEP publishes for that
    # province code. If anything had been apportioned — by area, by district
    # count, by anything — the value would no longer match its source record.
    # (Comparing district floats to national floats instead would be useless:
    # Bohemia holds 1.0 MMBO and Bulgaria produced 1.0 thousand bbl/day, which
    # collide numerically while meaning nothing to each other.)
    sys.path.insert(0, HERE)
    import sources as S  # noqa: E402
    wep_zip = os.path.join(ROOT, "spheres-web", "data", "wep_prva.zip")
    prows, _ = S.read_shapefile_zip(wep_zip, "WEP_PRVA/WEP_PRVA")
    src_by_code = {}
    for r in prows:
        src_by_code[(r.get("CODE") or "").strip()] = (
            (r.get("KWN_OIL") or "").strip(), (r.get("KWN_GAS") or "").strip())
    field = {"oil": 0, "gas": 1}
    mismatched = []
    checked = 0
    for d in sorted(D):
        for com in ("oil", "gas"):
            for p in D[d].get(com, {}).get("provinces", ()):
                raw = src_by_code.get(p["code"], (None, None))[field[com]]
                checked += 1
                if raw is None or float(raw) != float(p["known"]):
                    mismatched.append((d, com, p["name"], p["known"], raw))
    ok(not mismatched and checked > 0,
       f"every province volume equals its WEP source record ({checked} checked)",
       f"{mismatched[:3]}")

    # The general guard behind the named cases above: re-read MRDS and require
    # that every site id the artifact cites is a record the admission rules
    # actually admit FOR THAT COMMODITY. A plant slipping back in under any
    # commodity, or a tertiary assay being read as production, fails here
    # without anyone having to think of the example first.
    mrds_zip = os.path.join(ROOT, "spheres-web", "data", "mrds-csv.zip")
    admitted = {}
    published_at = {}
    for row, v in S.mrds_rows(mrds_zip, ("Iron", "Copper", "Aluminum", "Gold",
                                         "Uranium", "Phosphorus-Phosphates")):
        if not v["keep"]:
            continue
        rid = (row.get("dep_id") or row.get("mrds_id") or "").strip()
        admitted.setdefault(rid, set()).update(v["keep"])
        published_at[rid] = f"{v['lat']:.5f},{v['lon']:.5f}"
    TOKEN = {"iron": "Iron", "copper": "Copper", "bauxite": "Aluminum",
             "gold": "Gold", "uranium": "Uranium",
             "phosphate": "Phosphorus-Phosphates"}
    strays, checked_sites = [], 0
    for d in sorted(D):
        for com, v in sorted(D[d].items()):
            if v.get("src") != "mrds":
                continue
            for s in v.get("sites", ()):
                checked_sites += 1
                if TOKEN[com] not in admitted.get(s["id"], ()):
                    strays.append((d, com, s["id"], s["name"]))
    ok(not strays,
       f"every cited MRDS site is an admitted extraction record for its commodity "
       f"({checked_sites} checked)", f"{strays[:4]}")

    vocab = art["meta"].get("mrds_vocabulary", {})
    ok(bool(vocab.get("oper_type")) and bool(vocab.get("dev_stat")),
       "artifact enumerates the MRDS oper_type and dev_stat vocabularies")
    ok(all("admitted" in e and "reason" in e and "records" in e
           for f in ("oper_type", "dev_stat") for e in vocab.get(f, {}).values()),
       "every enumerated vocabulary value carries a count, a decision and a reason")
    ok(vocab.get("oper_type", {}).get("Processing Plant", {}).get("admitted") is False,
       "oper_type 'Processing Plant' is declared NOT admitted")
    ok(vocab.get("commodity_fields", {}).get("commod3", {}).get("admitted") is False,
       "commod3 is declared NOT admitted")

    corr = art["meta"].get("correction_2026_08_31", {})
    ok(bool(corr.get("removed")) and bool(corr.get("rules_added")),
       "artifact records the correction pass, its rules and what they removed")

    ok(art["meta"].get("do_not", "").strip().startswith("Do not rank"),
       "artifact carries the do-not-rank warning")
    ok("DERIVED, NOT TRANSCRIBED" in art["meta"].get("apportionment", ""),
       "artifact declares the area apportionment DERIVED, not transcribed")
    ok("never divided" in art["meta"].get("apportionment", ""),
       "artifact says national 1990 production is still never divided")
    ok("sources_rejected" in art and art["sources_rejected"],
       "artifact records the sources examined and rejected")

    # Coal stopped being national-only on 2026-08-31. What replaces the old
    # "no district claimed" assertion is not a weaker check but a stricter one:
    # coal must now be located, from all four declared sources, with the
    # vintage warning and the coke rule stated in the artifact rather than only
    # in this directory's source.
    coal = art["commodities"]["coal"]
    cm = art["meta"]["coal"]
    ok(coal["districts_with_presence"] > 0 and coal["location_source"],
       "coal is LOCATED — the national-only hole is filled",
       f'{coal["districts_with_presence"]} districts from {coal["location_source"]}')
    ok(sorted(coal.get("location_sources", ())) ==
       sorted(["minfac", "china2014", "fsucoal", "uscoalfields"]),
       "coal declares all four location sources",
       f'{coal.get("location_sources")}')
    ok(coal["districts_from_points"] > 0 and coal["districts_from_fields"] > 0,
       "coal is placed BOTH from mine points and from named coal fields",
       f'{coal["districts_from_points"]} point / {coal["districts_from_fields"]} field')
    ok("1990 census" in coal.get("vintage_warning", ""),
       "coal states that none of its location sources is a 1990 census")
    ok(cm["minfac_vocabulary"]["commodity"]["Coke: contained in domestic coal"]["admitted"]
       is False,
       "the coke-plant token is declared NOT admitted — coke is not mined coal")
    ok(cm["minfac_vocabulary"]["fac_type"]["Plant"]["admitted"] is False,
       "minfac fac_type 'Plant' is declared NOT admitted")
    ok(cm["removed"].get("commodity_stage", {}).get("total", 0) > 0
       and cm["removed"].get("facility_type", {}).get("total", 0) > 0,
       "both coal admission rules removed something, and the artifact says how much",
       f'{ {k: v["total"] for k, v in cm["removed"].items()} }')
    ok(len(cm["what_this_cannot_say"]) >= 4,
       "coal states in the artifact what it cannot support")
    # The hole, measured against the thing it was a hole in. 59 nations produced
    # coal in 1990 and every one of them was national-only; the number that
    # still is, is the number this pass did not reach.
    prod = set(cm["nations_producing_in_1990"])
    loc = set(cm["producers_now_located"])
    unloc = set(cm["producers_still_unlocated"])
    ok(prod == loc | unloc and not (loc & unloc),
       "every 1990 coal producer is either located or explicitly unlocated",
       f"{len(prod)} = {len(loc)} + {len(unloc)}")
    ok(len(loc) >= 45,
       f"{len(loc)} of the {len(prod)} 1990 coal producers now carry located coal",
       f"still unlocated: {sorted(unloc)}")
    ok({r["nation"] for r in art["unlocated_producers"]["coal"]} == unloc,
       "meta.coal and unlocated_producers.coal name the same unlocated nations",
       f'{sorted({r["nation"] for r in art["unlocated_producers"]["coal"]} ^ unloc)}')
    for big in ("China", "USA", "USSR", "Germany", "Poland", "India",
                "Australia", "SouthAfrica", "UK", "Czechoslovakia"):
        ok(big in loc, f"the 1990 coal power {big} carries located coal")
    for c in ("wheat", "rice"):
        e = art["commodities"][c]
        ok(e["districts_with_presence"] == 0 and e["location_source"] is None,
           f"{c} is national-only, with no district claimed", f"{e}")

    # --- 3b. ruling 4: the band counts SITES, and no coordinate moved ------
    #
    # The confidence band was re-based from "how many records" to "how many
    # DISTINCT COORDINATES", because six records filed on one point are one
    # piece of evidence and the old metric read them as six. That correction is
    # only trustworthy if two things hold, and neither was asserted before:
    #
    #   (a) the band really is a function of the coordinate count. Recomputed
    #       here from the artifact's own numbers, against a threshold table
    #       written out longhand — so a generator that bands on the wrong
    #       quantity fails here instead of agreeing with itself.
    #   (b) NO PUBLISHED COORDINATE MOVED. The whole ruling turns on counting
    #       coordinates, so a silently corrected point would change a band with
    #       nothing to catch it. Every cited coordinate is held against the
    #       coordinate its source published, to five decimals, from the raw zip.
    print("\nCHECK 3b: ruling 4 — banding on distinct sites")

    def band_sites(n):
        return ("strong" if n >= 6 else "moderate" if n >= 3
                else "sparse" if n >= 2 else "single")

    def band_area(a):
        return ("strong" if a >= 0.50 else "moderate" if a >= 0.15
                else "sparse" if a >= 0.02 else "single")

    ORDER = ["single", "sparse", "moderate", "strong"]

    moved, unknown, n_at = [], [], 0
    for d in sorted(D):
        for com, v in sorted(D[d].items()):
            if v.get("src") != "mrds":
                continue
            for s in v.get("sites", ()):
                n_at += 1
                pub = published_at.get(s["id"])
                if pub is None:
                    unknown.append((d, com, s["id"]))
                elif pub != s["at"]:
                    moved.append((d, com, s["id"], s["name"], s["at"], pub))
    ok(not unknown, f"every cited MRDS site is a row of the published source "
       f"({n_at} coordinates checked)", f"{unknown[:4]}")
    ok(not moved, "NO PUBLISHED COORDINATE MOVED: every cited point is the one "
       "its source published, to five decimals", f"{moved[:3]}")

    bad_band, bad_field, bad_sum, bad_rec, inflated, bad_small = [], [], [], [], [], []
    point_entries, hist = 0, {}
    for d in sorted(D):
        for com, v in sorted(D[d].items()):
            c = v.get("confidence") or {}
            if "distinct_coordinates" not in c:
                continue
            point_entries += 1
            dc, rec = c["distinct_coordinates"], c["records"]
            pband = band_sites(dc)
            if "band_on_fields" in c:
                fband = band_area(c["max_area_frac_district"])
                if c["band_on_fields"] != fband:
                    bad_field.append((d, com, c["band_on_fields"], fband))
                want = max((pband, fband), key=ORDER.index)
                if c.get("band_on_points") != pband:
                    bad_band.append((d, com, "band_on_points",
                                     c.get("band_on_points"), pband))
            else:
                want = pband
                # A points-only entry can never band ABOVE what the superseded
                # record count gave it: coordinates are records deduplicated, so
                # the new metric may shrink an evidence claim and never inflate.
                if ORDER.index(c["band"]) > ORDER.index(
                        c["superseded_band_on_records"]):
                    inflated.append((d, com, c["superseded_band_on_records"],
                                     c["band"]))
            if c["band"] != want:
                bad_band.append((d, com, c["band"], want, dc))
            if c["unflagged_coordinates"] + c["centroid_coordinates"] != dc:
                bad_sum.append((d, com))
            if rec < dc:
                bad_rec.append((d, com, rec, dc))
            # Where the sites array is not truncated at SAMPLE it must
            # reproduce both counts from the shipped rows themselves.
            if rec <= 8:
                rows = v.get("sites", ())
                if len(rows) != rec or len({r["at"] for r in rows}) != dc:
                    bad_small.append((d, com, rec, dc, len(rows)))
            hist[c["band"]] = hist.get(c["band"], 0) + 1

    ok(not bad_band, f"every band is recomputed here from the DISTINCT "
       f"COORDINATE count and agrees ({point_entries} point entries)",
       f"{bad_band[:4]}")
    ok(not bad_field, "every coal band_on_fields is recomputed from the "
       "measured area share and agrees", f"{bad_field[:4]}")
    ok(not bad_sum, "unflagged + centroid coordinates == distinct coordinates",
       f"{bad_sum[:4]}")
    ok(not bad_rec, "records >= distinct_coordinates in every entry",
       f"{bad_rec[:4]}")
    ok(not bad_small, "an untruncated sites array reproduces both its record "
       "count and its coordinate count", f"{bad_small[:4]}")
    ok(not inflated, "the site metric only ever SHRINKS an evidence claim: no "
       "points-only entry bands above its superseded record band",
       f"{inflated[:4]}")

    # The named cases from the defect report, pinned by id so a regression in
    # placement, in the census or in the flag breaks them by name.
    for did, com, rec, dc, was, now in (
            ("FRA_centre-val-de-loire", "bauxite", 6, 1, "strong", "single"),
            ("FRA_centre-val-de-loire", "iron", 17, 2, "strong", "sparse"),
            ("FRA_centre-val-de-loire", "uranium", 5, 1, "moderate", "single"),
            ("MA-12", "phosphate", 5, 2, "moderate", "sparse"),
            ("AU-NT", "bauxite", 3, 3, "moderate", "moderate"),
            ("AU-QLD", "bauxite", 6, 3, "strong", "moderate"),
            ("ESP_madrid", "iron", 6, 1, "strong", "single"),
            ("ITA_umbria", "copper", 8, 1, "strong", "single"),
            ("SE-Y", "copper", 14, 1, "strong", "single")):
        c = (D.get(did, {}).get(com) or {}).get("confidence") or {}
        got = (c.get("records"), c.get("distinct_coordinates"),
               c.get("superseded_band_on_records"), c.get("band"))
        ok(got == (rec, dc, was, now),
           f"{did} {com}: {rec} records on {dc} coordinate(s), {was} -> {now}",
           f"got {got}")

    # Every record is still shipped. The fix was arithmetic; nothing was dropped
    # to make a band fall.
    fr = D["FRA_centre-val-de-loire"]["bauxite"]
    ok(fr["n"] == 6 and len(fr["sites"]) == 6
       and all(s["at"] == "46.56346,2.55405" for s in fr["sites"])
       and all(s["at_centroid"] for s in fr["sites"]),
       "the FRA defect is closed by counting, not by deletion: all 6 records "
       "still ship, all on the published point, all flagged at_centroid")
    ok(D["ZM-08"]["copper"]["confidence"]["centroid_coordinates"] == 0
       and D["ZM-08"]["copper"]["confidence"]["band"] == "strong",
       "control: ZM-08 copper is unflagged and still bands strong")

    # A flag that fires everywhere says nothing. A flag that fires nowhere says
    # nothing either. And an entry located only to an admin unit must not be
    # allowed to claim more than `sparse`.
    flagged = wholly = 0
    wholly_bands = {}
    for d in sorted(D):
        for com, v in sorted(D[d].items()):
            c = v.get("confidence") or {}
            if not c.get("centroid_coordinates"):
                continue
            flagged += 1
            if c["unflagged_coordinates"] == 0:
                wholly += 1
                wholly_bands[c["band"]] = wholly_bands.get(c["band"], 0) + 1
    ok(0 < flagged < point_entries // 2,
       f"the filing-centroid flag is selective: {flagged} of {point_entries} "
       f"point entries touch one, {wholly} rest on nothing else")
    ok(set(wholly_bands) <= {"single", "sparse"},
       "no entry whose every point is a filing centroid bands above `sparse`",
       f"{wholly_bands}")

    r4 = art["meta"].get("confidence_ruling_4", {})
    ok({"the_defect", "the_correction", "the_flag",
        "what_the_flag_cannot_see"} <= set(r4),
       "the artifact states the ruling, the flag rule AND what the flag cannot "
       "see", f"{sorted(r4)}")

    # --- 3c. the named coal regions ---------------------------------------
    #
    # Coal is placed from four sources and the map is worthless if the places
    # that decided 1990 energy politics are not on it. Pinned by district id,
    # and each one asserts the SHAPE of its evidence too — a named mine or a
    # named field — so a district that acquires coal from the wrong source, or
    # from a coke oven, fails here rather than merely being present.
    print("\nCHECK 3c: the named coal regions")

    def coal(did):
        return (D.get(did, {}) or {}).get("coal")

    for did, label, kind in (
            ("DE-NW", "the RUHR (Nordrhein-Westfalen)", "points"),
            ("PL-SL", "UPPER SILESIA (Slaskie)", "points"),
            ("UA-14", "the DONBAS (Donets'ka)", "fields"),
            ("UA-09", "the Donbas (Luhans'ka)", "fields"),
            ("RU-KEM", "the KUZBASS (Kemerovo)", "fields"),
            ("KZ-KAR", "KARAGANDA", "fields"),
            ("KZ-PAV", "EKIBASTUZ (Pavlodar)", "fields"),
            ("RU-KO", "the PECHORA (Komi)", "fields"),
            ("CN-SX", "SHANXI", "points"),
            ("US-WV", "APPALACHIA (West Virginia)", "fields"),
            ("US-PA", "Appalachia (Pennsylvania)", "fields"),
            ("US-KY", "Appalachia (Kentucky)", "fields"),
            ("GBR_east-midlands", "the British pits (Notts: Thoresby, Welbeck)",
             "points"),
            ("GBR_yorkshire-and-the-humber",
             "the British pits (Yorks: Kellingley, Maltby)", "points"),
            ("GBR_east-wales", "the South Wales pits (Tower Colliery)",
             "points")):
        e = coal(did)
        if not e:
            ok(False, f"{label} ({did}) carries located coal", "absent")
            continue
        c = e["confidence"]
        has = ("fields" if e.get("fields") else "") or ""
        has_pts = "records" in c
        good = (e.get("fields") if kind == "fields" else has_pts)
        detail = []
        if has_pts:
            detail.append(f"{c['records']} named mine(s) on "
                          f"{c['distinct_coordinates']} coordinate(s)")
        if e.get("fields"):
            detail.append(f"{c['coal_fields']} named field(s)")
        ok(bool(good), f"{label} ({did}) carries coal as {kind} — "
           f"{c['band']}, " + "; ".join(detail), f"has_points={has_pts} {has}")

    # The Ruhr and Upper Silesia are the same commodity, the same source and
    # the same year, and they are the two ends of the ruling. The Ruhr files
    # named collieries at their own pitheads, so it earns `strong` on ten real
    # points. Upper Silesia files seven ROLLUP rows — "Mine at Upper Silesia
    # (17 mines)" — on one point, and the old metric read those seven rows as
    # `strong` too. Nothing was corrected: all seven rows still ship at the
    # coordinate the Yearbook published. They are now counted once.
    ruhr = coal("DE-NW")
    names = {s["name"] for s in ruhr["sites"]}
    ok("Prosper-Haniel Mine" in names and ruhr["confidence"]["band"] == "strong"
       and ruhr["confidence"]["centroid_coordinates"] == 0,
       "the Ruhr earns `strong` on named collieries at their own pitheads "
       "(Prosper-Haniel among them), none of them a filing centroid",
       f"{sorted(names)[:4]}")
    sil = coal("PL-SL")
    sc = sil["confidence"]
    ok(sc["band"] == "single" and sc["superseded_band_on_records"] == "strong"
       and sc["records"] == 7 and sc["distinct_coordinates"] == 1
       and sc["unflagged_coordinates"] == 0
       and all(s["at_centroid"] for s in sil["sites"])
       and all(s["at"] == "50.17000,18.83000" for s in sil["sites"]),
       "Upper Silesia is the ruling in one district: 7 rollup rows on ONE "
       "published point, all still shipped, all flagged — `strong` on records, "
       "`single` on sites", f"{sc}")

    # Defect 1 of the coal pass: a named field is not a polygon. Appalachia is
    # drawn as hundreds of fragments and banding on the largest one read
    # Pennsylvania at 26%. The fields must be aggregated PER NAMED FIELD.
    for did, want in (("US-PA", 0.304), ("US-WV", 0.696)):
        e = coal(did) or {}
        app = [f for f in e.get("fields", ())
               if f["name"] == "Appalachian Region"]
        ok(len(app) == 1 and abs(app[0]["area_frac_district"] - want) < 0.02
           and app[0]["polygons"] > 1,
           f"{did}: the Appalachian Region is ONE aggregated named field at "
           f"area_frac ~{want}, not its largest fragment",
           f"{[(f['name'], f['area_frac_district'], f['polygons']) for f in app]}")

    # Defect 2: china2014's IDNum restarts per province, so keying on it alone
    # collapsed 2,440 mines onto 253. If that regressed, an entry would carry
    # fewer records than it has distinct coordinates.
    ok(not [1 for d in D if coal(d)
            and "records" in D[d]["coal"]["confidence"]
            and D[d]["coal"]["confidence"]["records"]
            < D[d]["coal"]["confidence"]["distinct_coordinates"]],
       "no coal entry has more coordinates than records (the china2014 "
       "per-province IDNum key defect)")
    sx = coal("CN-SX")["confidence"]
    ok(sx["records"] >= 200 and sx["distinct_coordinates"] >= 100,
       f"Shanxi's mines did not collapse onto a shared key: {sx['records']} "
       f"records on {sx['distinct_coordinates']} coordinates")

    # The coke trap, re-derived from raw `minfac.csv` using the artifact's OWN
    # published vocabulary, so the count is anchored and not just the property.
    # minfac files coke ovens beside coal mines under a commodity string that
    # contains the word "coal" — `Coke: contained in domestic coal` — and five
    # of them sit in the Ruhr, one at Bottrop where Prosper-Haniel already is.
    # Note the candidate set must be taken from the vocabulary and NOT by
    # searching for the substring "coal": the bare `Coke` and `coke` rows do not
    # contain it, and a substring filter silently misses them.
    voc = art["meta"]["coal"]["minfac_vocabulary"]
    com_ok = {k for k, e in voc["commodity"].items() if e["admitted"]}
    com_no = {k for k, e in voc["commodity"].items() if not e["admitted"]}
    ft_no = {k for k, e in voc["fac_type"].items() if not e["admitted"]}
    zf = zipfile.ZipFile(os.path.join(ROOT, "spheres-web", "data",
                                      "minfac-csv.zip"))
    csv_name = [n for n in zf.namelist() if n.lower().endswith(".csv")][0]
    raw = list(csv.DictReader(io.StringIO(
        zf.read(csv_name).decode("utf-8", "replace"))))
    cand = [r for r in raw
            if (r["commodity"] or "").strip() in (com_ok | com_no)]
    rej = [r for r in cand if (r["commodity"] or "").strip() in com_no
           or (r["fac_type"] or "").strip() in ft_no]
    ok(len(cand) == 380 and len(rej) == 21 and len(cand) - len(rej) == 359,
       f"the coal admission rules re-derive from raw minfac.csv: "
       f"{len(cand)} candidates, {len(rej)} removed, {len(cand) - len(rej)} "
       f"admitted", f"{len(cand)}/{len(rej)}")
    ok(any("coking plant" in (r["fac_name"] or "").lower()
           and r["country"].strip() == "Germany" for r in rej),
       "the Ruhr's pitside coking plants are among the rows the rules remove")
    cited_ids = {s["id"] for d in D if D[d].get("coal")
                 for s in D[d]["coal"].get("sites", ())
                 if s.get("src") == "minfac"}
    leaked = [(r["rec_id"], r["fac_name"]) for r in rej
              if r["rec_id"] in cited_ids]
    ok(not leaked, "no removed coke or plant row is cited by any district",
       f"{leaked[:4]}")

    # Doctrine: coal is presence, never magnitude. No attachment may carry a
    # tonnage, a capacity or a reserve under any name.
    MAG = {"tonnes", "tonnage", "tons", "kt", "mt", "capacity", "reserves",
           "reserve", "output", "production"}
    mag = [(d, k) for d in sorted(D) if coal(d)
           for part in (coal(d).get("fields", ()), coal(d).get("sites", ()))
           for row in part for k in row if k.lower() in MAG]
    ok(not mag, "no coal field or coal mine carries a tonnage, capacity or "
       "reserve figure", f"{mag[:5]}")

    # --- 4. determinism ---------------------------------------------------
    print("\nCHECK 4: determinism")
    if fast:
        print("  SKIP  (--fast)")
    else:
        before = hashlib.sha256(open(ART, "rb").read()).hexdigest()
        subprocess.run([sys.executable, os.path.join(HERE, "make_resources.py")],
                       cwd=ROOT, check=True, stdout=subprocess.DEVNULL)
        after = hashlib.sha256(open(ART, "rb").read()).hexdigest()
        ok(before == after, "regeneration is byte-identical",
           f"{before[:12]} != {after[:12]}")

    print(f"\n{CHECKS[0]} checks, {len(FAILURES)} failed")
    if FAILURES:
        for f in FAILURES:
            print("  FAILED:", f)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
