"""CLAUDE-C01-46: the State Duma's eighth-convocation faction heads, 2021-2026, keep the faction elections and reported
selections, acting service, death reports and dated in-office attestations apart, append holder observations after the
five original holders without changing them, and state an end only where a Duma record names the office and the day."""
import copy
import hashlib
import json
import re
import unittest

import campaign_research as research


# Original response identity recorded in each new extract: (bytes, sha256) of the Internet Archive raw capture as served
# (curl without --compressed), downloaded byte-identical twice by this packet at least 30 minutes apart.
RESPONSES = {
    'ru_duma_news_52394_20211011': (247852, 'bea477cefb12a59e2b6c13f534b6248c717b2797823ebac3369eca322e4b1b88'),
    'ru_duma_news_53098_20211222': (245393, '5beb30c907d7fa7c019fb2ce3d79ebb97ef78e2ce4c2ad5ca81280a094590264'),
    'ru_duma_news_53987_20220406': (225439, '3c4962730ce2199a54b1a25bad0328debbc482c897cbc12ac2aed06202f70197'),
    'ru_duma_news_53988_20220406': (250245, '3cfc0a0403a6ba331e74c898428e5b4b923d7f5c26179e77cedcbbb7c9e7e7fc'),
    'ru_duma_news_54314_20220518': (232778, '3851ebdab4786543605f9d72ad7725fac50d8c111e4e05596db550429236b7e8'),
    'ru_duma_news_54910_20220707': (313338, '450c2f367443bad74ea39e987fe4a77f92ec08ee90977bcd4f7ef8f16a5dc47c'),
    'ru_duma_news_63980_20260727': (53864, 'e9f8fddf860ee5b707d59991b97407fc3c6096ccb085da026d59ba06bdfc3a4f'),
}
# The one capture the Internet Archive serves gzip even without Accept-Encoding: its decoded identity.
DECODED = {'ru_duma_news_63980_20260727': (238068, 'e8a6b66fa43a180ad4962ed2defcdb7ccb705a8a487ea0999537beee7168babf')}
CAPTURES = {
    'ru_duma_news_52394_20211011': ('20211011155146', '52394'),
    'ru_duma_news_53098_20211222': ('20220119095504', '53098'),
    'ru_duma_news_53987_20220406': ('20220407135620', '53987'),
    'ru_duma_news_53988_20220406': ('20220407092508', '53988'),
    'ru_duma_news_54314_20220518': ('20220524092011', '54314'),
    'ru_duma_news_54910_20220707': ('20220710204413', '54910'),
    'ru_duma_news_63980_20260727': ('20260728062748', '63980'),
}
NEW_SOURCES = list(RESPONSES)

ER, KPRF, LDPR, NL, SRZP = (f'ru_duma_faction_20211012_{k}_head' for k in ('er', 'kprf', 'ldpr', 'nl', 'srzp'))
FACTION_ROLES = (ER, KPRF, SRZP, LDPR, NL)
TITLE = 'Руководитель фракции — head of the parliamentary faction'
VAS, ZYU, ZHI, MIR, NEC, SLU = ('Владимир Васильев', 'Геннадий Зюганов', 'Владимир Жириновский', 'Сергей Миронов',
                                'Алексей Нечаев', 'Леонид Слуцкий')

