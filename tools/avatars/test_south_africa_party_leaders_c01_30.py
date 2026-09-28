"""CLAUDE-C01-30: the national leaders of the ACDP, the Freedom Front / Freedom Front Plus and the IFP, 1990-2026, are
three party offices kept apart from each other, from the ANC and DA roles and from state office. Elections, selections,
acceptances, result publications, retirement and valedictory statements, the President Emeritus title, deaths, the
parliamentary leader office and continuation attestations stay separate claims; founding, renaming and merger claims sit
on the organization and set no lifecycle or identity; every holder is a dated in-office observation with no start or
end under this packet's conservative evidence-selection decision; dated retirement statements remain separate claims."""
import copy
import hashlib
import json
import re
import unittest
from urllib.parse import urlsplit

import campaign_research as research
from test_south_africa_pac_presidents_c01_32 import RESPONSES as C01_32_RESPONSES

ANC_ID = 'za_iec_n2024_014'
# The CLAUDE-C01-16 ANC holders, unchanged by this packet (pinned literally to avoid a circular test import).
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
# The packet's sources before this packet: S10h, CLAUDE-C01-09, CLAUDE-C01-16 and CLAUDE-C01-21 (pinned in their tests).
EARLIER_SOURCE_COUNT = 168
LIFECYCLE_NOTE = 'These observations do not establish founding, dissolution, legal continuity, mergers, splits or exact terms.'
FF_TITLE = 'Leader of the Freedom Front (Vryheidsfront)'
FF_PARLIAMENTARY = 'Parliamentary leader of the Freedom Front Plus (parlementêre leier)'
IFP_EMERITUS = 'President Emeritus of the Inkatha Freedom Party'

