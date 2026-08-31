#!/usr/bin/env python3
"""
tools/resources/sources.py — readers for the six staged resource sources.

Each reader returns plain python and does no placement; `make_resources.py` owns
the transcription. Every reader is pure and order-stable.

The readers deliberately refuse to guess. Where a source encodes "not
applicable" (`XX`) or "zero" (`--`) they return None rather than 0.0, because a
Soviet republic that did not exist as a producer in 1990 is not a producer of
zero tonnes — it is not a row at all, and the distinction is the difference
between a transcription and a fabrication.

No third-party dependency: the shapefile and xlsx readers are the ~60-line pure
python ones verified during the probe, with a polygon branch added for WEP.
"""

import csv
import io
import json
import os
import struct
import zipfile
from xml.etree import ElementTree as ET

csv.field_size_limit(10 ** 9)

NS = "{http://schemas.openxmlformats.org/spreadsheetml/2006/main}"

# Values that mean "no datum", not "zero".
NULLISH = {"", "--", "XX", "NA", "(3)", "W", "(s)"}


# ---------------------------------------------------------------------------
# xlsx (USGS DS896)
# ---------------------------------------------------------------------------

def _col_to_idx(ref):
    s = "".join(c for c in ref if c.isalpha())
    n = 0
    for ch in s:
        n = n * 26 + (ord(ch) - 64)
    return n - 1


def read_xlsx(path, sheet_idx=0):
    z = zipfile.ZipFile(path)
    shared = []
    if "xl/sharedStrings.xml" in z.namelist():
        ss = ET.fromstring(z.read("xl/sharedStrings.xml"))
        for si in ss.iter(NS + "si"):
            shared.append("".join(t.text or "" for t in si.iter(NS + "t")))
    names = [s.get("name") for s in ET.fromstring(z.read("xl/workbook.xml")).iter(NS + "sheet")]
    sheets = sorted(n for n in z.namelist() if n.startswith("xl/worksheets/sheet"))
    ws = ET.fromstring(z.read(sheets[sheet_idx]))
    rows = []
    for row in ws.iter(NS + "row"):
        cells = {}
        for c in row.iter(NS + "c"):
            ref = c.get("r") or ""
            t = c.get("t")
            v = c.find(NS + "v")
            isel = c.find(NS + "is")
            if t == "s" and v is not None:
                val = shared[int(v.text)]
            elif isel is not None:
                val = "".join(x.text or "" for x in isel.iter(NS + "t"))
            elif v is not None:
                val = v.text
            else:
                val = ""
            cells[_col_to_idx(ref)] = val
        if cells:
            rows.append([cells.get(i, "") for i in range(max(cells) + 1)])
    return names, rows


def strip_footnote(name):
    """`U.S.S.R.6` -> `U.S.S.R.`, `Yugoslavia7` -> `Yugoslavia`.

    DS896 appends footnote markers straight onto the country name with no
    separator. Only a trailing digit is stripped, and only when the name has
    non-digit content left, so `Congo (Kinshasa)3` survives as
    `Congo (Kinshasa)`.
    """
    s = (name or "").strip()
    while s and s[-1].isdigit():
        s = s[:-1]
    return s.strip()


def ds896_year(path, sheet_idx, year="1990", label=None):
    """{country: value} for one DS896 sheet and one year.

    Locates the header row by its literal `Country` cell rather than a fixed
    offset — the sheets differ (copper carries a product-label column, bauxite
    and iron ore do not) and a hard-coded index would silently read the wrong
    year if USGS reflowed a sheet.
    """
    _names, rows = read_xlsx(path, sheet_idx)
    hdr = None
    for i, r in enumerate(rows):
        if r and (r[0] or "").strip() == "Country" and year in [(c or "").strip() for c in r]:
            hdr = i
            break
    if hdr is None:
        raise ValueError(f"{os.path.basename(path)} sheet {sheet_idx}: no Country/{year} header")
    head = [(c or "").strip() for c in rows[hdr]]
    ycol = head.index(year)
    lcol = 1 if head[1] and not head[1].isdigit() else None
    out = {}
    for r in rows[hdr + 1:]:
        if not r or not (r[0] or "").strip():
            continue
        country = strip_footnote(r[0])
        if not country or country[0].isdigit() or country.startswith("XX,"):
            continue          # footnote / legend rows
        if label is not None:
            if lcol is None or len(r) <= lcol or (r[lcol] or "").strip() != label:
                continue
        if len(r) <= ycol:
            continue
        raw = (r[ycol] or "").strip()
        if raw in NULLISH:
            continue
        try:
            out[country] = float(raw)
        except ValueError:
            continue
    return out


