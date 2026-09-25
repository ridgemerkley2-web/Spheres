"""CLAUDE-C01-22: the Workers' Party (PT) national presidency, 1990-2026, is a party office on the PT funding
observation, kept apart from the Presidency of the Republic. National meetings, direct elections and their counts,
Diretório Nacional selections, posse, resignations and leaves, interim and acting service and in-office attestations
stay separate claims; a holder has a start or an end only where a party, Chamber or electoral-court record states
the day, and acting or interim service is never a holder."""
import copy
import hashlib
import json
import re
import unittest
from urllib.parse import urlsplit

import campaign_research as research
import test_brazil_vice_presidents_c01_17 as vp

ORG_ID = 'br_tse_fefc_2024_party_03'
ROLE = 'br_pt_president'
T_PRES = 'Presidente Nacional do Partido dos Trabalhadores'
T_ACT = 'Presidente Nacional do Partido dos Trabalhadores em exercício ou interino'
T_VICE = 'Vice-Presidente Nacional do Partido dos Trabalhadores'
# The packet's sources before this packet: the S10f intake (5), CLAUDE-C01-10 (48) and CLAUDE-C01-17 (11).
EARLIER_SOURCE_COUNT = 64
# The PT observation's lifecycle, funding record and earlier coverage notes, which this packet must not change.
LIFECYCLE = {"status": "unresearched", "from": None, "until": None, "note": "A sourced observation or decision date does not establish organizational foundation, dissolution or a complete continuous status interval."}
FUNDING = {"election_year": 2024, "release_date": "2024-08-20", "release_cell_as_displayed": "20.8.2024", "attested_on": "2024-08-20", "from": None, "until": None, "amount": None, "consolidated_current_status": None}
ORG_UNRESOLVED = ["Reconcile the exact source identity and separately dated name variants before any game mapping.", "Research organizational history and separate party, parliamentary and executive offices from 1990 through the fixed cutoff.", "No leadership, succession eligibility, likeness, portrait-reuse permission or complete present-day status is established."]

