#!/usr/bin/env python3
"""Freeze source-tagged opening population inputs. Standard library only.

    python tools/population/fetch_social_baseline.py --fetch
    python tools/population/fetch_social_baseline.py --check

Normal rebuilds are offline and byte-deterministic. --fetch downloads missing
public WDI responses; --fetch --refresh deliberately updates the source snapshot.
Missing observations stay null. No national GDP or historical class shares are
inferred. See docs/POPULATION_DATA.md for denominators and territorial caveats.
"""
from __future__ import annotations

import argparse
from concurrent.futures import ThreadPoolExecutor
import gzip
import hashlib
import json
import math
from pathlib import Path
import re
import time
from urllib.request import Request, urlopen


ROOT = Path(__file__).resolve().parents[2]
CACHE = Path(__file__).resolve().parent / "social_source_cache"
OUT = ROOT / "spheres-sim/data/population_1990.json"
YEAR = 1990
WINDOW = (1988, 1992)
SOURCE = "World Bank World Development Indicators"
API = "https://api.worldbank.org/v2"

# Field, WDI series, denominator, source agency, admissible upper bound.
FIELDS = [
    ("population_age_0_14", "SP.POP.0014.TO.ZS", "total population", "UN Population Division", 100),
    ("population_age_15_64", "SP.POP.1564.TO.ZS", "total population", "UN Population Division", 100),
    ("population_age_65_plus", "SP.POP.65UP.TO.ZS", "total population", "UN Population Division", 100),
    ("labor_force_participation_15_64", "SL.TLF.ACTI.ZS", "population ages 15-64", "ILO modeled estimates", 100),
    ("labor_force_participation_15_plus", "SL.TLF.CACT.ZS", "population ages 15+", "ILO modeled estimates", 100),
    ("employment_population_ratio_15_plus", "SL.EMP.TOTL.SP.ZS", "population ages 15+", "ILO modeled estimates", 100),
    ("unemployment", "SL.UEM.TOTL.ZS", "total labor force", "ILO modeled estimates", 100),
    ("literacy", "SE.ADT.LITR.ZS", "population ages 15+", "UNESCO Institute for Statistics", 100),
    ("primary_enrollment", "SE.PRM.ENRR", "official primary school age population; all enrolled ages in numerator", "UNESCO Institute for Statistics", None),
    ("secondary_enrollment", "SE.SEC.ENRR", "official secondary school age population; all enrolled ages in numerator", "UNESCO Institute for Statistics", None),
    ("tertiary_enrollment", "SE.TER.ENRR", "five-year age group following secondary graduation; all enrolled ages in numerator", "UNESCO Institute for Statistics", None),
    ("education_attainment_secondary_25_plus", "SE.SEC.CUAT.UP.ZS", "population ages 25+; at least completed upper secondary, cumulative", "UNESCO Institute for Statistics", 100),
    ("education_attainment_tertiary_25_plus", "SE.TER.CUAT.ST.ZS", "population ages 25+; at least completed short-cycle tertiary, cumulative", "UNESCO Institute for Statistics", 100),
]
EXTRA = ["SP.POP.TOTL", "SL.UEM.TOTL.NE.ZS"]
AGE_FIELDS = {field for field, *_ in FIELDS if field.startswith("population_age_")}

