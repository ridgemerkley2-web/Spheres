#!/usr/bin/env python3
"""
tools/resources/make_resources_1990.py — the sim's 1990 resource table.

Writes `spheres-sim/data/resources_1990.json`. Run from the repo root, after
`make_resources.py` (this reads that artifact) and before
`check_resources_1990.py`:

    python tools/resources/make_resources_1990.py
    python tools/resources/check_resources_1990.py

WHAT THIS IS
------------
The join `district_resources.json` refuses to make, made once, by one written
rule, for the one consumer that needs it: the sim's HAVE ledger
(scratchpad SPEC-RESOURCE-SYSTEM.md section 1.2 and package D1, section 2).
That artifact carries WHERE (deposit points, province polygons) and HOW MUCH (a
1990 national figure) and never multiplies them. The sim has to know, when a
district changes hands, what fraction of the 1990 owner's figure went with it.
So this file carries, per tracked commodity and per 1990 producer, a LOCATION
WEIGHT per district - a share that sums to one over the producer's located
districts - beside the untouched national figure. The share is never a tonnage.
Multiplied by the figure it says "this much of the 1990 owner's output sits
here", which is a statement about the map; the figure itself is copied
verbatim and is asserted exact (the USSR's iron is 236,000,000 t, DS-896).

THE SHARE RULE (D: derived from transcribed inputs by this rule and nothing else)
  oil, gas               `apportionment.total` - the area-weighted province volume
  bauxite, copper, iron  MRDS sites in the Producer band; all admitted sites
                         where the nation has no Producer-band site
  coal                   mine points (any band); else coalfield intersection km2
Shares below 1e-3 are PRUNED and the rest renormalised: the coalfield fallback
leaves ~50 Soviet districts at 1e-6, and a 1e-6 district must not count as
located, as a district count, or as a war aim. A producer with no located
district after pruning is UNLOCATED and keeps its whole figure while it lives.
OIL SHARES LOCATE `oil_mbd` AND NEVER CREATE OUTPUT: the sim's oil ledger is the
nation's `oil_mbd`; `national_1990.oil` is carried for the record and the shares
say where that ledger sits on the map.

THE ONE NEW TRANSCRIPTION in this file is `price_1990`: a 1990 unit value per
tracked mined line, each with its source string verbatim and the arithmetic
that turns the printed figure into dollars per the sim's unit. Oil has no row -
the sim prices it at `w.oil_price`. A price that cannot be sourced is OMITTED
and `meta.prices_omitted` says so; nothing is estimated.

WHAT IS DROPPED, AND SAID
`national` keys that are not 1990 start nations are not seated: Namibia is a
roster seat that comes alive later and holds no 1990 district, so its rows are
listed in `meta.dropped_keys` and carried nowhere else. Zero rows (a nation the
source lists at 0) are omitted and counted in `meta.counts.zero_rows_omitted`.

DETERMINISM: no RNG, no wall clock, no set iteration reaching the output. Every
map emits sorted; floats are Python repr (full precision, round-trip exact);
the file is written with '\\n' newlines. Two runs are byte-identical, which
`check_resources_1990.py` verifies.
"""

import collections
import hashlib
import json
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
if HERE not in sys.path:
    sys.path.insert(0, HERE)

# The shared paths, so this generator reads the same files the others write.
from geo import DATA, DISTRICTS_JSON, ROOT                    # noqa: E402

ART = os.path.join(DATA, "district_resources.json")
POP = os.path.join(DATA, "district_population.json")
NATIONS_RS = os.path.join(ROOT, "spheres-sim", "src", "nations.rs")
NATIONS_DIR = os.path.join(ROOT, "spheres-sim", "data", "nations")
OUT = os.path.join(ROOT, "spheres-sim", "data", "resources_1990.json")

