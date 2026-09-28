"""CLAUDE-C01-32: the President of the Pan Africanist Congress of Azania, 1990-2026, is one party office kept apart from
every other role and from state office. Elections, result publications, acting service, suspensions, expulsions, rival
claims, court orders and findings, congress sessions and the Secretary General's office stay separate claims;
constitution amendments sit on the organization and set no lifecycle or identity; every holder is a dated in-office
observation with no start or end, because no record reviewed states one; and holder observations from the years in
which two people claimed the presidency are marked disputed and never resolved by inference."""
import copy
import hashlib
import json
import re
import unittest
from urllib.parse import urlsplit

import campaign_research as research

OBS = 'za_iec_n2024_039'
NAME = 'PAN AFRICANIST CONGRESS OF AZANIA'
ROW = 'national-results-page-2-row-2'
ROW_NO = '039'
ROLE = 'za_pac_president'
TITLE = 'President of the Pan Africanist Congress of Azania'
ACTING = 'Acting President of the Pan Africanist Congress of Azania'
SECGEN = 'Secretary-General of the Pan Africanist Congress of Azania'
PREFIX = 'za_pac_'
ANC_ID = 'za_iec_n2024_014'
# The CLAUDE-C01-16 ANC holders and the CLAUDE-C01-30 ACDP, Freedom Front and IFP holders, unchanged by this packet
# (pinned literally to avoid a circular test import).
ANC_HOLDERS = [
    ('Oliver Tambo', '1990-01-08', None, None),
    ('Nelson Mandela', '1991-07-18', None, None),
    ('Nelson Mandela', '1994-12-22', None, None),
    ('Thabo Mbeki', '1997-12-20', None, None),
    ('Thabo Mbeki', '2002-12-20', None, None),
    ('Jacob Zuma', '2007-12-20', None, None),
    ('Jacob Zuma', '2012-12-20', None, None),
    ('Cyril Ramaphosa', '2017-12-20', None, None),
    ('Cyril Ramaphosa', '2023-01-08', None, None),
    ('Cyril Ramaphosa', '2026-05-15', None, None),
]
C01_30_HOLDERS = {
    'za_acdp_president': [('Kenneth Meshoe', '1999-05-01'), ('Kenneth Meshoe', '2001-10-31'), ('Kenneth Meshoe', '2014-02-18'),
                          ('Kenneth Meshoe', '2018-02-14'), ('Kenneth Meshoe', '2026-03-04')],
    'za_ff_leader': [('Constand Viljoen', '1997-08-26'), ('Pieter Mulder', '2001-06-21'), ('Pieter Mulder', '2003-09-28'),
                     ('Pieter Mulder', '2016-11-12'), ('Pieter Groenewald', '2016-12-27'), ('Pieter Groenewald', '2020-04-02'),
                     ('Pieter Groenewald', '2024-08-22'), ('Corné Mulder', '2025-07-16'), ('Corné Mulder', '2026-03-25')],
    'za_ifp_president': [('Mangosuthu Buthelezi', '1995-10-24'), ('Mangosuthu Buthelezi', '2012-12-16'),
                         ('Mangosuthu Buthelezi', '2019-01-20'), ('Mangosuthu Buthelezi', '2019-08-24'),
                         ('Velenkosini Hlabisa', '2019-09-12'), ('Velenkosini Hlabisa', '2023-09-09'),
                         ('Velenkosini Hlabisa', '2024-08-10'), ('Velenkosini Hlabisa', '2026-02-01')],
}
# The packet's sources before this packet: S10h, CLAUDE-C01-09, CLAUDE-C01-16, CLAUDE-C01-21 and CLAUDE-C01-30.
EARLIER_SOURCE_COUNT = 207
LIFECYCLE_NOTE = 'These observations do not establish founding, dissolution, legal continuity, mergers, splits or exact terms.'

