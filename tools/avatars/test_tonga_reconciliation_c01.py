"""CLAUDE-C01-01: the Tonga reconciliation keeps label matches apart from legal identity and office terms."""
import copy
import hashlib
import json
import re
import unittest
from datetime import date
from urllib.parse import urlsplit

import campaign_research as research


REC_SOURCES = {
    'to_sc_kiu_v_tuionetoa_20220429': 'ago.gov.to',
    'to_ipu_2021': 'data.ipu.org',
    'to_ipu_2017': 'data.ipu.org',
}
REC_CLAIMS = {
    'to_paati_a_e_kakai_name_20211001',
    'to_peoples_party_incorporated_society_2022',
    'to_speech_transcript_ptoa_20211001',
    'to_ipu_2021_ptoa_expansion',
    'to_ipu_2021_tuionetoa_tpp',
    'to_ipu_2021_sovaleni_assembly_selection',
    'to_ipu_2017_dpfi_continuity',
}
REPORT = research.RESEARCH / 'tonga-reconciliation-01.md'


class TongaReconciliationTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.packet = json.loads((research.ROOT / research.RESEARCH / 'tonga.json').read_text(encoding='utf-8'))
        cls.sources = {s['id']: s for s in cls.packet['sources']}
        cls.claims = {c['id']: c for s in cls.packet['sources'] for c in s['claims']}
        cls.entries = {e['id']: e for category in ('organizations', 'institutions') for e in cls.packet[category]}
        cls.extracts = {
            sid: json.loads((research.ROOT / cls.sources[sid]['snapshot']['path']).read_text(encoding='utf-8'))
            for sid in REC_SOURCES
        }
        cls.report = (research.ROOT / REPORT).read_text(encoding='utf-8')

    def validate(self, packet=None):
        return research.validate(packet or self.packet, research.ROOT, {'Tonga'}, {'Tonga': set()})

    def test_new_records_are_bounded_and_every_claim_is_cited(self):
        ids = self.validate()
        self.assertEqual({c['id'] for sid in REC_SOURCES for c in self.sources[sid]['claims']}, REC_CLAIMS)
        self.assertLessEqual(REC_CLAIMS, set(ids['claims']))
        self.assertEqual(len(ids['entries']), 9)
        cited = {cid for e in self.entries.values() for cid in e['claim_ids']}
        self.assertLessEqual(REC_CLAIMS, cited)
        observations = re.findall(r'^### (TO-REC-\d\d)\b', self.report, re.M)
        self.assertEqual(observations, [f'TO-REC-{n:02d}' for n in range(1, 11)])
        for entry in self.entries.values():
            for rec in entry.get('identity_reconciliation', {}).get('observations', []):
                self.assertIn(f'### {rec} ', self.report)

    def test_patoa_is_a_variant_printing_and_the_court_record_is_not_rewritten(self):
        dpfi = self.entries['to_dpfi']
        variant, = [n for n in dpfi['name_observations'] if n['name'] == 'PATOA']
        self.assertEqual(variant['variant_of'], 'PTOA')
        self.assertEqual(variant['sources'], ['to_court_peoples_party_20220809'])
        transcript = self.claims['to_speech_transcript_ptoa_20211001']
        self.assertIn('spells the label PTOA', transcript['text'])
        self.assertIn('does not expand the acronym', transcript['uncertainty'])
        # The appellate claim keeps saying what the source says: its own labels disagree.
        speech = self.claims['to_tuionetoa_prior_party_speech_20211001']
        self.assertIn('PATOA in paragraph 21 and PTOA in paragraph 32', speech['text'])
        self.assertIn('unreconciled', speech['uncertainty'])
        self.assertEqual(self.sources['to_court_peoples_party_20220809']['snapshot']['sha256'],
                         'b2a8ebb81eb5e587c324f45500c7fcf6c7e94270f9e9cd628f631b0096fc3e49')

    def test_ptoa_grouping_with_dpfi_is_not_a_legal_identity_or_a_pdp_match(self):
        dpfi = self.entries['to_dpfi']
        link = dpfi['identity_reconciliation']
        self.assertEqual(link['status'], 'provisional_observation_grouping')
        self.assertFalse(link['automatic_merge'])
        self.assertIsNone(link['reconciled_organization_id'])
        self.assertIn('not a legal identity', link['note'])
        self.assertIn('to_ipu_2017_dpfi_continuity', dpfi['claim_ids'])
        self.assertIsNone(dpfi['lifecycle']['from'])
        self.assertEqual(dpfi['roles'][0]['holder_claims'], ['to_dpfi_pohiva_2010'])
        pdp = self.entries['to_pdp']
        self.assertEqual(pdp['claim_ids'], ['to_pdp_split_fuko', 'to_2008_party_candidacy', 'to_pdp_constitution_archive'])
        self.assertNotIn('identity_reconciliation', pdp)
        for entry in self.entries.values():
            self.assertNotIn('to_2010_ambiguous_democratic', entry['claim_ids'])
        self.assertIn('unresolved and uncounted', self.packet['coverage']['unresolved'][2])

    def test_society_president_is_a_separate_role_from_leader_with_no_term(self):
        party = self.entries['to_peoples_party']
        by_kind = {}
        for role in party['roles']:
            by_kind.setdefault(role['kind'], []).append(role['id'])
        self.assertEqual(by_kind, {'party_leader': ['to_peoples_party_leader'],
                                   'other': ['to_peoples_party_society_president', 'to_peoples_party_society_secretary']})
        for role in party['roles'][1:]:
            holder, = role['holder_claims']
            self.assertIsNone(holder['attested_on'])
            self.assertIsNone(holder['from'])
            self.assertIsNone(holder['until'])
            self.assertEqual(holder['attested_period'], {'from': '2022-04-19', 'through': '2022-04-21'})
            self.assertEqual(holder['claim_ids'], ['to_peoples_party_incorporated_society_2022'])
        claim = self.claims['to_peoples_party_incorporated_society_2022']
        self.assertIn('35 members', claim['text'])
        self.assertIn('not a statutory party registration', claim['uncertainty'])
        self.assertIn('No registration date', claim['uncertainty'])
        self.assertIsNone(party['lifecycle']['from'])
        self.assertIn('without a registration date', party['lifecycle']['note'])

    def test_set_aside_trial_judgment_supplies_identity_evidence_only(self):
        source = self.sources['to_sc_kiu_v_tuionetoa_20220429']
        self.assertIn('set aside', source['scope_note'])
        self.assertIn('no allegation, finding, declaration, penalty or office exclusion', source['scope_note'])
        for claim in source['claims']:
            self.assertNotRegex(claim['text'].lower(), r'brib|void|tokoni|\$')
        national = [r for e in self.packet['institutions'] for r in e['roles']]
        for role in national:
            self.assertNotIn('to_sc_kiu_v_tuionetoa_20220429', role['sources'])

    def test_assembly_selection_is_recorded_but_never_becomes_a_holder(self):
        role, = self.entries['to_prime_minister']['roles']
        selection = self.claims['to_ipu_2021_sovaleni_assembly_selection']
        self.assertEqual(selection['attested_on'], '2021-12-15')
        self.assertIn('Prime Minister Designate', selection['text'])
        self.assertIn(selection['id'], role['claim_ids'])
        self.assertNotIn(selection['id'], role['holder_claims'])
        names = [h['name'] for h in role['holder_claims'] if isinstance(h, dict)]
        self.assertNotIn('Siaosi Sovaleni', names)
        self.assertTrue(any('TO-REC-10' in u for u in self.entries['to_prime_minister']['coverage']['unresolved']))

    def test_extract_provenance_does_not_claim_unreproducible_hashes(self):
        for sid, host in REC_SOURCES.items():
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual(urlsplit(source['url']).hostname, host)
            self.assertEqual(source['accessed_date'], '2026-09-21')
            self.assertEqual(extract['source_url'], source['url'])
            self.assertEqual(extract['claims'], source['claims'])
            self.assertEqual(extract['scope_note'], source['scope_note'])
            self.assertFalse(extract['source_response_checked_in'])
            self.assertEqual(source['snapshot']['kind'], 'derived_factual_extract')
            data = (research.ROOT / source['snapshot']['path']).read_bytes()
            self.assertEqual(hashlib.sha256(data).hexdigest(), source['snapshot']['sha256'])
            self.assertIn('No open license, portrait permission or likeness approval', extract['rights_note'])
        court = self.extracts['to_sc_kiu_v_tuionetoa_20220429']
        self.assertEqual((court['source_response_bytes'], court['source_response_sha256']),
                         (602620, '57fb2ac33421699958da3267e9d0540fa533ff2f944c36c022d5535aa3012372'))
        self.assertEqual(court['visual_review']['pdf_pages_one_based'], [1, 2, 4, 33, 34])
        self.assertLessEqual(date.fromisoformat(self.sources['to_sc_kiu_v_tuionetoa_20220429']['published_date']),
                             date.fromisoformat(research.CUTOFF))
        for sid in ('to_ipu_2021', 'to_ipu_2017'):
            self.assertIsNone(self.extracts[sid]['source_response_sha256'])
            self.assertNotIn('published_date', self.sources[sid])

    def test_mutations_are_rejected(self):
        packet = copy.deepcopy(self.packet)
        next(s for s in packet['sources'] if s['id'] == 'to_ipu_2021')['snapshot']['sha256'] = '0' * 64
        with self.assertRaisesRegex(ValueError, 'checksum mismatch'):
            self.validate(packet)
        packet = copy.deepcopy(self.packet)
        party = next(e for e in packet['organizations'] if e['id'] == 'to_peoples_party')
        party['roles'][1]['holder_claims'][0]['attested_period']['through'] = '2026-09-08'
        with self.assertRaisesRegex(ValueError, 'exceeds cutoff'):
            self.validate(packet)
        packet = copy.deepcopy(self.packet)
        party = next(e for e in packet['organizations'] if e['id'] == 'to_peoples_party')
        party['roles'][1]['holder_claims'][0]['claim_ids'] = ['to_ipu_2021_tuionetoa_tpp']
        with self.assertRaisesRegex(ValueError, 'cited source'):
            self.validate(packet)

    def test_report_is_ready_for_review_and_closes_nothing(self):
        self.assertIn('ready_for_review', self.report)
        for marker in ('C01', 'C06', 'S23', 'WC1', 'CP1'):
            self.assertNotRegex(self.report, rf'\b{marker}\b[^.\n]*\bis (now )?complete\b')
        index = research.build()
        country = next(p for p in index['countries'] if p['nation'] == 'Tonga')
        self.assertFalse(country['country_census_complete'])
        self.assertIsNone(country['unrepresented_organization_count'])
        batch, = [row for row in index['work_orders'] if row['nation'] == 'Tonga']
        self.assertEqual(batch['status'], 'open')


if __name__ == '__main__':
    unittest.main()
