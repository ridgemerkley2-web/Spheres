"""Real source and negative controls for the bounded coverage-only repair."""
import copy
import hashlib
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

import resource_coverage
import sources


class CoverageTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        root = Path(__file__).resolve().parents[2]
        cls.artifact = json.loads((root / "spheres-web/data/district_resources.json").read_text(encoding="utf-8"))
        cls.roster = json.loads((root / "spheres-sim/data/districts.json").read_text(encoding="utf-8"))["nations"]
        cls.source = cls.artifact["sources"]["ds896_bauxite"]

    def test_pinned_real_source_year_units_and_every_dropped_bauxite_producer(self):
        entries, coverage = resource_coverage.build_report(self.roster, self.source)
        self.assertEqual({r["nation"]: r["value"] for r in entries["bauxite"]},
                         {"Guinea": 15800000.0, "Sierra Leone": 1430000.0})
        self.assertTrue(all(r["year"] == 1990 and r["units"] == "metric tons" for r in entries["bauxite"]))
        self.assertTrue(all(r["source_label"] == r["nation"] and r["basis"] == "outside_district_roster" for r in entries["bauxite"]))
        self.assertEqual(coverage["reviewed_commodities"], ["bauxite"])
        self.assertIn("unaudited", coverage["unreviewed_commodities"])
        self.assertNotIn("World", {r["nation"] for r in entries["bauxite"]})
        # The neighboring source year differs. Accidentally reading 1991 must
        # not pass merely because Guinea appears in that worksheet as well.
        other = sources.ds896_year(resource_coverage.WORKBOOK, 1, year="1991")
        self.assertNotEqual(other["Guinea"], entries["bauxite"][0]["value"])

    def test_report_changes_no_existing_resource_value_or_limit(self):
        original = copy.deepcopy(self.artifact)
        original.pop("unrostered_producers", None)
        original["meta"].pop("unrostered_producer_coverage", None)
        reported = resource_coverage.apply_report(copy.deepcopy(original), self.roster)
        self.assertNotIn("Guinea", reported["national"]["bauxite"])
        self.assertNotIn("Sierra Leone", reported["national"]["bauxite"])
        reported.pop("unrostered_producers")
        reported["meta"].pop("unrostered_producer_coverage")
        self.assertEqual(reported, original)

    def test_committed_reporting_matches_the_real_source(self):
        entries, coverage = resource_coverage.build_report(self.roster, self.source)
        self.assertEqual(self.artifact.get("unrostered_producers"), entries)
        self.assertEqual(self.artifact["meta"].get("unrostered_producer_coverage"), coverage)

    def test_corrupt_workbook_or_disagreed_source_pin_is_rejected(self):
        with tempfile.TemporaryDirectory() as temp:
            corrupt = Path(temp) / "corrupt.xlsx"
            corrupt.write_bytes(resource_coverage.WORKBOOK.read_bytes() + b"changed")
            with self.assertRaisesRegex(ValueError, "pinned source"):
                resource_coverage.build_report(self.roster, self.source, corrupt)
        wrong = dict(self.source, sha256="0" * 64)
        with self.assertRaisesRegex(ValueError, "source identity"):
            resource_coverage.build_report(self.roster, wrong)

    def test_wrong_sheet_or_unit_is_rejected(self):
        with patch.object(sources, "read_xlsx", return_value=(["Alumina", "Aluminum"], [["(Metric tons)"]])):
            with self.assertRaisesRegex(ValueError, "worksheet or source units"):
                resource_coverage.build_report(self.roster, self.source)
        with patch.object(sources, "read_xlsx", return_value=(["Alumina", "Bauxite"], [["(Thousand metric tons)"]])):
            with self.assertRaisesRegex(ValueError, "worksheet or source units"):
                resource_coverage.build_report(self.roster, self.source)

    def test_unknown_country_or_roster_change_requires_a_crosswalk_review(self):
        with self.assertRaisesRegex(ValueError, "unmapped"):
            resource_coverage.bauxite_rows({"Unreviewed new label": 1.0}, self.roster)
        with self.assertRaisesRegex(ValueError, "now-rostered"):
            resource_coverage.bauxite_rows({"Guinea": 15800000.0}, set(self.roster) | {"Guinea"})
        for value in [-1, float("nan"), float("inf")]:
            with self.assertRaises(ValueError):
                resource_coverage.bauxite_rows({"Guinea": value}, self.roster)
        self.assertEqual(resource_coverage.bauxite_rows({"Guinea": 0.0, "World": 100.0}, self.roster), [])

    def test_application_is_deterministic_and_idempotent(self):
        first = resource_coverage.apply_report(copy.deepcopy(self.artifact), self.roster)
        second = resource_coverage.apply_report(copy.deepcopy(first), self.roster)
        encode = lambda value: json.dumps(value, indent=1, sort_keys=True, ensure_ascii=False).encode("utf-8")
        self.assertEqual(hashlib.sha256(encode(first)).digest(), hashlib.sha256(encode(second)).digest())

    def test_bounded_report_preserves_other_coverage_markers(self):
        artifact = copy.deepcopy(self.artifact)
        untouched = [{"nation": "another reporting omission", "value": 17}]
        artifact.setdefault("unrostered_producers", {})["copper"] = untouched
        resource_coverage.apply_report(artifact, self.roster)
        self.assertEqual(artifact["unrostered_producers"]["copper"], untouched)


if __name__ == "__main__":
    unittest.main()
