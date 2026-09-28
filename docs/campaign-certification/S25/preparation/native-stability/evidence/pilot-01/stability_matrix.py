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
"""
from __future__ import annotations

import argparse
import datetime as dt
import gzip
import hashlib
import json
import math
import os
from pathlib import Path
import re
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


def canonical_archive_pin(path):
    """Hash exact archive content except its terminal wall-clock saved_unix.

    Use a bounded tail, never a parsed/duplicated 100MB JSON tree. Internal
    saved_unix strings remain significant, as do key order and all other bytes.
    """
    digest = hashlib.sha256()
    total, tail = 0, b""
    with Path(path).open("rb") as stream:
        while block := stream.read(1024 * 1024):
            pending = tail + block
            if len(pending) > 256:
                prefix, tail = pending[:-256], pending[-256:]
                digest.update(prefix)
                total += len(prefix)
            else:
                tail = pending
    match = re.fullmatch(rb'(.*),"saved_unix":(0|[1-9][0-9]*)}', tail, re.DOTALL)
    require(match is not None and int(match[2]) <= 2 ** 64 - 1, "Missing canonical terminal saved_unix timestamp")
    retained = match[1] + b"}"
    digest.update(retained)
    return {"bytes": total + len(retained), "sha256": digest.hexdigest(),
            "normalization": "Only terminal top-level saved_unix removed; every other byte preserved."}


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


def fnv64(data):
    value = 0xcbf29ce484222325
    for byte in data:
        value = ((value ^ byte) * 0x100000001b3) & ((1 << 64) - 1)
    return f"{value:016x}"


def is_fnv(value):
    return type(value) is str and re.fullmatch(r"[0-9a-f]{16}", value) is not None


def validate_native_result(result, cell, revision, native_root):
    native_root = Path(native_root).resolve(strict=True)
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
    require(Path(provenance.get("request_path", "")).resolve() == expected_request, "Wrong native request provenance")
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
        path = owned_file(native_root, artifact["path"])
        pin = file_pin(path)
        require(type(artifact["bytes"]) is int and artifact["bytes"] == pin["bytes"] and pin["bytes"] > 0, "Final save size mismatch")
        require(is_fnv(artifact["file_fnv64"]) and artifact["canonical_fnv64"] == checkpoint["canonical_fnv64"], "Final save fingerprint disagrees with terminal")
        canonical = canonical_archive_pin(path)
        require(canonical["bytes"] == checkpoint["canonical_bytes"], "Actual final archive canonical length disagrees with comparison")
        pairs[(kind, leg)] = canonical
        retained.append({"leg": leg, "kind": kind, "file": pin, "canonical": canonical})
    for kind in (("final", "sandbox") if full else ("final",)):
        require((kind, "uninterrupted") in pairs and (kind, "resumed") in pairs, "Required archive pair missing")
        require(pairs[(kind, "uninterrupted")] == pairs[(kind, "resumed")], "Archives differ beyond terminal saved_unix")
    return {"comparisons": len(schedule), "calendar_days_each_leg": days,
            "checks": checks, "actions": len(actions), "final_archives": retained,
            "actual_final_pair_sha256_equal": True}


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
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--revision", required=True)
    parser.add_argument("--plan", required=True, type=Path)
    parser.add_argument("--out", required=True, type=Path)
    parser.add_argument("--timeout-seconds", type=float)
    args = parser.parse_args(argv)
    try:
        proof = run_matrix(args.binary, args.revision, args.plan, args.out, args.timeout_seconds)
    except Exception as error:
        print(f"Stability runner refused: {type(error).__name__}: {error}", file=sys.stderr)
        return 2
    print(json.dumps({"passed": proof["passed"], "coverage": proof["coverage"], "output": str(args.out.resolve())}, indent=2))
    return 0 if proof["passed"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
