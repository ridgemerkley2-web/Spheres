"""CLAUDE-C01-19: Russian heads of government, 1991-2026, keep the President's nomination, its presentation at a sitting,
the parliament's consent, approval or refusal and the vote, the appointment decree, the Government's resignation, dismissal
or laying down of powers, continued duties and acting service apart, and state a start or an end only where a source does.
The presidency holders of CLAUDE-C01-05 and C01-14 and the USSR packet are unchanged."""
import copy
import hashlib
import json
import re
import unittest
from urllib.parse import parse_qs, urlsplit

import campaign_research as research


# Original response identity recorded in each new extract: (bytes, sha256), re-downloaded byte-identical by the dossier or
# the independent check that found the record, the check, and twice by this packet at least 30 minutes apart.
RESPONSES = {
    'ru_rsfsr_cpd_res_1830i_19911101': (25524, 'a65284a5f493a95a0fcd93ed7b9c5d1faef7d888ec19fb684da4d6106d734c8e'),
    'ru_rsfsr_ukaz_171_19911106': (20304, '9dcc2f762ed44d7d28fb75ef4da94915cff8b82932fee874fbfedfddc3b63b83'),
    'ru_rsfsr_ukaz_172_19911106': (36493, 'e748a13a873241c870e8026f8bdb70794487aea5cc921f67ad50b04e36e81fca'),
    'ru_rsfsr_gov_res_8_19911115': (8523, 'b8c9282efabbf114a54fe28c7547738e25b171f0e1d66fdfb2b2d831b174a31e'),
    'ru_ukaz_633_19920615': (23687, '3331989ed5dd8a66d3bef674174d2267bcb7a01bf3c1cc44ba6195ac49c569ec'),
    'ru_gov_res_457_19920701': (11718, '6e6f7ef11b628d4f382d277f05b351370c24e19acde37c288fe03722bbd6183e'),
    'ru_cpd_res_4063i_19921209': (7516, 'b2ca2b01dc1df82c002881ee49fd6c44a3d7fa1501bfa644649477b256a626a1'),
    'ru_cpd_res_4079i_19921212': (11520, '6cca2d3e4eb2ccee6b7f7b6dd4b50c2d727bc84d25d6201c06623504891d5151'),
    'ru_cpd_res_4088i_19921214': (7235, 'fb54c92021c1d1328eb925ebbbbf1f039473d0d30e89e5ebee1d9b22ccaf308a'),
    'ru_ukaz_1567_19921214': (23684, 'd33bfee761a86fd583c54eb32e96f512e4b0c1d6a751096ee973844950cb7b6d'),
    'ru_ukaz_1569_19921215': (24638, '12352d48ae48d90d931b0770a00b225b06bcc53942c179ec78113541121fce06'),
    'ru_ukaz_1570_19921215': (23571, '937cc54e07219b356335ad1b8d47f0b6ee6c002ca0b62896c90f0a8039bae424'),
    'ru_duma_steno_ia_19960810': (503153, 'c2bdcb97b64bcb93694ba8eb8b7fc4e437e1fd89bed0d2c09535c7898a448a31'),
    'ru_gd_res_624ii_19960810': (8180, '82293a19d66e2cb70d8e1c35c8c8dcaaef13b9ed424caea4aba80355a7ab580d'),
    'ru_ukaz_1152_19960810': (23652, '916f74835ed75c20ebc05e9d2ac97ff332a84508e59cf1c46b48d7115ae01db4'),
    'ru_ukaz_281_19980323': (23883, 'a3e71c3180b569aae119128df4e76775febd38ec1febf9f35b51aa21850e4254'),
    'ru_ukaz_287_19980323': (24048, '93a5ac887b5e3fda9704b85525a3a899c6d32b6ba7ede2e69699b868a31be57b'),
    'ru_ukaz_288_19980323': (23794, '71d0e6ef2df617c511bdb886909fcc43fe6c29623f3140003680c8a8acc37d13'),
    'ru_duma_steno_19980410': (351530, 'e2aaba021076bb77b3dc56797fc415f22142e660a4b106b0eb19cadd8c8bc896'),
    'ru_gd_res_2375ii_19980410': (8248, 'b68b61830744ed4f99c79fa6806858246aa2c5812858b44b7fe2704102dd7b17'),
    'ru_duma_steno_19980417': (500314, '817e75342de9fd1dc5a72134646e522dfdd7c64b2603cba59e6f4cc4f212519c'),
    'ru_gd_res_2402ii_19980417': (8273, 'c6f2ae813f96311982d28d300bd0d2d1c605cd0a5b2797668a5fb790d0f6b24c'),
    'ru_duma_steno_19980424': (392612, '8293a8c9673dc9c08c6d2ef67dc17bfda08e9f1f10966fec08995bd03435b778'),
    'ru_gd_res_2421ii_19980424': (28358, '459f55159070a9ecd54387d461b9d65fcfe1df21952a59387fca76441cbde7fc'),
    'ru_ukaz_436_19980424': (20085, '3ef6f580bcf77b2c7c0f49e5c8db673be273ecd2095dd983bf9b52e9d01779f1'),
    'ru_ukaz_983_19980823': (23827, '8ee3465c50a15c97aa2044897c93be96084dee10d92e6a4132242b6c2830f438'),
    'ru_ukaz_987_19980825': (20533, '8d49ad29ebe358a380afb931e6c9cc0ac4510d2712e06e8ee7de531674aeaff9'),
    'ru_duma_steno_19980831': (263526, '9b41c8f2eca60f91ce66d58d501b352b81042c6227e8503b10388dc4d202408f'),
    'ru_gd_res_2898ii_19980831': (8274, 'a846bf2a663a947a5ee085537303bedf841e7bb65b7a378f09fdcbeff34bd10b'),
    'ru_duma_steno_19980907': (227423, '2a7fdbb6e84c12ecc48621e8921fbba3a234dfe03347506820830823a61ff344'),
    'ru_gd_res_2928ii_19980907': (28360, '10da8b9f1ff1a431fc3a23cbd8560009ea4c5c01b88fe2f8fab41716752ef815'),
    'ru_duma_steno_19980911': (578566, '592cea0a85c14528c0f860ca6238dbcff4018612c3f91e4e408c293d5e140477'),
    'ru_gd_res_2961ii_19980911': (8293, 'eef235a0a889d98db66f67b48672e3d8368c44ced9b8c9f20eaadc2d1b4de262'),
    'ru_ukaz_1087_19980911': (23636, '88e46448f1148a24b2ba8b25e76b10f27c46e24b3990e3a48e267ec372d0bfa0'),
    'ru_ukaz_580_19990512': (24115, '0988a28cdc6c43bb734a3ccc45a9454460a95a06b5ae5f7213f87f6a47af3568'),
    'ru_duma_3961ii_19990512': (31216, '85bb8bf43ef224bd41487c9c4217ca9bb903f3c00105dea7d3699543613eaca2'),
    'ru_gov_res_528_19990513': (31708, '01d20d17f8aa1a6324e6dd82ddd081cf4f898026eafd6c4cfe6dfc1e92f34923'),
    'ru_duma_3965ii_19990519': (28371, 'b7823540cebb372fa72e7ebde34dbcfbf20f90cfbfc10dcf8358b389add9a585'),
    'ru_ukaz_611_19990519': (23628, '00f03014c7ae3faf937fcaabe18a3e5e140382f9e54f110b54207445987de8d5'),
    'ru_gov_res_905_19990806': (31777, 'c74956a73c3db334468b4b2884eb60addad0f41316d82421043560820a5c9a87'),
    'ru_ukaz_1012_19990809': (20522, 'fd660c0da02ec525c18cf18091e4c29ad00971299c15ea05bb987792d66e14d4'),
    'ru_gov_res_923_19990810': (7808, '0a4d7e5ebe41b7cccf62f5f276646eb063687e98928a9ceba054a5c1ad82d635'),
    'ru_duma_4276ii_19990816': (8297, '83dcb5305857dab295513d1252009e2e7a647c1dcb57b1931738cb7d35e15809'),
    'ru_ukaz_1052_19990816': (23754, '71c634af6f1c8a75537a4912825c3b843a87329f7930bc04bda85169d14857a1'),
    'ru_gov_rasp_647r_20000507': (31940, '49e80cb0e65e843a90d7bdbde6b79b852d202de2b1bb1ebe6b041441619c840c'),
    'ru_ukaz_834_20000507': (23925, '80c52749cb15b753d0e720ba7c6f5fcb44d1d77fbe16358d171cdba9f7e34d26'),
    'ru_ukaz_836_20000507': (24049, '9a8bab103ef7d9f087dd1a1066d6561ba03709d73d8228224adeb8155ed04468'),
    'ru_kremlin_news_38132_20000510': (34798, 'e2f76cc3a4d4ddc2c8360b9cd07f9053d1e7c142db5e1ad494368abe36719136'),
    'ru_duma_363iii_20000517': (8290, '9b3b03c38cab5cbcaec5cae9b83117bef386468df4e3192fd82456bc5f9d91db'),
    'ru_ukaz_861_20000517': (23705, '3ae314dbcbc32d559364d64bcf667be51cd49aa98bc3d68d299562a70febda25'),
    'ru_kremlin_news_38171_20000517': (36843, 'a2ad81169fd4010aa369d5253a4f6030da00ff16d0cbd6356ceedfa810f8ce35'),
    'ru_ukaz_264_20040224': (20097, 'e946494562e4a7e686149d809095136b8a0dd57aa16cb31415325f25959d45d4'),
    'ru_kremlin_transcript_22362_20040224': (42078, '3a2873e205f0f51a8a4f11c69ac8791dcca06de6e70ceb2bcfc07a175135f2db'),
    'ru_kremlin_news_30424_20040224': (37301, '2384295f8ecb44840860d6d0e51efee7f6c8c12bb092de1941321601a41b61d2'),
    'ru_gov_res_104_20040225': (29317, 'e77a8a3f6b5646e2c0366759012b590048e94ea181bfc2550e88c268f8d53c29'),
    'ru_kremlin_news_45963_20040301': (41738, '8c8daeaa21d225241ff5b773bdaf0189ad98c01114bfc1070237b55531efc34b'),
    'ru_duma_162iv_20040305': (8231, '55c2202193ea222b0ac3a7f817cecd0a246e49941ea142ad2db6542f10ba7dce'),
    'ru_ukaz_300_20040305': (23703, '025dc0f08c133378e66b80dbe107a544a2d8d2ad422c7e8173d48beb89548a9a'),
    'ru_gov_rasp_608r_20040507': (27540, '6d58f37bc475c1628f027c47c6fcaaa219bc6068b908c4237b808bd244cd9608'),
    'ru_ukaz_585_20040507': (24307, '7607b52cb09641e94af7c48a58805572994610823293167719fa4a3f357c4060'),
    'ru_kremlin_news_30891_20040507': (36430, '22151444e1a16d07d7b475f05f98517eb71fe5398c68b96cdff44c36032fca37'),
    'ru_kremlin_news_30892_20040507': (38383, '9407866e8c7b32cb1e23184e7089c301ae7b2bb2625a230a54b4673e983392b4'),
    'ru_gov_res_232_20040508': (8076, '7a1e27d7f7f5e7b481c209ec74b652a27677ff8a568205800d2e30f27925ea0a'),
    'ru_gov_res_233_20040511': (32704, '1c78b3382ba07aa0c4bdc704011cfa37bb4fb022cf5e106c9dc86637c7888267'),
    'ru_duma_489iv_20040512': (8173, '27be0dc60b7c524270d3976496dab995f97d6da92749236aed8ea2da6d638aae'),
    'ru_ukaz_610_20040512': (23702, '0ed6d5a303f289479643ed747822cf275a2d153091c30c24e35f9ae50ca26b73'),
    'ru_ukaz_1184_20070912': (23958, '74f07e57489e28044408b52c379ec0edbc7e2bff6aeb41045a142fdfa37581f3'),
    'ru_kremlin_news_42294_20070912': (38808, '1160b90a0e03f5a0106d72ce6cca3ae72e117f77341c6472e77017db75711006'),
    'ru_kremlin_transcript_24530_20070912': (44045, '0ea1a722b7459453aedb43b915fea939e21328d24cc5fa9a9e77ca7fd062c976'),
    'ru_gov_res_586_20070913': (20673, '30a898ad744e16130562bf4447cde57589ce2a5f358edf761460750022e1c088'),
    'ru_duma_res_5066_4_gd_20070914': (32541, '5215f20d1d6c43532ecc62571eddb8c85119153a3c8b28f312d5266bd5f00110'),
    'ru_ukaz_1202_20070914': (23709, 'd26dd6460991595db544dd2beb1cc7082da195ee265784c427f3ee67905a9173'),
    'ru_kremlin_news_42319_20070914': (1600, '576dfbaa83c986d838c6d921c217b21506a20dbe36fe213f9fe86c0f9f643fae'),
    'ru_gov_rasp_680r_20080507': (21754, '65b22ef2308eb80fe90c8b8602162e19086b93da085e065ef07a4daae6874295'),
    'ru_ukaz_717_20080507': (24317, '56194f58fb2f246dffb478918c6d85e365019c0668fd3c55564ec9dc97769168'),
    'ru_kremlin_news_6_20080507': (36559, '9335792f63d341a926d42f7bddf8f1ceaae15daf3228348cc49ad1dd2d667962'),
    'ru_kremlin_news_20_20080508': (42709, 'd5daa1fe66b659aff50f6c1941c0c6b8bb7b9e45563118c22dd2d1d9507fdb8e'),
    'ru_duma_res_458_5_gd_20080508': (32519, '748ee3079cdd7abcfabdf193518e3fee1362088d8ec5f663e2a647476474cb0e'),
    'ru_ukaz_723_20080508': (19788, '3742103bc55b99bc9cb32d2f34b1b960291dc6210a9bfbe92826293854d35da0'),
    'ru_gov_rasp_760r_20120507': (19957, 'ae197235b3568d83c3d4b281ede3118b848e99a3ccba8d287495d6127f76e665'),
    'ru_pub_ukaz_607_20120507': (39220, 'ae7b8880173534635bc89565ff13d3f5e369a88e19b74b331e152feab13d4733'),
    'ru_kremlin_news_15230_20120507': (39222, '20963d1501d74066260f48ff0a8bc2a1f72bd59e8033b7ab3d3d001829411a88'),
    'ru_kremlin_news_15266_20120508': (67777, 'ec4a77624a4e556c38caff28328a7a43b1640b7fa528cea9fbab71625d2461ee'),
    'ru_pub_duma_res_323_6_gd_20120508': (48243, '85a8103b76a951afceb7baac23da2be2db647785e861c006f079884afa16f4a4'),
    'ru_pub_ukaz_612_20120508': (33114, '1f0d6bae1fcf89d01725799ddc17145c8835bb9fd7aba7a9c008ece65fe4f71b'),
    'ru_pub_gov_rasp_875r_20180507': (24640, 'd853a5a2ef00a3a2fa8e4ba673c910c43194f6e23a6c6e99b088bc8e38a98115'),
    'ru_pub_ukaz_202_20180507': (38932, 'd8457eada4d7fb14bb79fc1c04bd85ff66582f4d23e8f4f99b5a5a59977803b3'),
    'ru_kremlin_news_57422_20180507': (1704, 'a9134e2134117cc89e157be23872e835ececd53b02670abc37d7fc8ab964a5d6'),
    'ru_pub_duma_res_3894_7_gd_20180508': (222756, 'baf9f3e8c825a3614d34f9019fa6b55116ba6a2f73a811ab8046d1be477eedbe'),
    'ru_pub_ukaz_209_20180508': (31157, 'b44446814f36a9e184c3f2379313352753af5ce79385a0ab3f51808ff0dbdfd3'),
    'ru_kremlin_news_62585_20200115': (7497, 'f2fbced0450729c91878558eb165b561c6779c599c72b20aef3fad54099f5ae0'),
    'ru_pub_ukaz_14_20200115': (43942, '585d9eca08e5aec8e80db42479c27d292d8e038f64529e97baa4e610e0526b46'),
    'ru_kremlin_news_62586_20200115': (1642, '44e824a1d0cb6690d64cba6cdd41a01b3647afbb50d9135cda654e493073011e'),
    'ru_pub_duma_res_7565_7_gd_20200116': (39429, '09e5c355a38a403705a6a5a74fa6c0c745b5365a9dea210cebab7a1c6a781988'),
    'ru_pub_ukaz_17_20200116': (32565, 'a177fb9963fe57a8671aa3cfc2e666e56d75fbc90b3825293045c438c2255c6e'),
    'ru_pub_gov_rasp_1121r_20240507': (107660, '5254fddee0cf9f254f6a19ba982e94a9719b1f3fce05704bc8366cd2f1989393'),
    'ru_pub_ukaz_306_20240507': (41213, '17c1e119bd992de4e20efab1aff84841ffb0cdf06e19ab9e8c0afb00cb2f8891'),
    'ru_kremlin_news_74009_20240510': (5947, '16c34183f2cb9a9cb7b19592488ea8301844089bda14595913d1b936ffc423cd'),
    'ru_duma_steno_20240510': (261112, 'dfdff77f177e510a26a07d89535bd9f555de2dfb8644eb2d8fb5e57006f72e3e'),
    'ru_pub_duma_res_6061_8_gd_20240510': (35692, '7b4448a54958befacf3af32635001831840676bb3c74f46d578e096224b5f6cc'),
    'ru_pub_ukaz_319_20240510': (32384, '3747d35f8b9d404d6f0a1ef4744470c5425bd54e4f05dea49c7235b33367d64b'),
    'ru_pub_gov_res_1125_20260903': (184530, '972865e66b5b6c31111ee868f1f99bf654f727a50c7ebddcca937fb30bcc060d'),
}
ARCHIVED = {
    'ru_duma_steno_ia_19960810': '20240528152653',
    'ru_duma_steno_19980410': '20240727075703',
    'ru_duma_steno_19980417': '20240528152617',
    'ru_duma_steno_19980907': '20220620072458',
    'ru_duma_steno_19980911': '20220615161227',
    'ru_kremlin_news_38132_20000510': '20260513105913',
    'ru_kremlin_news_38171_20000517': '20250425232300',
    'ru_kremlin_transcript_22362_20040224': '20260615170001',
    'ru_kremlin_news_30424_20040224': '20260616154745',
    'ru_kremlin_news_45963_20040301': '20260410175830',
    'ru_kremlin_news_30891_20040507': '20250521055537',
    'ru_kremlin_news_30892_20040507': '20240510083530',
    'ru_kremlin_news_42294_20070912': '20251216052618',
    'ru_kremlin_transcript_24530_20070912': '20260610191047',
    'ru_kremlin_news_42319_20070914': '20251210060946',
    'ru_kremlin_news_6_20080507': '20260421233600',
    'ru_kremlin_news_20_20080508': '20260520144350',
    'ru_kremlin_news_15230_20120507': '20260520130324',
    'ru_kremlin_news_15266_20120508': '20260729233942',
    'ru_kremlin_news_57422_20180507': '20250829021035',
    'ru_kremlin_news_62585_20200115': '20210513233827',
    'ru_kremlin_news_62586_20200115': '20230731173211',
    'ru_kremlin_news_74009_20240510': '20240512211122',
    'ru_duma_steno_20240510': '20240520233218',
}
HTTP_READ = {
    'ru_rsfsr_cpd_res_1830i_19911101': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102012920&page=1&rdk=0',
    'ru_rsfsr_ukaz_171_19911106': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102012981&page=1&rdk=0',
    'ru_rsfsr_ukaz_172_19911106': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102012988&page=1&rdk=0',
    'ru_rsfsr_gov_res_8_19911115': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102013105&page=1&rdk=0',
    'ru_ukaz_633_19920615': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102016829&page=1&rdk=0',
    'ru_gov_res_457_19920701': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102017184&page=1&rdk=0',
    'ru_cpd_res_4063i_19921209': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102020211&page=1&rdk=0',
    'ru_cpd_res_4079i_19921212': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102020277&page=1&rdk=0',
    'ru_cpd_res_4088i_19921214': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102020284&page=1&rdk=0',
    'ru_ukaz_1567_19921214': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102020298&page=1&rdk=0',
    'ru_ukaz_1569_19921215': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102020327&page=1&rdk=0',
    'ru_ukaz_1570_19921215': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102020328&page=1&rdk=0',
    'ru_gd_res_624ii_19960810': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102042901&page=1&rdk=0',
    'ru_ukaz_1152_19960810': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102042896&page=1&rdk=0',
    'ru_ukaz_281_19980323': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102052192&page=1&rdk=0',
    'ru_ukaz_287_19980323': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102052188&page=1&rdk=0',
    'ru_ukaz_288_19980323': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102052187&page=1&rdk=0',
    'ru_gd_res_2375ii_19980410': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102052472&page=1&rdk=0',
    'ru_gd_res_2402ii_19980417': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102052549&page=1&rdk=0',
    'ru_gd_res_2421ii_19980424': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102052658&page=1&rdk=0',
    'ru_ukaz_436_19980424': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102052676&page=1&rdk=0',
    'ru_duma_steno_19980424': 'http://transcript.duma.gov.ru/node/2568/',
    'ru_ukaz_983_19980823': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102055100&page=1&rdk=0',
    'ru_ukaz_987_19980825': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102055106&page=1&rdk=0',
    'ru_gd_res_2898ii_19980831': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102055136&page=1&rdk=0',
    'ru_duma_steno_19980831': 'http://transcript.duma.gov.ru/node/2517/',
    'ru_gd_res_2928ii_19980907': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102055315&page=1&rdk=0',
    'ru_gd_res_2961ii_19980911': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102055400&page=1&rdk=0',
    'ru_ukaz_1087_19980911': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102055453&page=1&rdk=0',
    'ru_ukaz_580_19990512': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102059649&page=1&rdk=0',
    'ru_duma_3961ii_19990512': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102059639&page=1&rdk=0',
    'ru_gov_res_528_19990513': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102059661&page=1&rdk=0',
    'ru_duma_3965ii_19990519': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102059766&page=1&rdk=0',
    'ru_ukaz_611_19990519': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102059754&page=1&rdk=0',
    'ru_gov_res_905_19990806': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102061269&page=1&rdk=0',
    'ru_ukaz_1012_19990809': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102061311&page=1&rdk=0',
    'ru_gov_res_923_19990810': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102061319&page=1&rdk=0',
    'ru_duma_4276ii_19990816': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102061352&page=1&rdk=0',
    'ru_ukaz_1052_19990816': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102061345&page=1&rdk=0',
    'ru_ukaz_834_20000507': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102065739&page=1&rdk=0',
    'ru_ukaz_836_20000507': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102065741&page=1&rdk=0',
    'ru_gov_rasp_647r_20000507': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102065738&page=1&rdk=0',
    'ru_duma_363iii_20000517': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102065777&page=1&rdk=0',
    'ru_ukaz_861_20000517': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102065790&page=1&rdk=0',
    'ru_ukaz_264_20040224': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102085491&page=1&rdk=0',
    'ru_gov_res_104_20040225': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102085550&page=1&rdk=0',
    'ru_duma_162iv_20040305': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102085648&page=1&rdk=0',
    'ru_ukaz_300_20040305': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102085647&page=1&rdk=0',
    'ru_ukaz_585_20040507': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102086675&page=1&rdk=0',
    'ru_gov_rasp_608r_20040507': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102086677&page=1&rdk=0',
    'ru_gov_res_232_20040508': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102086682&page=1&rdk=0',
    'ru_gov_res_233_20040511': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102086705&page=1&rdk=0',
    'ru_duma_489iv_20040512': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102086710&page=1&rdk=0',
    'ru_ukaz_610_20040512': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102086708&page=1&rdk=0',
    'ru_ukaz_1184_20070912': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102116591&page=1&rdk=0',
    'ru_gov_res_586_20070913': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102116640&page=1&rdk=0',
    'ru_duma_res_5066_4_gd_20070914': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102116607&page=1&rdk=0',
    'ru_ukaz_1202_20070914': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102116627&page=1&rdk=0',
    'ru_ukaz_717_20080507': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102121789&page=1&rdk=0',
    'ru_gov_rasp_680r_20080507': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102121745&page=1&rdk=0',
    'ru_duma_res_458_5_gd_20080508': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102121651&page=1&rdk=0',
    'ru_ukaz_723_20080508': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102121791&page=1&rdk=0',
    'ru_pub_ukaz_607_20120507': 'http://publication.pravo.gov.ru/file/pdf?eoNumber=0001201205070027',
    'ru_gov_rasp_760r_20120507': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102156287&page=1&rdk=0',
    'ru_pub_duma_res_323_6_gd_20120508': 'http://publication.pravo.gov.ru/file/pdf?eoNumber=0001201205080001',
    'ru_pub_ukaz_612_20120508': 'http://publication.pravo.gov.ru/file/pdf?eoNumber=0001201205080002',
    'ru_pub_ukaz_202_20180507': 'http://publication.pravo.gov.ru/file/pdf?eoNumber=0001201805070024',
    'ru_pub_gov_rasp_875r_20180507': 'http://publication.pravo.gov.ru/file/pdf?eoNumber=0001201805070016',
    'ru_pub_duma_res_3894_7_gd_20180508': 'http://publication.pravo.gov.ru/file/pdf?eoNumber=0001201805080026',
    'ru_pub_ukaz_209_20180508': 'http://publication.pravo.gov.ru/file/pdf?eoNumber=0001201805080028',
    'ru_pub_ukaz_14_20200115': 'http://publication.pravo.gov.ru/file/pdf?eoNumber=0001202001150020',
    'ru_pub_duma_res_7565_7_gd_20200116': 'http://publication.pravo.gov.ru/file/pdf?eoNumber=0001202001160027',
    'ru_pub_ukaz_17_20200116': 'http://publication.pravo.gov.ru/file/pdf?eoNumber=0001202001160035',
    'ru_pub_ukaz_306_20240507': 'http://publication.pravo.gov.ru/file/pdf?eoNumber=0001202405070003',
    'ru_pub_gov_rasp_1121r_20240507': 'http://publication.pravo.gov.ru/file/pdf?eoNumber=0001202405070002',
    'ru_pub_duma_res_6061_8_gd_20240510': 'http://publication.pravo.gov.ru/file/pdf?eoNumber=0001202405100001',
    'ru_pub_ukaz_319_20240510': 'http://publication.pravo.gov.ru/file/pdf?eoNumber=0001202405100015',
    'ru_pub_gov_res_1125_20260903': 'http://publication.pravo.gov.ru/file/pdf?eoNumber=0001202609040021',
}
ATTACHED = {
    'ru_rsfsr_cpd_res_1830i_19911101': [('portal_card_response', 3121, 'acb457281f9aea7cf7157aa13934cd0cb79ebdcf1d184677bcb98d80c1971d1d')],
    'ru_rsfsr_ukaz_171_19911106': [('portal_card_response', 2873, 'b7ee20522604c42570e493bbac59962383a2fe1a7571f967262b07d5c5a290a8')],
    'ru_rsfsr_ukaz_172_19911106': [('portal_card_response', 3078, '2ce25a0cf0a6bfb6e1d3b08d304616312eb543bc092449d27bee0d80c553ec7e')],
    'ru_rsfsr_gov_res_8_19911115': [('portal_card_response', 2964, '2a3c6f999c744cf7c20d7b67e703db9393f148da0ffd518ae85754be6f153273')],
    'ru_ukaz_633_19920615': [('portal_card_response', 3134, '161b2881135b0e784e144e3932f87b0309779d162bebedf0896ced0329884f35')],
    'ru_gov_res_457_19920701': [('portal_card_response', 3120, 'a889f0d8d66e35c6d9a961a80356901e8f312c36e1449a077e77770b056af0de')],
    'ru_cpd_res_4063i_19921209': [('portal_card_response', 3213, 'c553d2578e23809dec3f97f0637407e963393bc363adae2bb56875ce1127a45b')],
    'ru_cpd_res_4079i_19921212': [('portal_card_response', 3134, 'f1cda79e65fd0f3e9968852c42c9fff9bb871d322f277ecc32407f956dbb5916')],
    'ru_cpd_res_4088i_19921214': [('portal_card_response', 3152, '6f8f5533f59731f7a7c3776858174137817e20107dd2dd4182892dc3f692c6d5')],
    'ru_ukaz_1567_19921214': [('portal_card_response', 3263, '4db750218e762c6445feb4773ff111483d6c4de3cdad6ff5b49ab05a8aae73a4')],
    'ru_ukaz_1569_19921215': [('portal_card_response', 3250, '5d0f1509c6dcbad808705976aa43ae54a1c271a4e94b06636c05afa38c25f88f')],
    'ru_ukaz_1570_19921215': [('portal_card_response', 3209, '4efeb84101c0b0baed5606a0cadb9ee715c830c5e2b676d81b54a8646c8c75e1')],
    'ru_gd_res_624ii_19960810': [('portal_card_response', 3211, '32777fbcb449eeaa3f2c7ee4a0119d6574914d99ada138bc89486fc03e636bc1')],
    'ru_ukaz_1152_19960810': [('portal_card_response', 3059, '24be76228027cac829ca39625bf2e374d0d539f905e5c652212d9834066a630d')],
    'ru_ukaz_281_19980323': [('portal_card_response', 3115, '38c29453c7d0584f4e46984c9d92323b6570ea0c293ebd1c5f3ff6e9b900b53f')],
    'ru_ukaz_287_19980323': [('portal_card_response', 3164, '352dba2dfad52e572ac17adc3fcc1f070da65dc9f8a861d04b8b7426520cda0a')],
    'ru_ukaz_288_19980323': [('portal_card_response', 3153, 'f4db87e88c0df1791a86e824e528aa2bf36a703e22438f3219f8567a3cb88335')],
    'ru_gd_res_2375ii_19980410': [('portal_card_response', 3191, 'b3a94a0fb4eaaa097721e6a2de409339d8adfead4b69570aea652d502dc2f123')],
    'ru_gd_res_2402ii_19980417': [('portal_card_response', 3191, 'c931f12052e3dc9205d68ab1ad261e9e355b1bfdab75ee3008b68a1c82b7f2af')],
    'ru_gd_res_2421ii_19980424': [('portal_card_response', 3279, '09743f7100a4634948314873207b68c0a422d496a642fb4a5b2e69e0b8d4c002')],
    'ru_ukaz_436_19980424': [('portal_card_response', 3128, '950c6b1b4bf86495d7015da900cc11e0d514a4b1a182151595f16b03087b7d06')],
    'ru_ukaz_983_19980823': [('portal_card_response', 3115, 'b391b5829312952fb32632708a2d2389716b614b46fd0ab9656625965c7baed9')],
    'ru_ukaz_987_19980825': [('portal_card_response', 3148, 'a030e20379f9c517e2b37af6a1d8f71ea176d2067630707f516630bd02133002')],
    'ru_gd_res_2898ii_19980831': [('portal_card_response', 3191, '8840dd4ab515e5fe1e0fb19d6a00cc933747c06b02dc5b398da21885b853ed73')],
    'ru_gd_res_2928ii_19980907': [('portal_card_response', 3191, 'ca4018a72255a625945f84a9fc1404e23ef7fb2938b34299041406b14601dc44')],
    'ru_gd_res_2961ii_19980911': [('portal_card_response', 3209, '7f953cd1891f2432aacd44b03baeaf4ad8bd887c39f2bff6d30f7c86459c7a1b')],
    'ru_ukaz_1087_19980911': [('portal_card_response', 3059, '5003881ff5b13ce6b402300905c4e8b2d2c0929c84c7c4d36a3f268bf1b333f8')],
    'ru_ukaz_580_19990512': [('portal_card_response', 3128, 'a87cbcab281737d3abb7847f803131641c98a6084eac20ec26082c303b646246')],
    'ru_duma_3961ii_19990512': [('portal_card_response', 3171, '22bfeac206c1111fc1ec2ae0445e3fbfcb22afb0192e1a31702376271ccd1c56')],
    'ru_gov_res_528_19990513': [('portal_card_response', 3036, '01236efbca400e8bdfb75d2ef86a1d8f21b884e313600e10a1cf670e40b902ec')],
    'ru_duma_3965ii_19990519': [('portal_card_response', 3207, 'a834d1af9805063e023c3c4ed00e783335e71c3dfd0da7c415341d5f6fa21be0')],
    'ru_ukaz_611_19990519': [('portal_card_response', 3058, 'e3a6c1bb0bc0969a6224cd93e2657e01eb49c849bbc4125449e102640e3896c4')],
    'ru_gov_res_905_19990806': [('portal_card_response', 3113, 'b6ffc114091f42d11492bccae7877c88d7d2923edb66c7867696a81f39f73be0')],
    'ru_ukaz_1012_19990809': [('portal_card_response', 3129, 'd63d5af9c689e266a23809fcb491d253f42df7db30d3f00f7ff0cad4612bfb63')],
    'ru_gov_res_923_19990810': [('portal_card_response', 3110, '76d0ce95a87e564e6faff89f791c037828ba15c79bfe4b3ffed941dec536859c')],
    'ru_duma_4276ii_19990816': [('portal_card_response', 3210, 'f2b6d00d03cbca694912a9d34606e056eff12111d9b1b0a967ad75a0b3034e06')],
    'ru_ukaz_1052_19990816': [('portal_card_response', 3129, 'f52cb6061a70441867d5655b7b7e0866c21e2b9a1653c375f520558f282a4320')],
    'ru_ukaz_834_20000507': [('portal_card_response', 3153, '896f756ce2328a48712883e8105d1bef64d85a6c1d6e76d9c262d62ece5bb22f')],
    'ru_ukaz_836_20000507': [('portal_card_response', 3201, '4a781f04cbe4d6fcd9d6f24bbd79afa43e4595142256ea5306039e14ed134b64')],
    'ru_gov_rasp_647r_20000507': [('portal_card_response', 3022, 'f68aed24245beb804433ee4beb5e459ec9fd97371573073838a821a555c5fc94')],
    'ru_duma_363iii_20000517': [('portal_card_response', 3197, '09c112d81fc7463f735f0818740974343a9904628506613027144627c3fb66e1')],
    'ru_ukaz_861_20000517': [('portal_card_response', 3128, '864f0b2b75cfd21e05c4665409301d407fbd568308e3fb7d9268e51da159c1af')],
    'ru_ukaz_264_20040224': [('portal_card_response', 3043, '723e3bd4b7580059f80b5f978952b2c8996453b62d01081e65cf93c71ed1e1a2')],
    'ru_gov_res_104_20040225': [('portal_card_response', 3279, '4886ac9c53c03386b1e36c48ae6d2a3aeabce1868211a48323aebfaae1affc17')],
    'ru_duma_162iv_20040305': [('portal_card_response', 3192, 'fb4ea9e1f953267416176d19296254a42a5c5e885f0a07c058243791f5cb51db')],
    'ru_ukaz_300_20040305': [('portal_card_response', 3057, 'f291b26dd31c94a46be7a75b6b5c30429a2b7d9007200013aa205efea2ceda9a')],
    'ru_ukaz_585_20040507': [('portal_card_response', 3084, 'c4652207cd03195b1880b3a5dd678d4f9e2c623369fea50592f34e1b09adf9af')],
    'ru_gov_rasp_608r_20040507': [('portal_card_response', 3040, 'fb0ae3cf6afaeff9fd9a9121eed0fa7536b26538ee9375b63bf425ef7006eccf')],
    'ru_gov_res_232_20040508': [('portal_card_response', 3204, '6d37def800b411f14c5ee0d42df323899e8393fbc2961d8cf3224614f088e4ec')],
    'ru_gov_res_233_20040511': [('portal_card_response', 3210, '1db411a6cf1229b8bf3c455ffc8af099401bd2af537205ed863db889e45d7632')],
    'ru_duma_489iv_20040512': [('portal_card_response', 3266, 'e45d282739cdc34b21c93b793632e5622a97e1f03801d8c214e4944f4679d0b8')],
    'ru_ukaz_610_20040512': [('portal_card_response', 3128, '4058fac618502de9b525165c02b47333aa96ff2903ed5c7352134c0aea0b6b7b')],
    'ru_ukaz_1184_20070912': [('portal_card_response', 3131, '5f208b661b61b634c6e25b20f813d7f964e9dc6f720631ed3f169978d765681b')],
    'ru_gov_res_586_20070913': [('portal_card_response', 3315, '4721e79bfaa93b81e6d7fbb4d53ddaa3444d5a59f5ae4b0437da16c9c86d5ad2')],
    'ru_duma_res_5066_4_gd_20070914': [('portal_card_response', 3194, 'a2d2bafba5a8a42bef03fe7c7a5e63d603377c5e075bc7dd0cfeec51767648da')],
    'ru_ukaz_1202_20070914': [('portal_card_response', 3129, '6915f2853295251997b12205f06e6e1343a232dc8bcb1357f4a0750801862050')],
    'ru_ukaz_717_20080507': [('portal_card_response', 3066, '1128d36eba57b3d18aa27567f02c41b1755c48bceb94ca819a9ad2e144378499')],
    'ru_gov_rasp_680r_20080507': [('portal_card_response', 3022, '5e435ef933aa3ec65750b0122aee44b7cadf55f459db8a405b7a9abf86518d71')],
    'ru_duma_res_458_5_gd_20080508': [('portal_card_response', 3269, 'c8f62fa24a1e38538aaa518e4cdc49b6c8aff1c7f097a91e6500121def7954ad')],
    'ru_ukaz_723_20080508': [('portal_card_response', 3200, '04fbbbbefee68c2336b163752385aa62e262b5172419fb0cd8881885a1127199')],
    'ru_pub_ukaz_607_20120507': [('portal_card_response', 3209, '0e46e68baf9467e3283360834b86df52a06c213012ae1d832af06058e8ffeef0'), ('corroborating_responses', 24298, '168d27da9224f1e8bf4a0569f0e43a2eb346797c22c83f6c816f9600fa925d71')],
    'ru_gov_rasp_760r_20120507': [('portal_card_response', 3022, '2e26d137774823c08cb8e78e9f45c6ff518e1cf8d8bb31c32c1160400afdfd60')],
    'ru_pub_duma_res_323_6_gd_20120508': [('portal_card_response', 3412, '5e0e254036a98b3f94513ac2fc858cf432098a38eceea0b692151859830d4e91'), ('corroborating_responses', 24457, '506a9d9bede6c1447d507f7c1202ecc975803d4719479f9b76e25529626f84ec')],
    'ru_pub_ukaz_612_20120508': [('portal_card_response', 3270, 'c7b51f8652a459802c4de0b466100482e224659cd32bf5f1be666b3fb1a047a2'), ('corroborating_responses', 19785, 'b8df7269081481e0901b6fe398b4a39dc31292b7dc5b7b7eee46bc47055c1941')],
    'ru_pub_ukaz_202_20180507': [('portal_card_response', 3209, '3bd4165813eab9e91be890c6f6f7d28b44c948a2cafb735b6cfd2a18c7c41fea'), ('corroborating_responses', 24293, 'ea14b4a46fdad7207ad922694dd88eede249466474537d2c9d3b081eda0864cf')],
    'ru_pub_gov_rasp_875r_20180507': [('portal_card_response', 3165, '4f03448bc3d34a06bedb0e93392485269529420047c8508b68ec7c34a23b2e9c'), ('corroborating_responses', 17916, 'aa4e962d2ff8df6c620ce5f38e972a42e905bc495f9aeea5ee2d6032627b1301')],
    'ru_pub_duma_res_3894_7_gd_20180508': [('portal_card_response', 3340, '8ed9de95b47377828e988e615955a7e349ef8b53c0ed7379317bdb498404fcde'), ('corroborating_responses', 20579, 'f26edfcec2d295fc290d7a70c4d5cc1b179def73545d98cee127206d2e0e6fa2')],
    'ru_pub_ukaz_209_20180508': [('portal_card_response', 3270, 'f1dba388b8c26a7b28ff78436905d877f31b263903a543d025007884ddc818de'), ('corroborating_responses', 18517, '76329d071990ff4d636016190a144c9f47d41341e7401f91fbfce63dda15816a')],
    'ru_pub_ukaz_14_20200115': [('portal_card_response', 3186, 'bdb229aae44edeaef4bf7ecdcf0ee8d3dd58e8668cbb23981aa219176df34518'), ('corroborating_responses', 17995, 'c6a51abb4f9d566ce9497ddb851867a0d3a2f88e67a5846548ac0df72ad38c00')],
    'ru_pub_duma_res_7565_7_gd_20200116': [('portal_card_response', 3340, 'af2f606c0b9f696d444b13f6a80d7ab07fd9162a42dd2388c458fa7fec103938'), ('corroborating_responses', 18306, '91e272205fd239d535f0ac370646bd6ebadd546f328e3d85001bf14b23862dae')],
    'ru_pub_ukaz_17_20200116': [('portal_card_response', 3269, 'bfa6f3b411aa266f581d44e73c1b4c2100005bb2abe14ecdfc31d19f3ae991a5'), ('corroborating_responses', 17649, '2c1b1d05a727a0c627b67e2ddfaaae6bc030cf1e89d19d3de10c1f520e4de4c4')],
    'ru_pub_ukaz_306_20240507': [('portal_card_response', 3209, 'dfbde56a5c3042dfe006d43017cab5dc7254b79cf808038eacdb9004ce58f2d8'), ('corroborating_responses', 24340, 'bf31b202b14ed5e9f6f04d03fe39d75c73ca4431f3fe23202713e11e4c0c4e39')],
    'ru_pub_gov_rasp_1121r_20240507': [('portal_card_response', 3166, '9918b6d8a16fbae9d2ae840d51d17a6c713ee8de9b576275b016d12c99bca734'), ('corroborating_responses', 23862, 'd62d1bcee3df6bbfa2ad46c220043f8723650688b220c2f8b1b8cfdcc6b8fbdc')],
    'ru_pub_duma_res_6061_8_gd_20240510': [('portal_card_response', 3295, 'f983584bb959aeebf035e62fa55b0599c6009d42a35d7a780f2049ee4fd55f46'), ('corroborating_responses', 24321, 'b9913956d7d6061f6e6bcadef027b1bf55009e0137620beda8f56001950299eb')],
    'ru_pub_ukaz_319_20240510': [('portal_card_response', 3272, 'c066263a812f4a9a45b0107cfe9de41f0b6c38dc7d66f666888a4f2dfe710194'), ('corroborating_responses', 23749, '4e118998ebc838619775ad42b4a1ecc2b626439f1ce96361bab7dc304b7fa6a2')],
    'ru_pub_gov_res_1125_20260903': [('card_response', 20324, 'f860a58cf6f085c6a258356405f2cfe03b5e78c2467a33bee426509e44bc1593')],
}
EVENTS = {
    'ru_cpd_1830i_president_reorganizes_executive_19911101': ('1991-11-01', 'executive_reorganisation_authorised', 'RU-GOV-01'),
    'ru_ukaz_171_president_heads_government_19911106': ('1991-11-06', 'head_of_government_designation', 'RU-GOV-01'),
    'ru_ukaz_171_council_of_ministers_continues_19911106': ('1991-11-06', 'continuation_of_duties', 'RU-GOV-01'),
    'ru_ukaz_172_president_heads_government_19911106': ('1991-11-06', 'head_of_government_designation', 'RU-GOV-01'),
    'ru_gov_res_8_signed_yeltsin_19911115': ('1991-11-15', 'government_act_signature', 'RU-GOV-01'),
    'ru_ukaz_633_gaidar_acting_chairman_19920615': ('1992-06-15', 'acting_service_designation', 'RU-GOV-01'),
    'ru_gov_res_457_signed_gaidar_19920701': ('1992-07-01', 'government_act_signature', 'RU-GOV-01'),
    'ru_cpd_4063i_secret_ballot_results_approved_19921209': ('1992-12-09', 'parliamentary_vote_results_approved', 'RU-GOV-02'),
    'ru_cpd_4079i_chairman_selection_procedure_19921212': ('1992-12-12', 'selection_procedure_set', 'RU-GOV-02'),
    'ru_cpd_4088i_chernomyrdin_approved_19921214': ('1992-12-14', 'parliamentary_approval', 'RU-GOV-02'),
    'ru_ukaz_1567_chernomyrdin_appointed_19921214': ('1992-12-14', 'appointment', 'RU-GOV-02'),
    'ru_ukaz_1569_chairman_post_vacant_19921215': ('1992-12-15', 'vacancy_statement', 'RU-GOV-01'),
    'ru_ukaz_1569_current_government_continues_19921215': ('1992-12-15', 'continuation_of_duties', 'RU-GOV-02'),
    'ru_ukaz_1570_gaidar_released_from_acting_19921215': ('1992-12-15', 'release_from_acting_service', 'RU-GOV-01'),
    'ru_gd_624ii_consent_chernomyrdin_19960810': ('1996-08-10', 'parliamentary_consent', 'RU-GOV-02'),
    'ru_ukaz_1152_chernomyrdin_appointed_19960810': ('1996-08-10', 'appointment', 'RU-GOV-02'),
    'ru_duma_steno_nomination_letter_presented_19960810': ('1996-08-10', 'nomination_presented', 'RU-GOV-02'),
    'ru_duma_steno_consent_vote_19960810': ('1996-08-10', 'parliamentary_vote', 'RU-GOV-02'),
    'ru_ukaz_281_government_dismissed_19980323': ('1998-03-23', 'government_resignation_announced', 'RU-GOV-03'),
    'ru_ukaz_281_members_continue_19980323': ('1998-03-23', 'continuation_of_duties', 'RU-GOV-03'),
    'ru_ukaz_281_president_assumes_acting_duties_19980323': ('1998-03-23', 'acting_service_designation', 'RU-GOV-03'),
    'ru_ukaz_287_acting_point_repealed_19980323': ('1998-03-23', 'acting_designation_revoked', 'RU-GOV-03'),
    'ru_ukaz_288_kiriyenko_acting_19980323': ('1998-03-23', 'acting_service_designation', 'RU-GOV-03'),
    'ru_gd_2375ii_kiriyenko_rejected_19980410': ('1998-04-10', 'parliamentary_consent_refused', 'RU-GOV-03'),
    'ru_duma_steno_kiriyenko_submitted_19980410': ('1998-04-10', 'nomination_presented', 'RU-GOV-03'),
    'ru_duma_steno_kiriyenko_consent_vote_failed_19980410': ('1998-04-10', 'parliamentary_vote', 'RU-GOV-03'),
    'ru_gd_2402ii_kiriyenko_rejected_19980417': ('1998-04-17', 'parliamentary_consent_refused', 'RU-GOV-03'),
    'ru_duma_steno_kiriyenko_resubmitted_19980417': ('1998-04-17', 'nomination_presented', 'RU-GOV-03'),
    'ru_duma_steno_kiriyenko_consent_vote_failed_19980417': ('1998-04-17', 'parliamentary_vote', 'RU-GOV-03'),
    'ru_duma_steno_third_submission_not_received_19980417': ('1998-04-17', 'nomination_status_statement', 'RU-GOV-03'),
    'ru_gd_2421ii_consent_kiriyenko_19980424': ('1998-04-24', 'parliamentary_consent', 'RU-GOV-03'),
    'ru_ukaz_436_kiriyenko_appointed_19980424': ('1998-04-24', 'appointment', 'RU-GOV-03'),
    'ru_duma_steno_kiriyenko_third_submission_presented_19980424': ('1998-04-24', 'nomination_presented', 'RU-GOV-03'),
    'ru_duma_steno_kiriyenko_secret_ballot_result_19980424': ('1998-04-24', 'parliamentary_vote', 'RU-GOV-03'),
    'ru_ukaz_983_government_dismissed_19980823': ('1998-08-23', 'government_resignation_announced', 'RU-GOV-04'),
    'ru_ukaz_983_chernomyrdin_acting_19980823': ('1998-08-23', 'acting_service_designation', 'RU-GOV-04'),
    'ru_ukaz_987_members_continue_19980825': ('1998-08-25', 'continuation_of_duties', 'RU-GOV-04'),
    'ru_gd_2898ii_chernomyrdin_rejected_19980831': ('1998-08-31', 'parliamentary_consent_refused', 'RU-GOV-04'),
    'ru_duma_steno_chernomyrdin_first_submission_presented_19980831': ('1998-08-31', 'nomination_presented', 'RU-GOV-04'),
    'ru_duma_steno_chernomyrdin_consent_vote_failed_19980831': ('1998-08-31', 'parliamentary_vote', 'RU-GOV-04'),
    'ru_gd_2928ii_chernomyrdin_rejected_19980907': ('1998-09-07', 'parliamentary_consent_refused', 'RU-GOV-04'),
    'ru_duma_steno_chernomyrdin_resubmitted_19980907': ('1998-09-07', 'nomination_presented', 'RU-GOV-04'),
    'ru_duma_steno_chernomyrdin_styled_acting_19980907': ('1998-09-07', 'acting_service_attestation', 'RU-GOV-04'),
    'ru_duma_steno_chernomyrdin_consent_vote_failed_19980907': ('1998-09-07', 'parliamentary_vote', 'RU-GOV-04'),
    'ru_gd_2961ii_consent_primakov_19980911': ('1998-09-11', 'parliamentary_consent', 'RU-GOV-04'),
    'ru_duma_steno_primakov_submitted_19980911': ('1998-09-11', 'nomination_presented', 'RU-GOV-04'),
    'ru_duma_steno_primakov_consent_vote_19980911': ('1998-09-11', 'parliamentary_vote', 'RU-GOV-04'),
    'ru_ukaz_1087_primakov_appointed_19980911': ('1998-09-11', 'appointment', 'RU-GOV-04'),
    'ru_ukaz_580_government_dismissal_announced_19990512': ('1999-05-12', 'government_resignation_announced', 'RU-GOV-05'),
    'ru_ukaz_580_stepashin_acting_19990512': ('1999-05-12', 'acting_service_designation', 'RU-GOV-05'),
    'ru_ukaz_580_members_continue_19990512': ('1999-05-12', 'continuation_of_duties', 'RU-GOV-05'),
    'ru_duma_3961ii_primakov_government_dismissed_19990512': ('1999-05-12', 'dismissal_reference', 'RU-GOV-05'),
    'ru_gov_res_528_stepashin_signs_as_acting_19990513': ('1999-05-13', 'acting_service_attestation', 'RU-GOV-05'),
    'ru_duma_3965ii_consent_stepashin_19990519': ('1999-05-19', 'parliamentary_consent', 'RU-GOV-05'),
    'ru_ukaz_611_stepashin_appointed_19990519': ('1999-05-19', 'appointment', 'RU-GOV-05'),
    'ru_gov_res_905_stepashin_signs_as_chairman_19990806': ('1999-08-06', 'in_office_attestation', 'RU-GOV-05'),
    'ru_ukaz_1012_government_dismissal_announced_19990809': ('1999-08-09', 'government_resignation_announced', 'RU-GOV-05'),
    'ru_ukaz_1012_putin_acting_19990809': ('1999-08-09', 'acting_service_designation', 'RU-GOV-05'),
    'ru_ukaz_1012_members_continue_19990809': ('1999-08-09', 'continuation_of_duties', 'RU-GOV-05'),
    'ru_gov_res_923_putin_signs_as_acting_19990810': ('1999-08-10', 'acting_service_attestation', 'RU-GOV-05'),
    'ru_duma_4276ii_consent_putin_19990816': ('1999-08-16', 'parliamentary_consent', 'RU-GOV-05'),
    'ru_ukaz_1052_putin_appointed_19990816': ('1999-08-16', 'appointment', 'RU-GOV-05'),
    'ru_ukaz_834_kasyanov_acting_20000507': ('2000-05-07', 'acting_service_designation', 'RU-GOV-06'),
    'ru_ukaz_836_government_surrendered_powers_20000507': ('2000-05-07', 'government_powers_laid_down_recital', 'RU-GOV-06'),
    'ru_ukaz_836_government_to_continue_20000507': ('2000-05-07', 'continuation_of_duties', 'RU-GOV-06'),
    'ru_gov_647r_government_surrenders_powers_20000507': ('2000-05-07', 'government_powers_laid_down', 'RU-GOV-06'),
    'ru_gov_647r_kasyanov_signs_as_acting_20000507': ('2000-05-07', 'acting_service_attestation', 'RU-GOV-06'),
    'ru_kremlin_putin_nominates_kasyanov_20000510': ('2000-05-10', 'nomination', 'RU-GOV-06'),
    'ru_duma_363iii_consent_kasyanov_20000517': ('2000-05-17', 'parliamentary_consent', 'RU-GOV-06'),
    'ru_ukaz_861_kasyanov_appointed_20000517': ('2000-05-17', 'appointment', 'RU-GOV-06'),
    'ru_kremlin_kasyanov_certificate_presented_20000517': ('2000-05-17', 'certificate_presentation', 'RU-GOV-06'),
    'ru_ukaz_264_government_dismissal_announced_20040224': ('2004-02-24', 'government_resignation_announced', 'RU-GOV-06'),
    'ru_ukaz_264_khristenko_acting_20040224': ('2004-02-24', 'acting_service_designation', 'RU-GOV-06'),
    'ru_ukaz_264_government_to_continue_20040224': ('2004-02-24', 'continuation_of_duties', 'RU-GOV-06'),
    'ru_kremlin_putin_statement_dismissal_decision_20040224': ('2004-02-24', 'resignation_decision_statement', 'RU-GOV-06'),
    'ru_kremlin_putin_statement_nomination_intention_20040224': ('2004-02-24', 'nomination_intention', 'RU-GOV-06'),
    'ru_kremlin_putin_statement_government_continue_20040224': ('2004-02-24', 'continuation_of_duties', 'RU-GOV-06'),
    'ru_kremlin_kasyanov_styled_former_chairman_20040224': ('2004-02-24', 'former_holder_styling', 'RU-GOV-06'),
    'ru_gov_res_104_khristenko_signs_as_acting_20040225': ('2004-02-25', 'acting_service_attestation', 'RU-GOV-06'),
    'ru_kremlin_putin_proposes_fradkov_20040301': ('2004-03-01', 'nomination_proposal', 'RU-GOV-07'),
    'ru_duma_162iv_consent_fradkov_20040305': ('2004-03-05', 'parliamentary_consent', 'RU-GOV-07'),
    'ru_ukaz_300_fradkov_appointed_20040305': ('2004-03-05', 'appointment', 'RU-GOV-07'),
    'ru_ukaz_585_government_surrendered_powers_20040507': ('2004-05-07', 'government_powers_laid_down_recital', 'RU-GOV-07'),
    'ru_ukaz_585_government_to_continue_20040507': ('2004-05-07', 'continuation_of_duties', 'RU-GOV-07'),
    'ru_gov_608r_government_surrenders_powers_20040507': ('2004-05-07', 'government_powers_laid_down', 'RU-GOV-07'),
    'ru_gov_608r_fradkov_signs_as_chairman_20040507': ('2004-05-07', 'in_office_attestation', 'RU-GOV-07'),
    'ru_kremlin_premier_order_received_20040507': ('2004-05-07', 'government_powers_laid_down_reported', 'RU-GOV-07'),
    'ru_kremlin_decree_585_signing_reported_20040507': ('2004-05-07', 'continuation_of_duties_reported', 'RU-GOV-07'),
    'ru_kremlin_putin_to_submit_fradkov_20040507': ('2004-05-07', 'nomination_intention', 'RU-GOV-07'),
    'ru_kremlin_putin_nominates_fradkov_20040507': ('2004-05-07', 'nomination', 'RU-GOV-07'),
    'ru_gov_res_232_fradkov_signs_as_acting_20040508': ('2004-05-08', 'acting_service_attestation', 'RU-GOV-07'),
    'ru_gov_res_233_fradkov_signs_as_acting_20040511': ('2004-05-11', 'acting_service_attestation', 'RU-GOV-07'),
    'ru_duma_489iv_consent_fradkov_20040512': ('2004-05-12', 'parliamentary_consent', 'RU-GOV-07'),
    'ru_ukaz_610_fradkov_appointed_20040512': ('2004-05-12', 'appointment', 'RU-GOV-07'),
    'ru_ukaz_1184_government_resignation_announced_20070912': ('2007-09-12', 'government_resignation_announced', 'RU-GOV-07'),
    'ru_ukaz_1184_fradkov_acting_20070912': ('2007-09-12', 'acting_service_designation', 'RU-GOV-07'),
    'ru_ukaz_1184_government_to_continue_20070912': ('2007-09-12', 'continuation_of_duties', 'RU-GOV-07'),
    'ru_kremlin_fradkov_requests_resignation_20070912': ('2007-09-12', 'resignation_request', 'RU-GOV-07'),
    'ru_kremlin_putin_accepts_resignation_20070912': ('2007-09-12', 'resignation_acceptance', 'RU-GOV-07'),
    'ru_kremlin_putin_asks_fradkov_to_act_20070912': ('2007-09-12', 'acting_service_request', 'RU-GOV-07'),
    'ru_kremlin_steno_fradkov_asks_resignation_20070912': ('2007-09-12', 'resignation_request', 'RU-GOV-07'),
    'ru_kremlin_steno_putin_accepts_resignation_20070912': ('2007-09-12', 'resignation_acceptance', 'RU-GOV-07'),
    'ru_kremlin_steno_putin_asks_fradkov_to_act_20070912': ('2007-09-12', 'acting_service_request', 'RU-GOV-07'),
    'ru_gov_res_586_fradkov_signs_as_acting_20070913': ('2007-09-13', 'acting_service_attestation', 'RU-GOV-07'),
    'ru_duma_5066_4_consent_zubkov_20070914': ('2007-09-14', 'parliamentary_consent', 'RU-GOV-08'),
    'ru_ukaz_1202_zubkov_appointed_20070914': ('2007-09-14', 'appointment', 'RU-GOV-08'),
    'ru_kremlin_42319_zubkov_nominated_20070912': ('2007-09-12', 'nomination', 'RU-GOV-08'),
    'ru_kremlin_42319_duma_consent_reported_20070914': ('2007-09-14', 'parliamentary_consent_reported', 'RU-GOV-08'),
    'ru_kremlin_42319_appointment_decree_signed_20070914': ('2007-09-14', 'appointment_reported', 'RU-GOV-08'),
    'ru_ukaz_717_government_powers_laid_down_20080507': ('2008-05-07', 'government_powers_laid_down_recital', 'RU-GOV-08'),
    'ru_ukaz_717_government_to_continue_20080507': ('2008-05-07', 'continuation_of_duties', 'RU-GOV-08'),
    'ru_kremlin_6_putin_nominated_20080507': ('2008-05-07', 'nomination', 'RU-GOV-08'),
    'ru_gov_680r_government_powers_laid_down_20080507': ('2008-05-07', 'government_powers_laid_down', 'RU-GOV-08'),
    'ru_gov_680r_zubkov_signs_as_chairman_20080507': ('2008-05-07', 'in_office_attestation', 'RU-GOV-08'),
    'ru_kremlin_20_duma_vote_putin_20080508': ('2008-05-08', 'parliamentary_vote_reported', 'RU-GOV-08'),
    'ru_duma_458_5_consent_putin_20080508': ('2008-05-08', 'parliamentary_consent', 'RU-GOV-08'),
    'ru_ukaz_723_putin_appointed_20080508': ('2008-05-08', 'appointment', 'RU-GOV-08'),
    'ru_pub_ukaz_607_government_powers_laid_down_20120507': ('2012-05-07', 'government_powers_laid_down_recital', 'RU-GOV-09'),
    'ru_pub_ukaz_607_government_to_continue_20120507': ('2012-05-07', 'continuation_of_duties', 'RU-GOV-09'),
    'ru_kremlin_15230_medvedev_nominated_20120507': ('2012-05-07', 'nomination', 'RU-GOV-09'),
    'ru_gov_760r_government_powers_laid_down_20120507': ('2012-05-07', 'government_powers_laid_down', 'RU-GOV-09'),
    'ru_gov_760r_zubkov_signs_as_acting_20120507': ('2012-05-07', 'acting_service_attestation', 'RU-GOV-09'),
    'ru_kremlin_15266_duma_vote_medvedev_20120508': ('2012-05-08', 'parliamentary_vote_reported', 'RU-GOV-09'),
    'ru_pub_duma_323_6_consent_medvedev_20120508': ('2012-05-08', 'parliamentary_consent', 'RU-GOV-09'),
    'ru_pub_ukaz_612_medvedev_appointed_20120508': ('2012-05-08', 'appointment', 'RU-GOV-09'),
    'ru_pub_ukaz_202_government_powers_laid_down_20180507': ('2018-05-07', 'government_powers_laid_down_recital', 'RU-GOV-09'),
    'ru_pub_ukaz_202_government_to_continue_20180507': ('2018-05-07', 'continuation_of_duties', 'RU-GOV-09'),
    'ru_kremlin_57422_medvedev_nominated_20180507': ('2018-05-07', 'nomination', 'RU-GOV-09'),
    'ru_pub_gov_875r_government_powers_laid_down_20180507': ('2018-05-07', 'government_powers_laid_down', 'RU-GOV-09'),
    'ru_pub_gov_875r_medvedev_signs_as_chairman_20180507': ('2018-05-07', 'in_office_attestation', 'RU-GOV-09'),
    'ru_pub_duma_3894_7_consent_medvedev_20180508': ('2018-05-08', 'parliamentary_consent', 'RU-GOV-09'),
    'ru_pub_ukaz_209_medvedev_appointed_20180508': ('2018-05-08', 'appointment', 'RU-GOV-09'),
    'ru_kremlin_62585_medvedev_proposes_resignation_20200115': ('2020-01-15', 'resignation_proposed', 'RU-GOV-09'),
    'ru_kremlin_62585_putin_asks_duties_continue_20200115': ('2020-01-15', 'continuation_of_duties', 'RU-GOV-09'),
    'ru_pub_ukaz_14_government_resignation_announced_20200115': ('2020-01-15', 'government_resignation_announced', 'RU-GOV-09'),
    'ru_pub_ukaz_14_medvedev_acting_chairman_20200115': ('2020-01-15', 'acting_service_designation', 'RU-GOV-09'),
    'ru_pub_ukaz_14_government_to_continue_20200115': ('2020-01-15', 'continuation_of_duties', 'RU-GOV-09'),
    'ru_kremlin_62586_mishustin_nominated_20200115': ('2020-01-15', 'nomination', 'RU-GOV-10'),
    'ru_pub_duma_7565_7_consent_mishustin_20200116': ('2020-01-16', 'parliamentary_consent', 'RU-GOV-10'),
    'ru_pub_ukaz_17_mishustin_appointed_20200116': ('2020-01-16', 'appointment', 'RU-GOV-10'),
    'ru_pub_ukaz_306_government_powers_laid_down_20240507': ('2024-05-07', 'government_powers_laid_down_recital', 'RU-GOV-10'),
    'ru_pub_ukaz_306_government_to_continue_20240507': ('2024-05-07', 'continuation_of_duties', 'RU-GOV-10'),
    'ru_pub_gov_1121r_government_powers_laid_down_20240507': ('2024-05-07', 'government_powers_laid_down', 'RU-GOV-10'),
    'ru_pub_gov_1121r_mishustin_signs_as_chairman_20240507': ('2024-05-07', 'in_office_attestation', 'RU-GOV-10'),
    'ru_kremlin_74009_mishustin_styled_acting_20240510': ('2024-05-10', 'acting_service_attestation', 'RU-GOV-10'),
    'ru_kremlin_74009_nomination_submitted_20240510': ('2024-05-10', 'nomination_reference', 'RU-GOV-10'),
    'ru_pub_duma_6061_8_approval_mishustin_20240510': ('2024-05-10', 'parliamentary_approval', 'RU-GOV-10'),
    'ru_pub_ukaz_319_mishustin_appointed_20240510': ('2024-05-10', 'appointment', 'RU-GOV-10'),
    'ru_duma_steno_mishustin_submitted_20240509': ('2024-05-09', 'nomination', 'RU-GOV-10'),
    'ru_duma_steno_mishustin_approval_vote_20240510': ('2024-05-10', 'parliamentary_vote', 'RU-GOV-10'),
    'ru_pub_gov_res_1125_mishustin_signs_as_chairman_20260903': ('2026-09-03', 'in_office_attestation', 'RU-GOV-10'),
    'ru_ukaz_1146_resignation_statement_accepted_19960809': ('1996-08-09', 'government_resignation_accepted', 'RU-GOV-02'),
    'ru_ukaz_1146_government_continues_19960809': ('1996-08-09', 'continuation_of_duties', 'RU-GOV-02'),
}
NEVER_HOLDER_DATE = {'1991-11-01', '1991-11-06', '1991-11-15', '1992-06-15', '1992-07-01', '1992-12-09', '1992-12-12', '1992-12-15', '1996-08-09', '1998-03-23', '1998-04-10', '1998-04-17', '1998-08-23', '1998-08-25', '1998-08-31', '1998-09-07', '1999-05-12', '1999-05-13', '1999-08-06', '1999-08-09', '1999-08-10', '2000-05-07', '2000-05-10', '2004-02-24', '2004-02-25', '2004-03-01', '2004-05-07', '2004-05-08', '2004-05-11', '2007-09-12', '2007-09-13', '2008-05-07', '2012-05-07', '2018-05-07', '2020-01-15', '2024-05-07', '2024-05-09', '2026-09-03', '2026-09-04'}
NEW_SOURCES = list(RESPONSES)
# Two State Duma stenograms are live pages whose footer prints the current year: their identities expire on 1 January 2027.
LIVE_FOOTER = ('ru_duma_steno_19980424', 'ru_duma_steno_19980831')
# CLAUDE-C01-19 rows added to decree 1146's CLAUDE-C01-14 source and extract (the only edited existing extract).
SHARED_1146 = ['ru_ukaz_1146_resignation_statement_accepted_19960809', 'ru_ukaz_1146_government_continues_19960809']
SHARED_SOURCE = 'ru_ukaz_1146_19960809'

