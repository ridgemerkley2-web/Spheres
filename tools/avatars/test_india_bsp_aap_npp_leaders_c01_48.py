"""CLAUDE-C01-48: the national leaders of the Aam Aadmi Party, the Bahujan Samaj Party and the National People's Party,
1990-2026, are one party role on each of the three existing ECI 2024 national-party recognition observations (rows 1, 2
and 6 of the Election Commission's table of 23 March 2024), kept apart from the prime-ministership, the presidency and the
Congress, BJP, Janata Dal and CPI(M) party roles both ways. Recalled elections and selections, retrospective timelines and
stylings, founding and death recollections, in-office attestations and later attestations stay separate claims; the only
boundary is the effective day stated in the AAP National Executive's certified minutes of 27 April 2016. Every other
holder is a dated in-office observation. Kanshi Ram and Purno Agitok Sangma, the founders, are claims only."""
import copy
import hashlib
import json
import re
import unittest
from urllib.parse import urlsplit

import campaign_research as research
import test_india_bjp_presidents_c01_27 as c01_27
import test_india_cpim_general_secretaries_c01_40 as c01_40
import test_india_inc_presidents_c01_20 as c01_20
import test_india_janata_dal_presidents_c01_33 as c01_33
import test_india_presidents_c01_15 as c01_15
import test_india_prime_ministers_c01_11 as c01_11

AAP, BSP, NPP = 'in_eci_20240323_np_01', 'in_eci_20240323_np_02', 'in_eci_20240323_np_06'
ORGS = [AAP, BSP, NPP]
# observation: (role id, role title, observation name, recognition row claim)
ROLES = {
    AAP: ('in_aap_national_convenor', 'National Convenor of the Aam Aadmi Party', 'Aam Aadmi Party',
          'in_eci_20240323_np_row_01'),
    BSP: ('in_bsp_national_president', 'National President of the Bahujan Samaj Party', 'Bahujan Samaj Party',
          'in_eci_20240323_np_row_02'),
    NPP: ('in_npp_national_president', "National President of the National People's Party",
          'National People’s Party', 'in_eci_20240323_np_row_06'),
}
REVIEW = {AAP: ['AAP-NC-01', 'AAP-NC-02'], BSP: ['BSP-NP-01', 'BSP-NP-02'], NPP: ['NPP-NP-01', 'NPP-NP-02']}
KEJ, KR, MAY, PAS, CKS = 'Arvind Kejriwal', 'Kanshi Ram', 'Mayawati', 'Purno Agitok Sangma', 'Conrad K. Sangma'
# The name that every claim naming that person must carry in its text (any case); at most ten people in the packet.
SURNAMES = {KEJ: 'Kejriwal', KR: 'Kanshi Ram', MAY: 'Mayawati', PAS: 'Purno Agitok Sangma', CKS: 'Conrad'}
HOLDER_NAMES = {AAP: {KEJ}, BSP: {MAY}, NPP: {CKS}}
NOT_HOLDERS = {KR, PAS}
MONTHS = ['January', 'February', 'March', 'April', 'May', 'June', 'July', 'August', 'September', 'October', 'November',
          'December']
# The seven party-leader roles in observation order: this packet's AAP and BSP roles come before the BJP role and its
# NPP role after the Congress role; the Janata Dal observation is appended last.
LEADERS = [(AAP, ROLES[AAP][0]), (BSP, ROLES[BSP][0]), (c01_27.ORG, c01_27.ROLE), (c01_40.ORG, c01_40.ROLE),
           (c01_20.ORG, c01_20.ROLE), (NPP, ROLES[NPP][0]), (c01_33.ORG, c01_33.ROLE)]

