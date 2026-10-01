"""Regression checks for C01-40 original-content review; not historical approval."""
import json
import unittest

import campaign_research as research


class CpimOriginalReview(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        packet = json.loads((research.ROOT / 'docs/campaign-certification/C01/research/india.json').read_text(encoding='utf-8'))
        cls.claims = {claim['id']: claim for source in packet['sources'] for claim in source['claims']}
        cls.report = (research.ROOT / 'docs/campaign-certification/C01/research/india-cpim-general-secretaries-1990-2026-40.md').read_text(encoding='utf-8')

    def test_handover_points_to_its_own_paragraph_not_the_1992_recollection(self):
        locator = self.claims['in_pd_surjeet_says_karat_took_up_mantle_2005']['locator']['item']
        self.assertIn('third paragraph', locator)
        self.assertNotIn('first paragraph', locator)

    def test_rally_2012_locator_identifies_both_date_and_office_passages(self):
        locator = self.claims['in_pd_rally_newly_elected_gs_karat_took_salute_20120409']['locator']['item']
        self.assertIn('But this is a glimpse', locator)
        self.assertIn('Each contingent', locator)
        self.assertNotEqual(locator, 'first and third paragraphs')

    def test_2015_office_locators_use_named_anchors_and_explicit_date_section(self):
        for cid, anchor in [
            ('in_pd_rally_karat_outgoing_gs_20150419', 'Prakash Karat'),
            ('in_pd_rally_newly_elected_gs_yechury_20150419', 'Newly elected general secretary'),
        ]:
            with self.subTest(claim=cid):
                locator = self.claims[cid]['locator']['item']
                self.assertIn(anchor, locator)
                self.assertIn('Arise in Struggle', locator)
                self.assertNotIn('ninth paragraph', locator)
                self.assertNotIn('tenth paragraph', locator)

    def test_ems_opening_identity_is_not_asserted_without_evidence(self):
        self.assertNotIn('He was General Secretary when the period opened, by every account', self.report)
        self.assertIn('An opening-era holder is not established by this packet', self.report)

    def test_yechury_election_recollection_does_not_point_to_student_biography(self):
        locator = self.claims['in_cpim_pb_yechury_elected_gs_21st_congress_recalled']['locator']['item']
        self.assertIn('He was elected as the General Secretary', locator)
        self.assertNotEqual(locator, 'third paragraph')

    def test_current_rest_metadata_is_not_treated_as_a_historical_capture(self):
        self.assertIn('not independently timestamped historical captures', self.report)
        self.assertIn('current party record', self.report)


if __name__ == '__main__':
    unittest.main()
