"""Run predeclared native long-campaign stability cells; never build a binary.

Example (actual native execution, potentially many hours for the full matrix):
  python tools/campaign/stability_matrix.py --binary <native-test-exe>
    --revision <40-character-commit> --plan tools/campaign/stability-pilot.json
    --out <new-directory>

Every attempt gets a new directory. The pilot cannot satisfy the full matrix.
Even a complete full matrix leaves the controlled USSR-to-Russia case and other
campaign certification gates outstanding. Synthetic unit tests are runner tests,
not native campaign evidence. All verdicts retain qualification=false.

Only runner-created campaign archives may be replaced by verified gzip payloads.
Their original SHA-256/byte count and restore names remain in archive-manifest.json.
Reports, logs, requests and the original plan are never overwritten or removed.

Review a completed (possibly relocated) run without native execution:
  python tools/campaign/stability_matrix.py --verify <retained-run>
Restore its original archive bytes, keeping both originals and gzip evidence:
  python tools/campaign/stability_matrix.py --verify <retained-run> --restore <NEW-directory>
Verification rehashes all retained files, streams/decompresses every restore
payload, independently recomputes the native raw/canonical FNV fingerprints,
and reruns report/test-log/coverage checks. It preserves an original failed
verdict and never awards certification. The original binary is not executed or
required locally; its recorded provenance remains explicit. Full restoration
is claimed only after the new directory passes the same verifier.
"""
from __future__ import annotations

import argparse
import datetime as dt
import gzip
import hashlib
import json
import math
import os
from pathlib import Path, PurePosixPath, PureWindowsPath
import re
import shutil
import subprocess
import sys
import time

TEST_NAME = "s25_stability_tests::s25_stability_cell"
COUNTRIES = ("France", "Japan", "India", "Brazil", "SouthAfrica", "Tonga", "SaudiArabia", "USSR")
SEEDS = (1990, 7, 42)
START = dt.date(1990, 1, 1)
FULL_THROUGH = "2035-12-31"
PILOT_THROUGH = "1991-02-02"
REVISION_RE = re.compile(r"[0-9a-f]{40}\Z")
ID_RE = re.compile(r"[a-z0-9][a-z0-9-]{0,63}\Z")


class InvalidEvidence(ValueError):
    pass


def require(condition, message):
    if not condition:
        raise InvalidEvidence(message)


def utc_now():
    return dt.datetime.now(dt.timezone.utc).isoformat()


def reject_constant(value):
    raise InvalidEvidence(f"Nonfinite JSON number: {value}")


def unique_object(pairs):
    result = {}
    for key, value in pairs:
        require(key not in result, f"Duplicate JSON key: {key}")
        result[key] = value
    return result


def json_bytes(value):
    return (json.dumps(value, indent=2, ensure_ascii=False, allow_nan=False) + "\n").encode("utf-8")


def parse_json(data):
    return json.loads(data.decode("utf-8"), object_pairs_hook=unique_object,
                      parse_constant=reject_constant)


def read_json(path):
    return parse_json(Path(path).read_bytes())


def write_new(path, data):
    with Path(path).open("xb") as stream:
        stream.write(data)
        stream.flush()
        os.fsync(stream.fileno())


def write_json_new(path, value):
    write_new(path, json_bytes(value))


def hash_stream(stream):
    digest = hashlib.sha256()
    count = 0
    while block := stream.read(1024 * 1024):
        digest.update(block)
        count += len(block)
    return {"bytes": count, "sha256": digest.hexdigest()}


def file_pin(path):
    with Path(path).open("rb") as stream:
        return {"path": str(Path(path).resolve()), **hash_stream(stream)}


def integer(value, minimum=0):
    return type(value) is int and value >= minimum


def day(value):
    require(type(value) is str and re.fullmatch(r"\d{4}-\d{2}-\d{2}", value), "Expected ISO calendar date")
    try:
        return dt.date.fromisoformat(value)
    except ValueError as error:
        raise InvalidEvidence(f"Invalid calendar date: {value}") from error


def expected_cells(scope):
    if scope == "pilot":
        return {(COUNTRIES[0], 1990, PILOT_THROUGH), ("Tonga", 7, PILOT_THROUGH)}
    return {(country, seed, FULL_THROUGH) for country in COUNTRIES for seed in SEEDS}


def validate_plan(plan):
    require(type(plan) is dict, "Plan must be an object")
    require(set(plan) == {"format", "id", "scope", "cells"}, "Unexpected/missing plan fields")
    require(plan["format"] == "spheres-stability-plan/v1", "Unsupported plan format")
    scope = plan["scope"]
    require(scope in ("pilot", "full"), "Plan scope must be pilot or full")
    require(plan["id"] == f"s25-{scope}-v1", "Plan id disagrees with scope")
    require(type(plan["cells"]) is list, "Plan cells must be an array")
    ids, cases = set(), set()
    for cell in plan["cells"]:
        require(type(cell) is dict and set(cell) == {"id", "country", "seed", "through"}, "Unexpected/missing cell fields")
        require(type(cell["id"]) is str and ID_RE.fullmatch(cell["id"]), "Unsafe cell id")
        require(cell["id"] not in ids, "Duplicate cell id")
        ids.add(cell["id"])
        require(cell["country"] in COUNTRIES, "Unknown canonical native country")
        require(integer(cell["seed"]) and cell["seed"] in SEEDS, "Invalid canonical seed")
        require(day(cell["through"]) >= START, "Cell ends before campaign start")
        case = (cell["country"], cell["seed"], cell["through"])
        require(case not in cases, "Duplicate country/seed/date cell")
        cases.add(case)
    require(cases == expected_cells(scope), "Plan omits/adds/shortens a canonical cell")
    return plan