# ---------------------------------------------------------------------------
# EIA International Energy Statistics
# ---------------------------------------------------------------------------

EIA_SERIES = {
    "oil": ("Crude oil including lease condensate production", "TBPD"),
    "gas": ("Dry natural gas production", "BCF"),
    "coal": ("Coal production", "MT"),
}


def eia_year(path, year="1990"):
    """{key: {country: (value, units, geo)}} for the three energy series.

    Only single-country series are kept (`geography` without a `+`), and only
    where the year carries a real number. That filter alone selects the correct
    1990 vintage: EIA reports `--` for Russia, Ukraine and Kazakhstan in 1990 and
    puts the production under `Former U.S.S.R.` — the successor states are
    present as series but empty, exactly as the history requires.
    """
    out = {k: {} for k in EIA_SERIES}
    z = zipfile.ZipFile(path)
    with z.open("INTL.txt") as fh:
        for line in fh:
            try:
                d = json.loads(line)
            except Exception:
                continue
            sid = d.get("series_id") or ""
            if not sid.endswith(".A"):
                continue
            geo = d.get("geography") or ""
            if not geo or "+" in geo:
                continue
            nm = d.get("name") or ""
            for key, (phrase, unit) in EIA_SERIES.items():
                if not nm.startswith(phrase + ", ") or f"-{unit}.A" not in sid:
                    continue
                v = next((v for y, v in (d.get("data") or []) if y == year), None)
                if not isinstance(v, (int, float)):
                    continue
                country = nm[len(phrase) + 2:].rsplit(", Annual", 1)[0]
                out[key][country] = (float(v), d.get("units"), geo)
    return out


# ---------------------------------------------------------------------------
# USDA PSD (grains)
# ---------------------------------------------------------------------------

def psd_year(path, commodities, year="1990"):
    """{commodity: {country: value_1000MT}} for Production in one market year."""
    out = {c: {} for c in commodities}
    z = zipfile.ZipFile(path)
    name = z.namelist()[0]
    with z.open(name) as fh:
        rdr = csv.DictReader(io.TextIOWrapper(fh, encoding="utf-8", errors="replace"))
        for row in rdr:
            if (row.get("Market_Year") or "").strip() != year:
                continue
            if (row.get("Attribute_Description") or "").strip() != "Production":
                continue
            com = (row.get("Commodity_Description") or "").strip()
            if com not in out:
                continue
            try:
                val = float((row.get("Value") or "").strip())
            except ValueError:
                continue
            country = (row.get("Country_Name") or "").strip()
            if country:
                out[com][country] = val
    return out


# ---------------------------------------------------------------------------
# MRDS
# ---------------------------------------------------------------------------

# ---------------------------------------------------------------------------
# The MRDS admission rules
# ---------------------------------------------------------------------------
#
# Three rules decide whether an MRDS record is evidence that a district produced
# a commodity. Every value of every controlling vocabulary is enumerated below
# with a decision and a reason, and the reasons are USGS's own field definitions
# from `mrds.met`, quoted, not this project's opinion. Nothing is left to a
# default: a value that appears in the data and not in these tables raises.
#
# They are stated here rather than buried in the generator because they are the
# most consequential choices in the transcription. Getting them wrong is how the
# shipped artifact came to make Norway a bauxite producer on the strength of two
# aluminium smelters, and how Steep Rock Iron Mine came to be filed as a bauxite
# and phosphate deposit.