# New sources, in packet order, with the original response identity recorded in each extract: (bytes, sha256).
RESPONSES = {
    'br_pt_fpa_dn4_composition':
        (73201, '5dd0c53872bad09ae8648b3558d8ca1b4e1946a250a6e8254e4be406d90de694'),
    'br_pt_fpa_dn5_composition':
        (24224, 'ac9d36d61967905fd64198011c383ab6f7f61acc6441bee0cdfb1ce1d33b3e0c'),
    'br_pt_fpa_dn6_composition':
        (66843, 'd636ec6d2360e6b2c3fc5ca4865f7666d89c8549119eed7f5029a9dbba94a2ed'),
    'br_pt_fpa_dn7_composition':
        (80176, 'e1740c2360ef4c73ff0a053c4acdccb7dcea87abc82c5ba4207c9f9821117fb5'),
    'br_pt_fpa_dn8_composition':
        (66311, '2ec679d8d6a0a5b026e8a1e191d5dd717a89bddad1ae80eb36f21ea8e47f8cdf'),
    'br_pt_fpa_dn9_composition':
        (21991, 'a2f0a3ebb75531f8c93b7f9974419099d762143aeeffa6a61d3a335d7958eb23'),
    'br_pt_fpa_dn10_composition':
        (112487, 'aaf8e614c7ffc416720e96cbbb978d5e3e3334aeef08fdf1b47689112d11cb75'),
    'br_pt_bn049_199003':
        (5871191, '8c842d384cbfac3d7f62091ce7316acd067cfe45b57b2a971c3db998e3f6eac2'),
    'br_pt_bn050_199005':
        (8076668, 'c91b27e11d85d24fa50356d97a1185b5b020706df17a989ec384461c9b37bfee'),
    'br_pt_bn051_199007':
        (6234294, '1674cdeffb554f9ce742fca4b6e5d29412d3d291896428f725d2bf69ff891c8b'),
    'br_pt_csbh_resumo_7_encontro_1990':
        (201799, 'df83aaa57858f67d760acb02b5f265c5f511a79e387b6f5ffa7fe629e6b8b600'),
    'br_pt_bn085_199404':
        (1134723, '2cb7d3d14528d535bd92a47f9d79463500b190263303e1884d44f8fded21dca0'),
    'br_pt_csbh_resumo_9_encontro_1994':
        (188888, '291b85bc389c9f9518b30479a8e5733585a381580af912473804ed8a9247bd6a'),
    'br_pt_bn087_199405':
        (2387248, '6ff517df605367a19991138040d7d4708016e906e05a5bb0694c30372187d3b4'),
    'br_pt_bn091_199410':
        (1835767, '3428336f9524e2fdcbec5ebaef13772fe236a498160466f679ca4769e1cbb4a8'),
    'br_pt_bn_edicao_especial_199412':
        (7589722, '4f13a575d873f58ccc877f3ca7fd136d6f95ad569a7d16754d431b9fc0b38bec'),
    'br_pt_site_rui_falcao_biography_20170617':
        (53135, 'c2ea56104bcd950c91d84442c25251cb6ed3a6b5a5330fbab47849d552908d50'),
    'br_pt_csbh_resumo_10_encontro_1995':
        (196092, 'e14102e9595baffbc2e44327f0533bfd72e2aafce75db888deef1c2f31859fa7'),
    'br_pt_sncr_oficio_001_19950824':
        (894821, 'ee797a76abf271b121aa899df867566083603111ef6c3757b4df3a13e317d085'),
    'br_pt_csbh_resumo_11_encontro_1997':
        (192624, '5571ab78298f1a3fe29298d212772ab21be4b3213f55b15c27f03dddf68c7d07'),
    'br_pt_noticias_051_19970910':
        (12285885, 'e91bc278b940cf8db5b6bd08dc99c39887f3b5adf39fba322d9f66cc71790b37'),
    'br_pt_noticias_052_19970920':
        (11754811, 'b673c856ec5ab4fb64d4ac4e6ea02bad72be65ca968a0d695738bcf59c08111d'),
    'br_pt_noticias_087_19991216':
        (9375590, '4434183f0924746390fb4283c6389615ff899912bbbd9acadc9ba0c00fb68ad5'),
    'br_pt_presidencia_comunicacao_19991202':
        (473863, 'fd1e18d727067affa65dab0a09ac47bd8795ca2ad543dce18f73120ef0e3008d'),
    'br_pt_noticias_105_20010720':
        (4815117, '9e68759d6a21412c52e3665bc1cb28977be5558f0f0acc45778f9a53fac85f20'),
    'br_pt_noticias_106_20010804':
        (6038454, '3bf47b0f59e5bcda4f9fb80551519c68f498e8640f791293dcbd6ff5dfa57ab0'),
    'br_pt_noticias_107_20010819':
        (16049882, '8065054bac4318ace891cab8b779917fbe70318946738dcc57a9b8cc3d45b397'),
    'br_pt_noticias_108_20010910':
        (9061147, 'f57c7a0c6146f04aac8afaf27e524a20d6bc1bd1fb4b06290538b16c2d7aa1f3'),
    'br_pt_noticias_109_20011010':
        (5198077, '9710168f531276f812d11fca731e4898699ea22eadf486aea6263be69f51a497'),
    'br_pt_noticias_ee12_200201':
        (8801111, '52c7550e5f80c7d8b0806a1b90fbbcd6287cee5af827eaf26133a9da107e5e02'),
    'br_pt_noticias_124_20021216':
        (7539258, '4e42a824ef5c3110d10a1c8a5689be75737860ecb875f250847b23c80c5164ef'),
    'br_pt_camara_bio_gushiken_20250807':
        (39743, '1281ac6f52da4edd448e7f4cb727c65d160fd1abd424783b1abcc0a86a404734'),
    'br_pt_camara_bio_dirceu_20251108':
        (40033, 'af0740c7a0391644382f585b9918ec905f7610ef4d3a8e8f07d72b918e96e40c'),
    'br_pt_portal_genoino_congress_talks_20030108':
        (21287, '8fa5b50f03dda07eb538172447447aa1cec2fc2631ac7ab89602cb56c0026afc'),
    'br_pt_official_note_lauro_campos_20030113':
        (20306, '9671c1a0a4f39d856b5a8625da81ecbd98377a5428202d678ee00c35e519ed5a'),
    'br_pt_portal_genoino_governor_affiliation_20030318':
        (22950, '07397ece60fe3c77c326129631f18356bdd7652d15bff49c9a111f0ca008e658'),
    'br_pt_portal_genoino_leaves_presidency_20050709':
        (32460, '57489335a5ca08517447b4a8892b15ca064926bdb950f7fd3817321664f1d0d2'),
    'br_pt_portal_reactions_genoino_departure_20050709':
        (26028, '4d9c918ac7c0b947d53d70e03c2959c5302b0b018a59399d9d33c414d4ef037d'),
    'br_pt_portal_tarso_genro_new_president_20050709':
        (26893, 'acda68ac428e5c84aa5b832904534b6555481d0b4a656a90bb79e9137dd0c3ed'),
    'br_pt_portal_tarso_first_press_conference_20050709':
        (28683, 'ba66c4cbd11c5a919b1fa1d00de49451fd0e938f8aa50bdc5e3356d6702177f0'),
    'br_pt_portal_tarso_after_cen_meeting_20050710':
        (31370, '7e136a44536dee8c8ec494a793d6ca082ce55cd44aabb39ddcbf59901ccfd168'),
    'br_pt_portal_ped2005_voting_closed_20050918':
        (22879, 'e0804b14d975e57b02cc011d0b8ca2f77973295a3c318bbeb4096b8021e407a6'),
    'br_pt_portal_ped2005_first_partial_bulletin_20050919':
        (59110, 'e2b97f4d5f2f00998f9052c6a66c41d41ded1b091d5b399248688bb9b094c668'),
    'br_pt_portal_posse_date_anticipated_20050919':
        (22790, '815b0e3493a6b18d54790f5e4a8a92b10f684c94474d11504bb3c6ac0c26a3e8'),
    'br_pt_portal_ped2005_second_round_closed_20051009':
        (23334, '5c0061d8f708d27a6e1eea4fc30c611639f94ae249400c2b2ad88515f892ab32'),
    'br_pt_portal_ped2005_second_round_first_partial_20051010':
        (36655, '604527cfc8b273cbb1011ca4019c66f3c17bcfc3aceef7ebd2952ad8cce0265b'),
    'br_pt_portal_berzoini_new_president_20051011':
        (36620, '8a4b934b339b7c3a78cbde62a6d40af0025ca33143f5729e25697f1062d178d2'),
    'br_pt_portal_ped2005_berzoini_elected_20051013':
        (38217, '702c93c0ad4ad24a15202f95544e71393dec2a90533ba8b74438e40a5172a071'),
    'br_pt_portal_tarso_announces_final_results_20051013':
        (23435, 'ad125fce8513deb8b4b64606d9dc6d88e99cc8bbf1940c3d605c1c9188998b27'),
    'br_pt_portal_tarso_meets_pcdob_psb_20051014':
        (21821, '6898d30c6cf563079ebffba321e03235a03f22e8536e3fff1000bb712c052371'),
    'br_pt_portal_dn_posse_new_executive_20051022':
        (33668, 'e0b5abccc7b431fd4948c1a0bcd74ad4190f362e2ceef6ab3ac930ae3fece5f4'),
    'br_pt_resolution_campaign_coordination_20060428':
        (22565, '91223796b8e4be6207e65c7744fb0a0c5e93f950cf15131ea3522f3d042782e0'),
    'br_pt_site_cen_members_20061127':
        (40667, '2e43f2665b6841cb76c5a99dc622a0009ace965304757c7cba95a60e03dca2de'),
    'br_pt_dn_guidelines_3rd_congress_20061226':
        (26168, 'c42aaf594b0d4ebd3ed9e8b3ede6b073723e35ad445e4fc244f31786543bf657'),
    'br_pt_site_cen_members_20070103':
        (40727, 'adbaa2b75c1a9e825772f5065ba224adfcbdd841a94e14b00f4079f64ae54f01'),
    'br_pt_portal_berzoini_weekly_interview_20070305':
        (63862, '50d90211a2ea5c9a4a6eac4227359c595592acabe62a22ff7491934c4d3f3b49'),
    'br_pt_site_cen_members_20070313':
        (40131, 'e2c35c491c13239e55c69f222f7e3d4f12613e44b9fc6a44bc44d84b285c2298'),
    'br_pt_portal_berzoini_votes_ped2007_20071216':
        (22541, '14e38550b1f01c822d1eb4ee7c7ed2427f201f83bc4832ee1a7eea7dea206d57'),
    'br_pt_cen_resolution_ped2007_result_20071219':
        (23056, '3ebe5a41a5d40bad4dbad237ff4712a1765f0c80ce2ec24aab6000eaba727b49'),
    'br_pt_portal_cen_members_2008_2009_20091113':
        (19440, '4935adf2e1bf460dbe05be0fd91efcd94030e56f28f78530d3538377834f2244'),
    'br_pt_portal_berzoini_press_conference_ped2009_20091124':
        (21947, 'fbc58c10646faba58fca5d75ed3b28c6ee37cbb83b771bc1b082547e83deb270'),
    'br_pt_portal_dutra_elected_20091125':
        (21925, '59300480dc1a10c75baaaa0495f1b198f7d36bf36ab554f6aa830bd2ae144f27'),
    'br_pt_portal_ped2009_final_result_20091202':
        (25425, '964ba1e891d2607d2c1eb467f8cb8f85ea5142dce800c24140e1ee97ba431945'),
    'br_pt_portal_ped2009_posse_rule_20100111':
        (20479, 'c9cd883d7f2a8afdffef5160eb317456dfe38139abc40f1be582c95c779afa73'),
    'br_pt_camara_dcd_berzoini_farewell_20100211':
        (6444492, '48098bf9bcc91c4db679e84b6434cf8247087ab8d2bfe8618cc9153d521dd9a8'),
    'br_pt_portal_berzoini_farewell_reported_20100211':
        (33841, 'e33ad62e682ad8867be66b9aa5d2fa2db5fd115063274781b63cc8b818c2cd61'),
    'br_pt_portal_iv_congress_posse_programme_20100219':
        (22090, '03e5d8cd242221ceefeb53e3478a812c28f9844e6c442dd39211c93f3febf6fe'),
    'br_pt_portal_dn_posse_dutra_20100219':
        (25662, '9eaa13b5b80429c27202992973570283431bf34b606b0255b51663cac12e5f4e'),
    'br_pt_portal_dn_defines_new_cen_20100220':
        (22646, '41d50b70109f530050fc6001d2ab37a504f0a08cb059746500e4fe13a6d66b6d'),
    'br_pt_portal_dutra_dn_meeting_20110426':
        (23735, '1aea07b6fc1a074236e7778c58cafbea6aa39c49494c484c0ae42bc6343d5c38'),
    'br_pt_portal_novo_comando_20110508':
        (25987, 'a5599021756e7f7cde10c7e70fdebc3e6a9d23c0ad3e317f81506d0f4446ebd8'),
    'br_pt_portal_falcao_president_20110615':
        (25552, '1c440eef0bb0463d9fd1148948eafb85341eede44f13a481377835b352595413'),
    'br_pt_ped2013_falcao_reelected_20131112':
        (26772, '8ec431d6a99bb6f114782ce0374c3bcd0c480efad5b33b7bcce5b2ca0473b9e1'),
    'br_pt_dn_resolution_ped2013_posse_20131118':
        (25649, '2e7aea03a84561c953dbbc5f1324579b8738266e7d46793a48eab5835b6c3e30'),
    'br_pt_portal_guia_5th_congress_20131210':
        (26097, 'a902f4c5b76c7cc0556cb1580f664d1b9ba486768907f6e1984e834969f57e14'),
    'br_pt_portal_dn_elects_cen_20131211':
        (26619, '8c495bc5ecaac78de27e81ecd10d616b61239c79fcf9b1468d700ffa435eacd3'),
    'br_pt_falcao_posse_speech_20131212':
        (44714, 'd068ba43a1db841ba160e419386f53085efb593e2ed2f15f2426becc784be0d7'),
    'br_pt_portal_dn_composition_20131218':
        (28165, 'f7c0e3cc8030988c0229f6f952432516173331691f0845926dad725e096d9d77'),
    'br_pt_tse_sgip_dn_2014_2017':
        (41754, '4b7a8986eecaa5525195f9b54a02245a12fa494337d6cfa36de035fb40de601b'),
    'br_pt_tse_sgip_cen_2014_2017':
        (18797, '723f11fd942f494415cf35a1712c2a6cd4eaa17a3610d614f0c1431464f982ca'),
    'br_pt_agencia_5th_congress_falcao_20150612':
        (55662, 'dced8c74fe33ba8f40f83d4926dcde79a902884d1223c1c0c65458452d674c49'),
    'br_pt_dn_page_falcao_20160805':
        (96335, '3efa4283b7e0175cd34d5e6d977d47e50306c7873cb1cde7c773c5cbbae92e6b'),
    'br_pt_falcao_balance_20170527':
        (65582, 'bb3b8bf766d688649fbb91efeefa93eed95a746d08760f231668c1e7986d6381'),
    'br_pt_gleisi_elected_6th_congress_20170603':
        (67962, 'd50070c00044b97db0e974a679520273963c5fa2f2432cabb6619f5340245448'),
    'br_pt_new_directorate_gleisi_posse_20170705':
        (62459, 'cda5ae16282b8ac93a28960a4596d90a007a48755c5bf275d5f402d8ee06ac5b'),
    'br_pt_lula_at_gleisi_posse_20170705':
        (68205, 'e72dc6234a5bf58cac188638af4d3919bd28f7b627988e9243638428f273a883'),
    'br_pt_tse_sgip_dn_2017_2020':
        (43974, 'c132f13247a4e3feec59546fe38d9481b346638368a3140612d6076d1a7317a3'),
    'br_pt_dn_page_gleisi_20180225':
        (182595, '95dd6e108d98d7fdc2324dff6e60dffb68ba13d932b2ef6f7027c7af472726ac'),
    'br_pt_gleisi_reelected_7th_congress_20191124':
        (62315, '61dd6ddb6c42d356f8375f853ed5ad0d6773b742e990899440ed835da9dbb4c1'),
    'br_pt_gleisi_reelected_text_20191124':
        (71267, 'f5d73d2d91459c857f304d4ddf6a78527eb853981aefcc1af0cf29aab8277a17'),
    'br_pt_7th_congress_new_leadership_20191126':
        (66164, 'c958459b902b8f50dd96ef6c8b2be39970b2d9a2fc6fdfa71dc3ecf72244b371'),
    'br_pt_tse_sgip_dn_2020_2025':
        (65506, 'd53c2fdf6f51ba64b6e3a18e1530a57851f999dded38a65a00702d7c578bad62'),
    'br_pt_dn_page_gleisi_20200401':
        (174032, '40e6c71522bc6dceedfbe1c8118d8d126d323e38216f8c165f51b9e0ced9f8bd'),
    'br_pt_portal_gleisi_sri_congratulations_20250228':
        (98654, 'bb6c575d40e425cd4064405b2ba51a2c1c1135b7454343cffaa63306354cd576'),
    'br_pt_humberto_interim_executive_20250307':
        (98815, 'bfdf4e5a4521f5ef4f8a22e23a6b965d4531adcdcd3ec1b494e3c971ff7e19ea'),
    'br_pt_dn_page_humberto_20250308':
        (46464, 'a223ce2bbda0f88740410afec39c5de9ba4782abf4dde3cfd8da15053ec1e77c'),
    'br_pt_humberto_priorities_20250312':
        (97522, '6c10728b5aeaebea0971fed0c010dc4ca1fef528d70fefe5f1953dd24ffc5834'),
    'br_pt_portal_humberto_ped_mobilization_20250313':
        (95733, '86ee396348e1d2d8a2bad33fe70d0ef9b6eec2facdfc9c6bf4c5c112a6dab56a'),
    'br_pt_dn_elects_humberto_20250320':
        (96231, '2c38caf5ba68327c429996031ba70fa2ddf88908781ec80846a4ac247685f7d5'),
    'br_pt_portal_humberto_reiterates_unity_20250320':
        (99562, '4e732da3e22d1baf50f4b0aaf0d624da4ab641741119beaea63cdd0fb957784f'),
    'br_pt_edinho_elected_ped_20250707':
        (108049, 'f4df9d159f610032926756c334a063b0338edff7b27d79e957f19aa10e553a7d'),
    'br_pt_ped2025_totalization_20250715':
        (263758, '220aaa111e66cb7b93af478fed9e452cd65e0a14b8782e2e5142f07fefbb51fe'),
    'br_pt_ped2025_press_conference_20250716':
        (125726, 'ca88b20c5853ad3c3406f7962682c49d5136e578b14d6a79b5e8d9af83b97e73'),
    'br_pt_encontro_posse_notice_20250723':
        (91839, 'abcdce0ea5298c48bb7ed844733ea24e3543d0f6ec70bde3962a8803daa6bebe'),
    'br_pt_portal_edinho_speech_report_20250803':
        (101233, '4b68321f31d227caa846f913ab0ff948a816600942b016aca877feadacec2425'),
    'br_pt_edinho_posse_speech_20250803':
        (116902, '38bde2b43baf6f03cd116f3bc47a22d17cb91f6f3f985f0557d4b2764eabf228'),
    'br_pt_dn_page_edinho_20250917':
        (40639, 'c179b4311e2e0f87c97129c1830b73297e2e3876956670cb1108c99f2dcbffdb'),
    'br_pt_dn_page_edinho_20260815':
        (159613, '37fedb84a70816598fe4cbc19526bc6055b779362390c509e6eb8c57f9c0657b'),
}
# Capture timestamp of each raw id_ capture (all before the 7 September 2026 cutoff).
ARCHIVED = {
    'br_pt_site_rui_falcao_biography_20170617': '20170617175854',
    'br_pt_camara_bio_gushiken_20250807': '20250807161350',
    'br_pt_camara_bio_dirceu_20251108': '20251108014215',
    'br_pt_portal_genoino_congress_talks_20030108': '20050125095249',
    'br_pt_official_note_lauro_campos_20030113': '20050125095624',
    'br_pt_portal_genoino_governor_affiliation_20030318': '20050104165955',
    'br_pt_portal_genoino_leaves_presidency_20050709': '20050928133658',
    'br_pt_portal_reactions_genoino_departure_20050709': '20050928133953',
    'br_pt_portal_tarso_genro_new_president_20050709': '20050928111258',
    'br_pt_portal_tarso_first_press_conference_20050709': '20050928134100',
    'br_pt_portal_tarso_after_cen_meeting_20050710': '20050928133821',
    'br_pt_portal_ped2005_voting_closed_20050918': '20050928033518',
    'br_pt_portal_ped2005_first_partial_bulletin_20050919': '20050928030718',
    'br_pt_portal_posse_date_anticipated_20050919': '20050928031210',
    'br_pt_portal_ped2005_second_round_closed_20051009': '20051013032429',
    'br_pt_portal_ped2005_second_round_first_partial_20051010': '20051012233642',
    'br_pt_portal_berzoini_new_president_20051011': '20051012220941',
    'br_pt_portal_ped2005_berzoini_elected_20051013': '20051020093922',
    'br_pt_portal_tarso_announces_final_results_20051013': '20051020093610',
    'br_pt_portal_tarso_meets_pcdob_psb_20051014': '20051020094509',
    'br_pt_portal_dn_posse_new_executive_20051022': '20051104104844',
    'br_pt_resolution_campaign_coordination_20060428': '20101007190733',
    'br_pt_site_cen_members_20061127': '20061127061706',
    'br_pt_dn_guidelines_3rd_congress_20061226': '20101007190715',
    'br_pt_site_cen_members_20070103': '20070103183836',
    'br_pt_portal_berzoini_weekly_interview_20070305': '20070314010600',
    'br_pt_site_cen_members_20070313': '20070313174805',
    'br_pt_portal_berzoini_votes_ped2007_20071216': '20101007070507',
    'br_pt_cen_resolution_ped2007_result_20071219': '20101007190624',
    'br_pt_portal_cen_members_2008_2009_20091113': '20091113060550',
    'br_pt_portal_berzoini_press_conference_ped2009_20091124': '20091128101545',
    'br_pt_portal_dutra_elected_20091125': '20091129120106',
    'br_pt_portal_ped2009_final_result_20091202': '20091205215819',
    'br_pt_portal_ped2009_posse_rule_20100111': '20101007164713',
    'br_pt_portal_berzoini_farewell_reported_20100211': '20101007225905',
    'br_pt_portal_iv_congress_posse_programme_20100219': '20100223022527',
    'br_pt_portal_dn_posse_dutra_20100219': '20100223022603',
    'br_pt_portal_dn_defines_new_cen_20100220': '20100223074538',
    'br_pt_portal_dutra_dn_meeting_20110426': '20110430124011',
    'br_pt_portal_novo_comando_20110508': '20110511191926',
    'br_pt_portal_falcao_president_20110615': '20110620074204',
    'br_pt_ped2013_falcao_reelected_20131112': '20131115052616',
    'br_pt_dn_resolution_ped2013_posse_20131118': '20131122032523',
    'br_pt_portal_guia_5th_congress_20131210': '20131211165915',
    'br_pt_portal_dn_elects_cen_20131211': '20131219155157',
    'br_pt_falcao_posse_speech_20131212': '20131219161742',
    'br_pt_portal_dn_composition_20131218': '20131223013204',
    'br_pt_agencia_5th_congress_falcao_20150612': '20150615034739',
    'br_pt_dn_page_falcao_20160805': '20160805185002',
    'br_pt_falcao_balance_20170527': '20181011062306',
    'br_pt_gleisi_elected_6th_congress_20170603': '20170605161800',
    'br_pt_new_directorate_gleisi_posse_20170705': '20170707180315',
    'br_pt_lula_at_gleisi_posse_20170705': '20170707180319',
    'br_pt_dn_page_gleisi_20180225': '20180225173344',
    'br_pt_gleisi_reelected_7th_congress_20191124': '20191221151511',
    'br_pt_gleisi_reelected_text_20191124': '20191220070514',
    'br_pt_7th_congress_new_leadership_20191126': '20191221000310',
    'br_pt_dn_page_gleisi_20200401': '20200401220443',
    'br_pt_portal_gleisi_sri_congratulations_20250228': '20250301032709',
    'br_pt_humberto_interim_executive_20250307': '20250308082549',
    'br_pt_dn_page_humberto_20250308': '20250308122611',
    'br_pt_humberto_priorities_20250312': '20250313042802',
    'br_pt_portal_humberto_ped_mobilization_20250313': '20250314021449',
    'br_pt_dn_elects_humberto_20250320': '20250321015920',
    'br_pt_portal_humberto_reiterates_unity_20250320': '20250321212958',
    'br_pt_edinho_elected_ped_20250707': '20260608175940',
    'br_pt_ped2025_totalization_20250715': '20250717145537',
    'br_pt_ped2025_press_conference_20250716': '20260815121147',
    'br_pt_encontro_posse_notice_20250723': '20251014224334',
    'br_pt_portal_edinho_speech_report_20250803': '20250804063953',
    'br_pt_edinho_posse_speech_20250803': '20251015000332',
    'br_pt_dn_page_edinho_20250917': '20250917095950',
    'br_pt_dn_page_edinho_20260815': '20260815061427',
}
# Captures the archive stores and serves gzip-encoded: decoded identity (bytes, sha256), recorded beside the served one.
GZIP = {
    'br_pt_dn_page_humberto_20250308': (209451, 'd8488fc5216703dd6d5a0ad1abf37a563688dee6e3ef64f703259de3b04d758e'),
    'br_pt_dn_page_edinho_20250917': (190467, '49b4b012814c1f13aa328cac9d0f1622942e58dad35297ca4caccfc6420cce9e'),
}
# PDF pages rendered and visually reviewed.
PDF_PAGES = {
    'br_pt_bn049_199003': [1, 6, 12],
    'br_pt_bn050_199005': [8],
    'br_pt_bn051_199007': [1, 5, 7, 9],
    'br_pt_bn085_199404': [3],
    'br_pt_bn087_199405': [2],
    'br_pt_bn091_199410': [2],
    'br_pt_bn_edicao_especial_199412': [2, 4],
    'br_pt_sncr_oficio_001_19950824': [1],
    'br_pt_noticias_051_19970910': [1],
    'br_pt_noticias_052_19970920': [1],
    'br_pt_noticias_087_19991216': [2, 7],
    'br_pt_presidencia_comunicacao_19991202': [1],
    'br_pt_noticias_105_20010720': [1],
    'br_pt_noticias_108_20010910': [1],
    'br_pt_noticias_109_20011010': [1, 2, 3, 7],
    'br_pt_noticias_ee12_200201': [1],
    'br_pt_noticias_124_20021216': [1],
    'br_pt_ped2025_totalization_20250715': [1],
}
# Every new claim, in packet order: (review observation, attested_on, event_kind, holder_name, title code).
EVENTS = {
    'br_pt_fpa_cen_altered_presidente_gushiken_19881210_11':
        ('PT-PRES-01', None, 'executive_committee_change', "Luiz Gushiken", 'P'),
    'br_pt_fpa_dn5_elected_7th_encontro_presidente_lula_19900601_03':
        ('PT-PRES-01', None, 'national_meeting_directorate_election', "Luiz Inácio Lula da Silva", 'P'),
    'br_pt_fpa_cen_elected_presidente_lula_19900715':
        ('PT-PRES-01', "1990-07-15", 'executive_committee_selection', "Luiz Inácio Lula da Silva", 'P'),
    'br_pt_fpa_cen_altered_presidente_lula_19920713':
        ('PT-PRES-01', "1992-07-13", 'executive_committee_change', "Luiz Inácio Lula da Silva", 'P'),
    'br_pt_fpa_dn6_elected_8th_encontro_presidente_lula_19930611_13':
        ('PT-PRES-01', None, 'national_meeting_directorate_election', "Luiz Inácio Lula da Silva", 'P'),
    'br_pt_fpa_cen_elected_presidente_lula_19930613':
        ('PT-PRES-01', "1993-06-13", 'executive_committee_selection', "Luiz Inácio Lula da Silva", 'P'),
    'br_pt_fpa_dn7_elected_10th_encontro_presidente_dirceu_19950818_20':
        ('PT-PRES-02', None, 'national_meeting_directorate_election', "José Dirceu", 'P'),
    'br_pt_fpa_cen_approved_presidente_dirceu_19951028_29':
        ('PT-PRES-02', None, 'executive_committee_selection', "José Dirceu", 'P'),
    'br_pt_fpa_cen_altered_early_1997_presidente_dirceu':
        ('PT-PRES-03', None, 'executive_committee_change', "José Dirceu", 'P'),
    'br_pt_fpa_dn8_elected_11th_encontro_presidente_dirceu_19970828_30':
        ('PT-PRES-03', None, 'national_meeting_directorate_election', "José Dirceu", 'P'),
    'br_pt_fpa_cen_elected_presidente_dirceu_19970920_21':
        ('PT-PRES-03', None, 'executive_committee_selection', "José Dirceu", 'P'),
    'br_pt_fpa_cen_1999_2001_presidente_dirceu':
        ('PT-PRES-03', None, 'retrospective_composition_list', "José Dirceu", 'P'),
    'br_pt_fpa_dn10_elected_12th_encontro_presidente_dirceu_20011214_16':
        ('PT-PRES-03', None, 'national_meeting_directorate_election', "José Dirceu", 'P'),
    'br_pt_fpa_dn_genoino_substitutes_dirceu_interim_20021207':
        ('PT-PRES-03', "2002-12-07", 'interim_substitution', "José Genoino", 'A'),
    'br_pt_fpa_dn_dirceu_resigns_national_presidency_20030315_16':
        ('PT-PRES-03', None, 'resignation_day_not_stated', "José Dirceu", 'P'),
    'br_pt_fpa_dn_genoino_elected_national_president_20030315':
        ('PT-PRES-04', "2003-03-15", 'directorate_election', "José Genoino", 'P'),
    'br_pt_fpa_cen_dirceu_to_federal_government_printed_20020110':
        ('PT-PRES-03', None, 'left_for_state_office_listing', "José Dirceu", 'P'),
    'br_pt_bn049_gushiken_presidente_nacional_meeting_19900216_18':
        ('PT-PRES-01', None, 'in_office_period_attestation', "Luiz Gushiken", 'P'),
    'br_pt_bn049_gushiken_signs_official_note_haiti_199003':
        ('PT-PRES-01', None, 'in_office_period_attestation', "Luiz Gushiken", 'P'),
    'br_pt_bn050_gushiken_presidente_nacional_theses_199005':
        ('PT-PRES-01', None, 'in_office_period_attestation', "Luiz Gushiken", 'P'),
    'br_pt_bn051_vii_encontro_reconducts_lula_19900531_0603':
        ('PT-PRES-01', None, 'national_meeting_election', "Luiz Inácio Lula da Silva", 'P'),
    'br_pt_bn051_lula_reconducted_closing_speech_19900603':
        ('PT-PRES-01', "1990-06-03", 'in_office_attestation', "Luiz Inácio Lula da Silva", 'P'),
    'br_pt_bn051_lula_states_he_assumes_presidency_19900603':
        ('PT-PRES-01', "1990-06-03", 'stated_assumption_of_office', "Luiz Inácio Lula da Silva", 'P'),
    'br_pt_bn051_lula_receives_baton_from_gushiken_19900603':
        ('PT-PRES-01', "1990-06-03", 'handover_statement', "Luiz Gushiken", 'P'),
    'br_pt_csbh_resumo_vii_encontro_elects_5th_dn_19900531_0603':
        ('PT-PRES-01', None, 'national_meeting_summary', None, 'P'),
    'br_pt_bn085_lula_styled_presidente_do_pt_199404':
        ('PT-PRES-01', None, 'in_office_period_attestation', "Luiz Inácio Lula da Silva", 'P'),
    'br_pt_bn085_rui_falcao_vice_president_199404':
        ('PT-PRES-02', None, 'vice_president_attestation', "Rui Falcão", 'V'),
    'br_pt_csbh_resumo_ix_encontro_19940429_0501':
        ('PT-PRES-02', None, 'national_meeting_summary', None, 'P'),
    'br_pt_bn087_editorial_signed_rui_falcao_presidente_nacional_199405':
        ('PT-PRES-02', None, 'in_office_period_attestation', "Rui Falcão", 'P'),
    'br_pt_bn091_editorial_signed_rui_falcao_presidente_nacional_199410':
        ('PT-PRES-02', None, 'in_office_period_attestation', "Rui Falcão", 'P'),
    'br_pt_bn091_gushiken_former_national_president_199410':
        ('PT-PRES-01', None, 'retrospective_former_holder', "Luiz Gushiken", 'P'),
    'br_pt_bnee94_rui_falcao_opens_seminar_as_president_19941125':
        ('PT-PRES-02', "1994-11-25", 'in_office_attestation', "Rui Falcão", 'P'),
    'br_pt_site_bio_rui_falcao_president_in_1994':
        ('PT-PRES-02', None, 'retrospective_term_span', "Rui Falcão", 'P'),
    'br_pt_csbh_resumo_x_encontro_presidential_vote_19950818_20':
        ('PT-PRES-02', None, 'election_result', "José Dirceu", 'P'),
    'br_pt_sncr_oficio_addressed_to_president_dirceu_19950824':
        ('PT-PRES-02', "1995-08-24", 'in_office_attestation', "José Dirceu", 'P'),
    'br_pt_csbh_resumo_xi_encontro_19970828_30':
        ('PT-PRES-03', None, 'national_meeting_summary', None, 'P'),
    'br_pt_ptn051_xi_encontro_reconducts_dirceu_19970829_31':
        ('PT-PRES-03', None, 'national_meeting_election', "José Dirceu", 'P'),
    'br_pt_ptn051_presidential_vote_dirceu_284_temer_256_19970831':
        ('PT-PRES-03', "1997-08-31", 'election_vote_result', "José Dirceu", 'P'),
    'br_pt_ptn051_dirceu_proclaimed_national_president_199708':
        ('PT-PRES-03', None, 'result_declaration', "José Dirceu", 'P'),
    'br_pt_ptn051_masthead_presidente_nacional_dirceu_19970910_16':
        ('PT-PRES-03', None, 'in_office_period_attestation', "José Dirceu", 'P'),
    'br_pt_ptn052_dirceu_national_president_at_vitoria_meeting_19970917':
        ('PT-PRES-03', "1997-09-17", 'in_office_attestation', "José Dirceu", 'P'),
    'br_pt_ptn052_masthead_presidente_nacional_dirceu_19970920_26':
        ('PT-PRES-03', None, 'in_office_period_attestation', "José Dirceu", 'P'),
    'br_pt_ptn087_ii_congresso_held_19991124_28':
        ('PT-PRES-03', None, 'national_meeting_held', None, 'P'),
    'br_pt_ptn087_result_known_dirceu_reelected_19991128':
        ('PT-PRES-03', "1999-11-28", 'election_result_declared', "José Dirceu", 'P'),
    'br_pt_ptn087_dirceu_assumed_third_term_during_congresso_199911':
        ('PT-PRES-03', None, 'stated_assumption_day_not_printed', "José Dirceu", 'P'),
    'br_pt_ptn087_masthead_presidente_nacional_dirceu_19991216_20000105':
        ('PT-PRES-03', None, 'in_office_period_attestation', "José Dirceu", 'P'),
    'br_pt_presidencia_letter_signed_dirceu_19991202':
        ('PT-PRES-03', "1999-12-02", 'in_office_attestation', "José Dirceu", 'P'),
    'br_pt_ptn105_dirceu_leave_to_run_for_reelection_20010717':
        ('PT-PRES-03', "2001-07-17", 'leave_of_absence', "José Dirceu", 'P'),
    'br_pt_ptn105_masthead_genoino_em_exercicio_20010720_0803':
        ('PT-PRES-03', None, 'acting_service', "José Genoino", 'A'),
    'br_pt_ptn106_genoino_signs_note_as_acting_president_20010803':
        ('PT-PRES-03', "2001-08-03", 'acting_service', "José Genoino", 'A'),
    'br_pt_ptn107_dirceu_signs_as_president_on_leave_200108':
        ('PT-PRES-03', None, 'leave_of_absence', "José Dirceu", 'P'),
    'br_pt_ptn107_genoino_signs_note_as_acting_president_20010816':
        ('PT-PRES-03', "2001-08-16", 'acting_service', "José Genoino", 'A'),
    'br_pt_ptn108_genoino_acting_national_president_20010901':
        ('PT-PRES-03', "2001-09-01", 'acting_service', "José Genoino", 'A'),
    'br_pt_ptn108_masthead_genoino_em_exercicio_20010910_25':
        ('PT-PRES-03', None, 'acting_service', "José Genoino", 'A'),
    'br_pt_ptn109_ped_vote_held_20010916':
        ('PT-PRES-03', "2001-09-16", 'direct_election_vote', None, 'P'),
    'br_pt_ped_commission_note_totalization_failures_20010920':
        ('PT-PRES-03', "2001-09-20", 'result_process_note', None, 'P'),
    'br_pt_ptn109_ped_totalization_dirceu_elected_20010927':
        ('PT-PRES-03', "2001-09-27", 'election_result_declared', "José Dirceu", 'P'),
    'br_pt_ptn109_dirceu_resumed_party_leadership_20011001':
        ('PT-PRES-03', "2001-10-01", 'return_from_leave', "José Dirceu", 'P'),
    'br_pt_ptn109_masthead_genoino_em_exercicio_20011010_25':
        ('PT-PRES-03', None, 'acting_service', "José Genoino", 'A'),
    'br_pt_ptn109_interview_dirceu_reassumed_presidency_200110':
        ('PT-PRES-03', None, 'stated_resumption_of_office', "José Dirceu", 'P'),
    'br_pt_ptn109_dirceu_signs_note_as_president_20011004':
        ('PT-PRES-03', "2001-10-04", 'in_office_attestation', "José Dirceu", 'P'),
    'br_pt_ptn109_dirceu_signs_note_as_president_20011006':
        ('PT-PRES-03', "2001-10-06", 'in_office_continuation_attestation', "José Dirceu", 'P'),
    'br_pt_ptn109_dirceu_signs_official_note_as_president_20011010':
        ('PT-PRES-03', "2001-10-10", 'in_office_continuation_attestation', "José Dirceu", 'P'),
    'br_pt_ptn_ee12_dirceu_presidente_nacional_at_cultural_opening_20011213':
        ('PT-PRES-03', "2001-12-13", 'in_office_continuation_attestation', "José Dirceu", 'P'),
    'br_pt_ptn124_dn_acclaims_genoino_replacing_dirceu_20021207':
        ('PT-PRES-03', "2002-12-07", 'selection_by_directorate', "José Genoino", 'P'),
    'br_pt_ptn124_dirceu_on_leave_for_lula_government_200212':
        ('PT-PRES-03', None, 'leave_of_absence', "José Dirceu", 'P'),
    'br_pt_ptn124_retrospective_fifth_president_count_200212':
        ('PT-PRES-01', None, 'retrospective_holder_count', None, 'P'),
    'br_pt_camara_bio_gushiken_presidente_dn_1988_1990':
        ('PT-PRES-01', None, 'retrospective_term_span', "Luiz Gushiken", 'P'),
    'br_pt_camara_bio_dirceu_presidente_pt_1995_1997_1998_2002':
        ('PT-PRES-03', None, 'retrospective_term_span', "José Dirceu", 'P'),
    'br_pt_genoino_styled_president_interim_period_20030108':
        ('PT-PRES-04', "2003-01-08", 'styled_president_during_interim_period', "José Genoino", 'P'),
    'br_pt_genoino_signs_official_note_interim_period_20030113':
        ('PT-PRES-04', "2003-01-13", 'styled_president_during_interim_period', "José Genoino", 'P'),
    'br_pt_genoino_attends_affiliation_as_pt_president_20030318':
        ('PT-PRES-04', "2003-03-18", 'in_office_attestation', "José Genoino", 'P'),
    'br_pt_genoino_departure_announcement_reported_20050709':
        ('PT-PRES-04', "2005-07-09", 'departure_announcement_reported', "José Genoino", 'P'),
    'br_pt_genoino_hands_office_to_dn_20050709':
        ('PT-PRES-04', "2005-07-09", 'office_handover_statement', "José Genoino", 'P'),
    'br_pt_genoino_describes_act_as_leave_20050709':
        ('PT-PRES-04', "2005-07-09", 'leave_statement', "José Genoino", 'P'),
    'br_pt_genoino_recalls_30_months_in_presidency':
        ('PT-PRES-04', None, 'retrospective_term_length_statement', "José Genoino", 'P'),
    'br_pt_genoino_departure_request_reported_20050709':
        ('PT-PRES-04', "2005-07-09", 'departure_request_reported', "José Genoino", 'P'),
    'br_pt_dn_approves_tarso_genro_for_presidency_20050709':
        ('PT-PRES-05', "2005-07-09", 'national_directorate_selection', "Tarso Genro", 'P'),
    'br_pt_empossados_until_ped_statement_20050709':
        ('PT-PRES-05', "2005-07-09", 'prospective_tenure_statement', None, 'P'),
    'br_pt_tarso_styled_new_president_press_conference_20050709':
        ('PT-PRES-05', "2005-07-09", 'styled_on_selection_day', "Tarso Genro", 'P'),
    'br_pt_tarso_reports_as_president_after_cen_meeting_20050710':
        ('PT-PRES-05', "2005-07-10", 'in_office_attestation', "Tarso Genro", 'P'),
    'br_pt_ped2005_first_round_ballot_20050918':
        ('PT-PRES-05', "2005-09-18", 'ped_ballot', None, 'P'),
    'br_pt_tarso_styled_president_releasing_ped_bulletin_20050919':
        ('PT-PRES-05', "2005-09-19", 'in_office_continuation_attestation', "Tarso Genro", 'P'),
    'br_pt_ped2005_first_round_partial_berzoini_leads_20050919':
        ('PT-PRES-05', "2005-09-19", 'ped_partial_result', None, 'P'),
    'br_pt_cen_amends_ped_rule_on_posse_20050919':
        ('PT-PRES-05', "2005-09-19", 'procedure_rule_amendment', None, 'P'),
    'br_pt_ped2005_second_round_ballot_20051009':
        ('PT-PRES-05', "2005-10-09", 'ped_ballot', None, 'P'),
    'br_pt_ped2005_second_round_first_partial_20051010':
        ('PT-PRES-05', "2005-10-10", 'ped_partial_result', None, 'P'),
    'br_pt_ped2005_coordinator_says_berzoini_president_elect_20051011':
        ('PT-PRES-05', "2005-10-11", 'result_announcement', "Ricardo Berzoini", 'P'),
    'br_pt_ped2005_berzoini_elected_declared_20051013':
        ('PT-PRES-05', "2005-10-13", 'result_declaration', "Ricardo Berzoini", 'P'),
    'br_pt_ped2005_leadership_to_take_office_20051022_prospective_20051013':
        ('PT-PRES-05', "2005-10-13", 'assumption_scheduled', "Ricardo Berzoini", 'P'),
    'br_pt_tarso_styled_president_berzoini_virtual_elect_20051013':
        ('PT-PRES-05', "2005-10-13", 'in_office_continuation_attestation', "Tarso Genro", 'P'),
    'br_pt_tarso_styled_national_president_20051014':
        ('PT-PRES-05', "2005-10-14", 'in_office_continuation_attestation', "Tarso Genro", 'P'),
    'br_pt_dn_gives_posse_to_berzoini_20051022':
        ('PT-PRES-05', "2005-10-22", 'posse_reported', "Ricardo Berzoini", 'P'),
    'br_pt_new_cen_lists_berzoini_presidency_20051022':
        ('PT-PRES-05', "2005-10-22", 'in_office_attestation', "Ricardo Berzoini", 'P'),
    'br_pt_berzoini_styled_national_president_20060428':
        ('PT-PRES-06', "2006-04-28", 'in_office_continuation_attestation', "Ricardo Berzoini", 'P'),
    'br_pt_marco_aurelio_garcia_interim_president_listed_20061127':
        ('PT-PRES-06', "2006-11-27", 'interim_service_attestation', "Marco Aurélio Garcia", 'A'),
    'br_pt_dn_guidelines_name_president_marco_aurelio_garcia_20061226':
        ('PT-PRES-06', "2006-12-26", 'interim_service_attestation', "Marco Aurélio Garcia", 'A'),
    'br_pt_marco_aurelio_garcia_interim_president_listed_20070103':
        ('PT-PRES-06', "2007-01-03", 'interim_service_attestation', "Marco Aurélio Garcia", 'A'),
    'br_pt_berzoini_styled_national_president_20070305':
        ('PT-PRES-06', "2007-03-05", 'in_office_continuation_attestation', "Ricardo Berzoini", 'P'),
    'br_pt_berzoini_listed_president_cen_page_20070313':
        ('PT-PRES-06', "2007-03-13", 'in_office_continuation_attestation', "Ricardo Berzoini", 'P'),
    'br_pt_berzoini_styled_president_and_candidate_ped2007_20071216':
        ('PT-PRES-06', "2007-12-16", 'in_office_continuation_attestation', "Ricardo Berzoini", 'P'),
    'br_pt_ped2007_second_round_ballot_20071216':
        ('PT-PRES-06', "2007-12-16", 'ped_ballot', None, 'P'),
    'br_pt_cen_salutes_ped2007_participation_20071219':
        ('PT-PRES-06', "2007-12-19", 'ped_participation_resolution', None, 'P'),
    'br_pt_berzoini_listed_president_cen_biennium_2008_2009_20091113':
        ('PT-PRES-06', "2009-11-13", 'in_office_attestation', "Ricardo Berzoini", 'P'),
    'br_pt_berzoini_styled_national_president_20091124':
        ('PT-PRES-06', "2009-11-24", 'in_office_continuation_attestation', "Ricardo Berzoini", 'P'),
    'br_pt_ped2009_partial_count_dutra_leads_20091124':
        ('PT-PRES-06', "2009-11-24", 'ped_partial_result', None, 'P'),
    'br_pt_ped2009_ballot_20091122':
        ('PT-PRES-06', "2009-11-22", 'ped_ballot', None, 'P'),
    'br_pt_dutra_declared_president_elect_20091125':
        ('PT-PRES-06', "2009-11-25", 'result_declaration', "José Eduardo Dutra", 'P'),
    'br_pt_dutra_posse_scheduled_february_2010_20091125':
        ('PT-PRES-06', "2009-11-25", 'assumption_scheduled', "José Eduardo Dutra", 'P'),
    'br_pt_ped2009_final_result_dutra_20091202':
        ('PT-PRES-06', "2009-12-02", 'final_result_declaration', "José Eduardo Dutra", 'P'),
    'br_pt_cen_amends_ped2009_posse_rule_20091207':
        ('PT-PRES-06', "2009-12-07", 'procedure_rule_amendment', None, 'P'),
    'br_pt_berzoini_states_last_day_of_mandate_20100210':
        ('PT-PRES-06', "2010-02-10", 'term_last_day_statement', "Ricardo Berzoini", 'P'),
    'br_pt_berzoini_recalls_leading_first_interim_2005':
        ('PT-PRES-05', None, 'retrospective_statement', "Ricardo Berzoini", 'P'),
    'br_pt_deputy_aparte_berzoini_president_2005_2007_2007_2010':
        ('PT-PRES-06', None, 'retrospective_term_span', "Ricardo Berzoini", 'P'),
    'br_pt_portal_reports_berzoini_farewell_speech_20100210':
        ('PT-PRES-06', "2010-02-10", 'farewell_speech_reported', "Ricardo Berzoini", 'P'),
    'br_pt_iv_congress_posse_scheduled_evening_20100219':
        ('PT-PRES-06', "2010-02-19", 'assumption_scheduled', None, 'P'),
    'br_pt_dutra_empossado_president_20100219':
        ('PT-PRES-06', "2010-02-19", 'posse_reported', "José Eduardo Dutra", 'P'),
    'br_pt_berzoini_leaves_presidency_statement_20100219':
        ('PT-PRES-06', "2010-02-19", 'predecessor_departure_statement', "Ricardo Berzoini", 'P'),
    'br_pt_dutra_posse_speech_lists_past_pt_presidents':
        ('PT-PRES-06', None, 'retrospective_list', None, 'P'),
    'br_pt_new_cen_lists_dutra_president_20100220':
        ('PT-PRES-06', "2010-02-20", 'in_office_continuation_attestation', "José Eduardo Dutra", 'P'),
    'br_pt_falcao_styled_acting_president_20110426':
        ('PT-PRES-07', "2011-04-26", 'acting_service', "Rui Falcão", 'A'),
    'br_pt_dutra_styled_president_pending_decision_20110426':
        ('PT-PRES-07', "2011-04-26", 'in_office_continuation_attestation', "José Eduardo Dutra", 'P'),
    'br_pt_dutra_resignation_20110429':
        ('PT-PRES-07', "2011-04-29", 'resignation_stated', "José Eduardo Dutra", 'P'),
    'br_pt_falcao_elected_unanimously_2011':
        ('PT-PRES-07', None, 'directorate_election', "Rui Falcão", 'P'),
    'br_pt_falcao_styled_new_national_president_20110508':
        ('PT-PRES-07', "2011-05-08", 'in_office_attestation', "Rui Falcão", 'P'),
    'br_pt_falcao_styled_president_20110615':
        ('PT-PRES-07', "2011-06-15", 'in_office_continuation_attestation', "Rui Falcão", 'P'),
    'br_pt_ped2013_vote_held_20131110':
        ('PT-PRES-07', "2013-11-10", 'direct_election_vote', None, 'P'),
    'br_pt_ped2013_falcao_reelection_announced_20131112':
        ('PT-PRES-07', "2013-11-12", 'result_declaration_partial', "Rui Falcão", 'P'),
    'br_pt_falcao_styled_national_president_20131112':
        ('PT-PRES-07', "2013-11-12", 'in_office_continuation_attestation', "Rui Falcão", 'P'),
    'br_pt_dn_resolution_posse_window_20131118':
        ('PT-PRES-07', "2013-11-18", 'posse_window_resolved', None, 'P'),
    'br_pt_guia_schedules_president_posse_20131210':
        ('PT-PRES-07', "2013-12-10", 'posse_scheduled', "Rui Falcão", 'P'),
    'br_pt_dn_elects_and_inducts_cen_falcao_president_20131211':
        ('PT-PRES-07', "2013-12-11", 'executive_committee_selection', "Rui Falcão", 'P'),
    'br_pt_falcao_posse_speech_published_20131212':
        ('PT-PRES-07', "2013-12-12", 'posse_speech_published', "Rui Falcão", 'P'),
    'br_pt_dn_composition_lists_falcao_president_20131218':
        ('PT-PRES-07', "2013-12-18", 'in_office_attestation', "Rui Falcão", 'P'),
    'br_pt_tse_sgip_falcao_dn_registered_exercise_20140219_20170908':
        ('PT-PRES-07', None, 'registry_exercise_period', "Rui Falcão", 'P'),
    'br_pt_tse_sgip_falcao_cen_registered_exercise_20140219_20171009':
        ('PT-PRES-07', None, 'registry_exercise_period', "Rui Falcão", 'P'),
    'br_pt_falcao_styled_national_president_5th_congress_20150612':
        ('PT-PRES-07', "2015-06-12", 'in_office_continuation_attestation', "Rui Falcão", 'P'),
    'br_pt_dn_page_falcao_presidente_nacional_20160805':
        ('PT-PRES-07', "2016-08-05", 'in_office_continuation_attestation', "Rui Falcão", 'P'),
    'br_pt_falcao_balance_retrospective_span':
        ('PT-PRES-07', None, 'retrospective_span', "Rui Falcão", 'P'),
    'br_pt_gleisi_elected_6th_congress_20170603':
        ('PT-PRES-08', "2017-06-03", 'congress_election', "Gleisi Hoffmann", 'P'),
    'br_pt_gleisi_posse_20170705':
        ('PT-PRES-08', "2017-07-05", 'posse_reported', "Gleisi Hoffmann", 'P'),
    'br_pt_gleisi_posse_reported_lula_20170705':
        ('PT-PRES-08', "2017-07-05", 'in_office_attestation', "Gleisi Hoffmann", 'P'),
    'br_pt_tse_sgip_gleisi_registered_exercise_20170909_20200117':
        ('PT-PRES-08', None, 'registry_exercise_period', "Gleisi Hoffmann", 'P'),
    'br_pt_dn_page_gleisi_presidenta_20180225':
        ('PT-PRES-08', "2018-02-25", 'in_office_continuation_attestation', "Gleisi Hoffmann", 'P'),
    'br_pt_gleisi_reelected_7th_congress_20191124':
        ('PT-PRES-08', "2019-11-24", 'congress_reelection', "Gleisi Hoffmann", 'P'),
    'br_pt_gleisi_reelected_text_20191124':
        ('PT-PRES-08', "2019-11-24", 'congress_reelection', "Gleisi Hoffmann", 'P'),
    'br_pt_7th_congress_reelects_gleisi_20191124':
        ('PT-PRES-08', "2019-11-24", 'congress_reelection', "Gleisi Hoffmann", 'P'),
    'br_pt_tse_sgip_gleisi_registered_exercise_20200117_20250307':
        ('PT-PRES-08', None, 'registry_exercise_period', "Gleisi Hoffmann", 'P'),
    'br_pt_tse_sgip_humberto_registered_president_20250307_20250823':
        ('PT-PRES-09', None, 'registry_exercise_period', "Humberto Costa", 'P'),
    'br_pt_dn_page_gleisi_presidenta_20200401':
        ('PT-PRES-08', "2020-04-01", 'in_office_attestation', "Gleisi Hoffmann", 'P'),
    'br_pt_gleisi_styled_presidenta_nacional_20250228':
        ('PT-PRES-08', "2025-02-28", 'in_office_continuation_attestation', "Gleisi Hoffmann", 'P'),
    'br_pt_gleisi_departure_announced_20250307':
        ('PT-PRES-08', "2025-03-07", 'departure_announced', "Gleisi Hoffmann", 'P'),
    'br_pt_humberto_interim_designated_executive_20250307':
        ('PT-PRES-09', "2025-03-07", 'interim_designation', "Humberto Costa", 'A'),
    'br_pt_dn_page_gleisi_resignation_referenced':
        ('PT-PRES-08', None, 'resignation_referenced', "Gleisi Hoffmann", 'P'),
    'br_pt_dn_page_humberto_temporary_designation_20250307':
        ('PT-PRES-09', "2025-03-07", 'interim_designation', "Humberto Costa", 'A'),
    'br_pt_dn_page_humberto_heading_20250308':
        ('PT-PRES-09', "2025-03-08", 'interim_service_attestation', "Humberto Costa", 'A'),
    'br_pt_humberto_interim_confirmed_recited_20250307':
        ('PT-PRES-09', "2025-03-07", 'interim_designation_recited', "Humberto Costa", 'A'),
    'br_pt_humberto_styled_national_president_interim_20250312':
        ('PT-PRES-09', "2025-03-12", 'interim_service_attestation', "Humberto Costa", 'A'),
    'br_pt_humberto_styled_interim_national_president_20250313':
        ('PT-PRES-09', "2025-03-13", 'interim_service_attestation', "Humberto Costa", 'A'),
    'br_pt_humberto_assumed_interim_early_in_week':
        ('PT-PRES-09', None, 'interim_assumption_day_not_printed', "Humberto Costa", 'A'),
    'br_pt_dn_elects_humberto_20250320':
        ('PT-PRES-09', "2025-03-20", 'directorate_election', "Humberto Costa", 'P'),
    'br_pt_dn_ratifies_humberto_choice_20250320':
        ('PT-PRES-09', "2025-03-20", 'directorate_ratification', "Humberto Costa", 'P'),
    'br_pt_humberto_states_interim_to_effective_20250320':
        ('PT-PRES-09', "2025-03-20", 'stated_assumption_of_office', "Humberto Costa", 'P'),
    'br_pt_humberto_assumed_previous_week_recital':
        ('PT-PRES-09', None, 'interim_assumption_day_not_printed', "Humberto Costa", 'A'),
    'br_pt_ped2025_result_announced_20250707':
        ('PT-PRES-09', "2025-07-07", 'result_declaration_partial', "Edinho Silva", 'P'),
    'br_pt_humberto_styled_president_at_announcement_20250707':
        ('PT-PRES-09', "2025-07-07", 'in_office_continuation_attestation', "Humberto Costa", 'P'),
    'br_pt_ped2025_totalization_20250715':
        ('PT-PRES-09', "2025-07-15", 'result_totalization', "Edinho Silva", 'P'),
    'br_pt_edinho_styled_president_elect_20250716':
        ('PT-PRES-09', "2025-07-16", 'president_elect_styled', "Edinho Silva", 'P'),
    'br_pt_humberto_styled_current_president_20250716':
        ('PT-PRES-09', "2025-07-16", 'in_office_continuation_attestation', "Humberto Costa", 'P'),
    'br_pt_posse_scheduled_notice_20250723':
        ('PT-PRES-09', "2025-07-23", 'posse_scheduled', "Edinho Silva", 'P'),
    'br_pt_encontro_confirms_edinho_20250803':
        ('PT-PRES-09', "2025-08-03", 'nomination_confirmed_at_meeting', "Edinho Silva", 'P'),
    'br_pt_edinho_official_assumption_announced_for_20250804':
        ('PT-PRES-09', "2025-08-03", 'official_assumption_announced_prospective', "Edinho Silva", 'P'),
    'br_pt_edinho_empossado_20250803':
        ('PT-PRES-09', "2025-08-03", 'posse_reported', "Edinho Silva", 'P'),
    'br_pt_edinho_speech_thanks_humberto_20250803':
        ('PT-PRES-09', "2025-08-03", 'predecessor_reference', "Humberto Costa", 'P'),
    'br_pt_dn_page_edinho_ped_election_recited':
        ('PT-PRES-09', None, 'retrospective_election_recital', "Edinho Silva", 'P'),
    'br_pt_dn_page_edinho_presidente_20250917':
        ('PT-PRES-09', "2025-09-17", 'in_office_continuation_attestation', "Edinho Silva", 'P'),
    'br_pt_dn_page_edinho_presidente_nacional_20260815':
        ('PT-PRES-10', "2026-08-15", 'in_office_attestation', "Edinho Silva", 'P'),
}
HOLDER_CLAIMS = [
    ["br_pt_bn051_lula_reconducted_closing_speech_19900603", "br_pt_bn051_lula_states_he_assumes_presidency_19900603"],
    ["br_pt_bnee94_rui_falcao_opens_seminar_as_president_19941125"],
    ["br_pt_sncr_oficio_addressed_to_president_dirceu_19950824"],
    ["br_pt_ptn052_dirceu_national_president_at_vitoria_meeting_19970917"],
    ["br_pt_presidencia_letter_signed_dirceu_19991202"],
    ["br_pt_ptn109_dirceu_signs_note_as_president_20011004"],
    ["br_pt_genoino_attends_affiliation_as_pt_president_20030318"],
    ["br_pt_tarso_reports_as_president_after_cen_meeting_20050710"],
    ["br_pt_dn_gives_posse_to_berzoini_20051022", "br_pt_new_cen_lists_berzoini_presidency_20051022"],
    ["br_pt_berzoini_listed_president_cen_biennium_2008_2009_20091113"],
    ["br_pt_dutra_empossado_president_20100219", "br_pt_dutra_resignation_20110429"],
    ["br_pt_falcao_styled_new_national_president_20110508"],
    ["br_pt_dn_composition_lists_falcao_president_20131218"],
    ["br_pt_gleisi_posse_20170705", "br_pt_gleisi_posse_reported_lula_20170705"],
    ["br_pt_dn_page_gleisi_presidenta_20200401"],
    ["br_pt_humberto_states_interim_to_effective_20250320"],
    ["br_pt_edinho_empossado_20250803"],
    ["br_pt_dn_page_edinho_presidente_nacional_20260815"],
]
NEW_SOURCES = list(RESPONSES)
NEW_CLAIMS = list(EVENTS)
# Exact holder observations of br_pt_president, (name, attested_on, from, until), in chronological order.
HOLDERS = [
    ('Luiz Inácio Lula da Silva', None, '1990-06-03', None),
    ('Rui Falcão', '1994-11-25', None, None),
    ('José Dirceu', '1995-08-24', None, None),
    ('José Dirceu', '1997-09-17', None, None),
    ('José Dirceu', '1999-12-02', None, None),
    ('José Dirceu', '2001-10-04', None, None),
    ('José Genoino', '2003-03-18', None, None),
    ('Tarso Genro', '2005-07-10', None, None),
    ('Ricardo Berzoini', None, '2005-10-22', None),
    ('Ricardo Berzoini', '2009-11-13', None, None),
    ('José Eduardo Dutra', None, '2010-02-19', '2011-04-29'),
    ('Rui Falcão', '2011-05-08', None, None),
    ('Rui Falcão', '2013-12-18', None, None),
    ('Gleisi Hoffmann', None, '2017-07-05', None),
    ('Gleisi Hoffmann', '2020-04-01', None, None),
    ('Humberto Costa', None, '2025-03-20', None),
    ('Edinho Silva', None, '2025-08-03', None),
    ('Edinho Silva', '2026-08-15', None, None),
]
# The observation each holder answers, in holder order.
HOLDER_OBSERVATIONS = ['PT-PRES-01', 'PT-PRES-02', 'PT-PRES-02', 'PT-PRES-03', 'PT-PRES-03', 'PT-PRES-03', 'PT-PRES-04',
                       'PT-PRES-05', 'PT-PRES-05', 'PT-PRES-06', 'PT-PRES-06', 'PT-PRES-07', 'PT-PRES-07', 'PT-PRES-08',
                       'PT-PRES-08', 'PT-PRES-09', 'PT-PRES-09', 'PT-PRES-10']
