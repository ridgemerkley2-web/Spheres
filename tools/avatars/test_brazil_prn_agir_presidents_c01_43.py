"""CLAUDE-C01-43: the national presidency of the party registered as the PRN, renamed PTC and then Agir, is one party
office on the AGIR funding observation, kept apart from the Presidency of the Republic and from the PT, PDT and MDB
offices. The court's renamings, the party's conventions, meetings, minutes, rosters and listings stay separate claims; a
holder is dated only by a signed act (a signed convocation included) or a party item of its own day and has no start
or end, because no source states one. The PDS (1980-1993) is claims only."""
import copy
import hashlib
import json
import re
import unittest
from urllib.parse import urlsplit

import campaign_research as research
import test_brazil_party_presidents_c01_34 as bp
import test_brazil_pt_presidents_c01_22 as pt
import test_brazil_vice_presidents_c01_17 as vp

ORG_ID = 'br_tse_fefc_2024_party_07'
ROLE = 'br_agir_president'
T_PRES = 'Presidente Nacional do Partido da Reconstrução Nacional (PRN) / Partido Trabalhista Cristão (PTC) / Agir'
FUNDING = {"election_year": 2024, "release_date": "2024-08-29", "release_cell_as_displayed": "29.8.2024",
           "attested_on": "2024-08-29", "from": None, "until": None, "amount": None, "consolidated_current_status": None}
CASE = '0613121-03.2024.6.00.0000'
# The packet's sources before this packet: the S10f intake (5), CLAUDE-C01-10 (48), CLAUDE-C01-17 (11), CLAUDE-C01-22
# (108) and CLAUDE-C01-34 (76).
EARLIER_SOURCE_COUNT = 248
EARLIER_CLAIM_COUNT = 535

