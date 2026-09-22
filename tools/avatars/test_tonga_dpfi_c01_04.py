"""CLAUDE-C01-04: Tonga's DPFI/PTOA identity, leadership and 2025 record keep every event, office and lead apart."""
import copy
import hashlib
import json
import re
import unittest
from urllib.parse import urlsplit

import campaign_research as research


# Original response identity recorded in each extract: (bytes, sha256); None where it is not reproducible.
RESPONSES = {
    'to_sc_helu_piukala_v_ec_20220829': (348680, 'a5f84dcfa45b40af7cc2a5db28ccd47eff6d3d43f5581c9c733e8876cc8681c9'),
    'to_ca_helu_piukala_v_ec_20230406': (246435, '4148871de69a8a13f240554579e692e25712b3f6d0f0f2789556d4b3adbdfa65'),
    'to_sc_tuivakano_v_police_20211028': (1288307, 'af4f8fe0e4d2db8536ab8fe136d6db62e0a2ae407871a2276d3c5903ab593c08'),
    'to_ipu_2014': (15787, None),
    'to_assembly_byelection_20191119': (54685, None),
    'to_assembly_siaosi_oath_20200511': (54069, None),
    'to_mcctil_incsoc_consultation_20260721': (985683, 'd70f817469fcbfa77e721ae5d4fe270638d7c8d645be1db93277b6f13efcb757'),
    'to_ipu_2025': (164916, None),
    'to_tec_results_2025': (8171403, 'fe27cd3c700ca2e4ca5f11c6ab22c9a8c4d87882aaf471f55f741b5145381f88'),
    'to_tec_home_2025': (410934, 'dcbbc7eae13f81b7f7808183d86b459a434609956b856fd7a7a834cf39678677'),
    'to_sc_fasi_v_sika_20260518': (5984803, '8b8a29bcc418d2aab2ac4637956e63e8ae4d398544f3fd769cf0689063c8f2d9'),
}
NEW_SOURCES = set(RESPONSES)
IPU_2014_NORMALISED = (14849, 'c3b08924ed7547489f2dd744e5f69c1f28370f61f6095a5f602178d4ca0620d7')
HOSTS = {
    'to_sc_helu_piukala_v_ec_20220829': 'ago.gov.to',
    'to_ca_helu_piukala_v_ec_20230406': 'ago.gov.to',
    'to_sc_tuivakano_v_police_20211028': 'ago.gov.to',
    'to_sc_fasi_v_sika_20260518': 'ago.gov.to',
    'to_ipu_2014': 'data.ipu.org',
    'to_ipu_2025': 'data.ipu.org',
    'to_assembly_byelection_20191119': 'parliament.gov.to',
    'to_assembly_siaosi_oath_20200511': 'parliament.gov.to',
    'to_mcctil_incsoc_consultation_20260721': 'www.businessregistries.gov.to',
    'to_tec_results_2025': 'elections.gov.to',
    'to_tec_home_2025': 'elections.gov.to',
}
NEW_CLAIMS = {
    'to_ptoa_tongan_name_20220829', 'to_ptoa_2021_candidates_20211118', 'to_ptoa_president_helu_20220829',
    'to_ptoa_self_described_objective_20220829', 'to_ptoa_applicants_individuals_20220829',
    'to_ptoa_democratic_party_20230406', 'to_ptoa_pohiva_campaign_2017',
    'to_ipu_2014_dpfi_pohiva_leader', 'to_ipu_2014_pohiva_assembly_selection_20141229',
    'to_ipu_2014_royal_endorsement_following_day', 'to_ipu_2014_cabinet_took_office_20150119',
    'to_pohiva_death_month_2019', 'to_tongatapu1_byelection_notice_20191119',
    'to_siaosi_pohiva_byelection_win_201911', 'to_siaosi_pohiva_oath_20200511',
    'to_incsoc_paper_register_20260721', 'to_ipu_2025_no_party_result',
    'to_tec_2025_tongatapu1', 'to_tec_2025_tongatapu2', 'to_tec_2025_no_party_column',
    'to_tec_2025_candidate_list_no_party', 'to_fasi_v_sika_poll_result_20251120',
}
PM_2014 = ('to_ipu_2014_pohiva_assembly_selection_20141229', 'to_ipu_2014_royal_endorsement_following_day',
           'to_ipu_2014_cabinet_took_office_20150119')
