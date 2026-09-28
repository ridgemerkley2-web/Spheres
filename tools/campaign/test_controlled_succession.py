"""Synthetic verifier fixtures; these tests are not native campaign evidence."""
import copy
import json
import shutil
from pathlib import Path
import tempfile
import unittest

import controlled_succession as c

REVISION = "a" * 40


def fixture(root):
    root = Path(root)
    rules = {"seed": 1990, "daily_simulation": True, "ideology_blocs": True, "economic_competition": False}
    report = {"format": "spheres-controlled-succession-result/v1", "id": c.CASE,
              "seed": 1990, "revision": REVISION, "compiled_revision": REVISION[:12],
              "passed": True, "failure": None, "artifact_errors": [], "qualification": False,
              "s25_complete": False, "organic_history": False, "fixture": True, "setup": copy.deepcopy(c.SETUP),
              "start_native_date": "1990-01-01", "end_native_date": "1990-01-09", "days_each_leg": 8,
              "successor_days_each_leg": 7, "legs": 2, "checks": {"native_validation_checks": 40,
              "daily_invariant_checks": 16, "scheduled_reloads": 10}, "initial_rules": rules,
              "provenance": {"request_unchanged": True, "no_observer_fallback": True, "uninterrupted_leg_never_loaded": True},
              "comparisons": [], "observations": [], "artifacts": [], "actions": []}
    for slot, date, player in c.schedule():
        before = slot == "before_collapse"
        selected = player == "Russia"
        journey = {"beyond_2035": False, "observing": False, "transitions":
                   [{"from": "USSR", "to": "Russia", "date": "2 Jan 1990"}] if selected else []}
        ob = {"kind": slot, "date": date, "player": player, "ussr_alive": before,
              "russia_alive": not before, "dissolved": not before, "observing": False,
              "paused": slot == "paused_succession", "journey": journey,
              "ussr_districts": int(before), "russia_districts": int(not before), "history_rows": 1, "log_rows": 0}
        report["observations"].append(ob)
        raw = {"format": "spheres-campaign", "version": 1, "world": {"format": "spheres-integrated-save", "version": 1,
               "world": {"year": 1990, "month": 1, "day": int(date[-2:]), "player": player, "rules": rules,
               "nations": [{"id": "USSR", "alive": before, "munitions": 0.75}, {"id": "Russia", "alive": not before, "munitions": 0.5}],
               "districts": {"synthetic-district": "USSR" if before else "Russia"},
               "flags": [] if before else ["ussr_dissolved"], "governments": {"states": [{"nation": "Russia"}]},
               "campaign_aims": {"active": None, "history": [{"goal": {"nation": "USSR", "aim": "prosperity", "completed_day": None},
                                 "ended_day": 1, "outcome": "government ended"}] if selected else []} }},
               "history": [{"synthetic": True}], "log": [], "journey": journey,
               "saved_date": f"{int(date[-2:])} Jan 1990", "player": player, "saved_unix": 1}
        if before:
            del raw["world"]["world"]["day"]  # Native World omits the first day.
        for leg in ("uninterrupted", "resumed"):
            relative = f"{leg}/saves/{slot}.json"
            path = root / relative
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(json.dumps(raw, separators=(",", ":")), encoding="utf-8")
            pins = c.shared.archive_content_pins(path)
            report["artifacts"].append({"leg": leg, "kind": slot, "date": date, "path": relative,
                "bytes": path.stat().st_size, "file_fnv64": pins["file_fnv64"], "canonical_fnv64": pins["canonical_fnv64"]})
        for suffix in ("before_reload", "after_reload"):
            report["comparisons"].append({"kind": f"{slot}_{suffix}", "date": date, "player": player,
                "matched": True, "absolute_day": int(date[-2:]) - 1, "canonical_fnv64": pins["canonical_fnv64"],
                "canonical_bytes": pins["canonical"]["bytes"], "history_rows": 1, "log_rows": 0})
    report["actions"] = [{"kind": "collapse_day", "observation": {"from": "1990-01-01", "to": "1990-01-02",
                          "requested_days": 1, "commands": [], "interruption": "Authored synthetic fixture"}, "legs": 2, "matched": True},
                         {"kind": "legal_refusals", "observation": {"not_applied": True, "archive_unchanged": True,
                          "paused_turn": "Paused", "invalid_successor": "No unrelated successor"}, "legs": 2, "matched": True},
                         {"kind": "served_russia_choice", "observation": {"command": {"kind": "continue_campaign",
                          "action": "successor", "target": "Russia", "player": "USSR", "date": "2 Jan 1990"},
                          "replay_unchanged": True, "successor_state_unchanged": True, "legacy_aim_archived": True}, "legs": 2, "matched": True}]
    report["actions"] += [{"kind": "successor_day", "index": i, "observation": {"from": f"1990-01-{i+1:02}",
                          "to": f"1990-01-{i+2:02}", "requested_days": 1, "commands": [], "interruption": None},
                          "legs": 2, "matched": True} for i in range(1, 8)]
    return report


class ControlledVerifierTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.report = fixture(self.root)

    def valid(self, report=None, manifest=()):
        return c.validate_report(report or self.report, REVISION, self.root, manifest)

    def reject(self, change):
        report = copy.deepcopy(self.report)
        change(report)
        with self.assertRaises((c.shared.InvalidEvidence, KeyError, TypeError)):
            self.valid(report)

    def test_complete_synthetic_report_accepts_without_qualification(self):
        proof = self.valid()
        self.assertEqual(proof["archive_pairs"], 10)
        self.assertFalse(proof["qualification"])
        self.assertFalse(proof["organic_history"])

    def test_native_omitted_day_means_only_first_day(self):
        for index in (0, 1):
            slot, date, player = c.schedule()[index]
            archive, world = c.read_archive(self.root / f"uninterrupted/saves/{slot}.json", False)
            world.pop("day", None)
            args = (archive, world, slot, date, player, self.report["observations"][index], self.report["initial_rules"])
            if index == 0:
                c.archive_identity(*args)
            else:
                with self.assertRaisesRegex(c.shared.InvalidEvidence, "calendar"):
                    c.archive_identity(*args)
        for invalid in (None, 0, 2):
            archive, world = c.read_archive(self.root / "uninterrupted/saves/before_collapse.json", False)
            world["day"] = invalid
            with self.subTest(day=invalid), self.assertRaisesRegex(c.shared.InvalidEvidence, "calendar"):
                c.archive_identity(archive, world, "before_collapse", "1990-01-01", "USSR", self.report["observations"][0], self.report["initial_rules"])

    def test_rejects_wrong_modified_or_unpinned_build(self):
        for key, value in [("revision", "b"*40), ("compiled_revision", REVISION[:12]+"-modified"), ("seed", 7), ("id", "ordinary-ussr")]:
            with self.subTest(key=key):
                self.reject(lambda r: r.__setitem__(key, value))

    def test_rejects_short_single_leg_or_unretained_boundary(self):
        for key, value in [("days_each_leg", 7), ("successor_days_each_leg", 6), ("legs", 1), ("end_native_date", "1990-01-08")]:
            with self.subTest(key=key):
                self.reject(lambda r: r.__setitem__(key, value))
        self.reject(lambda r: r["checks"].__setitem__("scheduled_reloads", 9))
        self.reject(lambda r: r["comparisons"].pop())
        self.reject(lambda r: r["observations"].reverse())

    def test_rejects_observer_missing_transition_or_live_parent(self):
        self.reject(lambda r: r["observations"][2].__setitem__("observing", True))
        self.reject(lambda r: r["observations"][2]["journey"]["transitions"].clear())
        self.reject(lambda r: r["observations"][2].__setitem__("ussr_alive", True))
        self.reject(lambda r: r["observations"][1].__setitem__("paused", False))

    def test_rejects_authored_or_organic_scope_drift(self):
        self.reject(lambda r: r["setup"]["authored_fields"].append({"field": "treasury", "value": 99}))
        self.reject(lambda r: r["setup"].__setitem__("native_aim", "Victory"))
        for field in ("qualification", "s25_complete", "organic_history"):
            self.reject(lambda r: r.__setitem__(field, True))
        self.reject(lambda r: r["provenance"].__setitem__("request_unchanged", False))

    def test_rejects_wrong_missing_duplicate_and_traversal_artifacts(self):
        self.reject(lambda r: r["artifacts"].pop())
        self.reject(lambda r: r["artifacts"].__setitem__(1, r["artifacts"][0]))
        self.reject(lambda r: r["artifacts"][0].__setitem__("path", "../escape.json"))
        self.reject(lambda r: r["artifacts"][0].__setitem__("kind", "other"))

    def test_rejects_stale_native_fingerprints_after_both_archives_change(self):
        for leg in ("uninterrupted", "resumed"):
            path = self.root / leg / "saves/successor_day_7.json"
            raw = path.read_bytes()
            self.assertIn(b'"munitions":0.75', raw)
            path.write_bytes(raw.replace(b'"munitions":0.75', b'"munitions":0.25'))
        with self.assertRaisesRegex(c.shared.InvalidEvidence, "fingerprint"):
            self.valid()

    def test_wall_clock_difference_alone_is_accepted_after_actual_pin_update(self):
        path = self.root / "resumed/saves/successor_day_7.json"
        path.write_bytes(path.read_bytes().replace(b'"saved_unix":1}', b'"saved_unix":2}'))
        self.report["artifacts"][-1]["file_fnv64"] = c.shared.archive_content_pins(path)["file_fnv64"]
        self.valid()

    def test_rejects_bad_journal_commands_and_bypassed_refusals(self):
        self.reject(lambda r: r["actions"][2]["observation"]["command"].__setitem__("action", "observe"))
        self.reject(lambda r: r["actions"][2]["observation"].__setitem__("replay_unchanged", False))
        self.reject(lambda r: r["actions"][1]["observation"].__setitem__("archive_unchanged", False))
        self.reject(lambda r: r["actions"][3]["observation"].__setitem__("requested_days", 7))
        self.reject(lambda r: r["actions"][3]["observation"].__setitem__("commands", [{"kind": "grant_money"}]))

    def test_rejects_zero_test_or_wrong_test_log(self):
        path = self.root / "test.log"
        good = f"running 1 test\ntest {c.TEST} ... ok\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 999 filtered out; finished in 1s\n"
        path.write_text(good, encoding="utf-8")
        c.test_log(path)
        for bad in [good.replace("1 test", "0 tests"), good.replace(c.TEST, "another::test"), good+good, good.replace("1 passed", "0 passed")]:
            path.write_text(bad, encoding="utf-8")
            with self.assertRaises(c.shared.InvalidEvidence):
                c.test_log(path)

    def test_lossless_compressed_archives_remain_independently_verifiable(self):
        names = [a["path"] for a in self.report["artifacts"]]
        events = []
        manifest = c.shared.archive_campaign_files(self.root, names, events.append)
        self.assertTrue(all(not (self.root / name).exists() for name in names))
        proof = self.valid(manifest=manifest)
        self.assertEqual(len(proof["complete_archive_pins"]), 20)
        path = self.root / manifest[0]["gzip_relative"]
        path.write_bytes(path.read_bytes()[:-1])
        with self.assertRaises(c.shared.InvalidEvidence):
            self.valid(manifest=manifest)

    def test_rejects_missing_or_duplicate_restore_mapping(self):
        manifest = c.shared.archive_campaign_files(self.root, [a["path"] for a in self.report["artifacts"]], lambda _: None)
        for changed in (manifest[:-1], manifest + [manifest[0]]):
            with self.assertRaises(c.shared.InvalidEvidence):
                self.valid(manifest=changed)

    def test_controlled_discovery_retains_every_boundary_but_no_unrelated_file(self):
        (self.root / "other.json").write_text("{}")
        self.assertEqual(set(c.discover_archives(self.root)), {a["path"] for a in self.report["artifacts"]})

    def retained_fixture(self):
        """Complete SYNTHETIC run ledger, never launches the fake executable."""
        root = self.root / "retained"
        root.mkdir()
        native = root / "native"
        report = fixture(native)
        c.shared.write_json_new(root / "request.json", c.request(REVISION))
        raw = (root / "request.json").read_bytes()
        report["provenance"].update(request_path=str(root / "request.json"), request_bytes=len(raw), request_fnv64=c.shared.fnv64(raw))
        c.shared.write_json_new(native / "result.json", report)
        binary = self.root / "synthetic-not-executable.exe"
        binary.write_bytes(b"Synthetic verifier fixture only; never executed")
        harnesses = {}
        for source in (Path(c.__file__), Path(c.shared.__file__)):
            shutil.copyfile(source, root / source.name)
            harnesses[source.name] = c.shared.file_pin(root / source.name)
        pin = c.shared.file_pin(binary)
        c.shared.write_json_new(root / "freeze.json", {"format": "spheres-controlled-succession-freeze/v1",
            "revision": REVISION, "native_test": c.TEST, "binary": pin, "harnesses": harnesses,
            "request": c.shared.file_pin(root / "request.json"), "qualification": False})
        c.shared.write_json_new(root / "execution.json", {"args": [str(binary), "--exact", c.TEST, "--ignored", "--nocapture", "--test-threads=1"],
            "cwd": str(root), "environment": {"SPHERES_S25_SUCCESSION_REQUEST": str(root / "request.json"),
            "SPHERES_S25_SUCCESSION_OUT": str(native)}, "binary_before": pin, "binary_after": pin, "exit_code": 0})
        (root / "stdout.log").write_text(f"running 1 test\ntest {c.TEST} ... ok\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 999 filtered out; finished in 1s\n", encoding="utf-8")
        (root / "stderr.log").write_text("", encoding="utf-8")
        archives = c.shared.archive_campaign_files(native, c.discover_archives(native), lambda _: None)
        c.shared.write_json_new(root / "archive-manifest.json", {"format": "spheres-stability-archives/v1", "archives": archives})
        self.reledger(root)
        return root

    def reledger(self, root, passed=True):
        receipt = root / "receipt.json"
        if receipt.exists():
            receipt.unlink()  # Test-owned synthetic metadata only.
        files = [{"relative": p.relative_to(root).as_posix(), **c.shared.file_pin(p)} for p in sorted(root.rglob("*")) if p.is_file()]
        c.shared.write_json_new(receipt, {"format": "spheres-controlled-succession-retention/v1", "passed": passed,
            "failure": None if passed else "Synthetic original failure preserved", "files": files,
            "qualification": False, "s25_complete": False, "organic_history": False})

    def test_retained_verification_is_portable_and_never_promotes_failed_run(self):
        root = self.retained_fixture()
        self.assertTrue(c.verify(root)["original_passed"])
        relocated = self.root / "relocated"
        shutil.copytree(root, relocated)
        self.assertTrue(c.verify(relocated)["integrity_verified"])
        self.reledger(relocated, passed=False)
        result = c.verify(relocated)
        self.assertFalse(result["original_passed"])
        self.assertIsNone(result["proof"])
        self.assertFalse(result["native_reexecuted"])

    def test_retained_verifier_rejects_ledger_omission_and_wrong_native_environment(self):
        root = self.retained_fixture()
        (root / "unlisted.txt").write_text("not declared")
        with self.assertRaisesRegex(c.shared.InvalidEvidence, "ledger"):
            c.verify(root)
        (root / "unlisted.txt").unlink()
        execution = c.shared.read_json(root / "execution.json")
        execution["environment"]["SPHERES_S25_SUCCESSION_OUT"] = str(self.root / "outside")
        (root / "execution.json").write_bytes(c.shared.json_bytes(execution))
        self.reledger(root)
        with self.assertRaisesRegex(c.shared.InvalidEvidence, "isolation"):
            c.verify(root)

    def test_existing_output_is_never_overwritten(self):
        binary = self.root / "not-an-executable"
        binary.write_bytes(b"never executed")
        with self.assertRaises(FileExistsError):
            c.run(binary, REVISION, self.root)
        self.assertEqual(binary.read_bytes(), b"never executed")


if __name__ == "__main__":
    unittest.main()