# New sources, in packet order, with the original response identity recorded in each extract: (bytes, sha256).
RESPONSES = {
    "br_agir_tse_partidos_registrados_capt20260802":
        (36548, "63222b4c603a9653a194e1d21930c543d60b7feb072e91a50515f0436b71bd6a"),
    "br_agir_tse_glossario_partidos_capt20230325":
        (127257, "72ec97fd5fe4906d8c8d95f7e134034b211f0da2245adce780e45f5d2dbb6ee8"),
    "br_agir_ptc_historia_partido_capt20071011":
        (27308, "05cd95f56e424e28fccd04f48649a8ede9652666a7bde970f333c8488b9b2345"),
    "br_agir_ptc_comissao_executiva_capt20071020":
        (23214, "2c15b81cab612c24d3ac91e377d2e80e62995952c25818a2ba2e12c38bc40d05"),
    "br_agir_ptc_apoio_aecio_20140516":
        (20490, "68b1d9a12847fc0242912fd9b045a55d98ef78564ffadc0575911e1c71a6e3af"),
    "br_agir_ptc_convencao_aracaju_20150728":
        (28704, "3594488d8e31ac9bc4a3ebc41ed17f04dfd9c008ad249f2e1637a9cd00c4105d"),
    "br_agir_ptc_ata_executiva_20180703":
        (1130004, "a93a97cdc04946e756b8d2d8c719de4b82389c97c5ded1b0f4260990ec6431fb"),
    "br_agir_ptc_edital_convencao_20180719":
        (322372, "fcd6ba94e8a75b5dbd101fc5ce900d66e287c13c49a0546e66fc6956a1367c4d"),
    "br_agir_ptc_resolucao_02_2018_20180725":
        (336809, "2bf6b2aaa64f1524e104482e0218bf9c2c64f49fb150aaf48c590a121e976624"),
    "br_agir_ptc_resolucao_001_2020_20200807":
        (30056, "448fc9f5c5252d5399015e41ed55131c119421219d2b6f47523dc188db44ea34"),
    "br_agir_ptc_agora_agir36_20210601":
        (37384, "29ee50b675f1ba83a4840956b25a3a509452ee2beb437e44b902d6a81c101b85"),
    "br_agir_ptc_comunicado_20210723":
        (27090, "cc13f2566449dad458244b5c4094ff99c7ba9eea419d92b6d1b8c4b9e80de9cf"),
    "br_agir_executiva_resolucao_01_2022_20221111":
        (14315, "f252cd32737f39cd9fdd0c2330433f499c1eeb8191cf650a6ad2b0f7a5d29bf8"),
    "br_agir_plenaria_nacional_capt20240301":
        (127999, "406dbbe231109e96d2f1af44c753de35295a6f2d37ec86efa8dbccea7d1d01ac"),
}
# Raw Internet Archive captures: the 14-digit capture stamp of each source.
ARCHIVED = {
    "br_agir_tse_partidos_registrados_capt20260802": "20260802212048",
    "br_agir_tse_glossario_partidos_capt20230325": "20230325042738",
    "br_agir_ptc_historia_partido_capt20071011": "20071011184316",
    "br_agir_ptc_comissao_executiva_capt20071020": "20071020032949",
    "br_agir_ptc_apoio_aecio_20140516": "20141008035057",
    "br_agir_ptc_convencao_aracaju_20150728": "20160829223324",
    "br_agir_ptc_ata_executiva_20180703": "20181220141003",
    "br_agir_ptc_edital_convencao_20180719": "20181220140928",
    "br_agir_ptc_resolucao_02_2018_20180725": "20181220140943",
    "br_agir_ptc_resolucao_001_2020_20200807": "20200928205237",
    "br_agir_ptc_agora_agir36_20210601": "20210602153422",
    "br_agir_ptc_comunicado_20210723": "20210728155528",
    "br_agir_executiva_resolucao_01_2022_20221111": "20230125002213",
    "br_agir_plenaria_nacional_capt20240301": "20240301164758",
}
# Captures the archive serves gzip-encoded even to an identity request: (encoding, decoded bytes, decoded sha256).
ENCODED = {
    "br_agir_tse_partidos_registrados_capt20260802":
        ("gzip", 148444, "e2abefa96b04cd0eda06b7f20b068a77139b77945ff48d0f70765bdb74b8a376"),
    "br_agir_executiva_resolucao_01_2022_20221111":
        ("gzip", 77862, "80d14ef5acd3ac13acf702901a7d839124b4343b6af509dbb47c7687df8ae20f"),
}
# Scanned PDFs read visually, by one-based page.
PDF_PAGES = {
    'br_agir_ptc_ata_executiva_20180703': [1],
    'br_agir_ptc_edital_convencao_20180719': [1],
    'br_agir_ptc_resolucao_02_2018_20180725': [1],
}
# Every new claim: (review observation, attested_on, event kind, holder named, role-row code). Code 'A' is the role
# title, None a row without holder, 'D' a PDS row (claims only, no observation and no role).
EVENTS = {
    'br_agir_tse_registry_prn_renamed_ptc_20010424':
        ('AGIR-PRES-01', '2001-04-24', 'tse_renaming_decision', None, None),
    'br_agir_tse_registry_ptc_renamed_agir_20220331':
        ('AGIR-PRES-03', '2022-03-31', 'tse_renaming_decision', None, None),
    'br_agir_tse_registry_lists_tourinho_presidente_nacional':
        ('AGIR-PRES-03', None, 'registry_listing', 'Daniel Tourinho', 'A'),
    'br_agir_tse_glossary_prn_definitive_registration_19900222':
        ('AGIR-PRES-01', '1990-02-22', 'tse_registration_decision', None, None),
    'br_agir_tse_glossary_prn_renamed_ptc_20010424':
        ('AGIR-PRES-01', '2001-04-24', 'tse_renaming_decision', None, None),
    'br_pds_tse_glossary_fusion_into_ppr_19930608':
        ('PDS-PRES-01', '1993-06-08', 'tse_merger_decision', None, 'D'),
    'br_agir_history_prn_president_requests_statute_adaptation_1997':
        ('AGIR-PRES-01', None, 'retrospective_statement', 'Daniel Tourinho', 'A'),
    'br_agir_history_prn_requests_renaming_ptc_2000':
        ('AGIR-PRES-01', None, 'retrospective_statement', None, None),
    'br_agir_ptc_roster_tourinho_presidente_capt20071020':
        ('AGIR-PRES-01', None, 'executive_roster_undated', 'Daniel Tourinho', 'A'),
    'br_agir_ptc_tourinho_styled_presidente_nacional_20140516':
        ('AGIR-PRES-02', '2014-05-16', 'in_office_attestation', 'Daniel Tourinho', 'A'),
    'br_agir_ptc_convention_elects_directorate_20150725':
        ('AGIR-PRES-02', '2015-07-25', 'convention_held', None, None),
    'br_agir_ptc_tourinho_opens_convention_as_presidente_20150725':
        ('AGIR-PRES-02', '2015-07-25', 'styled_on_convention_day', 'Daniel Tourinho', 'A'),
    'br_agir_ptc_executive_minutes_presidente_20180703':
        ('AGIR-PRES-02', '2018-07-03', 'executive_minutes_styling', 'Daniel Tourinho', 'A'),
    'br_agir_ptc_tourinho_convokes_convention_20180719':
        ('AGIR-PRES-02', '2018-07-19', 'in_office_attestation', 'Daniel Tourinho', 'A'),
    'br_agir_ptc_tourinho_signs_resolution_02_2018_20180725':
        ('AGIR-PRES-02', '2018-07-25', 'in_office_attestation', 'Daniel Tourinho', 'A'),
    'br_agir_ptc_tourinho_signs_resolution_001_2020_20200807':
        ('AGIR-PRES-02', '2020-08-07', 'in_office_attestation', 'Daniel Tourinho', 'A'),
    'br_agir_ptc_announces_name_change_20210601':
        ('AGIR-PRES-03', '2021-06-01', 'renaming_statement', None, None),
    'br_agir_ptc_tourinho_signs_communique_20210723':
        ('AGIR-PRES-02', '2021-07-23', 'in_office_attestation', 'Daniel Tourinho', 'A'),
    'br_agir_tourinho_styled_presidente_nacional_20221111':
        ('AGIR-PRES-03', '2022-11-11', 'in_office_attestation', 'Daniel Tourinho', 'A'),
    'br_agir_executive_approves_resolution_01_2022_20221110':
        ('AGIR-PRES-03', '2022-11-10', 'executive_meeting_reported', 'Daniel Tourinho', 'A'),
    'br_agir_plenary_tourinho_styled_presidente_do_partido':
        ('AGIR-PRES-03', None, 'styled_undated', 'Daniel Tourinho', 'A'),
}
NEW_SOURCES = list(RESPONSES)
NEW_CLAIMS = list(EVENTS)
# Exact holder observations of br_agir_president, (name, attested_on, from, until), in chronological order.
HOLDERS = [
    ('Daniel Tourinho', '2014-05-16', None, None),
    ('Daniel Tourinho', '2018-07-19', None, None),
    ('Daniel Tourinho', '2018-07-25', None, None),
    ('Daniel Tourinho', '2020-08-07', None, None),
    ('Daniel Tourinho', '2021-07-23', None, None),
    ('Daniel Tourinho', '2022-11-11', None, None),
]
HOLDER_CLAIMS = [
    ['br_agir_ptc_tourinho_styled_presidente_nacional_20140516'],
    ['br_agir_ptc_tourinho_convokes_convention_20180719'],
    ['br_agir_ptc_tourinho_signs_resolution_02_2018_20180725'],
    ['br_agir_ptc_tourinho_signs_resolution_001_2020_20200807'],
    ['br_agir_ptc_tourinho_signs_communique_20210723'],
    ['br_agir_tourinho_styled_presidente_nacional_20221111'],
]
HOLDER_OBSERVATIONS = ['AGIR-PRES-02', 'AGIR-PRES-02', 'AGIR-PRES-02', 'AGIR-PRES-02', 'AGIR-PRES-02', 'AGIR-PRES-03']
HOLDER_KINDS = {'in_office_attestation'}
ORGANIZATION_KINDS = {'tse_renaming_decision', 'tse_registration_decision', 'tse_merger_decision', 'renaming_statement'}
ELECTION_KINDS = {'convention_held', 'styled_on_convention_day', 'executive_meeting_reported', 'executive_minutes_styling'}
UNDATED_KINDS = {'registry_listing', 'retrospective_statement', 'executive_roster_undated', 'styled_undated'}
# Days that are events, never holder dates (2018-07-28 is the convention the signed convocation of 19 July 2018
# convokes; the convocation itself dates an observation and is never a boundary).
NEVER_HOLDER_DATE = {'1990-02-22', '1993-06-08', '2001-04-24', '2015-07-25', '2018-07-03', '2018-07-28',
                     '2021-06-01', '2022-03-31', '2022-11-10'}