# Original response identity recorded in each extract, (bytes, sha256), fetched as the extract's fetch_recipe says:
# the raw id_ capture, no Accept-Encoding header, no decoding. Every source is a raw Internet Archive capture.
RESPONSES = {
    'za_acdp_party_history_page_2004': (5801, '900bc56afe15a382280bcdec50c16693579e64e31eed1000438eabe6cea3bb1a'),
    'za_acdp_kzn_newsletter_199805': (28926, '6a8775063f3b78d6d740788b751ab9da74e4915dc5979278dc24404bf38d40ee'),
    'za_acdp_home_page_19990501': (15288, '3882d1c053c2909fdb5e09ed33cd15fc9859d53c8a92eeed82f4836b027d064f'),
    'za_acdp_meshoe_nepad_speech_20011031': (5392, '368641b66dfd518bfe35ba066235eff29f0c7466a160396c0d4123aeca29ddaf'),
    'za_acdp_manifesto_launch_notice_20140218': (46513, 'bd196b45b00a384845d0aaee864079795fdbc0dec500aef9b76099646ea09fdb'),
    'za_acdp_leader_statement_20180214': (35076, '0cbd8f55400ce012758b5da23ca8f86f70c256fa51e49431dd996115f5f7cee0'),
    'za_acdp_our_leadership_page_20190401': (33337, '3dac19935d484f9a3c3e98dd9f0e7f493819e6e6d8cafe8206ecadb98452d7b5'),
    'za_acdp_meshoe_profile_page_2021': (49801, '5c672e31974c569a5350b348ab03a9fe89d44bb653cdb1cd82ac2d306ba56ca7'),
    'za_acdp_president_statement_lekota_20260304': (289679, 'af26b87e6f98cd2b765b8cf6d95190e6fad367f4d2ef498793add83e4468ae3d'),
    'za_ff_viljoen_media_release_19970826': (2781, 'b600d12bbb2c5f1d0a7fc1262d94912cf35621e7f19fdc57a7b0c60c8ba0d4b3'),
    'za_ff_viljoen_profile_page_2003': (4891, 'adb70c46ba2729f0c3b7ccd271968262442f50aa957de68b59266c6ff85eac4c'),
    'za_ff_mulder_profile_page_af_2003': (6982, '9651ae1268c077de0ab4f62d83424be1f4a39b14f7a09920b68aa0e8bf659719'),
    'za_ff_mulder_profile_page_en_2003': (6383, 'f91b23fc94fcf36bfbe4af719ce0769eb43b7922a9093da899f93e3059155335'),
    'za_ff_mulder_budget_debate_speech_20010621': (8784, '6dc5f5871a56797ba1eed9fffb1a525819b0f51fcae7f22fe6b382a8c81ace7a'),
    'za_ff_vfplus_media_release_20030928': (9164, 'd08f47ee2046b7671d1675c065e585972ae9f9fd25afddbb07820c28c16a1502'),
    'za_ff_mulder_reelected_release_20131118': (25341, 'c802afdd6579a0078f7f78da969ec4398fe132d8735fc7e16580848980e935de'),
    'za_ff_mulder_not_available_release_20161112': (27527, '832b2613534bf70220832284d07f47480136699779e35b093e67b04c1defb446'),
    'za_ff_new_leadership_release_20161112': (24764, '696b390157bb07417a551a486fbee2a6d4a917d792a8f605eb320f039b95da94'),
    'za_ff_groenewald_farm_visit_release_20161227': (17511, '9074ae292ac8ac0008038ed47ed389230df1ec570df62d48f9fdc8b7b60ba7df'),
    'za_ff_groenewald_covid_item_20200402': (24703, '0f604a2289f730f2883fd8bdc5236028e68dd028ceb4db3416cfba4e43fd3355'),
    'za_ff_viljoen_death_release_20200403': (28106, '656c79b87d33b87adce694bbb9baa649600f7f624453f047da358473afb48ebc'),
    'za_ff_minister_post_release_20240722': (533669, 'd26d9397e3b32f5d8eea5b64a8c522c82bbbdcd837a92c1b935734420d28703f'),
    'za_ff_da_ff_leaders_release_20240822': (533105, '9f8ecd10e2d56276b13fc0192bd43b885fa117338c519e7681cc257e01804d1a'),
    'za_ff_presidency_budget_debate_item_20250716': (352158, '5bfd2c09e79b7b515099b63ee1684c363c4fb0979f9aa7a9a2dcd9cd51b10bb1'),
    'za_ff_new_members_release_20260325': (377918, '91fbac0ac112f0c25122a4415564b46092c935a63fdf1007b25aaaf410ff724c'),
    'za_ifp_history_profile_page_1997': (15061, '57d9b3b99c4643d863c598c04e1020bb4e735e66e810c8a03644b1224cb1b72e'),
    'za_ifp_historical_perspective_page_2000': (31247, 'fb01094acba41f0e6c418011c2eb4f7376a07df63c3fbd371c27dd46dec95c92'),
    'za_ifp_press_release_19951024': (3399, '1e52bd314a935dd879dac6db100c384bc4f63a76b82d6541ee8943d91c55279f'),
    'za_ifp_elective_conference_resolutions_2012': (90159, '1a69238ba505964deab806b7cb4d328068aacadb4e435adff3e47c1a2796480d'),
    'za_ifp_extended_nc_statement_buthelezi_20190120': (69503, '2d1ace2d5163aeacdd574cc730f82469a2f0e61ff524961ddb0b6d4c11b22302'),
    'za_ifp_conference_address_buthelezi_20190824': (131553, '4a17dae160556bb76bea97a0aa2353e1bbfe2e6680ec7caef58c8beab5af568f'),
    'za_ifp_conference_address_hlabisa_20190824': (92933, '42478aa14915a48507b77627926ed19c273866a2b547f9984249e829ef664de9'),
    'za_ifp_newly_elected_leadership_20190825': (79219, 'c4e8e66ac60124020fd8cce2f2e3c87b267fc7c803117e0b4b9255e4543f8f87'),
    'za_ifp_nec_statement_20190827': (80745, 'eb77e03fbfc72ef4968a27a6f50eefdb28dc02ecc2eb45f391ccd3712884be9d'),
    'za_ifp_rally_advisory_20190912': (74055, '842ae7e80db08a7cd8b07fd3ded9299af83d03cf0b0bab1e9a5fe67d0115f575'),
    'za_ifp_statement_passing_buthelezi_20230909': (21125, 'db02a7f143dd2bcc2088108dd78f367a4ba557e211ec47362554025cbd8453f0'),
    'za_ifp_buthelezi_timeline_page_2024': (322928, '5e9e423844f07e66000dccdb05fc192b7da1a0683d4e6add406fe2a6b2e29220'),
    'za_ifp_extended_nc_resolutions_20240810': (166100, 'dad6c274b6e65b7fd809f0bb88ca7b684ce7b7bdd73d00c51ef0c31095f7d6f3'),
    'za_ifp_rally_advisory_abaqulusi_20260201': (25268, '9a73beed532d374f6985e795d1d830cb3ba35b2d3e8ab268bd2859f042c3b7f2'),
}
# Capture timestamp of each raw id_ capture (all before the 7 September 2026 cutoff).
ARCHIVED = {
    'za_acdp_party_history_page_2004': '20040113080431',
    'za_acdp_kzn_newsletter_199805': '19990904001603',
    'za_acdp_home_page_19990501': '19990501020807',
    'za_acdp_meshoe_nepad_speech_20011031': '20060925055108',
    'za_acdp_manifesto_launch_notice_20140218': '20180113011359',
    'za_acdp_leader_statement_20180214': '20190509205419',
    'za_acdp_our_leadership_page_20190401': '20190401103633',
    'za_acdp_meshoe_profile_page_2021': '20211207181933',
    'za_acdp_president_statement_lekota_20260304': '20260304132154',
    'za_ff_viljoen_media_release_19970826': '19980613213641',
    'za_ff_viljoen_profile_page_2003': '20030504130710',
    'za_ff_mulder_profile_page_af_2003': '20030429154123',
    'za_ff_mulder_profile_page_en_2003': '20030904113907',
    'za_ff_mulder_budget_debate_speech_20010621': '20061007034931',
    'za_ff_vfplus_media_release_20030928': '20060929152114',
    'za_ff_mulder_reelected_release_20131118': '20190511232924',
    'za_ff_mulder_not_available_release_20161112': '20190511235549',
    'za_ff_new_leadership_release_20161112': '20190511235549',
    'za_ff_groenewald_farm_visit_release_20161227': '20170511222502',
    'za_ff_groenewald_covid_item_20200402': '20211201072721',
    'za_ff_viljoen_death_release_20200403': '20200814200230',
    'za_ff_minister_post_release_20240722': '20240803012225',
    'za_ff_da_ff_leaders_release_20240822': '20240904150336',
    'za_ff_presidency_budget_debate_item_20250716': '20250722073536',
    'za_ff_new_members_release_20260325': '20260325202348',
    'za_ifp_history_profile_page_1997': '19970702032307',
    'za_ifp_historical_perspective_page_2000': '20000930003618',
    'za_ifp_press_release_19951024': '19970702040121',
    'za_ifp_elective_conference_resolutions_2012': '20190512041138',
    'za_ifp_extended_nc_statement_buthelezi_20190120': '20190123104637',
    'za_ifp_conference_address_buthelezi_20190824': '20190827151121',
    'za_ifp_conference_address_hlabisa_20190824': '20190827150556',
    'za_ifp_newly_elected_leadership_20190825': '20190827151253',
    'za_ifp_nec_statement_20190827': '20190827151236',
    'za_ifp_rally_advisory_20190912': '20190913050337',
    'za_ifp_statement_passing_buthelezi_20230909': '20230909123509',
    'za_ifp_buthelezi_timeline_page_2024': '20240403000605',
    'za_ifp_extended_nc_resolutions_20240810': '20240911231127',
    'za_ifp_rally_advisory_abaqulusi_20260201': '20260217210214',
}
# Captures the archive stores and serves gzip-encoded: decoded identity (bytes, sha256), recorded beside the served one.
GZIP = {
    'za_ifp_statement_passing_buthelezi_20230909': (100551, 'fd4e39bc3bfacd90d965444ac057cdb3986c6d0d75d2d7703cbbd5b9e90e6ae7'),
    'za_ifp_rally_advisory_abaqulusi_20260201': (138887, '04bc565b4d05e38316b173dfcea46a437e1f9644c8faf7ce94d2e466bc0ed268'),
}
# Every new claim's (attested_on, event_kind), exactly: distinct dated events are never re-dated or relabelled.
EVENTS = {
    'za_acdp_history_party_launched_december_1993': (None, 'founding_retrospective'),
    'za_acdp_kzn_newsletter_word_from_president_meshoe_199805': (None, 'in_office_attestation_month_only'),
    'za_acdp_home_page_president_meshoe_19990501': ('1999-05-01', 'in_office_attestation'),
    'za_acdp_meshoe_speech_as_president_20011031': ('2001-10-31', 'in_office_attestation'),
    'za_acdp_manifesto_notice_president_meshoe_20140218': ('2014-02-18', 'in_office_attestation'),
    'za_acdp_leader_meshoe_statement_20180214': ('2018-02-14', 'in_office_attestation'),
    'za_acdp_leadership_page_president_meshoe_20190401': ('2019-04-01', 'in_office_continuation_attestation'),
    'za_acdp_profile_meshoe_founded_party_december_1993': (None, 'founding_retrospective'),
    'za_acdp_president_meshoe_statement_20260304': ('2026-03-04', 'in_office_attestation'),
    'za_ff_viljoen_media_release_leader_19970826': ('1997-08-26', 'in_office_attestation'),
    'za_ff_viljoen_profile_founded_freedom_front': (None, 'founding_retrospective'),
    'za_ff_viljoen_profile_retired_as_leader_2001': (None, 'retirement_retrospective'),
    'za_ff_mulder_cv_af_freedom_front_founded_199403': (None, 'founding_retrospective'),
    'za_ff_mulder_cv_af_elected_leader_200104': (None, 'election_reference_retrospective'),
    'za_ff_mulder_cv_en_freedom_front_founded_199403': (None, 'founding_retrospective'),
    'za_ff_mulder_cv_en_elected_leader_200103': (None, 'election_reference_retrospective'),
    'za_ff_mulder_speech_newly_elected_party_leader_20010621': ('2001-06-21', 'in_office_attestation'),
    'za_ff_mulder_speech_elected_leader_in_march_2001': (None, 'election_reference'),
    'za_ff_release_mulder_vf_leader_20030928': ('2003-09-28', 'in_office_attestation'),
    'za_ff_vf_kp_aeb_single_unit_announced_20030927': ('2003-09-27', 'merger_announcement'),
    'za_ff_name_vryheidsfront_plus_announced_20030927': ('2003-09-27', 'renaming_announcement'),
    'za_ff_kp_aeb_leaders_pledge_loyalty_to_mulder_20030927': ('2003-09-27', 'leadership_acceptance_by_merging_parties'),
    'za_ff_mulder_unanimously_reelected_published_20131118': ('2013-11-18', 'result_publication'),
    'za_ff_release_mulder_vf_plus_leader_20161112': ('2016-11-12', 'in_office_attestation'),
    'za_ff_mulder_not_standing_again_20161112': ('2016-11-12', 'not_standing_again_announcement'),
    'za_ff_mulder_led_party_since_2001_unopposed': (None, 'election_reference_retrospective'),
    'za_ff_groenewald_designated_new_leader_20161112': ('2016-11-12', 'selection_by_congress'),
    'za_ff_release_leader_groenewald_20161227': ('2016-12-27', 'in_office_attestation'),
    'za_ff_item_leader_groenewald_20200402': ('2020-04-02', 'in_office_attestation'),
    'za_ff_passing_of_former_leader_viljoen_reported_20200403': ('2020-04-03', 'death_reported'),
    'za_ff_viljoen_very_first_leader_of_the_party': (None, 'predecessor_reference'),
    'za_ff_release_vf_plus_leader_groenewald_appointed_minister_20240722': ('2024-07-22', 'in_office_continuation_attestation'),
    'za_ff_corne_mulder_designated_parliamentary_leader_20240722': ('2024-07-22', 'other_office_designation'),
    'za_ff_joint_release_national_party_leader_groenewald_20240822': ('2024-08-22', 'in_office_attestation'),
    'za_ff_item_leader_corne_mulder_20250716': ('2025-07-16', 'in_office_attestation'),
    'za_ff_release_leader_corne_mulder_20260325': ('2026-03-25', 'in_office_attestation'),
    'za_ff_referendum_party_dissolves_into_ff_plus_20260325': ('2026-03-25', 'other_party_dissolves_and_joins'),
    'za_ifp_profile_inkatha_became_party_19900714': (None, 'renaming_retrospective'),
    'za_ifp_history_party_came_into_being_19900714': (None, 'renaming_retrospective'),
    'za_ifp_history_buthelezi_elected_president_1990': (None, 'election_reference_retrospective'),
    'za_ifp_press_release_president_buthelezi_19951024': ('1995-10-24', 'in_office_attestation'),
    'za_ifp_2012_conference_session_dates': (None, 'conference_session_dates'),
    'za_ifp_2012_conference_acclaims_buthelezi_reelection_20121216': ('2012-12-16', 'election_reference'),
    'za_ifp_2012_resolutions_mandate_president_buthelezi_20121216': ('2012-12-16', 'in_office_attestation'),
    'za_ifp_statement_president_buthelezi_20190120': ('2019-01-20', 'in_office_attestation'),
    'za_ifp_buthelezi_recalls_october_2017_announcement': (None, 'retirement_announcement_retrospective'),
    'za_ifp_extended_nc_conference_after_2019_elections_20190120': ('2019-01-20', 'conference_scheduling_decision'),
    'za_ifp_conference_address_president_buthelezi_20190824': ('2019-08-24', 'in_office_attestation'),
    'za_ifp_buthelezi_time_as_president_finished_20190824': ('2019-08-24', 'valedictory_statement'),
    'za_ifp_extended_nc_single_candidate_for_president_20190824': ('2019-08-24', 'nomination_reference'),
    'za_ifp_hlabisa_accepts_election_20190824': ('2019-08-24', 'election_acceptance_statement'),
    'za_ifp_conference_declares_buthelezi_president_emeritus_20190824': ('2019-08-24', 'honorary_title_resolution'),
    'za_ifp_conference_elected_hlabisa_president_published_20190825': ('2019-08-25', 'result_publication'),
    'za_ifp_former_president_buthelezi_made_president_emeritus_20190825': ('2019-08-25', 'predecessor_reference'),
    'za_ifp_35th_national_general_conference_dates': (None, 'conference_session_dates'),
    'za_ifp_advisory_president_hlabisa_20190912': ('2019-09-12', 'in_office_attestation'),
    'za_ifp_death_founder_president_emeritus_buthelezi_20230909': ('2023-09-09', 'death'),
    'za_ifp_statement_president_hlabisa_20230909': ('2023-09-09', 'in_office_attestation'),
    'za_ifp_timeline_inkatha_becomes_ifp_19900710': (None, 'renaming_retrospective'),
    'za_ifp_timeline_buthelezi_elected_president_1990': (None, 'election_reference_retrospective'),
    'za_ifp_timeline_buthelezi_retires_presidency_24_august': (None, 'retirement_retrospective'),
    'za_ifp_extended_nc_thanks_president_hlabisa_20240810': ('2024-08-10', 'in_office_attestation'),
    'za_ifp_extended_nc_conferences_due_20240810': ('2024-08-10', 'conference_scheduling_decision'),
    'za_ifp_advisory_president_hlabisa_20260201': ('2026-02-01', 'in_office_attestation'),
}
# Claims printed without a structured date: retrospective, month-only, year-only and span-only claims.
UNDATED = (
    'za_acdp_history_party_launched_december_1993', 'za_acdp_kzn_newsletter_word_from_president_meshoe_199805',
    'za_acdp_profile_meshoe_founded_party_december_1993', 'za_ff_viljoen_profile_founded_freedom_front',
    'za_ff_viljoen_profile_retired_as_leader_2001', 'za_ff_mulder_cv_af_freedom_front_founded_199403',
    'za_ff_mulder_cv_af_elected_leader_200104', 'za_ff_mulder_cv_en_freedom_front_founded_199403',
    'za_ff_mulder_cv_en_elected_leader_200103', 'za_ff_mulder_speech_elected_leader_in_march_2001',
    'za_ff_mulder_led_party_since_2001_unopposed', 'za_ff_viljoen_very_first_leader_of_the_party',
    'za_ifp_profile_inkatha_became_party_19900714', 'za_ifp_history_party_came_into_being_19900714',
    'za_ifp_history_buthelezi_elected_president_1990', 'za_ifp_2012_conference_session_dates',
    'za_ifp_buthelezi_recalls_october_2017_announcement', 'za_ifp_35th_national_general_conference_dates',
    'za_ifp_timeline_inkatha_becomes_ifp_19900710', 'za_ifp_timeline_buthelezi_elected_president_1990',
    'za_ifp_timeline_buthelezi_retires_presidency_24_august',
)
NEW_SOURCES = list(RESPONSES)
# The three chains. holders are (name, attested_on, from, until), one per reviewed observation; never_holder are the role
# claims that must never feed a holder; organization_only are founding, renaming, merger and joining claims, cited by the
# organization observation and never by the role; never_holder_date are days never used as any holder's observation,
# start or end.
CHAINS = {
    'acdp': {
        'obs': 'za_iec_n2024_008',
        'name': 'AFRICAN CHRISTIAN DEMOCRATIC PARTY',
        'row': 'national-results-page-1-row-8',
        'row_no': '008',
        'role': 'za_acdp_president',
        'title': 'President of the African Christian Democratic Party',
        'prefix': 'za_acdp',
        'holders': [
            ('Kenneth Meshoe', '1999-05-01', None, None),
            ('Kenneth Meshoe', '2001-10-31', None, None),
            ('Kenneth Meshoe', '2014-02-18', None, None),
            ('Kenneth Meshoe', '2018-02-14', None, None),
            ('Kenneth Meshoe', '2026-03-04', None, None),
        ],
        'holder_claims': [
            ['za_acdp_home_page_president_meshoe_19990501'],
            ['za_acdp_meshoe_speech_as_president_20011031'],
            ['za_acdp_manifesto_notice_president_meshoe_20140218'],
            ['za_acdp_leader_meshoe_statement_20180214'],
            ['za_acdp_president_meshoe_statement_20260304'],
        ],
        'never_holder': (
            'za_acdp_kzn_newsletter_word_from_president_meshoe_199805',
            'za_acdp_leadership_page_president_meshoe_20190401',
        ),
        'organization_only': (
            'za_acdp_history_party_launched_december_1993',
            'za_acdp_profile_meshoe_founded_party_december_1993',
        ),
        'never_holder_date': {'1993-12-01', '1994-01-17', '1999-06-02', '2014-02-22', '2019-04-01', '2026-06-26'},
        'review': ['ZA-ACDP-01', 'ZA-ACDP-02', 'ZA-ACDP-03', 'ZA-ACDP-04', 'ZA-ACDP-05'],
        'holder_review': ['ZA-ACDP-01', 'ZA-ACDP-02', 'ZA-ACDP-03', 'ZA-ACDP-04', 'ZA-ACDP-05'],
    },
    'ff': {
        'obs': 'za_iec_n2024_051',
        'name': 'VRYHEIDSFRONT PLUS',
        'row': 'national-results-page-2-row-14',
        'row_no': '051',
        'role': 'za_ff_leader',
        'title': 'Leader of the Freedom Front Plus (Vryheidsfront Plus)',
        'prefix': 'za_ff',
        'holders': [
            ('Constand Viljoen', '1997-08-26', None, None),
            ('Pieter Mulder', '2001-06-21', None, None),
            ('Pieter Mulder', '2003-09-28', None, None),
            ('Pieter Mulder', '2016-11-12', None, None),
            ('Pieter Groenewald', '2016-12-27', None, None),
            ('Pieter Groenewald', '2020-04-02', None, None),
            ('Pieter Groenewald', '2024-08-22', None, None),
            ('Corné Mulder', '2025-07-16', None, None),
            ('Corné Mulder', '2026-03-25', None, None),
        ],
        'holder_claims': [
            ['za_ff_viljoen_media_release_leader_19970826'],
            ['za_ff_mulder_speech_newly_elected_party_leader_20010621'],
            ['za_ff_release_mulder_vf_leader_20030928'],
            ['za_ff_release_mulder_vf_plus_leader_20161112'],
            ['za_ff_release_leader_groenewald_20161227'],
            ['za_ff_item_leader_groenewald_20200402'],
            ['za_ff_joint_release_national_party_leader_groenewald_20240822'],
            ['za_ff_item_leader_corne_mulder_20250716'],
            ['za_ff_release_leader_corne_mulder_20260325'],
        ],
        'never_holder': (
            'za_ff_viljoen_profile_retired_as_leader_2001',
            'za_ff_mulder_cv_af_elected_leader_200104',
            'za_ff_mulder_cv_en_elected_leader_200103',
            'za_ff_mulder_speech_elected_leader_in_march_2001',
            'za_ff_kp_aeb_leaders_pledge_loyalty_to_mulder_20030927',
            'za_ff_mulder_unanimously_reelected_published_20131118',
            'za_ff_mulder_not_standing_again_20161112',
            'za_ff_mulder_led_party_since_2001_unopposed',
            'za_ff_groenewald_designated_new_leader_20161112',
            'za_ff_passing_of_former_leader_viljoen_reported_20200403',
            'za_ff_viljoen_very_first_leader_of_the_party',
            'za_ff_release_vf_plus_leader_groenewald_appointed_minister_20240722',
            'za_ff_corne_mulder_designated_parliamentary_leader_20240722',
        ),
        'organization_only': (
            'za_ff_viljoen_profile_founded_freedom_front',
            'za_ff_mulder_cv_af_freedom_front_founded_199403',
            'za_ff_mulder_cv_en_freedom_front_founded_199403',
            'za_ff_vf_kp_aeb_single_unit_announced_20030927',
            'za_ff_name_vryheidsfront_plus_announced_20030927',
            'za_ff_referendum_party_dissolves_into_ff_plus_20260325',
        ),
        'never_holder_date': {'1994-03-01', '2001-03-01', '2001-04-01', '2003-09-27', '2013-11-18', '2020-04-03', '2024-07-22', '2025-02-22', '2026-03-26'},
        'review': ['ZA-FF-01', 'ZA-FF-02', 'ZA-FF-03', 'ZA-FF-04', 'ZA-FF-05', 'ZA-FF-06', 'ZA-FF-07', 'ZA-FF-08', 'ZA-FF-09'],
        'holder_review': ['ZA-FF-01', 'ZA-FF-02', 'ZA-FF-03', 'ZA-FF-04', 'ZA-FF-05', 'ZA-FF-06', 'ZA-FF-07', 'ZA-FF-08', 'ZA-FF-09'],
    },
    'ifp': {
        'obs': 'za_iec_n2024_034',
        'name': 'INKATHA FREEDOM PARTY',
        'row': 'national-results-page-1-row-34',
        'row_no': '034',
        'role': 'za_ifp_president',
        'title': 'President of the Inkatha Freedom Party',
        'prefix': 'za_ifp',
        'holders': [
            ('Mangosuthu Buthelezi', '1995-10-24', None, None),
            ('Mangosuthu Buthelezi', '2012-12-16', None, None),
            ('Mangosuthu Buthelezi', '2019-01-20', None, None),
            ('Mangosuthu Buthelezi', '2019-08-24', None, None),
            ('Velenkosini Hlabisa', '2019-09-12', None, None),
            ('Velenkosini Hlabisa', '2023-09-09', None, None),
            ('Velenkosini Hlabisa', '2024-08-10', None, None),
            ('Velenkosini Hlabisa', '2026-02-01', None, None),
        ],
        'holder_claims': [
            ['za_ifp_press_release_president_buthelezi_19951024'],
            ['za_ifp_2012_resolutions_mandate_president_buthelezi_20121216'],
            ['za_ifp_statement_president_buthelezi_20190120'],
            ['za_ifp_conference_address_president_buthelezi_20190824'],
            ['za_ifp_advisory_president_hlabisa_20190912'],
            ['za_ifp_statement_president_hlabisa_20230909'],
            ['za_ifp_extended_nc_thanks_president_hlabisa_20240810'],
            ['za_ifp_advisory_president_hlabisa_20260201'],
        ],
        'never_holder': (
            'za_ifp_history_buthelezi_elected_president_1990',
            'za_ifp_2012_conference_session_dates',
            'za_ifp_2012_conference_acclaims_buthelezi_reelection_20121216',
            'za_ifp_buthelezi_recalls_october_2017_announcement',
            'za_ifp_extended_nc_conference_after_2019_elections_20190120',
            'za_ifp_buthelezi_time_as_president_finished_20190824',
            'za_ifp_extended_nc_single_candidate_for_president_20190824',
            'za_ifp_hlabisa_accepts_election_20190824',
            'za_ifp_conference_declares_buthelezi_president_emeritus_20190824',
            'za_ifp_conference_elected_hlabisa_president_published_20190825',
            'za_ifp_former_president_buthelezi_made_president_emeritus_20190825',
            'za_ifp_35th_national_general_conference_dates',
            'za_ifp_death_founder_president_emeritus_buthelezi_20230909',
            'za_ifp_timeline_buthelezi_elected_president_1990',
            'za_ifp_timeline_buthelezi_retires_presidency_24_august',
            'za_ifp_extended_nc_conferences_due_20240810',
        ),
        'organization_only': (
            'za_ifp_profile_inkatha_became_party_19900714',
            'za_ifp_history_party_came_into_being_19900714',
            'za_ifp_timeline_inkatha_becomes_ifp_19900710',
        ),
        'never_holder_date': {'1975-03-21', '1990-07-10', '1990-07-14', '2012-12-14', '2012-12-15', '2017-10-29', '2019-08-23', '2019-08-25', '2019-08-26', '2019-08-27', '2019-09-15', '2023-09-10'},
        'review': ['ZA-IFP-01', 'ZA-IFP-02', 'ZA-IFP-03', 'ZA-IFP-04', 'ZA-IFP-05', 'ZA-IFP-06', 'ZA-IFP-07', 'ZA-IFP-08'],
        'holder_review': ['ZA-IFP-01', 'ZA-IFP-02', 'ZA-IFP-03', 'ZA-IFP-04', 'ZA-IFP-05', 'ZA-IFP-06', 'ZA-IFP-07', 'ZA-IFP-08'],
    },
}
HOLDER_KINDS = {'in_office_attestation'}
BOUNDARY_KINDS = {'assumption_of_office', 'end_of_term_statement', 'resignation_effective', 'oath_of_office'}
# Secondary leads and live pages that must never be a source: history sites, encyclopaedias, news.
LEAD_URL_MARKERS = ('sahistory', 'wikipedia', 'britannica', 'news24', 'iol.co.za', 'timeslive', 'dailymaverick',
                    'ewn.co.za', 'sabcnews', 'politicsweb', 'polity.org', 'reuters', 'bbc.', 'voanews', 'mg.co.za',
                    'nelsonmandela.org', 'citizen.co.za', 'netwerk24', 'maroelamedia')
