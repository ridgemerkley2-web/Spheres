"""CLAUDE-C01-42: the 日本共産党 Executive Committee chair (幹部会委員長) and Central Committee chair (中央委員会議長) and the
国民民主党 representative (代表), 1990-2026, extended on the existing roles and kept apart from state office and from each
other. Each plenum or convention election, 'new chair' styling, withdrawal, honorary office, rule change, suspension, acting
service and organization event stays a separate dated claim; a holder is dated by a same-day party record or the party organ
naming the person in the office, and no holder in this packet has a stated start or end. The 国民民主党 of May 2018 to
September 2020 is an earlier organization with the same name: its offices are claims only, never this role's holders."""
import copy
import hashlib
import json
import re
import unittest
from urllib.parse import urlsplit

import campaign_research as research

JCP_ORG = 'jp_sangiin_pr_2025_01'
DPFP_ORG = 'jp_sangiin_pr_2025_07'
EXEC = 'jp_jcp_executive_committee_chair'
CENTRAL = 'jp_jcp_central_committee_chair'
DPFP = 'jp_dpfp_representative'

T_EXEC = '幹部会委員長 — Executive Committee chair'
T_CENTRAL = '中央委員会議長 — Central Committee chair'
T_DPFP = '代表 — party representative'

# The packet's sources before this packet (CLAUDE-C01-31, accepted into integration 13367c99, ends at 503).
EARLIER_SOURCE_COUNT = 503

GROUPS = ['jp_shugiin_group_20260218_011',
 'jp_shugiin_group_20260218_020',
 'jp_shugiin_group_20260218_030',
 'jp_shugiin_group_20260218_040',
 'jp_shugiin_group_20260218_050',
 'jp_shugiin_group_20260218_060',
 'jp_shugiin_group_20260218_070']
ALL_ROLES = ['jp_jcp_executive_committee_chair',
 'jp_jcp_central_committee_chair',
 'jp_dpfp_representative',
 'jp_sdp_chair',
 'jp_ldp_party_president',
 'jp_komeito_representative',
 'jp_pm']

PM_HOLDER_COUNT = 30

ACCESSED = '2026-10-01'

PREFIX = {'jp_jcp_executive_committee_chair': 'jp_jcp_',
 'jp_jcp_central_committee_chair': 'jp_jcp_',
 'jp_dpfp_representative': 'jp_dpfp_'}

# Each organization's sources, claims and unresolved-note count before this packet.
ORG_EARLIER = {'jp_sangiin_pr_2025_01': (['jp_tokyo_pr_2025', 'jp_jcp_chairs_2024'],
                           ['jp_pr2025_submission_01',
                            'jp_tamura_executive_chair_20240118',
                            'jp_shii_central_chair_20240118'],
                           3),
 'jp_sangiin_pr_2025_07': (['jp_tokyo_pr_2025', 'jp_dpfp_tamaki_elected_2026'],
                           ['jp_pr2025_submission_07', 'jp_tamaki_representative_20260906'],
                           3)}

# Each role's sources and claims before this packet.
ROLE_EARLIER = {'jp_jcp_executive_committee_chair': (['jp_jcp_chairs_2024'], ['jp_tamura_executive_chair_20240118']),
 'jp_jcp_central_committee_chair': (['jp_jcp_chairs_2024'], ['jp_shii_central_chair_20240118']),
 'jp_dpfp_representative': (['jp_dpfp_tamaki_elected_2026'], ['jp_tamaki_representative_20260906'])}

# The original intake holder observations, which stay exactly as they were.
EARLIER_HOLDERS = {'jp_jcp_executive_committee_chair': [{'name': '田村智子',
                                       'attested_on': '2024-01-18',
                                       'from': None,
                                       'until': None,
                                       'sources': ['jp_jcp_chairs_2024'],
                                       'claim_ids': ['jp_tamura_executive_chair_20240118'],
                                       'note': 'Observation of the stated party office on this date only; not a '
                                               'continuous term, biography or likeness authorization.'}],
 'jp_jcp_central_committee_chair': [{'name': '志位和夫',
                                     'attested_on': '2024-01-18',
                                     'from': None,
                                     'until': None,
                                     'sources': ['jp_jcp_chairs_2024'],
                                     'claim_ids': ['jp_shii_central_chair_20240118'],
                                     'note': 'Observation of the stated party office on this date only; not a '
                                             'continuous term, biography or likeness authorization.'}],
 'jp_dpfp_representative': [{'name': '玉木雄一郎',
                             'attested_on': '2026-09-06',
                             'from': None,
                             'until': None,
                             'sources': ['jp_dpfp_tamaki_elected_2026'],
                             'claim_ids': ['jp_tamaki_representative_20260906'],
                             'note': 'Observation of the stated party office on this date only; not a continuous '
                                     'term, biography or likeness authorization.'}]}

# Each role's scope note before this packet; the packet only appends a sentence.
EARLIER_SCOPE = {'jp_jcp_executive_committee_chair': 'Distinct from 中央委員会議長. Succession rules and previous/subsequent holders remain '
                                     'unresolved.',
 'jp_jcp_central_committee_chair': 'Distinct from 幹部会委員長. Do not collapse the two chair titles into one leader slot.',
 'jp_dpfp_representative': 'Party office only. No mapping to earlier organizations with the same name or to a House '
                           'group is asserted.'}

# Original response identity recorded in each extract: (bytes, sha256) of the identity-encoded body, fetched as the
# extract's fetch_recipe says. Every source is a raw Internet Archive capture made before the cutoff.
RESPONSES = {'jp_jcp_programme_report_20th_congress_19940719': (123155,
                                                    'fdf64ab3f5737f6a6e5cd5fc7e39ab32a5ec1e7269fcf569a0c136348f4cd9f4'),
 'jp_jcp_21st_congress_diary_19970926': (6152, 'faae9295cb3e43b31c80ba87adef9629b684109084fca8026c0f5c663e242806'),
 'jp_jcp_21st_congress_honorary_officers_19970926': (5843,
                                                     'ea7808aa4bba9d8dfe5d4dcdae31ff47f2509644d77b502a9c82b8d31118086d'),
 'jp_jcp_21st_first_plenum_19970926': (2564, 'd61a5690250c7d94cae2f214f4bcb6731568f77174c8c83681ea0c5eeb836395'),
 'jp_jcp_21st_rules_amendment_proposal_19970925': (2643,
                                                   '13875b2f48382b37f88fde6b8f74819644f36e89495b17621a0127febf4697d6'),
 'jp_jcp_22nd_congress_opening_address_20001120': (7763,
                                                   '033be9512175dd76e7d2196455c44a00e46ae6b45a06e14182915296f57330ec'),
 'jp_jcp_22nd_officers_20001124': (4098, '0072351e1a23a3582f94248ad1ae7eebd032cdcf2a88f0acb1576edef2b03d40'),
 'jp_jcp_22nd_congress_closing_address_20001124': (13305,
                                                   'cd1f59eab88f28f8dfca3e84a6237f10d931449067254632e21a0a9c8b23ae02'),
 'jp_jcp_second_plenum_presidium_report_20010529': (27352,
                                                    '4a5bd561d273b453359e603a603ca094e4722a4f65584c87e012e807cf3cf3ab'),
 'jp_jcp_23rd_first_plenum_20040117': (2147, 'ceb40559b7a76a7da74ffcc3acab43b9c39dff972e4cd1c886eba48737710ba4'),
 'jp_jcp_23rd_congress_summation_20040117': (22278,
                                             'cf19238fde631a447ff446d71096bf4d15d839dc5a2e6c8c508667f73675c0cc'),
 'jp_jcp_24th_congress_opening_address_20060112': (15238,
                                                   '5b9c2fee32c21ab9df170f881a7ea7040e48050d215f859de5beba1b41509169'),
 'jp_jcp_24th_personnel_report_20060115': (14225, 'f362e25e69d4f70634d8b0376f19e51c57ab8cf33943a0c42c6d7bd5c10ba070'),
 'jp_jcp_24th_congress_closing_address_20060114': (14258,
                                                   'cc19f705bfb6d2f5f98587a31d6a5ea719c655a07ffaba1db1530289a19a8b36'),
 'jp_jcp_25th_congress_closing_address_20100116': (19260,
                                                   '86f31e66b4ee20f88bb057d8e658484bc48282778238e82df253c1cadae5562c'),
 'jp_jcp_26th_congress_summation_20140118': (54492,
                                             '5f9df74ddade365dd82bdd580401431d894b1201e107bc5257cf8a3f344b64c8'),
 'jp_jcp_27th_officers_20170118': (25279, '750ec4b54e4ca94e01a5621d0dd05ff68dbff1ca4c781c56bedeb6a36de3f4aa'),
 'jp_jcp_27th_congress_closing_address_20170118': (28252,
                                                   'cac24a428338c8d31721645e84430b23b0c33152cdd0a3985faabd07ffcab9a3'),
 'jp_jcp_28th_officers_20200118': (53694, 'b3d2ea002ff57940c604a52b2bd9af3d2b3ebddcf6522b3847dc1ae6e7858f6c'),
 'jp_jcp_28th_congress_closing_address_20200118': (26965,
                                                   '94d7f46ebd27e45fef0526d98e45ae9ceaa529a087e4df282a5b65f06ff8b470'),
 'jp_dpfp_2018_founding_convention_20180507': (37294,
                                               '9c118b8d620a4a0e86e56a54bfe499a5c177542fc59d10a5ffd8cdc018c0c760'),
 'jp_dpfp_2018_representative_elected_20180904': (28188,
                                                  '1936c0b1a645f274419c80a175dcf11199e5faafe627842d7d1b2054920389c4'),
 'jp_dpfp_2018_representative_press_conference_20180904': (76494,
                                                           '48538545b2ba063f9970af947c0f90ab41aae01fd74972309941d12c4c9ed7d7'),
 'jp_dpfp_2018_party_dissolved_20200911': (49403, '75ed156389d1155a2612ef311e45359be2f1e992f43b525afa05346e21a57e41'),
 'jp_dpfp_founding_convention_20200915': (31483, '186a9c5eefb491402bce0e09515f8d2aedaf1b668e8cdd7522d1a5991a1234fa'),
 'jp_dpfp_tamaki_video_comment_20201026': (31518, 'd707c155518ff06861d19876e9fc4427eba8df57447c01fd010607966f23061d'),
 'jp_dpfp_representative_election_20201218': (31072,
                                              '7a4c2de784bd1ca5d8eae87f15e704b9ac39f24ec61f438e27ab691df3c02384'),
 'jp_dpfp_new_officers_20201223': (29692, 'f18c3492cbb617a2307ece66cd2c55a1680896be6c25073ee5a7681b7947fcf0'),
 'jp_dpfp_representative_election_20230902': (58288,
                                              '22acf9b15b772e2e2489a8c0981d8b825e29b1475925bfec66628a64a58d0ad6'),
 'jp_dpfp_representative_press_conference_20230905': (56228,
                                                      '783123f6dc727b0a7666fe67e1dd4f926aaf9d9b3e04ca03aceca7a69f443baa'),
 'jp_dpfp_suspension_notice_20241204': (52624, 'b42b33ed7e7380e24af7f50547eb34c26bc8d5d0a7a88501dc25ed95208efda5'),
 'jp_dpfp_representative_press_conference_20250304': (60811,
                                                      'ebbfac6d2ef9e1358d92d94542b46976a7d4db45c3c65813b17c206975a79d14')}

