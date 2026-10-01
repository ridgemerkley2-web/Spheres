"""CLAUDE-C01-40: the General Secretaries of the Communist Party of India (Marxist), 1990-2026, are one party role on the
existing CPI(M) recognition observation (row 4 of the Election Commission's national-party table of 23 March 2024), kept
apart from the prime-ministership, the presidency and the Congress, BJP and Janata Dal presidencies both ways. Party
Congress and Central Committee elections, 'newly elected' stylings on or for an election, later attestations, 'former',
'outgoing' and handover stylings, retrospective pages and the interim coordinator arrangement of 2024-2025 stay separate
claims; the only boundary is the death in office
of 12 September 2024, stated with the office in the Polit Bureau's own statement. No source states a day of assumption,
so every holder is a dated in-office observation."""
import copy
import hashlib
import json
import re
import unittest
from urllib.parse import urlsplit

import campaign_research as research
import test_india_bjp_presidents_c01_27 as c01_27
import test_india_inc_presidents_c01_20 as c01_20
import test_india_janata_dal_presidents_c01_33 as c01_33
import test_india_presidents_c01_15 as c01_15
import test_india_prime_ministers_c01_11 as c01_11

ORG = 'in_eci_20240323_np_04'
ROLE = 'in_cpm_general_secretary'
TITLE = 'General Secretary of the Communist Party of India (Marxist)'
REVIEW = [f'CPM-GS-{n:02d}' for n in range(1, 7)]
EMS, SUR, PK, SY, MAB = ('EMS Namboodiripad', 'Harkishan Singh Surjeet', 'Prakash Karat', 'Sitaram Yechury', 'M. A. Baby')
# The surname that every claim naming that person must carry in its text; at most ten people in the packet.
SURNAMES = {EMS: 'Namboodiripad', SUR: 'Surjeet', PK: 'Karat', SY: 'Yechury', MAB: 'Baby'}
HOLDER_NAMES = {SUR, PK, SY, MAB}
MONTHS = ['January', 'February', 'March', 'April', 'May', 'June', 'July', 'August', 'September', 'October', 'November',
          'December']

