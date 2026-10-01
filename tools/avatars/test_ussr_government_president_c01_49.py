"""CLAUDE-C01-49: the USSR President and head of government, dated attestations, 1990-1991.

The existing su_president holders (CLAUDE-C01-05) and su_government_head holders (CLAUDE-C01-26) stay first and unchanged; this
packet appends dated observations from Soviet records. The President's oath observation carries from 1990-03-15 because the
Congress's official stenographic report records the oath and the presiding officer's declaration that he had assumed the office;
every other new holder rests on an in-office signature with attested_on only. The ballot, the election result and resolution, the
Premier's approval, stylings, the Vice-President's acting claim of August 1991 and the Presidium's declaration stay separate dated
claims and never feed a holder, an until or a lifecycle bound. Party office and state office stay separate, and nothing reaches
russia.json or the simulation's party rows."""
import copy
from datetime import datetime
import hashlib
import json
import re
import unittest

import campaign_research as research

# The 6 sources this packet appends, in packet order (document dates ascending), at positions 68-73.
NEW_SOURCES = [
    'su_snd3_steno_vol3_president',
    'su_pravda_no13_19910115',
    'su_pravda_no20_19910123',
    'su_izv_197_19910820',
    'su_ved_1991_35_president',
    'su_ved_1991_41_president',
]
FIRST = 68
COUNTS = (6, 17)
TOTALS = (74, 174, 7, 8)
FILES = {
    'su_snd3_steno_vol3_president': 'ussr-snd3-stenogram-vol3-president-election-oath-19900315-facts.json',
    'su_pravda_no13_19910115': 'ussr-pravda-no13-premier-approval-19910115-facts.json',
    'su_pravda_no20_19910123': 'ussr-pravda-no20-decree-and-resolution-19910123-facts.json',
    'su_izv_197_19910820': 'ussr-izvestia-no197-vice-president-decree-19910820-facts.json',
    'su_ved_1991_35_president': 'ussr-ved-1991-35-presidium-and-decree-19910828-facts.json',
    'su_ved_1991_41_president': 'ussr-ved-1991-41-presidential-decree-19911009-facts.json',
}
# Recorded response identity of each new source: (bytes, sha256), downloaded identical twice at least 30 minutes apart.
RESPONSES = {
    'su_snd3_steno_vol3_president': (12922126, 'ae895c021cd21ea893c0c97a5386a434eff9d7c681721162c1999dfa0b07b345'),
    'su_pravda_no13_19910115': (3947088, '848926054db410ed5540a91fb703e9153f963ed8bf150d9235d8521612ea845e'),
    'su_pravda_no20_19910123': (5294185, 'a6539752c01ca89b7ad5ef6482d4a3b9b00c1e500cde3ee4494ab594b91d8a13'),
    'su_izv_197_19910820': (39761395, 'ddb564e7447c86f66a51e9591104aba840af13aef90689bf96178c7e63e45e4d'),
    'su_ved_1991_35_president': (618372, '5a8c0630da633ac68ef5c0514c05107497a9e84cda82541d561bc22013c55a63'),
    'su_ved_1991_41_president': (556781, '91cf65718b9dceaeb8cb4b4c6ce4eccf337af45afa3cdb897f197c97bdff902e'),
}
# Internet Archive stored files: (item, file, archive.org's recorded SHA-1).
STORED = {
    'su_pravda_no13_19910115': ('199113_8099', 'Правда, 1991 , № 13.pdf', '5b52dc7a3fb41f62f1006c2457464c0c2fc34672'),
    'su_pravda_no20_19910123': ('199120_2066', 'Правда, 1991 , № 20.pdf', '93841a36aa4f733f51a1f439f4e5acd3b8cc2bb3'),
}
# Raw Internet Archive captures (all before the cutoff), each with the byte-identical live file attached.
ARCHIVED = {
    'su_snd3_steno_vol3_president': '20240903210942',
    'su_ved_1991_35_president': '20211204065955',
    'su_ved_1991_41_president': '20240906045835',
}
# The one live static file recorded as the identity (the CDX index holds no capture of it).
LIVE_ONLY = {'su_izv_197_19910820': ('https://izvestija.sssr.su/1991/197.pdf', 'Fri, 02 Dec 2016 01:36:05 GMT')}
SAME_BYTES = {'su_snd3_steno_vol3_president': 'su_snd3_steno_vol3', 'su_ved_1991_35_president': 'su_ved_1991_35',
              'su_ved_1991_41_president': 'su_ved_1991_41'}
EARLIER_SNAPSHOTS = {'su_snd3_steno_vol3': '3836f951fb58e93216c6ea4ec0e99463621903509080c0cccb9fc53262feeed6',
                     'su_ved_1991_35': '0fcaa0721ac623d0131eea0f55d1b8fd0dcf88da9506ddc2ca7749a4832af8fc',
                     'su_ved_1991_41': 'de77d0008490ddfb87af4f490bf52642206feab3b43d922985037526d3d137b9'}
