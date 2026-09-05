#!/usr/bin/env python3
"""Validate and aggregate complete daily_calibration artifacts (stdlib only).

python tools/calibration/aggregate.py --input-dir artifacts --output merged.csv
python tools/calibration/aggregate.py --self-test

Each input CSV must have its original sibling .summary.json. No live campaigns
are read and no trajectories are rerun. Missing observations stay missing.
"""
from __future__ import annotations

import argparse
import csv
import datetime as dt
import io
import json
import math
import os
from pathlib import Path
import statistics
import tempfile

FORMAT = "spheres-daily-calibration-aggregate"
IDENTIFIERS = {"scenario", "seed", "elapsed_years", "date", "country", "alive"}
META_COLUMNS = ["economic_competition", "instrument_schema", "instrument", "source_csv"]


class InvalidArtifacts(ValueError):
    pass


def require(condition, message):
    if not condition:
        raise InvalidArtifacts(message)


def integer(value, label):
    require(type(value) is int and value >= 0, f"{label}: expected nonnegative integer")
    return value


def load_json(path):
    try:
        return json.loads(path.read_text(encoding="utf-8-sig"),
                          parse_constant=lambda v: (_ for _ in ()).throw(ValueError(f"nonfinite {v}")))
    except (OSError, ValueError) as error:
        raise InvalidArtifacts(f"{path}: {error}") from error


def number_stats(values, total):
    n = len(values)
    return {"n": n, "missing": total - n,
            "mean": statistics.mean(values) if n else None,
            "sample_variance": statistics.variance(values) if n > 1 else None,
            "min": min(values) if n else None, "max": max(values) if n else None}


def matches_cell(cell, value):
    if value is None:
        return cell == ""
    if isinstance(value, bool):
        return cell == str(value).lower()
    if isinstance(value, str):
        return cell == value
    if isinstance(value, (int, float)):
        try:
            parsed = int(cell) if type(value) is int else float(cell)
            return math.isfinite(parsed) and parsed == value
        except (ValueError, OverflowError):
            return False
    return False