def validate_test_log(path):
    # Stream the output; an assertion containing a world must not be copied into
    # a second giant in-memory string or embedded in a verdict.
    running, results, test_ok = [], [], 0
    pattern = re.compile(r"^test result: (\w+)\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out; finished in .+$")
    with Path(path).open("r", encoding="utf-8", errors="strict") as stream:
        for raw in stream:
            line = raw.strip()
            if re.fullmatch(r"running \d+ tests?", line):
                running.append(line)
            if line.startswith("test result:"):
                match = pattern.fullmatch(line)
                require(match is not None, "Malformed native test summary")
                results.append(match.groups())
            if re.fullmatch(re.escape("test " + TEST_NAME + " ... ") + r"ok", line):
                test_ok += 1
    require(running in (["running 1 test"], ["running 1 tests"]), "Exactly one native test must execute")
    require(len(results) == 1 and results[0][:5] == ("ok", "1", "0", "0", "0"), "Native test did not pass exactly one nonignored test")
    require(test_ok == 1, "Missing/duplicate exact native test success")
    return {"executed": 1, "passed": 1, "failed": 0, "ignored": 0, "filtered": int(results[0][5])}


def owned_file(root, relative):
    require(type(relative) is str and relative and "\\" not in relative, "Artifact path must use relative forward slashes")
    parts = relative.split("/")
    require(all(part not in ("", ".", "..") for part in parts), "Unsafe artifact path")
    root = Path(root).resolve(strict=True)
    path = root.joinpath(*parts)
    require(not Path(relative).is_absolute(), "Absolute artifact path is forbidden")
    current = root
    for part in parts:
        current = current / part
        require(not current.is_symlink(), "Symlink artifact is forbidden")
    resolved = path.resolve(strict=True)
    require(resolved.is_relative_to(root) and resolved.is_file(), "Artifact escapes cell directory or is not a file")
    return resolved


def archive_content_pins(path, compressed=False):
    """Independently link native FNV reports to the actual raw/canonical bytes.

    Both FNV states share the entire large prefix; only the bounded terminal
    wall-clock member differs. SHA-256 proves canonical pair equality too.
    No parsed or duplicated world-sized JSON tree is needed.
    """
    digest = hashlib.sha256()
    total, tail, prefix_fnv = 0, b"", 0xcbf29ce484222325
    opener = gzip.open if compressed else open
    with opener(path, "rb") as stream:
        while block := stream.read(1024 * 1024):
            pending = tail + block
            if len(pending) > 256:
                prefix, tail = pending[:-256], pending[-256:]
                digest.update(prefix)
                prefix_fnv = fnv_update(prefix_fnv, prefix)
                total += len(prefix)
            else:
                tail = pending
    match = re.fullmatch(rb'(.*),"saved_unix":(0|[1-9][0-9]*)}', tail, re.DOTALL)
    require(match is not None and int(match[2]) <= 2 ** 64 - 1, "Missing canonical terminal saved_unix timestamp")
    retained = match[1] + b"}"
    digest.update(retained)
    return {"canonical": {"bytes": total + len(retained), "sha256": digest.hexdigest(),
                          "normalization": "Only terminal top-level saved_unix removed; every other byte preserved."},
            "file_fnv64": f"{fnv_update(prefix_fnv, tail):016x}",
            "canonical_fnv64": f"{fnv_update(prefix_fnv, retained):016x}"}


def canonical_archive_pin(path, compressed=False):
    return archive_content_pins(path, compressed)["canonical"]


def archive_campaign_files(root, names, journal, records=None):
    """Losslessly retain only declared, newly-created native campaign archives.

    This function never follows symlinks, overwrites existing gzip payloads or
    deletes external inputs. A raw file is removed only after streamed roundtrip
    verification and a durable restore record. On any error, remaining raw bytes
    are retained. The caller's new output tree is the ownership boundary.
    """
    root = Path(root).resolve(strict=True)
    require(len(names) == len(set(names)), "Duplicate archive path")
    if records is None:
        records = []
    for name in names:
        source = owned_file(root, name)
        require((source.name.endswith(".json") or source.name.endswith(".json.bak")) and source.name not in ("result.json", "request.json"), "Only campaign JSON archives may be archived")
        original = file_pin(source)
        target = Path(str(source) + ".gz")
        # Small structural prefix check prevents a faulty declaration from
        # deleting an unrelated report. All native archives use compact JSON.
        with source.open("rb") as stream:
            prefix = stream.read(128)
        require(prefix.startswith(b'{"format":"spheres-campaign","version":1,'), "Declared archive is not a native campaign envelope")
        with target.open("xb") as output:
            with gzip.GzipFile(filename="", mode="wb", fileobj=output, mtime=0) as compressed:
                with source.open("rb") as incoming:
                    while block := incoming.read(1024 * 1024):
                        compressed.write(block)
            output.flush()
            os.fsync(output.fileno())
        with gzip.open(target, "rb") as restored:
            decoded = hash_stream(restored)
        require(decoded == {k: original[k] for k in ("bytes", "sha256")}, "Archive roundtrip mismatch; raw bytes retained")
        require(file_pin(source) == original, "Archive source changed; raw bytes retained")
        packed = file_pin(target)
        record = {"original": original, "original_relative": name,
                  "gzip": packed, "gzip_relative": name + ".gz", "decoded": decoded,
                  "roundtrip_verified": True, "raw_removed": False,
                  "restore": "Decode this gzip into a NEW destination at original_relative; verify decoded bytes and sha256."}
        records.append(record)
        journal({"event": "archive_verified_before_removal", "record": record})
        source.unlink()  # Exact newly-created file, revalidated above; no recursive operation.
        record["raw_removed"] = True
        journal({"event": "archive_raw_removed", "original_relative": name,
                 "sha256": original["sha256"]})
    return records


def coverage(plan, outcomes):
    declared = {cell["id"]: cell for cell in plan["cells"]}
    seen, passed = set(), set()
    for outcome in outcomes:
        require(type(outcome) is dict and outcome.get("id") in declared, "Unknown result cell")
        require(outcome["id"] not in seen, "Duplicate result cell")
        seen.add(outcome["id"])
        require(type(outcome.get("passed")) is bool, "Result must have boolean passed")
        if outcome["passed"]:
            passed.add(outcome["id"])
    requested_passed = passed == set(declared)
    full_expected = expected_cells("full")
    completed = {(declared[c]["country"], declared[c]["seed"], declared[c]["through"]) for c in passed}
    missing_full = sorted(full_expected - completed)
    return {"declared": len(declared), "attempted": len(seen), "passed": len(passed),
            "failed": sorted(seen - passed), "missing_requested": sorted(set(declared) - seen),
            "requested_plan_passed": requested_passed,
            "pilot_passed": plan["scope"] == "pilot" and requested_passed,
            "full_matrix_passed": plan["scope"] == "full" and not missing_full and requested_passed,
            "missing_full_cases": [{"country": c, "seed": s, "through": d} for c, s, d in missing_full],
            "controlled_ussr_to_russia_case_passed": False,
            "s25_complete": False, "qualification": False}


