"""CLAUDE-C01-51: the RSFSR President and Vice-President, 1991-1993, keep signed acts and official records naming the office
apart from suspension, declared termination, declared or disputed acting service and procedure, append holder observations
after the unchanged CLAUDE-C01-05 holders, add no ru_president holder, and state an end only where an act releases the
Vice-President and states that it takes effect on its signature."""
import copy
import hashlib
import json
import re
import unittest

import campaign_research as research


# Original response identity recorded in each new extract: (bytes, sha256) of the portal's original-edition document frame
# and of its date-restricted card, as served over HTTP (curl, no --compressed), downloaded twice at least 30 minutes apart.
RESPONSES = {
    'ru_rsfsr_ukaz_8_19910719': (23766, 'f5e3acab586f6fd2e3248ae7974ecebe6e82a7d269f6ad0dec44cb34080da047'),
    'ru_rsfsr_vp_rasp_1rv_19910729': (6964, 'd0c0f32fdf8bd0aeb71451cbefd0ed84767ec83d51997c1591ba2bb7fb8e6656'),
    'ru_rsfsr_rasp_98rp_19911119': (24378, 'edb848b7f4335d06f141304b42eedd540beca69ac6b2a25ac9a90635e6252746'),
    'ru_rsfsr_vp_rasp_8rv_19911205': (6940, '6e73e35058156e358efdec9528e4087caa571d394416530ad811906e0caef032'),
    'ru_ukaz_316_19911226': (23808, '07507eb83977434535145ef9f1a7f5aefd7874663a9c18823e5177f9d944b1cb'),
    'ru_rsfsr_ukaz_318_19911226': (36815, '7b73e65bb8d63b1de773431e548cc5e209b32610bc140cc9f545257d475a7467'),
    'ru_rsfsr_ukaz_245n_19920103': (24066, '57f8bdc42ecb0cc27198cb46c9c8c28a1dd435c8e70d8ddd53a04b4606591d00'),
    'ru_vp_rasp_1rv_19920116': (7014, '07b74b9cb96ae1c15df68b0f6a0c7ae1d654a66353605ff3f9d9ed571b8bab3f'),
    'ru_vs_res_4825i_19930416': (10197, '0fd28136d89f8d59a01d61e074b633646fc746082c96ea02b850b68aecb04c04'),
    'ru_ukaz_1328_19930901': (24813, '0349671ebb1b706d06e25a5c2dbf6b28253b7438824af4edb9b709b57feb9030'),
    'ru_ukaz_1398_19930918': (24601, '9449113ef2bcbfe3d92c608b7b076ddf7ee5b0de8df89bfe546afe40b6c8a61a'),
    'ru_vs_presidium_res_5779i_19930921': (8650, '220cd093eedd054663db5f367cd50efc8535502a2fbcb20ac7d18ef6930d9e2c'),
    'ru_vs_res_5780i_19930922': (7886, '371375cc83f2496989d2edfd229fbd91bfe8521b89b12fec5e94937e0c0c651b'),
    'ru_vs_res_5781i_19930922': (7629, 'c1fc0e6c9e1dc91c2edb6cc358ac31df536570e55a7af93e414789a85ff68f6c'),
    'ru_ukaz_1410_19930922': (25150, 'a155b8145211ef5a32ecb9f3cdaf697d924bf78e1dab92692d01caa61ddb0cf5'),
    'ru_ukaz_1576_19931003': (27293, 'b7e2f7813bbb370e7482fe16f9d6c11f87697f6ad46ebbb1d6920015e7332e18'),
}
CARDS = {
    'ru_rsfsr_ukaz_8_19910719': (3062, '5c3a316d61047a3f1f6d00871a8f85e20159ce5089d80f6c1e9403c7380bc851'),
    'ru_rsfsr_vp_rasp_1rv_19910729': (2897, 'ed936312556f5c3897f0b55261d8a069c1b137f98af3f3781cbeb678b1b66df2'),
    'ru_rsfsr_rasp_98rp_19911119': (3051, '56d962e559c3443f989e3a284b9d670022489b6d487d73622a309f20bb3d4a91'),
    'ru_rsfsr_vp_rasp_8rv_19911205': (2896, '60c76c34acfe7074e460e952422c6bc50241068f9ae10f6c484add4b5f628c0e'),
    'ru_ukaz_316_19911226': (3083, 'f7dd214d892a1752d46c555b71f73afffc939e0a706a8748cb638dc26a10cc94'),
    'ru_rsfsr_ukaz_318_19911226': (3127, '453a9993cfefce4c4302a2f4b0951166aa30d85094037723667897809e3926ad'),
    'ru_rsfsr_ukaz_245n_19920103': (3072, '90c450cfe48a929a08c82c480d0d17bdc9e36bc5466a33e77b8e2e746f083011'),
    'ru_vp_rasp_1rv_19920116': (2926, '2f964e3cd2f0dec8967b1355194b34cbd44f31c5daf3c110ebd1fec85c4c902a'),
    'ru_vs_res_4825i_19930416': (3362, '0bc66a0a22939dca41e2152a86188ffd5c84d4c5427eaf8f59c1b96944b21aa8'),
    'ru_ukaz_1328_19930901': (3102, 'e1cd83157926f71d1a82261f4bbee5acb9b354940404e27da763480f10de4e65'),
    'ru_ukaz_1398_19930918': (3133, '4ee6f498c56df65d202a0ee64569e5f4930cde425590b5edf22602cea6cc6a76'),
    'ru_vs_presidium_res_5779i_19930921': (3073, '602873b91986e2554a2d94902d0e78ab7f5bce30227c8f843a664f343973a9cc'),
    'ru_vs_res_5780i_19930922': (2953, '379fc0c7a871c5811976e5fc3451be5c88eda6d23792befb33146cfe6a0c4b0d'),
    'ru_vs_res_5781i_19930922': (3087, 'b0a45abcef91b38a7a5c3cd36dd0572ade55c47d4b0e4e481a9a79ba87553846'),
    'ru_ukaz_1410_19930922': (3093, '0d62bc3209226c0059f9881deda0f24bf89e72adfbf69074b590beaac3543dc0'),
    'ru_ukaz_1576_19931003': (3174, 'd3b03643b94b14e7ffbac6f235de248f8efbab39554ef7100481255c3a6c038c'),
}
NEW_SOURCES = list(RESPONSES)
ND = {
    'ru_rsfsr_ukaz_8_19910719': '102012111',
    'ru_rsfsr_vp_rasp_1rv_19910729': '102012167',
    'ru_rsfsr_rasp_98rp_19911119': '102013152',
    'ru_rsfsr_vp_rasp_8rv_19911205': '102013436',
    'ru_ukaz_316_19911226': '102013793',
    'ru_rsfsr_ukaz_318_19911226': '102013783',
    'ru_rsfsr_ukaz_245n_19920103': '102013951',
    'ru_vp_rasp_1rv_19920116': '102014149',
    'ru_vs_res_4825i_19930416': '102022817',
    'ru_ukaz_1328_19930901': '102025933',
    'ru_ukaz_1398_19930918': '102026121',
    'ru_vs_presidium_res_5779i_19930921': '102026142',
    'ru_vs_res_5780i_19930922': '102026171',
    'ru_vs_res_5781i_19930922': '102026182',
    'ru_ukaz_1410_19930922': '102026198',
    'ru_ukaz_1576_19931003': '102026474',
}

