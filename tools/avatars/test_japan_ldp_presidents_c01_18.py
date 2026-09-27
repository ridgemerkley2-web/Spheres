"""CLAUDE-C01-18: the Liberal Democratic Party's presidents (総裁), 1990-2009, are a party office kept apart from the
prime-ministership. Each party election or selection, notice, candidacy, declaration and report of results, statement of
assumption and resignation stays a separate claim; a holder is dated by same-day party records or the president's own
statement of the office, has a start only where a source states the day office was assumed, and has no end, because no
source reviewed states the day a presidency ended."""
import copy
import hashlib
import json
import re
import unittest
from urllib.parse import urlsplit

import campaign_research as research

ORG_ID = 'jp_sangiin_pr_2025_13'
ROLE = 'jp_ldp_party_president'
T_LDP = '総裁 — party president'
PM = 'jp_pm'
# The packet's sources before this packet: the original S10d intake (7), CLAUDE-C01-12 (119) and CLAUDE-C01-13 (211).
EARLIER_SOURCE_COUNT = 337
EARLIER_CLAIMS = ['jp_ishiba_president_20240927', 'jp_takaichi_president_20251004']
EARLIER_SOURCES = ['jp_ldp_ishiba_elected_2024', 'jp_ldp_takaichi_elected_2025']
ORG_EARLIER_CLAIMS = ['jp_pr2025_submission_13'] + EARLIER_CLAIMS
ORG_EARLIER_SOURCES = ['jp_tokyo_pr_2025'] + EARLIER_SOURCES
GROUPS = ['jp_shugiin_group_20260218_011', 'jp_shugiin_group_20260218_020', 'jp_shugiin_group_20260218_030',
          'jp_shugiin_group_20260218_040', 'jp_shugiin_group_20260218_050', 'jp_shugiin_group_20260218_060',
          'jp_shugiin_group_20260218_070']
PM_HOLDER_COUNT = 30  # CLAUDE-C01-12's fourteen and CLAUDE-C01-13's sixteen, unchanged

