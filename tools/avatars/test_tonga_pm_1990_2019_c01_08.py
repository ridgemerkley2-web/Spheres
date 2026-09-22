"""CLAUDE-C01-08: Tonga's 1990-2019 prime ministers keep selection, appointment, effect, acting service, resignation,
death and oath apart, and state an end only where a source does."""
import copy
import hashlib
import json
import re
import unittest
from urllib.parse import urlsplit

import campaign_research as research


# Original response identity recorded in each extract: (bytes, sha256); None where it is not reproducible.
RESPONSES = {
    'to_pmo_former_pms_2026': (75947, 'bb1231c3051919bf91495476a9a4820cc020a2fc0ea750702bd5ff8a029c2442'),
    'to_pmo_former_pms_2015': (53745, '1f2f361acbe02029aa45a4dedee6bc0d4571358d17fd7e3e2ce21441b5c5d9f7'),
    'to_mic_pm_list_2011': (30099, '91c22bf205096ec2e7ddf24d2369ed707036e71d933e9616e7bb82307064989a'),
    'to_ipu_1990': (3669, None),
    'to_ipu_1993': (4401, None),
    'to_pmo_vaea_tribute_20090613': (28846, '3c5f1b3be1b36afbd5e3eb471b390b4d3ad77e957c4bab594ecc21d40cb92ad1'),
    'to_pmo_office_pm_2002': (7015, '05bd72340c40011fe15e3ae9fe30050ac27278b1119d692bdaa98f7bb18bbda8'),
    'to_pmo_timeline_2000_en': (19908, '16f6e16d1c7c73538a0aec6570f2df4a1ff240981874551682d3a025ce9ea6f0'),
    'to_pmo_timeline_2000_to': (28854, '6155930ad11be3c998a8f16495c9b7d4aa80eb13ebf162e7b9e1f11fbe118e02'),
    'to_pmo_20060213_resignation': (11375, 'bb2d758aa139cc7c81d96c3a63a97765d2c2bf4dd046347ac190b58b69788877'),
    'to_pmo_20060213_resignation_to': (12521, 'f58d6cd40419f47402f86f1b34e1ce4f32b9660318138afb1d24cba759125614'),
    'to_pmo_20060330_farewell_speech': (24857, 'a0267a662e69eff6ee77244d29419e2cf6673ed2c211d20d75dc64b5541679a7'),
    'to_pmo_20060410_marist_remarks': (18700, 'b1c0dce7a8a6ef90e1960f1a27f1d879ef2dc325c808b8dd4da10d9ac92c01f1'),
    'to_pmo_20060413_psc': (12476, '50504b1b78df092d5e7c7aac097acd85006032303a04ea9c52982bcbce522eac'),
    'to_pmo_timeline_2006_en': (28570, '96a9b335fef7bc3924dda71d00975d504ebab7e78940554ab96727c6be24dd63'),
    'to_pmo_timeline_2006_to': (28746, 'eb86f3342b27cc2333d88ecd229af842a730224cafdd8a7a6e050f2c5611b250'),
    'to_mic_20101220_nominees': (31679, '6614d97f37c92db4cf89530fbefd4367a43782b3ea46c7af1b37c2e786c32e02'),
    'to_mic_20101221_designate': (35956, 'deb4413c1fdc33b4cac4f5cf6225a16a53bb21ecefbddcf1a0ebf12b3f74e9e3'),
    'to_palace_20101222_appointment': (34100, 'dd2d472b4ddbb972d586f5b8599cd062d10f0d969749429a7400b5e09f44684d'),
    'to_la_20141209_special_sitting': (46657, '2b34e30806269b6e58993862745ce832502716bda9e08d0df41e94ce9fdeb1fa'),
    'to_la_20141229_nominations': (32613, '9ef62e7f05b9f7e675d5015968126c3ceb512d95510d5c1793ec1ef0278bd24a'),
    'to_la_20141229_report_to_king': (32805, '4e10fc8c948350a63046cc5d4d88a95f883bab7620a6217b3d32f662ea9b18ea'),
    'to_la_20141229_pohiva_elected': (32247, 'a018c79a07bbfc9a4474a1dbbf0f50b242c3b8c994bf283b666010bd93e50495'),
    'to_pmo_20141229_pm_elect': (48086, '68baba1b74a53c8ba0c1901d7e05e9d2f6cf7b0e2981bfb7a39fbd30d5db6608'),
    'to_pmo_20141230_appointment': (47640, 'a72e7de90d3ad48c5a89dbfcc6ba11468dfac38a99063ac3a73f26533fb9d350'),
    'to_gazette_supp_14_2017': (204017, '5124737e65e4ff232e0e122858f3b0625946abc1451521b3c3133a59703907ae'),
    'to_la_20170912_speaker_release': (33527, 'a6908cbe88ed37bb7aee5ce9e7ff0de9e633b713b6f72874949c7363dad0921b'),
    'to_mic_20170907_sovaleni_letter': (35165, '365cbeac661f31f846ee0d933ce4aab0f8b58f5bb12935dddb90d0b63f239165'),
    'to_mic_20171120_interim_speaker': (32141, 'ba0ac1467d8a353ecea7d4dc44b0a93eedd02cd9cf1ef32d9c85c11852a885e6'),
    'to_mic_20171215_nominations': (29425, '94a74a451c5628196bbc4eb707ae1716d934c9f89dff30d96e0c02633dbe2f85'),
    'to_la_20171215_meeting_programme': (39505, 'f44496493a0965d935349923d59b9d1799f7a33b23655ece922ea6740e1b34ef'),
    'to_mic_20171218_selection_to': (27861, '7aae9e76242ce2b1150925ef921c598cabe8e29b98772873ad4cc4cdf0c511be'),
    'to_pmo_20180104_appointment': (31574, 'a6021ca934fd05804f521992f15f275ef78379abfb04b03e269b3fc1ee5e97e3'),
    'to_pmo_20180122_cabinet': (34016, 'f47dc0dce1a3e533be47bc1307fdd7e45ac65070633a52cdfafc12f6acf1c9e6'),
    'to_la_minutes_20180118': (311902, '69b70a0381fb1c14b8752c86d6e0671b8f5f31e2ffdaf59466bd1b82c5e8f181'),
    'to_la_minutes_20190912': (832241, '6eee316fb7d07de26c0ce9590c07b64fcb6e665b7d49df40d0d50fa7fd58268d'),
    'to_pmo_20190914_funeral_programme': (502596, '020d23f8fe799f8d5d38f2a618f0fff7e2ad99c4ec8202bba699b500fbbb35cf'),
    'to_pmo_20190916_public_notice': (162294, '4250dc7e2f77e55ebf5cfd9a0d63814134dae326fc63c0ac629a2807f7e96c3b'),
    'to_gazette_ext_28_2019': (64374, 'eb1f70f2dad6c22177b595762926c25e2ac59f427c906f53cc630e1ff1db75a4'),
}
NEW_SOURCES = set(RESPONSES)
# The IPU pages embed a per-request Cloudflare script; the extracts record a normalised hash and its rule instead.
IPU_NORMALISED = {
    'to_ipu_1990': (2731, '94904e4ba80231f4462c86fec2dfd7edf2a00de4038cbba18b8d8a2723fbad07'),
    'to_ipu_1993': (3463, 'c4caf4cea6514b6f1ee1decb0da41c5533e0b1dc9c8589b4d8194130afd5d78b'),
}
# Raw Internet Archive captures: source id -> capture timestamp.
ARCHIVED = {
    'to_pmo_former_pms_2015': '20150423093610', 'to_mic_pm_list_2011': '20110131121132',
    'to_pmo_vaea_tribute_20090613': '20100825201222', 'to_pmo_office_pm_2002': '20020420031118',
    'to_pmo_timeline_2000_en': '20100824203123', 'to_pmo_timeline_2000_to': '20100824231826',
    'to_pmo_20060213_resignation': '20060501015618', 'to_pmo_20060213_resignation_to': '20060501015706',
    'to_pmo_20060330_farewell_speech': '20060426083239', 'to_pmo_20060410_marist_remarks': '20060501190324',
    'to_pmo_20060413_psc': '20060501190232', 'to_pmo_timeline_2006_en': '20100824203137',
    'to_pmo_timeline_2006_to': '20100824231841', 'to_mic_20101220_nominees': '20111130060905',
    'to_mic_20101221_designate': '20120525141049', 'to_palace_20101222_appointment': '20111130040811',
    'to_la_20141209_special_sitting': '20161027022423', 'to_la_20141229_nominations': '20161027052431',
    'to_la_20141229_report_to_king': '20161027052419', 'to_la_20141229_pohiva_elected': '20161027052405',
    'to_pmo_20141229_pm_elect': '20150706224416', 'to_pmo_20141230_appointment': '20150707000535',
    'to_la_20170912_speaker_release': '20170914091449', 'to_mic_20170907_sovaleni_letter': '20170910132342',
    'to_mic_20171120_interim_speaker': '20181031091602', 'to_mic_20171215_nominations': '20180115225644',
    'to_la_20171215_meeting_programme': '20180302231437', 'to_mic_20171218_selection_to': '20180115225424',
    'to_pmo_20180104_appointment': '20180113055859', 'to_pmo_20180122_cabinet': '20181030055037',
}
PDF_PAGES = {'to_gazette_supp_14_2017': [1], 'to_la_minutes_20180118': [2, 7], 'to_la_minutes_20190912': [2, 7],
             'to_pmo_20190914_funeral_programme': [1, 2], 'to_pmo_20190916_public_notice': [1],
             'to_gazette_ext_28_2019': [1]}
