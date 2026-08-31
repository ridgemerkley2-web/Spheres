#!/usr/bin/env python3
"""
tools/resources/make_resources.py — transcribe 1990 resources onto districts.

Writes `spheres-web/data/district_resources.json`. Run from the repo root:

    python tools/resources/make_resources.py

WHAT THIS IS
------------
A transcription, not a model. Every deposit in the output exists because a
public-domain dataset put a named site at a coordinate that falls inside that
district's polygon. Nothing is interpolated, nothing is scattered, nothing is
scored. A district with no sourced deposit does not appear in the file.

THE ONE RULE THAT SHAPES THE WHOLE ARTIFACT
-------------------------------------------
`where` and `how much` come from different places and are never multiplied
together.

  * WHERE is a point or a polygon: MRDS and PP1802 deposit coordinates, WEP
    petroleum province outlines. These land on districts.
  * HOW MUCH is a national 1990 production figure: EIA, USGS DS896, USDA PSD.
    These land on NATIONS, in a separate top-level `national` block, and are
    never divided among a nation's districts.

The temptation is to spread a country's 1990 copper across its copper districts
by deposit count. That would be a fabrication with a citation stapled to it: as
the probe established, MRDS record density measures how hard the USGS looked,
not what is in the ground — Canada has 147 cobalt records to Zaire's 37, though
Zaire was more than half the world's cobalt in 1990. So districts carry
presence and evidence; nations carry tonnage; the join is left to the consumer,
who now has both halves and a written warning about what happens if they
multiply them.

The same rule governs the WEP provinces. A province polygon covers many
districts and carries a real known-oil figure, so each district records the
province it lies in AND that province's total, explicitly flagged `shared`. The
figure is the province's, repeated, not the district's share. Dividing it by
area would invent the one number nobody published.

Coal, wheat and rice are national-only by construction: MRDS holds 157 coal
records for the entire world, and no source places a wheat field. They appear in
`national` with an empty district footprint, and the artifact says so.

DETERMINISM
-----------
No RNG, no wall clock, no set iteration reaching the output. Districts and
commodities emit through sorted keys; deposits sort by source record id; the
JSON is written with `sort_keys=True`. Running twice produces byte-identical
files, which `check_resources.py` verifies.
"""

import hashlib
import json
import os
import sys
import zipfile

HERE = os.path.dirname(os.path.abspath(__file__))
if HERE not in sys.path:
    sys.path.insert(0, HERE)

import crosswalk as X                                   # noqa: E402
import sources as S                                     # noqa: E402
from geo import (                                       # noqa: E402
    CENTROID_KM,
    DATA,
    DISTRICTS_JSON,
    GAME,
    SLIVER_FRAC,
    AdminCentroids,
    DistrictIndex,
    NationCrosswalk,
    ProvinceIntersector,
    geom_area_km2,
    rings_to_geometry,
)

OUT = os.path.join(DATA, "district_resources.json")

# How many named sites to carry per district per commodity. The exact count `n`
# is always recorded; the sample exists so the file stays readable and small.
# Sites are chosen by sorted record id, so the sample is reproducible and the
# full list is recoverable by re-running the same filter against the source.
SAMPLE = 8

# Offshore rigs and coastal mines can fall just outside a generalised coastline.
# A point may attach to the nearest district centroid within this many degrees,
# and the attachment is recorded as `snapped` so it is never mistaken for
# containment. Beyond it the record is dropped and counted in `unplaced`.
SNAP_DEG = 0.75


# ---------------------------------------------------------------------------
# Commodity table, and the ore-versus-refined-metal audit
# ---------------------------------------------------------------------------
# `where` names the dataset that places the commodity on the map; `how_much`
# names the dataset that gives its 1990 national production. A commodity may
# have one, the other, or both — and the artifact states which, per commodity,
# so a consumer can see at a glance that coal has magnitude but no location and
# gold has location but (here) no magnitude.
#
# THE AUDIT (2026-08-31). The game's commodities are ORES — what comes out of
# the ground in a district. MRDS's 183-token commodity vocabulary is not
# uniformly at that stage: it names elements, ore minerals, industrial-material
# forms and refined products in one flat list, and it carries no `Bauxite` token
# at all. Every game commodity was checked against that vocabulary and the
# result is recorded here rather than assumed, because exactly one of the nine
# was wrong and it had shipped.
#
#   iron       token "Iron" — ore-stage. MRDS carries "Pig Iron" as a SEPARATE
#              token for the smelted form, so "Iron" at a mine is ore. The 43
#              producing records that also say "Pig Iron" are magnetite and
#              gossan mines naming what their ore was smelted into; none is a
#              plant by oper_type, so the qualifier is a form note, not a marker.
#   copper     token "Copper" — ore-stage. MRDS's ore-form qualifiers are
#              "Copper Oxide" and "Copper Sulfide"; refined copper has no token
#              of its own, and copper refineries are caught by oper_type.
#   gold       token "Gold" — ore-stage; gold is won as the element.
#   uranium    token "Uranium" — ore-stage.
#   phosphate  token "Phosphorus-Phosphates" — ore-stage (phosphate rock). The
#              refined products are fertiliser and "Sulfuric Acid", which is its
#              own token and whose sites are plants by oper_type.
#   bauxite    token "Aluminum" — *** REFINED METAL. THE DEFECT. ***  MRDS has
#              NO bauxite token; "Aluminum" is the only token an editor could
#              reach for, and they reached for it at bauxite mines AND at
#              aluminium smelters alike. Mapping it straight onto `bauxite` made
#              Norway — whose nine Aluminum records are nine smelters and not one
#              mine — a bauxite producer. Fixed by two gates: the operation must
#              extract ore (rule 2), and the record must not name a non-bauxite
#              aluminous ore mineral (rule 4, below).
#   cobalt, rare_earths, platinum_group — PP1802, a deposit catalogue with no
#              operation-type and no development-status field and therefore no
#              plants in it at all; CRITICAL_M names elements in deposits and
#              DEPOSIT_TY names deposit types. Ore-stage by construction. Rules
#              1-3 have nothing to bite on there, and the artifact says so per
#              commodity rather than implying the same filtering was applied.
#
# The general lesson, recorded because it is the trap: MRDS's refined-stage
# tokens ("Pig Iron", "Contained or Metal", "Smelter", "Refinery", "Ferrochrome",
# "Mill Concentrate", "Ultra Pure") are FORM QUALIFIERS attached to a commodity,
# not site classifications. Filtering on them would delete real magnetite mines.
# The field that classifies the site is `oper_type`, and that is what rule 2 uses.

# Rule 4 — ORE STAGE. Only bauxite needs it, and only because MRDS has no token
# for the ore. The gate is driven entirely by what the source itself writes in
# `ore` and `dep_type`; it never infers from a co-listed commodity. That
# restraint matters: Georgia's Eufaula, Andersonville and Irwinton bauxite
# districts and Australia's Weipa Andoom are all filed with "Kaolin" beside
# "Aluminum" because bauxite genuinely occurs with kaolin, and a co-commodity
# heuristic would have deleted four real bauxite districts to remove none.
BAUXITE_MINERALS = ("bauxite", "gibbsite", "boehmite", "diaspore")
BAUXITE_SETTINGS = ("laterite", "bauxite", "weathering residual", "residual", "karst")
NON_BAUXITE_ALUMINOUS = (
    "alunite", "natroalunite", "kaolin", "halloysite", "corundum", "andalusite",
    "kyanite", "sillimanite", "dumortierite", "montmorillonite", "dawsonite",
    "nahcolite", "cryolite", "nepheline", "staurolite", "emery", "pyrophyllite",
)


def bauxite_ore_gate(row):
    """(admit, reason) — is this extraction record evidence of BAUXITE?

    Admits on silence. A bauxite mine whose `ore` column is blank is still a
    bauxite mine, and 511 of the 772 surviving Aluminum records are blank there:
    demanding positive proof would erase China, India, Hungary, Kazakhstan,
    Greece and Suriname from the bauxite map, which is a fabricated absence and
    exactly as dishonest as Norway's fabricated presence. It rejects only where
    the source names a different aluminous ore mineral — alunite, kaolinite,
    corundum, dumortierite — because that is the source saying, in its own
    words, that what is mined here is not bauxite.
    """
    ore = (row.get("ore") or "").strip().lower()
    dep = (row.get("dep_type") or "").strip().lower()
    # Order matters, and this is the order of evidential strength. A named ore
    # MINERAL is the source stating what is mined; a deposit-type word like
    # "residual" only describes the setting, and settings are shared — alunite
    # weathers residually too. So both mineral tests run before the setting
    # test, and silence is admitted last.
    for k in BAUXITE_MINERALS:
        if k in ore or k in dep:
            return True, "bauxite ore mineral named"
    for k in NON_BAUXITE_ALUMINOUS:
        if k in ore:
            return False, f"ore mineral named is aluminous but not bauxite ({k})"
    for k in BAUXITE_SETTINGS:
        if k in dep:
            return True, "bauxite-forming deposit type named"
    return True, "no contrary ore mineral named"


MRDS_COMMODITIES = {
    # game key      MRDS commod token(s)     stage of that token   rule-4 gate
    "iron":        (("Iron",),                "ore",               None),
    "copper":      (("Copper",),              "ore",               None),
    "bauxite":     (("Aluminum",),            "refined metal name", bauxite_ore_gate),
    "gold":        (("Gold",),                "ore",               None),
    "uranium":     (("Uranium",),             "ore",               None),
    "phosphate":   (("Phosphorus-Phosphates",), "ore",             None),
}

PP1802_COMMODITIES = {
    "rare_earths":     ("Rare-Earth Elements",),
    "cobalt":          ("Cobalt",),
    "platinum_group":  ("Platinum-Group Elements",),
}


# ---------------------------------------------------------------------------
# Confidence
# ---------------------------------------------------------------------------
# Ruling 1, 2026-08-31: the map states its own confidence rather than being
# thinned until only the certain survives. A district resting on one MRDS record
# stays on the map and is rendered visibly weaker than the Copperbelt. That is
# only possible if the artifact carries the grade, so every district-commodity
# entry does.
#
# The two bases are different measurements and are labelled as such — banding a
# province by its record count would be meaningless, since one province polygon
# covering the whole of Al-Basrah is stronger evidence than four polygons
# clipping the corners of Al-Anbar.
#
# RULING 4, 2026-08-31 — THE COUNT WAS WRONG, THE DATA WAS NOT.
# The first edition banded a point-sourced entry on the number of distinct
# source RECORDS, and MRDS stacks records on administrative centroids. Six
# bauxite records reading `strong` in FRA_centre-val-de-loire were six Var mines
# — Mazaugues, Peygros, St Julien/Tourves, Blanquette/Combecave, La
# Rouquette/Montplaisir, Union des Bauxites — filed at 46.56346N 2.55405E, the
# centre of France, 400 km from the Var. Twenty-seven records in all sit on that
# one point. A band that a single fictitious coordinate can carry to `strong`
# does not mean what a reader takes it to mean.
#
# The correction is arithmetic and nothing else. NO COORDINATE IS MOVED,
# CORRECTED OR DROPPED: every record still ships, at the coordinate its source
# published, and `records` still reports how many there are. What changed is
# what the band COUNTS — DISTINCT COORDINATES, because six records at one point
# is one site's worth of evidence whatever the six are called. The thresholds
# are unchanged so the two editions are directly comparable: 6+ strong, 3-5
# moderate, 2 sparse, 1 single, now read off distinct coordinates.

CONFIDENCE_BANDS_SITES = ((6, "strong"), (3, "moderate"), (2, "sparse"), (1, "single"))
CONFIDENCE_BANDS_AREA = ((0.50, "strong"), (0.15, "moderate"), (0.02, "sparse"), (0.0, "single"))

# Clause (S) of the filing-centroid rule. Three DISTINCTLY NAMED sites on one
# published coordinate: three differently named mines cannot share a point to
# five decimal places, so the point is where the paperwork was filed and not
# where the mines are. Distinct names, not records, so a mine reported once per
# production year — Homestake 1944, 1945, 1946 — is a duplicate and not a stack.
CENTROID_NAMES = 3


def band_by_sites(n):
    """Band a point-sourced entry on its count of DISTINCT COORDINATES."""
    for lo, name in CONFIDENCE_BANDS_SITES:
        if n >= lo:
            return name
    return "single"


def band_by_records(n):
    """The superseded metric. Kept only so the generator can report the old
    distribution beside the new one; nothing in the artifact bands on it."""
    for lo, name in CONFIDENCE_BANDS_SITES:
        if n >= lo:
            return name
    return "single"


def band_by_area(frac):
    for lo, name in CONFIDENCE_BANDS_AREA:
        if frac >= lo:
            return name
    return "single"


# Coal, and only coal, can arrive on a district as TWO independent kinds of
# evidence at once: named mines from a point source and a named coal field from
# a polygon source. Donetsk carries both. Where that happens the entry keeps
# both measurements, states both bands, and takes the stronger — two independent
# sources agreeing is not weaker than either alone, and averaging them would
# invent a number neither source supports.
BAND_ORDER = ("single", "sparse", "moderate", "strong")


def stronger_band(a, b):
    return max((a, b), key=BAND_ORDER.index)

# The four coal location sources, in the order the artifact reports them. Two
# point sources and two polygon sources, complementary by construction: minfac
# is explicitly "outside the United States" and uscoalfields is exactly the
# United States; china2014 carries the 2,440 Chinese mines minfac reduces to
# eleven province rollups; fsucoal carries the named Soviet basins as outlines.
COAL_SOURCES = ("minfac", "china2014", "fsucoal", "uscoalfields")

# Where the US coal-field layer lives inside the published GIS archive.
US_COALFIELDS_SHP = "GIS/Updated Coal Fields/Coal_Fields.shp"
US_COALFIELDS_DBF = "GIS/Updated Coal Fields/Coal_Fields.dbf"