CONVOCATION = 'br_agir_ptc_tourinho_convokes_convention_20180719'
PEOPLE = {'Daniel Tourinho'}
REVIEW = ['AGIR-PRES-01', 'AGIR-PRES-02', 'AGIR-PRES-03', 'PDS-PRES-01']
HOSTS = {'web.archive.org'}
LEAD_URL_MARKERS = ('wikipedia', 'politize', 'atribunarj', 'metropoles', 'cmm.am.gov.br', 'fgv.br', 'luiscardoso',
                    'contraponto', 'prezi', 'neamp')
REPORT = research.RESEARCH / 'brazil-prn-ptc-agir-presidents-1990-2026-43.md'
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-43.md'
PER_REQUEST_URL = re.compile(r'cdx/search|[?&]cb=|nocache|cachebust|[?&]_=|/search|[?&]s=|jsessionid|token|timemap|'
                             r'/feed/?$|/embed/?$|wp-json|/tag/|/category/|/page/\d|sgip3', re.I)


def load_rows():
    packet = json.loads((research.ROOT / research.RESEARCH / 'brazil.json').read_text(encoding='utf-8'))
    rows = {}
    for source in packet['sources'][EARLIER_SOURCE_COUNT:]:
        extract = json.loads((research.ROOT / source['snapshot']['path']).read_text(encoding='utf-8'))
        rows.update({row['claim_id']: row for row in extract['rows']})
    return rows


def entry(packet, org_id):
    found, = [o for o in packet['organizations'] if o['id'] == org_id]
    return found


def role_of(packet):
    found, = entry(packet, ORG_ID)['roles']
    return found


def refs(obj):
    cs = set(obj['claim_ids']) | {c for r in obj.get('roles', []) for c in r['claim_ids']} | {
        c for r in obj.get('roles', []) for h in r['holder_claims'] for c in h['claim_ids']}
    ss = set(obj['sources']) | {s for r in obj.get('roles', []) for s in r['sources']} | {
        s for r in obj.get('roles', []) for h in r['holder_claims'] for s in h['sources']}
    return cs, ss