SHA1 = {'jp_jcp_programme_report_20th_congress_19940719': '4KKNSEK7X6WSMOQK34POGYCOELPRKVPV',
 'jp_jcp_21st_congress_diary_19970926': 'DPW5NBUM73EBHAVVHXFXMEQO2C5JM7HE',
 'jp_jcp_21st_congress_honorary_officers_19970926': 'OW36YMW4M53QFBGQXKSMEVYLKVXGVKKS',
 'jp_jcp_21st_first_plenum_19970926': '6DNF5SWK5UEE34TJCXRXG7YRTHNUWR3E',
 'jp_jcp_21st_rules_amendment_proposal_19970925': 'MN74MQ6JGUEYLLYYIDGY5K34XHAH23MQ',
 'jp_jcp_22nd_congress_opening_address_20001120': 'YPKC2U2TF6PTTD3L4MD72ET3JQYZGC6J',
 'jp_jcp_22nd_officers_20001124': '3Z2M6PJITDFSIE4RNP63ZU6DATXMQ6HM',
 'jp_jcp_22nd_congress_closing_address_20001124': '43F4R52MDGWPFT43FITFWNT4DYYVNA7X',
 'jp_jcp_second_plenum_presidium_report_20010529': 'MOACRDHJYPD5BGJQ5GY6LTQMQFTXUNRT',
 'jp_jcp_23rd_first_plenum_20040117': 'DTSVN5UGMRUHW5K6X7RVP2AADATTSEMQ',
 'jp_jcp_23rd_congress_summation_20040117': 'D2L2BCGCIRD4HJYWK7ZK5RZZ5R4A3EYY',
 'jp_jcp_24th_congress_opening_address_20060112': '33EIPVFTE4HVC2MUYJKMWBKQQ5FWCIDM',
 'jp_jcp_24th_personnel_report_20060115': '6QRSJWA323P6MYXPMQMPVRNHKAFGFV4M',
 'jp_jcp_24th_congress_closing_address_20060114': 'C6G7NSHE3OJHPJRH36SCRLMDT44ARJOC',
 'jp_jcp_25th_congress_closing_address_20100116': 'PRE3AYPD36X7CA5X27M3K2TVLPLQ36BZ',
 'jp_jcp_26th_congress_summation_20140118': 'D5K3CQLKWLWYFZ5Z7Y36CSOEGBFKU4S4',
 'jp_jcp_27th_officers_20170118': 'TWHCQ5VFE5W3ZMDEVB4ICN3JAKMRAGBH',
 'jp_jcp_27th_congress_closing_address_20170118': 'EHQMBIH4X5ZMWITHJYC6HM6AIDVG33QB',
 'jp_jcp_28th_officers_20200118': 'DOKATG7TOL2WTS5QNPUT2HMXD6AF55F3',
 'jp_jcp_28th_congress_closing_address_20200118': 'LJHNBNMVTBB2VWVM2ZPOD4IPWKES53OR',
 'jp_dpfp_2018_founding_convention_20180507': 'RSD7HTY7V43RDO3ZNFZUHRHATCZ2M33B',
 'jp_dpfp_2018_representative_elected_20180904': 'PURQYK57EGMOPMCLLUEKMTKD2CYGY63A',
 'jp_dpfp_2018_representative_press_conference_20180904': 'DOWE5NNUZOA2VH5NEVVWIUDJZNRUQ5R5',
 'jp_dpfp_2018_party_dissolved_20200911': 'EOFMWMMZ4CYM6SK4NSE45LJAREG6WMBM',
 'jp_dpfp_founding_convention_20200915': 'BHEYL6VZGWW6BPG7HRUA5RYTVITP2KO6',
 'jp_dpfp_tamaki_video_comment_20201026': 'VT6KTQCWVUG7IMCY4RYSN2VNH2DX27V7',
 'jp_dpfp_representative_election_20201218': '2JZSTKDVGZJWGCQK2VJYAY7XLUOPRFWW',
 'jp_dpfp_new_officers_20201223': 'MLWQCTK3D7XBVY5ZP4TMKSDRXJCSW37G',
 'jp_dpfp_representative_election_20230902': 'YS4PPSNLO3RDSYUJ7VNDKLX2TDZGUKMF',
 'jp_dpfp_representative_press_conference_20230905': 'VJ4KVBUENDJXT7JEJDOB2U7KHSN7POQA',
 'jp_dpfp_suspension_notice_20241204': 'EBXBBLO7LM7UJIBGTGGXQPYNAUFL7T6Y',
 'jp_dpfp_representative_press_conference_20250304': '5MFSOVYTOBR4LOHTXHXPLO7KWPOULI3U'}

ARCHIVED = {'jp_jcp_programme_report_20th_congress_19940719': '20120604115302',
 'jp_jcp_21st_congress_diary_19970926': '20010718222448',
 'jp_jcp_21st_congress_honorary_officers_19970926': '20010718222059',
 'jp_jcp_21st_first_plenum_19970926': '20010718222700',
 'jp_jcp_21st_rules_amendment_proposal_19970925': '20010718223734',
 'jp_jcp_22nd_congress_opening_address_20001120': '20010717015239',
 'jp_jcp_22nd_officers_20001124': '20010717014642',
 'jp_jcp_22nd_congress_closing_address_20001124': '20010717015501',
 'jp_jcp_second_plenum_presidium_report_20010529': '20010717014512',
 'jp_jcp_23rd_first_plenum_20040117': '20040121025135',
 'jp_jcp_23rd_congress_summation_20040117': '20040121025200',
 'jp_jcp_24th_congress_opening_address_20060112': '20060204074827',
 'jp_jcp_24th_personnel_report_20060115': '20070209031400',
 'jp_jcp_24th_congress_closing_address_20060114': '20060330012516',
 'jp_jcp_25th_congress_closing_address_20100116': '20100120090447',
 'jp_jcp_26th_congress_summation_20140118': '20140121235351',
 'jp_jcp_27th_officers_20170118': '20170215023443',
 'jp_jcp_27th_congress_closing_address_20170118': '20170215023446',
 'jp_jcp_28th_officers_20200118': '20240725191524',
 'jp_jcp_28th_congress_closing_address_20200118': '20200119055123',
 'jp_dpfp_2018_founding_convention_20180507': '20190604072816',
 'jp_dpfp_2018_representative_elected_20180904': '20180905214906',
 'jp_dpfp_2018_representative_press_conference_20180904': '20190722045733',
 'jp_dpfp_2018_party_dissolved_20200911': '20200930195530',
 'jp_dpfp_founding_convention_20200915': '20201130051711',
 'jp_dpfp_tamaki_video_comment_20201026': '20201130041032',
 'jp_dpfp_representative_election_20201218': '20201218080538',
 'jp_dpfp_new_officers_20201223': '20201223072231',
 'jp_dpfp_representative_election_20230902': '20230926144708',
 'jp_dpfp_representative_press_conference_20230905': '20230926154937',
 'jp_dpfp_suspension_notice_20241204': '20241205053256',
 'jp_dpfp_representative_press_conference_20250304': '20250305120512'}

ORIGINAL_HOSTS = {'jp_jcp_programme_report_20th_congress_19940719': 'www.jcp.or.jp',
 'jp_jcp_21st_congress_diary_19970926': 'www.jcp.or.jp',
 'jp_jcp_21st_congress_honorary_officers_19970926': 'www.jcp.or.jp',
 'jp_jcp_21st_first_plenum_19970926': 'www.jcp.or.jp',
 'jp_jcp_21st_rules_amendment_proposal_19970925': 'www.jcp.or.jp',
 'jp_jcp_22nd_congress_opening_address_20001120': 'www.jcp.or.jp',
 'jp_jcp_22nd_officers_20001124': 'www.jcp.or.jp',
 'jp_jcp_22nd_congress_closing_address_20001124': 'www.jcp.or.jp',
 'jp_jcp_second_plenum_presidium_report_20010529': 'www.jcp.or.jp',
 'jp_jcp_23rd_first_plenum_20040117': 'www.jcp.or.jp',
 'jp_jcp_23rd_congress_summation_20040117': 'www.jcp.or.jp',
 'jp_jcp_24th_congress_opening_address_20060112': 'www.jcp.or.jp',
 'jp_jcp_24th_personnel_report_20060115': 'www.jcp.or.jp',
 'jp_jcp_24th_congress_closing_address_20060114': 'www.jcp.or.jp',
 'jp_jcp_25th_congress_closing_address_20100116': 'www.jcp.or.jp',
 'jp_jcp_26th_congress_summation_20140118': 'www.jcp.or.jp',
 'jp_jcp_27th_officers_20170118': 'www.jcp.or.jp',
 'jp_jcp_27th_congress_closing_address_20170118': 'www.jcp.or.jp',
 'jp_jcp_28th_officers_20200118': 'www.jcp.or.jp',
 'jp_jcp_28th_congress_closing_address_20200118': 'www.jcp.or.jp',
 'jp_dpfp_2018_founding_convention_20180507': 'www.dpfp.or.jp',
 'jp_dpfp_2018_representative_elected_20180904': 'www.dpfp.or.jp',
 'jp_dpfp_2018_representative_press_conference_20180904': 'www.dpfp.or.jp',
 'jp_dpfp_2018_party_dissolved_20200911': 'www.dpfp.or.jp',
 'jp_dpfp_founding_convention_20200915': 'new-kokumin.jp',
 'jp_dpfp_tamaki_video_comment_20201026': 'new-kokumin.jp',
 'jp_dpfp_representative_election_20201218': 'new-kokumin.jp',
 'jp_dpfp_new_officers_20201223': 'new-kokumin.jp',
 'jp_dpfp_representative_election_20230902': 'new-kokumin.jp',
 'jp_dpfp_representative_press_conference_20230905': 'new-kokumin.jp',
 'jp_dpfp_suspension_notice_20241204': 'new-kokumin.jp',
 'jp_dpfp_representative_press_conference_20250304': 'new-kokumin.jp'}

