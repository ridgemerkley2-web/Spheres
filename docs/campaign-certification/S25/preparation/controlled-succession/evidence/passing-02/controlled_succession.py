"""Run/independently verify the authored paired USSR -> Russia native preflight.

Run: python controlled_succession.py --binary <test.exe> --revision <40hex> --out <NEW-dir>
Review: python controlled_succession.py --verify <retained-dir>
Only the one exact ignored native test is executed. No compilation, organic
history claim, full-matrix claim, or campaign qualification is possible here.
The existing S25 archive helpers losslessly retain each newly produced native
save as gzip with its exact restore name/hash. Verification requires no native
binary, streams those gzip payloads, recomputes raw/canonical FNV and SHA-256,
and checks actual archive identities/dates/ownership/journey and every scheduled
comparison. Original failed attempts remain failed after integrity verification.
"""
from __future__ import annotations

import argparse
import datetime as dt
import gzip
import json
import os
from pathlib import Path
from pathlib import PurePosixPath, PureWindowsPath
import re
import subprocess
import sys
import time

import stability_matrix as shared

TEST = "s25_stability_tests::s25_controlled_ussr_russia_continuity"
CASE = "controlled-ussr-russia-1990"
SETUP = {
    "recipe": "S24 N1 / campaign_journey::tests::dissolved",
    "constructor": "Game::new_fresh(1990, Some(USSR)); fresh_play_rules",
    "native_aim": "Prosperity",
    "authored_fields": [{"nation": "USSR", "field": "stability", "value": 0.0},
                        {"nation": "USSR", "field": "separatism", "value": 1.0}],
    "other_direct_world_edits": 0,
    "no_competition_adoption": True,
}
require = shared.require


def schedule():
    return [("before_collapse", "1990-01-01", "USSR"),
            ("paused_succession", "1990-01-02", "USSR"),
            ("after_russia_choice", "1990-01-02", "Russia")] + [
        (f"successor_day_{i}", f"1990-01-{i+2:02}", "Russia") for i in range(1, 8)]


def request(revision):
    require(type(revision) is str and shared.REVISION_RE.fullmatch(revision), "Full clean revision required")
    return {"format": "spheres-controlled-succession-request/v1", "id": CASE,
            "seed": 1990, "revision": revision}


def exact_pin(path, expected):
    actual = shared.file_pin(path)
    require(all(actual[k] == expected[k] for k in ("bytes", "sha256")), f"Retained file differs: {path}")


def discover_archives(root):
    # Only this controlled test's explicitly declared save locations are owned.
    # The ordinary matrix's final/monthly filename whitelist is different.
    names = [f"{leg}/saves/{slot}.json" for slot, _, _ in schedule() for leg in ("uninterrupted", "resumed")]
    names += [f"{leg}/saves/failure.json" for leg in ("uninterrupted", "resumed")]
    return [name for name in names if (Path(root) / name).exists()]


def test_log(path):
    rows = Path(path).read_text(encoding="utf-8").splitlines()
    require([r.strip() for r in rows if re.fullmatch(r"running \d+ tests?", r.strip())]
            in (["running 1 test"], ["running 1 tests"]), "Exactly one native test must execute")
    summaries = [r.strip() for r in rows if r.strip().startswith("test result:")]
    require(len(summaries) == 1 and re.fullmatch(
        r"test result: ok\. 1 passed; 0 failed; 0 ignored; 0 measured; \d+ filtered out; finished in .+",
        summaries[0]), "Native summary must prove one executed passing test")
    require(sum(r.strip() == f"test {TEST} ... ok" for r in rows) == 1,
            "Missing or duplicate exact controlled native test success")