ENTRY_ONLY = ('to_dissolution_instrument_20170824', 'to_pohiva_cabinet_mourning_decisions_20190916')

TUIPELEHAKE, VAEA, LAVAKA, SEVELE = "Fatafehi Tu'ipelehake", 'Baron Vaea', "Prince 'Ulukalala Lavaka Ata", 'Feleti Sevele'
TUIVAKANO, POHIVA = "Lord Tu'ivakano", "Samuela 'Akilisi Pohiva"
# CLAUDE-C01-03 appends Eke (attested at his 2025 appointment) after Sovaleni.
NAMES = [TUIPELEHAKE, VAEA, LAVAKA, SEVELE, TUIVAKANO, POHIVA, POHIVA, "Pohiva Tu'i'onetoa",
         "Siaosi 'Ofakivahafolau Sovaleni", "'Aisake Valu Eke"]
# (attested_on, from, until, attested_period) of the seven holders this packet adds, in list order.
HOLDERS = [
    (None, None, None, {'from': '1990-01-01', 'through': '1990-12-31'}),
    (None, None, None, {'from': '1992-01-01', 'through': '1998-12-31'}),
    (None, '2000-01-03', '2006-02-11', None),
    ('2006-04-07', None, None, None),
    ('2010-12-22', None, None, None),
    ('2014-12-30', None, None, None),
    (None, '2018-01-02', None, None),
]
HOLDER_CLAIMS = [
    ['to_pmo_former_pms_2026_tuipelehake', 'to_pmo_former_pms_2015_list', 'to_mic_pm_list_2011_list',
     'to_ipu_1990_tuipelehake'],
    ['to_pmo_former_pms_2026_vaea_lavaka', 'to_pmo_former_pms_2015_list', 'to_mic_pm_list_2011_list',
     'to_pmo_vaea_tribute_appointment_undated', 'to_ipu_1993_retirement_succession_199108'],
    ['to_pmo_lavaka_ata_commencement_20000103', 'to_pmo_timeline_lavaka_ata_appointed_200001',
     'to_lavaka_ata_resignation_accepted_20060211', 'to_lavaka_ata_resignation_accepted_20060211_to'],
    ['to_sevele_pm_20060407', 'to_sevele_acting_then_pm_20060413'],
    ['to_tuivakano_royal_appointment_20101222'],
    ['to_pohiva_royal_appointment_20141230'],
    ['to_pohiva_royal_appointment_20180102', 'to_pohiva_appointment_repeated_20180122'],
]
# Claims that must never feed a holder: selections, procedure, oaths, acting service, retrospective or relative dates,
# leave-taking, descriptions, dissolution, death and funeral records.
NEVER_HOLDER = (
    'to_pm_2010_special_meeting_nominees_20101220', 'to_tuivakano_assembly_result_presented_20101221',
    'to_tuivakano_appointment_announced_20101221', 'to_sevele_leave_audience_20101222',
    'to_pm_2014_writ_returned_timetable_20141209', 'to_pm_2014_two_nominations_20141229',
    'to_pm_2014_result_reported_to_king_20141229', 'to_pohiva_assembly_selection_20141229',
    'to_pohiva_declared_pm_elect_pmo_20141229', 'to_dissolution_instrument_20170824', 'to_pohiva_styled_pm_20170912',
    'to_pohiva_conveys_deputy_pm_removal_20170905', 'to_interim_speaker_tangi_20171117',
    'to_pm_2017_nominations_closed_20171214', 'to_pm_2017_meeting_scheduled_20171218',
    'to_pohiva_assembly_reselection_20171218', 'to_pohiva_oath_assembly_20180118',
    'to_sevele_acting_pm_appointed_20060211', 'to_sevele_acting_pm_20060323', 'to_pmo_vaea_tribute_acting_pm_20090613',
    'to_sika_acting_pm_20190914_programme', 'to_sika_acting_pm_20190916_gazette', 'to_sevele_appointed_20060330',
    'to_sevele_appointed_2006_to', 'to_pmo_former_pms_2026_later_years', 'to_la_20190912_pm_prayers_adjourned',
    'to_pohiva_late_pm_state_funeral_20190914', 'to_pohiva_cabinet_mourning_decisions_20190916',
    'to_gazette_pohiva_death_notice_20190916', 'to_ipu_2014_pohiva_assembly_selection_20141229',
    'to_ipu_2014_royal_endorsement_following_day', 'to_pohiva_death_month_2019')