# Native report validation is intentionally separate from subprocess execution;
# unit fixtures exercise rejection paths without pretending to be campaigns.

def expected_comparisons(through):
    end = day(through) + dt.timedelta(days=1)
    rows = [("opening", START)]
    after_reload = None
    current = START + dt.timedelta(days=1)
    while current <= end:
        if current == after_reload:
            rows.append(("day_after_reload", current))
            after_reload = None
        if current.day == 1:
            rows.append(("monthly_after_reload", current))
            after_reload = current + dt.timedelta(days=1)
        if (current.month, current.day) == (1, 1) or current.isoformat() in ("2026-09-07", "2026-09-08", "2035-12-31"):
            rows.append(("mandatory", current))
        current += dt.timedelta(days=1)
    rows.extend([("terminal", end), ("terminal_reload", end)])
    if through == FULL_THROUGH:
        rows.append(("sandbox_continuation", end + dt.timedelta(days=1)))
    return rows


def finite(value, nonnegative=False):
    return type(value) in (int, float) and math.isfinite(value) and (not nonnegative or value >= 0)


def fnv_update(value, data):
    for byte in data:
        value = ((value ^ byte) * 0x100000001b3) & 0xffffffffffffffff
    return value


def fnv64(data):
    return f"{fnv_update(0xcbf29ce484222325, data):016x}"


def is_fnv(value):
    return type(value) is str and re.fullmatch(r"[0-9a-f]{16}", value) is not None