# Original response identity recorded in each extract: (bytes, sha256), in packet order. Every source is reproducible:
# raw Internet Archive captures (id_) made before the cutoff, or the party website's WordPress REST record of a post.
RESPONSES = {
    'in_cpim_site_ems_caption_page_2001':
        (757, '33441055f20b435128756fce7bcd8f438360d8e083428491766701a948e316b1'),
    'in_cpim_18th_congress_site_surjeet_biodata_2005':
        (10960, '7107de326e466360295b7425874d92f27bad9a7c8dd553c3b9e0320c3deedb19'),
    'in_cpim_site_party_committees_page_2001':
        (3997, '7a7dbfdfafd9901179162a9116721291c63bfa9ce3a724587d08817c0bb656f4'),
    'in_pd_20040307_surjeet_letter_to_cec':
        (6512, '0ef848e878f9734c4918262ca0845ae1aaf705c636fea513579190832ec8f3b1'),
    'in_pd_20050417_new_polit_bureau':
        (7999, 'c69549381c2a88e240f8a1b718ce020d66b0adcf02bc34d4c0af611e1ae4610d'),
    'in_pd_20050417_rally_concludes_18th_congress':
        (15375, '2b9011d7fa502b8f5b92fe13863e63733b1174509e73ddbeab022621704aae2d'),
    'in_pd_20050417_surjeet_break_the_impasse':
        (24529, 'e9ba6930441922a9d28ea7d5224736e8d7050628c09b1484cc58be64780bc7b7'),
    'in_cpim_gs_note_to_nic_20110910':
        (12336, 'dc2e86ccc93d9da313f9f1d44061d7d29857bd9424930b54c654fa9a785dfad2'),
    'in_pd_20120415_rally_concludes_20th_congress':
        (699954, 'e537657ed6c2ea4152fde11b3a1781304a46fe4da99c995e8fe05d315883df8f'),
    'in_pd_20150426_join_to_bring_forth_change':
        (54802, '737f58484923183c78ac0731e3f1f2c4356d3dc03965b855fbe7f8c25d319842'),
    'in_pd_20150426_central_committee_elected_21st_congress':
        (45816, 'd91fa63d8430d924fc47980b6972119d9e860fb92a6fd8f1ffb6e9870fbbe5f3'),
    'in_cpim_gs_letter_to_naidu_20150508':
        (5790, '8d48f4e16cae84c2e03fbb491e8dccb10be0e9cdd83125d984d76ad362a4c36e'),
    'in_cpim_22nd_congress_new_cc_elected_20180422':
        (6385, 'e7492f9e17d479c3d50e0b454c337ca8ad6f510096240cc7c73aaca3a38119f2'),
    'in_cpim_23rd_congress_new_cc_elected_20220410':
        (16071, 'd93cef3d84c756306df3d6099c871af89f73409bef51561ecdb9c43b721382e8'),
    'in_cpim_pb_homage_yechury_20240912':
        (6954, '510c06aba2d3d633866751cd718eae821e574ae5d1b67c3ef0fb564f9d25e80d'),
    'in_cpim_cc_karat_coordinator_interim_20240929':
        (2616, 'a6f5b9caff41ac41a38961d04600353103e863540bb617120db55479a3d0eed7'),
    'in_cpim_karat_coordinator_inaugural_speech_20250402':
        (17491, 'b7382cde1af9b8d34b084561a04e17640a59a23c12dffcfbe4dde699b0b090e0'),
    'in_cpim_24th_congress_new_cc_elected_20250406':
        (8686, '9626ab880c30d1387227a2d6ee0ee8e2d88a99beb4f1184797af74585508524e'),
    'in_cpim_gs_letter_to_pm_20250512':
        (4911, '2704ad765fe48047bc29ae441ef9b67ed54c86cf69b338f5c9b412abeb896f39'),
    'in_cpim_memo_census_2027_20260824':
        (9965, '2933d246f61b9fa58e1bf21772eed135893635672c1b3a9d1994b0408adad4bc'),
}
WAYBACK = {
    'in_cpim_site_ems_caption_page_2001': ('20010424181354', 'http://cpim.org/ems~2.htm'),
    'in_cpim_18th_congress_site_surjeet_biodata_2005': ('20050414085223', 'http://www.cpim.org/18cong/lead/bio-hks.htm'),
    'in_cpim_site_party_committees_page_2001': ('20010925192536', 'http://cpim.org/party_com.htm'),
    'in_pd_20040307_surjeet_letter_to_cec': ('20040928103149', 'http://www.cpim.org/pd/2004/0307/03072004_surjeet%20letter.htm'),
    'in_pd_20050417_new_polit_bureau': ('20060623204706', 'http://pd.cpim.org/2005/0417/04172004_polit%20bureau.htm'),
    'in_pd_20050417_rally_concludes_18th_congress': ('20050426231814', 'http://pd.cpim.org/2005/0417/04172004_massive%20rally.htm'),
    'in_pd_20050417_surjeet_break_the_impasse': ('20060623205104', 'http://pd.cpim.org/2005/0417/04172004_surjeet.htm'),
    'in_pd_20120415_rally_concludes_20th_congress': ('20120812040620', 'http://pd.cpim.org/2012/0415_pd/04152012_6.html'),
    'in_pd_20150426_join_to_bring_forth_change': ('20150429172939', 'http://peoplesdemocracy.in/2015/0426_pd/join-bring-forth-change'),
    'in_pd_20150426_central_committee_elected_21st_congress': ('20150429173030', 'http://peoplesdemocracy.in/2015/0426_pd/central-committee-elected-21st-congress'),
}
REST_POSTS = {
    'in_cpim_gs_note_to_nic_20110910': 1411,
    'in_cpim_gs_letter_to_naidu_20150508': 4346,
    'in_cpim_22nd_congress_new_cc_elected_20180422': 5567,
    'in_cpim_23rd_congress_new_cc_elected_20220410': 6757,
    'in_cpim_pb_homage_yechury_20240912': 11608,
    'in_cpim_cc_karat_coordinator_interim_20240929': 11626,
    'in_cpim_karat_coordinator_inaugural_speech_20250402': 11826,
    'in_cpim_24th_congress_new_cc_elected_20250406': 11931,
    'in_cpim_gs_letter_to_pm_20250512': 11982,
    'in_cpim_memo_census_2027_20260824': 12752,
}
# The two sources added by the checker fixes were first downloaded on 1 October 2026, every other on 30 September.
ACCESSED_20261001 = ('in_cpim_gs_note_to_nic_20110910', 'in_cpim_gs_letter_to_naidu_20150508')
# Every claim: (attested_on, event_kind, review observation, holder_name), in packet order.
EVENTS = {
    'in_cpim_site_ems_former_general_secretary_caption':
        (None, 'retrospective_former_styling', 'CPM-GS-01', EMS),
    'in_cpim_biodata_surjeet_elected_gs_1992_14th_congress':
        (None, 'retrospective_biography_statement', 'CPM-GS-02', SUR),
    'in_cpim_site_cc_elected_surjeet_gs_recalled_19981011':
        ('1998-10-11', 'election_recalled_dated', 'CPM-GS-02', SUR),
    'in_pd_surjeet_gs_letter_to_cec_20040302':
        ('2004-03-02', 'in_office_attestation', 'CPM-GS-02', SUR),
    'in_pd_karat_unanimously_elected_gs_18th_congress':
        (None, 'election_result', 'CPM-GS-03', PK),
    'in_pd_rally_surjeet_former_gs_20050411':
        ('2005-04-11', 'former_styling', 'CPM-GS-02', SUR),
    'in_pd_rally_newly_elected_gs_karat_20050411':
        ('2005-04-11', 'newly_elected_styling', 'CPM-GS-03', PK),
    'in_pd_surjeet_general_secretary_since_1992_recalled':
        (None, 'retrospective_biography_statement', 'CPM-GS-02', SUR),
    'in_pd_surjeet_says_karat_took_up_mantle_2005':
        (None, 'handover_statement', 'CPM-GS-02', SUR),
    'in_cpim_karat_gs_note_to_nic_20110910':
        ('2011-09-10', 'in_office_attestation', 'CPM-GS-03', PK),
    'in_pd_rally_newly_elected_gs_karat_took_salute_20120409':
        ('2012-04-09', 'newly_elected_styling', 'CPM-GS-03', PK),
    'in_pd_karat_gs_inaugurated_21st_congress_20150414':
        ('2015-04-14', 'in_office_continuation_attestation', 'CPM-GS-03', PK),
    'in_pd_21st_congress_elected_general_secretary_20150419':
        ('2015-04-19', 'election_result_unnamed', 'CPM-GS-04', None),
    'in_pd_rally_karat_outgoing_gs_20150419':
        ('2015-04-19', 'outgoing_styling', 'CPM-GS-03', PK),
    'in_pd_rally_newly_elected_gs_yechury_20150419':
        ('2015-04-19', 'newly_elected_styling', 'CPM-GS-04', SY),
    'in_pd_pb_list_yechury_general_secretary_21st_congress':
        (None, 'election_result', 'CPM-GS-04', SY),
    'in_cpim_yechury_gs_letter_to_naidu_20150506':
        ('2015-05-06', 'in_office_attestation', 'CPM-GS-04', SY),
    'in_cpim_cc_elected_yechury_gs_22nd_congress_20180422':
        ('2018-04-22', 'election_result', 'CPM-GS-04', SY),
    'in_cpim_cc_reelected_yechury_gs_23rd_congress_20220410':
        ('2022-04-10', 'election_result', 'CPM-GS-04', SY),
    'in_cpim_pb_yechury_general_secretary_died_20240912':
        ('2024-09-12', 'death_in_office_stated', 'CPM-GS-04', SY),
    'in_cpim_pb_yechury_elected_gs_21st_congress_recalled':
        (None, 'retrospective_biography_statement', 'CPM-GS-04', SY),
    'in_cpim_cc_karat_coordinator_interim_arrangement_20240929':
        ('2024-09-29', 'interim_arrangement_decision', 'CPM-GS-05', PK),
    'in_cpim_karat_styled_coordinator_inaugural_speech_20250402':
        ('2025-04-02', 'interim_service_attestation', 'CPM-GS-05', PK),
    'in_cpim_cc_elected_baby_gs_24th_congress_20250406':
        ('2025-04-06', 'election_result', 'CPM-GS-06', MAB),
    'in_cpim_baby_gs_letter_to_pm_20250512':
        ('2025-05-12', 'in_office_attestation', 'CPM-GS-06', MAB),
    'in_cpim_baby_gs_memo_census_20260824':
        ('2026-08-24', 'in_office_continuation_attestation', 'CPM-GS-06', MAB),
}
NEW_SOURCES = list(RESPONSES)

