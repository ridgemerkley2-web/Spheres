"""CLAUDE-C01-31: the chair (委員長) of 公明党 until its December 1994 division and the representative (代表) of the 公明党
formed in November 1998, 1990-2026, are one party role kept apart from state office. Each party selection, recommendation,
resignation announcement, acting or interim arrangement and organization event stays a separate dated claim; a holder is
dated by a same-day party record or the leader's own statement of the office, has a start or an end only where a source
states the day, and the 1994 division, the organizations of 1994-1998, the 1998 merger and 中道改革連合 are claims about the
organizations, never a merge or a mapping of identities."""
import copy
import hashlib
import json
import re
import unittest
from urllib.parse import urlsplit

import campaign_research as research
# CLAUDE-C01-42 (claimed while stacked on CLAUDE-C01-31, now based on integration) appends the sources of the 日本共産党 chair and 国民民主党 representative roles
# after CLAUDE-C01-31's; its exact sources and holders are pinned in its own test.
import test_japan_jcp_dpfp_leaders_c01_42 as jcp42

ORG_ID = 'jp_sangiin_pr_2025_15'
ROLE = 'jp_komeito_representative'
T_KOMEITO = '代表 / 委員長 — representative of 公明党 and, until its December 1994 division, its chair (委員長)'
PM = 'jp_pm'
LDP_ORG, LDP_ROLE = 'jp_sangiin_pr_2025_13', 'jp_ldp_party_president'
SDP_ORG, SDP_ROLE = 'jp_sangiin_pr_2025_10', 'jp_sdp_chair'
CDP_ORG = 'jp_sangiin_pr_2025_05'  # 立憲民主党, with which 中道改革連合 was formed in 2026: never mapped or merged here
# The packet's sources before this packet: the original S10d intake (7), CLAUDE-C01-12 (119), CLAUDE-C01-13 (211),
# CLAUDE-C01-18 (81) and CLAUDE-C01-29 (54).
EARLIER_SOURCE_COUNT = 472
ORG_EARLIER_CLAIMS = ['jp_pr2025_submission_15']
ORG_EARLIER_SOURCES = ['jp_tokyo_pr_2025']
ORG_EARLIER_UNRESOLVED = 3
GROUPS = ['jp_shugiin_group_20260218_011', 'jp_shugiin_group_20260218_020', 'jp_shugiin_group_20260218_030',
          'jp_shugiin_group_20260218_040', 'jp_shugiin_group_20260218_050', 'jp_shugiin_group_20260218_060',
          'jp_shugiin_group_20260218_070']
EARLIER_ROLES = ['jp_jcp_executive_committee_chair', 'jp_jcp_central_committee_chair', 'jp_dpfp_representative',
                 SDP_ROLE, LDP_ROLE, PM]
PM_HOLDER_COUNT = 30  # CLAUDE-C01-12's fourteen and CLAUDE-C01-13's sixteen, unchanged
LDP_HOLDER_COUNT = 16  # CLAUDE-C01-18's fourteen and the original two, unchanged
SDP_HOLDER_COUNT = 16  # CLAUDE-C01-29's sixteen, unchanged
ACCESSED = '2026-09-30'