P, VP, RF = 'ru_rsfsr_president', 'ru_rsfsr_vice_president', 'ru_president'
YEL, RUT = 'Борис Николаевич Ельцин', 'Александр Владимирович Руцкой'
TITLES = {P: 'President of the RSFSR (Президент РСФСР)', VP: 'Vice-President of the RSFSR (вице-президент РСФСР)',
          RF: 'Президент Российской Федерации — President of the Russian Federation'}

# Every claim of this packet: (attested_on, event kind, role, review observation).
EVENTS = {
    'ru_rsfsr_ukaz_8_signed_as_president_rsfsr_19910719': ('1991-07-19', 'signed_act_attestation', P, 'RU-RSP-01'),
    'ru_rsfsr_vp_rasp_1rv_signed_as_vice_president_19910729': ('1991-07-29', 'signed_act_attestation', VP, 'RU-RSP-01'),
    'ru_rsfsr_rasp_98rp_signed_as_president_rsfsr_19911119': ('1991-11-19', 'signed_act_attestation', P, 'RU-RSP-01'),
    'ru_rsfsr_rasp_98rp_vice_president_duties_19911119': ('1991-11-19', 'duties_assigned', VP, 'RU-RSP-01'),
    'ru_rsfsr_vp_rasp_8rv_signed_as_vice_president_19911205': ('1991-12-05', 'signed_act_attestation', VP, 'RU-RSP-02'),
    'ru_ukaz_316_signed_as_president_rf_19911226': ('1991-12-26', 'restyled_signature', P, 'RU-RSP-02'),
    'ru_rsfsr_ukaz_318_signed_as_president_rsfsr_19911226': ('1991-12-26', 'signed_act_attestation', P, 'RU-RSP-02'),
    'ru_rsfsr_ukaz_245n_signed_as_president_rsfsr_19920103': ('1992-01-03', 'signed_act_attestation', P, 'RU-RSP-02'),
    'ru_vp_rasp_1rv_signed_as_vice_president_rf_19920116': ('1992-01-16', 'signed_act_attestation', VP, 'RU-RSP-02'),
    'ru_vs_4825i_rutskoi_styled_vice_president_19930416': ('1993-04-16', 'official_record_attestation', VP, 'RU-RSP-03'),
    'ru_ukaz_1328_rutskoi_styled_vice_president_19930901': ('1993-09-01', 'official_record_attestation', VP, 'RU-RSP-04'),
    'ru_ukaz_1328_rutskoi_suspended_from_duties_19930901': ('1993-09-01', 'suspension_from_duties', VP, 'RU-RSP-04'),
    'ru_ukaz_1398_vice_president_powers_by_decree_only_19930918': ('1993-09-18', 'procedure_rule', VP, 'RU-RSP-04'),
    'ru_vs_presidium_5779i_yeltsin_powers_deemed_terminated_19930921': ('1993-09-21', 'termination_declared', RF, 'RU-RSP-05'),
    'ru_vs_presidium_5779i_rutskoi_began_exercising_powers_19930921': ('1993-09-21', 'acting_service_declared', RF, 'RU-RSP-05'),
    'ru_vs_5780i_yeltsin_powers_declared_terminated_19930922': ('1993-09-22', 'termination_declared', RF, 'RU-RSP-05'),
    'ru_vs_5781i_rutskoi_exercises_presidential_powers_19930922': ('1993-09-22', 'acting_service_declared', RF, 'RU-RSP-05'),
    'ru_ukaz_1410_rutskoi_assumption_declared_unlawful_19930922': ('1993-09-22', 'acting_service_disputed', RF, 'RU-RSP-05'),
    'ru_ukaz_1576_rutskoi_released_as_vice_president_19931003': ('1993-10-03', 'removal', VP, 'RU-RSP-06'),
    'ru_ukaz_1576_presidential_succession_rule_19931003': ('1993-10-03', 'procedure_rule', VP, 'RU-RSP-06'),
}
HOLDER_KINDS = {'signed_act_attestation', 'official_record_attestation', 'removal'}
CLAIM_ONLY_KINDS = {'duties_assigned', 'restyled_signature', 'suspension_from_duties', 'procedure_rule', 'termination_declared',
                    'acting_service_declared', 'acting_service_disputed'}