HOLDER_KINDS = {'in_office_attestation'}
FROM_KINDS = {'assumption_statement'}
UNTIL_KINDS = {'death_in_office_stated'}
ELECTION_KINDS = {'election_result', 'election_result_unnamed', 'election_recalled_dated'}
CONTINUATION_KINDS = {'in_office_continuation_attestation'}
STYLING_KINDS = {'former_styling', 'outgoing_styling', 'handover_statement'}
# A 'newly elected' styling on or for the election is an election claim, never a holder date or a start (ruling (a) of
# this packet's review, following CLAUDE-C01-27's 'Newly Elected President').
NEWLY_ELECTED_KINDS = {'newly_elected_styling'}
RETROSPECTIVE_KINDS = {'retrospective_biography_statement', 'retrospective_former_styling'}
INTERIM_KINDS = {'interim_arrangement_decision', 'interim_service_attestation'}
NEVER_KINDS = (ELECTION_KINDS | NEWLY_ELECTED_KINDS | CONTINUATION_KINDS | STYLING_KINDS | RETROSPECTIVE_KINDS
               | INTERIM_KINDS)

# Exact holder observations of in_cpm_general_secretary: (name, attested_on, from, until), in chronological order.
HOLDERS = [
    (SUR, '2004-03-02', None, None),
    (PK, '2011-09-10', None, None),
    (SY, '2015-05-06', None, '2024-09-12'),
    (MAB, '2025-05-12', None, None),
]
HOLDER_CLAIMS = [
    ['in_pd_surjeet_gs_letter_to_cec_20040302'],
    ['in_cpim_karat_gs_note_to_nic_20110910'],
    ['in_cpim_yechury_gs_letter_to_naidu_20150506', 'in_cpim_pb_yechury_general_secretary_died_20240912'],
    ['in_cpim_baby_gs_letter_to_pm_20250512'],
]
STARTS = []
ENDS = [(SY, '2024-09-12')]
HOLDER_OBSERVATIONS = tuple(cid for cid, e in EVENTS.items() if e[1] in HOLDER_KINDS)
NEVER_HOLDER = tuple(cid for cid, e in EVENTS.items() if e[1] in NEVER_KINDS)
# Each Party Congress or Central Committee election is its own claim; these are the days the sources give, in order.
ELECTION_DAYS = ['1998-10-11', '2015-04-19', '2018-04-22', '2022-04-10', '2025-04-06']
# The interim coordinator of the Polit Bureau and the Central Committee is never a holder of this office.
INTERIM = {PK: ('2024-09-29', '2025-04-02')}
# The 'newly elected' stylings at the rallies closing the 18th, 20th and 21st Congresses: (person, day), in order.
NEWLY_ELECTED = [(PK, '2005-04-11'), (PK, '2012-04-09'), (SY, '2015-04-19')]
NOT_HOLDERS = {EMS}
HOLDER_DATES = {d for h in HOLDERS for d in h[1:] if d}
# Days carried only by claims that never feed a holder; no holder may be dated by one of them.
NEVER_HOLDER_DATE = sorted({e[0] for cid, e in EVENTS.items() if cid not in HOLDER_OBSERVATIONS and e[0]} - HOLDER_DATES)
UNDATED = tuple(cid for cid, e in EVENTS.items() if e[0] is None)
PER_REQUEST = re.compile(r'[?&](cb|_|s|q|token|sessionid|search|nocache|_fields|page)=|/search|cdn-cgi|X-Amz-|Signature=',
                         re.I)
REPORT = research.RESEARCH / 'india-cpim-general-secretaries-1990-2026-40.md'
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-40.md'


def cpm_org(packet):
    org, = [o for o in packet['organizations'] if o['id'] == ORG]
    return org


def cpm_role(packet):
    return cpm_org(packet)['roles'][0]


def other_role(packet, org_id):
    org, = [o for o in packet['organizations'] if o['id'] == org_id]
    return org['roles'][0]


def load_rows(packet):
    rows = {}
    for source in packet['sources']:
        if source['id'] in RESPONSES:
            extract = json.loads((research.ROOT / source['snapshot']['path']).read_text(encoding='utf-8'))
            rows.update({row['claim_id']: row for row in extract['rows']})
    return rows


