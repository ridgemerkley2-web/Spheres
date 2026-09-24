"""CLAUDE-C01-11: India's prime ministers, 1990-2026, keep the President's appointment, the oath, stated effective dates,
confidence votes, resignations, acceptances, requests to continue and continuation in office apart, and state a start or
an end only where a source does."""
import copy
import hashlib
import json
import re
import unittest
from urllib.parse import urlsplit

import campaign_research as research


# Original response identity recorded in each extract: (bytes, sha256). Every new source is reproducible.
RESPONSES = {
    'in_ls_debates_19900312':
        (1043511, '57751214004313157ed91bd93044a7078f98479f750906cb3617f0c7315ab51b'),
    'in_ls_debates_19901107':
        (6878854, '1a9a7964c8a8b3df0e2f0aa287eb48851b4d2a9a787e6e4da4d5e23d487467a9'),
    'in_pmo_former_pm_vp_singh':
        (49150, '644d8d885f49c06598f7017028eee441c491f26888f8ab5b78d6e6ecc5c01cdf'),
    'in_ls_debates_19901116':
        (6599374, '49f3e736cf37f72c6f962cf0ff8f0d66f31be706fedb6aa1676a1d3346d385c4'),
    'in_rs_debates_19901227':
        (125860, '69c9a4300dce2ee18c9eb7909274d3ac1595e2203117fe81196d51aed3bddb85'),
    'in_ls_debates_19910306':
        (18429324, '52d5806a84885d75c03d1dcb76a3d7843eba308f8306b2b6dab44c74665c08e8'),
    'in_rs_debates_19910306':
        (82642, 'b5dce7e89d67c14a01190ede9e48ac91f1eed05921265ccab2d6ba57b9f1c11f'),
    'in_rs_papers_19910307':
        (11469, '6899ed53814df00ea4834c5db2630b867ee6114c0eed1b648bb287b33ea71410'),
    'in_ls_debates_19910311':
        (6588530, '32eddc3a7648376194e2e628dce74e2a57c7ea138d90f453c18611325d882559'),
    'in_rs_debates_19910604':
        (365671, '4e7b007de8b1941567bd50c4cb018974ec2e9acfa75676cc1856518c1109bbda'),
    'in_gazette_ext_p1s2_no15_19910705':
        (66039, 'c9c8e5c885d2da365d4412dc4b4d1c9902fdba59bf88370b3713ea8ca7297099'),
    'in_pmo_former_pm_chandra_shekhar':
        (50298, '7338776db01c2e96bc082c91766a7759e1b479a1d9c24e6e4e53e6e422ac2bee'),
    'in_gazette_ext_p1s2_no17_19910705':
        (100967, '4f7497fb3613d0344ebe1acfa483b43578562c93b4aa7f35ac8c5066f3b7246d'),
    'in_gazette_ext_p1s2_no14_19960524':
        (96678, '2dcf9b380eb6d52b231de786f483a99ed0159a8399a5b25b9ee057a57364465b'),
    'in_ls_debates_19960528_printed':
        (3318468, 'ee5d24c029fb9c4cb9b1d20cc5051d51fccba21559473956a9d54d1063c85ed6'),
    'in_gazette_ext_p1s2_no13_19960524':
        (50883, '735caffcde49208ca63d3429e71f5ad98cd2a34eb054a529e3dc5c8497f2080c'),
    'in_ls_debates_19960523_council_introduction':
        (3769, 'f4c261b43f1ba3a3a94d7e2d7871731ec43d2ef85f4bd49f74d7c2a194265f9e'),
    'in_ls_contents_19960527':
        (5013, 'ad9c75ed9f62e07bd09e3d276332c44572f975155dcfbc1d20c96a5429eb5258'),
    'in_ls_debates_19960527_confidence_debate':
        (20037, 'ff6efff9ed3a28a81d4a82763c6cb56bd8c617b7cb4202554730e52517242344'),
    'in_ls_debates_19960528_confidence_debate_close':
        (17497, '805dac2b04212814de1fd408a9745314e1f0511a52ed6e37fad993beebd7e654'),
    'in_ls_debates_19960611_pm_introduction':
        (2436, 'f1f9b8b4bac17c7c6315c96795fb0a4700f8e778ac2789880c79f845824ecbaf'),
    'in_ls_debates_19960611_confidence_debate':
        (20705, 'c8616a3a4750946ebf3aded2366792760f6857a2ad4a04769565933560d40f40'),
    'in_pmo_former_pm_vajpayee_1996':
        (50896, '1c6a8478aa8d419c9a02935f2225192e91ba2e141ebfb346dcf4e0d9c9f7d716'),
    'in_pmo_former_pm_deve_gowda':
        (53596, '6fa4edf635d4f08851fb503fa19bb45f93e9e22e3b7e45a5c7d70b36deedee42'),
    'in_ls_contents_19970411':
        (3331, '0e7a97297fd8f933f74d417c8323f044a1911c6a525dcc7d854bafbc7b552bcd'),
    'in_ls_debates_19970411_confidence_vote':
        (18288, '564eb8f1f5027322a3f1d4c40d41aa55f13a5f79c259b0f839287e9b78f6b41f'),
    'in_ls_debates_19970421_pm_introduction':
        (1710, '14a2eb14897b1d09b4d1c56d3dfd7aecada42ca99373b94b4216869a976e2ed2'),
    'in_ls_debates_19970422_resignation_papers':
        (770, '578786535c6ae1900be9d6370dc93f451d9e37159b1cc92c68eb158b85d04bf6'),
    'in_gazette_ext_p1s2_no13_19970429':
        (119649, 'e911c1de3967aac09aeb9e410901d6bab77186ff9a0696e966816fac687b23ab'),
    'in_ls_debates_19971202_resignation_papers':
        (1490, 'eec9da5d0607c1c7c036ab9f1e9ea09a9e4eed7e9814249c53ca53117871dcba'),
    'in_pib_19980312_releases':
        (7178, '76e2c5de2ada163f0a16665f787fcda6631dc1bb315f313a5b62d59d861b3598'),
    'in_pmo_former_pm_gujral':
        (52144, '7e4774192ffbf2bd547083193a12ad2f0c2ab557b43f7c59ad92315b37d04263'),
    'in_gazette_ext_p1s2_no28_19980814':
        (43121, '909711304306ca7f5bd13b8d17e0cbef2cb458db3c04cbb163569a5776432333'),
    'in_pib_19980316_releases':
        (10956, 'ed540ddb523cc2d275911967efb4dca700a1a298b9e85649503bad71b8631c0c'),
    'in_pib_19980319_releases':
        (10344, 'bf038a4d633de34c7410204c4d3847954aac210bc2d847fc86f2ea0d874c4bb7'),
    'in_pib_photo_19980319_oath':
        (2279, 'a470249326b0cab0e1575e50d4af568197c2aa38f4c3b6b4747ded660c761066'),
    'in_gazette_ext_p1s2_no6_19980406':
        (125176, '48f8a0c2857614970232c1582ed31f7a28efbef27b7dab86bd688fea91d898b5'),
    'in_ls_debates_19990417_confidence_vote':
        (5436, '1e2e509fb84117abe9a94cb87978800e3512d3198bc2ba494ad65527e5c9bbf5'),
    'in_ls_debates_19990419_resignation_papers':
        (1221, '1f18018d1d82af80ffdf6708a80440c6b6cc3f37033779f76063b5258fab7cae'),
    'in_pib_photo_19990815_red_fort':
        (1812, '666f9627142ce72f076d608d1d9cf3272f0024ef95e2f30c6c31ee0113d1bd5b'),
    'in_pib_photo_19991013_oath':
        (1857, '9dadd19ca18d11dc6299722702a59d23e2f4fbda3ebfc0213c077f816c1dc606'),
    'in_pib_photo_19991013_council':
        (1863, '27dbd01e28d5088deee161d2af9eed2a8b7d880c4760b3aa278b35345ad941cc'),
    'in_pib_19991013_vajpayee_profile':
        (16058, '19b0426323fb7084006e78608ac21fafb3609b730c5b6730e3326bf3727b4c01'),
    'in_gazette_ext_p1s2_no16_19991026':
        (188787, 'a33a2822288c7ff04deec15ae1105ea4f4482cececdeaf351261f4467124189a'),
    'in_pmo_former_pm_vajpayee_1998':
        (50761, 'cc51d96e45eb5020e0fd4d203b564266cd7590e580d258acb61458a76b90f86a'),
    'in_rb_kalam_pm_resignation_accepted_20040513':
        (16811, '578d03405ea6ae694b7641c83127a2de344551d63febb90fc4850f0b2edce643'),
    'in_pib_president_communique_vajpayee_20040513':
        (45914, '23af7200f26d3284a04dec252cf9f58683bdbb8b5c1887355a66b7e0c002d37d'),
    'in_pib_pmo_vajpayee_address_to_nation_20040513':
        (25343, 'dd194bbe7ba79271510b93dab3fc90326548c58a40d4a26cd1e69b2e977cd6ca'),
    'in_pib_president_communique_council_20040522':
        (51513, 'b65478a123744e88636ab833679e0b406b09833ee82cc59ee3cc5d2ca57201f8'),
    'in_pmo_former_prime_ministers_page':
        (14207, '9c191ce2b6aaa01e6fa7f1d9ba94141bd75f4a0f4fd9000a06c9a69d2317f69d'),
    'in_rb_patil_pm_resignation_20090518':
        (3027, '355fdbc71d9213bbeb704f23e72d97328688e3cdc6fd03873d06dbd58e0df225'),
    'in_rb_patil_appoints_mms_20090520':
        (3751, '289ef715b8cee86eb9305f0c8b7421517c5759a2d745d48c7edf35180d3ce6a0'),
    'in_rb_patil_council_oath_20090522':
        (3889, '7d28e8f2438f8a4c2456804d4aff4f7530fe532f468f075cbdacd83886bb563a'),
    'in_rb_pranab_pm_resignation_20140517':
        (5669, '748d3ceb518d6efd220db355e3135447ab43601f95fe85847c99662f3563d37e'),
    'in_rb_pranab_appoints_modi_20140520':
        (6626, '9c5c4cfd89acc0db014c2f15da9ab07b527a510e7ae53f57873684f69ba70a69'),
    'in_rb_pranab_council_oath_20140526':
        (8329, '3d04a0f4a1dcc5e3be9f52948240ba3d95aa80dbc74399a429503d473978a7fb'),
    'in_cabsec_modi_sworn_in_20140526':
        (93288, '8deb95c028cdd7045db7e1d454b52dd699f4b9ca1abe0639417a4bd4639f9255'),
    'in_rb_kovind_pm_resignation_20190524':
        (60746, 'fcf4df3d1c5eed8ea7766a4840dbcb3691639a4b5ac35df96177050697031224'),
    'in_rb_kovind_appoints_modi_20190525':
        (63689, 'c76eaf2adfa1b2fe990fc07bc86000ea9e7a26c25928ded82b3df382373a0be9'),
    'in_rb_kovind_oath_schedule_20190526':
        (60735, 'cadbecef6609fd23e6d25d538213bbbb63f0b04e3df0b42dc3cbaff86a88c38c'),
    'in_rb_kovind_council_oath_20190530':
        (77506, '423f780579d52ceb3d7ec3584d264f71bd7a1538da5e8a2f6e6084f920da73fb'),
    'in_cabsec_modi_sworn_in_20190530':
        (963210, '01af62f34e6d06c5b42d3f43d62d9ade756e1a1248d56b149039bab6884ddadf'),
    'in_rb_murmu_pm_resignation_20240605':
        (32505, 'a86f194b9f200277fdd91ef53c96f572859ca21d3e4c5f4d56aed0596245ee88'),
    'in_rb_murmu_appoints_modi_20240607':
        (43211, 'c2ce4bd3f6e6481e626d84734e75118640b7e58bc13ab3ad9e309c2409b25cc4'),
    'in_rb_murmu_oath_schedule_20240607':
        (40937, '0627952fbe6342fa08afd5af61d362a46a588ca55dec7d84c6aab6cb6d3a8456'),
    'in_rb_murmu_council_oath_20240609':
        (43520, '0210454a3b5317f7780eefc6e73807225d4952b2fcd704704374a9799f7352d3'),
    'in_cabsec_modi_sworn_in_20240609':
        (1515782, '1d45c3c0dd8ae413203069775a8bb3e4b4d2a00720c847939b814c247ea11bf8'),
    'in_cabsec_council_list_20260623':
        (208003, '13df919e710e686f84966070428d71a3b05424dcaaffb3024dcb2c93df45f0e1'),
    'in_cabsec_council_list_20260725':
        (2037089, '9b3ea4fc75cfe3c814afce5e11641697062a912b0f484cc0abad1380f8811371'),
    'in_pmo_uzbekistan_joint_statement_20260830':
        (19526, '17a56d1e9f8b060af7ec309bbdc895a4af981747dd4ef5a21a383a78fe239ab2'),
}
# Raw Internet Archive captures (id_ form), all made before the cutoff: source id -> capture timestamp.
ARCHIVED = {
    'in_ls_debates_19900312': '20210519104239',
    'in_ls_debates_19901107': '20240928132810',
    'in_pmo_former_pm_vp_singh': '20251107120348',
    'in_pmo_former_pm_chandra_shekhar': '20260411232705',
    'in_ls_debates_19960528_printed': '20250513155724',
    'in_ls_debates_19960523_council_introduction': '20060517033808',
    'in_ls_contents_19960527': '20041122171828',
    'in_ls_debates_19960527_confidence_debate': '20060518161238',
    'in_ls_debates_19960528_confidence_debate_close': '20090411041412',
    'in_ls_debates_19960611_pm_introduction': '20060519082009',
    'in_ls_debates_19960611_confidence_debate': '20060519082442',
    'in_pmo_former_pm_vajpayee_1996': '20251107123027',
    'in_pmo_former_pm_deve_gowda': '20260515014304',
    'in_ls_contents_19970411': '20041027141913',
    'in_ls_debates_19970411_confidence_vote': '20090411041101',
    'in_ls_debates_19970421_pm_introduction': '20090411022014',
    'in_ls_debates_19970422_resignation_papers': '20041105025819',
    'in_ls_debates_19971202_resignation_papers': '20060517010213',
    'in_pib_19980312_releases': '20020311012706',
    'in_pmo_former_pm_gujral': '20251107114706',
    'in_pib_19980316_releases': '20010306071634',
    'in_pib_19980319_releases': '20020627214656',
    'in_pib_photo_19980319_oath': '20030108123604',
    'in_ls_debates_19990417_confidence_vote': '20041123180823',
    'in_ls_debates_19990419_resignation_papers': '20041123181707',
    'in_pib_photo_19990815_red_fort': '20020401001057',
    'in_pib_photo_19991013_oath': '20020401115605',
    'in_pib_photo_19991013_council': '20020401114524',
    'in_pib_19991013_vajpayee_profile': '20020316161744',
    'in_pmo_former_pm_vajpayee_1998': '20251107115937',
    'in_rb_kalam_pm_resignation_accepted_20040513': '20040629140504',
    'in_pib_president_communique_vajpayee_20040513': '20140328131705',
    'in_pib_pmo_vajpayee_address_to_nation_20040513': '20040819231551',
    'in_pib_president_communique_council_20040522': '20040811175902',
    'in_pmo_former_prime_ministers_page': '20260221002548',
    'in_rb_patil_pm_resignation_20090518': '20090619063954',
    'in_rb_patil_appoints_mms_20090520': '20090619063712',
    'in_rb_patil_council_oath_20090522': '20090619063144',
    'in_rb_pranab_pm_resignation_20140517': '20140520100258',
    'in_rb_pranab_appoints_modi_20140520': '20140523042849',
    'in_rb_pranab_council_oath_20140526': '20140529084744',
    'in_rb_kovind_pm_resignation_20190524': '20190529132506',
    'in_rb_kovind_appoints_modi_20190525': '20190716083258',
    'in_rb_kovind_oath_schedule_20190526': '20190716082852',
    'in_rb_kovind_council_oath_20190530': '20190716082409',
    'in_rb_murmu_pm_resignation_20240605': '20240605153208',
    'in_rb_murmu_appoints_modi_20240607': '20240704043120',
    'in_rb_murmu_oath_schedule_20240607': '20240703213724',
    'in_rb_murmu_council_oath_20240609': '20240609192532',
    'in_pmo_uzbekistan_joint_statement_20260830': '20260902065902',
}
# Stored files in Internet Archive items that mirror an official document (not Wayback captures).
MIRRORS = ('in_ls_debates_19901116',
           'in_ls_debates_19910306',
           'in_ls_debates_19910311',
           'in_gazette_ext_p1s2_no28_19980814',
           'in_gazette_ext_p1s2_no6_19980406',
           'in_gazette_ext_p1s2_no16_19991026')