# Twelve lines, alphabetical, and that order IS the bit index in `presence`
# and the column index in `quality`. The sim's `Commodity` enum mirrors it.
COMMODITIES = [
    "bauxite", "coal", "cobalt", "copper", "gas", "gold", "iron", "oil",
    "phosphate", "platinum_group", "rare_earths", "uranium",
]
# The six with a transcribed 1990 national figure in the artifact.
TRACKED = ["bauxite", "coal", "copper", "gas", "iron", "oil"]
# The sim's unit per tracked line, and the artifact unit string it is a
# relabelling of. A row whose artifact unit differs is a refusal, not a guess.
UNITS = {"bauxite": "t", "coal": "kt", "copper": "t", "gas": "bcf",
         "iron": "t", "oil": "kb/d"}
ART_UNITS = {"bauxite": "metric tons", "coal": "1000 metric tons",
             "copper": "metric tons", "gas": "billion cubic feet",
             "iron": "metric tons", "oil": "thousand barrels per day"}
PRUNE = 1e-3
BAND_RANK = {"strong": 3, "moderate": 2, "sparse": 1, "single": 1}

SHORT_TON_T = 0.90718474      # metric tons per short ton (exact definition)
LB_PER_T = 2204.62262185      # pounds per metric ton

# ---------------------------------------------------------------------------
# price_1990 - THE ONE NEW TRANSCRIPTION (T). One row per tracked mined line,
# the printed figure as printed, the conversion to dollars per the sim's unit
# written out, and the source. `None` means "not sourced": the row is omitted
# and meta.prices_omitted carries the reason. Nothing here is estimated.
# ---------------------------------------------------------------------------
PRICE_1990 = {
    "coal": {
        "as_printed": "21.76 dollars per short ton (Total, nominal), 1990",
        "usd_per_unit": 21.76 / SHORT_TON_T * 1000.0,
        "conversion": "21.76 $/short ton / 0.90718474 t per short ton x 1000 t per kt",
        "source": "EIA Annual Energy Review 2011, Table 7.9 'Coal Prices, "
                  "1949-2011 (Dollars per Short Ton)', 1990 row, Total, "
                  "nominal: 21.76 (average open market mine price). "
                  "https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0709",
    },
    "gas": {
        "as_printed": "1.71 dollars per thousand cubic feet, 1990",
        "usd_per_unit": 1.71 * 1.0e6,
        "conversion": "1.71 $/Mcf x 1,000,000 Mcf per bcf",
        "source": "EIA Natural Gas data, 'U.S. Natural Gas Wellhead Price "
                  "(Dollars per Thousand Cubic Feet)', annual series N9190US3, "
                  "1990: 1.71. https://www.eia.gov/dnav/ng/hist/n9190us3a.htm",
    },
    "copper": {
        "as_printed": "123.16 cents per pound (Price: Producer, weighted average), 1990",
        "usd_per_unit": 123.16 / 100.0 * LB_PER_T,
        "conversion": "123.16 c/lb / 100 x 2204.62262185 lb per metric ton",
        "source": "U.S. Bureau of Mines, Minerals Yearbook 1994, vol. I, Copper, "
                  "Table 1 'Salient copper statistics', row 'Price: Producer, "
                  "weighted average, cents per pound', 1990 column: 123.16 (United "
                  "States producer refined copper). https://d9-wret.s3.us-west-2."
                  "amazonaws.com/assets/palladium/production/mineral-pubs/copper/240494.pdf",
    },
    "iron": {
        "as_printed": "27.52 dollars per metric ton (Average value at mines, usable ore "
                      "shipped; r/ revised), 1990",
        "usd_per_unit": 27.52,
        "conversion": "27.52 $/t; the sim's unit is the metric ton",
        "source": "U.S. Bureau of Mines, Minerals Yearbook 1994, vol. I, Iron Ore, "
                  "Table 1 'Salient iron ore statistics', row 'Average value at mines, "
                  "dollars per ton', 1990 column: $27.52 r/ (f.o.b. mine, usable ore "
                  "shipped, United States). https://d9-wret.s3.us-west-2.amazonaws.com/"
                  "assets/palladium/production/mineral-pubs/iron-ore/340494.pdf",
    },
    "bauxite": {
        "as_printed": "27.2 dollars per metric ton (Unit value ($/t)), 1990",
        "usd_per_unit": 27.2,
        "conversion": "27.2 $/t; the sim's unit is the metric ton",
        "source": "USGS Data Series 140, Historical Statistics for Mineral and Material "
                  "Commodities in the United States: Bauxite (sheet 'Bauxite', last "
                  "modification January 19, 2017), column 'Unit value ($/t)', 1990 row: "
                  "27.2 - nominal dollars per metric ton of apparent consumption (U.S. "
                  "production is withheld; the value is that of imported bauxite). "
                  "https://d9-wret.s3.us-west-2.amazonaws.com/assets/palladium/production/"
                  "mineral-pubs/historical-statistics/ds140-bauxi.xlsx",
    },
}
PRICE_UNSOURCED = {
    # Filled in when a row above is None: the reason the figure is missing.
}


