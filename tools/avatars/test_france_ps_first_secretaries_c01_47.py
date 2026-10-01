"""CLAUDE-C01-47: the Parti socialiste's Premier secrétaire, 1 January 1990 to the cutoff (Pierre Mauroy to Olivier Faure), added as
the party role fr_ps_first_secretary on the CNCCFP organization fr_cnccfp_76 through the importer's supplement (organization_roles).
Elections, results, ratifications, departures and resignations stay separate dated claims; holders are dated observations only
(attested_on), never inferred boundaries; acting, delegated, provisional and collective arrangements are claims only; party office
and state office stay separate, and CLAUDE-C01-23's presidency and CLAUDE-C01-37/38's prime ministers stay unchanged."""
import copy
import hashlib
import json
import re
import unittest
from urllib.parse import urlsplit

import campaign_research as research
import import_cnccfp_census as importer

ORG, ROLE, TITLE = 'fr_cnccfp_76', 'fr_ps_first_secretary', 'Premier secrétaire'
PRIOR_SUPPLEMENT_SOURCES = 49 + 31 + 22  # CLAUDE-C01-23, CLAUDE-C01-37 and CLAUDE-C01-38
FR_PM_HOLDERS = 16 + 12
ACCESSED = '2026-10-01'

# Original response identity recorded in each extract: (bytes, sha256), the body as received.
RESPONSES = {
    'fr_ps_vendredi_054_19900324': (24168728, '1a11ce4bbe47f2a006dbb25169c4be4a9bd70310eaafa574b367e30350f2d1b6'),
    'fr_ps_vendredi_127_19920110': (20394417, 'a7298aee6e88168634c3e54881b7dac6d7cb964ba6e4774c55816104d63cf5ae'),
    'fr_ps_vendredi_183_19930409': (21583640, 'e665b9e3b5a2c2351f452b938852d8cad9d381aad4890309803d1cf00bf709aa'),
    'fr_ps_vendredi_205_supplement_19931024': (14336813, 'bf1a45d245e0393074edea012c5d77fc41728c26ee93d3dc5695f44661c13f06'),
    'fr_ps_vendredi_207_19931112': (11885552, '0a9942cecfb918be9571520aa7dbe9b6253fbf8b8f5d7b5ba3cdb2695c3d2107'),
    'fr_ps_vendredi_234_19940624': (11784262, '2bd8037dbbc0177933247067687c046259881b3872e83fd4cfe1317c95e66b3c'),
    'fr_ps_vendredi_261_19951020': (13177998, '06ede7d0fe7a723b351f1eb48be4c409276d0decd7b887f3c132ca9467165567'),
    'fr_ps_hebdo_023_19970606': (2419753, '503163bf18764570a5909ec45b816231a3befd68e55617f84a7964de4762342f'),
    'fr_ps_hebdo_041_19971127': (947874, 'c972996885273f425cd98362e144a3b94aa931043c8809c1165fb9aa72307be6'),
    'fr_ps_hebdo_043_19971205': (5233445, '5a91eac0fd9dd62527d410ae2587e2a5fd4df7c0247c264bb3d1972da6e2a428'),
    'fr_ps_hebdo_508_20081129': (8842715, 'c65e635bb8854875529c92d2a26e58cd39b1beeefc96cb08ca4be083320a5b82'),
    'fr_ps_hebdo_667_20121016': (3273847, 'f70ccf3ee5afb22777ca64d3b8d286e1d9e8339e05d93484a67611b8f5ebd161'),
    'fr_ps_hebdo_677_20130112': (588055, '5bfd603f651640cdc6ec8aefdaf61e8753c887873d77867f3c12065334390371'),
    'fr_ps_hebdo_730_731_20140419': (780537, 'e052d534f4e886f28239d1a960912663c7bf55c563cc63e5e120c993f665b66e'),
    'fr_ps_hebdo_867_20170617': (778786, '0a3e1a44c5828c2c69c4dd35001637f32bdaae585dcd744a00a9b5178be29cee'),
    'fr_ps_hebdo_868_869_20170701': (2709620, '8f407b58d29d99b87ac2db8fd7f8a0a52e31ea46db1eade6d9f85215b9990330'),
    'fr_ps_site_vote_20180316': (121043, 'c85090e7a8fea249996bb22adbb5d3541a26fdf0fddb5496a879ab11bb0a4eee'),
    'fr_ps_site_press_conference_2018': (115977, 'a55b6460312621dd154c47370c1210aecf6be856937714cea5788bb5a40144d4'),
    'fr_ps_site_versailles_reaction_20180709': (120083, '57fc330c3a2dd0d05a9ebb454c2b1752bb90f7ecd4b1d56f468e13c64133f963'),
    'fr_ps_site_reconduit_20230120': (53341, '5f434b90ab670588983ba4b23feeecb6b9b6ab7539ec42b0da5b81b3d58715d3'),
    'fr_ps_site_recolement_20230122': (55076, '9fe66b88b5bdc87c1dbcf2f5033fa1946304807a9070b1afa7b06f5a69705a3e'),
    'fr_ps_site_conseil_national_20230311': (54412, '4a59851ea5e027235976ee0737c88129a3e3b449d47cd075df1479e36b9d640a'),
    'fr_ps_site_congress_ratification_2025': (11622, '2cfc536b7f04a4dbe223293b59119d9fb87d4bf2c4cd4f7f481c7afccfed2258'),
    'fr_ps_site_premier_secretaire_page_2026': (254458, '40658ac6f5932439c33b2c8e24b17f2593e7addbe0a41da8332831572456117f'),
}
# The party weeklies as served by the Fondation Jean-Jaurès's archive: (catalogue record id, image id, stored file).
ARCHIVE = {
    'fr_ps_vendredi_054_19900324': (5646, 3210, 'depot/presse/vendredi/vendredi_054.pdf'),
    'fr_ps_vendredi_127_19920110': (5730, 3293, 'depot/presse/vendredi/vendredi_127.pdf'),
    'fr_ps_vendredi_183_19930409': (5793, 3355, 'depot/presse/vendredi/vendredi_183.pdf'),
    'fr_ps_vendredi_205_supplement_19931024': (5821, 3381, 'depot/presse/vendredi/vendredi_205supp.pdf'),
    'fr_ps_vendredi_207_19931112': (5827, 3384, 'depot/presse/vendredi/vendredi_207.pdf'),
    'fr_ps_vendredi_234_19940624': (5858, 3414, 'depot/presse/vendredi/vendredi_234.pdf'),
    'fr_ps_vendredi_261_19951020': (5889, 3444, 'depot/presse/vendredi/vendredi_261.pdf'),
    'fr_ps_hebdo_023_19970606': (20543, 10933, 'depot/presse/hds/hds_023_p.pdf'),
    'fr_ps_hebdo_041_19971127': (14548, 7342, 'depot/presse/hds/hds_041_p_ocr.pdf'),
    'fr_ps_hebdo_043_19971205': (14552, 7344, 'depot/presse/hds/hds_043_p_ocr.pdf'),
    'fr_ps_hebdo_508_20081129': (14592, 7381, 'depot/presse/hds/hds_508_p_ocr.pdf'),
    'fr_ps_hebdo_667_20121016': (14600, 7387, 'depot/presse/hds/hds_667_n.pdf'),
    'fr_ps_hebdo_677_20130112': (21578, 11571, 'depot/presse/hds/hds_677_n.pdf'),
    'fr_ps_hebdo_730_731_20140419': (21654, 11617, 'depot/presse/hds/hds_730-731_n.pdf'),
    'fr_ps_hebdo_867_20170617': (21808, 11729, 'depot/presse/hds/hds_867_n.pdf'),
    'fr_ps_hebdo_868_869_20170701': (21809, 11730, 'depot/presse/hds/hds_868-869_n.pdf'),
}
# Raw Internet Archive captures of the party website (capture timestamp).
WAYBACK = {
    'fr_ps_site_vote_20180316': '20180409042650',
    'fr_ps_site_press_conference_2018': '20180414154012',
    'fr_ps_site_versailles_reaction_20180709': '20180710122030',
    'fr_ps_site_reconduit_20230120': '20230120083422',
    'fr_ps_site_recolement_20230122': '20230122153522',
    'fr_ps_site_conseil_national_20230311': '20230313182432',
    'fr_ps_site_congress_ratification_2025': '20250617103637',
    'fr_ps_site_premier_secretaire_page_2026': '20260626072954',
}
DECODED = {
    'fr_ps_site_congress_ratification_2025': (59386, '83d1ba01424a04a42c4f8fd50bd88ed26e2888d54e797d894dd6d710c2e6f615'),
    'fr_ps_site_premier_secretaire_page_2026': (696397, '4870a5633ff773d6dc134fa4961a3f17cb79a639f44ad7916ae8365c697a75c2'),
}
EVENTS = {
    'fr_ps_mauroy_reelected_by_comite_directeur_1990': ('1990-03-24', 'election_by_comite_directeur', 'FR-PS-01'),
    'fr_ps_mauroy_listed_premier_secretaire_19900324': ('1990-03-24', 'in_office_listing', 'FR-PS-01'),
    'fr_ps_mauroy_mandate_at_disposal_19920107': ('1992-01-07', 'mandate_placed_at_disposal', 'FR-PS-01'),
    'fr_ps_mauroy_elected_19880514_retrospective': ('1988-05-14', 'election_retrospective', 'FR-PS-01'),
    'fr_ps_fabius_elected_by_comite_directeur_19920109': ('1992-01-09', 'election_by_comite_directeur', 'FR-PS-02'),
    'fr_ps_fabius_acceptance_address_19920110': ('1992-01-10', 'acceptance_statement', 'FR-PS-02'),
    'fr_ps_fabius_styled_premier_secretaire_19920110': ('1992-01-10', 'in_office_styling', 'FR-PS-02'),
    'fr_ps_mauroy_leaves_19920110': ('1992-01-10', 'departure_reported', 'FR-PS-01'),
    'fr_ps_fabius_resigned_comite_directeur_19930403': ('1993-04-03', 'resignation_reported', 'FR-PS-02'),
    'fr_ps_direction_provisoire_installed_19930406': ('1993-04-06', 'provisional_direction_installed', 'FR-PS-03'),
    'fr_ps_rocard_presides_direction_provisoire_19930406': ('1993-04-06', 'interim_presidency', 'FR-PS-03'),
    'fr_ps_rocard_elected_by_congress_19931023': ('1993-10-23', 'election_by_congress', 'FR-PS-03'),
    'fr_ps_rocard_in_office_19931112': ('1993-11-12', 'in_office_styling', 'FR-PS-03'),
    'fr_ps_rocard_no_confidence_vote_19940619': ('1994-06-19', 'no_confidence_vote', 'FR-PS-03'),
    'fr_ps_emmanuelli_elected_by_conseil_national_19940619': ('1994-06-19', 'election_by_conseil_national', 'FR-PS-04'),
    'fr_ps_emmanuelli_styled_premier_secretaire_19940624': ('1994-06-24', 'in_office_styling', 'FR-PS-04'),
    'fr_ps_jospin_members_vote_result_1995': ('1995-10-20', 'members_vote_result', 'FR-PS-05'),
    'fr_ps_jospin_secretariat_national_1995': ('1995-10-20', 'in_office_listing', 'FR-PS-05'),
    'fr_ps_jospin_receives_flnks_19951018': ('1995-10-18', 'in_office_styling', 'FR-PS-05'),
    'fr_ps_hollande_premier_secretaire_delegue_1997': ('1997-06-06', 'delegated_office', 'FR-PS-06'),
    'fr_ps_jospin_remains_premier_secretaire_19970606': ('1997-06-06', 'in_office_statement', 'FR-PS-05'),
    'fr_ps_first_secretary_ballot_19971127': ('1997-11-27', 'members_vote_scheduled', 'FR-PS-06'),
    'fr_ps_hollande_members_vote_result_1997': ('1997-12-05', 'members_vote_result', 'FR-PS-06'),
    'fr_ps_hollande_styled_premier_secretaire_19971205': ('1997-12-05', 'in_office_styling', 'FR-PS-06'),
    'fr_ps_hollande_farewell_20081119': ('2008-11-19', 'departure_farewell', 'FR-PS-06'),
    'fr_ps_recolement_report_approved_20081125': ('2008-11-25', 'members_vote_result', 'FR-PS-07'),
    'fr_ps_aubry_elected_20081125': ('2008-11-25', 'election_proclaimed_by_conseil_national', 'FR-PS-07'),
    'fr_ps_aubry_styled_premiere_secretaire_20081129': ('2008-11-29', 'in_office_styling', 'FR-PS-07'),
    'fr_ps_first_secretary_ballot_20121018': ('2012-10-18', 'members_vote_scheduled', 'FR-PS-08'),
    'fr_ps_desir_at_jarnac_20130108': ('2013-01-08', 'in_office_styling', 'FR-PS-08'),
    'fr_ps_cambadelis_elected_by_conseil_national_20140415': ('2014-04-15', 'election_by_conseil_national', 'FR-PS-09'),
    'fr_ps_desir_left_for_government_2014': ('2014-04-19', 'departure_reported', 'FR-PS-08'),
    'fr_ps_cambadelis_listed_premier_secretaire_20140419': ('2014-04-19', 'in_office_listing', 'FR-PS-09'),
    'fr_ps_cambadelis_cedes_place_2017': (None, 'resignation_announced', 'FR-PS-09'),
    'fr_ps_bureau_national_collegial_direction_20170620': ('2017-06-20', 'interim_arrangement', 'FR-PS-10'),
    'fr_ps_cambadelis_former_premier_secretaire_20170624': ('2017-06-24', 'former_holder_reference', 'FR-PS-09'),
    'fr_ps_collegial_direction_pending_20170624': ('2017-06-24', 'interim_arrangement', 'FR-PS-10'),
    'fr_ps_orientation_text_vote_20180315': ('2018-03-15', 'orientation_text_vote_result', 'FR-PS-11'),
    'fr_ps_temal_national_coordinator_20180316': ('2018-03-16', 'interim_coordinator_styling', 'FR-PS-10'),
    'fr_ps_faure_premier_secretaire_elu_20180329': ('2018-03-29', 'members_vote_election', 'FR-PS-11'),
    'fr_ps_faure_signed_statement_20180709': ('2018-07-09', 'in_office_signed_statement', 'FR-PS-11'),
    'fr_ps_members_vote_20230119': ('2023-01-19', 'members_vote_result', 'FR-PS-11'),
    'fr_ps_recolement_result_20230122': ('2023-01-22', 'members_vote_result', 'FR-PS-11'),
    'fr_ps_faure_closes_conseil_national_20230311': ('2023-03-11', 'in_office_styling', 'FR-PS-11'),
    'fr_ps_ratification_81e_congres_20250605': (None, 'members_vote_ratified', 'FR-PS-11'),
    'fr_ps_page_faure_elected_20180315_retrospective': ('2018-03-15', 'election_retrospective', 'FR-PS-11'),
    'fr_ps_page_faure_reconduit_retrospective': (None, 'reelection_retrospective', 'FR-PS-11'),
}
HOLDERS = [
    ('Pierre Mauroy', '1990-03-24', None, None),
    ('Pierre Mauroy', '1992-01-07', None, None),
    ('Laurent Fabius', '1992-01-10', None, None),
    ('Michel Rocard', '1993-11-12', None, None),
    ('Henri Emmanuelli', '1994-06-24', None, None),
    ('Lionel Jospin', '1995-10-18', None, None),
    ('Lionel Jospin', '1997-06-06', None, None),
    ('François Hollande', '1997-12-05', None, None),
    ('Martine Aubry', '2008-11-29', None, None),
    ('Harlem Désir', '2013-01-08', None, None),
    ('Jean-Christophe Cambadélis', '2014-04-19', None, None),
    ('Olivier Faure', '2018-07-09', None, None),
    ('Olivier Faure', '2023-03-11', None, None),
]
HOLDER_CLAIMS = [
    ['fr_ps_mauroy_listed_premier_secretaire_19900324'],
    ['fr_ps_mauroy_mandate_at_disposal_19920107'],
    ['fr_ps_fabius_styled_premier_secretaire_19920110'],
    ['fr_ps_rocard_in_office_19931112'],
    ['fr_ps_emmanuelli_styled_premier_secretaire_19940624'],
    ['fr_ps_jospin_receives_flnks_19951018'],
    ['fr_ps_jospin_remains_premier_secretaire_19970606'],
    ['fr_ps_hollande_styled_premier_secretaire_19971205'],
    ['fr_ps_aubry_styled_premiere_secretaire_20081129'],
    ['fr_ps_desir_at_jarnac_20130108'],
    ['fr_ps_cambadelis_listed_premier_secretaire_20140419'],
    ['fr_ps_faure_signed_statement_20180709'],
    ['fr_ps_faure_closes_conseil_national_20230311'],
]