# Territorial reconstructions are explicit aggregates, never a modern successor
# used as if it were the whole historical country. Only age/population series are
# combined; labor and education need different denominator weights.
COMPOSITES = {
    "USSR": ["ARM", "AZE", "BLR", "EST", "GEO", "KAZ", "KGZ", "LTU", "LVA", "MDA", "RUS", "TJK", "TKM", "UKR", "UZB"],
    "Yugoslavia": ["BIH", "HRV", "MKD", "MNE", "SRB", "SVN", "XKX"],
    "Czechoslovakia": ["CZE", "SVK"],
    "Sudan": ["SDN", "SSD"],
    "Ethiopia": ["ETH", "ERI"],
}
TERRITORY_NOTES = {
    "USSR": "Reconstructed from all 15 successor republic territories using their 1990 population and age shares. This is a territorial reconstruction, not a published USSR observation.",
    "Yugoslavia": "Reconstructed from six successor republic territories plus Kosovo (XKX), which WDI reports separately from Serbia. This is a territorial reconstruction, not a published Yugoslavia observation.",
    "Czechoslovakia": "Reconstructed from Czechia and Slovakia using 1990 population and age shares. This is a territorial reconstruction, not a published Czechoslovakia observation.",
    "Sudan": "1990 Sudan included present-day South Sudan. Age shares use SDN + SSD; education and labor remain null because modern Sudan is not the whole opening country.",
    "Ethiopia": "1990 Ethiopia included present-day Eritrea. Age shares use ETH + ERI; education and labor remain null because modern Ethiopia is not the whole opening country.",
    "Taiwan": "TWN is the geographic identifier, but these WDI queries provide no observations for Taiwan. Missing values remain null; China is never used as a proxy.",
    "Germany": "WDI Germany is the unified-territory series, consistent with the game's single Germany roster entry; it is not a West Germany-only observation.",
    "Yemen": "WDI Yemen is the unified-territory series, consistent with the game's single Yemen roster entry.",
}


def encoded(value: object) -> bytes:
    return (json.dumps(value, ensure_ascii=False, indent=2, allow_nan=False) + "\n").encode("utf-8")


def roster() -> dict[str, str | None]:
    # Reuse the checked-in industry source collector's canonical ISO table,
    # reading its declaration as data rather than executing JavaScript.
    source = (ROOT / "tools/industry/collect_industry_1990.cjs").read_text(encoding="utf-8")
    match = re.search(r"const ISO = Object\.fromEntries\(`(.*?)`\.trim\(\)", source, re.S)
    if not match:
        raise ValueError("Industry ISO declaration changed; inspect and update the mapping reader")
    result = {}
    for pair in match.group(1).strip().split(";"):
        nation, iso = pair.split()
        if nation in result:
            raise ValueError(f"Duplicate canonical nation {nation}")
        result[nation] = None if iso == "-" else iso
    ids = {json.loads(p.read_text(encoding="utf-8"))["id"] for p in (ROOT / "spheres-sim/data/nations").glob("*.json")}
    if set(result) != ids:
        raise ValueError(f"Roster drift: unmapped={sorted(ids - set(result))}; obsolete={sorted(set(result) - ids)}")
    if len(ids) != 137:
        raise ValueError("Opening roster changed from 137 nations; review coverage before updating this assertion")
    return dict(sorted(result.items()))


def request_url(indicator: str) -> str:
    return f"{API}/country/all/indicator/{indicator}?date={WINDOW[0]}:{WINDOW[1]}&format=json&per_page=20000"


def fetch(indicator: str, enabled: bool, refresh: bool) -> tuple[str, dict, list[dict], str]:
    path = CACHE / f"{indicator}.json.gz"
    if refresh or not path.exists():
        if not enabled:
            raise FileNotFoundError(f"Missing source snapshot {path}; run with --fetch")
        last_error = None
        for attempt in range(3):
            try:
                request = Request(request_url(indicator), headers={"User-Agent": "Spheres-population-data/1.0 (public historical data collector)"})
                with urlopen(request, timeout=45) as response:
                    payload = response.read()
                data = json.loads(payload)
                if not isinstance(data, list) or len(data) != 2 or not isinstance(data[1], list):
                    raise ValueError(f"Unexpected WDI response for {indicator}: {str(data)[:200]}")
                if int(data[0].get("pages", 0)) != 1 or len(data[1]) != int(data[0]["total"]):
                    raise ValueError(f"WDI pagination changed for {indicator}; refusing an incomplete source")
                CACHE.mkdir(parents=True, exist_ok=True)
                # gzip mtime=0 and normalized JSON make cache encoding reproducible.
                path.write_bytes(gzip.compress(encoded(data), mtime=0))
                break
            except Exception as error:
                last_error = error
                if attempt == 2:
                    raise
                time.sleep(attempt + 1)
        if not path.exists():
            raise RuntimeError(last_error)
    raw = gzip.decompress(path.read_bytes())
    metadata, rows = json.loads(raw)
    if int(metadata.get("pages", 0)) != 1 or len(rows) != int(metadata["total"]):
        raise ValueError(f"Incomplete cached response for {indicator}")
    return indicator, metadata, rows, hashlib.sha256(raw).hexdigest()


