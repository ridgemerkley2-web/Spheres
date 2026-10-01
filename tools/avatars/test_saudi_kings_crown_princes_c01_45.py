"""CLAUDE-C01-45: Saudi primary attestations of the kings and crown princes, and the Allegiance Commission secretary."""
import copy
from datetime import datetime, timedelta
import hashlib
import json
import re
import unittest

import campaign_research as research


REPORT = research.RESEARCH / 'saudi-arabia-kings-crown-princes-1990-2026-45.md'
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-45.md'
BASE_SOURCES = 103  # CLAUDE-C01-06 and CLAUDE-C01-25 sources come first.
ACCESSED = '2026-09-30'
FAHD, ABD, SAL = 'Fahd bin Abdulaziz Al Saud', 'Abdullah bin Abdulaziz Al Saud', 'Salman bin Abdulaziz Al Saud'
SUL, NAY, MUQ = 'Sultan bin Abdulaziz Al Saud', 'Nayef bin Abdulaziz Al Saud', 'Muqrin bin Abdulaziz Al Saud'
MBN, MBS = 'Mohammed bin Nayef bin Abdulaziz Al Saud', 'Mohammed bin Salman bin Abdulaziz Al Saud'
TUW = 'Khalid bin Abdulaziz Al-Tuwaijri'

# Response identities as recorded (bytes, SHA-256); each was downloaded twice at least 30 minutes apart.
RESPONSES = {
    'sa_embassy_fahd_delegates_19960101': (62784, '383724bf0bb2b23e510f1f459862a4f761d904a51d0ab07ea11eb223617068f3'),
    'sa_embassy_fahd_resumes_19960222': (24905, '1b3bcaf97bdd44ceabaec4d9760f0e15bf18d065bbea79ba88c4d08bf8d80f8d'),
    'sa_spa_cp_abdullah_obs_20050730': (2428, '538c33ba91ba710ae98dc41467a2f35574eab1d705de86f311b06727628aaf49'),
    'sa_spa_order_a193_20050731': (2869, '6f89b870a9461c4fc0acac7be1fea895f6201f73f6bbd9f0c6827dd07167be5b'),
    'sa_spa_pledge_held_20050803': (2143, '1c0a88ea99f2da901121a14012499bc1a2bb3d7af0937e85c1b1e453e6c9efb4'),
    'sa_spa_order_a136_20061020': (2420, '7c4a3614486727eebeb91ee83a6b50d8b64596ae9ba2b44153ab3860e560a4ac'),
    'sa_spa_allegiance_law_art24_20061020': (1971, '79b60c3a2abfa239dca5cf73fa73dcdd663c8af23f7f022daf76cdf36ef89639'),
    'sa_spa_order_a175_20071029': (2647, 'af1798d8e4db08fa31810590ea4add8f7dc002bdd1efc1e39da2953f0b75c47a'),
    'sa_spa_nayef_hajj_reception_20111107': (2002, '13dc061370249299b21b43a5e4c74e3f42c5d4342b0243f2ee0cd278ebec0846'),
    'sa_spa_salman_cp_pledge_directive_20120618': (2151, '2bf582b00d7e1235e8645fba089ac1ed411dba43d3baec1fd34d52a73ee61e8a'),
    'sa_spa_order_a145_20140520': (2518, '212b35ec541369eb2f4d4f30b8b5884db60eb45ed1d1af56f61a01b46ba68061'),
    'sa_spa_pledge_held_20150123': (2678, 'd8d7885c13a4090690a3fd58a8016f18270d7e34585b693f8b5541624ea371d5'),
    'sa_spa_muqrin_cp_obs_20150428': (2983, 'b905fc2cb7901e3d72c7bcc186368369268c1ec4a0e3923f378d66281353f8bd'),
    'sa_spa_order_a267_20150725': (2327, '1c718e8b83e7b5964a8417a673912b869eabb96955154b29da6eb1ec82aacad9'),
    'sa_spa_order_a128_20170225': (2487, '5df78e647ba99333432e856f0f14b8e10797b3e574b82b7607cbb3d53842f63c'),
    'sa_spa_mbs_pledge_held_20170621': (2356, 'f59c465ae43af836a1b9fbb4eb5770e99f0cf0e11ff0af42dc0c7f2f6b6e52d2'),
    'sa_spa_mbs_cabinet_20260901': (12812, '069441cd3fe0f4f2ffb804c492f5e1f832fddd1b9d800a55768a66b51ba5ffc8'),
    'sa_spa_king_order_judges_20260904': (3328, '2a7fa39ac3bbb2c69f7f55692f226680701b3608b48898bcaae7c87e6dc26862'),
}
NEW_SOURCES = list(RESPONSES)
# Printed date lines kept although the portal filed the item on the next Makkah day: (published_date, filing day).
DATELINE_KEPT = {'sa_spa_salman_cp_pledge_directive_20120618': ('2012-06-18', '2012-06-19'),
                 'sa_spa_mbs_pledge_held_20170621': ('2017-06-21', '2017-06-22')}
