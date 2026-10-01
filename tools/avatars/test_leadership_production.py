"""Institutional exports stay exact, separate and outside the party census."""
import copy
import unittest
from unittest.mock import patch

import leadership_production as production


def candidate_export():
    """Fixture for the pending native export, without changing shared snapshots."""
    data = production.load(production.ROOT / "spheres-web/data/future_candidates_2035.json")
    data["candidates"] = [row for row in data["candidates"] if row.get("party") is not None]
    source = production.load(production.ROOT / production.INSTITUTIONAL_SOURCE)
    for row in source["future_candidates"]:
        data["candidates"].append({
            "person_id": row["person"]["id"], "name": row["person"]["name"],
            "nation": row["nation"], "origin": row["origin"], "role": row["role"],
            "party": None, "institution": "tonga_civilian_institutions", "component": None,
            "appearance_seed": row["appearance_seed"], "presentation": row["presentation"],
            "fictional_biography": row["fictional_biography"],
            "eligible_from": row["eligible_from"],
            "eligible_until_exclusive": row["eligible_until_exclusive"],
            "restriction": row["restriction"],
        })
    data["candidate_count"] = len(data["candidates"])
    data["country_count"] = len({row["nation"] for row in data["candidates"]})
    data["institutional_candidate_count"] = 4
    return data


class InstitutionalCandidateTests(unittest.TestCase):
    def setUp(self):
        self.export = candidate_export()

    def inventory(self, data):
        native_load = production.load

        def load(path):
            if path == production.ROOT / "spheres-web/data/future_candidates_2035.json":
                return copy.deepcopy(data)
            return native_load(path)

        with patch.object(production, "load", load):
            return production.future_inventory(production.ROOT)

    def test_institutions_do_not_increase_party_or_component_counts(self):
        parties, institutions = production.partition_future_candidates(
            production.ROOT, self.export["candidates"])
        self.assertEqual(len(institutions), 4)
        self.assertEqual({p["role"] for p in institutions}, {
            "peoples_representative", "prime_minister", "nonelected_minister", "party_organizer"})
        self.assertTrue(all(p["party"] is None and p["component"] is None for p in institutions))
        inventory = self.inventory(self.export)
        self.assertEqual(inventory["represented_party_count"], len(production.simulation_parties(production.ROOT)))
        self.assertEqual(inventory["validated_party_candidate_count"], len(parties))
        self.assertEqual(inventory["institutional_candidate_count"], 4)
        self.assertEqual(inventory["represented_institution_count"], 1)
        self.assertEqual(inventory["available_component_templates"] + inventory["archived_component_templates"], len(parties))
        self.assertEqual(inventory["validated_candidate_count"], len(parties) + 4)
        self.assertEqual(inventory["rendered_person_count"], 0)
        organizer = next(p for p in institutions if p["role"] == "party_organizer")
        self.assertIn("Reference only", organizer["restriction"])

    def test_unknown_missing_and_duplicate_institutional_rows_are_rejected(self):
        for mode in ("unknown", "missing", "duplicate", "missing_party_key"):
            with self.subTest(mode=mode):
                data = copy.deepcopy(self.export)
                row = data["candidates"][-1]
                if mode == "unknown":
                    row["person_id"] = "fictional_unknown_institutional_person"
                elif mode == "missing":
                    data["candidates"].pop()
                elif mode == "duplicate":
                    data["candidates"].append(copy.deepcopy(row))
                else:
                    del row["party"]
                with self.assertRaisesRegex(ValueError, "Institutional candidate export"):
                    production.partition_future_candidates(production.ROOT, data["candidates"])

    def test_authored_identity_appearance_and_permissions_cannot_drift(self):
        changes = {"name": "Another person", "role": "king", "nation": "SaudiArabia",
                   "appearance_seed": "0000000000000000", "presentation": "female",
                   "fictional_biography": "Rewritten biography", "restriction": "Guaranteed appointment",
                   "eligible_from": "2026-09-07", "eligible_until_exclusive": "2037-01-01",
                   "institution": "invented_institution", "component": "invented_component",
                   "origin": "historical"}
        for key, value in changes.items():
            with self.subTest(field=key):
                rows = copy.deepcopy(self.export["candidates"])
                rows[-1][key] = value
                with self.assertRaisesRegex(ValueError, "Institutional candidate export"):
                    production.partition_future_candidates(production.ROOT, rows)

    def test_institutional_person_cannot_be_relabelled_as_party_candidate(self):
        rows = copy.deepcopy(self.export["candidates"])
        rows[-1]["party"] = "fake_tonga_party"
        rows[-1].pop("institution")
        with self.assertRaisesRegex(ValueError, "cannot masquerade"):
            production.partition_future_candidates(production.ROOT, rows)

    def test_missing_party_and_component_coverage_remain_errors(self):
        for mode in ("party", "component"):
            with self.subTest(mode=mode):
                data = copy.deepcopy(self.export)
                target = next(row for row in data["candidates"] if row.get("component"))
                if mode == "party":
                    data["candidates"] = [row for row in data["candidates"]
                                          if (row["nation"], row["party"]) != (target["nation"], target["party"])]
                else:
                    data["candidates"] = [row for row in data["candidates"]
                                          if not (row["nation"] == target["nation"] and
                                                  row["party"] == target["party"] and
                                                  row.get("component") == target["component"])]
                with self.assertRaisesRegex(ValueError, "simulation party" if mode == "party" else "coalition component"):
                    self.inventory(data)

    def test_institutional_count_is_required_and_cannot_inflate_party_count(self):
        for field in ("candidate_count", "party_count", "country_count", "institutional_candidate_count"):
            with self.subTest(field=field):
                data = copy.deepcopy(self.export)
                data[field] += 1
                with self.assertRaisesRegex(ValueError, "Reported future counts"):
                    self.inventory(data)
        del self.export["institutional_candidate_count"]
        with self.assertRaisesRegex(ValueError, "Reported future counts"):
            self.inventory(self.export)


if __name__ == "__main__":
    unittest.main()
