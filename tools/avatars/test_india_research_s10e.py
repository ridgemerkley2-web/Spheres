"""Keep ECI recognition observations distinct from invented party histories."""
import copy
import json
import unittest
from collections import Counter
from datetime import date
from urllib.parse import urlsplit

import campaign_research as research


class IndiaDiscoveryTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.packet = json.loads((research.ROOT / research.RESEARCH / 'india.json').read_text(encoding='utf-8'))
        cls.extracts = {
            s['id']: json.loads((research.ROOT / s['snapshot']['path']).read_text(encoding='utf-8'))
            for s in cls.packet['sources']
        }

    def validate(self, packet=None):
        return research.validate(packet or self.packet, research.ROOT, {'India'}, {'India': set()})

    def test_dated_registers_are_valid_and_partial(self):
        ids = self.validate()
        self.assertEqual(tuple(len(ids[k]) for k in ('entries', 'sources', 'claims', 'roles')), (82, 2, 82, 0))
        coverage = self.packet['coverage']
        self.assertFalse(coverage['exhaustive_organization_register_reviewed'])
        self.assertIsNone(coverage['unrepresented_organization_total'])
        self.assertFalse(coverage['art_completion_claim'])
        self.assertEqual([r['records'] for r in coverage['bounded_registers']], [6, 76])
        self.assertEqual(coverage['bounded_registers'][1]['jurisdictions'], 26)
        self.assertEqual(self.packet['institutions'], [])

    def test_all_source_jurisdiction_rows_are_retained_without_deduplication(self):
        state = self.extracts['in_eci_state_parties_20240323']['rows']
        counts = Counter(r['source_state_number'] for r in state)
        self.assertEqual(list(counts.values()), [2, 3, 4, 5, 1, 3, 2, 3, 3, 1, 5, 5, 3, 5, 3, 5, 1, 3, 1, 1, 2, 4, 4, 3, 2, 2])
        self.assertEqual(set(counts), set(range(1, 27)))
        jds = [o for o in self.packet['organizations'] if o['name'] == 'Janata Dal (Secular)']
        self.assertEqual({o['jurisdiction'] for o in jds}, {'Arunachal Pradesh', 'Karnataka', 'Kerala'})
        self.assertEqual(len({o['id'] for o in jds}), 3)
        self.assertTrue(all(o['reconciled_organization_id'] is None for o in jds))

    def test_source_numbering_and_spelling_are_not_repaired_into_invented_rows(self):
        state = self.extracts['in_eci_state_parties_20240323']['rows']
        kerala = [r for r in state if r['jurisdiction'] == 'Kerala']
        self.assertEqual([r['source_party_number'] for r in kerala], [1, 2, 3, 4, 6])
        self.assertEqual(kerala[-1]['name'], 'Communist Party of India')
        self.assertEqual(sum(r['source_party_number'] is None for r in state), 5)
        self.assertIn('Shiv Sena (Uddhav Balasaheb Thackrey)', {r['name'] for r in state})
        self.assertIn('Janta Congress Chhattisgarh (J)', {r['name'] for r in state})
        self.assertEqual({r['pdf_page'] for r in state}, set(range(3, 11)))

    def test_frozen_names_and_court_qualifications_survive_without_successor_grants(self):
        qualified = [o for o in self.packet['organizations'] if o['recognition']['source_qualifications']]
        self.assertEqual(len(qualified), 5)
        frozen = [o for o in qualified if o['recognition']['source_qualifications'][0]['kind'] == 'name_and_symbol_frozen']
        self.assertEqual({o['name'] for o in frozen}, {'Lok Jan Shakti Party', 'Jammu & Kashmir National Panthers Party'})
        self.assertEqual({o['recognition']['source_qualifications'][0]['referenced_order_date'] for o in frozen}, {'2021-10-02', '2024-03-20'})
        pending = [o for o in qualified if o['recognition']['source_qualifications'][0]['kind'] == 'pending_court_order']
        self.assertEqual(Counter(o['name'] for o in pending), {
            'Shiv Sena (Uddhav Balasaheb Thackrey)': 1,
            'Nationalist Congress Party – Sharadchandra Pawar': 2})
        self.assertTrue(all(o['recognition']['consolidated_current_status'] is None for o in qualified))

    def test_access_dates_never_extend_historical_attestations(self):
        for source in self.packet['sources']:
            self.assertEqual(urlsplit(source['url']).hostname, 'www.ceo.kerala.gov.in')
            self.assertEqual(source['published_date'], '2024-03-28')
            self.assertEqual(source['accessed_date'], '2026-09-13')
            self.assertLessEqual(date.fromisoformat(source['published_date']), date.fromisoformat(research.CUTOFF))
            self.assertEqual({c['attested_on'] for c in source['claims']}, {'2024-03-23'})
        p = copy.deepcopy(self.packet)
        p['organizations'][0]['recognition']['attested_on'] = '2026-09-08'
        with self.assertRaisesRegex(ValueError, 'exceeds cutoff'):
            self.validate(p)

    def test_recognition_does_not_grant_roles_lifespans_or_game_identity(self):
        for entry in self.packet['organizations']:
            self.assertEqual(entry['roles'], [])
            self.assertEqual(entry['represented_party_ids'], [])
            self.assertEqual(entry['coverage']['status'], 'reporting_identity_only')
            self.assertIsNone(entry['reconciled_organization_id'])
            for field in ('lifecycle', 'recognition'):
                self.assertIsNone(entry[field]['from'])
                self.assertIsNone(entry[field]['until'])
        p = copy.deepcopy(self.packet)
        p['organizations'][0]['represented_party_ids'] = ['India/guessed_aap']
        with self.assertRaisesRegex(ValueError, 'foreign represented party mapping'):
            self.validate(p)

    def test_factual_extracts_reconcile_every_claim_and_protect_source_bytes(self):
        orgs = {o['id']: o for o in self.packet['organizations']}
        for source in self.packet['sources']:
            data = self.extracts[source['id']]
            self.assertEqual(data['format'], 'spheres-c01-derived-factual-table/v1')
            self.assertEqual(data['source_url'], source['url'])
            self.assertEqual(data['source_notification_date'], '2024-03-23')
            self.assertEqual(data['gazette_publication_date'], '2024-03-28')
            self.assertRegex(data['source_response_sha256'], r'^[0-9a-f]{64}$')
            self.assertGreater(data['source_response_bytes'], 300000)
            self.assertEqual({r['claim_id'] for r in data['rows']}, {c['id'] for c in source['claims']})
            for row in data['rows']:
                org = orgs[row['observation_id']]
                self.assertEqual((org['name'], org['jurisdiction']), (row['name'], row['jurisdiction']))
                self.assertEqual(org['recognition']['source_qualifications'], row['source_qualifications'])
        p = copy.deepcopy(self.packet)
        p['sources'][0]['snapshot']['sha256'] = '0' * 64
        with self.assertRaisesRegex(ValueError, 'checksum mismatch'):
            self.validate(p)

    def test_discovery_batches_remain_open_and_separate_from_campaign_certification(self):
        index = research.build()
        india = next(p for p in index['countries'] if p['nation'] == 'India')
        self.assertFalse(india['country_census_complete'])
        self.assertEqual(india['mapping_pending'], 82)
        self.assertFalse(index['runtime_roster_modified'])
        self.assertFalse(index['c01_complete'])
        self.assertFalse(index['g2_prerequisite_satisfied'])
        work = [w for w in index['work_orders'] if w['nation'] == 'India']
        self.assertEqual([len(w['members']) for w in work], [10] * 8 + [2])
        self.assertEqual({m for w in work for m in w['members']}, set(self.validate()['entries']))
        self.assertEqual({w['status'] for w in work}, {'open'})


if __name__ == '__main__':
    unittest.main()