def read_artifact(path, input_dir):
    summary_path = path.with_suffix(".summary.json")
    require(summary_path.is_file(), f"{path}: missing summary; partial runs are not aggregate inputs")
    summary = load_json(summary_path)
    require(isinstance(summary, dict), f"{summary_path}: expected summary object")
    require(summary.get("format") != FORMAT, f"{path}: an aggregate is not an independent input run")
    years = integer(summary.get("years"), f"{path}: years")
    require(years <= 100, f"{path}: unsupported campaign length")
    ai = summary.get("economic_competition")
    require(type(ai) is bool, f"{path}: missing boolean economic_competition metadata")
    seeds = summary.get("seeds")
    require(isinstance(seeds, list) and seeds, f"{path}: missing seeds")
    for seed in seeds:
        integer(seed, f"{path}: seed")
    require(len(set(seeds)) == len(seeds), f"{path}: duplicate summary seeds")
    schema = integer(summary.get("instrument_schema", 1), f"{path}: instrument_schema")
    instrument = summary.get("instrument", "legacy_daily_calibration_v1")
    require(isinstance(instrument, str) and instrument, f"{path}: invalid instrument")
    groups = summary.get("results")
    require(isinstance(groups, list) and groups, f"{path}: no completed result groups")
    terminals = {}
    scenarios = set()
    for group in groups:
        scenario = group.get("scenario")
        require(isinstance(scenario, str) and scenario and scenario not in scenarios,
                f"{path}: duplicate or invalid result scenario")
        scenarios.add(scenario)
        terminal_rows = group.get("terminal_rows")
        require(isinstance(terminal_rows, list) and group.get("runs") == len(terminal_rows) == len(seeds),
                f"{path}/{scenario}: completed run count does not match declared seeds")
        for row in terminal_rows:
            require(isinstance(row, dict), f"{path}: invalid terminal row")
            seed = integer(row.get("seed"), f"{path}: terminal seed")
            key = (scenario, seed)
            require(row.get("scenario") == scenario and seed in seeds and key not in terminals,
                    f"{path}: duplicate or inconsistent terminal row {key}")
            require(row.get("elapsed_years") == years,
                    f"{path}/{key}: terminal year is incomplete")
            terminals[key] = row
        require({seed for name, seed in terminals if name == scenario} == set(seeds),
                f"{path}/{scenario}: a declared seed is incomplete")
    rule_records = summary.get("rules_by_run")
    require(isinstance(rule_records, list), f"{path}: missing per-run rules")
    rules = {}
    for record in rule_records:
        key = (record.get("scenario"), record.get("seed"))
        rule = record.get("rules")
        require(key in terminals and key not in rules and isinstance(rule, dict),
                f"{path}: duplicate, unknown or missing rules for {key}")
        require(rule.get("seed") == key[1], f"{path}/{key}: rule seed mismatch")
        require(type(rule.get("economic_competition", False)) is bool
                and rule.get("economic_competition", False) == ai,
                f"{path}/{key}: economic-AI flag disagrees with rules")
        if "physical_logistics" in summary:
            require(rule.get("physical_logistics", False) == summary["physical_logistics"],
                    f"{path}/{key}: physical-logistics flag disagrees with rules")
        rules[key] = {k: v for k, v in rule.items() if k != "seed"}
    require(set(rules) == set(terminals), f"{path}: missing per-run rules")
    try:
        with path.open(newline="", encoding="utf-8-sig") as handle:
            reader = csv.DictReader(handle)
            fields = reader.fieldnames or []
            require(fields and len(fields) == len(set(fields)) and IDENTIFIERS <= set(fields),
                    f"{path}: missing required or duplicate CSV columns")
            rows = list(reader)
    except (OSError, csv.Error) as error:
        raise InvalidArtifacts(f"{path}: {error}") from error
    by_run = {}
    for row in rows:
        require(None not in row and None not in row.values(), f"{path}: malformed CSV row")
        try:
            seed = int(row["seed"])
            age = int(row["elapsed_years"])
        except ValueError as error:
            raise InvalidArtifacts(f"{path}: invalid seed/year") from error
        key = (row["scenario"], seed)
        require(key in terminals, f"{path}: CSV contains undeclared run {key}")
        require(0 <= age <= years, f"{path}/{key}: CSV age outside declared range")
        dates = by_run.setdefault(key, {})
        require(age not in dates, f"{path}/{key}: duplicate annual row {age}")
        expected_date = dt.date(1990 + age, 1, 1).isoformat()
        require(row["date"] == expected_date,
                f"{path}/{key}: expected actual annual boundary {expected_date}, got {row['date']}")
        dates[age] = row
    require(set(by_run) == set(terminals), f"{path}: CSV omits a completed run")
    runs = []
    relative = path.relative_to(input_dir).as_posix()
    for key, terminal in sorted(terminals.items()):
        annual = by_run[key]
        require(set(annual) == set(range(years + 1)), f"{path}/{key}: missing annual rows; partial run")
        require(set(fields) == set(terminal), f"{path}/{key}: CSV and terminal schema disagree")
        require(all(matches_cell(annual[years][field], value) for field, value in terminal.items()),
                f"{path}/{key}: CSV terminal row disagrees with completed summary")
        require(all(row["country"] == terminal["country"] for row in annual.values()),
                f"{path}/{key}: tracked government changed within run")
        for row in annual.values():
            require(row["alive"] in ("true", "false"), f"{path}/{key}: invalid alive flag")
            # Every declared numeric column must remain numeric/nullable over
            # the trajectory, not merely in the terminal summary.
            for field, value in terminal.items():
                if field not in IDENTIFIERS and (value is None or type(value) in (int, float)) and row[field] != "":
                    try:
                        require(math.isfinite(float(row[field])), f"{path}/{key}: nonfinite {field}")
                    except ValueError as error:
                        raise InvalidArtifacts(f"{path}/{key}: nonnumeric {field}") from error
        runs.append({"scenario": key[0], "seed": key[1], "economic_competition": ai,
                     "years": years, "rules": rules[key], "instrument_schema": schema,
                     "instrument": instrument, "source_csv": relative, "terminal": terminal,
                     "rows": [annual[age] for age in range(years + 1)]})
    metadata = {key: summary[key] for key in (
        "instrument", "instrument_schema", "calendar", "initialization", "scope", "sampling",
        "metric_definitions", "unavailable_metrics", "scenarios", "run_diagnostics") if key in summary}
    return runs, {"csv": relative, "summary": summary_path.relative_to(input_dir).as_posix(),
                  "instrument_schema": schema, "instrument": instrument,
                  "legacy_instrument": schema < 2, "metadata": metadata}


def atomic_text(path, text):
    path.parent.mkdir(parents=True, exist_ok=True)
    temp = None
    try:
        with tempfile.NamedTemporaryFile(mode="w", encoding="utf-8", newline="", dir=path.parent,
                                         prefix=path.name + ".", suffix=".tmp", delete=False) as handle:
            temp = Path(handle.name)
            handle.write(text)
            handle.flush()
            os.fsync(handle.fileno())
        os.replace(temp, path)
    finally:
        if temp is not None and temp.exists():
            temp.unlink()