def read_archive(path, compressed):
    opener = gzip.open if compressed else open
    with opener(path, "rb") as stream:
        # A single checkpoint is parsed at a time. Its original byte identity is
        # checked separately; JSON parsing is never the equality comparator.
        value = json.load(stream, object_pairs_hook=shared.unique_object,
                          parse_constant=shared.reject_constant)
    require(value.get("format") == "spheres-campaign" and value.get("version") == 1,
            "Expected native campaign archive")
    world = value.get("world")
    require(type(world) is dict and world.get("format") == "spheres-integrated-save"
            and world.get("version") == 1 and type(world.get("world")) is dict,
            "Controlled fresh campaign requires the retained integrated capability envelope")
    return value, world["world"]


def archive_source(root, relative, manifest):
    matching = [r for r in manifest if r.get("original_relative") == relative]
    if matching:
        require(len(matching) == 1, "Duplicate restore mapping")
        record = matching[0]
        require(record.get("roundtrip_verified") is True and record.get("gzip_relative") == relative + ".gz",
                "Invalid native restore mapping")
        payload = shared.owned_file(root, record["gzip_relative"])
        exact_pin(payload, record["gzip"])
        with gzip.open(payload, "rb") as stream:
            decoded = shared.hash_stream(stream)
        require(decoded == record["decoded"] == {k: record["original"][k] for k in ("bytes", "sha256")},
                "Restored archive bytes/hash differ")
        raw = Path(root) / relative
        if raw.exists():
            exact_pin(shared.owned_file(root, relative), record["original"])
        return payload, True, decoded
    payload = shared.owned_file(root, relative)
    return payload, False, {k: shared.file_pin(payload)[k] for k in ("bytes", "sha256")}


def archive_identity(archive, world, slot, date, player, observation, rules):
    year, month, day = map(int, date.split("-"))
    # World serializes day 1 by omission (serde first_day/is_first_day).
    # Apply that exact native default, never the requested checkpoint's day.
    require((world.get("year"), world.get("month"), world.get("day", 1)) == (year, month, day),
            f"Archive native calendar differs at {slot}")
    require(world.get("player") == player and world.get("rules") == rules, "Archive player or rules changed")
    require(archive.get("saved_date") == f"{day} Jan 1990", "Archive saved date differs")
    nations = world.get("nations")
    require(type(nations) is list, "Native nation roster missing")
    ids = [n.get("id") for n in nations]
    require(len(ids) == len(set(ids)), "Duplicate nation identity")
    by_id = {n["id"]: n for n in nations}
    before = slot == "before_collapse"
    selected = player == "Russia"
    require(by_id.get("USSR", {}).get("alive") is before, "USSR life-state mismatch")
    require(bool(by_id.get("Russia", {}).get("alive", False)) is (not before), "Russia life-state mismatch")
    require(("ussr_dissolved" in world.get("flags", [])) is (not before), "Missing/early actual dissolution flag")
    journey = archive.get("journey")
    expected_journey = {"beyond_2035": False, "observing": False, "transitions": [
        {"from": "USSR", "to": "Russia", "date": "2 Jan 1990"}] if selected else []}
    require(journey == expected_journey == observation.get("journey"), "Observer fallback or missing/duplicate actual journey")
    owners = world.get("districts")
    require(type(owners) is dict and owners, "Native property/ownership ledger absent")
    counts = {country: sum(owner == country for owner in owners.values()) for country in ("USSR", "Russia")}
    require(counts["USSR" if before else "Russia"] > 0 and counts["Russia" if before else "USSR"] == 0,
            "Territory did not pass from ceased USSR to real Russia")
    for country in ("USSR", "Russia"):
        require(observation.get(country.lower() + "_districts") == counts[country], "Recorded ownership count mismatch")
    if not before:
        require(any(g.get("nation") == "Russia" for g in world.get("governments", {}).get("states", [])),
                "Russia has no actual government")
    aims = world.get("campaign_aims", {})
    if selected:
        records = aims.get("history")
        require(aims.get("active") is None and type(records) is list and len(records) == 1
                and records[0].get("goal", {}).get("nation") == "USSR"
                and records[0]["goal"].get("aim") == "prosperity"
                and records[0]["goal"].get("completed_day") is None
                and records[0].get("ended_day") == 1 and records[0].get("outcome") == "government ended",
                "USSR aim was lost, rewritten or awarded to Russia")
    require(len(archive.get("history", [])) == observation.get("history_rows")
            and len(archive.get("log", [])) == observation.get("log_rows"), "Retained history/log counts differ")