def validate_native_result(result, cell, revision, native_root, retained_archives=None, recorded_native_root=None):
    native_root = Path(native_root).resolve(strict=True)
    recorded_native_root = recorded_native_root or str(native_root)
    full = cell["through"] == FULL_THROUGH
    expected_end = day(cell["through"]) + dt.timedelta(days=1)
    days = (expected_end - START).days
    fields = {"format", "id", "country", "seed", "through", "revision", "compiled_revision", "passed", "scope",
              "start_native_date", "end_native_date", "days_each_leg", "legs", "comparisons", "actions", "artifacts",
              "artifact_errors", "failure", "checks", "provenance", "initial_rules"}
    if full:
        fields.add("sandbox_end_native_date")
    require(type(result) is dict and set(result) == fields, "Missing/unexpected native result fields")
    require(result["format"] == "spheres-stability-result/v1", "Unexpected native report format")
    for key in ("id", "country", "seed", "through"):
        require(type(result[key]) is type(cell[key]) and result[key] == cell[key], f"Native cell {key} mismatch")
    require(result["revision"] == revision and result["compiled_revision"] == revision[:12], "Wrong or modified native candidate")
    require(result["passed"] is True and result["failure"] is None and result["artifact_errors"] == [], "Native failure or archive error recorded")
    require(type(result["scope"]) is str and "no S25 or CP1" in result["scope"], "Native scope must retain certification limitation")
    rules = result["initial_rules"]
    require(type(rules) is dict and type(rules.get("seed")) is int and rules["seed"] == cell["seed"], "Missing native starting rules/seed")
    for rule in ("daily_simulation", "economic_competition", "logistics_routes", "physical_logistics",
                 "military_operations", "production_system", "manufacturing_system", "industry_rebuild", "ideology_blocs"):
        require(rules.get(rule) is True, f"Required ordinary campaign rule not enabled: {rule}")
    require(rules.get("resource_gates", True) is True, "Resource gates were disabled")
    require(all(finite(rules.get(key)) and rules[key] == 1.0 for key in ("ai_aggression", "crisis_intensity")), "Workload intensity changed")
    require(result["start_native_date"] == START.isoformat() and result["end_native_date"] == expected_end.isoformat(), "Short/incorrect native date range")
    require(type(result["days_each_leg"]) is int and result["days_each_leg"] == days, "Incorrect calendar day count")
    require(type(result["legs"]) is int and result["legs"] == 2, "Both native legs are required")
    schedule = expected_comparisons(cell["through"])
    comparisons = result["comparisons"]
    require(type(comparisons) is list and len(comparisons) == len(schedule), "Missing/extra native comparisons")
    row_fields = {"kind", "date", "absolute_day", "matched", "canonical_bytes", "canonical_fnv64", "history_rows", "log_rows", "player"}
    allowed_players = {cell["country"], None} | ({"Russia"} if cell["country"] == "USSR" else set())
    for row, (kind, date) in zip(comparisons, schedule):
        require(type(row) is dict and set(row) == row_fields, "Malformed comparison row")
        require(row["kind"] == kind and row["date"] == date.isoformat(), "Comparison order/date differs from mandatory schedule")
        require(type(row["absolute_day"]) is int and row["absolute_day"] == (date - START).days, "Incorrect comparison calendar index")
        require(row["matched"] is True, "Unmatched native archives")
        require(integer(row["canonical_bytes"], 1) and is_fnv(row["canonical_fnv64"]), "Invalid comparison fingerprint")
        require(integer(row["history_rows"], 1) and integer(row["log_rows"]), "Missing/invalid retained history/log counts")
        require(row["player"] in allowed_players, "Unrelated player in comparison")
    require(comparisons[0]["player"] == cell["country"], "Opening player mismatch")
    terminal = next(r for r in comparisons if r["kind"] == "terminal")
    loaded = next(r for r in comparisons if r["kind"] == "terminal_reload")
    for key in ("canonical_bytes", "canonical_fnv64", "history_rows", "log_rows", "player"):
        require(loaded[key] == terminal[key], f"Final reload changed {key}")
    checks = result["checks"]
    expected_checks = {"daily_invariant_checks": days * 2, "sandbox_daily_invariant_checks": 2 if full else 0,
                       "native_validation_checks": len(schedule) * 2,
                       "monthly_reloads": sum(k == "monthly_after_reload" for k, _ in schedule),
                       "terminal_reloads": 2, "competition_adoptions": 2,
                       "full_horizon_pause": True if full else None, "sandbox_continued": full}
    require(type(checks) is dict and set(checks) == set(expected_checks), "Missing/unexpected native check counters")
    for key, expected in expected_checks.items():
        require(type(checks[key]) is type(expected) and checks[key] == expected, f"Incorrect/missing native check: {key}")
    if full:
        require(result["sandbox_end_native_date"] == (expected_end + dt.timedelta(days=1)).isoformat(), "Sandbox must settle the next actual day")
    actions = result["actions"]
    require(type(actions) is list and actions, "Ordinary action evidence missing")
    adoption, sandbox_actions, prior = 0, 0, START
    for index, action in enumerate(actions):
        require(type(action) is dict and action.get("matched") is True and type(action.get("legs")) is int and action["legs"] == 2, "Action did not match both legs")
        date = day(action.get("date"))
        require(prior <= date <= (expected_end if full else day(cell["through"])), "Action date outside ordered settled range")
        prior = date
        kind = action.get("kind")
        base = {"kind", "date", "legs", "matched"}
        if kind == "competition_adoption":
            require(set(action) == base | {"observation"} and index == 0 and date == START, "Adoption must be the exact opening action")
            observation = action["observation"]
            require(type(observation) is dict and set(observation) == {"command", "price_pc", "pc_before", "pc_after"}, "Malformed adoption observation")
            require(observation["command"] == {"kind": "enable_economic_competition"}, "Unexpected adoption command")
            require(all(finite(observation[k], True) for k in ("price_pc", "pc_before", "pc_after")), "Invalid adoption finances")
            require(observation["pc_after"] <= observation["pc_before"], "Adoption cannot grant political capital")
            adoption += 1
        elif kind == "budget_renewal":
            require(set(action) == base | {"commands", "prices_pc"}, "Malformed budget action")
            commands, prices = action["commands"], action["prices_pc"]
            require(type(commands) is list and commands and type(prices) is list and len(commands) == len(prices), "Missing renewal commands/prices")
            require(all(finite(price, True) for price in prices), "Invalid renewal price")
            for command in commands:
                require(type(command) is dict and len(command) == 1, "Malformed native renewal command")
                variant, payload = next(iter(command.items()))
                require(variant in ("SetAnnualBudget", "SetProgramBudget") and type(payload) is dict, "Unapproved command in stability workload")
                required = {"nation", "fiscal_year", "allocations"} | ({"departments"} if variant == "SetProgramBudget" else set())
                require(set(payload) == required and payload["nation"] in allowed_players - {None}, "Malformed renewal payload")
                require(type(payload["fiscal_year"]) is int and payload["fiscal_year"] == date.year, "Wrong fiscal year")
                require(type(payload["allocations"]) is list and len(payload["allocations"]) == 10 and all(finite(x, True) for x in payload["allocations"]), "Invalid original allocations")
        elif kind == "event_acknowledgment":
            require(set(action) == base | {"reason"} and type(action["reason"]) is str and action["reason"], "Missing native interruption reason")
        elif kind == "continuation":
            require(set(action) == base | {"action"}, "Malformed continuation record")
            command = action["action"]
            require(type(command) is dict and command.get("kind") == "continue_campaign", "Unapproved continuation command")
            mode = command.get("action")
            keys = {"kind", "action", "date", "player"} | ({"target"} if mode in ("successor", "observe") else set())
            require(set(command) == keys and command["player"] in allowed_players and type(command["date"]) is str and command["date"], "Malformed continuation identity")
            if mode == "beyond_2035":
                require(full and date == expected_end, "Sandbox continuation before full horizon")
                sandbox_actions += 1
            elif mode == "successor":
                require(cell["country"] == "USSR" and command["player"] == "USSR" and command["target"] == "Russia", "Unapproved successor transition")
            else:
                require(mode == "observe" and command["target"] is None, "Unapproved observer transition")
        else:
            raise InvalidEvidence("Unknown stability action kind")
    require(adoption == 1 and sandbox_actions == (1 if full else 0), "Missing/duplicate adoption or full-horizon continuation")
    provenance = result["provenance"]
    require(type(provenance) is dict, "Missing native provenance")
    expected_request = native_root.parent / "request.json"
    require(logical_path(provenance.get("request_path", "")) == recorded_join(recorded_parent(recorded_native_root), "request.json"), "Wrong native request provenance")
    request_bytes = expected_request.read_bytes()
    require(type(provenance.get("request_bytes")) is int and provenance["request_bytes"] == len(request_bytes), "Wrong native request byte count")
    require(provenance.get("request_fnv64") == fnv64(request_bytes) and provenance.get("request_unchanged") is True, "Changed or incorrect native request")
    require(parse_json(request_bytes) == {"format": "spheres-stability-cell/v1", **cell, "revision": revision}, "Actual native request mismatch")
    for key in ("fingerprint_scope", "initialization", "archive_exception", "cessation_policy", "retention"):
        require(type(provenance.get(key)) is str and provenance[key], f"Missing provenance scope: {key}")
    artifacts = result["artifacts"]
    require(type(artifacts) is list and len(artifacts) == (4 if full else 2), "All final/sandbox archive pairs required for success")
    pairs, retained = {}, []
    for artifact in artifacts:
        require(type(artifact) is dict and set(artifact) == {"leg", "kind", "path", "bytes", "file_fnv64", "canonical_fnv64", "date"}, "Malformed final artifact")
        leg, kind = artifact["leg"], artifact["kind"]
        require(leg in ("uninterrupted", "resumed") and kind in (("final", "sandbox") if full else ("final",)) and (kind, leg) not in pairs, "Missing/duplicate archive leg")
        checkpoint = terminal if kind == "final" else comparisons[-1]
        require(artifact["path"] == f"{leg}/saves/{kind}.json" and artifact["date"] == checkpoint["date"], "Wrong archive save path/date")
        if retained_archives is None:
            path = owned_file(native_root, artifact["path"])
            pin = file_pin(path)
            actual_fingerprints = archive_content_pins(path)
        else:
            matches = [r for r in retained_archives if r["original_relative"] == artifact["path"]]
            require(len(matches) == 1, "Final archive missing/duplicated in restore map")
            retained_archive = verify_archive_record(native_root, recorded_native_root, matches[0])
            pin = matches[0]["original"]
            actual_fingerprints = archive_content_pins(retained_archive, compressed=True)
        canonical = actual_fingerprints["canonical"]
        require(type(artifact["bytes"]) is int and artifact["bytes"] == pin["bytes"] and pin["bytes"] > 0, "Final save size mismatch")
        require(artifact["file_fnv64"] == actual_fingerprints["file_fnv64"] and artifact["canonical_fnv64"] == actual_fingerprints["canonical_fnv64"] == checkpoint["canonical_fnv64"], "Actual archive fingerprint disagrees with artifact or checkpoint")
        require(canonical["bytes"] == checkpoint["canonical_bytes"], "Actual final archive canonical length disagrees with comparison")
        pairs[(kind, leg)] = canonical
        retained.append({"leg": leg, "kind": kind, "file": pin, "canonical": canonical})
    for kind in (("final", "sandbox") if full else ("final",)):
        require((kind, "uninterrupted") in pairs and (kind, "resumed") in pairs, "Required archive pair missing")
        require(pairs[(kind, "uninterrupted")] == pairs[(kind, "resumed")], "Archives differ beyond terminal saved_unix")
    return {"comparisons": len(schedule), "calendar_days_each_leg": days,
            "checks": checks, "actions": len(actions), "final_archives": retained,
            "actual_final_pair_sha256_equal": True}