# RULE 1 — DEVELOPMENT STATUS. Did the site ever yield ore?
# Occurrences and prospects are geology; 1990 statecraft turned on mines that
# ran. `Plant` is USGS's own marker for a smelter or refinery and by its own
# definition "will have no geological information associated with it".
MRDS_DEV_STATUS = {
    "Producer":      (True,  "a mine in production at the time the data was entered"),
    "Past Producer": (True,  "a mine formerly operating that has closed"),
    "Occurrence":    (False, "no production has taken place; grade and extent essentially unknown"),
    "Prospect":      (False, "explored beyond occurrence, but no production asserted"),
    "Unknown":       (False, "development status not established by the source record"),
    "Plant":         (False, "a processing plant (smelter, refiner, beneficiation); "
                             "USGS: 'will have no geological information associated with it'"),
}

# RULE 2 — OPERATION TYPE. Is ore extracted at this coordinate?
# NEW in the 2026-08-31 correction pass. `Processing Plant` is USGS's own
# "No ore extraction at the site, only a mill, smelter, etc." — the value that
# makes an aluminium smelter look like a bauxite mine. `Geothermal` is an energy
# operation; it extracts heat, not ore. Everything else in the vocabulary is an
# extraction method, including `Well` and `Brine Operation` (solution mining and
# in-situ leaching are extraction through a borehole) and `Leach` (ore is mined
# first, then heaped on a pad).
MRDS_OPER_TYPE = {
    "Unknown":             (True,  "USGS: 'unknown or undetermined by evaluator' — silence, "
                                   "not a statement that no ore is extracted"),
    "Surface":             (True,  "open-pit, open-cast, quarry or strip mine"),
    "Underground":         (True,  "shaft or adit mine"),
    "Surface-Underground": (True,  "both surface and underground workings present"),
    "Placer":              (True,  "a stream-sediment or beach-sand mine"),
    "Well":                (True,  "product extracted through a borehole, including "
                                   "solution mining and in-situ leaching"),
    "Offshore":            (True,  "underwater mining operation"),
    "Brine Operation":     (True,  "product produced from a well or open pan"),
    "Leach":               (True,  "mined ore or concentrate heaped on a pad and percolated; "
                                   "the ore still came out of the ground here"),
    "Processing Plant":    (False, "USGS: 'No ore extraction at the site, only a mill, "
                                   "smelter, etc.' — the site consumes ore, it does not "
                                   "produce it"),
    "Geothermal":          (False, "USGS: 'energy extracted from heat stored in the earth' — "
                                   "an energy operation, not an ore mine"),
}

# RULE 3 — COMMODITY TIER. Is this commodity something the site produced?
# NEW in the 2026-08-31 correction pass. USGS defines the three commodity
# columns by economics, and the third one disqualifies itself:
#   commod1  "commodities that have a strong effect on the economics of the
#             project, and might be economically viable as the only commodity"
#   commod2  "commodities that can be economically recovered but have little
#             effect on the economic viability of the project"
#   commod3  "commodities that are economically interesting but NOT ECONOMICALLY
#             RECOVERABLE as of the date of the source information"
# A tertiary commodity is therefore a statement about assay, not about output.
# Reading it as production is what filed Steep Rock Iron Mine — commod1 and
# commod2 both empty, commod3 reading "Aluminum, Iron, Manganese, Silica,
# Sulfur, Phosphorus-Phosphates" — under bauxite AND iron AND phosphate.
MRDS_COMMODITY_FIELDS = ("commod1", "commod2")
MRDS_COMMODITY_FIELDS_EXCLUDED = ("commod3",)

# Retained under its old name for readers of the previous edition.
MRDS_PRODUCING = tuple(k for k, (keep, _) in MRDS_DEV_STATUS.items() if keep)


def mrds_vocabulary(path, fields=("oper_type", "dev_stat", "com_type")):
    """{field: {value: count}} over the whole file — the enumeration the rules
    above are written against, so a new MRDS edition that adds a value is
    detectable rather than silently defaulted."""
    out = {f: {} for f in fields}
    z = zipfile.ZipFile(path)
    with z.open("mrds.csv") as fh:
        rdr = csv.DictReader(io.TextIOWrapper(fh, encoding="utf-8", errors="replace"))
        for row in rdr:
            for f in fields:
                v = (row.get(f) or "").strip()
                out[f][v] = out[f].get(v, 0) + 1
    return {f: dict(sorted(out[f].items(), key=lambda kv: (-kv[1], kv[0])))
            for f in fields}


