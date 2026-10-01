"""CLAUDE-C01-29: the chair of the Japan Socialist Party (日本社会党委員長) and, from the party's January 1996 renaming, the
party leader of the Social Democratic Party (社会民主党党首), 1990-2026, are one party role kept apart from state office.
Each party election or selection, declaration, acting arrangement, resignation and the renaming stays a separate dated
claim; a holder is dated by a same-day party record or the chair's own statement of the office, has a start or an end only
where a source states the day, and the renaming and the 2020 split are claims about the organization, never a merge or a
mapping of identities."""
import copy
import hashlib
import json
import re
import unittest
from urllib.parse import urlsplit

import campaign_research as research
# CLAUDE-C01-31 appends the sources of a new party role, jp_komeito_representative, on the 公明党 observation after this
# packet's; its exact sources and holders are pinned in its own test.
import test_japan_komeito_representatives_c01_31 as komeito
# CLAUDE-C01-42 (stacked on CLAUDE-C01-31) appends the sources of the 日本共産党 chair and 国民民主党 representative roles
# after CLAUDE-C01-31's; its exact sources and holders are pinned in its own test.
import test_japan_jcp_dpfp_leaders_c01_42 as jcp42

ORG_ID = 'jp_sangiin_pr_2025_10'
ROLE = 'jp_sdp_chair'
T_SDP = '委員長 / 党首 — chair of the Japan Socialist Party and, from its 1996 renaming, the Social Democratic Party'
PM = 'jp_pm'
LDP_ORG, LDP_ROLE = 'jp_sangiin_pr_2025_13', 'jp_ldp_party_president'
CDP_ORG = 'jp_sangiin_pr_2025_05'  # 立憲民主党, the party most members joined in 2020: never mapped or merged here
# The packet's sources before this packet: the original S10d intake (7), CLAUDE-C01-12 (119), CLAUDE-C01-13 (211) and
# CLAUDE-C01-18 (81).
EARLIER_SOURCE_COUNT = 418
ORG_EARLIER_CLAIMS = ['jp_pr2025_submission_10']
ORG_EARLIER_SOURCES = ['jp_tokyo_pr_2025']
ORG_EARLIER_UNRESOLVED = 3
GROUPS = ['jp_shugiin_group_20260218_011', 'jp_shugiin_group_20260218_020', 'jp_shugiin_group_20260218_030',
          'jp_shugiin_group_20260218_040', 'jp_shugiin_group_20260218_050', 'jp_shugiin_group_20260218_060',
          'jp_shugiin_group_20260218_070']
EARLIER_ROLES = ['jp_jcp_executive_committee_chair', 'jp_jcp_central_committee_chair', 'jp_dpfp_representative',
                 LDP_ROLE, PM]
PM_HOLDER_COUNT = 30  # CLAUDE-C01-12's fourteen and CLAUDE-C01-13's sixteen, unchanged
LDP_HOLDER_COUNT = 16  # CLAUDE-C01-18's fourteen and the original two, unchanged
ACCESSED = '2026-09-28'

