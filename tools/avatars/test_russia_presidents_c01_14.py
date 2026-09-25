"""CLAUDE-C01-14: Russian presidents, 1991-2026, keep election calling, voting, CEC results, oath, stated assumption of
office, inauguration ceremony, resignation, acting service and outgoing statements apart, and state a start or an end
only where a source does. The C01-05 RSFSR holders and the USSR packet are unchanged, and no holder is merged across the
RSFSR and Russian Federation titles."""
import copy
import hashlib
import json
import re
import unittest
from urllib.parse import urlsplit

import campaign_research as research


# Original response identity recorded in each extract: (bytes, sha256), re-downloaded byte-identical by the dossier,
# the independent check and this packet.
RESPONSES = {
    'ru_rf_law_2708i_19920421': (104315, '0df2faeca1a5fb1d7a1e0248f9e1762c701ffa53c7d77c70e1d7b4ec370306ba'),
    'ru_ks_post_9p_19921130': (371762, 'bd71ec4f7c44ff1a94602a2a6c8e471b6e7d46cb82270adea001388f4e0ebfc6'),
    'ru_ks_conclusion_19930323': (55038, 'bd3f3ddc5f300413858adfffaceb7282e04b5ad5d07d64a599f4d66dbe29885e'),
    'ru_ks_det_134o_19981105': (36602, '2c45f919d34688e7547d13235ee52abcdce13619d51cc34880e3cf38715d464f'),
    'ru_const_1993_original': (117303, '5b49db4a3544487690ff68114e1864412c633a4fd12dbf6b7fe7bdf60ed5e5ed'),
    'ru_sf_res_697i_19951115': (7748, 'd173929bbf18b58cdc30ed3c207a2dc01f13f317358d132a192e25c4c12a78b5'),
    'ru_cec_protocol_first_round_19960620': (79360, '6e587cc366331a6c4a94e8c5f354c3659c7f05b5d2ef96dedc59644c4c34f917'),
    'ru_cec_protocol_runoff_19960709': (15872, '0d1a31c3c0db730c9858714bfb6241d5d66d1d07dd25c519a2d08df7cae6043f'),
    'ru_cec_res_112_845ii_19960719': (22132, '0d10bf756926848f1328d2124b3a24ccd39cbeb6e0a738c6bfac1eee49d70c56'),
    'ru_ukaz_1138_19960805': (9749, '4ecae803e817f03016bfea28df341aaa43ffc419f25361d6bb48baefe6fe90ca'),
    'ru_duma_steno_19960810': (503153, 'c6a8530f2f88459a5af63e05bc8612784ba7b93e2f26e7ff3237efa385e65f74'),
    'ru_ukaz_1146_19960809': (24028, '82667085b973315be9d941ae90bfd429b9407b82eb126a14ebcf90531553228d'),
    'ru_pravo_ukaz_1761_19991231': (20095, '28c0900f1ebe3bed97172ab1bc418ca36f58ab557d7c2d111c588592e7fed2ed'),
    'ru_pravo_ukaz_1762_19991231': (19902, '5612fd3a91cbc0e663b48323776fba5ae2a4b318359ca17a1b6c9ca5efabbdbc'),
    'ru_kremlin_yeltsin_statement_19991231': (44320, '12d7d8387ea5a2f51224f3af9d087fcac6463fc952c7521a105e1cbcef2c0358'),
    'ru_kremlin_news_resignation_19991231': (10557, 'c796d1646b9555ef4e586a7c7f3f54ffbd7fa31aa41faf6292c044032a8d7b1f'),
    'ru_pravo_sf_4sf_20000105': (8365, '9f7ded63721b4550099368477daec671cc691ba86c9b29ab8b29931ff155bdf2'),
    'ru_kremlin_news_acting_president_20000506': (36393, 'a579074b5038f3199144890af88c981ad7991b062dd53ad303f62e043266f058'),
    'ru_fci_cec_results_20000326': (5207, '53f135dcb192e28e8a9d3c373e2fd0fb2f3de9ddf64dd0ee0bdce95d309e93d5'),
    'ru_fci_cec_resolutions_april_2000': (5647, '01e526644c6c8360184ba673c15e4a35e423b0fe626613ed63066f846a0e430a'),
    'ru_rg_cec_res_98_1110_3_20000405': (5684, 'af2e5bf0e82b3513e4025f08a4e576311a8c1d6dc27ce0b3fb113846d9a94981'),
    'ru_cec_protocol_2000_20000405': (57344, '2815f790dbaac18ea037bfbb0916342eb17341780af976d5fcf983122a4cb226'),
    'ru_kremlin_inauguration_stenogram_20000507': (64522, '3d6f0277b6363157eb154c03223670a66cef96819e867656c4c9b98f4c8d4da6'),
    'ru_kremlin_news_inauguration_20000507': (56816, '94ea965d2142e0c0cbdbf19a0b08ed84bbcf22d67189d5a3a4be289276acb205'),
    'ru_cec_res_106_1149_3_20000707': (32209, 'b4752f246849c61c35d09a8013164e5d6715e6930a09176292d6ae1369f75358'),
    'ru_pravo_sf_337sf_20031210': (7843, '698bc387bdd631a56f5d8af96aa264a5fde8f6a62fb782e188b7c4c375cd4cd1'),
    'ru_rg_cec_res_99_799_4_20040323': (5991, '0616440b8f3af10d6c159ef56e43b0bdc0d80ed9b8f093dc29551853e8277cb5'),
    'ru_kremlin_inauguration_broadcast_20040507': (62383, '77ce374313c190fa30d6949bc9acd0d08e1312ba55daa8c3994c1302cfe902e1'),
    'ru_kremlin_news_inauguration_20040507': (43710, '34c36670c3730a5bf06b96ba6f74bf543090a41512ee77724d3bb5c214b67412'),
    'ru_cec_res_125_902_4_20041029': (33983, '31e36b147c3cfa15c7ae3a897581013c4a4d7785eacc2f5eebed79d3a110c1a0'),
    'ru_cec_res_135_935_4_20050120': (14959, '760ab1e2689c1788422e418693a54fa4c983f8986cc81bb0d66bf25e2c827fbe'),
    'ru_cec_res_175_1128_4_20060425': (32055, 'b2d68d5f69d0b25aed3f51cdb1f0f567e7ac7729b5d3cbe59298ec25fd78d9b2'),
    'ru_fc_res_550sf_20071126': (32327, '1d0ca88f17a45eb4aa01581602495b72d32445f925eae1bbc968c954d06e2615'),
    'ru_cec_res_104_777_5_20080307': (15657, 'eee45922db95c0baf2410dd0416f4e722bfb8ec1bb41bc032d9bd1edfae2274d'),
    'ru_rg_cec_res_104_777_5_20080308': (41974, '9d871e47a8ee5d74604b65e8d9ce34808bdd3904c9683a9d8f176d68922734d9'),
    'ru_kremlin_news_4_20080507': (57494, '3410c4de15b4ea69349c05f3abf0e429182246a801d21354f723303bc4eceacd'),
    'ru_kremlin_transcript_3_20080507': (38354, 'b2c728d1c736ea6cb7e4e29c7e1a50de14a82032e7cc3b7b5b83fc563eeb29f5'),
    'ru_kremlin_news_11_20080507': (37548, '21d2277343dc2be0d2e24273d6fe66eef45b3873a10fa71ddb3ffcd0ba41418d'),
    'ru_fc_res_442sf_20111125': (24040, '35c52f74636351067fbc9c8b9643a2aa2d71f740093021477ca184449c524776'),
    'ru_kremlin_news_14680_20120304': (46512, 'e1423d45c1c49375a2340fc2cb2d230d25108eb6fea4cfd44388277b006636a0'),
    'ru_cec_res_112_893_6_20120307': (12676, 'e40cc33478c250cb083e61414ce8e826d72ed5639c812bead7db6eaef70baf48'),
    'ru_rg_cec_res_112_893_6_20120308': (45730, '847bc03abccd7d5bfcdb8903d499fc044fd1053341a9a2079a562b87b1989006'),
    'ru_kremlin_news_15224_20120507': (16063, '1c0d7d4e272d4c34aeecbbffe07083531993bcec7b142dadec2839b1996348ef'),
    'ru_fc_res_528sf_20171215': (20353, 'effab7f8549cfc42bf780e94a14d7fbcd22f5acaef788d110ceab3c46c9f5e3c'),
    'ru_kremlin_news_57083_20180318': (42708, '6df21b995a5ba5263c64fc2a6f83616e70a04c54c1f5547edd37d9b766640963'),
    'ru_cec_res_152_1255_7_20180323': (40311, '50e3a6fae32c4c0966f05a3ebbc6f13f41ae37158f817ab554b28359018a9188'),
    'ru_rg_cec_res_152_1255_7_20180324': (143832, '322f1a53086ba885615ff734fb841759d6b85bc561c9643845705f64664f126c'),
    'ru_kremlin_news_57416_20180507': (20506, 'e0ac877d3d2639692cf93aef4e462f12c15cbb221f7050492999b616bacab8d7'),
    'ru_fc_res_678sf_20231207': (24272, '50613a83cbb52563c7ad10426f4996501ac67487acd404bbd08f6a178bdd2915'),
    'ru_kremlin_news_73658_20240315': (37431, '3dffdcc9816ea8c6a5de2c0857884e1fdfe2f62d5224f3bf13cd4024d25c9c86'),
    'ru_rg_cec_res_163_1291_8_20240321': (92949, 'e7ba12c9024fe8b5928c082ccaf34c2782f107a457e56fe49a8e8f61f20a937f'),
    'ru_kremlin_news_73981_20240507': (13029, 'a55150b95b758046ce44d6c7896019f7bc7d93c84d45a8b0b7ae554f541af56c'),
    'ru_pub_decree_636_20260904': (32911, '986757f81ecc143f0cd0beed083fe5f3f2c0cea99384c77648c759c667269c80'),
}
NEW_SOURCES = list(RESPONSES)
# Raw Internet Archive captures (id_ form, all made before the 7 September 2026 cutoff): id -> 14-digit timestamp.
ARCHIVED = {
    'ru_cec_protocol_first_round_19960620': '20160401180417',
    'ru_cec_protocol_runoff_19960709': '20160401153401',
    'ru_cec_res_112_845ii_19960719': '20160401213450',
    'ru_kremlin_yeltsin_statement_19991231': '20260212101412',
    'ru_kremlin_news_resignation_19991231': '20260405070524',
    'ru_kremlin_news_acting_president_20000506': '20260611174508',
    'ru_fci_cec_results_20000326': '20000511054425',
    'ru_fci_cec_resolutions_april_2000': '20000831231210',
    'ru_rg_cec_res_98_1110_3_20000405': '20010218034958',
    'ru_cec_protocol_2000_20000405': '20140802013752',
    'ru_kremlin_inauguration_stenogram_20000507': '20260208102524',
    'ru_kremlin_news_inauguration_20000507': '20260514112545',
    'ru_cec_res_106_1149_3_20000707': '20140801070526',
    'ru_rg_cec_res_99_799_4_20040323': '20041021110403',
    'ru_kremlin_inauguration_broadcast_20040507': '20260519104156',
    'ru_kremlin_news_inauguration_20040507': '20251206040020',
    'ru_cec_res_125_902_4_20041029': '20111119154354',
    'ru_cec_res_135_935_4_20050120': '20111119155204',
    'ru_cec_res_175_1128_4_20060425': '20111119155112',
    'ru_cec_res_104_777_5_20080307': '20080313193604',
    'ru_rg_cec_res_104_777_5_20080308': '20080310004304',
    'ru_kremlin_news_4_20080507': '20260312204311',
    'ru_kremlin_transcript_3_20080507': '20220526201821',
    'ru_kremlin_news_11_20080507': '20260517065034',
    'ru_kremlin_news_14680_20120304': '20230202123957',
    'ru_cec_res_112_893_6_20120307': '20120311220745',
    'ru_rg_cec_res_112_893_6_20120308': '20120311050014',
    'ru_kremlin_news_15224_20120507': '20251012044258',
    'ru_kremlin_news_57083_20180318': '20180320095741',
    'ru_cec_res_152_1255_7_20180323': '20180324211630',
    'ru_rg_cec_res_152_1255_7_20180324': '20180324135637',
    'ru_kremlin_news_57416_20180507': '20251229124214',
    'ru_kremlin_news_73658_20240315': '20260513124847',
    'ru_rg_cec_res_163_1291_8_20240321': '20240405043140',
    'ru_kremlin_news_73981_20240507': '20250504092901',
}
# Responses read over HTTP because HTTPS (port 443) failed; the packet URL is the same path on HTTPS.
HTTP_READ = {
    'ru_rf_law_2708i_19920421': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102015844&page=1&rdk=0',
    'ru_ks_post_9p_19921130': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102020039&page=1&rdk=0',
    'ru_ks_conclusion_19930323': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102022316&page=1&rdk=0',
    'ru_ks_det_134o_19981105': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102056287&page=1&rdk=0',
    'ru_const_1993_original': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102027595&page=1&rdk=0',
    'ru_sf_res_697i_19951115': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102038135&page=1&rdk=0',
    'ru_ukaz_1138_19960805': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102042830&page=1&rdk=0',
    'ru_duma_steno_19960810': 'http://transcript.duma.gov.ru/node/2898/',
    'ru_ukaz_1146_19960809': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102042895&page=1&rdk=0',
    'ru_pravo_ukaz_1761_19991231': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102063835&page=1&rdk=0',
    'ru_pravo_ukaz_1762_19991231': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102063834&page=1&rdk=0',
    'ru_pravo_sf_4sf_20000105': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102063946&page=1&rdk=0',
    'ru_pravo_sf_337sf_20031210': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102084559&page=1&rdk=0',
    'ru_fc_res_550sf_20071126': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102118390&page=1&rdk=0',
    'ru_fc_res_442sf_20111125': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102152264&page=1&rdk=0',
    'ru_fc_res_528sf_20171215': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102454764&page=1&rdk=0',
    'ru_fc_res_678sf_20231207': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=606168232&page=1&rdk=0',
    'ru_pub_decree_636_20260904': 'http://publication.pravo.gov.ru/file/pdf?eoNumber=0001202609040002',
}
# Hashed responses attached to a source (portal cards, CEC index pages, annexes, facsimiles, corroborating captures).
ATTACHED = {
    'ru_rf_law_2708i_19920421': [('portal_card_response', 9558, '89cdc72d2525b87734d4753ea7aaaae9f59e787ea886b986262b81510e242210')],
    'ru_ks_post_9p_19921130': [('portal_card_response', 80685, 'b60f225bf7ad3ae18650065274ddee6d2c15d787a225b8d0f0ad9ef602431ca0')],
    'ru_ks_conclusion_19930323': [('portal_card_response', 9452, 'c3902c4ebe6f993d13a313128d983900d9d6b806f12430bdbc33a6194b971a5d')],
    'ru_ks_det_134o_19981105': [('portal_card_response', 20681, 'f056140cee8d9bf7ab3c6df173aee1d0587c1c30a1fda8bee4472b3d918453b4')],
    'ru_const_1993_original': [('portal_card_response', 10151, '019a9adf93fbc4a79d28787f4069c9e425aa664a6a897c9675be31a0c847995f')],
    'ru_sf_res_697i_19951115': [('portal_card_response', 135597, 'ca463b87eb994a086aa975e4fc69fa04ac6a359df95861c4b9965820a46a20aa')],
    'ru_cec_protocol_runoff_19960709': [('index_response', 15137, 'b8d50e1fdf879e6d82191a177d9668bdba37e8a04789711af980074fc2f58836')],
    'ru_ukaz_1138_19960805': [('portal_card_response', 5322, '91aed1574a5c9cf8504a4807f4cecb6bfff1a9ab8610777bf6aefb5706b9bf48')],
    'ru_ukaz_1146_19960809': [('portal_card_response', 21917, '1f61933b5c54fccc9de6f76b598695403f5c443c9126c6f50c65afa53ae51276')],
    'ru_pravo_ukaz_1761_19991231': [('portal_card_response', 3205, '2e458ea838c8c4261199d5b5f3bc135eb6ac5b4a5eb88b46c1dde9b7488e91e1'), ('corroborating_responses', 7429, '7fc6d35f59174dda18934c55f7543ab30c475113e05a7eb301d3f8982036f182')],
    'ru_pravo_ukaz_1762_19991231': [('portal_card_response', 3214, '4788164f2b215c7a0b0bd253de2dbc8dd5d0569c6301b89383c81dab398c44f9')],
    'ru_pravo_sf_4sf_20000105': [('portal_card_response', 3179, 'ece7d1cbde676577aa6ea66009152c915ac5565f2fcf2a5894868b7bf4e7ae3e')],
    'ru_rg_cec_res_98_1110_3_20000405': [('annex_responses', 4893, '8279818e8003d650d1b4ceea5c828d4e0753b5deb0f5273efc6e7bca424e094e')],
    'ru_cec_protocol_2000_20000405': [('index_response', 10968, '6809b85012cf2e3018b6ff38d1bd070aec9508d1a92e87005bdfa6cc2682edf7')],
    'ru_pravo_sf_337sf_20031210': [('portal_card_response', 3246, '8254367c00c3de14663b059751cd3c64e2e759361c784d2cf01b5fa612300d4f')],
    'ru_cec_res_125_902_4_20041029': [('index_response', 10287, 'c908538bf462bbe7a5741fff5e31a7038c2abf68a64c6f62930ffd7de179e895')],
    'ru_fc_res_550sf_20071126': [('portal_card_response', 44147, 'a7f5e26145f76784c3aba83cbaa201ea5e16edb4554f036e806ca79a41e4ae85')],
    'ru_rg_cec_res_104_777_5_20080308': [('annex_responses', 11240, '25a076d2ea7e69ded51dfc64ed3ede75177bc3f1b6f2faf92195793e20804ce1')],
    'ru_kremlin_transcript_3_20080507': [('corroborating_responses', 77737, 'ac1a5e24818ba695ab9ec439a640104d81c6bf556c62a704cadb79b177868a77')],
    'ru_fc_res_442sf_20111125': [('portal_card_response', 59211, 'ee5074acbfe1eee43c8212b043b66b6ffda98d136c8fef066821d54fc207b1a7'), ('facsimile_responses', 45191, '768c128e3066bba4c9495d9098348537c23a99ae68ecd153760fdf98725072ac')],
    'ru_fc_res_528sf_20171215': [('portal_card_response', 46955, '6b88f9738262d93c1e0937e2ee5ff5f7014b457d7483eba1faa4f02403249a8a'), ('facsimile_responses', 65007, '2fa290f529eb6e8d37758fe8fee1c1a4f1297fa7f220f1fee730bd03f04fac3f')],
    'ru_cec_res_152_1255_7_20180323': [('annex_responses', 18560, '21e55f09f8ef6a0c2debd25c8ed568b9f40cab0d076871a49cd298e502b45c02')],
    'ru_fc_res_678sf_20231207': [('portal_card_response', 11992, '2ae2edce15034f18ac834a5de6cf25b6a0d0b7c26cdc5307d95d73b6ea0aef71'), ('facsimile_responses', 123828, '37be57dc2de28ebaa6b80e84c4e7fa17ea8aef9d977af397f9a208497de2895b')],
    'ru_rg_cec_res_163_1291_8_20240321': [('facsimile_responses', 85147, '6ff16d38b85f6fb562bd96b45abc2aa63c6f101df3257752dc5d56f88e598075')],
    'ru_pub_decree_636_20260904': [('card_response', 20071, '1acd4fc56f14c2962d725adc7d3b1ab3ab38a4ad6f52c83a29e48f87fd10bb90')],
}
# The kremlin.ru capture served gzip-compressed: the identity is the compressed body; the decoded body is pinned too.
GZIP_DECODED = {'ru_kremlin_news_resignation_19991231': (42935, '6c08885ac31d0a54ef597e2cc182e5a1934049e1fd85252a5788ec04bf1f7913')}
# Every new claim's (attested_on, event_kind, review observation), exactly: distinct dated events are never re-dated,
# relabelled or merged.
EVENTS = {
    'ru_law_2708i_constitution_renamed_19920421': ('1992-04-21', 'title_renaming', 'RU-PRES-01'),
    'ru_law_2708i_art121_8_president_rf_19920421': ('1992-04-21', 'procedure', 'RU-PRES-01'),
    'ru_law_2708i_in_force_on_publication_19920421': ('1992-04-21', 'entry_into_force_rule', 'RU-PRES-01'),
    'ru_law_2708i_published_signature_19920421': ('1992-04-21', 'office_signature_published', 'RU-PRES-01'),
    'ru_ks_9p_decrees_styled_president_rf_19921130': ('1992-11-30', 'retitled_office_attestation', 'RU-PRES-01'),
    'ru_ks_yeltsin_styled_president_rf_19930323': ('1993-03-23', 'retitled_office_attestation', 'RU-PRES-01'),
    'ru_ks_134o_rsfsr_president_retitled_19981105': (None, 'retrospective_retitling_statement', 'RU-PRES-01'),
    'ru_ks_134o_continuation_new_constitution_19931225': ('1993-12-25', 'continuation', 'RU-PRES-02'),
    'ru_ks_134o_cec_declared_second_term_19960709': ('1996-07-09', 'result_declaration', 'RU-PRES-03'),
    'ru_ks_134o_inauguration_certificate_19960809': ('1996-08-09', 'inauguration_ceremony', 'RU-PRES-03'),
    'ru_ks_134o_oath_of_office_19960809': ('1996-08-09', 'oath_of_office', 'RU-PRES-03'),
    'ru_ks_134o_assumption_of_office_19960809': ('1996-08-09', 'assumption_of_office', 'RU-PRES-03'),
    'ru_ks_134o_determination_terms_19981105': ('1998-11-05', 'court_determination', 'RU-PRES-02'),
    'ru_ks_134o_dissent_identity_question_19981105': ('1998-11-05', 'separate_opinion', 'RU-PRES-01'),
    'ru_const1993_entry_into_force_rule': (None, 'procedure', 'RU-PRES-02'),
    'ru_const1993_transitional_president_rule': (None, 'procedure', 'RU-PRES-02'),
    'ru_const1993_art80_head_of_state': (None, 'procedure', 'RU-PRES-02'),
    'ru_const1993_oath_and_term_rules': (None, 'procedure', 'RU-PRES-02'),
    'ru_portal_const1993_publication_citation': (None, 'publication_citation', 'RU-PRES-02'),
    'ru_sf_697i_election_called_19951115': ('1995-11-15', 'election_called', 'RU-PRES-03'),
    'ru_cec_first_round_voting_19960616': ('1996-06-16', 'election_voting', 'RU-PRES-03'),
    'ru_cec_first_round_protocol_figures_19960620': ('1996-06-20', 'result_determination', 'RU-PRES-03'),
    'ru_cec_runoff_voting_19960703': ('1996-07-03', 'election_voting', 'RU-PRES-03'),
    'ru_cec_runoff_protocol_figures_19960709': ('1996-07-09', 'result_determination', 'RU-PRES-03'),
    'ru_cec_protocol_amended_dagestan_19960719': ('1996-07-19', 'result_correction', 'RU-PRES-03'),
    'ru_ukaz_1138_inauguration_symbols_rules_19960805': ('1996-08-05', 'procedure', 'RU-PRES-03'),
    'ru_duma_steno_inauguration_held_19960809': ('1996-08-09', 'inauguration_ceremony', 'RU-PRES-03'),
    'ru_ukaz_1146_signed_as_president_19960809': ('1996-08-09', 'in_office_signature', 'RU-PRES-03'),
    'ru_ukaz_1761_yeltsin_ceases_powers_19991231': ('1999-12-31', 'resignation_effective', 'RU-PRES-04'),
    'ru_ukaz_1761_pm_to_act_19991231': ('1999-12-31', 'acting_service_designation', 'RU-PRES-04'),
    'ru_ukaz_1762_yeltsin_resignation_cited_19991231': ('1999-12-31', 'resignation_reference', 'RU-PRES-04'),
    'ru_ukaz_1762_putin_begins_acting_19991231': ('1999-12-31', 'acting_service_start', 'RU-PRES-04'),
    'ru_kremlin_yeltsin_address_resigns_19991231': ('1999-12-31', 'resignation_announcement', 'RU-PRES-04'),
    'ru_kremlin_yeltsin_address_decree_acting_19991231': ('1999-12-31', 'acting_service_designation', 'RU-PRES-04'),
    'ru_kremlin_news_resignation_televised_19991231': ('1999-12-31', 'resignation_announcement', 'RU-PRES-04'),
    'ru_sf_4sf_yeltsin_ceased_by_resignation_20000105': ('2000-01-05', 'resignation_reference', 'RU-PRES-04'),
    'ru_sf_4sf_early_election_scheduled_20000105': ('2000-01-05', 'election_called', 'RU-PRES-05'),
    'ru_kremlin_putin_styled_acting_president_20000506': ('2000-05-06', 'acting_service_attestation', 'RU-PRES-04'),
    'ru_cec_election_voting_20000326': ('2000-03-26', 'election_voting', 'RU-PRES-05'),
    'ru_cec_results_table_20000326': ('2000-03-26', 'result_figures', 'RU-PRES-05'),
    'ru_cec_res_97_1110_3_results_20000405': ('2000-04-05', 'result_declaration_index_entry', 'RU-PRES-05'),
    'ru_cec_res_97_1111_3_certificate_20000405': ('2000-04-05', 'election_certificate_index_entry', 'RU-PRES-05'),
    'ru_rg_cec_98_1110_3_putin_elected_20000405': ('2000-04-05', 'result_declaration', 'RU-PRES-05'),
    'ru_rg_cec_98_1110_3_candidate_figures_20000405': ('2000-04-05', 'result_figures', 'RU-PRES-05'),
    'ru_cec_protocol_2000_figures_20000405': ('2000-04-05', 'result_determination', 'RU-PRES-05'),
    'ru_kremlin_20000507_cec_chair_announces_result': ('2000-05-07', 'result_announcement', 'RU-PRES-05'),
    'ru_kremlin_20000507_oath': ('2000-05-07', 'oath_of_office', 'RU-PRES-05'),
    'ru_kremlin_20000507_putin_assumed_office': ('2000-05-07', 'assumption_of_office', 'RU-PRES-05'),
    'ru_kremlin_news_20000507_inauguration_noon': ('2000-05-07', 'inauguration_ceremony', 'RU-PRES-05'),
    'ru_cec_106_1149_3_result_correction_20000707': ('2000-07-07', 'result_correction', 'RU-PRES-05'),
    'ru_sf_337sf_election_scheduled_20031210': ('2003-12-10', 'election_called', 'RU-PRES-06'),
    'ru_cec_res_99_799_4_putin_elected_20040323': ('2004-03-23', 'result_declaration', 'RU-PRES-06'),
    'ru_cec_res_99_799_4_candidate_figures_20040323': ('2004-03-23', 'result_figures', 'RU-PRES-06'),
    'ru_kremlin_20040507_oath': ('2004-05-07', 'oath_of_office', 'RU-PRES-06'),
    'ru_kremlin_20040507_putin_assumed_office': ('2004-05-07', 'assumption_of_office', 'RU-PRES-06'),
    'ru_kremlin_20040507_narration_vote_20040314': ('2004-03-14', 'retrospective_statement', 'RU-PRES-06'),
    'ru_kremlin_20040507_narration_first_oath_20000507': ('2000-05-07', 'retrospective_statement', 'RU-PRES-06'),
    'ru_kremlin_news_20040507_inauguration_ceremony': ('2004-05-07', 'inauguration_ceremony', 'RU-PRES-06'),
    'ru_cec_125_902_4_voting_20040314': ('2004-03-14', 'election_voting', 'RU-PRES-06'),
    'ru_cec_125_902_4_result_correction_20041029': ('2004-10-29', 'result_correction', 'RU-PRES-06'),
    'ru_cec_135_935_4_result_correction_20050120': ('2005-01-20', 'result_correction', 'RU-PRES-06'),
    'ru_cec_175_1128_4_result_correction_20060425': ('2006-04-25', 'result_correction', 'RU-PRES-06'),
    'ru_fc_550sf_election_called_20071126': ('2007-11-26', 'election_called', 'RU-PRES-07'),
    'ru_cec_104_777_5_medvedev_elected_20080307': ('2008-03-07', 'result_declaration', 'RU-PRES-07'),
    'ru_rg_104_777_5_published_20080308': ('2008-03-08', 'result_publication', 'RU-PRES-07'),
    'ru_rg_104_777_5_candidate_figures_20080307': ('2008-03-07', 'result_figures', 'RU-PRES-07'),
    'ru_kremlin_medvedev_oath_20080507': ('2008-05-07', 'oath_of_office', 'RU-PRES-07'),
    'ru_kremlin_medvedev_took_office_20080507': ('2008-05-07', 'assumption_of_office', 'RU-PRES-07'),
    'ru_kremlin_election_held_20080302': ('2008-03-02', 'election_voting', 'RU-PRES-07'),
    'ru_kremlin_steno_putin_outgoing_20080507': ('2008-05-07', 'outgoing_holder_statement', 'RU-PRES-07'),
    'ru_kremlin_steno_medvedev_oath_20080507': ('2008-05-07', 'oath_of_office', 'RU-PRES-07'),
    'ru_kremlin_steno_zorkin_medvedev_took_office_20080507': ('2008-05-07', 'assumption_of_office', 'RU-PRES-07'),
    'ru_kremlin_medvedev_styled_president_nuclear_control_20080507': ('2008-05-07', 'in_office_attestation', 'RU-PRES-07'),
    'ru_fc_442sf_election_called_20111125': ('2011-11-25', 'election_called', 'RU-PRES-08'),
    'ru_kremlin_medvedevs_voted_20120304': ('2012-03-04', 'election_voting', 'RU-PRES-08'),
    'ru_cec_112_893_6_putin_elected_20120307': ('2012-03-07', 'result_declaration', 'RU-PRES-08'),
    'ru_rg_112_893_6_published_20120308': ('2012-03-08', 'result_publication', 'RU-PRES-08'),
    'ru_kremlin_medvedev_concluding_presidency_20120507': ('2012-05-07', 'outgoing_holder_statement', 'RU-PRES-08'),
    'ru_kremlin_putin_oath_20120507': ('2012-05-07', 'oath_of_office', 'RU-PRES-08'),
    'ru_kremlin_zorkin_putin_took_office_20120507': ('2012-05-07', 'assumption_of_office', 'RU-PRES-08'),
    'ru_fc_528sf_election_called_20171215': ('2017-12-15', 'election_called', 'RU-PRES-09'),
    'ru_kremlin_putin_voted_20180318': ('2018-03-18', 'election_voting', 'RU-PRES-09'),
    'ru_cec_152_1255_7_putin_elected_20180323': ('2018-03-23', 'result_declaration', 'RU-PRES-09'),
    'ru_rg_152_1255_7_published_20180324': ('2018-03-24', 'result_publication', 'RU-PRES-09'),
    'ru_rg_152_1255_7_candidate_figures_20180323': ('2018-03-23', 'result_figures', 'RU-PRES-09'),
    'ru_kremlin_putin_oath_20180507': ('2018-05-07', 'oath_of_office', 'RU-PRES-09'),
    'ru_kremlin_putin_took_office_20180507': ('2018-05-07', 'assumption_of_office', 'RU-PRES-09'),
    'ru_fc_678sf_election_called_20231207': ('2023-12-07', 'election_called', 'RU-PRES-10'),
    'ru_kremlin_voting_15_17_march_20240315': ('2024-03-15', 'election_voting', 'RU-PRES-10'),
    'ru_cec_163_1291_8_putin_elected_20240321': ('2024-03-21', 'result_declaration', 'RU-PRES-10'),
    'ru_rg_163_1291_8_candidate_figures_20240321': ('2024-03-21', 'result_figures', 'RU-PRES-10'),
    'ru_kremlin_putin_oath_20240507': ('2024-05-07', 'oath_of_office', 'RU-PRES-10'),
    'ru_kremlin_putin_took_office_20240507': ('2024-05-07', 'assumption_of_office', 'RU-PRES-10'),
    'ru_decree_636_putin_signs_as_president_20260904': ('2026-09-04', 'in_office_attestation', 'RU-PRES-10'),
}