# Original response identity recorded in each extract: (bytes, sha256) of the identity-encoded body, fetched as the
# extract's fetch_recipe says. Every source is a raw Internet Archive capture or an official Diet minutes API response.
RESPONSES = {
    'jp_ldp_history_presidents_list':
        (159872, 'dcbd1425ba61bbaf4601b52f68eadbddc0d3f413d15c68c355c3719cb5c1d7b4'),
    'jp_ldp_history_kaifu_era':
        (12460, '627457550f2e9661e79b406a82b8e0df902e4c756f55a4c2b377484e59ab1de4'),
    'jp_ldp_history_miyazawa_era':
        (18264, 'd6c60c8997cb0327ea81d2e923f39670e7f2e4030dddd2a0bf8f5de6a83438a9'),
    'jp_ldp_history_kono_era':
        (18492, '9b2d9bcd7e7e75684dc42e445a6e925bdd62f01d827b175b61bd6d2ff58edbab'),
    'jp_ldp_history_hashimoto_era':
        (19673, 'ade2763655de836831ee1e0f82c827b856227dcbf8a48a563b475d1a617fd567'),
    'jp_ldp_ayumi_2015':
        (3313141, '2924c7dd5ba66d939531b961892d4493ae4e08a60e89de8f521b1c7b72c0f482'),
    'jp_ldp_diet_hc_kessan_19891213':
        (444330, '93db3b6c193e3be2852ff2eea39478805a9097cb4b73d8a73a175710a5e42817'),
    'jp_ldp_diet_hr_yosan_19900322':
        (259890, '17fe1ecb11fdfb068bd8c0c015dc10ccc100082d621250456387c63a0516ee12'),
    'jp_ldp_diet_hc_yosan_19900514':
        (430898, '06ba7990ea37f65bf69344236b4e7722972bae5691a85cfd6d67ee2a1bdda60e'),
    'jp_ldp_diet_hr_honkaigi_19911111':
        (140695, '805ceb503014c89fd20e8cbf360aed8bf95713310125dc4fd71329c12f36a469'),
    'jp_ldp_diet_hc_honkaigi_19911112':
        (136088, 'e8fb61813f4a513ed939253c0809c417d6062dc29c81771781ce350d2250b313'),
    'jp_ldp_diet_hc_honkaigi_19911113':
        (195182, '872f18013cd67564285e66837f1925ce3e028b16a8b9de7d0b5b6166ad023162'),
    'jp_ldp_diet_hr_yosan_19920220':
        (570013, '6341943c7f3da3f918012c93599a8ba2b573043fa88ba9319039eb5ded7f6482'),
    'jp_ldp_diet_hc_yosan_19920331':
        (412347, 'a5a65c742d60f556ba3f416206a5795651dc93a35a7d602a30ffc596969f1573'),
    'jp_ldp_diet_hr_koshokusen_19921130':
        (391976, '04f49975390f7a5ea3ce7ca589941f947685cc6ec99f4d19b629b9e01d2d80a3'),
    'jp_ldp_diet_hr_yosan_19941011':
        (594581, '45b4d370add2b2033734b6d0cc006b9cc0a0241e2f745c6f1afbd6b7e78d7115'),
    'jp_ldp_diet_hr_yosan_19941013':
        (274055, '5b39594057702e159d4c0a8bfa335c340660d2c24e96b5ce915a4201bab359d7'),
    'jp_ldp_diet_hr_wto_19941125':
        (352687, 'f5a58ae98ff98a47ba89852bc352357d0e1f2f8ca160b7c83265f3ed38c6d280'),
    'jp_ldp_diet_hr_yosan_19950131':
        (535128, '3c02d4da0aa7f5469dc01f372319bf9e608e80b5047e6432f968527aed007fe9'),
    'jp_ldp_diet_hr_honkaigi_19951002':
        (114052, '47c45682e2af7e57034c470418b25ec42896617e64b9c9afc20a229650784863'),
    'jp_ldp_diet_hc_honkaigi_19951003':
        (173253, '94f7c6df5f7ebd2d4b5e8baf38a8cec172b2af25cbe3271c732718dea8a011da'),
    'jp_ldp_diet_hr_yosan_19951026':
        (521450, '098ac5caf4cc514304862982de8321da1e92e4c4891f8f9f9f63c865a9474bb7'),
    'jp_ldp_sousai_ayumi_20011109':
        (20913, 'c6caae50dec836685c41d2d6eda5cc483a53609c325b8621b0f33dc302f48c3e'),
    'jp_ldp_sousai98_index':
        (7704, 'cdd05a24e17b2446c322c9753294ed44fb12640eea537740426f0c821dca2a3f'),
    'jp_ldp_sousai98_vote_page1_19980724':
        (3262, '5e1bd2dbbd7a6d13cc13b013290f2c28b534e85c9ae441fd11cf53d1f4950b20'),
    'jp_ldp_sousai98_vote_page3_19980724':
        (4677, 'c976c2e7503e4b91b58d94687d42ceab5e1a379793b64e0452c3236b9ed85492'),
    'jp_ldp_hatsugen_obuchi_new_year_19990101':
        (1400, '764ae3c474cf4a657db4ddf0ad401694d596c6cae2729b617f7a2e1f2b4910a5'),
    'jp_ldp_sousai99_index':
        (14505, '259da6b4b9531bae853b3d172b177e6a493fcdb693d945f352c42e6e76d55df8'),
    'jp_ldp_sousai99_diet_member_vote_19990921':
        (5530, '18a0f64b359159b9ad2f66047921a2691cd75dcda09ea577ec1917dc0bb690c0'),
    'jp_ldp_sousai99_extraordinary_congress_19990922':
        (5397, 'a0cb2029421253efa337dcb3d54d2cbcfcfa693048ddb062deda4a71463f92c7'),
    'jp_ldp_sousai99_obuchi_greeting_19990922':
        (4132, 'a601c30f818c48817aec60bd1f457366116d6fe9a5d7711c72a83c0a8b8bc441'),
    'jp_ldp_sousai99_press_conference_19990922':
        (5784, '3614f3e6d4d6aa84c6d4fdc37babaabec1006a9a0b1da50b732f43340491613e'),
    'jp_ldp_mori_greeting_joint_plenary_20000405':
        (4270, 'c52bf4725103ea43689d9c3f380c7f66c146fe8a98af548f745bd0a0ed70aa5f'),
    'jp_ldp_mori_special_20000414':
        (4592, '2a623445749f4cac8c708898318ea9d6e88ccb4f9b9fa4a83f02c31a801f876f'),
    'jp_ldp_toutaikai67_programme_20010313':
        (6524, 'adc8eda49de0a36b21b4c2b21d5d244aec7fd1b6f5ebd2eeaadc9b000f16b6c1'),
    'jp_ldp_toutaikai67_mori_address_20010313':
        (8889, '57842b32c905b97fd2dc151e5b8c4b42c23dcecdd116d1af911cd4a840bb82c1'),
    'jp_ldp_sousai01_top_20010428':
        (12026, '089f3476d1ea148c5a21be866982889a67f2299c85420b71bd45a54b3d654097'),
    'jp_ldp_sousai01_rules_2001':
        (10188, '38b91baa3d27e2684c6c1fe254da31107a5d6a29157ac0c2806955a2328d7f0b'),
    'jp_ldp_sousai01_joint_plenary_20010424':
        (6779, '2fecf5c7b21092cac7e15ac906b0213ea07fdce8d01524003355603cad07a3dd'),
    'jp_ldp_sousai01_press_conference_20010424':
        (2843, 'e3e64b57bac704a297c9837f5b1bf260179873870fd66116dd57c0d6c666527a'),
    'jp_ldp_koizumi_reappointment_greeting_20010810':
        (1758, 'c3cf6e8daf4dbcc3388f38f6c042849885ea4363ec07d0993f9c16606ea7c095'),
    'jp_ldp_sousai03_notice_20030908':
        (13043, '1cad6d55ea0e574b8cfd4ccbbbeb14e9adc975c3f158c140e3f2ac8d7b9a2226'),
    'jp_ldp_sousai03_result_20030920':
        (13094, '51b5bdbd5ae678f72a2afcb1e6250ca880e3fdcd626144271a47464f24ac693a'),
    'jp_ldp_sousai03_press_conference_20030920':
        (24948, 'be50bc13d5a28c25873f3a88dae776971694636b53ab5d45479c0d0d761563f5'),
    'jp_ldp_diet_sangiin_budget_obuchi_19980821':
        (4628, 'd4182c28479f567e9ec765bce83144371257059728944b2b0babcb885877c804'),
    'jp_ldp_diet_joint_budget_obuchi_19991110':
        (2359, '7771c162bfe100ebcd7be65fa883f1d9040ec6c8d01cf555619b1af786470431'),
    'jp_ldp_diet_shugiin_plenary_mori_20000410':
        (15191, '5c7afccd514dd0112ce8be9d9e1f3cdeff11ca91475b0372fcbd36232d572a2b'),
    'jp_ldp_diet_sangiin_budget_koizumi_20010522':
        (1780, '97c78b04c72df639a9fbc7fcb19416e3f30db4dae7619308e0e4ddff2a86e376'),
    'jp_ldp_diet_shugiin_budget_koizumi_20010914':
        (2915, '15346ccff638d0ca8651c58cd28ae2b037f0d3af691289d3d26958d2a2e48d0d'),
    'jp_ldp_diet_sangiin_plenary_koizumi_20030930':
        (25199, 'eae14fb36cb3803999989aa95ab183b9283733f8d74fabde30d924ce367d72f0'),
    'jp_ldp_news_2006_election_notice_20060908':
        (31058, '72477037604324c984b6664924d3af1094d5d4d8d9cc2eaa372cc98b7719207f'),
    'jp_ldp_sousai06_schedule_20060918':
        (14859, '74df327a08ae9df7141afd4b3b4ee46b91a82176245fef3af902f24f83b037b5'),
    'jp_ldp_news_abe_elected_20060920':
        (31440, '81f2fd2a10897f8d3027e01abc19a5b0e4ac8d437ed5ef6021944e93a348c352'),
    'jp_ldp_news_abe_first_press_conference_20060920':
        (31167, 'ab9c9b65d25e0b1a4fb1000d36c5d97a4001a63645f10354fe2f255890ccda58'),
    'jp_ldp_news_joint_plenary_20060920':
        (31271, 'a439ef39de535d3e6fe227bfd5847d88c9fd7ad49a6951cbf07b8d3b8d6a0fda'),
    'jp_ldp_news_abe_executive_20060925':
        (30948, '5a297bf4531dc3e480b4f5ba5db80e5fb187b6825b69abcfa9bd5a8b335714c1'),
    'jp_ldp_jiyuminshu_2247_notice_20060925':
        (31011, '39921770826282214f015c9281f8d371ffde3156ffd974b0fe7fed18f6531289'),
    'jp_ldp_news_abe_president_joint_plenary_20060926':
        (31192, '512e9f4b17f3e983c7f5155d1d71a578a957094e9be12f64edebede9fd7237be'),
    'jp_ldp_jiyuminshu_2248_notice_20061002':
        (37752, 'd3e1a01ce27e8e2fec8dc7b390a07fec42fad52b750a27998d0579e8741596e9'),
    'jp_ldp_news_abe_resignation_election_method_20070912':
        (29063, '2d7818a848f3bf551ed333120506a5f18ca05b2a65843851c38657605f36c933'),
    'jp_ldp_news_2007_election_schedule_20070913':
        (29390, '461fb04bd1e5470d52dabadb54365381b9d3cdc489cfb8a5478bc6401bf6a6a1'),
    'jp_ldp_news_2007_candidates_filed_20070915':
        (30557, '0cefee039588590e61b6ab2aabe16d941b76bedf5e2d7cc4a0d1a201b53cce8c'),
    'jp_ldp_news_fukuda_elected_20070923':
        (28615, '638239c17e4fcefb5e30856ae70dd78867ab5e2737dc13fef6f640718fa1ec75'),
    'jp_ldp_news_fukuda_first_press_conference_20070923':
        (29407, '369c33ee07067c717081aa914c0817cd01d897a252de357566955031cf9310f8'),
    'jp_ldp_news_fukuda_party_officers_20070924':
        (29533, '6dba64c4ea78b7610e7087062482e0848d73e09a36a214b0144de2cc3fda7429'),
    'jp_ldp_jiyuminshu_2293_notice_20070925':
        (29433, 'c07ce04cdf2b4a38fc7779961e80fa83321aa914752d861b41eb4dfd81395fd2'),
    'jp_ldp_news_joint_plenary_schedule_20080903':
        (27588, 'd8b87c67f6b3968fefca4ece0d3c89f1e381d60bc254d4d6c085bda0f78fc521'),
    'jp_ldp_news_2008_election_notice_20080910':
        (25194, 'dd33bfd974e14742fbcce8d8c1c9f26bc7542726fe579f6cebc2234f4afad9ef'),
    'jp_ldp_news_aso_elected_20080922':
        (29124, '5bc47cf1a3d86077f7e5150b5acf9f3dc1e5538435b8de7b46d9d21c9314bb27'),
    'jp_ldp_news_aso_first_press_conference_20080922':
        (27170, '8d1a1f958bfe40aea1bbe9d3a992f045d35cdccb70f334dbe9ff37a781704b49'),
    'jp_ldp_news_aso_party_officers_20080922':
        (26978, '90cf884b6de85f4c37f1e78e5e0360f2ac75bd4374bf5124216480e89f8ec5cf'),
    'jp_ldp_jiyuminshu_2339_notice_20080924':
        (25552, 'b301771ad6feaf819637c70c8f0449c987221329dab0194fc4e465cf0a7f0bdb'),
    'jp_ldp_news_joint_plenary_schedule_20090908':
        (26822, 'd162ceb56cc3d024a3cf7cc90da2c67c4e6d00793d36b75e273951f470aca6da'),
    'jp_ldp_news_2009_notice_day_20090918':
        (26739, '6f731e44cf6cb0b7f4c1a9d7f3a7db2a9a24daddb6905b5330b83bfb0f15bd6a'),
    'jp_ldp_sousai09_notice_20090918':
        (8088, 'aabfb292795cbfb1b578b5988225d76fe72638dda30d213404dea812e5a5fb23'),
    'jp_ldp_sousai09_schedule_20090921':
        (11987, 'f25547d79cac4819995aaaeca4c7938c4afbd461ac5c1c5395c15a84cbe0a6ab'),
    'jp_ldp_news_index_200909_20091001':
        (27053, '422b8ee220b8f555f560bf26dec6c23acda92d1ab8700bdf330aac6ae633fced'),
    'jp_ldp_sousai09_tanigaki_elected_20090928':
        (8172, 'bfc6602c4a23df348c22bedf4274048e5bccef186e5d6679560467789c7a3ffa'),
    'jp_ldp_sousai09_tanigaki_press_conference_20090928':
        (7959, '8808b85e05e5700c8aef5ce396a7df3d6f27f9665eda137c39a833f3ea2775be'),
    'jp_ldp_news_tanigaki_yamba_20091002':
        (27753, '5be1abc555aed3c45918ae903063a7f1b926a587198b491f2eb05a6cc91b75ae'),
    'jp_ldp_history_tanigaki_era':
        (17970, '751add7caa7136d7614574e311a5b510bf923f0a6e708a4fcaa0ec8a6a033fe9'),
}
# Base32 SHA-1 of each recorded response (for comparison with Internet Archive CDX digests).
SHA1 = {
    'jp_ldp_history_presidents_list': 'WXZ3W6YQ6KUYRALTVYTT7GPRRELLPHTM',
    'jp_ldp_history_kaifu_era': 'HTYSJSICDJA4UTCLTUG7SYJCEDTQ7PFT',
    'jp_ldp_history_miyazawa_era': 'KOCHVRE2MR4R65WDVXWPIRXWPRARVZW6',
    'jp_ldp_history_kono_era': 'FG36PFW33WOB6LZJREYTLI2E54WQZ65S',
    'jp_ldp_history_hashimoto_era': 'S2FJO7FQFQHOF4VB4KO7VRSLUGYY6ZF5',
    'jp_ldp_ayumi_2015': 'WXLQISOJP7736TAPLAW7VSABPVAF7AVX',
    'jp_ldp_diet_hc_kessan_19891213': '7JMNVR2ZF5TQVG52SMWJAC5MYLXS7OXS',
    'jp_ldp_diet_hr_yosan_19900322': '2VJR6PFEAQVNP6ESJGXZ5SQVA6V4BSZV',
    'jp_ldp_diet_hc_yosan_19900514': 'NQKAKV6RER62NVGG4IN526F27Z4SQL5M',
    'jp_ldp_diet_hr_honkaigi_19911111': 'V2Q5I7W2MMQLUKZKAHVYVZA3AKLQLSBG',
    'jp_ldp_diet_hc_honkaigi_19911112': 'AEQGTXZO3K5MW6KM5ZZVR5TNWNWMSLMZ',
    'jp_ldp_diet_hc_honkaigi_19911113': 'QS2ELSFKYIF4P5FL3TQXHMSHGY3XZ24C',
    'jp_ldp_diet_hr_yosan_19920220': 'KPA2XSJYKOTHMAD3LB7E43PWRJSZVWYA',
    'jp_ldp_diet_hc_yosan_19920331': 'UXMETXCIZSY36WTDXM554JOO2X3BZJFI',
    'jp_ldp_diet_hr_koshokusen_19921130': 'WY3HPRNDBC2Z3MCIJZFDFANL7KMZPF7W',
    'jp_ldp_diet_hr_yosan_19941011': 'XG2WKSGHNQL3RPJN4Q3EB77SIPVV4CC6',
    'jp_ldp_diet_hr_yosan_19941013': 'LZFTU7LBLBSKB23ZPSPMYXJYZFMH23YH',
    'jp_ldp_diet_hr_wto_19941125': '65JBNKGTQLW6XL4CTVOF2CAYNILMTXVC',
    'jp_ldp_diet_hr_yosan_19950131': 'NXBBARGZPVKFBYLAB6VIJNLFP3B3PPJN',
    'jp_ldp_diet_hr_honkaigi_19951002': 'WM3T4F6JJCV4BWGYJPDFIT4HICMKGX4Y',
    'jp_ldp_diet_hc_honkaigi_19951003': '772VZJG4YCOMLWNYTJ753V6EPS6LGWDR',
    'jp_ldp_diet_hr_yosan_19951026': '5TEOS674JBPBFGU7QQ6BYD7DM6CLVHQQ',
    'jp_ldp_sousai_ayumi_20011109': '647EOH2H32M3ZELTRI4DM334MOHJACXW',
    'jp_ldp_sousai98_index': 'A5GN3ACAQQY6MTEWY7BZC7ZMIKQYPEKG',
    'jp_ldp_sousai98_vote_page1_19980724': 'WXTGYJLFVYRWVQF3SLXNXHXPDYAFF7UT',
    'jp_ldp_sousai98_vote_page3_19980724': 'AYNKQML6IZFR3JD2PDAXKRX6QGUNB74J',
    'jp_ldp_hatsugen_obuchi_new_year_19990101': 'OWR6XGIKONS3YJ22TZVY5Y4VWX6R2JZE',
    'jp_ldp_sousai99_index': '4JPNUY7WVOVNLRYO6FTOZAELZ34QFTML',
    'jp_ldp_sousai99_diet_member_vote_19990921': 'X25VJK4YZ5NXBAUK3JRGJGQWTGFMUPJQ',
    'jp_ldp_sousai99_extraordinary_congress_19990922': 'J4PJAHLFY6WKFH33LAEBOIDGKPPROP6R',
    'jp_ldp_sousai99_obuchi_greeting_19990922': 'WO2APXZXICQUY5NM3PK6OZM2AAQNZZ7L',
    'jp_ldp_sousai99_press_conference_19990922': 'YJGSY2KAWOINXJSPPAGL3CSULTIIE3NG',
    'jp_ldp_mori_greeting_joint_plenary_20000405': '6J4UVIAR4T4VVOEAWS75JRNPQBXJ2NH2',
    'jp_ldp_mori_special_20000414': 'HJ7QJ6YSWSPOEZYD5J77EFEG4NUGO4X4',
    'jp_ldp_toutaikai67_programme_20010313': 'K6A5KCPPWWG6XGVR23HFJW5ZLCBDETTE',
    'jp_ldp_toutaikai67_mori_address_20010313': 'UTJWI42BCSKJPB3IPJX5GSOBFO2SQIMJ',
    'jp_ldp_sousai01_top_20010428': 'ZXEZ4RZ3JPIJV4PGXTHCM5GEV3W5MOO2',
    'jp_ldp_sousai01_rules_2001': 'TENFGH36HAVDNBUGA65QVZ7IVZBVQ7DO',
    'jp_ldp_sousai01_joint_plenary_20010424': 'BFXU2E7LIRQEC6LRAOHDKHADQF33HOFG',
    'jp_ldp_sousai01_press_conference_20010424': '72WDQQ6GU6LGGVQXGA6KCS2BSVFA3AUY',
    'jp_ldp_koizumi_reappointment_greeting_20010810': 'V2N2QC6MVDTNECXMZSRHTTCCYNG5FOTL',
    'jp_ldp_sousai03_notice_20030908': 'KXHE67WRZTVDAVFYNZSY2L434KWIXVS3',
    'jp_ldp_sousai03_result_20030920': 'LLIREWFHGKUJY47Z7WRSYXVXPLZ5RQTI',
    'jp_ldp_sousai03_press_conference_20030920': 'IV7DNMBKCZWHK7PSFJTJJQFRUJQOPKWE',
    'jp_ldp_diet_sangiin_budget_obuchi_19980821': 'SZKRTSKABRJ5V6MUPWFI45Q7W6ZL5UMV',
    'jp_ldp_diet_joint_budget_obuchi_19991110': '6MMHUAVSAAQV4OQIK7W5VBNJ4MTC2OL7',
    'jp_ldp_diet_shugiin_plenary_mori_20000410': 'E6VS7UQBYTB4SQTMNKPIMUKH3NQMFQ3L',
    'jp_ldp_diet_sangiin_budget_koizumi_20010522': 'QXUMRBSCVZWIEHEB37MZDU3NXWZNRXTP',
    'jp_ldp_diet_shugiin_budget_koizumi_20010914': 'EPUKES5JJ3FCUM4CCFQ46J5TPAB6UB2X',
    'jp_ldp_diet_sangiin_plenary_koizumi_20030930': 'DCWNXGSO2NLHS6SNYD5Q6CAJ2BSEENXI',
    'jp_ldp_news_2006_election_notice_20060908': 'K2RMNR26I6CZIWLOIPWUDD7XXOOGMH5I',
    'jp_ldp_sousai06_schedule_20060918': 'YOC5FY6TIQXAITTJPCXLCGGR2W3LGYWT',
    'jp_ldp_news_abe_elected_20060920': '2GXB2AUNTXHNMYW6XKRGVGFCKYQM6PZ2',
    'jp_ldp_news_abe_first_press_conference_20060920': 'LTUXJURFDVT754GJQEHHNKF6THL7WALZ',
    'jp_ldp_news_joint_plenary_20060920': 'E4XR2EMNVPITRDLB4KZDSBSCN7Z3L7Q2',
    'jp_ldp_news_abe_executive_20060925': '465STGJRJOCZZ7THCZTJY4TEFV6IAUOL',
    'jp_ldp_jiyuminshu_2247_notice_20060925': 'G6KRUIBVNO5RHOUXJONPWVU2UGEJSYJG',
    'jp_ldp_news_abe_president_joint_plenary_20060926': 'QEOC4LSPLFCAIEBCWXR2KJWN5TCPOJ7S',
    'jp_ldp_jiyuminshu_2248_notice_20061002': 'OMP7HLFNMWBUE7Q22RLIO5I4OACWSNGO',
    'jp_ldp_news_abe_resignation_election_method_20070912': 'ASJT44CS56RLWGQECBZTFF4RHLT55JD6',
    'jp_ldp_news_2007_election_schedule_20070913': 'YBMVMGBFMEJSDV32OCQGK6ULX4FSLMA7',
    'jp_ldp_news_2007_candidates_filed_20070915': 'K7KFIZYBWKHPHWBKVMYTQ2Q3LW6543GV',
    'jp_ldp_news_fukuda_elected_20070923': 'PD6Y7FSZDMSXRMDUVDWVHFOIY3Z5ADTB',
    'jp_ldp_news_fukuda_first_press_conference_20070923': 'XZ7DHWMIU5V6KJDK3P7VEDGYKCMARASJ',
    'jp_ldp_news_fukuda_party_officers_20070924': 'HGB3A74EEXPMT5MWWTHSXRULBNKRONOG',
    'jp_ldp_jiyuminshu_2293_notice_20070925': 'RNFQZVBTBUZADAPBSPSPOYTMLVUUSSL3',
    'jp_ldp_news_joint_plenary_schedule_20080903': 'GTNNUMCYG2PEXE6NYENBLKJ3KCMJIJRI',
    'jp_ldp_news_2008_election_notice_20080910': 'U3Y2TVXGJXOCGCUJB2TTBO6KNATND6XS',
    'jp_ldp_news_aso_elected_20080922': '7O3WAHL6MUOAJ5FATNZCXKPESFE3NZJE',
    'jp_ldp_news_aso_first_press_conference_20080922': 'QA7DUIKUC6K5ORXOR4JHVVTEYZGGAZFE',
    'jp_ldp_news_aso_party_officers_20080922': '35DYXN47LBVCNSTTFITAG3TG5MPKKP2S',
    'jp_ldp_jiyuminshu_2339_notice_20080924': 'K6YWCYZ2U2TTSGLBMTH3EOF7O47QHK4Z',
    'jp_ldp_news_joint_plenary_schedule_20090908': 'GRXQ2JZ53NGL7KDLRYW7WWZAE3T6ACO5',
    'jp_ldp_news_2009_notice_day_20090918': 'OEM2WLIJOV3YKPG6TX6MEUMRAX4JQPB7',
    'jp_ldp_sousai09_notice_20090918': 'UQEGH2G576WH2AUJLRDLSEZOCHX3NJCN',
    'jp_ldp_sousai09_schedule_20090921': 'UGYZ7J6FPINXAKLD5C7AKAOTME5UJYZE',
    'jp_ldp_news_index_200909_20091001': 'UGW2XWJEHJS6TZKM4YYBUURUEJOJINTY',
    'jp_ldp_sousai09_tanigaki_elected_20090928': '3WXTWZNWXXDVINEGQVVKFZBLS7ZDI7VT',
    'jp_ldp_sousai09_tanigaki_press_conference_20090928': '2FNRMYPKMQ3QRVMMS4IBW5GGMF6M53ZP',
    'jp_ldp_news_tanigaki_yamba_20091002': 'PL4BRO5RA7BN7UDTGP3GMXN2GADIMLSZ',
    'jp_ldp_history_tanigaki_era': 'SR4XDBXTLC5Q542AGU2PYOO6QRQJYY5L',
}
# Raw Internet Archive captures (id_ form), all made before the cutoff: source id -> capture timestamp.
ARCHIVED = {
    'jp_ldp_history_presidents_list': '20260509234631',
    'jp_ldp_history_kaifu_era': '20260208085612',
    'jp_ldp_history_miyazawa_era': '20260208085606',
    'jp_ldp_history_kono_era': '20260208085618',
    'jp_ldp_history_hashimoto_era': '20260208085602',
    'jp_ldp_ayumi_2015': '20260208085555',
    'jp_ldp_sousai_ayumi_20011109': '20011109084346',
    'jp_ldp_sousai98_index': '20010309225342',
    'jp_ldp_sousai98_vote_page1_19980724': '20010701164749',
    'jp_ldp_sousai98_vote_page3_19980724': '20010719140025',
    'jp_ldp_hatsugen_obuchi_new_year_19990101': '20010701172658',
    'jp_ldp_sousai99_index': '20000605140739',
    'jp_ldp_sousai99_diet_member_vote_19990921': '20000118211111',
    'jp_ldp_sousai99_extraordinary_congress_19990922': '20000118222400',
    'jp_ldp_sousai99_obuchi_greeting_19990922': '20000118144432',
    'jp_ldp_sousai99_press_conference_19990922': '20000118161157',
    'jp_ldp_mori_greeting_joint_plenary_20000405': '20001213045900',
    'jp_ldp_mori_special_20000414': '20000510195818',
    'jp_ldp_toutaikai67_programme_20010313': '20010430045728',
    'jp_ldp_toutaikai67_mori_address_20010313': '20020908064926',
    'jp_ldp_sousai01_top_20010428': '20010428132737',
    'jp_ldp_sousai01_rules_2001': '20010416234950',
    'jp_ldp_sousai01_joint_plenary_20010424': '20010611135737',
    'jp_ldp_sousai01_press_conference_20010424': '20010428123941',
    'jp_ldp_koizumi_reappointment_greeting_20010810': '20020221063322',
    'jp_ldp_sousai03_notice_20030908': '20031216225028',
    'jp_ldp_sousai03_result_20030920': '20030922030149',
    'jp_ldp_sousai03_press_conference_20030920': '20030922025841',
    'jp_ldp_news_2006_election_notice_20060908': '20090105170917',
    'jp_ldp_sousai06_schedule_20060918': '20070228145727',
    'jp_ldp_news_abe_elected_20060920': '20090105170744',
    'jp_ldp_news_abe_first_press_conference_20060920': '20090105170942',
    'jp_ldp_news_joint_plenary_20060920': '20090105170748',
    'jp_ldp_news_abe_executive_20060925': '20090105170753',
    'jp_ldp_jiyuminshu_2247_notice_20060925': '20090105170644',
    'jp_ldp_news_abe_president_joint_plenary_20060926': '20090105170804',
    'jp_ldp_jiyuminshu_2248_notice_20061002': '20070816034738',
    'jp_ldp_news_abe_resignation_election_method_20070912': '20090704021916',
    'jp_ldp_news_2007_election_schedule_20070913': '20090704021938',
    'jp_ldp_news_2007_candidates_filed_20070915': '20090704013620',
    'jp_ldp_news_fukuda_elected_20070923': '20080207140717',
    'jp_ldp_news_fukuda_first_press_conference_20070923': '20090704013625',
    'jp_ldp_news_fukuda_party_officers_20070924': '20090425071429',
    'jp_ldp_jiyuminshu_2293_notice_20070925': '20090910070331',
    'jp_ldp_news_joint_plenary_schedule_20080903': '20080909030240',
    'jp_ldp_news_2008_election_notice_20080910': '20080917044143',
    'jp_ldp_news_aso_elected_20080922': '20080924101756',
    'jp_ldp_news_aso_first_press_conference_20080922': '20080924100844',
    'jp_ldp_news_aso_party_officers_20080922': '20080924100850',
    'jp_ldp_jiyuminshu_2339_notice_20080924': '20080927044817',
    'jp_ldp_news_joint_plenary_schedule_20090908': '20090912062048',
    'jp_ldp_news_2009_notice_day_20090918': '20091001235255',
    'jp_ldp_sousai09_notice_20090918': '20090924114240',
    'jp_ldp_sousai09_schedule_20090921': '20090923172511',
    'jp_ldp_news_index_200909_20091001': '20091001235323',
    'jp_ldp_sousai09_tanigaki_elected_20090928': '20091003234019',
    'jp_ldp_sousai09_tanigaki_press_conference_20090928': '20091003234038',
    'jp_ldp_news_tanigaki_yamba_20091002': '20091011114119',
    'jp_ldp_history_tanigaki_era': '20260208085614',
}
# Official Diet minutes API responses (whole meeting records or single speeches).
DIET = [
    'jp_ldp_diet_hc_kessan_19891213',
    'jp_ldp_diet_hr_yosan_19900322',
    'jp_ldp_diet_hc_yosan_19900514',
    'jp_ldp_diet_hr_honkaigi_19911111',
    'jp_ldp_diet_hc_honkaigi_19911112',
    'jp_ldp_diet_hc_honkaigi_19911113',
    'jp_ldp_diet_hr_yosan_19920220',
    'jp_ldp_diet_hc_yosan_19920331',
    'jp_ldp_diet_hr_koshokusen_19921130',
    'jp_ldp_diet_hr_yosan_19941011',
    'jp_ldp_diet_hr_yosan_19941013',
    'jp_ldp_diet_hr_wto_19941125',
    'jp_ldp_diet_hr_yosan_19950131',
    'jp_ldp_diet_hr_honkaigi_19951002',
    'jp_ldp_diet_hc_honkaigi_19951003',
    'jp_ldp_diet_hr_yosan_19951026',
    'jp_ldp_diet_sangiin_budget_obuchi_19980821',
    'jp_ldp_diet_joint_budget_obuchi_19991110',
    'jp_ldp_diet_shugiin_plenary_mori_20000410',
    'jp_ldp_diet_sangiin_budget_koizumi_20010522',
    'jp_ldp_diet_shugiin_budget_koizumi_20010914',
    'jp_ldp_diet_sangiin_plenary_koizumi_20030930',
]
# Byte-identical copies recorded beside a capture (not the recorded identity).
ALTERNATES = {
    'jp_ldp_ayumi_2015': ('https://storage2.jimin.jp/pdf/aboutus/history/jimin_history.pdf', 3313141,
        '2924c7dd5ba66d939531b961892d4493ae4e08a60e89de8f521b1c7b72c0f482'),
    'jp_ldp_mori_greeting_joint_plenary_20000405': ('https://web.archive.org/web/20010311194932id_/http://www.jimin.or.jp:80/jimin/weekly_bn/120413/morispe/aisatsu.html', 4270,
        'c52bf4725103ea43689d9c3f380c7f66c146fe8a98af548f745bd0a0ed70aa5f'),
}
# PDF pages viewed (rendered, or read from the text layer) for each PDF source.
PDF_PAGES = {
    'jp_ldp_ayumi_2015': [74, 76, 78, 84, 90, 96, 99, 100, 104, 107, 110, 113, 114, 178, 180, 181, 182, 183, 184, 185, 187, 188, 189, 190, 191, 193, 194],
}
SOURCE_TYPES = {
    'jp_ldp_history_presidents_list': 'primary_party_record_archived_retrospective',
    'jp_ldp_history_kaifu_era': 'primary_party_record_archived_retrospective',
    'jp_ldp_history_miyazawa_era': 'primary_party_record_archived_retrospective',
    'jp_ldp_history_kono_era': 'primary_party_record_archived_retrospective',
    'jp_ldp_history_hashimoto_era': 'primary_party_record_archived_retrospective',
    'jp_ldp_ayumi_2015': 'primary_party_history_pdf_archived_retrospective',
    'jp_ldp_diet_hc_kessan_19891213': 'primary_diet_minutes_api_json',
    'jp_ldp_diet_hr_yosan_19900322': 'primary_diet_minutes_api_json',
    'jp_ldp_diet_hc_yosan_19900514': 'primary_diet_minutes_api_json',
    'jp_ldp_diet_hr_honkaigi_19911111': 'primary_diet_minutes_api_json',
    'jp_ldp_diet_hc_honkaigi_19911112': 'primary_diet_minutes_api_json',
    'jp_ldp_diet_hc_honkaigi_19911113': 'primary_diet_minutes_api_json',
    'jp_ldp_diet_hr_yosan_19920220': 'primary_diet_minutes_api_json',
    'jp_ldp_diet_hc_yosan_19920331': 'primary_diet_minutes_api_json',
    'jp_ldp_diet_hr_koshokusen_19921130': 'primary_diet_minutes_api_json',
    'jp_ldp_diet_hr_yosan_19941011': 'primary_diet_minutes_api_json',
    'jp_ldp_diet_hr_yosan_19941013': 'primary_diet_minutes_api_json',
    'jp_ldp_diet_hr_wto_19941125': 'primary_diet_minutes_api_json',
    'jp_ldp_diet_hr_yosan_19950131': 'primary_diet_minutes_api_json',
    'jp_ldp_diet_hr_honkaigi_19951002': 'primary_diet_minutes_api_json',
    'jp_ldp_diet_hc_honkaigi_19951003': 'primary_diet_minutes_api_json',
    'jp_ldp_diet_hr_yosan_19951026': 'primary_diet_minutes_api_json',
    'jp_ldp_sousai_ayumi_20011109': 'primary_party_record_archived_retrospective',
    'jp_ldp_sousai98_index': 'primary_party_record_archived',
    'jp_ldp_sousai98_vote_page1_19980724': 'primary_party_record_archived',
    'jp_ldp_sousai98_vote_page3_19980724': 'primary_party_record_archived',
    'jp_ldp_hatsugen_obuchi_new_year_19990101': 'primary_party_record_archived',
    'jp_ldp_sousai99_index': 'primary_party_record_archived',
    'jp_ldp_sousai99_diet_member_vote_19990921': 'primary_party_record_archived',
    'jp_ldp_sousai99_extraordinary_congress_19990922': 'primary_party_record_archived',
    'jp_ldp_sousai99_obuchi_greeting_19990922': 'primary_party_record_archived',
    'jp_ldp_sousai99_press_conference_19990922': 'primary_party_record_archived',
    'jp_ldp_mori_greeting_joint_plenary_20000405': 'primary_party_record_archived',
    'jp_ldp_mori_special_20000414': 'primary_party_record_archived',
    'jp_ldp_toutaikai67_programme_20010313': 'primary_party_record_archived',
    'jp_ldp_toutaikai67_mori_address_20010313': 'primary_party_record_archived',
    'jp_ldp_sousai01_top_20010428': 'primary_party_record_archived',
    'jp_ldp_sousai01_rules_2001': 'primary_party_rules_archived',
    'jp_ldp_sousai01_joint_plenary_20010424': 'primary_party_record_archived',
    'jp_ldp_sousai01_press_conference_20010424': 'primary_party_record_archived',
    'jp_ldp_koizumi_reappointment_greeting_20010810': 'primary_party_record_archived',
    'jp_ldp_sousai03_notice_20030908': 'primary_party_record_archived',
    'jp_ldp_sousai03_result_20030920': 'primary_party_record_archived',
    'jp_ldp_sousai03_press_conference_20030920': 'primary_party_record_archived',
    'jp_ldp_diet_sangiin_budget_obuchi_19980821': 'primary_diet_minutes_api_json',
    'jp_ldp_diet_joint_budget_obuchi_19991110': 'primary_diet_minutes_api_json',
    'jp_ldp_diet_shugiin_plenary_mori_20000410': 'primary_diet_minutes_api_json',
    'jp_ldp_diet_sangiin_budget_koizumi_20010522': 'primary_diet_minutes_api_json',
    'jp_ldp_diet_shugiin_budget_koizumi_20010914': 'primary_diet_minutes_api_json',
    'jp_ldp_diet_sangiin_plenary_koizumi_20030930': 'primary_diet_minutes_api_json',
    'jp_ldp_news_2006_election_notice_20060908': 'primary_party_record_archived',
    'jp_ldp_sousai06_schedule_20060918': 'primary_party_record_archived',
    'jp_ldp_news_abe_elected_20060920': 'primary_party_record_archived',
    'jp_ldp_news_abe_first_press_conference_20060920': 'primary_party_record_archived',
    'jp_ldp_news_joint_plenary_20060920': 'primary_party_record_archived',
    'jp_ldp_news_abe_executive_20060925': 'primary_party_record_archived',
    'jp_ldp_jiyuminshu_2247_notice_20060925': 'primary_party_record_archived',
    'jp_ldp_news_abe_president_joint_plenary_20060926': 'primary_party_record_archived',
    'jp_ldp_jiyuminshu_2248_notice_20061002': 'primary_party_record_archived',
    'jp_ldp_news_abe_resignation_election_method_20070912': 'primary_party_record_archived',
    'jp_ldp_news_2007_election_schedule_20070913': 'primary_party_record_archived',
    'jp_ldp_news_2007_candidates_filed_20070915': 'primary_party_record_archived',
    'jp_ldp_news_fukuda_elected_20070923': 'primary_party_record_archived',
    'jp_ldp_news_fukuda_first_press_conference_20070923': 'primary_party_record_archived',
    'jp_ldp_news_fukuda_party_officers_20070924': 'primary_party_record_archived',
    'jp_ldp_jiyuminshu_2293_notice_20070925': 'primary_party_record_archived',
    'jp_ldp_news_joint_plenary_schedule_20080903': 'primary_party_record_archived',
    'jp_ldp_news_2008_election_notice_20080910': 'primary_party_record_archived',
    'jp_ldp_news_aso_elected_20080922': 'primary_party_record_archived',
    'jp_ldp_news_aso_first_press_conference_20080922': 'primary_party_record_archived',
    'jp_ldp_news_aso_party_officers_20080922': 'primary_party_record_archived',
    'jp_ldp_jiyuminshu_2339_notice_20080924': 'primary_party_record_archived',
    'jp_ldp_news_joint_plenary_schedule_20090908': 'primary_party_record_archived',
    'jp_ldp_news_2009_notice_day_20090918': 'primary_party_record_archived',
    'jp_ldp_sousai09_notice_20090918': 'primary_party_record_archived',
    'jp_ldp_sousai09_schedule_20090921': 'primary_party_record_archived',
    'jp_ldp_news_index_200909_20091001': 'primary_party_record_archived',
    'jp_ldp_sousai09_tanigaki_elected_20090928': 'primary_party_record_archived',
    'jp_ldp_sousai09_tanigaki_press_conference_20090928': 'primary_party_record_archived',
    'jp_ldp_news_tanigaki_yamba_20091002': 'primary_party_record_archived',
    'jp_ldp_history_tanigaki_era': 'primary_party_record_archived_retrospective',
}
# Every new claim's (attested_on, event_kind, review observation), exactly: distinct dated events are never re-dated,
# relabelled or moved to another observation. Retrospective records and undated references carry no structured date.
EVENTS = {
    'jp_ldp_list_kaifu_span_19890808_19911030': (None, 'retrospective_term_span', 'LDP-PRES-01'),
    'jp_ldp_list_kaifu_joint_plenary_election_19890808': (None, 'retrospective_election_record', 'LDP-PRES-01'),
    'jp_ldp_list_kaifu_result_19890808': (None, 'retrospective_result_record', 'LDP-PRES-01'),
    'jp_ldp_list_kaifu_sole_candidate_notice_19891006': (None, 'retrospective_election_notice_record', 'LDP-PRES-01'),
    'jp_ldp_list_kaifu_convention_approval_19891031': (None, 'retrospective_convention_record', 'LDP-PRES-01'),
    'jp_ldp_list_miyazawa_span_19911031_19930730': (None, 'retrospective_term_span', 'LDP-PRES-02'),
    'jp_ldp_list_miyazawa_public_election_19911027': (None, 'retrospective_election_record', 'LDP-PRES-02'),
    'jp_ldp_list_miyazawa_result_19911027': (None, 'retrospective_result_record', 'LDP-PRES-02'),
    'jp_ldp_list_miyazawa_convention_approval_19911029': (None, 'retrospective_convention_record', 'LDP-PRES-02'),
    'jp_ldp_list_kono_span_19930730_19950930': (None, 'retrospective_term_span', 'LDP-PRES-03'),
    'jp_ldp_list_kono_joint_plenary_election_19930730': (None, 'retrospective_election_record', 'LDP-PRES-03'),
    'jp_ldp_list_kono_result_19930730': (None, 'retrospective_result_record', 'LDP-PRES-03'),
    'jp_ldp_list_kono_sole_candidate_notice_19930917': (None, 'retrospective_election_notice_record', 'LDP-PRES-03'),
    'jp_ldp_list_kono_convention_decision_19930930': (None, 'retrospective_convention_record', 'LDP-PRES-03'),
    'jp_ldp_list_hashimoto_span_19951001_19980724': (None, 'retrospective_term_span', 'LDP-PRES-04'),
    'jp_ldp_list_hashimoto_public_election_19950922': (None, 'retrospective_election_record', 'LDP-PRES-04'),
    'jp_ldp_list_hashimoto_result_19950922': (None, 'retrospective_result_record', 'LDP-PRES-04'),
    'jp_ldp_list_hashimoto_convention_report_19950925': (None, 'retrospective_convention_record', 'LDP-PRES-04'),
    'jp_ldp_list_hashimoto_sole_candidate_notice_19970908': (None, 'retrospective_election_notice_record', 'LDP-PRES-04'),
    'jp_ldp_list_hashimoto_joint_plenary_report_19970911': (None, 'retrospective_convention_record', 'LDP-PRES-04'),
    'jp_ldp_list_obuchi_span_19980724_20000405': (None, 'retrospective_term_span', 'LDP-PRES-05'),
    'jp_ldp_list_obuchi_joint_plenary_decision_19980724': (None, 'retrospective_election_record', 'LDP-PRES-05'),
    'jp_ldp_list_obuchi_result_19980724': (None, 'retrospective_result_record', 'LDP-PRES-05'),
    'jp_ldp_list_obuchi_public_election_19990921': (None, 'retrospective_election_record', 'LDP-PRES-05'),
    'jp_ldp_list_obuchi_result_19990921': (None, 'retrospective_result_record', 'LDP-PRES-05'),
    'jp_ldp_list_obuchi_convention_report_19990922': (None, 'retrospective_convention_record', 'LDP-PRES-05'),
    'jp_ldp_list_mori_span_20000405_20010424': (None, 'retrospective_term_span', 'LDP-PRES-06'),
    'jp_ldp_list_mori_joint_plenary_decision_20000405': (None, 'retrospective_selection_record', 'LDP-PRES-06'),
    'jp_ldp_list_koizumi_span_20010424_20060930': (None, 'retrospective_term_span', 'LDP-PRES-07'),
    'jp_ldp_list_koizumi_joint_plenary_decision_20010424': (None, 'retrospective_election_record', 'LDP-PRES-07'),
    'jp_ldp_list_koizumi_result_20010424': (None, 'retrospective_result_record', 'LDP-PRES-07'),
    'jp_ldp_list_koizumi_reappointment_decided_20010810': (None, 'retrospective_reappointment_record', 'LDP-PRES-07'),
    'jp_ldp_list_koizumi_public_election_20030920': (None, 'retrospective_election_record', 'LDP-PRES-07'),
    'jp_ldp_list_koizumi_result_20030920': (None, 'retrospective_result_record', 'LDP-PRES-07'),
    'jp_ldp_list_abe_span_20061001_20070923': (None, 'retrospective_term_span', 'LDP-PRES-08'),
    'jp_ldp_list_abe_public_election_20060920': (None, 'retrospective_election_record', 'LDP-PRES-08'),
    'jp_ldp_list_abe_result_20060920': (None, 'retrospective_result_record', 'LDP-PRES-08'),
    'jp_ldp_list_fukuda_span_20070923_20080922': (None, 'retrospective_term_span', 'LDP-PRES-08'),
    'jp_ldp_list_fukuda_joint_plenary_decision_20070923': (None, 'retrospective_election_record', 'LDP-PRES-08'),
    'jp_ldp_list_fukuda_result_20070923': (None, 'retrospective_result_record', 'LDP-PRES-08'),
    'jp_ldp_list_aso_span_20080922_20090930': (None, 'retrospective_term_span', 'LDP-PRES-09'),
    'jp_ldp_list_aso_joint_plenary_decision_20080922': (None, 'retrospective_election_record', 'LDP-PRES-09'),
    'jp_ldp_list_aso_result_20080922': (None, 'retrospective_result_record', 'LDP-PRES-09'),
    'jp_ldp_list_tanigaki_span_20091001_20120930': (None, 'retrospective_term_span', 'LDP-PRES-10'),
    'jp_ldp_list_tanigaki_public_election_20090928': (None, 'retrospective_election_record', 'LDP-PRES-10'),
    'jp_ldp_list_tanigaki_result_20090928': (None, 'retrospective_result_record', 'LDP-PRES-10'),
    'jp_ldp_kaifu_era_elected_joint_plenary_19890808': (None, 'retrospective_election_record', 'LDP-PRES-01'),
    'jp_ldp_kaifu_era_assumption_reference_19890808': (None, 'retrospective_assumption_reference', 'LDP-PRES-01'),
    'jp_ldp_kaifu_era_reelection_notice_19891006': (None, 'retrospective_election_notice_record', 'LDP-PRES-01'),
    'jp_ldp_kaifu_era_reappointment_decided_19891031': (None, 'retrospective_convention_record', 'LDP-PRES-01'),
    'jp_ldp_kaifu_era_declines_candidacy_199110': (None, 'retrospective_withdrawal_reference', 'LDP-PRES-01'),
    'jp_ldp_miyazawa_era_elected_19911027': (None, 'retrospective_election_record', 'LDP-PRES-02'),
    'jp_ldp_miyazawa_era_kono_assumed_19930730': (None, 'retrospective_assumption_reference', 'LDP-PRES-03'),
    'jp_ldp_kono_era_miyazawa_resignation_referenced_1993': (None, 'retrospective_resignation_reference', 'LDP-PRES-03'),
    'jp_ldp_kono_era_elected_19930730': (None, 'retrospective_election_record', 'LDP-PRES-03'),
    'jp_ldp_hashimoto_era_elected_19950922': (None, 'retrospective_election_record', 'LDP-PRES-04'),
    'jp_ldp_hashimoto_era_kono_withdrew_199508': (None, 'retrospective_withdrawal_reference', 'LDP-PRES-03'),
    'jp_ldp_ayumi_chronology_kaifu_selected_19890808': (None, 'retrospective_election_record', 'LDP-PRES-01'),
    'jp_ldp_ayumi_chronology_kaifu_confirmed_19891031': (None, 'retrospective_convention_record', 'LDP-PRES-01'),
    'jp_ldp_ayumi_chronology_miyazawa_elected_19911027': (None, 'retrospective_election_record', 'LDP-PRES-02'),
    'jp_ldp_ayumi_chronology_miyazawa_confirmed_19911029': (None, 'retrospective_convention_record', 'LDP-PRES-02'),
    'jp_ldp_ayumi_chronology_kono_elected_19930730': (None, 'retrospective_election_record', 'LDP-PRES-03'),
    'jp_ldp_ayumi_chronology_57th_convention_19930930': (None, 'retrospective_convention_record', 'LDP-PRES-03'),
    'jp_ldp_ayumi_chronology_hashimoto_elected_19950922': (None, 'retrospective_election_record', 'LDP-PRES-04'),
    'jp_ldp_ayumi_chronology_hashimoto_selected_convention_19950925': (None, 'retrospective_convention_record', 'LDP-PRES-04'),
    'jp_ldphist_hashimoto_resignation_announced_1998': (None, 'retrospective_resignation_intent_reference', 'LDP-PRES-04'),
    'jp_ldphist_obuchi_elected_18th_president_19980724': (None, 'retrospective_election_record', 'LDP-PRES-05'),
    'jp_ldphist_obuchi_inherited_term_election_announced_19990909': (None, 'retrospective_election_notice_record', 'LDP-PRES-05'),
    'jp_ldphist_obuchi_reelected_19990921': (None, 'retrospective_election_record', 'LDP-PRES-05'),
    'jp_ldphist_65th_extraordinary_congress_held_19990922': (None, 'retrospective_convention_record', 'LDP-PRES-05'),
    'jp_ldphist_mori_chosen_19th_president_joint_plenary_20000405': (None, 'retrospective_selection_record', 'LDP-PRES-06'),
    'jp_ldphist_mori_early_election_signalled_20010313': (None, 'retrospective_early_election_reference', 'LDP-PRES-06'),
    'jp_ldphist_mori_statement_at_joint_plenary_20010411': (None, 'retrospective_resignation_intent_reference', 'LDP-PRES-06'),
    'jp_ldphist_koizumi_elected_20th_president_20010424': (None, 'retrospective_election_record', 'LDP-PRES-07'),
    'jp_ldphist_koizumi_reelected_unopposed_joint_plenary_20010810': (None, 'retrospective_reappointment_record', 'LDP-PRES-07'),
    'jp_ldphist_koizumi_reelected_20030920': (None, 'retrospective_election_record', 'LDP-PRES-07'),
    'jp_ldp_diet_kaifu_refers_to_assuming_presidency_19891213': ('1989-12-13', 'in_office_reference_pre_period', 'LDP-PRES-01'),
    'jp_ldp_diet_kaifu_recalls_campaign_as_president_stated_19900322': (None, 'in_office_recollection', 'LDP-PRES-01'),
    'jp_ldp_diet_kaifu_speaks_as_president_19900514': ('1990-05-14', 'in_office_attestation', 'LDP-PRES-01'),
    'jp_ldp_diet_mori_miyazawa_chosen_15th_president_stated_19911111': (None, 'election_reference_by_party_officer', 'LDP-PRES-02'),
    'jp_ldp_diet_miyazawa_states_elected_president_stated_19911112': (None, 'election_recalled_by_holder', 'LDP-PRES-02'),
    'jp_ldp_diet_miyazawa_states_elected_president_stated_19911113': (None, 'election_recalled_by_holder', 'LDP-PRES-02'),
    'jp_ldp_diet_miyazawa_recalls_election_oct_nov_1991_stated_19920220': (None, 'election_recalled_by_holder', 'LDP-PRES-02'),
    'jp_ldp_diet_miyazawa_president_standpoint_19920220': ('1992-02-20', 'implicit_in_office_reference', 'LDP-PRES-02'),
    'jp_ldp_diet_miyazawa_president_standpoint_19920331': ('1992-03-31', 'implicit_in_office_reference', 'LDP-PRES-02'),
    'jp_ldp_diet_miyazawa_is_president_19921130': ('1992-11-30', 'in_office_attestation', 'LDP-PRES-02'),
    'jp_ldp_diet_kono_recalls_president_and_officers_resigned_stated_19941011': (None, 'resignation_recalled_by_holder', 'LDP-PRES-03'),
    'jp_ldp_diet_kono_recalls_own_selection_stated_19941011': (None, 'selection_recalled_by_holder', 'LDP-PRES-03'),
    'jp_ldp_diet_kono_recalls_issuing_president_statement_stated_19941013': (None, 'in_office_act_recalled', 'LDP-PRES-03'),
    'jp_ldp_diet_kono_is_president_19941125': ('1994-11-25', 'in_office_attestation', 'LDP-PRES-03'),
    'jp_ldp_diet_kono_speaks_as_president_19950131': ('1995-01-31', 'in_office_continuation_attestation', 'LDP-PRES-03'),
    'jp_ldp_diet_hashimoto_speaks_as_president_19951002': ('1995-10-02', 'in_office_attestation', 'LDP-PRES-04'),
    'jp_ldp_diet_hashimoto_speaks_as_president_19951003': ('1995-10-03', 'in_office_continuation_attestation', 'LDP-PRES-04'),
    'jp_ldp_diet_hashimoto_chosen_at_convention_19950925': ('1995-09-25', 'convention_selection_recalled_by_holder', 'LDP-PRES-04'),
    'jp_ldpayumi01_obuchi_span_18th': (None, 'retrospective_term_span', 'LDP-PRES-05'),
    'jp_ldpayumi01_obuchi_elected_19980724': (None, 'retrospective_election_record', 'LDP-PRES-05'),
    'jp_ldpayumi01_obuchi_elected_19990921': (None, 'retrospective_election_record', 'LDP-PRES-05'),
    'jp_ldpayumi01_obuchi_result_reported_19990922': (None, 'retrospective_convention_record', 'LDP-PRES-05'),
    'jp_ldpayumi01_mori_span_19th': (None, 'retrospective_term_span', 'LDP-PRES-06'),
    'jp_ldpayumi01_mori_selected_joint_plenary_20000405': (None, 'retrospective_selection_record', 'LDP-PRES-06'),
    'jp_ldpayumi01_koizumi_open_span_20th': (None, 'retrospective_term_span', 'LDP-PRES-07'),
    'jp_ldpayumi01_koizumi_elected_20010424': (None, 'retrospective_election_record', 'LDP-PRES-07'),
    'jp_ldp98idx_candidacies_filed_19980721': ('1998-07-21', 'candidacy_filing', 'LDP-PRES-05'),
    'jp_ldp98idx_vote_session_listed_19980724': ('1998-07-24', 'election_session_listing', 'LDP-PRES-05'),
    'jp_ldp98p1_joint_plenary_convened_19980724': ('1998-07-24', 'joint_plenary_meeting_held', 'LDP-PRES-05'),
    'jp_ldp98p1_hashimoto_greets_joint_plenary_19980724': ('1998-07-24', 'in_office_continuation_attestation', 'LDP-PRES-04'),
    'jp_ldp98p3_result_declared_19980724': ('1998-07-24', 'result_declared', 'LDP-PRES-05'),
    'jp_ldp98p3_hashimoto_called_former_president_19980724': ('1998-07-24', 'predecessor_referred_to_as_former', 'LDP-PRES-04'),
    'jp_ldp98p3_obuchi_new_president_after_decision_19980724': ('1998-07-24', 'in_office_attestation', 'LDP-PRES-05'),
    'jp_ldp_hatsugen_obuchi_signs_as_president_19990101': ('1999-01-01', 'in_office_continuation_attestation', 'LDP-PRES-05'),
    'jp_ldp99idx_result_table_19990921': ('1999-09-21', 'presidential_vote_count', 'LDP-PRES-05'),
    'jp_ldp99p8_diet_member_vote_result_announced_19990921': ('1999-09-21', 'result_declared', 'LDP-PRES-05'),
    'jp_ldp99p9_congress_opened_19990922': ('1999-09-22', 'party_convention_held', 'LDP-PRES-05'),
    'jp_ldp99p9_result_reported_to_congress_19990922': ('1999-09-22', 'result_reported_to_convention', 'LDP-PRES-05'),
    'jp_ldp99p9_obuchi_greeting_19990922': ('1999-09-22', 'in_office_attestation', 'LDP-PRES-05'),
    'jp_ldp99aisatu_obuchi_greeting_as_president_19990922': ('1999-09-22', 'in_office_attestation', 'LDP-PRES-05'),
    'jp_ldp99p10_obuchi_press_conference_after_election_19990922': ('1999-09-22', 'in_office_attestation', 'LDP-PRES-05'),
    'jp_ldpmori_stated_assumption_19th_president_20000405': ('2000-04-05', 'assumption_stated', 'LDP-PRES-06'),
    'jp_ldpmori_predecessor_called_former_president_20000405': ('2000-04-05', 'predecessor_referred_to_as_former', 'LDP-PRES-06'),
    'jp_ldpmorisp_joint_plenary_elected_successor_20000405': ('2000-04-05', 'joint_plenary_selection', 'LDP-PRES-06'),
    'jp_ldp_toutaikai67_programme_lists_president_address': (None, 'convention_programme_listing', 'LDP-PRES-06'),
    'jp_ldp_toutaikai67_mori_announces_early_election': (None, 'early_election_announced_by_holder', 'LDP-PRES-06'),
    'jp_ldp01top_election_notice_20010411': ('2001-04-11', 'presidential_election_notice', 'LDP-PRES-07'),
    'jp_ldp01top_new_president_decided_koizumi_20010424': ('2001-04-24', 'presidential_vote_count', 'LDP-PRES-07'),
    'jp_ldp01rules_joint_plenary_election_procedure': (None, 'election_procedure', 'LDP-PRES-07'),
    'jp_ldp01d10_vote_closed_and_counted_20010424': ('2001-04-24', 'presidential_vote_count', 'LDP-PRES-07'),
    'jp_ldp01d10_mori_voting_as_president_20010424': ('2001-04-24', 'in_office_continuation_attestation', 'LDP-PRES-06'),
    'jp_ldp01d10_koizumi_inaugural_greeting_20010424': ('2001-04-24', 'in_office_attestation', 'LDP-PRES-07'),
    'jp_ldp01d11_first_press_conference_after_assuming_20010424': ('2001-04-24', 'in_office_attestation', 'LDP-PRES-07'),
    'jp_ldpsainin_koizumi_reappointment_greeting_20010810': ('2001-08-10', 'in_office_attestation', 'LDP-PRES-07'),
    'jp_ldp03_election_notice_20030908': ('2003-09-08', 'presidential_election_notice', 'LDP-PRES-07'),
    'jp_ldp03_candidacies_filed_20030908': ('2003-09-08', 'candidacy_filing', 'LDP-PRES-07'),
    'jp_ldp03_koizumi_elected_399_20030920': ('2003-09-20', 'presidential_vote_count', 'LDP-PRES-07'),
    'jp_ldp03_winner_reported_approved_joint_plenary_20030920': ('2003-09-20', 'result_reported_to_convention', 'LDP-PRES-07'),
    'jp_ldp03kaiken_press_conference_as_president_20030920': ('2003-09-20', 'in_office_attestation', 'LDP-PRES-07'),
    'jp_ldp_diet_obuchi_states_won_party_presidency_19980821': (None, 'election_recalled_by_holder', 'LDP-PRES-05'),
    'jp_ldp_diet_obuchi_decides_as_party_president_19991110': ('1999-11-10', 'in_office_continuation_attestation', 'LDP-PRES-05'),
    'jp_ldp_diet_mori_as_party_president_20000410': ('2000-04-10', 'in_office_continuation_attestation', 'LDP-PRES-06'),
    'jp_ldp_diet_koizumi_states_became_president_20010522': (None, 'election_recalled_by_holder', 'LDP-PRES-07'),
    'jp_ldp_diet_koizumi_as_party_president_20010914': ('2001-09-14', 'in_office_continuation_attestation', 'LDP-PRES-07'),
    'jp_ldp_diet_koizumi_states_reelected_20030930': (None, 'election_recalled_by_holder', 'LDP-PRES-07'),
    'jp_ldp_2006_election_notice_20060908': ('2006-09-08', 'presidential_election_notice', 'LDP-PRES-08'),
    'jp_ldp_2006_candidacies_filed_20060908': ('2006-09-08', 'candidacy_filing', 'LDP-PRES-08'),
    'jp_ldp_koizumi_incumbent_term_expiry_20060908': ('2006-09-08', 'in_office_continuation_attestation', 'LDP-PRES-07'),
    'jp_ldp_sousai06_schedule_term_expiry_20060930': ('2006-09-18', 'term_expiry_schedule', 'LDP-PRES-08'),
    'jp_ldp_abe_elected_21st_president_20060920': ('2006-09-20', 'presidential_vote_count', 'LDP-PRES-08'),
    'jp_ldp_abe_new_president_addresses_joint_plenary_20060920': ('2006-09-20', 'in_office_attestation', 'LDP-PRES-08'),
    'jp_ldp_abe_first_press_conference_as_president_20060920': ('2006-09-20', 'in_office_attestation', 'LDP-PRES-08'),
    'jp_ldp_joint_plenary_after_election_new_president_20060920': ('2006-09-20', 'in_office_attestation', 'LDP-PRES-08'),
    'jp_ldp_abe_president_decides_executive_20060925': ('2006-09-25', 'in_office_continuation_attestation', 'LDP-PRES-08'),
    'jp_ldp_jiyuminshu_abe_21st_president_succeeds_koizumi_20060925': ('2006-09-25', 'in_office_continuation_attestation', 'LDP-PRES-08'),
    'jp_ldp_abe_president_addresses_joint_plenary_20060926': ('2006-09-26', 'in_office_continuation_attestation', 'LDP-PRES-08'),
    'jp_ldp_jiyuminshu_under_new_president_abe_20061002': ('2006-10-02', 'in_office_continuation_attestation', 'LDP-PRES-08'),
    'jp_ldp_abe_announces_resignation_20070912': ('2007-09-12', 'resignation_intent_announced', 'LDP-PRES-08'),
    'jp_ldp_decides_joint_plenary_election_method_20070912': ('2007-09-12', 'election_method_decided', 'LDP-PRES-08'),
    'jp_ldp_2007_election_schedule_decided_20070913': ('2007-09-13', 'election_schedule_decided', 'LDP-PRES-08'),
    'jp_ldp_2007_fukuda_aso_file_candidacies_20070915': ('2007-09-15', 'candidacy_filing', 'LDP-PRES-08'),
    'jp_ldp_fukuda_elected_22nd_president_joint_plenary_20070923': ('2007-09-23', 'presidential_vote_count', 'LDP-PRES-08'),
    'jp_ldp_fukuda_new_president_address_20070923': ('2007-09-23', 'in_office_attestation', 'LDP-PRES-08'),
    'jp_ldp_fukuda_first_press_conference_as_president_20070923': ('2007-09-23', 'in_office_attestation', 'LDP-PRES-08'),
    'jp_ldp_fukuda_president_nominates_officers_20070924': ('2007-09-24', 'in_office_continuation_attestation', 'LDP-PRES-08'),
    'jp_ldp_jiyuminshu_fukuda_22nd_president_20070923': ('2007-09-23', 'election_reference', 'LDP-PRES-08'),
    'jp_ldp_fukuda_resignation_announcement_20080901': ('2008-09-01', 'resignation_intent_announced', 'LDP-PRES-09'),
    'jp_ldp_2008_election_schedule_reported_joint_plenary_20080903': ('2008-09-03', 'election_schedule_reported', 'LDP-PRES-09'),
    'jp_ldp_fukuda_last_request_as_president_20080903': ('2008-09-03', 'in_office_continuation_attestation', 'LDP-PRES-09'),
    'jp_ldp_2008_election_notice_20080910': ('2008-09-10', 'presidential_election_notice', 'LDP-PRES-09'),
    'jp_ldp_2008_candidacies_filed_20080910': ('2008-09-10', 'candidacy_filing', 'LDP-PRES-09'),
    'jp_ldp_aso_elected_23rd_president_joint_plenary_20080922': ('2008-09-22', 'presidential_vote_count', 'LDP-PRES-09'),
    'jp_ldp_aso_election_declared_by_committee_chair_20080922': ('2008-09-22', 'result_declared', 'LDP-PRES-09'),
    'jp_ldp_aso_new_president_address_20080922': ('2008-09-22', 'in_office_attestation', 'LDP-PRES-09'),
    'jp_ldp_aso_first_press_conference_as_president_20080922': ('2008-09-22', 'in_office_attestation', 'LDP-PRES-09'),
    'jp_ldp_aso_appoints_officers_after_taking_office_20080922': ('2008-09-22', 'in_office_attestation', 'LDP-PRES-09'),
    'jp_ldp_jiyuminshu_aso_23rd_president_20080924': ('2008-09-24', 'in_office_continuation_attestation', 'LDP-PRES-09'),
    'jp_ldp_2009_election_schedule_approved_joint_plenary_20090908': ('2009-09-08', 'election_schedule_approved', 'LDP-PRES-10'),
    'jp_ldp_aso_states_will_resign_presidency_with_cabinet_20090908': ('2009-09-08', 'prospective_resignation_statement', 'LDP-PRES-10'),
    'jp_ldp_aso_president_addresses_joint_plenary_20090908': ('2009-09-08', 'in_office_continuation_attestation', 'LDP-PRES-10'),
    'jp_ldp_2009_notice_day_proposals_to_candidates_20090918': ('2009-09-18', 'notice_day_reference', 'LDP-PRES-10'),
    'jp_ldp_sousai09_election_notice_20090918': ('2009-09-18', 'presidential_election_notice', 'LDP-PRES-10'),
    'jp_ldp_sousai09_candidacies_filed_20090918': ('2009-09-18', 'candidacy_filing', 'LDP-PRES-10'),
    'jp_ldp_sousai09_schedule_term_expiry_20090930': ('2009-09-21', 'term_expiry_schedule', 'LDP-PRES-10'),
    'jp_ldp_index_tanigaki_elected_24th_president_20090928': ('2009-09-28', 'news_index_headline', 'LDP-PRES-10'),
    'jp_ldp_index_tanigaki_president_first_press_conference_20090928': ('2009-09-28', 'news_index_headline', 'LDP-PRES-10'),
    'jp_ldp_index_2009_candidates_filed_20090918': ('2009-09-18', 'news_index_headline', 'LDP-PRES-10'),
    'jp_ldp_sousai09_tanigaki_vote_count_20090928': ('2009-09-28', 'presidential_vote_count', 'LDP-PRES-10'),
    'jp_ldp_sousai09_result_declared_by_chair_20090928': ('2009-09-28', 'result_declared', 'LDP-PRES-10'),
    'jp_ldp_sousai09_tanigaki_address_joint_plenary_20090928': ('2009-09-28', 'in_office_attestation', 'LDP-PRES-10'),
    'jp_ldp_sousai09_aso_called_former_president_20090928': ('2009-09-28', 'predecessor_referred_to_as_former', 'LDP-PRES-10'),
    'jp_ldp_sousai09_tanigaki_first_press_conference_20090928': ('2009-09-28', 'in_office_attestation', 'LDP-PRES-10'),
    'jp_ldp_tanigaki_president_visits_yamba_20091002': ('2009-10-02', 'in_office_continuation_attestation', 'LDP-PRES-10'),
    'jp_ldp_history_tanigaki_era_election_narrative': (None, 'retrospective_election_narrative', 'LDP-PRES-10'),
    'jp_ldp_tanigaki_era_assumption_reference': (None, 'retrospective_assumption_reference', 'LDP-PRES-10'),
    'jp_ldp_tanigaki_era_aso_stepped_down_reference': (None, 'retrospective_resignation_reference', 'LDP-PRES-10'),
}
# The holder name each extract row carries; None where the source names no holder of this office.
ROW_HOLDERS = {
    'jp_ldp_list_kaifu_span_19890808_19911030': '海部俊樹',
    'jp_ldp_list_kaifu_joint_plenary_election_19890808': '海部俊樹',
    'jp_ldp_list_kaifu_result_19890808': '海部俊樹',
    'jp_ldp_list_kaifu_sole_candidate_notice_19891006': None,
    'jp_ldp_list_kaifu_convention_approval_19891031': '海部俊樹',
    'jp_ldp_list_miyazawa_span_19911031_19930730': '宮澤喜一',
    'jp_ldp_list_miyazawa_public_election_19911027': '宮澤喜一',
    'jp_ldp_list_miyazawa_result_19911027': '宮澤喜一',
    'jp_ldp_list_miyazawa_convention_approval_19911029': '宮澤喜一',
    'jp_ldp_list_kono_span_19930730_19950930': '河野洋平',
    'jp_ldp_list_kono_joint_plenary_election_19930730': '河野洋平',
    'jp_ldp_list_kono_result_19930730': '河野洋平',
    'jp_ldp_list_kono_sole_candidate_notice_19930917': None,
    'jp_ldp_list_kono_convention_decision_19930930': '河野洋平',
    'jp_ldp_list_hashimoto_span_19951001_19980724': '橋本龍太郎',
    'jp_ldp_list_hashimoto_public_election_19950922': '橋本龍太郎',
    'jp_ldp_list_hashimoto_result_19950922': '橋本龍太郎',
    'jp_ldp_list_hashimoto_convention_report_19950925': '橋本龍太郎',
    'jp_ldp_list_hashimoto_sole_candidate_notice_19970908': None,
    'jp_ldp_list_hashimoto_joint_plenary_report_19970911': '橋本龍太郎',
    'jp_ldp_list_obuchi_span_19980724_20000405': '小渕恵三',
    'jp_ldp_list_obuchi_joint_plenary_decision_19980724': '小渕恵三',
    'jp_ldp_list_obuchi_result_19980724': '小渕恵三',
    'jp_ldp_list_obuchi_public_election_19990921': '小渕恵三',
    'jp_ldp_list_obuchi_result_19990921': '小渕恵三',
    'jp_ldp_list_obuchi_convention_report_19990922': '小渕恵三',
    'jp_ldp_list_mori_span_20000405_20010424': '森喜朗',
    'jp_ldp_list_mori_joint_plenary_decision_20000405': '森喜朗',
    'jp_ldp_list_koizumi_span_20010424_20060930': '小泉純一郎',
    'jp_ldp_list_koizumi_joint_plenary_decision_20010424': '小泉純一郎',
    'jp_ldp_list_koizumi_result_20010424': '小泉純一郎',
    'jp_ldp_list_koizumi_reappointment_decided_20010810': '小泉純一郎',
    'jp_ldp_list_koizumi_public_election_20030920': '小泉純一郎',
    'jp_ldp_list_koizumi_result_20030920': '小泉純一郎',
    'jp_ldp_list_abe_span_20061001_20070923': '安倍晋三',
    'jp_ldp_list_abe_public_election_20060920': '安倍晋三',
    'jp_ldp_list_abe_result_20060920': '安倍晋三',
    'jp_ldp_list_fukuda_span_20070923_20080922': '福田康夫',
    'jp_ldp_list_fukuda_joint_plenary_decision_20070923': '福田康夫',
    'jp_ldp_list_fukuda_result_20070923': '福田康夫',
    'jp_ldp_list_aso_span_20080922_20090930': '麻生太郎',
    'jp_ldp_list_aso_joint_plenary_decision_20080922': '麻生太郎',
    'jp_ldp_list_aso_result_20080922': '麻生太郎',
    'jp_ldp_list_tanigaki_span_20091001_20120930': '谷垣禎一',
    'jp_ldp_list_tanigaki_public_election_20090928': '谷垣禎一',
    'jp_ldp_list_tanigaki_result_20090928': '谷垣禎一',
    'jp_ldp_kaifu_era_elected_joint_plenary_19890808': '海部俊樹',
    'jp_ldp_kaifu_era_assumption_reference_19890808': '海部俊樹',
    'jp_ldp_kaifu_era_reelection_notice_19891006': '海部俊樹',
    'jp_ldp_kaifu_era_reappointment_decided_19891031': '海部俊樹',
    'jp_ldp_kaifu_era_declines_candidacy_199110': '海部俊樹',
    'jp_ldp_miyazawa_era_elected_19911027': '宮澤喜一',
    'jp_ldp_miyazawa_era_kono_assumed_19930730': '河野洋平',
    'jp_ldp_kono_era_miyazawa_resignation_referenced_1993': '宮澤喜一',
    'jp_ldp_kono_era_elected_19930730': '河野洋平',
    'jp_ldp_hashimoto_era_elected_19950922': '橋本龍太郎',
    'jp_ldp_hashimoto_era_kono_withdrew_199508': '河野洋平',
    'jp_ldp_ayumi_chronology_kaifu_selected_19890808': '海部俊樹',
    'jp_ldp_ayumi_chronology_kaifu_confirmed_19891031': '海部俊樹',
    'jp_ldp_ayumi_chronology_miyazawa_elected_19911027': '宮澤喜一',
    'jp_ldp_ayumi_chronology_miyazawa_confirmed_19911029': '宮澤喜一',
    'jp_ldp_ayumi_chronology_kono_elected_19930730': '河野洋平',
    'jp_ldp_ayumi_chronology_57th_convention_19930930': None,
    'jp_ldp_ayumi_chronology_hashimoto_elected_19950922': '橋本龍太郎',
    'jp_ldp_ayumi_chronology_hashimoto_selected_convention_19950925': '橋本龍太郎',
    'jp_ldphist_hashimoto_resignation_announced_1998': '橋本龍太郎',
    'jp_ldphist_obuchi_elected_18th_president_19980724': '小渕恵三',
    'jp_ldphist_obuchi_inherited_term_election_announced_19990909': '小渕恵三',
    'jp_ldphist_obuchi_reelected_19990921': '小渕恵三',
    'jp_ldphist_65th_extraordinary_congress_held_19990922': None,
    'jp_ldphist_mori_chosen_19th_president_joint_plenary_20000405': '森喜朗',
    'jp_ldphist_mori_early_election_signalled_20010313': '森喜朗',
    'jp_ldphist_mori_statement_at_joint_plenary_20010411': '森喜朗',
    'jp_ldphist_koizumi_elected_20th_president_20010424': '小泉純一郎',
    'jp_ldphist_koizumi_reelected_unopposed_joint_plenary_20010810': '小泉純一郎',
    'jp_ldphist_koizumi_reelected_20030920': '小泉純一郎',
    'jp_ldp_diet_kaifu_refers_to_assuming_presidency_19891213': '海部俊樹',
    'jp_ldp_diet_kaifu_recalls_campaign_as_president_stated_19900322': '海部俊樹',
    'jp_ldp_diet_kaifu_speaks_as_president_19900514': '海部俊樹',
    'jp_ldp_diet_mori_miyazawa_chosen_15th_president_stated_19911111': '宮澤喜一',
    'jp_ldp_diet_miyazawa_states_elected_president_stated_19911112': '宮澤喜一',
    'jp_ldp_diet_miyazawa_states_elected_president_stated_19911113': '宮澤喜一',
    'jp_ldp_diet_miyazawa_recalls_election_oct_nov_1991_stated_19920220': '宮澤喜一',
    'jp_ldp_diet_miyazawa_president_standpoint_19920220': '宮澤喜一',
    'jp_ldp_diet_miyazawa_president_standpoint_19920331': '宮澤喜一',
    'jp_ldp_diet_miyazawa_is_president_19921130': '宮澤喜一',
    'jp_ldp_diet_kono_recalls_president_and_officers_resigned_stated_19941011': None,
    'jp_ldp_diet_kono_recalls_own_selection_stated_19941011': '河野洋平',
    'jp_ldp_diet_kono_recalls_issuing_president_statement_stated_19941013': '河野洋平',
    'jp_ldp_diet_kono_is_president_19941125': '河野洋平',
    'jp_ldp_diet_kono_speaks_as_president_19950131': '河野洋平',
    'jp_ldp_diet_hashimoto_speaks_as_president_19951002': '橋本龍太郎',
    'jp_ldp_diet_hashimoto_speaks_as_president_19951003': '橋本龍太郎',
    'jp_ldp_diet_hashimoto_chosen_at_convention_19950925': '橋本龍太郎',
    'jp_ldpayumi01_obuchi_span_18th': '小渕恵三',
    'jp_ldpayumi01_obuchi_elected_19980724': '小渕恵三',
    'jp_ldpayumi01_obuchi_elected_19990921': '小渕恵三',
    'jp_ldpayumi01_obuchi_result_reported_19990922': '小渕恵三',
    'jp_ldpayumi01_mori_span_19th': '森喜朗',
    'jp_ldpayumi01_mori_selected_joint_plenary_20000405': '森喜朗',
    'jp_ldpayumi01_koizumi_open_span_20th': '小泉純一郎',
    'jp_ldpayumi01_koizumi_elected_20010424': '小泉純一郎',
    'jp_ldp98idx_candidacies_filed_19980721': None,
    'jp_ldp98idx_vote_session_listed_19980724': None,
    'jp_ldp98p1_joint_plenary_convened_19980724': None,
    'jp_ldp98p1_hashimoto_greets_joint_plenary_19980724': '橋本龍太郎',
    'jp_ldp98p3_result_declared_19980724': '小渕恵三',
    'jp_ldp98p3_hashimoto_called_former_president_19980724': '橋本龍太郎',
    'jp_ldp98p3_obuchi_new_president_after_decision_19980724': '小渕恵三',
    'jp_ldp_hatsugen_obuchi_signs_as_president_19990101': '小渕恵三',
    'jp_ldp99idx_result_table_19990921': '小渕恵三',
    'jp_ldp99p8_diet_member_vote_result_announced_19990921': None,
    'jp_ldp99p9_congress_opened_19990922': None,
    'jp_ldp99p9_result_reported_to_congress_19990922': None,
    'jp_ldp99p9_obuchi_greeting_19990922': '小渕恵三',
    'jp_ldp99aisatu_obuchi_greeting_as_president_19990922': '小渕恵三',
    'jp_ldp99p10_obuchi_press_conference_after_election_19990922': '小渕恵三',
    'jp_ldpmori_stated_assumption_19th_president_20000405': '森喜朗',
    'jp_ldpmori_predecessor_called_former_president_20000405': '小渕恵三',
    'jp_ldpmorisp_joint_plenary_elected_successor_20000405': '森喜朗',
    'jp_ldp_toutaikai67_programme_lists_president_address': '森喜朗',
    'jp_ldp_toutaikai67_mori_announces_early_election': '森喜朗',
    'jp_ldp01top_election_notice_20010411': None,
    'jp_ldp01top_new_president_decided_koizumi_20010424': '小泉純一郎',
    'jp_ldp01rules_joint_plenary_election_procedure': None,
    'jp_ldp01d10_vote_closed_and_counted_20010424': None,
    'jp_ldp01d10_mori_voting_as_president_20010424': '森喜朗',
    'jp_ldp01d10_koizumi_inaugural_greeting_20010424': '小泉純一郎',
    'jp_ldp01d11_first_press_conference_after_assuming_20010424': '小泉純一郎',
    'jp_ldpsainin_koizumi_reappointment_greeting_20010810': '小泉純一郎',
    'jp_ldp03_election_notice_20030908': None,
    'jp_ldp03_candidacies_filed_20030908': None,
    'jp_ldp03_koizumi_elected_399_20030920': '小泉純一郎',
    'jp_ldp03_winner_reported_approved_joint_plenary_20030920': '小泉純一郎',
    'jp_ldp03kaiken_press_conference_as_president_20030920': '小泉純一郎',
    'jp_ldp_diet_obuchi_states_won_party_presidency_19980821': '小渕恵三',
    'jp_ldp_diet_obuchi_decides_as_party_president_19991110': '小渕恵三',
    'jp_ldp_diet_mori_as_party_president_20000410': '森喜朗',
    'jp_ldp_diet_koizumi_states_became_president_20010522': '小泉純一郎',
    'jp_ldp_diet_koizumi_as_party_president_20010914': '小泉純一郎',
    'jp_ldp_diet_koizumi_states_reelected_20030930': '小泉純一郎',
    'jp_ldp_2006_election_notice_20060908': None,
    'jp_ldp_2006_candidacies_filed_20060908': None,
    'jp_ldp_koizumi_incumbent_term_expiry_20060908': '小泉純一郎',
    'jp_ldp_sousai06_schedule_term_expiry_20060930': None,
    'jp_ldp_abe_elected_21st_president_20060920': '安倍晋三',
    'jp_ldp_abe_new_president_addresses_joint_plenary_20060920': '安倍晋三',
    'jp_ldp_abe_first_press_conference_as_president_20060920': '安倍晋三',
    'jp_ldp_joint_plenary_after_election_new_president_20060920': '安倍晋三',
    'jp_ldp_abe_president_decides_executive_20060925': '安倍晋三',
    'jp_ldp_jiyuminshu_abe_21st_president_succeeds_koizumi_20060925': '安倍晋三',
    'jp_ldp_abe_president_addresses_joint_plenary_20060926': '安倍晋三',
    'jp_ldp_jiyuminshu_under_new_president_abe_20061002': '安倍晋三',
    'jp_ldp_abe_announces_resignation_20070912': '安倍晋三',
    'jp_ldp_decides_joint_plenary_election_method_20070912': None,
    'jp_ldp_2007_election_schedule_decided_20070913': None,
    'jp_ldp_2007_fukuda_aso_file_candidacies_20070915': None,
    'jp_ldp_fukuda_elected_22nd_president_joint_plenary_20070923': '福田康夫',
    'jp_ldp_fukuda_new_president_address_20070923': '福田康夫',
    'jp_ldp_fukuda_first_press_conference_as_president_20070923': '福田康夫',
    'jp_ldp_fukuda_president_nominates_officers_20070924': '福田康夫',
    'jp_ldp_jiyuminshu_fukuda_22nd_president_20070923': '福田康夫',
    'jp_ldp_fukuda_resignation_announcement_20080901': '福田康夫',
    'jp_ldp_2008_election_schedule_reported_joint_plenary_20080903': None,
    'jp_ldp_fukuda_last_request_as_president_20080903': '福田康夫',
    'jp_ldp_2008_election_notice_20080910': None,
    'jp_ldp_2008_candidacies_filed_20080910': None,
    'jp_ldp_aso_elected_23rd_president_joint_plenary_20080922': '麻生太郎',
    'jp_ldp_aso_election_declared_by_committee_chair_20080922': '麻生太郎',
    'jp_ldp_aso_new_president_address_20080922': '麻生太郎',
    'jp_ldp_aso_first_press_conference_as_president_20080922': '麻生太郎',
    'jp_ldp_aso_appoints_officers_after_taking_office_20080922': '麻生太郎',
    'jp_ldp_jiyuminshu_aso_23rd_president_20080924': '麻生太郎',
    'jp_ldp_2009_election_schedule_approved_joint_plenary_20090908': '麻生太郎',
    'jp_ldp_aso_states_will_resign_presidency_with_cabinet_20090908': '麻生太郎',
    'jp_ldp_aso_president_addresses_joint_plenary_20090908': '麻生太郎',
    'jp_ldp_2009_notice_day_proposals_to_candidates_20090918': None,
    'jp_ldp_sousai09_election_notice_20090918': None,
    'jp_ldp_sousai09_candidacies_filed_20090918': None,
    'jp_ldp_sousai09_schedule_term_expiry_20090930': None,
    'jp_ldp_index_tanigaki_elected_24th_president_20090928': '谷垣禎一',
    'jp_ldp_index_tanigaki_president_first_press_conference_20090928': '谷垣禎一',
    'jp_ldp_index_2009_candidates_filed_20090918': None,
    'jp_ldp_sousai09_tanigaki_vote_count_20090928': '谷垣禎一',
    'jp_ldp_sousai09_result_declared_by_chair_20090928': '谷垣禎一',
    'jp_ldp_sousai09_tanigaki_address_joint_plenary_20090928': '谷垣禎一',
    'jp_ldp_sousai09_aso_called_former_president_20090928': '麻生太郎',
    'jp_ldp_sousai09_tanigaki_first_press_conference_20090928': '谷垣禎一',
    'jp_ldp_tanigaki_president_visits_yamba_20091002': '谷垣禎一',
    'jp_ldp_history_tanigaki_era_election_narrative': '谷垣禎一',
    'jp_ldp_tanigaki_era_assumption_reference': '谷垣禎一',
    'jp_ldp_tanigaki_era_aso_stepped_down_reference': '麻生太郎',
}