NEW_SOURCES = list(RESPONSES)
PEOPLE = ['Pierre Mauroy', 'Laurent Fabius', 'Michel Rocard', 'Henri Emmanuelli', 'Lionel Jospin', 'François Hollande',
          'Martine Aubry', 'Harlem Désir', 'Jean-Christophe Cambadélis', 'Olivier Faure']
HOLDER_KINDS = {'in_office_listing', 'in_office_styling', 'in_office_statement', 'in_office_signed_statement',
                'mandate_placed_at_disposal'}
ELECTION_KINDS = {'election_by_comite_directeur', 'election_by_congress', 'election_by_conseil_national',
                  'election_proclaimed_by_conseil_national', 'members_vote_election', 'members_vote_result', 'members_vote_ratified',
                  'members_vote_scheduled', 'orientation_text_vote_result', 'acceptance_statement'}
DEPARTURE_KINDS = {'departure_reported', 'departure_farewell', 'resignation_reported', 'resignation_announced', 'no_confidence_vote',
                   'former_holder_reference'}
INTERIM_KINDS = {'provisional_direction_installed', 'interim_presidency', 'delegated_office', 'interim_arrangement',
                 'interim_coordinator_styling'}
RETROSPECTIVE_KINDS = {'election_retrospective', 'reelection_retrospective'}
NEVER_KINDS = ELECTION_KINDS | DEPARTURE_KINDS | INTERIM_KINDS | RETROSPECTIVE_KINDS
NEVER_HOLDER = tuple(cid for cid, (_, kind, _) in EVENTS.items() if kind in NEVER_KINDS)
INTERIM_NAMES = ('Rachid Temal', 'Rachid TEMAL', 'direction provisoire', 'direction collégiale', 'direction collective')
LEAD_MARKERS = ('wikipedia', 'wikipédia', 'britannica', 'larousse', 'france24', 'franceinfo', 'lemonde', 'liberation', 'vie-publique')
REPORT = research.RESEARCH / 'france-ps-first-secretaries-1990-2026-47.md'
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-47.md'