def aggregate(input_dir, output):
    input_dir, output = Path(input_dir).resolve(), Path(output).resolve()
    require(input_dir.is_dir(), f"Not an input directory: {input_dir}")
    require(output.suffix.lower() == ".csv", "--output must name a .csv file")
    report_path = output.with_suffix(".summary.json")
    if output.exists():
        require(report_path.is_file() and load_json(report_path).get("format") == FORMAT,
                "Refusing to replace an existing source CSV; choose a new aggregate output")
    paths = sorted(path for path in input_dir.rglob("*.csv") if path.resolve() != output)
    require(paths, "No independent CSV inputs found")
    runs, sources = [], []
    seen = set()
    comparisons = {}
    for path in paths:
        file_runs, metadata = read_artifact(path, input_dir)
        sources.append(metadata)
        for run in file_runs:
            unique = (run["scenario"], run["economic_competition"], run["seed"])
            require(unique not in seen, f"Duplicate independent run {unique}")
            seen.add(unique)
            key = unique[:2]
            members = comparisons.setdefault(key, [])
            if members:
                require(run["years"] == members[0]["years"], f"{key}: mismatched campaign years")
                require(run["rules"] == members[0]["rules"], f"{key}: mismatched simulation rules (excluding seed)")
            members.append(run)
            runs.append(run)
    results = []
    for (scenario, ai), members in sorted(comparisons.items()):
        terminals = [run["terminal"] for run in members]
        fields = sorted(set().union(*(row.keys() for row in terminals)) - IDENTIFIERS)
        metrics = {}
        for field in fields:
            values = [row[field] for row in terminals if row.get(field) is not None]
            if all(type(value) in (int, float) for value in values):
                require(all(math.isfinite(value) for value in values), f"{scenario}/{ai}: nonfinite {field}")
                metrics[field] = number_stats(values, len(members))
        schemas = sorted({run["instrument_schema"] for run in members})
        results.append({"scenario": scenario, "economic_competition": ai, "years": members[0]["years"],
                        "n": len(members), "seeds": sorted(run["seed"] for run in members),
                        "surviving_governments": sum(row["alive"] is True for row in terminals),
                        "rules_without_seed": members[0]["rules"], "instrument_schemas": schemas,
                        "legacy_instrument_present": any(schema < 2 for schema in schemas),
                        "mixed_instruments": len({(r["instrument_schema"], r["instrument"]) for r in members}) > 1,
                        "terminal_statistics": metrics,
                        "runs": [{k: run[k] for k in ("seed", "source_csv", "instrument", "instrument_schema")}
                                 for run in sorted(members, key=lambda run: run["seed"])]})
    report = {"format": FORMAT, "version": 1, "input_directory": str(input_dir), "run_count": len(runs),
              "source_count": len(sources), "sources": sources, "results": results,
              "scope": "Completed annual trajectories only. Groups are scenario and economic-AI flag; years and all other rules must match within each group. Means and unbiased n-1 sample variance describe terminal observations across distinct seeds, not annual rows as independent samples.",
              "missing_metrics": "Absent columns and null observations remain missing, with per-metric n and missing counts. No legacy value is imputed as zero. Instrument/schema provenance is retained; mixed or legacy instruments are explicitly labeled and need review before comparing changed definitions.",
              "limits": "Descriptive evidence only; no historical-fit, difficulty or statistical power claim. Fewer than two available values have null sample variance. Source metric definitions and unavailable metrics remain attached."}
    columns = META_COLUMNS + sorted(set().union(*(run["rows"][0].keys() for run in runs)) - set(META_COLUMNS))
    stream = io.StringIO(newline="")
    writer = csv.DictWriter(stream, fieldnames=columns)
    writer.writeheader()
    for run in sorted(runs, key=lambda r: (r["scenario"], r["economic_competition"], r["seed"])):
        for row in run["rows"]:
            writer.writerow({**row, **{key: str(run[key]).lower() if type(run[key]) is bool else run[key]
                                      for key in META_COLUMNS}})
    # Validate all inputs before replacing either published artifact.
    encoded_report = json.dumps(report, indent=2, allow_nan=False) + "\n"
    atomic_text(output, stream.getvalue())
    atomic_text(report_path, encoded_report)
    return report