# Original response identity recorded in each extract: (bytes, sha256), in packet order. Every source is reproducible:
# raw Internet Archive captures (id_) made before the cutoff, or a WordPress REST record of a post on the Aam Aadmi
# Party's own websites.
RESPONSES = {
    'in_aap_nri_release_kejriwal_interact_20131007':
        (60785, '2ee8b421727527d8913feee110c67507f0d584e46e1696eabb2943475e226d42'),
    'in_aap_letter_to_eci_org_election_20150529':
        (1891611, '40498164efd553482af6d1ee3b56136ee1de7802f2de41bc1e965df81672a826'),
    'in_aap_letter_to_eci_new_national_executive_20160531':
        (623833, '708dfdf9cdba85e1e7ffee454932cd68f645a8ae49c63658060d7419772b6891'),
    'in_aap_release_kejriwal_demonetisation_20161112':
        (101927, '75e16c576352f22121fb589b44c705be9001adb107d9193ccef0f3f016b70223'),
    'in_eci_catalogue_order_to_kejriwal_20170120':
        (107889, '8ea555e65c4fcf964674f919b8221cf645a80779ec8a8931ab95f680af8cae68'),
    'in_aap_post_national_council_meeting_20221218':
        (76109, '2c7152d087d151318de1c768dd73074b327f0f0541cadacf97b7d8addbd8d759'),
    'in_aap_post_shokeen_minister_20241118':
        (6125, 'a624a138035190ce84a0bd057f74292393e60584a9bdcf060581c33ae92ac61e'),
    'in_aap_news_e20_townhall_goa_20260825':
        (11586, '2ec5726cef7e55b6088740e39c431fad9621ab688dd4e7e4291b084998255752'),
    'in_bsp_site_about_bsp_2009':
        (25135, 'ed631909958c1c7bd308f63e7826da26e7b03d4b891b8f528e3333e0d7527cc1'),
    'in_bsp_site_mayawati_profile_2009':
        (32452, '28f7c635183844976aa42772a3d17a9922b223a787eeedd6eac991b2024dba2d'),
    'in_bsp_press_release_cochin_20090703':
        (28694, 'f3a4c8848b2447e83f8b68ef4da93495bf3c0f15f0f38c9d883e6ad71901f7ae'),
    'in_bsp_letter_to_cec_office_bearers_20120716':
        (2639930, 'a21314ff14a3ad960290035852aa4d9d75e75c4db56a99dc922677e428d31ae3'),
    'in_eci_catalogue_order_to_mayawati_20190415':
        (97766, '49bde93e58b6ce4c96b5fcbed3287db1f05c68abd513ea5ddca07b92fbf0f86a'),
    'in_bsp_press_note_lucknow_20210901':
        (84146, 'cc7bb6a8f3cdb8c44fb9dbb0377705fb71ff0a588260832daf1c636d4f516c35'),
    'in_npp_site_leadership_page_2019':
        (85879, 'b8fb32fc200fc2f080f4789380399ad9bbd0d646ad3eac22bc001cf3fcf139ad'),
    'in_npp_press_release_resolution_20200711':
        (117420, '83af81106ef0b7b2cfbd4734a55a257fec3b644c97064a0abb064f41c7f4cde8'),
    'in_npp_site_conrad_sangma_page_2021':
        (124635, '49746d13db3a631891d73a4291b8880da8ff0ea4de5888b0039340bc5b8cdb69'),
    'in_npp_notification_manipur_candidates_20220204':
        (1071596, '41996d12d265283d5417e5dab78e56777d131873cbc1c002bee57f778c83096d'),
    'in_npp_notification_national_committee_20250417':
        (404254, '8422b24f623a37859b35b4dbe30cb29cfdd5f61179c617e6e274f7c49144eb67'),
}
WAYBACK = {
    'in_aap_nri_release_kejriwal_interact_20131007':
        ('20140819153447', 'http://nri.aamaadmiparty.org/news/arvind-kejriwal-national-convener-of-the-aam-aadmi-party-aap-to-interact-online-with-supporters'),
    'in_aap_letter_to_eci_org_election_20150529':
        ('20160804213413', 'http://eci.nic.in/eci_main/mis-Political_Parties/Organisational_Elections/Aam%20Aadmi%20Party%20year%202012-15.pdf'),
    'in_aap_letter_to_eci_new_national_executive_20160531':
        ('20170329121746', 'http://eci.nic.in/eci_main/mis-Political_Parties/Organisational_Elections/AAP_Org_2016-19.pdf'),
    'in_aap_release_kejriwal_demonetisation_20161112':
        ('20161121224935', 'http://www.aamaadmiparty.org/aap-national-convenor-delhi-cm-arvind-kejriwal-on-the-demonetization-scam'),
    'in_eci_catalogue_order_to_kejriwal_20170120':
        ('20210928010601', 'https://eci.gov.in/files/file/1260-commissions-order-to-shri-arvind-kejriwal-national-convener-aam-aadmi-party/'),
    'in_bsp_site_about_bsp_2009':
        ('20090328123255', 'http://bspindia.org/about-bsp.php'),
    'in_bsp_site_mayawati_profile_2009':
        ('20090331050921', 'http://bspindia.org/kumari-mayawati.php'),
    'in_bsp_press_release_cochin_20090703':
        ('20090726014731', 'http://bspindia.org/bsp-press-releases.php'),
    'in_bsp_letter_to_cec_office_bearers_20120716':
        ('20150801083802', 'http://eci.nic.in/eci_main/miscellaneous_statistics/PPS2_01032013.pdf'),
    'in_eci_catalogue_order_to_mayawati_20190415':
        ('20190516052807', 'https://www.eci.gov.in/files/file/9927-commissions-order-dated-15042019-to-ms-mayawati-and-shri-yogi-adityanath-along-with-letter-to-all-chief-electoral-officers/'),
    'in_bsp_press_note_lucknow_20210901':
        ('20211230002707', 'https://www.bspindia.org/download/pr/01-09-2021-BSP-PRESS-NOTE-REVIEW-MEETING-2.pdf'),
    'in_npp_site_leadership_page_2019':
        ('20191116212724', 'http://www.nppindia.in/leadership-and-key-people/'),
    'in_npp_press_release_resolution_20200711':
        ('20210225154913', 'https://nppindia.in/2020/07/11/npp-adopts-resolution-to-fight-discrimination-of-ne-citizens/'),
    'in_npp_site_conrad_sangma_page_2021':
        ('20210225155801', 'https://nppindia.in/conrad-sangma/'),
    'in_npp_notification_manipur_candidates_20220204':
        ('20220204184638', 'https://nppindia.in/wp-content/uploads/2022/02/Manipur-Election-2022-List-of-candidates.pdf'),
    'in_npp_notification_national_committee_20250417':
        ('20251013120752', 'https://nppindia.in/wp-content/uploads/2025/04/National-Committee-2025-28.pdf'),
}
# source: (host, REST route, post id)
REST_POSTS = {
    'in_aap_post_national_council_meeting_20221218': ('archive.aamaadmiparty.org', 'posts', 773305),
    'in_aap_post_shokeen_minister_20241118': ('aamaadmiparty.org', 'posts', 883198),
    'in_aap_news_e20_townhall_goa_20260825': ('aamaadmiparty.org', 'aap_news', 904246),
}
# Every claim: (attested_on, event_kind, review observation, holder_name, observation), in packet order.
EVENTS = {
    'in_aap_nri_kejriwal_national_convener_20131007':
        ('2013-10-07', 'in_office_attestation', 'AAP-NC-01', KEJ, AAP),
    'in_aap_letter_first_office_bearer_election_20121125':
        ('2012-11-25', 'election_recalled_dated', 'AAP-NC-01', KEJ, AAP),
    'in_aap_ne_minutes_kejriwal_convener_wef_20160427':
        ('2016-04-27', 'office_bearer_election_effective_day_stated', 'AAP-NC-02', KEJ, AAP),
    'in_aap_release_kejriwal_national_convenor_20161112':
        ('2016-11-12', 'in_office_continuation_attestation', 'AAP-NC-02', KEJ, AAP),
    'in_eci_catalogue_kejriwal_national_convener_20170120':
        ('2017-01-20', 'in_office_continuation_attestation', 'AAP-NC-02', KEJ, AAP),
    'in_aap_post_national_convenor_address_20221218':
        ('2022-12-18', 'in_office_continuation_attestation', 'AAP-NC-02', KEJ, AAP),
    'in_aap_post_kejriwal_national_convenor_approved_20241118':
        ('2024-11-18', 'in_office_continuation_attestation', 'AAP-NC-02', KEJ, AAP),
    'in_aap_news_kejriwal_national_convenor_townhall_20260825':
        ('2026-08-25', 'in_office_continuation_attestation', 'AAP-NC-02', KEJ, AAP),
    'in_bsp_site_kanshi_ram_founded_bsp_19840414':
        ('1984-04-14', 'founding_recalled', 'BSP-NP-01', KR, BSP),
    'in_bsp_site_mayawati_declared_successor_20011215':
        ('2001-12-15', 'successor_designation_recalled', 'BSP-NP-02', MAY, BSP),
    'in_bsp_site_party_made_mayawati_national_president_20030918':
        ('2003-09-18', 'selection_recalled_dated', 'BSP-NP-02', MAY, BSP),
    'in_bsp_profile_mayawati_assumed_national_president_20030918':
        ('2003-09-18', 'assumption_recalled_retrospective', 'BSP-NP-02', MAY, BSP),
    'in_bsp_profile_mayawati_reelected_national_president_20060827':
        ('2006-08-27', 'election_recalled_dated', 'BSP-NP-02', MAY, BSP),
    'in_bsp_profile_mayawati_at_present_national_president':
        (None, 'undated_office_styling', 'BSP-NP-02', MAY, BSP),
    'in_bsp_release_founder_president_kanshi_ram_died_20061009':
        ('2006-10-09', 'death_recalled_with_founder_styling', 'BSP-NP-01', KR, BSP),
    'in_bsp_letter_mayawati_national_president_20120716':
        ('2012-07-16', 'in_office_attestation', 'BSP-NP-02', MAY, BSP),
    'in_eci_catalogue_mayawati_national_president_20190415':
        ('2019-04-15', 'in_office_continuation_attestation', 'BSP-NP-02', MAY, BSP),
    'in_bsp_press_note_mayawati_national_president_20210901':
        ('2021-09-01', 'in_office_continuation_attestation', 'BSP-NP-02', MAY, BSP),
    'in_npp_site_purno_agitok_sangma_founder_president':
        (None, 'retrospective_founder_styling', 'NPP-NP-01', PAS, NPP),
    'in_npp_site_conrad_sangma_onus_national_president_2016':
        (None, 'retrospective_biography_statement', 'NPP-NP-02', CKS, NPP),
    'in_npp_release_conclave_attended_by_national_president_20200711':
        ('2020-07-11', 'in_office_attestation', 'NPP-NP-02', CKS, NPP),
    'in_npp_site_conrad_sangma_elected_national_president_march_2016':
        (None, 'election_recalled_month_only', 'NPP-NP-02', CKS, NPP),
    'in_npp_notification_conrad_sangma_national_president_20220204':
        ('2022-02-04', 'in_office_continuation_attestation', 'NPP-NP-02', CKS, NPP),
    'in_npp_notification_national_committee_conrad_sangma_20250417':
        ('2025-04-17', 'in_office_continuation_attestation', 'NPP-NP-02', CKS, NPP),
}
NEW_SOURCES = list(RESPONSES)
SOURCES_BY_ORG = {
    AAP: [
        'in_aap_nri_release_kejriwal_interact_20131007',
        'in_aap_letter_to_eci_org_election_20150529',
        'in_aap_letter_to_eci_new_national_executive_20160531',
        'in_aap_release_kejriwal_demonetisation_20161112',
        'in_eci_catalogue_order_to_kejriwal_20170120',
        'in_aap_post_national_council_meeting_20221218',
        'in_aap_post_shokeen_minister_20241118',
        'in_aap_news_e20_townhall_goa_20260825',
    ],
    BSP: [
        'in_bsp_site_about_bsp_2009',
        'in_bsp_site_mayawati_profile_2009',
        'in_bsp_press_release_cochin_20090703',
        'in_bsp_letter_to_cec_office_bearers_20120716',
        'in_eci_catalogue_order_to_mayawati_20190415',
        'in_bsp_press_note_lucknow_20210901',
    ],
    NPP: [
        'in_npp_site_leadership_page_2019',
        'in_npp_press_release_resolution_20200711',
        'in_npp_site_conrad_sangma_page_2021',
        'in_npp_notification_manipur_candidates_20220204',
        'in_npp_notification_national_committee_20250417',
    ],
}