# Every claim of this packet: (attested_on, event kind, role, review observation).
EVENTS = {
    'ru_duma_news_52394_vasilyev_elected_er_head_20211007': ('2021-10-07', 'election', ER, 'RU-FH-01'),
    'ru_duma_news_52394_zyuganov_heads_kprf_reported_20211011': ('2021-10-11', 'selection_reported', KPRF, 'RU-FH-01'),
    'ru_duma_news_52394_zhirinovsky_heads_ldpr_reported_20211011': ('2021-10-11', 'selection_reported', LDPR, 'RU-FH-01'),
    'ru_duma_news_52394_mironov_elected_srzp_head_reported_20211011': ('2021-10-11', 'election', SRZP, 'RU-FH-01'),
    'ru_duma_news_52394_nechaev_heads_nl_reported_20211011': ('2021-10-11', 'selection_reported', NL, 'RU-FH-01'),
    'ru_duma_news_53098_zyuganov_kprf_head_20211222': ('2021-12-22', 'in_office_attestation', KPRF, 'RU-FH-02'),
    'ru_duma_news_53098_zhirinovsky_ldpr_head_20211222': ('2021-12-22', 'in_office_attestation', LDPR, 'RU-FH-02'),
    'ru_duma_news_53098_nechaev_nl_head_20211222': ('2021-12-22', 'in_office_attestation', NL, 'RU-FH-02'),
    'ru_duma_news_53098_vasilyev_er_head_20211222': ('2021-12-22', 'in_office_attestation', ER, 'RU-FH-02'),
    'ru_duma_news_53987_zhirinovsky_death_reported_20220406': ('2022-04-06', 'death_reported', LDPR, 'RU-FH-03'),
    'ru_duma_news_53988_zhirinovsky_died_as_ldpr_head_20220406': ('2022-04-06', 'death_in_office', LDPR, 'RU-FH-03'),
    'ru_duma_news_53988_vasilyev_er_head_20220406': ('2022-04-06', 'in_office_attestation', ER, 'RU-FH-03'),
    'ru_duma_news_53988_zyuganov_kprf_head_20220406': ('2022-04-06', 'in_office_attestation', KPRF, 'RU-FH-03'),
    'ru_duma_news_53988_mironov_srzp_head_20220406': ('2022-04-06', 'in_office_attestation', SRZP, 'RU-FH-03'),
    'ru_duma_news_53988_nechaev_nl_head_20220406': ('2022-04-06', 'in_office_attestation', NL, 'RU-FH-03'),
    'ru_duma_news_54314_slutsky_elected_ldpr_head_20220518': ('2022-05-18', 'election', LDPR, 'RU-FH-04'),
    'ru_duma_news_54314_slutsky_acting_ldpr_head_reported_20220518': ('2022-05-18', 'acting_service_reported', LDPR, 'RU-FH-04'),
    'ru_duma_news_54910_zyuganov_kprf_head_20220707': ('2022-07-07', 'in_office_attestation', KPRF, 'RU-FH-05'),
    'ru_duma_news_54910_slutsky_ldpr_head_20220707': ('2022-07-07', 'in_office_attestation', LDPR, 'RU-FH-05'),
    'ru_duma_news_54910_mironov_srzp_head_20220707': ('2022-07-07', 'in_office_attestation', SRZP, 'RU-FH-05'),
    'ru_duma_news_54910_nechaev_nl_head_20220707': ('2022-07-07', 'in_office_attestation', NL, 'RU-FH-05'),
    'ru_duma_news_54910_vasilyev_er_head_20220707': ('2022-07-07', 'in_office_attestation', ER, 'RU-FH-05'),
    'ru_duma_news_63980_vasilyev_er_head_20260727': ('2026-07-27', 'in_office_attestation', ER, 'RU-FH-06'),
    'ru_duma_news_63980_zyuganov_kprf_head_20260727': ('2026-07-27', 'in_office_attestation', KPRF, 'RU-FH-06'),
    'ru_duma_news_63980_mironov_srzp_head_20260727': ('2026-07-27', 'in_office_attestation', SRZP, 'RU-FH-06'),
    'ru_duma_news_63980_slutsky_ldpr_head_20260727': ('2026-07-27', 'in_office_attestation', LDPR, 'RU-FH-06'),
    'ru_duma_news_63980_nechaev_nl_head_20260727': ('2026-07-27', 'in_office_attestation', NL, 'RU-FH-06'),
}
HOLDER_KINDS = {'in_office_attestation', 'death_in_office'}
CLAIM_ONLY_KINDS = {'election', 'selection_reported', 'acting_service_reported', 'death_reported'}
NEVER_HOLDER = tuple(c for c, v in EVENTS.items() if v[1] in CLAIM_ONLY_KINDS)
# Election, selection-report and acting-report days; never a holder's attested_on, from or until.
NEVER_HOLDER_DATE = {'2021-10-07', '2021-10-11', '2022-05-18'}
DEATH = 'ru_duma_news_53988_zhirinovsky_died_as_ldpr_head_20220406'
DEATH_REPORT = 'ru_duma_news_53987_zhirinovsky_death_reported_20220406'