PDF_PAGES = {
    'in_ls_debates_19900312': [1, 28],
    'in_ls_debates_19901107': [27, 36, 133, 144],
    'in_ls_debates_19901116': [33, 98, 107],
    'in_rs_debates_19901227': [1],
    'in_ls_debates_19910306': [405, 406, 410],
    'in_rs_debates_19910306': [1, 2],
    'in_rs_papers_19910307': [1],
    'in_ls_debates_19910311': [33],
    'in_rs_debates_19910604': [3],
    'in_gazette_ext_p1s2_no15_19910705': [1, 2],
    'in_gazette_ext_p1s2_no17_19910705': [1, 2],
    'in_gazette_ext_p1s2_no14_19960524': [1, 2, 3],
    'in_ls_debates_19960528_printed': [9],
    'in_gazette_ext_p1s2_no13_19960524': [1, 2],
    'in_gazette_ext_p1s2_no13_19970429': [1, 2, 3],
    'in_gazette_ext_p1s2_no28_19980814': [1],
    'in_gazette_ext_p1s2_no6_19980406': [1, 2, 3],
    'in_gazette_ext_p1s2_no16_19991026': [1, 3, 5],
    'in_cabsec_modi_sworn_in_20140526': [1, 2],
    'in_cabsec_modi_sworn_in_20190530': [1],
    'in_cabsec_modi_sworn_in_20240609': [1, 2],
    'in_cabsec_council_list_20260623': [1],
    'in_cabsec_council_list_20260725': [1],
}
# First accessed on 24 September 2026; every other new source on 23 September 2026.
ACCESSED_20260924 = ('in_gazette_ext_p1s2_no13_19960524',
                     'in_gazette_ext_p1s2_no13_19970429',
                     'in_pib_president_communique_vajpayee_20040513',
                     'in_rb_kovind_pm_resignation_20190524',
                     'in_rb_kovind_appoints_modi_20190525',
                     'in_rb_kovind_oath_schedule_20190526',
                     'in_rb_kovind_council_oath_20190530',
                     'in_cabsec_council_list_20260725')