CAPTURES = {'sa_embassy_fahd_delegates_19960101': '20000930104650', 'sa_embassy_fahd_resumes_19960222': '19991003024900'}
# Every new claim in packet order: (attested_on, event kind, review observation, role or None, holder or None).
EVENTS = {
    'sa_fahd_king_obs_19960101': ('1996-01-01', 'attestation', 'SA-KCP-01', 'sa_king', FAHD),
    'sa_abdullah_cp_obs_19960101': ('1996-01-01', 'attestation', 'SA-KCP-03', 'sa_crown_prince', ABD),
    'sa_fahd_delegation_19960101': ('1996-01-01', 'delegation_of_state_affairs', 'SA-KCP-02', None, None),
    'sa_fahd_chairs_cabinet_19960212': ('1996-02-12', 'attestation', 'SA-KCP-01', 'sa_king', FAHD),
    'sa_fahd_delegation_ended_19960221': ('1996-02-21', 'delegation_ended', 'SA-KCP-02', None, None),
    'sa_abdullah_cp_obs_20050730': ('2005-07-30', 'attestation', 'SA-KCP-03', 'sa_crown_prince', ABD),
    'sa_fahd_king_order_a193_20050731': ('2005-07-31', 'attestation', 'SA-KCP-01', 'sa_king', FAHD),
    'sa_a193_signed_on_behalf_20050731': ('2005-07-31', 'signature_on_behalf', 'SA-KCP-01', None, None),
    'sa_citizen_pledge_held_20050803': ('2005-08-03', 'pledge_held', 'SA-KCP-04', None, None),
    'sa_tuwaijri_sg_appointed_a136_20061020': ('2006-10-20', 'appointment_by_royal_order', 'SA-KCP-09', 'sa_succession_secretary', TUW),
    'sa_abdullah_king_order_a136_20061020': ('2006-10-20', 'attestation', 'SA-KCP-04', 'sa_king', ABD),
    'sa_allegiance_law_art24_secretary': (None, 'procedure', 'SA-KCP-09', 'sa_succession_secretary', None),
    'sa_sultan_cp_obs_a175_20071029': ('2007-10-29', 'attestation', 'SA-KCP-05', 'sa_crown_prince', SUL),
    'sa_sultan_deputized_a175_20071029': ('2007-10-29', 'delegation_of_state_affairs', 'SA-KCP-05', None, None),
    'sa_nayef_cp_obs_20111107': ('2011-11-07', 'attestation', 'SA-KCP-05', 'sa_crown_prince', NAY),
    'sa_salman_cp_obs_20120618': ('2012-06-18', 'attestation', 'SA-KCP-05', 'sa_crown_prince', SAL),
    'sa_salman_cp_pledge_scheduled_20120618': ('2012-06-18', 'pledge_scheduled', 'SA-KCP-05', None, None),
    'sa_abdullah_king_order_a145_20140520': ('2014-05-20', 'attestation', 'SA-KCP-04', 'sa_king', ABD),
    'sa_salman_cp_obs_a145_20140520': ('2014-05-20', 'attestation', 'SA-KCP-05', 'sa_crown_prince', SAL),
    'sa_salman_deputized_a145_20140520': ('2014-05-20', 'delegation_of_state_affairs', 'SA-KCP-05', None, None),
    'sa_citizen_pledge_held_20150123': ('2015-01-23', 'pledge_held', 'SA-KCP-06', None, None),
    'sa_muqrin_cp_obs_20150428': ('2015-04-28', 'attestation', 'SA-KCP-06', 'sa_crown_prince', MUQ),
    'sa_salman_king_order_a267_20150725': ('2015-07-25', 'attestation', 'SA-KCP-07', 'sa_king', SAL),
    'sa_mbn_cp_obs_a267_20150725': ('2015-07-25', 'attestation', 'SA-KCP-07', 'sa_crown_prince', MBN),
    'sa_mbn_deputized_a267_20150725': ('2015-07-25', 'delegation_of_state_affairs', 'SA-KCP-07', None, None),
    'sa_mbn_cp_obs_a128_20170225': ('2017-02-25', 'attestation', 'SA-KCP-07', 'sa_crown_prince', MBN),
    'sa_mbn_deputized_a128_20170225': ('2017-02-25', 'delegation_of_state_affairs', 'SA-KCP-07', None, None),
    'sa_mbs_pledge_held_20170621': ('2017-06-21', 'pledge_held', 'SA-KCP-08', None, None),
    'sa_mbs_cp_obs_20260901': ('2026-09-01', 'attestation', 'SA-KCP-08', 'sa_crown_prince', MBS),
    'sa_salman_king_obs_20260904': ('2026-09-04', 'attestation', 'SA-KCP-08', 'sa_king', SAL),
}
NEVER_HOLDER_KINDS = {'delegation_of_state_affairs', 'delegation_ended', 'signature_on_behalf', 'pledge_held',
                      'pledge_scheduled', 'procedure'}
