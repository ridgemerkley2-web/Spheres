"""Synthetic runner regression fixtures only: no native campaign proof or build."""
import copy
import gzip
import hashlib
import json
import shutil
from pathlib import Path
import tempfile
import threading
import unittest
from unittest.mock import patch

import stability_matrix as matrix

HERE = Path(__file__).resolve().parent
REV = "a" * 40


def plan(scope="pilot"):
    return matrix.read_json(HERE / f"stability-{scope}.json")


def synthetic_native_report(root, cell, revision=REV):
    """Tiny, explicit fake output: only validates orchestration, never a campaign."""
    root = Path(root)
    root.mkdir(parents=True)
    request = {"format": "spheres-stability-cell/v1", **cell, "revision": revision}
    if not (root.parent / "request.json").exists():
        matrix.write_json_new(root.parent / "request.json", request)
    request_raw = (root.parent / "request.json").read_bytes()
    end = matrix.day(cell["through"]) + matrix.dt.timedelta(days=1)
    full = cell["through"] == matrix.FULL_THROUGH
    raw = b'{"format":"spheres-campaign","version":1,"world":{"synthetic":true},"saved_unix":1}'
    canonical = raw.rsplit(b',"saved_unix":', 1)[0] + b'}'
    schedule = matrix.expected_comparisons(cell["through"])
    comparisons = [{"kind": kind, "date": date.isoformat(), "absolute_day": (date - matrix.START).days,
                    "matched": True, "canonical_bytes": len(canonical), "canonical_fnv64": matrix.fnv64(canonical),
                    "history_rows": 1, "log_rows": 0, "player": cell["country"]} for kind, date in schedule]
    artifacts = []
    for kind in (("final", "sandbox") if full else ("final",)):
        for leg in ("uninterrupted", "resumed"):
            path = f"{leg}/saves/{kind}.json"
            (root / path).parent.mkdir(parents=True, exist_ok=True)
            (root / path).write_bytes(raw)
            artifacts.append({"leg": leg, "kind": kind, "path": path, "bytes": len(raw),
                              "file_fnv64": matrix.fnv64(raw), "canonical_fnv64": matrix.fnv64(canonical),
                              "date": (end + matrix.dt.timedelta(days=kind == "sandbox")).isoformat()})
    rules = {key: True for key in ("daily_simulation", "economic_competition", "logistics_routes", "physical_logistics",
                                  "military_operations", "production_system", "manufacturing_system", "industry_rebuild", "ideology_blocs")}
    rules.update(seed=cell["seed"], ai_aggression=1.0, crisis_intensity=1.0)
    actions = [{"kind": "competition_adoption", "date": matrix.START.isoformat(), "legs": 2, "matched": True,
                "observation": {"command": {"kind": "enable_economic_competition"}, "price_pc": 2.0, "pc_before": 60.0, "pc_after": 58.0}}]
    if full:
        actions.append({"kind": "continuation", "date": end.isoformat(), "legs": 2, "matched": True,
                        "action": {"kind": "continue_campaign", "action": "beyond_2035", "date": "1 Jan 2036", "player": cell["country"]}})
    value = {"format": "spheres-stability-result/v1", **cell, "revision": revision, "compiled_revision": revision[:12],
             "passed": True, "failure": None, "scope": "Synthetic runner fixture only; no S25 or CP1 certification",
             "start_native_date": matrix.START.isoformat(), "end_native_date": end.isoformat(), "days_each_leg": (end - matrix.START).days,
             "legs": 2, "comparisons": comparisons, "actions": actions, "artifacts": artifacts, "artifact_errors": [], "initial_rules": rules,
             "checks": {"daily_invariant_checks": (end - matrix.START).days * 2, "sandbox_daily_invariant_checks": 2 if full else 0,
                        "native_validation_checks": len(comparisons) * 2, "monthly_reloads": sum(k == "monthly_after_reload" for k, _ in schedule),
                        "terminal_reloads": 2, "competition_adoptions": 2, "full_horizon_pause": True if full else None, "sandbox_continued": full},
             "provenance": {"request_path": str((root.parent / "request.json").resolve()), "request_bytes": len(request_raw),
                            "request_fnv64": matrix.fnv64(request_raw), "request_unchanged": True,
                            **{k: "Synthetic test fixture; not native proof" for k in ("fingerprint_scope", "initialization", "archive_exception", "cessation_policy", "retention")}}}
    if full:
        value["sandbox_end_native_date"] = "2036-01-02"
    return value


class PlanTests(unittest.TestCase):
    def test_committed_exact_pilot_and_full_membership(self):
        self.assertEqual(len(matrix.validate_plan(plan())["cells"]), 2)
        self.assertEqual(len(matrix.validate_plan(plan("full"))["cells"]), 24)

    def test_short_missing_duplicate_unknown_and_extra_cells_are_rejected(self):
        edits = [lambda p: p["cells"].pop(),
                 lambda p: p["cells"].append(copy.deepcopy(p["cells"][0])),
                 lambda p: p["cells"][0].update(country="Russia"),
                 lambda p: p["cells"][0].update(seed=True),
                 lambda p: p["cells"][0].update(through="2035-12-30"),
                 lambda p: p["cells"][0].update(through="2035-02-29"),
                 lambda p: p["cells"][0].update(id="../escape"),
                 lambda p: p["cells"][0].update(unreviewed=True)]
        for edit in edits:
            with self.subTest(edit=edit):
                value = plan("full")
                edit(value)
                with self.assertRaises(matrix.InvalidEvidence):
                    matrix.validate_plan(value)

    def test_distinct_ids_cannot_hide_duplicate_country_seed(self):
        value = plan("full")
        value["cells"][1].update(country=value["cells"][0]["country"], seed=value["cells"][0]["seed"])
        with self.assertRaisesRegex(matrix.InvalidEvidence, "Duplicate country"):
            matrix.validate_plan(value)

    def test_pilot_cannot_be_relabeled_full(self):
        value = plan()
        value.update(scope="full", id="s25-full-v1")
        with self.assertRaises(matrix.InvalidEvidence):
            matrix.validate_plan(value)

    def test_duplicate_json_keys_and_nonfinite_numbers_rejected(self):
        for raw in [b'{"id":1,"id":2}', b'{"x":NaN}', b'{"x":Infinity}']:
            with self.subTest(raw=raw), self.assertRaises(matrix.InvalidEvidence):
                matrix.parse_json(raw)


