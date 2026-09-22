"""CLAUDE-C01-07: Tonga's Crown chronology keeps accession, proclamation, installation, coronation, death and regency apart."""
import copy
import hashlib
import json
import re
import unittest
from urllib.parse import urlsplit

import campaign_research as research


# Original response identity recorded in each extract: (bytes, sha256).
RESPONSES = {
    'to_act_income_tax_amend_1990': (35041, '25d2c916b3fb7e2187d7e5ae0350651ea453de3e1a4ba84caa786f90c485c419'),
    'to_act_constitution_amend_1990': (32413, '4eab11f44316d1687023313b05119ff41d64f78e84c2622043d445c841dca90a'),
    'to_tlr_2006': (2053972, '2b4b56d06edd5cdaa65fe74d55b29dff37aa366a162374fb6f3801d3114c3aef'),
    'to_act_income_tax_amend_2006': (28627, '379391bdb43899697e95948988f99a9e0e27ef89809f47299ef00e8565934731'),
    'to_pmo_20060313_qsc_anniversary': (13490, '3c22f49704578d31f44fee0e1c77941bb1fd9756f491fb343d2eba4d7474b3b5'),
    'to_pmo_20060601_throne_speech': (23572, '2643d47bb38d0960d9882e4cce00c4990f3041573c007b69926f358cc7223803'),
    'to_pmo_20060818_succession': (11836, 'e910e5c38ce005cd0dbd2666c3de9e1916a2f3a20d47af6f899fa757da8011b5'),
    'to_pmo_20060911_proclamation': (11706, '637e14492c723f8ccf2a6f8cbba34a20b21cbbcce23930e2c0dca9dbaef5d378'),
    'to_pmo_20060911_death_notice': (11613, '0e8ebd4c7b7ef60b26b61030853879d4b29ae7a6791986168fba73632e64c73e'),
    'to_pmo_20060911_proclamation_to': (12662, 'f02525397741bd0337680bc50b51aa29482c8567294f43f767be7978d8a6ec77'),
    'to_pmo_20060914_funeral_holiday': (13615, '9947f7776dd84d2f27fce159ba67214b868ff78d69a9d7bb84608edddb8a9001'),
    'to_pmo_20060920_last_tribute': (12099, 'c9de08e98677ab1070b4316685b71f8c99d40555f703f1f8cd8caf361e0bb993'),
    'to_pmo_20060928_crown_prince': (12821, '4db64e8e4fa052db1c5c483e1b7d4227c2dd284f0edd1fef9988e96d0518b68a'),
    'to_act_public_enterprises_amend_2006': (55577, 'dfbd2d2833725bbc8cd80e9a64c9c0e016ca2959ab5567dbb6cc556b4b61ce99'),
    'to_pmo_2008_coronation_programme': (25085, 'c574845d509dd4d3c065c8f798e0c5f1eb10be94cc88e5dd50b3f04500bd053b'),
    'to_pmo_20080728_lord_chamberlain_profile': (41693, '31da340f408061683c9e3c53179dcf101582457df8bbdb5b7eee04befa35ef03'),
    'to_pmo_20080730_taumafa_kava': (27576, '8fa1b7c092d03dbf48aa4d6a5075f22f3ce2bfc7352c7a13a3f706217ba64e75'),
    'to_pmo_2008_pm_luncheon_address': (32168, '68506f3797cb02f6414ff95792069302d47dd952fd0be3d713245f437d46e8aa'),
    'to_pmo_20080801_royal_visitors': (24911, 'ad3fd360dab76be82edf0d3f99228eb74528ae650a959c4b9bb1a8eacb244206'),
    'to_gazette_ext_8_2012': (659563, 'ec50ad5d05fbf90ec52ea6dce7913178b4e4bf04456e5c78e8e619c98eef0b6b'),
    'to_gazette_ext_9_2012': (662692, '773fd96d13b46e8fbba1e943e1f85d7b6d406cbc65937b313c3631f5b294d3e9'),
    'to_gazette_ext_14_2012': (650317, 'fd66a32fafb8df73fc748d31024030520750f28fe7e4af3b4caf1343bf4cad33'),
    'to_gazette_supp_4_2015': (11341, 'fb5fa4fa1f6c68ed4b86b974995d2f3010829ee5d0a5d7d329035f7846e74877'),
    'to_nrbt_ar_2015': (12583944, 'fc1b8717f0a247e956ae2af7b2664de1704d115b16e2ff22b6e0430aeec2041e'),
    'to_un_a61_pv1': (56229, '36a4903f5b55ef362f6daa70d8de272d6467f1dd9de6f91117b697be9ee23b8e'),
    'to_iha_2008_tonga_visit': (11320, 'b357802d80a19eca7b8f5ace84250d439800daab70f5a39437e30aad64d0dc41'),
    'to_iha_2015_tonga_visit': (10032, '9f90055bca476f2e81a6d50702c9ca9eb9b97a3c0a27db5ea754503f739fd884'),
}
NEW_SOURCES = set(RESPONSES)
# Raw Internet Archive captures of the original PMO pages: source id -> capture timestamp.
ARCHIVED = {
    'to_pmo_20060313_qsc_anniversary': '20060907124302',
    'to_pmo_20060601_throne_speech': '20061005234255',
    'to_pmo_20060818_succession': '20061018001211',
    'to_pmo_20060911_proclamation': '20061005235237',
    'to_pmo_20060911_death_notice': '20070514085818',
    'to_pmo_20060911_proclamation_to': '20070312032819',
    'to_pmo_20060914_funeral_holiday': '20061017154738',
    'to_pmo_20060920_last_tribute': '20061105045105',
    'to_pmo_20060928_crown_prince': '20061005234019',
    'to_pmo_2008_coronation_programme': '20080731092343',
    'to_pmo_20080728_lord_chamberlain_profile': '20080801052730',
    'to_pmo_20080730_taumafa_kava': '20080807055636',
    'to_pmo_2008_pm_luncheon_address': '20080807055641',
    'to_pmo_20080801_royal_visitors': '20080807055652',
}
REUSED_CLAIMS = {
    'to_constitution_2020': {'to_crown_const2020_succession_c32', 'to_crown_const2020_coronation_oath_c34',
                             'to_crown_const2020_prince_regent_c42_43'},
    'to_constitution_older': {'to_crown_const1988_form_tupou_iv_c31', 'to_crown_const1988_prince_regent_c42_43'},
    'to_ipu_2008': {'to_ipu_2008_gtv_speaker_20080502'},
    'to_ipu_2010': {'to_ipu_2010_gtv_acceded_sept2006'},
}
DEATH_1 = 'to_crown_tupou_iv_death_20060911'
DEATH_1_UN = 'to_crown_tupou_iv_death_20060910_un'
PROCLAIM_1 = ('to_crown_gtv_devolution_proclamation_20060911', 'to_crown_gtv_devolution_proclamation_to_20060911')
DEATH_2 = 'to_crown_gtv_death_20120318'
PROCLAIM_2 = 'to_crown_tupou_vi_devolution_proclamation_20120319'
INSTALLATION = ('to_crown_gtv_taumafa_kava_scheduled_20080730', 'to_crown_gtv_taumafa_kava_held_20080730',
                'to_crown_gtv_taumafa_kava_declaration_20080730', 'to_crown_gtv_investiture_address_wednesday')