INST, ROLE = 'ru_government', 'ru_government_chairman'
ROLE_TITLE = 'Председатель Правительства — Chairman of the Government'
ACTING_TITLE = 'Исполняющий обязанности Председателя Правительства — Acting Chairman of the Government'
INST_NAME = 'Правительство Российской Федерации — Government of the Russian Federation'
CHERN, KIR, PRIM = 'Виктор Степанович Черномырдин', 'Сергей Владиленович Кириенко', 'Евгений Максимович Примаков'
STEP, PUTIN, KAS = 'Сергей Вадимович Степашин', 'Владимир Владимирович Путин', 'Михаил Михайлович Касьянов'
FRAD, ZUB, MED = 'Михаил Ефимович Фрадков', 'Виктор Алексеевич Зубков', 'Дмитрий Анатольевич Медведев'
MISH, YELTSIN, GAIDAR, KHR = 'Михаил Владимирович Мишустин', 'Борис Николаевич Ельцин', 'Егор Тимурович Гайдар', 'Христенко В.Б.'
NAMES = {CHERN, KIR, PRIM, STEP, PUTIN, KAS, FRAD, ZUB, MED, MISH, YELTSIN, GAIDAR, KHR}
# Exact holder observations on ru_government_chairman, in chronological order: (name, attested_on, from, until).
HOLDERS = [
    (CHERN, '1992-12-14', None, None),
    (CHERN, '1996-08-10', None, None),
    (KIR, '1998-04-24', None, None),
    (PRIM, '1998-09-11', None, None),
    (STEP, '1999-05-19', None, None),
    (PUTIN, None, '1999-08-16', None),
    (KAS, None, '2000-05-17', None),
    (FRAD, None, '2004-03-05', None),
    (FRAD, None, '2004-05-12', None),
    (ZUB, None, '2007-09-14', None),
    (PUTIN, None, '2008-05-08', None),
    (MED, None, '2012-05-08', None),
    (MED, None, '2018-05-08', None),
    (MISH, None, '2020-01-16', None),
    (MISH, None, '2024-05-10', None),
]
HOLDER_CLAIMS = [
    ['ru_ukaz_1567_chernomyrdin_appointed_19921214'],
    ['ru_ukaz_1152_chernomyrdin_appointed_19960810'],
    ['ru_ukaz_436_kiriyenko_appointed_19980424'],
    ['ru_ukaz_1087_primakov_appointed_19980911'],
    ['ru_ukaz_611_stepashin_appointed_19990519', 'ru_gov_res_905_stepashin_signs_as_chairman_19990806'],
    ['ru_ukaz_1052_putin_appointed_19990816'],
    ['ru_ukaz_861_kasyanov_appointed_20000517'],
    ['ru_ukaz_300_fradkov_appointed_20040305', 'ru_gov_608r_fradkov_signs_as_chairman_20040507'],
    ['ru_ukaz_610_fradkov_appointed_20040512'],
    ['ru_ukaz_1202_zubkov_appointed_20070914', 'ru_gov_680r_zubkov_signs_as_chairman_20080507'],
    ['ru_ukaz_723_putin_appointed_20080508'],
    ['ru_pub_ukaz_612_medvedev_appointed_20120508', 'ru_pub_gov_875r_medvedev_signs_as_chairman_20180507'],
    ['ru_pub_ukaz_209_medvedev_appointed_20180508'],
    ['ru_pub_ukaz_17_mishustin_appointed_20200116', 'ru_pub_gov_1121r_mishustin_signs_as_chairman_20240507'],
    ['ru_pub_ukaz_319_mishustin_appointed_20240510', 'ru_pub_gov_res_1125_mishustin_signs_as_chairman_20260903'],
]
# A start rests only on an appointment decree that states it takes effect on signing; an attested day only on an appointment
# decree without such a clause. No holder has an end: no reviewed source states the day a Chairman's office ended.
STARTS = {'1999-08-16': 'ru_ukaz_1052_putin_appointed_19990816', '2000-05-17': 'ru_ukaz_861_kasyanov_appointed_20000517',
          '2004-03-05': 'ru_ukaz_300_fradkov_appointed_20040305', '2004-05-12': 'ru_ukaz_610_fradkov_appointed_20040512',
          '2007-09-14': 'ru_ukaz_1202_zubkov_appointed_20070914', '2008-05-08': 'ru_ukaz_723_putin_appointed_20080508',
          '2012-05-08': 'ru_pub_ukaz_612_medvedev_appointed_20120508', '2018-05-08': 'ru_pub_ukaz_209_medvedev_appointed_20180508',
          '2020-01-16': 'ru_pub_ukaz_17_mishustin_appointed_20200116', '2024-05-10': 'ru_pub_ukaz_319_mishustin_appointed_20240510'}