def validate_report(report, revision, native, manifest=()):
    require(report.get("format") == "spheres-controlled-succession-result/v1"
            and report.get("id") == CASE and report.get("seed") == 1990,
            "Wrong controlled fixture identity")
    require(report.get("revision") == revision and report.get("compiled_revision") == revision[:12],
            "Wrong/modified compiled native revision")
    require(report.get("passed") is True and report.get("failure") is None and report.get("artifact_errors") == [],
            "Native controlled case is not a completed pass")
    for field, expected in (("qualification", False), ("s25_complete", False), ("organic_history", False), ("fixture", True)):
        require(report.get(field) is expected, "Controlled preparation must not claim organic history or qualification")
    require(report.get("setup") == SETUP, "Authored setup changed or is incompletely declared")
    require(report.get("start_native_date") == "1990-01-01" and report.get("end_native_date") == "1990-01-09"
            and report.get("days_each_leg") == 8 and report.get("successor_days_each_leg") == 7
            and report.get("legs") == 2, "Short/overshot or single-leg controlled proof")
    require(report.get("checks") == {"native_validation_checks": 40, "daily_invariant_checks": 16, "scheduled_reloads": 10},
            "Missing native invariants or scheduled resume boundaries")
    require(report.get("provenance", {}).get("request_unchanged") is True
            and report["provenance"].get("no_observer_fallback") is True
            and report["provenance"].get("uninterrupted_leg_never_loaded") is True,
            "Native provenance does not establish controlled uninterrupted/resumed scope")
    comparisons, observations, artifacts = (report.get(k) for k in ("comparisons", "observations", "artifacts"))
    require(type(comparisons) is list and len(comparisons) == 20 and type(observations) is list
            and len(observations) == 10 and type(artifacts) is list and len(artifacts) == 20,
            "Incomplete/duplicate boundary observations or archives")
    expected_paths = {f"{leg}/saves/{slot}.json" for slot, _, _ in schedule() for leg in ("uninterrupted", "resumed")}
    paths = [a.get("path") for a in artifacts]
    require(len(paths) == len(set(paths)) and set(paths) == expected_paths, "Missing, duplicate or unexpected native artifact")
    if manifest:
        require(len(manifest) == 20 and {r.get("original_relative") for r in manifest} == expected_paths,
                "Restore mapping omits/adds controlled archives")
    rules = report.get("initial_rules")
    require(type(rules) is dict and rules.get("seed") == 1990 and rules.get("daily_simulation") is True
            and rules.get("ideology_blocs") is True and rules.get("economic_competition", False) is False,
            "Controlled N1 rules changed")
    verified = []
    for index, (slot, date, player) in enumerate(schedule()):
        ob = observations[index]
        require((ob.get("kind"), ob.get("date"), ob.get("player")) == (slot, date, player), "Observation schedule/order drift")
        require(ob.get("ussr_alive") is (slot == "before_collapse") and ob.get("russia_alive") is (slot != "before_collapse")
                and ob.get("dissolved") is (slot != "before_collapse") and ob.get("observing") is False
                and ob.get("paused") is (slot == "paused_succession"), "Wrong life-state/pause or observer fallback")
        pair_pins = []
        for leg in ("uninterrupted", "resumed"):
            relative = f"{leg}/saves/{slot}.json"
            artifact = next(a for a in artifacts if a["path"] == relative)
            require((artifact.get("leg"), artifact.get("kind"), artifact.get("date")) == (leg, slot, date), "Artifact scope mismatch")
            path, compressed, raw_pin = archive_source(native, relative, manifest)
            pins = shared.archive_content_pins(path, compressed)
            require(raw_pin["bytes"] == artifact.get("bytes") and pins["file_fnv64"] == artifact.get("file_fnv64")
                    and pins["canonical_fnv64"] == artifact.get("canonical_fnv64"), "Actual archive does not match native fingerprint")
            archive, world = read_archive(path, compressed)
            archive_identity(archive, world, slot, date, player, ob, rules)
            del archive, world
            pair_pins.append(pins)
            verified.append({"relative": relative, **raw_pin, **pins})
        require(pair_pins[0]["canonical"] == pair_pins[1]["canonical"], "Actual complete archive pair differs")
        for offset, suffix in enumerate(("before_reload", "after_reload")):
            row = comparisons[2 * index + offset]
            require((row.get("kind"), row.get("date"), row.get("player")) == (f"{slot}_{suffix}", date, player)
                    and row.get("matched") is True and row.get("absolute_day") == (shared.day(date) - shared.START).days
                    and row.get("canonical_fnv64") == pair_pins[0]["canonical_fnv64"]
                    and row.get("canonical_bytes") == pair_pins[0]["canonical"]["bytes"]
                    and row.get("history_rows") == ob["history_rows"] and row.get("log_rows") == ob["log_rows"],
                    "Comparison report does not describe its retained full archive")
    actions = report.get("actions")
    require(type(actions) is list and [a.get("kind") for a in actions] ==
            ["collapse_day", "legal_refusals", "served_russia_choice"] + ["successor_day"] * 7, "Incomplete/extra command journal")
    require(all(a.get("legs") == 2 and a.get("matched") is True for a in actions), "Commands not applied/comparable on both legs")
    refusals = actions[1].get("observation", {})
    require(refusals.get("not_applied") is True and refusals.get("archive_unchanged") is True
            and all(type(refusals.get(k)) is str and refusals[k] for k in ("paused_turn", "invalid_successor")), "Legal refusals not retained")
    choice = actions[2].get("observation", {})
    require(choice.get("command") == {"kind": "continue_campaign", "action": "successor", "target": "Russia", "player": "USSR", "date": "2 Jan 1990"}
            and choice.get("replay_unchanged") is True and choice.get("successor_state_unchanged") is True
            and choice.get("legacy_aim_archived") is True,
            "Missing exact served Russia choice or unchanged replay/state")
    for i, action in enumerate([actions[0]] + actions[3:]):
        ob = action.get("observation", {})
        require(ob.get("from") == f"1990-01-{i+1:02}" and ob.get("to") == f"1990-01-{i+2:02}"
                and ob.get("requested_days") == 1 and ob.get("commands") == [] and "interruption" in ob
                and (ob["interruption"] is None or type(ob["interruption"]) is str), "Skipped/overshot ordinary daily step")
        if i:
            require(action.get("index") == i, "Successor day index mismatch")
    return {"controlled_pair_passed": True, "archive_pairs": 10, "scheduled_reloads": 10,
            "complete_archive_pins": verified, "qualification": False, "s25_complete": False, "organic_history": False}


