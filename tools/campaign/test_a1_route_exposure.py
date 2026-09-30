import unittest

from tools.campaign.a1_route_exposure import Exposure, roster_codes, trigger_blockers, verify_firings


def row(stage="trigger_firing", electoral=True, date=(1990, 1, 1)):
    return {"country": "Example", "date": list(date), "stage": stage,
            "government": {"electoral": electoral, "has_army": True, "settled_months": 12, "awaiting_first_election": False},
            "army": {"effective_loyalty": 0.20, "pressure": 1.0}, "discontent": {"total": 0.25},
            "thresholds": {"army": 0.35, "discontent": 0.25, "settled_months": 12, "pressure": 1.0}}


class RouteExposureTests(unittest.TestCase):
    def test_native_headline_array_must_match_firing_chronology(self):
        firing = row(date=(1991, 8, 1))
        result = {"firing_cases": [firing], "elected_coup_headlines": [
            {"month": 20, "headline": "COUP IN EXAMPLE: the Army removes the elected government."}]}
        verify_firings([firing], result)
        result["elected_coup_headlines"][0]["month"] = 21
        with self.assertRaisesRegex(ValueError, "chronology"):
            verify_firings([firing], result)

    def test_later_regime_state_does_not_erase_a_real_firing(self):
        audit = Exposure()
        audit.add(row(), 1)
        audit.add(row("ai_before_funding", False), 2)
        result = next(x for x in audit.summary({"Example"}) if x["country"] == "Example")
        self.assertEqual((result["firings"], result["electoral_at_funding"], result["non_electoral_at_funding"]), (1, 0, 1))
        self.assertEqual(result["same_month_exit_witnesses"][0]["trigger"]["line"], 1)

    def test_nonfiring_exit_is_not_invented_as_a_coup(self):
        audit = Exposure()
        trigger = row("trigger_live_conditions_inactive")
        trigger["army"]["effective_loyalty"] = 0.8
        audit.add(trigger, 1)
        audit.add(row("ai_before_funding", False), 2)
        self.assertEqual(len(audit.firings), 0)
        self.assertEqual(len(audit.countries["Example"]["same_month_exit_witnesses"]), 1)

    def test_exact_loyalty_boundary_is_not_hostile(self):
        trigger = row("trigger_live_conditions_inactive")
        trigger["army"]["effective_loyalty"] = 0.35
        self.assertEqual(trigger_blockers(trigger), ["army_not_hostile"])

    def test_grace_and_interim_are_distinct_from_live_conditions(self):
        trigger = row("trigger_unsettled_interim_or_no_army")
        trigger["government"]["settled_months"] = 11
        trigger["government"]["awaiting_first_election"] = True
        self.assertEqual(trigger_blockers(trigger), ["unsettled", "awaiting_first_election"])

    def test_rejects_duplicate_or_conflicting_observations(self):
        audit = Exposure()
        audit.add(row(), 1)
        with self.assertRaisesRegex(ValueError, "Duplicate"):
            audit.add(row(), 2)
        changed = row("trigger_pressure_not_ready")
        with self.assertRaisesRegex(ValueError, "Multiple trigger"):
            audit.add(changed, 3)

    def test_rejects_false_pass_branch_and_changed_threshold(self):
        trigger = row()
        trigger["army"]["pressure"] = 0.5
        with self.assertRaisesRegex(ValueError, "disagrees"):
            Exposure().add(trigger, 1)
        trigger = row()
        trigger["thresholds"]["army"] = 0.36
        with self.assertRaisesRegex(ValueError, "limits"):
            Exposure().add(trigger, 1)

    def test_roster_match_excludes_comment_and_alias_names(self):
        source = '// Niger is absent\n row("Nigeria", "Nigeria", &["Niger"],\n row("Haiti", "Haiti",\n'
        self.assertEqual(roster_codes(source), {"Nigeria", "Haiti"})

    def test_missing_months_do_not_form_a_continuous_regime_interval(self):
        audit = Exposure()
        audit.add(row("ai_before_funding", False), 1)
        audit.add(row("ai_before_funding", False, (1990, 3, 1)), 2)
        self.assertEqual(len(audit.countries["Example"]["funding_stage_regime_intervals"]), 2)

    def test_missing_country_has_no_invented_zero_month_success(self):
        result = next(x for x in Exposure().summary(set()) if x["country"] == "Niger")
        self.assertEqual(result["classification"], "outside_roster")
        self.assertEqual(result["funding_stage_regime_intervals"], [])


if __name__ == "__main__":
    unittest.main()