ATTESTED = {'1992-12-14': 'ru_ukaz_1567_chernomyrdin_appointed_19921214', '1996-08-10': 'ru_ukaz_1152_chernomyrdin_appointed_19960810',
            '1998-04-24': 'ru_ukaz_436_kiriyenko_appointed_19980424', '1998-09-11': 'ru_ukaz_1087_primakov_appointed_19980911',
            '1999-05-19': 'ru_ukaz_611_stepashin_appointed_19990519'}
NO_EFFECT_CLAUSE = tuple(ATTESTED.values())
HOLDER_KINDS = {'appointment', 'in_office_attestation'}
ACTING_KINDS = {'acting_service_designation', 'acting_service_attestation', 'acting_designation_revoked',
                'release_from_acting_service', 'acting_service_request'}
BODY_KINDS = {'executive_reorganisation_authorised', 'continuation_of_duties', 'continuation_of_duties_reported',
              'government_resignation_announced', 'government_resignation_accepted', 'government_powers_laid_down',
              'government_powers_laid_down_recital', 'government_powers_laid_down_reported', 'resignation_decision_statement',
              'dismissal_reference', 'resignation_proposed'}
NOMINATION_KINDS = {'nomination', 'nomination_presented', 'nomination_reference', 'nomination_proposal', 'nomination_intention',
                    'nomination_status_statement'}