def validate_request_link(root, report, revision):
    raw = shared.owned_file(root, "request.json").read_bytes()
    require(shared.parse_json(raw) == request(revision), "Frozen controlled request differs")
    require(report.get("provenance", {}).get("request_bytes") == len(raw)
            and report["provenance"].get("request_fnv64") == shared.fnv64(raw), "Native report refers to a different request")


def verify(root):
    root = Path(root).resolve(strict=True)
    receipt = shared.read_json(shared.owned_file(root, "receipt.json"))
    require(receipt.get("format") == "spheres-controlled-succession-retention/v1"
            and type(receipt.get("passed")) is bool, "Unsupported retained receipt")
    for field in ("qualification", "s25_complete", "organic_history"):
        require(receipt.get(field) is False, "Retained receipt overclaims qualification")
    files = receipt.get("files")
    require(type(files) is list and len(files) == len({r.get("relative") for r in files}), "Invalid/duplicate retained file ledger")
    actual = {p.relative_to(root).as_posix() for p in root.rglob("*") if p.is_file() and p != root / "receipt.json"}
    require(actual == {r["relative"] for r in files}, "Retained file ledger omitted or added a file")
    for row in files:
        exact_pin(shared.owned_file(root, row["relative"]), row)
    freeze = shared.read_json(shared.owned_file(root, "freeze.json"))
    revision = freeze.get("revision")
    request(revision)
    require(freeze.get("format") == "spheres-controlled-succession-freeze/v1"
            and freeze.get("native_test") == TEST and freeze.get("qualification") is False, "Wrong frozen native test/scope")
    for name in ("controlled_succession.py", "stability_matrix.py"):
        exact_pin(shared.owned_file(root, name), freeze["harnesses"][name])
    exact_pin(shared.owned_file(root, "request.json"), freeze["request"])
    execution = shared.read_json(shared.owned_file(root, "execution.json"))
    require(execution.get("binary_before") == freeze.get("binary"), "Initial executable pin differs")
    if receipt["passed"]:
        require(execution.get("binary_before") == execution.get("binary_after"), "Executable pin changed")
    expected_args = [freeze["binary"]["path"], "--exact", TEST, "--ignored", "--nocapture", "--test-threads=1"]
    require(execution.get("args") == expected_args, "Wrong native invocation")
    recorded = execution.get("cwd")
    require(type(recorded) is str, "Native working directory missing")
    path_type = PureWindowsPath if PureWindowsPath(recorded).drive else PurePosixPath
    require(path_type(recorded).is_absolute(), "Recorded native working directory must be absolute")
    require(execution.get("environment") == {"SPHERES_S25_SUCCESSION_REQUEST": str(path_type(recorded) / "request.json"),
            "SPHERES_S25_SUCCESSION_OUT": str(path_type(recorded) / "native")}, "Native request/output isolation mismatch")
    manifest = shared.read_json(shared.owned_file(root, "archive-manifest.json"))
    require(manifest.get("format") == "spheres-stability-archives/v1", "Wrong restore manifest")
    archives = manifest.get("archives")
    require(type(archives) is list and len(archives) == len({r.get("original_relative") for r in archives}), "Duplicate restore records")
    for record in archives:
        archive_source(root / "native", record["original_relative"], archives)
    proof = None
    if receipt["passed"]:
        require(execution.get("exit_code") == 0 and receipt.get("failure") is None, "Failed execution promoted to success")
        test_log(shared.owned_file(root, "stdout.log"))
        report = shared.read_json(shared.owned_file(root, "native/result.json"))
        require(report.get("provenance", {}).get("request_path") == execution["environment"]["SPHERES_S25_SUCCESSION_REQUEST"],
                "Native loaded another request path")
        validate_request_link(root, report, revision)
        proof = validate_report(report, revision, root / "native", archives)
    else:
        require(type(receipt.get("failure")) is str and receipt["failure"], "Failed attempt must retain its diagnostic")
    return {"format": "spheres-controlled-succession-verification/v1", "integrity_verified": True,
            "original_passed": receipt["passed"], "proof": proof, "native_reexecuted": False,
            "qualification": False, "s25_complete": False, "organic_history": False}