# Contents pages that date an undated continuation page: (capture, bytes, sha256).
DATE_ANCHORS = {
    'in_ls_debates_19960528_confidence_debate_close':
        ('20041122171832', 3095, '31c017d830fbf23d2382d5f082ac0888d2777c5dff27685c98f95e65950742be'),
    'in_ls_debates_19960611_confidence_debate':
        ('20041122171807', 4377, 'f0e218e283af6c6912267ac26252c58ab229f41bef965dda5a2c74f1150f93c2'),
    'in_ls_debates_19990417_confidence_vote':
        ('20041028165646', 937, '4b1c1bb87d50b29c2c3d29ef7a6b7c36068ec84975b9e6379540ca68e5dc0b81'),
}
# Captures the Internet Archive serves gzip-encoded: decoded (bytes, sha256) recorded beside the encoded identity.
GZIP = {
    'in_pmo_former_prime_ministers_page':
        (71386, '71dac5225f5f880004db505d69feff270ae1f580fafa9e8325de418605ea94ef'),
    'in_pmo_uzbekistan_joint_statement_20260830':
        (81083, '4a07b0f399e7fb65f74634661bdd1c5c4bd478a72c481e322c98deb6d7d87884'),
}
# Every new claim's (attested_on, event_kind, review observation), exactly: distinct dated events are never re-dated,
# relabelled or moved to another observation.
EVENTS = {
    'in_vp_singh_pm_introduces_minister_19900312': ('1990-03-12', 'in_office_attestation', 'IN-PM-01'),
    'in_vp_singh_moves_confidence_motion_19901107': ('1990-11-07', 'confidence_motion_moved', 'IN-PM-01'),
    'in_ls_confidence_motion_negatived_19901107': ('1990-11-07', 'confidence_vote_lost', 'IN-PM-01'),
    'in_pmo_span_vp_singh_19891202_19901110': (None, 'retrospective_term_span', 'IN-PM-01'),
    'in_chandra_shekhar_moves_confidence_motion_19901116': ('1990-11-16', 'confidence_motion_moved', 'IN-PM-02'),
    'in_ls_confidence_motion_adopted_19901116': ('1990-11-16', 'confidence_vote_won', 'IN-PM-02'),
    'in_chandra_shekhar_introduces_ministers_rs_19901227': ('1990-12-27', 'in_office_attestation', 'IN-PM-02'),
    'in_chandra_shekhar_announces_resignation_19910306': ('1991-03-06', 'resignation_announced', 'IN-PM-02'),
    'in_rs_leader_announces_pm_resignation_19910306': ('1991-03-06', 'resignation_announced', 'IN-PM-02'),
    'in_rs_lays_chandra_shekhar_resignation_letter_19910306': ('1991-03-06', 'resignation_tendered', 'IN-PM-02'),
    'in_rs_lays_president_acceptance_19910306': ('1991-03-06', 'resignation_accepted', 'IN-PM-02'),
    'in_rs_lays_continuation_request_19910306': ('1991-03-06', 'continuation_requested', 'IN-PM-02'),
    'in_chandra_shekhar_resignation_letter_19910306': ('1991-03-06', 'resignation_tendered', 'IN-PM-02'),
    'in_president_accepts_chandra_shekhar_resignation_19910306': ('1991-03-06', 'resignation_accepted', 'IN-PM-02'),
    'in_president_requests_chandra_shekhar_continue_19910306': ('1991-03-06', 'continuation_requested', 'IN-PM-02'),
    'in_chandra_shekhar_pm_in_rajya_sabha_19910604': ('1991-06-04', 'continuation_in_office_attestation', 'IN-PM-02'),
    'in_president_accepts_chandra_shekhar_resignation_wef_19910621': ('1991-06-21', 'resignation_accepted_with_effect', 'IN-PM-02'),
    'in_pmo_span_chandra_shekhar_19901110_19910621': (None, 'retrospective_term_span', 'IN-PM-02'),
    'in_rao_appointed_pm_wef_19910621': ('1991-06-21', 'appointment_with_effect', 'IN-PM-03'),
    'in_president_accepts_rao_resignation_wef_19960516': ('1996-05-16', 'resignation_accepted_with_effect', 'IN-PM-03'),
    'in_rao_swearing_in_recalled_by_speaker_19910621': ('1991-06-21', 'swearing_in_recalled_retrospective', 'IN-PM-03'),
    'in_vajpayee_appointed_pm_wef_19960516': ('1996-05-16', 'appointment_with_effect', 'IN-PM-04'),
    'in_vajpayee_introduces_council_as_pm_19960523': ('1996-05-23', 'in_office_attestation', 'IN-PM-04'),
    'in_vajpayee_moves_confidence_motion_19960527': ('1996-05-27', 'confidence_motion_moved', 'IN-PM-04'),
    'in_vajpayee_swearing_in_recalled_in_house_19960516': ('1996-05-16', 'swearing_in_recalled_in_debate', 'IN-PM-04'),
    'in_vajpayee_resignation_announced_in_house_19960528': ('1996-05-28', 'resignation_announced', 'IN-PM-04'),
    'in_vajpayee_confidence_motion_not_put_to_vote_19960528': ('1996-05-28', 'confidence_motion_not_put_to_vote', 'IN-PM-04'),
    'in_deve_gowda_introduced_as_pm_19960611': ('1996-06-11', 'in_office_attestation', 'IN-PM-04'),
    'in_deve_gowda_swearing_in_recalled_in_house_19960601': ('1996-06-01', 'swearing_in_recalled_in_debate', 'IN-PM-04'),
    'in_pmo_span_vajpayee_19960516_19960601': (None, 'retrospective_term_span', 'IN-PM-04'),
    'in_pmo_span_deve_gowda_19960601_19970421': (None, 'retrospective_term_span', 'IN-PM-04'),
    'in_deve_gowda_moves_confidence_motion_19970411': ('1997-04-11', 'confidence_motion_moved', 'IN-PM-05'),
    'in_deve_gowda_confidence_motion_negatived_19970411': ('1997-04-11', 'confidence_vote_lost', 'IN-PM-05'),
    'in_gujral_swearing_in_concluded_19970421': ('1997-04-21', 'swearing_in_ceremony_reported', 'IN-PM-05'),
    'in_gujral_introduced_as_pm_19970421': ('1997-04-21', 'in_office_attestation', 'IN-PM-05'),
    'in_deve_gowda_resignation_tendered_19970411': ('1997-04-11', 'resignation_tendered', 'IN-PM-05'),
    'in_president_accepts_deve_gowda_resignation_19970412': ('1997-04-12', 'resignation_accepted', 'IN-PM-05'),
    'in_deve_gowda_requested_to_continue_19970412': ('1997-04-12', 'continuation_requested', 'IN-PM-05'),
    'in_president_accepts_deve_gowda_resignation_wef_19970421': ('1997-04-21', 'resignation_accepted_with_effect', 'IN-PM-05'),
    'in_gujral_appointed_pm_wef_19970421': ('1997-04-21', 'appointment_with_effect', 'IN-PM-05'),
    'in_gujral_resignation_tendered_19971128': ('1997-11-28', 'resignation_tendered', 'IN-PM-05'),
    'in_president_accepts_gujral_resignation_19971128': ('1997-11-28', 'resignation_accepted', 'IN-PM-05'),
    'in_gujral_requested_to_continue_19971128': ('1997-11-28', 'continuation_requested', 'IN-PM-05'),
    'in_gujral_styled_pm_holi_greetings_19980312': ('1998-03-12', 'continuation_in_office_attestation', 'IN-PM-05'),
    'in_pmo_gujral_swearing_in_recalled_19970421': ('1997-04-21', 'swearing_in_recalled_retrospective', 'IN-PM-05'),
    'in_pmo_span_gujral_19970421_19980319': (None, 'retrospective_term_span', 'IN-PM-05'),
    'in_gazette_corrigendum_gujral_acceptance_19980814': ('1998-08-14', 'resignation_acceptance_corrigendum', 'IN-PM-05'),
    'in_president_invites_vajpayee_to_form_government_19980316': ('1998-03-16', 'invitation_to_form_government', 'IN-PM-06'),
    'in_pib_profile_vajpayee_1996_span_19980316': (None, 'retrospective_term_span', 'IN-PM-04'),
    'in_vajpayee_sworn_in_19980319': ('1998-03-19', 'oath_of_office', 'IN-PM-06'),
    'in_vajpayee_heads_council_list_19980319': ('1998-03-19', 'in_office_attestation', 'IN-PM-06'),
    'in_pib_profile_vajpayee_1996_span_19980319': (None, 'retrospective_term_span', 'IN-PM-04'),
    'in_vajpayee_oath_of_office_19980319': ('1998-03-19', 'oath_of_office', 'IN-PM-06'),
    'in_gujral_resignation_accepted_wef_superseded_19980406': (None, 'resignation_accepted_with_effect_superseded', 'IN-PM-05'),
    'in_vajpayee_appointed_pm_wef_19980319': ('1998-03-19', 'appointment_with_effect', 'IN-PM-06'),
    'in_vajpayee_confidence_motion_negatived_19990417': ('1999-04-17', 'confidence_vote_lost', 'IN-PM-06'),
    'in_vajpayee_resignation_tendered_19990417': ('1999-04-17', 'resignation_tendered', 'IN-PM-06'),
    'in_president_accepts_vajpayee_resignation_19990417': ('1999-04-17', 'resignation_accepted', 'IN-PM-06'),
    'in_vajpayee_requested_to_continue_19990417': ('1999-04-17', 'continuation_requested', 'IN-PM-06'),
    'in_vajpayee_styled_pm_independence_day_19990815': ('1999-08-15', 'continuation_in_office_attestation', 'IN-PM-06'),
    'in_vajpayee_oath_of_office_19991013': ('1999-10-13', 'oath_of_office', 'IN-PM-06'),
    'in_vajpayee_with_new_council_19991013': ('1999-10-13', 'in_office_attestation', 'IN-PM-06'),
    'in_vajpayee_sworn_in_19991013': ('1999-10-13', 'oath_of_office', 'IN-PM-06'),
    'in_vajpayee_assumes_office_statement_19991013': ('1999-10-13', 'assumption_statement', 'IN-PM-06'),
    'in_pib_profile_vajpayee_spans_19991013': (None, 'retrospective_term_span', 'IN-PM-06'),
    'in_vajpayee_appointed_pm_wef_19991013': ('1999-10-13', 'appointment_with_effect', 'IN-PM-06'),
    'in_president_accepts_vajpayee_resignation_wef_19991013': ('1999-10-13', 'resignation_accepted_with_effect', 'IN-PM-06'),
    'in_pmo_vajpayee_took_charge_recalled_19991013': ('1999-10-13', 'assumption_recalled_retrospective', 'IN-PM-06'),
    'in_pmo_span_vajpayee_19980319_20040522': (None, 'retrospective_term_span', 'IN-PM-06'),
    'in_vajpayee_tenders_resignation_20040513': ('2004-05-13', 'resignation_tendered', 'IN-PM-07'),
    'in_kalam_accepts_vajpayee_resignation_20040513': ('2004-05-13', 'resignation_accepted', 'IN-PM-07'),
    'in_vajpayee_requested_to_continue_20040513': ('2004-05-13', 'continuation_requested', 'IN-PM-07'),
    'in_pib_releases_president_communique_20040513': ('2004-05-13', 'communique_released', 'IN-PM-07'),
    'in_vajpayee_states_resignation_submitted_20040513': ('2004-05-13', 'resignation_statement', 'IN-PM-07'),
    'in_mms_appointed_pm_communique_20040522': ('2004-05-22', 'appointment_communique', 'IN-PM-07'),
    'in_council_oath_administered_20040522': ('2004-05-22', 'council_oath_administered', 'IN-PM-07'),
    'in_pmo_list_span_vajpayee_19980319_20040522': (None, 'retrospective_term_span', 'IN-PM-07'),
    'in_pmo_list_span_mms_20040522_20140526': (None, 'retrospective_term_span', 'IN-PM-07'),
    'in_mms_tenders_resignation_20090518': ('2009-05-18', 'resignation_tendered', 'IN-PM-08'),
    'in_patil_accepts_mms_resignation_20090518': ('2009-05-18', 'resignation_accepted', 'IN-PM-08'),
    'in_mms_requested_to_continue_20090518': ('2009-05-18', 'continuation_requested', 'IN-PM-08'),
    'in_patil_appoints_mms_pm_20090520': ('2009-05-20', 'appointment', 'IN-PM-08'),
    'in_mms_oath_announced_20090520': ('2009-05-20', 'oath_scheduled', 'IN-PM-08'),
    'in_mms_appointed_pm_communique_20090522': ('2009-05-22', 'appointment_communique', 'IN-PM-08'),
    'in_council_oath_administered_20090522': ('2009-05-22', 'council_oath_administered', 'IN-PM-08'),
    'in_mms_tenders_resignation_20140517': ('2014-05-17', 'resignation_tendered', 'IN-PM-09'),
    'in_pranab_accepts_mms_resignation_20140517': ('2014-05-17', 'resignation_accepted', 'IN-PM-09'),
    'in_mms_requested_to_continue_20140517': ('2014-05-17', 'continuation_requested', 'IN-PM-09'),
    'in_pranab_appoints_modi_pm_20140520': ('2014-05-20', 'appointment', 'IN-PM-09'),
    'in_modi_oath_announced_20140520': ('2014-05-20', 'oath_scheduled', 'IN-PM-09'),
    'in_modi_appointed_pm_communique_20140526': ('2014-05-26', 'appointment_communique', 'IN-PM-09'),
    'in_council_oath_administered_20140526': ('2014-05-26', 'council_oath_administered', 'IN-PM-09'),
    'in_cabsec_modi_sworn_in_as_pm_20140526': ('2014-05-26', 'oath_of_office', 'IN-PM-09'),
    'in_modi_tenders_resignation_20190524': ('2019-05-24', 'resignation_tendered', 'IN-PM-09'),
    'in_kovind_accepts_modi_resignation_20190524': ('2019-05-24', 'resignation_accepted', 'IN-PM-09'),
    'in_modi_requested_to_continue_20190524': ('2019-05-24', 'continuation_requested', 'IN-PM-09'),
    'in_kovind_appoints_modi_pm_20190525': ('2019-05-25', 'appointment', 'IN-PM-09'),
    'in_modi_oath_announced_20190526': ('2019-05-26', 'oath_scheduled', 'IN-PM-09'),
    'in_modi_appointed_pm_communique_20190530': ('2019-05-30', 'appointment_communique', 'IN-PM-09'),
    'in_modi_oath_administered_20190530': ('2019-05-30', 'oath_of_office', 'IN-PM-09'),
    'in_cabsec_modi_sworn_in_as_pm_20190530': ('2019-05-30', 'oath_of_office', 'IN-PM-09'),
    'in_modi_tenders_resignation_20240605': ('2024-06-05', 'resignation_tendered', 'IN-PM-10'),
    'in_murmu_accepts_modi_resignation_20240605': ('2024-06-05', 'resignation_accepted', 'IN-PM-10'),
    'in_modi_requested_to_continue_20240605': ('2024-06-05', 'continuation_requested', 'IN-PM-10'),
    'in_murmu_appoints_modi_pm_20240607': ('2024-06-07', 'appointment', 'IN-PM-10'),
    'in_modi_oath_announced_20240607': ('2024-06-07', 'oath_scheduled', 'IN-PM-10'),
    'in_modi_appointed_pm_communique_20240609': ('2024-06-09', 'appointment_communique', 'IN-PM-10'),
    'in_council_oath_administered_20240609': ('2024-06-09', 'council_oath_administered', 'IN-PM-10'),
    'in_cabsec_modi_sworn_in_as_pm_20240609': ('2024-06-09', 'oath_of_office', 'IN-PM-10'),
    'in_cabsec_lists_modi_pm_20260623': ('2026-06-23', 'in_office_attestation', 'IN-PM-10'),
    'in_cabsec_lists_modi_pm_20260725': ('2026-07-25', 'in_office_attestation', 'IN-PM-10'),
    'in_modi_pm_state_visit_uzbekistan_20260830': ('2026-08-30', 'in_office_attestation', 'IN-PM-10'),
}
NEW_SOURCES = list(RESPONSES)
ORIGINAL_SOURCES = ('in_eci_national_parties_20240323', 'in_eci_state_parties_20240323')
PM = 'in_pm'
T_PM = 'Prime Minister of India'
OFFICIAL_HOSTS = {'egazette.gov.in', 'bucketapi.rajyasabha.digital', 'cabsec.gov.in'}