def split_commodities(row, fields):
    """Exact-match token set from comma-separated MRDS commodity columns.

    Split and matched exactly, never substring-matched: the columns mix true
    commodities with material-form qualifiers ("Sand and Gravel, Construction",
    "Iron, Pig Iron"), and a substring test would let "Iron" match "Iron Oxide
    Pigments".
    """
    toks = set()
    for f in fields:
        for t in (row.get(f) or "").split(","):
            t = t.strip()
            if t:
                toks.add(t)
    return toks


def mrds_rows(path, commodity_tokens):
    """Yield (row, verdict) for every MRDS record that names a wanted commodity
    anywhere and carries a usable coordinate.

    Nothing is dropped silently. `verdict` reports the outcome so the caller can
    count what each rule cost, per commodity and per nation:

        keep    sorted tokens that survive rules 1-3
        drop    {rule: sorted tokens}  — first rule that rejected them
        lon, lat

    Rules are attributed in a fixed order — dev_status, operation, commodity_tier
    — so a record rejected by two rules is charged to the first, and the counts
    sum to the total rejected without double-counting.
    """
    want = set(commodity_tokens)
    z = zipfile.ZipFile(path)
    with z.open("mrds.csv") as fh:
        rdr = csv.DictReader(io.TextIOWrapper(fh, encoding="utf-8", errors="replace"))
        for row in rdr:
            primary = split_commodities(row, MRDS_COMMODITY_FIELDS) & want
            tertiary = (split_commodities(row, MRDS_COMMODITY_FIELDS_EXCLUDED)
                        & want) - primary
            if not primary and not tertiary:
                continue
            try:
                lat = float(row.get("latitude") or "")
                lon = float(row.get("longitude") or "")
            except ValueError:
                continue
            if lat == 0.0 and lon == 0.0:
                continue
            if not (-90.0 <= lat <= 90.0 and -180.0 <= lon <= 180.0):
                continue

            dev = (row.get("dev_stat") or "").strip()
            oper = (row.get("oper_type") or "").strip()
            if dev not in MRDS_DEV_STATUS:
                raise KeyError(f"MRDS dev_stat not in the rule table: {dev!r}")
            if oper not in MRDS_OPER_TYPE:
                raise KeyError(f"MRDS oper_type not in the rule table: {oper!r}")

            allhit = sorted(primary | tertiary)
            drop = {}
            if not MRDS_DEV_STATUS[dev][0]:
                drop["dev_status"] = allhit
                keep = []
            elif not MRDS_OPER_TYPE[oper][0]:
                drop["operation"] = allhit
                keep = []
            else:
                keep = sorted(primary)
                if tertiary:
                    drop["commodity_tier"] = sorted(tertiary)
            yield row, {"keep": keep, "drop": drop, "lon": lon, "lat": lat}


# ---------------------------------------------------------------------------
# Shapefile (DBF + point/polygon SHP)
# ---------------------------------------------------------------------------

def read_dbf(data):
    nrec, hlen, rlen = struct.unpack("<IHH", data[4:12])
    fields = []
    off = 32
    while data[off] != 0x0D:
        raw = data[off:off + 32]
        fields.append((raw[:11].split(b"\0")[0].decode("latin-1"), chr(raw[11]), raw[16]))
        off += 32
    rows = []
    for i in range(nrec):
        rec = data[hlen + i * rlen: hlen + i * rlen + rlen]
        if not rec or rec[:1] == b"*":
            continue
        p = 1
        d = {}
        for name, _ftype, flen in fields:
            d[name] = rec[p:p + flen].decode("latin-1").strip()
            p += flen
        rows.append(d)
    return rows