# Original response identity recorded in each extract: (bytes, sha256) of the identity-encoded body, fetched as the
# extract's fetch_recipe says. Every source is a raw Internet Archive capture or an official Diet minutes API response.
RESPONSES = {
    'jp_komeito_diet_hr_budget_19930129':
        (420107, '41cf9e3e51b5c20e151ada1da4dd3f81def2dd3148d3797b6c634c0a2ea5cd3f'),
    'jp_komeito_diet_hr_political_reform_19931105':
        (4931, '176fff9e51d69222435c0bcc876b39a52f05a2514cf92015f08439cc993c247e'),
    'jp_komeito_diet_hc_budget_19940616':
        (1949, '6c2a273102625a28f61a594a1629e34c015cad89e3dfd6469edbc5ec4b217f0d'),
    'jp_komeito_komei_representative_message_retrospective':
        (2645, 'f9149626667cf018d7c7068a449d7240d5952aa8119fded3bc9d5ace39315267'),
    'jp_komeito_history_2003_retrospective':
        (27148, '1ac4e853d360316db830f0735d9bdb21af83e316ff27b69cdaf20fbfe996a0ca'),
    'jp_komeito_founding_convention_report_19981108':
        (9379, '7addfc837f4ad2e12ab6f09d1ed68c6c9958b7ef985f5732e8f03fadbe0a1334'),
    'jp_komeito_diet_hr_plenary_19981130':
        (23924, '233a528efa9541a8a46bdfde92cff1a1b92d1dac1436674e8a8defcfa3468930'),
    'jp_komeito_kanzaki_reappointed_20021103':
        (18955, 'dbede32d5c6b4906ad581802c05b3036ed25d02ce7c0fd18823135798450ef5f'),
    'jp_komeito_kanzaki_fourth_convention_address_20021103':
        (30044, '4a3f0161701b96c30aae3b7c228032c4e51a51c49563531aec6bb4915828e7f0'),
    'jp_komeito_kanzaki_fifth_convention_address_20041101':
        (23741, '89313a0227fd7b94047c55930811552d46900f5d639e5cf8294571eeb98a5035'),
    'jp_komeito_ota_sixth_convention_20061001':
        (12680, '2209ccbc6b17dcef68f81aadf00e6fd6b0b7fe7191148353baff8875f56cc8b2'),
    'jp_komeito_ota_address_sixth_convention_20061001':
        (22693, 'b320b6ded5b7108cf5b4a14ed4262e8e3e849eb15ecb90db986b194c91dd46ab'),
    'jp_komeito_diet_hr_plenary_20061003':
        (25649, 'd045d0b08200e6b4720cd746390b7ca8f163f8b017026a46f116757670c3d7d4'),
    'jp_komeito_yamaguchi_selection_greeting_20090909':
        (36772, '5eff44f27ff8aec3d93adfcc31bcc0619f70c479c09de18ac5e28ede35d42c46'),
    'jp_komeito_diet_hc_plenary_20091030':
        (29900, 'cc5895a5f373b637bacaa401d8077c3c861dd128affeb81ea6a60f7571c4024b'),
    'jp_komeito_yamaguchi_ninth_convention_20120923':
        (29146, '2f91cc88555bc5b09ce116ed62f3514099fe853873e122f43467b5d9f7c053eb'),
    'jp_komeito_yamaguchi_tenth_convention_20140922':
        (37043, 'fa384f17f11787bc6302eb73e1fbb01cfa9691644abef566fee819e1a7869d76'),
    'jp_komeito_yamaguchi_eleventh_convention_20160918':
        (36226, '06971791723f9473fa60cba011a39b798a5537a68bbcaf8e4b32816c10cdc2d4'),
    'jp_komeito_yamaguchi_twelfth_convention_20181001':
        (34760, '2c495a62266f4e0877406eac5646982e51ca35fc526eaa4bf76ad53f54dcda47'),
    'jp_komeito_yamaguchi_thirteenth_convention_20200928':
        (45098, 'dbac19c58fa0861b8aa562543958db723db5954c28c273975abd253df73a7813'),
    'jp_komeito_yamaguchi_fourteenth_convention_20220926':
        (43206, '474ece40146d5c18c5b03be21127485401ff2d72792a0060732032d2caca7007'),
    'jp_komeito_ishii_new_representative_interview_20240930':
        (54284, 'ac2f2c18911aa7105637ac9b3b3e4d49ddb9d39a40a67ad84e5466648ed9783d'),
    'jp_komeito_ishii_resignation_intent_20241101':
        (40398, '5451604c7a62766525f8eedb47aa7ac833d7092eaaa08a553be35b5d33db3537'),
    'jp_komeito_saito_recommended_20241108':
        (40718, 'b4c664da6eab760154152795c654343953096b0bb9a47924aabd2e2b1ce5dedd'),
    'jp_komeito_saito_extraordinary_convention_20241110':
        (45124, '952397cdc4f4d72e367d18125af1dec4402b8c8d7aba9f5f0ab4d746fde61838'),
    'jp_komeito_diet_hr_plenary_20241203':
        (25257, '2ef5f85e77c8bf3078c905ad0055abdc3bcecd8f505120261f627df2cacb995b'),
    'jp_komeito_takeya_interim_20260123':
        (47569, '958980b45167ec59a8c92aebfd4c2cd785514443e5aae56b7fa24dfbbc5b182b'),
    'jp_komeito_chudo_founding_convention_20260125':
        (50500, 'c20324907719dcf9d55ede20cc1bd617f6230925bc347d051665a9cc1f34e83e'),
    'jp_komeito_takeya_recommended_20260312':
        (42116, '2c1670ece3a9e46c246be9f1c6b8972ce89ca33fb3a3a6cc6a51856d03ef0f56'),
    'jp_komeito_takeya_address_20260315':
        (56383, '6b8c093e9dfff1bceb87384cce955633c5977caf1592366936495766d8b5f025'),
    'jp_komeito_takeya_extraordinary_convention_20260315':
        (48871, '8e4609a0441bd34a8c0696cb5924e463b1ff457ad77633e38e339aad478e3cf8'),
}
# Base32 SHA-1 of each recorded response (for comparison with Internet Archive CDX digests).
SHA1 = {
    'jp_komeito_diet_hr_budget_19930129': 'HK7PX575YNM5ZPOFUE7S7GBEAF2D53F5',
    'jp_komeito_diet_hr_political_reform_19931105': 'T6YUN64I3WTY25V26KNI2ZS6VXGBDJRB',
    'jp_komeito_diet_hc_budget_19940616': 'HNOQB54Q75RILGY7H5X56RRNOLIHXD5H',
    'jp_komeito_komei_representative_message_retrospective': 'UF5C75FO4STXHS67JTSVYBD2PJ2JYKEB',
    'jp_komeito_history_2003_retrospective': 'CKAUZB6CBQQM4NWA7GTLKXUCBLJYTACN',
    'jp_komeito_founding_convention_report_19981108': 'BQ3OWHK2PRJFGTSIBJEFT4UTUTCMVZLG',
    'jp_komeito_diet_hr_plenary_19981130': 'FSO6XATQT32BFVB6MZ7MH2W7ABNT24IZ',
    'jp_komeito_kanzaki_reappointed_20021103': 'VCEW3IZTZAG2FJ52EBOSKYPBQNCJSPQL',
    'jp_komeito_kanzaki_fourth_convention_address_20021103': 'NHZMTXYFJBWP5ISR245R3POZZ4CGB3Y2',
    'jp_komeito_kanzaki_fifth_convention_address_20041101': 'FSYA7GI6KG3FBY3XOQYIQLIIYAHNFYXQ',
    'jp_komeito_ota_sixth_convention_20061001': 'B5YDDC6CX76LGNFCZZ53KUBNL45NRBFE',
    'jp_komeito_ota_address_sixth_convention_20061001': 'HA75OUUZVNAP25YNJDVIRWZCK5QSGJVN',
    'jp_komeito_diet_hr_plenary_20061003': '7H3SVV7CI5YMYBLSBAVZOON4KF2IAEQK',
    'jp_komeito_yamaguchi_selection_greeting_20090909': 'NLRSOOIW5ZMTQE4J4W44WRED3363WNTF',
    'jp_komeito_diet_hc_plenary_20091030': 'QAENWEA5QLWHGHG26YMILIZ5QO3AT64T',
    'jp_komeito_yamaguchi_ninth_convention_20120923': 'CCWTJKDFYGHV7YA7RRNOWTT6YQ4LNMBQ',
    'jp_komeito_yamaguchi_tenth_convention_20140922': 'OMJUFO256ZIC46CNQJD3FZEZDS7E5T5N',
    'jp_komeito_yamaguchi_eleventh_convention_20160918': 'MNK3DUA7OQROSUNF6T5STXGXVVTY5HHW',
    'jp_komeito_yamaguchi_twelfth_convention_20181001': 'FVYFCHXXL777WTEK4EK445Q7UWOZZOP2',
    'jp_komeito_yamaguchi_thirteenth_convention_20200928': 'BTKUZMWQS6RPYA54EUTMF56NWE3FCGCD',
    'jp_komeito_yamaguchi_fourteenth_convention_20220926': '375BYBHBDIRTLBQ66X4ZZUACNYGWTOSF',
    'jp_komeito_ishii_new_representative_interview_20240930': 'Q7CXHPSVZHJ5IM2VMA7F2N6AX7ZAQTW2',
    'jp_komeito_ishii_resignation_intent_20241101': 'SGAXJAREEN3Z7DIAE72PQNFZSI3RWTTY',
    'jp_komeito_saito_recommended_20241108': 'TA2G6OLBBN27ZJ47MPMHNDSXRXDM6KPE',
    'jp_komeito_saito_extraordinary_convention_20241110': 'YI3VPLMUEJQU7JAWPSLFPAHSHCQ77KOY',
    'jp_komeito_diet_hr_plenary_20241203': '75JFQSPRVVQZ2S7KKOVIFFC6KG4PBBR5',
    'jp_komeito_takeya_interim_20260123': 'GKEJQ5DKJ4NYDRHEWGP4WUHGRMMTYZQG',
    'jp_komeito_chudo_founding_convention_20260125': 'J322GRFHYXTQ5L45C5PDKKR3Y3NHCJI2',
    'jp_komeito_takeya_recommended_20260312': '2GXCNOUUTZYB2B2DZCWYRKV2Z7VC235Z',
    'jp_komeito_takeya_address_20260315': 'VYAJUGEELIEIAZRI5VFJCIPOT6CMNPFB',
    'jp_komeito_takeya_extraordinary_convention_20260315': 'PU24OVD5HANQKZSVAXQ6TC546TREWQ2P',
}
# Raw Internet Archive captures (id_ form), all made before the cutoff: source id -> capture timestamp.
ARCHIVED = {
    'jp_komeito_komei_representative_message_retrospective': '19980202152557',
    'jp_komeito_history_2003_retrospective': '20030206013142',
    'jp_komeito_founding_convention_report_19981108': '19991004170254',
    'jp_komeito_kanzaki_reappointed_20021103': '20021123180520',
    'jp_komeito_kanzaki_fourth_convention_address_20021103': '20021123181318',
    'jp_komeito_kanzaki_fifth_convention_address_20041101': '20070624115054',
    'jp_komeito_ota_sixth_convention_20061001': '20070623012014',
    'jp_komeito_ota_address_sixth_convention_20061001': '20070623011929',
    'jp_komeito_yamaguchi_selection_greeting_20090909': '20110711232815',
    'jp_komeito_yamaguchi_ninth_convention_20120923': '20130107011050',
    'jp_komeito_yamaguchi_tenth_convention_20140922': '20140924161639',
    'jp_komeito_yamaguchi_eleventh_convention_20160918': '20190722002155',
    'jp_komeito_yamaguchi_twelfth_convention_20181001': '20190722004140',
    'jp_komeito_yamaguchi_thirteenth_convention_20200928': '20201027010306',
    'jp_komeito_yamaguchi_fourteenth_convention_20220926': '20220928122912',
    'jp_komeito_ishii_new_representative_interview_20240930': '20240930045917',
    'jp_komeito_ishii_resignation_intent_20241101': '20241106194128',
    'jp_komeito_saito_recommended_20241108': '20241108042912',
    'jp_komeito_saito_extraordinary_convention_20241110': '20241111013606',
    'jp_komeito_takeya_interim_20260123': '20260128043946',
    'jp_komeito_chudo_founding_convention_20260125': '20260129004916',
    'jp_komeito_takeya_recommended_20260312': '20260312230500',
    'jp_komeito_takeya_address_20260315': '20260317063320',
    'jp_komeito_takeya_extraordinary_convention_20260315': '20260318121519',
}
# Official Diet minutes API responses (whole meeting records or single speeches).
DIET = [
    'jp_komeito_diet_hr_budget_19930129',
    'jp_komeito_diet_hr_political_reform_19931105',
    'jp_komeito_diet_hc_budget_19940616',
    'jp_komeito_diet_hr_plenary_19981130',
    'jp_komeito_diet_hr_plenary_20061003',
    'jp_komeito_diet_hc_plenary_20091030',
    'jp_komeito_diet_hr_plenary_20241203',
]
SOURCE_TYPES = {
    'jp_komeito_diet_hr_budget_19930129': 'primary_diet_minutes_api_json',
    'jp_komeito_diet_hr_political_reform_19931105': 'primary_diet_minutes_api_json',
    'jp_komeito_diet_hc_budget_19940616': 'primary_diet_minutes_api_json',
    'jp_komeito_komei_representative_message_retrospective': 'primary_party_web_page_retrospective',
    'jp_komeito_history_2003_retrospective': 'primary_party_history_retrospective',
    'jp_komeito_founding_convention_report_19981108': 'primary_party_publication_archived',
    'jp_komeito_diet_hr_plenary_19981130': 'primary_diet_minutes_api_json',
    'jp_komeito_kanzaki_reappointed_20021103': 'primary_party_publication_archived',
    'jp_komeito_kanzaki_fourth_convention_address_20021103': 'primary_party_publication_archived',
    'jp_komeito_kanzaki_fifth_convention_address_20041101': 'primary_party_publication_archived',
    'jp_komeito_ota_sixth_convention_20061001': 'primary_party_publication_archived',
    'jp_komeito_ota_address_sixth_convention_20061001': 'primary_party_publication_archived',
    'jp_komeito_diet_hr_plenary_20061003': 'primary_diet_minutes_api_json',
    'jp_komeito_yamaguchi_selection_greeting_20090909': 'primary_party_publication_archived',
    'jp_komeito_diet_hc_plenary_20091030': 'primary_diet_minutes_api_json',
    'jp_komeito_yamaguchi_ninth_convention_20120923': 'primary_party_publication_archived',
    'jp_komeito_yamaguchi_tenth_convention_20140922': 'primary_party_publication_archived',
    'jp_komeito_yamaguchi_eleventh_convention_20160918': 'primary_party_publication_archived',
    'jp_komeito_yamaguchi_twelfth_convention_20181001': 'primary_party_publication_archived',
    'jp_komeito_yamaguchi_thirteenth_convention_20200928': 'primary_party_publication_archived',
    'jp_komeito_yamaguchi_fourteenth_convention_20220926': 'primary_party_publication_archived',
    'jp_komeito_ishii_new_representative_interview_20240930': 'primary_party_publication_archived',
    'jp_komeito_ishii_resignation_intent_20241101': 'primary_party_publication_archived',
    'jp_komeito_saito_recommended_20241108': 'primary_party_publication_archived',
    'jp_komeito_saito_extraordinary_convention_20241110': 'primary_party_publication_archived',
    'jp_komeito_diet_hr_plenary_20241203': 'primary_diet_minutes_api_json',
    'jp_komeito_takeya_interim_20260123': 'primary_party_publication_archived',
    'jp_komeito_chudo_founding_convention_20260125': 'primary_party_publication_archived',
    'jp_komeito_takeya_recommended_20260312': 'primary_party_publication_archived',
    'jp_komeito_takeya_address_20260315': 'primary_party_publication_archived',
    'jp_komeito_takeya_extraordinary_convention_20260315': 'primary_party_publication_archived',
}
# Every new claim's (attested_on, event_kind, review observation), exactly: distinct dated events are never re-dated,
# relabelled or moved to another observation. Retrospective records and undated references carry no structured date.
EVENTS = {
    'jp_komeito_ishida_named_chair_by_secretary_general_19930129': ('1993-01-29', 'in_office_attestation', 'KOMEITO-01'),
    'jp_komeito_ishida_assumption_recalled_may_1989': (None, 'assumption_recalled_by_holder', 'KOMEITO-01'),
    'jp_komeito_ishida_still_chair_19940616': ('1994-06-16', 'in_office_continuation_attestation', 'KOMEITO-01'),
    'jp_komeito_acting_chair_in_place_19940616': ('1994-06-16', 'acting_chair_in_place', 'KOMEITO-01'),
    'jp_komeito_komei_formed_retrospective': (None, 'retrospective_organization_record', 'KOMEITO-02'),
    'jp_komeito_komei_representative_since_19941205_retrospective': (None, 'other_office_retrospective', 'KOMEITO-02'),
    'jp_komeito_history_ishida_chair_in_cabinet_retrospective': (None, 'retrospective_office_reference', 'KOMEITO-01'),
    'jp_komeito_history_nfp_founded_retrospective': (None, 'retrospective_organization_record', 'KOMEITO-02'),
    'jp_komeito_history_division_retrospective': (None, 'retrospective_organization_record', 'KOMEITO-02'),
    'jp_komeito_history_nfp_dissolved_retrospective': (None, 'retrospective_organization_record', 'KOMEITO-02'),
    'jp_komeito_history_reimei_heiwa_formed_retrospective': (None, 'retrospective_organization_record', 'KOMEITO-02'),
    'jp_komeito_history_reformation_retrospective': (None, 'retrospective_organization_record', 'KOMEITO-03'),
    'jp_komeito_new_name_decided_19981024': ('1998-10-24', 'organization_renaming_decided', 'KOMEITO-03'),
    'jp_komeito_merger_form_reported': (None, 'organization_merger', 'KOMEITO-03'),
    'jp_komeito_kanzaki_convention_confidence': (None, 'convention_selection', 'KOMEITO-03'),
    'jp_komeito_kanzaki_in_office_founding_report_19981108': ('1998-11-08', 'in_office_attestation', 'KOMEITO-03'),
    'jp_komeito_merger_convention_recalled_19981107': ('1998-11-07', 'organization_merger', 'KOMEITO-03'),
    'jp_komeito_kanzaki_sole_candidate_reelected_2002': (None, 'convention_selection', 'KOMEITO-04'),
    'jp_komeito_kanzaki_in_office_reappointed_20021103': ('2002-11-03', 'in_office_attestation', 'KOMEITO-04'),
    'jp_komeito_kanzaki_self_stated_continuing_20021103': ('2002-11-03', 'in_office_attestation', 'KOMEITO-04'),
    'jp_komeito_kanzaki_reelected_20041031': ('2004-10-31', 'convention_selection', 'KOMEITO-04'),
    'jp_komeito_kanzaki_self_stated_again_20041031': ('2004-10-31', 'in_office_attestation', 'KOMEITO-04'),
    'jp_komeito_ota_selected_20060930': ('2006-09-30', 'convention_selection', 'KOMEITO-05'),
    'jp_komeito_ota_in_office_convention_20060930': ('2006-09-30', 'in_office_attestation', 'KOMEITO-05'),
    'jp_komeito_ota_assumption_stated_20060930': ('2006-09-30', 'assumption_stated', 'KOMEITO-05'),
    'jp_komeito_kanzaki_former_four_terms_20060930': ('2006-09-30', 'predecessor_referred_to_as_former', 'KOMEITO-04'),
    'jp_komeito_ota_self_stated_representative_20061003': ('2006-10-03', 'in_office_continuation_attestation', 'KOMEITO-05'),
    'jp_komeito_yamaguchi_selected_20090908': ('2009-09-08', 'national_representatives_meeting_selection', 'KOMEITO-06'),
    'jp_komeito_yamaguchi_in_office_greeting_20090908': ('2009-09-08', 'in_office_attestation', 'KOMEITO-06'),
    'jp_komeito_ota_former_20090908': ('2009-09-08', 'predecessor_referred_to_as_former', 'KOMEITO-05'),
    'jp_komeito_yamaguchi_selection_recalled_20090908': ('2009-09-08', 'election_recalled_by_holder', 'KOMEITO-06'),
    'jp_komeito_yamaguchi_in_office_diet_20091030': ('2009-10-30', 'in_office_continuation_attestation', 'KOMEITO-06'),
    'jp_komeito_yamaguchi_reelected_20120922': ('2012-09-22', 'convention_selection', 'KOMEITO-06'),
    'jp_komeito_yamaguchi_in_office_convention_20120922': ('2012-09-22', 'in_office_attestation', 'KOMEITO-06'),
    'jp_komeito_yamaguchi_reelected_20140921': ('2014-09-21', 'convention_selection', 'KOMEITO-07'),
    'jp_komeito_yamaguchi_in_office_convention_20140921': ('2014-09-21', 'in_office_attestation', 'KOMEITO-07'),
    'jp_komeito_yamaguchi_reelected_20160917': ('2016-09-17', 'convention_selection', 'KOMEITO-07'),
    'jp_komeito_yamaguchi_in_office_convention_20160917': ('2016-09-17', 'in_office_attestation', 'KOMEITO-07'),
    'jp_komeito_yamaguchi_reelected_20180930': ('2018-09-30', 'convention_selection', 'KOMEITO-07'),
    'jp_komeito_yamaguchi_in_office_convention_20180930': ('2018-09-30', 'in_office_attestation', 'KOMEITO-07'),
    'jp_komeito_yamaguchi_reelected_20200927': ('2020-09-27', 'convention_selection', 'KOMEITO-07'),
    'jp_komeito_yamaguchi_in_office_convention_20200927': ('2020-09-27', 'in_office_attestation', 'KOMEITO-07'),
    'jp_komeito_yamaguchi_reelected_20220925': ('2022-09-25', 'convention_selection', 'KOMEITO-07'),
    'jp_komeito_yamaguchi_in_office_convention_20220925': ('2022-09-25', 'in_office_attestation', 'KOMEITO-07'),
    'jp_komeito_ishii_in_office_convention_20240928': ('2024-09-28', 'in_office_attestation', 'KOMEITO-08'),
    'jp_komeito_ishii_appointment_recalled': (None, 'election_recalled_by_holder', 'KOMEITO-08'),
    'jp_komeito_yamaguchi_former_eight_terms_recalled': (None, 'predecessor_referred_to_as_former', 'KOMEITO-07'),
    'jp_komeito_ishii_resignation_intent_20241031': ('2024-10-31', 'resignation_intent_announced', 'KOMEITO-08'),
    'jp_komeito_saito_recommended_20241107': ('2024-11-07', 'candidate_recommended', 'KOMEITO-09'),
    'jp_komeito_saito_convention_selection_20241109': ('2024-11-09', 'convention_selection', 'KOMEITO-09'),
    'jp_komeito_saito_in_office_convention_20241109': ('2024-11-09', 'in_office_attestation', 'KOMEITO-09'),
    'jp_komeito_ishii_former_20241110': ('2024-11-10', 'predecessor_referred_to_as_former', 'KOMEITO-08'),
    'jp_komeito_saito_in_office_diet_20241203': ('2024-12-03', 'in_office_continuation_attestation', 'KOMEITO-09'),
    'jp_komeito_takeya_interim_approved_20260122': ('2026-01-22', 'interim_representative_designated', 'KOMEITO-10'),
    'jp_komeito_interim_styled_representative_decided_20260122': ('2026-01-22', 'interim_title_decided', 'KOMEITO-10'),
    'jp_komeito_saito_former_20260122': ('2026-01-22', 'predecessor_referred_to_as_former', 'KOMEITO-09'),
    'jp_komeito_chudo_founded_20260122': ('2026-01-22', 'organization_formed', 'KOMEITO-09'),
    'jp_komeito_saito_chudo_co_representative_20260122': ('2026-01-22', 'other_office_attestation', 'KOMEITO-09'),
    'jp_komeito_takeya_styled_representative_20260122': ('2026-01-22', 'interim_representative_styled', 'KOMEITO-10'),
    'jp_komeito_takeya_recommended_20260311': ('2026-03-11', 'candidate_recommended', 'KOMEITO-10'),
    'jp_komeito_takeya_assumption_stated_20260314': ('2026-03-14', 'in_office_attestation', 'KOMEITO-10'),
    'jp_komeito_takeya_convention_selection_20260314': ('2026-03-14', 'convention_selection', 'KOMEITO-10'),
    'jp_komeito_takeya_in_office_convention_20260314': ('2026-03-14', 'in_office_attestation', 'KOMEITO-10'),
    'jp_komeito_vacancy_recalled_january_2026': (None, 'office_vacancy_recalled', 'KOMEITO-09'),
}
# The holder name each extract row carries; None where the source names no holder of this office.
ROW_HOLDERS = {
    'jp_komeito_ishida_named_chair_by_secretary_general_19930129': '石田幸四郎',
    'jp_komeito_ishida_assumption_recalled_may_1989': '石田幸四郎',
    'jp_komeito_ishida_still_chair_19940616': '石田幸四郎',
    'jp_komeito_acting_chair_in_place_19940616': None,
    'jp_komeito_komei_formed_retrospective': None,
    'jp_komeito_komei_representative_since_19941205_retrospective': None,
    'jp_komeito_history_ishida_chair_in_cabinet_retrospective': '石田幸四郎',
    'jp_komeito_history_nfp_founded_retrospective': None,
    'jp_komeito_history_division_retrospective': None,
    'jp_komeito_history_nfp_dissolved_retrospective': None,
    'jp_komeito_history_reimei_heiwa_formed_retrospective': None,
    'jp_komeito_history_reformation_retrospective': None,
    'jp_komeito_new_name_decided_19981024': None,
    'jp_komeito_merger_form_reported': None,
    'jp_komeito_kanzaki_convention_confidence': '神崎武法',
    'jp_komeito_kanzaki_in_office_founding_report_19981108': '神崎武法',
    'jp_komeito_merger_convention_recalled_19981107': None,
    'jp_komeito_kanzaki_sole_candidate_reelected_2002': '神崎武法',
    'jp_komeito_kanzaki_in_office_reappointed_20021103': '神崎武法',
    'jp_komeito_kanzaki_self_stated_continuing_20021103': '神崎武法',
    'jp_komeito_kanzaki_reelected_20041031': '神崎武法',
    'jp_komeito_kanzaki_self_stated_again_20041031': '神崎武法',
    'jp_komeito_ota_selected_20060930': '太田昭宏',
    'jp_komeito_ota_in_office_convention_20060930': '太田昭宏',
    'jp_komeito_ota_assumption_stated_20060930': '太田昭宏',
    'jp_komeito_kanzaki_former_four_terms_20060930': '神崎武法',
    'jp_komeito_ota_self_stated_representative_20061003': '太田昭宏',
    'jp_komeito_yamaguchi_selected_20090908': '山口那津男',
    'jp_komeito_yamaguchi_in_office_greeting_20090908': '山口那津男',
    'jp_komeito_ota_former_20090908': '太田昭宏',
    'jp_komeito_yamaguchi_selection_recalled_20090908': '山口那津男',
    'jp_komeito_yamaguchi_in_office_diet_20091030': '山口那津男',
    'jp_komeito_yamaguchi_reelected_20120922': '山口那津男',
    'jp_komeito_yamaguchi_in_office_convention_20120922': '山口那津男',
    'jp_komeito_yamaguchi_reelected_20140921': '山口那津男',
    'jp_komeito_yamaguchi_in_office_convention_20140921': '山口那津男',
    'jp_komeito_yamaguchi_reelected_20160917': '山口那津男',
    'jp_komeito_yamaguchi_in_office_convention_20160917': '山口那津男',
    'jp_komeito_yamaguchi_reelected_20180930': '山口那津男',
    'jp_komeito_yamaguchi_in_office_convention_20180930': '山口那津男',
    'jp_komeito_yamaguchi_reelected_20200927': '山口那津男',
    'jp_komeito_yamaguchi_in_office_convention_20200927': '山口那津男',
    'jp_komeito_yamaguchi_reelected_20220925': '山口那津男',
    'jp_komeito_yamaguchi_in_office_convention_20220925': '山口那津男',
    'jp_komeito_ishii_in_office_convention_20240928': '石井啓一',
    'jp_komeito_ishii_appointment_recalled': '石井啓一',
    'jp_komeito_yamaguchi_former_eight_terms_recalled': '山口那津男',
    'jp_komeito_ishii_resignation_intent_20241031': '石井啓一',
    'jp_komeito_saito_recommended_20241107': '斉藤鉄夫',
    'jp_komeito_saito_convention_selection_20241109': '斉藤鉄夫',
    'jp_komeito_saito_in_office_convention_20241109': '斉藤鉄夫',
    'jp_komeito_ishii_former_20241110': '石井啓一',
    'jp_komeito_saito_in_office_diet_20241203': '斉藤鉄夫',
    'jp_komeito_takeya_interim_approved_20260122': None,
    'jp_komeito_interim_styled_representative_decided_20260122': None,
    'jp_komeito_saito_former_20260122': '斉藤鉄夫',
    'jp_komeito_chudo_founded_20260122': None,
    'jp_komeito_saito_chudo_co_representative_20260122': None,
    'jp_komeito_takeya_styled_representative_20260122': None,
    'jp_komeito_takeya_recommended_20260311': '竹谷とし子',
    'jp_komeito_takeya_assumption_stated_20260314': '竹谷とし子',
    'jp_komeito_takeya_convention_selection_20260314': '竹谷とし子',
    'jp_komeito_takeya_in_office_convention_20260314': '竹谷とし子',
    'jp_komeito_vacancy_recalled_january_2026': '斉藤鉄夫',
}
# Rows about another office, an acting or interim office carry that office as their role title; all others T_KOMEITO.
OTHER_TITLES = {
    'jp_komeito_acting_chair_in_place_19940616': '委員長代行 — acting chair of 公明党 (another office; claims only)',
    'jp_komeito_komei_representative_since_19941205_retrospective': '公明代表 — representative of 公明, the organization of 1994-1998 (another organization; claims only)',
    'jp_komeito_takeya_interim_approved_20260122': '代表代理 (styled 代表) — interim office under the party rules until a national convention elects a representative (claims only)',
    'jp_komeito_interim_styled_representative_decided_20260122': '代表代理 (styled 代表) — interim office under the party rules until a national convention elects a representative (claims only)',
    'jp_komeito_saito_chudo_co_representative_20260122': '中道改革連合共同代表 — co-representative of 中道改革連合 (another organization; claims only)',
    'jp_komeito_takeya_styled_representative_20260122': '代表代理 (styled 代表) — interim office under the party rules until a national convention elects a representative (claims only)',
}