PAGES = {
    'su_snd3_steno_vol3_president': [56, 57, 58],
    'su_pravda_no13_19910115': [1],
    'su_pravda_no20_19910123': [1, 2],
    'su_izv_197_19910820': [1],
    'su_ved_1991_35_president': [14, 17],
    'su_ved_1991_41_president': [3, 22],
}
PRES, GOV = 'su_president', 'su_government_head'
PRES_INST, GOV_INST = 'su_presidency', 'su_government'
GORB, PAVLOV = 'Михаил Сергеевич Горбачев', 'Валентин Сергеевич Павлов'
# Every claim: (attested_on, event kind, review observation, role).
EVENTS = {
    'su_snd3p_president_ballot_held_19900314': ('1990-03-14', 'secret_ballot_held', 'SU-PRES-01', PRES),
    'su_snd3p_president_vote_result_19900315': ('1990-03-15', 'election_result_announced', 'SU-PRES-01', PRES),
    'su_snd3p_protocol_approved_resolution_adopted_19900315': ('1990-03-15', 'election_resolution_adopted', 'SU-PRES-01', PRES),
    'su_snd3p_gorbachev_oath_19900315': ('1990-03-15', 'oath_of_office', 'SU-PRES-01', PRES),
    'su_snd3p_assumption_of_office_declared_19900315': ('1990-03-15', 'assumption_of_office', 'SU-PRES-01', PRES),
    'su_pravda13_supreme_soviet_approves_premier_19910114': ('1991-01-14', 'appointment_approved', 'SU-GOV-11', GOV),
    'su_pravda13_note_styles_premier_19910115': ('1991-01-15', 'styling_in_biographical_note', 'SU-GOV-11', GOV),
    'su_pravda20_president_signs_decree_19910122': ('1991-01-22', 'in_office_attestation', 'SU-PRES-02', PRES),
    'su_pravda20_premier_signs_resolution_19910122': ('1991-01-22', 'in_office_attestation', 'SU-GOV-12', GOV),
    'su_izv197_vice_president_decree_assumes_duties_19910818': ('1991-08-18', 'acting_service_claimed', 'SU-PRES-03', PRES),
    'su_izv197_leadership_statement_powers_passed_19910818': ('1991-08-18', 'powers_transfer_claimed', 'SU-PRES-03', PRES),
    'su_izv197_statement_lists_pavlov_as_premier_19910818': ('1991-08-18', 'styling_in_signed_statement', 'SU-GOV-12', GOV),
    'su_izv197_acting_president_styling_19910818': ('1991-08-18', 'acting_styling_claimed', 'SU-PRES-03', PRES),
    'su_ved35p_presidium_2352i_removal_unlawful_19910821': ('1991-08-21', 'removal_declared_unlawful', 'SU-PRES-03', PRES),
    'su_ved35p_presidium_2352i_vice_president_acts_19910821': ('1991-08-21', 'acting_acts_revocation_demanded', 'SU-PRES-03', PRES),
    'su_ved35p_up2443_president_signs_19910822': ('1991-08-22', 'in_office_attestation', 'SU-PRES-03', PRES),
    'su_ved41p_up2668_president_signs_19911005': ('1991-10-05', 'in_office_attestation', 'SU-PRES-02', PRES),
}
ROW_HOLDERS = {cid: None for cid in EVENTS}
ROW_HOLDERS.update({cid: GORB for cid in (
    'su_snd3p_president_vote_result_19900315', 'su_snd3p_protocol_approved_resolution_adopted_19900315',
    'su_snd3p_gorbachev_oath_19900315', 'su_snd3p_assumption_of_office_declared_19900315',
    'su_pravda20_president_signs_decree_19910122', 'su_ved35p_presidium_2352i_removal_unlawful_19910821',
    'su_ved35p_up2443_president_signs_19910822', 'su_ved41p_up2668_president_signs_19911005')})
ROW_HOLDERS.update({cid: PAVLOV for cid in (
    'su_pravda13_supreme_soviet_approves_premier_19910114', 'su_pravda13_note_styles_premier_19910115',
    'su_pravda20_premier_signs_resolution_19910122', 'su_izv197_statement_lists_pavlov_as_premier_19910818')})
