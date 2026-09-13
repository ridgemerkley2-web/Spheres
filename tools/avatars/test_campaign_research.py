"""Failure cases for historical discovery intake and bounded follow-up work."""
import copy
import unittest

import campaign_research as research
import import_cnccfp_census as france


class ResearchTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.france = france.build((research.ROOT / france.RAW).read_bytes())
        cls.countries = {'France'}
        cls.parties = {'France': {'France/fr_ps'}}

    def valid(self, packet):
        return research.validate(packet, research.ROOT, self.countries, self.parties)

    def test_official_snapshot_is_pinned_and_observations_are_not_terms(self):
        ids = self.valid(self.france)
        self.assertEqual(len(ids['entries']), 635)
        self.assertFalse(ids['roles'])
        self.assertTrue(all(o['lifecycle']['status'] == 'unknown' and not o['represented_party_ids'] for o in self.france['organizations']))
        with self.assertRaisesRegex(ValueError, 'checksum mismatch'):
            france.build((research.ROOT / france.RAW).read_bytes() + b'\n')

    def test_duplicate_identity_and_broken_claim_fail(self):
        p = copy.deepcopy(self.france)
        p['organizations'].append(p['organizations'][0])
        with self.assertRaisesRegex(ValueError, 'Duplicate research identity'):
            self.valid(p)
        p = copy.deepcopy(self.france)
        p['organizations'][0]['claim_ids'] = ['missing']
        with self.assertRaisesRegex(ValueError, 'claim reference'):
            self.valid(p)

    def test_claim_must_belong_to_the_cited_source(self):
        p = copy.deepcopy(self.france)
        p['organizations'][0]['claim_ids'] = ['fr_cnccfp_identity_scope']
        with self.assertRaisesRegex(ValueError, 'does not belong'):
            self.valid(p)

    def test_no_postcutoff_holders_but_later_research_access_is_allowed(self):
        self.valid(self.france)  # Access 13 September, historical cutoff 7 September.
        p = copy.deepcopy(self.france)
        p['organizations'][0]['lifecycle']['from'] = '2026-09-08'
        with self.assertRaisesRegex(ValueError, 'exceeds cutoff'):
            self.valid(p)
        p['organizations'][0]['lifecycle'].update({'from': '2020-01-01', 'until': '2019-01-01'})
        with self.assertRaisesRegex(ValueError, 'Reversed'):
            self.valid(p)

    def test_unknown_party_mapping_is_not_silently_accepted(self):
        p = copy.deepcopy(self.france)
        p['organizations'][0]['represented_party_ids'] = ['SaudiArabia/fake']
        with self.assertRaisesRegex(ValueError, 'foreign represented'):
            self.valid(p)

    def test_partial_intake_cannot_close_census(self):
        for update in ({'status': 'complete'}, {'unresolved': []}):
            p = copy.deepcopy(self.france)
            p['coverage'].update(update)
            with self.assertRaisesRegex(ValueError, 'cannot close'):
                self.valid(p)

    def test_snapshot_path_cannot_escape_sources(self):
        p = copy.deepcopy(self.france)
        p['sources'][0]['snapshot']['path'] = 'Cargo.toml'
        with self.assertRaisesRegex(ValueError, 'escapes'):
            self.valid(p)

    def test_role_holders_need_explicit_roles_and_matching_source_claims(self):
        p = copy.deepcopy(self.france)
        entry = p['organizations'][0]
        p['sources'].append({'id': 'fr_synthetic_source', 'url': 'https://example.org/test-only',
            'title': 'Synthetic validation fixture, not historical evidence',
            'publisher': 'Test fixture', 'accessed_date': '2026-09-13',
            'claims': [{'id': 'fr_synthetic_holder',
                'text': 'The synthetic Validation Fixture person is attested as chair on 2020-01-01.'}]})
        role = {'id': 'fr_test_chair', 'title': 'Chair', 'kind': 'party_chair',
                'sources': ['fr_synthetic_source'], 'claim_ids': ['fr_synthetic_holder'],
                'holder_claims': [{'name': 'Validation fixture', 'from': None, 'until': None,
                    'attested_on': '2020-01-01', 'sources': ['fr_synthetic_source'], 'claim_ids': ['fr_synthetic_holder']}]}
        entry['roles'] = [role]
        self.assertIn('fr_test_chair', self.valid(p)['roles'])
        role['holder_claims'][0]['attested_on'] = '2026-09-08'
        with self.assertRaisesRegex(ValueError, 'exceeds cutoff'):
            self.valid(p)
        role['holder_claims'][0]['attested_on'] = '2020-01-01'
        role['holder_claims'][0]['claim_ids'] = []
        with self.assertRaisesRegex(ValueError, 'missing claim'):
            self.valid(p)
        role['holder_claims'] = ['fr_cnccfp_identity_scope']
        with self.assertRaisesRegex(ValueError, 'cited role source'):
            self.valid(p)

    def test_every_discovery_gets_one_bounded_work_order_without_closing_gate(self):
        index = research.build()
        self.assertFalse(index['c01_complete'])
        self.assertFalse(index['g2_prerequisite_satisfied'])
        members = [m for w in index['work_orders'] for m in w['members']]
        self.assertEqual(len(set(members)), len(members))
        self.assertEqual(len(members), index['counts']['organization_observations'] + index['counts']['institution_observations'])
        self.assertTrue(all(0 < len(w['members']) <= 10 for w in index['work_orders']))
        self.assertTrue(all(c['unrepresented_organization_count'] is None for c in index['countries']))


if __name__ == '__main__':
    unittest.main()