def att(role_key, day):
    return next(c for c, v in EVENTS.items() if v[2] == role_key and v[0] == day and v[1] == 'in_office_attestation')


# The original holder of each role (unchanged), then this packet's observations in date order.
ORIGINAL = {
    ER: (VAS, '2021-10-12', None, None, ['ru_duma_factions_20211012'], ['ru_duma_faction_20211012_er_leader']),
    KPRF: (ZYU, '2021-10-12', None, None, ['ru_duma_factions_20211012'], ['ru_duma_faction_20211012_kprf_leader']),
    SRZP: (MIR, '2021-10-12', None, None, ['ru_duma_factions_20211012'], ['ru_duma_faction_20211012_srzp_leader']),
    LDPR: (ZHI, '2021-10-12', None, None, ['ru_duma_factions_20211012'], ['ru_duma_faction_20211012_ldpr_leader']),
    NL: (NEC, '2021-10-12', None, None, ['ru_duma_factions_20211012'], ['ru_duma_faction_20211012_nl_leader']),
}
DAYS = ('2021-12-22', '2022-04-06', '2022-07-07', '2026-07-27')
HOLDERS = {
    ER: [(VAS, d, None, None, [att(ER, d)]) for d in DAYS],
    KPRF: [(ZYU, d, None, None, [att(KPRF, d)]) for d in DAYS],
    SRZP: [(MIR, d, None, None, [att(SRZP, d)]) for d in DAYS[1:]],
    LDPR: [(ZHI, '2021-12-22', None, '2022-04-06', [att(LDPR, '2021-12-22'), DEATH]),
           (SLU, '2022-07-07', None, None, [att(LDPR, '2022-07-07')]),
           (SLU, '2026-07-27', None, None, [att(LDPR, '2026-07-27')])],
    NL: [(NEC, d, None, None, [att(NL, d)]) for d in DAYS],
}
NAMES = {VAS, ZYU, ZHI, MIR, NEC, SLU}
REPORT = research.RESEARCH / 'russia-duma-faction-heads-2021-2026-46.md'
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-46.md'
VOLATILE_URL = re.compile(r'(ysclid=|sessid=|PHPSESSID|[?&]cb=|nocache|token=|utm_|fbclid|yclid|DDoS|/web/\d{4}id_/|'
                          r'/web/\d{14}/)')
# Holders on the other Russia roles, which this packet never touches: (role, number of holder observations).
OTHER_ROLE_HOLDERS = {'ru_rsfsr_president': 1, 'ru_rsfsr_vice_president': 1, 'ru_president': 7, 'ru_government_chairman': 15}