# Original response identity recorded in each extract, (bytes, sha256), fetched as the extract's fetch_recipe says:
# the raw id_ capture, no Accept-Encoding header, no decoding. Every source is a raw Internet Archive capture served
# without content encoding.
RESPONSES = {
    'za_pac_previous_leaders_page_2008': (27554, '46c0301092dc619c93a888e7a104f6b3f429d7ddb6138db8ecabc18aac333cee'),
    'za_pac_previous_presidents_page_2016': (45833, '5a40a0fc02723e4df095596eec6c50e8bcd95ff753f7dcb1be327c1b0fc1dbce'),
    'za_pac_paca_history_page_1998': (5473, 'df5a5a8e8adfaf6298190031b5970c6275585c34690640c0415389a627f20371'),
    'za_pac_constitution_1996_page': (29655, 'e5993685c9524c456007bc97ef67f2162b55916b4bd210cf1e8d980eeedf4339'),
    'za_pac_paca_home_page_1998': (4645, '31bf3f980e859bc15c2f843e544af6cc28eaff4d4d9185d49215dcc86ca43f41'),
    'za_pac_statement_rustenburg_killings_19980519': (2617, 'f924eb285699ddd4594294ff36a8947ee5dc0d8a358083952f1eecc5c46c0dd5'),
    'za_pac_release_june16_rally_19980616': (3866, '84cbb18aeb763eb08a18d7dff33d7c1fbfec67256c22358422ea2c24709f89c9'),
    'za_pac_address_hawkers_council_19981012': (4314, 'e074dc53ef78ccb3035a613f5ed5d034bbd1ea78cdcd88d046a0ea0bdf21a460'),
    'za_pac_release_lesotho_19981103': (3204, '83e27b9f958419188798e60d0dd6a0689e8cc69f02789248f99f4eb4d2be0f02'),
    'za_pac_election_manifesto_1999': (37428, 'd84e45f72c83d63e7c6400de72bc32a29ba8cac0a836bc5725f661aeb24fb38c'),
    'za_pac_election_manifesto_2004': (113533, '020db37d0f02e3d0d5442b0c5bba99d8ecb2bbb8369334d60097fd8fb7ac4e21'),
    'za_pac_constitution_2008_page': (29315, 'e312e3bdfc7728be91ebed11b50ab210fd31b6b83ee2f79aa429cc60b4e06f3d'),
    'za_pac_statement_mbeki_recall_20080920': (23914, '1e9be96285bd4b113c98cac1f71ce4cf9e44ffa7f75268e3ae3685a691f1a1c7'),
    'za_pac_notice_election_summit_20081003': (25711, '8ca7771ac2cc6c58f6932d8260d37f63df0cb2293764aa9bf230ff250a25e348'),
    'za_pac_current_leadership_page_2008': (25008, 'e13fd9c6002180dbdf4d96f69a1c7a4242b0d22cdc7a83898d731122a7589dd4'),
    'za_pac_statement_zuma_case_2009': (23447, 'f1fad097b260e275256d73060ace7a334e2c6536448d58fbc302e59f1acaaaa9'),
    'za_pac_profile_mphahlele_2013': (14207, '98029a6cddc52bc1eaa93b47741b143f078d814e494b5abf399adab75e312cc7'),
    'za_pac_home_page_2013_acting': (25361, 'ea8cd52f387c2c5d231fc08ea43d71ce6d9f4e0fcf23b168aa4245fcc88e2148'),
    'za_pac_home_page_2013_president': (26260, '8ce5db578f8c1ff99e78d008b57e2deb7a79f46e3c32b7c00b2c96de11050fec'),
    'za_pac_latest_news_page_2014': (31154, '07bb0b9bc447059439a4d2a299bc8d854630029e73b139957e9c95a593ace180'),
    'za_pac_home_page_2015': (37124, 'af4ade1eebfc53ed1f39765fe87956a4fabcb451839150fe89832c40ce4dde7b'),
    'za_pac_release_strategic_plan_20160131': (110991, '5d649bc513d7edd4383f7bf93d542d6ce5a1bcad5f2f1358f7941b691d5b2598'),
    'za_pac_statement_iec_20160629': (103669, '81c9584e2ea65accc4641283b7c307720e5b4312111275fda208ec7abd62923e'),
    'za_pac_saflii_gphc_2016_485': (32608, '2058879b224b324ec78325740ef9c128f64ad03e5ac5e00f6cc6c2ba5873a9e0'),
    'za_pac_interview_moloto_secretary_general_20171004': (47211, 'cf12842e8f1a04cc30b1c67494af7cb5143cfbb7ace4fdcc33634b9b7a2ecfb5'),
    'za_pac_post_case_against_mbinda_20180119': (46442, '5ac9d40b80f7d301d8cb1212f320fb5f36ad64e6d30d6e338ecc7a71bb00cf29'),
    'za_pac_release_fraud_case_mbinda_20180304': (49110, 'e382f881e487bd03390847ad3c738deebf428ceec2f6fa6e629761f1da8cb411'),
    'za_pac_statement_moloto_africa_day_20180525': (163464, 'a851231a2e96425c07dc03604e3a997ad70377df7bd985f562c1bd16db579154'),
    'za_pac_saflii_gphc_2019_537': (45931, 'd65722a97a841c156173e801431f3373362e2269e20a23e1c96703740f41b0d7'),
    'za_pac_statement_litigation_moloto_20191230': (44241, '3fb6983fe8606ce517110e38c9f712ce0f5d8af8a4c4e705af53184d95207d22'),
    'za_pac_post_nyhontso_re_elected_20190901': (38094, '6a4a7f44ffcd7332eba7994d8ddef3ea0072f6c1f18486f0c474124ff12db868'),
    'za_pac_saflii_gphc_2021_539': (56722, 'f0df955a447f2bb4fb94556c4774dd6c184de04050a2ac29088dc103283be51c'),
    'za_pac_speech_polokwane_congress_20200215': (43319, '016d7f50bf035e8288690b575ce0492dc43d2755896fb562c612b3d1b0b6aeb0'),
    'za_pac_post_president_regional_tour_20210719': (41816, '8036d4123331db97f1eaf03441b9380675eb60c726d45d096b8cd049e83ad695'),
    'za_pac_new_year_message_20211231': (51616, '02d99cf51977dce791f404a158675fef59fc0cac50f8a923087cc475679985fe'),
    'za_pac_saflii_sca_2023_140': (342004, '64a61a15bb8720a744a0202bafca2b9077b6e8799f31fe9a8c626f402ba37be0'),
    'za_pac_x_elective_congress_opens_20251211': (6398, '4ca01fee1db58b1e3d5bb4f68eb372a3aa9beb13e01397d8f220e643b7c9e248'),
    'za_pac_x_manifesto_launch_20260829': (6735, '553585de4dcf6403a7e4c247dc01f37b1c9016bc917d19d07f3c01a03f596a7e'),
}
# Capture timestamp of each raw id_ capture (all before the 7 September 2026 cutoff).
ARCHIVED = {
    'za_pac_previous_leaders_page_2008': '20081214170731',
    'za_pac_previous_presidents_page_2016': '20160331141517',
    'za_pac_paca_history_page_1998': '19990202211516',
    'za_pac_constitution_1996_page': '20081214170751',
    'za_pac_paca_home_page_1998': '19981203082344',
    'za_pac_statement_rustenburg_killings_19980519': '19991009200645',
    'za_pac_release_june16_rally_19980616': '19990203023632',
    'za_pac_address_hawkers_council_19981012': '19990203055228',
    'za_pac_release_lesotho_19981103': '19991009203935',
    'za_pac_election_manifesto_1999': '20000116035918',
    'za_pac_election_manifesto_2004': '20040408052313',
    'za_pac_constitution_2008_page': '20081214170655',
    'za_pac_statement_mbeki_recall_20080920': '20081214170837',
    'za_pac_notice_election_summit_20081003': '20081214170847',
    'za_pac_current_leadership_page_2008': '20081214170721',
    'za_pac_statement_zuma_case_2009': '20100821043051',
    'za_pac_profile_mphahlele_2013': '20130324104447',
    'za_pac_home_page_2013_acting': '20130704024729',
    'za_pac_home_page_2013_president': '20130918015202',
    'za_pac_latest_news_page_2014': '20140421043905',
    'za_pac_home_page_2015': '20150726193947',
    'za_pac_release_strategic_plan_20160131': '20160915171535',
    'za_pac_statement_iec_20160629': '20160915235930',
    'za_pac_saflii_gphc_2016_485': '20250215013134',
    'za_pac_interview_moloto_secretary_general_20171004': '20171008235651',
    'za_pac_post_case_against_mbinda_20180119': '20180322212837',
    'za_pac_release_fraud_case_mbinda_20180304': '20180322212842',
    'za_pac_statement_moloto_africa_day_20180525': '20190512085731',
    'za_pac_saflii_gphc_2019_537': '20250427101519',
    'za_pac_statement_litigation_moloto_20191230': '20210516213210',
    'za_pac_post_nyhontso_re_elected_20190901': '20210618173923',
    'za_pac_saflii_gphc_2021_539': '20210901121341',
    'za_pac_speech_polokwane_congress_20200215': '20240601013704',
    'za_pac_post_president_regional_tour_20210719': '20210719205027',
    'za_pac_new_year_message_20211231': '20220101114152',
    'za_pac_saflii_sca_2023_140': '20240812212246',
    'za_pac_x_elective_congress_opens_20251211': '20251211181701',
    'za_pac_x_manifesto_launch_20260829': '20260829111630',
}
# Every new claim's (attested_on, event_kind), exactly: distinct dated events are never re-dated or relabelled.
EVENTS = {
    'za_pac_previous_leaders_list_2008': (None, 'retrospective_list'),
    'za_pac_previous_presidents_list_2016': (None, 'retrospective_list'),
    'za_pac_history_mogoba_elected_president_december_1996': (None, 'election_reference_retrospective'),
    'za_pac_constitution_amended_thohoyandou_congress_1996': (None, 'constitution_amended_at_congress'),
    'za_pac_home_page_congress_keynote_president_mogoba_199712': (None, 'in_office_attestation_month_only'),
    'za_pac_home_page_president_mogoba_1998': (None, 'in_office_continuation_attestation'),
    'za_pac_statement_president_mogoba_19980519': ('1998-05-19', 'in_office_attestation'),
    'za_pac_release_president_mogoba_19980616': ('1998-06-16', 'in_office_attestation'),
    'za_pac_address_president_mogoba_19981012': ('1998-10-12', 'in_office_attestation'),
    'za_pac_release_president_mogoba_19981103': ('1998-11-03', 'in_office_attestation'),
    'za_pac_manifesto_1999_president_mogoba': (None, 'in_office_continuation_attestation'),
    'za_pac_manifesto_2004_foreword_president_pheko': (None, 'in_office_continuation_attestation'),
    'za_pac_constitution_amendments_1990_1992_1996_2000': (None, 'constitution_amendment_years'),
    'za_pac_constitution_adopted_9th_national_congress_2008': (None, 'constitution_adopted_at_congress'),
    'za_pac_statement_president_mphahlele_20080920': ('2008-09-20', 'in_office_attestation'),
    'za_pac_notice_president_mphahlele_20081003': ('2008-10-03', 'in_office_attestation'),
    'za_pac_leadership_page_president_mphahlele_2008': (None, 'in_office_continuation_attestation'),
    'za_pac_statement_president_mphahlele_20090114': ('2009-01-14', 'in_office_attestation'),
    'za_pac_profile_president_mphahlele_2013': (None, 'in_office_continuation_attestation'),
    'za_pac_profile_mphahlele_elected_president_2006': (None, 'election_reference_retrospective'),
    'za_pac_profile_mphahlele_reelected_unopposed_2008': (None, 'election_reference_retrospective'),
    'za_pac_home_page_acting_president_mpheti_2013': (None, 'acting_service'),
    'za_pac_home_page_pheko_elected_president_15_june': (None, 'election_reference_retrospective'),
    'za_pac_home_page_president_mpheti_2013': (None, 'in_office_continuation_attestation'),
    'za_pac_report_president_mphethi_20140321': ('2014-03-21', 'in_office_attestation'),
    'za_pac_home_page_president_mbinda_2015': (None, 'in_office_continuation_attestation'),
    'za_pac_home_page_mbinda_elected_president_2014': (None, 'election_reference_retrospective'),
    'za_pac_release_president_mbinda_20160131': ('2016-01-31', 'in_office_attestation'),
    'za_pac_statement_president_mbinda_sworn_in_mp_2015': ('2016-06-29', 'in_office_continuation_attestation'),
    'za_pac_statement_expelled_alton_mpheti_2016': (None, 'expulsion_reference'),
    'za_pac_court_order_iec_to_deal_with_mbinda_moloto_20160420': ('2016-04-20', 'court_order'),
    'za_pac_court_mphahlele_expelled_may_2013': (None, 'expulsion_reference'),
    'za_pac_court_mphahlele_claims_presidency_2016': (None, 'rival_claim'),
    'za_pac_iec_funding_suspended_leadership_struggle_20150617': ('2015-06-17', 'iec_decision_on_leadership_dispute'),
    'za_pac_court_leave_to_appeal_refused_mphahlele_2016': (None, 'court_order'),
    'za_pac_interview_secretary_general_moloto_20171004': ('2017-10-04', 'other_office_attestation'),
    'za_pac_post_president_moloto_20180119': ('2018-01-19', 'in_office_attestation'),
    'za_pac_post_former_president_mbinda_expelled_2017': (None, 'expulsion_reference'),
    'za_pac_post_then_president_mphethi_2014': (None, 'predecessor_reference'),
    'za_pac_post_case_against_mbinda_hearing_scheduled_20180119': ('2018-01-19', 'court_hearing_scheduled'),
    'za_pac_release_mbinda_expulsion_took_effect_20170613': ('2017-06-13', 'expulsion_effective'),
    'za_pac_release_mbinda_claims_to_be_legitimate_pac_20180304': ('2018-03-04', 'rival_claim'),
    'za_pac_statement_president_moloto_20180525': ('2018-05-25', 'in_office_attestation'),
    'za_pac_court_consent_order_moloto_acknowledged_president_20190308': ('2019-03-08', 'court_order'),
    'za_pac_court_moloto_invokes_emergency_powers_20190609': ('2019-06-09', 'emergency_powers_invocation'),
    'za_pac_court_current_president_moloto_20190712': ('2019-07-12', 'in_office_attestation'),
    'za_pac_court_emergency_powers_set_aside_20190712': ('2019-07-12', 'court_order'),
    'za_pac_statement_rival_necs_kimberly_2018_mpumalanga_2017': (None, 'rival_congresses_reference'),
    'za_pac_statement_moloto_chaired_as_then_president': (None, 'retrospective_in_office_reference'),
    'za_pac_statement_moloto_suspended_20190720': ('2019-07-20', 'suspension'),
    'za_pac_statement_nyhontso_elected_at_bloemfontein_20191230': ('2019-12-30', 'election_reference'),
    'za_pac_statement_electoral_court_application_20191230': ('2019-12-30', 'court_application'),
    'za_pac_post_nyhontso_re_elected_president_published_20190901': ('2019-09-01', 'result_publication'),
    'za_pac_court_mavundla_order_president_moloto_20190308': ('2019-03-08', 'court_order'),
    'za_pac_court_limpopo_congress_elects_moloto_2019': (None, 'election_by_congress'),
    'za_pac_court_bloemfontein_congress_elects_nyhontso_2019': (None, 'election_by_congress'),
    'za_pac_court_declares_limpopo_election_invalid_20210823': ('2021-08-23', 'court_order'),
    'za_pac_court_declares_bloemfontein_nec_lawful_20210823': ('2021-08-23', 'court_order'),
    'za_pac_speech_president_nyhontso_20200215': ('2020-02-15', 'in_office_attestation'),
    'za_pac_post_president_nyhontso_20210719': ('2021-07-19', 'in_office_attestation'),
    'za_pac_new_year_message_president_nyhontso_20211231': ('2021-12-31', 'in_office_attestation'),
    'za_pac_sca_appeal_dismissed_20231027': ('2023-10-27', 'court_order'),
    'za_pac_sca_moloto_not_re_elected_rebel_group_20231027': ('2023-10-27', 'court_finding'),
    'za_pac_x_national_elective_congress_opens_20251211': ('2025-12-11', 'congress_session_start'),
    'za_pac_x_president_nyhontso_manifesto_launch_20260829': ('2026-08-29', 'in_office_attestation'),
}
# Claims printed without a structured date: retrospective, undated, month-only, year-only, span-only and
# conflicting-day claims.
UNDATED = (
    'za_pac_previous_leaders_list_2008',
    'za_pac_previous_presidents_list_2016',
    'za_pac_history_mogoba_elected_president_december_1996',
    'za_pac_constitution_amended_thohoyandou_congress_1996',
    'za_pac_home_page_congress_keynote_president_mogoba_199712',
    'za_pac_home_page_president_mogoba_1998',
    'za_pac_manifesto_1999_president_mogoba',
    'za_pac_manifesto_2004_foreword_president_pheko',
    'za_pac_constitution_amendments_1990_1992_1996_2000',
    'za_pac_constitution_adopted_9th_national_congress_2008',
    'za_pac_leadership_page_president_mphahlele_2008',
    'za_pac_profile_president_mphahlele_2013',
    'za_pac_profile_mphahlele_elected_president_2006',
    'za_pac_profile_mphahlele_reelected_unopposed_2008',
    'za_pac_home_page_acting_president_mpheti_2013',
    'za_pac_home_page_pheko_elected_president_15_june',
    'za_pac_home_page_president_mpheti_2013',
    'za_pac_home_page_president_mbinda_2015',
    'za_pac_home_page_mbinda_elected_president_2014',
    'za_pac_statement_expelled_alton_mpheti_2016',
    'za_pac_court_mphahlele_expelled_may_2013',
    'za_pac_court_mphahlele_claims_presidency_2016',
    'za_pac_court_leave_to_appeal_refused_mphahlele_2016',
    'za_pac_post_former_president_mbinda_expelled_2017',
    'za_pac_post_then_president_mphethi_2014',
    'za_pac_statement_rival_necs_kimberly_2018_mpumalanga_2017',
    'za_pac_statement_moloto_chaired_as_then_president',
    'za_pac_court_limpopo_congress_elects_moloto_2019',
    'za_pac_court_bloemfontein_congress_elects_nyhontso_2019',
)
NEW_SOURCES = list(RESPONSES)
# Holder observations (name, attested_on, from, until), one per cited in-office attestation, in date order.
HOLDERS = [
    ('Stanley Mogoba', '1998-05-19', None, None),
    ('Stanley Mogoba', '1998-06-16', None, None),
    ('Stanley Mogoba', '1998-10-12', None, None),
    ('Stanley Mogoba', '1998-11-03', None, None),
    ('Letlapa Mphahlele', '2008-09-20', None, None),
    ('Letlapa Mphahlele', '2008-10-03', None, None),
    ('Letlapa Mphahlele', '2009-01-14', None, None),
    ('Alton Mphethi', '2014-03-21', None, None),
    ('Luthando Mbinda', '2016-01-31', None, None),
    ('Narius Moloto', '2018-01-19', None, None),
    ('Narius Moloto', '2018-05-25', None, None),
    ('Narius Moloto', '2019-07-12', None, None),
    ('Mzwanele Nyhontso', '2020-02-15', None, None),
    ('Mzwanele Nyhontso', '2021-07-19', None, None),
    ('Mzwanele Nyhontso', '2021-12-31', None, None),
    ('Mzwanele Nyhontso', '2026-08-29', None, None),
]
HOLDER_CLAIMS = [
    ['za_pac_statement_president_mogoba_19980519'],
    ['za_pac_release_president_mogoba_19980616'],
    ['za_pac_address_president_mogoba_19981012'],
    ['za_pac_release_president_mogoba_19981103'],
    ['za_pac_statement_president_mphahlele_20080920'],
    ['za_pac_notice_president_mphahlele_20081003'],
    ['za_pac_statement_president_mphahlele_20090114'],
    ['za_pac_report_president_mphethi_20140321'],
    ['za_pac_release_president_mbinda_20160131'],
    ['za_pac_post_president_moloto_20180119'],
    ['za_pac_statement_president_moloto_20180525'],
    ['za_pac_court_current_president_moloto_20190712'],
    ['za_pac_speech_president_nyhontso_20200215'],
    ['za_pac_post_president_nyhontso_20210719'],
    ['za_pac_new_year_message_president_nyhontso_20211231'],
    ['za_pac_x_president_nyhontso_manifesto_launch_20260829'],
]
HOLDER_REVIEW = ['ZA-PAC-03', 'ZA-PAC-03', 'ZA-PAC-03', 'ZA-PAC-03', 'ZA-PAC-05', 'ZA-PAC-05', 'ZA-PAC-05', 'ZA-PAC-06', 'ZA-PAC-07', 'ZA-PAC-08', 'ZA-PAC-08', 'ZA-PAC-09', 'ZA-PAC-10', 'ZA-PAC-10', 'ZA-PAC-10', 'ZA-PAC-11']
# Holder observations from years in which another person claimed the presidency: marked disputed, never resolved.
DISPUTED = [7, 8, 9, 10, 12, 13, 14]
# Role claims that must never feed a holder.
NEVER_HOLDER = (
    'za_pac_previous_leaders_list_2008',
    'za_pac_previous_presidents_list_2016',
    'za_pac_history_mogoba_elected_president_december_1996',
    'za_pac_home_page_congress_keynote_president_mogoba_199712',
    'za_pac_home_page_president_mogoba_1998',
    'za_pac_manifesto_1999_president_mogoba',
    'za_pac_manifesto_2004_foreword_president_pheko',
    'za_pac_leadership_page_president_mphahlele_2008',
    'za_pac_profile_president_mphahlele_2013',
    'za_pac_profile_mphahlele_elected_president_2006',
    'za_pac_profile_mphahlele_reelected_unopposed_2008',
    'za_pac_home_page_acting_president_mpheti_2013',
    'za_pac_home_page_pheko_elected_president_15_june',
    'za_pac_home_page_president_mpheti_2013',
    'za_pac_home_page_president_mbinda_2015',
    'za_pac_home_page_mbinda_elected_president_2014',
    'za_pac_statement_president_mbinda_sworn_in_mp_2015',
    'za_pac_statement_expelled_alton_mpheti_2016',
    'za_pac_court_order_iec_to_deal_with_mbinda_moloto_20160420',
    'za_pac_court_mphahlele_expelled_may_2013',
    'za_pac_court_mphahlele_claims_presidency_2016',
    'za_pac_iec_funding_suspended_leadership_struggle_20150617',
    'za_pac_court_leave_to_appeal_refused_mphahlele_2016',
    'za_pac_interview_secretary_general_moloto_20171004',
    'za_pac_post_former_president_mbinda_expelled_2017',
    'za_pac_post_then_president_mphethi_2014',
    'za_pac_post_case_against_mbinda_hearing_scheduled_20180119',
    'za_pac_release_mbinda_expulsion_took_effect_20170613',
    'za_pac_release_mbinda_claims_to_be_legitimate_pac_20180304',
    'za_pac_court_consent_order_moloto_acknowledged_president_20190308',
    'za_pac_court_moloto_invokes_emergency_powers_20190609',
    'za_pac_court_emergency_powers_set_aside_20190712',
    'za_pac_statement_rival_necs_kimberly_2018_mpumalanga_2017',
    'za_pac_statement_moloto_chaired_as_then_president',
    'za_pac_statement_moloto_suspended_20190720',
    'za_pac_statement_nyhontso_elected_at_bloemfontein_20191230',
    'za_pac_statement_electoral_court_application_20191230',
    'za_pac_post_nyhontso_re_elected_president_published_20190901',
    'za_pac_court_mavundla_order_president_moloto_20190308',
    'za_pac_court_limpopo_congress_elects_moloto_2019',
    'za_pac_court_bloemfontein_congress_elects_nyhontso_2019',
    'za_pac_court_declares_limpopo_election_invalid_20210823',
    'za_pac_court_declares_bloemfontein_nec_lawful_20210823',
    'za_pac_sca_appeal_dismissed_20231027',
    'za_pac_sca_moloto_not_re_elected_rebel_group_20231027',
    'za_pac_x_national_elective_congress_opens_20251211',
)
# Organization claims (constitution amendments), cited by the organization observation and never by the role.
ORGANIZATION_ONLY = (
    'za_pac_constitution_amended_thohoyandou_congress_1996',
    'za_pac_constitution_amendments_1990_1992_1996_2000',
    'za_pac_constitution_adopted_9th_national_congress_2008',
)
REVIEW = ['ZA-PAC-01', 'ZA-PAC-02', 'ZA-PAC-03', 'ZA-PAC-04', 'ZA-PAC-05', 'ZA-PAC-06', 'ZA-PAC-07', 'ZA-PAC-08', 'ZA-PAC-09', 'ZA-PAC-10', 'ZA-PAC-11']
# Days that must never be any holder's observation, start or end: elections and congress spans, result publications,
# expulsions, suspensions, court orders and findings, the IEC decision, the other office, acting service and lead days
# (Mothopeng's reported death, Makwetu's reported election, the TRC hearing, the dropped 2008 and 2009 items, the
# ministerial appointment announced in 2024).
NEVER_HOLDER_DATE = {'1990-10-23', '1990-12-01', '1996-12-01', '1997-10-07', '1997-12-01', '2003-06-15', '2006-09-25',
                     '2008-07-04', '2008-07-06', '2008-10-23', '2009-01-16', '2013-05-01', '2014-09-28', '2015-06-17',
                     '2016-04-20', '2016-06-20', '2016-06-21', '2016-06-29', '2017-06-13', '2017-10-04', '2018-03-01',
                     '2018-03-04', '2019-03-08', '2019-06-09', '2019-07-20', '2019-08-24', '2019-08-25', '2019-08-29',
                     '2019-08-30', '2019-09-01', '2019-12-30', '2021-08-23', '2023-10-27', '2024-06-30', '2025-12-11',
                     '2025-12-14'}