# The exact holders, existing first and unchanged, then the appended ones: (name, attested_on, from, until).
HOLDERS = {
    PRES: [
        ('Mikhail Gorbachev', '1990-03-20', None, None),
        ('Mikhail Gorbachev', '1991-12-25', None, None),
        (GORB, None, '1990-03-15', None),
        (GORB, '1991-01-22', None, None),
        (GORB, '1991-08-22', None, None),
        (GORB, '1991-10-05', None, None),
    ],
    GOV: [
        ('Николай Иванович Рыжков', '1990-01-12', None, None),
        ('Николай Иванович Рыжков', '1990-11-24', None, None),
        (PAVLOV, '1991-08-19', None, None),
        (PAVLOV, '1991-01-22', None, None),
    ],
}
HOLDER_CLAIMS = {
    PRES: [['su_gorbachev_president_letter_19900320'], ['su_telcon_gorbachev_title_19911225'],
           ['su_snd3p_gorbachev_oath_19900315', 'su_snd3p_assumption_of_office_declared_19900315'],
           ['su_pravda20_president_signs_decree_19910122'], ['su_ved35p_up2443_president_signs_19910822'],
           ['su_ved41p_up2668_president_signs_19911005']],
    GOV: [['su_sprsfsr_60_ryzhkov_signs_as_chairman_19900112'], ['su_ips_cm_1177_ryzhkov_signs_as_chairman_19901124'],
          ['su_km_943r_pavlov_signs_as_premier_19910819'], ['su_pravda20_premier_signs_resolution_19910122']],
}
BASE_COUNT = {PRES: 2, GOV: 3}
# SHA-256 (sorted-key JSON) of the base holder lists of the two roles and of every other USSR role's holders (5ea4f8fc).
BASE_HOLDERS_SHA256 = {PRES: '2bb1453412c0ba86bd0e2ae3c7b01b9b1abc743185fd26850561afbfdf362563',
                       GOV: 'f39f83320f28b8a633b7b3718f01031b190f8e59937aaf077076e3f9b4d18ea1'}
OTHER_HOLDERS_SHA256 = '7c93b61beb22a048e6bb75f721a1a8d68609bfc1c1ceecc372fa034a0e84c3dd'
BASE_ROLE_CLAIMS = {PRES: ['su_presidency_created_19900314', 'su_first_president_congress_rule_19900314',
                           'su_gorbachev_president_letter_19900320', 'su_telcon_gorbachev_title_19911225',
                           'su_telcon_gorbachev_intent_19911225', 'su_telcon_resignation_decrees_19911225',
                           'su_bush_address_resign_19911225', 'su_garf_1362i_gorbachev_elected_president_19900315']}
BASE_GOV_CLAIMS_SHA256 = '6c84cf46262eb41d4a88b17d60e3158117b1a3df9b725dd65824caa84456c441'
BASE_ROLE_SOURCES = {PRES: ['su_presidency_law_19900314', 'su_bush_presidential_letter_19900320',
                            'su_nara_bush_gorbachev_telcon_19911225', 'su_bush_address_cis_19911225',
                            'su_garf_exhibit_res_1362i_19900315'],
                     GOV: ['su_sprsfsr_1990_8_art59_19891224', 'su_sprsfsr_1990_8_art60_19900112', 'su_ips_cm_res_525_19900526',
                           'su_ips_cm_res_1177_19901124', 'su_snd4_steno_vol3', 'su_ips_cm_res_27_19910110',
                           'su_km_rasp_943r_19910819', 'su_ved_1991_35', 'su_ved_1991_36']}
INST_SOURCES_ADDED = {PRES_INST: ['su_snd3_steno_vol3_president', 'su_pravda_no20_19910123', 'su_izv_197_19910820',
                                  'su_ved_1991_35_president', 'su_ved_1991_41_president'],
                      GOV_INST: ['su_pravda_no13_19910115', 'su_pravda_no20_19910123', 'su_izv_197_19910820']}
BASE_COVERAGE = {PRES_INST: 7, GOV_INST: 8, 'packet': 9}
SIGNATURE_KINDS = {'in_office_attestation'}
START_KINDS = {'oath_of_office', 'assumption_of_office'}
ELECTION_KINDS = {'secret_ballot_held', 'election_result_announced', 'election_resolution_adopted', 'appointment_approved'}
STYLING_KINDS = {'styling_in_biographical_note', 'styling_in_signed_statement'}
ACTING_KINDS = {'acting_service_claimed', 'powers_transfer_claimed', 'acting_styling_claimed', 'removal_declared_unlawful',
                'acting_acts_revocation_demanded'}
VOCABULARY = SIGNATURE_KINDS | START_KINDS | ELECTION_KINDS | STYLING_KINDS | ACTING_KINDS
HOLDER_CLAIM_IDS = {c for ids in HOLDER_CLAIMS.values() for group in ids for c in group}
NEVER_HOLDER = tuple(cid for cid in EVENTS if cid not in HOLDER_CLAIM_IDS)
OATH = ['su_snd3p_gorbachev_oath_19900315', 'su_snd3p_assumption_of_office_declared_19900315']
LEAD_URL_MARKERS = ('wikipedia', 'gorby.ru', '1000dokumente', 'sssr.su/1991-12.pdf', 'vedomosti.sssr.su/1991/52',
                    'sten.sr.vs.sssr.su', 'yandex', 'garant', 'consultant', 'izvestija.sssr.su/1991/200', 'pravda1991_302')