def logical_path(value):
    require(type(value) is str and value, "Missing recorded path")
    return value.replace("\\", "/").rstrip("/")


def recorded_parent(value):
    return logical_path(value).rsplit("/", 1)[0]


def recorded_join(root, *parts):
    return "/".join([logical_path(root), *parts])


def validate_pin(pin):
    require(type(pin) is dict and set(pin) == {"path", "bytes", "sha256"}, "Malformed file pin")
    require(integer(pin["bytes"]) and type(pin["sha256"]) is str and re.fullmatch(r"[0-9a-f]{64}", pin["sha256"]), "Invalid file digest/size")
    value = pin["path"]
    require(type(value) is str and (PureWindowsPath(value).is_absolute() or PurePosixPath(value).is_absolute()), "File pin path is not absolute")


def verify_pin(path, pin, recorded_path=None):
    validate_pin(pin)
    if recorded_path is not None:
        require(logical_path(pin["path"]) == logical_path(recorded_path), "Recorded file path mismatch")
    observed = file_pin(path)
    require(all(observed[k] == pin[k] for k in ("sha256", "bytes")), f"Retained file changed: {path}")
    return observed


def verify_archive_record(root, recorded_root, record):
    required = {"original", "original_relative", "gzip", "gzip_relative", "decoded", "roundtrip_verified", "raw_removed", "restore"}
    require(type(record) is dict and set(record) == required, "Malformed archive restoration record")
    name = record["original_relative"]
    require(name in declared_archive_names(), "Unapproved native archive restore path")
    require(record["gzip_relative"] == name + ".gz", "Gzip restoration path mismatch")
    require(record["roundtrip_verified"] is True and type(record["raw_removed"]) is bool, "Archive was not verified")
    require(type(record["restore"]) is str and record["restore"], "Missing archive restore instructions")
    validate_pin(record["original"])
    require(logical_path(record["original"]["path"]) == recorded_join(recorded_root, name), "Original archive path mismatch")
    require(record["decoded"] == {k: record["original"][k] for k in ("bytes", "sha256")}, "Decoded restoration pin mismatch")
    packed = owned_file(root, record["gzip_relative"])
    verify_pin(packed, record["gzip"], recorded_join(recorded_root, name + ".gz"))
    with gzip.open(packed, "rb") as stream:
        decoded = hash_stream(stream)
    require(decoded == record["decoded"], "Retained gzip does not restore the declared original bytes")
    raw = Path(root) / name
    if raw.exists():
        verify_pin(owned_file(root, name), record["original"], recorded_join(recorded_root, name))
    else:
        require(record["raw_removed"] is True, "Unremoved original archive is missing")
    return packed


def declared_archive_names():
    return {f"{leg}/saves/{filename}" for leg in ("uninterrupted", "resumed")
            for filename in ("final.json", "sandbox.json", "failure.json", "monthly.json", "monthly.json.bak")}


