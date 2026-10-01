"""CLAUDE-C01-38: French prime ministers appointed from 31 March 2014 to the cutoff (Manuel Valls to Sébastien Lecornu, the batch after
CLAUDE-C01-37) extend the role fr_pm through the importer's supplement only. The Prime Minister's resignation letter, the decree ending the
Government's functions, the appointment decree and a decree the Prime Minister signs stay apart; decree dates are observations
(attested_on) and never inferred effective boundaries; CLAUDE-C01-23's presidency and CLAUDE-C01-37's first batch stay unchanged."""
import copy
import hashlib
import json
import re
import unittest
from urllib.parse import urlsplit

import campaign_research as research
import import_cnccfp_census as importer

PRESIDENCY, PRESIDENT, INSTITUTION, ROLE = 'fr_presidency', 'fr_president', 'fr_prime_minister', 'fr_pm'
C01_23_SOURCES, C01_37_SOURCES, C01_37_CLAIMS, C01_37_HOLDERS = 49, 31, 48, 16
ACCESSED = '2026-09-30'

# Original response identity recorded in each extract: (bytes, sha256), the body as received.
RESPONSES = {
    'fr_jorf_nomination_valls_20140331': (97353, 'af417b523e6fbaf859c3721a23be1982e7b0fdba7213288e11a499810a3d4903'),
    'fr_jorf_cessation_valls_20140825': (14083, '3f80f2659faf3cf93ee39b4995bf85b42fe84fc399051026863b5e309a829d81'),
    'fr_jorf_nomination_valls_20140825': (14013, '469ffc07bf47b488d2ab34812ae25cadae727ee87deb36e171b584d5f183ed85'),
    'fr_jorf_cessation_valls_20161206': (96343, '54d623b86b40ef99cd8085e61bf5138b9ed8b5807a919d3d96f8c08793075092'),
    'fr_jorf_nomination_cazeneuve_20161206': (12977, '96e3d3bdf8cec9af7c3b82ea612d4b9c794b34948622de6bfee91646be0d716f'),
    'fr_jorf_cessation_cazeneuve_20170510': (96449, 'f6a550147079cc67bfcec2348012bf68e8fa8cb6efcf104f7cb9c9c335c38928'),
    'fr_jorf_nomination_philippe_20170515': (86327, '32ee9316e3f39e5fc01eed7ab0fd303d23f7a89d5f85e169b192f3f2cadecc6b'),
    'fr_jorf_cessation_philippe_20170619': (96167, 'e7ddb2b69e5bca3d94064b86335abd364e1819455c529375248433ed80ece197'),
    'fr_jorf_nomination_philippe_20170619': (14589, '3b6355eae43497b22615b422beec8ebe4af6f4f07935c3ef2387c0ba93140d95'),
    'fr_jorf_cessation_philippe_20200703': (17202, '8388ace1a15a0f2aa411a62fd8ebbf3b951935c45302382595ef72db259b785b'),
    'fr_jorf_nomination_castex_20200703': (96081, '9991cb10bddd832f38331c1d803a914dd6d7927244e1e14832e9798415f5f683'),
    'fr_jorf_cessation_castex_20220516': (96124, '3cdc39bf9a6c4deda11e5d9e88bd5ae043009f6d77b04b6c17c33bffcc66924e'),
    'fr_jorf_nomination_borne_20220516': (100491, 'b0278dd3e9c4e8bb74eac936535bf47b5b00e303615fcb058d5afab80c12146c'),
    'fr_jorf_cessation_borne_20240109': (90564, '9ae7b26bb02bf94052688f64651d6b1eb7ee32703b16595bd8321967df6cac1e'),
    'fr_jorf_nomination_attal_20240109': (95344, '553a022ad3c6cff7c5f835e67a6e93a0472582c81443cdc7c2753cb4b6905a6d'),
    'fr_jorf_cessation_attal_20240716': (96301, '6dce5f1e87cf6c5a757d5fd1f478a303fcdf1de56f0d761cec04d6710c1b4975'),
    'fr_jorf_nomination_barnier_20240905': (100390, '9a49893a6bf70dda0b494c486792673c4069e8e5c2510fe4fb834bd1703a2a0b'),
    'fr_jorf_nomination_bayrou_20241213': (17546, '16b8582ae70e39de3ff419dc4c8384d00dd7ab033727136e614b00b1b94bc679'),
    'fr_jorf_dila_opendata_20250910': (133987, '74892eb74e45ff28ddbbdd10dd13c1dc84bd0465134a0fdfe20ffb79f69036d9'),
    'fr_jorf_dila_opendata_20251007': (54757, 'a8b48bc2c874d4e1f96c392c5b1d8316d8caba8d865359b0758fa52b83cbd9cb'),
    'fr_jorf_dila_opendata_20251011': (102454, '31b5f8aa376205754ec39584da4d6bf31d209e35c40b3510b2f2a68baa480072'),
    'fr_jorf_dila_opendata_pm_signature_20260904': (126639, 'edec7f27c7326f96effe331d76f3288195e0b210de8798472b0365b189d6ad1b'),
}
# Raw Internet Archive captures of Légifrance text pages (capture timestamp).
ARCHIVED = {
    'fr_jorf_nomination_valls_20140331': '20250401101344',
    'fr_jorf_cessation_valls_20140825': '20160317061550',
    'fr_jorf_nomination_valls_20140825': '20200614082953',
    'fr_jorf_cessation_valls_20161206': '20240906195146',
    'fr_jorf_nomination_cazeneuve_20161206': '20221102101154',
    'fr_jorf_cessation_cazeneuve_20170510': '20241205103833',
    'fr_jorf_nomination_philippe_20170515': '20220321231409',
    'fr_jorf_cessation_philippe_20170619': '20241111072049',
    'fr_jorf_nomination_philippe_20170619': '20240110213603',
    'fr_jorf_cessation_philippe_20200703': '20240110213556',
    'fr_jorf_nomination_castex_20200703': '20241204221633',
    'fr_jorf_cessation_castex_20220516': '20250123185209',
    'fr_jorf_nomination_borne_20220516': '20241202161449',
    'fr_jorf_cessation_borne_20240109': '20240110101925',
    'fr_jorf_nomination_attal_20240109': '20240301021148',
    'fr_jorf_cessation_attal_20240716': '20240829143727',
    'fr_jorf_nomination_barnier_20240905': '20240912183250',
    'fr_jorf_nomination_bayrou_20241213': '20241226115851',
}
GZIP = {'fr_jorf_nomination_cazeneuve_20161206', 'fr_jorf_nomination_philippe_20170619', 'fr_jorf_cessation_philippe_20200703', 'fr_jorf_nomination_bayrou_20241213'}
DECODED = {
    'fr_jorf_nomination_cazeneuve_20161206': (87692, 'a3db3895de84654270b257d6db4c6a5419f9c2f8a6ea310e09384ce218edd697'),
    'fr_jorf_nomination_philippe_20170619': (94659, '6807e9ab771e8755d7a623b3c3c7d7ed1467fa0a5e09892bcc6e3211e013de3f'),
    'fr_jorf_cessation_philippe_20200703': (90450, '751b41d07dba38384b691d8874810600c5f1635cc1c4df0c9d7f2de88269f77c'),
    'fr_jorf_nomination_bayrou_20241213': (100274, '9771019131a90ab400073aaa4c5ff2fcd4a2f69f0fd19444852ec5caa8b49b43'),
}
# DILA daily Journal officiel open-data exports (file name).
DILA = {
    'fr_jorf_dila_opendata_20250910': 'JORF_20250910-005809',
    'fr_jorf_dila_opendata_20251007': 'JORF_20251007-001919',
    'fr_jorf_dila_opendata_20251011': 'JORF_20251011-005544',
    'fr_jorf_dila_opendata_pm_signature_20260904': 'JORF_20260905-004620',
}

