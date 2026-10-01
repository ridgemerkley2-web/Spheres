"""CLAUDE-C01-50: Saudi prime ministers 1990-2026 (role sa_pm), from Council of Ministers records and royal orders."""
import copy
from datetime import datetime, timedelta
import hashlib
import json
import re
import unittest

import campaign_research as research


REPORT = research.RESEARCH / 'saudi-arabia-prime-ministers-1990-2026-50.md'
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-50.md'
BASE_SOURCES = 121  # CLAUDE-C01-06, 25 and 45 sources come first.
BASE_CLAIMS = 188
ACCESSED = '2026-10-01'
FAHD, ABD = 'Fahd bin Abdulaziz Al Saud', 'Abdullah bin Abdulaziz Al Saud'
SAL, MBS = 'Salman bin Abdulaziz Al Saud', 'Mohammed bin Salman bin Abdulaziz Al Saud'

# Response identities as recorded (bytes, SHA-256); each was downloaded twice at least 30 minutes apart.
RESPONSES = {
    'sa_embassy_fahd_cabinet_19960305': (50505, 'ba272c789a523bc36b0ba146d42da72e8790da3e91ce3dc6811baede74c0bf9f'),
    'sa_spa_fahd_pm_styled_20041003': (2797, '1564a16ba0db82019ce5683cdafc7273ba4e66551e393cc7769ee894d636bfb1'),
    'sa_spa_fahd_cabinet_20050425': (3339, 'fa1568ec0a7c813b7161663b9eb5d610249eabe7dbb6ad2b7945f238d66bd30d'),
    'sa_spa_order_a194_20050801': (2608, '4e0f481a0f5e762c06e468a407b012f2fa0729b8bb1bec7909df1fb606ecfb53'),
    'sa_spa_order_a29_20070322': (2734, '19e1ab1d2aedeb7f279f17ca70c9b82f06b808ad4d9aa1389bddc97da0bc8fd1'),
    'sa_spa_abdullah_cabinet_20121229': (4104, 'ca5bb338e33ef826d684aaf5df5e69177d1fa30b317c604c3014e223e2991232'),
    'sa_spa_order_a54_20150123': (2552, 'd33161c5205bc7cb72aaaa7465fbf512cc54a40927e3f2779108b6aa3ec044ba'),
    'sa_spa_order_a68_20150129': (4174, '2a16b1defafd918d0a9afb1b0a18078d3c8f1462a1457f70a2f54fb556dc7c18'),
    'sa_spa_order_a138_20181227': (71964, '350162ca482fbf61d0cbe334773ad0bc4bda0347724e046fdf7c4c7c17499605'),
    'sa_spa_salman_cabinet_20220517': (17272, '06aafe86e8ade9e11d8f00fba9b679b1f20cfab2e00ffb736fe5dc41e32096c1'),
    'sa_spa_order_a61_20220927': (10057, '57d979143b7bd438c314db0a9ddeba68634ed31a2d6229c849b1c80929f4f744'),
    'sa_spa_king_cabinet_20220927': (15884, 'e6a0b58d0f787b07f2171fa5bcca7d391efa36dfc0de19a27aa4463d7edc161d'),
    'sa_uqn_order_a61_20221007': (1701491, '6921275b222d715fba63e9730e7d516c99f2ab06091ea60d4b2211231e4c2561'),
    'sa_spa_mbs_cabinet_20221025': (14747, '3ccd440aad4bca3517039f7b6d4d5c75f085d637e2b5377f7e0c31097928e596'),
    'sa_spa_mbs_cabinet_20260616': (20007, '77504694df7d64d3a306a71df2fc1db58152e11f0faa3104b471d98e5a45662e'),
}
NEW_SOURCES = list(RESPONSES)
CAPTURES = {
    'sa_embassy_fahd_cabinet_19960305': ('20000930104631', 'http://www.saudiembassy.net:80/press_release/96_spa/96_03.html'),
    'sa_uqn_order_a61_20221007': ('20250429135455', 'https://www.uqn.gov.sa/details?p=20294'),
}
# Every new claim in packet order: (attested_on, event kind, review observation, role or None, holder or None).
EVENTS = {
    'sa_fahd_pm_chairs_cabinet_19960304': ('1996-03-04', 'attestation', 'SA-PM-01', 'sa_pm', FAHD),
    'sa_abdullah_deputy_chairs_cabinet_19960311': ('1996-03-11', 'deputy_chaired_session', 'SA-PM-05', None, None),
    'sa_fahd_pm_styled_20041003': ('2004-10-03', 'attestation', 'SA-PM-01', 'sa_pm', FAHD),
    'sa_fahd_pm_chairs_cabinet_20050425': ('2005-04-25', 'attestation', 'SA-PM-01', 'sa_pm', FAHD),
    'sa_abdullah_pm_order_a194_20050801': ('2005-08-01', 'council_continued_by_royal_order', 'SA-PM-02', 'sa_pm', ABD),
    'sa_abdullah_pm_order_a29_20070322': ('2007-03-22', 'council_reconstituted_by_royal_order', 'SA-PM-02', 'sa_pm', ABD),
    'sa_abdullah_pm_chairs_cabinet_20121229': ('2012-12-29', 'attestation', 'SA-PM-02', 'sa_pm', ABD),
    'sa_salman_pm_order_a54_20150123': ('2015-01-23', 'council_continued_by_royal_order', 'SA-PM-03', 'sa_pm', SAL),
    'sa_salman_pm_order_a68_20150129': ('2015-01-29', 'council_reconstituted_by_royal_order', 'SA-PM-03', 'sa_pm', SAL),
    'sa_salman_pm_order_a138_20181227': ('2018-12-27', 'council_reconstituted_by_royal_order', 'SA-PM-03', 'sa_pm', SAL),
    'sa_salman_pm_chairs_cabinet_20220517': ('2022-05-17', 'attestation', 'SA-PM-03', 'sa_pm', SAL),
    'sa_mbs_pm_order_a61_20220927': ('2022-09-27', 'appointment_by_royal_order', 'SA-PM-04', 'sa_pm', MBS),
    'sa_king_chair_reservation_a61_20220927': ('2022-09-27', 'chair_reservation', 'SA-PM-05', None, None),
    'sa_mbs_pm_order_a62_20220927': ('2022-09-27', 'council_reconstituted_by_royal_order', 'SA-PM-04', 'sa_pm', MBS),
    'sa_king_chairs_cabinet_20220927': ('2022-09-27', 'session_chaired_by_king', 'SA-PM-05', None, None),
    'sa_uqn_a61_published_20221007': ('2022-10-07', 'gazette_publication', 'SA-PM-04', None, None),
    'sa_mbs_pm_chairs_cabinet_20221025': ('2022-10-25', 'attestation', 'SA-PM-04', 'sa_pm', MBS),
    'sa_mbs_pm_chairs_cabinet_20260616': ('2026-06-16', 'attestation', 'SA-PM-04', 'sa_pm', MBS),
}
HOLDER_KINDS = {'attestation', 'appointment_by_royal_order', 'council_continued_by_royal_order',
                'council_reconstituted_by_royal_order'}