SOURCE_TYPES = {'jp_jcp_programme_report_20th_congress_19940719': 'primary_party_record_archived',
 'jp_jcp_21st_congress_diary_19970926': 'primary_party_record_archived',
 'jp_jcp_21st_congress_honorary_officers_19970926': 'primary_party_record_archived',
 'jp_jcp_21st_first_plenum_19970926': 'primary_party_record_archived',
 'jp_jcp_21st_rules_amendment_proposal_19970925': 'primary_party_rules_archived',
 'jp_jcp_22nd_congress_opening_address_20001120': 'primary_party_record_archived',
 'jp_jcp_22nd_officers_20001124': 'primary_party_record_archived',
 'jp_jcp_22nd_congress_closing_address_20001124': 'primary_party_record_archived',
 'jp_jcp_second_plenum_presidium_report_20010529': 'primary_party_record_archived',
 'jp_jcp_23rd_first_plenum_20040117': 'primary_party_publication_archived',
 'jp_jcp_23rd_congress_summation_20040117': 'primary_party_publication_archived',
 'jp_jcp_24th_congress_opening_address_20060112': 'primary_party_publication_archived',
 'jp_jcp_24th_personnel_report_20060115': 'primary_party_publication_archived',
 'jp_jcp_24th_congress_closing_address_20060114': 'primary_party_publication_archived',
 'jp_jcp_25th_congress_closing_address_20100116': 'primary_party_publication_archived',
 'jp_jcp_26th_congress_summation_20140118': 'primary_party_publication_archived',
 'jp_jcp_27th_officers_20170118': 'primary_party_record_archived',
 'jp_jcp_27th_congress_closing_address_20170118': 'primary_party_publication_archived',
 'jp_jcp_28th_officers_20200118': 'primary_party_record_archived',
 'jp_jcp_28th_congress_closing_address_20200118': 'primary_party_publication_archived',
 'jp_dpfp_2018_founding_convention_20180507': 'primary_party_record_archived',
 'jp_dpfp_2018_representative_elected_20180904': 'primary_party_record_archived',
 'jp_dpfp_2018_representative_press_conference_20180904': 'primary_party_record_archived',
 'jp_dpfp_2018_party_dissolved_20200911': 'primary_party_record_archived',
 'jp_dpfp_founding_convention_20200915': 'primary_party_publication_archived',
 'jp_dpfp_tamaki_video_comment_20201026': 'primary_party_publication_archived',
 'jp_dpfp_representative_election_20201218': 'primary_party_publication_archived',
 'jp_dpfp_new_officers_20201223': 'primary_party_publication_archived',
 'jp_dpfp_representative_election_20230902': 'primary_party_publication_archived',
 'jp_dpfp_representative_press_conference_20230905': 'primary_party_publication_archived',
 'jp_dpfp_suspension_notice_20241204': 'primary_party_publication_archived',
 'jp_dpfp_representative_press_conference_20250304': 'primary_party_publication_archived'}

# Records of the 国民民主党 of May 2018 to September 2020, an earlier organization with the same name: claims only.
OLD_PARTY_SOURCES = ['jp_dpfp_2018_founding_convention_20180507',
 'jp_dpfp_2018_representative_elected_20180904',
 'jp_dpfp_2018_representative_press_conference_20180904',
 'jp_dpfp_2018_party_dissolved_20200911']

# Every new claim's (attested_on, event_kind, review observation), exactly.
EVENTS = {'jp_jcp_fuwa_executive_chair_report_19940719': ('1994-07-19', 'in_office_attestation', 'JCP-01'),
 'jp_jcp_miyamoto_named_chair_by_executive_chair_19940719': ('1994-07-19', 'in_office_attestation', 'JCP-01'),
 'jp_jcp_miyamoto_absence_reported_19970923': ('1997-09-23', 'in_office_attestation', 'JCP-02'),
 'jp_jcp_rules_amendment_adopted_19970926': ('1997-09-26', 'party_rules_amended', 'JCP-02'),
 'jp_jcp_honorary_chair_introduced_19970926': ('1997-09-26', 'honorary_office_conferred', 'JCP-02'),
 'jp_jcp_fuwa_closing_address_diary_19970926': ('1997-09-26', 'in_office_attestation', 'JCP-02'),
 'jp_jcp_miyamoto_named_honorary_chair_19970926': ('1997-09-26', 'honorary_office_conferred', 'JCP-02'),
 'jp_jcp_21st_first_plenum_officers_elected_19970926': ('1997-09-26', 'central_committee_plenum_election', 'JCP-02'),
 'jp_jcp_fuwa_in_office_first_plenum_19970926': ('1997-09-26', 'in_office_attestation', 'JCP-02'),
 'jp_jcp_central_chair_made_optional_proposed': (None, 'party_rules_procedure', 'JCP-02'),
 'jp_jcp_fuwa_opening_address_20001120': ('2000-11-20', 'in_office_attestation', 'JCP-03'),
 'jp_jcp_fuwa_elected_central_chair_20001124': ('2000-11-24', 'central_committee_plenum_election', 'JCP-03'),
 'jp_jcp_shii_elected_executive_chair_20001124': ('2000-11-24', 'central_committee_plenum_election', 'JCP-03'),
 'jp_jcp_fuwa_central_chair_closing_address_20001124': ('2000-11-24', 'in_office_attestation', 'JCP-03'),
 'jp_jcp_shii_new_executive_chair_styled_20001124': ('2000-11-24', 'newly_elected_styling', 'JCP-03'),
 'jp_jcp_shii_executive_chair_report_20010529': ('2001-05-29', 'in_office_attestation', 'JCP-04'),
 'jp_jcp_23rd_first_plenum_four_officers_elected_20040117': ('2004-01-17',
                                                             'central_committee_plenum_election',
                                                             'JCP-04'),
 'jp_jcp_fuwa_central_chair_first_plenum_20040117': ('2004-01-17', 'in_office_attestation', 'JCP-04'),
 'jp_jcp_shii_executive_chair_summation_20040117': ('2004-01-17', 'in_office_attestation', 'JCP-04'),
 'jp_jcp_fuwa_central_chair_self_stated_20060112': ('2006-01-12', 'in_office_attestation', 'JCP-05'),
 'jp_jcp_fuwa_withdrawal_accepted_reported_20060115': ('2006-01-15', 'withdrawal_accepted_reported', 'JCP-05'),
 'jp_jcp_24th_first_plenum_three_officers_elected_20060115': ('2006-01-15',
                                                              'central_committee_plenum_election',
                                                              'JCP-05'),
 'jp_jcp_fuwa_offices_recalled_retrospective': (None, 'retrospective_office_reference', 'JCP-05'),
 'jp_jcp_shii_executive_chair_closing_address_20060114': ('2006-01-14', 'in_office_attestation', 'JCP-05'),
 'jp_jcp_fuwa_steps_down_confirmed_20060114': ('2006-01-14', 'withdrawal_confirmed', 'JCP-05'),
 'jp_jcp_shii_executive_chair_closing_address_20100116': ('2010-01-16', 'in_office_attestation', 'JCP-06'),
 'jp_jcp_shii_executive_chair_summation_20140118': ('2014-01-18', 'in_office_attestation', 'JCP-06'),
 'jp_jcp_shii_elected_executive_chair_20170118': ('2017-01-18', 'central_committee_plenum_election', 'JCP-06'),
 'jp_jcp_shii_executive_chair_closing_address_20170118': ('2017-01-18', 'in_office_attestation', 'JCP-06'),
 'jp_jcp_shii_elected_executive_chair_20200118': ('2020-01-18', 'central_committee_plenum_election', 'JCP-06'),
 'jp_jcp_shii_executive_chair_closing_address_20200118': ('2020-01-18', 'in_office_attestation', 'JCP-06'),
 'jp_dpfp_2018_party_founded_20180507': ('2018-05-07', 'organization_formed', 'DPFP-01'),
 'jp_dpfp_2018_co_representatives_approved_20180507': ('2018-05-07', 'co_representatives_approved', 'DPFP-01'),
 'jp_dpfp_2018_co_representatives_inaugural_greetings_20180507': ('2018-05-07',
                                                                  'other_office_attestation',
                                                                  'DPFP-01'),
 'jp_dpfp_2018_tamaki_elected_20180904': ('2018-09-04', 'other_office_election', 'DPFP-02'),
 'jp_dpfp_2018_second_representative_styled_20180904': ('2018-09-04', 'other_office_attestation', 'DPFP-02'),
 'jp_dpfp_2018_tamaki_assumption_stated_20180904': ('2018-09-04', 'other_office_assumption_stated', 'DPFP-02'),
 'jp_dpfp_2018_party_dissolved_20200911': ('2020-09-11', 'organization_dissolved', 'DPFP-02'),
 'jp_dpfp_2018_tamaki_final_greeting_20200911': ('2020-09-11', 'other_office_attestation', 'DPFP-02'),
 'jp_dpfp_party_founded_20200915': ('2020-09-15', 'organization_formed', 'DPFP-03'),
 'jp_dpfp_tamaki_selected_founding_convention_20200915': ('2020-09-15', 'convention_selection', 'DPFP-03'),
 'jp_dpfp_tamaki_in_office_20201026': ('2020-10-26', 'in_office_attestation', 'DPFP-03'),
 'jp_dpfp_tamaki_elected_20201218': ('2020-12-18', 'convention_selection', 'DPFP-03'),
 'jp_dpfp_tamaki_in_office_20201223': ('2020-12-23', 'in_office_attestation', 'DPFP-03'),
 'jp_dpfp_tamaki_elected_20230902': ('2023-09-02', 'convention_selection', 'DPFP-04'),
 'jp_dpfp_tamaki_in_office_20230905': ('2023-09-05', 'in_office_attestation', 'DPFP-04'),
 'jp_dpfp_tamaki_suspended_from_posts_20241204': ('2024-12-04', 'suspension_from_posts', 'DPFP-05'),
 'jp_dpfp_tamaki_in_office_20250304': ('2025-03-04', 'in_office_attestation', 'DPFP-05'),
 'jp_dpfp_suspension_recalled_20250304': ('2025-03-04', 'suspension_recalled', 'DPFP-05'),
 'jp_dpfp_furukawa_acting_recalled_20250304': ('2025-03-04', 'acting_service_recalled', 'DPFP-05')}