YELTSIN, PUTIN, MEDVEDEV = 'Борис Николаевич Ельцин', 'Владимир Владимирович Путин', 'Дмитрий Анатольевич Медведев'
PR, RSFSR, VICE = 'ru_president', 'ru_rsfsr_president', 'ru_rsfsr_vice_president'
PRESIDENT_TITLE = 'Президент Российской Федерации — President of the Russian Federation'
ACTING_TITLE = 'Исполняющий обязанности Президента Российской Федерации — Acting President of the Russian Federation'
# Exact holder observations on ru_president, in chronological order: (name, attested_on, from, until).
HOLDERS = [
    (YELTSIN, None, '1996-08-09', '1999-12-31'),
    (PUTIN, None, '2000-05-07', None),
    (PUTIN, None, '2004-05-07', None),
    (MEDVEDEV, None, '2008-05-07', None),
    (PUTIN, None, '2012-05-07', None),
    (PUTIN, None, '2018-05-07', None),
    (PUTIN, None, '2024-05-07', None),
]
HOLDER_CLAIMS = [
    ['ru_ks_134o_assumption_of_office_19960809', 'ru_ks_134o_oath_of_office_19960809', 'ru_ukaz_1761_yeltsin_ceases_powers_19991231'],
    ['ru_kremlin_20000507_putin_assumed_office', 'ru_kremlin_20000507_oath'],
    ['ru_kremlin_20040507_putin_assumed_office', 'ru_kremlin_20040507_oath'],
    ['ru_kremlin_steno_zorkin_medvedev_took_office_20080507', 'ru_kremlin_steno_medvedev_oath_20080507',
     'ru_kremlin_medvedev_took_office_20080507', 'ru_kremlin_medvedev_oath_20080507'],
    ['ru_kremlin_zorkin_putin_took_office_20120507', 'ru_kremlin_putin_oath_20120507'],
    ['ru_kremlin_putin_took_office_20180507', 'ru_kremlin_putin_oath_20180507'],
    ['ru_kremlin_putin_took_office_20240507', 'ru_kremlin_putin_oath_20240507', 'ru_decree_636_putin_signs_as_president_20260904'],
]
# Each holder start rests on exactly one kind of claim: a stated assumption of office. The one end is a stated
# cessation of powers.
STARTS = {'1996-08-09': 'ru_ks_134o_assumption_of_office_19960809', '2000-05-07': 'ru_kremlin_20000507_putin_assumed_office',
          '2004-05-07': 'ru_kremlin_20040507_putin_assumed_office', '2008-05-07': 'ru_kremlin_steno_zorkin_medvedev_took_office_20080507',
          '2012-05-07': 'ru_kremlin_zorkin_putin_took_office_20120507', '2018-05-07': 'ru_kremlin_putin_took_office_20180507',
          '2024-05-07': 'ru_kremlin_putin_took_office_20240507'}