# CLAUDE-C01-06 holders, unchanged: (name or claim ID, attested_on, from, until); this packet appends observations only.
C01_06_HOLDERS = {
    'sa_king': ['sa_salman_king_observation', (FAHD, '1990-08-08', None, None), (ABD, '2005-08-01', None, '2015-01-23'),
                (SAL, '2015-01-23', None, None), 'sa_salman_king_obs_20260813'],
    'sa_crown_prince': ['sa_mbs_pm_appointment', (ABD, '2005-08-01', None, None), (SUL, '2005-08-01', None, '2011-10-22'),
                        (NAY, '2011-10-27', None, '2012-06-16'), (SAL, '2012-06-18', None, '2015-01-23'),
                        (MUQ, '2015-01-23', None, '2015-04-29'), (MBN, '2015-04-29', None, '2017-06-21'),
                        (MBS, '2017-06-21', None, None), 'sa_mbs_cp_pm_obs_20260813'],
}
APPENDED_OBSERVATIONS = {
    'sa_king': ['sa_fahd_king_obs_19960101', 'sa_fahd_chairs_cabinet_19960212', 'sa_fahd_king_order_a193_20050731',
                'sa_abdullah_king_order_a136_20061020', 'sa_abdullah_king_order_a145_20140520',
                'sa_salman_king_order_a267_20150725', 'sa_salman_king_obs_20260904'],
    'sa_crown_prince': ['sa_abdullah_cp_obs_19960101', 'sa_abdullah_cp_obs_20050730', 'sa_sultan_cp_obs_a175_20071029',
                        'sa_nayef_cp_obs_20111107', 'sa_salman_cp_obs_20120618', 'sa_salman_cp_obs_a145_20140520',
                        'sa_muqrin_cp_obs_20150428', 'sa_mbn_cp_obs_a267_20150725', 'sa_mbn_cp_obs_a128_20170225',
                        'sa_mbs_cp_obs_20260901'],
}
# Observations before the existing holder's attested_on, kept as observations pending a ruling (the holder is unchanged).
BEFORE_ATTESTED = {'sa_abdullah_cp_obs_19960101', 'sa_abdullah_cp_obs_20050730'}
SECRETARY = (TUW, '2006-10-20', None, None, ['sa_spa_order_a136_20061020'], ['sa_tuwaijri_sg_appointed_a136_20061020'])
BASE_PREFIX = {  # Existing (sources, claim_ids) lengths that stay first, unchanged.
    'sa_king': (11, 14), 'sa_crown_prince': (18, 25), 'sa_succession_secretary': (1, 1), 'sa_crown': (6, 6),
}
BASE_UNRESOLVED = {'sa_crown': 8, 'sa_succession_commission': 6}
SPA_API_URL = re.compile(r'^https://portalapi\.spa\.gov\.sa/api/v1/news/([0-9a-f]{10}|N\d{7})$')
IA_URL = re.compile(r'^https://web\.archive\.org/web/(\d{14})id_/(http://www\.saudiembassy\.net:80/press_release/96_spa/96_0[12]\.html)$')
VOLATILE = re.compile(r'(?i)(cdx|wayback/available|/search|[?&](q|query|page|start|rows|cb)=|views_count|uqn\.gov\.sa)')
LEADS = ('wikipedia.org', 'news/7a34c6edb4', 'news/d25612a743', 'news/051683c0fa', 'news/106b9a7f0a', 'news/724439bffa',
         'news/7f343c89aa', 'news/5b0cd66f79', 'news/359e4f88c1', 'news/f62f808111', 'news/e15afcc823', 'bush41library')


