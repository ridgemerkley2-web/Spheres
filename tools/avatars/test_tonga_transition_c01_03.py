"""CLAUDE-C01-03: Tonga's 2024-2025 transition keeps resignation, acting service, selection, appointment,
publication, Cabinet effect, letters and oath apart, and ends a term only where a source states it."""
import copy
import hashlib
import json
import re
import unittest
from urllib.parse import urlsplit

import campaign_research as research


ASSEMBLY_HTML = {
    'to_assembly_vonc_notice_20241125': 57203,
    'to_assembly_resignation_news_20241209': 56637,
    'to_assembly_pm_nominations_open_20241210': 55980,
    'to_assembly_first_nomination_20241220': 56828,
    'to_assembly_two_nominations_20241223': 56838,
    'to_assembly_eke_elected_news_20241224': 60909,
    'to_assembly_oath_sitting_notice_20250129': 54109,
    'to_assembly_oath_news_20250203': 70501,
}
MINUTES = {
    # source id: (bytes, sha256, rendered pages, text-layer-only pages)
    'to_assembly_minutes_48_20241209': (404595, '9e9cf1dd8d5833fc4b2f9b1c4ccf3df01cf7471686a5e7768f00aa42023c27d5',
                                        [2, 3, 40, 41, 42, 43, 46, 48, 49, 50], [47]),
    'to_assembly_minutes_pm_election_20241224': (412540, 'd798e1237d660f3cb4a5b207fa8ca17b6ccf18d8fee45065c9327dda85443347',
                                                 [3, 7, 8, 9, 11, 12, 13, 52, 53, 55], [19, 47]),
    'to_assembly_minutes_01_20250131': (410718, '3ef9f082c60a3bd580ba7e66f399d19ed25c675b724eeb1039e84dff38cd320c',
                                        [2, 7, 8, 9, 17], [18, 23]),
}
PMO = {
    # source id: (response url, bytes, sha256); the response differs from the post URL except for the live page.
    'to_pmo_caretaker_announcement_20250106': (
        'https://pmo.gov.to/wp-content/uploads/2025/01/1.jpg', 137870,
        'db9c580da9fe40f160f89b658f1830dda59726145a28f57d307d994ed9d7d250'),
    'to_pmo_eke_appointment_20250122': (
        'https://web.archive.org/web/20250123155100id_/https://pmo.gov.to/his-majesty-king-tupou-vi-appoints-hon-dr-aisake-valu-eke-as-prime-minister-of-tonga/',
        66444, 'ed2e113eef3a7e54aee258e55f65558dca88c715036ef957a61fb4592690354a'),
    'to_pmo_pif_joint_release_20250124': (
        'https://pmo.gov.to/pacific-islands-forum-secretary-general-meets-new-prime-minister-of-tonga/', 71411,
        'be3dff707279e0943b06a9c599c6813775976175d446af5fc638bf0713c30d76'),
    'to_pmo_eke_cabinet_20250128': (
        'https://pmo.gov.to/wp-content/uploads/2025/01/Media-Release_28_01_2025_Prime-Minister-Hon.Dr_.Aisake-Valu-Eke-announces-new-Cabinet-Ministers.pdf',
        396595, 'ad766e4ddce9bd50442776e2a23237cf40c57722a6d959c671f89a4a318ae134'),
}
OTHER = {
    'to_gazette_gse22_20241210': (273587, 'f5f830a993c20954a34f1fe806618d03cf81c07239f2b563414429585ec6fa5b'),
    # Shared with CLAUDE-C01-04, whose accepted fetch (164,916 bytes) the merged extract keeps; this packet's
    # own fetch returned 165,854 bytes, which the extract's provenance note records.
    'to_ipu_2025': (164916, None),
}
NEW_SOURCES = set(ASSEMBLY_HTML) | set(MINUTES) | set(PMO) | set(OTHER)
RESIGNATION = ('to_sovaleni_resignation_statement_20241209', 'to_palace_acceptance_letter_20241209')
PROCEDURE = {
    'to_vonc_notice_received_20241123', 'to_speaker_vonc_withdrawn_procedure_20241209',
    'to_assembly_news_resignation_accepted_20241209', 'to_pm_nominations_invited_20241210',
    'to_pm_first_nomination_20241220', 'to_pm_two_nominations_20241223', 'to_pm_election_nominations_read_20241224',
}
SELECTION = {'to_eke_assembly_selection_20241224', 'to_assembly_news_eke_elected_20241224',
             'to_eke_assembly_designate_pmo_20250122'}