def run(binary, revision, out):
    requested = request(revision)
    binary = Path(binary).resolve(strict=True)
    require(binary.is_file(), "Compiled native test executable required")
    out = Path(out).resolve()
    out.mkdir(parents=True, exist_ok=False)
    binary_pin = shared.file_pin(binary)
    shared.write_json_new(out / "request.json", requested)
    harnesses = {}
    for source in (Path(__file__).resolve(), Path(shared.__file__).resolve()):
        shared.write_new(out / source.name, source.read_bytes())
        harnesses[source.name] = shared.file_pin(out / source.name)
    freeze = {"format": "spheres-controlled-succession-freeze/v1", "revision": revision,
              "native_test": TEST, "binary": binary_pin, "harnesses": harnesses,
              "request": shared.file_pin(out / "request.json"),
              "created_utc": shared.utc_now(), "qualification": False}
    shared.write_json_new(out / "freeze.json", freeze)
    args = [str(binary), "--exact", TEST, "--ignored", "--nocapture", "--test-threads=1"]
    supplied = {"SPHERES_S25_SUCCESSION_REQUEST": str(out / "request.json"), "SPHERES_S25_SUCCESSION_OUT": str(out / "native")}
    env = {k: v for k, v in os.environ.items() if not k.startswith("SPHERES_S25_")}
    env.update(supplied)
    execution = {"args": args, "environment": supplied, "cwd": str(out), "binary_before": binary_pin, "started_utc": shared.utc_now()}
    passed, failure, process, archives = False, None, None, []
    started = time.monotonic()
    with (out / "journal.jsonl").open("xb") as events:
        def journal(value):
            events.write(shared.json_bytes({"utc": shared.utc_now(), **value}).replace(b"\n", b" ") + b"\n")
            events.flush()
            os.fsync(events.fileno())
        try:
            journal({"event": "controlled_native_started", "revision": revision})
            with (out / "stdout.log").open("xb") as stdout, (out / "stderr.log").open("xb") as stderr:
                process = subprocess.Popen(args, cwd=out, env=env, stdout=stdout, stderr=stderr,
                                           creationflags=getattr(subprocess, "CREATE_NO_WINDOW", 0))
                execution["exit_code"] = process.wait()
            require(shared.file_pin(binary) == binary_pin, "Compiled native binary changed")
            require(execution["exit_code"] == 0, "Native controlled test failed")
            test_log(out / "stdout.log")
            report = shared.read_json(out / "native/result.json")
            exact_pin(out / "request.json", freeze["request"])
            validate_request_link(out, report, revision)
            validate_report(report, revision, out / "native")
            passed = True
        except (Exception, KeyboardInterrupt) as error:
            if process is not None and process.poll() is None:
                process.kill()
                process.wait()
            failure = f"{type(error).__name__}: {str(error)[:2000]}"
        finally:
            execution.setdefault("exit_code", process.returncode if process is not None else None)
            execution["finished_utc"] = shared.utc_now()
            execution["elapsed_seconds"] = time.monotonic() - started
            try:
                execution["binary_after"] = shared.file_pin(binary)
            except OSError:
                execution["binary_after"] = None
            try:
                if (out / "native").is_dir():
                    names = discover_archives(out / "native")
                    shared.archive_campaign_files(out / "native", names, journal, archives)
            except Exception as error:
                passed = False
                failure = f"Archive retention failed: {error}; original diagnostic: {failure}"
            if execution["binary_after"] != binary_pin:
                passed, failure = False, "Compiled binary changed"
            shared.write_json_new(out / "execution.json", execution)
            shared.write_json_new(out / "archive-manifest.json", {"format": "spheres-stability-archives/v1", "archives": archives})
            journal({"event": "finished", "passed": passed, "failure": failure})
    files = [{"relative": p.relative_to(out).as_posix(), **shared.file_pin(p)} for p in sorted(out.rglob("*")) if p.is_file()]
    shared.write_json_new(out / "receipt.json", {"format": "spheres-controlled-succession-retention/v1",
        "passed": passed, "failure": failure, "files": files, "qualification": False, "s25_complete": False, "organic_history": False})
    verified = verify(out)
    return verified


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--binary")
    parser.add_argument("--revision")
    parser.add_argument("--out")
    parser.add_argument("--verify")
    args = parser.parse_args()
    try:
        if args.verify:
            require(not any((args.binary, args.revision, args.out)), "Verification never executes a native binary")
            result = verify(args.verify)
        else:
            require(all((args.binary, args.revision, args.out)), "Run requires binary, revision and NEW out")
            result = run(args.binary, args.revision, args.out)
        print(json.dumps(result, indent=2, ensure_ascii=False))
        return 0 if result["original_passed"] else 1
    except (Exception, KeyboardInterrupt) as error:
        print(f"Controlled succession failed: {type(error).__name__}: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
