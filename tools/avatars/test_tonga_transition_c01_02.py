"""CLAUDE-C01-02: Tonga's 2021 transition keeps selection, appointment, publication and Cabinet dates apart."""
import copy
import hashlib
import json
import re
import unittest
from urllib.parse import urlsplit

import campaign_research as research


PMO_SOURCES = {
    'to_pmo_sovaleni_appointment_20211228',
    'to_pmo_sovaleni_appointment_to_20211228',
    'to_pmo_cabinet_20211229',
}
ASSEMBLY_SOURCES = {
    'to_assembly_interim_speaker_20211122',
    'to_assembly_pm_nominations_20211201',
    'to_assembly_pm_meeting_20211213',
    'to_assembly_pm_nominations_20211214',
}
NEW_SOURCES = PMO_SOURCES | ASSEMBLY_SOURCES
NEW_CLAIMS = {
    'to_interim_speaker_tangi_20211120',
    'to_pm_nominations_invited_20211130',
    'to_pm_election_meeting_notice_20211213',
    'to_pm_two_nominations_20211214',
    'to_sovaleni_royal_appointment_en_20211227',
    'to_sovaleni_assembly_election_pmo_20211215',
    'to_sovaleni_royal_appointment_to_20211227',
    'to_sovaleni_cabinet_effective_20211228',
    'to_sovaleni_cabinet_letters_oath_20211229',
}
PROCEDURE = {
    'to_interim_speaker_tangi_20211120',
    'to_pm_nominations_invited_20211130',
    'to_pm_election_meeting_notice_20211213',
    'to_pm_two_nominations_20211214',
}
SELECTION = ('to_ipu_2021_sovaleni_assembly_selection', 'to_sovaleni_assembly_election_pmo_20211215')
APPOINTMENT = ('to_sovaleni_royal_appointment_en_20211227', 'to_sovaleni_royal_appointment_to_20211227')
CABINET_EFFECTIVE = 'to_sovaleni_cabinet_effective_20211228'
LETTERS_OATH = 'to_sovaleni_cabinet_letters_oath_20211229'
SOVALENI = "Siaosi 'Ofakivahafolau Sovaleni"
# Original release images, downloaded directly and hashed before extraction (not checked in).
RELEASE_IMAGES = {
    'to_pmo_sovaleni_appointment_20211228': [
        ('https://pmo.gov.to/wp-content/uploads/2021/12/NewPrimeMinister2021.jpg', 228292,
         '877ef23c5938d223205d5ba4e55cab764e07cad5696aea530f418ebf7fcb61f2')],
    'to_pmo_sovaleni_appointment_to_20211228': [
        ('https://pmo.gov.to/wp-content/uploads/2021/12/newprimeminister2.jpg', 208565,
         '28af517a9264f5334b1eea3153380291eb83b3213d7f1d2de8fd95f1e38c92e2')],
    'to_pmo_cabinet_20211229': [
        ('https://pmo.gov.to/wp-content/uploads/2021/12/primeminister-2021.jpg', 277933,
         '246495c5920258a9a211b31866f58eaf37b4efcf2b2f24ce77b9fb00cfdabde5'),
        ('https://pmo.gov.to/wp-content/uploads/2021/12/primeminister-2021-1-1.jpg', 297845,
         'e4c96820254d201b0dfb4218550da87c5bd1758f64500dea0bf332490af860fc')],
}
PAGE_BYTES = {
    'to_pmo_sovaleni_appointment_20211228': 76329,
    'to_pmo_sovaleni_appointment_to_20211228': 76399,
    'to_pmo_cabinet_20211229': 77161,
    'to_assembly_interim_speaker_20211122': 55949,
    'to_assembly_pm_nominations_20211201': 55879,
    'to_assembly_pm_meeting_20211213': 55539,
    'to_assembly_pm_nominations_20211214': 54450,
}
REPORT = research.RESEARCH / 'tonga-transition-2021-02.md'
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-02.md'


