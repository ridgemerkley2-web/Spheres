"""C01 boundary checks: unknown coverage, role separation and exact work scopes."""
import copy
import unittest
from unittest.mock import patch

import campaign_census as census


class CensusTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.outputs = census.build()

    def test_unknown_and_zero_rows_cannot_close_census(self):
        summary = self.outputs["census.json"]
        self.assertFalse(summary["c01_complete"])
        self.assertFalse(summary["g2_prerequisite_satisfied"])
        for country in self.outputs["countries.json"]:
            self.assertIsNone(country["unrepresented_organization_count"])
            self.assertEqual(country["all_real_organizations_status"], "unreviewed_exhaustiveness")
        empty = {c["id"] for c in self.outputs["countries.json"] if c["party_rows"] == 0}
        self.assertTrue({"Tonga", "SaudiArabia"} <= empty)
        work = {w["id"] for w in self.outputs["work-orders.json"]}
        for country in empty:
            self.assertIn(f"C01-{country}-ORG-001", work)
            self.assertIn(f"C01-{country}-ROLE-001", work)

    def test_every_known_member_once_and_ten_record_limit(self):
        work = self.outputs["work-orders.json"]
        self.assertEqual(len(work), len({w["id"] for w in work}))
        for w in work:
            if "members" in w:
                self.assertGreater(len(w["members"]), 0)
                self.assertLessEqual(len(w["members"]), 10)
        historical = [m for w in work if w["phase"] == "known_historical_chains" for m in w["members"]]
        expected = [o["id"] for o in self.outputs["represented-organizations.json"]]
        self.assertCountEqual(historical, expected)
        future = [m for w in work if w["phase"] == "fictional_editorial_review" for m in w["members"]]
        self.assertCountEqual(future, [c["person_id"] for c in census.load(census.ROOT, "spheres-web/data/future_candidates_2035.json")["candidates"]])
        art = [m["job_id"] for w in work if w["phase"] == "historical_cartoon_jobs" for m in w["members"]]
        self.assertCountEqual(art, [j["id"] for j in census.load(census.ROOT, "spheres-web/data/leadership_production_2035.json")["historical_art_jobs"]])

    def test_source_roles_uncertainty_and_components_are_lossless(self):
        original = census.load(census.ROOT, "spheres-sim/data/party_leaders.json")
        recorded = self.outputs["roles-and-lifecycle.json"]
        by_id = {t["id"]: t for t in recorded["term_records"]}
        for p in original["parties"]:
            for term in p["terms"]:
                self.assertEqual(term, {k: by_id[term["id"]][k] for k in term})
        self.assertEqual(recorded["seed_executive_observations"], original["office_links"])
        self.assertEqual(recorded["historical_default_executive_policy"], "party_only")
        chairs = [t for t in by_id.values() if t["nation"] == "USA"]
        self.assertTrue(chairs)
        self.assertTrue(all("Chair" in t["role"] for t in chairs))
        self.assertTrue(any(t["kind"] == "co_leader" for t in by_id.values()))
        self.assertTrue(any(t.get("component") for t in by_id.values()))
        self.assertTrue(any(t["from"]["kind"] != "day" for t in by_id.values()))

    def test_closed_or_unverified_organization_does_not_become_exhaustive(self):
        rows = self.outputs["roles-and-lifecycle.json"]["lifecycle_disclosures"]
        continuation = [r for r in rows if r["kind"] == "future_editorial_continuation_disclosure"]
        self.assertEqual({r["status"] for r in continuation}, {"ceased", "unverified"})
        self.assertEqual(len(continuation), 4)
        self.assertTrue(any(r.get("bounds", {}).get("dissolved") for r in rows))
        self.assertEqual(self.outputs["census.json"]["counts"]["exhaustive_country_censuses"], 0)

    def test_existing_stale_snapshot_is_disclosed_without_promoting_art(self):
        audit = self.outputs["census.json"]["production_snapshot"]
        self.assertFalse(audit["physical_art_revalidated_this_run"])
        self.assertEqual(audit["all_declared_inputs_current"], not audit["stale_inputs"])
        self.assertTrue(audit["current_party_rows_match_snapshot_exactly"])
        self.assertTrue(audit["job_person_and_portrait_inputs_current"])

    def test_duplicate_fictional_identity_is_rejected(self):
        native_load = census.load

        def modified(root, name):
            data = native_load(root, name)
            if name == "spheres-web/data/future_candidates_2035.json":
                data["candidates"].append(copy.deepcopy(data["candidates"][0]))
            return data

        with patch.object(census, "load", modified):
            with self.assertRaisesRegex(ValueError, "Duplicate fictional person"):
                census.build()

    def test_missing_component_export_is_rejected(self):
        native_load = census.load

        def modified(root, name):
            data = native_load(root, name)
            if name == "spheres-web/data/future_candidates_2035.json":
                component = next(c["component"] for c in data["candidates"] if c.get("component"))
                data["candidates"] = [c for c in data["candidates"] if c.get("component") != component]
            return data

        with patch.object(census, "load", modified):
            with self.assertRaisesRegex(ValueError, "Future component export"):
                census.build()


if __name__ == "__main__":
    unittest.main()