REPORT = research.RESEARCH / 'ussr-government-president-attestations-1990-1991-49.md'
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-49.md'
# Every person named in a row (stems, lower case): the two holders, the Vice-President and four signatories or officers.
PEOPLE = ('горбачев', 'павлов', 'янаев', 'лукьянов', 'осипьян', 'бакланов', 'шкабардн')
OBSERVATIONS = ['SU-PRES-01', 'SU-PRES-02', 'SU-PRES-03', 'SU-PRES-04', 'SU-GOV-11', 'SU-GOV-12']


def roles_of(packet):
    return {r['id']: (e, r) for g in ('organizations', 'institutions') for e in packet[g] for r in e['roles']}


def digest(obj):
    return hashlib.sha256(json.dumps(obj, ensure_ascii=False, sort_keys=True).encode('utf-8')).hexdigest()


def c49_invariants(ussr, russia):
    """Packet-level rules this test owns; raises AssertionError, KeyError, IndexError or StopIteration on any violation."""
    claims = {c['id']: c for s in ussr['sources'] for c in s['claims']}
    owner = {c['id']: s['id'] for s in ussr['sources'] for c in s['claims']}
    roles = roles_of(ussr)
    insts = {e['id']: e for e in ussr['institutions']}
    # The existing holders stay first and byte-for-byte unchanged; every other USSR role's holders are unchanged.
    for role_id, n in BASE_COUNT.items():
        assert digest(roles[role_id][1]['holder_claims'][:n]) == BASE_HOLDERS_SHA256[role_id], role_id
    other = [(r['id'], r['holder_claims']) for g in ('organizations', 'institutions') for e in ussr[g] for r in e['roles']
             if r['id'] not in (PRES, GOV)]
    assert digest(other) == OTHER_HOLDERS_SHA256
    # Rules first, so a mutation meets the rule it breaks; the exact lists are compared at the end.
    for role_id, n in BASE_COUNT.items():
        role = roles[role_id][1]
        for h in role['holder_claims'][n:]:
            assert h['until'] is None, h['name']
            assert not re.search(r'(?i)acting|исполняющ|временно|и\. ?о\.|Янаев|ЯНАЕВ', h['name']), h['name']
            assert not set(h['claim_ids']) & set(NEVER_HOLDER), h['name']
            assert all(cid in role['claim_ids'] and EVENTS[cid][3] == role_id for cid in h['claim_ids']), h['name']
            assert h['sources'] == list(dict.fromkeys(owner[cid] for cid in h['claim_ids'])), h['name']
            kinds = {EVENTS[cid][1] for cid in h['claim_ids']}
            if h['from'] is not None:
                # A start only from an oath and a declared assumption of office, on their own day; never with attested_on.
                assert role_id == PRES and kinds == START_KINDS and h['attested_on'] is None, h['name']
                assert {claims[cid]['attested_on'] for cid in h['claim_ids']} == {h['from']}, h['name']
            else:
                assert kinds <= SIGNATURE_KINDS and h['attested_on'], h['name']
                assert all(claims[cid]['attested_on'] == h['attested_on'] for cid in h['claim_ids']), h['name']
    for role_id in (PRES, GOV):
        for h in roles[role_id][1]['holder_claims']:
            assert h['until'] is None, (role_id, h['name'])
    # The exact holders.
    for role_id, expected in HOLDERS.items():
        holders = roles[role_id][1]['holder_claims']
        assert [(h['name'], h.get('attested_on'), h['from'], h['until']) for h in holders] == expected, role_id
        assert [h['claim_ids'] for h in holders] == HOLDER_CLAIMS[role_id], role_id
    # Each claim is cited by its own role only, after the existing citations, and by nothing else in either packet.
    ours = set(EVENTS)
    pres, gov = roles[PRES][1], roles[GOV][1]
    assert pres['claim_ids'] == BASE_ROLE_CLAIMS[PRES] + [c for c, v in EVENTS.items() if v[3] == PRES]
    n = len(gov['claim_ids']) - sum(v[3] == GOV for v in EVENTS.values())
    assert digest(gov['claim_ids'][:n]) == BASE_GOV_CLAIMS_SHA256
    assert gov['claim_ids'][n:] == [c for c, v in EVENTS.items() if v[3] == GOV]
    for role_id in (PRES, GOV):
        ids = [c for c, v in EVENTS.items() if v[3] == role_id]
        assert roles[role_id][1]['sources'] == BASE_ROLE_SOURCES[role_id] + list(dict.fromkeys(owner[c] for c in ids)), role_id
    for inst_id, added in INST_SOURCES_ADDED.items():
        assert insts[inst_id]['sources'][-len(added):] == added, inst_id
    for packet in (russia, ussr):
        for group in ('organizations', 'institutions'):
            for entry in packet[group]:
                assert not set(entry['claim_ids']) & ours, entry['id']
                if entry['id'] not in INST_SOURCES_ADDED:
                    assert not set(entry['sources']) & set(NEW_SOURCES), entry['id']
                for r in entry['roles']:
                    cited = set(r['claim_ids']) | {c for h in r['holder_claims'] if isinstance(h, dict) for c in h['claim_ids']}
                    assert not {c for c in cited & ours if EVENTS[c][3] != r['id']}, r['id']
                    if r['id'] not in (PRES, GOV):
                        assert not set(r['sources']) & set(NEW_SOURCES), r['id']
    # Acting and transfer claims, the Presidium's declaration, elections and stylings name no holder of their own.
    for cid, v in EVENTS.items():
        if v[1] in ACTING_KINDS | ELECTION_KINDS | STYLING_KINDS:
            assert cid in NEVER_HOLDER, cid
    # No lifecycle bound comes from any claim.
    assert (insts[PRES_INST]['lifecycle']['from'], insts[PRES_INST]['lifecycle']['until']) == ('1990-03-14', None)
    assert (insts[GOV_INST]['lifecycle']['from'], insts[GOV_INST]['lifecycle']['until']) == (None, None)
    for cid in ours:
        assert '1990-01-01' <= claims[cid]['attested_on'] <= '1991-12-25', cid