PARLIAMENT_KINDS = {'parliamentary_consent', 'parliamentary_approval', 'parliamentary_consent_refused', 'parliamentary_vote',
                    'parliamentary_vote_reported', 'parliamentary_consent_reported', 'parliamentary_vote_results_approved',
                    'selection_procedure_set'}
CONTEXT_KINDS = {'head_of_government_designation', 'government_act_signature', 'vacancy_statement', 'appointment_reported',
                 'certificate_presentation', 'former_holder_styling', 'resignation_request', 'resignation_acceptance'}
# Claims that must never feed a holder observation: everything but the appointment decrees and in-office attestations.
NEVER_HOLDER = tuple(c for c, v in EVENTS.items() if v[1] not in HOLDER_KINDS)
# Tempting ends that are never used: the Government's resignation or laying down of powers, acceptance of a resignation,
# a former-holder styling and an acting designation; each names a day and none states that a Chairman's office ended.
TEMPTING_ENDS = (
    'ru_ukaz_281_government_dismissed_19980323', 'ru_ukaz_983_government_dismissed_19980823',
    'ru_duma_3961ii_primakov_government_dismissed_19990512', 'ru_ukaz_1012_government_dismissal_announced_19990809',
    'ru_gov_647r_government_surrenders_powers_20000507', 'ru_kremlin_kasyanov_styled_former_chairman_20040224',
    'ru_gov_608r_government_surrenders_powers_20040507', 'ru_kremlin_steno_putin_accepts_resignation_20070912',
    'ru_gov_680r_government_powers_laid_down_20080507', 'ru_gov_760r_government_powers_laid_down_20120507',
    'ru_pub_gov_875r_government_powers_laid_down_20180507', 'ru_pub_ukaz_14_government_resignation_announced_20200115',
    'ru_pub_gov_1121r_government_powers_laid_down_20240507')