def verify_retained_run(root):
    """Rehash and revalidate a completed attempt without launching native code.

    Original absolute paths remain provenance, not a requirement to review on
    the original machine. All reads use confined local relative paths; gzip
    streams are compared to original hashes without writing giant raw copies.
    A coherent failed/incomplete run remains failed, never promoted.
    """
    root = Path(root).resolve(strict=True)
    require(root.is_dir(), "Retained run is not a directory")
    freeze = read_json(owned_file(root, "freeze.json"))
    proof = read_json(owned_file(root, "result.json"))
    plan = validate_plan(read_json(owned_file(root, "plan.json")))
    require(freeze.get("format") == "spheres-stability-freeze/v1", "Unsupported freeze format")
    require(proof.get("format") == "spheres-stability-matrix-result/v1", "Unsupported matrix result")
    revision = freeze.get("candidate_revision")
    require(type(revision) is str and REVISION_RE.fullmatch(revision), "Invalid frozen candidate")
    origin = recorded_parent(freeze["frozen_plan"]["path"])
    verify_pin(root / "plan.json", freeze["frozen_plan"], recorded_join(origin, "plan.json"))
    verify_pin(root / "plan.json", freeze["plan"])
    verify_pin(root / "stability_matrix.py", freeze["frozen_harness"], recorded_join(origin, "stability_matrix.py"))
    verify_pin(root / "stability_matrix.py", freeze["harness"])
    validate_pin(freeze["binary"])
    require(freeze.get("native_test") == TEST_NAME and freeze.get("expected_compiled_revision") == revision[:12], "Frozen native execution identity mismatch")
    require(freeze.get("scope") == plan["scope"] and proof.get("scope") == plan["scope"] and proof.get("plan_id") == plan["id"], "Plan scope mismatch")
    require(proof.get("revision") == revision and type(proof.get("binary_unchanged")) is bool and type(proof.get("interrupted")) is bool, "Invalid matrix identity/completion record")
    require(freeze.get("qualification") is False and freeze.get("s25_complete") is False and proof.get("qualification") is False and proof.get("s25_complete") is False, "Unsupported certification claim")
    cells = proof.get("cells")
    require(type(cells) is list, "Missing retained cell outcomes")
    summary = coverage(plan, cells)
    require(proof.get("coverage") == summary, "Recorded matrix coverage disagrees with cell outcomes")
    computed_pass = summary["requested_plan_passed"] and proof["binary_unchanged"] and not proof["interrupted"]
    require(type(proof.get("passed")) is bool and proof["passed"] == computed_pass, "Matrix pass claim disagrees with its records")
    require([c["id"] for c in cells] == [c["id"] for c in plan["cells"][:len(cells)]], "Cells were reordered, retried, or cherry-picked")
    journal_path = owned_file(root, "journal.jsonl")
    with journal_path.open("rb") as stream:
        events = [parse_json(line) for line in stream if line.strip()]
    require(events and events[0].get("event") == "matrix_started" and events[-1].get("event") == "matrix_finished", "Missing matrix journal boundaries")
    require(events[0].get("revision") == revision and events[0].get("scope") == plan["scope"], "Journal identity mismatch")
    require(events[-1].get("coverage") == summary and events[-1].get("passed") == proof["passed"], "Journal final outcome mismatch")
    started = [e for e in events if e.get("event") == "cell_started"]
    finished = [e for e in events if e.get("event") == "cell_finished"]
    require([e.get("id") for e in started] == [c["id"] for c in cells] == [e.get("id") for e in finished], "Missing/duplicate native attempt journal rows")
    expected_files = {"freeze.json", "result.json", "plan.json", "stability_matrix.py", "journal.jsonl"}
    verified_cells, archives = [], []
    for cell, outcome, begin, finish in zip(plan["cells"], cells, started, finished):
        prefix = "cells/" + cell["id"]
        cell_root = root / prefix
        native_root = cell_root / "native"
        native_origin = recorded_join(origin, prefix, "native")
        local_files = outcome.get("files")
        require(type(local_files) is list and local_files, "Missing cell file ledger")
        seen = set()
        for entry in local_files:
            require(type(entry) is dict and set(entry) == {"relative", "path", "bytes", "sha256"}, "Malformed retained file ledger entry")
            relative = entry["relative"]
            require(relative not in seen, "Duplicate retained file")
            seen.add(relative)
            path = owned_file(cell_root, relative)
            verify_pin(path, {k: entry[k] for k in ("path", "bytes", "sha256")}, recorded_join(origin, prefix, relative))
            expected_files.add(prefix + "/" + relative)
        require({"stdout.log", "stderr.log", "execution.json", "archive-manifest.json", "request.json"} <= seen, "Cell execution evidence missing")
        verdict_path = owned_file(root, prefix + "/verdict.json")
        require(read_json(verdict_path) == outcome, "Cell verdict differs from matrix result")
        expected_files.add(prefix + "/verdict.json")
        request_path = cell_root / "request.json"
        require(read_json(request_path) == {"format": "spheres-stability-cell/v1", **cell, "revision": revision}, "Frozen cell request mismatch")
        verify_pin(request_path, begin["request"], recorded_join(origin, prefix, "request.json"))
        execution = read_json(cell_root / "execution.json")
        require(execution.get("args") == [freeze["binary"]["path"], "--exact", TEST_NAME, "--ignored", "--nocapture", "--test-threads=1"], "Wrong native test invocation")
        require(logical_path(execution.get("cwd")) == recorded_join(origin, prefix), "Wrong native working directory")
        env = execution.get("environment")
        require(type(env) is dict and set(env) == {"SPHERES_S25_REQUEST", "SPHERES_S25_OUT"}, "Wrong native environment record")
        require(logical_path(env["SPHERES_S25_REQUEST"]) == recorded_join(origin, prefix, "request.json") and logical_path(env["SPHERES_S25_OUT"]) == native_origin, "Wrong native request/output isolation")
        require(finite(execution.get("elapsed_seconds"), True), "Invalid elapsed execution time")
        if outcome["passed"]:
            require(execution.get("binary_before") == freeze["binary"] == execution.get("binary_after"), "Binary drift in a passing cell")
            require(type(execution.get("exit_code")) is int and execution["exit_code"] == 0, "Passing native test did not exit zero")
        require(finish.get("passed") is outcome["passed"] and finish.get("exit_code") == execution.get("exit_code") and finish.get("failure") == outcome.get("failure"), "Journal cell outcome mismatch")
        archive_manifest = read_json(cell_root / "archive-manifest.json")
        records = outcome.get("archives")
        require(archive_manifest == {"format": "spheres-stability-archives/v1", "archives": records} and type(records) is list, "Archive manifest/outcome mismatch")
        require(len({r["original_relative"] for r in records}) == len(records), "Duplicate archive restoration map")
        for record in records:
            verify_archive_record(native_root, native_origin, record)
            prepared = [e for e in events if e.get("event") == "archive_verified_before_removal"
                        and e.get("record", {}).get("original") == record["original"]]
            removed = [e for e in events if e.get("event") == "archive_raw_removed"
                       and e.get("original_relative") == record["original_relative"]
                       and e.get("sha256") == record["original"]["sha256"]]
            # A same-state archive may have the same digest in multiple cells;
            # the complete prepared record still pins the unique absolute path.
            require(len(prepared) == 1 and prepared[0]["record"] == {**record, "raw_removed": False}, "Archive lacked pre-removal restoration evidence")
            if record["raw_removed"]:
                require(removed, "Missing archive removal journal record")
            if (native_root / record["original_relative"]).exists():
                expected_files.add(prefix + "/native/" + record["original_relative"])
            archives.append({"relative": prefix + "/native/" + record["original_relative"], "record": record})
        if outcome["passed"]:
            require(outcome.get("failure") is None and "archive_failure" not in outcome, "Passing cell retains failure")
            require(outcome.get("test_execution") == validate_test_log(cell_root / "stdout.log"), "Native executed-test summary mismatch")
            result_path = owned_file(native_root, "result.json")
            verify_pin(result_path, outcome["native_result"], recorded_join(native_origin, "result.json"))
            validated = validate_native_result(read_json(result_path), cell, revision, native_root, records, native_origin)
            require(validated == outcome.get("native_validation"), "Replayed native validation differs from recorded result")
        verified_cells.append({"id": cell["id"], "passed": outcome["passed"], "files_verified": len(local_files), "archives_verified": len(records)})
    if (root / "restoration.json").exists():
        restored = read_json(owned_file(root, "restoration.json"))
        require(restored.get("format") == "spheres-stability-restoration/v1" and restored.get("qualification") is False, "Unknown restoration sidecar")
        expected_files.add("restoration.json")
    actual_files = set()
    for path in root.rglob("*"):
        require(not path.is_symlink(), "Symlink in retained run")
        if path.is_file():
            relative = path.relative_to(root).as_posix()
            owned_file(root, relative)
            actual_files.add(relative)
    require(actual_files == expected_files, "Unlisted or missing retained artifacts")
    return {"format": "spheres-stability-retained-verification/v1", "integrity_verified": True,
            "native_reexecuted": False, "source": str(root), "recorded_origin": origin,
            "candidate_revision": revision, "frozen_harness_sha256": freeze["frozen_harness"]["sha256"],
            "verifier": file_pin(Path(__file__)), "binary_reexecuted_or_rebuilt": False,
            "passed": proof["passed"], "coverage": summary, "cells": verified_cells,
            "files_verified": len(actual_files), "archives_verified": len(archives),
            "qualification": False, "s25_complete": False}