def kcp_invariants(packet, extracts):
    """The CLAUDE-C01-45 rules, over any copy of the Saudi packet and its new extracts."""
    sources = {s['id']: s for s in packet['sources']}
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    entries = {e['id']: e for e in packet['institutions']}
    roles = {r['id']: r for e in packet['institutions'] for r in e['roles']}
    assert [s['id'] for s in packet['sources'][BASE_SOURCES:]] == NEW_SOURCES
    assert [c['id'] for sid in NEW_SOURCES for c in sources[sid]['claims']] == list(EVENTS)
    for cid, (day, kind, _, role, holder) in EVENTS.items():
        assert claims[cid].get('attested_on') == day and 'period' not in claims[cid], cid
        if role:
            assert cid in roles[role]['claim_ids'] and cid not in entries['sa_crown']['claim_ids'], cid
            assert [r for r in roles.values() if cid in r['claim_ids']] == [roles[role]], cid
        else:
            assert kind in NEVER_HOLDER_KINDS and cid in entries['sa_crown']['claim_ids'], cid
            assert not [r for r in roles.values() if cid in r['claim_ids']], cid
    # Holder evidence: attestations and the one appointment only; ceremonies, acting service and procedure never.
    for role in roles.values():
        for entry in role['holder_claims']:
            for cid in [entry] if isinstance(entry, str) else entry['claim_ids']:
                if cid in EVENTS:
                    assert EVENTS[cid][1] not in NEVER_HOLDER_KINDS and EVENTS[cid][3] == role['id'], (role['id'], cid)
    for role_id, base in C01_06_HOLDERS.items():
        entries_ = roles[role_id]['holder_claims']
        got = [h if isinstance(h, str) else (h['name'], h['attested_on'], h['from'], h['until']) for h in entries_]
        assert got == base + APPENDED_OBSERVATIONS[role_id], role_id
        tenures = {}
        for h in entries_:
            if isinstance(h, dict):
                tenures.setdefault(h['name'], []).append(h)
        for cid in APPENDED_OBSERVATIONS[role_id]:
            day, kind, _, _, name = EVENTS[cid]
            assert kind == 'attestation' and len(tenures[name]) == 1, cid
            h = tenures[name][0]
            assert day <= (h['until'] or research.CUTOFF), cid
            assert (day < h['attested_on']) == (cid in BEFORE_ATTESTED), cid
    sec = roles['sa_succession_secretary']['holder_claims']
    assert len(sec) == 1 and (sec[0]['name'], sec[0]['attested_on'], sec[0]['from'], sec[0]['until'], sec[0]['sources'],
                              sec[0]['claim_ids']) == SECRETARY
    assert list(sec[0]) == ['name', 'attested_on', 'from', 'until', 'sources', 'claim_ids', 'note', 'uncertainty']
    # Extracts carry the packet's claims exactly.
    for sid in NEW_SOURCES:
        ex, src = extracts[sid], sources[sid]
        rows = {r['claim_id']: r for r in ex['rows']}
        assert list(rows) == [c['id'] for c in src['claims']], sid
        for c in src['claims']:
            day, kind, obs, role, holder = EVENTS[c['id']]
            r = rows[c['id']]
            assert (r['text'], r['attested_on'], r['event_kind'], r['review_observation'], r['role_id'], r['holder_name']) == \
                (c['text'], day, kind, obs, role, holder), c['id']
            assert r['observation_id'] == ('sa_succession_commission' if role == 'sa_succession_secretary' else 'sa_crown')
        assert (ex['source_response_bytes'], ex['source_response_sha256']) == RESPONSES[sid], sid
        assert ex['source_url'] == src['url'] and ex['source_id'] == sid, sid
        assert ex['scope_note'] == src['scope_note'], sid
        for c in src['claims']:
            assert rows[c['id']]['locator'] == c['locator'], c['id']