NEW_SOURCES = list(RESPONSES)
NEW_CLAIMS = list(EVENTS)
# The 2024 and 2025 observations, byte-for-byte as before this packet.
EXISTING_HOLDERS = [
    {'name': '石破茂', 'attested_on': '2024-09-27', 'from': None, 'until': None, 'sources': ['jp_ldp_ishiba_elected_2024'],
     'claim_ids': ['jp_ishiba_president_20240927'],
     'note': 'Observation of the stated party office on this date only; not a continuous term, biography or likeness authorization.'},
    {'name': '高市早苗', 'attested_on': '2025-10-04', 'from': None, 'until': None, 'sources': ['jp_ldp_takaichi_elected_2025'],
     'claim_ids': ['jp_takaichi_president_20251004'],
     'note': 'Observation of the stated party office on this date only; not a continuous term, biography or likeness authorization.'},
]
# Exact holder observations of jp_ldp_party_president: (name, attested_on, from, until), in chronological order.
HOLDERS = [
    ('海部俊樹', '1990-05-14', None, None),
    ('宮澤喜一', '1992-11-30', None, None),
    ('河野洋平', '1994-11-25', None, None),
    ('橋本龍太郎', '1995-10-02', None, None),
    ('小渕恵三', '1998-07-24', None, None),
    ('小渕恵三', '1999-09-22', None, None),
    ('森喜朗', None, '2000-04-05', None),
    ('小泉純一郎', '2001-04-24', None, None),
    ('小泉純一郎', '2001-08-10', None, None),
    ('小泉純一郎', '2003-09-20', None, None),
    ('安倍晋三', '2006-09-20', None, None),
    ('福田康夫', '2007-09-23', None, None),
    ('麻生太郎', '2008-09-22', None, None),
    ('谷垣禎一', '2009-09-28', None, None),
    ('石破茂', '2024-09-27', None, None),
    ('高市早苗', '2025-10-04', None, None),
]
HOLDER_CLAIMS = [
    ['jp_ldp_diet_kaifu_speaks_as_president_19900514'],
    ['jp_ldp_diet_miyazawa_is_president_19921130'],
    ['jp_ldp_diet_kono_is_president_19941125'],
    ['jp_ldp_diet_hashimoto_speaks_as_president_19951002'],
    ['jp_ldp98p3_obuchi_new_president_after_decision_19980724'],
    ['jp_ldp99p9_obuchi_greeting_19990922', 'jp_ldp99aisatu_obuchi_greeting_as_president_19990922',
     'jp_ldp99p10_obuchi_press_conference_after_election_19990922'],
    ['jp_ldpmori_stated_assumption_19th_president_20000405'],
    ['jp_ldp01d10_koizumi_inaugural_greeting_20010424', 'jp_ldp01d11_first_press_conference_after_assuming_20010424'],
    ['jp_ldpsainin_koizumi_reappointment_greeting_20010810'],
    ['jp_ldp03kaiken_press_conference_as_president_20030920'],
    ['jp_ldp_abe_new_president_addresses_joint_plenary_20060920', 'jp_ldp_abe_first_press_conference_as_president_20060920',
     'jp_ldp_joint_plenary_after_election_new_president_20060920'],
    ['jp_ldp_fukuda_new_president_address_20070923', 'jp_ldp_fukuda_first_press_conference_as_president_20070923'],
    ['jp_ldp_aso_new_president_address_20080922', 'jp_ldp_aso_first_press_conference_as_president_20080922',
     'jp_ldp_aso_appoints_officers_after_taking_office_20080922'],
    ['jp_ldp_sousai09_tanigaki_address_joint_plenary_20090928', 'jp_ldp_sousai09_tanigaki_first_press_conference_20090928'],
    ['jp_ishiba_president_20240927'],
    ['jp_takaichi_president_20251004'],
]
NEW_HOLDERS = 14
# Only the holder's own first-person statement that he takes office now, on a dated occasion, states a start.
STARTS = [('森喜朗', '2000-04-05')]
REVIEW = [f'LDP-PRES-{n:02d}' for n in range(1, 11)]
HOLDER_REVIEW = ['LDP-PRES-01', 'LDP-PRES-02', 'LDP-PRES-03', 'LDP-PRES-04', 'LDP-PRES-05', 'LDP-PRES-05', 'LDP-PRES-06',
                 'LDP-PRES-07', 'LDP-PRES-07', 'LDP-PRES-07', 'LDP-PRES-08', 'LDP-PRES-08', 'LDP-PRES-09', 'LDP-PRES-10']
