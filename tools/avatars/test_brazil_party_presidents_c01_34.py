"""CLAUDE-C01-34: the national presidencies of the PDT and of the PMDB/MDB are party offices on the PDT and MDB
funding observations, kept apart from the Presidency of the Republic and from each other; the PFL/DEM national
presidency is recorded as claims only. Conventions, elections, the assumption of the office, leaves, acting service,
deaths, renamings, the fusion of the Democratas into União Brasil and in-office attestations stay separate claims; a
holder has a start or an end only where a party record states the day, and acting or interim service is never a
holder."""
import copy
import hashlib
import json
import re
import unittest
from urllib.parse import urlsplit

import campaign_research as research
import test_brazil_pt_presidents_c01_22 as pt
import test_brazil_vice_presidents_c01_17 as vp

MDB_ORG = 'br_tse_fefc_2024_party_01'
PDT_ORG = 'br_tse_fefc_2024_party_02'
PT_ORG = pt.ORG_ID
UNIAO_ORG = 'br_tse_fefc_2024_party_27'
MDB = 'br_mdb_president'
PDT = 'br_pdt_president'
ROLE_ORG = {MDB: MDB_ORG, PDT: PDT_ORG}
ORG_LABEL = {MDB_ORG: ('MDB', '0613111-56.2024.6.00.0000'), PDT_ORG: ('PDT', '0613177-36.2024.6.00.0000'),
             UNIAO_ORG: ('UNIÃO', '0613083-88.2024.6.00.0000')}
# The role, acting and PFL/DEM titles, by the one-letter codes used in EVENTS.
TITLES = {
    'P': 'Presidente Nacional do Partido Democrático Trabalhista',
    'PA': 'Presidente Nacional do Partido Democrático Trabalhista em exercício ou interino',
    'M': 'Presidente Nacional do PMDB / MDB (Movimento Democrático Brasileiro)',
    'MA': 'Presidente Nacional do PMDB / MDB em exercício ou interino',
    'F': 'Presidente Nacional do Partido da Frente Liberal (PFL) / Democratas (DEM) (claims only, no role)',
}
ROLE_TITLE = {PDT: TITLES['P'], MDB: TITLES['M']}
ROLE_CODES = {PDT: {'P', 'PA'}, MDB: {'M', 'MA'}}
PREFIX = {PDT: 'br_pdt_', MDB: 'br_mdb_'}
PFL_PREFIX = 'br_pfl_'
# The packet's sources before this packet: the S10f intake (5), CLAUDE-C01-10 (48), CLAUDE-C01-17 (11) and
# CLAUDE-C01-22 (108).
EARLIER_SOURCE_COUNT = 172

