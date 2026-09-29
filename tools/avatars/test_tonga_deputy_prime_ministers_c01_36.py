"""CLAUDE-C01-36: Tonga's 1990-2021 Deputy Prime Ministers keep royal appointment, Cabinet announcement, effective date,
oath, resignation, revocation, death and acting service apart, and state a boundary only where a source states the day."""
import copy
import hashlib
import json
import re
import unittest
from urllib.parse import urlsplit

import campaign_research as research


# Original response identity recorded in each extract: (bytes, sha256) of the body, in packet order.
RESPONSES = {
    'to_pmo_2000_press_releases': (53628, 'd31a98c9903902629e7695c0461b02548e1109ec476286f23ce3392ba69a739a'),
    'to_pmo_news_in_brief_2001': (31877, 'dd163157b535109414c35179576f4118762f507364ddaf2be78061b8b7eb04f4'),
    'to_pmo_20010124_pm_address': (11567, 'd89cb254ab151dfd28d5e2c910870e4a76e72fac151ff137c7878c1ae1ad4f6d'),
    'to_pmo_20010928_tupou_resignation_to': (4898, '958ebb0a55d5827e27e0875ff9ca12807affc420a1405a6c7cb49cbea10aa653'),
    'to_pmo_20020111_acting_dpm_statement': (22721, '7d8c1fb16fcc95b7e27a0176475293d460b7062a3fa6f26892be8c71aba3167f'),
    'to_pmo_20020514_acting_dpm_paunga': (11566, '87e5594913a3d2cc7d7b191a2bfcee15dbf9792ea098bbc2964806ead6a82e65'),
    'to_pmo_20020903_wssd_statement': (25681, 'a10dbe0193e1eb3d871d511071ad205da0535fa683f3cada7b78aca299506bfa'),
    'to_pmo_20040909_confirmation_of_appointments': (19494, 'c8aa0ca0318d16914932eb12920b196316ecef26bed47a08b630b4de5e4e3476'),
    'to_pmo_20050411_pope_homage': (19686, '76df4a308340ca16a21534a8fdd5eb2d1738e787e9c93ef3a889474ff534ab7d'),
    'to_pmo_20060310_ub40_concert': (13311, '7924a5942904e4abce7817bbf1ee16385d20fc39bb6c69486dc6beb2613a6f3d'),
    'to_pmo_20060609_pohiva_statements': (14317, '50a84552b6214d1263489c0456040bc68ca388a6bbb19b795645bf78275c3a7d'),
    'to_pmo_20060809_common_sense': (12953, 'cf7cfbe2157142123ac9ba17c25994c5a30dc4edbdc5cf3e13b0805a70f2d7b8'),
    'to_pmo_20090114_hauofa_funeral': (19408, 'c5174de668f8de3c02b2f5c50db9d477f5cc4e76642739ca9ac1417972ca3ebe'),
    'to_pmo_2009_australia_day_speech': (23691, 'd5f8460a4426c83eaaac0d43a61d31b82577a1a406260c7e6c43a4c429954638'),
    'to_pmo_20100624_throne_reply': (36849, '9b299f69a7c630797c2800e1cde6a74bf8319b435f546de5f30c5356a017d2ff'),
    'to_mic_20101230_lord_tangi': (35793, 'edab13cfcd4bb62e251510f02d33d1feeec594ee0495b5fcdcb8d3a9f731ebfc'),
    'to_mic_20110103_vaipulu_appointed': (31190, '69ef89602fe479027d91ea3e33fbb2ce832b1f3e168bb9fbd20519d99248f9a4'),
    'to_mic_20110104_cabinet_named': (30867, '5217317070781a7dd180176ae38fd72ba9a0d5a6f9265b6a6932147630608289'),
    'to_mic_20120213_foi_speech': (29486, 'ecb526ea466ce71c98525d4e434ccefb77d2f0194459833ba4f1fb01789a37e0'),
    'to_mic_20130424_rti_speech': (44363, 'c7af0f2ac1ccdb5b295cdad7276a668e20763160532a9ffcd49dc94561eac770'),
    'to_mic_20140619_half_mast': (33061, '94f91e69e78011d33d62996fcf71117fd8ddf0feb858486eb40a6f8ca17d5a97'),
    'to_mic_20140909_toloa': (30803, 'b46af3a70632addfe0e2f05af77504ede91fb74299e5ae5d11de3d4b9332a9a3'),
    'to_pmo_20141231_new_cabinet': (47371, '6ff556adae3b8d70096fd5520ae800e80d731d5c7f3d6fa7a1e96f19bc9ceb2f'),
    'to_mic_20150212_sovaleni_profile': (32128, '279eaf8c7339092714b1a58989f8b96ddd6b178a18def8210e4130d098c256f3'),
    'to_mic_20151116_acting_pm_itu': (35533, '354e629b1495f9479b86cbe13ea81221f51734d7f06180fbe7c50450d6df7a66'),
    'to_mic_20160907_renewable': (31933, '71286caa8d86e3fddbb277a32ddbd22e2caeac0fb8d413401fb284fa60375f78'),
    'to_mic_20170404_acting_pm_whales': (36681, '22156a022914a44de0f4ed8f91b8b1fe0116abfb2128c57accbeb2c21a32eaa3'),
    'to_mic_20170831_macbio': (33077, '2d78ccd04685450a79cccc2d6cb0f3fbbab7917fbbb5014ff78d572d62f76d98'),
    'to_pmo_20170906_ministerial_appointments': (41468, '72b063fb52eac4bf6bcb4bbf347b49c22c2ac9f0614342ebab2dd2a4ab26b323'),
    'to_mic_20180122_cabinet_to': (34581, 'f97d7a8f781f3466a5de7243f39d2e0f59e7d368d48da185ef1b6037a5c7c30c'),
    'to_mic_20180615_dpm_gita': (30732, '934102be5923291e184b70b457a1956bed6b923b8446cd7d4bb482cf8990f5a8'),
    'to_la_news_20181108_auditor': (40794, '5ecaa85307a153cb0fe62ce7ad2f2f437fa67fc2a3c580976ba602e84b0dd878'),
    'to_la_term_report_2017_2021': (15526342, '37d85723dd6c1a794c60431135d305dd0f3191d47f022ec2465a108cb3a76f87'),
    'to_pmo_20191010_new_cabinet': (48927, '81d5b44876e6fcce84072d4d67ad1175ae349fa68f0ce7e4c1a694f2aeb71bf7'),
    'to_la_news_20191028_oath': (37376, '19e943fd9abc5f7cb86cfe8cee22cad447ac3e37e6a33dc98aa82ac4cdc9f4a1'),
    'to_pmo_20200416_dpm_clarification': (88720, 'ab551fc715cfb705d662eb9f775fa8b4fc0432210e7b66688a89ea6c6ce59331'),
    'to_la_news_20210901_faotusia': (41553, 'ec4ae5f74b62bd61e98fea8b2dceb7a0a76bbd9793c4c8a6eb039db54d604f5c'),
    'to_la_member_maafu_2021': (40129, 'b9c6399c3df8ed2331740a20d3abc0523e7b1288cbf727fa1478bea1f7ca7136'),
    'to_pmo_20211213_maafu_death': (258072, '3adfa6775d2554271056c0514d421051a03092c38087cbb3c0c0c259ee18978a'),
}
NEW_SOURCES = set(RESPONSES)
ORDER = list(RESPONSES)
# Every source is a raw Internet Archive capture: source id -> capture timestamp.
ARCHIVED = {
    'to_pmo_2000_press_releases': '20011230032256',
    'to_pmo_news_in_brief_2001': '20010708233245',
    'to_pmo_20010124_pm_address': '20010430180727',
    'to_pmo_20010928_tupou_resignation_to': '20011230022245',
    'to_pmo_20020111_acting_dpm_statement': '20020210211730',
    'to_pmo_20020514_acting_dpm_paunga': '20020816151102',
    'to_pmo_20020903_wssd_statement': '20021219084233',
    'to_pmo_20040909_confirmation_of_appointments': '20041105172122',
    'to_pmo_20050411_pope_homage': '20051018153427',
    'to_pmo_20060310_ub40_concert': '20060426083232',
    'to_pmo_20060609_pohiva_statements': '20060615072009',
    'to_pmo_20060809_common_sense': '20060822122944',
    'to_pmo_20090114_hauofa_funeral': '20090122085852',
    'to_pmo_2009_australia_day_speech': '20090326123858',
    'to_pmo_20100624_throne_reply': '20111130060948',
    'to_mic_20101230_lord_tangi': '20140102125353',
    'to_mic_20110103_vaipulu_appointed': '20171024214701',
    'to_mic_20110104_cabinet_named': '20171024212340',
    'to_mic_20120213_foi_speech': '20170906162059',
    'to_mic_20130424_rti_speech': '20171024223318',
    'to_mic_20140619_half_mast': '20200611163419',
    'to_mic_20140909_toloa': '20170127082234',
    'to_pmo_20141231_new_cabinet': '20151002125546',
    'to_mic_20150212_sovaleni_profile': '20171024204047',
    'to_mic_20151116_acting_pm_itu': '20151206105609',
    'to_mic_20160907_renewable': '20160916095045',
    'to_mic_20170404_acting_pm_whales': '20170407191540',
    'to_mic_20170831_macbio': '20170903073650',
    'to_pmo_20170906_ministerial_appointments': '20170910020802',
    'to_mic_20180122_cabinet_to': '20181030055038',
    'to_mic_20180615_dpm_gita': '20181030053741',
    'to_la_news_20181108_auditor': '20190723035015',
    'to_la_term_report_2017_2021': '20211124235017',
    'to_pmo_20191010_new_cabinet': '20191011220754',
    'to_la_news_20191028_oath': '20191122121749',
    'to_pmo_20200416_dpm_clarification': '20230802205155',
    'to_la_news_20210901_faotusia': '20210902180606',
    'to_la_member_maafu_2021': '20210421132335',
    'to_pmo_20211213_maafu_death': '20250415130759',
}
# PDF pages rendered and visually checked for this packet.
PDF_PAGES = {
    'to_la_term_report_2017_2021': [206, 208, 209],
}
# Rows whose source names nobody carry a null holder_name.
NAMELESS = {
    'to_dpm_office_with_health_20060609',
    'to_dpm_sovaleni_appointment_recommended_20141231',
}
# The event kind of every row.
KINDS = {
    'to_dpm_kavaliku_retirement_announced_2000': 'retirement_announced',
    'to_dpm_kavaliku_appointed_1991_retro': 'retrospective_statement',
    'to_dpm_kavaliku_listed_20001106': 'attested_in_office',
    'to_dpm_tupou_appointment_effective_20010124': 'royal_appointment',
    'to_dpm_tupou_resignation_accepted_20010928': 'resignation_accepted',
    'to_dpm_tupou_appointed_jan2001_retro': 'retrospective_statement',
    'to_acting_dpm_edwards_appointed_20010928': 'acting_service',
    'to_acting_dpm_edwards_20020111': 'acting_service',
    'to_dpm_cocker_abroad_20020513': 'attested_in_office',
    'to_acting_dpm_paunga_20020513': 'acting_service',
    'to_dpm_cocker_wssd_statement_20020903': 'attested_in_office',
    'to_dpm_cocker_appointment_confirmed_20040824': 'appointment_confirmed',
    'to_acting_pm_cocker_20050410': 'acting_service',
    'to_dpm_cocker_styled_20060308': 'attested_in_office',
    'to_dpm_office_with_health_20060609': 'attested_in_office',
    'to_dpm_tangi_styled_20060808': 'attested_in_office',
    'to_dpm_tangi_represents_government_20090114': 'attested_in_office',
    'to_acting_pm_tangi_australia_day_2009': 'acting_service',
    'to_dpm_tangi_throne_reply_20100624': 'attested_in_office',
    'to_dpm_tangi_styled_life_peer_20101230': 'attested_in_office',
    'to_dpm_tangi_appointed_may2006_retro': 'retrospective_statement',
    'to_dpm_vaipulu_appointment_reported_20110103': 'appointment_reported',
    'to_dpm_vaipulu_cabinet_commences_20110104': 'cabinet_commencement',
    'to_dpm_vaipulu_styled_20120213': 'attested_in_office',
    'to_acting_pm_vaipulu_20130422': 'acting_service',
    'to_acting_pm_vaipulu_20140618': 'acting_service',
    'to_dpm_vaipulu_styled_20140905': 'attested_in_office',
    'to_acting_pm_vaipulu_20140905': 'acting_service',
    'to_dpm_sovaleni_appointment_recommended_20141231': 'appointment_recommended',
    'to_dpm_sovaleni_cabinet_list_20141231': 'attested_in_office',
    'to_dpm_sovaleni_profile_20150212': 'attested_in_office',
    'to_acting_pm_sovaleni_20151112': 'acting_service',
    'to_dpm_sovaleni_styled_20160904': 'attested_in_office',
    'to_acting_pm_sovaleni_20170404': 'acting_service',
    'to_dpm_sovaleni_styled_20170830': 'attested_in_office',
    'to_dpm_sovaleni_revocation_recommended_20170901': 'revocation_recommended',
    'to_dpm_maafu_appointment_recommended_20170901': 'appointment_recommended',
    'to_dpm_maafu_royal_endorsement_effective_20170901': 'royal_appointment',
    'to_dpm_maafu_endorsement_conveyed_20170905': 'royal_endorsement_conveyed',
    'to_acting_pm_maafu_20170901': 'acting_service',
    'to_dpm_sika_appointment_effective_20180105': 'royal_appointment',
    'to_dpm_sika_styled_20180615': 'attested_in_office',
    'to_dpm_sika_styled_20181108': 'attested_in_office',
    'to_la_report_sika_dpm_jan2018_sep2019': 'retrospective_list',
    'to_la_report_faotusia_dpm_oct2019_2020': 'retrospective_list',
    'to_la_report_maafu_dpm_from_dec2020': 'retrospective_list',
    'to_dpm_faotusia_appointment_effective_20191009': 'royal_appointment',
    'to_dpm_faotusia_oath_news_20191028': 'oath',
    'to_dpm_faotusia_styled_20200416': 'attested_in_office',
    'to_dpm_faotusia_resignation_2020_retro': 'retrospective_statement',
    'to_dpm_maafu_member_page_20210421': 'attested_in_office',
    'to_dpm_maafu_styled_at_death_20211213': 'styled_at_death',
    'to_dpm_maafu_death_20211212': 'death_in_office',
}
# Rows on the Prime Minister's role (acting premierships by a Deputy); every other row is on to_deputy_pm.
PM_ROWS = {
    'to_acting_pm_cocker_20050410': 'to_pm',
    'to_acting_pm_tangi_australia_day_2009': 'to_pm',
    'to_acting_pm_vaipulu_20130422': 'to_pm',
    'to_acting_pm_vaipulu_20140618': 'to_pm',
    'to_acting_pm_vaipulu_20140905': 'to_pm',
    'to_acting_pm_sovaleni_20151112': 'to_pm',
    'to_acting_pm_sovaleni_20170404': 'to_pm',
    'to_acting_pm_maafu_20170901': 'to_pm',
}
# The review observation of every row.
REVIEWS = {
    'to_dpm_kavaliku_retirement_announced_2000': 'TO-DPM-02',
    'to_dpm_kavaliku_appointed_1991_retro': 'TO-DPM-01',
    'to_dpm_kavaliku_listed_20001106': 'TO-DPM-02',
    'to_dpm_tupou_appointment_effective_20010124': 'TO-DPM-03',
    'to_dpm_tupou_resignation_accepted_20010928': 'TO-DPM-03',
    'to_dpm_tupou_appointed_jan2001_retro': 'TO-DPM-03',
    'to_acting_dpm_edwards_appointed_20010928': 'TO-DPM-03',
    'to_acting_dpm_edwards_20020111': 'TO-DPM-04',
    'to_dpm_cocker_abroad_20020513': 'TO-DPM-04',
    'to_acting_dpm_paunga_20020513': 'TO-DPM-04',
    'to_dpm_cocker_wssd_statement_20020903': 'TO-DPM-04',
    'to_dpm_cocker_appointment_confirmed_20040824': 'TO-DPM-04',
    'to_acting_pm_cocker_20050410': 'TO-DPM-04',
    'to_dpm_cocker_styled_20060308': 'TO-DPM-04',
    'to_dpm_office_with_health_20060609': 'TO-DPM-05',
    'to_dpm_tangi_styled_20060808': 'TO-DPM-05',
    'to_dpm_tangi_represents_government_20090114': 'TO-DPM-05',
    'to_acting_pm_tangi_australia_day_2009': 'TO-DPM-05',
    'to_dpm_tangi_throne_reply_20100624': 'TO-DPM-05',
    'to_dpm_tangi_styled_life_peer_20101230': 'TO-DPM-05',
    'to_dpm_tangi_appointed_may2006_retro': 'TO-DPM-05',
    'to_dpm_vaipulu_appointment_reported_20110103': 'TO-DPM-06',
    'to_dpm_vaipulu_cabinet_commences_20110104': 'TO-DPM-06',
    'to_dpm_vaipulu_styled_20120213': 'TO-DPM-06',
    'to_acting_pm_vaipulu_20130422': 'TO-DPM-06',
    'to_acting_pm_vaipulu_20140618': 'TO-DPM-06',
    'to_dpm_vaipulu_styled_20140905': 'TO-DPM-06',
    'to_acting_pm_vaipulu_20140905': 'TO-DPM-06',
    'to_dpm_sovaleni_appointment_recommended_20141231': 'TO-DPM-07',
    'to_dpm_sovaleni_cabinet_list_20141231': 'TO-DPM-07',
    'to_dpm_sovaleni_profile_20150212': 'TO-DPM-07',
    'to_acting_pm_sovaleni_20151112': 'TO-DPM-07',
    'to_dpm_sovaleni_styled_20160904': 'TO-DPM-07',
    'to_acting_pm_sovaleni_20170404': 'TO-DPM-07',
    'to_dpm_sovaleni_styled_20170830': 'TO-DPM-07',
    'to_dpm_sovaleni_revocation_recommended_20170901': 'TO-DPM-07',
    'to_dpm_maafu_appointment_recommended_20170901': 'TO-DPM-08',
    'to_dpm_maafu_royal_endorsement_effective_20170901': 'TO-DPM-08',
    'to_dpm_maafu_endorsement_conveyed_20170905': 'TO-DPM-08',
    'to_acting_pm_maafu_20170901': 'TO-DPM-08',
    'to_dpm_sika_appointment_effective_20180105': 'TO-DPM-08',
    'to_dpm_sika_styled_20180615': 'TO-DPM-08',
    'to_dpm_sika_styled_20181108': 'TO-DPM-08',
    'to_la_report_sika_dpm_jan2018_sep2019': 'TO-DPM-08',
    'to_la_report_faotusia_dpm_oct2019_2020': 'TO-DPM-09',
    'to_la_report_maafu_dpm_from_dec2020': 'TO-DPM-10',
    'to_dpm_faotusia_appointment_effective_20191009': 'TO-DPM-09',
    'to_dpm_faotusia_oath_news_20191028': 'TO-DPM-09',
    'to_dpm_faotusia_styled_20200416': 'TO-DPM-09',
    'to_dpm_faotusia_resignation_2020_retro': 'TO-DPM-09',
    'to_dpm_maafu_member_page_20210421': 'TO-DPM-10',
    'to_dpm_maafu_styled_at_death_20211213': 'TO-DPM-10',
    'to_dpm_maafu_death_20211212': 'TO-DPM-10',
}
STRING_HOLDER = 'to_cabinet_appointment'
# The ten holder observations this packet adds after the string observation, in list order, then the three existing ones.
NAMES = [
    'Langi Kavaliku',
    'Tevita Poasi Tupou',
    'James Cecil Cocker',
    "Viliami Ta'u Tangi",
    'Samiu Kuita Vaipulu',
    'Siaosi Sovaleni',
    "Lord Ma'afu",
    'Semisi Kioa Lafu Sika',
    "Sione Vuna Fa'otusia",
    "Lord Ma'afu",
]
EXISTING = [
    ('Poasi Mataele Tei', None, '2021-12-28', None, ['to_pmo_cabinet_20211229'], ['to_sovaleni_cabinet_effective_20211228']),
    ('Samiu Vaipulu', '2024-12-09', None, None, ['to_assembly_minutes_48_20241209'], ['to_deputy_pm_vaipulu_listed_20241209']),
    ('Taniela Likuohihifo Fusimalohi', None, '2025-01-28', None, ['to_pmo_eke_cabinet_20250128'],
     ['to_eke_cabinet_effective_20250128']),
]
# (attested_on, from, until) of this packet's holders, in list order.
HOLDERS = [
    ('2000-11-06', None, None),
    (None, '2001-01-24', '2001-09-28'),
    ('2002-05-13', None, None),
    ('2006-08-08', None, None),
    (None, '2011-01-04', None),
    ('2014-12-31', None, None),
    (None, '2017-09-01', None),
    (None, '2018-01-05', None),
    (None, '2019-10-09', None),
    ('2021-04-21', None, '2021-12-12'),
]
HOLDER_CLAIMS = [
    ['to_dpm_kavaliku_listed_20001106'],
    ['to_dpm_tupou_appointment_effective_20010124', 'to_dpm_tupou_resignation_accepted_20010928'],
    ['to_dpm_cocker_abroad_20020513', 'to_dpm_cocker_wssd_statement_20020903', 'to_dpm_cocker_appointment_confirmed_20040824', 'to_dpm_cocker_styled_20060308'],
    ['to_dpm_tangi_styled_20060808', 'to_dpm_tangi_represents_government_20090114', 'to_dpm_tangi_throne_reply_20100624', 'to_dpm_tangi_styled_life_peer_20101230'],
    ['to_dpm_vaipulu_cabinet_commences_20110104', 'to_dpm_vaipulu_appointment_reported_20110103', 'to_dpm_vaipulu_styled_20120213', 'to_dpm_vaipulu_styled_20140905'],
    ['to_dpm_sovaleni_cabinet_list_20141231', 'to_dpm_sovaleni_profile_20150212', 'to_dpm_sovaleni_styled_20160904', 'to_dpm_sovaleni_styled_20170830'],
    ['to_dpm_maafu_royal_endorsement_effective_20170901', 'to_dpm_maafu_endorsement_conveyed_20170905'],
    ['to_dpm_sika_appointment_effective_20180105', 'to_dpm_sika_styled_20180615', 'to_dpm_sika_styled_20181108'],
    ['to_dpm_faotusia_appointment_effective_20191009', 'to_dpm_faotusia_styled_20200416'],
    ['to_dpm_maafu_member_page_20210421', 'to_dpm_maafu_styled_at_death_20211213', 'to_dpm_maafu_death_20211212'],
]
HOLDER_SOURCES = [
    ['to_pmo_news_in_brief_2001'],
    ['to_pmo_20010124_pm_address', 'to_pmo_20010928_tupou_resignation_to'],
    ['to_pmo_20020514_acting_dpm_paunga', 'to_pmo_20020903_wssd_statement', 'to_pmo_20040909_confirmation_of_appointments', 'to_pmo_20060310_ub40_concert'],
    ['to_pmo_20060809_common_sense', 'to_pmo_20090114_hauofa_funeral', 'to_pmo_20100624_throne_reply', 'to_mic_20101230_lord_tangi'],
    ['to_mic_20110104_cabinet_named', 'to_mic_20110103_vaipulu_appointed', 'to_mic_20120213_foi_speech', 'to_mic_20140909_toloa'],
    ['to_pmo_20141231_new_cabinet', 'to_mic_20150212_sovaleni_profile', 'to_mic_20160907_renewable', 'to_mic_20170831_macbio'],
    ['to_pmo_20170906_ministerial_appointments'],
    ['to_mic_20180122_cabinet_to', 'to_mic_20180615_dpm_gita', 'to_la_news_20181108_auditor'],
    ['to_pmo_20191010_new_cabinet', 'to_pmo_20200416_dpm_clarification'],
    ['to_la_member_maafu_2021', 'to_pmo_20211213_maafu_death'],
]
# Acting service: a Deputy acting as Prime Minister is a claim on to_pm only; an acting Deputy is a claim on to_deputy_pm only.
ACTING_PM = (
    'to_acting_pm_cocker_20050410',
    'to_acting_pm_tangi_australia_day_2009',
    'to_acting_pm_vaipulu_20130422',
    'to_acting_pm_vaipulu_20140618',
    'to_acting_pm_vaipulu_20140905',
    'to_acting_pm_sovaleni_20151112',
    'to_acting_pm_sovaleni_20170404',
    'to_acting_pm_maafu_20170901',
)
ACTING_DPM = (
    'to_acting_dpm_edwards_appointed_20010928',
    'to_acting_dpm_edwards_20020111',
    'to_acting_dpm_paunga_20020513',
)
# Claims that must never feed a holder: acting service, retirement announcements, recommendations, oaths, retrospective
# statements and lists, the office named without its holder, and the revocation recommendation.
NEVER_HOLDER = ACTING_PM + ACTING_DPM + (
    'to_dpm_kavaliku_retirement_announced_2000',
    'to_dpm_kavaliku_appointed_1991_retro',
    'to_dpm_tupou_appointed_jan2001_retro',
    'to_dpm_office_with_health_20060609',
    'to_dpm_tangi_appointed_may2006_retro',
    'to_dpm_sovaleni_appointment_recommended_20141231',
    'to_dpm_sovaleni_revocation_recommended_20170901',
    'to_dpm_maafu_appointment_recommended_20170901',
    'to_la_report_sika_dpm_jan2018_sep2019',
    'to_la_report_faotusia_dpm_oct2019_2020',
    'to_la_report_maafu_dpm_from_dec2020',
    'to_dpm_faotusia_oath_news_20191028',
    'to_dpm_faotusia_resignation_2020_retro',
)
# Claims of other packets that bear on the office and must stay theirs: never cited by a to_deputy_pm holder.
OTHER_PACKETS = ('to_pohiva_conveys_deputy_pm_removal_20170905', 'to_pohiva_appointment_repeated_20180122',
                 'to_pm_2014_two_nominations_20141229', 'to_pohiva_declared_pm_elect_pmo_20141229',
                 'to_sika_acting_pm_20190914_programme', 'to_sika_acting_pm_20190916_gazette',
                 'to_pmo_vaea_tribute_acting_pm_20090613', 'to_ipu_2014_cabinet_took_office_20150119',
                 'to_la_20190912_pm_prayers_adjourned')
