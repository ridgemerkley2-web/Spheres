"""Keep Russia's dated ballot and faction observations separate from histories."""
import copy
import hashlib
import json
import unittest
from urllib.parse import urlsplit

import campaign_research as research


class RussiaDiscoveryTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.packet = json.loads((research.ROOT / research.RESEARCH / 'russia.json').read_text(encoding='utf-8'))
        cls.sources = {source['id']: source for source in cls.packet['sources']}
        cls.extracts = {
            source['id']: json.loads((research.ROOT / source['snapshot']['path']).read_text(encoding='utf-8'))
            for source in cls.packet['sources']
        }

    def validate(self, packet=None):
        return research.validate(packet or self.packet, research.ROOT, {'Russia'}, {'Russia': set()})

    def factions(self):
        # CLAUDE-C01-05 added exactly one non-faction institution, the RSFSR presidency, pinned in
        # test_ussr_russia_transition_c01_05. Faction checks keep their full strength on the five factions.
        rows = [e for e in self.packet['institutions'] if e['kind'] == 'parliamentary_faction']
        self.assertEqual(len(rows), 5)
        self.assertEqual([e['id'] for e in self.packet['institutions'] if e['kind'] != 'parliamentary_faction'],
                         ['ru_rsfsr_presidency'])
        return rows

    def test_partial_intake_has_fourteen_lists_and_five_distinct_institutions(self):
        ids = self.validate()
        # CLAUDE-C01-05 added 13 sources, 24 claims, one institution and two roles.
        self.assertEqual(tuple(len(ids[key]) for key in ('entries', 'sources', 'claims', 'roles')), (20, 15, 48, 7))
        self.assertEqual(len(self.packet['organizations']), 14)
        self.assertEqual(len(self.packet['institutions']), 6)
        self.assertEqual({e['kind'] for e in self.packet['organizations']}, {'federal_election_ballot_party_list'})
        self.assertEqual({e['kind'] for e in self.packet['institutions']}, {'parliamentary_faction', 'executive_presidency_office'})
        self.assertEqual(len(self.factions()), 5)
        coverage = self.packet['coverage']
        self.assertFalse(coverage['exhaustive_organization_register_reviewed'])
        self.assertIsNone(coverage['unrepresented_organization_total'])
        self.assertFalse(coverage['art_completion_claim'])
        self.assertEqual([row['records'] for row in coverage['bounded_registers']], [14, 5])

    def test_complete_ballot_order_preserves_exact_printed_identity_labels(self):
        rows = self.extracts['ru_cec_ballot_order_20210816']['rows']
        self.assertEqual([row['ballot_position'] for row in rows], list(range(1, 15)))
        self.assertEqual([row['party_label'] for row in rows], [
            'Политическая партия «КОММУНИСТИЧЕСКАЯ ПАРТИЯ РОССИЙСКОЙ ФЕДЕРАЦИИ»',
            'Политическая партия «Российская экологическая партия «ЗЕЛЁНЫЕ»',
            'Политическая партия ЛДПР – Либерально-демократическая партия России',
            'Политическая партия «НОВЫЕ ЛЮДИ»',
            'Всероссийская политическая партия «ЕДИНАЯ РОССИЯ»',
            'Партия СПРАВЕДЛИВАЯ РОССИЯ – ЗА ПРАВДУ',
            'Политическая партия «Российская объединенная демократическая партия «ЯБЛОКО»',
            'Всероссийская политическая партия «ПАРТИЯ РОСТА»',
            'Политическая партия РОССИЙСКАЯ ПАРТИЯ СВОБОДЫ И СПРАВЕДЛИВОСТИ',
            'Политическая партия КОММУНИСТИЧЕСКАЯ ПАРТИЯ КОММУНИСТЫ РОССИИ',
            'Политическая партия «Гражданская Платформа»',
            'Политическая партия ЗЕЛЕНАЯ АЛЬТЕРНАТИВА',
            'ВСЕРОССИЙСКАЯ ПОЛИТИЧЕСКАЯ ПАРТИЯ «РОДИНА»',
            'ПАРТИЯ ПЕНСИОНЕРОВ'])
        entries = {e['id']: e for e in self.packet['organizations']}
        for row in rows:
            entry = entries[row['observation_id']]
            self.assertEqual(entry['name'], row['party_label'])
            self.assertEqual(entry['source_identifier']['value'], str(row['ballot_position']))
            self.assertEqual(entry['claim_ids'], [row['claim_id']])

    def test_resolution_observation_never_becomes_election_or_publication_day(self):
        source = self.sources['ru_cec_ballot_order_20210816']
        self.assertIsNone(source['published_date'])
        extract = self.extracts[source['id']]
        self.assertEqual(extract['document']['resolution'], '42/337-8')
        self.assertEqual(extract['document']['resolution_date'], '2021-08-16')
        self.assertEqual(extract['document']['named_election_date'], '2021-09-19')
        for claim in source['claims']:
            self.assertEqual(claim['attested_on'], '2021-08-16')
            self.assertEqual(claim['locator']['pdf_page_one_based'], 145)

    def test_faction_membership_is_not_silently_changed_into_party_election_seats(self):
        rows = self.extracts['ru_duma_factions_20211012']['rows']
        self.assertEqual([(r['faction_label'], r['members']) for r in rows], [
            ('Единая Россия', 324), ('КПРФ', 57), ('Справедливая Россия — За правду', 28),
            ('ЛДПР', 23), ('Новые люди', 15)])
        self.assertEqual(sum(row['members'] for row in rows), 447)
        for entry in self.factions():
            self.assertIsNone(entry['membership_observation']['party_election_seats'])
            self.assertEqual(entry['membership_observation']['attested_on'], '2021-10-12')
            self.assertEqual(entry['constituent_organization_ids'], [])

    def test_only_parliamentary_offices_are_attested_without_invented_starts(self):
        expected = {'Владимир Васильев', 'Геннадий Зюганов', 'Владимир Жириновский', 'Сергей Миронов', 'Алексей Нечаев'}
        actual = set()
        for entry in self.factions():
            self.assertEqual(len(entry['roles']), 1)
            role = entry['roles'][0]
            self.assertEqual(role['kind'], 'parliamentary_leader')
            self.assertEqual(len(role['holder_claims']), 1)
            holder = role['holder_claims'][0]
            actual.add(holder['name'])
            self.assertEqual(holder['attested_on'], '2021-10-12')
            self.assertIsNone(holder['from'])
            self.assertIsNone(holder['until'])
            self.assertEqual(len(holder['name'].split()), 2, 'Do not import dynamic hover biography names')
        self.assertEqual(actual, expected)
        self.assertTrue(all(not entry['roles'] for entry in self.packet['organizations']))

    def test_downloaded_pdf_hash_is_distinct_from_the_derived_factual_extract(self):
        source = self.sources['ru_cec_ballot_order_20210816']
        extract = self.extracts[source['id']]
        self.assertEqual(extract['source_response_bytes'], 3838526)
        self.assertEqual(extract['source_response_sha256'], 'b6f39b95e37f16586826ac7fe1a4c06ff552ad00ed55d2ad94a538ba852fa5e3')
        self.assertFalse(extract['source_response_checked_in'])
        self.assertEqual(extract['visual_review']['pdf_pages_one_based'], [1, 144, 145])
        path = research.ROOT / source['snapshot']['path']
        self.assertEqual(hashlib.sha256(path.read_bytes()).hexdigest(), source['snapshot']['sha256'])
        self.assertNotEqual(source['snapshot']['sha256'], extract['source_response_sha256'])
        packet = copy.deepcopy(self.packet)
        packet['sources'][0]['snapshot']['sha256'] = '0' * 64
        with self.assertRaisesRegex(ValueError, 'checksum mismatch'):
            self.validate(packet)

    def test_indexed_duma_text_never_claims_a_downloaded_original_response(self):
        source = self.sources['ru_duma_factions_20211012']
        extract = self.extracts[source['id']]
        self.assertEqual(source['access_method'], 'official_dated_article_text_via_search_index')
        self.assertEqual(extract['access_method'], source['access_method'])
        self.assertIsNone(extract['source_response_bytes'])
        self.assertIsNone(extract['source_response_sha256'])
        self.assertFalse(extract['source_response_checked_in'])
        self.assertIn('timed out', extract['provenance_note'])
        self.assertIn('Original response verification remains open', ' '.join(self.packet['coverage']['unresolved']))

    def test_sources_and_claims_reconcile_with_factual_snapshots(self):
        # CLAUDE-C01-05 added the legal portal, Rosarkhiv and Presidential Library sources.
        self.assertEqual({urlsplit(s['url']).hostname for s in self.packet['sources']},
                         {'www.rcoit.ru', 'duma.gov.ru', 'pravo.gov.ru', 'projects.rusarchives.ru', 'www.prlib.ru'})
        for source in self.packet['sources']:
            extract = self.extracts[source['id']]
            self.assertEqual(extract['source_url'], source['url'])
            original = source['id'] in {'ru_cec_ballot_order_20210816', 'ru_duma_factions_20211012'}
            self.assertEqual(source['accessed_date'], '2026-09-13' if original else '2026-09-21')
            self.assertEqual(extract['format'], 'spheres-c01-derived-factual-table/v1')
            if 'claims' in extract:
                self.assertEqual(extract['claims'], source['claims'])
            else:
                self.assertEqual({r['claim_id'] for r in extract['rows']}, {c['id'] for c in source['claims']})
            self.assertTrue(all(c['attested_on'] <= research.CUTOFF for c in source['claims']))
        packet = copy.deepcopy(self.packet)
        packet['sources'][0]['claims'][0]['attested_on'] = '2026-09-08'
        with self.assertRaisesRegex(ValueError, 'exceeds cutoff'):
            self.validate(packet)

    def test_unknown_identities_do_not_merge_factions_parties_or_predecessor_states(self):
        for entry in self.packet['organizations'] + self.packet['institutions']:
            self.assertEqual(entry['represented_party_ids'], [])
            self.assertIsNone(entry['reconciled_organization_id'])
            self.assertIsNone(entry['lifecycle']['from'])
            self.assertIsNone(entry['lifecycle']['until'])
        self.assertIn('Russia and USSR', ' '.join(self.packet['coverage']['unresolved']))
        packet = copy.deepcopy(self.packet)
        packet['organizations'][0]['represented_party_ids'] = ['USSR/cpsu']
        with self.assertRaisesRegex(ValueError, 'foreign represented party mapping'):
            self.validate(packet)

    def test_intake_build_produces_only_open_discovery_batches(self):
        index = research.build()
        country = next(p for p in index['countries'] if p['nation'] == 'Russia')
        self.assertFalse(country['country_census_complete'])
        self.assertEqual(country['mapping_pending'], 20)
        self.assertEqual(country['role_observations'], 7)
        work = [row for row in index['work_orders'] if row['nation'] == 'Russia']
        self.assertEqual([len(row['members']) for row in work], [10, 10])
        self.assertEqual({member for row in work for member in row['members']}, set(self.validate()['entries']))
        self.assertEqual({row['status'] for row in work}, {'open'})
        self.assertFalse(index['runtime_roster_modified'])
        self.assertFalse(index['c01_complete'])
        self.assertFalse(index['g2_prerequisite_satisfied'])


if __name__ == '__main__':
    unittest.main()