NEVER_HOLDER_KINDS = {'deputy_chaired_session', 'chair_reservation', 'session_chaired_by_king', 'gazette_publication'}
# The CLAUDE-C01-06 entries stay first and unchanged; the four holders follow, each followed by its own observations.
C01_06_PM = ['sa_mbs_pm_appointment', 'sa_mbs_cp_pm_obs_20260813']
HOLDERS = [  # (name, attested_on, from, until, sources, claim_ids)
    (FAHD, '1996-03-04', None, None, ['sa_embassy_fahd_cabinet_19960305'], ['sa_fahd_pm_chairs_cabinet_19960304']),
    (ABD, '2005-08-01', None, None, ['sa_spa_order_a194_20050801'], ['sa_abdullah_pm_order_a194_20050801']),
    (SAL, '2015-01-23', None, None, ['sa_spa_order_a54_20150123'], ['sa_salman_pm_order_a54_20150123']),
    (MBS, '2022-09-27', None, None, ['sa_spa_order_a61_20220927'], ['sa_mbs_pm_order_a61_20220927']),
]
OBSERVATIONS = {
    FAHD: ['sa_fahd_pm_styled_20041003', 'sa_fahd_pm_chairs_cabinet_20050425'],
    ABD: ['sa_abdullah_pm_order_a29_20070322', 'sa_abdullah_pm_chairs_cabinet_20121229'],
    SAL: ['sa_salman_pm_order_a68_20150129', 'sa_salman_pm_order_a138_20181227', 'sa_salman_pm_chairs_cabinet_20220517'],
    MBS: ['sa_mbs_pm_order_a62_20220927', 'sa_mbs_pm_chairs_cabinet_20221025', 'sa_mbs_pm_chairs_cabinet_20260616'],
}
HOLDER_ORDER = C01_06_PM + [x for h in HOLDERS for x in [h[0]] + OBSERVATIONS[h[0]]]
BASE_PREFIX = {'sa_pm': (3, 4), 'sa_prime_minister': (3, 4)}  # Existing (sources, claim_ids) lengths that stay first.
BASE_UNRESOLVED = {'sa_prime_minister': 5}
OTHER_ROLES = ('sa_king', 'sa_crown_prince', 'sa_cabinet_ministers', 'sa_shura_chair', 'sa_shura_members',
               'sa_succession_chair', 'sa_succession_secretary', 'sa_succession_members', 'sa_municipal_members')