CORONATION = ('to_crown_gtv_coronation_scheduled_20080801', 'to_crown_gtv_coronation_day_address_20080801',
              'to_crown_gtv_coronation_guests_20080801', 'to_crown_gtv_coronation_iha_20080801',
              'to_crown_tupou_vi_coronation_20150704_nrbt', 'to_crown_tupou_vi_coronation_iha_20150704',
              'to_crown_tupou_vi_coronation_exemptions_2015')
OATH = ('to_crown_const2020_coronation_oath_c34', 'to_crown_succession_not_oath_dependent_2006')
FUNERAL_MOURNING = ('to_crown_tupou_iv_funeral_announced_20060911', 'to_crown_tupou_iv_mourning_20060911',
                    'to_crown_tupou_iv_state_funeral_holiday_20060919', 'to_crown_tupou_iv_burial_reported_20060920',
                    'to_crown_gtv_mourning_declared_20120319', 'to_crown_gtv_mourning_revised_20120330')
REGENCY = ('to_crown_const2020_prince_regent_c42_43', 'to_crown_const1988_prince_regent_c42_43',
           'to_crown_regent_tupouto_a_2004_pleading', 'to_crown_prince_regent_tupouto_a_20060313',
           'to_crown_princess_regent_speech_20060601', 'to_crown_princess_regent_opening_20060601_court',
           'to_crown_regent_assent_practice_2006_court', 'to_crown_assent_tupouto_a_20060630',
           'to_crown_gtv_prince_regent_numerous_occasions')
TUPOU_IV, GEORGE, TUPOU_VI = "Taufa'ahau Tupou IV", 'George Tupou V', 'Tupou VI'
# The only holder boundaries a source states: deaths and the devolution of the Crown on them (Tongan dates).
BOUNDARIES = {TUPOU_IV: (None, '2006-09-11'), GEORGE: ('2006-09-11', '2012-03-18'), TUPOU_VI: ('2012-03-18', None)}
# Dates that must never be a reign boundary: NZ calendar date, proclamation, funeral, installation, coronations.
NOT_BOUNDARIES = {'2006-09-10', '2006-09-19', '2008-07-30', '2008-08-01', '2012-03-19', '2015-07-04'}
LEAD_MARKERS = ('state.gov', 'matangi', 'wikipedia', 'royal central', 'unofficial royalty', 'talanoa',
                'beehive', 'special-national-holiday', 'annual_report_2009', 'criminaloffences')