HOLDER_KINDS = {'in_office_attestation'}
FROM_KINDS = {'office_bearer_election_effective_day_stated'}
UNTIL_KINDS = set()
ELECTION_KINDS = {'election_recalled_dated', 'election_recalled_month_only', 'selection_recalled_dated'}
ASSUMPTION_RECALLED_KINDS = {'assumption_recalled_retrospective'}
CONTINUATION_KINDS = {'in_office_continuation_attestation'}
RETROSPECTIVE_KINDS = {'retrospective_founder_styling', 'retrospective_biography_statement', 'undated_office_styling'}
OTHER_KINDS = {'founding_recalled', 'successor_designation_recalled', 'death_recalled_with_founder_styling'}
NEVER_KINDS = ELECTION_KINDS | ASSUMPTION_RECALLED_KINDS | CONTINUATION_KINDS | RETROSPECTIVE_KINDS | OTHER_KINDS

# Exact holder observations of each role: (name, attested_on, from, until), in chronological order.
HOLDERS = {
    AAP: [(KEJ, '2013-10-07', None, None), (KEJ, None, '2016-04-27', None)],
    BSP: [(MAY, '2012-07-16', None, None)],
    NPP: [(CKS, '2020-07-11', None, None)],
}
HOLDER_CLAIMS = {
    AAP: [['in_aap_nri_kejriwal_national_convener_20131007'], ['in_aap_ne_minutes_kejriwal_convener_wef_20160427']],
    BSP: [['in_bsp_letter_mayawati_national_president_20120716']],
    NPP: [['in_npp_release_conclave_attended_by_national_president_20200711']],
}
STARTS = [(KEJ, '2016-04-27')]
ENDS = []
HOLDER_OBSERVATIONS = tuple(cid for cid, e in EVENTS.items() if e[1] in HOLDER_KINDS)
NEVER_HOLDER = tuple(cid for cid, e in EVENTS.items() if e[1] in NEVER_KINDS)
# Recalled elections and selections with a day, in packet order (AAP, then BSP); none is a start.
ELECTION_DAYS = ['2012-11-25', '2003-09-18', '2006-08-27']
# Retrospective entries recalling Mayawati's selection and assumption on 18 September 2003; claims only.
RETRO_2003 = ['in_bsp_site_party_made_mayawati_national_president_20030918',
              'in_bsp_profile_mayawati_assumed_national_president_20030918']
HOLDER_DATES = {d for org in ORGS for h in HOLDERS[org] for d in h[1:] if d}
NEVER_HOLDER_DATE = sorted({e[0] for cid, e in EVENTS.items() if cid not in HOLDER_OBSERVATIONS and e[1] not in FROM_KINDS
                            and e[0]} - HOLDER_DATES)
UNDATED = tuple(cid for cid, e in EVENTS.items() if e[0] is None)
PER_REQUEST = re.compile(r'[?&](cb|_|s|q|token|sessionid|search|nocache|_fields|page|csrfKey|do)=|/search|cdn-cgi|'
                         r'X-Amz-|Signature=', re.I)
REPORT = research.RESEARCH / 'india-bsp-aap-npp-leaders-1990-2026-48.md'
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-48.md'