NEW_SOURCES = list(RESPONSES)
NEW_CLAIMS = list(EVENTS)
# Exact holder observations of jp_komeito_representative: (name, attested_on, from, until), in chronological order.
HOLDERS = [
    ('石田幸四郎', '1993-01-29', None, None),
    ('神崎武法', '1998-11-08', None, None),
    ('神崎武法', '2002-11-03', None, None),
    ('神崎武法', '2004-10-31', None, None),
    ('太田昭宏', None, '2006-09-30', None),
    ('山口那津男', '2009-09-08', None, None),
    ('山口那津男', '2012-09-22', None, None),
    ('山口那津男', '2014-09-21', None, None),
    ('山口那津男', '2016-09-17', None, None),
    ('山口那津男', '2018-09-30', None, None),
    ('山口那津男', '2020-09-27', None, None),
    ('山口那津男', '2022-09-25', None, None),
    ('石井啓一', '2024-09-28', None, None),
    ('斉藤鉄夫', '2024-11-09', None, None),
    ('竹谷とし子', '2026-03-14', None, None),
]
HOLDER_CLAIMS = [
    ['jp_komeito_ishida_named_chair_by_secretary_general_19930129'],
    ['jp_komeito_kanzaki_in_office_founding_report_19981108'],
    ['jp_komeito_kanzaki_in_office_reappointed_20021103', 'jp_komeito_kanzaki_self_stated_continuing_20021103'],
    ['jp_komeito_kanzaki_self_stated_again_20041031'],
    ['jp_komeito_ota_in_office_convention_20060930', 'jp_komeito_ota_assumption_stated_20060930'],
    ['jp_komeito_yamaguchi_in_office_greeting_20090908'],
    ['jp_komeito_yamaguchi_in_office_convention_20120922'],
    ['jp_komeito_yamaguchi_in_office_convention_20140921'],
    ['jp_komeito_yamaguchi_in_office_convention_20160917'],
    ['jp_komeito_yamaguchi_in_office_convention_20180930'],
    ['jp_komeito_yamaguchi_in_office_convention_20200927'],
    ['jp_komeito_yamaguchi_in_office_convention_20220925'],
    ['jp_komeito_ishii_in_office_convention_20240928'],
    ['jp_komeito_saito_in_office_convention_20241109'],
    ['jp_komeito_takeya_assumption_stated_20260314', 'jp_komeito_takeya_in_office_convention_20260314'],
]
STARTS = [('太田昭宏', '2006-09-30')]
ENDS = []
START_CLAIMS = ['jp_komeito_ota_assumption_stated_20060930']
END_CLAIMS = []
REVIEW = ['KOMEITO-01', 'KOMEITO-02', 'KOMEITO-03', 'KOMEITO-04', 'KOMEITO-05', 'KOMEITO-06', 'KOMEITO-07', 'KOMEITO-08', 'KOMEITO-09', 'KOMEITO-10']
HOLDER_REVIEW = ['KOMEITO-01', 'KOMEITO-03', 'KOMEITO-04', 'KOMEITO-04', 'KOMEITO-05', 'KOMEITO-06', 'KOMEITO-06',
                 'KOMEITO-07', 'KOMEITO-07', 'KOMEITO-07', 'KOMEITO-07', 'KOMEITO-07', 'KOMEITO-08', 'KOMEITO-09',
                 'KOMEITO-10']