NEW_SOURCES = list(RESPONSES)
WAYBACK = [sid for sid in NEW_SOURCES if sid in ARCHIVED]
DILA_SOURCES = [sid for sid in NEW_SOURCES if sid in DILA]

EVENTS = {
    'fr_jorf_valls_appointed_pm_20140331': ('2014-03-31', 'appointment_decree', 'FR-PM-13'),
    'fr_jorf_valls_government_resignation_letter_20140825': ('2014-08-25', 'government_resignation_presented', 'FR-PM-13'),
    'fr_jorf_valls_functions_ended_20140825': ('2014-08-25', 'cessation_of_functions_decree', 'FR-PM-13'),
    'fr_jorf_valls_appointed_pm_20140825': ('2014-08-25', 'appointment_decree', 'FR-PM-13'),
    'fr_jorf_valls_government_resignation_letter_20161206': ('2016-12-06', 'government_resignation_presented', 'FR-PM-13'),
    'fr_jorf_valls_functions_ended_20161206': ('2016-12-06', 'cessation_of_functions_decree', 'FR-PM-13'),
    'fr_jorf_cazeneuve_appointed_pm_20161206': ('2016-12-06', 'appointment_decree', 'FR-PM-14'),
    'fr_jorf_cazeneuve_government_resignation_letter_20170510': ('2017-05-10', 'government_resignation_presented', 'FR-PM-14'),
    'fr_jorf_cazeneuve_functions_ended_20170510': ('2017-05-10', 'cessation_of_functions_decree', 'FR-PM-14'),
    'fr_jorf_philippe_appointed_pm_20170515': ('2017-05-15', 'appointment_decree', 'FR-PM-15'),
    'fr_jorf_philippe_government_resignation_letter_20170619': ('2017-06-19', 'government_resignation_presented', 'FR-PM-15'),
    'fr_jorf_philippe_functions_ended_20170619': ('2017-06-19', 'cessation_of_functions_decree', 'FR-PM-15'),
    'fr_jorf_philippe_appointed_pm_20170619': ('2017-06-19', 'appointment_decree', 'FR-PM-15'),
    'fr_jorf_philippe_government_resignation_letter_20200702': ('2020-07-02', 'government_resignation_presented', 'FR-PM-15'),
    'fr_jorf_philippe_functions_ended_20200703': ('2020-07-03', 'cessation_of_functions_decree', 'FR-PM-15'),
    'fr_jorf_castex_appointed_pm_20200703': ('2020-07-03', 'appointment_decree', 'FR-PM-16'),
    'fr_jorf_castex_government_resignation_letter_20220516': ('2022-05-16', 'government_resignation_presented', 'FR-PM-16'),
    'fr_jorf_castex_functions_ended_20220516': ('2022-05-16', 'cessation_of_functions_decree', 'FR-PM-16'),
    'fr_jorf_borne_appointed_pm_20220516': ('2022-05-16', 'appointment_decree', 'FR-PM-17'),
    'fr_jorf_borne_government_resignation_letter_20240108': ('2024-01-08', 'government_resignation_presented', 'FR-PM-17'),
    'fr_jorf_borne_functions_ended_20240109': ('2024-01-09', 'cessation_of_functions_decree', 'FR-PM-17'),
    'fr_jorf_attal_appointed_pm_20240109': ('2024-01-09', 'appointment_decree', 'FR-PM-18'),
    'fr_jorf_attal_government_resignation_letter_20240708': ('2024-07-08', 'government_resignation_presented', 'FR-PM-18'),
    'fr_jorf_attal_functions_ended_20240716': ('2024-07-16', 'cessation_of_functions_decree', 'FR-PM-18'),
    'fr_jorf_barnier_appointed_pm_20240905': ('2024-09-05', 'appointment_decree', 'FR-PM-19'),
    'fr_jorf_bayrou_appointed_pm_20241213': ('2024-12-13', 'appointment_decree', 'FR-PM-20'),
    'fr_jorf_bayrou_government_resignation_letter_20250909': ('2025-09-09', 'government_resignation_presented', 'FR-PM-20'),
    'fr_jorf_bayrou_functions_ended_20250909': ('2025-09-09', 'cessation_of_functions_decree', 'FR-PM-20'),
    'fr_jorf_lecornu_appointed_pm_20250909': ('2025-09-09', 'appointment_decree', 'FR-PM-21'),
    'fr_jorf_lecornu_government_resignation_letter_20251006': ('2025-10-06', 'government_resignation_presented', 'FR-PM-21'),
    'fr_jorf_lecornu_functions_ended_20251006': ('2025-10-06', 'cessation_of_functions_decree', 'FR-PM-21'),
    'fr_jorf_lecornu_appointed_pm_20251010': ('2025-10-10', 'appointment_decree', 'FR-PM-21'),
    'fr_jorf_lecornu_signs_decree_as_pm_20260904': ('2026-09-04', 'in_office_signature_as_pm', 'FR-PM-21'),
}
HOLDERS = [
    ('Manuel Valls', '2014-03-31', None, None), ('Manuel Valls', '2014-08-25', None, None),
    ('Bernard Cazeneuve', '2016-12-06', None, None),
    ('Edouard Philippe', '2017-05-15', None, None), ('Edouard Philippe', '2017-06-19', None, None),
    ('Jean Castex', '2020-07-03', None, None), ('Elisabeth Borne', '2022-05-16', None, None),
    ('Gabriel Attal', '2024-01-09', None, None), ('Michel Barnier', '2024-09-05', None, None),
    ('François Bayrou', '2024-12-13', None, None),
    ('Sébastien Lecornu', '2025-09-09', None, None), ('Sébastien Lecornu', '2025-10-10', None, None),
]
HOLDER_CLAIMS = [
    ['fr_jorf_valls_appointed_pm_20140331', 'fr_jorf_valls_functions_ended_20140825'],
    ['fr_jorf_valls_appointed_pm_20140825', 'fr_jorf_valls_functions_ended_20161206'],
    ['fr_jorf_cazeneuve_appointed_pm_20161206', 'fr_jorf_cazeneuve_functions_ended_20170510'],
    ['fr_jorf_philippe_appointed_pm_20170515', 'fr_jorf_philippe_functions_ended_20170619'],
    ['fr_jorf_philippe_appointed_pm_20170619', 'fr_jorf_philippe_functions_ended_20200703'],
    ['fr_jorf_castex_appointed_pm_20200703', 'fr_jorf_castex_functions_ended_20220516'],
    ['fr_jorf_borne_appointed_pm_20220516', 'fr_jorf_borne_functions_ended_20240109'],
    ['fr_jorf_attal_appointed_pm_20240109', 'fr_jorf_attal_functions_ended_20240716'],
    ['fr_jorf_barnier_appointed_pm_20240905'],
    ['fr_jorf_bayrou_appointed_pm_20241213', 'fr_jorf_bayrou_functions_ended_20250909'],
    ['fr_jorf_lecornu_appointed_pm_20250909', 'fr_jorf_lecornu_functions_ended_20251006'],
    ['fr_jorf_lecornu_appointed_pm_20251010', 'fr_jorf_lecornu_signs_decree_as_pm_20260904'],
]
PEOPLE = ['Manuel Valls', 'Bernard Cazeneuve', 'Edouard Philippe', 'Jean Castex', 'Elisabeth Borne', 'Gabriel Attal', 'Michel Barnier',
          'François Bayrou', 'Sébastien Lecornu']