SURNAMES = {'海部俊樹': ('海部',), '宮澤喜一': ('宮澤', '宮沢'), '河野洋平': ('河野',), '橋本龍太郎': ('橋本',), '小渕恵三': ('小渕',),
            '森喜朗': ('森',), '小泉純一郎': ('小泉',), '安倍晋三': ('安倍',), '福田康夫': ('福田',), '麻生太郎': ('麻生',),
            '谷垣禎一': ('谷垣',)}
ATTEST_KINDS = {'in_office_attestation'}
FROM_KINDS = {'assumption_stated'}
HOLDER_KINDS = ATTEST_KINDS | FROM_KINDS
ELECTION_KINDS = {'presidential_election_notice', 'candidacy_filing', 'election_session_listing', 'presidential_vote_count',
                  'result_declared', 'result_reported_to_convention', 'joint_plenary_selection', 'joint_plenary_meeting_held',
                  'party_convention_held', 'election_recalled_by_holder', 'election_reference_by_party_officer',
                  'selection_recalled_by_holder', 'convention_selection_recalled_by_holder', 'election_reference',
                  'news_index_headline', 'notice_day_reference', 'election_method_decided', 'election_schedule_decided',
                  'election_schedule_reported', 'election_schedule_approved', 'election_procedure',
                  'convention_programme_listing', 'term_expiry_schedule'}
