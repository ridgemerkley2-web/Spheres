"""Keep ECI recognition observations distinct from invented party histories."""
import copy
import json
import unittest
from collections import Counter
from datetime import date
from urllib.parse import urlsplit

import campaign_research as research

# The two sources of the original S10e intake keep every assertion below by id. CLAUDE-C01-11 adds 70 sources for the
# in_prime_minister institution whose response identities are pinned exactly (bytes and SHA-256) in
# test_india_prime_ministers_c01_11.py, and CLAUDE-C01-15 then adds 56 sources for the in_presidency institution, pinned
# the same way in test_india_presidents_c01_15.py. CLAUDE-C01-20 then adds 73 sources for one party role, in_inc_president,
# on the Indian National Congress recognition observation, pinned in test_india_inc_presidents_c01_20.py. CLAUDE-C01-27
# then adds 74 sources for a second party role, in_bjp_president, on the Bharatiya Janata Party recognition observation,
# pinned in test_india_bjp_presidents_c01_27.py. CLAUDE-C01-33 then adds one organization observation, the Janata Dal row
# of the Election Commission's national-party table of 10 January 1998, with 22 sources and one party role,
# in_jd_president, pinned in test_india_janata_dal_presidents_c01_33.py.
ORIGINAL_SOURCES = ('in_eci_national_parties_20240323', 'in_eci_state_parties_20240323')
# CLAUDE-C01-11 sources: raw Internet Archive captures, Internet Archive item copies, and the eGazette, Rajya Sabha and
# Cabinet Secretariat file hosts, accessed on 2026-09-23 or 2026-09-24.
C01_11_HOSTS = {'web.archive.org', 'archive.org', 'egazette.gov.in', 'bucketapi.rajyasabha.digital', 'cabsec.gov.in'}
C01_11_ACCESS_DATES = {'2026-09-23', '2026-09-24'}
C01_11_SOURCE_COUNT = 70
# CLAUDE-C01-15 sources: raw Internet Archive captures, Internet Archive item copies, and the eGazette, Rajya Sabha
# (cms.rajyasabha.nic.in) and President's Secretariat (one static PDF attachment) file hosts, all accessed on 2026-09-24.
C01_15_HOSTS = {'web.archive.org', 'archive.org', 'egazette.gov.in', 'cms.rajyasabha.nic.in', 'www.presidentofindia.gov.in'}
C01_15_ACCESS_DATES = {'2026-09-24'}
C01_15_SOURCE_COUNT = 56
# CLAUDE-C01-20 sources: raw Internet Archive captures, the Rajya Sabha debates store and the Indian National Congress's own
# media store (res.cloudinary.com), all accessed on 2026-09-25; their rows belong to the party role on one recognition
# observation, which otherwise keeps its identity, lifecycle and empty game mapping.
C01_20_HOSTS = {'web.archive.org', 'bucketapi.rajyasabha.digital', 'res.cloudinary.com'}
C01_20_ACCESS_DATES = {'2026-09-25'}
C01_20_SOURCE_COUNT = 73
C01_20_ORGANIZATION = 'in_eci_20240323_np_05'
C01_20_ROLE = 'in_inc_president'
# CLAUDE-C01-27 sources: raw Internet Archive and Arquivo.pt captures, the Bharatiya Janata Party's own e-Library
# (library.bjp.org) and the Rajya Sabha debates store, all accessed on 2026-09-28; their rows belong to the party role on
# one recognition observation, which otherwise keeps its identity, lifecycle and empty game mapping.
C01_27_HOSTS = {'web.archive.org', 'arquivo.pt', 'library.bjp.org', 'bucketapi.rajyasabha.digital'}
C01_27_ACCESS_DATES = {'2026-09-28'}
C01_27_SOURCE_COUNT = 74
C01_27_ORGANIZATION = 'in_eci_20240323_np_03'
C01_27_ROLE = 'in_bjp_president'
# CLAUDE-C01-33 sources: the Rajya Sabha debates store and CMS, eGazette, stored Internet Archive copies of Gazette and
# Parliament Digital Library files, one raw Internet Archive capture, and the IFES Election Judgments copy of one Election
# Commission order, all accessed on 2026-09-28; their rows belong to the new Janata Dal recognition observation and its
# party role, which otherwise keeps an unresearched lifecycle and an empty game mapping.
C01_33_HOSTS = {'bucketapi.rajyasabha.digital', 'cms.rajyasabha.nic.in', 'egazette.gov.in', 'archive.org',
                'web.archive.org', 'electionjudgments.org'}