def expected_organizations(base, supplement):
    """The CNCCFP organizations with the supplement's organization_roles appended to their targets only."""
    expected = copy.deepcopy(base['organizations'])
    for addition in supplement['organization_roles']:
        target = next(o for o in expected if o['id'] == addition['organization_id'])
        for key in ('roles', 'sources', 'claim_ids'):
            target[key] = target[key] + addition[key]
        target['coverage']['unresolved'] = target['coverage']['unresolved'] + addition['coverage_unresolved']
    return expected


def party_invariants(packet):
    """Packet-level rules this test owns; raises AssertionError, KeyError, IndexError or StopIteration on any violation."""
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    owner = {c['id']: s['id'] for s in packet['sources'] for c in s['claims']}
    org = next(o for o in packet['organizations'] if o['id'] == ORG)
    assert org['name'] == 'PARTI SOCIALISTE' and org['external_ids'] == {'CNCCFP': '76'}
    assert org['represented_party_ids'] == [] and org['representation_status'] == 'unreconciled', 'no game mapping'
    assert org['lifecycle']['status'] == 'unknown' and 'from' not in org['lifecycle'] and 'until' not in org['lifecycle']
    assert [(r['id'], r['kind'], r['title']) for r in org['roles']] == [(ROLE, 'party_leader', TITLE)], 'exactly one party role'
    for entry in packet['organizations']:
        if entry['id'] != ORG:
            assert entry['roles'] == [] and not set(entry['sources']) & set(NEW_SOURCES), entry['id']
            assert not set(entry['claim_ids']) & set(EVENTS), entry['id']
    assert [e['id'] for e in packet['institutions']] == ['fr_presidency', 'fr_prime_minister']
    assert [r['id'] for e in packet['institutions'] for r in e['roles']] == ['fr_president', 'fr_pm']
    role = org['roles'][0]
    assert role['claim_ids'] == list(EVENTS) and role['sources'] == NEW_SOURCES
    assert org['claim_ids'][-len(EVENTS):] == list(EVENTS) and org['sources'][-len(NEW_SOURCES):] == NEW_SOURCES
    assert org['claim_ids'][:2] == [f'{ORG}_reported_2024', f'{ORG}_publication_2024']
    holders = role['holder_claims']
    assert [(h['name'], h['attested_on'], h['from'], h['until']) for h in holders] == HOLDERS
    assert [h['claim_ids'] for h in holders] == HOLDER_CLAIMS
    assert sorted({h['name'] for h in holders}, key=PEOPLE.index) == PEOPLE and len(PEOPLE) <= 10, 'at most ten people'
    for index, h in enumerate(holders):
        assert h['from'] is None and h['until'] is None, 'no reviewed party record states a start or an end'
        assert not set(h['claim_ids']) & set(NEVER_HOLDER), h['name']
        assert h['attested_on'] == claims[h['claim_ids'][0]]['attested_on'], 'the observation is the day of its basis claim'
        assert all(EVENTS[cid][1] in HOLDER_KINDS for cid in h['claim_ids']), h['name']
        assert h['sources'] == list(dict.fromkeys(owner[cid] for cid in h['claim_ids'])), h['name']
        for name in INTERIM_NAMES:
            assert name not in h['name'], 'interim, delegated or collective service is claims only'
    for cid, (day, kind, _) in EVENTS.items():
        assert claims[cid].get('attested_on') == day, cid
    # Separation both ways: no institution cites this role's sources or claims, and this role cites none of theirs.
    institution_claims = {cid for e in packet['institutions'] for r in e['roles'] for cid in r['claim_ids']}
    for entry in packet['institutions']:
        assert not set(entry['sources']) & set(NEW_SOURCES) and not set(entry['claim_ids']) & set(EVENTS), entry['id']
        for r in entry['roles']:
            assert not set(r['claim_ids']) & set(EVENTS)
            for h in r['holder_claims']:
                assert not set(h['claim_ids']) & set(EVENTS), h['name']
    for h in holders:
        assert not set(h['claim_ids']) & institution_claims, h['name']
    pm = packet['institutions'][1]['roles'][0]
    assert len(pm['holder_claims']) == FR_PM_HOLDERS
    return role