# New sources, in packet order, with the original response identity recorded in each extract: (bytes, sha256).
RESPONSES = {
    "br_pdt_legalidade_interview_199611":
        (18394, "fb0bf4fcb0ed1c0635364bf42e42a19898fcf350452e9e3832c8bd40a9e21f15"),
    "br_pdt_home_19970411":
        (25433, "03f0b964fee4637ae4cb3cc9350f153cc391d9d6f45f854829a06c2e0204166a"),
    "br_pdt_marcha_speech_19990826":
        (18015, "5e4e7c6f57d5841362c4e8d3730684700d9fac9c03018eeeb3a8e756d7f54d4b"),
    "br_pdt_tijolaco_35_2004":
        (25195, "cc77652ee528d524a0f4aff4df7f4f337c8306c2a26ada286300fcc2477c0df0"),
    "br_pdt_salario_minimo_20040602":
        (13833, "e3340fc33b4aca1ecb402aa9456c5c69767352b42929294af1d0ca22a3c5dccc"),
    "br_pdt_direcao_nacional_capt20040609":
        (14611, "709efb8dabd73c7467950275ba702c6e88c9434b15d7579191853dc0ecc68585"),
    "br_pdt_home_20040621":
        (65862, "6434552e5529bff3ac6919dcc884a19eff233b06a56710e311aee39fbebe5c3e"),
    "br_pdt_curitiba_perda_20040622":
        (15909, "1b6b81bdd8217fe780edabdcad2213fbe72e397bfaf534523b009b84b4c75dd3"),
    "br_pdt_home_20040706":
        (46336, "7430e33a5f63f88bde80b56b64fc318c6c6feb7c392641cbd4a84cb57ebce046"),
    "br_pdt_direcao_nacional_capt20040722":
        (15559, "1f29d9dc56b12f36992413be08074e24b3bf5f6ed9e5de762123a356b11a6f74"),
    "br_pdt_perfil_lupi_capt20041011":
        (19160, "4a6e4c43bc6750f43bcda5cba3a35d31c0f031cffe00fe15b8f9a2a95c280c68"),
    "br_pdt_edital_convencao_20050228":
        (20965, "b3419d17a307dde665c447e93683886a976d395fd3f760580461acaf74017db4"),
    "br_pdt_convencao_reelege_20050321":
        (44214, "0813cd63c1108142c232dbf969ec833bf139fe92f9543dc1f52b5de84482d22a"),
    "br_pdt_edital_convencao_20070209":
        (20067, "4c470275a725b531edfbdc0e09a00d69da449e8fc3f561fe7f712a2aad3cbca9"),
    "br_pdt_convencao_reelege_20090306":
        (72613, "e43e21986754af7d3666473dd19d038205c4a49736ef3d2e161905fefe642a52"),
    "br_pdt_tse_sgip_cen_2017_2019":
        (14992, "39ac6e0ca1c5216f4481eb60254a0c5b60e8249bd1525fee6d7cbbb7cd71572d"),
    "br_pdt_xxiii_convencao_20170318":
        (63558, "853b9be30199ba0c5c65ada3116e1e41fbb8c54bdc1e447cb6534b1f023e70ce"),
    "br_pdt_historia_convencoes_20190315":
        (101392, "a23cf8f995f9e7d86aa78721fbeb4bb9afdc170cdc65526aaaf26268c49d3e79"),
    "br_pdt_xxv_convencao_20190318":
        (73307, "f6960ed1e4d8a4e8c5b10d199ce02fe91d63ac195a2769d84fe832e897dd03a2"),
    "br_pdt_tse_sgip_cen_2019_2022":
        (15692, "cde06d860b4f625fb7bf2690bf68d0fa6bfb54cdb442e80117987d0b89b21c7d"),
    "br_pdt_convencao_hibrida_20211221":
        (66378, "586d61f82f732bee41f427f06d8d48e7dfd78846435dd9025fe3566ef4eaf905"),
    "br_pdt_convencao_virtual_20220110":
        (69032, "c41343a7553dfd8f452adee7a5a64ff318fd2a3b6e1748ec5fdd0a32946c2fcb"),
    "br_pdt_discurso_lupi_20220121":
        (76754, "0d13d37f8de19f8a441f2de316c064d5df03ecf3603f969b3de223d5303a51d2"),
    "br_pdt_figueiredo_exercicio_20230202":
        (88104, "8fb843214e35c9873cabb6732e28dd94c8b7dc9e5142fff0a4c46e2fab0d4145"),
    "br_pdt_lupi_retorna_20250521":
        (73756, "f6269943fc95c3c14fd14deed647e41195b245dd345499923d96a98dc474ef80"),
    "br_pdt_comite_fortaleza_20260819":
        (19286, "62e503d4ac859ccaeca86a1a77da3e0904b47ca4de8b4cfdbc5585ad2b87b690"),
    "br_pdt_eleicoes_2026_20260904":
        (24187, "99e5090f3968db00f07018a04e17ede002253921ded13cfe1561f9bb720c2d48"),
    "br_mdb_jader_presidente_page_1999":
        (1976, "969e0f67bf42b4e12f7e955f05fa23ea5b6d0d9cd5d9732ecaa387761507efd3"),
    "br_mdb_temer_vota_clt_200112":
        (10603, "30ec40a39f7a413e3e76861b04695cf8e26526f6c6bfe8a4f1da7da8da595f82"),
    "br_mdb_nota_oficial_roriz_20030702":
        (12317, "f2110d78153bf94fcef70aca6c10c4f3f27fc2fa5a696526054323e886563a48"),
    "br_mdb_executiva_roster_capt2009":
        (26330, "7155be74983beae708f39769c57f2341ac950c3dd0dddfb36b6841560a934e98"),
    "br_mdb_iris_interina_20090310":
        (47549, "c7a8b570f147e4976a9bd31b8f7e0f08ebf6b62ca9ec568bf383cd88e4c686d0"),
    "br_mdb_nota_temer_licenciado_20100121":
        (45487, "f04896ba6d291ee985cee0c4fc0072d032ce989ed7b70d2bfa0fc17972200603"),
    "br_mdb_executiva_roster_capt2011":
        (26833, "3c34c2cf63ec1a410389378e63e87787a69340525bd97bc748476134b745beda"),
    "br_mdb_convencao_2013_20130302":
        (24869, "045d5b38dbf90c939d034b1f212710c800b56681f493ee34f366c4257d61e912"),
    "br_mdb_tse_sgip_cen_2013_2019":
        (18945, "f6a0c4cacda5f5ce2e5021c74a923880eb6f2a9d270b2fb80bac1aed4c17b17d"),
    "br_mdb_raupp_presidente_nacional_20140514":
        (29191, "20b987ff614df845a6f2758977dfac8e8afe6313fd80e45dff9f893a4cc912e1"),
    "br_mdb_convencao_2016_20160312":
        (26811, "8df431b56af013f55fc50a0f463675d37c3e8a289d0672cd9e959dc1b38f314a"),
    "br_mdb_rompe_alianca_20160329":
        (26097, "02b1d59bd0269a48d87910640a74004cc758275848dcbc428059d28f26f9c85f"),
    "br_mdb_comunicado_licenca_temer_20160405":
        (22944, "bece8f741b6da2926d39f81ef63df7a6c87999e3310e6e3aeed5f269db6ba71d"),
    "br_mdb_juca_em_exercicio_20160407":
        (23911, "48664939a6514e0088b243f2d985c9f2c636d27c86c51b708a4ec83d3953d510"),
    "br_mdb_edital_convencao_20171123":
        (23935, "d835f7606e7b3797286e44681b108e0fb6b861dcf0e2a4d0124b9153f3bfa9f3"),
    "br_mdb_pmdb_muda_sigla_20171219":
        (26785, "ab5b523acbdf71dc143e32833c13edc9ed80633cd0c903576cf1cd586469dca0"),
    "br_mdb_adocao_sigla_tse_20180201":
        (19879, "ac7c03e352f54699dc45442c73a54dabe62e64eede810c03da12f2ec693e4ce6"),
    "br_mdb_prorroga_mandatos_20180222":
        (19612, "6e841c6667e5c3848ed51305a58a6ab43918e3c8c0aeea09a732292f03432df9"),
    "br_mdb_tse_name_change_20180515":
        (90320, "8489553b141275ee5d0cbc62239b41adba9d1f402ff9668c34eb524bca8375d9"),
    "br_mdb_executiva_roster_capt201901":
        (22963, "4b2e254c640213684a22209b058f1341d420ca420c2c73fc86e76574d75c04d3"),
    "br_mdb_presidentes_lista_capt201901":
        (20891, "dc42d8070c8cf504ca59d9775dd2da641da2e45c8ea3702eef04b36004148183"),
    "br_mdb_juca_juventude_20190617":
        (21843, "d4f1d3206c6701ab022a72bda52035712fc5d707ce140e9be2fd44e38cbef697"),
    "br_mdb_eleicao_baleia_20191007":
        (57402, "2df1129a16479263b8b668d0440a20a64d28e3ec0ca29f4fec177949b2af6953"),
    "br_mdb_tse_sgip_cen_2019_2023":
        (21597, "a2a977c86c8967127f78d3ac0a29d01b73579268d7f9e2c00d87350c06945697"),
    "br_mdb_baleia_reune_executiva_20191017":
        (46414, "b865cd05d330adc89b15e3969ab863c63c2752e92f42434b6de379a247c97eb6"),
    "br_mdb_baleia_prorrogacao_20210223":
        (25530, "e8883937863e3c42e0dcbb6da44196f7289ecf55945b6b1265f421df2acadb5f"),
    "br_mdb_convencao_20231005":
        (43330, "a8c259541a6062e40d17cb5217513eef27855dccdfe63ba4e3b4fd620a364e1e"),
    "br_mdb_ata_diretorio_20231005":
        (232963, "11e40f63775353561791c7b908aebe12eea72872cc46c49600b1d132987bef00"),
    "br_mdb_ata_executiva_20250716":
        (246382, "34de51f9758a6d442539783b90ebad5ed12831d2368ed848f9978b2b2a8261cb"),
    "br_mdb_executiva_convocada_df_20260608":
        (27805, "111b30b564711c2a166e2e9c0e5b83b2dd1fce465507769bbb6c38dcc9960809"),
    "br_mdb_resolucao_blinda_20260615":
        (31429, "6bfcfc3908787472a743b69432050ec394e319d485c6f28c90796c3ae2ec88e2"),
    "br_pfl_site_executiva_nacional_1999":
        (3808, "2383d5710185953bf65dc3e7ce65a59dea55c8b04756962c58fa614b0f0b1e79"),
    "br_pfl_agencia_bornhausen_20030307":
        (11708, "0937d44119f32e10e3b0830edea7991ff38ef81289794533126e8364a19b0327"),
    "br_pfl_convencao_2007_edital_20070313":
        (16102, "58a436876abbb399adaee310cb336efbc45968299cb221807690af14122476ce"),
    "br_pfl_site_democratas_transformation_2007":
        (2298, "d5d0f6cfa1a40459f4e425a147b5119d5c6157dd971f1afe59d480dbd5bc67ef"),
    "br_pfl_dem_resolucao_006_20070329":
        (42365, "8429f4266587d57b37a26d8dbb7a9928543b53f0a3aa1490237c1b85231a328b"),
    "br_pfl_dem_resolucao_112_20110215":
        (46200, "458aa24579577c57c6e8c81b0a3ab85e5d44fbc0497cd309b50b596d10156edc"),
    "br_pfl_dem_convencao_20110314":
        (42578, "786f897a9181a38f9f847af336fb84020e8b362b028d6fe81f63562e94cd2b25"),
    "br_pfl_dem_agripino_eleito_20110316":
        (21728, "68cc2f883740ea611f2ec1ed3e3f0db6cf9a3b30f4d4b6373408813505680101"),
    "br_pfl_dem_resolucao_113_20110324":
        (44631, "487a0eace0526e420fa8bea86338716fb71a16ceb5a4dba42e84ffb191b71df6"),
    "br_pfl_tse_sgip_dem_cen_2015_2018":
        (23088, "c88585aee6e866a7ec1fe2f7bd1952ba016a8931bc0891521164bacfb701fd10"),
    "br_pfl_dem_cen_refundacao_2018":
        (24687, "28d6e0826ef3731adbeb3631478675eb387656a38d76ea07e23a56f7ef821efc"),
    "br_pfl_tse_sgip_dem_cen_2018_2019":
        (24059, "ae9ce0414b83ca5b87523757058f8eea87ef687f60fb2c571d973a8d2495c079"),
    "br_pfl_tse_sgip_dem_cen_2019_2022":
        (24913, "d22590506835f02a7d3cedcc97ec755227ae623627a6af11df467497014bacbf"),
    "br_pfl_dem_convencao_fusao_20211005":
        (61005, "6b48db86f1f4116d38812f71f4f563783be43c8d9252677a150b500cf44428db"),
    "br_pfl_dem_informe_acm_neto_20220112":
        (58967, "b76c74734356dfbfd093bedccaddeeb1bdd97662f4b34176f0ed128465e92ac1"),
    "br_pfl_dem_homologacao_uniao_20220208":
        (60279, "744c26ceb80930b1e08db38cf3c18896745ab984d8c67c540c2e78ef440f14af"),
    "br_pfl_tse_sgip_uniao_cen_2022_2024":
        (22515, "1e5d34f13d653eef16d48c02ed9df27055e1096f7252a33657c16e2cd774dfa3"),
    "br_pfl_tse_uniao_registro_20220208":
        (93606, "0ebc6d52bca747d6f1bb570dd61a05e8b4adc74072eaa5b48871afc6772914fe"),
}
# Capture timestamp of each raw id_ capture (all before the 7 September 2026 cutoff).
ARCHIVED = {
    "br_pdt_legalidade_interview_199611": "19970630073837",
    "br_pdt_home_19970411": "19970414062134",
    "br_pdt_marcha_speech_19990826": "20000116085821",
    "br_pdt_tijolaco_35_2004": "20040615071610",
    "br_pdt_salario_minimo_20040602": "20040612091805",
    "br_pdt_direcao_nacional_capt20040609": "20040609183021",
    "br_pdt_home_20040621": "20040622095219",
    "br_pdt_curitiba_perda_20040622": "20040818202640",
    "br_pdt_home_20040706": "20040707083829",
    "br_pdt_direcao_nacional_capt20040722": "20040722065735",
    "br_pdt_perfil_lupi_capt20041011": "20041011122547",
    "br_pdt_edital_convencao_20050228": "20050305064221",
    "br_pdt_convencao_reelege_20050321": "20050927215635",
    "br_pdt_edital_convencao_20070209": "20070308093648",
    "br_pdt_convencao_reelege_20090306": "20210513004503",
    "br_pdt_xxiii_convencao_20170318": "20170318211225",
    "br_pdt_historia_convencoes_20190315": "20190319020710",
    "br_pdt_xxv_convencao_20190318": "20190320060546",
    "br_pdt_convencao_hibrida_20211221": "20220120040443",
    "br_pdt_convencao_virtual_20220110": "20220120052812",
    "br_pdt_discurso_lupi_20220121": "20220122061423",
    "br_pdt_figueiredo_exercicio_20230202": "20230202164642",
    "br_pdt_lupi_retorna_20250521": "20250521210115",
    "br_pdt_comite_fortaleza_20260819": "20260819171758",
    "br_pdt_eleicoes_2026_20260904": "20260905041426",
    "br_mdb_jader_presidente_page_1999": "19990209051703",
    "br_mdb_temer_vota_clt_200112": "20020111051010",
    "br_mdb_nota_oficial_roriz_20030702": "20030814050244",
    "br_mdb_executiva_roster_capt2009": "20091108153056",
    "br_mdb_iris_interina_20090310": "20101012103809",
    "br_mdb_nota_temer_licenciado_20100121": "20101010032018",
    "br_mdb_executiva_roster_capt2011": "20110521133404",
    "br_mdb_convencao_2013_20130302": "20130305171905",
    "br_mdb_raupp_presidente_nacional_20140514": "20140528172803",
    "br_mdb_convencao_2016_20160312": "20160314231847",
    "br_mdb_rompe_alianca_20160329": "20160409013308",
    "br_mdb_comunicado_licenca_temer_20160405": "20160413120030",
    "br_mdb_juca_em_exercicio_20160407": "20160421191806",
    "br_mdb_edital_convencao_20171123": "20171230052402",
    "br_mdb_pmdb_muda_sigla_20171219": "20171222052258",
    "br_mdb_adocao_sigla_tse_20180201": "20190107050031",
    "br_mdb_prorroga_mandatos_20180222": "20190106054630",
    "br_mdb_tse_name_change_20180515": "20180519032632",
    "br_mdb_executiva_roster_capt201901": "20190105014041",
    "br_mdb_presidentes_lista_capt201901": "20190105045549",
    "br_mdb_juca_juventude_20190617": "20190619212148",
    "br_mdb_eleicao_baleia_20191007": "20221101093640",
    "br_mdb_baleia_reune_executiva_20191017": "20221102170525",
    "br_mdb_baleia_prorrogacao_20210223": "20210223193830",
    "br_mdb_convencao_20231005": "20231014034532",
    "br_mdb_ata_diretorio_20231005": "20240301071321",
    "br_mdb_ata_executiva_20250716": "20250915073142",
    "br_mdb_executiva_convocada_df_20260608": "20260618071709",
    "br_mdb_resolucao_blinda_20260615": "20260618071709",
    "br_pfl_site_executiva_nacional_1999": "20001017171549",
    "br_pfl_agencia_bornhausen_20030307": "20030313120640",
    "br_pfl_convencao_2007_edital_20070313": "20070317095802",
    "br_pfl_site_democratas_transformation_2007": "20070329002404",
    "br_pfl_dem_resolucao_006_20070329": "20110825144154",
    "br_pfl_dem_resolucao_112_20110215": "20111007170601",
    "br_pfl_dem_convencao_20110314": "20110318015626",
    "br_pfl_dem_agripino_eleito_20110316": "20190121161430",
    "br_pfl_dem_resolucao_113_20110324": "20111007170605",
    "br_pfl_dem_cen_refundacao_2018": "20181016194449",
    "br_pfl_dem_convencao_fusao_20211005": "20211027093533",
    "br_pfl_dem_informe_acm_neto_20220112": "20220112155245",
    "br_pfl_dem_homologacao_uniao_20220208": "20220209035246",
    "br_pfl_tse_uniao_registro_20220208": "20230326221359",
}
# Captures the archive serves content-encoded even to an identity request: (encoding, decoded bytes, decoded sha256).
ENCODED = {
    "br_pdt_comite_fortaleza_20260819":
        ("zstd", 81758, "ae6b36dedd06d3354722464585921d939757399e7412aca454dc4744e5888758"),
    "br_pdt_eleicoes_2026_20260904":
        ("zstd", 95681, "b836ef065993754d305254ff3b18eb23c158d39d107f4df1919f5c98ae2b43c4"),
    "br_mdb_executiva_convocada_df_20260608":
        ("zstd", 126473, "76e523b782094669e858f2dfad5891ab11d5820db09caa7235382f561bac97e9"),
    "br_mdb_resolucao_blinda_20260615":
        ("zstd", 136700, "efa57533d6c3e5eba5f58fb124139a7f4ae48592e92d574f243bbcf49e3f8c5e"),
}
# PDF pages read (text layer, and page 1 of the 2023 minutes rendered for its image table).
PDF_PAGES = {
    "br_mdb_ata_diretorio_20231005": [1, 2, 3],
    "br_mdb_ata_executiva_20250716": [1, 2, 3, 4, 5, 6],
}
# Closed SGIP organs recorded (organ id by source).
SGIP_ORGANS = {
    "br_pdt_tse_sgip_cen_2017_2019": "70996",
    "br_pdt_tse_sgip_cen_2019_2022": "269142",
    "br_mdb_tse_sgip_cen_2013_2019": "70945",
    "br_mdb_tse_sgip_cen_2019_2023": "291052",
    "br_pfl_tse_sgip_dem_cen_2015_2018": "70981",
    "br_pfl_tse_sgip_dem_cen_2018_2019": "248141",
    "br_pfl_tse_sgip_dem_cen_2019_2022": "275827",
    "br_pfl_tse_sgip_uniao_cen_2022_2024": "405173",
}
# Every new claim, in packet order: (review observation, attested_on, event_kind, holder_name, title code; None for claims naming no holder).
EVENTS = {
    "br_pdt_brizola_styled_presidente_nacional_199611":
        ("PDT-PRES-01", None, "in_office_period_attestation", "Leonel Brizola", "P"),
    "br_pdt_brizola_presidente_nacional_home_19970411":
        ("PDT-PRES-01", "1997-04-11", "in_office_attestation", "Leonel Brizola", "P"),
    "br_pdt_brizola_styled_presidente_do_pdt_19990826":
        ("PDT-PRES-01", "1999-08-26", "in_office_attestation", "Leonel Brizola", "P"),
    "br_pdt_brizola_signs_tijolaco_35_as_presidente_nacional":
        ("PDT-PRES-01", None, "in_office_period_attestation", "Leonel Brizola", "P"),
    "br_pdt_brizola_meets_caucuses_as_presidente_nacional_20040602":
        ("PDT-PRES-01", "2004-06-02", "in_office_attestation", "Leonel Brizola", "P"),
    "br_pdt_roster_brizola_presidente_lupi_first_vice_capt20040609":
        ("PDT-PRES-01", None, "in_office_period_attestation", "Leonel Brizola", "P"),
    "br_pdt_brizola_dies_as_presidente_nacional_20040621":
        ("PDT-PRES-01", "2004-06-21", "death_in_office_stated", "Leonel Brizola", "P"),
    "br_pdt_curitiba_reports_death_of_presidente_nacional":
        ("PDT-PRES-01", "2004-06-21", "death_reported", "Leonel Brizola", "P"),
    "br_pdt_lupi_presidente_nacional_home_20040706":
        ("PDT-PRES-02", "2004-07-06", "styled_president_during_interim_period", "Carlos Lupi", "P"),
    "br_pdt_home_links_executive_note_after_succession_20040706":
        ("PDT-PRES-02", None, "executive_note_reference", None, "P"),
    "br_pdt_roster_lupi_presidente_capt20040722":
        ("PDT-PRES-02", None, "styled_president_during_interim_period", "Carlos Lupi", "P"),
    "br_pdt_lupi_assumes_automatically_on_brizola_death":
        ("PDT-PRES-02", None, "stated_assumption_day_not_printed", "Carlos Lupi", "P"),
    "br_pdt_lupi_signs_convocation_as_presidente_nacional_20050228":
        ("PDT-PRES-02", "2005-02-28", "styled_president_during_interim_period", "Carlos Lupi", "P"),
    "br_pdt_convention_convoked_for_20050321_20050228":
        ("PDT-PRES-03", "2005-02-28", "convention_called_prospective", None, "P"),
    "br_pdt_convention_held_20050321":
        ("PDT-PRES-03", "2005-03-21", "convention_held", None, "P"),
    "br_pdt_lupi_styled_atual_presidente_on_convention_day_20050321":
        ("PDT-PRES-03", "2005-03-21", "styled_on_election_day", "Carlos Lupi", "P"),
    "br_pdt_lupi_reelected_convention_20050321":
        ("PDT-PRES-03", "2005-03-21", "reelection", "Carlos Lupi", "P"),
    "br_pdt_lupi_signs_resolution_001_07_as_presidente_nacional_20070209":
        ("PDT-PRES-03", "2007-02-09", "in_office_attestation", "Carlos Lupi", "P"),
    "br_pdt_convention_convoked_for_20070309":
        ("PDT-PRES-03", None, "convention_called_prospective", None, "P"),
    "br_pdt_lupi_reelected_while_licenciado_20090306":
        ("PDT-PRES-03", "2009-03-06", "reelection_while_on_leave", "Carlos Lupi", "P"),
    "br_pdt_vieira_da_cunha_keeps_command_20090306":
        ("PDT-PRES-03", "2009-03-06", "acting_service", "Vieira da Cunha", "PA"),
    "br_pdt_tse_sgip_lupi_registered_exercise_20170318_20190318":
        ("PDT-PRES-03", None, "registry_exercise_period", "Carlos Lupi", "P"),
    "br_pdt_lupi_reconducted_xxiii_convention_20170318":
        ("PDT-PRES-03", "2017-03-18", "reelection", "Carlos Lupi", "P"),
    "br_pdt_history_v_convention_reconducts_brizola_1992":
        ("PDT-PRES-01", None, "retrospective_statement", "Leonel Brizola", "P"),
    "br_pdt_history_ix_convention_reconducts_brizola_1999":
        ("PDT-PRES-01", None, "retrospective_statement", "Leonel Brizola", "P"),
    "br_pdt_history_lupi_assumed_interim_executive_meeting_2004":
        ("PDT-PRES-02", None, "retrospective_statement", "Carlos Lupi", "P"),
    "br_pdt_history_xvi_convention_reelects_lupi_2007":
        ("PDT-PRES-03", None, "retrospective_statement", "Carlos Lupi", "P"),
    "br_pdt_history_lupi_leave_to_vieira_da_cunha_2008":
        ("PDT-PRES-03", None, "retrospective_statement", "Carlos Lupi", "P"),
    "br_pdt_lupi_reconducted_xxv_convention_20190318":
        ("PDT-PRES-03", "2019-03-18", "reelection", "Carlos Lupi", "P"),
    "br_pdt_tse_sgip_lupi_registered_exercise_20190318_20220121":
        ("PDT-PRES-03", None, "registry_exercise_period", "Carlos Lupi", "P"),
    "br_pdt_lupi_signs_normative_resolution_003_2021_20211221":
        ("PDT-PRES-03", "2021-12-21", "in_office_attestation", "Carlos Lupi", "P"),
    "br_pdt_convention_convoked_for_20220121_20211221":
        ("PDT-PRES-03", "2021-12-21", "convention_called_prospective", None, "P"),
    "br_pdt_lupi_signs_communique_as_presidente_nacional_20220110":
        ("PDT-PRES-03", "2022-01-10", "in_office_continuation_attestation", "Carlos Lupi", "P"),
    "br_pdt_convention_speech_published_20220121":
        ("PDT-PRES-03", "2022-01-21", "convention_speech_published", "Carlos Lupi", "P"),
    "br_pdt_lupi_takes_leave_of_presidency_20230202":
        ("PDT-PRES-04", "2023-02-02", "leave_of_absence", "Carlos Lupi", "P"),
    "br_pdt_figueiredo_announced_presidente_em_exercicio_20230202":
        ("PDT-PRES-04", "2023-02-02", "acting_service", "André Figueiredo", "PA"),
    "br_pdt_lupi_returns_to_presidency_20250520":
        ("PDT-PRES-04", "2025-05-20", "return_from_leave", "Carlos Lupi", "P"),
    "br_pdt_lupi_leave_2023_recalled":
        ("PDT-PRES-04", None, "retrospective_statement", "Carlos Lupi", "P"),
    "br_pdt_lupi_styled_presidente_nacional_20250521":
        ("PDT-PRES-04", "2025-05-21", "in_office_attestation", "Carlos Lupi", "P"),
    "br_pdt_lupi_styled_presidente_nacional_20260819":
        ("PDT-PRES-04", "2026-08-19", "in_office_continuation_attestation", "Carlos Lupi", "P"),
    "br_pdt_lupi_styled_presidente_nacional_20260904":
        ("PDT-PRES-04", "2026-09-04", "in_office_attestation", "Carlos Lupi", "P"),
    "br_mdb_jader_styled_presidente_nacional_undated":
        ("MDB-PRES-01", None, "in_office_period_attestation", "Jader Barbalho", "M"),
    "br_mdb_jader_elected_19980915":
        ("MDB-PRES-01", "1998-09-15", "election", "Jader Barbalho", "M"),
    "br_mdb_temer_styled_presidente_do_pmdb_undated_2001":
        ("MDB-PRES-02", None, "in_office_period_attestation", "Michel Temer", "M"),
    "br_mdb_temer_signs_official_note_as_presidente_20030702":
        ("MDB-PRES-02", "2003-07-02", "in_office_attestation", "Michel Temer", "M"),
    "br_mdb_roster_temer_presidente_licenciado_capt2009":
        ("MDB-PRES-02", None, "leave_status_roster", "Michel Temer", "M"),
    "br_mdb_iris_presidente_em_exercicio_roster_capt2009":
        ("MDB-PRES-02", None, "acting_service", "Iris de Araújo", "MA"),
    "br_mdb_iris_assumes_interim_presidency_20090310":
        ("MDB-PRES-02", "2009-03-10", "interim_service", "Iris de Araújo", "MA"),
    "br_mdb_temer_leave_20090310":
        ("MDB-PRES-02", "2009-03-10", "leave_of_absence", "Michel Temer", "M"),
    "br_mdb_temer_signs_note_as_presidente_licenciado_20100121":
        ("MDB-PRES-03", "2010-01-21", "leave_status_signature", "Michel Temer", "M"),
    "br_mdb_convention_set_for_20100206_prospective_20100120":
        ("MDB-PRES-03", "2010-01-20", "convention_called_prospective", None, "M"),
    "br_mdb_executive_posse_20100310":
        ("MDB-PRES-03", "2010-03-10", "executive_posse_dated", "Michel Temer", "M"),
    "br_mdb_roster_temer_presidente_licenciado_capt2011":
        ("MDB-PRES-03", None, "leave_status_roster", "Michel Temer", "M"),
    "br_mdb_raupp_em_exercicio_roster_capt2011":
        ("MDB-PRES-03", None, "acting_service", "Valdir Raupp", "MA"),
    "br_mdb_convention_held_20130302":
        ("MDB-PRES-03", "2013-03-02", "convention_held", None, "M"),
    "br_mdb_temer_reelected_20130302":
        ("MDB-PRES-03", "2013-03-02", "reelection", "Michel Temer", "M"),
    "br_mdb_tse_sgip_temer_registered_presidente_licenciado_20140716_20160405":
        ("MDB-PRES-03", None, "registry_exercise_period", "Michel Temer", "M"),
    "br_mdb_tse_sgip_juca_registered_presidente_em_exercicio_20160312_20191006":
        ("MDB-PRES-04", None, "registry_acting_period", "Romero Jucá", "MA"),
    "br_mdb_raupp_styled_presidente_nacional_20140514":
        ("MDB-PRES-03", "2014-05-14", "styled_president_conflicting_record", "Valdir Raupp", "M"),
    "br_mdb_convention_held_20160312":
        ("MDB-PRES-03", "2016-03-12", "convention_held", None, "M"),
    "br_mdb_temer_reelected_20160312":
        ("MDB-PRES-03", "2016-03-12", "reelection", "Michel Temer", "M"),
    "br_mdb_temer_styled_presidente_nacional_20160329":
        ("MDB-PRES-03", "2016-03-29", "in_office_attestation", "Michel Temer", "M"),
    "br_mdb_temer_leave_20160405":
        ("MDB-PRES-04", "2016-04-05", "leave_of_absence", "Michel Temer", "M"),
    "br_mdb_juca_assumes_in_temer_place_20160405":
        ("MDB-PRES-04", "2016-04-05", "acting_service", "Romero Jucá", "MA"),
    "br_mdb_juca_styled_presidente_em_exercicio_20160407":
        ("MDB-PRES-04", "2016-04-07", "acting_service", "Romero Jucá", "MA"),
    "br_mdb_juca_signs_edital_as_presidente_nacional_20171123":
        ("MDB-PRES-04", "2017-11-23", "styled_president_during_acting_period", "Romero Jucá", "M"),
    "br_mdb_convention_called_renaming_prospective_20171123":
        ("MDB-PRES-04", "2017-11-23", "convention_called_prospective", None, "M"),
    "br_mdb_extraordinary_convention_held_20171219":
        ("MDB-PRES-04", "2017-12-19", "convention_held", None, "M"),
    "br_mdb_convention_votes_renaming_20171219":
        ("MDB-PRES-04", "2017-12-19", "renaming_decision", None, "M"),
    "br_mdb_juca_styled_presidente_nacional_20171219":
        ("MDB-PRES-04", "2017-12-19", "styled_president_during_acting_period", "Romero Jucá", "M"),
    "br_mdb_renaming_communicated_to_tse_20180131":
        ("MDB-PRES-04", "2018-01-31", "renaming_filed", None, "M"),
    "br_mdb_executive_extends_mandates_20180221":
        ("MDB-PRES-04", "2018-02-21", "mandate_extension", None, "M"),
    "br_mdb_juca_styled_presidente_nacional_20180222":
        ("MDB-PRES-04", "2018-02-22", "styled_president_during_acting_period", "Romero Jucá", "M"),
    "br_mdb_tse_approves_name_change_20180515":
        ("MDB-PRES-04", "2018-05-15", "tse_renaming_decision", None, "M"),
    "br_mdb_roster_temer_presidente_licenciado_capt201901":
        ("MDB-PRES-04", None, "leave_status_roster", "Michel Temer", "M"),
    "br_mdb_roster_juca_first_vice_em_exercicio_capt201901":
        ("MDB-PRES-04", None, "acting_service", "Romero Jucá", "MA"),
    "br_mdb_roster_temer_elected_six_times":
        ("MDB-PRES-04", None, "retrospective_statement", "Michel Temer", "M"),
    "br_mdb_list_jader_mandate_1998_2001":
        ("MDB-PRES-01", None, "retrospective_statement", "Jader Barbalho", "M"),
    "br_mdb_list_maguito_mandate_2001":
        ("MDB-PRES-01", None, "retrospective_statement", "Maguito Vilela", "M"),
    "br_mdb_list_temer_mandate_2001_leave_2009":
        ("MDB-PRES-02", None, "retrospective_statement", "Michel Temer", "M"),
    "br_mdb_list_iris_interim_2009_2010":
        ("MDB-PRES-02", None, "interim_service", "Iris de Araújo", "MA"),
    "br_mdb_list_temer_resumes_2010_leave_2010":
        ("MDB-PRES-03", None, "retrospective_statement", "Michel Temer", "M"),
    "br_mdb_list_raupp_interim_2010_2014":
        ("MDB-PRES-03", None, "interim_service", "Valdir Raupp", "MA"),
    "br_mdb_list_temer_assumes_2014":
        ("MDB-PRES-03", None, "retrospective_statement", "Michel Temer", "M"),
    "br_mdb_list_juca_interim_2016":
        ("MDB-PRES-04", None, "interim_service", "Romero Jucá", "MA"),
    "br_mdb_juca_styled_presidente_do_mdb_20190617":
        ("MDB-PRES-04", "2019-06-17", "styled_president_during_acting_period", "Romero Jucá", "M"),
    "br_mdb_convention_elects_executive_baleia_20191006":
        ("MDB-PRES-05", "2019-10-06", "election", "Baleia Rossi", "M"),
    "br_mdb_tse_sgip_baleia_registered_exercise_20191006_20231006":
        ("MDB-PRES-05", None, "registry_exercise_period", "Baleia Rossi", "M"),
    "br_mdb_baleia_styled_novo_presidente_20191017":
        ("MDB-PRES-05", "2019-10-17", "in_office_attestation", "Baleia Rossi", "M"),
    "br_mdb_executive_extends_baleia_mandate_20210223":
        ("MDB-PRES-05", "2021-02-23", "mandate_extension", "Baleia Rossi", "M"),
    "br_mdb_convention_held_20231005":
        ("MDB-PRES-05", "2023-10-05", "convention_held", None, "M"),
    "br_mdb_baleia_reelected_20231005":
        ("MDB-PRES-05", "2023-10-05", "reelection", "Baleia Rossi", "M"),
    "br_mdb_directorate_elects_executive_by_acclamation_20231005":
        ("MDB-PRES-05", "2023-10-05", "election", "Baleia Rossi", "M"),
    "br_mdb_baleia_opens_executive_meeting_as_presidente_nacional_20250716":
        ("MDB-PRES-05", "2025-07-16", "in_office_attestation", "Baleia Rossi", "M"),
    "br_mdb_executive_and_directorate_extended_to_20270515_20250716":
        ("MDB-PRES-05", "2025-07-16", "mandate_extension", None, "M"),
    "br_mdb_baleia_reconduction_congratulated_20250716":
        ("MDB-PRES-05", None, "retrospective_statement", "Baleia Rossi", "M"),
    "br_mdb_baleia_signs_decision_as_presidente_nacional_20260607":
        ("MDB-PRES-05", "2026-06-07", "in_office_attestation", "Baleia Rossi", "M"),
    "br_mdb_baleia_styled_presidente_nacional_20260608":
        ("MDB-PRES-05", "2026-06-08", "in_office_continuation_attestation", "Baleia Rossi", "M"),
    "br_mdb_baleia_signs_resolution_1_2026_20260330":
        ("MDB-PRES-05", "2026-03-30", "in_office_continuation_attestation", "Baleia Rossi", "M"),
    "br_mdb_baleia_styled_presidente_20260615":
        ("MDB-PRES-05", "2026-06-15", "in_office_continuation_attestation", "Baleia Rossi", "M"),
    "br_pfl_cen_elected_and_invested_bornhausen_presidente_19990507":
        ("PFL-PRES-01", "1999-05-07", "executive_elected_and_invested_at_convention", "Jorge Bornhausen", "F"),
    "br_pfl_bornhausen_styled_presidente_do_pfl_20030307":
        ("PFL-PRES-01", "2003-03-07", "in_office_attestation", "Jorge Bornhausen", "F"),
    "br_pfl_bornhausen_signs_convocation_as_presidente_20070313":
        ("PFL-PRES-01", "2007-03-13", "in_office_attestation", "Jorge Bornhausen", "F"),
    "br_pfl_convention_called_for_20070328_prospective_20070313":
        ("PFL-PRES-02", "2007-03-13", "convention_called_prospective", None, None),
    "br_pfl_site_states_transformation_into_democratas":
        ("PFL-PRES-02", None, "renaming_statement", None, None),
    "br_pfl_resolution_006_issued_in_name_of_democratas_20070329":
        ("PFL-PRES-02", "2007-03-29", "name_in_use_attestation", None, None),
    "br_pfl_maia_signs_resolution_006_as_presidente_nacional_20070329":
        ("PFL-PRES-02", "2007-03-29", "in_office_attestation", "Rodrigo Maia", "F"),
    "br_pfl_maia_signs_resolution_112_as_presidente_nacional_20110215":
        ("PFL-PRES-02", "2011-02-15", "in_office_attestation", "Rodrigo Maia", "F"),
    "br_pfl_single_slate_agreement_announced_20110216":
        ("PFL-PRES-03", "2011-02-16", "slate_agreement_announced", None, None),
    "br_pfl_maia_styled_atual_presidente_20110314":
        ("PFL-PRES-02", "2011-03-14", "in_office_attestation", "Rodrigo Maia", "F"),
    "br_pfl_convention_to_formalize_agripino_prospective_20110314":
        ("PFL-PRES-03", "2011-03-14", "convention_scheduled_prospective", "José Agripino", "F"),
    "br_pfl_agripino_elected_by_acclamation_reported":
        ("PFL-PRES-03", None, "election_reported", "José Agripino", "F"),
    "br_pfl_agripino_signs_resolution_113_as_presidente_nacional_20110324":
        ("PFL-PRES-03", "2011-03-24", "in_office_attestation", "José Agripino", "F"),
    "br_pfl_tse_sgip_agripino_registered_exercise_20151203_20180308":
        ("PFL-PRES-03", None, "registry_exercise_period", "José Agripino", "F"),
    "br_pfl_cen_refundacao_elected_convention_acm_neto_20180308":
        ("PFL-PRES-04", "2018-03-08", "executive_elected_at_convention", "ACM Neto", "F"),
    "br_pfl_tse_sgip_acm_neto_registered_exercise_20180308_20190530":
        ("PFL-PRES-04", None, "registry_exercise_period", "ACM Neto", "F"),
    "br_pfl_tse_sgip_acm_neto_registered_exercise_20190530_20220208":
        ("PFL-PRES-04", None, "registry_exercise_period", "ACM Neto", "F"),
    "br_pfl_tse_sgip_dem_party_record_extinct_by_fusion":
        ("PFL-PRES-04", None, "registry_party_record", None, None),
    "br_pfl_fusion_convention_scheduled_prospective_20211005":
        ("PFL-PRES-04", "2021-10-05", "convention_scheduled_prospective", None, None),
    "br_pfl_acm_neto_styled_presidente_do_dem_20211005":
        ("PFL-PRES-04", "2021-10-05", "in_office_attestation", "ACM Neto", "F"),
    "br_pfl_acm_neto_styled_presidente_nacional_20220112":
        ("PFL-PRES-04", "2022-01-12", "in_office_attestation", "ACM Neto", "F"),
    "br_pfl_fusion_approved_joint_convention_october_2021":
        ("PFL-PRES-04", None, "fusion_approved_by_conventions", None, None),
    "br_pfl_tse_session_on_uniao_scheduled_prospective_20220208":
        ("PFL-PRES-04", "2022-02-08", "tse_session_scheduled_prospective", None, None),
    "br_pfl_acm_neto_styled_uniao_secretary_general_20220208":
        ("PFL-PRES-04", "2022-02-08", "other_organization_office_attestation", None, None),
    "br_pfl_tse_sgip_uniao_organ_registry_identity_20220208":
        ("PFL-PRES-04", "2022-02-08", "registry_identity_of_fused_party", None, None),
    "br_pfl_tse_approves_uniao_registration_fusion_20220208":
        ("PFL-PRES-04", "2022-02-08", "tse_registration_decision", None, None),
    "br_pfl_tse_states_fusion_approved_joint_convention_20211006":
        ("PFL-PRES-04", "2021-10-06", "fusion_approved_by_conventions", None, None),
}
# Exact holder observations of each party role, (name, attested_on, from, until), in chronological order.
HOLDERS = {
    "br_mdb_president": [
        ("Michel Temer", "2003-07-02", None, None),
        ("Michel Temer", "2016-03-29", None, None),
        ("Baleia Rossi", "2019-10-17", None, None),
        ("Baleia Rossi", "2025-07-16", None, None),
        ("Baleia Rossi", "2026-06-07", None, None),
    ],
    "br_pdt_president": [
        ("Leonel Brizola", "1997-04-11", None, None),
        ("Leonel Brizola", "1999-08-26", None, None),
        ("Leonel Brizola", "2004-06-02", None, "2004-06-21"),
        ("Carlos Lupi", "2007-02-09", None, None),
        ("Carlos Lupi", "2021-12-21", None, None),
        ("Carlos Lupi", "2025-05-21", None, None),
        ("Carlos Lupi", "2026-09-04", None, None),
    ],
}
HOLDER_CLAIMS = {
    "br_mdb_president": [
        ["br_mdb_temer_signs_official_note_as_presidente_20030702"],
        ["br_mdb_temer_styled_presidente_nacional_20160329"],
        ["br_mdb_baleia_styled_novo_presidente_20191017"],
        ["br_mdb_baleia_opens_executive_meeting_as_presidente_nacional_20250716"],
        ["br_mdb_baleia_signs_decision_as_presidente_nacional_20260607"],
    ],
    "br_pdt_president": [
        ["br_pdt_brizola_presidente_nacional_home_19970411"],
        ["br_pdt_brizola_styled_presidente_do_pdt_19990826"],
        ["br_pdt_brizola_meets_caucuses_as_presidente_nacional_20040602", "br_pdt_brizola_dies_as_presidente_nacional_20040621"],
        ["br_pdt_lupi_signs_resolution_001_07_as_presidente_nacional_20070209"],
        ["br_pdt_lupi_signs_normative_resolution_003_2021_20211221"],
        ["br_pdt_lupi_styled_presidente_nacional_20250521"],
        ["br_pdt_lupi_styled_presidente_nacional_20260904"],
    ],
}
# The review observation each holder answers, in holder order.
HOLDER_OBSERVATIONS = {
    "br_mdb_president": ["MDB-PRES-02", "MDB-PRES-03", "MDB-PRES-05", "MDB-PRES-05", "MDB-PRES-05"],
    "br_pdt_president": ["PDT-PRES-01", "PDT-PRES-01", "PDT-PRES-01", "PDT-PRES-03", "PDT-PRES-03", "PDT-PRES-04", "PDT-PRES-04"],
}
STARTS = {
    "br_mdb_president": [],
    "br_pdt_president": [],
}
ENDS = {
    "br_mdb_president": [],
    "br_pdt_president": [("Leonel Brizola", "2004-06-21")],
}
# Days that are never any holder's attested_on, start or end in that role: conventions, elections, re-elections, extensions,
# leaves and returns, acting service, stylings of an interim period (Lupi, 6 July 2004 and 28 February 2005),
# prospective and scheduled days, continuation stylings, registry periods and the days the SGIP registry and
# retrospective lists give.
NEVER_HOLDER_DATE = {
    "br_mdb_president": {
        "1998-09-15", "2001-05-15", "2001-09-09", "2009-03-10", "2010-01-20", "2010-01-21", "2010-01-27", "2010-03-10",
        "2010-06-15", "2013-03-02", "2013-03-11", "2014-01-14", "2014-03-11", "2014-05-14", "2014-07-16", "2016-03-12",
        "2016-04-05", "2016-04-07", "2017-11-23", "2017-12-19", "2018-01-31", "2018-02-21", "2018-02-22", "2018-05-15",
        "2019-06-17", "2019-10-06", "2021-02-23", "2023-10-05", "2023-10-06", "2026-03-30", "2026-06-08", "2026-06-15",
    },
    "br_pdt_president": {
        "1991-01-07", "1992-04-26", "1999-04-19", "2003-03-21", "2004-06-28", "2004-07-06", "2005-02-28", "2005-03-21",
        "2007-03-09",
        "2008-03-07", "2008-03-12", "2009-03-06", "2017-03-18", "2019-03-18", "2022-01-10", "2022-01-21", "2022-01-22",
        "2023-02-02", "2025-05-20", "2026-08-18", "2026-08-19",
    },
}
FROM_KINDS = {'posse_reported', 'stated_assumption_of_office'}
# The only structured end: the party's same-day report of Leonel Brizola's death in office (21 June 2004).
UNTIL_KINDS = {'death_in_office_stated', 'resignation_stated'}
SUPPORT_KINDS = {'in_office_attestation'}
# Acting and interim service: claims only, never holders, never splitting or ending a holder.
ACTING_KINDS = {'acting_service', 'interim_service', 'registry_acting_period'}
# Conventions, elections, extensions, leaves, returns, stylings that cannot date a holder, and organization events.
ELECTION_KINDS = {'election', 'reelection', 'reelection_while_on_leave', 'convention_held', 'executive_dated_roster',
                  'executive_posse_dated', 'executive_elected_and_invested_at_convention',
                  'executive_elected_at_convention', 'election_reported', 'slate_agreement_announced',
                  'mandate_extension', 'convention_called_prospective', 'convention_scheduled_prospective',
                  'convention_speech_published', 'styled_on_election_day', 'executive_note_reference'}