FROM_KINDS = {'posse_reported', 'stated_assumption_of_office'}
UNTIL_KINDS = {'resignation_stated'}
SUPPORT_KINDS = {'in_office_attestation'}
HOLDER_KINDS = FROM_KINDS | UNTIL_KINDS | SUPPORT_KINDS
# Acting and interim service: claims only, never holders, never splitting or ending a holder.
ACTING_KINDS = {'acting_service', 'interim_substitution', 'interim_designation', 'interim_designation_recited',
                'interim_service_attestation', 'interim_assumption_day_not_printed'}
# Elections, selections, declarations, leaves, departures, registry periods and retrospective statements.
ELECTION_KINDS = {'national_meeting_directorate_election', 'executive_committee_selection', 'national_meeting_election',
                  'election_result', 'election_vote_result', 'result_declaration', 'election_result_declared',
                  'direct_election_vote', 'ped_ballot', 'ped_partial_result', 'result_announcement',
                  'final_result_declaration', 'result_declaration_partial', 'result_totalization', 'congress_election',
                  'congress_reelection', 'directorate_election', 'directorate_ratification',
                  'national_directorate_selection', 'selection_by_directorate', 'nomination_confirmed_at_meeting',
                  'president_elect_styled'}
DEPARTURE_KINDS = {'resignation_day_not_stated', 'leave_of_absence', 'leave_statement', 'office_handover_statement',
                   'departure_announcement_reported', 'departure_request_reported', 'departure_announced',
                   'resignation_referenced', 'predecessor_departure_statement', 'predecessor_reference',
                   'handover_statement', 'term_last_day_statement', 'farewell_speech_reported',
                   'left_for_state_office_listing'}