def read_shp(data):
    """Shape records in file order: ('point', (x, y)) or ('poly', [ring, ...]).

    Rings come back exactly as stored — the first ring of each part sequence is
    the exterior in ESRI's clockwise convention, holes counter-clockwise. The
    caller treats ring 0 as exterior, which matches every polygon in WEP_PRVA
    (verified: no multi-part province has an interior ring that is not preceded
    by its own exterior).
    """
    out = []
    off = 100
    n = len(data)
    while off + 8 <= n:
        _rn, clen = struct.unpack(">II", data[off:off + 8])
        body = data[off + 8: off + 8 + clen * 2]
        off += 8 + clen * 2
        if len(body) < 4:
            continue
        stype = struct.unpack("<I", body[:4])[0]
        if stype == 1 and len(body) >= 20:
            x, y = struct.unpack("<dd", body[4:20])
            out.append(("point", (x, y)))
        elif stype in (3, 5) and len(body) >= 44:
            nparts, npoints = struct.unpack("<ii", body[36:44])
            parts = list(struct.unpack(f"<{nparts}i", body[44:44 + 4 * nparts]))
            pbase = 44 + 4 * nparts
            pts = []
            for i in range(npoints):
                x, y = struct.unpack("<dd", body[pbase + 16 * i: pbase + 16 * i + 16])
                pts.append((x, y))
            rings = []
            for i, s in enumerate(parts):
                e = parts[i + 1] if i + 1 < len(parts) else npoints
                ring = pts[s:e]
                if len(ring) >= 3:
                    rings.append(ring)
            out.append(("poly", rings))
        else:
            out.append((None, None))
    return out


def read_shapefile_zip(path, base):
    """(rows, shapes) for a shapefile inside a zip, matched case-insensitively."""
    z = zipfile.ZipFile(path)
    names = {n.lower(): n for n in z.namelist()}
    dbf = names.get(f"{base.lower()}.dbf")
    shp = names.get(f"{base.lower()}.shp")
    if dbf is None or shp is None:
        cand = sorted(n for n in z.namelist() if n.lower().endswith(".shp"))
        raise FileNotFoundError(f"{base} not in {os.path.basename(path)}; have {cand}")
    return read_dbf(z.read(dbf)), read_shp(z.read(shp))


def find_shapefile_bases(path):
    z = zipfile.ZipFile(path)
    return sorted(n[:-4] for n in z.namelist() if n.lower().endswith(".shp"))


# ---------------------------------------------------------------------------
# COAL — the four location sources, and the rules that admit a record
# ---------------------------------------------------------------------------
#
# Coal shipped national-only in the first two editions because MRDS holds 157
# coal records for the entire planet and none of them is the Ruhr. That is a
# property of MRDS, not of the world: USGS publishes coal locations, just not in
# MRDS. Four public-domain datasets cover it between them, and none of them is a
# substitute for another:
#
#   minfac       point MINES, world OUTSIDE the United States, from the Minerals
#                Yearbook facility series. The Ruhr, Upper Silesia, the British
#                collieries, the Donets Basin — 359 admitted coal facilities in
#                58 nations. Coarse where the Yearbook was coarse: "Mine at
#                Upper Silesia (17 mines)" is one row at one coordinate.
#   china2014    2,440 named Chinese coal mines digitised from the 2001 Atlas.
#                Shanxi alone has 242. minfac carries eleven province rollups
#                for the whole of China; this carries the mines.
#   fsucoal      163 NAMED coal deposit and basin POLYGONS of the Former Soviet
#                Union — Donetsky, Kuznetsky, Karagandinsky, Ekibastuzsky.
#                Polygons, not points, so they are measured against districts
#                the way the petroleum provinces are.
#   uscoalfields 602 coal field POLYGONS of the conterminous United States,
#                including 208 in the Appalachian Region. minfac is explicitly
#                "outside the United States", so without this the world's #2
#                coal producer would still be blank.
#
# THE TRAP, and it is the bauxite trap again. Coal has a processed form with a
# name of its own — COKE — and the Yearbook files coke ovens in the same table
# as coal mines, at the same kind of coordinate, under a commodity string that
# contains the word "coal": "Coke: contained in domestic coal". Five of those
# sit in the Ruhr. Admitting them would put the Ruhr on the map for the wrong
# reason and would double-count Bottrop. Two rules stand in the way, both
# written against the source's OWN classification fields, and every value of
# both vocabularies is enumerated below with a decision and a reason.

