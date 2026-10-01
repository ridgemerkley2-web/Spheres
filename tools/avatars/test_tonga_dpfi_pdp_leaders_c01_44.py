"""CLAUDE-C01-44: Tonga's opposition parties (DPFI/PTOA and PDP), 1990-2026. New records are claims only: founding
statements, a court recital under a different party name, descriptions of legal form and a government statement never
become holders, dates, lifecycle boundaries or name mappings, and the existing leader and president holders stay as
they were."""
import copy
import hashlib
import json
import re
import unittest
from urllib.parse import urlsplit

import campaign_research as research


REPORT = research.RESEARCH / 'tonga-dpfi-pdp-leaders-1990-2026-44.md'
HANDOFF = 'docs/campaign-certification/C01/reviews/CLAUDE-C01-44-resumed-20261001/submission-handoff.md'
# Original response identity recorded in each extract: (bytes, sha256) of the body, in packet order.
RESPONSES = {
    'to_la_news_pga_award_20131214': (35057, 'bcaa0769b9b25cd07350815a25f43b7d8cd67e669749b3dc2b9c451eb1918918'),
    'to_tlr_2014_pohiva_v_tuivakano': (1510292, 'b302db6f8386835e807b31c657d55c3612121e5f04c17a107c3632fee542916e'),
    'to_ca_psa_pohiva_v_kot_20150916': (2668960, '53307c4ec539ca82b3eb9fb9d6d5dced6f60c40816977a5a08fffefef314554b'),
    'to_la_profile_pohiva_2016': (33380, '84404948cbd1e7983016bdc25d4b1ea6a8ee5eff5686b665b4dc7d4ee29399b1'),
    'to_pmo_20200317_ptoa_reply': (74793, '082584197664175c38d1003b03384bbc6bb7ade06ba46f2cd6f7cbf3e2998d2d'),
}
NEW_SOURCES = set(RESPONSES)
ORDER = list(RESPONSES)
# Raw Internet Archive captures: source id -> capture timestamp. The two court records are AGO downloads.
ARCHIVED = {
    'to_la_news_pga_award_20131214': '20161027095552',
    'to_la_profile_pohiva_2016': '20160304030700',
    'to_pmo_20200317_ptoa_reply': '20200703103038',
}
DIRECT = {'to_tlr_2014_pohiva_v_tuivakano', 'to_ca_psa_pohiva_v_kot_20150916'}
PDF_PAGES = {'to_ca_psa_pohiva_v_kot_20150916': [3, 21]}
# claim -> (source, attested_on, period, event kind, review observation, holder name as printed; None where the source
# names no holder of any office)
CLAIMS = {
    'to_la_pga_dpfi_established_sept2010': (
        'to_la_news_pga_award_20131214', None, {'from': '2010-09-01', 'through': '2010-09-30'},
        'retrospective_founding_statement', 'TO-OPP-01', None),
    'to_tlr_pohiva_leader_tonga_democratic_party_20140117': (
        'to_tlr_2014_pohiva_v_tuivakano', '2014-01-17', None, 'court_recital_of_party_office', 'TO-OPP-02', 'Mr Pohiva'),
    'to_ca_fidp_unincorporated_body_20150916': (
        'to_ca_psa_pohiva_v_kot_20150916', '2015-09-16', None, 'court_description_of_legal_form', 'TO-OPP-03', None),
    'to_ca_fidp_party_funds_argument_20150916': (
        'to_ca_psa_pohiva_v_kot_20150916', '2015-09-16', None, 'court_description_of_legal_form', 'TO-OPP-03', None),
    'to_la_profile_dpfi_established_2010': (
        'to_la_profile_pohiva_2016', None, None, 'retrospective_founding_statement', 'TO-OPP-01', None),
    'to_pmo_ptoa_unregistered_statement_20200317': (
        'to_pmo_20200317_ptoa_reply', '2020-03-17', None, 'government_statement_on_legal_form', 'TO-OPP-03', None),
}
NEW_CLAIMS = list(CLAIMS)
# Party names printed by the new sources and the 2011 Assembly lead that differ from every name observed for to_dpfi:
# No identity reconciliation is established by these sources; keep all four variants unmapped.
UNMAPPED_NAMES = ('Tonga Democratic Party', 'Friendly Islands Democratic Party', 'Friendly Island Democratic Party',
                  'Friendly Island Democratic Party (FIDP)')