DEPARTURE_KINDS = {'leave_of_absence', 'leave_status_signature', 'leave_status_roster', 'return_from_leave',
                   'death_reported'}
# Stylings of an interim period use CLAUDE-C01-22's kind for Genoino's: Lupi's service from June 2004 to his election
# of 21 March 2005, interim by the party's 2019 history (integrator's ruling), is claims only.
STYLING_KINDS = {'in_office_continuation_attestation', 'in_office_period_attestation',
                 'styled_president_during_acting_period', 'styled_president_during_interim_period',
                 'styled_president_conflicting_record', 'stated_assumption_day_not_printed'}
ORGANIZATION_KINDS = {'renaming_decision', 'renaming_filed', 'tse_renaming_decision', 'renaming_statement',
                      'name_in_use_attestation', 'fusion_approved_by_conventions', 'tse_session_scheduled_prospective',
                      'tse_registration_decision', 'registry_identity_of_fused_party', 'registry_party_record',
                      'other_organization_office_attestation'}
RETROSPECTIVE_KINDS = {'retrospective_statement', 'registry_exercise_period', 'registry_acting_period'}
# Claims that carry no structured date at all.
UNDATED_KINDS = RETROSPECTIVE_KINDS | {'in_office_period_attestation', 'stated_assumption_day_not_printed'}
SURNAMES = {
    'br_mdb_president': {'Michel Temer': 'Temer', 'Baleia Rossi': 'Baleia Rossi', 'Jader Barbalho': 'JADER',
                         'Romero Jucá': 'JUCÁ'},
    'br_pdt_president': {'Leonel Brizola': 'Brizola', 'Carlos Lupi': 'Lupi'},
}
OTHER_NAMES = {
    'br_mdb_president': {'Iris de Araújo', 'Valdir Raupp', 'Maguito Vilela'},
    'br_pdt_president': {'André Figueiredo', 'Vieira da Cunha'},
}
PFL_NAMES = {'Jorge Bornhausen', 'Rodrigo Maia', 'José Agripino', 'ACM Neto'}
# People researched by this packet (at most ten) and those it names only as acting or outside its list.
PEOPLE = {'Leonel Brizola', 'Carlos Lupi', 'Jader Barbalho', 'Michel Temer', 'Romero Jucá', 'Baleia Rossi',
          'Jorge Bornhausen', 'Rodrigo Maia', 'José Agripino', 'ACM Neto'}