# ---------------------------------------------------------------------------
# Inputs
# ---------------------------------------------------------------------------

def load_json(path):
    with open(path, encoding="utf-8") as f:
        return json.load(f)


def sha256_of(path):
    with open(path, "rb") as f:
        return hashlib.sha256(f.read()).hexdigest()


def roster_codes():
    """Every `row("Code"` of nations.rs, in roster order. The join between the
    artifact's `national` keys and the sim is this string: `NationId`'s debug
    name is its code, and the artifact's crosswalk already writes codes."""
    with open(NATIONS_RS, encoding="utf-8") as f:
        return re.findall(r'row\("([A-Za-z]+)"', f.read())


def start_nations(codes):
    """The codes with a 1990 data file: on the board in January 1990. A roster
    seat without one (Russia, Namibia, ...) is a successor and holds nothing
    at the start."""
    slugs = {f[:-5] for f in os.listdir(NATIONS_DIR) if f.endswith(".json")}
    return [c for c in codes if c.lower() in slugs]


def owners_1990(districts_json, start):
    """district id -> its 1990 owner. Start nations in roster order; a
    federation's list already carries its republics, and no district may be
    listed under two start nations."""
    owner = {}
    for code in start:
        for d in districts_json["nations"].get(code, []):
            prev = owner.setdefault(d["id"], code)
            if prev != code:
                raise SystemExit(f"{d['id']} is listed under both {prev} and {code}")
    return owner


def weight(c, entry):
    """The documented location weight of one district for one commodity.
    Zero means 'present but carries no weight under the rule' (an offshore
    province whose apportioned volume is 0, a coal field with no measured
    intersection)."""
    x = entry.get(c)
    if not x:
        return 0.0
    if c in ("oil", "gas"):
        return float(x.get("apportionment", {}).get("total", 0.0))
    if c == "coal":
        pts = sum(x.get("bands", {}).values())
        if pts > 0:
            return float(pts)
        return float(sum(f.get("intersection_sqkm", 0.0) for f in x.get("fields", [])))
    bands = x.get("bands", {})
    prod = float(bands.get("Producer", 0))
    return prod if prod > 0 else float(sum(bands.values()))


# ---------------------------------------------------------------------------
# The build
# ---------------------------------------------------------------------------