# Dates that must never be a holder date or boundary: the retirement's due day, the office on 1 January 1990, acting-only
# days, month-only retrospective starts and ends read as days, a report date, the Palace letter, the oath, IPU's Cabinet day,
# other packets' days and the next holder's start.
NOT_BOUNDARIES = {'1990-01-01', '1991-01-01', '2000-11-11', '2001-01-01', '2002-01-11', '2006-05-01', '2006-06-09',
                  '2010-12-22', '2011-01-03', '2011-01-14', '2014-12-29', '2015-01-19', '2017-09-05', '2019-09-30',
                  '2019-10-28', '2020-01-01', '2020-12-01', '2020-12-31', '2021-12-28'}
EVENT_KINDS = {'acting_service', 'appointment_confirmed', 'appointment_recommended',
               'appointment_reported', 'attested_in_office', 'cabinet_commencement', 'death_in_office', 'oath',
               'resignation_accepted', 'retirement_announced', 'retrospective_list', 'retrospective_statement',
               'revocation_recommended', 'royal_appointment', 'royal_endorsement_conveyed', 'styled_at_death'}
# Leads named in the report that must never become sources: encyclopaedias, news, stale government rosters, copies of
# imported releases, the Assembly item already owned by CLAUDE-C01-08 and the download-form minutes.
LEAD_URL_MARKERS = ('wikipedia', 'rnz', 'matangi', 'king-appoints-new-pm-and-cabinet-ministers', '538-pohiva-vaipulu',
                    'executive_in_government', 'legislative_assembly.htm', 'gpr19july', 'PR2001', '2004_tlr',
                    'fakanofo-o-e-houeiki-minisita', '6895-ministerial-appointments', '6897-fakanofo',
                    '4051-deputy-prime-minister', 'ongoongo-tukuatu', 'Tongan_Version_confirmation', 'gpr6dec02',
                    'Announcement_of_the', 'article_123', 'cabinet.html', 'prime-minister-announced-new-cabinet',
                    'miniti-fika', '356-a-tribute')