NEVER_HOLDER = tuple(c for c, v in EVENTS.items() if v[1] in CLAIM_ONLY_KINDS)
REMOVAL = 'ru_ukaz_1576_rutskoi_released_as_vice_president_19931003'
SUSPENSION = 'ru_ukaz_1328_rutskoi_suspended_from_duties_19930901'
ACTING = ('ru_vs_presidium_5779i_rutskoi_began_exercising_powers_19930921',
          'ru_vs_5781i_rutskoi_exercises_presidential_powers_19930922',
          'ru_ukaz_1410_rutskoi_assumption_declared_unlawful_19930922')
TERMINATION = ('ru_vs_presidium_5779i_yeltsin_powers_deemed_terminated_19930921',
               'ru_vs_5780i_yeltsin_powers_declared_terminated_19930922')
# Days of claim-only events that no observation may carry (the acting service, the declared terminations, the procedure rule).
NEVER_HOLDER_DATE = {'1993-09-18', '1993-09-21', '1993-09-22'}


def att(role_key, day):
    return next(c for c, v in EVENTS.items() if v[2] == role_key and v[0] == day and v[1] in ('signed_act_attestation',
                                                                                        'official_record_attestation'))


# The CLAUDE-C01-05 holder of each RSFSR role (unchanged), then this packet's observations in date order.
ORIGINAL = {
    P: (YEL, None, '1991-07-10', None,
        ['ru_rsfsr_law_1494i_19910627', 'ru_prlib_inauguration_stenogram_19910710', 'ru_rsfsr_res_1595i_19910710'],
        ['ru_rsfsr_inauguration_law_rules_19910627', 'ru_steno_oath_19910710', 'ru_res_1595i_yeltsin_release_19910710']),
    VP: (RUT, None, '1991-07-10', None, ['ru_rsfsr_law_1494i_19910627', 'ru_rsfsr_res_1596i_19910710'],
         ['ru_rsfsr_inauguration_law_rules_19910627', 'ru_res_1596i_rutskoi_release_19910710']),
}
HOLDERS = {
    P: [(YEL, d, None, None, [att(P, d)]) for d in ('1991-07-19', '1991-11-19', '1991-12-26', '1992-01-03')],
    VP: [(RUT, d, None, None, [att(VP, d)]) for d in ('1991-07-29', '1991-12-05', '1992-01-16', '1993-04-16')]
        + [(RUT, '1993-09-01', None, '1993-10-03', [att(VP, '1993-09-01'), REMOVAL])],
}
# Role lists before this packet: (claims, sources); this packet appends after them and never edits them.
ORIGINAL_ROLE_LISTS = {P: (13, 6), VP: (5, 5), RF: (94, 53)}
REPORT = research.RESEARCH / 'russia-rsfsr-president-vice-president-1991-1993-51.md'
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-51.md'
# Holders on the other Russia roles, which this packet never touches: (role, number of holder observations).
OTHER_ROLE_HOLDERS = {RF: 7, 'ru_government_chairman': 15, 'ru_duma_faction_20211012_er_head': 5,
                      'ru_duma_faction_20211012_kprf_head': 5, 'ru_duma_faction_20211012_srzp_head': 4,
                      'ru_duma_faction_20211012_ldpr_head': 4, 'ru_duma_faction_20211012_nl_head': 5}