# Exact holder observations of in_pm: (name, attested_on, from, until), in chronological order.
HOLDERS = [
    ('Vishwanath Pratap Singh', '1990-03-12', None, None),
    ('Chandra Shekhar', '1990-11-16', None, '1991-06-21'),
    ('P. V. Narasimha Rao', None, '1991-06-21', '1996-05-16'),
    ('Atal Bihari Vajpayee', None, '1996-05-16', None),
    ('H. D. Deve Gowda', '1996-06-11', None, '1997-04-21'),
    ('I. K. Gujral', None, '1997-04-21', None),
    ('Atal Bihari Vajpayee', None, '1998-03-19', '1999-10-13'),
    ('Atal Bihari Vajpayee', None, '1999-10-13', None),
    ('Manmohan Singh', '2004-05-22', None, None),
    ('Manmohan Singh', '2009-05-22', None, None),
    ('Narendra Modi', '2014-05-26', None, None),
    ('Narendra Modi', '2019-05-30', None, None),
    ('Narendra Modi', '2024-06-09', None, None),
]
HOLDER_CLAIMS = [
    ['in_vp_singh_pm_introduces_minister_19900312'],
    ['in_chandra_shekhar_moves_confidence_motion_19901116', 'in_chandra_shekhar_introduces_ministers_rs_19901227',
     'in_president_accepts_chandra_shekhar_resignation_wef_19910621'],
    ['in_rao_appointed_pm_wef_19910621', 'in_president_accepts_rao_resignation_wef_19960516'],
    ['in_vajpayee_appointed_pm_wef_19960516', 'in_vajpayee_introduces_council_as_pm_19960523'],
    ['in_deve_gowda_introduced_as_pm_19960611', 'in_president_accepts_deve_gowda_resignation_wef_19970421'],
    ['in_gujral_appointed_pm_wef_19970421', 'in_gujral_introduced_as_pm_19970421'],
    ['in_vajpayee_appointed_pm_wef_19980319', 'in_vajpayee_sworn_in_19980319', 'in_vajpayee_oath_of_office_19980319',
     'in_vajpayee_heads_council_list_19980319', 'in_president_accepts_vajpayee_resignation_wef_19991013'],
    ['in_vajpayee_appointed_pm_wef_19991013', 'in_vajpayee_oath_of_office_19991013', 'in_vajpayee_with_new_council_19991013',
     'in_vajpayee_sworn_in_19991013', 'in_vajpayee_assumes_office_statement_19991013'],
    ['in_mms_appointed_pm_communique_20040522'],
    ['in_mms_appointed_pm_communique_20090522'],
    ['in_modi_appointed_pm_communique_20140526', 'in_cabsec_modi_sworn_in_as_pm_20140526'],
    ['in_modi_appointed_pm_communique_20190530', 'in_modi_oath_administered_20190530', 'in_cabsec_modi_sworn_in_as_pm_20190530'],
    ['in_modi_appointed_pm_communique_20240609', 'in_cabsec_modi_sworn_in_as_pm_20240609', 'in_cabsec_lists_modi_pm_20260623',
     'in_cabsec_lists_modi_pm_20260725', 'in_modi_pm_state_visit_uzbekistan_20260830'],
]
# Claims that must never feed a holder observation.
CONFIDENCE = (  # the vote and its outcome, and three motions kept as context before an unknown or unstated end
    'in_vp_singh_moves_confidence_motion_19901107', 'in_ls_confidence_motion_negatived_19901107',
    'in_ls_confidence_motion_adopted_19901116', 'in_vajpayee_moves_confidence_motion_19960527',
    'in_vajpayee_confidence_motion_not_put_to_vote_19960528', 'in_deve_gowda_moves_confidence_motion_19970411',
    'in_deve_gowda_confidence_motion_negatived_19970411', 'in_vajpayee_confidence_motion_negatived_19990417')
RESIGNATIONS = (  # announcements, letters, statements and acceptances that state no effective day
    'in_chandra_shekhar_announces_resignation_19910306', 'in_rs_leader_announces_pm_resignation_19910306',
    'in_rs_lays_chandra_shekhar_resignation_letter_19910306', 'in_rs_lays_president_acceptance_19910306',
    'in_chandra_shekhar_resignation_letter_19910306', 'in_president_accepts_chandra_shekhar_resignation_19910306',
    'in_vajpayee_resignation_announced_in_house_19960528', 'in_deve_gowda_resignation_tendered_19970411',
    'in_president_accepts_deve_gowda_resignation_19970412', 'in_gujral_resignation_tendered_19971128',
    'in_president_accepts_gujral_resignation_19971128', 'in_gujral_resignation_accepted_wef_superseded_19980406',
    'in_gazette_corrigendum_gujral_acceptance_19980814', 'in_vajpayee_resignation_tendered_19990417',
    'in_president_accepts_vajpayee_resignation_19990417', 'in_vajpayee_tenders_resignation_20040513',
    'in_kalam_accepts_vajpayee_resignation_20040513', 'in_pib_releases_president_communique_20040513',
    'in_vajpayee_states_resignation_submitted_20040513', 'in_mms_tenders_resignation_20090518',
    'in_patil_accepts_mms_resignation_20090518', 'in_mms_tenders_resignation_20140517',
    'in_pranab_accepts_mms_resignation_20140517', 'in_modi_tenders_resignation_20190524',
    'in_kovind_accepts_modi_resignation_20190524', 'in_modi_tenders_resignation_20240605',
    'in_murmu_accepts_modi_resignation_20240605')