ACTING = {'to_speaker_caretaker_ruling_20241209', 'to_cabinet_proclamation_20241210', 'to_acting_pm_addressed_20241224',
          'to_acting_pm_vaipulu_floor_20241224', 'to_caretaker_acting_pm_vaipulu_20250106'}
APPOINTMENT = 'to_eke_royal_appointment_20250122'
CORROBORATION = {'to_eke_pm_pif_release_20250124', 'to_ipu_2025_transition'}
CABINET_EFFECTIVE = 'to_eke_cabinet_effective_20250128'
LETTERS = 'to_eke_cabinet_letters_planned_20250128'
OATH = {'to_oath_sitting_notice_20250129', 'to_eke_oath_20250131', 'to_assembly_news_oath_20250131'}
DPM_LISTED = 'to_deputy_pm_vaipulu_listed_20241209'
UPPER_BOUND = 'to_vaipulu_peoples_rep_listed_20250131'
NEW_CLAIMS = (set(RESIGNATION) | PROCEDURE | SELECTION | ACTING | {APPOINTMENT} | CORROBORATION
              | {CABINET_EFFECTIVE, LETTERS, DPM_LISTED, UPPER_BOUND} | OATH)
SOVALENI, EKE = "Siaosi 'Ofakivahafolau Sovaleni", "'Aisake Valu Eke"
VAIPULU, FUSIMALOHI = 'Samiu Vaipulu', 'Taniela Likuohihifo Fusimalohi'
REPORT = research.RESEARCH / 'tonga-transition-2024-03.md'
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-03.md'
TR21_04 = ("TO-TR21-04: the signature and presentation dates of the Royal Warrant appointing Siaosi Sovaleni are not "
           "recorded in any retrieved primary source; the 28 December release date is not a signature date. A secondary "
           "news account of a 28 December presentation is a search lead only.")