# The existing holders of the three roles in scope, unchanged by this packet.
LEADER_2014 = {
    'name': "'Akilisi Pohiva", 'attested_on': '2014-11-27', 'from': None, 'until': None, 'sources': ['to_ipu_2014'],
    'claim_ids': ['to_ipu_2014_dpfi_pohiva_leader'],
}
PRESIDENT_2022 = {
    'name': 'Fatai Helu', 'attested_on': '2022-08-29', 'from': None, 'until': None,
    'sources': ['to_sc_helu_piukala_v_ec_20220829'], 'claim_ids': ['to_ptoa_president_helu_20220829'],
}
NOTES_DPFI = ['TO-OPP-01 (tonga-dpfi-pdp-leaders-1990-2026-44.md)', 'TO-OPP-02', 'TO-OPP-03', 'TO-OPP-04', 'TO-OPP-05']
NOTES_PDP = ['TO-OPP-06 (tonga-dpfi-pdp-leaders-1990-2026-44.md)', 'TO-OPP-07']
EVENT_KINDS = {'retrospective_founding_statement', 'court_recital_of_party_office', 'court_description_of_legal_form',
               'government_statement_on_legal_form'}
LEAD_MARKERS = ('wikipedia', 'matangi', 'kaniva', 'talanoa', 'rnz', 'tonga independent', 'core team', "people's team",
                'pmn.co.nz', 'scoop.co.nz', 'devpolicy', 'pireport', 'aceproject', 'senituli', 'teisa')


def opposition_invariants(packet):
    """The rules this packet relies on, as one function the mutation tests can break."""
    sources = {s['id']: s for s in packet['sources']}
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    entries = {e['id']: e for category in ('organizations', 'institutions') for e in packet[category]}
    roles = {r['id']: r for e in entries.values() for r in e['roles']}
    dpfi, pdp = entries['to_dpfi'], entries['to_pdp']
    # New claims are org-level claims on to_dpfi only: no role, holder or other entry cites them.
    for rid, role in roles.items():
        assert not set(NEW_CLAIMS) & set(role['claim_ids']), f'new claim placed on role {rid}'
        for holder in role['holder_claims']:
            ids = [holder] if isinstance(holder, str) else holder.get('claim_ids', [])
            assert not set(NEW_CLAIMS) & set(ids), f'new claim used as holder evidence on {rid}'
    for eid, entry in entries.items():
        if eid != 'to_dpfi':
            assert not set(NEW_CLAIMS) & set(entry['claim_ids']), f'new claim placed on {eid}'
    # The existing holders stay exactly as they were.
    leader = roles['to_dpfi_leader']['holder_claims']
    assert len(leader) == 2 and leader[0] == 'to_dpfi_pohiva_2010', 'to_dpfi_leader holders changed'
    assert {k: leader[1].get(k) for k in LEADER_2014} == LEADER_2014, 'to_dpfi_leader holders changed'
    president = roles['to_dpfi_president']['holder_claims']
    assert len(president) == 1 and {k: president[0].get(k) for k in PRESIDENT_2022} == PRESIDENT_2022, \
        'to_dpfi_president holders changed'
    assert roles['to_pdp_leader']['holder_claims'] == ['to_pdp_split_fuko'], 'to_pdp_leader holders changed'
    assert dpfi['claim_ids'][-len(NEW_CLAIMS):] == NEW_CLAIMS, 'new claims missing from to_dpfi'
    assert dpfi['sources'][-len(ORDER):] == ORDER, 'new sources missing from to_dpfi'
    # Founding statements and legal-form descriptions date nothing on the organization.
    for entry in (dpfi, pdp):
        assert entry['lifecycle']['from'] is None and entry['lifecycle']['until'] is None, 'lifecycle boundary added'
    names = [n['name'] for n in dpfi.get('name_observations', [])]
    for name in UNMAPPED_NAMES:
        assert name not in names, f'name mapped without a ruling: {name}'
    # Dates exactly as the sources state them; no claim of this packet carries from/until.
    for cid, (sid, attested, period, *_rest) in CLAIMS.items():
        claim = claims[cid]
        assert cid in [c['id'] for c in sources[sid]['claims']], f'{cid} moved'
        assert claim.get('attested_on') == attested, f'{cid} date changed'
        assert claim.get('period') == period, f'{cid} period changed'
        assert 'from' not in claim and 'until' not in claim, f'{cid} carries a boundary'


class TongaOppositionPartyTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.raw = (research.ROOT / research.RESEARCH / 'tonga.json').read_text(encoding='utf-8')
        cls.packet = json.loads(cls.raw)
        cls.sources = {s['id']: s for s in cls.packet['sources']}
        cls.claims = {c['id']: c for s in cls.packet['sources'] for c in s['claims']}
        cls.entries = {e['id']: e for category in ('organizations', 'institutions') for e in cls.packet[category]}
        cls.roles = {r['id']: r for e in cls.entries.values() for r in e['roles']}
        cls.extracts = {
            sid: json.loads((research.ROOT / cls.sources[sid]['snapshot']['path']).read_text(encoding='utf-8'))
            for sid in NEW_SOURCES
        }
        cls.report = (research.ROOT / REPORT).read_text(encoding='utf-8')

    def validate(self, packet=None):
        return research.validate(packet or self.packet, research.ROOT, {'Tonga'}, {'Tonga': set()})

    def section(self, heading):
        parts = self.report.split(f'\n## {heading}', 1)
        self.assertEqual(len(parts), 2, heading)
        return parts[1].split('\n## ', 1)[0]

    def test_new_records_are_bounded_and_follow_earlier_packets(self):
        ids = self.validate()
        self.assertLessEqual(NEW_SOURCES, set(ids['sources']))
        new_claims = [c['id'] for sid in ORDER for c in self.sources[sid]['claims']]
        self.assertEqual(new_claims, NEW_CLAIMS)
        self.assertEqual((len(ORDER), len(new_claims)), (5, 6))
        # The new sources follow CLAUDE-C01-36's, which end at index 207.
        self.assertEqual([s['id'] for s in self.packet['sources']][206], 'to_pmo_20211213_maafu_death')
        self.assertEqual([s['id'] for s in self.packet['sources'][207:207 + len(ORDER)]], ORDER)
        self.assertEqual(len(ids['entries']), 9)
        self.assertEqual({e['id'] for e in self.packet['organizations']}, {'to_fihrdm', 'to_pdp', 'to_dpfi', 'to_peoples_party'})
        self.assertEqual([r['id'] for r in self.entries['to_dpfi']['roles']], ['to_dpfi_leader', 'to_dpfi_president'])
        self.assertEqual([r['id'] for r in self.entries['to_pdp']['roles']], ['to_pdp_leader'])
        observations = re.findall(r'^### (TO-OPP-\d\d)\b', self.report, re.M)
        self.assertEqual(observations, [f'TO-OPP-{n:02d}' for n in range(1, 8)])
        for cid in NEW_CLAIMS:
            claim = self.claims[cid]
            if claim.get('attested_on'):
                self.assertTrue('1990-01-01' <= claim['attested_on'] <= research.CUTOFF, cid)
        for sid in ORDER:
            self.assertLessEqual(self.sources[sid].get('published_date', ''), research.CUTOFF)
            self.assertEqual(self.sources[sid]['accessed_date'], '2026-09-30')

    def test_existing_holders_unchanged_and_new_claims_never_hold_office(self):
        opposition_invariants(self.packet)
        # Nobody is a holder because of this packet: the names printed by the new sources appear in no holder of the
        # three roles in scope (the state-office holders of other packets keep their own names).
        for rid in ('to_dpfi_leader', 'to_dpfi_president', 'to_pdp_leader'):
            for holder in self.roles[rid]['holder_claims']:
                if isinstance(holder, dict):
                    self.assertNotIn(holder['name'], {'Mr Pohiva', 'Mr. Pohiva', "Hon. Samiuela 'Akilisi Pohiva",
                                                      "Samuela 'Akilisi Pohiva"})
        self.assertIn('not a to_dpfi_leader holder observation',
                      self.claims['to_tlr_pohiva_leader_tonga_democratic_party_20140117']['uncertainty'])
        self.assertIn('names no party office', self.claims['to_ca_fidp_unincorporated_body_20150916']['uncertainty'])
        self.assertIn('names no PTOA officer', self.claims['to_pmo_ptoa_unregistered_statement_20200317']['uncertainty'])
        for cid in ('to_la_pga_dpfi_established_sept2010', 'to_la_profile_dpfi_established_2010'):
            self.assertIn('lifecycle.from stays null', self.claims[cid]['uncertainty'])
            self.assertIn("not read as holding the leader's office" if 'pga' in cid else
                          "not treated as holding the leader's office", self.claims[cid]['uncertainty'])

    def test_republished_text_is_typed_as_non_primary(self):
        types = {sid: self.sources[sid]['source_type'] for sid in ORDER}
        self.assertEqual(types, {
            'to_la_news_pga_award_20131214': 'legislature_republished_reference_text',
            'to_tlr_2014_pohiva_v_tuivakano': 'primary_court_record',
            'to_ca_psa_pohiva_v_kot_20150916': 'primary_court_record',
            'to_la_profile_pohiva_2016': 'primary_legislature_member_profile_archived',
            'to_pmo_20200317_ptoa_reply': 'primary_government_release_archived',
        })
        self.assertIn('PGA Forum website', self.claims['to_la_pga_dpfi_established_sept2010']['text'])
        self.assertIn('never holder evidence', self.claims['to_la_pga_dpfi_established_sept2010']['uncertainty'])

    def test_extracts_match_packet_claims_and_record_original_responses(self):
        for sid in NEW_SOURCES:
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual(extract['format'], 'spheres-c01-derived-factual-table/v1')
            self.assertEqual((extract['source_id'], extract['source_url']), (sid, source['url']))
            self.assertEqual(extract['scope_note'], source['scope_note'])
            self.assertEqual(extract['access_method'], source['access_method'])
            self.assertEqual(extract['published_date'], source.get('published_date'))
            self.assertEqual(extract['accessed_date'], source['accessed_date'])
            self.assertFalse(extract['source_response_checked_in'])
            self.assertEqual((extract['source_response_bytes'], extract['source_response_sha256']), RESPONSES[sid])
            self.assertEqual(extract['source_response_content_encoding'], 'identity')
            self.assertIn("'Accept-Encoding: identity'", extract['fetch_recipe'])
            self.assertIn('not checked into this repository', extract['provenance_note'])
            self.assertIn('No open license, portrait permission or likeness approval', extract['rights_note'])
            self.assertEqual(extract['visual_review']['pdf_pages_one_based'], PDF_PAGES.get(sid, []))
            gap = re.search(r'(\d+) minutes later', extract['stability_check'])
            self.assertTrue(gap and int(gap.group(1)) >= 30, sid)
            self.assertEqual([r['claim_id'] for r in extract['rows']], [c['id'] for c in source['claims']])
            for row, claim in zip(extract['rows'], source['claims']):
                _, attested, _, kind, review, holder = CLAIMS[claim['id']]
                self.assertNotIn('name', row)
                self.assertEqual((row['observation_id'], row['role_id']), ('to_dpfi', None))
                self.assertIn('not placed on a role', row['role_title'])
                self.assertEqual((row['event_kind'], row['review_observation'], row['holder_name']), (kind, review, holder))
                self.assertIn(row['event_kind'], EVENT_KINDS)
                self.assertEqual((row['text'], row['locator'], row['uncertainty'], row['attested_on']),
                                 (claim['text'], claim['locator'], claim['uncertainty'], claim.get('attested_on')))
                if holder:
                    self.assertIn(holder, claim['text'])
            snapshot = source['snapshot']
            self.assertEqual(snapshot['kind'], 'derived_factual_extract')
            self.assertTrue(snapshot['path'].startswith('docs/campaign-certification/C01/research/sources/tonga-'))
            data = (research.ROOT / snapshot['path']).read_bytes()
            self.assertEqual((len(data), hashlib.sha256(data).hexdigest()), (snapshot['bytes'], snapshot['sha256']))
            self.assertTrue(data.endswith(b'}\n') and b'\r' not in data)
            self.assertEqual(data.decode('utf-8'), json.dumps(extract, indent=2, ensure_ascii=False) + '\n')
            self.assertIn('Organization identities stay unresolved', extract['bounded_scope'])

    def test_source_addresses_are_stable_and_pre_cutoff(self):
        self.assertEqual(set(ARCHIVED) | DIRECT, NEW_SOURCES)
        for sid, stamp in ARCHIVED.items():
            source, extract = self.sources[sid], self.extracts[sid]
            url = urlsplit(source['url'])
            self.assertEqual(url.hostname, 'web.archive.org')
            self.assertTrue(url.path.startswith(f'/web/{stamp}id_/http'), sid)
            self.assertLess(stamp, '20260907')
            self.assertEqual(extract['original_url'], source['original_url'])
            self.assertEqual(extract['archive_capture_utc'].replace('-', '').replace(':', '').replace('T', '').rstrip('Z'), stamp)
            self.assertIn('Raw Internet Archive capture', extract['provenance_note'])
            self.assertIn(urlsplit(source['original_url']).hostname, {'parliament.gov.to', 'www.parliament.gov.to', 'pmo.gov.to'})
        for sid in DIRECT:
            url = urlsplit(self.sources[sid]['url'])
            self.assertEqual(url.hostname, 'ago.gov.to')
            self.assertRegex(url.query, r'^download=\d+:')
            self.assertNotIn('original_url', self.sources[sid])
        for sid in NEW_SOURCES:
            url = self.sources[sid]['url']
            for marker in ('?start=', 'search', 'limit=', 'tmpl=component', 'print=1', 'switch_to_desktop', '/tag/', '/page/'):
                self.assertNotIn(marker, url, sid)
        originals = [s.get('original_url', s['url']) for s in self.packet['sources'] if s['id'] not in NEW_SOURCES]
        for sid in NEW_SOURCES:
            self.assertNotIn(self.sources[sid].get('original_url', self.sources[sid]['url']), originals, sid)

    def test_party_and_state_office_stay_separate(self):
        for eid in ('to_prime_minister', 'to_cabinet', 'to_legislative_assembly', 'to_peoples_party', 'to_pdp', 'to_fihrdm'):
            self.assertFalse(set(NEW_CLAIMS) & set(self.entries[eid]['claim_ids']), eid)
            self.assertFalse(NEW_SOURCES & set(self.entries[eid]['sources']), eid)
        for rid in ('to_pm', 'to_deputy_pm', 'to_ministers', 'to_peoples_representatives', 'to_dpfi_leader',
                    'to_dpfi_president', 'to_pdp_leader'):
            self.assertFalse(NEW_SOURCES & set(self.roles[rid]['sources']), rid)
        # The PMO release's statement about another party's registration is not imported.
        self.assertNotIn('Incorporated Society', self.sources['to_pmo_20200317_ptoa_reply']['claims'][0]['text'])
        self.assertIn("another party's registration", self.sources['to_pmo_20200317_ptoa_reply']['scope_note'])

    def test_coverage_notes_are_appended_after_existing_ones(self):
        dpfi = self.entries['to_dpfi']['coverage']['unresolved']
        pdp = self.entries['to_pdp']['coverage']['unresolved']
        self.assertEqual(len(dpfi), 18)
        self.assertTrue(dpfi[12].startswith('TO-DPFI-08'))
        for note, prefix in zip(dpfi[13:], NOTES_DPFI):
            self.assertTrue(note.startswith(prefix), prefix)
        self.assertEqual(len(pdp), 6)
        self.assertTrue(pdp[3].startswith('Continuation, dissolution and inactive periods'))
        for note, prefix in zip(pdp[4:], NOTES_PDP):
            self.assertTrue(note.startswith(prefix), prefix)
        packet_notes = self.packet['coverage']['unresolved']
        self.assertEqual(len(packet_notes), 16)
        self.assertTrue(packet_notes[15].startswith('Opposition party packet 44 (tonga-dpfi-pdp-leaders-1990-2026-44.md)'))
        self.assertIn('It adds no holder, organization, role or name observation', packet_notes[15])

    def test_leads_stay_out_of_the_packet(self):
        lowered = self.raw.lower()
        for marker in LEAD_MARKERS:
            self.assertNotIn(marker, lowered, marker)
        leads, added = self.section('Leads not imported'), self.section('Sources added')
        for marker in ('matangitonga.to', 'kanivatonga.co.nz', 'rnz.co.nz', 'talanoaotonga.to', 'wikipedia.org',
                       '161-sitiveni-halapua', '215-friendly-island-democratic-party'):
            self.assertIn(marker, leads, marker)
            self.assertNotIn(marker, added, marker)
        attempted = self.section('Sources attempted')
        for marker in ('ptoa.org', 'idcpc', 'download=1611', 'natlib', 'Tonga LR'):
            self.assertIn(marker, attempted, marker)

    def test_packet_formatting_is_preserved(self):
        data = (research.ROOT / research.RESEARCH / 'tonga.json').read_bytes()
        self.assertNotIn(b'\r', data)
        self.assertEqual(data.decode('utf-8'), json.dumps(self.packet, indent=2, ensure_ascii=False) + '\n')

    def test_mutations_are_rejected(self):
        def mutated(change):
            packet = copy.deepcopy(self.packet)
            change(packet)
            return packet

        def entry(packet, eid):
            return next(e for e in packet['organizations'] if e['id'] == eid)

        def role(packet, rid):
            return next(r for e in packet['organizations'] for r in e['roles'] if r['id'] == rid)

        def claim(packet, cid):
            return next(c for s in packet['sources'] for c in s['claims'] if c['id'] == cid)

        cases = {
            'used as holder evidence': lambda p: role(p, 'to_dpfi_leader')['holder_claims'].append({
                'name': 'Mr Pohiva', 'attested_on': '2014-01-17', 'from': None, 'until': None,
                'sources': ['to_tlr_2014_pohiva_v_tuivakano'],
                'claim_ids': ['to_tlr_pohiva_leader_tonga_democratic_party_20140117']}),
            'placed on role': lambda p: role(p, 'to_dpfi_leader')['claim_ids'].append(
                'to_tlr_pohiva_leader_tonga_democratic_party_20140117'),
            'lifecycle boundary added': lambda p: entry(p, 'to_dpfi')['lifecycle'].update({'from': '2010-09-01'}),
            'name mapped without a ruling': lambda p: entry(p, 'to_dpfi')['name_observations'].append(
                {'name': 'Friendly Islands Democratic Party', 'attested_on': '2015-09-16'}),
            'carries a boundary': lambda p: claim(p, 'to_la_profile_dpfi_established_2010').update({'from': '2010-01-01'}),
            'date changed': lambda p: claim(p, 'to_pmo_ptoa_unregistered_statement_20200317').update(
                {'attested_on': '2020-03-18'}),
            'period changed': lambda p: claim(p, 'to_la_pga_dpfi_established_sept2010').update(
                {'period': {'from': '2010-01-01', 'through': '2010-12-31'}}),
            'to_dpfi_president holders changed': lambda p: role(p, 'to_dpfi_president')['holder_claims'][0].update(
                {'until': '2025-11-20'}),
            'to_pdp_leader holders changed': lambda p: role(p, 'to_pdp_leader')['holder_claims'].append(
                {'name': 'Tesina Fuko', 'attested_on': '2005-04-15', 'from': None, 'until': None,
                 'sources': ['to_ipu_2008'], 'claim_ids': ['to_pdp_split_fuko']}),
            'placed on to_pdp': lambda p: entry(p, 'to_pdp')['claim_ids'].append('to_ca_fidp_party_funds_argument_20150916'),
        }
        for expected, change in cases.items():
            with self.subTest(mutation=expected):
                with self.assertRaisesRegex(AssertionError, re.escape(expected.split(' on ')[0])):
                    opposition_invariants(mutated(change))
        # A snapshot that no longer matches its extract is rejected by the shared validator.
        packet = mutated(lambda p: next(s for s in p['sources'] if s['id'] == ORDER[0])['snapshot'].update(
            {'sha256': '0' * 64}))
        with self.assertRaisesRegex(ValueError, 'Source snapshot checksum mismatch'):
            self.validate(packet)

    def test_review_preserves_existing_primary_observation_and_does_not_infer_authority(self):
        self.assertIn('existing primary 2022 Fatai Helu presidency observation remains unchanged', self.report)
        self.assertIn('not independently verified or treated', self.report)
        self.assertIn('as user authorization', self.report)
        self.assertNotIn('decided by Ridge', self.report)
        self.assertIn('government assertion, not an objective legal-status finding', self.report)

    def test_submitted_report_and_handoff_close_no_parent_gate(self):
        self.assertIn('ready_for_review', self.report)
        table = self.section('Outcome')
        decisions = {'01': 'Unresolved', '02': 'Claims only', '03': 'Claims only', '04': 'Unresolved',
                     '05': 'Unresolved', '06': 'Unresolved', '07': 'Unresolved'}
        for number, decision in decisions.items():
            row, = [line for line in table.splitlines() if line.startswith(f'| TO-OPP-{number} ')]
            self.assertIn(f'**{decision}', row)
        for marker in ('C01', 'C06', 'S23', 'WC1', 'CP1'):
            self.assertNotRegex(self.report, rf'\b{marker}\b[^.\n]*\bis (now )?complete\b')
        notes = self.section('Integration notes')
        for text in ('fdb71175', '509bd289', 'research-index.json', 'test_tonga_research_s10g.py', 'census.json',
                     'government.rs', 'continue existing claims first'):
            self.assertIn(text, notes)
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        for text in ('ready_for_review', 'tonga-dpfi-pdp-leaders-1990-2026-44.md', 'claude/c01-to-44', 'fdb71175',
                     '509bd289', 'test_tonga_dpfi_pdp_leaders_c01_44.py'):
            self.assertIn(text, handoff)
        self.assertNotIn('State: **claimed**', handoff)
        index = research.build()
        country = next(p for p in index['countries'] if p['nation'] == 'Tonga')
        self.assertFalse(country['country_census_complete'])
        self.assertFalse(index['c01_complete'])


if __name__ == '__main__':
    unittest.main()