class FrancePsFirstSecretariesTests(unittest.TestCase):
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
        cls.org = next(o for o in cls.packet['organizations'] if o['id'] == ORG)
        cls.role = cls.org['roles'][0]
        cls.extracts = {sid: json.loads((research.ROOT / cls.sources[sid]['snapshot']['path']).read_text(encoding='utf-8'))
                        for sid in NEW_SOURCES}
        cls.rows = {row['claim_id']: row for sid in NEW_SOURCES for row in cls.extracts[sid]['rows']}
        cls.new_claims = [c['id'] for sid in NEW_SOURCES for c in cls.sources[sid]['claims']]
        cls.report = (research.ROOT / REPORT).read_text(encoding='utf-8')

    def validate(self, packet=None):
        return research.validate(packet or self.packet, research.ROOT, {'France'}, {'France': {'France/fr_ps'}})

    def section(self, heading):
        parts = self.report.split(f'\n## {heading}', 1)
        self.assertEqual(len(parts), 2, heading)
        return parts[1].split('\n## ', 1)[0]

    def test_packet_is_the_importer_output_and_only_appends_to_the_supplement(self):
        built = importer.build(self.csv)
        self.assertEqual(self.raw, json.dumps(built, ensure_ascii=False, indent=2) + '\n')
        self.assertNotIn(b'\r', self.supplement_bytes)
        self.assertEqual(self.supplement_bytes.decode('utf-8'), json.dumps(self.supplement, indent=2, ensure_ascii=False) + '\n')
        self.assertEqual(list(self.supplement), ['sources', 'institutions', 'coverage_unresolved', 'organization_roles'])
        ids = [s['id'] for s in self.supplement['sources']]
        self.assertEqual(len(ids), PRIOR_SUPPLEMENT_SOURCES + len(NEW_SOURCES))
        self.assertEqual(ids[PRIOR_SUPPLEMENT_SOURCES:], NEW_SOURCES)
        self.assertEqual([s['id'] for s in self.packet['sources']][-len(NEW_SOURCES):], NEW_SOURCES)
        notes = self.supplement['coverage_unresolved']
        self.assertEqual(len(notes), 4)
        self.assertTrue(notes[3].startswith('CLAUDE-C01-47 adds the party role fr_ps_first_secretary'))
        self.assertEqual(self.packet['coverage']['unresolved'][-4:], notes)
        addition, = self.supplement['organization_roles']
        self.assertEqual(list(addition), ['organization_id', 'roles', 'sources', 'claim_ids', 'coverage_unresolved'])
        self.assertEqual((addition['organization_id'], addition['sources'], addition['claim_ids']), (ORG, NEW_SOURCES, list(EVENTS)))
        self.assertEqual(len(addition['coverage_unresolved']), 4)
        self.assertTrue(addition['coverage_unresolved'][0].startswith('CLAUDE-C01-47 adds the party role'))
        self.assertIn('France/fr_ps stays unmapped', addition['coverage_unresolved'][3])
        # Only fr_cnccfp_76 changes, and only by appending; the institutions are the supplement's, unchanged by this packet.
        base = importer.build(self.csv, supplement=b'{"sources": [], "institutions": [], "coverage_unresolved": []}')
        self.assertEqual(self.packet['organizations'], expected_organizations(base, self.supplement))
        before = next(o for o in base['organizations'] if o['id'] == ORG)
        self.assertEqual((before['roles'], before['coverage']['status']), ([], self.org['coverage']['status']))
        self.assertEqual(self.org['coverage']['unresolved'][:len(before['coverage']['unresolved'])], before['coverage']['unresolved'])
        self.assertEqual({k: v for k, v in self.org.items() if k not in ('roles', 'sources', 'claim_ids', 'coverage')},
                         {k: v for k, v in before.items() if k not in ('roles', 'sources', 'claim_ids', 'coverage')})
        self.assertEqual(self.packet['institutions'], self.supplement['institutions'])

    def test_importer_refuses_malformed_organization_roles(self):
        good = json.loads(self.supplement_bytes)

        def encoded(change):
            value = copy.deepcopy(good)
            change(value)
            return json.dumps(value, ensure_ascii=False).encode('utf-8')

        def addition(v):
            return v['organization_roles'][0]

        cases = [
            (lambda v: v.update(organizations=[]), 'expected exactly'),
            (lambda v: v.update(organization_roles={}), 'entries'),
            (lambda v: addition(v).pop('coverage_unresolved'), 'organization_roles'),
            (lambda v: addition(v).update(roles=[]), 'organization_roles'),
            (lambda v: addition(v)['coverage_unresolved'].append(' '), 'organization_roles'),
            (lambda v: addition(v).update(organization_id='fr_cnccfp_999999'), 'names no organization'),
            (lambda v: v['organization_roles'].append(copy.deepcopy(addition(v))), 'id collision'),
            (lambda v: addition(v)['roles'][0].update(id='fr_pm'), 'role id collision'),
            (lambda v: addition(v)['sources'].append('fr_cnccfp_2024'), 'its own supplement sources'),
            (lambda v: addition(v)['sources'].append(NEW_SOURCES[0]), 'its own supplement sources'),
            (lambda v: addition(v)['claim_ids'].append('fr_cnccfp_identity_scope'), 'its own supplement sources'),
            (lambda v: addition(v)['claim_ids'].append(f'{ORG}_publication_2024'), 'its own supplement sources'),
            (lambda v: addition(v).update(sources=NEW_SOURCES[1:]), 'its own supplement sources'),
        ]
        for change, message in cases:
            with self.subTest(message=message), self.assertRaisesRegex(ValueError, message):
                importer.build(self.csv, supplement=encoded(change))
        # Without the optional key the importer reproduces the packet without this role.
        without = encoded(lambda v: v.pop('organization_roles'))
        self.assertEqual(next(o for o in importer.build(self.csv, supplement=without)['organizations'] if o['id'] == ORG)['roles'], [])

    def test_new_records_are_bounded_and_every_claim_is_classified(self):
        ids = self.validate()
        self.assertEqual((len(NEW_SOURCES), len(self.new_claims)), (24, 47))
        self.assertEqual((len(ARCHIVE), len(WAYBACK), len(DECODED)), (16, 8, 2))
        self.assertEqual(len(ids['entries']), 637)
        self.assertEqual(ids['roles'], {'fr_president', 'fr_pm', ROLE})
        self.assertEqual(self.new_claims, list(EVENTS))
        holder_claims = {cid for ids_ in HOLDER_CLAIMS for cid in ids_}
        self.assertFalse(holder_claims & set(NEVER_HOLDER))
        self.assertEqual({self.rows[cid]['event_kind'] for cid in holder_claims}, HOLDER_KINDS)
        self.assertEqual({kind for _, kind, _ in EVENTS.values()}, NEVER_KINDS | HOLDER_KINDS)
        self.assertFalse(HOLDER_KINDS & NEVER_KINDS)
        observations = re.findall(r'^### (FR-PS-\d\d)\b', self.report, re.M)
        self.assertEqual(observations, [f'FR-PS-{n:02d}' for n in range(1, 12)])
        self.assertEqual({row['review_observation'] for row in self.rows.values()}, {f'FR-PS-{n:02d}' for n in range(1, 12)})
        # Elections, results and departures keep separate claims with only supported dates; acting and collective arrangements name no holder.
        kinds = [kind for _, kind, _ in EVENTS.values()]
        self.assertEqual((sum(k in ELECTION_KINDS for k in kinds), sum(k in DEPARTURE_KINDS for k in kinds),
                          sum(k in INTERIM_KINDS for k in kinds), sum(k in HOLDER_KINDS for k in kinds)), (17, 7, 6, 14))
        for cid, (_, kind, _) in EVENTS.items():
            if kind in INTERIM_KINDS or kind in ('members_vote_scheduled', 'orientation_text_vote_result'):
                self.assertIsNone(self.rows[cid]['holder_name'], cid)
            else:
                self.assertTrue(self.rows[cid]['holder_name'], cid)

    def test_holders_are_exactly_as_intended(self):
        party_invariants(self.packet)
        for holder in self.role['holder_claims']:
            self.assertTrue(holder['note'].startswith('Observed on ') and 'No start or end established' in holder['uncertainty'])
            for cid in holder['claim_ids']:
                row = self.rows[cid]
                self.assertEqual((row['role_id'], row['observation_id'], row['role_title']), (ROLE, ORG, TITLE), cid)
                self.assertIn(holder['name'].split()[-1].casefold(), row['holder_name'].casefold(), cid)
        # Names as the sources print them.
        self.assertIn("'Harlem Désir, Premier secrétaire du PS", self.claims['fr_ps_desir_at_jarnac_20130108']['text'])
        self.assertIn("'Premier secrétaire – Jean-Christophe Cambadélis'",
                      self.claims['fr_ps_cambadelis_listed_premier_secretaire_20140419']['text'])
        self.assertIn("'Olivier FAURE Premier secrétaire du Parti socialiste", self.claims['fr_ps_faure_signed_statement_20180709']['text'])
        for cid in NEVER_HOLDER:
            self.assertTrue(self.claims[cid]['uncertainty'], cid)
        for cid, (_, kind, _) in EVENTS.items():
            if kind in DEPARTURE_KINDS:
                self.assertRegex(self.claims[cid]['uncertainty'], r'never an end|gives no end|no end', cid)
            if kind in INTERIM_KINDS:
                self.assertRegex(self.claims[cid]['uncertainty'], r'claims only|never a holder', cid)

    def test_undated_announcements_do_not_borrow_issue_or_vote_days(self):
        # The June issue covers 17–30 June and contains an explicitly dated 20 June report.
        # Its interval start cannot date its undated resignation announcement.
        self.assertIsNone(self.claims['fr_ps_cambadelis_cedes_place_2017']['attested_on'])
        self.assertIsNone(self.sources['fr_ps_hebdo_867_20170617'].get('published_date'))
        self.assertIsNone(self.extracts['fr_ps_hebdo_867_20170617']['published_date'])
        self.assertEqual(self.claims['fr_ps_bureau_national_collegial_direction_20170620']['attested_on'], '2017-06-20')
        self.assertIn('17 to 30 June 2017', self.sources['fr_ps_hebdo_867_20170617']['scope_note'])
    def test_ratification_day_does_not_borrow_vote_day(self):
        # The communiqué ratifies two earlier votes; neither is the unknown ratification day.
        self.assertIsNone(self.claims['fr_ps_ratification_81e_congres_20250605']['attested_on'])
        self.assertIn('27 May and 5 June 2025', self.claims['fr_ps_ratification_81e_congres_20250605']['text'])
        for cid in ('fr_ps_cambadelis_cedes_place_2017', 'fr_ps_ratification_81e_congres_20250605'):
            self.assertIsNone(self.rows[cid]['attested_on'])
            self.assertNotIn(cid, {c for h in self.role['holder_claims'] for c in h['claim_ids']})
        party_invariants(self.packet)

    def test_flnks_locator_names_its_own_dated_item(self):
        claim = self.claims['fr_ps_jospin_receives_flnks_19951018']
        self.assertEqual(claim['attested_on'], '1995-10-18')
        self.assertEqual(claim['locator'], {
            'pdf_page_1_based': 14,
            'item': "FLNKS delegation communiqué, lower-left column, dated '18 octobre'; not the adjacent '17 octobre' items"})
        self.assertEqual(self.rows[claim['id']]['locator'], claim['locator'])

    def test_distinct_events_keep_distinct_claims(self):
        claims = self.claims
        # Each election and its first holder observation are separate claims on different days.
        for election, observation in (('fr_ps_fabius_elected_by_comite_directeur_19920109', 'fr_ps_fabius_styled_premier_secretaire_19920110'),
                                      ('fr_ps_rocard_elected_by_congress_19931023', 'fr_ps_rocard_in_office_19931112'),
                                      ('fr_ps_emmanuelli_elected_by_conseil_national_19940619', 'fr_ps_emmanuelli_styled_premier_secretaire_19940624'),
                                      ('fr_ps_aubry_elected_20081125', 'fr_ps_aubry_styled_premiere_secretaire_20081129'),
                                      ('fr_ps_cambadelis_elected_by_conseil_national_20140415', 'fr_ps_cambadelis_listed_premier_secretaire_20140419'),
                                      ('fr_ps_faure_premier_secretaire_elu_20180329', 'fr_ps_faure_signed_statement_20180709')):
            self.assertLess(claims[election]['attested_on'], claims[observation]['attested_on'])
        # Relative days resolve from the dateline of the dated issue that prints them.
        self.assertIn("'jeudi' is 9 January 1992", claims['fr_ps_fabius_elected_by_comite_directeur_19920109']['text'])
        self.assertIn("'samedi' is 23 October 1993", claims['fr_ps_rocard_elected_by_congress_19931023']['text'])
        # A predecessor's departure and a successor's election stay apart, and no departure is an end.
        for departure, successor in (('fr_ps_mauroy_leaves_19920110', 'fr_ps_fabius_elected_by_comite_directeur_19920109'),
                                     ('fr_ps_rocard_no_confidence_vote_19940619', 'fr_ps_emmanuelli_elected_by_conseil_national_19940619'),
                                     ('fr_ps_hollande_farewell_20081119', 'fr_ps_aubry_elected_20081125'),
                                     ('fr_ps_desir_left_for_government_2014', 'fr_ps_cambadelis_elected_by_conseil_national_20140415')):
            self.assertNotEqual(departure, successor)
            people = [h for h in self.role['holder_claims'] if h['name'] == self.rows[departure]['holder_name']]
            self.assertTrue(people and all(h['until'] is None for h in people), departure)
        # The 2023 vote: provisional result, reviewed count and the next observation are three claims.
        self.assertEqual([claims[c]['attested_on'] for c in ('fr_ps_members_vote_20230119', 'fr_ps_recolement_result_20230122',
                                                            'fr_ps_faure_closes_conseil_national_20230311')],
                         ['2023-01-19', '2023-01-22', '2023-03-11'])
        # The party page's retrospective 15 March 2018 election is kept beside the 29 March vote, not reconciled.
        self.assertIn('29 March', claims['fr_ps_page_faure_elected_20180315_retrospective']['uncertainty'])
        self.assertIsNone(claims['fr_ps_page_faure_reconduit_retrospective']['attested_on'])

    def test_extracts_match_packet_claims_and_record_original_responses(self):
        for sid in NEW_SOURCES:
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual(extract['format'], 'spheres-c01-derived-factual-table/v1')
            self.assertEqual((extract['source_id'], extract['source_url']), (sid, source['url']))
            self.assertEqual(extract['scope_note'], source['scope_note'])
            self.assertEqual(extract['access_method'], source['access_method'])
            self.assertEqual(extract['published_date'], source.get('published_date'))
            self.assertEqual((extract['accessed_date'], source['accessed_date']), (ACCESSED, ACCESSED))
            if source.get('published_date'):
                self.assertLessEqual(source['published_date'], research.CUTOFF)
            self.assertFalse(extract['source_response_checked_in'])
            self.assertEqual((extract['source_response_bytes'], extract['source_response_sha256']), RESPONSES[sid])
            for text in ('not checked into this repository', 'derived factual extract', 'same byte count and SHA-256',
                         'no Accept-Encoding header', 'more than 30 minutes apart'):
                self.assertIn(text, extract['provenance_note'], text)
            if sid in DECODED:
                self.assertIn('Content-Encoding: gzip', extract['provenance_note'])
                self.assertIn('the gzip stream', extract['provenance_note'])
                self.assertIn('gzip-compressed', source['scope_note'])
                self.assertEqual((extract['source_response_content_encoding'], extract['decoded_response_bytes'],
                                  extract['decoded_response_sha256']), ('gzip',) + DECODED[sid])
            else:
                self.assertNotIn('decoded_response_sha256', extract)
                self.assertIn('served without Content-Encoding', extract['provenance_note'])
            self.assertIn('No open license, portrait permission or likeness approval', extract['rights_note'])
            self.assertEqual(extract['rights_note'], source['rights_note'])
            self.assertIn('CLAUDE-C01-47 only', extract['bounded_scope'])
            snapshot = source['snapshot']
            self.assertRegex(snapshot['path'], r'^docs/campaign-certification/C01/research/sources/france-ps-(vendredi|hebdo|site)-[a-z0-9-]+-\d{8}-facts\.json$')
            data = (research.ROOT / snapshot['path']).read_bytes()
            self.assertEqual((len(data), hashlib.sha256(data).hexdigest()), (snapshot['bytes'], snapshot['sha256']))
            self.assertNotEqual(snapshot['sha256'], extract['source_response_sha256'])
            self.assertTrue(data.endswith(b'}\n') and b'\r' not in data)
            self.assertEqual(data.decode('utf-8'), json.dumps(extract, indent=2, ensure_ascii=False) + '\n')
            self.assertEqual([r['claim_id'] for r in extract['rows']], [c['id'] for c in source['claims']])
            for row, claim in zip(extract['rows'], source['claims']):
                self.assertEqual((row['text'], row['locator'], row['attested_on']), (claim['text'], claim['locator'], claim['attested_on']))
                self.assertEqual((row['observation_id'], row['role_id'], row['role_title']), (ORG, ROLE, TITLE))
                self.assertEqual((row['attested_on'], row['event_kind'], row['review_observation']), EVENTS[row['claim_id']])
                self.assertTrue(claim['uncertainty'])
        self.assertEqual({cid: (row['attested_on'], row['event_kind'], row['review_observation']) for cid, row in self.rows.items()}, EVENTS)
        for sid, (record, image, stored) in ARCHIVE.items():
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual(source['url'], f'https://archives-socialistes.fr/_recherche-images/download/{record}/image/{image}/0')
            self.assertEqual(source['source_type'], 'primary_party_newspaper_scan_pdf')
            self.assertEqual((extract['archive_record']['catalogue_record_id'], extract['archive_record']['stored_file']), (record, stored))
            self.assertFalse(extract['archive_record']['viewer_allow_download'])
            self.assertTrue(extract['archive_record']['catalogue_record_ark'].startswith('https://archives-socialistes.fr/ark:21895/'))
            for text in ('allowDownload to false', 'no login, form, cookie or token', 'Cache-Control: max-age=864000, public', 'ETag'):
                self.assertIn(text, extract['provenance_note'], (sid, text))
            self.assertIn("Fondation Jean-Jaurès's Centre d'archives socialistes", source['scope_note'])
            self.assertIn('OCR', extract['visual_review']['method'])
        self.assertEqual(self.extracts['fr_ps_hebdo_508_20081129']['visual_review']['pdf_pages_one_based'], [1])
        for sid, stamp in WAYBACK.items():
            source, extract = self.sources[sid], self.extracts[sid]
            url = urlsplit(source['url'])
            self.assertEqual(url.hostname, 'web.archive.org')
            self.assertTrue(url.path.startswith(f'/web/{stamp}id_/http'), sid)
            self.assertLess(stamp, '20260907')
            self.assertEqual(extract['original_url'], source['original_url'])
            self.assertIn(urlsplit(source['original_url']).hostname, ('www.parti-socialiste.fr', 'parti-socialiste.fr'))
            self.assertTrue(source['url'].endswith(source['original_url']), sid)
            self.assertEqual(extract['archive_capture_utc'].replace('-', '').replace(':', '').replace('T', '').rstrip('Z'), stamp)
            self.assertIn('Raw Internet Archive capture', extract['provenance_note'])
        self.assertEqual(set(ARCHIVE) | set(WAYBACK), set(NEW_SOURCES))
        self.assertFalse(set(ARCHIVE) & set(WAYBACK))

    def test_leads_and_volatile_responses_stay_out_of_the_packet(self):
        new = json.dumps([self.sources[s] for s in NEW_SOURCES], ensure_ascii=False).lower()
        for marker in LEAD_MARKERS:
            self.assertNotIn(marker, new, marker)
        for sid in NEW_SOURCES:
            url = self.sources[sid]['url']
            self.assertIsNone(re.search(r'[?&#]|/recherche\?|visionneuse|moteur', url), url)
        # Retrospective party pages are claims only and never a holder's basis.
        retrospective = {cid for cid, (_, kind, _) in EVENTS.items() if kind in RETROSPECTIVE_KINDS}
        self.assertEqual(retrospective, {'fr_ps_mauroy_elected_19880514_retrospective', 'fr_ps_page_faure_elected_20180315_retrospective',
                                         'fr_ps_page_faure_reconduit_retrospective'})
        self.assertEqual(self.sources['fr_ps_site_premier_secretaire_page_2026']['source_type'],
                         'primary_party_record_archived_retrospective')
        leads, added, attempted = self.section('Leads not imported'), self.section('Sources added'), self.section('Sources attempted')
        for marker in ('Wikipedia', 'France 24', 'vie-publique.fr'):
            self.assertIn(marker, leads + attempted, marker)
            self.assertNotIn(marker, added, marker)
        for marker in ('vie-publique.fr', 'JavaScript', 'not bypassed', 'HTTP 429', 'Temporarily Offline', 'allowDownload'):
            self.assertIn(marker, attempted, marker)

    def test_mutations_are_rejected(self):
        def mutated(change):
            packet = copy.deepcopy(self.packet)
            change(packet)
            return packet

        def org(packet):
            return next(o for o in packet['organizations'] if o['id'] == ORG)

        def role(packet):
            return org(packet)['roles'][0]

        def holder(packet, index):
            return role(packet)['holder_claims'][index]

        def source(packet, sid):
            return next(s for s in packet['sources'] if s['id'] == sid)

        def claim(packet, cid):
            return next(c for s in packet['sources'] for c in s['claims'] if c['id'] == cid)

        validator_cases = [
            (lambda p: source(p, 'fr_ps_vendredi_054_19900324')['snapshot'].update(sha256='0' * 64), 'checksum mismatch'),
            (lambda p: source(p, 'fr_ps_site_congress_ratification_2025')['snapshot'].update(bytes=1), 'checksum mismatch'),
            (lambda p: holder(p, 12).update(until='2026-09-08'), 'exceeds cutoff'),
            (lambda p: claim(p, 'fr_ps_faure_closes_conseil_national_20230311').update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: holder(p, 4).update({'from': '1994-06-24', 'until': '1994-06-19'}), 'Reversed historical interval'),
            (lambda p: holder(p, 3)['claim_ids'].append('fr_ps_emmanuelli_styled_premier_secretaire_19940624'), 'cited source'),
            (lambda p: role(p)['claim_ids'].append('fr_ps_does_not_exist'), 'Unknown'),
        ]
        for change, message in validator_cases:
            with self.subTest(message=message), self.assertRaisesRegex(ValueError, message):
                self.validate(mutated(change))
        interim = {'name': 'Rachid Temal', 'attested_on': '2018-03-16', 'from': None, 'until': None,
                   'sources': ['fr_ps_site_vote_20180316'], 'claim_ids': ['fr_ps_temal_national_coordinator_20180316'],
                   'note': 'Observed on 16 March 2018.', 'uncertainty': 'No start or end established.'}
        delegated = dict(copy.deepcopy(interim), name='François Hollande', attested_on='1997-06-06', sources=['fr_ps_hebdo_023_19970606'],
                         claim_ids=['fr_ps_hollande_premier_secretaire_delegue_1997'])
        provisional = dict(copy.deepcopy(interim), name='Michel Rocard', attested_on='1993-04-06', sources=['fr_ps_vendredi_183_19930409'],
                           claim_ids=['fr_ps_rocard_presides_direction_provisoire_19930406'])
        invariant_cases = [
            ('election promoted to a start (Emmanuelli)', lambda p: holder(p, 4).update({'from': '1994-06-19', 'attested_on': None})),
            ('election promoted to a start (Cambadélis)', lambda p: holder(p, 10).update({'from': '2014-04-15', 'attested_on': None})),
            ('resignation used as an end (Fabius)', lambda p: holder(p, 2).update(until='1993-04-03')),
            ('no-confidence vote used as an end (Rocard)', lambda p: holder(p, 3).update(until='1994-06-19')),
            ("successor's election used as an end (Mauroy)", lambda p: holder(p, 1).update(until='1992-01-09')),
            ('farewell used as an end (Hollande)', lambda p: holder(p, 7).update(until='2008-11-19')),
            ('announced departure used as an end (Cambadélis)', lambda p: holder(p, 10).update(until='2017-06-17')),
            ("'Premier secrétaire élu' styling used as the observation (Faure)", lambda p: holder(p, 11).update(
                attested_on='2018-03-29', claim_ids=['fr_ps_faure_premier_secretaire_elu_20180329'],
                sources=['fr_ps_site_press_conference_2018'])),
            ('retrospective page used as a holder basis (Faure)', lambda p: holder(p, 12)['claim_ids'].append(
                'fr_ps_page_faure_reconduit_retrospective')),
            ('vote result used as a holder basis (Jospin)', lambda p: holder(p, 5)['claim_ids'].append('fr_ps_jospin_members_vote_result_1995')),
            ('observation re-dated to its election (Aubry)', lambda p: holder(p, 8).update(attested_on='2008-11-25')),
            ('interim coordinator added as a holder', lambda p: role(p)['holder_claims'].append(interim)),
            ('delegated office added as a holder', lambda p: role(p)['holder_claims'].insert(7, delegated)),
            ('provisional presidency added as a holder', lambda p: role(p)['holder_claims'].insert(3, provisional)),
            ('eleventh person added', lambda p: role(p)['holder_claims'].append(dict(copy.deepcopy(holder(p, 12)), name='Next Holder'))),
            ('holder removed', lambda p: role(p)['holder_claims'].pop(5)),
            ('holders reordered', lambda p: role(p)['holder_claims'].reverse()),
            ('accent removed from a printed name (Désir)', lambda p: holder(p, 9).update(name='Harlem Desir')),
            ('organization mapped to a game party', lambda p: org(p)['represented_party_ids'].append('France/fr_ps')),
            ('organization given a lifespan', lambda p: org(p)['lifecycle'].update({'from': '1990-01-01'})),
            ('second party role added', lambda p: org(p)['roles'].append(dict(copy.deepcopy(role(p)), id='fr_ps_other'))),
            ('role placed on another organization', lambda p: next(o for o in p['organizations'] if o['id'] == 'fr_cnccfp_78')[
                'roles'].append(copy.deepcopy(role(p)))),
            ('prime-minister claim feeding a party holder', lambda p: holder(p, 12)['claim_ids'].append(
                'fr_jorf_lecornu_signs_decree_as_pm_20260904')),
            ('party claim added to the prime ministership', lambda p: p['institutions'][1]['roles'][0]['claim_ids'].append(
                'fr_ps_jospin_receives_flnks_19951018')),
            ('party claim added to the presidency', lambda p: p['institutions'][0]['claim_ids'].append(
                'fr_ps_faure_signed_statement_20180709')),
            ('claim re-dated', lambda p: claim(p, 'fr_ps_desir_at_jarnac_20130108').update(attested_on='2013-01-12')),
            ('claim dropped from the role', lambda p: role(p)['claim_ids'].pop()),
        ]
        party_invariants(self.packet)
        for label, change in invariant_cases:
            with self.subTest(label=label), self.assertRaises((AssertionError, KeyError, IndexError, StopIteration, TypeError)):
                party_invariants(mutated(change))

    def test_report_and_handoff_close_no_parent_gate(self):
        self.assertIn('ready_for_review', self.report)
        table = self.section('Outcome')
        decisions = {f'{i:02}': 'Accepted in part' for i in range(1, 12)}
        decisions['10'] = 'Accepted'
        for number, decision in decisions.items():
            row, = [line for line in table.splitlines() if line.startswith(f'| FR-PS-{number} ')]
            self.assertIn(f'**{decision}:**', row)
        self.assertIn('\n### Date ledger', self.report)
        for heading in ('Sources added', 'Response identities and stability checks', 'Sources attempted', 'Leads not imported',
                        'Next work', 'Decisions for Codex', 'Integration notes', 'Checks'):
            self.assertIn(f'\n## {heading}', self.report)
        for marker in ('C01', 'C06', 'S23', 'WC1', 'CP1'):
            self.assertNotRegex(self.report, rf'\b{marker}\b[^.\n]*\bis (now )?complete\b')
        notes = self.section('Integration notes')
        for text in ('6f657179', '5ea4f8fc', 'research-index.json', 'import_cnccfp_census.py', 'organization_roles',
                     'supplements/france.json', 'test_france_prime_ministers_c01_38.py', 'test_import_cnccfp_census.py'):
            self.assertIn(text, notes)
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        for text in ('ready_for_review', 'france-ps-first-secretaries-1990-2026-47.md', 'claude/c01-fr-47', '6f657179', '5ea4f8fc',
                     'test_france_ps_first_secretaries_c01_47.py', 'supplements/france.json', 'import_cnccfp_census.py'):
            self.assertIn(text, handoff)
        index = research.build()
        country = next(p for p in index['countries'] if p['nation'] == 'France')
        self.assertFalse(country['country_census_complete'])
        self.assertEqual((country['institution_observations'], country['role_observations']), (2, 3))
        self.assertFalse(index['c01_complete'])


if __name__ == '__main__':
    unittest.main()