REVIEW = ([f'PDT-PRES-{n:02d}' for n in range(1, 5)] + [f'MDB-PRES-{n:02d}' for n in range(1, 6)] +
          [f'PFL-PRES-{n:02d}' for n in range(1, 5)])
HOSTS = {'web.archive.org', 'sgip3.tse.jus.br'}
# Secondary leads, blocked, live or listing pages that must never be a recorded identity.
LEAD_URL_MARKERS = ('wikipedia', 'folha', 'estadao', 'g1.globo', 'poder360', 'oglobo', 'uol.com', 'veja.abril',
                    'agenciabrasil', 'cnnbrasil', 'metropoles', 'congressoemfoco', 'lideres-historicos',
                    'g1-lupi-diz-que-se-licenciou', 'bzcnac17201', 'not_direita', 'executiva_direita',
                    'noticias.php?cd=2108', 'ATA-DA-REUNIAO-DA-CONVENCAO-NACIONAL-ORDINARIA', 'orgaoPartidario/consulta',
                    'api/v1/partidos', 'idOrgaoPartidario=406194', 'idOrgaoPartidario=406570',
                    'idOrgaoPartidario=468918', 'pdt280604', 'resolucao-mdb-01-2026', 'dirna.htm', 'flb-ap.org.br')
# Pairs of distinct dated events that must stay distinct and in this order.
ORDERED_PAIRS = (
    ('br_pdt_brizola_meets_caucuses_as_presidente_nacional_20040602', 'br_pdt_brizola_dies_as_presidente_nacional_20040621'),
    ('br_pdt_brizola_dies_as_presidente_nacional_20040621', 'br_pdt_lupi_presidente_nacional_home_20040706'),
    ('br_pdt_convention_convoked_for_20050321_20050228', 'br_pdt_convention_held_20050321'),
    ('br_pdt_lupi_reconducted_xxv_convention_20190318', 'br_pdt_lupi_signs_normative_resolution_003_2021_20211221'),
    ('br_pdt_lupi_takes_leave_of_presidency_20230202', 'br_pdt_lupi_returns_to_presidency_20250520'),
    ('br_pdt_lupi_returns_to_presidency_20250520', 'br_pdt_lupi_styled_presidente_nacional_20250521'),
    ('br_mdb_jader_elected_19980915', 'br_mdb_temer_signs_official_note_as_presidente_20030702'),
    ('br_mdb_iris_assumes_interim_presidency_20090310', 'br_mdb_temer_signs_note_as_presidente_licenciado_20100121'),
    ('br_mdb_temer_reelected_20160312', 'br_mdb_temer_styled_presidente_nacional_20160329'),
    ('br_mdb_temer_styled_presidente_nacional_20160329', 'br_mdb_temer_leave_20160405'),
    ('br_mdb_convention_votes_renaming_20171219', 'br_mdb_renaming_communicated_to_tse_20180131'),
    ('br_mdb_renaming_communicated_to_tse_20180131', 'br_mdb_tse_approves_name_change_20180515'),
    ('br_mdb_convention_elects_executive_baleia_20191006', 'br_mdb_baleia_styled_novo_presidente_20191017'),
    ('br_mdb_baleia_reelected_20231005', 'br_mdb_baleia_opens_executive_meeting_as_presidente_nacional_20250716'),
    ('br_pfl_convention_called_for_20070328_prospective_20070313', 'br_pfl_maia_signs_resolution_006_as_presidente_nacional_20070329'),
    ('br_pfl_maia_signs_resolution_112_as_presidente_nacional_20110215', 'br_pfl_agripino_signs_resolution_113_as_presidente_nacional_20110324'),
    ('br_pfl_fusion_convention_scheduled_prospective_20211005', 'br_pfl_tse_states_fusion_approved_joint_convention_20211006'),
    ('br_pfl_tse_states_fusion_approved_joint_convention_20211006', 'br_pfl_tse_approves_uniao_registration_fusion_20220208'),
    ('br_mdb_convention_set_for_20100206_prospective_20100120', 'br_mdb_temer_signs_note_as_presidente_licenciado_20100121'),
)