CONTINUATION = (  # requests to continue and in-office attestations during a continuation
    'in_rs_lays_continuation_request_19910306', 'in_president_requests_chandra_shekhar_continue_19910306',
    'in_chandra_shekhar_pm_in_rajya_sabha_19910604', 'in_deve_gowda_requested_to_continue_19970412',
    'in_gujral_requested_to_continue_19971128', 'in_gujral_styled_pm_holi_greetings_19980312',
    'in_vajpayee_requested_to_continue_19990417', 'in_vajpayee_styled_pm_independence_day_19990815',
    'in_vajpayee_requested_to_continue_20040513', 'in_mms_requested_to_continue_20090518',
    'in_mms_requested_to_continue_20140517', 'in_modi_requested_to_continue_20190524',
    'in_modi_requested_to_continue_20240605')
RETROSPECTIVE = (  # PMO and PIB spans, and later recollections of a swearing-in or assumption
    'in_pmo_span_vp_singh_19891202_19901110', 'in_pmo_span_chandra_shekhar_19901110_19910621',
    'in_rao_swearing_in_recalled_by_speaker_19910621', 'in_vajpayee_swearing_in_recalled_in_house_19960516',
    'in_deve_gowda_swearing_in_recalled_in_house_19960601', 'in_pmo_span_vajpayee_19960516_19960601',
    'in_pmo_span_deve_gowda_19960601_19970421', 'in_pmo_gujral_swearing_in_recalled_19970421',
    'in_pmo_span_gujral_19970421_19980319', 'in_pib_profile_vajpayee_1996_span_19980316',
    'in_pib_profile_vajpayee_1996_span_19980319', 'in_pib_profile_vajpayee_spans_19991013',
    'in_pmo_vajpayee_took_charge_recalled_19991013', 'in_pmo_span_vajpayee_19980319_20040522',
    'in_pmo_list_span_vajpayee_19980319_20040522', 'in_pmo_list_span_mms_20040522_20140526')
PROSPECTIVE = (  # an invitation, the appointment of a Prime Minister-designate and oath announcements
    'in_president_invites_vajpayee_to_form_government_19980316', 'in_patil_appoints_mms_pm_20090520',
    'in_mms_oath_announced_20090520', 'in_pranab_appoints_modi_pm_20140520', 'in_modi_oath_announced_20140520',
    'in_kovind_appoints_modi_pm_20190525', 'in_modi_oath_announced_20190526', 'in_murmu_appoints_modi_pm_20240607',
    'in_modi_oath_announced_20240607')
COUNCIL_OATHS = ('in_council_oath_administered_20040522', 'in_council_oath_administered_20090522',
                 'in_council_oath_administered_20140526', 'in_council_oath_administered_20240609')
CEREMONY = ('in_gujral_swearing_in_concluded_19970421',)  # the Speaker's ceremony report names no one
NEVER_HOLDER = CONFIDENCE + RESIGNATIONS + CONTINUATION + RETROSPECTIVE + PROSPECTIVE + COUNCIL_OATHS + CEREMONY
SPANS = tuple(cid for cid in RETROSPECTIVE if '_span' in cid)
# Event kinds that may bound or date a holder; every other kind never does.
FROM_KINDS = {'appointment_with_effect', 'assumption_statement'}
UNTIL_KINDS = {'resignation_accepted_with_effect'}
ATTEST_KINDS = {'in_office_attestation', 'confidence_motion_moved', 'oath_of_office', 'appointment_communique'}
HOLDER_KINDS = FROM_KINDS | UNTIL_KINDS | ATTEST_KINDS
# Dates that are never any holder's attested_on, start or end: votes, resignations, acceptances without an effective
# day, requests and attestations during a continuation, invitations, appointments of a Prime Minister-designate, oath
# announcements, retrospective span ends, members' recollections and the Gazette publication days.
NEVER_HOLDER_DATE = {
    '1989-12-02', '1990-11-07', '1990-11-10', '1991-03-06', '1991-03-07', '1991-03-11', '1991-06-04', '1991-07-05',
    '1996-05-24', '1996-05-27', '1996-05-28', '1996-05-31', '1996-06-01', '1997-04-11', '1997-04-12', '1997-04-22',
    '1997-04-29', '1997-11-28', '1997-12-02', '1998-03-12', '1998-03-16', '1998-04-06', '1998-08-14', '1999-04-17',
    '1999-04-19', '1999-08-15', '1999-10-26', '2004-05-13', '2004-05-19', '2009-05-18', '2009-05-20', '2014-05-17',
    '2014-05-20', '2019-05-24', '2019-05-25', '2019-05-26', '2024-06-05', '2024-06-07'}
STARTS = [(h[0], h[2]) for h in HOLDERS if h[2]]
ENDS = [('Chandra Shekhar', '1991-06-21'), ('P. V. Narasimha Rao', '1996-05-16'), ('H. D. Deve Gowda', '1997-04-21'),
        ('Atal Bihari Vajpayee', '1999-10-13')]
SURNAMES = {'Vishwanath Pratap Singh': 'Vishwanath Pratap Singh', 'Chandra Shekhar': 'Chandra Shekhar',
            'P. V. Narasimha Rao': 'Narasimha Rao', 'Atal Bihari Vajpayee': 'Vajpayee', 'H. D. Deve Gowda': 'Deve Gowda',
            'I. K. Gujral': 'Gujral', 'Manmohan Singh': 'Manmohan Singh', 'Narendra Modi': 'Modi'}
# Leads, secondary sources and records the checks declined: never a source URL.
LEAD_URL_MARKERS = ('wikipedia', 'micro_IA1176910', 'micro_IA1176911', 'jpi_june_1996', 'lsd_09_06_27-12-1990',
                    'E-0575-1990-0014', 'E-0575-1990-0017', 'E-0575-1990-0018', 'narayanan_25_10_1999',
                    'lsd_11_01_28-05-1996_hindi', 'lsd_11_4_11-04-1997', 'PIBR170398', 'PIBR130198', 'c220497',
                    'c120696', '170499.html', 'r060599', 'r170499', 'r190499', 'r121099', '1310993', '1903982',
                    'archivepmo', 'cnn.com', 'deccanherald', 'newsonair', 'PressReleseDetail', 'relid=1734',
                    'relid=1703', 'relid=1743', 'prlatest1.jsp?id=242', 'press-communique-13', 'press-communique-14',
                    'press-communique-18', '1_Upload_3871', '1_Upload_4252', 'pr260519', 'pr250519.html',
                    'pms-address-from-the-ramparts', 'ramnathkovind')
# URL patterns of responses generated per request, cache-busting queries or session state: never a recorded identity.
PER_REQUEST_PATTERNS = ('cb=', 'reviewcb', 'form_build_id', 'token=', 'X-Amz-', 'Signature=', 'sessionid', 'ASPSESSION',
                        '__VIEWSTATE', 'seqPagina', 'PrintRelease', 'SearchMenu', 'error.aspx', 'ajax_page_data')
# Claim ids and field names withdrawn or renamed by the checks; none may remain in the packet.
STALE_IDS = ('in_chandra_shekhar_continuation_ends_19910621', 'attested_period', 'in_mms_oath_administered_20040522',
             'in_mms_oath_administered_20090522', 'in_modi_oath_administered_20140526', 'in_modi_oath_administered_20240609',
             'in_pmo_vajpayee_span_19980319_20040522', 'in_pmo_mms_span_20040522_20140526',
             'resignation_accepted_by_president', 'request_to_continue_in_office',
             'styled_prime_minister_during_continuation', 'retrospective_office_span', 'retrospective_tenure_statement',
             'continuation_ended')
# Event kinds of the three research parts that one vocabulary replaced; no row may use them.
OLD_KINDS = {'resignation_announced_in_house', 'introduced_as_prime_minister', 'swearing_in', 'oath',
             'introduced_council_of_ministers_as_prime_minister', 'council_of_ministers_list', 'resignation_accepted_by_president',
             'request_to_continue_in_office', 'styled_prime_minister_during_continuation', 'retrospective_office_span',
             'retrospective_tenure_statement', 'continuation_ended'}
REPORT = research.RESEARCH / 'india-prime-ministers-1990-2026-11.md'
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-11.md'


def load_rows():
    packet = json.loads((research.ROOT / research.RESEARCH / 'india.json').read_text(encoding='utf-8'))
    rows = {}
    for source in packet['sources'][len(ORIGINAL_SOURCES):]:
        extract = json.loads((research.ROOT / source['snapshot']['path']).read_text(encoding='utf-8'))
        rows.update({row['claim_id']: row for row in extract['rows']})
    return rows


def pm_rules(packet, rows):
    """Rule-based checks that hold without the pinned holder list; raise AssertionError, KeyError or IndexError."""
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    assert [i['id'] for i in packet['institutions']] == ['in_prime_minister'], 'exactly one prime-ministership institution'
    office = packet['institutions'][0]
    assert office['kind'] == 'executive_institution' and office['lifecycle']['status'] == 'unknown'
    assert [r['id'] for r in office['roles']] == [PM], 'exactly one role'
    role = office['roles'][0]
    assert (role['title'], role['kind']) == (T_PM, 'head_of_government')
    heads = [r['id'] for e in packet['organizations'] + packet['institutions'] for r in e['roles']
             if r['kind'] == 'head_of_government']
    assert heads == [PM], 'no other head-of-government role'
    previous = ''
    for holder in role['holder_claims']:
        name = holder['name']
        assert isinstance(holder, dict) and name in SURNAMES, name
        dated = [d for d in (holder['attested_on'], holder['from']) if d]
        assert len(dated) == 1, (name, 'a holder is dated by exactly one of attested_on and from')
        assert dated[0] >= previous, (name, 'holders stay in chronological order')
        previous = dated[0]
        assert not {holder['attested_on'], holder['from'], holder['until']} & NEVER_HOLDER_DATE, name
        assert not set(holder['claim_ids']) & set(NEVER_HOLDER), name
        kinds = {}
        for cid in holder['claim_ids']:
            row = rows[cid]
            assert cid in role['claim_ids'], cid
            assert row['holder_name'] == name and row['role_title'] == T_PM, (name, cid)
            assert row['event_kind'] in HOLDER_KINDS, (name, cid)
            assert SURNAMES[name] in claims[cid]['text'], (name, cid)
            kinds.setdefault(row['event_kind'], set()).add(claims[cid].get('attested_on'))
        from_days = set().union(*(kinds.get(k, set()) for k in FROM_KINDS))
        until_days = set().union(*(kinds.get(k, set()) for k in UNTIL_KINDS))
        attest_days = set().union(*(kinds.get(k, set()) for k in ATTEST_KINDS))
        # A start only where a source states the day the appointment took effect or office was assumed.
        assert ({holder['from']} == from_days) if holder['from'] else not from_days, (name, 'start')
        # An end only where a source states the day the office ended (a resignation accepted with effect from a day).
        assert ({holder['until']} == until_days) if holder['until'] else not until_days, (name, 'end')
        if holder['attested_on']:
            assert holder['attested_on'] in attest_days, (name, 'observation')
        if holder['until']:
            assert holder['until'] <= research.CUTOFF and (holder['from'] or holder['attested_on']) <= holder['until']
    # Continuation, resignation and retrospective claims stay on the role, and no claim carries a structured period.
    for cid in NEVER_HOLDER:
        assert cid in role['claim_ids'], cid
        assert rows[cid]['event_kind'] not in FROM_KINDS | UNTIL_KINDS, cid
    for cid in SPANS:
        assert not {'attested_on', 'period', 'attested_period'} & set(claims[cid]), cid
    for cid in office['claim_ids']:
        assert 'period' not in claims[cid] and 'attested_period' not in claims[cid], cid