def cpm_rules(packet, rows):
    """Rule-based checks that hold without the pinned holder list; raise AssertionError, KeyError or IndexError."""
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    claim_source = {c['id']: s['id'] for s in packet['sources'] for c in s['claims']}
    orgs = [o for o in packet['organizations'] if o['id'] == ORG]
    assert len(orgs) == 1
    org = orgs[0]
    # The observation keeps its recognition identity: no lifespan, no successor and no game mapping.
    assert (org['name'], org['kind'], org['jurisdiction']) == ('Communist Party of India (Marxist)',
                                                               'political_party_national_recognition_observation', 'India')
    assert org['recognition'] == {'level': 'national', 'attested_on': '2024-03-23', 'from': None, 'until': None,
                                  'source_qualifications': [], 'consolidated_current_status': None}
    assert (org['lifecycle']['status'], org['lifecycle']['from'], org['lifecycle']['until']) == ('unresearched', None, None)
    assert org['represented_party_ids'] == [] and org['reconciled_organization_id'] is None
    assert org['coverage']['status'] == 'reporting_identity_only'
    assert org['sources'][0] == 'in_eci_national_parties_20240323' and org['claim_ids'][0] == 'in_eci_20240323_np_row_04'
    assert [r['id'] for r in org['roles']] == [ROLE], 'exactly one role on the CPI(M) observation'
    role = org['roles'][0]
    assert (role['title'], role['kind']) == (TITLE, 'party_leader')
    leaders = [(e['id'], r['id']) for e in packet['organizations'] + packet['institutions'] for r in e['roles']
               if r['kind'] == 'party_leader' or r['id'] == ROLE]
    assert leaders == [(c01_27.ORG, c01_27.ROLE), (ORG, ROLE), (c01_20.ORG, c01_20.ROLE), (c01_33.ORG, c01_33.ROLE)], \
        'the four party-leader roles only, and no copy of this one'
    assert set(role['claim_ids']) <= set(org['claim_ids']) and set(role['sources']) <= set(org['sources'])
    assert org['claim_ids'][1:] == role['claim_ids'] and org['sources'][1:] == role['sources']
    # Every row of this packet belongs to the role on this observation.
    for cid in role['claim_ids']:
        assert (rows[cid]['observation_id'], rows[cid]['role_id'], rows[cid]['role_title']) == (ORG, ROLE, TITLE), cid
        assert rows[cid]['review_observation'] in REVIEW, cid
        name = rows[cid]['holder_name']
        assert name is None or SURNAMES[name] in claims[cid]['text'], (cid, 'the named person appears in the text')
    # Party office and state office never feed each other, and the other party roles share nothing with this one.
    cpm_claims = set(role['claim_ids']) | {c for h in role['holder_claims'] for c in h['claim_ids']}
    cpm_sources = set(role['sources']) | {s for h in role['holder_claims'] for s in h['sources']}
    for inst in packet['institutions']:
        i_claims = set(inst['claim_ids']) | {c for r in inst['roles'] for c in r['claim_ids']} | {
            c for r in inst['roles'] for h in r['holder_claims'] for c in h['claim_ids']}
        i_sources = set(inst['sources']) | {s for r in inst['roles'] for s in r['sources']} | {
            s for r in inst['roles'] for h in r['holder_claims'] for s in h['sources']}
        assert not cpm_claims & i_claims and not cpm_sources & i_sources, inst['id']
        for r in inst['roles']:
            assert not {h['name'] for h in r['holder_claims']} & HOLDER_NAMES, (inst['id'], 'cross-institution holder')
    for entry in packet['organizations']:
        if entry is not org:
            assert not set(entry['claim_ids']) & cpm_claims and not set(entry['sources']) & cpm_sources, entry['id']
            for r in entry['roles']:
                assert not set(r['claim_ids']) & cpm_claims and not set(r['sources']) & cpm_sources, r['id']
                assert not {h['name'] for h in r['holder_claims']} & HOLDER_NAMES, (r['id'], 'cross-party holder')
    # A 'newly elected' styling on or for the election never dates the holder it names.
    styled = {(rows[c]['holder_name'], claims[c].get('attested_on')) for c in role['claim_ids']
              if rows[c]['event_kind'] in NEWLY_ELECTED_KINDS}
    previous = ''
    for holder in role['holder_claims']:
        name = holder['name']
        assert isinstance(holder, dict) and name in HOLDER_NAMES, name
        assert name not in NOT_HOLDERS, (name, 'no in-office attestation in the period')
        dated = [d for d in (holder['attested_on'], holder['from']) if d]
        assert len(dated) == 1, (name, 'a holder is dated by exactly one of attested_on and from')
        assert dated[0] >= previous, (name, 'holders stay in chronological order')
        assert '1990-01-01' <= dated[0] <= research.CUTOFF, (name, 'inside the period')
        previous = dated[0]
        assert not {holder['attested_on'], holder['from'], holder['until']} & set(NEVER_HOLDER_DATE), name
        assert not set(holder['claim_ids']) & set(NEVER_HOLDER), name
        assert (name, holder['attested_on']) not in styled, (name, "a 'newly elected' styling is not an observation")
        if name in INTERIM:
            assert dated[0] not in INTERIM[name] and holder['until'] not in INTERIM[name], \
                (name, 'the interim coordinatorship is never a holder')
        expected, kinds = [], {}
        for cid in holder['claim_ids']:
            row = rows[cid]
            assert cid in role['claim_ids'], cid
            assert (row['holder_name'], row['role_id'], row['role_title']) == (name, ROLE, TITLE), (name, cid)
            assert row['event_kind'] in HOLDER_KINDS | FROM_KINDS | UNTIL_KINDS, (name, cid)
            kinds.setdefault(row['event_kind'], set()).add(claims[cid].get('attested_on'))
            if claim_source[cid] not in expected:
                expected.append(claim_source[cid])
        assert holder['sources'] == expected, name
        from_days = set().union(*(kinds.get(k, set()) for k in FROM_KINDS))
        until_days = set().union(*(kinds.get(k, set()) for k in UNTIL_KINDS))
        attest_days = set().union(*(kinds.get(k, set()) for k in HOLDER_KINDS))
        # A start only where a source states the day the office was assumed; an end only where one states the day it ended.
        assert ({holder['from']} == from_days) if holder['from'] else not from_days, (name, 'start')
        assert ({holder['until']} == until_days) if holder['until'] else not until_days, (name, 'end')
        # Each observation cites only in-office attestations of its own day.
        assert attest_days == ({holder['attested_on']} if holder['attested_on'] else set()), (name, 'observation')
        if holder['until']:
            assert holder['until'] <= research.CUTOFF and dated[0] <= holder['until']
    # Claims that never feed a holder stay on the role with their own kinds; undated claims carry no structured date.
    for cid in NEVER_HOLDER:
        assert cid in role['claim_ids'], cid
        assert rows[cid]['event_kind'] in NEVER_KINDS, cid
    for cid in UNDATED:
        assert not {'attested_on', 'period', 'attested_period'} & set(claims[cid]), cid
    for cid in role['claim_ids']:
        assert rows[cid]['event_kind'] in NEVER_KINDS | HOLDER_KINDS | UNTIL_KINDS, cid
        assert (claims[cid].get('attested_on') or '') <= research.CUTOFF, cid
    # The interim coordinator arrangement is claims only and never feeds a holder.
    interim = [cid for cid in role['claim_ids'] if rows[cid]['event_kind'] in INTERIM_KINDS]
    assert interim and not set(interim) & {c for h in role['holder_claims'] for c in h['claim_ids']}