# Original response identity recorded in each extract: (bytes, sha256) of the identity-encoded body, fetched as the
# extract's fetch_recipe says. Every source is a raw Internet Archive capture or an official Diet minutes API response.
RESPONSES = {
    'jp_sdp_diet_hr_budget_19900406':
        (430425, 'a1b1bb0b139fb2452a2d31f54309ede4eafbd9836e1c83768b170993abc644e4'),
    'jp_sdp_diet_hr_budget_19901019':
        (483933, 'b0397d9eb15706f84a32fc333008595e1de1c5ee3f23b24719f4f1a8b99a3aa0'),
    'jp_sdp_doi_profile_2000_retrospective':
        (2726, '9c071b37b1562347fac94334305de10578f78d38109297ce995907096f7e33dd'),
    'jp_sdp_diet_hr_budget_19910820':
        (560358, '0028ee34fb897d7030d9684d4ba05d08f200bafff051a5eb22c335fdd7538220'),
    'jp_sdp_diet_hr_plenary_19930125':
        (32223, '6e0b09252a9a3248b0fe0f428ec2e34b4b1db2ac12a15d19a7f1dface6dafbf4'),
    'jp_sdp_diet_hc_plenary_19930924':
        (7544, '6422e3c97512ca08f0cd24fe7d85051ba852b0c83924b9fbd0c6e996a5b5a041'),
    'jp_sdp_diet_hr_budget_19941013':
        (1934, 'b0daae1c88e23cc28769d20f4ec33dd578416e054e8946900e950f8015a3f00a'),
    'jp_sdp_diet_hc_budget_19941018':
        (1292, '82f216a029143d09391f80b50557228b64aa293356c676c8c07896686e30771d'),
    'jp_sdp_diet_hr_budget_19950206':
        (3003, '64aebb051aeb907b6bd1e168bde9f88fe305346fbccc055a8b928b819e9c69f8'),
    'jp_sdp_diet_hc_budget_19950519':
        (1026, '440f7597fe3e708ce09beac22b9c5e32bdd33a31cd16b1c01d642442eaecbd2c'),
    'jp_sdp_diet_hr_budget_19951011_s99':
        (1571, '44e2e9983237eb30e363404929b7f5d9df3e8dc370b8df2df9d5efe0d6a7183f'),
    'jp_sdp_diet_hr_budget_19951011_s101':
        (1421, 'c0c883a7fe5ab135d3e8858775583c25ffde8f84de7b54fa3d8f128399a92ef3'),
    'jp_sdp_diet_hr_plenary_19960124':
        (29960, 'fa2026135fa9dcbab89392c41ad6f099bfc9abefdded19ee3c3606ce52758989'),
    'jp_sdp_statement_20th_anniversary_20160119':
        (14253, '3bbdac1415fef49ec20bdd04d2290a7df6ec15ed33e29badc6b6f67ff2646da7'),
    'jp_sdp_statement_murayama_obituary_20251017':
        (69842, 'cb9efc5316a309abf5b69bbd102690bc6b3d928fe5954662ea1a8688fc3f5d1c'),
    'jp_sdp_doi_kenren_kanjicho_meeting_19961130':
        (10317, 'f5b5f34ebbeb1bde67d4a7f7eab114af49fee6f22be3bdcc7c9c52076ed5ba38'),
    'jp_sdp_doi_second_rinji_convention_19961222':
        (11828, 'd41ce777ab01394c7061873d13a699637b5e5803bd80b7b0307cade514c6b224'),
    'jp_sdp_doi_reelection_shakaishinpo_19980121':
        (2595, '900c21b270bc2b8c3ed3f86e112775583bbea2f951b3aab8f94bc2edcd3ba255'),
    'jp_sdp_doi_fifth_rinji_convention_19980830':
        (11151, '03b7f4168649934f45a6f0ea557d7316b4ec83c83e22ef3e1298bb318e0bc6fb'),
    'jp_sdp_doi_third_term_danwa_20000121':
        (1385, '8d6c2abdf8ba7a10d56caa9fb41b6a5c0f980b86ef138b8eff3a8015cca3c61b'),
    'jp_sdp_doi_third_election_shakaishinpo_20000126':
        (5079, 'f8d180f5d9357f2031b429b52e0ec40d04eb5d97fbe43a578a9a9a7cf81d67eb'),
    'jp_sdp_doi_resignation_kaiken_20031113':
        (7791, 'bacc1cac9a4c19d8c88b5ccaaae2df68b9d03b9f7f64f20fe3bbb1300186ff23'),
    'jp_sdp_doi_resignation_greeting_20031115':
        (6373, 'b39999f382a5dcc21b5257b14f5b21150082d366b4f285f52cc6231f176b193f'),
    'jp_sdp_fukushima_inaugural_greeting_20031115':
        (4574, 'fb89762cc9037cbbae2d49b8e7fb2c804d48ce45ffbaae8829de6003541860a2'),
    'jp_sdp_diet_hc_budget_20031126':
        (1815, '09ba8677cba5a257cf7c436dade777deb389eaf4129018dfbe07c723ffdc8df3'),
    'jp_sdp_fukushima_eighth_convention_20031213':
        (10695, '62ddee95a5120bdb6e353108c434e060c4a064433409ea50c8d1116bde19189f'),
    'jp_sdp_fukushima_fourth_election_shakaishinpo_20091216':
        (7177, '52d75adb7a420fd96b47dc20056b4aa18104f6d7fbb3099fcbe51d19f9f94955'),
    'jp_sdp_fukushima_fifth_election_shakaishinpo_20120201':
        (8935, '5c64d4430a06b596ba14c28543d1549129b3af470e5580d041027c4c70f22f6a'),
    'jp_sdp_fukushima_resignation_kaiken_20130725':
        (11741, '55e6a2b845d476fc09fdf8527d952b0b76ef1536a21a17246941c931a685b98e'),
    'jp_sdp_mataichi_acting_leader_20130805':
        (11447, 'bf6efa38c215ab1cd86eca35e5d0ed094ec5faa3f28d3590f8c66fa8d550bca6'),
    'jp_sdp_leader_election_2013_decided_20130830':
        (12330, 'be90310db57bab4fefb6c825738441520f045192e872928a3ce29a565e89d04a'),
    'jp_sdp_leader_election_2013_schedule_20130907':
        (12253, 'a5ecfee15b8d36a23a28e59eade28dd7f747115664ecc7a285488d922babed25'),
    'jp_sdp_yoshida_leader_election_page_2013':
        (7742, 'fdf2f3042dc8580ce71d5d6765ed544b8dfe67c53c95ee930b7013cb1bdf2413'),
    'jp_sdp_diet_hc_budget_20131024':
        (3091, '0b4c1fa818c748e81bfed01b907d8551be96156b343944e642abc0ac688f9a8a'),
    'jp_sdp_yoshida_teirei_kaiken_20131106':
        (12002, '1c1a6c6a96f5b560c487e40a2e816cf187e99c490576584ecf13488bf75e296f'),
    'jp_sdp_yoshida_candidacy_2015_20151204':
        (14246, '5b21c4cce0ff5378bc31cf066dfa5ff7fb9220204486583ce61a2d1b136b1b21'),
    'jp_sdp_yoshida_stays_on_20160905':
        (15830, 'a21597dc9eb0032a6bbec25ccbc8adf81a744853626ea6ee83f5029c38b00f5b'),
    'jp_sdp_leader_election_2018_renotice_20180120':
        (15120, 'dc7f404d1420c3ae35ab9f631f1c027004e5f3eed18187eb8804c67427ef050b'),
    'jp_sdp_mataichi_uncontested_2018_20180208':
        (16798, '7c12ede063791ff676f1ed923ae5748d19ebc5746bd589e9f94e76f230dac4df'),
    'jp_sdp_yoshida_last_greeting_20180224':
        (23878, '2467e576561c1f381c8cbce1f8f480c62c22a054e263a4709ba52b60ddb39d0d'),
    'jp_sdp_mataichi_inaugural_greeting_20180225':
        (13887, '324db929c04f29c0284d3f4dc93cfc5f5a9f7fadb054942edba34ea19a1800f6'),
    'jp_sdp_diet_hc_budget_20180302':
        (1512, '656525dbd7f72b381bcb87b07abf85fc1e7066e9ea90d95ea6870a608e99af54'),
    'jp_sdp_top_page_17th_convention_2020':
        (42355, 'a41a004669a7349c45929560bec54b70e183966ee04a07504637bd1c8b491531'),
    'jp_sdp_convention_agenda_2020_20201022':
        (14957, 'dfa50d87ce9c5ac62777e6c161605f4a6671f975446fe0580f9d69ea0e8299f4'),
    'jp_sdp_fukushima_edano_meeting_20201117':
        (14040, '7d4bce35ace5c07af62c6318a1fd8d29f61e1dfdb354830c85b237ad949ebc6e'),
    'jp_sdp_fukushima_reelection_12th_20220114':
        (46384, '5ef1ce124d2b808f3d6046b7c2b3b2f21721e5d7779d5f9c088c1ae577a28241'),
    'jp_sdp_fukushima_reelection_20231201':
        (63761, '3642525858f312ae7f4936fe64ee545ce70d1517a5d9b60bdcaeb86659e75753'),
    'jp_sdp_leader_election_2026_schedule_20260220':
        (70650, '13d54142d767d39b5218a65d2c96d95b6035ff0e1da3ca14d3ff347a888b040a'),
    'jp_sdp_leader_election_2026_candidates_20260306':
        (80487, '0197a7bea79b7ba2021b79e298e4aafe0a76678e3b214042db7eafd1214f3735'),
    'jp_sdp_leader_election_2026_first_count_20260323':
        (69898, 'b79f8ad5a39777130815872651e339cd1a95f96dcab6fcdf6bb36735691de9cf'),
    'jp_sdp_leader_election_2026_rerun_result_20260406':
        (70343, 'b14b01452ab16abe2b7bf2497206a3260007f8a7171c346b8f53c0f431fbeed8'),
    'jp_sdp_fukushima_kaiken_20260408':
        (73642, '4f5f18e5ba7160106082221a587df6c43779b1db5cd864c757ee0b6b53cd8cad'),
    'jp_sdp_fukushima_21st_convention_20260429':
        (76555, 'cb9ffbb0b5bd769fc122bd901cff36c0a2f8dd77b82f970483241aefc6afa902'),
    'jp_sdp_fukushima_kaiken_20260729':
        (81011, 'a6756e25f4d65575a3f0909fc4748bac1bf5d6c6c35c1383b56a1a5103097264'),
}
# Base32 SHA-1 of each recorded response (for comparison with Internet Archive CDX digests).
SHA1 = {
    'jp_sdp_diet_hr_budget_19900406': 'QOLQGJHDL33MDONT3NHV2QGUQSQNJ5S7',
    'jp_sdp_diet_hr_budget_19901019': 'P6E6DCICN27TQMXL6INZS3EGUUX4IMKX',
    'jp_sdp_doi_profile_2000_retrospective': 'WQ3YJCQWPK6HWJ6GFWDL4JP2HCTFZF7D',
    'jp_sdp_diet_hr_budget_19910820': 'ZIJGEDWAPB22LYX3OFE2H2Y5JBSIBVXN',
    'jp_sdp_diet_hr_plenary_19930125': 'YEXRX6WJOZE5HTTFJCKJMYCGNVI5F7BV',
    'jp_sdp_diet_hc_plenary_19930924': 'BHL4KO5DZYZ3LKFFJOZRVW4HPRI76WTC',
    'jp_sdp_diet_hr_budget_19941013': '4BDOWXFZBJZLOJIALKUH2FKF4LZGV34U',
    'jp_sdp_diet_hc_budget_19941018': '7PMZW4ANLDCGW7WJ7JKDGFRBXJPFHM4D',
    'jp_sdp_diet_hr_budget_19950206': 'AIV2RFVZREH6KHD6ZWDFN7VVS2LZJI5A',
    'jp_sdp_diet_hc_budget_19950519': 'XRAQUDRV4XNN5Q3ZZNYOTCAMO4QVOK2T',
    'jp_sdp_diet_hr_budget_19951011_s99': '4H44NQYKT5D7MNOIJPRIOVVCBV4KGZE6',
    'jp_sdp_diet_hr_budget_19951011_s101': 'WZ42YNTPFUKTMPYYPD4PHTNPLCYA4CWK',
    'jp_sdp_diet_hr_plenary_19960124': 'OYSOOYRC3KN63XG7P542RYPXM7EDJOJA',
    'jp_sdp_statement_20th_anniversary_20160119': 'FK4KZAWACMSZXNXTGGWMBH4UY5DGPTFG',
    'jp_sdp_statement_murayama_obituary_20251017': 'AAOGPDD7EZLVW2EHDDRHF27NEH2P5UT3',
    'jp_sdp_doi_kenren_kanjicho_meeting_19961130': 'TA7UPKWGITWDVSPTUKOESDFVY3KRAYUT',
    'jp_sdp_doi_second_rinji_convention_19961222': 'MBOUDHQXRKVIQSD7TKQ76TFIYFPD6MCO',
    'jp_sdp_doi_reelection_shakaishinpo_19980121': '46NLR7FKLQ3VJTYF6IDDZHTWRGNJYLYL',
    'jp_sdp_doi_fifth_rinji_convention_19980830': '5EKIIBY4OP2I7EI7HIB4OTAQ3WELQ3MC',
    'jp_sdp_doi_third_term_danwa_20000121': 'HERVYGPAEJF77SYYGHXKFJEVELL7FICD',
    'jp_sdp_doi_third_election_shakaishinpo_20000126': 'T7QKZ2JBVPNZQKWY2JZYVRHCTBZENKAO',
    'jp_sdp_doi_resignation_kaiken_20031113': 'NPEZSBCBBNS4A3UNP4UF4V3YQ5GTO5U6',
    'jp_sdp_doi_resignation_greeting_20031115': 'XIPHQAMAMA7YAT275KG66ROQGVSYXR5W',
    'jp_sdp_fukushima_inaugural_greeting_20031115': 'DZYLBHE6IIEKJ26EEM4TWKS2M3QO3TJF',
    'jp_sdp_diet_hc_budget_20031126': 'GJ3T5N442M4YXPF63FU6UQIMHTCTK7MF',
    'jp_sdp_fukushima_eighth_convention_20031213': 'S352JEE5D6XLEPM236GFNCC4TYSHWGK7',
    'jp_sdp_fukushima_fourth_election_shakaishinpo_20091216': 'GYHO4Q7JJ64CNSQNJ7OQ6EBKWKKXMCKS',
    'jp_sdp_fukushima_fifth_election_shakaishinpo_20120201': 'SA6OJBA7UAO2F6FFMY6E47RH2HC2E5L6',
    'jp_sdp_fukushima_resignation_kaiken_20130725': 'NMV4S3EJBGGUCOGIBVAQWJO64VOJBH6H',
    'jp_sdp_mataichi_acting_leader_20130805': 'FEFI6HB6OOXTHLW3IJR7MKCSE2E5DMWO',
    'jp_sdp_leader_election_2013_decided_20130830': 'OBWJUF5B7M2UOJWVLPD6BYUHH5KAYFPT',
    'jp_sdp_leader_election_2013_schedule_20130907': 'ACA72NVSSKFACHFLZYDEQFMTXNNLN7O7',
    'jp_sdp_yoshida_leader_election_page_2013': 'ELMVE6TQH5JMOGOBJMFFRVT4UKD3W25F',
    'jp_sdp_diet_hc_budget_20131024': 'VY4AYWR7AG7S53UBGVZC6M7JA2ORENK3',
    'jp_sdp_yoshida_teirei_kaiken_20131106': 'MCJEOCPOHVREQ7QXUOBKOO6N4UL3FHW2',
    'jp_sdp_yoshida_candidacy_2015_20151204': 'KZVELAXNZD42HL4SD4VPDYPPUY2FDSK7',
    'jp_sdp_yoshida_stays_on_20160905': 'VHUAT7MQDZ7AT6MSWJ2I7N645L7N4XQT',
    'jp_sdp_leader_election_2018_renotice_20180120': '734GWEXLDOSX46XEQPQXZUNSL7QGTY4A',
    'jp_sdp_mataichi_uncontested_2018_20180208': 'TLUMP2B4JCX5ONXEFIQ7IOLBEOGM3W35',
    'jp_sdp_yoshida_last_greeting_20180224': 'ZGVXYA7U3ZEMIY7VHX32NMXI6427HNFP',
    'jp_sdp_mataichi_inaugural_greeting_20180225': 'GUON53ZC575HRZRF4JFFRYSIMLRGY45T',
    'jp_sdp_diet_hc_budget_20180302': 'B5I462ZUWZ3VABZTXF7XIN6Y6AOIQHQQ',
    'jp_sdp_top_page_17th_convention_2020': 'HUOAZABH2UA5TD3TNE5J3LDWLLEJ6XYF',
    'jp_sdp_convention_agenda_2020_20201022': '4KVO3S6QQGRAHED65JDYALWJEJKGKMHW',
    'jp_sdp_fukushima_edano_meeting_20201117': '6OED27K6KAXRXC5SBBVBR3RUYUX26BGC',
    'jp_sdp_fukushima_reelection_12th_20220114': 'U2KOSXWRH2K7JHIQNMJBS2OKEXH3L6Y3',
    'jp_sdp_fukushima_reelection_20231201': 'CDZ626WQVRIENLLR5YSVX6JZI6TYJBLK',
    'jp_sdp_leader_election_2026_schedule_20260220': 'FEMFQL4WTMQGJALXEWODHJEBRUEV5R4O',
    'jp_sdp_leader_election_2026_candidates_20260306': 'WFXQQACW423UDKHAAYJDFWJIFFIVOKLU',
    'jp_sdp_leader_election_2026_first_count_20260323': 'JQCYSGNT5X2NTAGONGG6S2PDCOGNSUW4',
    'jp_sdp_leader_election_2026_rerun_result_20260406': 'HMBFLDYBVABABXUJINUSEHYPY3MKC3Q2',
    'jp_sdp_fukushima_kaiken_20260408': 'QOYIQEY4CIPR45QWM2NKFIUH5VB5N26M',
    'jp_sdp_fukushima_21st_convention_20260429': 'ESFG2QX4JT2BMFGZHS3O24GSVKWZY7PO',
    'jp_sdp_fukushima_kaiken_20260729': 'MLNVSMXCQZERRBCV4L23HFBACJF3KODX',
}
# Raw Internet Archive captures (id_ form), all made before the cutoff: source id -> capture timestamp.
ARCHIVED = {
    'jp_sdp_doi_profile_2000_retrospective': '20000516232711',
    'jp_sdp_statement_20th_anniversary_20160119': '20160125221536',
    'jp_sdp_statement_murayama_obituary_20251017': '20251017084702',
    'jp_sdp_doi_kenren_kanjicho_meeting_19961130': '20031219113138',
    'jp_sdp_doi_second_rinji_convention_19961222': '20031219110410',
    'jp_sdp_doi_reelection_shakaishinpo_19980121': '20020619173724',
    'jp_sdp_doi_fifth_rinji_convention_19980830': '20000917114251',
    'jp_sdp_doi_third_term_danwa_20000121': '20001118090400',
    'jp_sdp_doi_third_election_shakaishinpo_20000126': '20001023191956',
    'jp_sdp_doi_resignation_kaiken_20031113': '20031208183856',
    'jp_sdp_doi_resignation_greeting_20031115': '20031208185818',
    'jp_sdp_fukushima_inaugural_greeting_20031115': '20031119041447',
    'jp_sdp_fukushima_eighth_convention_20031213': '20040205234202',
    'jp_sdp_fukushima_fourth_election_shakaishinpo_20091216': '20110531215640',
    'jp_sdp_fukushima_fifth_election_shakaishinpo_20120201': '20120614095542',
    'jp_sdp_fukushima_resignation_kaiken_20130725': '20130820115925',
    'jp_sdp_mataichi_acting_leader_20130805': '20130820140124',
    'jp_sdp_leader_election_2013_decided_20130830': '20131122043121',
    'jp_sdp_leader_election_2013_schedule_20130907': '20131122043054',
    'jp_sdp_yoshida_leader_election_page_2013': '20131014174026',
    'jp_sdp_yoshida_teirei_kaiken_20131106': '20131122043046',
    'jp_sdp_yoshida_candidacy_2015_20151204': '20161126230723',
    'jp_sdp_yoshida_stays_on_20160905': '20161126220321',
    'jp_sdp_leader_election_2018_renotice_20180120': '20190722040948',
    'jp_sdp_mataichi_uncontested_2018_20180208': '20190722040945',
    'jp_sdp_yoshida_last_greeting_20180224': '20190426141337',
    'jp_sdp_mataichi_inaugural_greeting_20180225': '20190427204110',
    'jp_sdp_top_page_17th_convention_2020': '20200324082830',
    'jp_sdp_convention_agenda_2020_20201022': '20201205022156',
    'jp_sdp_fukushima_edano_meeting_20201117': '20210125100528',
    'jp_sdp_fukushima_reelection_12th_20220114': '20220116094341',
    'jp_sdp_fukushima_reelection_20231201': '20231201213844',
    'jp_sdp_leader_election_2026_schedule_20260220': '20260312035200',
    'jp_sdp_leader_election_2026_candidates_20260306': '20260306044954',
    'jp_sdp_leader_election_2026_first_count_20260323': '20260324042609',
    'jp_sdp_leader_election_2026_rerun_result_20260406': '20260406155003',
    'jp_sdp_fukushima_kaiken_20260408': '20260410042414',
    'jp_sdp_fukushima_21st_convention_20260429': '20260516150409',
    'jp_sdp_fukushima_kaiken_20260729': '20260815230711',
}
# Official Diet minutes API responses (whole meeting records or single speeches).
DIET = [
    'jp_sdp_diet_hr_budget_19900406',
    'jp_sdp_diet_hr_budget_19901019',
    'jp_sdp_diet_hr_budget_19910820',
    'jp_sdp_diet_hr_plenary_19930125',
    'jp_sdp_diet_hc_plenary_19930924',
    'jp_sdp_diet_hr_budget_19941013',
    'jp_sdp_diet_hc_budget_19941018',
    'jp_sdp_diet_hr_budget_19950206',
    'jp_sdp_diet_hc_budget_19950519',
    'jp_sdp_diet_hr_budget_19951011_s99',
    'jp_sdp_diet_hr_budget_19951011_s101',
    'jp_sdp_diet_hr_plenary_19960124',
    'jp_sdp_diet_hc_budget_20031126',
    'jp_sdp_diet_hc_budget_20131024',
    'jp_sdp_diet_hc_budget_20180302',
]
SOURCE_TYPES = {
    'jp_sdp_diet_hr_budget_19900406': 'primary_diet_minutes_api_json',
    'jp_sdp_diet_hr_budget_19901019': 'primary_diet_minutes_api_json',
    'jp_sdp_doi_profile_2000_retrospective': 'primary_party_biography_retrospective',
    'jp_sdp_diet_hr_budget_19910820': 'primary_diet_minutes_api_json',
    'jp_sdp_diet_hr_plenary_19930125': 'primary_diet_minutes_api_json',
    'jp_sdp_diet_hc_plenary_19930924': 'primary_diet_minutes_api_json',
    'jp_sdp_diet_hr_budget_19941013': 'primary_diet_minutes_api_json',
    'jp_sdp_diet_hc_budget_19941018': 'primary_diet_minutes_api_json',
    'jp_sdp_diet_hr_budget_19950206': 'primary_diet_minutes_api_json',
    'jp_sdp_diet_hc_budget_19950519': 'primary_diet_minutes_api_json',
    'jp_sdp_diet_hr_budget_19951011_s99': 'primary_diet_minutes_api_json',
    'jp_sdp_diet_hr_budget_19951011_s101': 'primary_diet_minutes_api_json',
    'jp_sdp_diet_hr_plenary_19960124': 'primary_diet_minutes_api_json',
    'jp_sdp_statement_20th_anniversary_20160119': 'primary_party_statement_retrospective',
    'jp_sdp_statement_murayama_obituary_20251017': 'primary_party_statement_retrospective',
    'jp_sdp_doi_kenren_kanjicho_meeting_19961130': 'primary_party_record_archived',
    'jp_sdp_doi_second_rinji_convention_19961222': 'primary_party_record_archived',
    'jp_sdp_doi_reelection_shakaishinpo_19980121': 'primary_party_publication_archived',
    'jp_sdp_doi_fifth_rinji_convention_19980830': 'primary_party_record_archived',
    'jp_sdp_doi_third_term_danwa_20000121': 'primary_party_statement_archived',
    'jp_sdp_doi_third_election_shakaishinpo_20000126': 'primary_party_publication_archived',
    'jp_sdp_doi_resignation_kaiken_20031113': 'primary_party_record_archived',
    'jp_sdp_doi_resignation_greeting_20031115': 'primary_party_record_archived',
    'jp_sdp_fukushima_inaugural_greeting_20031115': 'primary_party_record_archived',
    'jp_sdp_diet_hc_budget_20031126': 'primary_diet_minutes_api_json',
    'jp_sdp_fukushima_eighth_convention_20031213': 'primary_party_record_archived',
    'jp_sdp_fukushima_fourth_election_shakaishinpo_20091216': 'primary_party_publication_archived',
    'jp_sdp_fukushima_fifth_election_shakaishinpo_20120201': 'primary_party_publication_archived',
    'jp_sdp_fukushima_resignation_kaiken_20130725': 'primary_party_web_page_archived',
    'jp_sdp_mataichi_acting_leader_20130805': 'primary_party_web_page_archived',
    'jp_sdp_leader_election_2013_decided_20130830': 'primary_party_web_page_archived',
    'jp_sdp_leader_election_2013_schedule_20130907': 'primary_party_web_page_archived',
    'jp_sdp_yoshida_leader_election_page_2013': 'primary_party_record_archived',
    'jp_sdp_diet_hc_budget_20131024': 'primary_diet_minutes_api_json',
    'jp_sdp_yoshida_teirei_kaiken_20131106': 'primary_party_web_page_archived',
    'jp_sdp_yoshida_candidacy_2015_20151204': 'primary_party_publication_archived',
    'jp_sdp_yoshida_stays_on_20160905': 'primary_party_web_page_archived',
    'jp_sdp_leader_election_2018_renotice_20180120': 'primary_party_publication_archived',
    'jp_sdp_mataichi_uncontested_2018_20180208': 'primary_party_web_page_archived',
    'jp_sdp_yoshida_last_greeting_20180224': 'primary_party_statement_archived',
    'jp_sdp_mataichi_inaugural_greeting_20180225': 'primary_party_statement_archived',
    'jp_sdp_diet_hc_budget_20180302': 'primary_diet_minutes_api_json',
    'jp_sdp_top_page_17th_convention_2020': 'primary_party_web_page_archived',
    'jp_sdp_convention_agenda_2020_20201022': 'primary_party_web_page_archived',
    'jp_sdp_fukushima_edano_meeting_20201117': 'primary_party_web_page_archived',
    'jp_sdp_fukushima_reelection_12th_20220114': 'primary_party_web_page_archived',
    'jp_sdp_fukushima_reelection_20231201': 'primary_party_publication_archived',
    'jp_sdp_leader_election_2026_schedule_20260220': 'primary_party_web_page_archived',
    'jp_sdp_leader_election_2026_candidates_20260306': 'primary_party_publication_archived',
    'jp_sdp_leader_election_2026_first_count_20260323': 'primary_party_record_archived',
    'jp_sdp_leader_election_2026_rerun_result_20260406': 'primary_party_record_archived',
    'jp_sdp_fukushima_kaiken_20260408': 'primary_party_publication_archived',
    'jp_sdp_fukushima_21st_convention_20260429': 'primary_party_publication_archived',
    'jp_sdp_fukushima_kaiken_20260729': 'primary_party_publication_archived',
}
# Every new claim's (attested_on, event_kind, review observation), exactly: distinct dated events are never re-dated,
# relabelled or moved to another observation. Retrospective records and undated references carry no structured date.
EVENTS = {
    'jp_sdp_doi_named_chair_by_secretary_general_19900406': ('1990-04-06', 'in_office_attestation', 'SDP-CHAIR-01'),
    'jp_sdp_doi_leaders_meeting_recalled_by_secretary_general': (None, 'in_office_recollection', 'SDP-CHAIR-01'),
    'jp_sdp_doi_profile_chair_from_september_1986': (None, 'retrospective_term_span', 'SDP-CHAIR-01'),
    'jp_sdp_doi_profile_resigned_july_1991': (None, 'retrospective_resignation_reference', 'SDP-CHAIR-01'),
    'jp_sdp_tanabe_named_chair_by_secretary_general_19910820': ('1991-08-20', 'in_office_attestation', 'SDP-CHAIR-02'),
    'jp_sdp_yamahana_self_stated_chair_19930125': ('1993-01-25', 'in_office_attestation', 'SDP-CHAIR-03'),
    'jp_sdp_yamahana_still_chair_19930924': ('1993-09-24', 'in_office_continuation_attestation', 'SDP-CHAIR-03'),
    'jp_sdp_murayama_self_stated_chair_19941013': ('1994-10-13', 'in_office_attestation', 'SDP-CHAIR-04'),
    'jp_sdp_murayama_self_stated_chair_19941018': ('1994-10-18', 'in_office_continuation_attestation', 'SDP-CHAIR-04'),
    'jp_sdp_murayama_self_stated_chair_19950206': ('1995-02-06', 'in_office_continuation_attestation', 'SDP-CHAIR-04'),
    'jp_sdp_murayama_self_stated_chair_19950519': ('1995-05-19', 'in_office_continuation_attestation', 'SDP-CHAIR-04'),
    'jp_sdp_murayama_self_stated_chair_s99_19951011': ('1995-10-11', 'in_office_continuation_attestation', 'SDP-CHAIR-04'),
    'jp_sdp_murayama_self_stated_chair_s101_19951011': ('1995-10-11', 'in_office_continuation_attestation', 'SDP-CHAIR-04'),
    'jp_sdp_renamed_party_named_by_secretary_general_19960124': ('1996-01-24', 'organization_name_attested', 'SDP-CHAIR-05'),
    'jp_sdp_renaming_retrospective_anniversary_statement': (None, 'retrospective_renaming_record', 'SDP-CHAIR-05'),
    'jp_sdp_murayama_obituary_chair_retrospective': (None, 'retrospective_term_span', 'SDP-CHAIR-04'),
    'jp_sdp_murayama_obituary_renaming_retrospective': (None, 'retrospective_renaming_record', 'SDP-CHAIR-05'),
    'jp_sdp_murayama_obituary_first_party_leader_retrospective': (None, 'retrospective_election_record', 'SDP-CHAIR-05'),
    'jp_sdp_murayama_obituary_handover_to_doi_retrospective': (None, 'retrospective_resignation_reference', 'SDP-CHAIR-06'),
    'jp_sdp_doi_in_office_kanjicho_meeting_19961130': ('1996-11-30', 'in_office_attestation', 'SDP-CHAIR-06'),
    'jp_sdp_doi_assumption_reported_to_meeting_19961130': ('1996-11-30', 'assumption_reported_to_party_organ', 'SDP-CHAIR-06'),
    'jp_sdp_doi_convention_approval_19961222': ('1996-12-22', 'convention_approval', 'SDP-CHAIR-06'),
    'jp_sdp_doi_in_office_second_rinji_convention_19961222': ('1996-12-22', 'in_office_continuation_attestation', 'SDP-CHAIR-06'),
    'jp_sdp_doi_leader_renotice_19980108': ('1998-01-08', 'chair_election_notice', 'SDP-CHAIR-06'),
    'jp_sdp_doi_reelection_confirmed_19980110': ('1998-01-10', 'result_declared', 'SDP-CHAIR-06'),
    'jp_sdp_doi_in_office_shakaishinpo_19980121': ('1998-01-21', 'in_office_attestation', 'SDP-CHAIR-06'),
    'jp_sdp_doi_in_office_fifth_rinji_convention_19980830': ('1998-08-30', 'in_office_continuation_attestation', 'SDP-CHAIR-06'),
    'jp_sdp_doi_third_election_statement_2000': (None, 'election_recalled_by_holder', 'SDP-CHAIR-06'),
    'jp_sdp_doi_in_office_third_term_danwa_20000121': ('2000-01-21', 'in_office_attestation', 'SDP-CHAIR-06'),
    'jp_sdp_doi_third_election_declared_20000121': ('2000-01-21', 'result_declared', 'SDP-CHAIR-06'),
    'jp_sdp_doi_in_office_resignation_kaiken_20031113': ('2003-11-13', 'in_office_continuation_attestation', 'SDP-CHAIR-06'),
    'jp_sdp_doi_resignation_intent_20031113': ('2003-11-13', 'resignation_intent_announced', 'SDP-CHAIR-06'),
    'jp_sdp_doi_resignation_accepted_standing_committee_20031113': ('2003-11-13', 'resignation_accepted', 'SDP-CHAIR-06'),
    'jp_sdp_doi_in_office_resignation_greeting_20031115': ('2003-11-15', 'in_office_continuation_attestation', 'SDP-CHAIR-06'),
    'jp_sdp_doi_resignation_submitted_20031115': ('2003-11-15', 'resignation_submitted', 'SDP-CHAIR-06'),
    'jp_sdp_doi_resignation_recalled_three_officers_20031111': ('2003-11-11', 'resignation_recalled_by_holder', 'SDP-CHAIR-06'),
    'jp_sdp_doi_resignation_recalled_standing_committee_20031113': ('2003-11-13', 'resignation_recalled_by_holder', 'SDP-CHAIR-06'),
    'jp_sdp_murayama_predecessor_recalled': (None, 'predecessor_referred_to_as_former', 'SDP-CHAIR-06'),
    'jp_sdp_fukushima_selection_reported_by_doi_20031115': ('2003-11-15', 'joint_plenary_selection', 'SDP-CHAIR-07'),
    'jp_sdp_fukushima_joint_plenary_selection_20031115': ('2003-11-15', 'joint_plenary_selection', 'SDP-CHAIR-07'),
    'jp_sdp_fukushima_in_office_inaugural_greeting_20031115': ('2003-11-15', 'in_office_attestation', 'SDP-CHAIR-07'),
    'jp_sdp_doi_resignation_greeting_referred_20031115': ('2003-11-15', 'resignation_submitted', 'SDP-CHAIR-06'),
    'jp_sdp_fukushima_assumption_stated_20031115': ('2003-11-15', 'assumption_stated', 'SDP-CHAIR-07'),
    'jp_sdp_fukushima_in_office_diet_20031126': ('2003-11-26', 'in_office_continuation_attestation', 'SDP-CHAIR-07'),
    'jp_sdp_fukushima_convention_approval_20031213': ('2003-12-13', 'convention_approval', 'SDP-CHAIR-07'),
    'jp_sdp_fukushima_in_office_eighth_convention_20031213': ('2003-12-13', 'in_office_continuation_attestation', 'SDP-CHAIR-07'),
    'jp_sdp_doi_former_leader_referred_20031213': ('2003-12-13', 'predecessor_referred_to_as_former', 'SDP-CHAIR-06'),
    'jp_sdp_doi_term_span_recalled': (None, 'retrospective_term_span', 'SDP-CHAIR-06'),
    'jp_sdp_leader_election_2009_notice_20091204': ('2009-12-04', 'chair_election_notice', 'SDP-CHAIR-07'),
    'jp_sdp_fukushima_fourth_election_decided_20091204': ('2009-12-04', 'result_declared', 'SDP-CHAIR-07'),
    'jp_sdp_fukushima_in_office_20091209': ('2009-12-09', 'in_office_attestation', 'SDP-CHAIR-07'),
    'jp_sdp_leader_election_2012_notice_20120120': ('2012-01-20', 'chair_election_notice', 'SDP-CHAIR-07'),
    'jp_sdp_fukushima_fifth_election_decided_20120120': ('2012-01-20', 'result_declared', 'SDP-CHAIR-07'),
    'jp_sdp_fukushima_in_office_20120124': ('2012-01-24', 'in_office_attestation', 'SDP-CHAIR-07'),
    'jp_sdp_fukushima_in_office_20130725': ('2013-07-25', 'in_office_continuation_attestation', 'SDP-CHAIR-07'),
    'jp_sdp_fukushima_resignation_intent_20130725': ('2013-07-25', 'resignation_intent_announced', 'SDP-CHAIR-07'),
    'jp_sdp_fukushima_resignation_accepted_20130725': ('2013-07-25', 'resignation_accepted', 'SDP-CHAIR-07'),
    'jp_sdp_fukushima_end_stated_20130725': ('2013-07-25', 'end_of_office_stated', 'SDP-CHAIR-07'),
    'jp_sdp_fukushima_tenure_record_2003_2013': (None, 'retrospective_election_record', 'SDP-CHAIR-07'),
    'jp_sdp_mataichi_acting_designated_20130801': ('2013-08-01', 'acting_chair_designated', 'SDP-CHAIR-08'),
    'jp_sdp_fukushima_former_20130805': ('2013-08-05', 'predecessor_referred_to_as_former', 'SDP-CHAIR-07'),
    'jp_sdp_leader_election_called_20130829': ('2013-08-29', 'leader_election_called', 'SDP-CHAIR-08'),
    'jp_sdp_leader_election_2013_term_procedure': (None, 'election_procedure', 'SDP-CHAIR-08'),
    'jp_sdp_leader_election_2013_schedule_20130905': ('2013-09-05', 'election_schedule', 'SDP-CHAIR-08'),
    'jp_sdp_leader_election_2013_notice_20130927': ('2013-09-27', 'chair_election_notice', 'SDP-CHAIR-08'),
    'jp_sdp_leader_election_2013_filings_20130927': ('2013-09-27', 'candidacy_filing', 'SDP-CHAIR-08'),
    'jp_sdp_leader_election_2013_count_20131014': ('2013-10-14', 'chair_vote_count', 'SDP-CHAIR-08'),
    'jp_sdp_yoshida_result_declared_20131014': ('2013-10-14', 'result_declared', 'SDP-CHAIR-08'),
    'jp_sdp_yoshida_election_recalled_diet_2013': (None, 'election_recalled_by_holder', 'SDP-CHAIR-08'),
    'jp_sdp_yoshida_in_office_diet_20131024': ('2013-10-24', 'in_office_attestation', 'SDP-CHAIR-08'),
    'jp_sdp_yoshida_in_office_kaiken_20131106': ('2013-11-06', 'in_office_continuation_attestation', 'SDP-CHAIR-08'),
    'jp_sdp_yoshida_in_office_20151126': ('2015-11-26', 'in_office_continuation_attestation', 'SDP-CHAIR-08'),
    'jp_sdp_yoshida_candidacy_intent_20151126': ('2015-11-26', 'candidacy_intent_announced', 'SDP-CHAIR-08'),
    'jp_sdp_yoshida_resignation_intent_20160714': ('2016-07-14', 'resignation_intent_announced', 'SDP-CHAIR-08'),
    'jp_sdp_yoshida_resignation_withdrawn_20160901': ('2016-09-01', 'resignation_withdrawn', 'SDP-CHAIR-08'),
    'jp_sdp_yoshida_in_office_20160901': ('2016-09-01', 'in_office_continuation_attestation', 'SDP-CHAIR-08'),
    'jp_sdp_leader_election_2018_notice_20180112': ('2018-01-12', 'chair_election_notice', 'SDP-CHAIR-09'),
    'jp_sdp_leader_election_2018_reschedule': (None, 'election_schedule', 'SDP-CHAIR-09'),
    'jp_sdp_mataichi_renotice_20180126': ('2018-01-26', 'chair_election_notice', 'SDP-CHAIR-09'),
    'jp_sdp_mataichi_uncontested_election_20180126': ('2018-01-26', 'result_declared', 'SDP-CHAIR-09'),
    'jp_sdp_mataichi_term_procedure_2018': (None, 'election_procedure', 'SDP-CHAIR-09'),
    'jp_sdp_yoshida_in_office_last_greeting_20180224': ('2018-02-24', 'in_office_continuation_attestation', 'SDP-CHAIR-08'),
    'jp_sdp_yoshida_takeover_recalled_20131101': ('2013-11-01', 'assumption_recalled_by_holder', 'SDP-CHAIR-08'),
    'jp_sdp_yoshida_departure_recalled_2018': (None, 'resignation_recalled_by_holder', 'SDP-CHAIR-08'),
    'jp_sdp_mataichi_in_office_inaugural_greeting_20180225': ('2018-02-25', 'in_office_attestation', 'SDP-CHAIR-09'),
    'jp_sdp_mataichi_in_office_diet_20180302': ('2018-03-02', 'in_office_continuation_attestation', 'SDP-CHAIR-09'),
    'jp_sdp_fukushima_uncontested_election_20200222': ('2020-02-22', 'result_declared', 'SDP-CHAIR-10'),
    'jp_sdp_mataichi_in_office_convention_20200222': ('2020-02-22', 'in_office_continuation_attestation', 'SDP-CHAIR-09'),
    'jp_sdp_fukushima_new_leader_news_item_20200228': ('2020-02-28', 'in_office_attestation', 'SDP-CHAIR-10'),
    'jp_sdp_convention_agenda_decided_20201022': ('2020-10-22', 'convention_motion_decided', 'SDP-CHAIR-10'),
    'jp_sdp_split_decision_20201114': ('2020-11-14', 'organization_split_decision', 'SDP-CHAIR-10'),
    'jp_sdp_fukushima_in_office_20201117': ('2020-11-17', 'in_office_continuation_attestation', 'SDP-CHAIR-10'),
    'jp_sdp_leader_election_2022_notice_20220114': ('2022-01-14', 'chair_election_notice', 'SDP-CHAIR-10'),
    'jp_sdp_fukushima_filing_2022_20220114': ('2022-01-14', 'candidacy_filing', 'SDP-CHAIR-10'),
    'jp_sdp_fukushima_reelection_declared_20220114': ('2022-01-14', 'result_declared', 'SDP-CHAIR-10'),
    'jp_sdp_fukushima_in_office_20220114': ('2022-01-14', 'in_office_attestation', 'SDP-CHAIR-10'),
    'jp_sdp_fukushima_new_term_planned_2022': (None, 'election_schedule', 'SDP-CHAIR-10'),
    'jp_sdp_leader_election_2023_notice_20231201': ('2023-12-01', 'chair_election_notice', 'SDP-CHAIR-10'),
    'jp_sdp_fukushima_reelection_declared_20231201': ('2023-12-01', 'result_declared', 'SDP-CHAIR-10'),
    'jp_sdp_fukushima_in_office_20231201': ('2023-12-01', 'in_office_attestation', 'SDP-CHAIR-10'),
    'jp_sdp_leader_election_2026_schedule_20260218': ('2026-02-18', 'election_schedule', 'SDP-CHAIR-11'),
    'jp_sdp_leader_election_2026_notice_20260304': ('2026-03-04', 'chair_election_notice', 'SDP-CHAIR-11'),
    'jp_sdp_leader_election_2026_filings_20260304': ('2026-03-04', 'candidacy_filing', 'SDP-CHAIR-11'),
    'jp_sdp_fukushima_incumbent_20260304': ('2026-03-04', 'in_office_continuation_attestation', 'SDP-CHAIR-11'),
    'jp_sdp_leader_election_2026_first_round_count_20260323': ('2026-03-23', 'chair_vote_count', 'SDP-CHAIR-11'),
    'jp_sdp_leader_election_2026_renotice_20260323': ('2026-03-23', 'chair_election_notice', 'SDP-CHAIR-11'),
    'jp_sdp_leader_election_2026_rerun_count_20260406': ('2026-04-06', 'chair_vote_count', 'SDP-CHAIR-11'),
    'jp_sdp_fukushima_elected_2026_20260406': ('2026-04-06', 'result_declared', 'SDP-CHAIR-11'),
    'jp_sdp_fukushima_in_office_kaiken_20260408': ('2026-04-08', 'in_office_attestation', 'SDP-CHAIR-11'),
    'jp_sdp_fukushima_in_office_21st_convention_20260429': ('2026-04-29', 'in_office_continuation_attestation', 'SDP-CHAIR-11'),
    'jp_sdp_fukushima_in_office_kaiken_20260729': ('2026-07-29', 'in_office_continuation_attestation', 'SDP-CHAIR-11'),
}
# The holder name each extract row carries; None where the source names no holder of this office.
ROW_HOLDERS = {
    'jp_sdp_doi_named_chair_by_secretary_general_19900406': '土井たか子',
    'jp_sdp_doi_leaders_meeting_recalled_by_secretary_general': '土井たか子',
    'jp_sdp_doi_profile_chair_from_september_1986': '土井たか子',
    'jp_sdp_doi_profile_resigned_july_1991': '土井たか子',
    'jp_sdp_tanabe_named_chair_by_secretary_general_19910820': '田邊誠',
    'jp_sdp_yamahana_self_stated_chair_19930125': '山花貞夫',
    'jp_sdp_yamahana_still_chair_19930924': '山花貞夫',
    'jp_sdp_murayama_self_stated_chair_19941013': '村山富市',
    'jp_sdp_murayama_self_stated_chair_19941018': '村山富市',
    'jp_sdp_murayama_self_stated_chair_19950206': '村山富市',
    'jp_sdp_murayama_self_stated_chair_19950519': '村山富市',
    'jp_sdp_murayama_self_stated_chair_s99_19951011': '村山富市',
    'jp_sdp_murayama_self_stated_chair_s101_19951011': '村山富市',
    'jp_sdp_renamed_party_named_by_secretary_general_19960124': None,
    'jp_sdp_renaming_retrospective_anniversary_statement': None,
    'jp_sdp_murayama_obituary_chair_retrospective': '村山富市',
    'jp_sdp_murayama_obituary_renaming_retrospective': None,
    'jp_sdp_murayama_obituary_first_party_leader_retrospective': '村山富市',
    'jp_sdp_murayama_obituary_handover_to_doi_retrospective': '村山富市',
    'jp_sdp_doi_in_office_kanjicho_meeting_19961130': '土井たか子',
    'jp_sdp_doi_assumption_reported_to_meeting_19961130': '土井たか子',
    'jp_sdp_doi_convention_approval_19961222': '土井たか子',
    'jp_sdp_doi_in_office_second_rinji_convention_19961222': '土井たか子',
    'jp_sdp_doi_leader_renotice_19980108': None,
    'jp_sdp_doi_reelection_confirmed_19980110': '土井たか子',
    'jp_sdp_doi_in_office_shakaishinpo_19980121': '土井たか子',
    'jp_sdp_doi_in_office_fifth_rinji_convention_19980830': '土井たか子',
    'jp_sdp_doi_third_election_statement_2000': '土井たか子',
    'jp_sdp_doi_in_office_third_term_danwa_20000121': '土井たか子',
    'jp_sdp_doi_third_election_declared_20000121': '土井たか子',
    'jp_sdp_doi_in_office_resignation_kaiken_20031113': '土井たか子',
    'jp_sdp_doi_resignation_intent_20031113': '土井たか子',
    'jp_sdp_doi_resignation_accepted_standing_committee_20031113': '土井たか子',
    'jp_sdp_doi_in_office_resignation_greeting_20031115': '土井たか子',
    'jp_sdp_doi_resignation_submitted_20031115': '土井たか子',
    'jp_sdp_doi_resignation_recalled_three_officers_20031111': '土井たか子',
    'jp_sdp_doi_resignation_recalled_standing_committee_20031113': '土井たか子',
    'jp_sdp_murayama_predecessor_recalled': '村山富市',
    'jp_sdp_fukushima_selection_reported_by_doi_20031115': '福島瑞穂',
    'jp_sdp_fukushima_joint_plenary_selection_20031115': '福島瑞穂',
    'jp_sdp_fukushima_in_office_inaugural_greeting_20031115': '福島瑞穂',
    'jp_sdp_doi_resignation_greeting_referred_20031115': '土井たか子',
    'jp_sdp_fukushima_assumption_stated_20031115': '福島瑞穂',
    'jp_sdp_fukushima_in_office_diet_20031126': '福島瑞穂',
    'jp_sdp_fukushima_convention_approval_20031213': '福島瑞穂',
    'jp_sdp_fukushima_in_office_eighth_convention_20031213': '福島瑞穂',
    'jp_sdp_doi_former_leader_referred_20031213': '土井たか子',
    'jp_sdp_doi_term_span_recalled': '土井たか子',
    'jp_sdp_leader_election_2009_notice_20091204': None,
    'jp_sdp_fukushima_fourth_election_decided_20091204': '福島瑞穂',
    'jp_sdp_fukushima_in_office_20091209': '福島瑞穂',
    'jp_sdp_leader_election_2012_notice_20120120': None,
    'jp_sdp_fukushima_fifth_election_decided_20120120': '福島瑞穂',
    'jp_sdp_fukushima_in_office_20120124': '福島瑞穂',
    'jp_sdp_fukushima_in_office_20130725': '福島瑞穂',
    'jp_sdp_fukushima_resignation_intent_20130725': '福島瑞穂',
    'jp_sdp_fukushima_resignation_accepted_20130725': '福島瑞穂',
    'jp_sdp_fukushima_end_stated_20130725': '福島瑞穂',
    'jp_sdp_fukushima_tenure_record_2003_2013': '福島瑞穂',
    'jp_sdp_mataichi_acting_designated_20130801': None,
    'jp_sdp_fukushima_former_20130805': '福島瑞穂',
    'jp_sdp_leader_election_called_20130829': None,
    'jp_sdp_leader_election_2013_term_procedure': None,
    'jp_sdp_leader_election_2013_schedule_20130905': None,
    'jp_sdp_leader_election_2013_notice_20130927': None,
    'jp_sdp_leader_election_2013_filings_20130927': None,
    'jp_sdp_leader_election_2013_count_20131014': None,
    'jp_sdp_yoshida_result_declared_20131014': '吉田忠智',
    'jp_sdp_yoshida_election_recalled_diet_2013': '吉田忠智',
    'jp_sdp_yoshida_in_office_diet_20131024': '吉田忠智',
    'jp_sdp_yoshida_in_office_kaiken_20131106': '吉田忠智',
    'jp_sdp_yoshida_in_office_20151126': '吉田忠智',
    'jp_sdp_yoshida_candidacy_intent_20151126': '吉田忠智',
    'jp_sdp_yoshida_resignation_intent_20160714': '吉田忠智',
    'jp_sdp_yoshida_resignation_withdrawn_20160901': '吉田忠智',
    'jp_sdp_yoshida_in_office_20160901': '吉田忠智',
    'jp_sdp_leader_election_2018_notice_20180112': None,
    'jp_sdp_leader_election_2018_reschedule': None,
    'jp_sdp_mataichi_renotice_20180126': None,
    'jp_sdp_mataichi_uncontested_election_20180126': '又市征治',
    'jp_sdp_mataichi_term_procedure_2018': '又市征治',
    'jp_sdp_yoshida_in_office_last_greeting_20180224': '吉田忠智',
    'jp_sdp_yoshida_takeover_recalled_20131101': '吉田忠智',
    'jp_sdp_yoshida_departure_recalled_2018': '吉田忠智',
    'jp_sdp_mataichi_in_office_inaugural_greeting_20180225': '又市征治',
    'jp_sdp_mataichi_in_office_diet_20180302': '又市征治',
    'jp_sdp_fukushima_uncontested_election_20200222': '福島瑞穂',
    'jp_sdp_mataichi_in_office_convention_20200222': '又市征治',
    'jp_sdp_fukushima_new_leader_news_item_20200228': '福島瑞穂',
    'jp_sdp_convention_agenda_decided_20201022': None,
    'jp_sdp_split_decision_20201114': None,
    'jp_sdp_fukushima_in_office_20201117': '福島瑞穂',
    'jp_sdp_leader_election_2022_notice_20220114': None,
    'jp_sdp_fukushima_filing_2022_20220114': '福島瑞穂',
    'jp_sdp_fukushima_reelection_declared_20220114': '福島瑞穂',
    'jp_sdp_fukushima_in_office_20220114': '福島瑞穂',
    'jp_sdp_fukushima_new_term_planned_2022': '福島瑞穂',
    'jp_sdp_leader_election_2023_notice_20231201': None,
    'jp_sdp_fukushima_reelection_declared_20231201': '福島瑞穂',
    'jp_sdp_fukushima_in_office_20231201': '福島瑞穂',
    'jp_sdp_leader_election_2026_schedule_20260218': None,
    'jp_sdp_leader_election_2026_notice_20260304': None,
    'jp_sdp_leader_election_2026_filings_20260304': None,
    'jp_sdp_fukushima_incumbent_20260304': '福島瑞穂',
    'jp_sdp_leader_election_2026_first_round_count_20260323': None,
    'jp_sdp_leader_election_2026_renotice_20260323': None,
    'jp_sdp_leader_election_2026_rerun_count_20260406': None,
    'jp_sdp_fukushima_elected_2026_20260406': '福島瑞穂',
    'jp_sdp_fukushima_in_office_kaiken_20260408': '福島瑞穂',
    'jp_sdp_fukushima_in_office_21st_convention_20260429': '福島瑞穂',
    'jp_sdp_fukushima_in_office_kaiken_20260729': '福島瑞穂',
}
# Rows about another office carry that office as their role title; every other row carries T_SDP.
OTHER_TITLES = {
    'jp_sdp_mataichi_acting_designated_20130801': '党首代行 — acting party leader (another office; claims only)',
}