# The holder name each extract row carries; None where the row names no holder of its own office.
ROW_HOLDERS = {'jp_jcp_fuwa_executive_chair_report_19940719': '不破哲三',
 'jp_jcp_miyamoto_named_chair_by_executive_chair_19940719': '宮本顕治',
 'jp_jcp_miyamoto_absence_reported_19970923': '宮本顕治',
 'jp_jcp_rules_amendment_adopted_19970926': None,
 'jp_jcp_honorary_chair_introduced_19970926': None,
 'jp_jcp_fuwa_closing_address_diary_19970926': '不破哲三',
 'jp_jcp_miyamoto_named_honorary_chair_19970926': None,
 'jp_jcp_21st_first_plenum_officers_elected_19970926': None,
 'jp_jcp_fuwa_in_office_first_plenum_19970926': '不破哲三',
 'jp_jcp_central_chair_made_optional_proposed': None,
 'jp_jcp_fuwa_opening_address_20001120': '不破哲三',
 'jp_jcp_fuwa_elected_central_chair_20001124': '不破哲三',
 'jp_jcp_shii_elected_executive_chair_20001124': '志位和夫',
 'jp_jcp_fuwa_central_chair_closing_address_20001124': '不破哲三',
 'jp_jcp_shii_new_executive_chair_styled_20001124': '志位和夫',
 'jp_jcp_shii_executive_chair_report_20010529': '志位和夫',
 'jp_jcp_23rd_first_plenum_four_officers_elected_20040117': None,
 'jp_jcp_fuwa_central_chair_first_plenum_20040117': '不破哲三',
 'jp_jcp_shii_executive_chair_summation_20040117': '志位和夫',
 'jp_jcp_fuwa_central_chair_self_stated_20060112': '不破哲三',
 'jp_jcp_fuwa_withdrawal_accepted_reported_20060115': '不破哲三',
 'jp_jcp_24th_first_plenum_three_officers_elected_20060115': None,
 'jp_jcp_fuwa_offices_recalled_retrospective': '不破哲三',
 'jp_jcp_shii_executive_chair_closing_address_20060114': '志位和夫',
 'jp_jcp_fuwa_steps_down_confirmed_20060114': '不破哲三',
 'jp_jcp_shii_executive_chair_closing_address_20100116': '志位和夫',
 'jp_jcp_shii_executive_chair_summation_20140118': '志位和夫',
 'jp_jcp_shii_elected_executive_chair_20170118': '志位和夫',
 'jp_jcp_shii_executive_chair_closing_address_20170118': '志位和夫',
 'jp_jcp_shii_elected_executive_chair_20200118': '志位和夫',
 'jp_jcp_shii_executive_chair_closing_address_20200118': '志位和夫',
 'jp_dpfp_2018_party_founded_20180507': None,
 'jp_dpfp_2018_co_representatives_approved_20180507': None,
 'jp_dpfp_2018_co_representatives_inaugural_greetings_20180507': None,
 'jp_dpfp_2018_tamaki_elected_20180904': None,
 'jp_dpfp_2018_second_representative_styled_20180904': None,
 'jp_dpfp_2018_tamaki_assumption_stated_20180904': None,
 'jp_dpfp_2018_party_dissolved_20200911': None,
 'jp_dpfp_2018_tamaki_final_greeting_20200911': None,
 'jp_dpfp_party_founded_20200915': None,
 'jp_dpfp_tamaki_selected_founding_convention_20200915': '玉木雄一郎',
 'jp_dpfp_tamaki_in_office_20201026': '玉木雄一郎',
 'jp_dpfp_tamaki_elected_20201218': '玉木雄一郎',
 'jp_dpfp_tamaki_in_office_20201223': '玉木雄一郎',
 'jp_dpfp_tamaki_elected_20230902': '玉木雄一郎',
 'jp_dpfp_tamaki_in_office_20230905': '玉木雄一郎',
 'jp_dpfp_tamaki_suspended_from_posts_20241204': None,
 'jp_dpfp_tamaki_in_office_20250304': '玉木雄一郎',
 'jp_dpfp_suspension_recalled_20250304': None,
 'jp_dpfp_furukawa_acting_recalled_20250304': None}

ROW_ROLES = {'jp_jcp_fuwa_executive_chair_report_19940719': 'jp_jcp_executive_committee_chair',
 'jp_jcp_miyamoto_named_chair_by_executive_chair_19940719': 'jp_jcp_central_committee_chair',
 'jp_jcp_miyamoto_absence_reported_19970923': 'jp_jcp_central_committee_chair',
 'jp_jcp_rules_amendment_adopted_19970926': 'jp_jcp_central_committee_chair',
 'jp_jcp_honorary_chair_introduced_19970926': 'jp_jcp_central_committee_chair',
 'jp_jcp_fuwa_closing_address_diary_19970926': 'jp_jcp_executive_committee_chair',
 'jp_jcp_miyamoto_named_honorary_chair_19970926': 'jp_jcp_central_committee_chair',
 'jp_jcp_21st_first_plenum_officers_elected_19970926': 'jp_jcp_executive_committee_chair',
 'jp_jcp_fuwa_in_office_first_plenum_19970926': 'jp_jcp_executive_committee_chair',
 'jp_jcp_central_chair_made_optional_proposed': 'jp_jcp_central_committee_chair',
 'jp_jcp_fuwa_opening_address_20001120': 'jp_jcp_executive_committee_chair',
 'jp_jcp_fuwa_elected_central_chair_20001124': 'jp_jcp_central_committee_chair',
 'jp_jcp_shii_elected_executive_chair_20001124': 'jp_jcp_executive_committee_chair',
 'jp_jcp_fuwa_central_chair_closing_address_20001124': 'jp_jcp_central_committee_chair',
 'jp_jcp_shii_new_executive_chair_styled_20001124': 'jp_jcp_executive_committee_chair',
 'jp_jcp_shii_executive_chair_report_20010529': 'jp_jcp_executive_committee_chair',
 'jp_jcp_23rd_first_plenum_four_officers_elected_20040117': 'jp_jcp_central_committee_chair',
 'jp_jcp_fuwa_central_chair_first_plenum_20040117': 'jp_jcp_central_committee_chair',
 'jp_jcp_shii_executive_chair_summation_20040117': 'jp_jcp_executive_committee_chair',
 'jp_jcp_fuwa_central_chair_self_stated_20060112': 'jp_jcp_central_committee_chair',
 'jp_jcp_fuwa_withdrawal_accepted_reported_20060115': 'jp_jcp_central_committee_chair',
 'jp_jcp_24th_first_plenum_three_officers_elected_20060115': 'jp_jcp_executive_committee_chair',
 'jp_jcp_fuwa_offices_recalled_retrospective': 'jp_jcp_central_committee_chair',
 'jp_jcp_shii_executive_chair_closing_address_20060114': 'jp_jcp_executive_committee_chair',
 'jp_jcp_fuwa_steps_down_confirmed_20060114': 'jp_jcp_central_committee_chair',
 'jp_jcp_shii_executive_chair_closing_address_20100116': 'jp_jcp_executive_committee_chair',
 'jp_jcp_shii_executive_chair_summation_20140118': 'jp_jcp_executive_committee_chair',
 'jp_jcp_shii_elected_executive_chair_20170118': 'jp_jcp_executive_committee_chair',
 'jp_jcp_shii_executive_chair_closing_address_20170118': 'jp_jcp_executive_committee_chair',
 'jp_jcp_shii_elected_executive_chair_20200118': 'jp_jcp_executive_committee_chair',
 'jp_jcp_shii_executive_chair_closing_address_20200118': 'jp_jcp_executive_committee_chair',
 'jp_dpfp_2018_party_founded_20180507': 'jp_dpfp_representative',
 'jp_dpfp_2018_co_representatives_approved_20180507': 'jp_dpfp_representative',
 'jp_dpfp_2018_co_representatives_inaugural_greetings_20180507': 'jp_dpfp_representative',
 'jp_dpfp_2018_tamaki_elected_20180904': 'jp_dpfp_representative',
 'jp_dpfp_2018_second_representative_styled_20180904': 'jp_dpfp_representative',
 'jp_dpfp_2018_tamaki_assumption_stated_20180904': 'jp_dpfp_representative',
 'jp_dpfp_2018_party_dissolved_20200911': 'jp_dpfp_representative',
 'jp_dpfp_2018_tamaki_final_greeting_20200911': 'jp_dpfp_representative',
 'jp_dpfp_party_founded_20200915': 'jp_dpfp_representative',
 'jp_dpfp_tamaki_selected_founding_convention_20200915': 'jp_dpfp_representative',
 'jp_dpfp_tamaki_in_office_20201026': 'jp_dpfp_representative',
 'jp_dpfp_tamaki_elected_20201218': 'jp_dpfp_representative',
 'jp_dpfp_tamaki_in_office_20201223': 'jp_dpfp_representative',
 'jp_dpfp_tamaki_elected_20230902': 'jp_dpfp_representative',
 'jp_dpfp_tamaki_in_office_20230905': 'jp_dpfp_representative',
 'jp_dpfp_tamaki_suspended_from_posts_20241204': 'jp_dpfp_representative',
 'jp_dpfp_tamaki_in_office_20250304': 'jp_dpfp_representative',
 'jp_dpfp_suspension_recalled_20250304': 'jp_dpfp_representative',
 'jp_dpfp_furukawa_acting_recalled_20250304': 'jp_dpfp_representative'}

# Rows about another office, an honorary office or acting service carry that office's title.
OTHER_TITLES = {'jp_jcp_honorary_chair_introduced_19970926': '名誉議長 — honorary chair of the Central Committee (another office; '
                                              'claims only)',
 'jp_jcp_miyamoto_named_honorary_chair_19970926': '名誉議長 — honorary chair of the Central Committee (another office; '
                                                  'claims only)',
 'jp_dpfp_2018_co_representatives_approved_20180507': '共同代表 — co-representative of the 国民民主党 of May 2018 to '
                                                      'September 2020 (旧・国民民主党; an earlier organization with the '
                                                      'same name; claims only)',
 'jp_dpfp_2018_co_representatives_inaugural_greetings_20180507': '共同代表 — co-representative of the 国民民主党 of May 2018 '
                                                                 'to September 2020 (旧・国民民主党; an earlier '
                                                                 'organization with the same name; claims only)',
 'jp_dpfp_2018_tamaki_elected_20180904': '代表 — representative of the 国民民主党 of May 2018 to September 2020 (旧・国民民主党; '
                                         'an earlier organization with the same name; claims only)',
 'jp_dpfp_2018_second_representative_styled_20180904': '代表 — representative of the 国民民主党 of May 2018 to September '
                                                       '2020 (旧・国民民主党; an earlier organization with the same name; '
                                                       'claims only)',
 'jp_dpfp_2018_tamaki_assumption_stated_20180904': '代表 — representative of the 国民民主党 of May 2018 to September 2020 '
                                                   '(旧・国民民主党; an earlier organization with the same name; claims '
                                                   'only)',
 'jp_dpfp_2018_tamaki_final_greeting_20200911': '代表 — representative of the 国民民主党 of May 2018 to September 2020 '
                                                '(旧・国民民主党; an earlier organization with the same name; claims only)',
 'jp_dpfp_furukawa_acting_recalled_20250304': '代表代行 — acting representative while the representative was suspended '
                                              'from party posts (acting service; claims only)'}