RESIGNATION_KINDS = {'resignation_intent_announced', 'prospective_resignation_statement', 'resignation_recalled_by_holder',
                     'predecessor_referred_to_as_former', 'early_election_announced_by_holder'}
CONTINUATION_KINDS = {'in_office_continuation_attestation', 'implicit_in_office_reference', 'in_office_reference_pre_period',
                      'in_office_recollection', 'in_office_act_recalled'}
RETROSPECTIVE_KINDS = {'retrospective_term_span', 'retrospective_election_record', 'retrospective_result_record',
                       'retrospective_election_notice_record', 'retrospective_convention_record',
                       'retrospective_assumption_reference', 'retrospective_resignation_reference',
                       'retrospective_withdrawal_reference', 'retrospective_selection_record',
                       'retrospective_reappointment_record', 'retrospective_early_election_reference',
                       'retrospective_resignation_intent_reference', 'retrospective_election_narrative'}
ALL_KINDS = HOLDER_KINDS | ELECTION_KINDS | RESIGNATION_KINDS | CONTINUATION_KINDS | RETROSPECTIVE_KINDS
# Kinds that carry no structured date: retrospective records, undated recollections and references, rules and plans.
UNDATED_KINDS = RETROSPECTIVE_KINDS | {'in_office_recollection', 'in_office_act_recalled', 'election_recalled_by_holder',
                                       'election_reference_by_party_officer', 'selection_recalled_by_holder',
                                       'resignation_recalled_by_holder', 'convention_programme_listing',
                                       'early_election_announced_by_holder', 'election_procedure'}