HOLDER_KINDS = {'in_office_attestation'}
BOUNDARY_KINDS = {'assumption_of_office', 'end_of_term_statement', 'resignation_effective', 'oath_of_office'}
# Rows whose printed office is not the role's own title.
OTHER_TITLE_CLAIMS = {
    'za_pac_home_page_acting_president_mpheti_2013': ACTING,
    'za_pac_interview_secretary_general_moloto_20171004': SECGEN,
}
# Holder names and the surname each cited claim prints (the 2008 statement prints "MPHAHLEL").
PEOPLE = {'Stanley Mogoba': 'Mogoba', 'Letlapa Mphahlele': 'Mphahlel', 'Alton Mphethi': 'Mphethi',
          'Luthando Mbinda': 'Mbinda', 'Narius Moloto': 'Moloto', 'Mzwanele Nyhontso': 'Nyhontso'}
# People the packet names as holders of the office in any claim, including those in lists and references only: nine.
NAMED_PEOPLE = set(PEOPLE) | {'Motsoko Pheko', 'Zephania Mothopeng', 'Clarence Makwetu'}
COURT_SOURCES = ('za_pac_saflii_gphc_2016_485', 'za_pac_saflii_gphc_2019_537', 'za_pac_saflii_gphc_2021_539',
                 'za_pac_saflii_sca_2023_140')