class SaudiKingsCrownPrincesTests(unittest.TestCase):
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
        self.assertEqual((len(NEW_SOURCES), len(EVENTS)), (18, 30))
        self.assertEqual((len(ids['sources']), len(ids['claims']), len(ids['entries']), len(ids['roles'])), (121, 188, 12, 10))
        kcp_invariants(self.packet, self.extracts)
        self.assertEqual({v[2] for v in EVENTS.values()}, {f'SA-KCP-{n:02d}' for n in range(1, 10)})
        self.assertEqual(re.findall(r'^### (SA-KCP-\d\d)\b', self.report, re.M), [f'SA-KCP-{n:02d}' for n in range(1, 10)])
        # Existing sources and claim IDs stay first and in order; the new ones follow in packet order.
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
        for key, n in BASE_UNRESOLVED.items():
            unresolved = self.entries[key]['coverage']['unresolved']
            added = [u for u in unresolved if 'CLAUDE-C01-45' in u]
            self.assertEqual(unresolved[-len(added):], added, key)
            self.assertEqual(len(unresolved), n + len(added), key)
        self.assertEqual(len([u for u in self.entries['sa_crown']['coverage']['unresolved'] if 'CLAUDE-C01-45' in u]), 4)
        self.assertFalse([u for u in self.packet['coverage']['unresolved'] if 'CLAUDE-C01-45' in u])
        self.assertTrue(self.roles['sa_succession_secretary']['scope_note'].startswith('CLAUDE-C01-45.'))

    def test_holders_and_observations_are_exactly_as_intended(self):
        # At most ten people: eight existing holders, the Secretary General and nobody else.
        names = {v[4] for v in EVENTS.values() if v[4]}
        self.assertEqual(names, {FAHD, ABD, SAL, SUL, NAY, MUQ, MBN, MBS, TUW})
        sec = self.roles['sa_succession_secretary']['holder_claims'][0]
        self.assertIn('no effective day', sec['uncertainty'])
        self.assertIn('author-supplied leads, not independently reviewed evidence', sec['uncertainty'])
        self.assertIn("أمينا عاما لهيئة البيعة", self.claims['sa_tuwaijri_sg_appointed_a136_20061020']['text'])
        self.assertIn('Article 24', self.claims['sa_tuwaijri_sg_appointed_a136_20061020']['text'])
        self.assertIn("يعين الملك أمينا عاما للهيئة", self.claims['sa_allegiance_law_art24_secretary']['text'])
        self.assertNotIn('attested_on', self.claims['sa_allegiance_law_art24_secretary'])
        for cid in BEFORE_ATTESTED:
            self.assertIn('predates his CLAUDE-C01-06', self.claims[cid]['uncertainty'])
        # The Prime Minister role and the two chair roles are untouched by this packet.
        for rid in ('sa_pm', 'sa_shura_chair', 'sa_succession_chair', 'sa_cabinet_ministers', 'sa_succession_members'):
            self.assertFalse(set(self.roles[rid]['sources']) & set(NEW_SOURCES), rid)
            self.assertFalse(set(self.roles[rid]['claim_ids']) & set(EVENTS), rid)

    def test_dates_events_and_wording_never_collapse(self):
        text = lambda cid: self.claims[cid]['text']
        doubt = lambda cid: self.claims[cid]['uncertainty']
        # 1996: the delegation, a cabinet session during it, and its end are three dated claims.
        self.assertIn('we hereby delegate your Highness', text('sa_fahd_delegation_19960101'))
        self.assertIn('yesterday evening chaired', text('sa_fahd_chairs_cabinet_19960212'))
        self.assertIn('February 13, 1996', text('sa_fahd_chairs_cabinet_19960212'))
        self.assertIn('A-112 dated 11/30/95', text('sa_fahd_delegation_ended_19960221'))
        self.assertIn('A-199 dated 02/21/96', text('sa_fahd_delegation_ended_19960221'))
        self.assertIn('prints no year', doubt('sa_fahd_delegation_ended_19960221'))
        # 2005: the last order in Fahd's name, signed on his behalf, then the held pledge after the accession.
        self.assertIn('نحن فهد بن عبدالعزيز آل سعود ملك المملكة العربية السعودية', text('sa_fahd_king_order_a193_20050731'))
        self.assertIn('عنه / عبدالله بن عبدالعزيز', text('sa_a193_signed_on_behalf_20050731'))
        self.assertIn('name match', doubt('sa_a193_signed_on_behalf_20050731'))
        self.assertIn('قصر الحكم بالرياض اليوم', text('sa_citizen_pledge_held_20050803'))
        self.assertIn('sa_citizen_pledge_scheduled_20050803', doubt('sa_citizen_pledge_held_20050803'))
        self.assertIn('بعد صلاة العشاء لهذا اليوم الجمعة 3 ربيع الآخر 1436 هـ', text('sa_citizen_pledge_held_20150123'))
        self.assertIn('مساء اليوم في قصر الصفا', text('sa_mbs_pledge_held_20170621'))
        self.assertIn('01:30 Makkah time on 22 June 2017', doubt('sa_mbs_pledge_held_20170621'))
        self.assertIn('3 و 4/8/1433هـ', text('sa_salman_cp_pledge_scheduled_20120618'))
        self.assertIn('does not attest that it was held', doubt('sa_salman_cp_pledge_scheduled_20120618'))
        # Deputations cite Article 66 and the day of travel; they are acting service only.
        for cid in ('sa_sultan_deputized_a175_20071029', 'sa_salman_deputized_a145_20140520', 'sa_mbn_deputized_a267_20150725',
                    'sa_mbn_deputized_a128_20170225'):
            self.assertIn('Article 66', text(cid))
            self.assertIn('acting service, a claim only', doubt(cid))
            self.assertIn('خلال فترة غيابنا عن المملكة', text(cid))
        # Orders give attested_on only; no new claim states a from or an until.
        self.assertIn('no effective day', doubt('sa_tuwaijri_sg_appointed_a136_20061020'))
        self.assertIn('never an end', doubt('sa_muqrin_cp_obs_20150428'))
        self.assertIn('never an end date', doubt('sa_fahd_king_order_a193_20050731'))
        for cid, (day, *_) in EVENTS.items():
            if day:
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
                self.assertIn('both identical', ex['stability_check'])
                self.assertIn(f"{RESPONSES[sid][0]} bytes, SHA-256 {RESPONSES[sid][1]}", ex['stability_check'])
                self.assertFalse(VOLATILE.search(src['url']), sid)
                for lead in LEADS:
                    self.assertNotIn(lead, src['url'])
                if sid in CAPTURES:
                    m = IA_URL.match(src['url'])
                    self.assertTrue(m and m.group(1) == CAPTURES[sid] and m.group(2) == src['original_url'])
                    self.assertLess(CAPTURES[sid], '20260907')
                    self.assertEqual(src['source_type'], 'primary_official_government_news_release')
                    self.assertTrue(ex['source_response_encoding'].startswith('identity'))
                else:
                    m = SPA_API_URL.match(src['url'])
                    self.assertTrue(m, sid)
                    self.assertEqual(src['original_url'], f'https://www.spa.gov.sa/{m.group(1)}')
                    self.assertEqual(ex['portal_metadata']['uuid'], m.group(1))
                    self.assertEqual(ex['portal_metadata']['locale'], 'ar')
                    self.assertRegex(ex['article_content_sha256'], r'^[0-9a-f]{64}$')
                    self.assertTrue(src['source_type'].startswith('primary_'))
                    # The publication date is the Makkah day of filing, except two printed date lines kept and declared.
                    stamp = datetime.fromisoformat(ex['portal_metadata']['published_at_utc'].replace('Z', '+00:00'))
                    makkah = (stamp + timedelta(hours=3)).date().isoformat()
                    self.assertEqual((src['published_date'], makkah), DATELINE_KEPT.get(sid, (makkah, makkah)))
                    if sid in DATELINE_KEPT:
                        self.assertIn('date line', src['scope_note'])

    def test_metadata_disagreement_is_recorded_without_inventing_a_cause(self):
        sid = 'sa_spa_salman_cp_pledge_directive_20120618'
        source = self.sources[sid]
        scope = source['scope_note']
        doubt = self.claims['sa_salman_cp_obs_20120618']['uncertainty']
        for text in (scope, doubt):
            self.assertIn('cause is unknown', text)
            self.assertNotIn('import artefact', text)
        self.assertIn('2012-06-19T00:06:51Z', scope)
        self.assertIn('18 June 2012', scope)
        self.assertIn('23:38', scope)
        self.assertEqual(source['published_date'], '2012-06-18')
        self.assertEqual(self.claims['sa_salman_cp_obs_20120618']['attested_on'], '2012-06-18')

    def test_report_and_handoff_are_ready_for_review_and_close_nothing(self):
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        self.assertIn('State: **ready_for_review**', handoff)
        self.assertIn('ready_for_review', self.report)
        for heading in ('Outcome', 'Sources added', 'Date ledger', 'Leads not imported', 'Sources attempted',
                        'Suggested next work orders', 'Integration notes', 'Checks'):
            self.assertRegex(self.report, rf'\n##+ {heading}( \([^)\n]*\))?\n')
        for sid in NEW_SOURCES:
            self.assertIn(f'`{sid}`', self.report)
        self.assertIn('State: **ready_for_review** (not complete)', self.report.split('\n## Outcome')[0])

    def test_packet_formatting_is_preserved(self):
        data = (research.ROOT / research.RESEARCH / 'saudi-arabia.json').read_bytes()
        self.assertNotIn(b'\r', data)
        self.assertEqual(data.decode('utf-8'), json.dumps(self.packet, indent=2, ensure_ascii=False) + '\n')

    def test_mutations_are_rejected(self):
        kcp_invariants(self.packet, self.extracts)

        def mutated(change):
            packet, extracts = copy.deepcopy(self.packet), copy.deepcopy(self.extracts)
            change(packet, extracts)
            return packet, extracts

        roles = lambda p: {r['id']: r for e in p['institutions'] for r in e['roles']}
        crown = lambda p: next(e for e in p['institutions'] if e['id'] == 'sa_crown')
        claim = lambda p, cid: next(c for s in p['sources'] for c in s['claims'] if c['id'] == cid)

        def muqrin_until_from_successor(p, e):
            next(h for h in roles(p)['sa_crown_prince']['holder_claims'] if isinstance(h, dict) and h['name'] == MUQ)['until'] = '2015-04-28'

        def pledge_as_observation(p, e):
            roles(p)['sa_king']['holder_claims'].append('sa_citizen_pledge_held_20150123')

        def deputation_on_role(p, e):
            crown(p)['claim_ids'].remove('sa_mbn_deputized_a267_20150725')
            roles(p)['sa_crown_prince']['claim_ids'].append('sa_mbn_deputized_a267_20150725')

        def secretary_from_order_day(p, e):
            roles(p)['sa_succession_secretary']['holder_claims'][0]['from'] = '2006-10-20'

        def existing_holder_rebased(p, e):
            next(h for h in roles(p)['sa_crown_prince']['holder_claims'] if isinstance(h, dict) and h['name'] == ABD)['attested_on'] = '1996-01-01'

        def observation_on_wrong_tenure(p, e):
            obs = roles(p)['sa_crown_prince']['holder_claims']
            i, j = obs.index('sa_muqrin_cp_obs_20150428'), obs.index('sa_mbn_cp_obs_a267_20150725')
            obs[i], obs[j] = obs[j], obs[i]

        def claim_text_drifts(p, e):
            claim(p, 'sa_nayef_cp_obs_20111107')['text'] += ' (edited)'

        def relative_day_misresolved(p, e):
            claim(p, 'sa_fahd_chairs_cabinet_19960212')['attested_on'] = '1996-02-13'

        def response_identity_changed(p, e):
            e['sa_spa_order_a136_20061020']['source_response_bytes'] += 1

        def observation_on_pm(p, e):
            roles(p)['sa_pm']['holder_claims'].append('sa_mbs_cp_obs_20260901')

        def source_extract_scope_drift(p, e):
            e['sa_spa_order_a136_20061020']['scope_note'] += ' Continuous tenure established.'

        def source_extract_locator_drift(p, e):
            e['sa_spa_order_a136_20061020']['rows'][0]['locator'] = 'Unrelated order'

        for change in (muqrin_until_from_successor, pledge_as_observation, deputation_on_role, secretary_from_order_day,
                       existing_holder_rebased, observation_on_wrong_tenure, claim_text_drifts, relative_day_misresolved,
                       response_identity_changed, observation_on_pm, source_extract_scope_drift,
                       source_extract_locator_drift):
            with self.subTest(change=change.__name__):
                with self.assertRaises((AssertionError, KeyError, StopIteration)):
                    kcp_invariants(*mutated(change))


if __name__ == '__main__':
    unittest.main()
