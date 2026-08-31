#!/usr/bin/env python
# check.py — ground-truth verification of the committed resource artifact.
# Run LAST, after make_resources.py (see README). Companion to, not a
# replacement for, check_resources.py: that script audits the artifact against
# its own declared rules, this one holds it against the world.
#
# Inputs (read-only, anchored on the repo root this file sits in):
#   spheres-web/data/district_resources.json  (the committed artifact)
#   spheres-web/data/mrds-csv.zip             (USGS MRDS, raw — the plant guard)
#   spheres-web/data/ds896-aluminum.xlsx      (USGS DS896 — the Guinea ledger)
#   spheres-sim/data/districts.json           (district roster, READ ONLY)
#
# Invocation:  python tools/resources/check.py          (deterministic, no RNG)
#              python tools/resources/check.py --fast   (skip regeneration)
#
# This file never imports sources.py or make_resources.py for the admission
# rules. The plant guard below re-reads mrds.csv column by column and rebuilds
# the admitted set from scratch, so a bug shared between the generator and its
# own checker cannot hide here. That independence is the point.
#
# Checks:
#   1. STRUCTURE — every cited district id is in the roster, the artifact's own
#      coverage counters match the file they describe, and no mineral entry
#      carries a magnitude of any kind.
#   2. THE PLANT GUARD — independently re-derived from raw mrds.csv. Every cited
#      site must be a Producer/Past Producer, must NOT be a Processing Plant or
#      a Geothermal operation, and must name its commodity in commod1/commod2
#      rather than only in commod3. This is the check the correction pass exists
#      for; a smelter returning under any commodity fails it.
#   3. POSITIVE GROUND TRUTH — the 1990 mining and petroleum world, pinned by
#      district id: the Persian Gulf, the Copperbelt and Katanga, Antofagasta,
#      the Pilbara and Weipa, the Witwatersrand and the Bushveld, Krivoy Rog and
#      the Urals, Jamaica, Khouribga. Each reports its measured band.
#   4. NEGATIVE GROUND TRUTH — the absences that carry as much weight as the
#      presences: no bauxite in Norway or Japan, nothing at all in Switzerland
#      or Singapore, no district anywhere sourced from a processing plant.
#   5. APPORTIONMENT — the arithmetic of ruling 2. Every apportioned volume is
#      exactly `known` x `area_frac_province`, no province apportions more than
#      it holds, `known` is identical everywhere one province is cited, and the
#      three named rankings (Iraq, Nigeria, Mexico) read as measured.
#   6. THE HONEST-LIMIT LEDGER — numbers that must stay visible because they
#      bound what the artifact can be used for: the single-record share, the
#      continental coverage spread, the MRDS coordinate collisions, and the
#      three structural holes. Drift in these FAILS; the holes themselves WARN,
#      because they are the source's shape and not a regression.
#   7. DETERMINISM — regenerate and require byte-identical output.

import csv
import hashlib
import io
import json
import os
import subprocess
import sys
import zipfile

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(HERE))
ART = os.path.join(ROOT, "spheres-web", "data", "district_resources.json")
ROSTER = os.path.join(ROOT, "spheres-sim", "data", "districts.json")
MRDS = os.path.join(ROOT, "spheres-web", "data", "mrds-csv.zip")
DS896_AL = os.path.join(ROOT, "spheres-web", "data", "ds896-aluminum.xlsx")
MINFAC = os.path.join(ROOT, "spheres-web", "data", "minfac-csv.zip")

failures = []
warnings = []
n_checks = [0]


def check(section, cond, msg):
    n_checks[0] += 1
    if cond:
        print(f"  PASS  {msg}")
    else:
        print(f"  FAIL  {msg}")
        failures.append((section, msg))


def warn(section, msg):
    print(f"  WARN  {msg}")
    warnings.append((section, msg))


def note(msg):
    print(f"        {msg}")


# --- MRDS admission vocabulary, transcribed from USGS mrds.met ---------------
# Re-stated here rather than imported. If sources.py and this file ever
# disagree, the disagreement is the finding.
DEV_ADMITTED = {"Producer", "Past Producer"}
DEV_REJECTED = {"Occurrence", "Prospect", "Unknown", "Plant"}
OPER_REJECTED = {"Processing Plant", "Geothermal"}
TIER_ADMITTED = ("commod1", "commod2")
TIER_REJECTED = "commod3"