def pm_invariants(packet, rows):
    """The rules plus the exact pinned holder list this packet intends."""
    pm_rules(packet, rows)
    role = packet['institutions'][0]['roles'][0]
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    got = [(h['name'], h['attested_on'], h['from'], h['until']) for h in role['holder_claims']]
    assert got == HOLDERS, got
    assert [h['claim_ids'] for h in role['holder_claims']] == HOLDER_CLAIMS
    assert [(h['name'], h['until']) for h in role['holder_claims'] if h['until']] == ENDS
    assert [(h['name'], h['from']) for h in role['holder_claims'] if h['from']] == STARTS
    # Distinct dated events stay distinct and keep their own days.
    for cid, (day, _kind, _obs) in EVENTS.items():
        assert claims[cid].get('attested_on') == day, cid


class IndiaPrimeMinistersTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.raw = (research.ROOT / research.RESEARCH / 'india.json').read_text(encoding='utf-8')
        cls.packet = json.loads(cls.raw)
        cls.sources = {s['id']: s for s in cls.packet['sources']}
        cls.claims = {c['id']: c for s in cls.packet['sources'] for c in s['claims']}
        cls.claim_source = {c['id']: s['id'] for s in cls.packet['sources'] for c in s['claims']}
        cls.office = cls.packet['institutions'][0]
        cls.role = cls.office['roles'][0]
        cls.extracts = {sid: json.loads((research.ROOT / cls.sources[sid]['snapshot']['path']).read_text(encoding='utf-8'))
                        for sid in NEW_SOURCES}
        cls.rows = load_rows()
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
        self.assertEqual((len(NEW_SOURCES), len(self.new_claims)), (70, 112))
        self.assertEqual([s['id'] for s in self.packet['sources']], list(ORIGINAL_SOURCES) + NEW_SOURCES)
        self.assertEqual((len(ids['entries']), len(ids['roles'])), (83, 1))
        self.assertEqual(len(self.packet['organizations']), 82)
        # Every new claim is either a holder claim or a claim that never feeds a holder, never both.
        holder_claims = {cid for ids_ in HOLDER_CLAIMS for cid in ids_}
        self.assertEqual(len(NEVER_HOLDER), len(set(NEVER_HOLDER)))
        self.assertFalse(holder_claims & set(NEVER_HOLDER))
        self.assertEqual(holder_claims | set(NEVER_HOLDER), set(self.new_claims))
        self.assertEqual((len(holder_claims), len(NEVER_HOLDER)), (34, 78))
        self.assertEqual(set(EVENTS), set(self.new_claims))
        # Every new claim and source is cited by the institution and its one role, and by no organization.
        self.assertEqual(self.office['claim_ids'], self.new_claims)
        self.assertEqual(self.role['claim_ids'], self.new_claims)
        self.assertEqual(self.office['sources'], NEW_SOURCES)
        self.assertEqual(self.role['sources'], NEW_SOURCES)
        for entry in self.packet['organizations']:
            self.assertFalse(set(self.new_claims) & set(entry['claim_ids']), entry['id'])
            self.assertFalse(set(NEW_SOURCES) & set(entry['sources']), entry['id'])
        # At most ten observations, each reported and each carrying rows.
        observations = re.findall(r'^### (IN-PM-\d\d)\b', self.report, re.M)
        self.assertEqual(observations, [f'IN-PM-{n:02d}' for n in range(1, 11)])
        self.assertEqual({row['review_observation'] for row in self.rows.values()},
                         {f'IN-PM-{n:02d}' for n in range(1, 11)})
        for stale in STALE_IDS:
            self.assertNotIn(stale, self.raw, stale)
            for extract in self.extracts.values():
                self.assertNotIn(stale, json.dumps(extract), stale)
        self.assertFalse({row['event_kind'] for row in self.rows.values()} & OLD_KINDS)

    def test_holders_are_exactly_as_intended(self):
        pm_invariants(self.packet, self.rows)
        for holder in self.role['holder_claims']:
            self.assertEqual(list(holder), ['name', 'attested_on', 'from', 'until', 'sources', 'claim_ids', 'note',
                                            'uncertainty'])
            self.assertTrue(holder['note'] and holder['uncertainty'], holder['name'])
            expected_sources = []
            for cid in holder['claim_ids']:
                if self.claim_source[cid] not in expected_sources:
                    expected_sources.append(self.claim_source[cid])
            self.assertEqual(holder['sources'], expected_sources, holder['name'])
            # The atlas shows only 'Observed on' for a holder with attested_on, so such a holder's note carries its end.
            if holder['attested_on'] and holder['until']:
                day = int(holder['until'][8:])
                self.assertIn('this note carries the end', holder['note'])
                self.assertRegex(holder['note'], rf'end: {day} \w+ {holder["until"][:4]}')
        self.assertEqual(list(self.office), ['id', 'name', 'kind', 'represented_party_ids', 'reconciled_organization_id',
                                             'lifecycle', 'roles', 'sources', 'claim_ids', 'coverage'])
        self.assertEqual(self.office['represented_party_ids'], [])
        self.assertIsNone(self.office['reconciled_organization_id'])
        self.assertEqual(self.office['lifecycle'], {
            'status': 'unknown', 'from': None, 'until': None,
            'note': 'These observations do not establish founding, dissolution, legal continuity or exact terms of the institution.'})
        self.assertEqual(self.office['coverage']['status'], 'partial')
        self.assertEqual(self.office['coverage']['period'], {'from': '1990-01-01', 'through': '2026-09-07'})
        self.assertEqual(list(self.role), ['id', 'title', 'kind', 'sources', 'claim_ids', 'holder_claims', 'scope_note'])
        for cid in NEVER_HOLDER:
            self.assertNotIn(self.rows[cid]['event_kind'], FROM_KINDS | UNTIL_KINDS, cid)
        for cid in self.new_claims:
            self.assertEqual((self.rows[cid]['role_title'], self.rows[cid]['role_id']), (T_PM, PM), cid)
        # The council's oath names no one's oath individually; every other row names the Prime Minister concerned.
        self.assertEqual({cid for cid in self.new_claims if self.rows[cid]['holder_name'] is None}, set(COUNCIL_OATHS))
        self.assertEqual({self.rows[cid]['holder_name'] for cid in self.new_claims} - {None}, set(SURNAMES))

    def test_starts_and_ends_only_where_a_source_states_one(self):
        claims, rows = self.claims, self.rows
        for cid, row in rows.items():
            if row['event_kind'] in ('appointment_with_effect', 'resignation_accepted_with_effect'):
                self.assertIn('with effect from', claims[cid]['text'], cid)
                self.assertIn('Gazette', self.sources[self.claim_source[cid]]['title'], cid)
        # Acceptances without an effective day, requests to continue and continuation attestations are not ends.
        for cid in CONTINUATION + RESIGNATIONS:
            self.assertNotIn(rows[cid]['event_kind'], UNTIL_KINDS, cid)
        self.assertEqual({rows[cid]['event_kind'] for cid in CONTINUATION},
                         {'continuation_requested', 'continuation_in_office_attestation'})
        for cid in CONTINUATION:
            self.assertRegex(claims[cid]['uncertainty'], r'(?i)never a separate holder|claims only|a claim only', cid)
        # Gujral's first-printed acceptance 'with effect from 19th March, 1998' was superseded and dates nothing.
        superseded = claims['in_gujral_resignation_accepted_wef_superseded_19980406']
        self.assertNotIn('attested_on', superseded)
        self.assertIn('with effect from 19th March, 1998', superseded['text'])
        self.assertTrue(superseded['uncertainty'].startswith('Superseded'))
        self.assertIn('advised them to continue till a new Government is formed',
                      claims['in_gazette_corrigendum_gujral_acceptance_19980814']['text'])
        self.assertIsNone(self.role['holder_claims'][5]['until'])
        # The 1998 notification's English and Hindi hours differ, so only the day is kept.
        self.assertIn('09.00 hours', claims['in_vajpayee_appointed_pm_wef_19980319']['uncertainty'])
        # The 1991 notification states no end of the continuation; that reading is the packet's, not the source's.
        self.assertIn("not the source's words", claims['in_president_accepts_chandra_shekhar_resignation_wef_19910621']['uncertainty'])
        # Swearing-in reports and appointment communiques date an observation and never make a start.
        for cid in ('in_cabsec_modi_sworn_in_as_pm_20140526', 'in_cabsec_modi_sworn_in_as_pm_20190530',
                    'in_cabsec_modi_sworn_in_as_pm_20240609', 'in_modi_appointed_pm_communique_20140526',
                    'in_mms_appointed_pm_communique_20040522', 'in_mms_appointed_pm_communique_20090522'):
            self.assertRegex(claims[cid]['uncertainty'], r'(?i)not a from', cid)
        for cid in COUNCIL_OATHS:
            self.assertEqual(rows[cid]['event_kind'], 'council_oath_administered')
            self.assertIn("does not name the Prime Minister's own oath", claims[cid]['uncertainty'])
        for cid in SPANS:
            self.assertEqual(rows[cid]['event_kind'], 'retrospective_term_span', cid)
            self.assertIn('no structured date is stored', claims[cid]['uncertainty'], cid)
        for cid in ('in_vajpayee_swearing_in_recalled_in_house_19960516', 'in_deve_gowda_swearing_in_recalled_in_house_19960601'):
            self.assertEqual(rows[cid]['event_kind'], 'swearing_in_recalled_in_debate')
            self.assertIn("member's statement", claims[cid]['uncertainty'])
        # Holder wording: stated ends explained, unstated ends refused, and no end inferred from a swearing-in.
        holders = self.role['holder_claims']
        self.assertIn('earliest 1990 attestation pinned to a byte-stable response', holders[0]['note'])
        for holder in holders:
            if not holder['until']:
                self.assertTrue(re.search(r'No (start and no )?end', holder['uncertainty']), holder['name'])
            self.assertNotRegex(holder['uncertainty'] + holder['note'], r'(?i)until the \d+ \w+ \d{4} swearing-in')
        scope = self.role['scope_note']
        for text in ('never as a separate holder', 'procedure only, never a date', "a successor's appointment or swearing-in",
                     'with effect from a stated day'):
            self.assertIn(text, scope)
        unresolved = self.office['coverage']['unresolved']
        self.assertTrue(unresolved[0].startswith('Prime ministers 1990-2026 (CLAUDE-C01-11'))
        self.assertTrue(unresolved[1].startswith('Stated starts:'))
        self.assertIn('Stated ends: Chandra Shekhar (21 June 1991)', unresolved[1])
        self.assertTrue(unresolved[-1].startswith('Keep executive office distinct from party leadership'))
        coverage = self.packet['coverage']
        self.assertEqual(sum('CLAUDE-C01-11' in u for u in coverage['unresolved']), 1)
        self.assertTrue(coverage['unresolved'][-1].startswith('Prime ministers 1990-2026 (CLAUDE-C01-11)'))
        self.assertEqual(len(coverage['unresolved']), 8)
        self.assertEqual([r['records'] for r in coverage['bounded_registers']], [6, 76])

    def test_extracts_match_packet_claims_and_record_original_responses(self):
        for sid in NEW_SOURCES:
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual(extract['format'], 'spheres-c01-derived-factual-table/v1')
            self.assertEqual((extract['source_id'], extract['source_url']), (sid, source['url']))
            for key in ('scope_note', 'access_method', 'published_date', 'rights_note', 'accessed_date'):
                self.assertEqual(extract[key], source[key], (sid, key))
            self.assertEqual(source['accessed_date'], '2026-09-24' if sid in ACCESSED_20260924 else '2026-09-23', sid)
            self.assertLessEqual(source['published_date'] or '', research.CUTOFF)
            self.assertIs(extract['source_response_checked_in'], False)
            self.assertEqual((extract['source_response_bytes'], extract['source_response_sha256']), RESPONSES[sid])
            self.assertIn('not checked into this repository', extract['provenance_note'])
            self.assertIn('derived factual extract', extract['provenance_note'])
            self.assertIn('same byte count and SHA-256', extract['provenance_note'])
            self.assertIn('No open license, portrait permission or likeness approval', extract['rights_note'])
            self.assertEqual(extract['visual_review']['pdf_pages_one_based'], PDF_PAGES.get(sid, []))
            self.assertTrue(source['source_type'].startswith('primary_') and source['scope_note'])
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
                self.assertEqual((row['observation_id'], row['role_id']), ('in_prime_minister', PM))
                self.assertNotIn('name', row)
                self.assertEqual(list(row), ['claim_id', 'observation_id', 'review_observation', 'role_id', 'holder_name',
                                             'role_title', 'event_kind', 'attested_on', 'text', 'locator'])
        self.assertEqual(len({self.sources[s]['snapshot']['path'] for s in NEW_SOURCES}), len(NEW_SOURCES))
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
            self.assertEqual(extract['archive_capture_utc'].replace('-', '').replace(':', '').replace('T', '').rstrip('Z'),
                             stamp)
            self.assertIn('Raw Internet Archive capture', extract['provenance_note'])
        for sid in MIRRORS:
            url = urlsplit(self.sources[sid]['url'])
            self.assertEqual((url.hostname, url.path.split('/')[1]), ('archive.org', 'download'), sid)
            self.assertTrue(self.extracts[sid]['mirror_of'], sid)
            self.assertIn('Internet Archive item', self.extracts[sid]['provenance_note'])
            self.assertNotIn('original_url', self.sources[sid])
        others = [sid for sid in NEW_SOURCES if sid not in ARCHIVED and sid not in MIRRORS]
        self.assertEqual((len(ARCHIVED), len(MIRRORS), len(others)), (50, 6, 14))
        self.assertEqual({urlsplit(self.sources[s]['url']).hostname for s in others}, OFFICIAL_HOSTS)
        for sid in others:
            self.assertNotIn('original_url', self.sources[sid])
            self.assertNotIn('archive_capture_utc', self.extracts[sid])
        for sid, (decoded_bytes, decoded_sha) in GZIP.items():
            extract = self.extracts[sid]
            self.assertEqual(extract['source_response_content_encoding'], 'gzip')
            self.assertEqual((extract['decoded_response_bytes'], extract['decoded_response_sha256']),
                             (decoded_bytes, decoded_sha))
            self.assertIn('gzip-encoded', extract['provenance_note'])
            self.assertIn(decoded_sha, extract['provenance_note'])
        for sid in set(NEW_SOURCES) - set(GZIP):
            self.assertNotIn('source_response_content_encoding', self.extracts[sid])
        for sid, (stamp, size, sha) in DATE_ANCHORS.items():
            anchor = self.extracts[sid]['date_anchor']
            self.assertEqual((anchor['source_response_bytes'], anchor['source_response_sha256']), (size, sha))
            self.assertTrue(anchor['url'].startswith(f'https://web.archive.org/web/{stamp}id_/http'), sid)
            self.assertLess(stamp, '20260907')
        self.assertEqual({sid for sid in NEW_SOURCES if 'date_anchor' in self.extracts[sid]}, set(DATE_ANCHORS))

    def test_response_identities_are_reproducible_urls(self):
        urls = [self.sources[sid]['url'] for sid in NEW_SOURCES]
        urls += [self.extracts[sid]['date_anchor']['url'] for sid in DATE_ANCHORS]
        for url in urls:
            for pattern in PER_REQUEST_PATTERNS:
                self.assertNotIn(pattern, url, url)
            parts = urlsplit(url)
            self.assertEqual(parts.scheme, 'https', url)
            if parts.hostname == 'web.archive.org':
                self.assertRegex(parts.path, r'^/web/\d{14}id_/https?://', url)
            else:
                # Dynamic presidential, PMO and PIB sites are recorded only through fixed archive captures.
                self.assertIn(parts.hostname, OFFICIAL_HOSTS | {'archive.org'}, url)
            # The Rajya Sabha store's query only sets the served content type and file name; it is not a session value.
            if parts.hostname == 'bucketapi.rajyasabha.digital':
                self.assertEqual(sorted(k.split('=')[0] for k in parts.query.split('&')),
                                 ['response-content-disposition', 'response-content-type'])
        # The live, per-request Drupal and ASP.NET pages the research rejected are never recorded identities.
        for source in self.packet['sources']:
            self.assertNotRegex(source['url'], r'^https://(www\.)?(presidentofindia|pmindia|pib|archive\.pib)\.', source['id'])

    def test_secondary_and_unimported_leads_stay_out_of_the_packet(self):
        for source in self.packet['sources']:
            for marker in LEAD_URL_MARKERS:
                self.assertNotIn(marker, source['url'], source['id'])
        lowered = self.raw.lower()
        for marker in ('wikipedia', 'cnn.com', 'deccanherald', 'newsonair', 'fbis'):
            self.assertNotIn(marker, lowered, marker)
        leads, added = self.section('Leads not imported'), self.section('Sources added')
        for marker in ('micro_IA1176910_0664', 'jpi_june_1996', 'relid=1734', 'narayanan_25_10_1999', 'PIBR170398',
                       '1903982', '1310993', '1_Upload_4252', 'pr260519', 'wikipedia'):
            self.assertIn(marker, leads, marker)
            self.assertNotIn(marker, added, marker)

    def test_packet_formatting_is_preserved(self):
        data = (research.ROOT / research.RESEARCH / 'india.json').read_bytes()
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

        def role(packet):
            return packet['institutions'][0]['roles'][0]

        def holder(packet, index):
            return role(packet)['holder_claims'][index]

        def cite(index, cid, **dates):
            def change(packet):
                holder(packet, index).update(dates)
                holder(packet, index)['claim_ids'].append(cid)
                sid = self.claim_source[cid]
                if sid not in holder(packet, index)['sources']:
                    holder(packet, index)['sources'].append(sid)
            return change

        validator_cases = [
            (lambda p: source(p, 'in_gazette_ext_p1s2_no16_19991026')['snapshot'].update(sha256='0' * 64), 'checksum mismatch'),
            (lambda p: source(p, 'in_ls_debates_19910311')['snapshot'].update(bytes=1), 'checksum mismatch'),
            (lambda p: source(p, 'in_rs_papers_19910307')['snapshot'].update(path=REPORT.as_posix()), 'escapes'),
            (lambda p: holder(p, 12).update(until='2026-09-08'), 'exceeds cutoff'),
            (lambda p: holder(p, 12).update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: claim(p, 'in_modi_pm_state_visit_uzbekistan_20260830').update(attested_on='2026-09-08'),
             'exceeds cutoff'),
            (lambda p: claim(p, 'in_pmo_list_span_mms_20040522_20140526').update(
                period={'from': '2004-05-22', 'through': '2026-09-30'}), 'exceeds cutoff'),
            (lambda p: holder(p, 6).update(until='1998-01-01'), 'Reversed historical interval'),
            (lambda p: holder(p, 3)['claim_ids'].append('in_gujral_appointed_pm_wef_19970421'), 'cited source'),
            (lambda p: role(p)['claim_ids'].append('in_does_not_exist'), 'Unknown'),
        ]
        for change, message in validator_cases:
            with self.subTest(message=message), self.assertRaisesRegex(ValueError, message):
                self.validate(mutated(change))

        def continuation_holder(name, day, cid):
            return {'name': name, 'attested_on': day, 'from': None, 'until': None,
                    'sources': [self.claim_source[cid]], 'claim_ids': [cid], 'note': 'x', 'uncertainty': 'x'}

        rule_cases = [
            # A successor's swearing-in or appointment used as an end.
            ('successor swearing-in used as an end (V. P. Singh)', lambda p: holder(p, 0).update(until='1990-11-16')),
            ('successor swearing-in used as an end (Vajpayee 1996)', lambda p: holder(p, 3).update(until='1996-06-11')),
            ('successor swearing-in used as an end (Gujral)', lambda p: holder(p, 5).update(until='1998-03-19')),
            ('successor oath cited as an end (Gujral)', cite(5, 'in_vajpayee_oath_of_office_19980319', until='1998-03-19')),
            ('successor swearing-in used as an end (Vajpayee 1999)', lambda p: holder(p, 7).update(until='2004-05-22')),
            ('own reappointment used as an end (Manmohan Singh 2004)', lambda p: holder(p, 8).update(until='2009-05-22')),
            ('successor swearing-in used as an end (Manmohan Singh 2009)', lambda p: holder(p, 9).update(until='2014-05-26')),
            ('own swearing-in used as an end (Modi 2019)', lambda p: holder(p, 11).update(until='2024-06-09')),
            # A resignation announcement, letter or acceptance without a stated effective day used as an end.
            ('resignation announcement used as an end (Chandra Shekhar)',
             cite(1, 'in_chandra_shekhar_announces_resignation_19910306', until='1991-03-06')),
            ('resignation announcement used as an end (Vajpayee 1996)',
             cite(3, 'in_vajpayee_resignation_announced_in_house_19960528', until='1996-05-28')),
            ('resignation letter used as an end (Deve Gowda)', lambda p: holder(p, 4).update(until='1997-04-11')),
            ('acceptance letter used as an end (Gujral)',
             cite(5, 'in_president_accepts_gujral_resignation_19971128', until='1997-11-28')),
            ('superseded Gazette acceptance used as an end (Gujral)',
             cite(5, 'in_gujral_resignation_accepted_wef_superseded_19980406', until='1998-03-19')),
            ('resignation communique used as an end (Vajpayee 2004)',
             cite(7, 'in_vajpayee_tenders_resignation_20040513', until='2004-05-13')),
            ('acceptance communique used as an end (Modi 2019)',
             cite(11, 'in_kovind_accepts_modi_resignation_20190524', until='2019-05-24')),
            ('lost confidence vote used as an end (V. P. Singh)',
             cite(0, 'in_ls_confidence_motion_negatived_19901107', until='1990-11-07')),
            ('lost confidence vote used as an end (Vajpayee 1998)', lambda p: holder(p, 6).update(until='1999-04-17')),
            ('retrospective list used as an end (V. P. Singh)',
             cite(0, 'in_pmo_span_vp_singh_19891202_19901110', until='1990-11-10')),
            ('retrospective list used as a start (Chandra Shekhar)',
             lambda p: holder(p, 1).update({'attested_on': None, 'from': '1990-11-10'})),
            ('retrospective list used as an end (Vajpayee 1996)', lambda p: holder(p, 3).update(until='1996-06-01')),
            ('member recollection used as an observation (Deve Gowda)',
             lambda p: holder(p, 4).update(attested_on='1996-06-01')),
            # A continuation after a resignation added as a holder, or its claims cited by a holder.
            ('caretaker continuation added as a holder (Chandra Shekhar)', lambda p: role(p)['holder_claims'].insert(
                2, continuation_holder('Chandra Shekhar', '1991-06-04', 'in_chandra_shekhar_pm_in_rajya_sabha_19910604'))),
            ('caretaker continuation added as a holder (Gujral)', lambda p: role(p)['holder_claims'].insert(
                6, continuation_holder('I. K. Gujral', '1998-03-12', 'in_gujral_styled_pm_holi_greetings_19980312'))),
            ('caretaker continuation added as a holder (Vajpayee 1999)', lambda p: role(p)['holder_claims'].insert(
                7, continuation_holder('Atal Bihari Vajpayee', '1999-08-15', 'in_vajpayee_styled_pm_independence_day_19990815'))),
            ('caretaker continuation added as a holder (Modi 2024)', lambda p: role(p)['holder_claims'].insert(
                12, continuation_holder('Narendra Modi', '2024-06-05', 'in_modi_requested_to_continue_20240605'))),
            ('continuation request cited by a holder', cite(9, 'in_mms_requested_to_continue_20140517')),
            ('continuation attestation cited by a holder', cite(6, 'in_vajpayee_styled_pm_independence_day_19990815')),
            ('confidence vote cited by a holder', cite(1, 'in_ls_confidence_motion_adopted_19901116')),
            # Starts and observations only where a source states them.
            ('swearing-in report used as a start (Modi 2014)',
             lambda p: holder(p, 10).update({'attested_on': None, 'from': '2014-05-26'})),
            ('appointment communique used as a start (Manmohan Singh 2004)',
             lambda p: holder(p, 8).update({'attested_on': None, 'from': '2004-05-22'})),
            ('designate appointment used as an observation (Modi 2014)', lambda p: holder(p, 10).update(attested_on='2014-05-20')),
            ('designate appointment cited by a holder (Manmohan Singh 2009)', cite(9, 'in_patil_appoints_mms_pm_20090520')),
            ('oath announcement cited by a holder (Modi 2019)', cite(11, 'in_modi_oath_announced_20190526')),
            ('council oath cited by a holder (Manmohan Singh 2004)', cite(8, 'in_council_oath_administered_20040522')),
            ('member recollection cited by a holder (Vajpayee 1996)',
             cite(3, 'in_vajpayee_swearing_in_recalled_in_house_19960516')),
            ('invitation used as a start (Vajpayee 1998)', lambda p: holder(p, 6).update({'from': '1998-03-16'})),
            ('span given a structured date', lambda p: claim(p, 'in_pmo_span_gujral_19970421_19980319').update(
                attested_on='1998-03-19')),
            ('span stored as a structured period', lambda p: claim(p, 'in_pmo_span_deve_gowda_19960601_19970421').update(
                attested_period={'from': '1996-06-01', 'through': '1997-04-21'})),
            ('second role', lambda p: p['institutions'][0]['roles'].append(dict(copy.deepcopy(role(p)), id='in_pm_2'))),
            ('second institution', lambda p: p['institutions'].append(dict(copy.deepcopy(p['institutions'][0]),
                                                                           id='in_prime_minister_2'))),
            ('head of government moved onto an organization', lambda p: p['organizations'][0]['roles'].append(
                dict(copy.deepcopy(role(p)), id='in_pm_org'))),
            ('holders out of order', lambda p: role(p)['holder_claims'].reverse()),
        ]
        pm_rules(self.packet, self.rows)
        pm_invariants(self.packet, self.rows)
        for label, change in rule_cases:
            with self.subTest(label=label):
                packet = mutated(change)
                with self.assertRaises((AssertionError, KeyError, IndexError)):
                    pm_rules(packet, self.rows)
                with self.assertRaises((AssertionError, KeyError, IndexError)):
                    pm_invariants(packet, self.rows)
        # Event collapses are caught by the pinned events.
        for cid, day in (('in_president_accepts_chandra_shekhar_resignation_19910306', '1991-06-21'),
                         ('in_deve_gowda_requested_to_continue_19970412', '1997-04-21'),
                         ('in_modi_oath_announced_20240607', '2024-06-09'),
                         ('in_pranab_appoints_modi_pm_20140520', '2014-05-26')):
            with self.subTest(collapsed=cid), self.assertRaises(AssertionError):
                pm_invariants(mutated(lambda p: claim(p, cid).update(attested_on=day)), self.rows)

    def test_report_and_handoff_close_no_parent_gate(self):
        self.assertIn('ready_for_review', self.report)
        table = self.section('Outcome')
        decisions = {'01': 'Accepted in part', '02': 'Accepted in part', '03': 'Accepted in part',
                     '04': 'Accepted in part', '05': 'Accepted in part', '06': 'Accepted', '07': 'Accepted in part',
                     '08': 'Accepted in part', '09': 'Accepted in part', '10': 'Accepted in part'}
        for number, decision in decisions.items():
            row, = [line for line in table.splitlines() if line.startswith(f'| IN-PM-{number} ')]
            self.assertIn(f'**{decision}:**', row)
        defects = self.section('Checker defects')
        rows = [line for line in defects.splitlines() if re.match(r'\| [ABC]\d+ ', line)]
        self.assertEqual([re.match(r'\| ([ABC]\d+) ', r).group(1) for r in rows],
                         [f'A{n}' for n in range(1, 15)] + [f'B{n}' for n in range(1, 10)] + [f'C{n}' for n in range(1, 11)])
        for row in rows:
            self.assertRegex(row, r'\*\*(Applied|Resolved by removal)')
        for marker in ('C01', 'C06', 'S23', 'WC1', 'CP1'):
            self.assertNotRegex(self.report, rf'\b{marker}\b[^.\n]*\bis (now )?complete\b')
        notes = self.section('Integration notes')
        for text in ('38f36fa2', 'ffe54b02', 'research-index.json', 'test_india_research_s10e.py', 'test_campaign_census',
                     'Observed on'):
            self.assertIn(text, notes)
        identities = self.section('Response identities and stability checks')
        for sid, (size, sha) in RESPONSES.items():
            self.assertIn(f'`{sid}`', identities, sid)
            self.assertIn(sha[:12], identities, sid)
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        for text in ('ready_for_review', 'india-prime-ministers-1990-2026-11.md', 'claude/c01-in-11', '38f36fa2',
                     'ffe54b02', 'test_india_prime_ministers_c01_11.py'):
            self.assertIn(text, handoff)
        index = research.build()
        country = next(p for p in index['countries'] if p['nation'] == 'India')
        self.assertFalse(country['country_census_complete'])
        self.assertIsNone(country['unrepresented_organization_count'])
        self.assertEqual((country['institution_observations'], country['role_observations']), (1, 1))
        self.assertEqual({w['status'] for w in index['work_orders'] if w['nation'] == 'India'}, {'open'})
        self.assertFalse(index['c01_complete'])


if __name__ == '__main__':
    unittest.main()
