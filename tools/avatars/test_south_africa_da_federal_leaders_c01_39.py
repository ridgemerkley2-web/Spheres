"""CLAUDE-C01-39: the Federal Leader of the Democratic Alliance, 2000-2026, extends the existing S10h role
za_da_federal_leader and stays one party office kept apart from every other role and from state office. The two S10h
holders are unchanged; every added holder is a dated observation with no start, because no record reviewed states one.
Ridge's rulings: Helen Zille's own acceptance speech of 6 May 2007 dates an observation, never a start (b), and Mmusi
Maimane's last observation ends on 23 October 2019, the Wednesday of the DA's vacancy statement of Friday 25 October 2019
resolved from its dateline (a). Result publications, congress sessions, the resignation decision, interim service and
tributes stay separate claims; the Democratic Party (1989-2000), the DA's formation and the DP/NNP alliance are claims
about organizations with no role, never merged identities."""
import copy
import hashlib
import json
import re
import unittest
from datetime import date, timedelta
from urllib.parse import urlsplit

import campaign_research as research

OBS = 'za_iec_n2024_027'
NAME = 'DEMOCRATIC ALLIANCE'
ROW = 'national-results-page-1-row-27'
ROW_NO = '027'
ROLE = 'za_da_federal_leader'
TITLE = 'Federal Leader'
INTERIM = 'Interim Federal Leader'
DP_LEADER = 'Leader of the Democratic Party'
PREFIX = 'za_da_'
# The S10h intake's DA sources, role claims and holders, unchanged by this packet.
S10H_SOURCES = ['za_iec_national_results_20240621', 'za_iec_national_seats_20240606', 'za_da_kzn_leader_20230403',
                'za_da_leadership_20260412']
S10H_ORG_CLAIMS = [f'za_n2024_ballot_{ROW_NO}', f'za_n2024_seats_{ROW_NO}', 'za_da_steenhuisen_attested_20230403',
                   'za_da_leader_elected_20260412', 'za_da_federal_chair_elected_20260412',
                   'za_da_council_chair_elected_20260412']
S10H_ROLE_SOURCES = ['za_da_kzn_leader_20230403', 'za_da_leadership_20260412']
S10H_ROLE_CLAIMS = ['za_da_steenhuisen_attested_20230403', 'za_da_leader_elected_20260412']
S10H_NOTE = 'Dated observation only; no continuous term, game identity, biography or likeness rights granted.'
S10H_HOLDERS = [
    {'name': 'John Steenhuisen', 'attested_on': '2023-04-03', 'from': None, 'until': None,
     'sources': ['za_da_kzn_leader_20230403'], 'claim_ids': ['za_da_steenhuisen_attested_20230403'], 'note': S10H_NOTE},
    {'name': 'Geordin Hill-Lewis', 'attested_on': '2026-04-12', 'from': None, 'until': None,
     'sources': ['za_da_leadership_20260412'], 'claim_ids': ['za_da_leader_elected_20260412'], 'note': S10H_NOTE},
]
CHAIR_ROLES = [('za_da_federal_chair', 'Federal Chairperson', 'Solly Msimanga'),
               ('za_da_council_chair', 'Chairperson of the Federal Council', 'Ashor Sarupen')]
# Every other role, unchanged: holder observation counts (pinned literally to avoid a circular test import).
OTHER_ROLE_HOLDER_COUNTS = {'za_president_election': 10, 'za_state_president': 1, 'za_deputy_president': 12,
                            'za_acdp_president': 5, 'za_anc_president': 10, 'za_ifp_president': 8, 'za_pac_president': 14,
                            'za_ff_leader': 9}
ROLE_MAP = {'za_iec_n2024_008': ['za_acdp_president'], 'za_iec_n2024_014': ['za_anc_president'],
            'za_iec_n2024_027': ['za_da_federal_leader', 'za_da_federal_chair', 'za_da_council_chair'],
            'za_iec_n2024_034': ['za_ifp_president'], 'za_iec_n2024_039': ['za_pac_president'],
            'za_iec_n2024_051': ['za_ff_leader']}
# The packet's sources before this packet: S10h, CLAUDE-C01-09, -16, -21, -30 and -32.
EARLIER_SOURCE_COUNT = 245
LIFECYCLE_NOTE = 'These observations do not establish founding, dissolution, legal continuity, mergers, splits or exact terms.'