def build(responses: list[tuple], mapping: dict) -> dict:
    observations = {}
    sources = {}
    for indicator, metadata, rows, digest in sorted(responses):
        sources[indicator] = {
            "provider": SOURCE,
            "url": request_url(indicator),
            "metadata_url": f"https://databank.worldbank.org/metadataglossary/world-development-indicators/series/{indicator}",
            "snapshot": f"tools/population/social_source_cache/{indicator}.json.gz",
            "uncompressed_sha256": digest,
            "source_last_updated": metadata.get("lastupdated"),
            "license": "CC BY 4.0",
        }
        for row in rows:
            if row["indicator"]["id"] != indicator:
                raise ValueError(f"Unexpected indicator in response for {indicator}")
            value = row.get("value")
            if value is None or not row.get("countryiso3code"):
                continue
            if isinstance(value, bool) or not isinstance(value, (int, float)) or not math.isfinite(value) or value < 0:
                raise ValueError(f"Invalid numeric observation: {row}")
            key = (row["countryiso3code"], indicator, int(row["date"]))
            if key in observations:
                raise ValueError(f"Duplicate observation: {key}")
            observations[key] = value

    def observed(iso, indicator, maximum=100, exact=False):
        years = [YEAR] if exact else sorted(range(WINDOW[0], WINDOW[1] + 1), key=lambda y: (abs(y - YEAR), y))
        for year in years:
            value = observations.get((iso, indicator, year))
            if value is None:
                continue
            if maximum is not None and value > maximum:
                raise ValueError(f"Observation outside expected units: {iso} {indicator} {year} {value}")
            return {"value": value / 100, "year": year, "indicator": indicator, "source": SOURCE,
                    "iso3": iso, "quality": "source_1990" if year == YEAR else "source_nearest_1988_1992"}
        return None

    countries = {}
    for nation, iso in mapping.items():
        country = {"iso3": iso, "notes": [], "class_shares": None}
        if nation in TERRITORY_NOTES:
            country["notes"].append(TERRITORY_NOTES[nation])
        for field, indicator, denominator, agency, maximum in FIELDS:
            result = None
            if nation in COMPOSITES:
                if field in AGE_FIELDS:
                    components = []
                    for component in COMPOSITES[nation]:
                        age = observed(component, indicator, exact=True)
                        population = observations.get((component, "SP.POP.TOTL", YEAR))
                        if age is None or population is None or population <= 0:
                            country["notes"].append(f"{field}: missing exact-1990 component {component}; aggregate withheld.")
                            components = []
                            break
                        components.append({"iso3": component, "population": population, "value": age["value"]})
                    if components:
                        total = math.fsum(c["population"] for c in components)
                        result = {"value": math.fsum(c["value"] * c["population"] for c in components) / total,
                                  "year": YEAR, "indicator": indicator, "source": SOURCE,
                                  "quality": "source_1990_territorial_reconstruction", "weight_indicator": "SP.POP.TOTL",
                                  "components": components}
            elif iso:
                result = observed(iso, indicator, maximum, exact=field in AGE_FIELDS)
                if field == "unemployment":
                    national = observed(iso, "SL.UEM.TOTL.NE.ZS", exact=True)
                    # Prefer a national observation in the actual opening year.
                    # Otherwise use ILO's consistent modeled series, then the
                    # nearest national observation only if ILO has no value.
                    result = national or result or observed(iso, "SL.UEM.TOTL.NE.ZS")
                    if result and result["indicator"] == "SL.UEM.TOTL.NE.ZS":
                        agency = "ILO national estimates"
            if result:
                result["denominator"] = denominator
                result["source_agency"] = agency
            country[field] = result
        country["observed_field_count"] = sum(country[f] is not None for f, *_ in FIELDS)
        countries[nation] = country
    coverage = {}
    for field, *_ in FIELDS:
        values = [country[field] for country in countries.values()]
        coverage[field] = {"available": sum(v is not None for v in values),
                           "exact_1990": sum(v is not None and v["year"] == YEAR for v in values),
                           "territorial_reconstructions": sum(v is not None and "components" in v for v in values),
                           "missing": [n for n, c in countries.items() if c[field] is None]}
    result = {
        "schema_version": 1, "target_year": YEAR, "roster_count": len(mapping),
        "value_units": "fractions (source percentages divided by 100); gross enrollment may exceed 1",
        "selection": "Age shares: exact 1990 only. Other fields: nearest 1988-1992, tie to earlier year. Unemployment prefers exact-1990 national estimates, then nearest ILO modeled estimate, then nearest national estimate.",
        "historical_class_shares": "Not sourced; class_shares remains null for every nation. Gameplay class allocation is a separate model, not historical data.",
        "sources": sources, "coverage": coverage, "countries": countries,
    }
    validate(result, mapping)
    return result