def build():
    art = load_json(ART)
    pop = load_json(POP)["nations"]
    dj = load_json(DISTRICTS_JSON)
    codes = roster_codes()
    start = start_nations(codes)
    owner = owners_1990(dj, start)
    D = art["districts"]
    start_set = set(start)

    # national_1990 -- the figure, verbatim, keyed by code.
    national = {c: {} for c in TRACKED}
    dropped_rows = collections.defaultdict(dict)
    zero_omitted = {}
    for c in TRACKED:
        zero_omitted[c] = 0
        for n in sorted(art["national"][c]):
            rec = art["national"][c][n]
            if rec["units"] != ART_UNITS[c]:
                raise SystemExit(f"{c}/{n}: units {rec['units']!r}, expected {ART_UNITS[c]!r}")
            v = float(rec["value"])
            if n not in start_set:
                dropped_rows[n][c] = v
                continue
            if v <= 0.0:
                zero_omitted[c] += 1
                continue
            national[c][n] = {"value": v, "source": rec["source"]}

    # located -- shares per 1990 producer, pruned and renormalised.
    located = {c: {} for c in TRACKED}
    pruned = {c: {} for c in TRACKED}
    unlocated = {c: [] for c in TRACKED}
    rows = {}
    for c in TRACKED:
        by = collections.defaultdict(list)
        for d in sorted(D):
            if c in D[d] and d in owner:
                w = weight(c, D[d])
                if w > 0.0:
                    by[owner[d]].append((d, w))
        rows[c] = 0
        for n in sorted(national[c]):
            cand = by.get(n, [])
            if not cand:
                unlocated[c].append(n)
                continue
            tot = sum(w for _, w in cand)
            shares = [(d, w / tot) for d, w in cand]
            keep = [(d, s) for d, s in shares if s >= PRUNE]
            if len(keep) < len(shares):
                pruned[c][n] = len(shares) - len(keep)
            if not keep:
                raise SystemExit(f"{c}/{n}: every share pruned")
            t2 = sum(s for _, s in keep)
            keep = [(d, s / t2) for d, s in keep]
            keep.sort(key=lambda t: (-t[1], t[0]))
            located[c][n] = [[d, s] for d, s in keep]
            rows[c] += len(keep)

    # presence and quality -- every district the artifact places anything in.
    rank = {}
    for c in ("oil", "gas"):
        groups = collections.defaultdict(list)
        for d in sorted(D):
            if c in D[d]:
                groups[owner.get(d, "")].append((d, weight(c, D[d])))
        for g in sorted(groups):
            lst = sorted(groups[g], key=lambda t: (-t[1], t[0]))
            n = len(lst)
            for p, (d, _) in enumerate(lst):
                rank[(c, d)] = 3 if 3 * p < n else (2 if 3 * p < 2 * n else 1)
    presence, quality = {}, {}
    for d in sorted(D):
        mask, q = 0, [0] * 12
        for i, c in enumerate(COMMODITIES):
            if c in D[d]:
                mask |= 1 << i
                if c in ("oil", "gas"):
                    q[i] = rank[(c, d)]
                else:
                    q[i] = BAND_RANK[D[d][c]["confidence"]["band"]]
        if mask == 0:
            raise SystemExit(f"{d}: in the artifact with no commodity")
        presence[d] = mask
        quality[d] = q

    # pop_share / pop_1990 -- of the 1990 owner, every district it held.
    pop_share, pop_1990 = {}, {}
    for d in sorted(owner):
        rec = pop[owner[d]]["districts"][d]
        pop_share[d] = float(rec["share"])
        pop_1990[d] = int(rec["pop_1990"])

    # price_1990 -- the transcription, or the stated absence.
    price, omitted = {}, {}
    for c in TRACKED:
        if c == "oil":
            continue
        row = PRICE_1990.get(c)
        if row is None:
            omitted[c] = PRICE_UNSOURCED.get(c, "no sourced 1990 unit value transcribed")
            continue
        if not (row["usd_per_unit"] > 0.0 and row["source"].strip()):
            raise SystemExit(f"price_1990.{c}: a positive figure and a source are required")
        price[c] = {k: row[k] for k in ("usd_per_unit", "as_printed", "conversion", "source")}

    used_sources = sorted({r["source"] for c in TRACKED for r in national[c].values()})
    meta = {
        "generator": "tools/resources/make_resources_1990.py",
        "upstream": {
            "generator": art["meta"]["generator"],
            "vintage": art["meta"]["vintage"],
            "sources": {k: {"title": art["sources"][k]["title"],
                            "url": art["sources"][k]["url"],
                            "role": art["sources"][k]["role"]} for k in used_sources},
        },
        "source_sha256": {
            "district_resources.json": sha256_of(ART),
            "district_population.json": sha256_of(POP),
            "districts.json": sha256_of(DISTRICTS_JSON),
        },
        "dropped_keys": sorted(dropped_rows),
        "rules": {
            "share": "oil,gas: apportionment.total; bauxite,copper,iron: Producer-band MRDS "
                     "sites, all sites if none; coal: mine points, else coalfield km2. "
                     "Shares < 1e-3 pruned, remainder renormalised.",
            "oil": "shares LOCATE the nation's oil_mbd and never create output",
            "unlocated": "a producer with no located district keeps its national figure "
                         "while it lives",
            "located": "located[c][n] lists only 1990 producers with >= 1 located district; "
                       "each list sums to 1, every share >= 1e-3, sorted share desc then id; "
                       "a producer absent here is unlocated (national-only)",
            "national": "national_1990[c][n] is the artifact's 1990 figure verbatim, keyed by "
                        "roster code, 1990 start nations only, zero rows omitted",
            "dropped": "national keys that are not 1990 start nations are not seated: Namibia "
                       "is a roster seat that comes alive later and holds no 1990 district",
            "presence": "presence[d] is a 12-bit mask over `commodities` in order: bit i set "
                        "when commodity i is present in d at any band or level",
            "quality": "quality[d][i] is a presence rank used only to choose a war aim, never "
                       "a tonnage: oil,gas = tertile of apportionment.total within the 1990 "
                       "owner (3 top, 2, 1); others = confidence band (strong 3, moderate 2, "
                       "sparse or single 1); 0 = absent",
            "pop_share": "district_population.json share of the 1990 owner, every district it "
                         "held; pop_1990 is the same file's GHS-POP 1990 count (a modelled "
                         "surface, not a census line)",
            "price": "price_1990[c].usd_per_unit is dollars per one `units[c]`, derived from "
                     "the printed figure by the stated conversion; oil has no row (the sim "
                     "prices it at w.oil_price); an unsourced line is omitted and named in "
                     "prices_omitted",
        },
        "counts": {
            "presence_districts": len(presence),
            "located_rows": rows,
            "located_nations": {c: len(located[c]) for c in TRACKED},
            "national_rows": {c: len(national[c]) for c in TRACKED},
            "pruned": pruned,
            "unlocated_producers": unlocated,
            "zero_rows_omitted": zero_omitted,
            "dropped_rows": {n: dropped_rows[n] for n in sorted(dropped_rows)},
            "pop_districts": len(pop_share),
        },
        "prices_omitted": omitted,
    }
    return {
        "meta": meta,
        "commodities": COMMODITIES,
        "tracked": TRACKED,
        "units": UNITS,
        "price_1990": price,
        "national_1990": national,
        "located": located,
        "presence": presence,
        "quality": quality,
        "pop_share": pop_share,
        "pop_1990": pop_1990,
    }