class TongaTransitionTests(unittest.TestCase):
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

    def test_new_records_are_bounded_and_every_claim_is_cited(self):
        ids = self.validate()
        self.assertLessEqual(NEW_SOURCES, set(ids['sources']))
        self.assertEqual({c['id'] for sid in NEW_SOURCES for c in self.sources[sid]['claims']}, NEW_CLAIMS)
        self.assertEqual(len(ids['entries']), 9)
        cited = {cid for e in self.entries.values() for cid in e['claim_ids']}
        self.assertLessEqual(NEW_CLAIMS, cited)
        observations = re.findall(r'^### (TO-TR21-\d\d)\b', self.report, re.M)
        self.assertEqual(observations, [f'TO-TR21-{n:02d}' for n in range(1, 7)])
        # The existing IPU record is reused, not duplicated or rewritten.
        self.assertEqual(self.claim_source[SELECTION[0]], 'to_ipu_2021')
        self.assertEqual(sum(1 for s in self.packet['sources'] if 'data.ipu.org' in s['url'] and '2021' in s['url']), 1)

    def test_selection_effect_publication_cabinet_and_oath_dates_stay_separate(self):
        for cid in SELECTION:
            self.assertEqual(self.claims[cid]['attested_on'], '2021-12-15')
        for cid in APPOINTMENT:
            claim = self.claims[cid]
            self.assertEqual(claim['attested_on'], '2021-12-27')
            self.assertNotIn('period', claim)
            # The release date is publication metadata, never the appointment's own date.
            self.assertEqual(self.sources[self.claim_source[cid]]['published_date'], '2021-12-28')
            self.assertNotEqual(self.sources[self.claim_source[cid]]['published_date'], claim['attested_on'])
        self.assertIn('with effect from 27 December 2021', self.claims[APPOINTMENT[0]]['text'])
        self.assertIn('signed or presented', self.claims[APPOINTMENT[0]]['uncertainty'])
        self.assertIn('27 Tisema 2021', self.claims[APPOINTMENT[1]]['text'])
        self.assertIn('not an official translation', self.claims[APPOINTMENT[1]]['uncertainty'])
        effective, letters = self.claims[CABINET_EFFECTIVE], self.claims[LETTERS_OATH]
        self.assertEqual((effective['attested_on'], letters['attested_on']), ('2021-12-28', '2021-12-29'))
        self.assertEqual(self.sources['to_pmo_cabinet_20211229']['published_date'], '2021-12-29')
        self.assertIn('with effect from 28 December 2021', effective['text'])
        self.assertIn('not read as a second start date for the premiership', effective['uncertainty'])
        self.assertIn('Wednesday 29th December, 2021', letters['text'])
        self.assertIn('Ministerial oath', letters['text'])
        self.assertIn('no date of its own', letters['uncertainty'])
        self.assertIn('not the Royal Warrant', letters['uncertainty'])
        dated = {
            'selection': self.claims[SELECTION[1]]['attested_on'],
            'effective': self.claims[APPOINTMENT[0]]['attested_on'],
            'release': self.sources['to_pmo_sovaleni_appointment_20211228']['published_date'],
            'cabinet_effective': effective['attested_on'],
            'letters_oath': letters['attested_on'],
        }
        self.assertEqual(dated, {'selection': '2021-12-15', 'effective': '2021-12-27', 'release': '2021-12-28',
                                 'cabinet_effective': '2021-12-28', 'letters_oath': '2021-12-29'})
        self.assertLess(dated['selection'], dated['effective'])
        self.assertLess(dated['effective'], dated['release'])
        self.assertLess(dated['cabinet_effective'], dated['letters_oath'])
        # The Assembly procedure notices are dated before the selection and record no result.
        self.assertEqual(self.claims['to_interim_speaker_tangi_20211120']['attested_on'], '2021-11-20')
        self.assertEqual(self.claims['to_pm_nominations_invited_20211130']['period'],
                         {'from': '2021-11-30', 'through': '2021-12-14'})
        self.assertEqual(self.claims['to_pm_two_nominations_20211214']['attested_on'], '2021-12-14')
        self.assertIn('no Assembly notice of the 15 December result', self.claims['to_pm_two_nominations_20211214']['uncertainty'])

    def test_sovaleni_holder_starts_on_the_stated_effective_date_with_no_end(self):
        holder = self.holder('to_pm', SOVALENI)
        self.assertEqual((holder['from'], holder['until'], holder['attested_on']), ('2021-12-27', None, None))
        self.assertEqual(holder['claim_ids'], list(APPOINTMENT))
        self.assertEqual(holder['sources'], ['to_pmo_sovaleni_appointment_20211228', 'to_pmo_sovaleni_appointment_to_20211228'])
        self.assertNotIn('attested_period', holder)
        self.assertIn('TO-TR21-04', holder['uncertainty'])
        self.assertIn("Hu'akavameiliku", holder['note'])
        self.assertIn("(Hu'akavameiliku)", self.claims[APPOINTMENT[1]]['text'])
        self.assertIn('no chiefly-title grant', self.claims[APPOINTMENT[1]]['uncertainty'])
        # Selection and Cabinet claims never become a holder or a start date.
        for role in self.roles.values():
            for entry in role['holder_claims']:
                ids = [entry] if isinstance(entry, str) else entry['claim_ids']
                self.assertFalse(set(ids) & set(SELECTION))
                self.assertFalse(set(ids) & PROCEDURE)
                if isinstance(entry, dict):
                    self.assertNotEqual(entry['name'], "Hu'akavameiliku")
        self.assertNotIn(CABINET_EFFECTIVE, holder['claim_ids'])
        self.assertNotEqual(holder['from'], self.claims[CABINET_EFFECTIVE]['attested_on'])
        self.assertNotEqual(holder['from'], self.claims[SELECTION[1]]['attested_on'])

    def test_tuionetoa_premiership_gains_no_inferred_end(self):
        self.assertEqual(self.holder('to_pm', "Pohiva Tu'i'onetoa"), {
            'name': "Pohiva Tu'i'onetoa",
            'attested_on': '2019-10-08',
            'from': None,
            'until': None,
            'sources': ['to_pm_appointment_20191008'],
            'claim_ids': ['to_tuionetoa_royal_appointment_20191008'],
            'uncertainty': 'Royal appointment event only. The earlier Assembly recommendation is retained separately and is not a holder observation.',
        })
        for role in self.roles.values():
            for entry in role['holder_claims']:
                if isinstance(entry, dict) and "Tu'i'onetoa" in entry['name']:
                    self.assertIsNone(entry['until'])
        unresolved = self.entries['to_prime_minister']['coverage']['unresolved']
        end, = [u for u in unresolved if u.startswith('TO-TR21-05')]
        self.assertIn('no end date is inferred', end)
        signature, = [u for u in unresolved if u.startswith('TO-TR21-04')]
        self.assertIn('not a signature date', signature)
        self.assertTrue(any('TO-REC-10' in u for u in unresolved))

    def test_deputy_prime_minister_starts_on_cabinet_effect_not_on_letters(self):
        role = self.roles['to_deputy_pm']
        self.assertEqual(role['holder_claims'][0], 'to_cabinet_appointment')
        holder = self.holder('to_deputy_pm', 'Poasi Mataele Tei')
        self.assertEqual((holder['from'], holder['until'], holder['attested_on']), ('2021-12-28', None, None))
        self.assertEqual(holder['claim_ids'], [CABINET_EFFECTIVE])
        self.assertEqual(holder['sources'], ['to_pmo_cabinet_20211229'])
        self.assertIn(LETTERS_OATH, role['claim_ids'])
        self.assertIn('Hon. Poasi Mataele Tei as Deputy Prime Minister', self.claims[CABINET_EFFECTIVE]['text'])
        # Other ministers in the twelve-member list are not imported as holders.
        self.assertEqual(self.roles['to_ministers']['holder_claims'], [])
        self.assertTrue(any(u.startswith('TO-TR21-06') for u in self.entries['to_cabinet']['coverage']['unresolved']))
        # The interim Speaker procedure notice does not become a Speaker holder.
        self.assertEqual(self.roles['to_speaker']['holder_claims'], ['to_speakers_appointment'])

    def test_no_second_prime_minister_institution(self):
        self.assertEqual({e['id'] for e in self.packet['institutions']},
                         {'to_crown', 'to_prime_minister', 'to_cabinet', 'to_privy_council', 'to_legislative_assembly'})
        heads = [r['id'] for r in self.roles.values() if r['kind'] == 'head_of_government']
        self.assertEqual(heads, ['to_pm'])
        titled = sorted(r['id'] for r in self.roles.values() if 'prime minister' in r['title'].lower())
        self.assertEqual(titled, ['to_deputy_pm', 'to_pm'])
        holders = self.roles['to_pm']['holder_claims']
        self.assertEqual(sum(isinstance(h, str) for h in holders), 2)
        self.assertEqual([h['name'] for h in holders if isinstance(h, dict)], ["Pohiva Tu'i'onetoa", SOVALENI])

    def test_secondary_warrant_account_is_a_lead_not_a_source(self):
        for source in self.packet['sources']:
            self.assertNotIn('matangi', urlsplit(source['url']).hostname)
        self.assertNotIn('matangi', self.raw.lower())
        self.assertNotIn('four years', self.raw)
        leads = self.report.split('## Leads not imported', 1)
        self.assertEqual(len(leads), 2)
        self.assertIn('matangitonga.to', leads[1].split('\n## ', 1)[0])
        self.assertNotIn('matangitonga.to', self.report.split('## Sources added', 1)[1].split('\n## ', 1)[0])

    def test_extracts_match_packet_claims_and_record_original_responses(self):
        for sid in NEW_SOURCES:
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual(extract['source_url'], source['url'])
            self.assertEqual(extract['claims'], source['claims'])
            self.assertEqual(extract['scope_note'], source['scope_note'])
            self.assertEqual(extract['published_date'], source['published_date'])
            self.assertEqual((extract['accessed_date'], source['accessed_date']), ('2026-09-21', '2026-09-21'))
            self.assertFalse(extract['source_response_checked_in'])
            self.assertEqual(source['snapshot']['kind'], 'derived_factual_extract')
            data = (research.ROOT / source['snapshot']['path']).read_bytes()
            self.assertEqual((len(data), hashlib.sha256(data).hexdigest()),
                             (source['snapshot']['bytes'], source['snapshot']['sha256']))
            self.assertNotEqual(source['snapshot']['sha256'], extract['source_response_sha256'])
            self.assertIn('No open license, portrait permission or likeness approval', extract['rights_note'])
            self.assertLessEqual(source['published_date'], research.CUTOFF)
        for sid in PMO_SOURCES:
            extract, images = self.extracts[sid], RELEASE_IMAGES[sid]
            self.assertEqual(urlsplit(self.sources[sid]['url']).hostname, 'pmo.gov.to')
            self.assertEqual([(i['url'], i['bytes'], i['sha256']) for i in extract['release_image_responses']], images)
            self.assertEqual((extract['source_response_url'], extract['source_response_bytes'],
                              extract['source_response_sha256']), images[0])
            self.assertEqual(extract['page_response']['bytes'], PAGE_BYTES[sid])
            self.assertIsNone(extract['page_response']['sha256'])
            self.assertEqual(extract['visual_review']['facsimile_pages_one_based'], list(range(1, len(images) + 1)))
            self.assertEqual(extract['visual_review']['pdf_pages_one_based'], [])
            self.assertIn('HTTP 403', self.sources[sid]['scope_note'])
        for sid in ASSEMBLY_SOURCES:
            extract = self.extracts[sid]
            self.assertEqual(urlsplit(self.sources[sid]['url']).hostname, 'parliament.gov.to')
            self.assertEqual((extract['source_response_bytes'], extract['source_response_sha256']), (PAGE_BYTES[sid], None))
            self.assertIn('form token', extract['provenance_note'])
            self.assertIn('hit counter', extract['provenance_note'])

    def test_mutations_are_rejected(self):
        def mutated(change):
            packet = copy.deepcopy(self.packet)
            change(packet)
            return packet

        def role(packet, role_id):
            return next(r for e in packet['institutions'] for r in e['roles'] if r['id'] == role_id)

        def sovaleni(packet):
            return next(h for h in role(packet, 'to_pm')['holder_claims'] if isinstance(h, dict) and h['name'] == SOVALENI)

        def source(packet, sid):
            return next(s for s in packet['sources'] if s['id'] == sid)

        cases = [
            (lambda p: source(p, 'to_pmo_cabinet_20211229')['snapshot'].update(sha256='0' * 64), 'checksum mismatch'),
            (lambda p: source(p, 'to_assembly_pm_meeting_20211213')['snapshot'].update(bytes=1), 'checksum mismatch'),
            (lambda p: sovaleni(p).update({'from': '2026-09-08'}), 'exceeds cutoff'),
            (lambda p: sovaleni(p).update(until='2021-12-26'), 'Reversed historical interval'),
            (lambda p: source(p, 'to_pmo_cabinet_20211229')['claims'][1].update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: next(h for h in role(p, 'to_deputy_pm')['holder_claims'] if isinstance(h, dict)).update(
                claim_ids=[APPOINTMENT[0]]), 'cited source'),
        ]
        for change, message in cases:
            with self.subTest(message=message), self.assertRaisesRegex(ValueError, message):
                self.validate(mutated(change))

    def test_report_and_handoff_are_ready_for_review_and_close_nothing(self):
        self.assertIn('ready_for_review', self.report)
        table = self.report.split('## Outcome', 1)[1].split('\n## ', 1)[0]
        for rid in ('TO-TR21-04', 'TO-TR21-05'):
            row, = [line for line in table.splitlines() if line.startswith(f'| {rid} ')]
            self.assertIn('**Unresolved', row)
            self.assertNotIn('Accepted', row)
        for rid in ('TO-TR21-01', 'TO-TR21-02', 'TO-TR21-03', 'TO-TR21-06'):
            row, = [line for line in table.splitlines() if line.startswith(f'| {rid} ')]
            self.assertIn('**Accepted', row)
        for marker in ('C01', 'C06', 'S23', 'WC1', 'CP1'):
            self.assertNotRegex(self.report, rf'\b{marker}\b[^.\n]*\bis (now )?complete\b')
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        self.assertIn('ready_for_review', handoff)
        self.assertIn('tonga-transition-2021-02.md', handoff)
        index = research.build()
        country = next(p for p in index['countries'] if p['nation'] == 'Tonga')
        self.assertFalse(country['country_census_complete'])
        self.assertIsNone(country['unrepresented_organization_count'])
        batch, = [row for row in index['work_orders'] if row['nation'] == 'Tonga']
        self.assertEqual(batch['status'], 'open')


if __name__ == '__main__':
    unittest.main()
