#!/usr/bin/env python3
"""Source-pinned reporting of producers excluded by the district roster.

This bounded audit covers 1990 bauxite only. It adds no nation, deposit or
runtime production, and does not certify coverage of other commodities.
"""
import argparse
import hashlib
import json
import math
from pathlib import Path

import crosswalk
import sources

ROOT = Path(__file__).resolve().parents[2]
WORKBOOK = Path(__file__).resolve().parent / "fixtures/ds896-aluminum.xlsx"
WORKBOOK_SHA256 = "0d8ae069552229b306ce4795a29704d7eb7c859346cfb1058ecddf1e9d268214"
SOURCE = "ds896_bauxite"
YEAR = 1990


def bauxite_rows(table, roster):
    """Preserve positive source values rejected by the existing crosswalk."""
    result = []
    for country, value in sorted(table.items()):
        if not isinstance(value, (int, float)) or not math.isfinite(value) or value < 0:
            raise ValueError(f"invalid production for {country}")
        if country == "World":  # the sole aggregate in this pinned worksheet
            continue
        if country in crosswalk.IGNORE:
            if country in roster:
                raise ValueError(f"crosswalk ignores a now-rostered country: {country}")
            if value > 0:
                result.append({
                    "nation": country,
                    "source_label": country,
                    "year": YEAR,
                    "value": value,
                    "units": "metric tons",
                    "source": SOURCE,
                    "basis": "outside_district_roster",
                    "reason": "Reported producer outside the district roster; no modeled nation or district receives this production.",
                })
        else:
            nation = crosswalk.DS896.get(country, country)
            if nation not in roster:
                raise ValueError(f"unmapped source country: {country}")
    return result


def build_report(roster, source, workbook=WORKBOOK):
    raw = Path(workbook).read_bytes()
    if hashlib.sha256(raw).hexdigest() != WORKBOOK_SHA256:
        raise ValueError("bauxite coverage workbook differs from the pinned source")
    if source.get("sha256") != WORKBOOK_SHA256 or source.get("bytes") != len(raw):
        raise ValueError("artifact source identity differs from the reviewed workbook")
    names, rows = sources.read_xlsx(workbook, 1)
    if names[1] != "Bauxite" or not any(row and row[0].strip() == "(Metric tons)" for row in rows):
        raise ValueError("bauxite worksheet or source units differ")
    table = sources.ds896_year(workbook, 1, year=str(YEAR))
    entries = {"bauxite": bauxite_rows(table, set(roster))}
    coverage = {
        "reviewed_commodities": ["bauxite"],
        "year": YEAR,
        "source": SOURCE,
        "source_sha256": WORKBOOK_SHA256,
        "worksheet": "Bauxite",
        "units": "metric tons",
        "unreviewed_commodities": "All other commodities remain unaudited for crosswalk omissions by this bounded reporting pass; no marker does not establish complete coverage.",
        "meaning": "Positive national production excluded because the country is outside the district roster. Reporting only; these values are not added to national production or allocated to districts.",
    }
    return entries, coverage


def apply_report(artifact, roster, workbook=WORKBOOK):
    entries, coverage = build_report(roster, artifact["sources"][SOURCE], workbook)
    # Only these reporting keys are owned here. Existing resource values and
    # all previous limitations are deliberately retained unchanged.
    artifact.setdefault("unrostered_producers", {})["bauxite"] = entries["bauxite"]
    artifact["meta"]["unrostered_producer_coverage"] = coverage
    return artifact


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--artifact", type=Path, default=ROOT / "spheres-web/data/district_resources.json")
    parser.add_argument("--roster", type=Path, default=ROOT / "spheres-sim/data/districts.json")
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--check", action="store_true")
    mode.add_argument("--write", action="store_true")
    args = parser.parse_args()
    artifact = json.loads(args.artifact.read_text(encoding="utf-8"))
    roster = json.loads(args.roster.read_text(encoding="utf-8"))["nations"]
    entries, coverage = build_report(roster, artifact["sources"][SOURCE])
    if args.check:
        if artifact.get("unrostered_producers", {}).get("bauxite") != entries["bauxite"] or artifact["meta"].get("unrostered_producer_coverage") != coverage:
            raise SystemExit("FAIL: source-pinned unrostered bauxite coverage is missing or stale")
    else:
        apply_report(artifact, roster)
        updated = (json.dumps(artifact, indent=1, sort_keys=True, ensure_ascii=False) + "\n").encode("utf-8")
        if args.artifact.read_bytes() != updated:
            args.artifact.write_bytes(updated)
    print(f"PASS: {len(entries['bauxite'])} unrostered 1990 bauxite producers explicitly reported; other commodities remain unaudited")


if __name__ == "__main__":
    main()