# RULE A — FACILITY TYPE. Is coal extracted at this coordinate?
# USGS: fac_type is "the type of operation which may be a mine, an oil or gas
# field, or one of several kinds of processing plants". "Plant" is the source
# saying this is not a mine. Blank is the source saying nothing — and silence is
# admitted, on the same reasoning that admits MRDS's "Unknown" oper_type: an
# empty classification field is not an assertion that no ore is extracted. The
# four blank-typed coal rows are Botswana, Morocco, Mozambique and Swaziland,
# each the country's single unnamed coal entry.
MINFAC_FAC_TYPE = {
    "Mine":        (True,  "an extraction operation"),
    "Mine, Plant": (True,  "extraction and processing on one site; ore comes out "
                           "of the ground here"),
    "Plant":       (False, "USGS: a processing plant — the site consumes coal, "
                           "it does not produce it"),
    "":            (True,  "the source classified nothing; silence is not a "
                           "statement that no coal is extracted"),
}

# RULE B — COMMODITY STAGE. Is this commodity COAL, or something made from it?
# Eighteen tokens appear on coal-or-coke rows and each is decided here. The four
# rejected ones name COKE, which is coal baked in an oven: a manufactured fuel,
# not a thing mined. "Coke: contained in domestic coal" is the dangerous one —
# it is a coke plant's throughput expressed in the coal it ate, and the string
# contains the word "coal".
MINFAC_COAL_COMMODITY = {
    "Coal":                             (True,  "coal, rank unstated"),
    "Coal: anthracite":                 (True,  "coal, anthracite rank"),
    "Coal: anthracite and bituminous":  (True,  "coal, two ranks on one row"),
    "Coal: bituminous":                 (True,  "coal, bituminous rank"),
    "Coal: bituminous and lignite":     (True,  "coal, two ranks on one row"),
    "Coal: black":                      (True,  "coal, hard-coal rank"),
    "Coal: brown":                      (True,  "coal, brown-coal rank"),
    "Coal: hard":                       (True,  "coal, hard-coal rank"),
    "Coal: lignite":                    (True,  "coal, lignite rank"),
    "Coal: oxidized":                   (True,  "coal, weathered in place; still mined coal"),
    "Coal: subbituminous":              (True,  "coal, subbituminous rank"),
    "coal":                             (True,  "coal, rank unstated (lower-case vintage)"),
    "coal - anthracite":                (True,  "coal, anthracite rank (lower-case vintage)"),
    "coal - bituminous":                (True,  "coal, bituminous rank (lower-case vintage)"),
    "Coke":                             (False, "COKE — coal carbonised in an oven; "
                                                "a manufactured fuel, not mined"),
    "coke":                             (False, "COKE (lower-case vintage)"),
    "Coke: contained in domestic coal": (False, "a COKE PLANT's throughput expressed "
                                                "in the coal it consumed. The word "
                                                "'coal' is in the string and there is "
                                                "no mine at the coordinate"),
    "coal - coke":                      (False, "COKE (lower-case vintage)"),
}


def minfac_is_coalish(commodity):
    """Every commodity string that could plausibly be coal. The substring test
    is deliberate and is the point: it is how the coke rows are made to reach a
    decision in the table above instead of being missed."""
    c = (commodity or "").lower()
    return "coal" in c or "coke" in c