# Page shapes a server can generate per request or that grow over time (search, API, listings, feeds, cache-busting).
PER_REQUEST_URL = re.compile(r'wp-json|cdx/search|cdn-cgi|email-protection|nocache|cachebust|[?&]s=|[?&]_=|[?&]cb=|'
                             r'/search[/?]|/feed|/embed|/page/\d|/tag/|/category/|download\.php|rnd=', re.I)
ARCHIVED_URL = re.compile(r'^https://web\.archive\.org/web/(\d{14})id_/(https?://(?:www\.)?(?:acdp\.org\.za|'
                          r'vryheidsfront\.co\.za|vfplus\.org\.za|ifp\.org\.za)(?::80)?/.*)$')
# Rows whose printed office is not the role's own title: the Freedom Front's name before the 2003 announcement, the
# parliamentary (caucus) leader, and the IFP's honorary title.
FF_TITLE_CLAIMS = (
    'za_ff_viljoen_media_release_leader_19970826', 'za_ff_viljoen_profile_retired_as_leader_2001',
    'za_ff_mulder_cv_af_elected_leader_200104', 'za_ff_mulder_cv_en_elected_leader_200103',
    'za_ff_mulder_speech_newly_elected_party_leader_20010621', 'za_ff_mulder_speech_elected_leader_in_march_2001',
    'za_ff_release_mulder_vf_leader_20030928', 'za_ff_passing_of_former_leader_viljoen_reported_20200403',
    'za_ff_viljoen_very_first_leader_of_the_party')