X_SOURCES = ('za_pac_x_elective_congress_opens_20251211', 'za_pac_x_manifesto_launch_20260829')
PDF_PAGES = {'za_pac_statement_moloto_africa_day_20180525': [1, 2], 'za_pac_saflii_sca_2023_140': [1, 3, 4, 10]}
# Secondary leads and pages that must never be a source: history sites, encyclopaedias, news, mirrors and live pages.
LEAD_URL_MARKERS = ('sahistory', 'wikipedia', 'britannica', 'news24', 'iol.co.za', 'timeslive', 'dailymaverick',
                    'ewn.co.za', 'sabcnews', 'politicsweb', 'polity.org', 'reuters', 'bbc.', 'voanews', 'mg.co.za',
                    'nelsonmandela.org', 'omalley', 'disa.ukzn', 'citizen.co.za', 'lawlibrary', 'justice.gov.za',
                    'digitallibrary.un.org', 'pmg.org.za', 'x.com/')
# Page shapes a server can generate per request or that grow over time (search, API, listings, feeds, cache-busting).
PER_REQUEST_URL = re.compile(r'wp-json|cdx/search|cdn-cgi|email-protection|nocache|cachebust|[?&]s=|[?&]_=|[?&]cb=|'
                             r'/search[/?]|/feed|/embed|/page/\d|/tag/|/category/|download\.php|rnd=|/api/', re.I)