NEW_SOURCES = list(RESPONSES)
NEW_CLAIMS = list(EVENTS)
# Exact holder observations of jp_sdp_chair: (name, attested_on, from, until), in chronological order.
HOLDERS = [
    ('土井たか子', '1990-04-06', None, None),
    ('田邊誠', '1991-08-20', None, None),
    ('山花貞夫', '1993-01-25', None, None),
    ('村山富市', '1994-10-13', None, None),
    ('土井たか子', '1996-11-30', None, None),
    ('土井たか子', '1998-01-21', None, None),
    ('土井たか子', '2000-01-21', None, None),
    ('福島瑞穂', None, '2003-11-15', None),
    ('福島瑞穂', '2009-12-09', None, None),
    ('福島瑞穂', '2012-01-24', None, '2013-07-25'),
    ('吉田忠智', '2013-10-24', None, None),
    ('又市征治', '2018-02-25', None, None),
    ('福島瑞穂', '2020-02-28', None, None),
    ('福島瑞穂', '2022-01-14', None, None),
    ('福島瑞穂', '2023-12-01', None, None),
    ('福島瑞穂', '2026-04-08', None, None),
]
HOLDER_CLAIMS = [
    ['jp_sdp_doi_named_chair_by_secretary_general_19900406'],
    ['jp_sdp_tanabe_named_chair_by_secretary_general_19910820'],
    ['jp_sdp_yamahana_self_stated_chair_19930125'],
    ['jp_sdp_murayama_self_stated_chair_19941013'],
    ['jp_sdp_doi_in_office_kanjicho_meeting_19961130'],
    ['jp_sdp_doi_in_office_shakaishinpo_19980121'],
    ['jp_sdp_doi_in_office_third_term_danwa_20000121'],
    ['jp_sdp_fukushima_assumption_stated_20031115', 'jp_sdp_fukushima_in_office_inaugural_greeting_20031115'],
    ['jp_sdp_fukushima_in_office_20091209'],
    ['jp_sdp_fukushima_in_office_20120124', 'jp_sdp_fukushima_end_stated_20130725'],
    ['jp_sdp_yoshida_in_office_diet_20131024'],
    ['jp_sdp_mataichi_in_office_inaugural_greeting_20180225'],
    ['jp_sdp_fukushima_new_leader_news_item_20200228'],
    ['jp_sdp_fukushima_in_office_20220114'],
    ['jp_sdp_fukushima_in_office_20231201'],
    ['jp_sdp_fukushima_in_office_kaiken_20260408'],
]
# Only the holder's own statement of the day she became leader states a start; only her own words state an end.
STARTS = [('福島瑞穂', '2003-11-15')]
ENDS = [('福島瑞穂', '2013-07-25')]
REVIEW = [f'SDP-CHAIR-{n:02d}' for n in range(1, 12)]
HOLDER_REVIEW = ['SDP-CHAIR-01', 'SDP-CHAIR-02', 'SDP-CHAIR-03', 'SDP-CHAIR-04', 'SDP-CHAIR-06', 'SDP-CHAIR-06',
                 'SDP-CHAIR-06', 'SDP-CHAIR-07', 'SDP-CHAIR-07', 'SDP-CHAIR-07', 'SDP-CHAIR-08', 'SDP-CHAIR-09',
                 'SDP-CHAIR-10', 'SDP-CHAIR-10', 'SDP-CHAIR-10', 'SDP-CHAIR-11']