# Kinds that would state a boundary or an acting holder; none is used for this office.
BOUNDARY_KINDS = {'end_of_office_stated', 'resignation_effective', 'term_start_stated', 'acting_president_designated',
                  'interim_president', 'appointment_statement', 'imperial_appointment_ceremony'}
HOLDER_CLAIM_SET = {cid for ids in HOLDER_CLAIMS[:NEW_HOLDERS] for cid in ids}
NEVER_HOLDER = tuple(cid for cid in EVENTS if cid not in HOLDER_CLAIM_SET)
ELECTIONS = tuple(cid for cid, (_d, kind, _o) in EVENTS.items() if kind in ELECTION_KINDS)
RESIGNATIONS = tuple(cid for cid, (_d, kind, _o) in EVENTS.items() if kind in RESIGNATION_KINDS)
CONTINUATION = tuple(cid for cid, (_d, kind, _o) in EVENTS.items() if kind in CONTINUATION_KINDS)
RETROSPECTIVE = tuple(cid for cid, (_d, kind, _o) in EVENTS.items() if kind in RETROSPECTIVE_KINDS)
UNDATED = tuple(cid for cid, (_d, kind, _o) in EVENTS.items() if kind in UNDATED_KINDS)
COUNTS = {
    'sources_claims': (81, 192),
    'holder_never': (23, 169),
    'categories': (51, 8, 23, 87, 101),
    'archived_diet': (59, 22),
}
# Dates that are never a holder's attested_on, start or end: elections, notices, candidacies, selections, declarations,
# conventions, list starts and ends, term expiries, announcements and later or implicit attestations.
NEVER_HOLDER_DATE = {
    '1989-08-08', '1989-10-06', '1989-10-31', '1989-12-13', '1990-01-01', '1990-03-22', '1991-10-27', '1991-10-29',
    '1991-10-30', '1991-10-31', '1991-11-11', '1991-11-12', '1991-11-13', '1992-02-20', '1992-03-31', '1993-07-30',
    '1993-09-17', '1993-09-30', '1993-12-14', '1994-10-11', '1994-10-13', '1995-01-31', '1995-09-22', '1995-09-25',
    '1995-09-30', '1995-10-01', '1995-10-03', '1997-09-08', '1997-09-11', '1998-07-21', '1998-08-21', '1999-01-01',
    '1999-09-09', '1999-09-21', '1999-11-10', '2000-04-02', '2000-04-04', '2000-04-10', '2001-03-13', '2001-04-11',
    '2001-05-22', '2001-09-14', '2003-09-08', '2003-09-30', '2006-09-08', '2006-09-18', '2006-09-25', '2006-09-26',
    '2006-09-30', '2006-10-01', '2006-10-02', '2007-09-12', '2007-09-13', '2007-09-14', '2007-09-15', '2007-09-24',
    '2007-09-25', '2008-09-01', '2008-09-03', '2008-09-10', '2008-09-24', '2009-09-08', '2009-09-16', '2009-09-18',
    '2009-09-21', '2009-09-30', '2009-10-01', '2009-10-02', '2012-09-30'}
# On each transition day these events stay separate claims.
TRANSITIONS = {
    '1998-07-24': ['joint_plenary_meeting_held', 'result_declared', 'predecessor_referred_to_as_former', 'in_office_attestation',
                   'in_office_continuation_attestation'],
    '1999-09-21': ['presidential_vote_count', 'result_declared'],
    '1999-09-22': ['party_convention_held', 'result_reported_to_convention', 'in_office_attestation'],
    '2000-04-05': ['assumption_stated', 'joint_plenary_selection', 'predecessor_referred_to_as_former'],
    '2001-04-24': ['presidential_vote_count', 'in_office_continuation_attestation', 'in_office_attestation'],
    '2003-09-08': ['presidential_election_notice', 'candidacy_filing'],
    '2003-09-20': ['presidential_vote_count', 'result_reported_to_convention', 'in_office_attestation'],
    '2006-09-08': ['presidential_election_notice', 'candidacy_filing', 'in_office_continuation_attestation'],
    '2006-09-20': ['presidential_vote_count', 'in_office_attestation'],
    '2007-09-12': ['resignation_intent_announced', 'election_method_decided'],
    '2007-09-23': ['presidential_vote_count', 'in_office_attestation', 'election_reference'],
    '2008-09-10': ['presidential_election_notice', 'candidacy_filing'],
    '2008-09-22': ['presidential_vote_count', 'result_declared', 'in_office_attestation'],
    '2009-09-08': ['election_schedule_approved', 'prospective_resignation_statement', 'in_office_continuation_attestation'],
    '2009-09-18': ['presidential_election_notice', 'candidacy_filing', 'notice_day_reference', 'news_index_headline'],
    '2009-09-28': ['presidential_vote_count', 'result_declared', 'in_office_attestation', 'predecessor_referred_to_as_former',
                   'news_index_headline'],
}
# Secondary sources and leads that must never be a source URL here.
LEAD_URL_MARKERS = ('wikipedia', 'britannica', 'kotobank', 'nikkei', 'asahi.com', 'yomiuri', 'mainichi', 'nhk.or.jp',
                    'jiji.com', 'sankei', 'aboutus/history/18.html', 'aboutus/history/19.html', 'aboutus/history/20.html',
                    'aboutus/history/21.html', 'aboutus/history/22.html', 'aboutus/history/23.html', 'pics-42', 'ss_kaiken',
                    '150920b', 'kenren/kekka', 'sousai03/schedule', 'hatsugen/index', '113413968X00119951019',
                    '111815254X00319900306', '111805261X01719900509', '112815261X00319931008', '112804573X00419931019',
                    '113005254X00219940720', '112705254X00619930826', '115724293X00120031009', '114304056X00419980828',
                    'speechNumber=2&', 'e-presidentRule', 'warp.ndl.go.jp', 'sousai09/kekka', 'data_01/kekka',
                    'data_01/pdf/kekka', 'youryou', '200902a', '120413/obu', 'sousai01/index', '130810',
                    '09_10index', '211005a', '190925a', '190926a', '200924b', '200924d', '20260825202443', '20260208090814')