ENDS = {'1999-12-31': 'ru_ukaz_1761_yeltsin_ceases_powers_19991231'}
HOLDER_KINDS = {'oath_of_office', 'assumption_of_office', 'resignation_effective', 'in_office_attestation'}

# Claims that must never feed a holder observation.
ELECTIONS = (
    'ru_sf_697i_election_called_19951115', 'ru_cec_first_round_voting_19960616', 'ru_cec_first_round_protocol_figures_19960620',
    'ru_cec_runoff_voting_19960703', 'ru_cec_runoff_protocol_figures_19960709', 'ru_ks_134o_cec_declared_second_term_19960709',
    'ru_cec_protocol_amended_dagestan_19960719', 'ru_sf_4sf_early_election_scheduled_20000105', 'ru_cec_election_voting_20000326',
    'ru_cec_results_table_20000326', 'ru_cec_res_97_1110_3_results_20000405', 'ru_cec_res_97_1111_3_certificate_20000405',
    'ru_rg_cec_98_1110_3_putin_elected_20000405', 'ru_rg_cec_98_1110_3_candidate_figures_20000405',
    'ru_cec_protocol_2000_figures_20000405', 'ru_kremlin_20000507_cec_chair_announces_result',
    'ru_cec_106_1149_3_result_correction_20000707', 'ru_sf_337sf_election_scheduled_20031210', 'ru_cec_125_902_4_voting_20040314',
    'ru_cec_res_99_799_4_putin_elected_20040323', 'ru_cec_res_99_799_4_candidate_figures_20040323',
    'ru_cec_125_902_4_result_correction_20041029', 'ru_cec_135_935_4_result_correction_20050120',
    'ru_cec_175_1128_4_result_correction_20060425', 'ru_fc_550sf_election_called_20071126', 'ru_kremlin_election_held_20080302',
    'ru_cec_104_777_5_medvedev_elected_20080307', 'ru_rg_104_777_5_candidate_figures_20080307', 'ru_rg_104_777_5_published_20080308',
    'ru_fc_442sf_election_called_20111125', 'ru_kremlin_medvedevs_voted_20120304', 'ru_cec_112_893_6_putin_elected_20120307',
    'ru_rg_112_893_6_published_20120308', 'ru_fc_528sf_election_called_20171215', 'ru_kremlin_putin_voted_20180318',
    'ru_cec_152_1255_7_putin_elected_20180323', 'ru_rg_152_1255_7_candidate_figures_20180323', 'ru_rg_152_1255_7_published_20180324',
    'ru_fc_678sf_election_called_20231207', 'ru_kremlin_voting_15_17_march_20240315', 'ru_cec_163_1291_8_putin_elected_20240321',
    'ru_rg_163_1291_8_candidate_figures_20240321')