SURNAMES = {'土井たか子': ('土井',), '田邊誠': ('田邊', '田辺'), '山花貞夫': ('山花',), '村山富市': ('村山',),
            '福島瑞穂': ('福島',), '吉田忠智': ('吉田',), '又市征治': ('又市',)}
ACTING_TITLE = '党首代行 — acting party leader (another office; claims only)'
ATTEST_KINDS = {'in_office_attestation'}
FROM_KINDS = {'assumption_stated'}
END_KINDS = {'end_of_office_stated'}
ELECTION_KINDS = {'assumption_recalled_by_holder', 'assumption_reported_to_party_organ', 'candidacy_filing',
                  'candidacy_intent_announced', 'chair_election_notice', 'chair_vote_count', 'convention_approval',
                  'election_procedure', 'election_recalled_by_holder', 'election_schedule', 'joint_plenary_selection',
                  'leader_election_called', 'result_declared'}
RESIGNATION_KINDS = {'predecessor_referred_to_as_former', 'resignation_accepted', 'resignation_intent_announced',
                     'resignation_recalled_by_holder', 'resignation_submitted', 'resignation_withdrawn'}
CONTINUATION_KINDS = {'in_office_continuation_attestation', 'in_office_recollection'}
ORGANIZATION_KINDS = {'convention_motion_decided', 'organization_name_attested', 'organization_split_decision'}
ACTING_KINDS = {'acting_chair_designated'}
RETROSPECTIVE_KINDS = {'retrospective_election_record', 'retrospective_renaming_record',
                       'retrospective_resignation_reference', 'retrospective_term_span'}