# Names that other packets' tests keep out of every holder except these exact to_deputy_pm observations.
EXEMPT_NAMES = {'Langi Kavaliku': 'Kavaliku', "Viliami Ta'u Tangi": 'Tangi', 'Semisi Kioa Lafu Sika': 'Sika'}
REPORT = research.RESEARCH / 'tonga-deputy-prime-ministers-1990-2026-36.md'
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-36.md'


def dpm_invariants(packet):
    """Packet-level rules this test owns; raises AssertionError or KeyError on any violation."""
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    claim_source = {c['id']: s['id'] for s in packet['sources'] for c in s['claims']}
    entries = {e['id']: e for category in ('organizations', 'institutions') for e in packet[category]}
    roles = {r['id']: r for e in entries.values() for r in e['roles']}
    cabinet = entries['to_cabinet']
    assert [r['id'] for r in cabinet['roles']] == ['to_ministers', 'to_deputy_pm'], 'no new Cabinet role'
    assert [r['id'] for r in entries['to_prime_minister']['roles']] == ['to_pm'], 'no acting Prime Minister role'
    role = roles['to_deputy_pm']
    assert role['holder_claims'][0] == STRING_HOLDER, 'the string observation stays first'
    assert [h for h in role['holder_claims'] if isinstance(h, str)] == [STRING_HOLDER], 'one string observation'
    holders = [h for h in role['holder_claims'] if isinstance(h, dict)]
    assert [h['name'] for h in holders] == NAMES + [e[0] for e in EXISTING], 'exactly the intended holders, in order'
    for holder, expected, cids, sids in zip(holders, HOLDERS, HOLDER_CLAIMS, HOLDER_SOURCES):
        got = (holder['attested_on'], holder['from'], holder['until'])
        assert got == expected, (holder['name'], got)
        assert 'attested_period' not in holder, holder['name']
        assert holder['claim_ids'] == cids and holder['sources'] == sids, holder['name']
        assert not set(holder['claim_ids']) & set(NEVER_HOLDER), holder['name']
        assert not set(holder['claim_ids']) & set(OTHER_PACKETS), holder['name']
        assert not {holder['attested_on'], holder['from'], holder['until']} & NOT_BOUNDARIES, holder['name']
        for cid in holder['claim_ids']:
            assert cid not in PM_ROWS and claim_source[cid] in holder['sources'], (holder['name'], cid)
        assert any(holder['name'] in claims[cid]['text'] for cid in holder['claim_ids']), 'name as printed'
        for word in ('Acting', 'Edwards', 'Paunga', 'Hon.', 'Dr'):
            assert word not in holder['name'], (word, holder['name'])
        # A boundary is always a day one of the cited claims attests.
        for day in (holder['from'], holder['until']):
            assert day is None or day in {claims[cid].get('attested_on') for cid in holder['claim_ids']}, (holder['name'], day)
    for holder, (name, attested, start, end, sids, cids) in zip(holders[len(NAMES):], EXISTING):
        assert (holder['name'], holder['attested_on'], holder['from'], holder['until'], holder['sources'],
                holder['claim_ids']) == (name, attested, start, end, sids, cids), 'existing holders do not change'
    dated = [h['from'] or h['attested_on'] for h in holders]
    assert dated == sorted(dated), 'chronological order'
    # Acting and other roles' claims stay where they belong; no holder anywhere cites acting service of this packet.
    for cid in ACTING_DPM + tuple(c for c in NEVER_HOLDER if c not in ACTING_PM):
        assert cid in role['claim_ids'] and cid in cabinet['claim_ids'], cid
    for cid in ACTING_PM:
        assert cid in roles['to_pm']['claim_ids'] and cid in entries['to_prime_minister']['claim_ids'], cid
        assert cid not in role['claim_ids'] and cid not in cabinet['claim_ids'], cid
    for rid, other in roles.items():
        for entry in other['holder_claims']:
            ids = [entry] if isinstance(entry, str) else entry['claim_ids']
            assert not set(ids) & set(ACTING_PM + ACTING_DPM), rid
            if isinstance(entry, dict) and rid != 'to_deputy_pm':
                assert not set(ids) & set(REVIEWS), rid
    # Names other packets keep out of holders appear only on the exact to_deputy_pm observation.
    for rid, other in roles.items():
        for entry in other['holder_claims']:
            if isinstance(entry, dict):
                for name, word in EXEMPT_NAMES.items():
                    if word in entry['name']:
                        assert (rid, entry['name']) == ('to_deputy_pm', name), (rid, entry['name'])
    # Distinct dated events stay distinct claims.
    assert claims['to_dpm_tupou_appointment_effective_20010124']['attested_on'] == '2001-01-24'
    assert claims['to_dpm_tupou_resignation_accepted_20010928']['attested_on'] == '2001-09-28'
    assert claims['to_acting_dpm_edwards_appointed_20010928']['attested_on'] == '2001-09-28'
    assert claims['to_dpm_maafu_appointment_recommended_20170901']['attested_on'] == '2017-09-01'
    assert claims['to_dpm_maafu_royal_endorsement_effective_20170901']['attested_on'] == '2017-09-01'
    assert claims['to_dpm_maafu_endorsement_conveyed_20170905']['attested_on'] == '2017-09-05'
    assert claims['to_dpm_faotusia_appointment_effective_20191009']['attested_on'] == '2019-10-09'
    assert claims['to_dpm_faotusia_oath_news_20191028']['attested_on'] == '2019-10-28'
    assert claims['to_dpm_maafu_death_20211212']['attested_on'] == '2021-12-12'
    # Undated, year-only and month-only claims carry no structured date.
    for cid in ('to_dpm_kavaliku_retirement_announced_2000', 'to_dpm_kavaliku_appointed_1991_retro',
                'to_dpm_tupou_appointed_jan2001_retro', 'to_dpm_tangi_appointed_may2006_retro',
                'to_acting_pm_tangi_australia_day_2009', 'to_la_report_sika_dpm_jan2018_sep2019',
                'to_la_report_faotusia_dpm_oct2019_2020', 'to_la_report_maafu_dpm_from_dec2020',
                'to_dpm_faotusia_resignation_2020_retro', 'to_dpm_maafu_styled_at_death_20211213'):
        assert 'attested_on' not in claims[cid] and 'period' not in claims[cid], cid


class TongaDeputyPrimeMinisterTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.raw = (research.ROOT / research.RESEARCH / 'tonga.json').read_text(encoding='utf-8')
        cls.packet = json.loads(cls.raw)
        cls.sources = {s['id']: s for s in cls.packet['sources']}
        cls.claims = {c['id']: c for s in cls.packet['sources'] for c in s['claims']}
        cls.claim_source = {c['id']: s['id'] for s in cls.packet['sources'] for c in s['claims']}
        cls.entries = {e['id']: e for category in ('organizations', 'institutions') for e in cls.packet[category]}
        cls.roles = {r['id']: r for e in cls.entries.values() for r in e['roles']}
        cls.cabinet = cls.entries['to_cabinet']
        cls.role = cls.roles['to_deputy_pm']
        cls.holders = [h for h in cls.role['holder_claims'] if isinstance(h, dict)][:len(NAMES)]
        cls.extracts = {
            sid: json.loads((research.ROOT / cls.sources[sid]['snapshot']['path']).read_text(encoding='utf-8'))
            for sid in NEW_SOURCES
        }
        cls.report = (research.ROOT / REPORT).read_text(encoding='utf-8')
        cls.new_claims = [c['id'] for sid in ORDER for c in cls.sources[sid]['claims']]
        cls.dpm_claims = [cid for cid in cls.new_claims if cid not in PM_ROWS]
        cls.pm_claims = [cid for cid in cls.new_claims if cid in PM_ROWS]

    def validate(self, packet=None):
        return research.validate(packet or self.packet, research.ROOT, {'Tonga'}, {'Tonga': set()})

    def section(self, heading):
        parts = self.report.split(f'\n## {heading}', 1)
        self.assertEqual(len(parts), 2, heading)
        return parts[1].split('\n## ', 1)[0]

    def test_new_records_are_bounded_reuse_ids_and_every_claim_is_cited(self):
        ids = self.validate()
        self.assertLessEqual(NEW_SOURCES, set(ids['sources']))
        self.assertEqual((len(NEW_SOURCES), len(self.new_claims), len(set(self.new_claims))), (39, 53, 53))
        self.assertEqual(set(self.new_claims), set(KINDS))
        # The new sources follow every earlier packet's at a fixed position (CLAUDE-C01-24's end at index 168).
        self.assertEqual([s['id'] for s in self.packet['sources'][168:168 + len(ORDER)]], ORDER)
        self.assertEqual([s['id'] for s in self.packet['sources']][167], 'to_la_news_ministers_oath_20260818')
        # Deputy Prime Minister claims sit on to_deputy_pm and to_cabinet after the existing ones; acting premierships sit
        # on to_pm and to_prime_minister; no other entry or role cites a claim of this packet.
        self.assertEqual(self.role['claim_ids'][7:7 + len(self.dpm_claims)], self.dpm_claims)
        self.assertEqual(self.cabinet['claim_ids'][17:17 + len(self.dpm_claims)], self.dpm_claims)
        self.assertEqual(self.roles['to_pm']['claim_ids'][83:83 + len(self.pm_claims)], self.pm_claims)
        self.assertEqual(self.entries['to_prime_minister']['claim_ids'][88:88 + len(self.pm_claims)], self.pm_claims)
        dpm_sources = [s for s in ORDER if any(c['id'] not in PM_ROWS for c in self.sources[s]['claims'])]
        pm_sources = [s for s in ORDER if any(c['id'] in PM_ROWS for c in self.sources[s]['claims'])]
        self.assertEqual(self.role['sources'][5:5 + len(dpm_sources)], dpm_sources)
        self.assertEqual(self.cabinet['sources'][14:14 + len(dpm_sources)], dpm_sources)
        self.assertEqual(self.roles['to_pm']['sources'][64:64 + len(pm_sources)], pm_sources)
        self.assertEqual(self.entries['to_prime_minister']['sources'][69:69 + len(pm_sources)], pm_sources)
        for eid, entry in self.entries.items():
            if eid not in ('to_cabinet', 'to_prime_minister'):
                self.assertFalse(set(self.new_claims) & set(entry['claim_ids']), eid)
        for rid, role in self.roles.items():
            if rid not in ('to_deputy_pm', 'to_pm'):
                self.assertFalse(set(self.new_claims) & set(role['claim_ids']), rid)
        # No new organization, institution or role; existing identities are reused.
        self.assertEqual(len(ids['entries']), 9)
        self.assertEqual({e['id'] for e in self.packet['institutions']},
                         {'to_crown', 'to_prime_minister', 'to_cabinet', 'to_privy_council', 'to_legislative_assembly'})
        observations = re.findall(r'^### (TO-DPM-\d\d)\b', self.report, re.M)
        self.assertEqual(observations, [f'TO-DPM-{n:02d}' for n in range(1, 11)])
        self.assertEqual(sorted(set(REVIEWS.values())), [f'TO-DPM-{n:02d}' for n in range(1, 11)])
        # Every structured date lies inside the period and before the cutoff; no claim carries a period.
        for cid in self.new_claims:
            value = self.claims[cid].get('attested_on')
            if value:
                self.assertTrue('1990-01-01' <= value <= research.CUTOFF, cid)
            self.assertNotIn('period', self.claims[cid])
        for sid in ORDER:
            self.assertLessEqual(self.sources[sid].get('published_date', ''), research.CUTOFF)

    def test_holders_are_exactly_as_intended(self):
        dpm_invariants(self.packet)
        self.assertEqual(len(self.role['holder_claims']), 1 + len(NAMES) + len(EXISTING))
        for holder in self.holders:
            self.assertTrue(holder['note'] and holder['uncertainty'], holder['name'])
        self.assertIn('never holders or boundaries', self.role['scope_note'])
        self.assertIn(STRING_HOLDER, self.role['scope_note'])
        # People: ten observations of nine people; Lord Ma'afu's two deputy premierships stay separate observations.
        self.assertEqual(len(set(NAMES)), 9)
        self.assertEqual([i for i, n in enumerate(NAMES) if n == "Lord Ma'afu"], [6, 9])
        # The 2011 and 2024 Vaipulu observations stay separate; the 2014-2017 Sovaleni observation is not the to_pm holder.
        self.assertEqual(sum('Vaipulu' in h['name'] for h in self.role['holder_claims'] if isinstance(h, dict)), 2)
        self.assertNotIn("Siaosi 'Ofakivahafolau Sovaleni", NAMES)
        self.assertEqual(sum('are claims on to_deputy_pm only' in u for u in self.cabinet['coverage']['unresolved']), 1)
        self.assertEqual(sum(u.startswith('CLAUDE-C01-36 adds acting Prime Minister claims')
                             for u in self.entries['to_prime_minister']['coverage']['unresolved']), 1)

    def test_appointment_announcement_effect_oath_resignation_and_death_never_collapse(self):
        dpm_invariants(self.packet)
        tupou = [self.claims[c]['attested_on'] for c in ('to_dpm_tupou_appointment_effective_20010124',
                                                         'to_dpm_tupou_resignation_accepted_20010928')]
        self.assertEqual(tupou, ['2001-01-24', '2001-09-28'])
        self.assertIn("with effect from 24th January, 2001'", self.claims['to_dpm_tupou_appointment_effective_20010124']['text'])
        self.assertIn("('Na'e tali 'a e poaki ni')", self.claims['to_dpm_tupou_resignation_accepted_20010928']['text'])
        # Announcement and commencement; recommendation, endorsement and the Palace letter; appointment and oath.
        self.assertLess(self.claims['to_dpm_vaipulu_appointment_reported_20110103']['attested_on'],
                        self.claims['to_dpm_vaipulu_cabinet_commences_20110104']['attested_on'])
        self.assertIn("The Cabinet Ministers commence today, Tuesday 4th. January, 2011",
                      self.claims['to_dpm_vaipulu_cabinet_commences_20110104']['text'])
        september = [self.claims[c]['attested_on'] for c in ('to_dpm_maafu_appointment_recommended_20170901',
                                                             'to_dpm_maafu_royal_endorsement_effective_20170901',
                                                             'to_dpm_maafu_endorsement_conveyed_20170905')]
        self.assertEqual(september, ['2017-09-01', '2017-09-01', '2017-09-05'])
        self.assertIn('Royal Sign Manual', self.claims['to_dpm_maafu_royal_endorsement_effective_20170901']['text'])
        self.assertIn('not a start', self.claims['to_dpm_faotusia_oath_news_20191028']['uncertainty'])
        self.assertIn("with effect from 9th October, 2019'",
                      self.claims['to_dpm_faotusia_appointment_effective_20191009']['text'])
        # Acting claims say what they are.
        for cid in ACTING_PM:
            self.assertIn('a claim on to_pm only, never a holder', self.claims[cid]['uncertainty'])
        for cid in ACTING_DPM:
            self.assertIn('a claim on to_deputy_pm only, never a holder', self.claims[cid]['uncertainty'])
        # Tongan wording is quoted as printed with the reading given separately.
        sika = self.claims['to_dpm_sika_appointment_effective_20180105']['text']
        self.assertIn("''Eiki Tokoni Palēmia' (Deputy Prime Minister)", sika)
        self.assertIn("''o kamata lau mei he 'aho 5'o Sanuali, 2018'", sika)
        self.assertIn("'Tokoni Palemia (mei Tisema 2020)'", self.claims['to_la_report_maafu_dpm_from_dec2020']['text'])
        # The office named without its holder feeds nobody.
        self.assertIsNone(next(r for r in self.extracts['to_pmo_20060609_pohiva_statements']['rows'])['holder_name'])

    def test_boundaries_only_where_a_source_states_the_day(self):
        starts = [(h['name'], h['from']) for h in self.holders if h['from']]
        self.assertEqual(starts, [('Tevita Poasi Tupou', '2001-01-24'), ('Samiu Kuita Vaipulu', '2011-01-04'),
                                  ("Lord Ma'afu", '2017-09-01'),
                                  ('Semisi Kioa Lafu Sika', '2018-01-05'), ("Sione Vuna Fa'otusia", '2019-10-09')])
        ends = [(h['name'], h['until']) for h in self.holders if h['until']]
        self.assertEqual(ends, [('Tevita Poasi Tupou', '2001-09-28'), ("Lord Ma'afu", '2021-12-12')])
        # A retirement announcement, a revocation recommendation and a year-only resignation give no end.
        kavaliku, sovaleni, faotusia = self.holders[0], self.holders[5], self.holders[8]
        self.assertIsNone(kavaliku['until'])
        self.assertIn('not a stated end', self.claims['to_dpm_kavaliku_retirement_announced_2000']['uncertainty'])
        self.assertIsNone(sovaleni['until'])
        # The 2014 release states a recommendation and a forecast effective day, not a royal act: no start either.
        self.assertEqual((sovaleni['attested_on'], sovaleni['from']), ('2014-12-31', None))
        self.assertIn('not used as a start', self.claims['to_dpm_sovaleni_appointment_recommended_20141231']['uncertainty'])
        self.assertIn('not used as Sovaleni', self.claims['to_dpm_sovaleni_revocation_recommended_20170901']['uncertainty'])
        self.assertIsNone(faotusia['until'])
        self.assertIn('year only', self.claims['to_dpm_faotusia_resignation_2020_retro']['uncertainty'])
        # No end from a successor's start, a later roster or a change of government.
        self.assertIn('none is inferred', self.holders[2]['uncertainty'])
        self.assertIn('supply none', self.holders[3]['uncertainty'])
        self.assertIn('supplies none', self.holders[6]['uncertainty'])
        self.assertIn('death in office', self.claims['to_dpm_maafu_death_20211212']['uncertainty'])
        unresolved = self.cabinet['coverage']['unresolved']
        rows = [u for u in unresolved if u.startswith('TO-DPM-')]
        self.assertEqual([u.split(' ', 1)[0].rstrip(':') for u in rows],
                         ['TO-DPM-01', 'TO-DPM-02/03/04', 'TO-DPM-05/06', 'TO-DPM-07/08', 'TO-DPM-09/10'])
        self.assertTrue(unresolved[0].startswith('Build dated Cabinet and acting-minister membership'))
        self.assertEqual(sum('Deputy Prime Minister packet 36' in u for u in self.packet['coverage']['unresolved']), 1)

    def test_extracts_match_packet_claims_and_record_original_responses(self):
        for sid in NEW_SOURCES:
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual(extract['format'], 'spheres-c01-derived-factual-table/v1')
            self.assertEqual((extract['source_id'], extract['source_url']), (sid, source['url']))
            self.assertEqual(extract['scope_note'], source['scope_note'])
            self.assertEqual(extract['access_method'], source['access_method'])
            self.assertEqual(extract['published_date'], source.get('published_date'))
            self.assertEqual((extract['accessed_date'], source['accessed_date']), ('2026-09-28', '2026-09-28'))
            self.assertFalse(extract['source_response_checked_in'])
            self.assertEqual((extract['source_response_bytes'], extract['source_response_sha256']), RESPONSES[sid])
            self.assertIn('not checked into this repository', extract['provenance_note'])
            self.assertIn('No open license, portrait permission or likeness approval', extract['rights_note'])
            self.assertEqual(extract['visual_review']['pdf_pages_one_based'], PDF_PAGES.get(sid, []))
            self.assertRegex(extract['stability_check'], r'(\d+) minutes later')
            self.assertGreaterEqual(int(re.search(r'(\d+) minutes later', extract['stability_check']).group(1)), 30)
            # Rows repeat the packet claims exactly, keyed by claim_id, with a holder_name and never a name key.
            self.assertEqual([r['claim_id'] for r in extract['rows']], [c['id'] for c in source['claims']])
            for row, claim in zip(extract['rows'], source['claims']):
                self.assertNotIn('name', row)
                expected = (('to_prime_minister', 'to_pm', 'Prime Minister') if claim['id'] in PM_ROWS
                            else ('to_cabinet', 'to_deputy_pm', 'Deputy Prime Minister'))
                self.assertEqual((row['observation_id'], row['role_id'], row['role_title']), expected)
                self.assertEqual((row['event_kind'], row['review_observation']), (KINDS[claim['id']], REVIEWS[claim['id']]))
                self.assertIn(row['event_kind'], EVENT_KINDS)
                self.assertEqual((row['text'], row['locator'], row['uncertainty'], row['attested_on']),
                                 (claim['text'], claim['locator'], claim['uncertainty'], claim.get('attested_on')))
                self.assertIn('holder_name', row)
                if row['holder_name'] is not None:
                    self.assertIn(row['holder_name'], claim['text'])
            snapshot = source['snapshot']
            self.assertEqual(snapshot['kind'], 'derived_factual_extract')
            self.assertTrue(snapshot['path'].startswith('docs/campaign-certification/C01/research/sources/tonga-'))
            data = (research.ROOT / snapshot['path']).read_bytes()
            self.assertEqual((len(data), hashlib.sha256(data).hexdigest()), (snapshot['bytes'], snapshot['sha256']))
            self.assertNotEqual(snapshot['sha256'], extract['source_response_sha256'])
            self.assertTrue(data.endswith(b'}\n') and b'\r' not in data)
            self.assertEqual(data.decode('utf-8'), json.dumps(extract, indent=2, ensure_ascii=False) + '\n')
            self.assertTrue(source['source_type'] and source['scope_note'])
        nameless = {r['claim_id'] for e in self.extracts.values() for r in e['rows'] if r['holder_name'] is None}
        self.assertEqual(nameless, NAMELESS)
        # Raw Internet Archive captures made before the cutoff, recorded as received (no gzip transfer).
        self.assertEqual(set(ARCHIVED), NEW_SOURCES)
        for sid, stamp in ARCHIVED.items():
            source, extract = self.sources[sid], self.extracts[sid]
            url = urlsplit(source['url'])
            self.assertEqual(url.hostname, 'web.archive.org')
            self.assertTrue(url.path.startswith(f'/web/{stamp}id_/http'), sid)
            self.assertLess(stamp, '20260907')
            self.assertEqual(extract['original_url'], source['original_url'])
            self.assertNotIn(':80', source['original_url'])
            self.assertEqual(extract['archive_capture_utc'].replace('-', '').replace(':', '').replace('T', '').rstrip('Z'), stamp)
            self.assertIn('Raw Internet Archive capture', extract['provenance_note'])
            self.assertIn("'Accept-Encoding: identity'", extract['fetch_recipe'])
            self.assertEqual(extract['source_response_content_encoding'], 'identity')
            self.assertNotIn('source_response_transfer_bytes', extract)
            self.assertIn(urlsplit(source['original_url']).hostname,
                          {'pmo.gov.to', 'www.pmo.gov.to', 'mic.gov.to', 'www.mic.gov.to', 'parliament.gov.to'})

    def test_no_per_request_or_growing_source_urls(self):
        for sid in NEW_SOURCES:
            url = self.sources[sid]['url']
            for marker in ('?start=', 'search', 'finder', 'cb=', 'nocache', 'hansards-debates', 'tmpl=component', 'print=1',
                           'format=', 'task=view', 'data.ipu.org', '/category/', '/tag/', '/page/'):
                self.assertNotIn(marker, url, sid)
        # No source is reused from another packet: every original address is new to the packet.
        originals = [s.get('original_url') for s in self.packet['sources'] if s['id'] not in NEW_SOURCES]
        for sid in NEW_SOURCES:
            self.assertNotIn(self.sources[sid]['original_url'], originals, sid)

    def test_secondary_and_unimported_leads_stay_out_of_the_packet(self):
        for source in self.packet['sources']:
            if source['id'] in NEW_SOURCES:
                self.assertFalse(any(marker in source['url'] for marker in LEAD_URL_MARKERS), source['id'])
        lowered = self.raw.lower()
        for marker in ('wikipedia', 'matangi', 'rnz', 'four years', 'kaniva', 'talanoa', '12 september 2019'):
            self.assertNotIn(marker, lowered, marker)
        leads, added = self.section('Leads not imported'), self.section('Sources added')
        for marker in ('wikipedia.org', '538-pohiva-vaipulu', 'executive_in_government.htm', 'legislative_assembly.htm',
                       'gpr19july', '2004_tlr', '6895-ministerial-appointments', '4051-deputy-prime-minister'):
            self.assertIn(marker, leads, marker)
            self.assertNotIn(marker, added, marker)
        attempted = self.section('Sources attempted')
        for marker in ('gazettes-by-year', 'license_agree', 'paclii', '2020/12'):
            self.assertIn(marker, attempted, marker)

    def test_packet_formatting_is_preserved(self):
        data = (research.ROOT / research.RESEARCH / 'tonga.json').read_bytes()
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

        def role(packet, rid='to_deputy_pm'):
            return next(r for e in packet['institutions'] for r in e['roles'] if r['id'] == rid)

        def holder(packet, index):
            return [h for h in role(packet)['holder_claims'] if isinstance(h, dict)][index]

        validator_cases = [
            (lambda p: source(p, 'to_pmo_20010928_tupou_resignation_to')['snapshot'].update(sha256='0' * 64),
             'checksum mismatch'),
            (lambda p: source(p, 'to_la_term_report_2017_2021')['snapshot'].update(bytes=1), 'checksum mismatch'),
            (lambda p: source(p, 'to_mic_20110104_cabinet_named')['snapshot'].update(path=REPORT.as_posix()), 'escapes'),
            (lambda p: holder(p, 9).update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: holder(p, 1).update(until='2026-09-30'), 'exceeds cutoff'),
            (lambda p: claim(p, 'to_dpm_maafu_death_20211212').update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: holder(p, 1).update({'from': '2001-10-01'}), 'Reversed historical interval'),
            (lambda p: holder(p, 4).update(claim_ids=['to_dpm_sika_styled_20180615']), 'cited source'),
            (lambda p: role(p)['claim_ids'].append('to_dpm_does_not_exist'), 'Unknown'),
            (lambda p: claim(p, 'to_dpm_tangi_appointed_may2006_retro').update(period={'from': '2006-05', 'through': '2006-05'}),
             '(?i)invalid'),
        ]
        for change, message in validator_cases:
            with self.subTest(message=message), self.assertRaisesRegex(ValueError, message):
                self.validate(mutated(change))
        invariant_cases = [
            ('successor start as end', lambda p: holder(p, 0).update(until='2001-01-24')),
            ('successor start as end (Sika)', lambda p: holder(p, 7).update(until='2019-10-09')),
            ('successor start as end (Cocker)', lambda p: holder(p, 2).update(until='2006-08-09')),
            ('retirement announcement as end', lambda p: holder(p, 0).update(until='2000-11-11')),
            ('revocation recommendation as end', lambda p: holder(p, 5).update(until='2017-09-01')),
            ('report date as start', lambda p: holder(p, 4).update({'from': '2011-01-03'})),
            ('Palace letter as start', lambda p: holder(p, 6).update({'from': '2017-09-05'})),
            ('oath as start', lambda p: holder(p, 8).update({'from': '2019-10-28'})),
            ('retrospective month as start', lambda p: holder(p, 9).update({'from': '2020-12-01'})),
            ('retrospective month as end', lambda p: holder(p, 7).update(until='2019-09-30')),
            ('IPU Cabinet day as start', lambda p: holder(p, 5).update({'from': '2015-01-19'})),
            ('acting Deputy as holder', lambda p: role(p)['holder_claims'].insert(2, {
                'name': 'Hon. Clive Edwards', 'attested_on': '2001-09-28', 'from': None, 'until': None,
                'sources': ['to_pmo_20010928_tupou_resignation_to'], 'claim_ids': ['to_acting_dpm_edwards_appointed_20010928']})),
            ('acting Prime Minister as a to_pm holder', lambda p: role(p, 'to_pm')['holder_claims'].append({
                'name': "Lord Ma'afu", 'attested_on': '2017-09-01', 'from': None, 'until': None,
                'sources': ['to_pmo_20170906_ministerial_appointments'], 'claim_ids': ['to_acting_pm_maafu_20170901']})),
            ('acting premiership cited by a Deputy holder', lambda p: holder(p, 6)['claim_ids'].append('to_acting_pm_maafu_20170901')),
            ('recommendation cited by a holder', lambda p: holder(p, 6)['claim_ids'].append(
                'to_dpm_maafu_appointment_recommended_20170901')),
            ('retrospective list cited by a holder', lambda p: holder(p, 7)['claim_ids'].append(
                'to_la_report_sika_dpm_jan2018_sep2019')),
            ('another packet\'s claim cited by a holder', lambda p: holder(p, 5)['claim_ids'].append(
                'to_pohiva_conveys_deputy_pm_removal_20170905')),
            ('Deputy claim cited by the Prime Minister\'s holder', lambda p: next(
                h for h in role(p, 'to_pm')['holder_claims'] if isinstance(h, dict))['claim_ids'].append(
                'to_dpm_tupou_appointment_effective_20010124')),
            ('acting premiership moved onto the Deputy role', lambda p: role(p)['claim_ids'].append('to_acting_pm_cocker_20050410')),
            ('existing holder changed', lambda p: holder(p, 10).update({'from': '2021-12-29'})),
            ('string observation moved', lambda p: role(p)['holder_claims'].append(role(p)['holder_claims'].pop(0))),
            ('holders out of order', lambda p: role(p)['holder_claims'].insert(1, role(p)['holder_claims'].pop(4))),
            ('year-only claim given a date', lambda p: claim(p, 'to_dpm_faotusia_resignation_2020_retro').update(
                attested_on='2020-01-01')),
            ('second Deputy role', lambda p: next(e for e in p['institutions'] if e['id'] == 'to_cabinet')['roles'].append(
                {'id': 'to_acting_deputy_pm', 'title': 'Acting Deputy Prime Minister', 'kind': 'institutional_office',
                 'sources': ['to_pmo_20020514_acting_dpm_paunga'], 'claim_ids': ['to_acting_dpm_paunga_20020513'],
                 'holder_claims': []})),
            ('exempt name on another role', lambda p: role(p, 'to_ministers')['holder_claims'].append({
                'name': 'Semisi Kioa Lafu Sika', 'attested_on': '2018-01-05', 'from': None, 'until': None,
                'sources': ['to_mic_20180122_cabinet_to'], 'claim_ids': ['to_dpm_sika_appointment_effective_20180105']})),
        ]
        dpm_invariants(self.packet)
        for label, change in invariant_cases:
            with self.subTest(label=label), self.assertRaises((AssertionError, KeyError, StopIteration)):
                dpm_invariants(mutated(change))

    def test_submitted_report_and_handoff_close_no_parent_gate(self):
        self.assertIn('ready_for_review', self.report)
        table = self.section('Outcome')
        decisions = {'01': 'Unresolved', '02': 'Accepted in part', '03': 'Accepted', '04': 'Accepted in part',
                     '05': 'Accepted in part', '06': 'Accepted in part', '07': 'Accepted in part', '08': 'Accepted in part',
                     '09': 'Accepted in part', '10': 'Accepted in part'}
        for number, decision in decisions.items():
            row, = [line for line in table.splitlines() if line.startswith(f'| TO-DPM-{number} ')]
            self.assertIn(f'**{decision}', row)
        for marker in ('C01', 'C06', 'S23', 'WC1', 'CP1'):
            self.assertNotRegex(self.report, rf'\b{marker}\b[^.\n]*\bis (now )?complete\b')
        notes = self.section('Integration notes')
        for text in ('e8821f40', '44098c5a', 'research-index.json', 'test_tonga_research_s10g.py', 'test_tonga_speakers_c01_24.py',
                     'test_tonga_transition_c01_02.py', 'test_tonga_transition_c01_03.py', 'test_tonga_pm_1990_2019_c01_08.py',
                     'test_tonga_dpfi_c01_04.py', 'census.json', 'government.rs'):
            self.assertIn(text, notes)
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        for text in ('ready_for_review', 'tonga-deputy-prime-ministers-1990-2026-36.md', 'claude/c01-to-36', 'e8821f40',
                     '44098c5a', 'test_tonga_deputy_prime_ministers_c01_36.py'):
            self.assertIn(text, handoff)
        self.assertNotIn('State: **claimed**', handoff)
        index = research.build()
        country = next(p for p in index['countries'] if p['nation'] == 'Tonga')
        self.assertFalse(country['country_census_complete'])
        self.assertIsNone(country['unrepresented_organization_count'])
        self.assertEqual(country['mapping_pending'], 9)
        batch, = [row for row in index['work_orders'] if row['nation'] == 'Tonga']
        self.assertEqual(batch['status'], 'open')
        self.assertFalse(index['c01_complete'])


if __name__ == '__main__':
    unittest.main()
