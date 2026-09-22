"""CLAUDE-C01-03: Saudi kings and crown princes 1990-2026 keep death, pledge, selection and relief dates apart."""
import copy
from datetime import datetime, timedelta
import hashlib
import json
import re
import unittest
from urllib.parse import urlsplit

import campaign_research as research


NEW_SOURCES = [
    'sa_bush41_address_19900808',
    'sa_spa_fahd_death_20050801_ar',
    'sa_spa_fahd_death_20050801_en',
    'sa_spa_allegiance_20050801_ar',
    'sa_spa_allegiance_20050801_en',
    'sa_spa_fahd_bio_20050801',
    'sa_spa_sultan_death_20111022',
    'sa_spa_nayef_cp_20111027',
    'sa_spa_nayef_death_20120616',
    'sa_spa_salman_cp_20120618',
    'sa_spa_abdullah_death_20150123_en',
    'sa_spa_abdullah_death_20150123_ar',
    'sa_spa_order_a159_20150429',
    'sa_spa_order_a160_20150429',
    'sa_spa_pledge_call_20150429',
    'sa_spa_pledge_held_20150429',
    'sa_spa_order_a255_20170621_ar',
    'sa_spa_order_a255_20170621_en',
    'sa_spa_pledge_call_20170621',
    'sa_spa_mbn_pledge_20170621_ar',
    'sa_spa_mbn_pledge_20170621_en',
    'sa_spa_orders_20260813',
]
REVERIFIED = 'sa_spa_pm_2022'
# Every new claim with its own event or observation date. No two events share a claim.
EVENT_DATES = {
    'sa_fahd_king_obs_19900808': '1990-08-08',
    'sa_fahd_death_announced_20050801': '2005-08-01',
    'sa_abdullah_cp_obs_20050801': '2005-08-01',
    'sa_abdullah_family_pledge_20050801': '2005-08-01',
    'sa_sultan_cp_selected_20050801': '2005-08-01',
    'sa_citizen_pledge_scheduled_20050803': '2005-08-01',
    'sa_family_pledge_en_translation_20050801': '2005-08-01',
    'sa_fahd_mourned_26_6_1426': '2005-08-01',
    'sa_sultan_death_20111022': '2011-10-22',
    'sa_nayef_cp_order_a224': '2011-10-27',
    'sa_allegiance_law_a135_cited': '2011-10-27',
    'sa_nayef_death_20120616': '2012-06-16',
    'sa_salman_cp_order_a139': '2012-06-18',
    'sa_abdullah_death_20150123': '2015-01-23',
    'sa_salman_king_pledge_20150123': '2015-01-23',
    'sa_muqrin_cp_pledge_20150123': '2015-01-23',
    'sa_citizen_pledge_scheduled_20150123': '2015-01-23',
    'sa_abdullah_death_ar_dateline': '2015-01-23',
    'sa_muqrin_relieved_a159': '2015-04-29',
    'sa_mbn_cp_a159': '2015-04-29',
    'sa_mbs_deputy_cp_a160': '2015-04-29',
    'sa_pledge_call_20150429': '2015-04-29',
    'sa_pledge_held_20150429': '2015-04-29',
    'sa_mbn_relieved_20170621': '2017-06-21',
    'sa_mbs_cp_selected_20170621': '2017-06-21',
    'sa_allegiance_31_of_34_20170621': '2017-06-21',
    'sa_mbs_deputy_cp_date_discrepancy': '2017-06-21',
    'sa_orders_20170621_en_translation': '2017-06-21',
    'sa_pledge_call_20170621': '2017-06-21',
    'sa_mbn_pledge_to_mbs_20170621': '2017-06-21',
    'sa_mbn_pledge_en_translation_20170621': '2017-06-21',
    'sa_salman_king_obs_20260813': '2026-08-13',
    'sa_mbs_cp_pm_obs_20260813': '2026-08-13',
}
NEW_CLAIMS = set(EVENT_DATES)
# Claims that record a scheduled, called or held ceremony, a translation, a citation or a
# corroboration. They are dated claims but never holder observations.
NEVER_HOLDERS = {
    'sa_citizen_pledge_scheduled_20050803', 'sa_family_pledge_en_translation_20050801', 'sa_fahd_mourned_26_6_1426',
    'sa_allegiance_law_a135_cited', 'sa_citizen_pledge_scheduled_20150123', 'sa_abdullah_death_ar_dateline',
    'sa_mbs_deputy_cp_a160', 'sa_pledge_call_20150429', 'sa_pledge_held_20150429', 'sa_allegiance_31_of_34_20170621',
    'sa_mbs_deputy_cp_date_discrepancy', 'sa_orders_20170621_en_translation', 'sa_pledge_call_20170621',
    'sa_mbn_pledge_to_mbs_20170621', 'sa_mbn_pledge_en_translation_20170621',
}
# Death while styled in office, relief by royal order, or the holder's own accession to the throne.
END_CLAIMS = {
    'sa_abdullah_death_20150123': 'died at one o\'clock that morning',
    'sa_sultan_death_20111022': 'who died at dawn that day',
    'sa_nayef_death_20120616': 'who died that day',
    'sa_salman_king_pledge_20150123': 'received the pledge of allegiance as King',
    'sa_muqrin_relieved_a159': 'relieves him, at his request, of the crown princeship',
    'sa_mbn_relieved_20170621': 'relieves Prince Mohammed bin Nayef bin Abdulaziz of the crown princeship',
}
KING_HOLDERS = [
    ('Fahd bin Abdulaziz Al Saud', '1990-08-08', None, ['sa_fahd_king_obs_19900808', 'sa_fahd_death_announced_20050801']),
    ('Abdullah bin Abdulaziz Al Saud', '2005-08-01', '2015-01-23', ['sa_abdullah_family_pledge_20050801', 'sa_abdullah_death_20150123']),
    ('Salman bin Abdulaziz Al Saud', '2015-01-23', None, ['sa_salman_king_pledge_20150123']),
]
CROWN_PRINCE_HOLDERS = [
    ('Abdullah bin Abdulaziz Al Saud', '2005-08-01', None, ['sa_abdullah_cp_obs_20050801', 'sa_abdullah_family_pledge_20050801']),
    ('Sultan bin Abdulaziz Al Saud', '2005-08-01', '2011-10-22', ['sa_sultan_cp_selected_20050801', 'sa_sultan_death_20111022']),
    ('Nayef bin Abdulaziz Al Saud', '2011-10-27', '2012-06-16', ['sa_nayef_cp_order_a224', 'sa_nayef_death_20120616']),
    ('Salman bin Abdulaziz Al Saud', '2012-06-18', '2015-01-23', ['sa_salman_cp_order_a139', 'sa_salman_king_pledge_20150123']),
    ('Muqrin bin Abdulaziz Al Saud', '2015-01-23', '2015-04-29', ['sa_muqrin_cp_pledge_20150123', 'sa_muqrin_relieved_a159']),
    ('Mohammed bin Nayef bin Abdulaziz Al Saud', '2015-04-29', '2017-06-21', ['sa_mbn_cp_a159', 'sa_mbn_relieved_20170621']),
    ('Mohammed bin Salman bin Abdulaziz Al Saud', '2017-06-21', None, ['sa_mbs_cp_selected_20170621']),
]
# The existing 2022 claims, byte-identical to the base packet (reverified, not rewritten).
BASE_PM_CLAIMS = [
    ('sa_mbs_pm_appointment', 'King Salman appointed Crown Prince Mohammed bin Salman prime minister on 27 September 2022, expressly excepting Article 56 of the Basic Law and related Cabinet-law provisions.'),
    ('sa_king_cabinet_chair_exception', 'The same order reserves chairing of Cabinet sessions attended by the King to the King, and separately reconstitutes the Cabinet with Mohammed bin Salman as prime minister.'),
    ('sa_salman_king_observation', 'The release identifies Salman bin Abdulaziz Al Saud as King.'),
]
# Reproducible identity: SHA-256 of the UTF-8 newsDetails.content field, identical in every download.
ARTICLE_CONTENT = {
    'sa_spa_fahd_death_20050801_ar': 'cc562d7e5d0edd92efc31bd2707af98d0cba287a22aa140263ee4392084e5327',
    'sa_spa_fahd_death_20050801_en': 'd1354c5710bc1f9e60551d77e53ea4a91a7af0cb8b4516ea1adf391b3c31e976',
    'sa_spa_allegiance_20050801_ar': '7c029335a983b0601aff7ff1bb321b6b634a089d212246effeea4d083586eb67',
    'sa_spa_allegiance_20050801_en': '7a3a8e9e4a0298dfe9ea67e089ddbb96d09e50e64f7fb6b4acb5d8ec5897843b',
    'sa_spa_fahd_bio_20050801': 'f7434392d2e79e260d60b2da656559c5b1bb429b14deac750e73bff9003a6ccd',
    'sa_spa_sultan_death_20111022': '656ce67d44fdcd357dccf2a32ea490f86974649ba3b498fb22089869e82ad6f0',
    'sa_spa_nayef_cp_20111027': '01a8e5423e8a92db43ed7e5aa908255a4bb9af834bbaae03740048ee6458a0a2',
    'sa_spa_nayef_death_20120616': '5c4ef41015978b5da9ad9438a965497b4784299642f432d63582279371cde8d4',
    'sa_spa_salman_cp_20120618': '045a57f40d1b03c5c5e71f8b0caa83d8abe8597e00008b42461ab767b0d11347',
    'sa_spa_abdullah_death_20150123_en': '0d1f4e2c93a805e2b3ddef1c5012a04bca4be9838d837c1c014eda874584078c',
    'sa_spa_abdullah_death_20150123_ar': '152a9b7687a77404b81ebd2caa6a004bd6b56b170311d729d887ad60a27efb63',
    'sa_spa_order_a159_20150429': '460122e798164cd1eeba90f475ffdffd1caad8d84a9c536565b69c52d31fd0dc',
    'sa_spa_order_a160_20150429': '0bfc867331af9d8e6a4e9e06d72f9d0c1d0ab814cd5c632b673598b19eb0159e',
    'sa_spa_pledge_call_20150429': '66e124f9590eaf0b79d0ac5f4ba578cd94a2830605a3ebf22d2d90f2ceb054bc',
    'sa_spa_pledge_held_20150429': 'a560fe2cecdadbe4fdaa474942712f3f5c88a942f4614718e397ff635df82847',
    'sa_spa_order_a255_20170621_ar': 'b0161db294693e805fdddc7ecc0197b98299682becc4fde901aa9978b366c28e',
    'sa_spa_order_a255_20170621_en': '31ec07b1cdb1278e25e767b66a44b715db81154e804fa5e730b3a50d861d2e79',
    'sa_spa_pledge_call_20170621': 'fdb11dadeae08841c8f020d36867a90ca364774d3d083b45dc086d0a74fd7d51',
    'sa_spa_mbn_pledge_20170621_ar': '35a5c1757fbdbadb6e37a7c192a008323a9af65610dcb99ace9ec9f5aea176be',
    'sa_spa_mbn_pledge_20170621_en': 'd301cc5f85045d92043b4cb1a62336e81d24b2bd63aea6e4a12ad73d5eb02c5b',
    'sa_spa_orders_20260813': 'ddac38970a06e83ffc1399472b4fcea83f292af261b5bc12dd3629c574a902b6',
    REVERIFIED: 'fdf5daa9b68e30b4208c70c3396fac1df690d9f75db4f642d763b39884f7eaef',
}
# Bytes of the requested URL form; SPA response hashes are deliberately not asserted.
RESPONSE_BYTES = {
    'sa_spa_fahd_death_20050801_ar': 173207, 'sa_spa_fahd_death_20050801_en': 164267,
    'sa_spa_allegiance_20050801_ar': 172480, 'sa_spa_allegiance_20050801_en': 169616,
    'sa_spa_fahd_bio_20050801': 179929, 'sa_spa_sultan_death_20111022': 173837,
    'sa_spa_nayef_cp_20111027': 174548, 'sa_spa_nayef_death_20120616': 173562,
    'sa_spa_salman_cp_20120618': 175052, 'sa_spa_abdullah_death_20150123_en': 176591,
    'sa_spa_abdullah_death_20150123_ar': 181502, 'sa_spa_order_a159_20150429': 182370,
    'sa_spa_order_a160_20150429': 183113, 'sa_spa_pledge_call_20150429': 173829,
    'sa_spa_pledge_held_20150429': 171961, 'sa_spa_order_a255_20170621_ar': 203626,
    'sa_spa_order_a255_20170621_en': 173485, 'sa_spa_pledge_call_20170621': 171509,
    'sa_spa_mbn_pledge_20170621_ar': 170053, 'sa_spa_mbn_pledge_20170621_en': 168103,
    'sa_spa_orders_20260813': 166476, REVERIFIED: 182652,
}
BUSH_RESPONSE = (31889, '539f115582aea3de304d4557c61bb18169f829616c04d72b50d591c5f91ba1c3')
SECONDARY = ('aljazeera', 'npr.org', 'meed', 'arabnews', 'alriyadh', 'okaz', 'aawsat', 'abc.net', 'saudipedia',
             'nsarchive', 'govinfo', 'mof.gov.sa', 'my.gov.sa', 'wikipedia')