HOLDER_KINDS = ATTEST_KINDS | FROM_KINDS | END_KINDS
ALL_KINDS = (HOLDER_KINDS | ELECTION_KINDS | RESIGNATION_KINDS | CONTINUATION_KINDS | ORGANIZATION_KINDS | ACTING_KINDS |
             RETROSPECTIVE_KINDS)
# Kinds that never carry a structured date: retrospective records, undated recollections, rules and plans.
UNDATED_KINDS = RETROSPECTIVE_KINDS | {'election_procedure', 'election_recalled_by_holder', 'in_office_recollection'}
COUNTS = {
    'sources_claims': (54, 111),
    'holder_never': (18, 93),
    'categories': (42, 14, 24, 3, 1, 9, 18),
    'archived_diet': (39, 15),
}
# Dates that are never a holder's attested_on, start or end: elections, notices, filings, counts, conventions,
# resignations, acting arrangements, recollections, leads' dates, month ends and later or implicit attestations.
NEVER_HOLDER_DATE = {
    '1990-01-01', '1990-03-05', '1990-08-22', '1990-10-19', '1991-01-28', '1991-07-31', '1991-08-07', '1992-11-04',
    '1993-09-24', '1993-10-07', '1994-05-12', '1994-06-30', '1994-10-18', '1995-02-06', '1995-05-19', '1995-10-11',
    '1996-01-19', '1996-01-24', '1996-01-26', '1996-03-31', '1996-09-28', '1996-12-22', '1996-12-31', '1998-01-08',
    '1998-01-10', '1998-08-30', '2003-11-11', '2003-11-13', '2003-11-26', '2003-12-13', '2009-12-04', '2012-01-20',
    '2013-08-01', '2013-08-05', '2013-08-29', '2013-09-05', '2013-09-27', '2013-10-14', '2013-10-26', '2013-11-01',
    '2013-11-06', '2015-11-26', '2016-07-14', '2016-09-01', '2018-01-12', '2018-01-26', '2018-02-24', '2018-03-02',
    '2020-02-22', '2020-02-23', '2020-10-22', '2020-11-14', '2020-11-17', '2022-03-19', '2022-03-20', '2024-02-23',
    '2026-02-18', '2026-03-04', '2026-03-23', '2026-04-06', '2026-04-29', '2026-07-29',
}
# On each transition day these events stay separate claims.
TRANSITIONS = {
    '1996-11-30': ['assumption_reported_to_party_organ', 'in_office_attestation'],
    '1996-12-22': ['convention_approval', 'in_office_continuation_attestation'],
    '2000-01-21': ['in_office_attestation', 'result_declared'],
    '2003-11-13': ['in_office_continuation_attestation', 'resignation_accepted', 'resignation_intent_announced', 'resignation_recalled_by_holder'],
    '2003-11-15': ['assumption_stated', 'in_office_attestation', 'in_office_continuation_attestation', 'joint_plenary_selection', 'resignation_submitted'],
    '2003-12-13': ['convention_approval', 'in_office_continuation_attestation', 'predecessor_referred_to_as_former'],
    '2009-12-04': ['chair_election_notice', 'result_declared'],
    '2012-01-20': ['chair_election_notice', 'result_declared'],
    '2013-07-25': ['end_of_office_stated', 'in_office_continuation_attestation', 'resignation_accepted', 'resignation_intent_announced'],
    '2013-09-27': ['candidacy_filing', 'chair_election_notice'],
    '2013-10-14': ['chair_vote_count', 'result_declared'],
    '2015-11-26': ['candidacy_intent_announced', 'in_office_continuation_attestation'],
    '2016-09-01': ['in_office_continuation_attestation', 'resignation_withdrawn'],
    '2018-01-26': ['chair_election_notice', 'result_declared'],
    '2020-02-22': ['in_office_continuation_attestation', 'result_declared'],
    '2022-01-14': ['candidacy_filing', 'chair_election_notice', 'in_office_attestation', 'result_declared'],
    '2023-12-01': ['chair_election_notice', 'in_office_attestation', 'result_declared'],
    '2026-03-04': ['candidacy_filing', 'chair_election_notice', 'in_office_continuation_attestation'],
    '2026-03-23': ['chair_election_notice', 'chair_vote_count'],
    '2026-04-06': ['chair_vote_count', 'result_declared'],
}
# Secondary sources and leads that must never be a source URL here (the Prime Minister's replies and a former chair's
# statements as a minister are Diet leads, not party officers' statements).
LEAD_URL_MARKERS = ('wikipedia', 'britannica', 'kotobank', 'nikkei', 'asahi.com', 'yomiuri', 'mainichi', 'nhk.or.jp',
                    'jiji.com', 'sankei', '111805254X00419900305', '112005254X00719910128', '112105254X00219910807',
                    '112505254X00219921104', '112815261X00219931007', '112905254X01819940512', '113615254X00319960126',
                    '113605254X02919960528', '111805261X00219900308', 'sdp.or.jp/2026leader', 'information/touhushusen',
                    'information/260429leaderspeech', 'information/260410notice', 'sdp-paper/kettou')
