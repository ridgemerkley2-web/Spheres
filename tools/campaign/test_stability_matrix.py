"""Synthetic runner regression fixtures only: no native campaign proof or build."""
import copy
import gzip
import hashlib
import json
from pathlib import Path
import tempfile
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
        with self.assertRaisesRegex(matrix.InvalidEvidence, "Archives differ"):
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


if __name__ == "__main__":
    unittest.main()