# Original response identity recorded in each extract, (bytes, sha256), fetched as the extract's fetch_recipe says:
# the raw id_ capture, no Accept-Encoding header, no decoding. Every source is a raw Internet Archive capture served
# without content encoding.
RESPONSES = {
    'za_da_dp_speech_taalbeleid_19970522': (22392, 'e998f741414e4578ca3c0c762b1c6fdcd4002c2e94398609868e0325c52fac40'),
    'za_da_dp_party_info_page_2000': (24997, '1fa2a4293c185a617137699512ed60d16e6fcff9773bc15297bea00e8400d5c4'),
    'za_da_dp_whos_who_page_1999': (25299, '164b275fb975d2162e9d56a513809f07171dc4d337457ccc56b0b03bb31c3bdc'),
    'za_da_speech_campaign_launch_deputy_leader_20001014': (7920, 'afb6df11d7b729a5161d074e0526a52f46f77c6e41151bf0ab82be8aad3dfaa7'),
    'za_da_about_introduction_page_2001': (2348, 'ff35358de1293ee6de90502c510c3984cc67763d095d5b790959a705aaf2b7da'),
    'za_da_speech_local_elections_launch_20001014': (15073, '532864af1aa95fd6df60f2ebcba862271835474ae23e13e240d6072785bdd922'),
    'za_da_speech_kzn_coalition_20001122': (11474, '3ed5f3fae2cefc3e2210c48891fded9115bf4da46c61a6b72defb8d797e9f9ff'),
    'za_da_zille_acceptance_speech_20070506': (55578, '4c069e135b40f5447ee8c2c6475efbfc6ed135e3559ca0cd17d81b0b684171c0'),
    'za_da_leon_former_leader_profile_2007': (34802, '1b1579544269c46571b03cf7183494306c3037538b57b00594cd5840f206d92c'),
    'za_da_sa_today_provinces_20101015': (23610, 'e0bc603e06ee7d809ce03502f1e6753f340c7793fab157fe7365d14ba59d5b41'),
    'za_da_press_release_nkandla_20121104': (20704, '1a8c346606aaf701b344cb86e16d914010853d46e002265933f17144cf16d007'),
    'za_da_speech_growth_victory_20140509': (21843, 'ac49080070bd84b5b39b710de005070462024267717c4c7ee5287f282ace70e5'),
    'za_da_federal_leadership_announcement_20150510': (69610, '67a5d71e41832a91f8f2b4eb799b1805195f40dd82354a062ce67725d7dd6cd7'),
    'za_da_news_election_posters_20160425': (75816, 'a8f9b17bc3e314563c58b833a36e28fe3a7d7f9516bfa94f9b7c267fce9ff6c2'),
    'za_da_news_puppet_allegations_20180407': (78240, '3ce2f52bdac78feeb974f69b9d24d42efffdfed3129d09a06b58e40ec58ac6a3'),
    'za_da_closing_speech_congress_20180408': (86722, '60cd833b785500e650b9c05e87f65fabda7d56d01a3efc40ab8e939686fb5d25'),
    'za_da_finance_chair_statement_20191004': (95560, 'd2a31ff9a233170eb2c37b195ae19231853c77efef635876bf112b84d130cce0'),
    'za_da_fedcouncil_chair_leadership_vacancies_20191024': (145135, '28b3452ba46f8665f4fc3b14875504a43d8ddaae8fa275b8716ab330995a76c0'),
    'za_da_fedcouncil_chair_interim_election_20191025': (118278, '521125be68dfeb329d5e5f491ef72d81c9d956d3a41170c40beb3dd9ff23de64'),
    'za_da_fedex_statement_interim_leadership_20191123': (112207, '3251192c1f1c3cd9b09f00ef1cfe092c789d1ec73c4485e250cad2749da01b85'),
    'za_da_leadership_election_results_20201101': (140775, '3a94aa39c9947a2b8adca0405e8829e10c1fed5f00f68764bdbb775fd8e562c3'),
    'za_da_statement_stellenbosch_visit_20210310': (134450, '730846b64891bac5bcd0fe512300a18b2413d9e8274fce780ff7027d04d5e32c'),
    'za_da_statement_thanks_steenhuisen_20260204': (125751, 'ab6172e0c47f1efeb8cc7d4a1b6260c47ed39f9e58ab0f33bdcf94585cac3c49'),
    'za_da_statement_condolences_cope_20260304': (121009, '11cb414dc370c41b24d493ee466222a90c287e711076911438c58bd36e5d34ca'),
}
# Capture timestamp of each raw id_ capture (all before the 7 September 2026 cutoff).
ARCHIVED = {
    'za_da_dp_speech_taalbeleid_19970522': '19991117021031',
    'za_da_dp_party_info_page_2000': '20000413093038',
    'za_da_dp_whos_who_page_1999': '19990220054116',
    'za_da_speech_campaign_launch_deputy_leader_20001014': '20010506224314',
    'za_da_about_introduction_page_2001': '20011122020551',
    'za_da_speech_local_elections_launch_20001014': '20010501013646',
    'za_da_speech_kzn_coalition_20001122': '20010502012024',
    'za_da_zille_acceptance_speech_20070506': '20070518152213',
    'za_da_leon_former_leader_profile_2007': '20070907071445',
    'za_da_sa_today_provinces_20101015': '20101019153909',
    'za_da_press_release_nkandla_20121104': '20121108013705',
    'za_da_speech_growth_victory_20140509': '20140603024344',
    'za_da_federal_leadership_announcement_20150510': '20150715172231',
    'za_da_news_election_posters_20160425': '20160426174047',
    'za_da_news_puppet_allegations_20180407': '20180407201337',
    'za_da_closing_speech_congress_20180408': '20180408142243',
    'za_da_finance_chair_statement_20191004': '20191006050047',
    'za_da_fedcouncil_chair_leadership_vacancies_20191024': '20211108232735',
    'za_da_fedcouncil_chair_interim_election_20191025': '20191028070115',
    'za_da_fedex_statement_interim_leadership_20191123': '20191128130944',
    'za_da_leadership_election_results_20201101': '20201101140257',
    'za_da_statement_stellenbosch_visit_20210310': '20210311042629',
    'za_da_statement_thanks_steenhuisen_20260204': '20260204113607',
    'za_da_statement_condolences_cope_20260304': '20260410145936',
}
# Every new claim's (attested_on, event_kind), exactly: distinct dated events are never re-dated or relabelled.
EVENTS = {
    'za_da_dp_speech_leader_of_dp_leon_19970522': ('1997-05-22', 'other_party_leader_attestation'),
    'za_da_dp_formed_by_merger_retrospective': (None, 'other_party_founding_retrospective'),
    'za_da_dp_leader_de_beer_retrospective': (None, 'other_party_leader_reference_retrospective'),
    'za_da_dp_whos_who_dp_leader_leon_1999': (None, 'other_party_leader_continuation_attestation'),
    'za_da_dp_whos_who_leon_elected_dp_leadership_1994': (None, 'other_party_election_reference_retrospective'),
    'za_da_speech_dp_and_nnp_led_into_da_20001014': ('2000-10-14', 'alliance_of_parties_statement'),
    'za_da_formed_by_dp_nnp_fa_retrospective': (None, 'formation_by_parties_retrospective'),
    'za_da_speech_leader_leon_20001014': ('2000-10-14', 'in_office_attestation'),
    'za_da_speech_leader_leon_20001122': ('2000-11-22', 'in_office_attestation'),
    'za_da_zille_elected_leader_acceptance_20070506': ('2007-05-06', 'in_office_attestation'),
    'za_da_profile_leon_former_leader_2007': (None, 'former_holder_styled'),
    'za_da_profile_formation_under_leon_leadership': (None, 'formation_under_leadership_retrospective'),
    'za_da_sa_today_leader_zille_20101015': ('2010-10-15', 'in_office_attestation'),
    'za_da_press_release_leader_zille_20121104': ('2012-11-04', 'in_office_attestation'),
    'za_da_speech_leader_zille_20140509': ('2014-05-09', 'in_office_attestation'),
    'za_da_sixth_federal_congress_9_10_may_2015': (None, 'congress_session_dates'),
    'za_da_leader_maimane_elected_published_20150510': ('2015-05-10', 'result_publication'),
    'za_da_news_leader_maimane_20160425': ('2016-04-25', 'in_office_attestation'),
    'za_da_news_leader_maimane_20180407': ('2018-04-07', 'in_office_attestation'),
    'za_da_closing_speech_federal_leader_maimane_20180408': ('2018-04-08', 'in_office_attestation'),
    'za_da_statement_federal_leader_maimane_20191004': ('2019-10-04', 'in_office_attestation'),
    'za_da_fedex_informed_maimane_resignation_decision_20191024': ('2019-10-24', 'resignation_decision_reported'),
    'za_da_federal_leader_position_vacant_20191025': ('2019-10-25', 'vacancy_stated'),
    'za_da_fedco_interim_leader_election_scheduled_20191025': ('2019-10-25', 'interim_election_scheduled_prospective'),
    'za_da_fedex_congratulates_interim_leader_steenhuisen_20191123': ('2019-11-23', 'interim_election_reference'),
    'za_da_leader_steenhuisen_elected_published_20201101': ('2020-11-01', 'result_publication'),
    'za_da_statement_federal_leader_steenhuisen_20210310': ('2021-03-10', 'in_office_attestation'),
    'za_da_tribute_steenhuisen_six_years_federal_leader_20260204': ('2026-02-04', 'service_tribute_departure_reference'),
    'za_da_statement_leader_steenhuisen_20260304': ('2026-03-04', 'in_office_attestation'),
}
# Claims printed without a structured date: retrospective, undated, year-only and span-only claims.
UNDATED = (
    'za_da_dp_formed_by_merger_retrospective',
    'za_da_dp_leader_de_beer_retrospective',
    'za_da_dp_whos_who_dp_leader_leon_1999',
    'za_da_dp_whos_who_leon_elected_dp_leadership_1994',
    'za_da_formed_by_dp_nnp_fa_retrospective',
    'za_da_profile_leon_former_leader_2007',
    'za_da_profile_formation_under_leon_leadership',
    'za_da_sixth_federal_congress_9_10_may_2015',
)
NEW_SOURCES = list(RESPONSES)
# Holder observations (name, attested_on, from, until) in date order: twelve added and the two S10h rows (11 and 13).
HOLDERS = [
    ('Tony Leon', '2000-10-14', None, None),
    ('Tony Leon', '2000-11-22', None, None),
    ('Helen Zille', '2007-05-06', None, None),
    ('Helen Zille', '2010-10-15', None, None),
    ('Helen Zille', '2012-11-04', None, None),
    ('Helen Zille', '2014-05-09', None, None),
    ('Mmusi Maimane', '2016-04-25', None, None),
    ('Mmusi Maimane', '2018-04-07', None, None),
    ('Mmusi Maimane', '2018-04-08', None, None),
    ('Mmusi Maimane', '2019-10-04', None, '2019-10-23'),
    ('John Steenhuisen', '2021-03-10', None, None),
    ('John Steenhuisen', '2023-04-03', None, None),
    ('John Steenhuisen', '2026-03-04', None, None),
    ('Geordin Hill-Lewis', '2026-04-12', None, None),
]
HOLDER_CLAIMS = [
    ['za_da_speech_leader_leon_20001014'],
    ['za_da_speech_leader_leon_20001122'],
    ['za_da_zille_elected_leader_acceptance_20070506'],
    ['za_da_sa_today_leader_zille_20101015'],
    ['za_da_press_release_leader_zille_20121104'],
    ['za_da_speech_leader_zille_20140509'],
    ['za_da_news_leader_maimane_20160425'],
    ['za_da_news_leader_maimane_20180407'],
    ['za_da_closing_speech_federal_leader_maimane_20180408'],
    ['za_da_statement_federal_leader_maimane_20191004', 'za_da_federal_leader_position_vacant_20191025'],
    ['za_da_statement_federal_leader_steenhuisen_20210310'],
    ['za_da_steenhuisen_attested_20230403'],
    ['za_da_statement_leader_steenhuisen_20260304'],
    ['za_da_leader_elected_20260412'],
]
S10H_INDEXES = [11, 13]
NEW_INDEXES = [i for i in range(len(HOLDERS)) if i not in S10H_INDEXES]
HOLDER_REVIEW = {0: 'ZA-DA-03', 1: 'ZA-DA-03', 2: 'ZA-DA-04', 3: 'ZA-DA-05', 4: 'ZA-DA-05', 5: 'ZA-DA-05', 6: 'ZA-DA-07',
                 7: 'ZA-DA-07', 8: 'ZA-DA-07', 9: 'ZA-DA-07', 10: 'ZA-DA-09', 12: 'ZA-DA-10'}