LEAD_REPORT_MARKERS = ('111805254X00419900305', '112105254X00219910807', '112815261X00219931007', '112905254X01819940512',
                       '113615254X00319960126', '111805261X00219900308', '2026leader', 'touhushusen')
# Response shapes a server can generate per request, cache-busting queries, open searches and live listing pages.
PER_REQUEST_PATTERNS = ('cb=', '_cb', 'cbx=', 'any=', 'token=', 'sessionid', 'X-Amz-', 'Signature=', 'searchResult',
                        'opensearch', 'cdx/search', '?q=', '&q=', 'speaker=', 'from=', 'until=', 'startRecord', '?s=',
                        'fbclid', 'nocache', 'wp-json')
# Ids withdrawn, merged or renamed during integration, and date fields the packet never uses; none may remain.
STALE_IDS = ('"jp_diet_', 'jp_sdp_diet_hc_yosan_', 'jp_sdp_doi_named_chair_in_reply_', 'jp_sdp_tanabe_named_chair_in_reply_',
             'jp_sdp_murayama_named_chair_in_reply_', 'jp_sdp_yamahana_resignation_recalled',
             'jp_sdp_murayama_new_chair_referred_19931007', 'jp_sdp_fukushima_in_office_20200222',
             'jp_sdp_doi_leaders_meeting_recalled_19900822', 'jp_sdp_yoshida_assumption_recalled_20131101',
             'jp_sdp_fukushima_uncontested_selection_20200222', 'jp_sdp_tanabe_recent_convention_referred',
             'jp_sdp_doi_profile_chair_assumed_retrospective', 'jp_sdp_doi_profile_resignation_retrospective',
             'attested_period', 'printed_date', 'statement_date', 'printed_range', 'stated_assumption_of_office')
HOLDER_CLAIM_SET = {cid for ids in HOLDER_CLAIMS for cid in ids}
NEVER_HOLDER = tuple(cid for cid in EVENTS if cid not in HOLDER_CLAIM_SET)
ELECTIONS = tuple(cid for cid, (_d, kind, _o) in EVENTS.items() if kind in ELECTION_KINDS)
RESIGNATIONS = tuple(cid for cid, (_d, kind, _o) in EVENTS.items() if kind in RESIGNATION_KINDS)
CONTINUATION = tuple(cid for cid, (_d, kind, _o) in EVENTS.items() if kind in CONTINUATION_KINDS)
ORGANIZATION = tuple(cid for cid, (_d, kind, _o) in EVENTS.items() if kind in ORGANIZATION_KINDS)
ACTING = tuple(cid for cid, (_d, kind, _o) in EVENTS.items() if kind in ACTING_KINDS)
RETROSPECTIVE = tuple(cid for cid, (_d, kind, _o) in EVENTS.items() if kind in RETROSPECTIVE_KINDS)
UNDATED = tuple(cid for cid, (_d, _k, _o) in EVENTS.items() if _d is None)
REPORT = research.RESEARCH / 'japan-sdp-chairs-1990-2026-29.md'
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-29.md'


def load_rows(packet):
    rows = {}
    for source in packet['sources'][EARLIER_SOURCE_COUNT:EARLIER_SOURCE_COUNT + len(NEW_SOURCES)]:
        extract = json.loads((research.ROOT / source['snapshot']['path']).read_text(encoding='utf-8'))
        rows.update({row['claim_id']: row for row in extract['rows']})
    return rows


def holder_day(holder):
    return holder['attested_on'] or holder['from']


def sdp_rules(packet, rows):
    """Rule-based checks that hold without the pinned holder list; raise AssertionError, KeyError or IndexError."""
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    source_of = {c['id']: s['id'] for s in packet['sources'] for c in s['claims']}
    org, = [o for o in packet['organizations'] if o['id'] == ORG_ID]
    assert (org['name'], org['lifecycle'], org['represented_party_ids']) == (
        '社会民主党', {'status': 'unknown', 'from': None, 'until': None,
                   'note': 'Participation at one election does not establish organization birth, dissolution or continuity.'},
        []), 'the renaming and the 2020 split never become a lifecycle or a mapping'
    assert [r['id'] for r in org['roles']] == [ROLE], 'exactly one party role on the SDP observation'
    role = org['roles'][0]
    assert (role['title'], role['kind']) == (T_SDP, 'party_leader')
    assert list(role) == ['id', 'title', 'kind', 'sources', 'claim_ids', 'holder_claims', 'scope_note']
    # No organization or institution is added, and no other role changes identity.
    assert len(packet['organizations']) == 16 and [i['id'] for i in packet['institutions']] == GROUPS + ['jp_prime_minister']
    assert sorted(r['id'] for e in packet['organizations'] + packet['institutions'] for r in e['roles']) == sorted(
        EARLIER_ROLES + [ROLE] + [komeito.ROLE]), 'one new role, and only CLAUDE-C01-31\'s after it'
    assert all(not e['represented_party_ids'] for e in packet['organizations'] + packet['institutions'])
    holders = role['holder_claims']
    previous = ''
    for holder in holders:
        name = holder['name']
        assert name in SURNAMES, name
        assert list(holder) == ['name', 'attested_on', 'from', 'until', 'sources', 'claim_ids', 'note', 'uncertainty']
        dated = [d for d in (holder['attested_on'], holder['from']) if d]
        assert len(dated) == 1, (name, 'a holder is dated by exactly one of attested_on and from')
        day = dated[0]
        assert day > previous, (name, 'holders stay in chronological order')
        previous = day
        assert day not in NEVER_HOLDER_DATE and day <= research.CUTOFF, (name, day)
        if holder['until']:
            assert holder['until'] not in NEVER_HOLDER_DATE and day <= holder['until'] <= research.CUTOFF, name
        assert not set(holder['claim_ids']) & set(NEVER_HOLDER), name
        expected = []
        kinds = []
        for cid in holder['claim_ids']:
            row, claim = rows[cid], claims[cid]
            assert cid in role['claim_ids'] and cid.startswith('jp_sdp'), cid
            assert row['holder_name'] == name and (row['role_id'], row['role_title']) == (ROLE, T_SDP), (name, cid)
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
        if kind in HOLDER_KINDS and name and row['role_title'] == T_SDP:
            assert len(cited) == 1 and cited[0]['name'] == name, cid
        else:
            assert not cited, cid
        if kind in ACTING_KINDS or row['role_title'] != T_SDP:
            assert not any(h['name'] == name and cid in h['claim_ids'] for h in holders), cid
        if kind in UNDATED_KINDS:
            assert 'attested_on' not in claims[cid] and row['attested_on'] is None, cid
        if 'attested_on' in claims[cid]:
            assert claims[cid]['attested_on'] == row['attested_on'] <= research.CUTOFF, cid
        else:
            assert row['attested_on'] is None, cid
        assert not {'period', 'attested_period', 'printed_date', 'statement_date', 'printed_range'} & set(claims[cid]), cid
    # Party office and state office never feed each other; the LDP presidency is a different party's role.
    pm_role = packet['institutions'][-1]['roles'][0]
    assert len(pm_role['holder_claims']) == PM_HOLDER_COUNT
    pm_claims = set(pm_role['claim_ids']) | {c for h in pm_role['holder_claims'] for c in h['claim_ids']} | set(
        packet['institutions'][-1]['claim_ids'])
    pm_sources = set(pm_role['sources']) | {s for h in pm_role['holder_claims'] for s in h['sources']} | set(
        packet['institutions'][-1]['sources'])
    ldp_org, = [o for o in packet['organizations'] if o['id'] == LDP_ORG]
    ldp_role = ldp_org['roles'][0]
    assert len(ldp_role['holder_claims']) == LDP_HOLDER_COUNT
    sdp_claims = set(role['claim_ids']) | {c for h in holders for c in h['claim_ids']}
    sdp_sources = set(role['sources']) | {s for h in holders for s in h['sources']}
    assert not sdp_claims & pm_claims and not sdp_sources & pm_sources
    assert not any(c.startswith('jp_sdp') for c in pm_claims) and not any(s.startswith('jp_sdp') for s in pm_sources)
    assert all(c.startswith('jp_sdp') for c in sdp_claims) and all(s.startswith('jp_sdp') for s in sdp_sources)
    for entry in packet['organizations'] + packet['institutions']:
        if entry is not org:
            assert not set(entry['claim_ids']) & sdp_claims and not set(entry['sources']) & sdp_sources, entry['id']
            for other in entry['roles']:
                assert not set(other['claim_ids']) & sdp_claims and not set(other['sources']) & sdp_sources, other['id']
                assert not {c for h in other['holder_claims'] for c in h['claim_ids']} & sdp_claims, other['id']
    assert set(role['claim_ids']) <= set(org['claim_ids']) and set(role['sources']) <= set(org['sources'])


def sdp_invariants(packet, rows):
    """The rules plus the exact pinned holders, rows and events this packet intends."""
    sdp_rules(packet, rows)
    org, = [o for o in packet['organizations'] if o['id'] == ORG_ID]
    role = org['roles'][0]
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    got = [(h['name'], h['attested_on'], h['from'], h['until']) for h in role['holder_claims']]
    assert got == HOLDERS, got
    assert [h['claim_ids'] for h in role['holder_claims']] == HOLDER_CLAIMS
    assert [(h['name'], h['from']) for h in role['holder_claims'] if h['from']] == STARTS
    assert [(h['name'], h['until']) for h in role['holder_claims'] if h['until']] == ENDS
    assert {cid: row['holder_name'] for cid, row in rows.items()} == ROW_HOLDERS
    assert {cid: row['role_title'] for cid, row in rows.items() if row['role_title'] != T_SDP} == OTHER_TITLES
    for cid, (day, kind, obs) in EVENTS.items():
        assert claims[cid].get('attested_on') == day, cid
        assert (rows[cid]['attested_on'], rows[cid]['event_kind'], rows[cid]['review_observation']) == (day, kind, obs), cid