PEOPLE = ['石田幸四郎', '神崎武法', '太田昭宏', '山口那津男', '石井啓一', '斉藤鉄夫', '竹谷とし子']
SURNAMES = {'石田幸四郎': ('石田',), '神崎武法': ('神崎',), '太田昭宏': ('太田',), '山口那津男': ('山口',),
            '石井啓一': ('石井',), '斉藤鉄夫': ('斉藤', '齊藤', '斎藤'), '竹谷とし子': ('竹谷',)}
ACTING_TITLE = '委員長代行 — acting chair of 公明党 (another office; claims only)'
INTERIM_TITLE = '代表代理 (styled 代表) — interim office under the party rules until a national convention elects a representative (claims only)'
ISHIDA, KANZAKI_98 = ('石田幸四郎', '1993-01-29'), ('神崎武法', '1998-11-08')
ISHII, SAITO, TAKEYA = ('石井啓一', '2024-09-28'), ('斉藤鉄夫', '2024-11-09'), ('竹谷とし子', '2026-03-14')
ATTEST_KINDS = {'in_office_attestation'}
FROM_KINDS = {'assumption_stated'}
END_KINDS = {'end_of_office_stated'}
ELECTION_KINDS = {'assumption_recalled_by_holder', 'candidate_recommended', 'convention_selection',
                  'election_recalled_by_holder', 'national_representatives_meeting_selection'}
RESIGNATION_KINDS = {'office_vacancy_recalled', 'predecessor_referred_to_as_former', 'resignation_intent_announced'}
CONTINUATION_KINDS = {'in_office_continuation_attestation'}
ORGANIZATION_KINDS = {'organization_formed', 'organization_merger', 'organization_renaming_decided'}
INTERIM_KINDS = {'acting_chair_in_place', 'interim_representative_designated', 'interim_representative_styled',
                 'interim_title_decided'}
OTHER_OFFICE_KINDS = {'other_office_attestation', 'other_office_retrospective'}
RETROSPECTIVE_KINDS = {'retrospective_office_reference', 'retrospective_organization_record'}
HOLDER_KINDS = ATTEST_KINDS | FROM_KINDS | END_KINDS
ALL_KINDS = (HOLDER_KINDS | ELECTION_KINDS | RESIGNATION_KINDS | CONTINUATION_KINDS | ORGANIZATION_KINDS |
             INTERIM_KINDS | OTHER_OFFICE_KINDS | RETROSPECTIVE_KINDS)
