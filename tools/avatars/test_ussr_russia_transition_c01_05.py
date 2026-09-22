"""CLAUDE-C01-05: the 1991 USSR/RSFSR executive transition keeps every dated act separate.

Adoption, entry into force, approval, voting, result approval, oath, signature,
ratification, renaming and cessation are separate claims. No end date is
inferred, no USSR institution is mapped to Russia, and secondary or non-official
texts stay leads.
"""
import copy
import hashlib
import json
import re
import unittest
from urllib.parse import urlsplit

import campaign_research as research


RU_SOURCES = {
    'ru_rsfsr_law_1098i_19910424', 'ru_garf_exhibit_law_1098i', 'ru_rsfsr_res_1099i_19910424',
    'ru_rsfsr_res_1324i_19910522', 'ru_rsfsr_res_1325i_19910522', 'ru_rsfsr_law_1326i_19910524',
    'ru_garf_cec_result_19910619', 'ru_rsfsr_law_1494i_19910627', 'ru_prlib_inauguration_stenogram_19910710',
    'ru_rsfsr_res_1595i_19910710', 'ru_rsfsr_res_1596i_19910710', 'ru_rsfsr_law_2094i_19911225',
    'ru_garf_exhibit_law_2094i',
}
SU_SOURCES = {
    'su_un_a46_771_minsk_19911208', 'su_garf_exhibit_belovezha_copy', 'su_rsfsr_res_2014i_19911212',
    'su_garf_exhibit_res_2014i', 'su_un_a47_60_almaata_19911221', 'su_nara_bush_gorbachev_telcon_19911225',
    'su_bush_address_cis_19911225',
}
# Original responses (not checked in): bytes and SHA-256 recorded by the dossier, the check and this packet.
# None means the response is not reproducible and no hash is asserted.
RESPONSES = {
    'ru_rsfsr_law_1098i_19910424': (31645, 'e49f34e8e2513b631205696fd495452f272871fd300df05db5ea67b8c2e7eb86'),
    'ru_garf_exhibit_law_1098i': (18664, '9ead61c87155ad91637c39bb479fcb9dc50244a92748d05ef7d65bb627d55298'),
    'ru_rsfsr_res_1099i_19910424': (23820, '5442288fee517dc41c53178cbe8319220daa74471e978aba26bb963ac1aea832'),
    'ru_rsfsr_res_1324i_19910522': (23707, '145bd8bd920247811a87a81aca6c490d01586f59586b39098f4ea21c08496a0b'),
    'ru_rsfsr_res_1325i_19910522': (25056, '3f4310ff4f539167c7ab4a01903b43b3e67199bef91072526d75eab69a553efe'),
    'ru_rsfsr_law_1326i_19910524': (43600, '1bf0fdfc84f92fe922e2a7dabbaeb9ed294804e36cfb8163694c414c2cc4f2e4'),
    'ru_garf_cec_result_19910619': (15700, '54000d3ca315063827147ddd5112ac8f2bb153d5d133ffc0d9ebb7c98019683e'),
    'ru_rsfsr_law_1494i_19910627': (25981, 'c296f80617ba6afe8de1fc834d4729dd64cb71c8002582ea0a79b260d549e56a'),
    'ru_prlib_inauguration_stenogram_19910710': (59392, None),
    'ru_rsfsr_res_1595i_19910710': (24555, '105a436c00ffb7b0e5527d1f273e8a2cc06c02aa1f5505c3b8e705e853a02e84'),
    'ru_rsfsr_res_1596i_19910710': (24498, '056f17a400f86373c0c4964fe78ac02645b1bced61085ef5703d516bda8e013b'),
    'ru_rsfsr_law_2094i_19911225': (20875, '18db383ad8a42245f7bed952490206145fa8a788f1ef282f17badac37bf49935'),
    'ru_garf_exhibit_law_2094i': (12807, '57573abb325a763f854b43737be95f6c9f6e46129d48747b0219e18ddab1abc5'),
    'su_un_a46_771_minsk_19911208': (389492, '3a7726bd6cbb0203a9821f29cebb0393490c5e153b7db9381b8f552ecd8ccf93'),
    'su_garf_exhibit_belovezha_copy': (36632, 'a5599981e5268a2b9db3d5a28e4b69cd151d58759f734c3b1167c457d52a1547'),
    'su_rsfsr_res_2014i_19911212': (24748, '5ec47cbe36709480a778dc097ddc7dfd0249391804ce2992e4afdcc6062f37c0'),
    'su_garf_exhibit_res_2014i': (13596, 'aab8fb84c6ebef13036950c8e708718a6b79b2b2480aa8bbb602f640f90d867a'),
    'su_un_a47_60_almaata_19911221': (148826, 'cade387d0dc6fc3fabbd95eeded60c04570d519572eec318389eade57e4b347a'),
    'su_nara_bush_gorbachev_telcon_19911225': (259720, '28e8ee5d8e7ca7be061c6c0bc1206dccb1cfef72f7cd096e7b7acc3f89024d3f'),
    'su_bush_address_cis_19911225': (28644, '8cf7d76e158349e9563f52240f5db80e7b2467c2b432a16c67e3f489d882d3d6'),
}
FACSIMILES = {
    'ru_garf_exhibit_law_1098i': [(28081, '3b0322a144ba0ef03ec78cf5c205b2e22b20c1fe22cc41c687f3f0dca13e89b9'),
                                  (23640, '03839340274da90caf121fde6e1803e2ed4e0e74d5b287cc1919b50fb614273c')],
    'ru_garf_cec_result_19910619': [(98762, '9052337ceab3818e95968197989853581e6b3aaf21560455ced9ad8c2c62b6c2'),
                                    (133816, '0a6d5e086e016decf3cd8167ce320e3628b10290f5698f8d6d8f6db4596a5229'),
                                    (114508, '188d308d5631fdb54d5873014d3f3a9b7a91b0f9c97cd337b21173c5a09e328e')],
    'ru_garf_exhibit_law_2094i': [(137825, '5ef34cca2624a582c976844ac714780033e8299d3cf5d1597fe28a3ce8160830')],
    'su_garf_exhibit_belovezha_copy': [(166429, '8f57200075fe4bf0a5991e4e38254edee941a57d7572e5bd6d016b75f61f1e71'),
                                       (145917, '1b129b351c00e6540394fedf912df35a934e276d14dbbc74919e2e63e9892810'),
                                       (178616, '425e690e2d5084ca7f62c54cff39f1ad99c4ce9c71dead6945d966b107451429')],
    'su_garf_exhibit_res_2014i': [(155943, '94792a1df535d101d4db52af2f3b88c92fb4e6f13daf501d82d850c5d7b7d65b')],
}
# Each distinct act and its own date. No two of these may be merged into one claim or one field.
DATED = {
    'ru_rsfsr_president_office_created_19910424': '1991-04-24',            # adoption
    'ru_rsfsr_president_law_in_force_on_publication_19910424': '1991-04-24',  # resolution ordering entry into force from publication
    'ru_rsfsr_president_law_congress_approval_19910522': '1991-05-22',     # Congress approval
    'ru_rsfsr_candidates_registered_19910522': '1991-05-22',
    'ru_rsfsr_constitution_ch13_1_19910524': '1991-05-24',                 # constitutional amendment
    'ru_cec_election_voting_19910612': '1991-06-12',                       # voting
    'ru_cec_resolution_20_25_19910619': '1991-06-19',                      # result approval
    'ru_cec_communication_results_19910619': '1991-06-19',
    'ru_cec_yeltsin_rutskoi_elected_19910619': '1991-06-19',
    'ru_rsfsr_inauguration_law_rules_19910627': '1991-06-27',
    'ru_steno_oath_19910710': '1991-07-10',                                # oath
    'ru_res_1595i_yeltsin_release_19910710': '1991-07-10',
    'ru_res_1596i_rutskoi_release_19910710': '1991-07-10',
    'su_minsk_agreement_art11_14_19911208': '1991-12-08',                  # signature
    'su_rsfsr_minsk_ratification_19911212': '1991-12-12',                  # ratification
    'su_almaata_protocol_19911221': '1991-12-21',
    'su_almaata_declaration_19911221': '1991-12-21',
    'ru_law_2094i_rename_19911225': '1991-12-25',                          # renaming (adoption = entry into force)
    'su_telcon_resignation_decrees_19911225': '1991-12-25',                # announced cessation
    'su_bush_address_resign_19911225': '1991-12-25',
}
# Secondary, non-official or renamed identifiers that must not appear as packet claims.
EXCLUDED_CLAIMS = {
    'su_gorbachev_address_cessation_19911225', 'su_ved52_decree_up3162_19911225', 'su_ved52_declaration_142n_19911226',
    'su_ved52_order_141n_19911226', 'su_ved52_issue_notes_19911225', 'su_bush_address_recognition_19911225',
    'su_almaata_un_membership_decision_19911221', 'su_almaata_minutes_armed_forces_19911221',
    'su_un_a46_771_transmittal_19911212', 'su_un_a47_60_transmittal_19911227', 'su_steno_gorbachev_lukyanov_titles_19910710',
    'su_steno_gorbachev_confirms_oath_19910710', 'ru_cec_communication_results_19910612',
}
EXCLUDED_HOSTS = {'vedomosti.sssr.su', 'sten.sr.vs.sssr.su', 'www.gorby.ru', 'base.garant.ru', 'www.consultant.ru',
                  'www.presidency.ucsb.edu'}