RETROSPECTIVE_KINDS = {'retrospective_term_span', 'retrospective_former_holder', 'retrospective_holder_count',
                       'retrospective_list', 'retrospective_term_length_statement', 'retrospective_composition_list',
                       'retrospective_span', 'retrospective_election_recital', 'retrospective_statement',
                       'registry_exercise_period'}
# The only structured end: José Eduardo Dutra's stated resignation. Berzoini's own 'último dia' is a claim.
ENDS = [('José Eduardo Dutra', '2011-04-29')]
STARTS = [('Luiz Inácio Lula da Silva', '1990-06-03'), ('Ricardo Berzoini', '2005-10-22'),
          ('José Eduardo Dutra', '2010-02-19'), ('Gleisi Hoffmann', '2017-07-05'), ('Humberto Costa', '2025-03-20'),
          ('Edinho Silva', '2025-08-03')]
# Dates that are never any holder's attested_on, start or end: national meetings and executive-committee elections,
# votes, counts and declarations, selections, leaves and acting service, interim designations, the interim period's
# stylings, prospective or scheduled days, registry periods, the resignation meeting, Berzoini's stated last day and
# the day a successor took office after it.
NEVER_HOLDER_DATE = {
    '1988-12-10', '1988-12-11', '1990-05-31', '1990-06-01', '1990-07-15', '1992-07-13', '1993-06-11', '1993-06-13',
    '1994-04-29', '1994-05-01', '1995-08-18', '1995-08-20', '1995-10-28', '1995-10-29', '1997-08-28', '1997-08-29',
    '1997-08-30', '1997-08-31', '1997-09-20', '1997-09-21', '1999-11-24', '1999-11-28', '2001-07-13', '2001-07-17',
    '2001-08-03', '2001-08-16', '2001-09-01', '2001-09-16', '2001-09-20', '2001-09-27', '2001-10-01', '2001-12-14',
    '2001-12-16', '2002-12-07', '2003-01-08', '2003-01-13', '2003-03-15', '2003-03-16', '2005-07-09', '2005-09-18',
    '2005-09-19', '2005-10-09', '2005-10-10', '2005-10-11', '2005-10-13', '2006-11-27', '2006-12-26', '2007-01-03',
    '2007-12-16', '2007-12-19', '2009-11-22', '2009-11-25', '2009-12-02', '2009-12-07', '2010-02-10', '2010-02-20',
    '2011-04-26', '2011-04-30', '2013-11-10', '2013-11-12', '2013-11-18', '2013-12-10', '2013-12-11', '2013-12-12',
    '2014-02-19', '2017-06-03', '2017-09-08', '2017-09-09', '2017-10-09', '2019-11-24', '2020-01-17', '2025-03-07',
    '2025-03-08', '2025-03-12', '2025-03-13', '2025-07-06', '2025-07-07', '2025-07-15', '2025-07-16', '2025-07-23',
    '2025-08-04', '2025-08-23', '2029-08-21'}