OTHER_TITLE_CLAIMS = {
    'za_ff_corne_mulder_designated_parliamentary_leader_20240722': FF_PARLIAMENTARY,
    'za_ifp_conference_declares_buthelezi_president_emeritus_20190824': IFP_EMERITUS,
    'za_ifp_death_founder_president_emeritus_buthelezi_20230909': IFP_EMERITUS,
}
PEOPLE = {'Kenneth Meshoe': 'Meshoe', 'Constand Viljoen': 'Viljoen', 'Pieter Mulder': 'Mulder',
          'Pieter Groenewald': 'Groenewald', 'Corné Mulder': 'Mulder', 'Mangosuthu Buthelezi': 'Buthelezi',
          'Velenkosini Hlabisa': 'Hlabisa'}
DECISIONS = {
    'ZA-ACDP-01': 'Accepted in part', 'ZA-ACDP-02': 'Accepted', 'ZA-ACDP-03': 'Accepted', 'ZA-ACDP-04': 'Accepted',
    'ZA-ACDP-05': 'Accepted',
    'ZA-FF-01': 'Accepted in part', 'ZA-FF-02': 'Accepted in part', 'ZA-FF-03': 'Accepted in part',
    'ZA-FF-04': 'Accepted in part', 'ZA-FF-05': 'Accepted in part', 'ZA-FF-06': 'Accepted',
    'ZA-FF-07': 'Accepted in part', 'ZA-FF-08': 'Accepted in part', 'ZA-FF-09': 'Accepted',
    'ZA-IFP-01': 'Accepted in part', 'ZA-IFP-02': 'Accepted in part', 'ZA-IFP-03': 'Accepted in part',
    'ZA-IFP-04': 'Accepted in part', 'ZA-IFP-05': 'Accepted', 'ZA-IFP-06': 'Accepted', 'ZA-IFP-07': 'Accepted',
    'ZA-IFP-08': 'Accepted',
}
REPORT = research.RESEARCH / 'south-africa-acdp-ff-ifp-leaders-1990-2026-30.md'
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-30.md'