# Dossier identifiers, keys and kinds replaced after the checks; none may appear in the packet or the new extracts.
STALE = ('ru_kremlin_premier_order_government_surrender_20040507', 'ru_ukaz_1146_government_resignation_accepted_19960809',
         'holder_name_normalized', 'until_status', 'first_fetched_via', 'secondary_responses', '"appointment_decree"',
         '"resignation_tendered"', '"acting_government_act_signature"', '"parliamentary_consent_vote"', 'The Fifth Congress',
         'representative to the European Communities', 'before the election of 14 March 2004', 'with 448 deputies (100%) taking part')
# Leads not imported (secondary, duplicate, off-role or live-template pages); none may appear among the packet's URLs.
LEAD_URL_MARKERS = ('nd=102052189', 'nd=102052191', 'nd=102016822', 'nd=102012917', 'nd=102012985', 'nd=102061314',
                    'nd=102085636', 'acts/bank/', 'government.ru', 'news/38168', 'news/30420', 'news/30421', 'news/30422',
                    'news/30505', 'news/30924', 'transcripts/22460', 'transcripts/22372', 'transcripts/22364', 'by-date',
                    'news/57431', 'news/57415', 'news/62584', 'news/73976', '0001202609070021', 'duma.gov.ru/news/',
                    'wikipedia', 'cntd.ru', 'a7date=')