# Ridge's ruling (a), the one stated end: a relative day in a dated party statement resolves from its dateline. The
# statement of Friday 25 October 2019 says the office "became vacant on Wednesday", that is 23 October 2019.
# Claim id -> (holder, the observation it ends, the statement's dateline, the weekday printed, until).
END_CLAIMS = {
    'za_da_federal_leader_position_vacant_20191025': ('Mmusi Maimane', '2019-10-04', '2019-10-25', 'Wednesday',
                                                      '2019-10-23'),
}
WEEKDAYS = ('Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday', 'Sunday')
# Role claims that must never feed a holder.
NEVER_HOLDER = (
    'za_da_profile_leon_former_leader_2007',
    'za_da_sixth_federal_congress_9_10_may_2015',
    'za_da_leader_maimane_elected_published_20150510',
    'za_da_fedex_informed_maimane_resignation_decision_20191024',
    'za_da_fedco_interim_leader_election_scheduled_20191025',
    'za_da_fedex_congratulates_interim_leader_steenhuisen_20191123',
    'za_da_leader_steenhuisen_elected_published_20201101',
    'za_da_tribute_steenhuisen_six_years_federal_leader_20260204',
)
# Claims about organizations (the DP, its leaders, the DA's formation and the DP/NNP alliance): cited by the
# organization observation with no role, never by the role.
ORGANIZATION_ONLY = (
    'za_da_dp_speech_leader_of_dp_leon_19970522',
    'za_da_dp_formed_by_merger_retrospective',
    'za_da_dp_leader_de_beer_retrospective',
    'za_da_dp_whos_who_dp_leader_leon_1999',
    'za_da_dp_whos_who_leon_elected_dp_leadership_1994',
    'za_da_speech_dp_and_nnp_led_into_da_20001014',
    'za_da_formed_by_dp_nnp_fa_retrospective',
    'za_da_profile_formation_under_leon_leadership',
)
DP_CLAIMS = ORGANIZATION_ONLY[:5]
INTERIM_CLAIMS = ('za_da_fedco_interim_leader_election_scheduled_20191025',
                  'za_da_fedex_congratulates_interim_leader_steenhuisen_20191123')
REVIEW = [f'ZA-DA-{n:02d}' for n in range(1, 11)]
# Days that must never be any holder's observation, start or end: result publications, congress spans, the dates of
# the resignation and vacancy statements, the interim election and its reporting, the tribute, the DP's dated record and
# formation day, and lead days (the DA's reported launch). Zille's acceptance day (ruling (b)) is an observation day
# only, and the resolved Wednesday (ruling (a)) is Maimane's end only.
NEVER_HOLDER_DATE = {'1989-04-08', '1997-05-22', '2000-06-24', '2015-05-09', '2015-05-10', '2019-10-24', '2019-10-25',
                     '2019-11-17', '2019-11-23', '2020-10-31', '2020-11-01', '2026-02-04'}
ONLY_OBSERVATION_DATE = {'2007-05-06'}
ONLY_END_DATE = {'2019-10-23'}
HOLDER_KINDS = {'in_office_attestation'}
END_KINDS = {'vacancy_stated'}
BOUNDARY_KINDS = {'assumption_of_office', 'end_of_term_statement', 'resignation_effective', 'oath_of_office'}
# Holder names and the surname each cited claim prints.
PEOPLE = {'Tony Leon': 'Leon', 'Helen Zille': 'Zille', 'Mmusi Maimane': 'Maimane', 'John Steenhuisen': 'Steenhuisen',
          'Geordin Hill-Lewis': 'Hill-Lewis'}
# People the packet names as leaders of the DA or the DP in any row: six (at most ten).
NAMED_LEADERS = set(PEOPLE) | {'Zach de Beer'}
DP_URL = re.compile(r'^https://web\.archive\.org/web/(\d{14})id_/(http://www\.dp\.org\.za(?::80)?/.*)$')
DA_URL = re.compile(r'^https://web\.archive\.org/web/(\d{14})id_/(https?://(?:www\.)?da\.org\.za(?::80)?/.*)$')
HOSTS = {'www.dp.org.za', 'www.da.org.za', 'da.org.za'}
LEAD_URL_MARKERS = ('sahistory', 'wikipedia', 'britannica', 'news24', 'iol.co.za', 'timeslive', 'dailymaverick',
                    'ewn.co.za', 'sabcnews', 'politicsweb', 'polity.org', 'reuters', 'bbc.', 'mg.co.za', 'citizen.co.za',
                    'x.com/', 'twitter.com')
PER_REQUEST_URL = re.compile(r'wp-json|cdx/search|cdn-cgi|email-protection|nocache|cachebust|[?&]s=|[?&]_=|[?&]cb=|'
                             r'/search[/?]|/feed|/embed|/page/\d|/tag/|/category/|download\.php|rnd=|/api/|'
                             r'SpeechArchive|NewsLetterArchive|list_press|archives_statements', re.I)
