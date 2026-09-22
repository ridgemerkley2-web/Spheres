"""Keep Tonga party observations distinct from office appointments and full histories."""
import copy
import json
import unittest
from datetime import date
from urllib.parse import urlsplit

import campaign_research as research


NEW_SOURCES = {
    'to_court_peoples_party_20220809',
    'to_idcpc_dialogue_20210528',
    'to_pm_appointment_20191008',
}


class TongaDiscoveryTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.packet = json.loads((research.ROOT / research.RESEARCH / 'tonga.json').read_text(encoding='utf-8'))
        cls.sources = {s['id']: s for s in cls.packet['sources']}
        cls.claims = {c['id']: c for s in cls.packet['sources'] for c in s['claims']}
        cls.entries = {e['id']: e for category in ('organizations', 'institutions') for e in cls.packet[category]}
        cls.extracts = {
            sid: json.loads((research.ROOT / cls.sources[sid]['snapshot']['path']).read_text(encoding='utf-8'))
            for sid in NEW_SOURCES
        }

    def validate(self, packet=None):
        return research.validate(packet or self.packet, research.ROOT, {'Tonga'}, {'Tonga': set()})

    def test_new_observation_does_not_close_country_or_relabel_other_parties(self):
        ids = self.validate()
        # CLAUDE-C01-01 added three sources and seven claims; test_tonga_reconciliation_c01 owns them.
        # CLAUDE-C01-02 added seven sources and nine claims; test_tonga_transition_c01_02 owns them.
        # CLAUDE-C01-03 added seventeen sources and 27 claims; test_tonga_transition_c01_03 owns them.
        self.assertEqual((len(ids['entries']), len(ids['sources']), len(ids['claims'])), (9, 43, 71))
        self.assertEqual({e['id'] for e in self.packet['organizations']},
                         {'to_fihrdm', 'to_pdp', 'to_dpfi', 'to_peoples_party'})
        self.assertEqual(len(self.packet['institutions']), 5)
        self.assertEqual(sum(len(self.sources[sid]['claims']) for sid in NEW_SOURCES), 5)
        coverage = self.packet['coverage']
        self.assertFalse(coverage['exhaustive_organization_register_reviewed'])
        self.assertIsNone(coverage['unrepresented_organization_total'])
        self.assertFalse(coverage['art_completion_claim'])
        self.assertIn('unresolved and uncounted', coverage['unresolved'][2])

    def test_event_host_title_is_a_dated_leader_observation_only(self):
        party = self.entries['to_peoples_party']
        # Society President/Secretary roles are separate 'other' offices, never a second leader.
        role, = [r for r in party['roles'] if r['kind'] == 'party_leader']
        self.assertEqual((role['title'], role['kind']), ('Leader', 'party_leader'))
        self.assertEqual(role['sources'], ['to_idcpc_dialogue_20210528'])
        holder, = role['holder_claims']
        self.assertEqual(holder['name'], "Pohiva Tu'i'onetoa")
        self.assertEqual(holder['attested_on'], '2021-05-28')
        self.assertIsNone(holder['from'])
        self.assertIsNone(holder['until'])
        self.assertEqual(holder['claim_ids'], ['to_peoples_party_leader_20210528'])
        self.assertEqual(self.sources['to_idcpc_dialogue_20210528']['source_type'], 'primary_event_organizer_account')
        self.assertNotIn('chair', role['title'].lower())

    def test_court_event_and_judgment_dates_and_party_label_discrepancy_survive(self):
        source = self.sources['to_court_peoples_party_20220809']
        self.assertEqual(source['published_date'], '2022-08-09')
        self.assertEqual({c['attested_on'] for c in source['claims']}, {'2021-10-01'})
        speech = self.claims['to_tuionetoa_prior_party_speech_20211001']
        self.assertIn('PATOA in paragraph 21 and PTOA in paragraph 32', speech['text'])
        self.assertIn('unreconciled', speech['uncertainty'])
        self.assertIn('allows the appeal', source['scope_note'])
        self.assertIn('no allegation, conviction, penalty or office exclusion', source['scope_note'])
        link = self.entries['to_peoples_party']['identity_reconciliation']
        self.assertFalse(link['automatic_merge'])
        self.assertIsNone(link['reconciled_organization_id'])
        self.assertEqual(link['status'], 'provisional_observation_grouping')

    def test_assembly_selection_never_becomes_the_royal_appointment(self):
        recommendation = self.claims['to_tuionetoa_assembly_recommendation_20190927']
        appointment = self.claims['to_tuionetoa_royal_appointment_20191008']
        self.assertEqual(recommendation['attested_on'], '2019-09-27')
        self.assertEqual(appointment['attested_on'], '2019-10-08')
        self.assertIn('15-8', recommendation['text'])
        role, = self.entries['to_prime_minister']['roles']
        # CLAUDE-C01-02 added exactly one later holder (Sovaleni, from 2021-12-27) and CLAUDE-C01-03 one more
        # (Eke, appointment event on 2025-01-22); the 2019 one stays first.
        dict_holders = [h for h in role['holder_claims'] if isinstance(h, dict)]
        self.assertEqual([h['name'] for h in dict_holders],
                         ["Pohiva Tu'i'onetoa", "Siaosi 'Ofakivahafolau Sovaleni", "'Aisake Valu Eke"])
        new_holder = dict_holders[0]
        self.assertEqual(new_holder['attested_on'], '2019-10-08')
        self.assertEqual(new_holder['claim_ids'], [appointment['id']])
        self.assertIsNone(new_holder['from'])
        self.assertIsNone(new_holder['until'])
        self.assertNotIn(recommendation['id'], role['holder_claims'])
        self.assertNotIn('to_pm_appointment_20191008', self.entries['to_peoples_party']['roles'][0]['sources'])

    def test_dated_labels_do_not_create_a_lifespan_or_merge_peoples_democratic_party(self):
        party = self.entries['to_peoples_party']
        self.assertEqual([(n['name'], n['attested_on']) for n in party['name_observations']], [
            ("Tonga People's Party", '2021-05-28'), ("People's Party", '2021-10-01'),
            ("Paati 'a e Kakai; the People's Party", '2021-10-01'), ("Tonga People's Party (TPP)", '2021-11-18')])
        self.assertIsNone(party['lifecycle']['from'])
        self.assertIsNone(party['lifecycle']['until'])
        self.assertIn('do not establish a founding date', party['lifecycle']['note'])
        self.assertIn('remains a separate entry', party['identity_reconciliation']['note'])
        self.assertEqual(self.entries['to_pdp']['name'], "People's Democratic Party (PDP)")
        self.assertEqual(self.entries['to_pdp']['roles'][0]['holder_claims'], ['to_pdp_split_fuko'])

    def test_original_response_provenance_is_distinct_from_derived_snapshot_checksums(self):
        expected = {
            'to_court_peoples_party_20220809': (307617, 'ff8af75772e2f6dd0f3eaaa0b300239bc2925a867dff8f9926142299dee90973'),
            'to_idcpc_dialogue_20210528': (53986, '969d688a9fbb6c80753e2526bd1d42dff5210b6c58aff222e0c982a730d7f470'),
            'to_pm_appointment_20191008': (181073, '4c33fc2a358ac9f2427b70e4f2ad4bd00ead8a5d7907be11e75327eca5e4a1d6'),
        }
        for sid, extract in self.extracts.items():
            source = self.sources[sid]
            self.assertEqual(extract['source_url'], source['url'])
            self.assertEqual(extract['claims'], source['claims'])
            self.assertEqual(extract['scope_note'], source['scope_note'])
            self.assertEqual((extract['source_response_bytes'], extract['source_response_sha256']), expected[sid])
            self.assertFalse(extract['source_response_checked_in'])
            self.assertEqual(source['snapshot']['kind'], 'derived_factual_extract')
            self.assertNotEqual(source['snapshot']['sha256'], extract['source_response_sha256'])
        packet = copy.deepcopy(self.packet)
        next(s for s in packet['sources'] if s['id'] in NEW_SOURCES)['snapshot']['sha256'] = '0' * 64
        with self.assertRaisesRegex(ValueError, 'checksum mismatch'):
            self.validate(packet)

    def test_visual_review_scope_is_specific_and_does_not_claim_portrait_review(self):
        self.assertEqual(self.extracts['to_court_peoples_party_20220809']['visual_review']['pdf_pages_one_based'], [1, 8, 9, 12, 15])
        self.assertEqual(self.extracts['to_pm_appointment_20191008']['visual_review']['pdf_pages_one_based'], [1])
        self.assertEqual(self.extracts['to_idcpc_dialogue_20210528']['visual_review']['pdf_pages_one_based'], [])
        for extract in self.extracts.values():
            self.assertIn('No open license, portrait permission or likeness approval', extract['rights_note'])

    def test_later_access_does_not_move_historical_cutoff(self):
        for sid in NEW_SOURCES:
            source = self.sources[sid]
            self.assertIn(urlsplit(source['url']).hostname, {'ago.gov.to', 'www.idcpc.org.cn', 'pmo.gov.to'})
            self.assertEqual(source['accessed_date'], '2026-09-13')
            self.assertLessEqual(date.fromisoformat(source['published_date']), date.fromisoformat(research.CUTOFF))
        packet = copy.deepcopy(self.packet)
        next(e for e in packet['organizations'] if e['id'] == 'to_peoples_party')['roles'][0]['holder_claims'][0]['attested_on'] = '2026-09-08'
        with self.assertRaisesRegex(ValueError, 'exceeds cutoff'):
            self.validate(packet)

    def test_research_cannot_add_game_mapping_or_complete_the_discovery_batch(self):
        for entry in self.entries.values():
            self.assertEqual(entry['represented_party_ids'], [])
        packet = copy.deepcopy(self.packet)
        packet['organizations'][-1]['represented_party_ids'] = ['Tonga/guessed_peoples_party']
        with self.assertRaisesRegex(ValueError, 'foreign represented party mapping'):
            self.validate(packet)
        index = research.build()
        self.assertFalse(index['c01_complete'])
        self.assertFalse(index['g2_prerequisite_satisfied'])
        self.assertFalse(index['runtime_roster_modified'])
        country = next(p for p in index['countries'] if p['nation'] == 'Tonga')
        self.assertFalse(country['country_census_complete'])
        self.assertEqual(country['mapping_pending'], 9)
        batch, = [row for row in index['work_orders'] if row['nation'] == 'Tonga']
        self.assertEqual(batch['status'], 'open')
        self.assertEqual(set(batch['members']), set(self.entries))


if __name__ == '__main__':
    unittest.main()