# URL fragments that mark a page or file generated per request, an unresolved capture or a growing search; never allowed.
VOLATILE_URL = re.compile(r'(ysclid=|sessid=|PHPSESSID|[?&]cb=|nocache|token=|utm_|fbclid|yclid|form_build_id|DDoS|'
                          r'/web/\d{4}id_/|/web/\d{14}/)')
REPORT = research.RESEARCH / 'russia-heads-of-government-1991-2026-19.md'
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-19.md'
PRESIDENCY_HOLDERS = [
    ('ru_rsfsr_president', ['Борис Николаевич Ельцин']), ('ru_rsfsr_vice_president', ['Александр Владимирович Руцкой']),
    ('ru_president', ['Борис Николаевич Ельцин', PUTIN, PUTIN, MED, PUTIN, PUTIN, PUTIN])]


def government_invariants(russia, ussr):
    """Packet-level rules this test owns; raises AssertionError, KeyError or IndexError on any violation."""
    claims = {c['id']: c for s in russia['sources'] for c in s['claims']}
    owner = {c['id']: s['id'] for s in russia['sources'] for c in s['claims']}
    assert [e['id'] for e in russia['institutions'] if not e['id'].startswith('ru_duma_faction_')] == ['ru_rsfsr_presidency', INST]
    gov = next(e for e in russia['institutions'] if e['id'] == INST)
    assert (gov['name'], gov['kind'], gov['lifecycle']['status']) == (INST_NAME, 'executive_institution', 'unknown')
    assert (gov['lifecycle']['from'], gov['lifecycle']['until']) == (None, None)
    assert [r['id'] for r in gov['roles']] == [ROLE]
    role = gov['roles'][0]
    assert (role['title'], role['kind']) == (ROLE_TITLE, 'head_of_government')
    kinds = {}
    for packet in (russia, ussr):
        for group in ('organizations', 'institutions'):
            for entry in packet[group]:
                for r in entry['roles']:
                    kinds.setdefault(r['kind'], []).append(r['id'])
    # CLAUDE-C01-26 added the Union head of government in ussr.json (su_government_head), pinned in its own test; no other
    # head-of-government role exists in either packet.
    assert kinds['head_of_government'] == [ROLE, 'su_government_head'], 'no other head-of-government role'
    assert kinds['head_of_state'] == ['ru_president', 'su_president']
    # The presidency holders are unchanged and share nothing with this role.
    presidency = next(e for e in russia['institutions'] if e['id'] == 'ru_rsfsr_presidency')
    assert [(r['id'], [h['name'] for h in r['holder_claims']]) for r in presidency['roles']] == PRESIDENCY_HOLDERS
    assert all(h['until'] in (None, '1999-12-31') for r in presidency['roles'] for h in r['holder_claims'])
    ours = set(EVENTS)
    for r in presidency['roles']:
        cited = set(r['claim_ids']) | {c for h in r['holder_claims'] for c in h['claim_ids']}
        assert not cited & ours, r['id']
    assert not set(role['claim_ids']) & {c for r in presidency['roles'] for c in r['claim_ids']}
    assert set(role['sources']) & {s for r in presidency['roles'] for s in r['sources']} == {SHARED_SOURCE}
    # No other role in either packet cites a claim of this packet (no cross-role or cross-institution holder).
    for packet in (russia, ussr):
        for group in ('organizations', 'institutions'):
            for entry in packet[group]:
                for r in entry['roles']:
                    if r['id'] == ROLE:
                        continue
                    cited = set(r['claim_ids']) | {c for h in r['holder_claims'] if isinstance(h, dict) for c in h['claim_ids']}
                    assert not cited & ours, r['id']
    su = next(r for e in ussr['institutions'] for r in e['roles'] if r['id'] == 'su_president')
    assert [h['name'] for h in su['holder_claims']] == ['Mikhail Gorbachev', 'Mikhail Gorbachev']
    # Exact holders; each start or attested day rests on its own appointment decree, dated that day; no end anywhere.
    holders = role['holder_claims']
    assert [(h['name'], h['attested_on'], h['from'], h['until']) for h in holders] == HOLDERS
    assert [h['claim_ids'] for h in holders] == HOLDER_CLAIMS
    for h in holders:
        assert (h['attested_on'] is None) != (h['from'] is None), h['name']
        assert h['until'] is None, h['name']
        assert not set(h['claim_ids']) & set(NEVER_HOLDER), h['name']
        assert not {h['attested_on'], h['from'], h['until']} & NEVER_HOLDER_DATE, h['name']
        assert 'Исполняющий' not in h['name'] and 'Acting' not in h['name'], h['name']
        assert all(cid in role['claim_ids'] for cid in h['claim_ids']), h['name']
        expected = []
        for cid in h['claim_ids']:
            if owner[cid] not in expected:
                expected.append(owner[cid])
        assert h['sources'] == expected, h['name']
        if h['from']:
            assert STARTS[h['from']] == h['claim_ids'][0] and claims[STARTS[h['from']]]['attested_on'] == h['from'], h['name']
        else:
            assert ATTESTED[h['attested_on']] == h['claim_ids'][0], h['name']
            assert claims[ATTESTED[h['attested_on']]]['attested_on'] == h['attested_on'], h['name']
        for cid in h['claim_ids'][1:]:
            assert claims[cid]['attested_on'] > (h['from'] or h['attested_on']), h['name']
    assert [h['from'] for h in holders if h['from']] == list(STARTS)
    assert [h['attested_on'] for h in holders if h['attested_on']] == list(ATTESTED)
    # Every claim of this packet is on the role; body-level claims are also on the institution, and only those.
    assert set(role['claim_ids']) == ours
    assert set(gov['claim_ids']) == {c for c, v in EVENTS.items() if v[1] in BODY_KINDS}
    # Distinct dated events stay distinct: nomination or presentation, consent or approval, appointment.
    for nomination, consent, appointment in (
            ('ru_duma_steno_nomination_letter_presented_19960810', 'ru_gd_624ii_consent_chernomyrdin_19960810', 'ru_ukaz_1152_chernomyrdin_appointed_19960810'),
            ('ru_duma_steno_kiriyenko_third_submission_presented_19980424', 'ru_gd_2421ii_consent_kiriyenko_19980424', 'ru_ukaz_436_kiriyenko_appointed_19980424'),
            ('ru_kremlin_putin_nominates_kasyanov_20000510', 'ru_duma_363iii_consent_kasyanov_20000517', 'ru_ukaz_861_kasyanov_appointed_20000517'),
            ('ru_kremlin_42319_zubkov_nominated_20070912', 'ru_duma_5066_4_consent_zubkov_20070914', 'ru_ukaz_1202_zubkov_appointed_20070914'),
            ('ru_kremlin_6_putin_nominated_20080507', 'ru_duma_458_5_consent_putin_20080508', 'ru_ukaz_723_putin_appointed_20080508'),
            ('ru_kremlin_62586_mishustin_nominated_20200115', 'ru_pub_duma_7565_7_consent_mishustin_20200116', 'ru_pub_ukaz_17_mishustin_appointed_20200116'),
            ('ru_duma_steno_mishustin_submitted_20240509', 'ru_pub_duma_6061_8_approval_mishustin_20240510', 'ru_pub_ukaz_319_mishustin_appointed_20240510')):
        assert claims[nomination]['attested_on'] <= claims[consent]['attested_on'] <= claims[appointment]['attested_on'], appointment
        assert len({nomination, consent, appointment}) == 3
    assert claims['ru_duma_steno_mishustin_submitted_20240509']['attested_on'] == '2024-05-09'
    assert claims['ru_kremlin_42319_zubkov_nominated_20070912']['attested_on'] == '2007-09-12'
    for cid in TEMPTING_ENDS:
        assert claims[cid]['attested_on'] in NEVER_HOLDER_DATE, cid


class RussianHeadsOfGovernmentTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.raw = (research.ROOT / research.RESEARCH / 'russia.json').read_text(encoding='utf-8')
        cls.ussr_raw = (research.ROOT / research.RESEARCH / 'ussr.json').read_text(encoding='utf-8')
        cls.packet, cls.ussr = json.loads(cls.raw), json.loads(cls.ussr_raw)
        cls.sources = {s['id']: s for s in cls.packet['sources']}
        cls.claims = {c['id']: c for s in cls.packet['sources'] for c in s['claims']}
        cls.gov = next(e for e in cls.packet['institutions'] if e['id'] == INST)
        cls.role = cls.gov['roles'][0]
        cls.extracts = {sid: json.loads((research.ROOT / cls.sources[sid]['snapshot']['path']).read_text(encoding='utf-8'))
                        for sid in NEW_SOURCES + [SHARED_SOURCE]}
        cls.rows = {row['claim_id']: row for sid in NEW_SOURCES + [SHARED_SOURCE] for row in cls.extracts[sid]['rows']
                    if row['claim_id'] in EVENTS}
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
        self.assertEqual((len(NEW_SOURCES), len(self.new_claims), len(EVENTS)), (102, 151, 153))
        self.assertEqual([s['id'] for s in self.packet['sources'][68:]], NEW_SOURCES)
        self.assertEqual((len(ids['sources']), len(ids['claims']), len(ids['entries']), len(ids['roles'])), (170, 295, 21, 9))
        self.assertEqual(set(self.new_claims) | set(SHARED_1146), set(EVENTS))
        self.assertEqual([c['id'] for c in self.sources[SHARED_SOURCE]['claims']][1:], SHARED_1146)
        # New sources are appended in date order; every claim has one kind from the pinned vocabulary.
        dates = [self.sources[sid]['document_date'] for sid in NEW_SOURCES]
        self.assertEqual(dates, sorted(dates))
        vocabulary = HOLDER_KINDS | ACTING_KINDS | BODY_KINDS | NOMINATION_KINDS | PARLIAMENT_KINDS | CONTEXT_KINDS
        self.assertEqual({v[1] for v in EVENTS.values()}, vocabulary)
        self.assertEqual(len(vocabulary), sum(map(len, (HOLDER_KINDS, ACTING_KINDS, BODY_KINDS, NOMINATION_KINDS,
                                                         PARLIAMENT_KINDS, CONTEXT_KINDS))))
        holder_claims = {cid for ids_ in HOLDER_CLAIMS for cid in ids_}
        self.assertFalse(holder_claims & set(NEVER_HOLDER))
        self.assertEqual(holder_claims | set(NEVER_HOLDER), set(EVENTS))
        self.assertEqual(holder_claims, {c for c, v in EVENTS.items() if v[1] in HOLDER_KINDS})
        # Every new claim and source is cited by the role, in packet order, with decree 1146's two rows in date order.
        expected = []
        for sid in NEW_SOURCES:
            if SHARED_SOURCE not in expected and self.sources[sid]['document_date'] > '1996-08-09':
                expected.append(SHARED_SOURCE)
            expected.append(sid)
        self.assertEqual(self.role['sources'], expected)
        self.assertEqual(self.role['claim_ids'], [c for sid in expected for c in (
            SHARED_1146 if sid == SHARED_SOURCE else [x['id'] for x in self.sources[sid]['claims']])])
        observations = re.findall(r'^### (RU-GOV-\d\d)\b', self.report, re.M)
        self.assertEqual(observations, [f'RU-GOV-{n:02d}' for n in range(1, 11)])
        self.assertEqual({v[2] for v in EVENTS.values()}, {f'RU-GOV-{n:02d}' for n in range(1, 11)})
        extract_texts = {sid: (research.ROOT / self.sources[sid]['snapshot']['path']).read_text(encoding='utf-8') for sid in NEW_SOURCES}
        for stale in STALE:
            self.assertNotIn(stale, self.raw, stale)
            for sid, text in extract_texts.items():
                self.assertNotIn(stale, text, (sid, stale))

    def test_holders_are_exactly_as_intended(self):
        government_invariants(self.packet, self.ussr)
        for holder in self.role['holder_claims']:
            self.assertTrue(holder['note'] and holder['uncertainty'], holder['name'])
            for cid in holder['claim_ids']:
                row = self.rows[cid]
                self.assertEqual((row['holder_name'], row['role_id'], row['role_title']), (holder['name'], ROLE, ROLE_TITLE), cid)
                self.assertIn(row['event_kind'], HOLDER_KINDS, cid)
        # Kinds that could date a holder never appear on a claim that must not feed one.
        for cid in NEVER_HOLDER:
            self.assertNotIn(self.rows[cid]['event_kind'], HOLDER_KINDS, cid)
        for cid, (day, kind, obs) in EVENTS.items():
            row = self.rows[cid]
            if kind in ACTING_KINDS:
                self.assertEqual(row['role_title'], ACTING_TITLE, cid)
                self.assertIn('never a holder', self.claims[cid]['uncertainty'], cid)
            else:
                self.assertEqual(row['role_title'], ROLE_TITLE, cid)
            if kind in BODY_KINDS - {'dismissal_reference', 'resignation_proposed'}:
                self.assertIsNone(row['holder_name'], cid)
        # Rows whose source names nobody carry no holder: the body-level kinds and exactly these five context rows.
        nameless = {'ru_cpd_4063i_secret_ballot_results_approved_19921209', 'ru_cpd_4079i_chairman_selection_procedure_19921212',
                    'ru_ukaz_1569_chairman_post_vacant_19921215', 'ru_ukaz_287_acting_point_repealed_19980323',
                    'ru_kremlin_putin_statement_nomination_intention_20040224'}
        self.assertEqual({cid for cid, row in self.rows.items() if row['holder_name'] is None},
                         {cid for cid, v in EVENTS.items() if v[1] in BODY_KINDS - {'dismissal_reference', 'resignation_proposed'}} | nameless)
        # One canonical name per person (the C01-14 form for Putin and Medvedev); printed forms kept beside them.
        for row in self.rows.values():
            self.assertIn(row['holder_name'], NAMES | {None}, row['claim_id'])
            self.assertEqual('printed_name' in row, row['holder_name'] is not None, row['claim_id'])
            self.assertNotIn('name', row)
        president = next(r for e in self.packet['institutions'] for r in e['roles'] if r['id'] == 'ru_president')
        self.assertIn(PUTIN, {h['name'] for h in president['holder_claims']})
        self.assertIn(MED, {h['name'] for h in president['holder_claims']})
        self.assertEqual(self.rows['ru_pub_gov_res_1125_mishustin_signs_as_chairman_20260903']['printed_name'], 'М.Мишустин')
        self.assertEqual(self.rows['ru_ukaz_1052_putin_appointed_19990816']['printed_name'], 'Путина Владимира Владимировича (accusative)')
        self.assertEqual(self.rows['ru_ukaz_264_khristenko_acting_20040224']['holder_name'], KHR)

    def test_starts_and_ends_only_where_a_source_states_one(self):
        claims = self.claims
        for day, cid in STARTS.items():
            self.assertRegex(claims[cid]['text'], r'(in|into) force on (the day of )?(its )?sign', cid)
            self.assertRegex(claims[cid]['uncertainty'], r"(holder's|observation's) `?from", cid)
        for cid in NO_EFFECT_CLAUSE:
            self.assertRegex(claims[cid]['uncertainty'], r'(?i)(no effect clause|no entry-into-force clause)', cid)
            self.assertNotIn('in force', claims[cid]['text'], cid)
        # No holder has an end; every tempting end says why it is not one.
        self.assertEqual([h['until'] for h in self.role['holder_claims']], [None] * 15)
        for cid in TEMPTING_ENDS:
            self.assertRegex(claims[cid]['uncertainty'], r'(?i)(never an until|not an until|no holder until|no until|inferred end|not used as an until|never a holder boundary)', cid)
        fradkov = self.role['holder_claims'][8]
        self.assertIn('An integrator ruling is requested', fradkov['uncertainty'])
        self.assertIn('ru_kremlin_steno_putin_accepts_resignation_20070912 alone', fradkov['uncertainty'])
        medvedev = self.role['holder_claims'][12]
        self.assertIn('the same rule must be applied to every holder at once', medvedev['uncertainty'])
        for cid, (day, kind, obs) in EVENTS.items():
            if kind == 'nomination_presented':
                self.assertIn('never used to date a holder', claims[cid]['uncertainty'], cid)
        # Acting service and continued duties are claims only; the 1991-1992 arrangement is claims only.
        self.assertIn('Acting and continuing service are claims only, never holders', self.role['scope_note'])
        self.assertIn('procedure only, never a date', self.role['scope_note'])
        self.assertIn('No holder has an until', self.role['scope_note'])
        for cid in ('ru_ukaz_171_president_heads_government_19911106', 'ru_ukaz_172_president_heads_government_19911106'):
            self.assertIn('Claim only', claims[cid]['uncertainty'], cid)
        self.assertIn("until 14 December 1992", claims['ru_ukaz_1569_chairman_post_vacant_19921215']['text'])
        self.assertIn('not treated as a statement', claims['ru_ukaz_1569_chairman_post_vacant_19921215']['uncertainty'])
        # The coverage says what remains open.
        unresolved = self.gov['coverage']['unresolved']
        self.assertEqual([u.split(':')[0] for u in unresolved[:-1]],
                         ['RU-GOV-01', 'RU-GOV-02', 'RU-GOV-03', 'RU-GOV-04', 'RU-GOV-03 and 04', 'RU-GOV-05', 'RU-GOV-06',
                          'RU-GOV-07', 'RU-GOV-08 to 10'])
        self.assertIn('resolution 1125 of 3 September 2026', unresolved[-1])
        self.assertEqual(sum('CLAUDE-C01-19' in u for u in self.packet['coverage']['unresolved']), 1)

    def test_extracts_match_packet_claims_and_record_original_responses(self):
        for sid in NEW_SOURCES:
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual(extract['format'], 'spheres-c01-derived-factual-table/v1')
            self.assertEqual((extract['source_id'], extract['source_url']), (sid, source['url']))
            self.assertEqual(extract['scope_note'], source['scope_note'])
            self.assertEqual((extract['access_method'], extract['published_date'], extract['document_date']),
                             (source['access_method'], source['published_date'], source['document_date']))
            self.assertEqual((extract['accessed_date'], source['accessed_date']), ('2026-09-25', '2026-09-25'))
            self.assertFalse(extract['source_response_checked_in'])
            self.assertEqual((extract['source_response_bytes'], extract['source_response_sha256']), RESPONSES[sid], sid)
            self.assertIn('not checked into this repository', extract['provenance_note'])
            self.assertIn('derived factual extract', extract['provenance_note'])
            self.assertRegex(extract['stability_check'], r'This packet: re-downloaded at 2026-09-25T\d\d:\d\d:\d\dZ and again at 2026-09-25T')
            self.assertIn('No portrait permission or likeness approval', extract['rights_note'])
            self.assertIn('CLAUDE-C01-19 only', extract['bounded_scope'])
            self.assertTrue(source['source_type'].startswith('primary_'), sid)
            snapshot = source['snapshot']
            self.assertEqual(snapshot['kind'], 'derived_factual_extract')
            self.assertRegex(snapshot['path'], r'^docs/campaign-certification/C01/research/sources/russia-[a-z0-9-]+-\d{8}-facts\.json$')
            data = (research.ROOT / snapshot['path']).read_bytes()
            self.assertEqual((len(data), hashlib.sha256(data).hexdigest()), (snapshot['bytes'], snapshot['sha256']))
            self.assertNotEqual(snapshot['sha256'], extract['source_response_sha256'])
            self.assertTrue(data.endswith(b'}\n') and b'\r' not in data)
            self.assertEqual(data.decode('utf-8'), json.dumps(extract, indent=2, ensure_ascii=False) + '\n')
            got = [(k, extract[k]['bytes'], extract[k]['sha256']) for k in ('portal_card_response', 'card_response') if k in extract]
            got += [(k, r['bytes'], r['sha256']) for k in ('corroborating_responses',) for r in extract.get(k, [])]
            self.assertEqual(got, ATTACHED.get(sid, []), sid)
            self.assertNotIn('alternate_responses', extract)
            # Rows repeat the packet claims exactly, in order, keyed by claim_id; no row has a bare 'name' key.
            self.assertEqual([r['claim_id'] for r in extract['rows']], [c['id'] for c in source['claims']])
            for row, claim in zip(extract['rows'], source['claims']):
                self.assertEqual((row['text'], row['locator'], row['attested_on']), (claim['text'], claim['locator'], claim['attested_on']))
                self.assertEqual((row['observation_id'], row['role_id']), (INST, ROLE))
                self.assertNotIn('name', row)
        self.assertEqual({cid: (row['attested_on'], row['event_kind'], row['review_observation']) for cid, row in self.rows.items()}, EVENTS)
        for sid, stamp in ARCHIVED.items():
            source, extract = self.sources[sid], self.extracts[sid]
            url = urlsplit(source['url'])
            self.assertEqual(url.hostname, 'web.archive.org')
            self.assertEqual(url.scheme, 'https')
            self.assertTrue(url.path.startswith(f'/web/{stamp}id_/http'), sid)
            self.assertLess(stamp, '20260907')
            self.assertEqual(extract['source_response_url'], source['url'])
            self.assertEqual(extract['original_url'], source['original_url'])
            self.assertEqual(source['url'].split('id_/', 1)[1].replace(':80/', '/', 1), source['original_url'])
            self.assertEqual(extract['archive_capture_utc'].replace('-', '').replace(':', '').replace('T', '').rstrip('Z'), stamp)
            self.assertIn('Raw Internet Archive capture', extract['provenance_note'])
            self.assertIn('recorded identity is the uncompressed body', extract['provenance_note'])
            self.assertEqual(source['access_method'], 'internet_archive_raw_capture')
            if 'kremlin.ru' in source['original_url'] and '/copy/' not in source['original_url']:
                self.assertIn('Последнее обновление материала', extract['provenance_note'], sid)
        for sid, http in HTTP_READ.items():
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual(source['url'], http.replace('http://', 'https://', 1))
            self.assertEqual(extract['source_response_url'], http)
            self.assertIn('port 443', extract['provenance_note'])
            self.assertNotIn('original_url', source)
        self.assertEqual(set(ARCHIVED) | set(HTTP_READ), set(NEW_SOURCES))
        self.assertFalse(set(ARCHIVED) & set(HTTP_READ))
        for sid in LIVE_FOOTER:
            self.assertIn('1 January 2027', self.extracts[sid]['provenance_note'])
            self.assertEqual(self.sources[sid]['access_method'], 'official_site_html_downloaded_over_http')
        self.assertEqual({sid for sid in NEW_SOURCES if self.sources[sid]['access_method'] == 'official_site_html_downloaded_over_http'},
                         set(LIVE_FOOTER))
        # PDFs were rendered and viewed; the IPS text of the same act is the corroborating response.
        for sid in NEW_SOURCES:
            if self.sources[sid]['access_method'] == 'official_publication_portal_pdf_downloaded_over_http':
                self.assertTrue(self.extracts[sid]['visual_review']['pdf_pages_one_based'], sid)
                self.assertIn('rendered', self.extracts[sid]['visual_review']['method'], sid)
        self.assertEqual(self.sources['ru_pub_gov_res_1125_20260903']['published_date'], '2026-09-04')
        self.assertIn('Дата опубликования: 04.09.2026', self.extracts['ru_pub_gov_res_1125_20260903']['card_response']['note'])
        # The 1996 stenogram rows rest on the frozen capture, not on the C01-14 live page.
        self.assertEqual(self.sources['ru_duma_steno_ia_19960810']['original_url'], 'http://transcript.duma.gov.ru/node/2898/')
        self.assertEqual([c['id'] for c in self.sources['ru_duma_steno_19960810']['claims']], ['ru_duma_steno_inauguration_held_19960809'])

    def test_the_one_edited_existing_extract_is_bounded(self):
        source, extract = self.sources[SHARED_SOURCE], self.extracts[SHARED_SOURCE]
        rows = extract['rows']
        self.assertEqual([r['claim_id'] for r in rows], ['ru_ukaz_1146_signed_as_president_19960809'] + SHARED_1146)
        self.assertEqual((rows[0]['observation_id'], rows[0]['role_id']), ('ru_rsfsr_presidency', 'ru_president'))
        for row in rows[1:]:
            self.assertEqual((row['observation_id'], row['role_id'], row['holder_name'], row['role_title']), (INST, ROLE, None, ROLE_TITLE))
        self.assertEqual((extract['source_response_bytes'], extract['source_response_sha256']),
                         (24028, '82667085b973315be9d941ae90bfd429b9407b82eb126a14ebcf90531553228d'))
        self.assertIn('CLAUDE-C01-19 adds two rows', source['scope_note'])
        self.assertEqual(extract['scope_note'], source['scope_note'])
        self.assertIn('CLAUDE-C01-19 (the last two rows)', extract['bounded_scope'])
        self.assertIn('CLAUDE-C01-19: re-downloaded at 2026-09-25T', extract['stability_check'])
        data = (research.ROOT / source['snapshot']['path']).read_bytes()
        self.assertEqual((len(data), hashlib.sha256(data).hexdigest()), (source['snapshot']['bytes'], source['snapshot']['sha256']))
        self.assertEqual(data.decode('utf-8'), json.dumps(extract, indent=2, ensure_ascii=False) + '\n')
        self.assertEqual(source['accessed_date'], '2026-09-24')
        # Removing the two rows and the appended sentences restores the CLAUDE-C01-14 extract byte for byte.
        base = copy.deepcopy(extract)
        base['rows'] = rows[:1]
        base['scope_note'] = base['scope_note'].split(' CLAUDE-C01-19 adds', 1)[0]
        base['stability_check'] = base['stability_check'].split(' CLAUDE-C01-19: re-downloaded', 1)[0]
        base['bounded_scope'] = ('Presidency observations for CLAUDE-C01-14 only. Election calling, voting, result determination, '
                                 'declaration, correction and publication, oath, stated assumption of office, inauguration ceremony, '
                                 'resignation, acting service and outgoing-holder statements stay separate rows; holder boundaries '
                                 'are decided in the packet, not by any row. The C01-05 RSFSR roles and the USSR packet are outside '
                                 'this extract.')
        original = (json.dumps(base, indent=2, ensure_ascii=False) + '\n').encode('utf-8')
        self.assertEqual((len(original), hashlib.sha256(original).hexdigest()),
                         (4123, 'fb7eee63741d0676d52a43c4bc897b6024e92d71f0baccca0a3be6ec68e69ca8'))

    def test_secondary_leads_and_volatile_urls_stay_out_of_the_packet(self):
        urls = []
        for sid in NEW_SOURCES:
            extract = self.extracts[sid]
            urls += [self.sources[sid]['url'], extract['source_response_url']]
            for key in ('portal_card_response', 'card_response'):
                if key in extract:
                    urls.append(extract[key]['url'])
            urls += [r['url'] for r in extract.get('corroborating_responses', [])]
        for url in urls:
            self.assertIsNone(VOLATILE_URL.search(url), url)
            for marker in LEAD_URL_MARKERS:
                self.assertNotIn(marker, url)
            parts = urlsplit(url)
            # kremlin.ru and the Duma's search service appear only inside raw captures.
            if parts.hostname != 'web.archive.org' and ('kremlin.ru' in parts.hostname or 'api_search' in parts.path):
                self.fail(url)
            if 'list_itself' in (parts.query or ''):
                # Portal cards are single-document searches restricted to one day: never a growing number search.
                query = parse_qs(parts.query, keep_blank_values=True)
                self.assertEqual(query['a7type'], ['4'], url)
                self.assertEqual(query['a7from'], query['a7to'], url)
                self.assertIn('a8', query, url)
        api = [u for u in urls if 'api_search' in u]
        self.assertEqual(sorted(set(api)), ['https://web.archive.org/web/20240520233218id_/http://transcript.duma.gov.ru/api_search/?kodz=2125&kodvopr=2'])
        # The earlier packets cite duma.gov.ru news on factions; the markers are checked on this packet's own records.
        lowered = json.dumps([self.sources[sid] for sid in NEW_SOURCES] + [self.gov], ensure_ascii=False).lower()
        for marker in LEAD_URL_MARKERS:
            self.assertNotIn(marker.lower(), lowered, marker)
        leads, added = self.section('Leads not imported'), self.section('Sources added')
        for marker in ('nd=102052189', 'acts/bank/13000', 'news/30420', 'news/57431', '0001202609070021', 'duma.gov.ru/news/47525',
                       'duma.gov.ru/news/26901', 'duma.gov.ru/news/59261'):
            self.assertIn(marker, leads, marker)
            self.assertNotIn(marker, added, marker)

    def test_separation_from_the_presidency_and_the_ussr_packet(self):
        for sid in NEW_SOURCES:
            self.assertNotIn(f'"{sid}"', self.ussr_raw)
            self.assertTrue(sid.startswith('ru_'))
        for cid in EVENTS:
            self.assertNotIn(f'"{cid}"', self.ussr_raw)
        presidency = next(e for e in self.packet['institutions'] if e['id'] == 'ru_rsfsr_presidency')
        self.assertEqual((len(presidency['sources']), len(presidency['claim_ids'])), (8, 10))
        self.assertEqual(len(self.gov['claim_ids']), 44)
        self.assertEqual(self.gov['jurisdiction']['automatic_successor_mapping'], False)
        self.assertIn('not the USSR Cabinet of Ministers', self.gov['jurisdiction']['note'])
        self.assertEqual((self.gov['represented_party_ids'], self.gov['reconciled_organization_id']), ([], None))
        self.assertIn('Not merged with any ru_president holder', self.role['holder_claims'][5]['uncertainty'])
        self.assertIn('never used as an end', self.role['holder_claims'][10]['uncertainty'])

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

        def role(packet, role_id=ROLE):
            return next(r for e in packet['institutions'] for r in e['roles'] if r['id'] == role_id)

        def holder(packet, index, role_id=ROLE):
            return role(packet, role_id)['holder_claims'][index]

        validator_cases = [
            (lambda p: source(p, 'ru_ukaz_1052_19990816')['snapshot'].update(sha256='0' * 64), 'checksum mismatch'),
            (lambda p: source(p, 'ru_pub_gov_res_1125_20260903')['snapshot'].update(bytes=1), 'checksum mismatch'),
            (lambda p: source(p, SHARED_SOURCE)['snapshot'].update(sha256='f' * 64), 'checksum mismatch'),
            (lambda p: holder(p, 14).update(until='2026-09-08'), 'exceeds cutoff'),
            (lambda p: claim(p, 'ru_pub_gov_res_1125_mishustin_signs_as_chairman_20260903').update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: holder(p, 5).update(until='1999-08-15'), 'Reversed historical interval'),
            (lambda p: holder(p, 6)['claim_ids'].append('ru_ukaz_1052_putin_appointed_19990816'), 'cited source'),
            (lambda p: source(p, 'ru_ukaz_1567_19921214').update(url=HTTP_READ['ru_ukaz_1567_19921214']), 'Invalid public source URL'),
        ]
        for change, message in validator_cases:
            with self.subTest(message=message), self.assertRaisesRegex(ValueError, message):
                self.validate(mutated(change))

        def acting_holder(name, day, sid, cid):
            return lambda p: role(p)['holder_claims'].insert(0, {'name': name, 'attested_on': day, 'from': None, 'until': None,
                                                                 'sources': [sid], 'claim_ids': [cid]})

        invariant_cases = [
            ('successor start used as an end (Chernomyrdin 1996)', lambda p: holder(p, 1).update(until='1998-04-24')),
            ('successor start used as an end (Putin 1999)', lambda p: holder(p, 5).update(until='2000-05-17')),
            ('successor start used as an end (Medvedev 2018)', lambda p: holder(p, 12).update(until='2020-01-16')),
            ('reappointment used as an end (Fradkov 2004)', lambda p: holder(p, 7).update(until='2004-05-12')),
            ('dismissal used as an end (Kiriyenko)', lambda p: holder(p, 2).update(until='1998-08-23')),
            ('resignation decree used as an end (Medvedev 2018)', lambda p: (holder(p, 12).update(until='2020-01-15'),
                                                                            holder(p, 12)['claim_ids'].append('ru_pub_ukaz_14_government_resignation_announced_20200115'),
                                                                            holder(p, 12)['sources'].append('ru_pub_ukaz_14_20200115'))),
            ('acceptance of resignation used as an end (Fradkov 2007)', lambda p: holder(p, 8).update(until='2007-09-12')),
            ('laying down of powers used as an end (Zubkov)', lambda p: holder(p, 9).update(until='2008-05-07')),
            ('former-holder styling used as an end (Kasyanov)', lambda p: holder(p, 6).update(until='2004-02-24')),
            ('Duma statement used as an end (Primakov)', lambda p: holder(p, 3).update(until='1999-05-12')),
            ('latest attestation used as an end', lambda p: holder(p, 14).update(until='2026-09-03')),
            ('nomination date used as from (Kasyanov)', lambda p: holder(p, 6).update({'from': '2000-05-10'})),
            ('nomination date used as from (Mishustin 2024)', lambda p: holder(p, 14).update({'from': '2024-05-09'})),
            ('consent date used as from (Zubkov)', lambda p: holder(p, 9).update({'from': '2007-09-13'})),
            ('from set where the decree has no effect clause (Chernomyrdin 1992)', lambda p: holder(p, 0).update({'from': '1992-12-14', 'attested_on': None})),
            ('from set where the decree has no effect clause (Stepashin)', lambda p: holder(p, 4).update({'from': '1999-05-19', 'attested_on': None})),
            ('presentation date used as the attested day', lambda p: holder(p, 2).update(attested_on='1998-04-10')),
            ('acting service added as a holder (Gaidar 1992)', acting_holder(GAIDAR, '1992-06-15', 'ru_ukaz_633_19920615', 'ru_ukaz_633_gaidar_acting_chairman_19920615')),
            ('acting service added as a holder (Medvedev 2020)', acting_holder(MED, '2020-01-15', 'ru_pub_ukaz_14_20200115', 'ru_pub_ukaz_14_medvedev_acting_chairman_20200115')),
            ('acting service added as a holder (Zubkov 2012)', acting_holder(ZUB, '2012-05-07', 'ru_gov_rasp_760r_20120507', 'ru_gov_760r_zubkov_signs_as_acting_20120507')),
            ('President heading the Government added as a holder', acting_holder(YELTSIN, '1991-11-06', 'ru_rsfsr_ukaz_171_19911106', 'ru_ukaz_171_president_heads_government_19911106')),
            ('acting claim cited by a holder', lambda p: (holder(p, 5)['claim_ids'].append('ru_ukaz_1012_putin_acting_19990809'),
                                                           holder(p, 5)['sources'].append('ru_ukaz_1012_19990809'))),
            ('continuation claim cited by a holder', lambda p: (holder(p, 11)['claim_ids'].append('ru_pub_ukaz_202_government_to_continue_20180507'),
                                                                 holder(p, 11)['sources'].append('ru_pub_ukaz_202_20180507'))),
            ('consent cited by a holder', lambda p: (holder(p, 13)['claim_ids'].append('ru_pub_duma_7565_7_consent_mishustin_20200116'),
                                                      holder(p, 13)['sources'].append('ru_pub_duma_res_7565_7_gd_20200116'))),
            ('an until on any holder', lambda p: holder(p, 13).update(until='2024-05-10')),
            ('cross-role holder (ru_president holder on this role)', lambda p: role(p)['holder_claims'].insert(
                6, copy.deepcopy(holder(p, 1, 'ru_president')))),
            ('cross-role holder (a Chairman holder on ru_president)', lambda p: role(p, 'ru_president')['holder_claims'].append(
                copy.deepcopy(holder(p, 5)))),
            ('cross-role claim', lambda p: role(p, 'ru_president')['claim_ids'].append('ru_ukaz_1052_putin_appointed_19990816')),
            ('holder merged with the presidency (Putin 1999 extended)', lambda p: holder(p, 1, 'ru_president').update(
                claim_ids=holder(p, 1, 'ru_president')['claim_ids'] + ['ru_ukaz_1052_putin_appointed_19990816'])),
            ('second head-of-government role', lambda p: p['institutions'][0]['roles'][0].update(kind='head_of_government')),
            ('role removed', lambda p: next(e for e in p['institutions'] if e['id'] == INST)['roles'].pop()),
            ('institution removed', lambda p: p['institutions'].pop()),
            ('nomination re-dated to the sitting', lambda p: claim(p, 'ru_duma_steno_mishustin_submitted_20240509').update(attested_on='2024-05-10') or
             claim(p, 'ru_kremlin_42319_zubkov_nominated_20070912').update(attested_on='2007-09-14')),
            ('consent collapsed after the appointment', lambda p: claim(p, 'ru_duma_363iii_consent_kasyanov_20000517').update(attested_on='2000-05-18')),
            ('lifecycle given a start', lambda p: next(e for e in p['institutions'] if e['id'] == INST)['lifecycle'].update({'from': '1991-11-06'})),
        ]
        government_invariants(self.packet, self.ussr)
        for label, change in invariant_cases:
            with self.subTest(label=label), self.assertRaises((AssertionError, KeyError, IndexError, StopIteration)):
                government_invariants(mutated(change), self.ussr)
        # A cross-institution holder: a Chairman observation added to the USSR Presidency.
        ussr = copy.deepcopy(self.ussr)
        next(r for e in ussr['institutions'] for r in e['roles'] if r['id'] == 'su_president')['holder_claims'].append(
            copy.deepcopy(self.role['holder_claims'][5]))
        with self.assertRaises(AssertionError):
            government_invariants(self.packet, ussr)

    def test_report_and_handoff_are_ready_for_review_and_close_nothing(self):
        self.assertIn('ready_for_review', self.report)
        table = self.section('Outcome')
        decisions = {'01': 'Accepted', '02': 'Accepted in part', '03': 'Accepted in part', '04': 'Accepted in part',
                     '05': 'Accepted in part', '06': 'Accepted', '07': 'Accepted in part', '08': 'Accepted', '09': 'Accepted',
                     '10': 'Accepted'}
        for number, decision in decisions.items():
            row, = [line for line in table.splitlines() if line.startswith(f'| RU-GOV-{number} ')]
            self.assertIn(f'**{decision}:**', row)
        defects = self.section('Checker defects')
        rows = [line for line in defects.splitlines() if re.match(r'\| [ABC]\d+ ', line)]
        self.assertEqual([re.match(r'\| ([ABC]\d+) ', r).group(1) for r in rows],
                         [f'A{n}' for n in range(1, 11)] + [f'B{n}' for n in range(1, 16)] + [f'C{n}' for n in range(1, 12)])
        for row in rows:
            self.assertRegex(row, r'\*\*(Applied|Applied in part|Declined|Resolved by removal)\*\*')
        for marker in ('C01', 'C06', 'S23', 'WC1', 'CP1'):
            self.assertNotRegex(self.report, rf'\b{marker}\b[^.\n]*\bis (now )?complete\b')
        notes = self.section('Integration notes')
        for text in ('claude/c01-ru-14', '1d749e15', 'research-index.json', 'russia-ips-ukaz-1146-19960809-facts.json',
                     'test_russia_research_s10h.py', 'test_ussr_russia_transition_c01_05.py', 'test_russia_presidents_c01_14.py',
                     'test_campaign_census', '1 January 2027'):
            self.assertIn(text, notes)
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        for text in ('ready_for_review', 'russia-heads-of-government-1991-2026-19.md', 'claude/c01-ru-19', 'cb64061a',
                     'claude/c01-ru-14', 'test_russia_heads_of_government_c01_19.py', 'pending Codex acceptance',
                     'russia-ips-ukaz-1146-19960809-facts.json'):
            self.assertIn(text, handoff)
        index = research.build()
        country = next(p for p in index['countries'] if p['nation'] == 'Russia')
        self.assertFalse(country['country_census_complete'])
        self.assertIsNone(country['unrepresented_organization_count'])
        self.assertEqual((country['role_observations'], country['source_claims'], country['mapping_pending']), (9, 295, 21))
        self.assertEqual({w['status'] for w in index['work_orders'] if w['nation'] == 'Russia'}, {'open'})
        self.assertFalse(index['c01_complete'])


if __name__ == '__main__':
    unittest.main()