def invariants(russia):
    """Packet-level rules this test owns; raises AssertionError, KeyError, IndexError or StopIteration on any violation."""
    claims = {c['id']: c for s in russia['sources'] for c in s['claims']}
    owner = {c['id']: s['id'] for s in russia['sources'] for c in s['claims']}
    ours = set(EVENTS)
    factions = [e for e in russia['institutions'] if e['kind'] == 'parliamentary_faction']
    assert [r['id'] for e in factions for r in e['roles']] == [ER, KPRF, SRZP, LDPR, NL]
    for entry in factions:
        role_id = entry['roles'][0]['id']
        assert len(entry['roles']) == 1 and entry['id'] == role_id[:-len('_head')]
        role = entry['roles'][0]
        assert (role['title'], role['kind']) == (TITLE, 'parliamentary_leader')
        # The faction observation itself is unchanged: no lifecycle bound, no party link, entry-level citations as before.
        assert entry['sources'] == ['ru_duma_factions_20211012']
        assert entry['claim_ids'] == [ORIGINAL[role_id][5][0].replace('_leader', '_membership'), ORIGINAL[role_id][5][0]]
        assert entry['lifecycle'] == {'status': 'documented_at_specific_observation', 'from': None, 'until': None,
                                      'attested_on': '2021-10-12'}
        assert (entry['represented_party_ids'], entry['constituent_organization_ids'], entry['reconciled_organization_id']) == ([], [], None)
        holders = role['holder_claims']
        first = holders[0]
        assert (first['name'], first['attested_on'], first['from'], first['until'], first['sources'], first['claim_ids']) == ORIGINAL[role_id]
        got = [(h['name'], h['attested_on'], h['from'], h['until'], h['claim_ids']) for h in holders[1:]]
        assert got == HOLDERS[role_id], got
        expected_claims = list(ORIGINAL[role_id][5]) + [c for c in EVENTS if EVENTS[c][2] == role_id]
        assert role['claim_ids'] == expected_claims
        expected_sources = ['ru_duma_factions_20211012']
        for cid in expected_claims[1:]:
            if owner[cid] not in expected_sources:
                expected_sources.append(owner[cid])
        assert role['sources'] == expected_sources
        for h in holders[1:]:
            assert h['name'] in NAMES and len(h['name'].split()) == 2, h['name']
            assert h['from'] is None, h['name']
            assert all(EVENTS[c][1] in HOLDER_KINDS and EVENTS[c][2] == role_id for c in h['claim_ids']), h['name']
            assert not set(h['claim_ids']) & set(NEVER_HOLDER), h['name']
            assert not {h['attested_on'], h['until']} & NEVER_HOLDER_DATE, h['name']
            assert EVENTS[h['claim_ids'][0]][1] == 'in_office_attestation'
            assert claims[h['claim_ids'][0]]['attested_on'] == h['attested_on'], h['name']
            expected = []
            for cid in h['claim_ids']:
                if owner[cid] not in expected:
                    expected.append(owner[cid])
            assert h['sources'] == expected, h['name']
            # An end only where the last claim is the Duma's report of a death in office, on its day.
            if h['until'] is not None:
                assert h['claim_ids'][-1] == DEATH and h['until'] == claims[DEATH]['attested_on'] > h['attested_on']
            else:
                assert DEATH not in h['claim_ids']
    # No other role, in any group, cites a claim of this packet or carries one of its holder observations.
    for group in ('organizations', 'institutions'):
        for entry in russia[group]:
            for r in entry['roles']:
                if r['id'] in FACTION_ROLES:
                    continue
                cited = set(r['claim_ids']) | {c for h in r['holder_claims'] if isinstance(h, dict) for c in h['claim_ids']}
                assert not cited & ours, r['id']
                assert not set(r['sources']) & set(NEW_SOURCES), r['id']
                if r['id'] in OTHER_ROLE_HOLDERS:
                    assert len(r['holder_claims']) == OTHER_ROLE_HOLDERS[r['id']], r['id']
            assert not set(entry['claim_ids']) & ours and not set(entry['sources']) & set(NEW_SOURCES), entry['id']
    # The death report without a day never ends a holder; the death in office states its day by 'Сегодня'.
    assert 'Сегодня' in claims[DEATH]['text'] and 'Руководитель фракции ЛДПР' in claims[DEATH]['text']
    assert 'Сегодня' not in claims[DEATH_REPORT]['text']
    assert all(DEATH_REPORT not in h['claim_ids'] for e in factions for h in e['roles'][0]['holder_claims'])
    # Слуцкий: elected 18 May 2022 and acting before it (claims only); his first holder observation is later.
    ldpr = next(e for e in factions if e['roles'][0]['id'] == LDPR)['roles'][0]
    assert [h['attested_on'] for h in ldpr['holder_claims'] if h['name'] == SLU] == ['2022-07-07', '2026-07-27']


class RussianDumaFactionHeadsTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.raw = (research.ROOT / research.RESEARCH / 'russia.json').read_text(encoding='utf-8')
        cls.packet = json.loads(cls.raw)
        cls.sources = {s['id']: s for s in cls.packet['sources']}
        cls.claims = {c['id']: c for s in cls.packet['sources'] for c in s['claims']}
        cls.extracts = {sid: json.loads((research.ROOT / cls.sources[sid]['snapshot']['path']).read_text(encoding='utf-8'))
                        for sid in NEW_SOURCES}
        cls.rows = {row['claim_id']: row for sid in NEW_SOURCES for row in cls.extracts[sid]['rows']}
        cls.report = (research.ROOT / REPORT).read_text(encoding='utf-8')

    def validate(self, packet=None):
        return research.validate(packet or self.packet, research.ROOT, {'Russia'}, {'Russia': set()})

    def check(self, packet):
        self.validate(packet)
        invariants(packet)

    def section(self, heading):
        parts = self.report.split(f'\n## {heading}', 1)
        self.assertEqual(len(parts), 2, heading)
        return parts[1].split('\n## ', 1)[0]

    def test_new_records_are_bounded_and_every_claim_is_classified(self):
        ids = self.validate()
        self.assertEqual((len(ids['sources']), len(ids['claims']), len(ids['entries']), len(ids['roles'])), (177, 322, 21, 9))
        self.assertEqual([s['id'] for s in self.packet['sources'][170:]], NEW_SOURCES)
        new_claims = [c['id'] for sid in NEW_SOURCES for c in self.sources[sid]['claims']]
        self.assertEqual(new_claims, list(EVENTS))
        dates = [self.sources[sid]['document_date'] for sid in NEW_SOURCES]
        self.assertEqual(dates, sorted(dates))
        self.assertEqual({v[1] for v in EVENTS.values()}, HOLDER_KINDS | CLAIM_ONLY_KINDS)
        self.assertFalse(HOLDER_KINDS & CLAIM_ONLY_KINDS)
        self.assertEqual({v[3] for v in EVENTS.values()}, {f'RU-FH-{n:02d}' for n in range(1, 7)})
        self.assertEqual(re.findall(r'^### (RU-FH-\d\d)\b', self.report, re.M), [f'RU-FH-{n:02d}' for n in range(1, 7)])
        cited = {c for rid in FACTION_ROLES for h in HOLDERS[rid] for c in h[4]}
        self.assertEqual(cited, {c for c, v in EVENTS.items() if v[1] in HOLDER_KINDS})
        self.assertEqual(sum(len(v) for v in HOLDERS.values()), 18)
        self.assertEqual({h[0] for v in HOLDERS.values() for h in v}, NAMES)
        for sid in NEW_SOURCES:
            source = self.sources[sid]
            self.assertEqual((source['accessed_date'], source['access_method'], source['source_type']),
                             ('2026-10-01', 'internet_archive_raw_capture', 'primary_legislature_news_notice_archived'))
            self.assertEqual(source['published_date'], source['document_date'])
            self.assertEqual(sid, f'ru_duma_news_{CAPTURES[sid][1]}_{source["document_date"].replace("-", "")}')
            for claim in source['claims']:
                self.assertLessEqual(claim['attested_on'], source['document_date'], claim['id'])
                self.assertTrue(claim['uncertainty'] and claim['locator'], claim['id'])

    def test_holders_are_exactly_as_intended(self):
        invariants(self.packet)
        for rid in FACTION_ROLES:
            role = next(r for e in self.packet['institutions'] for r in e['roles'] if r['id'] == rid)
            for holder in role['holder_claims'][1:]:
                self.assertTrue(holder['note'].startswith(f"Observed on {holder['attested_on']}: State Duma news item"))
                self.assertTrue(holder['uncertainty'], holder['name'])
                for cid in holder['claim_ids']:
                    row = self.rows[cid]
                    self.assertEqual((row['holder_name'], row['role_id'], row['role_title']), (holder['name'], rid, TITLE), cid)
                    self.assertIn(row['event_kind'], HOLDER_KINDS, cid)
        zhi = next(r for e in self.packet['institutions'] for r in e['roles'] if r['id'] == LDPR)['holder_claims'][1]
        self.assertIn('fallback: a death claim only and no `until`', zhi['uncertainty'])
        self.assertIn("'Сегодня после затяжной болезни скончался'", zhi['note'])

    def test_claim_only_events_never_date_a_holder(self):
        for cid in NEVER_HOLDER:
            claim = self.claims[cid]
            kind = EVENTS[cid][1]
            if kind in ('election', 'selection_reported'):
                self.assertIn('never a holder date', claim['uncertainty'], cid)
            elif kind == 'acting_service_reported':
                self.assertIn('never a holder', claim['uncertainty'], cid)
                self.assertIn('временно исполняющим обязанности руководителя фракции', claim['text'])
            else:
                self.assertIn("never the holder's `until`", claim['uncertainty'], cid)
        self.assertIn('7 октября, был избран Владимир Васильев',
                      self.claims['ru_duma_news_52394_vasilyev_elected_er_head_20211007']['text'])
        self.assertIn('на заседании фракции 18 мая', self.claims['ru_duma_news_54314_slutsky_elected_ldpr_head_20220518']['text'])
        self.assertIn('(C01-27, C01-40)', self.claims['ru_duma_news_54314_slutsky_elected_ldpr_head_20220518']['uncertainty'])
        # Party office stays separate: the faction's 'Председатель ЛДПР' is quoted but feeds no party role.
        self.assertIn('Председатель ЛДПР', self.claims[DEATH]['text'])
        self.assertIn('party office and is not recorded here', self.claims[DEATH]['uncertainty'])

    def test_extracts_match_the_packet_and_their_snapshots(self):
        for sid in NEW_SOURCES:
            source, extract = self.sources[sid], self.extracts[sid]
            path = research.ROOT / source['snapshot']['path']
            data = path.read_bytes()
            self.assertEqual((len(data), hashlib.sha256(data).hexdigest()), (source['snapshot']['bytes'], source['snapshot']['sha256']))
            self.assertEqual(source['snapshot']['kind'], 'derived_factual_extract')
            self.assertTrue(path.name.startswith('russia-duma-news-') and path.name.endswith('-facts.json'), path.name)
            self.assertTrue(data.endswith(b'\n') and b'\r' not in data, sid)
            self.assertEqual(data.decode('utf-8'), json.dumps(extract, ensure_ascii=False, indent=2) + '\n')
            self.assertEqual(extract['format'], 'spheres-c01-derived-factual-table/v1')
            self.assertEqual((extract['source_id'], extract['source_url'], extract['original_url'], extract['accessed_date']),
                             (sid, source['url'], source['original_url'], source['accessed_date']))
            self.assertEqual((extract['published_date'], extract['document_date']), (source['published_date'], source['document_date']))
            self.assertFalse(extract['source_response_checked_in'])
            self.assertIn('not checked into this', extract['provenance_note'])
            self.assertIn("curl's default User-Agent", extract['provenance_note'])
            self.assertEqual([r['claim_id'] for r in extract['rows']], [c['id'] for c in source['claims']])
            for row, claim in zip(extract['rows'], source['claims']):
                day, kind, rid, obs = EVENTS[row['claim_id']]
                self.assertEqual((row['text'], row['locator'], row['attested_on']), (claim['text'], claim['locator'], claim['attested_on']))
                self.assertEqual((row['attested_on'], row['event_kind'], row['role_id'], row['review_observation']), (day, kind, rid, obs))
                self.assertEqual((row['observation_id'], row['role_title']), (rid[:-len('_head')], TITLE))
                self.assertIn(row['holder_name'], NAMES, row['claim_id'])
                self.assertTrue(row['printed_name'], row['claim_id'])
                self.assertNotIn('name', row)

    def test_response_identities_are_pinned_raw_captures(self):
        for sid, (size, digest) in RESPONSES.items():
            source, extract = self.sources[sid], self.extracts[sid]
            ts, n = CAPTURES[sid]
            url = f'https://web.archive.org/web/{ts}id_/http://duma.gov.ru/news/{n}/'
            self.assertLess(ts, '20260907')
            self.assertEqual((source['url'], extract['source_response_url']), (url, url))
            self.assertEqual(source['original_url'], f'http://duma.gov.ru/news/{n}/')
            self.assertEqual(extract['archive_capture_utc'], f'{ts[:4]}-{ts[4:6]}-{ts[6:8]}T{ts[8:10]}:{ts[10:12]}:{ts[12:]}Z')
            self.assertEqual((extract['source_response_bytes'], extract['source_response_sha256']), (size, digest))
            self.assertNotRegex(url, VOLATILE_URL)
            self.assertRegex(extract['stability_check'], r'downloaded at 2026-10-01T\d\d:\d\d:\d\dZ.* and again at 2026-10-01T')
            if sid in DECODED:
                self.assertEqual(extract['source_response_content_encoding'], 'gzip')
                self.assertEqual((extract['decoded_response_bytes'], extract['decoded_response_sha256']), DECODED[sid])
            else:
                self.assertNotIn('source_response_content_encoding', extract)
                self.assertIn('served uncompressed', extract['source_response_encoding'])

    def test_report_and_handoff_are_ready_for_review_and_close_nothing(self):
        for heading in ('Outcome', 'Observations', 'Sources added', 'Response identities and stability checks', 'Date ledger',
                        'Leads not imported', 'Sources attempted', 'Suggested next work orders',
                        'Integration notes (outside this packet', 'Checks'):
            self.section(heading)
        self.assertIn('ready_for_review', self.report)
        for sid in NEW_SOURCES:
            self.assertIn(RESPONSES[sid][1][:12], self.section('Response identities and stability checks'), sid)
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        for text in ('ready_for_review', 'russia-duma-faction-heads-2021-2026-46.md', 'claude/c01-ru-46', '3ee3357f',
                     'test_russia_duma_faction_heads_c01_46.py', 'pending Codex acceptance'):
            self.assertIn(text, handoff)
        index = research.build()
        country = next(p for p in index['countries'] if p['nation'] == 'Russia')
        self.assertFalse(country['country_census_complete'])
        self.assertEqual((country['role_observations'], country['source_claims'], country['mapping_pending']), (9, 322, 21))
        self.assertFalse(index['c01_complete'])

    def test_mutations_are_rejected(self):
        self.check(copy.deepcopy(self.packet))

        def role(packet, rid):
            return next(r for e in packet['institutions'] for r in e['roles'] if r['id'] == rid)

        def holder(packet, rid, name, day):
            return next(h for h in role(packet, rid)['holder_claims'] if h['name'] == name and h['attested_on'] == day)

        def setter(rid, name, day, key, value):
            def mutate(p):
                holder(p, rid, name, day)[key] = value
            return mutate

        def add_holder(rid, name, day, claim_ids, until=None):
            def mutate(p):
                r = role(p, rid)
                sources = []
                for cid in claim_ids:
                    sid = next(s['id'] for s in p['sources'] if any(c['id'] == cid for c in s['claims']))
                    if sid not in sources:
                        sources.append(sid)
                r['holder_claims'].append({'name': name, 'attested_on': day, 'from': None, 'until': until,
                                           'sources': sources, 'claim_ids': claim_ids, 'note': 'x', 'uncertainty': 'x'})
            return mutate

        def move(p):
            h = holder(p, LDPR, SLU, '2026-07-27')
            role(p, LDPR)['holder_claims'].remove(h)
            role(p, KPRF)['holder_claims'].append(h)

        def cross_claim(p):
            gov = role(p, 'ru_government_chairman')
            gov['claim_ids'].append('ru_duma_news_63980_slutsky_ldpr_head_20260727')
            gov['sources'].append('ru_duma_news_63980_20260727')

        def death_from_report(p):
            h = holder(p, LDPR, ZHI, '2021-12-22')
            h['claim_ids'][-1] = DEATH_REPORT
            h['sources'][-1] = 'ru_duma_news_53987_20220406'

        def end_original(p):
            role(p, LDPR)['holder_claims'][0]['until'] = '2022-04-06'

        def drop_original(p):
            role(p, ER)['holder_claims'].pop(0)

        def drop_end(p):
            holder(p, LDPR, ZHI, '2021-12-22')['until'] = None

        def cite_election(p):
            h = holder(p, LDPR, SLU, '2022-07-07')
            h['claim_ids'].insert(0, 'ru_duma_news_54314_slutsky_elected_ldpr_head_20220518')
            h['sources'].insert(0, 'ru_duma_news_54314_20220518')

        def beyond_cutoff(p):
            next(c for s in p['sources'] for c in s['claims'] if c['id'] == 'ru_duma_news_63980_slutsky_ldpr_head_20260727')['attested_on'] = '2026-09-08'

        def party_mapping(p):
            next(e for e in p['institutions'] if e['id'] == 'ru_duma_faction_20211012_ldpr')['represented_party_ids'] = ['Russia/ldpr']

        def snapshot(p):
            next(s for s in p['sources'] if s['id'] == 'ru_duma_news_54910_20220707')['snapshot']['sha256'] = '0' * 64

        def lifecycle(p):
            next(e for e in p['institutions'] if e['id'] == 'ru_duma_faction_20211012_ldpr')['lifecycle']['until'] = '2022-04-06'

        def entry_citation(p):
            e = next(e for e in p['institutions'] if e['id'] == 'ru_duma_faction_20211012_er')
            e['claim_ids'].append('ru_duma_news_63980_vasilyev_er_head_20260727')
            e['sources'].append('ru_duma_news_63980_20260727')

        def uncited_claim(p):
            role(p, NL)['claim_ids'].remove('ru_duma_news_52394_nechaev_heads_nl_reported_20211011')

        mutations = {
            'election as Слуцкий start': setter(LDPR, SLU, '2022-07-07', 'from', '2022-05-18'),
            'election as Васильев start': setter(ER, VAS, '2021-12-22', 'from', '2021-10-07'),
            'election day as attested day': setter(LDPR, SLU, '2022-07-07', 'attested_on', '2022-05-18'),
            'capture day as attested day': setter(LDPR, SLU, '2022-07-07', 'attested_on', '2022-07-10'),
            'latest attestation as end': setter(ER, VAS, '2026-07-27', 'until', '2026-07-27'),
            'successor start as Жириновский end': setter(LDPR, ZHI, '2021-12-22', 'until', '2022-05-18'),
            'acting service as holder': add_holder(LDPR, SLU, '2022-05-18',
                                                   ['ru_duma_news_54314_slutsky_acting_ldpr_head_reported_20220518']),
            'selection report as holder': add_holder(NL, NEC, '2021-10-11', ['ru_duma_news_52394_nechaev_heads_nl_reported_20211011']),
            'election cited by holder': cite_election,
            'death report without a day as end': death_from_report,
            'end on the unchanged original holder': end_original,
            'original holder removed': drop_original,
            'death end removed': drop_end,
            'cross-role holder': move,
            'cross-role claim on the Government': cross_claim,
            'faction entry cites a new claim': entry_citation,
            'patronymic name': setter(LDPR, SLU, '2026-07-27', 'name', 'Леонид Эдуардович Слуцкий'),
            'printed genitive name': setter(NL, NEC, '2022-04-06', 'name', 'Алексея Нечаева'),
            'claim beyond cutoff': beyond_cutoff,
            'party mapping': party_mapping,
            'faction lifecycle end': lifecycle,
            'snapshot checksum mismatch': snapshot,
            'claim left uncited by its role': uncited_claim,
        }
        for label, mutate in mutations.items():
            packet = copy.deepcopy(self.packet)
            mutate(packet)
            with self.assertRaises((AssertionError, ValueError, KeyError, IndexError, StopIteration), msg=label):
                self.check(packet)


if __name__ == '__main__':
    unittest.main()