def org_of(packet, org_id):
    org, = [o for o in packet['organizations'] if o['id'] == org_id]
    return org


def role_of(packet, org_id):
    return org_of(packet, org_id)['roles'][0]


def other_role(packet, org_id):
    return org_of(packet, org_id)['roles'][0]


def load_rows(packet):
    rows = {}
    for source in packet['sources']:
        if source['id'] in RESPONSES:
            extract = json.loads((research.ROOT / source['snapshot']['path']).read_text(encoding='utf-8'))
            rows.update({row['claim_id']: row for row in extract['rows']})
    return rows


def leader_rules(packet, rows):
    """Rule-based checks that hold without the pinned holder lists; raise AssertionError, KeyError or ValueError."""
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    claim_source = {c['id']: s['id'] for s in packet['sources'] for c in s['claims']}
    leaders = [(e['id'], r['id']) for e in packet['organizations'] + packet['institutions'] for r in e['roles']
               if r['kind'] == 'party_leader' or r['id'] in {ROLES[o][0] for o in ORGS}]
    assert leaders == LEADERS, 'the seven party-leader roles only, and no copy of these three'
    ours_claims, ours_sources = {}, {}
    for org_id in ORGS:
        role_id, title, name, row_claim = ROLES[org_id]
        orgs = [o for o in packet['organizations'] if o['id'] == org_id]
        assert len(orgs) == 1
        org = orgs[0]
        # The observation keeps its recognition identity: no lifespan, no successor and no game mapping.
        assert (org['name'], org['kind'], org['jurisdiction']) == (name, 'political_party_national_recognition_observation',
                                                                   'India')
        assert org['recognition'] == {'level': 'national', 'attested_on': '2024-03-23', 'from': None, 'until': None,
                                      'source_qualifications': [], 'consolidated_current_status': None}
        assert (org['lifecycle']['status'], org['lifecycle']['from'], org['lifecycle']['until']) == ('unresearched', None,
                                                                                                    None)
        assert org['represented_party_ids'] == [] and org['reconciled_organization_id'] is None
        assert org['coverage']['status'] == 'reporting_identity_only'
        assert org['sources'][0] == 'in_eci_national_parties_20240323' and org['claim_ids'][0] == row_claim
        assert [r['id'] for r in org['roles']] == [role_id], 'exactly one role on the observation'
        role = org['roles'][0]
        assert (role['title'], role['kind']) == (title, 'party_leader')
        assert org['claim_ids'][1:] == role['claim_ids'] and org['sources'][1:] == role['sources']
        for cid in role['claim_ids']:
            row = rows[cid]
            assert (row['observation_id'], row['role_id'], row['role_title']) == (org_id, role_id, title), cid
            assert row['review_observation'] in REVIEW[org_id], cid
            person = row['holder_name']
            assert person is None or SURNAMES[person].lower() in claims[cid]['text'].lower(), (cid, 'named in text')
            assert row['event_kind'] in NEVER_KINDS | HOLDER_KINDS | FROM_KINDS | UNTIL_KINDS, cid
            assert (claims[cid].get('attested_on') or '') <= research.CUTOFF, cid
            if row['event_kind'] in NEVER_KINDS:
                assert cid not in {c for h in role['holder_claims'] for c in h['claim_ids']}, cid
        ours_claims[org_id] = set(role['claim_ids']) | {c for h in role['holder_claims'] for c in h['claim_ids']}
        ours_sources[org_id] = set(role['sources']) | {s for h in role['holder_claims'] for s in h['sources']}
        previous = ''
        for holder in role['holder_claims']:
            person = holder['name']
            assert isinstance(holder, dict) and person in HOLDER_NAMES[org_id], person
            assert person not in NOT_HOLDERS, (person, 'no in-office attestation in the period')
            dated = [d for d in (holder['attested_on'], holder['from']) if d]
            assert len(dated) == 1, (person, 'a holder is dated by exactly one of attested_on and from')
            assert dated[0] >= previous, (person, 'holders stay in chronological order')
            assert '1990-01-01' <= dated[0] <= research.CUTOFF, (person, 'inside the period')
            previous = dated[0]
            assert not set(holder['claim_ids']) & set(NEVER_HOLDER), person
            expected, kinds = [], {}
            for cid in holder['claim_ids']:
                row = rows[cid]
                assert cid in role['claim_ids'], cid
                assert (row['holder_name'], row['role_id'], row['role_title']) == (person, role_id, title), (person, cid)
                assert row['event_kind'] in HOLDER_KINDS | FROM_KINDS | UNTIL_KINDS, (person, cid)
                kinds.setdefault(row['event_kind'], set()).add(claims[cid].get('attested_on'))
                if claim_source[cid] not in expected:
                    expected.append(claim_source[cid])
            assert holder['sources'] == expected, person
            from_days = set().union(*(kinds.get(k, set()) for k in FROM_KINDS))
            until_days = set().union(*(kinds.get(k, set()) for k in UNTIL_KINDS))
            attest_days = set().union(*(kinds.get(k, set()) for k in HOLDER_KINDS))
            # A start only where a source states the day the office took effect; an end only where one states the day it
            # ended.
            assert ({holder['from']} == from_days) if holder['from'] else not from_days, (person, 'start')
            assert ({holder['until']} == until_days) if holder['until'] else not until_days, (person, 'end')
            assert attest_days == ({holder['attested_on']} if holder['attested_on'] else set()), (person, 'observation')
            for cid in holder['claim_ids']:
                if rows[cid]['event_kind'] in FROM_KINDS:
                    assert 'W.E.F.' in claims[cid]['text'], (person, 'the effective day is stated in the text')
            if holder['until']:
                assert holder['until'] <= research.CUTOFF and dated[0] <= holder['until']
    # The three roles share nothing with each other, with the other party roles or with the institutions, and none of
    # their holders sits on another role.
    names = set().union(*HOLDER_NAMES.values()) | NOT_HOLDERS
    for org_id in ORGS:
        for other in ORGS:
            if other != org_id:
                assert not ours_claims[org_id] & ours_claims[other] and not ours_sources[org_id] & ours_sources[other]
    all_claims, all_sources = set().union(*ours_claims.values()), set().union(*ours_sources.values())
    for inst in packet['institutions']:
        i_claims = set(inst['claim_ids']) | {c for r in inst['roles'] for c in r['claim_ids']} | {
            c for r in inst['roles'] for h in r['holder_claims'] for c in h['claim_ids']}
        i_sources = set(inst['sources']) | {s for r in inst['roles'] for s in r['sources']} | {
            s for r in inst['roles'] for h in r['holder_claims'] for s in h['sources']}
        assert not all_claims & i_claims and not all_sources & i_sources, inst['id']
        for r in inst['roles']:
            assert not {h['name'] for h in r['holder_claims']} & names, (inst['id'], 'cross-institution holder')
    for entry in packet['organizations']:
        if entry['id'] not in ORGS:
            assert not set(entry['claim_ids']) & all_claims and not set(entry['sources']) & all_sources, entry['id']
            for r in entry['roles']:
                assert not set(r['claim_ids']) & all_claims and not set(r['sources']) & all_sources, r['id']
                assert not {h['name'] for h in r['holder_claims']} & names, (r['id'], 'cross-party holder')
    for cid in NEVER_HOLDER:
        assert rows[cid]['event_kind'] in NEVER_KINDS, cid
    for cid in UNDATED:
        assert not {'attested_on', 'period', 'attested_period'} & set(claims[cid]), cid