NEW_SOURCES = list(RESPONSES)
NEW_CLAIMS = list(EVENTS)
REPORT = research.RESEARCH / 'brazil-party-presidents-1990-2026-34.md'
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-34.md'
HOLDER_KINDS = FROM_KINDS | UNTIL_KINDS | SUPPORT_KINDS
PER_REQUEST_URL = re.compile(r'cdx/search|[?&]cb=|nocache|cachebust|[?&]_=|/search|[?&]s=|montaPdf|dc_20b|'
                             r'seqPagina|jsessionid|token|[?&]sid=|consulta\?|timemap|/feed/?$|/embed/?$|wp-json|'
                             r'/tag/|/category/|/page/\d', re.I)


def load_rows():
    packet = json.loads((research.ROOT / research.RESEARCH / 'brazil.json').read_text(encoding='utf-8'))
    rows = {}
    for source in packet['sources'][EARLIER_SOURCE_COUNT:]:
        extract = json.loads((research.ROOT / source['snapshot']['path']).read_text(encoding='utf-8'))
        rows.update({row['claim_id']: row for row in extract['rows']})
    return rows


def entry(packet, org_id):
    found, = [o for o in packet['organizations'] if o['id'] == org_id]
    return found


def role_of(packet, role_id):
    org = entry(packet, ROLE_ORG[role_id])
    found, = [r for r in org['roles'] if r['id'] == role_id]
    return found


def party_rules(packet, rows):
    """Rule-based checks that hold without the pinned holder lists; raise AssertionError, KeyError or ValueError."""
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    claim_source = {c['id']: s['id'] for s in packet['sources'] for c in s['claims']}
    # The funding observations keep their identity, lifecycle, empty mapping and funding record.
    for org_id, (label, case) in ORG_LABEL.items():
        org = entry(packet, org_id)
        assert (org['name'], org['kind']) == (label, 'political_party_election_funding_observation'), org_id
        assert org['source_identifier']['value'] == case, org_id
        assert org['represented_party_ids'] == [] and org['reconciled_organization_id'] is None, org_id
        assert org['lifecycle'] == pt.LIFECYCLE and org['funding_observation'] == pt.FUNDING, org_id
        assert org['coverage']['unresolved'][:3] == pt.ORG_UNRESOLVED, org_id
    # UNIÃO keeps no role, no PFL/DEM claim or source and its single funding row.
    uniao = entry(packet, UNIAO_ORG)
    assert uniao['roles'] == [] and uniao['sources'] == ['br_tse_fefc_2024'], 'UNIÃO changed'
    assert uniao['claim_ids'] == ['br_tse_fefc_2024_row_27'] and len(uniao['coverage']['unresolved']) == 3
    # Exactly three party offices, each once, each on its own funding observation.
    placed = [(e['id'], r['id']) for e in packet['organizations'] + packet['institutions'] for r in e['roles']
              if r['kind'] == 'party_leader' or r['id'] in ROLE_ORG or r['title'] in TITLES.values()]
    # CLAUDE-C01-43's party role on the AGIR observation follows in packet order.
    assert placed == [(MDB_ORG, MDB), (PDT_ORG, PDT), (PT_ORG, pt.ROLE),
                      ('br_tse_fefc_2024_party_07', 'br_agir_president')], placed
    for role_id, org_id in ROLE_ORG.items():
        org = entry(packet, org_id)
        assert [(r['id'], r['title'], r['kind']) for r in org['roles']] == [(role_id, ROLE_TITLE[role_id], 'party_leader')]
        assert list(org['roles'][0]) == ['id', 'title', 'kind', 'sources', 'claim_ids', 'holder_claims', 'scope_note']
    assert [i['id'] for i in packet['institutions']] == ['br_presidency']
    presidency = packet['institutions'][0]
    assert [r['id'] for r in presidency['roles']] == ['br_president', 'br_vice_president']

    def refs(obj):
        cs = set(obj['claim_ids']) | {c for r in obj.get('roles', []) for c in r['claim_ids']} | {
            c for r in obj.get('roles', []) for h in r['holder_claims'] for c in h['claim_ids']}
        ss = set(obj['sources']) | {s for r in obj.get('roles', []) for s in r['sources']} | {
            s for r in obj.get('roles', []) for h in r['holder_claims'] for s in h['sources']}
        return cs, ss

    # The PFL/DEM chain is claims only: no organization, institution, role or holder cites its claims or sources.
    pfl_claims = {cid for cid, e in EVENTS.items() if e[4] == 'F' or cid.startswith(PFL_PREFIX)}
    pfl_sources = {s for s in NEW_SOURCES if s.startswith(PFL_PREFIX)}
    for e in packet['organizations'] + packet['institutions']:
        cs, ss = refs(e)
        assert not cs & pfl_claims and not ss & pfl_sources, (e['id'], 'PFL/DEM claims are never cited')
    for cid in pfl_claims:
        row = rows[cid]
        assert (row['observation_id'], row['role_id']) == (None, None), cid
        assert row['role_title'] == (TITLES['F'] if row['holder_name'] else None), cid
        assert row['holder_name'] in set(PFL_NAMES) | {None}, cid
    p_claims, p_sources = refs(presidency)
    pt_claims, pt_sources = refs(entry(packet, PT_ORG))
    for role_id, org_id in ROLE_ORG.items():
        org = entry(packet, org_id)
        role = org['roles'][0]
        r_claims = set(role['claim_ids']) | {c for h in role['holder_claims'] for c in h['claim_ids']}
        r_sources = set(role['sources']) | {s for h in role['holder_claims'] for s in h['sources']}
        # Party office, the Presidency of the Republic, the PT office and the other party office never feed each other.
        assert not r_claims & p_claims and not r_sources & p_sources, (role_id, 'presidency')
        assert not r_claims & (pt_claims - {'br_tse_fefc_2024_row_03'}) and not r_sources & (pt_sources - {'br_tse_fefc_2024'})
        assert all(c.startswith(PREFIX[role_id]) for c in r_claims), role_id
        assert all(s.startswith(PREFIX[role_id]) for s in r_sources), role_id
        for other in packet['organizations'] + packet['institutions']:
            if other is not org:
                cs, ss = refs(other)
                assert not cs & r_claims and not ss & r_sources, (role_id, other['id'])
        assert set(role['claim_ids']) <= set(org['claim_ids']) and set(role['sources']) <= set(org['sources'])
        for cid in role['claim_ids']:
            row = rows[cid]
            assert (row['observation_id'], row['role_id']) == (org_id, role_id), cid
            code = {v: k for k, v in TITLES.items()}[row['role_title']]
            assert code in ROLE_CODES[role_id], cid
            assert (code in {'PA', 'MA'}) == (row['event_kind'] in ACTING_KINDS), cid
            assert row['holder_name'] in set(SURNAMES[role_id]) | OTHER_NAMES[role_id] | {None}, cid
            if row['event_kind'] in UNDATED_KINDS:
                assert 'attested_on' not in claims[cid], (cid, 'retrospective and period claims carry no structured date')
        previous = ''
        for holder in role['holder_claims']:
            name = holder['name']
            assert isinstance(holder, dict) and name in SURNAMES[role_id], name
            assert list(holder) == ['name', 'attested_on', 'from', 'until', 'sources', 'claim_ids', 'note', 'uncertainty']
            dated = [d for d in (holder['attested_on'], holder['from']) if d]
            assert len(dated) == 1, (name, 'a holder is dated by exactly one of attested_on and from')
            assert dated[0] >= previous, (name, 'holders stay in chronological order')
            previous = dated[0]
            assert not {holder['attested_on'], holder['from'], holder['until']} & NEVER_HOLDER_DATE[role_id], name
            kinds = {}
            expected_sources = []
            for cid in holder['claim_ids']:
                row = rows[cid]
                assert cid in role['claim_ids'], cid
                # Cross-role guard: a holder rests only on this role's rows for the same person, titled as the office.
                assert row['role_id'] == role_id and row['holder_name'] == name and row['role_title'] == ROLE_TITLE[role_id]
                assert row['event_kind'] in HOLDER_KINDS, (name, cid)
                assert SURNAMES[role_id][name].lower() in claims[cid]['text'].lower(), (name, cid)
                kinds.setdefault(row['event_kind'], set()).add(claims[cid].get('attested_on'))
                if claim_source[cid] not in expected_sources:
                    expected_sources.append(claim_source[cid])
            assert holder['sources'] == expected_sources, name
            from_days = set().union(*(kinds.get(k, set()) for k in FROM_KINDS))
            until_days = set().union(*(kinds.get(k, set()) for k in UNTIL_KINDS))
            support_days = set().union(*(kinds.get(k, set()) for k in SUPPORT_KINDS))
            # A start only where a reported posse or a stated assumption gives that day; otherwise no such claim.
            assert (from_days == {holder['from']}) if holder['from'] else not from_days, (name, 'start')
            # An end only where a stated death in office or resignation gives that day; otherwise no end claim.
            assert (until_days == {holder['until']}) if holder['until'] else not until_days, (name, 'end')
            # An observation date only from an in-office attestation of that very day.
            assert support_days <= {holder['from'] or holder['attested_on']}, (name, 'support on another day')
            if holder['attested_on']:
                assert support_days == {holder['attested_on']}, (name, 'observation')
            if holder['until']:
                assert holder['until'] <= research.CUTOFF and dated[0] <= holder['until']
        # Claims that never feed a holder carry a kind that cannot date one, and no acting service is a holder.
        cited = {c for h in role['holder_claims'] for c in h['claim_ids']}
        for cid in role['claim_ids']:
            assert (rows[cid]['event_kind'] in HOLDER_KINDS) == (cid in cited), cid