COMMODITY_DOC = {
    "oil": ("Crude oil", "wep_prva", "eia_intl"),
    "gas": ("Natural gas", "wep_prva", "eia_intl"),
    "coal": ("Coal", "+".join(COAL_SOURCES), "eia_intl"),
    "iron": ("Iron ore", "mrds", "ds896_iron"),
    "copper": ("Copper", "mrds", "ds896_copper"),
    "bauxite": ("Bauxite / aluminium ore", "mrds", "ds896_bauxite"),
    "gold": ("Gold", "mrds", None),
    "uranium": ("Uranium", "mrds", None),
    "phosphate": ("Phosphate rock", "mrds", None),
    "rare_earths": ("Rare-earth elements", "pp1802", None),
    "cobalt": ("Cobalt", "pp1802", None),
    "platinum_group": ("Platinum-group elements", "pp1802", None),
    "wheat": ("Wheat", None, "usda_psd"),
    "rice": ("Rice, milled", None, "usda_psd"),
}


# ---------------------------------------------------------------------------
# Provenance
# ---------------------------------------------------------------------------

def sha256(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


SOURCE_FILES = {
    "mrds": "mrds-csv.zip",
    "pp1802": "pp1802_shp.zip",
    "wep_prva": "wep_prva.zip",
    "eia_intl": "INTL.zip",
    "ds896_copper": "ds896-copper.xlsx",
    "ds896_iron": "ds896-iron-steel.xlsx",
    "ds896_bauxite": "ds896-aluminum.xlsx",
    "usda_psd": "psd_grains.zip",
    "minfac": "minfac-csv.zip",
    "china2014": "china-coal-mines.zip",
    "fsucoal_shp": "fsucoal_deposit.shp",
    "fsucoal_dbf": "fsucoal_deposit.dbf",
    "uscoalfields": "us-coalfields-gis.zip",
}

SOURCE_DOC = {
    "mrds": {
        "title": "USGS Mineral Resources Data System (MRDS)",
        "edition": "20160315",
        "url": "https://mrdata.usgs.gov/mrds/",
        "licence": "public domain (work of the U.S. Government)",
        "role": "deposit LOCATIONS for hard-rock metals and industrial minerals",
        "caveat": (
            "Record density measures USGS reporting effort, not endowment: 87.5% "
            "of records are United States, and systematic updates ceased in 2011. "
            "MRDS carries no tonnage, grade or reserve field of any kind. Use for "
            "WHERE, never for HOW MUCH, and never rank districts or countries by "
            "record count."
        ),
        "filter": (
            "Four rules, each stated against USGS's own field definition and "
            "each counted in meta.correction_2026_08_31.removed. "
            "(1) dev_stat in {Producer, Past Producer} — Occurrence, Prospect, "
            "Unknown and Plant are rejected. "
            "(2) oper_type NOT in {Processing Plant, Geothermal} — USGS defines "
            "Processing Plant as 'No ore extraction at the site, only a mill, "
            "smelter, etc.' "
            "(3) the commodity token must appear in commod1 (primary) or commod2 "
            "(secondary); commod3 is rejected because USGS defines it as "
            "commodities 'not economically recoverable', which is assay, not "
            "production. "
            "(4) ore stage: bauxite is admitted from the token 'Aluminum' only "
            "where the record does not name a different aluminous ore mineral. "
            "Tokens are split on comma and matched exactly, never by substring. "
            "lat/lon present, non-(0,0) and in range."
        ),
        "vocabulary_note": (
            "The controlling vocabularies are enumerated in full with counts in "
            "meta.mrds_vocabulary, and tools/resources/sources.py carries a "
            "decision and a reason for every value of each. An MRDS edition that "
            "introduces a new oper_type or dev_stat value raises rather than "
            "falling through to a default."
        ),
    },
    "pp1802": {
        "title": "USGS Professional Paper 1802 — global critical mineral deposits",
        "edition": "PP1802_CritMin_Shapefiles",
        "url": "https://www.sciencebase.gov/catalog/item/594d3c8ee4b062508e39b332",
        "licence": "public domain (work of the U.S. Government)",
        "role": "deposit LOCATIONS for cobalt, REE and platinum-group elements",
        "caveat": (
            "2,121 named points, 74 commodities, worldwide and far less "
            "US-skewed than MRDS — which is why cobalt, REE and PGE are taken "
            "from here. Still locations only; the file carries no tonnage."
        ),
        "filter": "CRITICAL_M split on ';' and matched exactly.",
    },
    "wep_prva": {
        "title": "USGS World Petroleum Assessment 2000 — geologic provinces",
        "edition": "WEP_PRVA",
        "url": "https://www.sciencebase.gov/catalog/file/get/60ad2fa1d34e4043c850ed98",
        "licence": "public domain; metadata states Access_Constraints none, Use_Constraints none",
        "role": "oil and gas province OUTLINES with known volumes",
        "caveat": (
            "142 province polygons, not fields. KWN_OIL (MMBO) and KWN_GAS (BCFG) "
            "are PROVINCE totals as assessed in 2000 — a resource assessment, not "
            "1990 production. `known` is carried verbatim on every district the "
            "province covers and flagged 'shared'. `apportioned` beside it is a "
            "DERIVED area-weighted split of that same figure and is labelled as "
            "derived everywhere it appears."
        ),
        "filter": (
            "Districts are discovered through an STRtree over the district "
            "geometries, then the province polygon's intersection with each "
            "district polygon is computed EXACTLY by GEOS polygon clipping "
            "(shapely) in WGS84 lon/lat, and the area of the result is the exact "
            "spherical integral over its straight lon/lat edges — no sampling, no "
            "lattice, no latitude discretisation. Each attachment records "
            "`intersection_sqkm`, `area_frac_district` (how much of the district "
            "the province covers) and `area_frac_province` (how much of the "
            "province lies in the district). An overlap under "
            f"{SLIVER_FRAC:g} of BOTH polygons is discarded as a boundary "
            "ribbon where the two sources' outlines of a shared border disagree; "
            "15 are discarded and they are listed in `meta.boundary_slivers`."
        ),
    },
    "eia_intl": {
        "title": "EIA International Energy Statistics (bulk INTL)",
        "edition": "api.eia.gov/bulk/INTL.zip",
        "url": "https://api.eia.gov/bulk/INTL.zip",
        "licence": "public domain; series carry \"copyright\": \"None\"",
        "role": "1990 NATIONAL production of crude oil, dry natural gas and coal",
        "caveat": (
            "Country-level only — no location. Keyed to 1990-era polities: the "
            "1990 datum sits on Former U.S.S.R., Former Czechoslovakia, Former "
            "Yugoslavia and the two Germanies, while successor states report no "
            "value for 1990, so filtering on 'has a 1990 number' selects the "
            "correct vintage by itself."
        ),
        "filter": "single-country series only (geography without '+'), annual, 1990 value numeric.",
    },
    "ds896_copper": {
        "title": "USGS DS 896 — Historical Statistics for Mineral Commodities: copper",
        "url": "https://www.usgs.gov/centers/national-minerals-information-center/historical-statistics-mineral-and-material-commodities",
        "licence": "public domain (work of the U.S. Government)",
        "role": "1990 NATIONAL mine production of copper (metric tons)",
        "caveat": "Country-level only. Sheet 'Mine', rows labelled 'Mine: Total'.",
        "filter": "header row located by literal 'Country' cell; 1990 column by name; 'XX' and '--' are not zero and are dropped.",
    },
    "ds896_iron": {
        "title": "USGS DS 896 — Historical Statistics for Mineral Commodities: iron and steel",
        "url": "https://www.usgs.gov/centers/national-minerals-information-center/historical-statistics-mineral-and-material-commodities",
        "licence": "public domain (work of the U.S. Government)",
        "role": "1990 NATIONAL iron ore production, gross weight (metric tons)",
        "caveat": "Country-level only. Sheet 'Iron ore, gross weight'.",
        "filter": "as ds896_copper.",
    },
    "ds896_bauxite": {
        "title": "USGS DS 896 — Historical Statistics for Mineral Commodities: aluminum",
        "url": "https://www.usgs.gov/centers/national-minerals-information-center/historical-statistics-mineral-and-material-commodities",
        "licence": "public domain (work of the U.S. Government)",
        "role": "1990 NATIONAL bauxite production (metric tons)",
        "caveat": "Country-level only. Sheet 'Bauxite'.",
        "filter": "as ds896_copper.",
    },
    "usda_psd": {
        "title": "USDA Foreign Agricultural Service — Production, Supply and Distribution",
        "url": "https://apps.fas.usda.gov/psdonline/downloads/psd_grains_pulses_csv.zip",
        "licence": "public domain (work of the U.S. Government)",
        "role": "1990 NATIONAL wheat and milled-rice production (1000 metric tons)",
        "caveat": (
            "Country-level only, and the one source that does NOT use 1990-era "
            "polities: PSD back-casts the USSR into successor republics for 1990. "
            "Those rows are kept as reported and flagged; they are not "
            "re-aggregated into USSR."
        ),
        "filter": "Market_Year 1990, Attribute_Description 'Production'.",
    },

    # --- the four coal location sources ------------------------------------
    # Added 2026-08-31 by the coal pass. Coal had shipped national-only in both
    # previous editions and the artifact said so; what it did not say is that
    # the hole was MRDS's, not the world's. All four are USGS, all four publish
    # Access_Constraints "none" and Use_Constraints "none" in their own FGDC
    # metadata, and all four were fetched over HTTPS and digested before use.
    "minfac": {
        "title": "USGS Mineral Operations Outside the United States (minfac)",
        "edition": "mrdata.usgs.gov/mineral-operations, minfac-csv.zip",
        "url": "https://mrdata.usgs.gov/mineral-operations/",
        "licence": "public domain; metadata states Access_Constraints none, Use_Constraints none",
        "role": "coal MINE locations, world outside the United States",
        "caveat": (
            "The facility tables of the USGS Minerals Yearbook, compiled 2003-2008 "
            "and NOT a 1990 census: `year` is 2003, 2004, 2005, 2006, 2007 or 2008 "
            "per record and ships on every cited site. It is coarse where the "
            "Yearbook was coarse — 'Mine at Upper Silesia (17 mines)' is ONE row at "
            "ONE coordinate, and China is eleven province rollups. Nothing in the "
            "United States: the title is literal, which is why a separate US source "
            "is carried. `capacity` is a facility capacity in the source's own "
            "units and is NOT transcribed into this artifact, because a 2007 "
            "capacity read as 1990 output is exactly the fabrication the "
            "apportionment note forbids."
        ),
        "filter": (
            "Two rules, each stated against the source's own classification field "
            "and each counted in meta.coal.removed. "
            "(A) fac_type must not be 'Plant' — USGS defines fac_type as the type "
            "of operation, and Plant is the source saying no extraction happens "
            "here. Blank is admitted: silence is not a denial. "
            "(B) the commodity must be coal and not COKE. Coke is coal baked in an "
            "oven, and the Yearbook files coke ovens beside coal mines under "
            "'Coke: contained in domestic coal' — a string with the word 'coal' in "
            "it and no mine at the coordinate. Five sit in the Ruhr. "
            "Every value of both vocabularies is enumerated with a decision and a "
            "reason in tools/resources/sources.py and shipped at "
            "meta.coal.minfac_vocabulary; an unknown value RAISES."
        ),
    },
    "china2014": {
        "title": "USGS OFR 2014-1219 — GIS data of coal mines and coal-bearing areas in China",
        "edition": "AllChinaCoalMines.shp",
        "url": "https://pubs.usgs.gov/of/2014/1219/",
        "licence": "public domain; metadata states Access_Constraints none, "
                   "Use_Constraints 'none; interpretations using this shapefile "
                   "must acknowledge USGS as source'",
        "role": "coal MINE locations, China (2,440 named mines)",
        "caveat": (
            "Digitised by USGS from the 'Atlas of solid fuels and nonmetal "
            "resources of China' (2001), so the mine set is as of 2001, not 1990. "
            "Carries mine name, province, county, rank and a three-value MineSize "
            "(Large / Medium / Small) which is a SIZE CLASS and not a tonnage; it "
            "ships as a band label and is never read as output. Datum is Beijing "
            "1954 (Krasovsky), not WGS84 — the offset is of order 100 m and is far "
            "under district resolution, and the published coordinates are "
            "transcribed unshifted."
        ),
        "filter": "LatDD/LongDD present and in range. No status field exists in "
                  "this source, so no status rule is applied and none is implied.",
    },
    "fsucoal": {
        "title": "USGS OFR 01-104 — Coal Quality and Resources of the Former Soviet Union",
        "edition": "fsucoal/views/shapes/deposit.shp + deposit.dbf",
        "url": "https://pubs.usgs.gov/of/2001/ofr-01-104/fsucoal/html/data1.htm",
        "licence": "public domain; the deposit layer's own FGDC metadata states "
                   "Access_Constraints none, Use_Constraints none",
        "role": "NAMED coal deposit and basin OUTLINES of the Former Soviet Union",
        "caveat": (
            "163 named polygons — Donetsky, Kuznetsky, Karagandinsky, "
            "Ekibastuzsky, L'vov-Volynsky, Pechora, Lensky, Irkutsky, Minusinsky. "
            "A coal-bearing AREA, not a mine and not a production figure: the "
            "polygon says coal is in the ground here, and nothing about how much "
            "came out in 1990. Datum is Pulkovo 1942 (Krasovsky); the offset is "
            "far under district resolution and coordinates are transcribed "
            "unshifted. THE REPORT'S OWN README carries an ESRI restriction on "
            "the coastline and country-boundary layers (cis, roads, rail, rivers, "
            "lakes) — those layers are NOT read here and are not redistributed. "
            "Only the coal layer, which is USGS/Vernadsky work with no such "
            "restriction, is used."
        ),
        "filter": "polygon records of deposit.shp with their DEPOSIT name and "
                  "DEP_AGE, measured against districts by exact GEOS clipping.",
    },
    "uscoalfields": {
        "title": "USGS OFR 2012-1205 — Coal Fields of the Conterminous United States",
        "edition": "GIS/Updated Coal Fields/Coal_Fields.shp",
        "url": "https://pubs.usgs.gov/of/2012/1205/",
        "licence": "public domain; metadata states Access_Constraints None and a "
                   "use statement that is a warranty disclaimer, not a restriction",
        "role": "coal field OUTLINES of the conterminous United States",
        "caveat": (
            "602 polygons in six coal provinces, 208 of them named 'Appalachian "
            "Region'. Compiled from the USGS National Coal Resource Assessment and "
            "published 2011-2012. A coal FIELD, not a mine: the map sheet says "
            "explicitly that it 'does not differentiate between potentially minable "
            "coal and uneconomic coal', so a field polygon is evidence that coal is "
            "in the ground, not that a district mined it in 1990. Conterminous "
            "United States only — no Alaska, and nothing outside the US, which is "
            "the exact complement of minfac."
        ),
        "filter": "polygon records with NAME, PROVINCE, RANK and AGE, measured "
                  "against districts by exact GEOS clipping.",
    },
}

# Sources examined during the probe and deliberately NOT used, recorded so the
# absence is a decision on the record rather than an oversight.
SOURCES_REJECTED = {
    "faostat_crops": {
        "title": "FAOSTAT Production_Crops_Livestock",
        "reason": (
            "Reachable and rich, but FAO bolts a commercial restriction onto its "
            "CC BY 4.0 terms: datasets 'shall not be used for or in conjunction "
            "with the promotion of a commercial enterprise and/or its "
            "product(s)'. USDA PSD covers wheat and rice for 1990 and is public "
            "domain, so nothing here needs FAO."
        ),
    },
    "usgs_myb_pdfs": {
        "title": "USGS Minerals Yearbook 1994 chapters (gold, uranium, phosphate, REE)",
        "reason": (
            "The only route to 1990 national production for the four commodities "
            "DS896 omits, but `pdftotext -layout` misaligns the columns: the gold "
            "chapter renders Australia's 244 t against Belize. Extracting it "
            "would have made Belize the world's second gold producer. Shipped "
            "without magnitude instead — those four commodities carry locations "
            "only, and the artifact says so."
        ),
    },
    "usgs_dds060": {
        "title": "USGS DDS-60 regional CD-ROM bundles",
        "reason": "~600 MB of report bundles for field geometry WEP_PRVA supplies in 1.5 MB.",
    },

    # --- probed by the coal pass, 2026-08-31 -------------------------------
    "gem_gcmt": {
        "title": "Global Energy Monitor — Global Coal Mine Tracker (August 2026 release)",
        "url": "https://globalenergymonitor.org/projects/global-coal-mine-tracker/download-data/",
        "reason": (
            "REACHABLE (HTTP 200) and genuinely open — CC BY 4.0, ~7,000 mines in "
            "70 countries — and rejected on two grounds. FIRST, the download is "
            "gated: there is no direct file URL, only a form served by "
            "api.globalenergymonitor.org that requires an email address, and this "
            "pipeline does not submit forms or hand over an address on the owner's "
            "behalf. SECOND, and decisive even if it were ungated, the tracker's "
            "own scope statement is 'mines abandoned or permanently closed since "
            "2015'. A 1990 map needs the pits that closed BEFORE 2015, and that is "
            "most of the Ruhr, almost every British colliery, and much of the "
            "Donbas. It is the wrong vintage for this artifact by construction. "
            "Worth buying or requesting for a modern-day map; not for January 1990."
        ),
    },
    "usgs_wocqi": {
        "title": "USGS OFR 2010-1196 — World Coal Quality Inventory, version 1",
        "url": "https://pubs.usgs.gov/of/2010/1196/",
        "reason": (
            "REACHABLE, public domain (Access_Constraints none), global, and "
            "carries decimal lat/lon with a location-accuracy estimate on every "
            "row — and still not used. It is a COAL QUALITY SAMPLE inventory, not "
            "a mine inventory: 1,580 chemical analyses collected 1995-2007, of "
            "which 1,538 have a coordinate. A sample is not a site — the United "
            "Kingdom's 84 rows collapse to 22 coordinates, Norway's 28 to one, and "
            "South Africa's 40 carry no coordinate at all. It has NO Germany, NO "
            "Poland and NO United States, so it does not reach the Ruhr, Silesia "
            "or Appalachia, which are the three holes that mattered. Its only "
            "published form is a 2003-vintage binary .xls (OLE2/BIFF8) that no "
            "standard-library module reads; using it would add a third-party "
            "dependency to buy coverage minfac already has. Its one real "
            "advantage is Turkey (143 coordinates), which minfac lacks entirely — "
            "recorded here so that gap is a known trade and not an oversight."
        ),
    },
    "msha_mines": {
        "title": "US Mine Safety and Health Administration — Mines Data Set",
        "reason": (
            "Not fetched. It would give mine-level United States coal with "
            "coordinates and an abandoned/active status, which is finer than the "
            "coal-field polygons used instead. It was passed over because OFR "
            "2012-1205 is a USGS product of the same kind as the three other coal "
            "sources and measures against districts through machinery this "
            "directory already has, whereas MSHA would be a fourth vocabulary and "
            "a fifth admission rule set for one country. Named as the obvious next "
            "improvement to United States coal resolution, not as a dead end."
        ),
    },
}


# ---------------------------------------------------------------------------
# Build
# ---------------------------------------------------------------------------

def district_owners():
    """{district id: [nation, ...]} — 375 ids carry a predecessor AND successor."""
    with open(DISTRICTS_JSON, encoding="utf-8") as f:
        nations = json.load(f)["nations"]
    owners = {}
    for nation in sorted(nations):
        for d in nations[nation]:
            owners.setdefault(d["id"], []).append(nation)
    for k in owners:
        owners[k] = sorted(set(owners[k]))
    return owners


def coord_key(lon, lat):
    """The published coordinate, formatted once and used everywhere.

    Five decimals is MRDS's own precision — every MRDS coordinate round-trips
    through this exactly, and the 56,325 distinct admitted coordinates stay
    56,325 — so the string is the source's number and not a rounding of it.
    """
    return "%.5f,%.5f" % (lat, lon)


def add_site(bucket, did, commodity, src, rec_id, name, band, lon, lat, extra=None):
    """One point record's worth of evidence, attached to a district.

    `srcs` is a SET because coal — alone among the commodities — is placed from
    more than one point dataset, and a district that holds both a minfac
    colliery and a china2014 mine is supported by two independent surveys, not
    by one of them twice. Everything else has exactly one source and the set of
    one collapses to the string it always was.

    Record ids are namespaced by their source for the same reason: minfac's
    `rec_id` and OFR 2014-1219's `IDNum` are both small integers and would
    collide in a bare set, which would silently under-count the evidence.

    `extra` is the source's own extra words about the site — the Yearbook's
    reporting `year`, its commodity string, the mine's rank — carried so a
    reader can see on the site itself that this is a 2007 record and not a 1990
    one. It never carries a tonnage.
    """
    slot = bucket.setdefault(did, {}).setdefault(commodity, {
        "src": src, "srcs": set(), "n": 0, "bands": {}, "sites": [],
        "ids": set(), "coords": {},
    })
    at = coord_key(lon, lat)
    slot["srcs"].add(src)
    slot["n"] += 1
    slot["ids"].add((src, str(rec_id)))
    slot["coords"][at] = slot["coords"].get(at, 0) + 1
    if band:
        slot["bands"][band] = slot["bands"].get(band, 0) + 1
    xj = json.dumps(extra, sort_keys=True) if extra else ""
    slot["sites"].append((str(rec_id), name or "", band or "", at, src, xj))


def coord_census(census, lon, lat, name):
    """One tally per PUBLISHED COORDINATE, across every district and commodity.

    The stack that matters spans commodities — 46.56346N 2.55405E carries
    bauxite, iron and uranium — so the census cannot be per entry. It is taken
    over every point record admitted to the artifact, from both point sources,
    and it is what the filing-centroid rule is evaluated against.
    """
    slot = census.setdefault(coord_key(lon, lat), {
        "lon": lon, "lat": lat, "records": 0, "names": set(),
    })
    slot["records"] += 1
    if name:
        slot["names"].add(name)


def find_filing_centroids(census, admin):
    """{coordinate: reason} for every published coordinate that is a filing
    centroid rather than a site. Two clauses, either sufficient, both measured:

      (S) three or more DISTINCTLY NAMED sites are filed on the one coordinate;
      (A) the coordinate is within CENTROID_KM of the centroid of a Natural
          Earth admin-0 or admin-1 polygon.

    Neither clause alone is enough. (S) misses a lone national rollup — one
    bauxite record named "Bauxite - Vietnam" sitting on Vietnam's centroid — and
    (A) misses the centre of France, because Natural Earth's France includes
    Guyane and Reunion and its mainland centroid is 10 km off the point MRDS
    used. Together they catch both, and the clause that caught each coordinate
    is recorded so the flag can be audited one point at a time.
    """
    flags = {}
    for at in sorted(census):
        c = census[at]
        why = []
        if len(c["names"]) >= CENTROID_NAMES:
            why.append("stacked_names")
        near = admin.nearest(c["lon"], c["lat"])
        admin_hit = None
        if near is not None and near[0] <= CENTROID_KM:
            why.append("admin_centre")
            admin_hit = {
                "km": round(near[0], 3),
                "level": near[2],
                "name": near[1],
                "which": near[3],
            }
        if not why:
            continue
        flag = {
            "distinct_names": len(c["names"]),
            "records": c["records"],
            "why": why,
        }
        if admin_hit:
            flag["admin"] = admin_hit
        flags[at] = flag
    return flags


def charge(book, rule, commodity, nation_list):
    """Book one removed (record, commodity) pair against the rule that removed
    it, per commodity and per game nation, so the correction pass can be audited
    in the same units the artifact is read in."""
    slot = book.setdefault(rule, {"total": 0, "by_commodity": {}, "by_nation": {}})
    slot["total"] += 1
    slot["by_commodity"][commodity] = slot["by_commodity"].get(commodity, 0) + 1
    for nation in nation_list or ("(unplaced)",):
        slot["by_nation"][nation] = slot["by_nation"].get(nation, 0) + 1


def national_add(block, commodity, nation, value, source, label):
    slot = block.setdefault(commodity, {}).setdefault(nation, {
        "value": 0.0, "source": source, "source_labels": [],
    })
    slot["value"] += value
    slot["source_labels"].append(label)


def main():
    report = {}
    index = DistrictIndex()
    owners = district_owners()
    assert len(index.polys) == 2610, f"expected 2610 districts, indexed {len(index.polys)}"

    districts = {}
    national = {}
    stats = {"unplaced": {}, "snapped": {}, "country_mismatch": {}}
    removed = {}      # rule -> counts; the correction pass's own receipt
    census = {}       # published coordinate -> records and distinct names on it

    # --- MRDS point deposits ------------------------------------------------
    # Four rules stand between an MRDS row and a district entry. Rules 1-3 live
    # in sources.py against USGS's own field definitions; rule 4 is the ore-stage
    # gate and lives in this file with the commodity crosswalk. Every rejection
    # is counted per commodity and per nation rather than dropped quietly —
    # `removed` is what the correction pass is audited on.
    tok_to_key = {}
    for key, (toks, _stage, _gate) in MRDS_COMMODITIES.items():
        for t in toks:
            tok_to_key[t] = key
    mrds_zip = os.path.join(DATA, SOURCE_FILES["mrds"])
    seen = {k: 0 for k in MRDS_COMMODITIES}
    for row, v in S.mrds_rows(mrds_zip, tok_to_key.keys()):
        lon, lat = v["lon"], v["lat"]
        keys = sorted({tok_to_key[t] for t in v["keep"] if t in tok_to_key})

        # Rejections are located too, so "removed 10 bauxite entries from Japan"
        # is a statement about the map and not about MRDS's country column.
        # Only the rules NEW in this pass are located: the development-status
        # rule is unchanged from the previous edition, and locating the 236,000
        # occurrences and prospects it rejects would cost minutes to report a
        # number that did not move.
        if v["drop"]:
            drop_did, _ = index.locate(lon, lat, snap_deg=SNAP_DEG)
            owner = owners.get(drop_did, []) if drop_did else []
            for rule, toks in sorted(v["drop"].items()):
                if rule == "dev_status":
                    continue
                for k in sorted({tok_to_key[t] for t in toks if t in tok_to_key}):
                    charge(removed, rule, k, owner)

        if not keys:
            continue

        # Rule 4 — ore stage. Applied per game commodity, because only bauxite
        # is sourced from a token that names the refined metal.
        gated = []
        for k in keys:
            gate = MRDS_COMMODITIES[k][2]
            if gate is None:
                gated.append(k)
                continue
            admit, _why = gate(row)
            if admit:
                gated.append(k)
            else:
                drop_did, _ = index.locate(lon, lat, snap_deg=SNAP_DEG)
                charge(removed, "ore_stage", k,
                       owners.get(drop_did, []) if drop_did else [])
        keys = gated
        if not keys:
            continue

        did, how = index.locate(lon, lat, snap_deg=SNAP_DEG)
        if did is None:
            for k in keys:
                stats["unplaced"][k] = stats["unplaced"].get(k, 0) + 1
            continue
        if how == "snapped":
            for k in keys:
                stats["snapped"][k] = stats["snapped"].get(k, 0) + 1
        dev = (row.get("dev_stat") or "").strip()
        name = (row.get("site_name") or "").strip()
        rid = (row.get("dep_id") or row.get("mrds_id") or "").strip()
        for k in keys:
            seen[k] += 1
            add_site(districts, did, k, "mrds", rid, name, dev, lon, lat)
        coord_census(census, lon, lat, name)
        # QA only: does the district's owner agree with MRDS's country field?
        ctry = (row.get("country") or "").strip()
        if ctry:
            stats["country_mismatch"].setdefault(ctry, {})
            own = owners.get(did, [])
            stats["country_mismatch"][ctry][",".join(own)] = \
                stats["country_mismatch"][ctry].get(",".join(own), 0) + 1
    report["mrds_placed"] = dict(seen)
    report["removed"] = removed

    # --- PP1802 point deposits ---------------------------------------------
    pp_zip = os.path.join(DATA, SOURCE_FILES["pp1802"])
    rows, shapes = S.read_shapefile_zip(pp_zip, "PP1802_CritMin_pts")
    want_pp = {}
    for key, toks in PP1802_COMMODITIES.items():
        for t in toks:
            want_pp[t] = key
    ppseen = {k: 0 for k in PP1802_COMMODITIES}
    order = sorted(range(len(rows)),
                   key=lambda i: ((rows[i].get("DEPOSIT_NA") or ""), i))
    for i in order:
        r = rows[i]
        kind, pt = shapes[i]
        if kind != "point" or pt is None:
            continue
        toks = [t.strip() for t in (r.get("CRITICAL_M") or "").split(";") if t.strip()]
        keys = sorted({want_pp[t] for t in toks if t in want_pp})
        if not keys:
            continue
        lon, lat = pt
        did, how = index.locate(lon, lat, snap_deg=SNAP_DEG)
        if did is None:
            for k in keys:
                stats["unplaced"][k] = stats["unplaced"].get(k, 0) + 1
            continue
        if how == "snapped":
            for k in keys:
                stats["snapped"][k] = stats["snapped"].get(k, 0) + 1
        name = (r.get("DEPOSIT_NA") or "").strip()
        dtype = (r.get("DEPOSIT_TY") or "").strip()
        for k in keys:
            ppseen[k] += 1
            add_site(districts, did, k, "pp1802", name, name, dtype, lon, lat)
        coord_census(census, lon, lat, name)
    report["pp1802_placed"] = dict(ppseen)

    # --- COAL, part 1: the two point sources --------------------------------
    #
    # THE HOLE THIS FILLS. Both previous editions shipped coal national-only and
    # said so honestly — but the honest note was about the wrong thing. It said
    # "no dataset places coal at district resolution", and what was true was
    # "MRDS does not". MRDS holds 157 coal records for the whole planet;
    # Nordrhein-Westfalen carried copper, gas, gold and oil and no coal, while
    # Germany's national 1990 figure was 434 million tonnes. For a game that
    # starts in January 1990 that is not a rounding error: Polish and Silesian
    # coal, the Donbas, Appalachia, Shanxi, the Ruhr and the wreckage of the
    # British miners' strike are all 1990 politics, and the map said none of it
    # existed. USGS publishes coal locations. It publishes them somewhere else.
    #
    # Two point sources go in here and two polygon sources go in below. They are
    # complementary by construction and none is a subset of another: minfac is
    # explicitly "outside the United States", uscoalfields is exactly the United
    # States, china2014 carries the 2,440 Chinese mines minfac reduces to eleven
    # province rollups, and fsucoal carries the named Soviet basins as outlines.
    coal_seen = {k: 0 for k in COAL_SOURCES}
    coal_removed = {}
    coal_status_vocab = {}

    minfac_zip = os.path.join(DATA, SOURCE_FILES["minfac"])
    for row, v in S.minfac_coal_rows(minfac_zip):
        lon, lat = v["lon"], v["lat"]
        if not v["keep"]:
            drop_did, _ = index.locate(lon, lat, snap_deg=SNAP_DEG)
            charge(coal_removed, v["drop"], "coal",
                   owners.get(drop_did, []) if drop_did else [])
            continue
        did, how = index.locate(lon, lat, snap_deg=SNAP_DEG)
        if did is None:
            stats["unplaced"]["coal"] = stats["unplaced"].get("coal", 0) + 1
            continue
        if how == "snapped":
            stats["snapped"]["coal"] = stats["snapped"].get("coal", 0) + 1
        # The Yearbook's own words for what this row is, kept verbatim as the
        # entry's band label the way MRDS's dev_stat is: `status` says whether
        # the operation was producing in `year`, and `year` is 2003-2008 and is
        # never 1990. Both travel with the site so a reader can see the vintage
        # on the site and not only in the source block.
        status = (row.get("status") or "").strip()
        coal_status_vocab[status] = coal_status_vocab.get(status, 0) + 1
        name = (row.get("fac_name") or "").strip()
        rid = (row.get("rec_id") or "").strip()
        coal_seen["minfac"] += 1
        add_site(districts, did, "coal", "minfac", rid, name, status, lon, lat,
                 extra={"year": (row.get("year") or "").strip(),
                        "commodity": (row.get("commodity") or "").strip(),
                        "where": (row.get("location") or "").strip()})
        coord_census(census, lon, lat, name)

    china_zip = os.path.join(DATA, SOURCE_FILES["china2014"])
    for m in S.china_coal_mines(china_zip):
        did, how = index.locate(m["lon"], m["lat"], snap_deg=SNAP_DEG)
        if did is None:
            stats["unplaced"]["coal"] = stats["unplaced"].get("coal", 0) + 1
            continue
        if how == "snapped":
            stats["snapped"]["coal"] = stats["snapped"].get("coal", 0) + 1
        coal_seen["china2014"] += 1
        # MineSize is a THREE-VALUE CLASS — Large, Medium, Small — and it is the
        # band label here for the same reason dev_stat is elsewhere: it is what
        # the source says about the site. It is not a tonnage and must never be
        # summed, ranked or converted into one.
        add_site(districts, did, "coal", "china2014", m["id"], m["name"],
                 m["size"], m["lon"], m["lat"],
                 extra={"rank": m["rank"], "where": m["county"] or m["province"]})
        coord_census(census, m["lon"], m["lat"], m["name"])
    report["coal_points_placed"] = {"minfac": coal_seen["minfac"],
                                    "china2014": coal_seen["china2014"]}

    # --- WEP petroleum provinces -------------------------------------------
    wep_zip = os.path.join(DATA, SOURCE_FILES["wep_prva"])
    prows, pshapes = S.read_shapefile_zip(wep_zip, "WEP_PRVA/WEP_PRVA")
    porder = sorted(range(len(prows)), key=lambda i: ((prows[i].get("NAME") or ""), i))
    prov_hits = {"oil": 0, "gas": 0}
    unplaced_provinces = []
    sentinel_provinces = []
    live_provinces = []
    for i in porder:
        r = prows[i]
        kind, rings = pshapes[i]
        if kind != "poly" or not rings:
            continue

        def num(field):
            """WEP writes -9999 for 'not assessed'. That is a sentinel, not a
            volume, and it must never be read as a number — a province that was
            not assessed is absent, not empty."""
            raw = (r.get(field) or "").strip()
            try:
                v = float(raw or 0.0)
            except ValueError:
                return None
            if v <= -9990.0:
                return None
            return v

        kwn_oil, kwn_gas = num("KWN_OIL"), num("KWN_GAS")
        if kwn_oil is None and kwn_gas is None:
            sentinel_provinces.append((r.get("NAME") or "").strip())
            continue
        kwn_oil = kwn_oil or 0.0
        kwn_gas = kwn_gas or 0.0
        if kwn_oil <= 0.0 and kwn_gas <= 0.0:
            continue
        entry_base = {
            "code": (r.get("CODE") or "").strip(),
            "name": (r.get("NAME") or "").strip(),
            "o_g": (r.get("O_G") or "").strip(),
        }
        live_provinces.append((entry_base, rings, kwn_oil, kwn_gas))

    # Measured intersection, province polygon against district polygon.
    #
    # RULING 2, 2026-08-31. The first edition asked a 0.25-degree lattice "does
    # this province touch this district" and recorded a yes as an entry
    # indistinguishable from any other, so the province landed on whichever
    # district was biggest: Iraq read Al-Anbar rather than Al-Basrah, Nigeria
    # Cross River rather than Delta. The fix is not a better sample. It is the
    # actual intersection: GEOS polygon clipping through shapely, discovery
    # through an STRtree, and an exact spherical area for the clipped geometry.
    # `geo.ProvinceIntersector` owns it and the method is documented there,
    # including the boundary-ribbon floor that keeps three digitising artifacts
    # from handing the United States a located oil district.
    intersector = ProvinceIntersector(index)
    prov_area = {}
    touched = {}          # code -> {did: (intersection_km2, dfrac, pfrac)}
    boundary_slivers = []
    for entry_base, rings, kwn_oil, kwn_gas in live_provinces:
        code = entry_base["code"]
        geom = rings_to_geometry(rings)
        if geom is None or geom.is_empty:
            continue
        prov_area[code] = geom_area_km2(geom)
        hits, dropped = intersector.measure(geom)
        if hits:
            touched[code] = hits
        for did, inter_km2, dfrac, pfrac in dropped:
            boundary_slivers.append({
                "province": entry_base["name"], "code": code, "district": did,
                "sqkm": round(inter_km2, 4),
                "frac_of_district": float("%.3e" % dfrac),
                "frac_of_province": float("%.3e" % pfrac),
            })
    boundary_slivers.sort(key=lambda s: (-s["sqkm"], s["code"], s["district"]))

    # How much of each province lies inside ANY district at all. This is the
    # measurement that says "offshore" without anyone having to assert it: the
    # North Sea Graben has 0.26% of itself on land in the roster, Vestfjord-
    # Helgeland 0.09%, the Santos Basin 0.35%. A district that clips the edge of
    # such a province is a coastline artefact, not the place the oil is — and
    # Norway's 1.72 mbd and the UK's 1.82 mbd in 1990 came entirely from the
    # water those polygons cover.
    #
    # OFFSHORE_FLOOR is a stated choice, not a discovered constant. The measured
    # distribution has a 3.3x gap between the four wholly-offshore provinces at
    # 0.0009-0.0035 (Vestfjord-Helgeland, the Lesser Antilles Deformed Belt, the
    # North Sea Graben, the Santos Basin) and the next province up, the Tobago
    # Trough at 0.0115, and the cut is placed in that gap. The full measured
    # list ships in the artifact's `provinces` block so the choice can be
    # re-argued against the numbers rather than taken on trust.
    OFFSHORE_FLOOR = 0.01
    located_frac = {}
    for code in touched:
        located_frac[code] = sum(pf for _a, _df, pf in touched[code].values())

    # THE APPORTIONMENT, and exactly what it is.
    #
    # Ruling 2 asks for the province to be apportioned to districts on the real
    # intersection, and `area_frac_province` — the share of the province polygon
    # this district holds — is that weight. `apportioned` is the province's
    # assessed volume times that weight.
    #
    # It is DERIVED and it is labelled derived in every place it appears. Two
    # things keep it honest:
    #
    #   * `known` stays beside it, verbatim, flagged `shared: true`. The
    #     transcribed source record is never overwritten by the estimate, and
    #     `check_resources.py` still holds every `known` against the WEP record
    #     byte-for-byte.
    #   * The weight is the share of the WHOLE province, not of the province's
    #     on-land part. A province 83% offshore apportions 17% of its volume to
    #     districts and leaves the rest EXPLICITLY UNAPPORTIONED, recorded per
    #     province as `unapportioned_offshore`. Normalising over the on-land
    #     part instead would shove all of Cantarell and all of Ekofisk onto the
    #     nearest coastal province, which is the fabrication ruling 3 forbids,
    #     wearing an equation.
    #
    # What it is NOT: a district production figure. Oil is not spread evenly
    # through a sedimentary basin, and 1990 NATIONAL production stays whole in
    # the `national` block and is never divided. This apportions a 2000-vintage
    # resource assessment by measured area, and says so.
    prov_doc = {}
    for entry_base, _rings, kwn_oil, kwn_gas in live_provinces:
        code = entry_base["code"]
        hits = touched.get(code, {})
        frac_in = located_frac.get(code, 0.0)
        offshore = frac_in < OFFSHORE_FLOOR
        prov_doc[code] = dict(
            entry_base,
            known_oil_mmbo=kwn_oil, known_gas_bcfg=kwn_gas,
            sqkm=round(prov_area.get(code, 0.0), 1),
            in_districts_frac=round(frac_in, 6),
            districts=len(hits),
            # Measured, not inferred: the nations whose districts this province
            # polygon actually reaches. It is what lets a reader see that the
            # Villahermosa Uplift's 19,534 unapportioned MMBO — the Bay of
            # Campeche, Cantarell — sits against Mexico and nobody else, without
            # anyone having to assert an EEZ this pipeline does not carry.
            nations=sorted({n for d in hits for n in owners.get(d, ())}),
            offshore_or_outside_roster=offshore,
            unapportioned_offshore={
                "oil_mmbo": round(kwn_oil * (1.0 - frac_in), 3),
                "gas_bcfg": round(kwn_gas * (1.0 - frac_in), 3),
                "note": ("the part of this province lying outside every district. "
                         "Left unapportioned on purpose: no source says which land "
                         "district owns an offshore field."),
            },
        )
        if not hits:
            # Wholly offshore, or outside every district polygon. The volume is
            # real and cited, so it is recorded here rather than discarded —
            # but it is NOT attached to the nearest coast, because which
            # district owns an offshore field is a claim no source makes.
            unplaced_provinces.append(dict(
                entry_base,
                known_oil_mmbo=kwn_oil, known_gas_bcfg=kwn_gas,
                reason="province polygon overlaps no district (offshore or outside the roster)",
            ))
            continue
        for did in sorted(hits):
            inter_km2, dfrac, pfrac = hits[did]
            for key, val, unit in (("oil", kwn_oil, "MMBO"), ("gas", kwn_gas, "BCFG")):
                if val <= 0.0:
                    continue
                slot = districts.setdefault(did, {}).setdefault(key, {
                    "src": "wep_prva", "province_level": True, "provinces": [],
                })
                slot["provinces"].append(dict(
                    entry_base, known=val, units=unit, shared=True,
                    apportioned=round(val * pfrac, 3),
                    apportioned_is_derived=True,
                    intersection_sqkm=round(inter_km2, 2),
                    area_frac_district=round(dfrac, 4),
                    area_frac_province=round(pfrac, 6),
                    offshore_or_outside_roster=offshore,
                ))
                prov_hits[key] += 1

    # Provinces WEP carries with no known volume, or with its -9999 "not
    # assessed" sentinel. They are skipped above — a province with no assessed
    # volume is not evidence of production — but skipping them silently is how
    # the Bay of Campeche disappears: `Campeche-Sigsbee Salt Basin` is in WEP
    # with KWN_OIL = 0, which is why no measurement can ever make Campeche
    # Mexico's leading oil district. The absence is put on the record instead.
    for i in porder:
        r = prows[i]
        kind, rings = pshapes[i]
        if kind != "poly" or not rings:
            continue
        code = (r.get("CODE") or "").strip()
        if code in prov_doc:
            continue
        raw_o = (r.get("KWN_OIL") or "").strip()
        raw_g = (r.get("KWN_GAS") or "").strip()
        try:
            sentinel = float(raw_o or 0.0) <= -9990.0 or float(raw_g or 0.0) <= -9990.0
        except ValueError:
            sentinel = False
        prov_doc[code] = {
            "code": code,
            "name": (r.get("NAME") or "").strip(),
            "o_g": (r.get("O_G") or "").strip(),
            "known_oil_mmbo": None if sentinel else 0.0,
            "known_gas_bcfg": None if sentinel else 0.0,
            "attached": False,
            "reason": ("WEP records -9999, its 'not assessed' sentinel, which is not a "
                       "volume and is never read as one"
                       if sentinel else
                       "WEP records a known volume of zero for this province; a province "
                       "with no assessed volume is not evidence of production"),
        }

    report["wep_province_attachments"] = prov_hits
    report["wep_unplaced_provinces"] = len(unplaced_provinces)
    report["wep_sentinel_provinces"] = sorted(sentinel_provinces)
    report["wep_districts_measured"] = len({d for h in touched.values() for d in h})
    report["wep_boundary_slivers_dropped"] = len(boundary_slivers)
    report["wep_offshore_provinces"] = sorted(
        c for c in prov_doc if prov_doc[c].get("offshore_or_outside_roster"))

    # --- COAL, part 2: the two polygon sources ------------------------------
    #
    # The Donets Basin and the Appalachian Region are not points. They are areas
    # a geological survey drew a line around, and the honest way to attach one to
    # a district is the way the petroleum provinces are attached: clip the two
    # polygons against each other with GEOS and measure the result exactly. The
    # same `ProvinceIntersector` does it, so the sliver floor, the spherical area
    # integral and the STRtree determinism are shared and not reimplemented.
    #
    # WHAT IS DELIBERATELY NOT DONE HERE. There is no apportionment. WEP
    # publishes a volume per province and the artifact splits it by area and
    # labels the split derived; OFR 01-104 and OFR 2012-1205 publish NO tonnage
    # at all, so there is nothing to split and nothing is invented to split. A
    # coal-field attachment says one thing — this much of this district lies
    # inside a coal field this survey named — and the number beside it is an
    # AREA, in square kilometres, never a quantity of coal.
    coal_field_layers = []
    fsu_polys = S.coal_field_polygons(
        open(os.path.join(DATA, SOURCE_FILES["fsucoal_shp"]), "rb").read(),
        open(os.path.join(DATA, SOURCE_FILES["fsucoal_dbf"]), "rb").read(),
        "DEPOSIT", ("DEP_AGE",))
    coal_field_layers.append(("fsucoal", fsu_polys))
    _usz = zipfile.ZipFile(os.path.join(DATA, SOURCE_FILES["uscoalfields"]))
    us_polys = S.coal_field_polygons(
        _usz.read(US_COALFIELDS_SHP), _usz.read(US_COALFIELDS_DBF),
        "NAME", ("PROVINCE", "RANK", "AGE"))
    coal_field_layers.append(("uscoalfields", us_polys))
    _usz.close()

    # A NAMED FIELD IS NOT A POLYGON, and conflating the two would repeat
    # ruling 4's mistake in a new place. OFR 2012-1205 draws the Appalachian
    # Region as 208 separate polygons, 102 of which clip Pennsylvania. Banding
    # on the largest single fragment would read Pennsylvania as `moderate` on
    # 26% coverage when the fragments together — they are a partition of one
    # mapped region and do not overlap — cover far more of the state. So the
    # attachment is aggregated PER NAMED FIELD: intersections are summed, the
    # polygon count is published beside the sum so the transcription is still
    # visible, and `area_frac_field` is measured against that name's whole
    # mapped area rather than one fragment of it.
    coal_field_slivers = []
    coal_field_unplaced = []
    coal_field_stats = {}
    agg = {}              # (did, src, name) -> accumulator
    for src, polys in coal_field_layers:
        # Total mapped area per NAME, over the whole layer — the denominator
        # `area_frac_field` needs if it is to mean "this share of the named
        # field lies in this district".
        name_area = {}
        geoms = {}
        for i, (name, _extra, rings) in enumerate(polys):
            g = rings_to_geometry(rings)
            if g is None or g.is_empty:
                continue
            a = geom_area_km2(g)
            if a <= 0.0:
                continue
            geoms[i] = (g, a)
            name_area[name] = name_area.get(name, 0.0) + a
        # Sorted by (name, index) so the emission order is a function of the
        # data and not of the file's record order — the same rule PP1802 and WEP
        # already follow.
        order = sorted(geoms, key=lambda i: (polys[i][0], i))
        placed_polys = 0
        for i in order:
            name, extra, _rings = polys[i]
            geom, farea = geoms[i]
            hits, dropped = intersector.measure(geom)
            for did, inter_km2, dfrac, pfrac in dropped:
                coal_field_slivers.append({
                    "src": src, "field": name, "district": did,
                    "sqkm": round(inter_km2, 4),
                    "frac_of_district": float("%.3e" % dfrac),
                    "frac_of_polygon": float("%.3e" % pfrac),
                })
            if not hits:
                # A named coal field polygon that reaches no district in the
                # roster. Kept and named rather than dropped, exactly as an
                # unplaced petroleum province is: the survey drew it, and the
                # roster simply has no polygon under it.
                coal_field_unplaced.append({
                    "src": src, "field": name, "sqkm": round(farea, 1),
                    "reason": "coal field polygon overlaps no district in the roster",
                    **extra,
                })
                continue
            placed_polys += 1
            for did in sorted(hits):
                inter_km2, _dfrac, _pfrac = hits[did]
                slot = agg.setdefault((did, src, name), {
                    "polygons": 0, "sqkm": 0.0, "extra": {},
                })
                slot["polygons"] += 1
                slot["sqkm"] += inter_km2
                for k, v in extra.items():
                    slot["extra"].setdefault(k, set()).add(v)
        coal_field_stats[src] = {
            "polygons": len(polys),
            "polygons_placed": placed_polys,
            "named_fields": len(name_area),
        }
        coal_field_stats[src]["_name_area"] = name_area

    for (did, src, name) in sorted(agg):
        slot = agg[(did, src, name)]
        darea = intersector.district_area(did)
        farea = coal_field_stats[src]["_name_area"].get(name, 0.0)
        entry_slot = districts.setdefault(did, {}).setdefault("coal", {
            "src": src, "srcs": set(), "n": 0, "bands": {}, "sites": [],
            "ids": set(), "coords": {},
        })
        entry_slot["srcs"].add(src)
        field = {
            "src": src,
            "name": name,
            "polygons": slot["polygons"],
            "sqkm_field_total": round(farea, 1),
            "intersection_sqkm": round(slot["sqkm"], 2),
            "area_frac_district": round(min(slot["sqkm"] / darea, 1.0), 4) if darea > 0 else 0.0,
            "area_frac_field": round(min(slot["sqkm"] / farea, 1.0), 6) if farea > 0 else 0.0,
            "note": ("a coal-bearing AREA the survey named, measured against "
                     "this district. No tonnage exists in this source and none "
                     "is derived from it."),
        }
        for k in sorted(slot["extra"]):
            field[k] = sorted(slot["extra"][k])
        entry_slot.setdefault("fields", []).append(field)
    for src in coal_field_stats:
        coal_field_stats[src].pop("_name_area", None)
        coal_seen[src] = sum(1 for (_d, s, _n) in agg if s == src)
    report["coal_field_layers"] = coal_field_stats
    report["coal_field_slivers_dropped"] = len(coal_field_slivers)
    report["coal_field_unplaced"] = len(coal_field_unplaced)
    coal_field_slivers.sort(key=lambda s: (-s["sqkm"], s["src"], s["field"], s["district"]))
    coal_field_unplaced.sort(key=lambda s: (s["src"], s["field"]))

    # --- national magnitudes ------------------------------------------------
    eia_cw = NationCrosswalk(X.EIA, X.IGNORE)
    eia = S.eia_year(os.path.join(DATA, SOURCE_FILES["eia_intl"]))
    units = {}
    for key in ("oil", "gas", "coal"):
        for country in sorted(eia[key]):
            value, unit, _geo = eia[key][country]
            nation = eia_cw.get(country)
            if nation is None:
                continue
            units[key] = unit
            national_add(national, key, nation, value, "eia_intl", country)

    ds_cw = NationCrosswalk(X.DS896, X.IGNORE)
    for key, fname, sheet, label, src in (
        ("copper", "ds896_copper", 0, "Mine: Total", "ds896_copper"),
        ("iron", "ds896_iron", 0, None, "ds896_iron"),
        ("bauxite", "ds896_bauxite", 1, None, "ds896_bauxite"),
    ):
        table = S.ds896_year(os.path.join(DATA, SOURCE_FILES[fname]), sheet, label=label)
        units[key] = "metric tons"
        for country in sorted(table):
            nation = ds_cw.get(country)
            if nation is None:
                continue
            national_add(national, key, nation, table[country], src, country)

    psd_cw = NationCrosswalk(X.PSD, X.IGNORE)
    psd = S.psd_year(os.path.join(DATA, SOURCE_FILES["usda_psd"]), ["Wheat", "Rice, Milled"])
    for key, com in (("wheat", "Wheat"), ("rice", "Rice, Milled")):
        units[key] = "1000 metric tons"
        for country in sorted(psd[com]):
            nation = psd_cw.get(country)
            if nation is None:
                continue
            national_add(national, key, nation, psd[com][country], "usda_psd", country)

    unmapped = {}
    for nm, cw in (("eia_intl", eia_cw), ("ds896", ds_cw), ("usda_psd", psd_cw)):
        if cw.unmapped:
            unmapped[nm] = dict(sorted(cw.unmapped.items()))
    report["unmapped_country_names"] = unmapped

    # --- finalise districts -------------------------------------------------
    # Every district-commodity entry carries a `confidence` block. Ruling 1
    # keeps the single-record districts on the map and renders them weaker
    # instead of deleting them, and the renderer can only do that if the grade
    # travels with the entry.
    admin = AdminCentroids()
    filing_centroids = find_filing_centroids(census, admin)
    report["coordinate_census"] = {
        "records": sum(c["records"] for c in census.values()),
        "distinct_coordinates": len(census),
        "shared_coordinates": sum(1 for c in census.values() if c["records"] > 1),
        "records_on_shared_coordinates": sum(
            c["records"] for c in census.values() if c["records"] > 1),
        "filing_centroids": len(filing_centroids),
        "filing_centroids_by_clause": {
            "admin_centre": sum(1 for f in filing_centroids.values()
                                if "admin_centre" in f["why"]),
            "both": sum(1 for f in filing_centroids.values() if len(f["why"]) == 2),
            "stacked_names": sum(1 for f in filing_centroids.values()
                                 if "stacked_names" in f["why"]),
        },
        "records_on_filing_centroids": sum(
            census[at]["records"] for at in filing_centroids),
    }

    out_districts = {}
    conf_hist = {}
    old_hist = {}
    reband = {}
    for did in sorted(districts):
        entry = {}
        for com in sorted(districts[did]):
            v = districts[did][com]
            if "sites" in v:
                sites = sorted(set(v["sites"]))[:SAMPLE]
                records = len(v["ids"])
                coords = sorted(v["coords"])
                flagged = [at for at in coords if at in filing_centroids]
                fields = sorted(v.get("fields", ()),
                                key=lambda f: (f["src"], f["name"]))
                src = "+".join(sorted(v["srcs"])) or v["src"]
                have_points = bool(coords)
                # A coal entry can rest on named mines, on a named coal field,
                # or on both. Each kind is banded on its own measurement and
                # both bands ship; the entry takes the stronger. Everything that
                # is not coal has points and nothing else, and reads exactly as
                # it did before this pass.
                pband = band_by_sites(len(coords)) if have_points else None
                fband = (band_by_area(max(f["area_frac_district"] for f in fields))
                         if fields else None)
                old = band_by_records(records) if have_points else None
                if have_points and fields:
                    band = stronger_band(pband, fband)
                    banded_on = "stronger_of_points_and_fields"
                    basis = "coal_mine_points+coal_field_overlap"
                elif fields:
                    band = fband
                    banded_on = "coal_field_area_frac"
                    basis = "coal_field_overlap"
                else:
                    band = pband
                    banded_on = "distinct_coordinates"
                    basis = ("mrds_records" if src == "mrds"
                             else "pp1802_deposits" if src == "pp1802"
                             else "coal_mine_points")
                conf = {"banded_on": banded_on, "basis": basis}
                if have_points:
                    conf.update({
                        "records": records,
                        "distinct_coordinates": len(coords),
                        "unflagged_coordinates": len(coords) - len(flagged),
                        "centroid_coordinates": len(flagged),
                        "centroid_records": sum(v["coords"][at] for at in flagged),
                        "centroid_stacked": bool(flagged),
                    })
                conf["band"] = band
                if have_points:
                    conf["superseded_band_on_records"] = old
                if fields:
                    conf["coal_fields"] = len(fields)
                    conf["max_area_frac_district"] = max(
                        f["area_frac_district"] for f in fields)
                    conf["band_on_fields"] = fband
                if have_points and fields:
                    conf["band_on_points"] = pband
                if flagged:
                    conf["centroids"] = [
                        dict(filing_centroids[at], at=at,
                             records_here=v["coords"][at])
                        for at in flagged[:SAMPLE]
                    ]
                site_rows = []
                for a, b, c, d, s, xj in sites:
                    row = {"id": a, "name": b, "band": c, "at": d,
                           "at_centroid": d in filing_centroids}
                    # Only coal is multi-source, so only coal names the source
                    # on the site; every other commodity's site is exactly the
                    # object the previous edition emitted.
                    if com == "coal":
                        row["src"] = s
                        if xj:
                            row.update(json.loads(xj))
                    site_rows.append(row)
                entry[com] = {"src": src}
                if have_points:
                    entry[com].update({
                        "n": v["n"],
                        "bands": dict(sorted(v["bands"].items())),
                        "sites": site_rows,
                    })
                if fields:
                    entry[com]["field_level"] = True
                    entry[com]["fields"] = fields
                entry[com]["confidence"] = conf
                key = (src, band)
                if old is not None:
                    old_hist[(src, old)] = old_hist.get((src, old), 0) + 1
                    if old != band:
                        reband[f"{old}->{band}"] = reband.get(f"{old}->{band}", 0) + 1
                else:
                    old_hist[key] = old_hist.get(key, 0) + 1
            else:
                provs = sorted(v["provinces"], key=lambda p: (p["name"], p["code"]))
                top = max(p["area_frac_district"] for p in provs)
                # Coverage by a province that is itself 99%+ offshore is a
                # coastline artefact. It stays on the map — it was measured —
                # but it is banded on the best ONSHORE province, so Nordland's
                # 0.2% clip of Vestfjord-Helgeland cannot read as Norway having
                # located oil. Where every province is offshore the entry is
                # `single`: something is there, and it is the weakest thing on
                # the map.
                onshore = [p["area_frac_district"] for p in provs
                           if not p.get("offshore_or_outside_roster")]
                band = band_by_area(max(onshore)) if onshore else "single"
                # The rolled-up apportionment. This is the number that ranks a
                # nation's districts, and the reason Al-Basrah now leads Iraq:
                # 0.898 of Al-Basrah is Mesopotamian Foredeep and the Foredeep
                # is the 292,442 MMBO province, while Al-Anbar's larger absolute
                # overlap is mostly with the 17,435 MMBO Widyan Basin. Area
                # alone cannot tell those apart; area times the province's own
                # assessed volume can.
                entry[com] = {
                    "src": v["src"],
                    "province_level": True,
                    "provinces": provs,
                    "apportionment": {
                        "basis": "area_frac_province",
                        "derived": True,
                        "total": round(sum(p["apportioned"] for p in provs), 3),
                        "units": provs[0]["units"],
                        "note": ("DERIVED, not transcribed: this province's assessed "
                                 "volume times the measured share of the province "
                                 "polygon lying in this district. Not a production "
                                 "figure and not published by any source. `known` "
                                 "beside it is the transcribed province total."),
                    },
                    "confidence": {
                        "basis": "province_overlap",
                        "provinces": len(provs),
                        "max_area_frac_district": top,
                        "onshore_provinces": len(onshore),
                        "band": band,
                    },
                }
                key = (v["src"], band)
                old_hist[key] = old_hist.get(key, 0) + 1
            conf_hist[key] = conf_hist.get(key, 0) + 1
        out_districts[did] = entry

    out_national = {}
    for com in sorted(national):
        out_national[com] = {}
        for nation in sorted(national[com]):
            s = national[com][nation]
            out_national[com][nation] = {
                "value": round(s["value"], 6),
                "units": units.get(com, ""),
                "source": s["source"],
                "source_labels": sorted(s["source_labels"]),
                "merged": len(s["source_labels"]) > 1,
            }

    # --- unlocated producers ------------------------------------------------
    # Ruling 3, 2026-08-31: a nation that produced a commodity in 1990 and whose
    # districts carry none of it is UNLOCATED, not absent. The United States was
    # the world's number two oil producer and has no located oil here at all,
    # because the World Petroleum Assessment 2000 assessed provinces OUTSIDE the
    # United States; Norway's is entirely offshore. The hole is named and sized
    # so a renderer can draw it as a hole. It is never filled with a guess.
    UNLOCATED_REASON = {
        "oil": ("no ONSHORE petroleum province reaches a district of this nation; "
                "see `basis` for which of the three ways that happened."),
        "gas": ("no ONSHORE petroleum province reaches a district of this nation; "
                "see `basis`, the oil note and the `provinces` block."),
        "coal": ("no coal mine and no named coal field of the four coal location "
                 "sources falls in a district of this nation. Those sources date "
                 "from 2001-2012, so a nation that mined coal in 1990 and had "
                 "stopped by then lands here — see meta.known_gaps.coal_vintage."),
        "wheat": "no location source exists at district resolution (see known_gaps).",
        "rice": "no location source exists at district resolution (see known_gaps).",
        "copper": "no admitted MRDS extraction record falls in a district of this nation.",
        "iron": "no admitted MRDS extraction record falls in a district of this nation.",
        "bauxite": "no admitted MRDS extraction record falls in a district of this nation.",
    }
    # "Located" is not the same test for the two kinds of evidence, and pretending
    # it is would be the error in a new place. A point source locates a deposit at
    # a coordinate: one MRDS record is thin evidence but it IS a location, so any
    # district entry counts. A province polygon locates a basin, not a field: a
    # nation whose only petroleum overlap is a coastal clip of a wholly offshore
    # province has no located petroleum, which is exactly Norway's and the United
    # Kingdom's 1990 position.
    located_nations = {}
    touched_nations = {}      # commodity -> nation -> True, onshore or not
    for did in out_districts:
        for com, v in out_districts[did].items():
            if v.get("province_level"):
                touched_nations.setdefault(com, set()).update(owners.get(did, ()))
                if not v["confidence"]["onshore_provinces"]:
                    continue
            located_nations.setdefault(com, set()).update(owners.get(did, ()))

    # A nation whose only overlaps were dropped as boundary ribbons has a third,
    # distinct reason to be unlocated, and it is the one that matters most here:
    # the United States. WEP's North American polygons are cut at the 49th
    # parallel and the Yukon-Alaska line and those cuts miss Natural Earth's
    # border by a few hundred metres, so exact clipping produced 0.14 km2 of the
    # Alberta Basin in Montana, 0.08 km2 of the Williston Basin in North Dakota
    # and 13.9 km2 of the Mackenzie Foldbelt in Alaska. Reading those as located
    # oil would give the world's number two producer three oil districts made
    # entirely of a digitising mismatch. They are dropped, and the drop is named
    # here rather than left to look like an absence of data.
    sliver_nations = {}
    for s in boundary_slivers:
        for nat in owners.get(s["district"], ()):
            sliver_nations.setdefault(nat, []).append(
                "%s in %s (%.2f km2)" % (s["province"], s["district"], s["sqkm"]))

    UNLOCATED_BASIS = {
        "no_province_assessed": (
            "no WEP 2000 petroleum province polygon reaches any district of this "
            "nation. The assessment covered provinces OUTSIDE the United States "
            "and covered only 142 priority provinces worldwide, so this is a gap "
            "in the source, not evidence that the nation has no petroleum."),
        "offshore_only": (
            "every petroleum province touching this nation lies 99%+ outside "
            "every district — a coastline clip of a wholly offshore polygon. "
            "Norway's 1990 production came out of the North Sea, not out of "
            "Nordland."),
        "boundary_sliver_only": (
            "the only overlaps measured were boundary ribbons where the two "
            "sources' outlines of a shared border disagree, and were dropped as "
            "digitising artifacts rather than read as located petroleum."),
    }

    unlocated = {}
    for com in sorted(out_national):
        rows_u = []
        for nation in sorted(out_national[com]):
            e = out_national[com][nation]
            if e["value"] <= 0.0:
                continue
            if nation in located_nations.get(com, ()):
                continue
            extra = {}
            if com in ("oil", "gas"):
                if nation in touched_nations.get(com, ()):
                    basis = "offshore_only"
                elif nation in sliver_nations:
                    basis = "boundary_sliver_only"
                    extra["dropped_boundary_slivers"] = sorted(sliver_nations[nation])
                else:
                    basis = "no_province_assessed"
                extra["basis"] = basis
                extra["basis_note"] = UNLOCATED_BASIS[basis]
            rows_u.append({
                **extra,
                "nation": nation,
                "value": e["value"],
                "units": e["units"],
                "source": e["source"],
                "reason": UNLOCATED_REASON.get(com, "no located evidence for this nation."),
            })
        if rows_u:
            unlocated[com] = sorted(rows_u, key=lambda r: (-r["value"], r["nation"]))

    commodities = {}
    for key in sorted(COMMODITY_DOC):
        label, where, howmuch = COMMODITY_DOC[key]
        n_d = sum(1 for d in out_districts if key in out_districts[d])
        doc = {
            "label": label,
            "location_source": where,
            "magnitude_source": howmuch,
            "districts_with_presence": n_d,
            "nations_with_1990_production": len(out_national.get(key, {})),
            # The count above is nations with a 1990 ROW, and for coal that is
            # 136 while only 59 of them mined anything: EIA carries a series for
            # every country and most report zero. A reader who sees "136
            # nations" over a coal map is being told the wrong number, so the
            # producing count ships beside it under its own name.
            "nations_producing_above_zero": sum(
                1 for e in out_national.get(key, {}).values() if e["value"] > 0.0),
            "nations_producing_but_unlocated": len(unlocated.get(key, ())),
            "placement": ("province polygon" if where == "wep_prva"
                          else "mine point and named coal-field polygon"
                          if key == "coal"
                          else "deposit point" if where else "none — national figure only"),
        }
        if key == "coal":
            doc["location_sources"] = list(COAL_SOURCES)
            doc["source_tokens"] = sorted(
                k for k, (keep, _r) in S.MINFAC_COAL_COMMODITY.items() if keep)
            doc["token_stage"] = (
                "mined coal. The four COKE tokens in the same column are "
                "REJECTED and listed in meta.coal.minfac_vocabulary: coke is "
                "coal carbonised in an oven, and the Yearbook files coke plants "
                "beside coal mines under a string containing the word 'coal'.")
            doc["ore_stage_gate"] = (
                "minfac_coal_rules: the facility must not be classified `Plant`, "
                "and the commodity must be coal and not coke. china2014, fsucoal "
                "and uscoalfields are coal-only datasets with no plants in them "
                "and nothing for the rules to act on.")
            doc["districts_from_points"] = sum(
                1 for d in out_districts if "coal" in out_districts[d]
                and out_districts[d]["coal"].get("sites"))
            doc["districts_from_fields"] = sum(
                1 for d in out_districts if "coal" in out_districts[d]
                and out_districts[d]["coal"].get("fields"))
            doc["vintage_warning"] = (
                "NONE of the four location sources is a 1990 census. minfac is "
                "the Minerals Yearbook facility tables of 2003-2008, china2014 "
                "digitises a 2001 atlas, fsucoal was published in 2001 and "
                "uscoalfields in 2012. They say WHERE coal is mined and where "
                "coal-bearing ground was mapped; the 1990 tonnage stays whole in "
                "the `national` block and is never divided among them. A mine "
                "that opened after 1990 is in this file, and a pit that closed "
                "before 2001 may not be — the British and Ruhr collieries "
                "especially. Read the per-site `year` before drawing a "
                "conclusion about January 1990.")
        if key in MRDS_COMMODITIES:
            toks, stage, gate = MRDS_COMMODITIES[key]
            doc["source_tokens"] = list(toks)
            doc["token_stage"] = stage
            doc["ore_stage_gate"] = (
                "none needed — the MRDS token already names the ore" if gate is None
                else "bauxite_ore_gate: MRDS has NO bauxite token, so the metal "
                     "name 'Aluminum' is admitted as bauxite only at an "
                     "extraction operation and only where the record does not "
                     "name a different aluminous ore mineral")
        elif key in PP1802_COMMODITIES:
            doc["source_tokens"] = list(PP1802_COMMODITIES[key])
            doc["token_stage"] = "ore"
            doc["ore_stage_gate"] = (
                "none needed — PP1802 is a deposit catalogue with no operation-type "
                "and no development-status field, so it contains no plants and the "
                "MRDS admission rules have nothing to act on")
        commodities[key] = doc

    # One source, one digest — except OFR 01-104, which publishes its coal
    # deposit layer as loose shapefile parts rather than an archive. Both parts
    # are staged exactly as published and both are digested, because a changed
    # .dbf with an unchanged .shp would silently rename every basin.
    MULTIPART = {"fsucoal": ("fsucoal_shp", "fsucoal_dbf")}
    sources = {}
    for key in sorted(SOURCE_DOC):
        doc = dict(SOURCE_DOC[key])
        parts = MULTIPART.get(key, (key,))
        files = []
        total = 0
        for p in parts:
            path = os.path.join(DATA, SOURCE_FILES[p])
            n = os.path.getsize(path)
            total += n
            files.append({"file": SOURCE_FILES[p], "bytes": n, "sha256": sha256(path)})
        if len(files) == 1:
            doc.update(file=files[0]["file"], bytes=files[0]["bytes"],
                       sha256=files[0]["sha256"])
        else:
            doc["files"] = files
            doc["bytes"] = total
        sources[key] = doc

    # The controlling vocabularies, enumerated in full with counts and carried
    # in the artifact beside the decision made for each value. A rule you cannot
    # audit against the vocabulary it filters is a rule nobody can check.
    counts = S.mrds_vocabulary(mrds_zip, ("oper_type", "dev_stat"))
    for field, table in (("oper_type", S.MRDS_OPER_TYPE),
                         ("dev_stat", S.MRDS_DEV_STATUS)):
        stray = sorted(set(counts[field]) - set(table))
        if stray:
            raise KeyError(f"MRDS {field} values with no decision: {stray}")
    mrds_vocab = {
        "note": (
            "Every value of every controlling MRDS vocabulary, with its count "
            "over all 304,632 records and the decision this transcription makes "
            "about it. Reasons quoted from the USGS field definitions in "
            "mrds.met. A value present in the data and absent from the decision "
            "table raises rather than defaulting."
        ),
        "oper_type": {
            k: {"records": counts["oper_type"].get(k, 0),
                "admitted": S.MRDS_OPER_TYPE[k][0],
                "reason": S.MRDS_OPER_TYPE[k][1]}
            for k in sorted(S.MRDS_OPER_TYPE)
        },
        "dev_stat": {
            k: {"records": counts["dev_stat"].get(k, 0),
                "admitted": S.MRDS_DEV_STATUS[k][0],
                "reason": S.MRDS_DEV_STATUS[k][1]}
            for k in sorted(S.MRDS_DEV_STATUS)
        },
        "commodity_fields": {
            "commod1": {"admitted": True,
                        "reason": "USGS: primary commodities, 'might be economically "
                                  "viable as the only commodity'"},
            "commod2": {"admitted": True,
                        "reason": "USGS: secondary commodities, 'can be economically "
                                  "recovered'"},
            "commod3": {"admitted": False,
                        "reason": "USGS: tertiary commodities, 'economically interesting "
                                  "but not economically recoverable' — assay, not output"},
        },
    }

    # --- the coal receipt ---------------------------------------------------
    # Everything the coal pass did, in the units the artifact is read in: what
    # each source contributed, what the two admission rules removed and from
    # where, the vocabularies those rules are written against, the named fields
    # that reached no district, and the coverage against the 1990 producers.
    coal_nations = sorted({n for d in out_districts if "coal" in out_districts[d]
                           for n in owners.get(d, ())})
    # A PRODUCER is a nation whose 1990 figure is above zero. EIA carries a coal
    # series for 136 nations and most of them report nothing: counting Malta and
    # the Maldives as unlocated coal producers would inflate the hole by eighty
    # nations that never mined a tonne. Same test the `unlocated_producers`
    # block uses, so the two numbers agree.
    coal_national = {n for n, e in out_national.get("coal", {}).items()
                     if e["value"] > 0.0}
    coal_meta = {
        "note": (
            "Coal shipped national-only in the first two editions on the "
            "strength of MRDS holding 157 coal records for the whole planet. "
            "MRDS is not where USGS publishes coal. Four public-domain USGS "
            "datasets now place it: mine points outside the United States "
            "(minfac), 2,440 named Chinese mines (china2014), the named coal "
            "basins of the Former Soviet Union as polygons (fsucoal), and the "
            "coal fields of the conterminous United States as polygons "
            "(uscoalfields). All four state Access_Constraints none."
        ),
        "sources": list(COAL_SOURCES),
        "placed": dict(coal_seen),
        "districts_with_coal": sum(1 for d in out_districts
                                   if "coal" in out_districts[d]),
        "nations_with_located_coal": coal_nations,
        "nations_producing_in_1990": sorted(coal_national),
        "producers_now_located": sorted(coal_national & set(coal_nations)),
        "producers_still_unlocated": sorted(coal_national - set(coal_nations)),
        "producer_definition": ("a nation whose transcribed 1990 coal figure is "
                                "above zero. EIA carries a coal series for "
                                f"{len(out_national.get('coal', {}))} nations and "
                                f"only {len(coal_national)} of them report any "
                                "production."),
        "minfac_vocabulary": {
            "note": (
                "Every commodity string on a coal-or-coke row and every facility "
                "type they carry, with the decision this transcription makes "
                "about each. A value present in the data and absent from these "
                "tables RAISES rather than defaulting, so a later Yearbook "
                "edition cannot slip a coke oven onto the map."
            ),
            "commodity": {
                k: {"admitted": v[0], "reason": v[1]}
                for k, v in sorted(S.MINFAC_COAL_COMMODITY.items())
            },
            "fac_type": {
                (k or "(blank)"): {"admitted": v[0], "reason": v[1]}
                for k, v in sorted(S.MINFAC_FAC_TYPE.items())
            },
        },
        "minfac_status_vocabulary": dict(sorted(coal_status_vocab.items())),
        "coordinate_census_effect": (
            "Coal's point records join the shared coordinate census, because the "
            "census exists to find filing centroids and coal is the most "
            "centroid-ridden source in the file — eight 'Mine at Upper Silesia' "
            "rows on 50.17N 18.83E would not otherwise be caught. Adding 2,798 "
            "coal records took the census from 60,321 records on 56,694 "
            "coordinates to 63,119 on 59,307, and the flagged coordinates from "
            "492 to 531. MEASURED BY ABLATION, 2026-08-31: re-running the "
            "generator with coal excluded from the census alone changes ZERO "
            "non-coal entries. All 39 new flags are on coordinates only coal "
            "occupies, so no previously shipped entry's band, flag or count "
            "moved because coal arrived."
        ),
        "removed": coal_removed,
        "field_layers": coal_field_stats,
        "field_slivers_dropped": {
            "rule": (
                f"the same boundary-ribbon floor the petroleum provinces use: an "
                f"overlap under {SLIVER_FRAC:g} of BOTH polygons is a disagreement "
                f"between two surveys' outlines, not ground."
            ),
            "count": len(coal_field_slivers),
            "worst": coal_field_slivers[:SAMPLE],
        },
        "unplaced_fields": {
            "note": ("named coal fields whose polygon reaches no district in the "
                     "roster. Kept and named rather than dropped, exactly as an "
                     "unplaced petroleum province is."),
            "count": len(coal_field_unplaced),
            "fields": coal_field_unplaced,
        },
        "what_this_cannot_say": [
            "How much coal a district produced in 1990. No coal source here "
            "carries a district tonnage and none is derived.",
            "That a district mined coal in 1990. Three of the four sources are "
            "2001-2012 and the two polygon sources map coal-bearing GROUND, not "
            "workings.",
            "That a district which mined coal in 1990 appears here. Pits closed "
            "between 1990 and 2001 are missing — most of the Ruhr's, most of "
            "Britain's, and an unknown share of the Donbas's.",
            "Anything by counting. `n` is a citation count, and one minfac row "
            "can be seventeen Silesian mines.",
        ],
    }

    artifact = {
        "meta": {
            "generator": "tools/resources/make_resources.py",
            "vintage": 1990,
            "districts_total": len(index.polys),
            "districts_with_any_resource": len(out_districts),
            "doctrine": (
                "Transcribed, not invented. Every district entry is a real "
                "deposit or petroleum province from a public-domain dataset, "
                "matched to the district whose polygon contains its coordinate. "
                "Districts carry PRESENCE and EVIDENCE only — no tonnage, no "
                "grade, no score. National 1990 production lives in the separate "
                "`national` block and is NEVER divided among districts, because "
                "no source publishes that split and inventing it would be a "
                "fabrication with a citation attached. Districts with nothing "
                "sourced are absent from `districts` by design: a sparse map is "
                "the honest map."
            ),
            "do_not": (
                "Do not rank districts or nations by `n`. MRDS record density "
                "measures USGS survey effort, not endowment — 87.5% of MRDS is "
                "the United States, Canada holds 147 cobalt records to Zaire's "
                "37, and Botswana has 3 diamond records while leading the world "
                "by value in 1990. `n` is a count of citations, nothing more."
            ),
            "province_figures": (
                "`provinces[].known` is the WHOLE province's assessed volume, "
                "repeated on every district the province covers and marked "
                "`shared: true`. It is not this district's share and must not be "
                "summed across districts."
            ),
            "province_geometry": (
                "`intersection_sqkm`, `area_frac_district` and "
                "`area_frac_province` are MEASURED AREAS: the exact GEOS "
                "intersection of the province polygon with the district polygon, "
                "the fraction of this district lying inside the province, and the "
                "fraction of the province lying inside this district. They are "
                "what tells Al-Basrah — which the Mesopotamian Foredeep covers "
                "whole — from Al-Anbar, which it clips."
            ),
            "apportionment": (
                "`provinces[].apportioned` and the entry's `apportionment.total` "
                "are DERIVED, NOT TRANSCRIBED: the province's assessed `known` "
                "volume times `area_frac_province`. Ruling 2 asks for the "
                "province to be apportioned on the real intersection and this is "
                "that number, but it is an area weighting and nothing more — oil "
                "is not distributed evenly through a sedimentary basin and no "
                "source publishes the district split. Two things are true of it "
                "and must stay true: `known` beside it is the untouched source "
                "record, and the weight is the share of the WHOLE province, so a "
                "province that is 83% offshore apportions 17% of its volume and "
                "leaves the rest in `provinces[].unapportioned_offshore` rather "
                "than pushing Cantarell and Ekofisk onto the nearest coast. "
                "NEVER confuse it with 1990 production: that is national, it is "
                "in the `national` block, and it is never divided."
            ),
            "confidence": (
                "Every district-commodity entry carries `confidence.band`, one "
                "of single / sparse / moderate / strong, so the map can state how "
                "much is behind each patch instead of drawing an n=1 record like "
                "the Copperbelt. Single-site districts are KEPT and rendered "
                "weaker; a graded map is more honest than a sparse one and more "
                "honest than a uniform one. The two bases are different "
                "measurements and are labelled. Point sources band on the count "
                "of DISTINCT PUBLISHED COORDINATES — `confidence."
                "distinct_coordinates`, 6+ strong, 3-5 moderate, 2 sparse, 1 "
                "single — and NOT on the record count, which is still reported "
                "beside it as `confidence.records` and is a different and larger "
                "number. Petroleum provinces band on how much of the district "
                "the province polygon covers (>=50% strong, >=15% moderate, "
                ">=2% sparse, above zero single), because one province covering "
                "a whole district is stronger evidence than four clipping its "
                "corners."
            ),
            "confidence_ruling_4": {
                "the_defect": (
                    "The first edition banded point sources on RECORDS, and MRDS "
                    "stacks records on administrative centroids. 46.56346N "
                    f"2.55405E — the centre of France — carries "
                    f"{census.get('46.56346,2.55405', {}).get('records', 0)} "
                    "admitted records under three commodities, and six of them "
                    "were the six Var bauxite mines that made "
                    "FRA_centre-val-de-loire read `strong` on evidence from one "
                    "fictitious point 400 km from the Var. A band a single "
                    "coordinate can carry to `strong` does not mean what a "
                    "reader takes it to mean."
                ),
                "the_correction": (
                    "Arithmetic, not editorial. NO COORDINATE WAS MOVED, "
                    "CORRECTED OR DROPPED — every record still ships at the "
                    "coordinate its source published, `n` still counts records "
                    "and `confidence.records` still counts distinct record ids. "
                    "The band now counts DISTINCT COORDINATES, because six "
                    "records at one point is one site's worth of evidence "
                    "whatever the six are called. Thresholds are unchanged so "
                    "the two editions compare directly, and each entry carries "
                    "`confidence.superseded_band_on_records` so any reader can "
                    "see what it used to say."
                ),
                "the_flag": (
                    "`confidence.centroid_stacked` is true when any coordinate "
                    "under the entry is a FILING CENTROID: a point where records "
                    "were filed rather than a place where a mine is. Two "
                    "clauses, either sufficient — (S) three or more distinctly "
                    "named sites share the one published coordinate, since three "
                    "differently named mines cannot sit on one point to five "
                    "decimals; or (A) the coordinate is within "
                    f"{CENTROID_KM:g} km of the centroid of a Natural Earth 10m "
                    "admin-0 or admin-1 polygon, whole geometry or largest part, "
                    "which is a location no mine has. `confidence.centroids` "
                    "names the matched unit and the clause for each, "
                    "`confidence.unflagged_coordinates` is the count with the "
                    "flagged points removed, and each sampled site carries "
                    "`at` and `at_centroid` so the flag can be checked one "
                    "record at a time."
                ),
                "what_the_flag_cannot_see": (
                    "Neither clause is a distance-to-the-real-mine test, because "
                    "no such test exists without a gazetteer this pipeline does "
                    "not have and would not be transcribing if it did. A single "
                    "record filed at a point that is neither shared nor an "
                    "administrative centre is invisible to the rule even when it "
                    "is plainly wrong. AU-NT is the worked example of both "
                    "halves: the `Gove Mine` record filed 400 km inland at "
                    "-19.41107,133.36423 IS caught, because that point is the "
                    "Northern Territory's centroid to 0.62 km and carries nine "
                    "differently named mines; the `Bauxite - Australia` record "
                    "at -25.99640,134.99894 is NOT, because it is a lone "
                    "national rollup on a point that is neither shared nor "
                    "within 2 km of any centroid, and only its name gives it "
                    "away. Nothing here can tell you that MA-12's `Bou Craa "
                    "Mine` is 700 km from Bou Craa either. "
                    "Clause (A) also carries a measured exposure to coincidence: "
                    "5,656 centroids times a 12.6 km2 disc is 0.05% of the "
                    "world's land, so of the "
                    f"{report['coordinate_census']['distinct_coordinates']:,} "
                    "distinct coordinates roughly 27 admin-centre matches are "
                    "expected by chance. The flag is evidence, not proof, and "
                    "the band is not conditioned on it."
                ),
            },
            "correction_2026_08_31": {
                "what": (
                    "A contamination pass. The previous edition mapped the MRDS "
                    "commodity token 'Aluminum' — the refined METAL — straight "
                    "onto the ore `bauxite`, and admitted records regardless of "
                    "whether the site extracted anything. Two consequences shipped: "
                    "Norway became a bauxite producer on the strength of the "
                    "Tyssedal and Mosjoen aluminium SMELTERS, and Steep Rock Iron "
                    "Mine (MRDS 10157857, Ontario) was filed as a bauxite, iron and "
                    "phosphate deposit because its commod1 and commod2 are empty "
                    "and its commod3 lists six assayed elements."
                ),
                "rules_added": {
                    "operation": (
                        "oper_type must be an extraction method. Processing Plant "
                        "('No ore extraction at the site, only a mill, smelter, "
                        "etc.') and Geothermal ('energy extracted from heat stored "
                        "in the earth') are rejected; the other nine values are "
                        "extraction methods and are kept. This is the rule that "
                        "removes Norway's smelters, Japan's ten aluminium plants "
                        "and Nigeria's Delta Steel plant at Warri."
                    ),
                    "commodity_tier": (
                        "presence must come from commod1 or commod2. USGS defines "
                        "commod3 as commodities 'economically interesting but not "
                        "economically recoverable', which is a statement about "
                        "assay and not about output. This is the rule that "
                        "un-files Steep Rock."
                    ),
                    "ore_stage": (
                        "a commodity token that names a refined metal rather than "
                        "an ore is admitted only through a per-commodity gate. "
                        "Exactly one of the nine commodities needed one: MRDS has "
                        "no 'Bauxite' token at all, so 'Aluminum' is admitted as "
                        "bauxite only at an extraction operation and only where "
                        "the record does not name a different aluminous ore "
                        "mineral (alunite, kaolinite, corundum, dumortierite). "
                        "See commodities[].token_stage for the full audit."
                    ),
                },
                "removed": removed,
                "not_done_and_why": (
                    "Records were not deleted for silence. A bauxite record whose "
                    "`ore` column is blank stays: 511 of the 772 surviving "
                    "Aluminum extraction records are blank there, and demanding "
                    "positive proof would have erased China, India, Hungary, "
                    "Kazakhstan, Greece and Suriname from the bauxite map. A "
                    "fabricated absence is exactly as dishonest as a fabricated "
                    "presence."
                ),
            },
            "placement": {
                "rule": (
                    "A deposit attaches to the district whose polygon contains "
                    "its coordinate. Points falling outside every district — "
                    "offshore platforms, coastal sites lost to coastline "
                    "generalisation — may attach to the nearest district "
                    f"centroid within {SNAP_DEG} degrees; beyond that they are "
                    "dropped, not guessed."
                ),
                "snapped_records": dict(sorted(stats["snapped"].items())),
                "dropped_records": dict(sorted(stats["unplaced"].items())),
                "note": (
                    "Snapped and dropped counts are given per commodity so the "
                    "cost of the rule is visible. Together they are well under "
                    "1% of placed records."
                ),
            },
            "known_gaps": {
                "united_states_oil_and_gas": (
                    "The USGS World Petroleum Assessment 2000 assessed provinces "
                    "OUTSIDE the United States — every North American polygon "
                    "stops at the 49th parallel — so no US district carries oil "
                    "or gas here, though the USA was the world's #2 producer of "
                    "both in 1990. The national figures are present; the "
                    "locations are not. This is the single largest hole in the "
                    "artifact. Exact polygon clipping does find three US "
                    "overlaps, and all three are false: the cuts at the 49th "
                    "parallel and the Yukon-Alaska line miss Natural Earth's "
                    "border by a few hundred metres, giving 0.14 km2 of the "
                    "Alberta Basin in Montana, 0.08 km2 of the Williston Basin "
                    "in North Dakota and 13.9 km2 of the Mackenzie Foldbelt in "
                    "Alaska. They are dropped as boundary ribbons and listed in "
                    "`meta.boundary_slivers`. Filling this hole from them would "
                    "have been a fabrication three hundred metres wide."
                ),
                "offshore_petroleum": (
                    "Offshore provinces are the second largest hole. Five overlap "
                    "no district at all and keep their volumes in "
                    "`unplaced_provinces`. Others clip a coastline: the North Sea "
                    "Graben's polygon reaches 8.4% of one north-eastern British "
                    "district and Vestfjord-Helgeland's reaches 0.2% of Nordland, "
                    "though 99.7% and 99.9% of those provinces respectively lie "
                    "outside every district. Those attachments are measured and "
                    "kept, flagged `offshore_or_outside_roster`, and banded "
                    "`single` — the weakest thing on the map — because Norway's "
                    "1.72 mbd and the UK's 1.82 mbd in 1990 came out of the water, "
                    "not out of Nordland. Both nations are listed in "
                    "`unlocated_producers` for oil and gas."
                ),
                "petroleum_provinces_with_no_known_volume": (
                    "WEP carries provinces with a known volume of zero and "
                    "provinces marked -9999, its 'not assessed' sentinel. Neither "
                    "is evidence of production, so neither is attached to a "
                    "district — but both are listed in the `provinces` block "
                    "rather than dropped, because the silence is load-bearing. "
                    "`Campeche-Sigsbee Salt Basin` is one of them, which is why "
                    "no measurement here can make Campeche Mexico's leading oil "
                    "district: Mexico's Bay of Campeche production is real, and "
                    "this source does not carry a volume for it."
                ),
                "wheat_and_rice": (
                    "No location source exists for these at district resolution: "
                    "no dataset places a wheat field. They are national-only, by "
                    "refusal rather than by omission."
                ),
                "coal_was_national_only_and_is_not_any_more": (
                    "SUPERSEDED 2026-08-31 by the coal pass, and kept here "
                    "because the note it replaces was wrong in an instructive "
                    "way. Two editions shipped 'no location source exists for "
                    "coal at district resolution' on the strength of MRDS holding "
                    "157 coal records for the whole planet. What was true was "
                    "that MRDS does not carry coal. USGS does — in the Minerals "
                    "Yearbook facility tables, in OFR 2014-1219, in OFR 01-104 "
                    "and in OFR 2012-1205 — and all four are public domain and "
                    "were reachable the whole time. The lesson is that 'our "
                    "source does not have it' and 'no source has it' are "
                    "different sentences, and the artifact printed the second "
                    "when it had only established the first."
                ),
                "coal_vintage": (
                    "The coal location sources date from 2001-2012 and none is a "
                    "1990 census. This cuts hardest exactly where 1990 politics "
                    "was hottest: the Ruhr appears as the nine collieries still "
                    "open in 2006, not the twenty-odd working in 1990, and "
                    "British coal appears as the twenty pits left in 2007, after "
                    "the closures the 1984-85 strike failed to stop. A district "
                    "that mined coal in 1990 and had stopped by 2001 can be "
                    "absent from this file. The gap is one-directional and it "
                    "UNDERSTATES 1990 coal; it does not invent any."
                ),
                "coal_national_rollups": (
                    "minfac is the Minerals Yearbook, and the Yearbook reports at "
                    "whatever resolution its correspondent had. 'Mine at Upper "
                    "Silesia (17 mines)' is one row at one coordinate; eight such "
                    "rows stack on 50.17N 18.83E. China is eleven province "
                    "rollups — 'Mine in Shanxi' at 37N 112E is a round-number "
                    "province centre. Those coordinates are transcribed as "
                    "published and never corrected, and the filing-centroid rule "
                    "catches the stacks: the Silesian point carries "
                    "`centroid_stacked`. Where a better source exists it is used "
                    "beside minfac rather than instead of it — china2014 puts 242 "
                    "named mines in Shanxi against minfac's one."
                ),
                "coal_fields_are_not_mines": (
                    "fsucoal and uscoalfields are POLYGONS of coal-bearing "
                    "ground, not mines. OFR 2012-1205 states outright that it "
                    "'does not differentiate between potentially minable coal and "
                    "uneconomic coal'. A district that overlaps the Appalachian "
                    "Region polygon has coal under it; whether it mined any in "
                    "1990 is a different claim and this artifact does not make "
                    "it. That is why a field-banded entry publishes an AREA "
                    "fraction and never a tonnage, and why the band on fields is "
                    "reported separately from the band on mines wherever a "
                    "district has both."
                ),
                "coal_has_no_district_magnitude": (
                    "As with every other commodity: the 1990 national coal "
                    "figures in `national` are never divided among districts. "
                    "minfac does carry a `capacity` column and it is deliberately "
                    "NOT transcribed, because it is a 2003-2008 facility capacity "
                    "and reading it as 1990 output — or summing it per district — "
                    "would be the exact fabrication the apportionment note "
                    "forbids. China's `MineSize` is a three-value class, Large / "
                    "Medium / Small, and ships as a band label for the same "
                    "reason: it is a word, not a number."
                ),
                "no_magnitude_commodities": (
                    "gold, uranium, phosphate, cobalt, rare_earths and "
                    "platinum_group carry locations but no 1990 production, "
                    "because DS896 does not cover them and the Minerals Yearbook "
                    "PDFs could not be parsed without column-misalignment errors "
                    "that would have invented figures."
                ),
                "mrds_administrative_centroids": (
                    "Some MRDS records carry an administrative centroid instead "
                    "of a site coordinate — six French bauxite mines of the Var "
                    "all sit at 46.563N 2.554E, which is the centre of France; "
                    "four of Weipa's six records sit at 22.570S 144.547E, the "
                    "centroid of Queensland, 1,300 km from Cape York; 'Bauxite - "
                    "Australia' sits at 25.996S 134.999E. The coordinate is what "
                    "the source published, so it is transcribed as published; it "
                    "is not corrected, because a corrected coordinate would be an "
                    "invented one. What is corrected is the COUNTING: ruling 4 "
                    "bands on distinct coordinates, so those six French records "
                    f"are one site's worth of evidence, and "
                    f"{len(filing_centroids)} such points are named and flagged "
                    "`centroid_stacked` on the entries that rest on them. That is "
                    "why a bauxite district still appears in Centre-Val-de-Loire "
                    "and why it is now drawn at the weakest band there is."
                ),
                "pp1802_has_no_status": (
                    "cobalt, rare_earths and platinum_group come from PP1802, "
                    "which has no operation-type and no development-status field. "
                    "The MRDS admission rules cannot be applied to them, and they "
                    "are not: those entries are deposits, which may never have "
                    "been mined. Their confidence basis is labelled "
                    "`pp1802_deposits` and not `mrds_records` for that reason."
                ),
            },
            "mrds_vocabulary": mrds_vocab,
            "coal": coal_meta,
            "boundary_slivers": {
                "rule": (
                    f"A province-district overlap under {SLIVER_FRAC:g} of BOTH "
                    "polygons is discarded. Exact clipping resolves the "
                    "disagreement between two sources' outlines of the same "
                    "border, and that ribbon is a digitising artifact, not "
                    "ground. Requiring BOTH fractions to be negligible is what "
                    "keeps the genuinely tiny districts: Port of Spain is 0.77 "
                    "km2 of the East Venezuela Basin, which is 5.3% of Port of "
                    "Spain."
                ),
                "stated_choice": (
                    f"{SLIVER_FRAC:g} is a stated choice, not a discovered "
                    "constant. Every dropped overlap is listed below with its "
                    "measurements so the cut can be re-argued against the "
                    "numbers rather than taken on trust."
                ),
                "dropped": boundary_slivers,
            },
        },
        "sources": sources,
        "sources_rejected": SOURCES_REJECTED,
        "commodities": commodities,
        "national": out_national,
        "provinces": prov_doc,
        "unlocated_producers": unlocated,
        "unplaced_provinces": sorted(unplaced_provinces,
                                     key=lambda p: (p["name"], p["code"])),
        "districts": out_districts,
    }

    with open(OUT, "w", encoding="utf-8") as f:
        json.dump(artifact, f, indent=1, sort_keys=True, ensure_ascii=False)
        f.write("\n")

    report["districts_with_any"] = len(out_districts)
    report["out_bytes"] = os.path.getsize(OUT)
    report["stats"] = stats
    report["confidence"] = dict(sorted(
        ((f"{src}:{band}", n) for (src, band), n in conf_hist.items())))
    report["confidence_superseded"] = dict(sorted(
        ((f"{src}:{band}", n) for (src, band), n in old_hist.items())))
    report["reband"] = dict(sorted(reband.items()))

    # The headline. How many entries rest on ONE distinct site, and how many of
    # those single sites are themselves a filing centroid rather than a mine.
    # Point entries are those with a coordinate count to report. A coal entry
    # resting only on a coal-field polygon has none, and is counted with the
    # province-level entries rather than diluting a statistic about sites.
    point = [v for e in out_districts.values() for v in e.values()
             if "distinct_coordinates" in v["confidence"]]
    one_site = [v for v in point if v["confidence"]["distinct_coordinates"] == 1]
    every = [v for e in out_districts.values() for v in e.values()]
    report["single_site"] = {
        "entries_total": len(every),
        "point_entries": len(point),
        "on_one_distinct_coordinate": len(one_site),
        "share_of_point_entries": round(100.0 * len(one_site) / len(point), 2),
        "share_of_all_entries": round(100.0 * len(one_site) / len(every), 2),
        "and_that_one_is_a_filing_centroid": sum(
            1 for v in one_site if v["confidence"]["centroid_stacked"]),
    }
    report["centroid_flag"] = {
        "point_entries": len(point),
        "flagged_entries": sum(1 for v in point if v["confidence"]["centroid_stacked"]),
        "entries_wholly_on_centroids": sum(
            1 for v in point if v["confidence"]["unflagged_coordinates"] == 0),
        "records_sitting_on_a_centroid": sum(
            v["confidence"]["centroid_records"] for v in point),
        "records_in_flagged_entries": sum(
            v["n"] for v in point if v["confidence"]["centroid_stacked"]),
    }
    report["unlocated_producers"] = {k: len(v) for k, v in sorted(unlocated.items())}
    return report, artifact


if __name__ == "__main__":
    rep, art = main()
    print("wrote", OUT, rep["out_bytes"], "bytes")
    print("districts with any resource:", rep["districts_with_any"], "/ 2610")
    print("MRDS placed:", rep["mrds_placed"])
    print("PP1802 placed:", rep["pp1802_placed"])
    print("WEP attachments:", rep["wep_province_attachments"])
    print("coal points placed:", rep["coal_points_placed"])
    print("coal field layers:", rep["coal_field_layers"],
          "slivers", rep["coal_field_slivers_dropped"],
          "unplaced fields", rep["coal_field_unplaced"])
    print("unplaced:", rep["stats"]["unplaced"])
    print("snapped:", rep["stats"]["snapped"])
    print("confidence:", rep["confidence"])
    print("confidence (superseded, on records):", rep["confidence_superseded"])
    print("rebanded:", rep["reband"])
    print("coordinate census:", rep["coordinate_census"])
    print("centroid flag:", rep["centroid_flag"])
    print("single-site:", rep["single_site"])
    print("unlocated producers:", rep["unlocated_producers"])
    for rule in sorted(rep["removed"]):
        r = rep["removed"][rule]
        print(f"removed by {rule}: {r['total']}  {dict(sorted(r['by_commodity'].items()))}")
    if rep["unmapped_country_names"]:
        print("UNMAPPED COUNTRY NAMES:", rep["unmapped_country_names"])