class TestSummaryTests(unittest.TestCase):
    GOOD = ("running 1 test\n" + "test " + matrix.TEST_NAME + " ... ok\n" +
            "test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 470 filtered out; finished in 4.00s\n")

    def validate(self, value):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "stdout.log"
            path.write_text(value, encoding="utf-8")
            return matrix.validate_test_log(path)

    def test_exact_native_test_summary(self):
        self.assertEqual(self.validate(self.GOOD)["executed"], 1)

    def test_zero_test_ignored_failed_duplicate_and_other_test_are_rejected(self):
        bad = [self.GOOD.replace("running 1 test", "running 0 tests"),
               self.GOOD.replace("1 passed", "0 passed"),
               self.GOOD.replace("0 ignored", "1 ignored"),
               self.GOOD.replace("0 failed", "1 failed"), self.GOOD * 2,
               self.GOOD.replace(matrix.TEST_NAME, "other::test"),
               self.GOOD.replace(" ... ok", " ... ignored")]
        for value in bad:
            with self.subTest(value=value), self.assertRaises(matrix.InvalidEvidence):
                self.validate(value)


class CoverageTests(unittest.TestCase):
    def test_passing_pilot_does_not_complete_full_matrix_or_qualification(self):
        p = plan()
        value = matrix.coverage(p, [{"id": c["id"], "passed": True} for c in p["cells"]])
        self.assertTrue(value["pilot_passed"])
        self.assertTrue(value["requested_plan_passed"])
        self.assertFalse(value["full_matrix_passed"])
        self.assertEqual(len(value["missing_full_cases"]), 24)
        self.assertFalse(value["qualification"])
        self.assertFalse(value["s25_complete"])

    def test_complete_matrix_still_leaves_controlled_successor_and_certification_open(self):
        p = plan("full")
        value = matrix.coverage(p, [{"id": c["id"], "passed": True} for c in p["cells"]])
        self.assertTrue(value["full_matrix_passed"])
        self.assertFalse(value["controlled_ussr_to_russia_case_passed"])
        self.assertFalse(value["qualification"])
        self.assertFalse(value["s25_complete"])

    def test_missing_failed_duplicate_unknown_results(self):
        p = plan()
        value = matrix.coverage(p, [{"id": p["cells"][0]["id"], "passed": False}])
        self.assertFalse(value["requested_plan_passed"])
        self.assertEqual(value["missing_requested"], [p["cells"][1]["id"]])
        for rows in [[{"id": "unknown", "passed": True}],
                     [{"id": p["cells"][0]["id"], "passed": True}] * 2,
                     [{"id": p["cells"][0]["id"], "passed": 1}]]:
            with self.subTest(rows=rows), self.assertRaises(matrix.InvalidEvidence):
                matrix.coverage(p, rows)