REPORT = research.RESEARCH / 'saudi-executive-chronology-03.md'
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-03.md'


class SaudiExecutiveChronologyTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.raw = (research.ROOT / research.RESEARCH / 'saudi-arabia.json').read_text(encoding='utf-8')
        cls.packet = json.loads(cls.raw)
        cls.sources = {s['id']: s for s in cls.packet['sources']}
        cls.claims = {c['id']: c for s in cls.packet['sources'] for c in s['claims']}
        cls.claim_source = {c['id']: s['id'] for s in cls.packet['sources'] for c in s['claims']}
        cls.entries = {e['id']: e for category in ('organizations', 'institutions') for e in cls.packet[category]}
        cls.roles = {r['id']: r for e in cls.entries.values() for r in e['roles']}
        cls.extracts = {
            sid: json.loads((research.ROOT / cls.sources[sid]['snapshot']['path']).read_text(encoding='utf-8'))
            for sid in NEW_SOURCES + [REVERIFIED]
        }
        cls.report = (research.ROOT / REPORT).read_text(encoding='utf-8')

    def validate(self, packet=None):
        return research.validate(packet or self.packet, research.ROOT, {'SaudiArabia'}, {'SaudiArabia': set()})

    def dict_holders(self, role_id):
        return [h for h in self.roles[role_id]['holder_claims'] if isinstance(h, dict)]

    def test_new_records_are_bounded_reuse_ids_and_every_claim_is_cited(self):
        ids = self.validate()
        self.assertEqual((len(ids['sources']), len(ids['claims']), len(ids['entries']), len(ids['roles'])), (32, 48, 12, 10))
        self.assertEqual([s['id'] for s in self.packet['sources'][-len(NEW_SOURCES):]], NEW_SOURCES)
        self.assertEqual({c['id'] for sid in NEW_SOURCES for c in self.sources[sid]['claims']}, NEW_CLAIMS)
        cited = {cid for row in list(self.entries.values()) + list(self.roles.values()) for cid in row['claim_ids']}
        self.assertLessEqual(NEW_CLAIMS, cited)
        self.assertEqual(set(self.roles), {'sa_king', 'sa_crown_prince', 'sa_pm', 'sa_cabinet_ministers', 'sa_shura_chair',
                                           'sa_shura_members', 'sa_succession_chair', 'sa_succession_secretary',
                                           'sa_succession_members', 'sa_municipal_members'})
        self.assertFalse([r for r in self.roles.values() if 'deputy' in r['title'].lower()])
        self.assertEqual([r['id'] for r in self.roles.values() if r['kind'] == 'head_of_government'], ['sa_pm'])
        # At most eight observations, each a report section.
        observations = re.findall(r'^### (SA-EXEC-\d\d)\b', self.report, re.M)
        self.assertEqual(observations, [f'SA-EXEC-{n:02d}' for n in range(1, 9)])
        # The 2022 source is reverified, not rewritten, and keeps its original holders first.
        self.assertEqual([(c['id'], c['text']) for c in self.sources[REVERIFIED]['claims']], BASE_PM_CLAIMS)
        self.assertEqual(self.roles['sa_king']['holder_claims'][0], 'sa_salman_king_observation')
        self.assertEqual(self.roles['sa_crown_prince']['holder_claims'][0], 'sa_mbs_pm_appointment')
        self.assertEqual(self.roles['sa_pm']['holder_claims'], ['sa_mbs_pm_appointment', 'sa_mbs_cp_pm_obs_20260813'])

    def test_death_pledge_selection_relief_and_publication_dates_never_collapse(self):
        for cid, day in EVENT_DATES.items():
            with self.subTest(cid=cid):
                claim = self.claims[cid]
                self.assertEqual(claim['attested_on'], day)
                self.assertNotIn('period', claim)
                self.assertLessEqual(day, research.CUTOFF)
        text = lambda cid: self.claims[cid]['text']
        doubt = lambda cid: self.claims[cid]['uncertainty']
        # 2005: announced death, family pledge, selection and scheduled citizens' pledge are four claims.
        self.assertIn('no day or hour of death', doubt('sa_fahd_death_announced_20050801'))
        self.assertIn('no separate date or time for the family pledge', doubt('sa_abdullah_family_pledge_20050801'))
        self.assertIn('Royal Court statement dated Riyadh 26 Jumada II 1426 / 1 August 2005 reports that', text('sa_abdullah_family_pledge_20050801'))
        self.assertIn('day after tomorrow, Wednesday', text('sa_citizen_pledge_scheduled_20050803'))
        self.assertIn('does not attest that the pledge took place', doubt('sa_citizen_pledge_scheduled_20050803'))
        self.assertIn('not the Royal Court statement', doubt('sa_fahd_mourned_26_6_1426'))
        # 2011: death, then a separate order five days later; publication after midnight is not the order date.
        self.assertLess(EVENT_DATES['sa_sultan_death_20111022'], EVENT_DATES['sa_nayef_cp_order_a224'])
        self.assertEqual(self.sources['sa_spa_nayef_cp_20111027']['published_date'], '2011-10-27')
        self.assertIn('00:51 Makkah time on 28 October 2011', doubt('sa_nayef_cp_order_a224'))
        self.assertIn('1432-12-01', doubt('sa_nayef_cp_order_a224'))
        self.assertIn('notified', text('sa_nayef_cp_order_a224'))
        # 2015 January: death, King's pledge, Crown Prince's pledge and scheduled citizens' pledge.
        self.assertIn("died at one o'clock that morning", text('sa_abdullah_death_20150123'))
        self.assertIn('Royal Order A/86', text('sa_muqrin_cp_pledge_20150123'))
        self.assertNotIn('Decree', text('sa_muqrin_cp_pledge_20150123'))
        self.assertIn("'Royal Decree'", doubt('sa_muqrin_cp_pledge_20150123'))
        self.assertIn('will begin', text('sa_citizen_pledge_scheduled_20150123'))
        self.assertIn('does not attest', doubt('sa_citizen_pledge_scheduled_20150123'))
        self.assertIn('22 January 2015', text('sa_abdullah_death_ar_dateline'))
        self.assertIn('02:34 Makkah time on 23 January', doubt('sa_abdullah_death_ar_dateline'))
        self.assertIn('filing artefact', doubt('sa_abdullah_death_ar_dateline'))
        self.assertEqual(self.sources['sa_spa_abdullah_death_20150123_ar']['published_date'], '2015-01-23')
        # 2015 April: relief, selection, deputy selection, call and the held ceremony stay five claims.
        self.assertIn('called pledge', doubt('sa_pledge_call_20150429').lower())
        self.assertIn('that evening', text('sa_pledge_held_20150429'))
        self.assertIn('kept separate from the orders', doubt('sa_pledge_held_20150429'))
        # 2017: the 10/6/1436 recital is an error in the order text; the 29 April 2015 date stands.
        self.assertIn('recorded as an error in the 2017 order text', text('sa_mbs_deputy_cp_date_discrepancy'))
        self.assertIn('not an English translation error', doubt('sa_mbs_deputy_cp_date_discrepancy'))
        self.assertIn('10/6/1436', text('sa_orders_20170621_en_translation'))
        self.assertIn('31 of 34', text('sa_allegiance_31_of_34_20170621'))
        self.assertIn('misprints the Hijri year as 1437', doubt('sa_pledge_call_20170621'))
        self.assertIn('today', text('sa_mbn_pledge_to_mbs_20170621'))
        self.assertFalse([cid for cid, c in self.claims.items() if str(c.get('attested_on', '')).startswith('2015-03')])
        # Source publication dates are separate metadata and stay inside the cutoff.
        for sid in NEW_SOURCES:
            self.assertLessEqual(self.sources[sid]['published_date'], research.CUTOFF)
        for sid in NEW_SOURCES[1:] + [REVERIFIED]:
            stamp = datetime.fromisoformat(self.extracts[sid]['portal_metadata']['published_at_utc'].replace('Z', '+00:00'))
            makkah_day = (stamp + timedelta(hours=3)).date().isoformat()
            published = self.sources[sid]['published_date']
            if sid == 'sa_spa_nayef_cp_20111027':
                # Printed dateline kept; the release went out after midnight Makkah time.
                self.assertEqual((published, makkah_day), ('2011-10-27', '2011-10-28'))
                self.assertIn('00:52 Makkah time on 28 October 2011', self.sources[sid]['scope_note'])
            else:
                self.assertEqual(published, makkah_day, sid)

    def test_holders_are_event_dated_with_stated_ends_only(self):
        king, crown = self.roles['sa_king']['holder_claims'], self.roles['sa_crown_prince']['holder_claims']
        self.assertEqual([h if isinstance(h, str) else h['name'] for h in king],
                         ['sa_salman_king_observation'] + [h[0] for h in KING_HOLDERS] + ['sa_salman_king_obs_20260813'])
        self.assertEqual([h if isinstance(h, str) else h['name'] for h in crown],
                         ['sa_mbs_pm_appointment'] + [h[0] for h in CROWN_PRINCE_HOLDERS] + ['sa_mbs_cp_pm_obs_20260813'])
        for role_id, expected in (('sa_king', KING_HOLDERS), ('sa_crown_prince', CROWN_PRINCE_HOLDERS)):
            holders = self.dict_holders(role_id)
            self.assertEqual([(h['name'], h['attested_on'], h['until'], h['claim_ids']) for h in holders], expected)
            for holder in holders:
                with self.subTest(role=role_id, name=holder['name']):
                    self.assertEqual(list(holder), ['name', 'attested_on', 'from', 'until', 'sources', 'claim_ids', 'note', 'uncertainty'])
                    self.assertIsNone(holder['from'])  # No source uses effective-date wording.
                    self.assertEqual(holder['attested_on'], self.claims[holder['claim_ids'][0]]['attested_on'])
                    self.assertEqual(holder['sources'], [self.claim_source[cid] for cid in holder['claim_ids']])
                    self.assertFalse(set(holder['claim_ids']) & NEVER_HOLDERS)
        # Selection, ceremony, translation and citation claims never become holders on any role.
        for role in self.roles.values():
            for entry in role['holder_claims']:
                ids = [entry] if isinstance(entry, str) else entry['claim_ids']
                self.assertFalse(set(ids) & NEVER_HOLDERS, role['id'])
        # Deputy Crown Prince stays out of the Crown Prince role entirely.
        self.assertNotIn('sa_mbs_deputy_cp_a160', self.roles['sa_crown_prince']['claim_ids'])
        self.assertIn('sa_mbs_deputy_cp_a160', self.entries['sa_crown']['claim_ids'])

    def test_term_ends_come_from_the_holders_own_death_relief_or_accession(self):
        for role_id in ('sa_king', 'sa_crown_prince'):
            holders = self.dict_holders(role_id)
            starts = {h['claim_ids'][0] for h in holders}
            for index, holder in enumerate(holders):
                with self.subTest(role=role_id, name=holder['name']):
                    if holder['until'] is None:
                        continue
                    end = holder['claim_ids'][-1]
                    self.assertIn(end, END_CLAIMS)
                    self.assertIn(END_CLAIMS[end], self.claims[end]['text'])
                    self.assertEqual(self.claims[end]['attested_on'], holder['until'])
                    self.assertLessEqual(holder['attested_on'], holder['until'])
                    # The end is never a successor's selection or pledge claim.
                    self.assertNotIn(end, starts - {holder['claim_ids'][0]})
                    if index + 1 < len(holders):
                        self.assertNotIn(holders[index + 1]['claim_ids'][0], holder['claim_ids'])
        by_name = lambda role_id, name: next(h for h in self.dict_holders(role_id) if h['name'] == name)
        # Fahd: a successor is reported on 1 August 2005, but no day of death is stated.
        fahd = by_name('sa_king', 'Fahd bin Abdulaziz Al Saud')
        self.assertIsNone(fahd['until'])
        self.assertIn('sa_fahd_death_announced_20050801', fahd['claim_ids'])
        self.assertIn('until is null rather than the announcement date', fahd['uncertainty'])
        self.assertIn('foreign record', fahd['uncertainty'])
        # Abdullah as Crown Prince: Sultan's selection that day does not end his office by inference.
        abdullah_cp = by_name('sa_crown_prince', 'Abdullah bin Abdulaziz Al Saud')
        self.assertIsNone(abdullah_cp['until'])
        self.assertIn('no separate date for the family pledge', abdullah_cp['uncertainty'])
        # Salman's crown princeship ends on his own accession, not on Muqrin's pledge.
        salman_cp = by_name('sa_crown_prince', 'Salman bin Abdulaziz Al Saud')
        self.assertEqual(salman_cp['claim_ids'][-1], 'sa_salman_king_pledge_20150123')
        self.assertNotIn('sa_muqrin_cp_pledge_20150123', salman_cp['claim_ids'])
        self.assertIn("not taken from Muqrin's pledge", salman_cp['uncertainty'])
        for role_id, name in (('sa_king', 'Salman bin Abdulaziz Al Saud'),
                              ('sa_crown_prince', 'Mohammed bin Salman bin Abdulaziz Al Saud')):
            current = by_name(role_id, name)
            self.assertIsNone(current['until'])
            self.assertIn('No end date is inferred', current['uncertainty'])
            self.assertIn('continuity to 7 September 2026 is not established', current['uncertainty'])
        # No until anywhere equals a later holder's start unless the holder's own end claim states it.
        for role in self.roles.values():
            for holder in role['holder_claims']:
                if isinstance(holder, dict) and holder.get('until'):
                    self.assertIn(holder['claim_ids'][-1], END_CLAIMS)

    def test_extracts_match_packet_claims_and_record_reproducible_identities(self):
        for sid in NEW_SOURCES + [REVERIFIED]:
            source, extract = self.sources[sid], self.extracts[sid]
            with self.subTest(source=sid):
                self.assertEqual(extract['format'], 'spheres-c01-derived-factual-table/v1')
                self.assertEqual(extract['source_url'], source['url'])
                self.assertEqual(extract['claims'], source['claims'])
                self.assertEqual(extract['scope_note'], source['scope_note'])
                self.assertEqual(extract['published_date'], source['published_date'])
                self.assertEqual(extract['access_method'], source['access_method'])
                self.assertEqual((extract['accessed_date'], source['accessed_date']), ('2026-09-21', '2026-09-21'))
                self.assertFalse(extract['source_response_checked_in'])
                self.assertEqual(source['snapshot']['kind'], 'derived_factual_extract')
                self.assertTrue(source['snapshot']['path'].startswith('docs/campaign-certification/C01/research/sources/saudi-arabia-'))
                data = (research.ROOT / source['snapshot']['path']).read_bytes()
                self.assertEqual((len(data), hashlib.sha256(data).hexdigest()),
                                 (source['snapshot']['bytes'], source['snapshot']['sha256']))
                self.assertTrue(data.endswith(b'}\n') and b'\r' not in data)
                self.assertNotEqual(source['snapshot']['sha256'], extract['source_response_sha256'])
                self.assertIn('No open license, portrait permission or likeness approval', extract['rights_note'])
                self.assertIn('derived factual extract, not the original response', extract['provenance_note'])
                self.assertEqual(set(extract['visual_review']), {'pdf_pages_one_based', 'facsimile_pages_one_based', 'method'})
                self.assertIn(urlsplit(source['url']).hostname, {'www.spa.gov.sa', 'www.bush41library.gov'})
        bush = self.extracts['sa_bush41_address_19900808']
        self.assertEqual((bush['source_response_bytes'], bush['source_response_sha256']), BUSH_RESPONSE)
        for sid, content in ARTICLE_CONTENT.items():
            extract = self.extracts[sid]
            with self.subTest(source=sid):
                self.assertEqual(urlsplit(self.sources[sid]['url']).hostname, 'www.spa.gov.sa')
                self.assertIsNone(extract['source_response_sha256'])
                self.assertEqual(extract['source_response_bytes'], RESPONSE_BYTES[sid])
                self.assertEqual(extract['article_content_sha256'], content)
                self.assertIn('views_count', extract['provenance_note'])
                self.assertIn('server_id', extract['provenance_note'])
                rows = extract['response_observations']
                self.assertEqual((rows[0]['url_form'], rows[0]['bytes']), (self.sources[sid]['url'], RESPONSE_BYTES[sid]))
                # Repeated downloads differ in their response hash, which is why none is asserted.
                self.assertEqual(len({r['sha256'] for r in rows}), len(rows))
                self.assertTrue(all(re.fullmatch(r'[0-9a-f]{64}', r['sha256']) for r in rows))
                uuid = extract['portal_metadata']['uuid']
                self.assertTrue(self.sources[sid]['url'].endswith('/' + uuid) or
                                urlsplit(self.sources[sid]['url']).path.strip('/').isdigit())
        self.assertEqual(set(ARTICLE_CONTENT), set(NEW_SOURCES[1:] + [REVERIFIED]))

    def test_secondary_and_unread_leads_stay_out_of_the_packet(self):
        lowered = self.raw.lower()
        for token in SECONDARY:
            self.assertNotIn(token, lowered)
        for source in self.packet['sources']:
            host = urlsplit(source['url']).hostname
            self.assertFalse(any(token in host for token in SECONDARY), host)
        leads = self.report.split('## Leads not imported', 1)[1].split('\n## ', 1)[0]
        added = self.report.split('## Sources added', 1)[1].split('\n## ', 1)[0]
        for token in ('saudipedia', 'nsarchive', 'govinfo', 'mof.gov.sa', 'Al Jazeera'):
            self.assertIn(token.lower(), leads.lower())
            self.assertNotIn(token.lower(), added.lower())
        # The Arabic 2026 release and the GPO print edition are leads, not sources.
        self.assertIn('N2653088', leads)
        self.assertNotIn('spa.gov.sa/N2653088', self.raw)

    def test_mutations_are_rejected(self):
        def mutated(change):
            packet = copy.deepcopy(self.packet)
            change(packet)
            return packet

        def role(packet, role_id):
            return next(r for e in packet['institutions'] for r in e['roles'] if r['id'] == role_id)

        def holder(packet, role_id, name):
            return next(h for h in role(packet, role_id)['holder_claims'] if isinstance(h, dict) and h['name'] == name)

        def source(packet, sid):
            return next(s for s in packet['sources'] if s['id'] == sid)

        mbn = 'Mohammed bin Nayef bin Abdulaziz Al Saud'
        cases = [
            (lambda p: source(p, 'sa_spa_order_a255_20170621_ar')['snapshot'].update(sha256='0' * 64), 'checksum mismatch'),
            (lambda p: source(p, 'sa_bush41_address_19900808')['snapshot'].update(bytes=1), 'checksum mismatch'),
            (lambda p: source(p, REVERIFIED)['snapshot'].update(sha256='f' * 64), 'checksum mismatch'),
            (lambda p: holder(p, 'sa_crown_prince', 'Mohammed bin Salman bin Abdulaziz Al Saud').update(until='2026-09-08'), 'exceeds cutoff'),
            (lambda p: holder(p, 'sa_king', 'Salman bin Abdulaziz Al Saud').update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: source(p, 'sa_spa_orders_20260813')['claims'][0].update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: holder(p, 'sa_crown_prince', mbn).update({'from': '2018-01-01'}), 'Reversed historical interval'),
            (lambda p: holder(p, 'sa_crown_prince', mbn).update(claim_ids=['sa_mbn_cp_a159', 'sa_pledge_held_20150429']), 'cited source'),
            (lambda p: source(p, 'sa_spa_pledge_held_20150429')['snapshot'].update(path='Cargo.toml'), 'escapes'),
        ]
        for change, message in cases:
            with self.subTest(message=message), self.assertRaisesRegex(ValueError, message):
                self.validate(mutated(change))
        # A later research access date is metadata, not a historical observation.
        self.validate(mutated(lambda p: source(p, 'sa_spa_orders_20260813').update(accessed_date='2026-09-30')))

    def test_report_and_handoff_are_ready_for_review_and_close_nothing(self):
        self.assertIn('ready_for_review', self.report)
        table = self.report.split('## Outcome', 1)[1].split('\n## ', 1)[0]
        for n in range(1, 9):
            row, = [line for line in table.splitlines() if line.startswith(f'| SA-EXEC-{n:02d} ')]
            self.assertIn('**Accepted', row)
        for marker in ('C01', 'C06', 'S23', 'WC1', 'CP1'):
            self.assertNotRegex(self.report, rf'\b{marker}\b[^.\n]*\bis (now )?complete\b')
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        for phrase in ('ready_for_review', 'saudi-executive-chronology-03.md', 'pending Codex acceptance',
                       "21 September 2026 instruction for six further autonomous sessions"):
            self.assertIn(phrase, handoff)
        index = research.build()
        country = next(p for p in index['countries'] if p['nation'] == 'SaudiArabia')
        self.assertEqual((country['source_claims'], country['role_observations']), (48, 10))
        self.assertFalse(country['country_census_complete'])
        self.assertIsNone(country['unrepresented_organization_count'])
        batches = [row for row in index['work_orders'] if row['nation'] == 'SaudiArabia']
        self.assertEqual([b['status'] for b in batches], ['open', 'open'])


if __name__ == '__main__':
    unittest.main()