DECISIONS = {
    'ZA-DA-01': 'Claims only', 'ZA-DA-02': 'Claims only', 'ZA-DA-03': 'Accepted', 'ZA-DA-04': 'Accepted in part',
    'ZA-DA-05': 'Accepted', 'ZA-DA-06': 'Accepted in part', 'ZA-DA-07': 'Accepted', 'ZA-DA-08': 'Accepted in part',
    'ZA-DA-09': 'Accepted in part', 'ZA-DA-10': 'Accepted in part',
}
REPORT = research.RESEARCH / 'south-africa-da-federal-leaders-2000-2026-39.md'
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-39.md'


def da_invariants(packet):
    """Packet-level rules this test owns; raises AssertionError, KeyError, IndexError or ValueError on any violation."""
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    claim_source = {c['id']: s['id'] for s in packet['sources'] for c in s['claims']}
    sources = {s['id']: s for s in packet['sources']}
    orgs = {o['id']: o for o in packet['organizations']}
    new_claims = [c['id'] for sid in NEW_SOURCES for c in sources[sid]['claims']]
    assert len(packet['institutions']) == 1 and packet['institutions'][0]['id'] == 'za_presidency'
    # Every other role is unchanged in count, and the role map is exact.
    roles = {r['id']: r for e in packet['organizations'] + packet['institutions'] for r in e['roles']}
    assert {rid: len(r['holder_claims']) for rid, r in roles.items() if rid in OTHER_ROLE_HOLDER_COUNTS} == \
        OTHER_ROLE_HOLDER_COUNTS
    assert {o['id']: [r['id'] for r in o['roles']] for o in packet['organizations'] if o['roles']} == ROLE_MAP
    org = orgs[OBS]
    # The IEC reporting identity, unknown lifecycle and empty game mapping stay unchanged.
    assert (org['name'], org['kind']) == (NAME, 'national_ballot_party_reporting_identity')
    assert org['source_identifier']['value'] == ROW
    assert org['represented_party_ids'] == [] and org['reconciled_organization_id'] is None
    assert org['lifecycle'] == {'status': 'unknown', 'from': None, 'until': None, 'note': LIFECYCLE_NOTE}
    role = org['roles'][0]
    assert (role['id'], role['title'], role['kind']) == (ROLE, TITLE, 'party_leader')
    for index, (rid, title, name) in enumerate(CHAIR_ROLES, start=1):
        chair = org['roles'][index]
        assert (chair['id'], chair['title'], chair['kind']) == (rid, title, 'party_chair')
        assert [h['name'] for h in chair['holder_claims']] == [name]
        assert not set(chair['claim_ids']) & set(new_claims) and not set(chair['sources']) & set(NEW_SOURCES)
    holders = role['holder_claims']
    assert all(isinstance(h, dict) for h in holders)
    # The S10h holders are unchanged, object for object.
    assert [holders[i] for i in S10H_INDEXES] == S10H_HOLDERS
    for index, h in enumerate(holders):
        assert 'acting' not in h['name'].lower() and 'interim' not in h['name'].lower(), h['name']
        assert h['from'] is None, h['name']
        assert not {h['attested_on'], h['from'], h['until']} & NEVER_HOLDER_DATE, h['name']
        assert h['attested_on'] not in ONLY_END_DATE and h['until'] not in ONLY_OBSERVATION_DATE, h['name']
        assert not set(h['claim_ids']) & set(NEVER_HOLDER), h['name']
        assert not set(h['claim_ids']) & set(ORGANIZATION_ONLY), h['name']
        # An end only where a cited claim states it (ruling (a)): the printed weekday, resolved from the dateline.
        ends = [cid for cid in h['claim_ids'] if cid in END_CLAIMS]
        assert len(ends) == (0 if h['until'] is None else 1), h['name']
        for cid in ends:
            name, observed, dateline, weekday, until = END_CLAIMS[cid]
            assert (h['name'], h['attested_on'], h['until']) == (name, observed, until), cid
            assert claims[cid]['attested_on'] == dateline and f'on {weekday}' in claims[cid]['text'], cid
            day = date.fromisoformat(dateline)
            back = (day.weekday() - WEEKDAYS.index(weekday)) % 7
            assert back and (day - timedelta(days=back)).isoformat() == until, cid
            assert h['attested_on'] < until < dateline, cid
        assert h['claim_ids'][0] not in END_CLAIMS, h['name']
        expected = []
        for cid in h['claim_ids']:
            assert cid in role['claim_ids'] and cid.startswith(PREFIX), cid
            if cid not in END_CLAIMS:
                assert claims[cid]['attested_on'] == h['attested_on'], cid
            if claim_source[cid] not in expected:
                expected.append(claim_source[cid])
        assert h['sources'] == expected, h['name']
        if index in NEW_INDEXES:
            assert set(h['sources']) <= set(NEW_SOURCES), h['name']
    # The exact holder list, checked after the rules above so that each rule is exercised on its own.
    assert [(h['name'], h['attested_on'], h['from'], h['until']) for h in holders] == HOLDERS
    assert [h['claim_ids'] for h in holders] == HOLDER_CLAIMS
    assert [h['attested_on'] for h in holders] == sorted(h['attested_on'] for h in holders)
    # Organization claims (the DP, the DA's formation, the alliance) sit on the observation, never on the role.
    assert not set(role['claim_ids']) & set(ORGANIZATION_ONLY)
    assert set(ORGANIZATION_ONLY) <= set(org['claim_ids'])
    # The role's and the organization's lists keep their S10h entries first, then this packet's, in order.
    role_new = [cid for cid in new_claims if cid not in ORGANIZATION_ONLY]
    assert role['claim_ids'] == S10H_ROLE_CLAIMS + role_new
    assert role['sources'] == S10H_ROLE_SOURCES + [sid for sid in NEW_SOURCES
                                                   if {c['id'] for c in sources[sid]['claims']} - set(ORGANIZATION_ONLY)]
    assert org['claim_ids'] == S10H_ORG_CLAIMS + new_claims
    assert org['sources'] == S10H_SOURCES + NEW_SOURCES
    role_claims = set(role['claim_ids']) | {c for h in holders for c in h['claim_ids']}
    role_sources = set(role['sources']) | {s for h in holders for s in h['sources']}
    assert all(claim_source[c] in role_sources for c in role_claims)
    assert set(role['claim_ids']) <= set(org['claim_ids']) and set(role['sources']) <= set(org['sources'])
    # Party office and state office, and every other role, never feed each other.
    da_ids = set(org['claim_ids']) - set(S10H_ORG_CLAIMS[:2])
    da_srcs = set(org['sources']) - set(S10H_SOURCES[:2])
    assert all(c.startswith(PREFIX) for c in da_ids) and all(s.startswith(PREFIX) for s in da_srcs)
    for entry in packet['organizations'] + packet['institutions']:
        if entry is org:
            continue
        entry_claims = set(entry['claim_ids']) | {c for r in entry['roles'] for c in r['claim_ids']} | {
            c for r in entry['roles'] for h in r['holder_claims'] for c in h['claim_ids']}
        entry_sources = set(entry['sources']) | {s for r in entry['roles'] for s in r['sources']} | {
            s for r in entry['roles'] for h in r['holder_claims'] for s in h['sources']}
        assert not entry_claims & da_ids and not entry_sources & da_srcs, entry['id']
        assert not any(c.startswith(PREFIX) for c in entry_claims | entry_sources), entry['id']
        assert not any(r['id'] == ROLE for r in entry['roles']), entry['id']
    # Undated claims carry no structured date at all, and distinct dated events keep their own day.
    for cid in UNDATED:
        assert not {'attested_on', 'attested_period', 'period'} & set(claims[cid]), cid
    for cid, (day, _kind) in EVENTS.items():
        assert claims[cid].get('attested_on') == day, cid


class SouthAfricaDaFederalLeadersTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.raw = (research.ROOT / research.RESEARCH / 'south-africa.json').read_text(encoding='utf-8')
        cls.packet = json.loads(cls.raw)
        cls.sources = {s['id']: s for s in cls.packet['sources']}
        cls.claims = {c['id']: c for s in cls.packet['sources'] for c in s['claims']}
        cls.claim_source = {c['id']: s['id'] for s in cls.packet['sources'] for c in s['claims']}
        cls.orgs = {o['id']: o for o in cls.packet['organizations']}
        cls.org = cls.orgs[OBS]
        cls.role = cls.org['roles'][0]
        cls.extracts = {sid: json.loads((research.ROOT / cls.sources[sid]['snapshot']['path']).read_text(encoding='utf-8'))
                        for sid in NEW_SOURCES}
        cls.rows = {row['claim_id']: row for sid in NEW_SOURCES for row in cls.extracts[sid]['rows']}
        cls.new_claims = [c['id'] for sid in NEW_SOURCES for c in cls.sources[sid]['claims']]
        cls.report = (research.ROOT / REPORT).read_text(encoding='utf-8')

    def validate(self, packet=None):
        return research.validate(packet or self.packet, research.ROOT, {'SouthAfrica'}, {'SouthAfrica': set()})

    def section(self, heading):
        parts = self.report.split(f'\n## {heading}', 1)
        self.assertEqual(len(parts), 2, heading)
        return parts[1].split('\n## ', 1)[0]

    def test_new_records_are_bounded_and_every_claim_is_classified(self):
        ids = self.validate()
        self.assertEqual((len(NEW_SOURCES), len(self.new_claims)), (24, 29))
        order = [s['id'] for s in self.packet['sources']]
        self.assertEqual(len(order), EARLIER_SOURCE_COUNT + len(NEW_SOURCES))
        self.assertEqual(order[EARLIER_SOURCE_COUNT:], NEW_SOURCES)
        self.assertEqual([sid for sid in order[:EARLIER_SOURCE_COUNT] if sid.startswith(PREFIX)], S10H_ROLE_SOURCES)
        self.assertEqual((len(ids['entries']), len(ids['roles'])), (53, 11))
        self.assertEqual(list(EVENTS), self.new_claims)
        # Every new claim is exactly one of: a holder observation's claim, a holder's end claim, a role claim that
        # never feeds a holder, or an organization-only claim.
        holder_claims = [HOLDER_CLAIMS[i][0] for i in NEW_INDEXES]
        end_claims = [cid for i in NEW_INDEXES for cid in HOLDER_CLAIMS[i][1:]]
        groups = holder_claims + end_claims + list(NEVER_HOLDER) + list(ORGANIZATION_ONLY)
        self.assertEqual(len(groups), len(set(groups)))
        self.assertEqual(set(groups), set(self.new_claims))
        self.assertEqual(end_claims, list(END_CLAIMS))
        self.assertEqual((len(holder_claims), len(end_claims), len(NEVER_HOLDER), len(ORGANIZATION_ONLY)),
                         (12, 1, 8, 8))
        observations = re.findall(r'^### (ZA-DA-\d\d)\b', self.report, re.M)
        self.assertEqual(observations, REVIEW)
        self.assertEqual({row['review_observation'] for row in self.rows.values()}, set(REVIEW))
        self.assertEqual({i: self.rows[HOLDER_CLAIMS[i][0]]['review_observation'] for i in NEW_INDEXES}, HOLDER_REVIEW)
        # At most ten people named as leaders of the DA or the DP; the DA's holders are five.
        self.assertEqual({h[0] for h in HOLDERS}, set(PEOPLE))
        self.assertEqual({row['holder_name'] for row in self.rows.values()} - {None}, NAMED_LEADERS - {'Geordin Hill-Lewis'})
        self.assertLessEqual(len(NAMED_LEADERS), 10)
        self.assertNotIn('"name":', json.dumps([self.extracts[sid]['rows'] for sid in NEW_SOURCES]))

    def test_holders_are_exactly_as_intended(self):
        da_invariants(self.packet)
        for index in NEW_INDEXES:
            holder = self.role['holder_claims'][index]
            self.assertTrue(holder['note'].startswith('Observed on '), holder['name'])
            self.assertRegex(holder['uncertainty'], r'^No start', holder['name'])
            if holder['until'] is None:
                self.assertIn('No end', holder['uncertainty'], holder['name'])
            else:
                self.assertIn('Until 23 October 2019: ', holder['note'])
                self.assertIn('resolves from its dateline', holder['note'])
                self.assertIn("Ridge's ruling", holder['uncertainty'])
            for cid in holder['claim_ids']:
                row = self.rows[cid]
                self.assertIn(PEOPLE[holder['name']].lower(), self.claims[cid]['text'].lower(), cid)
                self.assertEqual((row['holder_name'], row['role_id'], row['role_title']), (holder['name'], ROLE, TITLE), cid)
                if cid in END_CLAIMS:
                    self.assertIn(row['event_kind'], END_KINDS, cid)
                    self.assertIn('Gives the end of the holder observation', self.claims[cid]['uncertainty'], cid)
                else:
                    self.assertIn(row['event_kind'], HOLDER_KINDS, cid)
                    self.assertIn('Dates the holder observation', self.claims[cid]['uncertainty'], cid)
        # Ruling (b): Zille's own acceptance speech, addressed to the DA's congress, dates an observation, never a start.
        zille = self.role['holder_claims'][2]
        self.assertIn('"your leader" to the DA\'s congress', zille['note'])
        self.assertIn('988d5379', zille['note'])
        for cid in NEVER_HOLDER:
            self.assertNotIn(self.rows[cid]['event_kind'], HOLDER_KINDS | BOUNDARY_KINDS, cid)
            self.assertEqual(self.rows[cid]['role_id'], ROLE, cid)
            self.assertRegex(self.claims[cid]['uncertainty'], r'Never a holder date', cid)
        for cid in ORGANIZATION_ONLY:
            row = self.rows[cid]
            self.assertIsNone(row['role_id'], cid)
            self.assertEqual(row['role_title'], DP_LEADER if row['holder_name'] else None, cid)
            self.assertRegex(self.claims[cid]['uncertainty'], r'Never a holder date', cid)
        for cid in DP_CLAIMS:
            self.assertIn('DP', self.claims[cid]['text'], cid)
            self.assertRegex(self.claims[cid]['uncertainty'], r'another organization|organization claim', cid)
        self.assertEqual({self.rows[cid]['holder_name'] for cid in DP_CLAIMS} - {None}, {'Tony Leon', 'Zach de Beer'})
        # Interim service keeps its own title and never feeds a holder; every other role row carries the role title.
        for cid in self.new_claims:
            if cid in ORGANIZATION_ONLY:
                continue
            self.assertEqual(self.rows[cid]['role_title'], INTERIM if cid in INTERIM_CLAIMS else TITLE, cid)
        for cid in INTERIM_CLAIMS:
            self.assertIn('Interim', self.claims[cid]['text'], cid)
            self.assertIn('nterim service is claims only', self.claims[cid]['uncertainty'], cid)
        kinds = {self.rows[cid]['event_kind'] for cid in NEVER_HOLDER}
        for kind in ('result_publication', 'congress_session_dates', 'resignation_decision_reported',
                     'interim_election_scheduled_prospective', 'interim_election_reference', 'former_holder_styled',
                     'service_tribute_departure_reference'):
            self.assertIn(kind, kinds, kind)
        # Ruling (b) re-kinds Zille's acceptance row as the dated observation it supports (as Codex's 988d5379 did for
        # CLAUDE-C01-31); ruling (a) keeps the vacancy row a vacancy statement, which gives the end.
        self.assertEqual(self.rows['za_da_zille_elected_leader_acceptance_20070506']['event_kind'], 'in_office_attestation')
        self.assertEqual({self.rows[cid]['event_kind'] for cid in END_CLAIMS}, END_KINDS)
        self.assertFalse(kinds & (HOLDER_KINDS | END_KINDS))

    def test_no_start_or_end_is_stated_or_inferred(self):
        claims = self.claims
        for cid in UNDATED:
            self.assertIn('no structured date', claims[cid]['uncertainty'].lower(), cid)
            self.assertTrue(self.rows[cid]['printed_range'], cid)
        # The October 2019 weekday statements (ruling (a)): the vacancy gives the end, resolved from its dateline, and
        # the resignation decision fixes the week but is never an end.
        for cid in ('za_da_fedex_informed_maimane_resignation_decision_20191024',
                    'za_da_federal_leader_position_vacant_20191025'):
            self.assertIn('on Wednesday', claims[cid]['text'].replace('On Wednesday', 'on Wednesday'), cid)
            self.assertIn("Ridge's ruling", claims[cid]['uncertainty'], cid)
            self.assertIn('convened on Thursday morning', claims[cid]['uncertainty'], cid)
        self.assertIn('resolves from its dateline', claims['za_da_federal_leader_position_vacant_20191025']['uncertainty'])
        # Evidence recorded but never used as a boundary, each saying why.
        self.assertIn('never an end', claims['za_da_fedex_informed_maimane_resignation_decision_20191024']['uncertainty'])
        self.assertIn('never an end', claims['za_da_tribute_steenhuisen_six_years_federal_leader_20260204']['uncertainty'])
        self.assertIn('never an end', claims['za_da_profile_leon_former_leader_2007']['uncertainty'])
        self.assertIn('never a start', claims['za_da_zille_elected_leader_acceptance_20070506']['uncertainty'])
        for cid in ('za_da_leader_maimane_elected_published_20150510', 'za_da_leader_steenhuisen_elected_published_20201101'):
            self.assertIn('never a start', claims[cid]['uncertainty'], cid)
        self.assertEqual(self.rows['za_da_dp_formed_by_merger_retrospective']['printed_range'],
                         '"April 8, 1989" in an undated retrospective history (before the 1990 coverage period)')
        # State, parliamentary and caucus offices in the same documents never feed the party office.
        for cid, phrase in (
                ('za_da_speech_leader_leon_20001014', 'state office'),
                ('za_da_zille_elected_leader_acceptance_20070506', 'Leader of the Official Opposition'),
                ('za_da_news_leader_maimane_20160425', 'Parliamentary Leader of the Democratic Alliance'),
                ('za_da_press_release_leader_zille_20121104', 'no za_presidency claim is fed'),
                ('za_da_tribute_steenhuisen_six_years_federal_leader_20260204', 'state office')):
            self.assertIn(phrase, claims[cid]['uncertainty'], cid)
        scope = self.role['scope_note']
        for phrase in ('no za_presidency claim or source feeds it', 'Do not fill the interval', 'interim service',
                       "infer an outgoing holder's last day", 'so no added holder has a start, and one has an end',
                       'The S10h intake\'s two discrete attestations (2023-04-03 and 2026-04-12) are unchanged',
                       'Democratic Party'):
            self.assertIn(phrase, scope)
        self.assertEqual(self.org['coverage']['unresolved'][:3], [
            'Reconcile legal registration, precise organization identity, renames, alliances, mergers and splits against original records.',
            'Research all independently dated party, parliamentary and executive leadership roles before character or succession eligibility.',
            'No game-party mapping is asserted. An electoral entry is not automatically a parliamentary caucus, coalition or umbrella affiliation.'])
        self.assertEqual(len(self.org['coverage']['unresolved']), 4)
        self.assertIn('(CLAUDE-C01-39, ', self.org['coverage']['unresolved'][3])
        self.assertIn('unknown lifecycle and empty game mapping are unchanged', self.org['coverage']['unresolved'][3])
        # The packet note sits after the CLAUDE-C01-09, -30 and -32 notes and before the CLAUDE-C01-21 and CLAUDE-C01-16
        # notes, which their own tests pin as the last two entries.
        unresolved = self.packet['coverage']['unresolved']
        self.assertEqual(sum('CLAUDE-C01-39' in u for u in unresolved), 1)
        self.assertTrue(unresolved[-6].startswith('Heads of state 1990-2024 (CLAUDE-C01-09'))
        self.assertTrue(unresolved[-5].startswith('ACDP, Freedom Front and IFP leaders 1990-2026 (CLAUDE-C01-30'))
        self.assertTrue(unresolved[-4].startswith('PAC Presidents 1990-2026 (CLAUDE-C01-32'))
        self.assertTrue(unresolved[-3].startswith('DA Federal Leaders 2000-2026 (CLAUDE-C01-39'))
        self.assertTrue(unresolved[-2].startswith('Deputy Presidents 1994-2026 (CLAUDE-C01-21'))
        self.assertTrue(unresolved[-1].startswith('ANC Presidents 1990-2026 (CLAUDE-C01-16'))

    def test_extracts_match_packet_claims_and_record_original_responses(self):
        for sid in NEW_SOURCES:
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual(extract['format'], 'spheres-c01-derived-factual-table/v1')
            self.assertEqual((extract['source_id'], extract['source_url'], extract['original_url']),
                             (sid, source['url'], source['original_url']))
            for key in ('scope_note', 'access_method', 'published_date', 'rights_note'):
                self.assertEqual(extract[key], source[key], (sid, key))
            self.assertEqual((extract['accessed_date'], source['accessed_date']), ('2026-09-30', '2026-09-30'))
            self.assertLessEqual(source['published_date'] or '', research.CUTOFF)
            self.assertFalse(extract['source_response_checked_in'])
            self.assertEqual((extract['source_response_bytes'], extract['source_response_sha256']), RESPONSES[sid])
            self.assertRegex(extract['source_response_sha1_base32'], r'^[A-Z2-7]{32}$')
            self.assertIn('no Accept-Encoding request header and no automatic decoding', extract['fetch_recipe'])
            self.assertEqual(extract['source_response_content_encoding'], 'identity')
            self.assertNotIn('decoded_response_sha256', extract)
            self.assertIn('without content encoding', source['scope_note'])
            self.assertIn('downloaded again', extract['stability_check'])
            self.assertIn('at least 30 minutes after the first', extract['stability_check'])
            self.assertIn("equals the Internet Archive's CDX digest", extract['stability_check'])
            self.assertIn('not checked into this repository', extract['provenance_note'])
            self.assertIn('derived factual extract', extract['provenance_note'])
            self.assertIn('Raw Internet Archive capture', extract['provenance_note'])
            self.assertIn('No open license, portrait permission or likeness approval', extract['rights_note'])
            self.assertEqual(extract['visual_review']['pdf_pages_one_based'], [])
            self.assertTrue(source['source_type'] and source['scope_note'] and source['publisher'])
            self.assertNotEqual(source['source_type'], 'party_republished_news_text', sid)
            snapshot = source['snapshot']
            self.assertTrue(snapshot['path'].startswith('docs/campaign-certification/C01/research/sources/south-africa-d'))
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
                self.assertEqual(row['observation_id'], OBS)
                self.assertIn(row['role_id'], (ROLE, None))
                self.assertNotIn('name', row)
                self.assertIn(row['review_observation'], REVIEW)
                self.assertEqual('printed_range' in row, row['attested_on'] is None)
            # Every source is a raw Internet Archive capture of the DA's or the DP's own pages, made before the cutoff.
            match = DA_URL.match(source['url']) or DP_URL.match(source['url'])
            self.assertTrue(match, sid)
            stamp = match.group(1)
            self.assertEqual(stamp, ARCHIVED[sid])
            self.assertLess(stamp, '20260907')
            self.assertEqual(extract['archive_capture_utc'].replace('-', '').replace(':', '').replace('T', '').rstrip('Z'),
                             stamp)
            self.assertEqual(source['original_url'], match.group(2).replace(':80/', '/'))
            self.assertNotIn(':80', source['original_url'])
            host = urlsplit(source['original_url']).hostname
            self.assertIn(host, HOSTS, sid)
            dp_rows = [r for r in extract['rows'] if r['claim_id'] in DP_CLAIMS]
            self.assertEqual(host == 'www.dp.org.za', bool(dp_rows), sid)
        self.assertEqual({cid: (row['attested_on'], row['event_kind']) for cid, row in self.rows.items()}, EVENTS)
        self.assertEqual({cid for cid, row in self.rows.items() if row['attested_on'] is None}, set(UNDATED))
        # The 2014 byline claim prints the source's curly apostrophe.
        self.assertTrue(self.claims['za_da_speech_leader_zille_20140509']['text'].startswith(
            'The DA speech item "DA\u2019s growth is a victory for all South Africans"'))

    def test_secondary_leads_and_per_request_pages_stay_out_of_the_packet(self):
        for sid in NEW_SOURCES:
            url = self.sources[sid]['url']
            self.assertEqual(urlsplit(url).hostname, 'web.archive.org', sid)
            self.assertIsNone(PER_REQUEST_URL.search(url), sid)
            for marker in LEAD_URL_MARKERS:
                self.assertNotIn(marker, url, sid)
        lowered = json.dumps([self.sources[sid] for sid in NEW_SOURCES], ensure_ascii=False).lower()
        for marker in ('wikipedia', 'sahistory', 'britannica', 'news24', 'iol.co.za', 'dailymaverick', 'politicsweb'):
            self.assertNotIn(marker, lowered, marker)
        leads, added = self.section('Leads not imported'), self.section('Sources added')
        for marker in ('wikipedia.org', 'Speech.asp?ID=393', 'Speech.asp?ID=1190', 'print_satoday.asp',
                       'congress/congress.asp.htm', 'old/leader.htm', 'da-pleased-to-announce-john-steenhuisen'):
            self.assertIn(marker, leads, marker)
            self.assertNotIn(marker, added, marker)

    def test_packet_formatting_is_preserved(self):
        data = (research.ROOT / research.RESEARCH / 'south-africa.json').read_bytes()
        self.assertNotIn(b'\r', data)
        self.assertEqual(data.decode('utf-8'), json.dumps(self.packet, indent=2, ensure_ascii=False) + '\n')

    def test_mutations_are_rejected(self):
        def mutated(change):
            packet = copy.deepcopy(self.packet)
            change(packet)
            return packet

        def source(packet, sid):
            return next(s for s in packet['sources'] if s['id'] == sid)

        def claim(packet, cid):
            return next(c for s in packet['sources'] for c in s['claims'] if c['id'] == cid)

        def org(packet, oid=OBS):
            return next(o for o in packet['organizations'] if o['id'] == oid)

        def role(packet, oid=OBS):
            return org(packet, oid)['roles'][0]

        def holder(packet, index):
            return role(packet)['holder_claims'][index]

        def president_role(packet):
            return packet['institutions'][0]['roles'][0]

        def added(name, day, sid, cid):
            return {'name': name, 'attested_on': day, 'from': None, 'until': None, 'sources': [sid], 'claim_ids': [cid],
                    'note': f'Observed on {day}', 'uncertainty': 'No start. No end.'}

        validator_cases = [
            (lambda p: source(p, 'za_da_statement_condolences_cope_20260304')['snapshot'].update(sha256='0' * 64),
             'checksum mismatch'),
            (lambda p: source(p, 'za_da_dp_party_info_page_2000')['snapshot'].update(bytes=1), 'checksum mismatch'),
            (lambda p: holder(p, 12).update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: claim(p, 'za_da_statement_leader_steenhuisen_20260304').update(attested_on='2026-09-08'),
             'exceeds cutoff'),
            (lambda p: holder(p, 10)['claim_ids'].append('za_da_leader_steenhuisen_elected_published_20201101'),
             'cited source'),
            (lambda p: role(p)['claim_ids'].append('za_da_does_not_exist'), 'Unknown'),
            (lambda p: org(p).update(represented_party_ids=['SouthAfrica/za_dp']), 'foreign represented party'),
        ]
        for change, message in validator_cases:
            with self.subTest(message=message), self.assertRaisesRegex(ValueError, message):
                self.validate(mutated(change))
        invariant_cases = [
            ('successor election used as an end (Leon)', lambda p: holder(p, 1).update(until='2007-05-06')),
            ('successor election used as an end (Zille)', lambda p: holder(p, 5).update(until='2015-05-10')),
            ('vacancy statement date used as the end (Maimane)', lambda p: holder(p, 9).update(until='2019-10-25')),
            ('resignation statement used as an end (Maimane)', lambda p: holder(p, 9).update(until='2019-10-24')),
            ('vacancy weekday resolved to the wrong week (Maimane)', lambda p: holder(p, 9).update(until='2019-10-16')),
            ('vacancy end dropped (Maimane)', lambda p: holder(p, 9).update(until=None)),
            ('vacancy end kept without its claim (Maimane)', lambda p: (
                holder(p, 9)['claim_ids'].pop(), holder(p, 9)['sources'].pop())),
            ('vacancy end moved to an earlier observation (Maimane 2018)', lambda p: (
                holder(p, 8).update(until='2019-10-23'), holder(p, 9).update(until=None))),
            ('vacancy weekday used as an observation (Maimane)', lambda p: holder(p, 9).update(attested_on='2019-10-23')),
            ('tribute used as an end (Steenhuisen)', lambda p: holder(p, 12).update(until='2026-02-04')),
            ('successor election used as an end (Steenhuisen)', lambda p: holder(p, 12).update(until='2026-04-12')),
            ('acceptance used as a start (Zille 2007)', lambda p: holder(p, 2).update({'from': '2007-05-06'})),
            ('acceptance day used as an end (Zille 2007)', lambda p: holder(p, 2).update(until='2007-05-06')),
            ('acceptance observation dropped (Zille 2007)', lambda p: role(p)['holder_claims'].pop(2)),
            ('acceptance cited by a later holder (Zille 2010)', lambda p: (
                holder(p, 3)['claim_ids'].append('za_da_zille_elected_leader_acceptance_20070506'),
                holder(p, 3)['sources'].append('za_da_zille_acceptance_speech_20070506'))),
            ('election used as a start (Zille 2010)', lambda p: holder(p, 3).update({'from': '2007-05-06'})),
            ('result publication used as a start (Maimane)', lambda p: holder(p, 6).update({'from': '2015-05-10'})),
            ('result publication used as a start (Steenhuisen)', lambda p: holder(p, 10).update({'from': '2020-11-01'})),
            ('interim election used as a start (Steenhuisen)', lambda p: holder(p, 10).update({'from': '2019-11-17'})),
            ('in-office observation used as a start (Leon)', lambda p: holder(p, 0).update({'from': '2000-10-14'})),
            ('result publication used as observation (Maimane)', lambda p: holder(p, 6).update(attested_on='2015-05-10')),
            ('interim leader added as a holder', lambda p: role(p)['holder_claims'].insert(10, added(
                'John Steenhuisen', '2019-11-23', 'za_da_fedex_statement_interim_leadership_20191123',
                'za_da_fedex_congratulates_interim_leader_steenhuisen_20191123'))),
            ('DP leader added as a DA holder (Leon 1997)', lambda p: role(p)['holder_claims'].insert(0, added(
                'Tony Leon', '1997-05-22', 'za_da_dp_speech_taalbeleid_19970522',
                'za_da_dp_speech_leader_of_dp_leon_19970522'))),
            ('DP claim moved onto the role', lambda p: role(p)['claim_ids'].append(
                'za_da_dp_speech_leader_of_dp_leon_19970522')),
            ('formation claim moved onto the role', lambda p: role(p)['claim_ids'].append(
                'za_da_speech_dp_and_nnp_led_into_da_20001014')),
            ('tribute cited by a holder (Steenhuisen 2026)', lambda p: (
                holder(p, 12)['claim_ids'].append('za_da_tribute_steenhuisen_six_years_federal_leader_20260204'),
                holder(p, 12)['sources'].append('za_da_statement_thanks_steenhuisen_20260204'))),
            ('S10h holder changed (Steenhuisen 2023)', lambda p: holder(p, 11).update(note='Observed on 3 April 2023')),
            ('S10h holder given an end (Steenhuisen 2023)', lambda p: holder(p, 11).update(until='2026-04-12')),
            ('S10h holder removed (Hill-Lewis)', lambda p: role(p)['holder_claims'].pop()),
            ('holder order changed', lambda p: role(p)['holder_claims'].reverse()),
            ('formation day used as a lifecycle start', lambda p: org(p)['lifecycle'].update({'from': '2000-06-24'})),
            ('DP identity merged by name', lambda p: org(p).update(reconciled_organization_id='SouthAfrica/za_dp')),
            ('DP formation given a date', lambda p: claim(p, 'za_da_dp_formed_by_merger_retrospective').update(
                attested_on='1989-04-08')),
            ('congress span given a date', lambda p: claim(p, 'za_da_sixth_federal_congress_9_10_may_2015').update(
                attested_on='2015-05-09')),
            ('DA claim fed into the PAC role', lambda p: (
                role(p, 'za_iec_n2024_039')['claim_ids'].append('za_da_statement_leader_steenhuisen_20260304'),
                role(p, 'za_iec_n2024_039')['sources'].append('za_da_statement_condolences_cope_20260304'))),
            ('state-office claim fed into the DA role', lambda p: (
                role(p)['claim_ids'].append('za_ramaphosa_president_elect_20240614'),
                role(p)['sources'].append('za_parliament_president_elect_20240614'))),
            ('presidency holder added to the DA role', lambda p: role(p)['holder_claims'].append(
                copy.deepcopy(president_role(p)['holder_claims'][8]))),
            ('DA holder added to the Presidency', lambda p: president_role(p)['holder_claims'].append(
                copy.deepcopy(holder(p, 12)))),
            ('DA claim fed into the Federal Chairperson role', lambda p: (
                org(p)['roles'][1]['claim_ids'].append('za_da_federal_leader_position_vacant_20191025'),
                org(p)['roles'][1]['sources'].append('za_da_fedcouncil_chair_interim_election_20191025'))),
            ('second DA leader role', lambda p: org(p)['roles'].append(dict(copy.deepcopy(role(p)), id='za_da_interim_leader'))),
            ('ACDP holder changed', lambda p: role(p, 'za_iec_n2024_008')['holder_claims'].pop()),
        ]
        da_invariants(self.packet)
        for label, change in invariant_cases:
            with self.subTest(label=label), self.assertRaises((AssertionError, KeyError, IndexError, ValueError)):
                da_invariants(mutated(change))

    def test_report_and_handoff_close_no_parent_gate(self):
        self.assertIn('ready_for_review', self.report)
        table = self.section('Outcome')
        for review, decision in DECISIONS.items():
            row, = [line for line in table.splitlines() if line.startswith(f'| {review} ')]
            self.assertIn(f'**{decision}:**', row)
        self.assertEqual(set(DECISIONS), set(REVIEW))
        for marker in ('C01', 'C06', 'S23', 'WC1', 'CP1'):
            self.assertNotRegex(self.report, rf'\b{marker}\b[^.\n]*\bis (now )?complete\b')
        # Ridge's rulings (a)-(e) are recorded as decisions; Codex may still decide otherwise at integration.
        self.assertNotIn('ruling question', self.report)
        rulings = self.section('Rulings recorded (decided by Ridge)')
        for marker in ('(a)', '(b)', '(c)', '(d)', '(e)', 'Codex may still decide otherwise', '2019-10-23',
                       'za_da_federal_leader_position_vacant_20191025', '988d5379', 'C01-SouthAfrica-DP-001',
                       'SouthAfrica/za_dp'):
            self.assertIn(marker, rulings, marker)
        notes = self.section('Integration notes')
        for text in ('03cd0bb9', '02d2c5a2', 'not stacked', 'research-index.json', 'continue existing claims first',
                     'test_south_africa_research_s10h.py', 'test_south_africa_heads_of_state_c01_09.py',
                     'test_south_africa_anc_presidents_c01_16.py', 'test_south_africa_deputy_presidents_c01_21.py',
                     'test_south_africa_party_leaders_c01_30.py', 'test_south_africa_pac_presidents_c01_32.py'):
            self.assertIn(text, notes)
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        for text in ('ready_for_review', 'south-africa-da-federal-leaders-2000-2026-39.md', 'claude/c01-za-39', '03cd0bb9',
                     'test_south_africa_da_federal_leaders_c01_39.py', 'continue existing claims first',
                     'decided by Ridge', 'Codex may still decide otherwise', '2019-10-23', 'C01-SouthAfrica-DP-001'):
            self.assertIn(text, handoff)
        self.assertNotIn('Ruling requested', handoff)
        index = research.build()
        country = next(p for p in index['countries'] if p['nation'] == 'SouthAfrica')
        self.assertFalse(country['country_census_complete'])
        self.assertIsNone(country['unrepresented_organization_count'])
        self.assertEqual((country['role_observations'], country['source_claims']), (11, 500))
        self.assertEqual(country['mapping_pending'], 53)
        self.assertEqual({w['status'] for w in index['work_orders'] if w['nation'] == 'SouthAfrica'}, {'open'})
        self.assertFalse(index['c01_complete'])


if __name__ == '__main__':
    unittest.main()