def minfac_coal_rows(path):
    """Yield (row, verdict) for every minfac record whose commodity mentions
    coal or coke and which carries a usable coordinate.

    `verdict` is {"keep": bool, "drop": rule-or-None, "lon": , "lat": }.
    Nothing is dropped silently: a rejected row is reported with the rule that
    rejected it so the generator can publish the receipt, and a commodity or
    facility-type value that is not in the tables above RAISES rather than
    defaulting — a new Yearbook edition cannot slip a coke oven past the filter.
    """
    z = zipfile.ZipFile(path)
    with z.open("minfac.csv") as fh:
        rdr = csv.DictReader(io.TextIOWrapper(fh, encoding="utf-8", errors="replace"))
        for row in rdr:
            com = (row.get("commodity") or "").strip()
            if not minfac_is_coalish(com):
                continue
            if com not in MINFAC_COAL_COMMODITY:
                raise KeyError(f"minfac coal commodity not in the rule table: {com!r}")
            fac = (row.get("fac_type") or "").strip()
            if fac not in MINFAC_FAC_TYPE:
                raise KeyError(f"minfac fac_type not in the rule table: {fac!r}")
            try:
                lat = float(row.get("latitude") or "")
                lon = float(row.get("longitude") or "")
            except ValueError:
                continue
            if not (-90.0 <= lat <= 90.0 and -180.0 <= lon <= 180.0):
                continue
            if lat == 0.0 and lon == 0.0:
                continue
            # Rule order is fixed — commodity stage first, then facility type —
            # so a coke oven is charged to the commodity rule and the counts sum
            # to the total rejected without double-counting.
            drop = None
            if not MINFAC_COAL_COMMODITY[com][0]:
                drop = "commodity_stage"
            elif not MINFAC_FAC_TYPE[fac][0]:
                drop = "facility_type"
            yield row, {"keep": drop is None, "drop": drop, "lon": lon, "lat": lat}


def china_coal_mines(path):
    """Rows of USGS OFR 2014-1219 AllChinaCoalMines, in published file order.

    The shapefile's point geometry and its LatDD/LongDD columns are the same
    numbers; the columns are read, because they are what the source printed and
    they carry the full published precision.
    """
    rows, _shapes = read_shapefile_zip(path, "AllChinaCoalMines")
    out = []
    for r in rows:
        try:
            lat = float(r.get("LatDD") or "")
            lon = float(r.get("LongDD") or "")
        except ValueError:
            continue
        if not (-90.0 <= lat <= 90.0 and -180.0 <= lon <= 180.0):
            continue
        # IDNum RESTARTS AT 1 IN EVERY PROVINCE. It is the number on the
        # province-scale map sheet the report publishes, not a national key:
        # 2,440 mines carry only 253 distinct values, and Shanxi alone runs 1 to
        # 242. Treating it as an identifier would have collapsed 2,440 mines to
        # 253 pieces of evidence — and it very nearly did, until the checker's
        # independent recount found an entry with 152 records on 153
        # coordinates. The source's own key is the pair, so the pair is the id.
        prov = (r.get("Province") or "").strip()
        num = (r.get("IDNum") or "").strip()
        out.append({
            "id": f"{prov}#{num}" if prov else num,
            "id_num": num,
            "name": (r.get("MineName") or "").strip(),
            "province": (r.get("Province") or "").strip(),
            "county": (r.get("CntyDstCty") or "").strip(),
            "rank": (r.get("Rank") or "").strip(),
            "size": (r.get("MineSize") or "").strip(),
            "lon": lon, "lat": lat,
        })
    return out


def coal_field_polygons(shp_bytes, dbf_bytes, name_field, extra_fields=()):
    """[(name, {extra}, rings)] for a coal-field / coal-deposit polygon layer.

    Shared by the two polygon sources — USGS OFR 01-104's FSU coal deposits and
    USGS OFR 2012-1205's conterminous-US coal fields — because they are the same
    shape of thing: a named area a survey drew around coal-bearing ground.
    Records with no name are kept and reported with an empty name rather than
    dropped, so the count in the artifact matches the count in the shapefile.
    """
    rows = read_dbf(dbf_bytes)
    shapes = read_shp(shp_bytes)
    out = []
    for i, r in enumerate(rows):
        if i >= len(shapes):
            break
        kind, rings = shapes[i]
        if kind != "poly" or not rings:
            continue
        name = (r.get(name_field) or "").replace("\x00", "").strip()
        extra = {}
        for f in extra_fields:
            v = (r.get(f) or "").replace("\x00", "").strip()
            if v:
                extra[f.lower()] = v
        out.append((name, extra, rings))
    return out