def self_test():
    import unittest

    def fixture(root, name, seed=7, ai=False, years=2, schema=1, gdp=2.0, extra=False):
        path = root / f"{name}.csv"
        rows = []
        for age in range(years + 1):
            row = {"scenario": "idle_human", "seed": seed, "elapsed_years": age,
                   "date": f"{1990+age}-01-01", "country": "United States", "alive": True,
                   "gdp_bn": gdp if age == years else 1.0, "unavailable_loss": None}
            if extra:
                row["new_metric"] = 9.0
            rows.append(row)
        with path.open("w", newline="", encoding="utf-8") as handle:
            writer = csv.DictWriter(handle, fieldnames=rows[0])
            writer.writeheader()
            for row in rows:
                writer.writerow({k: str(v).lower() if type(v) is bool else "" if v is None else v for k, v in row.items()})
        summary = {"years": years, "seeds": [seed], "economic_competition": ai,
                   "rules_by_run": [{"scenario": "idle_human", "seed": seed,
                                     "rules": {"seed": seed, "economic_competition": ai, "daily_simulation": True}}],
                   "results": [{"scenario": "idle_human", "runs": 1, "terminal_rows": [rows[-1]]}]}
        if schema >= 2:
            summary.update(instrument_schema=schema, instrument="daily_calibration_v2")
        path.with_suffix(".summary.json").write_text(json.dumps(summary), encoding="utf-8")
        return path

    class Tests(unittest.TestCase):
        def setUp(self):
            self.temp = tempfile.TemporaryDirectory()
            self.addCleanup(self.temp.cleanup)
            self.root = Path(self.temp.name)
            self.output = self.root / "merged.csv"

        def test_variance_ai_groups_and_missing_legacy_metrics(self):
            fixture(self.root, "a")
            fixture(self.root, "b", seed=42, gdp=4.0, schema=2, extra=True)
            fixture(self.root, "c", ai=True)
            report = aggregate(self.root, self.output)
            group = report["results"][0]
            self.assertEqual(group["n"], 2)
            self.assertEqual(group["terminal_statistics"]["gdp_bn"]["mean"], 3.0)
            self.assertEqual(group["terminal_statistics"]["gdp_bn"]["sample_variance"], 2.0)
            self.assertEqual(group["terminal_statistics"]["new_metric"]["n"], 1)
            self.assertEqual(group["terminal_statistics"]["new_metric"]["missing"], 1)
            self.assertIsNone(group["terminal_statistics"]["new_metric"]["sample_variance"])
            self.assertEqual(group["terminal_statistics"]["unavailable_loss"]["n"], 0)
            self.assertTrue(group["mixed_instruments"])
            self.assertTrue(group["legacy_instrument_present"])
            self.assertEqual(aggregate(self.root, self.output)["run_count"], 3)

        def test_rejects_duplicate_seed(self):
            fixture(self.root, "a")
            fixture(self.root, "b")
            with self.assertRaisesRegex(InvalidArtifacts, "Duplicate independent"):
                aggregate(self.root, self.output)
            self.assertFalse(self.output.exists())

        def test_rejects_partial_csv_and_missing_summary(self):
            path = fixture(self.root, "a")
            path.write_text("\n".join(path.read_text().splitlines()[:-1]) + "\n")
            with self.assertRaisesRegex(InvalidArtifacts, "missing annual"):
                aggregate(self.root, self.output)
            path.with_suffix(".summary.json").unlink()
            with self.assertRaisesRegex(InvalidArtifacts, "missing summary"):
                aggregate(self.root, self.output)

        def test_rejects_rules_and_year_mismatch(self):
            fixture(self.root, "a")
            path = fixture(self.root, "b", seed=42)
            summary = load_json(path.with_suffix(".summary.json"))
            summary["rules_by_run"][0]["rules"]["daily_simulation"] = False
            path.with_suffix(".summary.json").write_text(json.dumps(summary))
            with self.assertRaisesRegex(InvalidArtifacts, "mismatched simulation rules"):
                aggregate(self.root, self.output)
            fixture(self.root, "b", seed=42, years=1)
            with self.assertRaisesRegex(InvalidArtifacts, "mismatched campaign years"):
                aggregate(self.root, self.output)

        def test_rejects_terminal_disagreement(self):
            path = fixture(self.root, "a")
            summary = load_json(path.with_suffix(".summary.json"))
            summary["results"][0]["terminal_rows"][0]["gdp_bn"] = 99.0
            path.with_suffix(".summary.json").write_text(json.dumps(summary))
            with self.assertRaisesRegex(InvalidArtifacts, "terminal row disagrees"):
                aggregate(self.root, self.output)

    result = unittest.TextTestRunner(verbosity=2).run(unittest.defaultTestLoader.loadTestsFromTestCase(Tests))
    return 0 if result.wasSuccessful() else 1


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--input-dir", type=Path)
    parser.add_argument("--output", type=Path)
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    if args.self_test:
        return self_test()
    if args.input_dir is None or args.output is None:
        parser.error("--input-dir and --output are required unless --self-test is selected")
    try:
        report = aggregate(args.input_dir, args.output)
    except (InvalidArtifacts, OSError) as error:
        parser.exit(2, f"Calibration aggregation refused: {error}\n")
    print(json.dumps({"csv": str(args.output.resolve()), "summary": str(args.output.with_suffix('.summary.json').resolve()),
                      "runs": report["run_count"], "groups": len(report["results"])}))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