def agir_rules(packet, rows):
    """Rule-based checks that hold without the pinned holder list; raise AssertionError, KeyError or ValueError."""
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    claim_source = {c['id']: s['id'] for s in packet['sources'] for c in s['claims']}
    org = entry(packet, ORG_ID)
    # The funding observation keeps its identity, lifecycle, empty mapping and funding record.
    assert (org['name'], org['kind']) == ('AGIR', 'political_party_election_funding_observation')
    assert org['source_identifier']['value'] == CASE and org['funding_observation'] == FUNDING
    assert org['represented_party_ids'] == [] and org['reconciled_organization_id'] is None
    assert org['lifecycle'] == pt.LIFECYCLE and org['coverage']['unresolved'][:3] == pt.ORG_UNRESOLVED
    assert org['claim_ids'][0] == 'br_tse_fefc_2024_row_07' and org['sources'][0] == 'br_tse_fefc_2024'
    # Exactly one party office here, once.
    assert [(r['id'], r['title'], r['kind']) for r in org['roles']] == [(ROLE, T_PRES, 'party_leader')]
    role = org['roles'][0]
    assert list(role) == ['id', 'title', 'kind', 'sources', 'claim_ids', 'holder_claims', 'scope_note']
    placed = [(e['id'], r['id']) for e in packet['organizations'] + packet['institutions'] for r in e['roles']
              if r['kind'] == 'party_leader' or r['id'] == ROLE or r['title'] == T_PRES]
    assert placed == [(bp.MDB_ORG, bp.MDB), (bp.PDT_ORG, bp.PDT), (pt.ORG_ID, pt.ROLE), (ORG_ID, ROLE)], placed
    r_claims = set(role['claim_ids']) | {c for h in role['holder_claims'] for c in h['claim_ids']}
    r_sources = set(role['sources']) | {s for h in role['holder_claims'] for s in h['sources']}
    assert set(role['claim_ids']) <= set(org['claim_ids']) and set(role['sources']) <= set(org['sources'])
    assert all(c.startswith('br_agir_') for c in r_claims) and all(s.startswith('br_agir_') for s in r_sources)
    # The party office never feeds, or is fed by, any other entry: the presidency, the PT, PDT and MDB offices.
    for other in packet['organizations'] + packet['institutions']:
        if other is not org:
            cs, ss = refs(other)
            assert not cs & r_claims and not ss & r_sources, other['id']
    # PDS rows are claims only: no entry, role or holder cites them.
    pds = {cid for cid, row in rows.items() if cid.startswith('br_pds_')}
    for e in packet['organizations'] + packet['institutions']:
        assert not refs(e)[0] & pds, (e['id'], 'PDS claims are never cited')
    for cid in pds:
        assert (rows[cid]['observation_id'], rows[cid]['role_id'], rows[cid]['holder_name']) == (None, None, None), cid
    for cid in role['claim_ids']:
        row = rows[cid]
        assert (row['observation_id'], row['role_id']) == (ORG_ID, ROLE), cid
        assert row['role_title'] == (T_PRES if row['holder_name'] else None), cid
        assert row['holder_name'] in PEOPLE | {None}, cid
        if row['event_kind'] in UNDATED_KINDS:
            assert 'attested_on' not in claims[cid], (cid, 'undated and retrospective claims carry no structured date')
    previous = ''
    for holder in role['holder_claims']:
        name = holder['name']
        assert list(holder) == ['name', 'attested_on', 'from', 'until', 'sources', 'claim_ids', 'note', 'uncertainty']
        assert name in PEOPLE, name
        # No source states a day of assumption or departure: no start, no end, one observation day each.
        assert holder['from'] is None and holder['until'] is None, (name, 'no stated start or end')
        assert holder['attested_on'] and holder['attested_on'] >= previous, (name, 'chronological')
        assert holder['attested_on'] <= research.CUTOFF
        previous = holder['attested_on']
        assert holder['attested_on'] not in NEVER_HOLDER_DATE, (name, holder['attested_on'])
        expected_sources = []
        for cid in holder['claim_ids']:
            row = rows[cid]
            assert cid in role['claim_ids'] and row['holder_name'] == name and row['role_title'] == T_PRES, cid
            assert row['event_kind'] in HOLDER_KINDS, (name, cid)
            assert claims[cid].get('attested_on') == holder['attested_on'], (name, cid, 'support on another day')
            assert 'tourinho' in claims[cid]['text'].lower(), cid
            if claim_source[cid] not in expected_sources:
                expected_sources.append(claim_source[cid])
        assert holder['sources'] == expected_sources, name
    # A claim dates a holder exactly when its kind can.
    cited = {c for h in role['holder_claims'] for c in h['claim_ids']}
    for cid in role['claim_ids']:
        assert (rows[cid]['event_kind'] in HOLDER_KINDS) == (cid in cited), cid


def agir_invariants(packet, rows):
    """The rules plus the exact pinned holders and events this packet intends."""
    agir_rules(packet, rows)
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    role = role_of(packet)
    assert [(h['name'], h['attested_on'], h['from'], h['until']) for h in role['holder_claims']] == HOLDERS
    assert [h['claim_ids'] for h in role['holder_claims']] == HOLDER_CLAIMS
    for cid, (_, day, _, _, _) in EVENTS.items():
        assert claims[cid].get('attested_on') == day, cid
    # The presidency, vice-presidency, PT, PDT and MDB holders are untouched by this packet.
    president, vice = packet['institutions'][0]['roles']
    assert [(h['name'], h['attested_on'], h['from'], h['until']) for h in president['holder_claims']] == vp.PRESIDENT_HOLDERS
    assert [(h['name'], h['attested_on'], h['from'], h['until']) for h in vice['holder_claims']] == vp.HOLDERS
    pt_role = entry(packet, pt.ORG_ID)['roles'][0]
    assert [(h['name'], h['attested_on'], h['from'], h['until']) for h in pt_role['holder_claims']] == pt.HOLDERS
    for role_id in bp.ROLE_ORG:
        got = [(h['name'], h['attested_on'], h['from'], h['until']) for h in bp.role_of(packet, role_id)['holder_claims']]
        assert got == bp.HOLDERS[role_id], role_id


class BrazilPrnAgirPresidentsTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.raw = (research.ROOT / research.RESEARCH / 'brazil.json').read_text(encoding='utf-8')
        cls.packet = json.loads(cls.raw)
        cls.sources = {s['id']: s for s in cls.packet['sources']}
        cls.claims = {c['id']: c for s in cls.packet['sources'] for c in s['claims']}
        cls.extracts = {sid: json.loads((research.ROOT / cls.sources[sid]['snapshot']['path']).read_text(encoding='utf-8'))
                        for sid in NEW_SOURCES}
        cls.rows = load_rows()
        cls.report = (research.ROOT / REPORT).read_text(encoding='utf-8')

    def validate(self, packet=None):
        return research.validate(packet or self.packet, research.ROOT, {'Brazil'}, {'Brazil': set()})

    def section(self, heading):
        parts = self.report.split(f'\n## {heading}', 1)
        self.assertEqual(len(parts), 2, heading)
        return parts[1].split('\n## ', 1)[0]

    def test_new_records_are_bounded_and_every_claim_is_classified(self):
        ids = self.validate()
        self.assertEqual(tuple(len(ids[k]) for k in ('entries', 'sources', 'claims', 'roles')),
                         (32, EARLIER_SOURCE_COUNT + 14, EARLIER_CLAIM_COUNT + 21, 6))
        self.assertEqual((len(NEW_SOURCES), len(NEW_CLAIMS)), (14, 21))
        order = [s['id'] for s in self.packet['sources']]
        self.assertEqual(order[EARLIER_SOURCE_COUNT:], NEW_SOURCES)
        self.assertEqual(order[EARLIER_SOURCE_COUNT - len(bp.NEW_SOURCES):EARLIER_SOURCE_COUNT], bp.NEW_SOURCES)
        self.assertFalse([sid for sid in order[:EARLIER_SOURCE_COUNT] if sid.startswith(('br_agir_', 'br_pds_'))])
        self.assertEqual([c['id'] for sid in NEW_SOURCES for c in self.sources[sid]['claims']], NEW_CLAIMS)
        role = role_of(self.packet)
        role_claims = [c for c in NEW_CLAIMS if EVENTS[c][4] != 'D']
        self.assertEqual(role['claim_ids'], role_claims)
        self.assertEqual(role['sources'], NEW_SOURCES)
        org = entry(self.packet, ORG_ID)
        self.assertEqual(org['claim_ids'], ['br_tse_fefc_2024_row_07'] + role_claims)
        self.assertEqual(org['sources'], ['br_tse_fefc_2024'] + NEW_SOURCES)
        self.assertEqual([c for c in NEW_CLAIMS if EVENTS[c][4] == 'D'], ['br_pds_tse_glossary_fusion_into_ppr_19930608'])
        known = HOLDER_KINDS | ORGANIZATION_KINDS | ELECTION_KINDS | UNDATED_KINDS
        cited = [cid for ids_ in HOLDER_CLAIMS for cid in ids_]
        for cid, event in EVENTS.items():
            self.assertIn(event[2], known, cid)
            self.assertEqual(event[2] in HOLDER_KINDS, cid in cited, cid)
            self.assertIn(event[3], PEOPLE | {None}, cid)
            self.assertEqual(event[4] is None, event[3] is None and event[4] != 'D', cid)
        observations = re.findall(r'^### ((?:AGIR|PDS)-PRES-\d\d)\b', self.report, re.M)
        self.assertEqual(observations, REVIEW)
        self.assertEqual({e[0] for e in EVENTS.values()}, set(REVIEW))
        self.assertLessEqual(len(PEOPLE), 10)

    def test_holders_are_exactly_as_intended(self):
        agir_invariants(self.packet, self.rows)
        for index, holder in enumerate(role_of(self.packet)['holder_claims']):
            self.assertTrue(holder['note'].startswith('Observed on '), index)
            self.assertTrue(holder['uncertainty'], index)
            self.assertEqual({self.rows[c]['review_observation'] for c in holder['claim_ids']},
                             {HOLDER_OBSERVATIONS[index]}, index)
            for cid in holder['claim_ids']:
                self.assertRegex(self.claims[cid]['uncertainty'], r'Dates the holder observation on its own day; '
                                                                  r'not itself a start')
        # Printed forms stay in the texts; holder names are normalised.
        self.assertIn('Daniel Sampaio Tourinho Presidente Nacional PTC',
                      self.claims['br_agir_ptc_tourinho_signs_resolution_02_2018_20180725']['text'])
        self.assertIn('DANIEL S. TOURINHO', self.claims['br_agir_tse_registry_lists_tourinho_presidente_nacional']['text'])
        # Stylings that are not the national office, or that fall on an event day, are claims only.
        for cid in ('br_agir_ptc_executive_minutes_presidente_20180703',
                    'br_agir_ptc_tourinho_opens_convention_as_presidente_20150725',
                    'br_agir_plenary_tourinho_styled_presidente_do_partido',
                    'br_agir_executive_approves_resolution_01_2022_20221110'):
            self.assertEqual(self.rows[cid]['holder_name'], 'Daniel Tourinho', cid)
            self.assertNotIn(self.rows[cid]['event_kind'], HOLDER_KINDS, cid)
        # The convocation signed as 'Presidente do Diretório Nacional' is a signed act of its own day: it attests the
        # office (the C01-37/C01-38 signature rule) and dates one observation with no start and no end; the convention
        # it convokes for 28 July 2018 is never a boundary.
        self.assertIn('Presidente do Diretório Nacional', self.claims[CONVOCATION]['text'])
        self.assertIn('signature rule', self.claims[CONVOCATION]['uncertainty'])
        self.assertEqual(self.rows[CONVOCATION]['event_kind'], 'in_office_attestation')
        convocation, = [h for h in role_of(self.packet)['holder_claims'] if CONVOCATION in h['claim_ids']]
        self.assertEqual((convocation['attested_on'], convocation['from'], convocation['until']), ('2018-07-19', None, None))
        self.assertEqual((convocation['sources'], convocation['claim_ids']), (['br_agir_ptc_edital_convencao_20180719'],
                                                                             [CONVOCATION]))
        self.assertIn('2018-07-28', NEVER_HOLDER_DATE)
        self.assertNotIn('2018-07-28', {d for h in role_of(self.packet)['holder_claims']
                                        for d in (h['attested_on'], h['from'], h['until'])})

    def test_dates_renamings_and_event_kinds(self):
        undated = [cid for cid, e in EVENTS.items() if e[1] is None]
        self.assertEqual(len(undated), 5)
        for cid in undated:
            self.assertNotIn('attested_on', self.claims[cid], cid)
            self.assertIn(EVENTS[cid][2], UNDATED_KINDS, cid)
            self.assertIn('no structured date is stored', self.claims[cid]['uncertainty'].lower(), cid)
        # The renamings are the court's decisions of its own register, claims about the organization.
        renamings = {cid: e[1] for cid, e in EVENTS.items() if e[2] == 'tse_renaming_decision'}
        self.assertEqual(renamings, {'br_agir_tse_registry_prn_renamed_ptc_20010424': '2001-04-24',
                                     'br_agir_tse_registry_ptc_renamed_agir_20220331': '2022-03-31',
                                     'br_agir_tse_glossary_prn_renamed_ptc_20010424': '2001-04-24'})
        for cid in renamings:
            self.assertIsNone(self.rows[cid]['holder_name'], cid)
            self.assertIn('never a merged identity', self.claims[cid]['uncertainty'], cid)
        self.assertIn("'RPP nº 51-91.1989.6.00.0000'", self.claims['br_agir_tse_registry_ptc_renamed_agir_20220331']['text'])
        self.assertIn("'PET nº 341 (1069-69.1997.6.00.0000)'", self.claims['br_agir_tse_registry_prn_renamed_ptc_20010424']['text'])
        # The executive meeting day resolves from the item's dateline and stays apart from the observation.
        self.assertEqual(EVENTS['br_agir_executive_approves_resolution_01_2022_20221110'][1], '2022-11-10')
        self.assertIn("'ontem' in an item of Friday 11 November 2022",
                      self.claims['br_agir_executive_approves_resolution_01_2022_20221110']['uncertainty'])
        # Distinct events stay distinct and ordered.
        for earlier, later in (('br_agir_tse_glossary_prn_definitive_registration_19900222', 'br_pds_tse_glossary_fusion_into_ppr_19930608'),
                               ('br_pds_tse_glossary_fusion_into_ppr_19930608', 'br_agir_tse_registry_prn_renamed_ptc_20010424'),
                               ('br_agir_ptc_announces_name_change_20210601', 'br_agir_tse_registry_ptc_renamed_agir_20220331'),
                               ('br_agir_executive_approves_resolution_01_2022_20221110', 'br_agir_tourinho_styled_presidente_nacional_20221111')):
            self.assertLess(self.claims[earlier]['attested_on'], self.claims[later]['attested_on'])
        # Republished reference text is never primary and never holder evidence.
        hist = 'br_agir_ptc_historia_partido_capt20071011'
        self.assertEqual(self.sources[hist]['source_type'], 'party_republished_reference_text')
        self.assertFalse({c['id'] for c in self.sources[hist]['claims']} & {c for ids_ in HOLDER_CLAIMS for c in ids_})

    def test_extracts_match_packet_claims_and_record_original_responses(self):
        for sid in NEW_SOURCES:
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual(extract['format'], 'spheres-c01-derived-factual-table/v1')
            self.assertEqual((extract['source_id'], extract['source_url']), (sid, source['url']))
            for key in ('scope_note', 'access_method', 'published_date', 'rights_note', 'accessed_date'):
                self.assertEqual(extract[key], source[key], (sid, key))
            self.assertEqual(source['accessed_date'], '2026-10-01')
            self.assertLessEqual(source['published_date'] or '', research.CUTOFF)
            self.assertIs(extract['source_response_checked_in'], False)
            self.assertEqual((extract['source_response_bytes'], extract['source_response_sha256']), RESPONSES[sid])
            self.assertIn('not checked into this repository', extract['provenance_note'])
            self.assertIn('derived factual extract', extract['provenance_note'])
            self.assertIn('No open license, portrait permission or likeness approval', extract['rights_note'])
            self.assertRegex(extract['stability_check'], r'and again at')
            self.assertIn(extract['stability_check'], extract['provenance_note'])
            self.assertEqual(extract['visual_review']['pdf_pages_one_based'], PDF_PAGES.get(sid, []))
            self.assertTrue(source['scope_note'] and source['publisher'])
            if sid in ENCODED:
                encoding, size, digest = ENCODED[sid]
                self.assertEqual(extract['source_response_content_encoding'], encoding)
                self.assertEqual((extract['decoded_response_bytes'], extract['decoded_response_sha256']), (size, digest))
                self.assertIn('the recorded identity is the gzip-encoded body exactly as served', extract['provenance_note'])
            else:
                self.assertEqual(extract['source_response_content_encoding'], 'identity')
                self.assertNotIn('decoded_response_sha256', extract)
            snapshot = source['snapshot']
            self.assertTrue(snapshot['path'].startswith('docs/campaign-certification/C01/research/sources/brazil-'))
            self.assertTrue(snapshot['path'].endswith('-facts.json'))
            data = (research.ROOT / snapshot['path']).read_bytes()
            self.assertEqual((len(data), hashlib.sha256(data).hexdigest()), (snapshot['bytes'], snapshot['sha256']))
            self.assertTrue(data.endswith(b'}\n') and b'\r' not in data)
            self.assertEqual(data.decode('utf-8'), json.dumps(extract, indent=2, ensure_ascii=False) + '\n')
            self.assertEqual([r['claim_id'] for r in extract['rows']], [c['id'] for c in source['claims']])
            for row, claim in zip(extract['rows'], source['claims']):
                self.assertEqual((row['text'], row['locator'], row['attested_on']),
                                 (claim['text'], claim['locator'], claim.get('attested_on')))
                self.assertEqual(list(row), ['claim_id', 'observation_id', 'review_observation', 'role_id',
                                             'holder_name', 'role_title', 'event_kind', 'attested_on', 'text', 'locator'])
            stamp = ARCHIVED[sid]
            url = urlsplit(source['url'])
            self.assertEqual(url.hostname, 'web.archive.org')
            self.assertTrue(url.path.startswith(f'/web/{stamp}id_/http'), sid)
            self.assertLess(stamp, '20260907')
            self.assertEqual(source['url'].split('id_/', 1)[1].replace(':80/', '/', 1), source['original_url'])
            self.assertEqual(extract['original_url'], source['original_url'])
            self.assertEqual(extract['archive_capture_utc'].replace('-', '').replace(':', '').replace('T', '').rstrip('Z'),
                             stamp)
            self.assertIn('no automatic decoding', extract['fetch_recipe'])
        self.assertEqual(len({self.sources[s]['snapshot']['path'] for s in NEW_SOURCES}), len(NEW_SOURCES))
        codes = {T_PRES: 'A', None: None}
        got = {cid: (row['review_observation'], row['attested_on'], row['event_kind'], row['holder_name'],
                     'D' if row['role_id'] is None and cid.startswith('br_pds_') else codes[row['role_title']])
               for cid, row in self.rows.items() if cid in EVENTS}
        self.assertEqual(got, EVENTS)
        self.assertEqual(list(got), NEW_CLAIMS)

    def test_no_per_request_url_no_secondary_lead_and_no_personal_registry_data(self):
        self.assertEqual({urlsplit(self.sources[s]['url']).hostname for s in NEW_SOURCES}, HOSTS)
        for sid in NEW_SOURCES:
            self.assertIsNone(PER_REQUEST_URL.search(self.sources[sid]['url']), sid)
            self.assertFalse(self.sources[sid]['source_type'].startswith('secondary'), sid)
        for source in self.packet['sources'][EARLIER_SOURCE_COUNT:]:
            for marker in LEAD_URL_MARKERS:
                self.assertNotIn(marker, source['url'], (source['id'], marker))
        for sid in NEW_SOURCES:
            text = (research.ROOT / self.sources[sid]['snapshot']['path']).read_text(encoding='utf-8')
            self.assertIsNone(re.search(r'\b\d{3}\.\d{3}\.\d{3}-\d{2}\b|\b\d{11}\b|@gmail|@uol', text), sid)
        leads, added = self.section('Leads not imported'), self.section('Sources added')
        for marker in ('pt.wikipedia.org/wiki/Agir_(Brasil)', 'atribunarj', 'cmm.am.gov.br', 'sgip3.tse.jus.br',
                       'tse-aprova-alteracao-e-partido-trabalhista-cristao-passa-a-se-chamar-agir'):
            self.assertIn(marker, leads, marker)
            self.assertNotIn(marker, added, marker)

    def test_packet_formatting_is_preserved(self):
        data = (research.ROOT / research.RESEARCH / 'brazil.json').read_bytes()
        self.assertNotIn(b'\r', data)
        self.assertEqual(data.decode('utf-8'), json.dumps(self.packet, indent=2, ensure_ascii=False) + '\n')

    def test_mutations_are_rejected(self):
        def mutated(change):
            packet = copy.deepcopy(self.packet)
            change(packet)
            return packet

        def claim(packet, cid):
            return next(c for s in packet['sources'] for c in s['claims'] if c['id'] == cid)

        def holder(packet, index):
            return role_of(packet)['holder_claims'][index]

        def cite(packet, index, cid, sid):
            holder(packet, index)['claim_ids'].append(cid)
            holder(packet, index)['sources'].append(sid)

        def set_from(packet, index, day):
            holder(packet, index)['from'] = day

        def set_until(packet, index, day):
            holder(packet, index)['until'] = day

        def to_presidency(packet):
            packet['institutions'][0]['roles'][0]['claim_ids'].append('br_agir_ptc_tourinho_signs_communique_20210723')
            packet['institutions'][0]['roles'][0]['sources'].append('br_agir_ptc_comunicado_20210723')

        def pds_cited(packet):
            org = entry(packet, ORG_ID)
            org['claim_ids'].append('br_pds_tse_glossary_fusion_into_ppr_19930608')

        def to_pt(packet):
            pt_role = entry(packet, pt.ORG_ID)['roles'][0]
            pt_role['claim_ids'].append('br_agir_ptc_tourinho_signs_resolution_02_2018_20180725')

        def move_role(packet):
            org = entry(packet, ORG_ID)
            entry(packet, 'br_tse_fefc_2024_party_08')['roles'] = org['roles']
            org['roles'] = []

        def acting_as_holder(packet):
            role = role_of(packet)
            role['holder_claims'].insert(1, {
                'name': 'Daniel Tourinho', 'attested_on': '2015-07-25', 'from': None, 'until': None,
                'sources': ['br_agir_ptc_convencao_aracaju_20150728'],
                'claim_ids': ['br_agir_ptc_tourinho_opens_convention_as_presidente_20150725'], 'note': 'n',
                'uncertainty': 'u'})

        def wrong_day(packet):
            claim(packet, 'br_agir_ptc_tourinho_signs_resolution_001_2020_20200807')['attested_on'] = '2020-08-20'

        def set_day(packet, cid, day):
            claim(packet, cid)['attested_on'] = day
            holder(packet, 1)['attested_on'] = day

        def date_listing(packet):
            claim(packet, 'br_agir_tse_registry_lists_tourinho_presidente_nacional')['attested_on'] = '2026-08-02'

        def renaming_as_holder(packet):
            cite(packet, 5, 'br_agir_tse_registry_ptc_renamed_agir_20220331', 'br_agir_tse_partidos_registrados_capt20260802')

        cases = {
            'start from a signed act': lambda p: set_from(p, 0, '2014-05-16'),
            'end inferred from the renaming': lambda p: set_until(p, 4, '2022-03-31'),
            'start from the signed convocation': lambda p: set_from(p, 1, '2018-07-19'),
            'end at the convoked convention': lambda p: set_until(p, 1, '2018-07-28'),
            'convocation dated by its convention day': lambda p: set_day(p, CONVOCATION, '2018-07-28'),
            'party office fed to the Presidency of the Republic': to_presidency,
            'PDS claim cited by the AGIR observation': pds_cited,
            'AGIR claim fed to the PT office': to_pt,
            'role moved to another funding observation': move_role,
            'convention-day styling as a holder': acting_as_holder,
            'support claim moved to another day': wrong_day,
            'undated registry listing given a day': date_listing,
            'renaming decision cited by a holder': renaming_as_holder,
        }
        for label, change in cases.items():
            with self.subTest(label):
                with self.assertRaises((AssertionError, KeyError, ValueError, StopIteration)):
                    agir_invariants(mutated(change), self.rows)
        # The undamaged packet passes.
        agir_invariants(self.packet, self.rows)

    def test_report_and_handoff_close_no_parent_gate(self):
        for heading in ('Outcome', 'Observations', 'Sources added', 'Response identities and stability checks',
                        'Date ledger', 'Leads not imported', 'Sources attempted', 'Suggested next work orders',
                        'Integration notes (outside this packet\'s file boundary)', 'Checks'):
            self.section(heading)
        self.assertIn('**ready_for_review**', self.report)
        for sid in NEW_SOURCES:
            self.assertIn(f'`{sid}`', self.section('Sources added'), sid)
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        for text in ('ready_for_review', 'brazil-prn-ptc-agir-presidents-1990-2026-43.md', 'claude/c01-br-43', '6e6a9d06',
                     '509bd289', 'test_brazil_prn_agir_presidents_c01_43.py'):
            self.assertIn(text, handoff)
        index = research.build()
        country = next(p for p in index['countries'] if p['nation'] == 'Brazil')
        self.assertFalse(country['country_census_complete'])
        self.assertIsNone(country['unrepresented_organization_count'])
        self.assertEqual((country['institution_observations'], country['role_observations'], country['source_claims']),
                         (1, 6, EARLIER_CLAIM_COUNT + 21))
        self.assertEqual({w['status'] for w in index['work_orders'] if w['nation'] == 'Brazil'}, {'open'})
        self.assertFalse(index['c01_complete'])


if __name__ == '__main__':
    unittest.main()