ARCHIVED_URL = re.compile(r'^https://web\.archive\.org/web/(\d{14})id_/(https?://(?:(?:www\.|new-web\.)?pac\.org\.za|'
                          r'(?:www\.)?paca\.org\.za|www\.saflii\.org|www\.pacofazania\.org\.za|pacofazania\.org|'
                          r'twitter\.com/MyPAConline/status/\d+)(?::80)?(?:/.*)?)$')
HOSTS = {'www.paca.org.za', 'paca.org.za', 'www.pac.org.za', 'pac.org.za', 'new-web.pac.org.za', 'www.saflii.org',
         'www.pacofazania.org.za', 'pacofazania.org', 'twitter.com'}
DECISIONS = {
    'ZA-PAC-01': 'Unresolved', 'ZA-PAC-02': 'Accepted in part', 'ZA-PAC-03': 'Accepted', 'ZA-PAC-04': 'Unresolved',
    'ZA-PAC-05': 'Accepted in part', 'ZA-PAC-06': 'Accepted in part', 'ZA-PAC-07': 'Accepted in part',
    'ZA-PAC-08': 'Accepted in part', 'ZA-PAC-09': 'Accepted in part', 'ZA-PAC-10': 'Accepted in part',
    'ZA-PAC-11': 'Accepted in part',
}
REPORT = research.RESEARCH / 'south-africa-pac-presidents-1990-2026-32.md'
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-32.md'


def pac_invariants(packet):
    """Packet-level rules this test owns; raises AssertionError, KeyError, IndexError or ValueError on any violation."""
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    claim_source = {c['id']: s['id'] for s in packet['sources'] for c in s['claims']}
    orgs = {o['id']: o for o in packet['organizations']}
    assert len(packet['institutions']) == 1 and packet['institutions'][0]['id'] == 'za_presidency'
    presidency = packet['institutions'][0]
    # The presidency roles and the ANC, DA, ACDP, Freedom Front and IFP party roles are unchanged by this packet.
    assert [r['id'] for r in presidency['roles']] == ['za_president_election', 'za_state_president', 'za_deputy_president']
    assert sum(len(r['holder_claims']) for r in presidency['roles']) == 23
    assert [(h['name'], h['attested_on'], h['from'], h['until'])
            for h in orgs[ANC_ID]['roles'][0]['holder_claims']] == ANC_HOLDERS
    earlier_roles = {r['id']: r for o in packet['organizations'] for r in o['roles'] if r['id'] in C01_30_HOLDERS}
    assert set(earlier_roles) == set(C01_30_HOLDERS)
    for rid, holders in C01_30_HOLDERS.items():
        assert [(h['name'], h['attested_on']) for h in earlier_roles[rid]['holder_claims']] == holders, rid
        assert all(h['from'] is None and h['until'] is None for h in earlier_roles[rid]['holder_claims']), rid
    assert {o['id']: [r['id'] for r in o['roles']] for o in packet['organizations'] if o['roles']} == {
        'za_iec_n2024_008': ['za_acdp_president'], 'za_iec_n2024_014': ['za_anc_president'],
        'za_iec_n2024_027': ['za_da_federal_leader', 'za_da_federal_chair', 'za_da_council_chair'],
        'za_iec_n2024_034': ['za_ifp_president'], 'za_iec_n2024_039': ['za_pac_president'],
        'za_iec_n2024_051': ['za_ff_leader']}
    org = orgs[OBS]
    # The IEC reporting identity, unknown lifecycle and empty game mapping stay unchanged.
    assert (org['name'], org['kind']) == (NAME, 'national_ballot_party_reporting_identity')
    assert org['source_identifier']['value'] == ROW
    assert org['represented_party_ids'] == [] and org['reconciled_organization_id'] is None
    assert org['lifecycle'] == {'status': 'unknown', 'from': None, 'until': None, 'note': LIFECYCLE_NOTE}
    assert [r['id'] for r in org['roles']] == [ROLE]
    role = org['roles'][0]
    assert (role['title'], role['kind']) == (TITLE, 'party_leader')
    holders = role['holder_claims']
    assert all(isinstance(h, dict) for h in holders)
    for index, h in enumerate(holders):
        assert 'acting' not in h['name'].lower() and 'interim' not in h['name'].lower(), h['name']
        assert h['from'] is None and h['until'] is None, h['name']
        assert h['attested_on'] not in NEVER_HOLDER_DATE, h['name']
        assert not set(h['claim_ids']) & set(NEVER_HOLDER), h['name']
        assert not set(h['claim_ids']) & set(ORGANIZATION_ONLY), h['name']
        # A disputed presidency is recorded as disputed on exactly the holder observations of the disputed years.
        assert ('Disputed:' in h.get('uncertainty', '')) == (index in DISPUTED), (index, h['name'])
        expected = []
        for cid in h['claim_ids']:
            assert cid in role['claim_ids'] and cid.startswith(PREFIX), cid
            assert claims[cid]['attested_on'] == h['attested_on'], cid
            if claim_source[cid] not in expected:
                expected.append(claim_source[cid])
        assert h['sources'] == expected, h['name']
    # The exact holder list, checked after the rules above so that each rule is exercised on its own.
    assert [(h['name'], h['attested_on'], h['from'], h['until']) for h in holders] == HOLDERS
    assert [h['claim_ids'] for h in holders] == HOLDER_CLAIMS
    # Organization claims (constitution amendments) sit on the observation, never on the role.
    assert not set(role['claim_ids']) & set(ORGANIZATION_ONLY)
    assert set(ORGANIZATION_ONLY) <= set(org['claim_ids'])
    role_claims = set(role['claim_ids']) | {c for h in holders for c in h['claim_ids']}
    role_sources = set(role['sources']) | {s for h in holders for s in h['sources']}
    assert all(claim_source[c] in role_sources for c in role_claims)
    assert set(role['claim_ids']) <= set(org['claim_ids']) and set(role['sources']) <= set(org['sources'])
    chain_ids = set(org['claim_ids']) - {f'za_n2024_ballot_{ROW_NO}', f'za_n2024_seats_{ROW_NO}'}
    chain_srcs = set(org['sources']) - {'za_iec_national_results_20240621', 'za_iec_national_seats_20240606'}
    assert chain_ids and all(c.startswith(PREFIX) for c in chain_ids)
    assert chain_srcs and all(s.startswith(PREFIX) for s in chain_srcs)
    assert role_claims <= chain_ids and role_sources <= chain_srcs
    # Party office and state office, and every other role, never feed each other.
    for entry in packet['organizations'] + packet['institutions']:
        if entry is org:
            continue
        entry_claims = set(entry['claim_ids']) | {c for r in entry['roles'] for c in r['claim_ids']} | {
            c for r in entry['roles'] for h in r['holder_claims'] for c in h['claim_ids']}
        entry_sources = set(entry['sources']) | {s for r in entry['roles'] for s in r['sources']} | {
            s for r in entry['roles'] for h in r['holder_claims'] for s in h['sources']}
        assert not entry_claims & chain_ids and not entry_sources & chain_srcs, entry['id']
        assert not any(c.startswith(PREFIX) for c in entry_claims | entry_sources), entry['id']
        assert not any(r['id'] == ROLE for r in entry['roles']), entry['id']
    # Undated claims carry no structured date at all, and distinct dated events keep their own day.
    for cid in UNDATED:
        assert not {'attested_on', 'attested_period', 'period'} & set(claims[cid]), cid
    for cid, (day, _kind) in EVENTS.items():
        assert claims[cid].get('attested_on') == day, cid