def cpm_invariants(packet, rows):
    """The rules plus the exact pinned holder list this packet intends."""
    cpm_rules(packet, rows)
    role = cpm_role(packet)
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    got = [(h['name'], h['attested_on'], h['from'], h['until']) for h in role['holder_claims']]
    assert got == HOLDERS, got
    assert [h['claim_ids'] for h in role['holder_claims']] == HOLDER_CLAIMS
    assert [(h['name'], h['from']) for h in role['holder_claims'] if h['from']] == STARTS
    assert [(h['name'], h['until']) for h in role['holder_claims'] if h['until']] == ENDS
    assert role['claim_ids'] == list(EVENTS) and role['sources'] == NEW_SOURCES
    # Distinct dated events stay distinct and keep their own days, kinds and people.
    for cid, (day, kind, review, name) in EVENTS.items():
        assert claims[cid].get('attested_on') == day, cid
        assert (rows[cid]['attested_on'], rows[cid]['event_kind'], rows[cid]['review_observation'],
                rows[cid]['holder_name']) == (day, kind, review, name), cid
    # The earlier prime-minister, president, Congress-president, BJP-president and Janata Dal holders are unchanged.
    pm, presidency = packet['institutions']
    assert [(h['name'], h['attested_on'], h['from'], h['until']) for h in pm['roles'][0]['holder_claims']] == c01_11.HOLDERS
    assert [(h['name'], h['attested_on'], h['from'], h['until'])
            for h in presidency['roles'][0]['holder_claims']] == c01_15.HOLDERS
    for mod in (c01_20, c01_27, c01_33):
        other = other_role(packet, mod.ORG)
        assert [(h['name'], h['attested_on'], h['from'], h['until']) for h in other['holder_claims']] == mod.HOLDERS
    assert other_role(packet, c01_20.ORG)['claim_ids'] == list(c01_20.EVENTS)
    assert other_role(packet, c01_27.ORG)['claim_ids'] == list(c01_27.EVENTS)


class IndiaCpimGeneralSecretariesTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.raw = (research.ROOT / research.RESEARCH / 'india.json').read_text(encoding='utf-8')
        cls.packet = json.loads(cls.raw)
        cls.sources = {s['id']: s for s in cls.packet['sources']}
        cls.claims = {c['id']: c for s in cls.packet['sources'] for c in s['claims']}
        cls.org = cpm_org(cls.packet)
        cls.role = cls.org['roles'][0]
        cls.extracts = {sid: json.loads((research.ROOT / cls.sources[sid]['snapshot']['path']).read_text(encoding='utf-8'))
                        for sid in NEW_SOURCES}
        cls.rows = load_rows(cls.packet)
        cls.new_claims = [c['id'] for sid in NEW_SOURCES for c in cls.sources[sid]['claims']]
        cls.report = (research.ROOT / REPORT).read_text(encoding='utf-8')

    def validate(self, packet=None):
        return research.validate(packet or self.packet, research.ROOT, {'India'}, {'India': set()})

    def section(self, heading):
        parts = self.report.split(f'\n## {heading}', 1)
        self.assertEqual(len(parts), 2, heading)
        return parts[1].split('\n## ', 1)[0]

    def test_new_records_are_bounded_and_every_claim_is_classified(self):
        ids = self.validate()
        self.assertEqual((len(NEW_SOURCES), len(self.new_claims)), (20, 26))
        pm, presidency = self.packet['institutions']
        inc, bjp = other_role(self.packet, c01_20.ORG), other_role(self.packet, c01_27.ORG)
        jd, = [o for o in self.packet['organizations'] if o['id'] == c01_33.ORG]
        # This packet's sources are appended after the CLAUDE-C01-33 sources.
        self.assertEqual([s['id'] for s in self.packet['sources']],
                         list(c01_27.ORIGINAL_SOURCES) + pm['sources'] + presidency['sources'] + inc['sources']
                         + bjp['sources'] + jd['sources'] + NEW_SOURCES)
        self.assertEqual((pm['sources'], presidency['sources'], inc['sources'], bjp['sources'], jd['sources']),
                         (c01_11.NEW_SOURCES, c01_15.NEW_SOURCES, c01_20.NEW_SOURCES, c01_27.NEW_SOURCES,
                          c01_33.NEW_SOURCES))
        self.assertEqual((len(ids['entries']), len(ids['sources']), len(ids['claims']), len(ids['roles'])),
                         (85, 317, 625, 6))
        self.assertEqual(len(self.packet['organizations']), 83)
        self.assertEqual(self.org['sources'], ['in_eci_national_parties_20240323'] + NEW_SOURCES)
        self.assertEqual(self.org['claim_ids'], ['in_eci_20240323_np_row_04'] + list(EVENTS))
        # Every new claim is a holder claim or a claim that never feeds a holder; never both.
        holder_claims = [cid for ids_ in HOLDER_CLAIMS for cid in ids_]
        self.assertEqual(len(set(holder_claims)), len(holder_claims))
        self.assertFalse(set(holder_claims) & set(NEVER_HOLDER))
        self.assertEqual(set(holder_claims) | set(NEVER_HOLDER), set(self.new_claims))
        self.assertEqual(sorted(set(holder_claims) - {c for c, e in EVENTS.items() if e[1] in UNTIL_KINDS}),
                         sorted(HOLDER_OBSERVATIONS))
        self.assertEqual((len(holder_claims), len(NEVER_HOLDER), len(HOLDERS)), (5, 21, 4))
        # At most ten people, each named as the sources print them; one claim names nobody.
        people = {e[3] for e in EVENTS.values()} - {None}
        self.assertEqual(people, set(SURNAMES))
        self.assertLessEqual(len(people), 10)
        self.assertEqual([cid for cid, e in EVENTS.items() if e[3] is None],
                         ['in_pd_21st_congress_elected_general_secretary_20150419'])
        # Six observations, each reported and each carrying rows.
        observations = re.findall(r'^### (CPM-GS-\d\d)\b', self.report, re.M)
        self.assertEqual(observations, REVIEW)
        self.assertEqual({e[2] for e in EVENTS.values()}, set(REVIEW))

    def test_holders_are_exactly_as_intended(self):
        cpm_invariants(self.packet, self.rows)
        self.assertEqual(self.role['scope_note'].count('CLAUDE-C01-40'), 1)
        for holder in self.role['holder_claims']:
            self.assertTrue(holder['note'].startswith('Observed on '), holder['name'])
            self.assertTrue(holder['uncertainty'].startswith('No start'), holder['name'])

    def test_starts_and_ends_only_where_a_source_states_one(self):
        # No source states the day an office was assumed; the one end is a death in office stated with the office.
        self.assertFalse([h for h in self.role['holder_claims'] if h['from']])
        death, = [cid for cid, e in EVENTS.items() if e[1] in UNTIL_KINDS]
        self.assertEqual(death, 'in_cpim_pb_yechury_general_secretary_died_20240912')
        self.assertIn('General Secretary of the Party on September 12, 2024', self.claims[death]['text'])
        self.assertEqual(self.sources['in_cpim_pb_homage_yechury_20240912']['publisher'],
                         'Communist Party of India (Marxist) (cpim.org, the party website)')
        # 'Former', 'outgoing' and handover stylings, elections and the interim arrangement are never boundaries.
        for cid, (day, kind, _review, name) in EVENTS.items():
            if kind in STYLING_KINDS | ELECTION_KINDS | NEWLY_ELECTED_KINDS | INTERIM_KINDS:
                for holder in self.role['holder_claims']:
                    self.assertNotIn(cid, holder['claim_ids'])
                    if day and holder['name'] == name:
                        self.assertNotIn(day, (holder['from'], holder['until']), cid)
        # Each Party Congress or Central Committee election recorded with a day is its own claim with its own day.
        dated = [e[0] for e in EVENTS.values() if e[1] in ELECTION_KINDS and e[0]]
        self.assertEqual(dated, ELECTION_DAYS)
        self.assertEqual(len([e for e in EVENTS.values() if e[1] in ELECTION_KINDS]), 7)
        # A 'newly elected' styling on or for the election is an election claim, never a holder date or a start:
        # Karat's of 11 April 2005 and 9 April 2012 and Yechury's of 19 April 2015 (ruling (a), after CLAUDE-C01-27).
        styled = [(e[3], e[0]) for e in EVENTS.values() if e[1] in NEWLY_ELECTED_KINDS]
        self.assertEqual(styled, NEWLY_ELECTED)
        for name, day in styled:
            holder = next(h for h in self.role['holder_claims'] if h['name'] == name)
            self.assertNotIn(day, (holder['attested_on'], holder['from'], holder['until']), name)
        for cid in [c for c, e in EVENTS.items() if e[1] in NEWLY_ELECTED_KINDS]:
            self.assertTrue(self.claims[cid]['uncertainty'].startswith('An election claim, never a holder date'), cid)
        # Karat's end is not inferred from Yechury's election or observation on 19 April 2015, nor Surjeet's from Karat's.
        karat = next(h for h in self.role['holder_claims'] if h['name'] == PK)
        surjeet = next(h for h in self.role['holder_claims'] if h['name'] == SUR)
        self.assertIsNone(karat['until'])
        self.assertIsNone(surjeet['until'])
        self.assertIn('outgoing', karat['uncertainty'])
        self.assertIn('former', surjeet['uncertainty'])
        # The interim coordinator is not styled General Secretary and never holds the office.
        for cid in ('in_cpim_cc_karat_coordinator_interim_arrangement_20240929',
                    'in_cpim_karat_styled_coordinator_inaugural_speech_20250402'):
            self.assertIn('claims only', self.claims[cid]['uncertainty'])
            self.assertEqual(EVENTS[cid][3], PK)
        self.assertIn('claims only, never a holder', self.role['scope_note'])
        # EMS Namboodiripad has no in-office attestation in the period.
        self.assertNotIn(EMS, {h['name'] for h in self.role['holder_claims']})
        self.assertEqual([cid for cid, e in EVENTS.items() if e[3] == EMS],
                         ['in_cpim_site_ems_former_general_secretary_caption'])

    def test_party_and_state_offices_stay_separate(self):
        cpm_rules(self.packet, self.rows)
        pm, presidency = self.packet['institutions']
        for inst in (pm, presidency):
            self.assertFalse(set(inst['sources']) & set(NEW_SOURCES), inst['id'])
            self.assertFalse(set(inst['claim_ids']) & set(self.new_claims), inst['id'])
        for entry in self.packet['organizations']:
            if entry['id'] != ORG:
                self.assertFalse(set(entry['sources']) & set(NEW_SOURCES), entry['id'])
                self.assertFalse(set(entry['claim_ids']) & set(self.new_claims), entry['id'])
        self.assertIn('Party office and state office stay separate', self.role['scope_note'])
        rows = json.dumps([self.extracts[s]['rows'] for s in NEW_SOURCES])
        for other in ('in_prime_minister', 'in_presidency', 'in_inc_president', 'in_bjp_president', 'in_jd_president'):
            self.assertNotIn(other, rows)

    def test_extracts_match_packet_claims_and_record_original_responses(self):
        for sid in NEW_SOURCES:
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual(extract['format'], 'spheres-c01-derived-factual-table/v1')
            self.assertEqual((extract['source_id'], extract['source_url']), (sid, source['url']))
            for key in ('scope_note', 'access_method', 'published_date', 'rights_note', 'accessed_date'):
                self.assertEqual(extract[key], source[key], (sid, key))
            self.assertEqual(source['accessed_date'], '2026-10-01' if sid in ACCESSED_20261001 else '2026-09-30', sid)
            self.assertLessEqual(source['published_date'] or '', research.CUTOFF)
            self.assertIs(extract['source_response_checked_in'], False)
            self.assertEqual((extract['source_response_bytes'], extract['source_response_sha256']), RESPONSES[sid])
            for phrase in ('not checked into this repository', 'derived factual extract', 'same byte count and SHA-256',
                           'at least 30 minutes apart', 'no Accept-Encoding request header', 'default User-Agent',
                           'no Content-Encoding'):
                self.assertIn(phrase, extract['provenance_note'], sid)
            self.assertIn('No open license, portrait permission or likeness approval', extract['rights_note'])
            self.assertIn('CLAUDE-C01-40 only', extract['bounded_scope'])
            self.assertEqual(extract['visual_review']['pdf_pages_one_based'], [])
            self.assertTrue(source['source_type'].startswith('primary_') and source['scope_note'] and source['publisher'])
            self.assertTrue(source['publisher'].startswith('Communist Party of India (Marxist) ('), sid)
            snapshot = source['snapshot']
            self.assertTrue(snapshot['path'].startswith('docs/campaign-certification/C01/research/sources/india-'))
            self.assertTrue(snapshot['path'].endswith('-facts.json'))
            data = (research.ROOT / snapshot['path']).read_bytes()
            self.assertEqual((len(data), hashlib.sha256(data).hexdigest()), (snapshot['bytes'], snapshot['sha256']))
            self.assertNotEqual(snapshot['sha256'], extract['source_response_sha256'])
            self.assertTrue(data.endswith(b'}\n') and b'\r' not in data)
            self.assertEqual(data.decode('utf-8'), json.dumps(extract, indent=2, ensure_ascii=False) + '\n')
            # Rows repeat the packet claims exactly, in order, keyed by claim_id; no row has a bare 'name' key.
            self.assertEqual([r['claim_id'] for r in extract['rows']], [c['id'] for c in source['claims']])
            for row, claim in zip(extract['rows'], source['claims']):
                self.assertEqual((row['text'], row['locator'], row['attested_on']),
                                 (claim['text'], claim['locator'], claim.get('attested_on')))
                self.assertNotIn('name', row)
                self.assertEqual(list(row), ['claim_id', 'observation_id', 'review_observation', 'role_id', 'holder_name',
                                             'role_title', 'event_kind', 'attested_on', 'text', 'locator'])
            self.assertEqual('original_url' in source, sid in WAYBACK, sid)
            self.assertEqual('archive_capture_utc' in extract, sid in WAYBACK, sid)
        self.assertEqual(len({self.sources[s]['snapshot']['path'] for s in NEW_SOURCES}), len(NEW_SOURCES))
        self.assertEqual({cid: (row['attested_on'], row['event_kind'], row['review_observation'], row['holder_name'])
                          for cid, row in self.rows.items()}, EVENTS)
        self.assertEqual(set(WAYBACK) | set(REST_POSTS), set(NEW_SOURCES))
        self.assertFalse(set(WAYBACK) & set(REST_POSTS))
        self.assertEqual((len(WAYBACK), len(REST_POSTS)), (10, 10))
        for sid, (stamp, original) in WAYBACK.items():
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual((source['original_url'], extract['original_url']), (original, original))
            self.assertLess(stamp, '20260907')
            self.assertEqual(extract['archive_capture_utc'].replace('-', '').replace(':', '').replace('T', '').rstrip('Z'),
                             stamp)
            self.assertIn('Raw Internet Archive capture (id_ form)', extract['provenance_note'])
            captured = f'Internet Archive capture of {int(stamp[6:8])} {MONTHS[int(stamp[4:6]) - 1]} {stamp[:4]}'
            self.assertTrue(source['title'].endswith(captured), sid)
        for sid, post in REST_POSTS.items():
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertIn(f'WordPress REST record of post {post}', extract['provenance_note'])
            self.assertIn('per-request cache timestamp', extract['provenance_note'])
            self.assertIn(f'cpim.org post {post}', source['title'])
            self.assertTrue(source['published_date'], sid)

    def test_response_identities_are_reproducible_urls(self):
        for sid in NEW_SOURCES:
            url = self.sources[sid]['url']
            self.assertIsNone(PER_REQUEST.search(url), url)
            parts = urlsplit(url)
            self.assertEqual(parts.scheme, 'https', url)
            if sid in WAYBACK:
                stamp, original = WAYBACK[sid]
                prefix = f'https://web.archive.org/web/{stamp}id_/'
                self.assertTrue(url.startswith(prefix), sid)
                # The capture is addressed as the archive records it (some with the explicit port 80).
                self.assertEqual(url[len(prefix):].replace(':80/', '/', 1), original, sid)
            else:
                self.assertEqual(url, f'https://cpim.org/wp-json/wp/v2/posts/{REST_POSTS[sid]}', sid)
                self.assertEqual(parts.query, '', sid)
        self.assertEqual({urlsplit(self.sources[s]['url']).hostname for s in NEW_SOURCES}, {'web.archive.org', 'cpim.org'})

    def test_secondary_and_unimported_leads_stay_out_of_the_packet(self):
        lowered = json.dumps([self.sources[s] for s in NEW_SOURCES], ensure_ascii=False).lower()
        for marker in ('wikipedia', 'britannica', 'thehindu', 'indianexpress', 'ndtv', 'hindustantimes', 'timesofindia',
                       'frontline', 'deccanherald', 'business-standard'):
            self.assertNotIn(marker, lowered, marker)
        leads, added = self.section('Leads not imported'), self.section('Sources added')
        for marker in ('posts/11973', '18cong/speeches/hks_opening.htm', 'E-0298-1996-0190-10568', '04132008_1.htm',
                       'posts/597'):
            self.assertIn(marker, leads, marker)
            self.assertNotIn(marker, added, marker)

    def test_packet_formatting_is_preserved(self):
        data = (research.ROOT / research.RESEARCH / 'india.json').read_bytes()
        self.assertNotIn(b'\r', data)
        self.assertEqual(data.decode('utf-8'), json.dumps(self.packet, indent=2, ensure_ascii=False) + '\n')

    def test_mutations_are_rejected(self):
        def holder(p, name):
            return next(h for h in cpm_role(p)['holder_claims'] if h['name'] == name)

        def cite(p, name, cid, sid):
            h = holder(p, name)
            h['claim_ids'].append(cid)
            h['sources'].append(sid)

        def fake(name, day, sid, cid):
            return {'name': name, 'attested_on': day, 'from': None, 'until': None, 'sources': [sid], 'claim_ids': [cid],
                    'note': '', 'uncertainty': ''}

        mutations = [
            ('election used as a start (Baby, 6 April 2025)',
             lambda p: (holder(p, MAB).update({'from': '2025-04-06'}),
                        cite(p, MAB, 'in_cpim_cc_elected_baby_gs_24th_congress_20250406',
                             'in_cpim_24th_congress_new_cc_elected_20250406'))),
            ('the stated end removed (Yechury)', lambda p: holder(p, SY).update({'until': None})),
            ('death claim dropped while the end stays',
             lambda p: (holder(p, SY)['claim_ids'].pop(), holder(p, SY)['sources'].pop())),
            ("'former' styling used as Surjeet's end", lambda p: holder(p, SUR).update({'until': '2005-04-11'})),
            ("'outgoing' styling used as Karat's end", lambda p: holder(p, PK).update({'until': '2015-04-19'})),
            ("a successor's start used as an end (Karat until Yechury)",
             lambda p: holder(p, PK).update({'until': '2015-04-18'})),
            ('interim coordinator made a holder',
             lambda p: cpm_role(p)['holder_claims'].append(
                 fake(PK, '2024-09-29', 'in_cpim_cc_karat_coordinator_interim_20240929',
                      'in_cpim_cc_karat_coordinator_interim_arrangement_20240929'))),
            ('EMS made a holder from a retrospective caption',
             lambda p: cpm_role(p)['holder_claims'].insert(
                 0, fake(EMS, '1990-01-01', 'in_cpim_site_ems_caption_page_2001',
                         'in_cpim_site_ems_former_general_secretary_caption'))),
            ('holder dated by a recalled election (Surjeet, 11 October 1998)',
             lambda p: (holder(p, SUR).update({'attested_on': '1998-10-11'}),
                        cite(p, SUR, 'in_cpim_site_cc_elected_surjeet_gs_recalled_19981011',
                             'in_cpim_site_party_committees_page_2001'))),
            ('holders out of order', lambda p: cpm_role(p)['holder_claims'].reverse()),
            ('a holder dated by both attested_on and from', lambda p: holder(p, PK).update({'from': '2005-04-11'})),
            ('a second role on the observation',
             lambda p: cpm_org(p)['roles'].append(dict(copy.deepcopy(cpm_role(p)), id='in_cpm_copy'))),
            ('a game mapping granted', lambda p: cpm_org(p).update({'represented_party_ids': ['India/cpim']})),
            ('lifecycle closed at the recognition date',
             lambda p: cpm_org(p)['lifecycle'].update({'status': 'dissolved', 'until': '2024-03-23'})),
            ('a claim moved to the prime-ministership',
             lambda p: p['institutions'][0]['claim_ids'].append('in_cpim_baby_gs_letter_to_pm_20250512')),
            ('a holder shared with the Congress presidency',
             lambda p: other_role(p, c01_20.ORG)['holder_claims'].append(
                 {'name': SY, 'attested_on': '2016-01-01', 'from': None, 'until': None, 'sources': [],
                  'claim_ids': []})),
            ('an election claim dropped from the role',
             lambda p: cpm_role(p)['claim_ids'].remove('in_cpim_cc_reelected_yechury_gs_23rd_congress_20220410')),
            ("'newly elected' styling used as Karat's observation (11 April 2005)",
             lambda p: holder(p, PK).update({'attested_on': '2005-04-11',
                                             'sources': ['in_pd_20050417_rally_concludes_18th_congress'],
                                             'claim_ids': ['in_pd_rally_newly_elected_gs_karat_20050411']})),
            ("'newly elected' styling used as Yechury's observation (19 April 2015)",
             lambda p: holder(p, SY).update({'attested_on': '2015-04-19'})),
        ]
        failures = []
        for label, fn in mutations:
            p = copy.deepcopy(self.packet)
            fn(p)
            try:
                cpm_invariants(p, self.rows)
            except (AssertionError, KeyError, IndexError, StopIteration):
                continue
            failures.append(label)
        self.assertEqual(failures, [])
        self.assertEqual(len(mutations), 19)
        # A claim whose day is changed, and an extract row that disagrees with the packet, are both caught.
        p = copy.deepcopy(self.packet)
        next(c for s in p['sources'] for c in s['claims']
             if c['id'] == 'in_pd_rally_newly_elected_gs_karat_20050411')['attested_on'] = '2005-04-12'
        with self.assertRaises(AssertionError):
            cpm_invariants(p, self.rows)
        rows = copy.deepcopy(self.rows)
        rows['in_pd_rally_karat_outgoing_gs_20150419']['event_kind'] = 'in_office_attestation'
        with self.assertRaises(AssertionError):
            cpm_invariants(self.packet, rows)
        rows = copy.deepcopy(self.rows)
        rows['in_pd_rally_newly_elected_gs_yechury_20150419']['event_kind'] = 'in_office_attestation'
        with self.assertRaises(AssertionError):
            cpm_invariants(self.packet, rows)

    def test_report_and_handoff_close_no_parent_gate(self):
        self.assertIn('State: **ready_for_review**', self.report)
        for heading in ('Outcome', 'Observations', 'Sources added', 'Response identities and stability checks',
                        'Leads not imported', 'Sources attempted', 'Suggested next work orders',
                        "Integration notes (outside this packet's file boundary)", 'Checks'):
            self.section(heading)
        self.assertIn('### Date ledger', self.report)
        self.assertIn('continue existing claims first', self.section("Integration notes (outside this packet's file "
                                                                     'boundary)'))
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        self.assertIn('State: **ready_for_review**', handoff)
        for text in ('C01 (incomplete)', 'research-index.json', 'continue existing claims first'):
            self.assertIn(text, handoff)
        self.assertTrue(self.org['coverage']['unresolved'][-1].startswith(
            'CPI(M) General Secretaries 1990-2026 (CLAUDE-C01-40)'))
        coverage = self.packet['coverage']
        self.assertEqual(sum('CLAUDE-C01-40' in u for u in coverage['unresolved']), 1)
        self.assertTrue(coverage['unresolved'][-1].startswith('CPI(M) General Secretaries 1990-2026 (CLAUDE-C01-40, '
                                                              'CPM-GS-01..06)'))
        self.assertEqual(len(coverage['unresolved']), 13)
        index = research.build()
        country = next(p for p in index['countries'] if p['nation'] == 'India')
        self.assertFalse(country['country_census_complete'])
        self.assertIsNone(country['unrepresented_organization_count'])
        self.assertEqual((country['organization_observations'], country['institution_observations'],
                          country['role_observations']), (83, 2, 6))
        self.assertEqual(country['mapping_pending'], 85)
        self.assertEqual({w['status'] for w in index['work_orders'] if w['nation'] == 'India'}, {'open'})
        self.assertFalse(index['c01_complete'])


if __name__ == '__main__':
    unittest.main()