def restore_retained_run(source, destination):
    """Copy a verified bundle and restore each archive into a NEW directory."""
    source = Path(source).resolve(strict=True)
    destination = Path(destination).resolve()
    require(not destination.exists(), "Restore destination already exists")
    verified = verify_retained_run(source)
    destination.mkdir(parents=True, exist_ok=False)
    restored = []
    for path in sorted(source.rglob("*")):
        if not path.is_file() or path.name == "restoration.json":
            continue
        relative = path.relative_to(source).as_posix()
        incoming = owned_file(source, relative)
        target = destination / relative
        target.parent.mkdir(parents=True, exist_ok=True)
        before = file_pin(incoming)
        with incoming.open("rb") as reader, target.open("xb") as writer:
            shutil.copyfileobj(reader, writer, length=1024 * 1024)
        verify_pin(target, before)
        require(file_pin(incoming) == before, "Source changed during restoration")
    proof = read_json(destination / "result.json")
    for outcome in proof["cells"]:
        native_root = destination / "cells" / outcome["id"] / "native"
        for record in outcome["archives"]:
            target = native_root / record["original_relative"]
            if not target.exists():
                target.parent.mkdir(parents=True, exist_ok=True)
                with gzip.open(owned_file(native_root, record["gzip_relative"]), "rb") as reader, target.open("xb") as writer:
                    shutil.copyfileobj(reader, writer, length=1024 * 1024)
            verify_pin(target, record["original"])
            restored.append({"relative": target.relative_to(destination).as_posix(),
                             "bytes": record["original"]["bytes"], "sha256": record["original"]["sha256"]})
    # Revalidate the entire restored bundle before recording completion. On any
    # error the partial NEW directory is left intact, never reported complete.
    replayed = verify_retained_run(destination)
    receipt = {"format": "spheres-stability-restoration/v1", "created_utc": utc_now(),
               "source": str(source), "destination": str(destination), "restored": restored,
               "source_integrity_verified": verified["integrity_verified"],
               "restored_integrity_verified": replayed["integrity_verified"],
               "native_reexecuted": False, "qualification": False}
    write_json_new(destination / "restoration.json", receipt)
    return receipt