REPORT = research.RESEARCH / 'tonga-crown-07.md'
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-07.md'


def crown_invariants(packet):
    """Packet-level rules this test owns; raises AssertionError on any violation."""
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    crown = next(e for e in packet['institutions'] if e['id'] == 'to_crown')
    assert [r['id'] for r in crown['roles']] == ['to_king'], 'no second Crown or regent role'
    king = crown['roles'][0]
    holders = [h for h in king['holder_claims'] if isinstance(h, dict)]
    assert king['holder_claims'][0] == 'to_tupou_vi_2025', 'existing attestation stays first'
    assert [h['name'] for h in holders] == [TUPOU_IV, GEORGE, TUPOU_VI], 'exactly three reigns; no regent'
    for holder in holders:
        assert (holder['from'], holder['until']) == BOUNDARIES[holder['name']], holder['name']
        assert holder['attested_on'] is None, holder['name']
        assert not {holder['from'], holder['until']} & NOT_BOUNDARIES, holder['name']
        assert not set(holder['claim_ids']) & set(INSTALLATION + CORONATION + OATH + FUNERAL_MOURNING + REGENCY)
    assert not set(king['claim_ids']) & set(REGENCY + FUNERAL_MOURNING), 'regency and funerals are entry-level'
    # Death and proclamation stay distinct claims even where the Tongan day is shared.
    assert claims[DEATH_1]['attested_on'] == '2006-09-11' and claims[PROCLAIM_1[0]]['attested_on'] == '2006-09-11'
    assert claims[DEATH_1]['printed_times'] == [
        {'zone': 'New Zealand time', 'date': '2006-09-10', 'time': '23:34'},
        {'zone': 'Tongan time', 'date': '2006-09-11', 'time': '00:34'}]
    assert claims[DEATH_2]['attested_on'] == '2012-03-18' and claims[PROCLAIM_2]['attested_on'] == '2012-03-19'
    for cid in ('to_crown_gtv_taumafa_kava_held_20080730', 'to_crown_gtv_taumafa_kava_declaration_20080730'):
        assert claims[cid]['attested_on'] == '2008-07-30', cid
    assert claims['to_crown_gtv_coronation_day_address_20080801']['attested_on'] == '2008-08-01'
    assert claims['to_crown_tupou_vi_coronation_20150704_nrbt']['attested_on'] == '2015-07-04'
    for cid in ('to_crown_regent_tupouto_a_2004_pleading', 'to_crown_tupou_iv_burial_reported_20060920',
                'to_crown_crown_prince_tupouto_a_lavaka_2006', 'to_crown_gtv_investiture_address_wednesday',
                'to_crown_const1988_form_tupou_iv_c31', 'to_crown_gtv_prince_regent_numerous_occasions'):
        assert 'attested_on' not in claims[cid] and 'period' not in claims[cid], cid


class TongaCrownTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.raw = (research.ROOT / research.RESEARCH / 'tonga.json').read_text(encoding='utf-8')
        cls.packet = json.loads(cls.raw)
        cls.sources = {s['id']: s for s in cls.packet['sources']}
        cls.claims = {c['id']: c for s in cls.packet['sources'] for c in s['claims']}
        cls.claim_source = {c['id']: s['id'] for s in cls.packet['sources'] for c in s['claims']}
        cls.entries = {e['id']: e for category in ('organizations', 'institutions') for e in cls.packet[category]}
        cls.roles = {r['id']: r for e in cls.entries.values() for r in e['roles']}
        cls.crown = cls.entries['to_crown']
        cls.king = cls.roles['to_king']
        cls.holders = {h['name']: h for h in cls.king['holder_claims'] if isinstance(h, dict)}
        cls.extracts = {
            sid: json.loads((research.ROOT / cls.sources[sid]['snapshot']['path']).read_text(encoding='utf-8'))
            for sid in NEW_SOURCES
        }
        cls.report = (research.ROOT / REPORT).read_text(encoding='utf-8')
        cls.new_claims = {c['id'] for sid in NEW_SOURCES for c in cls.sources[sid]['claims']}
        cls.new_claims |= set().union(*REUSED_CLAIMS.values())

    def validate(self, packet=None):
        return research.validate(packet or self.packet, research.ROOT, {'Tonga'}, {'Tonga': set()})

    def section(self, heading):
        parts = self.report.split(f'\n## {heading}', 1)
        self.assertEqual(len(parts), 2, heading)
        return parts[1].split('\n## ', 1)[0]

    def test_new_records_are_bounded_reuse_ids_and_every_claim_is_cited(self):
        ids = self.validate()
        self.assertLessEqual(NEW_SOURCES, set(ids['sources']))
        self.assertEqual(len(self.new_claims), 44)
        self.assertEqual(len(NEW_SOURCES), 27)
        for sid, expected in REUSED_CLAIMS.items():
            self.assertLessEqual(expected, {c['id'] for c in self.sources[sid]['claims']})
            self.assertNotIn('snapshot', self.sources[sid])
        # Every new claim is cited by the Crown entry, and no other entry cites one.
        self.assertLessEqual(self.new_claims, set(self.crown['claim_ids']))
        for eid, entry in self.entries.items():
            if eid != 'to_crown':
                self.assertFalse(self.new_claims & set(entry['claim_ids']), eid)
        # No new organization, institution or role; the existing Crown entry and King role are reused.
        self.assertEqual(len(ids['entries']), 9)
        self.assertEqual({e['id'] for e in self.packet['institutions']},
                         {'to_crown', 'to_prime_minister', 'to_cabinet', 'to_privy_council', 'to_legislative_assembly'})
        self.assertEqual([r['id'] for r in self.crown['roles']], ['to_king'])
        self.assertEqual([r['id'] for r in self.roles.values() if r['kind'] == 'head_of_state'], ['to_king'])
        self.assertFalse(any('regent' in r['id'] or 'regent' in r['title'].lower() for r in self.roles.values()))
        observations = re.findall(r'^### (TO-CROWN-\d\d)\b', self.report, re.M)
        self.assertEqual(observations, [f'TO-CROWN-{n:02d}' for n in range(1, 9)])
        # The reused IPU and constitution records are not duplicated.
        for fragment, sid in (('2317_08', 'to_ipu_2008'), ('2317_10', 'to_ipu_2010')):
            self.assertEqual([s['id'] for s in self.packet['sources'] if fragment in s['url']], [sid])
        self.assertEqual(sum('ConstitutionofTonga.pdf' in s['url'] for s in self.packet['sources']), 1)
        self.assertEqual(sum('ActofConstitutionofTonga_1.pdf' in s['url'] for s in self.packet['sources']), 1)

    def test_accession_coronation_installation_and_death_never_collapse(self):
        crown_invariants(self.packet)
        # Each reign boundary is a death, and the devolution stated on it; nothing else.
        self.assertEqual(self.holders[TUPOU_IV]['until'], self.claims[DEATH_1]['attested_on'])
        self.assertEqual(self.holders[GEORGE]['from'], self.claims[DEATH_1]['attested_on'])
        self.assertEqual(self.holders[GEORGE]['until'], self.claims[DEATH_2]['attested_on'])
        self.assertEqual(self.holders[TUPOU_VI]['from'], self.claims[DEATH_2]['attested_on'])
        for cid in PROCLAIM_1 + (PROCLAIM_2,):
            self.assertRegex(self.claims[cid]['text'], r"devolves upon|passed to Crown Prince")
        self.assertIn('not used as the start', self.claims[PROCLAIM_1[0]]['uncertainty'])
        self.assertIn('not used as the start', self.claims[PROCLAIM_2]['uncertainty'])
        # Same Tongan day, separate events.
        self.assertNotEqual(DEATH_1, PROCLAIM_1[0])
        self.assertIn(DEATH_1, self.holders[GEORGE]['claim_ids'])
        self.assertIn(PROCLAIM_1[0], self.holders[GEORGE]['claim_ids'])
        self.assertIn('as a separate event', self.holders[GEORGE]['note'])
        # Installation, coronation and oath are distinct dates and never a start.
        dates = {
            'accession': self.holders[GEORGE]['from'],
            'proclamation': self.claims[PROCLAIM_1[0]]['attested_on'],
            'installation': self.claims['to_crown_gtv_taumafa_kava_held_20080730']['attested_on'],
            'coronation': self.claims['to_crown_gtv_coronation_day_address_20080801']['attested_on'],
            'death': self.claims[DEATH_2]['attested_on'],
        }
        self.assertEqual(dates, {'accession': '2006-09-11', 'proclamation': '2006-09-11', 'installation': '2008-07-30',
                                 'coronation': '2008-08-01', 'death': '2012-03-18'})
        self.assertEqual(len({dates['installation'], dates['coronation'], dates['accession'], dates['death']}), 4)
        declaration = self.claims['to_crown_gtv_taumafa_kava_declaration_20080730']
        self.assertIn('from that moment His Majesty became King', declaration['text'])
        self.assertIn('not the constitutional accession', declaration['uncertainty'])
        self.assertIn('not used as a start date', declaration['uncertainty'])
        for cid in OATH:
            self.assertRegex(self.claims[cid]['text'], r'(?i)oath')
        self.assertIn('no retrieved source dates the oath', self.claims[OATH[0]]['uncertainty'])
        self.assertNotIn('attested_on', self.claims[OATH[0]])
        self.assertIn('Tupou VI', self.claims['to_crown_tupou_vi_coronation_20150704_nrbt']['text'])
        self.assertIn('not the start of the reign', self.claims['to_crown_tupou_vi_coronation_20150704_nrbt']['uncertainty'])
        self.assertIn('not the start of the reign', self.claims['to_crown_gtv_coronation_scheduled_20080801']['uncertainty'])
        # The installation venue is kept as printed in both records.
        self.assertIn("Mala'e Pangai", self.claims[INSTALLATION[0]]['text'])
        self.assertIn('Pangai Lahi', self.claims[INSTALLATION[1]]['text'])
        # Funerals and mourning are separate from deaths and from reigns.
        self.assertEqual(self.claims['to_crown_tupou_iv_mourning_20060911']['period'], {'from': '2006-09-11', 'through': '2006-10-17'})
        self.assertEqual(self.claims['to_crown_gtv_mourning_declared_20120319']['period'], {'from': '2012-03-19', 'through': '2012-06-19'})
        self.assertEqual(self.claims['to_crown_tupou_iv_state_funeral_holiday_20060919']['attested_on'], '2006-09-19')

    def test_both_calendar_dates_of_tupou_iv_death_are_kept(self):
        death, un = self.claims[DEATH_1], self.claims[DEATH_1_UN]
        self.assertIn("'at 11:34 pm on 10th September 2006 (New Zealand time), 12:34 am on 11th September 2006, (Tongan time)'",
                      death['text'])
        self.assertEqual(self.claim_source[DEATH_1], 'to_pmo_20060911_death_notice')
        self.assertEqual(un['attested_on'], '2006-09-10')
        self.assertIn('Corroboration only', un['uncertainty'])
        self.assertIn('New Zealand calendar date', un['uncertainty'])
        self.assertIn('00:34 on 11 September', un['uncertainty'])
        self.assertIn('23:34 on 10 September, New Zealand time', self.holders[TUPOU_IV]['note'])
        # George Tupou V's death keeps both printed times on one Tongan date.
        self.assertEqual(self.claims[DEATH_2]['printed_times'], [
            {'zone': 'Hong Kong, China time', 'date': '2012-03-18', 'time': '15:58'},
            {'zone': 'Tonga time', 'date': '2012-03-18', 'time': '20:58'}])
        for claim in self.claims.values():
            for printed in claim.get('printed_times', []):
                self.assertLessEqual(printed['date'], research.CUTOFF)
                self.assertRegex(printed['time'], r'^\d\d:\d\d$')

    def test_regencies_are_observations_not_reigns(self):
        for cid in REGENCY:
            self.assertIn(cid, self.crown['claim_ids'])
            self.assertNotIn(cid, self.king['claim_ids'])
        for role in self.roles.values():
            for entry in role['holder_claims']:
                ids = [entry] if isinstance(entry, str) else entry['claim_ids']
                self.assertFalse(set(ids) & set(REGENCY), role['id'])
                if isinstance(entry, dict):
                    for name in ('Pilolevu', 'Regent', "Tupouto'a", 'Lavaka'):
                        self.assertNotIn(name, entry['name'])
        for cid in ('to_crown_prince_regent_tupouto_a_20060313', 'to_crown_princess_regent_speech_20060601',
                    'to_crown_princess_regent_opening_20060601_court'):
            self.assertIn('not a reign', self.claims[cid]['uncertainty'])
        self.assertEqual(self.claims['to_crown_prince_regent_tupouto_a_20060313']['attested_on'], '2006-03-13')
        self.assertEqual(self.claims['to_crown_princess_regent_opening_20060601_court']['attested_on'], '2006-06-01')
        pleading = self.claims['to_crown_regent_tupouto_a_2004_pleading']
        self.assertIn("a party's allegation", pleading['uncertainty'].lower())
        self.assertIn('on or after', pleading['uncertainty'])
        self.assertNotIn('2004-07-01', self.raw)
        assent = self.claims['to_crown_assent_tupouto_a_20060630']
        self.assertIn('not accepted as a regency observation', assent['uncertainty'])
        # Clause 43 is quoted, not paraphrased as a nobles' ballot.
        clause = self.claims['to_crown_const2020_prince_regent_c42_43']
        self.assertIn("'but the representatives of the people shall have no voice in such election'", clause['text'])
        self.assertNotIn("nobles", clause['text'])
        older = self.claims['to_crown_const1988_prince_regent_c42_43']
        self.assertIn('worded identically', older['text'])
        self.assertNotIn('in force for', older['text'])
        self.assertIn('to_crown_princess_regent_opening_20060601_court', older['uncertainty'])
        self.assertIn('[2006] Tonga LR', self.claims['to_crown_princess_regent_opening_20060601_court']['locator'])
        self.assertIn('142', self.claims['to_crown_princess_regent_opening_20060601_court']['locator'])
        coverage = [u for u in self.crown['coverage']['unresolved'] if u.startswith('TO-CROWN-02')]
        self.assertEqual(len(coverage), 1)
        self.assertIn('not as reigns or to_king holders', coverage[0])

    def test_holders_state_only_sourced_boundaries_and_tupou_vi_has_no_end(self):
        self.assertEqual(self.king['holder_claims'][0], 'to_tupou_vi_2025')
        self.assertEqual(len(self.king['holder_claims']), 4)
        expected_sources = {
            TUPOU_IV: ['to_act_income_tax_amend_1990', 'to_act_constitution_amend_1990', 'to_pmo_20060911_death_notice',
                       'to_pmo_20060911_proclamation'],
            GEORGE: ['to_pmo_20060911_death_notice', 'to_pmo_20060911_proclamation', 'to_pmo_20060911_proclamation_to',
                     'to_gazette_ext_8_2012', 'to_gazette_ext_9_2012'],
            TUPOU_VI: ['to_gazette_ext_8_2012', 'to_gazette_ext_9_2012'],
        }
        for name, sources in expected_sources.items():
            self.assertEqual(self.holders[name]['sources'], sources)
            for cid in self.holders[name]['claim_ids']:
                self.assertIn(self.claim_source[cid], sources)
        self.assertIsNone(self.holders[TUPOU_VI]['until'])
        self.assertIn('No end is inferred', self.holders[TUPOU_VI]['uncertainty'])
        self.assertIn('to_tupou_vi_2025', self.holders[TUPOU_VI]['uncertainty'])
        self.assertIsNone(self.holders[TUPOU_IV]['from'])
        self.assertIn('1965', self.holders[TUPOU_IV]['uncertainty'])
        self.assertIn("'Ulukalala Lavaka Ata", self.holders[TUPOU_VI]['note'])
        self.assertIn('Siaosi Tupou V', self.holders[GEORGE]['note'])
        self.assertIn('never a start date', self.king['scope_note'])
        # No other role gains a from/until from this packet.
        for role_id, role in self.roles.items():
            if role_id != 'to_king':
                for entry in role['holder_claims']:
                    if isinstance(entry, dict):
                        self.assertFalse(set(entry['claim_ids']) & self.new_claims, role_id)

    def test_relative_dates_stay_unconverted(self):
        for cid, word in (('to_crown_tupou_iv_burial_reported_20060920', "'yesterday'"),
                          ('to_crown_crown_prince_tupouto_a_lavaka_2006', "'yesterday'"),
                          ('to_crown_gtv_investiture_address_wednesday', "'On Wednesday this week'")):
            self.assertIn(word, self.claims[cid]['text'])
            self.assertRegex(self.claims[cid]['uncertainty'], r'^No structured date')
        self.assertNotIn('2006-09-27', self.raw)
        self.assertEqual(self.claims['to_crown_gtv_profile_proclaimed_sept2006']['period'],
                         {'from': '2006-09-01', 'through': '2006-09-30'})
        self.assertNotIn('attested_on', self.claims['to_crown_gtv_profile_proclaimed_sept2006'])
        # Publication, notice and instrument dates are metadata, kept apart from the events they carry.
        holiday = self.sources['to_pmo_20060914_funeral_holiday']
        self.assertEqual((holiday['published_date'], holiday['document_date']), ('2006-09-14', '2006-09-13'))
        self.assertEqual(self.sources['to_pmo_2008_pm_luncheon_address']['published_date'], '2008-08-01')
        self.assertIn("'From the office of the Prime Minister,Nuku'alofa, Tomnga, 1 August 2008'",
                      self.claims['to_crown_gtv_coronation_day_address_20080801']['text'])
        self.assertNotIn('published_date', self.sources['to_pmo_2008_coronation_programme'])
        for sid in ('to_act_income_tax_amend_1990', 'to_act_constitution_amend_1990', 'to_act_income_tax_amend_2006',
                    'to_act_public_enterprises_amend_2006', 'to_un_a61_pv1', 'to_iha_2008_tonga_visit',
                    'to_iha_2015_tonga_visit'):
            self.assertNotIn('published_date', self.sources[sid])
            self.assertIn('document_date', self.sources[sid])
        c31 = self.claims['to_crown_const1988_form_tupou_iv_c31']
        self.assertIn('Act 12 of 1990', c31['text'])
        self.assertIn('not a 1988 text', c31['uncertainty'])
        # The 2015 coronation carries its evidence tier.
        self.assertIn('non-ceremonial Tongan public body plus foreign corroboration',
                      self.claims['to_crown_tupou_vi_coronation_20150704_nrbt']['uncertainty'])

    def test_extracts_match_packet_claims_and_record_original_responses(self):
        for sid in NEW_SOURCES:
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual(extract['format'], 'spheres-c01-derived-factual-table/v1')
            self.assertEqual(extract['source_url'], source['url'])
            self.assertEqual(extract['claims'], source['claims'])
            self.assertEqual(extract['scope_note'], source['scope_note'])
            self.assertEqual(extract['access_method'], source['access_method'])
            self.assertEqual(extract['published_date'], source.get('published_date'))
            self.assertEqual(extract.get('document_date'), source.get('document_date'))
            self.assertEqual((extract['accessed_date'], source['accessed_date']), ('2026-09-21', '2026-09-21'))
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
            self.assertIn('pdf_pages_one_based', extract['visual_review'])
            self.assertTrue(source['source_type'])
            if source.get('published_date'):
                self.assertLessEqual(source['published_date'], research.CUTOFF)
        for sid, stamp in ARCHIVED.items():
            source, extract = self.sources[sid], self.extracts[sid]
            url = urlsplit(source['url'])
            self.assertEqual(url.hostname, 'web.archive.org')
            self.assertTrue(url.path.startswith(f'/web/{stamp}id_/http://www.pmo.gov.to'), sid)
            self.assertEqual(urlsplit(source['original_url']).hostname, 'www.pmo.gov.to')
            self.assertEqual(extract['original_url'], source['original_url'])
            self.assertEqual(extract['archive_capture_utc'].replace('-', '').replace(':', '').replace('T', '').rstrip('Z'), stamp)
            self.assertIn('Raw Internet Archive capture', extract['provenance_note'])
            self.assertEqual(extract['visual_review']['pdf_pages_one_based'], [])
        for sid in ('to_pmo_20060911_death_notice', 'to_pmo_20060911_proclamation_to', 'to_pmo_20080730_taumafa_kava'):
            self.assertIn('a second download for this packet matched', self.extracts[sid]['provenance_note'])
        self.assertEqual({urlsplit(self.sources[s]['url']).hostname for s in NEW_SOURCES - set(ARCHIVED)},
                         {'ago.gov.to', 'www.reservebank.to', 'documents.un.org', 'www.kunaicho.go.jp'})
        self.assertEqual(self.extracts['to_tlr_2006']['visual_review']['pdf_pages_one_based'], [166, 170])
        self.assertEqual(self.extracts['to_nrbt_ar_2015']['visual_review']['pdf_pages_one_based'], [30])

    def test_secondary_and_superseded_leads_stay_out_of_the_packet(self):
        lowered = self.raw.lower()
        for marker in LEAD_MARKERS:
            self.assertNotIn(marker, lowered, marker)
        for source in self.packet['sources']:
            self.assertNotIn('state.gov', source['url'])
        leads, added = self.section('Leads not imported'), self.section('Sources added')
        for marker in ('2001-2009.state.gov', 'special-national-holiday', 'annual_report_2009_eng.pdf',
                       'CriminalOffencesAmendmentAct2005.pdf', 'Matangi Tonga', 'Wikipedia'):
            self.assertIn(marker, leads, marker)
            self.assertNotIn(marker, added, marker)

    def test_packet_formatting_is_preserved(self):
        data = (research.ROOT / research.RESEARCH / 'tonga.json').read_bytes()
        self.assertNotIn(b'\r', data)
        self.assertEqual(data.decode('utf-8'), json.dumps(self.packet, indent=2, ensure_ascii=False) + '\n')
        self.assertIn('トンガご訪問', self.raw)

    def test_mutations_are_rejected(self):
        def mutated(change):
            packet = copy.deepcopy(self.packet)
            change(packet)
            return packet

        def source(packet, sid):
            return next(s for s in packet['sources'] if s['id'] == sid)

        def claim(packet, cid):
            return next(c for s in packet['sources'] for c in s['claims'] if c['id'] == cid)

        def king(packet):
            return next(r for e in packet['institutions'] for r in e['roles'] if r['id'] == 'to_king')

        def holder(packet, name):
            return next(h for h in king(packet)['holder_claims'] if isinstance(h, dict) and h['name'] == name)

        validator_cases = [
            (lambda p: source(p, 'to_pmo_20060911_death_notice')['snapshot'].update(sha256='0' * 64), 'checksum mismatch'),
            (lambda p: source(p, 'to_gazette_ext_9_2012')['snapshot'].update(bytes=1), 'checksum mismatch'),
            (lambda p: source(p, 'to_nrbt_ar_2015')['snapshot'].update(path=REPORT.as_posix()), 'escapes'),
            (lambda p: holder(p, TUPOU_VI).update(until='2026-09-08'), 'exceeds cutoff'),
            (lambda p: claim(p, DEATH_1).update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: holder(p, GEORGE).update(until='2006-09-10'), 'Reversed historical interval'),
            (lambda p: claim(p, 'to_crown_tupou_iv_mourning_20060911')['period'].update(through='2006-09-10'),
             'Reversed historical interval'),
            (lambda p: holder(p, TUPOU_VI).update(claim_ids=[DEATH_1]), 'cited source'),
            (lambda p: king(p)['claim_ids'].append('to_crown_does_not_exist'), 'Unknown'),
        ]
        for change, message in validator_cases:
            with self.subTest(message=message), self.assertRaisesRegex(ValueError, message):
                self.validate(mutated(change))
        invariant_cases = [
            ('coronation as start', lambda p: holder(p, GEORGE).update({'from': '2008-08-01'})),
            ('installation as start', lambda p: holder(p, GEORGE).update({'from': '2008-07-30'})),
            ('proclamation as start', lambda p: holder(p, TUPOU_VI).update({'from': '2012-03-19'})),
            ('NZ date as end', lambda p: holder(p, TUPOU_IV).update(until='2006-09-10')),
            ('inferred end for Tupou VI', lambda p: holder(p, TUPOU_VI).update(until='2025-12-18')),
            ('regent as a reign', lambda p: king(p)['holder_claims'].append({
                'name': "Princess Salote Mafile'o Pilolevu Tuita", 'attested_on': '2006-06-01', 'from': None,
                'until': None, 'sources': ['to_pmo_20060601_throne_speech'],
                'claim_ids': ['to_crown_princess_regent_speech_20060601']})),
            ('regency claim on the King role', lambda p: king(p)['claim_ids'].append('to_crown_prince_regent_tupouto_a_20060313')),
            ('coronation cited as a boundary', lambda p: holder(p, TUPOU_VI)['claim_ids'].append(
                'to_crown_tupou_vi_coronation_20150704_nrbt')),
            ('death collapsed into proclamation', lambda p: claim(p, PROCLAIM_2).update(attested_on='2012-03-18')),
            ('NZ date as the death date', lambda p: claim(p, DEATH_1).update(attested_on='2006-09-10')),
            ('printed time lost', lambda p: claim(p, DEATH_1).pop('printed_times')),
            ('relative date converted', lambda p: claim(p, 'to_crown_tupou_iv_burial_reported_20060920').update(
                attested_on='2006-09-19')),
            ('pleading given a month', lambda p: claim(p, 'to_crown_regent_tupouto_a_2004_pleading').update(
                period={'from': '2004-07-01', 'through': '2004-07-31'})),
            ('regent role added', lambda p: next(e for e in p['institutions'] if e['id'] == 'to_crown')['roles'].append(
                {'id': 'to_prince_regent', 'title': 'Prince Regent', 'kind': 'other', 'sources': ['to_tlr_2006'],
                 'claim_ids': ['to_crown_princess_regent_opening_20060601_court'], 'holder_claims': []})),
        ]
        crown_invariants(self.packet)
        for label, change in invariant_cases:
            with self.subTest(label=label), self.assertRaises((AssertionError, KeyError)):
                crown_invariants(mutated(change))

    def test_report_and_handoff_are_ready_for_review_stacked_and_close_nothing(self):
        self.assertIn('ready_for_review', self.report)
        table = self.section('Outcome')
        for number in range(1, 9):
            row, = [line for line in table.splitlines() if line.startswith(f'| TO-CROWN-{number:02d} ')]
            self.assertIn('**Accepted', row)
        row, = [line for line in table.splitlines() if line.startswith('| TO-CROWN-02 ')]
        self.assertIn('observations only', row)
        defects = self.section('Checker defects')
        rows = [line for line in defects.splitlines() if re.match(r'\| D\d+ ', line)]
        self.assertEqual([re.match(r'\| (D\d+) ', r).group(1) for r in rows], [f'D{n}' for n in range(1, 12)])
        for row in rows:
            self.assertRegex(row, r'\*\*(Applied|Applied as corrected disclosure)\*\*')
        self.assertIn('Major', rows[0])
        self.assertIn('0da095e75feaa3de5568a652f22f89ae79e974e82de7104e44afc04d508f920a', self.report)
        self.assertIn('af6bcefa59a7e96e520ddaacd5215715e966cb7b613181f777edb64a1864e825', self.report)
        for marker in ('C01', 'C06', 'S23', 'WC1', 'CP1'):
            self.assertNotRegex(self.report, rf'\b{marker}\b[^.\n]*\bis (now )?complete\b')
        notes = self.section('Integration notes')
        self.assertIn('CLAUDE-C01-04', notes)
        self.assertIn('62690e61', notes)
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        for text in ('ready_for_review', 'tonga-crown-07.md', 'claude/c01-tonga-07', 'CLAUDE-C01-04', '62690e61',
                     '21 September 2026 instruction', 'pending Codex acceptance', 'test_tonga_crown_c01_07.py'):
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