SURNAMES = {name: name.split()[-1].casefold() for name in PEOPLE}
HOLDER_KINDS = {'appointment_decree', 'cessation_of_functions_decree', 'in_office_signature_as_pm'}
NEVER_KINDS = {'government_resignation_presented'}
NEVER_HOLDER = tuple(cid for cid, (_, kind, _) in EVENTS.items() if kind in NEVER_KINDS)
# Publication days, resignation letters, cessation decrees, successors' appointments and the in-office signature are never a boundary.
NEVER_BOUNDARY = {
    0: {'2014-04-01', '2014-08-25', '2014-08-26'}, 1: {'2014-08-26', '2016-12-06', '2016-12-07'},
    2: {'2016-12-07', '2017-05-10', '2017-05-11', '2017-05-15'}, 3: {'2017-05-16', '2017-06-20'},
    4: {'2017-06-20', '2020-07-02', '2020-07-04'}, 5: {'2020-07-04', '2022-05-17'},
    6: {'2022-05-17', '2024-01-08', '2024-01-10'}, 7: {'2024-01-10', '2024-07-08', '2024-07-16', '2024-07-17', '2024-09-05'},
    8: {'2024-09-06', '2024-12-05', '2024-12-13'}, 9: {'2024-12-14', '2025-09-10'}, 10: {'2025-09-10', '2025-10-07', '2025-10-10'},
    11: {'2025-10-11', '2026-09-04', '2026-09-05', '2026-09-07'}}
CESSATION_BASIS = {i: ids[-1] for i, ids in enumerate(HOLDER_CLAIMS) if '_functions_ended_' in ids[-1]}
LEAD_URL_MARKERS = ('wikipedia', 'attribmin', 'persee', 'info.gouv.fr', 'gouvernement.fr', 'assemblee-nationale.fr', 'elysee.fr',
                    'france-politique', '/jorf/jo/')
VOLATILE_URL = re.compile(r'([?&](cb|_|_cb|nocache|token|sig|exp|s|q|query|page|dateTexte|categorieLien|oldAction)=|jsessionid|'
                          r'/recherche|/search)', re.I)