SURNAMES = {'Luiz Inácio Lula da Silva': 'Lula', 'Rui Falcão': 'Falc', 'José Dirceu': 'Dirceu', 'José Genoino': 'Geno',
            'Tarso Genro': 'Tarso', 'Ricardo Berzoini': 'Berzoini', 'José Eduardo Dutra': 'Dutra',
            'Gleisi Hoffmann': 'Gleisi', 'Humberto Costa': 'Humberto', 'Edinho Silva': 'Edinho'}
OTHER_NAMES = {'Luiz Gushiken', 'Marco Aurélio Garcia'}
REVIEW = [f'PT-PRES-{n:02d}' for n in range(1, 11)]
HOSTS = {'web.archive.org', 'fpabramo.org.br', 'siac.fpabramo.org.br', 'imagem.camara.leg.br', 'sgip3.tse.jus.br'}
# Secondary leads, blocked or live pages that must never be a recorded identity.
LEAD_URL_MARKERS = ('wikipedia', 'poder360', 'otempo', 'em.com.br', 'al.sp.gov.br', 'focusbrasil', 'fpabramo.org.br/20',
                    'J_BA_', 'idOrgaoPartidario=570664', 'DCD19DEZ2002', 'dadosabertos', 'rui-falcao-e-eleito',
                    'como-presidente-interino', 'gleisi-hoffmann-assumira', 'wp-json', 'sitemap', '?s=',
                    'camara.leg.br/deputados/73540', 'camara.leg.br/deputados/73604', 'ped.pt.org.br', 'cod=36604',
                    'cod=36615', 'cod=39223', 'cod=39181', 'resolucoes_int', 'execut.htm', '58721')