CEREMONIES = ('ru_ks_134o_inauguration_certificate_19960809', 'ru_duma_steno_inauguration_held_19960809',
              'ru_kremlin_news_20000507_inauguration_noon', 'ru_kremlin_news_20040507_inauguration_ceremony')
ACTING = ('ru_ukaz_1761_pm_to_act_19991231', 'ru_ukaz_1762_putin_begins_acting_19991231',
          'ru_kremlin_yeltsin_address_decree_acting_19991231', 'ru_kremlin_putin_styled_acting_president_20000506')
RESIGNATION_CONTEXT = ('ru_ukaz_1762_yeltsin_resignation_cited_19991231', 'ru_kremlin_yeltsin_address_resigns_19991231',
                       'ru_kremlin_news_resignation_televised_19991231', 'ru_sf_4sf_yeltsin_ceased_by_resignation_20000105')
OUTGOING = ('ru_kremlin_steno_putin_outgoing_20080507', 'ru_kremlin_medvedev_concluding_presidency_20120507')
PROCEDURE = ('ru_law_2708i_art121_8_president_rf_19920421', 'ru_law_2708i_in_force_on_publication_19920421',
             'ru_const1993_entry_into_force_rule', 'ru_const1993_transitional_president_rule', 'ru_const1993_art80_head_of_state',
             'ru_const1993_oath_and_term_rules', 'ru_portal_const1993_publication_citation', 'ru_ukaz_1138_inauguration_symbols_rules_19960805')