# --- nation -> continent, hand-authored over the roster's own 160 names ------
# Used only for the coverage-spread ledger in section 6. Districts shared by a
# successor state and the USSR resolve to the first nation in sorted order,
# which is deterministic and puts each district under its own modern continent.
CONTINENT = {
    "Afghanistan": "Asia", "Albania": "Europe", "Algeria": "Africa",
    "Angola": "Africa", "Argentina": "SouthAmerica", "Armenia": "Asia",
    "Australia": "Oceania", "Austria": "Europe", "Azerbaijan": "Asia",
    "Bahamas": "NorthAmerica", "Bahrain": "Asia", "Bangladesh": "Asia",
    "Belarus": "Europe", "Belgium": "Europe", "Belize": "NorthAmerica",
    "Bhutan": "Asia", "Bolivia": "SouthAmerica", "Bosnia": "Europe",
    "Botswana": "Africa", "Brazil": "SouthAmerica", "Brunei": "Asia",
    "Bulgaria": "Europe", "Cambodia": "Asia", "Cameroon": "Africa",
    "Canada": "NorthAmerica", "CapeVerde": "Africa",
    "CentralAfricanRepublic": "Africa", "Chad": "Africa",
    "Chile": "SouthAmerica", "China": "Asia", "Colombia": "SouthAmerica",
    "Comoros": "Africa", "Congo": "Africa", "CostaRica": "NorthAmerica",
    "Croatia": "Europe", "Cuba": "NorthAmerica", "Cyprus": "Asia",
    "Czechoslovakia": "Europe", "Denmark": "Europe",
    "DominicanRepublic": "NorthAmerica", "EastTimor": "Asia",
    "Ecuador": "SouthAmerica", "Egypt": "Africa", "ElSalvador": "NorthAmerica",
    "EquatorialGuinea": "Africa", "Estonia": "Europe", "Ethiopia": "Africa",
    "Fiji": "Oceania", "Finland": "Europe", "France": "Europe",
    "Gabon": "Africa", "Georgia": "Asia", "Germany": "Europe",
    "Ghana": "Africa", "Greece": "Europe", "Guatemala": "NorthAmerica",
    "Guyana": "SouthAmerica", "Haiti": "NorthAmerica",
    "Honduras": "NorthAmerica", "Hungary": "Europe", "Iceland": "Europe",
    "India": "Asia", "Indonesia": "Asia", "Iran": "Asia", "Iraq": "Asia",
    "Ireland": "Europe", "Israel": "Asia", "Italy": "Europe",
    "Jamaica": "NorthAmerica", "Japan": "Asia", "Jordan": "Asia",
    "Kazakhstan": "Asia", "Kenya": "Africa", "Kuwait": "Asia",
    "Kyrgyzstan": "Asia", "Laos": "Asia", "Latvia": "Europe",
    "Lebanon": "Asia", "Lesotho": "Africa", "Libya": "Africa",
    "Lithuania": "Europe", "Luxembourg": "Europe", "Macedonia": "Europe",
    "Madagascar": "Africa", "Malawi": "Africa", "Malaysia": "Asia",
    "Maldives": "Asia", "Malta": "Europe", "Mauritius": "Africa",
    "Mexico": "NorthAmerica", "Moldova": "Europe", "Mongolia": "Asia",
    "Montenegro": "Europe", "Morocco": "Africa", "Mozambique": "Africa",
    "Myanmar": "Asia", "Namibia": "Africa", "Nepal": "Asia",
    "Netherlands": "Europe", "NewZealand": "Oceania",
    "Nicaragua": "NorthAmerica", "Nigeria": "Africa", "NorthKorea": "Asia",
    "Norway": "Europe", "Oman": "Asia", "Pakistan": "Asia",
    "Panama": "NorthAmerica", "PapuaNewGuinea": "Oceania",
    "Paraguay": "SouthAmerica", "Peru": "SouthAmerica", "Philippines": "Asia",
    "Poland": "Europe", "Portugal": "Europe", "Qatar": "Asia",
    "Romania": "Europe", "Russia": "Europe", "Samoa": "Oceania",
    "SaoTome": "Africa", "SaudiArabia": "Asia", "Senegal": "Africa",
    "Serbia": "Europe", "Seychelles": "Africa", "Singapore": "Asia",
    "Slovenia": "Europe", "SolomonIslands": "Oceania",
    "SouthAfrica": "Africa", "SouthKorea": "Asia", "Spain": "Europe",
    "SriLanka": "Asia", "Sudan": "Africa", "Suriname": "SouthAmerica",
    "Swaziland": "Africa", "Sweden": "Europe", "Switzerland": "Europe",
    "Syria": "Asia", "Taiwan": "Asia", "Tajikistan": "Asia",
    "Tanzania": "Africa", "Thailand": "Asia", "Tonga": "Oceania",
    "TrinidadTobago": "NorthAmerica", "Tunisia": "Africa", "Turkey": "Asia",
    "Turkmenistan": "Asia", "UAE": "Asia", "UK": "Europe",
    "USA": "NorthAmerica", "USSR": "Europe", "Uganda": "Africa",
    "Ukraine": "Europe", "Uruguay": "SouthAmerica", "Uzbekistan": "Asia",
    "Vanuatu": "Oceania", "Venezuela": "SouthAmerica", "Vietnam": "Asia",
    "Yemen": "Asia", "Yugoslavia": "Europe", "Zaire": "Africa",
    "Zambia": "Africa", "Zimbabwe": "Africa",
}


def load():
    with open(ART, encoding="utf-8") as fh:
        art = json.load(fh)
    with open(ROSTER, encoding="utf-8") as fh:
        roster = json.load(fh)["nations"]
    return art, roster


def entry(art, did, com):
    return art["districts"].get(did, {}).get(com)


def band_of(art, did, com):
    e = entry(art, did, com)
    return None if e is None else e["confidence"]["band"]


def nation_districts(roster, nation):
    return [d["id"] for d in roster.get(nation, ())]


def carriers(art, roster, nation, com):
    """District ids of `nation` carrying `com`, with band and support."""
    out = []
    for did in nation_districts(roster, nation):
        e = entry(art, did, com)
        if e is None:
            continue
        support = (e["apportionment"]["total"] if e.get("province_level")
                   else e.get("n", 0))
        out.append((support, did, e["confidence"]["band"]))
    out.sort(key=lambda r: -r[0])
    return out


def ranked(art, roster, nation, com):
    return [(did, band, support)
            for support, did, band in carriers(art, roster, nation, com)]