def c49_extract_check(packet, root):
    """Extracts equal the packet: snapshot bytes and hash, claim ids and texts, dates, kinds and locators."""
    sources = {s['id']: s for s in packet['sources']}
    for sid in NEW_SOURCES:
        source = sources[sid]
        data = (root / source['snapshot']['path']).read_bytes()
        assert (len(data), hashlib.sha256(data).hexdigest()) == (source['snapshot']['bytes'], source['snapshot']['sha256']), sid
        extract = json.loads(data)
        assert [r['claim_id'] for r in extract['rows']] == [c['id'] for c in source['claims']], sid
        for row, claim in zip(extract['rows'], source['claims']):
            assert row['text'] == claim['text'] and row['locator'] == claim['locator'], claim['id']
            assert row['attested_on'] == claim['attested_on'] == EVENTS[claim['id']][0], claim['id']
            assert (row['event_kind'], row['review_observation'], row['role_id']) == EVENTS[claim['id']][1:4], claim['id']
            assert row['holder_name'] == ROW_HOLDERS[claim['id']], claim['id']
            assert row['observation_id'] == (PRES_INST if row['role_id'] == PRES else GOV_INST), claim['id']
        assert extract['source_url'] == source['url'], sid
        assert (extract['source_response_bytes'], extract['source_response_sha256']) == RESPONSES[sid], sid


class UssrGovernmentPresidentTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.raw = (research.ROOT / research.RESEARCH / 'ussr.json').read_text(encoding='utf-8')
        cls.russia_raw = (research.ROOT / research.RESEARCH / 'russia.json').read_text(encoding='utf-8')
        cls.packet, cls.russia = json.loads(cls.raw), json.loads(cls.russia_raw)
        cls.sources = {s['id']: s for s in cls.packet['sources']}
        cls.claims = {c['id']: c for s in cls.packet['sources'] for c in s['claims']}
        cls.roles = roles_of(cls.packet)
        cls.extracts = {sid: json.loads((research.ROOT / cls.sources[sid]['snapshot']['path']).read_text(encoding='utf-8'))
                        for sid in NEW_SOURCES}
        cls.rows = {row['claim_id']: row for sid in NEW_SOURCES for row in cls.extracts[sid]['rows']}
        cls.report = (research.ROOT / REPORT).read_text(encoding='utf-8')

    def validate(self, packet=None):
        return research.validate(packet or self.packet, research.ROOT, {'USSR', 'Russia'}, {'USSR': set(), 'Russia': set()})

    def section(self, heading):
        parts = self.report.split(f'\n## {heading}', 1)
        self.assertEqual(len(parts), 2, heading)
        return parts[1].split('\n## ', 1)[0]

    def test_new_records_are_bounded_and_every_claim_is_classified(self):
        ids = self.validate()
        self.assertEqual((len(NEW_SOURCES), len(EVENTS)), COUNTS)
        self.assertEqual([s['id'] for s in self.packet['sources'][FIRST:]], NEW_SOURCES)
        self.assertEqual((len(ids['sources']), len(ids['claims']), len(ids['entries']), len(ids['roles'])), TOTALS)
        self.assertEqual([c['id'] for sid in NEW_SOURCES for c in self.sources[sid]['claims']], list(EVENTS))
        dates = [self.sources[sid]['document_date'] for sid in NEW_SOURCES]
        self.assertEqual(dates, sorted(dates))
        self.assertEqual({v[1] for v in EVENTS.values()}, VOCABULARY)
        self.assertEqual({v[2] for v in EVENTS.values()}, set(OBSERVATIONS) - {'SU-PRES-04'})
        self.assertEqual(re.findall(r'^### (SU-(?:PRES|GOV)-\d\d)\b', self.report, re.M), OBSERVATIONS)
        for sid in NEW_SOURCES:
            self.assertEqual(self.sources[sid]['accessed_date'], '2026-10-01')
            self.assertTrue(sid.startswith('su_') and all(c['id'].startswith('su_') for c in self.sources[sid]['claims']))
        # At most ten people: two holders here; the others appear only as signatories or claimants in persons_named.
        names = {h['name'] for rid in (PRES, GOV) for h in self.roles[rid][1]['holder_claims'][BASE_COUNT[rid]:]}
        self.assertEqual(names, {GORB, PAVLOV})
        for p in {p for row in self.rows.values() for p in row['persons_named']}:
            self.assertTrue(any(name in p.lower() for name in PEOPLE), p)
        self.assertLessEqual(len(PEOPLE), 10)

    def test_holders_are_exactly_as_intended(self):
        c49_invariants(self.packet, self.russia)
        role = self.roles[PRES][1]
        for phrase in ('CLAUDE-C01-49', 'unchanged', 'claims only', 'No holder has an until'):
            self.assertIn(phrase, role['scope_note'])
        for rid in (PRES, GOV):
            for h in self.roles[rid][1]['holder_claims'][BASE_COUNT[rid]:]:
                self.assertTrue(h['note'] and h['uncertainty'], h['name'])
                for cid in h['claim_ids']:
                    self.assertEqual((self.rows[cid]['holder_name'], self.rows[cid]['role_id']), (h['name'], rid), cid)

    def test_starts_and_ends_only_where_a_source_states_one(self):
        oath = self.roles[PRES][1]['holder_claims'][2]
        self.assertEqual((oath['from'], oath['attested_on'], oath['until'], oath['claim_ids']), ('1990-03-15', None, None, OATH))
        self.assertIn('вступил в должность', self.claims['su_snd3p_assumption_of_office_declared_19900315']['text'])
        self.assertIn('su_first_president_congress_rule_19900314', oath['note'])
        # The ballot (14 March) and the result (15 March) stay two dated claims, never a start.
        self.assertEqual(self.claims['su_snd3p_president_ballot_held_19900314']['attested_on'], '1990-03-14')
        self.assertIn('вчера', self.claims['su_snd3p_president_ballot_held_19900314']['text'])
        self.assertIn('sets no from', self.claims['su_snd3p_protocol_approved_resolution_adopted_19900315']['uncertainty'])
        self.assertIn('gives no from', self.claims['su_pravda13_supreme_soviet_approves_premier_19910114']['uncertainty'])
        for cid in ('su_izv197_vice_president_decree_assumes_duties_19910818', 'su_izv197_acting_president_styling_19910818'):
            self.assertRegex(self.claims[cid]['uncertainty'], r'(?i)claims only')
            self.assertIsNone(self.rows[cid]['holder_name'])
        for rid in (PRES, GOV):
            for h in self.roles[rid][1]['holder_claims']:
                self.assertIsNone(h['until'])
                self.assertNotIn('Янаев', h['name'])
        self.assertIsNone(self.roles[PRES][0]['lifecycle']['until'])
        self.assertTrue(any(u.startswith('SU-PRES-04') and 'no until is set' in u
                            for u in self.roles[PRES][0]['coverage']['unresolved']))

    def test_extracts_match_packet_claims_and_record_original_responses(self):
        c49_extract_check(self.packet, research.ROOT)
        for sid in NEW_SOURCES:
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual((extract['format'], extract['source_id']), ('spheres-c01-derived-factual-table/v1', sid))
            self.assertEqual(extract['scope_note'], source['scope_note'])
            self.assertFalse(extract['source_response_checked_in'])
            self.assertIn('not checked into this repository', extract['provenance_note'])
            self.assertIn('No portrait permission or likeness approval', extract['rights_note'])
            self.assertEqual(extract['visual_review']['pdf_pages_one_based'], PAGES[sid])
            self.assertEqual(source['snapshot']['path'], 'docs/campaign-certification/C01/research/sources/' + FILES[sid])
            self.assertEqual(source['snapshot']['kind'], 'derived_factual_extract')
            self.assertNotIn('source_response_content_encoding', extract)
            self.assertEqual(extract['downloads'][0]['response'], 'source_response')
            for d in extract['downloads']:
                self.assertEqual(d['result'], 'identical on both downloads')
                t1, t2 = (datetime.fromisoformat(t.replace('Z', '+00:00')) for t in d['downloaded_at'])
                self.assertGreaterEqual((t2 - t1).total_seconds(), 1800)
                self.assertEqual((d['bytes'], d['sha256']), RESPONSES[sid])
            self.assertEqual(extract['downloads'][0]['url'], source['url'])
            if sid in STORED:
                ident, name, sha1 = STORED[sid]
                stored = extract['stored_file']
                self.assertEqual((stored['identifier'], stored['file'], stored['archive_org_file_sha1']), (ident, name, sha1))
                self.assertEqual(stored['archive_org_file_size'], RESPONSES[sid][0])
                self.assertTrue(source['url'].startswith('https://ia') and f'/items/{ident}/' in source['url'])
                self.assertEqual(source['access_method'], 'internet_archive_stored_file')
                self.assertNotIn('@', json.dumps(stored))
            elif sid in ARCHIVED:
                self.assertEqual(source['url'], f"https://web.archive.org/web/{ARCHIVED[sid]}id_/{source['original_url']}")
                self.assertLess(ARCHIVED[sid][:8], '20260907')
                self.assertEqual(source['access_method'], 'internet_archive_raw_capture')
                self.assertEqual((extract['live_file_response']['bytes'], extract['live_file_response']['sha256']), RESPONSES[sid])
                self.assertEqual(extract['live_file_response']['url'], source['original_url'])
            else:
                url, modified = LIVE_ONLY[sid]
                self.assertEqual((source['url'], extract['source_response_last_modified']), (url, modified))
                self.assertEqual(source['access_method'], 'scanned_facsimile_downloaded_over_https')
        # Three sources repeat bytes already recorded by earlier packets under their own IDs; those records stay unchanged.
        for sid, earlier in SAME_BYTES.items():
            old = self.sources[earlier]
            old_extract = json.loads((research.ROOT / old['snapshot']['path']).read_text(encoding='utf-8'))
            self.assertEqual((old_extract['source_response_bytes'], old_extract['source_response_sha256']), RESPONSES[sid])
            self.assertEqual(old['snapshot']['sha256'], EARLIER_SNAPSHOTS[earlier])
            self.assertFalse({c['id'] for c in old['claims']} & set(EVENTS))

    def test_secondary_leads_and_volatile_urls_stay_out_of_the_packet(self):
        for sid in NEW_SOURCES:
            source = self.sources[sid]
            for url in (source['url'], source.get('original_url') or ''):
                for marker in LEAD_URL_MARKERS:
                    self.assertNotIn(marker, url, (sid, marker))
            self.assertNotRegex(source['url'], r'[?&](cb|_cb|_chk|nocache|q)=|/web/\d{14}/|search|list_itself')
            self.assertTrue(source['source_type'].startswith('primary_'), sid)
            self.assertIn('non_official_host', source['source_type'])
        # Pravda is the party's organ: its printing of state acts is disclosed as such in every Pravda record.
        for sid in ('su_pravda_no13_19910115', 'su_pravda_no20_19910123'):
            self.assertIn('not a state publication', self.sources[sid]['scope_note'])
        leads = self.section('Leads not imported')
        for marker in ('Pravda No. 302', 'sssr.su/1991-12.pdf', 'gorby.ru', 'Izvestia No. 200'):
            self.assertIn(marker, leads)

    def test_separation_from_party_office_russia_and_the_simulation(self):
        c49_invariants(self.packet, self.russia)
        for rid in ('su_cpsu_general_secretary', 'su_cpsu_deputy_general_secretary'):
            role = self.roles[rid][1]
            self.assertFalse(set(role['claim_ids']) & set(EVENTS), rid)
            self.assertFalse(set(role['sources']) & set(NEW_SOURCES), rid)
        for token in list(EVENTS) + NEW_SOURCES:
            self.assertNotIn(token, self.russia_raw)
        for inst in (PRES_INST, GOV_INST):
            entry = self.roles[PRES if inst == PRES_INST else GOV][0]
            self.assertEqual(entry['represented_party_ids'], [])
            self.assertIs(entry['jurisdiction']['automatic_successor_mapping'], False)
        # The coverage items are appended after the existing ones.
        pres_cov = self.roles[PRES][0]['coverage']['unresolved']
        gov_cov = self.roles[GOV][0]['coverage']['unresolved']
        self.assertEqual([u.split(' ')[0] for u in pres_cov[BASE_COVERAGE[PRES_INST]:]], ['SU-PRES-01', 'SU-PRES-02/03', 'SU-PRES-04'])
        self.assertEqual([u.split(' ')[0] for u in gov_cov[BASE_COVERAGE[GOV_INST]:]], ['SU-GOV-11', 'SU-GOV-12'])
        self.assertEqual(len(self.packet['coverage']['unresolved']), BASE_COVERAGE['packet'] + 1)
        self.assertTrue(self.packet['coverage']['unresolved'][-1].startswith('CLAUDE-C01-49 '))

    def test_report_and_handoff_are_ready_for_review(self):
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        for text in ('ready_for_review', 'ussr-government-president-attestations-1990-1991-49.md', 'claude/c01-su-49',
                     '5ea4f8fc', 'b2ae720b', 'test_ussr_government_president_c01_49.py', 'Decisions for Codex'):
            self.assertIn(text, handoff)
        for heading in ('Outcome', 'Observations', 'Sources added', 'Response identities and stability checks', 'Date ledger',
                        'Leads not imported', 'Sources attempted', 'Suggested next work orders', 'Integration notes', 'Checks'):
            self.section(heading)
        index = research.build()
        country = next(p for p in index['countries'] if p['nation'] == 'USSR')
        self.assertEqual((country['institution_observations'], country['role_observations'], country['source_claims']), (4, 8, 174))
        self.assertFalse(country['country_census_complete'])
        self.assertFalse(index['c01_complete'])

    def test_packet_formatting_is_preserved(self):
        self.assertEqual(self.raw, json.dumps(self.packet, indent=2, ensure_ascii=False) + '\n')
        for sid in NEW_SOURCES:
            text = (research.ROOT / self.sources[sid]['snapshot']['path']).read_text(encoding='utf-8')
            self.assertEqual(text, json.dumps(self.extracts[sid], indent=2, ensure_ascii=False) + '\n')
            self.assertNotIn('\r', text)

    def test_mutations_are_rejected(self):
        def mutated(change):
            packet = copy.deepcopy(self.packet)
            change(packet)
            return packet

        def role(packet, role_id):
            return roles_of(packet)[role_id][1]

        def claim(packet, cid):
            return next(c for s in packet['sources'] for c in s['claims'] if c['id'] == cid)

        cases = {
            'announced end as until': lambda p: role(p, PRES)['holder_claims'][5].update({'until': '1991-12-25'}),
            'approval day as the Premier start': lambda p: role(p, GOV)['holder_claims'][3].update({'from': '1991-01-14'}),
            'election day moved onto the start': lambda p: role(p, PRES)['holder_claims'][2].update(
                {'claim_ids': ['su_snd3p_president_vote_result_19900315'], 'from': '1990-03-15'}),
            'ballot day as from': lambda p: role(p, PRES)['holder_claims'][2].update({'from': '1990-03-14'}),
            'oath observation given attested_on': lambda p: role(p, PRES)['holder_claims'][2].update({'attested_on': '1990-03-15'}),
            'holder day moved off its claim': lambda p: role(p, GOV)['holder_claims'][3].update({'attested_on': '1991-01-23'}),
            'acting Vice-President as holder': lambda p: role(p, PRES)['holder_claims'].append(
                {'name': 'Геннадий Иванович Янаев', 'attested_on': '1991-08-18', 'from': None, 'until': None,
                 'sources': ['su_izv_197_19910820'], 'claim_ids': ['su_izv197_vice_president_decree_assumes_duties_19910818']}),
            'Premier styling as holder': lambda p: role(p, GOV)['holder_claims'].append(
                {'name': PAVLOV, 'attested_on': '1991-08-18', 'from': None, 'until': None, 'sources': ['su_izv_197_19910820'],
                 'claim_ids': ['su_izv197_statement_lists_pavlov_as_premier_19910818']}),
            'existing holder changed': lambda p: role(p, PRES)['holder_claims'][1].update({'until': '1991-12-25'}),
            'existing holder reconciled': lambda p: role(p, PRES)['holder_claims'][0].update({'name': GORB}),
            'state claim on the party role': lambda p: role(p, 'su_cpsu_general_secretary')['claim_ids'].append(
                'su_ved35p_up2443_president_signs_19910822'),
            'Presidency lifecycle ended': lambda p: next(e for e in p['institutions'] if e['id'] == PRES_INST)['lifecycle'].update(
                {'until': '1991-12-25'}),
            'claim day moved past the period': lambda p: claim(p, 'su_ved41p_up2668_president_signs_19911005').update(
                {'attested_on': '1991-12-26'}),
        }
        for name, change in cases.items():
            with self.subTest(name), self.assertRaises((AssertionError, KeyError, IndexError, StopIteration)):
                c49_invariants(mutated(change), self.russia)
        # The validator rejects a broken snapshot and a date past the cutoff.
        packet = mutated(lambda p: next(s for s in p['sources'] if s['id'] == 'su_izv_197_19910820')['snapshot'].update(
            sha256='0' * 64))
        with self.assertRaisesRegex(ValueError, 'checksum mismatch'):
            self.validate(packet)
        packet = mutated(lambda p: role(p, PRES)['holder_claims'][4].update({'attested_on': '2026-09-08'}))
        with self.assertRaisesRegex(ValueError, 'exceeds cutoff'):
            self.validate(packet)
        # An extract whose text differs from the packet fails the extract check.
        packet = mutated(lambda p: claim(p, 'su_pravda20_premier_signs_resolution_19910122').update({'text': 'changed'}))
        with self.assertRaises(AssertionError):
            c49_extract_check(packet, research.ROOT)


if __name__ == '__main__':
    unittest.main()