class TongaTransition2024Tests(unittest.TestCase):
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

    def holder(self, role_id, name):
        found = [h for h in self.roles[role_id]['holder_claims'] if isinstance(h, dict) and h['name'] == name]
        self.assertEqual(len(found), 1, f'{name} on {role_id}')
        return found[0]

    def all_holders(self):
        for role in self.roles.values():
            for entry in role['holder_claims']:
                yield role['id'], entry

    def test_new_records_are_bounded_and_every_claim_is_cited(self):
        ids = self.validate()
        self.assertLessEqual(NEW_SOURCES, set(ids['sources']))
        self.assertEqual(len(NEW_SOURCES), 17)
        # to_ipu_2025 is shared with CLAUDE-C01-04, which owns its to_ipu_2025_no_party_result claim.
        self.assertEqual({c['id'] for sid in NEW_SOURCES for c in self.sources[sid]['claims']},
                         NEW_CLAIMS | {'to_ipu_2025_no_party_result'})
        self.assertEqual(len(NEW_CLAIMS), 27)
        cited = {cid for e in self.entries.values() for cid in e['claim_ids']}
        self.assertLessEqual(NEW_CLAIMS, cited)
        # Reuse, not new identities: nine entries and the same roles as before this packet.
        self.assertEqual(len(ids['entries']), 9)
        self.assertEqual(set(self.roles), {
            'to_pdp_leader', 'to_dpfi_leader', 'to_dpfi_president', 'to_peoples_party_leader', 'to_peoples_party_society_president',
            'to_peoples_party_society_secretary', 'to_king', 'to_pm', 'to_ministers', 'to_deputy_pm',
            'to_privy_councillors', 'to_speaker', 'to_deputy_speaker', 'to_peoples_representatives',
            'to_nobles_representatives'})
        self.assertEqual([r['id'] for r in self.roles.values() if r['kind'] == 'head_of_government'], ['to_pm'])
        self.assertFalse([r for r in self.roles.values() if 'acting' in r['title'].lower()])
        observations = re.findall(r'^### (TO-TR24-\d\d)\b', self.report, re.M)
        self.assertEqual(observations, [f'TO-TR24-{n:02d}' for n in range(1, 7)])
        # Every PM-office claim sits on both the role and the entry; Cabinet-only claims stay on the Cabinet.
        pm_role, pm_entry = self.roles['to_pm'], self.entries['to_prime_minister']
        for cid in NEW_CLAIMS - {'to_cabinet_proclamation_20241210', CABINET_EFFECTIVE, LETTERS, DPM_LISTED, UPPER_BOUND}:
            self.assertIn(cid, pm_role['claim_ids'])
            self.assertIn(cid, pm_entry['claim_ids'])
        for cid in ('to_cabinet_proclamation_20241210', CABINET_EFFECTIVE, LETTERS, UPPER_BOUND):
            self.assertIn(cid, self.entries['to_cabinet']['claim_ids'])
            self.assertNotIn(cid, pm_role['claim_ids'])

    def test_each_transition_event_keeps_its_own_date(self):
        attested = {cid: self.claims[cid].get('attested_on') for cid in NEW_CLAIMS}
        self.assertEqual(attested['to_vonc_notice_received_20241123'], '2024-11-23')
        for cid in RESIGNATION + ('to_speaker_vonc_withdrawn_procedure_20241209', 'to_speaker_caretaker_ruling_20241209',
                                  'to_assembly_news_resignation_accepted_20241209', DPM_LISTED):
            self.assertEqual(attested[cid], '2024-12-09')
        self.assertEqual(attested['to_pm_nominations_invited_20241210'], '2024-12-10')
        self.assertEqual(attested['to_cabinet_proclamation_20241210'], '2024-12-10')
        self.assertEqual(attested['to_pm_first_nomination_20241220'], '2024-12-20')
        self.assertEqual(attested['to_pm_two_nominations_20241223'], '2024-12-23')
        for cid in SELECTION | {'to_pm_election_nominations_read_20241224', 'to_acting_pm_addressed_20241224',
                                'to_acting_pm_vaipulu_floor_20241224'}:
            self.assertEqual(attested[cid], '2024-12-24')
        self.assertEqual(attested['to_caretaker_acting_pm_vaipulu_20250106'], '2025-01-06')
        self.assertEqual(attested[APPOINTMENT], '2025-01-22')
        self.assertEqual(attested['to_eke_pm_pif_release_20250124'], '2025-01-24')
        self.assertEqual((attested[CABINET_EFFECTIVE], attested[LETTERS]), ('2025-01-28', '2025-01-28'))
        self.assertEqual(attested['to_oath_sitting_notice_20250129'], '2025-01-29')
        for cid in ('to_eke_oath_20250131', 'to_assembly_news_oath_20250131', UPPER_BOUND):
            self.assertEqual(attested[cid], '2025-01-31')
        self.assertEqual(self.claims['to_ipu_2025_transition']['period'], {'from': '2024-12-01', 'through': '2025-01-31'})
        self.assertIsNone(attested['to_ipu_2025_transition'])
        # The order the sources give: resignation < invitation < selection < appointment < Cabinet < oath.
        chain = [attested[RESIGNATION[0]], attested['to_pm_nominations_invited_20241210'],
                 attested['to_eke_assembly_selection_20241224'], attested[APPOINTMENT], attested[CABINET_EFFECTIVE],
                 attested['to_eke_oath_20250131']]
        self.assertEqual(chain, sorted(set(chain)))
        # Report and upload dates are publication metadata, never the event date.
        self.assertEqual(self.sources['to_assembly_oath_news_20250203']['published_date'], '2025-02-03')
        for sid in ('to_assembly_minutes_48_20241209', 'to_assembly_minutes_pm_election_20241224'):
            self.assertEqual(self.sources[sid]['published_date'], '2025-03-04')
            self.assertIn('upload date', self.sources[sid]['scope_note'])
        # The 23 January posting date is derived, not a claim, and the letters are a forecast.
        self.assertNotIn('2025-01-23', attested.values())
        scope = self.sources['to_pmo_eke_appointment_20250122']['scope_note']
        self.assertIn('23 January 2025 local time', scope)
        self.assertIn('UTC+13', scope)
        self.assertIn('derived observation, not a claim', scope)
        self.assertIn("'will present'", self.claims[LETTERS]['text'])
        self.assertIn('forecast', self.claims[LETTERS]['uncertainty'])
        self.assertIn("'with effect from today, Tuesday 28th January 2025'", self.claims[CABINET_EFFECTIVE]['text'])
        self.assertIn('not a second start date for the premiership', self.claims[CABINET_EFFECTIVE]['uncertainty'])
        self.assertIn("'Official beginning of their term in Parliament'", self.claims['to_assembly_news_oath_20250131']['uncertainty'])
        self.assertIn('not read as the start of office', self.claims['to_assembly_news_oath_20250131']['uncertainty'])

    def test_sovaleni_ends_on_the_stated_resignation_not_on_the_successor(self):
        holder = self.holder('to_pm', SOVALENI)
        self.assertEqual((holder['from'], holder['until'], holder['attested_on']), ('2021-12-27', '2024-12-09', None))
        self.assertEqual(holder['until'], self.claims[RESIGNATION[0]]['attested_on'])
        self.assertEqual(holder['until'], self.claims[RESIGNATION[1]]['attested_on'])
        self.assertEqual(holder['claim_ids'][-2:], list(RESIGNATION))
        self.assertIn('to_assembly_minutes_48_20241209', holder['sources'])
        self.assertIn('effective immediately', self.claims[RESIGNATION[0]]['text'])
        self.assertIn('accepted the resignation', self.claims[RESIGNATION[1]]['text'])
        self.assertIn('within that recess', self.claims[RESIGNATION[1]]['uncertainty'])
        self.assertIn('clock time is not recorded', self.claims[RESIGNATION[1]]['uncertainty'])
        # The end sentence was replaced, not contradicted, and the 2021 open question survives.
        self.assertNotIn('No end date or term is inferred', holder['uncertainty'])
        self.assertIn('No term length is inferred', holder['uncertainty'])
        self.assertIn('TO-TR21-04', holder['uncertainty'])
        self.assertIn('1400-1405', holder['uncertainty'])
        self.assertIn("'Siaosi 'Ofa ki Vahafolau Sovaleni'", holder['note'])
        # Nothing from the successor's selection, appointment or Cabinet supports the end.
        self.assertFalse(set(holder['claim_ids']) & (SELECTION | {APPOINTMENT, CABINET_EFFECTIVE} | CORROBORATION))
        self.assertNotIn('to_pmo_eke_appointment_20250122', holder['sources'])

    def test_eke_is_an_appointment_event_with_no_inferred_start_or_end(self):
        self.assertEqual(self.holder('to_pm', EKE), {
            'name': EKE,
            'attested_on': '2025-01-22',
            'from': None,
            'until': None,
            'sources': ['to_pmo_eke_appointment_20250122'],
            'claim_ids': [APPOINTMENT],
            'note': self.holder('to_pm', EKE)['note'],
            'uncertainty': self.holder('to_pm', EKE)['uncertainty'],
        })
        holder = self.holder('to_pm', EKE)
        self.assertIn('TO-TR24-05', holder['uncertainty'])
        self.assertIn('No end or term is inferred', holder['uncertainty'])
        self.assertIn("'today, Wednesday 22 January 2025, at the Royal Palace, Nuku'alofa'", self.claims[APPOINTMENT]['text'])
        self.assertIn("'as the Prime Minister Tonga' (as printed", self.claims[APPOINTMENT]['text'])
        self.assertIn("no separate 'with effect from' date", self.claims[APPOINTMENT]['uncertainty'])
        # The existing string holders and the 2019 and 2025 observations are untouched.
        holders = self.roles['to_pm']['holder_claims']
        self.assertEqual(holders[:2], ['to_fakafanua_appointment', 'to_fakafanua_august'])
        self.assertEqual([h['name'] for h in holders if isinstance(h, dict)], ["Pohiva Tu'i'onetoa", SOVALENI, EKE])
        self.assertIsNone(self.holder('to_pm', "Pohiva Tu'i'onetoa")['until'])
        # No selection, procedure, acting, corroboration, oath or letters claim becomes any holder.
        banned = SELECTION | PROCEDURE | ACTING | CORROBORATION | OATH | {LETTERS, UPPER_BOUND}
        for role_id, entry in self.all_holders():
            ids = [entry] if isinstance(entry, str) else entry['claim_ids']
            self.assertFalse(set(ids) & banned, role_id)

    def test_acting_premiership_stays_claims_not_a_holder(self):
        for role_id, entry in self.all_holders():
            if isinstance(entry, dict) and 'Vaipulu' in entry['name']:
                self.assertEqual(role_id, 'to_deputy_pm')
        caretaker = self.claims['to_caretaker_acting_pm_vaipulu_20250106']
        self.assertIn('Hon. Samiu Kuita Vaipulu', caretaker['text'])
        self.assertIn("'caretaker mode'", caretaker['text'])
        self.assertIn('does not say when Vaipulu began acting', caretaker['uncertainty'])
        self.assertIn('commencement of the New Government', caretaker['uncertainty'])
        floor = self.claims['to_acting_pm_vaipulu_floor_20241224']
        self.assertIn("'me'a mai Palēmia Le'ole'o'", floor['text'])
        self.assertIn('Samiu Kuita Vaipulu', floor['text'])
        self.assertIn('9 December is not inferred', floor['uncertainty'])
        self.assertIn('text layer only', floor['uncertainty'])
        self.assertIn('names no person and dates no start', self.claims['to_speaker_caretaker_ruling_20241209']['uncertainty'])
        self.assertIn('None of these salutations names', self.claims['to_acting_pm_addressed_20241224']['text'])
        unresolved = self.entries['to_prime_minister']['coverage']['unresolved']
        acting, = [u for u in unresolved if u.startswith('TO-TR24-02')]
        for phrase in ('24 December 2024', '6 January 2025', 'none is inferred'):
            self.assertIn(phrase, acting)
        warrant, = [u for u in unresolved if u.startswith('TO-TR24-05')]
        self.assertIn('no Gazette notice is ruled out', warrant)

    def test_deputy_prime_minister_observations_use_stated_dates_only(self):
        role = self.roles['to_deputy_pm']
        holders = role['holder_claims']
        self.assertEqual(holders[0], 'to_cabinet_appointment')
        self.assertEqual([h['name'] for h in holders[1:]], ['Poasi Mataele Tei', VAIPULU, FUSIMALOHI])
        self.assertIsNone(self.holder('to_deputy_pm', 'Poasi Mataele Tei')['until'])
        vaipulu = self.holder('to_deputy_pm', VAIPULU)
        self.assertEqual((vaipulu['attested_on'], vaipulu['from'], vaipulu['until']), ('2024-12-09', None, None))
        self.assertEqual((vaipulu['sources'], vaipulu['claim_ids']), (['to_assembly_minutes_48_20241209'], [DPM_LISTED]))
        self.assertIn('upper bound, not an end date', vaipulu['uncertainty'])
        fusimalohi = self.holder('to_deputy_pm', FUSIMALOHI)
        self.assertEqual((fusimalohi['attested_on'], fusimalohi['from'], fusimalohi['until']), (None, '2025-01-28', None))
        self.assertEqual((fusimalohi['sources'], fusimalohi['claim_ids']), (['to_pmo_eke_cabinet_20250128'], [CABINET_EFFECTIVE]))
        self.assertEqual(fusimalohi['from'], self.claims[CABINET_EFFECTIVE]['attested_on'])
        self.assertIn("'Taniela Liku'ohihifo Fusimālohi'", fusimalohi['note'])
        # The 31 January listing bounds Vaipulu but is cited by no holder; other ministers stay unimported.
        self.assertIn(UPPER_BOUND, role['claim_ids'])
        self.assertIn('upper bound only', self.claims[UPPER_BOUND]['uncertainty'])
        self.assertEqual(self.roles['to_ministers']['holder_claims'], [])
        self.assertIn('Items 4-11 are not imported', self.claims[CABINET_EFFECTIVE]['uncertainty'])
        self.assertTrue(any(u.startswith('TO-TR24-06') for u in self.entries['to_cabinet']['coverage']['unresolved']))

    def test_holder_names_are_printed_in_the_claims_they_cite(self):
        expected = {('to_pm', EKE): "Honourable Dr. 'Aisake Valu Eke", ('to_deputy_pm', VAIPULU): 'Hon. Samiu Vaipulu',
                    ('to_deputy_pm', FUSIMALOHI): 'Hon. Dr. Taniela Likuohihifo Fusimalohi'}
        for (role_id, name), printed in expected.items():
            holder = self.holder(role_id, name)
            self.assertIn(name, printed)
            self.assertTrue(any(printed in self.claims[cid]['text'] for cid in holder['claim_ids']), name)

    def test_news_and_social_leads_are_not_sources(self):
        banned_hosts = ('matangitonga.to', 'rnz.co.nz', 'usnews.com', 'x.com', 'wikipedia.org')
        for source in self.packet['sources']:
            host = urlsplit(source['url']).hostname
            self.assertFalse(any(host.endswith(b) for b in banned_hosts), host)
        for text in ('matangi', 'rnz', 'reuters', '9:53', 'former-prime-minsiters'):
            self.assertNotIn(text, self.raw.lower())
        leads = self.report.split('## Leads not imported', 1)[1].split('\n## ', 1)[0]
        for text in ('matangitonga.to', 'rnz.co.nz', 'usnews.com', 'x.com/TongaMissionUN', '2026-03-06', 'must not be merged'):
            self.assertIn(text, leads)
        added = self.report.split('## Sources added', 1)[1].split('\n## ', 1)[0]
        for text in ('matangitonga.to', 'rnz.co.nz', 'usnews.com'):
            self.assertNotIn(text, added)

    def test_extracts_match_the_packet_and_name_the_hashed_response(self):
        for sid in NEW_SOURCES:
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual(extract['format'], 'spheres-c01-derived-factual-table/v1')
            self.assertEqual(extract['source_url'], source['url'])
            self.assertEqual(extract['claims'], source['claims'])
            self.assertEqual(extract['scope_note'], source['scope_note'])
            self.assertEqual(extract.get('published_date'), source.get('published_date'))
            self.assertEqual((extract['accessed_date'], source['accessed_date']), ('2026-09-21', '2026-09-21'))
            self.assertFalse(extract['source_response_checked_in'])
            self.assertEqual(source['snapshot']['kind'], 'derived_factual_extract')
            self.assertTrue(source['snapshot']['path'].startswith('docs/campaign-certification/C01/research/sources/tonga-'))
            data = (research.ROOT / source['snapshot']['path']).read_bytes()
            self.assertEqual((len(data), hashlib.sha256(data).hexdigest()),
                             (source['snapshot']['bytes'], source['snapshot']['sha256']))
            self.assertNotIn(b'\r', data)
            self.assertNotEqual(source['snapshot']['sha256'], extract['source_response_sha256'])
            self.assertIn('No open license, portrait permission or likeness approval', extract['rights_note'])
            self.assertIn('derived factual extract', extract['provenance_note'])
        for sid, nbytes in ASSEMBLY_HTML.items():
            extract = self.extracts[sid]
            self.assertEqual(urlsplit(self.sources[sid]['url']).hostname, 'parliament.gov.to')
            self.assertEqual((extract['source_response_bytes'], extract['source_response_sha256']), (nbytes, None))
            self.assertIn('form token', extract['provenance_note'])
            self.assertIn('hit counter', extract['provenance_note'])
        for sid, (nbytes, digest, rendered, text_only) in MINUTES.items():
            extract = self.extracts[sid]
            self.assertEqual((extract['source_response_bytes'], extract['source_response_sha256']), (nbytes, digest))
            self.assertEqual(extract['source_response_url'], self.sources[sid]['url'])
            self.assertIn('HTTP POST of the Phoca Download form', extract['source_response_request'])
            self.assertEqual(extract['visual_review']['pdf_pages_one_based'], rendered)
            self.assertEqual(extract['visual_review']['text_layer_only_pages_one_based'], text_only)
            self.assertIn('re-download', extract['provenance_note'])
        self.assertIn('stale form token', self.extracts['to_assembly_minutes_48_20241209']['provenance_note'])
        for sid, (url, nbytes, digest) in PMO.items():
            extract = self.extracts[sid]
            self.assertEqual(urlsplit(self.sources[sid]['url']).hostname, 'pmo.gov.to')
            self.assertEqual((extract['source_response_url'], extract['source_response_bytes'],
                              extract['source_response_sha256']), (url, nbytes, digest))
            if sid != 'to_pmo_pif_joint_release_20250124':
                self.assertNotEqual(extract['source_response_url'], self.sources[sid]['url'])
                self.assertIn('differs from', extract['provenance_note'])
        images = self.extracts['to_pmo_caretaker_announcement_20250106']['release_image_responses']
        self.assertEqual([(i['language'], i['bytes'], i['sha256']) for i in images], [
            ('English', 137870, 'db9c580da9fe40f160f89b658f1830dda59726145a28f57d307d994ed9d7d250'),
            ('Tongan', 160214, '682a45b55665a39c0c0ff865e71f12d39a30c552f3834e1dae33d466088fa488')])
        page = self.extracts['to_pmo_caretaker_announcement_20250106']['page_response']
        self.assertEqual((page['bytes'], page['sha256']), (69871, '7fce529e1a9cfba399c4333762fe5c93b12b8a8c1105b139ce1704df25f91163'))
        captures = self.extracts['to_pmo_eke_appointment_20250122']['archive_captures']
        self.assertEqual([(c['memento_datetime'], c['bytes']) for c in captures],
                         [('2025-01-23T15:51:00Z', 66444), ('2025-02-17T16:48:54Z', 75048)])
        self.assertEqual(self.extracts['to_pmo_eke_appointment_20250122']['live_response']['status'], 404)
        self.assertIsNone(self.extracts['to_pmo_eke_cabinet_20250128']['page_response']['sha256'])
        for sid, (nbytes, digest) in OTHER.items():
            self.assertEqual((self.extracts[sid]['source_response_bytes'], self.extracts[sid]['source_response_sha256']),
                             (nbytes, digest))
        self.assertEqual(self.extracts['to_gazette_gse22_20241210']['visual_review']['text_layer_only_pages_one_based'], [1])
        self.assertIn('selective', self.sources['to_gazette_gse22_20241210']['scope_note'])

    def test_cutoff_holds_and_mutations_are_rejected(self):
        for cid in NEW_CLAIMS:
            claim = self.claims[cid]
            day = claim.get('attested_on') or claim['period']['through']
            self.assertLessEqual(day, '2025-01-31')
            self.assertLess(day, research.CUTOFF)
        for sid in NEW_SOURCES:
            if self.sources[sid].get('published_date'):
                self.assertLessEqual(self.sources[sid]['published_date'], research.CUTOFF)

        def mutated(change):
            packet = copy.deepcopy(self.packet)
            change(packet)
            return packet

        def role(packet, role_id):
            return next(r for e in packet['institutions'] for r in e['roles'] if r['id'] == role_id)

        def named(packet, role_id, name):
            return next(h for h in role(packet, role_id)['holder_claims'] if isinstance(h, dict) and h['name'] == name)

        def source(packet, sid):
            return next(s for s in packet['sources'] if s['id'] == sid)

        cases = [
            (lambda p: named(p, 'to_pm', EKE).update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: named(p, 'to_pm', SOVALENI).update(until='2021-12-26'), 'Reversed historical interval'),
            (lambda p: named(p, 'to_pm', SOVALENI)['sources'].remove('to_assembly_minutes_48_20241209'), 'cited source'),
            (lambda p: named(p, 'to_deputy_pm', FUSIMALOHI).update(claim_ids=[APPOINTMENT]), 'cited source'),
            (lambda p: named(p, 'to_deputy_pm', VAIPULU).update(claim_ids=['to_caretaker_acting_pm_vaipulu_20250106']),
             'cited source'),
            (lambda p: role(p, 'to_deputy_pm')['sources'].remove('to_assembly_minutes_01_20250131'), 'cited source'),
            (lambda p: source(p, 'to_pmo_eke_appointment_20250122')['snapshot'].update(sha256='0' * 64), 'checksum mismatch'),
            (lambda p: source(p, 'to_assembly_minutes_48_20241209')['snapshot'].update(bytes=1), 'checksum mismatch'),
            (lambda p: next(c for c in source(p, 'to_ipu_2025')['claims'] if c['id'] == 'to_ipu_2025_transition')['period'].update(through='2026-09-08'), 'exceeds cutoff'),
        ]
        for change, message in cases:
            with self.subTest(message=message), self.assertRaisesRegex(ValueError, message):
                self.validate(mutated(change))

    def test_2021_questions_are_preserved_and_nothing_closes(self):
        unresolved = self.entries['to_prime_minister']['coverage']['unresolved']
        self.assertIn(TR21_04, unresolved)
        tr21_05, = [u for u in unresolved if u.startswith('TO-TR21-05')]
        self.assertIn('no end date is inferred from Sovaleni', tr21_05)
        self.assertIn('Tu\'i\'onetoa', tr21_05)
        self.assertTrue(any(u.startswith('2024-2025 transition (tonga-transition-2024-03.md)') for u in unresolved))
        self.assertIn('ready_for_review', self.report)
        table = self.report.split('## Outcome', 1)[1].split('\n### ', 1)[0]
        rows = {rid: row for rid in (f'TO-TR24-{n:02d}' for n in range(1, 7))
                for row in [line for line in table.splitlines() if line.startswith(f'| {rid} ')]}
        self.assertEqual(len(rows), 6)
        for rid in ('TO-TR24-01', 'TO-TR24-03', 'TO-TR24-04', 'TO-TR24-06'):
            self.assertIn('**Accepted', rows[rid])
            self.assertNotIn('nresolved', rows[rid])
        self.assertIn('**Accepted as attestation', rows['TO-TR24-02'])
        self.assertIn('**unresolved**', rows['TO-TR24-02'])
        self.assertIn('**Partly resolved', rows['TO-TR24-05'])
        self.assertIn('**unresolved**', rows['TO-TR24-05'])
        self.assertNotIn('Accepted', rows['TO-TR24-05'])
        for marker in ('C01', 'C06', 'S23', 'WC1', 'CP1'):
            self.assertNotRegex(self.report, rf'\b{marker}\b[^.\n]*\bis (now )?complete\b')
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        self.assertIn('ready_for_review', handoff)
        self.assertIn('tonga-transition-2024-03.md', handoff)
        self.assertIn('test_tonga_transition_c01_03.py', handoff)
        index = research.build()
        self.assertFalse(index['c01_complete'])
        country = next(p for p in index['countries'] if p['nation'] == 'Tonga')
        self.assertFalse(country['country_census_complete'])
        self.assertIsNone(country['unrepresented_organization_count'])
        self.assertEqual(country['mapping_pending'], 9)
        batch, = [row for row in index['work_orders'] if row['nation'] == 'Tonga']
        self.assertEqual(batch['status'], 'open')
        for entry in self.entries.values():
            self.assertEqual(entry['represented_party_ids'], [])


if __name__ == '__main__':
    unittest.main()