C01_33_ACCESS_DATES = {'2026-09-28'}
C01_33_SOURCE_COUNT = 22
C01_33_ORGANIZATION = 'in_eci_19980110_np_06'
C01_33_ROLE = 'in_jd_president'


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
        # 82 organization observations plus the CLAUDE-C01-11 prime-ministership (70 sources, 112 claims, one role) and the
        # CLAUDE-C01-15 presidency (56 sources, 84 claims, one role); CLAUDE-C01-20 adds no entry, only one party role on
        # the Indian National Congress recognition observation (73 sources, 113 claims), and CLAUDE-C01-27 likewise one
        # party role on the Bharatiya Janata Party recognition observation (74 sources, 167 claims). CLAUDE-C01-33 adds one
        # organization observation, the Janata Dal recognition row of 10 January 1998, with one party role (22 sources, 41
        # claims).
        self.assertEqual(tuple(len(ids[k]) for k in ('entries', 'sources', 'claims', 'roles')), (85, 297, 599, 5))
        self.assertEqual(len(self.packet['organizations']), 83)
        self.assertEqual(sum(len(s['claims']) for s in self.packet['sources'] if s['id'] in ORIGINAL_SOURCES), 82)
        coverage = self.packet['coverage']
        self.assertFalse(coverage['exhaustive_organization_register_reviewed'])
        self.assertIsNone(coverage['unrepresented_organization_total'])
        self.assertFalse(coverage['art_completion_claim'])
        self.assertEqual([r['records'] for r in coverage['bounded_registers']], [6, 76])
        self.assertEqual(coverage['bounded_registers'][1]['jurisdictions'], 26)
        # Exactly two institutions, the CLAUDE-C01-11 prime-ministership and then the CLAUDE-C01-15 presidency, each with
        # exactly one role; no other institution.
        self.assertEqual([entry['id'] for entry in self.packet['institutions']], ['in_prime_minister', 'in_presidency'])
        self.assertEqual([role['id'] for role in self.packet['institutions'][0]['roles']], ['in_pm'])
        self.assertEqual([role['id'] for role in self.packet['institutions'][1]['roles']], ['in_president'])
        # Exactly three organization roles, in observation order: the CLAUDE-C01-27 party role on the Bharatiya Janata
        # Party observation, the CLAUDE-C01-20 party role on the Indian National Congress observation and the CLAUDE-C01-33
        # party role on the Janata Dal observation.
        self.assertEqual([(entry['id'], role['id'], role['kind']) for entry in self.packet['organizations']
                          for role in entry['roles']], [(C01_27_ORGANIZATION, C01_27_ROLE, 'party_leader'),
                                                        (C01_20_ORGANIZATION, C01_20_ROLE, 'party_leader'),
                                                        (C01_33_ORGANIZATION, C01_33_ROLE, 'party_leader')])

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
        c01_15 = set(self.packet['institutions'][1]['sources'])
        self.assertEqual(len(c01_15), C01_15_SOURCE_COUNT)
        roles = {(o['id'], r['id']): r for o in self.packet['organizations'] for r in o['roles']}
        c01_20 = set(roles[(C01_20_ORGANIZATION, C01_20_ROLE)]['sources'])
        self.assertEqual(len(c01_20), C01_20_SOURCE_COUNT)
        c01_27 = set(roles[(C01_27_ORGANIZATION, C01_27_ROLE)]['sources'])
        self.assertEqual(len(c01_27), C01_27_SOURCE_COUNT)
        self.assertFalse(c01_20 & c01_27)
        # CLAUDE-C01-33's sources belong to its own observation; its party role cites a subset of them.
        c01_33 = set(next(o for o in self.packet['organizations'] if o['id'] == C01_33_ORGANIZATION)['sources'])
        c01_33_role = set(roles[(C01_33_ORGANIZATION, C01_33_ROLE)]['sources'])
        self.assertEqual(len(c01_33), C01_33_SOURCE_COUNT)
        self.assertTrue(c01_33_role < c01_33)
        self.assertFalse((c01_20 | c01_27) & c01_33)
        self.assertEqual({s for o in self.packet['organizations'] for r in o['roles'] for s in r['sources']},
                         c01_20 | c01_27 | c01_33_role)
        for source in self.packet['sources']:
            if source['id'] in ORIGINAL_SOURCES:
                self.assertEqual(urlsplit(source['url']).hostname, 'www.ceo.kerala.gov.in')
                self.assertEqual(source['published_date'], '2024-03-28')
                self.assertEqual(source['accessed_date'], '2026-09-13')
                self.assertLessEqual(date.fromisoformat(source['published_date']), date.fromisoformat(research.CUTOFF))
                self.assertEqual({c['attested_on'] for c in source['claims']}, {'2024-03-23'})
            elif source['id'] in c01_33:
                self.assertIn(urlsplit(source['url']).hostname, C01_33_HOSTS)
                self.assertIn(source['accessed_date'], C01_33_ACCESS_DATES)
                if source['published_date']:
                    self.assertLessEqual(date.fromisoformat(source['published_date']), date.fromisoformat(research.CUTOFF))
                # CLAUDE-C01-33's recollections, spans and undated statements carry no structured date.
                for claim in source['claims']:
                    if claim.get('attested_on'):
                        self.assertLessEqual(claim['attested_on'], research.CUTOFF)
            elif source['id'] in c01_27:
                self.assertIn(urlsplit(source['url']).hostname, C01_27_HOSTS)
                self.assertIn(source['accessed_date'], C01_27_ACCESS_DATES)
                if source['published_date']:
                    self.assertLessEqual(date.fromisoformat(source['published_date']), date.fromisoformat(research.CUTOFF))
                # CLAUDE-C01-27's multi-day meeting headings, spans, lists and undated statements carry no structured date.
                for claim in source['claims']:
                    if claim.get('attested_on'):
                        self.assertLessEqual(claim['attested_on'], research.CUTOFF)
            elif source['id'] in c01_20:
                self.assertIn(urlsplit(source['url']).hostname, C01_20_HOSTS)
                self.assertIn(source['accessed_date'], C01_20_ACCESS_DATES)
                if source['published_date']:
                    self.assertLessEqual(date.fromisoformat(source['published_date']), date.fromisoformat(research.CUTOFF))
                # CLAUDE-C01-20's retrospective spans, lists and undated statements carry no structured date at all.
                for claim in source['claims']:
                    if claim.get('attested_on'):
                        self.assertLessEqual(claim['attested_on'], research.CUTOFF)
            elif source['id'] in c01_15:
                self.assertIn(urlsplit(source['url']).hostname, C01_15_HOSTS)
                self.assertIn(source['accessed_date'], C01_15_ACCESS_DATES)
                if source['published_date']:
                    self.assertLessEqual(date.fromisoformat(source['published_date']), date.fromisoformat(research.CUTOFF))
                # CLAUDE-C01-15's retrospective spans carry no structured date at all.
                for claim in source['claims']:
                    if claim.get('attested_on'):
                        self.assertLessEqual(claim['attested_on'], research.CUTOFF)
            else:
                self.assertIn(urlsplit(source['url']).hostname, C01_11_HOSTS)
                self.assertIn(source['accessed_date'], C01_11_ACCESS_DATES)
                if source['published_date']:
                    self.assertLessEqual(date.fromisoformat(source['published_date']), date.fromisoformat(research.CUTOFF))
                # CLAUDE-C01-11's retrospective spans and one superseded Gazette text carry no structured date at all.
                for claim in source['claims']:
                    if claim.get('attested_on'):
                        self.assertLessEqual(claim['attested_on'], research.CUTOFF)
        self.assertEqual({urlsplit(s['url']).hostname for s in self.packet['sources']},
                         {'www.ceo.kerala.gov.in'} | C01_11_HOSTS | C01_15_HOSTS | C01_20_HOSTS | C01_27_HOSTS
                         | C01_33_HOSTS)
        self.assertEqual({urlsplit(s['url']).hostname for s in self.packet['sources'] if s['id'] in c01_33}, C01_33_HOSTS)
        self.assertEqual({urlsplit(s['url']).hostname for s in self.packet['sources'] if s['id'] in c01_15}, C01_15_HOSTS)
        self.assertEqual({urlsplit(s['url']).hostname for s in self.packet['sources'] if s['id'] in c01_20}, C01_20_HOSTS)
        self.assertEqual({urlsplit(s['url']).hostname for s in self.packet['sources'] if s['id'] in c01_27}, C01_27_HOSTS)
        p = copy.deepcopy(self.packet)
        p['organizations'][0]['recognition']['attested_on'] = '2026-09-08'
        with self.assertRaisesRegex(ValueError, 'exceeds cutoff'):
            self.validate(p)

    def test_recognition_does_not_grant_roles_lifespans_or_game_identity(self):
        party_roles = {C01_20_ORGANIZATION: (C01_20_ROLE, 'Indian National Congress'),
                       C01_27_ORGANIZATION: (C01_27_ROLE, 'Bharatiya Janata Party'),
                       C01_33_ORGANIZATION: (C01_33_ROLE, 'Janata Dal')}
        for entry in self.packet['organizations']:
            # Only the CLAUDE-C01-20, CLAUDE-C01-27 and CLAUDE-C01-33 party roles are added, one to each of three
            # observations; recognition grants no other role.
            if entry['id'] in party_roles:
                role_id, name = party_roles[entry['id']]
                self.assertEqual([(r['id'], r['kind']) for r in entry['roles']], [(role_id, 'party_leader')])
                self.assertEqual(entry['name'], name)
                self.assertEqual(entry['lifecycle']['status'], 'unresearched')
            else:
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
        c01_15 = set(self.packet['institutions'][1]['sources'])
        c01_20 = set(orgs[C01_20_ORGANIZATION]['roles'][0]['sources'])
        c01_27 = set(orgs[C01_27_ORGANIZATION]['roles'][0]['sources'])
        c01_33 = set(orgs[C01_33_ORGANIZATION]['sources'])
        self.assertEqual({s for o in self.packet['organizations'] for r in o['roles'] for s in r['sources']},
                         c01_20 | c01_27 | set(orgs[C01_33_ORGANIZATION]['roles'][0]['sources']))
        for source in self.packet['sources']:
            data = self.extracts[source['id']]
            self.assertEqual(data['format'], 'spheres-c01-derived-factual-table/v1')
            self.assertEqual(data['source_url'], source['url'])
            self.assertEqual({r['claim_id'] for r in data['rows']}, {c['id'] for c in source['claims']})
            if source['id'] in ORIGINAL_SOURCES:
                self.assertEqual(data['source_notification_date'], '2024-03-23')
                self.assertEqual(data['gazette_publication_date'], '2024-03-28')
                self.assertRegex(data['source_response_sha256'], r'^[0-9a-f]{64}$')
                self.assertGreater(data['source_response_bytes'], 300000)
                for row in data['rows']:
                    org = orgs[row['observation_id']]
                    self.assertEqual((org['name'], org['jurisdiction']), (row['name'], row['jurisdiction']))
                    self.assertEqual(org['recognition']['source_qualifications'], row['source_qualifications'])
            elif source['id'] in c01_33:
                # CLAUDE-C01-33 records every original response identity, pinned in
                # test_india_janata_dal_presidents_c01_33.py; its rows belong to the Janata Dal observation, either to its
                # party role or (role_id null) to the observation itself, and to no institution.
                self.assertIsInstance(data['source_response_bytes'], int)
                self.assertGreater(data['source_response_bytes'], 0)
                self.assertRegex(data['source_response_sha256'], r'^[0-9a-f]{64}$')
                self.assertNotEqual(data['source_response_sha256'], source['snapshot']['sha256'])
                self.assertIs(data['source_response_checked_in'], False)
                self.assertLessEqual({(row['observation_id'], row['role_id']) for row in data['rows']},
                                     {(C01_33_ORGANIZATION, C01_33_ROLE), (C01_33_ORGANIZATION, None)})
            elif source['id'] in c01_27:
                # CLAUDE-C01-27 records every original response identity, pinned in test_india_bjp_presidents_c01_27.py;
                # its rows belong to the party role on the Bharatiya Janata Party observation and to no institution.
                self.assertIsInstance(data['source_response_bytes'], int)
                self.assertGreater(data['source_response_bytes'], 0)
                self.assertRegex(data['source_response_sha256'], r'^[0-9a-f]{64}$')
                self.assertNotEqual(data['source_response_sha256'], source['snapshot']['sha256'])
                self.assertIs(data['source_response_checked_in'], False)
                self.assertEqual({(row['observation_id'], row['role_id']) for row in data['rows']},
                                 {(C01_27_ORGANIZATION, C01_27_ROLE)})
            elif source['id'] in c01_20:
                # CLAUDE-C01-20 records every original response identity, pinned in test_india_inc_presidents_c01_20.py;
                # its rows belong to the party role on the Indian National Congress observation and to no institution.
                self.assertIsInstance(data['source_response_bytes'], int)
                self.assertGreater(data['source_response_bytes'], 0)
                self.assertRegex(data['source_response_sha256'], r'^[0-9a-f]{64}$')
                self.assertNotEqual(data['source_response_sha256'], source['snapshot']['sha256'])
                self.assertIs(data['source_response_checked_in'], False)
                self.assertEqual({(row['observation_id'], row['role_id']) for row in data['rows']},
                                 {(C01_20_ORGANIZATION, C01_20_ROLE)})
            else:
                # CLAUDE-C01-11 and CLAUDE-C01-15 record every original response identity; the exact values are pinned in
                # test_india_prime_ministers_c01_11.py and test_india_presidents_c01_15.py, and the extract's own checksum
                # is a different file's. Their rows belong to the prime-ministership or the presidency institution
                # respectively, never to a recognition observation.
                self.assertIsInstance(data['source_response_bytes'], int)
                self.assertGreater(data['source_response_bytes'], 0)
                self.assertRegex(data['source_response_sha256'], r'^[0-9a-f]{64}$')
                self.assertNotEqual(data['source_response_sha256'], source['snapshot']['sha256'])
                self.assertIs(data['source_response_checked_in'], False)
                self.assertEqual({row['observation_id'] for row in data['rows']},
                                 {'in_presidency' if source['id'] in c01_15 else 'in_prime_minister'})
                self.assertFalse({row['observation_id'] for row in data['rows']} & set(orgs))
        self.assertEqual([s['id'] for s in self.packet['sources']][:2], list(ORIGINAL_SOURCES))
        self.assertEqual(len(self.packet['sources']),
                         len(ORIGINAL_SOURCES) + C01_11_SOURCE_COUNT + C01_15_SOURCE_COUNT + C01_20_SOURCE_COUNT
                         + C01_27_SOURCE_COUNT + C01_33_SOURCE_COUNT)
        self.assertEqual([s['id'] for s in self.packet['sources']],
                         list(ORIGINAL_SOURCES) + self.packet['institutions'][0]['sources']
                         + self.packet['institutions'][1]['sources'] + orgs[C01_20_ORGANIZATION]['roles'][0]['sources']
                         + orgs[C01_27_ORGANIZATION]['roles'][0]['sources'] + orgs[C01_33_ORGANIZATION]['sources'])
        self.assertEqual(len(c01_33), C01_33_SOURCE_COUNT)
        self.assertEqual((len(self.packet['institutions'][0]['sources']), len(c01_15)),
                         (C01_11_SOURCE_COUNT, C01_15_SOURCE_COUNT))
        p = copy.deepcopy(self.packet)
        p['sources'][0]['snapshot']['sha256'] = '0' * 64
        with self.assertRaisesRegex(ValueError, 'checksum mismatch'):
            self.validate(p)

    def test_discovery_batches_remain_open_and_separate_from_campaign_certification(self):
        index = research.build()
        india = next(p for p in index['countries'] if p['nation'] == 'India')
        self.assertFalse(india['country_census_complete'])
        # 83 recognition observations (82 of 23 March 2024 and CLAUDE-C01-33's Janata Dal row of 10 January 1998), the
        # CLAUDE-C01-11 prime-ministership and the CLAUDE-C01-15 presidency, none mapped to a game identity.
        self.assertEqual(india['mapping_pending'], 85)
        self.assertEqual(self.packet['institutions'][0]['represented_party_ids'], [])
        self.assertEqual(self.packet['institutions'][1]['represented_party_ids'], [])
        self.assertFalse(index['runtime_roster_modified'])
        self.assertFalse(index['c01_complete'])
        self.assertFalse(index['g2_prerequisite_satisfied'])
        work = [w for w in index['work_orders'] if w['nation'] == 'India']
        self.assertEqual([len(w['members']) for w in work], [10] * 8 + [5])
        self.assertEqual(work[-1]['members'], ['in_eci_20240323_sp_26_01', 'in_eci_20240323_sp_26_02', C01_33_ORGANIZATION,
                                               'in_prime_minister', 'in_presidency'])
        self.assertEqual({m for w in work for m in w['members']}, set(self.validate()['entries']))
        self.assertEqual({w['status'] for w in work}, {'open'})


if __name__ == '__main__':
    unittest.main()