# ---------------------------------------------------------------------------
# Emission: one entry per line at the first three depths so a diff reads as a
# list of nations or districts, leaves compact, every map sorted.
# ---------------------------------------------------------------------------

def compact(v):
    return json.dumps(v, sort_keys=True, separators=(",", ":"), ensure_ascii=True)


def render(obj, depth=0):
    if isinstance(obj, dict) and obj and (depth < 3 or any(isinstance(v, dict) for v in obj.values())):
        pad = " " * depth
        items = [f'{pad} {json.dumps(k)}: {render(obj[k], depth + 1)}' for k in sorted(obj)]
        return "{\n" + ",\n".join(items) + "\n" + pad + "}"
    return compact(obj)


def write(table, out):
    text = render(table) + "\n"
    with open(out, "w", encoding="utf-8", newline="\n") as f:
        f.write(text)
    return len(text.encode("utf-8"))


def main(argv):
    out = OUT
    if "--out" in argv:
        out = argv[argv.index("--out") + 1]
    table = build()
    n = write(table, out)
    m = table["meta"]["counts"]
    print(f"wrote {out} ({n} bytes)")
    print(f"  national rows {m['national_rows']}")
    print(f"  located nations {m['located_nations']}, rows {m['located_rows']}")
    print(f"  pruned {m['pruned']}")
    print(f"  unlocated {m['unlocated_producers']}")
    print(f"  zero rows omitted {m['zero_rows_omitted']}; dropped {table['meta']['dropped_keys']} {m['dropped_rows']}")
    print(f"  presence districts {m['presence_districts']}; pop districts {m['pop_districts']}")
    print(f"  prices {sorted(table['price_1990'])}; omitted {table['meta']['prices_omitted']}")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
