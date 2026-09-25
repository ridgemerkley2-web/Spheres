"""Protect dated Brazilian funding and naming observations from invented histories."""
import copy
import json
import unittest
from collections import Counter
from datetime import date
from urllib.parse import urlsplit

import campaign_research as research
from test_brazil_vice_presidents_c01_17 import NEW_SOURCES as C01_17_SOURCES

# The five sources of the original S10f intake keep every assertion below by id. CLAUDE-C01-10 adds 48 sources for
# the br_presidency institution whose response identities are pinned exactly (bytes and SHA-256) in
# test_brazil_presidents_c01_10.py. CLAUDE-C01-17 then adds 11 sources and the second role br_vice_president; its
# identities are pinned exactly in test_brazil_vice_presidents_c01_17.py.
ORIGINAL_SOURCES = ('br_tse_fefc_2024', 'br_tse_fefc_announcement_20240617', 'br_tse_missao_20251104',
                    'br_tse_pmb_rename_20251202', 'br_trerj_pmb_name_notices')
# CLAUDE-C01-10 sources: raw Internet Archive captures and the Senate, Chamber and TSE document services, all accessed
# on 2026-09-23.
C01_10_HOSTS = {'web.archive.org', 'legis.senado.leg.br', 'imagem.camara.leg.br', 'bibliotecadigital.tse.jus.br',
                'www12.senado.leg.br'}
C01_10_SOURCE_COUNT = 48
# CLAUDE-C01-17 sources: raw Internet Archive captures and the Senate and TSE document services, accessed on
# 2026-09-24.
C01_17_HOSTS = {'web.archive.org', 'legis.senado.leg.br', 'bibliotecadigital.tse.jus.br'}
C01_17_SOURCE_COUNT = 11


class BrazilDiscoveryTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.packet = json.loads((research.ROOT / research.RESEARCH / 'brazil.json').read_text(encoding='utf-8'))
        cls.extracts = {
            source['id']: json.loads((research.ROOT / source['snapshot']['path']).read_text(encoding='utf-8'))
            for source in cls.packet['sources']
        }

    def validate(self, packet=None):
        return research.validate(packet or self.packet, research.ROOT, {'Brazil'}, {'Brazil': set()})

    def test_partial_inventory_is_valid_without_claiming_31_distinct_parties(self):
        ids = self.validate()
        # 31 organization observations plus the presidency: CLAUDE-C01-10 (48 sources, 112 claims, br_president) and
        # CLAUDE-C01-17 (11 sources, 85 claims, br_vice_president).
        self.assertEqual(tuple(len(ids[k]) for k in ('entries', 'sources', 'claims', 'roles')), (32, 64, 231, 2))
        self.assertEqual(len(self.packet['organizations']), 31)
        coverage = self.packet['coverage']
        self.assertFalse(coverage['exhaustive_organization_register_reviewed'])
        self.assertIsNone(coverage['unrepresented_organization_total'])
        self.assertFalse(coverage['art_completion_claim'])
        self.assertEqual([row['records'] for row in coverage['bounded_registers']], [29, 1, 1])
        self.assertIsNone(coverage['bounded_registers'][0]['registry_as_of_date'])
        # Exactly one institution, the presidency, with exactly two roles (CLAUDE-C01-10's br_president and
        # CLAUDE-C01-17's br_vice_president); no other institution.
        self.assertEqual([entry['id'] for entry in self.packet['institutions']], ['br_presidency'])
        self.assertEqual([role['id'] for role in self.packet['institutions'][0]['roles']],
                         ['br_president', 'br_vice_president'])

    def test_all_funding_rows_retain_source_order_and_exact_labels(self):
        rows = self.extracts['br_tse_fefc_2024']['rows']
        self.assertEqual([row['source_row'] for row in rows], list(range(1, 30)))
        self.assertEqual([row['party_label'] for row in rows], [
            'MDB', 'PDT', 'PT', 'PCdoB', 'PSB', 'PSDB', 'AGIR', 'MOBILIZA',
            'CIDADANIA', 'PV', 'AVANTE', 'PSTU', 'PCB', 'PRTB', 'DC', 'PCO',
            'PODEMOS', 'REPUBLICANOS', 'PSOL', 'PL', 'PSD', 'SOLIDARIEDADE',
            'NOVO', 'REDE', 'PMB', 'UP', 'UNIÃO', 'PRD', 'PROGRESSISTAS'])
        self.assertEqual(rows[-1]['criteria_case_number_as_displayed'], '0612998.05.2024.6.00.0000')
        self.assertTrue(all(row['original_full_name'] is None for row in rows))
        entries = {entry['id']: entry for entry in self.packet['organizations']}
        for row in rows:
            entry = entries[row['observation_id']]
            self.assertEqual(entry['name'], row['party_label'])
            self.assertEqual(entry['source_identifier']['value'], row['criteria_case_number_as_displayed'])
            self.assertIsNone(entry['expanded_name'])

    def test_dashes_remain_unknown_and_are_not_nonpayment_or_inactivity(self):
        rows = self.extracts['br_tse_fefc_2024']['rows']
        self.assertEqual({row['party_label'] for row in rows if row['release_date'] is None}, {'PCB', 'PMB'})
        self.assertEqual(Counter(row['release_date'] for row in rows), {
            '2024-08-20': 23, '2024-08-29': 2, '2024-08-23': 1,
            '2024-09-26': 1, None: 2})
        entries = {entry['id']: entry for entry in self.packet['organizations']}
        for row in rows:
            observation = entries[row['observation_id']]['funding_observation']
            self.assertEqual(observation['attested_on'], row['release_date'])
            self.assertIsNone(observation['amount'])
            self.assertIsNone(observation['consolidated_current_status'])
            if row['release_date'] is None:
                self.assertEqual(row['release_cell_as_displayed'], '-')

    def test_aggregate_announcement_does_not_backdate_later_release_rows(self):
        table = next(source for source in self.packet['sources'] if source['id'] == 'br_tse_fefc_2024')
        self.assertIsNone(table['published_date'])
        announcement = self.extracts['br_tse_fefc_announcement_20240617']
        self.assertEqual(announcement['reported_party_total'], 29)
        self.assertEqual(announcement['attested_on'], '2024-06-17')
        self.assertNotIn('2024-06-17', {claim['attested_on'] for claim in table['claims']})
        self.assertEqual(self.extracts['br_tse_fefc_2024']['table_read_method'],
                         'Complete HTML table text read; no image, linked process document or PDF reviewed.')

    def test_missao_registration_does_not_reuse_ptb_identity_or_grant_a_leader(self):
        entry = next(e for e in self.packet['organizations'] if e['id'] == 'br_tse_missao_registration_2025')
        self.assertEqual(entry['recognition']['attested_on'], '2025-11-04')
        self.assertEqual(entry['recognition']['electoral_number'], 14)
        self.assertEqual(entry['roles'], [])
        self.assertIsNone(entry['reconciled_organization_id'])
        self.assertIsNone(entry['recognition']['from'])
        self.assertIsNone(entry['recognition']['until'])

    def test_pmb_name_discrepancy_and_later_rectification_survive(self):
        entry = next(e for e in self.packet['organizations'] if e['id'] == 'br_pmb_name_decisions_2025_2026')
        self.assertEqual([(o['name'], o['attested_on']) for o in entry['name_observations']], [
            ('O DEMOCRATA', '2025-12-02'), ('Democrata', '2025-12-02'), ('DEMOCRATA', '2026-02-12')])
        link = entry['identity_reconciliation']
        self.assertEqual(link['candidate_observation_id'], 'br_tse_fefc_2024_party_25')
        self.assertFalse(link['automatic_merge'])
        self.assertEqual(link['status'], 'pending')
        self.assertIsNone(entry['reconciled_organization_id'])

    def test_access_and_update_dates_never_extend_historical_attestations(self):
        for source in self.packet['sources']:
            if source['id'] in ORIGINAL_SOURCES:
                self.assertIn(urlsplit(source['url']).hostname, {'www.tse.jus.br', 'www.tre-rj.jus.br'})
                self.assertEqual(source['accessed_date'], '2026-09-13')
            elif source['id'] in C01_17_SOURCES:
                self.assertIn(urlsplit(source['url']).hostname, C01_17_HOSTS)
                self.assertEqual(source['accessed_date'], '2026-09-24')
            else:
                self.assertIn(urlsplit(source['url']).hostname, C01_10_HOSTS)
                self.assertEqual(source['accessed_date'], '2026-09-23')
            if source['published_date']:
                self.assertLessEqual(date.fromisoformat(source['published_date']), date.fromisoformat(research.CUTOFF))
            for claim in source['claims']:
                if source['id'] in ORIGINAL_SOURCES:
                    self.assertIn('attested_on', claim)
                # CLAUDE-C01-10's three retrospective library spans carry no structured date at all.
                if claim.get('attested_on'):
                    self.assertLessEqual(claim['attested_on'], research.CUTOFF)
        self.assertEqual({urlsplit(s['url']).hostname for s in self.packet['sources']},
                         {'www.tse.jus.br', 'www.tre-rj.jus.br'} | C01_10_HOSTS)
        packet = copy.deepcopy(self.packet)
        packet['sources'][0]['claims'][0]['attested_on'] = '2026-09-08'
        with self.assertRaisesRegex(ValueError, 'exceeds cutoff'):
            self.validate(packet)

    def test_factual_snapshots_reconcile_claims_without_invented_original_hashes(self):
        for source in self.packet['sources']:
            extract = self.extracts[source['id']]
            self.assertEqual(extract['source_url'], source['url'])
            self.assertEqual(extract['format'], 'spheres-c01-derived-factual-table/v1')
            if source['id'] in ORIGINAL_SOURCES:
                self.assertIsNone(extract['source_response_bytes'])
                self.assertIsNone(extract['source_response_sha256'])
            else:
                # CLAUDE-C01-10 records every original response identity; the exact values are pinned in
                # test_brazil_presidents_c01_10.py, and the extract's own checksum is a different file's.
                self.assertIsInstance(extract['source_response_bytes'], int)
                self.assertGreater(extract['source_response_bytes'], 0)
                self.assertRegex(extract['source_response_sha256'], r'^[0-9a-f]{64}$')
                self.assertNotEqual(extract['source_response_sha256'], source['snapshot']['sha256'])
                self.assertIs(extract['source_response_checked_in'], False)
            found = {row['claim_id'] for row in extract['rows']} if 'rows' in extract else {claim['id'] for claim in extract['claims']}
            self.assertEqual(found, {claim['id'] for claim in source['claims']})
        self.assertEqual([s['id'] for s in self.packet['sources']][:5], list(ORIGINAL_SOURCES))
        self.assertEqual(len(self.packet['sources']), len(ORIGINAL_SOURCES) + C01_10_SOURCE_COUNT + C01_17_SOURCE_COUNT)
        self.assertEqual([s['id'] for s in self.packet['sources']][-C01_17_SOURCE_COUNT:], C01_17_SOURCES)
        packet = copy.deepcopy(self.packet)
        packet['sources'][0]['snapshot']['sha256'] = '0' * 64
        with self.assertRaisesRegex(ValueError, 'checksum mismatch'):
            self.validate(packet)

    def test_reconciliation_never_grants_game_identity_or_closes_certification(self):
        for entry in self.packet['organizations']:
            self.assertEqual(entry['represented_party_ids'], [])
            self.assertEqual(entry['roles'], [])
            self.assertIsNone(entry['lifecycle']['from'])
            self.assertIsNone(entry['lifecycle']['until'])
        packet = copy.deepcopy(self.packet)
        packet['organizations'][0]['represented_party_ids'] = ['Brazil/guessed_mdb']
        with self.assertRaisesRegex(ValueError, 'foreign represented party mapping'):
            self.validate(packet)
        index = research.build()
        country = next(p for p in index['countries'] if p['nation'] == 'Brazil')
        self.assertFalse(country['country_census_complete'])
        # 31 organization observations and the CLAUDE-C01-10 presidency, none mapped to a game identity.
        self.assertEqual(country['mapping_pending'], 32)
        self.assertEqual(self.packet['institutions'][0]['represented_party_ids'], [])
        self.assertFalse(index['runtime_roster_modified'])
        self.assertFalse(index['c01_complete'])
        self.assertFalse(index['g2_prerequisite_satisfied'])
        work = [row for row in index['work_orders'] if row['nation'] == 'Brazil']
        self.assertEqual([len(row['members']) for row in work], [10, 10, 10, 2])
        self.assertEqual(work[-1]['members'], ['br_pmb_name_decisions_2025_2026', 'br_presidency'])
        self.assertEqual({member for row in work for member in row['members']}, set(self.validate()['entries']))
        self.assertEqual({row['status'] for row in work}, {'open'})


if __name__ == '__main__':
    unittest.main()