class ArchiveTests(unittest.TestCase):
    RAW = b'{"format":"spheres-campaign","version":1,"world":{"fixture":true},"saved_unix":1}'

    def test_lossless_archive_restore_hash_and_owned_raw_removal(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            path = root / "resumed" / "saves" / "final.json"
            path.parent.mkdir(parents=True)
            path.write_bytes(self.RAW)
            events = []
            rows = matrix.archive_campaign_files(root, ["resumed/saves/final.json"], events.append)
            self.assertFalse(path.exists())
            self.assertEqual(gzip.decompress(Path(str(path) + ".gz").read_bytes()), self.RAW)
            self.assertEqual(rows[0]["original"]["sha256"], hashlib.sha256(self.RAW).hexdigest())
            self.assertEqual(rows[0]["decoded"]["bytes"], len(self.RAW))
            self.assertEqual([r["event"] for r in events], ["archive_verified_before_removal", "archive_raw_removed"])

    def test_preexisting_payload_and_other_files_never_overwritten_or_deleted(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            path = root / "final.json"
            path.write_bytes(self.RAW)
            Path(str(path) + ".gz").write_bytes(b"existing")
            with self.assertRaises(FileExistsError):
                matrix.archive_campaign_files(root, ["final.json"], lambda _: None)
            self.assertEqual(path.read_bytes(), self.RAW)
            self.assertEqual(Path(str(path) + ".gz").read_bytes(), b"existing")
            (root / "result.json").write_bytes(self.RAW)
            with self.assertRaises(matrix.InvalidEvidence):
                matrix.archive_campaign_files(root, ["result.json"], lambda _: None)
            self.assertTrue((root / "result.json").exists())

    def test_path_traversal_absolute_and_non_campaign_rejected_without_removal(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root / "report.json").write_bytes(b'{"passed":true}')
            for name in ["../outside.json", str(root / "report.json"), "report.json"]:
                with self.subTest(name=name), self.assertRaises(matrix.InvalidEvidence):
                    matrix.archive_campaign_files(root, [name], lambda _: None)
            self.assertTrue((root / "report.json").exists())

    def test_partial_archive_error_leaves_source_intact(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root / "final.json").write_bytes(self.RAW)
            with patch.object(matrix, "hash_stream", side_effect=[{"bytes": len(self.RAW), "sha256": "a" * 64}, {"bytes": 0, "sha256": "b" * 64}]):
                with self.assertRaisesRegex(matrix.InvalidEvidence, "roundtrip mismatch"):
                    matrix.archive_campaign_files(root, ["final.json"], lambda _: None)
            self.assertEqual((root / "final.json").read_bytes(), self.RAW)


class NativeInvocationTests(unittest.TestCase):
    def test_existing_output_refused_before_process_or_modification(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            binary = root / "synthetic-binary"
            binary.write_bytes(b"not executed")
            out = root / "existing"
            out.mkdir()
            sentinel = out / "keep"
            sentinel.write_bytes(b"untouched")
            with patch.object(matrix.subprocess, "Popen") as spawn, self.assertRaises(FileExistsError):
                matrix.run_matrix(binary, REV, HERE / "stability-pilot.json", out)
            spawn.assert_not_called()
            self.assertEqual(sentinel.read_bytes(), b"untouched")

    def test_zero_tests_retained_and_exact_command_and_identity_frozen(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            binary = root / "synthetic-binary"
            binary.write_bytes(b"not executed")
            out = root / "new"
            invocations = []
            class FakeProcess:
                def __init__(self, args, **kwargs):
                    invocations.append((args, kwargs["env"]))
                    kwargs["stdout"].write(b"running 0 tests\ntest result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s\n")
                    self.returncode = 0
                def wait(self, timeout=None):
                    return self.returncode
            with patch.object(matrix.subprocess, "Popen", FakeProcess):
                value = matrix.run_matrix(binary, REV, HERE / "stability-pilot.json", out)
            self.assertFalse(value["passed"])
            self.assertEqual(value["coverage"]["attempted"], 2)
            self.assertEqual(value["coverage"]["passed"], 0)
            for args, env in invocations:
                self.assertEqual(args[1:], ["--exact", matrix.TEST_NAME, "--ignored", "--nocapture", "--test-threads=1"])
                self.assertTrue(Path(env["SPHERES_S25_REQUEST"]).is_absolute())
                self.assertFalse(Path(env["SPHERES_S25_OUT"]).exists())
            frozen = matrix.read_json(out / "freeze.json")
            self.assertEqual(frozen["candidate_revision"], REV)
            self.assertEqual(frozen["binary"]["sha256"], hashlib.sha256(b"not executed").hexdigest())
            self.assertEqual((out / "plan.json").read_bytes(), (HERE / "stability-pilot.json").read_bytes())
            self.assertEqual(len(list((out / "cells").glob("*/stdout.log"))), 2)

    def test_binary_drift_halts_remaining_cells_and_retains_failed_attempt(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            binary = root / "synthetic-binary"
            binary.write_bytes(b"initial synthetic bytes")
            class FakeProcess:
                def __init__(self, args, **kwargs):
                    self.returncode = 0
                    binary.write_bytes(b"changed synthetic bytes")
                def wait(self, timeout=None):
                    return self.returncode
            with patch.object(matrix.subprocess, "Popen", FakeProcess):
                value = matrix.run_matrix(binary, REV, HERE / "stability-pilot.json", root / "new")
            self.assertFalse(value["passed"])
            self.assertFalse(value["binary_unchanged"])
            self.assertEqual(value["coverage"]["attempted"], 1)
            self.assertEqual(len(value["coverage"]["missing_requested"]), 1)


class MatrixLifecycleTests(unittest.TestCase):
    def run_synthetic(self, fail_first=False):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            binary = root / "synthetic-binary"
            binary.write_bytes(b"unit fixture, no executable launched")
            calls = []
            class FakeProcess:
                def __init__(self, args, **kwargs):
                    request = matrix.read_json(kwargs["env"]["SPHERES_S25_REQUEST"])
                    cell = {k: request[k] for k in ("id", "country", "seed", "through")}
                    native = Path(kwargs["env"]["SPHERES_S25_OUT"])
                    value = synthetic_native_report(native, cell)
                    matrix.write_json_new(native / "result.json", value)
                    calls.append(request["id"])
                    self.returncode = 101 if fail_first and len(calls) == 1 else 0
                    kwargs["stdout"].write(TestSummaryTests.GOOD.encode())
                def wait(self, timeout=None):
                    return self.returncode
            with patch.object(matrix.subprocess, "Popen", FakeProcess):
                value = matrix.run_matrix(binary, REV, HERE / "stability-pilot.json", root / "output")
            self.assertEqual(len(calls), 2)
            self.assertEqual(value["coverage"]["passed"], 1 if fail_first else 2)
            self.assertEqual(value["passed"], not fail_first)
            self.assertFalse(value["qualification"])
            self.assertEqual(matrix.read_json(root / "output/result.json"), value)
            for outcome in value["cells"]:
                self.assertEqual(len(outcome["archives"]), 2)
                self.assertTrue(all(row["raw_removed"] and row["roundtrip_verified"] for row in outcome["archives"]))
                native = root / "output/cells" / outcome["id"] / "native"
                self.assertTrue((native / "result.json").exists())
                self.assertFalse((native / "uninterrupted/saves/final.json").exists())
                self.assertTrue((native / "uninterrupted/saves/final.json.gz").exists())
                self.assertEqual(matrix.read_json(native / "result.json")["id"], outcome["id"])
            if fail_first:
                self.assertIn("exit code 101", value["cells"][0]["failure"])
                self.assertTrue(value["cells"][1]["passed"])

    def test_passing_mocked_pilot_archives_outputs_without_native_execution(self):
        self.run_synthetic()

    def test_failed_attempt_retained_and_next_declared_cell_runs(self):
        self.run_synthetic(fail_first=True)


class CanonicalArchiveTests(unittest.TestCase):
    def test_only_terminal_timestamp_normalized_in_bounded_stream(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "final.json"
            body = b'{"format":"spheres-campaign","version":1,"world":{"text":"saved_unix:23","padding":"' + b'a' * (1024 * 1024 + 45) + b'"}'
            path.write_bytes(body + b',"saved_unix":1}')
            first = matrix.canonical_archive_pin(path)
            path.write_bytes(body + b',"saved_unix":999999}')
            self.assertEqual(first, matrix.canonical_archive_pin(path))
            self.assertEqual(first["sha256"], hashlib.sha256(body + b'}').hexdigest())
            path.write_bytes(body.replace(b'saved_unix:23', b'saved_unix:24') + b',"saved_unix":1}')
            self.assertNotEqual(first, matrix.canonical_archive_pin(path))

    def test_noncanonical_timestamp_cannot_be_silently_ignored(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "final.json"
            for value in [b'{"saved_unix":1,"world":{}}', b'{"world":{},"saved_unix":01}',
                          b'{"world":{},"saved_unix":-1}', b'{"world":{},"saved_unix":18446744073709551616}']:
                path.write_bytes(value)
                with self.subTest(value=value), self.assertRaises(matrix.InvalidEvidence):
                    matrix.canonical_archive_pin(path)


class NativeEvidenceTests(unittest.TestCase):
    def validate_case(self, scope="pilot", edit=None):
        with tempfile.TemporaryDirectory() as tmp:
            cell = plan(scope)["cells"][0]
            root = Path(tmp) / "native"
            value = synthetic_native_report(root, cell)
            if edit:
                edit(value, root)
            return matrix.validate_native_result(value, cell, REV, root)

    def test_synthetic_pilot_and_full_schema_accepted(self):
        self.assertEqual(self.validate_case()["calendar_days_each_leg"], 398)
        value = self.validate_case("full")
        self.assertEqual(value["calendar_days_each_leg"], 16801)
        self.assertEqual(len(value["final_archives"]), 4)

    def test_exact_calendar_schedule_includes_adjacent_cutoff_and_terminal(self):
        rows = matrix.expected_comparisons(matrix.FULL_THROUGH)
        for date in ("2026-09-07", "2026-09-08", "2035-12-31", "2036-01-01"):
            self.assertIn(("mandatory", matrix.day(date)), rows)
        self.assertEqual(rows[-3:], [("terminal", matrix.day("2036-01-01")), ("terminal_reload", matrix.day("2036-01-01")), ("sandbox_continuation", matrix.day("2036-01-02"))])
        self.assertNotIn(("day_after_reload", matrix.day("2036-01-02")), rows)

    def test_missing_duplicate_unknown_short_or_mismatched_reports_rejected(self):
        edits = [lambda r, _: r.update(country="Japan"),
                 lambda r, _: r.update(seed=True),
                 lambda r, _: r.update(revision="b" * 40),
                 lambda r, _: r.update(compiled_revision=REV[:12] + "-modified"),
                 lambda r, _: r.update(passed=False),
                 lambda r, _: r.update(passed=1),
                 lambda r, _: r.update(failure="native failure"),
                 lambda r, _: r.update(end_native_date="1991-02-02"),
                 lambda r, _: r.update(days_each_leg=397),
                 lambda r, _: r.update(legs=1),
                 lambda r, _: r["comparisons"].pop(2),
                 lambda r, _: r["comparisons"].insert(2, r["comparisons"][1]),
                 lambda r, _: r["comparisons"][1].update(kind="optional"),
                 lambda r, _: r["comparisons"][1].update(date="1990-02-02"),
                 lambda r, _: r["comparisons"][1].update(absolute_day=32),
                 lambda r, _: r["comparisons"][1].update(matched=False),
                 lambda r, _: r["comparisons"][-1].update(canonical_fnv64="b" * 16),
                 lambda r, _: r["checks"].update(monthly_reloads=0),
                 lambda r, _: r["checks"].update(daily_invariant_checks=0),
                 lambda r, _: r["checks"].update(native_validation_checks=True),
                 lambda r, _: r["initial_rules"].update(daily_simulation=False),
                 lambda r, _: r["initial_rules"].update(resource_gates=False),
                 lambda r, _: r["actions"].clear(),
                 lambda r, _: r["actions"][0].update(legs=1),
                 lambda r, _: r["actions"][0].update(matched=False),
                 lambda r, _: r["provenance"].update(request_unchanged=False),
                 lambda r, _: r["provenance"].update(request_fnv64="0" * 16),
                 lambda r, _: r["artifacts"].pop(),
                 lambda r, _: r["artifacts"].__setitem__(1, r["artifacts"][0]),
                 lambda r, _: r["artifacts"][0].update(date="1990-01-01"),
                 lambda r, _: r["artifacts"][0].update(bytes=1),
                 lambda r, _: r["artifacts"][0].update(path="../elsewhere.json")]
        for index, edit in enumerate(edits):
            with self.subTest(index=index), self.assertRaises(matrix.InvalidEvidence):
                self.validate_case(edit=edit)

    def test_full_requires_pause_continuation_and_sandbox_archives(self):
        edits = [lambda r, _: r["checks"].update(full_horizon_pause=False),
                 lambda r, _: r["checks"].update(sandbox_continued=False),
                 lambda r, _: r.update(sandbox_end_native_date="2036-01-01"),
                 lambda r, _: r["actions"].pop(),
                 lambda r, _: r["artifacts"].pop(),
                 lambda r, _: r["artifacts"][-1].update(kind="final")]
        for index, edit in enumerate(edits):
            with self.subTest(index=index), self.assertRaises(matrix.InvalidEvidence):
                self.validate_case("full", edit=edit)

    def test_actual_archive_drift_rejected_even_if_report_claims_match(self):
        def edit(value, root):
            path = root / "resumed/saves/final.json"
            path.write_bytes(path.read_bytes().replace(b"true", b"null"))
        with self.assertRaisesRegex(matrix.InvalidEvidence, "Actual archive fingerprint"):
            self.validate_case(edit=edit)

    def test_same_length_both_leg_drift_cannot_match_stale_native_fingerprints(self):
        def edit(value, root):
            for leg in ("uninterrupted", "resumed"):
                path = root / leg / "saves/final.json"
                path.write_bytes(path.read_bytes().replace(b"true", b"null"))
        with self.assertRaisesRegex(matrix.InvalidEvidence, "Actual archive fingerprint"):
            self.validate_case(edit=edit)

    def test_missing_actual_archive_or_changed_request_rejected(self):
        def missing(value, root):
            (root / "resumed/saves/final.json").unlink()
        with self.assertRaises(FileNotFoundError):
            self.validate_case(edit=missing)
        def changed(value, root):
            path = root.parent / "request.json"
            path.write_bytes(path.read_bytes().replace(b"France", b"Brazil"))
        with self.assertRaises(matrix.InvalidEvidence):
            self.validate_case(edit=changed)


def make_retained_fixture(root, fail_first=False, scope="pilot", interrupt_first=False, **options):
    """Build a mocked completed run with real gzip files; no native execution."""
    root = Path(root)
    binary = root / "synthetic-binary"
    binary.write_bytes(b"synthetic fixture only")
    calls = []
    class FakeProcess:
        def __init__(self, args, **kwargs):
            request = matrix.read_json(kwargs["env"]["SPHERES_S25_REQUEST"])
            cell = {k: request[k] for k in ("id", "country", "seed", "through")}
            native = Path(kwargs["env"]["SPHERES_S25_OUT"])
            result = synthetic_native_report(native, cell)
            matrix.write_json_new(native / "result.json", result)
            calls.append(request["id"])
            self.returncode = 101 if fail_first and len(calls) == 1 else 0
            kwargs["stdout"].write(TestSummaryTests.GOOD.encode())
        def wait(self, timeout=None):
            if interrupt_first and len(calls) == 1 and self.returncode == 0:
                self.returncode = -9
                raise KeyboardInterrupt()
            return self.returncode
        def kill(self):
            self.returncode = -9
    out = root / "run"
    with patch.object(matrix.subprocess, "Popen", FakeProcess):
        matrix.run_matrix(binary, REV, HERE / f"stability-{scope}.json", out, **options)
    return out


class RetainedVerificationTests(unittest.TestCase):
    def test_compressed_run_reverified_without_binary_or_process_or_writes(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            out = make_retained_fixture(root)
            (root / "synthetic-binary").unlink()
            before = {str(p.relative_to(out)): matrix.file_pin(p)["sha256"] for p in out.rglob("*") if p.is_file()}
            with patch.object(matrix.subprocess, "Popen", side_effect=AssertionError("Must not launch native")):
                verified = matrix.verify_retained_run(out)
            after = {str(p.relative_to(out)): matrix.file_pin(p)["sha256"] for p in out.rglob("*") if p.is_file()}
            self.assertEqual(before, after)
            self.assertTrue(verified["integrity_verified"])
            self.assertTrue(verified["passed"])
            self.assertFalse(verified["native_reexecuted"])
            self.assertFalse(verified["qualification"])
            self.assertEqual(verified["archives_verified"], 4)

    def test_relocated_bundle_keeps_original_provenance_and_validates_locally(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            out = make_retained_fixture(root)
            moved = root / "relocated"
            shutil.copytree(out, moved)
            verified = matrix.verify_retained_run(moved)
            # The runner records a resolved origin; Windows TEMP may use an
            # 8.3 alias (RUNNER~1) for that same directory.
            self.assertEqual(verified["recorded_origin"], matrix.logical_path(str(out.resolve())))
            self.assertEqual(verified["source"], str(moved.resolve()))
            self.assertTrue(verified["passed"])

    def test_restore_new_directory_verifies_every_decoded_original_and_keeps_source(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            out = make_retained_fixture(root)
            restored = root / "restored"
            receipt = matrix.restore_retained_run(out, restored)
            self.assertTrue(receipt["restored_integrity_verified"])
            self.assertEqual(len(receipt["restored"]), 4)
            for row in receipt["restored"]:
                self.assertEqual(matrix.file_pin(restored / row["relative"])["sha256"], row["sha256"])
                self.assertFalse((out / row["relative"]).exists())
            self.assertTrue(matrix.verify_retained_run(restored)["passed"])
            with self.assertRaisesRegex(matrix.InvalidEvidence, "already exists"):
                matrix.restore_retained_run(out, restored)

    def test_failed_attempt_is_reviewable_but_never_promoted(self):
        with tempfile.TemporaryDirectory() as tmp:
            out = make_retained_fixture(Path(tmp), fail_first=True)
            verified = matrix.verify_retained_run(out)
            self.assertTrue(verified["integrity_verified"])
            self.assertFalse(verified["passed"])
            self.assertEqual(verified["coverage"]["passed"], 1)
            self.assertFalse(verified["coverage"]["requested_plan_passed"])

    def test_changed_gzip_and_unlisted_or_missing_files_refused(self):
        for mode in ("gzip", "extra", "missing"):
            with self.subTest(mode=mode), tempfile.TemporaryDirectory() as tmp:
                out = make_retained_fixture(Path(tmp))
                if mode == "gzip":
                    target = next(out.glob("cells/*/native/uninterrupted/saves/final.json.gz"))
                    target.write_bytes(target.read_bytes()[:-1] + b"x")
                elif mode == "extra":
                    (out / "unlisted.json").write_text("{}")
                else:
                    next(out.glob("cells/*/native/result.json")).unlink()
                with self.assertRaises((matrix.InvalidEvidence, FileNotFoundError)):
                    matrix.verify_retained_run(out)

    def test_recomputed_result_ledger_cannot_hide_short_native_report(self):
        with tempfile.TemporaryDirectory() as tmp:
            out = make_retained_fixture(Path(tmp))
            proof = matrix.read_json(out / "result.json")
            outcome = proof["cells"][0]
            cell = out / "cells" / outcome["id"]
            target = cell / "native/result.json"
            native = matrix.read_json(target)
            native["end_native_date"] = "1991-02-02"
            target.write_bytes(matrix.json_bytes(native))
            new_pin = matrix.file_pin(target)
            outcome["native_result"] = new_pin
            for row in outcome["files"]:
                if row["relative"] == "native/result.json":
                    row.update(new_pin)
            (cell / "verdict.json").write_bytes(matrix.json_bytes(outcome))
            (out / "result.json").write_bytes(matrix.json_bytes(proof))
            with self.assertRaisesRegex(matrix.InvalidEvidence, "Short/incorrect native date"):
                matrix.verify_retained_run(out)

    def test_cli_verification_cannot_trigger_execution_or_overwrite_output(self):
        with patch.object(matrix.subprocess, "Popen") as spawn:
            self.assertEqual(matrix.main(["--verify", "unused", "--binary", "forbidden"]), 2)
            self.assertEqual(matrix.main(["--restore", "unused"]), 2)
            spawn.assert_not_called()


class ConcurrentMatrixTests(unittest.TestCase):
    def test_jobs_bounds_rejected_before_creating_output_or_launch(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            binary = root / "binary"
            binary.write_bytes(b"synthetic only")
            for jobs in (0, 9, -1, True, 1.5, "2"):
                with self.subTest(jobs=jobs), patch.object(matrix.subprocess, "Popen") as spawn:
                    with self.assertRaisesRegex(matrix.InvalidEvidence, "Jobs"):
                        matrix.run_matrix(binary, REV, HERE / "stability-pilot.json", root / "unused", jobs=jobs)
                    spawn.assert_not_called()
                    self.assertFalse((root / "unused").exists())

    def test_two_cells_can_finish_out_of_order_but_results_and_verifier_stay_canonical(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            binary = root / "binary"
            binary.write_bytes(b"synthetic only")
            first, second = [cell["id"] for cell in plan()["cells"]]
            release_first = threading.Event()
            actual_wait = matrix.concurrent.futures.wait
            def wait_then_release(*args, **kwargs):
                result = actual_wait(*args, **kwargs)
                if result[0]:
                    release_first.set()  # second worker already journalled its finish
                return result
            class FakeProcess:
                def __init__(self, args, **kwargs):
                    request = matrix.read_json(kwargs["env"]["SPHERES_S25_REQUEST"])
                    self.identity = request["id"]
                    cell = {k: request[k] for k in ("id", "country", "seed", "through")}
                    native = Path(kwargs["env"]["SPHERES_S25_OUT"])
                    matrix.write_json_new(native / "result.json", synthetic_native_report(native, cell))
                    kwargs["stdout"].write(TestSummaryTests.GOOD.encode())
                    self.returncode = 0
                def wait(self, timeout=None):
                    if self.identity == first:
                        assert release_first.wait(10), "second independent worker did not finish"
                    return self.returncode
            out = root / "run"
            with patch.object(matrix.subprocess, "Popen", FakeProcess), patch.object(matrix.concurrent.futures, "wait", wait_then_release):
                result = matrix.run_matrix(binary, REV, HERE / "stability-pilot.json", out, jobs=2)
            self.assertTrue(result["passed"])
            self.assertEqual([c["id"] for c in result["cells"]], [first, second])
            events = [matrix.parse_json(line) for line in (out / "journal.jsonl").read_bytes().splitlines()]
            self.assertEqual([e["id"] for e in events if e["event"] == "cell_started"], [first, second])
            self.assertEqual([e["id"] for e in events if e["event"] == "cell_finished"], [second, first])
            self.assertEqual(matrix.read_json(out / "freeze.json")["jobs"], 2)
            self.assertTrue(matrix.verify_retained_run(out)["passed"])
            frozen = matrix.read_json(out / "freeze.json")
            frozen["jobs"] = 1
            (out / "freeze.json").write_bytes(matrix.json_bytes(frozen))
            with self.assertRaisesRegex(matrix.InvalidEvidence, "concurrency mismatch"):
                matrix.verify_retained_run(out)

    def test_full_plan_attempts_each_declared_cell_once_with_bounded_overlap_and_failures_retained(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            binary = root / "binary"
            binary.write_bytes(b"synthetic only")
            mutex, first_wave = threading.Lock(), threading.Event()
            active, peak, count = 0, 0, 0
            ids = []
            class FakeProcess:
                def __init__(self, args, **kwargs):
                    nonlocal active, peak, count
                    request = matrix.read_json(kwargs["env"]["SPHERES_S25_REQUEST"])
                    with mutex:
                        active += 1
                        peak = max(peak, active)
                        count += 1
                        ids.append(request["id"])
                        if count == 3:
                            first_wave.set()
                    self.returncode = 101  # deliberately failed native attempt, no retry
                def wait(self, timeout=None):
                    nonlocal active
                    assert first_wave.wait(10), "configured workers did not overlap"
                    with mutex:
                        active -= 1
                    return self.returncode
            out = root / "run"
            with patch.object(matrix.subprocess, "Popen", FakeProcess):
                result = matrix.run_matrix(binary, REV, HERE / "stability-full.json", out, jobs=3)
            self.assertEqual(peak, 3)
            self.assertEqual(active, 0)
            self.assertEqual(len(ids), 24)
            self.assertEqual(set(ids), {c["id"] for c in plan("full")["cells"]})
            self.assertEqual(result["coverage"]["attempted"], 24)
            self.assertEqual(result["coverage"]["passed"], 0)
            self.assertFalse(result["passed"])
            self.assertTrue(matrix.verify_retained_run(out)["integrity_verified"])

    def test_observed_binary_drift_stops_new_dispatch_and_retains_already_started_cells(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            binary = root / "binary"
            binary.write_bytes(b"original")
            both_started = threading.Event()
            calls = []
            class FakeProcess:
                def __init__(self, args, **kwargs):
                    calls.append(matrix.read_json(kwargs["env"]["SPHERES_S25_REQUEST"])["id"])
                    if len(calls) == 2:
                        both_started.set()
                    self.returncode = 0
                def wait(self, timeout=None):
                    assert both_started.wait(10)
                    binary.write_bytes(b"changed")
                    return self.returncode
            out = root / "run"
            with patch.object(matrix.subprocess, "Popen", FakeProcess):
                result = matrix.run_matrix(binary, REV, HERE / "stability-full.json", out, jobs=2)
            self.assertEqual(len(calls), 2)
            self.assertEqual(result["coverage"]["attempted"], 2)
            self.assertFalse(result["binary_unchanged"])
            self.assertFalse(result["passed"])
            self.assertTrue(matrix.verify_retained_run(out)["integrity_verified"])

    def test_interrupt_kills_and_reaps_owned_children_and_does_not_dispatch_remaining_plan(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            binary = root / "binary"
            binary.write_bytes(b"synthetic only")
            both_started = threading.Event()
            children = []
            class FakeProcess:
                def __init__(self, args, **kwargs):
                    self.ended = threading.Event()
                    self.returncode = None
                    self.killed = self.reaped = False
                    children.append(self)
                    if len(children) == 2:
                        both_started.set()
                def wait(self, timeout=None):
                    assert self.ended.wait(10), "owned child was not stopped"
                    self.reaped = True
                    return self.returncode
                def kill(self):
                    self.killed = True
                    self.returncode = -9
                    self.ended.set()
            def interrupt_once(*args, **kwargs):
                assert both_started.wait(10)
                raise KeyboardInterrupt()
            out = root / "run"
            with patch.object(matrix.subprocess, "Popen", FakeProcess), patch.object(matrix.concurrent.futures, "wait", interrupt_once):
                result = matrix.run_matrix(binary, REV, HERE / "stability-full.json", out, jobs=2)
            self.assertEqual(len(children), 2)
            self.assertTrue(all(child.killed and child.reaped for child in children))
            self.assertTrue(result["interrupted"])
            self.assertFalse(result["passed"])
            self.assertEqual(result["coverage"]["attempted"], 2)
            self.assertTrue(matrix.verify_retained_run(out)["integrity_verified"])

    def test_journal_rejects_oversubscription_duplicate_missing_and_premature_finishes(self):
        cells = [{"id": "a"}, {"id": "b"}, {"id": "c"}]
        def rows(sequence):
            return [{"event": "cell_" + kind, "id": identity} for kind, identity in sequence]
        good = rows([("started", "a"), ("started", "b"), ("finished", "b"),
                     ("started", "c"), ("finished", "a"), ("finished", "c")])
        matrix.validate_attempt_journal(good, cells, 2)
        bad = [good[:-1], good + [good[-1]], [good[2]] + good, good + [good[0]],
               rows([("started", "a"), ("started", "b"), ("started", "c"),
                     ("finished", "a"), ("finished", "b"), ("finished", "c")]),
               [{"event": "binary_changed_matrix_halted"}] + good]
        for events in bad:
            with self.subTest(events=events), self.assertRaises(matrix.InvalidEvidence):
                matrix.validate_attempt_journal(events, cells, 2)
        with self.assertRaises(matrix.InvalidEvidence):
            matrix.validate_attempt_journal(good, cells, 1)

    def test_legacy_serial_bundle_keeps_old_order_requirement_without_jobs_fields(self):
        with tempfile.TemporaryDirectory() as tmp:
            out = make_retained_fixture(Path(tmp))
            for name in ("freeze.json", "result.json"):
                obj = matrix.read_json(out / name)
                obj.pop("jobs")
                (out / name).write_bytes(matrix.json_bytes(obj))
            events = [matrix.parse_json(line) for line in (out / "journal.jsonl").read_bytes().splitlines()]
            events[0].pop("jobs")
            (out / "journal.jsonl").write_bytes(b"".join(matrix.json_bytes(e).replace(b"\n", b" ") + b"\n" for e in events))
            self.assertTrue(matrix.verify_retained_run(out)["passed"])
            finishes = [i for i, event in enumerate(events) if event["event"] == "cell_finished"]
            events[finishes[0]], events[finishes[1]] = events[finishes[1]], events[finishes[0]]
            (out / "journal.jsonl").write_bytes(b"".join(matrix.json_bytes(e).replace(b"\n", b" ") + b"\n" for e in events))
            with self.assertRaises(matrix.InvalidEvidence):
                matrix.verify_retained_run(out)

    def test_cli_execution_defaults_to_one_job_and_preserves_explicit_jobs(self):
        arguments = ["--binary", "native.exe", "--revision", REV, "--plan", "plan.json", "--out", "new-output"]
        for extra, expected in (([], 1), (["--jobs", "3"], 3)):
            with self.subTest(jobs=expected), patch.object(matrix, "run_matrix", return_value={"passed": True, "coverage": {}}) as run:
                self.assertEqual(matrix.main(arguments + extra), 0)
                run.assert_called_once_with(Path("native.exe"), REV, Path("plan.json"), Path("new-output"), None, expected, scratch_root=None, compress_scratch=False, min_free_bytes=None, only_cell=None, batch_sha256=None)

    def test_cli_verification_refuses_jobs_before_native_execution(self):
        with patch.object(matrix.subprocess, "Popen") as spawn:
            self.assertEqual(matrix.main(["--verify", "unused", "--jobs", "2"]), 2)
            spawn.assert_not_called()


class ResourceExecutionTests(unittest.TestCase):
    def test_scratch_success_is_lossless_offloaded_reviewable_and_restorable(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            scratch = root / "ssd"
            out = make_retained_fixture(root, scratch_root=scratch, min_free_bytes=1)
            self.assertFalse(any((scratch / "cells").iterdir()))
            self.assertTrue((scratch / "scratch-owner.json").is_file())
            checked = matrix.verify_retained_run(out)
            self.assertTrue(checked["passed"])
            self.assertEqual(checked["archives_verified"], 4)
            moved = root / "relocated"
            shutil.copytree(out, moved)
            self.assertTrue(matrix.verify_retained_run(moved)["passed"])
            restored = matrix.restore_retained_run(moved, root / "restored")
            self.assertEqual(len(restored["restored"]), 4)
            receipt = matrix.read_json(next(out.glob("cells/*/transfer.json")))
            self.assertTrue(receipt["verified_before_cleanup"])
            self.assertTrue(all(row["original"]["sha256"] == row["retained"]["sha256"] for row in receipt["files"]))

    def test_failed_and_interrupted_cells_retain_all_archives_without_promotion(self):
        for mode in ("failed", "interrupted"):
            with self.subTest(mode=mode), tempfile.TemporaryDirectory() as tmp:
                root = Path(tmp)
                out = make_retained_fixture(root, fail_first=mode == "failed", interrupt_first=mode == "interrupted",
                                            scratch_root=root / "ssd", min_free_bytes=1)
                proof = matrix.read_json(out / "result.json")
                self.assertFalse(proof["passed"])
                self.assertEqual(proof["coverage"]["attempted"], 1 if mode == "interrupted" else 2)
                self.assertTrue(matrix.verify_retained_run(out)["integrity_verified"])
                self.assertFalse(any((root / "ssd/cells").iterdir()))
                self.assertTrue(list(out.glob("cells/*/native/*/saves/*.gz")))

    def test_unsafe_existing_overlapping_roots_and_reserves_refused_without_launch(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            binary = root / "binary"
            binary.write_bytes(b"fixture")
            existing = root / "existing"
            existing.mkdir()
            sentinel = existing / "user.txt"
            sentinel.write_text("unchanged")
            choices = [(existing, root / "out", {}), (root / "same", root / "same", {}),
                       (root / "parent", root / "parent/out", {}), (root / "parent/in", root / "parent", {}),
                       (root / "scratch", root / "out", {"min_free_bytes": 0})]
            for scratch, out, extra in choices:
                with self.subTest(scratch=scratch), patch.object(matrix.subprocess, "Popen") as spawn, self.assertRaises(matrix.InvalidEvidence):
                    matrix.run_matrix(binary, REV, HERE / "stability-pilot.json", out, scratch_root=scratch, **extra)
                spawn.assert_not_called()
            self.assertEqual(sentinel.read_text(), "unchanged")

    def test_disk_reserve_stops_before_launch_and_keeps_full_requested_plan(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            with patch.object(matrix.shutil, "disk_usage", return_value=type("Usage", (), {"free": 4})()):
                out = make_retained_fixture(root, scratch_root=root / "ssd", min_free_bytes=5)
            proof = matrix.read_json(out / "result.json")
            self.assertTrue(proof["resource_halted"])
            self.assertEqual(proof["coverage"]["attempted"], 0)
            self.assertEqual(len(proof["coverage"]["missing_requested"]), 2)
            self.assertFalse(proof["passed"])
            self.assertTrue(matrix.verify_retained_run(out)["integrity_verified"])

    def test_corrupt_offload_does_not_remove_unverified_scratch_and_halts(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            original_copy = matrix.shutil.copyfileobj
            def corrupt(reader, writer, **kwargs):
                original_copy(reader, writer, **kwargs)
                writer.write(b"corrupt")
            with patch.object(matrix.shutil, "copyfileobj", corrupt):
                out = make_retained_fixture(root, scratch_root=root / "ssd", min_free_bytes=1)
            proof = matrix.read_json(out / "result.json")
            self.assertFalse(proof["passed"])
            self.assertTrue(proof["resource_halted"])
            self.assertEqual(proof["coverage"]["attempted"], 1)
            self.assertIn("retention_failure", proof["cells"][0])
            self.assertTrue(list((root / "ssd").glob("cells/*/native/*/saves/*.gz")))
            with self.assertRaises(matrix.InvalidEvidence):
                matrix.verify_retained_run(out)

    def test_cleanup_failure_keeps_verified_copy_and_remaining_scratch(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            original_unlink = Path.unlink
            def deny(path, *args, **kwargs):
                if path.name == "request.json" and "ssd" in path.parts:
                    raise PermissionError("synthetic cleanup lock")
                return original_unlink(path, *args, **kwargs)
            with patch.object(Path, "unlink", deny):
                out = make_retained_fixture(root, scratch_root=root / "ssd", min_free_bytes=1)
            proof = matrix.read_json(out / "result.json")
            self.assertFalse(proof["passed"])
            self.assertEqual(proof["coverage"]["attempted"], 1)
            self.assertTrue(list((root / "ssd").glob("cells/*/request.json")))
            self.assertTrue(matrix.verify_retained_run(out)["integrity_verified"])

    def test_transfer_source_drift_and_changed_ownership_never_delete_original(self):
        for mode in ("drift", "owner"):
            with self.subTest(mode=mode), tempfile.TemporaryDirectory() as tmp:
                root = Path(tmp)
                scratch, out = root / "ssd", root / "out"
                out.mkdir()
                setup = matrix.setup_scratch(scratch, out, False)
                source = scratch / "cells/france-1990"
                source.mkdir(parents=True)
                raw = source / "important.txt"
                raw.write_bytes(b"retained original")
                if mode == "owner":
                    (scratch / "scratch-owner.json").write_text("{}")
                copy = matrix.shutil.copyfileobj
                def drift(reader, writer, **kwargs):
                    copy(reader, writer, **kwargs)
                    raw.write_bytes(b"changed original")
                with patch.object(matrix.shutil, "copyfileobj", drift if mode == "drift" else copy), self.assertRaises(matrix.InvalidEvidence):
                    matrix.offload_cell(source, out / "cells/france-1990", scratch, setup, lambda row: None)
                self.assertTrue(raw.is_file())

    def test_live_disk_guard_kills_only_owned_child_retains_evidence_and_halts_dispatch(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            binary = root / "binary"
            binary.write_bytes(b"fixture")
            started = threading.Event()
            children = []
            class FakeProcess:
                def __init__(self, args, **kwargs):
                    self.ended = threading.Event()
                    self.returncode = None
                    self.reaped = False
                    children.append(self)
                    request = matrix.read_json(kwargs["env"]["SPHERES_S25_REQUEST"])
                    cell = {k: request[k] for k in ("id", "country", "seed", "through")}
                    native = Path(kwargs["env"]["SPHERES_S25_OUT"])
                    matrix.write_json_new(native / "result.json", synthetic_native_report(native, cell))
                    kwargs["stdout"].write(TestSummaryTests.GOOD.encode())
                    started.set()
                def wait(self, timeout=None):
                    assert self.ended.wait(10), "disk guard did not stop owned child"
                    self.reaped = True
                    return self.returncode
                def kill(self):
                    self.returncode = -9
                    self.ended.set()
            def free(_):
                return type("Usage", (), {"free": 4 if started.is_set() else 100})()
            out = root / "out"
            with patch.object(matrix.shutil, "disk_usage", free), patch.object(matrix.subprocess, "Popen", FakeProcess):
                proof = matrix.run_matrix(binary, REV, HERE / "stability-pilot.json", out,
                                          scratch_root=root / "ssd", min_free_bytes=5)
            self.assertEqual(len(children), 1)
            self.assertTrue(children[0].reaped)
            self.assertTrue(proof["resource_halted"])
            self.assertFalse(proof["passed"])
            self.assertEqual(proof["coverage"]["attempted"], 1)
            self.assertTrue(matrix.verify_retained_run(out)["integrity_verified"])

    @unittest.skipUnless(matrix.os.name == "nt", "NTFS is a Windows-only optional facility")
    def test_actual_ntfs_directory_compression_inherits_without_changing_payload(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            out = root / "out"
            out.mkdir()
            scratch = root / "ssd"
            setup = matrix.setup_scratch(scratch, out, True)
            self.assertTrue(setup["compression"]["directory_compressed_attribute"])
            child = scratch / "cells/test/native/resumed/saves/probe.json"
            child.parent.mkdir(parents=True)
            raw = b"tiny synthetic compression probe, no campaign\n" * 1024
            child.write_bytes(raw)
            self.assertTrue(child.stat().st_file_attributes & 0x800)
            self.assertEqual(child.read_bytes(), raw)

    def test_reparse_attribute_is_rejected_even_without_symlink_support(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            original = matrix.linked_path
            with patch.object(matrix, "linked_path", side_effect=lambda p: p == root or original(p)), self.assertRaises(matrix.InvalidEvidence):
                matrix.plain_path(root / "future-child")

    def test_scratch_symlink_is_rejected_before_any_delete(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            external = root / "external"
            external.mkdir()
            sentinel = external / "keep.txt"
            sentinel.write_text("keep")
            alias = root / "alias"
            try:
                alias.symlink_to(external, target_is_directory=True)
            except OSError:
                self.skipTest("Host does not allow creation of directory symlinks")
            with self.assertRaisesRegex(matrix.InvalidEvidence, "symlink/reparse"):
                matrix.plain_path(alias / "new-run")
            self.assertEqual(sentinel.read_text(), "keep")


class ShardExecutionTests(unittest.TestCase):
    BATCH = "c" * 64

    def test_arbitrary_full_plan_cell_passes_only_its_frozen_shard_and_replays(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            chosen = plan("full")["cells"][-1]["id"]
            out = make_retained_fixture(root, scope="full", only_cell=chosen, batch_sha256=self.BATCH)
            proof = matrix.read_json(out / "result.json")
            self.assertEqual([c["id"] for c in proof["cells"]], [chosen])
            self.assertTrue(proof["selected_cell_passed"])
            self.assertFalse(proof["passed"])
            self.assertFalse(proof["coverage"]["full_matrix_passed"])
            self.assertEqual(len(proof["coverage"]["missing_full_cases"]), 23)
            checked = matrix.verify_retained_run(out)
            self.assertTrue(checked["selected_cell_passed"])
            self.assertFalse(checked["passed"])
            self.assertEqual(checked["batch_sha256"], self.BATCH)
            self.assertEqual(matrix.main(["--verify", str(out)]), 0)

    def test_shard_and_scratch_together_preserve_original_request_identity(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            chosen = plan("full")["cells"][7]["id"]
            out = make_retained_fixture(root, scope="full", only_cell=chosen, batch_sha256=self.BATCH,
                                        scratch_root=root / "ssd", min_free_bytes=1)
            checked = matrix.verify_retained_run(out)
            self.assertTrue(checked["selected_cell_passed"])
            self.assertFalse(checked["passed"])
            self.assertEqual(checked["archives_verified"], 4)

    def test_selection_batch_pair_scope_and_jobs_are_fail_closed(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            binary = root / "native"
            binary.write_bytes(b"synthetic")
            chosen = plan("full")["cells"][0]["id"]
            cases = [("full", {"only_cell": chosen}), ("full", {"batch_sha256": self.BATCH}),
                     ("pilot", {"only_cell": chosen, "batch_sha256": self.BATCH}),
                     ("full", {"only_cell": "missing", "batch_sha256": self.BATCH}),
                     ("full", {"only_cell": chosen, "batch_sha256": "C" * 64}),
                     ("full", {"only_cell": chosen, "batch_sha256": self.BATCH, "jobs": 2})]
            for scope, options in cases:
                with self.subTest(options=options), patch.object(matrix.subprocess, "Popen") as spawn, self.assertRaises(matrix.InvalidEvidence):
                    matrix.run_matrix(binary, REV, HERE / f"stability-{scope}.json", root / "out", **options)
                spawn.assert_not_called()

    def test_changed_batch_and_promoted_matrix_claim_are_rejected(self):
        for mode in ("batch", "matrix"):
            with self.subTest(mode=mode), tempfile.TemporaryDirectory() as tmp:
                root = Path(tmp)
                out = make_retained_fixture(root, scope="full", only_cell=plan("full")["cells"][3]["id"], batch_sha256=self.BATCH)
                proof = matrix.read_json(out / "result.json")
                proof["batch_sha256" if mode == "batch" else "passed"] = "d" * 64 if mode == "batch" else True
                (out / "result.json").write_bytes(matrix.json_bytes(proof))
                with self.assertRaises(matrix.InvalidEvidence):
                    matrix.verify_retained_run(out)

    def test_failed_shard_never_returns_success(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            out = make_retained_fixture(root, fail_first=True, scope="full", only_cell=plan("full")["cells"][0]["id"], batch_sha256=self.BATCH)
            checked = matrix.verify_retained_run(out)
            self.assertFalse(checked["selected_cell_passed"])
            self.assertFalse(checked["passed"])
            self.assertEqual(matrix.main(["--verify", str(out)]), 1)


if __name__ == "__main__":
    unittest.main()