LEAD_REPORT_MARKERS = ('pics-42', 'ss_kaiken', '113413968X00119951019', '112804573X00419931019', 'sousai09/kekka.html',
                       'data_01/kekka.html', 'youryou.html', '200902a', '20260825202443', 'aboutus/history/21.html')
# Response shapes a server can generate per request, cache-busting queries, open searches and live listing pages.
PER_REQUEST_PATTERNS = ('cb=', '_cb', 'cbx=', 'any=', 'token=', 'sessionid', 'X-Amz-', 'Signature=', 'searchResult',
                        'opensearch', 'cdx/search', '?q=', '&q=', 'speaker=', 'from=', 'until=', 'page-range', 'nocache')
LIVE_PDF = 'https://storage2.jimin.jp/pdf/aboutus/history/jimin_history.pdf'
# Ids withdrawn, merged or renamed after the checks; none may remain in the packet or the extracts.
STALE_IDS = ('"jp_diet_', 'jp_diet_hc_gaimu_19951019', 'jp_ldp_diet_hc_gaimu_19951019', 'kono_completed_term_stated_19951019',
             'kono_decided_not_to_stand_stated_19951019', 'jp_ldphist_obuchi_stroke_reported_20000402',
             'jp_ldp_rekidai_sosai_table_20260825', 'jp_ldp_history_successive_presidents_20260208',
             'jp_ldp_history_ayumi_pdf_2015', '"jp_ldptable_', 'jp_ldp_list_kaifu_sole_candidate_19891006_19891031',
             'jp_ldp_list_kono_sole_candidate_19930917_19930930', 'jp_ldp_kaifu_era_reelected_sole_candidate_19891031',
             'jp_ldp98p1_joint_plenary_convened_hashimoto_greeting_19980724',
             'jp_ldp_sousai03_assumption_press_conference_20030920',
             'jp_ldp03kaiken_press_conference_on_assumption_20030920',
             'jp_ldp_history_tanigaki_era_election_narrative_20090928', 'jp_ldp_history_tanigaki_era_20260208',
             'jp_ldp_2009_notice_day_three_candidates_20090918', 'jp_ldp_aso_acceptance_address_23rd_president_20080922',
             'jp_ldp_2006_election_notice_three_candidates_20060908', 'jp_ldp_2008_election_notice_five_candidates_20080910',
             'election_notice_and_nomination', 'stated_assumption_of_office', 'attested_period', 'printed_date',
             'statement_date', 'printed_range', 'holder_incapacity_reported')
REPORT = research.RESEARCH / 'japan-ldp-presidents-1990-2009-18.md'
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-18.md'


def load_rows(packet):
    rows = {}
    for source in packet['sources'][EARLIER_SOURCE_COUNT:EARLIER_SOURCE_COUNT + len(NEW_SOURCES)]:
        extract = json.loads((research.ROOT / source['snapshot']['path']).read_text(encoding='utf-8'))
        rows.update({row['claim_id']: row for row in extract['rows']})
    return rows


def holder_date(holder):
    return holder['attested_on'] or holder['from']


def ldp_rules(packet, rows):
    """Rule-based checks that hold without the pinned holder list; raise AssertionError, KeyError or IndexError."""
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    source_of = {c['id']: s['id'] for s in packet['sources'] for c in s['claims']}
    org, = [o for o in packet['organizations'] if o['id'] == ORG_ID]
    assert (org['name'], org['lifecycle']['status'], org['represented_party_ids']) == ('自由民主党', 'unknown', [])
    assert [r['id'] for r in org['roles']] == [ROLE], 'exactly one LDP role, no new role'
    role = org['roles'][0]
    assert (role['title'], role['kind']) == (T_LDP, 'party_leader')
    # No institution or role is added, and the party role exists only here.
    assert [i['id'] for i in packet['institutions']] == GROUPS + ['jp_prime_minister'], 'no new institution'
    assert [r['id'] for r in packet['institutions'][-1]['roles']] == [PM], 'the prime-ministership keeps one role'
    assert sum(r['id'] == ROLE for e in packet['organizations'] + packet['institutions'] for r in e['roles']) == 1
    holders = role['holder_claims']
    assert all(isinstance(h, dict) for h in holders)
    assert holders[NEW_HOLDERS:] == EXISTING_HOLDERS, 'the 2024 and 2025 observations are unchanged'
    previous = ''
    for holder in holders[:NEW_HOLDERS]:
        name = holder['name']
        assert name in SURNAMES, name
        dated = [d for d in (holder['attested_on'], holder['from']) if d]
        assert len(dated) == 1, (name, 'a holder is dated by exactly one of attested_on and from')
        day = dated[0]
        assert day > previous, (name, 'holders stay in chronological order')
        previous = day
        assert holder['until'] is None, (name, 'no source states the day a presidency ended')
        assert day not in NEVER_HOLDER_DATE, (name, day)
        assert day <= research.CUTOFF, name
        assert not set(holder['claim_ids']) & set(NEVER_HOLDER), name
        expected = []
        for cid in holder['claim_ids']:
            row, claim = rows[cid], claims[cid]
            assert cid in role['claim_ids'] and cid.startswith('jp_ldp'), cid
            assert row['holder_name'] == name and (row['role_id'], row['role_title']) == (ROLE, T_LDP), (name, cid)
            assert row['event_kind'] in (FROM_KINDS if holder['from'] else ATTEST_KINDS), (name, cid)
            assert claim['attested_on'] == day == row['attested_on'], (name, cid, 'a cited claim carries the holder\'s day')
            assert any(s in claim['text'] for s in SURNAMES[name]), (name, cid)
            if source_of[cid] not in expected:
                expected.append(source_of[cid])
        assert holder['sources'] == expected, name
    # Every named attestation or stated start is cited by exactly one holder; an unnamed row never is.
    for cid, row in rows.items():
        kind, name = row['event_kind'], row['holder_name']
        assert kind in ALL_KINDS and kind not in BOUNDARY_KINDS, (cid, kind)
        cited = [h for h in holders if cid in h['claim_ids']]
        if kind in HOLDER_KINDS and name:
            assert len(cited) == 1 and cited[0]['name'] == name, cid
        if kind not in HOLDER_KINDS or not name:
            assert not cited, cid
        if kind in UNDATED_KINDS:
            assert 'attested_on' not in claims[cid] and row['attested_on'] is None, cid
        else:
            assert claims[cid]['attested_on'] == row['attested_on'] <= research.CUTOFF, cid
        assert not {'period', 'attested_period', 'printed_date', 'statement_date'} & set(claims[cid]), cid
    # Party office and the prime-ministership never feed each other.
    pm_role = packet['institutions'][-1]['roles'][0]
    assert len(pm_role['holder_claims']) == PM_HOLDER_COUNT
    pm_claims = set(pm_role['claim_ids']) | {c for h in pm_role['holder_claims'] for c in h['claim_ids']} | set(
        packet['institutions'][-1]['claim_ids'])
    pm_sources = set(pm_role['sources']) | {s for h in pm_role['holder_claims'] for s in h['sources']} | set(
        packet['institutions'][-1]['sources'])
    ldp_claims = set(role['claim_ids']) | {c for h in holders for c in h['claim_ids']}
    ldp_sources = set(role['sources']) | {s for h in holders for s in h['sources']}
    assert not ldp_claims & pm_claims and not ldp_sources & pm_sources
    assert not any(c.startswith('jp_ldp') for c in pm_claims) and not any(s.startswith('jp_ldp') for s in pm_sources)
    assert all(c.startswith('jp_ldp') for c in ldp_claims - set(EARLIER_CLAIMS))
    assert all(s.startswith('jp_ldp') for s in ldp_sources - set(EARLIER_SOURCES))
    assert {h['name'] for h in pm_role['holder_claims']}.isdisjoint({'河野洋平', '谷垣禎一'}), 'never Prime Minister'
    for entry in packet['organizations'] + packet['institutions']:
        if entry is not org:
            assert not set(entry['claim_ids']) & ldp_claims and not set(entry['sources']) & ldp_sources, entry['id']
    assert set(role['claim_ids']) <= set(org['claim_ids']) and set(role['sources']) <= set(org['sources'])


def ldp_invariants(packet, rows):
    """The rules plus the exact pinned holders, rows and events this packet intends."""
    ldp_rules(packet, rows)
    org, = [o for o in packet['organizations'] if o['id'] == ORG_ID]
    role = org['roles'][0]
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    got = [(h['name'], h['attested_on'], h['from'], h['until']) for h in role['holder_claims']]
    assert got == HOLDERS, got
    assert [h['claim_ids'] for h in role['holder_claims']] == HOLDER_CLAIMS
    assert [(h['name'], h['from']) for h in role['holder_claims'] if h['from']] == STARTS
    assert not [h for h in role['holder_claims'] if h['until']]
    assert {cid: row['holder_name'] for cid, row in rows.items()} == ROW_HOLDERS
    for cid, (day, kind, obs) in EVENTS.items():
        assert claims[cid].get('attested_on') == day, cid
        assert (rows[cid]['attested_on'], rows[cid]['event_kind'], rows[cid]['review_observation']) == (day, kind, obs), cid