def ours(rid):
    return [c for c, v in EVENTS.items() if v[2] == rid]


def invariants(russia):
    """Packet-level rules this test owns; raises AssertionError, KeyError, IndexError or StopIteration on any violation."""
    claims = {c['id']: c for s in russia['sources'] for c in s['claims']}
    owner = {c['id']: s['id'] for s in russia['sources'] for c in s['claims']}
    new = set(EVENTS)
    inst = next(e for e in russia['institutions'] if e['id'] == 'ru_rsfsr_presidency')
    assert [r['id'] for r in inst['roles']] == [P, VP, RF]
    # The institution's own identity, lifecycle and lists are unchanged.
    assert (inst['lifecycle']['status'], inst['lifecycle']['from'], inst['lifecycle']['until']) == (
        'creation_adopted_entry_into_force_day_unverified', None, None)
    assert len(inst['sources']) == 8 and len(inst['claim_ids']) == 10
    assert not set(inst['claim_ids']) & new and not set(inst['sources']) & set(NEW_SOURCES)
    assert (inst['represented_party_ids'], inst['constituent_organization_ids'], inst['reconciled_organization_id']) == ([], [], None)
    roles = {r['id']: r for r in inst['roles']}
    for rid, (n_claims, n_sources) in ORIGINAL_ROLE_LISTS.items():
        role = roles[rid]
        assert role['title'] == TITLES[rid]
        assert role['kind'] == ('head_of_state' if rid == RF else 'institutional_office')
        assert role['claim_ids'][n_claims:] == ours(rid), rid
        assert not set(role['claim_ids'][:n_claims]) & new, rid
        expected = []
        for cid in ours(rid):
            if owner[cid] not in role['sources'][:n_sources] and owner[cid] not in expected:
                expected.append(owner[cid])
        assert role['sources'][n_sources:] == expected, rid
        assert not set(role['sources'][:n_sources]) & set(NEW_SOURCES), rid
    for rid in (P, VP):
        holders = roles[rid]['holder_claims']
        first = holders[0]
        assert (first['name'], first['attested_on'], first['from'], first['until'], first['sources'], first['claim_ids']) == ORIGINAL[rid]
        got = [(h['name'], h['attested_on'], h['from'], h['until'], h['claim_ids']) for h in holders[1:]]
        assert got == HOLDERS[rid], got
        for h in holders[1:]:
            assert h['from'] is None, h['name']
            assert all(EVENTS[c][1] in HOLDER_KINDS and EVENTS[c][2] == rid for c in h['claim_ids']), h['name']
            assert not set(h['claim_ids']) & set(NEVER_HOLDER), h['name']
            assert not {h['attested_on'], h['until']} & NEVER_HOLDER_DATE, h['name']
            assert EVENTS[h['claim_ids'][0]][1] != 'removal'
            assert claims[h['claim_ids'][0]]['attested_on'] == h['attested_on'], h['name']
            expected = []
            for cid in h['claim_ids']:
                if owner[cid] not in expected:
                    expected.append(owner[cid])
            assert h['sources'] == expected, h['name']
            # An end only where the last claim is decree 1576's release, on its day, after the observation.
            if h['until'] is not None:
                assert h['claim_ids'][-1] == REMOVAL and h['until'] == claims[REMOVAL]['attested_on'] > h['attested_on']
                assert 'с момента его подписания' in claims[REMOVAL]['text']
            else:
                assert REMOVAL not in h['claim_ids']
    # ru_president gains five claims-only events of September 1993 and no holder; its seven CLAUDE-C01-14 holders cite nothing of this packet.
    assert len(roles[RF]['holder_claims']) == 7
    assert roles[RF]['holder_claims'][0]['from'] == '1996-08-09'
    for h in roles[RF]['holder_claims']:
        assert not set(h['claim_ids']) & new and not set(h['sources']) & set(NEW_SOURCES)
    # No other role, in any group, cites a claim or source of this packet or changes its number of holders.
    for group in ('organizations', 'institutions'):
        for entry in russia[group]:
            for r in entry['roles']:
                if r['id'] in (P, VP, RF):
                    continue
                cited = set(r['claim_ids']) | {c for h in r['holder_claims'] if isinstance(h, dict) for c in h['claim_ids']}
                assert not cited & new, r['id']
                assert not set(r['sources']) & set(NEW_SOURCES), r['id']
                if r['id'] in OTHER_ROLE_HOLDERS:
                    assert len(r['holder_claims']) == OTHER_ROLE_HOLDERS[r['id']], r['id']
            if entry['id'] != 'ru_rsfsr_presidency':
                assert not set(entry['claim_ids']) & new and not set(entry['sources']) & set(NEW_SOURCES), entry['id']
    # The acting service and the declared terminations never reach a holder on any role.
    for rid in (P, VP, RF):
        for h in roles[rid]['holder_claims']:
            assert not set(h['claim_ids']) & set(ACTING + TERMINATION + (SUSPENSION,)), rid