SEAT_EVENTS = ('to_pohiva_death_month_2019', 'to_tongatapu1_byelection_notice_20191119',
               'to_siaosi_pohiva_byelection_win_201911', 'to_siaosi_pohiva_oath_20200511')
HOLDER_CLAIMS = {'to_ipu_2014_dpfi_pohiva_leader', 'to_ptoa_president_helu_20220829'}
# Secondary, tertiary and unimported primary leads that must stay in the report, never in the packet.
LEAD_HOSTS = ('wikipedia.org', 'kanivatonga.co.nz', 'tongaindependent.com', 'matangitonga.to', 'abc.net.au',
              'aceproject.org', 'idea.int', 'state.gov', 'natlib.govt.nz/records/search')
REPORT = research.RESEARCH / 'tonga-dpfi-04.md'
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-04.md'


class TongaDpfiTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.raw = (research.ROOT / research.RESEARCH / 'tonga.json').read_text(encoding='utf-8')
        cls.packet = json.loads(cls.raw)
        cls.sources = {s['id']: s for s in cls.packet['sources']}
        cls.claims = {c['id']: c for s in cls.packet['sources'] for c in s['claims']}
        cls.claim_source = {c['id']: s['id'] for s in cls.packet['sources'] for c in s['claims']}
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

    def holder_ids(self):
        for role in self.roles.values():
            for entry in role['holder_claims']:
                yield role['id'], entry, ([entry] if isinstance(entry, str) else entry['claim_ids'])

    def test_new_records_are_bounded_reuse_ids_and_every_claim_is_cited(self):
        ids = self.validate()
        self.assertLessEqual(NEW_SOURCES, set(ids['sources']))
        # to_ipu_2025 is shared with CLAUDE-C01-03, which owns its to_ipu_2025_transition claim.
        self.assertEqual({c['id'] for sid in NEW_SOURCES for c in self.sources[sid]['claims']},
                         NEW_CLAIMS | {'to_ipu_2025_transition'})
        cited = {cid for e in self.entries.values() for cid in e['claim_ids']}
        self.assertLessEqual(NEW_CLAIMS, cited)
        # No new organization or institution: the existing to_dpfi entry and its leader role are reused.
        self.assertEqual(len(ids['entries']), 9)
        self.assertEqual({e['id'] for e in self.packet['organizations']}, {'to_fihrdm', 'to_pdp', 'to_dpfi', 'to_peoples_party'})
        self.assertEqual([r['id'] for r in self.entries['to_dpfi']['roles']], ['to_dpfi_leader', 'to_dpfi_president'])
        observations = re.findall(r'^### (TO-DPFI-\d\d)\b', self.report, re.M)
        self.assertEqual(observations, [f'TO-DPFI-{n:02d}' for n in range(1, 9)])
        # Existing IPU records are reused, not duplicated or rewritten.
        for year, sid in (('2010', 'to_ipu_2010'), ('2017', 'to_ipu_2017'), ('2021', 'to_ipu_2021')):
            self.assertEqual([s['id'] for s in self.packet['sources'] if 'data.ipu.org' in s['url'] and
                              (f'E{year}' in s['url'] or f'2317_{year[2:]}' in s['url'])], [sid])
        self.assertIn('to_ipu_2021_ptoa_expansion', self.entries['to_dpfi']['claim_ids'])
        self.assertIn('to_ipu_2017_dpfi_continuity', self.entries['to_dpfi']['claim_ids'])
        for sid in NEW_SOURCES:
            self.assertEqual(urlsplit(self.sources[sid]['url']).hostname, HOSTS[sid])
            self.assertTrue(self.sources[sid]['source_type'])
        self.assertEqual({self.sources[sid]['source_type'] for sid in NEW_SOURCES}, {
            'primary_court_record', 'interparliamentary_election_record', 'primary_legislature_news_notice',
            'primary_registry_consultation_notice', 'primary_electoral_commission_record'})

    def test_selection_endorsement_cabinet_death_vacancy_win_and_oath_dates_never_collapse(self):
        leader, selection, endorsement, cabinet = (self.claims[c] for c in (
            'to_ipu_2014_dpfi_pohiva_leader',) + PM_2014)
        self.assertEqual((leader['attested_on'], selection['attested_on'], cabinet['attested_on']),
                         ('2014-11-27', '2014-12-29', '2015-01-19'))
        # 'The following day' stays relative: no derived 30 December date is stored anywhere.
        self.assertNotIn('attested_on', endorsement)
        self.assertNotIn('period', endorsement)
        self.assertIn("'the following day'", endorsement['text'])
        self.assertIn('30 December 2014 would be derived', endorsement['uncertainty'])
        self.assertNotIn('2014-12-30', self.raw)
        self.assertEqual(len({leader['attested_on'], selection['attested_on'], cabinet['attested_on']}), 3)
        death, notice, win, oath = (self.claims[c] for c in SEAT_EVENTS)
        self.assertEqual(death['period'], {'from': '2019-09-01', 'through': '2019-09-30'})
        self.assertNotIn('attested_on', death)
        self.assertEqual(notice['attested_on'], '2019-11-19')
        self.assertEqual(win['period'], {'from': '2019-11-01', 'through': '2019-11-30'})
        self.assertNotIn('attested_on', win)
        self.assertEqual(oath['attested_on'], '2020-05-11')
        self.assertLess(death['period']['through'], notice['attested_on'])
        self.assertLess(win['period']['through'], oath['attested_on'])
        # The scheduled poll day is never stored as the date of the win.
        self.assertNotIn('2019-11-28', self.raw)
        self.assertIn('not substituted', win['uncertainty'])
        self.assertIn('oath day is not given', oath['uncertainty'])
        # Court-dated claims: the ruling date is publication; the 2021 poll keeps its own date.
        self.assertEqual(self.sources['to_sc_helu_piukala_v_ec_20220829']['published_date'], '2022-08-29')
        self.assertEqual(self.claims['to_ptoa_2021_candidates_20211118']['attested_on'], '2021-11-18')
        self.assertIn("this packet's convention", self.claims['to_ptoa_2021_candidates_20211118']['uncertainty'])
        for cid in ('to_ptoa_president_helu_20220829', 'to_ptoa_self_described_objective_20220829',
                    'to_ptoa_tongan_name_20220829', 'to_ptoa_applicants_individuals_20220829'):
            self.assertEqual(self.claims[cid]['attested_on'], '2022-08-29')
        # The affidavit is not dated to the filing day.
        self.assertIn("affidavit's own date is not given", self.claims['to_ptoa_self_described_objective_20220829']['uncertainty'])
        self.assertNotIn('"2022-08-17"', self.raw)
        # The 2017 campaign has a month-precision start and no borrowed end.
        campaign = self.claims['to_ptoa_pohiva_campaign_2017']
        self.assertEqual(campaign['period'], {'from': '2017-08-01', 'through': None})
        self.assertNotIn('attested_on', campaign)
        self.assertIn('contextual', campaign['uncertainty'])
        self.assertIn('no end to the campaign', campaign['uncertainty'])
        self.assertIn("'Mr Pohiva and the PTOA party'", campaign['text'])
        self.assertNotIn('campaigning against', campaign['text'])
        # The 2025 poll, the petition judgment and its publication stay separate.
        for cid in ('to_tec_2025_tongatapu1', 'to_tec_2025_tongatapu2', 'to_fasi_v_sika_poll_result_20251120',
                    'to_ipu_2025_no_party_result'):
            self.assertEqual(self.claims[cid]['attested_on'], '2025-11-20')
        self.assertEqual(self.sources['to_sc_fasi_v_sika_20260518']['published_date'], '2026-05-18')
        self.assertNotIn('published_date', self.sources['to_tec_results_2025'])
        self.assertEqual(self.claims['to_incsoc_paper_register_20260721']['attested_on'], '2026-07-21')

    def test_no_end_date_or_term_is_inferred(self):
        for role_id in ('to_dpfi_leader', 'to_dpfi_president'):
            for entry in self.roles[role_id]['holder_claims']:
                if isinstance(entry, dict):
                    self.assertEqual((entry['from'], entry['until']), (None, None))
        self.assertEqual(self.entries['to_dpfi']['lifecycle']['from'], None)
        self.assertEqual(self.entries['to_dpfi']['lifecycle']['until'], None)
        # Pohiva's death ends no office here: no to_pm holder for him, and no holder cites the death or seat events.
        pm_holders = [h['name'] for h in self.roles['to_pm']['holder_claims'] if isinstance(h, dict)]
        # CLAUDE-C01-03 adds Eke (attested at his 2025 appointment); this packet adds no prime-minister holder.
        self.assertEqual(pm_holders, ["Pohiva Tu'i'onetoa", "Siaosi 'Ofakivahafolau Sovaleni", "'Aisake Valu Eke"])
        self.assertFalse(any('Akilisi' in name for name in pm_holders))
        for _, _, ids in self.holder_ids():
            self.assertFalse(set(ids) & set(SEAT_EVENTS))
            self.assertFalse(set(ids) & set(PM_2014))
        self.assertNotIn('to_pohiva_death_month_2019', self.entries['to_prime_minister']['claim_ids'])
        # CLAUDE-C01-03 appends its own notes after this one, so pin that exactly one entry carries it.
        self.assertEqual(sum('not used as the end of any premiership' in u
                             for u in self.entries['to_prime_minister']['coverage']['unresolved']), 1)
        self.assertIn('not used as the end of any office term', self.claims['to_pohiva_death_month_2019']['uncertainty'])
        # Every holder observation keeps from/until null unless a source states them: the 2021 PMO effective dates and,
        # from CLAUDE-C01-07, the Crown boundaries stated by the death notices and the devolution proclamations.
        stated = {("Siaosi 'Ofakivahafolau Sovaleni", '2021-12-27'), ('Poasi Mataele Tei', '2021-12-28'),
                  ('George Tupou V', '2006-09-11'), ('Tupou VI', '2012-03-18')}
        stated_ends = {("Taufa'ahau Tupou IV", '2006-09-11'), ('George Tupou V', '2012-03-18')}
        for _, entry, _ in self.holder_ids():
            if isinstance(entry, dict):
                if entry['until'] is not None:
                    self.assertIn((entry['name'], entry['until']), stated_ends)
                if entry['from'] is not None:
                    self.assertIn((entry['name'], entry['from']), stated)
        ends = [(e['name'], e['until']) for _, e, _ in self.holder_ids() if isinstance(e, dict) and e['until']]
        self.assertEqual(sorted(ends), sorted(stated_ends))
        starts = [(e['name'], e['from']) for _, e, _ in self.holder_ids() if isinstance(e, dict) and e['from']]
        self.assertEqual(sorted(starts), sorted(stated))

    def test_holders_are_exactly_as_intended(self):
        leader = self.roles['to_dpfi_leader']
        self.assertEqual(leader['holder_claims'][0], 'to_dpfi_pohiva_2010')
        second = leader['holder_claims'][1]
        self.assertEqual(len(leader['holder_claims']), 2)
        self.assertEqual({k: second[k] for k in ('name', 'attested_on', 'from', 'until', 'sources', 'claim_ids')}, {
            'name': "'Akilisi Pohiva", 'attested_on': '2014-11-27', 'from': None, 'until': None,
            'sources': ['to_ipu_2014'], 'claim_ids': ['to_ipu_2014_dpfi_pohiva_leader']})
        self.assertIn('IPU-only', second['uncertainty'])
        self.assertIn('IPU-only', self.claims['to_ipu_2014_dpfi_pohiva_leader']['uncertainty'])
        president = self.roles['to_dpfi_president']
        self.assertEqual((president['kind'], president['title']), ('other', 'President of the party (PTOA)'))
        holder, = president['holder_claims']
        self.assertEqual({k: holder[k] for k in ('name', 'attested_on', 'from', 'until', 'sources', 'claim_ids')}, {
            'name': 'Fatai Helu', 'attested_on': '2022-08-29', 'from': None, 'until': None,
            'sources': ['to_sc_helu_piukala_v_ec_20220829'], 'claim_ids': ['to_ptoa_president_helu_20220829']})
        recital = self.claims['to_ptoa_president_helu_20220829']['uncertainty']
        self.assertIn('not a court finding', recital)
        self.assertIn('self-description', recital)
        self.assertIn('not a court finding', holder['uncertainty'])
        # Only these two new claims feed holders; the party leader role gains no President, and vice versa.
        new_holder_claims = {cid for _, _, ids in self.holder_ids() for cid in ids} & NEW_CLAIMS
        self.assertEqual(new_holder_claims, HOLDER_CLAIMS)
        self.assertEqual(len([r for r in self.entries['to_dpfi']['roles'] if r['kind'] == 'party_leader']), 1)
        # Seat events, the 2025 poll and the executive member become nobody's holder observation.
        for role_id in ('to_peoples_representatives', 'to_ministers'):
            self.assertEqual(self.roles[role_id]['holder_claims'], [])
        self.assertEqual(self.roles['to_speaker']['holder_claims'], ['to_speakers_appointment'])
        for _, entry, _ in self.holder_ids():
            if isinstance(entry, dict):
                for name in ('Sika', 'Siaosi Pohiva', 'Siaosi Vailahi', 'Piukala', 'Puloka', 'Fasi'):
                    self.assertNotIn(name, entry['name'])

    def test_court_names_are_separate_observations_and_the_grouping_stays_provisional(self):
        dpfi = self.entries['to_dpfi']
        names = [(n['name'], n['attested_on']) for n in dpfi['name_observations']]
        self.assertEqual(names[:2], [('Democratic Party of the Friendly Islands', '2010-11-25'),
                                     ('Democratic Party of the Friendly Islands (DPFI)', '2014-11-27')])
        for expected in [('PTOA', '2022-08-29'), ("Paati Temokalati 'a e 'Otu Motu 'Anga'ofa", '2022-08-29'),
                         ('Democratic Party', '2022-08-29'), ('PTOA', '2023-04-06'), ('Democratic Party', '2023-04-06')]:
            self.assertIn(expected, names)
        # A disjunctive phrase is two names, never one.
        self.assertFalse(any(' or ' in name for name, _ in names))
        dated = [n['attested_on'] for n in dpfi['name_observations']]
        self.assertEqual(dated, sorted(dated))
        link = dpfi['identity_reconciliation']
        self.assertEqual((link['status'], link['automatic_merge'], link['reconciled_organization_id']),
                         ('provisional_observation_grouping', False, None))
        self.assertEqual(link['observations'], [f'TO-REC-0{n}' for n in range(1, 6)])
        self.assertIn('CLAUDE-C01-04', link['note'])
        self.assertIn('only through IPU 2021', link['note'])
        # No Tongan primary record uses the English name; the court claims say so.
        self.assertIn("does not use the English form 'Democratic Party of the Friendly Islands'",
                      self.claims['to_ptoa_tongan_name_20220829']['uncertainty'])
        self.assertIn("printed 'respondents' conflicts", self.claims['to_ptoa_democratic_party_20230406']['uncertainty'])
        self.assertIn('16 November 2021', self.claims['to_ptoa_democratic_party_20230406']['uncertainty'])
        self.assertIn('lacks legal personality', self.claims['to_ptoa_applicants_individuals_20220829']['uncertainty'])

    def test_2025_record_imports_no_party_result_and_no_seat_status(self):
        self.assertIn("'Not applicable. There is no party system or candidates stood as independents.'",
                      self.claims['to_ipu_2025_no_party_result']['text'])
        self.assertNotIn('no party system and candidates', self.raw)
        tongatapu2 = self.claims['to_tec_2025_tongatapu2']
        self.assertIn('CV 49/2025', tongatapu2['uncertainty'])
        self.assertIn('no seat holding at the cutoff is asserted', tongatapu2['uncertainty'])
        poll = self.claims['to_fasi_v_sika_poll_result_20251120']
        self.assertIn('the respondent was the successful candidate', poll['text'])
        self.assertIn('determination on the validity of that election is not imported', poll['uncertainty'])
        scope = self.sources['to_sc_fasi_v_sika_20260518']['scope_note']
        self.assertIn('are not imported', scope)
        self.assertIn('no reference to PTOA, DPFI or any party', scope)
        # The petition outcome, the 2017 accusation and the Lord Nuku debt stay out of every claim.
        for cid in NEW_CLAIMS:
            text = (self.claims[cid]['text'] + ' ' + self.claims[cid].get('uncertainty', '')).lower()
            for word in ('void', 'unseat', 'offence', 'passport', 'brib', 'debt'):
                self.assertNotIn(word, text, cid)
            self.assertNotIn('no seats', self.claims[cid]['text'].lower(), cid)
        for cid in ('to_ipu_2025_no_party_result', 'to_tec_2025_no_party_column', 'to_tec_2025_candidate_list_no_party'):
            self.assertIn(cid, self.entries['to_dpfi']['claim_ids'])
        for cid in ('to_tec_2025_tongatapu1', 'to_tec_2025_tongatapu2', 'to_fasi_v_sika_poll_result_20251120'):
            self.assertNotIn(cid, self.entries['to_dpfi']['claim_ids'])
            self.assertIn(cid, self.entries['to_legislative_assembly']['claim_ids'])
        unresolved = self.entries['to_dpfi']['coverage']['unresolved']
        eight, = [u for u in unresolved if u.startswith('TO-DPFI-08')]
        self.assertIn('not accepted', eight)
        self.assertIn('No dissolution or inactivity is inferred', eight)
        # Absence statements are bounded by the records searched.
        four, = [u for u in unresolved if u.startswith('TO-DPFI-04')]
        self.assertIn('records searched', four)
        seven, = [u for u in unresolved if u.startswith('TO-DPFI-07')]
        self.assertIn('records searched', seven)

    def test_secondary_and_unimported_leads_stay_out_of_the_packet(self):
        for source in self.packet['sources']:
            self.assertFalse(any(lead in source['url'] for lead in LEAD_HOSTS), source['id'])
        lowered = self.raw.lower()
        for marker in ('wikipedia', 'kaniva', 'tonga independent', 'matangi', 'core team', "people's team",
                       'anointed', '12 september 2019', 'cv 74', 'cv 54', 'help.aspx', 'nonprofit sector'):
            self.assertNotIn(marker, lowered)
        leads = self.section('Leads not imported')
        added = self.section('Sources added')
        for host in ('wikipedia.org', 'tongaindependent.com', 'kanivatonga.co.nz', 'matangitonga.to', 'abc.net.au',
                     'aceproject.org', 'category/206-cv-2022.html?download=2380', 'cv54',
                     'help.aspx?cn=NonprofitSector'):
            self.assertIn(host, leads, host)
            self.assertNotIn(host, added, host)

    def test_extracts_match_packet_claims_and_record_original_responses(self):
        for sid in NEW_SOURCES:
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual(extract['format'], 'spheres-c01-derived-factual-table/v1')
            self.assertEqual(extract['source_url'], source['url'])
            self.assertEqual(extract['claims'], source['claims'])
            self.assertEqual(extract['scope_note'], source['scope_note'])
            self.assertEqual(extract['access_method'], source['access_method'])
            self.assertEqual(extract['published_date'], source.get('published_date'))
            self.assertEqual((extract['accessed_date'], source['accessed_date']), ('2026-09-21', '2026-09-21'))
            self.assertFalse(extract['source_response_checked_in'])
            self.assertEqual((extract['source_response_bytes'], extract['source_response_sha256']), RESPONSES[sid])
            snapshot = source['snapshot']
            self.assertEqual(snapshot['kind'], 'derived_factual_extract')
            self.assertEqual(snapshot['path'], f"docs/campaign-certification/C01/research/sources/{snapshot['path'].rsplit('/', 1)[1]}")
            data = (research.ROOT / snapshot['path']).read_bytes()
            self.assertEqual((len(data), hashlib.sha256(data).hexdigest()), (snapshot['bytes'], snapshot['sha256']))
            self.assertNotEqual(snapshot['sha256'], extract['source_response_sha256'])
            self.assertTrue(data.endswith(b'}\n') and b'\r' not in data)
            self.assertIn('No open license, portrait permission or likeness approval', extract['rights_note'])
            self.assertIn('derived factual extract', extract['provenance_note'])
            self.assertIn('pdf_pages_one_based', extract['visual_review'])
            if source.get('published_date'):
                self.assertLessEqual(source['published_date'], research.CUTOFF)
            if extract['source_response_sha256'] is None:
                self.assertRegex(extract['provenance_note'], r'[Nn]o (raw )?response hash is asserted')
                self.assertRegex(extract['provenance_note'], r'form token|Cloudflare|per-request markup')
        ipu = self.extracts['to_ipu_2014']['normalized_response']
        self.assertEqual((ipu['bytes'], ipu['sha256']), IPU_2014_NORMALISED)
        self.assertIn("'<script>(function(){function c(){'", ipu['rule'])
        self.assertIn('Cloudflare', self.sources['to_ipu_2014']['scope_note'])
        for sid in ('to_assembly_byelection_20191119', 'to_assembly_siaosi_oath_20200511'):
            self.assertIn('Both parliament.gov.to and www.parliament.gov.to serve the article', self.sources[sid]['scope_note'])
            self.assertIn('www.parliament.gov.to', self.extracts[sid]['provenance_note'])
        self.assertEqual(self.extracts['to_tec_results_2025']['visual_review']['pdf_pages_one_based'], [1, 2])
        self.assertEqual(self.extracts['to_sc_helu_piukala_v_ec_20220829']['visual_review']['pdf_pages_one_based'], [1, 3, 10])

    def test_mutations_are_rejected(self):
        def mutated(change):
            packet = copy.deepcopy(self.packet)
            change(packet)
            return packet

        def source(packet, sid):
            return next(s for s in packet['sources'] if s['id'] == sid)

        def claim(packet, cid):
            return next(c for s in packet['sources'] for c in s['claims'] if c['id'] == cid)

        def role(packet, role_id):
            return next(r for e in packet['organizations'] + packet['institutions'] for r in e['roles'] if r['id'] == role_id)

        cases = [
            (lambda p: source(p, 'to_sc_helu_piukala_v_ec_20220829')['snapshot'].update(sha256='0' * 64), 'checksum mismatch'),
            (lambda p: source(p, 'to_ipu_2014')['snapshot'].update(bytes=1), 'checksum mismatch'),
            (lambda p: source(p, 'to_sc_fasi_v_sika_20260518')['snapshot'].update(path=REPORT.as_posix()), 'escapes'),
            (lambda p: role(p, 'to_dpfi_president')['holder_claims'][0].update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: role(p, 'to_dpfi_leader')['holder_claims'][1].update(until='2026-09-08'), 'exceeds cutoff'),
            (lambda p: claim(p, 'to_ptoa_pohiva_campaign_2017')['period'].update(through='2026-09-08'), 'exceeds cutoff'),
            (lambda p: claim(p, 'to_fasi_v_sika_poll_result_20251120').update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: claim(p, 'to_ptoa_pohiva_campaign_2017')['period'].update({'from': '2017-08'}), '(?i)invalid'),
            (lambda p: claim(p, 'to_pohiva_death_month_2019')['period'].update(through='2019-08-31'), 'Reversed historical interval'),
            (lambda p: role(p, 'to_dpfi_president')['holder_claims'][0].update(claim_ids=['to_ptoa_democratic_party_20230406']), 'cited source'),
        ]
        for change, message in cases:
            with self.subTest(message=message), self.assertRaisesRegex(ValueError, message):
                self.validate(mutated(change))

    def test_report_and_handoff_are_ready_for_review_stacked_and_close_nothing(self):
        self.assertIn('ready_for_review', self.report)
        table = self.section('Outcome')
        decisions = {'01': 'Accepted', '02': 'Accepted', '03': 'Unresolved', '04': 'Unresolved', '05': 'Accepted',
                     '06': 'Accepted', '07': 'Unresolved', '08': 'Not accepted'}
        for number, decision in decisions.items():
            row, = [line for line in table.splitlines() if line.startswith(f'| TO-DPFI-{number} ')]
            self.assertIn(f'**{decision}', row)
        defects = self.section('Checker defects')
        rows = [line for line in defects.splitlines() if re.match(r'\| D\d+ ', line)]
        self.assertEqual([re.match(r'\| (D\d+) ', r).group(1) for r in rows], [f'D{n}' for n in range(1, 16)])
        for row in rows:
            self.assertRegex(row, r'\*\*(Applied|Applied by exclusion|Applied as disclosure)\*\*')
        for marker in ('C01', 'C06', 'S23', 'WC1', 'CP1'):
            self.assertNotRegex(self.report, rf'\b{marker}\b[^.\n]*\bis (now )?complete\b')
        self.assertIn('CLAUDE-C01-02', self.section('Integration notes'))
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        for text in ('ready_for_review', 'tonga-dpfi-04.md', 'claude/c01-tonga-04', 'CLAUDE-C01-02', 'b6767837',
                     '21 September 2026 instruction', 'pending Codex acceptance'):
            self.assertIn(text, handoff)
        index = research.build()
        country = next(p for p in index['countries'] if p['nation'] == 'Tonga')
        self.assertFalse(country['country_census_complete'])
        self.assertIsNone(country['unrepresented_organization_count'])
        batch, = [row for row in index['work_orders'] if row['nation'] == 'Tonga']
        self.assertEqual(batch['status'], 'open')
        self.assertFalse(index['c01_complete'])


if __name__ == '__main__':
    unittest.main()