ACTING = ('to_sevele_acting_pm_appointed_20060211', 'to_sevele_acting_pm_20060323', 'to_pmo_vaea_tribute_acting_pm_20090613',
          'to_sika_acting_pm_20190914_programme', 'to_sika_acting_pm_20190916_gazette')
# Dates that must never be a holder boundary or holder date: selections, presentation, oath, acting appointments,
# the retrospective timeline date, the leave audience, the dissolution, the 12 September Auckland item and publications.
NOT_BOUNDARIES = {'2006-03-30', '2010-12-20', '2010-12-21', '2014-12-29', '2017-08-24', '2017-12-18', '2018-01-04',
                  '2018-01-18', '2019-09-12', '2019-09-14', '2019-09-16'}
LEAD_URL_MARKERS = ('2317_99', 'state.gov', 'matangitonga', '290-second-nomination', 'GazetteNo.4of2018',
                    'king-appoints-new-pm-and-cabinet-ministers', '306-miniti-fika-20a', 'wikipedia')
REPORT = research.RESEARCH / 'tonga-prime-ministers-1990-2019-08.md'
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-08.md'


def pm_invariants(packet):
    """Packet-level rules this test owns; raises AssertionError or KeyError on any violation."""
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    institutions = {e['id']: e for e in packet['institutions']}
    pm = institutions['to_prime_minister']
    assert [r['id'] for r in pm['roles']] == ['to_pm'], 'no second prime-minister role'
    heads = [r['id'] for e in packet['organizations'] + packet['institutions'] for r in e['roles']
             if r['kind'] == 'head_of_government']
    assert heads == ['to_pm'], 'no second head of government'
    role = pm['roles'][0]
    assert role['holder_claims'][:2] == ['to_fakafanua_appointment', 'to_fakafanua_august'], 'string holders stay first'
    holders = [h for h in role['holder_claims'] if isinstance(h, dict)]
    assert [h['name'] for h in holders] == NAMES, 'exactly the intended holders, in order'
    for holder, expected, cids in zip(holders, HOLDERS, HOLDER_CLAIMS):
        got = (holder['attested_on'], holder['from'], holder['until'], holder.get('attested_period'))
        assert got == expected, (holder['name'], got)
        assert holder['claim_ids'] == cids, holder['name']
        assert not set(holder['claim_ids']) & set(NEVER_HOLDER), holder['name']
        assert not {holder['attested_on'], holder['from'], holder['until']} & NOT_BOUNDARIES, holder['name']
        assert 'Acting' not in holder['name'] and 'Sika' not in holder['name'], holder['name']
    for cid in ACTING:
        assert cid in role['claim_ids'], cid
    # Distinct dated events stay distinct.
    ballot = claims['to_tuivakano_assembly_result_presented_20101221']
    assert ballot['attested_on'] == '2010-12-21' and claims['to_tuivakano_royal_appointment_20101222']['attested_on'] == '2010-12-22'
    assert claims['to_pohiva_assembly_selection_20141229']['attested_on'] == '2014-12-29'
    assert claims['to_pohiva_royal_appointment_20141230']['attested_on'] == '2014-12-30'
    assert claims['to_pohiva_assembly_reselection_20171218']['attested_on'] == '2017-12-18'
    assert claims['to_pohiva_royal_appointment_20180102']['attested_on'] == '2018-01-02'
    assert claims['to_pohiva_oath_assembly_20180118']['attested_on'] == '2018-01-18'
    assert claims['to_lavaka_ata_resignation_accepted_20060211']['attested_on'] == '2006-02-11'
    assert claims['to_sevele_acting_pm_appointed_20060211']['attested_on'] == '2006-02-11'
    assert claims['to_sevele_appointed_20060330']['attested_on'] == '2006-03-30'
    assert claims['to_gazette_pohiva_death_notice_20190916']['attested_on'] == '2019-09-16'
    # Undated or year-precision claims carry no structured date.
    for cid in ('to_ipu_1990_tuipelehake', 'to_pmo_former_pms_2026_tuipelehake', 'to_sevele_appointed_2006_to',
                'to_pmo_vaea_tribute_appointment_undated', 'to_tuivakano_appointment_announced_20101221',
                'to_ipu_2014_royal_endorsement_following_day'):
        assert 'attested_on' not in claims[cid] and 'period' not in claims[cid], cid
    funeral = claims['to_pohiva_late_pm_state_funeral_20190914']
    assert funeral['attested_on'] == '2019-09-14' and 'period' not in funeral, 'no death window'


class TongaPrimeMinisterTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.raw = (research.ROOT / research.RESEARCH / 'tonga.json').read_text(encoding='utf-8')
        cls.packet = json.loads(cls.raw)
        cls.sources = {s['id']: s for s in cls.packet['sources']}
        cls.claims = {c['id']: c for s in cls.packet['sources'] for c in s['claims']}
        cls.claim_source = {c['id']: s['id'] for s in cls.packet['sources'] for c in s['claims']}
        cls.entries = {e['id']: e for category in ('organizations', 'institutions') for e in cls.packet[category]}
        cls.roles = {r['id']: r for e in cls.entries.values() for r in e['roles']}
        cls.pm = cls.entries['to_prime_minister']
        cls.role = cls.roles['to_pm']
        cls.holders = [h for h in cls.role['holder_claims'] if isinstance(h, dict)]
        cls.extracts = {
            sid: json.loads((research.ROOT / cls.sources[sid]['snapshot']['path']).read_text(encoding='utf-8'))
            for sid in NEW_SOURCES
        }
        cls.report = (research.ROOT / REPORT).read_text(encoding='utf-8')
        cls.new_claims = {c['id'] for sid in NEW_SOURCES for c in cls.sources[sid]['claims']}

    def validate(self, packet=None):
        return research.validate(packet or self.packet, research.ROOT, {'Tonga'}, {'Tonga': set()})

    def section(self, heading):
        parts = self.report.split(f'\n## {heading}', 1)
        self.assertEqual(len(parts), 2, heading)
        return parts[1].split('\n## ', 1)[0]

    def test_new_records_are_bounded_reuse_ids_and_every_claim_is_cited(self):
        ids = self.validate()
        self.assertLessEqual(NEW_SOURCES, set(ids['sources']))
        self.assertEqual((len(NEW_SOURCES), len(self.new_claims)), (39, 47))
        # Every new claim is cited by the prime-minister entry and no other; all but two sit on the to_pm role.
        self.assertLessEqual(self.new_claims, set(self.pm['claim_ids']))
        self.assertEqual(self.new_claims - set(self.role['claim_ids']), set(ENTRY_ONLY))
        for eid, entry in self.entries.items():
            if eid != 'to_prime_minister':
                self.assertFalse(self.new_claims & set(entry['claim_ids']), eid)
        # No new organization, institution or role; existing IDs are reused and existing sources are untouched.
        self.assertEqual(len(ids['entries']), 9)
        self.assertEqual({e['id'] for e in self.packet['institutions']},
                         {'to_crown', 'to_prime_minister', 'to_cabinet', 'to_privy_council', 'to_legislative_assembly'})
        self.assertEqual([r['id'] for r in self.pm['roles']], ['to_pm'])
        self.assertEqual([c['id'] for c in self.sources['to_ipu_2017']['claims']], ['to_ipu_2017_dpfi_continuity'])
        self.assertEqual([s['id'] for s in self.packet['sources'][-39:]], [s for s in self.sources if s in NEW_SOURCES])
        self.assertEqual(list(self.sources)[:64][-1], 'to_iha_2015_tonga_visit')
        observations = re.findall(r'^### (TO-PM90-\d\d)\b', self.report, re.M)
        self.assertEqual(observations, [f'TO-PM90-{n:02d}' for n in range(1, 9)])
        # Renamed per the check: no claim ID encodes a date the claim does not attest.
        for stale in ('to_tuivakano_assembly_selection_20101221', 'to_sika_acting_pm_20190917_programme'):
            self.assertNotIn(stale, self.raw)
        # Every structured date in the new records falls inside the period and before the cutoff.
        for sid in NEW_SOURCES:
            self.assertLessEqual(self.sources[sid].get('published_date', ''), '2019-10-08')
        for cid in self.new_claims:
            claim = self.claims[cid]
            for value in (claim.get('attested_on'), *(claim.get('period') or {}).values()):
                if value:
                    self.assertTrue('1990-01-01' <= value <= '2019-10-08', cid)

    def test_selection_appointment_effect_oath_acting_resignation_and_death_never_collapse(self):
        pm_invariants(self.packet)
        dates = {
            '2010 selection presented': self.claims['to_tuivakano_assembly_result_presented_20101221']['attested_on'],
            '2010 appointment audience': self.claims['to_tuivakano_royal_appointment_20101222']['attested_on'],
            '2014 selection': self.claims['to_pohiva_assembly_selection_20141229']['attested_on'],
            '2014 appointment audience': self.claims['to_pohiva_royal_appointment_20141230']['attested_on'],
            '2017 selection': self.claims['to_pohiva_assembly_reselection_20171218']['attested_on'],
            '2018 effective': self.claims['to_pohiva_royal_appointment_20180102']['attested_on'],
            '2018 oath': self.claims['to_pohiva_oath_assembly_20180118']['attested_on'],
        }
        self.assertEqual(len(set(dates.values())), 7)
        self.assertLess(dates['2010 selection presented'], dates['2010 appointment audience'])
        self.assertLess(dates['2014 selection'], dates['2014 appointment audience'])
        self.assertLess(dates['2017 selection'], dates['2018 effective'])
        self.assertLess(dates['2018 effective'], dates['2018 oath'])
        # The effective date is stated; the release date is only publication.
        self.assertIn('with effect from 2 January, 2018', self.claims['to_pohiva_royal_appointment_20180102']['text'])
        self.assertEqual(self.sources['to_pmo_20180104_appointment']['published_date'], '2018-01-04')
        # The 2010 ballot keeps no date of its own: 21 December is the presentation and announcement only.
        ballot = self.claims['to_tuivakano_assembly_result_presented_20101221']
        self.assertIn('not of the ballot', ballot['uncertainty'])
        self.assertIn('20 or 21 December 2010', ballot['uncertainty'])
        self.assertIn("'unanimously'", ballot['uncertainty'])
        # Resignation acceptance and the acting appointment share a day but stay two claims; acting is never a holder.
        self.assertEqual(self.claim_source['to_lavaka_ata_resignation_accepted_20060211'],
                         self.claim_source['to_sevele_acting_pm_appointed_20060211'])
        for cid in ACTING:
            self.assertRegex(self.claims[cid]['uncertainty'], r'not (as )?a holder|never a holder')
        # The retrospective timeline date is kept, labelled, and not used.
        timeline = self.claims['to_sevele_appointed_20060330']['uncertainty']
        self.assertTrue(timeline.startswith('Retrospective timeline date'))
        self.assertIn('not the date of the holder observation', timeline)
        self.assertEqual(self.sources['to_pmo_timeline_2006_en']['source_type'],
                         'government_hosted_retrospective_timeline_archived')
        # Tongan wording is quoted as printed (cp1252 glottal stops kept, macrons absent), with the macron spelling in brackets.
        tongan = self.claims['to_lavaka_ata_resignation_accepted_20060211_to']['text']
        for printed in ("'malolo' as printed [mālōlō]", "'Palemia Le'ole'o' as printed [Palēmia Le'ole'o]"):
            self.assertIn(printed, tongan)
        self.assertNotIn("Le'ole'o (Acting", tongan)
        # Death: recorded, bounded to on or before the Auckland item, and never a day.
        funeral = self.claims['to_pohiva_late_pm_state_funeral_20190914']
        self.assertIn('on or before that day (Auckland date)', funeral['uncertainty'])
        self.assertIn('no day of death is recorded', funeral['uncertainty'])
        self.assertIn('DEATH OF THE PRIME MINISTER', self.claims['to_gazette_pohiva_death_notice_20190916']['text'])
        self.assertIn('not used to date the death', self.claims['to_la_20190912_pm_prayers_adjourned']['uncertainty'])
        # The 2017 context is recorded, dated, and decides nothing about his status.
        letter = self.claims['to_pohiva_conveys_deputy_pm_removal_20170905']
        self.assertEqual(letter['attested_on'], '2017-09-05')
        self.assertIn("the 'Palemia 'o Tonga Hon. 'Akilisi Pohiva'", letter['text'])
        self.assertIn('full or caretaker', letter['uncertainty'])
        self.assertIn('full or caretaker', self.claims['to_pohiva_assembly_reselection_20171218']['uncertainty'])
        self.assertIn("'served as the 16th Prime Minister from 2015-2017'",
                      self.claims['to_pohiva_royal_appointment_20180102']['uncertainty'])
        self.assertIn('not used as boundaries', self.claims['to_pohiva_royal_appointment_20180102']['uncertainty'])
        # IPU corroboration carries no borrowed date.
        self.assertNotIn('Last-Modified', self.claims['to_ipu_1993_retirement_succession_199108']['uncertainty'])
        self.assertEqual(self.claims['to_ipu_1993_retirement_succession_199108']['period'],
                         {'from': '1991-08-01', 'through': '1991-08-31'})
        self.assertIn("'the next month'", self.claims['to_ipu_1990_tuipelehake']['uncertainty'])

    def test_ends_only_where_a_source_states_one(self):
        ends = [(h['name'], h['until']) for h in self.holders if h['until']]
        # The only other stated end is CLAUDE-C01-03's: Sovaleni's resignation and its acceptance, 9 December 2024.
        self.assertEqual(ends, [(LAVAKA, '2006-02-11'), ("Siaosi 'Ofakivahafolau Sovaleni", '2024-12-09')])
        lavaka = self.holders[2]
        self.assertEqual(lavaka['until'], self.claims['to_lavaka_ata_resignation_accepted_20060211']['attested_on'])
        self.assertEqual(lavaka['from'], self.claims['to_pmo_lavaka_ata_commencement_20000103']['attested_on'])
        self.assertIn("'Commencement Date : 3 rd January, 2000'", self.claims['to_pmo_lavaka_ata_commencement_20000103']['text'])
        starts = [(h['name'], h['from']) for h in self.holders if h['from']]
        self.assertEqual(starts, [(LAVAKA, '2000-01-03'), (POHIVA, '2018-01-02'),
                                  ("Siaosi 'Ofakivahafolau Sovaleni", '2021-12-27')])
        # No end from a successor's start, a leave audience, a description, a year span or a death without a day.
        vaea, sevele, tuivakano, pohiva_2014, pohiva_2018 = (self.holders[i] for i in (1, 3, 4, 5, 6))
        self.assertIsNone(vaea['until'])
        self.assertIn('not inferred from this start', lavaka['uncertainty'])
        self.assertIn('not used as an end date', sevele['uncertainty'])
        self.assertIn("'former Prime Minister'", tuivakano['uncertainty'])
        self.assertIn('no end of this appointment is inferred', pohiva_2014['uncertainty'])
        self.assertIn('no end is set', pohiva_2018['uncertainty'])
        self.assertIn("Pohiva Tu'i'onetoa", [h['name'] for h in self.holders])
        self.assertIsNone(self.holders[7]['until'])
        # Year-range lists support only a year-precision observation, never an interval.
        for holder in self.holders[:2]:
            self.assertIn('Year precision only', holder['uncertainty'])
            self.assertIn('continuous tenure', holder['uncertainty'])
            self.assertEqual((holder['from'], holder['until'], holder['attested_on']), (None, None, None))
        self.assertIn('never holders or boundaries', self.role['scope_note'])
        unresolved = self.pm['coverage']['unresolved']
        packet_rows = [u for u in unresolved if u.startswith('TO-PM90-')]
        self.assertEqual([u.split(':', 1)[0].split(' ', 1)[0] for u in packet_rows],
                         ['TO-PM90-01/02', 'TO-PM90-03', 'TO-PM90-04', 'TO-PM90-05/06', 'TO-PM90-07', 'TO-PM90-08'])
        # CLAUDE-C01-03 appends its own notes after these, so pin exactly one entry each rather than the last one.
        dpfi = [u for u in unresolved if u.startswith('TO-DPFI-05')]
        self.assertEqual(len(dpfi), 1)
        self.assertIn('from the IPU record (CLAUDE-C01-08 later adds his holders', dpfi[0])
        self.assertEqual(sum('Prime-minister packet 08' in u for u in self.packet['coverage']['unresolved']), 1)

    def test_holders_are_exactly_as_intended(self):
        pm_invariants(self.packet)
        expected_sources = [
            ['to_pmo_former_pms_2026', 'to_pmo_former_pms_2015', 'to_mic_pm_list_2011', 'to_ipu_1990'],
            ['to_pmo_former_pms_2026', 'to_pmo_former_pms_2015', 'to_mic_pm_list_2011', 'to_pmo_vaea_tribute_20090613',
             'to_ipu_1993'],
            ['to_pmo_office_pm_2002', 'to_pmo_timeline_2000_en', 'to_pmo_20060213_resignation',
             'to_pmo_20060213_resignation_to'],
            ['to_pmo_20060410_marist_remarks', 'to_pmo_20060413_psc'],
            ['to_palace_20101222_appointment'],
            ['to_pmo_20141230_appointment'],
            ['to_pmo_20180104_appointment', 'to_pmo_20180122_cabinet'],
        ]
        for holder, sources in zip(self.holders, expected_sources):
            self.assertEqual(holder['sources'], sources, holder['name'])
            self.assertTrue(holder['note'] and holder['uncertainty'], holder['name'])
            for cid in holder['claim_ids']:
                self.assertIn(self.claim_source[cid], sources)
        # The existing 2019 and 2021 holders keep their places and content; the string holders stay first.
        self.assertEqual(self.role['holder_claims'][:2], ['to_fakafanua_appointment', 'to_fakafanua_august'])
        # Two string holders, this packet's seven, the 2019 and 2021 ones, and CLAUDE-C01-03's Eke.
        self.assertEqual(len(self.role['holder_claims']), 12)
        self.assertEqual(self.holders[7]['claim_ids'], ['to_tuionetoa_royal_appointment_20191008'])
        self.assertEqual(self.holders[8]['from'], '2021-12-27')
        # Acting service, the Deputy Prime Minister and the selections become nobody's holder anywhere.
        for role_id, role in self.roles.items():
            for entry in role['holder_claims']:
                ids = [entry] if isinstance(entry, str) else entry['claim_ids']
                self.assertFalse(set(ids) & set(NEVER_HOLDER), role_id)
                if isinstance(entry, dict):
                    self.assertNotIn('Sika', entry['name'])
                    self.assertNotIn('Tangi', entry['name'])
                    self.assertNotIn('Kavaliku', entry['name'])
        self.assertIn('Acting Prime Ministers are recorded only as claims', ' '.join(self.pm['coverage']['unresolved']))

    def test_extracts_match_packet_claims_and_record_original_responses(self):
        for sid in NEW_SOURCES:
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual(extract['format'], 'spheres-c01-derived-factual-table/v1')
            self.assertEqual(extract['source_url'], source['url'])
            self.assertEqual(extract['claims'], source['claims'])
            self.assertEqual(extract['scope_note'], source['scope_note'])
            self.assertEqual(extract['access_method'], source['access_method'])
            self.assertEqual(extract['published_date'], source.get('published_date'))
            self.assertEqual((extract['accessed_date'], source['accessed_date']), ('2026-09-22', '2026-09-22'))
            self.assertFalse(extract['source_response_checked_in'])
            self.assertEqual((extract['source_response_bytes'], extract['source_response_sha256']), RESPONSES[sid])
            snapshot = source['snapshot']
            self.assertEqual(snapshot['kind'], 'derived_factual_extract')
            self.assertTrue(snapshot['path'].startswith('docs/campaign-certification/C01/research/sources/tonga-'))
            data = (research.ROOT / snapshot['path']).read_bytes()
            self.assertEqual((len(data), hashlib.sha256(data).hexdigest()), (snapshot['bytes'], snapshot['sha256']))
            self.assertNotEqual(snapshot['sha256'], extract['source_response_sha256'])
            self.assertTrue(data.endswith(b'}\n') and b'\r' not in data)
            self.assertEqual(data.decode('utf-8'), json.dumps(extract, indent=2, ensure_ascii=False) + '\n')
            self.assertIn('No open license, portrait permission or likeness approval', extract['rights_note'])
            self.assertIn('derived factual extract', extract['provenance_note'])
            self.assertEqual(extract['visual_review']['pdf_pages_one_based'], PDF_PAGES.get(sid, []))
            self.assertTrue(source['source_type'] and source['scope_note'])
        for sid, stamp in ARCHIVED.items():
            source, extract = self.sources[sid], self.extracts[sid]
            url = urlsplit(source['url'])
            self.assertEqual(url.hostname, 'web.archive.org')
            self.assertTrue(url.path.startswith(f'/web/{stamp}id_/http'), sid)
            self.assertEqual(extract['original_url'], source['original_url'])
            self.assertNotIn(':80', source['original_url'])
            self.assertEqual(extract['archive_capture_utc'].replace('-', '').replace(':', '').replace('T', '').rstrip('Z'), stamp)
            self.assertIn('Raw Internet Archive capture', extract['provenance_note'])
        self.assertEqual({urlsplit(self.sources[s]['url']).hostname for s in NEW_SOURCES - set(ARCHIVED)},
                         {'pmo.gov.to', 'data.ipu.org', 'ago.gov.to', 'parliament.gov.to'})
        # Unreproducible raw responses assert no hash and say why; IPU records a normalised hash and its rule.
        for sid, (nbytes, nsha) in IPU_NORMALISED.items():
            extract = self.extracts[sid]
            self.assertIsNone(extract['source_response_sha256'])
            self.assertIn('no raw response hash is asserted', extract['provenance_note'])
            self.assertEqual((extract['normalized_response']['bytes'], extract['normalized_response']['sha256']), (nbytes, nsha))
            self.assertIn("'<script>(function(){function c(){'", extract['normalized_response']['rule'])
            self.assertIn('HTTP 403', extract['normalized_response']['note'])
            self.assertIn('Cloudflare', self.sources[sid]['scope_note'])
        # Pages read after the cutoff say so; the edited live list uses only its pre-2020 entries.
        for sid in ('to_pmo_former_pms_2026', 'to_ipu_1990', 'to_ipu_1993', 'to_gazette_supp_14_2017',
                    'to_la_minutes_20180118', 'to_la_minutes_20190912', 'to_pmo_20190914_funeral_programme',
                    'to_pmo_20190916_public_notice', 'to_gazette_ext_28_2019'):
            self.assertIn('after the', self.sources[sid]['scope_note'])
            self.assertIn('cutoff', self.sources[sid]['scope_note'])
        self.assertIn('only the entries for holders before 2020 are used', self.sources['to_pmo_former_pms_2026']['scope_note'])
        for sid in ('to_la_minutes_20180118', 'to_la_minutes_20190912'):
            self.assertIn('did not submit that form', self.extracts[sid]['provenance_note'])
        self.assertIn('Located by the independent check', self.extracts['to_mic_20170907_sovaleni_letter']['provenance_note'])

    def test_secondary_and_unimported_leads_stay_out_of_the_packet(self):
        for source in self.packet['sources']:
            self.assertFalse(any(marker in source['url'] for marker in LEAD_URL_MARKERS), source['id'])
        self.assertNotIn('to_ipu_1999', self.sources)
        self.assertEqual(sum('TO-LC01-E20171116' in s['url'] for s in self.packet['sources']), 1)
        for cid in self.new_claims:
            text = self.claims[cid]['text'].lower()
            for word in ('kavaliku', 'dismissed', 'wikipedia', 'matangi', 'u.s. department of state'):
                self.assertNotIn(word, text, cid)
        lowered = self.raw.lower()
        for marker in ('state.gov', 'matangi', 'wikipedia', 'four years', '12 september 2019'):
            self.assertNotIn(marker, lowered, marker)
        leads, added = self.section('Leads not imported'), self.section('Sources added')
        for marker in ('2317_99.htm', 'dismissed by the King', 'Country Reports on Human Rights Practices for 2011',
                       '290-second-nomination', 'king-appoints-new-pm-and-cabinet-ministers-effective-last-week',
                       'TO-LC01-E20171116'):
            self.assertIn(marker, leads, marker)
            self.assertNotIn(marker, added, marker)

    def test_packet_formatting_is_preserved(self):
        data = (research.ROOT / research.RESEARCH / 'tonga.json').read_bytes()
        self.assertNotIn(b'\r', data)
        self.assertEqual(data.decode('utf-8'), json.dumps(self.packet, indent=2, ensure_ascii=False) + '\n')

    def test_mutations_are_rejected(self):
        def mutated(change):
            packet = copy.deepcopy(self.packet)
            change(packet)
            return packet

        def source(packet, sid):
            return next(s for s in packet['sources'] if s['id'] == sid)

        def claim(packet, cid):
            return next(c for s in packet['sources'] for c in s['claims'] if c['id'] == cid)

        def role(packet):
            return next(r for e in packet['institutions'] for r in e['roles'] if r['id'] == 'to_pm')

        def holder(packet, index):
            return [h for h in role(packet)['holder_claims'] if isinstance(h, dict)][index]

        validator_cases = [
            (lambda p: source(p, 'to_pmo_20180104_appointment')['snapshot'].update(sha256='0' * 64), 'checksum mismatch'),
            (lambda p: source(p, 'to_gazette_ext_28_2019')['snapshot'].update(bytes=1), 'checksum mismatch'),
            (lambda p: source(p, 'to_ipu_1990')['snapshot'].update(path=REPORT.as_posix()), 'escapes'),
            (lambda p: holder(p, 6).update(until='2026-09-08'), 'exceeds cutoff'),
            (lambda p: holder(p, 5).update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: holder(p, 0)['attested_period'].update(through='2026-09-08'), 'exceeds cutoff'),
            (lambda p: claim(p, 'to_gazette_pohiva_death_notice_20190916').update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: claim(p, 'to_ipu_1993_retirement_succession_199108')['period'].update(through='2026-09-30'),
             'exceeds cutoff'),
            (lambda p: claim(p, 'to_pmo_former_pms_2026_tuipelehake').update(period={'from': '1965', 'through': '1991'}),
             '(?i)invalid'),
            (lambda p: holder(p, 2).update(until='1999-12-31'), 'Reversed historical interval'),
            (lambda p: holder(p, 4).update(claim_ids=['to_pohiva_royal_appointment_20141230']), 'cited source'),
            (lambda p: role(p)['claim_ids'].append('to_pm_does_not_exist'), 'Unknown'),
        ]
        for change, message in validator_cases:
            with self.subTest(message=message), self.assertRaisesRegex(ValueError, message):
                self.validate(mutated(change))
        invariant_cases = [
            ('selection as start', lambda p: holder(p, 6).update({'from': '2017-12-18'})),
            ('oath as start', lambda p: holder(p, 6).update({'from': '2018-01-18'})),
            ('release date as start', lambda p: holder(p, 6).update({'from': '2018-01-04'})),
            ('Vaea end inferred from Lavaka Ata start', lambda p: holder(p, 1).update(until='2000-01-03')),
            ('2014 end inferred from re-appointment', lambda p: holder(p, 5).update(until='2018-01-02')),
            ('death day inferred', lambda p: holder(p, 6).update(until='2019-09-12')),
            ('leave audience as end', lambda p: holder(p, 3).update(until='2010-12-22')),
            ('timeline date for Sevele', lambda p: holder(p, 3).update(attested_on='2006-03-30')),
            ('year range as interval', lambda p: holder(p, 0).update({'from': '1965-01-01', 'until': '1991-12-31'})),
            ('acting service as holder', lambda p: role(p)['holder_claims'].append({
                'name': 'Semisi K.L. Sika', 'attested_on': '2019-09-16', 'from': None, 'until': None,
                'sources': ['to_gazette_ext_28_2019'], 'claim_ids': ['to_sika_acting_pm_20190916_gazette']})),
            ('acting claim cited by a holder', lambda p: holder(p, 3)['claim_ids'].append('to_sevele_acting_pm_appointed_20060211')),
            ('selection cited by a holder', lambda p: holder(p, 5)['claim_ids'].append('to_pohiva_assembly_selection_20141229')),
            ('ballot collapsed into appointment', lambda p: claim(p, 'to_tuivakano_assembly_result_presented_20101221').update(
                attested_on='2010-12-22')),
            ('IPU 1990 given a date', lambda p: claim(p, 'to_ipu_1990_tuipelehake').update(attested_on='1990-02-16')),
            ('funeral as a death window', lambda p: claim(p, 'to_pohiva_late_pm_state_funeral_20190914').update(
                period={'from': '2019-09-12', 'through': '2019-09-14'})),
            ('second prime-minister role', lambda p: next(e for e in p['institutions'] if e['id'] == 'to_prime_minister')[
                'roles'].append({'id': 'to_acting_pm', 'title': 'Acting Prime Minister', 'kind': 'head_of_government',
                                 'sources': ['to_gazette_ext_28_2019'], 'claim_ids': ['to_sika_acting_pm_20190916_gazette'],
                                 'holder_claims': []})),
        ]
        pm_invariants(self.packet)
        for label, change in invariant_cases:
            with self.subTest(label=label), self.assertRaises((AssertionError, KeyError)):
                pm_invariants(mutated(change))

    def test_report_and_handoff_are_ready_for_review_stacked_and_close_nothing(self):
        self.assertIn('ready_for_review', self.report)
        table = self.section('Outcome')
        decisions = {'01': 'Accepted at year precision', '02': 'Unresolved', '03': 'Accepted', '04': 'Accepted in part',
                     '05': 'Accepted', '06': 'Accepted', '07': 'Accepted', '08': 'Accepted in part'}
        for number, decision in decisions.items():
            row, = [line for line in table.splitlines() if line.startswith(f'| TO-PM90-{number} ')]
            self.assertIn(f'**{decision}', row)
        defects = self.section('Checker defects')
        rows = [line for line in defects.splitlines() if re.match(r'\| D\d+ ', line)]
        self.assertEqual([re.match(r'\| (D\d+) ', r).group(1) for r in rows], [f'D{n}' for n in range(1, 14)])
        for row in rows:
            self.assertRegex(row, r'\*\*Applied')
        for marker in ('C01', 'C06', 'S23', 'WC1', 'CP1'):
            self.assertNotRegex(self.report, rf'\b{marker}\b[^.\n]*\bis (now )?complete\b')
        notes = self.section('Integration notes')
        for text in ('CLAUDE-C01-04', 'CLAUDE-C01-07', 'e6f9fa41', 'CLAUDE-C01-03', 'holder order', 'research-index.json'):
            self.assertIn(text, notes)
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        for text in ('ready_for_review', 'tonga-prime-ministers-1990-2019-08.md', 'claude/c01-tonga-08', 'e6f9fa41',
                     'CLAUDE-C01-03', '22 September 2026 instruction', 'pending Codex acceptance',
                     'test_tonga_pm_1990_2019_c01_08.py'):
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