def chain_claims(key):
    """Every new claim of a chain, in packet order."""
    return [cid for cid in EVENTS if cid.startswith(CHAINS[key]['prefix'] + '_')]


def party_invariants(packet):
    """Packet-level rules this test owns; raises AssertionError, KeyError, IndexError or ValueError on any violation."""
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    claim_source = {c['id']: s['id'] for s in packet['sources'] for c in s['claims']}
    orgs = {o['id']: o for o in packet['organizations']}
    assert len(packet['institutions']) == 1 and packet['institutions'][0]['id'] == 'za_presidency'
    presidency = packet['institutions'][0]
    # The presidency roles and the ANC and DA party roles are unchanged by this packet.
    assert [r['id'] for r in presidency['roles']] == ['za_president_election', 'za_state_president', 'za_deputy_president']
    assert sum(len(r['holder_claims']) for r in presidency['roles']) == 23
    assert [(h['name'], h['attested_on'], h['from'], h['until'])
            for h in orgs[ANC_ID]['roles'][0]['holder_claims']] == ANC_HOLDERS
    assert {o['id']: [r['id'] for r in o['roles']] for o in packet['organizations'] if o['roles']} == {
        'za_iec_n2024_008': ['za_acdp_president'], 'za_iec_n2024_014': ['za_anc_president'],
        'za_iec_n2024_027': ['za_da_federal_leader', 'za_da_federal_chair', 'za_da_council_chair'],
        'za_iec_n2024_034': ['za_ifp_president'], 'za_iec_n2024_039': ['za_pac_president'],
        'za_iec_n2024_051': ['za_ff_leader']}
    others = [presidency, orgs[ANC_ID], orgs['za_iec_n2024_027']]
    other_claims, other_sources = set(), set()
    for e in others:
        other_claims |= set(e['claim_ids']) | {c for r in e['roles'] for c in r['claim_ids']} | {
            c for r in e['roles'] for h in r['holder_claims'] for c in h['claim_ids']}
        other_sources |= set(e['sources']) | {s for r in e['roles'] for s in r['sources']} | {
            s for r in e['roles'] for h in r['holder_claims'] for s in h['sources']}
    seen_claims, seen_sources = set(), set()
    for key, chain in CHAINS.items():
        org = orgs[chain['obs']]
        # The IEC reporting identity, unknown lifecycle and empty game mapping stay unchanged.
        assert (org['name'], org['kind']) == (chain['name'], 'national_ballot_party_reporting_identity'), key
        assert org['source_identifier']['value'] == chain['row'], key
        assert org['represented_party_ids'] == [] and org['reconciled_organization_id'] is None, key
        assert org['lifecycle'] == {'status': 'unknown', 'from': None, 'until': None, 'note': LIFECYCLE_NOTE}, key
        assert [r['id'] for r in org['roles']] == [chain['role']], key
        role = org['roles'][0]
        assert (role['title'], role['kind']) == (chain['title'], 'party_leader'), key
        holders = role['holder_claims']
        assert all(isinstance(h, dict) for h in holders), key
        assert [(h['name'], h['attested_on'], h['from'], h['until']) for h in holders] == chain['holders'], key
        assert [h['claim_ids'] for h in holders] == chain['holder_claims'], key
        for h in holders:
            assert 'acting' not in h['name'].lower() and 'interim' not in h['name'].lower(), h['name']
            assert h['from'] is None and h['until'] is None, (key, h['name'])
            assert h['attested_on'] not in chain['never_holder_date'], (key, h['name'])
            assert not set(h['claim_ids']) & set(chain['never_holder']), (key, h['name'])
            assert not set(h['claim_ids']) & set(chain['organization_only']), (key, h['name'])
            expected = []
            for cid in h['claim_ids']:
                assert cid in role['claim_ids'] and cid.startswith(chain['prefix'] + '_'), cid
                assert claims[cid]['attested_on'] == h['attested_on'], cid
                if claim_source[cid] not in expected:
                    expected.append(claim_source[cid])
            assert h['sources'] == expected, (key, h['name'])
        # Organization claims (founding, renaming, merger, joining) sit on the observation, never on the role.
        assert not set(role['claim_ids']) & set(chain['organization_only']), key
        assert set(chain['organization_only']) <= set(org['claim_ids']), key
        role_claims = set(role['claim_ids']) | {c for h in holders for c in h['claim_ids']}
        role_sources = set(role['sources']) | {s for h in holders for s in h['sources']}
        assert all(claim_source[c] in role_sources for c in role_claims), key
        assert set(role['claim_ids']) <= set(org['claim_ids']) and set(role['sources']) <= set(org['sources']), key
        chain_ids = set(org['claim_ids']) - {f"za_n2024_ballot_{chain['row_no']}", f"za_n2024_seats_{chain['row_no']}"}
        chain_srcs = set(org['sources']) - {'za_iec_national_results_20240621', 'za_iec_national_seats_20240606'}
        assert chain_ids and all(c.startswith(chain['prefix'] + '_') for c in chain_ids), key
        assert chain_srcs and all(s.startswith(chain['prefix'] + '_') for s in chain_srcs), key
        assert role_claims <= chain_ids and role_sources <= chain_srcs, key
        # Party office and state office, the ANC and DA roles and the three chains never feed each other.
        assert not chain_ids & other_claims and not chain_srcs & other_sources, key
        assert not chain_ids & seen_claims and not chain_srcs & seen_sources, key
        seen_claims |= chain_ids
        seen_sources |= chain_srcs
        for entry in packet['organizations'] + packet['institutions']:
            if entry is not org:
                assert not set(entry['claim_ids']) & chain_ids and not set(entry['sources']) & chain_srcs, entry['id']
                assert not any(r['id'] == chain['role'] for r in entry['roles']), entry['id']
    assert not any(c.startswith(('za_acdp', 'za_ff', 'za_ifp')) for c in other_claims)
    # Undated claims carry no structured date at all, and distinct dated events keep their own day.
    for cid in UNDATED:
        assert not {'attested_on', 'attested_period', 'period'} & set(claims[cid]), cid
    for cid, (day, _kind) in EVENTS.items():
        assert claims[cid].get('attested_on') == day, cid


class SouthAfricaPartyLeadersTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.raw = (research.ROOT / research.RESEARCH / 'south-africa.json').read_text(encoding='utf-8')
        cls.packet = json.loads(cls.raw)
        cls.sources = {s['id']: s for s in cls.packet['sources']}
        cls.claims = {c['id']: c for s in cls.packet['sources'] for c in s['claims']}
        cls.claim_source = {c['id']: s['id'] for s in cls.packet['sources'] for c in s['claims']}
        cls.orgs = {o['id']: o for o in cls.packet['organizations']}
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

    def role(self, key):
        return self.orgs[CHAINS[key]['obs']]['roles'][0]

    def test_new_records_are_bounded_and_every_claim_is_classified(self):
        ids = self.validate()
        self.assertEqual((len(NEW_SOURCES), len(self.new_claims)), (39, 64))
        order = [s['id'] for s in self.packet['sources']]
        # CLAUDE-C01-32 appends its PAC sources after these, pinned in its own test.
        self.assertEqual(len(order), EARLIER_SOURCE_COUNT + len(NEW_SOURCES) + len(C01_32_RESPONSES))
        self.assertEqual(order[EARLIER_SOURCE_COUNT:EARLIER_SOURCE_COUNT + len(NEW_SOURCES)], NEW_SOURCES)
        self.assertEqual(order[EARLIER_SOURCE_COUNT + len(NEW_SOURCES):], list(C01_32_RESPONSES))
        self.assertFalse([sid for sid in order[:EARLIER_SOURCE_COUNT] if sid.startswith(('za_acdp', 'za_ff', 'za_ifp'))])
        # CLAUDE-C01-32 adds one party-leader role (za_pac_president).
        self.assertEqual((len(ids['entries']), len(ids['roles'])), (53, 11))
        self.assertEqual(list(EVENTS), self.new_claims)
        people = set()
        for key, chain in CHAINS.items():
            new = chain_claims(key)
            holder_claims = [cid for ids_ in chain['holder_claims'] for cid in ids_]
            groups = holder_claims + list(chain['never_holder']) + list(chain['organization_only'])
            # Every new claim is exactly one of: a holder claim, a role claim that never feeds a holder, or an
            # organization-only claim.
            self.assertEqual(len(groups), len(set(groups)), key)
            self.assertEqual(set(groups), set(new), key)
            role, org = self.role(key), self.orgs[chain['obs']]
            self.assertEqual(role['claim_ids'], [cid for cid in new if cid not in chain['organization_only']], key)
            chain_sources = [sid for sid in NEW_SOURCES if sid.startswith(chain['prefix'] + '_')]
            self.assertEqual(role['sources'], [sid for sid in chain_sources
                                               if set(c['id'] for c in self.sources[sid]['claims']) - set(chain['organization_only'])], key)
            base = [f"za_n2024_ballot_{chain['row_no']}", f"za_n2024_seats_{chain['row_no']}"]
            self.assertEqual(org['claim_ids'], base + new, key)
            self.assertEqual(org['sources'], ['za_iec_national_results_20240621', 'za_iec_national_seats_20240606']
                             + chain_sources, key)
            observations = re.findall(rf"^### ({chain['review'][0][:-3]}-\d\d)\b", self.report, re.M)
            self.assertEqual(observations, chain['review'], key)
            self.assertEqual({self.rows[cid]['review_observation'] for cid in new}, set(chain['review']), key)
            self.assertEqual([self.rows[ids_[0]]['review_observation'] for ids_ in chain['holder_claims']],
                             chain['holder_review'], key)
            people |= {h[0] for h in chain['holders']}
        # At most ten people in the packet: seven holders, and no row names anyone else as its holder.
        self.assertEqual(people, set(PEOPLE))
        self.assertLessEqual(len(people), 10)
        self.assertTrue({row['holder_name'] for row in self.rows.values()} <= set(PEOPLE) | {None})
        self.assertNotIn('"name":', json.dumps([self.extracts[sid]['rows'] for sid in NEW_SOURCES]))

    def test_holders_are_exactly_as_intended(self):
        party_invariants(self.packet)
        for key, chain in CHAINS.items():
            role = self.role(key)
            for index, holder in enumerate(role['holder_claims']):
                self.assertTrue(holder['note'].startswith('Observed on '), holder['name'])
                self.assertRegex(holder['uncertainty'], r'No start', holder['name'])
                for cid in holder['claim_ids']:
                    row = self.rows[cid]
                    self.assertIn(PEOPLE[holder['name']].lower(), self.claims[cid]['text'].lower(), cid)
                    self.assertEqual((row['holder_name'], row['role_id']), (holder['name'], chain['role']), cid)
                    self.assertIn(row['event_kind'], HOLDER_KINDS, cid)
                    self.assertEqual(row['review_observation'], chain['holder_review'][index], cid)
                    self.assertIn('Dates the holder observation', self.claims[cid]['uncertainty'], cid)
            for cid in chain['never_holder']:
                self.assertNotIn(self.rows[cid]['event_kind'], HOLDER_KINDS | BOUNDARY_KINDS, cid)
                self.assertEqual(self.rows[cid]['role_id'], chain['role'], cid)
            for cid in chain['organization_only']:
                row = self.rows[cid]
                self.assertEqual((row['role_id'], row['role_title']), (None, None), cid)
                self.assertRegex(row['event_kind'], r'founding|renaming|merger|joins', cid)
                self.assertIn('organization claim', self.claims[cid]['uncertainty'].lower(), cid)
            # Every role row carries the role's own title except the named exceptions.
            for cid in chain_claims(key):
                row = self.rows[cid]
                if cid in chain['organization_only']:
                    continue
                expected = OTHER_TITLE_CLAIMS.get(cid, FF_TITLE if cid in FF_TITLE_CLAIMS else chain['title'])
                self.assertEqual(row['role_title'], expected, cid)
        # The parliamentary leader office, the President Emeritus title and deaths never feed a holder.
        for cid in ('za_ff_corne_mulder_designated_parliamentary_leader_20240722',
                    'za_ifp_conference_declares_buthelezi_president_emeritus_20190824',
                    'za_ifp_death_founder_president_emeritus_buthelezi_20230909',
                    'za_ff_passing_of_former_leader_viljoen_reported_20200403'):
            self.assertIn(cid, CHAINS[cid.split('_')[1]]['never_holder'], cid)
        self.assertEqual(self.rows['za_ff_corne_mulder_designated_parliamentary_leader_20240722']['holder_name'],
                         'Corné Mulder')

    def test_no_start_or_end_is_stated_or_inferred(self):
        claims = self.claims
        for cid in UNDATED:
            self.assertIn('no structured date', claims[cid]['uncertainty'].lower(), cid)
            self.assertTrue(self.rows[cid]['printed_range'], cid)
        # Evidence that is recorded but never used as a boundary, each saying why.
        self.assertIn('never boundaries', claims['za_ifp_timeline_buthelezi_retires_presidency_24_august']['uncertainty'])
        self.assertIn('officially retires from the presidency of the Party',
                      claims['za_ifp_timeline_buthelezi_retires_presidency_24_august']['text'])
        self.assertIn('is finished', claims['za_ifp_buthelezi_time_as_president_finished_20190824']['text'])
        self.assertIn('no end is inferred', claims['za_ifp_buthelezi_time_as_president_finished_20190824']['uncertainty'])
        self.assertIn('last night', claims['za_ifp_conference_elected_hlabisa_president_published_20190825']['text'])
        self.assertIn('derived and not stored',
                      claims['za_ifp_conference_elected_hlabisa_president_published_20190825']['uncertainty'])
        self.assertIn('President Emeritus, not the party presidency',
                      claims['za_ifp_death_founder_president_emeritus_buthelezi_20230909']['uncertainty'])
        self.assertIn('the day of death itself is not printed',
                      claims['za_ff_passing_of_former_leader_viljoen_reported_20200403']['uncertainty'])
        self.assertIn('vandag aangewys', claims['za_ff_groenewald_designated_new_leader_20161112']['text'])
        self.assertIn('never a start', claims['za_ff_groenewald_designated_new_leader_20161112']['uncertainty'])
        self.assertIn('Voting Day is 32 days away!', claims['za_acdp_home_page_president_meshoe_19990501']['text'])
        self.assertIn('not by a dateline', claims['za_acdp_home_page_president_meshoe_19990501']['uncertainty'])
        # The dates of the 1990 transformation and the 2001 election conflict inside the parties' own records.
        self.assertEqual(self.rows['za_ifp_profile_inkatha_became_party_19900714']['printed_range'], '14 July 1990')
        self.assertEqual(self.rows['za_ifp_timeline_inkatha_becomes_ifp_19900710']['printed_range'], '10 July 1990')
        self.assertEqual(self.rows['za_ff_mulder_cv_af_elected_leader_200104']['printed_range'], 'April 2001')
        self.assertEqual(self.rows['za_ff_mulder_cv_en_elected_leader_200103']['printed_range'], 'March 2001')
        # State and legislature offices in the same documents never feed the party office.
        for cid, phrase in (
                ('za_ff_release_vf_plus_leader_groenewald_appointed_minister_20240722', 'state office'),
                ('za_ifp_statement_president_hlabisa_20230909', 'never feeds the party office'),
                ('za_ifp_hlabisa_accepts_election_20190824', 'state office'),
                ('za_ff_corne_mulder_designated_parliamentary_leader_20240722', 'different office'),
                ('za_acdp_leader_meshoe_statement_20180214', 'no za_presidency claim is fed')):
            self.assertIn(phrase, claims[cid]['uncertainty'], cid)
        for key, chain in CHAINS.items():
            role, org = self.role(key), self.orgs[chain['obs']]
            scope = role['scope_note']
            for phrase in ('no za_presidency claim or source feeds', 'never feed'):
                self.assertIn(phrase, scope, key)
            self.assertIn('so no holder has a start or an end' if key == 'acdp' else 'Do not fill the interval', scope)
            self.assertEqual(org['coverage']['unresolved'][:3], [
                'Reconcile legal registration, precise organization identity, renames, alliances, mergers and splits against original records.',
                'Research all independently dated party, parliamentary and executive leadership roles before character or succession eligibility.',
                'No game-party mapping is asserted. An electoral entry is not automatically a parliamentary caucus, coalition or umbrella affiliation.'], key)
            self.assertEqual(len(org['coverage']['unresolved']), 4, key)
            self.assertIn('(CLAUDE-C01-30, ', org['coverage']['unresolved'][3], key)
            self.assertIn('unknown lifecycle or empty game mapping', org['coverage']['unresolved'][3], key)
        # The packet note sits after the CLAUDE-C01-09 note and before the CLAUDE-C01-32, CLAUDE-C01-21 and CLAUDE-C01-16
        # notes; the last two are pinned by their own tests, and CLAUDE-C01-32 inserts its note between this one and them.
        unresolved = self.packet['coverage']['unresolved']
        self.assertEqual(sum('CLAUDE-C01-30' in u for u in unresolved), 1)
        self.assertTrue(unresolved[-5].startswith('Heads of state 1990-2024 (CLAUDE-C01-09'))
        self.assertTrue(unresolved[-4].startswith('ACDP, Freedom Front and IFP leaders 1990-2026 (CLAUDE-C01-30'))
        self.assertTrue(unresolved[-3].startswith('PAC Presidents 1990-2026 (CLAUDE-C01-32'))
        self.assertTrue(unresolved[-2].startswith('Deputy Presidents 1994-2026 (CLAUDE-C01-21'))
        self.assertTrue(unresolved[-1].startswith('ANC Presidents 1990-2026 (CLAUDE-C01-16'))
        for earlier in ('CLAUDE-C01-09', 'CLAUDE-C01-16', 'CLAUDE-C01-21'):
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
            if sid in GZIP:
                self.assertEqual(extract['source_response_content_encoding'], 'gzip')
                self.assertEqual((extract['decoded_response_bytes'], extract['decoded_response_sha256']), GZIP[sid])
                self.assertIn('gzip-encoded', source['scope_note'])
            else:
                self.assertEqual(extract['source_response_content_encoding'], 'identity')
                self.assertNotIn('decoded_response_sha256', extract)
                self.assertIn('without content encoding', source['scope_note'])
            self.assertIn('downloaded again', extract['stability_check'])
            self.assertIn('at least 30 minutes after the first', extract['stability_check'])
            self.assertIn('not checked into this repository', extract['provenance_note'])
            self.assertIn('derived factual extract', extract['provenance_note'])
            self.assertIn('Raw Internet Archive capture', extract['provenance_note'])
            self.assertIn('No open license, portrait permission or likeness approval', extract['rights_note'])
            self.assertEqual(extract['visual_review']['pdf_pages_one_based'], [])
            self.assertTrue(source['source_type'] and source['scope_note'] and source['publisher'])
            snapshot = source['snapshot']
            prefix = sid.split('_')[1]
            self.assertTrue(snapshot['path'].startswith(f'docs/campaign-certification/C01/research/sources/south-africa-{prefix}-'))
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
                chain = CHAINS[prefix]
                self.assertEqual(row['observation_id'], chain['obs'])
                self.assertIn(row['role_id'], (chain['role'], None))
                self.assertNotIn('name', row)
                self.assertIn(row['review_observation'], chain['review'])
                self.assertEqual('printed_range' in row, row['attested_on'] is None)
            # Every source is a raw Internet Archive capture of the party's own site made before the cutoff.
            match = ARCHIVED_URL.match(source['url'])
            self.assertTrue(match, sid)
            stamp = match.group(1)
            self.assertEqual(stamp, ARCHIVED[sid])
            self.assertLess(stamp, '20260907')
            self.assertEqual(extract['archive_capture_utc'].replace('-', '').replace(':', '').replace('T', '').rstrip('Z'),
                             stamp)
            self.assertEqual(source['original_url'], match.group(2).replace(':80/', '/'))
            self.assertNotIn(':80', source['original_url'])
            self.assertIn(urlsplit(source['original_url']).hostname, {
                'acdp': {'www.acdp.org.za', 'acdp.org.za'}, 'ff': {'www.vryheidsfront.co.za', 'www.vfplus.org.za'},
                'ifp': {'www.ifp.org.za', 'ifp.org.za'}}[prefix], sid)
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
        for marker in ('wikipedia', 'sahistory', 'britannica', 'news24', 'iol.co.za', 'dailymaverick', 'ewn.co.za'):
            self.assertNotIn(marker, lowered, marker)
        leads, added = self.section('Leads not imported'), self.section('Sources added')
        for marker in ('sahistory.org.za', 'wikipedia.org', 'iol.co.za', '20260326034627', '20260325190953',
                       'whos_who/presidency.htm', 'ifp-nec-voting-details', 'statement-by-hon-vf-hlabisa-mp-president',
                       'speach.asp?id=11', 'media.asp?id=6', 'kersboodskap', 'speechpresidentswelcome', 'ifpconf.htm',
                       'ifp-national-elective-conference-2019'):
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

        def org(packet, key):
            return next(o for o in packet['organizations'] if o['id'] == CHAINS[key]['obs'])

        def role(packet, key):
            return org(packet, key)['roles'][0]

        def holder(packet, key, index):
            return role(packet, key)['holder_claims'][index]

        def president_role(packet):
            return packet['institutions'][0]['roles'][0]

        def anc_role(packet):
            return next(o for o in packet['organizations'] if o['id'] == ANC_ID)['roles'][0]

        validator_cases = [
            (lambda p: source(p, 'za_ifp_buthelezi_timeline_page_2024')['snapshot'].update(sha256='0' * 64),
             'checksum mismatch'),
            (lambda p: source(p, 'za_acdp_home_page_19990501')['snapshot'].update(bytes=1), 'checksum mismatch'),
            (lambda p: holder(p, 'ff', 8).update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: claim(p, 'za_ifp_advisory_president_hlabisa_20260201').update(attested_on='2026-09-08'),
             'exceeds cutoff'),
            (lambda p: holder(p, 'ifp', 3)['claim_ids'].append('za_ifp_hlabisa_accepts_election_20190824'), 'cited source'),
            (lambda p: role(p, 'acdp')['claim_ids'].append('za_acdp_does_not_exist'), 'Unknown'),
            (lambda p: org(p, 'ff').update(represented_party_ids=['SouthAfrica/guessed_vf_plus']),
             'foreign represented party'),
        ]
        for change, message in validator_cases:
            with self.subTest(message=message), self.assertRaisesRegex(ValueError, message):
                self.validate(mutated(change))
        invariant_cases = [
            ('successor start used as an end (Viljoen)', lambda p: holder(p, 'ff', 0).update(until='2001-06-21')),
            ('successor start used as an end (Groenewald)', lambda p: holder(p, 'ff', 6).update(until='2025-07-16')),
            ('successor start used as an end (Buthelezi)', lambda p: holder(p, 'ifp', 3).update(until='2019-09-12')),
            ('successor election used as an end (Buthelezi)', lambda p: holder(p, 'ifp', 3).update(until='2019-08-25')),
            ('retrospective timeline retirement used as an end', lambda p: holder(p, 'ifp', 3).update(until='2019-08-24')),
            ('death of the President Emeritus used as an end', lambda p: holder(p, 'ifp', 3).update(until='2023-09-09')),
            ('non-candidacy used as an end (Mulder 2016)', lambda p: holder(p, 'ff', 3).update(until='2016-11-12')),
            ('designation used as start without stated assumption (Groenewald)',
             lambda p: holder(p, 'ff', 4).update({'from': '2016-11-12'})),
            ('election used as start without stated assumption (Hlabisa)',
             lambda p: holder(p, 'ifp', 4).update({'from': '2019-08-24'})),
            ('founding month used as start (Meshoe)', lambda p: holder(p, 'acdp', 0).update({'from': '1993-12-01'})),
            ('in-office observation used as start (Meshoe 2026)', lambda p: holder(p, 'acdp', 4).update({'from': '2026-03-04'})),
            ('result publication used as observation (Hlabisa)', lambda p: holder(p, 'ifp', 4).update(attested_on='2019-08-25')),
            ('acceptance used as observation (Hlabisa)', lambda p: holder(p, 'ifp', 4).update(attested_on='2019-08-24')),
            ('re-election publication used as observation (Mulder)', lambda p: holder(p, 'ff', 3).update(attested_on='2013-11-18')),
            ('acting holder added', lambda p: role(p, 'ifp')['holder_claims'].insert(4, {
                'name': 'Acting President of the IFP', 'attested_on': '2019-08-25', 'from': None, 'until': None,
                'sources': ['za_ifp_newly_elected_leadership_20190825'],
                'claim_ids': ['za_ifp_conference_elected_hlabisa_president_published_20190825']})),
            ('acceptance added as a holder', lambda p: role(p, 'ifp')['holder_claims'].insert(4, {
                'name': 'Velenkosini Hlabisa', 'attested_on': '2019-08-24', 'from': None, 'until': None,
                'sources': ['za_ifp_conference_address_hlabisa_20190824'],
                'claim_ids': ['za_ifp_hlabisa_accepts_election_20190824']})),
            ('parliamentary leader added as a holder', lambda p: role(p, 'ff')['holder_claims'].insert(7, {
                'name': 'Corné Mulder', 'attested_on': '2024-07-22', 'from': None, 'until': None,
                'sources': ['za_ff_minister_post_release_20240722'],
                'claim_ids': ['za_ff_corne_mulder_designated_parliamentary_leader_20240722']})),
            ('continuation claim cited by a holder (ACDP)', lambda p: (
                holder(p, 'acdp', 3)['claim_ids'].append('za_acdp_leadership_page_president_meshoe_20190401'),
                holder(p, 'acdp', 3)['sources'].append('za_acdp_our_leadership_page_20190401'))),
            ('mixed state-office attestation cited by a holder (Groenewald)', lambda p: (
                holder(p, 'ff', 6)['claim_ids'].append('za_ff_release_vf_plus_leader_groenewald_appointed_minister_20240722'),
                holder(p, 'ff', 6)['sources'].append('za_ff_minister_post_release_20240722'))),
            ('ANC claim fed into the IFP role', lambda p: (
                role(p, 'ifp')['claim_ids'].append('za_anc_nec_reaffirms_ramaphosa_anc_president_20260515'),
                role(p, 'ifp')['sources'].append('za_anc_sg_statement_special_nec_20260515'))),
            ('IFP claim fed into the Freedom Front role', lambda p: (
                role(p, 'ff')['claim_ids'].append('za_ifp_advisory_president_hlabisa_20260201'),
                role(p, 'ff')['sources'].append('za_ifp_rally_advisory_abaqulusi_20260201'))),
            ('state-office claim fed into the ACDP role', lambda p: (
                role(p, 'acdp')['claim_ids'].append('za_ramaphosa_president_elect_20240614'),
                role(p, 'acdp')['sources'].append('za_parliament_president_elect_20240614'))),
            ('presidency holder added to the IFP role', lambda p: role(p, 'ifp')['holder_claims'].append(
                copy.deepcopy(president_role(p)['holder_claims'][8]))),
            ('IFP holder added to the Presidency', lambda p: president_role(p)['holder_claims'].append(
                copy.deepcopy(holder(p, 'ifp', 7)))),
            ('Freedom Front claim moved onto the ANC role', lambda p: (
                anc_role(p)['claim_ids'].append('za_ff_item_leader_groenewald_20200402'),
                anc_role(p)['sources'].append('za_ff_groenewald_covid_item_20200402'))),
            ('organization merger claim moved onto the role', lambda p: role(p, 'ff')['claim_ids'].append(
                'za_ff_vf_kp_aeb_single_unit_announced_20030927')),
            ('renaming used to merge identities', lambda p: org(p, 'ff').update(reconciled_organization_id='SouthAfrica/ff')),
            ('founding used as a lifecycle start', lambda p: org(p, 'acdp')['lifecycle'].update({'from': '1993-12-01'})),
            ('renaming used as a lifecycle start', lambda p: org(p, 'ifp')['lifecycle'].update({'from': '1990-07-14'})),
            ('role copied to another party', lambda p: p['organizations'][0]['roles'].append(copy.deepcopy(role(p, 'acdp')))),
            ('second Freedom Front role', lambda p: org(p, 'ff')['roles'].append(
                dict(copy.deepcopy(role(p, 'ff')), id='za_ff_parliamentary_leader'))),
            ('IFP role removed', lambda p: org(p, 'ifp')['roles'].clear()),
            ('retrospective renaming given a date', lambda p: claim(p, 'za_ifp_history_party_came_into_being_19900714').update(
                attested_on='1990-07-14')),
            ('month-only election given a date', lambda p: claim(p, 'za_ff_mulder_cv_en_elected_leader_200103').update(
                attested_on='2001-03-01')),
            ('holder order changed', lambda p: role(p, 'ff')['holder_claims'].reverse()),
        ]
        party_invariants(self.packet)
        for label, change in invariant_cases:
            with self.subTest(label=label), self.assertRaises((AssertionError, KeyError, IndexError, ValueError)):
                party_invariants(mutated(change))

    def test_report_and_handoff_close_no_parent_gate(self):
        self.assertIn('ready_for_review', self.report)
        table = self.section('Outcome')
        for review, decision in DECISIONS.items():
            row, = [line for line in table.splitlines() if line.startswith(f'| {review} ')]
            self.assertIn(f'**{decision}:**', row)
        self.assertEqual(set(DECISIONS), {r for c in CHAINS.values() for r in c['review']})
        for marker in ('C01', 'C06', 'S23', 'WC1', 'CP1'):
            self.assertNotRegex(self.report, rf'\b{marker}\b[^.\n]*\bis (now )?complete\b')
        notes = self.section('Integration notes')
        for text in ('68535825', 'b949f64e', 'research-index.json', 'test_south_africa_research_s10h.py',
                     'test_south_africa_heads_of_state_c01_09.py', 'test_south_africa_anc_presidents_c01_16.py',
                     'test_south_africa_deputy_presidents_c01_21.py', 'test_campaign_census'):
            self.assertIn(text, notes)
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        for text in ('ready_for_review', 'south-africa-acdp-ff-ifp-leaders-1990-2026-30.md', 'claude/c01-za-30', '68535825',
                     'test_south_africa_party_leaders_c01_30.py', 'Corné Mulder'):
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