RETITLING = ('ru_law_2708i_constitution_renamed_19920421', 'ru_law_2708i_published_signature_19920421',
             'ru_ks_9p_decrees_styled_president_rf_19921130', 'ru_ks_yeltsin_styled_president_rf_19930323',
             'ru_ks_134o_rsfsr_president_retitled_19981105', 'ru_ks_134o_dissent_identity_question_19981105')
RETROSPECTIVE = ('ru_ks_134o_continuation_new_constitution_19931225', 'ru_ks_134o_determination_terms_19981105',
                 'ru_kremlin_20040507_narration_vote_20040314', 'ru_kremlin_20040507_narration_first_oath_20000507')
UNUSED_ATTESTATIONS = ('ru_ukaz_1146_signed_as_president_19960809', 'ru_kremlin_medvedev_styled_president_nuclear_control_20080507')
NEVER_HOLDER = ELECTIONS + CEREMONIES + ACTING + RESIGNATION_CONTEXT + OUTGOING + PROCEDURE + RETITLING + RETROSPECTIVE + UNUSED_ATTESTATIONS
# Claims printed without a structured date: constitution text and one retrospective court statement.
UNDATED = ('ru_ks_134o_rsfsr_president_retitled_19981105', 'ru_const1993_entry_into_force_rule',
           'ru_const1993_transitional_president_rule', 'ru_const1993_art80_head_of_state', 'ru_const1993_oath_and_term_rules',
           'ru_portal_const1993_publication_citation')
# Dates that are never any holder's attested_on, start or end: renaming and retitling, the continuation, election calling,
# voting, protocols, declarations, publications and corrections, the acting attestation, and the latest attestation.
NEVER_HOLDER_DATE = {
    '1991-12-25', '1992-04-21', '1992-11-30', '1993-03-23', '1993-12-25', '1995-11-15', '1996-06-16', '1996-06-20', '1996-07-03',
    '1996-07-09', '1996-07-19', '1996-08-05', '1996-08-10', '1998-11-05', '2000-01-05', '2000-03-26', '2000-04-05', '2000-05-06',
    '2000-07-07', '2003-12-10', '2004-03-14', '2004-03-23', '2004-10-29', '2005-01-20', '2006-04-25', '2007-11-26', '2008-03-02',
    '2008-03-07', '2008-03-08', '2011-11-25', '2012-03-04', '2012-03-07', '2012-03-08', '2017-12-15', '2018-03-18', '2018-03-23',
    '2018-03-24', '2023-12-07', '2024-03-15', '2024-03-21', '2026-09-04'}
# Renamed or split per the checks, moved to leads, or secondary: none of these may appear in the packet.
STALE_IDS = ('ru_ks_134o_oath_assumed_office_19960809', 'ru_ukaz_1146_government_resignation_accepted_19960809',
             'ru_rg_constitution_19931225', 'ru_rg_constitution_published_19931225', 'statement_date', 'attested_period')
LEAD_URL_MARKERS = ('catalog/persons/6/biography', 'rg.ru:80/1993/12/25/konstituciya', 'rg.ru/1993/12/25/konstituciya',
                    'cntd.ru', '1996-2-Svodnaya_CIK.xls', 'post_850_pr_1996', 'vybor_99/2000_4.htm', '2004/03/24/cik.html',
                    'news/37383', 'news/37410', 'transcripts/22280', 'news/38099', 'news/30885', 'transcripts/22452',
                    'acts/bank/14857', 'news/copy/73692', 'news/15221', 'news/57420', 'news/73983', 'news/57091',
                    'prlib.ru/search', 'text_pr.htm', '_1/doc_6_1.htm', 'wikipedia')
# URL fragments that mark a page or file generated per request, or an unresolved capture; never allowed.
VOLATILE_URL = re.compile(r'(ysclid=|sessid=|PHPSESSID|[?&]cb=|nocache|token=|utm_|fbclid|yclid|form_build_id|/web/\d{4}id_/|/web/\d{14}/)')
REPORT = research.RESEARCH / 'russia-presidents-1991-2026-14.md'
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-14.md'


def presidents_invariants(russia, ussr):
    """Packet-level rules this test owns; raises AssertionError, KeyError or IndexError on any violation."""
    claims = {c['id']: c for s in russia['sources'] for c in s['claims']}
    owner = {c['id']: s['id'] for s in russia['sources'] for c in s['claims']}
    presidency = next(e for e in russia['institutions'] if e['id'] == 'ru_rsfsr_presidency')
    roles = {r['id']: r for r in presidency['roles']}
    assert list(roles) == [RSFSR, VICE, PR], 'exactly the two C01-05 roles and ru_president'
    heads = [r['id'] for e in russia['organizations'] + russia['institutions'] for r in e['roles'] if r['kind'] == 'head_of_state']
    assert heads == [PR], 'no other head-of-state role'
    assert [e['id'] for e in russia['institutions'] if not e['id'].startswith('ru_duma_faction_')] == ['ru_rsfsr_presidency']
    # The C01-05 holders are unchanged: one start each, no end.
    for role_id, name in ((RSFSR, YELTSIN), (VICE, 'Александр Владимирович Руцкой')):
        got = [(h['name'], h['attested_on'], h['from'], h['until']) for h in roles[role_id]['holder_claims']]
        assert got == [(name, None, '1991-07-10', None)], (role_id, got)
    holders = roles[PR]['holder_claims']
    assert all(isinstance(h, dict) for h in holders)
    got = [(h['name'], h['attested_on'], h['from'], h['until']) for h in holders]
    assert got == HOLDERS, got
    assert [h['claim_ids'] for h in holders] == HOLDER_CLAIMS
    for h in holders:
        assert not set(h['claim_ids']) & set(NEVER_HOLDER), h['name']
        assert not {h['attested_on'], h['from'], h['until']} & NEVER_HOLDER_DATE, h['name']
        assert 'Исполняющий' not in h['name'] and 'Acting' not in h['name'], h['name']
        assert all(cid in roles[PR]['claim_ids'] for cid in h['claim_ids']), h['name']
        expected = []
        for cid in h['claim_ids']:
            if owner[cid] not in expected:
                expected.append(owner[cid])
        assert h['sources'] == expected, h['name']
        # A start rests on a stated assumption of office cited by the holder, dated that day; an end on a stated cessation.
        if h['from']:
            assert STARTS[h['from']] in h['claim_ids'] and claims[STARTS[h['from']]]['attested_on'] == h['from'], h['name']
        if h['until']:
            assert ENDS[h['until']] in h['claim_ids'] and claims[ENDS[h['until']]]['attested_on'] == h['until'], h['name']
    assert [h['from'] for h in holders if h['from']] == list(STARTS)
    assert [h['until'] for h in holders if h['until']] == list(ENDS)
    # No holder merge or cross-role citation across the RSFSR and Russian Federation titles.
    for role_id in (RSFSR, VICE):
        assert not set(roles[role_id]['claim_ids']) & set(roles[PR]['claim_ids']), role_id
        assert not set(roles[role_id]['sources']) & set(roles[PR]['sources']), role_id
        for h in roles[role_id]['holder_claims']:
            assert not set(h['claim_ids']) & set(EVENTS), role_id
    assert not [h for h in holders if h['from'] and h['from'] < '1996-01-01'], 'no ru_president holder for 1991-1996'
    # Acting service and continuation stay claims on the role.
    for cid in ACTING + RETROSPECTIVE + OUTGOING:
        assert cid in roles[PR]['claim_ids'], cid
    for cid in UNDATED:
        assert 'attested_on' not in claims[cid] and 'period' not in claims[cid] and 'attested_period' not in claims[cid], cid
    # No other role in either packet cites a claim of this packet (no cross-institution holder), and USSR roles are untouched.
    for packet in (russia, ussr):
        for group in ('organizations', 'institutions'):
            for entry in packet[group]:
                for role in entry['roles']:
                    if role['id'] == PR:
                        continue
                    cited = set(role['claim_ids']) | {c for h in role['holder_claims'] if isinstance(h, dict) for c in h['claim_ids']}
                    assert not cited & set(EVENTS), role['id']
    su = next(r for e in ussr['institutions'] for r in e['roles'] if r['id'] == 'su_president')
    assert [h['name'] for h in su['holder_claims']] == ['Mikhail Gorbachev', 'Mikhail Gorbachev']
    # Distinct dated events stay distinct in each election cycle.
    for year, (declaration, oath, assumption) in {
            '1996': ('ru_ks_134o_cec_declared_second_term_19960709', 'ru_ks_134o_oath_of_office_19960809', 'ru_ks_134o_assumption_of_office_19960809'),
            '2000': ('ru_rg_cec_98_1110_3_putin_elected_20000405', 'ru_kremlin_20000507_oath', 'ru_kremlin_20000507_putin_assumed_office'),
            '2008': ('ru_cec_104_777_5_medvedev_elected_20080307', 'ru_kremlin_steno_medvedev_oath_20080507', 'ru_kremlin_steno_zorkin_medvedev_took_office_20080507'),
            '2024': ('ru_cec_163_1291_8_putin_elected_20240321', 'ru_kremlin_putin_oath_20240507', 'ru_kremlin_putin_took_office_20240507')}.items():
        assert claims[declaration]['attested_on'] < claims[oath]['attested_on'] == claims[assumption]['attested_on'], year
    assert claims['ru_ukaz_1761_yeltsin_ceases_powers_19991231']['attested_on'] == '1999-12-31'
    assert claims['ru_kremlin_putin_styled_acting_president_20000506']['attested_on'] == '2000-05-06'


class RussianPresidentsTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.raw = (research.ROOT / research.RESEARCH / 'russia.json').read_text(encoding='utf-8')
        cls.ussr_raw = (research.ROOT / research.RESEARCH / 'ussr.json').read_text(encoding='utf-8')
        cls.packet, cls.ussr = json.loads(cls.raw), json.loads(cls.ussr_raw)
        cls.sources = {s['id']: s for s in cls.packet['sources']}
        cls.claims = {c['id']: c for s in cls.packet['sources'] for c in s['claims']}
        cls.claim_source = {c['id']: s['id'] for s in cls.packet['sources'] for c in s['claims']}
        cls.presidency = next(e for e in cls.packet['institutions'] if e['id'] == 'ru_rsfsr_presidency')
        cls.role = next(r for r in cls.presidency['roles'] if r['id'] == PR)
        cls.extracts = {sid: json.loads((research.ROOT / cls.sources[sid]['snapshot']['path']).read_text(encoding='utf-8'))
                        for sid in NEW_SOURCES}
        cls.rows = {row['claim_id']: row for sid in NEW_SOURCES for row in cls.extracts[sid]['rows']}
        cls.new_claims = [c['id'] for sid in NEW_SOURCES for c in cls.sources[sid]['claims']]
        cls.report = (research.ROOT / REPORT).read_text(encoding='utf-8')

    def validate(self, packet=None):
        return research.validate(packet or self.packet, research.ROOT, {'Russia'}, {'Russia': set()})

    def section(self, heading):
        parts = self.report.split(f'\n## {heading}', 1)
        self.assertEqual(len(parts), 2, heading)
        return parts[1].split('\n## ', 1)[0]

    def test_new_records_are_bounded_and_every_claim_is_classified(self):
        ids = self.validate()
        self.assertEqual((len(NEW_SOURCES), len(self.new_claims)), (53, 94))
        self.assertEqual([s['id'] for s in self.packet['sources'][15:]], NEW_SOURCES)
        self.assertEqual((len(ids['entries']), len(ids['roles'])), (20, 8))
        self.assertEqual(set(self.new_claims), set(EVENTS))
        holder_claims = {cid for ids_ in HOLDER_CLAIMS for cid in ids_}
        self.assertEqual(len(NEVER_HOLDER), len(set(NEVER_HOLDER)))
        self.assertFalse(holder_claims & set(NEVER_HOLDER))
        self.assertEqual(holder_claims | set(NEVER_HOLDER), set(self.new_claims))
        # Every new source and claim is cited by ru_president, in packet order, and by no other role.
        self.assertEqual(self.role['sources'], NEW_SOURCES)
        self.assertEqual(self.role['claim_ids'], self.new_claims)
        self.assertEqual((self.role['title'], self.role['kind']), (PRESIDENT_TITLE, 'head_of_state'))
        observations = re.findall(r'^### (RU-PRES-\d\d)\b', self.report, re.M)
        self.assertEqual(observations, [f'RU-PRES-{n:02d}' for n in range(1, 11)])
        self.assertEqual({row['review_observation'] for row in self.rows.values()}, {f'RU-PRES-{n:02d}' for n in range(1, 11)})
        for stale in STALE_IDS:
            self.assertNotIn(stale, self.raw)

    def test_holders_are_exactly_as_intended(self):
        presidents_invariants(self.packet, self.ussr)
        for holder in self.role['holder_claims']:
            self.assertTrue(holder['note'] and holder['uncertainty'], holder['name'])
            for cid in holder['claim_ids']:
                row = self.rows[cid]
                self.assertEqual((row['holder_name'], row['role_id'], row['role_title']), (holder['name'], PR, PRESIDENT_TITLE), cid)
                self.assertIn(row['event_kind'], HOLDER_KINDS, cid)
        # No event kind that dates a holder appears on a claim that never feeds one, except unused attestations.
        for cid in NEVER_HOLDER:
            self.assertNotIn(self.rows[cid]['event_kind'], {'assumption_of_office', 'resignation_effective', 'oath_of_office'}, cid)
        for cid in UNUSED_ATTESTATIONS:
            self.assertIn(self.rows[cid]['event_kind'], {'in_office_signature', 'in_office_attestation'}, cid)
            self.assertIn('not', self.claims[cid]['uncertainty'])
        for cid in ACTING:
            self.assertEqual(self.rows[cid]['role_title'], ACTING_TITLE, cid)
            self.assertIn('never a holder', self.claims[cid]['uncertainty'], cid)
        for cid in set(self.new_claims) - set(ACTING):
            self.assertEqual(self.rows[cid]['role_title'], PRESIDENT_TITLE, cid)
        # Holder names follow the packet convention; printed forms are kept beside them.
        for row in self.rows.values():
            self.assertIn(row['holder_name'], {YELTSIN, PUTIN, MEDVEDEV, None}, row['claim_id'])
            self.assertEqual('printed_name' in row, row['holder_name'] is not None, row['claim_id'])
        self.assertEqual(self.rows['ru_ukaz_1761_yeltsin_ceases_powers_19991231']['printed_name'], 'Б.Ельцин')
        self.assertEqual(self.rows['ru_decree_636_putin_signs_as_president_20260904']['printed_name'], 'В.Путин')

    def test_starts_and_ends_only_where_a_source_states_one(self):
        claims = self.claims
        self.assertIn('таким образом, вступил в должность на второй срок подряд', claims['ru_ks_134o_assumption_of_office_19960809']['text'])
        self.assertIn('принес присягу народу', claims['ru_ks_134o_oath_of_office_19960809']['text'])
        self.assertIn('ceases to exercise the powers of the President of the Russian Federation from 12:00 on 31 December 1999',
                      claims['ru_ukaz_1761_yeltsin_ceases_powers_19991231']['text'])
        for cid in ('ru_kremlin_20000507_putin_assumed_office', 'ru_kremlin_20040507_putin_assumed_office',
                    'ru_kremlin_steno_zorkin_medvedev_took_office_20080507', 'ru_kremlin_zorkin_putin_took_office_20120507'):
            self.assertRegex(claims[cid]['text'], r'(has assumed the office|has taken office)', cid)
        for cid in ('ru_kremlin_putin_took_office_20180507', 'ru_kremlin_putin_took_office_20240507'):
            self.assertIn("announced Putin's assumption of office", claims[cid]['text'], cid)
        # Oaths date nothing on their own; each says it is not the start.
        for cid in (c for c, v in EVENTS.items() if v[1] == 'oath_of_office'):
            self.assertIn('not used as', claims[cid]['uncertainty'], cid)
        # The retrospective 1996 start carries its fallback; the pre-oath farewells are claims, not ends.
        yeltsin = self.role['holder_claims'][0]
        self.assertIn('fallback is attested_on 1996-08-09', yeltsin['uncertainty'])
        self.assertIn('Never merged with the C01-05 ru_rsfsr_president holder', yeltsin['uncertainty'])
        for cid in OUTGOING:
            self.assertIn('before', claims[cid]['uncertainty'], cid)
            self.assertIn('does not state that his office had ended', claims[cid]['uncertainty'], cid)
        self.assertIsNone(self.role['holder_claims'][2]['until'])
        self.assertIsNone(self.role['holder_claims'][3]['until'])
        self.assertIn('not used as this end', self.role['holder_claims'][3]['uncertainty'])
        # Acting service has a sourced start and a dated attestation, and no stated end.
        self.assertIn('no source states when it ended', self.role['holder_claims'][1]['uncertainty'])
        self.assertIn('not inferred from the 7 May 2000 assumption', claims['ru_ukaz_1762_putin_begins_acting_19991231']['uncertainty'])
        # Constitution and statute texts are procedure only; retrospective statements are never boundaries.
        for cid in PROCEDURE:
            self.assertIn(self.rows[cid]['event_kind'], {'procedure', 'entry_into_force_rule', 'publication_citation'}, cid)
        self.assertIn('never used to infer an end', claims['ru_const1993_oath_and_term_rules']['uncertainty'])
        self.assertIn('never a holder or a start', claims['ru_ks_134o_continuation_new_constitution_19931225']['uncertainty'])
        self.assertIn('с 1994 года', claims['ru_ks_134o_continuation_new_constitution_19931225']['uncertainty'])
        self.assertIn('never a boundary', claims['ru_kremlin_20040507_narration_vote_20040314']['uncertainty'])
        # The role and coverage say what remains open.
        self.assertIn('never holders or boundaries', self.role['scope_note'])
        self.assertIn('procedure only, never a date', self.role['scope_note'])
        self.assertIn('no ru_president holder is recorded for 1991-1996', self.role['scope_note'])
        unresolved = self.presidency['coverage']['unresolved']
        self.assertEqual([u.split(':')[0].split(' (')[0] for u in unresolved[-7:]],
                         ['RU-PRES-01', 'RU-PRES-02', 'RU-PRES-03', 'RU-PRES-04', 'RU-PRES-05', 'RU-PRES-05 to 10', 'RU-PRES-07 to 10'])
        self.assertEqual(sum('CLAUDE-C01-14' in u for u in self.packet['coverage']['unresolved']), 1)

    def test_extracts_match_packet_claims_and_record_original_responses(self):
        for sid in NEW_SOURCES:
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual(extract['format'], 'spheres-c01-derived-factual-table/v1')
            self.assertEqual((extract['source_id'], extract['source_url']), (sid, source['url']))
            self.assertEqual(extract['scope_note'], source['scope_note'])
            self.assertEqual((extract['access_method'], extract['published_date'], extract['document_date']),
                             (source['access_method'], source['published_date'], source['document_date']))
            self.assertEqual((extract['accessed_date'], source['accessed_date']), ('2026-09-24', '2026-09-24'))
            self.assertFalse(extract['source_response_checked_in'])
            self.assertEqual((extract['source_response_bytes'], extract['source_response_sha256']), RESPONSES[sid], sid)
            self.assertIn('not checked into this repository', extract['provenance_note'])
            self.assertIn('derived factual extract', extract['provenance_note'])
            self.assertIn('This packet: re-downloaded at 2026-09-25T', extract['stability_check'])
            self.assertIn('No portrait permission or likeness approval', extract['rights_note'])
            self.assertTrue(source['source_type'].startswith('primary_'), sid)
            snapshot = source['snapshot']
            self.assertEqual(snapshot['kind'], 'derived_factual_extract')
            self.assertRegex(snapshot['path'], r'^docs/campaign-certification/C01/research/sources/russia-[a-z0-9-]+-\d{8}-facts\.json$')
            data = (research.ROOT / snapshot['path']).read_bytes()
            self.assertEqual((len(data), hashlib.sha256(data).hexdigest()), (snapshot['bytes'], snapshot['sha256']))
            self.assertNotEqual(snapshot['sha256'], extract['source_response_sha256'])
            self.assertTrue(data.endswith(b'}\n') and b'\r' not in data)
            self.assertEqual(data.decode('utf-8'), json.dumps(extract, indent=2, ensure_ascii=False) + '\n')
            got = [(k, r['bytes'], r['sha256']) for k in ('portal_card_response', 'card_response', 'index_response')
                   if k in extract for r in [extract[k]]]
            got += [(k, r['bytes'], r['sha256']) for k in ('annex_responses', 'facsimile_responses', 'corroborating_responses')
                    for r in extract.get(k, [])]
            self.assertEqual(got, ATTACHED.get(sid, []), sid)
            for alt in extract.get('alternate_responses', []):
                self.assertIn('never the recorded identity', alt['note'])
                self.assertNotIn('web.archive.org', alt['url'])
            # Rows repeat the packet claims exactly, in order, keyed by claim_id; no row has a bare 'name' key.
            self.assertEqual([r['claim_id'] for r in extract['rows']], [c['id'] for c in source['claims']])
            for row, claim in zip(extract['rows'], source['claims']):
                self.assertEqual((row['text'], row['locator'], row['attested_on']), (claim['text'], claim['locator'], claim.get('attested_on')))
                self.assertEqual((row['observation_id'], row['role_id']), ('ru_rsfsr_presidency', PR))
                self.assertNotIn('name', row)
        self.assertEqual({cid: (row['attested_on'], row['event_kind'], row['review_observation']) for cid, row in self.rows.items()}, EVENTS)
        for sid, stamp in ARCHIVED.items():
            source, extract = self.sources[sid], self.extracts[sid]
            url = urlsplit(source['url'])
            self.assertEqual(url.hostname, 'web.archive.org')
            self.assertTrue(url.path.startswith(f'/web/{stamp}id_/http'), sid)
            self.assertLess(stamp, '20260907')
            self.assertEqual(extract['source_response_url'], source['url'])
            self.assertEqual(extract['original_url'], source['original_url'])
            self.assertEqual(source['url'].split('id_/', 1)[1].replace(':80/', '/', 1), source['original_url'])
            self.assertEqual(extract['archive_capture_utc'].replace('-', '').replace(':', '').replace('T', '').rstrip('Z'), stamp)
            self.assertIn('Raw Internet Archive capture', extract['provenance_note'])
            self.assertEqual(source['access_method'], 'internet_archive_raw_capture')
        for sid, http in HTTP_READ.items():
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual(source['url'], http.replace('http://', 'https://', 1))
            self.assertEqual(extract['source_response_url'], http)
            self.assertIn('port 443', extract['provenance_note'])
            self.assertNotIn('original_url', source)
        self.assertEqual(set(ARCHIVED) | set(HTTP_READ), set(NEW_SOURCES))
        self.assertFalse(set(ARCHIVED) & set(HTTP_READ))
        # The kremlin.ru original URLs are the exact captured pages (www and text-version paths kept).
        self.assertEqual(self.sources['ru_kremlin_news_57416_20180507']['original_url'], 'http://www.kremlin.ru/events/president/news/copy/57416')
        self.assertEqual(self.sources['ru_kremlin_news_15224_20120507']['original_url'], 'http://kremlin.ru/events/president/news/copy/15224')
        self.assertEqual(self.sources['ru_kremlin_news_acting_president_20000506']['original_url'], 'http://www.kremlin.ru/events/president/news/38128')
        for sid, decoded in GZIP_DECODED.items():
            self.assertEqual((self.extracts[sid]['source_response_decoded']['bytes'], self.extracts[sid]['source_response_decoded']['sha256']), decoded)
        # Facsimiles and PDF pages were viewed and are recorded.
        self.assertEqual(self.extracts['ru_pub_decree_636_20260904']['visual_review']['pdf_pages_one_based'], [1])
        self.assertEqual(self.extracts['ru_rg_cec_res_163_1291_8_20240321']['visual_review']['facsimile_pages_one_based'], [1, 2, 3])
        for sid in ('ru_fc_res_442sf_20111125', 'ru_fc_res_528sf_20171215', 'ru_fc_res_678sf_20231207'):
            self.assertEqual(self.extracts[sid]['visual_review']['facsimile_pages_one_based'], [1], sid)
            self.assertNotIn('no facsimile', self.extracts[sid]['visual_review']['method'], sid)
        self.assertIsNone(self.sources['ru_kremlin_inauguration_broadcast_20040507']['published_date'])
        self.assertIsNone(self.sources['ru_rg_cec_res_98_1110_3_20000405']['published_date'])

    def test_secondary_leads_and_volatile_urls_stay_out_of_the_packet(self):
        urls = []
        for sid in NEW_SOURCES:
            extract = self.extracts[sid]
            urls += [self.sources[sid]['url'], extract['source_response_url']]
            for key in ('portal_card_response', 'card_response', 'index_response'):
                if key in extract:
                    urls.append(extract[key]['url'])
            for key in ('annex_responses', 'facsimile_responses', 'corroborating_responses', 'alternate_responses'):
                urls += [r['url'] for r in extract.get(key, [])]
        for url in urls:
            self.assertIsNone(VOLATILE_URL.search(url), url)
            for marker in LEAD_URL_MARKERS:
                self.assertNotIn(marker, url)
        lowered = self.raw.lower()
        for marker in LEAD_URL_MARKERS:
            self.assertNotIn(marker.lower(), lowered, marker)
        leads, added = self.section('Leads not imported'), self.section('Sources added')
        for marker in ('catalog/persons/6/biography', '79f5320a', 'rg.ru/1993/12/25/konstituciya', 'cntd.ru', '1996-2-Svodnaya_CIK.xls',
                       'vybor_99/2000_4.htm', 'news/37383', 'transcripts/22452', 'news/57420'):
            self.assertIn(marker, leads, marker)
            self.assertNotIn(marker, added, marker)

    def test_separation_from_the_c01_05_roles_and_the_ussr_packet(self):
        # No new source or claim is cited outside ru_president, and the USSR packet does not mention any of them.
        for sid in NEW_SOURCES:
            self.assertNotIn(f'"{sid}"', self.ussr_raw)
            self.assertTrue(sid.startswith('ru_'))
        for cid in self.new_claims:
            self.assertNotIn(f'"{cid}"', self.ussr_raw)
            self.assertNotIn(cid, self.presidency['claim_ids'])
        self.assertEqual(len(self.presidency['sources']), 8)
        self.assertEqual(len(self.presidency['claim_ids']), 10)
        for role_id in (RSFSR, VICE):
            role = next(r for r in self.presidency['roles'] if r['id'] == role_id)
            self.assertEqual(role['kind'], 'institutional_office')
        self.assertEqual((self.presidency['lifecycle']['from'], self.presidency['lifecycle']['until']), (None, None))
        self.assertIn('Stated in the determination of 5 November 1998', self.claims['ru_ks_134o_rsfsr_president_retitled_19981105']['uncertainty'])
        self.assertIn('does not merge ru_rsfsr_president and ru_president holders', self.claims['ru_ks_134o_rsfsr_president_retitled_19981105']['uncertainty'])
        self.assertIn('no ru_president holder is recorded for 1991-1996', self.claims['ru_ks_yeltsin_styled_president_rf_19930323']['uncertainty'])
        self.assertIn('does not touch the C01-05 ru_rsfsr_president holder', self.claims['ru_ukaz_1761_yeltsin_ceases_powers_19991231']['uncertainty'])
        self.assertIn('not a tie between ru_rsfsr_president and ru_president', self.claims['ru_kremlin_20000507_putin_assumed_office']['uncertainty'])

    def test_packet_formatting_is_preserved(self):
        data = (research.ROOT / research.RESEARCH / 'russia.json').read_bytes()
        self.assertNotIn(b'\r', data)
        self.assertEqual(data.decode('utf-8'), json.dumps(self.packet, indent=2, ensure_ascii=False) + '\n')

    def test_mutations_are_rejected(self):
        def mutated(change, packet=None):
            packet = copy.deepcopy(packet or self.packet)
            change(packet)
            return packet

        def source(packet, sid):
            return next(s for s in packet['sources'] if s['id'] == sid)

        def claim(packet, cid):
            return next(c for s in packet['sources'] for c in s['claims'] if c['id'] == cid)

        def role(packet, role_id=PR):
            return next(r for e in packet['institutions'] if e['id'] == 'ru_rsfsr_presidency' for r in e['roles'] if r['id'] == role_id)

        def holder(packet, index, role_id=PR):
            return role(packet, role_id)['holder_claims'][index]

        validator_cases = [
            (lambda p: source(p, 'ru_ks_det_134o_19981105')['snapshot'].update(sha256='0' * 64), 'checksum mismatch'),
            (lambda p: source(p, 'ru_pub_decree_636_20260904')['snapshot'].update(bytes=1), 'checksum mismatch'),
            (lambda p: holder(p, 6).update(until='2026-09-08'), 'exceeds cutoff'),
            (lambda p: claim(p, 'ru_decree_636_putin_signs_as_president_20260904').update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: holder(p, 0).update(until='1996-08-08'), 'Reversed historical interval'),
            (lambda p: holder(p, 0, RSFSR)['claim_ids'].append('ru_ks_134o_assumption_of_office_19960809'), 'cited source'),
            (lambda p: source(p, 'ru_fc_res_678sf_20231207').update(url=HTTP_READ['ru_fc_res_678sf_20231207']), 'Invalid public source URL'),
        ]
        for change, message in validator_cases:
            with self.subTest(message=message), self.assertRaisesRegex(ValueError, message):
                self.validate(mutated(change))
        acting_holder = {'name': PUTIN, 'attested_on': '1999-12-31', 'from': None, 'until': None,
                         'sources': ['ru_pravo_ukaz_1762_19991231'], 'claim_ids': ['ru_ukaz_1762_putin_begins_acting_19991231']}
        invariant_cases = [
            ('successor start used as an end (Medvedev)', lambda p: holder(p, 3).update(until='2012-05-07')),
            ('successor start used as an end (Putin 2004)', lambda p: holder(p, 2).update(until='2008-05-07')),
            ('re-election used as an end (Putin 2000)', lambda p: holder(p, 1).update(until='2004-05-07')),
            ('outgoing statement cited as an end', lambda p: (holder(p, 3).update(until='2012-05-07'),
                                                             holder(p, 3)['claim_ids'].append('ru_kremlin_medvedev_concluding_presidency_20120507'))),
            ('election date used as start (Putin 2000)', lambda p: holder(p, 1).update({'from': '2000-03-26'})),
            ('declaration date used as start (Putin 2000)', lambda p: holder(p, 1).update({'from': '2000-04-05'})),
            ('declaration date used as start (Medvedev)', lambda p: holder(p, 3).update({'from': '2008-03-07'})),
            ('declaration date used as start (Yeltsin 1996)', lambda p: holder(p, 0).update({'from': '1996-07-09'})),
            ('designation date used as start (2024)', lambda p: holder(p, 6).update({'from': '2023-12-07'})),
            ('election date used as observation', lambda p: holder(p, 5).update({'from': None, 'attested_on': '2018-03-18'})),
            ('latest attestation used as an end', lambda p: holder(p, 6).update(until='2026-09-04')),
            ('acting service added as a holder', lambda p: role(p)['holder_claims'].insert(1, copy.deepcopy(acting_holder))),
            ('acting claim cited by a holder', lambda p: holder(p, 1)['claim_ids'].append('ru_kremlin_putin_styled_acting_president_20000506')),
            ('continuation added as a holder', lambda p: role(p)['holder_claims'].insert(0, {
                'name': YELTSIN, 'attested_on': '1993-12-25', 'from': None, 'until': None,
                'sources': ['ru_ks_det_134o_19981105'], 'claim_ids': ['ru_ks_134o_continuation_new_constitution_19931225']})),
            ('ceremony claim cited by a holder', lambda p: holder(p, 1)['claim_ids'].append('ru_kremlin_news_20000507_inauguration_noon')),
            ('election claim cited by a holder', lambda p: holder(p, 3)['claim_ids'].append('ru_cec_104_777_5_medvedev_elected_20080307')),
            ('cross-role holder (C01-05 Yeltsin on ru_president)', lambda p: role(p)['holder_claims'].insert(
                0, copy.deepcopy(role(p, RSFSR)['holder_claims'][0]))),
            ('cross-role holder (ru_president holder on the RSFSR role)', lambda p: role(p, RSFSR)['holder_claims'].append(
                copy.deepcopy(holder(p, 0)))),
            ('1999 end moved onto the C01-05 holder', lambda p: holder(p, 0, RSFSR).update(until='1999-12-31')),
            ('cross-role claim', lambda p: role(p, RSFSR)['claim_ids'].append('ru_ks_134o_rsfsr_president_retitled_19981105')),
            ('undated claim given a date', lambda p: claim(p, 'ru_ks_134o_rsfsr_president_retitled_19981105').update(attested_on='1991-12-25')),
            ('oath collapsed into declaration', lambda p: claim(p, 'ru_kremlin_putin_oath_20240507').update(attested_on='2024-03-21')),
            ('second head-of-state role', lambda p: role(p, VICE).update(kind='head_of_state')),
            ('role removed', lambda p: next(e for e in p['institutions'] if e['id'] == 'ru_rsfsr_presidency')['roles'].pop()),
        ]
        presidents_invariants(self.packet, self.ussr)
        for label, change in invariant_cases:
            with self.subTest(label=label), self.assertRaises((AssertionError, KeyError, IndexError)):
                presidents_invariants(mutated(change), self.ussr)
        # A cross-institution holder: a ru_president observation added to the USSR Presidency.
        ussr = copy.deepcopy(self.ussr)
        next(r for e in ussr['institutions'] for r in e['roles'] if r['id'] == 'su_president')['holder_claims'].append(
            copy.deepcopy(self.role['holder_claims'][1]))
        with self.assertRaises(AssertionError):
            presidents_invariants(self.packet, ussr)

    def test_report_and_handoff_are_ready_for_review_and_close_nothing(self):
        self.assertIn('ready_for_review', self.report)
        table = self.section('Outcome')
        decisions = {'01': 'Accepted', '02': 'Accepted', '03': 'Accepted in part', '04': 'Accepted', '05': 'Accepted',
                     '06': 'Accepted', '07': 'Accepted', '08': 'Accepted', '09': 'Accepted', '10': 'Accepted'}
        for number, decision in decisions.items():
            row, = [line for line in table.splitlines() if line.startswith(f'| RU-PRES-{number} ')]
            self.assertIn(f'**{decision}:**', row)
        defects = self.section('Checker defects')
        rows = [line for line in defects.splitlines() if re.match(r'\| [ABC]\d+ ', line)]
        self.assertEqual([re.match(r'\| ([ABC]\d+) ', r).group(1) for r in rows],
                         [f'A{n}' for n in range(1, 14)] + [f'B{n}' for n in range(1, 15)] + [f'C{n}' for n in range(1, 10)])
        for row in rows:
            self.assertRegex(row, r'\*\*(Applied|Applied in part|Resolved by removal)\*\*')
        for marker in ('C01', 'C06', 'S23', 'WC1', 'CP1'):
            self.assertNotRegex(self.report, rf'\b{marker}\b[^.\n]*\bis (now )?complete\b')
        notes = self.section('Integration notes')
        for text in ('claude/c01-ussr-05', '1c698ed0', 'research-index.json', 'test_russia_research_s10h.py',
                     'test_ussr_russia_transition_c01_05.py', 'test_campaign_census'):
            self.assertIn(text, notes)
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        for text in ('ready_for_review', 'russia-presidents-1991-2026-14.md', 'claude/c01-ru-14', '9673a99f', 'claude/c01-ussr-05',
                     'test_russia_presidents_c01_14.py', 'pending Codex acceptance'):
            self.assertIn(text, handoff)
        index = research.build()
        country = next(p for p in index['countries'] if p['nation'] == 'Russia')
        self.assertFalse(country['country_census_complete'])
        self.assertIsNone(country['unrepresented_organization_count'])
        self.assertEqual((country['role_observations'], country['source_claims']), (8, 142))
        self.assertEqual({w['status'] for w in index['work_orders'] if w['nation'] == 'Russia'}, {'open'})
        self.assertFalse(index['c01_complete'])


if __name__ == '__main__':
    unittest.main()