# Kinds that never carry a structured date: retrospective records, undated recollections of another office, vacancies.
UNDATED_KINDS = RETROSPECTIVE_KINDS | {'other_office_retrospective', 'office_vacancy_recalled'}
COUNTS = {
    'sources_claims': (31, 64),
    'holder_never': (18, 46),
    'categories': (18, 7, 4, 4, 4, 2, 7, 15),
    'archived_diet': (24, 7),
}
# Dates that are never a holder's attested_on, start or end: selections, recommendations, conventions whose day no
# holder record carries, resignations, interim arrangements, organization events, month ends and leads' dates.
NEVER_HOLDER_DATE = {
    '1989-05-21', '1994-06-16', '1994-11-05', '1994-12-05', '1994-12-10', '1997-12-27', '1998-01-04', '1998-01-18',
    '1998-10-24', '1998-11-07', '2002-11-02', '2006-10-03', '2009-10-30', '2024-10-27', '2024-10-31', '2024-11-07',
    '2024-11-10', '2024-12-03', '2026-01-22', '2026-01-31', '2026-02-08', '2026-03-11', '2026-09-30', '2026-10-03',
}
# On each transition day these events stay separate claims.
TRANSITIONS = {
    '1994-06-16': ['acting_chair_in_place', 'in_office_continuation_attestation'],
    '2004-10-31': ['convention_selection', 'in_office_attestation'],
    '2006-09-30': ['assumption_stated', 'convention_selection', 'in_office_attestation', 'predecessor_referred_to_as_former'],
    '2009-09-08': ['election_recalled_by_holder', 'in_office_attestation', 'national_representatives_meeting_selection', 'predecessor_referred_to_as_former'],
    '2012-09-22': ['convention_selection', 'in_office_attestation'],
    '2014-09-21': ['convention_selection', 'in_office_attestation'],
    '2016-09-17': ['convention_selection', 'in_office_attestation'],
    '2018-09-30': ['convention_selection', 'in_office_attestation'],
    '2020-09-27': ['convention_selection', 'in_office_attestation'],
    '2022-09-25': ['convention_selection', 'in_office_attestation'],
    '2024-11-09': ['convention_selection', 'in_office_attestation'],
    '2026-01-22': ['interim_representative_designated', 'interim_representative_styled', 'interim_title_decided', 'organization_formed', 'other_office_attestation', 'predecessor_referred_to_as_former'],
    '2026-03-14': ['convention_selection', 'in_office_attestation'],
}
# Secondary sources and leads that must never be a source URL here.
LEAD_URL_MARKERS = ('wikipedia', 'britannica', 'kotobank', 'nikkei', 'asahi.com', 'yomiuri', 'mainichi', 'nhk.or.jp',
                    'jiji.com', 'sankei', 'cdp-japan.jp', 'komei.or.jp/km/', 'giin/news', '112505254X00219921104',
                    '112005254X00719910128', '111805254X00419900305', '112915261X01719940617', '115905254X00320040122',
                    '121405254X00320241007', 'eng/ayumi2', 'eng/shinshin', 'content/movement', 'p378965', 'p370030')
LEAD_REPORT_MARKERS = ('112505254X00219921104', '112915261X01719940617', '115905254X00320040122',
                       '121405254X00320241007', 'eng/ayumi2', 'content/movement26', 'p378965')
# Response shapes a server can generate per request, cache-busting queries, open searches and live listing pages.
PER_REQUEST_PATTERNS = ('cb=', '_cb', 'cbx=', 'any=', 'token=', 'sessionid', 'X-Amz-', 'Signature=', 'searchResult',
                        'opensearch', 'cdx/search', '?q=', '&q=', 'speaker=', 'from=', 'until=', 'startRecord', '?s=',
                        'fbclid', 'nocache', 'wp-json', 'doing_wp_cron')
# Date fields the packet never uses; none may remain.
STALE_IDS = ('"jp_diet_', 'attested_period', 'printed_date', 'statement_date', 'printed_range')
SCOPE_PHRASES = ('never feed a holder', 'procedure only, never a date', "outgoing leader's last day",
                 'never read the prime-ministership (jp_pm)', 'claims about the organizations only',
                 'never merged into one identity', 'only 太田昭宏', '代表代理')
HOLDER_CLAIM_SET = {cid for ids in HOLDER_CLAIMS for cid in ids}
NEVER_HOLDER = tuple(cid for cid in EVENTS if cid not in HOLDER_CLAIM_SET)
ELECTIONS = tuple(cid for cid, (_d, kind, _o) in EVENTS.items() if kind in ELECTION_KINDS)
RESIGNATIONS = tuple(cid for cid, (_d, kind, _o) in EVENTS.items() if kind in RESIGNATION_KINDS)
CONTINUATION = tuple(cid for cid, (_d, kind, _o) in EVENTS.items() if kind in CONTINUATION_KINDS)
ORGANIZATION = tuple(cid for cid, (_d, kind, _o) in EVENTS.items() if kind in ORGANIZATION_KINDS)
INTERIM = tuple(cid for cid, (_d, kind, _o) in EVENTS.items() if kind in INTERIM_KINDS)
OTHER_OFFICE = tuple(cid for cid, (_d, kind, _o) in EVENTS.items() if kind in OTHER_OFFICE_KINDS)
RETROSPECTIVE = tuple(cid for cid, (_d, kind, _o) in EVENTS.items() if kind in RETROSPECTIVE_KINDS)
UNDATED = tuple(cid for cid, (_d, _k, _o) in EVENTS.items() if _d is None)
REPORT = research.RESEARCH / 'japan-komeito-representatives-1990-2026-31.md'
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-31.md'


def load_rows(packet):
    rows = {}
    for source in packet['sources'][EARLIER_SOURCE_COUNT:EARLIER_SOURCE_COUNT + len(NEW_SOURCES)]:
        extract = json.loads((research.ROOT / source['snapshot']['path']).read_text(encoding='utf-8'))
        rows.update({row['claim_id']: row for row in extract['rows']})
    return rows


def komeito_rules(packet, rows):
    """Rule-based checks that hold without the pinned holder list; raise AssertionError, KeyError or IndexError."""
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    source_of = {c['id']: s['id'] for s in packet['sources'] for c in s['claims']}
    org, = [o for o in packet['organizations'] if o['id'] == ORG_ID]
    assert (org['name'], org['lifecycle'], org['represented_party_ids']) == (
        '公明党', {'status': 'unknown', 'from': None, 'until': None,
                 'note': 'Participation at one election does not establish organization birth, dissolution or continuity.'},
        []), 'the 1994 division, the 1998 merger and 中道改革連合 never become a lifecycle or a mapping'
    assert [r['id'] for r in org['roles']] == [ROLE], 'exactly one party role on the 公明党 observation'
    role = org['roles'][0]
    assert (role['title'], role['kind']) == (T_KOMEITO, 'party_leader')
    assert list(role) == ['id', 'title', 'kind', 'sources', 'claim_ids', 'holder_claims', 'scope_note']
    # No organization or institution is added, and no other role changes identity.
    assert len(packet['organizations']) == 16 and [i['id'] for i in packet['institutions']] == GROUPS + ['jp_prime_minister']
    assert sorted(r['id'] for e in packet['organizations'] + packet['institutions'] for r in e['roles']) == sorted(
        EARLIER_ROLES + [ROLE]), 'one new role, nothing else'
    assert all(not e['represented_party_ids'] for e in packet['organizations'] + packet['institutions'])
    holders = role['holder_claims']
    previous = ''
    for holder in holders:
        name = holder['name']
        assert name in SURNAMES, name
        assert list(holder) == ['name', 'attested_on', 'from', 'until', 'sources', 'claim_ids', 'note', 'uncertainty']
        dated = [x for x in (holder['attested_on'], holder['from']) if x]
        assert len(dated) == 1, (name, 'a holder is dated by exactly one of attested_on and from')
        day = dated[0]
        assert day > previous, (name, 'holders stay in chronological order')
        previous = day
        assert day not in NEVER_HOLDER_DATE and day <= research.CUTOFF, (name, day)
        if holder['until']:
            assert holder['until'] not in NEVER_HOLDER_DATE and day <= holder['until'] <= research.CUTOFF, name
        assert not set(holder['claim_ids']) & set(NEVER_HOLDER), name
        expected, kinds = [], []
        for cid in holder['claim_ids']:
            row, claim = rows[cid], claims[cid]
            assert cid in role['claim_ids'] and cid.startswith('jp_komeito_'), cid
            assert row['holder_name'] == name and (row['role_id'], row['role_title']) == (ROLE, T_KOMEITO), (name, cid)
            assert any(s in claim['text'] for s in SURNAMES[name]), (name, cid)
            if row['event_kind'] in END_KINDS:
                assert claim['attested_on'] == holder['until'] == row['attested_on'], (name, cid, 'an end cites its own day')
            else:
                assert row['event_kind'] in ((FROM_KINDS | ATTEST_KINDS) if holder['from'] else ATTEST_KINDS), (name, cid)
                assert claim['attested_on'] == day == row['attested_on'], (name, cid, "a cited claim carries the holder's day")
            kinds.append(row['event_kind'])
            if source_of[cid] not in expected:
                expected.append(source_of[cid])
        assert holder['sources'] == expected, name
        assert bool(holder['until']) == any(k in END_KINDS for k in kinds), (name, 'an until needs a stated end')
        assert bool(holder['from']) == any(k in FROM_KINDS for k in kinds), (name, 'a from needs a stated assumption')
        assert any(k not in END_KINDS for k in kinds), (name, 'a holder needs its own dating claim')
    # Every named attestation, stated start or stated end of this office is cited by exactly one holder; nothing else is.
    for cid, row in rows.items():
        kind, name = row['event_kind'], row['holder_name']
        assert kind in ALL_KINDS, (cid, kind)
        cited = [h for h in holders if cid in h['claim_ids']]
        if kind in HOLDER_KINDS and name and row['role_title'] == T_KOMEITO:
            assert len(cited) == 1 and cited[0]['name'] == name, cid
        else:
            assert not cited, cid
        if kind in INTERIM_KINDS | OTHER_OFFICE_KINDS or row['role_title'] != T_KOMEITO:
            assert name is None and not cited, cid
        if kind in ORGANIZATION_KINDS:
            assert name is None, cid
        if kind in UNDATED_KINDS:
            assert 'attested_on' not in claims[cid] and row['attested_on'] is None, cid
        if 'attested_on' in claims[cid]:
            assert claims[cid]['attested_on'] == row['attested_on'] <= research.CUTOFF, cid
        else:
            assert row['attested_on'] is None, cid
        assert not {'period', 'attested_period', 'printed_date', 'statement_date', 'printed_range'} & set(claims[cid]), cid
    # Party office and state office never feed each other; the LDP presidency and the SDP chair are other parties' roles.
    pm_role = packet['institutions'][-1]['roles'][0]
    assert len(pm_role['holder_claims']) == PM_HOLDER_COUNT
    pm_claims = set(pm_role['claim_ids']) | {c for h in pm_role['holder_claims'] for c in h['claim_ids']} | set(
        packet['institutions'][-1]['claim_ids'])
    pm_sources = set(pm_role['sources']) | {s for h in pm_role['holder_claims'] for s in h['sources']} | set(
        packet['institutions'][-1]['sources'])
    for other_org, other_role, count in ((LDP_ORG, LDP_ROLE, LDP_HOLDER_COUNT), (SDP_ORG, SDP_ROLE, SDP_HOLDER_COUNT)):
        entry, = [o for o in packet['organizations'] if o['id'] == other_org]
        assert [r['id'] for r in entry['roles']] == [other_role] and len(entry['roles'][0]['holder_claims']) == count
    komeito_claims = set(role['claim_ids']) | {c for h in holders for c in h['claim_ids']}
    komeito_sources = set(role['sources']) | {s for h in holders for s in h['sources']}
    assert not komeito_claims & pm_claims and not komeito_sources & pm_sources
    assert not any(c.startswith('jp_komeito') for c in pm_claims) and not any(s.startswith('jp_komeito') for s in pm_sources)
    assert all(c.startswith('jp_komeito_') for c in komeito_claims) and all(s.startswith('jp_komeito_') for s in komeito_sources)
    for entry in packet['organizations'] + packet['institutions']:
        if entry is not org:
            assert not set(entry['claim_ids']) & komeito_claims and not set(entry['sources']) & komeito_sources, entry['id']
            for other in entry['roles']:
                assert not set(other['claim_ids']) & komeito_claims and not set(other['sources']) & komeito_sources, other['id']
                assert not {c for h in other['holder_claims'] for c in h['claim_ids']} & komeito_claims, other['id']
    assert set(role['claim_ids']) <= set(org['claim_ids']) and set(role['sources']) <= set(org['sources'])