# Exact holder observations of each role, (name, attested_on, from, until), in chronological order.
HOLDERS = {'jp_jcp_executive_committee_chair': [('不破哲三', '1994-07-19', None, None),
                                      ('不破哲三', '1997-09-26', None, None),
                                      ('不破哲三', '2000-11-20', None, None),
                                      ('志位和夫', '2001-05-29', None, None),
                                      ('志位和夫', '2004-01-17', None, None),
                                      ('志位和夫', '2006-01-14', None, None),
                                      ('志位和夫', '2010-01-16', None, None),
                                      ('志位和夫', '2014-01-18', None, None),
                                      ('志位和夫', '2017-01-18', None, None),
                                      ('志位和夫', '2020-01-18', None, None),
                                      ('田村智子', '2024-01-18', None, None)],
 'jp_jcp_central_committee_chair': [('宮本顕治', '1994-07-19', None, None),
                                    ('宮本顕治', '1997-09-23', None, None),
                                    ('不破哲三', '2000-11-24', None, None),
                                    ('不破哲三', '2004-01-17', None, None),
                                    ('不破哲三', '2006-01-12', None, None),
                                    ('志位和夫', '2024-01-18', None, None)],
 'jp_dpfp_representative': [('玉木雄一郎', '2020-10-26', None, None),
                            ('玉木雄一郎', '2020-12-23', None, None),
                            ('玉木雄一郎', '2023-09-05', None, None),
                            ('玉木雄一郎', '2025-03-04', None, None),
                            ('玉木雄一郎', '2026-09-06', None, None)]}

HOLDER_CLAIMS = {'jp_jcp_executive_committee_chair': [['jp_jcp_fuwa_executive_chair_report_19940719'],
                                      ['jp_jcp_fuwa_closing_address_diary_19970926',
                                       'jp_jcp_fuwa_in_office_first_plenum_19970926'],
                                      ['jp_jcp_fuwa_opening_address_20001120'],
                                      ['jp_jcp_shii_executive_chair_report_20010529'],
                                      ['jp_jcp_shii_executive_chair_summation_20040117'],
                                      ['jp_jcp_shii_executive_chair_closing_address_20060114'],
                                      ['jp_jcp_shii_executive_chair_closing_address_20100116'],
                                      ['jp_jcp_shii_executive_chair_summation_20140118'],
                                      ['jp_jcp_shii_executive_chair_closing_address_20170118'],
                                      ['jp_jcp_shii_executive_chair_closing_address_20200118'],
                                      ['jp_tamura_executive_chair_20240118']],
 'jp_jcp_central_committee_chair': [['jp_jcp_miyamoto_named_chair_by_executive_chair_19940719'],
                                    ['jp_jcp_miyamoto_absence_reported_19970923'],
                                    ['jp_jcp_fuwa_central_chair_closing_address_20001124'],
                                    ['jp_jcp_fuwa_central_chair_first_plenum_20040117'],
                                    ['jp_jcp_fuwa_central_chair_self_stated_20060112'],
                                    ['jp_shii_central_chair_20240118']],
 'jp_dpfp_representative': [['jp_dpfp_tamaki_in_office_20201026'],
                            ['jp_dpfp_tamaki_in_office_20201223'],
                            ['jp_dpfp_tamaki_in_office_20230905'],
                            ['jp_dpfp_tamaki_in_office_20250304'],
                            ['jp_tamaki_representative_20260906']]}

PEOPLE_BY_ROLE = {'jp_jcp_executive_committee_chair': ['不破哲三', '志位和夫', '田村智子'],
 'jp_jcp_central_committee_chair': ['宮本顕治', '不破哲三', '志位和夫'],
 'jp_dpfp_representative': ['玉木雄一郎']}

PEOPLE = ['宮本顕治', '不破哲三', '志位和夫', '田村智子', '玉木雄一郎', '大塚耕平', '古川元久']

# Named in claims only: the 2018 co-representative and the 2024-2025 acting representative.
OTHER_PEOPLE = ['大塚耕平', '古川元久']

SURNAMES = {'宮本顕治': '宮本', '不破哲三': '不破', '志位和夫': '志位', '玉木雄一郎': '玉木'}

# Days that are never a holder's day for that office: elections, 'new' stylings, withdrawals, rule changes, the earlier
# party's events, suspensions and stated term ends.
NEVER_HOLDER_DATE = {'jp_jcp_executive_committee_chair': {'1997-09-23', '2006-01-12', '2006-01-15', '2000-11-24', '1997-09-25'},
 'jp_jcp_central_committee_chair': {'2000-11-20', '1997-09-25', '1997-09-26', '2006-01-14', '2006-01-15'},
 'jp_dpfp_representative': {'2018-05-07',
                            '2018-09-04',
                            '2020-09-11',
                            '2020-09-15',
                            '2020-12-18',
                            '2023-09-02',
                            '2023-09-30',
                            '2024-12-04',
                            '2025-03-03',
                            '2026-09-30'}}

# On each day with several new claims, exactly these event kinds stay separate claims.
TRANSITIONS = {'1997-09-26': ['central_committee_plenum_election',
                'honorary_office_conferred',
                'in_office_attestation',
                'party_rules_amended'],
 '2000-11-24': ['central_committee_plenum_election', 'in_office_attestation', 'newly_elected_styling'],
 '2004-01-17': ['central_committee_plenum_election', 'in_office_attestation'],
 '2006-01-14': ['in_office_attestation', 'withdrawal_confirmed'],
 '2006-01-15': ['central_committee_plenum_election', 'withdrawal_accepted_reported'],
 '2017-01-18': ['central_committee_plenum_election', 'in_office_attestation'],
 '2018-05-07': ['co_representatives_approved', 'organization_formed', 'other_office_attestation'],
 '2018-09-04': ['other_office_assumption_stated', 'other_office_attestation', 'other_office_election'],
 '2020-01-18': ['central_committee_plenum_election', 'in_office_attestation'],
 '2020-09-11': ['organization_dissolved', 'other_office_attestation'],
 '2020-09-15': ['convention_selection', 'organization_formed'],
 '2025-03-04': ['acting_service_recalled', 'in_office_attestation', 'suspension_recalled']}

REVIEW = ['JCP-01', 'JCP-02', 'JCP-03', 'JCP-04', 'JCP-05', 'JCP-06', 'DPFP-01', 'DPFP-02', 'DPFP-03', 'DPFP-04', 'DPFP-05']

COUNTS = {'sources_claims': (32, 50), 'holder_never': (20, 30), 'categories': (20, 11, 2, 2, 8, 3, 3, 1), 'undated': 2}

ORG_UNRESOLVED_LEAD = {'jp_sangiin_pr_2025_01': 'JCP chairs 1990-2026 (CLAUDE-C01-42)',
 'jp_sangiin_pr_2025_07': 'DPFP representatives 2020-2026 (CLAUDE-C01-42)'}

LEAD_REPORT_MARKERS = ['aik25/2025-12-31', 'yakuin-26th', '112905261X00819940524', 'daihyo2020-tamaki-prof', 'web_jcp/history']

KIND_GROUPS = {
    'attest': {'in_office_attestation'},
    'election': {'central_committee_plenum_election', 'newly_elected_styling', 'convention_selection'},
    'departure': {'withdrawal_accepted_reported', 'withdrawal_confirmed'},
    'procedure': {'party_rules_amended', 'party_rules_procedure'},
    'other_office': {'honorary_office_conferred', 'co_representatives_approved', 'other_office_attestation',
                     'other_office_election', 'other_office_assumption_stated'},
    'organization': {'organization_formed', 'organization_dissolved'},
    'interim': {'suspension_from_posts', 'suspension_recalled', 'acting_service_recalled'},
    'retrospective': {'retrospective_office_reference'},
}
ALL_KINDS = set().union(*KIND_GROUPS.values())
UNDATED_KINDS = KIND_GROUPS['retrospective'] | {'party_rules_procedure'}
ROLE_TITLES = {EXEC: T_EXEC, CENTRAL: T_CENTRAL, DPFP: T_DPFP}
ROLE_ORG = {EXEC: JCP_ORG, CENTRAL: JCP_ORG, DPFP: DPFP_ORG}
NEW_SOURCES = list(RESPONSES)
NEW_CLAIMS = list(EVENTS)
HOLDER_CLAIM_SET = {cid for role in HOLDER_CLAIMS for ids in HOLDER_CLAIMS[role] for cid in ids if cid in EVENTS}
NEVER_HOLDER = tuple(cid for cid in EVENTS if cid not in HOLDER_CLAIM_SET)
GROUPED = {name: tuple(cid for cid, (_d, kind, _o) in EVENTS.items() if kind in kinds) for name, kinds in KIND_GROUPS.items()}
UNDATED = tuple(cid for cid, (day, _k, _o) in EVENTS.items() if day is None)
PER_REQUEST_PATTERNS = ('cb=', '_cb', 'cbx=', 'any=', 'token=', 'sessionid', 'X-Amz-', 'Signature=', 'searchResult',
                        'opensearch', 'cdx/search', '?q=', '&q=', 'speaker=', 'from=', 'until=', 'startRecord', '?s=',
                        'fbclid', 'nocache', 'wp-json', 'doing_wp_cron', '/page/')
LEAD_URL_MARKERS = ('wikipedia', 'britannica', 'kotobank', 'nikkei', 'asahi.com', 'yomiuri', 'mainichi', 'nhk.or.jp',
                    'jiji.com', 'sankei', 'kyodo', 'bing.com', 'google.')
REPORT = research.RESEARCH / 'japan-jcp-chairs-dpfp-representatives-1990-2026-42.md'
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-42.md'


def load_rows(packet):
    rows = {}
    for source in packet['sources'][EARLIER_SOURCE_COUNT:EARLIER_SOURCE_COUNT + len(NEW_SOURCES)]:
        extract = json.loads((research.ROOT / source['snapshot']['path']).read_text(encoding='utf-8'))
        rows.update({row['claim_id']: row for row in extract['rows']})
    return rows


def roles_of(packet):
    orgs = {o['id']: o for o in packet['organizations']}
    return {r['id']: r for org_id in (JCP_ORG, DPFP_ORG) for r in orgs[org_id]['roles']}