def run_matrix(binary, revision, plan_path, out, timeout=None):
    require(type(revision) is str and REVISION_RE.fullmatch(revision), "Revision must be the exact lowercase 40-character commit")
    require(timeout is None or (type(timeout) in (int, float) and timeout > 0), "Timeout must be positive")
    binary = Path(binary).resolve(strict=True)
    require(binary.is_file(), "Native test binary is not a file")
    plan_path = Path(plan_path).resolve(strict=True)
    raw_plan = plan_path.read_bytes()
    plan = validate_plan(parse_json(raw_plan))
    out = Path(out).resolve()
    out.mkdir(parents=True, exist_ok=False)
    write_new(out / "plan.json", raw_plan)
    initial_pin = file_pin(binary)
    harness = Path(__file__).resolve()
    write_new(out / "stability_matrix.py", harness.read_bytes())
    freeze = {"format": "spheres-stability-freeze/v1", "created_utc": utc_now(),
              "candidate_revision": revision, "expected_compiled_revision": revision[:12],
              "binary": initial_pin, "plan": file_pin(plan_path), "frozen_plan": file_pin(out / "plan.json"),
              "harness": file_pin(harness), "frozen_harness": file_pin(out / "stability_matrix.py"),
              "native_test": TEST_NAME, "scope": plan["scope"],
              "through_semantics": "Inclusive last settled date; native end is through + one calendar day.",
              "qualification": False, "s25_complete": False}
    write_json_new(out / "freeze.json", freeze)
    outcomes, interrupted = [], False
    with (out / "journal.jsonl").open("xb") as events:
        def journal(value):
            events.write(json.dumps({"utc": utc_now(), **value}, ensure_ascii=False, allow_nan=False).encode() + b"\n")
            events.flush()
            os.fsync(events.fileno())
        journal({"event": "matrix_started", "revision": revision, "scope": plan["scope"]})
        for cell in plan["cells"]:
            if file_pin(binary) != initial_pin:
                journal({"event": "binary_changed_matrix_halted"})
                break
            cell_root = out / "cells" / cell["id"]
            cell_root.mkdir(parents=True, exist_ok=False)
            native_root = cell_root / "native"
            request = {"format": "spheres-stability-cell/v1", **cell, "revision": revision}
            request_path = cell_root / "request.json"
            write_json_new(request_path, request)
            request_pin = file_pin(request_path)
            args = [str(binary), "--exact", TEST_NAME, "--ignored", "--nocapture", "--test-threads=1"]
            supplied_env = {"SPHERES_S25_REQUEST": str(request_path), "SPHERES_S25_OUT": str(native_root)}
            env = os.environ.copy()
            # Do not inherit other cells' simulator test fixture switches.
            for key in list(env):
                if key.startswith("SPHERES_S25_"):
                    del env[key]
            env.update(supplied_env)
            execution = {"args": args, "environment": supplied_env, "cwd": str(cell_root),
                         "started_utc": utc_now(), "binary_before": file_pin(binary)}
            journal({"event": "cell_started", "id": cell["id"], "request": file_pin(request_path)})
            started = time.monotonic()
            outcome = {"id": cell["id"], "passed": False, "failure": None, "archives": []}
            process = None
            try:
                with (cell_root / "stdout.log").open("xb") as stdout, (cell_root / "stderr.log").open("xb") as stderr:
                    process = subprocess.Popen(args, cwd=cell_root, env=env, stdout=stdout, stderr=stderr,
                                               creationflags=getattr(subprocess, "CREATE_NO_WINDOW", 0))
                    try:
                        code = process.wait(timeout=timeout)
                    except (subprocess.TimeoutExpired, KeyboardInterrupt):
                        process.kill()
                        process.wait()
                        raise
                execution["exit_code"] = code
                require(file_pin(binary) == initial_pin, "Native binary changed during cell")
                require(file_pin(request_path) == request_pin, "Native request changed during cell")
                require(code == 0, f"Native cell exit code {code}")
                outcome["test_execution"] = validate_test_log(cell_root / "stdout.log")
                result_path = native_root / "result.json"
                result = read_json(result_path)
                outcome["native_validation"] = validate_native_result(result, cell, revision, native_root)
                outcome["native_result"] = file_pin(result_path)
                require(result.get("passed") is True, "Native result records failure")
                outcome["passed"] = True
            except KeyboardInterrupt:
                interrupted = True
                outcome["failure"] = "Interrupted by operator; remaining cells were not attempted."
            except Exception as error:
                outcome["failure"] = f"{type(error).__name__}: {str(error)[:2000]}"
            finally:
                execution["finished_utc"] = utc_now()
                execution["elapsed_seconds"] = time.monotonic() - started
                if process is not None:
                    execution.setdefault("exit_code", process.returncode)
                execution["binary_after"] = file_pin(binary)
                # Preserve failure reports, too. Never infer success from a
                # partial report; all original report bytes remain on disk.
                if native_root.is_dir():
                    try:
                        names = discover_campaign_archives(native_root)
                        outcome["archives"] = archive_campaign_files(native_root, names, journal, outcome["archives"])
                    except Exception as error:
                        outcome["passed"] = False
                        outcome["archive_failure"] = f"{type(error).__name__}: {str(error)[:2000]}"
                write_json_new(cell_root / "execution.json", execution)
                write_json_new(cell_root / "archive-manifest.json", {"format": "spheres-stability-archives/v1", "archives": outcome["archives"]})
                outcome["files"] = [{"relative": str(p.relative_to(cell_root)).replace("\\", "/"), **file_pin(p)}
                                    for p in sorted(cell_root.rglob("*")) if p.is_file()]
                write_json_new(cell_root / "verdict.json", outcome)
                outcomes.append(outcome)
                journal({"event": "cell_finished", "id": cell["id"], "passed": outcome["passed"],
                         "failure": outcome["failure"], "exit_code": execution.get("exit_code")})
            if interrupted or file_pin(binary) != initial_pin:
                break
        summary = coverage(plan, outcomes)
        final_pin = file_pin(binary)
        proof = {"format": "spheres-stability-matrix-result/v1", "finished_utc": utc_now(),
                 "revision": revision, "plan_id": plan["id"], "scope": plan["scope"],
                 "binary_unchanged": final_pin == initial_pin, "interrupted": interrupted,
                 "coverage": summary, "cells": outcomes,
                 "passed": summary["requested_plan_passed"] and final_pin == initial_pin and not interrupted,
                 "qualification": False, "s25_complete": False}
        journal({"event": "matrix_finished", "passed": proof["passed"], "coverage": summary})
    write_json_new(out / "result.json", proof)
    return proof


def discover_campaign_archives(root):
    # Restrict deletion/compression eligibility to the native test's fixed
    # archive locations. Unexpected files remain byte-for-byte untouched.
    root = Path(root).resolve(strict=True)
    names = []
    for leg in ("uninterrupted", "resumed"):
        for filename in ("final.json", "sandbox.json", "failure.json", "monthly.json", "monthly.json.bak"):
            name = f"{leg}/saves/{filename}"
            if (root / name).exists():
                owned_file(root, name)
                names.append(name)
    return names


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path)
    parser.add_argument("--revision")
    parser.add_argument("--plan", type=Path)
    parser.add_argument("--out", type=Path)
    parser.add_argument("--timeout-seconds", type=float)
    parser.add_argument("--verify", type=Path, help="Read-only verification of retained raw/compressed evidence; never runs native code")
    parser.add_argument("--restore", type=Path, help="With --verify: restore all original archive bytes into a NEW directory")
    args = parser.parse_args(argv)
    try:
        if args.verify is not None:
            require(all(value is None for value in (args.binary, args.revision, args.plan, args.out, args.timeout_seconds)), "Verification cannot be combined with native execution arguments")
            proof = verify_retained_run(args.verify)
            if args.restore is not None:
                proof["restoration"] = restore_retained_run(args.verify, args.restore)
            print(json.dumps(proof, indent=2))
            return 0 if proof["passed"] else 1
        require(args.restore is None, "--restore requires --verify")
        require(all(value is not None for value in (args.binary, args.revision, args.plan, args.out)), "Execution requires --binary, --revision, --plan and --out")
        proof = run_matrix(args.binary, args.revision, args.plan, args.out, args.timeout_seconds)
    except Exception as error:
        print(f"Stability runner refused: {type(error).__name__}: {error}", file=sys.stderr)
        return 2
    print(json.dumps({"passed": proof["passed"], "coverage": proof["coverage"], "output": str(args.out.resolve())}, indent=2))
    return 0 if proof["passed"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