def komeito_invariants(packet, rows):
    """The rules plus the exact pinned holders, rows and events this packet intends."""
    komeito_rules(packet, rows)
    org, = [o for o in packet['organizations'] if o['id'] == ORG_ID]
    role = org['roles'][0]
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    got = [(h['name'], h['attested_on'], h['from'], h['until']) for h in role['holder_claims']]
    assert got == HOLDERS, got
    assert [h['claim_ids'] for h in role['holder_claims']] == HOLDER_CLAIMS
    assert [(h['name'], h['from']) for h in role['holder_claims'] if h['from']] == STARTS
    assert [(h['name'], h['until']) for h in role['holder_claims'] if h['until']] == ENDS
    assert {cid: row['holder_name'] for cid, row in rows.items()} == ROW_HOLDERS
    assert {cid: row['role_title'] for cid, row in rows.items() if row['role_title'] != T_KOMEITO} == OTHER_TITLES
    for cid, (day, kind, obs) in EVENTS.items():
        assert claims[cid].get('attested_on') == day, cid
        assert (rows[cid]['attested_on'], rows[cid]['event_kind'], rows[cid]['review_observation']) == (day, kind, obs), cid


class JapanKomeitoRepresentativesTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.raw = (research.ROOT / research.RESEARCH / 'japan.json').read_text(encoding='utf-8')
        cls.packet = json.loads(cls.raw)
        cls.sources = {s['id']: s for s in cls.packet['sources']}
        cls.claims = {c['id']: c for s in cls.packet['sources'] for c in s['claims']}
        cls.claim_source = {c['id']: s['id'] for s in cls.packet['sources'] for c in s['claims']}
        cls.org = next(o for o in cls.packet['organizations'] if o['id'] == ORG_ID)
        cls.role = cls.org['roles'][0]
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
        self.assertEqual(order[EARLIER_SOURCE_COUNT:EARLIER_SOURCE_COUNT + len(NEW_SOURCES)], NEW_SOURCES)
        self.assertEqual(order[EARLIER_SOURCE_COUNT + len(NEW_SOURCES):], jcp42.NEW_SOURCES)
        self.assertFalse([sid for sid in order[:EARLIER_SOURCE_COUNT] if sid in RESPONSES or sid.startswith('jp_komeito')])
        self.assertEqual((len(ids['entries']), len(ids['roles'])), (24, 7))
        self.assertEqual((len(self.packet['organizations']), len(self.packet['institutions'])), (16, 8))
        self.assertEqual(self.new_claims, NEW_CLAIMS)
        self.assertEqual(set(EVENTS), set(self.rows))
        # Every new claim is either a holder claim or a claim that never feeds a holder, never both.
        self.assertFalse(HOLDER_CLAIM_SET & set(NEVER_HOLDER))
        self.assertEqual(HOLDER_CLAIM_SET | set(NEVER_HOLDER), set(self.new_claims))
        self.assertEqual((len(HOLDER_CLAIM_SET), len(NEVER_HOLDER)), COUNTS['holder_never'])
        self.assertEqual((len(ELECTIONS), len(RESIGNATIONS), len(CONTINUATION), len(ORGANIZATION), len(INTERIM),
                          len(OTHER_OFFICE), len(RETROSPECTIVE), len(UNDATED)), COUNTS['categories'])
        # The role and its organization cite every new claim and source, after the organization's earlier ones, and
        # nothing else cites them.
        self.assertEqual(self.role['claim_ids'], NEW_CLAIMS)
        self.assertEqual(self.role['sources'], NEW_SOURCES)
        self.assertEqual(self.org['claim_ids'], ORG_EARLIER_CLAIMS + NEW_CLAIMS)
        self.assertEqual(self.org['sources'], ORG_EARLIER_SOURCES + NEW_SOURCES)
        for entry in self.packet['organizations'] + self.packet['institutions']:
            if entry is not self.org:
                self.assertFalse(set(self.new_claims) & set(entry['claim_ids']), entry['id'])
                self.assertFalse(set(NEW_SOURCES) & set(entry['sources']), entry['id'])
        observations = re.findall(r'^### (KOMEITO-\d\d)\b', self.report, re.M)
        self.assertEqual(observations, REVIEW)
        self.assertEqual({row['review_observation'] for row in self.rows.values()}, set(REVIEW))
        self.assertEqual([self.rows[ids_[0]]['review_observation'] for ids_ in HOLDER_CLAIMS], HOLDER_REVIEW)
        people = list(dict.fromkeys(h[0] for h in HOLDERS))
        self.assertEqual(people, PEOPLE)
        self.assertLessEqual(len(people), 10)
        for stale in STALE_IDS:
            self.assertNotIn(stale, self.raw, stale)
            for extract in self.extracts.values():
                self.assertNotIn(stale, json.dumps(extract, ensure_ascii=False), stale)

    def test_holders_are_exactly_as_intended(self):
        komeito_invariants(self.packet, self.rows)
        for holder in self.role['holder_claims']:
            if holder['from']:
                self.assertTrue(holder['note'].startswith('From '), holder['name'])
            else:
                self.assertTrue(holder['note'].startswith('Observed on '), holder['name'])
            if not holder['from'] and not holder['until']:
                self.assertTrue(holder['uncertainty'].startswith('No start or end is stated'), holder['name'])
            for cid in holder['claim_ids']:
                kind = self.rows[cid]['event_kind']
                lead = {'end_of_office_stated': 'States the day office ended', 'assumption_stated': 'States the day office was assumed'}
                expected = lead.get(kind, 'Cited, with the statement of assumption' if holder['from'] else 'Dates the holder observation')
                self.assertTrue(self.claims[cid]['uncertainty'].startswith(expected), cid)
        # Row holder names are normalised holders of this office; the printed forms stay in the claim text.
        self.assertEqual({v for v in ROW_HOLDERS.values()} - {None}, set(SURNAMES))
        for cid, name in ROW_HOLDERS.items():
            if name:
                self.assertTrue(any(s in self.claims[cid]['text'] for s in SURNAMES[name]), cid)
        # Holders who also held state office (石田幸四郎 as 総務庁長官, 斉藤鉄夫 as 国土交通相) are dated only by party-office
        # wording; nothing about designation, appointment to a ministry or cabinets is cited.
        for cid in HOLDER_CLAIM_SET:
            self.assertNotRegex(self.claims[cid]['text'], r'内閣総理大臣に(任命|指名)|首班指名|組閣|大臣に任命', cid)

    def test_takeya_acceptance_does_not_infer_an_effective_start(self):
        holder, = [h for h in self.role['holder_claims'] if h['name'] == '竹谷とし子']
        self.assertIsNone(holder['from'])
        self.assertEqual(holder['attested_on'], '2026-03-14')
        cid = 'jp_komeito_takeya_assumption_stated_20260314'
        self.assertIn('ただいま皆さまのご信任を賜り', self.claims[cid]['text'])
        self.assertEqual(self.rows[cid]['event_kind'], 'in_office_attestation')
        mutated = copy.deepcopy(self.packet)
        role = next(o for o in mutated['organizations'] if o['id'] == ORG_ID)['roles'][0]
        candidate, = [h for h in role['holder_claims'] if h['name'] == '竹谷とし子']
        candidate.update(attested_on=None, **{'from': '2026-03-14'})
        with self.assertRaises(AssertionError):
            komeito_rules(mutated, self.rows)

    def test_starts_ends_and_claims_that_never_feed_a_holder(self):
        claims, rows = self.claims, self.rows
        for day, expected in TRANSITIONS.items():
            kinds = sorted({rows[cid]['event_kind'] for cid, (d, _k, _o) in EVENTS.items() if d == day})
            for kind in expected:
                self.assertIn(kind, kinds, (day, kind))
        for cid in ELECTIONS + RESIGNATIONS + CONTINUATION + ORGANIZATION + INTERIM + OTHER_OFFICE + RETROSPECTIVE:
            self.assertNotIn(cid, HOLDER_CLAIM_SET, cid)
        for cid in RESIGNATIONS:
            self.assertRegex(claims[cid]['uncertainty'], r'(?i)never an until|not an end|no end', cid)
        for cid in CONTINUATION + ELECTIONS:
            self.assertRegex(claims[cid]['uncertainty'], r'(?i)never a holder', cid)
        for cid in INTERIM:
            self.assertRegex(claims[cid]['uncertainty'], r'(?i)claims only', cid)
            self.assertRegex(claims[cid]['uncertainty'], r'(?i)never a holder', cid)
            self.assertIsNone(rows[cid]['holder_name'], cid)
        for cid in OTHER_OFFICE:
            self.assertRegex(claims[cid]['uncertainty'], r'(?i)another organization|claims only', cid)
            self.assertIsNone(rows[cid]['holder_name'], cid)
        for cid in ORGANIZATION:
            self.assertRegex(claims[cid]['uncertainty'], r'(?i)organization', cid)
            self.assertIsNone(rows[cid]['holder_name'], cid)
        for cid in RETROSPECTIVE:
            self.assertRegex(claims[cid]['uncertainty'], r'(?i)never a (holder )?boundary', cid)
        for cid in UNDATED:
            self.assertNotIn('attested_on', claims[cid], cid)
            self.assertRegex(claims[cid]['uncertainty'], r'(?i)no structured date', cid)
        start = [cid for cid, row in rows.items() if row['event_kind'] in FROM_KINDS]
        self.assertEqual(start, START_CLAIMS)
        end = [cid for cid, row in rows.items() if row['event_kind'] in END_KINDS]
        self.assertEqual(end, END_CLAIMS)
        # The interim 代表代理 styled 代表 (22 January to 14 March 2026) and the 1994 acting chair are claims only.
        interim_titles = {rows[cid]['role_title'] for cid in INTERIM}
        self.assertEqual(interim_titles, {INTERIM_TITLE, ACTING_TITLE})
        self.assertIn('代表代理', claims['jp_komeito_takeya_interim_approved_20260122']['text'])
        self.assertIn('「代表」と呼称', claims['jp_komeito_interim_styled_representative_decided_20260122']['text'])
        takeya = [h for h in self.role['holder_claims'] if h['name'] == '竹谷とし子']
        self.assertEqual([(h['attested_on'], h['from']) for h in takeya], [('2026-03-14', None)])
        # Organization events: the 1994 division, 公明, 公明新党, 新進党, 新党平和, 黎明クラブ, the 1998 merger and 中道改革連合
        # are claims only; each is kept apart and none is merged or mapped.
        org_text = ' '.join(claims[cid]['text'] for cid in ORGANIZATION + RETROSPECTIVE)
        for name in ('公明新党', '新進党', '新党平和', '黎明クラブ', '中道改革連合', 'Komei'):
            self.assertIn(name, org_text, name)
        scope = self.role['scope_note']
        self.assertTrue(scope.startswith('CLAUDE-C01-31 adds this role'))
        for phrase in SCOPE_PHRASES:
            self.assertIn(phrase, scope)
        unresolved = self.org['coverage']['unresolved']
        self.assertEqual(len(unresolved), ORG_EARLIER_UNRESOLVED + 1)
        self.assertTrue(unresolved[-1].startswith('Komeito representatives 1990-2026 (CLAUDE-C01-31)'))
        packet_unresolved = self.packet['coverage']['unresolved']
        self.assertEqual(sum('CLAUDE-C01-31' in u for u in packet_unresolved), 1)
        self.assertTrue(packet_unresolved[-2].startswith('Komeito representatives 1990-2026 (CLAUDE-C01-31)'))
        self.assertTrue(packet_unresolved[-3].startswith('SDP chairs 1990-2026 (CLAUDE-C01-29)'))
        self.assertTrue(packet_unresolved[-1].startswith('JCP chairs and DPFP representatives 1990-2026 (CLAUDE-C01-42)'))

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
                           'identity-encoded body', 'Downloaded again by this packet', '30 minutes or more apart'):
                self.assertIn(phrase, extract['provenance_note'], (sid, phrase))
            self.assertIn('No open license, portrait permission or likeness approval', extract['rights_note'])
            self.assertEqual(extract['visual_review']['pdf_pages_one_based'], [])
            self.assertEqual(source['source_type'], SOURCE_TYPES[sid], sid)
            self.assertTrue(source['source_type'].startswith('primary_') and source['scope_note'])
            snapshot = source['snapshot']
            self.assertTrue(snapshot['path'].startswith('docs/campaign-certification/C01/research/sources/japan-'))
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
                self.assertEqual((row['observation_id'], row['role_id']), (ORG_ID, ROLE))
                self.assertNotIn('name', row)
                self.assertEqual(list(row), ['claim_id', 'observation_id', 'review_observation', 'role_id', 'holder_name',
                                             'role_title', 'event_kind', 'attested_on', 'text', 'locator'])
                self.assertEqual(list(claim), ['id', 'text'] + (['attested_on'] if 'attested_on' in claim else []) +
                                 ['locator', 'uncertainty'])
        self.assertEqual(len({self.sources[s]['snapshot']['path'] for s in NEW_SOURCES}), len(NEW_SOURCES))
        earlier_paths = {s['snapshot']['path'] for s in self.packet['sources'][:EARLIER_SOURCE_COUNT] if 'snapshot' in s}
        self.assertFalse(earlier_paths & {self.sources[s]['snapshot']['path'] for s in NEW_SOURCES})
        self.assertEqual({cid: (row['attested_on'], row['event_kind'], row['review_observation'])
                          for cid, row in self.rows.items()}, EVENTS)
        for sid, stamp in ARCHIVED.items():
            source, extract = self.sources[sid], self.extracts[sid]
            url = urlsplit(source['url'])
            self.assertEqual(url.hostname, 'web.archive.org')
            self.assertTrue(url.path.startswith(f'/web/{stamp}id_/http'), sid)
            self.assertLess(stamp, '20260907')
            self.assertEqual(extract['original_url'], source['original_url'])
            self.assertEqual(source['url'].split('id_/', 1)[1].replace(':80/', '/', 1), source['original_url'])
            self.assertNotIn(':80', source['original_url'])
            self.assertRegex(source['original_url'], r'^https?://(www\.)?komei\.or\.jp/')
            self.assertEqual(extract['archive_capture_utc'].replace('-', '').replace(':', '').replace('T', '').rstrip('Z'), stamp)
            self.assertIn('Raw Internet Archive capture', extract['provenance_note'])
            self.assertRegex(extract['source_character_encoding'], r'^(UTF-8|Shift_JIS|EUC-JP)')
        self.assertEqual((len(ARCHIVED), len(DIET)), COUNTS['archived_diet'])
        self.assertEqual(set(ARCHIVED) | set(DIET), set(NEW_SOURCES))
        for sid in DIET:
            self.assertNotIn('original_url', self.sources[sid])
            record = self.extracts[sid]['diet_record']
            self.assertIn(record['issueID'], self.sources[sid]['url'])
            self.assertEqual(record['record_page_url'], f"https://kokkai.ndl.go.jp/txt/{record['issueID']}")
            self.assertIn('(cache-busting query)', self.extracts[sid]['provenance_note'], sid)
            for row in self.extracts[sid]['rows']:
                self.assertIn('speaker', row['locator'], row['claim_id'])

    def test_recovered_source_locators_cover_the_claimed_passages(self):
        # Checked against the returned article bodies: the guest greeting precedes Kanzaki's
        # continuation; Ota's report mixes p/br blocks; Saito's condition is in the next paragraph.
        expected = {
            'jp_komeito_kanzaki_self_stated_again_20041031': {'paragraph': 3},
            'jp_komeito_ota_selected_20060930': {
                'section': 'newsbody',
                'passage_starts': ['「新しい公明党」が勇躍スタート', 'これに先立ち、代表選出が行われ']},
            'jp_komeito_saito_recommended_20241107': {'paragraph': [1, 2]},
        }
        claims = {c['id']: c for source in self.packet['sources'] for c in source['claims']}
        for cid, locator in expected.items():
            with self.subTest(claim=cid):
                self.assertEqual(claims[cid]['locator'], locator)
                self.assertEqual(self.rows[cid]['locator'], locator)

    def test_response_identities_are_reproducible_urls(self):
        for sid in NEW_SOURCES:
            url = self.sources[sid]['url']
            for pattern in PER_REQUEST_PATTERNS:
                self.assertNotIn(pattern, url, url)
            parts = urlsplit(url)
            self.assertEqual(parts.scheme, 'https', url)
            if sid in ARCHIVED:
                self.assertRegex(parts.path, r'^/web/\d{14}id_/https?://', url)
            else:
                self.assertEqual(parts.hostname, 'kokkai.ndl.go.jp', url)
                self.assertRegex(url, r'^https://kokkai\.ndl\.go\.jp/api/(meeting\?issueID=\w+|speech\?issueID=\w+&speechNumber=\d+)'
                                      r'&recordPacking=json$')
            # No live party page is recorded: every komei.or.jp response is a fixed pre-cutoff capture.
            self.assertFalse(parts.hostname.endswith('komei.or.jp'), url)
        seen = {}
        for source in self.packet['sources']:
            seen.setdefault(source['url'], []).append(source['id'])
        for sid in NEW_SOURCES:
            self.assertEqual(seen[self.sources[sid]['url']], [sid], sid)

    def test_secondary_and_unimported_leads_stay_out_of_the_packet(self):
        for sid in NEW_SOURCES:
            for marker in LEAD_URL_MARKERS:
                self.assertNotIn(marker, self.sources[sid]['url'], sid)
        lowered = self.raw.lower()
        for marker in ('wikipedia', 'britannica', 'kotobank', 'nikkei', 'asahi.com', 'yomiuri', 'mainichi', 'nhk.or.jp',
                       'jiji.com', 'komei.or.jp/km/'):
            self.assertNotIn(marker, lowered, marker)
        leads, added = self.section('Leads not imported'), self.section('Sources added')
        for marker in LEAD_REPORT_MARKERS:
            self.assertIn(marker, leads, marker)
            self.assertNotIn(marker, added, marker)
        # Nothing after the cutoff: the late-September 2026 recommendation of a successor and the October 2026 convention.
        self.assertNotIn('岡本三成', self.raw)

    def test_packet_formatting_is_preserved(self):
        data = (research.ROOT / research.RESEARCH / 'japan.json').read_bytes()
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

        def org(packet):
            return next(o for o in packet['organizations'] if o['id'] == ORG_ID)

        def role(packet):
            return org(packet)['roles'][0]

        def pm_role(packet):
            return packet['institutions'][-1]['roles'][0]

        def other_role(packet, org_id):
            return next(o for o in packet['organizations'] if o['id'] == org_id)['roles'][0]

        def holder(packet, index):
            return role(packet)['holder_claims'][index]

        def cite(index, cid, **dates):
            def change(packet):
                holder(packet, index).update(dates)
                holder(packet, index)['claim_ids'].append(cid)
                sid = next(s['id'] for s in packet['sources'] for c in s['claims'] if c['id'] == cid)
                if sid not in holder(packet, index)['sources']:
                    holder(packet, index)['sources'].append(sid)
            return change

        def extra_holder(name, day, cid, start=False):
            return {'name': name, 'attested_on': None if start else day, 'from': day if start else None, 'until': None,
                    'sources': [self.claim_source[cid]], 'claim_ids': [cid], 'note': 'x', 'uncertainty': 'x'}

        index = {(h[0], h[1] or h[2]): i for i, h in enumerate(HOLDERS)}
        ishida = index[ISHIDA]
        kanzaki98 = index[KANZAKI_98]
        ishii = index[ISHII]
        saito = index[SAITO]
        takeya = index[TAKEYA]
        validator_cases = [
            (lambda p: source(p, NEW_SOURCES[0])['snapshot'].update(sha256='0' * 64), 'checksum mismatch'),
            (lambda p: source(p, NEW_SOURCES[-1])['snapshot'].update(bytes=1), 'checksum mismatch'),
            (lambda p: holder(p, takeya).update({'from': '2026-09-08'}), 'exceeds cutoff'),
            (lambda p: claim(p, 'jp_komeito_takeya_in_office_convention_20260314').update(attested_on='2026-09-08'),
             'exceeds cutoff'),
            (lambda p: holder(p, ishida).update(until='1990-01-01', attested_on=None, **{'from': '1993-01-29'}),
             'Reversed historical interval'),
            (lambda p: holder(p, ishii)['claim_ids'].append('jp_komeito_saito_in_office_convention_20241109'), 'cited source'),
            (lambda p: role(p)['claim_ids'].append('jp_komeito_does_not_exist'), 'Unknown'),
        ]
        for change, message in validator_cases:
            with self.subTest(message=message), self.assertRaisesRegex(ValueError, message):
                self.validate(mutated(change))

        rule_cases = [
            # A successor's selection or start used as an end.
            ('successor attestation used as an end (石田)', lambda p: holder(p, ishida).update(until='1998-11-08')),
            ('successor selection used as an end (石井)', lambda p: holder(p, ishii).update(until='2024-11-09')),
            ('successor start used as an end (斉藤)', lambda p: holder(p, saito).update(until='2026-03-14')),
            ('announced resignation cited as an end (石井)',
             cite(ishii, 'jp_komeito_ishii_resignation_intent_20241031', until='2024-10-31')),
            ('predecessor reference cited as an end (斉藤)', cite(saito, 'jp_komeito_saito_former_20260122', until='2026-01-22')),
            ('retrospective division cited as an end (石田)',
             cite(ishida, 'jp_komeito_history_division_retrospective', until='1994-12-05')),
            # An election, recommendation or convention date used as a start without a stated assumption.
            ('election date used as a start (斉藤)', lambda p: holder(p, saito).update({'from': '2024-11-09', 'attested_on': None})),
            ('convention date used as a start (石井)', lambda p: holder(p, ishii).update({'from': '2024-09-28', 'attested_on': None})),
            ('recommendation cited as a start (竹谷)',
             cite(takeya, 'jp_komeito_takeya_recommended_20260311')),
            ('acceptance promoted to effective start (竹谷)', lambda p: holder(p, takeya).update({'from': '2026-03-14', 'attested_on': None})),
            ('election claim cited by a holder (斉藤)', cite(saito, 'jp_komeito_saito_convention_selection_20241109')),
            ('retrospective record cited by a holder (石田)',
             cite(ishida, 'jp_komeito_history_ishida_chair_in_cabinet_retrospective')),
            ('organization claim cited by a holder (神崎 1998)', cite(kanzaki98, 'jp_komeito_merger_form_reported')),
            # Acting or interim service added as a holder.
            ('1994 acting chair added as a holder', lambda p: role(p)['holder_claims'].insert(
                ishida + 1, extra_holder('石田幸四郎', '1994-06-16', 'jp_komeito_acting_chair_in_place_19940616'))),
            ('2026 interim 代表代理 added as a holder', lambda p: role(p)['holder_claims'].insert(
                takeya, extra_holder('竹谷とし子', '2026-01-22', 'jp_komeito_takeya_interim_approved_20260122'))),
            ('interim styled 就任 used as a start', lambda p: role(p)['holder_claims'].insert(
                takeya, extra_holder('竹谷とし子', '2026-01-22', 'jp_komeito_takeya_styled_representative_20260122', start=True))),
            ('continuation attestation added as a holder (石田 1994)', lambda p: role(p)['holder_claims'].insert(
                ishida + 1, extra_holder('石田幸四郎', '1994-06-16', 'jp_komeito_ishida_still_chair_19940616'))),
            # A cross-role, cross-party or state-office claim feeding the role, or the reverse.
            ('jp_pm claim cited by a party holder', lambda p: holder(p, saito)['claim_ids'].append(
                pm_role(p)['holder_claims'][-1]['claim_ids'][0])),
            ('LDP claim cited by a party holder', lambda p: holder(p, ishida)['claim_ids'].append(
                other_role(p, LDP_ORG)['holder_claims'][0]['claim_ids'][0])),
            ('SDP claim moved onto this role', lambda p: role(p)['claim_ids'].append(
                other_role(p, SDP_ORG)['claim_ids'][0])),
            ('party claim moved onto the prime-ministership', lambda p: pm_role(p)['claim_ids'].append(
                'jp_komeito_saito_in_office_diet_20241203')),
            ('party role copied to another party', lambda p: next(o for o in p['organizations'] if o['id'] == CDP_ORG)[
                'roles'].append(dict(copy.deepcopy(role(p)), id='jp_komeito_representative_copy'))),
            ('second 公明党 role', lambda p: org(p)['roles'].append(dict(copy.deepcopy(role(p)), id='jp_komeito_deputy'))),
            ('new institution for 中道改革連合', lambda p: p['institutions'].append(
                dict(copy.deepcopy(p['institutions'][-1]), id='jp_chudo_office'))),
            ('merger turned into a lifecycle boundary', lambda p: org(p)['lifecycle'].update({'from': '1998-11-07'})),
            ('division turned into a mapping', lambda p: org(p)['represented_party_ids'].append('Japan/jp_komei_1994')),
            # The order and the structured dates.
            ('holders out of order', lambda p: role(p)['holder_claims'].reverse()),
            ('retrospective merger given a structured date', lambda p: claim(
                p, 'jp_komeito_history_reformation_retrospective').update(attested_on='1998-11-07')),
            ('beyond-cutoff attestation', lambda p: holder(p, takeya).update({'from': '2026-09-08'})),
        ]
        komeito_rules(self.packet, self.rows)
        komeito_invariants(self.packet, self.rows)
        for label, change in rule_cases:
            with self.subTest(label=label):
                packet = mutated(change)
                with self.assertRaises((AssertionError, KeyError, IndexError, ValueError, StopIteration)):
                    komeito_rules(packet, self.rows)
                with self.assertRaises((AssertionError, KeyError, IndexError, ValueError, StopIteration)):
                    komeito_invariants(packet, self.rows)
        # Row-level mutations: an organization row given a holder, an interim row relabelled, an announcement made an end.
        for cid, field, value in (('jp_komeito_chudo_founded_20260122', 'holder_name', '斉藤鉄夫'),
                                  ('jp_komeito_takeya_interim_approved_20260122', 'event_kind', 'in_office_attestation'),
                                  ('jp_komeito_takeya_interim_approved_20260122', 'role_title', T_KOMEITO),
                                  ('jp_komeito_saito_chudo_co_representative_20260122', 'holder_name', '斉藤鉄夫'),
                                  ('jp_komeito_ishii_resignation_intent_20241031', 'event_kind', 'end_of_office_stated')):
            rows = copy.deepcopy(self.rows)
            rows[cid][field] = value
            with self.subTest(row=cid, field=field), self.assertRaises(AssertionError):
                komeito_invariants(self.packet, rows)
        # Event collapses and re-dating are caught by the pinned events.
        for cid, day in (('jp_komeito_saito_recommended_20241107', '2024-11-09'),
                         ('jp_komeito_takeya_recommended_20260311', '2026-03-14'),
                         ('jp_komeito_ishii_resignation_intent_20241031', '2024-11-09')):
            with self.subTest(collapsed=cid), self.assertRaises(AssertionError):
                komeito_invariants(mutated(lambda p: claim(p, cid).update(attested_on=day)), self.rows)

    def test_report_and_handoff_close_no_parent_gate(self):
        self.assertIn('ready_for_review', self.report)
        table = self.section('Outcome')
        for obs in REVIEW:
            row, = [line for line in table.splitlines() if line.startswith(f'| {obs} ')]
            self.assertRegex(row, r'\*\*(Accepted in part|Accepted|Not established):\*\*')
        for marker in ('C01', 'C06', 'S23', 'WC1', 'CP1'):
            self.assertNotRegex(self.report, rf'\b{marker}\b[^.\n]*\bis (now )?complete\b')
        notes = self.section('Integration notes')
        for text in ('52b23d59', 'ff01ee61', 'research-index.json', 'only file', 'test_japan_research_s10d.py',
                     'test_japan_prime_ministers_c01_12.py', 'test_japan_prime_ministers_c01_13.py',
                     'test_japan_ldp_presidents_c01_18.py', 'test_japan_sdp_chairs_c01_29.py'):
            self.assertIn(text, notes)
        identities = self.section('Response identities and stability checks')
        for sid, (size, sha) in RESPONSES.items():
            self.assertIn(f'`{sid}`', identities, sid)
            self.assertIn(sha[:12], identities, sid)
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        for text in ('ready_for_review', REPORT.name, 'claude/c01-jp-31', '52b23d59',
                     'test_japan_komeito_representatives_c01_31.py'):
            self.assertIn(text, handoff)
        index = research.build()
        country = next(p for p in index['countries'] if p['nation'] == 'Japan')
        self.assertFalse(country['country_census_complete'])
        self.assertIsNone(country['unrepresented_organization_count'])
        self.assertEqual((country['institution_observations'], country['role_observations']), (8, 7))
        self.assertFalse(index['c01_complete'])


if __name__ == '__main__':
    unittest.main()