def leader_invariants(packet, rows):
    """The rules plus the exact pinned holder lists this packet intends."""
    leader_rules(packet, rows)
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    starts, ends = [], []
    for org_id in ORGS:
        role = role_of(packet, org_id)
        got = [(h['name'], h['attested_on'], h['from'], h['until']) for h in role['holder_claims']]
        assert got == HOLDERS[org_id], got
        for holder in role['holder_claims']:
            assert not {holder['attested_on'], holder['from'], holder['until']} & set(NEVER_HOLDER_DATE), holder['name']
        assert [h['claim_ids'] for h in role['holder_claims']] == HOLDER_CLAIMS[org_id]
        starts += [(h['name'], h['from']) for h in role['holder_claims'] if h['from']]
        ends += [(h['name'], h['until']) for h in role['holder_claims'] if h['until']]
        assert role['claim_ids'] == [cid for cid, e in EVENTS.items() if e[4] == org_id]
        assert role['sources'] == SOURCES_BY_ORG[org_id]
    assert starts == STARTS and ends == ENDS
    # Distinct dated events stay distinct and keep their own days, kinds, people and observations.
    for cid, (day, kind, review, name, org_id) in EVENTS.items():
        assert claims[cid].get('attested_on') == day, cid
        assert (rows[cid]['attested_on'], rows[cid]['event_kind'], rows[cid]['review_observation'],
                rows[cid]['holder_name'], rows[cid]['observation_id']) == (day, kind, review, name, org_id), cid
    # The earlier prime-minister, president and party-role holders are unchanged.
    pm, presidency = packet['institutions']
    assert [(h['name'], h['attested_on'], h['from'], h['until']) for h in pm['roles'][0]['holder_claims']] == c01_11.HOLDERS
    assert [(h['name'], h['attested_on'], h['from'], h['until'])
            for h in presidency['roles'][0]['holder_claims']] == c01_15.HOLDERS
    for mod in (c01_20, c01_27, c01_33, c01_40):
        other = other_role(packet, mod.ORG)
        assert [(h['name'], h['attested_on'], h['from'], h['until']) for h in other['holder_claims']] == mod.HOLDERS
    for mod in (c01_20, c01_27, c01_40):
        assert other_role(packet, mod.ORG)['claim_ids'] == list(mod.EVENTS), mod.ROLE


class IndiaBspAapNppLeadersTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.raw = (research.ROOT / research.RESEARCH / 'india.json').read_text(encoding='utf-8')
        cls.packet = json.loads(cls.raw)
        cls.sources = {s['id']: s for s in cls.packet['sources']}
        cls.claims = {c['id']: c for s in cls.packet['sources'] for c in s['claims']}
        cls.roles = {org_id: role_of(cls.packet, org_id) for org_id in ORGS}
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
        self.assertEqual((len(NEW_SOURCES), len(self.new_claims)), (19, 24))
        pm, presidency = self.packet['institutions']
        inc, bjp, cpm = (other_role(self.packet, m.ORG) for m in (c01_20, c01_27, c01_40))
        jd = org_of(self.packet, c01_33.ORG)
        # This packet's sources are appended after the CLAUDE-C01-40 sources, AAP first, then BSP, then NPP.
        self.assertEqual([s['id'] for s in self.packet['sources']],
                         list(c01_27.ORIGINAL_SOURCES) + pm['sources'] + presidency['sources'] + inc['sources']
                         + bjp['sources'] + jd['sources'] + cpm['sources'] + NEW_SOURCES)
        self.assertEqual(NEW_SOURCES, SOURCES_BY_ORG[AAP] + SOURCES_BY_ORG[BSP] + SOURCES_BY_ORG[NPP])
        self.assertEqual((cpm['sources'], jd['sources']), (c01_40.NEW_SOURCES, c01_33.NEW_SOURCES))
        self.assertEqual((len(ids['entries']), len(ids['sources']), len(ids['claims']), len(ids['roles'])),
                         (85, 336, 649, 9))
        self.assertEqual(len(self.packet['organizations']), 83)
        for org_id in ORGS:
            org = org_of(self.packet, org_id)
            self.assertEqual(org['sources'], ['in_eci_national_parties_20240323'] + SOURCES_BY_ORG[org_id])
            self.assertEqual(org['claim_ids'], [ROLES[org_id][3]] + [c for c, e in EVENTS.items() if e[4] == org_id])
        # Every new claim is a holder claim or a claim that never feeds a holder; never both.
        holder_claims = [cid for org_id in ORGS for ids_ in HOLDER_CLAIMS[org_id] for cid in ids_]
        self.assertEqual(len(set(holder_claims)), len(holder_claims))
        self.assertFalse(set(holder_claims) & set(NEVER_HOLDER))
        self.assertEqual(set(holder_claims) | set(NEVER_HOLDER), set(self.new_claims))
        self.assertEqual(sorted(set(holder_claims) - {c for c, e in EVENTS.items() if e[1] in FROM_KINDS | UNTIL_KINDS}),
                         sorted(HOLDER_OBSERVATIONS))
        self.assertEqual((len(holder_claims), len(NEVER_HOLDER), sum(len(h) for h in HOLDERS.values())), (4, 20, 4))
        # At most ten people, each named as the sources print them; every claim names someone.
        people = {e[3] for e in EVENTS.values()} - {None}
        self.assertEqual(people, set(SURNAMES))
        self.assertLessEqual(len(people), 10)
        self.assertFalse([cid for cid, e in EVENTS.items() if e[3] is None])
        # Six observations, each reported and each carrying rows.
        observations = re.findall(r'^### ((?:AAP-NC|BSP-NP|NPP-NP)-\d\d)\b', self.report, re.M)
        self.assertEqual(observations, REVIEW[AAP] + REVIEW[BSP] + REVIEW[NPP])
        self.assertEqual({e[2] for e in EVENTS.values()}, set(observations))

    def test_holders_are_exactly_as_intended(self):
        leader_invariants(self.packet, self.rows)
        for org_id in ORGS:
            role = self.roles[org_id]
            self.assertEqual(role['scope_note'].count('CLAUDE-C01-48'), 1)
            for holder in role['holder_claims']:
                if holder['from']:
                    self.assertTrue(holder['note'].startswith('From '), holder['name'])
                    self.assertTrue(holder['uncertainty'].startswith('No end'), holder['name'])
                else:
                    self.assertTrue(holder['note'].startswith('Observed on '), holder['name'])
                    self.assertTrue(holder['uncertainty'].startswith('No start'), holder['name'])

    def test_starts_and_ends_only_where_a_source_states_one(self):
        # One stated start: the AAP National Executive's certified minutes of 27 April 2016, 'W.E.F. 27.04.2016'.
        start, = [cid for cid, e in EVENTS.items() if e[1] in FROM_KINDS]
        self.assertEqual(start, 'in_aap_ne_minutes_kejriwal_convener_wef_20160427')
        self.assertIn("'ELECTED THE FOLLOWING PERSONS AS OFFICE BEARER OF THE PARTY W.E.F. 27.04.2016'",
                      self.claims[start]['text'])
        self.assertIn('NATIONAL CONVENER', self.claims[start]['text'])
        self.assertEqual(self.sources['in_aap_letter_to_eci_new_national_executive_20160531']['source_type'],
                         'primary_party_record_archived_pdf')
        # No source states an end, so no holder has one.
        self.assertFalse([h for org_id in ORGS for h in self.roles[org_id]['holder_claims'] if h['until']])
        # Recalled elections and selections, retrospective entries, stylings, founding, successor designation, the
        # founder's death and later attestations are never boundaries.
        for cid, (day, kind, _review, name, org_id) in EVENTS.items():
            if kind in NEVER_KINDS:
                for holder in self.roles[org_id]['holder_claims']:
                    self.assertNotIn(cid, holder['claim_ids'])
                    if day and holder['name'] == name:
                        self.assertNotIn(day, (holder['from'], holder['until']), cid)
        self.assertEqual([e[0] for e in EVENTS.values() if e[1] in ELECTION_KINDS and e[0]], ELECTION_DAYS)
        # Mayawati's selection and assumption of 18 September 2003 are retrospective entries: claims, never a start.
        mayawati, = self.roles[BSP]['holder_claims']
        self.assertIsNone(mayawati['from'])
        for cid in RETRO_2003:
            self.assertEqual(self.claims[cid]['attested_on'], '2003-09-18')
            self.assertIn('retrospective', self.claims[cid]['uncertainty'])
            self.assertEqual(self.sources[next(s for s in NEW_SOURCES if cid in [c['id'] for c in
                                                                               self.sources[s]['claims']])]['source_type'],
                             'primary_party_record_archived_retrospective')
        self.assertIn('18 September 2003', mayawati['uncertainty'])
        # The 2012 first election of AAP office bearers is recalled, not a start; the 2016 term gives no end to 2013.
        first, second = self.roles[AAP]['holder_claims']
        self.assertIsNone(first['until'])
        self.assertIn('25.11.2012', first['uncertainty'])
        self.assertNotIn('2012-11-25', (first['attested_on'], first['from']))
        # The founders hold nothing: Kanshi Ram's death is recalled with the 'founder President' styling, Purno Agitok
        # Sangma has years of life only.
        death, = [cid for cid, e in EVENTS.items() if e[1] == 'death_recalled_with_founder_styling']
        self.assertEqual((EVENTS[death][0], EVENTS[death][3]), ('2006-10-09', KR))
        self.assertIn('not treated as a death in office', self.claims[death]['uncertainty'])
        for name in NOT_HOLDERS:
            self.assertNotIn(name, {h['name'] for org_id in ORGS for h in self.roles[org_id]['holder_claims']})
        self.assertEqual([cid for cid, e in EVENTS.items() if e[3] == PAS],
                         ['in_npp_site_purno_agitok_sangma_founder_president'])
        self.assertIn('claims only', self.roles[BSP]['scope_note'])
        self.assertIn('claims only', self.roles[NPP]['scope_note'])

    def test_party_and_state_offices_stay_separate(self):
        leader_rules(self.packet, self.rows)
        pm, presidency = self.packet['institutions']
        for inst in (pm, presidency):
            self.assertFalse(set(inst['sources']) & set(NEW_SOURCES), inst['id'])
            self.assertFalse(set(inst['claim_ids']) & set(self.new_claims), inst['id'])
        for entry in self.packet['organizations']:
            if entry['id'] not in ORGS:
                self.assertFalse(set(entry['sources']) & set(NEW_SOURCES), entry['id'])
                self.assertFalse(set(entry['claim_ids']) & set(self.new_claims), entry['id'])
        for org_id in ORGS:
            self.assertIn('Party office and state office stay separate', self.roles[org_id]['scope_note'])
        rows = json.dumps([self.extracts[s]['rows'] for s in NEW_SOURCES])
        for other in ('in_prime_minister', 'in_presidency', 'in_inc_president', 'in_bjp_president', 'in_jd_president',
                      'in_cpm_general_secretary'):
            self.assertNotIn(other, rows)
        # Chief Ministerships named in the sources are never recorded as roles.
        self.assertEqual([r['id'] for org_id in ORGS for r in org_of(self.packet, org_id)['roles']],
                         [ROLES[org_id][0] for org_id in ORGS])

    def test_extracts_match_packet_claims_and_record_original_responses(self):
        for sid in NEW_SOURCES:
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual(extract['format'], 'spheres-c01-derived-factual-table/v1')
            self.assertEqual((extract['source_id'], extract['source_url']), (sid, source['url']))
            for key in ('scope_note', 'access_method', 'published_date', 'rights_note', 'accessed_date'):
                self.assertEqual(extract[key], source[key], (sid, key))
            self.assertEqual(source['accessed_date'], '2026-10-01', sid)
            self.assertLessEqual(source['published_date'] or '', research.CUTOFF)
            self.assertIs(extract['source_response_checked_in'], False)
            self.assertEqual((extract['source_response_bytes'], extract['source_response_sha256']), RESPONSES[sid])
            for phrase in ('not checked into this repository', 'derived factual extract', 'same byte count and SHA-256',
                           'at least 30 minutes apart', 'no Accept-Encoding request header', 'default User-Agent',
                           'no Content-Encoding'):
                self.assertIn(phrase, extract['provenance_note'], sid)
            self.assertIn('No open license, portrait permission or likeness approval', extract['rights_note'])
            self.assertIn('CLAUDE-C01-48 only', extract['bounded_scope'])
            pages = extract['visual_review']['pdf_pages_one_based']
            self.assertEqual(bool(pages), source['access_method'].startswith('downloaded_archived_raw_capture_pdf'), sid)
            self.assertTrue(source['source_type'].startswith('primary_') and source['scope_note'] and source['publisher'])
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
                if 'pdf_page' in claim['locator']:
                    self.assertIn(claim['locator']['pdf_page'], pages, sid)
            self.assertEqual('original_url' in source, sid in WAYBACK, sid)
            self.assertEqual('archive_capture_utc' in extract, sid in WAYBACK, sid)
        self.assertEqual(len({self.sources[s]['snapshot']['path'] for s in NEW_SOURCES}), len(NEW_SOURCES))
        self.assertEqual({cid: (row['attested_on'], row['event_kind'], row['review_observation'], row['holder_name'],
                                row['observation_id']) for cid, row in self.rows.items()}, EVENTS)
        self.assertEqual(set(WAYBACK) | set(REST_POSTS), set(NEW_SOURCES))
        self.assertFalse(set(WAYBACK) & set(REST_POSTS))
        self.assertEqual((len(WAYBACK), len(REST_POSTS)), (16, 3))
        for sid, (stamp, original) in WAYBACK.items():
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual((source['original_url'], extract['original_url']), (original, original))
            self.assertLess(stamp, '20260907')
            self.assertEqual(extract['archive_capture_utc'].replace('-', '').replace(':', '').replace('T', '').rstrip('Z'),
                             stamp)
            self.assertIn('Raw Internet Archive capture (id_ form)', extract['provenance_note'])
            captured = f'Internet Archive capture of {int(stamp[6:8])} {MONTHS[int(stamp[4:6]) - 1]} {stamp[:4]}'
            self.assertTrue(source['title'].endswith(captured), sid)
        for sid, (host, route, post) in REST_POSTS.items():
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertIn(f'WordPress REST record of post {post}', extract['provenance_note'])
            self.assertIn('change on every request', extract['provenance_note'])
            self.assertIn(f'post {post}', source['title'])
            self.assertTrue(source['published_date'], sid)
            self.assertEqual(source['access_method'], 'party_website_rest_record_download_and_post_text_read')

    def test_response_identities_are_reproducible_urls(self):
        for sid in NEW_SOURCES:
            url = self.sources[sid]['url']
            self.assertIsNone(PER_REQUEST.search(url), url)
            parts = urlsplit(url)
            self.assertEqual(parts.scheme, 'https', url)
            self.assertEqual(parts.query, '', url)
            if sid in WAYBACK:
                stamp, original = WAYBACK[sid]
                prefix = f'https://web.archive.org/web/{stamp}id_/'
                self.assertTrue(url.startswith(prefix), sid)
                # The capture is addressed as the archive records it (some with the explicit port 80).
                self.assertEqual(url[len(prefix):].replace(':80/', '/', 1), original, sid)
            else:
                host, route, post = REST_POSTS[sid]
                self.assertEqual(url, f'https://{host}/wp-json/wp/v2/{route}/{post}', sid)
        self.assertEqual({urlsplit(self.sources[s]['url']).hostname for s in NEW_SOURCES},
                         {'web.archive.org', 'aamaadmiparty.org', 'archive.aamaadmiparty.org'})

    def test_secondary_and_unimported_leads_stay_out_of_the_packet(self):
        lowered = json.dumps([self.sources[s] for s in NEW_SOURCES], ensure_ascii=False).lower()
        for marker in ('wikipedia', 'britannica', 'thehindu', 'indianexpress', 'ndtv', 'hindustantimes', 'timesofindia',
                       'deccanherald', 'business-standard', 'tribuneindia', 'outlookindia', 'aninews', 'assamtribune'):
            self.assertNotIn(marker, lowered, marker)
        leads, added = self.section('Leads not imported'), self.section('Sources added')
        for marker in ('aap_news/905580', '10347-recognition-of-national-peoples-party', 'bsp-news-till-sep2009.php',
                       'Organisaional_Election_of_the_parties.pdf', '7842-national-peoples-party'):
            self.assertIn(marker, leads, marker)
            self.assertNotIn(marker, added, marker)

    def test_packet_formatting_is_preserved(self):
        data = (research.ROOT / research.RESEARCH / 'india.json').read_bytes()
        self.assertNotIn(b'\r', data)
        self.assertEqual(data.decode('utf-8'), json.dumps(self.packet, indent=2, ensure_ascii=False) + '\n')

    def test_mutations_are_rejected(self):
        def holder(p, org_id, i=0):
            return role_of(p, org_id)['holder_claims'][i]

        def fake(name, day, sid, cid, start=None):
            return {'name': name, 'attested_on': None if start else day, 'from': start, 'until': None, 'sources': [sid],
                    'claim_ids': [cid], 'note': '', 'uncertainty': ''}

        mutations = [
            ("retrospective 2003 assumption used as Mayawati's start",
             lambda p: (holder(p, BSP).update({'from': '2003-09-18', 'attested_on': None,
                                               'sources': ['in_bsp_site_mayawati_profile_2009'],
                                               'claim_ids': ['in_bsp_profile_mayawati_assumed_national_president_20030918']}))),
            ('recalled 2012 election used as the first AAP start',
             lambda p: holder(p, AAP).update({'from': '2012-11-25', 'attested_on': None,
                                              'sources': ['in_aap_letter_to_eci_org_election_20150529'],
                                              'claim_ids': ['in_aap_letter_first_office_bearer_election_20121125']})),
            ('the stated start removed (Kejriwal, 27 April 2016)',
             lambda p: holder(p, AAP, 1).update({'from': None, 'attested_on': '2016-04-27'})),
            ("the 2016 re-election used as the end of the 2013 observation",
             lambda p: holder(p, AAP).update({'until': '2016-04-27'})),
            ("a successor's start used as an end (Kejriwal until the day before 27 April 2016)",
             lambda p: holder(p, AAP).update({'until': '2016-04-26'})),
            ("Kanshi Ram made a holder from the founder's death", lambda p: role_of(p, BSP)['holder_claims'].insert(
                0, fake(KR, '2006-10-09', 'in_bsp_press_release_cochin_20090703',
                        'in_bsp_release_founder_president_kanshi_ram_died_20061009'))),
            ('Purno Agitok Sangma made a holder from an undated styling',
             lambda p: role_of(p, NPP)['holder_claims'].insert(
                 0, fake(PAS, '2013-01-06', 'in_npp_site_leadership_page_2019',
                         'in_npp_site_purno_agitok_sangma_founder_president'))),
            ('a continuation attestation used as the observation (Mayawati, 1 September 2021)',
             lambda p: holder(p, BSP).update({'attested_on': '2021-09-01',
                                              'sources': ['in_bsp_press_note_lucknow_20210901'],
                                              'claim_ids': ['in_bsp_press_note_mayawati_national_president_20210901']})),
            ('an Election Commission catalogue entry used as holder evidence',
             lambda p: role_of(p, BSP)['holder_claims'].append(
                 fake(MAY, '2019-04-15', 'in_eci_catalogue_order_to_mayawati_20190415',
                      'in_eci_catalogue_mayawati_national_president_20190415'))),
            ('holders out of order', lambda p: role_of(p, AAP)['holder_claims'].reverse()),
            ('a holder dated by both attested_on and from', lambda p: holder(p, NPP).update({'from': '2020-07-11'})),
            ('a second role on the BSP observation',
             lambda p: org_of(p, BSP)['roles'].append(dict(copy.deepcopy(role_of(p, BSP)), id='in_bsp_copy'))),
            ('a game mapping granted', lambda p: org_of(p, AAP).update({'represented_party_ids': ['India/aap']})),
            ('lifecycle closed at the recognition date',
             lambda p: org_of(p, NPP)['lifecycle'].update({'status': 'dissolved', 'until': '2024-03-23'})),
            ('a claim moved to the prime-ministership',
             lambda p: p['institutions'][0]['claim_ids'].append('in_aap_release_kejriwal_national_convenor_20161112')),
            ('a holder shared with the CPI(M) role',
             lambda p: other_role(p, c01_40.ORG)['holder_claims'].append(
                 {'name': MAY, 'attested_on': '2016-01-01', 'from': None, 'until': None, 'sources': [],
                  'claim_ids': []})),
            ('an AAP source cited by the BSP role',
             lambda p: (role_of(p, BSP)['sources'].append('in_aap_release_kejriwal_demonetisation_20161112'),
                        org_of(p, BSP)['sources'].append('in_aap_release_kejriwal_demonetisation_20161112'))),
            ('an election claim dropped from the role',
             lambda p: role_of(p, BSP)['claim_ids'].remove('in_bsp_profile_mayawati_reelected_national_president_20060827')),
            ('a holder dated after the cutoff', lambda p: holder(p, NPP).update({'attested_on': '2026-09-10'})),
        ]
        failures = []
        for label, fn in mutations:
            p = copy.deepcopy(self.packet)
            fn(p)
            try:
                leader_invariants(p, self.rows)
            except (AssertionError, KeyError, IndexError, StopIteration, ValueError):
                continue
            failures.append(label)
        self.assertEqual(failures, [])
        self.assertEqual(len(mutations), 19)
        # A claim whose day is changed, and an extract row that disagrees with the packet, are both caught.
        p = copy.deepcopy(self.packet)
        next(c for s in p['sources'] for c in s['claims']
             if c['id'] == 'in_bsp_profile_mayawati_reelected_national_president_20060827')['attested_on'] = '2006-08-28'
        with self.assertRaises(AssertionError):
            leader_invariants(p, self.rows)
        rows = copy.deepcopy(self.rows)
        rows['in_bsp_profile_mayawati_assumed_national_president_20030918']['event_kind'] = 'in_office_attestation'
        with self.assertRaises(AssertionError):
            leader_invariants(self.packet, rows)
        rows = copy.deepcopy(self.rows)
        rows['in_aap_letter_first_office_bearer_election_20121125']['event_kind'] = \
            'office_bearer_election_effective_day_stated'
        with self.assertRaises(AssertionError):
            leader_invariants(self.packet, rows)

    def test_report_and_handoff_close_no_parent_gate(self):
        self.assertIn('State: **ready_for_review**', self.report)
        for heading in ('Outcome', 'Observations', 'Sources added', 'Response identities and stability checks',
                        'Leads not imported', 'Sources attempted', 'Suggested next work orders',
                        "Integration notes (outside this packet's file boundary)", 'Decisions for Codex', 'Checks'):
            self.section(heading)
        self.assertIn('### Date ledger', self.report)
        self.assertIn('continue existing claims first', self.section("Integration notes (outside this packet's file "
                                                                     'boundary)'))
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        self.assertIn('State: **ready_for_review**', handoff)
        for text in ('C01 (incomplete)', 'research-index.json', 'continue existing claims first', 'Decisions for Codex'):
            self.assertIn(text, handoff)
        prefixes = {AAP: 'AAP National Convenors 2012-2026 (CLAUDE-C01-48)',
                    BSP: 'BSP National Presidents 1990-2026 (CLAUDE-C01-48)',
                    NPP: 'NPP National Presidents 2013-2026 (CLAUDE-C01-48)'}
        for org_id, prefix in prefixes.items():
            unresolved = org_of(self.packet, org_id)['coverage']['unresolved']
            self.assertEqual(len(unresolved), 4)
            self.assertTrue(unresolved[-1].startswith(prefix))
        coverage = self.packet['coverage']
        self.assertEqual(sum('CLAUDE-C01-48' in u for u in coverage['unresolved']), 1)
        self.assertTrue(coverage['unresolved'][-1].startswith('BSP, AAP and NPP national leaders 1990-2026 (CLAUDE-C01-48, '
                                                              'AAP-NC-01..02, BSP-NP-01..02, NPP-NP-01..02)'))
        self.assertEqual(len(coverage['unresolved']), 14)
        index = research.build()
        country = next(p for p in index['countries'] if p['nation'] == 'India')
        self.assertFalse(country['country_census_complete'])
        self.assertIsNone(country['unrepresented_organization_count'])
        self.assertEqual((country['organization_observations'], country['institution_observations'],
                          country['role_observations']), (83, 2, 9))
        self.assertEqual(country['mapping_pending'], 85)
        self.assertEqual({w['status'] for w in index['work_orders'] if w['nation'] == 'India'}, {'open'})
        self.assertFalse(index['c01_complete'])


if __name__ == '__main__':
    unittest.main()
