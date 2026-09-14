"""Keep USSR party observations separate from state offices and successor states."""
import copy
import json
import unittest

import campaign_research as research


class UssrDiscoveryTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.packet = json.loads((research.ROOT / research.RESEARCH / 'ussr.json').read_text(encoding='utf-8'))
        cls.sources = {s['id']: s for s in cls.packet['sources']}
        cls.claims = {c['id']: c for s in cls.packet['sources'] for c in s['claims']}
        cls.entries = {e['id']: e for group in ('organizations', 'institutions') for e in cls.packet[group]}
        cls.extracts = {s['id']: json.loads((research.ROOT / s['snapshot']['path']).read_text(encoding='utf-8')) for s in cls.packet['sources']}

    def validate(self, packet=None):
        return research.validate(packet or self.packet, research.ROOT, {'USSR', 'Russia'}, {'USSR': set(), 'Russia': {'Russia/example'}})

    def test_bounded_inventory_cannot_certify_an_exhaustive_country(self):
        ids = self.validate()
        self.assertEqual(tuple(len(ids[key]) for key in ('entries', 'sources', 'claims', 'roles')), (4, 3, 8, 5))
        self.assertEqual(len(self.packet['organizations']), 1)
        self.assertEqual(len(self.packet['institutions']), 3)
        self.assertEqual(self.packet['coverage']['status'], 'partial_primary_source_inventory')
        self.assertFalse(self.packet['coverage']['exhaustive_organization_register_reviewed'])
        self.assertIsNone(self.packet['coverage']['unrepresented_organization_total'])
        self.assertFalse(self.packet['coverage']['art_completion_claim'])
        self.assertIn('1991 office changes', ' '.join(self.packet['coverage']['unresolved']))

    def test_union_institutions_never_merge_into_russia_or_party_rows(self):
        self.assertEqual(self.packet['nation'], 'USSR')
        for entry in self.entries.values():
            self.assertEqual(entry['jurisdiction']['nation'], 'USSR')
            self.assertEqual(entry['jurisdiction']['level'], 'union')
            self.assertFalse(entry['jurisdiction']['automatic_successor_mapping'])
            self.assertEqual(entry['represented_party_ids'], [])
        packet = copy.deepcopy(self.packet)
        packet['organizations'][0]['represented_party_ids'] = ['Russia/example']
        with self.assertRaisesRegex(ValueError, 'foreign represented party mapping'):
            self.validate(packet)

    def test_party_and_state_offices_have_separate_claims_and_kinds(self):
        general, deputy = self.entries['su_cpsu']['roles']
        president, = self.entries['su_presidency']['roles']
        chair, = self.entries['su_supreme_soviet']['roles']
        delegates, = self.entries['su_congress_peoples_deputies']['roles']
        self.assertEqual((general['kind'], deputy['kind'], president['kind'], chair['kind'], delegates['kind']),
                         ('party_leader', 'other', 'head_of_state', 'institutional_office', 'collective_seat'))
        self.assertEqual(general['sources'], ['su_japan_diplomatic_bluebook_1990'])
        self.assertEqual(deputy['sources'], general['sources'])
        self.assertEqual(chair['holder_claims'][0]['claim_ids'], ['su_gorbachev_chair_signature_19900314'])
        self.assertEqual(president['holder_claims'][0]['claim_ids'], ['su_gorbachev_president_letter_19900320'])
        self.assertEqual(delegates['holder_claims'], [])
        self.assertTrue(set(general['claim_ids']).isdisjoint(president['claim_ids']))

    def test_enactment_signature_and_letter_do_not_become_an_oath_or_term(self):
        law = self.sources['su_presidency_law_19900314']
        self.assertEqual(law['document_date'], '1990-03-14')
        self.assertEqual({c['attested_on'] for c in law['claims']}, {'1990-03-14'})
        self.assertIn('upon taking the oath', self.claims['su_first_president_congress_rule_19900314']['text'])
        self.assertIn('not evidence of who was elected', self.claims['su_first_president_congress_rule_19900314']['uncertainty'])
        self.assertIn('replaces Article 6', self.claims['su_article6_replacement_19900314']['text'])
        self.assertIn('not dissolution', self.claims['su_article6_replacement_19900314']['uncertainty'])
        president = self.entries['su_presidency']['roles'][0]['holder_claims'][0]
        chair = self.entries['su_supreme_soviet']['roles'][0]['holder_claims'][0]
        self.assertEqual((chair['attested_on'], president['attested_on']), ('1990-03-14', '1990-03-20'))
        self.assertIn('21 March 0001Z', self.sources['su_bush_presidential_letter_19900320']['scope_note'])
        for entry in self.entries.values():
            if entry['id'] == 'su_presidency':
                self.assertEqual(entry['lifecycle']['from'], '1990-03-14')
                self.assertEqual(entry['lifecycle']['precision'], 'creation_day_only')
                self.assertIn('not a person', entry['lifecycle']['note'])
            else:
                self.assertIsNone(entry['lifecycle']['from'])
            self.assertIsNone(entry['lifecycle']['until'])
            for role in entry['roles']:
                for holder in role['holder_claims']:
                    self.assertIsNone(holder['from'])
                    self.assertIsNone(holder['until'])

    def test_diplomatic_month_observations_keep_spelling_uncertainty(self):
        source = self.sources['su_japan_diplomatic_bluebook_1990']
        self.assertEqual(source['source_type'], 'contemporary_foreign_ministry_observation')
        self.assertNotIn('published_date', source)
        self.assertEqual(source['edition'], '1990')
        for claim in source['claims']:
            self.assertEqual(claim['period'], {'from': '1990-07-01', 'through': '1990-07-31'})
            self.assertEqual(claim['precision'], 'month')
            self.assertNotIn('attested_on', claim)
        holders = [r['holder_claims'][0] for r in self.entries['su_cpsu']['roles']]
        self.assertIn('Ivashkov (source spelling; identity reconciliation pending)', holders[1]['name'])
        for holder in holders:
            self.assertEqual(holder['observation_window'], {'from': '1990-07-01', 'through': '1990-07-31'})
            self.assertNotIn('attested_on', holder)

    def test_downloaded_raw_responses_are_distinct_from_factual_snapshots(self):
        expected = {
            'su_presidency_law_19900314': (166710, '710afdf4fd5df03e3098c69d783f0fdd1528f42b858e36420d531a05ce1c054e'),
            'su_bush_presidential_letter_19900320': (40716, 'e39829ace8d141b2b8651d9b8fcc7d8c6bfa19db42cda0ef0a6046aa589e3558'),
        }
        for sid, extract in self.extracts.items():
            source = self.sources[sid]
            self.assertEqual(extract['source_url'], source['url'])
            self.assertEqual(extract['claims'], source['claims'])
            self.assertEqual(extract['scope_note'], source['scope_note'])
            self.assertFalse(extract['source_response_checked_in'])
            self.assertEqual(source['snapshot']['kind'], 'derived_factual_extract')
            if sid in expected:
                self.assertEqual((extract['source_response_bytes'], extract['source_response_sha256']), expected[sid])
                self.assertNotEqual(extract['source_response_sha256'], source['snapshot']['sha256'])
        packet = copy.deepcopy(self.packet)
        packet['sources'][0]['snapshot']['sha256'] = '0' * 64
        with self.assertRaisesRegex(ValueError, 'checksum mismatch'):
            self.validate(packet)

    def test_unavailable_raw_download_never_claims_a_source_hash(self):
        extract = self.extracts['su_japan_diplomatic_bluebook_1990']
        self.assertIsNone(extract['source_response_sha256'])
        self.assertIsNone(extract['source_response_bytes'])
        self.assertIsNone(extract['source_response_url'])
        self.assertIn('403', extract['provenance_note'])
        self.assertIn('web retrieval', self.sources['su_japan_diplomatic_bluebook_1990']['access_method'])

    def test_visual_review_is_exactly_the_four_jpg_facsimiles(self):
        law = self.extracts['su_presidency_law_19900314']
        self.assertEqual(law['visual_review']['facsimile_pages_one_based'], [1, 3, 13, 14])
        self.assertEqual([r['page'] for r in law['reviewed_facsimile_responses']], [1, 3, 13, 14])
        for row in law['reviewed_facsimile_responses']:
            self.assertTrue(row['url'].endswith('.jpg'))
            self.assertGreater(row['bytes'], 1000)
            self.assertRegex(row['sha256'], r'^[0-9a-f]{64}$')
        for extract in self.extracts.values():
            self.assertEqual(extract['visual_review']['pdf_pages_one_based'], [])
            self.assertIn('no source artwork or portrait copied', extract['rights_note'])
            self.assertIn('No portrait permission or likeness approval', extract['rights_note'])

    def test_cutoff_and_unknown_term_guards_reject_future_or_reversed_history(self):
        self.assertEqual(self.packet['research_cutoff'], '2026-09-07')
        self.assertEqual(self.packet['coverage']['period'], {'from': '1990-01-01', 'through': '2026-09-07'})
        self.assertEqual({s['accessed_date'] for s in self.sources.values()}, {'2026-09-13'})
        packet = copy.deepcopy(self.packet)
        packet['institutions'][0]['roles'][0]['holder_claims'][0]['attested_on'] = '2026-09-08'
        with self.assertRaisesRegex(ValueError, 'exceeds cutoff'):
            self.validate(packet)
        packet = copy.deepcopy(self.packet)
        packet['organizations'][0]['roles'][0]['holder_claims'][0]['observation_window']['through'] = '1990-06-30'
        with self.assertRaisesRegex(ValueError, 'Reversed historical interval'):
            self.validate(packet)

    def test_discovery_work_stays_open_without_rewriting_runtime(self):
        index = research.build()
        country = next(c for c in index['countries'] if c['nation'] == 'USSR')
        self.assertEqual(country['organization_observations'], 1)
        self.assertEqual(country['institution_observations'], 3)
        self.assertEqual(country['source_claims'], 8)
        self.assertEqual(country['mapping_pending'], 4)
        self.assertFalse(country['country_census_complete'])
        self.assertFalse(index['c01_complete'])
        self.assertFalse(index['g2_prerequisite_satisfied'])
        self.assertFalse(index['runtime_roster_modified'])
        batches = [w for w in index['work_orders'] if w['nation'] == 'USSR']
        self.assertEqual(len(batches), 1)
        self.assertEqual(batches[0]['status'], 'open')
        self.assertEqual(set(batches[0]['members']), set(self.entries))


if __name__ == '__main__':
    unittest.main()