def leaders_rules(packet, rows):
    """Rule-based checks that hold without the pinned holder lists; raise AssertionError, KeyError or IndexError."""
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    source_of = {c['id']: s['id'] for s in packet['sources'] for c in s['claims']}
    orgs = {o['id']: o for o in packet['organizations']}
    lifecycle = {'status': 'unknown', 'from': None, 'until': None,
                 'note': 'Participation at one election does not establish organization birth, dissolution or continuity.'}
    for org_id, name, role_ids in ((JCP_ORG, '日本共産党', [EXEC, CENTRAL]), (DPFP_ORG, '国民民主党', [DPFP])):
        org = orgs[org_id]
        assert (org['name'], org['lifecycle'], org['represented_party_ids']) == (name, lifecycle, []), org_id
        assert [r['id'] for r in org['roles']] == role_ids, org_id
    assert len(packet['organizations']) == 16 and [i['id'] for i in packet['institutions']] == GROUPS + ['jp_prime_minister']
    assert sorted(r['id'] for e in packet['organizations'] + packet['institutions'] for r in e['roles']) == sorted(ALL_ROLES)
    assert all(not e['represented_party_ids'] for e in packet['organizations'] + packet['institutions'])
    roles = roles_of(packet)
    cited_by = {}
    for rid, role in roles.items():
        assert (role['title'], role['kind']) == (ROLE_TITLES[rid], 'party_chair' if rid != DPFP else 'party_leader'), rid
        assert list(role) == ['id', 'title', 'kind', 'sources', 'claim_ids', 'holder_claims', 'scope_note'], rid
        holders = role['holder_claims']
        earlier = [h for h in holders if h['claim_ids'][0] not in EVENTS]
        assert earlier == EARLIER_HOLDERS[rid], (rid, 'the earlier holder observations stay exactly as they were')
        previous = ''
        for holder in holders:
            name = holder['name']
            dated = [x for x in (holder['attested_on'], holder['from']) if x]
            assert len(dated) == 1 and holder['from'] is None and holder['until'] is None, (rid, name, 'no start or end')
            day = dated[0]
            assert day > previous, (rid, name, 'holders stay in chronological order')
            previous = day
            assert day <= research.CUTOFF and day not in NEVER_HOLDER_DATE[rid], (rid, name, day)
            if holder in earlier:
                continue
            assert name in PEOPLE_BY_ROLE[rid], (rid, name)
            assert list(holder) == ['name', 'attested_on', 'from', 'until', 'sources', 'claim_ids', 'note', 'uncertainty']
            expected = []
            for cid in holder['claim_ids']:
                row, claim = rows[cid], claims[cid]
                assert cid in role['claim_ids'] and cid.startswith(PREFIX[rid]), (rid, cid)
                assert source_of[cid] not in OLD_PARTY_SOURCES, (rid, cid, 'the earlier 国民民主党 never dates a holder')
                assert (row['role_id'], row['role_title'], row['holder_name']) == (rid, ROLE_TITLES[rid], name), (rid, cid)
                assert row['event_kind'] in KIND_GROUPS['attest'], (rid, cid)
                assert claim['attested_on'] == day == row['attested_on'], (rid, cid, "a cited claim carries the holder's day")
                assert SURNAMES[name] in claim['text'], (rid, cid)
                cited_by.setdefault(cid, []).append((rid, name))
                if source_of[cid] not in expected:
                    expected.append(source_of[cid])
            assert holder['sources'] == expected, (rid, name)
    # Every named in-office attestation of an office's own title is cited by exactly one holder of that office; no other
    # row is cited by any holder.
    for cid, row in rows.items():
        kind, name, rid = row['event_kind'], row['holder_name'], row['role_id']
        assert kind in ALL_KINDS and rid in ROLE_TITLES, (cid, kind, rid)
        assert row['observation_id'] == ROLE_ORG[rid] and cid.startswith(PREFIX[rid]), cid
        own = row['role_title'] == ROLE_TITLES[rid]
        if kind in KIND_GROUPS['attest'] and name and own:
            assert cited_by.get(cid) == [(rid, name)], cid
        else:
            assert cid not in cited_by, cid
        if kind in KIND_GROUPS['other_office'] | KIND_GROUPS['interim'] | KIND_GROUPS['organization'] | KIND_GROUPS['procedure']:
            assert name is None, cid
        if kind in KIND_GROUPS['other_office'] or kind == 'acting_service_recalled':
            assert not own, cid
        else:
            assert own, cid
        if source_of[cid] in OLD_PARTY_SOURCES:
            assert name is None and kind in KIND_GROUPS['other_office'] | KIND_GROUPS['organization'], cid
        if kind in UNDATED_KINDS:
            assert 'attested_on' not in claims[cid] and row['attested_on'] is None, cid
        if 'attested_on' in claims[cid]:
            assert claims[cid]['attested_on'] == row['attested_on'] <= research.CUTOFF, cid
        else:
            assert row['attested_on'] is None, cid
    # Party office and state office never feed each other, the two JCP chairs never share a claim, and no other entry
    # cites this packet's claims or sources.
    new_claims = set(rows)
    new_sources = {source_of[cid] for cid in rows}
    exec_claims = set(roles[EXEC]['claim_ids']) | {c for h in roles[EXEC]['holder_claims'] for c in h['claim_ids']}
    central_claims = set(roles[CENTRAL]['claim_ids']) | {c for h in roles[CENTRAL]['holder_claims'] for c in h['claim_ids']}
    assert not exec_claims & central_claims, 'the two chair titles never share a claim'
    for entry in packet['organizations'] + packet['institutions']:
        mine = entry['id'] in (JCP_ORG, DPFP_ORG)
        for other in entry['roles']:
            if other['id'] in roles:
                continue
            assert not set(other['claim_ids']) & new_claims and not set(other['sources']) & new_sources, other['id']
            assert not {c for h in other['holder_claims'] for c in h['claim_ids']} & new_claims, other['id']
        if not mine:
            assert not set(entry['claim_ids']) & new_claims and not set(entry['sources']) & new_sources, entry['id']
    for rid, role in roles.items():
        assert {cid for cid in role['claim_ids'] if cid in rows} == {cid for cid, row in rows.items() if row['role_id'] == rid}, rid
        org = orgs[ROLE_ORG[rid]]
        assert set(role['claim_ids']) <= set(org['claim_ids']) and set(role['sources']) <= set(org['sources']), rid
    pm_role = packet['institutions'][-1]['roles'][0]
    assert len(pm_role['holder_claims']) == PM_HOLDER_COUNT


def leaders_invariants(packet, rows):
    """The rules plus the exact pinned holders, rows and events this packet intends."""
    leaders_rules(packet, rows)
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    roles = roles_of(packet)
    for rid, role in roles.items():
        got = [(h['name'], h['attested_on'], h['from'], h['until']) for h in role['holder_claims']]
        assert got == HOLDERS[rid], (rid, got)
        assert [h['claim_ids'] for h in role['holder_claims']] == HOLDER_CLAIMS[rid], rid
    assert {cid: row['holder_name'] for cid, row in rows.items()} == ROW_HOLDERS
    assert {cid: row['role_id'] for cid, row in rows.items()} == ROW_ROLES
    assert {cid: row['role_title'] for cid, row in rows.items() if row['role_title'] != ROLE_TITLES[row['role_id']]} == OTHER_TITLES
    for cid, (day, kind, obs) in EVENTS.items():
        assert claims[cid].get('attested_on') == day, cid
        assert (rows[cid]['attested_on'], rows[cid]['event_kind'], rows[cid]['review_observation']) == (day, kind, obs), cid


class JapanJcpDpfpLeadersTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.raw = (research.ROOT / research.RESEARCH / 'japan.json').read_text(encoding='utf-8')
        cls.packet = json.loads(cls.raw)
        cls.sources = {s['id']: s for s in cls.packet['sources']}
        cls.claims = {c['id']: c for s in cls.packet['sources'] for c in s['claims']}
        cls.orgs = {o['id']: o for o in cls.packet['organizations']}
        cls.roles = roles_of(cls.packet)
        cls.extracts = {sid: json.loads((research.ROOT / cls.sources[sid]['snapshot']['path']).read_text(encoding='utf-8'))
                        for sid in NEW_SOURCES}
        cls.rows = load_rows(cls.packet)
        cls.new_claims = [c['id'] for sid in NEW_SOURCES for c in cls.sources[sid]['claims']]
        cls.report = (research.ROOT / REPORT).read_text(encoding='utf-8')

    def validate(self, packet=None):
        return research.validate(packet or self.packet, research.ROOT, {'Japan'}, {'Japan': set()})

    def section(self, heading):
        parts = self.report.split(f'\n## {heading}', 1)
        self.assertEqual(len(parts), 2, heading)
        return parts[1].split('\n## ', 1)[0]

    def test_new_records_are_bounded_and_every_claim_is_classified(self):
        ids = self.validate()
        self.assertEqual((len(NEW_SOURCES), len(self.new_claims)), COUNTS['sources_claims'])
        order = [s['id'] for s in self.packet['sources']]
        self.assertEqual(order[EARLIER_SOURCE_COUNT:], NEW_SOURCES)
        # CLAUDE-C01-31's 31 Komeito sources, accepted into integration, end exactly where this packet's begin.
        self.assertTrue(all(sid.startswith('jp_komeito_') for sid in order[EARLIER_SOURCE_COUNT - 31:EARLIER_SOURCE_COUNT]))
        self.assertFalse(order[EARLIER_SOURCE_COUNT - 32].startswith('jp_komeito_'))
        self.assertFalse([sid for sid in order[:EARLIER_SOURCE_COUNT] if sid in RESPONSES])
        self.assertEqual((len(ids['entries']), len(ids['roles'])), (24, 7))
        self.assertEqual((len(self.packet['organizations']), len(self.packet['institutions'])), (16, 8))
        self.assertEqual(self.new_claims, NEW_CLAIMS)
        self.assertEqual(set(EVENTS), set(self.rows))
        self.assertFalse(HOLDER_CLAIM_SET & set(NEVER_HOLDER))
        self.assertEqual(HOLDER_CLAIM_SET | set(NEVER_HOLDER), set(self.new_claims))
        self.assertEqual((len(HOLDER_CLAIM_SET), len(NEVER_HOLDER)), COUNTS['holder_never'])
        self.assertEqual(tuple(len(GROUPED[g]) for g in KIND_GROUPS), COUNTS['categories'])
        self.assertEqual(len(UNDATED), COUNTS['undated'])
        # Each role and organization cites its earlier claims and sources first, then exactly the new ones that concern it.
        for rid, role in self.roles.items():
            mine = [cid for cid in NEW_CLAIMS if ROW_ROLES[cid] == rid]
            sources = list(dict.fromkeys(sid for sid in NEW_SOURCES for c in self.sources[sid]['claims'] if ROW_ROLES[c['id']] == rid))
            self.assertEqual(role['claim_ids'], ROLE_EARLIER[rid][1] + mine, rid)
            self.assertEqual(role['sources'], ROLE_EARLIER[rid][0] + sources, rid)
        for org_id, (sources, claims, _n) in ORG_EARLIER.items():
            mine = [cid for cid in NEW_CLAIMS if ROLE_ORG[ROW_ROLES[cid]] == org_id]
            mine_sources = list(dict.fromkeys(sid for sid in NEW_SOURCES
                                              if ROLE_ORG[ROW_ROLES[self.sources[sid]['claims'][0]['id']]] == org_id))
            self.assertEqual(self.orgs[org_id]['claim_ids'], claims + mine, org_id)
            self.assertEqual(self.orgs[org_id]['sources'], sources + mine_sources, org_id)
        observations = re.findall(r'^### ((?:JCP|DPFP)-\d\d)\b', self.report, re.M)
        self.assertEqual(observations, REVIEW)
        self.assertEqual({row['review_observation'] for row in self.rows.values()}, set(REVIEW))
        people = list(dict.fromkeys([h[0] for rid in HOLDERS for h in HOLDERS[rid]] + OTHER_PEOPLE))
        self.assertEqual(sorted(people), sorted(PEOPLE))
        self.assertLessEqual(len(people), 10)

    def test_holders_are_exactly_as_intended(self):
        leaders_invariants(self.packet, self.rows)
        for rid, role in self.roles.items():
            for holder in role['holder_claims']:
                if holder in EARLIER_HOLDERS[rid]:
                    continue
                self.assertTrue(holder['note'].startswith('Observed on '), holder['name'])
                self.assertTrue(holder['uncertainty'].startswith('No start or end is stated'), holder['name'])
                for cid in holder['claim_ids']:
                    self.assertRegex(self.claims[cid]['uncertainty'], r'^(Dates the holder observation|Cited, with)', cid)
        # Row holder names are normalised holders of each office; the printed forms stay in the claim text.
        named = {v for v in ROW_HOLDERS.values()} - {None}
        self.assertEqual(named, set(SURNAMES))
        for cid, name in ROW_HOLDERS.items():
            if name:
                self.assertIn(SURNAMES[name], self.claims[cid]['text'], cid)
        # The holders are dated only by party wording: nothing about designation, appointment or Diet office is cited.
        for cid in HOLDER_CLAIM_SET:
            self.assertNotRegex(self.claims[cid]['text'], r'内閣総理大臣に(任命|指名)|首班指名|組閣|大臣に任命|議員に当選', cid)
        # 不破哲三 holds both titles at different times; his two series never share a claim or a day.
        fuwa = {rid: [h['attested_on'] for h in self.roles[rid]['holder_claims'] if h['name'] == '不破哲三'] for rid in (EXEC, CENTRAL)}
        self.assertEqual(fuwa, {EXEC: ['1994-07-19', '1997-09-26', '2000-11-20'], CENTRAL: ['2000-11-24', '2004-01-17', '2006-01-12']})

    def test_claims_that_never_feed_a_holder(self):
        claims, rows = self.claims, self.rows
        for cid in NEVER_HOLDER:
            self.assertFalse(any(cid in h['claim_ids'] for r in self.roles.values() for h in r['holder_claims']), cid)
        for cid in GROUPED['election']:
            self.assertRegex(claims[cid]['uncertainty'], r'(?i)never a holder', cid)
        for cid in GROUPED['departure']:
            self.assertRegex(claims[cid]['uncertainty'], r'(?i)never an until', cid)
            self.assertEqual(rows[cid]['holder_name'], '不破哲三', cid)
        for cid in GROUPED['procedure']:
            self.assertRegex(claims[cid]['uncertainty'], r'(?i)procedure only', cid)
            self.assertIsNone(rows[cid]['holder_name'], cid)
        for cid in GROUPED['other_office'] + GROUPED['interim']:
            self.assertRegex(claims[cid]['uncertainty'], r'(?i)claims only', cid)
            self.assertIsNone(rows[cid]['holder_name'], cid)
        for cid in GROUPED['organization']:
            self.assertRegex(claims[cid]['uncertainty'], r'(?i)organization', cid)
            self.assertIsNone(rows[cid]['holder_name'], cid)
        for cid in UNDATED:
            self.assertNotIn('attested_on', claims[cid], cid)
            self.assertRegex(claims[cid]['uncertainty'], r'(?i)no structured date', cid)
        for cid in GROUPED['retrospective']:
            self.assertRegex(claims[cid]['uncertainty'], r'(?i)never dates a holder', cid)
        # On each transition day these events stay separate claims.
        for day, expected in TRANSITIONS.items():
            kinds = sorted({rows[cid]['event_kind'] for cid, (d, _k, _o) in EVENTS.items() if d == day})
            self.assertEqual(kinds, expected, day)
        # The earlier 国民民主党 (May 2018 to September 2020) and the acting representative are claims only.
        for sid in OLD_PARTY_SOURCES:
            for c in self.sources[sid]['claims']:
                self.assertIsNone(rows[c['id']]['holder_name'], c['id'])
        self.assertIn('共同代表', claims['jp_dpfp_2018_co_representatives_approved_20180507']['text'])
        self.assertIn('代表代行', claims['jp_dpfp_furukawa_acting_recalled_20250304']['text'])
        self.assertIn('役職停止三カ月間', claims['jp_dpfp_tamaki_suspended_from_posts_20241204']['text'])

    def test_scope_and_coverage_notes(self):
        for rid, role in self.roles.items():
            self.assertTrue(role['scope_note'].startswith(EARLIER_SCOPE[rid] + ' CLAUDE-C01-42 extends the role'), rid)
        for phrase in ('never feed a holder', 'procedure only, never a date', "outgoing chair's last day",
                       'never read the prime-ministership (jp_pm)'):
            self.assertIn(phrase, self.roles[EXEC]['scope_note'], phrase)
        self.assertIn('Never fold this office into 幹部会委員長', self.roles[CENTRAL]['scope_note'])
        for phrase in ('earlier organization with the same name', 'claims only, never a holder, a lifecycle or a mapping',
                       '代表代行', 'never an until'):
            self.assertIn(phrase, self.roles[DPFP]['scope_note'], phrase)
        for org_id, (_s, _c, count) in ORG_EARLIER.items():
            unresolved = self.orgs[org_id]['coverage']['unresolved']
            self.assertEqual(len(unresolved), count + 1, org_id)
            self.assertTrue(unresolved[-1].startswith(ORG_UNRESOLVED_LEAD[org_id]), org_id)
        packet_unresolved = self.packet['coverage']['unresolved']
        self.assertEqual(sum('CLAUDE-C01-42' in u for u in packet_unresolved), 1)
        self.assertTrue(packet_unresolved[-1].startswith('JCP chairs and DPFP representatives 1990-2026 (CLAUDE-C01-42)'))
        self.assertTrue(packet_unresolved[-2].startswith('Komeito representatives 1990-2026 (CLAUDE-C01-31)'))

    def test_extracts_match_packet_claims_and_record_original_responses(self):
        for sid in NEW_SOURCES:
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual(extract['format'], 'spheres-c01-derived-factual-table/v1')
            self.assertEqual((extract['source_id'], extract['source_url']), (sid, source['url']))
            for key in ('scope_note', 'access_method', 'published_date', 'rights_note', 'accessed_date'):
                self.assertEqual(extract[key], source[key], (sid, key))
            self.assertEqual(source['accessed_date'], ACCESSED, sid)
            self.assertIsNone(source['published_date'], sid)
            self.assertIs(extract['source_response_checked_in'], False)
            self.assertEqual((extract['source_response_bytes'], extract['source_response_sha256']), RESPONSES[sid])
            self.assertEqual(extract['source_response_content_encoding'], 'identity')
            self.assertEqual(extract['source_response_sha1_base32'], SHA1[sid])
            self.assertIn('no Accept-Encoding request header and no automatic decoding', extract['fetch_recipe'])
            self.assertIn(source['url'], extract['fetch_recipe'])
            for phrase in ('not checked into this repository', 'derived factual extract', 'same byte count and SHA-256',
                           'identity-encoded body', 'Downloaded again by this packet', '30 minutes or more apart',
                           'Raw Internet Archive capture'):
                self.assertIn(phrase, extract['provenance_note'], (sid, phrase))
            self.assertIn('No open license, portrait permission or likeness approval', extract['rights_note'])
            self.assertEqual(extract['visual_review']['pdf_pages_one_based'], [])
            self.assertEqual(source['source_type'], SOURCE_TYPES[sid], sid)
            self.assertTrue(source['source_type'].startswith('primary_') and source['scope_note'])
            self.assertNotIn('retrospective', source['source_type'])
            snapshot = source['snapshot']
            self.assertTrue(snapshot['path'].startswith('docs/campaign-certification/C01/research/sources/japan-'))
            self.assertTrue(snapshot['path'].endswith('-facts.json'))
            data = (research.ROOT / snapshot['path']).read_bytes()
            self.assertEqual((len(data), hashlib.sha256(data).hexdigest()), (snapshot['bytes'], snapshot['sha256']))
            self.assertNotEqual(snapshot['sha256'], extract['source_response_sha256'])
            self.assertTrue(data.endswith(b'}\n') and b'\r' not in data)
            self.assertEqual(data.decode('utf-8'), json.dumps(extract, indent=2, ensure_ascii=False) + '\n')
            self.assertEqual([r['claim_id'] for r in extract['rows']], [c['id'] for c in source['claims']])
            for row, claim in zip(extract['rows'], source['claims']):
                self.assertEqual((row['text'], row['locator'], row['attested_on']),
                                 (claim['text'], claim['locator'], claim.get('attested_on')))
                self.assertEqual(row['observation_id'], ROLE_ORG[row['role_id']])
                self.assertNotIn('name', row)
                self.assertEqual(list(row), ['claim_id', 'observation_id', 'review_observation', 'role_id', 'holder_name',
                                             'role_title', 'event_kind', 'attested_on', 'text', 'locator'])
                self.assertEqual(list(claim), ['id', 'text'] + (['attested_on'] if 'attested_on' in claim else []) +
                                 ['locator', 'uncertainty'])
            # One source never mixes the two organizations.
            self.assertEqual(len({ROLE_ORG[r['role_id']] for r in extract['rows']}), 1, sid)
        self.assertEqual(len({self.sources[s]['snapshot']['path'] for s in NEW_SOURCES}), len(NEW_SOURCES))
        earlier_paths = {s['snapshot']['path'] for s in self.packet['sources'][:EARLIER_SOURCE_COUNT] if 'snapshot' in s}
        self.assertFalse(earlier_paths & {self.sources[s]['snapshot']['path'] for s in NEW_SOURCES})
        self.assertEqual({cid: (row['attested_on'], row['event_kind'], row['review_observation'])
                          for cid, row in self.rows.items()}, EVENTS)
        self.assertEqual(set(ARCHIVED), set(NEW_SOURCES))
        for sid, stamp in ARCHIVED.items():
            source, extract = self.sources[sid], self.extracts[sid]
            url = urlsplit(source['url'])
            self.assertEqual(url.hostname, 'web.archive.org')
            self.assertTrue(url.path.startswith(f'/web/{stamp}id_/http'), sid)
            self.assertLess(stamp, '20260907')
            self.assertEqual(extract['original_url'], source['original_url'])
            self.assertEqual(source['url'].split('id_/', 1)[1], source['original_url'])
            self.assertNotIn(':80', source['original_url'])
            host = urlsplit(source['original_url']).hostname
            self.assertEqual(host, ORIGINAL_HOSTS[sid], sid)
            self.assertEqual(extract['archive_capture_utc'].replace('-', '').replace(':', '').replace('T', '').rstrip('Z'), stamp)
            self.assertRegex(extract['source_character_encoding'], r'^(UTF-8|Shift_JIS|EUC-JP|ISO-2022-JP)')

    def test_response_identities_are_reproducible_urls(self):
        for sid in NEW_SOURCES:
            url = self.sources[sid]['url']
            for pattern in PER_REQUEST_PATTERNS + LEAD_URL_MARKERS:
                self.assertNotIn(pattern, url, url)
            parts = urlsplit(url)
            self.assertEqual(parts.scheme, 'https', url)
            self.assertRegex(parts.path, r'^/web/\d{14}id_/https?://', url)
        seen = {}
        for source in self.packet['sources']:
            seen.setdefault(source['url'], []).append(source['id'])
        for sid in NEW_SOURCES:
            self.assertEqual(seen[self.sources[sid]['url']], [sid], sid)
        # The original intake's live pages stay as they were; this packet records no live party page.
        self.assertEqual(self.sources['jp_jcp_chairs_2024']['url'], 'https://www.jcp.or.jp/akahata/aik23/2024-01-19/2024011903_01_0.html')
        self.assertEqual(self.sources['jp_dpfp_tamaki_elected_2026']['url'], 'https://new-kokumin.jp/news/business/20260906_2')

    def test_secondary_leads_stay_out_of_the_packet(self):
        leads, added = self.section('Leads not imported'), self.section('Sources added')
        for marker in LEAD_REPORT_MARKERS:
            self.assertIn(marker, leads, marker)
            self.assertNotIn(marker, added, marker)
        lowered = self.raw.lower()
        for marker in ('wikipedia', 'britannica', 'kotobank', 'nikkei', 'asahi.com', 'yomiuri', 'mainichi', 'nhk.or.jp'):
            self.assertNotIn(marker, lowered, marker)
        # Nothing after the cutoff: 不破哲三's death notice (December 2025) is a lead about a former chair, not a record here.
        self.assertNotIn('2025123101', self.raw)

    def test_packet_formatting_is_preserved(self):
        data = (research.ROOT / research.RESEARCH / 'japan.json').read_bytes()
        self.assertNotIn(b'\r', data)
        self.assertEqual(data.decode('utf-8'), json.dumps(self.packet, indent=2, ensure_ascii=False) + '\n')

    def test_mutations_are_rejected(self):
        def mutated(change):
            packet = copy.deepcopy(self.packet)
            change(packet)
            return packet

        def role(packet, rid):
            return roles_of(packet)[rid]

        def holder(packet, rid, name, day):
            return next(h for h in role(packet, rid)['holder_claims'] if h['name'] == name and h['attested_on'] == day)

        def claim(packet, cid):
            return next(c for s in packet['sources'] for c in s['claims'] if c['id'] == cid)

        def org(packet, org_id):
            return next(o for o in packet['organizations'] if o['id'] == org_id)

        leaders_rules(self.packet, self.rows)

        def add_old_party_holder(packet):
            role(packet, DPFP)['holder_claims'].insert(0, {
                'name': '玉木雄一郎', 'attested_on': '2018-09-04', 'from': None, 'until': None,
                'sources': ['jp_dpfp_2018_representative_press_conference_20180904'],
                'claim_ids': ['jp_dpfp_2018_tamaki_assumption_stated_20180904'], 'note': 'x', 'uncertainty': 'x'})
            role(packet, DPFP)['claim_ids'].append('jp_dpfp_2018_tamaki_assumption_stated_20180904')

        def move_miyamoto(packet):
            moved = holder(packet, CENTRAL, '宮本顕治', '1994-07-19')
            role(packet, CENTRAL)['holder_claims'].remove(moved)
            role(packet, EXEC)['holder_claims'].insert(0, moved)

        rule_mutations = {
            'withdrawal made an until': lambda p: holder(p, CENTRAL, '不破哲三', '2006-01-12').update(until='2006-01-14'),
            'founding convention made a start': lambda p: holder(p, DPFP, '玉木雄一郎', '2020-10-26').update({'from': '2020-09-15'}),
            'earlier 国民民主党 office made a holder': add_old_party_holder,
            'Central Committee chair moved into 幹部会委員長': move_miyamoto,
            'organization given a lifecycle': lambda p: org(p, JCP_ORG)['lifecycle'].update({'status': 'continuous', 'from': '1922-07-15'}),
            'game mapping asserted': lambda p: org(p, DPFP_ORG)['represented_party_ids'].append('dpfp'),
            'party claim fed to the prime-ministership': lambda p: p['institutions'][-1]['roles'][0]['claim_ids'].append('jp_jcp_fuwa_opening_address_20001120'),
            'election day made a holder day': lambda p: holder(p, EXEC, '志位和夫', '2001-05-29').update(attested_on='2000-11-24'),
            'earlier holder dropped': lambda p: role(p, EXEC)['holder_claims'].pop(),
            'earlier holder re-dated': lambda p: role(p, DPFP)['holder_claims'][-1].update(attested_on='2026-09-07'),
            'holders reordered': lambda p: role(p, EXEC)['holder_claims'].reverse(),
            'chair claim shared across titles': lambda p: holder(p, CENTRAL, '不破哲三', '2004-01-17')['claim_ids'].append('jp_jcp_shii_executive_chair_summation_20040117'),
        }
        for label, change in rule_mutations.items():
            with self.subTest(mutation=label), self.assertRaises((AssertionError, KeyError, IndexError)):
                leaders_rules(mutated(change), self.rows)
        for cid, field, value in (('jp_dpfp_furukawa_acting_recalled_20250304', 'holder_name', '古川元久'),
                                  ('jp_dpfp_furukawa_acting_recalled_20250304', 'role_title', T_DPFP),
                                  ('jp_jcp_shii_elected_executive_chair_20001124', 'event_kind', 'in_office_attestation'),
                                  ('jp_jcp_miyamoto_named_honorary_chair_19970926', 'holder_name', '宮本顕治'),
                                  ('jp_dpfp_2018_co_representatives_inaugural_greetings_20180507', 'role_title', T_DPFP),
                                  ('jp_dpfp_tamaki_suspended_from_posts_20241204', 'holder_name', '玉木雄一郎'),
                                  ('jp_jcp_central_chair_made_optional_proposed', 'attested_on', '1997-09-25'),
                                  ('jp_jcp_fuwa_opening_address_20001120', 'role_id', CENTRAL)):
            rows = copy.deepcopy(self.rows)
            rows[cid][field] = value
            with self.subTest(row=cid, field=field), self.assertRaises((AssertionError, KeyError)):
                leaders_rules(self.packet, rows)
        # Re-dating an event is caught by the pinned events.
        for cid, day in (('jp_jcp_fuwa_steps_down_confirmed_20060114', '2006-01-12'),
                         ('jp_dpfp_tamaki_elected_20230902', '2023-09-05'),
                         ('jp_jcp_miyamoto_absence_reported_19970923', '1997-09-26')):
            with self.subTest(redated=cid), self.assertRaises(AssertionError):
                leaders_invariants(mutated(lambda p: claim(p, cid).update(attested_on=day)), self.rows)

    def test_report_and_handoff_close_no_parent_gate(self):
        self.assertIn('ready_for_review', self.report)
        table = self.section('Outcome')
        for obs in REVIEW:
            row, = [line for line in table.splitlines() if line.startswith(f'| {obs} ')]
            self.assertRegex(row, r'\*\*(Accepted in part|Accepted|Not established):\*\*')
        for marker in ('C01', 'C06', 'S23', 'WC1', 'CP1'):
            self.assertNotRegex(self.report, rf'\b{marker}\b[^.\n]*\bis (now )?complete\b')
        notes = self.section('Integration notes')
        for text in ('60107819', '509bd289', 'claude/c01-jp-31', 'research-index.json', 'only shared file',
                     'test_japan_research_s10d.py', 'test_japan_prime_ministers_c01_12.py',
                     'test_japan_prime_ministers_c01_13.py', 'test_japan_ldp_presidents_c01_18.py',
                     'test_japan_sdp_chairs_c01_29.py', 'test_japan_komeito_representatives_c01_31.py',
                     'continue existing claims first'):
            self.assertIn(text, notes)
        identities = self.section('Response identities and stability checks')
        for sid, (size, sha) in RESPONSES.items():
            self.assertIn(f'`{sid}`', identities, sid)
            self.assertIn(sha[:12], identities, sid)
        ledger = self.report.split('\n### Date ledger', 1)[1].split('\n## ', 1)[0]
        for rid in HOLDERS:
            for name, day, _f, _u in HOLDERS[rid]:
                self.assertIn(name, ledger)
                year, month, dom = day.split('-')
                months = 'Jan Feb Mar Apr May Jun Jul Aug Sep Oct Nov Dec'.split()
                self.assertIn(f'| {int(dom)} {months[int(month) - 1]} {year} |', ledger, (name, day))
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        for text in ('ready_for_review', REPORT.name, 'claude/c01-jp-31', '60107819',
                     'test_japan_jcp_dpfp_leaders_c01_42.py'):
            self.assertIn(text, handoff)
        index = research.build()
        country = next(p for p in index['countries'] if p['nation'] == 'Japan')
        self.assertFalse(country['country_census_complete'])
        self.assertIsNone(country['unrepresented_organization_count'])
        self.assertEqual((country['institution_observations'], country['role_observations']), (8, 7))
        self.assertFalse(index['c01_complete'])


if __name__ == '__main__':
    unittest.main()