# =============================================================================
def main():
    fast = "--fast" in sys.argv
    art, roster = load()
    D = art["districts"]
    names = {d["id"]: d["name"] for n in roster for d in roster[n]}

    # -- 1. STRUCTURE ---------------------------------------------------------
    print("=" * 78)
    print("1. STRUCTURE")
    roster_ids = set(names)
    check("structure", not (set(D) - roster_ids),
          f"all {len(D)} districts in the artifact exist in the roster")
    check("structure", art["meta"]["districts_with_any_resource"] == len(D),
          f"meta.districts_with_any_resource ({art['meta']['districts_with_any_resource']}) "
          f"equals the districts block ({len(D)})")
    check("structure", art["meta"]["districts_total"] == len(roster_ids),
          f"meta.districts_total ({art['meta']['districts_total']}) equals the "
          f"roster ({len(roster_ids)} distinct ids)")
    note(f"coverage {len(D)}/{len(roster_ids)} = {100.0*len(D)/len(roster_ids):.2f}%")

    entries = [(did, com, v) for did, e in D.items() for com, v in e.items()]
    petro = [t for t in entries if t[1] in ("oil", "gas")]
    mineral = [t for t in entries if t[1] not in ("oil", "gas")]
    note(f"{len(entries)} district-commodity entries "
         f"({len(mineral)} mineral, {len(petro)} petroleum)")

    # No mineral entry may carry a magnitude. `n` is a citation count and is
    # declared as such; anything numeric beyond it would be an invented tonnage.
    # Coal adds `field_level` and `fields` — a named coal-bearing AREA measured
    # against the district — and those carry square kilometres, which is a
    # measurement of ground and not of ore. The distinction is enforced one
    # level down: no key inside a coal field may name a quantity of coal.
    allowed_mineral = {"src", "n", "bands", "sites", "confidence",
                       "field_level", "fields"}
    stray = sorted({k for _, _, v in mineral for k in v} - allowed_mineral)
    check("structure", not stray,
          f"no mineral entry carries a magnitude key (keys seen: "
          f"{sorted(allowed_mineral)})")
    if stray:
        note(f"unexpected keys: {stray}")

    allowed_field = {"src", "name", "polygons", "sqkm_field_total",
                     "intersection_sqkm", "area_frac_district", "area_frac_field",
                     "note", "dep_age", "province", "rank", "age"}
    fields = [f for _, _, v in mineral for f in (v.get("fields") or ())]
    stray_f = sorted({k for f in fields for k in f} - allowed_field)
    check("structure", not stray_f,
          f"no coal-field attachment carries a tonnage key ({len(fields)} "
          f"attachments, keys allowed: {sorted(allowed_field)})")
    if stray_f:
        note(f"unexpected field keys: {stray_f}")
    check("structure",
          all("no tonnage exists in this source" in (f.get("note") or "").lower()
              for f in fields),
          "every coal-field attachment says in the artifact that it carries no tonnage")

    check("structure",
          all(v.get("apportionment", {}).get("derived") is True for _, _, v in petro),
          f"all {len(petro)} petroleum entries flag their apportionment DERIVED")
    check("structure",
          all(p.get("shared") is True
              for _, _, v in petro for p in v.get("provinces", ())),
          "every cited province volume is flagged `shared`")
    check("structure", "DO NOT MULTIPLY" in art["meta"]["apportionment"].upper()
          or "NEVER" in art["meta"]["apportionment"].upper(),
          "meta.apportionment warns against reading the derived split as production")

    # -- 2. THE PLANT GUARD ---------------------------------------------------
    print()
    print("=" * 78)
    print("2. THE PLANT GUARD  (re-derived from raw mrds.csv, not from sources.py)")
    tokens = {}
    for com, meta in art["commodities"].items():
        if meta.get("location_source") == "mrds":
            for t in meta["source_tokens"]:
                tokens.setdefault(t, set()).add(com)

    cited = {}
    for did, e in D.items():
        for com, v in e.items():
            if v.get("src") != "mrds":
                continue
            for s in v.get("sites", ()):
                cited.setdefault(s["id"], set()).add(com)
    pairs = sum(len(v) for v in cited.values())
    note(f"{len(cited)} distinct cited MRDS site ids, {pairs} (site, commodity) pairs")

    rows = {}
    n_rows = 0
    with zipfile.ZipFile(MRDS) as z, z.open("mrds.csv") as fh:
        rdr = csv.DictReader(io.TextIOWrapper(fh, encoding="utf-8", errors="replace"))
        for row in rdr:
            n_rows += 1
            dep = (row.get("dep_id") or "").strip()
            if dep in cited:
                rows[dep] = row
    note(f"{n_rows} MRDS records read")

    check("plant-guard", len(rows) == len(cited),
          f"every cited site id resolves to a real MRDS record "
          f"({len(rows)}/{len(cited)})")

    bad_dev, bad_oper, bad_tier = [], [], []
    for dep, coms in sorted(cited.items()):
        row = rows.get(dep)
        if row is None:
            continue
        dev = (row.get("dev_stat") or "").strip()
        oper = (row.get("oper_type") or "").strip()
        primary = set()
        for f in TIER_ADMITTED:
            primary |= {t.strip() for t in (row.get(f) or "").split(",") if t.strip()}
        if dev not in DEV_ADMITTED:
            bad_dev.append((dep, row.get("site_name"), dev))
        if oper in OPER_REJECTED:
            bad_oper.append((dep, row.get("site_name"), oper, sorted(coms)))
        for com in coms:
            want = {t for t, cs in tokens.items() if com in cs}
            if not (want & primary):
                bad_tier.append((dep, row.get("site_name"), com))

    check("plant-guard", not bad_dev,
          f"every cited site is a Producer or Past Producer "
          f"({len(cited) - len(bad_dev)}/{len(cited)})")
    for b in bad_dev[:5]:
        note(f"dev_stat violation: {b}")
    check("plant-guard", not bad_oper,
          "NO cited site is a Processing Plant or a Geothermal operation "
          f"(0 of {len(cited)}) — the correction pass holds")
    for b in bad_oper[:5]:
        note(f"plant leaked through: {b}")
    check("plant-guard", not bad_tier,
          f"every (site, commodity) pair names its commodity in commod1/commod2, "
          f"never only in commod3 ({pairs - len(bad_tier)}/{pairs})")
    for b in bad_tier[:5]:
        note(f"commod3-only violation: {b}")

    # Steep Rock and the Delta Steel plant, named because they are the two the
    # correction pass was written against.
    check("plant-guard", "10157857" not in cited,
          "Steep Rock (10157857) is cited by no commodity (commod1/2 both empty)")
    check("plant-guard", "10304733" not in cited,
          "Dsc Warri (10304733), the Delta Steel PLANT, is cited by no commodity")

    # -- 2b. THE COKE GUARD ---------------------------------------------------
    # The same discipline for coal, and for the same reason. minfac files coke
    # OVENS beside coal MINES, and one of its commodity strings is "Coke:
    # contained in domestic coal" — the word "coal" is in it, five of them are
    # in the Ruhr, and admitting them would put Bottrop on the map twice for the
    # wrong reason. Re-derived here from raw minfac.csv, so a bug shared between
    # sources.py and the generator cannot hide in both.
    print()
    print("  -- the coke guard (re-derived from raw minfac.csv)")
    coal_cited = {}
    for did, e in D.items():
        for s in e.get("coal", {}).get("sites", ()):
            if s.get("src") == "minfac":
                coal_cited.setdefault(s["id"], []).append((did, s))
    mf_rows = {}
    mf_coalish = 0
    with zipfile.ZipFile(MINFAC) as z, z.open("minfac.csv") as fh:
        rdr = csv.DictReader(io.TextIOWrapper(fh, encoding="utf-8", errors="replace"))
        for row in rdr:
            com = (row.get("commodity") or "").strip().lower()
            if "coal" in com or "coke" in com:
                mf_coalish += 1
                mf_rows[(row.get("rec_id") or "").strip()] = row
    note(f"{mf_coalish} minfac rows mention coal or coke; "
         f"{len(coal_cited)} distinct ids cited by the artifact")
    check("coke-guard", all(i in mf_rows for i in coal_cited),
          f"every cited minfac coal id resolves to a real minfac record "
          f"({sum(1 for i in coal_cited if i in mf_rows)}/{len(coal_cited)})")
    leaked_coke = [(i, mf_rows[i].get("fac_name"), mf_rows[i].get("commodity"))
                   for i in sorted(coal_cited)
                   if i in mf_rows and "coke" in (mf_rows[i]["commodity"] or "").lower()]
    leaked_plant = [(i, mf_rows[i].get("fac_name"), mf_rows[i].get("fac_type"))
                    for i in sorted(coal_cited)
                    if i in mf_rows and (mf_rows[i]["fac_type"] or "").strip() == "Plant"]
    n_coke = sum(1 for r in mf_rows.values()
                 if "coke" in (r["commodity"] or "").lower())
    n_plant = sum(1 for r in mf_rows.values() if (r["fac_type"] or "").strip() == "Plant")
    check("coke-guard", not leaked_coke,
          f"NO coke record is cited as coal ({n_coke} coke rows in the source, "
          f"0 admitted)")
    for b in leaked_coke[:5]:
        note(f"coke leaked through: {b}")
    check("coke-guard", not leaked_plant,
          f"NO record classified `Plant` is cited as a coal mine "
          f"({n_plant} such rows in the source, 0 admitted)")
    for b in leaked_plant[:5]:
        note(f"plant leaked through: {b}")
    # Every cited coal site must carry the coordinate its source published.
    mismatch = [(i, s["at"], mf_rows[i]["latitude"], mf_rows[i]["longitude"])
                for i, hits in sorted(coal_cited.items()) if i in mf_rows
                for _d, s in hits
                if s["at"] != "%.5f,%.5f" % (float(mf_rows[i]["latitude"]),
                                             float(mf_rows[i]["longitude"]))]
    check("coke-guard", not mismatch,
          f"every cited coal site sits at the coordinate minfac published, "
          f"uncorrected ({len(mismatch)} mismatches)")
    for b in mismatch[:3]:
        note(f"coordinate moved: {b}")

    # -- 3. POSITIVE GROUND TRUTH --------------------------------------------
    print()
    print("=" * 78)
    print("3. POSITIVE GROUND TRUTH  (the 1990 world, pinned by district id)")

    def present(section, did, com, label, min_band=None):
        e = entry(art, did, com)
        if e is None:
            check(section, False, f"{label}: {did} carries {com} — ABSENT")
            return None
        b = e["confidence"]["band"]
        if e.get("province_level"):
            cov = e["confidence"]["max_area_frac_district"]
            detail = (f"band={b}, province covers {cov:.1%} of the district, "
                      f"{e['apportionment']['total']:,.0f} "
                      f"{e['apportionment']['units']} apportioned")
        else:
            c = e["confidence"]
            bits = []
            if e.get("sites"):
                bits.append(f"{c['distinct_coordinates']} distinct coordinates "
                            f"from {c['records']} records")
                if c["centroid_stacked"]:
                    bits.append(f"{c['centroid_coordinates']} of them a filing "
                                f"centroid holding {c['centroid_records']} records")
            if e.get("fields"):
                top = max(e["fields"], key=lambda f: f["area_frac_district"])
                bits.append(f"{c['coal_fields']} named coal field(s), "
                            f"{top['name']} covering "
                            f"{top['area_frac_district']:.1%} of the district")
            detail = f"band={b}, " + "; ".join(bits)
        okband = True
        if min_band:
            order = ["single", "sparse", "moderate", "strong"]
            okband = order.index(b) >= order.index(min_band)
        check(section, okband,
              f"{label}: {did} {names.get(did,'?')} carries {com} — {detail}"
              + ("" if okband else f"  [wanted >= {min_band}]"))
        return e

    print("  -- Persian Gulf")
    gulf = [("IQ-BA", "Iraq / Al-Basrah"), ("KW-AH", "Kuwait / Al Ahmadi"),
            ("SA-04", "Saudi / Ash Sharqiyah"), ("IR-10", "Iran / Khuzestan"),
            ("AE-AZ", "UAE / Abu Dhabi"), ("QA-RA", "Qatar / Ar Rayyan")]
    for did, label in gulf:
        present("gulf", did, "oil", label, "strong")
    gulf_nations = ["Iraq", "Kuwait", "SaudiArabia", "Iran", "UAE", "Qatar", "Bahrain"]
    tot = sum(len(nation_districts(roster, n)) for n in gulf_nations)
    with_oil = sum(1 for n in gulf_nations for d in nation_districts(roster, n)
                   if "oil" in D.get(d, {}))
    check("gulf", with_oil >= 60,
          f"the Gulf littoral carries oil broadly: {with_oil} of {tot} districts "
          f"across {len(gulf_nations)} nations ({100.0*with_oil/tot:.1f}%)")

    print("  -- the Central African Copperbelt")
    present("copperbelt", "ZM-08", "copper", "Zambia / Copperbelt", "strong")
    present("copperbelt", "CD-KA", "copper", "Zaire / Katanga", "strong")
    present("copperbelt", "CD-KA", "cobalt", "Zaire / Katanga cobalt", "strong")

    print("  -- the Andes")
    present("andes", "CL-AN", "copper", "Chile / Antofagasta", "strong")

    print("  -- Australia")
    present("australia", "AU-WA", "iron", "Pilbara (Western Australia)", "strong")
    e = entry(art, "AU-WA", "iron")
    pilbara = {"Mount Tom Price Mine", "Mount Whaleback Mine", "Paraburdoo Mine",
               "Robe River Mine"}
    got = {s["name"] for s in e["sites"]} if e else set()
    check("australia", pilbara & got,
          f"the Pilbara mines are the evidence, by name: "
          f"{sorted(pilbara & got)}")
    # Weipa wanted `strong` in the first edition and got it on six records —
    # but four of the six are filed at -22.57045,144.54695, the centroid of
    # Queensland, 1,300 km from Cape York. Two coordinates in Cape York are the
    # whole of the real evidence, so `moderate` is the honest reading and the
    # expectation moves to it. Ruling 4 caught this ground-truth check passing
    # on the same defect it was written to expose.
    present("australia", "AU-QLD", "bauxite", "Queensland (Weipa)", "moderate")
    present("australia", "AU-NT", "bauxite", "Northern Territory (Gove)", "sparse")

    print("  -- South Africa")
    present("southafrica", "ZA-GT", "gold", "Witwatersrand (Gauteng)", "strong")
    present("southafrica", "ZA-FS", "gold", "Free State goldfield", "strong")
    present("southafrica", "ZA-LP", "platinum_group", "Bushveld (Limpopo)", "strong")
    present("southafrica", "ZA-NW", "platinum_group", "Bushveld (North West)", "sparse")
    check("southafrica", entry(art, "ZA-WC", "gold") is None,
          "the Western Cape, which has no goldfield, carries no gold")

    print("  -- the Soviet iron belt")
    present("soviet", "UA-12", "iron", "Krivoy Rog (Dnipropetrovs'k)", "sparse")
    present("soviet", "RU-SVE", "iron", "the Urals (Sverdlovsk)", "strong")
    present("soviet", "RU-BEL", "iron", "the KMA (Belgorod)", "sparse")
    ru = ranked(art, roster, "Russia", "iron")
    ua = ranked(art, roster, "Ukraine", "iron")
    check("soviet", len(ru) >= 10 and len(ua) >= 2,
          f"Russia carries iron in {len(ru)} districts, Ukraine in {len(ua)}")

    print("  -- the bauxite islands and the phosphate desert")
    present("bauxite", "JM-12", "bauxite", "Jamaica / Manchester", "strong")
    jm = ranked(art, roster, "Jamaica", "bauxite")
    check("bauxite", len(jm) >= 6,
          f"Jamaica carries bauxite in {len(jm)} of "
          f"{len(nation_districts(roster,'Jamaica'))} districts")
    present("phosphate", "MA-09", "phosphate", "Khouribga (Chaouia-Ouardigha)", "sparse")
    present("phosphate", "MA-15", "phosphate", "Bou Craa (Laayoune)", "sparse")
    ma = ranked(art, roster, "Morocco", "phosphate")
    check("phosphate", len(ma) >= 4,
          f"Morocco carries phosphate in {len(ma)} districts")

    # -- the coal hole, and whether it is filled ------------------------------
    # Six coalfields chosen before the sources were read, because they are the
    # ones that decide 1990 politics: the Ruhr, Upper Silesia, the Donbas,
    # Appalachia, Shanxi and the British pits after the strike. Each is pinned
    # by district id and each reports the measurement it was banded on, so a
    # later edition cannot quietly lose one.
    print("  -- the coal hole (the six fields that decide 1990)")
    present("coal", "DE-NW", "coal", "the RUHR (Nordrhein-Westfalen)", "moderate")
    ruhr = entry(art, "DE-NW", "coal")
    ruhr_named = {s["name"] for s in (ruhr or {}).get("sites", ())}
    check("coal", {"Prosper-Haniel Mine", "Walsum Mine", "Lippe Mine"} & ruhr_named,
          f"the Ruhr's evidence is named collieries: "
          f"{sorted(n for n in ruhr_named if 'Mine' in n)[:4]}")
    present("coal", "PL-SL", "coal", "UPPER SILESIA (Slaskie)")
    silesia = entry(art, "PL-SL", "coal")
    check("coal", silesia is not None
          and silesia["confidence"]["distinct_coordinates"] == 1
          and silesia["confidence"]["centroid_stacked"],
          "Upper Silesia is `single` and FLAGGED: the Yearbook files seventeen "
          "mines as one row and eight rows stack on 50.17N 18.83E, so one "
          "coordinate is one site's worth of evidence (ruling 4, working)")
    present("coal", "UA-14", "coal", "the DONBAS (Donets'ka)", "moderate")
    present("coal", "UA-09", "coal", "the Donbas (Luhans'ka)", "moderate")
    donbas = entry(art, "UA-14", "coal")
    check("coal", donbas is not None
          and any(f["name"] == "Donetsky basin" for f in donbas.get("fields", ())),
          "the Donbas is named by its source: OFR 01-104's `Donetsky basin` "
          "polygon, not a guess at where the Donbas is")
    present("coal", "US-WV", "coal", "APPALACHIA (West Virginia)", "strong")
    present("coal", "US-PA", "coal", "Appalachia (Pennsylvania)", "moderate")
    present("coal", "US-KY", "coal", "Appalachia (Kentucky)", "moderate")
    appal = entry(art, "US-WV", "coal")
    check("coal", appal is not None
          and any(f["name"] == "Appalachian Region" for f in appal.get("fields", ())),
          "Appalachia is named by its source: OFR 2012-1205's `Appalachian "
          "Region`, and West Virginia is measured against it, not assigned to it")
    present("coal", "CN-SX", "coal", "SHANXI", "strong")
    shanxi = entry(art, "CN-SX", "coal")
    check("coal", shanxi is not None
          and shanxi["confidence"]["distinct_coordinates"] >= 100,
          f"Shanxi rests on "
          f"{shanxi['confidence']['distinct_coordinates'] if shanxi else 0} "
          f"distinct mine coordinates — minfac alone would have given it one "
          f"province centroid at 37N 112E")
    uk_coal = [d for d in nation_districts(roster, "UK") if "coal" in D.get(d, {})]
    check("coal", len(uk_coal) >= 6,
          f"BRITISH COAL after the strike: {len(uk_coal)} districts carry coal — "
          f"{sorted(uk_coal)[:4]}")
    uk_named = {s["name"] for d in uk_coal for s in D[d]["coal"].get("sites", ())}
    check("coal", {"Kellingley Colliery", "Tower Colliery", "Maltby Colliery"} & uk_named,
          f"and they are named pits: "
          f"{sorted(n for n in uk_named if 'Colliery' in n)[:4]}")

    print("  -- the Soviet coal basins, by their own names")
    for did, field, label in (
            ("RU-KEM", "Kuznetsky basin", "the KUZBASS (Kemerovo)"),
            ("KZ-KAR", "Karagandinsky basin", "KARAGANDA"),
            ("KZ-PAV", "Ekibastuzsky basin", "EKIBASTUZ (Pavlodar)"),
            ("RU-KO", "Pechorsky basin", "the PECHORA basin (Komi)")):
        e = entry(art, did, "coal")
        got = {f["name"] for f in (e or {}).get("fields", ())}
        check("coal", e is not None and field in got,
              f"{label}: {did} carries `{field}` — "
              f"{'yes' if e and field in got else f'NO (has {sorted(got)[:3]})'}")

    # -- 4. NEGATIVE GROUND TRUTH --------------------------------------------
    print()
    print("=" * 78)
    print("4. NEGATIVE GROUND TRUTH  (absences that carry as much weight)")

    def absent(section, nation, com, label):
        hits = ranked(art, roster, nation, com)
        check(section, not hits,
              f"{label}: {nation} carries no {com} anywhere "
              f"({len(nation_districts(roster, nation))} districts checked)")
        if hits:
            note(f"unexpected: {hits}")
        return hits

    absent("no-bauxite", "Norway", "bauxite",
           "Tyssedal and Mosjoen are aluminium SMELTERS")
    absent("no-bauxite", "Japan", "bauxite",
           "Japan has no bauxite ore geology")

    for nation in ("Switzerland", "Singapore"):
        hits = [d for d in nation_districts(roster, nation) if d in D]
        check("empty", not hits,
              f"{nation} carries no located resource of any kind "
              f"({len(nation_districts(roster, nation))} districts, "
              f"{len(hits)} with an entry)")

    # The two assertions that used to stand here — "no district anywhere on
    # earth carries coal" and "the Ruhr carries no coal" — were true of the
    # artifact and false of the world. They are replaced by their opposites in
    # group 3, which pin the Ruhr, Silesia, the Donbas, Appalachia and Shanxi by
    # district id. What survives as a negative is the one absence the coal
    # sources genuinely do assert: coke plants are not coal mines.
    coke_ruhr = [s for e in D.values() for s in (e.get("coal", {}).get("sites") or ())
                 if "coke" in (s.get("commodity") or "").lower()]
    check("empty", not coke_ruhr,
          "no coke plant was admitted as a coal mine "
          "(minfac files 14 coke rows under a commodity string containing 'coal')")
    us_petro = [d for d in nation_districts(roster, "USA")
                if "oil" in D.get(d, {}) or "gas" in D.get(d, {})]
    check("empty", not us_petro,
          "no US district carries oil or gas — WEP assessed only provinces "
          "OUTSIDE the United States")

    # Germany and France: report the measured value rather than assert a zero
    # the source will not support. Both are recorded as WARN, with the record
    # that keeps them non-zero named, so the number can be argued but not lost.
    for nation, expect_zero_label in (("Germany", "DE-TH Vogelsberg Mountain"),
                                      ("France", "the Var bauxite field")):
        hits = ranked(art, roster, nation, "bauxite")
        if hits:
            warn("no-bauxite",
                 f"{nation} carries bauxite in {len(hits)} district(s): "
                 + ", ".join(f"{d} {names.get(d,'?')} ({b}, n={int(s)})"
                             for d, b, s in hits)
                 + f" — NOT zero; the surviving evidence is {expect_zero_label}")
        else:
            check("no-bauxite", True, f"{nation} carries no bauxite anywhere")

    # -- 5. APPORTIONMENT -----------------------------------------------------
    print()
    print("=" * 78)
    print("5. APPORTIONMENT  (ruling 2 arithmetic)")
    # The artifact publishes `area_frac_province` to 6 dp and `apportioned` to
    # 3 dp, so the identity can only be verified to the artifact's own precision.
    # The bound below is that precision and nothing looser: a half-ulp of the
    # published fraction times the province total, plus a half-ulp of the
    # published volume. Anything the generator actually got wrong lands outside
    # it. Measured worst case is 0.9946 of the bound — every value fits under,
    # none is merely close to a slack tolerance.
    def round_bound(known):
        return known * 5e-7 + 5e-4

    n_arith = 0
    arith_bad = []
    worst = (0.0, None)
    per_province = {}
    contributors = {}
    known_of = {}
    known_clash = []
    for did, com, v in petro:
        for p in v.get("provinces", ()):
            n_arith += 1
            want = p["known"] * p["area_frac_province"]
            resid = abs(want - p["apportioned"])
            bound = round_bound(p["known"])
            if resid > bound:
                arith_bad.append((did, com, p["code"], p["apportioned"], want, bound))
            if bound and resid / bound > worst[0]:
                worst = (resid / bound, (did, com, p["code"]))
            key = (p["code"], com)
            per_province[key] = per_province.get(key, 0.0) + p["apportioned"]
            contributors[key] = contributors.get(key, 0) + 1
            if key in known_of and known_of[key] != p["known"]:
                known_clash.append((key, known_of[key], p["known"]))
            known_of[key] = p["known"]
    check("apportion", not arith_bad,
          f"every apportioned volume is known x area_frac_province to the "
          f"artifact's own published precision ({n_arith} checked)")
    note(f"worst residual is {worst[0]:.4f} of the rounding bound, at {worst[1]}")
    for b in arith_bad[:5]:
        note(f"arithmetic drift beyond rounding: {b}")
    check("apportion", not known_clash,
          f"every province's `known` total is identical wherever it is cited "
          f"({len(known_of)} province-commodity totals)")
    # Same reasoning on the sum: each of a province's N district shares is
    # rounded to 3 dp, so the total may sit up to 0.0005*N above `known` without
    # anything having been invented. Measured worst excess is +0.002 on a
    # 30,731-unit province with 30 contributors.
    over = [(k, v, known_of[k], v - known_of[k], contributors[k])
            for k, v in per_province.items()
            if v > known_of[k] + 5e-4 * contributors[k]]
    excess = max(((v - known_of[k]) for k, v in per_province.items()
                  if v > known_of[k]), default=0.0)
    check("apportion", not over,
          f"no province apportions more than it holds, to rounding "
          f"({len(per_province)} province-commodity totals summed; worst excess "
          f"+{excess:.4f})")
    for b in over[:5]:
        note(f"over-apportioned beyond rounding: {b}")

    def rank_of(nation, com, did):
        r = ranked(art, roster, nation, com)
        for i, (d, _, _) in enumerate(r, 1):
            if d == did:
                return i, len(r)
        return None, len(r)

    for nation, com, did, label in (("Iraq", "oil", "IQ-BA", "Al-Basrah"),
                                    ("Iraq", "oil", "IQ-AN", "Al-Anbar"),
                                    ("Nigeria", "oil", "NG-DE", "Delta"),
                                    ("Nigeria", "oil", "NG-CR", "Cross River"),
                                    ("Mexico", "oil", "MX-TAB", "Tabasco"),
                                    ("Mexico", "oil", "MX-CAM", "Campeche")):
        i, n = rank_of(nation, com, did)
        note(f"{nation} {com}: {label} ({did}) ranks {i} of {n} "
             f"by apportioned volume, band={band_of(art, did, com)}")

    ib, _ = rank_of("Iraq", "oil", "IQ-BA")
    ia, _ = rank_of("Iraq", "oil", "IQ-AN")
    check("apportion", ib < ia,
          f"Iraq: Al-Basrah (#{ib}) out-ranks Al-Anbar (#{ia}) — the named "
          f"failure is inverted")
    check("apportion", band_of(art, "IQ-BA", "oil") == "strong"
          and band_of(art, "IQ-AN", "oil") == "moderate",
          f"Al-Basrah bands strong and Al-Anbar moderate "
          f"({entry(art,'IQ-BA','oil')['confidence']['max_area_frac_district']:.4f} "
          f"vs {entry(art,'IQ-AN','oil')['confidence']['max_area_frac_district']:.4f} "
          f"district coverage)")
    nd, _ = rank_of("Nigeria", "oil", "NG-DE")
    check("apportion", nd == 1,
          f"Nigeria: Delta is the leading oil district (#{nd})")
    nc, ntot = rank_of("Nigeria", "oil", "NG-CR")
    check("apportion", nc > 5,
          f"Nigeria: Cross River has fallen to #{nc} of {ntot}")

    # Al-Basrah is NOT Iraq's #1 and the artifact must not pretend otherwise.
    if ib != 1:
        top = ranked(art, roster, "Iraq", "oil")[0]
        warn("apportion",
             f"Al-Basrah is Iraq's #{ib} oil district, not #1 — "
             f"{top[0]} {names.get(top[0],'?')} leads at {top[2]:,.0f} MMBO. "
             f"WEP publishes one volume for the whole Mesopotamian Foredeep and "
             f"holds no field geometry, so Rumaila and West Qurna cannot be placed.")

    # -- 6. THE HONEST-LIMIT LEDGER ------------------------------------------
    print()
    print("=" * 78)
    print("6. THE HONEST-LIMIT LEDGER")

    bands = {}
    for _, _, v in entries:
        bands[v["confidence"]["band"]] = bands.get(v["confidence"]["band"], 0) + 1
    single_share = 100.0 * bands.get("single", 0) / len(entries)
    note(f"bands: " + ", ".join(f"{k} {bands[k]}" for k in sorted(bands)))
    note(f"single-site / single-province entries: {bands.get('single',0)} of "
         f"{len(entries)} = {single_share:.1f}%")
    check("limits", 20.0 <= single_share <= 34.0,
          f"the single-evidence share is {single_share:.1f}% — kept and rendered "
          f"weaker, never filtered (ruling 1)")

    # Ruling 4. The bucket is DISTINCT COORDINATES, and it is recomputed here
    # from the entry's own coordinate count rather than read off its band, so a
    # generator that banded on the wrong number fails this instead of agreeing
    # with itself.
    # A coal entry that also rests on a named coal field is banded on the
    # STRONGER of two measurements and is checked separately below; the pure
    # point entries must still band on coordinates alone.
    point = [v for _, _, v in entries
             if v.get("sites") and not v.get("fields")]
    buckets = {"1": 0, "2": 0, "3-5": 0, "6+": 0}
    misbanded = []
    for v in point:
        c = v["confidence"]
        n = c["distinct_coordinates"]
        buckets["1" if n == 1 else "2" if n == 2 else "3-5" if n <= 5 else "6+"] += 1
        want = ("strong" if n >= 6 else "moderate" if n >= 3
                else "sparse" if n == 2 else "single")
        if c["band"] != want or c.get("banded_on") != "distinct_coordinates":
            misbanded.append((c["band"], want, n))
        if c["records"] < n:
            misbanded.append(("records<coords", c["records"], n))
    pt = len(point)
    note(f"point-sourced entries by DISTINCT COORDINATES: "
         + ", ".join(f"{k}->{buckets[k]}" for k in ("1", "2", "3-5", "6+"))
         + f"  (n={pt})")
    check("limits", not misbanded,
          f"every one of the {pt} point-sourced entries bands on its distinct "
          f"coordinate count, recomputed here independently"
          + ("" if not misbanded else f"  [{misbanded[:3]}]"))
    one = buckets["1"]
    check("limits", one > 0 and one / pt < 0.5,
          f"{one} of {pt} point-sourced entries rest on ONE distinct site "
          f"({100.0*one/pt:.1f}%) — the honest headline, and it is not small")

    # The coal-field band, recomputed the same independent way. An entry that
    # holds both kinds of evidence must publish BOTH bands and take the stronger
    # of them; taking the weaker would hide the Donets Basin behind one Yearbook
    # row, and taking an average would invent a grade neither source supports.
    ORDER = ("single", "sparse", "moderate", "strong")

    def area_band(frac):
        return ("strong" if frac >= 0.50 else "moderate" if frac >= 0.15
                else "sparse" if frac >= 0.02 else "single")

    fielded = [v for _, _, v in entries if v.get("fields")]
    badfield = []
    for v in fielded:
        c = v["confidence"]
        top = max(f["area_frac_district"] for f in v["fields"])
        want_f = area_band(top)
        if c.get("band_on_fields") != want_f or c.get("coal_fields") != len(v["fields"]):
            badfield.append(("field", c.get("band_on_fields"), want_f, top))
            continue
        if v.get("sites"):
            n = c["distinct_coordinates"]
            want_p = ("strong" if n >= 6 else "moderate" if n >= 3
                      else "sparse" if n == 2 else "single")
            want = max((want_p, want_f), key=ORDER.index)
            if (c.get("band_on_points") != want_p or c["band"] != want
                    or c.get("banded_on") != "stronger_of_points_and_fields"):
                badfield.append(("both", c["band"], want, n, top))
        elif c["band"] != want_f or c.get("banded_on") != "coal_field_area_frac":
            badfield.append(("fieldonly", c["band"], want_f, top))
    both = sum(1 for v in fielded if v.get("sites"))
    note(f"coal-field entries: {len(fielded)} ({both} of them also holding "
         f"mine points, banded on the stronger of the two)")
    check("limits", fielded and not badfield,
          f"every one of the {len(fielded)} coal-field entries bands on its "
          f"measured area share, recomputed here independently"
          + ("" if not badfield else f"  [{badfield[:3]}]"))

    cont_of = {}
    for nation in sorted(roster):
        for d in roster[nation]:
            cont_of.setdefault(d["id"], CONTINENT[nation])
    unmapped = [n for n in roster if n not in CONTINENT]
    check("limits", not unmapped, f"every roster nation has a continent ({unmapped})")
    ctot, ccov = {}, {}
    for did, c in cont_of.items():
        ctot[c] = ctot.get(c, 0) + 1
        if did in D:
            ccov[c] = ccov.get(c, 0) + 1
    pcts = {c: 100.0 * ccov.get(c, 0) / ctot[c] for c in ctot}
    for c in sorted(pcts, key=lambda x: pcts[x]):
        note(f"{c:<14} {ccov.get(c,0):4d} / {ctot[c]:4d} = {pcts[c]:5.1f}%")
    lo = min(pcts, key=lambda c: pcts[c])
    hi = max(pcts, key=lambda c: pcts[c])
    spread = pcts[hi] - pcts[lo]
    check("limits", spread > 30.0,
          f"continental coverage spread is {spread:.1f} points "
          f"({lo} {pcts[lo]:.1f}% to {hi} {pcts[hi]:.1f}%) — survey effort, "
          f"not endowment")
    warn("limits",
         f"coverage is NOT uniform: {lo} at {pcts[lo]:.1f}% against {hi} at "
         f"{pcts[hi]:.1f}%. Reading district count as endowment inverts the "
         f"real 1990 map.")

    # MRDS coordinate collisions — the defect ruling 4 was written against, and
    # the check that it is actually gone. `rows` is rebuilt independently from
    # raw mrds.csv by the plant guard above, so this reads the collision off the
    # source and not off the artifact's own bookkeeping.
    coords = {}
    for dep, row in rows.items():
        coords.setdefault((row.get("latitude"), row.get("longitude")), []).append(dep)
    dup = {k: v for k, v in coords.items() if len(v) > 1}
    in_dup = sum(len(v) for v in dup.values())
    worst = max(dup.items(), key=lambda kv: (len(kv[1]), kv[0])) if dup else (None, [])
    note(f"cited MRDS records sharing a coordinate with another: {in_dup} of "
         f"{len(rows)} ({100.0*in_dup/len(rows):.1f}%), over {len(dup)} coordinates")
    note(f"worst collision: {worst[0]} carries {len(worst[1])} distinct cited records")
    check("limits", in_dup > 0,
          "coordinate collisions are measured, not assumed away")

    fr = entry(art, "FRA_centre-val-de-loire", "bauxite")
    check("limits", fr is not None and fr["confidence"]["band"] == "single"
          and fr["n"] == 6 and fr["confidence"]["distinct_coordinates"] == 1,
          f"the named defect is closed: FRA_centre-val-de-loire bauxite still "
          f"carries all {fr['n'] if fr else 0} transcribed records and now bands "
          f"`{fr['confidence']['band'] if fr else '?'}` on "
          f"{fr['confidence']['distinct_coordinates'] if fr else 0} distinct "
          f"coordinate (it read `strong`)")
    check("limits", fr is not None and fr["confidence"]["centroid_stacked"]
          and all(s["at_centroid"] for s in fr["sites"]),
          "and every one of its sites is flagged `at_centroid`, so the map and "
          "the hover can say the point is a filing centroid")

    flagged = [v for v in point if v["confidence"]["centroid_stacked"]]
    whole = [v for v in flagged if v["confidence"]["unflagged_coordinates"] == 0]
    note(f"filing-centroid flag: {len(flagged)} of {pt} point-sourced entries "
         f"({100.0*len(flagged)/pt:.1f}%) touch one, {len(whole)} rest on "
         f"nothing else")
    check("limits", flagged and len(flagged) < pt // 2,
          f"the centroid flag is selective: {len(flagged)} entries flagged, "
          f"{len(whole)} of them wholly")
    warn("limits",
         f"{len(whole)} entries have NO unflagged coordinate at all: every point "
         f"under them is a filing centroid, so they are located to an "
         f"administrative unit and no further. The band cannot say that on its "
         f"own — `confidence.unflagged_coordinates` is the number to read.")

    # -- the Guinea hole ------------------------------------------------------
    print()
    print("  -- the unrostered producer")
    guinea_recs = []
    with zipfile.ZipFile(MRDS) as z, z.open("mrds.csv") as fh:
        rdr = csv.DictReader(io.TextIOWrapper(fh, encoding="utf-8", errors="replace"))
        for row in rdr:
            if (row.get("country") or "").strip() != "Guinea":
                continue
            prim = set()
            for f in TIER_ADMITTED:
                prim |= {t.strip() for t in (row.get(f) or "").split(",") if t.strip()}
            if "Aluminum" not in prim:
                continue
            if (row.get("dev_stat") or "").strip() not in DEV_ADMITTED:
                continue
            if (row.get("oper_type") or "").strip() in OPER_REJECTED:
                continue
            guinea_recs.append((row.get("dep_id"), row.get("site_name")))
    check("guinea", "Guinea" not in roster,
          f"the district roster models {len(roster)} nations and Guinea is not "
          f"one of them")
    check("guinea", len(guinea_recs) == 10,
          f"MRDS holds {len(guinea_recs)} ADMITTED bauxite-extraction records for "
          f"Guinea: {', '.join(sorted(n for _, n in guinea_recs))}")
    check("guinea", not any(dep in cited for dep, _ in guinea_recs),
          "none of them is cited by any district — there is no district to hold them")
    check("guinea", "Guinea" not in art["national"]["bauxite"],
          f"Guinea is absent from national.bauxite "
          f"({len(art['national']['bauxite'])} nations)")
    up_bx = {u["nation"] for u in art["unlocated_producers"].get("bauxite", ())}
    check("guinea", "Guinea" not in up_bx,
          f"Guinea is absent from unlocated_producers.bauxite ({sorted(up_bx)})")
    warn("guinea",
         "OPEN HOLE: Guinea produced 15,800,000 t of bauxite in 1990 (USGS DS896 "
         "sheet 2) — 14.0% of the 113,000,000 t world total and rank 2 behind "
         "Australia. MRDS locates 10 of its mines correctly. Both are dropped "
         "because the roster has no Guinea, and the shipped artifact says so "
         "NOWHERE: crosswalk.IGNORE swallows the name and the drop is not "
         "recorded in unlocated_producers. Doctrine requires absence PLUS an "
         "explicit unlocated marker; the marker is missing.")

    print()
    print("  -- the three structural holes")
    warn("holes",
         "1. NO PER-DISTRICT MAGNITUDE for any mineral. Districts carry presence "
         "and a citation count only; 1990 production is national and undivided.")
    warn("holes",
         f"2. NO LOCATED US PETROLEUM. {len(us_petro)} US districts carry oil or "
         f"gas; the USA's 7,355.32 kb/d sits in unlocated_producers.")
    coal_d = [d for d in D if "coal" in D[d]]
    coal_unloc = len(art.get("unlocated_producers", {}).get("coal", ()))
    warn("holes",
         f"3. COAL IS LOCATED BUT NOT AT 1990. {len(coal_d)} districts carry "
         f"coal from four USGS sources dated 2001-2012; {coal_unloc} nations "
         f"produced coal in 1990 and still have none located. The Ruhr shows "
         f"the nine collieries open in 2006, not the twenty-odd working in "
         f"1990, and British coal shows the twenty pits left in 2007. The hole "
         f"that remains is VINTAGE, not absence, and it understates 1990.")

    # -- 7. DETERMINISM -------------------------------------------------------
    print()
    print("=" * 78)
    print("7. DETERMINISM")
    if fast:
        print("  SKIP  (--fast)")
    else:
        before = hashlib.sha256(open(ART, "rb").read()).hexdigest()
        subprocess.run([sys.executable, os.path.join(HERE, "make_resources.py")],
                       cwd=ROOT, check=True, stdout=subprocess.DEVNULL)
        after = hashlib.sha256(open(ART, "rb").read()).hexdigest()
        check("determinism", before == after,
              f"regeneration is byte-identical ({before[:16]}...)")
        note(f"sha256 {after}   {os.path.getsize(ART)} bytes")

    # =========================================================================
    print()
    print("=" * 78)
    print(f"RESULT: {n_checks[0]} checks, {len(failures)} failure(s), "
          f"{len(warnings)} warning(s)")
    for s, m in failures:
        print(f"  FAIL [{s}] {m}")
    for s, m in warnings:
        print(f"  WARN [{s}] {m}")
    print("VERDICT:", "PASS" if not failures else "FAIL")
    return 0 if not failures else 1


if __name__ == "__main__":
    sys.exit(main())
