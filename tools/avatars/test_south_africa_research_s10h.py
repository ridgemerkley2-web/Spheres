"""Keep a closed IEC ballot universe separate from a complete historical census."""
import copy
import json
import unittest
from collections import Counter
from datetime import date
from urllib.parse import urlsplit

import campaign_research as research
from test_south_africa_heads_of_state_c01_09 import RESPONSES as C01_09_RESPONSES, UNDATED as C01_09_UNDATED
from test_south_africa_anc_presidents_c01_16 import (HOLDERS as C01_16_HOLDERS, RESPONSES as C01_16_RESPONSES,
                                                     UNDATED as C01_16_UNDATED)

# The five sources of the original S10h intake keep every assertion below by id. CLAUDE-C01-09 adds 50 sources whose
# response identities are pinned exactly (bytes and SHA-256) in test_south_africa_heads_of_state_c01_09.py.
# CLAUDE-C01-16 adds 54 ANC sources, pinned the same way in test_south_africa_anc_presidents_c01_16.py.
ORIGINAL_SOURCES = ('za_iec_national_results_20240621', 'za_iec_national_seats_20240606', 'za_da_kzn_leader_20230403',
                    'za_da_leadership_20260412', 'za_parliament_president_elect_20240614')
# CLAUDE-C01-09: every presidency holder observation, exactly, as (role, name, attested_on, from, until).
PRESIDENCY_HOLDERS = [
    ('za_president_election', 'Nelson Mandela', '1994-05-10', None, '1999-06-16'),
    ('za_president_election', 'Thabo Mbeki', '1999-06-16', None, None),
    ('za_president_election', 'Thabo Mbeki', '2004-04-27', None, '2008-09-25'),
    ('za_president_election', 'Kgalema Motlanthe', '2008-09-25', None, None),
    ('za_president_election', 'Jacob Zuma', '2009-05-09', None, None),
    ('za_president_election', 'Jacob Zuma', None, '2014-05-24', '2018-02-14'),
    ('za_president_election', 'Cyril Ramaphosa', '2018-02-15', None, None),
    ('za_president_election', 'Cyril Ramaphosa', '2019-05-25', None, None),
    ('za_president_election', 'Cyril Ramaphosa', '2024-06-14', None, None),
    ('za_president_election', 'Cyril Ramaphosa', '2024-06-19', None, None),
    ('za_state_president', 'F. W. de Klerk', '1990-02-02', None, None),
]


class SouthAfricaDiscoveryTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.packet = json.loads((research.ROOT / research.RESEARCH / 'south-africa.json').read_text(encoding='utf-8'))
        cls.extracts = {
            s['id']: json.loads((research.ROOT / s['snapshot']['path']).read_text(encoding='utf-8'))
            for s in cls.packet['sources']
        }

    def validate(self, packet=None):
        return research.validate(packet or self.packet, research.ROOT, {'SouthAfrica'}, {'SouthAfrica': set()})

    def test_valid_packet_closes_only_two_bounded_report_scopes(self):
        ids = self.validate()
        # CLAUDE-C01-09 adds 50 sources, 78 claims and one role (za_state_president) and no entry; CLAUDE-C01-16 adds 54
        # sources, 73 claims and one role (za_anc_president) and no entry.
        self.assertEqual(tuple(len(ids[k]) for k in ('entries', 'sources', 'claims', 'roles')), (53, 109, 260, 6))
        coverage = self.packet['coverage']
        self.assertFalse(coverage['exhaustive_organization_register_reviewed'])
        self.assertIsNone(coverage['unrepresented_organization_total'])
        self.assertFalse(coverage['art_completion_claim'])
        self.assertEqual([r['records'] for r in coverage['bounded_registers']], [52, 52])
        self.assertEqual(coverage['bounded_registers'][1]['source_rows'], 58)
        self.assertTrue(all(r['complete_within_stated_source_scope'] for r in coverage['bounded_registers']))
        self.assertIn('not the full', coverage['unresolved'][0])

    def test_full_ballot_rows_reconcile_votes_source_spelling_and_page_boundaries(self):
        rows = self.extracts['za_iec_national_results_20240621']['rows']
        self.assertEqual(Counter(r['pdf_page'] for r in rows), {1: 37, 2: 15})
        for page, size in ((1, 37), (2, 15)):
            self.assertEqual([r['row_on_page'] for r in rows if r['pdf_page'] == page], list(range(1, size + 1)))
        self.assertEqual(len({r['name'] for r in rows}), 52)
        self.assertEqual(sum(r['national_ballot_votes'] for r in rows), 16077342)
        self.assertEqual([rows[0]['name'], rows[36]['name'], rows[37]['name'], rows[-1]['name']],
                         ['#HOPE4SA', 'NORTHERN CAPE COMMUNITIES MOVEMENT', 'ORGANIC HUMANITY MOVEMENT', 'XILUVA'])
        names = {r['name']: r for r in rows}
        self.assertIn('AFRICA RESTORATION ALLIANCE', names)
        self.assertIn('CITIZANS', names)
        self.assertIn('CONGRESS  OF THE PEOPLE', names)
        self.assertNotIn('INDEPENDENT SOUTH AFRICAN NATIONAL CIVIC ORGANISATION', names)
        self.assertEqual(names['AFRICAN MOVEMENT CONGRESS']['source_abbreviation'], 'NO_ABBR')
        self.assertEqual(names['FREE DEMOCRATS']['source_abbreviation'], 'FREE DEMS')
        self.assertEqual(names['ACTIONSA']['source_abbreviation'], 'ACTIONSA')

    def test_separate_seat_columns_reconcile_without_dropping_zero_seat_parties(self):
        data = self.extracts['za_iec_national_seats_20240606']
        rows = data['rows']
        self.assertEqual(len(rows), 52)
        self.assertEqual(Counter(r['overall_seats_assigned'] > 0 for r in rows), {True: 18, False: 34})
        self.assertEqual(sum(r['regional_seats_assigned'] for r in rows), 200)
        self.assertEqual(sum(r['national_pr_seats_assigned'] for r in rows), 200)
        self.assertEqual(sum(r['overall_seats_assigned'] for r in rows), 400)
        self.assertEqual(sum(r['overall_seats_forfeited'] for r in rows), 0)
        self.assertTrue(all(r['regional_seats_assigned'] + r['national_pr_seats_assigned'] == r['overall_seats_assigned'] for r in rows))
        assigned = {r['name']: r['overall_seats_assigned'] for r in rows if r['overall_seats_assigned']}
        self.assertEqual(assigned, {
            'ACTIONSA': 6, 'AFRICAN CHRISTIAN DEMOCRATIC PARTY': 3, 'AFRICAN NATIONAL CONGRESS': 159,
            'AFRICAN TRANSFORMATION MOVEMENT': 2, 'AL JAMA-AH': 2, 'BUILD ONE SOUTH AFRICA WITH MMUSI MAIMANE': 2,
            'DEMOCRATIC ALLIANCE': 87, 'ECONOMIC FREEDOM FIGHTERS': 39, 'GOOD': 1, 'INKATHA FREEDOM PARTY': 17,
            'NATIONAL COLOURED CONGRESS': 2, 'PAN AFRICANIST CONGRESS OF AZANIA': 1, 'PATRIOTIC ALLIANCE': 9,
            'RISE MZANSI': 2, 'UMKHONTO WESIZWE': 58, 'UNITED AFRICANS TRANSFORMATION': 1,
            'UNITED DEMOCRATIC MOVEMENT': 3, 'VRYHEIDSFRONT PLUS': 6})

    def test_independent_candidate_rows_are_accounted_for_but_never_invented_as_parties(self):
        data = self.extracts['za_iec_national_seats_20240606']
        rows = data['excluded_non_organization_rows']
        self.assertEqual({r['name'] for r in rows}, {
            'ACHMAT ZACKIE - 583999', 'LIEBENBERG LOUIS PETRUS - 588209', 'MDA ANELE - 587892',
            'NDOU LOVEMORE RAY - 582100', 'PHATHELA NTAKADZENI FAITH - 587582',
            'RAMOBA LEHLOHONOLO BLESSINGS ANSWER - 501784'})
        self.assertTrue(all(r['national_pr_seats_assigned'] is None and r['overall_seats_assigned'] == 0 for r in rows))
        self.assertTrue(all('observation_id' not in r and 'claim_id' not in r for r in rows))
        self.assertFalse({r['name'] for r in rows} & {o['name'] for o in self.packet['organizations']})
        all_rows = data['rows'] + rows
        self.assertEqual(Counter(r['pdf_page'] for r in all_rows), {1: 32, 2: 26})
        for page, size in ((1, 32), (2, 26)):
            self.assertEqual(sorted(r['row_on_page'] for r in all_rows if r['pdf_page'] == page), list(range(1, size + 1)))

    def test_party_leader_and_two_chair_titles_remain_distinct(self):
        da = next(o for o in self.packet['organizations'] if o['name'] == 'DEMOCRATIC ALLIANCE')
        self.assertEqual([r['title'] for r in da['roles']], ['Federal Leader', 'Federal Chairperson', 'Chairperson of the Federal Council'])
        self.assertEqual([r['kind'] for r in da['roles']], ['party_leader', 'party_chair', 'party_chair'])
        self.assertEqual([(h['name'], h['attested_on']) for h in da['roles'][0]['holder_claims']],
                         [('John Steenhuisen', '2023-04-03'), ('Geordin Hill-Lewis', '2026-04-12')])
        self.assertEqual([r['holder_claims'][0]['name'] for r in da['roles'][1:]], ['Solly Msimanga', 'Ashor Sarupen'])
        first = self.extracts['za_da_kzn_leader_20230403']['rows'][0]
        self.assertIsNone(first['election_date'])
        # CLAUDE-C01-16 gives the ANC exactly one party role; every other organization still has none.
        self.assertEqual({o['name']: [r['id'] for r in o['roles']] for o in self.packet['organizations'] if o['roles']},
                         {'DEMOCRATIC ALLIANCE': ['za_da_federal_leader', 'za_da_federal_chair', 'za_da_council_chair'],
                          'AFRICAN NATIONAL CONGRESS': ['za_anc_president']})

    def test_election_as_president_elect_does_not_grant_inauguration_or_party_presidency(self):
        self.assertEqual(len(self.packet['institutions']), 1)
        presidency = self.packet['institutions'][0]
        self.assertEqual(presidency['id'], 'za_presidency')
        # CLAUDE-C01-09 adds the State President (1983 Constitution) as a second head-of-state role, never a holder of
        # the President's role.
        self.assertEqual([r['kind'] for r in presidency['roles']], ['head_of_state', 'head_of_state'])
        self.assertEqual([r['id'] for r in presidency['roles']], ['za_president_election', 'za_state_president'])
        row = self.extracts['za_parliament_president_elect_20240614']['rows'][0]
        self.assertEqual((row['holder_name'], row['event_kind'], row['attested_on']),
                         ('Cyril Ramaphosa', 'election_as_president_elect', '2024-06-14'))
        self.assertEqual(row['votes'], 283)
        self.assertEqual(row['other_candidate'], {'name': 'Julius Malema', 'votes': 44})
        self.assertIsNone(row['assumption_of_office_date'])
        anc = next(o for o in self.packet['organizations'] if o['name'] == 'AFRICAN NATIONAL CONGRESS')
        # CLAUDE-C01-16 adds the ANC's own party office, and only that role; the election as President-elect still
        # grants no party presidency: no presidency claim or source feeds the ANC role and no ANC claim or source feeds
        # the presidency.
        self.assertEqual([(r['id'], r['kind'], r['title']) for r in anc['roles']],
                         [('za_anc_president', 'party_leader', 'President of the African National Congress')])
        anc_role = anc['roles'][0]
        self.assertEqual([(h['name'], h['attested_on'], h['from'], h['until']) for h in anc_role['holder_claims']],
                         C01_16_HOLDERS)
        presidency_claims = set(presidency['claim_ids']) | {c for r in presidency['roles'] for c in r['claim_ids']}
        presidency_sources = set(presidency['sources']) | {s for r in presidency['roles'] for s in r['sources']}
        anc_claims = set(anc_role['claim_ids']) | {c for h in anc_role['holder_claims'] for c in h['claim_ids']}
        anc_sources = set(anc_role['sources']) | {s for h in anc_role['holder_claims'] for s in h['sources']}
        self.assertFalse(anc_claims & presidency_claims)
        self.assertFalse(anc_sources & presidency_sources)
        self.assertNotIn('za_ramaphosa_president_elect_20240614', anc_claims)
        self.assertNotIn('za_parliament_president_elect_20240614', anc_sources)

    def test_no_lifespans_continuous_holder_terms_or_game_identity_are_invented(self):
        for entry in self.packet['organizations'] + self.packet['institutions']:
            self.assertEqual(entry['represented_party_ids'], [])
            self.assertIsNone(entry['reconciled_organization_id'])
            self.assertEqual(entry['lifecycle']['status'], 'unknown')
            self.assertIsNone(entry['lifecycle']['from'])
            self.assertIsNone(entry['lifecycle']['until'])
            for role in entry['roles']:
                for holder in role['holder_claims']:
                    if entry['id'] == 'za_presidency':
                        continue
                    self.assertIsNone(holder['from'])
                    self.assertIsNone(holder['until'])
        # CLAUDE-C01-09: presidency holders carry a start or an end only where a primary source states one (Zuma's
        # 2014 assumption; the ends of Mandela, Mbeki and Zuma), pinned exactly; every other holder is a dated
        # observation with no interval.
        presidency = self.packet['institutions'][0]
        self.assertEqual([(r['id'], h['name'], h['attested_on'], h['from'], h['until'])
                          for r in presidency['roles'] for h in r['holder_claims']], PRESIDENCY_HOLDERS)
        p = copy.deepcopy(self.packet)
        p['organizations'][0]['represented_party_ids'] = ['SouthAfrica/guessed_hope4sa']
        with self.assertRaisesRegex(ValueError, 'foreign represented party mapping'):
            self.validate(p)

    def test_extracts_trace_every_claim_and_distinguish_response_from_derived_bytes(self):
        entries = {o['id']: o for o in self.packet['organizations'] + self.packet['institutions']}
        pins = {
            'za_iec_national_results_20240621': (48695, '2931930f27426b05362460e3293c29237d2975055d464ed7858a194dfdec12a0'),
            'za_iec_national_seats_20240606': (64176, '68690211abe6b4e6ee533352a22d059eaed905aece390fbf03eee53d82bd08ee')}
        for source in self.packet['sources']:
            data = self.extracts[source['id']]
            self.assertEqual(data['format'], 'spheres-c01-derived-factual-table/v1')
            self.assertEqual(data['source_url'], source['url'])
            self.assertEqual({r['claim_id'] for r in data['rows']}, {c['id'] for c in source['claims']})
            self.assertRegex(data['source_response_sha256'], r'^[0-9a-f]{64}$')
            if source['id'] in ORIGINAL_SOURCES:
                self.assertGreater(data['source_response_bytes'], 40000)
            else:
                # CLAUDE-C01-09 and CLAUDE-C01-16 archives include pages under 40,000 bytes; each response is pinned
                # exactly.
                self.assertEqual((data['source_response_bytes'], data['source_response_sha256']),
                                 {**C01_09_RESPONSES, **C01_16_RESPONSES}[source['id']])
            self.assertNotEqual(data['source_response_sha256'], source['snapshot']['sha256'])
            self.assertIn('not checked into this repository', data['provenance_note'])
            for row in data['rows']:
                entry = entries[row['observation_id']]
                self.assertIn(row['claim_id'], entry['claim_ids'])
                self.assertIn(source['id'], entry['sources'])
                if 'name' in row:
                    self.assertEqual(row['name'], entry['name'])
            if source['id'] in pins:
                self.assertEqual((data['source_response_bytes'], data['source_response_sha256']), pins[source['id']])
                self.assertEqual(data['reviewed_pdf_pages'], [1, 2])
        self.assertEqual([s['id'] for s in self.packet['sources']],
                         list(ORIGINAL_SOURCES) + list(C01_09_RESPONSES) + list(C01_16_RESPONSES))
        p = copy.deepcopy(self.packet)
        p['sources'][0]['snapshot']['sha256'] = '0' * 64
        with self.assertRaisesRegex(ValueError, 'checksum mismatch'):
            self.validate(p)

    def test_access_and_report_dates_do_not_extend_historical_coverage(self):
        hosts = {'results.elections.org.za', 'kzn.da.org.za', 'www-origin.da.org.za', 'www.parliament.gov.za'}
        # CLAUDE-C01-09 sources: raw Internet Archive captures, the gazettes.africa archive host and Parliament's
        # document store, all accessed on 2026-09-22.
        c01_09_hosts = {'web.archive.org', 'archive.gazettes.africa', 'www.parliament.gov.za'}
        # CLAUDE-C01-16 sources: raw Internet Archive captures of the ANC's own pages, all accessed on 2026-09-24.
        c01_16_hosts = {'web.archive.org'}
        for source in self.packet['sources']:
            if source['id'] in ORIGINAL_SOURCES:
                self.assertIn(urlsplit(source['url']).hostname, hosts)
                self.assertEqual(source['accessed_date'], '2026-09-14')
            elif source['id'] in C01_16_RESPONSES:
                self.assertIn(urlsplit(source['url']).hostname, c01_16_hosts)
                self.assertEqual(source['accessed_date'], '2026-09-24')
            else:
                self.assertIn(urlsplit(source['url']).hostname, c01_09_hosts)
                self.assertEqual(source['accessed_date'], '2026-09-22')
            for claim in source['claims']:
                if claim['id'] in C01_09_UNDATED or claim['id'] in C01_16_UNDATED:
                    # Retrospective list and directory spans, month-only and span-only claims and later web-edition
                    # headings carry no structured date at all.
                    self.assertNotIn('attested_on', claim)
                    continue
                self.assertLessEqual(date.fromisoformat(claim['attested_on']), date.fromisoformat(research.CUTOFF))
        self.assertEqual({urlsplit(s['url']).hostname for s in self.packet['sources']},
                         hosts | c01_09_hosts | c01_16_hosts)
        # Seven C01-09 claims and 31 C01-16 claims carry no structured date.
        self.assertEqual(sum('attested_on' not in c for s in self.packet['sources'] for c in s['claims']), 7 + 31)
        self.assertEqual([s['published_date'] for s in self.packet['sources'][:2]], [None, None])
        self.assertEqual(self.extracts['za_iec_national_results_20240621']['report_as_at'], '2024-06-21T13:51:28')
        self.assertEqual(self.extracts['za_iec_national_seats_20240606']['report_as_at'], '2024-06-06T11:56:55')
        p = copy.deepcopy(self.packet)
        p['sources'][0]['claims'][0]['attested_on'] = '2026-09-08'
        with self.assertRaisesRegex(ValueError, 'exceeds cutoff'):
            self.validate(p)
        p = copy.deepcopy(self.packet)
        p['organizations'][0]['roles'] = copy.deepcopy(self.packet['organizations'][26]['roles'])
        p['organizations'][0]['roles'][0]['sources'] = ['za_iec_national_results_20240621']
        with self.assertRaisesRegex(ValueError, 'Claim does not belong'):
            self.validate(p)

    def test_discovery_batches_stay_open_and_campaign_certification_is_not_granted(self):
        index = research.build()
        country = next(p for p in index['countries'] if p['nation'] == 'SouthAfrica')
        self.assertFalse(country['country_census_complete'])
        self.assertEqual(country['mapping_pending'], 53)
        self.assertFalse(index['runtime_roster_modified'])
        self.assertFalse(index['c01_complete'])
        self.assertFalse(index['g2_prerequisite_satisfied'])
        work = [w for w in index['work_orders'] if w['nation'] == 'SouthAfrica']
        self.assertEqual([len(w['members']) for w in work], [10] * 5 + [3])
        self.assertEqual({m for w in work for m in w['members']}, set(self.validate()['entries']))
        self.assertEqual({w['status'] for w in work}, {'open'})


if __name__ == '__main__':
    unittest.main()