def validate(data: dict, mapping: dict) -> None:
    assert set(data["countries"]) == set(mapping)
    for nation, country in data["countries"].items():
        assert country["class_shares"] is None, nation
        values = [country[field] for field in sorted(AGE_FIELDS)]
        if all(values):
            assert abs(math.fsum(v["value"] for v in values) - 1) < 1e-9, (nation, values)
        for field, _, _, _, maximum in FIELDS:
            value = country[field]
            if value is None:
                continue
            assert value["indicator"] in data["sources"], (nation, field)
            assert math.isfinite(value["value"]) and value["value"] >= 0, (nation, field)
            assert maximum is None or value["value"] <= 1, (nation, field)
            assert WINDOW[0] <= value["year"] <= WINDOW[1], (nation, field)
            if field in AGE_FIELDS:
                assert value["year"] == YEAR
            if "components" in value:
                assert sorted(c["iso3"] for c in value["components"]) == sorted(COMPOSITES[nation])


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--fetch", action="store_true", help="download missing public source responses")
    parser.add_argument("--refresh", action="store_true", help="explicitly replace frozen source responses; requires --fetch")
    parser.add_argument("--check", action="store_true", help="verify frozen artifact without changing it")
    args = parser.parse_args()
    if args.refresh and not args.fetch:
        parser.error("--refresh requires --fetch")
    mapping = roster()
    indicators = sorted({indicator for _, indicator, *_ in FIELDS} | set(EXTRA))
    with ThreadPoolExecutor(max_workers=4) as pool:
        responses = list(pool.map(lambda indicator: fetch(indicator, args.fetch, args.refresh), indicators))
    data = build(responses, mapping)
    output = encoded(data)
    if args.check:
        if not OUT.exists() or OUT.read_bytes() != output:
            raise SystemExit("Population artifact differs from the frozen sources; rebuild and review the difference")
        print("PASS: 137-nation roster, observed units, 1990 age closure, complete territorial weights, and byte-identical offline rebuild")
    else:
        OUT.write_bytes(output)
        print(f"Wrote {OUT.relative_to(ROOT)} ({len(output):,} bytes)")
    for field, counts in data["coverage"].items():
        print(f"{field}: {counts['available']}/137 available, {counts['exact_1990']} exact 1990, {counts['territorial_reconstructions']} reconstructed")


if __name__ == "__main__":
    main()