class RussianRsfsrPresidencyTests(unittest.TestCase):
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
        self.assertEqual((len(ids['sources']), len(ids['claims']), len(ids['entries']), len(ids['roles'])), (193, 342, 21, 9))
        self.assertEqual([s['id'] for s in self.packet['sources'][177:]], NEW_SOURCES)
        new_claims = [c['id'] for sid in NEW_SOURCES for c in self.sources[sid]['claims']]
        self.assertEqual(new_claims, list(EVENTS))
        dates = [self.sources[sid]['document_date'] for sid in NEW_SOURCES]
        self.assertEqual(dates, sorted(dates))
        self.assertEqual({v[1] for v in EVENTS.values()}, HOLDER_KINDS | CLAIM_ONLY_KINDS)
        self.assertFalse(HOLDER_KINDS & CLAIM_ONLY_KINDS)
        self.assertEqual({v[3] for v in EVENTS.values()}, {f'RU-RSP-{n:02d}' for n in range(1, 7)})
        self.assertEqual(re.findall(r'^### (RU-RSP-\d\d)\b', self.report, re.M), [f'RU-RSP-{n:02d}' for n in range(1, 7)])
        cited = {c for rid in (P, VP) for h in HOLDERS[rid] for c in h[4]}
        self.assertEqual(cited, {c for c, v in EVENTS.items() if v[1] in HOLDER_KINDS})
        self.assertEqual(sum(len(v) for v in HOLDERS.values()), 9)
        for sid in NEW_SOURCES:
            source = self.sources[sid]
            self.assertEqual((source['accessed_date'], source['access_method']),
                             ('2026-10-01', 'official_portal_original_edition_text_downloaded_over_http'))
            self.assertTrue(source['source_type'].startswith('primary_') and source['source_type'].endswith('_official_portal_text'))
            self.assertIsNone(source['published_date'])
            self.assertEqual(source['url'], f'https://pravo.gov.ru/proxy/ips/?doc_itself=&nd={ND[sid]}&page=1&rdk=0')
            self.assertTrue(sid.endswith(source['document_date'].replace('-', '')), sid)
            for claim in source['claims']:
                self.assertEqual(claim['attested_on'], source['document_date'], claim['id'])
                self.assertTrue(claim['uncertainty'] and claim['locator'], claim['id'])
                self.assertTrue(claim['id'].endswith(claim['attested_on'].replace('-', '')), claim['id'])

    def test_holders_are_exactly_as_intended(self):
        invariants(self.packet)
        inst = next(e for e in self.packet['institutions'] if e['id'] == 'ru_rsfsr_presidency')
        for role in inst['roles'][:2]:
            for holder in role['holder_claims'][1:]:
                self.assertTrue(holder['note'].startswith(f"Observed on {holder['attested_on']}: "), holder['attested_on'])
                self.assertTrue(holder['uncertainty'])
                for cid in holder['claim_ids']:
                    row = self.rows[cid]
                    self.assertEqual((row['holder_name'], row['role_id'], row['role_title']),
                                     (holder['name'], role['id'], TITLES[role['id']]), cid)
                    self.assertIn(row['event_kind'], HOLDER_KINDS, cid)
        last = inst['roles'][1]['holder_claims'][-1]
        self.assertIn('fallback under the strict signature rule (review 1739eccb): until null', last['uncertainty'])
        self.assertIn("'с момента его подписания'", last['note'])
        # The signatures that date the President observations print the RSFSR title; decree 316 does not.
        for d in ('1991-07-19', '1991-11-19', '1991-12-26', '1992-01-03'):
            self.assertRegex(self.claims[att(P, d)]['text'], r"'Президент РСФСР Б\. ?(Ельцин|ЕЛЬЦИН)")
        self.assertIn("'Президент Российской Федерации Б.Ельцин", self.claims['ru_ukaz_316_signed_as_president_rf_19911226']['text'])
        for d in ('1991-07-29', '1991-12-05'):
            self.assertIn("'Вице-президент РСФСР А. Руцкой", self.claims[att(VP, d)]['text'])
        self.assertIn("'Вице-президент Российской Федерации А. Руцкой", self.claims[att(VP, '1992-01-16')]['text'])

    def test_claim_only_events_never_date_a_holder(self):
        for cid in ACTING:
            self.assertIn('claims only', self.claims[cid]['uncertainty'], cid)
        self.assertIn('never a holder observation', self.claims[ACTING[0]]['uncertainty'])
        self.assertIn("'acting President' resolution", self.claims[ACTING[1]]['uncertainty'])
        self.assertIn('с 20 часов 00 минут 21 сентября 1993 года исполняет вице-президент', self.claims[ACTING[1]]['text'])
        for cid in TERMINATION:
            self.assertIn('never an end on any role', self.claims[cid]['uncertainty'], cid)
        self.assertIn('not a removal or an end', self.claims[SUSPENSION]['uncertainty'])
        self.assertIn('Временно отстранить', self.claims[SUSPENSION]['text'])
        self.assertIn('Освободить Руцкого А.В. от должности вице-президента', self.claims[REMOVAL]['text'])
        for cid in NEVER_HOLDER:
            self.assertIsNotNone(self.rows[cid]['event_kind'])

    def test_extracts_match_the_packet_and_their_snapshots(self):
        for sid in NEW_SOURCES:
            source, extract = self.sources[sid], self.extracts[sid]
            path = research.ROOT / source['snapshot']['path']
            data = path.read_bytes()
            self.assertEqual((len(data), hashlib.sha256(data).hexdigest()), (source['snapshot']['bytes'], source['snapshot']['sha256']))
            self.assertEqual(source['snapshot']['kind'], 'derived_factual_extract')
            self.assertTrue(path.name.startswith('russia-ips-') and path.name.endswith('-facts.json'), path.name)
            self.assertTrue(data.endswith(b'\n') and b'\r' not in data, sid)
            self.assertEqual(data.decode('utf-8'), json.dumps(extract, ensure_ascii=False, indent=2) + '\n')
            self.assertEqual(extract['format'], 'spheres-c01-derived-factual-table/v1')
            self.assertEqual((extract['source_id'], extract['source_title'], extract['accessed_date']),
                             (sid, source['title'], source['accessed_date']))
            self.assertEqual(extract['source_url'], source['url'])
            self.assertEqual((extract['published_date'], extract['document_date']), (source['published_date'], source['document_date']))
            self.assertEqual(extract['scope_note'], source['scope_note'])
            self.assertFalse(extract['source_response_checked_in'])
            self.assertIn('not checked into this repository', extract['provenance_note'])
            self.assertIn("curl's default User-Agent", extract['provenance_note'])
            self.assertEqual([r['claim_id'] for r in extract['rows']], [c['id'] for c in source['claims']])
            for row, claim in zip(extract['rows'], source['claims']):
                day, kind, rid, obs = EVENTS[row['claim_id']]
                self.assertEqual((row['text'], row['locator'], row['attested_on']), (claim['text'], claim['locator'], claim['attested_on']))
                self.assertEqual((row['attested_on'], row['event_kind'], row['role_id'], row['review_observation']), (day, kind, rid, obs))
                self.assertEqual((row['observation_id'], row['role_title']), ('ru_rsfsr_presidency', TITLES[rid]))
                self.assertIn(row['holder_name'], {YEL, RUT, None}, row['claim_id'])
                self.assertEqual('printed_name' in row, row['holder_name'] is not None, row['claim_id'])
                if kind in HOLDER_KINDS:
                    self.assertEqual(row['holder_name'], YEL if rid == P else RUT, row['claim_id'])

    def test_response_identities_are_pinned(self):
        for sid, (size, digest) in RESPONSES.items():
            source, extract = self.sources[sid], self.extracts[sid]
            url = f'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd={ND[sid]}&page=1&rdk=0'
            self.assertEqual((extract['source_url'], extract['source_response_url']), (source['url'], url))
            self.assertEqual(source['url'], url.replace('http://', 'https://', 1))
            self.assertEqual((extract['source_response_bytes'], extract['source_response_sha256']), (size, digest))
            card = extract['portal_card_response']
            self.assertEqual((card['bytes'], card['sha256']), CARDS[sid])
            self.assertTrue(card['url'].startswith('http://pravo.gov.ru/proxy/ips/?list_itself=&bpas=cd00000&a8='), sid)
            day = source['document_date']
            self.assertIn(f'&a7type=4&a7from={day[8:]}.{day[5:7]}.{day[:4]}&a7to={day[8:]}.{day[5:7]}.{day[:4]}&', card['url'])
            self.assertIn('one hit', card['note'])
            self.assertIn('served uncompressed', extract['source_response_encoding'])
            self.assertNotIn('source_response_content_encoding', extract)
            self.assertRegex(extract['stability_check'],
                             r'^Document frame: downloaded at 2026-10-01T\d\d:\d\d:\d\dZ and again at 2026-10-01T\d\d:\d\d:\d\dZ '
                             r'\((3\d|[4-9]\d|\d{3,}) minutes later\); identical ')

    def test_report_and_handoff_are_ready_for_review_and_close_nothing(self):
        for heading in ('Outcome', 'Observations', 'Sources added', 'Response identities and stability checks', 'Date ledger',
                        'Leads not imported', 'Sources attempted', 'Suggested next work orders', 'Decisions for Codex',
                        'Integration notes (outside this packet', 'Checks'):
            self.section(heading)
        self.assertIn('ready_for_review', self.report)
        for sid in NEW_SOURCES:
            self.assertIn(RESPONSES[sid][1][:12], self.section('Response identities and stability checks'), sid)
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        for text in ('ready_for_review', 'russia-rsfsr-president-vice-president-1991-1993-51.md', 'claude/c01-ru-51', '4b15a1d7',
                     'test_russia_rsfsr_presidency_c01_51.py', 'Decisions for Codex'):
            self.assertIn(text, handoff)
        index = research.build()
        country = next(p for p in index['countries'] if p['nation'] == 'Russia')
        self.assertFalse(country['country_census_complete'])
        self.assertEqual((country['role_observations'], country['source_claims'], country['mapping_pending']), (9, 342, 21))
        self.assertFalse(index['c01_complete'])

    def test_mutations_are_rejected(self):
        self.check(copy.deepcopy(self.packet))

        def role(packet, rid):
            return next(r for e in packet['institutions'] for r in e['roles'] if r['id'] == rid)

        def holder(packet, rid, day):
            return next(h for h in role(packet, rid)['holder_claims'][1:] if h['attested_on'] == day)

        def setter(rid, day, key, value):
            def mutate(p):
                holder(p, rid, day)[key] = value
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
            h = holder(p, VP, '1993-04-16')
            role(p, VP)['holder_claims'].remove(h)
            role(p, P)['holder_claims'].append(h)

        def merge_into_rf(p):
            h = copy.deepcopy(holder(p, P, '1992-01-03'))
            role(p, RF)['holder_claims'].insert(0, h)

        def cross_claim(p):
            gov = role(p, 'ru_government_chairman')
            gov['claim_ids'].append('ru_rsfsr_ukaz_318_signed_as_president_rsfsr_19911226')
            gov['sources'].append('ru_rsfsr_ukaz_318_19911226')

        def end_original(p):
            role(p, VP)['holder_claims'][0]['until'] = '1993-10-03'

        def drop_original(p):
            role(p, P)['holder_claims'].pop(0)

        def drop_end(p):
            holder(p, VP, '1993-09-01')['until'] = None

        def cite_acting(p):
            h = holder(p, VP, '1993-04-16')
            h['claim_ids'].append(ACTING[1])
            h['sources'].append('ru_vs_res_5781i_19930922')

        def end_from_suspension(p):
            h = holder(p, VP, '1993-04-16')
            h['until'] = '1993-09-01'
            h['claim_ids'].append(SUSPENSION)
            h['sources'].append('ru_ukaz_1328_19930901')

        def termination_ends_yeltsin(p):
            h = holder(p, P, '1992-01-03')
            h['until'] = '1993-09-22'
            h['claim_ids'].append(TERMINATION[1])
            h['sources'].append('ru_vs_res_5780i_19930922')

        def beyond_cutoff(p):
            next(c for s in p['sources'] for c in s['claims'] if c['id'] == REMOVAL)['attested_on'] = '2026-09-08'

        def party_mapping(p):
            next(e for e in p['institutions'] if e['id'] == 'ru_rsfsr_presidency')['represented_party_ids'] = ['Russia/kprf']

        def snapshot(p):
            next(s for s in p['sources'] if s['id'] == 'ru_ukaz_1576_19931003')['snapshot']['sha256'] = '0' * 64

        def lifecycle(p):
            next(e for e in p['institutions'] if e['id'] == 'ru_rsfsr_presidency')['lifecycle']['until'] = '1993-10-03'

        def entry_citation(p):
            e = next(e for e in p['institutions'] if e['id'] == 'ru_rsfsr_presidency')
            e['claim_ids'].append(REMOVAL)
            e['sources'].append('ru_ukaz_1576_19931003')

        def uncited_claim(p):
            role(p, RF)['claim_ids'].remove(TERMINATION[0])

        def edit_original_claim_list(p):
            role(p, P)['claim_ids'].pop(0)

        mutations = {
            'election result as start': setter(P, '1991-07-19', 'from', '1991-06-19'),
            'signature date as start': setter(VP, '1991-07-29', 'from', '1991-07-29'),
            'latest attestation as end': setter(P, '1992-01-03', 'until', '1992-01-03'),
            'renaming as Ельцин end': setter(P, '1991-12-26', 'until', '1991-12-25'),
            'acting day as attested day': setter(VP, '1993-04-16', 'attested_on', '1993-09-22'),
            'acting service as holder': add_holder(VP, RUT, '1993-09-22', [ACTING[1]]),
            'acting service as President holder': add_holder(P, RUT, '1993-09-21', [ACTING[0]]),
            'termination as holder': add_holder(RF, YEL, '1993-09-22', [TERMINATION[1]]),
            'restyled signature as holder': add_holder(P, YEL, '1991-12-26', ['ru_ukaz_316_signed_as_president_rf_19911226']),
            'acting claim cited by holder': cite_acting,
            'suspension as end': end_from_suspension,
            'declared termination as end': termination_ends_yeltsin,
            'end on the unchanged original holder': end_original,
            'original holder removed': drop_original,
            'release end removed': drop_end,
            'cross-role holder': move,
            'holder merged into ru_president': merge_into_rf,
            'cross-role claim on the Government': cross_claim,
            'institution cites a new claim': entry_citation,
            'printed initials as name': setter(VP, '1991-12-05', 'name', 'А. Руцкой'),
            'claim beyond cutoff': beyond_cutoff,
            'party mapping': party_mapping,
            'institution lifecycle end': lifecycle,
            'snapshot checksum mismatch': snapshot,
            'claim left uncited by its role': uncited_claim,
            'original role claim list edited': edit_original_claim_list,
        }
        for label, mutate in mutations.items():
            packet = copy.deepcopy(self.packet)
            mutate(packet)
            with self.assertRaises((AssertionError, ValueError, KeyError, IndexError, StopIteration), msg=label):
                self.check(packet)


if __name__ == '__main__':
    unittest.main()