# Forms a server builds per request, cache-busting parameters and growing listings are never recorded identities.
PER_REQUEST_URL = re.compile(r'cdx/search|[?&]cb=|nocache|cachebust|[?&]_=|/search|searchAcervo|montaPdf|dc_20b|'
                             r'seqPagina|jsessionid|token|[?&]sid=|consulta\?|timemap', re.I)
# Dossier ids renamed or withdrawn by the build and the checks: none may remain in the packet.
STALE_IDS = ('br_fpa_cen_altered_presidente_gushiken_19881211', 'br_fpa_dn5_elected_7th_encontro_presidente_lula_19900603',
             'br_fpa_dn_dirceu_resigns_national_presidency_20030315', 'br_ptn051_presidential_vote_dirceu_284_temer_256_19970831',
             'br_genoino_declares_office_handed_to_dn_on_leave_20050709', 'br_tarso_leadership_empossados_until_ped_20050709',
             'br_tse_sgip_pt_dn_2025_2029', 'br_tse_sgip_edinho_active_president_record_20260708',
             'br_tse_sgip_edinho_registered_exercise_from_20250823', 'br_ptn_ee12_dirceu_presidente_nacional_at_opening_20011213',
             'br_pt_dn_page_edinho_ped_election_recited_20250706', 'attested_period', 'printed_range', 'name_as_printed')
REPORT = research.RESEARCH / 'brazil-pt-presidents-1990-2026-22.md'
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-22.md'


def load_rows():
    packet = json.loads((research.ROOT / research.RESEARCH / 'brazil.json').read_text(encoding='utf-8'))
    rows = {}
    for source in packet['sources'][EARLIER_SOURCE_COUNT:]:
        extract = json.loads((research.ROOT / source['snapshot']['path']).read_text(encoding='utf-8'))
        rows.update({row['claim_id']: row for row in extract['rows']})
    return rows


def pt_entry(packet):
    entry, = [o for o in packet['organizations'] if o['id'] == ORG_ID]
    return entry


def pt_rules(packet, rows):
    """Rule-based checks that hold without the pinned holder list; raise AssertionError, KeyError or IndexError."""
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    claim_source = {c['id']: s['id'] for s in packet['sources'] for c in s['claims']}
    pt = pt_entry(packet)
    # The funding observation's identity, lifecycle, mapping and funding record stay unchanged.
    assert (pt['name'], pt['kind']) == ('PT', 'political_party_election_funding_observation')
    assert pt['source_identifier']['value'] == '0613102-94.2024.6.00.0000'
    assert pt['represented_party_ids'] == [] and pt['reconciled_organization_id'] is None
    assert pt['lifecycle'] == LIFECYCLE and pt['funding_observation'] == FUNDING
    assert [(r['id'], r['title'], r['kind']) for r in pt['roles']] == [(ROLE, T_PRES, 'party_leader')], 'one PT role'
    role = pt['roles'][0]
    assert list(role) == ['id', 'title', 'kind', 'sources', 'claim_ids', 'holder_claims', 'scope_note']
    # The party office exists once, on the PT observation; the presidency institution keeps exactly its two roles.
    placed = [(e['id'], r['id']) for e in packet['organizations'] + packet['institutions'] for r in e['roles']
              if r['id'] == ROLE or r['kind'] == 'party_leader' or r['title'] == T_PRES]
    assert placed == [(ORG_ID, ROLE)], placed
    assert [i['id'] for i in packet['institutions']] == ['br_presidency']
    presidency = packet['institutions'][0]
    assert [r['id'] for r in presidency['roles']] == ['br_president', 'br_vice_president']
    # Party office and the Presidency of the Republic never feed each other.
    p_claims = set(presidency['claim_ids']) | {c for r in presidency['roles'] for c in r['claim_ids']} | {
        c for r in presidency['roles'] for h in r['holder_claims'] for c in h['claim_ids']}
    p_sources = set(presidency['sources']) | {s for r in presidency['roles'] for s in r['sources']} | {
        s for r in presidency['roles'] for h in r['holder_claims'] for s in h['sources']}
    pt_claims = set(role['claim_ids']) | {c for h in role['holder_claims'] for c in h['claim_ids']}
    pt_sources = set(role['sources']) | {s for h in role['holder_claims'] for s in h['sources']}
    assert not pt_claims & p_claims and not pt_sources & p_sources, 'cross-institution claim or source'
    assert all(c.startswith('br_pt_') for c in pt_claims) and not any(c.startswith('br_pt_') for c in p_claims)
    assert all(s.startswith('br_pt_') for s in pt_sources) and not any(s.startswith('br_pt_') for s in p_sources)
    for entry in packet['organizations'] + packet['institutions']:
        if entry is not pt:
            assert not set(entry['claim_ids']) & pt_claims and not set(entry['sources']) & pt_sources, entry['id']
    assert set(role['claim_ids']) <= set(pt['claim_ids']) and set(role['sources']) <= set(pt['sources'])
    for cid in role['claim_ids']:
        row = rows[cid]
        assert (row['observation_id'], row['role_id']) == (ORG_ID, ROLE), cid
        assert row['role_title'] == (T_ACT if row['event_kind'] in ACTING_KINDS else
                                     T_VICE if row['event_kind'] == 'vice_president_attestation' else T_PRES), cid
        assert row['holder_name'] in set(SURNAMES) | OTHER_NAMES | {None}, cid
        assert not {'period', 'attested_period', 'printed_range'} & set(claims[cid]) - {'attested_on'}, cid
        if row['event_kind'] in RETROSPECTIVE_KINDS:
            assert 'attested_on' not in claims[cid], (cid, 'retrospective claims carry no structured date')
    previous = ''
    for holder in role['holder_claims']:
        name = holder['name']
        assert isinstance(holder, dict) and name in SURNAMES, name
        assert list(holder) == ['name', 'attested_on', 'from', 'until', 'sources', 'claim_ids', 'note', 'uncertainty']
        dated = [d for d in (holder['attested_on'], holder['from']) if d]
        assert len(dated) == 1, (name, 'a holder is dated by exactly one of attested_on and from')
        assert dated[0] >= previous, (name, 'holders stay in chronological order')
        previous = dated[0]
        assert not {holder['attested_on'], holder['from'], holder['until']} & NEVER_HOLDER_DATE, name
        kinds = {}
        expected_sources = []
        for cid in holder['claim_ids']:
            row = rows[cid]
            assert cid in role['claim_ids'], cid
            # Cross-role guard: a holder rests only on this role's rows for the same person, titled as the office.
            assert row['role_id'] == ROLE and row['holder_name'] == name and row['role_title'] == T_PRES, (name, cid)
            assert row['event_kind'] in HOLDER_KINDS, (name, cid)
            assert SURNAMES[name] in claims[cid]['text'], (name, cid)
            kinds.setdefault(row['event_kind'], set()).add(claims[cid].get('attested_on'))
            if claim_source[cid] not in expected_sources:
                expected_sources.append(claim_source[cid])
        assert holder['sources'] == expected_sources, name
        from_days = set().union(*(kinds.get(k, set()) for k in FROM_KINDS))
        until_days = set().union(*(kinds.get(k, set()) for k in UNTIL_KINDS))
        support_days = kinds.get('in_office_attestation', set())
        # A start only where a reported posse or a stated assumption gives that day; otherwise no such claim.
        assert (from_days == {holder['from']}) if holder['from'] else not from_days, (name, 'start')
        # An end only where a stated resignation gives that day; otherwise no end claim at all.
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


def pt_invariants(packet, rows):
    """The rules plus the exact pinned holder list and events this packet intends."""
    pt_rules(packet, rows)
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    role = pt_entry(packet)['roles'][0]
    got = [(h['name'], h['attested_on'], h['from'], h['until']) for h in role['holder_claims']]
    assert got == HOLDERS, got
    assert [h['claim_ids'] for h in role['holder_claims']] == HOLDER_CLAIMS
    assert [(h['name'], h['until']) for h in role['holder_claims'] if h['until']] == ENDS
    assert [(h['name'], h['from']) for h in role['holder_claims'] if h['from']] == STARTS
    for cid, (_, day, _, _, _) in EVENTS.items():
        assert claims[cid].get('attested_on') == day, cid
    # The presidency's holders are untouched by this packet.
    president, vice = packet['institutions'][0]['roles']
    assert [(h['name'], h['attested_on'], h['from'], h['until']) for h in president['holder_claims']] == vp.PRESIDENT_HOLDERS
    assert [(h['name'], h['attested_on'], h['from'], h['until']) for h in vice['holder_claims']] == vp.HOLDERS
    # Distinct dated events stay distinct.
    for earlier, later in (('br_pt_ptn051_presidential_vote_dirceu_284_temer_256_19970831', 'br_pt_ptn052_dirceu_national_president_at_vitoria_meeting_19970917'),
                           ('br_pt_ptn105_dirceu_leave_to_run_for_reelection_20010717', 'br_pt_ptn106_genoino_signs_note_as_acting_president_20010803'),
                           ('br_pt_ptn109_ped_vote_held_20010916', 'br_pt_ptn109_ped_totalization_dirceu_elected_20010927'),
                           ('br_pt_ptn109_ped_totalization_dirceu_elected_20010927', 'br_pt_ptn109_dirceu_resumed_party_leadership_20011001'),
                           ('br_pt_ptn109_dirceu_resumed_party_leadership_20011001', 'br_pt_ptn109_dirceu_signs_note_as_president_20011004'),
                           ('br_pt_fpa_dn_genoino_elected_national_president_20030315', 'br_pt_genoino_attends_affiliation_as_pt_president_20030318'),
                           ('br_pt_dn_approves_tarso_genro_for_presidency_20050709', 'br_pt_tarso_reports_as_president_after_cen_meeting_20050710'),
                           ('br_pt_ped2005_first_round_ballot_20050918', 'br_pt_ped2005_second_round_ballot_20051009'),
                           ('br_pt_ped2005_berzoini_elected_declared_20051013', 'br_pt_dn_gives_posse_to_berzoini_20051022'),
                           ('br_pt_ped2009_ballot_20091122', 'br_pt_dutra_declared_president_elect_20091125'),
                           ('br_pt_dutra_declared_president_elect_20091125', 'br_pt_ped2009_final_result_dutra_20091202'),
                           ('br_pt_berzoini_states_last_day_of_mandate_20100210', 'br_pt_dutra_empossado_president_20100219'),
                           ('br_pt_ped2013_vote_held_20131110', 'br_pt_ped2013_falcao_reelection_announced_20131112'),
                           ('br_pt_gleisi_elected_6th_congress_20170603', 'br_pt_gleisi_posse_20170705'),
                           ('br_pt_humberto_interim_designated_executive_20250307', 'br_pt_humberto_states_interim_to_effective_20250320'),
                           ('br_pt_ped2025_result_announced_20250707', 'br_pt_ped2025_totalization_20250715'),
                           ('br_pt_ped2025_totalization_20250715', 'br_pt_edinho_empossado_20250803')):
        assert claims[earlier]['attested_on'] < claims[later]['attested_on'], (earlier, later)


class BrazilPtPresidentsTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.raw = (research.ROOT / research.RESEARCH / 'brazil.json').read_text(encoding='utf-8')
        cls.packet = json.loads(cls.raw)
        cls.sources = {s['id']: s for s in cls.packet['sources']}
        cls.claims = {c['id']: c for s in cls.packet['sources'] for c in s['claims']}
        cls.pt = pt_entry(cls.packet)
        cls.role = cls.pt['roles'][0]
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
        self.assertEqual(tuple(len(ids[k]) for k in ('entries', 'sources', 'claims', 'roles')), (32, 172, 408, 3))
        self.assertEqual((len(NEW_SOURCES), len(NEW_CLAIMS), len(HOLDERS)), (108, 177, 18))
        order = [s['id'] for s in self.packet['sources']]
        self.assertEqual(order[EARLIER_SOURCE_COUNT:], NEW_SOURCES)
        self.assertFalse([sid for sid in order[:EARLIER_SOURCE_COUNT] if sid.startswith('br_pt_')])
        self.assertEqual([c['id'] for sid in NEW_SOURCES for c in self.sources[sid]['claims']], NEW_CLAIMS)
        self.assertEqual(self.role['claim_ids'], NEW_CLAIMS)
        self.assertEqual(self.role['sources'], NEW_SOURCES)
        self.assertEqual(self.pt['claim_ids'], ['br_tse_fefc_2024_row_03'] + NEW_CLAIMS)
        self.assertEqual(self.pt['sources'], ['br_tse_fefc_2024'] + NEW_SOURCES)
        # Every new claim is a holder claim or a claim that never feeds a holder, never both.
        holder_claims = [cid for ids_ in HOLDER_CLAIMS for cid in ids_]
        self.assertEqual(len(holder_claims), len(set(holder_claims)))
        self.assertEqual(len(holder_claims), 22)
        never = set(NEW_CLAIMS) - set(holder_claims)
        for cid in never:
            self.assertIn(EVENTS[cid][2], ACTING_KINDS | ELECTION_KINDS | DEPARTURE_KINDS | RETROSPECTIVE_KINDS | {
                'executive_committee_change', 'in_office_period_attestation', 'in_office_continuation_attestation',
                'national_meeting_summary', 'national_meeting_held', 'vice_president_attestation',
                'stated_assumption_day_not_printed', 'stated_resumption_of_office', 'return_from_leave',
                'result_process_note', 'styled_president_during_interim_period', 'styled_on_selection_day',
                'prospective_tenure_statement', 'procedure_rule_amendment', 'assumption_scheduled', 'posse_scheduled',
                'posse_window_resolved', 'posse_speech_published', 'ped_participation_resolution',
                'official_assumption_announced_prospective'}, cid)
        # At most ten observations, PT-PRES-01..10, each reported and each carrying rows.
        observations = re.findall(r'^### (PT-PRES-\d\d)\b', self.report, re.M)
        self.assertEqual(observations, REVIEW)
        self.assertEqual({row[0] for row in EVENTS.values()}, set(REVIEW))
        for stale in STALE_IDS:
            self.assertNotIn(stale, self.raw, stale)

    def test_holders_are_exactly_as_intended(self):
        pt_invariants(self.packet, self.rows)
        for index, holder in enumerate(self.role['holder_claims']):
            self.assertTrue(holder['note'].startswith('From ' if holder['from'] else 'Observed on '), holder['name'])
            self.assertTrue(holder['uncertainty'], holder['name'])
            cited_obs = {self.rows[cid]['review_observation'] for cid in holder['claim_ids']
                         if self.rows[cid]['event_kind'] not in UNTIL_KINDS}
            self.assertEqual(cited_obs, {HOLDER_OBSERVATIONS[index]}, holder['name'])
        # Holder names are normalised; the printed forms stay in the claim texts.
        self.assertEqual({row['holder_name'] for row in self.rows.values() if row['claim_id'] in EVENTS},
                         set(SURNAMES) | OTHER_NAMES | {None})
        self.assertIn('RUI GOETHE DA COSTA FALCAO', self.claims['br_pt_tse_sgip_falcao_dn_registered_exercise_20140219_20170908']['text'])
        self.assertIn("'180 EDINHO'", self.claims['br_pt_ped2025_totalization_20250715']['text'])
        self.assertIn("'130 RUI FALCAO'", self.claims['br_pt_ped2025_totalization_20250715']['text'])
        self.assertIn('Genoíno', self.claims['br_pt_genoino_styled_president_interim_period_20030108']['text'])
        self.assertIn('Rui Goethe da Costa Falcão', self.claims['br_pt_dn_composition_lists_falcao_president_20131218']['text'])
        # Acting and interim service is claims only, titled as such, and never cited by a holder.
        acting = [cid for cid, e in EVENTS.items() if e[2] in ACTING_KINDS]
        self.assertEqual(len(acting), 19)
        for cid in acting:
            self.assertEqual(EVENTS[cid][4], 'A', cid)
            self.assertRegex(self.claims[cid]['uncertainty'], r'(?i)claim only|never a holder', cid)
        self.assertEqual({EVENTS[cid][3] for cid in acting}, {'José Genoino', 'Marco Aurélio Garcia', 'Rui Falcão',
                                                               'Humberto Costa'})
        # Holder claims explain their use.
        for holder in self.role['holder_claims']:
            for cid in holder['claim_ids']:
                kind = EVENTS[cid][2]
                pattern = {'in_office_attestation': r'Dates the holder observation|not itself a start|not a start',
                           'posse_reported': r'basis for .* start', 'stated_assumption_of_office': r'basis for .* start',
                           'resignation_stated': r"basis for .* until"}[kind]
                self.assertRegex(self.claims[cid]['uncertainty'], pattern, cid)

    def test_starts_ends_and_interim_service_only_where_a_source_states_them(self):
        claims = self.claims
        undated = [cid for cid, e in EVENTS.items() if e[1] is None]
        self.assertEqual(len(undated), 57)
        for cid in undated:
            self.assertNotIn('attested_on', claims[cid], cid)
            self.assertIn('no structured date is stored', claims[cid]['uncertainty'].lower(), cid)
        # Starts: a reported posse or a stated assumption of that day.
        self.assertIn("'Volto a assumir a Presidência do Partido dos Trabalhadores'",
                      claims['br_pt_bn051_lula_states_he_assumes_presidency_19900603']['text'])
        self.assertIn("'deu posse, neste sábado (22)'", claims['br_pt_dn_gives_posse_to_berzoini_20051022']['uncertainty'])
        self.assertIn("agora saio da condição de presidente interino para presidente efetivo",
                      claims['br_pt_humberto_states_interim_to_effective_20250320']['text'])
        self.assertIn("'empossado neste domingo (3)'", claims['br_pt_edinho_empossado_20250803']['uncertainty'])
        # The one end: Dutra's stated resignation day.
        self.assertIn("'A renúncia do presidente do PT José Eduardo Dutra no dia 29 de abril",
                      claims['br_pt_dutra_resignation_20110429']['text'])
        # Recorded but never a boundary, each saying why.
        self.assertIn('never an until', claims['br_pt_berzoini_states_last_day_of_mandate_20100210']['uncertainty'])
        self.assertIn('hoje é o último dia do meu mandato', claims['br_pt_berzoini_states_last_day_of_mandate_20100210']['text'])
        self.assertIn('never a start', claims['br_pt_edinho_official_assumption_announced_for_20250804']['uncertainty'])
        self.assertIn('próxima segunda-feira (4)', claims['br_pt_edinho_official_assumption_announced_for_20250804']['text'])
        self.assertIn('two-day meeting', claims['br_pt_fpa_dn_dirceu_resigns_national_presidency_20030315_16']['uncertainty'])
        self.assertIn('names nobody', claims['br_pt_empossados_until_ped_statement_20050709']['uncertainty'])
        self.assertIn('stale masthead', claims['br_pt_ptn109_masthead_genoino_em_exercicio_20011010_25']['uncertainty'])
        self.assertIn('never a holder date', claims['br_pt_genoino_signs_official_note_interim_period_20030113']['uncertainty'])
        self.assertIn('procedure only', claims['br_pt_cen_amends_ped2009_posse_rule_20091207']['uncertainty'])
        self.assertIn('no structured date is stored', claims['br_pt_dn_page_gleisi_resignation_referenced']['uncertainty'])
        for cid in [c for c, e in EVENTS.items() if e[2] in RETROSPECTIVE_KINDS]:
            self.assertNotIn('attested_on', claims[cid], cid)
        scope = self.role['scope_note']
        for phrase in ('no br_presidency claim or source feeds this role', 'none of its claims feeds br_presidency',
                       'Do not fill the interval', "infer an outgoing holder's last day from a successor's",
                       'Acting, interim and continuation service are claims only, never holders',
                       'procedure only, never a date', 'every claim and source id of this role begins br_pt_'):
            self.assertIn(phrase, scope)
        unresolved = self.pt['coverage']['unresolved']
        self.assertEqual(unresolved[:-1], ORG_UNRESOLVED)
        self.assertTrue(unresolved[-1].startswith('PT national presidents 1990-2026 (CLAUDE-C01-22)'))
        packet_unresolved = self.packet['coverage']['unresolved']
        self.assertTrue(packet_unresolved[-1].startswith('PT national presidents 1990-2026 (CLAUDE-C01-22'))
        self.assertEqual(sum('CLAUDE-C01-22' in u for u in packet_unresolved), 1)
        self.assertTrue(packet_unresolved[-2].startswith('Vice-presidents 1990-2026 (CLAUDE-C01-17)'))

    def test_extracts_match_packet_claims_and_record_original_responses(self):
        for sid in NEW_SOURCES:
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual(extract['format'], 'spheres-c01-derived-factual-table/v1')
            self.assertEqual((extract['source_id'], extract['source_url']), (sid, source['url']))
            for key in ('scope_note', 'access_method', 'published_date', 'rights_note'):
                self.assertEqual(extract[key], source[key], (sid, key))
            self.assertEqual((extract['accessed_date'], source['accessed_date']), ('2026-09-25', '2026-09-25'))
            self.assertLessEqual(source['published_date'] or '', research.CUTOFF)
            self.assertIs(extract['source_response_checked_in'], False)
            self.assertEqual((extract['source_response_bytes'], extract['source_response_sha256']), RESPONSES[sid])
            self.assertIn('not checked into this repository', extract['provenance_note'])
            self.assertIn('derived factual extract', extract['provenance_note'])
            self.assertIn('No open license, portrait permission or likeness approval', extract['rights_note'])
            self.assertRegex(extract['stability_check'], r'again')
            self.assertIn(extract['stability_check'], extract['provenance_note'])
            self.assertEqual(extract['visual_review']['pdf_pages_one_based'], PDF_PAGES.get(sid, []))
            self.assertTrue(source['source_type'].startswith('primary_') and source['scope_note'] and source['publisher'])
            if sid in GZIP:
                self.assertEqual(extract['source_response_content_encoding'], 'gzip')
                self.assertEqual((extract['decoded_response_bytes'], extract['decoded_response_sha256']), GZIP[sid])
                self.assertIn('gzip-encoded body exactly as served', extract['provenance_note'])
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
                self.assertIn('no Accept-Encoding request header and no automatic decoding', extract['fetch_recipe'])
            else:
                self.assertNotEqual(url.hostname, 'web.archive.org')
                self.assertNotIn('original_url', source)
                self.assertNotIn('archive_capture_utc', extract)
        self.assertEqual(len({self.sources[s]['snapshot']['path'] for s in NEW_SOURCES}), len(NEW_SOURCES))
        got = {cid: (row['review_observation'], row['attested_on'], row['event_kind'], row['holder_name'],
                     {T_PRES: 'P', T_ACT: 'A', T_VICE: 'V'}[row['role_title']])
               for cid, row in self.rows.items() if row['role_id'] == ROLE}
        self.assertEqual(got, EVENTS)
        self.assertEqual(list(got), NEW_CLAIMS)
        # The Dirceu biography capture's gzip behaviour is stated (check A).
        self.assertIn('10,369-byte gzip-compressed body',
                      self.extracts['br_pt_camara_bio_dirceu_20251108']['provenance_note'])

    def test_no_per_request_url_no_secondary_lead_and_no_personal_registry_data(self):
        self.assertEqual({urlsplit(self.sources[s]['url']).hostname for s in NEW_SOURCES}, HOSTS)
        for sid in NEW_SOURCES:
            url = self.sources[sid]['url']
            self.assertIsNone(PER_REQUEST_URL.search(url), sid)
            host = urlsplit(url).hostname
            self.assertNotIn(host, {'pt.org.br', 'www.pt.org.br', 'www.camara.leg.br', 'www.tse.jus.br'}, sid)
            if host == 'sgip3.tse.jus.br':
                self.assertRegex(url, r'/orgaoPartidario/comAnotacoesEMembros\?idOrgaoPartidario=\d+&isMembrosAtivos=false$')
        for source in self.packet['sources']:
            for marker in LEAD_URL_MARKERS:
                self.assertNotIn(marker, source['url'], (source['id'], marker))
        # Only closed SGIP organs are recorded; the live organ in force is a lead.
        self.assertEqual(sorted(re.search(r'idOrgaoPartidario=(\d+)', self.sources[s]['url']).group(1)
                                for s in NEW_SOURCES if 'sgip3' in self.sources[s]['url']),
                         ['232052', '308884', '70953', '70954'])
        lowered = self.raw.lower()
        for marker in ('wikipedia', 'poder360', 'otempo', 'folha.uol', 'estadao', 'g1.globo'):
            self.assertNotIn(marker, lowered, marker)
        for sid in NEW_SOURCES:
            text = (research.ROOT / self.sources[sid]['snapshot']['path']).read_text(encoding='utf-8')
            for marker in ('nrCpf', 'nmSenha', 'nrTitulo', 'dsEmail', 'nrTelefone'):
                self.assertNotIn(marker, text, (sid, marker))
            self.assertIsNone(re.search(r'\b\d{3}\.\d{3}\.\d{3}-\d{2}\b|\b\d{11}\b', text), sid)
        leads, added = self.section('Leads not imported'), self.section('Sources added')
        for marker in ('idOrgaoPartidario=570664', 'DCD19DEZ2002', 'wikipedia', 'poder360', 'al.sp.gov.br',
                       'rui-falcao-e-eleito-por-unanimidade', 'J_BA_1995_0073', 'gleisi-hoffmann-assumira', '58721'):
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

        def role(packet):
            return pt_entry(packet)['roles'][0]

        def holder(packet, index):
            return role(packet)['holder_claims'][index]

        def president(packet):
            return packet['institutions'][0]['roles'][0]

        def cite(packet, index, cid, sid):
            holder(packet, index)['claim_ids'].append(cid)
            if sid not in holder(packet, index)['sources']:
                holder(packet, index)['sources'].append(sid)

        def extra(name, day, sid, cid, start=None):
            return {'name': name, 'attested_on': None if start else day, 'from': start, 'until': None,
                    'sources': [sid], 'claim_ids': [cid], 'note': 'n', 'uncertainty': 'u'}

        validator_cases = [
            (lambda p: source(p, 'br_pt_bn_edicao_especial_199412')['snapshot'].update(sha256='0' * 64), 'checksum mismatch'),
            (lambda p: source(p, 'br_pt_tse_sgip_dn_2020_2025')['snapshot'].update(bytes=1), 'checksum mismatch'),
            (lambda p: source(p, 'br_pt_dn_page_edinho_20260815')['snapshot'].update(path=REPORT.as_posix()), 'escapes'),
            (lambda p: holder(p, 17).update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: holder(p, 16).update(until='2026-09-08'), 'exceeds cutoff'),
            (lambda p: claim(p, 'br_pt_dn_page_edinho_presidente_nacional_20260815').update(attested_on='2026-09-08'),
             'exceeds cutoff'),
            (lambda p: holder(p, 10).update(until='2009-12-31'), 'Reversed historical interval'),
            (lambda p: holder(p, 2)['claim_ids'].append('br_pt_ptn052_dirceu_national_president_at_vitoria_meeting_19970917'),
             'cited source'),
            (lambda p: role(p)['claim_ids'].append('br_pt_does_not_exist'), 'Unknown'),
            (lambda p: pt_entry(p).update(represented_party_ids=['Brazil/guessed_pt']), 'foreign represented party'),
        ]
        for change, message in validator_cases:
            with self.subTest(message=message), self.assertRaisesRegex(ValueError, message):
                self.validate(mutated(change))
        lula_2003 = copy.deepcopy(next(h for h in self.packet['institutions'][0]['roles'][0]['holder_claims']
                                       if (h['name'], h['from']) == ('Luiz Inácio Lula da Silva', '2003-01-01')))
        rule_cases = [
            ("successor's styling used as an end (Lula, Rui Falcão)", lambda p: holder(p, 0).update(until='1994-11-25')),
            ("successor's start used as an end (Berzoini 2009, Dutra)", lambda p: holder(p, 9).update(until='2010-02-19')),
            ("successor's start used as an end (Gleisi 2020, Humberto Costa)", lambda p: holder(p, 14).update(until='2025-03-20')),
            ("successor's start used as an end (Humberto Costa, Edinho Silva)", lambda p: holder(p, 15).update(until='2025-08-03')),
            ("successor's selection used as an end (Genoino, Tarso Genro)", lambda p: holder(p, 6).update(until='2005-07-09')),
            ("successor's posse used as an end (Tarso Genro, Berzoini)", lambda p: holder(p, 7).update(until='2005-10-22')),
            ('stated last day used as an end (Berzoini 2009)', lambda p: (
                holder(p, 9).update(until='2010-02-10'),
                cite(p, 9, 'br_pt_berzoini_states_last_day_of_mandate_20100210', 'br_pt_camara_dcd_berzoini_farewell_20100211'))),
            ('two-day resignation used as an end (Dirceu 2001)', lambda p: holder(p, 5).update(until='2003-03-15')),
            ('leave used as an end (Dirceu 1999)', lambda p: holder(p, 4).update(until='2001-07-17')),
            ('registry period used as an end (Gleisi 2020)', lambda p: holder(p, 14).update(until='2025-03-07')),
            ('election date used as start (Dirceu 1997 vote)', lambda p: holder(p, 3).update({'from': '1997-08-31', 'attested_on': None})),
            ('election date used as start (Gleisi 2017)', lambda p: holder(p, 13).update({'from': '2017-06-03'})),
            ('result declaration used as start (Edinho Silva)', lambda p: holder(p, 16).update({'from': '2025-07-07'})),
            ('nomination used as start (Tarso Genro)', lambda p: holder(p, 7).update({'from': '2005-07-09', 'attested_on': None})),
            ('Diretório election used as start (Genoino)', lambda p: holder(p, 6).update({'from': '2003-03-15', 'attested_on': None})),
            ('prospective assumption used as start (Edinho Silva)', lambda p: holder(p, 16).update({'from': '2025-08-04'})),
            ('registry start used as start (Edinho Silva)', lambda p: holder(p, 16).update({'from': '2025-08-23'})),
            ('in-office observation used as start (Dirceu 1995)', lambda p: holder(p, 2).update({'from': '1995-08-24', 'attested_on': None})),
            ('election claim cited by a holder (Dirceu 1997)', lambda p: cite(
                p, 3, 'br_pt_ptn051_presidential_vote_dirceu_284_temer_256_19970831', 'br_pt_noticias_051_19970910')),
            ('continuation claim cited by a holder (Dirceu 2001)', lambda p: holder(p, 5)['claim_ids'].append(
                'br_pt_ptn109_dirceu_signs_note_as_president_20011006')),
            ('interim-period styling cited by a holder (Genoino)', lambda p: cite(
                p, 6, 'br_pt_genoino_signs_official_note_interim_period_20030113', 'br_pt_official_note_lauro_campos_20030113')),
            ('acting service added as a holder (Genoino 2001)', lambda p: role(p)['holder_claims'].insert(6, extra(
                'José Genoino', '2001-08-03', 'br_pt_noticias_106_20010804', 'br_pt_ptn106_genoino_signs_note_as_acting_president_20010803'))),
            ('interim service added as a holder (Humberto Costa 2025)', lambda p: role(p)['holder_claims'].insert(15, extra(
                'Humberto Costa', '2025-03-07', 'br_pt_humberto_interim_executive_20250307', 'br_pt_humberto_interim_designated_executive_20250307'))),
            ('interim service added as a holder (Marco Aurélio Garcia)', lambda p: role(p)['holder_claims'].insert(9, extra(
                'Marco Aurélio Garcia', '2006-11-27', 'br_pt_site_cen_members_20061127', 'br_pt_marco_aurelio_garcia_interim_president_listed_20061127'))),
            ('acting service added as a holder (Rui Falcão 2011)', lambda p: role(p)['holder_claims'].insert(11, extra(
                'Rui Falcão', '2011-04-26', 'br_pt_portal_dutra_dn_meeting_20110426', 'br_pt_falcao_styled_acting_president_20110426'))),
            ('interim-period styling added as a holder (Genoino 2003)', lambda p: role(p)['holder_claims'].insert(6, extra(
                'José Genoino', '2003-01-13', 'br_pt_official_note_lauro_campos_20030113', 'br_pt_genoino_signs_official_note_interim_period_20030113'))),
            ('cross-role holder: the Presidency of the Republic added to the party role',
             lambda p: role(p)['holder_claims'].insert(6, copy.deepcopy(lula_2003))),
            ('cross-role holder: a party president added to br_president',
             lambda p: president(p)['holder_claims'].insert(5, copy.deepcopy(holder(p, 5)))),
            ('cross-role claim: a presidency claim moved onto the party role', lambda p: (
                role(p)['claim_ids'].append('br_lula_posse_declared_20030101'),
                role(p)['sources'].append('br_dcn_1_2003_posse_20030101'))),
            ('cross-role claim: a party claim moved onto br_president', lambda p: (
                president(p)['claim_ids'].append('br_pt_edinho_empossado_20250803'),
                president(p)['sources'].append('br_pt_edinho_posse_speech_20250803'))),
            ('cross-institution: the party role copied onto another organization',
             lambda p: p['organizations'][0]['roles'].append(copy.deepcopy(role(p)))),
            ('cross-institution: the party role placed on the presidency institution',
             lambda p: p['institutions'][0]['roles'].append(copy.deepcopy(role(p)))),
            ('second party role', lambda p: pt_entry(p)['roles'].append(dict(copy.deepcopy(role(p)), id='br_pt_vice_president'))),
            ('party role removed', lambda p: pt_entry(p)['roles'].clear()),
            ('PT lifecycle given a start', lambda p: pt_entry(p)['lifecycle'].update({'from': '1980-02-10'})),
            ('retrospective span given a date', lambda p: claim(p, 'br_pt_falcao_balance_retrospective_span').update(
                attested_on='2017-05-27')),
            ('printed civil name used as the holder name', lambda p: holder(p, 12).update(name='Rui Goethe da Costa Falcão')),
            ('holders out of order', lambda p: role(p)['holder_claims'].reverse()),
        ]
        pt_rules(self.packet, self.rows)
        pt_invariants(self.packet, self.rows)
        for label, change in rule_cases:
            with self.subTest(label=label):
                packet = mutated(change)
                with self.assertRaises((AssertionError, KeyError, IndexError, ValueError)):
                    pt_rules(packet, self.rows)
                with self.assertRaises((AssertionError, KeyError, IndexError, ValueError)):
                    pt_invariants(packet, self.rows)
        # Event collapses are caught by the pinned events.
        for cid, day in (('br_pt_ptn051_presidential_vote_dirceu_284_temer_256_19970831', '1997-09-17'),
                         ('br_pt_dn_approves_tarso_genro_for_presidency_20050709', '2005-07-10'),
                         ('br_pt_ped2005_berzoini_elected_declared_20051013', '2005-10-22'),
                         ('br_pt_dn_elects_humberto_20250320', '2025-03-07'),
                         ('br_pt_edinho_official_assumption_announced_for_20250804', '2025-08-04')):
            with self.subTest(collapsed=cid), self.assertRaises(AssertionError):
                pt_invariants(mutated(lambda p: claim(p, cid).update(attested_on=day)), self.rows)

    def test_report_and_handoff_close_no_parent_gate(self):
        self.assertIn('ready_for_review', self.report)
        table = self.section('Outcome')
        decisions = dict.fromkeys([f'{n:02d}' for n in range(1, 11)], 'Accepted in part')
        decisions['10'] = 'Accepted'
        for number, decision in decisions.items():
            row, = [line for line in table.splitlines() if line.startswith(f'| PT-PRES-{number} ')]
            self.assertIn(f'**{decision}:**', row)
        defects = self.section('Checker defects')
        rows = [line for line in defects.splitlines() if re.match(r'\| [ABC]\d+ ', line)]
        self.assertEqual([re.match(r'\| ([ABC]\d+) ', r).group(1) for r in rows],
                         [f'A{n}' for n in range(1, 13)] + [f'B{n}' for n in range(1, 15)] + [f'C{n}' for n in range(1, 15)])
        for row in rows:
            self.assertRegex(row, r'\*\*(Applied|Applied in part|Resolved by removal|Declined)')
        missing = [line for line in defects.splitlines() if re.match(r'\| M[ABC]\d+ ', line)]
        self.assertEqual(len(missing), 25)
        for marker in ('C01', 'C06', 'S23', 'WC1', 'CP1'):
            self.assertNotRegex(self.report, rf'\b{marker}\b[^.\n]*\bis (now )?complete\b')
        notes = self.section('Integration notes')
        for text in ('45821428', '7d71acef', 'claude/c01-br-17', 'research-index.json', 'only file shared',
                     'test_brazil_research_s10f.py', 'test_brazil_presidents_c01_10.py',
                     'test_brazil_vice_presidents_c01_17.py', 'test_campaign_census'):
            self.assertIn(text, notes)
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        for text in ('ready_for_review', 'brazil-pt-presidents-1990-2026-22.md', 'claude/c01-br-22', '45821428',
                     'claude/c01-br-17', 'test_brazil_pt_presidents_c01_22.py'):
            self.assertIn(text, handoff)
        index = research.build()
        country = next(p for p in index['countries'] if p['nation'] == 'Brazil')
        self.assertFalse(country['country_census_complete'])
        self.assertIsNone(country['unrepresented_organization_count'])
        self.assertEqual((country['institution_observations'], country['role_observations'], country['source_claims']),
                         (1, 3, 408))
        self.assertEqual({w['status'] for w in index['work_orders'] if w['nation'] == 'Brazil'}, {'open'})
        self.assertFalse(index['c01_complete'])


if __name__ == '__main__':
    unittest.main()