REPORT = research.RESEARCH / 'france-prime-ministers-2014-2026-38.md'
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-38.md'


def batch_invariants(packet):
    """Packet-level rules this test owns; raises AssertionError, KeyError or IndexError on any violation."""
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    owner = {c['id']: s['id'] for s in packet['sources'] for c in s['claims']}
    assert [e['id'] for e in packet['institutions']] == [PRESIDENCY, INSTITUTION], 'presidency, then the prime ministership'
    pm = packet['institutions'][1]
    assert pm['represented_party_ids'] == [] and pm['reconciled_organization_id'] is None
    assert pm['lifecycle']['status'] == 'unknown' and pm['lifecycle']['from'] is None and pm['lifecycle']['until'] is None
    assert [(r['id'], r['kind'], r['title']) for r in pm['roles']] == [(ROLE, 'head_of_government', 'Premier ministre')]
    heads = [r['id'] for e in packet['organizations'] + packet['institutions'] for r in e['roles'] if r['kind'] == 'head_of_government']
    assert heads == [ROLE], 'no other head-of-government role'
    role = pm['roles'][0]
    new_claims, new_sources = list(EVENTS), NEW_SOURCES
    # CLAUDE-C01-37's first batch comes first and cites none of this batch; this batch is appended in date order.
    assert role['claim_ids'] == pm['claim_ids'] and role['sources'] == pm['sources']
    assert len(role['claim_ids']) == C01_37_CLAIMS + len(new_claims) and role['claim_ids'][C01_37_CLAIMS:] == new_claims
    assert len(role['sources']) == C01_37_SOURCES + len(new_sources) and role['sources'][C01_37_SOURCES:] == new_sources
    holders = role['holder_claims']
    assert all(isinstance(h, dict) for h in holders) and len(holders) == C01_37_HOLDERS + len(HOLDERS)
    for h in holders[:C01_37_HOLDERS]:
        assert not set(h['claim_ids']) & set(new_claims) and not set(h['sources']) & set(new_sources), h['name']
    batch = holders[C01_37_HOLDERS:]
    assert [(h['name'], h['attested_on'], h['from'], h['until']) for h in batch] == HOLDERS
    assert [h['claim_ids'] for h in batch] == HOLDER_CLAIMS
    assert sorted({h['name'] for h in batch}, key=PEOPLE.index) == PEOPLE and len(PEOPLE) <= 10, 'at most ten people'
    assert not {h['name'] for h in batch} & {h['name'] for h in holders[:C01_37_HOLDERS]}, 'a new person, not a first-batch holder'
    for index, h in enumerate(batch):
        assert not set(h['claim_ids']) & set(NEVER_HOLDER), h['name']
        assert not {h['attested_on'], h['from'], h['until']} & NEVER_BOUNDARY[index], (index, h['name'])
        assert set(h['claim_ids']) <= set(new_claims) and set(h['sources']) <= set(new_sources), 'no cross-role claim feeds a holder'
        for cid in h['claim_ids']:
            assert cid in role['claim_ids'] and owner[cid] in h['sources'], cid
        # Dated source observations only: signing/publication is not explicit effect evidence (CLAUDE-C01-37 review contract).
        assert h['from'] is None and h['until'] is None, 'no inferred effective boundaries'
        basis = claims[h['claim_ids'][0]]
        assert basis['attested_on'] == h['attested_on'] and re.search(r"est nommée? (Premier|Première) ministre", basis['text']), index
        if index in CESSATION_BASIS:
            assert h['claim_ids'][-1] == CESSATION_BASIS[index] and 'Il est mis fin' in claims[CESSATION_BASIS[index]]['text'], index
    for cid, (day, _, _) in EVENTS.items():
        assert claims[cid].get('attested_on') == day, cid
    # Separation: no organization and not the presidency cites this batch.
    for entry in packet['organizations']:
        assert not set(entry['claim_ids']) & set(new_claims) and not set(entry['sources']) & set(new_sources), entry['id']
        assert entry['roles'] == [] and entry['represented_party_ids'] == [], entry['id']
    presidency = packet['institutions'][0]
    assert not set(presidency['claim_ids']) & set(new_claims) and not set(presidency['sources']) & set(new_sources)
    for r in presidency['roles']:
        assert not set(r['claim_ids']) & set(new_claims) and not set(r['sources']) & set(new_sources)
        for h in r['holder_claims']:
            assert not set(h['claim_ids']) & set(new_claims)


class FrancePrimeMinistersSecondBatchTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.csv = (importer.ROOT / importer.RAW).read_bytes()
        cls.raw = (research.ROOT / research.RESEARCH / 'france.json').read_text(encoding='utf-8')
        cls.packet = json.loads(cls.raw)
        cls.supplement_bytes = (importer.ROOT / importer.SUPPLEMENT).read_bytes()
        cls.supplement = json.loads(cls.supplement_bytes)
        cls.sources = {s['id']: s for s in cls.packet['sources']}
        cls.claims = {c['id']: c for s in cls.packet['sources'] for c in s['claims']}
        cls.claim_source = {c['id']: s['id'] for s in cls.packet['sources'] for c in s['claims']}
        cls.pm = cls.packet['institutions'][1]
        cls.role = cls.pm['roles'][0]
        cls.batch = cls.role['holder_claims'][C01_37_HOLDERS:]
        cls.extracts = {sid: json.loads((research.ROOT / cls.sources[sid]['snapshot']['path']).read_text(encoding='utf-8'))
                        for sid in NEW_SOURCES}
        cls.rows = {row['claim_id']: row for sid in NEW_SOURCES for row in cls.extracts[sid]['rows']}
        cls.new_claims = [c['id'] for sid in NEW_SOURCES for c in cls.sources[sid]['claims']]
        cls.report = (research.ROOT / REPORT).read_text(encoding='utf-8')

    def validate(self, packet=None):
        return research.validate(packet or self.packet, research.ROOT, {'France'}, {'France': set()})

    def section(self, heading):
        parts = self.report.split(f'\n## {heading}', 1)
        self.assertEqual(len(parts), 2, heading)
        return parts[1].split('\n## ', 1)[0]

    def test_packet_is_the_importer_output_and_only_appends_to_the_supplement(self):
        built = importer.build(self.csv)
        self.assertEqual(self.raw, json.dumps(built, ensure_ascii=False, indent=2) + '\n')
        self.assertNotIn(b'\r', self.supplement_bytes)
        self.assertEqual(self.supplement_bytes.decode('utf-8'), json.dumps(self.supplement, indent=2, ensure_ascii=False) + '\n')
        ids = [s['id'] for s in self.supplement['sources']]
        first = C01_23_SOURCES + C01_37_SOURCES
        self.assertEqual(len(ids), first + len(NEW_SOURCES))
        self.assertEqual(ids[first:], NEW_SOURCES)
        self.assertEqual([s['id'] for s in self.packet['sources']][-len(NEW_SOURCES):], NEW_SOURCES)
        self.assertEqual(self.packet['institutions'], self.supplement['institutions'])
        notes = self.supplement['coverage_unresolved']
        self.assertEqual(len(notes), 3)
        self.assertTrue(notes[2].startswith('CLAUDE-C01-38 extends fr_prime_minister'))
        self.assertEqual(self.packet['coverage']['unresolved'][-3:], notes)
        # The first batch's text is kept and this batch only appends to it.
        scope = self.role['scope_note']
        self.assertTrue(scope.startswith('Premier ministre under the Constitution of 4 October 1958 (article 8)'))
        self.assertIn('first ten people to hold the office in the period (Michel Rocard to Jean-Marc Ayrault)', scope)
        self.assertIn(' CLAUDE-C01-38 extends this role with the next batch', scope)
        unresolved = self.pm['coverage']['unresolved']
        self.assertEqual(len(unresolved), 7)
        self.assertTrue(unresolved[3].startswith('Next batch: the Prime Ministers appointed from 31 March 2014'))
        self.assertTrue(unresolved[4].startswith('CLAUDE-C01-38 retains 22 primary source responses and 33 dated claims'))
        self.assertIn('JORFTEXT000050748889', unresolved[5])
        self.assertIn('re-verify them before then', unresolved[6])
        base = importer.build(self.csv, supplement=b'{"sources": [], "institutions": [], "coverage_unresolved": []}')
        self.assertEqual(self.packet['organizations'], base['organizations'])
        self.assertEqual(len(base['organizations']), 635)

    def test_new_records_are_bounded_and_every_claim_is_classified(self):
        ids = self.validate()
        self.assertEqual((len(NEW_SOURCES), len(self.new_claims)), (22, 33))
        self.assertEqual((len(WAYBACK), len(DILA_SOURCES)), (18, 4))
        self.assertEqual(len(ids['entries']), 637)
        self.assertEqual(ids['roles'], {PRESIDENT, ROLE})
        self.assertEqual(self.new_claims, list(EVENTS))
        holder_claims = {cid for ids_ in HOLDER_CLAIMS for cid in ids_}
        self.assertFalse(holder_claims & set(NEVER_HOLDER))
        self.assertEqual({self.rows[cid]['event_kind'] for cid in holder_claims}, HOLDER_KINDS)
        self.assertEqual({kind for _, kind, _ in EVENTS.values()}, NEVER_KINDS | HOLDER_KINDS)
        self.assertEqual(self.pm['claim_ids'][C01_37_CLAIMS:], self.new_claims)
        self.assertEqual(self.pm['sources'][C01_37_SOURCES:], NEW_SOURCES)
        observations = re.findall(r'^### (FR-PM-\d\d)\b', self.report, re.M)
        self.assertEqual(observations, [f'FR-PM-{n:02d}' for n in range(13, 23)])
        # FR-PM-22 (current affairs, acting service) records an absence and owns no row.
        self.assertEqual({row['review_observation'] for row in self.rows.values()}, {f'FR-PM-{n:02d}' for n in range(13, 22)})
        for cid in EVENTS:
            self.assertTrue(self.rows[cid]['holder_name'], cid)
        # Ten decrees end a Government's functions, each citing its own letter; one in-office signature; no acting holder.
        kinds = [kind for _, kind, _ in EVENTS.values()]
        self.assertEqual((kinds.count('appointment_decree'), kinds.count('cessation_of_functions_decree'),
                          kinds.count('government_resignation_presented'), kinds.count('in_office_signature_as_pm')), (12, 10, 10, 1))

    def test_holders_are_exactly_as_intended(self):
        batch_invariants(self.packet)
        for holder in self.batch:
            self.assertTrue(holder['note'].startswith('Observed on ') and 'No start' in holder['uncertainty'], holder['name'])
            expected = []
            for cid in holder['claim_ids']:
                if self.claim_source[cid] not in expected:
                    expected.append(self.claim_source[cid])
            self.assertEqual(holder['sources'], expected, holder['name'])
            for cid in holder['claim_ids']:
                row = self.rows[cid]
                self.assertEqual((row['role_id'], row['observation_id']), (ROLE, INSTITUTION), cid)
                self.assertIn(row['event_kind'], HOLDER_KINDS, cid)
                self.assertIn(SURNAMES[holder['name']], row['holder_name'].casefold(), cid)
        # Names are as printed (case aside): the Journal officiel prints 'Edouard' and 'Elisabeth' without an accent.
        self.assertIn("'M. Edouard Philippe est nommé Premier ministre'", self.claims['fr_jorf_philippe_appointed_pm_20170515']['text'])
        self.assertIn("'Mme Elisabeth BORNE est nommée Première ministre'", self.claims['fr_jorf_borne_appointed_pm_20220516']['text'])
        # Michel Barnier's cessation text was not retrieved, so his observation cites the appointment only.
        self.assertIn('JORFTEXT000050748889', self.batch[8]['note'])
        self.assertIn('4 September 2026', self.batch[11]['note'])
        for cid in NEVER_HOLDER:
            self.assertRegex(self.claims[cid]['uncertainty'], r'never an end', cid)

    def test_distinct_events_keep_distinct_claims(self):
        claims = self.claims
        letters = [cid for cid, (_, kind, _) in EVENTS.items() if kind == 'government_resignation_presented']
        ended = [cid for cid, (_, kind, _) in EVENTS.items() if kind == 'cessation_of_functions_decree']
        self.assertEqual(len(letters), len(ended))
        for letter, end in zip(letters, ended):
            self.assertEqual(self.claim_source[letter], self.claim_source[end])
            self.assertLessEqual(claims[letter]['attested_on'], claims[end]['attested_on'])
        # Three letters predate their decree; they stay separate dated claims and say so.
        for letter, end in (('fr_jorf_philippe_government_resignation_letter_20200702', 'fr_jorf_philippe_functions_ended_20200703'),
                            ('fr_jorf_borne_government_resignation_letter_20240108', 'fr_jorf_borne_functions_ended_20240109'),
                            ('fr_jorf_attal_government_resignation_letter_20240708', 'fr_jorf_attal_functions_ended_20240716')):
            self.assertLess(claims[letter]['attested_on'], claims[end]['attested_on'])
            self.assertIn('before the decree signed', claims[letter]['uncertainty'])
        # Same-day cessation and appointment are separate decrees (two sources, or two records of one export).
        for cess, nom in (('fr_jorf_valls_functions_ended_20140825', 'fr_jorf_valls_appointed_pm_20140825'),
                          ('fr_jorf_valls_functions_ended_20161206', 'fr_jorf_cazeneuve_appointed_pm_20161206'),
                          ('fr_jorf_philippe_functions_ended_20170619', 'fr_jorf_philippe_appointed_pm_20170619'),
                          ('fr_jorf_philippe_functions_ended_20200703', 'fr_jorf_castex_appointed_pm_20200703'),
                          ('fr_jorf_castex_functions_ended_20220516', 'fr_jorf_borne_appointed_pm_20220516'),
                          ('fr_jorf_borne_functions_ended_20240109', 'fr_jorf_attal_appointed_pm_20240109')):
            self.assertNotEqual(self.claim_source[cess], self.claim_source[nom])
            self.assertEqual(claims[cess]['attested_on'], claims[nom]['attested_on'])
        both = [c['id'] for c in self.sources['fr_jorf_dila_opendata_20250910']['claims']]
        self.assertEqual(both, ['fr_jorf_bayrou_government_resignation_letter_20250909', 'fr_jorf_bayrou_functions_ended_20250909',
                                'fr_jorf_lecornu_appointed_pm_20250909'])
        self.assertNotEqual(claims[both[1]]['locator']['archive_member'], claims[both[2]]['locator']['archive_member'])
        # Gaps stay gaps: 10 May to 15 May 2017, 16 July to 5 September 2024, 6 to 10 October 2025.
        for end, start in (('fr_jorf_cazeneuve_functions_ended_20170510', 'fr_jorf_philippe_appointed_pm_20170515'),
                           ('fr_jorf_attal_functions_ended_20240716', 'fr_jorf_barnier_appointed_pm_20240905'),
                           ('fr_jorf_lecornu_functions_ended_20251006', 'fr_jorf_lecornu_appointed_pm_20251010')):
            self.assertLess(claims[end]['attested_on'], claims[start]['attested_on'])

    def test_extracts_match_packet_claims_and_record_original_responses(self):
        for sid in NEW_SOURCES:
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual(extract['format'], 'spheres-c01-derived-factual-table/v1')
            self.assertEqual((extract['source_id'], extract['source_url']), (sid, source['url']))
            self.assertEqual(extract['scope_note'], source['scope_note'])
            self.assertEqual(extract['access_method'], source['access_method'])
            self.assertEqual(extract['published_date'], source['published_date'])
            self.assertEqual((extract['accessed_date'], source['accessed_date']), (ACCESSED, ACCESSED))
            self.assertLessEqual(source['published_date'], research.CUTOFF)
            self.assertFalse(extract['source_response_checked_in'])
            self.assertEqual((extract['source_response_bytes'], extract['source_response_sha256']), RESPONSES[sid])
            for text in ('not checked into this repository', 'derived factual extract', 'same byte count and SHA-256',
                         'no Accept-Encoding header', 'more than 30 minutes apart'):
                self.assertIn(text, extract['provenance_note'], text)
            if sid in GZIP:
                self.assertIn('Content-Encoding: gzip', extract['provenance_note'])
                self.assertIn('the gzip stream', extract['provenance_note'])
                self.assertIn('gzip-compressed', source['scope_note'])
                self.assertEqual((extract['source_response_content_encoding'], extract['decoded_response_bytes'],
                                  extract['decoded_response_sha256']), ('gzip',) + DECODED[sid])
            else:
                self.assertNotIn('decoded_response_sha256', extract)
                self.assertNotIn('source_response_content_encoding', extract)
            self.assertIn('No open license, portrait permission or likeness approval', extract['rights_note'])
            self.assertEqual(extract['rights_note'], source['rights_note'])
            self.assertEqual(extract['visual_review']['pdf_pages_one_based'], [])
            self.assertIn('HTTP 403 on 30 September 2026), which was not bypassed', source['scope_note'])
            self.assertIn('CLAUDE-C01-38 only', extract['bounded_scope'])
            snapshot = source['snapshot']
            self.assertRegex(snapshot['path'], r'^docs/campaign-certification/C01/research/sources/france-(jorf-[a-z0-9-]+|dila-jorf-opendata(-pm-signature)?)-\d{8}-facts\.json$')
            data = (research.ROOT / snapshot['path']).read_bytes()
            self.assertEqual((len(data), hashlib.sha256(data).hexdigest()), (snapshot['bytes'], snapshot['sha256']))
            self.assertNotEqual(snapshot['sha256'], extract['source_response_sha256'])
            self.assertTrue(data.endswith(b'}\n') and b'\r' not in data)
            self.assertEqual(data.decode('utf-8'), json.dumps(extract, indent=2, ensure_ascii=False) + '\n')
            self.assertEqual([r['claim_id'] for r in extract['rows']], [c['id'] for c in source['claims']])
            for row, claim in zip(extract['rows'], source['claims']):
                self.assertEqual((row['text'], row['locator'], row['attested_on']), (claim['text'], claim['locator'], claim['attested_on']))
                self.assertEqual((row['observation_id'], row['role_id'], row['role_title']), (INSTITUTION, ROLE, 'Premier ministre'))
                self.assertNotIn('name', row)
                self.assertEqual((row['attested_on'], row['event_kind'], row['review_observation']), EVENTS[row['claim_id']])
                self.assertTrue(claim['uncertainty'])
        self.assertEqual({cid: (row['attested_on'], row['event_kind'], row['review_observation']) for cid, row in self.rows.items()}, EVENTS)
        for sid, stamp in ARCHIVED.items():
            source, extract = self.sources[sid], self.extracts[sid]
            url = urlsplit(source['url'])
            self.assertEqual(url.hostname, 'web.archive.org')
            self.assertTrue(url.path.startswith(f'/web/{stamp}id_/http'), sid)
            self.assertLess(stamp, '20260907')
            self.assertEqual(extract['original_url'], source['original_url'])
            self.assertEqual(urlsplit(source['original_url']).hostname, 'www.legifrance.gouv.fr')
            self.assertTrue(source['url'].endswith(source['original_url'].split('://', 1)[1]), sid)
            self.assertEqual(extract['archive_capture_utc'].replace('-', '').replace(':', '').replace('T', '').rstrip('Z'), stamp)
            self.assertEqual(source['source_type'], 'primary_official_journal_text_archived')
            self.assertIn('Raw Internet Archive capture', extract['provenance_note'])
            if sid not in GZIP:
                self.assertIn('served without Content-Encoding', extract['provenance_note'])
        for sid, name in DILA.items():
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual(source['url'], f'https://echanges.dila.gouv.fr/OPENDATA/JORF/{name}.tar.gz')
            self.assertNotIn('original_url', source)
            self.assertEqual(source['source_type'], 'primary_official_journal_open_data_export')
            self.assertIn('A stored file downloaded directly from the publisher', extract['provenance_note'])
            for text in ('Last-Modified', 'ETag', 'not a Content-Encoding', 'Retention caveat', 'DATES_EFFET element'):
                self.assertIn(text, source['scope_note'], (sid, text))
            for claim in source['claims']:
                self.assertTrue(claim['locator']['archive_member'].startswith(name.replace('JORF_', '') + '/jorf/global/texte/version/'))
        self.assertEqual(set(ARCHIVED) | set(DILA), set(NEW_SOURCES))
        self.assertFalse(set(ARCHIVED) & set(DILA))

    def test_leads_and_volatile_responses_stay_out_of_the_packet(self):
        for sid in NEW_SOURCES:
            source = self.sources[sid]
            for url in (source['url'], source.get('original_url', '')):
                self.assertIsNone(VOLATILE_URL.search(url), url)
                for marker in LEAD_URL_MARKERS:
                    self.assertNotIn(marker, url, (sid, marker))
        lowered = self.raw.lower()
        for marker in ('wikipedia', 'wikipédia', 'britannica', 'larousse', 'attribmin', 'persee', 'france-politique'):
            self.assertNotIn(marker, lowered, marker)
        # The presidency's own DILA exports (CLAUDE-C01-23) are not reused by this batch.
        self.assertFalse({'fr_jorf_dila_opendata_20260904', 'fr_jorf_dila_opendata_20260905'} & set(NEW_SOURCES))
        self.assertNotIn('JORF_20260905-213606', json.dumps([self.sources[s] for s in NEW_SOURCES], ensure_ascii=False))
        leads, added, attempted = self.section('Leads not imported'), self.section('Sources added'), self.section('Sources attempted')
        for marker in ('Wikipedia', '/jorf/jo/', 'Freemium_jorf_global'):
            self.assertIn(marker, leads, marker)
            self.assertNotIn(marker, added, marker)
        for marker in ('legifrance.gouv.fr', 'HTTP 403', 'not bypassed', 'JORFTEXT000050748889', 'JORFTEXT000052380519'):
            self.assertIn(marker, attempted, marker)

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
            return packet['institutions'][1]['roles'][0]

        def holder(packet, index):
            return role(packet)['holder_claims'][C01_37_HOLDERS + index]

        validator_cases = [
            (lambda p: source(p, 'fr_jorf_nomination_valls_20140331')['snapshot'].update(sha256='0' * 64), 'checksum mismatch'),
            (lambda p: source(p, 'fr_jorf_dila_opendata_20251011')['snapshot'].update(bytes=1), 'checksum mismatch'),
            (lambda p: holder(p, 11).update(until='2026-09-08'), 'exceeds cutoff'),
            (lambda p: claim(p, 'fr_jorf_lecornu_signs_decree_as_pm_20260904').update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: holder(p, 5).update({'from': '2020-07-03', 'until': '2020-07-02'}), 'Reversed historical interval'),
            (lambda p: holder(p, 8)['claim_ids'].append('fr_jorf_bayrou_appointed_pm_20241213'), 'cited source'),
            (lambda p: role(p)['claim_ids'].append('fr_does_not_exist'), 'Unknown'),
        ]
        for change, message in validator_cases:
            with self.subTest(message=message), self.assertRaisesRegex(ValueError, message):
                self.validate(mutated(change))
        acting = {'name': 'Acting Prime Minister', 'attested_on': '2024-07-16', 'from': None, 'until': None,
                  'sources': ['fr_jorf_cessation_attal_20240716'], 'claim_ids': ['fr_jorf_attal_functions_ended_20240716'],
                  'note': 'Continued handling of current affairs.', 'uncertainty': 'No end.'}
        invariant_cases = [
            ('cessation decree used as an end (Valls 2014)', lambda p: holder(p, 0).update(until='2014-08-25')),
            ("successor's appointment used as an end (Barnier)", lambda p: holder(p, 8).update(until='2024-12-13')),
            ('resignation letter used as an end (Attal)', lambda p: holder(p, 7).update(until='2024-07-08')),
            ('in-office signature used as an end (Lecornu)', lambda p: holder(p, 11).update(until='2026-09-04')),
            ('publication used as a start (Castex)', lambda p: holder(p, 5).update({'from': '2020-07-04', 'attested_on': None})),
            ('appointment decree promoted to a start (Borne)', lambda p: holder(p, 6).update({'from': '2022-05-16', 'attested_on': None})),
            ('in-office signature used as the observation (Lecornu)', lambda p: holder(p, 11).update(attested_on='2026-09-04')),
            ('accent added to a printed name (Philippe)', lambda p: holder(p, 3).update(name='Édouard Philippe')),
            ('acting holder added', lambda p: role(p)['holder_claims'].insert(C01_37_HOLDERS + 8, acting)),
            ('resignation claim cited by a holder', lambda p: holder(p, 9)['claim_ids'].append(
                'fr_jorf_bayrou_government_resignation_letter_20250909')),
            ('presidency claim feeding a prime-minister holder', lambda p: holder(p, 11)['claim_ids'].append(
                'fr_jorf_decree_signed_macron_20260904')),
            ('prime-minister claim added to the presidency', lambda p: p['institutions'][0]['roles'][0]['claim_ids'].append(
                'fr_jorf_lecornu_appointed_pm_20251010')),
            ('first-batch holder citing this batch', lambda p: role(p)['holder_claims'][15]['claim_ids'].append(
                'fr_jorf_valls_appointed_pm_20140331')),
            ('holders reordered', lambda p: role(p)['holder_claims'].__setitem__(
                slice(C01_37_HOLDERS, None), role(p)['holder_claims'][C01_37_HOLDERS:][::-1])),
            ('holder removed', lambda p: role(p)['holder_claims'].pop(C01_37_HOLDERS + 4)),
            ('extra person added', lambda p: role(p)['holder_claims'].append(dict(copy.deepcopy(holder(p, 11)), name='Next Batch Holder'))),
            ('claim re-dated to its publication', lambda p: claim(p, 'fr_jorf_lecornu_appointed_pm_20251010').update(attested_on='2025-10-11')),
            ('cessation evidence dropped', lambda p: holder(p, 0)['claim_ids'].pop()),
            ('batch claims moved before the first batch', lambda p: role(p)['claim_ids'].reverse()),
            ('institution mapped to a party', lambda p: p['institutions'][1]['represented_party_ids'].append('France/fr_ps')),
            ('organization citing a prime-minister claim', lambda p: p['organizations'][0]['claim_ids'].append(
                'fr_jorf_barnier_appointed_pm_20240905')),
        ]
        batch_invariants(self.packet)
        for label, change in invariant_cases:
            with self.subTest(label=label), self.assertRaises((AssertionError, KeyError, IndexError)):
                batch_invariants(mutated(change))

    def test_report_and_handoff_close_no_parent_gate(self):
        self.assertIn('ready_for_review', self.report)
        table = self.section('Outcome')
        decisions = {f'{i:02}': 'Accepted in part' for i in range(13, 22)}
        decisions['22'] = 'Accepted'
        for number, decision in decisions.items():
            row, = [line for line in table.splitlines() if line.startswith(f'| FR-PM-{number} ')]
            self.assertIn(f'**{decision}:**', row)
        self.assertIn('\n### Date ledger', self.report)
        for heading in ('Sources added', 'Response identities and stability checks', 'Sources attempted', 'Leads not imported',
                        'Next work', 'Integration notes', 'Checks'):
            self.assertIn(f'\n## {heading}', self.report)
        for marker in ('C01', 'C06', 'S23', 'WC1', 'CP1'):
            self.assertNotRegex(self.report, rf'\b{marker}\b[^.\n]*\bis (now )?complete\b')
        notes = self.section('Integration notes')
        for text in ('f1420ba1', '02d2c5a2', 'research-index.json', 'test_france_prime_ministers_c01_37.py',
                     'test_france_pm_boundaries_review.py', 'supplements/france.json', 'continue existing claims first'):
            self.assertIn(text, notes)
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        for text in ('ready_for_review', 'france-prime-ministers-2014-2026-38.md', 'claude/c01-fr-38', 'f1420ba1', '02d2c5a2',
                     'test_france_prime_ministers_c01_38.py', 'supplements/france.json'):
            self.assertIn(text, handoff)
        index = research.build()
        country = next(p for p in index['countries'] if p['nation'] == 'France')
        self.assertFalse(country['country_census_complete'])
        self.assertEqual((country['institution_observations'], country['role_observations']), (2, 2))
        self.assertFalse(index['c01_complete'])


if __name__ == '__main__':
    unittest.main()