def party_invariants(packet, rows):
    """The rules plus the exact pinned holder lists and events this packet intends."""
    party_rules(packet, rows)
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    for role_id in ROLE_ORG:
        role = role_of(packet, role_id)
        got = [(h['name'], h['attested_on'], h['from'], h['until']) for h in role['holder_claims']]
        assert got == HOLDERS[role_id], (role_id, got)
        assert [h['claim_ids'] for h in role['holder_claims']] == HOLDER_CLAIMS[role_id], role_id
        assert [(h['name'], h['until']) for h in role['holder_claims'] if h['until']] == ENDS[role_id], role_id
        assert [(h['name'], h['from']) for h in role['holder_claims'] if h['from']] == STARTS[role_id], role_id
    for cid, (_, day, _, _, _) in EVENTS.items():
        assert claims[cid].get('attested_on') == day, cid
    # The presidency, vice-presidency and PT holders are untouched by this packet.
    president, vice = packet['institutions'][0]['roles']
    assert [(h['name'], h['attested_on'], h['from'], h['until']) for h in president['holder_claims']] == vp.PRESIDENT_HOLDERS
    assert [(h['name'], h['attested_on'], h['from'], h['until']) for h in vice['holder_claims']] == vp.HOLDERS
    pt_role = entry(packet, PT_ORG)['roles'][0]
    assert [(h['name'], h['attested_on'], h['from'], h['until']) for h in pt_role['holder_claims']] == pt.HOLDERS
    # Distinct dated events stay distinct.
    for earlier, later in ORDERED_PAIRS:
        assert claims[earlier]['attested_on'] < claims[later]['attested_on'], (earlier, later)


class BrazilPartyPresidentsTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.raw = (research.ROOT / research.RESEARCH / 'brazil.json').read_text(encoding='utf-8')
        cls.packet = json.loads(cls.raw)
        cls.sources = {s['id']: s for s in cls.packet['sources']}
        cls.claims = {c['id']: c for s in cls.packet['sources'] for c in s['claims']}
        cls.extracts = {sid: json.loads((research.ROOT / cls.sources[sid]['snapshot']['path']).read_text(encoding='utf-8'))
                        for sid in NEW_SOURCES}
        cls.rows = load_rows()
        cls.report = (research.ROOT / REPORT).read_text(encoding='utf-8')

    def validate(self, packet=None):
        return research.validate(packet or self.packet, research.ROOT, {'Brazil'}, {'Brazil': set()})

    def section(self, heading):
        parts = self.report.split(f'\n## {heading}', 1)
        self.assertEqual(len(parts), 2, heading)
        return parts[1].split('\n## ', 1)[0]

    def test_new_records_are_bounded_and_every_claim_is_classified(self):
        ids = self.validate()
        self.assertEqual(tuple(len(ids[k]) for k in ('entries', 'sources', 'claims', 'roles')), (32, 262, 556, 6))
        self.assertEqual((len(NEW_SOURCES), len(NEW_CLAIMS)), (76, 127))
        self.assertEqual({r: len(h) for r, h in HOLDERS.items()}, {MDB: 5, PDT: 7})
        order = [s['id'] for s in self.packet['sources']]
        # CLAUDE-C01-43's 14 sources follow this packet's.
        self.assertEqual(order[EARLIER_SOURCE_COUNT:EARLIER_SOURCE_COUNT + len(NEW_SOURCES)], NEW_SOURCES)
        self.assertEqual(len(order), EARLIER_SOURCE_COUNT + len(NEW_SOURCES) + 14)
        self.assertTrue(all(sid.startswith('br_agir_') for sid in order[EARLIER_SOURCE_COUNT + len(NEW_SOURCES):]))
        self.assertEqual(order[EARLIER_SOURCE_COUNT - len(pt.NEW_SOURCES):EARLIER_SOURCE_COUNT], pt.NEW_SOURCES)
        self.assertFalse([sid for sid in order[:EARLIER_SOURCE_COUNT] if sid.startswith(('br_pdt_', 'br_mdb_', 'br_pfl_'))])
        self.assertEqual([c['id'] for sid in NEW_SOURCES for c in self.sources[sid]['claims']], NEW_CLAIMS)
        # Each role holds exactly its own chain's claims and sources, in packet order.
        for role_id, prefix in PREFIX.items():
            role = role_of(self.packet, role_id)
            chain_sources = [s for s in NEW_SOURCES if s.startswith(prefix)]
            chain_claims = [c for c in NEW_CLAIMS if c.startswith(prefix)]
            self.assertEqual(role['sources'], chain_sources)
            self.assertEqual(role['claim_ids'], chain_claims)
            org = entry(self.packet, ROLE_ORG[role_id])
            row_id = {MDB: 'br_tse_fefc_2024_row_01', PDT: 'br_tse_fefc_2024_row_02'}[role_id]
            self.assertEqual(org['claim_ids'], [row_id] + chain_claims)
            self.assertEqual(org['sources'], ['br_tse_fefc_2024'] + chain_sources)
        self.assertEqual(len([s for s in NEW_SOURCES if s.startswith('br_pdt_')]), 27)
        self.assertEqual(len([s for s in NEW_SOURCES if s.startswith('br_mdb_')]), 31)
        self.assertEqual(len([s for s in NEW_SOURCES if s.startswith(PFL_PREFIX)]), 18)
        # Every new claim is a holder claim or a claim that never feeds a holder, never both.
        holder_claims = [cid for role in HOLDER_CLAIMS.values() for ids_ in role for cid in ids_]
        self.assertEqual(len(holder_claims), len(set(holder_claims)))
        self.assertEqual(len(holder_claims), 13)
        known = (HOLDER_KINDS | ACTING_KINDS | ELECTION_KINDS | DEPARTURE_KINDS | STYLING_KINDS | ORGANIZATION_KINDS |
                 RETROSPECTIVE_KINDS)
        for cid, event in EVENTS.items():
            self.assertIn(event[2], known, cid)
            # In the two roles a claim dates a holder exactly when its kind can; PFL/DEM claims date nothing.
            if not cid.startswith(PFL_PREFIX):
                self.assertEqual(event[2] in HOLDER_KINDS, cid in holder_claims, cid)
            else:
                self.assertNotIn(cid, holder_claims)
        # Thirteen observations, each reported and each carrying rows; at most ten people researched.
        observations = re.findall(r'^### ((?:PDT|MDB|PFL)-PRES-\d\d)\b', self.report, re.M)
        self.assertEqual(observations, REVIEW)
        self.assertEqual({row[0] for row in EVENTS.values()}, set(REVIEW))
        named = {e[3] for e in EVENTS.values() if e[3]}
        self.assertLessEqual(len(PEOPLE), 10)
        self.assertEqual(named - PEOPLE, set().union(*OTHER_NAMES.values()))
        self.assertEqual({h[0] for role in HOLDERS.values() for h in role},
                         {'Leonel Brizola', 'Carlos Lupi', 'Michel Temer', 'Baleia Rossi'})

    def test_holders_are_exactly_as_intended(self):
        party_invariants(self.packet, self.rows)
        for role_id in ROLE_ORG:
            role = role_of(self.packet, role_id)
            for index, holder in enumerate(role['holder_claims']):
                self.assertTrue(holder['note'].startswith('From ' if holder['from'] else 'Observed on '), holder['name'])
                self.assertTrue(holder['uncertainty'], holder['name'])
                cited_obs = {self.rows[cid]['review_observation'] for cid in holder['claim_ids']}
                self.assertEqual(cited_obs, {HOLDER_OBSERVATIONS[role_id][index]}, holder['name'])
        # Holder names are normalised; the printed forms stay in the claim texts.
        self.assertIn('LUIZ FELIPE BALEIA TENUTO ROSSI', self.claims['br_mdb_tse_sgip_baleia_registered_exercise_20191006_20231006']['text'])
        self.assertIn('MICHEL MIGUEL ELIAS TEMER LULIA', self.claims['br_mdb_tse_sgip_temer_registered_presidente_licenciado_20140716_20160405']['text'])
        self.assertIn('Leonel de Moura Brizola', self.claims['br_pdt_brizola_dies_as_presidente_nacional_20040621']['text'])
        self.assertIn('CARLOS ROBERTO LUPI', self.claims['br_pdt_tse_sgip_lupi_registered_exercise_20170318_20190318']['text'])
        self.assertIn('ANTÔNIO CARLOS MAGALHÃES NETO', self.claims['br_pfl_tse_sgip_acm_neto_registered_exercise_20180308_20190530']['text'])
        # Acting and interim service is claims only, titled as such, and never cited by a holder.
        acting = [cid for cid, e in EVENTS.items() if e[2] in ACTING_KINDS]
        self.assertEqual(len(acting), 12)
        for cid in acting:
            self.assertIn(EVENTS[cid][4], {'PA', 'MA'}, cid)
            self.assertRegex(self.claims[cid]['uncertainty'], r'(?i)claim only|never a holder', cid)
        self.assertEqual({EVENTS[cid][3] for cid in acting}, {'Vieira da Cunha', 'André Figueiredo', 'Iris de Araújo',
                                                               'Valdir Raupp', 'Romero Jucá'})
        # Romero Jucá is never a holder: his unqualified stylings are claims of the acting period.
        for cid, e in EVENTS.items():
            if e[3] == 'Romero Jucá':
                self.assertIn(e[2], ACTING_KINDS | {'styled_president_during_acting_period'}, cid)
        # Carlos Lupi's service before his election of 21 March 2005 is interim by the party's 2019 history
        # (integrator's ruling): its three stylings are claims of the interim period, and his first holder is 2007.
        interim = [cid for cid, e in EVENTS.items() if e[2] == 'styled_president_during_interim_period']
        self.assertEqual(interim, ['br_pdt_lupi_presidente_nacional_home_20040706', 'br_pdt_roster_lupi_presidente_capt20040722',
                                   'br_pdt_lupi_signs_convocation_as_presidente_nacional_20050228'])
        for cid in interim:
            self.assertEqual(EVENTS[cid][3], 'Carlos Lupi', cid)
            self.assertIn("the party's 2019 history records as his interim service", self.claims[cid]['uncertainty'], cid)
            self.assertIn('never a holder date', self.claims[cid]['uncertainty'], cid)
        lupi = [h['attested_on'] for h in role_of(self.packet, PDT)['holder_claims'] if h['name'] == 'Carlos Lupi']
        self.assertEqual(lupi[0], '2007-02-09')
        # Holder claims explain their use.
        for role_id in ROLE_ORG:
            for holder in role_of(self.packet, role_id)['holder_claims']:
                for cid in holder['claim_ids']:
                    kind = EVENTS[cid][2]
                    pattern = {'in_office_attestation': r'Dates the holder observation|not itself a start',
                               'posse_reported': r'basis for .* start', 'stated_assumption_of_office': r'basis for .* start',
                               'death_in_office_stated': r'basis for .* until', 'resignation_stated': r'basis for .* until'}[kind]
                    self.assertRegex(self.claims[cid]['uncertainty'], pattern, cid)

    def test_starts_ends_and_acting_service_only_where_a_source_states_them(self):
        claims = self.claims
        undated = [cid for cid, e in EVENTS.items() if e[1] is None]
        self.assertEqual(len(undated), 43)
        for cid in undated:
            self.assertNotIn('attested_on', claims[cid], cid)
            self.assertIn('no structured date is stored', claims[cid]['uncertainty'].lower(), cid)
        for cid in [c for c, e in EVENTS.items() if e[2] in UNDATED_KINDS]:
            self.assertNotIn('attested_on', claims[cid], cid)
        # No start anywhere; the one end is the party's same-day report of Brizola's death in office.
        self.assertEqual(STARTS, {MDB: [], PDT: []})
        self.assertEqual(ENDS, {MDB: [], PDT: [('Leonel Brizola', '2004-06-21')]})
        self.assertIn("'O presidente nacional do PDT e grande líder trabalhista brasiliero, Leonel de Moura Brizola, não "
                      "resistiu a uma parada cardíaca'", claims['br_pdt_brizola_dies_as_presidente_nacional_20040621']['text'])
        self.assertIn('fallback is a death claim only', claims['br_pdt_brizola_dies_as_presidente_nacional_20040621']['uncertainty'])
        # Recorded but never a boundary, each saying why.
        self.assertIn('not taken as his start', claims['br_mdb_executive_posse_20100310']['uncertainty'])
        self.assertIn('never a start', claims['br_pdt_lupi_assumes_automatically_on_brizola_death']['uncertainty'])
        self.assertIn("assumia interinamente", claims['br_pdt_history_lupi_assumed_interim_executive_meeting_2004']['text'])
        self.assertIn('never an end', claims['br_mdb_temer_leave_20160405']['uncertainty'])
        self.assertIn('never an end', claims['br_pdt_lupi_takes_leave_of_presidency_20230202']['uncertainty'])
        self.assertIn('never a start', claims['br_pdt_lupi_returns_to_presidency_20250520']['uncertainty'])
        self.assertIn('never a start', claims['br_mdb_convention_elects_executive_baleia_20191006']['uncertainty'])
        self.assertIn('never a merged identity', claims['br_mdb_convention_votes_renaming_20171219']['uncertainty'])
        self.assertIn('never a merged identity', claims['br_pfl_tse_approves_uniao_registration_fusion_20220208']['uncertainty'])
        for role_id, phrases in {
                PDT: ('no br_presidency claim or source feeds this role', 'Do not fill the interval',
                      "infer an outgoing holder's last day from a successor's", 'Acting service and continuation stylings '
                      'are claims only, never holders', 'every claim and source id of this role begins br_pdt_'),
                MDB: ('no br_presidency claim or source feeds this role', 'none of its claims feeds br_presidency',
                      'Do not fill the interval', "infer an outgoing holder's last day from a successor's",
                      'never by a name match', 'every claim and source id of this role begins br_mdb_')}.items():
            scope = role_of(self.packet, role_id)['scope_note']
            for phrase in phrases:
                self.assertIn(phrase, scope, (role_id, phrase))
        for org_id, label in ((MDB_ORG, 'PMDB/MDB national presidents 1990-2026 (CLAUDE-C01-34)'),
                              (PDT_ORG, 'PDT national presidents 1990-2026 (CLAUDE-C01-34)')):
            unresolved = entry(self.packet, org_id)['coverage']['unresolved']
            self.assertEqual(unresolved[:-1], pt.ORG_UNRESOLVED)
            self.assertTrue(unresolved[-1].startswith(label), org_id)
        packet_unresolved = self.packet['coverage']['unresolved']
        # CLAUDE-C01-43's note follows this packet's.
        self.assertTrue(packet_unresolved[-2].startswith('PFL/DEM, PDT and PMDB/MDB national presidents 1990-2026 (CLAUDE-C01-34)'))
        self.assertEqual(sum('CLAUDE-C01-34' in u for u in packet_unresolved), 1)
        self.assertTrue(packet_unresolved[-3].startswith('PT national presidents 1990-2026 (CLAUDE-C01-22'))
        self.assertTrue(packet_unresolved[-1].startswith('PRN / PTC / Agir national presidents 1990-2026 (CLAUDE-C01-43)'))

    def test_pfl_dem_chain_is_claims_only_and_never_tied_to_uniao(self):
        pfl_sources = [s for s in NEW_SOURCES if s.startswith(PFL_PREFIX)]
        pfl_claims = [c for s in pfl_sources for c in (x['id'] for x in self.sources[s]['claims'])]
        self.assertEqual(len(pfl_claims), 27)
        self.assertEqual(pfl_claims, [c for c in NEW_CLAIMS if c.startswith(PFL_PREFIX)])
        cited = set()
        for e in self.packet['organizations'] + self.packet['institutions']:
            cited |= set(e['claim_ids']) | set(e['sources'])
            for r in e['roles']:
                cited |= set(r['claim_ids']) | set(r['sources'])
                for h in r['holder_claims']:
                    cited |= set(h['claim_ids']) | set(h['sources'])
        self.assertFalse(cited & (set(pfl_claims) | set(pfl_sources)))
        for cid in pfl_claims:
            row = self.rows[cid]
            self.assertEqual((row['observation_id'], row['role_id']), (None, None), cid)
            self.assertEqual(EVENTS[cid][4], 'F' if EVENTS[cid][3] else None, cid)
        self.assertEqual({EVENTS[c][3] for c in pfl_claims} - {None}, PFL_NAMES)
        uniao = entry(self.packet, UNIAO_ORG)
        self.assertEqual((uniao['roles'], uniao['sources'], uniao['claim_ids']),
                         ([], ['br_tse_fefc_2024'], ['br_tse_fefc_2024_row_27']))
        # The TSE's fusion decision and the registry's separate CNPJ are recorded as organization claims only.
        self.assertIn('agremiação política resultante da fusão do Democratas (DEM)', self.claims['br_pfl_tse_approves_uniao_registration_fusion_20220208']['text'])
        self.assertIn('44.551.496/0001-67', self.claims['br_pfl_tse_sgip_uniao_organ_registry_identity_20220208']['text'])
        for cid in ('br_pfl_tse_sgip_agripino_registered_exercise_20151203_20180308',
                    'br_pfl_tse_sgip_acm_neto_registered_exercise_20190530_20220208'):
            self.assertIn('01.633.510/0001-69', self.claims[cid]['text'])
        self.assertIn("'sua transformação em DEMOCRATAS", self.claims['br_pfl_site_states_transformation_into_democratas']['text'])

    def test_extracts_match_packet_claims_and_record_original_responses(self):
        for sid in NEW_SOURCES:
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual(extract['format'], 'spheres-c01-derived-factual-table/v1')
            self.assertEqual((extract['source_id'], extract['source_url']), (sid, source['url']))
            for key in ('scope_note', 'access_method', 'published_date', 'rights_note'):
                self.assertEqual(extract[key], source[key], (sid, key))
            accessed = '2026-09-30' if sid == 'br_mdb_tse_sgip_cen_2013_2019' else '2026-09-28'
            self.assertEqual((extract['accessed_date'], source['accessed_date']), (accessed, accessed))
            self.assertLessEqual(source['published_date'] or '', research.CUTOFF)
            self.assertIs(extract['source_response_checked_in'], False)
            self.assertEqual((extract['source_response_bytes'], extract['source_response_sha256']), RESPONSES[sid])
            self.assertIn('not checked into this repository', extract['provenance_note'])
            self.assertIn('derived factual extract', extract['provenance_note'])
            self.assertIn('No open license, portrait permission or likeness approval', extract['rights_note'])
            self.assertRegex(extract['stability_check'], r'and again at')
            self.assertIn(extract['stability_check'], extract['provenance_note'])
            self.assertEqual(extract['visual_review']['pdf_pages_one_based'], PDF_PAGES.get(sid, []))
            self.assertTrue(source['source_type'].startswith('primary_') and source['scope_note'] and source['publisher'])
            if sid in ENCODED:
                encoding, size, digest = ENCODED[sid]
                self.assertEqual(extract['source_response_content_encoding'], encoding)
                self.assertEqual((extract['decoded_response_bytes'], extract['decoded_response_sha256']), (size, digest))
                self.assertIn(f'the recorded identity is the {encoding}-encoded body exactly as served', extract['provenance_note'])
            else:
                self.assertEqual(extract['source_response_content_encoding'], 'identity')
                self.assertNotIn('decoded_response_sha256', extract)
            snapshot = source['snapshot']
            self.assertTrue(snapshot['path'].startswith('docs/campaign-certification/C01/research/sources/brazil-'))
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
                self.assertNotIn('name', row)
                self.assertEqual(list(row), ['claim_id', 'observation_id', 'review_observation', 'role_id',
                                             'holder_name', 'role_title', 'event_kind', 'attested_on', 'text', 'locator'])
            url = urlsplit(source['url'])
            if sid in ARCHIVED:
                stamp = ARCHIVED[sid]
                self.assertEqual(url.hostname, 'web.archive.org')
                self.assertTrue(url.path.startswith(f'/web/{stamp}id_/http'), sid)
                self.assertLess(stamp, '20260907')
                self.assertEqual(source['url'].split('id_/', 1)[1].replace(':80/', '/', 1), source['original_url'])
                self.assertEqual(extract['original_url'], source['original_url'])
                self.assertEqual(extract['archive_capture_utc'].replace('-', '').replace(':', '').replace('T', '').rstrip('Z'),
                                 stamp)
                self.assertIn('Raw Internet Archive capture', extract['provenance_note'])
                self.assertIn('no automatic decoding', extract['fetch_recipe'])
            else:
                self.assertEqual(url.hostname, 'sgip3.tse.jus.br')
                self.assertNotIn('original_url', source)
                self.assertNotIn('archive_capture_utc', extract)
        self.assertEqual(len({self.sources[s]['snapshot']['path'] for s in NEW_SOURCES}), len(NEW_SOURCES))
        codes = {v: k for k, v in TITLES.items()}
        got = {cid: (row['review_observation'], row['attested_on'], row['event_kind'], row['holder_name'],
                     codes.get(row['role_title']))
               for cid, row in self.rows.items() if cid in EVENTS}
        self.assertEqual(got, EVENTS)
        self.assertEqual(list(got), NEW_CLAIMS)

    def test_changed_registry_response_preserves_original_and_limits(self):
        extract = self.extracts['br_mdb_tse_sgip_cen_2013_2019']
        prior = extract['submitted_response_identity']
        self.assertEqual((prior['accessed_date'], prior['source_response_bytes'], prior['source_response_sha256']),
                         ('2026-09-28', 18952, '1d00f3018ea6d2fdc9f863cff6762edee15cd546dd791880f6c75dcb0b2b2247'))
        self.assertIn('18952 bytes', prior['stability_check'])
        self.assertIn(prior['stability_check'], prior['provenance_note'])
        review = extract['current_response_review']
        self.assertEqual(review['reviewed_date'], '2026-09-30')
        self.assertEqual(len(review['requests']), 3)
        self.assertEqual(len({r['finished_utc'] for r in review['requests']}), 3)
        for r in review['requests']:
            self.assertEqual((r['bytes'], r['sha256'], r['http_status']),
                             (18945, 'f6a0c4cacda5f5ce2e5021c74a923880eb6f2a9d270b2fb80bac1aed4c17b17d', 200))
        self.assertIs(review['submitted_raw_body_recovered'], False)
        self.assertEqual(review['cause_of_change'], 'unknown')
        self.assertEqual(review['whole_body_equivalence'], 'unknown')
        for key in ('permanent_stability_claimed', 'uncited_fields_accepted', 'holders_or_dates_changed'):
            self.assertIs(review[key], False)
        self.assertEqual(review['directly_checked_claim_ids'], [r['claim_id'] for r in extract['rows']])
        self.assertEqual(len(review['directly_checked_claim_ids']), 2)
        self.assertTrue(all(row['event_kind'].startswith('registry_') and row['attested_on'] is None
                            for row in extract['rows']))
        self.assertIn('whole-body equivalence remain unknown', extract['stability_check'])

    def test_no_per_request_url_no_secondary_lead_and_no_personal_registry_data(self):
        self.assertEqual({urlsplit(self.sources[s]['url']).hostname for s in NEW_SOURCES}, HOSTS)
        for sid in NEW_SOURCES:
            url = self.sources[sid]['url']
            self.assertIsNone(PER_REQUEST_URL.search(url), sid)
            if urlsplit(url).hostname == 'sgip3.tse.jus.br':
                self.assertRegex(url, r'/orgaoPartidario/comAnotacoesEMembros\?idOrgaoPartidario=\d+&isMembrosAtivos=false$')
        for source in self.packet['sources']:
            for marker in LEAD_URL_MARKERS:
                self.assertNotIn(marker, source['url'], (source['id'], marker))
        # Only closed SGIP organs are recorded; the organs in force are leads.
        self.assertEqual(sorted(SGIP_ORGANS.values()), sorted(['70945', '291052', '70996', '269142', '70981', '248141',
                                                              '275827', '405173']))
        for sid, organ in SGIP_ORGANS.items():
            self.assertTrue(self.sources[sid]['url'].endswith(f'idOrgaoPartidario={organ}&isMembrosAtivos=false'))
            self.assertIn('Não Vigente', self.extracts[sid]['stability_check'])
        for sid in NEW_SOURCES:
            text = (research.ROOT / self.sources[sid]['snapshot']['path']).read_text(encoding='utf-8')
            for marker in ('nrCpf', '"cpf"', 'nrTitulo', 'dsEmail', 'nrTelefone', 'des_raca_cor', 'des_genero'):
                self.assertNotIn(marker, text, (sid, marker))
            self.assertIsNone(re.search(r'\b\d{3}\.\d{3}\.\d{3}-\d{2}\b|\b\d{11}\b', text), sid)
        leads, added = self.section('Leads not imported'), self.section('Sources added')
        for marker in ('lideres-historicos', 'g1-lupi-diz-que-se-licenciou', 'idOrgaoPartidario=406194',
                       'idOrgaoPartidario=468918', 'not_direita', 'pdt280604', 'api/v1/partidos'):
            self.assertIn(marker, leads, marker)
            self.assertNotIn(marker, added, marker)

    def test_packet_formatting_is_preserved(self):
        data = (research.ROOT / research.RESEARCH / 'brazil.json').read_bytes()
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

        def holder(packet, role_id, index):
            return role_of(packet, role_id)['holder_claims'][index]

        def president(packet):
            return packet['institutions'][0]['roles'][0]

        def cite(packet, role_id, index, cid, sid):
            holder(packet, role_id, index)['claim_ids'].append(cid)
            if sid not in holder(packet, role_id, index)['sources']:
                holder(packet, role_id, index)['sources'].append(sid)

        def extra(name, day, sid, cid, start=None):
            return {'name': name, 'attested_on': None if start else day, 'from': start, 'until': None,
                    'sources': [sid], 'claim_ids': [cid], 'note': 'n', 'uncertainty': 'u'}

        validator_cases = [
            (lambda p: source(p, 'br_pdt_home_20040621')['snapshot'].update(sha256='0' * 64), 'checksum mismatch'),
            (lambda p: source(p, 'br_mdb_tse_sgip_cen_2019_2023')['snapshot'].update(bytes=1), 'checksum mismatch'),
            (lambda p: source(p, 'br_pfl_tse_uniao_registro_20220208')['snapshot'].update(sha256='f' * 64), 'checksum mismatch'),
            (lambda p: source(p, 'br_mdb_executiva_convocada_df_20260608')['snapshot'].update(path=REPORT.as_posix()), 'escapes'),
            (lambda p: holder(p, PDT, 6).update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: holder(p, MDB, 4).update(until='2026-09-08'), 'exceeds cutoff'),
            (lambda p: claim(p, 'br_pdt_lupi_styled_presidente_nacional_20260904').update(attested_on='2026-09-08'),
             'exceeds cutoff'),
            (lambda p: claim(p, 'br_pfl_acm_neto_styled_presidente_nacional_20220112').update(attested_on='2026-09-10'),
             'exceeds cutoff'),
            (lambda p: holder(p, PDT, 2).update({'from': '2004-06-22', 'attested_on': None}), 'Reversed historical interval'),
            (lambda p: holder(p, MDB, 0)['claim_ids'].append('br_mdb_temer_styled_presidente_nacional_20160329'),
             'cited source'),
            (lambda p: role_of(p, PDT)['claim_ids'].append('br_pdt_does_not_exist'), 'Unknown'),
            (lambda p: entry(p, MDB_ORG).update(represented_party_ids=['Brazil/br_pmdb']), 'foreign represented party'),
            (lambda p: entry(p, PDT_ORG).update(represented_party_ids=['Brazil/br_pdt']), 'foreign represented party'),
        ]
        for change, message in validator_cases:
            with self.subTest(message=message), self.assertRaisesRegex(ValueError, message):
                self.validate(mutated(change))
        temer_vp = copy.deepcopy(next(h for h in self.packet['institutions'][0]['roles'][1]['holder_claims']
                                      if h['name'] == 'Michel Temer'))
        rule_cases = [
            ("successor's interim styling used as an end (Brizola, Lupi 2004)", lambda p: holder(p, PDT, 2).update(until='2004-07-06')),
            ('death reported next day cited as the end (Brizola)', lambda p: cite(
                p, PDT, 2, 'br_pdt_curitiba_reports_death_of_presidente_nacional', 'br_pdt_curitiba_perda_20040622')),
            ('death end removed while the death claim stays cited', lambda p: holder(p, PDT, 2).update(until=None)),
            ("successor's election used as an end (Temer, Baleia Rossi)", lambda p: holder(p, MDB, 1).update(until='2019-10-06')),
            ('leave used as an end (Temer 2016)', lambda p: holder(p, MDB, 1).update(until='2016-04-05')),
            ('leave used as an end (Lupi 2021)', lambda p: holder(p, PDT, 4).update(until='2023-02-02')),
            ('renaming used as a boundary (Temer 2016)', lambda p: holder(p, MDB, 1).update(until='2017-12-19')),
            ('registry end used as an end (Baleia Rossi 2019)', lambda p: holder(p, MDB, 2).update(until='2023-10-06')),
            ('election date used as start (Baleia Rossi 2019)', lambda p: holder(p, MDB, 2).update({'from': '2019-10-06', 'attested_on': None})),
            ('re-election used as start (Temer 2016)', lambda p: holder(p, MDB, 1).update({'from': '2016-03-12', 'attested_on': None})),
            ('executive posse on an undated roster used as start (Temer 2010)', lambda p: role_of(p, MDB)['holder_claims'].insert(1, extra(
                'Michel Temer', None, 'br_mdb_executiva_roster_capt2011', 'br_mdb_executive_posse_20100310', start='2010-03-10'))),
            ('retrospective assumption day used as start (Lupi 2004, on his first holder)', lambda p: holder(p, PDT, 3).update({'from': '2004-06-28', 'attested_on': None})),
            ('death of the predecessor used as start (Lupi 2004, on his first holder)', lambda p: holder(p, PDT, 3).update({'from': '2004-06-21', 'attested_on': None})),
            ('return from leave used as start (Lupi 2025)', lambda p: holder(p, PDT, 5).update({'from': '2025-05-20', 'attested_on': None})),
            ('registry start used as start (Lupi 2019)', lambda p: holder(p, PDT, 4).update({'from': '2019-03-18', 'attested_on': None})),
            ('election claim cited by a holder (Baleia Rossi 2023)', lambda p: cite(
                p, MDB, 3, 'br_mdb_baleia_reelected_20231005', 'br_mdb_convencao_20231005')),
            ('continuation claim cited by a holder (Lupi 2021)', lambda p: cite(
                p, PDT, 4, 'br_pdt_lupi_signs_communique_as_presidente_nacional_20220110', 'br_pdt_convencao_virtual_20220110')),
            ('Lupi 2004 interim styling restored as a holder', lambda p: role_of(p, PDT)['holder_claims'].insert(3, extra(
                'Carlos Lupi', '2004-07-06', 'br_pdt_home_20040706', 'br_pdt_lupi_presidente_nacional_home_20040706'))),
            ('acting service added as a holder (André Figueiredo)', lambda p: role_of(p, PDT)['holder_claims'].insert(5, extra(
                'André Figueiredo', '2023-02-02', 'br_pdt_figueiredo_exercicio_20230202', 'br_pdt_figueiredo_announced_presidente_em_exercicio_20230202'))),
            ('acting service added as a holder (Romero Jucá 2016)', lambda p: role_of(p, MDB)['holder_claims'].insert(2, extra(
                'Romero Jucá', '2016-04-07', 'br_mdb_juca_em_exercicio_20160407', 'br_mdb_juca_styled_presidente_em_exercicio_20160407'))),
            ('acting-period styling added as a holder (Romero Jucá 2017)', lambda p: role_of(p, MDB)['holder_claims'].insert(2, extra(
                'Romero Jucá', '2017-11-23', 'br_mdb_edital_convencao_20171123', 'br_mdb_juca_signs_edital_as_presidente_nacional_20171123'))),
            ('interim service added as a holder (Iris de Araújo)', lambda p: role_of(p, MDB)['holder_claims'].insert(1, extra(
                'Iris de Araújo', '2009-03-10', 'br_mdb_iris_interina_20090310', 'br_mdb_iris_assumes_interim_presidency_20090310'))),
            ('conflicting styling added as a holder (Valdir Raupp 2014)', lambda p: role_of(p, MDB)['holder_claims'].insert(1, extra(
                'Valdir Raupp', '2014-05-14', 'br_mdb_raupp_presidente_nacional_20140514', 'br_mdb_raupp_styled_presidente_nacional_20140514'))),
            ('undated roster continuation added as a holder (Jader Barbalho)', lambda p: role_of(p, MDB)['holder_claims'].insert(0, extra(
                'Jader Barbalho', '1998-09-15', 'br_mdb_jader_presidente_page_1999', 'br_mdb_jader_elected_19980915'))),
            ('cross-role holder: a PDT holder added to the MDB role', lambda p: role_of(p, MDB)['holder_claims'].insert(
                1, copy.deepcopy(holder(p, PDT, 1)))),
            ('cross-role claim: an MDB claim moved onto the PDT role', lambda p: (
                role_of(p, PDT)['claim_ids'].append('br_mdb_temer_leave_20160405'),
                role_of(p, PDT)['sources'].append('br_mdb_comunicado_licenca_temer_20160405'))),
            ('cross-role holder: the Vice-Presidency of the Republic added to the party role',
             lambda p: role_of(p, MDB)['holder_claims'].insert(1, copy.deepcopy(temer_vp))),
            ('cross-role holder: a party president added to br_president',
             lambda p: president(p)['holder_claims'].insert(5, copy.deepcopy(holder(p, MDB, 1)))),
            ('cross-role claim: a presidency claim moved onto the MDB role', lambda p: (
                role_of(p, MDB)['claim_ids'].append('br_lula_posse_declared_20030101'),
                role_of(p, MDB)['sources'].append('br_dcn_1_2003_posse_20030101'))),
            ('cross-role claim: a party claim moved onto br_president', lambda p: (
                president(p)['claim_ids'].append('br_mdb_temer_styled_presidente_nacional_20160329'),
                president(p)['sources'].append('br_mdb_rompe_alianca_20160329'))),
            ('cross-role claim: a PT claim moved onto the PDT role', lambda p: (
                role_of(p, PDT)['claim_ids'].append('br_pt_edinho_empossado_20250803'),
                role_of(p, PDT)['sources'].append('br_pt_edinho_posse_speech_20250803'))),
            ('PFL/DEM claim cited by the UNIÃO observation', lambda p: (
                entry(p, UNIAO_ORG)['claim_ids'].append('br_pfl_tse_approves_uniao_registration_fusion_20220208'),
                entry(p, UNIAO_ORG)['sources'].append('br_pfl_tse_uniao_registro_20220208'))),
            ('PFL/DEM presidency placed as a role on the UNIÃO observation', lambda p: entry(p, UNIAO_ORG)['roles'].append({
                'id': 'br_pfl_president', 'title': TITLES['F'], 'kind': 'party_leader',
                'sources': ['br_pfl_dem_informe_acm_neto_20220112'],
                'claim_ids': ['br_pfl_acm_neto_styled_presidente_nacional_20220112'], 'holder_claims': [], 'scope_note': 's'})),
            ('cross-institution: the PDT role copied onto another organization',
             lambda p: p['organizations'][4]['roles'].append(copy.deepcopy(role_of(p, PDT)))),
            ('cross-institution: the MDB role placed on the presidency institution',
             lambda p: p['institutions'][0]['roles'].append(copy.deepcopy(role_of(p, MDB)))),
            ('second party role on the MDB observation', lambda p: entry(p, MDB_ORG)['roles'].append(
                dict(copy.deepcopy(role_of(p, MDB)), id='br_mdb_vice_president'))),
            ('party role removed', lambda p: entry(p, PDT_ORG)['roles'].clear()),
            ('MDB lifecycle given a start', lambda p: entry(p, MDB_ORG)['lifecycle'].update({'from': '1981-06-03'})),
            ('UNIÃO lifecycle given a start', lambda p: entry(p, UNIAO_ORG)['lifecycle'].update({'from': '2022-02-08'})),
            ('retrospective span given a date', lambda p: claim(p, 'br_mdb_list_jader_mandate_1998_2001').update(
                attested_on='1998-09-15')),
            ('printed civil name used as the holder name', lambda p: holder(p, MDB, 2).update(name='Luiz Felipe Baleia Tenuto Rossi')),
            ('holders out of order', lambda p: role_of(p, PDT)['holder_claims'].reverse()),
        ]
        party_rules(self.packet, self.rows)
        party_invariants(self.packet, self.rows)
        for label, change in rule_cases:
            with self.subTest(label=label):
                packet = mutated(change)
                with self.assertRaises((AssertionError, KeyError, IndexError, ValueError)):
                    party_rules(packet, self.rows)
                with self.assertRaises((AssertionError, KeyError, IndexError, ValueError)):
                    party_invariants(packet, self.rows)
        # Restoring Lupi's 2004 interim styling as a holder fails on the never-holder date even with its row's kind
        # restored to an in-office attestation.
        rows = copy.deepcopy(self.rows)
        rows['br_pdt_lupi_presidente_nacional_home_20040706']['event_kind'] = 'in_office_attestation'
        packet = mutated(lambda p: role_of(p, PDT)['holder_claims'].insert(3, extra(
            'Carlos Lupi', '2004-07-06', 'br_pdt_home_20040706', 'br_pdt_lupi_presidente_nacional_home_20040706')))
        with self.subTest(label='Lupi 2004 interim styling restored as a holder, row kind restored'):
            with self.assertRaisesRegex(AssertionError, 'Carlos Lupi'):
                party_rules(packet, rows)
            with self.assertRaises(AssertionError):
                party_invariants(packet, rows)
        # Event collapses are caught by the pinned events.
        for cid, day in (('br_mdb_convention_elects_executive_baleia_20191006', '2019-10-17'),
                         ('br_pdt_lupi_returns_to_presidency_20250520', '2025-05-21'),
                         ('br_pdt_curitiba_reports_death_of_presidente_nacional', '2004-06-22'),
                         ('br_mdb_convention_votes_renaming_20171219', '2018-05-15'),
                         ('br_pfl_fusion_convention_scheduled_prospective_20211005', '2021-10-06')):
            with self.subTest(collapsed=cid), self.assertRaises(AssertionError):
                party_invariants(mutated(lambda p: claim(p, cid).update(attested_on=day)), self.rows)

    def test_report_and_handoff_close_no_parent_gate(self):
        self.assertIn('ready_for_review', self.report)
        table = self.section('Outcome')
        decisions = dict.fromkeys(REVIEW, 'Accepted in part')
        for code in ('PFL-PRES-01', 'PFL-PRES-02', 'PFL-PRES-03', 'PFL-PRES-04'):
            decisions[code] = 'Claims only'
        for code, decision in decisions.items():
            row, = [line for line in table.splitlines() if line.startswith(f'| {code} ')]
            self.assertIn(f'**{decision}:**', row)
        for marker in ('C01', 'C06', 'S23', 'WC1', 'CP1'):
            self.assertNotRegex(self.report, rf'\b{marker}\b[^.\n]*\bis (now )?complete\b')
        notes = self.section('Integration notes')
        for text in ('c2ff9396', '032cd6a3', 'not stacked', 'research-index.json', 'only file shared',
                     'test_brazil_research_s10f.py', 'test_brazil_presidents_c01_10.py',
                     'test_brazil_vice_presidents_c01_17.py', 'test_brazil_pt_presidents_c01_22.py', 'census.json',
                     'test_certified_gap_ledger.py', 'test_certified_boundary_matrix.py'):
            self.assertIn(text, notes)
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        for text in ('ready_for_review', 'brazil-party-presidents-1990-2026-34.md', 'claude/c01-br-34', 'c2ff9396',
                     '032cd6a3', 'test_brazil_party_presidents_c01_34.py'):
            self.assertIn(text, handoff)
        index = research.build()
        country = next(p for p in index['countries'] if p['nation'] == 'Brazil')
        self.assertFalse(country['country_census_complete'])
        self.assertIsNone(country['unrepresented_organization_count'])
        self.assertEqual((country['institution_observations'], country['role_observations'], country['source_claims']),
                         (1, 6, 556))
        self.assertEqual(country['mapping_pending'], 32)
        self.assertEqual({w['status'] for w in index['work_orders'] if w['nation'] == 'Brazil'}, {'open'})
        self.assertFalse(index['c01_complete'])


if __name__ == '__main__':
    unittest.main()