REPORT = research.RESEARCH / 'ussr-russia-transition-1991-05.md'
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-05.md'


def load(name):
    return json.loads((research.ROOT / research.RESEARCH / name).read_text(encoding='utf-8'))


def claims_of(packet):
    return {c['id']: c for s in packet['sources'] for c in s['claims']}


def holders(packet):
    for group in ('organizations', 'institutions'):
        for entry in packet[group]:
            for role in entry['roles']:
                for holder in role['holder_claims']:
                    if isinstance(holder, dict):
                        yield entry, role, holder


def inference_problems(ussr, russia):
    """Guards this packet adds on top of the validator: no inferred ends, no collapsed start, no mapping."""
    problems = []
    for packet in (ussr, russia):
        for entry, role, holder in holders(packet):
            if entry['id'] in {'su_presidency', 'ru_rsfsr_presidency'} and holder.get('until') is not None:
                problems.append(f"inferred end on {role['id']}: {holder['name']}")
    presidency = next(e for e in russia['institutions'] if e['id'] == 'ru_rsfsr_presidency')
    if presidency['lifecycle']['from'] is not None:
        problems.append('adoption day used as the RSFSR presidency start')
    if presidency['lifecycle']['until'] is not None:
        problems.append('inferred end of the RSFSR presidency')
    su = next(e for e in ussr['institutions'] if e['id'] == 'su_presidency')
    if su['lifecycle']['until'] is not None:
        problems.append('inferred end of the USSR Presidency')
    for packet, nation in ((ussr, 'USSR'), (russia, 'Russia')):
        for group in ('organizations', 'institutions'):
            for entry in packet[group]:
                jurisdiction = entry.get('jurisdiction')
                if isinstance(jurisdiction, dict):
                    if jurisdiction.get('automatic_successor_mapping') is not False or jurisdiction.get('nation') != nation:
                        problems.append(f"successor mapping on {entry['id']}")
                if entry['represented_party_ids'] or entry.get('reconciled_organization_id'):
                    problems.append(f"identity mapping on {entry['id']}")
    for role in presidency['roles']:
        cited = set(role['claim_ids']) | {c for h in role['holder_claims'] if isinstance(h, dict) for c in h['claim_ids']}
        if any(cid.startswith('su_') for cid in cited):
            problems.append(f"USSR claim cited by {role['id']}")
    return problems


class UssrRussiaTransitionTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.raw = {name: (research.ROOT / research.RESEARCH / name).read_text(encoding='utf-8')
                   for name in ('ussr.json', 'russia.json')}
        cls.ussr, cls.russia = json.loads(cls.raw['ussr.json']), json.loads(cls.raw['russia.json'])
        cls.sources = {s['id']: s for p in (cls.ussr, cls.russia) for s in p['sources']}
        cls.claims = {**claims_of(cls.ussr), **claims_of(cls.russia)}
        cls.owner = {c['id']: s['id'] for p in (cls.ussr, cls.russia) for s in p['sources'] for c in s['claims']}
        cls.presidency = next(e for e in cls.russia['institutions'] if e['id'] == 'ru_rsfsr_presidency')
        cls.su_presidency = next(e for e in cls.ussr['institutions'] if e['id'] == 'su_presidency')
        cls.roles = {r['id']: r for p in (cls.ussr, cls.russia) for g in ('organizations', 'institutions')
                     for e in p[g] for r in e['roles']}
        cls.extracts = {sid: json.loads((research.ROOT / cls.sources[sid]['snapshot']['path']).read_text(encoding='utf-8'))
                        for sid in RU_SOURCES | SU_SOURCES}
        cls.report = (research.ROOT / REPORT).read_text(encoding='utf-8')

    def validate(self, packet, nation):
        return research.validate(packet, research.ROOT, {'USSR', 'Russia'}, {'USSR': set(), 'Russia': set()})

    def test_bounded_records_land_in_the_right_packet(self):
        self.validate(self.ussr, 'USSR')
        self.validate(self.russia, 'Russia')
        self.assertLessEqual(RU_SOURCES, {s['id'] for s in self.russia['sources']})
        self.assertLessEqual(SU_SOURCES, {s['id'] for s in self.ussr['sources']})
        self.assertEqual(sum(len(self.sources[s]['claims']) for s in RU_SOURCES), 24)
        self.assertEqual(sum(len(self.sources[s]['claims']) for s in SU_SOURCES), 13)
        self.assertEqual(self.owner['su_rsfsr_minsk_ratification_19911212'], 'su_rsfsr_res_2014i_19911212')
        # Every new claim is cited by an entry, a role or a holder, so the atlas can show it.
        cited = set()
        for packet in (self.ussr, self.russia):
            for group in ('organizations', 'institutions'):
                for entry in packet[group]:
                    cited |= set(entry['claim_ids'])
                    for role in entry['roles']:
                        cited |= set(role['claim_ids'])
        new = {c['id'] for sid in RU_SOURCES | SU_SOURCES for c in self.sources[sid]['claims']}
        self.assertEqual(new - cited, set())
        self.assertEqual(re.findall(r'^### (SURU-TR91-\d\d)\b', self.report, re.M),
                         [f'SURU-TR91-{n:02d}' for n in range(1, 9)])

    def test_every_dated_act_keeps_its_own_claim_and_date(self):
        for cid, day in DATED.items():
            self.assertEqual(self.claims[cid]['attested_on'], day, cid)
            self.assertNotIn('period', self.claims[cid])
        # Adoption is not entry into force, and the entry-into-force day is not invented.
        in_force = self.claims['ru_rsfsr_president_law_in_force_on_publication_19910424']
        self.assertIn('from the moment of its publication', in_force['text'])
        self.assertIn('not the day the law entered into force', in_force['uncertainty'])
        self.assertNotIn('1991-04-27', self.raw['russia.json'])
        self.assertIn('27 April 1991', in_force['uncertainty'])
        # Voting and result approval are separate claims of one source; the renamed ID carries the approval date.
        self.assertEqual({self.owner['ru_cec_election_voting_19910612'], self.owner['ru_cec_communication_results_19910619']},
                         {'ru_garf_cec_result_19910619'})
        self.assertIn('12 June 1991', self.claims['ru_cec_election_voting_19910612']['text'])
        self.assertIn('45,552,041', self.claims['ru_cec_communication_results_19910619']['text'])
        # Signature is not ratification; ratification is not the Alma-Ata in-force rule.
        self.assertIn('Signature date only', self.claims['su_minsk_agreement_art11_14_19911208']['uncertainty'])
        self.assertIn('point 2', self.claims['su_minsk_agreement_art11_14_19911208']['uncertainty'])
        self.assertIn('separate from the 8 December signature', self.claims['su_rsfsr_minsk_ratification_19911212']['uncertainty'])
        self.assertIn('from the moment of its ratification', self.claims['su_almaata_protocol_19911221']['text'])
        # The rename's adoption and entry into force coincide only because point 5 says so.
        self.assertIn('enters into force on the day of its adoption', self.claims['ru_law_2094i_rename_19911225']['text'])
        # Lifecycle keeps each date in words and uses none of them as the start.
        lifecycle = self.presidency['lifecycle']
        self.assertEqual((lifecycle['from'], lifecycle['until'], lifecycle['precision']), (None, None, 'unknown'))
        for phrase in ('24 April 1991', 'from the moment of publication', '22 May 1991', '24 May 1991', 'not entry into force'):
            self.assertIn(phrase, lifecycle['note'])

    def test_holders_start_only_where_sources_state_it_and_never_end(self):
        yeltsin, = self.roles['ru_rsfsr_president']['holder_claims']
        rutskoi, = self.roles['ru_rsfsr_vice_president']['holder_claims']
        for holder, release in ((yeltsin, 'ru_res_1595i_yeltsin_release_19910710'), (rutskoi, 'ru_res_1596i_rutskoi_release_19910710')):
            self.assertEqual((holder['from'], holder['until'], holder['attested_on']), ('1991-07-10', None, None))
            self.assertIn('ru_rsfsr_inauguration_law_rules_19910627', holder['claim_ids'])
            self.assertIn(release, holder['claim_ids'])
            for cid in ('ru_cec_election_voting_19910612', 'ru_cec_yeltsin_rutskoi_elected_19910619', 'ru_cec_resolution_20_25_19910619'):
                self.assertNotIn(cid, holder['claim_ids'])
        self.assertIn('ru_steno_oath_19910710', yeltsin['claim_ids'])
        self.assertIn('five-year rule is not used', yeltsin['uncertainty'])
        # Gorbachev gains a dated observation, not an end: the announcement is a claim.
        letter, dec25 = self.roles['su_president']['holder_claims']
        self.assertEqual(letter['attested_on'], '1990-03-20')
        self.assertEqual((dec25['name'], dec25['attested_on'], dec25['from'], dec25['until']),
                         ('Mikhail Gorbachev', '1991-12-25', None, None))
        self.assertEqual(dec25['claim_ids'], ['su_telcon_gorbachev_title_19911225'])
        self.assertIn('no until is set', dec25['uncertainty'])
        for cid in ('su_telcon_resignation_decrees_19911225', 'su_bush_address_resign_19911225', 'su_telcon_gorbachev_intent_19911225'):
            self.assertIn(cid, self.roles['su_president']['claim_ids'])
        self.assertIsNone(self.su_presidency['lifecycle']['until'])
        self.assertEqual(self.su_presidency['lifecycle']['from'], '1990-03-14')
        self.assertEqual(inference_problems(self.ussr, self.russia), [])

    def test_no_automatic_ussr_to_russia_mapping(self):
        jurisdiction = self.presidency['jurisdiction']
        self.assertEqual((jurisdiction['nation'], jurisdiction['automatic_successor_mapping']), ('Russia', False))
        self.assertIn('not the USSR Presidency', jurisdiction['note'])
        self.assertEqual((self.presidency['represented_party_ids'], self.presidency['reconciled_organization_id']), ([], None))
        self.assertEqual({e['id'] for e in self.ussr['institutions']},
                         {'su_presidency', 'su_congress_peoples_deputies', 'su_supreme_soviet'})
        self.assertEqual([e['id'] for e in self.russia['institutions'] if not e['id'].startswith('ru_duma_faction_')],
                         ['ru_rsfsr_presidency'])
        self.assertEqual([r['id'] for r in self.presidency['roles']], ['ru_rsfsr_president', 'ru_rsfsr_vice_president'])
        self.assertEqual({r['kind'] for r in self.presidency['roles']}, {'institutional_office'})
        # Russia cites only Russia sources; the rename supports the filing, the signed title does not.
        for sid in self.presidency['sources']:
            self.assertTrue(sid.startswith('ru_'), sid)
        signature = 'ru_law_2094i_published_signature_19911225'
        self.assertIn(signature, self.presidency['claim_ids'])
        for role in self.presidency['roles']:
            self.assertNotIn(signature, role['claim_ids'])
            self.assertNotIn('ru_garf_law_2094i_original_19911225', role['claim_ids'])
        self.assertIn('Президент РСФСР', self.claims['ru_garf_law_2094i_original_19911225']['text'])
        self.assertIn('Президент Российской Федерации', self.claims[signature]['text'])
        # Nuclear authority, UN membership and US recognition do not become succession.
        self.assertIn('not an institutional succession', self.claims['su_telcon_resignation_decrees_19911225']['uncertainty'])
        self.assertIn('not a domestic institutional mapping', self.sources['su_un_a47_60_almaata_19911221']['scope_note'])
        self.assertIn('not a succession', self.sources['su_bush_address_cis_19911225']['scope_note'])
        self.assertIn('present tense', self.claims['su_almaata_declaration_19911221']['uncertainty'])
        mapped = copy.deepcopy(self.ussr)
        mapped['institutions'][0]['represented_party_ids'] = ['Russia/example']
        with self.assertRaisesRegex(ValueError, 'foreign represented party mapping'):
            research.validate(mapped, research.ROOT, {'USSR', 'Russia'}, {'USSR': set(), 'Russia': {'Russia/example'}})

    def test_secondary_and_non_official_texts_are_absent(self):
        for name, raw in self.raw.items():
            for host in EXCLUDED_HOSTS:
                self.assertNotIn(host, raw, name)
        for cid in EXCLUDED_CLAIMS:
            self.assertNotIn(cid, self.claims)
            self.assertNotIn(f'"{cid}"', self.raw['ussr.json'] + self.raw['russia.json'])
        for sid in RU_SOURCES | SU_SOURCES:
            self.assertNotIn('1000dokumente', urlsplit(self.sources[sid]['url']).hostname)
            self.assertTrue(self.sources[sid]['source_type'].startswith('primary_'), sid)
        # The leads remain visible in the report, outside the sources table.
        leads = self.report.split('## Leads not imported', 1)[1].split('\n## ', 1)[0]
        sources_added = self.report.split('## Sources added', 1)[1].split('\n## ', 1)[0]
        for host in ('1000dokumente.de', 'vedomosti.sssr.su', 'sten.sr.vs.sssr.su'):
            self.assertIn(host, leads)
            self.assertNotIn(host, sources_added)

    def test_extracts_match_claims_and_recorded_hashes(self):
        for sid in RU_SOURCES | SU_SOURCES:
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual(extract['source_url'], source['url'])
            self.assertEqual(extract['claims'], source['claims'])
            self.assertEqual(extract['scope_note'], source['scope_note'])
            self.assertEqual((extract['accessed_date'], source['accessed_date']), ('2026-09-21', '2026-09-21'))
            self.assertEqual(extract['published_date'], source['published_date'])
            self.assertFalse(extract['source_response_checked_in'])
            self.assertEqual(source['snapshot']['kind'], 'derived_factual_extract')
            data = (research.ROOT / source['snapshot']['path']).read_bytes()
            self.assertEqual((len(data), hashlib.sha256(data).hexdigest()),
                             (source['snapshot']['bytes'], source['snapshot']['sha256']))
            self.assertNotIn(b'\r', data)
            self.assertEqual((extract['source_response_bytes'], extract['source_response_sha256']), RESPONSES[sid], sid)
            self.assertNotEqual(source['snapshot']['sha256'], extract['source_response_sha256'])
            self.assertIn('no source artwork or portrait copied', extract['rights_note'])
            self.assertIn('No portrait permission or likeness approval', extract['rights_note'])
            self.assertEqual(urlsplit(source['url']).scheme, 'https')
            expected_format = ('spheres-c01-derived-factual-table/v1' if sid.startswith('ru_')
                               else 'spheres-primary-source-factual-extract/v1')
            self.assertEqual(extract['format'], expected_format)
        for sid, rows in FACSIMILES.items():
            self.assertEqual([(r['bytes'], r['sha256']) for r in self.extracts[sid]['facsimile_responses']], rows)
        # The non-reproducible stenogram page asserts no hash; its images do.
        steno = self.extracts['ru_prlib_inauguration_stenogram_19910710']
        self.assertIn('volatile Drupal tokens', steno['page_response_note'])
        self.assertEqual([r['image'] for r in steno['page_image_responses']], [1, 5, 6, 7, 8, 9, 14, 15])
        for row in steno['page_image_responses']:
            self.assertTrue(row['url'].startswith('https://content.prlib.ru/fcgi-bin/iipsrv.fcgi?FIF='))
            self.assertTrue(row['url'].endswith('.tiff&WID=1000&CVT=jpeg'))
            self.assertRegex(row['sha256'], r'^[0-9a-f]{64}$')
        # IPS responses were read over HTTP; the packet keeps the same path on HTTPS and says so.
        for sid in (s for s in RU_SOURCES | SU_SOURCES if self.sources[s]['access_method'].startswith('official_portal')):
            extract = self.extracts[sid]
            self.assertEqual(extract['source_response_url'], self.sources[sid]['url'].replace('https://', 'http://', 1))
            self.assertIn('port 443', extract['provenance_note'])
            self.assertIn('pravo.gov.ru/proxy/ips/?list_itself=', extract['portal_card_response']['url'])
        self.assertIn('No. 17, art. 513', self.sources['ru_rsfsr_res_1099i_19910424']['publisher'])
        self.assertEqual(self.claims['su_bush_address_resign_19911225']['locator'], 'Paragraphs 5-6 and closing note')
        self.assertEqual(self.extracts['su_un_a47_60_almaata_19911221']['language_issue_responses'][0]['sha256'],
                         '6c164a2135743ad5494c169537b0258d724dc70d7d95d06980063394f601cf15')
        cec = self.extracts['ru_garf_cec_result_19910619']['facsimile_responses']
        self.assertEqual([(r['handwritten_folio'], r['exhibit_caption_leaf']) for r in cec],
                         [('5', 'L. 6'), ('6', 'L. 6 ob.'), ('7', 'L. 7')])
        for extract in self.extracts.values():
            for row in extract.get('facsimile_responses', []):
                self.assertNotRegex(row['url'], r'/web/\d{4}id_/', 'year-wildcard capture left unresolved')

    def test_mutations_are_rejected(self):
        def mutated(packet, change):
            packet = copy.deepcopy(packet)
            change(packet)
            return packet

        def source(packet, sid):
            return next(s for s in packet['sources'] if s['id'] == sid)

        def holder(packet, role_id, index=0):
            role = next(r for g in ('organizations', 'institutions') for e in packet[g] for r in e['roles'] if r['id'] == role_id)
            return [h for h in role['holder_claims'] if isinstance(h, dict)][index]

        cases = [
            (self.russia, lambda p: source(p, 'ru_prlib_inauguration_stenogram_19910710')['snapshot'].update(sha256='0' * 64), 'checksum mismatch'),
            (self.ussr, lambda p: source(p, 'su_un_a47_60_almaata_19911221')['snapshot'].update(bytes=1), 'checksum mismatch'),
            (self.russia, lambda p: holder(p, 'ru_rsfsr_president').update(until='1991-06-19'), 'Reversed historical interval'),
            (self.russia, lambda p: holder(p, 'ru_rsfsr_vice_president').update({'from': '2026-09-08'}), 'exceeds cutoff'),
            (self.ussr, lambda p: source(p, 'su_rsfsr_res_2014i_19911212')['claims'][0].update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (self.russia, lambda p: holder(p, 'ru_rsfsr_vice_president').update(claim_ids=['ru_steno_oath_19910710']), 'cited source'),
            (self.ussr, lambda p: source(p, 'su_garf_exhibit_res_2014i').update(url='http://projects.rusarchives.ru/statehood/10-03-postanovlenie-ratifikaciya-sng.shtml'), 'Invalid public source URL'),
        ]
        for packet, change, message in cases:
            with self.subTest(message=message), self.assertRaisesRegex(ValueError, message):
                self.validate(mutated(packet, change), packet['nation'])
        # The packet's own guards catch an inferred end, a collapsed start and a USSR-to-Russia mapping.
        guard_cases = [
            (lambda u, r: holder(u, 'su_president', 1).update(until='1991-12-25'), 'inferred end on su_president'),
            (lambda u, r: next(e for e in r['institutions'] if e['id'] == 'ru_rsfsr_presidency')['lifecycle'].update({'from': '1991-04-24'}), 'adoption day'),
            (lambda u, r: next(e for e in u['institutions'] if e['id'] == 'su_presidency')['lifecycle'].update(until='1991-12-26'), 'USSR Presidency'),
            (lambda u, r: next(e for e in u['institutions'] if e['id'] == 'su_presidency')['jurisdiction'].update(automatic_successor_mapping=True), 'successor mapping'),
            (lambda u, r: next(e for e in r['institutions'] if e['id'] == 'ru_rsfsr_presidency')['roles'][0]['claim_ids'].append('su_telcon_resignation_decrees_19911225'), 'USSR claim cited'),
        ]
        for change, message in guard_cases:
            ussr, russia = copy.deepcopy(self.ussr), copy.deepcopy(self.russia)
            change(ussr, russia)
            with self.subTest(message=message):
                self.assertTrue(any(message in p for p in inference_problems(ussr, russia)), message)

    def test_report_and_handoff_are_ready_for_review_and_close_nothing(self):
        self.assertIn('ready_for_review', self.report)
        table = self.report.split('## Outcome', 1)[1].split('\n## ', 1)[0]
        rows = {rid: [line for line in table.splitlines() if line.startswith(f'| {rid} ')] for rid in
                (f'SURU-TR91-{n:02d}' for n in range(1, 9))}
        for rid in ('SURU-TR91-07', 'SURU-TR91-08'):
            self.assertIn('**Unresolved', rows[rid][0])
            self.assertNotIn('Accepted', rows[rid][0])
        for n in range(1, 7):
            self.assertIn('**Accepted', rows[f'SURU-TR91-{n:02d}'][0])
        self.assertIn('no `until`', rows['SURU-TR91-06'][0])
        for marker in ('C01', 'C06', 'S23', 'WC1', 'CP1'):
            self.assertNotRegex(self.report, rf'\b{marker}\b[^.\n]*\bis (now )?complete\b')
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        for phrase in ('ready_for_review', 'ussr-russia-transition-1991-05.md', 'Owner: Claude', 'pending Codex acceptance',
                       "21 September 2026 instruction"):
            self.assertIn(phrase, handoff)
        index = research.build()
        for nation in ('USSR', 'Russia'):
            country = next(p for p in index['countries'] if p['nation'] == nation)
            self.assertFalse(country['country_census_complete'])
            self.assertIsNone(country['unrepresented_organization_count'])
            self.assertEqual({w['status'] for w in index['work_orders'] if w['nation'] == nation}, {'open'})
        self.assertFalse(index['c01_complete'])


if __name__ == '__main__':
    unittest.main()