class JapanLdpPresidentsTests(unittest.TestCase):
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
        self.assertEqual(order[EARLIER_SOURCE_COUNT:], NEW_SOURCES)
        self.assertFalse([sid for sid in order[:EARLIER_SOURCE_COUNT] if sid in RESPONSES])
        self.assertEqual((len(ids['entries']), len(ids['roles'])), (24, 5))
        self.assertEqual((len(self.packet['organizations']), len(self.packet['institutions'])), (16, 8))
        self.assertEqual(self.new_claims, NEW_CLAIMS)
        self.assertEqual(set(EVENTS), set(self.rows))
        # Every new claim is either a holder claim or a claim that never feeds a holder, never both.
        self.assertFalse(HOLDER_CLAIM_SET & set(NEVER_HOLDER))
        self.assertEqual(HOLDER_CLAIM_SET | set(NEVER_HOLDER), set(self.new_claims))
        self.assertEqual((len(HOLDER_CLAIM_SET), len(NEVER_HOLDER)), COUNTS['holder_never'])
        self.assertEqual((len(ELECTIONS), len(RESIGNATIONS), len(CONTINUATION), len(RETROSPECTIVE), len(UNDATED)),
                         COUNTS['categories'])
        # The role and its organization cite every new claim and source after their earlier ones, and nothing else does.
        self.assertEqual(self.role['claim_ids'], EARLIER_CLAIMS + NEW_CLAIMS)
        self.assertEqual(self.role['sources'], EARLIER_SOURCES + NEW_SOURCES)
        self.assertEqual(self.org['claim_ids'], ORG_EARLIER_CLAIMS + NEW_CLAIMS)
        self.assertEqual(self.org['sources'], ORG_EARLIER_SOURCES + NEW_SOURCES)
        for entry in self.packet['organizations'] + self.packet['institutions']:
            if entry is not self.org:
                self.assertFalse(set(self.new_claims) & set(entry['claim_ids']), entry['id'])
                self.assertFalse(set(NEW_SOURCES) & set(entry['sources']), entry['id'])
        # At most ten observations, LDP-PRES-01..10, each reported and each carrying rows.
        observations = re.findall(r'^### (LDP-PRES-\d\d)\b', self.report, re.M)
        self.assertEqual(observations, REVIEW)
        self.assertEqual({row['review_observation'] for row in self.rows.values()}, set(REVIEW))
        self.assertEqual([self.rows[ids_[0]]['review_observation'] for ids_ in HOLDER_CLAIMS[:NEW_HOLDERS]], HOLDER_REVIEW)
        for stale in STALE_IDS:
            self.assertNotIn(stale, self.raw, stale)
            for extract in self.extracts.values():
                self.assertNotIn(stale, json.dumps(extract, ensure_ascii=False), stale)

    def test_holders_are_exactly_as_intended(self):
        ldp_invariants(self.packet, self.rows)
        for index, holder in enumerate(self.role['holder_claims'][:NEW_HOLDERS]):
            self.assertEqual(list(holder), ['name', 'attested_on', 'from', 'until', 'sources', 'claim_ids', 'note',
                                            'uncertainty'])
            if holder['from']:
                self.assertTrue(holder['note'].startswith('From 5 April 2000: '), holder['name'])
                self.assertTrue(holder['uncertainty'].startswith('No end is stated'), holder['name'])
            else:
                self.assertTrue(holder['note'].startswith('Observed on '), holder['name'])
                self.assertTrue(holder['uncertainty'].startswith('No start or end is stated'), holder['name'])
            for cid in holder['claim_ids']:
                self.assertIn('Dates the holder observation' if holder['attested_on'] else 'States the day office was assumed',
                              self.claims[cid]['uncertainty'], cid)
        # Row holder names are normalised holders of this office; the printed forms stay in the claim text.
        self.assertEqual({v for v in ROW_HOLDERS.values()} - {None}, set(SURNAMES))
        for cid, name in ROW_HOLDERS.items():
            self.assertEqual((self.rows[cid]['role_id'], self.rows[cid]['role_title']), (ROLE, T_LDP), cid)
            if name:
                self.assertTrue(any(s in self.claims[cid]['text'] for s in SURNAMES[name]), cid)
        self.assertIn('宮沢', self.claims['jp_ldp_list_miyazawa_span_19911031_19930730']['text'])
        self.assertEqual(ROW_HOLDERS['jp_ldp_list_miyazawa_span_19911031_19930730'], '宮澤喜一')
        self.assertIn('安倍新総裁', self.claims['jp_ldp_jiyuminshu_under_new_president_abe_20061002']['text'])
        self.assertEqual(ROW_HOLDERS['jp_ldp_jiyuminshu_under_new_president_abe_20061002'], '安倍晋三')
        # Two presidents of this period were never Prime Minister, and hold only the party office.
        self.assertEqual([h['name'] for h in self.role['holder_claims'] if h['name'] in ('河野洋平', '谷垣禎一')],
                         ['河野洋平', '谷垣禎一'])

    def test_starts_ends_and_claims_that_never_feed_a_holder(self):
        claims, rows = self.claims, self.rows
        # Elections, notices, candidacies, declarations, reports and resignations are distinct claims on each transition day.
        for day, expected in TRANSITIONS.items():
            kinds = sorted({rows[cid]['event_kind'] for cid, (d, _k, _o) in EVENTS.items() if d == day})
            for kind in expected:
                self.assertIn(kind, kinds, (day, kind))
        for cid in ELECTIONS + RESIGNATIONS + CONTINUATION + RETROSPECTIVE:
            self.assertNotIn(cid, HOLDER_CLAIM_SET, cid)
        # Resignations, announcements, predecessor references and withdrawals are never an end.
        for cid in RESIGNATIONS:
            self.assertRegex(claims[cid]['uncertainty'], r'(?i)never an (end|until)|not an end|no end', cid)
        for cid in CONTINUATION:
            self.assertRegex(claims[cid]['uncertainty'], r'(?i)never a holder|claim only', cid)
        # Retrospective lists, histories and undated recollections carry no structured date and are never a boundary.
        for cid in RETROSPECTIVE:
            self.assertIn('never a holder boundary', claims[cid]['uncertainty'], cid)
        for cid in UNDATED:
            self.assertNotIn('attested_on', claims[cid], cid)
            self.assertIn('no structured date', claims[cid]['uncertainty'], cid)
        # The one stated start is the holder's own statement of assumption on the dated occasion.
        start = [cid for cid, row in rows.items() if row['event_kind'] in FROM_KINDS]
        self.assertEqual(start, ['jp_ldpmori_stated_assumption_19th_president_20000405'])
        self.assertIn('ただ今', claims[start[0]]['text'])
        self.assertIn('就任致すことになりました', claims[start[0]]['text'])
        # 就任 wording in captions, headings and headlines dates an observation but is never a start.
        for cid in ('jp_ldp01d10_koizumi_inaugural_greeting_20010424', 'jp_ldp01d11_first_press_conference_after_assuming_20010424',
                    'jp_ldp03kaiken_press_conference_as_president_20030920', 'jp_ldp_abe_first_press_conference_as_president_20060920',
                    'jp_ldp_aso_appoints_officers_after_taking_office_20080922',
                    'jp_ldp_sousai09_tanigaki_first_press_conference_20090928'):
            self.assertEqual(rows[cid]['event_kind'], 'in_office_attestation', cid)
            self.assertIn('就任', claims[cid]['text'], cid)
        # Term-expiry schedules and the lists' printed spans name no boundary; the conflicts stay unresolved.
        for cid in ('jp_ldp_sousai06_schedule_term_expiry_20060930', 'jp_ldp_sousai09_schedule_term_expiry_20090930'):
            self.assertIsNone(rows[cid]['holder_name'], cid)
            self.assertIn('総裁の任期満了日', claims[cid]['text'], cid)
        for name in ('安倍晋三', '谷垣禎一', '麻生太郎'):
            holder = next(h for h in self.role['holder_claims'] if h['name'] == name)
            self.assertIn('conflict', holder['uncertainty'], name)
        # Acting or interim service is never a holder: no row or holder describes it.
        for row in rows.values():
            self.assertNotRegex(row['event_kind'], r'acting|interim|caretaker', row['claim_id'])
        # Retrospective sources are only the party's list, its era pages, its printed history and its 2001 election list.
        retro_sources = sorted({self.claim_source[cid] for cid in RETROSPECTIVE})
        self.assertEqual(retro_sources, sorted(['jp_ldp_history_presidents_list', 'jp_ldp_history_kaifu_era',
                                                'jp_ldp_history_miyazawa_era', 'jp_ldp_history_kono_era',
                                                'jp_ldp_history_hashimoto_era', 'jp_ldp_ayumi_2015',
                                                'jp_ldp_sousai_ayumi_20011109', 'jp_ldp_history_tanigaki_era']))
        for sid in retro_sources:
            self.assertTrue(self.sources[sid]['source_type'].endswith('_retrospective'), sid)
        scope = self.role['scope_note']
        self.assertTrue(scope.startswith('Two party election observations. The office is separate from prime minister; no '
                                         'national executive powers imported. CLAUDE-C01-18 extends the role'))
        for phrase in ('never feed a holder', 'procedure only, never a date', "outgoing president's last day",
                       'never read the prime-ministership (jp_pm) into this office or the reverse', 'only 森喜朗 has a stated start'):
            self.assertIn(phrase, scope)
        unresolved = self.org['coverage']['unresolved']
        self.assertEqual(len(unresolved), 4)
        self.assertTrue(unresolved[-1].startswith('LDP presidents 1990-2009 (CLAUDE-C01-18)'))
        packet_unresolved = self.packet['coverage']['unresolved']
        self.assertEqual(sum('CLAUDE-C01-18' in u for u in packet_unresolved), 1)
        self.assertTrue(packet_unresolved[-1].startswith('LDP presidents 1990-2009 (CLAUDE-C01-18'))
        self.assertTrue(packet_unresolved[-2].startswith('Prime ministers 2006-2026 (CLAUDE-C01-13'))

    def test_extracts_match_packet_claims_and_record_original_responses(self):
        for sid in NEW_SOURCES:
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual(extract['format'], 'spheres-c01-derived-factual-table/v1')
            self.assertEqual((extract['source_id'], extract['source_url']), (sid, source['url']))
            for key in ('scope_note', 'access_method', 'published_date', 'rights_note', 'accessed_date'):
                self.assertEqual(extract[key], source[key], (sid, key))
            self.assertEqual(source['accessed_date'], '2026-09-25', sid)
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
            self.assertEqual(extract['visual_review']['pdf_pages_one_based'], PDF_PAGES.get(sid, []))
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
                self.assertEqual((row['observation_id'], row['role_id'], row['role_title']), (ORG_ID, ROLE, T_LDP))
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
            self.assertRegex(source['original_url'], r'^https?://(www\.jimin\.(jp|or\.jp)|storage2\.jimin\.jp)/')
            self.assertEqual(extract['archive_capture_utc'].replace('-', '').replace(':', '').replace('T', '').rstrip('Z'), stamp)
            self.assertIn('Raw Internet Archive capture', extract['provenance_note'])
            if not source['url'].endswith('.pdf'):
                self.assertRegex(extract['source_character_encoding'], r'^(Shift_JIS|UTF-8)')
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
        for sid, (url, size, sha) in ALTERNATES.items():
            alternate = self.extracts[sid]['alternate_location']
            self.assertEqual((alternate['url'], alternate['bytes'], alternate['sha256']), (url, size, sha))
        self.assertEqual({sid for sid in NEW_SOURCES if 'alternate_location' in self.extracts[sid]}, set(ALTERNATES))

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
        # The party's list is recorded only as its fixed capture, and the live stored PDF only as an alternate location.
        self.assertNotEqual(self.sources['jp_ldp_ayumi_2015']['url'], LIVE_PDF)
        self.assertEqual(ALTERNATES['jp_ldp_ayumi_2015'][0], LIVE_PDF)
        self.assertEqual(ARCHIVED['jp_ldp_history_presidents_list'], '20260509234631')
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
        kaifu, miyazawa, kono = index[('海部俊樹', '1990-05-14')], index[('宮澤喜一', '1992-11-30')], index[('河野洋平', '1994-11-25')]
        hashimoto, obuchi98, obuchi99 = index[('橋本龍太郎', '1995-10-02')], index[('小渕恵三', '1998-07-24')], index[('小渕恵三', '1999-09-22')]
        mori, koizumi01, koizumi03 = index[('森喜朗', '2000-04-05')], index[('小泉純一郎', '2001-04-24')], index[('小泉純一郎', '2003-09-20')]
        abe, fukuda, aso, tanigaki = index[('安倍晋三', '2006-09-20')], index[('福田康夫', '2007-09-23')], \
            index[('麻生太郎', '2008-09-22')], index[('谷垣禎一', '2009-09-28')]
        ishiba = index[('石破茂', '2024-09-27')]
        validator_cases = [
            (lambda p: source(p, 'jp_ldp_history_presidents_list')['snapshot'].update(sha256='0' * 64), 'checksum mismatch'),
            (lambda p: source(p, 'jp_ldp_diet_hr_wto_19941125')['snapshot'].update(bytes=1), 'checksum mismatch'),
            (lambda p: source(p, 'jp_ldp_ayumi_2015')['snapshot'].update(path=REPORT.as_posix()), 'escapes'),
            (lambda p: holder(p, tanigaki).update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: claim(p, 'jp_ldp_tanigaki_president_visits_yamba_20091002').update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: holder(p, mori).update(until='1999-01-01'), 'Reversed historical interval'),
            (lambda p: holder(p, aso)['claim_ids'].append('jp_ldp_sousai09_tanigaki_first_press_conference_20090928'), 'cited source'),
            (lambda p: role(p)['claim_ids'].append('jp_ldp_does_not_exist'), 'Unknown'),
        ]
        for change, message in validator_cases:
            with self.subTest(message=message), self.assertRaisesRegex(ValueError, message):
                self.validate(mutated(change))

        def extra_holder(name, day, cid, start=False):
            return {'name': name, 'attested_on': None if start else day, 'from': day if start else None, 'until': None,
                    'sources': [self.claim_source[cid]], 'claim_ids': [cid], 'note': 'x', 'uncertainty': 'x'}

        rule_cases = [
            # A successor's election, selection or start used as an end.
            ('successor election used as an end (海部)', lambda p: holder(p, kaifu).update(until='1991-10-27')),
            ('successor start used as an end (橋本)', lambda p: holder(p, hashimoto).update(until='1998-07-24')),
            ('successor start used as an end (森)', lambda p: holder(p, mori).update(until='2001-04-24')),
            ('successor start used as an end (福田)', lambda p: holder(p, fukuda).update(until='2008-09-22')),
            ('successor attestation cited as an end (麻生)',
             cite(aso, 'jp_ldp_sousai09_tanigaki_first_press_conference_20090928', until='2009-09-28')),
            ('predecessor reference cited as an end (小渕)',
             cite(obuchi99, 'jp_ldpmori_predecessor_called_former_president_20000405', until='2000-04-05')),
            ('term-expiry schedule used as an end (麻生)', lambda p: holder(p, aso).update(until='2009-09-30')),
            ('announced resignation cited as an end (安倍)', cite(abe, 'jp_ldp_abe_announces_resignation_20070912', until='2007-09-12')),
            ('prospective resignation used as an end (麻生)', lambda p: holder(p, aso).update(until='2009-09-16')),
            ('retrospective span used as an end (宮澤)', cite(miyazawa, 'jp_ldp_list_miyazawa_span_19911031_19930730', until='1993-07-30')),
            # An election, nomination, declaration or list date used as a start.
            ('election date used as a start (安倍)', lambda p: holder(p, abe).update({'from': '2006-09-20', 'attested_on': None})),
            ('election date used as a start (宮澤)', lambda p: holder(p, miyazawa).update({'from': '1991-10-27', 'attested_on': None})),
            ('convention selection used as a start (橋本)', lambda p: holder(p, hashimoto).update({'from': '1995-09-25', 'attested_on': None})),
            ('list start used as a start (谷垣)', lambda p: holder(p, tanigaki).update({'from': '2009-10-01', 'attested_on': None})),
            ('nomination date used as a start (小泉 2003)', lambda p: holder(p, koizumi03).update({'from': '2003-09-08', 'attested_on': None})),
            ('就任 caption used as a start (小泉 2001)', lambda p: holder(p, koizumi01).update({'from': '2001-04-24', 'attested_on': None})),
            ('election claim cited by a holder (安倍)', cite(abe, 'jp_ldp_abe_elected_21st_president_20060920')),
            ('declaration cited by a holder (麻生)', cite(aso, 'jp_ldp_aso_election_declared_by_committee_chair_20080922')),
            ('nomination cited by a holder (谷垣)', cite(tanigaki, 'jp_ldp_sousai09_candidacies_filed_20090918')),
            ('retrospective list cited by a holder (森)', cite(mori, 'jp_ldp_list_mori_span_20000405_20010424')),
            ('unnamed caption cited by a holder (小渕 1998)', cite(obuchi98, 'jp_ldp98p1_joint_plenary_convened_19980724')),
            ('index headline cited by a holder (谷垣)', cite(tanigaki, 'jp_ldp_index_tanigaki_elected_24th_president_20090928')),
            ('named attestation left off its holder (福田)',
             lambda p: holder(p, fukuda)['claim_ids'].remove('jp_ldp_fukuda_first_press_conference_as_president_20070923')),
            # Acting, interim or continued service added as a holder.
            ('interim holder added in the 2000 vacancy', lambda p: role(p)['holder_claims'].insert(
                mori, extra_holder('森喜朗', '2000-04-04', 'jp_ldpmorisp_joint_plenary_elected_successor_20000405'))),
            ('continuation attestation added as a holder (小渕 1999-11-10)', lambda p: role(p)['holder_claims'].insert(
                mori, extra_holder('小渕恵三', '1999-11-10', 'jp_ldp_diet_obuchi_decides_as_party_president_19991110'))),
            ('incumbent before the vote added as a holder (橋本 1998)', lambda p: role(p)['holder_claims'].insert(
                obuchi98, extra_holder('橋本龍太郎', '1998-07-24', 'jp_ldp98p1_hashimoto_greets_joint_plenary_19980724'))),
            ('implicit reference added as a holder (宮澤 1992-02-20)', lambda p: role(p)['holder_claims'].insert(
                miyazawa, extra_holder('宮澤喜一', '1992-02-20', 'jp_ldp_diet_miyazawa_president_standpoint_19920220'))),
            # A cross-role or cross-institution holder.
            ('jp_pm claim cited by a party holder (森)', lambda p: holder(p, mori)['claim_ids'].append('jp_mori_appointed_pm_statement_20000405')),
            ('jp_pm holder moved onto the party presidency', lambda p: role(p)['holder_claims'].insert(
                mori + 1, copy.deepcopy(next(h for h in pm_role(p)['holder_claims'] if h['from'] == '2000-04-05')))),
            ('party holder moved onto the prime-ministership', lambda p: pm_role(p)['holder_claims'].append(
                copy.deepcopy(holder(p, tanigaki)))),
            ('party claim moved onto the prime-ministership', lambda p: pm_role(p)['claim_ids'].append(
                'jp_ldp_diet_kono_is_president_19941125')),
            ('party role copied onto an institution', lambda p: p['institutions'][0]['roles'].append(copy.deepcopy(role(p)))),
            ('party role copied to another party', lambda p: p['organizations'][0]['roles'].append(copy.deepcopy(role(p)))),
            ('second LDP role', lambda p: org(p)['roles'].append(dict(copy.deepcopy(role(p)), id='jp_ldp_vice_president'))),
            ('new institution', lambda p: p['institutions'].append(dict(copy.deepcopy(p['institutions'][-1]), id='jp_ldp_office'))),
            # The earlier observations, the order and the structured dates.
            ('2024 observation changed', lambda p: holder(p, ishiba).update(attested_on='2024-09-30')),
            ('holders out of order', lambda p: role(p)['holder_claims'].reverse()),
            ('retrospective span given a structured date', lambda p: claim(p, 'jp_ldp_list_abe_span_20061001_20070923').update(
                attested_on='2006-10-01')),
            ('beyond-cutoff attestation', lambda p: holder(p, tanigaki).update(attested_on='2026-09-08')),
        ]
        ldp_rules(self.packet, self.rows)
        ldp_invariants(self.packet, self.rows)
        for label, change in rule_cases:
            with self.subTest(label=label):
                packet = mutated(change)
                with self.assertRaises((AssertionError, KeyError, IndexError, ValueError, StopIteration)):
                    ldp_rules(packet, self.rows)
                with self.assertRaises((AssertionError, KeyError, IndexError, ValueError, StopIteration)):
                    ldp_invariants(packet, self.rows)
        # Row-level mutations: an unnamed notice or caption given a holder name, or a retrospective row relabelled.
        for cid, field, value in (('jp_ldp_sousai09_election_notice_20090918', 'holder_name', '谷垣禎一'),
                                  ('jp_ldp99p9_result_reported_to_congress_19990922', 'holder_name', '小渕恵三'),
                                  ('jp_ldp_list_tanigaki_span_20091001_20120930', 'event_kind', 'in_office_attestation'),
                                  ('jp_ldp_aso_states_will_resign_presidency_with_cabinet_20090908', 'event_kind', 'end_of_office_stated')):
            rows = copy.deepcopy(self.rows)
            rows[cid][field] = value
            with self.subTest(row=cid), self.assertRaises(AssertionError):
                ldp_invariants(self.packet, rows)
        # Event collapses and re-dating are caught by the pinned events.
        for cid, day in (('jp_ldp_sousai09_result_declared_by_chair_20090928', '2009-09-29'),
                         ('jp_ldp_2008_election_notice_20080910', '2008-09-22'),
                         ('jp_ldp99p9_result_reported_to_congress_19990922', '1999-09-21'),
                         ('jp_ldp_abe_announces_resignation_20070912', '2007-09-23')):
            with self.subTest(collapsed=cid), self.assertRaises(AssertionError):
                ldp_invariants(mutated(lambda p: claim(p, cid).update(attested_on=day)), self.rows)

    def test_report_and_handoff_close_no_parent_gate(self):
        self.assertIn('ready_for_review', self.report)
        table = self.section('Outcome')
        for number in range(1, 11):
            row, = [line for line in table.splitlines() if line.startswith(f'| LDP-PRES-{number:02d} ')]
            self.assertIn('**Accepted in part:**', row)
        defects = self.section('Checker defects')
        rows = [line for line in defects.splitlines() if re.match(r'\| [ABC]\d+ ', line)]
        self.assertEqual([re.match(r'\| ([ABC]\d+) ', r).group(1) for r in rows],
                         [f'A{n}' for n in range(1, 15)] + [f'B{n}' for n in range(1, 20)] + [f'C{n}' for n in range(1, 11)])
        for row in rows:
            self.assertRegex(row, r'\*\*(Applied|Applied in part|Resolved|Declined)')
        for marker in ('C01', 'C06', 'S23', 'WC1', 'CP1'):
            self.assertNotRegex(self.report, rf'\b{marker}\b[^.\n]*\bis (now )?complete\b')
        notes = self.section('Integration notes')
        for text in ('30c507d7', 'claude/c01-jp-13', '42f98dc8', 'research-index.json', 'only file', 'test_japan_research_s10d.py',
                     'test_japan_prime_ministers_c01_12.py', 'test_japan_prime_ministers_c01_13.py', 'test_campaign_census'):
            self.assertIn(text, notes)
        identities = self.section('Response identities and stability checks')
        for sid, (size, sha) in RESPONSES.items():
            self.assertIn(f'`{sid}`', identities, sid)
            self.assertIn(sha[:12], identities, sid)
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        for text in ('ready_for_review', 'japan-ldp-presidents-1990-2009-18.md', 'claude/c01-jp-18', '30c507d7',
                     'test_japan_ldp_presidents_c01_18.py', 'claude/c01-jp-13'):
            self.assertIn(text, handoff)
        index = research.build()
        country = next(p for p in index['countries'] if p['nation'] == 'Japan')
        self.assertFalse(country['country_census_complete'])
        self.assertIsNone(country['unrepresented_organization_count'])
        self.assertEqual((country['institution_observations'], country['role_observations']), (8, 5))
        self.assertFalse(index['c01_complete'])


if __name__ == '__main__':
    unittest.main()