class JapanSdpChairsTests(unittest.TestCase):
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
        self.assertEqual(order[EARLIER_SOURCE_COUNT + len(NEW_SOURCES):], komeito.NEW_SOURCES + jcp42.NEW_SOURCES)
        self.assertFalse([sid for sid in order[:EARLIER_SOURCE_COUNT] if sid in RESPONSES or sid.startswith('jp_sdp')])
        self.assertEqual((len(ids['entries']), len(ids['roles'])), (24, 7))
        self.assertEqual((len(self.packet['organizations']), len(self.packet['institutions'])), (16, 8))
        self.assertEqual(self.new_claims, NEW_CLAIMS)
        self.assertEqual(set(EVENTS), set(self.rows))
        # Every new claim is either a holder claim or a claim that never feeds a holder, never both.
        self.assertFalse(HOLDER_CLAIM_SET & set(NEVER_HOLDER))
        self.assertEqual(HOLDER_CLAIM_SET | set(NEVER_HOLDER), set(self.new_claims))
        self.assertEqual((len(HOLDER_CLAIM_SET), len(NEVER_HOLDER)), COUNTS['holder_never'])
        self.assertEqual((len(ELECTIONS), len(RESIGNATIONS), len(CONTINUATION), len(ORGANIZATION), len(ACTING),
                          len(RETROSPECTIVE), len(UNDATED)), COUNTS['categories'])
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
        # Eleven reviewed observations, each reported and each carrying rows; at most ten people hold the office.
        observations = re.findall(r'^### (SDP-CHAIR-\d\d)\b', self.report, re.M)
        self.assertEqual(observations, REVIEW)
        self.assertEqual({row['review_observation'] for row in self.rows.values()}, set(REVIEW))
        self.assertEqual([self.rows[ids_[0]]['review_observation'] for ids_ in HOLDER_CLAIMS], HOLDER_REVIEW)
        people = list(dict.fromkeys(h[0] for h in HOLDERS))
        self.assertEqual(people, ['土井たか子', '田邊誠', '山花貞夫', '村山富市', '福島瑞穂', '吉田忠智', '又市征治'])
        self.assertLessEqual(len(people), 10)
        for stale in STALE_IDS:
            self.assertNotIn(stale, self.raw, stale)
            for extract in self.extracts.values():
                self.assertNotIn(stale, json.dumps(extract, ensure_ascii=False), stale)

    def test_holders_are_exactly_as_intended(self):
        sdp_invariants(self.packet, self.rows)
        for holder in self.role['holder_claims']:
            if holder['from']:
                self.assertTrue(holder['note'].startswith('From 15 November 2003: '), holder['name'])
                self.assertTrue(holder['uncertainty'].startswith('No end is stated'), holder['name'])
            else:
                self.assertTrue(holder['note'].startswith('Observed on '), holder['name'])
            if holder['until']:
                self.assertIn('Until 25 July 2013: ', holder['note'])
                self.assertTrue(holder['uncertainty'].startswith('The end is stated by her own words'), holder['name'])
            elif not holder['from']:
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
        for cid in HOLDER_CLAIM_SET:
            if self.rows[cid]['holder_name'] == '福島瑞穂' and 'みずほ' in self.claims[cid]['text']:
                self.assertEqual(ROW_HOLDERS[cid], '福島瑞穂')
        self.assertIn('福島みずほ新党首', self.claims['jp_sdp_fukushima_new_leader_news_item_20200228']['text'])
        self.assertIn('田邊委員長', self.claims['jp_sdp_tanabe_named_chair_by_secretary_general_19910820']['text'])
        # Two holders of this period also held state office (村山富市 as Prime Minister; 福島瑞穂 as a minister); only
        # party-office wording is cited, and the prime-ministership's own holders are untouched.
        pm_role = self.packet['institutions'][-1]['roles'][0]
        self.assertIn('村山富市', {h['name'] for h in pm_role['holder_claims']})
        for cid in HOLDER_CLAIM_SET:
            self.assertNotRegex(self.claims[cid]['text'], r'内閣総理大臣に(任命|指名)|首班指名|組閣', cid)

    def test_starts_ends_and_claims_that_never_feed_a_holder(self):
        claims, rows = self.claims, self.rows
        # Elections, notices, filings, counts, declarations and resignations are distinct claims on each transition day.
        for day, expected in TRANSITIONS.items():
            kinds = sorted({rows[cid]['event_kind'] for cid, (d, _k, _o) in EVENTS.items() if d == day})
            for kind in expected:
                self.assertIn(kind, kinds, (day, kind))
        for cid in ELECTIONS + RESIGNATIONS + CONTINUATION + ORGANIZATION + ACTING + RETROSPECTIVE:
            self.assertNotIn(cid, HOLDER_CLAIM_SET, cid)
        for cid in RESIGNATIONS:
            self.assertRegex(claims[cid]['uncertainty'], r'(?i)never an until|not an end|no end', cid)
        for cid in CONTINUATION:
            self.assertRegex(claims[cid]['uncertainty'], r'(?i)never a holder', cid)
        for cid in ELECTIONS + ACTING:
            self.assertRegex(claims[cid]['uncertainty'], r'(?i)never a holder', cid)
        for cid in ORGANIZATION:
            self.assertRegex(claims[cid]['uncertainty'], r'(?i)organization only', cid)
            self.assertIsNone(rows[cid]['holder_name'], cid)
        for cid in RETROSPECTIVE:
            self.assertRegex(claims[cid]['uncertainty'], r'(?i)never a holder boundary', cid)
        for cid in UNDATED:
            self.assertNotIn('attested_on', claims[cid], cid)
            self.assertRegex(claims[cid]['uncertainty'], r'(?i)no structured date', cid)
        # The one stated start is her own statement of the day she became leader; the one stated end is her own words.
        start = [cid for cid, row in rows.items() if row['event_kind'] in FROM_KINDS]
        self.assertEqual(start, ['jp_sdp_fukushima_assumption_stated_20031115'])
        self.assertIn('十一月十五日', claims[start[0]]['text'])
        self.assertIn('新党首になりました', claims[start[0]]['text'])
        end = [cid for cid, row in rows.items() if row['event_kind'] in END_KINDS]
        self.assertEqual(end, ['jp_sdp_fukushima_end_stated_20130725'])
        self.assertIn('本日で辞任する', claims[end[0]]['text'])
        # 就任 wording in a heading or a statement without a day dates an observation but is never a start.
        for cid in ('jp_sdp_yamahana_self_stated_chair_19930125', 'jp_sdp_fukushima_in_office_inaugural_greeting_20031115',
                    'jp_sdp_mataichi_in_office_inaugural_greeting_20180225', 'jp_sdp_fukushima_new_leader_news_item_20200228'):
            self.assertEqual(rows[cid]['event_kind'], 'in_office_attestation', cid)
            self.assertIn('就任', claims[cid]['text'], cid)
        # A recollected takeover day that conflicts with the holder's own earlier statement is never a start.
        self.assertEqual(rows['jp_sdp_yoshida_takeover_recalled_20131101']['event_kind'], 'assumption_recalled_by_holder')
        yoshida = next(h for h in self.role['holder_claims'] if h['name'] == '吉田忠智')
        self.assertEqual((yoshida['attested_on'], yoshida['from']), ('2013-10-24', None))
        self.assertIn('conflict', yoshida['uncertainty'])
        # Acting service is never a holder: the one acting row names no holder and carries the acting title.
        self.assertEqual(ACTING, ('jp_sdp_mataichi_acting_designated_20130801',))
        self.assertEqual((rows[ACTING[0]]['holder_name'], rows[ACTING[0]]['role_title']), (None, ACTING_TITLE))
        self.assertEqual({h['name'] for h in self.role['holder_claims'] if h['attested_on'] and '2013-08' in h['attested_on']}, set())
        # Retrospective records come only from the party's profile and statements, and never carry a date.
        retro_sources = sorted({self.claim_source[cid] for cid in RETROSPECTIVE})
        self.assertEqual(retro_sources, sorted(['jp_sdp_doi_profile_2000_retrospective', 'jp_sdp_statement_20th_anniversary_20160119',
                                                'jp_sdp_statement_murayama_obituary_20251017',
                                                'jp_sdp_fukushima_resignation_kaiken_20130725',
                                                'jp_sdp_fukushima_eighth_convention_20031213']))
        for sid in ('jp_sdp_doi_profile_2000_retrospective', 'jp_sdp_statement_20th_anniversary_20160119',
                    'jp_sdp_statement_murayama_obituary_20251017'):
            self.assertTrue(self.sources[sid]['source_type'].endswith('_retrospective'), sid)
        # The renaming is a claim about the organization, recorded without a structured date, never a merge.
        for cid in ('jp_sdp_renaming_retrospective_anniversary_statement', 'jp_sdp_murayama_obituary_renaming_retrospective'):
            self.assertEqual(rows[cid]['event_kind'], 'retrospective_renaming_record')
            self.assertIn('社会民主党', claims[cid]['text'])
            self.assertIn('日本社会党', claims[cid]['text'])
        self.assertIn('１９９６年１月１９日', claims['jp_sdp_renaming_retrospective_anniversary_statement']['text'])
        self.assertIn('organization only', claims['jp_sdp_split_decision_20201114']['uncertainty'])
        scope = self.role['scope_note']
        self.assertTrue(scope.startswith('CLAUDE-C01-29 adds this role'))
        for phrase in ('never feed a holder', 'procedure only, never a date', "outgoing chair's last day",
                       'never read the prime-ministership (jp_pm)', 'claims about the organization only',
                       'does not merge 日本社会党 with this observation', 'only 福島瑞穂'):
            self.assertIn(phrase, scope)
        unresolved = self.org['coverage']['unresolved']
        self.assertEqual(len(unresolved), ORG_EARLIER_UNRESOLVED + 1)
        self.assertTrue(unresolved[-1].startswith('SDP chairs 1990-2026 (CLAUDE-C01-29)'))
        packet_unresolved = self.packet['coverage']['unresolved']
        self.assertEqual(sum('CLAUDE-C01-29' in u for u in packet_unresolved), 1)
        self.assertTrue(packet_unresolved[-3].startswith('SDP chairs 1990-2026 (CLAUDE-C01-29)'))
        self.assertTrue(packet_unresolved[-4].startswith('LDP presidents 1990-2009 (CLAUDE-C01-18'))
        self.assertTrue(packet_unresolved[-2].startswith('Komeito representatives 1990-2026 (CLAUDE-C01-31)'))
        self.assertTrue(packet_unresolved[-1].startswith('JCP chairs and DPFP representatives 1990-2026 (CLAUDE-C01-42)'))

    def test_retrospective_election_count_does_not_invent_additional_wins(self):
        claim = self.claims['jp_sdp_fukushima_tenure_record_2003_2013']
        self.assertEqual(claim['text'],
                         "The page adds that 福島党首 became leader in November 2003 as 土井たか子党首's successor "
                         "and won five consecutive uncontested leader elections.")
        self.assertIn('does not establish five additional re-elections after the initial election', claim['uncertainty'])
        self.assertIn('the contemporaneous January 2012 record calls that election her fifth', claim['uncertainty'])
        self.assertNotIn('attested_on', claim)
        self.assertNotIn(claim['id'], HOLDER_CLAIM_SET)
        self.assertIn('five consecutive uncontested leader elections', self.report)
        self.assertNotIn('re-elected unopposed five times after 2003', self.report)
        self.assertNotIn('counts five uncontested re-elections', self.report)

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
                           'identity-encoded body', 'by this packet at', '30 minutes or more apart'):
                self.assertIn(phrase, extract['provenance_note'], (sid, phrase))
            self.assertNotRegex(extract['provenance_note'], r'(?<!\d)0 minutes apart', sid)
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
            self.assertRegex(source['original_url'], r'^https?://(www\.|www5\.)?sdp\.or\.jp/')
            self.assertEqual(extract['archive_capture_utc'].replace('-', '').replace(':', '').replace('T', '').rstrip('Z'), stamp)
            self.assertIn('Raw Internet Archive capture', extract['provenance_note'])
            self.assertRegex(extract['source_character_encoding'], r'^(UTF-8|Shift_JIS|EUC-JP|ISO-2022-JP|iso2022_jp_ext)')
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
            # No live party page is recorded: every sdp.or.jp response is a fixed pre-cutoff capture.
            self.assertFalse(parts.hostname.endswith('sdp.or.jp'), url)
        # One response, one source: none of this packet's URLs repeats another source's URL.
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
        for marker in ('wikipedia', 'britannica', 'kotobank', 'nikkei', 'asahi.com', 'yomiuri', 'mainichi', 'nhk.or.jp'):
            self.assertNotIn(marker, lowered, marker)
        leads, added = self.section('Leads not imported'), self.section('Sources added')
        for marker in LEAD_REPORT_MARKERS:
            self.assertIn(marker, leads, marker)
            self.assertNotIn(marker, added, marker)

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

        def ldp_role(packet):
            return next(o for o in packet['organizations'] if o['id'] == LDP_ORG)['roles'][0]

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

        index = {(h[0], h[1] or h[2]): i for i, h in enumerate(HOLDERS)}
        doi90, tanabe, yamahana, murayama = (index[('土井たか子', '1990-04-06')], index[('田邊誠', '1991-08-20')],
                                             index[('山花貞夫', '1993-01-25')], index[('村山富市', '1994-10-13')])
        doi96, doi00 = index[('土井たか子', '1996-11-30')], index[('土井たか子', '2000-01-21')]
        fukushima03, fukushima12 = index[('福島瑞穂', '2003-11-15')], index[('福島瑞穂', '2012-01-24')]
        yoshida, mataichi = index[('吉田忠智', '2013-10-24')], index[('又市征治', '2018-02-25')]
        fukushima20, fukushima26 = index[('福島瑞穂', '2020-02-28')], index[('福島瑞穂', '2026-04-08')]
        validator_cases = [
            (lambda p: source(p, 'jp_sdp_diet_hr_budget_19900406')['snapshot'].update(sha256='0' * 64), 'checksum mismatch'),
            (lambda p: source(p, 'jp_sdp_leader_election_2026_rerun_result_20260406')['snapshot'].update(bytes=1), 'checksum mismatch'),
            (lambda p: source(p, 'jp_sdp_doi_profile_2000_retrospective')['snapshot'].update(path=REPORT.as_posix()), 'escapes'),
            (lambda p: holder(p, fukushima26).update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: claim(p, 'jp_sdp_fukushima_in_office_kaiken_20260729').update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: holder(p, fukushima12).update(until='2011-01-01', attested_on=None, **{'from': '2012-01-24'}),
             'Reversed historical interval'),
            (lambda p: holder(p, yoshida)['claim_ids'].append('jp_sdp_mataichi_in_office_inaugural_greeting_20180225'), 'cited source'),
            (lambda p: role(p)['claim_ids'].append('jp_sdp_does_not_exist'), 'Unknown'),
        ]
        for change, message in validator_cases:
            with self.subTest(message=message), self.assertRaisesRegex(ValueError, message):
                self.validate(mutated(change))

        def extra_holder(name, day, cid, start=False):
            return {'name': name, 'attested_on': None if start else day, 'from': day if start else None, 'until': None,
                    'sources': [self.claim_source[cid]], 'claim_ids': [cid], 'note': 'x', 'uncertainty': 'x'}

        rule_cases = [
            # A successor's election, selection, attestation or start used as an end.
            ('successor attestation used as an end (土井 1990)', lambda p: holder(p, doi90).update(until='1991-08-20')),
            ('successor attestation used as an end (田邊)', lambda p: holder(p, tanabe).update(until='1993-01-25')),
            ('successor start used as an end (土井 2000)', lambda p: holder(p, doi00).update(until='2003-11-15')),
            ('successor election used as an end (吉田)', lambda p: holder(p, yoshida).update(until='2018-01-26')),
            ('successor attestation cited as an end (又市)',
             cite(mataichi, 'jp_sdp_fukushima_new_leader_news_item_20200228', until='2020-02-28')),
            ('announced resignation cited as an end (土井 2000)',
             cite(doi00, 'jp_sdp_doi_resignation_intent_20031113', until='2003-11-13')),
            ('accepted resignation cited as an end (土井 2000)',
             cite(doi00, 'jp_sdp_doi_resignation_accepted_standing_committee_20031113', until='2003-11-13')),
            ('predecessor reference cited as an end (福島 2012)', cite(fukushima12, 'jp_sdp_fukushima_former_20130805', until='2013-08-05')),
            ('retrospective handover cited as an end (村山)',
             cite(murayama, 'jp_sdp_murayama_obituary_handover_to_doi_retrospective', until='1996-12-22')),
            ('end dropped from the 2012 observation', lambda p: holder(p, fukushima12).update(until=None)),
            # An election, notice, convention or recollection used as a start.
            ('election date used as a start (吉田)', lambda p: holder(p, yoshida).update({'from': '2013-10-14', 'attested_on': None})),
            ('election date used as a start (又市)', lambda p: holder(p, mataichi).update({'from': '2018-01-26', 'attested_on': None})),
            ('election date used as a start (福島 2020)', lambda p: holder(p, fukushima20).update({'from': '2020-02-22', 'attested_on': None})),
            ('run-off count used as a start (福島 2026)', lambda p: holder(p, fukushima26).update({'from': '2026-04-06', 'attested_on': None})),
            ('convention approval used as a start (土井 1996)', lambda p: holder(p, doi96).update({'from': '1996-12-22', 'attested_on': None})),
            ('recollected takeover used as a start (吉田)',
             cite(yoshida, 'jp_sdp_yoshida_takeover_recalled_20131101', **{'from': '2013-11-01', 'attested_on': None})),
            ('就任 without a day used as a start (山花)', lambda p: holder(p, yamahana).update({'from': '1993-01-25', 'attested_on': None})),
            ('stated start dropped (福島 2003)', lambda p: holder(p, fukushima03).update({'from': None, 'attested_on': '2003-11-15'})),
            ('election claim cited by a holder (吉田)', cite(yoshida, 'jp_sdp_yoshida_result_declared_20131014')),
            ('convention approval cited by a holder (福島 2003)', cite(fukushima03, 'jp_sdp_fukushima_convention_approval_20031213')),
            ('filing cited by a holder (福島 2026)', cite(fukushima26, 'jp_sdp_leader_election_2026_filings_20260304')),
            ('retrospective record cited by a holder (土井 1990)', cite(doi90, 'jp_sdp_doi_profile_chair_from_september_1986')),
            ('organization claim cited by a holder (村山)', cite(murayama, 'jp_sdp_renamed_party_named_by_secretary_general_19960124')),
            ('named attestation left off its holder (福島 2003)',
             lambda p: holder(p, fukushima03)['claim_ids'].remove('jp_sdp_fukushima_in_office_inaugural_greeting_20031115')),
            # Acting, interim or continued service added as a holder.
            ('acting leader added as a holder (2013)', lambda p: role(p)['holder_claims'].insert(
                mataichi, extra_holder('又市征治', '2013-08-01', 'jp_sdp_mataichi_acting_designated_20130801'))),
            ('continuation attestation added as a holder (村山 1995)', lambda p: role(p)['holder_claims'].insert(
                murayama + 1, extra_holder('村山富市', '1995-02-06', 'jp_sdp_murayama_self_stated_chair_19950206'))),
            ('incumbent before the vote added as a holder (福島 2026-03-04)', lambda p: role(p)['holder_claims'].insert(
                fukushima26, extra_holder('福島瑞穂', '2026-03-04', 'jp_sdp_fukushima_incumbent_20260304'))),
            # A cross-role or cross-institution holder.
            ('jp_pm claim cited by a party holder (村山)', lambda p: holder(p, murayama)['claim_ids'].append(
                pm_role(p)['holder_claims'][4]['claim_ids'][0])),
            ('jp_pm holder moved onto the party role', lambda p: role(p)['holder_claims'].insert(
                murayama + 1, copy.deepcopy(next(h for h in pm_role(p)['holder_claims'] if h['name'] == '村山富市')))),
            ('party holder moved onto the prime-ministership', lambda p: pm_role(p)['holder_claims'].append(
                copy.deepcopy(holder(p, murayama)))),
            ('party claim moved onto the prime-ministership', lambda p: pm_role(p)['claim_ids'].append(
                'jp_sdp_murayama_self_stated_chair_19941013')),
            ('party claim moved onto the LDP presidency', lambda p: ldp_role(p)['claim_ids'].append(
                'jp_sdp_doi_named_chair_by_secretary_general_19900406')),
            ('party role copied onto an institution', lambda p: p['institutions'][0]['roles'].append(copy.deepcopy(role(p)))),
            ('party role copied to another party', lambda p: next(o for o in p['organizations'] if o['id'] == CDP_ORG)[
                'roles'].append(dict(copy.deepcopy(role(p)), id='jp_sdp_chair_copy'))),
            ('second SDP role', lambda p: org(p)['roles'].append(dict(copy.deepcopy(role(p)), id='jp_sdp_deputy_leader'))),
            ('new institution', lambda p: p['institutions'].append(dict(copy.deepcopy(p['institutions'][-1]), id='jp_sdp_office'))),
            ('renaming turned into a lifecycle boundary', lambda p: org(p)['lifecycle'].update({'from': '1996-01-19'})),
            ('split turned into a mapping', lambda p: org(p)['represented_party_ids'].append('Japan/jp_jsp')),
            # The order and the structured dates.
            ('holders out of order', lambda p: role(p)['holder_claims'].reverse()),
            ('retrospective renaming given a structured date', lambda p: claim(
                p, 'jp_sdp_renaming_retrospective_anniversary_statement').update(attested_on='1996-01-19')),
            ('beyond-cutoff attestation', lambda p: holder(p, fukushima26).update(attested_on='2026-09-08')),
        ]
        sdp_rules(self.packet, self.rows)
        sdp_invariants(self.packet, self.rows)
        for label, change in rule_cases:
            with self.subTest(label=label):
                packet = mutated(change)
                with self.assertRaises((AssertionError, KeyError, IndexError, ValueError, StopIteration)):
                    sdp_rules(packet, self.rows)
                with self.assertRaises((AssertionError, KeyError, IndexError, ValueError, StopIteration)):
                    sdp_invariants(packet, self.rows)
        # Row-level mutations: an unnamed notice given a holder name, an acting or retrospective row relabelled.
        for cid, field, value in (('jp_sdp_leader_election_2026_notice_20260304', 'holder_name', '福島瑞穂'),
                                  ('jp_sdp_mataichi_acting_designated_20130801', 'event_kind', 'in_office_attestation'),
                                  ('jp_sdp_mataichi_acting_designated_20130801', 'role_title', T_SDP),
                                  ('jp_sdp_doi_profile_resigned_july_1991', 'event_kind', 'end_of_office_stated'),
                                  ('jp_sdp_fukushima_resignation_intent_20130725', 'event_kind', 'end_of_office_stated')):
            rows = copy.deepcopy(self.rows)
            rows[cid][field] = value
            with self.subTest(row=cid, field=field), self.assertRaises(AssertionError):
                sdp_invariants(self.packet, rows)
        # Event collapses and re-dating are caught by the pinned events.
        for cid, day in (('jp_sdp_leader_election_2026_rerun_count_20260406', '2026-04-08'),
                         ('jp_sdp_leader_election_2013_notice_20130927', '2013-10-14'),
                         ('jp_sdp_doi_resignation_intent_20031113', '2003-11-15'),
                         ('jp_sdp_fukushima_uncontested_election_20200222', '2020-02-28')):
            with self.subTest(collapsed=cid), self.assertRaises(AssertionError):
                sdp_invariants(mutated(lambda p: claim(p, cid).update(attested_on=day)), self.rows)

    def test_report_and_handoff_close_no_parent_gate(self):
        self.assertIn('ready_for_review', self.report)
        table = self.section('Outcome')
        for number in range(1, 12):
            row, = [line for line in table.splitlines() if line.startswith(f'| SDP-CHAIR-{number:02d} ')]
            self.assertRegex(row, r'\*\*(Accepted in part|Not established):\*\*')
        for marker in ('C01', 'C06', 'S23', 'WC1', 'CP1'):
            self.assertNotRegex(self.report, rf'\b{marker}\b[^.\n]*\bis (now )?complete\b')
        notes = self.section('Integration notes')
        for text in ('2d65b680', 'f3514fc6', 'research-index.json', 'only file', 'test_japan_research_s10d.py',
                     'test_japan_prime_ministers_c01_12.py', 'test_japan_prime_ministers_c01_13.py',
                     'test_japan_ldp_presidents_c01_18.py', 'test_campaign_census'):
            self.assertIn(text, notes)
        identities = self.section('Response identities and stability checks')
        for sid, (size, sha) in RESPONSES.items():
            self.assertIn(f'`{sid}`', identities, sid)
            self.assertIn(sha[:12], identities, sid)
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        for text in ('ready_for_review', REPORT.name, 'claude/c01-jp-29', '2d65b680', 'test_japan_sdp_chairs_c01_29.py'):
            self.assertIn(text, handoff)
        index = research.build()
        country = next(p for p in index['countries'] if p['nation'] == 'Japan')
        self.assertFalse(country['country_census_complete'])
        self.assertIsNone(country['unrepresented_organization_count'])
        self.assertEqual((country['institution_observations'], country['role_observations']), (8, 7))
        self.assertFalse(index['c01_complete'])


if __name__ == '__main__':
    unittest.main()