class SouthAfricaPacPresidentsTests(unittest.TestCase):
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
        self.assertEqual((len(NEW_SOURCES), len(self.new_claims)), (38, 65))
        order = [s['id'] for s in self.packet['sources']]
        self.assertEqual(len(order), EARLIER_SOURCE_COUNT + len(NEW_SOURCES))
        self.assertEqual(order[EARLIER_SOURCE_COUNT:], NEW_SOURCES)
        self.assertFalse([sid for sid in order[:EARLIER_SOURCE_COUNT] if sid.startswith(PREFIX)])
        self.assertEqual((len(ids['entries']), len(ids['roles'])), (53, 11))
        self.assertEqual(list(EVENTS), self.new_claims)
        # Every new claim is exactly one of: a holder claim, a role claim that never feeds a holder, or an
        # organization-only claim.
        holder_claims = [cid for ids_ in HOLDER_CLAIMS for cid in ids_]
        groups = holder_claims + list(NEVER_HOLDER) + list(ORGANIZATION_ONLY)
        self.assertEqual(len(groups), len(set(groups)))
        self.assertEqual(set(groups), set(self.new_claims))
        self.assertEqual((len(holder_claims), len(NEVER_HOLDER), len(ORGANIZATION_ONLY)), (16, 46, 3))
        self.assertEqual(self.role['claim_ids'], [cid for cid in self.new_claims if cid not in ORGANIZATION_ONLY])
        self.assertEqual(self.role['sources'], [sid for sid in NEW_SOURCES
                                                if {c['id'] for c in self.sources[sid]['claims']} - set(ORGANIZATION_ONLY)])
        self.assertEqual(self.org['claim_ids'], [f'za_n2024_ballot_{ROW_NO}', f'za_n2024_seats_{ROW_NO}'] + self.new_claims)
        self.assertEqual(self.org['sources'], ['za_iec_national_results_20240621', 'za_iec_national_seats_20240606']
                         + NEW_SOURCES)
        observations = re.findall(r'^### (ZA-PAC-\d\d)\b', self.report, re.M)
        self.assertEqual(observations, REVIEW)
        self.assertEqual(REVIEW, [f'ZA-PAC-{n:02d}' for n in range(1, 12)])
        self.assertEqual({row['review_observation'] for row in self.rows.values()}, set(REVIEW))
        self.assertEqual([self.rows[ids_[0]]['review_observation'] for ids_ in HOLDER_CLAIMS], HOLDER_REVIEW)
        # At most ten people in the packet: six holders, and no row names anyone else as its holder except Motsoko
        # Pheko; Zephania Mothopeng and Clarence Makwetu appear only in retrospective lists.
        self.assertEqual({h[0] for h in HOLDERS}, set(PEOPLE))
        self.assertEqual({row['holder_name'] for row in self.rows.values()}, set(PEOPLE) | {'Motsoko Pheko', None})
        self.assertLessEqual(len(NAMED_PEOPLE), 10)
        for person in ('Mothopeng', 'Makwetu'):
            self.assertTrue(any(person in self.claims[cid]['text'] for cid in self.new_claims), person)
        self.assertNotIn('"name":', json.dumps([self.extracts[sid]['rows'] for sid in NEW_SOURCES]))

    def test_holders_are_exactly_as_intended(self):
        pac_invariants(self.packet)
        self.assertEqual(len(DISPUTED), 7)
        self.assertEqual({HOLDERS[i][0] for i in DISPUTED},
                         {'Alton Mphethi', 'Luthando Mbinda', 'Narius Moloto', 'Mzwanele Nyhontso'})
        for index, holder in enumerate(self.role['holder_claims']):
            self.assertTrue(holder['note'].startswith('Observed on '), holder['name'])
            self.assertRegex(holder['uncertainty'], r'No start', holder['name'])
            if index in DISPUTED:
                self.assertIn('the dispute is recorded, not resolved', holder['uncertainty'], index)
            for cid in holder['claim_ids']:
                row = self.rows[cid]
                self.assertIn(PEOPLE[holder['name']].lower(), self.claims[cid]['text'].lower(), cid)
                self.assertEqual((row['holder_name'], row['role_id']), (holder['name'], ROLE), cid)
                self.assertIn(row['event_kind'], HOLDER_KINDS, cid)
                self.assertEqual(row['review_observation'], HOLDER_REVIEW[index], cid)
                self.assertIn('Dates the holder observation', self.claims[cid]['uncertainty'], cid)
        for cid in NEVER_HOLDER:
            self.assertNotIn(self.rows[cid]['event_kind'], HOLDER_KINDS | BOUNDARY_KINDS, cid)
            self.assertEqual(self.rows[cid]['role_id'], ROLE, cid)
            self.assertRegex(self.claims[cid]['uncertainty'], r'Never a holder date', cid)
        for cid in ORGANIZATION_ONLY:
            row = self.rows[cid]
            self.assertEqual((row['role_id'], row['role_title'], row['holder_name']), (None, None, None), cid)
            self.assertRegex(row['event_kind'], r'^constitution_', cid)
            self.assertIn('organization claim', self.claims[cid]['uncertainty'].lower(), cid)
        # Every role row carries the role's own title except acting service and the Secretary General's office.
        for cid in self.new_claims:
            if cid in ORGANIZATION_ONLY:
                continue
            self.assertEqual(self.rows[cid]['role_title'], OTHER_TITLE_CLAIMS.get(cid, TITLE), cid)
        for cid in OTHER_TITLE_CLAIMS:
            self.assertIn(cid, NEVER_HOLDER, cid)
        self.assertEqual(self.rows['za_pac_home_page_acting_president_mpheti_2013']['event_kind'], 'acting_service')
        # Court orders and findings, expulsions, suspensions, rival claims and elections never feed a holder.
        kinds = {self.rows[cid]['event_kind'] for cid in NEVER_HOLDER}
        for kind in ('court_order', 'court_finding', 'expulsion_effective', 'expulsion_reference', 'suspension',
                     'rival_claim', 'election_by_congress', 'result_publication', 'acting_service',
                     'other_office_attestation', 'retrospective_list', 'in_office_continuation_attestation'):
            self.assertIn(kind, kinds, kind)
        # One holder observation rests on a court judgment's description of the office; every other on a PAC record.
        court_holders = [h for h in self.role['holder_claims'] if set(h['sources']) & set(COURT_SOURCES)]
        self.assertEqual([(h['name'], h['attested_on']) for h in court_holders], [('Narius Moloto', '2019-07-12')])

    def test_no_start_or_end_is_stated_or_inferred(self):
        claims = self.claims
        for cid in UNDATED:
            self.assertIn('no structured date', claims[cid]['uncertainty'].lower(), cid)
            self.assertTrue(self.rows[cid]['printed_range'], cid)
        # Evidence that is recorded but never used as a boundary, each saying why.
        expulsion = claims['za_pac_release_mbinda_expulsion_took_effect_20170613']
        self.assertIn('took effect on 13 June 2017', expulsion['text'])
        self.assertIn('never an end', expulsion['uncertainty'])
        self.assertIn('never an end', claims['za_pac_sca_moloto_not_re_elected_rebel_group_20231027']['uncertainty'])
        self.assertIn('has not been re-elected as the President since 2020',
                      claims['za_pac_sca_moloto_not_re_elected_rebel_group_20231027']['text'])
        for cid in ('za_pac_court_declares_limpopo_election_invalid_20210823',
                    'za_pac_court_declares_bloemfontein_nec_lawful_20210823'):
            self.assertIn('court declaration', claims[cid]['uncertainty'], cid)
        self.assertIn('not an end of office', claims['za_pac_statement_moloto_suspended_20190720']['uncertainty'])
        self.assertIn('never a start', claims['za_pac_court_consent_order_moloto_acknowledged_president_20190308']['uncertainty'])
        for cid in ('za_pac_court_mphahlele_claims_presidency_2016', 'za_pac_release_mbinda_claims_to_be_legitimate_pac_20180304'):
            self.assertIn('never resolved by inference', claims[cid]['uncertainty'], cid)
        # Conflicts inside the records stay open: the PAC's two lists, and the two printed days of one judgment.
        self.assertIn('Bishop L. Mokgoba (1999 - 2003)', claims['za_pac_previous_leaders_list_2008']['text'])
        self.assertIn('Stanley Mogoba President 1996 - 2003', claims['za_pac_previous_presidents_list_2016']['text'])
        for cid in ('za_pac_court_mphahlele_claims_presidency_2016', 'za_pac_court_leave_to_appeal_refused_mphahlele_2016'):
            self.assertEqual(self.rows[cid]['printed_range'], 'judgment headed 21/6/2016, date of judgment 20 June 2016')
        self.assertIsNone(self.sources['za_pac_saflii_gphc_2016_485']['published_date'])
        self.assertEqual(self.rows['za_pac_home_page_pheko_elected_president_15_june']['printed_range'],
                         '15 June (no year printed)')
        # State and parliamentary offices in the same documents never feed the party office.
        for cid, phrase in (
                ('za_pac_release_president_mogoba_19981103', 'parliamentary seat is a state office'),
                ('za_pac_statement_president_mbinda_sworn_in_mp_2015', 'state office'),
                ('za_pac_post_president_nyhontso_20210719', 'state-office matter'),
                ('za_pac_post_former_president_mbinda_expelled_2017', 'state office'),
                ('za_pac_statement_president_mphahlele_20080920', 'no za_presidency claim is fed')):
            self.assertIn(phrase, claims[cid]['uncertainty'], cid)
        self.assertEqual(self.rows['za_pac_statement_president_mbinda_sworn_in_mp_2015']['event_kind'],
                         'in_office_continuation_attestation')
        scope = self.role['scope_note']
        for phrase in ('no za_presidency claim or source feeds it', 'Do not fill the interval', 'marked disputed',
                       'never resolved by inference', "infer an outgoing holder's last day", 'so no holder has a start'):
            self.assertIn(phrase, scope)
        self.assertEqual(self.org['coverage']['unresolved'][:3], [
            'Reconcile legal registration, precise organization identity, renames, alliances, mergers and splits against original records.',
            'Research all independently dated party, parliamentary and executive leadership roles before character or succession eligibility.',
            'No game-party mapping is asserted. An electoral entry is not automatically a parliamentary caucus, coalition or umbrella affiliation.'])
        self.assertEqual(len(self.org['coverage']['unresolved']), 4)
        self.assertIn('(CLAUDE-C01-32, ', self.org['coverage']['unresolved'][3])
        self.assertIn('unknown lifecycle or empty game mapping', self.org['coverage']['unresolved'][3])
        # The packet note sits after the CLAUDE-C01-09 and CLAUDE-C01-30 notes and before the CLAUDE-C01-21 and
        # CLAUDE-C01-16 notes, which their own tests pin as the last two entries.
        unresolved = self.packet['coverage']['unresolved']
        self.assertEqual(sum('CLAUDE-C01-32' in u for u in unresolved), 1)
        self.assertTrue(unresolved[-5].startswith('Heads of state 1990-2024 (CLAUDE-C01-09'))
        self.assertTrue(unresolved[-4].startswith('ACDP, Freedom Front and IFP leaders 1990-2026 (CLAUDE-C01-30'))
        self.assertTrue(unresolved[-3].startswith('PAC Presidents 1990-2026 (CLAUDE-C01-32'))
        self.assertTrue(unresolved[-2].startswith('Deputy Presidents 1994-2026 (CLAUDE-C01-21'))
        self.assertTrue(unresolved[-1].startswith('ANC Presidents 1990-2026 (CLAUDE-C01-16'))
        for earlier in ('CLAUDE-C01-09', 'CLAUDE-C01-16', 'CLAUDE-C01-21', 'CLAUDE-C01-30'):
            self.assertEqual(sum(earlier in u for u in unresolved), 1, earlier)

    def test_extracts_match_packet_claims_and_record_original_responses(self):
        for sid in NEW_SOURCES:
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual(extract['format'], 'spheres-c01-derived-factual-table/v1')
            self.assertEqual((extract['source_id'], extract['source_url'], extract['original_url']),
                             (sid, source['url'], source['original_url']))
            for key in ('scope_note', 'access_method', 'published_date', 'rights_note'):
                self.assertEqual(extract[key], source[key], (sid, key))
            self.assertEqual((extract['accessed_date'], source['accessed_date']), ('2026-09-28', '2026-09-28'))
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
            if sid in COURT_SOURCES:
                self.assertIn('as published by SAFLII', extract['provenance_note'])
                self.assertEqual(source['source_type'], 'court_judgment_archived')
            if sid in X_SOURCES:
                self.assertIn('@MyPAConline', extract['provenance_note'])
                self.assertEqual(source['source_type'], 'party_social_media_post_archived')
            self.assertIn('No open license, portrait permission or likeness approval', extract['rights_note'])
            self.assertEqual(extract['visual_review']['pdf_pages_one_based'], PDF_PAGES.get(sid, []))
            self.assertTrue(source['source_type'] and source['scope_note'] and source['publisher'])
            snapshot = source['snapshot']
            self.assertTrue(snapshot['path'].startswith('docs/campaign-certification/C01/research/sources/south-africa-pac-'))
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
            # Every source is a raw Internet Archive capture of the PAC's own pages, a SAFLII judgment or the PAC's
            # X account, made before the cutoff.
            match = ARCHIVED_URL.match(source['url'])
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
            self.assertEqual(host == 'www.saflii.org', sid in COURT_SOURCES, sid)
            self.assertEqual(host == 'twitter.com', sid in X_SOURCES, sid)
        self.assertEqual({cid: (row['attested_on'], row['event_kind']) for cid, row in self.rows.items()}, EVENTS)
        self.assertEqual({cid for cid, row in self.rows.items() if row['attested_on'] is None}, set(UNDATED))

    def test_secondary_leads_and_per_request_pages_stay_out_of_the_packet(self):
        for sid in NEW_SOURCES:
            url = self.sources[sid]['url']
            self.assertEqual(urlsplit(url).hostname, 'web.archive.org', sid)
            self.assertIsNone(PER_REQUEST_URL.search(url), sid)
            for marker in LEAD_URL_MARKERS:
                self.assertNotIn(marker, url, sid)
        lowered = self.raw.lower()
        for marker in ('wikipedia', 'sahistory', 'britannica', 'news24', 'iol.co.za', 'dailymaverick', 'ewn.co.za',
                       'sabcnews', 'lawlibrary', 'justice.gov.za', 'disa.ukzn', 'omalley'):
            self.assertNotIn(marker, lowered, marker)
        leads, added = self.section('Leads not imported'), self.section('Sources added')
        for marker in ('justice.gov.za/trc', 'lawlibrary.org.za', 'ZAGPPHC/2016/250', 'ZAGPPHC/2022/78', 'id=93',
                       'id=115', 'president-mbinda-lambast', 'PAC-Manifesto-2019.pdf', 'pacofazania.org.za/leadership',
                       '2000103433840349382', 'digitallibrary.un.org', 'wikipedia.org'):
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

        validator_cases = [
            (lambda p: source(p, 'za_pac_saflii_gphc_2021_539')['snapshot'].update(sha256='0' * 64), 'checksum mismatch'),
            (lambda p: source(p, 'za_pac_x_manifesto_launch_20260829')['snapshot'].update(bytes=1), 'checksum mismatch'),
            (lambda p: holder(p, 15).update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: claim(p, 'za_pac_x_president_nyhontso_manifesto_launch_20260829').update(attested_on='2026-09-08'),
             'exceeds cutoff'),
            (lambda p: holder(p, 12)['claim_ids'].append('za_pac_court_bloemfontein_congress_elects_nyhontso_2019'),
             'cited source'),
            (lambda p: role(p)['claim_ids'].append('za_pac_does_not_exist'), 'Unknown'),
            (lambda p: org(p).update(represented_party_ids=['SouthAfrica/guessed_pac']), 'foreign represented party'),
        ]
        for change, message in validator_cases:
            with self.subTest(message=message), self.assertRaisesRegex(ValueError, message):
                self.validate(mutated(change))
        invariant_cases = [
            ('successor start used as an end (Mogoba)', lambda p: holder(p, 3).update(until='2008-09-20')),
            ('successor start used as an end (Mphahlele)', lambda p: holder(p, 6).update(until='2014-03-21')),
            ('successor start used as an end (Mbinda)', lambda p: holder(p, 8).update(until='2018-01-19')),
            ('successor start used as an end (Moloto)', lambda p: holder(p, 11).update(until='2020-02-15')),
            ('expulsion month used as an end (Mphahlele)', lambda p: holder(p, 6).update(until='2013-05-01')),
            ('expulsion effective day used as an end (Mbinda)', lambda p: holder(p, 8).update(until='2017-06-13')),
            ('suspension used as an end (Moloto)', lambda p: holder(p, 11).update(until='2019-07-20')),
            ('court declaration used as an end (Moloto)', lambda p: holder(p, 11).update(until='2021-08-23')),
            ('court finding used as an end (Moloto)', lambda p: holder(p, 11).update(until='2023-10-27')),
            ('election used as start without stated assumption (Nyhontso)',
             lambda p: holder(p, 12).update({'from': '2019-08-29'})),
            ('election used as start without stated assumption (Mbinda)',
             lambda p: holder(p, 8).update({'from': '2014-09-28'})),
            ('election used as start without stated assumption (Mphahlele)',
             lambda p: holder(p, 4).update({'from': '2006-09-25'})),
            ('consent order used as a start (Moloto)', lambda p: holder(p, 11).update({'from': '2019-03-08'})),
            ('in-office observation used as start (Nyhontso 2026)', lambda p: holder(p, 15).update({'from': '2026-08-29'})),
            ('result publication used as observation (Nyhontso)', lambda p: holder(p, 12).update(attested_on='2019-09-01')),
            ('other office used as observation (Moloto)', lambda p: holder(p, 9).update(attested_on='2017-10-04')),
            ('acting holder added', lambda p: role(p)['holder_claims'].insert(7, {
                'name': 'Acting President of the PAC', 'attested_on': None, 'from': None, 'until': None,
                'sources': ['za_pac_home_page_2013_acting'],
                'claim_ids': ['za_pac_home_page_acting_president_mpheti_2013'], 'note': 'Observed on',
                'uncertainty': 'No start.'})),
            ('acting service added as a holder (Mphethi)', lambda p: role(p)['holder_claims'].insert(7, {
                'name': 'Alton Mphethi', 'attested_on': None, 'from': None, 'until': None,
                'sources': ['za_pac_home_page_2013_acting'],
                'claim_ids': ['za_pac_home_page_acting_president_mpheti_2013'], 'note': 'Observed on',
                'uncertainty': 'No start.'})),
            ('Secretary General added as a holder (Moloto)', lambda p: role(p)['holder_claims'].insert(9, {
                'name': 'Narius Moloto', 'attested_on': '2017-10-04', 'from': None, 'until': None,
                'sources': ['za_pac_interview_moloto_secretary_general_20171004'],
                'claim_ids': ['za_pac_interview_secretary_general_moloto_20171004'], 'note': 'Observed on',
                'uncertainty': 'No start.'})),
            ('rival claim cited by a holder (Mbinda)', lambda p: (
                holder(p, 9)['claim_ids'].append('za_pac_release_mbinda_claims_to_be_legitimate_pac_20180304'),
                holder(p, 9)['sources'].append('za_pac_release_fraud_case_mbinda_20180304'))),
            ('mixed state-office attestation cited by a holder (Mbinda)', lambda p: (
                holder(p, 8)['claim_ids'].append('za_pac_statement_president_mbinda_sworn_in_mp_2015'),
                holder(p, 8)['sources'].append('za_pac_statement_iec_20160629'))),
            ('dispute marker removed (Nyhontso 2020)', lambda p: holder(p, 12).update(
                uncertainty=holder(p, 12)['uncertainty'].replace('Disputed:', 'Contested:'))),
            ('dispute resolved by inference (Mphethi)', lambda p: holder(p, 7).update(
                uncertainty=holder(p, 7)['uncertainty'].split(' Disputed:')[0])),
            ('dispute marker added (Nyhontso 2026)', lambda p: holder(p, 15).update(
                uncertainty=holder(p, 15)['uncertainty'] + ' Disputed: invented.')),
            ('ANC claim fed into the PAC role', lambda p: (
                role(p)['claim_ids'].append('za_anc_nec_reaffirms_ramaphosa_anc_president_20260515'),
                role(p)['sources'].append('za_anc_sg_statement_special_nec_20260515'))),
            ('PAC claim fed into the IFP role', lambda p: (
                role(p, 'za_iec_n2024_034')['claim_ids'].append('za_pac_x_president_nyhontso_manifesto_launch_20260829'),
                role(p, 'za_iec_n2024_034')['sources'].append('za_pac_x_manifesto_launch_20260829'))),
            ('state-office claim fed into the PAC role', lambda p: (
                role(p)['claim_ids'].append('za_ramaphosa_president_elect_20240614'),
                role(p)['sources'].append('za_parliament_president_elect_20240614'))),
            ('presidency holder added to the PAC role', lambda p: role(p)['holder_claims'].append(
                copy.deepcopy(president_role(p)['holder_claims'][8]))),
            ('PAC holder added to the Presidency', lambda p: president_role(p)['holder_claims'].append(
                copy.deepcopy(holder(p, 15)))),
            ('PAC claim moved onto the ANC role', lambda p: (
                role(p, ANC_ID)['claim_ids'].append('za_pac_release_president_mbinda_20160131'),
                role(p, ANC_ID)['sources'].append('za_pac_release_strategic_plan_20160131'))),
            ('constitution claim moved onto the role', lambda p: role(p)['claim_ids'].append(
                'za_pac_constitution_amendments_1990_1992_1996_2000')),
            ('constitution year used as a lifecycle start', lambda p: org(p)['lifecycle'].update({'from': '1990-01-01'})),
            ('identity merged by name', lambda p: org(p).update(reconciled_organization_id='SouthAfrica/pac')),
            ('role copied to another party', lambda p: p['organizations'][0]['roles'].append(copy.deepcopy(role(p)))),
            ('second PAC role', lambda p: org(p)['roles'].append(dict(copy.deepcopy(role(p)), id='za_pac_deputy_president'))),
            ('PAC role removed', lambda p: org(p)['roles'].clear()),
            ('congress span given a date', lambda p: claim(p, 'za_pac_court_limpopo_congress_elects_moloto_2019').update(
                attested_on='2019-08-24')),
            ('conflicting judgment day given a date', lambda p: claim(p, 'za_pac_court_leave_to_appeal_refused_mphahlele_2016')
             .update(attested_on='2016-06-21')),
            ('holder order changed', lambda p: role(p)['holder_claims'].reverse()),
            ('ACDP holder changed', lambda p: role(p, 'za_iec_n2024_008')['holder_claims'][0].update(attested_on='1999-05-02')),
        ]
        pac_invariants(self.packet)
        for label, change in invariant_cases:
            with self.subTest(label=label), self.assertRaises((AssertionError, KeyError, IndexError, ValueError)):
                pac_invariants(mutated(change))

    def test_report_and_handoff_close_no_parent_gate(self):
        self.assertIn('ready_for_review', self.report)
        table = self.section('Outcome')
        for review, decision in DECISIONS.items():
            row, = [line for line in table.splitlines() if line.startswith(f'| {review} ')]
            self.assertIn(f'**{decision}:**', row)
        self.assertEqual(set(DECISIONS), set(REVIEW))
        for marker in ('C01', 'C06', 'S23', 'WC1', 'CP1'):
            self.assertNotRegex(self.report, rf'\b{marker}\b[^.\n]*\bis (now )?complete\b')
        notes = self.section('Integration notes')
        for text in ('a809bcba', '032cd6a3', 'claude/c01-za-30', 'research-index.json', 'test_south_africa_research_s10h.py',
                     'test_south_africa_heads_of_state_c01_09.py', 'test_south_africa_anc_presidents_c01_16.py',
                     'test_south_africa_deputy_presidents_c01_21.py', 'test_south_africa_party_leaders_c01_30.py',
                     'test_campaign_census'):
            self.assertIn(text, notes)
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        for text in ('ready_for_review', 'south-africa-pac-presidents-1990-2026-32.md', 'claude/c01-za-32', 'a809bcba',
                     'claude/c01-za-30', 'test_south_africa_pac_presidents_c01_32.py', 'disputed'):
            self.assertIn(text, handoff)
        index = research.build()
        country = next(p for p in index['countries'] if p['nation'] == 'SouthAfrica')
        self.assertFalse(country['country_census_complete'])
        self.assertIsNone(country['unrepresented_organization_count'])
        self.assertEqual((country['role_observations'], country['source_claims']), (11, 471))
        self.assertEqual(country['mapping_pending'], 53)
        self.assertEqual({w['status'] for w in index['work_orders'] if w['nation'] == 'SouthAfrica'}, {'open'})
        self.assertFalse(index['c01_complete'])


if __name__ == '__main__':
    unittest.main()