SPA_API_URL = re.compile(r'^https://portalapi\.spa\.gov\.sa/api/v1/news/([0-9a-z]{10,11}|N\d{7})$')
IA_URL = re.compile(r'^https://web\.archive\.org/web/(\d{14})id_/(\S+)$')
VOLATILE = re.compile(r'(?i)(cdx|wayback/available|/search|api/widget|api/article|[?&](q|query|page|pgno|start|rows|cb)=|'
                      r'views_count|www\.spa\.gov\.sa/)')
LEADS = ('wikipedia.org', 'news/f93316b9c3', 'news/84075314e3', 'news/859dea0e03', 'news/2c29108f57', 'news/98c4949860',
         'news/1134a54ece', 'news/N2666035', 'news/N2661413', 'news/N2653088', 'news/N2647669', 'news/N2643266',
         'decisions-and-regulations', '97_spa/97_01_4')


def pm_invariants(packet, extracts):
    """The CLAUDE-C01-50 rules, over any copy of the Saudi packet and its new extracts."""
    sources = {s['id']: s for s in packet['sources']}
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    entries = {e['id']: e for e in packet['institutions']}
    roles = {r['id']: r for e in packet['institutions'] for r in e['roles']}
    inst, pm = entries['sa_prime_minister'], roles['sa_pm']
    assert [s['id'] for s in packet['sources'][BASE_SOURCES:]] == NEW_SOURCES
    assert [c['id'] for sid in NEW_SOURCES for c in sources[sid]['claims']] == list(EVENTS)
    for cid, (day, kind, _, role, holder) in EVENTS.items():
        assert claims[cid]['attested_on'] == day and 'period' not in claims[cid], cid
        assert (kind in HOLDER_KINDS) == bool(role) == bool(holder) and (kind in NEVER_HOLDER_KINDS) != bool(role), cid
        holders_of = [r['id'] for r in roles.values() if cid in r['claim_ids']]
        if role:
            assert holders_of == ['sa_pm'] and cid not in inst['claim_ids'], cid
        else:
            assert not holders_of and cid in inst['claim_ids'], cid
        assert not [e for e in entries.values() if e is not inst and cid in e['claim_ids']], cid
    # Never-holder claims never become holder evidence on any role; the new claims feed sa_pm only.
    for role in roles.values():
        for entry in role['holder_claims']:
            for cid in [entry] if isinstance(entry, str) else entry['claim_ids']:
                if cid in EVENTS:
                    assert EVENTS[cid][1] in HOLDER_KINDS and role['id'] == 'sa_pm', (role['id'], cid)
    entries_ = pm['holder_claims']
    assert entries_[:len(C01_06_PM)] == C01_06_PM
    got = [h if isinstance(h, str) else h['name'] for h in entries_]
    assert got == HOLDER_ORDER, got
    dicts = [h for h in entries_ if isinstance(h, dict)]
    assert [(h['name'], h['attested_on'], h['from'], h['until'], h['sources'], h['claim_ids']) for h in dicts] == HOLDERS
    current = None
    for h in entries_[len(C01_06_PM):]:
        if isinstance(h, dict):
            assert list(h) == ['name', 'attested_on', 'from', 'until', 'sources', 'claim_ids', 'note', 'uncertainty'], h['name']
            assert all(EVENTS[cid][4] == h['name'] and EVENTS[cid][3] == 'sa_pm' for cid in h['claim_ids']), h['name']
            assert claims[h['claim_ids'][0]]['attested_on'] == h['attested_on'], h['name']
            # No source states the effective day of a premiership or its end: from and until stay null.
            assert h['from'] is None and h['until'] is None, h['name']
            current = h
        else:
            day, kind, _, role, name = EVENTS[h]
            assert kind in HOLDER_KINDS and name == current['name'] and current['attested_on'] <= day <= research.CUTOFF, h
    for a, b in zip(dicts, dicts[1:]):
        assert a['attested_on'] < b['attested_on']
        # A successor's start is never an end, and the last observation of a holder precedes the next holder's anchor.
        assert not a['until']
        assert max(EVENTS[c][0] for c in OBSERVATIONS[a['name']] + a['claim_ids']) <= b['attested_on']
    # Extracts carry the packet's claims exactly.
    for sid in NEW_SOURCES:
        ex, src = extracts[sid], sources[sid]
        rows = {r['claim_id']: r for r in ex['rows']}
        assert list(rows) == [c['id'] for c in src['claims']], sid
        for c in src['claims']:
            day, kind, obs, role, holder = EVENTS[c['id']]
            r = rows[c['id']]
            assert (r['text'], r['attested_on'], r['event_kind'], r['review_observation'], r['role_id'], r['holder_name'],
                    r['locator'], r['observation_id']) == (c['text'], day, kind, obs, role, holder, c['locator'],
                                                          'sa_prime_minister'), c['id']
            assert ('printed_name' in r) == bool(holder), c['id']
        assert (ex['source_response_bytes'], ex['source_response_sha256']) == RESPONSES[sid], sid
        assert ex['source_url'] == src['url'] and ex['source_id'] == sid and ex['scope_note'] == src['scope_note'], sid


class SaudiPrimeMinistersTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.raw = (research.ROOT / research.RESEARCH / 'saudi-arabia.json').read_text(encoding='utf-8')
        cls.packet = json.loads(cls.raw)
        cls.sources = {s['id']: s for s in cls.packet['sources']}
        cls.claims = {c['id']: c for s in cls.packet['sources'] for c in s['claims']}
        cls.entries = {e['id']: e for category in ('organizations', 'institutions') for e in cls.packet[category]}
        cls.roles = {r['id']: r for e in cls.entries.values() for r in e['roles']}
        cls.extract_bytes = {sid: (research.ROOT / cls.sources[sid]['snapshot']['path']).read_bytes() for sid in NEW_SOURCES}
        cls.extracts = {sid: json.loads(b) for sid, b in cls.extract_bytes.items()}
        cls.report = (research.ROOT / REPORT).read_text(encoding='utf-8')

    def validate(self, packet=None):
        return research.validate(packet or self.packet, research.ROOT, {'SaudiArabia'}, {'SaudiArabia': set()})

    def test_records_are_appended_classified_and_cited_once(self):
        ids = self.validate()
        self.assertEqual((len(NEW_SOURCES), len(EVENTS)), (15, 18))
        self.assertEqual((len(ids['sources']), len(ids['claims']), len(ids['entries']), len(ids['roles'])),
                         (BASE_SOURCES + 15, BASE_CLAIMS + 18, 12, 10))
        pm_invariants(self.packet, self.extracts)
        self.assertEqual({v[2] for v in EVENTS.values()}, {f'SA-PM-{n:02d}' for n in range(1, 6)})
        self.assertEqual(re.findall(r'^### (SA-PM-\d\d)\b', self.report, re.M), [f'SA-PM-{n:02d}' for n in range(1, 6)])
        order = list(EVENTS)
        for key, (n_sources, n_claims) in BASE_PREFIX.items():
            row = self.roles.get(key) or self.entries[key]
            new = row['claim_ids'][n_claims:]
            self.assertTrue(new and not set(row['claim_ids'][:n_claims]) & set(EVENTS), key)
            self.assertEqual(new, [cid for cid in order if cid in new], key)
            owners = list(dict.fromkeys(next(s for s in NEW_SOURCES if cid in {c['id'] for c in self.sources[s]['claims']})
                                        for cid in new))
            self.assertEqual(row['sources'][n_sources:], owners, key)
            self.assertFalse(set(row['sources'][:n_sources]) & set(NEW_SOURCES), key)
        unresolved = self.entries['sa_prime_minister']['coverage']['unresolved']
        added = [u for u in unresolved if 'CLAUDE-C01-50' in u]
        self.assertEqual(len(added), 3)
        self.assertEqual(unresolved[-3:], added)
        self.assertEqual(len(unresolved), BASE_UNRESOLVED['sa_prime_minister'] + 3)
        self.assertFalse([u for u in self.packet['coverage']['unresolved'] if 'CLAUDE-C01-50' in u])
        self.assertTrue(self.roles['sa_pm']['scope_note'].startswith('CLAUDE-C01-50.'))

    def test_holders_and_separation_are_exactly_as_intended(self):
        # At most ten people: four holders, nobody else named as a holder.
        self.assertEqual({v[4] for v in EVENTS.values() if v[4]}, {FAHD, ABD, SAL, MBS})
        for rid in OTHER_ROLES:
            self.assertFalse(set(self.roles[rid]['sources']) & set(NEW_SOURCES), rid)
            self.assertFalse(set(self.roles[rid]['claim_ids']) & set(EVENTS), rid)
        for eid, entry in self.entries.items():
            if eid != 'sa_prime_minister':
                self.assertFalse(set(entry['sources']) & set(NEW_SOURCES), eid)
                self.assertFalse(set(entry['claim_ids']) & set(EVENTS), eid)
        # The new holders cite only this packet's claims; the CLAUDE-C01-06 claims stay where they were.
        for h in self.roles['sa_pm']['holder_claims'][len(C01_06_PM):]:
            ids = [h] if isinstance(h, str) else h['claim_ids']
            self.assertTrue(set(ids) <= set(EVENTS), h)
        self.assertIn('period', self.claims['sa_mbs_pm_appointment'])
        self.assertNotIn('attested_on', self.claims['sa_mbs_pm_appointment'])
        for h in self.roles['sa_pm']['holder_claims']:
            if isinstance(h, dict):
                self.assertIn('until is null', h['uncertainty'])
                self.assertIn('from is null', h['uncertainty'])

    def test_dates_events_and_wording_never_collapse(self):
        text = lambda cid: self.claims[cid]['text']
        doubt = lambda cid: self.claims[cid]['uncertainty']
        # 1996: the King chairs ('yesterday'), then the Deputy Prime Minister chairs ('today'): two claims, one holder fact.
        self.assertIn('chairing the regular weekly session of the Council of Ministers yesterday', text('sa_fahd_pm_chairs_cabinet_19960304'))
        self.assertIn('March 5, 1996', text('sa_fahd_pm_chairs_cabinet_19960304'))
        self.assertIn('does not print the title Prime Minister', doubt('sa_fahd_pm_chairs_cabinet_19960304'))
        self.assertIn('Deputy Prime Minister and Commander of the National Guard', text('sa_abdullah_deputy_chairs_cabinet_19960311'))
        self.assertIn('never a Prime Minister observation', doubt('sa_abdullah_deputy_chairs_cabinet_19960311'))
        # Explicit stylings as رئيس مجلس الوزراء.
        self.assertIn('الملك فهد بن عبدالعزيز رئيس مجلس الوزراء', text('sa_fahd_pm_styled_20041003'))
        self.assertIn('الملك سلمان بن عبدالعزيز آل سعود رئيس مجلس الوزراء', text('sa_salman_pm_chairs_cabinet_20220517'))
        self.assertIn('ولي العهد رئيس مجلس الوزراء', text('sa_mbs_pm_chairs_cabinet_20221025'))
        # Royal orders: the Council 'برئاستنا', with an effective day only in A/29, which is never a from.
        for cid in ('sa_abdullah_pm_order_a194_20050801', 'sa_abdullah_pm_order_a29_20070322', 'sa_salman_pm_order_a54_20150123',
                    'sa_salman_pm_order_a68_20150129', 'sa_salman_pm_order_a138_20181227'):
            self.assertIn('برئاستنا', text(cid))
        self.assertIn('يعمل بهذا الأمر من تاريخه', text('sa_abdullah_pm_order_a29_20070322'))
        self.assertIn('never a from', doubt('sa_abdullah_pm_order_a29_20070322'))
        for cid in ('sa_abdullah_pm_order_a194_20050801', 'sa_salman_pm_order_a54_20150123', 'sa_mbs_pm_order_a61_20220927'):
            self.assertIn('no effective day', doubt(cid))
        self.assertIn('السادسة والخمسين', text('sa_mbs_pm_order_a61_20220927'))
        self.assertIn('تكون جلسات مجلس الوزراء التي نحضرها برئاستنا', text('sa_king_chair_reservation_a61_20220927'))
        self.assertIn('ولي العهد رئيساً لمجلس الوزراء', text('sa_mbs_pm_order_a62_20220927'))
        self.assertIn('does not style the King Prime Minister', doubt('sa_king_chairs_cabinet_20220927'))
        self.assertIn('never an effective boundary', doubt('sa_uqn_a61_published_20221007'))
        self.assertIn('ولي العهد رئيس مجلس الوزراء', text('sa_mbs_pm_chairs_cabinet_20260616'))
        self.assertIn('N2653088', doubt('sa_mbs_pm_chairs_cabinet_20260616'))
        self.assertIn('إعادة مصححة', text('sa_salman_pm_order_a54_20150123'))
        self.assertIn('f93316b9c3', doubt('sa_salman_pm_order_a54_20150123'))
        self.assertIn('9/4/1436', text('sa_salman_pm_order_a68_20150129'))
        self.assertIn('date_hijri field 1436-04-09', doubt('sa_salman_pm_order_a68_20150129'))
        # Ends: the death notices and the 2022 appointment never give an until.
        self.assertIn('names him King, not Prime Minister', doubt('sa_abdullah_pm_chairs_cabinet_20121229'))
        self.assertIn('states no day of death', doubt('sa_fahd_pm_chairs_cabinet_20050425'))
        self.assertIn('states no end', doubt('sa_salman_pm_chairs_cabinet_20220517'))
        for cid, (day, *_) in EVENTS.items():
            self.assertLessEqual(day, research.CUTOFF)
        for sid in NEW_SOURCES:
            self.assertLessEqual(self.sources[sid]['published_date'], research.CUTOFF)
            self.assertEqual(self.sources[sid]['accessed_date'], ACCESSED)

    def test_extracts_match_snapshots_and_record_reproducible_responses(self):
        for sid in NEW_SOURCES:
            with self.subTest(sid=sid):
                src, ex, data = self.sources[sid], self.extracts[sid], self.extract_bytes[sid]
                self.assertEqual((src['snapshot']['bytes'], src['snapshot']['sha256']), (len(data), hashlib.sha256(data).hexdigest()))
                self.assertEqual(src['snapshot']['kind'], 'derived_factual_extract')
                self.assertEqual(src['snapshot']['path'], f'{research.RESEARCH.as_posix()}/sources/saudi-arabia-{sid[3:].replace("_", "-")}-facts.json')
                self.assertEqual(data.decode('utf-8'), json.dumps(ex, indent=2, ensure_ascii=False) + '\n')
                self.assertNotIn(b'\r', data)
                self.assertEqual(ex['format'], 'spheres-c01-derived-factual-table/v1')
                self.assertEqual((ex['accessed_date'], ex['published_date']), (src['accessed_date'], src['published_date']))
                self.assertIn('not checked into this repository', ex['provenance_note'])
                self.assertFalse(ex['source_response_checked_in'])
                self.assertEqual(ex['source_response_url'], src['url'])
                self.assertTrue(ex['source_response_encoding'].startswith('identity'))
                self.assertIn('both identical', ex['stability_check'])
                self.assertIn(f"{RESPONSES[sid][0]} bytes, SHA-256 {RESPONSES[sid][1]}", ex['stability_check'])
                self.assertFalse(VOLATILE.search(src['url']), sid)
                for lead in LEADS:
                    self.assertNotIn(lead, src['url'])
                if sid in CAPTURES:
                    m = IA_URL.match(src['url'])
                    self.assertTrue(m and (m.group(1), m.group(2)) == CAPTURES[sid] and m.group(2) == src['original_url'])
                    self.assertLess(CAPTURES[sid][0], '20260907')
                    self.assertEqual(ex['archive_capture_utc'][:10].replace('-', ''), CAPTURES[sid][0][:8])
                else:
                    m = SPA_API_URL.match(src['url'])
                    self.assertTrue(m, sid)
                    self.assertEqual(src['original_url'], f'https://www.spa.gov.sa/{m.group(1)}')
                    self.assertEqual((ex['portal_metadata']['uuid'], ex['portal_metadata']['locale']), (m.group(1), 'ar'))
                    self.assertRegex(ex['article_content_sha256'], r'^[0-9a-f]{64}$')
                    self.assertTrue(src['source_type'].startswith('primary_'))
                    # The publication date is the Makkah day of filing for every SPA item here.
                    stamp = datetime.fromisoformat(ex['portal_metadata']['published_at_utc'].replace('Z', '+00:00'))
                    self.assertEqual(src['published_date'], (stamp + timedelta(hours=3)).date().isoformat())
        self.assertEqual(self.sources['sa_uqn_order_a61_20221007']['source_type'], 'primary_official_gazette_website_text')
        self.assertEqual(self.sources['sa_embassy_fahd_cabinet_19960305']['source_type'], 'primary_official_government_news_release')

    def test_report_and_handoff_are_ready_for_review_and_close_nothing(self):
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        self.assertIn('State: **ready_for_review**', handoff)
        self.assertIn('State: **ready_for_review** (not complete)', self.report.split('\n## Outcome')[0])
        for heading in ('Outcome', 'Sources added', 'Response identities and stability checks', 'Date ledger',
                        'Leads not imported', 'Sources attempted', 'Suggested next work orders', 'Integration notes',
                        'Decisions for Codex', 'Checks'):
            self.assertRegex(self.report, rf'\n##+ {heading}( \([^)\n]*\))?\n')
        for sid in NEW_SOURCES:
            self.assertIn(f'`{sid}`', self.report)
            self.assertIn(RESPONSES[sid][1], self.report)
        for marker in ('C01', 'C06', 'S23', 'WC1', 'CP1'):
            self.assertNotRegex(self.report, rf'\b{marker}\b[^.\n]*\bis (now )?complete\b')

    def test_packet_formatting_is_preserved(self):
        data = (research.ROOT / research.RESEARCH / 'saudi-arabia.json').read_bytes()
        self.assertNotIn(b'\r', data)
        self.assertEqual(data.decode('utf-8'), json.dumps(self.packet, indent=2, ensure_ascii=False) + '\n')

    def test_mutations_are_rejected(self):
        pm_invariants(self.packet, self.extracts)

        def mutated(change):
            packet, extracts = copy.deepcopy(self.packet), copy.deepcopy(self.extracts)
            change(packet, extracts)
            return packet, extracts

        roles = lambda p: {r['id']: r for e in p['institutions'] for r in e['roles']}
        holder = lambda p, name: next(h for h in roles(p)['sa_pm']['holder_claims'] if isinstance(h, dict) and h['name'] == name)
        claim = lambda p, cid: next(c for s in p['sources'] for c in s['claims'] if c['id'] == cid)
        pm = lambda p: roles(p)['sa_pm']['holder_claims']

        def salman_until_from_successor(p, e):
            holder(p, SAL)['until'] = '2022-09-27'

        def mbs_from_order_day(p, e):
            holder(p, MBS)['from'] = '2022-09-27'

        def mbs_from_gazette(p, e):
            holder(p, MBS)['from'] = '2022-10-07'

        def abdullah_from_reconstitution(p, e):
            holder(p, ABD)['from'] = '2007-03-22'

        def abdullah_until_from_death_notice(p, e):
            holder(p, ABD)['until'] = '2015-01-23'

        def king_session_after_reservation(p, e):
            pm(p).append('sa_king_chairs_cabinet_20220927')

        def deputy_chair_as_observation(p, e):
            pm(p).insert(pm(p).index('sa_fahd_pm_styled_20041003'), 'sa_abdullah_deputy_chairs_cabinet_19960311')

        def observation_on_wrong_tenure(p, e):
            i, j = pm(p).index('sa_fahd_pm_chairs_cabinet_20050425'), pm(p).index('sa_abdullah_pm_order_a29_20070322')
            pm(p)[i], pm(p)[j] = pm(p)[j], pm(p)[i]

        def existing_entry_dropped(p, e):
            pm(p).remove('sa_mbs_pm_appointment')

        def observation_on_king_role(p, e):
            roles(p)['sa_king']['holder_claims'].append('sa_salman_pm_chairs_cabinet_20220517')

        def reservation_on_role(p, e):
            inst = next(x for x in p['institutions'] if x['id'] == 'sa_prime_minister')
            inst['claim_ids'].remove('sa_king_chair_reservation_a61_20220927')
            roles(p)['sa_pm']['claim_ids'].append('sa_king_chair_reservation_a61_20220927')

        def relative_day_misresolved(p, e):
            claim(p, 'sa_fahd_pm_chairs_cabinet_19960304')['attested_on'] = '1996-03-05'

        def claim_text_drifts(p, e):
            claim(p, 'sa_mbs_pm_order_a61_20220927')['text'] += ' (edited)'

        def response_identity_changed(p, e):
            e['sa_spa_order_a61_20220927']['source_response_bytes'] += 1

        def extract_locator_drift(p, e):
            e['sa_spa_order_a194_20050801']['rows'][0]['locator'] = 'Unrelated order'

        for change in (salman_until_from_successor, mbs_from_order_day, mbs_from_gazette, abdullah_from_reconstitution,
                       abdullah_until_from_death_notice, king_session_after_reservation, deputy_chair_as_observation,
                       observation_on_wrong_tenure, existing_entry_dropped, observation_on_king_role, reservation_on_role,
                       relative_day_misresolved, claim_text_drifts, response_identity_changed, extract_locator_drift):
            with self.subTest(change=change.__name__):
                with self.assertRaises((AssertionError, KeyError, StopIteration, TypeError)):
                    pm_invariants(*mutated(change))


if __name__ == '__main__':
    unittest.main()
